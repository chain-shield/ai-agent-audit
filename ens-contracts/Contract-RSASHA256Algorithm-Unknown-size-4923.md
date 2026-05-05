
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

pragma solidity ^0.8.4;

import "./Algorithm.sol";
import "./RSAPKCS1Verify.sol";
import "../../utils/BytesUtils.sol";

/// @dev Implements the DNSSEC RSASHA256 algorithm.
contract RSASHA256Algorithm is Algorithm {
    using BytesUtils for *;

    function verify(
        bytes calldata key,
        bytes calldata data,
        bytes calldata sig
    ) external view override returns (bool) {
        bytes memory exponent;
        bytes memory modulus;

        uint16 exponentLen = uint16(key.readUint8(4));
        if (exponentLen != 0) {
            exponent = key.substring(5, exponentLen);
            modulus = key.substring(
                exponentLen + 5,
                key.length - exponentLen - 5
            );
        } else {
            exponentLen = key.readUint16(5);
            exponent = key.substring(7, exponentLen);
            modulus = key.substring(
                exponentLen + 7,
                key.length - exponentLen - 7
            );
        }

        return RSAPKCS1Verify.verifySHA256(modulus, exponent, sig, sha256(data));
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
pragma solidity ^0.8.4;

import "./ModexpPrecompile.sol";
import "../../utils/BytesUtils.sol";

library RSAVerify {
    /// @dev Recovers the input data from an RSA signature, returning the result in S.
    /// @param N The RSA public modulus.
    /// @param E The RSA public exponent.
    /// @param S The signature to recover.
    /// @return True if the recovery succeeded.
    function rsarecover(
        bytes memory N,
        bytes memory E,
        bytes memory S
    ) internal view returns (bool, bytes memory) {
        return ModexpPrecompile.modexp(S, E, N);
    }
}

pragma solidity ^0.8.4;

import "./RSAVerify.sol";
import "../../utils/BytesUtils.sol";

/// @dev Library for PKCS#1 v1.5 signature verification
library RSAPKCS1Verify {
    using BytesUtils for *;

    /// @dev PKCS#1 v1.5 DigestInfo prefix for SHA-1
    bytes constant SHA1_DIGEST_INFO = hex"3021300906052b0e03021a05000414";

    /// @dev PKCS#1 v1.5 DigestInfo prefix for SHA-256
    bytes constant SHA256_DIGEST_INFO = hex"3031300d060960864801650304020105000420";

    /// @dev Verifies an RSA signature with PKCS#1 v1.5 padding for SHA-1
    function verifySHA1(
        bytes memory modulus,
        bytes memory exponent,
        bytes memory sig,
        bytes20 hash
    ) internal view returns (bool) {
        (bool ok, bytes memory result) = recoverAndVerify(modulus, exponent, sig, SHA1_DIGEST_INFO);
        return ok && hash == result.readBytes20(result.length - 20);
    }

    /// @dev Verifies an RSA signature with PKCS#1 v1.5 padding for SHA-256
    function verifySHA256(
        bytes memory modulus,
        bytes memory exponent,
        bytes memory sig,
        bytes32 hash
    ) internal view returns (bool) {
        (bool ok, bytes memory result) = recoverAndVerify(modulus, exponent, sig, SHA256_DIGEST_INFO);
        return ok && hash == result.readBytes32(result.length - 32);
    }

    /// @dev Recovers RSA signature and verifies PKCS#1 v1.5 structure
    /// Format: 0x00 0x01 [0xFF padding] 0x00 [DigestInfo] [Hash]
    /// https://datatracker.ietf.org/doc/html/rfc8017#section-9.2
    function recoverAndVerify(
        bytes memory modulus,
        bytes memory exponent,
        bytes memory sig,
        bytes memory digestInfo
    ) private view returns (bool, bytes memory) {
        (bool ok, bytes memory result) = RSAVerify.rsarecover(modulus, exponent, sig);
        if (!ok || result.length != modulus.length) {
            return (false, result);
        }

        // Check leading bytes: 0x00 0x01
        if (result[0] != 0x00 || result[1] != 0x01) {
            return (false, result);
        }

        // Calculate positions working backwards from the end
        uint256 hashLen = digestInfo.length == 15 ? 20 : 32;
        uint256 hashStart = result.length - hashLen;
        uint256 digestInfoStart = hashStart - digestInfo.length;

        // Verify 0x00 separator before DigestInfo
        if (result[digestInfoStart - 1] != 0x00) {
            return (false, result);
        }

        // Verify DigestInfo matches expected value
        if (!result.equals(digestInfoStart, digestInfo, 0, digestInfo.length)) {
            return (false, result);
        }

        // Verify padding: all bytes from position 2 to separator must be 0xFF
        // Minimum 8 bytes of 0xFF padding required (RFC 3447)
        uint256 paddingLen = digestInfoStart - 1 - 2;
        if (paddingLen < 8) {
            return (false, result);
        }
        for (uint256 i = 2; i < digestInfoStart - 1; i++) {
            if (result[i] != 0xFF) {
                return (false, result);
            }
        }

        return (true, result);
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

//SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

library LibMem {
    /// @dev Copy `mem[src:src+len]` to `mem[dst:dst+len]`.
    ///      Equivalent to `mcopy()`.
    ///
    /// @param src The source memory offset.
    /// @param dst The destination memory offset.
    /// @param len The number of bytes to copy.
    function copy(uint256 dst, uint256 src, uint256 len) internal pure {
        assembly {
            // Copy word-length chunks while possible
            // prettier-ignore
            for {} gt(len, 31) {} {
                mstore(dst, mload(src))
                dst := add(dst, 32)
                src := add(src, 32)
                len := sub(len, 32)
            }
            // Copy remaining bytes
            if len {
                let mask := sub(shl(shl(3, sub(32, len)), 1), 1)
                let wSrc := and(mload(src), not(mask))
                let wDst := and(mload(dst), mask)
                mstore(dst, or(wSrc, wDst))
            }
        }
    }

    /// @dev Convert bytes to a memory offset.
    ///
    /// @param v The bytes to convert.
    ///
    /// @return ret The corresponding memory offset.
    function ptr(bytes memory v) internal pure returns (uint256 ret) {
        assembly {
            ret := add(v, 32)
        }
    }

    /// @dev Read word at memory offset.
    ///
    /// @param src The memory offset.
    ///
    /// @return ret The read word.
    function load(uint256 src) internal pure returns (uint256 ret) {
        assembly {
            ret := mload(src)
        }
    }
}

//SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import {LibMem} from "./LibMem/LibMem.sol";

library BytesUtils {
    /// @dev `offset` was beyond `length`.
    ///       Error selector: `0x8a3c1cfb`
    error OffsetOutOfBoundsError(uint256 offset, uint256 length);

    /// @dev Assert `end` is not beyond the length of `v`.
    function _checkBound(bytes memory v, uint256 end) internal pure {
        if (end > v.length) {
            revert OffsetOutOfBoundsError(end, v.length);
        }
    }

    /// @dev Compute `keccak256(v[off:off+len])`.
    /// @param v The source bytes.
    /// @param off The offset into the source.
    /// @param len The number of bytes to hash.
    /// @return ret The corresponding hash.
    function keccak(
        bytes memory v,
        uint256 off,
        uint256 len
    ) internal pure returns (bytes32 ret) {
        _checkBound(v, off + len);
        assembly ("memory-safe") {
            ret := keccak256(add(add(v, 32), off), len)
        }
    }

    /// @dev Lexicographically compare two byte strings.
    /// @param vA The first bytes to compare.
    /// @param vB The second bytes to compare.
    /// @return Positive number if `A > B`, negative number if `A < B`, or zero if `A == B`.
    function compare(
        bytes memory vA,
        bytes memory vB
    ) internal pure returns (int256) {
        return compare(vA, 0, vA.length, vB, 0, vB.length);
    }

    /// @dev Lexicographically compare two byte ranges: `A = vA[offA:offA+lenA]` and `B = vB[offB:offB+lenB]`.
    /// @param vA The first bytes.
    /// @param offA The offset of the first bytes.
    /// @param lenA The length of the first bytes.
    /// @param vB The second bytes.
    /// @param offB The offset of the second bytes.
    /// @param lenB The length of the second bytes.
    /// @return Positive number if `A > B`, negative number if `A < B`, or zero if `A == B`.
    function compare(
        bytes memory vA,
        uint256 offA,
        uint256 lenA,
        bytes memory vB,
        uint256 offB,
        uint256 lenB
    ) internal pure returns (int256) {
        _checkBound(vA, offA + lenA);
        _checkBound(vB, offB + lenB);
        unchecked {
            uint256 ptrA = LibMem.ptr(vA) + offA;
            uint256 ptrB = LibMem.ptr(vB) + offB;
            uint256 shortest = lenA < lenB ? lenA : lenB;
            for (uint256 i; i < shortest; i += 32) {
                uint256 a = LibMem.load(ptrA + i);
                uint256 b = LibMem.load(ptrB + i);
                if (a != b) {
                    uint256 rest = shortest - i;
                    if (rest < 32) {
                        rest = (32 - rest) << 3; // bits to drop
                        a >>= rest; // shift out the
                        b >>= rest; // irrelevant bits
                    }
                    if (a < b) {
                        return -1;
                    } else if (a > b) {
                        return 1;
                    }
                }
            }
        }
        return int256(lenA) - int256(lenB);
    }

    /// @dev Determine if `a[offA:offA+len] == b[offB:offB+len]`.
    /// @param vA The first bytes.
    /// @param offA The offset into the first bytes.
    /// @param vB The second bytes.
    /// @param offB The offset into the second bytes.
    /// @param len The number of bytes to compare.
    /// @return True if the byte ranges are equal.
    function equals(
        bytes memory vA,
        uint256 offA,
        bytes memory vB,
        uint256 offB,
        uint256 len
    ) internal pure returns (bool) {
        return keccak(vA, offA, len) == keccak(vB, offB, len);
    }

    /// @dev Determine if `a[offA:] == b[offB:]`.
    /// @param vA The first bytes.
    /// @param offA The offset into the first bytes.
    /// @param vB The second bytes.
    /// @param offB The offset into the second bytes.
    /// @return True if the byte ranges are equal.
    function equals(
        bytes memory vA,
        uint256 offA,
        bytes memory vB,
        uint256 offB
    ) internal pure returns (bool) {
        _checkBound(vA, offA);
        _checkBound(vB, offB);
        unchecked {
            return
                keccak(vA, offA, vA.length - offA) ==
                keccak(vB, offB, vB.length - offB);
        }
    }

    /// @dev Determine if `a[offA:] == b`.
    /// @param vA The first bytes.
    /// @param offA The offset into the first bytes.
    /// @param vB The second bytes.
    /// @return True if the byte ranges are equal.
    function equals(
        bytes memory vA,
        uint256 offA,
        bytes memory vB
    ) internal pure returns (bool) {
        return
            vA.length == offA + vB.length &&
            keccak(vA, offA, vB.length) == keccak256(vB);
    }

    /// @dev Determine if `a == b`.
    /// @param vA The first bytes.
    /// @param vB The second bytes.
    /// @return True if the bytes are equal.
    function equals(
        bytes memory vA,
        bytes memory vB
    ) internal pure returns (bool) {
        return vA.length == vB.length && keccak256(vA) == keccak256(vB);
    }

    /// @dev Returns `uint8(v[off])`.
    /// @param v The source bytes.
    /// @param off The offset into the source.
    /// @return The corresponding `uint8`.
    function readUint8(
        bytes memory v,
        uint256 off
    ) internal pure returns (uint8) {
        _checkBound(v, off + 1);
        unchecked {
            return uint8(v[off]);
        }
    }

    /// @dev Returns `uint16(bytes2(v[off:off+2]))`.
    /// @param v The source bytes.
    /// @param off The offset into the source.
    /// @return ret The corresponding `uint16`.
    function readUint16(
        bytes memory v,
        uint256 off
    ) internal pure returns (uint16 ret) {
        _checkBound(v, off + 2);
        assembly ("memory-safe") {
            ret := shr(240, mload(add(add(v, 32), off)))
        }
    }

    /// @dev Returns `uint32(bytes4(v[off:off+4]))`.
    /// @param v The source bytes.
    /// @param off The offset into the source.
    /// @return ret The corresponding `uint32`.
    function readUint32(
        bytes memory v,
        uint256 off
    ) internal pure returns (uint32 ret) {
        _checkBound(v, off + 4);
        assembly ("memory-safe") {
            ret := shr(224, mload(add(add(v, 32), off)))
        }
    }

    /// @dev Returns `bytes20(v[off:off+20])`.
    /// @param v The source bytes.
    /// @param off The offset into the source.
    /// @return ret The corresponding `bytes20`.
    function readBytes20(
        bytes memory v,
        uint256 off
    ) internal pure returns (bytes20 ret) {
        _checkBound(v, off + 20);
        assembly ("memory-safe") {
            ret := shl(96, mload(add(add(v, 20), off)))
        }
    }

    /// @dev Returns `bytes32(v[off:off+32])`.
    /// @param v The source bytes.
    /// @param off The offset into the source.
    /// @return ret The corresponding `bytes32`.
    function readBytes32(
        bytes memory v,
        uint256 off
    ) internal pure returns (bytes32 ret) {
        _checkBound(v, off + 32);
        assembly ("memory-safe") {
            ret := mload(add(add(v, 32), off))
        }
    }

    /// @dev Returns `bytes32(bytesN(v[off:off+len]))`.
    ///      Accepts 0-32 bytes or reverts.
    /// @param v The source bytes.
    /// @param off The offset into the source.
    /// @param len The number of bytes.
    /// @return ret The corresponding N-bytes left-aligned in a `bytes32`.
    function readBytesN(
        bytes memory v,
        uint256 off,
        uint256 len
    ) internal pure returns (bytes32 ret) {
        assert(len <= 32);
        _checkBound(v, off + len);
        assembly ("memory-safe") {
            let mask := sub(shl(shl(3, sub(32, len)), 1), 1) // <(32-N)x00><NxFF>
            ret := and(mload(add(add(v, 32), off)), not(mask))
        }
    }

    /// @dev Copy `vSrc[offSrc:offSrc+len]` to `vDst[offDst:offDst:len]`.
    /// @param vSrc The source bytes.
    /// @param offSrc The offset into the source to begin the copy.
    /// @param vDst The destination bytes.
    /// @param offDst The offset into the destination to place the copy.
    /// @param len The number of bytes to copy.
    function copyBytes(
        bytes memory vSrc,
        uint256 offSrc,
        bytes memory vDst,
        uint256 offDst,
        uint256 len
    ) internal pure {
        _checkBound(vSrc, offSrc + len);
        _checkBound(vDst, offDst + len);
        unchecked {
            LibMem.copy(
                LibMem.ptr(vDst) + offDst,
                LibMem.ptr(vSrc) + offSrc,
                len
            );
        }
    }

    /// @dev Copies a substring into a new byte string.
    /// @param vSrc The byte string to copy from.
    /// @param off The offset to start copying at.
    /// @param len The number of bytes to copy.
    /// @return vDst The copied substring.
    function substring(
        bytes memory vSrc,
        uint256 off,
        uint256 len
    ) internal pure returns (bytes memory vDst) {
        vDst = new bytes(len);
        copyBytes(vSrc, off, vDst, 0, len);
    }

    /// @dev Find the first occurrence of `needle`.
    /// @param v The bytes to search.
    /// @param off The offset to start searching.
    /// @param len The number of bytes to search.
    /// @param needle The byte to search for.
    /// @return The offset of `needle`, or `type(uint256).max` if not found.
    function find(
        bytes memory v,
        uint256 off,
        uint256 len,
        bytes1 needle
    ) internal pure returns (uint256) {
        for (uint256 end = off + len; off < end; off++) {
            if (v[off] == needle) {
                return off;
            }
        }
        return type(uint256).max;
    }

    /// @dev Returns `true` if word contains a zero byte.
    function hasZeroByte(uint256 word) internal pure returns (bool) {
        unchecked {
            return
                ((~word &
                    (word -
                        0x0101010101010101010101010101010101010101010101010101010101010101)) &
                    0x8080808080808080808080808080808080808080808080808080808080808080) !=
                0;
        }
    }

    /// @dev Efficiently check if `v[off:off+len]` contains `needle` byte.
    /// @param v The source bytes.
    /// @param off The offset into the source.
    /// @param len The number of bytes to search.
    /// @param needle The byte to search for.
    /// @return found `true` if `needle` was found.
    function includes(
        bytes memory v,
        uint256 off,
        uint256 len,
        bytes1 needle
    ) internal pure returns (bool found) {
        _checkBound(v, off + len);
        unchecked {
            uint256 wide = uint8(needle);
            wide |= wide << 8;
            wide |= wide << 16;
            wide |= wide << 32;
            wide |= wide << 64;
            wide |= wide << 128; // broadcast byte across word
            off += LibMem.ptr(v);
            len += off;
            while (off < len) {
                uint256 word = LibMem.load(off) ^ wide; // zero needle byte
                off += 32;
                if (hasZeroByte(word)) {
                    return
                        off <= len ||
                        hasZeroByte(
                            word | ((1 << ((off - len) << 3)) - 1) // recheck overflow by making it nonzero
                        );
                }
            }
        }
    }
}

pragma solidity ^0.8.4;

library ModexpPrecompile {
    /// @dev Computes (base ^ exponent) % modulus over big numbers.
    function modexp(
        bytes memory base,
        bytes memory exponent,
        bytes memory modulus
    ) internal view returns (bool success, bytes memory output) {
        bytes memory input = abi.encodePacked(
            uint256(base.length),
            uint256(exponent.length),
            uint256(modulus.length),
            base,
            exponent,
            modulus
        );

        output = new bytes(modulus.length);

        assembly {
            success := staticcall(
                gas(),
                5,
                add(input, 32),
                mload(input),
                add(output, 32),
                mload(modulus)
            )
        }
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

