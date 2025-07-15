# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol

PuppyRaffle is an on-chain raffle system that lets anyone compete for collectible puppy NFTs while the contract autonomously handles entries, randomness, payouts and fee distribution.

1. Entering the Raffle  
Participants call `enterRaffle()` supplying a list of new player addresses and paying `entranceFee` per address. The contract verifies payment, forbids duplicates, records players and accrues a small fee for the protocol.

2. Refunds  
Before a winner is chosen, any player may reclaim their stake through `refund()`, which deletes them from the players array, returns their entrance fee and adjusts the pot.

3. Raffle Cycle & Winner Selection  
The raffle runs for `raffleDuration` seconds after the first entry. Anyone can trigger `selectWinner()` once the timer elapses. A pseudo-random number derived from block data picks an index in `players`; the winner receives the accumulated pot (minus fees) and a freshly minted ERC-721 puppy NFT. `tokenIdToRarity` assigns common, rare or legendary status based on randomness, and `tokenURI()` supplies metadata.

4. Fees & Administration  
Collected fees accumulate in `totalFees` and are withdrawable by anyone to `feeAddress` via `withdrawFees()` when no players are active. The owner can update `feeAddress` with `changeFeeAddress()`.

Overall, PuppyRaffle delivers a simple, self-contained raffle/NFT minting experience secured by Solidity and the Ethereum network.
## High Risk Findings
[H-1]. Randomness issue found with High severity
[H-2]. Reentrancy issue found with High severity
## Medium Risk Findings
[M-1]. Integer Overflow issue found with Medium severity
[M-2]. Unexpected Eth issue found with Medium severity
[M-3]. DOS issue found with Medium severity
[M-4]. Gas Grief BlockLimit issue found with Medium severity
[M-5]. Timestamp Dependent Logic issue found with Medium severity
## Info Risk Findings
[I-1]. Event Consistency issue in PuppyRaffle::selectWinner
[I-2]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 2
- M: 5
- L: 0
- I: 2



# Info Risk Findings

## [I-1]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
Several critical state changes in the contract do not emit events. Specifically, the `selectWinner` function changes `previousWinner`, resets `raffleStartTime`, calculates `totalFees`, and clears the `players` array without emitting a dedicated `WinnerSelected` event. Similarly, `withdrawFees` resets `totalFees` to zero without an event. This lack of event logging makes it difficult for off-chain services, monitoring tools, and users to track the contract's lifecycle and verify its operations.

## Impact
Reduced observability of the contract's operations. Front-ends and other dependent services cannot easily react to important events like a winner being chosen or fees being withdrawn. It also complicates auditing and incident analysis.

## Proof of Concept
1. A raffle round concludes and `selectWinner()` is called.
2. A winner is selected, the prize is sent, and an NFT is minted. The `Transfer` event for the NFT is emitted by the ERC721 contract, but no single event captures all the details of the raffle outcome (winner, prize amount, new raffle start time).
3. A user interface that wants to display "Congratulations to address X, who won Y ETH!" has no event to listen to for this information and must instead rely on polling contract state, which is inefficient.

## Proof of Code
```solidity
// This is a conceptual test. Foundry's `vm.expectEmit` would be used to show an event is *not* emitted,
// but the assertion is on the absence of an event definition and emit statement in the source code.
// The vulnerability is the missing code itself.

// contract source code lacks this:
// event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 tokenId);

// And this:
// function selectWinner() { ... emit WinnerSelected(winner, prizePool, tokenId); ... }
```

## Suggested Mitigation
Define and emit events for all significant state transitions. Add a `WinnerSelected` event in `selectWinner` and a `FeesWithdrawn` event in `withdrawFees`.

```solidity
// In PuppyRaffle.sol

// ... Event definitions
event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);
event FeesWithdrawn(address indexed feeAddress, uint256 amount);

// ...

function selectWinner() external {
    // ... logic to determine winner, prizePool, tokenId

    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");

    _safeMint(winner, tokenId);

    emit WinnerSelected(winner, prizePool, tokenId);
}

function withdrawFees() external {
    // ...
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;

    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");

    emit FeesWithdrawn(feeAddress, feesToWithdraw);
}
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma `pragma solidity ^0.8.18;`. This allows the contract to be compiled with any compiler version from 0.8.18 up to (but not including) 0.9.0. While this provides some flexibility, it is a best practice to lock the pragma to a specific, audited version (e.g., `pragma solidity 0.8.18;`). Using a floating pragma can lead to unexpected behavior or bugs if the contract is deployed using a newer compiler version that has introduced subtle changes or bugs.

## Impact
The contract might be deployed with a compiler version that has known or unknown bugs, potentially introducing security vulnerabilities that were not present during the audit. It also leads to non-deterministic builds, where the same source code could produce different bytecode.

## Proof of Concept
1. A developer audits the contract with compiler version 0.8.18.
2. Later, the deployment script uses compiler version 0.8.22, which is allowed by `^0.8.18`.
3. Unknown to the developer, version 0.8.22 has a new optimization bug that affects the logic of the contract.
4. The contract is deployed with the vulnerability, even though the source code itself was deemed safe with the original compiler.

## Proof of Code
```solidity
// The vulnerability is in the pragma line itself.
// No test case can 'exploit' this, it's a matter of development and deployment best practice.

// Vulnerable Code:
// pragma solidity ^0.8.18;

// Mitigated Code:
// pragma solidity 0.8.18;
```

## Suggested Mitigation
Lock the pragma to the specific compiler version that was used for testing and auditing the contract. This ensures that the deployed bytecode corresponds exactly to the audited version and prevents accidental introduction of bugs from future compiler updates.

```solidity
// Change this:
pragma solidity ^0.8.18;

// To this:
pragma solidity 0.8.18;
```



