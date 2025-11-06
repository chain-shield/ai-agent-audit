# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Pattern


 **Derived From** : Let dupCount be the number of tickets in _tickets that TicketComboTracker.insert marks as duplicate. Then: drawingState[currentDrawingId].prizePool_after - drawingState[currentDrawingId].prizePool_before == dupCount * (drawingState[currentDrawingId].ticketPrice - drawingState[currentDrawingId].edgePerTicket)

[M-1]. Duplicate ticket prizePool math ignores referralFee, enabling settlement underflow/DoS when referralFee > lpEdgeTarget
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : Referral rounding dust systematically lost (not reallocated) and harms LP economics

[L-2]. Precision-dust leak in Jackpot._validateAndTrackReferrals/_payReferrersWinnings undercuts LP earnings via per-split truncation
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : entropyBaseGasLimit + entropyVariableGasLimit * drawingState[currentDrawingId].bonusballMax <= 4294967295

[L-3]. Overflow in entropy gas limit calculation DoS-es runJackpot and fee quoting, freezing drawings
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : bonusballMax not bounded → bit-pack overflow corrupts matching when ballMax+bonusballMax > 255

[H-4]. Bonusball bit-pack overflow makes every ticket look like a bonusball winner and misclassifies tiers
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : Settlement can be bricked by external token transfer in scaledEntropyCallback

[M-5]. Jackpot.scaledEntropyCallback can be DoS’d by ERC20 fee transfer revert, leaving jackpot locked
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Emergency refunds use current referralFee instead of epoch snapshot

[M-6]. Emergency refunds in Jackpot.emergencyRefundTickets use mutable referralFee, causing over/under-refunds if fee changed mid-drawing
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: RequiresAdminRole



 **Derived From** : Emergency refunds leave tracker counts → LP pays phantom winners after drawing resumes

[M-7]. Jackpot.emergencyRefundTickets does not remove combos; later settlement counts refunded tickets as winners and debits LP (phantom payouts)
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: RequiresAdminRole



 **Derived From** : Past drawings become unclaimable after payoutCalculator update (no per-drawing snapshot)

[M-8]. Historic winners can be paid 0 after admin updates payoutCalculator; claimWinnings reads from wrong calculator
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: RequiresAdminRole



 **Derived From** : Referral rounding dust shaved from winners on claim accumulates in contract

[L-9]. Referrer-share rounding dust in Jackpot.claimWinnings permanently strands value in contract and underpays winners
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 5
- L: 3
- I: 0

##Findings by Pattern


 **Derived From** : Let dupCount be the number of tickets in _tickets that TicketComboTracker.insert marks as duplicate. Then: drawingState[currentDrawingId].prizePool_after - drawingState[currentDrawingId].prizePool_before == dupCount * (drawingState[currentDrawingId].ticketPrice - drawingState[currentDrawingId].edgePerTicket)

## [M-1]. Duplicate ticket prizePool math ignores referralFee, enabling settlement underflow/DoS when referralFee > lpEdgeTarget

### Finding Severity Justification: If referralFee is configured greater than lpEdgeTarget, duplicate-ticket handling can inflate prizePool faster than economically backed LP earnings. In the extreme (e.g., premium weights concentrated on a tier the attacker duplicates and that tier wins), userWinnings can approach prizePool and make postDrawLpValue = lpPoolTotal + lpEarnings − userWinnings underflow, reverting settlement and locking the drawing. Impact is availability loss (drawing bricked until emergency), which meets Medium. Likelihood is low because it requires specific governance configuration and unfavorable randomness, but it is enabled by allowed parameters and contradicts stated LP edge guarantees.
## Derived From Pattern/Invariant
Let dupCount be the number of tickets in _tickets that TicketComboTracker.insert marks as duplicate. Then: drawingState[currentDrawingId].prizePool_after - drawingState[currentDrawingId].prizePool_before == dupCount * (drawingState[currentDrawingId].ticketPrice - drawingState[currentDrawingId].edgePerTicket)

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In Jackpot.buyTickets, duplicate purchases increase prizePool by (ticketPrice - edgePerTicket), while lpEarnings are increased by (ticketPrice - referralFee). When referralFee exceeds lpEdgeTarget, each duplicate increases prizePool more than lpEarnings by (referralFee - edgePerTicket). An attacker can buy enough duplicates so that, at settlement, postDrawLpValue = lpPoolTotal + lpEarnings - userWinnings - protocolFee becomes negative, causing an underflow revert in JackpotLPManager.processDrawingSettlement and bricking the drawing. This breaks the accounting invariant tying prizePool growth to economically-backed funds. Vulnerable snippets: 

- Edge per ticket is precomputed: edgePerTicket = lpEdgeTarget * ticketPrice / PRECISE_UNIT (Jackpot._setNewDrawingState)
- Duplicate adds to prize pool ignoring referrals: _currentDrawingState.prizePool += _currentDrawingState.ticketPrice - _currentDrawingState.edgePerTicket; (Jackpot._validateAndStoreTickets)
- Earnings exclude referralFee: currentDrawingState.lpEarnings += ticketsValue - referralFeeTotal; (Jackpot.buyTickets)
- Settlement uses subtraction without guard: uint256 postDrawLpValue = currentLP.lpPoolTotal + _lpEarnings - _userWinnings - _protocolFeeAmount; (JackpotLPManager.processDrawingSettlement)

If referralFee > lpEdgeTarget, then per duplicate: ΔprizePool − ΔlpEarnings = (ticketPrice − edgePerTicket) − (ticketPrice − referralFee) = (referralFee − edgePerTicket) > 0. With enough duplicates D, even in the conservative worst-case userWinnings ≤ prizePool, the settlement lower bound becomes postDrawLpValue ≥ lpPoolTotal*reserveRatio + D*(edgePerTicket − referralFee), which is negative once D*(referralFee − edgePerTicket) > lpPoolTotal*reserveRatio. A permissionless buyer can force this condition and brick settlement.

## Impact
If referralFee is configured greater than lpEdgeTarget, each duplicate ticket increases prizePool by more than the LP’s economically-backed earnings, creating a per-duplicate deficit of ticketPrice*(referralFee − lpEdgeTarget). With enough duplicates (often a small number), and if the duplicated combination wins a highly weighted tier (e.g., jackpot), userWinnings approaches the enlarged prizePool. Settlement then computes postDrawLpValue = lpPoolTotal + lpEarnings − userWinnings − protocolFee, which underflows and reverts. The drawing becomes un-settleable (DoS) until emergency procedures are used, locking winnings and LP operations for all users.

## Command to Run Test


## Proof of Concept
Preconditions and attack steps:
1) Governance misconfigures fees so that referralFee > lpEdgeTarget (e.g., 10% referral, 1% LP edge). This is currently allowed by setReferralFee / setLpEdgeTarget.
2) Configure payouts so the jackpot tier receives the entire premium allocation (premiumTierWeights[11] = 1e18). This makes userWinnings ≈ prizePool when the duplicated combo wins.
3) Attacker buys one unique ticket of a specific combination with a referral scheme (to incur referral fees), then buys D duplicates of the same exact combination. Each duplicate updates:
   • prizePool += ticketPrice − edgePerTicket
   • lpEarnings += ticketPrice − (ticketPrice * referralFee)
   Therefore, per duplicate: Δ(prizePool − (lpPoolTotal + lpEarnings)) = ticketPrice*(referralFee − lpEdgeTarget) > 0.
4) With reserveRatio = 0, after ≈ ceil((ticketPrice*(1−referralFee)) / (ticketPrice*(referralFee−lpEdgeTarget))) duplicates, prizePool exceeds lpPoolTotal + lpEarnings. For 10% referral and 1% edge, threshold ≈ 11 duplicates (since 0.9/(0.09)=10).
5) If randomness selects the attacker’s duplicated combination for the jackpot, the jackpot tier’s user winners equal total winners (1 unique + D duplicates), so userWinnings ≈ prizePool. The settlement calculation underflows: postDrawLpValue = lpPoolTotal + lpEarnings − userWinnings (protocolFee=0 in this case) and reverts, bricking the drawing.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {GuaranteedMinimumPayoutCalculator} from "contracts/GuaranteedMinimumPayoutCalculator.sol";
import {ScaledEntropyProvider} from "contracts/ScaledEntropyProvider.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockUSDC is IERC20 {
    string public name = "MockUSDC"; string public symbol = "USDC"; uint8 public decimals = 6;
    uint256 public totalSupply; mapping(address=>uint256) public balanceOf; mapping(address=>mapping(address=>uint256)) public allowance;
    function transfer(address to,uint256 amt) external returns(bool){balanceOf[msg.sender]-=amt;balanceOf[to]+=amt;return true;}
    function approve(address sp,uint256 amt) external returns(bool){allowance[msg.sender][sp]=amt;return true;}
    function transferFrom(address f,address t,uint256 a) external returns(bool){uint256 al=allowance[f][msg.sender];require(al>=a,"allowance");allowance[f][msg.sender]=al-a;balanceOf[f]-=a;balanceOf[t]+=a;return true;}
    function mint(address to,uint256 amt) external {balanceOf[to]+=amt;totalSupply+=amt;}
}

contract MockEntropyV2 {
    function getFeeV2(address, uint32) external pure returns (uint128) { return 0; }
}

contract ReferralVsEdge_DuplicateDoS_Test is Test {
    MockUSDC usdc; Jackpot jackpot; JackpotLPManager lpMgr; JackpotTicketNFT nft;
    GuaranteedMinimumPayoutCalculator calc; ScaledEntropyProvider entropy;
    address owner = address(0xA11CE); address attacker = address(0xBEEF);

    function setUp() public {
        vm.startPrank(owner);
        usdc = new MockUSDC();
        // lpEdgeTarget = 1% (1e16), reserveRatio = 0, referralFee set later to 10%
        jackpot = new Jackpot(1, 35, 1, 1e16, 0, 0, 0, 0, 0, 1_000_000, 4, 100000);
        lpMgr = new JackpotLPManager(jackpot);
        nft = new JackpotTicketNFT(jackpot);
        // Premium weights: 100% to jackpot tier (tier 11) so userWinnings ≈ prizePool when it hits
        bool[12] memory mins;
        uint256[12] memory weights; weights[11] = 1e18;
        calc = new GuaranteedMinimumPayoutCalculator(jackpot, 0, 0, mins, weights);
        MockEntropyV2 mev2 = new MockEntropyV2();
        entropy = new ScaledEntropyProvider(address(mev2), address(0xE1));

        jackpot.initialize(IERC20(address(usdc)), lpMgr, nft, entropy, calc);
        jackpot.initializeLPDeposits(type(uint256).max/2);

        // Seed LP with 10,000 USDC
        usdc.mint(owner, 10_000_000_000);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(10_000_000_000);
        jackpot.initializeJackpot(block.timestamp - 2);

        // Set referralFee = 10% > lpEdgeTarget = 1%
        jackpot.setReferralFee(10e16);
        vm.stopPrank();

        // Attacker funding
        vm.startPrank(attacker);
        usdc.mint(attacker, 1_000_000_000_000);
        usdc.approve(address(jackpot), type(uint256).max);
    }

    function _ticket(uint8 a,uint8 b,uint8 c,uint8 d,uint8 e,uint8 bonus) internal pure returns (IJackpot.Ticket memory t){
        uint8[] memory ns = new uint8[](5); ns[0]=a; ns[1]=b; ns[2]=c; ns[3]=d; ns[4]=e; t = IJackpot.Ticket({normals: ns, bonusball: bonus});
    }

    function test_ReferralGreaterThanEdge_Duplicates_CauseSettlementUnderflow() public {
        // 1) Buy the first unique ticket for the exact combo with a referral
        IJackpot.Ticket[] memory t = new IJackpot.Ticket[](1);
        t[0] = _ticket(1,2,3,4,5,1);
        address[] memory refs = new address[](1); refs[0]=address(0xC0FFEE);
        uint256[] memory splits = new uint256[](1); splits[0]=1e18;
        jackpot.buyTickets(t, attacker, refs, splits, bytes32(0));

        // 2) Buy enough duplicates so that prizePool > lpPoolTotal + lpEarnings
        // Threshold for referral=10%, edge=1% is >10 duplicates; use 12 to be safe
        uint256 dupCount = 12;
        IJackpot.Ticket[] memory batch = new IJackpot.Ticket[](dupCount);
        for (uint256 i=0;i<dupCount;i++){ batch[i] = _ticket(1,2,3,4,5,1); }
        jackpot.buyTickets(batch, attacker, refs, splits, bytes32(0));

        // 3) Lock drawing and force winning numbers to match the duplicated combo
        vm.prank(owner); jackpot.lockJackpot();
        uint256[][] memory nums = new uint256[][](2);
        nums[0] = new uint256[](5); nums[0][0]=1; nums[0][1]=2; nums[0][2]=3; nums[0][3]=4; nums[0][4]=5;
        nums[1] = new uint256[](1); nums[1][0]=1;

        // 4) Set entropy and call the callback as the provider to trigger settlement
        vm.prank(owner); jackpot.setEntropy(IScaledEntropyProvider(address(entropy)));
        vm.prank(address(entropy));
        vm.expectRevert();
        jackpot.scaledEntropyCallback(bytes32(0), nums, bytes(""));
    }
}


