## Verified Patterns Found: 9

## Verified Patterns Found in following Categories:

- UnsafeAssembyTypeCasts
- FeeOnTransferAssumption
- AccountingInvariantViolation
- PrecisionDriftAccumulation
- AccessControlOrAuthByPass



## Summary of Patterns

Stake/unstake route user rewards to launchpad (msg.sender) instead of staker

Distributor reward accounting assumes 1:1 token transfers (fee-on-transfer / rebasing tokens)

Reward accounting desync can lock rewards or misalign pending vs claimable amounts

Rounding in RewardsTracker permanently locks a portion of deposited rewards

Unchecked uint256→uint96 downcasts in reward debts can corrupt accounting at high scales

Reward rounding dust becomes permanently locked due to mismatched global accounting

addRewards accepts wrong quote token, breaking rewards accounting and claims

addRewards allows mismatched quote token, breaking reward accounting and locking tokens

AMM swaps can be permanently DOSed when Distributor.totalShares reaches zero

## Patterns



 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.increaseStake / decreaseStake / _distributeAssets

 ### Title
Stake/unstake route user rewards to launchpad (msg.sender) instead of staker
 ### Description/Code Snippet
In `Distributor`, the functions `increaseStake` and `decreaseStake` use `RewardsTrackerLib.stake/unstake` to update a specific user's share balance and compute that user's pending rewards, but then pay those rewards to `msg.sender` (the launchpad contract) instead of the `account` whose rewards are being settled.

Relevant code:

```solidity
function increaseStake(address launchAsset, address account, uint96 shares)
    external
    onlyLaunchpad
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

    (baseAmount, quoteAmount) = rs.stake(account, uint96(shares));
    _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function decreaseStake(address launchAsset, address account, uint96 shares)
    external
    onlyLaunchpad
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

    (baseAmount, quoteAmount) = rs.unstake(account, uint96(shares));
    _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount); // pays msg.sender, not `account`
    }

    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(msg.sender, quoteAmount); // pays msg.sender, not `account`
    }
}
```

And from `RewardsTrackerLib`:

```solidity
function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 existingShares = uint96(userData.shares);
    if (existingShares > 0) {
        baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
        quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
    }
    ...
}

function unstake(RewardPoolData storage self, address user, uint96 removeShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 existingShares = uint256(userData.shares);
    ...
    baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
    quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
    ...
}
```

`baseAmount` and `quoteAmount` returned by `stake`/`unstake` are the *pending rewards for `user`* (i.e., `account`), computed before changing the share count. However, `_distributeAssets` always transfers these rewards to `msg.sender`. In the normal flow, `increaseStake`/`decreaseStake` are only callable by `launchpad` via the `onlyLaunchpad` modifier, so these settled rewards are sent to the launchpad contract instead of to the user.

At the same time, `_distributeAssets` calls `_decreaseTotalPending` for the corresponding asset. `RewardsTrackerLib.stake/unstake` also update `userData.baseRewardDebt`/`quoteRewardDebt` to the new total accumulated value, so from the reward accounting perspective the user's pending rewards are considered fully paid. The global `totalPendingRewards[asset]` has been reduced and the user's per-account `rewardDebt` advanced, but the user never receives the tokens; they go to the launchpad instead.

This breaks the intended invariant that per-user pending rewards in `RewardsTracker` correspond to assets eventually transferred to that user by `Distributor`. Under the documented design ("Distributor – stakes, tracks per-account shares via RewardsTracker and pays out both base & quote rewards"), a user who accumulates rewards and then has their stake adjusted by launchpad loses those accrued rewards to the launchpad contract, with no on-chain path to reclaim them other than front-running with an explicit `claimRewards` call before launchpad adjusts their shares.
 ### Static Signals
rewards computed for `account` but paid to `msg.sender`, RewardsTrackerLib.stake/unstake return pending rewards for user, _distributeAssets has no recipient parameter, _decreaseTotalPending called before transfer
 ### Assets at Risk
