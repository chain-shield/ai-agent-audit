
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import "./Wallet.sol";

/// @title Factory
/// @author Agustin Aguilar, Michael Standen
/// @notice Factory for deploying wallets
contract Factory {

  /// @notice Error thrown when the deployment fails
  error DeployFailed(address _mainModule, bytes32 _salt);

  /// @notice Deploy a new wallet instance
  /// @param _mainModule Address of the main module to be used by the wallet
  /// @param _salt Salt used to generate the wallet, which is the imageHash of the wallet's configuration.
  /// @dev It is recommended to not have more than 200 signers as opcode repricing could make transactions impossible to execute as all the signers must be passed for each transaction.
  function deploy(address _mainModule, bytes32 _salt) public payable returns (address _contract) {
    bytes memory code = abi.encodePacked(Wallet.creationCode, uint256(uint160(_mainModule)));
    assembly {
      _contract := create2(callvalue(), add(code, 32), mload(code), _salt)
    }
    if (_contract == address(0)) {
      revert DeployFailed(_mainModule, _salt);
    }
  }

}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES

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

