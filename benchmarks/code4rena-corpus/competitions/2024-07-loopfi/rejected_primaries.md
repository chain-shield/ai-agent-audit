# Rejected Primary Findings: LoopFi

# Users can compute Permit2 signature for any ERC20 token, ultimately stealing from `PositionAction`'s Ether

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-401
- **Submitter:** 0xBugSlayer
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/401
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-401.md

## Brief Summary

Depositing less valuable tokens to the `CDPVault` and stealing all of the `PositionAction`'s Ether

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unbounded Credit Manager Debt Limit Allows Violation of Assets >= Borrowed Invariant

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-298
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/298
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-298.md

## Brief Summary

`setCreditManagerDebtLimit()` allows setting a credit manager's debt limit to any value without validation. This can lead to a scenario where the total borrowed amount exceeds the total assets in the pool, violating a critical invariant. Impact If a credit manager's debt limit is set too high, it can cause the pool's `totalBorrowed` to surpass `totalAssets`. This breaks the fundamental invariant `totalAssets >= totalBorrowed`, which is essential for maintaining the pool's solvency and correct operation. Even without malicious intent, incorrectly setting debt limits can lead to accounting errors and loss of user confidence in the system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_90_group

# Incorrect Borrowable Amount Calculation in edge case.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-712
- **Submitter:** Afriauditor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/712
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-712.md

## Brief Summary

The current implementation of the creditManagerBorrowable function does not check the borrowable amount against the available() function. This could lead to wrong borrowable value in scenarios where the function returns a borrowable amount greater than the actual available liquidity in the pool. though it checks availableToBorrow in interestRateModel contract however there is no way we can tell if it returns a value greater than availableLiquidity() which is the actual liquidity in the contract, because if it does availableLiquidity() becomes the actual Amount available to borrow.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_64_group

# Incorrect Rounding in Conversion Functions Allows Asset Drain (`AuraVault::withdraw` and `AuraVault::mint`)

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-280
- **Submitter:** Agontuk
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/280
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-280.md

## Brief Summary

