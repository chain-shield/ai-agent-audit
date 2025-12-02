## Verified Patterns Found: 20

## Verified Patterns Found in following Categories:

- GriefableCallbacks
- FeeOnTransferAssumption
- NonStandardERC20Behavior
- PermitMisuse
- FeeAccountingDrift
- UnsafeAssembyTypeCasts
- AccountingInvariantViolation
- Reentrancy



## Summary of Patterns

Unsafe uint96 downcasts in rewards tracking can corrupt reward accounting

Reward accounting can overflow 96-bit debts and break totalPendingRewards invariant

Rounding in RewardsTracker makes rewards dust unclaimable and locks skimExcessRewards

Unchecked downcasts in RewardsTracker can overflow user reward debt and break rewards accounting

Reward debt truncation can break equality between pending rewards and escrowed totals

addRewards allows mismatched quote token, desyncing accounting and bricking reward claims

Unchecked uint256→uint96/uint128 casts in RewardsTracker can corrupt reward accounting

Reward debt stored as uint96 can overflow and corrupt rewards accounting

FoT/rebasing tokens can desync Distributor’s pending-rewards accounting from actual balances

AMM swap depends on external Distributor callback that can revert and DoS pool

Reentrancy risk in Distributor reward payouts via untrusted ERC20 callbacks

Rounding in RewardsTracker desynchronizes totalPendingRewards and locks excess rewards

Untrusted Distributor callback can DoS AMM swaps when rewards shares go to zero

Non-standard ERC20 behaviors (rebasing/deflationary) can desync Distributor totalPendingRewards and actual balances

Reward accounting can overflow/truncate due to unsafe downcasts in RewardsTrackerLib

Reward accounting can overflow 96-bit debts and break pending rewards invariant

Rewards accounting assumes 1:1 ERC20 transfers (fee-on-transfer/rebasing tokens break Distributor invariants)

Reward debt downcasts to uint96 can overflow and corrupt rewards accounting

Reward debt uint96 truncation can break rewards accounting and lock claims

UniswapV2 LP token permit() accepts malleable signatures (no s/v validation)

## Patterns



 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.increaseStake / decreaseStake / claimRewards (via RewardsTrackerLib)

 ### Title
Unsafe uint96 downcasts in rewards tracking can corrupt reward accounting
 ### Description/Code Snippet
The rewards distribution logic used by the Distributor relies on `RewardsTrackerLib`, which stores per-user reward debts as `uint96` while computing them in full `uint256` precision. The library repeatedly downcasts potentially large `uint256` values into `uint96` without any bounds checking, which can silently truncate high bits and break accounting invariants.

Key code (one of several occurrences):

```solidity
// libraries/RewardsTracker.sol
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

    // POTENTIAL BUG: unsafe downcast from uint256 to uint96
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

    // POTENTIAL BUG: unsafe downcast
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

    // POTENTIAL BUG: unsafe downcast
    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}
```

`totalAccRewards(shares, accRewardsPerShare)` returns a `uint256`, and there is no explicit cap on `accBaseRewardPerShare` / `accQuoteRewardPerShare` beyond being updated with `uint128` `pendingBaseRewards`/`pendingQuoteRewards`. Over long lifetimes, or with large reward injections and high share counts, the accumulated per-share index multiplied by shares can exceed `2^96 - 1`.

When that happens, the `uint96(...)` cast will silently truncate the upper bits of the reward debt. This creates multiple failure modes:

- **Overpayment:** If `baseRewardDebt` or `quoteRewardDebt` is truncated to a much smaller number than the true value, later calculations of `baseAmount = totalAccRewards(...) - baseRewardDebt` will see an artificially small debt and thus a much larger `baseAmount` than intended for that user, potentially allowing one user to drain a disproportionate share of the reward pool.
- **Underpayment / locked rewards:** Conversely, depending on sequence and how per-share indices evolve after truncation, some users may end up with debts that exceed the actual accrued total, making their computed `baseAmount`/`quoteAmount` smaller than what should be owed, effectively trapping a portion of rewards as unclaimable.
- **Global accounting skew:** Distributor’s `totalPendingRewards[asset]` is adjusted based solely on the amounts returned by these library functions in `Distributor.increaseStake`, `decreaseStake`, and `claimRewards`. If the library miscomputes per-user owed amounts due to truncation, the global `totalPendingRewards` mapping can diverge from the mathematically correct “sum of unclaimed user rewards”.

This is a classic accounting-invariant violation: the intended invariant

`totalPendingRewards[asset] == Σ (user pending rewards for that asset)`

can be broken by integer truncation in the reward-debt fields, especially under high TVL / long-running pools.

Because the truncation is silent and no guard (e.g. SafeCastLib) is used, it is hard to detect operationally and could manifest only under stressed conditions (large reward campaigns or long-lived pools), making this a subtle but realistic vulnerability pattern.
 ### Static Signals
reward debt stored as uint96, totalAccRewards returns uint256, unsafe downcast uint256 -> uint96 without bounds check, accumulated reward index unbounded over time
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: RewardsTrackerLib.stake/unstake/claim (uint96 rewardDebt casts)

 ### Title
Reward accounting can overflow 96-bit debts and break totalPendingRewards invariant
 ### Description/Code Snippet
The rewards tracking library used by Distributor aggressively downcasts cumulative reward values to uint96 without any overflow checks. Over long-lived pools with large reward accrual, this can cause reward debts to wrap and become much smaller than the true cumulative rewards, so a user’s computed claim amount exceeds the total rewards added / tracked in Distributor.totalPendingRewards.

Key code (RewardsTrackerLib):

```solidity
struct UserRewardData {
    uint96 shares;
    uint96 baseRewardDebt;
    uint96 quoteRewardDebt;
}
...
function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}
...
// in stake()
userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
...
// in unstake()
userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
...
// in claim()
uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

userData.baseRewardDebt = uint96(totalAccBaseRewards);
userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
```

`totalAccRewards` returns a full 256-bit value, but it is repeatedly truncated to 96 bits when stored as `baseRewardDebt` / `quoteRewardDebt`. In Solidity 0.8, casting from `uint256` to `uint96` **truncates** silently; no revert is thrown.

If `shares * accRewardsPerShare / 1e12` ever exceeds `2^96 - 1` (~7.9e28), the high bits are discarded. Over time, debts will wrap modulo 2^96 while `acc*` increases monotonically. This can make `totalAccBaseRewards - baseRewardDebt` (and similarly for quote) **much larger than the true underlying rewards that were ever added**.

Distributor assumes its own `totalPendingRewards` tracks the maximum claimable amount:

```solidity
mapping(address => uint256) public totalPendingRewards;

function _increaseTotalPending(address asset, uint256 amount) internal {
    unchecked {
        totalPendingRewards[asset] += amount;
    }
}

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];

    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();

    unchecked {
        totalPendingRewards[asset] -= amount;
    }
}
```

All reward flows (`addRewards`, then `stake`/`unstake`/`claim` via Distributor) increase `totalPendingRewards` by the exact reward amounts added, and decrease it only when payouts occur. If a user’s computed `baseAmount` / `quoteAmount` is inflated due to 96-bit wraparound, `_decreaseTotalPending` will revert with `ClaimAmountExceedsTotalPendingRewards()`.

Consequences:
- Once cumulative rewards per share become large enough, some users can enter a state where **every claim / stake / unstake that tries to pay pending rewards reverts**, effectively bricking their ability to realize rewards.
- Since `increaseStake` and `decreaseStake` also pay pending rewards and tunnel them through `_distributeAssets → _decreaseTotalPending`, even position management invoked by the Launchpad can begin failing for affected accounts.
- If such an overflow happens for many users, a significant fraction of the global rewards can become **permanently unclaimable**, breaking the accounting invariant that `totalPendingRewards` equals the contract’s distributable reward pool and causing a protocol-level denial of service on reward extraction.

This is a classic accounting-invariant violation driven by unsafe downcasting in long-lived reward pools with high volume and/or large share counts.
 ### Static Signals
reward debts stored as uint96, uint256→uint96 truncation of totalAccRewards, PRECISION_FACTOR-based cumulative index grows unbounded, Distributor.totalPendingRewards used as hard cap with revert on over-claim
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Rounding in RewardsTracker makes rewards dust unclaimable and locks skimExcessRewards
 ### Description/Code Snippet
The Distributor relies on `totalPendingRewards[asset]` to track the total amount of rewards owed to all users for a given token. This is increased in `Distributor.addRewards` and decreased whenever rewards are paid out via `_distributeAssets`:

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

