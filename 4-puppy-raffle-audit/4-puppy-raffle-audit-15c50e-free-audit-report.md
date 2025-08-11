# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle Protocol**  

PuppyRaffle is a Solidity-based ERC-721 raffle that lets users compete for a randomly generated “puppy” NFT.  

• Entering: Anyone calls `enterRaffle(address[] participants)` and pays `entranceFee` (1 ETH) per address supplied. Duplicate addresses in the same or previous rounds are rejected, letting a single wallet buy multiple legitimate tickets.  

• Refunds: A ticket holder may call `refund(index)` before the draw to reclaim their stake; the address is zeroed in the players array, preserving array order while freeing the slot.  

• Draw: After `raffleDuration` (default 1 day) and with ≥ 4 active players, `selectWinner()` can be triggered. 90 % of pooled ETH is sent to the winner, 10 % accrues to `totalFees` for the `feeAddress`. A new ERC-721 token is minted to the winner with rarity determined by pseudo-randomness; `tokenURI` serves on-chain JSON containing name, description and image link.  

• Fees: When no players are active, owner calls `withdrawFees()` to move accumulated fees to the designated address; `changeFeeAddress()` lets the owner update that wallet.  

Comprehensive Forge tests and a deployment script are included.
## High Risk Findings
[H-1]. Frontrun/Backrun/Sandwhich MEV issue found with High severity
[H-2]. DOS issue found with High severity
[H-3]. Reentrancy issue found with High severity
[H-4]. Integer Overflow issue found with High severity
[H-5]. Unexpected Eth issue found with High severity
## Medium Risk Findings
[M-1]. Randomness issue found with Medium severity


### Number of Findings
- C: 0
- H: 5
- M: 1
- L: 0
- I: 0



