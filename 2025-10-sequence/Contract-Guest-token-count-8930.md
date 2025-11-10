
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { Calls } from "./modules/Calls.sol";
import { Payload } from "./modules/Payload.sol";

import { LibBytes } from "./utils/LibBytes.sol";
import { LibOptim } from "./utils/LibOptim.sol";

/// @title Guest
/// @author Agustin Aguilar, William Hua, Michael Standen
/// @notice Guest for dispatching calls
contract Guest {

  using LibBytes for bytes;

  /// @notice Error thrown when a delegate call is not allowed
  error DelegateCallNotAllowed(uint256 index);

  /// @notice Fallback function
  /// @dev Dispatches the guest call
  fallback() external payable {
    Payload.Decoded memory decoded = Payload.fromPackedCalls(msg.data);
    bytes32 opHash = Payload.hash(decoded);
    _dispatchGuest(decoded, opHash);
  }

  function _dispatchGuest(Payload.Decoded memory _decoded, bytes32 _opHash) internal {
    bool errorFlag = false;

    uint256 numCalls = _decoded.calls.length;
    for (uint256 i = 0; i < numCalls; i++) {
      Payload.Call memory call = _decoded.calls[i];

      // Skip onlyFallback calls if no error occurred
      if (call.onlyFallback && !errorFlag) {
        emit Calls.CallSkipped(_opHash, i);
        continue;
      }

      // Reset the error flag
      // onlyFallback calls only apply when the immediately preceding transaction fails
      errorFlag = false;

      uint256 gasLimit = call.gasLimit;
      if (gasLimit != 0 && gasleft() < gasLimit) {
        revert Calls.NotEnoughGas(_decoded, i, gasleft());
      }

      if (call.delegateCall) {
        revert DelegateCallNotAllowed(i);
      }

      bool success = LibOptim.call(call.to, call.value, gasLimit == 0 ? gasleft() : gasLimit, call.data);
      if (!success) {
        if (call.behaviorOnError == Payload.BEHAVIOR_IGNORE_ERROR) {
          errorFlag = true;
          emit Calls.CallFailed(_opHash, i, LibOptim.returnData());
          continue;
        }

        if (call.behaviorOnError == Payload.BEHAVIOR_REVERT_ON_ERROR) {
          revert Calls.Reverted(_decoded, i, LibOptim.returnData());
        }

        if (call.behaviorOnError == Payload.BEHAVIOR_ABORT_ON_ERROR) {
          emit Calls.CallAborted(_opHash, i, LibOptim.returnData());
          break;
        }
      }

      emit Calls.CallSucceeded(_opHash, i);
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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { SingletonDeployer, console } from "lib/erc2470-libs/script/SingletonDeployer.s.sol";
import { Factory } from "src/Factory.sol";
import { Guest } from "src/Guest.sol";
import { Stage1Module } from "src/Stage1Module.sol";
import { SessionManager } from "src/extensions/sessions/SessionManager.sol";

contract Deploy is SingletonDeployer {

  function run() external {
    uint256 pk = vm.envUint("PRIVATE_KEY");
    address entryPoint = vm.envAddress("ERC4337_ENTRY_POINT_V7");
    if (entryPoint == address(0)) {
      entryPoint = 0x0000000071727De22E5E9d8BAf0edAc6f37da032;
    }

    bytes32 salt = bytes32(0);

    bytes memory initCode = abi.encodePacked(type(Factory).creationCode);
    address factory = _deployIfNotAlready("Factory", initCode, salt, pk);

    initCode = abi.encodePacked(type(Stage1Module).creationCode, abi.encode(factory, entryPoint));
    address stage1Module = _deployIfNotAlready("Stage1Module", initCode, salt, pk);

    console.log("Stage2Module for Stage1Module is", Stage1Module(payable(stage1Module)).STAGE_2_IMPLEMENTATION());

    initCode = abi.encodePacked(type(Guest).creationCode);
    _deployIfNotAlready("Guest", initCode, salt, pk);

    initCode = abi.encodePacked(type(SessionManager).creationCode);
    _deployIfNotAlready("SessionManager", initCode, salt, pk);
  }

}

