## Verified Patterns Found: 5

## Verified Patterns Found in following Categories:

- ERC4626SharePriceMismatch
- UnsafeAssembyTypeCasts
- PermitMisuse
- GriefableCallbacks



## Summary of Patterns

RewardsTracker share-based accounting can overflow 96-bit debts causing reward misaccounting

Reward accounting uses unsafe uint256→uint96 downcasts which can overflow and corrupt debts

Launchpad AMM swaps can be DOSed when rewards shares hit zero

Launchpad fee callback to Distributor can revert and DoS all swaps when no shares

EIP-2612 permit accepts malleable signatures (no s/v validation)

## Patterns



 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: RewardsTrackerLib.stake

 ### Title
RewardsTracker share-based accounting can overflow 96-bit debts causing reward misaccounting
 ### Description/Code Snippet
The launchpad’s `Distributor` uses `RewardsTrackerLib` for pro‑rata reward distribution across stakers. The library tracks user shares and reward debts in 96‑bit fields, but performs reward-per-share math in full `uint256` and **casts results back to `uint96` without bounds checks**, which can break accounting invariants when values grow large.

Relevant structs (RewardsTrackerLib):

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

Core math (RewardsTrackerLib):

```solidity
function totalAccRewards(uint256 shares, uint256 accRewardsPerShare)
    internal
    pure
    returns (uint256)
{
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}

function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 existingShares = uint96(userData.shares);
    ...
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
    userData.shares -= removeShares;
    self.totalShares -= removeShares;

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
```

Problems:
- `totalAccRewards` returns a full `uint256` but is **blindly cast to `uint96`** when stored in `baseRewardDebt` / `quoteRewardDebt`.
- There is no explicit cap or sanity check on `accBaseRewardPerShare`, `accQuoteRewardPerShare`, or on the size of `pendingBaseRewards`, `pendingQuoteRewards`, or `totalShares` beyond their type widths.
- Over time, with large rewards and/or many shares, `totalAccRewards(shares, accRewardsPerShare)` can exceed `2^96-1`. When this happens, the cast to `uint96` silently truncates upper bits, corrupting the stored debt.

Why this matches the ERC4626SharePriceMismatch / AccountingInvariantViolation pattern:
- This is a **share-based accounting system**, analogous to ERC-4626 vault accounting: stakers hold `shares`, and rewards are distributed based on `accRewardPerShare` indexes.
- Truncating `totalAccRewards` into 96 bits effectively breaks the invariant that:

  `emitted rewards == Σ (pending rewards + realized rewards)`

  because some users’ debts become too small relative to the true accumulated rewards, enabling:
  * Over-claiming if their `baseRewardDebt` / `quoteRewardDebt` wraps down.
  * Under-claiming or permanent dust if other paths revert or if later math underflows/overflows due to inconsistent debts.
- This manifests as a **systematic mis-accounting of rewards** between users (similar to an ERC4626 share-price precision bug) rather than a one-off rounding error.

Plausible exploit scenario:
1. Over time, the pool accrues very large `pendingBaseRewards` / `pendingQuoteRewards` relative to `PRECISION_FACTOR` and `totalShares`.
2. `accBaseRewardPerShare` grows to the point where `totalAccRewards(shares, accBaseRewardPerShare) > type(uint96).max` for large stakers.
3. When such a staker `stake`s more, `unstake`s, or `claim`s, their `baseRewardDebt` / `quoteRewardDebt` is set to the **truncated** `uint96` value.
4. Future `totalAccRewards` computations (still full 256‑bit) minus the now‑small debt let them withdraw significantly more than their fair share; alternatively, some users’ claims can revert or yield zero due to mismatch between full-precision accumulators and truncated debts.

Because this logic is used by the `Distributor` for launchpad LP reward flows, the bug can affect the distribution of reward tokens across all stakers in that pool.
 ### Static Signals
share-based rewards using accRewardPerShare, uint96 storage for shares and rewardDebt, casts from uint256 to uint96 without bounds check, PRECISION_FACTOR 1e12 with large potential accumulators
 ### Assets at Risk
rewards, reward pool token balances in Distributor
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: RewardsTrackerLib.stake / unstake / claim

 ### Title
Reward accounting uses unsafe uint256→uint96 downcasts which can overflow and corrupt debts
 ### Description/Code Snippet
The rewards accounting library `RewardsTrackerLib` repeatedly downcasts full-precision accumulated rewards (`uint256`) into `uint96` debt fields without any range checks. This can silently overflow after enough rewards have accrued, breaking the core accounting invariant and potentially allowing over/under-claims.

Relevant code (all in `RewardsTrackerLib`):

```solidity
// stake()
userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));

// unstake()
userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));

// claim()
uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);
...
userData.baseRewardDebt = uint96(totalAccBaseRewards);
userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
```

`totalAccRewards` returns `uint256` and can grow over time as more rewards are added. Once it exceeds `2**96-1` (~7.9e28), the cast to `uint96` will wrap, making `baseRewardDebt`/`quoteRewardDebt` much smaller than the true accumulated rewards. Subsequent calls will compute:

