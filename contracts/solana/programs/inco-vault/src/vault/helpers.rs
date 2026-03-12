use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::keccak;
use inco_lightning::cpi::accounts::{Operation, Allow};
use inco_lightning::cpi::{new_euint128, allow};
use inco_lightning::types::Euint128;

use super::errors::VaultError;
use super::state::Vault;

// ─── Recipient Hash ──────────────────────────────────────────────────

/// Compute recipient hash: keccak256(pubkey_bytes)
/// Matches EVM: keccak256(abi.encodePacked(recipient))
pub fn compute_recipient_hash(pubkey: &Pubkey) -> [u8; 32] {
    keccak::hash(pubkey.as_ref()).to_bytes()
}

/// Compute salted hash: keccak256(salt || pubkey_bytes)
/// Matches EVM: keccak256(abi.encodePacked(claimSalt, msg.sender))
pub fn compute_salted_hash(salt: &[u8; 32], pubkey: &Pubkey) -> [u8; 32] {
    keccak::hashv(&[salt, pubkey.as_ref()]).to_bytes()
}

// ─── Recipient Verification ─────────────────────────────────────────

/// Verify the caller is the authorized recipient.
/// Path B (salt): keccak256(claim_salt, caller) == vault.recipient_hash
/// Path C (plain): keccak256(caller) == vault.recipient_hash
pub fn verify_recipient(
    vault: &Vault,
    caller: &Pubkey,
    claim_salt: &[u8; 32],
) -> Result<()> {
    let zero_salt = [0u8; 32];

    if *claim_salt != zero_salt {
        // Path B: Salt-based verification
        let hash = compute_salted_hash(claim_salt, caller);
        if hash != vault.recipient_hash {
            return Err(VaultError::InvalidRecipient.into());
        }
    } else {
        // Path C: Plaintext recipient verification
        let hash = compute_recipient_hash(caller);
        if hash != vault.recipient_hash {
            return Err(VaultError::InvalidRecipient.into());
        }
    }

    Ok(())
}

/// Verify fallback for refund (same logic as recipient but against fallback_hash)
pub fn verify_fallback(
    vault: &Vault,
    caller: &Pubkey,
    refund_salt: &[u8; 32],
) -> bool {
    let zero = [0u8; 32];
    if vault.fallback_hash == zero {
        return false;
    }

    if *refund_salt != zero {
        compute_salted_hash(refund_salt, caller) == vault.fallback_hash
    } else {
        compute_recipient_hash(caller) == vault.fallback_hash
    }
}

// ─── SOL Transfer ────────────────────────────────────────────────────

/// Transfer SOL from a PDA-owned account to a recipient.
/// The vault PDA is owned by our program, so we can directly manipulate lamports.
pub fn transfer_sol_from_vault<'info>(
    vault_account: &AccountInfo<'info>,
    recipient: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    **vault_account.try_borrow_mut_lamports()? = vault_account
        .lamports()
        .checked_sub(amount)
        .ok_or(VaultError::TransferFailed)?;
    **recipient.try_borrow_mut_lamports()? = recipient
        .lamports()
        .checked_add(amount)
        .ok_or(VaultError::Overflow)?;
    Ok(())
}

// ─── SPL Token Transfer ─────────────────────────────────────────────

/// Transfer SPL tokens from a vault PDA token account to a recipient token account.
/// Uses CPI with PDA signer seeds.
pub fn transfer_spl_from_vault<'info>(
    vault_token_account: &AccountInfo<'info>,
    recipient_token_account: &AccountInfo<'info>,
    vault_authority: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let seeds_slice = &[signer_seeds];
    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: *token_program.key,
        accounts: vec![
            anchor_lang::solana_program::instruction::AccountMeta::new(*vault_token_account.key, false),
            anchor_lang::solana_program::instruction::AccountMeta::new(*recipient_token_account.key, false),
            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(*vault_authority.key, true),
        ],
        data: {
            // SPL Token Transfer instruction: tag=3, amount as u64 LE
            let mut data = Vec::with_capacity(9);
            data.push(3); // Transfer instruction tag
            data.extend_from_slice(&amount.to_le_bytes());
            data
        },
    };
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            vault_token_account.clone(),
            recipient_token_account.clone(),
            vault_authority.clone(),
        ],
        seeds_slice,
    )?;
    Ok(())
}

// ─── FHE Helpers ─────────────────────────────────────────────────────

/// Encrypt a ciphertext via Inco Lightning CPI and return the handle.
pub fn encrypt_value<'info>(
    inco_program: &AccountInfo<'info>,
    signer: &AccountInfo<'info>,
    ciphertext: Vec<u8>,
    input_type: u8,
) -> Result<Euint128> {
    let cpi_ctx = CpiContext::new(
        inco_program.clone(),
        Operation { signer: signer.clone() },
    );
    let handle = new_euint128(cpi_ctx, ciphertext, input_type)?;
    Ok(handle)
}

