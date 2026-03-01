// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {euint256, ebool, e, inco} from "@inco/lightning/src/Lib.demonet.sol";
import {DecryptionAttestation} from "@inco/lightning/src/lightning-parts/DecryptionAttester.types.sol";
import {VaultTypes} from "../types/VaultTypes.sol";
import {VaultTransfers} from "./VaultTransfers.sol";

/// @title VaultRecipientVerify — Recipient verification logic
/// @notice Handles FHE attestation-based and legacy salt-based recipient verification.
///         v3.13.0: supports multi-recipient vaults.
abstract contract VaultRecipientVerify is VaultTransfers {
    
    /// @notice v3.9.0: Request FHE recipient verification (for encrypted-recipient vaults)
    /// @dev v3.14.0: checks against ALL encrypted recipients in multi-recipient vaults
    ///      using e.or() to combine FHE equality results.
    /// @param vaultId The vault to verify recipient for
    /// @return verifyHandle The ebool FHE handle — decrypt this via Inco SDK to get attestation
    function requestRecipientVerification(uint256 vaultId) external payable returns (ebool) {
        VaultTypes.Vault storage v = _vaults[vaultId];
        if (v.id == 0) revert VaultNotFound();
        if (euint256.unwrap(v.encryptedRecipient) == 0) revert RecipientNotEncrypted();
        
        euint256 callerEnc = e.asEuint256(uint256(uint160(msg.sender)));
        
        // v3.14.0: Check against all encrypted recipients using e.or()
        euint256[] storage allEnc = _vaultEncryptedRecipients[vaultId];
        ebool result;
        if (allEnc.length > 0) {
            result = e.eq(allEnc[0], callerEnc);
            for (uint256 i = 1; i < allEnc.length; i++) {
                result = e.or(result, e.eq(allEnc[i], callerEnc));
            }
        } else {
            // Backward compat: single encrypted recipient from vault struct
            result = e.eq(v.encryptedRecipient, callerEnc);
        }
        
        e.allow(result, msg.sender);
        e.allow(result, address(this));
        
        _recipientVerifyHandle[vaultId][msg.sender] = result;
        
        emit RecipientVerificationRequested(vaultId, msg.sender, result);
        return result;
    }

    /// @dev v3.9.0: Verify caller is the vault recipient — supports three paths:
    ///   Path A (attestation): FHE-verified via DecryptionAttestation (no URL secrets needed)
    ///   Path B (legacy salt): keccak256(claimSalt, msg.sender) matches a recipientHash
    ///   Path C (plaintext): keccak256(msg.sender) matches a recipientHash
    ///   v3.13.0: Paths B & C check against all recipient hashes in multi-recipient vaults
    function _verifyRecipient(
        VaultTypes.Vault storage v,
        uint256 vaultId,
        bytes32 claimSalt,
        DecryptionAttestation calldata attestation,
        bytes[] calldata attestationSignatures
    ) internal view {
        // Path A: FHE attestation (v3.9.0) — used for encrypted recipients without claimSalt
        if (attestation.handle != bytes32(0)) {
            // 1. Verify the handle matches what was stored by requestRecipientVerification
            if (ebool.unwrap(_recipientVerifyHandle[vaultId][msg.sender]) != attestation.handle) {
                revert VerificationNotRequested();
            }
            // 2. Verify the attestation value is true (recipient matched)
            if (attestation.value != bytes32(uint256(1))) revert InvalidRecipient();
            // 3. Verify TEE signatures via Inco Verifier
            if (!inco.incoVerifier().isValidDecryptionAttestation(attestation, attestationSignatures)) {
                revert InvalidAttestation();
            }
            return; // Verified via FHE attestation
        }
        
        // v3.13.0: Check multi-recipient hashes first
        bytes32[] storage hashes = _vaultRecipientHashes[vaultId];
        if (hashes.length > 0) {
            bool found = false;
            for (uint256 i = 0; i < hashes.length; i++) {
                if (claimSalt != bytes32(0)) {
                    if (keccak256(abi.encodePacked(claimSalt, msg.sender)) == hashes[i]) { found = true; break; }
                } else {
                    if (keccak256(abi.encodePacked(msg.sender)) == hashes[i]) { found = true; break; }
                }
            }
            if (!found) revert InvalidRecipient();
            return;
        }
        
        // Backward compat: single recipient hash from vault struct
        // Path B: Legacy salt-based verification
        if (claimSalt != bytes32(0)) {
            if (keccak256(abi.encodePacked(claimSalt, msg.sender)) != v.recipientHash) revert InvalidRecipient();
            return;
        }
        
        // Path C: Non-encrypted recipient (address in plaintext)
        if (keccak256(abi.encodePacked(msg.sender)) != v.recipientHash) revert InvalidRecipient();
    }
}
