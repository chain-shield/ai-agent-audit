
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../../../modules/Payload.sol";
import { LibBytes } from "../../../utils/LibBytes.sol";

import { SessionErrors } from "../SessionErrors.sol";
import { IExplicitSessionManager, SessionPermissions, SessionUsageLimits } from "./IExplicitSessionManager.sol";
import { Permission, UsageLimit } from "./Permission.sol";
import { PermissionValidator } from "./PermissionValidator.sol";

abstract contract ExplicitSessionManager is IExplicitSessionManager, PermissionValidator {

  using LibBytes for bytes;

  /// @notice Special address used for tracking native token value limits
  address public constant VALUE_TRACKING_ADDRESS = address(0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE);

  /// @inheritdoc IExplicitSessionManager
  function incrementUsageLimit(
    UsageLimit[] calldata limits
  ) external {
    address wallet = msg.sender;
    for (uint256 i = 0; i < limits.length; i++) {
      if (limits[i].usageAmount < getLimitUsage(wallet, limits[i].usageHash)) {
        // Cannot decrement usage limit
        revert SessionErrors.InvalidLimitUsageIncrement();
      }
      setLimitUsage(wallet, limits[i].usageHash, limits[i].usageAmount);
    }
  }

  /// @notice Validates an explicit call
  /// @param payload The decoded payload containing calls
  /// @param callIdx The index of the call to validate
  /// @param wallet The wallet's address
  /// @param sessionSigner The session signer's address
  /// @param allSessionPermissions All sessions' permissions
  /// @param permissionIdx The index of the permission to validate
  /// @param sessionUsageLimits The session usage limits
  /// @return newSessionUsageLimits The updated session usage limits
  function _validateExplicitCall(
    Payload.Decoded calldata payload,
    uint256 callIdx,
    address wallet,
    address sessionSigner,
    SessionPermissions[] memory allSessionPermissions,
    uint8 permissionIdx,
    SessionUsageLimits memory sessionUsageLimits
  ) internal view returns (SessionUsageLimits memory newSessionUsageLimits) {
    // Find the permissions for the given session signer
    SessionPermissions memory sessionPermissions;
    for (uint256 i = 0; i < allSessionPermissions.length; i++) {
      if (allSessionPermissions[i].signer == sessionSigner) {
        sessionPermissions = allSessionPermissions[i];
        break;
      }
    }
    if (sessionPermissions.signer == address(0)) {
      revert SessionErrors.InvalidSessionSigner(sessionSigner);
    }

    // Check if session chainId is valid
    if (sessionPermissions.chainId != 0 && sessionPermissions.chainId != block.chainid) {
      revert SessionErrors.InvalidChainId(sessionPermissions.chainId);
    }

    // Check if session has expired.
    if (sessionPermissions.deadline != 0 && block.timestamp > sessionPermissions.deadline) {
      revert SessionErrors.SessionExpired(sessionPermissions.deadline);
    }

    // Delegate calls are not allowed
    Payload.Call calldata call = payload.calls[callIdx];
    if (call.delegateCall) {
      revert SessionErrors.InvalidDelegateCall();
    }

    // Calls to incrementUsageLimit are the only allowed calls to this contract
    if (call.to == address(this)) {
      if (callIdx != 0) {
        // IncrementUsageLimit call is only allowed as the first call
        revert SessionErrors.InvalidLimitUsageIncrement();
      }
      if (call.value > 0) {
        revert SessionErrors.InvalidValue();
      }
      // No permissions required for the increment call
      return sessionUsageLimits;
    }

    // Get the permission for the current call
    if (permissionIdx >= sessionPermissions.permissions.length) {
      revert SessionErrors.MissingPermission();
    }
    Permission memory permission = sessionPermissions.permissions[permissionIdx];

    // Validate the permission for the current call
    (bool isValid, UsageLimit[] memory limits) =
      validatePermission(permission, call, wallet, sessionSigner, sessionUsageLimits.limits);
    if (!isValid) {
      revert SessionErrors.InvalidPermission();
    }
    sessionUsageLimits.limits = limits;

    // Increment the total value used
    if (call.value > 0) {
      sessionUsageLimits.totalValueUsed += call.value;
    }
    if (sessionUsageLimits.totalValueUsed > sessionPermissions.valueLimit) {
      // Value limit exceeded
      revert SessionErrors.InvalidValue();
    }

    return sessionUsageLimits;
  }

  /// @notice Verifies the limit usage increment
  /// @param call The first call in the payload, which is expected to be the increment call
  /// @param sessionUsageLimits The session usage limits
  /// @dev Reverts if the required increment call is missing or invalid
  /// @dev If no usage limits are used, this function does nothing
  function _validateLimitUsageIncrement(
    Payload.Call calldata call,
    SessionUsageLimits[] memory sessionUsageLimits
  ) internal view {
    // Limits call is only required if there are usage limits used
    if (sessionUsageLimits.length > 0) {
      // Verify the first call is the increment call and cannot be skipped
      if (call.to != address(this) || call.behaviorOnError != Payload.BEHAVIOR_REVERT_ON_ERROR || call.onlyFallback) {
        revert SessionErrors.InvalidLimitUsageIncrement();
      }

      // Construct expected limit increments
      uint256 totalLimitsLength = 0;
      for (uint256 i = 0; i < sessionUsageLimits.length; i++) {
        totalLimitsLength += sessionUsageLimits[i].limits.length;
        if (sessionUsageLimits[i].totalValueUsed > 0) {
          totalLimitsLength++;
        }
      }
      UsageLimit[] memory limits = new UsageLimit[](totalLimitsLength);
      uint256 limitIndex = 0;
      for (uint256 i = 0; i < sessionUsageLimits.length; i++) {
        for (uint256 j = 0; j < sessionUsageLimits[i].limits.length; j++) {
          limits[limitIndex++] = sessionUsageLimits[i].limits[j];
        }
        if (sessionUsageLimits[i].totalValueUsed > 0) {
          limits[limitIndex++] = UsageLimit({
            usageHash: keccak256(abi.encode(sessionUsageLimits[i].signer, VALUE_TRACKING_ADDRESS)),
            usageAmount: sessionUsageLimits[i].totalValueUsed
          });
        }
      }

      // Verify the increment call data
      bytes memory expectedData = abi.encodeWithSelector(this.incrementUsageLimit.selector, limits);
      bytes32 expectedDataHash = keccak256(expectedData);
      bytes32 actualDataHash = keccak256(call.data);
      if (actualDataHash != expectedDataHash) {
        revert SessionErrors.InvalidLimitUsageIncrement();
      }
    } else {
      // Do not allow self calls if there are no usage limits
      if (call.to == address(this)) {
        revert SessionErrors.InvalidLimitUsageIncrement();
      }
    }
  }

}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../../../modules/Payload.sol";
import { LibBytes } from "../../../utils/LibBytes.sol";
import { ParameterOperation, ParameterRule, Permission, UsageLimit } from "./Permission.sol";

