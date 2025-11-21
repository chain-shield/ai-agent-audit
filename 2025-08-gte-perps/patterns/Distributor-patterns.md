## Verified Patterns Found: 21

## Verified Patterns Found in following Categories:

- ERC20DecimalsMismatch
- UnsafeAssembyTypeCasts
- PrecisionDriftAccumulation
- FeeOnTransferAssumption
- StandardViolation
- PricePrecisionOrRoundingError
- GriefableCallbacks
- FlashLoanEconomicManipulation
- AccessControlOrAuthByPass
- ReserveOrPriceDesync
- AccountingInvariantViolation



## Summary of Patterns

Missing Asset Validation in addRewards Enables Reward Theft

Precision loss in reward accumulator leads to permanent loss of rewards

Precision Loss leading to Stuck Funds

User rewards misdirected to Launchpad contract instead of user

AMM Pair Denial-of-Service via Distributor Revert

Potential Denial of Service on Pair via Rewards Callback

Denial of Service in Uniswap V2 Pair via Distributor Integration

Rewards Loss due to Insufficient Precision Factor

Permanent locking of dust rewards due to accounting precision loss

Flash Loan / Sandwich Attack on Instant Reward Distribution

Token supply cap causes DoS or reward loss via implicit type casting

Precision Loss in RewardsTracker causes massive reward burning for standard decimals

High precision loss in rewards accumulator due to low PRECISION_FACTOR

Accumulated Precision Loss Locks Funds Permanently

Launchpad fee bypass via flash loan liquidity inflation

Permanent Lock of Reward Dust due to Precision Drift

Low Precision Factor Causes Reward Loss for High-Supply Tokens

DoS of Pair swaps when Distributor shares are zero

Fee-on-transfer tokens break reward accounting in addRewards

Missing Input Validation in addRewards Enables Reward Dilution and DoS

Arbitrary Token Injection via `addRewards` Corrupts Reward Accounting

## Patterns



 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Missing Asset Validation in addRewards Enables Reward Theft
 ### Description/Code Snippet
The `addRewards` function identifies the `launchAsset` pool but fails to verify that the second token argument (`quoteAsset`) matches the pool's configured `rs.quoteAsset`. An attacker can call `addRewards(ValidLaunchToken, FakeToken, 0, amount)` to inflate `rs.pendingQuoteRewards` with a worthless token. This increases the global `accQuoteRewardPerShare` for the valid pool. Users (including the attacker) can then claim legitimate quote tokens (e.g., USDC) from the contract based on this inflated accumulator, draining the distributor's balance of real assets.
 ### Static Signals
assumes invariant without verifying, input token not checked against storage
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: RewardsTracker.sol.update

 ### Title
Precision loss in reward accumulator leads to permanent loss of rewards
 ### Description/Code Snippet
The `RewardsTrackerLib.update` function calculates accumulated rewards per share using a precision factor of `1e12`: `(pending * 1e12) / totalShares`. For tokens with 18 decimals, `totalShares` (tracking token supply) is often much larger than `1e12`. If `pending` rewards are small (e.g., from frequent small fee accruals), `pending * 1e12` may be less than `totalShares`, causing the division to yield 0. The `pending` amount is deleted from storage but not added to the accumulator, resulting in a 100% loss of those rewards. An attacker can grief the protocol by repeatedly calling `claim()` to flush pending rewards while they are still in the dust range.
 ### Static Signals
division by totalShares, precision factor 1e12, integer truncation
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: RewardsTrackerLib.getAccRewardsPerShare

 ### Title
Precision Loss leading to Stuck Funds
 ### Description/Code Snippet
In `RewardsTrackerLib.getAccRewardsPerShare`, the calculation `(pending * PRECISION_FACTOR) / totalShares` floors to zero if `pending * 1e12 < totalShares`. However, `Distributor` tracks the full `amount` in `totalPendingRewards`. Repeated small deposits via `addRewards` will increment `totalPendingRewards` without increasing the `accRewardsPerShare` used for distribution. This causes `totalPendingRewards` to permanently exceed the actual distributable claims, preventing the `skimExcessRewards` function (which relies on `balance - totalPendingRewards`) from working and locking the dust amounts forever.
 ### Static Signals
division before addition, floor rounding in accumulator
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor._distributeAssets

 ### Title
User rewards misdirected to Launchpad contract instead of user
 ### Description/Code Snippet
