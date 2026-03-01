use anchor_lang::prelude::*;
use inco_lightning::types::Euint128;

// ─── Vault Counter (global PDA) ─────────────────────────────────────
// Seeds: ["vault_counter"]

#[account]
pub struct VaultCounter {
    pub count: u64,       // Current vault count (incremented on each creation)
    pub authority: Pubkey, // Admin who initialized
    pub bump: u8,
}

impl VaultCounter {
    pub const LEN: usize = 8 + 32 + 1; // 41
}

// ─── Vault (per-vault PDA) ──────────────────────────────────────────
// Seeds: ["vault", vault_id.to_le_bytes()]

#[account]
pub struct Vault {
    // ── Identity ─────────────────────────────────────
    pub id: u64,
    pub creator: Pubkey,
    pub vault_type: u8,       // 0=Asset, 1=Secret, 2=Hybrid
    pub status: u8,           // 0=Locked, 1=Claimed, 2=Refunded
    pub privacy_flags: u8,
    pub name: [u8; 32],       // UTF-8 name, zero-padded
    pub name_len: u8,

    // ── Recipient / Fallback ─────────────────────────
    pub recipient_hash: [u8; 32], // keccak256(recipient_pubkey)
    pub fallback_hash: [u8; 32],  // keccak256(fallback_pubkey) or [0;32]
    pub recipient_plain: Pubkey,  // Plaintext recipient (when not encrypted)
    pub fallback_plain: Pubkey,   // Plaintext fallback (when not encrypted)

    // ── Timing ───────────────────────────────────────
    pub deadline: i64,        // After this, refund is allowed (0 = no deadline)
    pub created_at: i64,

    // ── Deposit ──────────────────────────────────────
    pub deposit_token: Pubkey, // Pubkey::default() = native SOL
    pub deposit_amount: u64,   // Actual deposited amount (lamports or token units)
    pub is_confidential_token: bool,

    // ── Primary Condition (first condition, inline) ──
    pub condition_type: u8,
    pub unlock_value: u64,        // timestamp / duration / threshold
    pub monitoring_address: Pubkey,
    pub condition_token: Pubkey,  // Token for condition eval (default = SOL)
    pub condition_param: u64,     // Snapshot for IncomingTransaction

    // ── Encrypted Field Flags ────────────────────────
    pub has_encrypted_recipient: bool,
    pub has_encrypted_amount: bool,
    pub has_encrypted_name: bool,
    pub has_encrypted_condition_value: bool,
    pub has_encrypted_deposit: bool,
    pub has_encrypted_condition_salt: bool,
    pub has_encrypted_fallback: bool,

    // ── Encrypted Handles (Euint128 from Inco) ──────
    pub encrypted_recipient: Euint128,
    pub encrypted_amount: Euint128,
    pub encrypted_name: Euint128,
    pub encrypted_condition_value: Euint128,
    pub encrypted_deposit: Euint128,
    pub encrypted_condition_salt: Euint128,
    pub encrypted_fallback: Euint128,

    // ── Multi-condition / secret support ─────────────
    pub extra_conditions_count: u8,
    pub secret_chunks_count: u8,

    // ── Condition commit-reveal (max 4 conditions) ───
    pub condition_commits_count: u8,
    pub condition_value_commits: [[u8; 32]; 4],

    pub bump: u8,
}

impl Vault {
    pub const LEN: usize =
        8 +       // id
        32 +      // creator
        1 +       // vault_type
        1 +       // status
        1 +       // privacy_flags
        32 +      // name
        1 +       // name_len
        32 +      // recipient_hash
        32 +      // fallback_hash
        32 +      // recipient_plain
        32 +      // fallback_plain
        8 +       // deadline
        8 +       // created_at
        32 +      // deposit_token
        8 +       // deposit_amount
        1 +       // is_confidential_token
        1 +       // condition_type
        8 +       // unlock_value
        32 +      // monitoring_address
        32 +      // condition_token
        8 +       // condition_param
        7 +       // 7 bool flags
        7 * 32 +  // 7 Euint128 handles (32 bytes each)
        1 +       // extra_conditions_count
        1 +       // secret_chunks_count
        1 +       // condition_commits_count
        4 * 32 +  // condition_value_commits
        1;        // bump
    // Total: 724 bytes
}