The `AuraVault` contract implements a vault that allows users to deposit and withdraw assets, mint and redeem shares, and compound rewards. The contract uses conversion functions [`_convertToShares`](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/vendor/AuraVault.sol#L182-L184) and [`_convertToAssets`](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/vendor/AuraVault.sol#L189-L191) to handle the conversion between assets and shares. However, the rounding direction used in these functions is incorrect, leading to a critical vulnerability. Detailed Description The [`withdraw` function](https://github...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Length validation in `setOracles` function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-302
- **Submitter:** Akay
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/302
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-302.md

## Brief Summary

The `setOracles` function in the smart contract does not validate that the input arrays `_tokens` and `_oracles` have the same length. This can lead to issues whereby if `_oracles` is shorter than `_tokens`, the function will attempt to access an out-of-bounds index in `_oracles`, causing the transaction to revert or if `_oracles` is longer than `_tokens`, the extra elements in `_oracles` will be silently ignored.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_163_group

# Division before multiplication will lead to incorrect calculation of cumulativeIndexLU

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-508
- **Submitter:** Anirruth
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/508
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-508.md

## Brief Summary

The rate will get divided first with `SECONDS_PER_YEAR`(31536000) and get rounded down to 0 which will lead to improper calculation of the `cumulativeIndexLU`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_94_group

# Incorrect Order of Operations in `withdrawAndRepay()`

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-426
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/426
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-426.md

## Brief Summary

(https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/proxy/PositionAction.sol#L254 Vulnerability details Impact If users don't have sufficient funds to repay before withdrawal, their transactions to `withdrawAndRepay()` will fail.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Based on implementation Borrow should never revert but this function makes externals call and can revert.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-342
- **Submitter:** Bigsam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/342
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-342.md

## Brief Summary

The `borrow` function is designed not to revert based on its implementation. However, it makes an external call to the `poolV3` contract, which includes a `whenNotPaused` modifier. If the `poolV3` contract is paused, this external call will revert, causing the `borrow` function to fail unexpectedly. This issue also affects both flash loan functionality and the `cdpVault` action, potentially disrupting the protocol's operations and leading to failed transactions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_122_group

# PoolV3 does not comply with ERC4626 standard

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-569
- **Submitter:** Brenzee
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/569
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-569.md

## Brief Summary

[`PoolV3`](https://github.com/code-423n4/2024-07-loopfi/blob/main/src/PoolV3.sol#L46-L49) is a tokenized vault contract, which should be compatible with ERC4626 standard. But because of changes in the contract compared to original `PoolV3`, `PoolV3.maxWithdraw` and `PoolV3.maxMint` functions do not comply with ERC4626 standard.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_66_group

# Pause/unpause functionalities not implemented in many pausable contracts

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-336
- **Submitter:** Centaur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/336
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-336.md

## Brief Summary

The CDPVault and Vault Register are supposed to be pausable (they inherit from Pauseable, and ERC20Pausable) but they don't implement the external pause/unpause functionalities which means it will never be possible to pause them.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ExitPool:: allows users to withdraw lesser single token if Balancer Pool is imbalanced

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-352
- **Submitter:** Centaur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/352
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-352.md

## Brief Summary

Users will always receive lesser fewer tokens than they are due when exiting the pool, leading to potential financial losses.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_63_group

# `updateLeverJoin opens door for possible under-collaterized pool when providing liquidity to Balancer pool.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-365
- **Submitter:** Centaur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/365
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-365.md

## Brief Summary

The pool may issue more liquidity to the user than entitled to, diluting the value of tokens held by other participants.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_104_group

# UniswapV3:: ExactOutputParams and ExactInputParams swapParams recipient will be unable to receive ETH it's a secondary or primary token to be swapped

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-499
- **Submitter:** Centaur
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/499
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-499.md

## Brief Summary

`swapParams.recipient` will never receive any incoming ETH transfer or out going transfer. The primary impact of this bug is the potential for failed transactions or loss of ETH if the recipient address is not payable. This flaw undermines the reliability of the Uniswap V3 swap functions when dealing with ETH, leading to possible asset losses and user dissatisfaction.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Lack of Adherence to Checks-Effects-Interaction Pattern in zapVestingToLp Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-629
- **Submitter:** Centaur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/629
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-629.md

## Brief Summary

## TITLE: [M-7] Lack of Adherence to Checks-Effects-Interaction Pattern in zapVestingToLp Function **Description:** The `MultidistridbutionFee::zapVestingToLp` function in the contract is designed to allow a user to convert their vested tokens into liquidity provider tokens (LP tokens). However, this function does not adhere to the checks-effects-interaction pattern, which is crucial for preventing potential reentrancy attacks. The function performs external calls to transfer tokens and update the price provider before updating the user's balance, which could open up vulnerabilities if the external contract being called is malicious or compromised. Specifically, the function first transfers...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users and credit managers can be unfairly liquidated if the base interest rate is suddenly changed

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-451
- **Submitter:** Cryptor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/451
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-451.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Flash Fee Calculation Method

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-106
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/106
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-106.md

## Brief Summary

The inconsistent calculation method may lead to discrepancies in the fee charged for flash loans. Users and integrators expecting the documented fee calculation method might find the actual fees charged by the contract to be unexpectedly higher or lower, potentially leading to a loss of trust and financial inaccuracies.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_98_group

# Inconsistent Update of `availableLiquidityDelta` in `_deposit` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-109
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/109
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-109.md

## Brief Summary

If `availableLiquidityDelta` does not correctly account for the `assetsReceived` amount, the liquidity state of the contract may be incorrect. This could lead to inconsistencies in liquidity calculations, potentially affecting the contract's ability to handle future deposits or withdrawals properly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_71_group

# Missing Accrued Interest and Revenue Updates in _updateBaseInterest Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-112
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/112
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-112.md

## Brief Summary

The absence of updates for accrued interest and accrued revenue can lead to discrepancies in the calculations of the base interest rate and expected liquidity. This may cause incorrect financial calculations, potentially resulting in unintended financial outcomes or misalignment with the intended economic model of the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Conversion from `uint128` to `uint256` in `_convertToU256` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-113
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/113
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-113.md

## Brief Summary

This inconsistency can lead to unintended behavior or errors in the smart contract where the function `_convertToU256` is used. Specifically, if the consuming code relies on receiving a `uint256` value but gets a `uint128` instead, this could cause issues with arithmetic operations, comparisons, or other logic that expects the full `uint256` precision.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# Lack of Contract Validation in Delegate Call

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-133
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/133
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-133.md

## Brief Summary

If the `to` address is not a contract, the `delegatecall` will fail, causing the entire transaction to revert. This can disrupt the intended functionality of the contract and may lead to loss of execution continuity or unintended contract state changes. The lack of a contract check makes it harder to diagnose the issue and handle such failures gracefully.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_75_group

# Missing Slippage Checks in Critical Functions

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-136
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/136
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-136.md

## Brief Summary

Without slippage checks, these functions are vulnerable to price fluctuations, which could result in receiving significantly fewer tokens than expected. This discrepancy can lead to losses or other unintended effects, particularly in functions handling large or critical financial operations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Front-Running Risk in `updateEpoch` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-151
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/151
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-151.md

## Brief Summary

A malicious actor can manipulate the timing of epoch updates to ensure that the rates are updated in their favor. This can lead to unfair advantages in the system, where the attacker secures better quota rates before other users are aware that an epoch update is due.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_51_group

# Incorrect `totalAllocPoint` Calculation in `batchUpdateAllocPoint` Leading to Inconsistent Pool Allocation Points

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-172
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/172
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-172.md

## Brief Summary

An incorrect `totalAllocPoint` can lead to an inaccurate distribution of rewards among pools. If the `totalAllocPoint` does not reflect the true sum of allocation points across all pools, the rewards distribution logic that relies on this value will be skewed, potentially resulting in one or more pools receiving a disproportionate share of rewards.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Loop Termination in `setScheduledRewardsPerSecond` Function Due to Zero Offset

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-173
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/173
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-173.md

## Brief Summary

The primary impact is that the rewards schedule may not update correctly, leading to an inaccurate calculation of `rewardsPerSecond`. Since [_updateEmissions](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/reward/ChefIncentivesController.sol#L439-L447) is responsible for setting the correct reward rate based on the current timestamp, a failure here means that participants may receive incorrect reward amounts when they call claim. If the loop fails to execute as intended, it could result in the contract not updating the rewards schedule correctly. This could lead to incorrect rewards distribution, where the intended adjustment to the rewards pe...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Updating pool's reward information doesn't consider users `rewardDebt `

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-174
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/174
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-174.md

## Brief Summary

The `rewardDebt` is crucial for accurately tracking rewards that each user has already earned but not yet claimed. By not considering `rewardDebt` directly in `_updatePool`, there is a risk that the rewards distributed to users might be miscalculated when users claim them, leading to potential discrepancies in reward allocations. This could result in users receiving incorrect amounts of rewards, which could undermine trust in the system and affect user satisfaction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Negative Claimable Rewards Due to user.rewardDebt Exceeding Accumulated Rewards

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-176
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/176
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-176.md

## Brief Summary

A negative `claimable` value means that the contract allows the user to have a claimable amount that is less than zero, which could lead to incorrect reward distributions, potential overflows, or erroneous behavior in reward claiming mechanisms. This could undermine the integrity of the reward system and lead to unexpected losses or errors.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `_processEligibility` Always Returning True Causes Universal User Disqualification in `afterLockUpdate`

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-180
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/180
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-180.md

## Brief Summary

The constant passing of `true` to `_processEligibility` function call within the `afterLockUpdate` function can cause users to be disqualified and have their emissions stopped unintentionally. This may lead to users being unfairly penalized, missing out on rewards or other benefits they were otherwise eligible to receive, which could result in loss of trust in the system and possible financial impact.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_12_group

# Incorrect Disqualification Time Assignment in `setDqTime` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-198
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/198
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-198.md

## Brief Summary

If the disqualification time is set to a future timestamp, a user might continue to receive rewards despite being ineligible, leading to an unfair distribution of rewards and potentially impacting the protocol's integrity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Vesting Penalty Implementation in vestTokens Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-211
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/211
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-211.md

## Brief Summary

Users may receive full rewards on the entire vested amount, even for portions that should be subject to penalties. This could result in an unfair distribution of rewards and potentially allow users to circumvent intended vesting restrictions, leading to economic imbalances within the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Overly Restrictive Time Check in `individualEarlyExit` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-212
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/212
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-212.md

## Brief Summary

Users are unable to withdraw their funds at the exact moment of unlocking, potentially leading to user frustration and a slight delay in fund accessibility.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_142_group

# Possible incorrect party being incentivized in `claim` function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-345
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/345
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-345.md

## Brief Summary

The current logic could result in the incorrect party being incentivized, diminishing the effectiveness of the incentive mechanism designed to encourage users to initiate the claim process. This could reduce user engagement in the system and lead to delays in claim processing.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Slippage Risk in `claim` Function Due to Lack of `minAmountOut` Consideration

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-346
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/346
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-346.md

## Brief Summary

A user could deposit an expected amount of `WETH` only to receive fewer rewards (`BAL` or `AURA`) than anticipated, which could lead to a significant loss in value due to slippage during the swaps. This discrepancy arises because the function does not verify that the output of the rewards meets a minimum expected threshold (`minAmountOut`), which would otherwise protect against adverse market conditions or price changes during the transaction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Pool State Due to Separate Profit Minting and Repayment Operations

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-374
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/374
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-374.md

## Brief Summary

The primary impact of this vulnerability is the potential for significant mispricing of loans and incorrect assessment of the pool's liquidity. Due to the inconsistent pool state, interest rates may be calculated incorrectly, leading to either underpriced or overpriced loans. This could result in unfair advantages for some users while potentially causing financial losses for others or the protocol itself. The inaccurate liquidity assessment could also lead to over-borrowing, putting the pool at risk of insolvency in extreme cases.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_118_group

# `addQuotaToken` function allows non-quoted tokens to be added

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-399
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/399
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-399.md

## Brief Summary

If non-quoted tokens are added to the system, they will not be properly recognized by the `PoolQuotaKeeperV3` contract as part of the quota system. This could result in these tokens not contributing to or being affected by quota calculations, leading to potential revenue discrepancies and improper quota management. Non-quoted tokens might not be subject to the same quota-based restrictions or benefits, which could lead to unintended financial outcomes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_79_group

# Problem with external call

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-669
- **Submitter:** EPSec
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/669
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-669.md

## Brief Summary

In `PoolV3::_updateBaseInterest` an external call to `ILinearInterestRateModelV3::calcBorrowRate` is made. The implementation of `LinearInterestRateModelV3` can be found here: https://etherscan.io/address/0x86781a14F55677729b1C0394E06966BF8736bbbc#code If we look deeper, we can see that when the `isBorrowingMoreU2Forbidden` flag is true and `checkOptimalBorrowing` is set to true it shouldn't allow borrowing over `U_2`. However, due to an error in the implementation it will revert also if `U_WAD` is equal to `U_2` which from documentation is not by design and can cause random reverts in `PoolV3`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Liquidations will not be possible once oracle price is 0

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-766
- **Submitter:** ElCid
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/766
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-766.md

## Brief Summary

In the unlikely case that the price returned by the oracle is 0, unhealthy positions with or without bad debt won't be liquidatable because all the oracles revert with that value. If this ever happens, it is critical for `CDPVault` to be able to liquidate all of the unhealthy positions, so that losses don't grow and insolvency can be avoided.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_49_group

# No refund to the user if the amount paid is greater than the repayment amount

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-700
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/700
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-700.md

## Brief Summary

If the user contributes more to repayment than the debt itself. The difference is not returned to him. The design of the protocol assumes that the user will deposit the exact amount, but it cannot be ruled out that the opposite may happen and the user may lose his funds

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_53_group

# Insecure Access Control in CDPVault.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-692
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/692
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-692.md

## Brief Summary

CDPVault.sol is using an unsafe access control mechanism. The `accessControl` mechanism provided by OpenZeppelin is used in this contract, which casts the role as `bytes32` hash of a certain string. The role system has the capabilities to regulate authentication permissions. However, the smart contract is using the keccak256 hash of certain role names. If a collision of hashes occurs, it may allow an unauthorised individual access to these sensitive functions. These roles include: `VAULT_CONFIG_ROLE`, `VAULT_UNWINDER_ROLE`, `DEFAULT_ADMIN_ROLE`, and `PAUSER_ROLE`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Exposure to Unpriceable Tokens in CDPVault.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-699
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/699
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-699.md

## Brief Summary

The `token` in the smart contract is not required to comply with a specific interface. Particularly, the token does not need a price feed. Hence, it may be possible to provide unpriceable tokens in the CDPVault, which can lead to position manipulation and imbalanced debt positions. The issue is in this piece of code: In the constructor of `CDPVault`, the contract expects the ERC-20 tokens from constants passed. However, there is no check if the token has a corresponding price feed in the oracle's smart contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation in Flashlender.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-720
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/720
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-720.md

## Brief Summary

The contract Flashlender.sol does not validate the `amount` parameter from `flashLoan` and `creditFlashLoan` functions before use. This means, a malicious user could provide an excessive loan amount causing potential overflow issues.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Unchecked Return Values in Flashlender.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-724
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/724
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-724.md

## Brief Summary

In the `flashLoan` and `creditFlashLoan` functions, there is an external call to `receiver.onFlashLoan` and `receiver.onCreditFlashLoan` methods respectively. However, the return values of these functions are not being checked for correctness, resulting in potential issues if these functions do not complete as expected. There is a check against the `CALLBACK_SUCCESS` and `CALLBACK_SUCCESS_CREDIT` constants but that's insufficient as the receiver's contract might contain an error and still return the correct constant.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# PoolV3.sol is not fully up to EIP-4626's specification

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-746
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/746
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-746.md

## Brief Summary

The function 'maxMint' and 'maxDeposit' do not follow EIP-4626's specification. 'function maxMint MUST return the maximum amount of shares mint would allow to be deposited to receiver and not cause a revert, which MUST NOT be higher than the actual maximum that would be accepted (it should underestimate if necessary). This assumes that the user has infinite assets, i.e. MUST NOT rely on balanceOf of asset.'

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unlimited Token Generation Exploit in PoolV3.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-759
- **Submitter:** JC
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/759
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-759.md

## Brief Summary

In PoolV3.sol, just before the end, there is a function called `mintProfit`. This function is allowing the minting of unlimited tokens by the caller if the caller is marked as a credit manager. This can pose a massive security issue as it can potentially be exploited by an attacker to mint unlimited tokens, leading to high inflation and decreasing the value of existing tokens substantially.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Hardcoded Timestamp Values can introduce rounding errors

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-764
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/764
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-764.md

## Brief Summary

Hardcoding timestamp values and using the difference between timestamps to compute interest could potentially introduce rounding errors. Such financial computations should rely on timestamp differences as little as possible. In PoolV3.sol, the `lastBaseInterestUpdate` and `lastQuotaRevenueUpdate` are hard-coded timestamp values. These are used to compute the base interest rate and the quota revenue accrued since the last update, respectively (functions `_calcBaseInterestAccrued` and `_calcQuotaRevenueAccrued`).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unrestricted Withdraw Function in Silo.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-771
- **Submitter:** JC
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/771
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-771.md

## Brief Summary

The function "withdraw" is used to move funds from the contract to a specified address. It is only supposed to be accessed by the STAKING_VAULT, as per the "onlyStakingVault" modifier. However, there is no check incorporated to ensure that the "to" address is not a malicious one or an attacker's address. This could allow an attacker to drain funds from the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_83_group

# No Access Control on Contract Initialization in Silo.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-774
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/774
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-774.md

## Brief Summary

The contract Silo.sol has one common vulnerability related to initializations. The constructor of the contract takes two parameters, `_stakingVault` and `_lpEth`, which are then set as immutable state variables, `STAKING_VAULT` and `lpETH` respectively. The issue here is that there is no access control to prevent an unauthorized entity from deploying the contract and being able to initialize it with any arbitrary address. This could be a potential security risk as it could lead to unauthorized functionality in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inefficient Removal of Vaults in VaultRegistry.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-778
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/778
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-778.md

## Brief Summary

`_removeVaultFromList` function has an inefficient way of removing vaults from the 'vaultList' array. The function iterates over the 'vaultList' array until it finds a match, then it moves the last element to the position of the loop iterator and pops the last element from the array. This could lead to high gas usage when removing vaults, particularly when the 'vaultList' array grows large. Furthermore, the operation is not entirely safe as it can potentially reorder the vaults in the 'vaultList' array, which could lead to unexpected behavior. This function violates the "Principle of Least Astonishment," which states users should not be surprised by the behavior of a system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_139_group

# Potential Configuration Parameter Truncation Risk

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-645
- **Submitter:** Jerry0x
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/645
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-645.md

## Brief Summary

In the `setParameter` function, converting the `uint256` type `data` to `uint128` or `uint64` type may result in truncation. This means that if the value of `data` exceeds the maximum value of the target type, the higher bits will be discarded, which may lead to data loss or unexpected behavior. Such truncation can cause the contract configuration parameters to be set incorrectly, thereby affecting the normal operation and security of the contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# Precision loss in debt Calc for CDPVault Repayments

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-304
- **Submitter:** K42
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/304
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-304.md

## Brief Summary

I found precision loss in the debt calculation of [CDPVault](https://github.com/code-423n4/2024-07-loopfi/blob/main/src/CDPVault.sol#L652): - Impact is incorrect debt tracking, causing users to repay more or less debt than they actually owe. I wrote a

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Approval Race Conditions

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-235
- **Submitter:** Kavin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/235
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-235.md

## Brief Summary

Detailed description of the impact of this finding. Approval race conditions can lead to unexpected behaviors where an attacker can front-run the transaction. This could result in unauthorized spending of tokens, potentially leading to a loss of funds for the users of the contract. This issue poses a medium risk to the integrity and security of the token transfer operations within the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# contracts are vulnerable to feeontransfer accounting related issue

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-20
- **Submitter:** MFaizal14
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/20
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-20.md

## Brief Summary

Contracts that do not account for fee-on-transfer tokens may incorrectly calculate balances, leading to discrepancies between expected and actual token amounts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_76_group

# recoverERC20() can be used as a backdoor by the owner to retrieve rewardsToken

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-371
- **Submitter:** MSaptarshi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/371
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-371.md

## Brief Summary

Admin might recover the staked token, instead of the rewards token

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_97_group

# Griefing attack possible by continuous deposit and withdrawal

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-404
- **Submitter:** MSaptarshi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/404
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-404.md

## Brief Summary

A malicious user can flood the deposit() by making thousands of deposits for only dust amount. For a very small amount he can flood with thousands of requests without any impact for him, He can withdraw it similarly without any problem for him

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lenders are unable to stake lpETH.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-393
- **Submitter:** Modey
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/393
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-393.md

## Brief Summary

Lenders are unable to receive rewards from staking lpETH, because there is no functionality for this in `StakingLPEth.sol`

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_95_group

# Lack of Access Control in `deposit`, `withdraw`, and `modifyCollateralAndDebt` Functions

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-150
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/150
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-150.md

## Brief Summary

* The `CDPVault.sol` contract is vulnerable due to the lack of access control in the `modifyCollateralAndDebt` function, which is called by both `deposit` and `withdraw` functions. Implementing strict access controls is essential to secure these functions and prevent potential attacks on user balances and the contract state. Affected Code: **`deposit` Function: CDPVault.sol#L223** **`withdraw` Function: CDPVault.sol#L239** **`modifyCollateralAndDebt` Function:CDPVault.sol#L373** Impact: - **Unauthorized State Modification**: Any external contract or address can invoke `modifyCollateralAndDebt` directly, bypassing intended controls and potentially manipulating user collateral and debt balanc...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# Lack of Fee Calculation in `_amountWithFee` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-495
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/495
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-495.md

## Brief Summary

This issue can lead to incorrect asset calculations during minting, potentially resulting in users not being charged the intended fees, which could affect the protocol’s economics and lead to financial loss. **Description**: The `_amountWithFee` function in this contract `PoolV3.sol` the provided code is designed to apply a fee to the calculated assets. However, the current implementation of the function simply returns the input amount without any modification, effectively bypassing the fee calculation. This results in users receiving assets without any fee deduction, which might not align with the intended logic of the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Redundant Function Logic in `_convertToAssets`

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-496
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/496
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-496.md

## Brief Summary

The issue can lead to an inaccurate conversion of shares to assets, particularly in cases where the vault has a non-zero total supply and total assets. This can result in incorrect asset distribution to users, potentially causing financial discrepancies. **Description**: The `_convertToAssets` function is intended to convert a given number of shares into the corresponding amount of assets. However, the current implementation merely returns the number of shares without performing any meaningful conversion. The commented-out code within the function suggests that there is a more complex logic intended to handle cases where the total supply and total assets are non-zero, but this logic is curr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inadequate Error Handling in ChainlinkOracle's getStatus Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-728
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/728
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-728.md

## Brief Summary

The ChainlinkOracle contract provides a `getStatus` function to check the validity of price data for a given token. The `getStatus` function fails to properly communicate errors that occur during the price fetching process. It only returns a boolean status, which doesn't provide enough information about potential failures. The current implementation of `getStatus` is as follows: This implementation has two main problems: 1. It silently catches any exceptions that might occur in `_fetchAndValidate`. 2. It doesn't distinguish between different types of failures (e.g., stale data, zero price, or external call failure).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Race Condition in Price Validation can Lead to False Positives

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-729
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/729
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-729.md

## Brief Summary

The `BalancerOracle` contract is designed to provide price information for tokens in a Balancer pool. The `_getStatus` function is crucial for determining whether the current price data is valid and up-to-date. The `_getStatus` function has a race condition that can lead to returning a false positive status immediately after an update, even if the new price hasn't been properly set.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Non compliance with ERC-4626 Standard for totalAssets

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-731
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/731
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-731.md

## Brief Summary

According to ERC-4626, `totalAssets()` should return the total amount of underlying assets managed by the vault. This typically includes the actual balance of underlying tokens held by the vault plus any assets deployed elsewhere (e.g., lent out). The current implementation returns `expectedLiquidity()`, which, based on its definition elsewhere in the contract, includes not only the actual assets but also accrued interest and quota revenue that hasn't been realized yet. ERC-4626 expects `totalAssets()` to reflect the current, real value of assets, not projected or expected future values.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Preview Functions Don't Revert Appropriately According to ERC4626 standards

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-761
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/761
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-761.md

## Brief Summary

The preview functions (e.g., `previewDeposit`, `previewWithdraw`, `previewRedeem`) do not revert when the function they are previewing would revert, which is against ERC4626 standards. Impact - This can lead to unexpected behavior, as users may rely on these functions to assess the viability of their transactions without proper feedback on potential reverts. Details 1. **Deposit Function** - The `deposit` function will revert if the pool is paused, if the receiver address is zero, or if the assets sent are less than the required minimum due to fees. - The `previewDeposit` function does not revert even if the underlying conditions (like pool status) would cause the actual deposit to fail. Co...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# CDPVault::withdraw() should transfer funds to the "To" address passed as parameter, not the msg.sender

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-245
- **Submitter:** TECHFUND-inc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/245
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-245.md

## Brief Summary

When the collateral is withdrawn, the `CDPVault::withdraw()` function accepts `to` address as the target recipient of the tokens. But, the token are instead being transferred to the `msg.sender`, the caller of the function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_96_group

# "_penaltyInfo" can revert due to no check on "vestDuration"

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-582
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/582
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-582.md

## Brief Summary

Detailed description of the impact of this finding. In _penaltyInfo there is no check on vestDuration which can cause the penaltyAmount greater than earning.amount which can cause _ieeWithdrawableBalance,withdraw to revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# _previewReward can revert.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-637
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/637
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-637.md

## Brief Summary

Detailed description of the impact of this finding. IOracle(feed).spot(asset() can be 0 and _previewReward can revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No array length check in multisend

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-738
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/738
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-738.md

## Brief Summary

Detailed description of the impact of this finding. Here there is no array length check in multisend. There is no array length check whether targets ,data and delegateCall are all same length.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Validation in setParameter Allows Unsafe Configurations Leading to Potential Under-Collateralization

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-504
- **Submitter:** bhatmuneeb
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/504
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-504.md

## Brief Summary

The `setParameter` function in the `CDPVault.sol` contract is designed to allow privileged users (those with the `VAULT_CONFIG_ROLE`) to modify several critical configuration parameters. These parameters, including `debtFloor`, `liquidationRatio`, `liquidationPenalty`, and `liquidationDiscount`, are vital to the proper functioning and stability of the vault. However, the current implementation does not include validation checks for the values being set. This oversight creates a vulnerability where these parameters can be configured with unsafe values, potentially leading to under-collateralization of user positions. Such misconfigurations could destabilize the vault, leading to significant...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked Arithmetic in calcDecrease Function Can Lead to Integer Overflow/Underflow

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-522
- **Submitter:** bhatmuneeb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/522
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-522.md

## Brief Summary

The `calcDecrease` function in `CDPVault.sol` performs several arithmetic operations to compute the new debt and interest values after a debt repayment. Although Solidity 0.8.x includes built-in checks for overflow and underflow, it is still important to be cautious with arithmetic operations, especially when large values are involved. The function does not use safe math libraries, which could result in incorrect calculations under specific edge cases where large or unexpected values are processed. If such values lead to overflow or underflow, the function may calculate incorrect debt values, leading to financial discrepancies and potential losses for the protocol. The c`alcDecrease` functi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_233_group

# Lack of Zero-Address Check in mintProfit Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-725
- **Submitter:** bhatmuneeb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/725
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-725.md

## Brief Summary

The mintProfit function, which mints new tokens to the treasury based on a specified amount, does not include a check to ensure that the treasury address is not the zero address. If the treasury address is ever mistakenly set to the zero address, calling mintProfit could result in the accidental minting of tokens to the zero address, effectively burning them permanently. Zero-Address Risk: There is no validation to ensure that the treasury address is not set to the zero address. If this occurs, tokens intended for the treasury could be irretrievably lost. Unrecoverable Loss: If tokens are minted to the zero address, they are effectively burned and cannot be recovered, leading to a direct fi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inadequate Access Control in `setAllowed` Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-733
- **Submitter:** bhatmuneeb
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/733
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-733.md

## Brief Summary

The setAllowed function allows a controller to grant or revoke withdrawal and redemption permissions for specific accounts even when the pool is locked. However, there is no limitation or logging mechanism on who can be added or removed, which could be exploited by a malicious or compromised controller to allow unauthorized accounts to access pool funds during a lock period. Lack of Granular Control: The function allows any controller to add or remove accounts from the _allowed mapping, granting them privileges to withdraw or redeem from the pool even when it is locked. There is no mechanism to limit or monitor these changes, which could be abused. Potential for Exploitation: A malicious co...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# PositionActions.sol : CreditFlashloan function should be checked as well for fix

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-276
- **Submitter:** boraichodrunkenmaster
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/276
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-276.md

## Brief Summary

Can lead to loss of funds for user etc...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unauthorised call of the flashLoan function in the Flashlender contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-101
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/101
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-101.md

## Brief Summary

Detailed description of the impact of this finding. The vulnerability allows any external user to execute the flashLoan function without proper authorisation checks, potentially leading to unauthorised and exploitative flash loans. This can result in financial losses for the protocol if malicious users manipulate loan conditions for personal gain. The flashLoan function in the Flashlender contract does not implement sufficient authorisation checks to ensure that only authorised entities can call the function. The lack of access control allows any external actor to invoke the flash loan functionality, leading to potential misuse of protocol resources. The absence of authorisation checks on t...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_116_group

# Division by Zero in deposit Function within the CDPVault contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-142
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/142
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-142.md

## Brief Summary

Detailed description of the impact of this finding. This report highlights a critical division by zero vulnerability in the deposit function of the CDPVault contract. If amount or tokenScale are zero, a division by zero will cause the transaction to revert or not, either way consuming unnecessary gas associated with the transaction with or without making any state changes. This leads to waste of a transaction or a failed transactions and a poor user experience. The contract could become unreliable and frustrating for users if division by zero errors occur frequently. This impacts the overall trust in the contract’s functionality.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Lack of Input Validation in setCreditManager Function within PoolQuotaKeeperV3 contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-341
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/341
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-341.md

## Brief Summary

Detailed description of the impact of this finding. The setCreditManager function does not validate the token and vault parameters, which could lead to erroneous or malicious configurations if zero addresses are inadvertently set. This can cause downstream errors in other parts of the contract that rely on creditManagers mapping to be correctly initialised.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Lack of Parameter Validation for ERC20 Token Recovery in the RecoverERC20 contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-350
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/350
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-350.md

## Brief Summary

Detailed description of the impact of this finding. The lack of a balance check before transferring ERC20 tokens can result in failed transactions when the requested tokenAmount exceeds the contract’s balance. This not only leads to wasted gas fees for users but also results in a poor user experience due to unexpected transaction failures. The absence of this check can cause operational inefficiencies and confusion among users interacting with the contract. ***Use Case*** A similar issue was encountered in the DeFi protocol Compound. Users experienced transaction failures when attempting to withdraw more funds than were available in a particular market. This led to wasted gas fees and user...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unauthorized Access to setRelock Function in the MultiFeeDistribution contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-87
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/87
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-87.md

## Brief Summary

Detailed description of the impact of this finding. The setRelock function in the MultiFeeDistribution contract can be called by any address without any access control. This function allows users to enable or disable the auto-relock feature. Without proper access restrictions, unauthorised users can alter the auto-relock status. Location: MultiFeeDistribution.sol *

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_123_group

# Unsafe Position Allowed in modifyCollateralAndDebt Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-158
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/158
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-158.md

## Brief Summary

When `deltaDebt` is zero and `deltaCollateral` is negative, the function incorrectly determines that the position is safe, even if the collateral value is insufficient to cover the debt. The root cause is in the following code block in the `modifyCollateralAndDebt` function: [CDPVault.sol#L451-L454](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L451-L454) The condition `(deltaDebt > 0 || deltaCollateral < 0)` checks if the debt is increasing or the collateral is decreasing, which would make the position riskier. However, the subsequent check using the `_isCollateralized` function fails to handle the case when `deltaDebt` is zero...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_236_group

# Overflow in liquidatePositionBadDebt() Leading to System Insolvency

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-175
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/175
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-175.md

## Brief Summary

`liquidatePositionBadDebt()` responsible for liquidating positions that have fallen below the required collateralization ratio and have accumulated bad debt there is a vulnerability in the calculation of the loss variable within `liquidatePositionBadDebt()` that can lead to an overflow, can cause the system to become insolvent, as the new total debt may exceed the value of the collateral multiplied by the spot price. [CDPVault.sol#L600-L607](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L600-L607) Impact If the `calcTotalDebt(debtData)` is significantly larger than `repayAmount`, the subtraction operation `calcTotalDebt(debtData)...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# Incorrect Minting of Shares in _deposit() Function Violates Expected Behavior

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-234
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/234
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-234.md

## Brief Summary

In the `_deposit()` function, the receiver's balance should increase by the number of shares minted. However, when the `_deposit()` function is called with shares set to 0, no shares are minted to the receiver, violating **"Receiver's balance should increase correctly"**. [PoolV3.sol#L384-L395](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/PoolV3.sol#L380-L395) Impact If the `_deposit()` function is called with shares set to 0, the user's balance will not reflect the expected increase based on the assets deposited. This discrepancy can cause confusion and loss of funds for users who expect to receive shares proportional to their deposited ass...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_228_group

# Incorrect Liquidity Update in _withdraw() Can Lead to Liquidity Underflow

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-240
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/240
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-240.md

## Brief Summary

In the PoolV3, the `_withdraw()` function incorrectly updates the available liquidity, potentially leading to a liquidity underflow. This issue arises from the `_updateBaseInterest()` function call within `_withdraw()`, where the `availableLiquidityDelta` parameter is passed as the negative of `assetsSent`. The problemat is located in [PoolV3.sol#L412-L416](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/PoolV3.sol#L412-L416) Although the intention is to subtract `assetsSent` from the available liquidity, the resulting `availableLiquidity` after the update becomes a large number, indicating an underflow or incorrect calculation within `_updateB...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Debt Reduction in `repayCreditAccount()` Due to Precision Loss

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-256
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/256
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-256.md

## Brief Summary

`repayCreditAccount()` logic can lead to incorrect debt reduction when the `repaidAmount` parameter exceeds `type(uint128).max`. This problem arises from the conversion of `repaidAmount` to `uint128` before updating the `_totalDebt.borrowed` and `cmDebt.borrowed` variables. The root cause of the issue is the conversion of `repaidAmount` from `uint256` to `uint128` using the `toUint128()` on [line 540](https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/PoolV3.sol#L540). This conversion can lead to a loss of precision if `repaidAmount` exceeds `type(uint128).max`. The affected [lines, 572 - 573](https://github.com/code-423n4/2024-07-loopfi/blob/5787...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# DoS if user Redeem Exact Amount of the asset in stakingLPEth

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-579
- **Submitter:** golu
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/579
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-579.md

## Brief Summary

DoS if user Redeem Exact Amount of the asset in stakingLPEth

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `positions` variable is not updated

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-608
- **Submitter:** grearlake
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/608
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-608.md

## Brief Summary

In multiple functions in `CDPVault` contract: `modifyCollateralAndDebt()`, `liquidatePosition()`, `liquidatePositionBadDebt()`, all variable in `positions` is updated in temporatory variable: Position memory position = positions[owner]; <-- DebtData memory debtData = _calcDebt(position); . . . . . . . position = _modifyPosition(owner, position, newDebt, newCumulativeIndex, deltaCollateral, totalDebt); These temporatory variable is not writen back to its origin, which mean the value of them is not kept over time. Impact Variable value is not updated.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unable to withdraw in `PositionAction` contract

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-730
- **Submitter:** grearlake
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/730
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-730.md

## Brief Summary

`creditFlashLoan()` function in `Flashlender` contract is used to flash lending in the pool function creditFlashLoan( ICreditFlashBorrower receiver, uint256 amount, bytes calldata data ) external override nonReentrant returns (bool) { uint256 fee = wmul(amount, protocolFee); uint256 total = amount + fee; pool.lendCreditAccount(amount, address(receiver)); emit CreditFlashLoan(address(receiver), amount, fee); if (receiver.onCreditFlashLoan(msg.sender, amount, fee, data) != CALLBACK_SUCCESS_CREDIT) revert Flash__creditFlashLoan_callbackFailed(); // reverts if not enough Stablecoin have been send back underlyingToken.transferFrom(address(receiver), address(pool), total); pool.repayCreditAccount...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# Over-Leverage Risk in depositWithReferral Function Due to Missing Asset Limit Checks

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-296
- **Submitter:** hassan-truscova
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/296
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-296.md

## Brief Summary

The vulnerability allows users to deposit potentially excessive amounts of assets, leading to over-leverage. This exposes users to higher risks of market volatility and credit losses beyond the system's designed risk tolerance. Consequently, such over-leverage could destabilize the financial health of LoopFi, increasing the risk of defaults and adversely affecting all users and stakeholders.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_91_group

# Denial of Service Through Gas Exhaustion in withdraw Function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-355
- **Submitter:** hassan-truscova
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/355
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-355.md

## Brief Summary

The `withdraw` function is vulnerable to gas exhaustion attacks, which can significantly impact the contract’s reliability and performance. An attacker can exploit this vulnerability to cause the function to consume excessive gas, leading to out-of-gas errors.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Pausing `ChefIncentivesController` contract by the owner can unexpectedly block the users from claiming the reward

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-362
- **Submitter:** hassan-truscova
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/362
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-362.md

## Brief Summary

The vulnerability in `ChefIncentivesController` contract arises from the ability of the owner to unilaterally pause the contract. This can prevent users from performing critical operations such as claiming their rewards by calling `claim` function or accessing their funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Contract contains payable functions but no withdraw/sweep function

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-299
- **Submitter:** jauvany
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/299
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-299.md

## Brief Summary

In smart contract development, particularly for Ethereum, having payable functions without a corresponding withdraw or sweep function can lead to potential issues. Payable functions allow the contract to receive Ether, but without a mechanism to withdraw these funds, the Ether can become locked within the contract indefinitely. This situation might be intentional in some cases (like a burn function), but generally, it’s a design oversight. A withdraw or sweep function is necessary to transfer Ether out of the contract to a specific address, typically the owner's or a designated recipient. Without this, the contract lacks flexibility in managing its funds, potentially leading to lost or inac...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_196_group

# Attacker can potentially manipulate FlashLender into a profitable attack

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-145
- **Submitter:** jigster
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/145
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-145.md

## Brief Summary

Flashlender can be called from an arbitrary address and execute a flashloan. During the course of this flashloan, it is possible for a malicious vault whose modifyCollateralAndDebt method can implement arbitrary logic. While not demonstrated here, so long as this arbitrary logic can result in a profitable attack, ie repay the flashloan, the call will succeed. This

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# updateRates at the PoolQuotaKeeperV3 is vulnerable to front-runs that harm the quota revenue amounts

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-612
- **Submitter:** joaovwfreire
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/612
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-612.md

## Brief Summary

The PoolQuotaKeeperV3 updateRates function updates the quotaRevenue by multiplying the amount of amount borrowed by a credit manager and the current rate, then it call setQuotaRevenue at the Pool contract: Notice this quotaRevenue relies on the amount instantly borrowed by the creditManager: This borrowed amount can be increased by lending to a credit account and decreased by repaying a loan. This creates the opportunity for malicious users to repay their loans right before a Gauge calls updateRates in order to decreased the total amount of quotaRevenue. In the context of Mainnet, this is a realistic risk through front-runs. Impact Malicious parties can opt to repay their borrows right befo...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_108_group

# Reentrancy Risk in PositionActionPendle Contract Due to Delegated Calls

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-246
- **Submitter:** johnthebaptist
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/246
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-246.md

## Brief Summary

The `PositionActionPendle` contract contains functions that delegate calls to external contracts (`poolAction.join` and `poolAction.exit`). These delegated calls could introduce reentrancy risks if the external functions are not properly secured. Reentrancy attacks can allow an attacker to repeatedly call a function before the previous execution is complete, potentially manipulating state variables or draining funds from the contract. Specific Impact: 1. **Manipulation of State Variables**: An attacker could re-enter the contract and manipulate state variables, leading to incorrect contract behavior. 2. **Draining Funds**: An attacker could repeatedly withdraw funds, draining the contract's...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_102_group

# Possible precision loss in `BAL/USD` price from `AuraVault::_chainlinkSpot` calllculation of `price` could inflate rewards calculations

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-169
- **Submitter:** josephxander
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/169
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-169.md

## Brief Summary

The `AuraVault::_chainlinkSpot` is used to return the `BAL/USD` price in 8 decimals s defined by `AuraVault::BAL_CHAINLINK_DECIMALS` Due to mathematical operations, the utilized `price` is inflated to `e18`.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# There are no checks to ensure core parameters are set before core functions can be callable in `CDPVault`

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-219
- **Submitter:** josephxander
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/219
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-219.md

## Brief Summary

`CDPVault` has 2 `configuration` functions that sets core parameters of the contract; `CDPVault::setParameter(bytes, uint)` and `CDPVault::setParameter(bytes, address)`. If upon deployment, an actor can immediately perform certain actions that defy the intended baseline parameter bounds for such. For instance, one could deposit below the `debtFloor`, or deposit and self liquidate without the cost of `liquidationPenalty`. Without a flag or check or modifier of somesort that verifies that these mandatory parameters are set, some of these functions that depend on protocol sanitary parametrs can be executed at the protrocol's peril.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unsafe Casting

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-270
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/270
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-270.md

## Brief Summary

Downcasting int/uints in Solidity can be unsafe due to the potential for data loss and unintended behavior.When downcasting a larger integer type to a smaller one (e.g., uint256 to uint128), the value may exceed the range of the target type,leading to truncation and loss of significant digits Impact Truncation and loss of significant digits

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_126_group

# Incorrect Assembly Shift Parameter Order

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-271
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/271
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-271.md

## Brief Summary

The incorrect order of parameters in the `shl` (shift left) assembly operation can lead to unintended behavior in the smart contract. In Solidity's inline assembly, the `shl` function should be used as `shl(shift, value)`, where shift is the number of bits to shift left and value is the value to be shifted. If the parameters are reversed, as in `shl(value, shift)`, the result will be different from the intended behavior and can lead to logical errors or incorrect computations.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Return value of the function call is not checked

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-273
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/273
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-273.md

## Brief Summary

Ignoring the return values of function calls can lead to unintended behavior or missed errors in smart contracts. Functions that return values, especially those involving external contract interactions, may indicate the success or failure of an operation. Failing to check these return values can mask issues such as transaction failures, incorrect operations, or unexpected results, potentially leading to financial losses or compromised contract functionality.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Storage Array Edited with Memory

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-274
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/274
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-274.md

## Brief Summary

Storage reference is passed to a function with a memory parameter. This will not update the storage variable as expected.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Relying on Highly Risky Comparison Between `bytes32` and String Literals

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-326
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/326
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-326.md

## Brief Summary

Relying on implicit conversions can be risky because: - Implicit conversions might not handle all possible cases correctly, especially with different string lengths or character encodings. - Future versions of Solidity might handle these conversions differently, leading to unexpected behavior(especially in this case with the floating pragma, this is a huge problem which may arrise in future). - Code relying on implicit conversions can be harder to understand and maintain. Impact The current implementation of the `setParameter` functions in `CDPVault.sol` and `AuraVault.sol` involves comparing `bytes32` values with string literals directly. While the tests may pass in the current implementat...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect debt data calculation due to Invalid Validation of the address parameter of liquidatePosition() and liquidatePositionBadDebt().

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-480
- **Submitter:** la-arana-inteligente
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/480
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-480.md

## Brief Summary

The check for 'if (owner == address(0) || repayAmount == 0) revert CDPVault__liquidatePosition_invalidParameters();' on lines 511 and 581 could be bypassed by passing a non-zero 'owner' address that doesn't correspond to any existing position which results in incorrect logic execution further down the line.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Zero-cost minting (due to inheritance from the implemented ERC4626 contract) could potentially give an attacker a disproportionate share of StakingLPEth.sol

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-552
- **Submitter:** la-arana-inteligente
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/552
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-552.md

## Brief Summary

An attacker could mint a large number of shares at zero asset deposit, effectively gaining a disproportionate share of the pool. Details If the `mint` function of ERC4626.sol is accessible to `StakingLPEth.sol` (due to inheritance which is the case here) and the share price is zero (or very very low) due to contract initialization (or a specific edge case), an attacker could mint a large number of shares at zero or neaar-zero asset deposit, effectively gaining a disproportionate(unfair) share of the pool. Note: While `_checkMinShares` is a useful function for ensuring that the total supply of shares doesn’t fall below a small threshold, it does not address the root cause of the exploit desc...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Unspecified permission type could allow permitted address for

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-55
- **Submitter:** lanrebayode77
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/55
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-55.md

## Brief Summary

In the , different check was carried out against the caller who is modifying a user position. The check aims to verify the caller is either the owner of the poison or permitted by the owner to modify the position in certain ways. The problem with the check is that the does not check for specific permission. As a matter of fact, there is no method in Permission.sol that allows a user to give specified permission. This it is possible for the caller to perform a different modification contrary to what the owner permitted.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_61_group

# [M-01] Malicious consumption of fee

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-138
- **Submitter:** lanyi2023
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/138
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-138.md

## Brief Summary

Potential malicious depletion of receiver's token balance through excessive fees.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Deposits and withdrawals are rounded down in the same direction which is incorrect.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-161
- **Submitter:** lightoasis
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/161
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-161.md

## Brief Summary

Deposits and withdrawals are rounded down in the same direction, which is incorrect as they are two different operations. Vulnerability Details There are rounding errors in the cdp vault. Deposits and withdrawals are rounded down in the same direction, which is incorrect as they are two different operations. Deposits adds collateral to the vault, withdrawals removes collateral from the vault. Hence, they cannot be rounded in the same direction. Deposits should be rounded down and withdrawals should be rounded up.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Slippage check enforces max slippage on all users.

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-52
- **Submitter:** lightoasis
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/52
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-52.md

## Brief Summary

Incorrect slippage checks enforces max slippage on all users. Bug Description The slippage checks in `setAutocompound` and `setUserSlippage` enforces all users to use the max slippage of 10% which shouldn't be. Users have different slippage tolerance. Some users would want a lower slippage like 2 - 5% as not all users would want to use the max slippage of 10% which is too extreme.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_117_group

# Corruptible Upgradability Pattern

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-331
- **Submitter:** mrMorningstar
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/331
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-331.md

## Brief Summary

Storage of [BalancerOracle](https://github.com/code-423n4/2024-07-loopfi/blame/57871f64bdea450c1f04c9a53dc1a78223719164/src/oracle/BalancerOracle.sol#L53) contract might be corruptible during upgrade.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `_massUpdatePools()` is susceptible to DoS with block gas limit

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-640
- **Submitter:** nadin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/640
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-640.md

## Brief Summary

`_massUpdatePools()` is used to update the reward variables for all pools based on `poolLength()`. Hence, it is an unbounded loop, depending on the length of reward pools. If `registeredTokens.length` is big enough, block gas limit may be hit.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# dubious typecast

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-33
- **Submitter:** nnamdi0482
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/33
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-33.md

## Brief Summary

Detailed description of the impact of this finding. the typecast id dubious and may not be understood

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_68_group

# Potential for Optimization in Supply Rate Calculation to Ensure Precise Interest Payments

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-514
- **Submitter:** obingo76
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/514
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-514.md

## Brief Summary

Within PoolV3.sol, in the ``supplyRate()`` function, if the ``_totalDebt.borrowed`` amount approaches the ``totalAssets`` value, the interest rate calculation could result in truncation due to division. The ``baseInterestRate`` is in RAY form ``(scaled by 1e27)``, and when it's multiplied by ``_totalDebt.borrowed`` and then divided by ``totalAssets``, it could lead to rounding down to the nearest integer value.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_105_group

# StakingLPEth can be arbitraged by any users

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-556
- **Submitter:** pks_
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/556
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-556.md

## Brief Summary

From the [loopfi doc](https://docs.loopfi.xyz/the-protocol/lending-passive-eth-yield) about lpETH usage: we can know, any lpETH stakers can get slpETH, and the slpETH value will increase over time. And the slpETH increase as `the protocol yield is transferred into the Staking smart contract`, we can also find the related [test cases](https://github.com/code-423n4/2024-07-loopfi/blob/4f508781a49ffa53511e7e5ed6cda0ff0eb5bdc5/src/test/unit/StakingLP.t.sol#L50-L57) to understand it. So any users can monitor the mempool, staking lpETH before the protocol yield is transferred to `StakingLPEth` contract to receive slpETH, then wait for the protocol yield is transferred to `StakingLPEth` contract,...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_78_group

# Wrong insolvent calculation can cause users borrow more or avoid be liquidated

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-566
- **Submitter:** pks_
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/566
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-566.md

## Brief Summary

`CDPVault#_isCollateralized` is called by `CDPVault#liquidatePosition` and `CDPVault#modifyCollateralAndDebt` to check if the position is insolvent or not: However, the insolvent calculation is incorrect because the collateral value calculation is based on `position.collateral * spotPrice_` but the debt value is based on `debtData.debt + debtData.accruedInterest`, the token units is not the same, as collateral value is about `X $usd`, but the debt is the debt token units like `WETH` or something else. For example, if collateral token is 1 WETH or ETH derivatives and the price is about 3000 $usd, but the debt token is SOL and the price is 150 $usd. If he want to borrow 1000 SOL, then collate...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Two separate functions responsible for converting to shares will cause weird errors

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-410
- **Submitter:** samuraii77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/410
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-410.md

## Brief Summary

Two separate functions responsible for converting to shares will cause weird errors

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# base interest rate could be calculated incorrectly if a user withdraws more than available

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-529
- **Submitter:** silver_eth
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/529
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-529.md

## Brief Summary

base interest rate could be calculated incorrectly if a user withdraws more than available

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Debt repayment logic yields entirely different results if the user chooses to repay <= quotaInterest instead of the whole accrued interest at once

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-515
- **Submitter:** wallstreetvilkas
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/515
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-515.md

## Brief Summary

In order for the user to repay it's debt, it has to call modifyCollateralAndDebt() (https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L367C14-L367C37). When repaying debt, the following else if is executed (https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L402-L432), and, if the amount that the user is repaying isn't the total debt of the position, this else is executed: (https://github.com/code-423n4/2024-07-loopfi/blob/57871f64bdea450c1f04c9a53dc1a78223719164/src/CDPVault.sol#L418-L426). The problem arises in calcDecrease() function (https://github.com/code-423n4/2024-07-lo...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Deposit, mint and claim can be DOSed due to lack of safeApprove to 0

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-662
- **Submitter:** y0ng0p3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/662
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-662.md

## Brief Summary

When depositing assets, minting shares, or claiming rewards, `rewardPool` is approved to spend the asset tokens using the Openzeppelin's `safeApprove` function without first approving to 0. The issue here is that OpenZeppelin's `safeApprove` function does not allow changing a non-zero allowance to another non-zero allowance. OpenZeppelin's `safeApprove` function will revert if the account already is approved and the new safeApprove() is done with a non-zero value. This will therefore cause all subsequent approvals of the asset token after the first approval to non-zero allowance to fail , DoSsing the AuraVault's minting, depositing and claiming functionalities.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_184_group

# Arbitrary `from` passed to `transferFrom` (or `safeTransferFrom`)

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-636
- **Submitter:** yudistira19
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/636
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-636.md

## Brief Summary

Passing an arbitrary `from` address to `transferFrom` (or `safeTransferFrom`) can lead to loss of funds, because anyone can transfer tokens from the `from` address if an approval is made.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_146_group

# Liquidators can prevent users from making their positions healthy after an unpause

- **Contest:** LoopFi
- **Slug:** 2024-07-loopfi
- **Submission:** V-95
- **Submitter:** zzebra83
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-loopfi-validation/issues/95
- **Source snapshot:** competitions/2024-07-loopfi/submissions/raw/V-95.md

## Brief Summary

When the Pool is paused, users' debt and collateral valuations may place them at risk of liquidation. However, as soon as the pool is unpaused to resume operations, MEV (Maximal Extractable Value) bots can intercept and execute transactions before users have the opportunity to take actions like repaying debt or adding collateral to avoid liquidation. This exploitation can occur in the first block after the system transitions, potentially leading to the forced liquidation of vulnerable positions. The issue has been previously identified and confirmed valid in multiple contests including:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_92_group