## Suggested Mitigation
Fix the accounting mismatch between duplicate-driven prizePool growth and LP-backed earnings:
- Enforce parameter safety: require(referralFee <= lpEdgeTarget) in setReferralFee and symmetrically require(_lpEdgeTarget >= referralFee) in setLpEdgeTarget. This guarantees per-duplicate ΔprizePool ≤ ΔlpEarnings.
- Or, make duplicate accrual net-referral-aware: when a duplicate is purchased, update prizePool by ticketPrice − max(edgePerTicket, ticketPrice*referralFee). This caps per-duplicate prizePool growth to the economically backed amount and preserves LP solvency even if referralFee > lpEdgeTarget.
- Optionally add a settlement guard in JackpotLPManager.processDrawingSettlement that reverts with a descriptive error if _userWinnings + _protocolFeeAmount > currentLP.lpPoolTotal + _lpEarnings (instead of unchecked underflow), so issues fail fast with a clear reason while you deploy a governance fix.

Either the parameter constraint or the adjusted duplicate formula fully removes the underflow/DoS vector; applying both provides defense-in-depth.





 **Derived From** : Referral rounding dust systematically lost (not reallocated) and harms LP economics

## [L-2]. Precision-dust leak in Jackpot._validateAndTrackReferrals/_payReferrersWinnings undercuts LP earnings via per-split truncation

### Finding Severity Justification: The behavior is real: per-referrer integer division truncation leaves a small remainder unallocated, so lpEarnings is reduced by the full referralFeeTotal while only the floored sum is credited to referrers. The same occurs for referral win-shares on claims. However, the impact is strictly dust-level: the unallocated remainder per action is bounded by (maxReferrers - 1) USDC wei (1e-6) and cannot accumulate to material loss under realistic usage. The docs explicitly accept small rounding dust left in the contract balance. This does not create an insolvency path or enable fund theft; it is a minor accounting mismatch against LP that falls under acceptable rounding policy.
## Derived From Pattern/Invariant
Referral rounding dust systematically lost (not reallocated) and harms LP economics

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._validateAndTrackReferrals

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In buyTickets and claimWinnings, referral amounts are split per referrer as: referrerFee = total * split[i] / 1e18 and summed. Due to integer truncation, Σ floor(total*split[i]/1e18) ≤ total, leaving a dust remainder that is never reassigned. In buyTickets: lpEarnings is reduced by referralFeeTotal (ticketsValue * referralFee / 1e18), but only Σfloors is credited to referrers; the difference silently disappears (not re-credited to LP or anyone). In claimWinnings: winner receives (winningAmount − referrerShare), but Σfloors of referrerShare is credited; the difference is again silently lost. An attacker can maximize dust by passing many small splits (subject to maxReferrers) so that per-split truncation discards more units (≈ up to number_of_splits−1 USDC min-units per event). Over time and volume, this drains LP value and creates a growing accounting mismatch between contract USDC balance and recorded LP value. Vulnerable snippets:

- _validateAndTrackReferrals:
  referralFeeTotal = _ticketsValue * referralFee / PRECISE_UNIT;
  ...
  referrerFee = referralFeeTotal * _referralSplit[i] / PRECISE_UNIT;
  referralFees[_referrers[i]] += referrerFee;
  // no remainder allocation after loop

- _payReferrersWinnings:
  referrerShare = _winningAmount * _referralWinShare / PRECISE_UNIT;
  ...
  referrerFee = referrerShare * referralScheme.referralSplit[i] / PRECISE_UNIT;
  referralFees[referrer] += referrerFee;
  // no remainder allocation after loop

## Impact
Per-split integer truncation causes a small unassigned remainder whenever referral fees are split among multiple referrers. Two places are affected: (1) buyTickets: lpEarnings is reduced by referralFeeTotal, while only the floored sum of per-referrer credits is recorded; the remainder is not re-credited to LP. (2) claimWinnings: the winner pays referrerShare (reduced from their winnings), but only the floored sum of referrer credits is recorded; the remainder is lost from the winner’s payout. The dust per call is strictly bounded by (number_of_referrers − 1) units of the smallest USDC unit (1e-6). Over time, this creates a small, growing accounting mismatch between on-chain USDC and tracked obligations but does not create an insolvency path or direct profit vector for attackers.

## Command to Run Test


## Proof of Concept
Setup and reproduce per-split truncation dust on buyTickets:
1) Initialize Jackpot with referralFee > 0 (e.g., 25%) and allow ticket purchases. Ensure there is initial LP capital deposited and jackpot initialized.
2) Create a referral scheme with many referrers (e.g., 37) and PRECISE_UNIT-weight splits summing to 1e18 (equal weights with last adjusted by the remainder).
3) Buy a single ticket with this scheme. Let price = ticketPrice and referralFeeTotal = price * referralFee / 1e18.
4) Sum the credited referral balances Σ referralFees[referrer[i]] and compute dust = referralFeeTotal − Σcredited.
5) Observe drawingState.lpEarnings before and after purchase: actual delta is price − referralFeeTotal. The LP’s economically correct delta should be price − Σcredited. Therefore, (price − Σcredited) − (price − referralFeeTotal) = referralFeeTotal − Σcredited = dust > 0. This proves the mismatch and the dust leak from LP on purchases. The same per-split truncation applies during claimWinnings, where the winner’s net is reduced by referrerShare but Σcredited < referrerShare by the dust amount.

## Proof of Code
pragma solidity ^0.8.28;
import "forge-std/Test.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {GuaranteedMinimumPayoutCalculator} from "contracts/GuaranteedMinimumPayoutCalculator.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";

contract MockUSDC is IERC20 {
    string public name = "MockUSDC";
    string public symbol = "mUSDC";
    uint8 public decimals = 6;
    mapping(address=>uint256) public override balanceOf;
    mapping(address=>mapping(address=>uint256)) public override allowance;
    uint256 public override totalSupply;
    function transfer(address to, uint256 amount) external override returns (bool){
        balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true;
    }
    function approve(address spender, uint256 amount) external override returns (bool){
        allowance[msg.sender][spender] = amount; return true;
    }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool){
        uint256 a = allowance[from][msg.sender]; require(a>=amount, "allow");
        allowance[from][msg.sender] = a - amount; balanceOf[from] -= amount; balanceOf[to] += amount; return true;
    }
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; }
}

contract DummyEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64){ return 0; }
    function getFee(uint32) external pure returns (uint256){ return 0; }
}

contract ReferralDustLeakTest is Test {
    MockUSDC usdc;
    Jackpot jackpot;
    JackpotLPManager lpman;
    JackpotTicketNFT nft;
    GuaranteedMinimumPayoutCalculator calc;
    DummyEntropy entropy;

    address owner = address(0xA11CE);
    address lp = address(0xBEEF);
    address buyer = address(0xC0FFEE);

    function setUp() public {
        vm.startPrank(owner);
        usdc = new MockUSDC();
        jackpot = new Jackpot({
            _drawingDurationInSeconds: 60,
            _normalBallMax: 35,
            _bonusballMin: 4,
            _lpEdgeTarget: 1e17,
            _reserveRatio: 2e17,
            _referralFee: 25e16,        // 25%
            _referralWinShare: 2e17,    // 20%
            _protocolFee: 5e16,         // 5%
            _protocolFeeThreshold: 0,
            _ticketPrice: 10_000_000,   // 10 USDC (6 decimals)
            _maxReferrers: 50,
            _entropyBaseGasLimit: 500000
        });
        lpman = new JackpotLPManager(IJackpot(address(jackpot)));
        nft = new JackpotTicketNFT(IJackpot(address(jackpot)));
        entropy = new DummyEntropy();

        bool[12] memory minTiers;
        uint256[12] memory weights;
        for (uint i=0;i<12;i++){ weights[i] = (i==11) ? 1e18 : 0; }
        calc = new GuaranteedMinimumPayoutCalculator(IJackpot(address(jackpot)), 0, 0, minTiers, weights);

        jackpot.initialize(IERC20(address(usdc)), IJackpotLPManager(address(lpman)), IJackpotTicketNFT(address(nft)), IScaledEntropyProvider(address(entropy)), calc);
        jackpot.initializeLPDeposits(type(uint256).max/2);

        usdc.mint(lp, 1_000_000_000);    // 1,000 USDC
        usdc.mint(buyer, 1_000_000_000); // 1,000 USDC

        vm.stopPrank();
        vm.startPrank(lp);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(500_000_000); // 500 USDC
        vm.stopPrank();

        vm.startPrank(owner);
        jackpot.initializeJackpot(block.timestamp + 3600);
        vm.stopPrank();

        vm.startPrank(buyer);
        usdc.approve(address(jackpot), type(uint256).max);
        vm.stopPrank();
    }

    function test_dust_leaks_from_LP_on_buy_due_to_per_split_truncation() public {
        vm.prank(owner);
        jackpot.setMaxReferrers(37);

        // construct 1 ticket
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
        tickets[0].normals = new uint8[](5);
        tickets[0].normals[0]=1; tickets[0].normals[1]=2; tickets[0].normals[2]=3; tickets[0].normals[3]=4; tickets[0].normals[4]=5;
        tickets[0].bonusball = 1;

        // 37-way split summing to 1e18
        uint256 n = 37;
        address[] memory refs = new address[](n);
        uint256[] memory splits = new uint256[](n);
        uint256 base = 1e18 / n; // floored base
        uint256 sum;
        for (uint256 i=0;i<n;i++){
            refs[i] = address(uint160(0x1000 + i));
            splits[i] = base;
            sum += base;
        }
        if (sum != 1e18){ splits[n-1] += (1e18 - sum); }

        uint256 drawingId = jackpot.currentDrawingId();
        Jackpot.DrawingState memory s1 = jackpot.getDrawingState(drawingId);
        uint256 price = jackpot.ticketPrice();
        uint256 referralFeeTotal = price * 25e16 / 1e18; // 10 USDC * 25% = 2.5 USDC

        vm.startPrank(buyer);
        jackpot.buyTickets(tickets, buyer, refs, splits, bytes32("src"));
        vm.stopPrank();

        // sum credited referrer balances
        uint256 credited;
        for (uint256 i=0;i<n;i++){
            credited += jackpot.referralFees(refs[i]);
        }
        uint256 dust = referralFeeTotal - credited;
        assertGt(dust, 0, "expected dust > 0");

        Jackpot.DrawingState memory s2 = jackpot.getDrawingState(drawingId);
        uint256 lpDelta = s2.lpEarnings - s1.lpEarnings;               // actual accounting used
        uint256 expected = price - credited;                           // economically correct LP revenue
        assertEq(expected - lpDelta, dust, "LP loses dust due to rounding");
    }
}


