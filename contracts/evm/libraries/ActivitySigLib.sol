// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

/// @title ActivitySigLib — EIP-712 signature verification for recordActivityBySig
/// @notice Deployed as an external library to reduce main contract bytecode.
///         Functions use DELEGATECALL so address(this) and block.chainid
///         correctly resolve to the calling contract's context.
library ActivitySigLib {
    /// @dev EIP-712 domain typehash
    bytes32 internal constant DOMAIN_TYPEHASH = keccak256(
        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
    );
    /// @dev EIP-712 struct typehash for RecordActivity
    bytes32 internal constant RECORD_ACTIVITY_TYPEHASH = keccak256(
        "RecordActivity(address wallet,uint256 nonce,uint256 deadline)"
    );

    /// @notice Verify an EIP-712 RecordActivity signature
    /// @param wallet Expected signer address
    /// @param nonce Expected nonce value
    /// @param deadline Signature expiry timestamp
    /// @param contractAddr The calling contract address (for EIP-712 domain separator)
    /// @param v ECDSA recovery id
    /// @param r ECDSA r component
    /// @param s ECDSA s component
    /// @return signer The recovered signer address (address(0) on invalid sig)
    function verifyActivitySig(
        address wallet,
        uint256 nonce,
        uint256 deadline,
        address contractAddr,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external view returns (address signer) {
        bytes32 domainSeparator = keccak256(abi.encode(
            DOMAIN_TYPEHASH,
            keccak256("PrivateVault"),
            keccak256("1"),
            block.chainid,
            contractAddr
        ));

        bytes32 structHash = keccak256(abi.encode(
            RECORD_ACTIVITY_TYPEHASH,
            wallet,
            nonce,
            deadline
        ));

        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        signer = ecrecover(digest, v, r, s);
    }
}
