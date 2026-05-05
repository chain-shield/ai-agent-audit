
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {
    ERC165
} from "@openzeppelin/contracts-v5/utils/introspection/ERC165.sol";
import {IExtendedResolver} from "../resolvers/profiles/IExtendedResolver.sol";
import {IAddressResolver} from "../resolvers/profiles/IAddressResolver.sol";
import {IAddrResolver} from "../resolvers/profiles/IAddrResolver.sol";
import {INameResolver} from "../resolvers/profiles/INameResolver.sol";
import {INameReverser} from "./INameReverser.sol";
import {IERC7996} from "../utils/IERC7996.sol";
import {ENSIP19, COIN_TYPE_DEFAULT, COIN_TYPE_ETH} from "../utils/ENSIP19.sol";

abstract contract AbstractReverseResolver is
    IExtendedResolver,
    INameReverser,
    IERC7996,
    ERC165
{
    /// @inheritdoc INameReverser
    uint256 public immutable coinType;

    /// @inheritdoc INameReverser
    address public immutable chainRegistrar;

    /// @notice `resolve()` was called with a profile other than `name()` or `addr(*)`.
    /// @dev Error selector: `0x7b1c461b`
    error UnsupportedResolverProfile(bytes4 selector);

    /// @notice `name` is not a valid DNS-encoded ENSIP-19 reverse name or namespace.
    /// @dev Error selector: `0x5fe9a5df`
    error UnreachableName(bytes name);

    constructor(uint256 _coinType, address registrar) {
        coinType = _coinType;
        chainRegistrar = registrar;
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override returns (bool) {
        return
            interfaceId == type(IExtendedResolver).interfaceId ||
            interfaceId == type(INameReverser).interfaceId ||
            interfaceId == type(IERC7996).interfaceId ||
            super.supportsInterface(interfaceId);
    }

    /// @inheritdoc IERC7996
    function supportsFeature(bytes4) external pure returns (bool) {
        return false;
    }

    /// @inheritdoc INameReverser
    function chainId() external view returns (uint32) {
        return ENSIP19.chainFromCoinType(coinType);
    }

    /// @dev Resolve one address to a name.
    ///      If this reverts `OffchainLookup`, it must return an abi-encoded result since
    ///      it is invoked during `resolve()`.
    function _resolveName(
        address addr
    ) internal view virtual returns (string memory name);

    /// @notice Resolves the following profiles according to ENSIP-10:
    ///         - `name()` if `name` is an ENSIP-19 reverse name of an EVM address for `coinType`.
    ///         - `addr(*) = registrar` if `name` is an ENSIP-19 reverse namespace for `coinType`.
    ///         Caller should enable EIP-3668.
    /// @dev This function may execute over multiple steps.
    /// @param name The reverse name to resolve, in normalised and DNS-encoded form.
    /// @param data The resolution data, as specified in ENSIP-10.
    /// @return result The encoded response for the requested profile.
    function resolve(
        bytes calldata name,
        bytes calldata data
    ) external view returns (bytes memory result) {
        bytes4 selector = bytes4(data);
        if (selector == INameResolver.name.selector) {
            (bytes memory a, uint256 ct) = ENSIP19.parse(name);
            if (
                a.length != 20 ||
                !(
                    coinType == COIN_TYPE_DEFAULT
                        ? ENSIP19.isEVMCoinType(ct)
                        : ct == coinType
                )
            ) {
                revert UnreachableName(name);
            }
            address addr = address(bytes20(a));
            return abi.encode(_resolveName(addr));
        } else if (selector == IAddrResolver.addr.selector) {
            (bool valid, ) = ENSIP19.parseNamespace(name, 0);
            if (!valid) revert UnreachableName(name);
            return
                abi.encode(
                    coinType == COIN_TYPE_ETH ? chainRegistrar : address(0)
                );
        } else if (selector == IAddressResolver.addr.selector) {
            (bool valid, ) = ENSIP19.parseNamespace(name, 0);
            if (!valid) revert UnreachableName(name);
            (, uint256 ct) = abi.decode(data[4:], (bytes32, uint256));
            return
                abi.encode(
                    coinType == ct
                        ? abi.encodePacked(chainRegistrar)
                        : new bytes(0)
                );
        } else {
            revert UnsupportedResolverProfile(selector);
        }
    }

    // `INameReverser.resolveNames()` is not implemented here because it causes
    // an incorrect "Unreachable code" compiler warning if `_resolveName()` reverts.
    // https://github.com/ethereum/solidity/issues/15426#issuecomment-2917868211
    //
    // /// @inheritdoc INameReverser
    // function resolveNames(
    //     address[] memory addrs,
    //     uint8 /*perPage*/
    // ) external view returns (string[] memory names) {
    //     names = new string[](addrs.length);
    //     for (uint256 i; i < addrs.length; i++) {
    //         names[i] = _resolveName(addrs[i]);
    //     }
    // }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @notice A resolver for primary name resolution.
/// @dev Interface selector: `0x6beeaa0d`
interface INameReverser {
    /// @notice Resolve multiple EVM addresses to names.
    ///         Caller should enable EIP-3668.
    /// @dev This function may execute over multiple steps.
    /// @param addrs The addresses to resolve.
    /// @return names The resolved names.
    function resolveNames(
        address[] memory addrs
    ) external view returns (string[] memory names);

    /// @notice The coin type for the resolver.
    function coinType() external view returns (uint256);

    /// @notice The EVM Chain ID derived from `coinType()`.
    function chainId() external view returns (uint32);

    /// @notice The reverse registrar address on the corresponding chain.
    ///         The address returned by `addr(coinType)` for the resolver.
    function chainRegistrar() external view returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

interface IExtendedResolver {
    function resolve(
        bytes memory name,
        bytes memory data
    ) external view returns (bytes memory);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @notice Interface for expressing contract features not visible from the ABI.
/// @dev Interface selector: `0x582de3e7`
interface IERC7996 {
    /// @notice Check if a feature is supported.
    /// @param featureId The feature identifier.
    /// @return `true` if the feature is supported by the contract.
    function supportsFeature(bytes4 featureId) external view returns (bool);
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
pragma solidity >=0.8.4;

interface INameResolver {
    event NameChanged(bytes32 indexed node, string name);

    /// Returns the name associated with an ENS node, for reverse records.
    /// Defined in EIP181.
    /// @param node The ENS node to query.
    /// @return The associated name.
    function name(bytes32 node) external view returns (string memory);
}

//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {HexUtils} from "../utils/HexUtils.sol";
import {NameCoder} from "../utils/NameCoder.sol";

uint32 constant CHAIN_ID_ETH = 1;

uint256 constant COIN_TYPE_ETH = 60;
uint256 constant COIN_TYPE_DEFAULT = 1 << 31; // 0x8000_0000

string constant SLUG_ETH = "addr"; // <=> COIN_TYPE_ETH
string constant SLUG_DEFAULT = "default"; // <=> COIN_TYPE_DEFAULT
string constant TLD_REVERSE = "reverse";

/// @dev Library for generating reverse names according to ENSIP-19.
/// https://docs.ens.domains/ensip/19
library ENSIP19 {
    /// @dev The supplied address was `0x`.
    ///      Error selector: `0x7138356f`
    error EmptyAddress();

    /// @dev Extract Chain ID from `coinType`.
    /// @param coinType The coin type.
    /// @return The Chain ID or 0 if non-EVM Chain.
    function chainFromCoinType(
        uint256 coinType
    ) internal pure returns (uint32) {
        if (coinType == COIN_TYPE_ETH) return CHAIN_ID_ETH;
        coinType ^= COIN_TYPE_DEFAULT;
        return uint32(coinType < COIN_TYPE_DEFAULT ? coinType : 0);
    }

    /// @dev Determine if Coin Type is for an EVM address.
    /// @param coinType The coin type.
    /// @return True if coin type represents an EVM address.
    function isEVMCoinType(uint256 coinType) internal pure returns (bool) {
        return coinType == COIN_TYPE_DEFAULT || chainFromCoinType(coinType) > 0;
    }

    /// @dev Generate Reverse Name from Address + Coin Type.
    ///      Reverts `EmptyAddress` if `addressBytes` is `0x`.
    /// @param addressBytes The input address.
    /// @param coinType The coin type.
    /// @return The ENS reverse name, eg. `1234abcd.addr.reverse`.
    function reverseName(
        bytes memory addressBytes,
        uint256 coinType
    ) internal pure returns (string memory) {
        if (addressBytes.length == 0) {
            revert EmptyAddress();
        }
        return
            string(
                abi.encodePacked(
                    HexUtils.bytesToHex(addressBytes),
                    bytes1("."),
                    coinType == COIN_TYPE_ETH
                        ? SLUG_ETH
                        : coinType == COIN_TYPE_DEFAULT
                            ? SLUG_DEFAULT
                            : HexUtils.unpaddedUintToHex(coinType, true),
                    bytes1("."),
                    TLD_REVERSE
                )
            );
    }

    /// @dev Parse Reverse Name into Address + Coin Type.
    ///      Matches: `/^[0-9a-fA-F]+\.([0-9a-f]{1,64}|addr|default)\.reverse$/`.
    ///      Reverts `DNSDecodingFailed`.
    /// @param name The DNS-encoded name.
    /// @return addressBytes The address or empty if invalid.
    /// @return coinType The coin type.
    function parse(
        bytes memory name
    ) internal pure returns (bytes memory addressBytes, uint256 coinType) {
        (, uint256 offset) = NameCoder.readLabel(name, 0);
        bool valid;
        (addressBytes, valid) = HexUtils.hexToBytes(name, 1, offset);
        if (!valid || addressBytes.length == 0) return ("", 0); // addressBytes not 1+ hex
        (valid, coinType) = parseNamespace(name, offset);
        if (!valid) return ("", 0); // invalid namespace
    }

    /// @dev Parse Reverse Namespace into Coin Type.
    ///      Matches: `/^([0-9a-f]{1,64}|addr|default)\.reverse$/`.
    ///      Reverts `DNSDecodingFailed`.
    /// @param name The DNS-encoded name.
    /// @param offset The offset to begin parsing.
    /// @return valid True if a valid reverse namespace.
    /// @return coinType The coin type.
    function parseNamespace(
        bytes memory name,
        uint256 offset
    ) internal pure returns (bool valid, uint256 coinType) {
        (bytes32 labelHash, uint256 offsetTLD) = NameCoder.readLabel(
            name,
            offset
        );
        if (labelHash == keccak256(bytes(SLUG_ETH))) {
            coinType = COIN_TYPE_ETH;
        } else if (labelHash == keccak256(bytes(SLUG_DEFAULT))) {
            coinType = COIN_TYPE_DEFAULT;
        } else if (labelHash == bytes32(0)) {
            return (false, 0); // no slug
        } else {
            (bytes32 word, bool validHex) = HexUtils.hexStringToBytes32(
                name,
                1 + offset,
                offsetTLD
            );
            if (!validHex) return (false, 0); // invalid coinType or too long
            coinType = uint256(word);
        }
        (labelHash, offset) = NameCoder.readLabel(name, offsetTLD);
        if (labelHash != keccak256(bytes(TLD_REVERSE))) return (false, 0); // invalid tld
        (labelHash, ) = NameCoder.readLabel(name, offset);
        if (labelHash != bytes32(0)) return (false, 0); // not tld
        valid = true;
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
pragma solidity ^0.8.13;

import {LibMem} from "./LibMem/LibMem.sol";
import {BytesUtils} from "./BytesUtils.sol";

/// @dev Library for encoding/decoding names.
///
/// An ENS name is stop-separated labels, eg. "aaa.bb.c".
///
/// A DNS-encoded name is composed of byte length-prefixed labels with a terminator byte.
/// eg. "\x03aaa\x02bb\x01c\x00".
///
/// * maximum label length is 255 bytes.
/// * length = 0 is reserved for the terminator (root).
/// * `dns.length == 2 + ens.length` and the mapping is injective.
///
library NameCoder {
    /// @dev The namehash of "eth".
    bytes32 public constant ETH_NODE =
        0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae;

    /// @dev The label was empty.
    ///      Error selector: `0xbf9a2740`
    error LabelIsEmpty();

    /// @dev The label was more than 255 bytes.
    ///      Error selector: `0xdab6c73c`
    error LabelIsTooLong(string label);

    /// @dev The DNS-encoded name is malformed.
    ///      Error selector: `0xba4adc23`
    error DNSDecodingFailed(bytes dns);

    /// @dev A label of the ENS name has an invalid size.
    ///      Error selector: `0x9a4c3e3b`
    error DNSEncodingFailed(string ens);

    /// @dev The `name` did not end with `suffix`.
    ///
    /// @param name The DNS-encoded name.
    /// @param suffix The DNS-encoded suffix.
    error NoSuffixMatch(bytes name, bytes suffix);

    /// @dev Read the `size` of the label at `offset`.
    ///      If `size = 0`, it must be the end of `name` (no junk at end).
    ///      Reverts `DNSDecodingFailed`.
    ///
    /// @param name The DNS-encoded name.
    /// @param offset The offset into `name` to start reading.
    ///
    /// @return size The size of the label in bytes.
    /// @return nextOffset The offset into `name` of the next label.
    function nextLabel(
        bytes memory name,
        uint256 offset
    ) internal pure returns (uint8 size, uint256 nextOffset) {
        unchecked {
            if (offset >= name.length) {
                revert DNSDecodingFailed(name);
            }
            size = uint8(name[offset]);
            nextOffset = offset + 1 + size;
            if (
                size > 0 ? nextOffset >= name.length : nextOffset != name.length
            ) {
                revert DNSDecodingFailed(name);
            }
        }
    }

    /// @dev Find the offset of the label before `offset` in `name`.
    ///      * `prevOffset(name, 0)` reverts
    ///      * `prevOffset(name, name.length + 1)` reverts
    ///      * `prevOffset(name, name.length) = name.length - 1`
    ///      * `prevOffset(name, name.length - 1) = <tld>`
    ///      Reverts `DNSDecodingFailed`.
    ///
    /// @param name The DNS-encoded name.
    /// @param offset The offset into `name` to start reading backwards.
    ///
    /// @return prevOffset The offset into `name` of the previous label.
    function prevLabel(
        bytes memory name,
        uint256 offset
    ) internal pure returns (uint256 prevOffset) {
        while (true) {
            (, uint256 nextOffset) = nextLabel(name, prevOffset);
            if (nextOffset == offset) break;
            if (nextOffset > offset) {
                revert DNSDecodingFailed(name);
            }
            prevOffset = nextOffset;
        }
    }

    /// @dev Count number of labels in `name`.
    ///      * `countLabels("\x03eth\x00") = 1`
    ///      * `countLabels("\x00") = 0`
    ///      Reverts like `nextLabel()`.
    ///
    /// @param name The DNS-encoded parent name.
    /// @param offset The offset into `name` to start hashing.
    ///
    /// @return count The number of labels.
    function countLabels(
        bytes memory name,
        uint256 offset
    ) internal pure returns (uint256 count) {
        uint8 size;
        while (true) {
            (size, offset) = nextLabel(name, offset);
            if (size == 0) break;
            ++count;
        }
    }

    /// @dev Compute the ENS labelhash of the label at `offset` and the offset for the next label.
    ///      Reverts `DNSDecodingFailed`.
    ///
    /// @param name The DNS-encoded name.
    /// @param offset The offset into `name` to start reading.
    ///
    /// @return labelHash The resulting labelhash.
    /// @return nextOffset The offset into `name` of the next label.
    function readLabel(
        bytes memory name,
        uint256 offset
    ) internal pure returns (bytes32 labelHash, uint256 nextOffset) {
        uint8 size;
        (size, nextOffset) = nextLabel(name, offset);
        if (size > 0) {
            assembly {
                labelHash := keccak256(add(add(name, offset), 33), size)
            }
        }
    }

    /// @dev Read label at offset from a DNS-encoded name and the offset for the next label.
    ///      * `readLabel("\x03abc\x00", 0) = ("abc", 4)`
    ///      * `readLabel("\x00", 0) = ("", 1)`
    ///      Reverts `DNSDecodingFailed`.
    ///
    /// @param name The DNS-encoded name.
    /// @param offset The offset into `name` to start reading.
    ///
    /// @return label The label corresponding to `offset`.
    /// @return nextOffset The offset into `name` of the next label.
    function extractLabel(
        bytes memory name,
        uint256 offset
    ) internal pure returns (string memory label, uint256 nextOffset) {
        uint8 size;
        (size, nextOffset) = nextLabel(name, offset);
        bytes memory v = new bytes(size);
        unchecked {
            LibMem.copy(LibMem.ptr(v), LibMem.ptr(name) + offset + 1, size);
        }
        label = string(v);
    }

    /// @dev Reads first label from a DNS-encoded name.
    ///      Reverts `DNSDecodingFailed`.
    ///      Reverts `LabelIsEmpty` if the label was empty.
    ///
    /// @param name The DNS-encoded name.
    ///
    /// @return The first label.
    function firstLabel(
        bytes memory name
    ) internal pure returns (string memory) {
        (string memory label, ) = extractLabel(name, 0);
        if (bytes(label).length == 0) {
            revert LabelIsEmpty();
        }
        return label;
    }

    /// @dev Compute the namehash of `name[:offset]`.
    ///      Reverts `DNSDecodingFailed`.
    ///
    /// @param name The DNS-encoded name.
    /// @param offset The offset into `name` to start hashing.
    ///
    /// @return hash The namehash of `name[:offset]`.
    function namehash(
        bytes memory name,
        uint256 offset
    ) internal pure returns (bytes32 hash) {
        (hash, offset) = readLabel(name, offset);
        if (hash != bytes32(0)) {
            hash = namehash(namehash(name, offset), hash);
        }
    }

    /// @dev Compute a child namehash from a parent namehash and child labelhash.
    ///
    /// @param parentNode The namehash of the parent.
    /// @param labelHash The labelhash of the child.
    ///
    /// @return node The namehash of the child.
    function namehash(
        bytes32 parentNode,
        bytes32 labelHash
    ) internal pure returns (bytes32 node) {
        // ~100 gas less than: keccak256(abi.encode(parentNode, labelHash))
        assembly {
            mstore(0, parentNode)
            mstore(32, labelHash)
            node := keccak256(0, 64)
        }
    }

    /// @dev Convert DNS-encoded name to ENS name.
    ///      * `decode("\x00") = ""`
    ///      * `decode("\x03eth\x00") = "eth"`
    ///      * `decode("\x03aaa\x02bb\x01c\x00") = "aa.bb.c"`
    ///      * `decode("\x03a.b\x00")` reverts
    ///      Reverts like `nextLabel()`.
    ///
    /// @param dns The DNS-encoded name to convert.
    ///
    /// @return ens The equivalent ENS name.
    function decode(
        bytes memory dns
    ) internal pure returns (string memory ens) {
        unchecked {
            uint256 n = dns.length;
            if (n == 1 && dns[0] == 0) return ""; // only valid answer is root
            if (n < 3) revert DNSDecodingFailed(dns);
            bytes memory v = new bytes(n - 2); // always 2-shorter
            LibMem.copy(LibMem.ptr(v), LibMem.ptr(dns) + 1, n - 2); // shift by -1 byte
            uint256 offset;
            while (true) {
                (uint8 size, uint256 nextOffset) = nextLabel(dns, offset);
                if (size == 0) break;
                if (BytesUtils.includes(v, offset, size, ".")) {
                    revert DNSDecodingFailed(dns); // malicious label
                }
                if (offset > 0) {
                    v[offset - 1] = ".";
                }
                offset = nextOffset;
            }
            return string(v);
        }
    }

    /// @dev Convert ENS name to DNS-encoded name.
    ///      * `encode("aaa.bb.c") = "\x03aaa\x02bb\x01c\x00"`
    ///      * `encode("eth") = "\x03eth\x00"`
    ///      * `encode("") = "\x00"`
    ///      Reverts `DNSEncodingFailed`.
    ///
    /// @param ens The ENS name to convert.
    ///
    /// @return dns The corresponding DNS-encoded name, eg. `\x03aaa\x02bb\x01c\x00`.
    function encode(
        string memory ens
    ) internal pure returns (bytes memory dns) {
        unchecked {
            uint256 n = bytes(ens).length;
            if (n == 0) return hex"00"; // root
            dns = new bytes(n + 2); // always 2-longer
            LibMem.copy(LibMem.ptr(dns) + 1, LibMem.ptr(bytes(ens)), n); // shift by +1 byte
            uint256 start; // remember position to write length
            uint256 size;
            for (uint256 i; i < n; ++i) {
                if (bytes(ens)[i] == ".") {
                    size = i - start;
                    if (size == 0 || size > 255) {
                        revert DNSEncodingFailed(ens);
                    }
                    dns[start] = bytes1(uint8(size));
                    start = i + 1;
                }
            }
            size = n - start;
            if (size == 0 || size > 255) {
                revert DNSEncodingFailed(ens);
            }
            dns[start] = bytes1(uint8(size));
        }
    }

    /// @dev Find the offset into `name` that namehashes to `nodeSuffix`.
    ///
    /// @param name The DNS-encoded name to search.
    /// @param nodeSuffix The namehash to match.
    ///
    /// @return matched True if `name` ends with `nodeSuffix`.
    /// @return node The namehash of `name[offset:]`.
    /// @return prevOffset The offset into `name` of the label before `nodeSuffix`, or `matchOffset` if no match or no prior label.
    /// @return matchOffset The offset into `name` that namehashes to the `nodeSuffix`, or 0 if no match.
    function matchSuffix(
        bytes memory name,
        uint256 offset,
        bytes32 nodeSuffix
    )
        internal
        pure
        returns (
            bool matched,
            bytes32 node,
            uint256 prevOffset,
            uint256 matchOffset
        )
    {
        (bytes32 labelHash, uint256 next) = readLabel(name, offset);
        if (labelHash != bytes32(0)) {
            (matched, node, prevOffset, matchOffset) = matchSuffix(
                name,
                next,
                nodeSuffix
            );
            if (node == nodeSuffix) {
                matched = true;
                prevOffset = offset;
                matchOffset = next;
            }
            node = namehash(node, labelHash);
        }
        if (node == nodeSuffix) {
            matched = true;
            prevOffset = matchOffset = offset;
        }
    }

    /// @dev Assert `label` is an encodable size.
    ///
    /// @param label The label to check.
    ///
    /// @return The size of the label.
    function assertLabelSize(
        string memory label
    ) internal pure returns (uint8) {
        uint256 n = bytes(label).length;
        if (n == 0) revert LabelIsEmpty();
        if (n > 255) revert LabelIsTooLong(label);
        return uint8(n);
    }

    /// @dev Prepend `label` to DNS-encoded `name`.
    ///      * `addLabel("\x03eth\x00", "test") = "\x04test\x03eth\x00"`
    ///      * `addLabel("\x00", "eth") = "\x03eth\x00"`
    ///      * `addLabel("", "abc") = "\x03abc"` invalid
    ///      * `addLabel("", "")` reverts
    ///      Assumes `name` is properly encoded.
    ///      Reverts like `assertLabelSize()`.
    ///
    /// @param name The DNS-encoded parent name.
    /// @param label The child label to prepend.
    ///
    /// @return The DNS-encoded child name.
    function addLabel(
        bytes memory name,
        string memory label
    ) internal pure returns (bytes memory) {
        return abi.encodePacked(assertLabelSize(label), label, name);
    }

    /// @dev Transform `label` to DNS-encoded `{label}.eth`.
    ///      * `ethName("eth") = "\x04test\x03eth\x00"`
    ///      Behaves like `addLabel()`.
    ///
    /// @param label The label to encode.
    ///
    /// @return The DNS-encoded name.
    function ethName(string memory label) internal pure returns (bytes memory) {
        return addLabel("\x03eth\x00", label);
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


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from './AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from './GatewayVM.sol';
import {RLPReader, RLPReaderExt} from './RLPReaderExt.sol';

contract SelfVerifier is AbstractVerifier {
    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks
    ) AbstractVerifier(urls, window, hooks) {}

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(block.number - 1);
    }

    struct GatewayProof {
        bytes rlpEncodedBlock;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 blockNumber1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        RLPReader.RLPItem[] memory v = RLPReader.readList(p.rlpEncodedBlock);
        uint256 blockNumber = uint256(RLPReaderExt.bytes32FromRLP(v[8]));
        _checkWindow(blockNumber1, blockNumber);
        // TODO: change this to https://eips.ethereum.org/EIPS/eip-2935
        bytes32 blockHash = blockhash(blockNumber);
        require(blockHash == keccak256(p.rlpEncodedBlock), 'Self: blockhash');
        bytes32 stateRoot = RLPReaderExt.strictBytes32FromRLP(v[3]);
        return verify(req, stateRoot, p.proofs, p.order);
    }

    function verify(
        GatewayRequest memory req,
        bytes32 stateRoot,
        bytes[] memory proofs,
        bytes memory order
    ) public view returns (bytes[] memory outputs, uint8 exitCode) {
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, proofs, order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import {Ownable} from "@openzeppelin/contracts-v5/access/Ownable.sol";

import {L2ReverseRegistrar} from "./L2ReverseRegistrar.sol";
import {INameResolver} from "../resolvers/profiles/INameResolver.sol";
import {AddressUtils} from "../utils/AddressUtils.sol";

/// @notice An L2 Reverse Registrar that allows migrating from a prior resolver.
contract L2ReverseRegistrarWithMigration is L2ReverseRegistrar, Ownable {
    using AddressUtils for address;

    /// @notice The old reverse resolver to migrate from
    INameResolver immutable oldReverseResolver;

    /// @notice The parent node of reverse nodes. The convention is '${coinType}.reverse'
    bytes32 immutable parentNode;

    /// @notice Initialises the contract by setting the parent node, coin type, and old reverse resolver.
    ///
    /// @param coinType_ The cointype converted from the chainId of the chain this contract is deployed to.
    /// @param owner_ The initial owner of the contract.
    /// @param parentNode_ The parent node to set. The convention is '${coinType}.reverse'.
    /// @param oldReverseResolver_ The old reverse resolver.
    constructor(
        uint256 coinType_,
        address owner_,
        bytes32 parentNode_,
        INameResolver oldReverseResolver_
    ) L2ReverseRegistrar(coinType_) Ownable(owner_) {
        parentNode = parentNode_;
        oldReverseResolver = oldReverseResolver_;
    }

    /// @notice Migrates the names from the old reverse resolver to the new one.
    ///         Only callable by the owner.
    ///
    /// @param addresses The addresses to migrate.
    function batchSetName(address[] calldata addresses) external onlyOwner {
        for (uint256 i = 0; i < addresses.length; i++) {
            // namehash of `[addresses[i]].[coinType].reverse`
            bytes32 node = keccak256(
                abi.encodePacked(parentNode, addresses[i].sha3HexAddress())
            );
            string memory name = oldReverseResolver.name(node);

            // equivalent to _setName(addresses[i], name);
            // internal because the name value isn't in calldata
            _names[addresses[i]] = name;
            emit NameForAddrChanged(addresses[i], name);
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Ownable} from '@openzeppelin/contracts/access/Ownable.sol';
import {ERC165} from '@openzeppelin/contracts/utils/introspection/ERC165.sol';

import {IStandardGatewayVerifier, IGatewayVerifier} from './IStandardGatewayVerifier.sol';
import {IVerifierHooks} from './IVerifierHooks.sol';

abstract contract AbstractVerifier is IStandardGatewayVerifier, Ownable, ERC165 {
    event GatewayURLsChanged();

    string[] _urls;
    uint256 immutable _window;
    IVerifierHooks immutable _hooks;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks
    ) Ownable(msg.sender) {
        _urls = urls;
        _window = window;
        _hooks = hooks;
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override returns (bool) {
        return
            interfaceId == type(IGatewayVerifier).interfaceId ||
            interfaceId == type(IStandardGatewayVerifier).interfaceId ||
            super.supportsInterface(interfaceId);
    }

    function setGatewayURLs(string[] memory urls) external onlyOwner {
        _urls = urls;
        emit GatewayURLsChanged();
    }

    /// @inheritdoc IGatewayVerifier
    function gatewayURLs() external view returns (string[] memory) {
        return _urls;
    }

    /// @inheritdoc IStandardGatewayVerifier
    function getWindow() external view returns (uint256) {
        return _window;
    }

    /// @inheritdoc IStandardGatewayVerifier
    function getHooks() external view returns (IVerifierHooks) {
        return _hooks;
    }

    function _checkWindow(uint256 latest, uint256 got) internal view {
        if (got + _window < latest) revert CommitTooOld(latest, got, _window);
        if (got > latest) revert CommitTooNew(latest, got);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {NitroVerifierLib} from './NitroVerifierLib.sol';
import {BoLDVerifierLib} from './BoLDVerifierLib.sol';

contract ArbitrumVerifier is AbstractVerifier {
    address public immutable rollup;
    uint256 public immutable minAgeBlocks;
    bool public immutable isBoLD;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        address _rollup,
        uint256 _minAgeBlocks,
        bool _isBoLD
    ) AbstractVerifier(urls, window, hooks) {
        rollup = _rollup;
        minAgeBlocks = _minAgeBlocks;
        isBoLD = _isBoLD;
    }

    function getLatestContext() external view returns (bytes memory) {
        return
            abi.encode(
                isBoLD
                    ? BoLDVerifierLib.latestIndex(rollup, minAgeBlocks)
                    : NitroVerifierLib.latestIndex(rollup, minAgeBlocks)
            );
    }

    struct GatewayProof {
        bytes rollupProof;
        bytes[] proofs;
        bytes order;
    }

    function _verifyRollup(
        GatewayProof memory p,
        bytes memory context
    ) internal view returns (bytes32 stateRoot) {
        uint256 latest = abi.decode(context, (uint256));
        uint256 got;
        if (isBoLD) {
            (stateRoot, got) = BoLDVerifierLib.verifyRollup(
                rollup,
                minAgeBlocks,
                p.rollupProof
            );
        } else {
            (stateRoot, latest, got) = NitroVerifierLib.verifyRollup(
                rollup,
                minAgeBlocks,
                p.rollupProof,
                uint64(latest)
            );
        }
        _checkWindow(latest, got);
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view virtual returns (bytes[] memory, uint8 exitCode) {
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        bytes32 stateRoot = _verifyRollup(p, context);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {LazyTrustedVerifier, IVerifierHooks} from './LazyTrustedVerifier.sol';

contract TrustedVerifier is LazyTrustedVerifier {
    constructor(
        IVerifierHooks hooks,
        string[] memory urls,
        address[] memory signers,
        uint256 expSec
    ) {
        initialize(msg.sender, hooks, urls, signers, expSec);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';

interface IZKSyncDiamond {
    function storedBatchHash(
        uint256 batchNumber
    ) external view returns (bytes32);
    function l2LogsRootHash(
        uint256 batchNumber
    ) external view returns (bytes32);
    function getTotalBatchesExecuted() external view returns (uint256);
}

struct StoredBatchInfo {
    uint64 batchNumber;
    bytes32 batchHash;
    uint64 indexRepeatedStorageChanges;
    uint256 numberOfLayer1Txs;
    bytes32 priorityOperationsHash;
    bytes32 l2LogsTreeRoot;
    uint256 timestamp;
    bytes32 commitment;
}

contract ZKSyncVerifier is AbstractVerifier {
    IZKSyncDiamond immutable _diamond;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IZKSyncDiamond diamond
    ) AbstractVerifier(urls, window, hooks) {
        _diamond = diamond;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_diamond.getTotalBatchesExecuted() - 1);
    }

    struct GatewayProof {
        bytes encodedBatch;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 batchIndex1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        StoredBatchInfo memory batchInfo = abi.decode(
            p.encodedBatch,
            (StoredBatchInfo)
        );
        _checkWindow(batchIndex1, batchInfo.batchNumber);
        require(
            keccak256(p.encodedBatch) ==
                _diamond.storedBatchHash(batchInfo.batchNumber),
            'ZKS: batchHash'
        );
        require(
            batchInfo.l2LogsTreeRoot ==
                _diamond.l2LogsRootHash(batchInfo.batchNumber),
            'ZKS: l2LogsRootHash'
        );
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, batchInfo.batchHash, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from './AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from './GatewayVM.sol';

contract InteractiveVerifier is AbstractVerifier {
    event NewStateRoot(
        uint256 indexed prevIndex,
        uint256 indexed index,
        bytes32 stateRoot
    );

    struct Commit {
        bytes32 stateRoot;
        uint256 prevIndex;
    }

    mapping(uint256 => Commit) public commits;
    uint256 public latestIndex;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks
    ) AbstractVerifier(urls, window, hooks) {}

    function setStateRoot(uint256 index, bytes32 stateRoot) external onlyOwner {
        require(index > latestIndex, 'out of order');
        commits[index] = Commit(stateRoot, latestIndex);
        emit NewStateRoot(latestIndex, index, stateRoot);
        latestIndex = index;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(latestIndex);
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory values, uint8 exitCode) {
        uint256 index1 = abi.decode(context, (uint256));
        (uint256 index, bytes[] memory proofs, bytes memory order) = abi.decode(
            proof,
            (uint256, bytes[], bytes)
        );
        _checkWindow(index1, index);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(
                    0,
                    commits[index].stateRoot,
                    proofs,
                    order,
                    _hooks
                )
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {ERC165} from "@openzeppelin/contracts-v5/utils/introspection/ERC165.sol";
import {Ownable} from "@openzeppelin/contracts-v5/access/Ownable.sol";

import {GatewayFetchTarget, IGatewayVerifier} from "@unruggable/gateways/GatewayFetchTarget.sol";
import {GatewayFetcher, GatewayRequest, RequestOverflow} from "@unruggable/gateways/GatewayFetcher.sol";

import {AbstractReverseResolver} from "./AbstractReverseResolver.sol";
import {IStandaloneReverseRegistrar} from "../reverseRegistrar/IStandaloneReverseRegistrar.sol";
import {IVerifiableResolver} from "../resolvers/profiles/IVerifiableResolver.sol";
import {INameReverser} from "./INameReverser.sol";
import {ENSIP19} from "../utils/ENSIP19.sol";

/// @title Chain Reverse Resolver
/// @notice Reverses an EVM address using the first non-null response from the following sources:
///
/// 1. `L2ReverseRegistrar` on L2 chain via Unruggable Gateway
/// 2. `IStandaloneReverseRegistrar` for "default.reverse"
///
contract ChainReverseResolver is
    AbstractReverseResolver,
	IVerifiableResolver,
    GatewayFetchTarget,
    Ownable
{
    using GatewayFetcher for GatewayRequest;

    /// @notice Storage slot for the names mapping in `L2ReverseRegistrar`.
    uint256 constant NAMES_SLOT = 0;

    /// @notice The reverse registrar contract for "default.reverse".
    IStandaloneReverseRegistrar public immutable defaultRegistrar;

    /// @notice The verifier contract for the L2 chain.
    IGatewayVerifier public gatewayVerifier;

    /// @notice Gateway URLs for the verifier contract.
    string[] public gatewayURLs;

    /// @notice Emitted when the gateway verifier is changed.
    event GatewayVerifierChanged(address verifier);

    /// @notice Emitted when the gateway URLs are changed.
    event GatewayURLsChanged(string[] urls);

    constructor(
        address _owner,
        uint256 coinType,
        IStandaloneReverseRegistrar _defaultRegistrar,
        address _chainRegistrar,
        IGatewayVerifier verifier,
        string[] memory gateways
    ) Ownable(_owner) AbstractReverseResolver(coinType, _chainRegistrar) {
        defaultRegistrar = _defaultRegistrar;
        gatewayVerifier = verifier;
        gatewayURLs = gateways;
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceId
    ) public view override returns (bool) {
        return
            interfaceId == type(IVerifiableResolver).interfaceId ||
            super.supportsInterface(interfaceId);
    }

	/// @inheritdoc IVerifiableResolver
    function verifierMetadata(
        bytes memory name
    ) external view returns (address verifier, string[] memory gateways) {
		 (bytes memory a, uint256 ct) = ENSIP19.parse(name);
		 if (a.length == 20 && ct == coinType) {
			return (address(gatewayVerifier), gatewayURLs);
		 }
	}

    /// @notice Set gateway URLs.
    /// @param gateways The new gateway URLs.
    function setGatewayURLs(string[] memory gateways) external onlyOwner {
        gatewayURLs = gateways;
        emit GatewayURLsChanged(gateways);
    }

    /// @notice Set the verifier contract.
    /// @param verifier The new verifier contract.
    function setGatewayVerifier(address verifier) external onlyOwner {
        gatewayVerifier = IGatewayVerifier(verifier);
        emit GatewayVerifierChanged(verifier);
    }

    /// @inheritdoc AbstractReverseResolver
    function _resolveName(
        address addr
    ) internal view override returns (string memory) {
        GatewayRequest memory req = GatewayFetcher.newRequest(1);
        req.setTarget(chainRegistrar);
        req.setSlot(NAMES_SLOT).push(addr).follow().readBytes(); // names[addr]
        req.setOutput(0);
        fetch(
            gatewayVerifier,
            req,
            this.resolveNameCallback.selector, // ==> step 2
            abi.encode(addr),
            gatewayURLs
        );
    }

    /// @dev CCIP-Read callback for `_resolveName()`.
    /// @param values The outputs for `GatewayRequest` (1 name).
    /// @param extraData The contextual data passed from `_resolveName()`.
    /// @return result The abi-encoded name for the given address.
    function resolveNameCallback(
        bytes[] memory values,
        uint8 /* exitCode */,
        bytes calldata extraData
    ) external view returns (bytes memory result) {
        string memory name = string(values[0]);
        if (bytes(name).length == 0) {
            address addr = abi.decode(extraData, (address));
            name = defaultRegistrar.nameForAddr(addr);
        }
        result = abi.encode(name);
    }

    /// @inheritdoc INameReverser
    /// @dev Reverts with a variety of errors.
    /// - reverts `RequestOverflow` if too many addresses.
    /// - Gateway request may fail if too many proofs.
    /// - Gateway response may run out of gas.
    function resolveNames(
        address[] memory addrs
    ) external view returns (string[] memory) {
        if (addrs.length > 255) {
            revert RequestOverflow();
        }
        GatewayRequest memory req = GatewayFetcher.newRequest(
            uint8(addrs.length)
        );
        req.setTarget(chainRegistrar); // target L2 registrar
        for (uint256 i; i < addrs.length; ++i) {
            req.setSlot(NAMES_SLOT).push(addrs[i]).follow().readBytes(); // names[addr[i]]
            req.setOutput(uint8(i));
        }
        fetch(
            gatewayVerifier,
            req,
            this.resolveNamesCallback.selector, // ==> step 2
            abi.encode(addrs),
            gatewayURLs
        );
    }

    /// @dev CCIP-Read callback for `_resolveNames()`.
    ///      Recursive if there are still addresses to resolve.
    /// @param values The outputs for `GatewayRequest` (N names).
    /// @param extraData The contextual data passed from `_resolveNames()`.
    /// @return names The resolved names.
    function resolveNamesCallback(
        bytes[] memory values,
        uint8 /* exitCode */,
        bytes calldata extraData
    ) external view returns (string[] memory names) {
        address[] memory addrs = abi.decode(extraData, (address[]));
        names = new string[](addrs.length);
        for (uint256 i; i < addrs.length; ++i) {
            string memory name = string(values[i]);
            if (bytes(name).length == 0) {
                name = defaultRegistrar.nameForAddr(addrs[i]);
            }
            names[i] = name;
        }
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import {ERC165} from "@openzeppelin/contracts-v5/utils/introspection/ERC165.sol";

import {IStandaloneReverseRegistrar} from "./IStandaloneReverseRegistrar.sol";

/// @title Standalone Reverse Registrar
/// @notice A standalone reverse registrar, detached from the ENS registry.
contract StandaloneReverseRegistrar is ERC165, IStandaloneReverseRegistrar {
    /// @notice The mapping of addresses to names.
    mapping(address => string) internal _names;

    /// @inheritdoc IStandaloneReverseRegistrar
    function nameForAddr(
        address addr
    ) external view returns (string memory name) {
        name = _names[addr];
    }

    /// @notice Sets the name for an address.
    ///
    /// @dev Authorisation should be checked before calling.
    ///
    /// @param addr The address to set the name for.
    /// @param name The name to set.
    function _setName(address addr, string calldata name) internal {
        _names[addr] = name;
        emit NameForAddrChanged(addr, name);
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceID
    ) public view virtual override(ERC165) returns (bool) {
        return
            interfaceID == type(IStandaloneReverseRegistrar).interfaceId ||
            super.supportsInterface(interfaceID);
    }
}

//SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;
import "@openzeppelin/contracts/access/Ownable.sol";
import "./profiles/ABIResolver.sol";
import "./profiles/AddrResolver.sol";
import "./profiles/ContentHashResolver.sol";
import "./profiles/DNSResolver.sol";
import "./profiles/InterfaceResolver.sol";
import "./profiles/NameResolver.sol";
import "./profiles/PubkeyResolver.sol";
import "./profiles/TextResolver.sol";
import "./profiles/ExtendedResolver.sol";

/// A simple resolver anyone can use; only allows the owner of a node to set its
/// address.
contract OwnedResolver is
    Ownable,
    ABIResolver,
    AddrResolver,
    ContentHashResolver,
    DNSResolver,
    InterfaceResolver,
    NameResolver,
    PubkeyResolver,
    TextResolver,
    ExtendedResolver
{
    function isAuthorised(bytes32) internal view override returns (bool) {
        return msg.sender == owner();
    }

    function supportsInterface(
        bytes4 interfaceID
    )
        public
        view
        virtual
        override(
            ABIResolver,
            AddrResolver,
            ContentHashResolver,
            DNSResolver,
            InterfaceResolver,
            NameResolver,
            PubkeyResolver,
            TextResolver
        )
        returns (bool)
    {
        return super.supportsInterface(interfaceID);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {ILineaRollup} from './ILineaRollup.sol';

//import 'forge-std/console.sol';

contract UnfinalizedLineaVerifier is AbstractVerifier {
    ILineaRollup immutable _rollup;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        ILineaRollup rollup
    ) AbstractVerifier(urls, window, hooks) {
        _rollup = rollup;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(block.number - 1);
    }

    struct GatewayProof {
        uint256 l1BlockNumber;
        bytes abiEncodedTuple;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory /*context*/,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        //uint256 l1BlockNumber1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        // TODO: must prove a time constraint on the shnarf
        // ideas:
        // 1.) prove l1BlockNumber contains the transaction that that commit this shnarf
        // 2.) prove some L1 state on L2 using l2BlockNumber and stateRoot
        // 3.) use some heuristic based on L2.lastAnchoredL1MessageNumber and L1.nextMessageNumber?
        //_checkWindow(p.l1BlockNumber, l1BlockNumber1);
        bytes32 stateRoot = _extractStateRoot(p.abiEncodedTuple);
        uint256 l2BlockNumber = _rollup.shnarfFinalBlockNumbers(
            keccak256(p.abiEncodedTuple)
        );
        // this is the only guard available
        // the shnarf must be newer than the finalization
        if (l2BlockNumber < _rollup.currentL2BlockNumber()) {
            // TODO: remove this once we have a time constraint
            // if it's older than the finalization, it must match
            require(
                stateRoot == _rollup.stateRootHashes(l2BlockNumber),
                'UnfinalizedLinea: not finalized'
            );
        }
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }

    function _extractStateRoot(
        bytes memory v
    ) internal pure returns (bytes32 stateRoot) {
        assembly {
            stateRoot := mload(add(v, 96)) // see: ShnarfData
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {ERC165, IERC165} from "@openzeppelin/contracts/utils/introspection/ERC165.sol";

import "../../contracts/resolvers/profiles/IAddrResolver.sol";
import "../../contracts/resolvers/profiles/IExtendedResolver.sol";
import "../../contracts/resolvers/profiles/IExtendedDNSResolver.sol";
import "../dnssec-oracle/DNSSEC.sol";
import "../dnssec-oracle/RRUtils.sol";
import "../registry/ENSRegistry.sol";
import "../utils/HexUtils.sol";
import "../utils/BytesUtils.sol";
import {IDNSGateway} from "../dnssec-oracle/IDNSGateway.sol";
import {OffchainLookup} from "../ccipRead/EIP3668.sol";
import {LowLevelCallUtils} from "../utils/LowLevelCallUtils.sol";

error InvalidOperation();

uint16 constant CLASS_INET = 1;
uint16 constant TYPE_TXT = 16;

contract OffchainDNSResolver is IExtendedResolver, IERC165 {
    using RRUtils for *;
    using Address for address;
    using BytesUtils for bytes;
    using HexUtils for bytes;

    ENS public immutable ens;
    DNSSEC public immutable oracle;
    string public gatewayURL;

    error CouldNotResolve(bytes name);

    constructor(ENS _ens, DNSSEC _oracle, string memory _gatewayURL) {
        ens = _ens;
        oracle = _oracle;
        gatewayURL = _gatewayURL;
    }

    function supportsInterface(
        bytes4 interfaceId
    ) external pure override returns (bool) {
        return interfaceId == type(IExtendedResolver).interfaceId;
    }

    function resolve(
        bytes calldata name,
        bytes calldata data
    ) external view returns (bytes memory) {
        revertWithDefaultOffchainLookup(name, data);
    }

    function resolveCallback(
        bytes calldata response,
        bytes calldata extraData
    ) external view returns (bytes memory) {
        (bytes memory name, bytes memory query, bytes4 selector) = abi.decode(
            extraData,
            (bytes, bytes, bytes4)
        );

        if (selector != bytes4(0)) {
            (bytes memory targetData, address targetResolver) = abi.decode(
                query,
                (bytes, address)
            );
            return
                callWithOffchainLookupPropagation(
                    targetResolver,
                    name,
                    query,
                    abi.encodeWithSelector(
                        selector,
                        response,
                        abi.encode(targetData, address(this))
                    )
                );
        }

        DNSSEC.RRSetWithSignature[] memory rrsets = abi.decode(
            response,
            (DNSSEC.RRSetWithSignature[])
        );

        (bytes memory data, ) = oracle.verifyRRSet(rrsets);
        for (
            RRUtils.RRIterator memory iter = data.iterateRRs(0);
            !iter.done();
            iter.next()
        ) {
            // Ignore records with wrong name, type, or class
            bytes memory rrname = RRUtils.readName(iter.data, iter.offset);
            if (
                !rrname.equals(name) ||
                iter.class != CLASS_INET ||
                iter.dnstype != TYPE_TXT
            ) {
                continue;
            }

            // Look for a valid ENS-DNS TXT record
            (address dnsresolver, bytes memory context) = parseRR(
                iter.data,
                iter.rdataOffset,
                iter.nextOffset
            );

            // If we found a valid record, try to resolve it
            if (dnsresolver != address(0)) {
                if (
                    IERC165(dnsresolver).supportsInterface(
                        IExtendedDNSResolver.resolve.selector
                    )
                ) {
                    return
                        callWithOffchainLookupPropagation(
                            dnsresolver,
                            name,
                            query,
                            abi.encodeCall(
                                IExtendedDNSResolver.resolve,
                                (name, query, context)
                            )
                        );
                } else if (
                    IERC165(dnsresolver).supportsInterface(
                        IExtendedResolver.resolve.selector
                    )
                ) {
                    return
                        callWithOffchainLookupPropagation(
                            dnsresolver,
                            name,
                            query,
                            abi.encodeCall(
                                IExtendedResolver.resolve,
                                (name, query)
                            )
                        );
                } else {
                    (bool ok, bytes memory ret) = address(dnsresolver)
                        .staticcall(query);
                    if (ok) {
                        return ret;
                    } else {
                        revert CouldNotResolve(name);
                    }
                }
            }
        }

        // No valid records; revert.
        revert CouldNotResolve(name);
    }

    function parseRR(
        bytes memory data,
        uint256 idx,
        uint256 lastIdx
    ) internal view returns (address, bytes memory) {
        bytes memory txt = readTXT(data, idx, lastIdx);

        // Must start with the magic word
        if (txt.length < 5 || !txt.equals(0, "ENS1 ", 0, 5)) {
            return (address(0), "");
        }

        // Parse the name or address
        uint256 lastTxtIdx = txt.find(5, txt.length - 5, " ");
        if (lastTxtIdx > txt.length) {
            address dnsResolver = parseAndResolve(txt, 5, txt.length);
            return (dnsResolver, "");
        } else {
            address dnsResolver = parseAndResolve(txt, 5, lastTxtIdx);
            return (
                dnsResolver,
                txt.substring(lastTxtIdx + 1, txt.length - lastTxtIdx - 1)
            );
        }
    }

    function readTXT(
        bytes memory data,
        uint256 startIdx,
        uint256 lastIdx
    ) internal pure returns (bytes memory) {
        // TODO: Concatenate multiple text fields
        uint256 fieldLength = data.readUint8(startIdx);
        assert(startIdx + fieldLength < lastIdx);
        return data.substring(startIdx + 1, fieldLength);
    }

    function parseAndResolve(
        bytes memory nameOrAddress,
        uint256 idx,
        uint256 lastIdx
    ) internal view returns (address) {
        if (nameOrAddress[idx] == "0" && nameOrAddress[idx + 1] == "x") {
            (address ret, bool valid) = nameOrAddress.hexToAddress(
                idx + 2,
                lastIdx
            );
            if (valid) {
                return ret;
            }
        }
        return resolveName(nameOrAddress, idx, lastIdx);
    }

    function resolveName(
        bytes memory name,
        uint256 idx,
        uint256 lastIdx
    ) internal view returns (address) {
        bytes32 node = textNamehash(name, idx, lastIdx);
        address resolver = ens.resolver(node);
        if (resolver == address(0)) {
            return address(0);
        }
        return IAddrResolver(resolver).addr(node);
    }

    /// @dev Namehash function that operates on dot-separated names (not dns-encoded names)
    /// @param name Name to hash
    /// @param idx Index to start at
    /// @param lastIdx Index to end at
    function textNamehash(
        bytes memory name,
        uint256 idx,
        uint256 lastIdx
    ) internal view returns (bytes32) {
        uint256 separator = name.find(idx, name.length - idx, bytes1("."));
        bytes32 parentNode = bytes32(0);
        if (separator < lastIdx) {
            parentNode = textNamehash(name, separator + 1, lastIdx);
        } else {
            separator = lastIdx;
        }
        return
            keccak256(
                abi.encodePacked(parentNode, name.keccak(idx, separator - idx))
            );
    }

    function callWithOffchainLookupPropagation(
        address target,
        bytes memory name,
        bytes memory innerdata,
        bytes memory data
    ) internal view returns (bytes memory) {
        if (!target.isContract()) {
            revertWithDefaultOffchainLookup(name, innerdata);
        }

        bool result = LowLevelCallUtils.functionStaticCall(
            address(target),
            data
        );
        uint256 size = LowLevelCallUtils.returnDataSize();
        if (result) {
            bytes memory returnData = LowLevelCallUtils.readReturnData(0, size);
            return abi.decode(returnData, (bytes));
        }
        // Failure
        if (size >= 4) {
            bytes memory errorId = LowLevelCallUtils.readReturnData(0, 4);
            if (bytes4(errorId) == OffchainLookup.selector) {
                // Offchain lookup. Decode the revert message and create our own that nests it.
                bytes memory revertData = LowLevelCallUtils.readReturnData(
                    4,
                    size - 4
                );
                handleOffchainLookupError(revertData, target, name);
            }
        }
        LowLevelCallUtils.propagateRevert();
    }

    function revertWithDefaultOffchainLookup(
        bytes memory name,
        bytes memory data
    ) internal view {
        string[] memory urls = new string[](1);
        urls[0] = gatewayURL;

        revert OffchainLookup(
            address(this),
            urls,
            abi.encodeCall(IDNSGateway.resolve, (name, TYPE_TXT)),
            OffchainDNSResolver.resolveCallback.selector,
            abi.encode(name, data, bytes4(0))
        );
    }

    function handleOffchainLookupError(
        bytes memory returnData,
        address target,
        bytes memory name
    ) internal view {
        (
            address sender,
            string[] memory urls,
            bytes memory callData,
            bytes4 innerCallbackFunction,
            bytes memory extraData
        ) = abi.decode(returnData, (address, string[], bytes, bytes4, bytes));

        if (sender != target) {
            revert InvalidOperation();
        }

        revert OffchainLookup(
            address(this),
            urls,
            callData,
            OffchainDNSResolver.resolveCallback.selector,
            abi.encode(name, extraData, innerCallbackFunction)
        );
    }
}

//SPDX-License-Identifier: MIT
pragma solidity >=0.8.17 <0.9.0;

import "../registry/ENS.sol";
import "./profiles/ABIResolver.sol";
import "./profiles/AddrResolver.sol";
import "./profiles/ContentHashResolver.sol";
import "./profiles/DataResolver.sol";
import "./profiles/DNSResolver.sol";
import "./profiles/InterfaceResolver.sol";
import "./profiles/NameResolver.sol";
import "./profiles/PubkeyResolver.sol";
import "./profiles/TextResolver.sol";
import "./Multicallable.sol";
import {ReverseClaimer} from "../reverseRegistrar/ReverseClaimer.sol";
import {INameWrapper} from "../wrapper/INameWrapper.sol";

/// A simple resolver anyone can use; only allows the owner of a node to set its
/// address.
contract PublicResolver is
    Multicallable,
    ABIResolver,
    AddrResolver,
    ContentHashResolver,
    DataResolver,
    DNSResolver,
    InterfaceResolver,
    NameResolver,
    PubkeyResolver,
    TextResolver,
    ReverseClaimer
{
    ENS immutable ens;
    INameWrapper immutable nameWrapper;
    address immutable trustedETHController;
    address immutable trustedReverseRegistrar;

    /// A mapping of operators. An address that is authorised for an address
    /// may make any changes to the name that the owner could, but may not update
    /// the set of authorisations.
    /// (owner, operator) => approved
    mapping(address => mapping(address => bool)) private _operatorApprovals;

    /// A mapping of delegates. A delegate that is authorised by an owner
    /// for a name may make changes to the name's resolver, but may not update
    /// the set of token approvals.
    /// (owner, name, delegate) => approved
    mapping(address => mapping(bytes32 => mapping(address => bool)))
        private _tokenApprovals;

    // Logged when an operator is added or removed.
    event ApprovalForAll(
        address indexed owner,
        address indexed operator,
        bool approved
    );

    // Logged when a delegate is approved or  an approval is revoked.
    event Approved(
        address owner,
        bytes32 indexed node,
        address indexed delegate,
        bool indexed approved
    );

    constructor(
        ENS _ens,
        INameWrapper wrapperAddress,
        address _trustedETHController,
        address _trustedReverseRegistrar
    ) ReverseClaimer(_ens, msg.sender) {
        ens = _ens;
        nameWrapper = wrapperAddress;
        trustedETHController = _trustedETHController;
        trustedReverseRegistrar = _trustedReverseRegistrar;
    }

    /// @dev See {IERC1155-setApprovalForAll}.
    function setApprovalForAll(address operator, bool approved) external {
        require(
            msg.sender != operator,
            "ERC1155: setting approval status for self"
        );

        _operatorApprovals[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }

    /// @dev See {IERC1155-isApprovedForAll}.
    function isApprovedForAll(
        address account,
        address operator
    ) public view returns (bool) {
        return _operatorApprovals[account][operator];
    }

    /// @dev Approve a delegate to be able to updated records on a node.
    function approve(bytes32 node, address delegate, bool approved) external {
        require(msg.sender != delegate, "Setting delegate status for self");

        _tokenApprovals[msg.sender][node][delegate] = approved;
        emit Approved(msg.sender, node, delegate, approved);
    }

    /// @dev Check to see if the delegate has been approved by the owner for the node.
    function isApprovedFor(
        address owner,
        bytes32 node,
        address delegate
    ) public view returns (bool) {
        return _tokenApprovals[owner][node][delegate];
    }

    function isAuthorised(bytes32 node) internal view override returns (bool) {
        if (
            msg.sender == trustedETHController ||
            msg.sender == trustedReverseRegistrar
        ) {
            return true;
        }
        address owner = ens.owner(node);
        if (owner == address(nameWrapper)) {
            owner = nameWrapper.ownerOf(uint256(node));
        }
        return
            owner == msg.sender ||
            isApprovedForAll(owner, msg.sender) ||
            isApprovedFor(owner, node, msg.sender);
    }

    function supportsInterface(
        bytes4 interfaceID
    )
        public
        view
        override(
            Multicallable,
            ABIResolver,
            AddrResolver,
            ContentHashResolver,
            DataResolver,
            DNSResolver,
            InterfaceResolver,
            NameResolver,
            PubkeyResolver,
            TextResolver
        )
        returns (bool)
    {
        return super.supportsInterface(interfaceID);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {RLPReader, RLPReaderExt} from '../RLPReaderExt.sol';

// https://github.com/ethereum-optimism/optimism/blob/develop/packages/contracts-bedrock/src/L2/L1Block.sol
interface IL1Block {
    function number() external view returns (uint256);
}

contract ReverseOPVerifier is AbstractVerifier {
    uint256 immutable SLOT_HASH = 2;
    IL1Block immutable _l1Block;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IL1Block l1Block
    ) AbstractVerifier(urls, window, hooks) {
        _l1Block = l1Block;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_l1Block.number());
    }

    struct GatewayProof {
        bytes rlpEncodedL1Block;
        bytes rlpEncodedL2Block;
        bytes accountProof;
        bytes storageProof;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 blockNumber1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        RLPReader.RLPItem[] memory v = RLPReader.readList(p.rlpEncodedL2Block);
        bytes32 blockHash = blockhash(_extractBlockNumber(v));
        require(
            blockHash == keccak256(p.rlpEncodedL2Block),
            'ReverseOP: hash2'
        );
        bytes32 stateRoot = RLPReaderExt.strictBytes32FromRLP(v[3]);
        bytes32 storageRoot = _hooks.verifyAccountState(
            stateRoot,
            address(_l1Block),
            p.accountProof
        );
        blockHash = _hooks.verifyStorageValue(
            storageRoot,
            address(_l1Block),
            SLOT_HASH,
            p.storageProof
        );
        require(
            blockHash == keccak256(p.rlpEncodedL1Block),
            'ReverseOP: hash1'
        );
        v = RLPReader.readList(p.rlpEncodedL1Block);
        _checkWindow(blockNumber1, _extractBlockNumber(v));
        stateRoot = RLPReaderExt.strictBytes32FromRLP(v[3]);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }

    function _extractBlockNumber(
        RLPReader.RLPItem[] memory v
    ) internal pure returns (uint256) {
        return uint256(RLPReaderExt.bytes32FromRLP(v[8]));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IGatewayVerifier} from '../IGatewayVerifier.sol';
import {IVerifierHooks} from '../IVerifierHooks.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {ECDSA} from '@openzeppelin/contracts/utils/cryptography/ECDSA.sol';
import {LazyOwnable} from './LazyOwnable.sol';

event TrustedVerifierChanged();

contract LazyTrustedVerifier is LazyOwnable, IGatewayVerifier {

    IVerifierHooks _hooks;
    mapping(address => bool) _signers;
    uint256 _expSec;
    string[] _urls;

    function init(
        address _owner,
        IVerifierHooks hooks,
        string[] memory urls,
        address[] memory signers,
        uint256 expSec
    ) external {
        initialize(_owner, hooks, urls, signers, expSec);
    }

    function initialize(
        address _owner,
        IVerifierHooks hooks,
        string[] memory urls,
        address[] memory signers,
        uint256 expSec
    ) internal {
        super.initialize(_owner);
        _hooks = hooks;
        _urls = urls;
        for (uint256 i; i < signers.length; i++) {
            _signers[signers[i]] = true;
        }
        _expSec = expSec;
    }

    function getExpSec() external view returns (uint256) {
        return _expSec;
    }

    function getHooks() external view returns (IVerifierHooks) {
        return _hooks;
    }

    function isSigner(address signer) external view returns (bool) {
        return _signers[signer];
    }

    function setGatewayURLs(string[] memory urls) external onlyOwner {
        _urls = urls;
        emit TrustedVerifierChanged();
    }

    function setHooks(IVerifierHooks hooks) external onlyOwner {
        _hooks = hooks;
        emit TrustedVerifierChanged();
    }

    function setExpSec(uint256 expSec) external onlyOwner {
        _expSec = expSec;
        emit TrustedVerifierChanged();
    }

    function setSigner(address signer, bool allow) external onlyOwner {
        _signers[signer] = allow;
        emit TrustedVerifierChanged();
    }

    function gatewayURLs() external view returns (string[] memory) {
        return _urls;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(block.timestamp);
    }

    struct GatewayProof {
        bytes signature;
        uint64 signedAt;
        bytes32 stateRoot;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 t = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        bytes32 hash = keccak256(
            // https://github.com/ethereum/eips/issues/191
            abi.encodePacked(
                hex'1900', // magic + version(0)
                address(0), // unbound
                p.signedAt,
                p.stateRoot
            )
        );
        address signer = ECDSA.recover(hash, p.signature);
        require(_signers[signer], 'Trusted: signer');
        uint256 dt = p.signedAt > t ? p.signedAt - t : t - p.signedAt;
        require(dt <= _expSec, 'Trusted: expired');
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, p.stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

import "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import "../ResolverBase.sol";
import "./AddrResolver.sol";
import "./IInterfaceResolver.sol";

abstract contract InterfaceResolver is IInterfaceResolver, AddrResolver {
    mapping(uint64 => mapping(bytes32 => mapping(bytes4 => address))) versionable_interfaces;

    /// Sets an interface associated with a name.
    /// Setting the address to 0 restores the default behaviour of querying the contract at `addr()` for interface support.
    /// @param node The node to update.
    /// @param interfaceID The EIP 165 interface ID.
    /// @param implementer The address of a contract that implements this interface for this node.
    function setInterface(
        bytes32 node,
        bytes4 interfaceID,
        address implementer
    ) external virtual authorised(node) {
        versionable_interfaces[recordVersions[node]][node][
            interfaceID
        ] = implementer;
        emit InterfaceChanged(node, interfaceID, implementer);
    }

    /// Returns the address of a contract that implements the specified interface for this name.
    /// If an implementer has not been set for this interfaceID and name, the resolver will query
    /// the contract at `addr()`. If `addr()` is set, a contract exists at that address, and that
    /// contract implements EIP165 and returns `true` for the specified interfaceID, its address
    /// will be returned.
    /// @param node The ENS node to query.
    /// @param interfaceID The EIP 165 interface ID to check for.
    /// @return The address that implements this interface, or 0 if the interface is unsupported.
    function interfaceImplementer(
        bytes32 node,
        bytes4 interfaceID
    ) external view virtual override returns (address) {
        address implementer = versionable_interfaces[recordVersions[node]][
            node
        ][interfaceID];
        if (implementer != address(0)) {
            return implementer;
        }

        address a = addr(node);
        if (a == address(0)) {
            return address(0);
        }

        (bool success, bytes memory returnData) = a.staticcall(
            abi.encodeWithSignature(
                "supportsInterface(bytes4)",
                type(IERC165).interfaceId
            )
        );
        if (!success || returnData.length < 32 || returnData[31] == 0) {
            // EIP 165 not supported by target
            return address(0);
        }

        (success, returnData) = a.staticcall(
            abi.encodeWithSignature("supportsInterface(bytes4)", interfaceID)
        );
        if (!success || returnData.length < 32 || returnData[31] == 0) {
            // Specified interface not supported by target
            return address(0);
        }

        return a;
    }

    function supportsInterface(
        bytes4 interfaceID
    ) public view virtual override returns (bool) {
        return
            interfaceID == type(IInterfaceResolver).interfaceId ||
            super.supportsInterface(interfaceID);
    }
}

pragma solidity >=0.8.4;

import "./ENS.sol";
import "./ENSRegistry.sol";

/// The ENS registry contract.
contract ENSRegistryWithFallback is ENSRegistry {
    ENS public old;

    /// @dev Constructs a new ENS registrar.
    constructor(ENS _old) public ENSRegistry() {
        old = _old;
    }

    /// @dev Returns the address of the resolver for the specified node.
    /// @param node The specified node.
    /// @return address of the resolver.
    function resolver(bytes32 node) public view override returns (address) {
        if (!recordExists(node)) {
            return old.resolver(node);
        }

        return super.resolver(node);
    }

    /// @dev Returns the address that owns the specified node.
    /// @param node The specified node.
    /// @return address of the owner.
    function owner(bytes32 node) public view override returns (address) {
        if (!recordExists(node)) {
            return old.owner(node);
        }

        return super.owner(node);
    }

    /// @dev Returns the TTL of a node, and any records associated with it.
    /// @param node The specified node.
    /// @return ttl of the node.
    function ttl(bytes32 node) public view override returns (uint64) {
        if (!recordExists(node)) {
            return old.ttl(node);
        }

        return super.ttl(node);
    }

    function _setOwner(bytes32 node, address owner) internal override {
        address addr = owner;
        if (addr == address(0x0)) {
            addr = address(this);
        }

        super._setOwner(node, addr);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

import {IExtendedResolver} from "./IExtendedResolver.sol";

/// @notice A resolver that calls other resolvers.
/// @dev Interface selector: `0xc7e45d73`
interface ICompositeResolver is IExtendedResolver {
    /// @notice Fetch the underlying resolver for `name`.
    ///         Callers should enable EIP-3668.
    ///
    /// * If `offchain`, additional information is necessary to locate `resolver`.
    /// * If `resolver` is null, `offchain` is irrelevant.
    ///
    /// @param name The DNS-encoded name.
    ///
    /// @return resolver The underlying resolver address.
    /// @return offchain `true` if `resolver` is offchain.
    function getResolver(
        bytes memory name
    ) external view returns (address resolver, bool offchain);

    /// @notice Determine if resolving `name` requires offchain data.
    ///
    /// @param name The DNS-encoded name.
    ///
    /// @return `true` if requires offchain data.
    function requiresOffchain(bytes calldata name) external view returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IGatewayVerifier} from './IGatewayVerifier.sol';
import {IVerifierHooks} from './IVerifierHooks.sol';

interface IStandardGatewayVerifier is IGatewayVerifier {
    function getHooks() external view returns (IVerifierHooks);
    function getWindow() external view returns (uint256);
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import {Ownable} from "@openzeppelin/contracts-v5/access/Ownable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts-v5/utils/cryptography/MessageHashUtils.sol";
import {ERC165} from "@openzeppelin/contracts-v5/utils/introspection/ERC165.sol";

import {IDefaultReverseRegistrar} from "./IDefaultReverseRegistrar.sol";
import {StandaloneReverseRegistrar} from "./StandaloneReverseRegistrar.sol";
import {SignatureUtils} from "./SignatureUtils.sol";
import {Controllable} from "../root/Controllable.sol";

/// @title Default Reverse Registrar
/// @notice A default reverse registrar. Only one instance of this contract is deployed.
contract DefaultReverseRegistrar is
    IDefaultReverseRegistrar,
    ERC165,
    StandaloneReverseRegistrar,
    Controllable
{
    using SignatureUtils for bytes;
    using MessageHashUtils for bytes32;

    /// @inheritdoc IDefaultReverseRegistrar
    function setName(string calldata name) external {
        _setName(msg.sender, name);
    }

    /// @inheritdoc IDefaultReverseRegistrar
    function setNameForAddrWithSignature(
        address addr,
        uint256 signatureExpiry,
        string calldata name,
        bytes calldata signature
    ) external {
        // Follow ERC191 version 0 https://eips.ethereum.org/EIPS/eip-191
        bytes32 message = keccak256(
            abi.encodePacked(
                address(this),
                this.setNameForAddrWithSignature.selector,
                addr,
                signatureExpiry,
                name
            )
        ).toEthSignedMessageHash();

        signature.validateSignatureWithExpiry(addr, message, signatureExpiry);

        _setName(addr, name);
    }

    function setNameForAddr(
        address addr,
        string calldata name
    ) external onlyController {
        _setName(addr, name);
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceID
    ) public view override(ERC165, StandaloneReverseRegistrar) returns (bool) {
        return
            interfaceID == type(IDefaultReverseRegistrar).interfaceId ||
            super.supportsInterface(interfaceID);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {RLPReader, RLPReaderExt} from '../RLPReaderExt.sol';

interface IRootChain {
    // https://github.com/0xPolygon/pos-contracts/blob/main/contracts/root/IRootChain.sol
    function currentHeaderBlock() external view returns (uint256);
    // https://github.com/0xPolygon/pos-contracts/blob/main/contracts/root/RootChainStorage.sol
    function headerBlocks(
        uint256
    )
        external
        view
        returns (
            bytes32 root,
            uint256 start,
            uint256 end,
            uint256 createdAt,
            address proposer
        );
}

contract PolygonPoSVerifier is AbstractVerifier {
    IRootChain immutable _rootChain;
    address immutable _poster;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IRootChain rootChain,
        address poster
    ) AbstractVerifier(urls, window, hooks) {
        _rootChain = rootChain;
        _poster = poster;
    }

    function getPoster() external view returns (address) {
        return _poster;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_rootChain.currentHeaderBlock());
    }

    struct GatewayProof {
        bytes rlpEncodedProof;
        bytes rlpEncodedBlock;
        bytes[] proofs;
        bytes order;
    }

    // gas to prove stateRoot:
    // 20240828: ~100k gas
    // 20240829: ~275k gas (forgot receipt proof)
    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 headerBlock1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        RLPReader.RLPItem[] memory v = RLPReader.readList(p.rlpEncodedProof);
        uint256 headerBlock = uint256(RLPReaderExt.bytes32FromRLP(v[0]));
        (
            bytes32 rootHash,
            uint256 l2BlockNumberStart,
            ,
            uint256 createdAt,

        ) = _rootChain.headerBlocks(headerBlock);
        require(rootHash != bytes32(0), 'PolygonPoS: checkpoint');
        if (headerBlock1 != headerBlock) {
            (, , , uint256 createdAt1, ) = _rootChain.headerBlocks(
                headerBlock1
            );
            _checkWindow(createdAt1, createdAt);
        }
        bytes memory receipt = _proveReceiptInCheckpoint(
            v,
            rootHash,
            l2BlockNumberStart
        );
        bytes32 prevBlockHash = _extractPrevBlockHash(
            receipt,
            uint256(RLPReaderExt.bytes32FromRLP(v[9])) // logIndex
        );
        require(
            prevBlockHash == keccak256(p.rlpEncodedBlock),
            'PolygonPoS: blockHash'
        );
        v = RLPReader.readList(p.rlpEncodedBlock);
        bytes32 stateRoot = RLPReaderExt.strictBytes32FromRLP(v[3]);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }

    function _extractPrevBlockHash(
        bytes memory receipt,
        uint256 logIndex
    ) internal view returns (bytes32) {
        if (uint8(receipt[0]) != 0) {
            // remove transaction type prefix
            assembly {
                mstore(add(receipt, 1), sub(mload(receipt), 1))
                receipt := add(receipt, 1)
            }
        }
        RLPReader.RLPItem[] memory v = RLPReader.readList(receipt); // receipt
        v = RLPReader.readList(v[3]); // logs
        require(v.length > logIndex, 'PolygonPoS: logIndex');
        v = RLPReader.readList(v[logIndex]); // log
        address poster = address(
            uint160(uint256(RLPReaderExt.bytes32FromRLP(v[0])))
        );
        require(poster == _poster, 'PolygonPoS: poster');
        v = RLPReader.readList(v[1]); // topics
        return RLPReaderExt.strictBytes32FromRLP(v[1]); // prevBlockHash
    }

    function _proveReceiptInCheckpoint(
        RLPReader.RLPItem[] memory v,
        bytes32 rootHash,
        uint256 l2BlockNumberStart
    ) internal pure returns (bytes memory receipt) {
        uint256 l2BlockNumber = uint256(RLPReaderExt.bytes32FromRLP(v[2]));
        bytes32 receiptsRoot = RLPReaderExt.strictBytes32FromRLP(v[5]);
        bytes32 leafHash = keccak256(
            abi.encode(
                l2BlockNumber,
                RLPReaderExt.bytes32FromRLP(v[3]), // timestamp
                RLPReaderExt.strictBytes32FromRLP(v[4]), // transactionRoot
                receiptsRoot
            )
        );
        bytes32 computedRootHash = _computeRootHash(
            leafHash,
            RLPReader.readBytes(v[1]),
            l2BlockNumber - l2BlockNumberStart
        );
        require(rootHash == computedRootHash, 'PolygonPoS: rootHash');
        receipt = RLPReader.readBytes(v[6]);
        require(
            keccak256(receipt) ==
                _computeReceiptHash(
                    receiptsRoot,
                    RLPReader.readList(RLPReader.readBytes(v[7])), // branches
                    RLPReader.readBytes(v[8]) // path using hp-encoding
                ),
            'PolygonPos: receiptsRoot'
        );
    }

    function _computeRootHash(
        bytes32 leafHash,
        bytes memory proof,
        uint256 index
    ) internal pure returns (bytes32 ret) {
        ret = leafHash;
        for (uint256 i; i < proof.length; index >>= 1) {
            bytes32 next;
            assembly {
                i := add(i, 32)
                next := mload(add(proof, i))
            }
            if (index & 1 == 0) {
                ret = keccak256(abi.encodePacked(ret, next));
            } else {
                ret = keccak256(abi.encodePacked(next, ret));
            }
        }
    }

    function _computeReceiptHash(
        bytes32 root,
        RLPReader.RLPItem[] memory parentNodes,
        bytes memory path
    ) internal pure returns (bytes32 ret) {
        path = _nibblesFromHexPrefixed(path);
        bytes32 nodeKey = root;
        uint256 pathPtr;
        for (
            uint256 i = 0;
            i < parentNodes.length && pathPtr <= path.length;
            i++
        ) {
            if (nodeKey != RLPReaderExt.keccak256FromRawRLP(parentNodes[i]))
                break;
            RLPReader.RLPItem[] memory v = RLPReader.readList(parentNodes[i]);
            if (v.length == 17) {
                // branch
                if (pathPtr == path.length) {
                    ret = keccak256(RLPReader.readBytes(v[16]));
                    break;
                }
                uint8 next = uint8(path[pathPtr]);
                if (next > 16) break;
                nodeKey = RLPReaderExt.strictBytes32FromRLP(v[next]);
                pathPtr += 1;
            } else if (v.length == 2) {
                // extension/leaf
                bytes memory frag = _nibblesFromHexPrefixed(
                    RLPReader.readBytes(v[0])
                );
                uint256 shared;
                while (
                    shared < frag.length &&
                    path[pathPtr + shared] == frag[shared]
                ) shared++;
                if (pathPtr + shared == path.length) {
                    ret = keccak256(RLPReader.readBytes(v[1]));
                    break;
                }
                if (shared == 0) break; // extension
                pathPtr += shared;
                nodeKey = RLPReaderExt.strictBytes32FromRLP(v[1]);
            } else {
                break;
            }
        }
    }

    // https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie/#specification
    function _nibblesFromHexPrefixed(
        bytes memory v
    ) internal pure returns (bytes memory nibbles) {
        if (v.length != 0) {
            uint256 start = _nibbleAt(v, 0) & 1 == 0 ? 2 : 1;
            nibbles = new bytes((v.length << 1) - start);
            for (uint256 i; i < nibbles.length; i++) {
                nibbles[i] = bytes1(_nibbleAt(v, start + i));
            }
        }
    }
    function _nibbleAt(bytes memory v, uint256 i) private pure returns (uint8) {
        uint8 b = uint8(v[i >> 1]);
        return i & 1 == 0 ? b >> 4 : b & 15;
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import {Ownable} from "@openzeppelin/contracts-v5/access/Ownable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts-v5/utils/cryptography/MessageHashUtils.sol";
import {ERC165} from "@openzeppelin/contracts-v5/utils/introspection/ERC165.sol";

import {IL2ReverseRegistrar} from "./IL2ReverseRegistrar.sol";
import {StandaloneReverseRegistrar} from "./StandaloneReverseRegistrar.sol";
import {SignatureUtils} from "./SignatureUtils.sol";

/// @title L2 Reverse Registrar
/// @notice An L2 Reverse Registrar. Deployed to each L2 chain.
contract L2ReverseRegistrar is
    IL2ReverseRegistrar,
    ERC165,
    StandaloneReverseRegistrar
{
    using SignatureUtils for bytes;
    using MessageHashUtils for bytes32;

    /// @notice The coin type for the chain this contract is deployed to.
    uint256 public immutable coinType;

    /// @notice Thrown when the specified address is not the owner of the contract
    error NotOwnerOfContract();

    /// @notice Thrown when the coin type is not found in the provided array
    error CoinTypeNotFound();

    /// @notice The caller is not authorised to perform the action
    error Unauthorised();

    /// @notice Checks if the caller is authorised
    ///
    /// @param addr The address to check.
    modifier authorised(address addr) {
        if (addr != msg.sender && !_ownsContract(addr, msg.sender)) {
            revert Unauthorised();
        }
        _;
    }

    /// @notice Ensures the coin type of the contract is included in the provided array
    ///
    /// @param coinTypes The coin types to check.
    modifier validCoinTypes(uint256[] calldata coinTypes) {
        _validateCoinTypes(coinTypes);
        _;
    }

    /// @notice Initialises the contract by setting the coin type.
    ///
    /// @param coinType_ The cointype converted from the chainId of the chain this contract is deployed to.
    constructor(uint256 coinType_) {
        coinType = coinType_;
    }

    /// @inheritdoc IL2ReverseRegistrar
    function setName(string calldata name) external authorised(msg.sender) {
        _setName(msg.sender, name);
    }

    /// @inheritdoc IL2ReverseRegistrar
    function setNameForAddr(
        address addr,
        string calldata name
    ) external authorised(addr) {
        _setName(addr, name);
    }

    /// @inheritdoc IL2ReverseRegistrar
    function setNameForAddrWithSignature(
        address addr,
        uint256 signatureExpiry,
        string calldata name,
        uint256[] calldata coinTypes,
        bytes calldata signature
    ) external validCoinTypes(coinTypes) {
        // Follow ERC191 version 0 https://eips.ethereum.org/EIPS/eip-191
        bytes32 message = keccak256(
            abi.encodePacked(
                address(this),
                this.setNameForAddrWithSignature.selector,
                addr,
                signatureExpiry,
                name,
                coinTypes
            )
        ).toEthSignedMessageHash();

        signature.validateSignatureWithExpiry(addr, message, signatureExpiry);

        _setName(addr, name);
    }

    /// @inheritdoc IL2ReverseRegistrar
    function setNameForOwnableWithSignature(
        address contractAddr,
        address owner,
        uint256 signatureExpiry,
        string calldata name,
        uint256[] calldata coinTypes,
        bytes calldata signature
    ) external validCoinTypes(coinTypes) {
        // Follow ERC191 version 0 https://eips.ethereum.org/EIPS/eip-191
        bytes32 message = keccak256(
            abi.encodePacked(
                address(this),
                this.setNameForOwnableWithSignature.selector,
                contractAddr,
                owner,
                signatureExpiry,
                name,
                coinTypes
            )
        ).toEthSignedMessageHash();

        if (!_ownsContract(contractAddr, owner)) revert NotOwnerOfContract();

        signature.validateSignatureWithExpiry(owner, message, signatureExpiry);

        _setName(contractAddr, name);
    }

    /// @notice Checks if the provided contractAddr is a contract and is owned by the
    ///         provided addr.
    ///
    /// @param contractAddr The address of the contract to check.
    /// @param addr The address to check ownership against.
    function _ownsContract(
        address contractAddr,
        address addr
    ) internal view returns (bool) {
        if (contractAddr.code.length == 0) return false;
        try Ownable(contractAddr).owner() returns (address owner) {
            return owner == addr;
        } catch {
            return false;
        }
    }

    /// @notice Ensures the coin type for the contract is included in the provided array.
    ///
    /// @param coinTypes The coin types to check.
    function _validateCoinTypes(uint256[] calldata coinTypes) internal view {
        for (uint256 i = 0; i < coinTypes.length; i++) {
            if (coinTypes[i] == coinType) return;
        }

        revert CoinTypeNotFound();
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceID
    ) public view override(ERC165, StandaloneReverseRegistrar) returns (bool) {
        return
            interfaceID == type(IL2ReverseRegistrar).interfaceId ||
            super.supportsInterface(interfaceID);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {AbstractReverseResolver} from "./AbstractReverseResolver.sol";
import {IStandaloneReverseRegistrar} from "../reverseRegistrar/IStandaloneReverseRegistrar.sol";
import {INameReverser} from "./INameReverser.sol";
import {COIN_TYPE_DEFAULT} from "../utils/ENSIP19.sol";

/// @title Default Reverse Resolver
/// @notice Reverses an EVM address using the `IStandaloneReverseRegistrar` for "default.reverse".
contract DefaultReverseResolver is AbstractReverseResolver {
    constructor(
        IStandaloneReverseRegistrar defaultRegistrar
    ) AbstractReverseResolver(COIN_TYPE_DEFAULT, address(defaultRegistrar)) {}

    /// @inheritdoc AbstractReverseResolver
    function _resolveName(
        address addr
    ) internal view override returns (string memory name) {
        name = IStandaloneReverseRegistrar(chainRegistrar).nameForAddr(addr);
    }

    /// @inheritdoc INameReverser
    function resolveNames(
        address[] memory addrs
    ) external view returns (string[] memory names) {
        names = new string[](addrs.length);
        for (uint256 i; i < addrs.length; ++i) {
            names[i] = _resolveName(addrs[i]);
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

import {ResolverBase, IERC165} from "../ResolverBase.sol";
import {IAddrResolver} from "./IAddrResolver.sol";
import {IAddressResolver} from "./IAddressResolver.sol";
import {IHasAddressResolver} from "./IHasAddressResolver.sol";
import {ENSIP19, COIN_TYPE_ETH, COIN_TYPE_DEFAULT} from "../../utils/ENSIP19.sol";

abstract contract AddrResolver is
    IAddrResolver,
    IAddressResolver,
    IHasAddressResolver,
    ResolverBase
{
    mapping(uint64 => mapping(bytes32 => mapping(uint256 => bytes))) versionable_addresses;

    /// @notice The supplied address could not be converted to `address`.
    /// @dev Error selector: `0x8d666f60`
    error InvalidEVMAddress(bytes addressBytes);

    /// @notice Set `addr(60)` of the associated ENS node.
    ///         `address(0)` is stored as `new bytes(20)`.
    /// @param node The node to update.
    /// @param _addr The address to set.
    function setAddr(
        bytes32 node,
        address _addr
    ) external virtual authorised(node) {
        setAddr(node, COIN_TYPE_ETH, abi.encodePacked(_addr));
    }

    /// @notice Get `addr(60)` as `address` of the associated ENS node.
    /// @param node The node to query.
    /// @return The associated address.
    function addr(
        bytes32 node
    ) public view virtual override returns (address payable) {
        return payable(address(bytes20(addr(node, COIN_TYPE_ETH))));
    }

    /// @notice Set the address for coin type of the associated ENS node.
    ///         Reverts `InvalidEVMAddress` if coin type is EVM and not 0 or 20 bytes.
    /// @param node The node to update.
    /// @param coinType The coin type.
    /// @param addressBytes The address to set.
    function setAddr(
        bytes32 node,
        uint256 coinType,
        bytes memory addressBytes
    ) public virtual authorised(node) {
        if (
            addressBytes.length != 0 &&
            addressBytes.length != 20 &&
            ENSIP19.isEVMCoinType(coinType)
        ) {
            revert InvalidEVMAddress(addressBytes);
        }
        emit AddressChanged(node, coinType, addressBytes);
        if (coinType == COIN_TYPE_ETH) {
            emit AddrChanged(node, address(bytes20(addressBytes)));
        }
        versionable_addresses[recordVersions[node]][node][
            coinType
        ] = addressBytes;
    }

    /// @notice Get the address for coin type of the associated ENS node.
    ///         If coin type is EVM and empty, defaults to `addr(COIN_TYPE_DEFAULT)`.
    /// @param node The node to query.
    /// @param coinType The coin type.
    /// @return addressBytes The assocated address.
    function addr(
        bytes32 node,
        uint256 coinType
    ) public view virtual override returns (bytes memory addressBytes) {
        mapping(uint256 => bytes) storage addrs = versionable_addresses[
            recordVersions[node]
        ][node];
        addressBytes = addrs[coinType];
        if (
            addressBytes.length == 0 && ENSIP19.chainFromCoinType(coinType) > 0
        ) {
            addressBytes = addrs[COIN_TYPE_DEFAULT];
        }
    }

    /// @inheritdoc IHasAddressResolver
    function hasAddr(
        bytes32 node,
        uint256 coinType
    ) external view returns (bool) {
        return
            versionable_addresses[recordVersions[node]][node][coinType].length >
            0;
    }

    /// @inheritdoc IERC165
    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override returns (bool) {
        return
            type(IAddrResolver).interfaceId == interfaceId ||
            type(IAddressResolver).interfaceId == interfaceId ||
            type(IHasAddressResolver).interfaceId == interfaceId ||
            super.supportsInterface(interfaceId);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';

interface IScrollChain {
    function lastFinalizedBatchIndex() external view returns (uint256);
    function finalizedStateRoots(
        uint256 batchIndex
    ) external view returns (bytes32);
}

contract ScrollVerifier is AbstractVerifier {
    IScrollChain immutable _rollup;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IScrollChain rollup
    ) AbstractVerifier(urls, window, hooks) {
        _rollup = rollup;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_rollup.lastFinalizedBatchIndex());
    }

    struct GatewayProof {
        uint256 batchIndex;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 batchIndex1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        _checkWindow(batchIndex1, p.batchIndex);
        bytes32 stateRoot = _rollup.finalizedStateRoots(p.batchIndex);
        require(stateRoot != bytes32(0), 'Scroll: not finalized');
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';

contract UncheckedVerifier is AbstractVerifier {
    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks
    ) AbstractVerifier(urls, window, hooks) {}

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(block.timestamp);
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory values, uint8 exitCode) {
        uint256 t1 = abi.decode(context, (uint256));
        (uint256 t, bytes[] memory proofs, bytes memory order) = abi.decode(
            proof,
            (uint256, bytes[], bytes)
        );
        _checkWindow(t1, t);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, bytes32(0), proofs, order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {Hashing, Types} from '../../lib/optimism/packages/contracts-bedrock/src/libraries/Hashing.sol';
import { IOptimismPortal, IOPFaultGameFinder, IDisputeGame, OPFaultParams } from './OPInterfaces.sol';



contract OPFaultVerifier is AbstractVerifier {
    IOptimismPortal immutable _portal;
    IOPFaultGameFinder immutable _gameFinder;
    OPFaultParams private _params;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IOPFaultGameFinder gameFinder,
        OPFaultParams memory params
    ) AbstractVerifier(urls, window, hooks) {
        _portal = params.portal;
        _gameFinder = gameFinder;
        _params = params;
    }

    function getLatestContext() external view virtual returns (bytes memory) {
        return
            abi.encode(
                _gameFinder.findGameIndex(
                    _params,
                    0
                )
            );
    }

    struct GatewayProof {
        uint256 gameIndex;
        Types.OutputRootProof outputRootProof;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 gameIndex1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        (, , IDisputeGame gameProxy, uint256 blockNumber,) = _gameFinder
            .gameAtIndex(_params, p.gameIndex);
        require(blockNumber != 0, 'OPFault: invalid game');
        if (p.gameIndex != gameIndex1) {
            (, , IDisputeGame gameProxy1) = _portal
                .disputeGameFactory()
                .gameAtIndex(gameIndex1);
            _checkWindow(_getGameTime(gameProxy1), _getGameTime(gameProxy));
        }
        require(
            gameProxy.rootClaim() ==
                Hashing.hashOutputRootProof(p.outputRootProof),
            'OPFault: rootClaim'
        );
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(
                    0,
                    p.outputRootProof.stateRoot,
                    p.proofs,
                    p.order,
                    _hooks
                )
            );
    }

    function _getGameTime(IDisputeGame g) internal view returns (uint256) {
        return
            _params.minAgeSec == 0 ? g.resolvedAt() : g.createdAt();
    }
}

//SPDX-License-Identifier: MIT
pragma solidity ~0.8.17;

import {ERC1155Fuse, IERC165, IERC1155MetadataURI} from "./ERC1155Fuse.sol";
import {Controllable} from "./Controllable.sol";
import {INameWrapper, CANNOT_UNWRAP, CANNOT_BURN_FUSES, CANNOT_TRANSFER, CANNOT_SET_RESOLVER, CANNOT_SET_TTL, CANNOT_CREATE_SUBDOMAIN, CANNOT_APPROVE, PARENT_CANNOT_CONTROL, CAN_DO_EVERYTHING, IS_DOT_ETH, CAN_EXTEND_EXPIRY, PARENT_CONTROLLED_FUSES, USER_SETTABLE_FUSES} from "./INameWrapper.sol";
import {INameWrapperUpgrade} from "./INameWrapperUpgrade.sol";
import {IMetadataService} from "./IMetadataService.sol";
import {ENS} from "../registry/ENS.sol";
import {IReverseRegistrar} from "../reverseRegistrar/IReverseRegistrar.sol";
import {ReverseClaimer} from "../reverseRegistrar/ReverseClaimer.sol";
import {IBaseRegistrar} from "../ethregistrar/IBaseRegistrar.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import "@openzeppelin/contracts/token/ERC1155/IERC1155.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {BytesUtils_LEGACY} from "../utils/BytesUtils_LEGACY.sol";
import {ERC20Recoverable} from "../utils/ERC20Recoverable.sol";

error Unauthorised(bytes32 node, address addr);
error IncompatibleParent();
error IncorrectTokenType();
error LabelMismatch(bytes32 labelHash, bytes32 expectedLabelhash);
error LabelTooShort();
error LabelTooLong(string label);
error IncorrectTargetOwner(address owner);
error CannotUpgrade();
error OperationProhibited(bytes32 node);
error NameIsNotWrapped();
error NameIsStillExpired();

contract NameWrapper is
    Ownable,
    ERC1155Fuse,
    INameWrapper,
    Controllable,
    IERC721Receiver,
    ERC20Recoverable,
    ReverseClaimer
{
    using BytesUtils_LEGACY for bytes;

    ENS public immutable ens;
    IBaseRegistrar public immutable registrar;
    IMetadataService public metadataService;
    mapping(bytes32 => bytes) public names;
    string public constant name = "NameWrapper";

    uint64 private constant GRACE_PERIOD = 90 days;
    bytes32 private constant ETH_NODE =
        0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae;
    bytes32 private constant ETH_LABELHASH =
        0x4f5b812789fc606be1b3b16908db13fc7a9adf7ca72641f84d75b47069d3d7f0;
    bytes32 private constant ROOT_NODE =
        0x0000000000000000000000000000000000000000000000000000000000000000;

    INameWrapperUpgrade public upgradeContract;
    uint64 private constant MAX_EXPIRY = type(uint64).max;

    constructor(
        ENS _ens,
        IBaseRegistrar _registrar,
        IMetadataService _metadataService
    ) ReverseClaimer(_ens, msg.sender) {
        ens = _ens;
        registrar = _registrar;
        metadataService = _metadataService;

        /* Burn PARENT_CANNOT_CONTROL and CANNOT_UNWRAP fuses for ROOT_NODE and ETH_NODE and set expiry to max */

        _setData(
            uint256(ETH_NODE),
            address(0),
            uint32(PARENT_CANNOT_CONTROL | CANNOT_UNWRAP),
            MAX_EXPIRY
        );
        _setData(
            uint256(ROOT_NODE),
            address(0),
            uint32(PARENT_CANNOT_CONTROL | CANNOT_UNWRAP),
            MAX_EXPIRY
        );
        names[ROOT_NODE] = "\x00";
        names[ETH_NODE] = "\x03eth\x00";
    }

    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override(ERC1155Fuse, INameWrapper) returns (bool) {
        return
            interfaceId == type(INameWrapper).interfaceId ||
            interfaceId == type(IERC721Receiver).interfaceId ||
            super.supportsInterface(interfaceId);
    }

    /* ERC1155 Fuse */

    /// @notice Gets the owner of a name
    /// @param id Label as a string of the .eth domain to wrap
    /// @return owner The owner of the name
    function ownerOf(
        uint256 id
    ) public view override(ERC1155Fuse, INameWrapper) returns (address owner) {
        return super.ownerOf(id);
    }

    /// @notice Gets the owner of a name
    /// @param id Namehash of the name
    /// @return operator Approved operator of a name
    function getApproved(
        uint256 id
    )
        public
        view
        override(ERC1155Fuse, INameWrapper)
        returns (address operator)
    {
        address owner = ownerOf(id);
        if (owner == address(0)) {
            return address(0);
        }
        return super.getApproved(id);
    }

    /// @notice Approves an address for a name
    /// @param to address to approve
    /// @param tokenId name to approve
    function approve(
        address to,
        uint256 tokenId
    ) public override(ERC1155Fuse, INameWrapper) {
        (, uint32 fuses, ) = getData(tokenId);
        if (fuses & CANNOT_APPROVE == CANNOT_APPROVE) {
            revert OperationProhibited(bytes32(tokenId));
        }
        super.approve(to, tokenId);
    }

    /// @notice Gets the data for a name
    /// @param id Namehash of the name
    /// @return owner Owner of the name
    /// @return fuses Fuses of the name
    /// @return expiry Expiry of the name
    function getData(
        uint256 id
    )
        public
        view
        override(ERC1155Fuse, INameWrapper)
        returns (address owner, uint32 fuses, uint64 expiry)
    {
        (owner, fuses, expiry) = super.getData(id);

        (owner, fuses) = _clearOwnerAndFuses(owner, fuses, expiry);
    }

    /* Metadata service */

    /// @notice Set the metadata service. Only the owner can do this
    /// @param _metadataService The new metadata service
    function setMetadataService(
        IMetadataService _metadataService
    ) public onlyOwner {
        metadataService = _metadataService;
    }

    /// @notice Get the metadata uri
    /// @param tokenId The id of the token
    /// @return string uri of the metadata service
    function uri(
        uint256 tokenId
    )
        public
        view
        override(INameWrapper, IERC1155MetadataURI)
        returns (string memory)
    {
        return metadataService.uri(tokenId);
    }

    /// @notice Set the address of the upgradeContract of the contract. only admin can do this
    /// @dev The default value of upgradeContract is the 0 address. Use the 0 address at any time
    /// to make the contract not upgradable.
    /// @param _upgradeAddress address of an upgraded contract
    function setUpgradeContract(
        INameWrapperUpgrade _upgradeAddress
    ) public onlyOwner {
        if (address(upgradeContract) != address(0)) {
            registrar.setApprovalForAll(address(upgradeContract), false);
            ens.setApprovalForAll(address(upgradeContract), false);
        }

        upgradeContract = _upgradeAddress;

        if (address(upgradeContract) != address(0)) {
            registrar.setApprovalForAll(address(upgradeContract), true);
            ens.setApprovalForAll(address(upgradeContract), true);
        }
    }

    /// @notice Checks if msg.sender is the owner or operator of the owner of a name
    /// @param node namehash of the name to check
    modifier onlyTokenOwner(bytes32 node) {
        if (!canModifyName(node, msg.sender)) {
            revert Unauthorised(node, msg.sender);
        }

        _;
    }

    /// @notice Checks if owner or operator of the owner
    /// @param node namehash of the name to check
    /// @param addr which address to check permissions for
    /// @return whether or not is owner or operator
    function canModifyName(
        bytes32 node,
        address addr
    ) public view returns (bool) {
        (address owner, uint32 fuses, uint64 expiry) = getData(uint256(node));
        return
            (owner == addr || isApprovedForAll(owner, addr)) &&
            !_isETH2LDInGracePeriod(fuses, expiry);
    }

    /// @notice Checks if owner/operator or approved by owner
    /// @param node namehash of the name to check
    /// @param addr which address to check permissions for
    /// @return whether or not is owner/operator or approved
    function canExtendSubnames(
        bytes32 node,
        address addr
    ) public view returns (bool) {
        (address owner, uint32 fuses, uint64 expiry) = getData(uint256(node));
        return
            (owner == addr ||
                isApprovedForAll(owner, addr) ||
                getApproved(uint256(node)) == addr) &&
            !_isETH2LDInGracePeriod(fuses, expiry);
    }

    /// @notice Wraps a .eth domain, creating a new token and sending the original ERC721 token to this contract
    /// @dev Can be called by the owner of the name on the .eth registrar or an authorised caller on the registrar
    /// @param label Label as a string of the .eth domain to wrap
    /// @param wrappedOwner Owner of the name in this contract
    /// @param ownerControlledFuses Initial owner-controlled fuses to set
    /// @param resolver Resolver contract address
    function wrapETH2LD(
        string calldata label,
        address wrappedOwner,
        uint16 ownerControlledFuses,
        address resolver
    ) public returns (uint64 expiry) {
        uint256 tokenId = uint256(keccak256(bytes(label)));
        address registrant = registrar.ownerOf(tokenId);
        if (
            registrant != msg.sender &&
            !registrar.isApprovedForAll(registrant, msg.sender)
        ) {
            revert Unauthorised(
                _makeNode(ETH_NODE, bytes32(tokenId)),
                msg.sender
            );
        }

        // transfer the token from the user to this contract
        registrar.transferFrom(registrant, address(this), tokenId);

        // transfer the ens record back to the new owner (this contract)
        registrar.reclaim(tokenId, address(this));

        expiry = uint64(registrar.nameExpires(tokenId)) + GRACE_PERIOD;

        _wrapETH2LD(
            label,
            wrappedOwner,
            ownerControlledFuses,
            expiry,
            resolver
        );
    }

    /// @dev Registers a new .eth second-level domain and wraps it.
    ///      Only callable by authorised controllers.
    /// @param label The label to register (Eg, 'foo' for 'foo.eth').
    /// @param wrappedOwner The owner of the wrapped name.
    /// @param duration The duration, in seconds, to register the name for.
    /// @param resolver The resolver address to set on the ENS registry (optional).
    /// @param ownerControlledFuses Initial owner-controlled fuses to set
    /// @return registrarExpiry The expiry date of the new name on the .eth registrar, in seconds since the Unix epoch.
    function registerAndWrapETH2LD(
        string calldata label,
        address wrappedOwner,
        uint256 duration,
        address resolver,
        uint16 ownerControlledFuses
    ) external onlyController returns (uint256 registrarExpiry) {
        uint256 tokenId = uint256(keccak256(bytes(label)));
        registrarExpiry = registrar.register(tokenId, address(this), duration);
        _wrapETH2LD(
            label,
            wrappedOwner,
            ownerControlledFuses,
            uint64(registrarExpiry) + GRACE_PERIOD,
            resolver
        );
    }

    /// @notice Renews a .eth second-level domain.
    /// @dev Only callable by authorised controllers.
    /// @param tokenId The hash of the label to register (eg, `keccak256('foo')`, for 'foo.eth').
    /// @param duration The number of seconds to renew the name for.
    /// @return expires The expiry date of the name on the .eth registrar, in seconds since the Unix epoch.
    function renew(
        uint256 tokenId,
        uint256 duration
    ) external onlyController returns (uint256 expires) {
        bytes32 node = _makeNode(ETH_NODE, bytes32(tokenId));

        uint256 registrarExpiry = registrar.renew(tokenId, duration);

        // Do not set anything in wrapper if name is not wrapped
        try registrar.ownerOf(tokenId) returns (address registrarOwner) {
            if (
                registrarOwner != address(this) ||
                ens.owner(node) != address(this)
            ) {
                return registrarExpiry;
            }
        } catch {
            return registrarExpiry;
        }

        // Set expiry in Wrapper
        uint64 expiry = uint64(registrarExpiry) + GRACE_PERIOD;

        // Use super to allow names expired on the wrapper, but not expired on the registrar to renew()
        (address owner, uint32 fuses, ) = super.getData(uint256(node));
        _setData(node, owner, fuses, expiry);

        return registrarExpiry;
    }

    /// @notice Wraps a non .eth domain, of any kind. Could be a DNSSEC name vitalik.xyz or a subdomain
    /// @dev Can be called by the owner in the registry or an authorised caller in the registry
    /// @param name The name to wrap, in DNS format
    /// @param wrappedOwner Owner of the name in this contract
    /// @param resolver Resolver contract
    function wrap(
        bytes calldata name,
        address wrappedOwner,
        address resolver
    ) public {
        (bytes32 labelhash, uint256 offset) = name.readLabel(0);
        bytes32 parentNode = name.namehash(offset);
        bytes32 node = _makeNode(parentNode, labelhash);

        names[node] = name;

        if (parentNode == ETH_NODE) {
            revert IncompatibleParent();
        }

        address owner = ens.owner(node);

        if (owner != msg.sender && !ens.isApprovedForAll(owner, msg.sender)) {
            revert Unauthorised(node, msg.sender);
        }

        if (resolver != address(0)) {
            ens.setResolver(node, resolver);
        }

        ens.setOwner(node, address(this));

        _wrap(node, name, wrappedOwner, 0, 0);
    }

    /// @notice Unwraps a .eth domain. e.g. vitalik.eth
    /// @dev Can be called by the owner in the wrapper or an authorised caller in the wrapper
    /// @param labelhash Labelhash of the .eth domain
    /// @param registrant Sets the owner in the .eth registrar to this address
    /// @param controller Sets the owner in the registry to this address
    function unwrapETH2LD(
        bytes32 labelhash,
        address registrant,
        address controller
    ) public onlyTokenOwner(_makeNode(ETH_NODE, labelhash)) {
        if (registrant == address(this)) {
            revert IncorrectTargetOwner(registrant);
        }
        _unwrap(_makeNode(ETH_NODE, labelhash), controller);
        registrar.safeTransferFrom(
            address(this),
            registrant,
            uint256(labelhash)
        );
    }

    /// @notice Unwraps a non .eth domain, of any kind. Could be a DNSSEC name vitalik.xyz or a subdomain
    /// @dev Can be called by the owner in the wrapper or an authorised caller in the wrapper
    /// @param parentNode Parent namehash of the name e.g. vitalik.xyz would be namehash('xyz')
    /// @param labelhash Labelhash of the name, e.g. vitalik.xyz would be keccak256('vitalik')
    /// @param controller Sets the owner in the registry to this address
    function unwrap(
        bytes32 parentNode,
        bytes32 labelhash,
        address controller
    ) public onlyTokenOwner(_makeNode(parentNode, labelhash)) {
        if (parentNode == ETH_NODE) {
            revert IncompatibleParent();
        }
        if (controller == address(0x0) || controller == address(this)) {
            revert IncorrectTargetOwner(controller);
        }
        _unwrap(_makeNode(parentNode, labelhash), controller);
    }

    /// @notice Sets fuses of a name
    /// @param node Namehash of the name
    /// @param ownerControlledFuses Owner-controlled fuses to burn
    /// @return Old fuses
    function setFuses(
        bytes32 node,
        uint16 ownerControlledFuses
    )
        public
        onlyTokenOwner(node)
        operationAllowed(node, CANNOT_BURN_FUSES)
        returns (uint32)
    {
        // owner protected by onlyTokenOwner
        (address owner, uint32 oldFuses, uint64 expiry) = getData(
            uint256(node)
        );
        _setFuses(node, owner, ownerControlledFuses | oldFuses, expiry, expiry);
        return oldFuses;
    }

    /// @notice Extends expiry for a name
    /// @param parentNode Parent namehash of the name e.g. vitalik.xyz would be namehash('xyz')
    /// @param labelhash Labelhash of the name, e.g. vitalik.xyz would be keccak256('vitalik')
    /// @param expiry When the name will expire in seconds since the Unix epoch
    /// @return New expiry
    function extendExpiry(
        bytes32 parentNode,
        bytes32 labelhash,
        uint64 expiry
    ) public returns (uint64) {
        bytes32 node = _makeNode(parentNode, labelhash);

        if (!_isWrapped(node)) {
            revert NameIsNotWrapped();
        }

        // this flag is used later, when checking fuses
        bool canExtendSubname = canExtendSubnames(parentNode, msg.sender);
        // only allow the owner of the name or owner of the parent name
        if (!canExtendSubname && !canModifyName(node, msg.sender)) {
            revert Unauthorised(node, msg.sender);
        }

        (address owner, uint32 fuses, uint64 oldExpiry) = getData(
            uint256(node)
        );

        // Either CAN_EXTEND_EXPIRY must be set, or the caller must have permission to modify the parent name
        if (!canExtendSubname && fuses & CAN_EXTEND_EXPIRY == 0) {
            revert OperationProhibited(node);
        }

        // Max expiry is set to the expiry of the parent
        (, , uint64 maxExpiry) = getData(uint256(parentNode));
        expiry = _normaliseExpiry(expiry, oldExpiry, maxExpiry);

        _setData(node, owner, fuses, expiry);
        emit ExpiryExtended(node, expiry);
        return expiry;
    }

    /// @notice Upgrades a domain of any kind. Could be a .eth name vitalik.eth, a DNSSEC name vitalik.xyz, or a subdomain
    /// @dev Can be called by the owner or an authorised caller
    /// @param name The name to upgrade, in DNS format
    /// @param extraData Extra data to pass to the upgrade contract
    function upgrade(bytes calldata name, bytes calldata extraData) public {
        bytes32 node = name.namehash(0);

        if (address(upgradeContract) == address(0)) {
            revert CannotUpgrade();
        }

        if (!canModifyName(node, msg.sender)) {
            revert Unauthorised(node, msg.sender);
        }

        (address currentOwner, uint32 fuses, uint64 expiry) = getData(
            uint256(node)
        );

        address approved = getApproved(uint256(node));

        _burn(uint256(node));

        upgradeContract.wrapFromUpgrade(
            name,
            currentOwner,
            fuses,
            expiry,
            approved,
            extraData
        );
    }

    ///     /* @notice Sets fuses of a name that you own the parent of
    /// @param parentNode Parent namehash of the name e.g. vitalik.xyz would be namehash('xyz')
    /// @param labelhash Labelhash of the name, e.g. vitalik.xyz would be keccak256('vitalik')
    /// @param fuses Fuses to burn
    /// @param expiry When the name will expire in seconds since the Unix epoch
    function setChildFuses(
        bytes32 parentNode,
        bytes32 labelhash,
        uint32 fuses,
        uint64 expiry
    ) public {
        bytes32 node = _makeNode(parentNode, labelhash);
        _checkFusesAreSettable(node, fuses);
        (address owner, uint32 oldFuses, uint64 oldExpiry) = getData(
            uint256(node)
        );
        if (owner == address(0) || ens.owner(node) != address(this)) {
            revert NameIsNotWrapped();
        }
        // max expiry is set to the expiry of the parent
        (, uint32 parentFuses, uint64 maxExpiry) = getData(uint256(parentNode));
        if (parentNode == ROOT_NODE) {
            if (!canModifyName(node, msg.sender)) {
                revert Unauthorised(node, msg.sender);
            }
        } else {
            if (!canModifyName(parentNode, msg.sender)) {
                revert Unauthorised(parentNode, msg.sender);
            }
        }

        _checkParentFuses(node, fuses, parentFuses);

        expiry = _normaliseExpiry(expiry, oldExpiry, maxExpiry);

        // if PARENT_CANNOT_CONTROL has been burned and fuses have changed
        if (
            oldFuses & PARENT_CANNOT_CONTROL != 0 &&
            oldFuses | fuses != oldFuses
        ) {
            revert OperationProhibited(node);
        }
        fuses |= oldFuses;
        _setFuses(node, owner, fuses, oldExpiry, expiry);
    }

    /// @notice Sets the subdomain owner in the registry and then wraps the subdomain
    /// @param parentNode Parent namehash of the subdomain
    /// @param label Label of the subdomain as a string
    /// @param owner New owner in the wrapper
    /// @param fuses Initial fuses for the wrapped subdomain
    /// @param expiry When the name will expire in seconds since the Unix epoch
    /// @return node Namehash of the subdomain
    function setSubnodeOwner(
        bytes32 parentNode,
        string calldata label,
        address owner,
        uint32 fuses,
        uint64 expiry
    ) public onlyTokenOwner(parentNode) returns (bytes32 node) {
        bytes32 labelhash = keccak256(bytes(label));
        node = _makeNode(parentNode, labelhash);
        _checkCanCallSetSubnodeOwner(parentNode, node);
        _checkFusesAreSettable(node, fuses);
        bytes memory name = _saveLabel(parentNode, node, label);
        expiry = _checkParentFusesAndExpiry(parentNode, node, fuses, expiry);

        if (!_isWrapped(node)) {
            ens.setSubnodeOwner(parentNode, labelhash, address(this));
            _wrap(node, name, owner, fuses, expiry);
        } else {
            _updateName(parentNode, node, label, owner, fuses, expiry);
        }
    }

    /// @notice Sets the subdomain owner in the registry with records and then wraps the subdomain
    /// @param parentNode parent namehash of the subdomain
    /// @param label label of the subdomain as a string
    /// @param owner new owner in the wrapper
    /// @param resolver resolver contract in the registry
    /// @param ttl ttl in the registry
    /// @param fuses initial fuses for the wrapped subdomain
    /// @param expiry When the name will expire in seconds since the Unix epoch
    /// @return node Namehash of the subdomain
    function setSubnodeRecord(
        bytes32 parentNode,
        string memory label,
        address owner,
        address resolver,
        uint64 ttl,
        uint32 fuses,
        uint64 expiry
    ) public onlyTokenOwner(parentNode) returns (bytes32 node) {
        bytes32 labelhash = keccak256(bytes(label));
        node = _makeNode(parentNode, labelhash);
        _checkCanCallSetSubnodeOwner(parentNode, node);
        _checkFusesAreSettable(node, fuses);
        _saveLabel(parentNode, node, label);
        expiry = _checkParentFusesAndExpiry(parentNode, node, fuses, expiry);
        if (!_isWrapped(node)) {
            ens.setSubnodeRecord(
                parentNode,
                labelhash,
                address(this),
                resolver,
                ttl
            );
            _storeNameAndWrap(parentNode, node, label, owner, fuses, expiry);
        } else {
            ens.setSubnodeRecord(
                parentNode,
                labelhash,
                address(this),
                resolver,
                ttl
            );
            _updateName(parentNode, node, label, owner, fuses, expiry);
        }
    }

    /// @notice Sets records for the name in the ENS Registry
    /// @param node Namehash of the name to set a record for
    /// @param owner New owner in the registry
    /// @param resolver Resolver contract
    /// @param ttl Time to live in the registry
    function setRecord(
        bytes32 node,
        address owner,
        address resolver,
        uint64 ttl
    )
        public
        onlyTokenOwner(node)
        operationAllowed(
            node,
            CANNOT_TRANSFER | CANNOT_SET_RESOLVER | CANNOT_SET_TTL
        )
    {
        ens.setRecord(node, address(this), resolver, ttl);
        if (owner == address(0)) {
            (, uint32 fuses, ) = getData(uint256(node));
            if (fuses & IS_DOT_ETH == IS_DOT_ETH) {
                revert IncorrectTargetOwner(owner);
            }
            _unwrap(node, address(0));
        } else {
            address oldOwner = ownerOf(uint256(node));
            _transfer(oldOwner, owner, uint256(node), 1, "");
        }
    }

    /// @notice Sets resolver contract in the registry
    /// @param node namehash of the name
    /// @param resolver the resolver contract
    function setResolver(
        bytes32 node,
        address resolver
    ) public onlyTokenOwner(node) operationAllowed(node, CANNOT_SET_RESOLVER) {
        ens.setResolver(node, resolver);
    }

    /// @notice Sets TTL in the registry
    /// @param node Namehash of the name
    /// @param ttl TTL in the registry
    function setTTL(
        bytes32 node,
        uint64 ttl
    ) public onlyTokenOwner(node) operationAllowed(node, CANNOT_SET_TTL) {
        ens.setTTL(node, ttl);
    }

    /// @dev Allows an operation only if none of the specified fuses are burned.
    /// @param node The namehash of the name to check fuses on.
    /// @param fuseMask A bitmask of fuses that must not be burned.
    modifier operationAllowed(bytes32 node, uint32 fuseMask) {
        (, uint32 fuses, ) = getData(uint256(node));
        if (fuses & fuseMask != 0) {
            revert OperationProhibited(node);
        }
        _;
    }

    /// @notice Check whether a name can call setSubnodeOwner/setSubnodeRecord
    /// @dev Checks both CANNOT_CREATE_SUBDOMAIN and PARENT_CANNOT_CONTROL and whether they have been burnt
    ///      and checks whether the owner of the subdomain is 0x0 for creating or already exists for
    ///      replacing a subdomain. If either conditions are true, then it is possible to call
    ///      setSubnodeOwner
    /// @param parentNode Namehash of the parent name to check
    /// @param subnode Namehash of the subname to check
    function _checkCanCallSetSubnodeOwner(
        bytes32 parentNode,
        bytes32 subnode
    ) internal view {
        (
            address subnodeOwner,
            uint32 subnodeFuses,
            uint64 subnodeExpiry
        ) = getData(uint256(subnode));

        // check if the registry owner is 0 and expired
        // check if the wrapper owner is 0 and expired
        // If either, then check parent fuses for CANNOT_CREATE_SUBDOMAIN
        bool expired = subnodeExpiry < block.timestamp;
        if (
            expired &&
            // protects a name that has been unwrapped with PCC and doesn't allow the parent to take control by recreating it if unexpired
            (subnodeOwner == address(0) ||
                // protects a name that has been burnt and doesn't allow the parent to take control by recreating it if unexpired
                ens.owner(subnode) == address(0))
        ) {
            (, uint32 parentFuses, ) = getData(uint256(parentNode));
            if (parentFuses & CANNOT_CREATE_SUBDOMAIN != 0) {
                revert OperationProhibited(subnode);
            }
        } else {
            if (subnodeFuses & PARENT_CANNOT_CONTROL != 0) {
                revert OperationProhibited(subnode);
            }
        }
    }

    /// @notice Checks all Fuses in the mask are burned for the node
    /// @param node Namehash of the name
    /// @param fuseMask The fuses you want to check
    /// @return Boolean of whether or not all the selected fuses are burned
    function allFusesBurned(
        bytes32 node,
        uint32 fuseMask
    ) public view returns (bool) {
        (, uint32 fuses, ) = getData(uint256(node));
        return fuses & fuseMask == fuseMask;
    }

    /// @notice Checks if a name is wrapped
    /// @param node Namehash of the name
    /// @return Boolean of whether or not the name is wrapped
    function isWrapped(bytes32 node) public view returns (bool) {
        bytes memory name = names[node];
        if (name.length == 0) {
            return false;
        }
        (bytes32 labelhash, uint256 offset) = name.readLabel(0);
        bytes32 parentNode = name.namehash(offset);
        return isWrapped(parentNode, labelhash);
    }

    /// @notice Checks if a name is wrapped in a more gas efficient way
    /// @param parentNode Namehash of the name
    /// @param labelhash Namehash of the name
    /// @return Boolean of whether or not the name is wrapped
    function isWrapped(
        bytes32 parentNode,
        bytes32 labelhash
    ) public view returns (bool) {
        bytes32 node = _makeNode(parentNode, labelhash);
        bool wrapped = _isWrapped(node);
        if (parentNode != ETH_NODE) {
            return wrapped;
        }
        try registrar.ownerOf(uint256(labelhash)) returns (address owner) {
            return owner == address(this);
        } catch {
            return false;
        }
    }

    function onERC721Received(
        address to,
        address,
        uint256 tokenId,
        bytes calldata data
    ) public returns (bytes4) {
        //check if it's the eth registrar ERC721
        if (msg.sender != address(registrar)) {
            revert IncorrectTokenType();
        }

        (
            string memory label,
            address owner,
            uint16 ownerControlledFuses,
            address resolver
        ) = abi.decode(data, (string, address, uint16, address));

        bytes32 labelhash = bytes32(tokenId);
        bytes32 labelhashFromData = keccak256(bytes(label));

        if (labelhashFromData != labelhash) {
            revert LabelMismatch(labelhashFromData, labelhash);
        }

        // transfer the ens record back to the new owner (this contract)
        registrar.reclaim(uint256(labelhash), address(this));

        uint64 expiry = uint64(registrar.nameExpires(tokenId)) + GRACE_PERIOD;

        _wrapETH2LD(label, owner, ownerControlledFuses, expiry, resolver);

        return IERC721Receiver(to).onERC721Received.selector;
    }

    /***** Internal functions */

    function _beforeTransfer(
        uint256 id,
        uint32 fuses,
        uint64 expiry
    ) internal override {
        // For this check, treat .eth 2LDs as expiring at the start of the grace period.
        if (fuses & IS_DOT_ETH == IS_DOT_ETH) {
            expiry -= GRACE_PERIOD;
        }

        if (expiry < block.timestamp) {
            // Transferable if the name was not emancipated
            if (fuses & PARENT_CANNOT_CONTROL != 0) {
                revert("ERC1155: insufficient balance for transfer");
            }
        } else {
            // Transferable if CANNOT_TRANSFER is unburned
            if (fuses & CANNOT_TRANSFER != 0) {
                revert OperationProhibited(bytes32(id));
            }
        }

        // delete token approval if CANNOT_APPROVE has not been burnt
        if (fuses & CANNOT_APPROVE == 0) {
            delete _tokenApprovals[id];
        }
    }

    function _clearOwnerAndFuses(
        address owner,
        uint32 fuses,
        uint64 expiry
    ) internal view override returns (address, uint32) {
        if (expiry < block.timestamp) {
            if (fuses & PARENT_CANNOT_CONTROL == PARENT_CANNOT_CONTROL) {
                owner = address(0);
            }
            fuses = 0;
        }

        return (owner, fuses);
    }

    function _makeNode(
        bytes32 node,
        bytes32 labelhash
    ) private pure returns (bytes32) {
        return keccak256(abi.encodePacked(node, labelhash));
    }

    function _addLabel(
        string memory label,
        bytes memory name
    ) internal pure returns (bytes memory ret) {
        if (bytes(label).length < 1) {
            revert LabelTooShort();
        }
        if (bytes(label).length > 255) {
            revert LabelTooLong(label);
        }
        return abi.encodePacked(uint8(bytes(label).length), label, name);
    }

    function _mint(
        bytes32 node,
        address owner,
        uint32 fuses,
        uint64 expiry
    ) internal override {
        _canFusesBeBurned(node, fuses);
        (address oldOwner, , ) = super.getData(uint256(node));
        if (oldOwner != address(0)) {
            // burn and unwrap old token of old owner
            _burn(uint256(node));
            emit NameUnwrapped(node, address(0));
        }
        super._mint(node, owner, fuses, expiry);
    }

    function _wrap(
        bytes32 node,
        bytes memory name,
        address wrappedOwner,
        uint32 fuses,
        uint64 expiry
    ) internal {
        _mint(node, wrappedOwner, fuses, expiry);
        emit NameWrapped(node, name, wrappedOwner, fuses, expiry);
    }

    function _storeNameAndWrap(
        bytes32 parentNode,
        bytes32 node,
        string memory label,
        address owner,
        uint32 fuses,
        uint64 expiry
    ) internal {
        bytes memory name = _addLabel(label, names[parentNode]);
        _wrap(node, name, owner, fuses, expiry);
    }

    function _saveLabel(
        bytes32 parentNode,
        bytes32 node,
        string memory label
    ) internal returns (bytes memory) {
        bytes memory name = _addLabel(label, names[parentNode]);
        names[node] = name;
        return name;
    }

    function _updateName(
        bytes32 parentNode,
        bytes32 node,
        string memory label,
        address owner,
        uint32 fuses,
        uint64 expiry
    ) internal {
        (address oldOwner, uint32 oldFuses, uint64 oldExpiry) = getData(
            uint256(node)
        );
        bytes memory name = _addLabel(label, names[parentNode]);
        if (names[node].length == 0) {
            names[node] = name;
        }
        _setFuses(node, oldOwner, oldFuses | fuses, oldExpiry, expiry);
        if (owner == address(0)) {
            _unwrap(node, address(0));
        } else {
            _transfer(oldOwner, owner, uint256(node), 1, "");
        }
    }

    // wrapper function for stack limit
    function _checkParentFusesAndExpiry(
        bytes32 parentNode,
        bytes32 node,
        uint32 fuses,
        uint64 expiry
    ) internal view returns (uint64) {
        (, , uint64 oldExpiry) = getData(uint256(node));
        (, uint32 parentFuses, uint64 maxExpiry) = getData(uint256(parentNode));
        _checkParentFuses(node, fuses, parentFuses);
        return _normaliseExpiry(expiry, oldExpiry, maxExpiry);
    }

    function _checkParentFuses(
        bytes32 node,
        uint32 fuses,
        uint32 parentFuses
    ) internal pure {
        bool isBurningParentControlledFuses = fuses & PARENT_CONTROLLED_FUSES !=
            0;

        bool parentHasNotBurnedCU = parentFuses & CANNOT_UNWRAP == 0;

        if (isBurningParentControlledFuses && parentHasNotBurnedCU) {
            revert OperationProhibited(node);
        }
    }

    function _normaliseExpiry(
        uint64 expiry,
        uint64 oldExpiry,
        uint64 maxExpiry
    ) private pure returns (uint64) {
        // Expiry cannot be more than maximum allowed
        // .eth names will check registrar, non .eth check parent
        if (expiry > maxExpiry) {
            expiry = maxExpiry;
        }
        // Expiry cannot be less than old expiry
        if (expiry < oldExpiry) {
            expiry = oldExpiry;
        }

        return expiry;
    }

    function _wrapETH2LD(
        string memory label,
        address wrappedOwner,
        uint32 fuses,
        uint64 expiry,
        address resolver
    ) private {
        bytes32 labelhash = keccak256(bytes(label));
        bytes32 node = _makeNode(ETH_NODE, labelhash);
        // hardcode dns-encoded eth string for gas savings
        bytes memory name = _addLabel(label, "\x03eth\x00");
        names[node] = name;

        _wrap(
            node,
            name,
            wrappedOwner,
            fuses | PARENT_CANNOT_CONTROL | IS_DOT_ETH,
            expiry
        );

        if (resolver != address(0)) {
            ens.setResolver(node, resolver);
        }
    }

    function _unwrap(bytes32 node, address owner) private {
        if (allFusesBurned(node, CANNOT_UNWRAP)) {
            revert OperationProhibited(node);
        }

        // Burn token and fuse data
        _burn(uint256(node));
        ens.setOwner(node, owner);

        emit NameUnwrapped(node, owner);
    }

    function _setFuses(
        bytes32 node,
        address owner,
        uint32 fuses,
        uint64 oldExpiry,
        uint64 expiry
    ) internal {
        _setData(node, owner, fuses, expiry);
        emit FusesSet(node, fuses);
        if (expiry > oldExpiry) {
            emit ExpiryExtended(node, expiry);
        }
    }

    function _setData(
        bytes32 node,
        address owner,
        uint32 fuses,
        uint64 expiry
    ) internal {
        _canFusesBeBurned(node, fuses);
        super._setData(uint256(node), owner, fuses, expiry);
    }

    function _canFusesBeBurned(bytes32 node, uint32 fuses) internal pure {
        // If a non-parent controlled fuse is being burned, check PCC and CU are burnt
        if (
            fuses & ~PARENT_CONTROLLED_FUSES != 0 &&
            fuses & (PARENT_CANNOT_CONTROL | CANNOT_UNWRAP) !=
            (PARENT_CANNOT_CONTROL | CANNOT_UNWRAP)
        ) {
            revert OperationProhibited(node);
        }
    }

    function _checkFusesAreSettable(bytes32 node, uint32 fuses) internal pure {
        if (fuses | USER_SETTABLE_FUSES != USER_SETTABLE_FUSES) {
            // Cannot directly burn other non-user settable fuses
            revert OperationProhibited(node);
        }
    }

    function _isWrapped(bytes32 node) internal view returns (bool) {
        return
            ownerOf(uint256(node)) != address(0) &&
            ens.owner(node) == address(this);
    }

    function _isETH2LDInGracePeriod(
        uint32 fuses,
        uint64 expiry
    ) internal view returns (bool) {
        return
            fuses & IS_DOT_ETH == IS_DOT_ETH &&
            expiry - GRACE_PERIOD < block.timestamp;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {Hashing, Types} from '../../lib/optimism/packages/contracts-bedrock/src/libraries/Hashing.sol';

interface IOPOutputFinder {
    function findOutputIndex(address portal, uint256 minAgeSec) external view returns (uint256);
    function getOutput(
        address portal,
        uint256 outputIndex
    ) external view returns (Types.OutputProposal memory);
}

contract OPVerifier is AbstractVerifier {
    address immutable _portal;
    IOPOutputFinder immutable _outputFinder;
    uint256 immutable _minAgeSec;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        address portal,
		IOPOutputFinder outputFinder,
        uint256 minAgeSec
    ) AbstractVerifier(urls, window, hooks) {
        _portal = portal;
		_outputFinder = outputFinder;
        _minAgeSec = minAgeSec;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_outputFinder.findOutputIndex(_portal, _minAgeSec));
    }

    struct GatewayProof {
        uint256 outputIndex;
        Types.OutputRootProof outputRootProof;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 outputIndex1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        Types.OutputProposal memory output = _outputFinder.getOutput(
            _portal, 
            p.outputIndex
        );
        if (p.outputIndex != outputIndex1) {
            Types.OutputProposal memory output1 = _outputFinder.getOutput(
                _portal, outputIndex1
            );
            _checkWindow(output1.timestamp, output.timestamp);
            // NOTE: no addtional checks are required
            // newer outputs will fail window check
            // older outputs will be older (by definition)
            // therefore, older finalized outputs are also finalized 
        }
        bytes32 computedRoot = Hashing.hashOutputRootProof(p.outputRootProof);
        require(computedRoot == output.outputRoot, 'OP: invalid root');
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(
                    0,
                    p.outputRootProof.stateRoot,
                    p.proofs,
                    p.order,
                    _hooks
                )
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';

// https://github.com/taikoxyz/taiko-mono/blob/6e5b8c800c0128ecf080567dc6daadd75c79319d/packages/protocol/contracts/layer1/based/ITaikoInbox.sol

struct TransitionState {
    bytes32 parentHash;
    bytes32 blockHash;
    bytes32 stateRoot;
    address prover;
    bool inProvingWindow;
    uint48 createdAt;
}

interface ITaiko {
    function getTransitionById(
        uint64 blockId,
        uint24 tid
    ) external view returns (TransitionState memory);
    function getLastSyncedTransition()
        external
        view
        returns (uint64 batchId, uint64 blockId, TransitionState memory ts);
}

contract TaikoVerifier is AbstractVerifier {
    ITaiko immutable _rollup;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        ITaiko rollup
    ) AbstractVerifier(urls, window, hooks) {
        _rollup = rollup;
    }

    function getLatestContext() external view returns (bytes memory) {
        (uint64 batchId, , TransitionState memory ts) = _rollup
            .getLastSyncedTransition();
        return abi.encode(batchId, ts.createdAt);
    }

    struct GatewayProof {
        uint64 batchId;
        uint24 tid;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        (, uint256 createdAt) = abi.decode(context, (uint64, uint48));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        TransitionState memory ts = _rollup.getTransitionById(p.batchId, p.tid); // reverts if invalid
        _checkWindow(createdAt, ts.createdAt);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, ts.stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {ArbitrumVerifier, IVerifierHooks} from './ArbitrumVerifier.sol';
import {NitroVerifierLib} from './NitroVerifierLib.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';

contract DoubleArbitrumVerifier is ArbitrumVerifier {
    GatewayRequest public request;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        address rollup12,
        uint256 minAgeBlocks12,
        bool isBoLD12,
        GatewayRequest memory _request
    )
        ArbitrumVerifier(
            urls,
            window,
            hooks,
            rollup12,
            minAgeBlocks12,
            isBoLD12
        )
    {
        request = _request;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view override returns (bytes[] memory, uint8 exitCode) {
        GatewayProof[2] memory ps = abi.decode(proof, (GatewayProof[2]));
        bytes32 stateRoot = _verifyRollup(ps[0], context);
        (bytes[] memory outputs, ) = GatewayVM.evalRequest(
            request,
            ProofSequence(0, stateRoot, ps[0].proofs, ps[0].order, _hooks)
        );
        // outputs[0] = node
        // outputs[1] = confirmData
        // outputs[2] = createdAtBlock (not used yet)
        NitroVerifierLib.RollupProof memory p = abi.decode(
            ps[1].rollupProof,
            (NitroVerifierLib.RollupProof)
        );
        stateRoot = NitroVerifierLib.verifyStateRoot(p, bytes32(outputs[1]));
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, ps[1].proofs, ps[1].order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {AbstractReverseResolver} from "./AbstractReverseResolver.sol";
import {ENS} from "../registry/ENS.sol";
import {INameResolver} from "../resolvers/profiles/INameResolver.sol";
import {IStandaloneReverseRegistrar} from "../reverseRegistrar/IStandaloneReverseRegistrar.sol";
import {INameReverser} from "./INameReverser.sol";
import {COIN_TYPE_ETH} from "../utils/ENSIP19.sol";
import {NameCoder} from "../utils/NameCoder.sol";
import {HexUtils} from "../utils/HexUtils.sol";
import {LibABI} from "../utils/LibABI.sol";

/// @title Ethereum Reverse Resolver
/// @notice Reverses an EVM address using the first non-null response from the following sources:
///
/// 1. `IStandaloneReverseRegistrar` for "addr.reverse"
/// 2. `name()` from "{addr}.addr.reverse" in V1 Registry
/// 3. `IStandaloneReverseRegistrar` for "default.reverse"
///
contract ETHReverseResolver is AbstractReverseResolver {
    /// @dev Namehash of "addr.reverse"
    bytes32 constant ADDR_REVERSE_NODE =
        0x91d1777781884d03a6757a803996e38de2a42967fb37eeaca72729271025a9e2;

    /// @notice The ENS registry contract.
    ENS immutable ens;

    /// @notice The reverse registrar contract for "default.reverse".
    IStandaloneReverseRegistrar public immutable defaultRegistrar;

    constructor(
        ENS _ens,
        IStandaloneReverseRegistrar addrRegistrar,
        IStandaloneReverseRegistrar _defaultRegistrar
    ) AbstractReverseResolver(COIN_TYPE_ETH, address(addrRegistrar)) {
        ens = _ens;
        defaultRegistrar = _defaultRegistrar;
    }

    /// @inheritdoc AbstractReverseResolver
    function _resolveName(
        address addr
    ) internal view override returns (string memory name) {
        name = IStandaloneReverseRegistrar(chainRegistrar).nameForAddr(addr);
        if (bytes(name).length > 0) {
            return name;
        }
        bytes32 node = NameCoder.namehash(
            ADDR_REVERSE_NODE,
            keccak256(bytes(HexUtils.addressToHex(addr)))
        );
        address resolver = ens.resolver(node);
        if (resolver != address(0)) {
            // note: this only supports onchain direct calls (no extended, no offchain)
            (bool ok, bytes memory v) = resolver.staticcall{gas: 100_000}(
                abi.encodeCall(INameResolver.name, (node))
            );
            if (ok) {
                (ok, v) = LibABI.tryDecodeBytes(v);
            }
            if (!ok) {
                return ""; // terminate on revert or decode failure
            }
            if (v.length > 0) {
                return string(v);
            }
        }
        return defaultRegistrar.nameForAddr(addr);
    }

    /// @inheritdoc INameReverser
    function resolveNames(
        address[] memory addrs
    ) external view returns (string[] memory names) {
        names = new string[](addrs.length);
        for (uint256 i; i < addrs.length; ++i) {
            names[i] = _resolveName(addrs[i]);
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

import "../ResolverBase.sol";
import "./INameResolver.sol";

abstract contract NameResolver is INameResolver, ResolverBase {
    mapping(uint64 => mapping(bytes32 => string)) versionable_names;

    /// Sets the name associated with an ENS node, for reverse records.
    /// May only be called by the owner of that node in the ENS registry.
    /// @param node The node to update.
    function setName(
        bytes32 node,
        string calldata newName
    ) external virtual authorised(node) {
        versionable_names[recordVersions[node]][node] = newName;
        emit NameChanged(node, newName);
    }

    /// Returns the name associated with an ENS node, for reverse records.
    /// Defined in EIP181.
    /// @param node The ENS node to query.
    /// @return The associated name.
    function name(
        bytes32 node
    ) external view virtual override returns (string memory) {
        return versionable_names[recordVersions[node]][node];
    }

    function supportsInterface(
        bytes4 interfaceID
    ) public view virtual override returns (bool) {
        return
            interfaceID == type(INameResolver).interfaceId ||
            super.supportsInterface(interfaceID);
    }
}

//SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

import "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import "./profiles/IABIResolver.sol";
import "./profiles/IAddressResolver.sol";
import "./profiles/IAddrResolver.sol";
import "./profiles/IContentHashResolver.sol";
import "./profiles/IDNSRecordResolver.sol";
import "./profiles/IDNSZoneResolver.sol";
import "./profiles/IInterfaceResolver.sol";
import "./profiles/INameResolver.sol";
import "./profiles/IPubkeyResolver.sol";
import "./profiles/ITextResolver.sol";
import "./profiles/IExtendedResolver.sol";

/// A generic resolver interface which includes all the functions including the ones deprecated
interface Resolver is
    IERC165,
    IABIResolver,
    IAddressResolver,
    IAddrResolver,
    IContentHashResolver,
    IDNSRecordResolver,
    IDNSZoneResolver,
    IInterfaceResolver,
    INameResolver,
    IPubkeyResolver,
    ITextResolver,
    IExtendedResolver
{
    /* Deprecated events */
    event ContentChanged(bytes32 indexed node, bytes32 hash);

    function setApprovalForAll(address, bool) external;

    function approve(bytes32 node, address delegate, bool approved) external;

    function isApprovedForAll(address account, address operator) external;

    function isApprovedFor(
        address owner,
        bytes32 node,
        address delegate
    ) external;

    function setABI(
        bytes32 node,
        uint256 contentType,
        bytes calldata data
    ) external;

    function setAddr(bytes32 node, address addr) external;

    function setAddr(bytes32 node, uint256 coinType, bytes calldata a) external;

    function setContenthash(bytes32 node, bytes calldata hash) external;

    function setDnsrr(bytes32 node, bytes calldata data) external;

    function setName(bytes32 node, string calldata _name) external;

    function setPubkey(bytes32 node, bytes32 x, bytes32 y) external;

    function setText(
        bytes32 node,
        string calldata key,
        string calldata value
    ) external;

    function setInterface(
        bytes32 node,
        bytes4 interfaceID,
        address implementer
    ) external;

    function multicall(
        bytes[] calldata data
    ) external returns (bytes[] memory results);

    function multicallWithNodeCheck(
        bytes32 nodehash,
        bytes[] calldata data
    ) external returns (bytes[] memory results);

    /* Deprecated functions */
    function content(bytes32 node) external view returns (bytes32);

    function multihash(bytes32 node) external view returns (bytes memory);

    function setContent(bytes32 node, bytes32 hash) external;

    function setMultihash(bytes32 node, bytes calldata hash) external;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {ILineaRollup} from './ILineaRollup.sol';

contract LineaVerifier is AbstractVerifier {
    ILineaRollup immutable _rollup;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        ILineaRollup rollup
    ) AbstractVerifier(urls, window, hooks) {
        _rollup = rollup;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_rollup.currentL2BlockNumber());
    }

    struct GatewayProof {
        uint256 l2BlockNumber;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 l2BlockNumber1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        _checkWindow(l2BlockNumber1, p.l2BlockNumber);
        bytes32 stateRoot = _rollup.stateRootHashes(p.l2BlockNumber);
        if (stateRoot == bytes32(0)) revert('Linea: not finalized');
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {RLPReader, RLPReaderExt} from '../RLPReaderExt.sol';

interface IRollup {
    function stateRoot() external view returns (bytes32);
    function stateBlockNumber() external view returns (uint256);
}

// https://github.com/starkware-libs/cairo-lang/blob/master/src/starkware/starknet/solidity/Starknet.sol#L60
uint256 constant SLOT_STATE_ROOT = uint256(
    keccak256('STARKNET_1.0_INIT_STARKNET_STATE_STRUCT')
);

contract StarknetVerifier is AbstractVerifier {
    IRollup immutable _rollup;
    IVerifierHooks immutable _ethHooks;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IRollup rollup,
        IVerifierHooks ethHooks
    ) AbstractVerifier(urls, window, hooks) {
        _rollup = rollup;
        _ethHooks = ethHooks;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_rollup.stateBlockNumber());
    }

    struct GatewayProof {
        uint256 blockNumber;
        bytes rlpEncodedL1Block;
        bytes accountProof;
        bytes storageProof;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 blockNumber1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        bytes32 stateRoot;
        if (blockNumber1 == p.blockNumber) {
            stateRoot = _rollup.stateRoot();
        } else {
            //_checkWindow(blockNumber1, p.blockNumber);
            RLPReader.RLPItem[] memory v = RLPReader.readList(
                p.rlpEncodedL1Block
            );
            bytes32 blockHash = blockhash(
                uint256(RLPReaderExt.bytes32FromRLP(v[8]))
            );
            require(
                blockHash == keccak256(p.rlpEncodedL1Block),
                'Starknet: blockhash'
            );
            stateRoot = RLPReaderExt.strictBytes32FromRLP(v[3]);
            bytes32 storageRoot = _ethHooks.verifyAccountState(
                stateRoot,
                address(_rollup),
                p.accountProof
            );
            stateRoot = _ethHooks.verifyStorageValue(
                storageRoot,
                address(_rollup),
                SLOT_STATE_ROOT,
                p.storageProof
            );
        }
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

pragma solidity >=0.8.4;

import "./ENS.sol";

/// The ENS registry contract.
contract ENSRegistry is ENS {
    struct Record {
        address owner;
        address resolver;
        uint64 ttl;
    }

    mapping(bytes32 => Record) records;
    mapping(address => mapping(address => bool)) operators;

    // Permits modifications only by the owner of the specified node.
    modifier authorised(bytes32 node) {
        address owner = records[node].owner;
        require(owner == msg.sender || operators[owner][msg.sender]);
        _;
    }

    /// @dev Constructs a new ENS registry.
    constructor() public {
        records[0x0].owner = msg.sender;
    }

    /// @dev Sets the record for a node.
    /// @param node The node to update.
    /// @param owner The address of the new owner.
    /// @param resolver The address of the resolver.
    /// @param ttl The TTL in seconds.
    function setRecord(
        bytes32 node,
        address owner,
        address resolver,
        uint64 ttl
    ) external virtual override {
        setOwner(node, owner);
        _setResolverAndTTL(node, resolver, ttl);
    }

    /// @dev Sets the record for a subnode.
    /// @param node The parent node.
    /// @param label The hash of the label specifying the subnode.
    /// @param owner The address of the new owner.
    /// @param resolver The address of the resolver.
    /// @param ttl The TTL in seconds.
    function setSubnodeRecord(
        bytes32 node,
        bytes32 label,
        address owner,
        address resolver,
        uint64 ttl
    ) external virtual override {
        bytes32 subnode = setSubnodeOwner(node, label, owner);
        _setResolverAndTTL(subnode, resolver, ttl);
    }

    /// @dev Transfers ownership of a node to a new address. May only be called by the current owner of the node.
    /// @param node The node to transfer ownership of.
    /// @param owner The address of the new owner.
    function setOwner(
        bytes32 node,
        address owner
    ) public virtual override authorised(node) {
        _setOwner(node, owner);
        emit Transfer(node, owner);
    }

    /// @dev Transfers ownership of a subnode keccak256(node, label) to a new address. May only be called by the owner of the parent node.
    /// @param node The parent node.
    /// @param label The hash of the label specifying the subnode.
    /// @param owner The address of the new owner.
    function setSubnodeOwner(
        bytes32 node,
        bytes32 label,
        address owner
    ) public virtual override authorised(node) returns (bytes32) {
        bytes32 subnode = keccak256(abi.encodePacked(node, label));
        _setOwner(subnode, owner);
        emit NewOwner(node, label, owner);
        return subnode;
    }

    /// @dev Sets the resolver address for the specified node.
    /// @param node The node to update.
    /// @param resolver The address of the resolver.
    function setResolver(
        bytes32 node,
        address resolver
    ) public virtual override authorised(node) {
        emit NewResolver(node, resolver);
        records[node].resolver = resolver;
    }

    /// @dev Sets the TTL for the specified node.
    /// @param node The node to update.
    /// @param ttl The TTL in seconds.
    function setTTL(
        bytes32 node,
        uint64 ttl
    ) public virtual override authorised(node) {
        emit NewTTL(node, ttl);
        records[node].ttl = ttl;
    }

    /// @dev Enable or disable approval for a third party ("operator") to manage
    ///      all of `msg.sender`'s ENS records. Emits the ApprovalForAll event.
    /// @param operator Address to add to the set of authorized operators.
    /// @param approved True if the operator is approved, false to revoke approval.
    function setApprovalForAll(
        address operator,
        bool approved
    ) external virtual override {
        operators[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }

    /// @dev Returns the address that owns the specified node.
    /// @param node The specified node.
    /// @return address of the owner.
    function owner(
        bytes32 node
    ) public view virtual override returns (address) {
        address addr = records[node].owner;
        if (addr == address(this)) {
            return address(0x0);
        }

        return addr;
    }

    /// @dev Returns the address of the resolver for the specified node.
    /// @param node The specified node.
    /// @return address of the resolver.
    function resolver(
        bytes32 node
    ) public view virtual override returns (address) {
        return records[node].resolver;
    }

    /// @dev Returns the TTL of a node, and any records associated with it.
    /// @param node The specified node.
    /// @return ttl of the node.
    function ttl(bytes32 node) public view virtual override returns (uint64) {
        return records[node].ttl;
    }

    /// @dev Returns whether a record has been imported to the registry.
    /// @param node The specified node.
    /// @return Bool if record exists
    function recordExists(
        bytes32 node
    ) public view virtual override returns (bool) {
        return records[node].owner != address(0x0);
    }

    /// @dev Query if an address is an authorized operator for another address.
    /// @param owner The address that owns the records.
    /// @param operator The address that acts on behalf of the owner.
    /// @return True if `operator` is an approved operator for `owner`, false otherwise.
    function isApprovedForAll(
        address owner,
        address operator
    ) external view virtual override returns (bool) {
        return operators[owner][operator];
    }

    function _setOwner(bytes32 node, address owner) internal virtual {
        records[node].owner = owner;
    }

    function _setResolverAndTTL(
        bytes32 node,
        address resolver,
        uint64 ttl
    ) internal {
        if (resolver != records[node].resolver) {
            records[node].resolver = resolver;
            emit NewResolver(node, resolver);
        }

        if (ttl != records[node].ttl) {
            records[node].ttl = ttl;
            emit NewTTL(node, ttl);
        }
    }
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

