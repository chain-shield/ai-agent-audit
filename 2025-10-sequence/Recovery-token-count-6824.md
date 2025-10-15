
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../Payload.sol";

/// @title ISapient
/// @author Agustin Aguilar, Michael Standen
/// @notice Sapient signers take an explicit payload and return their own "imageHash" as result
/// @dev The consumer of this signer must validate if the imageHash is valid or not, for the desired configuration
interface ISapient {

  /// @notice Recovers the image hash of a given signature
  /// @param payload The payload to recover the signature from
  /// @param signature The signature to recover the image hash from
  /// @return imageHash The recovered image hash
  function recoverSapientSignature(
    Payload.Decoded calldata payload,
    bytes calldata signature
  ) external view returns (bytes32 imageHash);

}

/// @title ISapientCompact
/// @author Agustin Aguilar, Michael Standen
/// @notice Sapient signers take a compacted payload and return their own "imageHash" as result
/// @dev The consumer of this signer must validate if the imageHash is valid or not, for the desired configuration
interface ISapientCompact {

  /// @notice Recovers the image hash of a given signature, using a hashed payload
  /// @param digest The digest of the payload
  /// @param signature The signature to recover the image hash from
  /// @return imageHash The recovered image hash
  function recoverSapientSignatureCompact(
    bytes32 digest,
    bytes calldata signature
  ) external view returns (bytes32 imageHash);

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


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