// ─── Extra Condition (per additional condition PDA) ──────────────────
// Seeds: ["vault_condition", vault_id.to_le_bytes(), index as u8]

#[account]
pub struct ExtraCondition {
    pub vault_id: u64,
    pub index: u8,
    pub condition_type: u8,
    pub value: u64,               // 0 when encrypted
    pub monitoring_address: Pubkey,
    pub token_address: Pubkey,
    pub condition_param: u64,     // Snapshot for IncomingTransaction
    pub has_encrypted_value: bool,
    pub encrypted_value: Euint128,
    pub value_commit: [u8; 32],
    pub bump: u8,
}

impl ExtraCondition {
    pub const LEN: usize =
        8 +       // vault_id
        1 +       // index
        1 +       // condition_type
        8 +       // value
        32 +      // monitoring_address
        32 +      // token_address
        8 +       // condition_param
        1 +       // has_encrypted_value
        32 +      // encrypted_value
        32 +      // value_commit
        1;        // bump
    // Total: 156 bytes
}

// ─── Secret Chunk (encrypted data chunk PDA) ─────────────────────────
// Seeds: ["vault_secret", vault_id.to_le_bytes(), index as u8]

#[account]
pub struct SecretChunk {
    pub vault_id: u64,
    pub index: u8,
    pub data: Euint128,
    pub bump: u8,
}

impl SecretChunk {
    pub const LEN: usize =
        8 +       // vault_id
        1 +       // index
        32 +      // data (Euint128)
        1;        // bump
    // Total: 42 bytes
}

// ─── Activity Tracker (per user PDA for inactivity) ──────────────────
// Seeds: ["last_activity", user_pubkey]

#[account]
pub struct ActivityTracker {
    pub user: Pubkey,
    pub timestamp: i64,
    pub bump: u8,
}

impl ActivityTracker {
    pub const LEN: usize = 32 + 8 + 1; // 41
}

// ─── Observer List (per-vault PDA for observers) ─────────────────────
// Seeds: ["vault_observers", vault_id.to_le_bytes()]

#[account]
pub struct ObserverList {
    pub vault_id: u64,
    pub observers: Vec<Pubkey>,
    pub bump: u8,
}

impl ObserverList {
    /// Max 10 observers per vault (10 * 32 + 8 + 4 + 1 = 333)
    pub const MAX_OBSERVERS: usize = 10;
    pub const LEN: usize =
        8 +                       // vault_id
        4 + (32 * Self::MAX_OBSERVERS) + // Vec<Pubkey> (4 bytes for Vec len prefix + max 10 pubkeys)
        1;                        // bump
    // Total: 333 bytes
}

// ─── Condition Deposit Tracker (per-vault deposit tracking for triggers) ─
// Seeds: ["vault_deposits", vault_id.to_le_bytes(), token.as_ref()]
// Mirrors EVM: conditionDeposits[vaultId][tokenAddress]

#[account]
pub struct ConditionDepositTracker {
    pub vault_id: u64,
    pub token: Pubkey,        // Pubkey::default() = native SOL
    pub total_amount: u64,
    pub bump: u8,
}

impl ConditionDepositTracker {
    pub const LEN: usize = 8 + 32 + 8 + 1; // 49
}

// ─── Activity Nonce (per-wallet nonce for recordActivityBySig) ───────
// Seeds: ["activity_nonce", wallet.as_ref()]
// Mirrors EVM: activityNonces[wallet]

#[account]
pub struct ActivityNonce {
    pub wallet: Pubkey,
    pub nonce: u64,
    pub bump: u8,
}

impl ActivityNonce {
    pub const LEN: usize = 32 + 8 + 1; // 41
}
