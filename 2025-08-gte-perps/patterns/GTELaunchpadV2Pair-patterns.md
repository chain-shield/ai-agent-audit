## Verified Patterns Found: 36

## Verified Patterns Found in following Categories:

- PermitFrontRun
- FeeOnTransferAssumption
- UnsafeAssembyTypeCasts
- UnsafeRecipient
- GriefableCallbacks
- AccessControlOrAuthByPass
- StandardViolation
- FlashLoanEconomicManipulation
- FeeAccountingDrift
- UnprotectedPauseOrStop
- InitOrderOrUnintialized
- AccountingInvariantViolation
- NonStandardERC20Behavior
- PermitMisuse
- ERC20DecimalsMismatch
- PrecisionDriftAccumulation
- PricePrecisionOrRoundingError



## Summary of Patterns

Premature Permanent Disabling of Fees via endRewards

DoS of Claiming due to Unsafe Downcast

Premature disabling of Pair fees via endRewards breaks protocol incentives

Rewards lost due to insufficient precision factor (1e12)

Sandwich attack on permissionless reward distribution during bonding phase

Signature Malleability in Permit

Premature Permanent Disabling of Reward Fees

Rewards precision loss due to insufficient scaling factor

Rewards permanently locked due to precision loss in tracking

uint96 Share Cap Causes DoS for High Supply Tokens

Stuck Dust Rewards Prevent Skimming via Accounting Invariant Violation

Proxy Initialization Vulnerability

Insufficient Precision in RewardsTracker Causes Reward Loss

Systematic loss of rewards due to insufficient precision factor

Overflow in UserRewardData due to uint96 limits for high-supply tokens

Incompatible precision factor causes total reward loss for USDC pairs

AMM Swap DoS when Distributor has zero shares

Permit signature malleability enables front-running DoS

Unclaimable dust accumulation in totalPendingRewards

Precision Loss in Reward Distribution for High-Supply Tokens

Uninitialized Reward Pool DoS

Reward Loss due to Low Precision Factor

Precision Loss in RewardsTracker Erases Rewards for Large Pools

Launchpad fee rounding leads to revenue loss

Unclaimable dust rewards cause accounting drift and lock funds

Malleable signatures accepted in UniswapV2ERC20 permit

Low precision factor in RewardsTracker leads to significant yield loss

Claiming blocked by coupled asset transfer failure

Flash loan manipulation of LP totalSupply bypasses Launchpad fee

AMM Pair DoS via Reverting Fee Distribution Hook

Insolvency with Fee-On-Transfer tokens in `addRewards`

DoS of AMM Swaps when Distributor Shares are Zero

Rewards Misdirected to Launchpad Contract instead of User

Fee-On-Transfer Token Support Missing in addRewards

addRewards allows mismatched quote tokens enabling reward theft

Missing asset validation in addRewards allows draining legitimate reward tokens

## Patterns



 ### Issue Type: UnprotectedPauseOrStop

 ### Relevant Function/Location: GTELaunchpadV2Pair.endRewardsAccrual

 ### Title
Premature Permanent Disabling of Fees via endRewards
 ### Description/Code Snippet
The `Distributor.endRewards` function calls `pair.endRewardsAccrual()`, which permanently deletes `rewardsPoolActive` (setting it to 0). According to the documentation, `endRewards` is called at graduation (when the pair is created). This sequence immediately stops the pair from ever accruing Launchpad fees, breaking the intended economic flow where the pair fees feed the Distributor.
 ### Static Signals
delete rewardsPoolActive, rewardsPoolActive check in _update
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: Distributor.stake/unstake/claim

 ### Title
DoS of Claiming due to Unsafe Downcast
 ### Description/Code Snippet
In `RewardsTrackerLib`, the `baseRewardDebt` is updated by casting `totalAccRewards` (uint256) to `uint96`. For high-supply tokens or long-running pools, `accRewardPerShare` (scaled by 1e12) multiplied by shares can exceed `type(uint96).max` (approx 7.9e28). This downcast truncates the debt value. During `claim`, the full `totalAccRewards` is compared to the truncated debt, resulting in an erroneously huge claim amount that causes `_decreaseTotalPending` to revert due to underflow/insufficient balance, permanently locking user funds.
 ### Static Signals
