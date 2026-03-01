// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {euint256} from "@inco/lightning/src/Lib.demonet.sol";

/// @notice Interface for Inco ConfidentialERC20 tokens (cERC-20)
/// @dev These use euint256 (FHE handles) instead of plaintext uint256 for amounts.
///      euint256 is `type euint256 is bytes32` so ABI-encodes as bytes32.
interface IConfidentialERC20 {
    function transfer(address to, euint256 amount) external returns (bool);
    function transferFrom(address from, address to, euint256 amount) external returns (bool);
}
