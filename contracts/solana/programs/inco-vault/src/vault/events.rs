use anchor_lang::prelude::*;

// ─── Events (emitted via Anchor's `emit!` macro) ────────────────────

#[event]
pub struct VaultCreated {
    pub vault_id: u64,
    pub creator: Pubkey,
    pub vault_type: u8,
    pub condition_type: u8,
    pub unlock_value: u64,
    pub deadline: i64,
    pub recipient_hash: [u8; 32],
    pub name: [u8; 32],
    pub deposit_token: Pubkey,
    pub deposit_amount: u64,
    pub condition_token: Pubkey,
    pub privacy_flags: u8,
}

#[event]
pub struct VaultClaimed {
    pub vault_id: u64,
    pub recipient: Pubkey,
    pub amount: u64,
}

#[event]
pub struct VaultRefunded {
    pub vault_id: u64,
    pub refund_to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ActivityRecorded {
    pub user: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ExtraConditionAdded {
    pub vault_id: u64,
    pub index: u8,
    pub condition_type: u8,
    pub value: u64,
}

#[event]
pub struct SecretChunkAdded {
    pub vault_id: u64,
    pub index: u8,
}

#[event]
pub struct EncryptedFieldSet {
    pub vault_id: u64,
    pub field_type: u8,
}

#[event]
pub struct ObserverAdded {
    pub vault_id: u64,
    pub observer: Pubkey,
}

#[event]
pub struct ConditionDeposited {
    pub vault_id: u64,
    pub depositor: Pubkey,
    pub token: Pubkey,
    pub amount: u64,
}
