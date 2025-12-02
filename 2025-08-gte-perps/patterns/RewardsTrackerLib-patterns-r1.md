## Verified Patterns Found: 8

## Verified Patterns Found in following Categories:

- FlashLoanEconomicManipulation
- AccountingInvariantViolation
- GriefableCallbacks
- UnsafeAssembyTypeCasts
- Reentrancy
- PermitFrontRun
- PermitMisuse



## Summary of Patterns

Reward accounting can overflow uint96 rewardDebt, breaking per-user rewards invariants

Launchpad fee distribution relies on external Distributor that can brick swaps

Reward accounting can overflow/truncate due to uint96 casts in RewardsTracker

RewardsTracker downcasts to uint96/uint128 without range checks, risking reward accounting corruption

Permit allows malleable signatures enabling front‑running and griefing of approvals

Launchpad fee share on swaps manipulable via same-tx liquidity changes

Reentrancy in endRewardsAccrual via fee distribution callback can desync reserves and fees

UniswapV2-style LP token permit lacks malleability checks on signatures

## Patterns



 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: RewardsTrackerLib.stake/unstake/claim

 ### Title
Reward accounting can overflow uint96 rewardDebt, breaking per-user rewards invariants
 ### Description/Code Snippet
The rewards tracking library uses `uint96` to store per-user reward debts while using much larger ranges for accumulated rewards per share and pending rewards. This can cause silent truncation/overflow of reward debt and break the invariant that a user's claimable rewards equal their pro-rata share of added rewards.

Relevant code (one of several copies):
```solidity
struct UserRewardData {
    uint96 shares; // User's current share count (up to ~7.9e28)
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}

uint128 public constant PRECISION_FACTOR = 1e12;

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}

function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
}

function unstake(RewardPoolData storage self, address user, uint96 removeShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
}

function claim(RewardPoolData storage self, address user)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
    uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}

function getAccRewardsPerShare(RewardPoolData storage self)
    internal
    view
    returns (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare)
{
    uint96 totalShares = self.totalShares;
    ...
    if (self.pendingBaseRewards > 0) {
        accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
    }
    if (self.pendingQuoteRewards > 0) {
        accQuoteRewardsPerShare += ((self.pendingQuoteRewards * PRECISION_FACTOR) / uint128(totalShares));
    }
}
```

Key points:
- `pendingBaseRewards` / `pendingQuoteRewards` are `uint128` and unbounded by design (anyone can `addRewards` arbitrary large values over time in `Distributor.addRewards`).
- `accBaseRewardPerShare` / `accQuoteRewardPerShare` are `uint256` and can grow very large because the delta added each time is `(pendingRewards * 1e12) / totalShares`.
- `totalAccRewards(shares, accRewardsPerShare)` is computed in full `uint256` precision and can easily exceed `2**96 - 1` for large combinations of `shares` and `accRewardsPerShare`.
- The result is then **cast down** to `uint96` when stored as `baseRewardDebt` / `quoteRewardDebt` without any bounds checks. This silently truncates the high bits.

Once truncation happens, later calculations of `baseAmount` / `quoteAmount` (e.g. in `claim` or `stake`/`unstake` pre-claim logic) use the full `uint256 totalAccRewards` minus the truncated `uint96` `rewardDebt`. This can:
- Over-credit a user relative to their true pro-rata share (if `rewardDebt` wrapped to a smaller value), draining the reward pool at the expense of other stakers.
- Or, after enough additions, underflow-deny future claims (if `totalAccRewards` and truncated `rewardDebt` interact badly), effectively bricking reward withdrawals.

From the `Distributor` side:
```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    ...
    if (launchAssetAmount > 0) {
        rs.addBaseRewards(launchAsset, launchAssetAmount);
        _increaseTotalPending(launchAsset, launchAssetAmount);
        launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
    }
    if (quoteAssetAmount > 0) {
        rs.addQuoteRewards(launchAsset, quoteAsset, quoteAssetAmount);
        _increaseTotalPending(quoteAsset, quoteAssetAmount);
        quoteAsset.safeTransferFrom(msg.sender, address(this), uint256(quoteAssetAmount));
    }
}

function claimRewards(address launchAsset) external returns (uint256 baseAmount, uint256 quoteAmount) {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    (baseAmount, quoteAmount) = rs.claim(msg.sender);
    _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount);
    }
    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(msg.sender, quoteAmount);
    }
}
```

