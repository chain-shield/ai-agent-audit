
## *MAIN TARGET CONTRACT* TO REVIEW

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