The `increaseStake` and `decreaseStake` functions in `Distributor` are restricted to `onlyLaunchpad`. These functions trigger reward distribution via `_distributeAssets`, which transfers pending rewards to `msg.sender`. Since `msg.sender` is the Launchpad contract, any rewards accrued by a user (e.g., from a previous buy or added incentives) are transferred to the Launchpad contract instead of the user's wallet when they buy more or sell. Unless the Launchpad has a mechanism to recover these funds, they are permanently lost to the user.
 ### Static Signals
msg.sender used as recipient in restricted function, onlyLaunchpad modifier, safeTransfer(msg.sender)
 ### Assets at Risk
Reward tokens (Base and Quote assets)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
AMM Pair Denial-of-Service via Distributor Revert
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` contract calls `Distributor.addRewards` within its critical `_update` function (executed on every swap, mint, and burn). If `Distributor.addRewards` reverts, the entire AMM pair becomes unusable.

One specific trigger is the `NoSharesToIncentivize` revert in `Distributor.addRewards`, which occurs if `rs.totalShares == 0`. If the Launchpad logic allows all stakes to be removed (e.g., post-graduation) without simultaneously disabling the Pair's fee accrual via `endRewards`, any subsequent interaction with the Pair will revert. Additionally, if the reward token (e.g., USDC) blacklists the Distributor contract, the `safeTransferFrom` in `addRewards` will revert, bricking the AMM pair.
 ### Static Signals
external call in hot path, no try/catch, revert on external condition
 ### Assets at Risk
liquidity pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Potential Denial of Service on Pair via Rewards Callback
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` calls `Distributor.addRewards` during every swap if `rewardsPoolActive > 0`. The `Distributor.addRewards` function strictly reverts with `NoSharesToIncentivize` if `rs.totalShares == 0`. If a scenario arises where the pair is active but all users have unstaked (or no one has staked yet, and `totalShares` is 0), any attempt to swap on the pair will revert, effectively causing a DoS on the liquidity pool.
 ### Static Signals
external call in loop/hook, revert condition in callback
 ### Assets at Risk
