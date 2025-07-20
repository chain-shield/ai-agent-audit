# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an on-chain game that lets anyone buy tickets to win a randomly generated dog NFT.

**How it works**
1. **Enter** – Call `enterRaffle(address[] newPlayers)` and send `entranceFee` (set at deployment) for every listed address. Duplicate addresses are rejected so each ticket is unique.
2. **Refund** – Any player may exit prior to draw via `refund`, reclaiming their ether and freeing the slot.
3. **Raffle timer** – Each round lasts `raffleDuration` seconds from `raffleStartTime`. After the period the draw becomes available.
4. **Select winner** – Anyone can trigger `selectWinner()`. A pseudo-random index chooses the winner, an ERC-721 puppy NFT is minted to them, and the contract transfers the prize pot (balance minus fees).
5. **Fees** – A configurable `feeAddress` receives the protocol cut; the owner can update it and withdraw accrued `totalFees`.
6. **NFT metadata** – Token IDs are mapped to rarity (Common, Rare, Legendary) with IPFS image URIs and on-chain, Base64-encoded JSON metadata.

Built on OpenZeppelin ERC-721 & Ownable, the protocol is simple, auditable, and self-contained; only solidity 0.7.6 and no external oracles are required.
## High Risk Findings
[H-1]. Integer Overflow issue found with High severity
[H-2]. Integer Overflow issue found with High severity
[H-3]. Reentrancy issue found with High severity
[H-4]. Randomness issue found with High severity
## Medium Risk Findings
[M-1]. Unexpected Eth issue found with Medium severity
[M-2]. DOS issue found with Medium severity
[M-3]. DOS issue found with Medium severity
[M-4]. DOS issue found with Medium severity
[M-5]. Unexpected Eth issue found with Medium severity
[M-6]. DOS issue found with Medium severity


### Number of Findings
- H: 4
- M: 6
- L: 0
- I: 0