Because `totalPendingRewards[asset]` is only checked against the *per-claim* amounts (`_decreaseTotalPending`), a user whose `rewardDebt` has wrapped can be overpaid relative to their fair share, but still within `totalPendingRewards`, effectively stealing from other stakers. In more extreme cases, truncated debts can also cause certain users' claims to revert (`ClaimAmountExceedsTotalPendingRewards`) and permanently lock a portion of the rewards.

This is a classic accounting invariant violation: the intended invariant that rewards are distributed proportionally to shares breaks once `totalAccRewards` exceeds the `uint96` range used for `rewardDebt`. There is no practical cap in the code to prevent `accRewardsPerShare * shares` from exceeding `2**96` over long lifetimes or with large reward injections.
 ### Static Signals
rewardDebt stored as uint96, totalAccRewards computed as uint256(shares * accRewardsPerShare / 1e12), no bounds check before casting totalAccRewards to uint96, pendingBaseRewards/QuoteRewards are uint128 and unbounded, accBaseRewardPerShare/accQuoteRewardPerShare are uint256 and can grow large over time
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._distributeLaunchpadFees

 ### Title
Launchpad fee distribution relies on external Distributor that can brick swaps
 ### Description/Code Snippet
GTELaunchpadV2Pair routes a portion of swap fees to an external Distributor contract every time `_update` decides to distribute accumulated launchpad fees. This happens inside `_update` via `_distributeLaunchpadFees`, which performs an unbounded external call to an arbitrary `launchpadFeeDistributor` address without any isolation or fallback.

Relevant code:

```solidity
address public launchpadFeeDistributor;
...
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 _reserve0,
    uint112 _reserve1,
    uint112 newLaunchpadFee0,
    uint112 newLaunchpadFee1
) private {
    ...
    uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
    uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;

    uint32 blockTimestamp = uint32(block.timestamp % 2 ** 32);
    uint32 timeElapsed = blockTimestamp - blockTimestampLast; // overflow is desired
    if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
        ...
        if (launchpadFeeDistributor > address(0)) {
            if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
                delete accruedLaunchpadFee0;
                delete accruedLaunchpadFee1;
                _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
            }
        }
    } else if (launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0) {
        ...
    }
    ...
}

function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        address _token0 = token0;
        address _token1 = token1;
        address distributor = launchpadFeeDistributor;

        // Since only pairs created by the launchpad can accrue fee tracking, the tokens are trusted
        if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
        if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

        emit LaunchpadFeesCollected(fee0, fee1);
    }
}
```

Characteristics matching a `GriefableCallbacks` pattern:

- `_distributeLaunchpadFees` is called from core state paths (`swap`, `mint`, `burn`, `sync`, and `endRewardsAccrual` via `_update`).
- It forwards *all remaining gas* to `IDistributor(distributor).addRewards(...)` without:
  - `try/catch` to handle failures,
  - any gas stipend limitation,
  - any fallback path if the Distributor misbehaves.
- The `launchpadFeeDistributor` address is not hard-coded; it is provided at initialization by the factory and stored in an unvalidated `address public launchpadFeeDistributor`.

Because the external call is not isolated, any revert in the Distributor (or misconfigured distributor address pointing to a contract that reverts, consumes excessive gas, or is self-destructed / non-code) will make `_update` revert. Since `_update` is a critical internal function used in `swap`, `mint`, `burn`, `sync`, and `endRewardsAccrual`, this can escalate to a **DoS of the entire AMM pair** once launchpad-fee distribution is active and `totalLaunchpadFee0 | totalLaunchpadFee1 > 0`.

Example grief scenario:

