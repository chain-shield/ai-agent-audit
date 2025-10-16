
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

