
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

abstract contract BondingCurve {
    function viewAveragePriceInX(address token, uint256 deltaY, bool isBuy) public view virtual returns (uint256);

    // slither-disable-next-line naming-convention
    function getAverageCostInY(address token, uint256 x_0, uint256 x_1) public virtual returns (uint256);

    // slither-disable-next-line naming-convention
    function viewAverageCostInY(address token, uint256 x_0, uint256 x_1) public view virtual returns (uint256);
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES

## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

