// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/// @title ConditionsLib — Condition evaluation logic (external library)
/// @notice Deployed separately to reduce main contract bytecode.
///         Evaluates all 4 condition types: ReleaseAtDate, Inactivity, BalanceBelow, IncomingTransaction.
library ConditionsLib {
    /// @notice Evaluate a single condition
    /// @param condType 0=ReleaseAtDate, 1=Inactivity, 2=BalanceBelow, 3=IncomingTransaction
    /// @param value Condition value (timestamp, duration, threshold, or required deposit)
    /// @param monitored Pre-resolved monitoring address
    /// @param condToken Token address for condition (address(0) = ETH)
    /// @param condParam Snapshot value for legacy IncomingTransaction (pre-v3.15.0)
    /// @param lastActivityTs Last recorded activity timestamp for the monitored address
    /// @param depositAmt Cumulative deposits for this vault/token via depositForCondition
    function checkCondition(
        uint8 condType,
        uint256 value,
        address monitored,
        address condToken,
        uint256 condParam,
        uint256 lastActivityTs,
        uint256 depositAmt
    ) external view returns (bool) {
        // ReleaseAtDate
        if (condType == 0) return block.timestamp >= value;

        // Inactivity
        if (condType == 1) return block.timestamp >= lastActivityTs + value;

        // BalanceBelow
        if (condType == 2) {
            if (condToken != address(0)) {
                return IERC20(condToken).balanceOf(monitored) < value;
            }
            return monitored.balance < value;
        }

        // IncomingTransaction
        if (condType == 3) {
            // v3.15.0: Deposit-based approach
            if (value > 0) return depositAmt >= value;
            // Legacy fallback: snapshot-based
            if (condToken != address(0)) {
                return IERC20(condToken).balanceOf(monitored) > condParam;
            }
            return monitored.balance > condParam;
        }

        return false;
    }
}
