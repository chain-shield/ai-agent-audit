
## List of Files in Src Folder
4-puppy-raffle-audit/src/PuppyRaffle.sol
## 4-puppy-raffle-audit/README.md summary
The Puppy Raffle project is designed to facilitate a raffle for winning a dog NFT. Key functionalities include entering the raffle using the `enterRaffle` function, ensuring no duplicate addresses participate, providing users the option to refund their tickets via the `refund` function, and periodically drawing a winner to receive a random puppy NFT. The protocol owner can set a feeAddress to deduct a portion of the proceeds, with the remainder going to the winner.

### Getting Started
- **Requirements**:
  - **Git**: Successful installation is confirmed by running `git --version`.
  - **Foundry**: Verify installation by running `forge --version`.

- **Quickstart**:
  Clone the repository using Git and navigate to the project directory. Optionally, use Gitpod for a cloud-based setup.

### Usage
- **Testing**: Execute tests using `forge test`. Obtain test coverage using `forge coverage` with additional debugging using `forge coverage --report debug`.

### Audit Scope Details
- The focus is the `PuppyRaffle.sol` contract.
- Compatible with Solidity version 0.7.6 and intended for deployment on the Ethereum blockchain.

### Roles
- **Owner**: Can modify the fee destination.
- **Player**: Enter the raffle and request refunds.

### Known Issues
No known issues are reported.


## 4-puppy-raffle-audit/src/PuppyRaffle.sol summary
### Contract Summary

**PuppyRaffle** is a Solidity contract that implements an NFT raffle system for winning dog-themed NFTs. Participants enter the raffle by calling `enterRaffle` and paying an entrance fee. The contract handles duplicate entry prevention, offers refunds, selects winners at intervals, and distributes the prize pool and fees accordingly.

### Storage Variables

- **`entranceFee`**: `uint256` - The fee required for entering the raffle. 
- **`players`**: `address[]` - Array storing addresses of current raffle participants. 
- **`raffleDuration`**: `uint256` - Duration for each raffle round in seconds. 
- **`raffleStartTime`**: `uint256` - Timestamp marking the start of the current raffle. 
- **`previousWinner`**: `address` - Address of the previous raffle winner. 
- **`feeAddress`**: `address` - Address used for fee collection from each raffle. 
- **`totalFees`**: `uint64` - Accumulated fees collected from the raffle. 
- **`tokenIdToRarity`**: `mapping(uint256 => uint256)` - Maps token IDs to their rarity. 
- **`rarityToUri`**, **`rarityToName`**: `mapping(uint256 => string)` - Maps rarity to respective URI and name. 

### Function Summaries

- **constructor**: Initializes the contract with settings for entrance fee, fee address, and raffle duration.
- **enterRaffle**: `function enterRaffle(address[] memory newPlayers) public payable` - Allows players to enter the raffle, ensuring no duplicates.
- **refund**: `function refund(uint256 playerIndex) public` - Refunds the entrance fee to a player who opts out.
- **getActivePlayerIndex**: `external view returns (uint256)` - Retrieves the index of an active player.
- **selectWinner**: `external` - Selects a winner and mints an NFT, redistributing the prize pool.
- **withdrawFees**: `external` - Allows fee withdrawal by administrator.
- **changeFeeAddress**: `external` - Updates the fee address.
- **_isActivePlayer**: `internal view returns (bool)` - Checks if a player is active.
- **_baseURI**: `internal pure returns (string memory)` - Provides base URI for metadata.
- **tokenURI**: `public view virtual override returns (string memory)` - Retrieves the metadata URI of a given token ID.