/// Grant FHE decryption permission for a handle to an address.
/// Uses remaining_accounts: [allowance_account (mut), allowed_address_info]
pub fn grant_allow<'info>(
    inco_program: &AccountInfo<'info>,
    signer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    allowance_account: &AccountInfo<'info>,
    allowed_address_info: &AccountInfo<'info>,
    handle: Euint128,
    allowed_pubkey: Pubkey,
) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        inco_program.clone(),
        Allow {
            allowance_account: allowance_account.clone(),
            signer: signer.clone(),
            allowed_address: allowed_address_info.clone(),
            system_program: system_program.clone(),
        },
    );
    allow(cpi_ctx, handle.0, true, allowed_pubkey)?;
    Ok(())
}

/// Grant FHE decryption permission using PDA signer (for claim-time grants).
pub fn grant_allow_with_pda_signer<'info>(
    inco_program: &AccountInfo<'info>,
    vault_account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    allowance_account: &AccountInfo<'info>,
    allowed_address_info: &AccountInfo<'info>,
    handle: Euint128,
    allowed_pubkey: Pubkey,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let seeds_slice = &[signer_seeds];
    let cpi_ctx = CpiContext::new_with_signer(
        inco_program.clone(),
        Allow {
            allowance_account: allowance_account.clone(),
            signer: vault_account.clone(),
            allowed_address: allowed_address_info.clone(),
            system_program: system_program.clone(),
        },
        seeds_slice,
    );
    allow(cpi_ctx, handle.0, true, allowed_pubkey)?;
    Ok(())
}

// ─── Confidential Token (cToken) Helpers ─────────────────────────────

/// Inco Token Program wrap instruction discriminator: sha256("global:wrap")[0..8]
const WRAP_DISC: [u8; 8] = [0xb2, 0x28, 0x0a, 0xbd, 0xe4, 0x81, 0xba, 0x8c];
/// Inco Token Program transfer instruction discriminator: sha256("global:transfer")[0..8]
const CTOKEN_TRANSFER_DISC: [u8; 8] = [0xa3, 0x34, 0xc8, 0xe7, 0x8c, 0x03, 0x45, 0xba];

/// CPI wrap(0) to the Inco Token Program to create a Balance PDA for the vault.
///
/// remaining_accounts layout for cToken creation:
///   [offset+0] vault_balance_pda (mut) — PDA ["balance", vaultPda] on Inco Token Program
///   [offset+1] vault_spl_ata (mut) — vault PDA's ATA for base SPL mint
///   [offset+2] custody_ata (mut) — Inco Token Program's custody ATA
///   [offset+3] confidential_mint (mut) — Inco confidential mint account
///   [offset+4] base_spl_mint (mut) — base SPL mint
///   [offset+5] spl_token_program (readonly)
///   [offset+6] inco_token_program (readonly)
///   [offset+7] inco_lightning (readonly)
pub fn cpi_wrap_zero<'info>(
    remaining: &[AccountInfo<'info>],
    offset: usize,
    vault_authority: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    require!(
        remaining.len() >= offset + 8,
        VaultError::InsufficientAccounts
    );
    let vault_balance_pda = &remaining[offset];
    let vault_spl_ata    = &remaining[offset + 1];
    let custody_ata      = &remaining[offset + 2];
    let conf_mint        = &remaining[offset + 3];
    let base_mint        = &remaining[offset + 4];
    let spl_token_prog   = &remaining[offset + 5];
    let inco_token_prog  = &remaining[offset + 6];
    let inco_lightning    = &remaining[offset + 7];

    // Build wrap(0) instruction data: disc(8) + amount(u64 LE = 0)
    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(&WRAP_DISC);
    data.extend_from_slice(&0u64.to_le_bytes());

    let ix = solana_program::instruction::Instruction {
        program_id: *inco_token_prog.key,
        accounts: vec![
            solana_program::instruction::AccountMeta::new(*conf_mint.key, false),
            solana_program::instruction::AccountMeta::new(*vault_balance_pda.key, false),
            solana_program::instruction::AccountMeta::new(*vault_authority.key, true), // signer
            solana_program::instruction::AccountMeta::new(*vault_spl_ata.key, false),
            solana_program::instruction::AccountMeta::new(*custody_ata.key, false),
            solana_program::instruction::AccountMeta::new(*base_mint.key, false),
            solana_program::instruction::AccountMeta::new_readonly(*spl_token_prog.key, false),
            solana_program::instruction::AccountMeta::new_readonly(*system_program.key, false),
            solana_program::instruction::AccountMeta::new_readonly(*inco_lightning.key, false),
        ],
        data,
    };

    solana_program::program::invoke_signed(
        &ix,
        &[
            conf_mint.clone(),
            vault_balance_pda.clone(),
            vault_authority.clone(),
            vault_spl_ata.clone(),
            custody_ata.clone(),
            base_mint.clone(),
            spl_token_prog.clone(),
            system_program.clone(),
            inco_lightning.clone(),
        ],
        &[signer_seeds],
    )?;
    Ok(())
}