/// @title PermissionValidator
/// @author Michael Standen, Agustin Aguilar
/// @notice Validates permissions for a given call
abstract contract PermissionValidator {

  using LibBytes for bytes;

  /// @notice Emitted when the usage amount for a given wallet and usage hash is updated
  event LimitUsageUpdated(address wallet, bytes32 usageHash, uint256 usageAmount);

  /// @notice Mapping of usage limit hashes to their usage amounts
  mapping(address => mapping(bytes32 => uint256)) private limitUsage;

  /// @notice Get the usage amount for a given usage hash and wallet
  /// @param wallet The wallet address
  /// @param usageHash The usage hash
  /// @return The usage amount
  function getLimitUsage(address wallet, bytes32 usageHash) public view returns (uint256) {
    return limitUsage[wallet][usageHash];
  }

  /// @notice Set the usage amount for a given usage hash and wallet
  /// @param wallet The wallet address
  /// @param usageHash The usage hash
  /// @param usageAmount The usage amount
  function setLimitUsage(address wallet, bytes32 usageHash, uint256 usageAmount) internal {
    limitUsage[wallet][usageHash] = usageAmount;
    emit LimitUsageUpdated(wallet, usageHash, usageAmount);
  }

  /// @notice Validates a rules permission
  /// @param permission The rules permission to validate
  /// @param call The call to validate against
  /// @param wallet The wallet address
  /// @param signer The signer address
  /// @param usageLimits Array of current usage limits
  /// @return True if the permission is valid, false otherwise
  /// @return newUsageLimits New array of usage limits
  function validatePermission(
    Permission memory permission,
    Payload.Call calldata call,
    address wallet,
    address signer,
    UsageLimit[] memory usageLimits
  ) public view returns (bool, UsageLimit[] memory newUsageLimits) {
    if (permission.target != call.to) {
      return (false, usageLimits);
    }

    // Copy usage limits into array with space for new rules
    newUsageLimits = new UsageLimit[](usageLimits.length + permission.rules.length);
    for (uint256 i = 0; i < usageLimits.length; i++) {
      newUsageLimits[i] = usageLimits[i];
    }
    uint256 actualLimitsCount = usageLimits.length;

    // Check each rule
    for (uint256 i = 0; i < permission.rules.length; i++) {
      ParameterRule memory rule = permission.rules[i];

      // Extract value from calldata at offset
      (bytes32 value,) = call.data.readBytes32(rule.offset);

      // Apply mask
      value = value & rule.mask;

      if (rule.cumulative) {
        // Calculate cumulative usage
        uint256 value256 = uint256(value);
        // Find the usage limit for the current rule
        bytes32 usageHash = keccak256(abi.encode(signer, permission, i));
        uint256 previousUsage;
        UsageLimit memory usageLimit;
        for (uint256 j = 0; j < newUsageLimits.length; j++) {
          if (newUsageLimits[j].usageHash == bytes32(0)) {
            // Initialize new usage limit
            usageLimit = UsageLimit({ usageHash: usageHash, usageAmount: 0 });
            newUsageLimits[j] = usageLimit;
            actualLimitsCount = j + 1;
            break;
          }
          if (newUsageLimits[j].usageHash == usageHash) {
            // Value exists, use it
            usageLimit = newUsageLimits[j];
            previousUsage = usageLimit.usageAmount;
            break;
          }
        }
        if (previousUsage == 0) {
          // Not in current payload, use storage
          previousUsage = getLimitUsage(wallet, usageHash);
        }
        // Cumulate usage
        value256 += previousUsage;
        usageLimit.usageAmount = value256;
        // Use the cumulative value for comparison
        value = bytes32(value256);
      }

      // Compare based on operation
      if (rule.operation == ParameterOperation.EQUAL) {
        if (value != rule.value) {
          return (false, usageLimits);
        }
      } else if (rule.operation == ParameterOperation.LESS_THAN_OR_EQUAL) {
        if (uint256(value) > uint256(rule.value)) {
          return (false, usageLimits);
        }
      } else if (rule.operation == ParameterOperation.NOT_EQUAL) {
        if (value == rule.value) {
          return (false, usageLimits);
        }
      } else if (rule.operation == ParameterOperation.GREATER_THAN_OR_EQUAL) {
        if (uint256(value) < uint256(rule.value)) {
          return (false, usageLimits);
        }
      }
    }

    // Fix array length
    assembly {
      mstore(newUsageLimits, actualLimitsCount)
    }

    return (true, newUsageLimits);
  }

}

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

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Permission, UsageLimit } from "./Permission.sol";

/// @notice Permissions configuration for a specific session signer
/// @param signer Address of the session signer these permissions apply to
/// @param chainId Chain ID of the session (0 = any chain)
/// @param valueLimit Maximum native token value this signer can send
/// @param deadline Deadline for the session. (0 = no deadline)
/// @param permissions Array of encoded permissions granted to this signer
struct SessionPermissions {
  address signer;
  uint256 chainId;
  uint256 valueLimit;
  uint64 deadline;
  Permission[] permissions;
}

/// @notice Usage limits configuration for a specific session signer
/// @param signer Address of the session signer these limits apply to
/// @param limits Array of usage limits
/// @param totalValueUsed Total native token value used
struct SessionUsageLimits {
  address signer;
  UsageLimit[] limits;
  uint256 totalValueUsed;
}

/// @title IExplicitSessionManager
/// @author Agustin Aguilar, Michael Standen
/// @notice Interface for the explicit session manager
interface IExplicitSessionManager {

