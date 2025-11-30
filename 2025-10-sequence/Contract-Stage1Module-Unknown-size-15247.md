
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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
import { SelfAuth } from "./auth/SelfAuth.sol";
import { IERC1155Receiver } from "./interfaces/IERC1155Receiver.sol";
import { IERC223Receiver } from "./interfaces/IERC223Receiver.sol";
import { IERC721Receiver } from "./interfaces/IERC721Receiver.sol";
import { IERC777Receiver } from "./interfaces/IERC777Receiver.sol";

/// @title Hooks
/// @author Agustin Aguilar, Michael Standen
/// @notice Enables extension of the wallet by adding hooks
contract Hooks is SelfAuth, IERC1155Receiver, IERC777Receiver, IERC721Receiver, IERC223Receiver {

  /// @dev keccak256("org.arcadeum.module.hooks.hooks")
  bytes32 private constant HOOKS_KEY = bytes32(0xbe27a319efc8734e89e26ba4bc95f5c788584163b959f03fa04e2d7ab4b9a120);

  /// @notice Emitted when a hook is defined
  event DefinedHook(bytes4 selector, address implementation);

  /// @notice Error thrown when a hook already exists
  error HookAlreadyExists(bytes4 selector);
  /// @notice Error thrown when a hook does not exist
  error HookDoesNotExist(bytes4 selector);

  /// @notice Read a hook
  /// @param selector The selector of the hook
  /// @return implementation The implementation address of the hook
  function readHook(
    bytes4 selector
  ) external view returns (address) {
    return _readHook(selector);
  }

  /// @notice Add a hook
  /// @param selector The selector of the hook
  /// @param implementation The implementation address of the hook
  /// @dev Callable only by the contract itself
  function addHook(bytes4 selector, address implementation) external payable onlySelf {
    if (_readHook(selector) != address(0)) {
      revert HookAlreadyExists(selector);
    }
    _writeHook(selector, implementation);
  }

  /// @notice Remove a hook
  /// @param selector The selector of the hook
  /// @dev Callable only by the contract itself
  function removeHook(
    bytes4 selector
  ) external payable onlySelf {
    if (_readHook(selector) == address(0)) {
      revert HookDoesNotExist(selector);
    }
    _writeHook(selector, address(0));
  }

  function _readHook(
    bytes4 selector
  ) private view returns (address) {
    return address(uint160(uint256(Storage.readBytes32Map(HOOKS_KEY, bytes32(selector)))));
  }

  function _writeHook(bytes4 selector, address implementation) private {
    Storage.writeBytes32Map(HOOKS_KEY, bytes32(selector), bytes32(uint256(uint160(implementation))));
    emit DefinedHook(selector, implementation);
  }

  /// @inheritdoc IERC1155Receiver
  function onERC1155Received(address, address, uint256, uint256, bytes calldata) external pure returns (bytes4) {
    return Hooks.onERC1155Received.selector;
  }

  /// @inheritdoc IERC1155Receiver
  function onERC1155BatchReceived(
    address,
    address,
    uint256[] calldata,
    uint256[] calldata,
    bytes calldata
  ) external pure returns (bytes4) {
    return Hooks.onERC1155BatchReceived.selector;
  }

  /// @inheritdoc IERC777Receiver
  function tokensReceived(
    address operator,
    address from,
    address to,
    uint256 amount,
    bytes calldata data,
    bytes calldata operatorData
  ) external { }

  /// @inheritdoc IERC721Receiver
  function onERC721Received(address, address, uint256, bytes calldata) external pure returns (bytes4) {
    return Hooks.onERC721Received.selector;
  }

  /// @inheritdoc IERC223Receiver
  function tokenReceived(address, uint256, bytes calldata) external pure returns (bytes4) {
    return Hooks.tokenReceived.selector;
  }

  /// @notice Fallback function
  /// @dev Handles delegate calls to hooks
  fallback() external payable {
    if (msg.data.length >= 4) {
      address target = _readHook(bytes4(msg.data));
      if (target != address(0)) {
        (bool success, bytes memory result) = target.delegatecall(msg.data);
        assembly {
          if iszero(success) { revert(add(result, 32), mload(result)) }
          return(add(result, 32), mload(result))
        }
      }
    }
  }

  /// @notice Receive native tokens
  receive() external payable { }

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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

interface IEntryPoint {

  function depositTo(
    address account
  ) external payable;

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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

/**
 * User Operation struct
 * @param sender                - The sender account of this request.
 * @param nonce                 - Unique value the sender uses to verify it is not a replay.
 * @param initCode              - If set, the account contract will be created by this constructor
 * @param callData              - The method call to execute on this account.
 * @param accountGasLimits      - Packed gas limits for validateUserOp and gas limit passed to the callData method call.
 * @param preVerificationGas    - Gas not calculated by the handleOps method, but added to the gas paid.
 *                                Covers batch overhead.
 * @param gasFees               - packed gas fields maxPriorityFeePerGas and maxFeePerGas - Same as EIP-1559 gas parameters.
 * @param paymasterAndData      - If set, this field holds the paymaster address, verification gas limit, postOp gas limit and paymaster-specific extra data
 *                                The paymaster will pay for the transaction instead of the sender.
 * @param signature             - Sender-verified signature over the entire request, the EntryPoint address and the chain ID.
 */
struct PackedUserOperation {
  address sender;
  uint256 nonce;
  bytes initCode;
  bytes callData;
  bytes32 accountGasLimits;
  uint256 preVerificationGas;
  bytes32 gasFees;
  bytes paymasterAndData;
  bytes signature;
}

interface IAccount {

  /**
   * Validate user's signature and nonce
   * the entryPoint will make the call to the recipient only if this validation call returns successfully.
   * signature failure should be reported by returning SIG_VALIDATION_FAILED (1).
   * This allows making a "simulation call" without a valid signature
   * Other failures (e.g. nonce mismatch, or invalid signature format) should still revert to signal failure.
   *
   * @dev Must validate caller is the entryPoint.
   *      Must validate the signature and nonce
   * @param userOp              - The operation that is about to be executed.
   * @param userOpHash          - Hash of the user's request data. can be used as the basis for signature.
   * @param missingAccountFunds - Missing funds on the account's deposit in the entrypoint.
   *                              This is the minimum amount to transfer to the sender(entryPoint) to be
   *                              able to make the call. The excess is left as a deposit in the entrypoint
   *                              for future calls. Can be withdrawn anytime using "entryPoint.withdrawTo()".
   *                              In case there is a paymaster in the request (or the current deposit is high
   *                              enough), this value will be zero.
   * @return validationData       - Packaged ValidationData structure. use `_packValidationData` and
   *                              `_unpackValidationData` to encode and decode.
   *                              <20-byte> aggregatorOrSigFail - 0 for valid signature, 1 to mark signature failure,
   *                                 otherwise, an address of an "aggregator" contract.
   *                              <6-byte> validUntil - Last timestamp this operation is valid at, or 0 for "indefinitely"
   *                              <6-byte> validAfter - First timestamp this operation is valid
   *                                                    If an account doesn't use time-range, it is enough to
   *                                                    return SIG_VALIDATION_FAILED value (1) for signature failure.
   *                              Note that the validation code cannot use block.timestamp (or block.number) directly.
   */
  function validateUserOp(
    PackedUserOperation calldata userOp,
    bytes32 userOpHash,
    uint256 missingAccountFunds
  ) external returns (uint256 validationData);

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

import { SelfAuth } from "./auth/SelfAuth.sol";

/// @title Implementation
/// @author Agustin Aguilar
/// @notice Manages the implementation address of the proxy contract
contract Implementation is SelfAuth {

  /// @notice Emitted when the implementation is updated
  event ImplementationUpdated(address newImplementation);

  /// @notice Update the implementation
  /// @param _implementation The new implementation
  /// @dev Callable only by the contract itself
  function updateImplementation(
    address _implementation
  ) external payable virtual onlySelf {
    _updateImplementation(_implementation);
  }

  /// @notice Get the implementation
  /// @return implementation The implementation
  function getImplementation() external view virtual returns (address) {
    return _getImplementation();
  }

  function _updateImplementation(
    address _implementation
  ) internal virtual {
    _setImplementation(_implementation);
    emit ImplementationUpdated(_implementation);
  }

  function _setImplementation(
    address _imp
  ) internal {
    assembly {
      sstore(address(), _imp)
    }
  }

  function _getImplementation() internal view returns (address _imp) {
    assembly {
      _imp := sload(address())
    }
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

/// @title IERC1155Receiver
/// @notice Interface for the ERC1155 receiver module
interface IERC1155Receiver {

  /// @notice Called when a single ERC1155 token is transferred to this contract
  /// @param operator The address which initiated the transfer
  /// @param from The address which previously owned the token
  /// @param tokenId The ID of the token being transferred
  /// @param value The amount of token being transferred
  /// @param data Additional data with no specified format
  /// @return magicValue On a success, the selector of the function that was called
  function onERC1155Received(
    address operator,
    address from,
    uint256 tokenId,
    uint256 value,
    bytes calldata data
  ) external returns (bytes4 magicValue);

  /// @notice Called when multiple ERC1155 tokens are transferred to this contract
  /// @param operator The address which initiated the transfer
  /// @param from The address which previously owned the token
  /// @param ids The list of token IDs being transferred
  /// @param values The amounts of each token being transferred
  /// @param data Additional data with no specified format
  /// @return magicValue On a success, the selector of the function that was called
  function onERC1155BatchReceived(
    address operator,
    address from,
    uint256[] calldata ids,
    uint256[] calldata values,
    bytes calldata data
  ) external returns (bytes4 magicValue);

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
pragma solidity ^0.8.18;

/// @title IERC223Receiver
/// @notice Interface for the ERC223 receiver module
interface IERC223Receiver {

  /// @notice Called when ERC223 tokens are received by this contract
  /// @param from The address which previously owned the tokens
  /// @param value The amount of tokens being transferred
  /// @param data Transaction metadata
  /// @return signature The signature of the function to be called
  function tokenReceived(address from, uint256 value, bytes calldata data) external returns (bytes4 signature);

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.18;

/// @title IERC777Receiver
/// @notice Interface for the ERC777 receiver module
interface IERC777Receiver {

  /// @notice Called when tokens are received by this contract
  /// @param operator The address which initiated the transfer
  /// @param from The address which previously owned the tokens
  /// @param to The address which is receiving the tokens
  /// @param amount The amount of tokens being transferred
  /// @param data Additional data with no specified format
  /// @param operatorData Additional data with no specified format
  function tokensReceived(
    address operator,
    address from,
    address to,
    uint256 amount,
    bytes calldata data,
    bytes calldata operatorData
  ) external;

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

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

/*
// Delegate Proxy in Huff
// @title Delegate Proxy
// @notice Implements a proxy using the contract's own address to store the delegate target.
//         Calls with calldata (with or without ETH value) are forwarded to the stored target.
//         Calls sending only ETH without calldata do nothing and return immediately without forwarding.
// @author Agusx1211
#define macro CONSTRUCTOR() = takes (0) returns (0) {
  0x41                   // [code + arg size] (code_size + 32)
  __codeoffset(MAIN)     // [code_start, code + arg size]
  returndatasize         // [0, code_start, code + arg size]
  codecopy               // []

  __codesize(MAIN)       // [code_size]
  dup1                   // [code_size, code_size]
  mload                  // [arg1, code_size]
  address                // [address, arg1, code_size]
  sstore                 // [code_size]

  returndatasize         // [0, code_size]
  return
}

#define macro MAIN() = takes(0) returns(0) {
  returndatasize     // [0]
  returndatasize     // [0, 0]
  calldatasize       // [cs, 0, 0]
  iszero             // [cs == 0, 0, 0]
  callvalue          // [cv, cs == 0, 0, 0]
  mul                // [cv * cs == 0, 0, 0]
  success            // [nr, cv * cs == 0, 0, 0]
  jumpi
    calldatasize     // [cds, 0, 0]
    returndatasize   // [0, cds, 0, 0]
    returndatasize   // [0, 0, cds, 0, 0]
    calldatacopy     // [0, 0]
    returndatasize   // [0, 0, 0]
    calldatasize     // [cds, 0, 0, 0]
    returndatasize   // [0, cds, 0, 0, 0]
    address          // [addr, 0, cds, 0, 0, 0]
    sload            // [imp, 0, cds, 0, 0, 0]
    gas              // [gas, imp, 0, cds, 0, 0, 0]
    delegatecall     // [suc, 0]
    returndatasize   // [rds, suc, 0]
    dup3             // [0, rds, suc, 0]
    dup1             // [0, 0, rds, suc, 0]
    returndatacopy   // [suc, 0]
    swap1            // [0, suc]
    returndatasize   // [rds, 0, suc]
    swap2            // [suc, 0, rds]
    success          // [nr, suc, 0, rds]
    jumpi
      revert
  success:
    return
}
*/

library Wallet {

  bytes internal constant creationCode =
    hex"6041600e3d396021805130553df33d3d36153402601f57363d3d373d363d30545af43d82803e903d91601f57fd5bf3";

}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.18;

/// @title IERC721Receiver
/// @notice Interface for the ERC721 receiver module
interface IERC721Receiver {

  /// @notice Called when a single ERC721 token is transferred to this contract
  /// @param operator The address which initiated the transfer
  /// @param from The address which previously owned the token
  /// @param tokenId The ID of the token being transferred
  /// @param data Additional data with no specified format
  /// @return magicValue On a success, the selector of the function that was called
  function onERC721Received(
    address operator,
    address from,
    uint256 tokenId,
    bytes calldata data
  ) external returns (bytes4 magicValue);

}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
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

