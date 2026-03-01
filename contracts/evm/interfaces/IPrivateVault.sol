// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IPrivateVault {
    function lockAssets(address asset, uint256 amount) external;
    function releaseAssets(address recipient) external;
    function setConditions(bytes32 conditions) external;
    function getVaultDetails() external view returns (address, uint256, bytes32);
    function isLocked() external view returns (bool);
}