function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}
```

However, the underlying rewards accounting library `RewardsTrackerLib` uses integer division when converting `pendingBaseRewards`/`pendingQuoteRewards` into per-share indices, and then **deletes** the pending rewards completely:

```solidity
uint128 public constant PRECISION_FACTOR = 1e12;
...
function getAccRewardsPerShare(RewardPoolData storage self)
    internal view
    returns (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare)
{
    uint96 totalShares = self.totalShares;
    if (totalShares == 0) return (self.accBaseRewardPerShare, self.accQuoteRewardPerShare);

    accBaseRewardsPerShare = self.accBaseRewardPerShare;
    accQuoteRewardsPerShare = self.accQuoteRewardPerShare;

    if (self.pendingBaseRewards > 0) {
        accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
    }

    if (self.pendingQuoteRewards > 0) {
        accQuoteRewardsPerShare += ((self.pendingQuoteRewards * PRECISION_FACTOR) / uint128(totalShares));
    }
}

function update(RewardPoolData storage self)
    internal
    returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
{
    (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    if (self.pendingBaseRewards > 0) {
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        delete self.pendingBaseRewards; // <<<< deletes full amount
    }

    if (self.pendingQuoteRewards > 0) {
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        delete self.pendingQuoteRewards; // <<<< deletes full amount
    }
}
```

Because of integer division, a small `pendingBaseRewards` (or `pendingQuoteRewards`) relative to `totalShares` can result in **zero** being added to `accBaseRewardPerShare` / `accQuoteRewardPerShare`, but the entire `pending*_Rewards` is then deleted. This means part or all of the rewards credited via `addRewards` can be irreversibly lost in the pool’s internal accounting (no user will ever be able to claim them), while `Distributor.totalPendingRewards[asset]` remains unchanged.

Over time, this creates a systematic drift:

* `totalPendingRewards[asset]` is the sum of all rewards ever added minus the amounts actually paid out to users.
* Due to the rounding/deletion above, **the maximum sum of all users’ claimable rewards is strictly less** than `totalPendingRewards[asset]` by some accumulating dust.

After all users have withdrawn/un-staked and there are no more shares, there can still be a positive `totalPendingRewards[asset]` value, but:

* No user has any `shares`, so `claim()` and `getPendingRewards()` for everyone will return 0.
* The contract balance of `asset` will equal the accumulated unclaimable dust.
* `skimExcessRewards` will revert for any non-zero `amount`, because:

```solidity
if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
```

and here `asset.balanceOf(this) == totalPendingRewards[asset]` (from the Distributor’s perspective), so `asset.balanceOf - totalPendingRewards == 0`, i.e., **no amount is considered “excess”** even though no one can ever claim it. As more such rounding dust accumulates across multiple `addRewards` / `update` cycles, this becomes larger and permanently locks those tokens in the Distributor.

Patterns matched:
* Systematic, directional rounding loss from `pending*_Rewards` → per-share indices.
* Accumulated dust cannot be recovered by users (unclaimable) or by admin (skim is blocked by `SkimOverflow`).
* Breaks the intended accounting invariant between `totalPendingRewards[asset]` and actual claimable user rewards, and effectively bricks the recovery/sweep mechanism for that asset.

An attacker or any user can grief the system by repeatedly calling `addRewards` with very small amounts relative to `totalShares` (e.g., 1 wei per call) to maximize truncation, causing `totalPendingRewards` to grow while virtually none of it is claimable, eventually making `skimExcessRewards` permanently unusable for that token.
 ### Static Signals
reward index uses division before multiplication, pendingRewards deleted after index update, global totalPendingRewards never adjusted for per-user rounding, skimExcessRewards gates withdrawals on totalPendingRewards
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: RewardsTrackerLib (used by Distributor).stake / unstake / claim

 ### Title
Unchecked downcasts in RewardsTracker can overflow user reward debt and break rewards accounting
 ### Description/Code Snippet
The rewards accounting in `RewardsTrackerLib` relies heavily on downcasting 256-bit reward totals into 96-bit / 128-bit storage fields **without any range checks**. If the cumulative per-user rewards grow beyond the bit-width of these fields, the casts will silently truncate upper bits. This can corrupt per-user reward debt, distort pending reward calculations, and potentially cause the Distributor’s `totalPendingRewards` invariant to be violated, leading to systemic reward-accounting failures.

Key locations (library used by `Distributor`):

```solidity
// libraries/RewardsTracker.sol
struct UserRewardData {
    uint96 shares; // User's current share count (up to ~7.9e28)
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}

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
    // Update reward debts
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

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR; // PRECISION_FACTOR = 1e12
}
```

**Issue mechanics:**

- `totalAccRewards(...)` returns a `uint256` representing the cumulative rewards for a user (scaled and then divided by `PRECISION_FACTOR`).
- This value is assigned into `UserRewardData.baseRewardDebt` / `quoteRewardDebt` by **downcasting to `uint96`** with no upper-bound checks.
- Over time, if either:
  - total user shares (`shares`) are large, or
  - the total rewards accumulated for the pool (and hence `accBaseRewardPerShare` / `accQuoteRewardPerShare`) grow very large,
  then `totalAccRewards` can exceed `type(uint96).max` (~7.9e28). In that case, the cast `uint96(totalAccRewards(...))` will truncate the upper bits.

**Consequences of truncation:**

1. **Incorrect per-user debts and pending reward calculations**
   - When later computing pending rewards, the code does:
     ```solidity
     baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
     ```
   - `userData.baseRewardDebt` is stored as a **truncated 96-bit value**, while `totalAccRewards` is full 256-bit. If truncation occurred, the subtraction is effectively:
     `full_totalAcc - truncated_debt`, which can be hugely larger than the *real* net owed amount.
   - This can cause:
     - Overestimation of `baseAmount` / `quoteAmount` returned from `stake`, `unstake`, or `claim`.
     - Very large reward amounts which were never funded by actual `addRewards` calls.

2. **Interaction with Distributor’s `totalPendingRewards` invariant**
   - `Distributor` tracks aggregate pending rewards per token:
     ```solidity
     mapping(address => uint256) public totalPendingRewards;

     function _increaseTotalPending(address asset, uint256 amount) internal {
         unchecked { totalPendingRewards[asset] += amount; }
     }

     function _decreaseTotalPending(address asset, uint256 amount) internal {
         uint256 currTotal = totalPendingRewards[asset];
         if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
         unchecked { totalPendingRewards[asset] -= amount; }
     }
     ```
   - On any reward distribution, `_distributeAssets` is called from `increaseStake`, `decreaseStake`, and `claimRewards`:
     ```solidity
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
   - If the truncated `baseRewardDebt`/`quoteRewardDebt` leads to **inflated** `baseAmount`/`quoteAmount`, the `_decreaseTotalPending` check can start reverting with `ClaimAmountExceedsTotalPendingRewards()` despite the underlying `RewardsTracker` believing such rewards exist. This effectively bricks reward claiming for affected pools once values cross the 96-bit threshold.

3. **Systemic risk**
   - There is no explicit upper bound enforced on:
     - The amount of rewards that can be added via `Distributor.addRewards`/`GTELaunchpadV2Pair._distributeLaunchpadFees` over time, nor
     - The lifetime of a reward pool.
   - Given long enough operation or large fee inflows, reaching `type(uint96).max` on accumulated per-user rewards is *theoretically feasible*, at which point the above truncation behavior can manifest.

This fits the `UnsafeAssembyTypeCasts` / unchecked downcast pattern: the type system is bypassed by unsafe downcasts in core accounting, and no invariant or guard ensures the 96-bit ranges are respected. The failure mode is not just cosmetic rounding — it can lead to:

- Overstated user-level pending rewards (economic impact), and/or
- Reverts at the Distributor layer that prevent valid claims (DoS on rewards).

**Note:** Whether this is practically reachable depends on scale (total rewards and time), but the pattern is clearly present and exploitable in principle under realistic high-volume conditions.
 ### Static Signals
downcasts from uint256 to uint96 without range checks, rewardDebt stored in 96 bits but computed in 256 bits, cumulative rewards can grow unbounded over protocol lifetime
 ### Assets at Risk
rewards, launchpad fee distributions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor._distributeAssets / claimRewards (via RewardsTrackerLib)

 ### Title
Reward debt truncation can break equality between pending rewards and escrowed totals
 ### Description/Code Snippet
Because `UserRewardData.baseRewardDebt` and `quoteRewardDebt` are stored as `uint96` while the accumulated reward indices and per-user accumulated rewards are tracked in `uint256`, a truncation as described in the previous pattern can cause the Distributor’s high-level accounting (`totalPendingRewards`) to diverge from the actual per-user entitlements encoded in `RewardPoolData`. This is an accounting invariant violation pattern.