1. The factory initializes a pair with a valid `launchpadFeeDistributor` and rewards start accruing via swaps.
2. At some point, the Distributor contract is upgraded/replaced, or its logic is changed (within spec for that contract) such that under some state (e.g., `totalShares == 0`, pool not configured for the given tokens, or some invariant fails) `addRewards` reverts.
3. Now, whenever a swap or sync causes `_update` to run in a block with `timeElapsed > 0` and `totalLaunchpadFee0 | totalLaunchpadFee1 > 0`, `_distributeLaunchpadFees` will call into `addRewards` and revert.
4. From that point forward, **all swaps (and other reserve-updating ops) on this pair can be permanently bricked** until governance manually fixes the Distributor or re-deploys the pair.

This is a classic *griefable callback* pattern: a user does not control the callback target, but because the system’s core functionality depends on a non-isolated external call, any bug, misconfiguration, or future behavior change in the Distributor can brick the AMM. The pair makes a strong assumption that `launchpadFeeDistributor` is permanently well-behaved and compatible with all future fee distributions. Even if governance is trusted, this tight coupling between critical core logic and an external module with no failure isolation is fragile and presents a realistic DoS risk for the pool and its LP/users.

Later stages of review should determine whether the project’s specifications guarantee that `addRewards` can *never* revert during normal operations; if not, this is a concrete system-level DoS vector.
 ### Static Signals
external call to arbitrary distributor address, no try/catch around callback, callback required for core flow (_update) to complete, all gas forwarded to callback
 ### Assets at Risk
AMM liquidity usability, swap availability, launchpad fee rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.increaseStake / decreaseStake / claimRewards (via RewardsTrackerLib.stake/unstake/claim)

 ### Title
Reward accounting can overflow/truncate due to uint96 casts in RewardsTracker
 ### Description/Code Snippet
The rewards distribution logic backing GTELaunchpadV2Pair’s fee siphoning uses `RewardsTrackerLib`, which stores per-user shares and reward debts as `uint96`, but performs reward calculations in full `uint256` and then downcasts back to `uint96` without bounds checks.

Relevant structs and fields:
```solidity
struct UserRewardData {
    uint96 shares; // User's current share count (up to ~7.9e28)
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}

struct RewardPoolData {
    uint96 totalShares; // Sum of all user shares
    address quoteAsset; // Secondary reward token
    uint128 pendingBaseRewards;
    uint128 pendingQuoteRewards;
    uint256 accBaseRewardPerShare; // Accumulated base rewards per share, scaled by 1e12
    uint256 accQuoteRewardPerShare; // Accumulated quote rewards per share, scaled by 1e12
    mapping(address => UserRewardData) userRewards;
}
```

Core library logic (appears multiple times in the repo, same implementation):
```solidity
uint128 public constant PRECISION_FACTOR = 1e12;

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}

function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    if (newShares == 0) revert ZeroShareStake();

    (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

    UserRewardData storage userData = self.userRewards[user];
    uint256 existingShares = uint96(userData.shares);

    if (existingShares > 0) {
        baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
        quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
    }

    userData.shares += newShares;
    self.totalShares += newShares;

    userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
}

function unstake(RewardPoolData storage self, address user, uint96 removeShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

    UserRewardData storage userData = self.userRewards[user];
    if (removeShares == 0) revert ZeroShareStake();

    uint256 existingShares = uint256(userData.shares);
    if (existingShares < removeShares) revert InsufficientShares();

    baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
    quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;

    userData.shares -= removeShares;
    self.totalShares -= removeShares;

    userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
}

function claim(RewardPoolData storage self, address user)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    UserRewardData storage userData = self.userRewards[user];
    uint256 shares = uint256(userData.shares);
    if (shares == 0) revert ZeroShareClaim();

    (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

    uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
    uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}
```

Key issues:

