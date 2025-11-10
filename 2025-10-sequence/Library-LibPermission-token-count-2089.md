
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { LibBytes } from "../../../utils/LibBytes.sol";

/// @notice Permission for a specific session signer
/// @param target Address of the target contract this permission applies to
/// @param rules Array of parameter rules
struct Permission {
  address target;
  ParameterRule[] rules;
}

/// @notice Parameter operation for a specific session signer
enum ParameterOperation {
  EQUAL,
  NOT_EQUAL,
  GREATER_THAN_OR_EQUAL,
  LESS_THAN_OR_EQUAL
}

/// @notice Parameter rule for a specific session signer
/// @param cumulative If the value should accumulate over multiple calls
/// @param operation Operation to apply to the parameter
/// @param value Value to compare against the masked parameter
/// @param offset Offset in calldata to read the parameter
/// @param mask Mask to apply to the parameter
struct ParameterRule {
  bool cumulative;
  ParameterOperation operation;
  bytes32 value;
  uint256 offset;
  bytes32 mask;
}

/// @notice Usage limit for a specific session signer
/// @param usageHash Usage identifier
/// @param usageAmount Amount of usage
struct UsageLimit {
  bytes32 usageHash;
  uint256 usageAmount;
}

using LibBytes for bytes;

/// @title LibPermission
/// @author Michael Standen
/// @notice Library for permission management
library LibPermission {

  /// @notice Error thrown when the rules length exceeds the maximum
  error RulesLengthExceedsMax();

  /// @notice Reads a permission from a packed bytes array
  /// @param encoded The packed bytes array
  /// @param pointer The pointer to the start of the permission
  /// @return permission The decoded permission
  /// @return newPointer The new pointer to the end of the permission
  function readPermission(
    bytes calldata encoded,
    uint256 pointer
  ) internal pure returns (Permission memory permission, uint256 newPointer) {
    // Target
    (permission.target, pointer) = encoded.readAddress(pointer);
    // Rules
    uint256 rulesLength;
    (rulesLength, pointer) = encoded.readUint8(pointer);
    permission.rules = new ParameterRule[](rulesLength);
    for (uint256 i = 0; i < rulesLength; i++) {
      uint8 operationCumulative;
      (operationCumulative, pointer) = encoded.readUint8(pointer);
      // 000X: cumulative
      permission.rules[i].cumulative = operationCumulative & 1 != 0;
      // XXX0: operation
      permission.rules[i].operation = ParameterOperation(operationCumulative >> 1);

      (permission.rules[i].value, pointer) = encoded.readBytes32(pointer);
      (permission.rules[i].offset, pointer) = encoded.readUint256(pointer);
      (permission.rules[i].mask, pointer) = encoded.readBytes32(pointer);
    }
    return (permission, pointer);
  }

  /// @notice Encodes a permission into a packed bytes array
  /// @param permission The permission to encode
  /// @return packed The packed bytes array
  function toPacked(
    Permission calldata permission
  ) internal pure returns (bytes memory packed) {
    if (permission.rules.length > type(uint8).max) {
      revert RulesLengthExceedsMax();
    }
    packed = abi.encodePacked(permission.target, uint8(permission.rules.length));
    for (uint256 i = 0; i < permission.rules.length; i++) {
      packed = abi.encodePacked(packed, ruleToPacked(permission.rules[i]));
    }
  }

  /// @notice Encodes a rule into a packed bytes array
  /// @param rule The rule to encode
  /// @return packed The packed bytes array
  function ruleToPacked(
    ParameterRule calldata rule
  ) internal pure returns (bytes memory packed) {
    // Combine operation and cumulative flag into a single byte
    // 0x[operationx3][cumulative]
    uint8 operationCumulative = (uint8(rule.operation) << 1) | (rule.cumulative ? 1 : 0);

    return abi.encodePacked(operationCumulative, rule.value, rule.offset, rule.mask);
  }

}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.18;

/// @title Library for reading data from bytes arrays
/// @author Agustin Aguilar (aa@horizon.io), Michael Standen (mstan@horizon.io)
/// @notice This library contains functions for reading data from bytes arrays.
/// @dev These functions do not check if the input index is within the bounds of the data array.
/// @dev Reading out of bounds may return dirty values.
library LibBytes {

  function readFirstUint8(
    bytes calldata _data
  ) internal pure returns (uint8 a, uint256 newPointer) {
    assembly {
      let word := calldataload(_data.offset)
      a := shr(248, word)
      newPointer := 1
    }
  }

  function readUint8(bytes calldata _data, uint256 _index) internal pure returns (uint8 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := shr(248, word)
      newPointer := add(_index, 1)
    }
  }

  function readUint16(bytes calldata _data, uint256 _index) internal pure returns (uint16 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := shr(240, word)
      newPointer := add(_index, 2)
    }
  }

  function readUint24(bytes calldata _data, uint256 _index) internal pure returns (uint24 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := shr(232, word)
      newPointer := add(_index, 3)
    }
  }

  function readUint64(bytes calldata _data, uint256 _index) internal pure returns (uint64 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := shr(192, word)
      newPointer := add(_index, 8)
    }
  }

  function readUint160(bytes calldata _data, uint256 _index) internal pure returns (uint160 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := shr(96, word)
      newPointer := add(_index, 20)
    }
  }

  function readUint256(bytes calldata _data, uint256 _index) internal pure returns (uint256 a, uint256 newPointer) {
    assembly {
      a := calldataload(add(_index, _data.offset))
      newPointer := add(_index, 32)
    }
  }

  function readUintX(
    bytes calldata _data,
    uint256 _index,
    uint256 _length
  ) internal pure returns (uint256 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      let shift := sub(256, mul(_length, 8))
      a := and(shr(shift, word), sub(shl(mul(8, _length), 1), 1))
      newPointer := add(_index, _length)
    }
  }

  function readBytes4(bytes calldata _data, uint256 _pointer) internal pure returns (bytes4 a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_pointer, _data.offset))
      a := and(word, 0xffffffff00000000000000000000000000000000000000000000000000000000)
      newPointer := add(_pointer, 4)
    }
  }

  function readBytes32(bytes calldata _data, uint256 _pointer) internal pure returns (bytes32 a, uint256 newPointer) {
    assembly {
      a := calldataload(add(_pointer, _data.offset))
      newPointer := add(_pointer, 32)
    }
  }

  function readAddress(bytes calldata _data, uint256 _index) internal pure returns (address a, uint256 newPointer) {
    assembly {
      let word := calldataload(add(_index, _data.offset))
      a := and(shr(96, word), 0xffffffffffffffffffffffffffffffffffffffff)
      newPointer := add(_index, 20)
    }
  }

  /// @dev ERC-2098 Compact Signature
  function readRSVCompact(
    bytes calldata _data,
    uint256 _index
  ) internal pure returns (bytes32 r, bytes32 s, uint8 v, uint256 newPointer) {
    uint256 yParityAndS;
    assembly {
      r := calldataload(add(_index, _data.offset))
      yParityAndS := calldataload(add(_index, add(_data.offset, 32)))
      newPointer := add(_index, 64)
    }
    uint256 yParity = uint256(yParityAndS >> 255);
    s = bytes32(uint256(yParityAndS) & ((1 << 255) - 1));
    v = uint8(yParity) + 27;
  }

}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

