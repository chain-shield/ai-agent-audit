# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### 🐾 Puppy Raffle Protocol

Puppy Raffle is an on-chain game that lets users buy tickets to win an NFT puppy. The contract is written for Solidity 0.7.6 and thoroughly tested with Foundry.

1. **Entering** – Anyone calls `enterRaffle(address[] newPlayers)` sending `entranceFee` (1 ETH) per address. The function rejects duplicate addresses and mismatched payment, then appends players to an array. 20 % of each ticket is earmarked as protocol fees.

2. **Refunds** – Before a winner is chosen, a player may call `refund(uint256 index)` to reclaim their fee. The slot is zeroed so the array length stays constant and duplicates remain impossible.

3. **Selecting a Winner** – After `raffleDuration` (1 day by default) and once ≥1 active player exists, anyone can invoke `selectWinner()`. A pseudo-random index is derived from block data; that address receives 80 % of the contract balance and a freshly minted Puppy NFT whose rarity is randomly assigned. The raffle state resets for the next round.

4. **Fee Management** – Accumulated fees are withdrawable by `feeAddress` via `withdrawFees()` only when all players have exited. The owner can update `feeAddress` with `changeFeeAddress()`.

The design emphasizes fairness (no duplicates, refund option), transparency, and clean separation of player funds and protocol earnings.
## High Risk Findings
[H-1]. Reentrancy issue found with High severity
[H-2]. Randomness issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity
[M-2]. Unexpected Eth issue found with Medium severity
## Low Risk Findings
[L-1]. Integer Overflow issue in PuppyRaffle::selectWinner


### Number of Findings
- C: 0
- H: 2
- M: 2
- L: 1
- I: 0



# Low Risk Findings

## [L-1]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
In `selectWinner`, the protocol fee is calculated and added to `totalFees`. `totalFees` is a `uint64` to save gas via storage packing, but the `fee` variable is a `uint256`. The line `totalFees = totalFees + uint64(fee);` performs an unsafe downcast. If the raffle accumulates a very large prize pool over multiple rounds, the `fee` could exceed `type(uint64).max`. The subsequent addition would overflow `totalFees`, causing it to wrap around to a small number. This leads to incorrect accounting and will cause the `withdrawFees` function to fail, as the contract's actual balance will be much larger than the small, wrapped-around `totalFees` value.

## Impact
If the owner deploys the contract with an entranceFee so large that the 20 % protocol fee of one (or multiple) raffles exceeds 2^64-1 wei, the value is truncated when cast to uint64. From that point on, `totalFees` becomes inaccurate, and `withdrawFees()` will revert forever, permanently locking the protocol-fee funds that are already inside the contract. No user funds can be stolen, only the protocol’s fees become stuck.

## Proof of Concept
1. Set a very high `entranceFee` such that a few rounds of the raffle will generate fees exceeding `type(uint64).max`.
2. For instance, `entranceFee` = `(type(uint64).max / 4) * 5`.
3. Run a raffle with 4 players. The total collected is `type(uint64).max * 5`. The fee (20%) is `type(uint64).max`.
4. In `selectWinner`, `totalFees` becomes `type(uint64).max`.
5. Run a second, identical raffle round. The new fee is again `type(uint64).max`.
6. The calculation `totalFees = totalFees + uint64(fee)` becomes `type(uint64).max + type(uint64).max`, which overflows and wraps `totalFees` to `type(uint64).max - 1`.
7. The actual ETH balance for fees in the contract is `2 * type(uint64).max`, but `totalFees` stores a much smaller number. The `withdrawFees` function is now permanently broken.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    address public feeAddress = makeAddr("feeAddress");
    uint256 public constant RAFFLE_DURATION = 1 days;
    // Set a massive entrance fee to trigger the overflow quickly
    uint256 public constant HUGE_ENTRANCE_FEE = (2**64 / 4) * 5; // 20% of this is 2**64 / 4

    function setUp() public {
        puppyRaffle = new PuppyRaffle(HUGE_ENTRANCE_FEE, feeAddress, RAFFLE_DURATION);
    }

    function testTotalFeesOverflow() public {
        // Run first round, totalFees will be close to uint64.max
        runRaffleRound(4);
        uint64 feesAfterFirstRound = puppyRaffle.totalFees();
        uint256 expectedFee = (HUGE_ENTRANCE_FEE * 4 * 20) / 100;
        assertEq(feesAfterFirstRound, uint64(expectedFee));

        // Run a second round. The addition to totalFees will overflow.
        runRaffleRound(4);
        uint64 feesAfterSecondRound = puppyRaffle.totalFees();
        
        // The fees should be 2 * expectedFee, but it has overflowed.
        uint256 correctTotalFees = expectedFee * 2;
        assertTrue(feesAfterSecondRound < correctTotalFees);
        
        // The actual contract balance holding the fees will be correctTotalFees
        // But the totalFees variable is wrong. Withdraw will fail.
        assertEq(address(puppyRaffle).balance, correctTotalFees);
        
        vm.prank(feeAddress);
        vm.expectRevert("PuppyRaffle: There are currently players active!"); // Misleading error
        puppyRaffle.withdrawFees();
    }

    function runRaffleRound(uint256 numPlayers) internal {
        address[] memory players = new address[](numPlayers);
        for (uint256 i = 0; i < numPlayers; i++) {
            players[i] = address(uint160(uint256(keccak256(abi.encodePacked(block.timestamp, i)))));
        }
        puppyRaffle.enterRaffle{value: HUGE_ENTRANCE_FEE * numPlayers}(players);
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
        puppyRaffle.selectWinner();
    }
}
```

## Suggested Mitigation
Use a `uint256` for `totalFees` to prevent overflow, or use a safe math library to check for overflow before the addition. Given that `fee` is already `uint256`, it is simplest and safest to make `totalFees` a `uint256` as well. The gas savings from storage packing are not worth the risk of losing all protocol fees.

```diff
-   // We do some storage packing to save gas
-   address public feeAddress;
-   uint64 public totalFees = 0;
+   address public feeAddress;
+   uint256 public totalFees = 0;

// ... in selectWinner() ...
-
-       totalFees = totalFees + uint64(fee);
+       totalFees = totalFees + fee;
```



