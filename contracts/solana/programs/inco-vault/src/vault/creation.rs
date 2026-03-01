use anchor_lang::prelude::*;
use anchor_lang::system_program;
use inco_lightning::ID as INCO_LIGHTNING_ID;

use super::types::*;
use super::state::*;
use super::errors::VaultError;
use super::events::*;
use super::helpers;

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: initialize_vault_counter
// ═══════════════════════════════════════════════════════════════════════

pub fn initialize_vault_counter(ctx: Context<InitializeVaultCounter>) -> Result<()> {
    let counter = &mut ctx.accounts.vault_counter;
    counter.count = 0;
    counter.authority = ctx.accounts.authority.key();
    counter.bump = ctx.bumps.vault_counter;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVaultCounter<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + VaultCounter::LEN,
        seeds = [b"vault_counter"],
        bump,
    )]
    pub vault_counter: Account<'info, VaultCounter>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: create_vault
// ═══════════════════════════════════════════════════════════════════════
//
// Creates a vault with SOL deposit + primary condition.
// For SPL token deposits or FHE encryption, use additional instructions
// after creation (set_encrypted_field, etc.).
//
// Matches EVM: createAssetVaultETH / createSecretVault (non-encrypted path)

pub fn create_vault(ctx: Context<CreateVault>, args: CreateVaultArgs) -> Result<()> {
    let counter = &mut ctx.accounts.vault_counter;
    let clock = Clock::get()?;

    // Validate vault ID is sequential
    require!(args.vault_id == counter.count + 1, VaultError::InvalidVaultId);

    // Validate vault type
    require!(
        args.vault_type <= 2,
        VaultError::InvalidVaultTypeValue
    );

    // Validate name length
    require!(args.name_len <= 32, VaultError::NameTooLong);

    // Validate condition
    let encrypt_conditions = (args.privacy_flags & PRIVACY_ENCRYPT_CONDITIONS) != 0;
    if !encrypt_conditions {
        if args.condition.condition_type == ConditionType::ReleaseAtDate as u8 {
            require!(
                args.condition.value as i64 > clock.unix_timestamp,
                VaultError::InvalidUnlockTime
            );
        }
        if args.deadline > 0 {
            require!(
                args.deadline > args.condition.value as i64,
                VaultError::InvalidDeadline
            );
        }
    }

    // Validate deposit for asset/hybrid vaults
    if args.vault_type == VaultType::Asset as u8 || args.vault_type == VaultType::Hybrid as u8 {
        require!(args.deposit_amount > 0, VaultError::InsufficientDeposit);
    }

    // Transfer SOL deposit from creator to vault PDA
    if args.deposit_amount > 0 {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.creator.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );
        system_program::transfer(cpi_ctx, args.deposit_amount)?;
    }

    // Increment counter
    counter.count = args.vault_id;

    // Initialize vault state
    let vault = &mut ctx.accounts.vault;
    vault.id = args.vault_id;
    vault.creator = ctx.accounts.creator.key();
    vault.vault_type = args.vault_type;
    vault.status = VaultStatus::Locked as u8;
    vault.privacy_flags = args.privacy_flags;
    vault.name = args.name;
    vault.name_len = args.name_len;
    vault.deadline = args.deadline;
    vault.created_at = clock.unix_timestamp;

    // Recipient
    let recipient_hash = helpers::compute_recipient_hash(&args.recipient);
    let encrypt_recipient = (args.privacy_flags & PRIVACY_ENCRYPT_RECIPIENT) != 0;

    if encrypt_recipient && args.recipient_commit != [0u8; 32] {
        vault.recipient_hash = args.recipient_commit;
    } else {
        vault.recipient_hash = recipient_hash;
    }

    if !encrypt_recipient {
        vault.recipient_plain = args.recipient;
    }

    // Fallback
    if args.fallback_addr != Pubkey::default() {
        let encrypt_fallback = (args.privacy_flags & PRIVACY_ENCRYPT_FALLBACK) != 0;
        if encrypt_fallback && args.fallback_commit != [0u8; 32] {
            vault.fallback_hash = args.fallback_commit;
        } else {
            vault.fallback_hash = helpers::compute_recipient_hash(&args.fallback_addr);
        }
        if !encrypt_fallback {
            vault.fallback_plain = args.fallback_addr;
        }
    }

    // Deposit
    vault.deposit_token = Pubkey::default(); // SOL (SPL via separate instruction)
    vault.deposit_amount = args.deposit_amount;

    // Primary condition
    let monitoring = if args.condition.monitoring_address != Pubkey::default() {
        args.condition.monitoring_address
    } else {
        ctx.accounts.creator.key()
    };
    vault.condition_type = args.condition.condition_type;
    vault.unlock_value = if encrypt_conditions { 0 } else { args.condition.value };
    vault.monitoring_address = monitoring;
    vault.condition_token = args.condition.token_address;

    // Snapshot for IncomingTransaction (SOL balance of monitored address)
    if args.condition.condition_type == ConditionType::IncomingTransaction as u8 {
        // For SOL: read monitoring account balance from remaining_accounts[0]
        if let Some(mon_account) = ctx.remaining_accounts.first() {
            vault.condition_param = mon_account.lamports();
        }
    }

    // Initialize activity for Inactivity conditions
    if args.condition.condition_type == ConditionType::Inactivity as u8 {
        // Activity tracker is managed separately via record_activity
        // We just note that inactivity tracking is needed for this vault
        vault.condition_param = 0; // Not used for inactivity
    }

    // Condition commit (for encrypted conditions)
    if encrypt_conditions && args.condition.value_commit != [0u8; 32] {
        vault.condition_value_commits[0] = args.condition.value_commit;
        vault.condition_commits_count = 1;
    }

    vault.extra_conditions_count = 0;
    vault.secret_chunks_count = 0;
    vault.bump = ctx.bumps.vault;

    // Mask values for event based on privacy flags
    let mask_name = (args.privacy_flags & PRIVACY_ENCRYPT_NAME) != 0;
    let mask_conditions = encrypt_conditions;
    let mask_amount = (args.privacy_flags & PRIVACY_ENCRYPT_AMOUNT) != 0;

    emit!(VaultCreated {
        vault_id: args.vault_id,
        creator: ctx.accounts.creator.key(),
        vault_type: args.vault_type,
        condition_type: args.condition.condition_type,
        unlock_value: if mask_conditions { 0 } else { args.condition.value },
        deadline: args.deadline,
        recipient_hash: vault.recipient_hash,
        name: if mask_name { [0u8; 32] } else { args.name },
        deposit_token: if mask_amount { Pubkey::default() } else { Pubkey::default() },
        deposit_amount: if mask_amount { 0 } else { args.deposit_amount },
        condition_token: if mask_conditions { Pubkey::default() } else { args.condition.token_address },
        privacy_flags: args.privacy_flags,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: CreateVaultArgs)]