  /// @notice Increment usage for a caller's given session and target
  /// @param limits Array of limit/session/target combinations
  function incrementUsageLimit(
    UsageLimit[] calldata limits
  ) external;

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { LibBytes } from "../utils/LibBytes.sol";

using LibBytes for bytes;

/// @title Payload
/// @author Agustin Aguilar, Michael Standen, William Hua
/// @notice Library for encoding and decoding payloads
library Payload {

  /// @notice Error thrown when the kind is invalid
  error InvalidKind(uint8 kind);

  /// @dev keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)")
  bytes32 private constant EIP712_DOMAIN_TYPEHASH = 0x8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f;

  /// @dev keccak256("Sequence Wallet")
  bytes32 private constant EIP712_DOMAIN_NAME_SEQUENCE =
    0x4aa45ca7ad825ceb1bf35643f0a58c295239df563b1b565c2485f96477c56318;

  /// @dev keccak256("3")
  bytes32 private constant EIP712_DOMAIN_VERSION_SEQUENCE =
    0x2a80e1ef1d7842f27f2e6be0972bb708b9a135c38860dbe73c27c3486c34f4de;

  function domainSeparator(bool _noChainId, address _wallet) internal view returns (bytes32 _domainSeparator) {
    return keccak256(
      abi.encode(
        EIP712_DOMAIN_TYPEHASH,
        EIP712_DOMAIN_NAME_SEQUENCE,
        EIP712_DOMAIN_VERSION_SEQUENCE,
        _noChainId ? uint256(0) : uint256(block.chainid),
        _wallet
      )
    );
  }

  /// @dev keccak256("Call(address to,uint256 value,bytes data,uint256 gasLimit,bool delegateCall,bool onlyFallback,uint256 behaviorOnError)")
  bytes32 private constant CALL_TYPEHASH = 0x0603985259a953da1f65a522f589c17bd1d0117ec1d3abb7c0788aef251ef437;

  /// @dev keccak256("Calls(Call[] calls,uint256 space,uint256 nonce,address[] wallets)Call(address to,uint256 value,bytes data,uint256 gasLimit,bool delegateCall,bool onlyFallback,uint256 behaviorOnError)")
  bytes32 private constant CALLS_TYPEHASH = 0x11e1e4079a79a66e4ade50033cfe2678cdd5341d2dfe5ef9513edb1a0be147a2;

  /// @dev keccak256("Message(bytes message,address[] wallets)")
  bytes32 private constant MESSAGE_TYPEHASH = 0xe19a3b94fc3c7ece3f890d98a99bc422615537a08dea0603fa8425867d87d466;

  /// @dev keccak256("ConfigUpdate(bytes32 imageHash,address[] wallets)")
  bytes32 private constant CONFIG_UPDATE_TYPEHASH = 0x11fdeb7e8373a1aa96bfac8d0ea91526b2c5d15e5cee20e0543e780258f3e8e4;

  /// @notice Kind of transaction
  uint8 public constant KIND_TRANSACTIONS = 0x00;
  /// @notice Kind of digest
  uint8 public constant KIND_MESSAGE = 0x01;
  /// @notice Kind of config update
  uint8 public constant KIND_CONFIG_UPDATE = 0x02;
  /// @notice Kind of message
  uint8 public constant KIND_DIGEST = 0x03;

  /// @notice Behavior on error: ignore error
  uint8 public constant BEHAVIOR_IGNORE_ERROR = 0x00;
  /// @notice Behavior on error: revert on error
  uint8 public constant BEHAVIOR_REVERT_ON_ERROR = 0x01;
  /// @notice Behavior on error: abort on error
  uint8 public constant BEHAVIOR_ABORT_ON_ERROR = 0x02;

  /// @notice Payload call information
  /// @param to Address of the target contract
  /// @param value Value to send with the call
  /// @param data Data to send with the call
  /// @param gasLimit Gas limit for the call
  /// @param delegateCall If the call is a delegate call
  /// @param onlyFallback If the call should only be executed in an error scenario
  /// @param behaviorOnError Behavior on error
  struct Call {
    address to;
    uint256 value;
    bytes data;
    uint256 gasLimit;
    bool delegateCall;
    bool onlyFallback;
    uint256 behaviorOnError;
  }

  /// @notice Decoded payload
  /// @param kind Kind of payload
  /// @param noChainId If the chain ID should be omitted
  /// @param calls Array of calls (transaction kind)
  /// @param space Nonce space for the calls (transaction kind)
  /// @param nonce Nonce value for the calls (transaction kind)
  /// @param message Message to validate (message kind)
  /// @param imageHash Image hash to update to (config update kind)
  /// @param digest Digest to validate (digest kind)
  /// @param parentWallets Parent wallets
  struct Decoded {
    uint8 kind;
    bool noChainId;
    // Transaction kind
    Call[] calls;
    uint256 space;
    uint256 nonce;
    // Message kind
    bytes message;
    // Config update kind
    bytes32 imageHash;
    // Digest kind for 1271
    bytes32 digest;
    // Parent wallets
    address[] parentWallets;
  }

  function fromMessage(
    bytes memory message
  ) internal pure returns (Decoded memory _decoded) {
    _decoded.kind = KIND_MESSAGE;
    _decoded.message = message;
  }

  function fromConfigUpdate(
    bytes32 imageHash
  ) internal pure returns (Decoded memory _decoded) {
    _decoded.kind = KIND_CONFIG_UPDATE;
    _decoded.imageHash = imageHash;
  }

  function fromDigest(
    bytes32 digest
  ) internal pure returns (Decoded memory _decoded) {
    _decoded.kind = KIND_DIGEST;
    _decoded.digest = digest;
  }

  function fromPackedCalls(
    bytes calldata packed
  ) internal view returns (Decoded memory _decoded) {
    _decoded.kind = KIND_TRANSACTIONS;

    // Read the global flag
    (uint256 globalFlag, uint256 pointer) = packed.readFirstUint8();

    // First bit determines if space is zero or not
    if (globalFlag & 0x01 == 0x01) {
      _decoded.space = 0;
    } else {
      (_decoded.space, pointer) = packed.readUint160(pointer);
    }

    // Next 3 bits determine the size of the nonce
    uint256 nonceSize = (globalFlag >> 1) & 0x07;

    if (nonceSize > 0) {
      // Read the nonce
      (_decoded.nonce, pointer) = packed.readUintX(pointer, nonceSize);
    }

    uint256 numCalls;

    // Bit 5 determines if the batch contains a single call
    if (globalFlag & 0x10 == 0x10) {
      numCalls = 1;
    } else {
      // Bit 6 determines if the number of calls uses 1 byte or 2 bytes
      if (globalFlag & 0x20 == 0x20) {
        (numCalls, pointer) = packed.readUint16(pointer);
      } else {
        (numCalls, pointer) = packed.readUint8(pointer);
      }
    }

    // Read the calls
    _decoded.calls = new Call[](numCalls);

    for (uint256 i = 0; i < numCalls; i++) {
      uint8 flags;
      (flags, pointer) = packed.readUint8(pointer);

      // First bit determines if this is a call to self
      // or a call to another address
      if (flags & 0x01 == 0x01) {
        // Call to self
        _decoded.calls[i].to = address(this);
      } else {
        // Call to another address
        (_decoded.calls[i].to, pointer) = packed.readAddress(pointer);
      }

      // Second bit determines if the call has value or not
      if (flags & 0x02 == 0x02) {
        (_decoded.calls[i].value, pointer) = packed.readUint256(pointer);
      }

      // Third bit determines if the call has data or not
      if (flags & 0x04 == 0x04) {
        // 3 bytes determine the size of the calldata
        uint256 calldataSize;
        (calldataSize, pointer) = packed.readUint24(pointer);
        _decoded.calls[i].data = packed[pointer:pointer + calldataSize];
        pointer += calldataSize;
      }

      // Fourth bit determines if the call has a gas limit or not
      if (flags & 0x08 == 0x08) {
        (_decoded.calls[i].gasLimit, pointer) = packed.readUint256(pointer);
      }

      // Fifth bit determines if the call is a delegate call or not
      _decoded.calls[i].delegateCall = (flags & 0x10 == 0x10);

      // Sixth bit determines if the call is fallback only
      _decoded.calls[i].onlyFallback = (flags & 0x20 == 0x20);

      // Last 2 bits are directly mapped to the behavior on error
      _decoded.calls[i].behaviorOnError = (flags & 0xC0) >> 6;
    }
  }

  function hashCall(
    Call memory c
  ) internal pure returns (bytes32) {
    return keccak256(
      abi.encode(
        CALL_TYPEHASH, c.to, c.value, keccak256(c.data), c.gasLimit, c.delegateCall, c.onlyFallback, c.behaviorOnError
      )
    );
  }

  function hashCalls(
    Call[] memory calls
  ) internal pure returns (bytes32) {
    // In EIP712, an array is often hashed as the keccak256 of the concatenated
    // hashes of each item. So we hash each Call, pack them, and hash again.
    bytes32[] memory callHashes = new bytes32[](calls.length);
    for (uint256 i = 0; i < calls.length; i++) {
      callHashes[i] = hashCall(calls[i]);
    }
    return keccak256(abi.encodePacked(callHashes));
  }

  function toEIP712(
    Decoded memory _decoded
  ) internal pure returns (bytes32) {
    bytes32 walletsHash = keccak256(abi.encodePacked(_decoded.parentWallets));

    if (_decoded.kind == KIND_TRANSACTIONS) {
      bytes32 callsHash = hashCalls(_decoded.calls);
      // The top-level struct for Calls might be something like:
      // Calls(bytes32 callsHash,uint256 space,uint256 nonce,bytes32 walletsHash)
      return keccak256(abi.encode(CALLS_TYPEHASH, callsHash, _decoded.space, _decoded.nonce, walletsHash));
    } else if (_decoded.kind == KIND_MESSAGE) {
      // If you define your top-level as: Message(bytes32 messageHash,bytes32 walletsHash)
      return keccak256(abi.encode(MESSAGE_TYPEHASH, keccak256(_decoded.message), walletsHash));
    } else if (_decoded.kind == KIND_CONFIG_UPDATE) {
      // Top-level: ConfigUpdate(bytes32 imageHash,bytes32 walletsHash)
      return keccak256(abi.encode(CONFIG_UPDATE_TYPEHASH, _decoded.imageHash, walletsHash));
    } else if (_decoded.kind == KIND_DIGEST) {
      // Top-level: Use MESSAGE_TYPEHASH but assume the digest is already the hashed message
      return keccak256(abi.encode(MESSAGE_TYPEHASH, _decoded.digest, walletsHash));
    } else {
      // Unknown kind
      revert InvalidKind(_decoded.kind);
    }
  }

  function hash(
    Decoded memory _decoded
  ) internal view returns (bytes32) {
    bytes32 domain = domainSeparator(_decoded.noChainId, address(this));
    bytes32 structHash = toEIP712(_decoded);
    return keccak256(abi.encodePacked("\x19\x01", domain, structHash));
  }

  function hashFor(Decoded memory _decoded, address _wallet) internal view returns (bytes32) {
    bytes32 domain = domainSeparator(_decoded.noChainId, _wallet);
    bytes32 structHash = toEIP712(_decoded);
    return keccak256(abi.encodePacked("\x19\x01", domain, structHash));
  }

}

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

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

/// @title SessionErrors
/// @author Michael Standen
/// @notice Errors for the session manager
library SessionErrors {

  /// @notice Invalid session signer
  error InvalidSessionSigner(address invalidSigner);
  /// @notice Invalid chainId
  error InvalidChainId(uint256 invalidChainId);
  /// @notice Invalid self call
  error InvalidSelfCall();
  /// @notice Invalid delegate call
  error InvalidDelegateCall();
  /// @notice Invalid call behavior
  error InvalidBehavior();
  /// @notice Invalid value
  error InvalidValue();
  /// @notice Invalid node type in session configuration
  error InvalidNodeType(uint256 flag);
  /// @notice Error thrown when the payload kind is invalid
  error InvalidPayloadKind();
  /// @notice Error thrown when the calls length is invalid
  error InvalidCallsLength();
  /// @notice Error thrown when the payload space is invalid
  error InvalidSpace(uint256 space);

  // ---- Explicit session errors ----

  /// @notice Missing permission for explicit session
  error MissingPermission();
  /// @notice Invalid permission for explicit session
  error InvalidPermission();
  /// @notice Session expired
  error SessionExpired(uint256 deadline);
  /// @notice Invalid limit usage increment
  error InvalidLimitUsageIncrement();

  // ---- Implicit session errors ----

  /// @notice Blacklisted address
  error BlacklistedAddress(address target);
  /// @notice Invalid implicit result
  error InvalidImplicitResult();
  /// @notice Invalid identity signer
  error InvalidIdentitySigner();
  /// @notice Invalid blacklist
  error InvalidBlacklist();
  /// @notice Invalid attestation
  error InvalidAttestation();
  /// @notice The blacklist was not sorted
  error InvalidBlacklistUnsorted();

}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Stage2Module } from "./Stage2Module.sol";
import { Calls } from "./modules/Calls.sol";

