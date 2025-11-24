
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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