## Suggested Mitigation
Ensure the full computed totals are allocated by handling the per-split remainder explicitly:
- In _validateAndTrackReferrals: track allocatedSum while looping. After the loop, compute remainder = referralFeeTotal − allocatedSum and credit it to one referrer (e.g., the last or the largest-weight referrer). This guarantees Σallocated == referralFeeTotal so lpEarnings = ticketsValue − referralFeeTotal matches the actual credited amount.
- In _payReferrersWinnings: similarly, track allocatedSum of referrerShare and allocate the remainder to one referrer in the scheme. If the scheme is empty (bytes32(0)), current logic already credits the full referrerShare to LP, so no remainder exists in that branch.
Optionally, to avoid bias, allocate the remainder to the referrer with the highest split, or deterministically to the last referrer. An alternative acceptable pattern is to assign the remainder back to LP for purchases (but then adjust lpEarnings using allocatedSum instead of referralFeeTotal) and to referrers on winnings so the winner’s net matches Σcredited. The key requirement is that no remainder remains unassigned.





 **Derived From** : entropyBaseGasLimit + entropyVariableGasLimit * drawingState[currentDrawingId].bonusballMax <= 4294967295

## [L-3]. Overflow in entropy gas limit calculation DoS-es runJackpot and fee quoting, freezing drawings

### Finding Severity Justification: The issue can cause a temporary liveness DoS of runJackpot() and getEntropyCallbackFee() by overflowing the uint32 arithmetic in _calculateEntropyGasLimit. However, it requires an owner-set misconfiguration (excessively large entropyVariableGasLimit relative to bonusballMax). No assets are at risk, the state change to lock is reverted with the transaction, and recovery is trivial via an owner parameter update. Under Code4rena rules, this is governance/misconfiguration risk and thus QA/Low.
## Derived From Pattern/Invariant
entropyBaseGasLimit + entropyVariableGasLimit * drawingState[currentDrawingId].bonusballMax <= 4294967295

## Exploit Type
IntegerOverflow

## Location
Jackpot.runJackpot

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Jackpot._calculateEntropyGasLimit returns a uint32 computed as base + variable * bonusballMax. With large parameters this exceeds 2^32−1 and reverts due to Solidity 0.8 checked arithmetic on the implicit downcast to uint32. Vulnerable snippet:

function _calculateEntropyGasLimit(uint8 _bonusballMax) internal view returns (uint32) {
    return entropyBaseGasLimit + entropyVariableGasLimit * uint32(_bonusballMax);
}

Because runJackpot() first locks, then computes entropyGasLimit and fee, the overflow causes runJackpot() to revert before requesting entropy; getEntropyCallbackFee() also reverts, preventing keepers from quoting the fee. Result: drawing cannot be executed (liveness failure) until governance fixes params or engages emergency mode.

## Impact
Functional DoS: drawing cannot be run; fee quoting via getEntropyCallbackFee() reverts; users cannot progress to next drawing; LPs and winners are stuck until admin intervention.

## Command to Run Test


## Proof of Concept
Root cause: _calculateEntropyGasLimit does the multiplication and addition in uint32 arithmetic: entropyBaseGasLimit (uint32) + entropyVariableGasLimit (uint32) * uint32(_bonusballMax). With large values, the uint32 intermediate overflows and reverts (Solidity 0.8 checked arithmetic), preventing both runJackpot() and getEntropyCallbackFee() from functioning.

Reproduction steps:
1) Ensure the current drawing’s bonusballMax is high (e.g., >= 100). This occurs naturally if prizePool and parameters produce a large min ticket count, or governance sets a large bonusballMin before initializing the drawing.
2) Governance misconfigures gas params so base + variable * bonusballMax > 2^32−1. For example: entropyBaseGasLimit=500,000; entropyVariableGasLimit=43,000,000; bonusballMax=100 → 500,000 + 43,000,000*100 = 4,300,500,000 > 4,294,967,295.
3) Any caller invokes getEntropyCallbackFee() → reverts on overflow inside _calculateEntropyGasLimit.
4) Any caller invokes runJackpot() (after drawingTime) → the function locks first, then calls _calculateEntropyGasLimit and reverts on overflow before requesting entropy. The lock change is reverted with the transaction, so the drawing remains unlocked but cannot progress until governance fixes the params.

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
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    uint256 public override totalSupply;
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; totalSupply += amt; emit Transfer(address(0), to, amt); }
    function approve(address s, uint256 a) external override returns (bool){ allowance[msg.sender][s] = a; emit Approval(msg.sender, s, a); return true; }
    function transfer(address to, uint256 a) external override returns (bool){ _transfer(msg.sender, to, a); return true; }
    function transferFrom(address f,address t,uint256 a) external override returns (bool){ uint256 al = allowance[f][msg.sender]; require(al >= a, "allow"); if (al != type(uint256).max) allowance[f][msg.sender] = al - a; _transfer(f, t, a); return true; }
    function _transfer(address f,address t,uint256 a) internal { require(balanceOf[f] >= a, "bal"); balanceOf[f] -= a; balanceOf[t] += a; emit Transfer(f, t, a); }
}

contract MockEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64) { return 1; }
    function getFee(uint32) external view returns (uint256) { return 1; }
}

contract EntropyGasOverflowTest is Test {
    Jackpot jackpot;
    JackpotLPManager lpMgr;
    JackpotTicketNFT nft;
    GuaranteedMinimumPayoutCalculator calc;
    MockUSDC usdc;
    MockEntropy entropy;

    address owner = address(0xA11CE);
    address lp = address(0xBEEF);

    function setUp() public {
        vm.startPrank(owner);
        uint256 drawingDuration = 1;      // seconds
        uint8 normalBallMax = 10;         // small
        uint8 bonusballMin = 100;         // force large bonusballMax >= 100
        uint256 lpEdgeTarget = 1e17;      // 10%
        uint256 reserveRatio = 0;
        uint256 referralFee = 0;
        uint256 referralWinShare = 0;
        uint256 protocolFee = 0;
        uint256 protocolFeeThreshold = 0;
        uint256 ticketPrice = 1_000_000;  // 1 USDC (6 decimals)
        uint256 maxRefs = 3;
        uint32 entropyBaseGas = 500_000;  // base

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
            maxRefs,
            entropyBaseGas
        );
        lpMgr = new JackpotLPManager(IJackpot(address(jackpot)));
        nft = new JackpotTicketNFT(IJackpot(address(jackpot)));
        usdc = new MockUSDC();
        entropy = new MockEntropy();
        bool[12] memory minTiers;
        uint256[12] memory weights;
        weights[11] = 1e18; // sum == 1e18
        calc = new GuaranteedMinimumPayoutCalculator(IJackpot(address(jackpot)), 0, 0, minTiers, weights);

        jackpot.initialize(usdc, lpMgr, nft, entropy, calc);
        jackpot.initializeLPDeposits(1_000_000_000_000);

        // seed LP funds and deposit
        vm.stopPrank();
        usdc.mint(lp, 1_000_000_000); // 1,000 USDC
        vm.startPrank(lp);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(1_000_000_000);
        vm.stopPrank();

        // start jackpot with past drawing time so runJackpot is due
        vm.startPrank(owner);
        jackpot.initializeJackpot(block.timestamp - 1);

        // Configure dangerous gas params: 43,000,000 * 100 + 500,000 > 2^32-1
        jackpot.setEntropyVariableGasLimit(43_000_000);
        jackpot.setEntropyBaseGasLimit(500_000);
        vm.stopPrank();
    }

    function test_GetEntropyCallbackFee_RevertsOnOverflow() public {
        vm.expectRevert();
        jackpot.getEntropyCallbackFee();
    }

    function test_RunJackpot_RevertsOnOverflow_AndDoesNotLock() public {
        {
            Jackpot.DrawingState memory dsBefore = jackpot.getDrawingState(jackpot.currentDrawingId());
            assertEq(dsBefore.jackpotLock, false);
        }
        vm.expectRevert();
        jackpot.runJackpot{value: 0}();
        {
            Jackpot.DrawingState memory dsAfter = jackpot.getDrawingState(jackpot.currentDrawingId());
            assertEq(dsAfter.jackpotLock, false);
        }
    }
}


## Suggested Mitigation
Perform gas limit computation in 256-bit space and enforce an explicit upper bound with a descriptive revert; additionally validate admin inputs to prevent misconfiguration:

- Compute in uint256, then range-check:

function _calculateEntropyGasLimit(uint8 bbMax) internal view returns (uint32) {
    uint256 gas_ = uint256(entropyBaseGasLimit) + uint256(entropyVariableGasLimit) * uint256(bbMax);
    require(gas_ <= type(uint32).max, "EntropyGasLimitOverflow");
    return uint32(gas_);
}

- Tighten setters to prevent invalid states up front. For example, bound against a conservative maximum bonusball value (e.g., 255) or against the current drawing’s configured bonusballMax to avoid future overflows:

function setEntropyBaseGasLimit(uint32 v) external onlyOwner {
    // Example conservative check using 255 as an absolute upper bound for bonusball
    require(uint256(v) + uint256(entropyVariableGasLimit) * 255 <= type(uint32).max, "InvalidEntropyBase");
    entropyBaseGasLimit = v;
}

function setEntropyVariableGasLimit(uint32 v) external onlyOwner {
    require(uint256(entropyBaseGasLimit) + uint256(v) * 255 <= type(uint32).max, "InvalidEntropyVariable");
    entropyVariableGasLimit = v;
}

- Optionally, add a getter that previews the next drawing’s gas limit (based on the computed bonusballMax) and reverts early if unsafe; use the same check in getEntropyCallbackFee and runJackpot to produce a clear, protocol-specific error rather than a generic arithmetic panic.






 **Derived From** : bonusballMax not bounded → bit-pack overflow corrupts matching when ballMax+bonusballMax > 255

## [H-4]. Bonusball bit-pack overflow makes every ticket look like a bonusball winner and misclassifies tiers

