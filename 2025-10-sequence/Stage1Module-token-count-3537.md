
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

