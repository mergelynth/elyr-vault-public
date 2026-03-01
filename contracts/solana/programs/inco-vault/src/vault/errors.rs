use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Vault not found")]
    VaultNotFound,

    #[msg("Vault is not in locked state")]
    VaultNotLocked,

    #[msg("Unlock conditions are not met")]
    ConditionsNotMet,

    #[msg("Invalid unlock time — must be in the future")]
    InvalidUnlockTime,

    #[msg("Invalid deadline — must be after unlock time")]
    InvalidDeadline,

    #[msg("Invalid recipient")]
    InvalidRecipient,

    #[msg("Deadline has not been reached yet")]
    DeadlineNotReached,

    #[msg("Claim period has expired (past deadline)")]
    ClaimPeriodExpired,

    #[msg("Insufficient deposit amount")]
    InsufficientDeposit,

    #[msg("Insufficient fee for FHE operations")]
    InsufficientFee,

    #[msg("SOL transfer failed")]
    TransferFailed,

    #[msg("Not authorized for refund")]
    NotAuthorizedForRefund,

    #[msg("Invalid condition reveal (commit-reveal mismatch)")]
    InvalidConditionReveal,

    #[msg("Invalid vault ID (expected next sequential ID)")]
    InvalidVaultId,

    #[msg("Vault type not supported for this operation")]
    InvalidVaultType,

    #[msg("Invalid vault type value")]
    InvalidVaultTypeValue,

    #[msg("Only the creator can perform this action")]
    CreatorOnly,

    #[msg("Extra conditions limit reached (max 3 extra)")]
    TooManyConditions,

    #[msg("Secret chunks limit reached")]
    TooManySecretChunks,

    #[msg("Name too long (max 32 bytes)")]
    NameTooLong,

    #[msg("Vault name length exceeds the name buffer")]
    InvalidNameLength,

    #[msg("No conditions provided")]
    NoConditions,

    #[msg("Arithmetic overflow")]
    Overflow,

    #[msg("Condition count mismatch during commit verification")]
    ConditionCountMismatch,

    #[msg("Encrypted field not supported for this vault type")]
    UnsupportedEncryptedField,

    #[msg("Too many observers (max 10)")]
    TooManyObservers,

    #[msg("Observer list is full")]
    ObserverListFull,

    #[msg("Signature has expired")]
    SignatureExpired,

    #[msg("Invalid nonce for activity signature")]
    InvalidNonce,

    #[msg("Invalid Ed25519 signature")]
    InvalidSignature,

    #[msg("SPL token transfer failed")]
    SplTransferFailed,

    #[msg("Invalid remaining accounts for extra conditions")]
    InvalidExtraConditionAccounts,
}
