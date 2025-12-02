## Verified Patterns Found: 20

## Verified Patterns Found in following Categories:

- FeeOnTransferAssumption
- ERC20DecimalsMismatch
- UnsafeAssembyTypeCasts
- EpochOrIndexMonotonicity
- PrecisionDriftAccumulation
- MaturityorGatingByPass
- UnsafeRecipient
- AccessControlOrAuthByPass
- GriefableCallbacks
- FlashLoanEconomicManipulation
- AccountingInvariantViolation
- PricePrecisionOrRoundingError



## Summary of Patterns

Launchpad Graduation Permanently Disables Fee Accrual

Rewards accrual precision loss freezes funds due to index truncation

Precision loss in RewardsTrackerLib locks rewards for standard tokens

Permanent Lock of Reward Dust due to Precision Loss

Unsafe Downcast in Reward Debt Calculation

Unsafe Downcast to uint96 in RewardsTrackerLib

Rewards lost due to precision loss in RewardsTrackerLib

Loss of rewards due to low precision in RewardsTracker accumulator

Rewards accounting precision loss leads to stuck funds

Rewards permanently lost due to precision mismatch in RewardsTracker

increaseStake and decreaseStake divert user rewards to the Launchpad contract

Precision Loss in RewardsTrackerLib leads to permanent loss of rewards

Liquidity Pool DoS via Griefable Fee Distribution Callback

Insolvency via Fee-on-Transfer Tokens in Rewards Accounting

Premature disabling of LP rewards via endRewards call

Accrued fees deleted instead of distributed in endRewardsAccrual

addRewards allows depositing mismatched quote tokens to inflate reward pool

Flash loan manipulation of LP supply allows theft of Launchpad fees

Arbitrary token in addRewards breaks reward accounting causing DoS

DoS in Launchpad Pair swaps when Distributor staking pool is empty

## Patterns



 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.endRewards

 ### Title
Launchpad Graduation Permanently Disables Fee Accrual
 ### Description/Code Snippet
The `endRewards` function in `Distributor.sol` calls `pair.endRewardsAccrual()`, which sets `rewardsPoolActive` to 0. This function is intended to be called upon launchpad graduation to unlock transfers. However, disabling `rewardsPoolActive` permanently stops `GTELaunchpadV2Pair` from calculating and sending trading fees to the Distributor. This violates the protocol invariant that launchpad pairs feed the distributor, rendering the fee accrual mechanism non-functional exactly when trading begins.
 ### Static Signals
flag disables critical fee logic, lifecycle function disables revenue stream
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: EpochOrIndexMonotonicity

 ### Relevant Function/Location: Distributor.update

 ### Title
Rewards accrual precision loss freezes funds due to index truncation
 ### Description/Code Snippet
In `RewardsTrackerLib.getAccRewardsPerShare`, the reward accumulator increment is calculated as `(pending * 1e12) / totalShares`. If `totalShares` is large (e.g. 1e24) or pending rewards are small (dust from fees), this division truncates to 0. The subsequent `update` function deletes the `pending` rewards from storage but fails to increment the `accRewardPerShare` index. As a result, these rewards are permanently erased from the distribution logic but remain counted in `Distributor.totalPendingRewards`, making them indistributable to users and un-skimmable by admins (bricking the funds).
 ### Static Signals
delete self.pendingBaseRewards, division before summation
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.getAccRewardsPerShare

 ### Title
Precision loss in RewardsTrackerLib locks rewards for standard tokens
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a fixed `PRECISION_FACTOR` of `1e12`. For a standard 18-decimal staking token (LaunchToken), `totalShares` can easily reach `1e24` (e.g., 1 million tokens). If rewards are added in a low-decimal token (e.g., USDC, 6 decimals), the calculation `(pendingRewards * 1e12) / totalShares` will truncate to zero for significant reward amounts (e.g., < 1,000,000 USDC in the example). Since `pendingRewards` are deleted upon update regardless of whether they contributed to `accRewardPerShare`, these rewards are effectively burned/locked in the contract.
 ### Static Signals
PRECISION_FACTOR = 1e12, division before multiplication, high decimals shares vs low decimals rewards
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Permanent Lock of Reward Dust due to Precision Loss
 ### Description/Code Snippet
