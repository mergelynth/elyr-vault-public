use anchor_lang::prelude::*;

// ─── Condition Types (matches EVM VaultTypes.ConditionType) ──────────

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConditionType {
    ReleaseAtDate = 0,      // Unlocks at specific unix timestamp
    Inactivity = 1,         // Unlocks after creator inactivity period (seconds)
    BalanceBelow = 2,       // Unlocks when monitored address balance falls below threshold
    IncomingTransaction = 3, // Unlocks when monitored address receives tokens
}

// ─── Vault Status (matches EVM VaultTypes.VaultStatus) ───────────────

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VaultStatus {
    Locked = 0,    // Active, waiting for conditions
    Claimed = 1,   // Recipient claimed the assets
    Refunded = 2,  // Creator/fallback refunded after deadline
}

// ─── Vault Type ──────────────────────────────────────────────────────

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VaultType {
    Asset = 0,   // SOL or SPL token deposit
    Secret = 1,  // Encrypted secret data only
    Hybrid = 2,  // Asset + secret data
}

// ─── Privacy Flags (matches EVM bit flags) ───────────────────────────
//
// Bit 0 (0x01): encryptRecipient
// Bit 1 (0x02): encryptAmount
// Bit 2 (0x04): encryptFallback
// Bit 3 (0x08): encryptName
// Bit 4 (0x10): encryptConditions
//
pub const PRIVACY_ENCRYPT_RECIPIENT: u8   = 0x01;
pub const PRIVACY_ENCRYPT_AMOUNT: u8      = 0x02;
pub const PRIVACY_ENCRYPT_FALLBACK: u8    = 0x04;
pub const PRIVACY_ENCRYPT_NAME: u8        = 0x08;
pub const PRIVACY_ENCRYPT_CONDITIONS: u8  = 0x10;

// ─── Encrypted Field Selector ────────────────────────────────────────

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EncryptedFieldType {
    Recipient = 0,
    Amount = 1,
    Name = 2,
    ConditionValue = 3,
    ConditionSalt = 4,
    Deposit = 5,
    Fallback = 6,
}

// ─── Instruction Arg Structs ─────────────────────────────────────────

/// Condition input for vault creation (passed as instruction arg)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ConditionInput {
    pub condition_type: u8,
    pub value: u64,              // timestamp / duration(s) / threshold(lamports)
    pub monitoring_address: Pubkey,
    pub token_address: Pubkey,   // Pubkey::default() = native SOL
    pub value_commit: [u8; 32],  // keccak256(condIndex, value, salt) for commit-reveal
}

/// Args for create_vault instruction (flattened for tx size efficiency)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateVaultArgs {
    pub vault_id: u64,
    pub recipient: Pubkey,
    pub fallback_addr: Pubkey,
    pub name: [u8; 32],
    pub name_len: u8,
    pub deadline: i64,           // 0 = no deadline
    pub vault_type: u8,          // 0=Asset, 1=Secret, 2=Hybrid
    pub privacy_flags: u8,
    pub deposit_amount: u64,     // SOL lamports (0 for secret vaults)
    pub condition: ConditionInput,
    pub recipient_commit: [u8; 32],
    pub fallback_commit: [u8; 32],
}