Flow:
1. Rewards are added via `Distributor.addRewards`, which calls:
   ```solidity
   rs.addBaseRewards(launchAsset, launchAssetAmount);
   _increaseTotalPending(launchAsset, launchAssetAmount);
   ```
   So `RewardPoolData.pendingBaseRewards` and `Distributor.totalPendingRewards[asset]` both increase by the same amount.
2. When users stake/unstake/claim, `RewardsTrackerLib.update` folds `pending*Rewards` into the per-share indices, and then per-user debts are updated using the truncated `uint96` cast.
3. If at some point `totalAccRewards(...)` exceeds `type(uint96).max`, the stored `baseRewardDebt` wraps. Future `claim` calls will compute `baseAmount` using a wrapped debt and a large `totalAccRewards(...)`, which can yield an amount greater than what remains in `RewardPoolData` or what `Distributor.totalPendingRewards` expects after `_decreaseTotalPending`.
4. `_distributeAssets` then does:
   ```solidity
   _decreaseTotalPending(base, baseAmount);
   base.safeTransfer(msg.sender, baseAmount);
   ```
   `_decreaseTotalPending` reverts if `currTotal < amount` based on `totalPendingRewards[asset]`. After enough skew/rounding, either:
   - Users can drain more than the protocol intended before the revert starts hitting other claimants, or
   - Claims begin to revert systematically, effectively locking some rewards in the contract (the pool shows pending rewards on-chain, but they can’t be claimed).

This leads to a tangible break of the intended invariant that:
- Sum of all users’ owed rewards (as implied by their `shares` and `acc*RewardPerShare` minus `rewardDebt`) should never exceed `totalPendingRewards[asset]`, and
- Over the life of the pool, `totalPendingRewards[asset]` should decrease exactly by the amounts actually paid out.

Because the root cause is the unsafe `uint256 → uint96` cast in the reward debt fields, this is best seen as an accounting invariant violation triggered by unsafe type narrowing in the rewards math.
 ### Static Signals
per-user debts stored in uint96 while accumulators are uint256, global totalPendingRewards must track sum of user-level pending rewards, downcast can corrupt rewardDebt leading to mismatch between escrowed funds and claims
 ### Assets at Risk
rewards, future user claims, protocol reward escrow correctness
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
addRewards allows mismatched quote token, desyncing accounting and bricking reward claims
 ### Description/Code Snippet
The `Distributor.addRewards` function is permissionless and intended to let anyone top up rewards for an existing launch/quote asset pair. However, it does not validate that the `token0` / `token1` arguments match the actual `quoteAsset` configured in the reward pool.

Relevant code (Distributor.sol):

```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    (address launchAsset, address quoteAsset, uint128 launchAssetAmount, uint128 quoteAssetAmount) =
        (token0, token1, amount0, amount1);
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);

    if (rs.quoteAsset == address(0)) {
        rs = RewardsTrackerStorage.getRewardPool(token1);

        if (rs.quoteAsset == address(0)) revert RewardsDoNotExist();

        (launchAsset, quoteAsset, launchAssetAmount, quoteAssetAmount) = (token1, token0, amount1, amount0);
    }

    if (rs.totalShares == 0) revert NoSharesToIncentivize();

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
```

`createRewardsPair` sets `rs.quoteAsset` once, but only checks that no pool already exists:

```solidity
function createRewardsPair(address launchAsset, address quoteAsset) external onlyLaunchpad {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    RewardPoolData storage rsq = RewardsTrackerStorage.getRewardPool(quoteAsset);

    // Sanity check in case the admin makes the quote asset of launchpad an existing asset
    if (rs.quoteAsset != address(0) || rsq.quoteAsset != address(0)) revert RewardsExist();

    rs.initializePair(launchAsset, quoteAsset);
}
```

Crucially, `addRewards`:
- Only checks that **at least one** of `token0` or `token1` corresponds to an existing pool (`rs.quoteAsset != address(0)`),
- But **never validates** that the *other* token passed actually equals `rs.quoteAsset`.

Attack / grief scenario:
1. Assume there is a valid pool for `(launchAsset = L, quoteAsset = Q)` created by `createRewardsPair(L, Q)`.
2. Any attacker calls `addRewards` with `token0 = L`, `token1 = X` (where `X` is any arbitrary ERC20, not equal to `Q`), and `amount1 > 0`.
   - Because `getRewardPool(token0)` has `rs.quoteAsset != 0`, the `if (rs.quoteAsset == address(0))` branch is skipped.
   - The function treats `launchAsset = L`, `quoteAsset = token1 = X`.
   - It executes:
     ```solidity
     rs.addQuoteRewards(launchAsset, quoteAsset, quoteAssetAmount);
     _increaseTotalPending(quoteAsset, quoteAssetAmount); // uses X
     quoteAsset.safeTransferFrom(msg.sender, address(this), uint256(quoteAssetAmount)); // pulls X
     ```
   - The `RewardPoolData` for `L` still has `rs.quoteAsset == Q` (unchanged), but `pendingQuoteRewards` is now non-zero, and `totalPendingRewards[X]` is increased. The contract actually holds X tokens, not Q.
3. When users later interact via `Distributor.increaseStake`, `decreaseStake`, or `claimRewards` for `launchAsset = L`, the library computes `quoteAmount > 0` (because `pendingQuoteRewards` was incremented). `_distributeAssets` is then called:
   ```solidity
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
   Here, `quote` is `rs.quoteAsset`, i.e. **Q**, not X. `_decreaseTotalPending` now executes with `asset = Q`:
   ```solidity
   function _decreaseTotalPending(address asset, uint256 amount) internal {
       uint256 currTotal = totalPendingRewards[asset];
       if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
       unchecked {
           totalPendingRewards[asset] -= amount;
       }
   }
   ```
   Since all the previously added quote rewards were tracked as `totalPendingRewards[X]` and **not** `totalPendingRewards[Q]`, `currTotal` for Q is zero and the call reverts with `ClaimAmountExceedsTotalPendingRewards()`.

Consequences:
- Any non-zero `quoteAssetAmount` added with a mismatched token permanently desynchronizes accounting:
  - `RewardPoolData.pendingQuoteRewards` and the internal `accQuoteRewardPerShare` assume rewards are denominated in Q,
  - But `totalPendingRewards` and the actual tokens escrowed are for **X**.
- From that point on, any call that attempts to realize quote rewards for the pool (stake/unstake/claim that results in `quoteAmount > 0`) will revert because `_decreaseTotalPending(Q, quoteAmount)` fails.
- The X tokens sent in are now **stuck**:
  - They were counted into `totalPendingRewards[X]`,
  - No user can ever trigger `_distributeAssets` with `quote = X` (the pool’s quoteAsset is Q),
  - `skimExcessRewards` cannot withdraw them because it only allows withdrawing `balanceOf - totalPendingRewards` for a given asset, and here `balanceOf[X] == totalPendingRewards[X]`.

This is a protocol-breaking accounting invariant violation triggered by any permissionless user for any existing pool by simply calling `addRewards` with the correct `launchAsset` and an arbitrary `token1` as the fake quote token.

Mitigation direction:
- In `addRewards`, enforce that for whichever token resolves as `launchAsset`, the other token is exactly the configured `rs.quoteAsset`:
  ```solidity
  if (rs.quoteAsset == address(0)) { ... } else {
      require(token1 == rs.quoteAsset, "Invalid quote token");
  }
  ```
  (and symmetrically when the pool is found via `token1`).
- Alternatively, derive `quoteAsset` solely from `rs.quoteAsset` and ignore the caller-provided other token for accounting (but still use it for `safeTransferFrom`), reverting if they mismatch.

 ### Static Signals
permissionless addRewards modifies reward pool state, no check that token1 == rs.quoteAsset, pendingQuoteRewards updated using arbitrary quoteAsset, totalPendingRewards keyed by wrong asset, _decreaseTotalPending() underflows and reverts on claim
 ### Assets at Risk
launchpad rewards, user staking rewards for affected launchAsset, incorrectly added quote reward tokens stuck in contract
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: RewardsTrackerLib.stake / unstake / claim / totalAccRewards

 ### Title
Unchecked uint256→uint96/uint128 casts in RewardsTracker can corrupt reward accounting
 ### Description/Code Snippet
The `RewardsTrackerLib` library used by `Distributor` relies heavily on downcasts from `uint256` to `uint96` and `uint128` **without any range checks**. If the accumulated rewards or per-share indices grow large enough, these casts can truncate high bits and corrupt user reward debt and share tracking, violating core accounting invariants.

Key struct and fields:

```solidity
struct UserRewardData {
    uint96 shares;            // User's current share count
    uint96 baseRewardDebt;    // Used to calculate base token rewards owed
    uint96 quoteRewardDebt;   // Used to calculate quote token rewards owed
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

uint128 public constant PRECISION_FACTOR = 1e12;
```

Problematic casts (examples):

```solidity
function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    UserRewardData storage userData = self.userRewards[user];
    uint256 existingShares = uint96(userData.shares);
    ...
    // Update user shares
    userData.shares += newShares;
    self.totalShares += newShares;

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
    // Update reward debts
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
}

