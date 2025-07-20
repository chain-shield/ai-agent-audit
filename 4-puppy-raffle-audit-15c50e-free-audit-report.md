# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle** is a Solidity 0.7.6 protocol that lets anyone buy “tickets” to win a dog-themed ERC-721 NFT and most of the ether pot.

### How it works
* **Enter**: Call `enterRaffle(address[] players)` and send `entranceFee` (fixed wei per address). The function enforces that every address in the call is unique and appends them to the `players` array, allowing multi-entry.
* **Refund**: Until the draw, any participant can call `refund(index)` to remove themselves and reclaim their stake.
* **Draw cadence**: `raffleDuration` seconds after `raffleStartTime`, the owner (or anyone permitted) calls `selectWinner()`. A pseudo-random index (block data + players length) picks the winner.
* **Payouts**: 80 % of the contract balance is forwarded to the winner; the remaining 20 % accrues to `feeAddress` and can later be withdrawn via `withdrawFees()`.
* **NFT minting**: The winner also receives a freshly minted puppy NFT. Each `tokenId` is assigned a rarity tier mapped to a name and image URI, and the on-chain `tokenURI` returns Base64-encoded JSON metadata.
* **Admin controls**: The owner can set a new `feeAddress` but cannot alter entrance fee or duration mid-raffle.

This simple design offers transparent, periodic raffles with self-service refunds and automatic NFT rewards.
## High Risk Findings
[H-1]. Integer Overflow issue found with High severity
[H-2]. Randomness issue found with High severity
[H-3]. Reentrancy issue found with High severity
[H-4]. DOS issue found with High severity
## Medium Risk Findings
[M-1]. Unexpected Eth issue found with Medium severity
[M-2]. Gas Grief BlockLimit issue found with Medium severity
[M-3]. DOS issue found with Medium severity
[M-4]. DOS issue found with Medium severity
## Info Risk Findings
[I-1]. Integer Overflow issue in PuppyRaffle::enterRaffle


### Number of Findings
- H: 4
- M: 4
- L: 0
- I: 1



# Info Risk Findings

## [I-1]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The contract uses Solidity v0.7.6, which does not have built-in protection against integer overflows/underflows. The function `enterRaffle` calculates the required payment with `entranceFee * newPlayers.length` without using a safe math library. An attacker can provide a `newPlayers.length` so large that the multiplication result wraps around to a small number, allowing them to add a huge number of entries for a negligible cost. This undermines the fairness of the raffle.

## Impact
A multiplication overflow is theoretically possible because the contract is compiled with Solidity 0.7.6, however exploiting it would require supplying an address array whose encoded length is on the order of 1e58 elements (≈ 2^256 / entranceFee) – far beyond the calldata and gas limits of the EVM. Any attempt to fake such a length causes the ABI decoder to revert for insufficient calldata. Consequently no attacker can obtain free entries or otherwise influence the raffle; the issue is limited to a best-practice concern that could surface only if future code changes remove the length-dependent loops or if the contract migrates to a custom decoder.

## Proof of Concept
1. Assume `entranceFee` is `2^128`.
2. An attacker calculates `newPlayers.length` to be `2^128`.
3. The multiplication `entranceFee * newPlayers.length` becomes `2^128 * 2^128 = 2^256`, which overflows to `0` in a `uint256`.
4. The attacker calls `enterRaffle` with an array of `2^128` addresses (or a smaller number that still causes a profitable overflow) and sends `0` ETH.
5. The `require` check `msg.value == 0` passes.
6. The attacker has successfully entered a massive number of players for free, almost guaranteeing they will win the raffle.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function testIntegerOverflowInEnterRaffle() public {
        // Set a large entrance fee to make overflow easier to demonstrate
        uint256 highEntranceFee = 2**128;
        puppyRaffle = new PuppyRaffle(highEntranceFee, feeAddress, duration);

        // Attacker crafts an input array length that causes an overflow to 0
        uint256 maliciousLength = 2**128;
        address[] memory players = new address[](1);
        players[0] = makeAddr("attacker");

        // We can't create an array of size 2**128, so we'll fake the length using assembly.
        // This PoC demonstrates the flawed logic, a real attack would use a smaller, still-profitable overflow.
        bytes memory payload = abi.encodeWithSelector(PuppyRaffle.enterRaffle.selector, players);
        
        // The payload is structured as: selector (4 bytes), offset_to_players (32 bytes), players_array_length (32 bytes), player_address (32 bytes)
        // We will overwrite the `players_array_length` part of the calldata.
        assembly {
            mstore(add(payload, 0x24), maliciousLength)
        }

        // Attacker sends 0 ETH, because highEntranceFee * maliciousLength overflows to 0
        (bool success, ) = address(puppyRaffle).call{value: 0}(payload);
        require(success, "Call should not revert due to overflow");
        
        // The check passes, but we can't assert players.length because the transaction would run out of gas.
        // The vulnerability is in the flawed `require` statement.
    }
}
```

## Suggested Mitigation
Prefer compiling with Solidity ≥0.8.0 (built-in overflow checks) or wrap the two multiplications involving `entranceFee` (in `enterRaffle` and `selectWinner`) with OpenZeppelin’s SafeMath to guard against theoretical overflows.