user AMM-fee rewards, launchpad rewards pool
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
Distributor reward accounting assumes 1:1 token transfers (fee-on-transfer / rebasing tokens)
 ### Description/Code Snippet
Distributor’s reward accounting assumes that the number of tokens actually received/sent by the contract equals the `amount` argument passed to `safeTransferFrom` / `safeTransfer`. It never checks the real balance delta, which breaks if any reward asset is fee-on-transfer, rebasing, or otherwise non-standard.

Key paths:
- In `addRewards(address token0, address token1, uint128 amount0, uint128 amount1)`:
  - For the chosen `(launchAsset, quoteAsset)` it does:
    - `rs.addBaseRewards(launchAsset, launchAssetAmount);`
    - `_increaseTotalPending(launchAsset, launchAssetAmount);`
    - `launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));`
  - Similarly for `quoteAsset`.
  - `totalPendingRewards[asset]` and the pool’s `pendingBaseRewards` / `pendingQuoteRewards` are incremented by the nominal `amountX` without verifying that the actual token balance increased by the same amount.

- In `_distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount)`:
  - For each nonzero amount it does:
    - `_decreaseTotalPending(asset, amount);`
    - `asset.safeTransfer(msg.sender, amount);`
  - `_decreaseTotalPending` only checks `totalPendingRewards[asset] >= amount` and never checks the token balance.

If a reward asset charges a transfer fee or is rebasing, `totalPendingRewards[asset]` will no longer correspond to real tokens held:
- Example with a deflationary token paying 10 % burn on transfer:
  - `addRewards(..., amount = 100)` increases `totalPendingRewards[asset]` by 100 but the contract actually receives only 90 tokens.
  - Early claimers can still successfully withdraw up to 90 tokens; afterwards `totalPendingRewards[asset]` will be > 0 while the actual token balance is 0 or very small.
  - Subsequent claims will revert inside `safeTransfer` due to insufficient balance, even though `totalPendingRewards[asset]` still suggests rewards are available.
  - `skimExcessRewards` cannot be used to repair this because it computes `asset.balanceOf(this) - totalPendingRewards[asset]`; if this underflows (balance < totalPendingRewards), the call reverts before any corrective action.

This creates a mismatch between internal accounting and real balances for non-vanilla ERC20s, leading to stuck or non-claimable rewards and potential per-asset reward pool breakage. Because `addRewards` is permissionless, any integration that (intentionally or accidentally) configures a fee-on-transfer / rebasing token as a reward asset will hit this edge case.
 ### Static Signals
credits totalPendingRewards[asset] using the input amount, uses SafeTransferLib.safeTransferFrom without balanceBefore/balanceAfter checks, reward pool accounting based on transfer parameter, not actual received amount, _distributeAssets decrements accounting then calls token.transfer without verifying contract balance
 ### Assets at Risk
rewards, user pending rewards for launchpad pools
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.claimRewards

 ### Title
Reward accounting desync can lock rewards or misalign pending vs claimable amounts
 ### Description/Code Snippet
The Distributor tracks global reward obligations per token in `totalPendingRewards[asset]`, while per-user and per-pool rewards are tracked via `RewardsTrackerLib`.

Key points:
- On adding rewards, the Distributor increases the global counter by the full amount:
  `function addRewards(...) external { ... rs.addBaseRewards(launchAsset, launchAssetAmount); _increaseTotalPending(launchAsset, launchAssetAmount); launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount)); ... }`
  `_increaseTotalPending` simply does `totalPendingRewards[asset] += amount;`.
- Per-pool accounting in `RewardsTrackerLib` uses integer math and deletes `pendingBaseRewards` / `pendingQuoteRewards` after converting them to per-share indices:
  `accBaseRewardsPerShare += (self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares);` then `delete self.pendingBaseRewards;` (similar for quote).