uint96(totalAccRewards(...)), explicit downcast without SafeCast, uint96 used for cumulative debt tracking
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.endRewardsAccrual

 ### Title
Premature disabling of Pair fees via endRewards breaks protocol incentives
 ### Description/Code Snippet
The protocol documentation states that `GTELaunchpadV2Pair` fees feed the Launchpad Distributor. However, the `endRewards` function (called at graduation) invokes `pair.endRewardsAccrual()`, which sets `rewardsPoolActive` to 0. In `GTELaunchpadV2Pair`, fee collection is conditional on `rewardsPoolActive > 0`. Consequently, as soon as the token graduates and the pair is fully established, the fee mechanism is permanently disabled, violating the intended economic design.
 ### Static Signals
delete rewardsPoolActive, conditional fee logic depends on rewardsPoolActive
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: RewardsTrackerLib.getAccRewardsPerShare

 ### Title
Rewards lost due to insufficient precision factor (1e12)
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a `PRECISION_FACTOR` of `1e12`. In `getAccRewardsPerShare`, the calculation is `(pending * 1e12) / totalShares`. If `totalShares` is large (e.g., 80% of a 1B supply token = 8e26 wei) and the added reward amount is small (e.g., 1 USDC = 1e6 wei), the numerator `1e6 * 1e12 = 1e18` is smaller than the denominator `8e26`, resulting in 0. The `pending` rewards are deleted (cleared) in `update()`, but `accRewardsPerShare` does not increase, effectively burning the rewards.
 ### Static Signals
mix 6/8/18 decimals without normalization, precision factor 1e12 too low for high supply
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Sandwich attack on permissionless reward distribution during bonding phase
 ### Description/Code Snippet
Rewards added via `addRewards` are distributed instantaneously to current shareholders (updating `accRewardPerShare` immediately). During the bonding phase, shares are acquired by buying the `LaunchToken` on the bonding curve. An attacker can front-run a large `addRewards` transaction (e.g., from the Pair flushing fees or a project incentive) by flash-buying shares on the curve, claiming a large portion of the rewards, and back-running by selling the shares. This extracts value from the reward provider to the attacker.
 ### Static Signals
Instant reward distribution in RewardsTrackerLib, Permissionless addRewards, Liquid shares via bonding curve
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
Signature Malleability in Permit
 ### Description/Code Snippet
The `permit` function in `UniswapV2ERC20` uses `ecrecover` without validating that the `s` value of the signature is in the lower half of the curve (`s <= secp256k1n/2`). While nonces prevent replay of the exact message, this allows malleable signatures to be accepted, violating EIP-2612 strict compliance and potentially causing issues with transaction malleability checks in integrations.
 ### Static Signals
ecrecover usage without s-value check, missing require(uint256(s) <= 0x7FFFF...)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.endRewardsAccrual

 ### Title
Premature Permanent Disabling of Reward Fees
 ### Description/Code Snippet
The documentation states that `Distributor.endRewards` is called at 'Graduation', yet `endRewards` calls `pair.endRewardsAccrual()`, which permanently sets `rewardsPoolActive = 0` and deletes accrued fees. This action effectively disables the logic that feeds fees to the Distributor immediately upon the pair's official launch, contradicting the protocol's stated mechanic that LP fees should incentivize stakers.
 ### Static Signals
delete rewardsPoolActive, delete accruedLaunchpadFee0
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Rewards precision loss due to insufficient scaling factor
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a fixed precision factor of `1e12`. When distributing low-decimal rewards (e.g., USDC with 6 decimals) to holders of high-decimal shares (e.g., LaunchToken with 18 decimals), the accumulator calculation `(amount * 1e12) / totalShares` suffers from severe rounding. If `totalShares` (raw units) exceeds `amount * 1e12`, the result is zero. For example, distributing $100,000 USDC to 1,000,000 staked tokens (1e24 raw shares) results in `(100,000e6 * 1e12) / 1e24 = 1e23 / 1e24 = 0`. The rewards are permanently stuck in the contract and users receive nothing.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled, PRECISION_FACTOR = 1e12
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: RewardsTrackerLib.getAccRewardsPerShare

 ### Title
