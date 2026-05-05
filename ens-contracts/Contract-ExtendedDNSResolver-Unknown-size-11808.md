
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "../../resolvers/profiles/IExtendedDNSResolver.sol";
import "../../resolvers/profiles/IAddressResolver.sol";
import "../../resolvers/profiles/IAddrResolver.sol";
import "../../resolvers/profiles/ITextResolver.sol";
import "../../utils/HexUtils.sol";
import "../../utils/BytesUtils.sol";

/// @dev Resolves names on ENS by interpreting record data stored in a DNS TXT record.
///      This resolver implements the IExtendedDNSResolver interface, meaning that when
///      a DNS name specifies it as the resolver via a TXT record, this resolver's
///      resolve() method is invoked, and is passed any additional information from that
///      text record. This resolver implements a simple text parser allowing a variety
///      of records to be specified in text, which will then be used to resolve the name
///      in ENS.
///
///      To use this, set a TXT record on your DNS name in the following format:
///          ENS1 <address or name of ExtendedDNSResolver> <record data>
///
///      For example:
///          ENS1 2.dnsname.ens.eth a[60]=0x1234...
///
///      The record data consists of a series of key=value pairs, separated by spaces. Keys
///      may have an optional argument in square brackets, and values may be either unquoted
///       - in which case they may not contain spaces - or single-quoted. Single quotes in
///      a quoted value may be backslash-escaped.
///
///
///                                       ┌────────┐
///                                       │ ┌───┐  │
///        ┌──────────────────────────────┴─┤" "│◄─┴────────────────────────────────────────┐
///        │                                └───┘                                           │
///        │  ┌───┐    ┌───┐    ┌───┐    ┌───┐    ┌───┐    ┌───┐    ┌────────────┐    ┌───┐ │
///      ^─┴─►│key├─┬─►│"["├───►│arg├───►│"]"├─┬─►│"="├─┬─►│"'"├───►│quoted_value├───►│"'"├─┼─$
///           └───┘ │  └───┘    └───┘    └───┘ │  └───┘ │  └───┘    └────────────┘    └───┘ │
///                 └──────────────────────────┘        │          ┌──────────────┐         │
///                                                     └─────────►│unquoted_value├─────────┘
///                                                                └──────────────┘
///
///      Record types:
///       - a[<coinType>] - Specifies how an `addr()` request should be resolved for the specified
///         `coinType`. Ethereum has `coinType` 60. The value must be 0x-prefixed hexadecimal, and will
///         be returned unmodified; this means that non-EVM addresses will need to be translated
///         into binary format and then encoded in hex.
///         Examples:
///          - a[60]=0xFe89cc7aBB2C4183683ab71653C4cdc9B02D44b7
///          - a[0]=0x00149010587f8364b964fcaa70687216b53bd2cbd798
///       - a[e<chainId>] - Specifies how an `addr()` request should be resolved for the specified
///         `chainId`. The value must be 0x-prefixed hexadecimal. When encoding an address for an
///         EVM-based cryptocurrency that uses a chainId instead of a coinType, this syntax *must*
///         be used in place of the coin type - eg, Optimism is `a[e10]`, not `a[2147483658]`.
///         A list of supported cryptocurrencies for both syntaxes can be found here:
///           https://github.com/ensdomains/address-encoder/blob/master/docs/supported-cryptocurrencies.md
///         Example:
///          - a[e10]=0xFe89cc7aBB2C4183683ab71653C4cdc9B02D44b7
///       - t[<key>] - Specifies how a `text()` request should be resolved for the specified `key`.
///         Examples:
///          - t[com.twitter]=nicksdjohnson
///          - t[url]='https://ens.domains/'
///          - t[note]='I\'m great'
contract ExtendedDNSResolver is IExtendedDNSResolver, IERC165 {
    using HexUtils for *;
    using BytesUtils for *;
    using Strings for *;

    uint256 private constant COIN_TYPE_ETH = 60;

    error NotImplemented();
    error InvalidAddressFormat(bytes addr);

    function supportsInterface(
        bytes4 interfaceId
    ) external view virtual override returns (bool) {
        return interfaceId == type(IExtendedDNSResolver).interfaceId;
    }

    function resolve(
        bytes calldata /* name */,
        bytes calldata data,
        bytes calldata context
    ) external pure override returns (bytes memory) {
        bytes4 selector = bytes4(data);
        if (selector == IAddrResolver.addr.selector) {
            return _resolveAddr(context);
        } else if (selector == IAddressResolver.addr.selector) {
            return _resolveAddress(data, context);
        } else if (selector == ITextResolver.text.selector) {
            return _resolveText(data, context);
        }
        revert NotImplemented();
    }

    function _resolveAddress(
        bytes calldata data,
        bytes calldata context
    ) internal pure returns (bytes memory) {
        (, uint256 coinType) = abi.decode(data[4:], (bytes32, uint256));
        bytes memory value;
        // Per https://docs.ens.domains/ensip/11#specification
        if (coinType & 0x80000000 != 0) {
            value = _findValue(
                context,
                bytes.concat(
                    "a[e",
                    bytes((coinType & 0x7fffffff).toString()),
                    "]="
                )
            );
        } else {
            value = _findValue(
                context,
                bytes.concat("a[", bytes(coinType.toString()), "]=")
            );
        }
        if (value.length == 0) {
            return value;
        }
        (address record, bool valid) = value.hexToAddress(2, value.length);
        if (!valid) revert InvalidAddressFormat(value);
        return abi.encode(record);
    }

    function _resolveAddr(
        bytes calldata context
    ) internal pure returns (bytes memory) {
        bytes memory value = _findValue(context, "a[60]=");
        if (value.length == 0) {
            return value;
        }
        (address record, bool valid) = value.hexToAddress(2, value.length);
        if (!valid) revert InvalidAddressFormat(value);
        return abi.encode(record);
    }

    function _resolveText(
        bytes calldata data,
        bytes calldata context
    ) internal pure returns (bytes memory) {
        (, string memory key) = abi.decode(data[4:], (bytes32, string));
        bytes memory value = _findValue(
            context,
            bytes.concat("t[", bytes(key), "]=")
        );
        return abi.encode(value);
    }

    uint256 constant STATE_START = 0;
    uint256 constant STATE_IGNORED_KEY = 1;
    uint256 constant STATE_IGNORED_KEY_ARG = 2;
    uint256 constant STATE_VALUE = 3;
    uint256 constant STATE_QUOTED_VALUE = 4;
    uint256 constant STATE_UNQUOTED_VALUE = 5;
    uint256 constant STATE_IGNORED_VALUE = 6;
    uint256 constant STATE_IGNORED_QUOTED_VALUE = 7;
    uint256 constant STATE_IGNORED_UNQUOTED_VALUE = 8;

    /// @dev Implements a DFA to parse the text record, looking for an entry
    ///      matching `key`.
    /// @param data The text record to parse.
    /// @param key The exact key to search for.
    /// @return value The value if found, or an empty string if `key` does not exist.
    function _findValue(
        bytes memory data,
        bytes memory key
    ) internal pure returns (bytes memory value) {
        // Here we use a simple state machine to parse the text record. We
        // process characters one at a time; each character can trigger a
        // transition to a new state, or terminate the DFA and return a value.
        // For states that expect to process a number of tokens, we use
        // inner loops for efficiency reasons, to avoid the need to go
        // through the outer loop and switch statement for every character.
        uint256 state = STATE_START;
        uint256 len = data.length;
        for (uint256 i = 0; i < len; ) {
            if (state == STATE_START) {
                // Look for a matching key.
                if (data.equals(i, key, 0, key.length)) {
                    i += key.length;
                    state = STATE_VALUE;
                } else {
                    state = STATE_IGNORED_KEY;
                }
            } else if (state == STATE_IGNORED_KEY) {
                for (; i < len; i++) {
                    if (data[i] == "=") {
                        state = STATE_IGNORED_VALUE;
                        i += 1;
                        break;
                    } else if (data[i] == "[") {
                        state = STATE_IGNORED_KEY_ARG;
                        i += 1;
                        break;
                    }
                }
            } else if (state == STATE_IGNORED_KEY_ARG) {
                for (; i < len; i++) {
                    if (data[i] == "]") {
                        state = STATE_IGNORED_VALUE;
                        i += 1;
                        if (data[i] == "=") {
                            i += 1;
                        }
                        break;
                    }
                }
            } else if (state == STATE_VALUE) {
                if (data[i] == "'") {
                    state = STATE_QUOTED_VALUE;
                    i += 1;
                } else {
                    state = STATE_UNQUOTED_VALUE;
                }
            } else if (state == STATE_QUOTED_VALUE) {
                uint256 start = i;
                uint256 valueLen = 0;
                bool escaped = false;
                for (; i < len; i++) {
                    if (escaped) {
                        data[start + valueLen] = data[i];
                        valueLen += 1;
                        escaped = false;
                    } else {
                        if (data[i] == "\\") {
                            escaped = true;
                        } else if (data[i] == "'") {
                            return data.substring(start, valueLen);
                        } else {
                            data[start + valueLen] = data[i];
                            valueLen += 1;
                        }
                    }
                }
            } else if (state == STATE_UNQUOTED_VALUE) {
                uint256 start = i;
                for (; i < len; i++) {
                    if (data[i] == " ") {
                        return data.substring(start, i - start);
                    }
                }
                return data.substring(start, len - start);
            } else if (state == STATE_IGNORED_VALUE) {
                if (data[i] == "'") {
                    state = STATE_IGNORED_QUOTED_VALUE;
                    i += 1;
                } else {
                    state = STATE_IGNORED_UNQUOTED_VALUE;
                }
            } else if (state == STATE_IGNORED_QUOTED_VALUE) {
                bool escaped = false;
                for (; i < len; i++) {
                    if (escaped) {
                        escaped = false;
                    } else {
                        if (data[i] == "\\") {
                            escaped = true;
                        } else if (data[i] == "'") {
                            i += 1;
                            while (data[i] == " ") {
                                i += 1;
                            }
                            state = STATE_START;
                            break;
                        }
                    }
                }
            } else {
                assert(state == STATE_IGNORED_UNQUOTED_VALUE);
                for (; i < len; i++) {
                    if (data[i] == " ") {
                        while (data[i] == " ") {
                            i += 1;
                        }
                        state = STATE_START;
                        break;
                    }
                }
            }
        }
        return "";
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.1.0) (utils/Strings.sol)

pragma solidity ^0.8.20;

import {Math} from "./math/Math.sol";
import {SignedMath} from "./math/SignedMath.sol";

/**
 * @dev String operations.
 */
library Strings {
    bytes16 private constant HEX_DIGITS = "0123456789abcdef";
    uint8 private constant ADDRESS_LENGTH = 20;

    /**
     * @dev The `value` string doesn't fit in the specified `length`.
     */
    error StringsInsufficientHexLength(uint256 value, uint256 length);

    /**
     * @dev Converts a `uint256` to its ASCII `string` decimal representation.
     */
    function toString(uint256 value) internal pure returns (string memory) {
        unchecked {
            uint256 length = Math.log10(value) + 1;
            string memory buffer = new string(length);
            uint256 ptr;
            assembly ("memory-safe") {
                ptr := add(buffer, add(32, length))
            }
            while (true) {
                ptr--;
                assembly ("memory-safe") {
                    mstore8(ptr, byte(mod(value, 10), HEX_DIGITS))
                }
                value /= 10;
                if (value == 0) break;
            }
            return buffer;
        }
    }

    /**
     * @dev Converts a `int256` to its ASCII `string` decimal representation.
     */
    function toStringSigned(int256 value) internal pure returns (string memory) {
        return string.concat(value < 0 ? "-" : "", toString(SignedMath.abs(value)));
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` hexadecimal representation.
     */
    function toHexString(uint256 value) internal pure returns (string memory) {
        unchecked {
            return toHexString(value, Math.log256(value) + 1);
        }
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` hexadecimal representation with fixed length.
     */
    function toHexString(uint256 value, uint256 length) internal pure returns (string memory) {
        uint256 localValue = value;
        bytes memory buffer = new bytes(2 * length + 2);
        buffer[0] = "0";
        buffer[1] = "x";
        for (uint256 i = 2 * length + 1; i > 1; --i) {
            buffer[i] = HEX_DIGITS[localValue & 0xf];
            localValue >>= 4;
        }
        if (localValue != 0) {
            revert StringsInsufficientHexLength(value, length);
        }
        return string(buffer);
    }

    /**
     * @dev Converts an `address` with fixed length of 20 bytes to its not checksummed ASCII `string` hexadecimal
     * representation.
     */
    function toHexString(address addr) internal pure returns (string memory) {
        return toHexString(uint256(uint160(addr)), ADDRESS_LENGTH);
    }

    /**
     * @dev Converts an `address` with fixed length of 20 bytes to its checksummed ASCII `string` hexadecimal
     * representation, according to EIP-55.
     */
    function toChecksumHexString(address addr) internal pure returns (string memory) {
        bytes memory buffer = bytes(toHexString(addr));

        // hash the hex part of buffer (skip length + 2 bytes, length 40)
        uint256 hashValue;
        assembly ("memory-safe") {
            hashValue := shr(96, keccak256(add(buffer, 0x22), 40))
        }

        for (uint256 i = 41; i > 1; --i) {
            // possible values for buffer[i] are 48 (0) to 57 (9) and 97 (a) to 102 (f)
            if (hashValue & 0xf > 7 && uint8(buffer[i]) > 96) {
                // case shift by xoring with 0x20
                buffer[i] ^= 0x20;
            }
            hashValue >>= 4;
        }
        return string(buffer);
    }

    /**
     * @dev Returns true if the two strings are equal.
     */
    function equal(string memory a, string memory b) internal pure returns (bool) {
        return bytes(a).length == bytes(b).length && keccak256(bytes(a)) == keccak256(bytes(b));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

library HexUtils {
    /// @dev Convert `hexString[off:end]` to `bytes32`.
    ///      Accepts 0-64 hex-chars.
    ///      Uses right alignment: `1` &rarr; `0000000000000000000000000000000000000000000000000000000000000001`.
    /// @param hexString The string to parse.
    /// @param off The index to start parsing.
    /// @param end The (exclusive) index to stop parsing.
    /// @return word The parsed bytes32.
    /// @return valid True if the parse was successful.
    function hexStringToBytes32(
        bytes memory hexString,
        uint256 off,
        uint256 end
    ) internal pure returns (bytes32 word, bool valid) {
        if (end < off) return ("", false); // invalid range
        uint256 nibbles = end - off;
        if (nibbles > 64 || end > hexString.length) {
            return (bytes32(0), false); // too large or out of bounds
        }
        uint256 src;
        assembly {
            src := add(add(hexString, 32), off)
        }
        valid = unsafeBytes(src, 0, nibbles);
        assembly {
            let pad := sub(32, shr(1, add(nibbles, 1))) // number of bytes
            word := shr(shl(3, pad), mload(0)) // right align
        }
    }

    /// @dev Convert `hexString[off:end]` to `address`.
    ///      Accepts exactly 40 hex-chars.
    /// @param hexString The string to parse.
    /// @param off The index to start parsing.
    /// @param end The (exclusive) index to stop parsing.
    /// @return addr The parsed address.
    /// @return valid True if the parse was successful.
    function hexToAddress(
        bytes memory hexString,
        uint256 off,
        uint256 end
    ) internal pure returns (address addr, bool valid) {
        if (off + 40 != end) return (address(0), false); // wrong length
        bytes32 word;
        (word, valid) = hexStringToBytes32(hexString, off, end);
        addr = address(uint160(uint256(word)));
    }

    /// @dev Convert `hexString[off:end]` to `bytes`.
    ///      Accepts 0+ hex-chars.
    /// @param hexString The string to parse.
    /// @param off The index to start parsing.
    /// @param end The (exclusive) index to stop parsing.
    /// @return v The parsed bytes.
    /// @return valid True if the parse was successful.
    function hexToBytes(
        bytes memory hexString,
        uint256 off,
        uint256 end
    ) internal pure returns (bytes memory v, bool valid) {
        if (end < off) return ("", false); // invalid range
        uint256 nibbles = end - off;
        v = new bytes((1 + nibbles) >> 1); // round up
        uint256 src;
        uint256 dst;
        assembly {
            src := add(add(hexString, 32), off)
            dst := add(v, 32)
        }
        valid = unsafeBytes(src, dst, nibbles);
    }

    /// @dev Convert arbitrary hex-encoded memory to bytes.
    ///      If nibbles is odd, leading hex-char is padded, eg. `F` &rarr; `0x0F`.
    ///      Matches: `/^[0-9a-f]*$/i`.
    /// @param src The memory offset of first hex-char of input.
    /// @param dst The memory offset of first byte of output (cannot alias `src`).
    /// @param nibbles The number of hex-chars to convert.
    /// @return valid True if all characters were hex.
    function unsafeBytes(
        uint256 src,
        uint256 dst,
        uint256 nibbles
    ) internal pure returns (bool valid) {
        assembly {
            function getHex(c, i) -> ascii {
                c := byte(i, c)
                // chars 48-57: 0-9
                if and(gt(c, 47), lt(c, 58)) {
                    ascii := sub(c, 48)
                    leave
                }
                // chars 65-70: A-F
                if and(gt(c, 64), lt(c, 71)) {
                    ascii := add(sub(c, 65), 10)
                    leave
                }
                // chars 97-102: a-f
                if and(gt(c, 96), lt(c, 103)) {
                    ascii := add(sub(c, 97), 10)
                    leave
                }
                // invalid char
                ascii := 0x100
            }
            valid := true
            let end := add(src, nibbles)
            if and(nibbles, 1) {
                let b := getHex(mload(src), 0) // "f" -> 15
                mstore8(dst, b) // write ascii byte
                src := add(src, 1) // update pointers
                dst := add(dst, 1)
                if gt(b, 255) {
                    valid := false
                    src := end // terminate loop
                }
            }
            // prettier-ignore
            for {} lt(src, end) {
                src := add(src, 2) // 2 nibbles
                dst := add(dst, 1) // per byte
            } {
                let word := mload(src) // read word (left aligned)
                let b := or(shl(4, getHex(word, 0)), getHex(word, 1)) // "ff" -> 255
                if gt(b, 255) {
                    valid := false
                    break
                }
                mstore8(dst, b) // write ascii byte
            }
        }
    }

    /// @dev Format `address` as a hex string.
    /// @param addr The address to format.
    /// @return hexString The corresponding hex string w/o a 0x-prefix.
    function addressToHex(
        address addr
    ) internal pure returns (string memory hexString) {
        // return bytesToHex(abi.encodePacked(addr));
        hexString = new string(40);
        uint256 dst;
        assembly {
            mstore(0, addr)
            dst := add(hexString, 32)
        }
        unsafeHex(12, dst, 40);
    }

    /// @dev Format `uint256` as a variable-length hex string without zero padding.
    /// * unpaddedUintToHex(0, true)  = "0"
    /// * unpaddedUintToHex(1, true)  = "1"
    /// * unpaddedUintToHex(0, false) = "00"
    /// * unpaddedUintToHex(1, false) = "01"
    /// @param value The number to format.
    /// @param dropZeroNibble If true, the leading byte will use one nibble if less than 16.
    /// @return hexString The corresponding hex string w/o an 0x-prefix.
    function unpaddedUintToHex(
        uint256 value,
        bool dropZeroNibble
    ) internal pure returns (string memory hexString) {
        uint256 temp = value;
        uint256 shift;
        for (uint256 b = 128; b >= 8; b >>= 1) {
            if (temp < (1 << b)) {
                shift += b; // number of zero upper bits
            } else {
                temp >>= b; // shift away lower half
            }
        }
        if (dropZeroNibble && temp < 16) shift += 4;
        uint256 nibbles = 64 - (shift >> 2);
        hexString = new string(nibbles);
        uint256 dst;
        assembly {
            mstore(0, shl(shift, value)) // left-align
            dst := add(hexString, 32)
        }
        unsafeHex(0, dst, nibbles);
    }

    /// @dev Format `bytes` as a hex string.
    /// @param v The bytes to format.
    /// @return hexString The corresponding hex string w/o a 0x-prefix.
    function bytesToHex(
        bytes memory v
    ) internal pure returns (string memory hexString) {
        uint256 nibbles = v.length << 1;
        hexString = new string(nibbles);
        uint256 src;
        uint256 dst;
        assembly {
            src := add(v, 32)
            dst := add(hexString, 32)
        }
        unsafeHex(src, dst, nibbles);
    }

    /// @dev Converts arbitrary memory to a hex string.
    /// @param src The memory offset of first nibble of input.
    /// @param dst The memory offset of first hex-char of output (can alias `src`).
    /// @param nibbles The number of nibbles to convert and the byte-length of the output.
    function unsafeHex(
        uint256 src,
        uint256 dst,
        uint256 nibbles
    ) internal pure {
        unchecked {
            for (uint256 end = dst + nibbles; dst < end; src += 32) {
                uint256 word;
                assembly {
                    word := mload(src)
                }
                for (uint256 shift = 256; dst < end && shift > 0; dst++) {
                    uint256 b = (word >> (shift -= 4)) & 15; // each nibble
                    b = b < 10 ? b + 0x30 : b + 0x57; // ("a" - 10) => 0x57
                    assembly {
                        mstore8(dst, b)
                    }
                }
            }
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

interface IExtendedDNSResolver {
    function resolve(
        bytes memory name,
        bytes memory data,
        bytes memory context
    ) external view returns (bytes memory);
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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

library HexUtils {
    /// @dev Convert `hexString[off:end]` to `bytes32`.
    ///      Accepts 0-64 hex-chars.
    ///      Uses right alignment: `1` &rarr; `0000000000000000000000000000000000000000000000000000000000000001`.
    /// @param hexString The string to parse.
    /// @param off The index to start parsing.
    /// @param end The (exclusive) index to stop parsing.
    /// @return word The parsed bytes32.
    /// @return valid True if the parse was successful.
    function hexStringToBytes32(
        bytes memory hexString,
        uint256 off,
        uint256 end
    ) internal pure returns (bytes32 word, bool valid) {
        if (end < off) return ("", false); // invalid range
        uint256 nibbles = end - off;
        if (nibbles > 64 || end > hexString.length) {
            return (bytes32(0), false); // too large or out of bounds
        }
        uint256 src;
        assembly {
            src := add(add(hexString, 32), off)
        }
        valid = unsafeBytes(src, 0, nibbles);
        assembly {
            let pad := sub(32, shr(1, add(nibbles, 1))) // number of bytes
            word := shr(shl(3, pad), mload(0)) // right align
        }
    }

    /// @dev Convert `hexString[off:end]` to `address`.
    ///      Accepts exactly 40 hex-chars.
    /// @param hexString The string to parse.
    /// @param off The index to start parsing.
    /// @param end The (exclusive) index to stop parsing.
    /// @return addr The parsed address.
    /// @return valid True if the parse was successful.
    function hexToAddress(
        bytes memory hexString,
        uint256 off,
        uint256 end
    ) internal pure returns (address addr, bool valid) {
        if (off + 40 != end) return (address(0), false); // wrong length
        bytes32 word;
        (word, valid) = hexStringToBytes32(hexString, off, end);
        addr = address(uint160(uint256(word)));
    }

    /// @dev Convert `hexString[off:end]` to `bytes`.
    ///      Accepts 0+ hex-chars.
    /// @param hexString The string to parse.
    /// @param off The index to start parsing.
    /// @param end The (exclusive) index to stop parsing.
    /// @return v The parsed bytes.
    /// @return valid True if the parse was successful.
    function hexToBytes(
        bytes memory hexString,
        uint256 off,
        uint256 end
    ) internal pure returns (bytes memory v, bool valid) {
        if (end < off) return ("", false); // invalid range
        uint256 nibbles = end - off;
        v = new bytes((1 + nibbles) >> 1); // round up
        uint256 src;
        uint256 dst;
        assembly {
            src := add(add(hexString, 32), off)
            dst := add(v, 32)
        }
        valid = unsafeBytes(src, dst, nibbles);
    }

    /// @dev Convert arbitrary hex-encoded memory to bytes.
    ///      If nibbles is odd, leading hex-char is padded, eg. `F` &rarr; `0x0F`.
    ///      Matches: `/^[0-9a-f]*$/i`.
    /// @param src The memory offset of first hex-char of input.
    /// @param dst The memory offset of first byte of output (cannot alias `src`).
    /// @param nibbles The number of hex-chars to convert.
    /// @return valid True if all characters were hex.
    function unsafeBytes(
        uint256 src,
        uint256 dst,
        uint256 nibbles
    ) internal pure returns (bool valid) {
        assembly {
            function getHex(c, i) -> ascii {
                c := byte(i, c)
                // chars 48-57: 0-9
                if and(gt(c, 47), lt(c, 58)) {
                    ascii := sub(c, 48)
                    leave
                }
                // chars 65-70: A-F
                if and(gt(c, 64), lt(c, 71)) {
                    ascii := add(sub(c, 65), 10)
                    leave
                }
                // chars 97-102: a-f
                if and(gt(c, 96), lt(c, 103)) {
                    ascii := add(sub(c, 97), 10)
                    leave
                }
                // invalid char
                ascii := 0x100
            }
            valid := true
            let end := add(src, nibbles)
            if and(nibbles, 1) {
                let b := getHex(mload(src), 0) // "f" -> 15
                mstore8(dst, b) // write ascii byte
                src := add(src, 1) // update pointers
                dst := add(dst, 1)
                if gt(b, 255) {
                    valid := false
                    src := end // terminate loop
                }
            }
            // prettier-ignore
            for {} lt(src, end) {
                src := add(src, 2) // 2 nibbles
                dst := add(dst, 1) // per byte
            } {
                let word := mload(src) // read word (left aligned)
                let b := or(shl(4, getHex(word, 0)), getHex(word, 1)) // "ff" -> 255
                if gt(b, 255) {
                    valid := false
                    break
                }
                mstore8(dst, b) // write ascii byte
            }
        }
    }

    /// @dev Format `address` as a hex string.
    /// @param addr The address to format.
    /// @return hexString The corresponding hex string w/o a 0x-prefix.
    function addressToHex(
        address addr
    ) internal pure returns (string memory hexString) {
        // return bytesToHex(abi.encodePacked(addr));
        hexString = new string(40);
        uint256 dst;
        assembly {
            mstore(0, addr)
            dst := add(hexString, 32)
        }
        unsafeHex(12, dst, 40);
    }

    /// @dev Format `uint256` as a variable-length hex string without zero padding.
    /// * unpaddedUintToHex(0, true)  = "0"
    /// * unpaddedUintToHex(1, true)  = "1"
    /// * unpaddedUintToHex(0, false) = "00"
    /// * unpaddedUintToHex(1, false) = "01"
    /// @param value The number to format.
    /// @param dropZeroNibble If true, the leading byte will use one nibble if less than 16.
    /// @return hexString The corresponding hex string w/o an 0x-prefix.
    function unpaddedUintToHex(
        uint256 value,
        bool dropZeroNibble
    ) internal pure returns (string memory hexString) {
        uint256 temp = value;
        uint256 shift;
        for (uint256 b = 128; b >= 8; b >>= 1) {
            if (temp < (1 << b)) {
                shift += b; // number of zero upper bits
            } else {
                temp >>= b; // shift away lower half
            }
        }
        if (dropZeroNibble && temp < 16) shift += 4;
        uint256 nibbles = 64 - (shift >> 2);
        hexString = new string(nibbles);
        uint256 dst;
        assembly {
            mstore(0, shl(shift, value)) // left-align
            dst := add(hexString, 32)
        }
        unsafeHex(0, dst, nibbles);
    }

    /// @dev Format `bytes` as a hex string.
    /// @param v The bytes to format.
    /// @return hexString The corresponding hex string w/o a 0x-prefix.
    function bytesToHex(
        bytes memory v
    ) internal pure returns (string memory hexString) {
        uint256 nibbles = v.length << 1;
        hexString = new string(nibbles);
        uint256 src;
        uint256 dst;
        assembly {
            src := add(v, 32)
            dst := add(hexString, 32)
        }
        unsafeHex(src, dst, nibbles);
    }

    /// @dev Converts arbitrary memory to a hex string.
    /// @param src The memory offset of first nibble of input.
    /// @param dst The memory offset of first hex-char of output (can alias `src`).
    /// @param nibbles The number of nibbles to convert and the byte-length of the output.
    function unsafeHex(
        uint256 src,
        uint256 dst,
        uint256 nibbles
    ) internal pure {
        unchecked {
            for (uint256 end = dst + nibbles; dst < end; src += 32) {
                uint256 word;
                assembly {
                    word := mload(src)
                }
                for (uint256 shift = 256; dst < end && shift > 0; dst++) {
                    uint256 b = (word >> (shift -= 4)) & 15; // each nibble
                    b = b < 10 ? b + 0x30 : b + 0x57; // ("a" - 10) => 0x57
                    assembly {
                        mstore8(dst, b)
                    }
                }
            }
        }
    }
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

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

/// Interface for the new (multicoin) addr function.
interface IAddressResolver {
    event AddressChanged(
        bytes32 indexed node,
        uint256 coinType,
        bytes newAddress
    );

    function addr(
        bytes32 node,
        uint256 coinType
    ) external view returns (bytes memory);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

/// Interface for the legacy (ETH-only) addr function.
interface IAddrResolver {
    event AddrChanged(bytes32 indexed node, address a);

    /// Returns the address associated with an ENS node.
    /// @param node The ENS node to query.
    /// @return The associated address.
    function addr(bytes32 node) external view returns (address payable);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

interface IExtendedDNSResolver {
    function resolve(
        bytes memory name,
        bytes memory data,
        bytes memory context
    ) external view returns (bytes memory);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

interface ITextResolver {
    event TextChanged(
        bytes32 indexed node,
        string indexed indexedKey,
        string key,
        string value
    );

    /// Returns the text data associated with an ENS node and key.
    /// @param node The ENS node to query.
    /// @param key The text data key to query.
    /// @return The associated text data.
    function text(
        bytes32 node,
        string calldata key
    ) external view returns (string memory);
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