In `RewardsTrackerLib.update`, the calculation of `accBaseRewardPerShare` uses integer division, truncating remainders (dust). These truncated amounts are removed from `pendingBaseRewards` (marking them as distributed) but are not added to the accumulator, so users can never claim them. However, `Distributor.totalPendingRewards` tracks the full added amount. `skimExcessRewards` only allows withdrawing funds in excess of `totalPendingRewards`. Consequently, the 'lost' dust remains accounted for in `totalPendingRewards` but is mathematically unclaimable, permanently locking it in the contract.
 ### Static Signals
consistent floor toward sender/receiver, divide before multiply
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: RewardsTrackerLib.stake

 ### Title
Unsafe Downcast in Reward Debt Calculation
 ### Description/Code Snippet
In `RewardsTrackerLib.sol`, the `stake`, `unstake`, and `claim` functions explicitly cast the result of `totalAccRewards` (uint256) to `uint96` when updating `userData.baseRewardDebt` and `userData.quoteRewardDebt`. For tokens with high supply (common in meme coins or localized test tokens where supply > ~7.9e28 wei, i.e., ~79 billion tokens with 18 decimals), the accumulated rewards can exceed `type(uint96).max`. The explicit cast `uint96(...)` truncates the higher bits without reverting. This results in a significantly lower recorded debt than actual. When the user subsequently claims, the formula `reward = totalAccRewards - debt` uses the full `totalAccRewards` and the truncated `debt`, yielding a massive, incorrect payout that drains the distributor.
 ### Static Signals
userData.baseRewardDebt = uint96(totalAccRewards(...)), userData.quoteRewardDebt = uint96(totalAccRewards(...))
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: Distributor.stake

 ### Title
Unsafe Downcast to uint96 in RewardsTrackerLib
 ### Description/Code Snippet
In `RewardsTrackerLib`, `baseRewardDebt` and `quoteRewardDebt` are explicitly cast to `uint96` without overflow checks. If `accRewardPerShare` grows sufficiently large (e.g., due to rewards being added when `totalShares` is very low, such as 1 wei), the debt calculation `(shares * acc) / 1e12` can exceed `type(uint96).max`. The downcast truncates the high bits, resulting in a stored debt significantly lower than the actual distributed value. This causes future `claim` calls to calculate an inflated `pendingReward`, allowing the user to drain the contract.
 ### Static Signals
uint96(totalAccRewards(...)), downcasts without range checks
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.update

 ### Title
Rewards lost due to precision loss in RewardsTrackerLib
 ### Description/Code Snippet
In `RewardsTrackerLib.update`, the calculation `(pending * PRECISION_FACTOR) / totalShares` suffers from precision loss if `totalShares` is large relative to `pending` (specifically if `pending < totalShares / 1e12`). The `pending` rewards are deleted (cleared) regardless of whether they contributed to the accumulator, leading to permanent loss of 'dust' rewards stuck in the Distributor contract.
 ### Static Signals
division before addition, pending deleted unconditionally
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: RewardsTrackerLib.update

 ### Title
Loss of rewards due to low precision in RewardsTracker accumulator
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a `PRECISION_FACTOR` of `1e12` to calculate `accBaseRewardPerShare`. When `totalShares` is large (e.g., > 1e18) and the pending reward amount is small (e.g., < 1e6), the calculation `(pending * 1e12) / totalShares` rounds down to zero. The function then resets `pendingBaseRewards` to 0 without increasing the accumulator, causing those rewards to be permanently locked in `totalPendingRewards` and unclaimable by users.
 ### Static Signals
divide before multiply, mix 6/8/18 decimals without normalization
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: RewardsTrackerLib.update

 ### Title
Rewards accounting precision loss leads to stuck funds
 ### Description/Code Snippet
In `RewardsTrackerLib.update`, the calculation `(pendingBaseRewards * PRECISION_FACTOR) / totalShares` suffers from precision loss. `PRECISION_FACTOR` is 1e12. If `totalShares` (e.g., 18-decimal LaunchToken supply ~1e27) is significantly larger than `pendingBaseRewards * 1e12`, the division yields zero. The `pendingBaseRewards` are then deleted without incrementing `accBaseRewardPerShare`. These rewards remain in the contract balance and `totalPendingRewards` variable but are mathematically inaccessible to users, permanently locking them.
 ### Static Signals
accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares)), delete self.pendingBaseRewards
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: RewardsTrackerLib.update

 ### Title
Rewards permanently lost due to precision mismatch in RewardsTracker
 ### Description/Code Snippet
In `RewardsTrackerLib.sol`, `update()` calculates `accBaseRewardPerShare` using `(pending * 1e12) / totalShares`. Since `PRECISION_FACTOR` is fixed at `1e12`, and staking tokens (LaunchTokens) typically use 18 decimals with large supplies (e.g. `1e24` wei), while rewards can be low-decimal tokens (e.g. USDC, 6 decimals), the calculation `(rewardAmount * 1e12) / totalShares` frequently truncates to zero. This causes `pendingBaseRewards` to be cleared (set to 0) without increasing `accBaseRewardPerShare`. The rewards are effectively burned from the user's perspective but remain locked in the `Distributor` contract balance. Critically, `skimExcessRewards` cannot recover them because `totalPendingRewards` still tracks these lost amounts as liabilities.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled, PRECISION_FACTOR = 1e12
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Distributor.increaseStake

 ### Title
increaseStake and decreaseStake divert user rewards to the Launchpad contract
 ### Description/Code Snippet
When `increaseStake` or `decreaseStake` is called by the `Launchpad`, pending rewards for the user (`account`) are calculated via `rs.stake` / `rs.unstake`. However, the internal function `_distributeAssets` transfers these rewards to `msg.sender` (the `Launchpad` contract) instead of the `account`. Unless the Launchpad is explicitly designed to sweep these tokens to the user (which is non-standard for this flow), the rewards are locked in the Launchpad contract.
 ### Static Signals
base.safeTransfer(msg.sender, baseAmount), msg.sender != account
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: RewardsTrackerLib.getAccRewardsPerShare

 ### Title
Precision Loss in RewardsTrackerLib leads to permanent loss of rewards
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a `PRECISION_FACTOR` of `1e12` for reward accumulation (`acc += amount * 1e12 / totalShares`). However, `totalShares` can be up to `uint96` (approx 7.9e28). For high-supply launch tokens (e.g., 1 billion tokens = 1e27 wei) and low-decimal reward tokens like USDC (6 decimals), the numerator `amount * 1e12` is frequently smaller than `totalShares`. For example, a 10 USDC reward (1e7) yields `1e7 * 1e12 = 1e19`. If shares > 10 tokens (1e19), the division results in 0. The `update` function deletes the `pendingRewards` but adds 0 to `accBaseRewardPerShare`, causing the rewards to be permanently stuck in `totalPendingRewards` (preventing skimming) but unclaimed by users.
 ### Static Signals
PRECISION_FACTOR = 1e12, division by totalShares without sufficient scaling
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._distributeLaunchpadFees

 ### Title
