// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {euint256, ebool, inco} from "@inco/lightning/src/Lib.demonet.sol";
import {VaultTypes} from "../types/VaultTypes.sol";
import {VaultActions} from "./VaultActions.sol";

/// @title VaultViews — Read-only view functions
/// @notice All external view/pure functions for querying vault state.
abstract contract VaultViews is VaultActions {

    function getVaultInfo(uint256 vaultId) external view returns (
        uint256 id,
        address creator,
        uint8 vaultType,
        uint8 status,
        uint8 conditionType,
        string memory name,
        bytes32 recipientHash,
        uint256 unlockTime,
        uint256 deadline,
        uint256 createdAt,
        address depositToken,
        uint256 depositAmount
    ) {
        VaultTypes.Vault storage v = _vaults[vaultId];
        if (v.id == 0) revert VaultNotFound();
        bool maskAmount = (v.privacyFlags & 0x02) != 0;
        bool maskName = (v.privacyFlags & 0x08) != 0;
        bool maskConditions = (v.privacyFlags & 0x10) != 0;
        return (
            v.id, v.creator, v.vaultType, v.status, v.conditionType,
            maskName ? "" : v.name,
            v.recipientHash, maskConditions ? 0 : v.unlockTime, v.deadline, v.createdAt,
            maskAmount ? address(0) : v.depositToken,
            maskAmount ? 0 : v.depositAmount
        );
    }

    function getPrivacyInfo(uint256 vaultId) external view returns (
        uint8 privacyFlags,
        address recipientPlain,
        address fallbackPlain
    ) {
        VaultTypes.Vault storage v = _vaults[vaultId];
        if (v.id == 0) revert VaultNotFound();
        return (v.privacyFlags, v.recipientPlain, v.fallbackPlain);
    }

    /// @notice Get all FHE handles for a vault in one call
    function getHandles(uint256 vaultId) external view returns (
        euint256 recipient,
        euint256 amount,
        euint256 token,
        euint256 secret,
        euint256 fallback_,
        euint256 name_,
        euint256 conditionValue
    ) {
        VaultTypes.Vault storage v = _vaults[vaultId];
        if (v.id == 0) revert VaultNotFound();
        euint256 sec;
        if (euint256.unwrap(v.encryptedSecret) != 0) {
            sec = v.encryptedSecret;
        } else if (_secretChunks[vaultId].length > 0) {
            sec = _secretChunks[vaultId][0];
        }
        return (v.encryptedRecipient, v.encryptedAmount, v.encryptedToken, sec, v.encryptedFallback, v.encryptedName, v.encryptedConditionValue);
    }

    function getSecretChunk(uint256 vaultId, uint256 index) external view returns (euint256) {
        if (_vaults[vaultId].id == 0) revert VaultNotFound();
        if (_secretChunks[vaultId].length == 0 && index == 0) {
            return _vaults[vaultId].encryptedSecret;
        }
        return _secretChunks[vaultId][index];
    }

    function canClaim(uint256 vaultId) external view returns (bool) {
        VaultTypes.Vault storage v = _vaults[vaultId];
        if (v.id == 0) return false;
        if (v.status != uint8(VaultTypes.VaultStatus.Locked) &&
            v.status != uint8(VaultTypes.VaultStatus.PartialClaim)) return false;
        if (v.deadline > 0 && block.timestamp > v.deadline) return false;
        if ((v.privacyFlags & 0x10) != 0) return false;
        return _canClaim(v);
    }

    function getCreatorVaults(address creator) external view returns (uint256[] memory) {
        return _creatorVaults[creator];
    }

    function getVaultsByRecipientHash(bytes32 recipientHash) external view returns (uint256[] memory) {
        return _recipientHashVaults[recipientHash];
    }

    function getTotalVaults() external view returns (uint256) {
        return _vaultCounter;
    }

    function getRequiredFee() external view returns (uint256) {
        return inco.getFee();
    }

    function getConditionCount(uint256 vaultId) external view returns (uint256) {
        if (_vaults[vaultId].id == 0) revert VaultNotFound();
        return 1 + _extraConditions[vaultId].length;
    }

    function getExtraCondition(uint256 vaultId, uint256 index) external view returns (
        uint8 condType,
        uint256 value,
        address monitoringAddress,
        address tokenAddress,
        uint256 condParam
    ) {
        if (_vaults[vaultId].id == 0) revert VaultNotFound();
        bool maskConditions = (_vaults[vaultId].privacyFlags & 0x10) != 0;
        VaultTypes.StoredCondition storage c = _extraConditions[vaultId][index];
        return (
            c.conditionType,
            maskConditions ? 0 : c.value,
            c.monitoringAddress,
            maskConditions ? address(0) : c.tokenAddress,
            maskConditions ? 0 : c.conditionParam
        );
    }

    function getRecipientVerifyHandle(uint256 vaultId, address caller) external view returns (ebool) {
        return _recipientVerifyHandle[vaultId][caller];
    }

    function getObservers(uint256 vaultId) external view returns (address[] memory) {
        return _observers[vaultId];
    }

    function getObserverPermissions(uint256 vaultId, address observer) external view returns (uint8) {
        return _observerPermissions[vaultId][observer];
    }

    function getAccessConfig(uint256 vaultId) external view returns (
        uint8 claimDistribution,
        uint8 dataAccessMode,
        uint256 recipientCount,
        uint8 claimedCount,
        bool secretClaimed
    ) {
        if (_vaults[vaultId].id == 0) revert VaultNotFound();
        return (
            _claimDistribution[vaultId],
            _dataAccessMode[vaultId],
            _getRecipientCount(vaultId),
            _claimedCount[vaultId],
            _secretClaimed[vaultId]
        );
    }

    // v3.15.0: conditionDeposits(uint256,address) and activityNonces(address)
    // are auto-generated public getters from VaultStorage
}