```solidity
pending = totalAccRewards(shares, accPerShare) - userData.baseRewardDebt;
```

With a wrapped `baseRewardDebt`, `pending` may become far larger than the actually intended accrued amount (or even underflow in intermediate reasoning), enabling:

* Some users to repeatedly claim more than their fair share of rewards once the debt wraps.
* Or conversely, some users to be unable to fully realize their accrued rewards because the system believes they have already claimed more than they actually did.

Because `Distributor` relies on `RewardsTrackerLib` for all staking/claim logic and holds real ERC20 rewards, this is a concrete asset-risking issue over long lifetimes or with high reward emission rates.
 ### Static Signals
uint256 totalAccRewards(...) cast directly to uint96 without bounds check, repeated writes to uint96 debt fields from growing uint256 accumulators, no cap or sanity check on total accumulated rewards per share
 ### Assets at Risk
rewards, user reward balances
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair.swap

 ### Title
Launchpad AMM swaps can be DOSed when rewards shares hit zero
 ### Description/Code Snippet
The custom launchpad fee distribution path in `GTELaunchpadV2Pair` introduces a griefable callback to the external `Distributor` contract. Under realistic conditions, this callback can revert and brick core AMM functions (swaps, mints, burns, sync) until a privileged actor intervenes.

### Code path in the pair

In `swap`:
```solidity
function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external lock {
    ...
    uint256 balance0Adjusted = balance0.mul(1000).sub(amount0In.mul(3));
    uint256 balance1Adjusted = balance1.mul(1000).sub(amount1In.mul(3));

    if (balance0Adjusted.mul(balance1Adjusted) < uint256(_reserve0).mul(_reserve1).mul(1000 ** 2)) {
        revert("UniswapV2: K");
    }

    (uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
        && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));

    _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
    ...
}
```

The critical logic is in `_update`:
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

If enough time has elapsed and `totalLaunchpadFee0|totalLaunchpadFee1 > 0`, `_update` *unconditionally* calls `_distributeLaunchpadFees`, which in turn *unconditionally* calls `IDistributor(distributor).addRewards(...)` with **no try/catch** and no fallback.

### Distributor side: revert condition when there are no shares

The `Distributor.addRewards` implementation (abridged):
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
    ...
}
```

`rs.totalShares` is the total staking shares across all users for this launch asset. If all users have fully unstaked (e.g. after bonding/unlock), it is possible and likely for `rs.totalShares` to become **zero** while the Uniswap pair is still configured with a non-zero `launchpadFeeDistributor` and `rewardsPoolActive > 0`.

In that state, when the pair eventually attempts to distribute accumulated launchpad swap fees, `addRewards` will revert with `NoSharesToIncentivize()`.

### Effect: AMM DoS triggered by a user-controlled state

Under these conditions:
- `launchpadFeeDistributor != address(0)` (as set in `initialize`),
- `rewardsPoolActive > 0` (i.e. `endRewardsAccrual()` has not been called),
- the curve has been used and there are accumulated `totalLaunchpadFee0`/`1` > 0,
- but **`rs.totalShares == 0`** in `Distributor` for the launch asset (all users have exited the staking pool),

then the *first* call into any of the following that causes `_update` to enter the fee-distribution branch **will revert**:
- `swap(...)`
- `mint(...)`
- `burn(...)`
- `sync()`

The revert bubbles up from `IDistributor.addRewards` through `_distributeLaunchpadFees` and `_update` back into `swap`/`mint`/`burn`/`sync`, making the AMM pool effectively unusable for trading until a privileged launchpad/admin action calls `Distributor.endRewards(pair)` → `pair.endRewardsAccrual()` to zero out `rewardsPoolActive` and stop accrual.

This is a classic **griefable callback** pattern:
- Core protocol function (`swap`) depends on successful execution of an external callback (`addRewards`).
- The callback can revert based on *user-controlled* or at least *non-privileged* state (`totalShares` can drop to 0 purely from normal user unstaking).
- There is no isolation or try/catch; failures in the callback brick the entire core flow until governance intervenes.

Even if governance is trusted, the attack surface is that any final staker (or set of stakers) can choose to completely unstake, pushing `totalShares` to zero and thereby **DoSing the AMM pool** for all users until an admin transaction is sent. On a high-throughput chain where availability and continuous trading are goals, this is undesirable and fits the contest’s DoS/griefing category.

A robust pattern would:
- Either **skip** distribution when `totalShares == 0` (and let fees remain as protocol-owned LP or accumulate until re-activated), or
- Wrap the external call in a `try/catch` and fall back to storing `accruedLaunchpadFee*` without reverting core operations.

 ### Static Signals
external hook to Distributor without try/catch, callback success required for swap/mint/burn/sync to complete, callback behavior depends on user-controlled totalShares in rewards pool
 ### Assets at Risk
AMM liquidity usability, trader execution (launchpad pair swaps), launchpad fee distribution continuity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
Launchpad fee callback to Distributor can revert and DoS all swaps when no shares
 ### Description/Code Snippet
The launchpad-aware Uniswap V2 pair `GTELaunchpadV2Pair` calls into an external `launchpadFeeDistributor` inside `_update` to distribute accumulated launchpad fees:

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

    uint32 blockTimestamp = uint32(block.timestamp % 2 ** 32);
    uint32 timeElapsed = blockTimestamp - blockTimestampLast;
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

`launchpadFeeDistributor` is expected to be the `Distributor` contract. Its `addRewards` implementation can revert when there are no more staking shares:

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

Scenario:
- A valid rewards pool exists and has stakers (`totalShares > 0`). The pair accumulates launchpad swap fees over time.
- Eventually all users (or the launchpad flow) fully exit their stake so `rs.totalShares` becomes `0` while rewards accrual on the pair is still active (`rewardsPoolActive > 0` and `launchpadFeeDistributor != address(0)`).
- On the next block where a swap/mint/burn/sync triggers `_update` with `timeElapsed > 0` and non-zero `totalLaunchpadFee0/1`, `_distributeLaunchpadFees` is called.
- `Distributor.addRewards` sees `rs.totalShares == 0` and reverts with `NoSharesToIncentivize()`.
- This revert bubbles up and **reverts the entire `_update`**, meaning the whole outer operation (`swap`, `mint`, `burn`, `sync`) fails.

Because `_update` is called in the hot path of core AMM functions:

```solidity
function swap(...) external lock {
    ...
    _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
    ...
}

