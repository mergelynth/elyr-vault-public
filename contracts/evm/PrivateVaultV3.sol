// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {VaultViews} from "./base/VaultViews.sol";

/// @title PrivateVaultV3 - Main entry point (UUPS upgradeable)
/// @notice Inherits the full module chain:
///   VaultStorage -> VaultConditions -> VaultTransfers -> VaultRecipientVerify
///   -> VaultCreation -> VaultActions -> VaultViews -> PrivateVaultV3
///
/// All storage, events, errors, and logic are defined in the base modules.
/// This contract only provides initialization, upgrade authorization, and versioning.
///
/// @dev UUPS proxy pattern - storage layout is defined in VaultStorage and must
///      never be reordered. New storage may only be appended at the end.
contract PrivateVaultV3 is VaultViews {

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract (replaces constructor for proxy pattern)
    /// @param owner_ The address that will own the contract and can authorize upgrades
    function initialize(address owner_) external initializer {
        __Ownable_init(owner_);
        __ReentrancyGuard_init();
        __UUPSUpgradeable_init();
    }

    /// @notice Authorizes an upgrade to a new implementation
    /// @dev Only the owner can upgrade the contract
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    /// @notice Returns the current implementation version
    function version() external pure returns (string memory) {
        return "3.15.0";
    }

    receive() external payable {}
}