function claim(RewardPoolData storage self, address user)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);
    ...
    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}
```

`totalAccRewards` itself is:

```solidity
function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}
```

Since `accBaseRewardPerShare` / `accQuoteRewardPerShare` are `uint256` and monotonically increasing over time as more rewards are added, `totalAccRewards` can grow arbitrarily large (limited only by `uint256`). In high-usage or long-lived pools, it is realistic for `totalAccRewards` to exceed `2**96 - 1`.

When this happens, the assignments like:

```solidity
userData.baseRewardDebt = uint96(totalAccRewards(...));
```

will silently truncate the upper 160 bits, effectively wrapping the stored `baseRewardDebt` around modulo `2**96`. Subsequent reward calculations:

```solidity
baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
```

will then:
- Underestimate or overestimate the user’s pending rewards,
- Potentially cause extremely large `baseAmount` values (if `userData.baseRewardDebt` wrapped to a small number while `totalAccBaseRewards` is large),
- Or even underflow/overflow scenarios that are then only partially guarded by higher-level checks in `Distributor._decreaseTotalPending`.

This kind of silent truncation is a classic source of **reward/accounting corruption**:
- A large long-term staker (or a pool with high reward rate) could cross the `2**96` threshold in accumulated rewards, at which point their stored debts become inconsistent.
- Depending on how and when these overflows occur, either the user can overclaim significantly more than they should, or they can be permanently underpaid, and global `totalPendingRewards` can desynchronize from the actual sum of per-user entitlements.

Because the code relies on these downcasts in multiple locations with no safety checks or explicit upper bounds on `acc*RewardPerShare`, this matches the **UnsafeAssembyTypeCasts** pattern and is tightly related to potential **AccountingInvariantViolation** issues for reward distribution.
 ### Static Signals
uint256 to uint96 downcasts without range checks, accumulator per share stored as uint256 while debts are uint96, reward math can grow unbounded over time
 ### Assets at Risk
rewards, reward accounting invariants, user balances
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.claimRewards

 ### Title
Reward debt stored as uint96 can overflow and corrupt rewards accounting
 ### Description/Code Snippet
The rewards tracking library used by the Distributor stores per-user reward debt in `uint96`, while computing accumulated rewards in full `uint256` precision. This can overflow the debt fields and break the accounting invariants, potentially causing users to lose rewards or rendering claims impossible.

Key code (RewardsTrackerLib):

```solidity
struct UserRewardData {
    uint96 shares; // User's current share count
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}
...
function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}
...
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
    // Update reward debts
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

`totalAccRewards` returns a `uint256` that grows with both `shares` and `accRewardsPerShare`. There is no explicit upper bound enforced on either:
- `shares` is `uint96` per user but can be large (up to ~7.9e28), and `totalShares` can be similarly large.
- `accBaseRewardPerShare` and `accQuoteRewardPerShare` are `uint256` and increase whenever new rewards are added: 

```solidity
if (self.pendingBaseRewards > 0) {
    accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
}
```

Over time, with many reward additions or very large reward amounts, `totalAccRewards(shares, accRewardsPerShare)` can exceed `2**96 - 1`. When this is cast to `uint96`, it silently truncates the upper bits, effectively wrapping around.

Consequences:
- The stored `baseRewardDebt`/`quoteRewardDebt` can wrap far below the true cumulative reward index. On subsequent `claim()` or `unstake()`, the pending amount is computed as `totalAccRewards - rewardDebt`. If `rewardDebt` has wrapped, this difference can become extremely large or even underflow depending on future evolution and intermediate updates.
- In the Distributor, actual transfers are guarded by `totalPendingRewards[asset]`:

```solidity
function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked { totalPendingRewards[asset] -= amount; }
}
```

If the library overstates `baseAmount`/`quoteAmount` due to wrapped debt, `_decreaseTotalPending` can revert (DoS for that user’s claim) or, if there is enough balance from massive donations, incorrectly allow extraction of more than economically intended. In the opposite direction, if the wrap pushes `rewardDebt` far above the true cumulative index before the cast, users may see `baseAmount`/`quoteAmount` shrink toward zero and be unable to claim their fair share.

This is a classic accounting invariant violation: the combination of unbounded 256-bit accumulators and 96-bit debt storage without overflow checks means the internal invariant that `debt` approximates cumulative per-share rewards for that user can be broken once rewards and/or shares are large enough. Since `addRewards` is permissionless in `Distributor`, a malicious or careless participant can push the system toward these edge values by donating very large reward amounts over time.
 ### Static Signals
rewardDebt stored in uint96, totalAccRewards uses uint256 multiplication, no bounds check on cast uint256 -> uint96, accRewardPerShare grows unbounded over lifetime, permissionless addRewards can push state toward overflow
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor._increaseTotalPending / _decreaseTotalPending / skimExcessRewards

 ### Title
FoT/rebasing tokens can desync Distributor’s pending-rewards accounting from actual balances
 ### Description/Code Snippet
The Distributor maintains `totalPendingRewards[asset]` as metadata to reflect the aggregate rewards owed for each token. That metadata is increased when rewards are added and decreased when rewards are distributed:

```solidity
mapping(address => uint256) public totalPendingRewards;

function _increaseTotalPending(address asset, uint256 amount) internal {
    unchecked {
        totalPendingRewards[asset] += amount;
    }
    emit TotalPendingRewardsIncreased(asset, amount);
}

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];

    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();

    unchecked {
        totalPendingRewards[asset] -= amount;
    }

    emit TotalPendingRewardsDecreased(asset, amount);
}
```

The implicit invariant is that for any ERC‑20 `asset` used as a reward token:

`asset.balanceOf(address(this)) >= totalPendingRewards[asset]`.

However, due to the FeeOnTransfer assumption in `addRewards` (see previous pattern), plus the lack of balance‑based sanity checks anywhere else, `totalPendingRewards[asset]` can grow larger than the actual ERC‑20 balance when:

* The reward token is fee‑on‑transfer / taxed, and the contract is credited less than the nominal `amount` used to bump `_increaseTotalPending` and the internal reward indices.
* The reward token is rebasing downward, so `asset.balanceOf(address(this))` shrinks at some later time while `totalPendingRewards[asset]` remains unchanged.

Consequences of this desync include:

* **Reward distribution DoS** – Reward amounts computed by `RewardsTrackerLib` rely on the pending totals/indices, not on actual balances. If the library instructs `_distributeAssets` to transfer more tokens than the contract holds, `safeTransfer` will revert, blocking that claim and potentially all future claims for that asset.
* **Skim function unusable** – `skimExcessRewards` assumes the invariant to compute skimmable donations:

```solidity
function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();

    asset.safeTransfer(msg.sender, amount);
}
```

If `totalPendingRewards[asset] > asset.balanceOf(address(this))`, the subtraction `asset.balanceOf(address(this)) - totalPendingRewards[asset]` underflows and reverts before even reaching the `>` comparison, meaning `skimExcessRewards` can never succeed again for that asset. This also indicates that the system’s metadata is no longer self‑consistent with actual balances.

This is an **accounting invariant violation** driven by the same root cause as the FoT assumption: rewards accounting is advanced based on nominal `amount` inputs, with no mechanism to reconcile against real token balances or adjust `totalPendingRewards`/per‑share indices when the underlying token behavior diverges from plain ERC‑20 semantics.
 ### Static Signals
totalPendingRewards tracked separately from real ERC20 balances, no invariant check that balanceOf >= totalPendingRewards, rewards indices depend on nominal amounts, not balance deltas, skimExcessRewards uses balance - totalPendingRewards without underflow guard
 ### Assets at Risk
rewards, excess donations, protocol accounting invariants
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
AMM swap depends on external Distributor callback that can revert and DoS pool
 ### Description/Code Snippet
In `GTELaunchpadV2Pair`, the core `_update` function calls into an external `IDistributor` without any failure isolation. This call is reached on the critical swap path and can cause a full revert of swaps if the Distributor misbehaves or is misconfigured.

Relevant code in `GTELaunchpadV2Pair`:
```solidity
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 _reserve0,
    uint112 _reserve1,
    uint112 newLaunchpadFee0,
    uint112 newLaunchpadFee1
) private {
    ...
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
        accruedLaunchpadFee0 = totalLaunchpadFee0;
        accruedLaunchpadFee1 = totalLaunchpadFee1;
        emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
    }
    ...
}

function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        address _token0 = token0;
        address _token1 = token1;
        address distributor = launchpadFeeDistributor;

        if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
        if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

        emit LaunchpadFeesCollected(fee0, fee1);
    }
}
```