1. **Downcasting 256-bit accumulated rewards to uint96 without checks**
   * `totalAccRewards(...)` returns a `uint256` proportional to `shares * accRewardsPerShare / 1e12`.
   * This value is then cast to `uint96` for `baseRewardDebt` and `quoteRewardDebt`:
     ```solidity
     userData.baseRewardDebt = uint96(totalAccRewards(...));
     userData.quoteRewardDebt = uint96(totalAccRewards(...));
     ```
   * If cumulative rewards per share grow large over the pool’s lifetime (lots of `addRewards` calls and/or long-running pool), `totalAccRewards` can exceed `2^96 - 1`. Solidity’s unchecked downcast will truncate higher bits, corrupting the stored debt.

2. **Inconsistent type widths between calculation and storage**
   * The deltas for claimable rewards use:
     ```solidity
     pendingBase = totalAccRewards(...) - userData.baseRewardDebt;      // userData.baseRewardDebt is uint96
     pendingQuote = totalAccRewards(...) - userData.quoteRewardDebt;
     ```
   * Once `baseRewardDebt` is truncated, `pendingBase` no longer equals the real accumulated-minus-claimed value. Depending on the overflow pattern:
     * Some users can be overpaid (claim more than they “should”), or
     * Future claims can be underpaid or effectively zero for large pools.
   * This breaks the fundamental invariant that
     `sum(claimed rewards) + pending rewards == total tokens funded into the rewards pool`.

3. **System-level impact through Distributor and launchpad pair**
   * `Distributor.addRewards` and `_distributeAssets` rely on `RewardsTrackerLib` invariants:
     ```solidity
     function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
         ...
         if (launchAssetAmount > 0) {
             rs.addBaseRewards(launchAsset, launchAssetAmount);
             _increaseTotalPending(launchAsset, launchAssetAmount);
             launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
         }
         ...
     }

     function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
         if (baseAmount > 0) {
             _decreaseTotalPending(base, baseAmount);
             base.safeTransfer(msg.sender, baseAmount);
         }
         ...
     }
     ```
   * The launchpad pair `GTELaunchpadV2Pair` pushes a portion of swap fees into `Distributor.addRewards` via `_distributeLaunchpadFees`:
     ```solidity
     function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
         if ((fee0 | fee1) > 0) {
             address distributor = launchpadFeeDistributor;
             if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
             if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));
             IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));
             emit LaunchpadFeesCollected(fee0, fee1);
         }
     }
     ```
   * If the reward-debt truncation causes users to be overpaid or underpaid vs the `totalPendingRewards` mapping and actual token balances, it can:
     * drain reward pools faster than intended, effectively gifting some accounts extra rewards funded by others, or
     * leave residual `pending*Rewards` that are never distributable (accounting says all paid, but balances still exist), which may end up stuck or only recoverable via admin `skimExcessRewards`.

This is a classic **accounting invariant violation**: cumulative rewards per share are tracked at full 256-bit precision, but permanently stored per-user debts are only 96 bits without bounds checks. Over long operation or large reward flows, this can realistically corrupt the mapping between funded rewards and user claims.

A fix generally requires:
* Using wider types for `shares` and `rewardDebt` (e.g. `uint128` or `uint256`), or
* Explicitly checking that `totalAccRewards(...) <= type(uint96).max` and preventing further reward addition once the limit is approached, or
* Renormalizing `acc*RewardPerShare` when they grow too large.

 ### Static Signals
rewardDebt stored as uint96, shares and accRewardPerShare multiplied in uint256, uint256 -> uint96 downcast without check, totalPendingRewards assumes correct reward accounting
 ### Assets at Risk
launchpad swap-fee rewards, user reward balances in Distributor, residual reward tokens potentially skimmed to treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: RewardsTrackerLib.stake / unstake / claim

 ### Title
RewardsTracker downcasts to uint96/uint128 without range checks, risking reward accounting corruption
 ### Description/Code Snippet
The RewardsTracker library used by the launchpad `Distributor` stores key accounting values in `uint96` / `uint128` fields, but repeatedly downcasts full-width `uint256` intermediate results into these smaller types without explicit range checks.