### Finding Severity Justification: Bonusball is bit-packed at position (normalBallMax + bonusball). There is no clamp ensuring normalBallMax + bonusballMax ≤ 255 when computing the next drawing’s bonusballMax. If (normalBallMax + bonusball) ≥ 256, 1 << (normalBallMax + bonusball) evaluates to 0, so both tickets and the winning ticket can lose the bonusball bit. Then _calculateTicketTierId() right-shifts by (normalBallMax + 1), yielding zero for both, forcing bonusballMatch = 1 and subtracting 1 from the normal match count. This misclassifies tiers and can cause 0-match tickets to revert (underflow), and more critically, lets claimants receive tier payouts that were not budgeted for that tier, potentially exceeding the prizePool allocation and draining LP funds. The condition is realistically reachable for common parameter choices (e.g., normalBallMax ≤ 128 per Combinations.choose guard and large prize pools that imply large bonusballMax).
## Derived From Pattern/Invariant
bonusballMax not bounded → bit-pack overflow corrupts matching when ballMax+bonusballMax > 255

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
Jackpot._setNewDrawingState() computes newDrawingState.bonusballMax without clamping against the 255-bit packing limit. Tickets and the winning ticket are bit-packed with the bonusball bit at position (normalBallMax + bonusball). When normalBallMax + bonusballMax > 255, TicketComboTracker.insert() does ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax) which overflows and yields 0 for the bonus bit. The winning ticket bit does the same. In Jackpot._calculateTicketTierId(), the code shifts both packed values by (_normalBallMax + 1) and compares equality. Since both are now 0, every ticket is incorrectly treated as having a bonusball match, and the normal matches are undercounted by subtracting this bogus match. This breaks the core accounting invariant that tierId = 2*(matchedNormals) + (bonusMatch?1:0) reflects the true match state. Consequences: (1) claimWinnings misclassifies tiers, allowing users to claim higher-payout tiers than they actually won, overpaying beyond the pre-accrued userWinnings and draining the LP pool, (2) tickets with 0 normal matches will revert on claim due to underflow in (matches - bonusballMatch), causing DoS for claims, and (3) overall winner counting vs. per-claim tier payouts diverge, violating solvency invariants.

Vulnerable snippets:
- Jackpot._setNewDrawingState():
  uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
  newDrawingState.bonusballMax = newBonusball; // no clamp to 255 - normalBallMax

- TicketComboTracker.insert():
  ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax); // shift > 255 → 0

- Jackpot._calculateTicketTierId():
  uint256 ticketBonusball = _ticketNumbers >> (_normalBallMax + 1);
  uint256 winningBonusball = _winningNumbers >> (_normalBallMax + 1);
  uint256 bonusballMatch = (ticketBonusball == winningBonusball) ? 1 : 0; // always true after overflow
  return 2 * (matches - bonusballMatch) + bonusballMatch; // undercounts normals, can underflow

## Impact
If normalBallMax + bonusball ≥ 256, the packed bonusball bit is lost for both tickets and the winning ticket. During claims, _calculateTicketTierId then treats every ticket as if it had a bonusball match (because both shifted values are 0), subtracts 1 from the normal match count, and can revert for 0-match tickets (DoS). More critically, claimants are misclassified into different tiers than those budgeted at settlement, allowing overpayment across claims that can exceed the drawing’s stored userWinnings and drain LP funds. The condition is reachable because normalBallMax is effectively bounded to ≤128 by Combinations.choose, while bonusballMax can be large enough that normalBallMax + bonusballMax > 255.

## Command to Run Test


## Proof of Concept
Setup: normalBallMax=128 (valid due to Combinations.choose bound), bonusballMax ≥ 200, so some legal bonusball choices make normalBallMax + bonusball ≥ 256.
1) Drawing N+1 is initialized with bonusballMax=200 (no clamp against 255-normalBallMax).
2) Users buy tickets. Packing uses: packed = normalsBitset | (1 << (bonusball + normalMax)). For any bonusball ≥ 128, the left shift index ≥ 256, so the bonusball bit is 0 and is not recorded in the packed ticket. The winning ticket pack in settlement has the same issue.
3) Settlement computes tier payouts correctly off combo-tracker counts (keys by raw bonusball index), so the per-tier payout table is consistent with real winners.
4) On claim, _calculateTicketTierId shifts both packed values by (normalBallMax + 1); both are now 0 for the bonusball segment, so equality holds and bonusballMatch=1 for every ticket. Since matches was computed from matchingBits without the bonusball bit (it was never set), the function subtracts 1 from matches, misclassifying every ticket into a different tier. Tickets with 0 normal matches revert due to underflow; tickets with >0 normal matches are misclassified and can claim payouts for tiers that were not budgeted for them, allowing cumulative overpayment beyond drawingUserWinnings.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {LibBit} from "solady/src/utils/LibBit.sol";

contract BonusballOverflowTierTest is Test {
    // Helper: pack normals into bit vector [1..normalMax]
    function packNormals(uint8[] memory normals) internal pure returns (uint256 set) {
        for (uint256 i = 0; i < normals.length; i++) {
            set |= (uint256(1) << normals[i]);
        }
    }

    // Copy of Jackpot._calculateTicketTierId (pure logic under test)
    function _calcTier(uint256 _ticketNumbers, uint256 _winningNumbers, uint256 _normalBallMax) internal pure returns (uint256) {
        uint256 matchingBits = _ticketNumbers & _winningNumbers;
        uint256 matches = LibBit.popCount(matchingBits);
        uint256 ticketBonusball = _ticketNumbers >> (_normalBallMax + 1);
        uint256 winningBonusball = _winningNumbers >> (_normalBallMax + 1);
        uint256 bonusballMatch = (ticketBonusball == winningBonusball) ? 1 : 0;
        return 2 * (matches - bonusballMatch) + bonusballMatch;
    }

    function test_BonusballOverflow_ShiftYieldsZeroAndMisclassifies() public {
        // Configure a drawing with normalMax + bonusball >= 256 (e.g., 128 + 200)
        uint8 normalMax = 128;
        uint8 bonusball = 200; // any value >= 128 will push index >= 256

        // 1 << (normalMax + bonusball) must be zero due to >=256 shift
        uint256 shiftIndex = uint256(normalMax) + uint256(bonusball);
        assertGe(shiftIndex, 256, "precondition");
        assertEq(uint256(1) << shiftIndex, 0, "left-shift beyond 255 must yield zero");

        // Build ticket and winning numbers with the (overflowed) packed bonusball bit
        // Normals {1,2} vs winning normals {1,2,3,4,5}
        uint8[] memory t = new uint8[](2); t[0]=1; t[1]=2;
        uint8[] memory w = new uint8[](5); w[0]=1; w[1]=2; w[2]=3; w[3]=4; w[4]=5;
        uint256 ticketPacked = packNormals(t) | (uint256(1) << (uint256(bonusball) + uint256(normalMax))); // overflow to 0
        uint256 winningPacked = packNormals(w) | (uint256(1) << (uint256(bonusball) + uint256(normalMax))); // overflow to 0

        // Sanity: bonusball bits are absent (already zeroed by overflow)
        assertEq(ticketPacked, packNormals(t), "ticket bonusball bit lost");
        assertEq(winningPacked, packNormals(w), "winning bonusball bit lost");

        // Correct tier if no overflow would be: matches=2, bonusballMatch=false => tier=4
        // Buggy path: forces bonusballMatch=1, so tier=2*(2-1)+1=3
        uint256 buggyTier = _calcTier(ticketPacked, winningPacked, normalMax);
        assertEq(buggyTier, 3, "misclassified to 1-bonus tier");

        // DoS case: 0-normal-match ticket => matches=0, forced bonusballMatch=1 => underflow
        uint256 zeroMatchTicket = 0; // no normals set
        (bool reverted, ) = address(this).call(abi.encodeWithSelector(this._revertOnTierCalc.selector, zeroMatchTicket, winningPacked, uint256(normalMax)));
        assertTrue(reverted, "(matches - 1) underflow must revert for 0 matches");
    }

    function _revertOnTierCalc(uint256 a, uint256 b, uint256 c) external pure returns (uint256) {
        return _calcTier(a, b, c); // will revert when a has 0 matches but forced bonusballMatch=1
    }
}


## Suggested Mitigation
Fully prevent bit-pack overflow and harden tier calculation:
- Clamp bonusballMax at drawing initialization so normalBallMax + bonusballMax ≤ 255:
  • In _setNewDrawingState: compute target = max(bonusballMin, ceilDiv(minNumberTickets, combosPerBonusball));
  • Let maxBonus = uint256(type(uint8).max) - uint256(normalBallMax);
  • If target > maxBonus, set target = maxBonus.
  • Then safely cast: newDrawingState.bonusballMax = UintCasts.toUint8(target).
- Enforce invariant at all entry points:
  • In TicketComboTracker.init add: require(uint256(_normalMax) + uint256(_bonusballMax) <= 255, "bit-pack overflow");
  • In Jackpot.setNormalBallMax and setBonusballMin (or where admin sets values), optionally pre-validate that any future bonusball range can satisfy normalBallMax + bonusballMax ≤ 255 based on bounds, or document that the runtime clamp will apply.
  • Prefer require(_normalBallMax <= 128) in setNormalBallMax to avoid Combinations.choose assert.
- Use safe casts wherever values can exceed uint8: replace direct uint8(...) casts with UintCasts.toUint8 to avoid silent truncation.
- Defensive fix in _calculateTicketTierId to avoid underflow and false positives if an overflow ever slipped through:
  • Compute: ticketBonusball = _ticketNumbers >> (_normalBallMax + 1); winningBonusball = _winningNumbers >> (_normalBallMax + 1);
  • Set bonusballMatch = (ticketBonusball != 0 && ticketBonusball == winningBonusball) ? 1 : 0; // ensures both packed bonusball bits exist before matching
  • This prevents the always-true zero==zero case and eliminates (matches - 1) underflow for 0-match tickets.
Implementing the clamps plus the defensive check fully removes the vulnerability and adds robustness against configuration mistakes.





 **Derived From** : Settlement can be bricked by external token transfer in scaledEntropyCallback

## [M-5]. Jackpot.scaledEntropyCallback can be DoS’d by ERC20 fee transfer revert, leaving jackpot locked

### Finding Severity Justification: scaledEntropyCallback performs an external ERC20 transfer (protocol fee) inside the critical randomness settlement path. If the token transfer reverts (e.g., blacklisted/paused recipient or other ERC20 revert), the whole callback reverts and the previously set jackpotLock (from runJackpot) remains true, halting progress. Impact is protocol-wide liveness DoS for the current drawing: ticket purchases, settlement, and next-drawing initialization are blocked until owner intervention (e.g., update protocolFeeAddress or fee to 0 and re-run, or emergency flows). No immediate loss of assets occurs, hence Medium rather than High.
## Derived From Pattern/Invariant
Settlement can be bricked by external token transfer in scaledEntropyCallback

## Exploit Type
UncheckedReturn

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
scaledEntropyCallback updates critical settlement state, then calls _transferProtocolFee which performs an external ERC20 transfer: usdc.safeTransfer(protocolFeeAddress, protocolFeeAmount). If the token transfer reverts (e.g., token paused/blacklists sender/recipient or non-standard behavior), the whole callback reverts. Because jackpotLock was set in a prior tx (runJackpot), the lock is not reverted and the drawing remains permanently locked (JackpotLocked), requiring owner intervention (unlockJackpot/emergency). Vulnerable flow:

function scaledEntropyCallback(...) {
  ...
  currentDrawingState.winningTicket = winningNumbers;  // state update
  uint256 protocolFeeAmount = _transferProtocolFee(...);  // external ERC20 transfer
  ...
}

function _transferProtocolFee(...) {
  if (conditions) {
    protocolFeeAmount = ...;
    usdc.safeTransfer(protocolFeeAddress, protocolFeeAmount); // revert bricks settlement
  }
  emit ProtocolFeeCollected(...);
}

