# Rejected Primary Findings: Predy

# Missing of the Duplicate Check in `AddPairLogic.sol#addPair()` function

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-105
- **Submitter:** 0xHash
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/105
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-105.md

## Brief Summary

Since the duplicate check is missing in the `AddPairLogic.sol#addPair()` function, multiple trading pairs and corresponding vaults can be created for the same UniswapV3 token pair. As a result, the liquidity of the Predy pool may be dispersed and various negative problems may occur.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# [H-01] Uncontrolled Access in initialize() function leads to Frontrunning Attack and Admin Takeover in `PredyPool.sol`

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-579
- **Submitter:** 0xSurya_Alpha
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/579
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-579.md

## Brief Summary

Due to absence of proper access control of `initialize()` function of `PredyPool.sol`, attacker can frontrun the function and become `Operator`(Admin) of the protocol.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unrestricted External Access to Library Functions in AddPairLogic Allows Unauthorized Parameter Manipulation

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-572
- **Submitter:** 0xabhay
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/572
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-572.md

## Brief Summary

The impact of the vulnerability where a malicious user can directly call the external functions in the `AddPairLogic` library, bypassing intended access controls in the `PredyPool` contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_105_group

# Reversion in _executeOrderV3 Due to Uninitialized Quote Token Mappings in _validateQuoteTokenAddress Function

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-612
- **Submitter:** 0xabhay
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/612
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-612.md

## Brief Summary