Rewards permanently locked due to precision loss in tracking
 ### Description/Code Snippet
RewardsTrackerLib uses a fixed PRECISION_FACTOR of 1e12. If the reward token has low decimals (e.g., USDC with 6) and total shares are high (18 decimals), the calculation `(pending * 1e12) / totalShares` rounds to zero. The pending rewards are cleared (deleted) but the accumulator is not incremented, causing funds to be permanently locked.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled, precision factor 1e12 used with division
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.increaseStake

 ### Title
uint96 Share Cap Causes DoS for High Supply Tokens
 ### Description/Code Snippet
The `Distributor` and `RewardsTracker` structs limit `shares` and `totalShares` to `uint96` (max ~7.9e28). Many tokens (especially meme coins) have supplies exceeding this (e.g., 1 trillion tokens with 18 decimals = 1e30). If such a token is launched, the `increaseStake` function will revert when shares exceed `uint96` capacity, causing a Denial of Service for the Launchpad bonding/staking flow.
 ### Static Signals
uint96 cast of shares, uint96 type for totalShares
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.skimExcessRewards

 ### Title
Stuck Dust Rewards Prevent Skimming via Accounting Invariant Violation
 ### Description/Code Snippet
The `Distributor` contract tracks `totalPendingRewards` by adding amounts in `addRewards` and subtracting amounts in `claimRewards`. However, `claimRewards` calculates the payout using `RewardsTrackerLib` which uses floor rounding (integer division). As a result, the amount claimed and subtracted from `totalPendingRewards` is slightly less than the theoretical amount owed. The 'dust' difference remains in the contract's balance and `totalPendingRewards` count. The `skimExcessRewards` function strictly requires `balance > totalPendingRewards`. Since the dust is permanently counted in `totalPendingRewards` but is mathematically unclaimed by users, `totalPendingRewards` remains equal to or higher than the claimable balance, making it impossible to ever skim this residual value.
 ### Static Signals
balance - totalPendingRewards, unchecked subtraction
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: InitOrderOrUnintialized

 ### Relevant Function/Location: Distributor.initialize

 ### Title
Proxy Initialization Vulnerability
 ### Description/Code Snippet
The `Distributor` constructor initializes the owner using `_initializeOwner(msg.sender)`. If this contract is deployed via minimal proxy clones (standard for Launchpads to save gas), the constructor logic does not affect the proxy's storage. The `initialize` function has an `onlyOwner` modifier, but since the proxy's owner storage slot is 0 and the caller is non-zero, `onlyOwner` checks fail. This makes the proxy undeployable/unusable. The `initialize` function should call `_initializeOwner` if not initialized.
 ### Static Signals
_initializeOwner in constructor, initialize function protected by onlyOwner, missing owner setup in initialize
 ### Assets at Risk
availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: RewardsTrackerLib.getAccRewardsPerShare

 ### Title
Insufficient Precision in RewardsTracker Causes Reward Loss
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a `PRECISION_FACTOR` of `1e12` for reward accumulation. The formula `acc += (pending * 1e12) / totalShares` is prone to rounding to zero if `totalShares` is large relative to `pending * 1e12`. For a standard ERC20 token with 18 decimals, a supply of 1 million tokens results in `1e24` shares. If rewards are added in small increments (e.g., high-frequency fees from the Pair, or USDC with 6 decimals), any reward amount less than `1e12` (1 trillion units of reward token) relative to `totalShares` scale will result in zero accumulation. For example, 1 USDC (1e6) reward distributed to 1M share tokens (1e24) yields `1e6 * 1e12 / 1e24 = 0`. The pending rewards are deleted from state but never accrued to users, resulting in permanent loss of funds.
 ### Static Signals
PRECISION_FACTOR = 1e12, division by totalShares
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: Distributor.getAccRewardsPerShare (in RewardsTrackerLib)

 ### Title
