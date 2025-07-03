
## List of Files in Src Folder
src/PuppyRaffle.sol

## README.md summary
The Puppy Raffle project enables participants to enter a raffle for a dog NFT. Participants use the `enterRaffle` function to join with a list of addresses, disallowing duplicates. They can also request refunds via the `refund` function. Periodically, a winner is drawn, and a random puppy is minted to them. The protocol owner sets a fee address to take a portion of the funds, while the rest goes to the winner. Key implementation steps include cloning the GitHub repository and utilizing Foundry for testing and development. The testing involves running `forge test` and `forge coverage` to ensure code integrity and comprehensive coverage. The deployment is compatible with Solc Version 0.7.6 on the Ethereum chain. The owner has the authority to update the fee address, while players can participate in the raffle and manage refunds. There are no known issues with the protocol.


## src/PuppyRaffle.sol summary
The `PuppyRaffle` contract allows users to enter a raffle to win a dog NFT. It extends OpenZeppelin's `ERC721` and `Ownable`, and imports `Address` and `Base64` utilities. Users enter the raffle by paying a fee, with restrictions on duplicate entries and refund options. The winner is chosen based on random selection, and receives 80% of the total entrance fees, while 20% goes to the owner's designated fee address. The winner also receives a randomly generated dog NFT. Contract owner functions include fee address management and fee withdrawal. 

**enterRaffle function** (`enterRaffle(address[] memory newPlayers) public payable`): Allows players to enter the raffle by paying the necessary entrance fees. It ensures no duplicate entries and emits a `RaffleEnter` event when players enter.

**refund function** (`refund(uint256 playerIndex) public`): Enables players to withdraw from the raffle, receiving a refund, provided they have not already been refunded. The player's entry is marked as blank. 

**selectWinner function** (`selectWinner() external`): Selects a random winner when the raffle duration has ended and there are at least 4 players. The winner receives an NFT and a portion of the entrance fees, with the rest allocated to a fee address. 

**withdrawFees function** (`withdrawFees() external`): Allows the contract owner to withdraw collected fees, provided all player entries are resolved. 

**changeFeeAddress function** (`changeFeeAddress(address newFeeAddress) external onlyOwner`): Allows the owner to set a new fee address and emits an event upon any change.

**tokenURI function** (`tokenURI(uint256 tokenId) public view virtual override returns (string memory)`): Returns the metadata URI for a given tokenId by using the traits and images associated with the token's rarity. 

Storage Variables:
- `entranceFee` (uint256): The immutable cost to enter the raffle.
- `players` (address[]): List of players currently in the raffle.
- `raffleDuration` (uint256): Time period over which the raffle takes place.
- `raffleStartTime` (uint256): Timestamp marking when the raffle began.
- `previousWinner` (address): The most recent raffle winner.
- `feeAddress` (address): Designated address to receive a portion of the collected fees.
- `totalFees` (uint64): Tracks the total fees accrued from the raffle.
- `tokenIdToRarity` (mapping(uint256 => uint256)): Maps token IDs to their rarity levels.
- `rarityToUri` (mapping(uint256 => string)): Associates rarity levels with corresponding image URIs.
- `rarityToName` (mapping(uint256 => string)): Associates rarity levels with corresponding descriptive names.

