
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import "./Algorithm.sol";
import "./P256Precompile.sol";

/// @title P256SHA256Algorithm
/// @notice DNSSEC Algorithm 13 (ECDSAP256SHA256) implementation using EIP-7951 precompile
/// @dev Replaces the Solidity-based EllipticCurve implementation with native P-256 verification
contract P256SHA256Algorithm is Algorithm {
    /// @dev Verifies a DNSSEC signature.
    /// @param key The DNSKEY RDATA (68 bytes: 4-byte header + 64-byte public key).
    /// @param data The signed data to verify.
    /// @param signature The signature to verify (64 bytes: r + s).
    /// @return True iff the signature is valid.
    function verify(
        bytes calldata key,
        bytes calldata data,
        bytes calldata signature
    ) external view override returns (bool) {
        require(signature.length == 64, "Invalid p256 signature length");
        require(key.length == 68, "Invalid p256 key length");

        // Extract signature components (r, s) and public key (qx, qy)
        // Key format: 4-byte DNSKEY header (flags, protocol, algorithm) + 64-byte public key
        bytes32 r;
        bytes32 s;
        bytes32 qx;
        bytes32 qy;

        assembly {
            // signature.offset points to start of signature in calldata
            r := calldataload(signature.offset)
            s := calldataload(add(signature.offset, 32))
            // key.offset + 4 skips the DNSKEY header
            qx := calldataload(add(key.offset, 4))
            qy := calldataload(add(key.offset, 36))
        }

        return P256Precompile.verify(sha256(data), r, s, qx, qy);
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
pragma solidity ^0.8.4;

library P256Precompile {
    /// @dev Verifies a P-256 ECDSA signature using EIP-7951 precompile.
    /// @param messageHash The SHA-256 hash of the message being verified.
    /// @param r The r component of the signature (32 bytes).
    /// @param s The s component of the signature (32 bytes).
    /// @param qx The x-coordinate of the public key (32 bytes).
    /// @param qy The y-coordinate of the public key (32 bytes).
    /// @return success True if the signature is valid, false otherwise.
    function verify(
        bytes32 messageHash,
        bytes32 r,
        bytes32 s,
        bytes32 qx,
        bytes32 qy
    ) internal view returns (bool success) {
        // EIP-7951 precompile input: hash(32) + r(32) + s(32) + x(32) + y(32) = 160 bytes
        bytes memory input = abi.encodePacked(messageHash, r, s, qx, qy);

        bytes memory output = new bytes(32);

        assembly {
            success := staticcall(
                gas(),
                0x100, // EIP-7951 P-256 precompile address
                add(input, 32),
                mload(input), // 160 bytes
                add(output, 32),
                32 // Output is 32 bytes
            )
        }

        // Precompile returns 32 bytes: 0x00...01 for valid, 0x00...00 for invalid
        if (success) {
            success = (output[31] == bytes1(0x01));
        }
    }
}


pragma solidity ^0.8.4;

/// @dev An interface for contracts implementing a DNSSEC (signing) algorithm.
interface Algorithm {
    /// @dev Verifies a signature.
    /// @param key The public key to verify with.
    /// @param data The signed data to verify.
    /// @param signature The signature to verify.
    /// @return True iff the signature is valid.
    function verify(
        bytes calldata key,
        bytes calldata data,
        bytes calldata signature
    ) external view virtual returns (bool);
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