Systematic loss of rewards due to insufficient precision factor
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a hardcoded `PRECISION_FACTOR` of `1e12` to scale rewards. When distributing rewards for tokens with low decimals (e.g., USDC with 6 decimals) against a staking token with high decimals and large supply (e.g., 800M tokens = 8e26 wei), the calculation `(pending * 1e12) / totalShares` rounds to zero for very significant amounts (e.g., up to 800,000 USDC). This results in the systematic loss of almost all quote asset rewards, which remain locked in the contract.
 ### Static Signals
PRECISION_FACTOR = 1e12, mixes token amounts with 18-decimal math unscaled
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: Distributor.increaseStake

 ### Title
Overflow in UserRewardData due to uint96 limits for high-supply tokens
 ### Description/Code Snippet
The `UserRewardData` struct in `RewardsTracker.sol` uses `uint96` for `shares`, `baseRewardDebt`, and `quoteRewardDebt`. `uint96` has a maximum value of approximately `7.9e28`. For tokens with 18 decimals, this corresponds to a supply of roughly 79 billion tokens. Many launchpad projects (especially memecoins) feature supplies in the trillions or quadrillions (e.g., 1e12 * 1e18 = 1e30). If such a token is launched, `increaseStake` will revert due to overflow when `totalShares` exceeds ~79 billion, enabling a permanent DoS on the launchpad for that asset and locking user funds.
 ### Static Signals
uint96 shares, uint96 baseRewardDebt, mix 6/8/18 decimals without normalization
 ### Assets at Risk
users' staked assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: NonStandardERC20Behavior

 ### Relevant Function/Location: RewardsTrackerLib.getAccRewardsPerShare

 ### Title
Incompatible precision factor causes total reward loss for USDC pairs
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a hardcoded `PRECISION_FACTOR` of `1e12`. When the `launchAsset` has 18 decimals (common) and the `quoteAsset` is USDC (6 decimals), the reward accrual calculation `(pending * 1e12) / totalShares` rounds to zero for standard amounts. For instance, with 1M tokens staked (1e24 wei), 1 USDC reward (1e6 wei) results in `1e18 / 1e24 = 0`. This leads to the complete loss of yield for stakers unless improbably large reward amounts are added at once.
 ### Static Signals
fixed precision factor, division before multiplication potential, decimals mismatch
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.swap

 ### Title
AMM Swap DoS when Distributor has zero shares
 ### Description/Code Snippet
GTELaunchpadV2Pair calls `Distributor.addRewards` during swaps to distribute fees. `addRewards` reverts if `totalShares` is zero. If all users unstake from the Distributor (e.g., after launchpad graduation/migration), the AMM pair becomes unusable as every swap attempts to distribute fees and reverts.
 ### Static Signals
external call in critical path, revert condition in called contract triggers DoS
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitFrontRun

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
Permit signature malleability enables front-running DoS
 ### Description/Code Snippet
The `permit` function in UniswapV2ERC20 uses `ecrecover` without verifying that the `s` value is in the lower half of the curve. Attackers can observe a valid permit transaction, flip the `s` value to create a valid equivalent signature, and front-run the user to consume the nonce, causing the user's transaction to revert.
 ### Static Signals
ecrecover used without requiring s <= secp256k1n/2, no ECDSA library usage
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Unclaimable dust accumulation in totalPendingRewards
 ### Description/Code Snippet
In `RewardsTrackerLib.update()`, the `accBaseRewardPerShare` is calculated as `(pending * 1e12) / totalShares`. If `totalShares` is significantly large (which is expected for high-supply launch tokens) and `pending` is small, this division rounds down to zero. The `pending` amount is cleared from `pendingBaseRewards` but not added to the global accumulator. However, `Distributor` tracks this amount in `totalPendingRewards` via `addRewards`. This creates a permanent divergence where `totalPendingRewards` includes funds that are mathematically impossible for users to claim. This 'ghost' balance cannot be skimmed by `skimExcessRewards` (as `balance - totalPending` would imply no excess), effectively locking the dust in the contract forever.
 ### Static Signals