`_update` is called from `swap`, `mint`, `burn`, and `sync`, so any revert in `IDistributor.addRewards` will revert these core AMM operations. The Distributor can realistically revert in multiple cases:
- `RewardsDoNotExist` if no reward pool exists yet for the token pair (e.g., launchpad/Distributor misordering initialization).
- `NoSharesToIncentivize` if `rs.totalShares == 0` in `Distributor.addRewards`, meaning rewards are being added before any staking shares exist.
- Any other unforeseen change in Distributor logic that reverts `addRewards`.

Because there is no `try/catch` and no way to bypass fee distribution when it fails, *the entire AMM pair can be griefed/DoSed by a misconfigured or reverting Distributor*, even though swaps themselves do not directly depend on Distributor state. This tightly couples pool liveness to external callback success.

This matches the GriefableCallbacks pattern: a critical protocol flow (`swap`) forwards all gas to an untrusted (or at least separately deployed) contract without error isolation, so failures there can brick the pool. Even if governance is trusted, a purely configuration/ordering mistake (e.g., deploying a pair with `launchpadFeeDistributor` pointing to a Distributor that has no reward pool for these tokens yet) could render the AMM unusable until corrected, impacting users’ ability to trade or exit positions.
 ### Static Signals
external call to IDistributor.addRewards in core accounting function, no try/catch around external hook, callback success required for swap/mint/burn to complete, forwards essentially all gas to external contract
 ### Assets at Risk
AMM liquidity usability, traders’ ability to swap/exit, launchpad fee flow
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: Distributor.claimRewards

 ### Title
Reentrancy risk in Distributor reward payouts via untrusted ERC20 callbacks
 ### Description/Code Snippet
The `Distributor` contract sends arbitrary ERC20 tokens to `msg.sender` during reward distribution without any reentrancy guard and before fully finalizing all higher-level protocol flows, which allows malicious tokens (e.g. ERC777-like or tokens with reentrant `transfer`/`transferFrom` hooks) to reenter sensitive paths.

Key code paths:

```solidity
function claimRewards(address launchAsset)
    external
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

    (baseAmount, quoteAmount) = rs.claim(msg.sender);

    _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount);  // external call to untrusted token
    }

    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(msg.sender, quoteAmount); // external call to untrusted token
    }
}
```

`SafeTransferLib` performs a low-level call to the token contract; if that token is malicious or ERC777-style, it can call back into `Distributor` (e.g. `claimRewards`, `addRewards`, `increaseStake`, or `decreaseStake` via the launchpad path) while the outer call stack is still executing.

While the internal `RewardsTrackerLib` updates user reward debts before `_distributeAssets`, so straight double-claiming of the same reward is mitigated, this pattern still presents:

* A realistic **reentrancy surface** across multiple assets and functions.
* Potential for **griefing / invariant-breaking sequences** when combined with `addRewards()` reentrancy (where `_increaseTotalPending` and pool pending reward state are updated before the donation `safeTransferFrom` finishes). For example, inside a malicious token's callback during `addRewards`, an attacker can trigger `claimRewards` and `_distributeAssets` to pull out rewards based on newly incremented `pendingBaseRewards` and `totalPendingRewards` **before** the donated tokens have actually arrived, leading to transient mismatches between `totalPendingRewards[asset]` and the real token balance and making accounting-dependent checks (like `skimExcessRewards`) fragile and prone to revert or mis-account.

Given this contract is designed as a central reward escrow that should be robust to arbitrary ERC20 behaviors (and is already using a custom accounting layer `totalPendingRewards`), the lack of any `nonReentrant`/reentrancy pattern and the direct external calls in `_distributeAssets` is a plausible reentrancy vulnerability pattern that warrants deeper analysis in later phases.
 ### Static Signals
external token.transfer before end of function, no nonReentrant guard on reward payout, uses arbitrary ERC20 via SafeTransferLib
 ### Assets at Risk
rewards, accounting invariants (totalPendingRewards vs actual balances)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.skimExcessRewards

 ### Title
Rounding in RewardsTracker desynchronizes totalPendingRewards and locks excess rewards
 ### Description/Code Snippet
The Distributor maintains a `totalPendingRewards[asset]` mapping that is intended to track the total amount of claimable rewards that must remain in the contract and cannot be skimmed by admin:

```solidity
mapping(address => uint256) public totalPendingRewards;

function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}
```

Rewards themselves are accounted in `RewardPoolData` via `RewardsTrackerLib`. When new rewards are added through `Distributor.addRewards`, both the rewards pool and `totalPendingRewards` are incremented:

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
```

However, the per-share reward accounting in `RewardsTrackerLib` uses integer division in a way that can permanently drop some reward units on every `addRewards` call, while `totalPendingRewards` still counts the full amount:

```solidity
uint128 public constant PRECISION_FACTOR = 1e12;

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}

function getAccRewardsPerShare(RewardPoolData storage self)
    internal
    view
    returns (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare)
{
    uint96 totalShares = self.totalShares;
    if (totalShares == 0) return (self.accBaseRewardPerShare, self.accQuoteRewardPerShare);

    accBaseRewardsPerShare = self.accBaseRewardPerShare;
    accQuoteRewardsPerShare = self.accQuoteRewardPerShare;

    if (self.pendingBaseRewards > 0) {
        accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
    }

    if (self.pendingQuoteRewards > 0) {
        accQuoteRewardsPerShare += ((self.pendingQuoteRewards * PRECISION_FACTOR) / uint128(totalShares));
    }
}

function update(RewardPoolData storage self)
    internal
    returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
{
    (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    if (self.pendingBaseRewards > 0) {
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        delete self.pendingBaseRewards; // full amount removed from pending
    }

    if (self.pendingQuoteRewards > 0) {
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        delete self.pendingQuoteRewards;
    }
}
```

Because of the two stages of integer division (`pendingRewards * PRECISION_FACTOR / totalShares` and later `shares * accRewardsPerShare / PRECISION_FACTOR`), the sum of all users’ eventual `baseAmount`/`quoteAmount` claims from a given reward tranche can be strictly less than the tranche `amount` passed into `addRewards`. Any remainder caused by truncation is effectively **unreachable** by any user, but `totalPendingRewards[asset]` was increased by the full `amount` and is only ever decreased when rewards are actually paid out:

```solidity
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

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked { totalPendingRewards[asset] -= amount; }
}
```

As a consequence:

* After all stakers have completely exited and claimed all rewards that the `RewardsTrackerLib` logic will ever allocate to them, **`totalPendingRewards[asset]` can remain strictly positive**, representing the accumulated rounding dust from all historical `addRewards` calls.
* These leftover tokens are still physically sitting in the Distributor at `asset.balanceOf(address(this))`, and they are **counted** in `totalPendingRewards[asset]` but **no user has any remaining claim** on them (as all `shares == 0` accounts will see `getPendingRewards` return zero).
* The admin’s `skimExcessRewards` guard compares against `balanceOf - totalPendingRewards`. For this residual dust, we end up with `balanceOf == totalPendingRewards`, so `amount > balanceOf - totalPendingRewards` prohibits any skimming. Those tokens become permanently stuck in the contract: neither users nor admin can withdraw them, and there is no function that can reduce `totalPendingRewards` except via actual claims.

This is a contract-level **accounting invariant violation** between the internal notion of “pending rewards” and the actual claimable rewards in the per-share accounting. While the amounts per addition may seem small, over many reward top-ups and a large user base this discrepancy can accumulate to a non-trivial locked balance.

Impact considerations:

* Assets at risk are the reward tokens themselves: the protocol will be unable to ever recover some portion of contributed rewards.
* The issue is triggered by normal, permissionless use of `addRewards` and the standard reward-claiming lifecycle; no malicious admin behavior is needed.
* A fix would involve either:
  * tracking a separate "distributable" total that mirrors the rounding behavior, or
  * introducing a controlled write-path to reconcile `totalPendingRewards[asset]` downward once no shares remain (e.g., allowing the admin to sweep the rounding dust when `totalShares == 0`).
 ### Static Signals
separate totalPendingRewards mapping, per-share math uses PRECISION_FACTOR, divide-before-multiply in totalAccRewards & getAccRewardsPerShare, pendingBaseRewards deleted without adjusting totalPendingRewards, skim guard uses balanceOf - totalPendingRewards
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair.swap

 ### Title
Untrusted Distributor callback can DoS AMM swaps when rewards shares go to zero
 ### Description/Code Snippet
In `GTELaunchpadV2Pair.swap`, fee distribution to the launchpad Distributor is tightly coupled to the swap flow via an external callback, and any revert in that callback reverts the entire swap. There is no failure isolation (no try/catch or bypass), so AMM swaps can be DoS'ed when the rewards pool for the pair has zero shares.

Relevant flow:

```solidity
function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external lock {
    ...
    (uint112 _reserve0, uint112 _reserve1,) = getReserves();
    ...
    uint256 amount0In = ...;
    uint256 amount1In = ...;
    ...
    {
        uint256 balance0Adjusted = balance0.mul(1000).sub(amount0In.mul(3));
        uint256 balance1Adjusted = balance1.mul(1000).sub(amount1In.mul(3));
        ...
        (uint112 launchpadFee0, uint112 launchpadFee1) =
            launchpadFeeDistributor > address(0) && rewardsPoolActive > 0
                ? _getLaunchpadFees(amount0In, amount1In)
                : (uint112(0), uint112(0));

        _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
    }
    ...
}

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
    ...
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
        accruedLaunchpadFee0 = totalLaunchpadFee0;
        accruedLaunchpadFee1 = totalLaunchpadFee1;
        emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
    }
    ...
}

