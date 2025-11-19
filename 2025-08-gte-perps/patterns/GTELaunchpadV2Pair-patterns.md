## Verified Patterns Found: 6

## Verified Patterns Found in following Categories:

- ReserveOrPriceDesync
- AccountingInvariantViolation
- ERC4626SharePriceMismatch
- FeeAccountingDrift



## Summary of Patterns

Launchpad fee accrual can desync AMM reserves and constant-product math on same-block swaps

Large reward balances can overflow PRECISION math and brick rewards pool

LP share minting double‑counts pending launchpad fees, enabling share inflation

Accrued launchpad fees mishandled in mint/burn, distorting LP share accounting

Rounding in rewards index vs totalPendingRewards causes permanently locked reward dust

Launchpad fee accrual breaks Uniswap k-invariant on intra-block swaps

## Patterns



 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: GTELaunchpadV2Pair.swap

 ### Title
Launchpad fee accrual can desync AMM reserves and constant-product math on same-block swaps
 ### Description/Code Snippet
The launchpad fee mechanism introduces an internal notion of "accrued" fees (held in `accruedLaunchpadFee0/1`) that are *excluded* from the stored reserves but still physically present in the pair’s token balances. However, the swap math in `swap()` uses **raw token balances** (which include these accrued fees) to compute `amount0In` / `amount1In`, and then uses those values to compute new launchpad fees and update reserves. This can cause fee double-counting and a divergence between the constant-product invariant and actual balances when there are multiple swaps in the same block.

Key code paths:

```solidity
function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external lock {
    ...
    (uint112 _reserve0, uint112 _reserve1,) = getReserves();
    ...
    if (amount0Out > 0) _safeTransfer(_token0, to, amount0Out);
    if (amount1Out > 0) _safeTransfer(_token1, to, amount1Out);
    if (data.length > 0) IUniswapV2Callee(to).uniswapV2Call(...);
    balance0 = IERC20(_token0).balanceOf(address(this));
    balance1 = IERC20(_token1).balanceOf(address(this));
    ...
    uint256 amount0In = balance0 > _reserve0 - amount0Out
        ? balance0 - (_reserve0 - amount0Out)
        : 0;
    uint256 amount1In = balance1 > _reserve1 - amount1Out
        ? balance1 - (_reserve1 - amount1Out)
        : 0;
    ...
    (uint112 launchpadFee0, uint112 launchpadFee1) =
        launchpadFeeDistributor > address(0) && rewardsPoolActive > 0
            ? _getLaunchpadFees(amount0In, amount1In)
            : (uint112(0), uint112(0));

    _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
    ...
}
```

The launchpad fee logic inside `_update` and `_getLaunchpadFees` is:

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

    // Balances contain both accrued and new launchpad fees earned this tx
    // as balance is called before any fee distributions, so subtract the total
    reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;
    reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;
    ...
}

