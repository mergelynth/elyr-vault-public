// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {euint256, ebool} from "@inco/lightning/src/Lib.demonet.sol";

/// @title VaultTypes — Shared types, structs, enums, events, and errors
/// @notice Used by all PrivateVault modules. No storage or logic here.
library VaultTypes {
    /// @notice Condition types for vault unlock
    enum ConditionType {
        ReleaseAtDate,      // 0: unlocks at specific timestamp
        Inactivity,         // 1: unlocks after creator inactivity period
        BalanceBelow,       // 2: unlocks when monitored address's token balance falls below threshold
        IncomingTransaction // 3: unlocks when monitored address receives tokens
    }

    /// @notice Vault status
    enum VaultStatus {
        Locked,       // 0: Active, waiting for conditions
        Claimed,      // 1: All recipients claimed (or single/first-come claimed)
        Refunded,     // 2: Creator/fallback refunded after deadline
        Cancelled,    // 3: Removed in v3.10.0 — kept for storage layout compatibility
        PartialClaim  // 4: v3.13.0 — some but not all recipients claimed (equal-split mode)
    }

    /// @notice Privacy flags for vault data (packed into uint8)
    /// @dev Bit 0: encryptRecipient (1 = encrypted)
    /// @dev Bit 1: encryptAmount (1 = encrypted) - for Asset vaults
    /// @dev Bit 2: encryptFallback (1 = fallback address encrypted)
    /// @dev Bit 3: encryptName (1 = vault name encrypted via FHE)
    /// @dev Bit 4: encryptConditions (1 = condition values encrypted via FHE)

    /// @notice Input struct for conditions (used in calldata)
    struct ConditionInput {
        uint8 conditionType;       // See ConditionType enum
        uint256 value;             // timestamp, duration(s), threshold(wei) — 0 when encryptConditions
        address monitoringAddress; // Address to monitor (address(0) → defaults to creator)
        address tokenAddress;      // Token for condition evaluation (address(0) → native ETH)
        bytes encValueBytes;       // Client-encrypted condition value (when encryptConditions=true)
        bytes32 valueCommit;       // keccak256(condIndex, value, conditionSalt) for commit-reveal
    }

    /// @notice Stored condition for multi-condition vaults (v3.7.0+)
    struct StoredCondition {
        uint8 conditionType;
        uint256 value;             // unlockTime / duration / threshold
        address monitoringAddress;
        address tokenAddress;
        uint256 conditionParam;    // snapshot for IncomingTransaction
    }

    struct Vault {
        uint256 id;
        address creator;
        uint8 vaultType;        // 0 = Asset, 1 = Secret
        uint8 status;           // See VaultStatus enum
        uint8 conditionType;    // See ConditionType enum
        uint8 privacyFlags;     // Bit flags for privacy options
        string name;
        bytes32 recipientHash;  // keccak256(recipient) for verification
        bytes32 fallbackHash;   // keccak256(fallback) for refund verification
        uint256 unlockTime;     // ReleaseAtDate: timestamp | Inactivity: duration(s) | BalanceBelow: threshold(wei)
        uint256 deadline;       // After this, refund is allowed (0 = no deadline)
        uint256 createdAt;
        
        // Asset vault: deposited tokens
        address depositToken;   // address(0) = ETH, otherwise ERC20
        uint256 depositAmount;  // Actual deposited amount (always known to contract)
        
        // Monitoring address for conditions (inactivity, balance, incoming tx)
        address monitoringAddress; // Address to monitor for condition fulfillment (defaults to creator)
        
        // Condition-specific parameter (e.g., balance snapshot for IncomingTransaction)
        uint256 conditionParam;
        
        // Token used in condition evaluation: address(0) = native ETH, otherwise ERC-20
        address conditionToken;
        
        // Encrypted data
        euint256 encryptedRecipient;
        euint256 encryptedAmount;   // Display amount (can differ from deposit for privacy)
        euint256 encryptedSecret;   // For Secret vaults
        euint256 encryptedFallback; // Encrypted fallback address
        euint256 encryptedName;     // Encrypted vault name (up to 32 UTF-8 bytes)
        
        // Plaintext data (when not encrypted)
        address recipientPlain;
        address fallbackPlain;
        
        // Whether depositToken is a cERC-20 (ConfidentialERC20) that requires FHE handle transfers
        bool isConfidentialToken;
        
        // FHE handle for cERC-20 deposit amount (used for claim/refund transfers)
        // For standard ERC-20 / ETH this is zero; depositAmount holds plaintext value instead.
        euint256 encryptedDeposit;
        
        // Encrypted condition value (unlockTime / threshold) — appended at end for upgrade safety
        euint256 encryptedConditionValue;
        
        // FHE-encrypted token address (when encryptAmount masks depositToken for ERC-20 vaults)
        // Enables authorized users (creator/recipient) to decrypt via Inco SDK.
        // The plaintext depositToken is kept for operational use (_transferTokenOut).
        euint256 encryptedToken;
    }
}