- If `self.pendingBaseRewards` is small relative to `totalShares`, the per-share increment can be so small that after scaling back by `PRECISION_FACTOR` each user’s `totalAccRewards(shares, accRewardsPerShare)` rounds down to zero. In that case the full `amount` has been added to `totalPendingRewards[asset]`, but no user can ever accrue a positive `baseAmount` or `quoteAmount` in `stake/unstake/claim`.
- Because `_distributeAssets` is the *only* place where `totalPendingRewards[asset]` is decreased, and it only decreases by `baseAmount`/`quoteAmount` actually paid out:
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
  any reward increments that are fully lost to rounding will permanently inflate `totalPendingRewards[asset]` without ever becoming claimable.
- `skimExcessRewards` uses `asset.balanceOf(address(this)) - totalPendingRewards[asset]` to determine what is skimmable, so these “lost” rewards are also *not* recoverable by the admin and are effectively locked:
  ```solidity
  function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
      if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
      asset.safeTransfer(msg.sender, amount);
  }
  ```
- Additionally, `UserRewardData.baseRewardDebt` and `.quoteRewardDebt` are stored as `uint96`, but computed from potentially large 256-bit values:
  ```solidity
  userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
  userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
  ```
  If cumulative rewards per share grow very large (many reward additions over time, or extreme token amounts), these casts can overflow and wrap, causing `baseRewardDebt`/`quoteRewardDebt` to no longer match the true accumulated index. Later, `baseAmount = totalAccRewards(...) - userData.baseRewardDebt` can become much larger or smaller than the user’s fair share. `_decreaseTotalPending` only checks against the *global* `totalPendingRewards[asset]`, so a miscomputed user claim can consume other users’ rewards or cause future claims to revert with `ClaimAmountExceedsTotalPendingRewards()`.

Together, these behaviors create a realistic risk of breaking the intended invariant that `totalPendingRewards[asset]` reflects the sum of all unclaimed user rewards: some rewards can become permanently unclaimable (locked) while still counted as pending, and in overflow edge cases users can over- or under-claim relative to their fair share, limited only by the global cap.
 ### Static Signals
global totalPendingRewards separate from per-pool accounting, pendingBaseRewards and pendingQuoteRewards deleted after integer division, rewardDebt downcast from uint256 to uint96, skimExcessRewards relies on totalPendingRewards to protect user funds
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: RewardsTrackerLib.update

 ### Title
Rounding in RewardsTracker permanently locks a portion of deposited rewards
 ### Description/Code Snippet
The rewards distribution library RewardsTrackerLib uses fixed-point per-share accounting with PRECISION_FACTOR = 1e12, but drops integer-division remainders and never re-credits them. This can cause a portion of deposited rewards to become permanently unclaimable, while still being counted in totalPendingRewards and the contract's balances.

New rewards are folded into the per-share accumulator via:

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
            delete self.pendingBaseRewards;
        }

        if (self.pendingQuoteRewards > 0) {
            self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
            delete self.pendingQuoteRewards;
        }
    }

Rewards owed to a user are computed with:

    function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
        return (shares * accRewardsPerShare) / PRECISION_FACTOR;
    }

Because both divisions are integer divisions, a remainder from (pendingRewards * PRECISION_FACTOR) / totalShares is discarded. That remainder is not tracked anywhere; pendingBaseRewards and pendingQuoteRewards are set to zero in update(), while totalPendingRewards[asset] has already been incremented by the full deposit amount in Distributor.addRewards(). Over time, this creates a structural mismatch:

- The Distributor's totalPendingRewards[asset] exactly mirrors the total tokens ever deposited for that asset minus the amounts paid out through _distributeAssets.
- However, the sum over all users of claimable rewards derived from acc*RewardPerShare and totalAccRewards is strictly less than the total deposited amount whenever rounding occurs (e.g. small deposits with many shares).

Example: 2 users with 1 share each (totalShares = 2) and a base reward deposit of 1 token.

1. addBaseRewards() is called with amount = 1, and _increaseTotalPending(baseAsset, 1) sets totalPendingRewards[baseAsset] += 1; the contract balance increases by 1.
2. On the next update(), delta per share is:

       deltaAcc = (pendingBaseRewards * PRECISION_FACTOR) / totalShares
                = (1 * 1e12) / 2
                = 500,000,000,000 (floored)

   accBaseRewardPerShare becomes 500e9 and pendingBaseRewards is set to 0.
