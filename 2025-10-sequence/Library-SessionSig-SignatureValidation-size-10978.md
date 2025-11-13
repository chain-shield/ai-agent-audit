
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../../modules/Payload.sol";
import { LibBytes } from "../../utils/LibBytes.sol";
import { LibOptim } from "../../utils/LibOptim.sol";
import { SessionErrors } from "./SessionErrors.sol";
import { SessionPermissions } from "./explicit/IExplicitSessionManager.sol";
import { LibPermission, Permission } from "./explicit/Permission.sol";
import { Attestation, LibAttestation } from "./implicit/Attestation.sol";

using LibBytes for bytes;
using LibAttestation for Attestation;

/// @title SessionSig
/// @author Michael Standen, Agustin Aguilar
/// @notice Library for session signatures
library SessionSig {

  uint256 internal constant FLAG_PERMISSIONS = 0;
  uint256 internal constant FLAG_NODE = 1;
  uint256 internal constant FLAG_BRANCH = 2;
  uint256 internal constant FLAG_BLACKLIST = 3;
  uint256 internal constant FLAG_IDENTITY_SIGNER = 4;

  uint256 internal constant MIN_ENCODED_PERMISSION_SIZE = 94;

  /// @notice Call signature for a specific session
  /// @param isImplicit If the call is implicit
  /// @param sessionSigner Address of the session signer
  /// @param sessionPermission Session permission for explicit calls
  /// @param attestation Attestation for implicit calls
  struct CallSignature {
    bool isImplicit;
    address sessionSigner;
    uint8 sessionPermission;
    Attestation attestation;
  }

  /// @notice Decoded signature for a specific session
  /// @param imageHash Derived configuration image hash
  /// @param identitySigner Identity signer address
  /// @param implicitBlacklist Implicit blacklist addresses
  /// @param sessionPermissions Session permissions for each explicit signer
  /// @param callSignatures Call signatures for each call in the payload
  struct DecodedSignature {
    bytes32 imageHash;
    address identitySigner;
    address[] implicitBlacklist;
    SessionPermissions[] sessionPermissions;
    CallSignature[] callSignatures;
  }

  /// @notice Recovers the decoded signature from the encodedSignature bytes.
  /// @dev The encoded layout is conceptually separated into three parts:
  ///  1) Session Configuration
  ///  2) A reusable list of Attestations + their identity signatures (if any implicit calls exist)
  ///  3) Call Signatures (one per call in the payload)
  ///
  /// High-level layout:
  ///  - session_configuration: [uint24 size, <Session Configuration encoded>]
  ///  - attestation_list: [uint8 attestationCount, (Attestation + identitySig) * attestationCount]
  ///    (new section to allow reusing the same Attestation across multiple calls)
  ///  - call_signatures: [<CallSignature encoded>] - Size is payload.calls.length
  ///    - call_signature: [uint8 call_flags, <session_signature>]
  ///      - call_flags: [bool is_implicit (MSB), 7 bits encoded]
  ///      - if call_flags.is_implicit.MSB == 1:
  ///         - attestation_index: [uint8 index into the attestation list (7 bits of the call_flags)]
  ///         - session_signature: [r, s, v (compact)]
  ///      - if call_flags.is_implicit.MSB == 0:
  ///         - session_permission: [uint8 (7 bits of the call_flags)]
  ///         - session_signature: [r, s, v (compact)]
  function recoverSignature(
    Payload.Decoded calldata payload,
    bytes calldata encodedSignature
  ) internal view returns (DecodedSignature memory sig) {
    uint256 pointer = 0;
    bool hasBlacklistInConfig;

    // ----- Session Configuration -----
    {
      // First read the length of the session configuration bytes (uint24)
      uint256 dataSize;
      (dataSize, pointer) = encodedSignature.readUint24(pointer);

      // Recover the session configuration
      (sig, hasBlacklistInConfig) = recoverConfiguration(encodedSignature[pointer:pointer + dataSize]);
      pointer += dataSize;

      // Identity signer must be set
      if (sig.identitySigner == address(0)) {
        revert SessionErrors.InvalidIdentitySigner();
      }
    }

    // ----- Attestations for implicit calls -----
    Attestation[] memory attestationList;
    {
      uint8 attestationCount;
      (attestationCount, pointer) = encodedSignature.readUint8(pointer);
      attestationList = new Attestation[](attestationCount);
      // Parse each attestation and its identity signature, store in memory
      for (uint256 i = 0; i < attestationCount; i++) {
        Attestation memory att;
        (att, pointer) = LibAttestation.fromPacked(encodedSignature, pointer);

        // Read the identity signature that approves this attestation
        {
          bytes32 r;
          bytes32 s;
          uint8 v;
          (r, s, v, pointer) = encodedSignature.readRSVCompact(pointer);

          // Recover the identity signer from the attestation identity signature
          bytes32 attestationHash = att.toHash();
          address recoveredIdentitySigner = ecrecover(attestationHash, v, r, s);
          if (recoveredIdentitySigner != sig.identitySigner) {
            revert SessionErrors.InvalidIdentitySigner();
          }
        }

        attestationList[i] = att;
      }

      // If we have any implicit calls, we must have a blacklist in the configuration
      if (attestationCount > 0 && !hasBlacklistInConfig) {
        revert SessionErrors.InvalidBlacklist();
      }
    }

    // ----- Call Signatures -----
    {
      uint256 callsCount = payload.calls.length;
      sig.callSignatures = new CallSignature[](callsCount);

      for (uint256 i = 0; i < callsCount; i++) {
        CallSignature memory callSignature;

        // Determine signature type
        {
          uint8 flag;
          (flag, pointer) = encodedSignature.readUint8(pointer);
          callSignature.isImplicit = (flag & 0x80) != 0;

          if (callSignature.isImplicit) {
            // Read attestation index from the call_flags
            uint8 attestationIndex = uint8(flag & 0x7f);

            // Check if the attestation index is out of range
            if (attestationIndex >= attestationList.length) {
              revert SessionErrors.InvalidAttestation();
            }

            // Set the attestation
            callSignature.attestation = attestationList[attestationIndex];
          } else {
            // Session permission index is the entire byte, top bit is 0 => no conflict
            callSignature.sessionPermission = flag;
          }
        }

        // Read session signature and recover the signer
        {
          bytes32 r;
          bytes32 s;
          uint8 v;
          (r, s, v, pointer) = encodedSignature.readRSVCompact(pointer);

          bytes32 callHash = hashCallWithReplayProtection(payload, i);
          callSignature.sessionSigner = ecrecover(callHash, v, r, s);
          if (callSignature.sessionSigner == address(0)) {
            revert SessionErrors.InvalidSessionSigner(address(0));
          }
        }

        sig.callSignatures[i] = callSignature;
      }
    }

    return sig;
  }

  /// @notice Recovers the session configuration from the encoded data.
  /// The encoded layout is:
  /// - permissions_count: [uint8]
  /// - permissions_tree_element: [flag, <data>]
  ///   - flag: [uint8]
  ///   - data: [data]
  ///     - if flag == FLAG_PERMISSIONS: [SessionPermissions encoded]
  ///     - if flag == FLAG_NODE: [bytes32 node]
  ///     - if flag == FLAG_BRANCH: [uint256 size, nested encoding...]
  ///     - if flag == FLAG_BLACKLIST: [uint24 blacklist_count, blacklist_addresses...]
  ///     - if flag == FLAG_IDENTITY_SIGNER: [address identity_signer]
  /// @dev A valid configuration must have exactly one identity signer and at most one blacklist.
  function recoverConfiguration(
    bytes calldata encoded
  ) internal pure returns (DecodedSignature memory sig, bool hasBlacklist) {
    uint256 pointer;
    uint256 permissionsCount;

    // Guess maximum permissions size by bytes length
    {
      uint256 maxPermissionsSize = encoded.length / MIN_ENCODED_PERMISSION_SIZE;
      sig.sessionPermissions = new SessionPermissions[](maxPermissionsSize);
    }

    while (pointer < encoded.length) {
      // First byte is the flag (top 4 bits) and additional data (bottom 4 bits)
      uint256 firstByte;
      (firstByte, pointer) = encoded.readUint8(pointer);
      // The top 4 bits are the flag
      uint256 flag = (firstByte & 0xf0) >> 4;

      // Permissions configuration (0x00)
      if (flag == FLAG_PERMISSIONS) {
        SessionPermissions memory nodePermissions;
        uint256 pointerStart = pointer;

        // Read signer
        (nodePermissions.signer, pointer) = encoded.readAddress(pointer);

        // Read chainId
        (nodePermissions.chainId, pointer) = encoded.readUint256(pointer);

        // Read value limit
        (nodePermissions.valueLimit, pointer) = encoded.readUint256(pointer);

        // Read deadline
        (nodePermissions.deadline, pointer) = encoded.readUint64(pointer);

        // Read permissions array
        (nodePermissions.permissions, pointer) = _decodePermissions(encoded, pointer);

        // Update root
        {
          bytes32 permissionHash = _leafHashForPermissions(encoded[pointerStart:pointer]);
          sig.imageHash =
            sig.imageHash != bytes32(0) ? LibOptim.fkeccak256(sig.imageHash, permissionHash) : permissionHash;
        }

        // Push node permissions to the permissions array
        sig.sessionPermissions[permissionsCount++] = nodePermissions;
        continue;
      }

      // Node (0x01)
      if (flag == FLAG_NODE) {
        // Read pre-hashed node
        bytes32 node;
        (node, pointer) = encoded.readBytes32(pointer);

        // Update root
        sig.imageHash = sig.imageHash != bytes32(0) ? LibOptim.fkeccak256(sig.imageHash, node) : node;

        continue;
      }

      // Branch (0x02)
      if (flag == FLAG_BRANCH) {
        // Read branch size
        uint256 size;
        {
          uint256 sizeSize = uint8(firstByte & 0x0f);
          (size, pointer) = encoded.readUintX(pointer, sizeSize);
        }
        // Process branch
        uint256 nrindex = pointer + size;
        (DecodedSignature memory branchSig, bool branchHasBlacklist) = recoverConfiguration(encoded[pointer:nrindex]);
        pointer = nrindex;

        // Store the branch blacklist
        if (branchHasBlacklist) {
          if (hasBlacklist) {
            // Blacklist already set
            revert SessionErrors.InvalidBlacklist();
          }
          hasBlacklist = true;
          sig.implicitBlacklist = branchSig.implicitBlacklist;
        }

        // Store the branch identity signer
        if (branchSig.identitySigner != address(0)) {
          if (sig.identitySigner != address(0)) {
            // Identity signer already set
            revert SessionErrors.InvalidIdentitySigner();
          }
          sig.identitySigner = branchSig.identitySigner;
        }

        // Push all branch permissions to the permissions array
        for (uint256 i = 0; i < branchSig.sessionPermissions.length; i++) {
          sig.sessionPermissions[permissionsCount++] = branchSig.sessionPermissions[i];
        }

        // Update root
        sig.imageHash =
          sig.imageHash != bytes32(0) ? LibOptim.fkeccak256(sig.imageHash, branchSig.imageHash) : branchSig.imageHash;

        continue;
      }

      // Blacklist (0x03)
      if (flag == FLAG_BLACKLIST) {
        if (hasBlacklist) {
          // Blacklist already set
          revert SessionErrors.InvalidBlacklist();
        }
        hasBlacklist = true;

        // Read the blacklist count from the first byte's lower 4 bits
        uint256 blacklistCount = uint256(firstByte & 0x0f);
        if (blacklistCount == 0x0f) {
          // If it's max nibble, read the next 2 bytes for the actual size
          (blacklistCount, pointer) = encoded.readUint16(pointer);
        }
        uint256 pointerStart = pointer;

        // Read the blacklist addresses
        sig.implicitBlacklist = new address[](blacklistCount);
        address previousAddress;
        for (uint256 i = 0; i < blacklistCount; i++) {
          (sig.implicitBlacklist[i], pointer) = encoded.readAddress(pointer);
          if (sig.implicitBlacklist[i] < previousAddress) {
            revert SessionErrors.InvalidBlacklistUnsorted();
          }
          previousAddress = sig.implicitBlacklist[i];
        }

        // Update the root
        bytes32 blacklistHash = _leafHashForBlacklist(encoded[pointerStart:pointer]);
        sig.imageHash = sig.imageHash != bytes32(0) ? LibOptim.fkeccak256(sig.imageHash, blacklistHash) : blacklistHash;

        continue;
      }

      // Identity signer (0x04)
      if (flag == FLAG_IDENTITY_SIGNER) {
        if (sig.identitySigner != address(0)) {
          // Identity signer already set
          revert SessionErrors.InvalidIdentitySigner();
        }
        (sig.identitySigner, pointer) = encoded.readAddress(pointer);

        // Update the root
        bytes32 identitySignerHash = _leafHashForIdentitySigner(sig.identitySigner);
        sig.imageHash =
          sig.imageHash != bytes32(0) ? LibOptim.fkeccak256(sig.imageHash, identitySignerHash) : identitySignerHash;

        continue;
      }

      revert SessionErrors.InvalidNodeType(flag);
    }

    {
      // Update the permissions array length to the actual count
      SessionPermissions[] memory permissions = sig.sessionPermissions;
      assembly {
        mstore(permissions, permissionsCount)
      }
    }

    return (sig, hasBlacklist);
  }

  /// @notice Decodes an array of Permission objects from the encoded data.
  function _decodePermissions(
    bytes calldata encoded,
    uint256 pointer
  ) internal pure returns (Permission[] memory permissions, uint256 newPointer) {
    uint256 length;
    (length, pointer) = encoded.readUint8(pointer);
    permissions = new Permission[](length);
    for (uint256 i = 0; i < length; i++) {
      (permissions[i], pointer) = LibPermission.readPermission(encoded, pointer);
    }
    return (permissions, pointer);
  }

  /// @notice Hashes the encoded session permissions into a leaf node.
  function _leafHashForPermissions(
    bytes calldata encodedPermissions
  ) internal pure returns (bytes32) {
    return keccak256(abi.encodePacked(uint8(FLAG_PERMISSIONS), encodedPermissions));
  }

  /// @notice Hashes the encoded blacklist into a leaf node.
  function _leafHashForBlacklist(
    bytes calldata encodedBlacklist
  ) internal pure returns (bytes32) {
    return keccak256(abi.encodePacked(uint8(FLAG_BLACKLIST), encodedBlacklist));
  }

  /// @notice Hashes the identity signer into a leaf node.
  function _leafHashForIdentitySigner(
    address identitySigner
  ) internal pure returns (bytes32) {
    return keccak256(abi.encodePacked(uint8(FLAG_IDENTITY_SIGNER), identitySigner));
  }

  /// @notice Hashes a call with replay protection.
  /// @dev The replay protection is based on the chainId, space, nonce and index in the payload.
  /// @param payload The payload to hash
  /// @param callIdx The index of the call to hash
  /// @return callHash The hash of the call with replay protection
  function hashCallWithReplayProtection(
    Payload.Decoded calldata payload,
    uint256 callIdx
  ) public view returns (bytes32 callHash) {
    return keccak256(
      abi.encodePacked(
        payload.noChainId ? 0 : block.chainid,
        payload.space,
        payload.nonce,
        callIdx,
        Payload.hashCall(payload.calls[callIdx])
      )
    );
  }

}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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
pragma solidity ^0.8.18;