## Impact
Protocol-wide liveness DoS of the ongoing drawing; jackpot remains locked, ticket purchases and settlement cannot progress; LP/user funds become stuck until owner uses emergency/override.

## Command to Run Test


## Proof of Concept
1) A drawing is locked by runJackpot(). 2) Due to an external condition (e.g., USDC pause/blacklist to protocolFeeAddress or sender), ERC20.transfer in _transferProtocolFee reverts. 3) scaledEntropyCallback reverts, but the prior jackpotLock set by runJackpot persists, leaving the protocol stuck. 4) Subsequent runJackpot() reverts with JackpotLocked until owner intervention.

A permissionless actor can trigger runJackpot to enter lock state; if the token transfer fails during callback, the system becomes stuck without requiring any privileged actions by the attacker.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {IPayoutCalculator} from "contracts/interfaces/IPayoutCalculator.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";

contract MockUSDC is IERC20 {
    string public name = "MockUSDC";
    string public symbol = "mUSDC";
    uint8 public decimals = 6;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    uint256 public override totalSupply;

    address public denyTo;
    bool public denyEnabled;

    function setDeny(address to, bool enabled) external {
        denyTo = to; denyEnabled = enabled;
    }

    function mint(address to, uint256 amt) external {
        balanceOf[to] += amt; totalSupply += amt;
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount; return true;
    }

    function transfer(address to, uint256 amount) external override returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        if (denyEnabled && to == denyTo) revert("DENY");
        balanceOf[msg.sender] -= amount; balanceOf[to] += amount; return true;
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        require(allowance[from][msg.sender] >= amount, "allow");
        if (denyEnabled && to == denyTo) revert("DENY");
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount; balanceOf[to] += amount; return true;
    }
}

contract MockPayoutCalc is IPayoutCalculator {
    function calculateAndStoreDrawingUserWinnings(
        uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory
    ) external pure returns (uint256) {
        // return small user winnings so protocol fee > 0 when lpEarnings > 0
        return 0; // exaggerate LP earnings > user winnings
    }
    function setDrawingTierInfo(uint256) external {}
    function getTierPayout(uint256, uint256) external pure returns (uint256) { return 0; }
}

contract EntropyStub is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory)
        external payable returns (uint64) { return 0; }
    function getFee(uint32) external pure returns (uint256) { return 0; }
}

contract ExternalCallAfterStateChange_DoS_Test is Test {
    Jackpot jackpot;
    JackpotLPManager lpman;
    JackpotTicketNFT nft;
    MockUSDC usdc;
    MockPayoutCalc payout;
    EntropyStub entropy;

    address lp = address(0xBEEF);
    address buyer = address(0xA11CE);

    function setUp() public {
        // Deploy mocks
        usdc = new MockUSDC();
        payout = new MockPayoutCalc();
        entropy = new EntropyStub();

        // Jackpot params
        uint256 drawingDuration = 1 hours;
        uint8 normalMax = 35;
        uint8 bonusMin = 1;
        uint256 lpEdgeTarget = 1e17; // 10%
        uint256 reserveRatio = 0;    // simplify
        uint256 referralFee = 0;     // simplify
        uint256 referralWinShare = 0;
        uint256 protocolFee = 1e17;  // 10%
        uint256 protocolFeeThreshold = 0;
        uint256 ticketPrice = 1e6; // 1 USDC
        uint256 maxReferrers = 5;
        uint32 entropyBaseGas = 200000;

        jackpot = new Jackpot(
            drawingDuration,
            normalMax,
            bonusMin,
            lpEdgeTarget,
            reserveRatio,
            referralFee,
            referralWinShare,
            protocolFee,
            protocolFeeThreshold,
            ticketPrice,
            maxReferrers,
            entropyBaseGas
        );

        lpman = new JackpotLPManager(jackpot);
        nft = new JackpotTicketNFT(jackpot);

        // Wire dependencies
        jackpot.initialize(IERC20(address(usdc)), lpman, nft, entropy, payout);

        // Init LP system
        jackpot.initializeLPDeposits(1_000_000_000e6);

        // Fund LP and buyer
        usdc.mint(lp, 1_000_000e6);
        vm.startPrank(lp);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(500_000e6);
        vm.stopPrank();

        // Start jackpot (drawing 1)
        jackpot.initializeJackpot(block.timestamp);

        // Buyer buys a ticket to accrue lpEarnings
        usdc.mint(buyer, 100e6);
        vm.startPrank(buyer);
        usdc.approve(address(jackpot), type(uint256).max);
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
        uint8[] memory normals = new uint8[](5);
        normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        tickets[0] = IJackpot.Ticket({normals: normals, bonusball: 1});
        address[] memory refs = new address[](0);
        uint256[] memory splits = new uint256[](0);
        jackpot.buyTickets(tickets, buyer, refs, splits, bytes32(0));
        vm.stopPrank();

        // Owner locks the jackpot (simulates runJackpot lock step)
        jackpot.lockJackpot();

        // Configure USDC to revert when transferring to protocolFeeAddress
        usdc.setDeny(jackpot.protocolFeeAddress(), true);
    }

    function test_DoS_on_scaledEntropyCallback_due_to_fee_transfer_revert() public {
        // Prepare winning numbers for callback: [5 normals], [1 bonus]
        uint256[][] memory rnd = new uint256[][](2);
        rnd[0] = new uint256[](5); rnd[0][0]=1; rnd[0][1]=2; rnd[0][2]=3; rnd[0][3]=4; rnd[0][4]=5;
        rnd[1] = new uint256[](1); rnd[1][0]=1;

        // Call from entropy (onlyEntropy)
        vm.prank(address(entropy));
        vm.expectRevert();
        jackpot.scaledEntropyCallback(bytes32(0), rnd, "");

        // Jackpot remains locked, system stuck
        (
            uint256 prizePool,,,
            ,,,,
            ,,
            bool jackpotLock
        ) = _getCurrentDrawingState();
        assertTrue(jackpotLock, "jackpot should remain locked after revert");
        assertGt(prizePool, 0, "drawing still active");
    }

    function _getCurrentDrawingState() internal view returns (
        uint256 prizePool,
        uint256 ticketPrice,
        uint256 edgePerTicket,
        uint256 referralWinShare,
        uint256 globalTicketsBought,
        uint256 lpEarnings,
        uint8 ballMax,
        uint8 bonusballMax,
        uint256 drawingTime,
        uint256 winningTicket,
        bool jackpotLock
    ){
        Jackpot.DrawingState memory s = jackpot.getDrawingState(jackpot.currentDrawingId());
        return (s.prizePool, s.ticketPrice, s.edgePerTicket, s.referralWinShare, s.globalTicketsBought, s.lpEarnings, s.ballMax, s.bonusballMax, s.drawingTime, s.winningTicket, s.jackpotLock);
    }
}


## Suggested Mitigation
Avoid external token transfers inside the critical settlement callback. Options: (1) Accrue protocol fees to a payable balance (protocolFeeAccrued) and attempt the ERC20 transfer in a separate sweep function; settlement should proceed regardless of transfer success. (2) Wrap the transfer in try/catch and, on failure, record the debt for later sweep instead of reverting. (3) As an alternative, move the transfer before setting the lock-sensitive state and ensure failure does not brick the drawing, but prefer a pull/sweep pattern to fully decouple settlement from token-side failures.





 **Derived From** : Emergency refunds use current referralFee instead of epoch snapshot

## [M-6]. Emergency refunds in Jackpot.emergencyRefundTickets use mutable referralFee, causing over/under-refunds if fee changed mid-drawing

### Finding Severity Justification: Emergency refunds for referred tickets subtract the current global referralFee instead of the fee in effect when the ticket was purchased. This can materially over-refund users (LP loss) if the fee was lowered, or under-refund users (user loss) if the fee was raised, across all referred tickets in the active drawing. It does not require admin misuse (changing referralFee mid-drawing is allowed and non-retroactive), and can trigger under normal governance operations followed by a legitimate emergency. Impact can be significant in aggregate during refunds, but it is constrained to emergency mode events, thus Medium.
## Derived From Pattern/Invariant
Emergency refunds use current referralFee instead of epoch snapshot

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
In emergencyRefundTickets, the refund for referred tickets is computed using the current global referralFee instead of a per-drawing (or per-purchase) snapshot. Code: refundAmount = ticketInfo.referralScheme == bytes32(0) ? drawingState[ticketInfo.drawingId].ticketPrice : drawingState[ticketInfo.drawingId].ticketPrice * (PRECISE_UNIT - referralFee) / PRECISE_UNIT;. If governance updates referralFee after ticket purchase but before an emergency, refunds for that drawing are miscalculated. Decreasing the fee leads to over-refunds (LP loss); increasing it under-refunds users (user loss). This breaks epoch consistency and the accounting invariant that emergency refunds should reflect the fee in effect when the ticket was purchased.

## Impact
When referralFee is changed after some tickets are purchased but before an emergency refund, refunds for referred tickets are computed with the new fee instead of the fee in effect at purchase. If referralFee is lowered (e.g., 20% -> 0%), users get over-refunded by ticketPrice * (f_old - f_new) while referrers can still claim the original referral fee accrued at purchase, resulting in an aggregate outflow up to 120% of ticket price per ticket (LP loss). If referralFee is raised (e.g., 10% -> 20%), users are under-refunded by ticketPrice * (f_new - f_old), leaving excess funds stranded in the contract (user loss). The effect applies to all referred tickets in the current drawing during emergency refunds and can be material in aggregate.

## Command to Run Test


## Proof of Concept
1) Drawing starts with referralFee = 20%. Users buy referred tickets (referralFees are accrued in mapping at 20%). 2) Before settlement, governance lowers referralFee to 0% (intending it for a future epoch). 3) Owner enables emergency mode; users call emergencyRefundTickets. The contract refunds full ticketPrice for referred tickets (uses 0%), but referrers can still claim the previously accrued 20% from referralFees, causing total outflow of 120% per referred ticket. Conversely, if referralFee were raised before emergency (e.g., 10% -> 20%), the contract under-refunds users by 10% per referred ticket.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IPayoutCalculator} from "contracts/interfaces/IPayoutCalculator.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";

contract TestUSDC is IERC20 {
    string public name = "USDC";
    string public symbol = "USDC";
    uint8 public decimals = 6;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;
    uint256 public override totalSupply;
    function mint(address to, uint256 amount) external { balanceOf[to] += amount; totalSupply += amount; emit Transfer(address(0), to, amount); }
    function transfer(address to, uint256 amount) external override returns (bool) { require(balanceOf[msg.sender] >= amount, "bal"); balanceOf[msg.sender]-=amount; balanceOf[to]+=amount; emit Transfer(msg.sender,to,amount); return true; }
    function approve(address spender, uint256 amount) external override returns (bool) { allowance[msg.sender][spender]=amount; emit Approval(msg.sender,spender,amount); return true; }
    function transferFrom(address from, address to, uint256 amount) external override returns (bool) { require(allowance[from][msg.sender] >= amount, "allow"); require(balanceOf[from] >= amount, "bal"); allowance[from][msg.sender]-=amount; balanceOf[from]-=amount; balanceOf[to]+=amount; emit Transfer(from,to,amount); return true; }
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract DummyPayoutCalc is IPayoutCalculator {
    function calculateAndStoreDrawingUserWinnings(uint256, uint256, uint8, uint8, uint256[] memory, uint256[] memory) external pure returns (uint256) { return 0; }
    function setDrawingTierInfo(uint256) external {}
    function getTierPayout(uint256, uint256) external pure returns (uint256) { return 0; }
}

contract DummyEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, IScaledEntropyProvider.SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64) { return 0; }
    function getFee(uint32) external pure returns (uint256) { return 0; }
}

