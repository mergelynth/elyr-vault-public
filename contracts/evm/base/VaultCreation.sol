// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {euint256, e, inco} from "@inco/lightning/src/Lib.demonet.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IConfidentialERC20} from "../interfaces/IConfidentialERC20.sol";
import {VaultTypes} from "../types/VaultTypes.sol";
import {VaultRecipientVerify} from "./VaultRecipientVerify.sol";

/// @title VaultCreation — Vault creation logic
/// @notice Handles createAssetVaultETH, createAssetVaultERC20, createSecretVault, and shared _createAssetVault.
abstract contract VaultCreation is VaultRecipientVerify {
    using e for euint256;
    using e for uint256;
    using e for bytes;

    /// @notice Create an Asset Vault with ETH deposit
    /// @dev v3.13.0: accepts multiple recipients + access config flags
    function createAssetVaultETH(
        address[] calldata recipients,
        address fallbackAddr,
        string calldata name,
        uint256 deadline,
        bytes[] calldata encRecipientBytesArray,
        bytes calldata encAmountBytes,
        bytes calldata encFallbackBytes,
        bytes calldata encNameBytes,
        bytes[] calldata encSecretChunks,
        VaultTypes.ConditionInput[] calldata conditions,
        uint8 privacyFlags,
        uint256 depositAmount,
        bytes32[] calldata recipientCommits,
        bytes32 fallbackCommit,
        bytes calldata encConditionSaltBytes,
        address[] calldata observers,
        uint8[] calldata observerPerms,
        uint8 accessFlags
    ) external payable nonReentrant returns (uint256) {
        if (depositAmount == 0) revert InsufficientDeposit();
        if (msg.value < depositAmount) revert InsufficientDeposit();
        
        return _createAssetVault(
            recipients, fallbackAddr, name, deadline,
            encRecipientBytesArray, encAmountBytes, encFallbackBytes, encNameBytes, encSecretChunks,
            conditions, privacyFlags, address(0), depositAmount, false,
            recipientCommits, fallbackCommit, encConditionSaltBytes,
            observers, observerPerms, accessFlags
        );
    }

    /// @notice Create an Asset Vault with ERC20 deposit
    /// @dev v3.13.0: accepts multiple recipients + access config flags
    function createAssetVaultERC20(
        address[] calldata recipients,
        address fallbackAddr,
        string calldata name,
        uint256 deadline,
        bytes[] calldata encRecipientBytesArray,
        bytes calldata encAmountBytes,
        bytes calldata encFallbackBytes,
        bytes calldata encNameBytes,
        bytes[] calldata encSecretChunks,
        VaultTypes.ConditionInput[] calldata conditions,
        uint8 privacyFlags,
        address token,
        uint256 amount,
        bool isConfidential,
        bytes calldata encDepositBytes,
        bytes32[] calldata recipientCommits,
        bytes32 fallbackCommit,
        bytes calldata encConditionSaltBytes,
        address[] calldata observers,
        uint8[] calldata observerPerms,
        uint8 accessFlags
    ) external payable nonReentrant returns (uint256) {
        euint256 depositHandle;
        
        if (isConfidential) {
            if (encDepositBytes.length == 0) revert InsufficientDeposit();
            depositHandle = encDepositBytes.newEuint256(msg.sender);
            e.allow(depositHandle, token);
            e.allow(depositHandle, address(this));
            bool success = IConfidentialERC20(token).transferFrom(msg.sender, address(this), depositHandle);
            if (!success) revert TransferFailed();
        } else {
            if (amount == 0) revert InsufficientDeposit();
            bool success = IERC20(token).transferFrom(msg.sender, address(this), amount);
            if (!success) revert TransferFailed();
        }
        
        uint256 vaultId = _createAssetVault(
            recipients, fallbackAddr, name, deadline,
            encRecipientBytesArray, encAmountBytes, encFallbackBytes, encNameBytes, encSecretChunks,
            conditions, privacyFlags, token, isConfidential ? 0 : amount, isConfidential,
            recipientCommits, fallbackCommit, encConditionSaltBytes,
            observers, observerPerms, accessFlags
        );
        
        if (isConfidential) {
            _vaults[vaultId].encryptedDeposit = depositHandle;
        }
        
        return vaultId;
    }

    /// @notice Create a Secret Vault (no token deposit)
    function createSecretVault(
        address[] calldata recipients,
        string calldata name,
        bytes[] calldata encRecipientBytesArray,
        bytes[] calldata encSecretChunks,
        bytes calldata encNameBytes,
        VaultTypes.ConditionInput[] calldata conditions,
        uint8 privacyFlags,
        bytes32[] calldata recipientCommits,
        bytes calldata encConditionSaltBytes,
        address[] calldata observers,
        uint8[] calldata observerPerms,
        uint8 accessFlags
    ) external payable nonReentrant returns (uint256) {
        if (conditions.length == 0) revert ConditionsNotMet();
        if (recipients.length == 0) revert InvalidRecipient();
        if (recipients.length > 5) revert TooManyRecipients();
        
        bool encryptConditions = (privacyFlags & 0x10) != 0;
        
        if (!encryptConditions) {
            for (uint256 i = 0; i < conditions.length; i++) {
                if (conditions[i].conditionType == uint8(VaultTypes.ConditionType.ReleaseAtDate)) {
                    if (conditions[i].value <= block.timestamp) revert InvalidUnlockTime();
                }
            }
        }
        
        uint256 fee = inco.getFee();
        uint256 requiredFee = 0;
        
        bool encryptRecipient = (privacyFlags & 0x01) != 0;
        bool encryptName = (privacyFlags & 0x08) != 0;
        if (encryptRecipient && encRecipientBytesArray.length > 0) requiredFee += fee * encRecipientBytesArray.length;
        if (encSecretChunks.length > 0) requiredFee += fee * encSecretChunks.length;
        if (encryptName && encNameBytes.length > 0) requiredFee += fee;
        if (encryptConditions) requiredFee += fee * (conditions.length + 1);
        if (requiredFee == 0) requiredFee = fee;
        if (msg.value < requiredFee) revert InsufficientFee();

        _vaultCounter++;
        uint256 vaultId = _vaultCounter;
        
        VaultTypes.Vault storage v = _vaults[vaultId];
        v.id = vaultId;
        v.creator = msg.sender;
        v.vaultType = 1;
        v.status = uint8(VaultTypes.VaultStatus.Locked);
        v.privacyFlags = privacyFlags;
        v.name = name;
        v.createdAt = block.timestamp;
        
        v.conditionType = conditions[0].conditionType;
        v.unlockTime = encryptConditions ? 0 : conditions[0].value;
        v.monitoringAddress = conditions[0].monitoringAddress != address(0) ? conditions[0].monitoringAddress : msg.sender;
        v.conditionToken = conditions[0].tokenAddress;
        
        _storeRecipients(vaultId, v, recipients, encRecipientBytesArray, recipientCommits, encryptRecipient, accessFlags);
        
        for (uint256 i = 0; i < encSecretChunks.length; i++) {
            euint256 chunk = encSecretChunks[i].newEuint256(msg.sender);
            _secretChunks[vaultId].push(chunk);
            e.allow(chunk, address(this));
            e.allow(chunk, msg.sender);
        }
        
        if (encryptName && encNameBytes.length > 0) {
            euint256 encName = encNameBytes.newEuint256(msg.sender);
            v.encryptedName = encName;
            e.allow(encName, address(this));
            e.allow(encName, msg.sender);
            _allowForRecipients(encName, recipients);
        }
        
        if (encryptConditions) {
            if (encConditionSaltBytes.length > 0) {
                euint256 encSalt = encConditionSaltBytes.newEuint256(msg.sender);
                _encryptedConditionSalt[vaultId] = encSalt;
                e.allow(encSalt, address(this));
                e.allow(encSalt, msg.sender);
                _allowForRecipients(encSalt, recipients);
            }
            
            euint256 encCondition = conditions[0].encValueBytes.newEuint256(msg.sender);
            v.encryptedConditionValue = encCondition;
            _conditionValueCommits[vaultId].push(conditions[0].valueCommit);
            e.allow(encCondition, address(this));
            e.allow(encCondition, msg.sender);
            _allowForRecipients(encCondition, recipients);
            for (uint256 i = 1; i < conditions.length; i++) {
                euint256 enc = conditions[i].encValueBytes.newEuint256(msg.sender);
                _encryptedExtraConditionValues[vaultId].push(enc);
                _conditionValueCommits[vaultId].push(conditions[i].valueCommit);
                e.allow(enc, address(this));
                e.allow(enc, msg.sender);
                _allowForRecipients(enc, recipients);
            }
        }
        
        if (conditions[0].conditionType == uint8(VaultTypes.ConditionType.IncomingTransaction)) {
            address monitored = v.monitoringAddress;
            if (conditions[0].tokenAddress != address(0)) {
                v.conditionParam = IERC20(conditions[0].tokenAddress).balanceOf(monitored);
            } else {
                v.conditionParam = monitored.balance;
            }
        }
        
        if (conditions[0].conditionType == uint8(VaultTypes.ConditionType.Inactivity)) {
            address monitored = v.monitoringAddress;
            if (lastActivity[monitored] == 0) {
                lastActivity[monitored] = block.timestamp;
            }
        }
        
        _storeExtraConditions(vaultId, conditions, encryptConditions);
        
        _creatorVaults[msg.sender].push(vaultId);
        lastActivity[msg.sender] = block.timestamp;
        
        bool maskName = (privacyFlags & 0x08) != 0;
        bool maskConditions = (privacyFlags & 0x10) != 0;
        emit VaultCreated(vaultId, msg.sender, 1, conditions[0].conditionType,
            maskConditions ? 0 : conditions[0].value, 0, v.recipientHash,
            maskName ? "" : name, address(0), 0,
            maskConditions ? address(0) : conditions[0].tokenAddress, privacyFlags);

        _storeObservers(vaultId, observers, observerPerms);

        return vaultId;
    }

    // ─── Internal ────────────────────────────────────────────────────

    function _createAssetVault(
        address[] calldata recipients,
        address fallbackAddr,
        string calldata name,
        uint256 deadline,
        bytes[] calldata encRecipientBytesArray,
        bytes calldata encAmountBytes,
        bytes calldata encFallbackBytes,
        bytes calldata encNameBytes,
        bytes[] calldata encSecretChunks,
        VaultTypes.ConditionInput[] calldata conditions,
        uint8 privacyFlags,
        address depositToken,
        uint256 depositAmount,
        bool isConfidential,
        bytes32[] calldata recipientCommits,
        bytes32 fallbackCommit,
        bytes calldata encConditionSaltBytes,
        address[] calldata observers,
        uint8[] calldata observerPerms,
        uint8 accessFlags
    ) internal returns (uint256) {
        if (conditions.length == 0) revert ConditionsNotMet();
        if (recipients.length == 0) revert InvalidRecipient();
        if (recipients.length > 5) revert TooManyRecipients();
        
        bool encryptConditions = (privacyFlags & 0x10) != 0;
        
        if (!encryptConditions) {
            for (uint256 i = 0; i < conditions.length; i++) {
                if (conditions[i].conditionType == uint8(VaultTypes.ConditionType.ReleaseAtDate)) {
                    if (conditions[i].value <= block.timestamp) revert InvalidUnlockTime();
                }
            }
            if (deadline > 0 && deadline <= conditions[0].value) revert InvalidDeadline();
        }
        
        // Calculate Inco fee
        uint256 fee = inco.getFee();
        uint256 requiredFee = 0;
        
        bool encryptRecipient = (privacyFlags & 0x01) != 0;
        bool encryptAmount = (privacyFlags & 0x02) != 0;
        bool encryptFallback = (privacyFlags & 0x04) != 0;
        bool encryptName = (privacyFlags & 0x08) != 0;
        
        // v3.13.0: fee per encrypted recipient
        if (encryptRecipient && encRecipientBytesArray.length > 0) requiredFee += fee * encRecipientBytesArray.length;
        if (encryptAmount && encAmountBytes.length > 0) requiredFee += fee;
        if (encryptAmount && depositToken != address(0)) requiredFee += fee; // encrypted token address
        if (encryptFallback && encFallbackBytes.length > 0) requiredFee += fee;
        if (encryptName && encNameBytes.length > 0) requiredFee += fee;
        if (encryptConditions) requiredFee += fee * (conditions.length + 1);
        if (encSecretChunks.length > 0) requiredFee += fee * encSecretChunks.length;
        if (isConfidential) requiredFee += fee;
        if (requiredFee == 0) requiredFee = fee;
        
        uint256 feePayment = depositToken == address(0) ? msg.value - depositAmount : msg.value;
        if (feePayment < requiredFee) revert InsufficientFee();

        _vaultCounter++;
        uint256 vaultId = _vaultCounter;
        
        VaultTypes.Vault storage v = _vaults[vaultId];
        v.id = vaultId;
        v.creator = msg.sender;
        v.vaultType = encSecretChunks.length > 0 ? 2 : 0;
        v.status = uint8(VaultTypes.VaultStatus.Locked);
        v.privacyFlags = privacyFlags;
        v.name = name;
        v.fallbackHash = fallbackAddr != address(0) ? keccak256(abi.encodePacked(fallbackAddr)) : bytes32(0);
        v.deadline = deadline;
        v.createdAt = block.timestamp;
        v.depositToken = depositToken;
        v.depositAmount = depositAmount;
        v.isConfidentialToken = isConfidential;
        
        v.conditionType = conditions[0].conditionType;
        v.unlockTime = encryptConditions ? 0 : conditions[0].value;
        v.monitoringAddress = conditions[0].monitoringAddress != address(0) ? conditions[0].monitoringAddress : msg.sender;
        v.conditionToken = conditions[0].tokenAddress;
        
        if (encryptFallback && fallbackCommit != bytes32(0)) {
            v.fallbackHash = fallbackCommit;
        }
        
        // v3.13.0: Store recipients (multi-recipient aware)
        _storeRecipients(vaultId, v, recipients, encRecipientBytesArray, recipientCommits, encryptRecipient, accessFlags);
        
        // Handle encrypted amount
        if (encryptAmount && encAmountBytes.length > 0) {
            euint256 encAmount = encAmountBytes.newEuint256(msg.sender);
            v.encryptedAmount = encAmount;
            e.allow(encAmount, address(this));
            e.allow(encAmount, msg.sender);
        }
        
        // Handle encrypted token address (ERC-20 only)
        if (encryptAmount && depositToken != address(0)) {
            euint256 encToken = e.asEuint256(uint256(uint160(depositToken)));
            v.encryptedToken = encToken;
            e.allow(encToken, address(this));
            e.allow(encToken, msg.sender);
            _allowForRecipients(encToken, recipients);
        }
        
        // Handle encrypted fallback
        if (encryptFallback && encFallbackBytes.length > 0) {
            euint256 encFallback = encFallbackBytes.newEuint256(msg.sender);
            v.encryptedFallback = encFallback;
            e.allow(encFallback, address(this));
            e.allow(encFallback, msg.sender);
        } else {
            v.fallbackPlain = fallbackAddr;
        }
        
        // Handle encrypted name — grant to all non-encrypted recipients
        if (encryptName && encNameBytes.length > 0) {
            euint256 encName = encNameBytes.newEuint256(msg.sender);
            v.encryptedName = encName;
            e.allow(encName, address(this));
            e.allow(encName, msg.sender);
            _allowForRecipients(encName, recipients);
        }
        
        // Handle encrypted condition values
        if (encryptConditions) {
            if (encConditionSaltBytes.length > 0) {
                euint256 encSalt = encConditionSaltBytes.newEuint256(msg.sender);
                _encryptedConditionSalt[vaultId] = encSalt;
                e.allow(encSalt, address(this));
                e.allow(encSalt, msg.sender);
                _allowForRecipients(encSalt, recipients);
            }
            
            euint256 encCondition = conditions[0].encValueBytes.newEuint256(msg.sender);
            v.encryptedConditionValue = encCondition;
            _conditionValueCommits[vaultId].push(conditions[0].valueCommit);
            e.allow(encCondition, address(this));
            e.allow(encCondition, msg.sender);
            _allowForRecipients(encCondition, recipients);
            for (uint256 i = 1; i < conditions.length; i++) {
                euint256 enc = conditions[i].encValueBytes.newEuint256(msg.sender);
                _encryptedExtraConditionValues[vaultId].push(enc);
                _conditionValueCommits[vaultId].push(conditions[i].valueCommit);
                e.allow(enc, address(this));
                e.allow(enc, msg.sender);
                _allowForRecipients(enc, recipients);
            }
        }
        
        // Store encrypted secret chunks (hybrid vaults)
        for (uint256 i = 0; i < encSecretChunks.length; i++) {
            euint256 chunk = encSecretChunks[i].newEuint256(msg.sender);
            _secretChunks[vaultId].push(chunk);
            e.allow(chunk, address(this));
            e.allow(chunk, msg.sender);
        }
        
        // For IncomingTransaction condition[0], capture snapshot
        if (conditions[0].conditionType == uint8(VaultTypes.ConditionType.IncomingTransaction)) {
            address monitored = v.monitoringAddress;
            if (conditions[0].tokenAddress != address(0)) {
                v.conditionParam = IERC20(conditions[0].tokenAddress).balanceOf(monitored);
            } else {
                v.conditionParam = monitored.balance;
            }
        }
        
        // v3.10.0: Initialize lastActivity for Inactivity conditions
        if (conditions[0].conditionType == uint8(VaultTypes.ConditionType.Inactivity)) {
            address monitored = v.monitoringAddress;
            if (lastActivity[monitored] == 0) {
                lastActivity[monitored] = block.timestamp;
            }
        }
        
        // Store extra conditions (index 1+)
        _storeExtraConditions(vaultId, conditions, encryptConditions);
        
        _creatorVaults[msg.sender].push(vaultId);
        lastActivity[msg.sender] = block.timestamp;
        
        bool maskAmount = (privacyFlags & 0x02) != 0;
        bool maskName = (privacyFlags & 0x08) != 0;
        bool maskConditions = (privacyFlags & 0x10) != 0;
        emit VaultCreated(
            vaultId, msg.sender, v.vaultType, conditions[0].conditionType,
            maskConditions ? 0 : conditions[0].value, deadline, v.recipientHash,
            maskName ? "" : name,
            maskAmount ? address(0) : depositToken,
            maskAmount ? 0 : depositAmount,
            maskConditions ? address(0) : conditions[0].tokenAddress,
            privacyFlags
        );

        // v3.11.0: Store observers + v3.12.0: grant FHE rights per observer
        _storeObservers(vaultId, observers, observerPerms);

        return vaultId;
    }

    /// @dev Store extra conditions (index 1+) with IncomingTransaction snapshots and Inactivity init
    function _storeExtraConditions(
        uint256 vaultId,
        VaultTypes.ConditionInput[] calldata conditions,
        bool encryptConditions
    ) internal {
        for (uint256 i = 1; i < conditions.length; i++) {
            address mon = conditions[i].monitoringAddress != address(0) ? conditions[i].monitoringAddress : msg.sender;
            uint256 param = 0;
            if (conditions[i].conditionType == uint8(VaultTypes.ConditionType.IncomingTransaction)) {
                if (conditions[i].tokenAddress != address(0)) {
                    param = IERC20(conditions[i].tokenAddress).balanceOf(mon);
                } else {
                    param = mon.balance;
                }
            }
            // v3.10.0: Initialize lastActivity for Inactivity extra conditions
            if (conditions[i].conditionType == uint8(VaultTypes.ConditionType.Inactivity)) {
                if (lastActivity[mon] == 0) {
                    lastActivity[mon] = block.timestamp;
                }
            }
            _extraConditions[vaultId].push(VaultTypes.StoredCondition({
                conditionType: conditions[i].conditionType,
                value: encryptConditions ? 0 : conditions[i].value,
                monitoringAddress: mon,
                tokenAddress: conditions[i].tokenAddress,
                conditionParam: param
            }));
        }
    }

    /// @dev v3.13.0: Store recipients, hashes, encryption handles, and access config
    function _storeRecipients(
        uint256 vaultId,
        VaultTypes.Vault storage v,
        address[] calldata recipients,
        bytes[] calldata encRecipientBytesArray,
        bytes32[] calldata recipientCommits,
        bool encryptRecipient,
        uint8 accessFlags
    ) internal {
        uint256 recipientCount = recipients.length;
        
        if (recipientCount > 1) {
            _claimDistribution[vaultId] = (accessFlags & 0x01) != 0 ? 1 : 0;
            _dataAccessMode[vaultId] = (accessFlags & 0x02) != 0 ? 1 : 0;
        }
        
        for (uint256 i = 0; i < recipientCount; i++) {
            bytes32 hash;
            if (encryptRecipient && i < recipientCommits.length && recipientCommits[i] != bytes32(0)) {
                hash = recipientCommits[i];
            } else {
                hash = keccak256(abi.encodePacked(recipients[i]));
            }
            _vaultRecipientHashes[vaultId].push(hash);
            _recipientHashVaults[hash].push(vaultId);
        }
        v.recipientHash = _vaultRecipientHashes[vaultId][0];
        
        // v3.14.0: Store ALL encrypted recipients (not just first) for FHE verification
        if (encryptRecipient) {
            for (uint256 i = 0; i < recipientCount; i++) {
                if (i < encRecipientBytesArray.length && encRecipientBytesArray[i].length > 0) {
                    euint256 enc = encRecipientBytesArray[i].newEuint256(msg.sender);
                    _vaultEncryptedRecipients[vaultId].push(enc);
                    e.allow(enc, address(this));
                    e.allow(enc, msg.sender);
                    if (i == 0) {
                        v.encryptedRecipient = enc; // backward compat
                    }
                }
            }
        } else {
            v.recipientPlain = recipients[0];
        }
    }

    /// @dev v3.13.0: Grant FHE allow() to all non-zero (non-encrypted) recipients
    function _allowForRecipients(euint256 handle, address[] calldata recipients) internal {
        for (uint256 i = 0; i < recipients.length; i++) {
            if (recipients[i] != address(0)) {
                e.allow(handle, recipients[i]);
            }
        }
    }

    /// @dev v3.13.0: Get total recipient count for a vault (backward compat aware)
    function _getRecipientCount(uint256 vaultId) internal view returns (uint256) {
        uint256 count = _vaultRecipientHashes[vaultId].length;
        return count > 0 ? count : 1; // Fall back to single recipient for old vaults
    }

    /// @dev v3.12.0: Store observer addresses + permission flags, and grant scoped FHE rights
    function _storeObservers(uint256 vaultId, address[] calldata observers, uint8[] calldata observerPerms) internal {
        if (observers.length > 10) revert TooManyObservers();
        if (observerPerms.length > 0 && observerPerms.length != observers.length) revert InvalidObserverParams();
        
        VaultTypes.Vault storage v = _vaults[vaultId];
        for (uint256 i = 0; i < observers.length; i++) {
            if (observers[i] == address(0)) continue;
            _observers[vaultId].push(observers[i]);
            _observerVaults[observers[i]].push(vaultId);
            
            uint8 perms = i < observerPerms.length ? observerPerms[i] : 0;
            _observerPermissions[vaultId][observers[i]] = perms;
            
            // Grant FHE decrypt rights based on permission bitmap
            _grantScopedRights(v, vaultId, observers[i], perms);
            
            emit ObserverAdded(vaultId, observers[i]);
        }
    }

    /// @dev v3.12.0: Grant scoped FHE decryption rights to an observer
    /// Bitmap: 0x01=recipient, 0x02=amount+token, 0x04=fallback, 0x08=name, 0x10=conditions, 0x20=secret
    function _grantScopedRights(
        VaultTypes.Vault storage v,
        uint256 vaultId,
        address observer,
        uint8 perms
    ) internal {
        if (perms == 0) return;
        
        if ((perms & 0x01) != 0 && euint256.unwrap(v.encryptedRecipient) != 0) {
            e.allow(v.encryptedRecipient, observer);
        }
        if ((perms & 0x02) != 0) {
            if (euint256.unwrap(v.encryptedAmount) != 0) e.allow(v.encryptedAmount, observer);
            if (euint256.unwrap(v.encryptedToken) != 0) e.allow(v.encryptedToken, observer);
        }
        if ((perms & 0x04) != 0 && euint256.unwrap(v.encryptedFallback) != 0) {
            e.allow(v.encryptedFallback, observer);
        }
        if ((perms & 0x08) != 0 && euint256.unwrap(v.encryptedName) != 0) {
            e.allow(v.encryptedName, observer);
        }
        if ((perms & 0x10) != 0) {
            if (euint256.unwrap(v.encryptedConditionValue) != 0) e.allow(v.encryptedConditionValue, observer);
            euint256[] storage encExtras = _encryptedExtraConditionValues[vaultId];
            for (uint256 j = 0; j < encExtras.length; j++) e.allow(encExtras[j], observer);
            if (euint256.unwrap(_encryptedConditionSalt[vaultId]) != 0) e.allow(_encryptedConditionSalt[vaultId], observer);
        }
        if ((perms & 0x20) != 0) {
            euint256[] storage chunks = _secretChunks[vaultId];
            for (uint256 j = 0; j < chunks.length; j++) e.allow(chunks[j], observer);
            if (euint256.unwrap(v.encryptedSecret) != 0) e.allow(v.encryptedSecret, observer);
        }
    }
}