/// CPI transfer on the Inco Token Program to move cTokens between Balance PDAs.
/// Uses plaintext amount with input_type=0.
///
/// remaining_accounts layout for cToken claim/refund:
///   [offset+0] vault_balance_pda (mut) — source
///   [offset+1] recipient_balance_pda (mut) — destination
///   [offset+2] confidential_mint (mut)
///   [offset+3] inco_token_program (readonly)
pub fn cpi_transfer_ctoken<'info>(
    remaining: &[AccountInfo<'info>],
    offset: usize,
    vault_authority: &AccountInfo<'info>,
    recipient_wallet: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    inco_lightning: &AccountInfo<'info>,
    amount: u64,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    require!(
        remaining.len() >= offset + 4,
        VaultError::InsufficientAccounts
    );
    let vault_balance_pda    = &remaining[offset];
    let recipient_balance_pda = &remaining[offset + 1];
    let conf_mint            = &remaining[offset + 2];
    let inco_token_prog      = &remaining[offset + 3];

    // Build transfer instruction data: disc(8) + vec_len(4) + amount(8) + input_type(1)
    let mut data = Vec::with_capacity(21);
    data.extend_from_slice(&CTOKEN_TRANSFER_DISC);
    data.extend_from_slice(&8u32.to_le_bytes()); // vec length = 8 bytes
    data.extend_from_slice(&amount.to_le_bytes());
    data.push(0); // input_type = 0 (plaintext u64)

    let ix = solana_program::instruction::Instruction {
        program_id: *inco_token_prog.key,
        accounts: vec![
            solana_program::instruction::AccountMeta::new(*conf_mint.key, false),
            solana_program::instruction::AccountMeta::new(*vault_balance_pda.key, false),
            solana_program::instruction::AccountMeta::new(*recipient_balance_pda.key, false),
            solana_program::instruction::AccountMeta::new(*vault_authority.key, true), // signer
            solana_program::instruction::AccountMeta::new_readonly(*recipient_wallet.key, false),
            solana_program::instruction::AccountMeta::new_readonly(*system_program.key, false),
            solana_program::instruction::AccountMeta::new_readonly(*inco_lightning.key, false),
        ],
        data,
    };

    solana_program::program::invoke_signed(
        &ix,
        &[
            conf_mint.clone(),
            vault_balance_pda.clone(),
            recipient_balance_pda.clone(),
            vault_authority.clone(),
            recipient_wallet.clone(),
            system_program.clone(),
            inco_lightning.clone(),
        ],
        &[signer_seeds],
    )?;
    Ok(())
}

// ─── Account Close Helpers ───────────────────────────────────────────

/// Inco Token Program close_account instruction discriminator: sha256("global:close_account")[0..8]
const CLOSE_ACCOUNT_DISC: [u8; 8] = [0x7d, 0xff, 0x95, 0x0e, 0x6e, 0x22, 0x48, 0x18];

/// CPI close_account to the Inco Token Program to close a Balance PDA.
/// Transfers all lamports from the account to `destination`.
pub fn cpi_close_inco_account<'info>(
    inco_token_prog: &AccountInfo<'info>,
    account: &AccountInfo<'info>,
    destination: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let ix = solana_program::instruction::Instruction {
        program_id: *inco_token_prog.key,
        accounts: vec![
            solana_program::instruction::AccountMeta::new(*account.key, false),
            solana_program::instruction::AccountMeta::new(*destination.key, false),
            solana_program::instruction::AccountMeta::new(*authority.key, true),
        ],
        data: CLOSE_ACCOUNT_DISC.to_vec(),
    };
    solana_program::program::invoke_signed(
        &ix,
        &[account.clone(), destination.clone(), authority.clone()],
        &[signer_seeds],
    )?;
    Ok(())
}

/// Close an SPL token account via CPI to SPL Token Program.
/// Transfers all lamports from the account to `destination`.
pub fn cpi_close_spl_ata<'info>(
    spl_token_prog: &AccountInfo<'info>,
    account: &AccountInfo<'info>,
    destination: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let ix = solana_program::instruction::Instruction {
        program_id: *spl_token_prog.key,
        accounts: vec![
            solana_program::instruction::AccountMeta::new(*account.key, false),
            solana_program::instruction::AccountMeta::new(*destination.key, false),
            solana_program::instruction::AccountMeta::new_readonly(*authority.key, true),
        ],
        data: vec![9], // SPL Token CloseAccount instruction tag
    };
    solana_program::program::invoke_signed(
        &ix,
        &[account.clone(), destination.clone(), authority.clone()],
        &[signer_seeds],
    )?;
    Ok(())
}

/// Transfer all remaining lamports from a system-owned PDA to a destination.
/// Uses system_program::transfer CPI with PDA signer.
pub fn reclaim_pda_lamports<'info>(
    pda: &AccountInfo<'info>,
    destination: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let lamports = pda.lamports();
    if lamports > 0 {
        let ix = solana_program::system_instruction::transfer(
            pda.key,
            destination.key,
            lamports,
        );
        solana_program::program::invoke_signed(
            &ix,
            &[pda.clone(), destination.clone(), system_program.clone()],
            &[signer_seeds],
        )?;
    }
    Ok(())
}