contract EmergencyRefundReferralFeeEpochBugTest is Test {
    TestUSDC usdc;
    Jackpot jackpot;
    JackpotLPManager lpMgr;
    JackpotTicketNFT nft;
    DummyPayoutCalc payout;
    DummyEntropy entropy;

    address lp = address(0xBEEF);
    address user = address(0x1234);

    uint256 constant PRECISE_UNIT = 1e18;
    uint256 constant TICKET_PRICE = 1_000_000; // 1 USDC (6 decimals)

    function setUp() public {
        usdc = new TestUSDC();
        uint256 drawingDuration = 1 days;
        uint8 normalBallMax = 10;
        uint8 bonusballMin = 2;
        uint256 lpEdgeTarget = 1e17; // 10%
        uint256 reserveRatio = 2e17; // 20%
        uint256 referralFeeInitial = 2e17; // 20%
        uint256 referralWinShare = 1e17; // 10%
        uint256 protocolFee = 0;
        uint256 protocolFeeThreshold = 0;
        uint256 ticketPrice = TICKET_PRICE;
        uint256 maxReferrers = 3;
        uint32 entropyBaseGas = 200_000;
        jackpot = new Jackpot(
            drawingDuration, normalBallMax, bonusballMin, lpEdgeTarget, reserveRatio,
            referralFeeInitial, referralWinShare, protocolFee, protocolFeeThreshold, ticketPrice, maxReferrers, entropyBaseGas
        );
        lpMgr = new JackpotLPManager(jackpot);
        nft = new JackpotTicketNFT(jackpot);
        payout = new DummyPayoutCalc();
        entropy = new DummyEntropy();

        jackpot.initialize(IERC20(address(usdc)), lpMgr, nft, entropy, payout);
        jackpot.initializeLPDeposits(1_000_000_000_000_000);

        usdc.mint(lp, 10_000_000 * 1e6);
        vm.startPrank(lp);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(5_000_000 * 1e6);
        vm.stopPrank();

        jackpot.initializeJackpot(block.timestamp + 1 hours);

        // Fund contract to pay refunds
        usdc.mint(address(jackpot), 1_000_000 * 1e6);

        // Mint a referred ticket for the current drawing directly (simulate a referred buy)
        uint256 currId = jackpot.currentDrawingId(); // == 1
        uint256 ticketId = 777;
        bytes32 referralScheme = keccak256(abi.encode(address(0xA), uint256(1))); // non-zero => treated as referred
        vm.prank(address(jackpot));
        nft.mintTicket(user, ticketId, currId, 0, referralScheme);
    }

    function test_EmergencyRefund_UsesCurrentReferralFee_OverRefundsWhenFeeLowered() public {
        // Governance lowers referral fee to 0% BEFORE emergency
        jackpot.setReferralFee(0);

        // Engage emergency mode
        jackpot.enableEmergencyMode();

        uint256[] memory ids = new uint256[](1);
        ids[0] = 777;

        uint256 userBefore = usdc.balanceOf(user);
        vm.prank(user);
        jackpot.emergencyRefundTickets(ids);
        uint256 userAfter = usdc.balanceOf(user);
        uint256 got = userAfter - userBefore;

        // Expected correct refund should reflect original 20% referral fee at purchase epoch
        uint256 expectedCorrect = TICKET_PRICE * (PRECISE_UNIT - 2e17) / PRECISE_UNIT; // 80% of price

        // Actual refund (using current 0%) equals full ticket price
        assertEq(got, TICKET_PRICE, "refund uses current referralFee (0%)");
        assertGt(got, expectedCorrect, "over-refund vs snapshot fee");
    }
}


## Suggested Mitigation
Make emergency refunds use the referral fee that actually applied at purchase time. Two safe designs:
- Per-ticket snapshot (recommended): store referral fee used at purchase for each ticket. For example, add mapping(uint256 => uint256) ticketReferralFeeBps (or amount) in Jackpot, set it when minting each ticket in buyTickets (only if referralScheme != 0), and compute refund as ticketPrice * (PRECISE_UNIT - ticketReferralFeeBps[ticketId]) / PRECISE_UNIT during emergencyRefundTickets. This preserves exactness across mid-drawing fee changes without clawing back referrer balances.
- Freeze fee per drawing: extend DrawingState with referralFee and set it in _setNewDrawingState. Then: (1) use drawingState[currentDrawingId].referralFee for computing purchase-time referral fees in buyTickets, and (2) use the same snapshot in emergencyRefundTickets. Also change setReferralFee to only affect the next drawing (do not alter currentDrawingState), ensuring all purchases within a drawing share one fee snapshot.
Either approach fully eliminates over/under-refunds caused by mid-drawing referralFee changes. The per-ticket snapshot provides the most precise behavior if governance must be able to change fees mid-drawing.





 **Derived From** : Emergency refunds leave tracker counts → LP pays phantom winners after drawing resumes

## [M-7]. Jackpot.emergencyRefundTickets does not remove combos; later settlement counts refunded tickets as winners and debits LP (phantom payouts)

### Finding Severity Justification: Refunded tickets are not removed from the combo tracker. If emergencyMode is later disabled and the drawing is settled normally, phantom winners are counted and "userWinnings" debits the LP pool without any possibility for those burned tickets to claim. Impact is direct economic loss to LPs (reduced LP value/accumulator) with no USDC outflow, but requires the admin to enable emergency mode, allow refunds, and then resume the drawing, which lowers likelihood.
## Derived From Pattern/Invariant
Emergency refunds leave tracker counts → LP pays phantom winners after drawing resumes

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
When emergencyMode is used to refund current-drawing tickets, emergencyRefundTickets burns the NFTs and transfers USDC back to the owner but never removes their combinations from the on-chain combo tracker for the active drawing. If governance later disables emergencyMode and the drawing is run normally, scaledEntropyCallback uses the stale drawingEntries[...] in TicketComboTracker.countTierMatchesWithBonusball to compute uniqueResult/dupResult, so refunded (now non-existent) tickets are still counted as winners. The payout calculator then includes these phantom winners in drawingUserWinnings, and LPManager.processDrawingSettlement subtracts that amount from the LP pool, even though no NFT can ever claim it (burned). This breaks accounting invariants: contract USDC doesn’t decrease on settlement, but LP value is reduced, under-collateralizing LPs and depressing share price.

## Impact
LP pool/new accumulator is reduced by phantom userWinnings while no funds are paid out. Contract USDC stays the same but LP value decreases, creating an accounting gap and permanently harming LPs.

## Command to Run Test


## Proof of Concept
1) Initialize the system, deposit LP funds, and start a drawing. 2) A user buys a ticket with known numbers. 3) Owner enables emergencyMode; the user calls emergencyRefundTickets for that ticket (NFT burned; refund paid). 4) Owner disables emergencyMode and locks the jackpot. 5) Entropy provider calls scaledEntropyCallback with winning numbers equal to the refunded ticket. 6) Settlement uses stale drawingEntries and counts the refunded ticket as winner, computing drawingUserWinnings > 0. LPManager.processDrawingSettlement subtracts this from LP, but no USDC is transferred to winners at settlement (winners only get paid on subsequent claim, which is impossible because the NFT is burned). Result: USDC balance unchanged while LP pool is reduced — a phantom payout.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
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
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }
    function approve(address spender, uint256 amt) external returns (bool) { allowance[msg.sender][spender] = amt; return true; }
    function transfer(address to, uint256 amt) external returns (bool) { require(balanceOf[msg.sender] >= amt); balanceOf[msg.sender] -= amt; balanceOf[to] += amt; return true; }
    function transferFrom(address from, address to, uint256 amt) external returns (bool) { require(allowance[from][msg.sender] >= amt); require(balanceOf[from] >= amt); allowance[from][msg.sender] -= amt; balanceOf[from] -= amt; balanceOf[to] += amt; return true; }
}

contract MockEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64) { return 0; }
    function getFee(uint32) external pure returns (uint256) { return 0; }
}