liquidity pool
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Denial of Service in Uniswap V2 Pair via Distributor Integration
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` contract calls `Distributor.addRewards` in its `_update` function whenever fees are accrued. `Distributor.addRewards` contains a strict check: `if (rs.totalShares == 0) revert NoSharesToIncentivize();`. If the Distributor's total shares drop to zero (e.g., because all users have unstaked or sold their launch tokens), `addRewards` will revert. Consequently, the `_update` function in the Pair will revert, causing all Swaps, Mints, and Burns on the main trading pair to fail, effectively freezing the liquidity pool.
 ### Static Signals
revert NoSharesToIncentivize(), external call in _update
 ### Assets at Risk
Liquidity Pool Functionality
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: Distributor.RewardsTrackerLib.update

 ### Title
Rewards Loss due to Insufficient Precision Factor
 ### Description/Code Snippet
The `PRECISION_FACTOR` in `RewardsTrackerLib` is set to 1e12. When distributing rewards where the staking token has 18 decimals and a large total supply (e.g., > 1M tokens / 1e24 wei) and the reward token has low decimals (e.g., USDC with 6 decimals), the accumulator calculation `(amount * 1e12) / totalShares` frequently truncates to zero. For example, with 1M staked tokens, a reward injection of 1,000 USDC (1e9 wei) results in `1e9 * 1e12 / 1e24 = 0`, causing the entire reward amount to be lost permanently.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.skimExcessRewards

 ### Title
Permanent locking of dust rewards due to accounting precision loss
 ### Description/Code Snippet
In `RewardsTrackerLib.update()`, the function `getAccRewardsPerShare` calculates `accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares))`. If `pendingBaseRewards * 1e12` is less than `totalShares`, the increment is zero due to integer division. However, `update()` subsequently deletes `self.pendingBaseRewards` regardless of whether the accumulator increased. The `Distributor` contract tracks the full amount in `totalPendingRewards`. This creates a discrepancy where `totalPendingRewards` includes amounts that were effectively burned from the internal accounting. Since `skimExcessRewards` relies on `asset.balanceOf(this) - totalPendingRewards`, these dust amounts cannot be skimmed by the admin, nor can they be claimed by users, leading to permanently stuck funds.
 ### Static Signals
state variable tracks value cleared in library, precision loss in division
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Flash Loan / Sandwich Attack on Instant Reward Distribution
 ### Description/Code Snippet
The `addRewards` function distributes rewards immediately to current stakers without any vesting or streaming duration. An attacker can front-run an `addRewards` transaction (or their own reward addition) by flash-buying a large amount of shares via the Launchpad's bonding curve (calling `increaseStake`), capturing a large portion of the rewards, and then selling/claiming in the same transaction.
 ### Static Signals
rewards added and distributed in same block, no vesting/streaming period, shares manipulable via flash loan
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: Distributor.increaseStake

 ### Title
Token supply cap causes DoS or reward loss via implicit type casting
 ### Description/Code Snippet
The `RewardsTrackerLib` and `Distributor` enforce a `uint96` type for `shares` and `totalShares`. If a launched token has a total supply exceeding `type(uint96).max` (approx 7.9e28, or ~79 billion with 18 decimals), this creates a critical failure mode. For high-supply tokens (common in meme coins), `increaseStake` will either revert (if cast safely upstream) causing a DoS of the `Launchpad.buy` function, or silently truncate the stake (if cast unsafely) causing massive loss of user rewards. Given the permissionless nature of the launcher, this limit is easily breached.
 ### Static Signals
uint96(shares), uint96 totalShares
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: Distributor.update

 ### Title
Precision Loss in RewardsTracker causes massive reward burning for standard decimals
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a hardcoded `PRECISION_FACTOR` of `1e12` to calculate `accRewardPerShare`. The formula is `(pending * 1e12) / totalShares`. `totalShares` represents token amounts, typically with 18 decimals. If a project has a total supply of 100M tokens (1e26 units), and 1000 USDC (1e9 units, 6 decimals) are added as rewards: `1e9 * 1e12 = 1e21`. Dividing `1e21` by `1e26` yields 0. The `pending` rewards are deleted (cleared) in `update()`, but the accumulator is not incremented. This results in the complete loss of rewards for legitimate stakers whenever `(reward * 1e12) < totalShares`. For 18-decimal tokens, `1e12` precision is insufficient and leads to significant fund loss.
 ### Static Signals
PRECISION_FACTOR = 1e12, division before multiplication (implicit in ratio), delete pending without remainder check
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: RewardsTrackerLib.update

 ### Title
High precision loss in rewards accumulator due to low PRECISION_FACTOR
 ### Description/Code Snippet
The `RewardsTrackerLib` sets `PRECISION_FACTOR` to `1e12`. When `update()` calculates `accBaseRewardPerShare`, it uses the formula `(pending * 1e12) / totalShares`. If `totalShares` (often 18 decimals) exceeds `pending * 1e12`, the result truncates to zero. For example, if 1 token (1e18 shares) is staked, any reward amount less than 1e6 (1 USDC) is truncated to 0, removed from pending, and permanently locked in the contract (as `totalPendingRewards` still tracks it, preventing skimming).
 ### Static Signals
mix 6/8/18 decimals without normalization, divide before multiply
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: Distributor.skimExcessRewards

 ### Title
Accumulated Precision Loss Locks Funds Permanently
 ### Description/Code Snippet
Due to the precision issue in `RewardsTrackerLib`, rewards that round to zero are removed from `pendingBaseRewards` but not added to `accBaseRewardPerShare`. These tokens remain in the `Distributor` contract's balance. However, `skimExcessRewards` calculates skimmable amounts as `balance - totalPendingRewards`. Since `totalPendingRewards` was incremented when rewards were added (via `_increaseTotalPending`), the contract accounting believes these funds are pending distribution. This results in the funds being permanently locked: they cannot be claimed by users (due to zero accrual) and cannot be skimmed by the admin (due to accounting checks).
 ### Static Signals
balance check vs accounting var, rounding down to zero
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: GTELaunchpadV2Pair._getLaunchpadFees

 ### Title
Launchpad fee bypass via flash loan liquidity inflation
 ### Description/Code Snippet
In `GTELaunchpadV2Pair._getLaunchpadFees`, the fee collected for the Distributor is calculated as a fraction of the Launchpad's share of total liquidity (`totalSupply()`). An attacker can flash-mint a massive amount of LP tokens to inflate `totalSupply` before a swap, diluting the fee to near zero, then burn the LP tokens, effectively bypassing the protocol fee.
 ### Static Signals
fee calculation depends on totalSupply(), uses spot balance in swap path, no time-weighted average
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Permanent Lock of Reward Dust due to Precision Drift
 ### Description/Code Snippet
In `RewardsTrackerLib.update`, the accumulated reward per share is calculated as `acc += (amount * 1e12) / totalShares`. Any remainder from this division (dust) is lost from the distributable amount tracked by `accBaseRewardPerShare`/`accQuoteRewardPerShare`.

However, `Distributor.addRewards` increments `totalPendingRewards` by the full `amount`. Users can only claim the truncated amount derived from the accumulator. As a result, `totalPendingRewards` remains permanently higher than the sum of all possible claims by the amount of the dust.

Since `skimExcessRewards` enforces `amount <= balance - totalPendingRewards`, this dust portion is considered 'owed' to users by the accounting system but is mathematically impossible to claim, causing it to be locked in the contract forever.
 ### Static Signals
acc += amount / totalShares, balance - totalPendingRewards check, no dust rollup mechanism
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: RewardsTrackerLib.getAccRewardsPerShare

 ### Title
Low Precision Factor Causes Reward Loss for High-Supply Tokens
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a fixed `PRECISION_FACTOR` of `1e12` for calculating `accRewardsPerShare`. For tokens with high total supply (e.g., 18 decimals, 1M+ tokens = 1e24 units), the division `(pendingRewards * 1e12) / totalShares` will round down to zero for significant reward amounts (e.g., 1 USDC = 1e6 units -> 1e18 numerator < 1e24 denominator). These rewards are added to `totalPendingRewards` but never accrue to user shares, effectively locking them in the contract forever as they cannot be claimed nor skimmed (since `skimExcessRewards` respects `totalPendingRewards`).
 ### Static Signals
mix 6/8/18 decimals without normalization, divide before multiply, precision factor too low
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._distributeLaunchpadFees

 ### Title
DoS of Pair swaps when Distributor shares are zero
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` calls `Distributor.addRewards` during every swap/mint/burn via `_update`. `Distributor.addRewards` reverts if `totalShares == 0`. If the staking pool becomes empty (e.g. all users transfer out or unstake), the Uniswap Pair becomes unusable as all operations will revert.
 ### Static Signals