3. For each user with 1 share, totalAccRewards(1, accBaseRewardPerShare) yields:

       rewards = 1 * 500e9 / 1e12 = 0 (floored)

   So neither user can ever claim any of that 1 token. Since baseAmount will be zero, _decreaseTotalPending is never called, and totalPendingRewards[baseAsset] remains 1 even after all users have claimed their rewards.
4. Because actual token balance tracks totalPendingRewards[asset] precisely (reductions only happen via _decreaseTotalPending together with transfers), the 'dust' stays locked. skimExcessRewards() cannot withdraw it because it only allows withdrawing balance - totalPendingRewards[asset], which is zero in this scenario.

This pattern repeats for any deposit where (pendingRewards * PRECISION_FACTOR) is not exactly divisible by totalShares, causing small but permanent reward leakage into an unclaimable bucket. Over many deposits and many pools, this can accumulate into a non-negligible amount of locked tokens.

Impact:
- A fraction of every reward deposit can become permanently unclaimable due to rounding.
- Neither users nor the owner can ever withdraw these tokens (since they are always counted as pending in totalPendingRewards[asset] and hence never 'excess').
- This breaks the intended accounting invariant that all rewards deposited are either claimable by users or withdrawable by the owner as excess.
 ### Static Signals
rewards-per-share computation uses integer division with floor, pendingBaseRewards and pendingQuoteRewards are deleted after update without tracking division remainder, totalPendingRewards is only decreased by per-user floored payout amounts, skimExcessRewards() uses balance - totalPendingRewards, making dust non-withdrawable
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: RewardsTrackerLib.stake / unstake / claim

 ### Title
Unchecked uint256→uint96 downcasts in reward debts can corrupt accounting at high scales
 ### Description/Code Snippet
RewardsTrackerLib stores each user's reward debt in uint96, but computes it from a uint256 intermediate without any bounds check:

    struct UserRewardData {
        uint96 shares;
        uint96 baseRewardDebt;
        uint96 quoteRewardDebt;
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

If totalAccRewards(...) ever exceeds 2^96 − 1, the cast to uint96 will silently truncate the high bits. On the next claim(), baseAmount/quoteAmount are computed as:

    baseAmount = totalAccBaseRewards (full uint256) - uint128(truncatedDebt)

Since truncatedDebt <= 2^96 − 1 << totalAccBaseRewards, the subtraction will not underflow, but the difference will include any multiples of 2^96 that were previously truncated. This makes the computed owed amount dramatically larger than the true logical rewards, potentially far exceeding the total rewards ever deposited.

The Distributor contract then attempts to pay this amount and decrements totalPendingRewards[asset] accordingly:

    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount);
    }

However, _decreaseTotalPending() has a guard:

    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();

So once rewards get large enough for totalAccRewards to overflow 96 bits, any subsequent claim attempt will revert with ClaimAmountExceedsTotalPendingRewards instead of allowing an over-withdrawal. This still constitutes a latent accounting bug: at sufficiently large scales (large share counts and/or many reward distributions), claims will begin to fail even though there may still be legitimate rewards available, and there is no explicit cap or invariant preventing the system from reaching such a state.

While the 2^96 threshold is very high, the code has no safety check or documentation about this limit. If this system is deployed on chains or tokens with very large units or a very long lifetime, cumulative rewards could realistically approach this bound, at which point reward claims would become impossible and funds would be stuck.

This is a classic unsafely downcast pattern: using a smaller integer type for state, but feeding it from unbounded uint256 arithmetic without range checking or explicit caps.
 ### Static Signals
reward debts stored as uint96 but computed as uint256, uint256 to uint96 cast without any bounds check, accRewardPerShare is unbounded and monotonic, so totalAccRewards can grow beyond 2^96
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: Distributor.addRewards / claimRewards / increaseStake / decreaseStake

 ### Title
Reward rounding dust becomes permanently locked due to mismatched global accounting
 ### Description/Code Snippet
