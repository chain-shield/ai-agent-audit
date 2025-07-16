
## SLITHER GENERATED METADATA 


## List of Files in Src Folder
PuppyRaffle.sol
## 4-puppy-raffle-audit/src/PuppyRaffle.sol SUMMARY OF MAIN FILES
The `PuppyRaffle` contract is a decentralized application built on Ethereum allowing users to enter a raffle to win a dog-themed NFT. Built using Solidity 0.7.6, it imports various OpenZeppelin libraries for ERC721 functionality, ownership control, and utility functions. Participants enter by calling the `enterRaffle` function with a list of addresses, paying a set entrance fee per participant. The raffle ensures no duplicate entries and allows users to refund their ticket before the raffle ends. 

Key variables:
- `entranceFee`: Fee for each participant.
- `players`: List of current participants.
- `raffleDuration`, `raffleStartTime`: Timing management for the raffle.
- `feeAddress`, `totalFees`: Address for collecting and tracking fees.
- `tokenIdToRarity`, `rarityToUri`, `rarityToName`: Mapping to manage puppy rarity.

Functions include:
1. `enterRaffle`: Adds players to the raffle ensuring payment is correct and duplicates are managed.
2. `refund`: Allows players to refund their entrance fee.
3. `getActivePlayerIndex`: Retrieves a player's index if active.
4. `selectWinner`: Determines and rewards a winner post-raffle end, mints NFT.
5. `withdrawFees`: Allows withdrawal of collected fees by owner.
6. `changeFeeAddress`: Updates the fee collection address.
7. `_baseURI`, `tokenURI`: Manages NFT metadata URI generation.

Overall, the contract encapsulates raffle logic, entry, refund management, winner selection with pseudo-randomness, and NFT minting with rarity attributes.


 ## DOCUMENTATION: 

