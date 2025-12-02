
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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

/// @title IAuth
/// @author Agustin Aguilar, Michael Standen, William Hua
/// @notice Internal interface for the auth modules
abstract contract IAuth {

  function _isValidImage(
    bytes32 imageHash
  ) internal view virtual returns (bool isValid);

  function _updateImageHash(
    bytes32 imageHash
  ) internal virtual;

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { LibBytes } from "../../utils/LibBytes.sol";
import { LibOptim } from "../../utils/LibOptim.sol";
import { Payload } from "../Payload.sol";

import { ICheckpointer, Snapshot } from "../interfaces/ICheckpointer.sol";
import { IERC1271, IERC1271_MAGIC_VALUE_HASH } from "../interfaces/IERC1271.sol";
import { ISapient, ISapientCompact } from "../interfaces/ISapient.sol";

using LibBytes for bytes;
using Payload for Payload.Decoded;

/// @title BaseSig
/// @author Agustin Aguilar, Michael Standen, William Hua, Shun Kakinoki
/// @notice Library for recovering signatures from the base-auth payload
library BaseSig {

  uint256 internal constant FLAG_SIGNATURE_HASH = 0;
  uint256 internal constant FLAG_ADDRESS = 1;
  uint256 internal constant FLAG_SIGNATURE_ERC1271 = 2;
  uint256 internal constant FLAG_NODE = 3;
  uint256 internal constant FLAG_BRANCH = 4;
  uint256 internal constant FLAG_SUBDIGEST = 5;
  uint256 internal constant FLAG_NESTED = 6;
  uint256 internal constant FLAG_SIGNATURE_ETH_SIGN = 7;
  uint256 internal constant FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST = 8;
  uint256 internal constant FLAG_SIGNATURE_SAPIENT = 9;
  uint256 internal constant FLAG_SIGNATURE_SAPIENT_COMPACT = 10;

  /// @notice Error thrown when the weight is too low for a chained signature
  error LowWeightChainedSignature(bytes _signature, uint256 _threshold, uint256 _weight);
  /// @notice Error thrown when the ERC1271 signature is invalid
  error InvalidERC1271Signature(bytes32 _opHash, address _signer, bytes _signature);
  /// @notice Error thrown when the checkpoint order is wrong
  error WrongChainedCheckpointOrder(uint256 _nextCheckpoint, uint256 _checkpoint);
  /// @notice Error thrown when the snapshot is unused
  error UnusedSnapshot(Snapshot _snapshot);
  /// @notice Error thrown when the signature flag is invalid
  error InvalidSignatureFlag(uint256 _flag);

  function _leafForAddressAndWeight(address _addr, uint256 _weight) internal pure returns (bytes32) {
    return keccak256(abi.encodePacked("Sequence signer:\n", _addr, _weight));
  }

  function _leafForNested(bytes32 _node, uint256 _threshold, uint256 _weight) internal pure returns (bytes32) {
    return keccak256(abi.encodePacked("Sequence nested config:\n", _node, _threshold, _weight));
  }

  function _leafForSapient(address _addr, uint256 _weight, bytes32 _imageHash) internal pure returns (bytes32) {
    return keccak256(abi.encodePacked("Sequence sapient config:\n", _addr, _weight, _imageHash));
  }

  function _leafForHardcodedSubdigest(
    bytes32 _subdigest
  ) internal pure returns (bytes32) {
    return keccak256(abi.encodePacked("Sequence static digest:\n", _subdigest));
  }

  function _leafForAnyAddressSubdigest(
    bytes32 _anyAddressSubdigest
  ) internal pure returns (bytes32) {
    return keccak256(abi.encodePacked("Sequence any address subdigest:\n", _anyAddressSubdigest));
  }

  function recover(
    Payload.Decoded memory _payload,
    bytes calldata _signature,
    bool _ignoreCheckpointer,
    address _checkpointer
  ) internal view returns (uint256 threshold, uint256 weight, bytes32 imageHash, uint256 checkpoint, bytes32 opHash) {
    // First byte is the signature flag
    (uint256 signatureFlag, uint256 rindex) = _signature.readFirstUint8();

    // The possible flags are:
    // - 0000 00XX (bits [1..0]): signature type (00 = normal, 01/11 = chained, 10 = no chain id)
    // - 000X XX00 (bits [4..2]): checkpoint size (00 = 0 bytes, 001 = 1 byte, 010 = 2 bytes...)
    // - 00X0 0000 (bit [5]): threshold size (0 = 1 byte, 1 = 2 bytes)
    // - 0X00 0000 (bit [6]): set if imageHash checkpointer is used
    // - X000 0000 (bit [7]): reserved by base-auth

    Snapshot memory snapshot;

    // Recover the imageHash checkpointer if any
    // but checkpointer passed as argument takes precedence
    // since it can be defined by the chained signatures
    if (signatureFlag & 0x40 == 0x40 && _checkpointer == address(0)) {
      // Override the checkpointer
      // not ideal, but we don't have much room in the stack
      (_checkpointer, rindex) = _signature.readAddress(rindex);

      if (!_ignoreCheckpointer) {
        // Next 3 bytes determine the checkpointer data size
        uint256 checkpointerDataSize;
        (checkpointerDataSize, rindex) = _signature.readUint24(rindex);

        // Read the checkpointer data
        bytes memory checkpointerData = _signature[rindex:rindex + checkpointerDataSize];

        // Call the middleware
        snapshot = ICheckpointer(_checkpointer).snapshotFor(address(this), checkpointerData);

        rindex += checkpointerDataSize;
      }
    }

    // If signature type is 01 or 11 we do a chained signature
    if (signatureFlag & 0x01 == 0x01) {
      return recoverChained(_payload, _checkpointer, snapshot, _signature[rindex:]);
    }

    // If the signature type is 10 we do a no chain id signature
    _payload.noChainId = signatureFlag & 0x02 == 0x02;

    {
      // Recover the checkpoint using the size defined by the flag
      uint256 checkpointSize = (signatureFlag & 0x1c) >> 2;
      (checkpoint, rindex) = _signature.readUintX(rindex, checkpointSize);
    }

    // Recover the threshold, using the flag for the size
    {
      uint256 thresholdSize = ((signatureFlag & 0x20) >> 5) + 1;
      (threshold, rindex) = _signature.readUintX(rindex, thresholdSize);
    }

    // Recover the tree
    opHash = _payload.hash();
    (weight, imageHash) = recoverBranch(_payload, opHash, _signature[rindex:]);

    imageHash = LibOptim.fkeccak256(imageHash, bytes32(threshold));
    imageHash = LibOptim.fkeccak256(imageHash, bytes32(checkpoint));
    imageHash = LibOptim.fkeccak256(imageHash, bytes32(uint256(uint160(_checkpointer))));

    // If the snapshot is used, either the imageHash must match
    // or the checkpoint must be greater than the snapshot checkpoint
    if (snapshot.imageHash != bytes32(0) && snapshot.imageHash != imageHash && checkpoint <= snapshot.checkpoint) {
      revert UnusedSnapshot(snapshot);
    }
  }

  function recoverChained(
    Payload.Decoded memory _payload,
    address _checkpointer,
    Snapshot memory _snapshot,
    bytes calldata _signature
  ) internal view returns (uint256 threshold, uint256 weight, bytes32 imageHash, uint256 checkpoint, bytes32 opHash) {
    Payload.Decoded memory linkedPayload;
    linkedPayload.kind = Payload.KIND_CONFIG_UPDATE;

    uint256 rindex;
    uint256 prevCheckpoint = type(uint256).max;

    while (rindex < _signature.length) {
      uint256 nrindex;

      {
        uint256 sigSize;
        (sigSize, rindex) = _signature.readUint24(rindex);
        nrindex = sigSize + rindex;
      }

      address checkpointer = nrindex == _signature.length ? _checkpointer : address(0);

      if (prevCheckpoint == type(uint256).max) {
        (threshold, weight, imageHash, checkpoint, opHash) =
          recover(_payload, _signature[rindex:nrindex], true, checkpointer);
      } else {
        (threshold, weight, imageHash, checkpoint,) =
          recover(linkedPayload, _signature[rindex:nrindex], true, checkpointer);
      }

      if (weight < threshold) {
        revert LowWeightChainedSignature(_signature[rindex:nrindex], threshold, weight);
      }
      rindex = nrindex;

      if (_snapshot.imageHash == imageHash) {
        _snapshot.imageHash = bytes32(0);
      }

      if (checkpoint >= prevCheckpoint) {
        revert WrongChainedCheckpointOrder(checkpoint, prevCheckpoint);
      }

      linkedPayload.imageHash = imageHash;
      prevCheckpoint = checkpoint;
    }

    if (_snapshot.imageHash != bytes32(0) && checkpoint <= _snapshot.checkpoint) {
      revert UnusedSnapshot(_snapshot);
    }
  }

  function recoverBranch(
    Payload.Decoded memory _payload,
    bytes32 _opHash,
    bytes calldata _signature
  ) internal view returns (uint256 weight, bytes32 root) {
    unchecked {
      uint256 rindex;

      // Iterate until the image is completed
      while (rindex < _signature.length) {
        // The first byte is half flag (the top nibble)
        // and the second set of 4 bits can freely be used by the part

        // Read next item type
        uint256 firstByte;
        (firstByte, rindex) = _signature.readUint8(rindex);

        // The top 4 bits are the flag
        uint256 flag = (firstByte & 0xf0) >> 4;

        // Signature hash (0x00)
        if (flag == FLAG_SIGNATURE_HASH) {
          // Free bits layout:
          // - bits [3..0]: Weight (0000 = dynamic, 0001 = 1, ..., 1111 = 15)
          // We read 64 bytes for an ERC-2098 compact signature (r, yParityAndS).
          // The top bit of yParityAndS is yParity, the remaining 255 bits are s.

          uint8 addrWeight = uint8(firstByte & 0x0f);
          if (addrWeight == 0) {
            (addrWeight, rindex) = _signature.readUint8(rindex);
          }

          bytes32 r;
          bytes32 s;
          uint8 v;
          (r, s, v, rindex) = _signature.readRSVCompact(rindex);

          address addr = ecrecover(_opHash, v, r, s);

          weight += addrWeight;
          bytes32 node = _leafForAddressAndWeight(addr, addrWeight);
          root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
          continue;
        }

        // Address (0x01) (without signature)
        if (flag == FLAG_ADDRESS) {
          // Free bits layout:
          // - bits [3..0]: Weight (0000 = dynamic, 0001 = 1, 0010 = 2, ...)

          // Read weight
          uint8 addrWeight = uint8(firstByte & 0x0f);
          if (addrWeight == 0) {
            (addrWeight, rindex) = _signature.readUint8(rindex);
          }

          // Read address
          address addr;
          (addr, rindex) = _signature.readAddress(rindex);

          // Compute the merkle root WITHOUT adding the weight
          bytes32 node = _leafForAddressAndWeight(addr, addrWeight);
          root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
          continue;
        }

        // Signature ERC1271 (0x02)
        if (flag == FLAG_SIGNATURE_ERC1271) {
          // Free bits layout:
          // - XX00 : Signature size size (00 = 0 byte, 01 = 1 byte, 10 = 2 bytes, 11 = 3 bytes)
          // - 00XX : Weight (00 = dynamic, 01 = 1, 10 = 2, 11 = 3)

          // Read weight
          uint8 addrWeight = uint8(firstByte & 0x03);
          if (addrWeight == 0) {
            (addrWeight, rindex) = _signature.readUint8(rindex);
          }

          // Read signer
          address addr;
          (addr, rindex) = _signature.readAddress(rindex);

          // Read signature size
          uint256 sizeSize = uint8(firstByte & 0x0c) >> 2;
          uint256 size;
          (size, rindex) = _signature.readUintX(rindex, sizeSize);

          // Read dynamic size signature
          uint256 nrindex = rindex + size;

          // Call the ERC1271 contract to check if the signature is valid
          if (IERC1271(addr).isValidSignature(_opHash, _signature[rindex:nrindex]) != IERC1271_MAGIC_VALUE_HASH) {
            revert InvalidERC1271Signature(_opHash, addr, _signature[rindex:nrindex]);
          }
          rindex = nrindex;
          // Add the weight and compute the merkle root
          weight += addrWeight;
          bytes32 node = _leafForAddressAndWeight(addr, addrWeight);
          root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
          continue;
        }

        // Node (0x03)
        if (flag == FLAG_NODE) {
          // Free bits left unused

          // Read node hash
          bytes32 node;
          (node, rindex) = _signature.readBytes32(rindex);
          root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
          continue;
        }

        // Branch (0x04)
        if (flag == FLAG_BRANCH) {
          // Free bits layout:
          // - XXXX : Size size (0000 = 0 byte, 0001 = 1 byte, 0010 = 2 bytes, ...)

          // Read size
          uint256 sizeSize = uint8(firstByte & 0x0f);
          uint256 size;
          (size, rindex) = _signature.readUintX(rindex, sizeSize);

          // Enter a branch of the signature merkle tree
          uint256 nrindex = rindex + size;

          (uint256 nweight, bytes32 node) = recoverBranch(_payload, _opHash, _signature[rindex:nrindex]);
          rindex = nrindex;

          weight += nweight;
          root = LibOptim.fkeccak256(root, node);
          continue;
        }

        // Nested (0x06)
        if (flag == FLAG_NESTED) {
          // Unused free bits:
          // - XX00 : Weight (00 = dynamic, 01 = 1, 10 = 2, 11 = 3)
          // - 00XX : Threshold (00 = dynamic, 01 = 1, 10 = 2, 11 = 3)

          // Enter a branch of the signature merkle tree
          // but with an internal threshold and an external fixed weight
          uint256 externalWeight = uint8(firstByte & 0x0c) >> 2;
          if (externalWeight == 0) {
            (externalWeight, rindex) = _signature.readUint8(rindex);
          }

          uint256 internalThreshold = uint8(firstByte & 0x03);
          if (internalThreshold == 0) {
            (internalThreshold, rindex) = _signature.readUint16(rindex);
          }

          uint256 size;
          (size, rindex) = _signature.readUint24(rindex);
          uint256 nrindex = rindex + size;

          (uint256 internalWeight, bytes32 internalRoot) = recoverBranch(_payload, _opHash, _signature[rindex:nrindex]);
          rindex = nrindex;

          if (internalWeight >= internalThreshold) {
            weight += externalWeight;
          }

          bytes32 node = _leafForNested(internalRoot, internalThreshold, externalWeight);
          root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
          continue;
        }

        // Subdigest (0x05)
        if (flag == FLAG_SUBDIGEST) {
          // Free bits left unused

          // A hardcoded always accepted digest
          // it pushes the weight to the maximum
          bytes32 hardcoded;
          (hardcoded, rindex) = _signature.readBytes32(rindex);
          if (hardcoded == _opHash) {
            weight = type(uint256).max;
          }

          bytes32 node = _leafForHardcodedSubdigest(hardcoded);
          root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
          continue;
        }

        // Signature ETH Sign (0x07)
        if (flag == FLAG_SIGNATURE_ETH_SIGN) {
          // Free bits layout:
          // - bits [3..0]: Weight (0000 = dynamic, 0001 = 1, ..., 1111 = 15)
          // We read 64 bytes for an ERC-2098 compact signature (r, yParityAndS).
          // The top bit of yParityAndS is yParity, the remaining 255 bits are s.

          uint8 addrWeight = uint8(firstByte & 0x0f);
          if (addrWeight == 0) {
            (addrWeight, rindex) = _signature.readUint8(rindex);
          }

          bytes32 r;
          bytes32 s;
          uint8 v;
          (r, s, v, rindex) = _signature.readRSVCompact(rindex);

          address addr = ecrecover(keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", _opHash)), v, r, s);

          weight += addrWeight;
          bytes32 node = _leafForAddressAndWeight(addr, addrWeight);
          root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
          continue;
        }

        // Signature Any address subdigest (0x08)
        // similar to subdigest, but allows for counter-factual payloads
        if (flag == FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST) {
          // Free bits left unused

          // A hardcoded always accepted digest
          // it pushes the weight to the maximum
          bytes32 hardcoded;
          (hardcoded, rindex) = _signature.readBytes32(rindex);
          bytes32 anyAddressOpHash = _payload.hashFor(address(0));
          if (hardcoded == anyAddressOpHash) {
            weight = type(uint256).max;
          }

          bytes32 node = _leafForAnyAddressSubdigest(hardcoded);
          root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
          continue;
        }

        // Signature Sapient (0x09)
        if (flag == FLAG_SIGNATURE_SAPIENT) {
          // Free bits layout:
          // - XX00 : Signature size size (00 = 0 byte, 01 = 1 byte, 10 = 2 bytes, 11 = 3 bytes)
          // - 00XX : Weight (00 = dynamic, 01 = 1, 10 = 2, 11 = 3)

          // Read signer and weight
          uint8 addrWeight = uint8(firstByte & 0x03);
          if (addrWeight == 0) {
            (addrWeight, rindex) = _signature.readUint8(rindex);
          }

          address addr;
          (addr, rindex) = _signature.readAddress(rindex);

          // Read signature size
          uint256 size;
          {
            uint256 sizeSize = uint8(firstByte & 0x0c) >> 2;
            (size, rindex) = _signature.readUintX(rindex, sizeSize);
          }

          // Read dynamic size signature
          uint256 nrindex = rindex + size;

          // Call the ERC1271 contract to check if the signature is valid
          bytes32 sapientImageHash = ISapient(addr).recoverSapientSignature(_payload, _signature[rindex:nrindex]);
          rindex = nrindex;

          // Add the weight and compute the merkle root
          weight += addrWeight;
          bytes32 node = _leafForSapient(addr, addrWeight, sapientImageHash);
          root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
          continue;
        }

        // Signature Sapient Compact (0x0A)
        if (flag == FLAG_SIGNATURE_SAPIENT_COMPACT) {
          // Free bits layout:
          // - XX00 : Signature size size (00 = 0 byte, 01 = 1 byte, 10 = 2 bytes, 11 = 3 bytes)
          // - 00XX : Weight (00 = dynamic, 01 = 1, 10 = 2, 11 = 3)

          // Read signer and weight
          uint8 addrWeight = uint8(firstByte & 0x03);
          if (addrWeight == 0) {
            (addrWeight, rindex) = _signature.readUint8(rindex);
          }

          address addr;
          (addr, rindex) = _signature.readAddress(rindex);

          // Read signature size
          uint256 sizeSize = uint8(firstByte & 0x0c) >> 2;
          uint256 size;
          (size, rindex) = _signature.readUintX(rindex, sizeSize);

          // Read dynamic size signature
          uint256 nrindex = rindex + size;

          // Call the Sapient contract to check if the signature is valid
          bytes32 sapientImageHash =
            ISapientCompact(addr).recoverSapientSignatureCompact(_opHash, _signature[rindex:nrindex]);
          rindex = nrindex;
          // Add the weight and compute the merkle root
          weight += addrWeight;
          bytes32 node = _leafForSapient(addr, addrWeight, sapientImageHash);
          root = root != bytes32(0) ? LibOptim.fkeccak256(root, node) : node;
          continue;
        }

        revert InvalidSignatureFlag(flag);
      }
    }
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../Payload.sol";

/// @notice Snapshot for a specific wallet
/// @param imageHash Image hash of the wallet
/// @param checkpoint Checkpoint identifier
struct Snapshot {
  bytes32 imageHash;
  uint256 checkpoint;
}

/// @title ICheckpointer
/// @author Agustin Aguilar
/// @notice Interface for the checkpointer module
interface ICheckpointer {

  /// @notice Get the snapshot for a specific wallet
  /// @param _wallet The wallet address
  /// @param _proof The proof
  /// @return snapshot The snapshot
  function snapshotFor(address _wallet, bytes calldata _proof) external view returns (Snapshot memory snapshot);

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

/// @title Storage
/// @author Agustin Aguilar
/// @notice Library for storing data at certain storage slots
library Storage {

  function writeBytes32(bytes32 _key, bytes32 _val) internal {
    assembly {
      sstore(_key, _val)
    }
  }

  function readBytes32(
    bytes32 _key
  ) internal view returns (bytes32 val) {
    assembly {
      val := sload(_key)
    }
  }

  function writeBytes32Map(bytes32 _key, bytes32 _subKey, bytes32 _val) internal {
    bytes32 key = keccak256(abi.encode(_key, _subKey));
    assembly {
      sstore(key, _val)
    }
  }

  function readBytes32Map(bytes32 _key, bytes32 _subKey) internal view returns (bytes32 val) {
    bytes32 key = keccak256(abi.encode(_key, _subKey));
    assembly {
      val := sload(key)
    }
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Storage } from "./Storage.sol";

/// @title Nonce
/// @author Agustin Aguilar
/// @notice Manages the nonce of the wallet
contract Nonce {

  /// @notice Emitted when the nonce is changed
  event NonceChange(uint256 _space, uint256 _newNonce);

  /// @notice Error thrown when the nonce is bad
  error BadNonce(uint256 _space, uint256 _provided, uint256 _current);

  /// @dev keccak256("org.arcadeum.module.calls.nonce")
  bytes32 private constant NONCE_KEY = bytes32(0x8d0bf1fd623d628c741362c1289948e57b3e2905218c676d3e69abee36d6ae2e);

  /// @notice Read the nonce
  /// @param _space The space
  /// @return nonce The nonce
  function readNonce(
    uint256 _space
  ) public view virtual returns (uint256) {
    return uint256(Storage.readBytes32Map(NONCE_KEY, bytes32(_space)));
  }

  function _writeNonce(uint256 _space, uint256 _nonce) internal {
    Storage.writeBytes32Map(NONCE_KEY, bytes32(_space), bytes32(_nonce));
  }

  function _consumeNonce(uint256 _space, uint256 _nonce) internal {
    uint256 currentNonce = readNonce(_space);
    if (currentNonce != _nonce) {
      revert BadNonce(_space, _nonce, currentNonce);
    }

    unchecked {
      uint256 newNonce = _nonce + 1;

      _writeNonce(_space, newNonce);
      emit NonceChange(_space, newNonce);
      return;
    }
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Payload } from "../Payload.sol";

/// @title IPartialAuth
/// @author Agustin Aguilar
/// @notice Interface for the partial auth module
interface IPartialAuth {

  /// @notice Recover the partial signature
  /// @param _payload The payload
  /// @param _signature The signature to recover
  /// @return threshold The signature threshold
  /// @return weight The derived weight
  /// @return isValidImage Whether the image hash is valid
  /// @return imageHash The derived image hash
  /// @return checkpoint The checkpoint identifier
  /// @return opHash The hash of the payload
  function recoverPartialSignature(
    Payload.Decoded calldata _payload,
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
    );

}

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
pragma solidity ^0.8.27;

/// @title SelfAuth
/// @author Agustin Aguilar, Michael Standen
/// @notice Modifier for checking if the caller is the same as the contract
abstract contract SelfAuth {

  /// @notice Error thrown when the caller is not the same as the contract
  error OnlySelf(address _sender);

  modifier onlySelf() {
    if (msg.sender != address(this)) {
      revert OnlySelf(msg.sender);
    }
    _;
  }

}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Storage } from "./Storage.sol";

abstract contract ReentrancyGuard {

  bytes32 private constant _INITIAL_VALUE = bytes32(0);
  bytes32 private constant _NOT_ENTERED = bytes32(uint256(1));
  bytes32 private constant _ENTERED = bytes32(uint256(2));

  /// @dev keccak256("org.sequence.module.reentrancyguard.status")
  bytes32 private constant STATUS_KEY = bytes32(0xfc6e07e3992c7c3694a921dc9e412b6cfe475380556756a19805a9e3ddfe2fde);

  /// @notice Error thrown when a reentrant call is detected
  error ReentrantCall();

  /// @notice Prevents a contract from calling itself, directly or indirectly
  modifier nonReentrant() {
    // On the first call to nonReentrant
    // _status will be _NOT_ENTERED or _INITIAL_VALUE
    if (Storage.readBytes32(STATUS_KEY) == _ENTERED) {
      revert ReentrantCall();
    }

    // Any calls to nonReentrant after this point will fail
    Storage.writeBytes32(STATUS_KEY, _ENTERED);

    _;

    // By storing the original value once again, a refund is triggered (see
    // https://eips.ethereum.org/EIPS/eip-2200)
    // Notice that because constructors are not available
    // we always start with _INITIAL_VALUE, not _NOT_ENTERED
    Storage.writeBytes32(STATUS_KEY, _NOT_ENTERED);
  }

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.18;

bytes4 constant IERC1271_MAGIC_VALUE_HASH = 0x1626ba7e;
bytes4 constant IERC1271_MAGIC_VALUE_BYTES = 0x20c13b0b;

/// @title IERC1271
/// @notice Interface for ERC1271
interface IERC1271 {

  /// @notice Verifies whether the provided signature is valid with respect to the provided hash
  /// @dev MUST return the correct magic value if the signature provided is valid for the provided hash
  ///   > The bytes4 magic value to return when signature is valid is 0x1626ba7e : bytes4(keccak256("isValidSignature(bytes32,bytes)")
  ///   > This function MAY modify Ethereum's state
  /// @param _hash keccak256 hash that was signed
  /// @param _signature Signature byte array associated with _data
  /// @return magicValue Magic value 0x1626ba7e if the signature is valid and 0x0 otherwise
  function isValidSignature(bytes32 _hash, bytes calldata _signature) external view returns (bytes4 magicValue);

}

/// @title IERC1271Data
/// @notice Deprecated interface for ERC1271 using bytes instead of bytes32
interface IERC1271Data {

  /// @notice Verifies whether the provided signature is valid with respect to the provided hash
  /// @dev MUST return the correct magic value if the signature provided is valid for the provided hash
  ///   > The bytes4 magic value to return when signature is valid is 0x20c13b0b : bytes4(keccak256("isValidSignature(bytes,bytes)")
  ///   > This function MAY modify Ethereum's state
  /// @param _data Data that was signed
  /// @param _signature Signature byte array associated with _data
  /// @return magicValue Magic value 0x20c13b0b if the signature is valid and 0x0 otherwise
  function isValidSignature(bytes calldata _data, bytes calldata _signature) external view returns (bytes4 magicValue);

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

/// @title IDelegatedExtension
/// @author Agustin Aguilar
/// @notice Interface for the delegated extension module
interface IDelegatedExtension {

  /// @notice Handle a sequence delegate call
  /// @param _opHash The operation hash
  /// @param _startingGas The starting gas
  /// @param _index The index
  /// @param _numCalls The number of calls
  /// @param _space The space
  /// @param _data The data
  function handleSequenceDelegateCall(
    bytes32 _opHash,
    uint256 _startingGas,
    uint256 _index,
    uint256 _numCalls,
    uint256 _space,
    bytes calldata _data
  ) external;

}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
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


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