The reward distribution uses a standard `accRewardPerShare` pattern with integer division, but the global `totalPendingRewards` accounting in `Distributor` assumes all deposited rewards are eventually claimable. Due to rounding, a portion of rewards can never be claimed yet can also never be skimmed, causing systematic accumulation of locked dust.

Flow:
1. Rewards are added via `Distributor.addRewards`:
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

function _increaseTotalPending(address asset, uint256 amount) internal {
    unchecked {
        totalPendingRewards[asset] += amount;
    }
    emit TotalPendingRewardsIncreased(asset, amount);
}
```
This records the full `amount` in `totalPendingRewards[asset]` and transfers that many tokens into the contract.

2. At distribution time, `RewardsTrackerLib` uses fixed-point math with truncating integer division:
```solidity
uint128 public constant PRECISION_FACTOR = 1e12;

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR; // floors
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
        delete self.pendingBaseRewards; // dust from division is discarded
    }
    if (self.pendingQuoteRewards > 0) {
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        delete self.pendingQuoteRewards;
    }
}
```
Because of the `* PRECISION_FACTOR / totalShares` and then later `/ PRECISION_FACTOR` in `totalAccRewards`, the per-share math always floors, so for each deposit of `D` tokens into `pendingBaseRewards`/`pendingQuoteRewards`, the sum of all users' claimable rewards is at most `D`, and often strictly less by some dust `r > 0`.

3. When rewards are actually paid, `Distributor` decrements `totalPendingRewards` only by the amount paid out:
```solidity
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

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked {
        totalPendingRewards[asset] -= amount;
    }
    emit TotalPendingRewardsDecreased(asset, amount);
}
```
Over the lifetime of a reward pool, if a total of `D_total` tokens of some asset are added via `addRewards`, `totalPendingRewards[asset]` is incremented by `D_total`. However, because of the flooring in the per-share math, the total amount ever claimable and thus ever subtracted via `_decreaseTotalPending` is `D_total - R`, where `R` is the aggregate rounding dust across all deposits.

This leaves `R > 0` tokens:
- Still held by the contract (`asset.balanceOf(address(this))`), and
- Still reflected in `totalPendingRewards[asset]` (since only paid amounts are subtracted).

4. These residual tokens are **unclaimable** and also **unskimmable**:
- No user has any way to claim the dust because the reward-per-share index has discarded it at each `update()` call.
- The owner cannot recover it with `skimExcessRewards` because the function only allows skimming `balance - totalPendingRewards`, which remains zero for this dust:
```solidity
function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}
```
Since `asset.balanceOf(this)` and `totalPendingRewards[asset]` track the same quantity including the dust, their difference does not grow, and dust cannot be treated as "excess".

Impact:
- A fraction of every rewards deposit is effectively burned into the contract state and can never be reclaimed by users or admins.
- Over many fee accruals and external `addRewards` calls (e.g., from `GTELaunchpadV2Pair._distributeLaunchpadFees`), this locked dust can accumulate to a non-trivial amount of the rewards tokens.
- This is a systematic precision drift / accounting mismatch rather than a one-off rounding issue.

This pattern matches `PrecisionDriftAccumulation`/`AccountingInvariantViolation`: the global `totalPendingRewards` mapping treats all deposited tokens as payable obligations, but the per-share rounding makes a portion of those obligations unfulfillable while simultaneously preventing the protocol from skimming out the idle remainder.
 ### Static Signals
fixed-point rewards with PRECISION_FACTOR=1e12 and integer division, pending rewards deleted after division (dust discarded), global totalPendingRewards increased by full deposit but only decreased by paid amounts, skimExcessRewards uses balanceOf - totalPendingRewards, preventing recovery of residual dust
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
addRewards accepts wrong quote token, breaking rewards accounting and claims
 ### Description/Code Snippet
In `Distributor.addRewards` the contract allows *any* caller to add rewards to an existing pool using any second token address, without validating that this token matches the configured quote asset for the pool. The reward pool is keyed only by the launch asset (`launchAsset`), and the library `RewardPoolData` does not store which asset its `pendingQuoteRewards` correspond to.

Relevant code (simplified):
```solidity
function createRewardsPair(address launchAsset, address quoteAsset) external onlyLaunchpad {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    RewardPoolData storage rsq = RewardsTrackerStorage.getRewardPool(quoteAsset);

    if (rs.quoteAsset != address(0) || rsq.quoteAsset != address(0)) revert RewardsExist();

    rs.initializePair(launchAsset, quoteAsset); // sets rs.quoteAsset = quoteAsset
}

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