pub struct CreateVault<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Vault::LEN,
        seeds = [b"vault", args.vault_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"vault_counter"],
        bump = vault_counter.bump,
    )]
    pub vault_counter: Account<'info, VaultCounter>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
    // remaining_accounts: [monitoring_account] for IncomingTransaction snapshot
}

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: set_encrypted_field
// ═══════════════════════════════════════════════════════════════════════
//
// Encrypts a vault field via Inco Lightning FHE.
// Must be called by the vault creator while vault is locked.
// remaining_accounts: FHE allowance accounts for allow() CPI calls

pub fn set_encrypted_field<'info>(
    ctx: Context<'_, '_, '_, 'info, SetEncryptedField<'info>>,
    vault_id: u64,
    field_type: u8,
    ciphertext: Vec<u8>,
    input_type: u8,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    require!(vault.status == VaultStatus::Locked as u8, VaultError::VaultNotLocked);
    require!(vault.creator == ctx.accounts.creator.key(), VaultError::CreatorOnly);

    let inco_program = &ctx.accounts.inco_lightning_program;
    let signer = &ctx.accounts.creator;

    // Encrypt the value
    let handle = helpers::encrypt_value(
        &inco_program.to_account_info(),
        &signer.to_account_info(),
        ciphertext,
        input_type,
    )?;

    // Grant allow to vault PDA (for later grant_decryption_rights)
    if ctx.remaining_accounts.len() >= 2 {
        helpers::grant_allow(
            &inco_program.to_account_info(),
            &signer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            &ctx.remaining_accounts[0],
            &ctx.remaining_accounts[1],
            handle,
            vault.key(),
        )?;
    }

    // Grant allow to creator
    if ctx.remaining_accounts.len() >= 4 {
        helpers::grant_allow(
            &inco_program.to_account_info(),
            &signer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            &ctx.remaining_accounts[2],
            &ctx.remaining_accounts[3],
            handle,
            ctx.accounts.creator.key(),
        )?;
    }

    // Store handle in the appropriate field
    match field_type {
        t if t == EncryptedFieldType::Recipient as u8 => {
            vault.encrypted_recipient = handle;
            vault.has_encrypted_recipient = true;
        }
        t if t == EncryptedFieldType::Amount as u8 => {
            vault.encrypted_amount = handle;
            vault.has_encrypted_amount = true;
        }
        t if t == EncryptedFieldType::Name as u8 => {
            vault.encrypted_name = handle;
            vault.has_encrypted_name = true;
        }
        t if t == EncryptedFieldType::ConditionValue as u8 => {
            vault.encrypted_condition_value = handle;
            vault.has_encrypted_condition_value = true;
        }
        t if t == EncryptedFieldType::ConditionSalt as u8 => {
            vault.encrypted_condition_salt = handle;
            vault.has_encrypted_condition_salt = true;
        }
        t if t == EncryptedFieldType::Deposit as u8 => {
            vault.encrypted_deposit = handle;
            vault.has_encrypted_deposit = true;
        }
        t if t == EncryptedFieldType::Fallback as u8 => {
            vault.encrypted_fallback = handle;
            vault.has_encrypted_fallback = true;
        }
        _ => return Err(VaultError::UnsupportedEncryptedField.into()),
    }

    emit!(EncryptedFieldSet {
        vault_id,
        field_type,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct SetEncryptedField<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault_id.to_le_bytes().as_ref()],
        bump = vault.bump,
        constraint = vault.creator == creator.key() @ VaultError::CreatorOnly,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: Inco Lightning program
    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    // remaining_accounts: [vault_allowance, vault_address, creator_allowance, creator_address]
}

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: add_extra_condition
// ═══════════════════════════════════════════════════════════════════════
//
// Adds an extra condition (index 1+) to an existing vault.
// Mirrors EVM _storeExtraConditions.