import { ERC4337v07 } from "./modules/ERC4337v07.sol";
import { Hooks } from "./modules/Hooks.sol";
import { Stage1Auth } from "./modules/auth/Stage1Auth.sol";
import { IAuth } from "./modules/interfaces/IAuth.sol";

/// @title Stage1Module
/// @author Agustin Aguilar
/// @notice The initial stage of the wallet
contract Stage1Module is Calls, Stage1Auth, Hooks, ERC4337v07 {

  constructor(
    address _factory,
    address _entryPoint
  ) Stage1Auth(_factory, address(new Stage2Module(_entryPoint))) ERC4337v07(_entryPoint) { }

  /// @inheritdoc IAuth
  function _isValidImage(
    bytes32 _imageHash
  ) internal view virtual override(IAuth, Stage1Auth) returns (bool) {
    return super._isValidImage(_imageHash);
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../../modules/Payload.sol";
import { ISapient } from "../../modules/interfaces/ISapient.sol";
import { LibBytes } from "../../utils/LibBytes.sol";

import { SessionErrors } from "./SessionErrors.sol";
import { SessionSig } from "./SessionSig.sol";
import {
  ExplicitSessionManager,
  IExplicitSessionManager,
  SessionPermissions,
  SessionUsageLimits
} from "./explicit/ExplicitSessionManager.sol";
import { Permission, UsageLimit } from "./explicit/Permission.sol";
import { ImplicitSessionManager } from "./implicit/ImplicitSessionManager.sol";

using LibBytes for bytes;

/// @title SessionManager
/// @author Michael Standen, Agustin Aguilar
/// @notice Manager for smart sessions
contract SessionManager is ISapient, ImplicitSessionManager, ExplicitSessionManager {

  /// @notice Maximum nonce space allowed for sessions use.
  /// @dev This excludes half the possible bits (uint160 vs uint80)
  uint256 public constant MAX_SPACE = type(uint80).max - 1;

  /// @inheritdoc ISapient
  function recoverSapientSignature(
    Payload.Decoded calldata payload,
    bytes calldata encodedSignature
  ) external view returns (bytes32) {
    // Validate outer Payload
    if (payload.kind != Payload.KIND_TRANSACTIONS) {
      revert SessionErrors.InvalidPayloadKind();
    }
    if (payload.space > MAX_SPACE) {
      revert SessionErrors.InvalidSpace(payload.space);
    }
    if (payload.calls.length == 0) {
      revert SessionErrors.InvalidCallsLength();
    }

    // Decode signature
    SessionSig.DecodedSignature memory sig = SessionSig.recoverSignature(payload, encodedSignature);

    address wallet = msg.sender;

    // Initialize session usage limits for explicit session
    SessionUsageLimits[] memory sessionUsageLimits = new SessionUsageLimits[](payload.calls.length);

    for (uint256 i = 0; i < payload.calls.length; i++) {
      Payload.Call calldata call = payload.calls[i];

      // Ban delegate calls
      if (call.delegateCall) {
        revert SessionErrors.InvalidDelegateCall();
      }
      // Ban self calls to the wallet
      if (call.to == wallet) {
        revert SessionErrors.InvalidSelfCall();
      }

      // Check if this call could cause usage limits to be skipped
      if (call.behaviorOnError == Payload.BEHAVIOR_ABORT_ON_ERROR) {
        revert SessionErrors.InvalidBehavior();
      }

      // Validate call signature
      SessionSig.CallSignature memory callSignature = sig.callSignatures[i];
      if (callSignature.isImplicit) {
        // Validate implicit calls
        _validateImplicitCall(
          call, wallet, callSignature.sessionSigner, callSignature.attestation, sig.implicitBlacklist
        );
      } else {
        // Find the session usage limits for the current call
        SessionUsageLimits memory limits;
        uint256 limitsIdx;
        for (limitsIdx = 0; limitsIdx < sessionUsageLimits.length; limitsIdx++) {
          if (sessionUsageLimits[limitsIdx].signer == address(0)) {
            // Initialize new session usage limits
            limits.signer = callSignature.sessionSigner;
            limits.limits = new UsageLimit[](0);
            bytes32 usageHash = keccak256(abi.encode(callSignature.sessionSigner, VALUE_TRACKING_ADDRESS));
            limits.totalValueUsed = getLimitUsage(wallet, usageHash);
            break;
          }
          if (sessionUsageLimits[limitsIdx].signer == callSignature.sessionSigner) {
            limits = sessionUsageLimits[limitsIdx];
            break;
          }
        }
        // Validate explicit calls. Obtain usage limits for increment validation.
        (limits) = _validateExplicitCall(
          payload,
          i,
          wallet,
          callSignature.sessionSigner,
          sig.sessionPermissions,
          callSignature.sessionPermission,
          limits
        );
        sessionUsageLimits[limitsIdx] = limits;
      }
    }

    {
      // Reduce the size of the sessionUsageLimits array
      SessionUsageLimits[] memory actualSessionUsageLimits = new SessionUsageLimits[](sessionUsageLimits.length);
      uint256 actualSize;
      for (uint256 i = 0; i < sessionUsageLimits.length; i++) {
        if (sessionUsageLimits[i].limits.length > 0 || sessionUsageLimits[i].totalValueUsed > 0) {
          actualSessionUsageLimits[actualSize] = sessionUsageLimits[i];
          actualSize++;
        }
      }
      assembly {
        mstore(actualSessionUsageLimits, actualSize)
      }

      // Bulk validate the updated usage limits
      Payload.Call calldata firstCall = payload.calls[0];
      _validateLimitUsageIncrement(firstCall, actualSessionUsageLimits);
    }

    // Return the image hash
    return sig.imageHash;
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Calls } from "./modules/Calls.sol";

import { ERC4337v07 } from "./modules/ERC4337v07.sol";
import { Hooks } from "./modules/Hooks.sol";
import { Stage2Auth } from "./modules/auth/Stage2Auth.sol";
import { IAuth } from "./modules/interfaces/IAuth.sol";

/// @title Stage2Module
/// @author Agustin Aguilar
/// @notice The second stage of the wallet
contract Stage2Module is Calls, Stage2Auth, Hooks, ERC4337v07 {

  constructor(
    address _entryPoint
  ) ERC4337v07(_entryPoint) { }

  /// @inheritdoc IAuth
  function _isValidImage(
    bytes32 _imageHash
  ) internal view virtual override(IAuth, Stage2Auth) returns (bool) {
    return super._isValidImage(_imageHash);
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.18;

import { Calls } from "./Calls.sol";

import { ReentrancyGuard } from "./ReentrancyGuard.sol";
import { IAccount, PackedUserOperation } from "./interfaces/IAccount.sol";
import { IERC1271_MAGIC_VALUE_HASH } from "./interfaces/IERC1271.sol";
import { IEntryPoint } from "./interfaces/IEntryPoint.sol";

/// @title ERC4337v07
/// @author Agustin Aguilar, Michael Standen
/// @notice ERC4337 v7 support
abstract contract ERC4337v07 is ReentrancyGuard, IAccount, Calls {

  uint256 internal constant SIG_VALIDATION_FAILED = 1;

  address public immutable entrypoint;

  error InvalidEntryPoint(address _entrypoint);
  error ERC4337Disabled();

  constructor(
    address _entrypoint
  ) {
    entrypoint = _entrypoint;
  }

  /// @inheritdoc IAccount
  function validateUserOp(
    PackedUserOperation calldata userOp,
    bytes32 userOpHash,
    uint256 missingAccountFunds
  ) external returns (uint256 validationData) {
    if (entrypoint == address(0)) {
      revert ERC4337Disabled();
    }

    if (msg.sender != entrypoint) {
      revert InvalidEntryPoint(msg.sender);
    }

    // userOp.nonce is validated by the entrypoint

    if (missingAccountFunds != 0) {
      IEntryPoint(entrypoint).depositTo{ value: missingAccountFunds }(address(this));
    }

    if (this.isValidSignature(userOpHash, userOp.signature) != IERC1271_MAGIC_VALUE_HASH) {
      return SIG_VALIDATION_FAILED;
    }

    return 0;
  }

  /// @notice Execute a user operation
  /// @param _payload The packed payload
  /// @dev This is the execute function for the EntryPoint to call.
  function executeUserOp(
    bytes calldata _payload
  ) external nonReentrant {
    if (entrypoint == address(0)) {
      revert ERC4337Disabled();
    }

    if (msg.sender != entrypoint) {
      revert InvalidEntryPoint(msg.sender);
    }

    this.selfExecute(_payload);
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { LibOptim } from "../utils/LibOptim.sol";
import { Nonce } from "./Nonce.sol";
import { Payload } from "./Payload.sol";

import { ReentrancyGuard } from "./ReentrancyGuard.sol";
import { BaseAuth } from "./auth/BaseAuth.sol";
import { IDelegatedExtension } from "./interfaces/IDelegatedExtension.sol";

/// @title Calls
/// @author Agustin Aguilar, Michael Standen, William Hua
/// @notice Contract for executing calls
abstract contract Calls is ReentrancyGuard, BaseAuth, Nonce {

  /// @notice Emitted when a call succeeds
  event CallSucceeded(bytes32 _opHash, uint256 _index);
  /// @notice Emitted when a call fails
  event CallFailed(bytes32 _opHash, uint256 _index, bytes _returnData);
  /// @notice Emitted when a call is aborted
  event CallAborted(bytes32 _opHash, uint256 _index, bytes _returnData);
  /// @notice Emitted when a call is skipped
  event CallSkipped(bytes32 _opHash, uint256 _index);

  /// @notice Error thrown when a call reverts
  error Reverted(Payload.Decoded _payload, uint256 _index, bytes _returnData);
  /// @notice Error thrown when a signature is invalid
  error InvalidSignature(Payload.Decoded _payload, bytes _signature);
  /// @notice Error thrown when there is not enough gas
  error NotEnoughGas(Payload.Decoded _payload, uint256 _index, uint256 _gasLeft);

  /// @notice Execute a call
  /// @param _payload The payload
  /// @param _signature The signature
  function execute(bytes calldata _payload, bytes calldata _signature) external payable virtual nonReentrant {
    uint256 startingGas = gasleft();
    Payload.Decoded memory decoded = Payload.fromPackedCalls(_payload);

    _consumeNonce(decoded.space, decoded.nonce);
    (bool isValid, bytes32 opHash) = signatureValidation(decoded, _signature);

    if (!isValid) {
      revert InvalidSignature(decoded, _signature);
    }

    _execute(startingGas, opHash, decoded);
  }

  /// @notice Execute a call
  /// @dev Callable only by the contract itself
  /// @param _payload The payload
  function selfExecute(
    bytes calldata _payload
  ) external payable virtual onlySelf {
    uint256 startingGas = gasleft();
    Payload.Decoded memory decoded = Payload.fromPackedCalls(_payload);
    bytes32 opHash = Payload.hash(decoded);
    _execute(startingGas, opHash, decoded);
  }

  function _execute(uint256 _startingGas, bytes32 _opHash, Payload.Decoded memory _decoded) private {
    bool errorFlag = false;

    uint256 numCalls = _decoded.calls.length;
    for (uint256 i = 0; i < numCalls; i++) {
      Payload.Call memory call = _decoded.calls[i];

      // Skip onlyFallback calls if no error occurred
      if (call.onlyFallback && !errorFlag) {
        emit CallSkipped(_opHash, i);
        continue;
      }

      // Reset the error flag
      // onlyFallback calls only apply when the immediately preceding transaction fails
      errorFlag = false;

      uint256 gasLimit = call.gasLimit;
      if (gasLimit != 0 && gasleft() < gasLimit) {
        revert NotEnoughGas(_decoded, i, gasleft());
      }

      bool success;
      if (call.delegateCall) {
        (success) = LibOptim.delegatecall(
          call.to,
          gasLimit == 0 ? gasleft() : gasLimit,
          abi.encodeWithSelector(
            IDelegatedExtension.handleSequenceDelegateCall.selector,
            _opHash,
            _startingGas,
            i,
            numCalls,
            _decoded.space,
            call.data
          )
        );
      } else {
        (success) = LibOptim.call(call.to, call.value, gasLimit == 0 ? gasleft() : gasLimit, call.data);
      }

      if (!success) {
        if (call.behaviorOnError == Payload.BEHAVIOR_IGNORE_ERROR) {
          errorFlag = true;
          emit CallFailed(_opHash, i, LibOptim.returnData());
          continue;
        }

        if (call.behaviorOnError == Payload.BEHAVIOR_REVERT_ON_ERROR) {
          revert Reverted(_decoded, i, LibOptim.returnData());
        }

        if (call.behaviorOnError == Payload.BEHAVIOR_ABORT_ON_ERROR) {
          emit CallAborted(_opHash, i, LibOptim.returnData());
          break;
        }
      }

      emit CallSucceeded(_opHash, i);
    }
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Wallet } from "../../Wallet.sol";
import { Implementation } from "../Implementation.sol";
import { Storage } from "../Storage.sol";
import { BaseAuth } from "./BaseAuth.sol";

/// @title Stage1Auth
/// @author Agustin Aguilar
/// @notice Stage 1 auth contract
contract Stage1Auth is BaseAuth, Implementation {

  /// @notice Error thrown when the image hash is zero
  error ImageHashIsZero();
  /// @notice Error thrown when the signature type is invalid
  error InvalidSignatureType(bytes1 _type);

  /// @notice Initialization code hash
  bytes32 public immutable INIT_CODE_HASH;
  /// @notice Factory address
  address public immutable FACTORY;
  /// @notice Stage 2 implementation address
  address public immutable STAGE_2_IMPLEMENTATION;

  /// @dev keccak256("org.arcadeum.module.auth.upgradable.image.hash")
  bytes32 internal constant IMAGE_HASH_KEY = bytes32(0xea7157fa25e3aa17d0ae2d5280fa4e24d421c61842aa85e45194e1145aa72bf8);

  /// @notice Emitted when the image hash is updated
  event ImageHashUpdated(bytes32 newImageHash);

  constructor(address _factory, address _stage2) {
    // Build init code hash of the deployed wallets using that module
    bytes32 initCodeHash = keccak256(abi.encodePacked(Wallet.creationCode, uint256(uint160(address(this)))));

    INIT_CODE_HASH = initCodeHash;
    FACTORY = _factory;
    STAGE_2_IMPLEMENTATION = _stage2;
  }

  function _updateImageHash(
    bytes32 _imageHash
  ) internal virtual override {
    // Update imageHash in storage
    if (_imageHash == bytes32(0)) {
      revert ImageHashIsZero();
    }
    Storage.writeBytes32(IMAGE_HASH_KEY, _imageHash);
    emit ImageHashUpdated(_imageHash);

    // Update wallet implementation to stage2 version
    _updateImplementation(STAGE_2_IMPLEMENTATION);
  }

  function _isValidImage(
    bytes32 _imageHash
  ) internal view virtual override returns (bool) {
    return address(uint160(uint256(keccak256(abi.encodePacked(hex"ff", FACTORY, _imageHash, INIT_CODE_HASH)))))
      == address(this);
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Stage2Module } from "./Stage2Module.sol";

import { Payload } from "./modules/Payload.sol";
import { IDelegatedExtension } from "./modules/interfaces/IDelegatedExtension.sol";
import { LibOptim } from "./utils/LibOptim.sol";

/// @title Simulator
/// @author William Hua
/// @notice Helper for simulating the execution of a payload
contract Simulator is Stage2Module {

  constructor(
    address _entryPoint
  ) Stage2Module(_entryPoint) { }

  /// @notice Status of the call
  enum Status {
    Skipped,
    Succeeded,
    Failed,
    Aborted,
    Reverted,
    NotEnoughGas
  }

  /// @notice Result of the call
  struct Result {
    Status status;
    bytes result;
    uint256 gasUsed;
  }

  /// @notice Simulate the execution of a payload
  /// @param _calls The calls to simulate
  /// @return results The results of the calls
  function simulate(
    Payload.Call[] calldata _calls
  ) external returns (Result[] memory results) {
    uint256 startingGas = gasleft();
    bool errorFlag = false;

    uint256 numCalls = _calls.length;
    results = new Result[](numCalls);
    for (uint256 i = 0; i < numCalls; i++) {
      Payload.Call memory call = _calls[i];

      // Skip onlyFallback calls if no error occurred
      if (call.onlyFallback && !errorFlag) {
        continue;
      }

      // Reset the error flag
      // onlyFallback calls only apply when the immediately preceding transaction fails
      errorFlag = false;

      uint256 gasLimit = call.gasLimit;
      if (gasLimit != 0 && gasleft() < gasLimit) {
        results[i].status = Status.NotEnoughGas;
        results[i].result = abi.encode(gasleft());
        return results;
      }

      bool success;
      if (call.delegateCall) {
        uint256 initial = gasleft();
        (success) = LibOptim.delegatecall(
          call.to,
          gasLimit == 0 ? gasleft() : gasLimit,
          abi.encodeWithSelector(
            IDelegatedExtension.handleSequenceDelegateCall.selector, 0, startingGas, i, numCalls, 0, call.data
          )
        );
        results[i].gasUsed = initial - gasleft();
      } else {
        uint256 initial = gasleft();
        (success) = LibOptim.call(call.to, call.value, gasLimit == 0 ? gasleft() : gasLimit, call.data);
        results[i].gasUsed = initial - gasleft();
      }

      if (!success) {
        if (call.behaviorOnError == Payload.BEHAVIOR_IGNORE_ERROR) {
          errorFlag = true;
          results[i].status = Status.Failed;
          results[i].result = LibOptim.returnData();
          continue;
        }

        if (call.behaviorOnError == Payload.BEHAVIOR_REVERT_ON_ERROR) {
          results[i].status = Status.Reverted;
          results[i].result = LibOptim.returnData();
          return results;
        }

        if (call.behaviorOnError == Payload.BEHAVIOR_ABORT_ON_ERROR) {
          results[i].status = Status.Aborted;
          results[i].result = LibOptim.returnData();
          break;
        }
      }

      results[i].status = Status.Succeeded;
      results[i].result = LibOptim.returnData();
    }
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../Payload.sol";

import { Storage } from "../Storage.sol";
import { IAuth } from "../interfaces/IAuth.sol";
import { IERC1271, IERC1271_MAGIC_VALUE_HASH } from "../interfaces/IERC1271.sol";

import { IPartialAuth } from "../interfaces/IPartialAuth.sol";
import { ISapient } from "../interfaces/ISapient.sol";
import { BaseSig } from "./BaseSig.sol";

import { SelfAuth } from "./SelfAuth.sol";

using Payload for Payload.Decoded;

/// @title BaseAuth
/// @author Agustin Aguilar, Michael Standen
/// @notice Base contract for the auth module
abstract contract BaseAuth is IAuth, IPartialAuth, ISapient, IERC1271, SelfAuth {

  /// @dev keccak256("org.sequence.module.auth.static")
  bytes32 private constant STATIC_SIGNATURE_KEY =
    bytes32(0xc852adf5e97c2fc3b38f405671e91b7af1697ef0287577f227ef10494c2a8e86);

  /// @notice Error thrown when the sapient signature is invalid
  error InvalidSapientSignature(Payload.Decoded _payload, bytes _signature);
  /// @notice Error thrown when the signature weight is invalid
  error InvalidSignatureWeight(uint256 _threshold, uint256 _weight);
  /// @notice Error thrown when the static signature has expired
  error InvalidStaticSignatureExpired(bytes32 _opHash, uint256 _expires);
  /// @notice Error thrown when the static signature has the wrong caller
  error InvalidStaticSignatureWrongCaller(bytes32 _opHash, address _caller, address _expectedCaller);

  /// @notice Event emitted when a static signature is set
  event StaticSignatureSet(bytes32 _hash, address _address, uint96 _timestamp);

  function _getStaticSignature(
    bytes32 _hash
  ) internal view returns (address, uint256) {
    uint256 word = uint256(Storage.readBytes32Map(STATIC_SIGNATURE_KEY, _hash));
    return (address(uint160(word >> 96)), uint256(uint96(word)));
  }

  function _setStaticSignature(bytes32 _hash, address _address, uint256 _timestamp) internal {
    Storage.writeBytes32Map(
      STATIC_SIGNATURE_KEY, _hash, bytes32(uint256(uint160(_address)) << 96 | (_timestamp & 0xffffffffffffffffffffffff))
    );
  }

  /// @notice Get the static signature for a specific hash
  /// @param _hash The hash to get the static signature for
  /// @return address The address associated with the static signature
  /// @return timestamp The timestamp of the static signature
  function getStaticSignature(
    bytes32 _hash
  ) external view returns (address, uint256) {
    return _getStaticSignature(_hash);
  }

  /// @notice Set the static signature for a specific hash
  /// @param _hash The hash to set the static signature for
  /// @param _address The address to associate with the static signature
  /// @param _timestamp The timestamp of the static signature
  /// @dev Only callable by the wallet itself
  function setStaticSignature(bytes32 _hash, address _address, uint96 _timestamp) external onlySelf {
    _setStaticSignature(_hash, _address, _timestamp);
    emit StaticSignatureSet(_hash, _address, _timestamp);
  }

  /// @notice Update the image hash
  /// @param _imageHash The new image hash
  /// @dev Only callable by the wallet itself
  function updateImageHash(
    bytes32 _imageHash
  ) external virtual onlySelf {
    _updateImageHash(_imageHash);
  }

  function signatureValidation(
    Payload.Decoded memory _payload,
    bytes calldata _signature
  ) internal view virtual returns (bool isValid, bytes32 opHash) {
    // Read first bit to determine if static signature is used
    bytes1 signatureFlag = _signature[0];

    if (signatureFlag & 0x80 == 0x80) {
      opHash = _payload.hash();

      (address addr, uint256 timestamp) = _getStaticSignature(opHash);
      if (timestamp <= block.timestamp) {
        revert InvalidStaticSignatureExpired(opHash, timestamp);
      }

      if (addr != address(0) && addr != msg.sender) {
        revert InvalidStaticSignatureWrongCaller(opHash, msg.sender, addr);
      }

      return (true, opHash);
    }

    // Static signature is not used, recover and validate imageHash

    uint256 threshold;
    uint256 weight;
    bytes32 imageHash;

    (threshold, weight, imageHash,, opHash) = BaseSig.recover(_payload, _signature, false, address(0));

    // Validate the weight
    if (weight < threshold) {
      revert InvalidSignatureWeight(threshold, weight);
    }

    isValid = _isValidImage(imageHash);
  }

  /// @inheritdoc ISapient
  function recoverSapientSignature(
    Payload.Decoded memory _payload,
    bytes calldata _signature
  ) external view returns (bytes32) {
    // Copy parent wallets + add caller at the end
    address[] memory parentWallets = new address[](_payload.parentWallets.length + 1);

    for (uint256 i = 0; i < _payload.parentWallets.length; i++) {
      parentWallets[i] = _payload.parentWallets[i];
    }

    parentWallets[_payload.parentWallets.length] = msg.sender;
    _payload.parentWallets = parentWallets;

    (bool isValid,) = signatureValidation(_payload, _signature);
    if (!isValid) {
      revert InvalidSapientSignature(_payload, _signature);
    }

    return bytes32(uint256(1));
  }

  /// @inheritdoc IERC1271
  function isValidSignature(bytes32 _hash, bytes calldata _signature) external view returns (bytes4) {
    Payload.Decoded memory payload = Payload.fromDigest(_hash);

    (bool isValid,) = signatureValidation(payload, _signature);
    if (!isValid) {
      return bytes4(0);
    }

    return IERC1271_MAGIC_VALUE_HASH;
  }

  /// @inheritdoc IPartialAuth
  function recoverPartialSignature(
    Payload.Decoded memory _payload,
    bytes calldata _signature
  )
    external
    view
    returns (
      uint256 threshold,
      uint256 weight,
      bool isValidImage,
      bytes32 imageHash,
      uint256 checkpoint,
      bytes32 opHash
    )
  {
    (threshold, weight, imageHash, checkpoint, opHash) = BaseSig.recover(_payload, _signature, false, address(0));
    isValidImage = _isValidImage(imageHash);
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Stage2Module } from "./Stage2Module.sol";

import { Payload } from "./modules/Payload.sol";
import { IDelegatedExtension } from "./modules/interfaces/IDelegatedExtension.sol";
import { LibOptim } from "./utils/LibOptim.sol";

/// @title Estimator
/// @author William Hua
/// @notice Helper for estimating the gas used for payload validation and execution
contract Estimator is Stage2Module {

  constructor(
    address _entryPoint
  ) Stage2Module(_entryPoint) { }

  function _isValidImage(
    bytes32 _imageHash
  ) internal view virtual override returns (bool) {
    super._isValidImage(_imageHash);
    return true;
  }

  /// @notice Estimate the gas used for payload validation and execution
  /// @param _payload The payload to estimate the gas used for
  /// @param _signature The signature to validate the payload with
  /// @return gasUsed The gas used for payload validation and execution
  function estimate(
    bytes calldata _payload,
    bytes calldata _signature
  ) external payable virtual nonReentrant returns (uint256 gasUsed) {
    uint256 startingGas = gasleft();
    Payload.Decoded memory decoded = Payload.fromPackedCalls(_payload);

    _consumeNonce(decoded.space, readNonce(decoded.space));
    (bool isValid, bytes32 opHash) = signatureValidation(decoded, _signature);

    if (!isValid) {
      revert InvalidSignature(decoded, _signature);
    }

    _estimate(startingGas, opHash, decoded);

    return startingGas - gasleft();
  }

  function _estimate(uint256 _startingGas, bytes32 _opHash, Payload.Decoded memory _decoded) private {
    bool errorFlag = false;

    uint256 numCalls = _decoded.calls.length;
    for (uint256 i = 0; i < numCalls; i++) {
      Payload.Call memory call = _decoded.calls[i];

      // Skip onlyFallback calls if no error occurred
      if (call.onlyFallback && !errorFlag) {
        emit CallSkipped(_opHash, i);
        continue;
      }

      // Reset the error flag
      // onlyFallback calls only apply when the immediately preceding transaction fails
      errorFlag = false;

      uint256 gasLimit = call.gasLimit;
      if (gasLimit != 0 && gasleft() < gasLimit) {
        revert NotEnoughGas(_decoded, i, gasleft());
      }

      bool success;
      if (call.delegateCall) {
        (success) = LibOptim.delegatecall(
          call.to,
          gasLimit == 0 ? gasleft() : gasLimit,
          abi.encodeWithSelector(
            IDelegatedExtension.handleSequenceDelegateCall.selector,
            _opHash,
            _startingGas,
            i,
            numCalls,
            _decoded.space,
            call.data
          )
        );
      } else {
        (success) = LibOptim.call(call.to, call.value, gasLimit == 0 ? gasleft() : gasLimit, call.data);
      }

      if (!success) {
        if (call.behaviorOnError == Payload.BEHAVIOR_IGNORE_ERROR) {
          errorFlag = true;
          emit CallFailed(_opHash, i, LibOptim.returnData());
          continue;
        }

        if (call.behaviorOnError == Payload.BEHAVIOR_REVERT_ON_ERROR) {
          revert Reverted(_decoded, i, LibOptim.returnData());
        }

        if (call.behaviorOnError == Payload.BEHAVIOR_ABORT_ON_ERROR) {
          emit CallAborted(_opHash, i, LibOptim.returnData());
          break;
        }
      }

      emit CallSucceeded(_opHash, i);
    }
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../../modules/Payload.sol";
import { IERC1271, IERC1271_MAGIC_VALUE_HASH } from "../../modules/interfaces/IERC1271.sol";
import { ISapientCompact } from "../../modules/interfaces/ISapient.sol";
import { LibBytes } from "../../utils/LibBytes.sol";
import { LibOptim } from "../../utils/LibOptim.sol";

using LibBytes for bytes;

/// @title Recovery
/// @author Agustin Aguilar, William Hua, Michael Standen
/// @notice A recovery mode sapient signer
contract Recovery is ISapientCompact {

  bytes32 private constant EIP712_DOMAIN_TYPEHASH =
    keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

  bytes32 private constant EIP712_DOMAIN_NAME_SEQUENCE = keccak256("Sequence Wallet - Recovery Mode");
  bytes32 private constant EIP712_DOMAIN_VERSION_SEQUENCE = keccak256("1");

  // Make them similar to the flags in BaseSig.sol
  uint256 internal constant FLAG_RECOVERY_LEAF = 1;
  uint256 internal constant FLAG_NODE = 3;
  uint256 internal constant FLAG_BRANCH = 4;

  /// @notice Emitted when a new payload is queued
  event NewQueuedPayload(address _wallet, address _signer, bytes32 _payloadHash, uint256 _timestamp);

  /// @notice Error thrown when the signature is invalid
  error InvalidSignature(address _wallet, address _signer, Payload.Decoded _payload, bytes _signature);
  /// @notice Error thrown when the payload is already queued
  error AlreadyQueued(address _wallet, address _signer, bytes32 _payloadHash);
  /// @notice Error thrown when the queue is not ready
  error QueueNotReady(address _wallet, bytes32 _payloadHash);
  /// @notice Error thrown when the signature flag is invalid
  error InvalidSignatureFlag(uint256 _flag);

  function domainSeparator(bool _noChainId, address _wallet) internal view returns (bytes32 _domainSeparator) {
    return keccak256(
      abi.encode(
        EIP712_DOMAIN_TYPEHASH,
        EIP712_DOMAIN_NAME_SEQUENCE,
        EIP712_DOMAIN_VERSION_SEQUENCE,
        _noChainId ? uint256(0) : uint256(block.chainid),
        _wallet
      )
    );
  }

  /// @notice Mapping of queued timestamps
  /// @dev wallet -> signer -> payloadHash -> timestamp
  mapping(address => mapping(address => mapping(bytes32 => uint256))) public timestampForQueuedPayload;

  /// @notice Mapping of queued payload hashes
  /// @dev wallet -> signer -> payloadHash[]
  mapping(address => mapping(address => bytes32[])) public queuedPayloadHashes;

  /// @notice Get the total number of queued payloads
  /// @param _wallet The wallet to get the total number of queued payloads for
  /// @param _signer The signer to get the total number of queued payloads for
  /// @return The total number of queued payloads
  function totalQueuedPayloads(address _wallet, address _signer) public view returns (uint256) {
    return queuedPayloadHashes[_wallet][_signer].length;
  }

  function _leafForRecoveryLeaf(
    address _signer,
    uint256 _requiredDeltaTime,
    uint256 _minTimestamp
  ) internal pure returns (bytes32) {
    return keccak256(abi.encodePacked("Sequence recovery leaf:\n", _signer, _requiredDeltaTime, _minTimestamp));
  }

  function _recoverBranch(
    address _wallet,
    bytes32 _payloadHash,
    bytes calldata _signature
  ) internal view returns (bool verified, bytes32 root) {
    uint256 rindex;

    while (rindex < _signature.length) {
      // The first byte is the flag, it determines if we are reading
      uint256 flag;
      (flag, rindex) = _signature.readUint8(rindex);

      if (flag == FLAG_RECOVERY_LEAF) {
        // Read the signer and requiredDeltaTime
        address signer;
        uint256 requiredDeltaTime;
        uint256 minTimestamp;

        (signer, rindex) = _signature.readAddress(rindex);
        (requiredDeltaTime, rindex) = _signature.readUint24(rindex);
        (minTimestamp, rindex) = _signature.readUint64(rindex);

        // Check if we have a queued payload for this signer
        uint256 queuedAt = timestampForQueuedPayload[_wallet][signer][_payloadHash];
        if (queuedAt != 0 && queuedAt >= minTimestamp && block.timestamp - queuedAt >= requiredDeltaTime) {
          verified = true;
        }

        bytes32 node = _leafForRecoveryLeaf(signer, requiredDeltaTime, minTimestamp);
        root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
        continue;
      }

      if (flag == FLAG_NODE) {
        // Read node hash
        bytes32 node;
        (node, rindex) = _signature.readBytes32(rindex);
        root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
        continue;
      }

      if (flag == FLAG_BRANCH) {
        // Read size
        uint256 size;
        (size, rindex) = _signature.readUint24(rindex);

        // Enter a branch of the signature merkle tree
        uint256 nrindex = rindex + size;

        (bool nverified, bytes32 nroot) = _recoverBranch(_wallet, _payloadHash, _signature[rindex:nrindex]);
        rindex = nrindex;

        verified = verified || nverified;
        root = LibOptim.fkeccak256(root, nroot);
        continue;
      }

      revert InvalidSignatureFlag(flag);
    }

    return (verified, root);
  }

  /// @notice Get the recovery payload hash
  /// @param _wallet The wallet to get the recovery payload hash for
  /// @param _payload The payload to get the recovery payload hash for
  /// @return The recovery payload hash
  function recoveryPayloadHash(address _wallet, Payload.Decoded calldata _payload) public view returns (bytes32) {
    bytes32 domain = domainSeparator(_payload.noChainId, _wallet);
    bytes32 structHash = Payload.toEIP712(_payload);
    return keccak256(abi.encodePacked("\x19\x01", domain, structHash));
  }

  /// @inheritdoc ISapientCompact
  function recoverSapientSignatureCompact(
    bytes32 _payloadHash,
    bytes calldata _signature
  ) external view returns (bytes32) {
    (bool verified, bytes32 root) = _recoverBranch(msg.sender, _payloadHash, _signature);
    if (!verified) {
      revert QueueNotReady(msg.sender, _payloadHash);
    }

    return root;
  }

  /// @notice Queue a payload for recovery
  /// @param _wallet The wallet to queue the payload for
  /// @param _signer The signer to queue the payload for
  /// @param _payload The payload to queue
  /// @param _signature The signature to queue the payload for
  function queuePayload(
    address _wallet,
    address _signer,
    Payload.Decoded calldata _payload,
    bytes calldata _signature
  ) external {
    if (!isValidSignature(_wallet, _signer, _payload, _signature)) {
      revert InvalidSignature(_wallet, _signer, _payload, _signature);
    }

    bytes32 payloadHash = Payload.hashFor(_payload, _wallet);
    if (timestampForQueuedPayload[_wallet][_signer][payloadHash] != 0) {
      revert AlreadyQueued(_wallet, _signer, payloadHash);
    }

    timestampForQueuedPayload[_wallet][_signer][payloadHash] = block.timestamp;
    queuedPayloadHashes[_wallet][_signer].push(payloadHash);

    emit NewQueuedPayload(_wallet, _signer, payloadHash, block.timestamp);
  }

  function isValidSignature(
    address _wallet,
    address _signer,
    Payload.Decoded calldata _payload,
    bytes calldata _signature
  ) internal view returns (bool) {
    bytes32 rPayloadHash = recoveryPayloadHash(_wallet, _payload);

    if (_signature.length == 64) {
      // Try an ECDSA signature
      bytes32 r;
      bytes32 s;
      uint8 v;
      (r, s, v,) = _signature.readRSVCompact(0);

      address addr = ecrecover(rPayloadHash, v, r, s);
      if (addr == _signer) {
        return true;
      }
    }

    if (_signer.code.length != 0) {
      // ERC1271
      return IERC1271(_signer).isValidSignature(rPayloadHash, _signature) == IERC1271_MAGIC_VALUE_HASH;
    }

    return false;
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Wallet } from "../../Wallet.sol";
import { Implementation } from "../Implementation.sol";
import { Storage } from "../Storage.sol";
import { BaseAuth } from "./BaseAuth.sol";

/// @title Stage2Auth
/// @author Agustin Aguilar
/// @notice Stage 2 auth contract
contract Stage2Auth is BaseAuth, Implementation {

  /// @dev keccak256("org.arcadeum.module.auth.upgradable.image.hash")
  bytes32 internal constant IMAGE_HASH_KEY = bytes32(0xea7157fa25e3aa17d0ae2d5280fa4e24d421c61842aa85e45194e1145aa72bf8);

  /// @notice Emitted when the image hash is updated
  event ImageHashUpdated(bytes32 newImageHash);

  /// @notice Error thrown when the image hash is zero
  error ImageHashIsZero();

  /// @notice Get the image hash
  /// @return imageHash The image hash
  function imageHash() external view virtual returns (bytes32) {
    return Storage.readBytes32(IMAGE_HASH_KEY);
  }

  function _isValidImage(
    bytes32 _imageHash
  ) internal view virtual override returns (bool) {
    return _imageHash != bytes32(0) && _imageHash == Storage.readBytes32(IMAGE_HASH_KEY);
  }

  function _updateImageHash(
    bytes32 _imageHash
  ) internal virtual override {
    if (_imageHash == bytes32(0)) {
      revert ImageHashIsZero();
    }
    Storage.writeBytes32(IMAGE_HASH_KEY, _imageHash);
    emit ImageHashUpdated(_imageHash);
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { ISapientCompact } from "../../modules/interfaces/ISapient.sol";

import { LibBytes } from "../../utils/LibBytes.sol";
import { LibOptim } from "../../utils/LibOptim.sol";
import { WebAuthn } from "../../utils/WebAuthn.sol";

/// @title Passkeys
/// @author Agustin Aguilar, Michael Standen
/// @notice A sapient signer for passkeys
contract Passkeys is ISapientCompact {

  /// @notice Error thrown when the passkey signature is invalid
  error InvalidPasskeySignature(
    WebAuthn.WebAuthnAuth _webAuthnAuth, bool _requireUserVerification, bytes32 _x, bytes32 _y
  );

  function _rootForPasskey(
    bool _requireUserVerification,
    bytes32 _x,
    bytes32 _y,
    bytes32 _metadata
  ) internal pure returns (bytes32) {
    bytes32 a = LibOptim.fkeccak256(_x, _y);

    bytes32 ruv;
    assembly {
      ruv := _requireUserVerification
    }

    bytes32 b = LibOptim.fkeccak256(ruv, _metadata);
    return LibOptim.fkeccak256(a, b);
  }

  function _decodeSignature(
    bytes calldata _signature
  )
    internal
    pure
    returns (
      WebAuthn.WebAuthnAuth memory _webAuthnAuth,
      bool _requireUserVerification,
      bytes32 _x,
      bytes32 _y,
      bytes32 _metadata
    )
  {
    unchecked {
      // Global flag encoding:
      // 0000 000X : requireUserVerification
      // 0000 00X0 : 1 if 16 bits for authenticatorData size, 0 if 8 bits
      // 0000 0X00 : 1 if 16 bits for clientDataJSON size, 0 if 8 bits
      // 0000 X000 : 1 if 16 bits for challengeIndex, 0 if 8 bits
      // 000X 0000 : 1 if 16 bits for typeIndex, 0 if 8 bits
      // 00X0 0000 : 1 if fallback to abi decode data
      // 0X00 0000 : 1 if signature has metadata node
      // X000 0000 : unused

      bytes1 flags = _signature[0];
      if ((flags & 0x20) == 0) {
        _requireUserVerification = (flags & 0x01) != 0;
        uint256 bytesAuthDataSize = ((uint8(flags & 0x02)) >> 1) + 1;
        uint256 bytesClientDataJSONSize = ((uint8(flags & 0x04)) >> 2) + 1;
        uint256 bytesChallengeIndex = ((uint8(flags & 0x08)) >> 3) + 1;
        uint256 bytesTypeIndex = ((uint8(flags & 0x10)) >> 4) + 1;

        uint256 pointer = 1;

        if ((flags & 0x40) != 0) {
          (_metadata, pointer) = LibBytes.readBytes32(_signature, pointer);
        }

        {
          uint256 authDataSize;
          (authDataSize, pointer) = LibBytes.readUintX(_signature, pointer, bytesAuthDataSize);
          uint256 nextPointer = pointer + authDataSize;
          _webAuthnAuth.authenticatorData = _signature[pointer:nextPointer];
          pointer = nextPointer;
        }

        {
          uint256 clientDataJSONSize;
          (clientDataJSONSize, pointer) = LibBytes.readUintX(_signature, pointer, bytesClientDataJSONSize);
          uint256 nextPointer = pointer + clientDataJSONSize;
          _webAuthnAuth.clientDataJSON = string(_signature[pointer:nextPointer]);
          pointer = nextPointer;
        }

        (_webAuthnAuth.challengeIndex, pointer) = LibBytes.readUintX(_signature, pointer, bytesChallengeIndex);
        (_webAuthnAuth.typeIndex, pointer) = LibBytes.readUintX(_signature, pointer, bytesTypeIndex);

        (_webAuthnAuth.r, pointer) = LibBytes.readBytes32(_signature, pointer);
        (_webAuthnAuth.s, pointer) = LibBytes.readBytes32(_signature, pointer);

        (_x, pointer) = LibBytes.readBytes32(_signature, pointer);
        (_y, pointer) = LibBytes.readBytes32(_signature, pointer);
      } else {
        (_webAuthnAuth, _requireUserVerification, _x, _y, _metadata) =
          abi.decode(_signature[1:], (WebAuthn.WebAuthnAuth, bool, bytes32, bytes32, bytes32));
      }
    }
  }

  /// @inheritdoc ISapientCompact
  function recoverSapientSignatureCompact(bytes32 _digest, bytes calldata _signature) external view returns (bytes32) {
    (
      WebAuthn.WebAuthnAuth memory _webAuthnAuth,
      bool _requireUserVerification,
      bytes32 _x,
      bytes32 _y,
      bytes32 _metadata
    ) = _decodeSignature(_signature);

    if (!WebAuthn.verify(abi.encodePacked(_digest), _requireUserVerification, _webAuthnAuth, _x, _y)) {
      revert InvalidPasskeySignature(_webAuthnAuth, _requireUserVerification, _x, _y);
    }

    return _rootForPasskey(_requireUserVerification, _x, _y, _metadata);
  }

}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