Structs:
```solidity
struct UserRewardData {
    uint96 shares;           // up to ~7.9e28
    uint96 baseRewardDebt;   // Used to calculate base token rewards owed
    uint96 quoteRewardDebt;  // Used to calculate quote token rewards owed
}

struct RewardPoolData {
    uint96 totalShares;
    address quoteAsset;
    uint128 pendingBaseRewards;
    uint128 pendingQuoteRewards;
    uint256 accBaseRewardPerShare;
    uint256 accQuoteRewardPerShare;
    mapping(address => UserRewardData) userRewards;
}
```

Problematic casts (examples):
```solidity
function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 existingShares = uint96(userData.shares);
    ...
    // Update reward debts
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
}

function unstake(RewardPoolData storage self, address user, uint96 removeShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 existingShares = uint256(userData.shares);
    ...
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
}

function claim(RewardPoolData storage self, address user)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
    uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

    // Update reward debts
    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}
```

`totalAccRewards` is:
```solidity
function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR; // PRECISION_FACTOR = 1e12
}
```

There is no explicit upper bound ensuring that `totalAccRewards(...)` fits into 96 bits. Over long operation, with large `pendingBaseRewards`, `accBaseRewardPerShare` and `shares`, the accumulated rewards per user can realistically exceed `2^96-1`. When that happens, the cast to `uint96` silently truncates high bits:
- `userData.baseRewardDebt` / `quoteRewardDebt` lose precision or wrap,
- Subsequent `claim()` or `unstake()` rely on these truncated debts in subtractions like `totalAccBaseRewards - userData.baseRewardDebt`, which can miscompute user entitlements.

Plausible failure modes:
* If truncation decreases stored debt, users may over-claim rewards (reward pool insolvency / conservation invariant break).
* If truncation increases stored debt beyond the true accumulated amount, users may be permanently unable to claim a portion of their rewards (value stuck in Distributor’s reward balances).

Because the Distributor additionally tracks `totalPendingRewards[asset]` and decreases it by `baseAmount`/`quoteAmount` on distribution, any miscomputed `baseAmount`/`quoteAmount` from wrapped debts will directly corrupt that global invariant, potentially causing:
* `ClaimAmountExceedsTotalPendingRewards` reverts on honest claims, or
* Draining `totalPendingRewards` while actual per-user accounting still shows positive entitlements.

This pattern appears multiple times and applies to both base and quote reward paths, making it a systemic accounting risk.
 ### Static Signals
downcasts from uint256 to uint96 without bounds check, rewards and debts stored in reduced-width integers, PRECISION_FACTOR scaling amplifies intermediate values
 ### Assets at Risk
launchpad rewards, user reward balances, Distributor totalPendingRewards accounting
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitFrontRun

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
Permit allows malleable signatures enabling front‑running and griefing of approvals
 ### Description/Code Snippet
The UniswapV2-style LP token used by `GTELaunchpadV2Pair` implements EIP-2612-like `permit` but does not enforce the usual ECDSA malleability constraints on the `s` value, nor does it normalize / restrict `v` beyond what `ecrecover` itself enforces.

Relevant code:

```solidity
function permit(
    address owner,
    address spender,
    uint256 value,
    uint256 deadline,
    uint8 v,
    bytes32 r,
    bytes32 s
) external {
    require(deadline >= block.timestamp, "UniswapV2: EXPIRED");
    bytes32 digest = keccak256(
        abi.encodePacked(
            "\x19\x01",
            DOMAIN_SEPARATOR,
            keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))
        )
    );
    address recoveredAddress = ecrecover(digest, v, r, s);
    require(recoveredAddress != address(0) && recoveredAddress == owner, "UniswapV2: INVALID_SIGNATURE");
    _approve(owner, spender, value);
}
```

Issues:
- No check that `s` is in the lower half of the secp256k1 curve order (no `s <= secp256k1n/2` constraint).
- No explicit `v` range normalization (relying solely on `ecrecover` to return `address(0)` on bad `v`).

Because `s` is not restricted, **two distinct signatures** (high-s and low-s variants) exist for the same `(owner, spender, value, nonce, deadline)` payload. Either variant is valid and will increment `nonces[owner]`.

