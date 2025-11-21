## Verified Patterns Found: 16

## Verified Patterns Found in following Categories:

- PricePrecisionOrRoundingError
- AccountingInvariantViolation
- PermitFrontRun
- StandardViolation
- FeeOnTransferAssumption
- ForcedAssetVsStrictEquality
- AccessControlOrAuthByPass
- UnsafeRecipient
- PrecisionDriftAccumulation
- ERC20DecimalsMismatch
- EpochOrIndexMonotonicity



## Summary of Patterns

Reward calculation precision loss due to insufficient scaling factor

Insolvent reward accounting due to Fee-On-Transfer token support

Unauthorized token injection in addRewards corrupts reward accounting

Malleable Signatures in Uniswap V2 Permit Implementation

Permanent Reward Loss due to Precision Truncation in RewardsTrackerLib

Incompatibility with High-Supply Tokens due to `uint96` Casting

Fee-on-Transfer Tokens Break Reward Skimming and Distribution

Systematic Reward Loss due to Insufficient Precision Factor

Unchecked Quote Asset in `addRewards` allows Reward Pool DoS

Precision loss in reward accrual for high-supply assets

Precision loss in rewards accumulator locks funds

Coupled reward claiming causes stuck funds if one asset is non-receivable

Reward rounding dust is permanently locked

Insolvency via Fee-On-Transfer tokens in addRewards

DoS of Reward Pool via Mismatched Quote Asset in addRewards

AMM Denial of Service when Distributor has Zero Shares

## Patterns



 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: RewardsTrackerLib.getAccRewardsPerShare

 ### Title
Reward calculation precision loss due to insufficient scaling factor
 ### Description/Code Snippet
In `RewardsTrackerLib.sol`, `PRECISION_FACTOR` is set to `1e12`. The `accBaseRewardsPerShare` calculation uses this factor: `((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares))`. If the staking token (LaunchToken) has 18 decimals and the reward token has 6 decimals (e.g., USDC), or even if both have 18 decimals but the total staked supply is large, the division will truncate to zero. For example, with 1 million staked tokens (1e24 wei) and 100 USDC rewards (1e8 wei), `1e8 * 1e12 = 1e20`, which is less than `1e24`, resulting in 0 accrued rewards per share. This systematically prevents reward distribution.
 ### Static Signals
PRECISION_FACTOR = 1e12, division before multiplication (implicit in ratio size), mixes token amounts with unscaled math
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.sol.addRewards

 ### Title
Insolvent reward accounting due to Fee-On-Transfer token support
 ### Description/Code Snippet
The `addRewards` function updates the `totalPendingRewards` and pool `pending` balances using the `amount` parameter passed by the caller. It then calls `safeTransferFrom` to pull the tokens. If the asset is a Fee-On-Transfer token, the contract receives less than `amount`. The internal accounting (`totalPendingRewards`) will exceed the actual contract balance. This insolvency will cause `claimRewards` to revert for the last claimers (due to insufficient balance) and breaks `skimExcessRewards`.
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: Distributor.sol.addRewards

 ### Title
Unauthorized token injection in addRewards corrupts reward accounting
 ### Description/Code Snippet
The `addRewards` function accepts `token0` and `token1` as arguments and identifies the reward pool based on `token0` (or `token1` if swapped). However, it fails to verify that the second token argument matches the pool's actual `quoteAsset`. An attacker can pass a malicious token as `token1` alongside a legitimate `token0`. The contract will transfer the malicious token but credit the internal `pendingQuoteRewards` balance (which corresponds to the legitimate `quoteAsset`, e.g., USDC). This corrupts the reward state, diluting the legitimate rewards or enabling DoS by creating unbacked pending rewards.
 ### Static Signals
missing check: quoteAsset == rs.quoteAsset, public function updating sensitive accounting
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitFrontRun

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
Malleable Signatures in Uniswap V2 Permit Implementation
 ### Description/Code Snippet
The `UniswapV2ERC20` contract implements `permit` using `ecrecover` but fails to validate that the signature's `s` value is in the lower half of the secp256k1 curve order (`s <= 0x7FFFF...`). This allows an attacker to take a valid signature, modify the `s` value (malleability), and front-run the user's transaction. This consumes the nonce and causes the original transaction to revert, potentially disrupting intended transaction flows or batched operations.
 ### Static Signals
no s-value malleability guard, ecrecover usage without checks
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.update

 ### Title
Permanent Reward Loss due to Precision Truncation in RewardsTrackerLib
 ### Description/Code Snippet
The `RewardsTrackerLib` calculates reward accumulation using `(pending * 1e12) / totalShares`. Since `totalShares` can be very large (up to ~7.9e28 for `uint96`), and the precision factor is only `1e12`, relatively small but non-trivial reward amounts (e.g., < 1e6 ETH equivalent if shares are high) will result in a quotient of 0.

