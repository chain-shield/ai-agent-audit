# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol
Puppy Raffle is an on-chain game where users buy raffle tickets to win a randomly generated “puppy” NFT and a share of the ether pot.

1. **Enter** – Anyone calls `enterRaffle(address[] newPlayers)` supplying an array of participant addresses and `msg.value == entranceFee * newPlayers.length`. Duplicate addresses are rejected and the list is stored in `players`.
2. **Refund** – A participant can leave before the draw via `refund(index)`, receiving their ticket price back. Only the original player can claim their refund; their slot in `players` is then deleted.
3. **Draw** – After the configurable `duration` has elapsed and at least four players remain, anyone may call `selectWinner()`. A pseudo-random index is picked, the winner receives 80 % of the contract balance, and a Puppy NFT (ERC-721) with rarity-based metadata is minted to them. The remaining 20 % is earmarked as protocol fees.
4. **Fee withdrawal** – Once no active players remain, the owner can send accumulated fees to `feeAddress` with `withdrawFees()`. The owner alone may change `feeAddress`, but cannot touch player funds.

All logic is covered by extensive Foundry tests and a deployment script, ensuring secure, reproducible launches.
## High Risk Findings
[H-1]. Randomness issue found with High severity
[H-2]. Reentrancy issue found with High severity
[H-3]. Integer Overflow issue found with High severity
[H-4]. DOS issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity
[M-2]. Unexpected Eth issue found with Medium severity


### Number of Findings
- C: 0
- H: 4
- M: 2
- L: 0
- I: 0