This enables a realistic **permit front‑running** / griefing scenario:
1. A user signs a permit off-chain for a dApp (router) using a standard, low-`s` signature.
2. An attacker who sees or receives that signature can compute the corresponding high-`s` malleated signature (or vice versa, depending on the wallet implementation).
3. The attacker front-runs the victim’s transaction and submits the malleated signature to `permit`, consuming the nonce and granting approval to the same or a different `spender` of their choosing (if the dApp does not strictly tie the `spender` to a known value).
4. The victim’s subsequent tx using the original signature fails because `nonces[owner]` has already been incremented.

Even if the attacker cannot change `spender` (because the victim signs with a specific router address), they can still **grief** by burning the victim’s permit (consuming the nonce before the intended tx executes), or race to use the approval first on that router path if the router itself is not tightly designed around atomic permit+use flows.

While this pattern is inherited from the canonical Uniswap V2 implementation, in the context of this protocol it remains a concrete, exploitable pattern whenever off-chain permits are reused across multiple venues or when users rely on `permit` being single-use and non-front-runnable.

 ### Static Signals
no s <= secp256k1n/2 check, v not normalized / not explicitly restricted to {27,28}, nonce incremented inline with signature use, EIP-2612-style permit used for LP token approvals
 ### Assets at Risk
user LP token balances, downstream funds approved to routers or other spenders
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: GTELaunchpadV2Pair.swap

 ### Title
Launchpad fee share on swaps manipulable via same-tx liquidity changes
 ### Description/Code Snippet
In `GTELaunchpadV2Pair`, the launchpad’s share of swap fees is computed using the current LP token distribution at the time of the swap, without any multi-block observation or stabilization. Because the pair only uses a simple reentrancy lock (per-call stack) and **does not prevent multiple different entrypoints being called in the same transaction**, an attacker can change the LP token supply and distribution immediately before a swap and then restore it afterwards, all atomically (e.g., via a router / multicall / custom contract and flash loans).

Relevant code:

```solidity
function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external lock {
    ...
    uint256 amount0In = balance0 > _reserve0 - amount0Out ? balance0 - (_reserve0 - amount0Out) : 0;
    uint256 amount1In = balance1 > _reserve1 - amount1Out ? balance1 - (_reserve1 - amount1Out) : 0;
    if (amount0In == 0 && amount1In == 0) revert("UniswapV2: INSUFFICIENT_INPUT_AMOUNT");

    {
        uint256 balance0Adjusted = balance0.mul(1000).sub(amount0In.mul(3));
        uint256 balance1Adjusted = balance1.mul(1000).sub(amount1In.mul(3));
        if (balance0Adjusted.mul(balance1Adjusted) < uint256(_reserve0).mul(_reserve1).mul(1000 ** 2)) {
            revert("UniswapV2: K");
        }

        (uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
            && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));

        _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
    }
    ...
}

function _getLaunchpadFees(uint256 amount0In, uint256 amount1In)
    internal
    view
    returns (uint112 fee0, uint112 fee1)
{
    uint256 totalLpBal = this.totalSupply();
    uint256 launchpadLpBal = this.balanceOf(launchpadLp) + MINIMUM_LIQUIDITY;

    if (amount0In > 0) fee0 = uint112(amount0In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));
    if (amount1In > 0) fee1 = uint112(amount1In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));
}
```

Key points:

- `REWARDS_FEE_SHARE` is a fixed fraction of each swap’s input that should be routed to the launchpad rewards via the Distributor.
- The actual fraction allocated to the launchpad is scaled by `launchpadLpBal / totalLpBal`, where `totalLpBal = this.totalSupply()` and `launchpadLpBal = this.balanceOf(launchpadLp) + MINIMUM_LIQUIDITY`.
- `totalSupply` and the launchpad’s balance are fully manipulable within a single transaction by any LP:
  - Attacker can **mint** a huge amount of LP tokens right before the swap using borrowed underlying (flash loan) to massively increase `totalSupply`, making `launchpadLpBal / totalLpBal` tiny.
  - They then perform the swap; `_getLaunchpadFees` computes negligible `fee0/fee1`, so almost none of the swap fee is siphoned to the launchpad.
  - Afterwards, they can **burn** the LP they just minted to restore the pool state, all in the same transaction.
