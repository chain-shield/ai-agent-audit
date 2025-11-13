
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES

## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

