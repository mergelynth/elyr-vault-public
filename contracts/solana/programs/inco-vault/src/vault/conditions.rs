use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak;

use super::state::{Vault, ExtraCondition};
use super::errors::VaultError;
use super::types::ConditionType;

// ─── Single Condition Check ──────────────────────────────────────────
/// Evaluates a single condition (mirrors EVM ConditionsLib.checkCondition)
///
/// * `condition_type` — ConditionType discriminant
/// * `value` — threshold / timestamp / duration
/// * `clock` — Solana Clock sysvar
/// * `last_activity_ts` — last activity timestamp for Inactivity (0 if N/A)
/// * `deposit_or_balance` — for IncomingTransaction: cumulative deposit amount;
///                          for BalanceBelow: current SOL/SPL balance of monitored address
/// * `_condition_param` — legacy snapshot (unused in v3.15 deposit-based model)
pub fn check_single_condition(
    condition_type: u8,
    value: u64,
    clock: &Clock,
    last_activity_ts: i64,
    deposit_or_balance: u64,
    _condition_param: u64,
) -> bool {
    match condition_type {
        // ReleaseAtDate: unlocks when current time >= unlock timestamp
        t if t == ConditionType::ReleaseAtDate as u8 => {
            clock.unix_timestamp >= value as i64
        }
        // Inactivity: unlocks when current time >= lastActivity + duration
        t if t == ConditionType::Inactivity as u8 => {
            clock.unix_timestamp >= last_activity_ts.saturating_add(value as i64)
        }
        // BalanceBelow: unlocks when monitored balance < threshold
        t if t == ConditionType::BalanceBelow as u8 => {
            deposit_or_balance < value
        }
        // IncomingTransaction (v3.15 deposit-based): unlocks when deposits >= threshold
        // Mirrors EVM: if (value > 0) return depositAmt >= value;
        t if t == ConditionType::IncomingTransaction as u8 => {
            if value > 0 {
                deposit_or_balance >= value
            } else {
                deposit_or_balance > 0
            }
        }
        _ => false,
    }
}

// ─── Check All Conditions ────────────────────────────────────────────
/// Checks the primary condition (inline in Vault) + all extra conditions.
///
/// For BalanceBelow / IncomingTransaction, the caller must provide the
/// current monitoring balance via `monitoring_balances` slice (index 0 =
/// primary condition, index 1+ = extra conditions).
///
/// For Inactivity, pass the activity tracker timestamps in
/// `activity_timestamps` (same indexing).
pub fn can_claim(
    vault: &Vault,
    extra_conditions: &[ExtraCondition],
    clock: &Clock,
    activity_timestamps: &[i64],
    monitoring_balances: &[u64],
) -> bool {
    let primary_activity = activity_timestamps.first().copied().unwrap_or(0);
    let primary_balance = monitoring_balances.first().copied().unwrap_or(0);

    // Check primary condition
    if !check_single_condition(
        vault.condition_type,
        vault.unlock_value,
        clock,
        primary_activity,
        primary_balance,
        vault.condition_param,
    ) {
        return false;
    }

    // Check extra conditions
    for (i, cond) in extra_conditions.iter().enumerate() {
        let activity = activity_timestamps.get(i + 1).copied().unwrap_or(0);
        let balance = monitoring_balances.get(i + 1).copied().unwrap_or(0);

        if !check_single_condition(
            cond.condition_type,
            cond.value,
            clock,
            activity,
            balance,
            cond.condition_param,
        ) {
            return false;
        }
    }

    true // ALL conditions met
}

// ─── Verify Condition Commits (commit-reveal for encrypted conditions) ─
/// Mirrors EVM _verifyAndCheckConditions.
/// Verifies keccak256(index, value, salt) == commit for each condition,
/// then evaluates the revealed values.
pub fn verify_and_check_conditions(
    vault: &Vault,
    extra_conditions: &[ExtraCondition],
    condition_salt: &[u8; 32],
    condition_values: &[u64],
    clock: &Clock,
    activity_timestamps: &[i64],
    monitoring_balances: &[u64],
) -> Result<()> {
    let total = 1 + extra_conditions.len();

    if condition_values.len() != total {
        return Err(VaultError::ConditionCountMismatch.into());
    }
    if vault.condition_commits_count as usize != total {
        return Err(VaultError::ConditionCountMismatch.into());
    }

    // Verify and evaluate primary condition
    let commit_0 = compute_condition_commit(0, condition_values[0], condition_salt);
    if commit_0 != vault.condition_value_commits[0] {
        return Err(VaultError::InvalidConditionReveal.into());
    }

    let primary_activity = activity_timestamps.first().copied().unwrap_or(0);
    let primary_balance = monitoring_balances.first().copied().unwrap_or(0);

    if !check_single_condition(
        vault.condition_type,
        condition_values[0],
        clock,
        primary_activity,
        primary_balance,
        vault.condition_param,
    ) {
        return Err(VaultError::ConditionsNotMet.into());
    }

    // Verify and evaluate extra conditions
    for (i, cond) in extra_conditions.iter().enumerate() {
        let idx = i + 1;
        let commit = compute_condition_commit(idx as u64, condition_values[idx], condition_salt);
        if commit != vault.condition_value_commits[idx] {
            return Err(VaultError::InvalidConditionReveal.into());
        }

        let activity = activity_timestamps.get(idx).copied().unwrap_or(0);
        let balance = monitoring_balances.get(idx).copied().unwrap_or(0);

        if !check_single_condition(
            cond.condition_type,
            condition_values[idx],
            clock,
            activity,
            balance,
            cond.condition_param,
        ) {
            return Err(VaultError::ConditionsNotMet.into());
        }
    }

    Ok(())
}

// ─── Helpers ─────────────────────────────────────────────────────────

/// Compute condition commit: keccak256(index || value || salt)
/// Matches EVM: keccak256(abi.encodePacked(uint256(i), conditionValues[i], conditionSalt))
pub fn compute_condition_commit(index: u64, value: u64, salt: &[u8; 32]) -> [u8; 32] {
    let hash = keccak::hashv(&[
        &index.to_le_bytes(),
        &value.to_le_bytes(),
        salt,
    ]);
    hash.to_bytes()
}