function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        address _token0 = token0;
        address _token1 = token1;
        address distributor = launchpadFeeDistributor;

        if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
        if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

        emit LaunchpadFeesCollected(fee0, fee1);
    }
}
```

The Distributor callback:

```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    (address launchAsset, address quoteAsset, uint128 launchAssetAmount, uint128 quoteAssetAmount) =
        (token0, token1, amount0, amount1);
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);

    if (rs.quoteAsset == address(0)) {
        rs = RewardsTrackerStorage.getRewardPool(token1);
        if (rs.quoteAsset == address(0)) revert RewardsDoNotExist();
        (launchAsset, quoteAsset, launchAssetAmount, quoteAssetAmount) = (token1, token0, amount1, amount0);
    }

    if (rs.totalShares == 0) revert NoSharesToIncentivize();
    ...
}
```

Key properties:
- `swap` → `_update` → `_distributeLaunchpadFees` → `IDistributor.addRewards` is a mandatory call path when `launchpadFeeDistributor != address(0)` and `rewardsPoolActive > 0` and there are nonzero launchpad fees.
- `addRewards` reverts with `NoSharesToIncentivize()` when the associated reward pool has `totalShares == 0`.

A realistic sequence:
1. During/after bonding, the Launchpad uses `Distributor.increaseStake`/`decreaseStake` (only callable by Launchpad) to manage staking shares. Over time, all accounts can be fully unstaked, causing `RewardPoolData.totalShares` for that asset to become `0`.
2. The Launchpad has **not yet** called `Distributor.endRewards` / `GTELaunchpadV2Pair.endRewardsAccrual()` (so `rewardsPoolActive == 1`, and `launchpadFeeDistributor` remains set).
3. A regular user performs a swap on the AMM pair. Swap fees are computed, `_getLaunchpadFees` returns nonzero `launchpadFee0/1`, and `_update` calls `_distributeLaunchpadFees`.
4. `_distributeLaunchpadFees` calls `Distributor.addRewards(...)`. Because `rs.totalShares == 0`, `addRewards` reverts with `NoSharesToIncentivize()`.
5. This revert bubbles up, reverting `_update` and the entire `swap` call.

Result: **All swaps are reverted** as long as:
- launchpad fees are nonzero for the swap,
- `rewardsPoolActive > 0` and `launchpadFeeDistributor` is configured, and
- the corresponding reward pool’s `totalShares == 0`.

This creates a state-dependent DoS where the AMM becomes unusable until governance/Launchpad calls `endRewards` (which sets `rewardsPoolActive = 0` in the pair and stops fee distribution) or re-establishes staking shares. The failure condition is purely on Distributor’s internal state (`totalShares`), yet it fully blocks core trading functionality in the AMM, and can occur as a natural consequence of all stakers being unwound before `endRewards` is invoked.

Static characteristics matching the **GriefableCallbacks** pattern:
- The pair makes an external call to an untrusted contract (`IDistributor`) without `try/catch`.
- Swap success is **strictly dependent** on this callback succeeding; any revert halts the core protocol action (swaps).
- The callback reads reward-pool state (`totalShares`) that can reach 0 via normal protocol operations, leading to systematic failures even without malicious governance or tokens.

A robust design would isolate fee distribution from the critical swap path (e.g., accrue fees in-pair and let a separate actor harvest them), or wrap the callback in a `try/catch` and degrade gracefully (e.g., accrue but skip distribution) rather than revert swaps.
 ### Static Signals
external callback to IDistributor.addRewards without try/catch, swap core flow depends on Distributor state (totalShares), callback revert (NoSharesToIncentivize) reverts entire swap, rewardsPoolActive and launchpadFeeDistributor gate but do not guard totalShares==0
 ### Assets at Risk
AMM liquidity usability, trader swaps (availability DoS), launchpad fee flow
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: NonStandardERC20Behavior

 ### Relevant Function/Location: Distributor._distributeAssets

 ### Title
Non-standard ERC20 behaviors (rebasing/deflationary) can desync Distributor totalPendingRewards and actual balances
 ### Description/Code Snippet
The `Distributor` relies on `totalPendingRewards[asset]` to model how many tokens are owed to users and what portion of the contract’s balance is *excess* and skimmable. However, it does not account for ERC20s that can change balances without explicit transfers (rebasing, external burns/mints) or that process transfers with dynamic amounts.

Key code paths:

1. Accrual of rewards and pending accounting:
```solidity
function _increaseTotalPending(address asset, uint256 amount) internal {
    unchecked {
        totalPendingRewards[asset] += amount;
    }
    emit TotalPendingRewardsIncreased(asset, amount);
}

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];

    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();

    unchecked {
        totalPendingRewards[asset] -= amount;
    }

    emit TotalPendingRewardsDecreased(asset, amount);
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

2. Owner/admin skimming based on this accounting:
```solidity
function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();

    asset.safeTransfer(msg.sender, amount);
}
```

These mechanisms assume that:
- `totalPendingRewards[asset]` is always in sync with the actual ERC20 token balance held by the Distributor minus donations/excess, and
- the only way for the Distributor’s balance to change is via the controlled `safeTransferFrom` and `safeTransfer` operations in this contract.

With rebasing or otherwise non-standard tokens (e.g. inflationary/deflationary supply changes applied globally or per-account), this assumption fails:
- A **positive rebase** increasing `asset.balanceOf(address(this))` without updating `totalPendingRewards[asset]` makes `skimExcessRewards` see extra balance as "excess" and lets owner/ADMIN_ROLE skim it. This is arguably governance-policed but breaks the intended invariant that user rewards + donations are preserved.
- A **negative rebase** or external burn from the Distributor address can reduce `asset.balanceOf(address(this))` below `totalPendingRewards[asset]`, so later `_distributeAssets` transfers may revert (lack of balance), effectively locking future claims or stake/unstake flows.

Additionally, `addRewards` credits `pendingBaseRewards`/`pendingQuoteRewards` and `totalPendingRewards` based solely on the `amount` argument without checking actual received balance, compounding the effect for fee-on-transfer or deflationary tokens.

Overall, the reward accounting logic is tightly coupled to standard, non-rebasing ERC20 semantics; any non-standard behavior can cause reward desync, stuck claims, or unintended skimming.
 ### Static Signals
relies on totalPendingRewards as invariant vs asset.balanceOf, no handling for rebasing tokens, no detection of external mint/burn on Distributor balance, skimExcessRewards uses balanceOf(address(this)) - totalPendingRewards[asset] as excess
 ### Assets at Risk
rewards, user pending rewards in Distributor, donated rewards subject to skim logic
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: RewardsTrackerLib.stake / unstake / claim / addBaseRewards / addQuoteRewards

 ### Title
Reward accounting can overflow/truncate due to unsafe downcasts in RewardsTrackerLib
 ### Description/Code Snippet
The rewards-distribution library uses multiple unchecked downcasts from uint256 to uint96/uint128 when storing user reward debt and pending rewards. If the accumulated rewards per share grow large enough (either via a long-running pool or very large reward injections), these downcasts can silently truncate high bits and corrupt accounting.

Key code (RewardsTrackerLib):

```solidity
function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();
    UserRewardData storage userData = self.userRewards[user];
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
    (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();
    UserRewardData storage userData = self.userRewards[user];
    ...
    // Update reward debts
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
}

function claim(RewardPoolData storage self, address user)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    UserRewardData storage userData = self.userRewards[user];
    uint256 shares = uint256(userData.shares);
    ...
    uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
    uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

    // Update reward debts
    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}

function addBaseRewards(RewardPoolData storage self, address baseAsset, uint128 amount) internal {
    self.pendingBaseRewards += amount; // uint128 += uint128
    emit BaseRewardsAdded(baseAsset, amount);
}

function addQuoteRewards(..., uint128 amount) internal {
    self.pendingQuoteRewards += amount; // uint128 += uint128
    ...
}

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}
```