contract PhantomWinnersRefundTest is Test {
    Jackpot jackpot;
    JackpotLPManager lp;
    JackpotTicketNFT nft;
    GuaranteedMinimumPayoutCalculator pc;
    MockUSDC usdc;
    MockEntropy entropy;

    address owner = address(this);
    address lpUser = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        // Deploy core
        jackpot = new Jackpot({
            _drawingDurationInSeconds: 1 days,
            _normalBallMax: 10,
            _bonusballMin: 1,
            _lpEdgeTarget: 0,
            _reserveRatio: 0,
            _referralFee: 0,
            _referralWinShare: 0,
            _protocolFee: 0,
            _protocolFeeThreshold: 0,
            _ticketPrice: 1_000_000,
            _maxReferrers: 5,
            _entropyBaseGasLimit: 200000
        });

        lp = new JackpotLPManager(IJackpot(address(jackpot)));
        nft = new JackpotTicketNFT(IJackpot(address(jackpot)));
        usdc = new MockUSDC();
        entropy = new MockEntropy();

        // Payout calculator with all weight on jackpot tier (11), no minimums
        bool[12] memory minTiers;
        uint256[12] memory weights;
        weights[11] = 1e18; // all premium to tier 11
        pc = new GuaranteedMinimumPayoutCalculator(IJackpot(address(jackpot)), 0, 0, minTiers, weights);

        // Wire dependencies
        jackpot.initialize(IERC20(address(usdc)), lp, nft, IScaledEntropyProvider(address(entropy)), pc);

        // Initialize LP and pool cap
        jackpot.initializeLPDeposits(1_000_000_000_000); // big cap

        // Fund LP and attacker
        usdc.mint(lpUser, 1_000_000_000); // 1,000 USDC
        vm.startPrank(lpUser);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(1_000_000_000);
        vm.stopPrank();

        // Finish bootstrap and open first drawing
        jackpot.initializeJackpot(block.timestamp + 1);

        // Fund attacker for ticket buy
        usdc.mint(attacker, 10_000_000);
        vm.startPrank(attacker);
        usdc.approve(address(jackpot), type(uint256).max);
        vm.stopPrank();
    }

    function test_phantomWinnersAfterEmergencyRefundUndercollateralizesLP() public {
        // Build a single ticket (normals 1..5, bonusball 1)
        uint8[] memory normals = new uint8[](5);
        normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
        tickets[0] = IJackpot.Ticket({ normals: normals, bonusball: 1 });

        // Buy ticket
        vm.startPrank(attacker);
        uint256[] memory ids = jackpot.buyTickets(tickets, attacker, new address[](0), new uint256[](0), bytes32(0));
        vm.stopPrank();

        // Owner enables emergency mode
        jackpot.enableEmergencyMode();

        // Attacker refunds the ticket (NFT is burned)
        vm.startPrank(attacker);
        uint256[] memory r = new uint256[](1); r[0] = ids[0];
        jackpot.emergencyRefundTickets(r);
        vm.stopPrank();

        // Disable emergency and lock the drawing
        jackpot.disableEmergencyMode();
        jackpot.lockJackpot();

        uint256 dId = jackpot.currentDrawingId();
        Jackpot.DrawingState memory ds = jackpot.getDrawingState(dId);
        uint256 preUSDC = usdc.balanceOf(address(jackpot));
        uint256 preLP = lp.getLPDrawingState(dId).lpPoolTotal;

        // Entropy provider callback with the refunded ticket as the winning combination
        uint256[][] memory rand = new uint256[][](2);
        rand[0] = new uint256[](5); rand[0][0]=1; rand[0][1]=2; rand[0][2]=3; rand[0][3]=4; rand[0][4]=5; // normals
        rand[1] = new uint256[](1); rand[1][0]=1; // bonusball

        vm.prank(address(entropy));
        jackpot.scaledEntropyCallback(bytes32(0), rand, bytes(""));

        // After settlement, a new drawing has started
        uint256 newDid = jackpot.currentDrawingId();
        assertEq(newDid, dId + 1);

        uint256 postUSDC = usdc.balanceOf(address(jackpot));
        uint256 postLP = lp.getLPDrawingState(newDid).lpPoolTotal;

        // No USDC leaves the contract on settlement; LP pool decreases by prizePool (all premium to jackpot tier)
        assertEq(postUSDC, preUSDC, "USDC should not move during settlement");
        // Expected postLP = preLP + ds.lpEarnings - ds.prizePool (protocol fee = 0)
        uint256 expectedPostLP = preLP + ds.lpEarnings - ds.prizePool;
        assertEq(postLP, expectedPostLP, "LP debited by phantom winners");

        // The ticket is burned and cannot be claimed => phantom payout
        vm.startPrank(attacker);
        vm.expectRevert();
        jackpot.claimWinnings(r);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Two complementary fixes; adopt at least one fully (A is simplest and safest). A) Abort-on-refund (recommended): 1) Add DrawingState.refundedNet (sum of net ticket refunds = ticketPrice or ticketPrice - referralFee if scheme used). In emergencyRefundTickets, increment refundedNet, decrement globalTicketsBought, and burn NFT. 2) Add DrawingState.drawingAborted (bool). Set drawingAborted = true the first time any ticket is refunded. 3) In runJackpot and scaledEntropyCallback, revert if drawingAborted is true. 4) Add an owner-only abortDrawingAndRollover() that: - Calls jackpotLPManager.processDrawingSettlement(currentDrawingId, drawingState[currentDrawingId].lpEarnings - drawingState[currentDrawingId].refundedNet, 0, 0) - Then calls _setNewDrawingState(...) to start the next drawing (this reinitializes the combo tracker) - Resets drawingAborted/refundedNet for the new drawing. This ensures LP value is reduced by exactly the USDC that left the contract during refunds; no phantom winners are ever computed. B) Resume-drawing path (if you want to keep the drawing alive): 1) Implement TicketComboTracker.remove(tracker, normals, bonusball) that mirrors insert: - For each subset size k in [1..normalTiers], compute subsets from the ticket’s bit vector. - For each subset: if comboCounts[bonusball][subset].dupCount > 0: dupCount-- else: count-- (revert if both are zero). - Also apply the same logic to bonusballTicketCounts[bonusball] (dupCount first else count). This keeps invariants stable so that future insert sees unique if and only if count > 0. 2) In emergencyRefundTickets, before burning the NFT: - Unpack the ticket (normals, bonusball), call TicketComboTracker.remove(...) for currentDrawingId. - Decrement drawingState[currentDrawingId].globalTicketsBought by 1. - Decrement drawingState[currentDrawingId].lpEarnings by the net refund amount actually paid (ticketPrice if no referral scheme, else ticketPrice - referralFee). - If and only if the ticket was a duplicate at the moment of refund (i.e., comboCounts[bonusball][fullSet].dupCount was > 0 before decrement), also decrement drawingState[currentDrawingId].prizePool by (ticketPrice - edgePerTicket) to reverse the duplicate-induced prize pool increment. 3) Do not allow disabling emergency mode to resume the drawing unless all refunds have been removed from the tracker and accounting adjusted via the steps above. Either approach fully eliminates phantom payouts: (A) forbids settlement entirely after any refund and carries correct economics forward; (B) removes refunded tickets from winner math and fixes LP accounting so normal settlement is safe.





 **Derived From** : Past drawings become unclaimable after payoutCalculator update (no per-drawing snapshot)

## [M-8]. Historic winners can be paid 0 after admin updates payoutCalculator; claimWinnings reads from wrong calculator

### Finding Severity Justification: Winners from already-settled drawings can be paid 0 and have their tickets burned if the owner updates payoutCalculator between settlement and claim. Impact is high (loss of user winnings and permanent lock of funds), but the trigger requires a privileged action (owner calling setPayoutCalculator). Per Code4rena guidance, vulnerabilities reachable via reasonable use of privileged functions are capped at Medium.
## Derived From Pattern/Invariant
Past drawings become unclaimable after payoutCalculator update (no per-drawing snapshot)

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.claimWinnings

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
Jackpot.claimWinnings reads tier payouts from the mutable global payoutCalculator instead of the calculator that computed and stored payouts for that specific drawing. During settlement, payouts for drawing N are stored via the calculator active at that moment: payoutCalculator.calculateAndStoreDrawingUserWinnings(currentDrawingId, ...) and tier info is snapshotted for future drawings in _setNewDrawingState(): payoutCalculator.setDrawingTierInfo(currentDrawingId). If the owner later calls setPayoutCalculator() before users of drawing N claim, claimWinnings() will query the new calculator, which has no tierPayouts for drawing N and returns 0. Since tickets are burned before the lookup, the winner loses the right to claim and receives 0, breaking payout accounting invariants and locking winnings.

Vulnerable snippets:
- Jackpot.claimWinnings():
  uint256 winningAmount = payoutCalculator.getTierPayout(drawingId, tierId);
- Jackpot._setNewDrawingState():
  payoutCalculator.setDrawingTierInfo(currentDrawingId);
- Admin can swap mid-life:
  function setPayoutCalculator(IPayoutCalculator _payoutCalculator) external onlyOwner { ... }


## Impact
If the owner updates payoutCalculator after a drawing is settled but before winners claim, claimWinnings will consult the new calculator which has no stored payouts for the historical drawing and returns 0. As tickets are burned before payout retrieval, winners permanently lose their winnings and cannot re-claim even if the calculator is reverted. This causes loss of user funds and breaks payout accounting for that drawing. The issue is triggered by a privileged admin action, so severity is Medium.

## Command to Run Test


## Proof of Concept
1) Deploy system with payoutCalculator A; start drawing 1.
2) A player buys a jackpot-winning ticket.
3) Lock and settle the drawing using entropy callback; calculator A snapshots tier info for next drawing and stores drawing 1 tier payouts via calculateAndStoreDrawingUserWinnings.
4) Owner calls setPayoutCalculator(B). Calculator B has no stored payouts for drawing 1.
5) Player calls claimWinnings for the ticket from drawing 1. claimWinnings burns the ticket, then reads the payout from the current calculator (B), which returns 0. Player receives 0 and the ticket is irreversibly destroyed, making the rightful winnings unclaimable.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Jackpot} from "contracts/Jackpot.sol";
import {JackpotLPManager} from "contracts/JackpotLPManager.sol";
import {JackpotTicketNFT} from "contracts/JackpotTicketNFT.sol";
import {GuaranteedMinimumPayoutCalculator} from "contracts/GuaranteedMinimumPayoutCalculator.sol";
import {IScaledEntropyProvider} from "contracts/interfaces/IScaledEntropyProvider.sol";
import {IPayoutCalculator} from "contracts/interfaces/IPayoutCalculator.sol";
import {IJackpot} from "contracts/interfaces/IJackpot.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";

contract MockUSDC is ERC20("MockUSDC", "USDC") {
    function decimals() public pure override returns (uint8) { return 6; }
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract DummyEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(uint32, SetRequest[] memory, bytes4, bytes memory) external payable returns (uint64) { return 0; }
    function getFee(uint32) external pure returns (uint256) { return 0; }
}

contract PayoutCalcSnapshotBugTest is Test {
    MockUSDC usdc;
    Jackpot jackpot;
    JackpotLPManager lpMgr;
    JackpotTicketNFT nft;
    DummyEntropy entropy;
    GuaranteedMinimumPayoutCalculator calcA;
    GuaranteedMinimumPayoutCalculator calcB;

    address LP = address(0xBEEF);
    address PLAYER = address(0xCAFE);

    function setUp() public {
        usdc = new MockUSDC();
        jackpot = new Jackpot({
            _drawingDurationInSeconds: 60,
            _normalBallMax: 10,
            _bonusballMin: 2,
            _lpEdgeTarget: 1e17,           // 10%
            _reserveRatio: 2e17,           // 20%
            _referralFee: 0,
            _referralWinShare: 0,
            _protocolFee: 0,
            _protocolFeeThreshold: 0,
            _ticketPrice: 1e6,             // 1 USDC (6 decimals)
            _maxReferrers: 3,
            _entropyBaseGasLimit: 200000
        });

        lpMgr = new JackpotLPManager(IJackpot(address(jackpot)));
        nft = new JackpotTicketNFT(IJackpot(address(jackpot)));
        entropy = new DummyEntropy();

        // Calculator A: allocate 100% premium to jackpot tier (11)
        uint256[12] memory w; w[11] = 1e18; bool[12] memory minTiers;
        calcA = new GuaranteedMinimumPayoutCalculator(IJackpot(address(jackpot)), 0, 0, minTiers, w);
        calcB = new GuaranteedMinimumPayoutCalculator(IJackpot(address(jackpot)), 0, 0, minTiers, w);

        jackpot.initialize(IERC20(address(usdc)), IJackpotLPManager(address(lpMgr)), IJackpotTicketNFT(address(nft)), IScaledEntropyProvider(address(entropy)), IPayoutCalculator(address(calcA)));

        // Seed LPs
        jackpot.initializeLPDeposits(1_000_000_000e6);
        usdc.mint(LP, 1_000_000e6);
        vm.startPrank(LP);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(1_000_000e6);
        vm.stopPrank();

        // Start drawing #1
        jackpot.initializeJackpot(block.timestamp + 1);

        // Player buys a jackpot-winning ticket 1..5 + bonus 1
        usdc.mint(PLAYER, 10e6);
        vm.startPrank(PLAYER);
        usdc.approve(address(jackpot), type(uint256).max);
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
        uint8[] memory normals = new uint8[](5);
        normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        tickets[0] = IJackpot.Ticket({normals: normals, bonusball: 1});
        uint256[] memory ids = jackpot.buyTickets(tickets, PLAYER, new address[](0), new uint256[](0), bytes32("src"));
        vm.stopPrank();

        // Lock and settle drawing #1 with exact winning numbers
        jackpot.lockJackpot();
        uint256[][] memory rng = new uint256[][](2);
        rng[0] = new uint256[](5); rng[0][0]=1; rng[0][1]=2; rng[0][2]=3; rng[0][3]=4; rng[0][4]=5;
        rng[1] = new uint256[](1); rng[1][0]=1;
        vm.prank(address(entropy));
        jackpot.scaledEntropyCallback(bytes32(0), rng, bytes(""));

        // Sanity: calcA stored a positive jackpot tier payout for drawing #1
        uint256 aJackpotPayout = calcA.getTierPayout(1, 11);
        assertGt(aJackpotPayout, 0, "calcA payout for (1,11) should be > 0");

        // Admin swaps to calcB AFTER settlement, BEFORE claims
        jackpot.setPayoutCalculator(IPayoutCalculator(address(calcB)));
        // Prove calcB has no snapshot for drawing #1
        assertEq(calcB.getTierPayout(1, 11), 0, "calcB holds no payouts for settled drawing 1");

        // Claim: should pay 0 due to reading from current (wrong) calculator; ticket is burned first
        vm.startPrank(PLAYER);
        uint256 pre = usdc.balanceOf(PLAYER);
        uint256[] memory toClaim = new uint256[](1); toClaim[0] = ids[0];
        jackpot.claimWinnings(toClaim);
        vm.stopPrank();
        uint256 post = usdc.balanceOf(PLAYER);
        assertEq(post - pre, 0, "historic claim incorrectly paid > 0");

        // Verify ticket is irreversibly burned (ownerOf reverts)
        bool burned;
        try IERC721(address(nft)).ownerOf(ids[0]) returns (address) { burned = false; } catch { burned = true; }
        assertTrue(burned, "ticket should be burned and unclaimable after zero payout");
    }
}


