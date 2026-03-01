// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {euint256, e} from "@inco/lightning/src/Lib.demonet.sol";
import {DecryptionAttestation} from "@inco/lightning/src/lightning-parts/DecryptionAttester.types.sol";
import {VaultTypes} from "../types/VaultTypes.sol";
import {VaultCreation} from "./VaultCreation.sol";
import {ActivitySigLib} from "../libraries/ActivitySigLib.sol";

/// @title VaultActions — Claim, refund, grantDecryptionRights, recordActivity, depositForCondition
abstract contract VaultActions is VaultCreation {

    function claim(
        uint256 vaultId,
        bytes32 claimSalt,
        bytes32 conditionSalt,
        uint256[] calldata conditionValues,
        DecryptionAttestation calldata attestation,
        bytes[] calldata attestationSignatures
    ) external nonReentrant {
        VaultTypes.Vault storage v = _vaults[vaultId];
        if (v.id == 0) revert VaultNotFound();
        if (v.status != uint8(VaultTypes.VaultStatus.Locked) &&
            v.status != uint8(VaultTypes.VaultStatus.PartialClaim)) revert VaultNotLocked();
        if (v.vaultType != 0 && v.vaultType != 2) revert VaultNotFound();
        
        _verifyRecipient(v, vaultId, claimSalt, attestation, attestationSignatures);
        
        bool encryptConditions = (v.privacyFlags & 0x10) != 0;
        if (encryptConditions) {
            _verifyAndCheckConditions(v, conditionSalt, conditionValues);
        } else {
            if (!_canClaim(v)) revert ConditionsNotMet();
        }
        
        if (v.deadline > 0 && block.timestamp > v.deadline) revert ClaimPeriodExpired();
        
        uint256 recipientCount = _getRecipientCount(vaultId);
        uint8 dist = _claimDistribution[vaultId];
        
        if (recipientCount <= 1 || dist == 0) {
            v.status = uint8(VaultTypes.VaultStatus.Claimed);
            uint256 amount = v.depositAmount;
            _transferTokenOut(v, msg.sender, amount);
            _grantAllDecryptionRights(v, vaultId, msg.sender);
            lastActivity[msg.sender] = block.timestamp;
            emit VaultClaimed(vaultId, msg.sender, amount);
        } else {
            if (_recipientClaimed[vaultId][msg.sender]) revert AlreadyClaimed();
            _recipientClaimed[vaultId][msg.sender] = true;
            _claimedCount[vaultId]++;
            
            uint256 share = v.depositAmount / recipientCount;
            if (_claimedCount[vaultId] == uint8(recipientCount)) {
                share = v.depositAmount - (share * (recipientCount - 1));
                v.status = uint8(VaultTypes.VaultStatus.Claimed);
            } else {
                v.status = uint8(VaultTypes.VaultStatus.PartialClaim);
            }
            
            _transferTokenOut(v, msg.sender, share);
            _grantNonSecretDecryptionRights(v, vaultId, msg.sender);
            
            uint8 dataAccess = _dataAccessMode[vaultId];
            if (dataAccess == 0) {
                _grantSecretDecryptionRights(v, vaultId, msg.sender);
            } else if (!_secretClaimed[vaultId]) {
                _secretClaimed[vaultId] = true;
                _grantSecretDecryptionRights(v, vaultId, msg.sender);
            }
            
            lastActivity[msg.sender] = block.timestamp;
            emit VaultClaimed(vaultId, msg.sender, share);
        }
    }

    function refund(uint256 vaultId, bytes32 refundSalt) external nonReentrant {
        VaultTypes.Vault storage v = _vaults[vaultId];
        if (v.id == 0) revert VaultNotFound();
        if (v.status != uint8(VaultTypes.VaultStatus.Locked)) revert VaultNotLocked();
        if (v.vaultType != 0 && v.vaultType != 2) revert VaultNotFound();
        
        if (v.deadline == 0) revert DeadlineNotReached();
        if (block.timestamp <= v.deadline) revert DeadlineNotReached();
        
        bool isCreator = msg.sender == v.creator;
        bool isFallback;
        if (refundSalt != bytes32(0)) {
            isFallback = v.fallbackHash != bytes32(0) &&
                         keccak256(abi.encodePacked(refundSalt, msg.sender)) == v.fallbackHash;
        } else {
            isFallback = v.fallbackHash != bytes32(0) && 
                         keccak256(abi.encodePacked(msg.sender)) == v.fallbackHash;
        }
        
        if (!isCreator && !isFallback) revert NotAuthorizedForRefund();
        
        v.status = uint8(VaultTypes.VaultStatus.Refunded);
        uint256 amount = v.depositAmount;
        _transferTokenOut(v, msg.sender, amount);
        lastActivity[msg.sender] = block.timestamp;
        emit VaultRefunded(vaultId, msg.sender, amount);
    }

    function grantDecryptionRights(
        uint256 vaultId,
        bytes32 claimSalt,
        bytes32,
        uint256[] calldata,
        DecryptionAttestation calldata attestation,
        bytes[] calldata attestationSignatures
    ) external {
        VaultTypes.Vault storage v = _vaults[vaultId];
        if (v.id == 0) revert VaultNotFound();
        _verifyRecipient(v, vaultId, claimSalt, attestation, attestationSignatures);
        _grantNonSecretDecryptionRights(v, vaultId, msg.sender);
    }

    function recordActivity() external {
        lastActivity[msg.sender] = block.timestamp;
        emit ActivityRecorded(msg.sender, block.timestamp);
    }

    // ─── v3.15.0: Deposit-based condition fulfillment ────────────────

    /// @notice Deposit native ETH toward an IncomingTransaction condition
    /// @param vaultId The vault whose IncomingTransaction condition to fulfill
    function depositForCondition(uint256 vaultId) external payable nonReentrant {
        VaultTypes.Vault storage v = _vaults[vaultId];
        if (v.id == 0) revert VaultNotFound();
        if (v.status != uint8(VaultTypes.VaultStatus.Locked) &&
            v.status != uint8(VaultTypes.VaultStatus.PartialClaim)) revert VaultNotLocked();
        if (msg.value == 0) revert InsufficientDeposit();

        conditionDeposits[vaultId][address(0)] += msg.value;
        lastActivity[msg.sender] = block.timestamp;
        emit ConditionDeposit(vaultId, msg.sender, address(0), msg.value);
    }

    // ─── v3.15.0: EIP-712 signed activity proof ─────────────────────

    /// @notice Record activity for a wallet using an EIP-712 signature
    /// @dev Signature verification delegated to ActivitySigLib (external library).
    ///      Protects against replay via per-wallet nonce and deadline.
    function recordActivityBySig(
        address wallet,
        uint256 nonce,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external {
        if (block.timestamp > deadline) revert SignatureExpired();
        if (nonce != activityNonces[wallet]) revert InvalidNonce();

        address signer = ActivitySigLib.verifyActivitySig(wallet, nonce, deadline, address(this), v, r, s);
        if (signer != wallet || signer == address(0)) revert InvalidSignature();

        activityNonces[wallet]++;
        lastActivity[wallet] = block.timestamp;
        emit ActivityRecorded(wallet, block.timestamp);
    }

    // ─── Internal ────────────────────────────────────────────────────

    function _grantNonSecretDecryptionRights(VaultTypes.Vault storage v, uint256 vaultId, address recipient) internal {
        if (euint256.unwrap(v.encryptedRecipient) != 0) e.allow(v.encryptedRecipient, recipient);
        if (euint256.unwrap(v.encryptedAmount) != 0) e.allow(v.encryptedAmount, recipient);
        if (euint256.unwrap(v.encryptedToken) != 0) e.allow(v.encryptedToken, recipient);
        if (euint256.unwrap(v.encryptedFallback) != 0) e.allow(v.encryptedFallback, recipient);
        if (euint256.unwrap(v.encryptedName) != 0) e.allow(v.encryptedName, recipient);
        if (euint256.unwrap(v.encryptedConditionValue) != 0) e.allow(v.encryptedConditionValue, recipient);
        euint256[] storage encExtras = _encryptedExtraConditionValues[vaultId];
        for (uint256 i = 0; i < encExtras.length; i++) {
            e.allow(encExtras[i], recipient);
        }
        if (euint256.unwrap(_encryptedConditionSalt[vaultId]) != 0) e.allow(_encryptedConditionSalt[vaultId], recipient);
    }

    function _grantSecretDecryptionRights(VaultTypes.Vault storage v, uint256 vaultId, address recipient) internal {
        euint256[] storage chunks = _secretChunks[vaultId];
        for (uint256 i = 0; i < chunks.length; i++) {
            e.allow(chunks[i], recipient);
        }
        if (euint256.unwrap(v.encryptedSecret) != 0) e.allow(v.encryptedSecret, recipient);
    }

    function _grantAllDecryptionRights(VaultTypes.Vault storage v, uint256 vaultId, address recipient) internal {
        _grantNonSecretDecryptionRights(v, vaultId, recipient);
        _grantSecretDecryptionRights(v, vaultId, recipient);
    }
}