Static issues:
- `totalAccRewards` returns a `uint256` that can exceed 2^96–1 or 2^128–1 in extreme cases (large `shares` and/or large `accRewardsPerShare`).
- This value is then cast to `uint96` (`baseRewardDebt`, `quoteRewardDebt`) and compared as `uint128` in subtract operations, without any bounds checks.
- `pendingBaseRewards`/`pendingQuoteRewards` are `uint128`; repeated large calls to `addBaseRewards` / `addQuoteRewards` can overflow these too, since `+=` uses unchecked arithmetic in Solidity 0.8 only for smaller types packed into a storage slot (but here they are standalone `uint128` in the slot, and the compiler will revert on overflow inside a single tx). However, the bigger risk is the 96-bit truncation of `totalAccRewards`.

Impact sketch:
- Over time or via a large injected reward, `accBaseRewardPerShare` and `accQuoteRewardPerShare` can grow large.
- For a user with large `shares`, `totalAccRewards(shares, accRewardsPerShare)` can exceed 2^96–1.
- When this is cast to `uint96`, the high bits are discarded, effectively resetting `baseRewardDebt`/`quoteRewardDebt` modulo 2^96.
- Subsequent `claim` / `stake` / `unstake` calls use these truncated debts in subtractions such as
  `baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);`
  leading to incorrect rewards. In some wraparound scenarios this can:
  - make `baseAmount`/`quoteAmount` much larger than intended (overpaying some users at the expense of the pool), or
  - underpay users (some rewards effectively lost in accounting),
  depending on the relative values.

Because this library underpins all rewards in `Distributor` and is fed by permissionless `addRewards`, an attacker willing to deposit very large reward amounts could deliberately push the per-share indexes into ranges where these casts truncate, then claim in a sequence that harvests more than their fair share or causes systemic drift between `totalPendingRewards` and actual owed rewards.
 ### Static Signals
downcasts uint256->uint96 without range checks, downcasts uint256->uint128 without validation in arithmetic, reward index and debt stored in narrow types, totalAccRewards uses unbounded 256-bit multiplication before cast
 ### Assets at Risk
rewards, launchpad fee distributions, user reward balances
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: RewardsTrackerLib.stake/unstake/claim

 ### Title
Reward accounting can overflow 96-bit debts and break pending rewards invariant
 ### Description/Code Snippet
RewardsTrackerLib stores `baseRewardDebt` and `quoteRewardDebt` as `uint96`, but all reward index and pending reward math is done in full `uint256` precision, with no explicit bounds on growth. The library repeatedly casts large `uint256` values down to `uint96` without checks:

```solidity
struct UserRewardData {
    uint96 shares; // User's current share count (up to ~7.9e28)
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}
...
function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    // Update reward debts
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
}
...
function unstake(RewardPoolData storage self, address user, uint96 removeShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    // Update reward debts
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
}
...
function claim(RewardPoolData storage self, address user)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

    // Update reward debts
    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}
```

`totalAccRewards` returns a `uint256`:

```solidity
function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}
```

`accBaseRewardPerShare` / `accQuoteRewardPerShare` can grow over many reward additions:

```solidity
if (self.pendingBaseRewards > 0) {
    accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
}
```

There is no cap on how large `acc*RewardPerShare` or `totalAccRewards` can become other than the 128‑bit `pending*Rewards` and 96‑bit `totalShares`. Over a long enough lifetime or with large token amounts, `totalAccRewards` can exceed 2^96-1, so truncating it to `uint96` will silently wrap and corrupt `baseRewardDebt`/`quoteRewardDebt`.

Once this happens, the invariant that `totalPendingRewards[asset]` in `Distributor` matches the sum of claimable rewards breaks: pending amounts calculated as `totalAccRewards - debt` will be incorrect (potentially huge or zero), yet `totalPendingRewards` in `Distributor` is only adjusted by the public flows (`addRewards` / `_distributeAssets`). This can:

* Allow some users to over-claim and drive `totalPendingRewards[asset]` negative relative to what the contract actually owes (guarded only by `ClaimAmountExceedsTotalPendingRewards`), or
* Permanently strand rewards that can no longer be claimed because debts have wrapped.

Because this is a core pro-rata distribution primitive used by `Distributor.increaseStake`, `decreaseStake`, and `claimRewards`, a 96-bit overflow in debts constitutes an accounting invariant violation that can leak or lock rewards over the system's lifetime.
 ### Static Signals
rewardDebt stored as uint96, totalAccRewards computed as uint256, unchecked narrowing cast from uint256 to uint96, no upper bound on accRewardPerShare growth, pending rewards rely on difference of potentially wrapped values
 ### Assets at Risk
rewards, user balances, totalPendingRewards accounting in Distributor
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Rewards accounting assumes 1:1 ERC20 transfers (fee-on-transfer/rebasing tokens break Distributor invariants)
 ### Description/Code Snippet
The `Distributor` contract assumes that the amount passed as `amount0/amount1` to `addRewards` is exactly the amount of tokens that will be received.

Relevant code:
```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    (address launchAsset, address quoteAsset, uint128 launchAssetAmount, uint128 quoteAssetAmount) =
        (token0, token1, amount0, amount1);
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);

    if (rs.quoteAsset == address(0)) {
        rs = RewardsTrackerStorage.getRewardPool(token1);

        if (rs.quoteAsset == address(0)) revert RewardsDoNotExist();

        (launchAsset, quoteAsset, launchAssetAmount, quoteAssetAmount) = (token1, token0, amount1, amount0);
    }

    if (rs.totalShares == 0) revert NoSharesToIncentivize();

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

function _increaseTotalPending(address asset, uint256 amount) internal {
    unchecked {
        totalPendingRewards[asset] += amount;
    }
    emit TotalPendingRewardsIncreased(asset, amount);
}
```

`totalPendingRewards[asset]` is increased by the *input parameter* `launchAssetAmount` / `quoteAssetAmount` before or without verifying that the contract actually received exactly that many tokens. No `balanceBefore`/`balanceAfter` delta is used.

If the reward token is:
- fee-on-transfer / deflationary (contract receives less than `amount`), or
- rebasing / elastic supply (balance changes asynchronously), or
- has transfer hooks that siphon part of the amount,

then `totalPendingRewards[asset]` and the internal `pendingBaseRewards` / `pendingQuoteRewards` in `RewardPoolData` can overstate what is actually held by the `Distributor`.