/// @title LibOptim
/// @author Agustin Aguilar
/// @notice Library for optimized EVM operations
library LibOptim {

  /**
   * @notice Computes the keccak256 hash of two 32-byte inputs.
   * @dev It uses only scratch memory space.
   * @param _a The first 32 bytes of the hash.
   * @param _b The second 32 bytes of the hash.
   * @return c The keccak256 hash of the two 32-byte inputs.
   */
  function fkeccak256(bytes32 _a, bytes32 _b) internal pure returns (bytes32 c) {
    assembly {
      mstore(0, _a)
      mstore(32, _b)
      c := keccak256(0, 64)
    }
  }

  /**
   * @notice Returns the return data from the last call.
   * @return r The return data from the last call.
   */
  function returnData() internal pure returns (bytes memory r) {
    assembly {
      let size := returndatasize()
      r := mload(0x40)
      let start := add(r, 32)
      mstore(0x40, add(start, size))
      mstore(r, size)
      returndatacopy(start, 0, size)
    }
  }

  /**
   * @notice Calls another contract with the given parameters.
   * @dev This method doesn't increase the memory pointer.
   * @param _to The address of the contract to call.
   * @param _val The value to send to the contract.
   * @param _gas The amount of gas to provide for the call.
   * @param _data The data to send to the contract.
   * @return r The success status of the call.
   */
  function call(address _to, uint256 _val, uint256 _gas, bytes memory _data) internal returns (bool r) {
    assembly {
      r := call(_gas, _to, _val, add(_data, 32), mload(_data), 0, 0)
    }
  }

  /**
   * @notice Calls another contract with the given parameters, using delegatecall.
   * @dev This method doesn't increase the memory pointer.
   * @param _to The address of the contract to call.
   * @param _gas The amount of gas to provide for the call.
   * @param _data The data to send to the contract.
   * @return r The success status of the call.
   */
  function delegatecall(address _to, uint256 _gas, bytes memory _data) internal returns (bool r) {
    assembly {
      r := delegatecall(_gas, _to, add(_data, 32), mload(_data), 0, 0)
    }
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { LibBytes } from "../../../utils/LibBytes.sol";
import { ACCEPT_IMPLICIT_REQUEST_MAGIC_PREFIX } from "./ISignalsImplicitMode.sol";

using LibBytes for bytes;

/// @notice Attestation for a specific session
/// @param approvedSigner Address of the approved signer
/// @param identityType Identity type
/// @param issuerHash Hash of the issuer
/// @param audienceHash Hash of the audience
/// @param applicationData Unspecified application data
/// @param authData Auth data
struct Attestation {
  address approvedSigner;
  bytes4 identityType;
  bytes32 issuerHash;
  bytes32 audienceHash;
  bytes applicationData;
  AuthData authData;
}

/// @notice Auth data for an attestation
/// @param redirectUrl Authorization redirect URL
/// @param issuedAt Timestamp of the attestation issuance
struct AuthData {
  string redirectUrl;
  uint64 issuedAt;
}

/// @title LibAttestation
/// @author Michael Standen
/// @notice Library for attestation management
library LibAttestation {

  /// @notice Hashes an attestation
  function toHash(
    Attestation memory attestation
  ) internal pure returns (bytes32) {
    return keccak256(toPacked(attestation));
  }

  /// @notice Decodes an attestation from a packed bytes array
  /// @param encoded The packed bytes array
  /// @param pointer The pointer to the start of the attestation
  /// @return attestation The decoded attestation
  /// @return newPointer The new pointer to the end of the attestation
  function fromPacked(
    bytes calldata encoded,
    uint256 pointer
  ) internal pure returns (Attestation memory attestation, uint256 newPointer) {
    newPointer = pointer;
    (attestation.approvedSigner, newPointer) = encoded.readAddress(newPointer);
    (attestation.identityType, newPointer) = encoded.readBytes4(newPointer);
    (attestation.issuerHash, newPointer) = encoded.readBytes32(newPointer);
    (attestation.audienceHash, newPointer) = encoded.readBytes32(newPointer);
    // Application data (arbitrary bytes)
    uint256 dataSize;
    (dataSize, newPointer) = encoded.readUint24(newPointer);
    attestation.applicationData = encoded[newPointer:newPointer + dataSize];
    newPointer += dataSize;
    // Auth data
    (attestation.authData, newPointer) = fromPackedAuthData(encoded, newPointer);
    return (attestation, newPointer);
  }

  /// @notice Decodes the auth data from a packed bytes
  /// @param encoded The packed bytes containing the auth data
  /// @param pointer The pointer to the start of the auth data within the encoded data
  /// @return authData The decoded auth data
  /// @return newPointer The pointer to the end of the auth data within the encoded data
  function fromPackedAuthData(
    bytes calldata encoded,
    uint256 pointer
  ) internal pure returns (AuthData memory authData, uint256 newPointer) {
    uint24 redirectUrlLength;
    (redirectUrlLength, pointer) = encoded.readUint24(pointer);
    authData.redirectUrl = string(encoded[pointer:pointer + redirectUrlLength]);
    pointer += redirectUrlLength;
    (authData.issuedAt, pointer) = encoded.readUint64(pointer);
    return (authData, pointer);
  }

  /// @notice Encodes an attestation into a packed bytes array
  /// @param attestation The attestation to encode
  /// @return encoded The packed bytes array
  function toPacked(
    Attestation memory attestation
  ) internal pure returns (bytes memory encoded) {
    return abi.encodePacked(
      attestation.approvedSigner,
      attestation.identityType,
      attestation.issuerHash,
      attestation.audienceHash,
      uint24(attestation.applicationData.length),
      attestation.applicationData,
      toPackAuthData(attestation.authData)
    );
  }

  /// @notice Encodes the auth data into a packed bytes array
  /// @param authData The auth data to encode
  /// @return encoded The packed bytes array
  function toPackAuthData(
    AuthData memory authData
  ) internal pure returns (bytes memory encoded) {
    return abi.encodePacked(uint24(bytes(authData.redirectUrl).length), bytes(authData.redirectUrl), authData.issuedAt);
  }

  /// @notice Generates the implicit request magic return value
  /// @param attestation The attestation
  /// @param wallet The wallet
  /// @return magic The expected implicit request magic
  function generateImplicitRequestMagic(Attestation memory attestation, address wallet) internal pure returns (bytes32) {
    return keccak256(
      abi.encodePacked(ACCEPT_IMPLICIT_REQUEST_MAGIC_PREFIX, wallet, attestation.audienceHash, attestation.issuerHash)
    );
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

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../../../modules/Payload.sol";
import { Attestation } from "./Attestation.sol";

/// @dev Magic prefix for the implicit request
bytes32 constant ACCEPT_IMPLICIT_REQUEST_MAGIC_PREFIX = keccak256(abi.encodePacked("acceptImplicitRequest"));

/// @title ISignalsImplicitMode
/// @author Agustin Aguilar, Michael Standen
/// @notice Interface for the contracts that support implicit mode validation
interface ISignalsImplicitMode {

  /// @notice Determines if an implicit request is valid
  /// @param wallet The wallet's address
  /// @param attestation The attestation data
  /// @param call The call to validate
  /// @return magic The hash of the implicit request if valid
  function acceptImplicitRequest(
    address wallet,
    Attestation calldata attestation,
    Payload.Call calldata call
  ) external view returns (bytes32 magic);

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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

