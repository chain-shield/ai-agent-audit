# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an on-chain raffle that lets anyone vie for a cute dog NFT. Players call `enterRaffle`, passing an array of addresses and sending `entranceFee × n` ETH. The contract rejects duplicate addresses and records entrants in `players`. Until `raffleDuration` elapses, any entrant may call `refund` to reclaim their stake; only the player herself can trigger her own refund.

When the timer is up, anyone can invoke `selectWinner`. The function picks a pseudo-random player index, mints an ERC-721 puppy with rarity metadata, transfers the prize pot minus fees to the winner, and routes accumulated fees to `feeAddress`. The last winner is stored in `previousWinner` for transparency.

If no players are active, the owner can call `withdrawFees` to collect residual fees and may update `feeAddress` through `changeFeeAddress`.

Extensive Foundry tests cover entering, duplicate prevention, refunds, winner payout, URI correctness, and fee withdrawal. A Forge script automates deployment with a 1 ETH entrance fee, the deployer as `feeAddress`, and a 1-day raffle duration.

In short, Puppy Raffle combines a fair ticketing mechanism, secure fund handling, and NFT rewards in under 300 lines of Solidity.
## High Risk Findings
[H-1]. Integer Overflow issue found with High severity
[H-2]. Reentrancy issue found with High severity
[H-3]. Randomness issue found with High severity
[H-4]. Unexpected Eth issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity


### Number of Findings
- C: 0
- H: 4
- M: 1
- L: 0
- I: 0