external call in hot path, revert in external call blocks execution, addRewards reverts on zero shares
 ### Assets at Risk
liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Fee-on-transfer tokens break reward accounting in addRewards
 ### Description/Code Snippet
The `addRewards` function updates `totalPendingRewards` and the pool's `pendingBaseRewards` using the input `amount` parameter, but subsequently calls `safeTransferFrom` without verifying the actual amount received. If a fee-on-transfer or deflationary token is used as the reward asset, the contract will track more rewards than it actually holds. This insolvency ensures that the last users to attempt `claimRewards` will fail due to insufficient contract balance, effectively locking their funds.
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check, accounting based on transfer parameter, not actual balance change
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Missing Input Validation in addRewards Enables Reward Dilution and DoS
 ### Description/Code Snippet
The `addRewards` function accepts `token0` and `token1` and determines the target pool by checking if either is a registered launch asset. However, it fails to verify that the *other* token provided matches the pool's configured `quoteAsset` stored in `rs.quoteAsset`. An attacker can call `addRewards(RealLaunchAsset, MaliciousToken, ...)` which the contract accepts, incorrectly treating `MaliciousToken` as the pool's quote asset. This increments `rs.pendingQuoteRewards` and transfers `MaliciousToken` to the contract. Subsequent calls to `update()` or `claimRewards()` incorporate this inflated pending amount into `accQuoteRewardPerShare`. Since `claimRewards` pays out the *configured* `rs.quoteAsset` (not the malicious one), the inflated accumulator causes the contract to attempt transferring more valid quote tokens than it holds (or than are owed), draining the pool's quote asset reserves or causing `claimRewards` to revert (DoS) due to arithmetic underflow in `_decreaseTotalPending`.
 ### Static Signals
no require(token1 == rs.quoteAsset), rs.addQuoteRewards(..., quoteAsset, ...), quoteAsset variable derived from input parameters without validation against storage
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Arbitrary Token Injection via `addRewards` Corrupts Reward Accounting
 ### Description/Code Snippet
The `addRewards` function allows permissionless reward addition by specifying two tokens (`token0`, `token1`). The contract identifies the reward pool associated with one of the tokens (`LaunchAsset`) but fails to verify that the second token matches the pool's immutable `quoteAsset`. An attacker can call `addRewards` with a valid `LaunchAsset` and a worthless `MaliciousToken`. The contract accepts the `MaliciousToken` via transfer, but increments the pool's `pendingQuoteRewards` accumulator. Since `pendingQuoteRewards` drives the distribution of the legitimate `quoteAsset` (e.g., USDC) in `_distributeAssets`, users claiming rewards will be paid out in USDC based on the inflated value from the malicious token deposit, draining the legitimate USDC rewards from the contract.
 ### Static Signals
token/asset address changes without accounting migration, balance tracking references different token than actual holdings, no require(token1 == rs.quoteAsset)
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

