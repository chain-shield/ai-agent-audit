# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle Protocol**

Puppy Raffle is an on-chain raffle that lets anyone compete for a unique ERC-721 “puppy” NFT by paying a fixed entrance fee (1 ETH in the reference deployment).

Workflow
1. Entry: Players call `enterRaffle(address[] newPlayers)` and supply the exact fee per address. The contract rejects under-payment, duplicate addresses, or re-entry of existing players. Each address is stored in `players`.
2. Voluntary refund: Before the draw, a participant may reclaim their stake via `refund(playerIndex)`, freeing their slot.
3. Draw window: After `raffleDuration` (default 1 day) the owner or anyone can trigger `selectWinner()`. Preconditions: at least 4 active players and the raffle period has ended.
4. Winner selection & payout: A pseudo-random index chooses the winner, the contract mints them a new puppy NFT, and transfers the prize pool minus protocol fees. Fees are forwarded to `feeAddress` which the owner can update with `changeFeeAddress()`.
5. Reset: Player array is cleared for the next round.
6. Fee withdrawal: If no players are registered, the owner can pull any accumulated fees via `withdrawFees()`.

The accompanying Foundry tests and deployment script verify correct entry logic, refund behavior, winner selection, and administrative controls.
## High Risk Findings
[H-1]. Frontrun/Backrun/Sandwhich MEV issue found with High severity
[H-2]. Reentrancy issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity
[M-2]. Randomness issue found with Medium severity
[M-3]. Unexpected Eth issue found with Medium severity
[M-4]. Integer Overflow issue found with Medium severity


### Number of Findings
- C: 0
- H: 2
- M: 4
- L: 0
- I: 0