The `update` function then unconditionally deletes `self.pendingBaseRewards` and `self.pendingQuoteRewards` after this calculation. 

```solidity
(newAccBase, newAccQuote) = getAccRewardsPerShare(self);
if (self.pendingBaseRewards > 0) {
    self.accBaseRewardPerShare = newAccBase;
    delete self.pendingBaseRewards; // Deleted even if accumulation was 0
}
```

This causes the rewards to be deleted from the internal tracker without being distributed to users. However, `totalPendingRewards` in the `Distributor` remains incremented. These tokens become permanently stuck in the contract—users cannot claim them (debts don't increase) and the admin cannot `skim` them (as `skim` respects `totalPendingRewards`).
 ### Static Signals
integer division pending * PRECISION / totalShares, delete pendingRewards after calculation, low precision factor 1e12 relative to potential supply
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Distributor.increaseStake

 ### Title
Incompatibility with High-Supply Tokens due to `uint96` Casting
 ### Description/Code Snippet
The `Distributor` and `RewardsTrackerLib` use `uint96` to store user shares (`UserRewardData.shares`) and accept `uint96` in `increaseStake`. Many ERC20 tokens (especially meme tokens targeted by launchpads) have supplies significantly larger than `type(uint96).max` (~7.9e28). For example, a token with 100B supply and 18 decimals is 1e29. Launching such a token will cause `increaseStake` to revert due to overflow or SafeCast failure, rendering the Launchpad unusable for high-supply assets.
 ### Static Signals
uint96(shares), SafeCastLib
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Fee-on-Transfer Tokens Break Reward Skimming and Distribution
 ### Description/Code Snippet
The `Distributor` contract's `addRewards` function updates `totalPendingRewards` by the input `amount`, but the `safeTransferFrom` call results in a balance increase of `amount - fee` if the token has transfer fees. This breaks the internal accounting invariant `balance >= totalPendingRewards`. Consequently, `skimExcessRewards` (which calculates `balance - totalPendingRewards`) will revert due to underflow, preventing admins from recovering any excess funds. Furthermore, `_distributeAssets` will fail for the last claimers due to insufficient contract balance.
 ### Static Signals
strict equality or subtraction assuming exact balance, accounting state not updated on transfer result
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: RewardsTrackerLib.update

 ### Title
Systematic Reward Loss due to Insufficient Precision Factor
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a hardcoded `PRECISION_FACTOR` of `1e12`. When updating the accumulator in `update()`, the calculation is `(pending * 1e12) / totalShares`. If the staked Launch Asset uses 18 decimals and the reward token (e.g., USDC) uses 6 decimals, a `totalShares` amount of just ~100 tokens (1e20) is enough to cause `pending * 1e12` (1 USDC = 1e18) to be smaller than `totalShares`. The integer division truncates to zero, causing 100% loss of rewards for stakers. These rewards are deleted from `pending` but never added to `accRewardPerShare`, effectively locking them in the contract.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled, divide before multiply
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Unchecked Quote Asset in `addRewards` allows Reward Pool DoS
 ### Description/Code Snippet
The `addRewards` function allows any user to add rewards to an existing pool. It correctly identifies the pool via `token0` or `token1`, but fails to verify that the provided `quoteAsset` argument matches the pool's initialized `quoteAsset`. An attacker can call `addRewards(RealBase, FakeQuote, 0, amount)`. This increments `rs.pendingQuoteRewards` and `totalPendingRewards[FakeQuote]`, but leaves `totalPendingRewards[RealQuote]` unchanged. When legitimate users try to claim rewards, the contract calculates their share of `pendingQuoteRewards` (which is now inflated) and attempts to transfer `rs.quoteAsset` (RealQuote). The transfer logic in `_decreaseTotalPending` checks `totalPendingRewards[RealQuote]`, which is insufficient to cover the claim, causing a revert. This permanently DoSes claiming for all users of that pool.
 ### Static Signals
missing check quoteAsset == rs.quoteAsset, public function updating critical state
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: EpochOrIndexMonotonicity

 ### Relevant Function/Location: RewardsTracker.sol.getAccRewardsPerShare

 ### Title
Precision loss in reward accrual for high-supply assets
 ### Description/Code Snippet
The `RewardsTrackerLib.getAccRewardsPerShare` function calculates reward accrual using a `PRECISION_FACTOR` of `1e12`. If the `totalShares` (e.g., 18 decimals) is significantly larger than `pendingRewards` (e.g., 6 decimals like USDC) scaled by `1e12`, the division `(pending * 1e12) / totalShares` will truncate to zero. This effectively burns rewards for users in high-supply pools with low-decimal reward tokens, breaking the accrual mechanism.
 ### Static Signals
index set from smaller value, breaking accrual math
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: Distributor.getRewardsPoolData

 ### Title
Precision loss in rewards accumulator locks funds
 ### Description/Code Snippet
The `RewardsTrackerLib` calculates `accBaseRewardPerShare` using a precision factor of `1e12` (`(pending * 1e12) / totalShares`). If `totalShares` is large (e.g., high-supply launch tokens) or the reward token has low decimals (e.g., USDC), the division often truncates to zero for small to medium reward amounts. The `update` function clears `pendingBaseRewards` regardless of the truncation, effectively destroying the rewards in the accumulator logic. However, `Distributor`'s `totalPendingRewards` still accounts for these amounts. This desynchronization locks the funds in the contract: users cannot claim them (accumulator didn't increase) and the admin cannot skim them (accounting says they are owed).
 ### Static Signals