function _getLaunchpadFees(uint256 amount0In, uint256 amount1In)
    internal
    view
    returns (uint112 fee0, uint112 fee1)
{
    uint256 totalLpBal = this.totalSupply();
    uint256 launchpadLpBal = this.balanceOf(launchpadLp) + MINIMUM_LIQUIDITY;

    if (amount0In > 0)
        fee0 = uint112(amount0In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));
    if (amount1In > 0)
        fee1 = uint112(amount1In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));
}
```

The critical detail is that `getReserves()` returns `reserve0`/`reserve1` **excluding** `accruedLaunchpadFee0/1`, while `balance0`/`balance1` from `IERC20(...).balanceOf` are the *full* token balances, including previously accrued (but undistributed) launchpad fees. Therefore, when `accruedLaunchpadFee{0,1} > 0`, the computed `amount{0,1}In` is:

- `amount0In = (reserve0 + accrued0 + Δ0_in - Δ0_out) - (reserve0 - amount0Out)`
  = `accrued0 + Δ0_in`,

so earlier accrued fees are treated as if they were **fresh input** on the next swap.

Because `_getLaunchpadFees` computes `fee0/fee1` purely as a function of `amount0In`/`amount1In`, this means:
- launchpad fees are computed **on top of previously accrued fees** (fees-on-fees) rather than only on the new swap input;
- `totalLaunchpadFee{0,1} = accruedLaunchpadFee{0,1} + newLaunchpadFee{0,1}` then subtracts this summed value from `balance{0,1}` to recompute `reserve{0,1}`.

This pattern only occurs when `accruedLaunchpadFee{0,1}` is non-zero, which happens whenever multiple swaps occur in the **same block** (since `_update` goes through the `timeElapsed == 0` branch and defers actual distribution). An attacker can intentionally create this situation by, e.g.:

1. Calling `sync()` once at the start of a transaction to set `blockTimestampLast` to the current block timestamp.
2. Performing a first `swap()` in that same transaction; because `timeElapsed == 0`, new launchpad fees for that swap are stored in `accruedLaunchpadFee{0,1}` and *not* distributed.
3. Immediately performing a second (and possibly larger) `swap()` in the same transaction. Now `accruedLaunchpadFee{0,1}` is non-zero, and the second swap’s `amount{0,1}In` mis-treats those already accrued tokens as fresh input. `_getLaunchpadFees` then computes additional launchpad fees on top of them, and `_update` subtracts the sum from balances to derive new reserves.

Because the constant-product K-check in `swap()` uses `balance{0,1}` (which *include* accrued fees) and `_reserve{0,1}` (which *exclude* them), inflating `amount{0,1}In` with prior accrued fees makes the inequality:

```solidity
balance0Adjusted = balance0.mul(1000).sub(amount0In.mul(3));
balance1Adjusted = balance1.mul(1000).sub(amount1In.mul(3));
require(balance0Adjusted.mul(balance1Adjusted) >= uint256(_reserve0).mul(_reserve1).mul(1000 ** 2), "UniswapV2: K");
```

easier to satisfy than it should be. Intuitively, `balance{0,1}Adjusted` are artificially boosted by terms proportional to the accrued fees, so a trader can choose `amount{0,1}Out` closer to the edge (or beyond) of the true constant-product curve while still passing the check.

Potential consequences:
- **Reserve / price desynchronization**: on same-block multi-swap sequences, the stored reserves and the effective swap pricing can diverge from the true constant-product AMM curve because fees-on-fees are subtracted from reserves but also counted as input in the K-check.
- **Over-distribution of launchpad fees**: the launchpad fee distributor can receive more tokens than intended (fees charged on previously accrued fees), effectively draining additional value from LPs beyond the configured `REWARDS_FEE_SHARE` fraction.
- **Economic exploitation route**: an attacker can craft transactions with multiple swaps in the same block (optionally preceded by `sync()`) to (a) generate a large `accruedLaunchpadFee`, then (b) perform a second swap that extracts slightly more value than allowed by the ideal x*y=k invariant, potentially arbitraging against other venues or repeatedly skewing the pool.

This behavior depends only on standard ERC-20 transfers and the ability to issue multiple swaps per block, both of which are under a regular user’s control on an L2 like MegaETH.
 ### Static Signals
Reserves exclude accruedLaunchpadFee but balances include them, amountIn computed from raw token balances that include fee balances, Fee share computed purely from amountIn via _getLaunchpadFees, accruedLaunchpadFee carried across same-block swaps (timeElapsed == 0), K-invariant uses _reserve values that omit accrued fees
 ### Assets at Risk
AMM liquidity (pool reserves), Launchpad fee rewards, Trader pricing fairness
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: RewardsTrackerLib.getAccRewardsPerShare

 ### Title
Large reward balances can overflow PRECISION math and brick rewards pool
 ### Description/Code Snippet
In the rewards distribution logic, `pendingBaseRewards` and `pendingQuoteRewards` are stored as `uint128` but are multiplied by a large precision factor `PRECISION_FACTOR = 1e12` while still in the 128-bit domain.

Relevant code (library `RewardsTrackerLib`):

```solidity
uint128 public constant PRECISION_FACTOR = 1e12;

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
```

`self.pendingBaseRewards` and `self.pendingQuoteRewards` are `uint128`. Their product with `PRECISION_FACTOR` is also computed as a `uint128` (because both operands are `uint128`), and in Solidity 0.8 this multiplication reverts on overflow *before* the division. That means if `pending{Base,Quote}Rewards > ~3.4e26` (≈ `2^128 / 1e12`), `getAccRewardsPerShare` and any function that calls it (`update`, and transitively `stake`, `unstake`, `claim`, `getPendingRewards`) will revert.

Since `Distributor.addRewards` is permissionless and simply does:

```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    ...
    if (launchAssetAmount > 0) {
        rs.addBaseRewards(launchAsset, launchAssetAmount); // pendingBaseRewards += amount
        _increaseTotalPending(launchAsset, launchAssetAmount);
        launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
    }
    ...
}
```

an attacker (or even a well-meaning project) can push `pendingBaseRewards` or `pendingQuoteRewards` above this threshold by depositing a very large reward amount, especially for high-supply 18-decimal tokens. The `addRewards` call itself succeeds, but the **next** call to `stake`, `unstake`, `claimRewards`, or `getPendingRewards` for that rewards pool will revert due to the overflow in `pending * PRECISION_FACTOR`.

This creates a realistic DoS vector against the rewards system for a given launch asset: once a very large reward is added, no user can stake, unstake, or claim rewards in that pool anymore, and there is no on-chain mechanism to reduce `pending{Base,Quote}Rewards` to recover from this state. All already-deposited rewards for that asset become effectively stuck.

Because `GTELaunchpadV2Pair._distributeLaunchpadFees` continuously calls `IDistributor.addRewards` for each swap, a long period of heavy trading combined with infrequent `stake/unstake/claim` activity can also push `pending*PRECISION` into the overflow region over time, even without a single huge deposit.
 ### Static Signals
pendingBaseRewards and pendingQuoteRewards are uint128, PRECISION_FACTOR = 1e12, multiplication pendingBaseRewards * PRECISION_FACTOR in uint128 domain, update()/stake()/unstake()/claim() depend on getAccRewardsPerShare()
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: GTELaunchpadV2Pair.mint

 ### Title
LP share minting double‑counts pending launchpad fees, enabling share inflation
 ### Description/Code Snippet
GTELaunchpadV2Pair mints LP tokens in mint() based on the delta between current token balances and stored reserves: amount0 = balance0.sub(_reserve0), amount1 = balance1.sub(_reserve1). However reserves deliberately exclude accruedLaunchpadFee0/1 while balances include them when launchpad fees have been accrued but not yet distributed (timeElapsed == 0 path in _update). In _update(), when timeElapsed == 0 and newLaunchpadFee0/1 > 0, the contract sets accruedLaunchpadFee{0,1} = totalLaunchpadFee{0,1} and updates reserves as reserve0 = uint112(balance0) - totalLaunchpadFee0, so the difference balance0 - reserve0 equals the pending launchpad fees. If a liquidity provider forces this state (e.g. by calling sync() as the first call in a block to set blockTimestampLast, then doing a large swap that accrues new launchpad fees in the same block so timeElapsed == 0), accruedLaunchpadFee{0,1} will be non‑zero and balances will exceed reserves by exactly the pending fees. If the attacker then immediately calls mint() in the same block, amount0 and amount1 used for computing liquidity include both their deposited tokens and the pending launchpad fees: amount0 = deposit0 + accruedLaunchpadFee0 (and similarly for token1 when applicable). The subsequent _update() in mint() still subtracts totalLaunchpadFee{0,1} from balances when setting reserves, and those fees are later transferred out to the Distributor on the next cross‑block _update; however, the extra LP tokens minted based on the fee component are never clawed back. This inflates the attacker’s LP share relative to their actual deposit, effectively reallocating a small portion of pool value (and the economics of the launchpad fee stream) away from existing LPs and the intended rewards recipients. This matches the ERC4626SharePriceMismatch/inflation pattern: share minting uses a manipulable asset base (balance - reserve) that includes protocol‑owned pending fees rather than only true user deposits, and there is no guard (e.g. virtual shares/assets or forcing accruedLaunchpadFee to be zero) before mint.
 ### Static Signals
LP shares minted from (balance - reserve), reserves exclude accruedLaunchpadFee while balances include them, accruedLaunchpadFee0/1 updated in _update() but not zeroed before mint(), no virtual assets/shares to isolate protocol-owned fees, same-block sequence sync() -> swap() (timeElapsed == 0) -> mint() possible
 ### Assets at Risk
LP liquidity providers, launchpad rewards (portion of swap fees intended for Distributor recipients)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.mint

 ### Title
Accrued launchpad fees mishandled in mint/burn, distorting LP share accounting
 ### Description/Code Snippet
The pair tracks launchpad fee accrual in `accruedLaunchpadFee0/1` and excludes these amounts from reserves when syncing, but `mint`/`burn` compute liquidity using raw balances that *include* the accrued fees.

Key code paths:

1. Reserves exclude accrued fees in `_update`:
```solidity
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 _reserve0,
    uint112 _reserve1,
    uint112 newLaunchpadFee0,
    uint112 newLaunchpadFee1
) private {
    // ...
    uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
    uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;
    // ...
    reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;
    reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;
    // ...
}
```
After this, the invariant is intended to be:

`tokenBalance = reserve + accruedLaunchpadFee`.

2. `mint` uses `balance0 - _reserve0` as the deposited amount, but `_reserve0` does **not** include accrued fees while `balance0` does:
```solidity
function mint(address to) external lock returns (uint256 liquidity) {
    (uint112 _reserve0, uint112 _reserve1,) = getReserves();
    uint256 balance0 = IERC20(token0).balanceOf(address(this));
    uint256 balance1 = IERC20(token1).balanceOf(address(this));
    uint256 amount0 = balance0.sub(_reserve0);
    uint256 amount1 = balance1.sub(_reserve1);
    // ... liquidity = Math.min(amount0 * _totalSupply / _reserve0, ...);
}
```
If `accruedLaunchpadFee0 > 0` (which happens for swaps in the same block where `_update` saw `timeElapsed == 0`), then:

- Before `mint`:
  - `balance0 = reserve0 + accruedLaunchpadFee0 + userDeposit0`
  - `_reserve0 = reserve0` (does NOT include accrued fees)
  - So `amount0 = balance0 - _reserve0 = accruedLaunchpadFee0 + userDeposit0`.

Consequently, the new LP shares are minted as if the user had deposited both their own tokens **and** the yet-undistributed launchpad fees. However, when `_update` later runs in a block with `timeElapsed > 0`, it calls `_distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1)` and transfers **all** accrued fees out to `launchpadFeeDistributor`, while the over-minted LP shares remain. Existing LPs are therefore diluted; the newcomer effectively captures part of the launchpad fee pot.

3. `burn` also uses raw balances that include accrued fees:
```solidity
function burn(address to) external lock returns (uint256 amount0, uint256 amount1) {
    (uint112 _reserve0, uint112 _reserve1,) = getReserves();
    address _token0 = token0;
    address _token1 = token1;
    uint256 balance0 = IERC20(_token0).balanceOf(address(this));
    uint256 balance1 = IERC20(_token1).balanceOf(address(this));
    uint256 liquidity = balanceOf[address(this)];

    // uses balance0 / balance1, which include accrued fees
    amount0 = liquidity.mul(balance0) / _totalSupply;
    amount1 = liquidity.mul(balance1) / _totalSupply;
    // ...
    _safeTransfer(_token0, to, amount0);
    _safeTransfer(_token1, to, amount1);
    balance0 = IERC20(_token0).balanceOf(address(this));
    balance1 = IERC20(_token1).balanceOf(address(this));

    _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
}
```
If `accruedLaunchpadFee* > 0` and a large fraction of LP tokens is burned in the same block before fees are distributed, the post-burn balances can fall below `totalLaunchpadFee*`, causing `_update`'s final subtraction `uint112(balanceX) - totalLaunchpadFeeX` to underflow and revert. This means that burning near 100% of liquidity in the same block as sizable fee accrual is temporarily impossible—creating a block-level DoS on full exit while fees are pending.

**Why this matches AccountingInvariantViolation pattern**:
- The internal notion of "pool reserves" in `_update` excludes `accruedLaunchpadFee*`, but `mint` and `burn` treat those accrued fees as if they were part of the pool’s distributable assets.
- As a result, the mapping from LP token supply to underlying reserves is not strictly proportional to the actual contributed liquidity: newcomers can be slightly over-minted when there is non-zero `accruedLaunchpadFee*`, diluting existing LPs and effectively diverting some launchpad fees into LP ownership.
- In edge cases, `burn` can revert for large `liquidity` when `balanceX < totalLaunchpadFeeX` after transfers, temporarily preventing full liquidity withdrawal until a later block when `_update` runs with `timeElapsed > 0` and distributes the fees.

**Attack/abuse sketch (permissionless):**
1. In block N, someone performs a swap that triggers `_update` with `timeElapsed > 0`, clearing any prior `accruedLaunchpadFee*`.
2. An attacker then, in the **same block N**, performs another swap (so `_update` has `timeElapsed == 0`) to build up `accruedLaunchpadFee*`.
3. Still in block N, the attacker deposits liquidity and calls `mint(to)`. Because `amount0/amount1` include the accrued fees, they receive slightly more LP tokens than their pure deposit ratio would justify, effectively taking some value from existing LPs and from the launchpad fee pot.
4. In block N+1, any `_update` with `timeElapsed > 0` will actually send the accrued fees out to `launchpadFeeDistributor`, while the attacker's over-minted LP position remains.

Even if the absolute gain per block is small (bounded by `REWARDS_FEE_SHARE` and the size of `accruedLaunchpadFee*`), this is a non-trivial, systematic mis-accounting of liquidity shares and pending fees that can be exploited or at least used for MEV-style value extraction at LPs' expense.
 ### Static Signals
reserves exclude accruedLaunchpadFee but mint uses balance - _reserve, accruedLaunchpadFee updated in _update and subtracted from reserves, new LP liquidity proportional to (balance - reserve) which includes accrued fees, burn relies on balance >= totalLaunchpadFee to avoid underflow
 ### Assets at Risk
LP liquidity value, launchpad fee rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: RewardsTrackerLib.update

 ### Title
Rounding in rewards index vs totalPendingRewards causes permanently locked reward dust
 ### Description/Code Snippet
The rewards system maintains two layers of accounting:

1. Per-pool accounting in `RewardPoolData` (via `RewardsTrackerLib`), using `pendingBaseRewards`, `pendingQuoteRewards`, and per-share indices `accBaseRewardPerShare` / `accQuoteRewardPerShare`.
2. Global per-asset accounting in `Distributor.totalPendingRewards[asset]`, used to ensure the admin cannot skim user-owed rewards:

```solidity
mapping(address => uint256) public totalPendingRewards;
```

When rewards are added:

```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    ...
    if (launchAssetAmount > 0) {
        rs.addBaseRewards(launchAsset, launchAssetAmount); // pendingBaseRewards += amount
        _increaseTotalPending(launchAsset, launchAssetAmount); // totalPendingRewards[asset] += amount
        launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
    }
    ...
}
```

`RewardsTrackerLib.update()` later converts `pending{Base,Quote}Rewards` into per-share indices and zeroes the pending fields:

```solidity
function update(RewardPoolData storage self)
    internal
    returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
{
    (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    if (self.pendingBaseRewards > 0) {
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        delete self.pendingBaseRewards; // <-- zeroed here
    }

    if (self.pendingQuoteRewards > 0) {
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        delete self.pendingQuoteRewards; // <-- zeroed here
    }
}
```

But `getAccRewardsPerShare` uses floor-division when spreading pending rewards over shares:

```solidity
if (self.pendingBaseRewards > 0) {
    accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
}
```

Because of integer division, for each `update()` call there can be a non-zero remainder:

```text
remainder_base = pendingBaseRewards - floor(pendingBaseRewards * PRECISION_FACTOR / totalShares) * totalShares / PRECISION_FACTOR
```

This "dust" is effectively **discarded at the pool level** (since `pendingBaseRewards` is set to zero), but `Distributor.totalPendingRewards[asset]` is **not** reduced when `update()` runs; it only decreases when actual transfers are made in `_distributeAssets`:

```solidity
function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount); // totalPendingRewards[base] -= baseAmount
        base.safeTransfer(msg.sender, baseAmount);
    }
    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(msg.sender, quoteAmount);
    }
}
```

Over time, the sum of all user claims across a reward epoch is **strictly less** than the original `amount` added, due to per-share rounding. However, `totalPendingRewards[asset]` will have been reduced only by the actually claimed amounts, leaving it overstated by the cumulative dust remainders.

At the same time, the token balance of the `Distributor` contract tracks the true (post-rounding) amount left. Because `skimExcessRewards` relies on `totalPendingRewards` as an upper bound:

```solidity
function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}
```

any rounding dust that was never allocated to users:

- **cannot be claimed by users** (it is no longer reflected in `pending{Base,Quote}Rewards` or in `acc*RewardPerShare`), and
- **cannot be withdrawn by the admin as "excess"**, because `asset.balanceOf(this) - totalPendingRewards[asset]` does not account for the fact that `totalPendingRewards` is larger than the sum of all remaining claimable rewards.

As a result, over many reward epochs the system accumulates a growing amount of **permanently locked dust** per asset, which neither users nor the protocol can ever reclaim. This is a form of fee/accounting drift: the on-chain variable `totalPendingRewards[asset]` no longer matches the actual maximum claimable or distributable rewards, and a fraction of contributed rewards becomes unspendable.

While each individual remainder is small, the pattern is systematic and unbounded in the number of reward additions / updates, so the locked value can accumulate over time, especially for high-volume pools with frequent rewards.
 ### Static Signals
pendingBaseRewards and pendingQuoteRewards zeroed in update(), per-share accrual uses floor division in getAccRewardsPerShare(), Distributor.totalPendingRewards is never adjusted when pending*Rewards are cleared, skimExcessRewards() relies on totalPendingRewards as upper bound
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.swap

 ### Title
Launchpad fee accrual breaks Uniswap k-invariant on intra-block swaps
 ### Description/Code Snippet
GTELaunchpadV2Pair modifies the standard Uniswap V2 accounting by keeping a separate `accruedLaunchpadFee0/1` balance and setting reserves to `balance - totalLaunchpadFee`. This interacts badly with how `swap()` computes `amount0In/amount1In` and with `_getLaunchpadFees`.

Key code paths:

- In `_update` reserves are set net of launchpad fees:
```solidity
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 _reserve0,
    uint112 _reserve1,
    uint112 newLaunchpadFee0,
    uint112 newLaunchpadFee1
) private {
    // ...
    uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
    uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;
    // ...
    if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
        // distribute and delete accrued fees
        if (launchpadFeeDistributor > address(0)) {
            if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
                delete accruedLaunchpadFee0;
                delete accruedLaunchpadFee1;
                _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
            }
        }
    } else if (launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0) {
        // same-block: accumulate but do not distribute
        accruedLaunchpadFee0 = totalLaunchpadFee0;
        accruedLaunchpadFee1 = totalLaunchpadFee1;
        emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
    }

    // reserves are always net of *all* launchpad fees (accrued + new)
    reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;
    reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;
    // ...
}
```

- In `swap()` `amount0In`/`amount1In` are computed from the difference between raw token balances and those reserves:
```solidity
function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external lock {
    // ...
    balance0 = IERC20(_token0).balanceOf(address(this));
    balance1 = IERC20(_token1).balanceOf(address(this));
    // ...
    uint256 amount0In = balance0 > _reserve0 - amount0Out ? balance0 - (_reserve0 - amount0Out) : 0;
    uint256 amount1In = balance1 > _reserve1 - amount1Out ? balance1 - (_reserve1 - amount1Out) : 0;
    if (amount0In == 0 && amount1In == 0) revert("UniswapV2: INSUFFICIENT_INPUT_AMOUNT");

    uint256 balance0Adjusted = balance0.mul(1000).sub(amount0In.mul(3));
    uint256 balance1Adjusted = balance1.mul(1000).sub(amount1In.mul(3));
    if (balance0Adjusted.mul(balance1Adjusted) < uint256(_reserve0).mul(_reserve1).mul(1000 ** 2)) {
        revert("UniswapV2: K");
    }

    (uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
        && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));

    _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
    // ...
}
```

- When there are previously accrued but undistributed launchpad fees (i.e., `accruedLaunchpadFee0/1 > 0` after a prior swap in the *same block* where `timeElapsed == 0`), `reserve0/1` are set to `balance - totalLaunchpadFee`, but the next `swap()` still reads the **raw** balances via `IERC20.balanceOf`. As a result, for a subsequent swap in the same block:
  * Let `A0` be `accruedLaunchpadFee0` before the swap.
  * Let `dx0` be the *actual* token0 input of the new swap.
  * The new `balance0` becomes `reserve0 + A0 + dx0 - amount0Out`.
  * `_reserve0` from `getReserves()` is `reserve0` (which excludes `A0`).
  * The computed `amount0In` becomes `balance0 - (_reserve0 - amount0Out) = A0 + dx0`.

Thus **all previously accrued launchpad fee tokens `A0` are counted as additional `amount0In`**, even though no new tokens were transferred in for them. Symmetric reasoning holds for token1.

Consequences:

1. **Broken constant-product pricing for intra-block swaps**
   - The internal Uniswap invariant check uses `amount0In`/`amount1In`:
   ```solidity
   balance0Adjusted = balance0 * 1000 - amount0In * 3;
   balance1Adjusted = balance1 * 1000 - amount1In * 3;
   require(balance0Adjusted * balance1Adjusted >= _reserve0 * _reserve1 * 1000**2);
   ```
   - For a token0→token1 swap with no token0 output, substituting shows that the allowed `amount1Out` solves the same equation as standard Uniswap **but with an effective input of `dx0 + A0` instead of `dx0`**. In other words, the trader can, in principle, get the output corresponding to a larger notional than they actually provided, using the pending launchpad fee balance `A0` as “virtual input”.
   - An attacker contract can:
     * Execute many swaps in a single transaction or across multiple txs within the same block, generating non-zero `accruedLaunchpadFee0/1` while ending roughly flat in inventory (e.g., back-and-forth swaps), and
     * Then perform a final swap where no or minimal real input is sent on one side, but `amount{0,1}In` is inflated by the stored `accruedLaunchpadFee` from prior swaps, allowing them to withdraw underpriced tokens from the opposite reserve while still passing the `K` check.
   - This breaks the intended accounting invariant that only **new external input** should appear in `amountIn`, and that reserves track the true on-chain balances used in the constant-product formula.

2. **Launchpad fee double-counting and fee drift**
   - `_getLaunchpadFees` is called with this inflated `amount0In/amount1In`:
   ```solidity
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
   - Because `amountIn` includes prior `accruedLaunchpadFee` balances, the launchpad is effectively charged rewards on **its own previously accrued fees**, not just on fresh swap volume. Over multiple intra-block swaps, this can systematically over-accrue rewards and, when `_update` later distributes `totalLaunchpadFee*` via `_distributeLaunchpadFees`, remove **more tokens from the pool** than intended based on real trader volume.
   - Combined with the pricing issue above, this creates a realistic avenue for value to leak simultaneously from:
     * LP reserves (through underpriced output in the exploit swap), and
     * The launchpad rewards pool (because fee accrual is calculated on an inflated base).