_validateQuoteTokenAddress](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/base/BaseMarketUpgradable.sol#L152 Vulnerability details Impact The issue in the `_executeOrderV3` function, where the [_validateQuoteTokenAddress](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/perp/PerpMarketV1.sol#L167) might cause reversion due to outdated or uninitialized quote token mappings.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Handling of safeTransferFrom for WETH on Non-WETH9 Chains Causes Settlement Failures

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-654
- **Submitter:** 0xabhay
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/654
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-654.md

## Brief Summary

The failure to handle the `safeTransferFrom` function correctly can prevent the protocol from executing certain functions on chains that do not use the WETH9 contract. This can lead to: Failed settlements in trading operations. Overall disruption of protocol functionality on affected chains.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inability to Disable allowedUniswapPools Once Enabled

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-603
- **Submitter:** 0xblackskull
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/603
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-603.md

## Brief Summary

The registerPair function in the smart contract allows an operator to register a new trading pair by invoking the AddPairLogic.addPair function. However, there is no mechanism to disable the allowedUniswapPools feature once it has been set to true, which could lead to unwanted or potentially harmful interactions with Uniswap pools.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_59_group

# Incorrect Fee Calculation for Short Positions.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-100
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/100
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-100.md

## Brief Summary

When a trader opens a short position (i.e., `sqrtPerp.amount` is negative), the fees should be negative, representing a cost to the trader. However, due to the way the fee calculation is implemented, the fees for short positions are calculated as positive values, which is counterintuitive and incorrect. Relevant Code: The problematic code is in the `else if` block of the `computePremium` function, where the fee calculation for short positions is handled: [computePremium#L77-L84](https://github.com/code-423n4/2024-05-predy/blob/e96f378007e3dc56b184079f0c0e4fe48a72efaa/src/libraries/PerpFee.sol#L77-L84) The issue lies in the calculation of `growthDiff0` and `growthDiff1`. For short positions,...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incomplete Tuple Return in getAvailableLiquidityAmount Function

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-91
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/91
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-91.md

## Brief Summary

`getAvailableLiquidityAmount` retrieves the liquidity amount for a specific position by calling the `positions` function of the `IUniswapV3Pool` contract. nonetheless, the positions function returns a tuple with multiple values, and the function is only returning the first value of the tuple, which is the `liquidity` amount. According to the UniswapV3 Core documentation, the positions function returns the following values By only returning the liquidity value, the function is ignoring the other values in the tuple, which could lead to issues or unexpected behavior if those values are needed elsewhere in the codebase. By ignoring the other values in the tuple, the function is potentially lim...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent State Vulnerability in `settleUserBalance` Function where the update order of user rebalance entry values and positions can lead to an inconsistent state if the function is interrupted or reverted.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-92
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/92
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-92.md

## Brief Summary

`settleUserBalance` first updates the user's rebalance entry values (`_userStatus.sqrtPerp.baseRebalanceEntryValue` and `_userStatus.sqrtPerp.quoteRebalanceEntryValue`) before updating the pair's rebalance positions and the user's base and stable positions. This order of operations could lead to an inconsistent state if the settleUserBalance is interrupted or reverted after updating the user's rebalance entry values but before updating the positions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Fee and Premium Calculations due to Overwritten `lastFeeGrowth` Values

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-93
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/93
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-93.md

## Brief Summary

In the `updateFeeAndPremiumGrowth`, the code updates the `lastFee0Growth` and `lastFee1Growth` values after calculating the new `fee0Growth` and `fee1Growth` values. Although, in the `saveLastFeeGrowth` function, the code overwrites the `lastFee0Growth` and `lastFee1Growth` values with the current `feeGrowthInside0X128` and `feeGrowthInside1X128` values, effectively discarding the updated values from the `updateFeeAndPremiumGrowth` function. If `saveLastFeeGrowth` is called after `updateFeeAndPremiumGrowth`, the `lastFee0Growth and lastFee1Growth` values will be reset to their previous values, potentially causing incorrect calculations in subsequent calls to `updateFeeAndPremiumGrowth`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Calculation of `closeStableAmount` Leads to Inaccurate Payoff When Partially Closing a Position

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-96
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/96
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-96.md

## Brief Summary

In the `calculateEntry` the calculation of `closeStableAmount` when closing a position partially. In the case where `_positionAmount.abs() >= _tradeAmount.abs()`, which represents a partial position close, the calculation of `closeStableAmount is`: [#L689](https://github.com/code-423n4/2024-05-predy/blob/2fb1e0ec7a52fc06c2e9c8e561bccba84302e4bb/src/libraries/Perp.sol#L689) This calculation assumes that the `_entryValue` is the entry value for the entire position. However, when closing a position partially, the entry value should be proportional to the amount being closed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Liquidity Check in decrease Function Leads to Potential Loss of Funds or Unintended Consequences.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-97
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/97
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-97.md

## Brief Summary

Incorrect check for available liquidity in the `decrease` function could result in the following scenarios: **`Inability to decrease liquidity`** If the available liquidity (`_assetStatus.totalAmount - _assetStatus.borrowedAmount`) is equal to the requested liquidity amount (`_liquidityAmount`), the function will revert with the `NoCFMMLiquidityError`. Means that users will not be able to decrease their liquidity position even when they have enough liquidity available. **`Potential loss of funds`** If the available liquidity is slightly less than the requested liquidity amount (e.g., available liquidity is 99, and requested liquidity is 100), the function will not revert, and the `burn` ope...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Trade Fee Calculation Due to Outdated User Position.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-99
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/99
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-99.md

## Brief Summary

`settleUserFee` first settles the asset interest and rebalance interest, and then settles the trade fee (premium). But for all that, the trade fee settlement might depend on the user's updated position after settling the asset and rebalance interest. The problematic part: [settleUserFee#L46-L55](https://github.com/code-423n4/2024-05-predy/blob/e96f378007e3dc56b184079f0c0e4fe48a72efaa/src/libraries/PerpFee.sol#L46-L55) `settlePremium` function likely calculates the trade fee based on the user's position (`userStatus.sqrtPerp`) before it was updated by settling the asset and rebalance interest. This could lead to incorrect trade fee calculations if the user's position changes after settling t...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# TWAP Lag Leads to Delayed Liquidations

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-277
- **Submitter:** 0xhere2learn
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/277
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-277.md

## Brief Summary

Predy's current TWAP has a 30-minute period. This is fine in most situations and protects against atomic price manipulation in many cases. However, it is also an ineffective tool when determining a position's health in volatile times. Any large price swing would not be immediately recognized, making liquidations impossible. During times like this, timely liquidations are critical to maintain the health of the protocol. Any lag (especially the 30-minute TWAP being used) risks bad debt accruing and jeopardizes the viability of the protocol. With pool creation being permissionless, there are a variety of tokens that will be used, many of which will have volatile price swings. When this happens...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Reserve Implementation Prevents Withdrawals

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-278
- **Submitter:** 0xhere2learn
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/278
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-278.md

## Brief Summary

Currently, there is no reserve implemented to protect lenders from having their withdrawals prevented. Because of this, if there are enough positions, it will take up all the available liquidity for the pool, preventing any lender from withdrawing. Perpetuals can be held indefinitely as long as the position is healthy. Because of this, lenders may never be able to withdraw their collateral, let alone in a timely manner. Other perpetual protocols such as GMX have a reserve factor for each market (pool); this ensures that a percentage of the pool is always available for withdrawal, preventing lenders from having funds unavailable and deterring them from using the protocol. Currently, lenders...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_42_group

# Liquidation Burden on Liquidators

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-279
- **Submitter:** 0xhere2learn
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/279
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-279.md

## Brief Summary

Currently, if a position is significantly unhealthy, the liquidator is in charge of paying back that bad debt. This will come at the expense of the liquidator and disincentivizes users from liquidating unhealthy positions. If these positions are not liquidated in a timely manner, then the positions can accrue more bad debt, hurting the protocol more and more. Part of the risk of being a lender is that positions can take on such a loss that bad debt has accumulated. By putting this burden on the liquidator instead of the lenders, it actually hurts lenders even more. The reason being is that there will be fewer liquidators willing to take on a loss for the sake of the lenders, which leads to...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Counting Unearned Yield When Valuing Collateral Leads to Undercollateralized Pools

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-282
- **Submitter:** 0xhere2learn
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/282
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-282.md

## Brief Summary

Pools can become undercollateralized because unearned yield is prematurely counted upon withdrawals. This is the function used to determine the collateral value, which is then used to determine the amount lenders can withdraw. The collateral value is a combination of `totalCompoundDeposited` scaled by the current `assetScaler` and the `totalNormalDeposited`. The issue is in the first part of this calculation: when `totalCompoundDeposited` is scaled by the current `assetScaler`, it is being scaled in part by unpaid borrowing fees. This means that the collateral value is being inflated by funds that don't yet exist in the pool. With collateral value being inflated, the amount available for wi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# tokens that dont allow zero transfers will halt pool

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-288
- **Submitter:** 0xhere2learn
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/288
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-288.md

## Brief Summary

Because the pools will be permissionless any token can be used as the quote or base. Some tokens do not allow zero value transfers and will revert if attempted to do so. In most every case throughout the protocol there are checks to ensure zero value transfers do not occur. However `_verifyOrder` is called where permit is used to transfer funds to the protocol. It is important to note that `_verifyOrder` is called without and check as to what amount ought to be transferred. As you can see here if the `marginAmountUpdate` less than or equal to zero `_verifyOrderV3` will try and call `permitWitnessTransferFrom` with a value of 0. For certain ERC20 tokens this will lead to a revert. Impact Use...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Loss of creator funds to the smart contract

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-425
- **Submitter:** 0xx_Ninja
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/425
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-425.md

## Brief Summary

Part of the incentives is for creators to earn from depositing liquidity and protocol creator may lose fund to the Pool contract as its not properly implemented In the `PreedyPool::withdrawCreatorRevenue`. Initializes the revenue to 0 and still makes a check before sending out the token

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_49_group

# An attacker can front-run closing position orders to steal users' fund

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-216
- **Submitter:** 3n0ch
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/216
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-216.md

## Brief Summary

By design everyone can close their position by calling `PerpMarketV1::executeOrderV3` with signed order (`order`) and settlement parameters (`settlementParams`). An attacker can front-run a user's order with the same `order` and high `settlementParams.feePrice`, the `fee` will be calculated at then the `fee` will be sent to `settlementParams.sender`, which is the attacker's address As a result, when a user closes a position the attacker can steal his or her fund.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Double accounting for `sqrtRebalanceEntryUpdateStable` and `sqrtRebalanceEntryUpdateUnderlying`

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-573
- **Submitter:** 3n0ch
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/573
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-573.md

## Brief Summary

In gamma trading, the `sqrtRebalanceEntryUpdateStable` and `sqrtRebalanceEntryUpdateUnderlying` are used to make a LP position into Quart asset. First the the `sqrtRebalanceEntryUpdateStable` and `sqrtRebalanceEntryUpdateUnderlying` are added to `sqrtPerp` Then they are also added to `basePool.tokenStatus` and `quotePool.tokenStatus` By doing this, the rebalance interest will be counted twice. First time is from the `basePool.tokenStatus` and `quotePool.tokenStatus` Second time is from the `sqrtPerp` As a result, it would be favorable for the buyer because `sqrtRebalanceEntryUpdateStable` and `sqrtRebalanceEntryUpdateUnderlying` are considered asset in `basePool.tokenStatus` and `quotePool....

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Contract Storage Modification and Arbitrary Code Execution

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-327
- **Submitter:** Ali55
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/327
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-327.md

## Brief Summary

Title: Unchecked Contract Storage Modification and Arbitrary Code Execution Issue: The `PredyPool` contract does not perform appropriate checks when updating contract storage in the `execLiquidationCall` function, allowing an attacker to potentially overwrite critical contract state variables and execute arbitrary code within the contract. This critical vulnerability can lead to a total contract compromise, including unauthorized access, theft of assets, and disruption of the entire ecosystem. Impact: If an attacker successfully exploits this vulnerability, they can take complete control of the contract, modify any storage variable, and execute arbitrary code with the contract's permissions...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `borrowedAmount` does not always add to `_assetStatus.fee{0/1}Growth` and `borrowPremium{0/1}Growth` when `utilization > Constants.SQUART_KINK_UR`

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-548
- **Submitter:** Audinarey
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/548
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-548.md

## Brief Summary

`borrowedAmount` does not contributes to the `_assetStatus.fee{0/`}Growth` and `borrowPremium{0/1}Growth` owing to calculation of spread parameter (`spreadParam`) becuase of a rounding error and as such - the protocol does not deliver promised returns - this breaks core protocol functionality

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Squart positions are rebalanced into a wrong range due to rounding error in tick calculation causing the position to earn less fees

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-551
- **Submitter:** Audinarey
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/551
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-551.md

## Brief Summary

This will lead to a situation where - LP that constitutes a squart position will earn less fees because the liquidity can be reallocated to a range smaller than the defined `rangeSize` and even `rangeThreshold` as shown in the

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Protocol wrongly assumes the utilization rate in some instances

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-115
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/115
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-115.md

## Brief Summary

Assuming `assets == 0` means `0` utilization rate is wrong since it increases protocol's risk of insolvency and heavily favors borrowers instead of ensuring protocol is safe.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Protocol lacks any logic for bad debt socialization which allows _malicious_ tech savvy users to game honest users on the platform since this leads to a _bank-run_ like situation

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-119
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/119
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-119.md

## Brief Summary

Protocol and users would be left with no way to deal with bad debt, considering it's not being socialised amongst users, providing incentives for a _bank-run_ like ideology in users, as once users see that the tracked balance and the real amount of assets in protocol's control does not match they try to withdraw their stake from the protocol which leads to the last set of users not being able to withdraw cause protocol would not have enough assets to cover these withdrawals.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Attempts at liquidation could revert due to an Underflow in `PositionCalculator.calculateMinMargin`

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-124
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/124
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-124.md

## Brief Summary

DOS to valid attempts at liquidating vaults that are in danger, which showcases how protocol's core functionality is broken.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_57_group

# An attacker can always make `PerpMarketV1# predyTradeAfterCallback()` to revert

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-127
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/127
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-127.md

## Brief Summary

DOS to protocol's core functionality as the `PerpMarketV1# predyTradeAfterCallback()` would now revert. > NB: The same bug idea is applicable to the same context where the `_verifyOrder()` routes it's verification via `permit2`, i.e even [this instance](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/gamma/GammaTradeMarket.sol#L444-L456) from `GammaTradeMarket.sol`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Protocol would encounter DOS in multiple core functions due to the strict Access Control on `predyPool.take()`

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-131
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/131
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-131.md

## Brief Summary

DOS to core functionalities that require the success of the query to `predyPool.take`, in the case of this then means that when [` (marginAmountUpdate < 0)`](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/perp/PerpMarketV1.sol#L127) the `PerpMarketV1#predyTradeAfterCallback()` function would revert. > And the same case applicable to other instances where `predyPool.take` is being queried since the `msg.sender` of this query is not going to be the `locker` the `predyPool.take` function always [reverts](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/PredyPool.sol#L329).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# The Gamma Market's trading mechanism is not of the industry standard and is implemented incorrectly cause it hardcodes the maximum acceptable slippage value for all assets even if the volatility/market movement of all integrated assets are way different.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-133
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/133
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-133.md

## Brief Summary

As hinted under the last section of _Proof of Concept_, The issue highlighted in the report concerns the Gamma Market's trading mechanism, specifically its handling of slippage across various assets. The core problem is that the protocol applies a **fixed** 1.5% max slippage tolerance to **all trades**, regardless of the asset's market volatility or the user's preference. This approach is flawed for several reasons: - For newly introduced or highly volatile assets, the fixed 1.5% slippage tolerance can lead to a denial of service (DoS) situation. If the market movement exceeds 1.5%, trades fail, preventing users from participating in the market. This outcome is particularly problematic for...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_187_group

# Inefficient Liquidity Allocation Due to Symmetric Tick Range Assumption in Uniswap V3 Pools

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-266
- **Submitter:** BlockSafe
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/266
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-266.md

## Brief Summary

This assumption forces liquidity to be symmetrically distributed around zero, preventing liquidity providers from placing their liquidity at the most profitable or strategically advantageous price ranges based on market conditions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Decimal Difference Handling Requires int Type for `_decimalsDiff`

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-306
- **Submitter:** BlockSafe
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/306
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-306.md

## Brief Summary

This vulnerability affects the reliability and accuracy of the price feed mechanism, potentially leading to incorrect price calculations or even failures in scenarios where the base token has fewer decimals than the quote token.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Potential Zero Rounding Issue in Settlement Calculation Due to Integer Division

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-311
- **Submitter:** BlockSafe
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/311
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-311.md

## Brief Summary

The impact of this rounding issue is that it may result in the quoteAmount being calculated as zero, which could lead to erroneous or incomplete settlements.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Zero nonce management within the order validation process, which might make the contracts susceptible to replay attacks.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-338
- **Submitter:** BlockSafe
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/338
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-338.md

## Brief Summary

The primary impact of this vulnerability is financial loss due to unintended multiple executions of the same trade. An attacker can exploit this to drain funds from a trader's account by repeatedly submitting the same valid order. This can lead to significant financial loss for the trader and undermine the integrity of the trading platform.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_75_group

# Significant revenue loss due to round off error (accumulatedProtocolRevenue and accumulatedCreatorRevenue)

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-381
- **Submitter:** BlockSafe
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/381
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-381.md

## Brief Summary

Significant revenue loss occurred due to round off error even with the high utilization. Its happened with less decimals token like WBTC, WETH..etc. Here its considered WBTC-DAI Pair. Since WBTC is having 8 decimals , it caused significant reduction of accumulatedProtocolRevenue and accumulatedCreatorRevenue values. Normally users needs to pay at least base interest. But here users no need to pay any fees up to some extent of utilization. On current implementation , protocol revenue is significantly less compared to actual revenue that it should be earned(If round off error negligible) even with the high utilization conditions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_37_group

# Traders can lock all liquidity in a pool.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-476
- **Submitter:** Bob
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/476
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-476.md

## Brief Summary

In `predySettlementCallback()` traders can `take()` everything from quote token pool as long as they return these assets in `predyTradeAfterCallback()`. End result is that all assets from quote token pool move to trader's vault. No funds can actually be stolen from `PredyPool` this way, however this locks all available liquidity in a pool.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_61_group

# M-1: Use of `abi.encodePacked` can lead to hash collision

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-84
- **Submitter:** Chuch
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/84
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-84.md

## Brief Summary

ABI hash collisions pose a significant security risk in Ethereum smart contracts, particularly when using abi.encodePacked for hashing purposes. This vulnerability stems from the way abi.encodePacked concatenates data without padding, which can lead to different sets of data producing the same hash value. This is problematic because hashes generated with abi.encodePacked are often used as keys in mappings, for signature verification, or as identifiers in various contract mechanisms.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Perp::updateFeeAndPremiumGrowth might cause potential DoS

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-445
- **Submitter:** DPS
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/445
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-445.md

## Brief Summary

Since the implementation of Uniswap fees can overflow, there is a case where the fees overflow and cause an underflow, however as the the protocol is using solidity pragma over 0.8 this will cause a revert because it is not in an unchecked block.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Orders might be left unexecuted

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-446
- **Submitter:** DPS
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/446
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-446.md

## Brief Summary

The Fillers are responsible for executing the Trader's order, however if there is no exchange route that will yield some benefits for the fillers they might not be incentivesed to execute the order. In the documentation it is not mentioned that the Filler is a trusted position, so I would assume that if there is no incentive/penalty they will not behave as expected.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# SupplyToken is not EIP20 compliant

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-454
- **Submitter:** DPS
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/454
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-454.md

## Brief Summary

The readme specifies that the `SupplyToken` should follow EIP20, but there are missing functions that make it not compliant.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Validation Check for `rangeSize` is wrong.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-307
- **Submitter:** DanielArmstrong
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/307
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-307.md

## Brief Summary

When `rangeSize % tickSpacing != 0`, the result difference between `lowerTick` and `upperTick` changes when reallocation and it affects protocol's stability.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# High 02 Non Working Price-Based Auction

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-385
- **Submitter:** EPSec
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/385
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-385.md

## Brief Summary

The function `validateStopPrice` in `PerpMarketLib` incorrectly calls `DecayLib::decay2` with a price ratio alongside start and end times. This causes a logical error in the calculation of the decayed price, leading to the failure of the intended price-based auction mechanism. Vulnerability Detail In `PerpMarketLib::validateStopPrice`, the function `DecayLib::decay2` is invoked with the ratio of the oracle price to the stop price as the last argument. The `decay2` function is designed to support both time-based and price-based auctions. However, in the context of `validateStopPrice`, both `startTime` and `endTime` are provided, which causes the entire logic to default to a time-based auctio...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Mid 03 No IsContract Check On Safe Transfers

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-388
- **Submitter:** EPSec
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/388
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-388.md

## Brief Summary

The `SafeTransferLib` library from `@solmate/src/utils/SafeTransferLib.sol` is used across several contracts to facilitate safe transfers. Vulnerability Detail In the contracts `PredyPool`, `SettlementCallbackLib`, `LiquidationLogic`, `ReallocationLogic`, `SupplyLogic`, `GammaTradeMarket`, `PerpMarketV1`, `SpotMarket`, `UniswapSettlement`, and `GlobalData`, the `SafeTransferLib` is implemented for performing token transfers. However, there is no verification to ensure that the token addresses are actually contracts. This is a critical check when using the `solmate` library for safe transfers as the absence of this check can leave the code vulnerable to a honeypot attack. Impact The lack of...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Mid 04 Underflow Due To Unchecked Block

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-389
- **Submitter:** EPSec
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/389
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-389.md

## Brief Summary

The audit identified a potential vulnerability related to underflow in certain variables within an unchecked block during fee growth calculations in `UniHelper::getFeeGrowthInside` Vulnerability Detail The code conducts fee growth calculations within an unchecked block, which could result in underflow for variables `feeGrowthInside1X128`, `feeGrowthAbove0X128`, and `feeGrowthBelow1X128`. Impact Underflow in the mentioned variables could lead to incorrect fee growth calculations, potentially affecting the accuracy of financial transactions or causing unexpected behavior in the system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Mid 05 Potential Of PreddyPool Drainage

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-390
- **Submitter:** EPSec
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/390
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-390.md

## Brief Summary

The `take` function in the `PreddyPool` contract is vulnerable to potential misuse, allowing the contract's funds to be drained. This issue arises due to the lack of restrictions on the amount that can be transferred and the fact that the locker role, which can call this function, is not a trusted role within the protocol. Vulnerability Detail The `take` function allows token transfers to be made from within the `predySettlementCallback` and `predyTradeAfterCallback` functions. While it is restricted to being called only by the current locker, there are no limitations on the amount of tokens that can be transferred. Additionally, the locker role, which has the authority to call this functio...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Manipulation of TWAP and Oracle Price in `PerpMarketV1.sol`

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-113
- **Submitter:** ETHworker
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/113
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-113.md

## Brief Summary

The `PerpMarketV1.sol` contract, specifically the `executeOrderV3` function, does not implement sufficient price validation checks. This lack of validation could allow malicious actors to manipulate the Time-Weighted Average Price (TWAP) and oracle price, leading to significant financial consequences for users and the protocol itself. The absence of robust price validation mechanisms creates the following risks: 1. **Price Manipulation:** Malicious actors could manipulate the TWAP and oracle price by executing trades at extreme prices. This could trigger liquidations of other users' positions at unfair prices, leading to substantial losses. 2. **Front-Running Attacks:** Attackers could fron...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing 2-step Transfer Ownership Pattern for poolOwner

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-107
- **Submitter:** EaglesSecurity
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/107
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-107.md

## Brief Summary

The current ownership transfer process involves the current owner of pairId calling that calls that check if the new addres is not address(0) and proceeds.However if the nominated new owner is not a valid account, it is entirely possible the owner may accidentally transfer ownership to an uncontrolled account, breaking all functions in PredyPool.sol with the onlyPoolOwner() modifier for the given pairId.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Registration of a pair with malicious price feed can lead to extraction of funds from other pairs.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-594
- **Submitter:** Eeyore
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/594
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-594.md

## Brief Summary

The updated requirement allowing anyone to add pairs introduces a potential attack vector. An attacker can duplicate any existing pair containing liquidity with a new pair that uses a malicious price feed. This price feed, for example, always reverts if called by anyone other than the attacker. This prevents the liquidation of insolvent positions. While price feed manipulation alone does not benefit the attacker, a flaw in how the creator/protocol revenue is accounted for ahead of time creates an opportunity for exploitation. If the pair creator sets the fee to the maximum amount of 20%, 10% of that fee is designated for him. The issue within the protocol is that the fee is calculated ahead...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_212_group

# Incorrect calculation of Amount Token 0

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-684
- **Submitter:** Hawkeye
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/684
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-684.md

## Brief Summary

Improper calculation of token 0 will return an erroneous output for the required amounts of token 0 .

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unprotected State Modification

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-646
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/646
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-646.md

## Brief Summary

The function `updateQuoteTokenMap` is not protected by any modifier to restrict who can call it. This means anyone can change the `_quoteTokenMap` mapping as they desire, which could potentially lead to serious state corruption and manipulation of the smart contract. Furthermore, there are other functions like `_validateQuoteTokenAddress` that rely on this state being correct.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# swapExactIn(...) and swapExactOut(...) functions cannot swap ETH

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-172
- **Submitter:** Kaysoft
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/172
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-172.md

## Brief Summary

`swapExactIn(...) and swapExactOut(...)` can not swap ETH to other tokens.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Type mismatch in the the eip-712 order hash function of PerpOrderV3.sol

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-655
- **Submitter:** Kaysoft
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/655
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-655.md

## Brief Summary

Unexpected integration failures with EIP712-compliant wallets

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Attacker can update two different prices to Pyth oracle to exploit the `executeTrade(...)` function of GammaTradeMarketWrapper.sol

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-681
- **Submitter:** Kaysoft
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/681
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-681.md

## Brief Summary

Attacker can submit two different prices to update Pyth Oracle in order to perform arbitrage transaction within a single transaction.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Potential Underflow in `Reallocation::calculateAmount1ForLiquidity` Function

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-416
- **Submitter:** LhoussainePh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/416
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-416.md

## Brief Summary

There is a potential underflow issue in the code where the subtraction operation `sqrtRatioA - sqrtPrice` may result in an underflow if `sqrtPrice` is greater than `sqrtRatioA`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unsafe downcasts can lead to errors

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-419
- **Submitter:** LhoussainePh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/419
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-419.md

## Brief Summary

There are instances where a function takes a uint256 as an input parameter and within the function is downcasted to a much smaller uint . This is a problem because if the result of `(liquidityAmount * sqrtRatioB) / (liquidityAmount - denominator1)` is bigger than uint160 , then only the least significant 160 bits will be used. And `sqrtPrice` will be wrong.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# There is no slippage control in `modifyAutoHedgeAndClose` method

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-506
- **Submitter:** LhoussainePh
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/506
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-506.md

## Brief Summary

- Slippage protection is crucial in financial transactions to ensure that the execution price does not deviate significantly from the expected price. By setting slippage to `0`, you effectively allow no room for price fluctuations. This means that the transaction might fail if the price changes even slightly. - However, a `0` slippage setting can be particularly risky in volatile markets. An attacker could exploit this by front-running the transaction. Front-running involves observing a pending transaction in the mempool and then placing a transaction with a higher gas fee to be executed first.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# ReentrancyGuard Not Initialized Correctly

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-188
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/188
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-188.md

## Brief Summary

The order of operations and the way the `ReentrancyGuard` is initialized in `PredyPool.function initialize` The line `__ReentrancyGuard_init();` initializes the `ReentrancyGuard` contract, which is inherited from OpenZeppelin's `ReentrancyGuard` contract. This initialization should happen after the contract's state variables are initialized, not before. If the `ReentrancyGuard` is initialized before the contract's state variables, it could lead to a reentrancy vulnerability. This is because the `ReentrancyGuard` relies on a state variable (`_status`) to track the reentrancy status, and if this state variable is not properly initialized, it could allow reentrancy attacks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Assumption for baseAmountDelta Leads to Potential Price Manipulation

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-193
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/193
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-193.md

## Brief Summary

[BaseMarketUpgradable.decodeParamsV3](https://github.com/code-423n4/2024-05-predy/blob/2fb1e0ec7a52fc06c2e9c8e561bccba84302e4bb/src/base/BaseMarketUpgradable.sol#L49-L66) the calculation of the `minQuoteAmount` and `maxQuoteAmount`. The `BaseMarketUpgradable.decodeParamsV3` assumes that `baseAmountDelta` is positive when calculating `minQuoteAmount`, and negative when calculating `maxQuoteAmount`. This assumption may not always hold true, leading to incorrect calculations. The scenario where the bug could occur: Assume `baseAmountDelta` is a negative value, representing a sell order. The function calculates `minQuoteAmount` using `settlementParamsV3.minQuoteAmountPrice * tradeAmountAbs / Co...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incomplete Fee Logic - Positive Settlement Fees Not Transferred (Security Risk)

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-194
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/194
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-194.md

## Brief Summary

`SettlementCallbackLib.execSettlement`, The code checks if `settlementParams.fee` is less than 0, and if so, it transfers the absolute value of `settlementParams.fee` from the `settlementParams.sender` to the `predyPool` contract but the code does not handle the case where `settlementParams.fee` is greater than 0. If `settlementParams.fee` is positive, the code should transfer the fee amount from the predyPool contract to the `settlementParams.sender`. The current implementation does not perform this transfer, which could lead to a loss of funds or an incorrect fee calculation. **`Loss of funds`** If a positive fee is expected to be paid to the `settlementParams.sender`, the lack of a trans...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Integer Overflow/Underflow in SettlementCallbackLib (Loss of Funds)

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-195
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/195
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-195.md

## Brief Summary

`SettlementCallbackLib.execSettlementInternal` takes an `int256` parameter called `baseAmountDelta`. When `baseAmountDelta` is negative, the function calls the buy function with `uint256(-baseAmountDelta)` as the `buyAmount` parameter. [SettlementCallbackLib.execSettlementInternal#L77-L85](https://github.com/code-423n4/2024-05-predy/blob/2fb1e0ec7a52fc06c2e9c8e561bccba84302e4bb/src/base/SettlementCallbackLib.sol#L77-L85) Impact The conversion from `int256` to `uint256` using the unary negation operator (`-`) can lead to an overflow or underflow if the absolute value of `baseAmountDelta` is greater than the maximum value of `uint256` (2^256 - 1). For example, if `baseAmountDelta` is equal to...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# SettlementCallbackLib.buy Allows Unauthorized Quote Token Transfer (Potential User Loss)

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-197
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/197
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-197.md

## Brief Summary

In the `SettlementCallbackLib.buy`, the handling of the `quoteAmount` calculation and the subsequent token transfers: [SettlementCallbackLib.buy#L169-180](https://github.com/code-423n4/2024-05-predy/blob/2fb1e0ec7a52fc06c2e9c8e561bccba84302e4bb/src/base/SettlementCallbackLib.sol#L169-L181) In the case where `price` is not zero, and `quoteAmountToUni` is greater than `quoteAmount`. In this scenario, the code attempts to transfer `quoteAmountToUni - quoteAmount` tokens from the `sender` to the contract (`address(this)`), the contract may not have the necessary approval or allowance from the `sender` to perform this transfer. In this scenario, the code attempts to transfer the difference (`quo...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_48_group

# Unforeseen Underflow in Perp.SqrtPerpAssetStatus Can Cause System Disruptions.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-198
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/198
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-198.md

## Brief Summary

In the `Perp.SqrtPerpAssetStatus` struct, the [lastRebalanceTotalSquartAmount](https://github.com/code-423n4/2024-05-predy/blob/2fb1e0ec7a52fc06c2e9c8e561bccba84302e4bb/src/libraries/Perp.sol#L85) field is defined as a `uint256`. This field is used to store the total amount of positions that will have to pay rebalancing interest in the future. Since it is a `uint256`, it cannot represent negative values. If the `totalAmount` and `borrowedAmount` fields are both negative (which is theoretically possible with the `int256` type), their sum could result in a negative value. In this case, assigning this negative value to `lastRebalanceTotalSquartAmount` (which is a `uint256`) would cause an unde...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Interest Growth Calculation in Perp.updateRebalanceInterestGrowth (Loss of Interest)

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-199
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/199
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-199.md

## Brief Summary

In the `Perp.updateRebalanceInterestGrowth`, the calculation of `rebalanceInterestGrowthBase` and `rebalanceInterestGrowthQuote`. The code currently calculates the interest growth as follows: [Perp.updateRebalanceInterestGrowth#L166-L173](https://github.com/code-423n4/2024-05-predy/blob/2fb1e0ec7a52fc06c2e9c8e561bccba84302e4bb/src/libraries/Perp.sol#L166-L173) The issue here is that the division operation `1e18 / int256(_sqrtAssetStatus.lastRebalanceTotalSquartAmount)` is performed before the multiplication with the settled user fee. This means that if `_sqrtAssetStatus.lastRebalanceTotalSquartAmount` is a large value, the division operation could result in a value close to zero, effectivel...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_63_group

# Inconsistent Position Updates Due to Unaccounted Negative Amounts in Perp Contract (PredyPool)

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-201
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/201
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-201.md

## Brief Summary

The `receivedAmount0` and `receivedAmount1` values are obtained from the `burn` operation on the UniswapV3Pool contract. These values represent the amounts of tokens received after burning the liquidity position. [Perp.rebalanceForInRange#L284-L288](https://github.com/code-423n4/2024-05-predy/blob/2fb1e0ec7a52fc06c2e9c8e561bccba84302e4bb/src/libraries/Perp.sol#L284-L288) The `requiredAmount0` and `requiredAmount1` values are obtained from the `mint `operation on the UniswapV3Pool contract. These values represent the amounts of tokens required to mint the new liquidity position. > The problem arises when the `receivedAmount0` and `receivedAmount1` values are smaller than the `requiredAmount0...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_67_group

# Solmate safetransfer and safetransferfrom does not check the code size of the token address

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-299
- **Submitter:** MSaptarshi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/299
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-299.md

## Brief Summary

the safetransfer and safetransferfrom don't check the existence of code at the token address. This is a known issue while using solmate's libraries. Hence this may lead to miscalculation of funds and may lead to loss of funds, because if safetransfer() and safetransferfrom() are called on a token address that doesn't have a contract in it, it will always return success, bypassing the return value check. Due to this protocol will think that funds have been transferred successfully, and records will be accordingly calculated, but in reality, funds were never transferred. So this will lead to miscalculation and possibly loss of funds

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_140_group

# The `liquidate()` function checks for prices after applying interest rates and updating rebalances

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-270
- **Submitter:** Matin
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/270
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-270.md

## Brief Summary

The `liquidate()` function performs the square root of the TWAP after a possible trade. This can be exploited by attackers who can manipulate the price and reset it back to the initial price at no external arbitrage risk unlike what would have been incurred if the price data was taken at the end of the block.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Reentrancy Vulnerability in uniswapV3MintCallback Function

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-527
- **Submitter:** MrxSnowden
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/527
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-527.md

## Brief Summary

The absence of a reentrancy guard in the uniswapV3MintCallback function exposes the contract to a critical reentrancy attack. An attacker can exploit this vulnerability to call the uniswapV3MintCallback function recursively, which could result in unauthorized and unlimited transfers of tokens. This could lead to a complete depletion of the assets in the contract, as observed in the recent incident where Predy Pool was entirely drained of 219,585.737814 USDC and 83.9 WETH

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_38_group

# Unrestricted Initialization Function Allows Reinitialization by Any User

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-619
- **Submitter:** MrxSnowden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/619
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-619.md

## Brief Summary

The initializeGlobalData function can be called multiple times by any user, leading to unintended reinitialization of critical contract state variables. This can disrupt the contract's functioning by resetting important counters and configurations, which could be exploited by malicious actors to manipulate the contract state. For example, reinitializing the global data could reset the pair count and vault count, causing conflicts and inconsistencies in subsequent operations that depend on these values.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_56_group

# Incorrect Safety Assessment in `getIsSafe` Function which miscalculates the safety of a position by not considering the absolute value of a negative margin.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-153
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/153
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-153.md

## Brief Summary

In the [getIsSafe function](https://github.com/code-423n4/2024-05-predy/blob/e96f378007e3dc56b184079f0c0e4fe48a72efaa/src/libraries/PositionCalculator.sol#L63-L73), the `isSafe` value is calculated as follows [Line 72](https://github.com/code-423n4/2024-05-predy/blob/e96f378007e3dc56b184079f0c0e4fe48a72efaa/src/libraries/PositionCalculator.sol#L72) This calculation may not be correct in certain scenarios. The condition `_vault.margin >= 0` checks if the margin of the vault is non-negative, but it does not consider the case where the vault has a negative margin and the `vaultValue` is greater than or equal to the absolute value of the negative margin. For example, let's assume that `minMargi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# Truncation Bug in `minMinValue` Calculation due to truncation caused by division by 1e6.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-154
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/154
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-154.md

## Brief Summary

The calculation of `minMinValue` [Line 92-93](https://github.com/code-423n4/2024-05-predy/blob/e96f378007e3dc56b184079f0c0e4fe48a72efaa/src/libraries/PositionCalculator.sol#L92-L93) In the division by `1e6`, this division operation truncates the result towards zero, which can lead to incorrect results when dealing with negative values. For example, `if calculateRequiredCollateralWithDebt(pairStatus.riskParams.debtRiskRatio) * debtValue` is `-1000000`, then `(-1000000).toInt256() / 1e6` will result in `0` instead of the expected `-1`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Position Assessment in `calculateMinValue` where the `hasPosition` flag is not updated correctly due to the logical OR operator's short-circuit behavior.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-155
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/155
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-155.md

## Brief Summary

`PositionCalculator:calculateMinValue` is responsible for calculating the minimum value, vault value, debt value, and determining if a position exists based on the provided `positionParams`. The line `hasPosition = hasPosition || getHasPositionFlag(positionParams);` is meant to update the `hasPosition` flag based on the `positionParams`. But the way the logical OR operator (`||`) works in most programming languages, including Solidity, the right-hand side of the expression (`getHasPositionFlag(positionParams)`) is only evaluated if the left-hand side (`hasPosition`) is false. This means that if `hasPosition` is initially true, the `getHasPositionFlag` function will not be called, and the va...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Invalid Tick Value Generation in `calculateMinLowerTick` where the addition of tickSpacing to minLowerTick can result in an invalid tick value that is not a multiple of `tickSpacing`, contradicting the UniswapV3 documentation.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-157
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/157
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-157.md

## Brief Summary

[Reallocation:calculateMinLowerTick](https://github.com/code-423n4/2024-05-predy/blob/e96f378007e3dc56b184079f0c0e4fe48a72efaa/src/libraries/Reallocation.sol#L126-L142), In [Line:137](https://github.com/code-423n4/2024-05-predy/blob/e96f378007e3dc56b184079f0c0e4fe48a72efaa/src/libraries/Reallocation.sol#L137) This line adds `tickSpacing` to the calculated `minLowerTick`, but according to the "UniswapV3" documentation, the tick values should be multiples of the `tickSpacing`. By adding `tickSpacing` to `minLowerTick`, the resulting value may not be a valid tick value.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# Incorrect Supply Interest Rate Calculation in updateScaler where the calculation of the supply interest rate is incorrect, leading to users being under-rewarded or over-rewarded for their supplied assets, resulting in a loss or gain of funds for the protocol or users.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-159
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/159
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-159.md

## Brief Summary

The supply interest rate is used to calculate the interest earned by users who supply assets to the protocol. An incorrect calculation of the supply interest rate can lead to users being under-rewarded or over-rewarded for their supplied assets, resulting in a loss or gain of funds for the protocol or users. `ScaledAsset:updateScaler` the calculation for `supplyInterestRate` is as follows: [#L201-L207](https://github.com/code-423n4/2024-05-predy/blob/e96f378007e3dc56b184079f0c0e4fe48a72efaa/src/libraries/ScaledAsset.sol#L201-L207) The intended formula for calculating the supply interest rate should be. Where `utilization` is the ratio of `getTotalDebtValue(tokenState)` to `getTotalCollatera...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_77_group

# Incorrect Condition Check in `_modifyAutoHedgeAndClose` where an incorrect condition check prevents users from modifying their positions when the order quantities and margin amount are zero.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-161
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/161
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-161.md

## Brief Summary

The current condition **`if (gammaOrder.quantity != 0 || gammaOrder.quantitySqrt != 0 || gammaOrder.marginAmount != 0)`** is checking if any of the order quantities or the margin amount is non-zero. If this condition is true, it reverts the transaction with the `InvalidOrder` error. However, based on the function's name and the subsequent code that calls `_saveUserPosition` with `gammaOrder.modifyInfo`, the intended behavior seems to be allowing modifications to the user's position when the order quantities and margin amount are zero. This means that if a user tries to modify their position without changing the quantities or margin amount (e.g., updating the hedge interval, slippage toleran...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Position Closure in autoClose where a negative sign error causes the function to open a new position in the opposite direction instead of closing the existing position.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-162
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/162
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-162.md

## Brief Summary

**`GammaTradeMarket:autoClose`** is intended to close an existing position by executing a trade with the opposite position size. However, due to the negative sign applied to `vault.openPosition.perp.amount` and `vault.openPosition.sqrtPerp.amount`, the function is actually attempting to open a new position in the opposite direction of the existing position, rather than closing it. > Instead of closing the existing position, the function will open a new position in the opposite direction, effectively doubling the user's exposure. > If the newly opened position moves against the user, it can lead to a significant loss of funds, as the user's margin is now allocated to two opposing positions....

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient Condition for Determining Hedge or Close where the condition solely relying on vault.openPosition values may lead to missed hedge or close opportunities.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-163
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/163
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-163.md

## Brief Summary

The condition **`vault.openPosition.perp.amount == 0 && vault.openPosition.sqrtPerp.amount == 0`** is used to determine if a hedge or close is required for a given position. But this condition alone is not sufficient because it relies solely on the `vault.openPosition` values, which may not accurately reflect the current state of the position. If a position has already been closed or hedged, but the `vault.openPosition` values have not been updated accordingly, the function will incorrectly return (`false, false, positionId`), indicating that neither a hedge nor a close is required. This could lead to the following issues: **`Missed Hedge Opportunities`** If a position needs to be hedged ba...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_95_group

# Unprotected Assumption in `_removePosition` where the assumption that the provided `positionId` exists in the `userPositions` mapping is not verified.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-164
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/164
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-164.md

## Brief Summary

**`GammaTradeMarket:_removePosition`** is responsible for removing a user's position from the `userPositions` mapping and updating the `positionIDs` mapping accordingly. Even though the function assumes that the `positionId` provided as input exists in the `userPositions` mapping, it may not always be the case. If the `positionId` does not exist in the `userPositions` mapping, the line **`address trader = userPositions[positionId].owner;`** will revert with an error because it's trying to access a non-existent mapping entry. Even if it doesn't revert, the function will proceed to remove the `positionId` from the `positionIDs` mapping for the trader address, which could lead to data corrupti...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# single-step ownership transfer pattern is dangerous as the filler is the owner

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-272
- **Submitter:** Neo_Granicen
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/272
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-272.md

## Brief Summary

`BaseMarketUpgradable` uses single-step ownership transfer Impact Since all market contracts inherent the `BaseMarketUpgradable` and executing a trade and removing a position in them is only possible through the filler, then in this case, those 2 operations aren't going to be available until the whole contract is upgraded but in the meantime, people who were going to remove their positions or add more collateral but are unable might get liquidated since liquidation can be done by anyone.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unused Return Variable Declarations in `PredyPool.sol` Functions

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-42
- **Submitter:** Pelz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/42
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-42.md

## Brief Summary

Several functions in the `PredyPool.sol` contract declare return variables but do not utilize them. This can lead to confusion, reduce code readability, and increase the risk of introducing bugs in future code modifications. Unused return variables may also mislead developers into thinking the functions produce outputs that are not actually being utilized, which can affect the contract’s intended functionality and maintainability.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Re-Org Attack Vulnerability in `createPriceFeed` Function in `PriceFeed.sol`

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-44
- **Submitter:** Pelz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/44
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-44.md

## Brief Summary

The `createPriceFeed` function in `PriceFeed.sol` deploys a new `PriceFeed` contract using the `create` method. This deployment mechanism is susceptible to re-org attacks, particularly on L2 networks like Arbitrum and Optimism, where the protocol is intended to be deployed. In a re-org scenario, an attacker could exploit the nonce-based address derivation to predict and intercept the deployment, potentially resulting in the loss of user funds and compromising the integrity of the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_157_group

# Incorrect accounting of total and borrowed amounts in `updateSqrtPosition` function leading to potential fund loss

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-440
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/440
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-440.md

## Brief Summary

Incorrect accounting of total and borrowed amounts in the `updateSqrtPosition` function can lead to discrepancies in the contract's internal state. This could potentially allow users to withdraw more funds than they should be able to, resulting in fund loss for the contract owner or other users.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# PriceFeed contract vulnerable to arbitrage

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-444
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/444
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-444.md

## Brief Summary

The PriceFeed contract calculates the square root price of a base token in terms of a quote token using prices from two different oracles. If there is a price discrepancy between the oracles, an attacker can exploit the contract by buying the base token at the lower price calculated by the PriceFeed contract and selling it immediately at the higher market price, profiting from the price difference. This can lead to a loss of funds for users relying on the PriceFeed contract for accurate pricing information.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# updatePoolOwner should call withdrawCreatorRevenue, otherwise, creator fees will be given to the next owner

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-341
- **Submitter:** SBSecurity
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/341
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-341.md

## Brief Summary

Changing the pool owner without first invoking the `PredyPool::withdrawCreatorRevenue` admin will make him lose all of the accumulated fees. The issue is even worse because not only the Predy team will be able to create new pairs and it is not expected from the normal users to know that they should explicitly claim their fees, before changing the owner.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# pairStatus.lastUpdateTimestamp updated even no change in `interestRateQuote and interestRateBase. gives inapproriate applying interest rate timestamp

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-586
- **Submitter:** Sathish9098
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/586
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-586.md

## Brief Summary

When applyInterestForPoolStatus returns an interest rate of 0, pairStatus.lastUpdateTimestamp is still updated. If there is a valid need to update the interest rate within the same block, the check if (pairStatus.lastUpdateTimestamp >= block.timestamp) prevents this from happening. As a result, the interest rate and protocol fees are not recalculated and updated as needed. Incorrect or missed interest calculations result in lenders receiving less interest than they are entitled to. Over time, this can accumulate to significant financial losses for lenders. Incorrect interest calculations also affect the protocol's fee income.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_50_group

# Potential Token Loss Due to Rounding Issue in Small Deposits

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-599
- **Submitter:** Sathish9098
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/599
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-599.md

## Brief Summary

Small deposits, where _amount * Constants.ONE < tokenState.assetScaler, will not yield any shares, effectively rendering such deposits worthless and leading to potential losses for lenders making small contributions. The lender transfers their tokens to the contract but receives no shares (mintAmount = 0), resulting in a direct loss of the tokens supplied.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Front-running Risk Leading to Failed Withdrawals Due to Insufficient Collateral

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-610
- **Submitter:** Sathish9098
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/610
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-610.md

## Brief Summary

Users can exploit this by monitoring the mempool and front-running large withdrawals, causing others to face failed transactions. This creates a race condition where stakers might rush to withdraw their tokens, potentially leading to failed transactions and user frustration.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# Attacker can modify position (hedge or close) of any user

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-555
- **Submitter:** Shubham
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/555
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-555.md

## Brief Summary

`modifyAutoHedgeAndClose` function processes a trader's modify order to update auto close and auto hedge condition. Only the owner of that particular position should be able to modify. However in the current state the function allows anyone to change/close a user's position.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Incorrect Use of `uint256` for Negative Margin Amounts in Liquidation Process

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-103
- **Submitter:** Stormreckson
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/103
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-103.md

## Brief Summary

In the liquidation process, if the `remainingMargin` is negative, the protocol is designed to collect this margin from the liquidator to cover losses. However, the use of `uint256(-remainingMargin)` for a transfer operation is incorrect and problematic. The current implementation attempts to convert a negative `remainingMargin` to `uint256` for a transfer operation during liquidation. This incorrect type conversion leads to transaction failures and can result in liquidators unfairly profiting through arbitrage trades, ultimately passing losses onto the protocol. Impact If the negative margin isn't correctly transferred from the liquidator, the protocol bears the loss, leading to bad debt. #...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Assembly revert only be caught for those functions with returns declaration

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-211
- **Submitter:** TECHFUND-inc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/211
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-211.md

## Brief Summary

The below functions will not revert as expected.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Protocol is potentially vulnerable to flash-loan attacks

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-134
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/134
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-134.md

## Brief Summary

The `supply()` and `withdraw()` functions allows a `lender` to supply/withdraw either the `quoteToken` or the `baseToken` to/from the lending pool. During supply, `bond` tokens are minted for the `lender` and during withdrawa, these tokens get burned in the process. However, the protocol does not implement any measures to prevent supply and withdrawals within the same `block` which offers an easy path for `flash-loan` attack. An `attacker` is able to use a `flash loan` to manipulate the price of a token supplied to the pool and then withdraw a different asset at an advantageous rate due to the manipulated price.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_24_group

# `burnAmount` is rounded down during withdrawal in favor of the user

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-136
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/136
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-136.md

## Brief Summary

When a protocol `rounds down` in favor of the user, this only results in `value leak` from the protocol itself.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_100_group

# Liquidation may be potentially flawed by malicious oracles.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-150
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/150
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-150.md

## Brief Summary

Malicious actors could exploit the `createPriceFeed()` function to create `PriceFeed` contracts with invalid or malicious parameters. This can lead to the injection of incorrect or `manipulated prices` into the protocol when `getSqrtPrice()` is called. Vulnerability Details The [createPriceFeed()](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/PriceFeed.sol#L18-L24) function is external, meaning anyone can call it to create a new `PriceFeed` contract. This `PriceFeed` created is then used to fetch `sqrtPrice` via [getSqrtPrice()](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/PriceFeed.sol#L44-L58).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_86_group

# `executeOrderV3()` reverts when updating vault `recipient`

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-166
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/166
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-166.md

## Brief Summary

A `trader` signs an `order` off-chain and submits it to the `whitelistFiller` who then calls `executeOrderV3()` function to verify `signature` of the `order_v3` and execute the `trade`. However, after setting `userPosition.vaultId`, the function invokes `_predyPool.updateRecepient()` to set the `trader` whose `order` is being executed as the vault's `recipient`. This will revert since the `updateRecepient()` implementation is restricted by `onlyVaultOwner` modifier. Since the `whitelistFiller` is not the owner of this `vault`, this function reverts. Vulnerability Details The `executeOrderV3()` invokes [_executeOrderV3()](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c2...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Possible `interest-free` loans due to the current interest rate update mechanism

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-256
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/256
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-256.md

## Brief Summary

Due to `interest` update mode within the protocol, a `trader` can get a `loan` without `interest` as long as repayment is done within the same transaction, `block`. Since this can be repeated overtime, protocol will be losing alot of `unclaimed interest fee` which would have made more funds to the LPs and the protocol. Vulnerability Details > Each time a user interacts with the contract, interest and premium from the previous interaction to the current one are applied. This increases the amount available for withdrawal by the lender and the premium income for Squart. However, it is intended by the protocol to only update `interest rate` once per `block` and not for each transaction. This is...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Any user can register `quote token` address for any pair

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-258
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/258
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-258.md

## Brief Summary

During `pool` creation, the `operator` sets the `poolOwner` address. The `poolOwner` therefore takes control of all modifications pertaining to the pool given by `pairId`. If `quote token` address is not registered yet i.e `_quoteTokenMap[pairId] == address(0)`, it means that trading in this `pool` is not yet allowed. However, `updateQuoteTokenMap()` in `BaseMarket` contract allows anyone to register `quote token` address for the pair which may be against the `poolOwner`. Therefore, any user will be in a position to initiate trading in any `pool` by registering `quote token` address for these pairs without permission. Vulnerability Details [updateQuoteTokenMap()](https://github.com/code-423...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# A `trader` can deposit any non-vault assets and open a position

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-262
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/262
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-262.md

## Brief Summary

A `trader` can set the `permitted token` to a token other than the vault's asset. When this happens, the `permitted token` is deposited and a position is opened for the trader. A malicious user could create a random `ERC20` token and then deposit this new `ERC20` token into the `vault` and open a `position`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Protocol lacks graceful retirement feature

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-400
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/400
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-400.md

## Brief Summary

There is no way for protocol to selectively disable `deposits` and `borrows` from a `pool`. Without the ability to selectively disable certain actions like deposits and borrows, the protocol might face increased risks during periods of market volatility or hack. Users might rush to deposit assets or borrow against them, potentially leading to over-leveraged positions that could trigger liquidations if the market moves unfavorably. Vulnerability Details Commonly in lending platforms, when a certain `token` or `lending pool` has been deemed to be too risky or have been hacked, it is `retired`. This means that all `deposits` and `borrows` from the pool are stopped, however the pool is kept in...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect token transfer Logic in `sell` and `buy` during direct fill.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-402
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/402
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-402.md

## Brief Summary

The `sell()` and `buy()` functions in the `SettlementCallbackLib` library have incorrect token transfer logic when `settlementParams.contractAddress == address(0)`. This results in the user supplying and receiving the wrong tokens during direct fills.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `getSqrtTWAP()` will revert when `ago` calculation underflows

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-423
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/423
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-423.md

## Brief Summary

Because the Solidity version used by the current implementation of `UniHelper.sol` is `v0.8.17`: > Arithmetic operations revert on `underflow` and `overflow`. Therefore, while in `Uniswap`, subtraction overflow is desired, without `unchecked` block, this will revert during `ago` calculation and further break other parts of the system that relies on `getSqrtTWAP()`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# `entryTokenAddress` is not validated before executing a trade in `quoteTrade()`

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-437
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/437
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-437.md

## Brief Summary

`quoteTrade()` function proceeds to execute a trade without checking if `entryTokenAddress` is registerd for the `pair`. This may result in discrepancies within the protocol as trades would end up being executed with unverified `entryTokenAddress`es for any pool pairs. Vulnerability Details Before a trade is executed (open position), the [executeTrade()](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/gamma/GammaTradeMarketWrapper.sol#L16) via [_executeTrade()](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/gamma/GammaTradeMarket.sol#L158-L176) invokes the `_validateQuoteTokenAddress...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Permission denial with `permit` through front-running.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-49
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/49
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-49.md

## Brief Summary

By design, with `permit` calls, the token ignores the `msg.sender`. Combined with the fact TXs can be observed in the `mempool` (by anyone, or at least by the sequencer in some L2s), it means that a `permit` can be easily frontran (simply duplicate the TX arguments). The user's execution will be `reverted`, forcing the user to formulate the commands, thus spending in another transaction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Rebalance interest growth is not updated during supply and withdraw

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-86
- **Submitter:** Tigerfrake
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/86
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-86.md

## Brief Summary

Each time a user interacts with the contract the following two functions comes to play: 1. [applyInterestForToken()](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/libraries/ApplyInterestLib.sol#L26-L48): Here, interest and premium from the previous interaction to the current one are applied. 2. [updateRebalanceInterestGrowth()](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/libraries/Perp.sol#L158-L174): This settles the interest on rebalance positions up to this block and update the rebalance fee growth value. Both of these are invoked within functions such as `liquidate()`, `reallocate()` and `t...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Inconsistency in `ratio` calculation yields wrong results when `price2 > price1`.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-88
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/88
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-88.md

## Brief Summary

The `ratio()` function in `PerpMarketLib` is designed to calculate the percentage difference between two prices, scaled by a constant `Bps.ONE`. However, the issue here lies in the inconsistency of the `denominator` used when `price1` is less than `price2`. This yields an incorrect value.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Deflation in PremiumCurve Value Due to Math Error in Implementation

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-109
- **Submitter:** Topmark
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/109
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-109.md

## Brief Summary

Break of Protocol Functionality due to Deflation in Premium Curve Value as a result of Math Error in Implementation in PremiumCurveModel contract

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_62_group

# Break of Protocol Functionality when Position and Trade Amount are both Negative Values

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-83
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/83
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-83.md

## Brief Summary

Break of Protocol Functionality when position Amount and trade Amount are both Negative values which will wrongly assume position is Open when it is not an Open Position as positionAmount & tradeAmount are negative there by assigning a wrong value to the return of deltaEntry and payoff variable in contract

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lenders are unable to withdraw complete collateral

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-627
- **Submitter:** Tripathi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/627
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-627.md

## Brief Summary

The `PredyPool:withdraw()` function is used to withdraw either the `quoteToken` or the `baseToken` from the lending pool. However, due to a rounding issue, lenders are unable to completely exit the pool Impact Lenders are unable to exit completely from the PredyPool

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_89_group

# Unauthorized Liquidation in execLiquidationCall

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-214
- **Submitter:** XDZIBECX
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/214
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-214.md

## Brief Summary

The execLiquidationCall function it’s does not use any checks to verify if the caller is authorized to execute liquidations and this flaw is allows any external address to attempt and potentially succeed in liquidating any vault without proper authorization. So the the absence of an access control in the execLiquidationCall function make The function is directly calls the LiquidationLogic.liquidate function without verifying if the caller is authorized to perform the liquidation - An attacker can exploit this by calling the execLiquidationCall function with any vault ID, close ratio, and settlement data, leading to unauthorized liquidation Impact An unauthorized user can liquidate vaults, c...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_115_group

# Validation Flaw in Uniswap Settlement data

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-512
- **Submitter:** XDZIBECX
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/512
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-512.md

## Brief Summary

The bug is arise from the absence of validation on the data in the `swapExactIn` and `swapExactOut` functions. These functions directly pass the data to Uniswap's router without any checks, cause The data is used for routing swaps in Uniswap's exactInput and exactOutput , is passed without validation. and This parameter should typically contain the routing path for a swap, specifying the sequence of token pairs involved in the trade. If not validated, a malicious or malformed data can cause swaps to route through unexpected or harmful paths, resulting in incorrect token transfers or financial losses. - Here in `swapExactIn` the data is passed directly to `ISwapRouter.ExactInputParams `witho...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# All tokens available on Uniswap can be added to PredyPool without any limitations

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-431
- **Submitter:** Yunaci
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/431
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-431.md

## Brief Summary

As the protocol’s current intent is to use `WETH, USDC, ARB, USDT, DAI, WBTC`, it is evident that they wouldn’t want other tokens to be added as pairs. With improper validation, it is completely possible for any user to add any token that is supported by `Uniswap`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Reliance on uniswap oracle on L2's is risky as its susceptible to price manipulations

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-648
- **Submitter:** ZanyBonzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/648
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-648.md

## Brief Summary

The protocol aims to deploy on Arbitrum, Base, Optimism and in certain cases, can choose to not deploy or use the chainlink/pyth oracle pricefeed for certain assets. In this case, uniswap oracle is used as a sort of fallback oracle. The issue however is that cost of manipulating TWAP on L2 networks is very low, and even Uniswap team has major concerns about using their oracle on L2s.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_45_group

# Arbitrary `from` passed to `transferFrom` (or `safeTransferFrom`)

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-301
- **Submitter:** a1exweb
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/301
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-301.md

## Brief Summary

Passing an arbitrary `from` address to `transferFrom` (or `safeTransferFrom`) can lead to loss of funds, because anyone can transfer tokens from the `from` address if an approval is made.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_33_group

# getFinalTradeAmount: Incorrect Calculation for Short Position Reduction.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-222
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/222
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-222.md

## Brief Summary

`getFinalTradeAmount` will return incorrect values when reducing a short position, which could lead to incorrect position calculations, incorrect trade amounts. Code with the problematic: [getFinalTradeAmount#L49-L60](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/perp/PerpMarketLib.sol#L49-L60) The issue is with the condition [if (-currentPositionAmount > tradeAmount)](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/perp/PerpMarketLib.sol#L66). Let's consider an example: In this case, when `currentPositionAmount` is negative (i.e., a short position) and `tradeAmount` is positive (i....

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Market Order Validation for Sell Orders (Decayed Price Check)

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-226
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/226
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-226.md

## Brief Summary

`PerpMarketLib#validateMarketOrder` responsible for validating the trade price against the decayed price range based on the auction parameters decayed price range is calculated using the `DecayLib.decay` function, which takes the `startPrice`, `endPrice`, `startTime`, and `endTime` as inputs. The function checks two conditions. For buy orders (`tradeAmount > 0`), it checks if the `decayedPrice` is less than the `tradePrice`. If this condition is true, it means the trade price is higher than the upper bound of the decayed price range, and the function returns `false` to indicate an invalid trade. [validateMarketOrder#L200-L206](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Incorrect Net Value Usage in Initial Margin Calculation (_calculateInitialMargin).

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-227
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/227
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-227.md

## Brief Summary

In the calculation of `netValue` using the `_calculateNetValue`: The problem is that the `_calculateNetValue`, based on the name of the function and the context, seems that the `netValue` should represent the total value of the user's position, including both the margin and the position value. If this is the case, then the calculation of the initial margin requirement is incorrect: The initial margin requirement should be calculated as the difference between the total value of the position and the position value, not the other way around. Impact The initial margin requirement is calculated as follows: [_calculateInitialMargin#L233](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f8...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# getUserPosition Returns Uninitialized Vault Data for Users Without Positions.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-228
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/228
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-228.md

## Brief Summary

[PerpMarketV1#getUserPosition](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/perp/PerpMarketV1.sol#L247-L263) In Solidity, when a struct is declared as a memory variable, its fields are not automatically initialized to default values. This means that if you try to access the fields of an uninitialized struct, you may get unexpected or garbage values. The issue is that if the `userPosition.vaultId` is zero (indicating no position), the function returns an uninitialized `vaultStatus` and `vault` struct. This could lead to unexpected behavior or errors when the caller tries to access the fields of these uninitialized structs. When the `ge...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `SpotMarket::_execSell` gives ERC20 token allowances to `settlementParams.contractAddress` but doesn't remove allowances when `settlementParams.contractAddress` is updated

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-455
- **Submitter:** air_0x
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/455
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-455.md

## Brief Summary

The old `settlementParams.contractAddress` will continue to have `ERC20` token approvals for `Token` so it can continue to spend the tokens when this is not the protocol's intention as it has changed settlementParams.contractAddress

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# The Use of address entryTokenAddress instead of address(this) can be exploited for replay attacks if not managed correctly

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-461
- **Submitter:** air_0x
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/461
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-461.md

## Brief Summary

The `PERP_ORDER_V3_TYPE` is an `abi.encodePacked` string that defines the structure of the `PerpOrderV3`. It uses `entryTokenAddress`, which can cause inconsistencies if `entryTokenAddress` is not correctly set or intended to be the contract's address.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Bps.sol : The lower and upper logic is incorrect which would lead to serious calculation error during the trade

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-588
- **Submitter:** ak1
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/588
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-588.md

## Brief Summary

`upper :` The result of price * bps / ONE does not represent a price increase. Instead, it calculates a value proportional to the basis points but does not add the basis points to the original price. `lower :` The result of price * ONE / bps does not represent a price decrease. Instead, it calculates an inverse proportional value, which is not the intended operation of reducing the price by the specified percentage. Following functions will gets affected. [checkPrice](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/libraries/SlippageLib.sol#L33-L44) - used to check the slippage [validateStopPrice](https://github.com/code-423n4/2024-05-predy/blob...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_73_group

# SettlementCallbackLib : `execSettlement` will not send fee to the `predyPool` when `baseAmountDelta` is negative

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-626
- **Submitter:** ak1
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/626
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-626.md

## Brief Summary

The fee amount will not be sent to the `predyPool`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Liquidation: `execLiquidationCall` can be reverted when refunding the excess amount to the trader/recepient.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-638
- **Submitter:** ak1
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/638
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-638.md

## Brief Summary

Liquidation can not be carried out successfully.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_88_group

# Attackers can revert the trade function by transferring the token directly to the predypool.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-263
- **Submitter:** almurhasan
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/263
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-263.md

## Brief Summary

As predypool accounting depends on ERC20(currency).balanceOf(address(this)), so an attacker can directly transfer the minimumi token(1 wei) to the predypool to make the trade function revert. Impact

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_32_group

# Lenders may not get rewards for supplying tokens in predypool.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-265
- **Submitter:** almurhasan
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/265
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-265.md

## Brief Summary

If any long positions are not opened for a pairid, then quote token’s totalNormalDeposited will be always 0 and if any short positions are not opened for a pairid, then base token’s totalNormalDeposited will be always 0 . if totalNormalDeposited = 0 for a token, then tokenState.assetScaler will not be updated for this token. impact

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_82_group

# First Depositor Exploitation

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-173
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/173
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-173.md

## Brief Summary

The current implementation of the supply function lacks specific safeguards against the exploitation by the first depositor. The first depositor can set initial conditions in a manner that unfairly benefits themselves or adversely impacts subsequent depositors. Making then receive less shares The first depositor might set parameters that benefit them disproportionately compared to others. ie by manipulating the initial state making other suppliers to receive less bonds

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Missing Nonce in Hash Calculation

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-176
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/176
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-176.md

## Brief Summary

The absence of a nonce in the `hash` calculation for `GammaModifyInfo` can make the protocol vulnerable to replay attacks. An attacker could reuse a previously used GammaModifyInfo object to execute the same operation multiple times, leading to potential loss of funds or unintended behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Cross-chain signature replay attacks

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-177
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/177
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-177.md

## Brief Summary

The `hash` function does not include the `chainid` in its hash calculation. This omission makes it vulnerable to replay attacks across different chains. In such attacks, a valid transaction executed on one chain can be replayed on another chain, leading to unintended consequences such as the loss of funds or unexpected behaviors. As specified by the EIP4337 standard to prevent replay attacks ... the signature should depend on chainid

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_85_group

# Unexpected reverts due to incorrect usage of staticcall

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-181
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/181
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-181.md

## Brief Summary

According to the solidity docs, if a staticcall encounters a state change, it burns up all gas and returns. The issue is that this burns up all the gas sent with the call. By default, almost the entire available gas is forwarded to the staticcall, causing the entire call to revert with an 'out of gas' error, which leads to a DOS The `callUniswapObserve` function may suffer from a vulnerability due to incorrect usage of `staticcall`. This vulnerability could lead to unexpected gas exhaustion opening doors for denial-of-service (DoS) attacks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unsafe Casting from uint256 to uint32 in `callUniswapObserve` Function

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-182
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/182
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-182.md

## Brief Summary

The function `callUniswapObserve` in our protocol has potential vulnerabilities due to unsafe casting from uint256 to uint32. This can lead to truncation issues if the value of ago exceeds the maximum value of uint32 (2^32 - 1). Such truncation can cause significant discrepancies in the intended versus actual value of ago, potentially leading to erroneous data processing and logic execution in our protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Implementing Rebalance Logic in Withdraw Function

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-183
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/183
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-183.md

## Brief Summary

Some tokens accrue yeild by default, The current implementation of the withdrawal function lacks a rebalance logic, potentially leading to loss of accrued funds and imbalances in the lending pool and discrepancies in interest calculations. The withdrawal function allows users to withdraw tokens from the lending pool without considering the potential impact on the pool's balance due to tokens with rebalancing nature. Without rebalancing logic in the withdraw, there could be loss of some funds and large withdrawals could create imbalances between the assets in the pair, affecting trading efficiency and liquidity provision.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Wrong price calculation due to Division before Multiplication error

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-348
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/348
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-348.md

## Brief Summary

The bot found a loss of precision issue which is not related to this one Impact Unintended division before multiplication in Solidity arithmetic can lead to precision loss and unexpected outcomes, particularly when dealing with large or small numbers. This issue arises due to the left-to-right evaluation of expressions and the absence of parentheses to explicitly define the order of operations. Without careful consideration, such code can introduce inaccuracies, affecting the reliability and correctness of smart contracts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# Users could supply assets but receive no bond token in return

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-349
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/349
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-349.md

## Brief Summary

The current implementation allows users to supply assets to the protocol and receive bond tokens in return. users might receive zero bond tokens despite supplying assets. This issue could lead to users losing their supplied assets without receiving the corresponding bond tokens, resulting in a loss of assets.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Issue with Direct Transfers to `msg.sender` for Blacklisted Addresses

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-350
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/350
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-350.md

## Brief Summary

In the `withdrawProtocolRevenue` function, transferring accumulated protocol revenue directly to `msg.sender` can cause transactions to revert if msg.sender is blacklisted by certain ERC20 tokens like `USDC`. This can disrupt the withdrawal process and affect the operator's ability to access the protocol's revenue. If `msg.sender` is blacklisted, the safeTransfer call will revert, preventing the successful withdrawal of protocol revenue. This can lead to operational disruptions, especially if the protocol relies on consistent revenue withdrawals for liquidity or operational expenses.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_58_group

# `updatePriceOracle` Function could be exploited

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-351
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/351
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-351.md

## Brief Summary

The current implementation allows pool owner to update price feed without any restriction, such as timelock. This leads to an attack vector that a malicious pool owner can steal funds. There is a potential vulnerability in the `updatePriceOracle` function due to the ability of the pool owner to update the price oracle without any restrictions such as a timelock. This could allow a malicious pool owner to update the price oracle to a malicious contract, leading to the manipulation of prices and potential theft of user collateral during liquidations. Potential Vulnerability The vulnerability arises from the following: The updatePriceOracle function allows the pool owner to update the price fe...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Update Fee Flow

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-358
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/358
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-358.md

## Brief Summary

If new fees are set, those fees will be used to calculate fees for the period after the fees were changed. This can result in an incorrect amount of fees being accrued. The current implementation of the `updateFeeAndPremiumGrowth` function within the protocol lacks a mechanism to update fees correctly However, there's a potential issue where changing fees without performing a synchronization step could lead to inaccurate fee application,

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# Potential Off-by-One Error in Time Comparison Using >= with block.timestamp

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-359
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/359
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-359.md

## Brief Summary

Using the <= operator for time comparisons against block.timestamp can introduce off-by-one errors due to the nature of how block.timestamp is updated only once per block. This can lead to unexpected behavior if the condition is met at the exact second when block.timestamp changes. This issue is especially critical in scenarios where time-sensitive operations are performed, potentially causing operations to revert unexpectedly or execute when they shouldn't.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# Mismatched Decimals in Price Calculation

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-365
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/365
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-365.md

## Brief Summary

The current implementation of price calculation overlooks the importance of matching decimals between values retrieved from different price feeds. Particularly, the function responsible for computing prices does not scale the decimals to match the precision of another. the precision of the values must align to ensure accurate calculations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# Default return value lead to out-of-bounds array access.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-515
- **Submitter:** aua_oo7
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/515
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-515.md

## Brief Summary

**Description:-** The line `uint256 index = type(uint256 ).max;` initializes the index variable to the maximum value representable by a uint256 (which is 2^256 - 1). This value is intended to serve as a sentinel to indicate that the item was not found in the array. If the item is not found in the array, index will remain as type(uint256).max. When this value is used by other functions (e.g., removeItem,removeItemByIndex) without validation, it can lead to out-of-bounds array access or other unintended behavior. **Mitigation steps:** check the index value to should not be greater than array size. function removeItem(uint256[] storage items, uint256 item) internal { uint256 index = getItemInd...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The method used to calculate the minimum value is incorrect

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-243
- **Submitter:** ayden
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/243
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-243.md

## Brief Summary

The method used to calculate the minimum value is incorrect which can lead to the result of minimum margin is incorrct.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_133_group

# Anyone can call the initilizeGlobalData function and change the address of the UniswapFactory

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-209
- **Submitter:** boringslav
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/209
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-209.md

## Brief Summary

The `AddPairLogic::initializeGlobalData` is used to set global.uniswapFactory when a new PredyPool is initialized. The problem is that everyone can call the `AddPairLogic::initializeGlobalData` after the Pool is initialized and change the address of the UniswapFactory.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# New Operator Should be Set in a Two-Step Action

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-255
- **Submitter:** brevis
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/255
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-255.md

## Brief Summary

According to the protocol documentation, https://docs.predy.finance/predy-v6/dev/architecture#operator-poolcreator, the operator holds the admin role in the protocol, which is a critical one. Nevertheless, the function `setOperator` allows a new operator address to be set via a single-step action. While the new address is validated against zero address, there is no other validation to confirm that this address is correct. So, in case of a human error while invoking the `setOperator` function, the control over the protocol is lost forever.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Hardcoded Fee Limits Restrict Economic Efficiency and Innovation

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-663
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/663
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-663.md

## Brief Summary

The hardcoded fee limit in the smart contract poses several challenges. Firstly, it can lead to efficiency loss; economic efficiency is crucial for financial platforms, and a static fee structure may not always represent the most cost-effective transaction option for users, which could decrease platform usage and liquidity. Additionally, it acts as a barrier to innovation by stifling the development of new financial products and discouraging certain market behaviors with its rigidity, thus blocking potential innovative pricing strategies that could benefit both the platform and its users. Moreover, this inflexibility might result in a competitive disadvantage as it prevents the platform fro...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unvalidated Trade Results in Liquidation Logic Risk Financial Integrity and State Accuracy

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-665
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/665
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-665.md

## Brief Summary

The `liquidate` function in the `LiquidationLogic` library does not validate the `tradeResult` to ensure it was successful. If the trade fails, this could lead to incorrect state updates, resulting in discrepancies in the vault margin and other critical state variables. This oversight could potentially lead to financial losses or unintended behaviors in the protocol, as failed trades may go unnoticed and cause the system to operate on incorrect data.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_41_group

# Transaction Reordering by Miners Exposes Interest Rate Calculation to Exploits

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-667
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/667
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-667.md

## Brief Summary

The ability of miners to reorder transactions within a block can lead to the bypassing of the check that prevents applying interest rates multiple times in the same block. This can result in incorrect interest rate calculations and potentially exploitable vulnerabilities in the smart contract. Attackers may take advantage of this issue to manipulate the interest rates applied to assets, leading to financial losses or unintended behavior of the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incomplete Reset of Nested Structures in Smart Contracts Risks Data Integrity

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-669
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/669
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-669.md

## Brief Summary

When using the `delete` keyword on a variable of a nested structure, only the top-level fields are reset to their default values. Nested fields within the structure remain unchanged, which can lead to unintended behavior and potential vulnerabilities. This issue is present in the `finalizeLock` function within the `GlobalDataLibrary` library of the `PredyPool` contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Range Calculation When Available Amount is Zero Results in Suboptimal Trading Efficiency

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-671
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/671
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-671.md

## Brief Summary

When `availableAmount` is 0, the `_getNewRange` function calculates the `lower` and `upper` ticks based on unadjusted values. This leads to an incorrect range calculation where the range is not centered around the current tick. As a result, the liquidity provision and trading functionality of the contract may not behave as expected. This can lead to suboptimal liquidity distribution, reduced trading efficiency, and potential financial losses for users.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Out-of-Gas Errors Due to Lack of Uniswap Pool Removal Mechanism

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-676
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/676
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-676.md

## Brief Summary

The absence of a proper removal mechanism for Uniswap pools from the `allowedUniswapPools` mapping can lead to excessive gas consumption and potential out-of-gas errors. As the number of entries in the mapping grows over time without the ability to remove outdated or unnecessary entries, gas costs for interactions involving this mapping will increase. This mismanagement could eventually render certain operations prohibitively expensive or impossible due to gas limitations, causing disruptions and reducing the contract's usability and efficiency. The impact of this issue can manifest in the following ways: 1. Increased gas costs for functions that interact with the `allowedUniswapPools` mapp...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_142_group

# The onlyByLocker modifier is used to restrict access to certain functions, but it's not properly protected. An attacker can manipulate the globalData.lockData.locker variable to gain unauthorized access.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-321
- **Submitter:** cmishra
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/321
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-321.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The getSqrtPrice function does not implement any reentrancy protection mechanisms. If an external function called from getSqrtPrice modifies the state of the contract, it could lead to unexpected behavior and potentially allow for funds to be drained

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-323
- **Submitter:** cmishra
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/323
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-323.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# First user to open a position will never be able to close it because their `vaultId = 0`, which always creates a new vault

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-491
- **Submitter:** crypticdefense
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/491
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-491.md

## Brief Summary

The protocol allows users to open positions for trading, which they can later close. Positions act as vaults, which store information regarding the user's position, such as `vault owner`, `vault recipient`, `pair id`, `quote token used by pair` When creating a vault, if the user enters `vault id = 0`, a new vault will be created for them with a unique `vault id`. If `vault id != 0`, this means the user already has a position open, so the vault is fetched from the mapping. The problem is that the very first vault which is created will have a `vault id = 0`. This means that whenever the user decides to close their vault, it will create a new vault for them instead of using their vault which h...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_72_group

# Permit signatures do not verify receiving token address

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-492
- **Submitter:** crypticdefense
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/492
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-492.md

## Brief Summary

Users can execute trades via the `Perp Market` or `Gamma Market`, where `quote token` can be traded for `base token` and vice versa. Pairs can have the following tokens for `quote token` and `base token`: `WETH, USDC, ARB, USDT, DAI, WBTC` When finalizing trades, the protocol utilizes `permitWitnessTransferFrom`, which is described by [Uniswap docs](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#:~:text=Use%20permitWitnessTransferFrom%20when%20you%20want%20to%20transfer%20a%20token%20from%20an%20owner%20through%20signature%20validation%2C%20but%20you%20would%20also%20like%20to%20validate%20other%20data.%20Any%20other%20data%20you%20wish%20to%20be%20validated%20can%2...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_70_group

# `PriceFeed` does not account for the exponent of the base price

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-495
- **Submitter:** crypticdefense
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/495
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-495.md

## Brief Summary

The protocol utilizes `getPriceNoOlderThan` from the `Pyth` oracle to determine the price of the `base token`, which is subsequently used to calculate the `sqrtPrice`. The `Pyth` oracle returns a `Price struct` that contains the `price` and `exponent`. The protocol does not take into account the `exponent` returned and proceeds to utilize the `price` without the `exponent` for the `sqrt price` calculation, therefore the `sqrt price` will be incorrect. This will impact any calls to `PriceFeed::getSqrtPrice()`. An example is to check if positions are liquidatable if they are unsafe, if the `sqrt price` is incorrect, then safe positions may be liquidated or unsafe positions may be unable to be...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# `PriceFeed` calculation may overflow due to performing a direct multiplication with `Q96`, causing DoS

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-496
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/496
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-496.md

## Brief Summary

`PriceFeed::getSqrtPrice` provides the square root price of the base token in terms of the quote token. There is a problem where direct multiplication is performed with `Constants.Q96 (2^96)` twice. This can cause overflow in some cases resulting in DoS.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `InterestRateModel::calculateInterestRate` calculation is incorrect

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-499
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/499
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-499.md

## Brief Summary

The protocol earns rewards based off the interest rate calculation that is deducted from the `predy pool`. However, this formula will return an incorrect `interest rate` under a certain condition, causing the protocol to receive an incorrect amount of rewards.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_126_group

# `Perp::computeRequiredAmounts` applies the calculated `offset` incorrectly, leading to incorrect amount of tokens traded

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-503
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/503
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-503.md

## Brief Summary

When trades are executed, `Perp::computeRequiredAmounts` is called to calculate the `required token amounts` that will be used in the swap. This amount is used to determine how much `base tokens` to trade with the user. `Perp::computeRequiredAmounts` applies an `offset` on the `requiredAmounts` for better accuracy. As mentioned by the sponsor on discord, `"We remove the offset from the Uni LP and convert it to Squart."`. In addition, the [Predy docs](https://docs.predy.finance/predy-v6/dev/squart#switching-price-ranges-in-uniswap-v3-liquidity-provider-positions:~:text=What%20we%20aim%20to%20do%20is%20to%20maintain%20Squart%20at%202%E2%88%9Ax%20by%20adjusting%20the%20offset%20during%20rebala...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `Reallocation::calculateUsableTick` will experience precision loss due to division before multiplication, creating an incorrect tick range for pairs

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-504
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/504
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-504.md

## Brief Summary

During `reallocation` of pairs (pairs act as liquidity pools), the `tick range` is updated to correctly reflect the tick range of the `Uniswap V3 pool`. However, due to direct `division before multiplication` (solidity rounds down on division), the new tick range calculation will experience precision loss. Thus, the new `tick range` will be incorrect and will not correctly reflect the amount of liquidity and pricing of that range. This can lead to many issues, for example lead to `DoS of trades`, since the pair may not have the amount of liquidity that was calculated for that `tick range`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_43_group

# Malicious user can make profits by falsifying the priceFeed of a token pair.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-565
- **Submitter:** forgebyola
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/565
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-565.md

## Brief Summary

Malicious user can easily profit off price difference between two tokens by using a false decimalsDiff between the 2.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_20_group

# Users can steal tokens through fees in the market

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-632
- **Submitter:** forgebyola
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/632
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-632.md

## Brief Summary

Loss of funds for the market since users can gain tokens at no cost

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Handling of Negative marginAmount Values in PerpOrderLib.resolve()

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-66
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/66
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-66.md

## Brief Summary

[src/markets/perp/PerpOrder.sol#resolve](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/perp/PerpOrder.sol#L73-L77) uses a ternary operator to assign the value of `amount` based on the condition `perpOrder.marginAmount > 0`. If the condition is true, it converts `perpOrder.marginAmount` (an `int256` value) to `uint256` using an explicit cast. However, if `perpOrder.marginAmount` is negative, the explicit cast will not preserve the negative value but instead convert it to a large positive `uint256` value due to the way two's complement representation works for signed integers. For example, if `perpOrder.marginAmount is -1`, it will be co...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Delayed Initialization of operator Variable Leads to Access Control Bypass in `initialize()` Function

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-67
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/67
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-67.md

## Brief Summary

`operator` variable is a state variable that represents the address of the contract operator. It is initialized with the value of `msg.sender` in the `initialize` function. However, the line `operator = msg.sender;` is executed after the call to `AddPairLogic.initializeGlobalData(globalData, uniswapFactory);`. [src/PredyPool.sol#initialize ](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/PredyPool.sol#L70-L75)

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unauthorized Trade Execution via Signature Bypass.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-73
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/73
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-73.md

## Brief Summary

If the `PerpOrderV3Lib.resolve` does not properly validate the signature or allows an empty `bytes("") `as a valid signature, it could enable an attacker to execute orders without proper authorization. _An attacker could potentially execute unauthorized trades, leading to the loss of funds held in user positions or the contract itself._ [src/markets/gamma/GammaTradeMarket.sol#quoteExecuteOrderV3#L282-L311](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/perp/PerpMarketV1.sol#L282-L311) [src/markets/perp/PerpOrderV3.sol#resolve](https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/markets/perp/Perp...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_64_group

# The settlement address is not validated

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-673
- **Submitter:** gkrastenov
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/673
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-673.md

## Brief Summary

When a trade is made in the SpotMarket, Uniswap is used as the settlement or the `quoteAmount` is directly calculated when `settlementParams.contractAddress == address(0)`. Currently, the settlement contract address is not validated to ensure it is whitelisted when users buy or sell on the SpotMarket. This opens a potential problem where a user can set a malicious address for a settlement contract, causing incorrect or manipulated data to be returned when `swapExactIn`, `swapExactOut`, `quoteSwapExactIn` or `quoteSwapExactOut` functions are called.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# H-1: Unprotected initialize

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-319
- **Submitter:** golomp
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/319
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-319.md

## Brief Summary

Consider protecting the initializer functions with modifiers. <details><summary>4 Found Instances</summary> - Found in src/libraries/logic/AddPairLogic.sol [Line: 44](src/libraries/logic/AddPairLogic.sol#L44) - Found in src/markets/gamma/GammaTradeMarket.sol [Line: 408](src/markets/gamma/GammaTradeMarket.sol#L408) - Found in src/markets/perp/PerpMarketV1.sol [Line: 220](src/markets/perp/PerpMarketV1.sol#L220) - Found in src/types/GlobalData.sol [Line: 35](src/types/GlobalData.sol#L35) </details>

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_74_group

# Lack of Balance Check in uniswapV3MintCallback Function

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-18
- **Submitter:** igbinosuneric
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/18
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-18.md

## Brief Summary

The uniswapV3MintCallback function in the PredyPool contract does not check the contract's token balances before attempting to transfer tokens to the Uniswap pool. This can lead to transaction failures if the contract does not have sufficient tokens to cover the transfer amounts (amount0 and amount1). Impact 1. If the contract does not have sufficient balance of token0 or token1, the safeTransfer call will fail, causing the entire uniswapV3MintCallback function to revert. This will halt the minting process initiated by the Uniswap pool. 2. Functions that rely on successful minting operations with Uniswap pools could be disrupted. For example, the supply and reallocate functions may fail if...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Incomplete createVault Function in PredyPool Contract

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-48
- **Submitter:** igbinosuneric
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/48
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-48.md

## Brief Summary

The createVault function in the PredyPool contract relies on the globalData.validate(pairId) function to ensure the pairId is valid. However, the function references globalData.createOrGetVault(0, pairId), which does not exist in the provided code. This absence results in incomplete functionality, leading to potential errors, state inconsistencies, and a poor user experience. Impact 1. The createVault function cannot create or retrieve a vault without a defined mechanism, resulting in incomplete functionality. 2. Interacting with an undefined function could lead to state inconsistencies or unexpected behavior in the contract. 3. An incomplete function could be exploited by malicious actors...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_102_group

# Immediate User Liquidation Risk

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-605
- **Submitter:** igdbase
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/605
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-605.md

## Brief Summary

This is a popular bug Case: where if a protocol were to allow users to borrow their LTV exactly they might be liquidated in the next block. In cases where a protocol allows users to borrow their exact Loan-to-Value (LTV) ratio, there is a risk of immediate liquidation in the following block. Currently, in Predy, this scenario seems feasible, leading to potential unfair liquidation, financial losses, and diminished trust in the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_129_group

# Valid Pairs Incorrectly Marked as Invalid

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-609
- **Submitter:** igdbase
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/609
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-609.md

## Brief Summary

[RebalancingLibrary.sol]https://github.com/code-423n4/2024-05-predy/blob/a9246db5f874a91fb71c296aac6a66902289306a/src/libraries/RebalancingLibrary.sol#L26-L42 Vulnerability details Impact: Lack of proper validation for parameters can introduce unforeseen behavior and security risks in the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_40_group

# Don't verify if a pair is enabled leading to use trade even if a pair isn't enabled.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-167
- **Submitter:** jeremie
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/167
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-167.md

## Brief Summary

User can use trade with an disabled pair, this is not intended by the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# DoS attack: Inefficient input validation of '_amount' in SupplyLogic.supply() may be leveraged to clog the Network.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-635
- **Submitter:** la-arana-inteligente
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/635
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-635.md

## Brief Summary

The function in takes an argument and validates it for only positive values using the statement. However, an attacker could indeed use very small amounts ("dust") to continually invoke the supply function, leading to unnecessary execution and network congestion. This attack would clog the network by sending numerous transactions that consume gas, even if they pass the basic validation, thereby, eventually leading to DoS.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential underflow in `calculateSlippageTolerance` function

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-628
- **Submitter:** lydia_m_t
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/628
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-628.md

## Brief Summary

In the `calculateSlippageTolerance` function, there is a potential for underflow due to inconsistent data types used in arithmetic operations. An underflow in the slippage tolerance calculation can lead to incorrect slippage values.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_80_group

# validateRiskParams doesn't check boundary of min and max slippage

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-169
- **Submitter:** nnez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/169
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-169.md

## Brief Summary

validateRiskParams doesn't check sanity of min and max slippage Impact - Users' positions will be liquidated more than it should be.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_34_group

# Gamma position can be `removed` without closing the `position`.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-518
- **Submitter:** oxchsyston
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/518
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-518.md

## Brief Summary

The function `predyTradeAfterCallback()` is used to handles the aftermath of a trade in the `PredyPool`. It processes different types of callbacks, ensuring the appropriate actions are taken based on the callback type. For `non-quote` callbacks, it manages the margin updates and ensures positions are correctly handled, emitting relevant events for further processing or logging. The issue here is that the function directly closes the position after calling `take` to collect the `margin` by calling the internal function `_removePosition()`, which does not check if the position has been closed before removing it(vaultId) from the `tradeParams(struct)`. this can cause serious issues due to inco...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_234_group

# Increase and decrease of `liquidity` does not update the fee growth.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-550
- **Submitter:** oxchsyston
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/550
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-550.md

## Brief Summary

The functions `increase()` and `decrease()` are used to add and remove liquidity from the pool and in turn `lptoken` is minted or burnt respectively. the issue here is that the fee growth is not being updated after adding or removing liquidity from pool. this can lead to some liquidity providers receiving more fees than they are entitled to, while others might receive less. Incorrect fee distribution can lead to a misalignment of incentives, discouraging liquidity provision and potentially reducing the overall liquidity in the pool. Uniswap V3 positions are calculated based on the fee growth within specific tick ranges. Failing to update fee growth can result in inaccurate position valuatio...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# Integer Underflow in collect Function: The collect function uses unchecked subtraction, which can lead to integer underflow if position.tokensOwed0 or position.tokensOwed1 is less than the amounts being subtracted.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-148
- **Submitter:** safi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/148
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-148.md

## Brief Summary

Impact: This can result in incorrect token balances and unexpected behavior in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Initialization and Implementation Risks in Upgradeable BaseHookCallback

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-54
- **Submitter:** selverkjennelse
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/54
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-54.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# If a lender got blacklisted by asset contract, their collateral funds can be permanently frozen with the pool

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-563
- **Submitter:** skypper
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/563
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-563.md

## Brief Summary

It's impossible for lender to transfer their otherwise withdraw-able funds to another address. If for some reason borrower got blacklisted by collateral token contract (i.e. USDC as it contains blacklist functionality), these funds will be permanently frozen as now there is no mechanics to move them to another address or specify the recipient for the transfer. Principal funds of lender can be permanently frozen in full, but blacklisting is a low probability event, so setting the severity to be medium. The `trade()` and `reallocate()` functions and moreover the `predySettlementCallback()` and `predyTradeAfterCallback()` which are making token transfers are out of scope.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# A large number of pairs are maliciously added, which will cause the pair to reach the upper limit and cannot be added.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-531
- **Submitter:** steadyman
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/531
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-531.md

## Brief Summary

The addition of a large number of malicious pairs will cause normal pairs to reach the upper limit and cannot be added, making the entire system unable to operate normally.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unexpected protocol behavior because some tokens have implementations only on part of the target chains

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-233
- **Submitter:** unique
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/233
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-233.md

## Brief Summary

USDT and ARB tokens have no implementations on some of the target deployment chains. So there is no guarantee of correct integration when the implementations will appear. Vulnerability Details According to the contest documentation expected token integrations are WETH, USDC, ARB, USDT, DAI, WBTC, and target deployment chains are Arbitrum, Base, and Optimism. USDT has no trusted implementations on the Base chain (https://basescan.org/tokens ). ARB has no implementations on the Base, and Optimism chains ( https://cryptorank.io/blockchains/base , https://basescan.org/tokens ) , ( https://cryptorank.io/blockchains/optimism ). Since the implementations are absent no one can guarantee the protoco...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# totalCompoundDeposited is scaled twice, resulting in incorrect TotalCollateralValue

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-213
- **Submitter:** xiao
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/213
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-213.md

## Brief Summary

totalCompoundDeposited is scaled twice, resulting in incorrect TotalCollateralValue.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# When calculating supplyInterestRate, getTotalDebtValue(tokenState)>getTotalCollateralValue(tokenState) is not considered

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-260
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/260
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-260.md

## Brief Summary

If `getTotalDebtValue(tokenState)` > `getTotalCollateralValue(tokenState)` is not considered, the ratio may be greater than 1, so the supply interest rate will be amplified, causing the economic system to face the risk of instability. By limiting the ratio to 1, the supply interest rate is ensured to be within a reasonable range and the stability of the system is maintained.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The order of quoteToken(token0, token1) in the addPair function is reversed, resulting in incorrect order of the _storePairStatus parameters.

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-564
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/564
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-564.md

## Brief Summary

The order of quoteToken(token0, token1) in the addPair function is reversed, resulting in incorrect order of the _storePairStatus parameters.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_155_group

# approve()/safeApprove() may revert if the current approval is not zero

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-617
- **Submitter:** xposhti
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/617
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-617.md

## Brief Summary

Calling `approve()` without first calling `approve(0)` if the current approval is non-zero will revert with some tokens, such as Tether (USDT). While Tether is known to do this, it applies to other tokens as well, which are trying to protect against [this attack vector](https://docs.google.com/document/d/1YLPtQxZu1UAvO9cZ1O2RPXBbT0mooh4DYKjA_jp-RLM/edit)..

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_51_group

# test 2 bitcoin,pls fovgive me

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-334
- **Submitter:** xywang
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/334
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-334.md

## Brief Summary

Lgithub.com/<organization>/<repository>/blob/<branch_name>/README.md?plain=1#L10 Vulnerability details Impact Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Assets can remain stuck forever inside the protocol

- **Contest:** Predy
- **Slug:** 2024-05-predy
- **Submission:** V-377
- **Submitter:** y0ng0p3
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-predy-validation/issues/377
- **Source snapshot:** competitions/2024-05-predy/submissions/raw/V-377.md

## Brief Summary

At present, the protocol (the operator) has only one method of claiming accumulated revenue. To do this, they use the `PredyPool::withdrawProtocolRevenue` function which transfers the amount of the ERC20 asset from `pool.accumulatedProtocolRevenue` to the protocol operator. This function can be used to withdraw _only_ the amount in `pool.accumulatedProtocolRevenue` and _nothing else_. The variable is also updated in `ApplyInterestLib` when applying interest in the pool. The operator can withdraw any revenue that the protocol generates and this is perfectly correct. However, let's consider the case where someone has randomly or mistakenly sent ERC20 tokens into the protocol or any other case...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_02_group