- The `lock` modifier only prevents *reentrant* calls into the same function, not sequential calls to `mint` and then `swap` (or via an external router/multicall contract), so this intra-transaction manipulation is feasible.

Impact direction:

- The attack does **not** break the x*y=k invariant or steal assets directly from the pool users; instead it modifies the **fee split** so that the launchpad’s Distributor receives far less than the intended share of swap fees.
- This yields a better effective price for the attacker’s trade (since less of the input is diverted as protocol/launchpad rewards) and drains economic value from the launchpad rewards program over time.

This matches the `FlashLoanEconomicManipulation` pattern: a critical economic decision (how much of each swap’s fee goes to rewards) is based on intra-transaction-manipulable state (`totalSupply`, `launchpadLp` balance) with no TWAP or multi-block observation, and can be skewed using flash liquidity and same-tx LP mint/burn sequences.
 ### Static Signals
fee share depends on this.totalSupply() and this.balanceOf(launchpadLp), no TWAP or multi-block observation for LP distribution, lock modifier only prevents direct reentrancy, not multi-call within same tx, same-tx mint/burn can change totalSupply before swap, reward fee fraction computed from manipulable state
 ### Assets at Risk
launchpad rewards, fee revenue expected by Distributor/launchpad stakeholders
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: GTELaunchpadV2Pair.endRewardsAccrual

 ### Title
Reentrancy in endRewardsAccrual via fee distribution callback can desync reserves and fees
 ### Description/Code Snippet
In `GTELaunchpadV2Pair`, most state‑changing AMM entrypoints (`mint`, `burn`, `swap`, `skim`, `sync`) are protected by the `lock` reentrancy guard. However, `endRewardsAccrual()` is **not** guarded by `lock` and can reenter the pair while `_update` is mid‑execution.

Flow:
- `endRewardsAccrual()` is `external` and callable by `launchpadFeeDistributor`:
```solidity
function endRewardsAccrual() external {
    if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");

    // There are no more shares, so prevent distribution and accrual of any remaining rewards
    delete accruedLaunchpadFee0;
    delete accruedLaunchpadFee1;
    delete rewardsPoolActive;

    _update(
        IERC20(token0).balanceOf(address(this)),
        IERC20(token1).balanceOf(address(this)),
        reserve0,
        reserve1,
        uint112(0),
        uint112(0)
    );

    emit RewardsPoolDeactivated();
}
```
- `_update` may call `_distributeLaunchpadFees` **before** updating reserves and after wiping `accruedLaunchpadFee*`:
```solidity
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 _reserve0,
    uint112 _reserve1,
    uint112 newLaunchpadFee0,
    uint112 newLaunchpadFee1
) private {
    // ... compute totalLaunchpadFee0/1, TWAP, etc.

    if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
        // update cumulatives
        if (launchpadFeeDistributor > address(0)) {
            if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
                delete accruedLaunchpadFee0;
                delete accruedLaunchpadFee1;
                _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
            }
        }
    } else if (launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0) {
        accruedLaunchpadFee0 = totalLaunchpadFee0;
        accruedLaunchpadFee1 = totalLaunchpadFee1;
        emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
    }

    // finally updates reserves after fee distribution
    reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;
    reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;
    blockTimestampLast = blockTimestamp;
    emit Sync(_reserve0, _reserve1);
}

function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        address _token0 = token0;
        address _token1 = token1;
        address distributor = launchpadFeeDistributor;

        // Since only pairs created by the launchpad can accrue fee tracking, the tokens are trusted
        if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
        if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

        emit LaunchpadFeesCollected(fee0, fee1);
    }
}
```
- `_safeApprove` is a low-level call into `token0`/`token1`, and `addRewards` performs ERC20 `transferFrom` on those tokens. If either token is non‑standard or has hooks (ERC777 style, or a malicious ERC20 with callback in `approve`/`transferFrom`), it can **reenter the pair** during `_update` because `endRewardsAccrual` lacks the `lock` modifier.
- Reentrancy targets include `swap`, `mint`, `burn`, `skim`, `sync`, and even `endRewardsAccrual` itself (all externally callable, and only those with `lock` are protected – but `lock` is not active when we entered through `endRewardsAccrual`).