3. **Zero-input-style swap using only pending fees**
   - A particularly concerning edge case arises once non-zero `accruedLaunchpadFee` exists (from at least one prior same-block swap): a user can call `swap()` with `amount0Out = 0`, a chosen `amount1Out > 0`, and **send no token0 at all**. The contract will still compute `amount0In = A0 > 0` from the existing fee balance, pass the `INSUFFICIENT_INPUT_AMOUNT` check, and rely on `A0` as virtual input in the invariant. This allows withdrawal of some `amount1Out` purely backed by earlier accrued fees and reserves, with no new token0 provided.
   - At the same time, `_update` will treat those same `A0` tokens as part of `totalLaunchpadFee0` to be sent to the `launchpadFeeDistributor`, further reducing token0 reserves. Even if this is not always net-profitable in a single step, it clearly violates the underlying assumptions about conservation of liquidity and correct fee distribution.

Because swaps are `permissionless` and nothing prevents a contract from batching many swaps in a single transaction (all sharing the same `block.timestamp`, hence `timeElapsed == 0`), this mis-accounting of `amountIn` whenever `accruedLaunchpadFee* > 0` constitutes a plausible, exploitable **accounting invariant violation** that can leak value from LP reserves and/or the launchpad rewards pool during sequences of intra-block swaps.
 ### Static Signals
reserves set as balance - totalLaunchpadFee in _update, accruedLaunchpadFee incremented when timeElapsed == 0 instead of distributing, swap() computes amountIn from balance - reserve while reserves exclude fees, amountIn reused to compute launchpad fees in _getLaunchpadFees, multiple swaps per block possible without reentrancy guard across blocks
 ### Assets at Risk
LP liquidity, launchpad_rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