consistent floor toward sender/receiver, divide before multiply
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: Distributor.getAccRewardsPerShare

 ### Title
Precision Loss in Reward Distribution for High-Supply Tokens
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a hardcoded `PRECISION_FACTOR` of `1e12` while `totalShares` (uint96) allows up to ~7.9e28 shares. For tokens with high total supply, the reward accumulator calculation `(pending * 1e12) / totalShares` suffers from significant truncation (rounding to zero) if the pending reward amount is not sufficiently large relative to the supply. Frequent user interactions triggering `update()` can systematically lose rewards to rounding errors.
 ### Static Signals
mix 6/8/18 decimals without normalization, divide before multiply
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: InitOrderOrUnintialized

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Uninitialized Reward Pool DoS
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` is deployed via a factory and initialized. However, the corresponding reward pool in `Distributor` is created via a separate call to `createRewardsPair`. If the pair receives trades before `createRewardsPair` is called, `GTELaunchpadV2Pair._update` calls `Distributor.addRewards`, which reverts with `RewardsDoNotExist` (implied by the check `if (rs.quoteAsset == address(0))`). This bricks the pair until the distributor is configured.
 ### Static Signals
revert RewardsDoNotExist, ordering dependency
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: Distributor.RewardsTrackerLib.getAccRewardsPerShare

 ### Title
Reward Loss due to Low Precision Factor
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a `PRECISION_FACTOR` of `1e12` to calculate `accRewardPerShare`. If the staking token has high decimals (e.g., 18) and the reward token has low decimals (e.g., USDC with 6), the calculation `(rewardAmount * 1e12) / totalShares` often truncates to zero for realistic amounts (e.g., < 1000 USDC reward for 1M staked tokens). This results in complete loss of rewards for stakers.
 ### Static Signals
PRECISION_FACTOR = 1e12, division (pending * precision) / totalShares, scaling factor < 1e18
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: RewardsTrackerLib.update

 ### Title
Precision Loss in RewardsTracker Erases Rewards for Large Pools
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a `PRECISION_FACTOR` of `1e12` when calculating accumulated rewards per share. The update formula is `acc += (pending * 1e12) / totalShares`. If `pending * 1e12 < totalShares`, the result is zero. For tokens with high supply (e.g., 100 billion tokens = 1e29 wei), even significant reward amounts (up to ~1e17 wei) are rounded down to zero and permanently lost from the accounting, leading to fee accounting drift and user loss.
 ### Static Signals
PRECISION_FACTOR = 1e12, division by totalShares with low precision multiplier
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: GTELaunchpadV2Pair._getLaunchpadFees

 ### Title
Launchpad fee rounding leads to revenue loss
 ### Description/Code Snippet
In `GTELaunchpadV2Pair._getLaunchpadFees`, the fee calculation performs integer division `(amount * share * balance) / (total * 1000)`. Small trades or trades where the numerator is smaller than the denominator result in zero fees sent to the distributor, allowing systematic fee avoidance via order splitting.
 ### Static Signals
fee taken before scaling normalization, division before summation
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: Distributor.skimExcessRewards

 ### Title
Unclaimable dust rewards cause accounting drift and lock funds
 ### Description/Code Snippet
Due to integer division in `RewardsTrackerLib`, a small portion of rewards (dust) is often left undistributed in the accumulator logic. However, `Distributor.totalPendingRewards` tracks the full input amount. As users claim, `totalPendingRewards` is decremented by the claimed amount, but not the dust. Over time, `totalPendingRewards` drifts higher than the actual claimable balance. Since `skimExcessRewards` calculates skimmable funds as `balance - totalPendingRewards`, this drift causes the function to under-calculate the excess, effectively locking the dust in the contract forever.
 ### Static Signals
fee taken before scaling normalization, flooring in looped reward distribution
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: GTELaunchpadV2Pair.permit

 ### Title
Malleable signatures accepted in UniswapV2ERC20 permit
 ### Description/Code Snippet
The `permit` function in `UniswapV2ERC20` (inherited by `GTELaunchpadV2Pair`) uses `ecrecover` without verifying that the `s` value is in the lower half of the curve (`s <= secp256k1n/2`) or that `v` is 27/28. This allows an attacker to construct a valid signature with a high `s` value from a user's valid signature. While the nonce prevents replay of the same action, an attacker can front-run a user's `permit` transaction with the malleable signature, consuming the nonce and causing the user's original transaction (and any batched logic) to revert.
 ### Static Signals
ecrecover used without requiring s <= secp256k1n/2, v not validated to 27/28
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: RewardsTrackerLib.update

 ### Title
Low precision factor in RewardsTracker leads to significant yield loss
 ### Description/Code Snippet
The `RewardsTrackerLib` uses a `PRECISION_FACTOR` of `1e12` for calculating `accRewardPerShare`. Launchpad tokens typically have 18 decimals and large supplies (e.g., 1 billion = 1e27 wei). If `totalShares` is around 1e27, any reward addition `amount` less than `1e15` (0.001 tokens) results in `(amount * 1e12) / 1e27 = 0`. Since `GTELaunchpadV2Pair` sends fees on every swap (often small amounts), a substantial portion of rewards will be lost to rounding errors and permanently locked in the `Distributor`.
 ### Static Signals
PRECISION_FACTOR = 1e12, division by totalShares without high precision scaling
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: Distributor.claimRewards

 ### Title
Claiming blocked by coupled asset transfer failure
 ### Description/Code Snippet
The `claimRewards` function attempts to distribute both `baseAsset` and `quoteAsset` rewards in the same transaction via `_distributeAssets`. If the `quoteAsset` (e.g., USDC) is paused, blacklists the user, or reverts on transfer for any reason, the user is unable to claim their `baseAsset` (Project Token) rewards. The failure of one asset's transfer griefs the claim of the other.
 ### Static Signals
no try/catch around external hook, callback success required for core flow to proceed
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: GTELaunchpadV2Pair._getLaunchpadFees

 ### Title
Flash loan manipulation of LP totalSupply bypasses Launchpad fee
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` calculates the `launchpadFee` (a protocol tax sent to the Distributor) based on the ratio of the Launchpad's LP balance to the total LP supply: `fee = amountIn * REWARDS_FEE_SHARE * launchpadLpBal / (totalLpBal * 1000)`. An attacker can flash-mint a massive amount of LP tokens to inflate `totalLpBal` (`totalSupply`) while `launchpadLpBal` remains constant. This drives the fee ratio to near zero. The attacker then performs swaps without paying the protocol tax (the 0.3% swap fee remains in the pool instead of being diverted). Finally, the attacker burns their LP tokens to reclaim their liquidity plus the untaxed swap fees, effectively stealing yield from the Distributor.
 ### Static Signals
