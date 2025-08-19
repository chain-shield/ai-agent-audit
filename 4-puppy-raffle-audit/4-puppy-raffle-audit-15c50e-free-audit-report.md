# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Overview  
Puppy Raffle is an on-chain ERC-721 raffle that lets anyone buy tickets to win a randomly generated dog NFT.

1. Deployment sets an immutable ticket price (`entranceFee`), a fee recipient, and how long each round lasts (`raffleDuration`).  
2. `enterRaffle(address[] newPlayers)` is payable; `msg.value` must equal `entranceFee * newPlayers.length`. It records each unique address, rejecting duplicates or under-payment. Group entries or multiple tickets are allowed by passing multiple addresses.  
3. Any player can exit before the draw by calling `refund(index)`, voiding their ticket and returning their funds.  
4. When `raffleDuration` has elapsed and at least four active players exist, anyone may call `selectWinner()`. A pseudo-random index picks the winner, PuppyRaffle mints an NFT to them with rarity metadata, and sends the pot minus a protocol fee to the winner. The fee is stored in `totalFees`.  
5. After all players have either won or refunded, the owner can `withdrawFees()` to move the accumulated fees to `feeAddress`, and can update that address via `changeFeeAddress()`.  

Comprehensive Foundry tests cover duplicate entries, refunds, prize payment, URI correctness, and fee withdrawal.
## High Risk Findings
[H-1]. Accounting Invariant Violation issue found with High severity
[H-2]. Reentrancy issue found with High severity
## Medium Risk Findings
[M-1]. DOS issue found with Medium severity


### Number of Findings
- C: 0
- H: 2
- M: 1
- L: 0
- I: 0



