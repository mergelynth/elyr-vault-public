// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {VaultTypes} from "../types/VaultTypes.sol";
import {VaultStorage} from "./VaultStorage.sol";
import {ConditionsLib} from "../libraries/ConditionsLib.sol";

/// @title VaultConditions — Condition evaluation logic
/// @notice Handles checking whether vault conditions are met (date, inactivity, balance, incoming tx).
///         Core evaluation delegated to ConditionsLib (external library) to reduce bytecode.
abstract contract VaultConditions is VaultStorage {
    
    function _canClaim(VaultTypes.Vault storage v) internal view returns (bool) {
        uint256 vaultId = v.id;
        
        // Check first condition (stored in Vault struct — backward compat)
        if (!_checkCondition(vaultId, v.conditionType, v.unlockTime, v.monitoringAddress, v.creator, v.conditionToken, v.conditionParam)) {
            return false;
        }
        
        // Check extra conditions (v3.7.0+)
        VaultTypes.StoredCondition[] storage extras = _extraConditions[vaultId];
        for (uint256 i = 0; i < extras.length; i++) {
            if (!_checkCondition(vaultId, extras[i].conditionType, extras[i].value, extras[i].monitoringAddress, v.creator, extras[i].tokenAddress, extras[i].conditionParam)) {
                return false;
            }
        }
        
        return true; // ALL conditions met
    }
    
    /// @dev v3.8.0: Verify condition value commits and evaluate conditions using revealed values
    function _verifyAndCheckConditions(VaultTypes.Vault storage v, bytes32 conditionSalt, uint256[] calldata conditionValues) internal view {
        uint256 vaultId = v.id;
        VaultTypes.StoredCondition[] storage extras = _extraConditions[vaultId];
        uint256 totalConds = 1 + extras.length;
        
        if (conditionValues.length != totalConds) revert InvalidConditionReveal();
        
        bytes32[] storage commits = _conditionValueCommits[vaultId];
        if (commits.length != totalConds) revert InvalidConditionReveal();
        
        // Verify and evaluate first condition
        if (keccak256(abi.encodePacked(uint256(0), conditionValues[0], conditionSalt)) != commits[0]) {
            revert InvalidConditionReveal();
        }
        if (!_checkCondition(vaultId, v.conditionType, conditionValues[0], v.monitoringAddress, v.creator, v.conditionToken, v.conditionParam)) {
            revert ConditionsNotMet();
        }
        
        // Verify and evaluate extra conditions
        for (uint256 i = 0; i < extras.length; i++) {
            if (keccak256(abi.encodePacked(i + 1, conditionValues[i + 1], conditionSalt)) != commits[i + 1]) {
                revert InvalidConditionReveal();
            }
            if (!_checkCondition(vaultId, extras[i].conditionType, conditionValues[i + 1], extras[i].monitoringAddress, v.creator, extras[i].tokenAddress, extras[i].conditionParam)) {
                revert ConditionsNotMet();
            }
        }
    }
    
    /// @dev Check a single condition — resolves monitored address and delegates to ConditionsLib
    function _checkCondition(
        uint256 vaultId,
        uint8 condType,
        uint256 value,
        address monitoringAddr,
        address creator,
        address condToken,
        uint256 condParam
    ) internal view returns (bool) {
        address monitored = monitoringAddr != address(0) ? monitoringAddr : creator;
        return ConditionsLib.checkCondition(
            condType, value, monitored, condToken, condParam,
            lastActivity[monitored],
            conditionDeposits[vaultId][condToken]
        );
    }
}