Key issues:

1. **No validation that the `quoteAsset` passed to `addRewards` matches the pool’s configured `rs.quoteAsset`.**
   - The pool is identified only by `launchAsset` (whichever of `token0`/`token1` corresponds to a pool with `rs.quoteAsset != 0`).
   - The other address is blindly treated as `quoteAsset` and used for `_increaseTotalPending(quoteAsset, amount)` and `quoteAsset.safeTransferFrom`.
   - However, reward *distribution* later uses `rs.quoteAsset` (set in `createRewardsPair`) as the quote token address, **ignoring** whatever `quoteAsset` was used in `addRewards`.

2. **Accounting mismatch between `RewardPoolData.pendingQuoteRewards` and `totalPendingRewards` mapping.**
   - `addQuoteRewards` only increments `self.pendingQuoteRewards`; it does not track which token those rewards represent.
   - `_increaseTotalPending(quoteAsset, quoteAssetAmount)` records the provided `quoteAsset` address (which may be wrong) as the asset whose balance backs those rewards.
   - When users claim via `claimRewards`, the library computes a `quoteAmount` using `accQuoteRewardPerShare` (which has incorporated all `pendingQuoteRewards`), and `_distributeAssets` calls:
     ```solidity
     _decreaseTotalPending(rs.quoteAsset, quoteAmount);
     rs.quoteAsset.safeTransfer(msg.sender, quoteAmount);
     ```
     i.e. it **always uses `rs.quoteAsset`**, not the `quoteAsset` argument passed to `addRewards`.

3. **Attack scenario (permissionless griefing / DoS of reward claims):**
   - Assume a legitimate pool exists for `(launchAsset, realQuote)` and `rs.totalShares > 0`.
   - An attacker deploys a worthless ERC20 `FakeToken`, approves `Distributor`, and calls:
     ```solidity
     Distributor.addRewards(launchAsset, FakeToken, 0, FAKE_AMOUNT);
     ```
   - Because `rs = getRewardPool(launchAsset)` has `rs.quoteAsset == realQuote != 0`, the function accepts this call:
     - `pendingQuoteRewards` in the pool increases by `FAKE_AMOUNT`.
     - `totalPendingRewards[FakeToken]` increases by `FAKE_AMOUNT`.
     - `FakeToken.safeTransferFrom` moves `FAKE_AMOUNT` FakeTokens into the Distributor.
   - On the next `claimRewards(launchAsset)` or `increaseStake` / `decreaseStake`:
     - `RewardsTrackerLib.update()` incorporates `pendingQuoteRewards` (which includes `FAKE_AMOUNT`) into `accQuoteRewardPerShare` and zeros `pendingQuoteRewards`.
     - `quoteAmount` for each user is computed as if **`FAKE_AMOUNT` units of `realQuote`** were added.
     - `_distributeAssets` then executes:
       ```solidity
       _decreaseTotalPending(realQuote, quoteAmount);
       realQuote.safeTransfer(msg.sender, quoteAmount);
       ```
     - But `totalPendingRewards[realQuote]` was **never increased** for the fake deposit, so `currTotal < quoteAmount` and `_decreaseTotalPending` reverts with `ClaimAmountExceedsTotalPendingRewards()`.
   - Result: all subsequent reward-distribution flows involving quote rewards for this pool (claims, stake/unstake that stream rewards) **revert**, effectively **bricking the reward pool** until enough *real* quote tokens are donated to cover the fake amount.