## Suggested Mitigation
Snapshot the payout calculator per drawing and use that snapshot for claims. Concretely: store mapping(uint256 => IPayoutCalculator) drawingCalculator; set drawingCalculator[currentDrawingId] = payoutCalculator at settlement time (immediately after calculateAndStoreDrawingUserWinnings for that drawing). In claimWinnings, read tier payouts via drawingCalculator[drawingId].getTierPayout(drawingId, tierId) instead of the mutable global payoutCalculator. Alternatively, persist the computed tier payouts in Jackpot’s own storage during settlement and have claimWinnings read from immutable Jackpot data. As an additional defense, restrict setPayoutCalculator to only affect future drawings (e.g., stage a pending calculator and activate it inside _setNewDrawingState), ensuring historical drawings always reference the original calculator.





 **Derived From** : Referral rounding dust shaved from winners on claim accumulates in contract

## [L-9]. Referrer-share rounding dust in Jackpot.claimWinnings permanently strands value in contract and underpays winners

### Finding Severity Justification: The issue causes at most a few wei (USDC 6‑decimals) of rounding dust per claim when distributing referral win-share across multiple referrers. The user’s net payout subtracts the full referrerShare, while referrers are credited the floored per-split amounts, leaving a tiny remainder unallocated. Impact is de minimis and aligns with project’s stated policy that rounding truncates in favor of solvency. No realistic loss of meaningful assets or solvency risk.
## Derived From Pattern/Invariant
Referral rounding dust shaved from winners on claim accumulates in contract

## Exploit Type
RoundingError

## Location
Jackpot.claimWinnings

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In Jackpot.claimWinnings the net user payout subtracts referrerShare = winningAmount * referralWinShare / 1e18, while _payReferrersWinnings distributes per-referrer amounts as floor(referrerShare * split[i] / 1e18). Because integer division floors each term, sum(distributed) <= referrerShare and the remainder (dust) is not credited to any accounting bucket. When a referral scheme exists, this dust is not added to lpEarnings either, so it remains stranded in the contract. Winners systematically lose the remainder and accounting drifts. Vulnerable snippets:

- claimWinnings: totalClaimAmount += winningAmount - referrerShare;
- _payReferrersWinnings (referral path): for each referrer: referrerFee = referrerShare * referralSplit[i] / 1e18; referralFees[referrer] += referrerFee; return referrerShare;

No code path credits the unallocated remainder back to LPs or to the winner.

## Impact
Each claim with a referral scheme underpays the winner by up to (maxReferrers - 1) units of the payout token’s smallest unit (USDC wei) due to per-split flooring, and the unallocated remainder is not credited to referrers or to lpEarnings. This creates a persistent positive balance drift on the Jackpot contract that is not reflected in lpEarnings or referralFees, violating the implicit accounting invariant that on-chain USDC should equal tracked obligations (LP value + referral balances + pending payouts). While the monetary effect per claim is minimal, the drift accumulates over time.

## Command to Run Test


## Proof of Concept
1) Initialize the system and start a drawing with ticket price = 1e6 (USDC 6 decimals) and referralWinShare = 10%.
2) User buys a ticket with a referral scheme of two referrers, splits [1/3, 2/3] in PRECISE_UNIT.
3) Configure payout calculator to return a fixed winningAmount = 1e6 (1 USDC) for any tier.
4) After settlement, the user claims. The contract subtracts referrerShare = floor(1e6 * 0.1) = 100,000 from the user and pays the user 900,000. It credits ref1 with floor(100,000 * 1/3) = 33,333 and ref2 with floor(100,000 * 2/3) = 66,666. sum = 99,999 < 100,000; dust = 1 remains in the contract unaccounted.
5) Assert: referralFees[ref1]+referralFees[ref2] == referrerShare - 1; the jackpot USDC balance drops only by (winningAmount - referrerShare), so 1 wei of USDC remains as stranded dust. Repeat over many claims to accumulate dust.

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
    constructor() ERC20("MockUSDC", "USDC") {}
    function decimals() public view override returns (uint8) { return 6; }
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract MockPayoutCalc is IPayoutCalculator {
    uint256 public payoutPerTier = 1_000_000; // 1 USDC (6 decimals)
    function calculateAndStoreDrawingUserWinnings(
        uint256,
        uint256,
        uint8,
        uint8,
        uint256[] memory,
        uint256[] memory
    ) external pure returns (uint256) {
        return 0; // not used in this PoC
    }
    function setDrawingTierInfo(uint256) external {}
    function getTierPayout(uint256, uint256) external view returns (uint256) {
        return payoutPerTier;
    }
}

contract MockEntropy is IScaledEntropyProvider {
    function requestAndCallbackScaledRandomness(
        uint32,
        SetRequest[] memory,
        bytes4,
        bytes memory
    ) external payable returns (uint64) {
        return 1; // no-op for test
    }
    function getFee(uint32) external pure returns (uint256) { return 0; }
    // helper to invoke Jackpot callback as the entropy provider
    function trigger(Jackpot j, uint256[][] memory nums) external {
        j.scaledEntropyCallback(bytes32(0), nums, bytes(""));
    }
}

contract ReferralDustPoCTest is Test {
    uint256 constant PRECISE_UNIT = 1e18;

    MockUSDC usdc;
    Jackpot jackpot;
    JackpotLPManager lpMgr;
    JackpotTicketNFT nft;
    MockEntropy entropy;
    MockPayoutCalc payout;

    address lp = address(0xA11CE);
    address user = address(0xBEEF);
    address ref1 = address(0x1111);
    address ref2 = address(0x2222);

    function setUp() public {
        usdc = new MockUSDC();
        jackpot = new Jackpot(
            1,          // drawingDurationInSeconds
            35,         // normalBallMax
            1,          // bonusballMin
            2e16,       // lpEdgeTarget 2%
            0,          // reserveRatio
            0,          // referralFee on buys
            1e17,       // referralWinShare 10%
            0,          // protocolFee
            0,          // protocolFeeThreshold
            1_000_000,  // ticketPrice = 1 USDC
            5,          // maxReferrers
            100_000     // entropyBaseGasLimit
        );
        lpMgr = new JackpotLPManager(jackpot);
        nft = new JackpotTicketNFT(jackpot);
        entropy = new MockEntropy();
        payout = new MockPayoutCalc();

        jackpot.initialize(IERC20(address(usdc)), lpMgr, nft, entropy, payout);
        jackpot.initializeLPDeposits(1_000_000_000_000);

        // fund LP and deposit
        usdc.mint(lp, 10_000_000_000); // 10,000 USDC
        vm.startPrank(lp);
        usdc.approve(address(jackpot), type(uint256).max);
        jackpot.lpDeposit(5_000_000_000); // deposit 5,000 USDC
        vm.stopPrank();

        // start drawing 1
        jackpot.initializeJackpot(block.timestamp - 1);

        // user buys 1 ticket with referral scheme 1/3 & 2/3
        usdc.mint(user, 1_000_000);
        vm.startPrank(user);
        usdc.approve(address(jackpot), type(uint256).max);

        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
        uint8[] memory normals = new uint8[](5);
        normals[0]=1; normals[1]=2; normals[2]=3; normals[3]=4; normals[4]=5;
        tickets[0] = IJackpot.Ticket({normals: normals, bonusball: 1});

        address[] memory refs = new address[](2);
        refs[0] = ref1; refs[1] = ref2;
        uint256[] memory splits = new uint256[](2);
        splits[0] = 333333333333333333; // 1/3
        splits[1] = 666666666666666667; // 2/3

        jackpot.buyTickets(tickets, user, refs, splits, bytes32(0));
        vm.stopPrank();

        // lock and settle drawing
        vm.prank(address(0xCAFE));
        jackpot.runJackpot{value: 0}();
        uint256[][] memory nums = new uint256[][](2);
        nums[0] = new uint256[](5); // normals
        nums[0][0]=1; nums[0][1]=2; nums[0][2]=3; nums[0][3]=4; nums[0][4]=5;
        nums[1] = new uint256[](1); // bonusball
        nums[1][0]=1;
        entropy.trigger(jackpot, nums);
    }

    function test_RoundingDustAccumulatesOnClaim() public {
        // fetch the user's single ticket id for drawing 1
        JackpotTicketNFT.ExtendedTrackedTicket[] memory list = nft.getUserTickets(user, 1);
        assertEq(list.length, 1, "user should have 1 ticket in drawing 1");
        uint256[] memory ids = new uint256[](1);
        ids[0] = list[0].ticketId;

        uint256 preJackpotBal = usdc.balanceOf(address(jackpot));
        uint256 ref1Before = jackpot.referralFees(ref1);
        uint256 ref2Before = jackpot.referralFees(ref2);

        uint256 expectedWinning = 1_000_000; // 1 USDC
        uint256 expectedRefShare = 100_000;   // 10%
        uint256 expectedUserNet = expectedWinning - expectedRefShare; // 900,000

        vm.prank(user);
        jackpot.claimWinnings(ids);

        uint256 postJackpotBal = usdc.balanceOf(address(jackpot));
        uint256 delta = preJackpotBal - postJackpotBal;
        assertEq(delta, expectedUserNet, "Jackpot should transfer only net winnings to user");

        uint256 ref1After = jackpot.referralFees(ref1);
        uint256 ref2After = jackpot.referralFees(ref2);
        uint256 credited = (ref1After - ref1Before) + (ref2After - ref2Before);
        assertEq(credited, expectedRefShare - 1, "Per-split floors under-credit referrers by 1");

        uint256 stranded = expectedRefShare - credited; // should be 1
        assertEq(stranded, 1, "Remainder dust remains unaccounted inside contract");
    }
}


## Suggested Mitigation
Eliminate untracked dust by ensuring the amount subtracted from the winner exactly equals the amount credited to referrers (or explicitly crediting the remainder elsewhere). Two safe approaches:
- Preferred (round-last to referrers): In _payReferrersWinnings, compute per-referrer amounts with floor for the first N-1 referrers, accumulate sumPaid, then assign the remainder (referrerShare - sumPaid) to the last referrer. Return referrerShare, which now equals the sum credited, so claimWinnings subtracts the exact amount distributed to referrers.
- Alternative (subtract actual distributed): In _payReferrersWinnings, sum the per-referrer floors into totalPaid and return totalPaid; in claimWinnings subtract totalPaid (not the unrounded referrerShare). This keeps the winner from being underpaid, at the cost of referrers receiving slightly less when splits round down.
If aligning with the project’s solvency policy is preferred, you may additionally route any residual (when present) to lpEarnings explicitly (and still subtract exactly what was credited) so that accounting stays consistent. In all cases, ensure no path leaves a remainder untracked in the contract balance.



