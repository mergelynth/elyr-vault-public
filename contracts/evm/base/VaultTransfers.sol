// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {euint256, e} from "@inco/lightning/src/Lib.demonet.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IConfidentialERC20} from "../interfaces/IConfidentialERC20.sol";
import {VaultTypes} from "../types/VaultTypes.sol";
import {VaultConditions} from "./VaultConditions.sol";

/// @title VaultTransfers — Token transfer logic
/// @notice Handles transferring ETH, ERC-20, and cERC-20 tokens out of vaults.
abstract contract VaultTransfers is VaultConditions {
    
    /// @dev Transfer tokens out of the vault — handles both standard ERC-20 and cERC-20
    function _transferTokenOut(VaultTypes.Vault storage v, address to, uint256 amount) internal {
        if (v.depositToken == address(0)) {
            // Native ETH
            (bool success, ) = payable(to).call{value: amount}("");
            if (!success) revert TransferFailed();
        } else if (v.isConfidentialToken) {
            // cERC-20: prefer stored encrypted deposit handle (v3.5.0+)
            euint256 handle = v.encryptedDeposit;
            if (euint256.unwrap(handle) == 0) {
                // Backward compat: vaults created before v3.5.0 stored plaintext depositAmount
                handle = e.asEuint256(amount);
            }
            e.allow(handle, v.depositToken);
            e.allow(handle, to);
            bool success = IConfidentialERC20(v.depositToken).transfer(to, handle);
            if (!success) revert TransferFailed();
        } else {
            // Standard ERC-20
            bool success = IERC20(v.depositToken).transfer(to, amount);
            if (!success) revert TransferFailed();
        }
    }
}