Later, when users call `claimRewards`, `increaseStake`, or `decreaseStake`, the library computes rewards from these inflated pending values and `Distributor._distributeAssets` sends tokens:
```solidity
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

`_decreaseTotalPending` guards only against `amount > totalPendingRewards[asset]`, not `amount > actualBalance`. If `totalPendingRewards[asset]` has been incremented beyond what was actually received (e.g. due to transfer fees), this invariant can break negatively later when the token balance is insufficient to pay users, causing:
- user-facing reverts on claims once `balanceOf(Distributor)` is depleted even though `totalPendingRewards[asset]` still claims there is more, or
- skewed reward distribution if some users manage to claim early (while balance is still sufficient) and others fail later.

The issue is realistic because the `GTELaunchpadV2Pair` uses arbitrary ERC20 tokens for `token0`/`token1`, and `addRewards` is also permissionless – any project could choose fee-on-transfer or rebasing tokens as launch assets or quote assets.

This matches the `FeeOnTransferAssumption` pattern: accounting is based on the transfer parameter `amount` rather than the actual balance change, with no mitigation for non-standard ERC20 behaviors.
 ### Static Signals
uses input amount directly for accounting, no balanceBefore/balanceAfter delta check, assumes transferFrom(amount) == received amount, no handling for fee-on-transfer/rebasing tokens
 ### Assets at Risk
rewards, user pending rewards, launchpad fee-derived incentives
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: RewardsTrackerLib.stake / unstake / claim

 ### Title
Reward debt downcasts to uint96 can overflow and corrupt rewards accounting
 ### Description/Code Snippet
The rewards tracking library used by `Distributor` stores per-user reward debts as `uint96`, but computes them in `uint256` and then **unchecked downcasts** to `uint96` without any range validation. If `totalAccRewards(...)` exceeds `type(uint96).max`, the value will silently wrap/truncate, breaking the core rewards accounting and enabling over/under-payment.

Relevant code (from `RewardsTrackerLib` in `RewardsTracker.sol`):

```solidity
struct UserRewardData {
    uint96 shares; // User's current share count (up to ~7.9e28)
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}
...
function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
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
    // Update reward debts
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

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR; // PRECISION_FACTOR = 1e12
}
```

**Why this is dangerous in this system:**

- `totalAccRewards` is a `uint256` that can grow with successive `addRewards` calls and large `shares`. It is then cast to `uint96` when stored in `baseRewardDebt` / `quoteRewardDebt`.
- If `totalAccRewards > type(uint96).max (~7.9e28)`, the cast truncates the high bits, effectively changing the user’s recorded debt to a much smaller value.
- On the next `claim` / `stake` / `unstake`, pending rewards are computed as:
  ```solidity
  baseAmount = totalAccRewards(...) - userData.baseRewardDebt;
  ```
  With a truncated `baseRewardDebt`, this difference can become **much larger** than the true pending amount, causing the user to be overpaid, or in other overflow regimes underpaid.
- Since `Distributor` trusts the library’s output and uses it to update `totalPendingRewards` and to transfer actual tokens in `_distributeAssets`, this mis-accounting directly affects the reward pool’s solvency and `totalPendingRewards[asset]` invariant.

This can realistically happen for:
- High-supply tokens (e.g., 10–100+ billion tokens with 18 decimals), where cumulative rewards and shares can push `totalAccRewards` beyond `2^96-1`.
- Long-lived pools with repeated `addRewards` over time.

Once overflowed, the bug is persistent for that user: their future reward calculations remain based on incorrect debts, potentially allowing:
- Extraction of more rewards than actually allocated (pool insolvency / other users short-changed).
- Or loss of rewards if the truncation goes in the opposite direction.

Because `uint96` is chosen for gas/packing, this constitutes a classic unsafe downcast that breaks core accounting invariants for sufficiently large values.

Mitigation directions (for later phases): enforce upper bounds such that `totalAccRewards` never exceeds `type(uint96).max`, or store debts in a wider type (e.g. `uint128` / `uint256`) consistent with the maximum theoretical reward growth.

 ### Static Signals
downcasts without range checks, uint256→uint96 cast of totalAccRewards, rewardDebt stored in smaller type than computation
 ### Assets at Risk
rewards, user balances in Distributor reward pools
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: RewardsTrackerLib.stake / unstake / claim

 ### Title
Reward debt uint96 truncation can break rewards accounting and lock claims
 ### Description/Code Snippet
The rewards tracking library used by `Distributor` stores user reward debts in `uint96`, while the accumulated reward indices and totals are `uint256` with no explicit upper bounds. This can cause reward-debt truncation (wraparound), breaking the accounting invariant between per-user rewards and `totalPendingRewards` and potentially making all further claims revert.

Key structs (from `RewardsTrackerLib`):
```solidity
struct UserRewardData {
    uint96 shares; // User's current share count (up to ~7.9e28)
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}

struct RewardPoolData {
    uint96 totalShares; // Sum of all user shares
    address quoteAsset;
    uint128 pendingBaseRewards;
    uint128 pendingQuoteRewards;
    uint256 accBaseRewardPerShare; // scaled by 1e12
    uint256 accQuoteRewardPerShare; // scaled by 1e12
    mapping(address => UserRewardData) userRewards;
}
```

In `stake`, `unstake`, and `claim`, the library computes a 256-bit cumulative reward value and then casts it down to `uint96` for storage:
```solidity
function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();
    UserRewardData storage userData = self.userRewards[user];
    uint256 existingShares = uint96(userData.shares);
    ...
    userData.shares += newShares;
    self.totalShares += newShares;

    // POTENTIAL OVERFLOW/TRUNCATION
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
}

function unstake(RewardPoolData storage self, address user, uint96 removeShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();
    UserRewardData storage userData = self.userRewards[user];
    ...
    uint256 existingShares = uint256(userData.shares);
    ...
    // POTENTIAL OVERFLOW/TRUNCATION
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
}

function claim(RewardPoolData storage self, address user)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    UserRewardData storage userData = self.userRewards[user];
    uint256 shares = uint256(userData.shares);
    ...
    (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

    uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
    uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

    // POTENTIAL OVERFLOW/TRUNCATION
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

There are no guards ensuring `totalAccRewards(...) <= type(uint96).max`. Over a long period or with large reward injections (e.g., large swap volumes feeding fees into `GTELaunchpadV2Pair` → `Distributor.addRewards`), `accBaseRewardPerShare` and `accQuoteRewardPerShare` can grow such that `totalAccRewards` exceeds `2^96 - 1`. When this happens:

* The cast to `uint96` silently truncates the high bits of `totalAccRewards`, storing a much smaller `baseRewardDebt` / `quoteRewardDebt`.
* On the next `claim` / `stake` / `unstake`, `baseAmount` and `quoteAmount` are computed as 256-bit differences between a large `totalAccRewards` and a truncated (wrapped) 96-bit debt, producing an inflated payout amount that does not correspond to actual rewards funded.

`Distributor` then tries to settle these amounts against its global `totalPendingRewards` tracker:
```solidity
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

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked {
        totalPendingRewards[asset] -= amount;
    }
}
```

If the truncated debt caused an overly large `baseAmount` or `quoteAmount`, two bad outcomes are possible:

1. **Global rewards DoS / lockup**: Once accumulated indices grow sufficiently, some user’s next claim will compute a massive `baseAmount`/`quoteAmount` that exceeds `totalPendingRewards[asset]`. `_decreaseTotalPending` will revert with `ClaimAmountExceedsTotalPendingRewards()`, causing the entire `claimRewards` / `increaseStake` / `decreaseStake` call to revert. From that point, *any* attempt to touch rewards for that pool (including legitimate, smaller claims) may revert, effectively bricking withdrawals of already-funded rewards.

2. **Potential mis-accounting between `RewardsTracker` state and actual token balances**: Even before hitting the revert, the truncation breaks the intended invariant that `totalPendingRewards[asset]` equals the sum of all users’ claimable amounts implied by `acc*PerShare` and `baseRewardDebt`/`quoteRewardDebt`. This divergence complicates reasoning about safety and may allow certain sequences of large reward injections + claims to extract more or less than fair share of rewards, depending on how the truncation interacts with per-user states.

Because there is no explicit cap on `acc*PerShare` or `totalAccRewards`, an attacker (or normal high-volume usage over time) can realistically drive `totalAccRewards` past the `uint96` limit by:

* repeatedly feeding large rewards into a pool (via swaps in the associated `GTELaunchpadV2Pair` or direct `Distributor.addRewards`), while
* keeping `totalShares` moderate (so per-share rewards grow quickly).

Eventually, the accumulated per-share index grows enough that `shares * acc / 1e12 > 2^96 - 1` for some user, triggering truncation on the next `stake` / `unstake` / `claim` touching that user.

This matches the `AccountingInvariantViolation` pattern: a core accounting index (`baseRewardDebt` / `quoteRewardDebt`) can silently wrap, breaking the relationship between global pending rewards and individual user entitlements and plausibly leading to stuck rewards or mis-distributed payouts.
 ### Static Signals
reward debts stored as uint96, accumulated rewards and totals are uint256 without upper bound, casts totalAccRewards(...) to uint96, no check that totalAccRewards <= type(uint96).max, global totalPendingRewards used as hard cap and can cause revert on oversized claim
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
UniswapV2 LP token permit() accepts malleable signatures (no s/v validation)
 ### Description/Code Snippet
The UniswapV2-style LP token used by GTELaunchpadV2Pair implements EIP-2612-like permits but does not enforce standard secp256k1 malleability protections on the signature parameters `v` and `s`.

In `UniswapV2ERC20.permit`:

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

There is **no check** that:
- `s <= secp256k1n/2` (low-s requirement), and
- `v` is in the canonical set {27, 28} (or a normalized {0,1} → {27,28}).

This means an attacker or any holder of a valid signature can produce an alternative, mathematically distinct but equally valid `(v, r, s')` for the same `digest`, exploiting ECDSA signature malleability. While the nonce is correctly incremented and prevents *on-chain* replay in this contract, the lack of canonicalization breaks common EIP-2612 assumptions and can:

- Undermine off-chain or cross-contract replay protections that assume a one-to-one mapping between a logical permit and a unique `(v,r,s)` triple.
- Break integrations or relayers that try to pre-verify or index signatures under the standard low-s requirement (e.g. many wallets and tooling libraries expect EIP-2-compliant, non-malleable signatures).

Given this LP token is intended to be integrated into routers/aggregators and possibly third-party systems, accepting malleable signatures is a non-compliant and potentially exploitable deviation from best practices around `permit`.
 ### Static Signals
ecrecover used directly, no s <= secp256k1n/2 check, v not validated to 27/28, EIP-2612-style permit on LP token
 ### Assets at Risk
LP token approvals, integrations relying on permit, user funds controlled via permit-based allowances
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