function mint(address to) external lock returns (uint256 liquidity) {
    ...
    _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
    ...
}

function burn(address to) external lock returns (uint256 amount0, uint256 amount1) {
    ...
    _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
    ...
}

function sync() external lock {
    _update(...);
}
```

once the condition `rs.totalShares == 0` is met while there are still launchpad fees to distribute, **any subsequent swap/mint/burn/sync that crosses a block boundary and has non-zero `totalLaunchpadFee*` will revert**. This can be triggered purely by normal user behaviour (all stakers unstaking) and is independent of any misconfiguration by a privileged role.

Until the launchpad (trusted admin) calls `Distributor.endRewards` → `pair.endRewardsAccrual()` to disable rewards accrual, the AMM pool is effectively bricked: no swaps or liquidity operations can succeed if they attempt to distribute fees through the now-unincentivized rewards pool. This is a classic griefable-callback / DoS pattern where an external module’s revert condition makes a core protocol path unusable.

Assets at risk include the **entire AMM liquidity pool and all traders**, as the market can become non-functional (no swaps, no burns) solely based on the staking pool’s share count reaching zero.
 ### Static Signals
external callback to untrusted/complex contract in core path, no try/catch or fallback around callback, callback success required for swap/mint/burn/sync to succeed, callback (Distributor.addRewards) reverts when totalShares==0
 ### Assets at Risk
AMM liquidity, traders, launchpad rewards distribution continuity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
EIP-2612 permit accepts malleable signatures (no s/v validation)
 ### Description/Code Snippet
The UniswapV2-style LP token implements EIP-2612 `permit` without enforcing the standard secp256k1 malleability constraints on the signature parameters `v` and `s`.

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
            keccak256(
                abi.encode(
                    PERMIT_TYPEHASH,
                    owner,
                    spender,
                    value,
                    nonces[owner]++,
                    deadline
                )
            )
        )
    );

    address recoveredAddress = ecrecover(digest, v, r, s);
    require(
        recoveredAddress != address(0) && recoveredAddress == owner,
        "UniswapV2: INVALID_SIGNATURE"
    );
    _approve(owner, spender, value);
}
```

Issues:
- **No low-s check**: There is no requirement that `s` lies in the lower half of the secp256k1 curve order (i.e. `s <= secp256k1n / 2`). This means two different signatures (`(v,r,s)` and `(v^1,r,n-s)`) are both considered valid for the same logical message.
- **No v normalization/validation**: `v` is passed directly to `ecrecover` without verifying that it is in `{27,28}` (or normalizing `{0,1}` to that range). Some clients or tooling may accept different v encodings; combined with malleable `s` this can lead to multiple distinct encodings of a single logical signature.

Security impact pattern:
- Malleable signatures make the permit domain **non-unique per logical authorization**, which weakens assumptions for off-chain signing and meta-tx flows and can complicate replay resistance across layers or chains.
- Any external system that relies on the uniqueness of a given `(owner, spender, value, nonce, deadline)` signature (e.g. off-chain relayers, aggregators, or cross-chain bridges) can be confused or griefed by alternative but still-valid encodings.

While this pattern mirrors classic Uniswap V2 behaviour, it is still a valid `PermitMisuse` pattern: the implementation diverges from the stricter EIP-2612 recommendations and modern best practices, and accepts malleable signatures.
 ### Static Signals
no deadline check beyond >= now (ok but minimal), ecrecover used without requiring s <= secp256k1n/2, v not validated or normalized to {27,28}
 ### Assets at Risk
LP token allowances, downstream integrations relying on unique permits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