4. **Stuck fake tokens that cannot be recovered:**
   - The attacker’s `FakeToken` deposit is recorded as `totalPendingRewards[FakeToken]`, so `skimExcessRewards(FakeToken, ...)` will see `balanceOf - totalPendingRewards == 0` and cannot withdraw them as “excess”.
   - Since `_decreaseTotalPending` is only called with `rs.quoteAsset` (the real quote asset), there is no code path that burns down `totalPendingRewards[FakeToken]` or returns those tokens to anyone.
   - The fake tokens are stuck, but more importantly, the pool’s accounting for actual rewards is now inconsistent, causing a persistent denial-of-service for legitimate reward claims.

This matches an access-control / gating pattern: a sensitive state-changing function (`addRewards`) that mutates global reward accounting is left fully permissionless *and* does not restrict which token addresses can be used for a given pool. Because distribution logic hard-codes the configured `rs.quoteAsset`, a malicious user can bypass the implicit “quote token must be rs.quoteAsset” assumption and corrupt the reward state for all users of that pool.

 ### Static Signals
addRewards is external and permissionless, does not validate token1 == rewardsPool.quoteAsset, RewardPoolData.pendingQuoteRewards has no asset binding, totalPendingRewards keyed by provided quoteAsset, not rs.quoteAsset, _distributeAssets always uses rs.quoteAsset for payouts
 ### Assets at Risk
rewards, LP fee revenue, user reward claims
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
addRewards allows mismatched quote token, breaking reward accounting and locking tokens
 ### Description/Code Snippet
The `Distributor.addRewards` function does not validate that the `token0`/`token1` arguments are consistent with the reward pool’s configured quote asset.

`RewardPoolData.quoteAsset` is set once in `createRewardsPair` via `initializePair(launchAsset, quoteAsset)` and denotes the *only* quote token that should ever be distributed for that pool. However, `addRewards` will happily accept any second token as `quoteAsset`, even if it does not match `rs.quoteAsset`:

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

Scenario:
- A valid rewards pool exists for `(launchAsset = A, quoteAsset = B)`, so `RewardsTrackerStorage.getRewardPool(A).quoteAsset == B`.
- Any user (permissionless) calls `addRewards(A, C, 0, amount1)` where `C` is an arbitrary ERC20 not equal to `B`.
- The first `rs = getRewardPool(token0)` finds the pool for `A` and *does not* reassign to `token1`, because `rs.quoteAsset != address(0)`.
- The function then:
  - Calls `rs.addQuoteRewards(launchAsset, quoteAsset, quoteAssetAmount)`, which increases `pendingQuoteRewards` for the pool (logically denominated in the pool’s configured quote asset `B`).
  - Calls `_increaseTotalPending(quoteAsset, quoteAssetAmount)` and transfers `C` tokens into the `Distributor`.

Now the accounting is inconsistent:
- At the pool level, `pendingQuoteRewards` and subsequent `accQuoteRewardPerShare` treat the new rewards as **B**-denominated amounts.
- At the `Distributor` level, `totalPendingRewards[C]` and the ERC20 balance for `C` increased, but **no** `B` tokens were actually funded for these rewards.

When users later stake/unstake/claim, quote rewards are paid using `rs.quoteAsset` (i.e. `B`) and the `totalPendingRewards` mapping for `B`:

```solidity
function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount);
    }

    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount); // quote == rs.quoteAsset (B)
        quote.safeTransfer(msg.sender, quoteAmount);
    }
}
```

Because the phantom rewards from the `(A, C)` call increased `pendingQuoteRewards` but **not** `totalPendingRewards[B]`, the following happens:
- The pool will eventually attempt to pay out more `B` than was ever actually deposited for `B`.
- `_decreaseTotalPending(B, quoteAmount)` will revert with `ClaimAmountExceedsTotalPendingRewards()` once expected quote payouts exceed the real `totalPendingRewards[B]`.
- Legitimate users are then **unable to claim remaining B rewards** from that pool.
- The `C` tokens sitting in `Distributor` are marked as `totalPendingRewards[C]` and **cannot be skimmed**, so they are effectively locked forever.

