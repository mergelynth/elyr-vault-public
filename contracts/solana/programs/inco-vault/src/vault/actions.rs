use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::solana_program::keccak;
use anchor_lang::solana_program::ed25519_program;
use anchor_lang::solana_program::sysvar::instructions as ix_sysvar;
use inco_lightning::ID as INCO_LIGHTNING_ID;

use super::types::*;
use super::state::*;
use super::errors::VaultError;
use super::events::*;
use super::conditions;
use super::helpers;

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: claim
// ═══════════════════════════════════════════════════════════════════════
//
// Claims vault assets. Verifies recipient, checks conditions, transfers SOL/SPL.
// Mirrors EVM VaultActions.claim().
//
// remaining_accounts layout (variable):
//   Phase 1: ExtraCondition PDAs (count = vault.extra_conditions_count)
//   Phase 2: Condition data accounts (1 per condition: ActivityTracker OR
//            monitoring account OR ConditionDepositTracker)
//   Phase 3: FHE [allowance_account, allowed_address] × N encrypted fields
//   Phase 4 (SPL): [vault_token_account, claimer_token_account, token_program]

pub fn claim<'info>(
    ctx: Context<'_, '_, '_, 'info, ClaimVault<'info>>,
    vault_id: u64,
    claim_salt: [u8; 32],
    condition_salt: [u8; 32],
    condition_values: Vec<u64>,
) -> Result<()> {
    let vault = &ctx.accounts.vault;
    require!(vault.status == VaultStatus::Locked as u8, VaultError::VaultNotLocked);
    require!(
        vault.vault_type == VaultType::Asset as u8 || vault.vault_type == VaultType::Hybrid as u8,
        VaultError::InvalidVaultType
    );

    // Verify recipient
    helpers::verify_recipient(vault, &ctx.accounts.claimer.key(), &claim_salt)?;

    // Check conditions
    let clock = Clock::get()?;
    let encrypt_conditions = (vault.privacy_flags & PRIVACY_ENCRYPT_CONDITIONS) != 0;
    let extra_conditions = collect_extra_conditions(ctx.remaining_accounts, vault)?;
    let (activity_ts, deposit_or_balance) = collect_condition_data(
        ctx.remaining_accounts, vault, &extra_conditions,
    );

    if encrypt_conditions {
        conditions::verify_and_check_conditions(
            vault,
            &extra_conditions,
            &condition_salt,
            &condition_values,
            &clock,
            &activity_ts,
            &deposit_or_balance,
        )?;
    } else {
        require!(
            conditions::can_claim(vault, &extra_conditions, &clock, &activity_ts, &deposit_or_balance),
            VaultError::ConditionsNotMet
        );
    }

    // Check deadline hasn't passed
    if vault.deadline > 0 {
        require!(
            clock.unix_timestamp <= vault.deadline,
            VaultError::ClaimPeriodExpired
        );
    }

    // Transfer deposit to claimer
    let amount = vault.deposit_amount;
    if amount > 0 {
        if vault.deposit_token == Pubkey::default() {
            // Native SOL transfer
            helpers::transfer_sol_from_vault(
                &ctx.accounts.vault.to_account_info(),
                &ctx.accounts.claimer.to_account_info(),
                amount,
            )?;
        } else if !vault.is_confidential_token {
            // SPL token transfer via remaining_accounts
            let extra_count = vault.extra_conditions_count as usize;
            let total_conds = 1 + extra_count;
            let fhe_pairs = count_fhe_pairs(vault);
            let spl_start = extra_count + total_conds + fhe_pairs * 2;

            if ctx.remaining_accounts.len() >= spl_start + 3 {
                let vault_id_bytes = vault.id.to_le_bytes();
                let seeds: &[&[u8]] = &[b"vault", &vault_id_bytes, &[vault.bump]];
                helpers::transfer_spl_from_vault(
                    &ctx.remaining_accounts[spl_start],
                    &ctx.remaining_accounts[spl_start + 1],
                    &ctx.accounts.vault.to_account_info(),
                    &ctx.remaining_accounts[spl_start + 2],
                    amount,
                    seeds,
                )?;
            }
        }
    }

    // Update vault status
    let vault = &mut ctx.accounts.vault;
    vault.status = VaultStatus::Claimed as u8;

    emit!(VaultClaimed {
        vault_id,
        recipient: ctx.accounts.claimer.key(),
        amount,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct ClaimVault<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault_id.to_le_bytes().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub claimer: Signer<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: Inco Lightning program (optional, for FHE grants)
    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: AccountInfo<'info>,
    // remaining_accounts: condition data + FHE allowance accounts
}

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: refund
// ═══════════════════════════════════════════════════════════════════════
//
// Refunds vault after deadline. Creator or fallback can call.
// Mirrors EVM VaultActions.refund().

pub fn refund<'info>(
    ctx: Context<'_, '_, '_, 'info, RefundVault<'info>>,
    vault_id: u64,
    refund_salt: [u8; 32],
) -> Result<()> {
    let vault = &ctx.accounts.vault;
    require!(vault.status == VaultStatus::Locked as u8, VaultError::VaultNotLocked);
    require!(
        vault.vault_type == VaultType::Asset as u8 || vault.vault_type == VaultType::Hybrid as u8,
        VaultError::InvalidVaultType
    );

    // Deadline must exist and be passed
    require!(vault.deadline > 0, VaultError::DeadlineNotReached);
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp > vault.deadline,
        VaultError::DeadlineNotReached
    );

    // Verify caller is creator or fallback
    let caller = ctx.accounts.caller.key();
    let is_creator = caller == vault.creator;
    let is_fallback = helpers::verify_fallback(vault, &caller, &refund_salt);

    require!(is_creator || is_fallback, VaultError::NotAuthorizedForRefund);

    // Transfer deposit back
    let amount = vault.deposit_amount;
    if amount > 0 {
        if vault.deposit_token == Pubkey::default() {
            // Native SOL
            helpers::transfer_sol_from_vault(
                &ctx.accounts.vault.to_account_info(),
                &ctx.accounts.caller.to_account_info(),
                amount,
            )?;
        } else if !vault.is_confidential_token {
            // SPL token: remaining_accounts = [vault_token_account, caller_token_account, token_program]
            if ctx.remaining_accounts.len() >= 3 {
                let vault_id_bytes = vault.id.to_le_bytes();
                let seeds: &[&[u8]] = &[b"vault", &vault_id_bytes, &[vault.bump]];
                helpers::transfer_spl_from_vault(
                    &ctx.remaining_accounts[0],
                    &ctx.remaining_accounts[1],
                    &ctx.accounts.vault.to_account_info(),
                    &ctx.remaining_accounts[2],
                    amount,
                    seeds,
                )?;
            }
        }
    }

    // Update vault status
    let vault = &mut ctx.accounts.vault;
    vault.status = VaultStatus::Refunded as u8;

    emit!(VaultRefunded {
        vault_id,
        refund_to: ctx.accounts.caller.key(),
        amount,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct RefundVault<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault_id.to_le_bytes().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub caller: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: record_activity
// ═══════════════════════════════════════════════════════════════════════
//
// Records wallet activity (resets inactivity timer).
// Mirrors EVM VaultActions.recordActivity().

pub fn record_activity(ctx: Context<RecordActivity>) -> Result<()> {
    let clock = Clock::get()?;
    let tracker = &mut ctx.accounts.activity_tracker;
    tracker.user = ctx.accounts.user.key();
    tracker.timestamp = clock.unix_timestamp;
    tracker.bump = ctx.bumps.activity_tracker;

    emit!(ActivityRecorded {
        user: ctx.accounts.user.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct RecordActivity<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + ActivityTracker::LEN,
        seeds = [b"last_activity", user.key().as_ref()],
        bump,
    )]
    pub activity_tracker: Account<'info, ActivityTracker>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: grant_decryption_rights
// ═══════════════════════════════════════════════════════════════════════
//
// Grants FHE decryption rights to caller (for Secret vaults after conditions met).
// Mirrors EVM VaultActions.grantDecryptionRights().
//
// remaining_accounts layout:
//   Phase 1: ExtraCondition PDAs (count = vault.extra_conditions_count)
//   Phase 2: Condition data accounts (1 per condition)
//   Phase 3: FHE [allowance, address] × N encrypted fields
//   Phase 4: FHE [allowance, address] × extra condition encrypted values

pub fn grant_decryption_rights<'info>(
    ctx: Context<'_, '_, '_, 'info, GrantDecryptionRights<'info>>,
    _vault_id: u64,
    claim_salt: [u8; 32],
    condition_salt: [u8; 32],
    condition_values: Vec<u64>,
) -> Result<()> {
    let vault = &ctx.accounts.vault;

    // Verify recipient
    helpers::verify_recipient(vault, &ctx.accounts.caller.key(), &claim_salt)?;

    // Check conditions (properly loading extras + condition data)
    let clock = Clock::get()?;
    let encrypt_conditions = (vault.privacy_flags & PRIVACY_ENCRYPT_CONDITIONS) != 0;
    let extra_conditions = collect_extra_conditions(ctx.remaining_accounts, vault)?;
    let (activity_ts, deposit_or_balance) = collect_condition_data(
        ctx.remaining_accounts, vault, &extra_conditions,
    );

    if encrypt_conditions {
        conditions::verify_and_check_conditions(
            vault,
            &extra_conditions,
            &condition_salt,
            &condition_values,
            &clock,
            &activity_ts,
            &deposit_or_balance,
        )?;
    } else {
        require!(
            conditions::can_claim(vault, &extra_conditions, &clock, &activity_ts, &deposit_or_balance),
            VaultError::ConditionsNotMet
        );
    }

    // Grant FHE decryption rights to caller using vault PDA as signer
    let vault_id_bytes = vault.id.to_le_bytes();
    let seeds: &[&[u8]] = &[b"vault", &vault_id_bytes, &[vault.bump]];
    let caller_key = ctx.accounts.caller.key();
    let inco = &ctx.accounts.inco_lightning_program;
    let sys = &ctx.accounts.system_program.to_account_info();
    let vault_info = &ctx.accounts.vault.to_account_info();

    // FHE accounts start after ExtraCondition PDAs + condition data accounts
    let extra_count = vault.extra_conditions_count as usize;
    let total_conds = 1 + extra_count;
    let mut ra_offset = extra_count + total_conds;
    let ra = ctx.remaining_accounts;

    // Grant access for each encrypted field that is set
    let fields_to_grant: Vec<(bool, inco_lightning::types::Euint128)> = vec![
        (vault.has_encrypted_recipient, vault.encrypted_recipient),
        (vault.has_encrypted_amount, vault.encrypted_amount),
        (vault.has_encrypted_name, vault.encrypted_name),
        (vault.has_encrypted_condition_value, vault.encrypted_condition_value),
        (vault.has_encrypted_condition_salt, vault.encrypted_condition_salt),
        (vault.has_encrypted_fallback, vault.encrypted_fallback),
    ];

    for (is_set, handle) in fields_to_grant {
        if is_set && ra.len() >= ra_offset + 2 {
            helpers::grant_allow_with_pda_signer(
                &inco.to_account_info(),
                vault_info,
                sys,
                &ra[ra_offset],
                &ra[ra_offset + 1],
                handle,
                caller_key,
                seeds,
            )?;
            ra_offset += 2;
        }
    }

    // Grant access for extra condition encrypted values
    for cond in &extra_conditions {
        if cond.has_encrypted_value && ra.len() >= ra_offset + 2 {
            helpers::grant_allow_with_pda_signer(
                &inco.to_account_info(),
                vault_info,
                sys,
                &ra[ra_offset],
                &ra[ra_offset + 1],
                cond.encrypted_value,
                caller_key,
                seeds,
            )?;
            ra_offset += 2;
        }
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct GrantDecryptionRights<'info> {
    #[account(
        seeds = [b"vault", vault_id.to_le_bytes().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub caller: Signer<'info>,

    /// CHECK: Inco Lightning program
    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    // remaining_accounts: ExtraCondition PDAs + condition data + FHE allowance accounts
}

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: deposit_for_condition
// ═══════════════════════════════════════════════════════════════════════
//
// Deposits SOL into a vault's condition tracker (for IncomingTransaction trigger).
// Mirrors EVM VaultActions.depositForCondition().
//
// The deposit is tracked separately in a ConditionDepositTracker PDA.
// Anyone can contribute deposits that satisfy the IncomingTransaction threshold.

pub fn deposit_for_condition(
    ctx: Context<DepositForCondition>,
    vault_id: u64,
    amount: u64,
) -> Result<()> {
    let vault = &ctx.accounts.vault;
    require!(vault.status == VaultStatus::Locked as u8, VaultError::VaultNotLocked);
    require!(amount > 0, VaultError::InsufficientDeposit);

    // Transfer SOL from depositor to vault PDA
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.depositor.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );
    system_program::transfer(cpi_ctx, amount)?;

    // Update deposit tracker
    let tracker = &mut ctx.accounts.deposit_tracker;
    tracker.vault_id = vault_id;
    tracker.token = Pubkey::default(); // Native SOL
    tracker.total_amount = tracker.total_amount.checked_add(amount).ok_or(VaultError::Overflow)?;
    tracker.bump = ctx.bumps.deposit_tracker;

    // Update depositor's activity (mirrors EVM: lastActivity[msg.sender] = block.timestamp)
    let clock = Clock::get()?;
    let activity = &mut ctx.accounts.activity_tracker;
    activity.user = ctx.accounts.depositor.key();
    activity.timestamp = clock.unix_timestamp;
    activity.bump = ctx.bumps.activity_tracker;

    emit!(ConditionDeposited {
        vault_id,
        depositor: ctx.accounts.depositor.key(),
        token: Pubkey::default(),
        amount,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct DepositForCondition<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault_id.to_le_bytes().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = 8 + ConditionDepositTracker::LEN,
        seeds = [b"vault_deposits", vault_id.to_le_bytes().as_ref(), Pubkey::default().as_ref()],
        bump,
    )]
    pub deposit_tracker: Account<'info, ConditionDepositTracker>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = 8 + ActivityTracker::LEN,
        seeds = [b"last_activity", depositor.key().as_ref()],
        bump,
    )]
    pub activity_tracker: Account<'info, ActivityTracker>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: record_activity_by_sig
// ═══════════════════════════════════════════════════════════════════════
//
// Records wallet activity via Ed25519 signature verification.
// Allows a third party (relayer) to record activity for a wallet.
// Mirrors EVM VaultActions.recordActivityBySig().
//
// The transaction MUST include an Ed25519 program instruction (index 0)
// that verifies: wallet signed message = keccak256(wallet || nonce || deadline || program_id)

pub fn record_activity_by_sig(
    ctx: Context<RecordActivityBySig>,
    wallet: Pubkey,
    nonce: u64,
    deadline: i64,
) -> Result<()> {
    let clock = Clock::get()?;
    require!(clock.unix_timestamp <= deadline, VaultError::SignatureExpired);

    // Verify nonce
    let nonce_account = &mut ctx.accounts.activity_nonce;
    if nonce_account.wallet == Pubkey::default() {
        nonce_account.wallet = wallet;
        nonce_account.nonce = 0;
        nonce_account.bump = ctx.bumps.activity_nonce;
    }
    require!(nonce == nonce_account.nonce, VaultError::InvalidNonce);

    // Verify Ed25519 signature from instruction sysvar
    let expected_message = compute_activity_message(&wallet, nonce, deadline);
    verify_ed25519_signature(
        &ctx.accounts.instruction_sysvar,
        &wallet,
        &expected_message,
    )?;

    // Increment nonce (replay protection)
    nonce_account.nonce += 1;

    // Update activity tracker
    let tracker = &mut ctx.accounts.activity_tracker;
    tracker.user = wallet;
    tracker.timestamp = clock.unix_timestamp;
    tracker.bump = ctx.bumps.activity_tracker;

    emit!(ActivityRecorded {
        user: wallet,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(wallet: Pubkey)]
pub struct RecordActivityBySig<'info> {
    #[account(
        init_if_needed,
        payer = relayer,
        space = 8 + ActivityTracker::LEN,
        seeds = [b"last_activity", wallet.as_ref()],
        bump,
    )]
    pub activity_tracker: Account<'info, ActivityTracker>,

    #[account(
        init_if_needed,
        payer = relayer,
        space = 8 + ActivityNonce::LEN,
        seeds = [b"activity_nonce", wallet.as_ref()],
        bump,
    )]
    pub activity_nonce: Account<'info, ActivityNonce>,

    #[account(mut)]
    pub relayer: Signer<'info>,

    /// CHECK: Instructions sysvar for Ed25519 verification
    #[account(address = ix_sysvar::ID)]
    pub instruction_sysvar: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// ═══════════════════════════════════════════════════════════════════════
// INTERNAL HELPERS
// ═══════════════════════════════════════════════════════════════════════

/// Count how many FHE field pairs are needed for grant_decryption_rights
fn count_fhe_pairs(vault: &Vault) -> usize {
    let mut count = 0;
    if vault.has_encrypted_recipient { count += 1; }
    if vault.has_encrypted_amount { count += 1; }
    if vault.has_encrypted_name { count += 1; }
    if vault.has_encrypted_condition_value { count += 1; }
    if vault.has_encrypted_condition_salt { count += 1; }
    if vault.has_encrypted_fallback { count += 1; }
    count
}

/// Collect extra condition accounts from remaining_accounts.
/// Deserializes ExtraCondition PDAs at indices [0..extra_conditions_count).
fn collect_extra_conditions(
    remaining_accounts: &[AccountInfo],
    vault: &Vault,
) -> Result<Vec<ExtraCondition>> {
    let count = vault.extra_conditions_count as usize;
    if count == 0 {
        return Ok(Vec::new());
    }

    let mut extras = Vec::with_capacity(count);
    for i in 0..count {
        if i >= remaining_accounts.len() {
            return Err(VaultError::InvalidExtraConditionAccounts.into());
        }

        let account = &remaining_accounts[i];
        let data = account.try_borrow_data()
            .map_err(|_| VaultError::InvalidExtraConditionAccounts)?;

        if data.len() < 8 + ExtraCondition::LEN {
            return Err(VaultError::InvalidExtraConditionAccounts.into());
        }

        // Manually deserialize ExtraCondition (skip 8-byte discriminator)
        let d = &data[8..];
        let vault_id = u64::from_le_bytes(d[0..8].try_into().unwrap());
        let index = d[8];
        let condition_type = d[9];
        let value = u64::from_le_bytes(d[10..18].try_into().unwrap());
        let monitoring_address = Pubkey::try_from(&d[18..50]).unwrap();
        let token_address = Pubkey::try_from(&d[50..82]).unwrap();
        let condition_param = u64::from_le_bytes(d[82..90].try_into().unwrap());
        let has_encrypted_value = d[90] != 0;
        let enc_bytes: [u8; 16] = d[91..107].try_into().unwrap();
        let encrypted_value = inco_lightning::types::Euint128(
            u128::from_le_bytes(enc_bytes)
        );
        let value_commit: [u8; 32] = d[107..139].try_into().unwrap();
        let bump = d[139];

        if vault_id != vault.id || index != i as u8 {
            return Err(VaultError::InvalidExtraConditionAccounts.into());
        }

        extras.push(ExtraCondition {
            vault_id,
            index,
            condition_type,
            value,
            monitoring_address,
            token_address,
            condition_param,
            has_encrypted_value,
            encrypted_value,
            value_commit,
            bump,
        });
    }

    Ok(extras)
}

/// Collect activity timestamps and monitoring data from remaining_accounts.
///
/// Accounts layout (after ExtraCondition PDAs):
///   index `extra_count + 0`: primary condition data
///   index `extra_count + 1`: extra condition 0 data
///   index `extra_count + N`: extra condition N-1 data
///
/// Per condition type:
///   - Inactivity: ActivityTracker PDA → read timestamp
///   - BalanceBelow: monitoring account → read lamport balance
///   - IncomingTransaction: ConditionDepositTracker PDA → read total_amount
///   - ReleaseAtDate: any account (not read)
fn collect_condition_data(
    remaining_accounts: &[AccountInfo],
    vault: &Vault,
    extra_conditions: &[ExtraCondition],
) -> (Vec<i64>, Vec<u64>) {
    let extra_count = vault.extra_conditions_count as usize;
    let total_conditions = 1 + extra_count;
    let data_start = extra_count;

    let mut activity_ts = vec![0i64; total_conditions];
    let mut deposit_or_balance = vec![0u64; total_conditions];

    // Primary condition
    if remaining_accounts.len() > data_start {
        let acct = &remaining_accounts[data_start];
        read_condition_data(vault.condition_type, acct, &mut activity_ts[0], &mut deposit_or_balance[0]);
    }

    // Extra conditions
    for (i, cond) in extra_conditions.iter().enumerate() {
        let idx = data_start + 1 + i;
        if idx >= remaining_accounts.len() {
            break;
        }
        let acct = &remaining_accounts[idx];
        read_condition_data(cond.condition_type, acct, &mut activity_ts[i + 1], &mut deposit_or_balance[i + 1]);
    }

    (activity_ts, deposit_or_balance)
}

/// Read condition-specific data from a remaining account.
fn read_condition_data(
    condition_type: u8,
    account: &AccountInfo,
    activity_ts: &mut i64,
    deposit_or_balance: &mut u64,
) {
    match condition_type {
        t if t == ConditionType::Inactivity as u8 => {
            *activity_ts = read_activity_timestamp(account);
        }
        t if t == ConditionType::BalanceBelow as u8 => {
            *deposit_or_balance = account.lamports();
        }
        t if t == ConditionType::IncomingTransaction as u8 => {
            *deposit_or_balance = read_deposit_total(account);
        }
        _ => {} // ReleaseAtDate — uses Clock, no account data needed
    }
}

/// Read activity timestamp from an ActivityTracker account.
fn read_activity_timestamp(account: &AccountInfo) -> i64 {
    if let Ok(data) = account.try_borrow_data() {
        if data.len() >= 8 + ActivityTracker::LEN {
            // Skip 8-byte discriminator + 32-byte user pubkey → read i64 timestamp
            if let Ok(ts_bytes) = <[u8; 8]>::try_from(&data[40..48]) {
                return i64::from_le_bytes(ts_bytes);
            }
        }
    }
    0
}

/// Read total deposit amount from a ConditionDepositTracker account.
fn read_deposit_total(account: &AccountInfo) -> u64 {
    if let Ok(data) = account.try_borrow_data() {
        // Layout: 8 discriminator + 8 vault_id + 32 token + 8 total_amount
        if data.len() >= 8 + ConditionDepositTracker::LEN {
            if let Ok(total_bytes) = <[u8; 8]>::try_from(&data[48..56]) {
                return u64::from_le_bytes(total_bytes);
            }
        }
    }
    0
}

// ═══════════════════════════════════════════════════════════════════════
// Ed25519 SIGNATURE VERIFICATION HELPERS
// ═══════════════════════════════════════════════════════════════════════

/// Compute the activity message that the wallet must sign.
/// Message = keccak256(wallet || nonce || deadline || program_id)
fn compute_activity_message(wallet: &Pubkey, nonce: u64, deadline: i64) -> [u8; 32] {
    keccak::hashv(&[
        wallet.as_ref(),
        &nonce.to_le_bytes(),
        &deadline.to_le_bytes(),
        crate::ID.as_ref(),
    ]).to_bytes()
}

/// Verify that the transaction includes a valid Ed25519 program instruction
/// that proves `expected_signer` signed `expected_message`.
///
/// The Ed25519 instruction must be at index 0 in the transaction.
fn verify_ed25519_signature(
    instruction_sysvar: &AccountInfo,
    expected_signer: &Pubkey,
    expected_message: &[u8; 32],
) -> Result<()> {
    let ix = ix_sysvar::load_instruction_at_checked(0, instruction_sysvar)
        .map_err(|_| VaultError::InvalidSignature)?;

    require!(
        ix.program_id == ed25519_program::id(),
        VaultError::InvalidSignature
    );

    let data = &ix.data;
    require!(data.len() >= 16, VaultError::InvalidSignature);

    let num_signatures = data[0];
    require!(num_signatures >= 1, VaultError::InvalidSignature);

    // Parse first signature descriptor
    let pubkey_offset = u16::from_le_bytes([data[6], data[7]]) as usize;
    let msg_offset = u16::from_le_bytes([data[10], data[11]]) as usize;
    let msg_size = u16::from_le_bytes([data[12], data[13]]) as usize;

    // Verify public key matches expected signer
    require!(data.len() >= pubkey_offset + 32, VaultError::InvalidSignature);
    let pubkey_bytes = &data[pubkey_offset..pubkey_offset + 32];
    require!(pubkey_bytes == expected_signer.as_ref(), VaultError::InvalidSignature);

    // Verify message matches
    require!(data.len() >= msg_offset + msg_size, VaultError::InvalidSignature);
    require!(msg_size == 32, VaultError::InvalidSignature);
    let msg_bytes = &data[msg_offset..msg_offset + msg_size];
    require!(msg_bytes == expected_message.as_ref(), VaultError::InvalidSignature);

    Ok(())
}