pub fn add_extra_condition(
    ctx: Context<AddExtraCondition>,
    vault_id: u64,
    condition: ConditionInput,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    require!(vault.status == VaultStatus::Locked as u8, VaultError::VaultNotLocked);
    require!(vault.creator == ctx.accounts.creator.key(), VaultError::CreatorOnly);
    require!(vault.extra_conditions_count < 3, VaultError::TooManyConditions);

    let clock = Clock::get()?;
    let encrypt_conditions = (vault.privacy_flags & PRIVACY_ENCRYPT_CONDITIONS) != 0;

    // Validate
    if !encrypt_conditions {
        if condition.condition_type == ConditionType::ReleaseAtDate as u8 {
            require!(
                condition.value as i64 > clock.unix_timestamp,
                VaultError::InvalidUnlockTime
            );
        }
    }

    let monitoring = if condition.monitoring_address != Pubkey::default() {
        condition.monitoring_address
    } else {
        ctx.accounts.creator.key()
    };

    let extra = &mut ctx.accounts.extra_condition;
    extra.vault_id = vault_id;
    extra.index = vault.extra_conditions_count;
    extra.condition_type = condition.condition_type;
    extra.value = if encrypt_conditions { 0 } else { condition.value };
    extra.monitoring_address = monitoring;
    extra.token_address = condition.token_address;
    extra.bump = ctx.bumps.extra_condition;

    // Snapshot for IncomingTransaction
    if condition.condition_type == ConditionType::IncomingTransaction as u8 {
        if let Some(mon_account) = ctx.remaining_accounts.first() {
            extra.condition_param = mon_account.lamports();
        }
    }

    // Store condition commit if encrypted
    if encrypt_conditions && condition.value_commit != [0u8; 32] {
        extra.value_commit = condition.value_commit;
        let idx = (vault.condition_commits_count) as usize;
        if idx < 4 {
            vault.condition_value_commits[idx] = condition.value_commit;
            vault.condition_commits_count += 1;
        }
    }

    let index = vault.extra_conditions_count;
    vault.extra_conditions_count += 1;

    emit!(ExtraConditionAdded {
        vault_id,
        index,
        condition_type: condition.condition_type,
        value: if encrypt_conditions { 0 } else { condition.value },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vault_id: u64, condition: ConditionInput)]
pub struct AddExtraCondition<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault_id.to_le_bytes().as_ref()],
        bump = vault.bump,
        constraint = vault.creator == creator.key() @ VaultError::CreatorOnly,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = creator,
        space = 8 + ExtraCondition::LEN,
        seeds = [b"vault_condition", vault_id.to_le_bytes().as_ref(), &[vault.extra_conditions_count]],
        bump,
    )]
    pub extra_condition: Account<'info, ExtraCondition>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
    // remaining_accounts: [monitoring_account] for IncomingTransaction snapshot
}

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: add_secret_chunk
// ═══════════════════════════════════════════════════════════════════════
//
// Adds an encrypted secret data chunk (for Secret/Hybrid vaults).
// remaining_accounts: FHE allowance accounts for allow() CPI calls