uses totalSupply/totalAssets in same tx as deposit/withdraw, fee calculation depends on spot totalSupply
 ### Assets at Risk
rewards, fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
AMM Pair DoS via Reverting Fee Distribution Hook
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` contract calls `IDistributor(distributor).addRewards` inside its `_update` function (triggered by `swap`, `mint`, `burn`). This external call is not wrapped in a try/catch block. If `addRewards` reverts—for example, if the pair tokens are not registered in the Distributor (`RewardsDoNotExist`), or if there are no shares (`NoSharesToIncentivize`)—the core AMM functionality reverts. This renders the pair unusable for any tokens not explicitly supported by the Launchpad/Distributor, and creates a fragility where Distributor state issues brick the liquidity pool.
 ### Static Signals
no try/catch around external hook, callback success required for core flow to proceed
 ### Assets at Risk
Liquidity Pool functionality
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Insolvency with Fee-On-Transfer tokens in `addRewards`
 ### Description/Code Snippet
The `addRewards` function updates the internal `totalPendingRewards` and pool accounting by the full `amount` parameter, but uses `safeTransferFrom` to pull tokens. If the reward token (Base or Quote) implements a fee-on-transfer mechanism, the Distributor contract receives fewer tokens than it accounts for. This discrepancy creates a deficit, eventually causing the contract to hold insufficient funds to pay out the last claimers, leading to a DoS or loss of funds for late claimers.
 ### Static Signals
safeTransferFrom used without checking balance increase, Accounting state updated with input parameter directly
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
DoS of AMM Swaps when Distributor Shares are Zero
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` attempts to distribute fees to the `Distributor` via `addRewards` on every swap where fees accrue. `Distributor.addRewards` explicitly reverts with `NoSharesToIncentivize` if `totalShares == 0`. If all users unstake (e.g., by selling their launch tokens or via Launchpad logic) or if the Launchpad initializes the pair before any stakes exist, the Uniswap pair becomes completely unusable (DoS) as all swap/mint/burn operations will revert.
 ### Static Signals