divide before multiply, mix 6/8/18 decimals without normalization
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Distributor.claimRewards

 ### Title
Coupled reward claiming causes stuck funds if one asset is non-receivable
 ### Description/Code Snippet
The `claimRewards` function triggers distributions for both the Base Asset and Quote Asset in a single transaction via `_distributeAssets`. If either token transfer fails (e.g., a user is blocklisted by USDC, or the token reverts on transfer to the specific receiver), the entire transaction reverts. This creates a vulnerability where a user cannot claim their Base Asset rewards because of an issue with the Quote Asset, effectively freezing their funds.
 ### Static Signals
transfers to non-receivable addresses cause value loss or stuck funds, coupled transfers in single function
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: Distributor.skimExcessRewards

 ### Title
Reward rounding dust is permanently locked
 ### Description/Code Snippet
In `RewardsTrackerLib`, the `accRewardPerShare` calculation truncates remainders, but `Distributor.addRewards` tracks the full `amount` in `totalPendingRewards`. As a result, `totalPendingRewards` will always be slightly higher than the sum of all claimable user rewards. The `skimExcessRewards` function restricts withdrawals to `balance - totalPendingRewards`, meaning the dust (remainder) is permanently locked in the contract and cannot be skimmed by the admin.
 ### Static Signals
consistent floor toward sender/receiver, accumulated value locked by strict accounting
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Insolvency via Fee-On-Transfer tokens in addRewards
 ### Description/Code Snippet
The `addRewards` function updates the internal `totalPendingRewards` and `pendingBaseRewards` accounting with the full input `amount` *before* transferring tokens in via `safeTransferFrom`. If the token implements a fee-on-transfer mechanism, the `Distributor` contract receives less than the accounted `amount`. This discrepancies creates immediate insolvency where `totalPendingRewards > balanceOf(this)`, causing future calls to `claimRewards` (and potentially `increaseStake`) to revert due to insufficient funds, effectively locking the protocol for all users.
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check, accounting based on transfer parameter
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
DoS of Reward Pool via Mismatched Quote Asset in addRewards
 ### Description/Code Snippet
The `addRewards` function allows any user to add rewards to a pool. While it correctly identifies the pool storage `rs` based on `token0` or `token1`, it fails to validate that the *other* token passed by the user matches the pool's configured `quoteAsset`. 

If a pool is initialized with `(LaunchToken, RealQuote)`, an attacker can call `addRewards(LaunchToken, JunkToken, 0, amount)`. The function will:
1. Load the pool for `LaunchToken`.
2. Call `rs.addQuoteRewards(..., amount)` which increases `rs.pendingQuoteRewards` (internal accounting) by `amount`.
3. Call `_increaseTotalPending(JunkToken, amount)` which increases the contract's `totalPendingRewards` for `JunkToken`.

Crucially, `totalPendingRewards[RealQuote]` is **not** increased. However, the internal `rs.pendingQuoteRewards` is inflated. When legitimate users subsequently call `claimRewards`, the contract calculates their share of the 'quote' rewards (which now includes the junk amount). `_distributeAssets` then attempts to transfer `RealQuote` tokens and decrease `totalPendingRewards[RealQuote]` by this inflated amount.

Since `totalPendingRewards[RealQuote]` was never incremented for the junk addition, `_decreaseTotalPending` will revert due to underflow protection (`currTotal < amount`), causing a permanent Denial of Service for all claims in that pool.
 ### Static Signals
user input token1 not checked against rs.quoteAsset, internal accounting (pendingQuoteRewards) updated independently of global accounting (totalPendingRewards)
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
AMM Denial of Service when Distributor has Zero Shares
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` contract calls `Distributor.addRewards` on every reserve update (mint, burn, swap). The `Distributor.addRewards` function explicitly reverts with `NoSharesToIncentivize` if `rs.totalShares == 0` (i.e., if there are no stakers). If all users unstake their Launch Tokens from the Distributor—a valid user action—the AMM pair becomes completely unusable because `_update` will always revert, violating the invariant that the AMM should function independently of the staking contract's participation levels.
 ### Static Signals
revert on zero divisor or zero value, external call in critical path
 ### Assets at Risk
liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

