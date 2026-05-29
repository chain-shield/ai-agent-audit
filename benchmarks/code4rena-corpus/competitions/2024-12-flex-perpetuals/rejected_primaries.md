# Rejected Primary Findings: Flex Perpetuals

# Using `block.timestamp` as a deadline parameter for `swapExactTokensForTokens` offers no protection

- **Contest:** Flex Perpetuals
- **Slug:** 2024-12-flex-perpetuals
- **Submission:** F-27
- **Submitter:** zanderbyte
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-flex-perpetuals/submissions/F-27
- **Source snapshot:** competitions/2024-12-flex-perpetuals/submissions/raw/F-27.txt

## Brief Summary

The run function of the AerodromeDexter contract uses block.timestamp as the deadline parameter for swapExactTokensForTokens, which is incorrect and dangerous. For example a validator can hold the transaction and include it a later block, causing worse price for the swap and exposing users to MEV opportunities.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# The AerodromeDexter.sol contract doesn't support tokens that revert on large approvals

- **Contest:** Flex Perpetuals
- **Slug:** 2024-12-flex-perpetuals
- **Submission:** F-28
- **Submitter:** alix40
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-flex-perpetuals/submissions/F-28
- **Source snapshot:** competitions/2024-12-flex-perpetuals/submissions/raw/F-28.txt

## Brief Summary

The AerodromeDexter.sol contract doesn't support tokens that revert on large approvals and/or transfers As mentioned in the readme such tokens are in scope

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# IntentHandler.sol has no checks validating that market is active.

- **Contest:** Flex Perpetuals
- **Slug:** 2024-12-flex-perpetuals
- **Submission:** F-30
- **Submitter:** Tumelo_Crypto
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-flex-perpetuals/submissions/F-30
- **Source snapshot:** competitions/2024-12-flex-perpetuals/submissions/raw/F-30.txt

## Brief Summary

Delisted Market transactions can still be executed by the IntentHandler.sol contract

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# The `AerodromeDexter.sol` contract doesn't support fee on transfer tokens

- **Contest:** Flex Perpetuals
- **Slug:** 2024-12-flex-perpetuals
- **Submission:** F-29
- **Submitter:** alix40
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-flex-perpetuals/submissions/F-29
- **Source snapshot:** competitions/2024-12-flex-perpetuals/submissions/raw/F-29.txt

## Brief Summary

In the run() function of the AerodromeDexter.sol contract, the amount to be swapped is transfered directly to the contract. This is how the run function is used inside the flex perp protcol function execute(uint256 _amount, address[] calldata _path) external returns (uint256 _amountOut) { if (_amount == 0) revert SwitchCollateral_BadAmount(); if (_path.length < 2) revert SwitchCollateral_BadPath(); for (uint i = 0; i < _path.length - 1; i++) { (address _tokenIn, address _tokenOut) = (_path[i], _path[i + 1]); IDexter _dexter = dexterOf[_tokenIn][_tokenOut]; // Check if the dexterOf[tokenIn][tokenOut] is registered. if (address(_dexter) == address(0)) revert SwitchCollateralRouter_NotFoundDex...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.