This breaks the intended invariant that:
- `totalPendingRewards[asset]` should reflect real, claimable rewards for that asset, and
- pool-level `pendingQuoteRewards/accQuoteRewardPerShare` should only track rewards backed by that same asset.

Because `addRewards` is permissionless and does not enforce `quoteAsset == rs.quoteAsset`, any user can grief a rewards pool by donating a wrong token as the quote reward, causing a permanent accounting mismatch and denial-of-service for quote reward claims.
 ### Static Signals
reward pool identified solely by first token, second token not validated against pool.quoteAsset, pendingQuoteRewards updated for pool while totalPendingRewards updated for a different token address, payout uses rs.quoteAsset but funding used arbitrary quoteAsset argument
 ### Assets at Risk
launchpad rewards, user staking rewards, mis-deposited ERC20 tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.addRewards

 ### Title
AMM swaps can be permanently DOSed when Distributor.totalShares reaches zero
 ### Description/Code Snippet
The `Distributor` and `GTELaunchpadV2Pair` share an implicit invariant: the pair should only attempt to send launchpad fees to the `Distributor` while there are active staking shares (`RewardPoolData.totalShares > 0`). This invariant is not enforced on-chain and is instead assumed to be respected off-chain by the Launchpad when it calls `endRewards`.

In `Distributor.addRewards`:

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

The rewards pair (Uniswap V2 fork) accrues launchpad fees and forwards them to `Distributor.addRewards` on each swap via `_update` and `_distributeLaunchpadFees`:

```solidity
// GTELaunchpadV2Pair.swap
(uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
    && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));

_update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
...

function _update(..., uint112 newLaunchpadFee0, uint112 newLaunchpadFee1) private {
    ...
    if (launchpadFeeDistributor > address(0)) {
        if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
            delete accruedLaunchpadFee0;
            delete accruedLaunchpadFee1;
            _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
        }
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
        ...
    }
}
```

Once a rewards pool’s `totalShares` becomes zero (all stakers effectively removed), but `GTELaunchpadV2Pair.rewardsPoolActive` is still `1` (its initial value, only cleared by `endRewardsAccrual()`), the next swap that generates a non-zero `launchpadFee0` or `launchpadFee1` will:

1. Call `_distributeLaunchpadFees(...)` from `_update`.
2. `_distributeLaunchpadFees` calls `Distributor.addRewards(...)`.
3. Inside `addRewards`, `rs.totalShares == 0` triggers `revert NoSharesToIncentivize()`.
4. This revert propagates all the way back to `swap`, causing the swap to revert.

From that point onward, **every** swap that attempts to accrue launchpad fees continues to revert, effectively DOSing the AMM for that pair until governance/Launchpad manually calls `Distributor.endRewards(pair)` which in turn calls `GTELaunchpadV2Pair.endRewardsAccrual()` to set `rewardsPoolActive` to `0`.

Because the condition `rs.totalShares == 0` is entirely managed by Launchpad logic and not coupled on-chain to `GTELaunchpadV2Pair.rewardsPoolActive`, there is a realistic window where:

- All shares are removed (`totalShares == 0`), **but**
- `rewardsPoolActive` is still `1` (no call to `endRewardsAccrual()` yet), and
- Regular users calling `swap` on the pair will cause reverts due to `Distributor.addRewards` failing.

This breaks the system-level invariant that the AMM should remain usable regardless of staking state and that launchpad fee accounting should not be able to brick trading. It also means fee accounting and staking state become inconsistent between the pair and the Distributor. The DOS is triggered by a permissionless `swap` call and persists until privileged actors intervene.
 ### Static Signals
addRewards() reverts when totalShares == 0, GTELaunchpadV2Pair.swap() always attempts fee distribution while rewardsPoolActive > 0, rewardsPoolActive not auto-synced with Distributor totalShares, fee distribution path lacks graceful handling for zero-share state
 ### Assets at Risk
AMM liquidity usability, trading availability for launched token, launchpad fee distribution accounting
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

