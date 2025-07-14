# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### 🐶 Puppy Raffle Protocol

Puppy Raffle is an on-chain game where users buy **raffle tickets** to win a randomly generated *Puppy* ERC-721 NFT and the pot’s ether.

1. **Entering**  
   • `enterRaffle(address[] participants)` is called with the entrance fee (`msg.value = fee × addresses`).  
   • The contract checks that each address is unique and stores them in `players`.  

2. **Refunds**  
   Any player may exit before a draw with `refund(index)`, receiving their ticket price back and being removed from `players`.

3. **Raffle Cycle**  
   Each round lasts `raffleDuration` seconds from `raffleStartTime`.  
   When the period elapses, anyone can call `selectWinner()` which:  
   • Pseudo-randomly selects a player.  
   • Mints a Puppy NFT with on-chain rarity/URI data.  
   • Splits the ether pot: `feeAddress` receives a protocol fee, the winner gets the remainder.

4. **Administration**  
   The owner (deployer) can update the fee recipient via `changeFeeAddress` and withdraw accumulated fees with `withdrawFees()`.

Security is aided by duplicate-entry checks, OpenZeppelin ERC-721 inheritance, and comprehensive Foundry tests. The contract targets Solidity 0.7.6 and is intended for Ethereum mainnet deployment.
## High Risk Findings
[H-1]. Timestamp Dependent Logic issue found with High severity
[H-2]. Reentrancy issue found with High severity
[H-3]. Randomness issue found with High severity
[H-4]. Integer Overflow/Math issue found with High severity
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue found with Medium severity
[M-2]. Integer Overflow issue found with Medium severity
[M-3]. Integer Overflow/Math issue found with Medium severity
[M-4]. DOS issue found with Medium severity
[M-5]. Unexpected Eth issue found with Medium severity
## Info Risk Findings
[I-1]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
[I-2]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 4
- M: 5
- L: 0
- I: 2



# Info Risk Findings

## [I-1]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
Several critical state changes occur without emitting corresponding events, which hinders off-chain monitoring and transparency. Specifically:
1. In `selectWinner`, when a winner is chosen and paid, no event is emitted.
2. In `withdrawFees`, when the owner withdraws accumulated fees, no event is emitted.

## Impact
The lack of events makes it difficult for users, developers, and monitoring tools to track important activities on the contract. This reduces transparency and makes it harder to build reliable off-chain applications that interact with the raffle.

## Proof of Concept
1. Call `selectWinner`. Observe that no event like `WinnerSelected` is present in the transaction receipt.
2. Call `withdrawFees`. Observe that no event like `FeesWithdrawn` is present in the transaction receipt.

## Proof of Code
```solidity
// This is an observability issue. A test would involve checking the transaction logs
// (via `vm.getRecordedLogs()`) and asserting that expected logs are missing.
// The proof is the absence of `emit` statements in the source code.

// In PuppyRaffle.sol -> selectWinner():
// ...
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
_safeMint(winner, tokenId);
// No event emitted here.

// In PuppyRaffle.sol -> withdrawFees():
// ...
(bool success, ) = feeAddress.call{value: feesToWithdraw}("");
require(success, "PuppyRaffle: Failed to withdraw fees");
// No event emitted here.
```

## Suggested Mitigation
Define and emit events for all significant state changes to improve observability.

```solidity
// Add event declarations to the contract
event WinnerSelected(address indexed winner, uint256 indexed tokenId, uint256 prizeAmount);
event FeesWithdrawn(address indexed feeAddress, uint256 amount);

// In selectWinner()
// ...
_safeMint(winner, tokenId);
emit WinnerSelected(winner, tokenId, prizePool);

// In withdrawFees()
// ...
require(success, "PuppyRaffle: Failed to withdraw fees");
emit FeesWithdrawn(feeAddress, feesToWithdraw);
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract is deployed using `pragma solidity 0.7.6;`. While locking pragmas is good practice, this is an older version of the compiler that predates the introduction of default overflow/underflow checks in Solidity 0.8.0. Using older versions exposes the contract to a class of bugs that are now mitigated by default in the compiler, and may also contain other known but unfixed bugs.

## Impact
The contract is more susceptible to integer arithmetic bugs, as demonstrated by the multiple overflow vulnerabilities found. It also misses out on other security improvements and optimizations available in newer compiler versions.

## Proof of Concept
The vulnerability is the use of an outdated pragma version, as seen on the first line of the contract source code: `pragma solidity 0.7.6;`.

## Proof of Code
```solidity
// The proof is the pragma statement in PuppyRaffle.sol
// pragma solidity 0.7.6;
```

## Suggested Mitigation
It is highly recommended to upgrade the contract to a more recent and stable Solidity version, such as `0.8.20` or later. This provides automatic protection against arithmetic overflows and underflows and includes other security enhancements. After upgrading, `SafeMath` is no longer necessary.

```solidity
// Change this:
// pragma solidity 0.7.6;

// To this:
pragma solidity ^0.8.20;
```