Consequences:
- Reentrancy while `_update` is in the middle of:
  * having **deleted** `accruedLaunchpadFee0/1`,
  * not yet having updated `reserve0/1`,
  * using stale `_reserve0/_reserve1` and pre-fee `balance0/balance1`,
  can break the expected constant‑product and fee‑accounting invariants.
- An attacker controlling a token contract could:
  * trigger multiple `_distributeLaunchpadFees` calls with inconsistent `totalLaunchpadFee*`, potentially double‑claiming rewards intended for the launchpad LP,
  * execute swaps/mints/burns against stale reserves leading to incorrect `k`-invariant enforcement and mispricing, potentially extracting value from LPs or leaving pool state corrupted.

This is a **non-standard external call path before all state is fully updated**, and unlike Uniswap V2’s usual pattern, it is **not covered by the pair’s reentrancy guard**.

Mitigation ideas (for later phases):
- Apply the `lock` modifier to `endRewardsAccrual()` so that `_update` and `_distributeLaunchpadFees` run under the same global reentrancy guard as `swap/mint/burn/skim/sync`.
- Alternatively, ensure `_update` has no external calls (move `_distributeLaunchpadFees` outside or after a guard) or introduce a dedicated internal reentrancy flag around the fee distribution call.
 ### Static Signals
external function without nonReentrant/lock, calls _update which makes external calls before final state updates, uses low-level token.call in _safeApprove, calls external IDistributor.addRewards during reserve update, accruedLaunchpadFee* deleted before external call
 ### Assets at Risk
AMM LP liquidity, launchpad rewards fees, pool pricing invariants
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
UniswapV2-style LP token permit lacks malleability checks on signatures
 ### Description/Code Snippet
The LP token implemented in `UniswapV2ERC20` exposes an EIP-2612-style `permit` that is used to set spend allowances via off-chain signatures. However, the implementation does not enforce standard ECDSA malleability constraints on the recovered signature, allowing multiple distinct signatures (high-s / low-s variants, non-standard `v` values) to be valid for the same logical permit.

Code (simplified):

```solidity
function permit(
    address owner,
    address spender,
    uint256 value,
    uint256 deadline,
    uint8 v,
    bytes32 r,
    bytes32 s
) external {
    require(deadline >= block.timestamp, "UniswapV2: EXPIRED");
    bytes32 digest = keccak256(
        abi.encodePacked(
            "\x19\x01",
            DOMAIN_SEPARATOR,
            keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))
        )
    );
    address recoveredAddress = ecrecover(digest, v, r, s);
    require(recoveredAddress != address(0) && recoveredAddress == owner, "UniswapV2: INVALID_SIGNATURE");
    _approve(owner, spender, value);
}
```

Missing checks:
- No constraint on `s` to be in the lower half of the secp256k1 curve order (`s <= secp256k1n/2`).
- No validation that `v` is strictly one of {27, 28} (or normalized from {0,1}).

Impact pattern:
- Because signatures are malleable, a relayer or adversary observing a valid permit can transform it into an alternative signature that is also accepted by the contract. While the nonce protects against *replay* across transactions, malleability can still break off-chain assumptions about uniqueness and can interact poorly with aggregators or meta-tx systems that cache or index signatures by `(r,s,v)` rather than the logical message.
- In multi-system integrations (routers, batchers, off-chain signing UX), this may open paths to front-running or misaccounting where the same logical permit is submitted in a different malleated form, consuming the nonce earlier than the user or intended relayer expects and potentially breaking their flow.

Given this is a core LP token that may be used across other contracts and routers, enforcing standard ECDSA constraints is important to avoid subtle cross-system auth issues.

 ### Static Signals
no s <= secp256k1n/2 check, v not validated to 27/28, raw ecrecover used directly
 ### Assets at Risk
user approvals, LP token allowances
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