Liquidity Pool DoS via Griefable Fee Distribution Callback
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` calls `_distributeLaunchpadFees` during every `_update` (triggered by `swap`, `mint`, `burn`). This function calls `Distributor.addRewards`, which executes a `safeTransferFrom` to pull fees from the Pair. If this transfer fails (e.g., the reward token is paused, the Distributor address is blacklisted by the token like USDC, or the token has a malicious hook), the entire Pair transaction reverts. This allows a single external factor to freeze the liquidity pool.
 ### Static Signals
external call in loop without failure isolation, no try/catch around external hook, callback success required for core flow to proceed
 ### Assets at Risk
liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Insolvency via Fee-on-Transfer Tokens in Rewards Accounting
 ### Description/Code Snippet
The `addRewards` function accounts for the full input `amount` in `totalPendingRewards` and the `RewardsTracker` state, but uses `safeTransferFrom` to pull tokens. If the reward token has a transfer fee (FOT), the contract receives less than `amount`. This creates a discrepancy where `totalPendingRewards` exceeds the actual contract balance. Eventually, the Distributor will lack sufficient funds to pay out the last claimers, leading to insolvency.
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check, accounting based on transfer parameter
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: GTELaunchpadV2Pair.endRewardsAccrual

 ### Title
Premature disabling of LP rewards via endRewards call
 ### Description/Code Snippet
The `Distributor.endRewards` function calls `pair.endRewardsAccrual()`, which sets `rewardsPoolActive` to 0. According to the system overview, `endRewards` is called at 'Graduation', immediately after the pair is deployed. This action permanently disables the `GTELaunchpadV2Pair` from accruing fees for the Distributor, violating the documented behavior that the pair's fees feed the distributor.
 ### Static Signals
delete rewardsPoolActive, endRewards called at graduation
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: GTELaunchpadV2Pair.endRewardsAccrual

 ### Title
Accrued fees deleted instead of distributed in endRewardsAccrual
 ### Description/Code Snippet
`GTELaunchpadV2Pair.endRewardsAccrual` deletes `accruedLaunchpadFee0` and `accruedLaunchpadFee1` before calling `_update`. If this function is called when there are undistributed accrued fees (e.g., from swaps in the same block or if `_distributeLaunchpadFees` hasn't run), those fees are permanently destroyed instead of being sent to the Distributor.
 ### Static Signals
delete accruedLaunchpadFee0, call to _update after delete
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
addRewards allows depositing mismatched quote tokens to inflate reward pool
 ### Description/Code Snippet
The `addRewards` function identifies the reward pool using one of the two input tokens but fails to verify that the *second* token matches the pool's configured `quoteAsset`. An attacker can provide a valid `launchAsset` and a worthless/malicious token as the `quoteAsset`. The contract accepts the worthless token, transfers it in, and increments the pool's `pendingQuoteRewards`. Since `pendingQuoteRewards` is used to calculate the accumulator for the pool's *actual* `quoteAsset` (e.g., USDC), users will effectively claim real USDC based on the attacker's worthless deposits, draining the contract.
 ### Static Signals
rs.quoteAsset != address(0), missing require(quoteAsset == rs.quoteAsset)
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: GTELaunchpadV2Pair._getLaunchpadFees

 ### Title
Flash loan manipulation of LP supply allows theft of Launchpad fees
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` calculates the `launchpadFee` based on the ratio of `launchpadLp` balance to the total LP supply (`totalSupply`). An attacker can use a flash loan to deposit assets and mint a massive amount of LP tokens, inflating `totalSupply` and diluting the Launchpad's fee share to near zero. The attacker then performs swaps where the fee stays in the pool (captured by their majority LP position) instead of being sent to the Distributor.
 ### Static Signals
fee calculated using totalSupply, uses totalSupply in same tx as deposit/withdraw, fee share depends on balance ratio
 ### Assets at Risk
rewards, treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Arbitrary token in addRewards breaks reward accounting causing DoS
 ### Description/Code Snippet
The `addRewards` function allows a caller to specify `token1` arbitrarily. If `token0` is a valid launch asset, the function assumes `token1` is the corresponding quote asset without validation. An attacker can call `addRewards(validLaunchAsset, maliciousToken, 0, amount)`. This inflates the internal `pendingQuoteRewards` (which tracks the valid quote asset, e.g., USDC) while transferring and tracking `totalPendingRewards` for the malicious token. When users claim, the contract attempts to transfer the real quote asset. The `_decreaseTotalPending` check fails because `totalPendingRewards[USDC]` was never incremented, causing a revert. This permanently prevents all users from claiming rewards (DoS).
 ### Static Signals
missing validation of token1 == rs.quoteAsset, update internal state based on unchecked input asset
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
DoS in Launchpad Pair swaps when Distributor staking pool is empty
 ### Description/Code Snippet
 The `GTELaunchpadV2Pair` contract logic mandates fee distribution to the `Distributor` during every `swap()` that accrues fees (`_update` -> `_distributeLaunchpadFees` -> `Distributor.addRewards`). However, `Distributor.addRewards` explicitly reverts with `NoSharesToIncentivize` if `totalShares == 0`. If all users unstake their tokens (or if the pool has 0 shares for any reason, e.g. post-launch sell-off), any swap attempt on the Pair will revert due to this check. This creates a denial of service on the main liquidity pool dependent on the state of the external staking contract.
 ### Static Signals
revert NoSharesToIncentivize, fee distribution logic coupled to external state
 ### Assets at Risk
fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