revert NoSharesToIncentivize, external call in state changing function
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Distributor.increaseStake

 ### Title
Rewards Misdirected to Launchpad Contract instead of User
 ### Description/Code Snippet
The functions `increaseStake` and `decreaseStake` are called by the `launchpad` contract to update user shares. These functions trigger `_distributeAssets` to pay out pending rewards accumulated by the user (`account`). However, `_distributeAssets` transfers the assets to `msg.sender` (the `launchpad` contract) instead of the user (`account`). Unless the Launchpad contract has specific logic to handle and forward these arbitrary reward tokens, the funds will be permanently stuck in the Launchpad contract.
 ### Static Signals
transfer to msg.sender in function called by intermediary, mismatch between beneficiary account and recipient
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Fee-On-Transfer Token Support Missing in addRewards
 ### Description/Code Snippet
The `addRewards` function accepts arbitrary token addresses and amounts, updating the internal `totalPendingRewards` and `RewardPoolData` tracking based on the input `amount`. It then calls `safeTransferFrom(msg.sender, address(this), amount)`. If a Fee-On-Transfer (FOT) token is used, the contract receives less than `amount`, but the accounting records the full `amount`. This discrepancy leads to an insolvency of the reward pool, causing the last users who attempt to `claimRewards` to fail due to insufficient contract balance.
 ### Static Signals
accounting based on transfer parameter, not actual balance change, no balanceBefore/After check
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
addRewards allows mismatched quote tokens enabling reward theft
 ### Description/Code Snippet
The `addRewards` function allows permissionless addition of rewards to a pool. It identifies the target pool using `token0` and `token1` (determining which is base and which is quote). However, it fails to verify that the identified `quoteAsset` argument actually matches the pool's stored `quoteAsset` (e.g. `rs.quoteAsset`). 

An attacker can call `addRewards(BaseToken, FakeToken, 0, Amount)`. The contract identifies the pool for `BaseToken`, sees `rs.quoteAsset` is set (e.g. to USDC), but proceeds to use `FakeToken` as the `quoteAsset` for the transfer. It transfers `FakeToken` from the attacker, but increases the pool's `pendingQuoteRewards` accumulator. 

When `update()` runs, this amount is credited to the share accumulator. When users (or the attacker) claim, `claimRewards` distributes the pool's *actual* `quoteAsset` (USDC) based on the inflated accumulator. This allows 1:1 theft of the pool's quote assets using worthless tokens.
 ### Static Signals
missing check: quoteAsset == rs.quoteAsset, input argument mismatch with storage
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Missing asset validation in addRewards allows draining legitimate reward tokens
 ### Description/Code Snippet
The `addRewards` function in `Distributor.sol` accepts two arbitrary token addresses (`token0`, `token1`) and determines which one is the `launchAsset` by checking if a reward pool exists. However, it blindly accepts the *other* token as the `quoteAsset` without verifying it matches `rs.quoteAsset` (the pool's immutable quote token). An attacker can call `addRewards(launchAsset, FakeToken, 0, amount)`, which transfers `FakeToken` to the contract but increases the `rs.pendingQuoteRewards` counter. When legitimate users call `claimRewards`, the contract uses `rs.quoteAsset` (the real token, e.g., USDC) to pay out the inflated amount, effectively allowing the attacker to drain all real quote tokens from the Distributor.
 ### Static Signals
input token variable used as key for state update without equality check against stored asset, rs.addQuoteRewards called with user-supplied address, claim function uses stored asset address while addRewards uses input address
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

