// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {euint256, ebool, e, inco} from "@inco/lightning/src/Lib.demonet.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import {VaultTypes} from "../types/VaultTypes.sol";

/// @title VaultStorage — Storage layout for PrivateVault
/// @notice Abstract contract that defines ALL storage variables.
///         Storage order must NEVER change (UUPS proxy compatibility).
///         New storage slots may only be appended at the end.
abstract contract VaultStorage is Initializable, UUPSUpgradeable, OwnableUpgradeable, ReentrancyGuardUpgradeable {
    using e for euint256;
    using e for ebool;
    using e for uint256;
    using e for bytes;

    // ─── Storage (order is immutable for proxy compat) ───────────────

    uint256 internal _vaultCounter;
    mapping(uint256 => VaultTypes.Vault) internal _vaults;
    mapping(address => uint256[]) internal _creatorVaults;
    mapping(bytes32 => uint256[]) internal _recipientHashVaults;
    mapping(address => uint256) public lastActivity;

    /// @dev Secret text stored as multiple 32-byte euint256 chunks (euint256 max = 256 bits = 32 bytes plaintext)
    mapping(uint256 => euint256[]) internal _secretChunks;

    /// @dev v3.7.0: Extra conditions beyond the first one (stored in Vault struct for backward compat)
    mapping(uint256 => VaultTypes.StoredCondition[]) internal _extraConditions;
    /// @dev v3.7.0: Encrypted condition value handles for each extra condition
    mapping(uint256 => euint256[]) internal _encryptedExtraConditionValues;

    /// @dev v3.8.0: Condition value commits for encrypted conditions (commit-reveal pattern)
    mapping(uint256 => bytes32[]) internal _conditionValueCommits;
    /// @dev v3.8.0: Encrypted condition salt (one per vault, for recipient to decrypt at claim time)
    mapping(uint256 => euint256) internal _encryptedConditionSalt;

    /// @dev v3.9.0: FHE recipient verification handles (vaultId → caller → ebool handle)
    mapping(uint256 => mapping(address => ebool)) internal _recipientVerifyHandle;

    /// @dev v3.11.0: Observers per vault (vaultId → observer addresses)
    mapping(uint256 => address[]) internal _observers;
    /// @dev v3.11.0: Reverse lookup — vaults where an address is an observer
    mapping(address => uint256[]) internal _observerVaults;
    /// @dev v3.12.0: Per-observer permission flags (vaultId → observer → uint8 bitmap)
    /// Bitmap mirrors privacyFlags: 0x01=recipient, 0x02=amount+token, 0x04=fallback, 0x08=name, 0x10=conditions, 0x20=secret
    mapping(uint256 => mapping(address => uint8)) internal _observerPermissions;

    // ─── v3.13.0: Multi-recipient support ────────────────────────────

    /// @dev All recipient hashes for multi-recipient vaults
    mapping(uint256 => bytes32[]) internal _vaultRecipientHashes;
    /// @dev Encrypted recipient handles for multi-recipient vaults
    mapping(uint256 => euint256[]) internal _vaultEncryptedRecipients;
    /// @dev Plaintext recipient addresses for multi-recipient vaults (when not encrypted)
    mapping(uint256 => address[]) internal _vaultRecipientsPlain;
    /// @dev Whether a specific recipient has already claimed (vaultId → address → bool)
    mapping(uint256 => mapping(address => bool)) internal _recipientClaimed;
    /// @dev How many recipients have claimed so far
    mapping(uint256 => uint8) internal _claimedCount;
    /// @dev Claim distribution mode: 0 = first-come (full amount), 1 = equal split
    mapping(uint256 => uint8) internal _claimDistribution;
    /// @dev Data access mode for secrets: 0 = all recipients get secrets, 1 = first claimer only
    mapping(uint256 => uint8) internal _dataAccessMode;
    /// @dev Whether the secret data has been claimed (first-only enforcement)
    mapping(uint256 => bool) internal _secretClaimed;

    // ─── v3.15.0: Condition deposits & EIP-712 activity signatures ───

    /// @dev Cumulative deposits per vault per token via depositForCondition (vaultId → token → amount)
    /// token = address(0) for native ETH
    mapping(uint256 => mapping(address => uint256)) public conditionDeposits;

    /// @dev EIP-712 nonces for recordActivityBySig replay protection (wallet → nonce)
    mapping(address => uint256) public activityNonces;

    // ─── Events ──────────────────────────────────────────────────────

    event VaultCreated(
        uint256 indexed vaultId,
        address indexed creator,
        uint8 vaultType,
        uint8 conditionType,
        uint256 unlockTime,
        uint256 deadline,
        bytes32 indexed recipientHash,
        string name,
        address depositToken,
        uint256 depositAmount,
        address conditionToken,
        uint8 privacyFlags
    );
    
    event VaultClaimed(uint256 indexed vaultId, address indexed recipient, uint256 amount);
    event VaultRefunded(uint256 indexed vaultId, address indexed refundTo, uint256 amount);
    event ActivityRecorded(address indexed user, uint256 timestamp);
    event ConditionDeposit(uint256 indexed vaultId, address indexed sender, address token, uint256 amount);
    event RecipientVerificationRequested(uint256 indexed vaultId, address indexed caller, ebool verifyHandle);
    event ObserverAdded(uint256 indexed vaultId, address indexed observer);

    // ─── Errors ──────────────────────────────────────────────────────

    error VaultNotFound();
    error VaultNotLocked();
    error ConditionsNotMet();
    error InvalidUnlockTime();
    error InvalidDeadline();
    error InvalidRecipient();
    error DeadlineNotReached();
    error ClaimPeriodExpired();
    error InsufficientDeposit();
    error InsufficientFee();
    error TransferFailed();
    error NotAuthorizedForRefund();
    error InvalidConditionReveal();
    error InvalidAttestation();
    error RecipientNotEncrypted();
    error VerificationNotRequested();
    error TooManyObservers();
    error InvalidObserverParams();
    error AlreadyClaimed();
    error TooManyRecipients();
    error SecretAlreadyClaimed();
    error InvalidSignature();
    error SignatureExpired();
    error InvalidNonce();
}
