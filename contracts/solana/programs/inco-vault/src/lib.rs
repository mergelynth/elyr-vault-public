#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod vault;
pub use vault::*;

declare_id!("8kKZoqm42xJtu1JWvH1ZeoLsucVyUKpGfmhrY2eBHjBK");

#[program]
pub mod inco_token {
    use super::*;

    // ========== VAULT INSTRUCTIONS ==========

    /// Initialize the global vault counter (one-time admin setup)
    pub fn initialize_vault_counter(ctx: Context<InitializeVaultCounter>) -> Result<()> {
        vault::creation::initialize_vault_counter(ctx)
    }

    /// Create a vault with SOL deposit + primary condition
    pub fn create_vault(ctx: Context<CreateVault>, args: vault::types::CreateVaultArgs) -> Result<()> {
        vault::creation::create_vault(ctx, args)
    }

    /// Add encrypted field to an existing vault (FHE)
    /// remaining_accounts: [vault_allowance, vault_address, creator_allowance, creator_address]
    pub fn set_encrypted_field<'info>(
        ctx: Context<'_, '_, '_, 'info, SetEncryptedField<'info>>,
        vault_id: u64,
        field_type: u8,
        ciphertext: Vec<u8>,
        input_type: u8,
    ) -> Result<()> {
        vault::creation::set_encrypted_field(ctx, vault_id, field_type, ciphertext, input_type)
    }

    /// Add an extra condition to a vault
    pub fn add_extra_condition(
        ctx: Context<AddExtraCondition>,
        vault_id: u64,
        condition: vault::types::ConditionInput,
    ) -> Result<()> {
        vault::creation::add_extra_condition(ctx, vault_id, condition)
    }

    /// Add encrypted secret chunk to a vault
    /// remaining_accounts: [vault_allowance, vault_address, creator_allowance, creator_address]
    pub fn add_secret_chunk<'info>(
        ctx: Context<'_, '_, '_, 'info, AddSecretChunk<'info>>,
        vault_id: u64,
        ciphertext: Vec<u8>,
        input_type: u8,
    ) -> Result<()> {
        vault::creation::add_secret_chunk(ctx, vault_id, ciphertext, input_type)
    }

    /// Add an observer to a vault's observer list
    pub fn add_observer(
        ctx: Context<AddObserver>,
        vault_id: u64,
        observer: Pubkey,
    ) -> Result<()> {
        vault::creation::add_observer(ctx, vault_id, observer)
    }

    /// Claim vault assets (verify recipient + check conditions)
    pub fn claim_vault<'info>(
        ctx: Context<'_, '_, '_, 'info, ClaimVault<'info>>,
        vault_id: u64,
        claim_salt: [u8; 32],
        condition_salt: [u8; 32],
        condition_values: Vec<u64>,
    ) -> Result<()> {
        vault::actions::claim(ctx, vault_id, claim_salt, condition_salt, condition_values)
    }

    /// Refund vault after deadline
    /// remaining_accounts (SPL): [vault_token_account, caller_token_account, token_program]
    pub fn refund_vault<'info>(
        ctx: Context<'_, '_, '_, 'info, RefundVault<'info>>,
        vault_id: u64,
        refund_salt: [u8; 32],
    ) -> Result<()> {
        vault::actions::refund(ctx, vault_id, refund_salt)
    }

    /// Record wallet activity (resets inactivity timer)
    pub fn record_activity(ctx: Context<RecordActivity>) -> Result<()> {
        vault::actions::record_activity(ctx)
    }

    /// Grant FHE decryption rights (for secret vaults after conditions met)
    /// remaining_accounts: ExtraCondition PDAs + condition data + FHE [allowance, address] pairs
    pub fn grant_decryption_rights<'info>(
        ctx: Context<'_, '_, '_, 'info, GrantDecryptionRights<'info>>,
        vault_id: u64,
        claim_salt: [u8; 32],
        condition_salt: [u8; 32],
        condition_values: Vec<u64>,
    ) -> Result<()> {
        vault::actions::grant_decryption_rights(ctx, vault_id, claim_salt, condition_salt, condition_values)
    }

    /// Deposit SOL for IncomingTransaction condition trigger
    pub fn deposit_for_condition(
        ctx: Context<DepositForCondition>,
        vault_id: u64,
        amount: u64,
    ) -> Result<()> {
        vault::actions::deposit_for_condition(ctx, vault_id, amount)
    }

    /// Record wallet activity via Ed25519 signature (relayer pattern)
    pub fn record_activity_by_sig(
        ctx: Context<RecordActivityBySig>,
        wallet: Pubkey,
        nonce: u64,
        deadline: i64,
    ) -> Result<()> {
        vault::actions::record_activity_by_sig(ctx, wallet, nonce, deadline)
    }
}
