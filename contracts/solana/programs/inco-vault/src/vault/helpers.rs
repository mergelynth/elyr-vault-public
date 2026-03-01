use anchor_lang::prelude::*;
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