pub fn add_secret_chunk<'info>(
    ctx: Context<'_, '_, '_, 'info, AddSecretChunk<'info>>,
    vault_id: u64,
    ciphertext: Vec<u8>,
    input_type: u8,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    require!(vault.status == VaultStatus::Locked as u8, VaultError::VaultNotLocked);
    require!(vault.creator == ctx.accounts.creator.key(), VaultError::CreatorOnly);

    let inco_program = &ctx.accounts.inco_lightning_program;
    let signer = &ctx.accounts.creator;

    // Encrypt the secret chunk
    let handle = helpers::encrypt_value(
        &inco_program.to_account_info(),
        &signer.to_account_info(),
        ciphertext,
        input_type,
    )?;

    // Store encrypted chunk
    let chunk = &mut ctx.accounts.secret_chunk;
    chunk.vault_id = vault_id;
    chunk.index = vault.secret_chunks_count;
    chunk.data = handle;
    chunk.bump = ctx.bumps.secret_chunk;

    // Grant allow to vault PDA
    if ctx.remaining_accounts.len() >= 2 {
        helpers::grant_allow(
            &inco_program.to_account_info(),
            &signer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            &ctx.remaining_accounts[0],
            &ctx.remaining_accounts[1],
            handle,
            vault.key(),
        )?;
    }

    // Grant allow to creator
    if ctx.remaining_accounts.len() >= 4 {
        helpers::grant_allow(
            &inco_program.to_account_info(),
            &signer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            &ctx.remaining_accounts[2],
            &ctx.remaining_accounts[3],
            handle,
            ctx.accounts.creator.key(),
        )?;
    }

    let index = vault.secret_chunks_count;
    vault.secret_chunks_count += 1;

    // Update vault type to Hybrid if it was Asset and now has secrets
    if vault.vault_type == VaultType::Asset as u8 && vault.deposit_amount > 0 {
        vault.vault_type = VaultType::Hybrid as u8;
    }

    emit!(SecretChunkAdded { vault_id, index });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vault_id: u64)]
pub struct AddSecretChunk<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault_id.to_le_bytes().as_ref()],
        bump = vault.bump,
        constraint = vault.creator == creator.key() @ VaultError::CreatorOnly,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = creator,
        space = 8 + SecretChunk::LEN,
        seeds = [b"vault_secret", vault_id.to_le_bytes().as_ref(), &[vault.secret_chunks_count]],
        bump,
    )]
    pub secret_chunk: Account<'info, SecretChunk>,

    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: Inco Lightning program
    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    // remaining_accounts: [vault_allowance, vault_address, creator_allowance, creator_address]
}

// ═══════════════════════════════════════════════════════════════════════
// INSTRUCTION: add_observer
// ═══════════════════════════════════════════════════════════════════════
//
// Adds an observer to a vault's observer list PDA.
// Must be called by the vault creator while vault is locked.
// The ObserverList PDA is created on the first call (init_if_needed).

pub fn add_observer(
    ctx: Context<AddObserver>,
    vault_id: u64,
    observer: Pubkey,
) -> Result<()> {
    let vault = &ctx.accounts.vault;
    require!(vault.status == VaultStatus::Locked as u8, VaultError::VaultNotLocked);
    require!(vault.creator == ctx.accounts.creator.key(), VaultError::CreatorOnly);

    let observer_list = &mut ctx.accounts.observer_list;

    // Initialize on first call
    if observer_list.vault_id == 0 {
        observer_list.vault_id = vault_id;
        observer_list.observers = Vec::new();
        observer_list.bump = ctx.bumps.observer_list;
    }

    require!(
        observer_list.observers.len() < ObserverList::MAX_OBSERVERS,
        VaultError::ObserverListFull
    );

    // Avoid duplicates
    if !observer_list.observers.contains(&observer) {
        observer_list.observers.push(observer);
    }

    emit!(ObserverAdded {
        vault_id,
        observer,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vault_id: u64, observer: Pubkey)]
pub struct AddObserver<'info> {
    #[account(
        seeds = [b"vault", vault_id.to_le_bytes().as_ref()],
        bump = vault.bump,
        constraint = vault.creator == creator.key() @ VaultError::CreatorOnly,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init_if_needed,
        payer = creator,
        space = 8 + ObserverList::LEN,
        seeds = [b"vault_observers", vault_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub observer_list: Account<'info, ObserverList>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}
