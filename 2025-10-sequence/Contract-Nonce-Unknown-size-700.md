
## *MAIN TARGET CONTRACT* TO REVIEW

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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

