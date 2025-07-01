
## List of Files in Src Folder
src/PuppyRaffle.sol

## README.md summary
The Puppy Raffle project is a system designed to allow participants to enter a raffle with the possibility of winning a dog NFT. To participate, users must call the `enterRaffle` function with an array of their addresses, ensuring no duplicates. The system includes a `refund` function, permitting users to claim their ticket value back. The process draws a winner periodically, through which a random puppy NFT is minted. The contract's owner sets a fee address, where a portion of the participation value is sent, and the remaining funds go to the winner. 

### Getting Started

The initial setup requires installing git and foundry. The project can be run locally or in Gitpod by cloning from the provided repository link.

### Usage

Testing is done using `forge test`, with test coverage available using `forge coverage` and detailed reports via `forge coverage --report debug`.

### Audit Scope

The audit covers the `PuppyRaffle.sol` file within the `src` directory, relevant for Solidity compiler version 0.7.6 and targeted for deployment on Ethereum.

### Roles and Known Issues

**Roles:**
- **Owner:** Manages fee address changes
- **Player:** Enters raffle and processes refunds

There are currently no known issues with the project.


## src/PuppyRaffle.sol summary
### Contract Overview
The `PuppyRaffle` contract is designed for running a raffle where participants can win a cute dog NFT. Users enter by paying an entry fee, and a winner is drawn at intervals.

### Contract Definition
- **Name**: PuppyRaffle
- **Inherits**: ERC721, Ownable
- **Purpose**: Manage a raffle for winning a puppy NFT with various levels of rarity.

### Storage Variables
- **entranceFee (uint256)**: The fee required from participants to enter the raffle.
- **players (address[])**: List of players who have entered the raffle.
- **raffleDuration (uint256)**: Duration for each raffle round.
- **raffleStartTime (uint256)**: Timestamp for when the current raffle round started.
- **previousWinner (address)**: Address of the last raffle winner.
- **feeAddress (address)**: Destination address for collected fees.
- **totalFees (uint64)**: Total accumulated fees pending withdrawal.
- **tokenIdToRarity (mapping)**: Maps token ID to its rarity.
- **rarityToUri (mapping)**: Maps rarity levels to their corresponding URI.
- **rarityToName (mapping)**: Maps rarity levels to their names.

### Functions

#### constructor
- **Arguments**: `_entranceFee (uint256), _feeAddress (address), _raffleDuration (uint256)`
- **Purpose**: Initializes the raffle with specified entrance fee, fee address, and raffle duration.

#### enterRaffle
- **Interface**: `function enterRaffle(address[] memory newPlayers) public payable`
- **Functionality**: Allows participants to enter the raffle by sending the correct fee, ensuring no duplicates.

#### refund
- **Interface**: `function refund(uint256 playerIndex) public`
- **Functionality**: Allows a participant to get a refund by specifying their index if they're active.

#### getActivePlayerIndex
- **Interface**: `function getActivePlayerIndex(address player) external view returns (uint256)`
- **Functionality**: Retrieves the index of an active player in the list.

#### selectWinner
- **Interface**: `function selectWinner() external`
- **Functionality**: Selects and rewards a raffle winner if conditions are met (4 players, duration elapsed).

#### withdrawFees
- **Interface**: `function withdrawFees() external`
- **Functionality**: Withdraws accumulated fees to the designated fee address.

#### changeFeeAddress
- **Interface**: `function changeFeeAddress(address newFeeAddress) external onlyOwner`
- **Functionality**: Allows the contract owner to update the fee collection address.

#### _isActivePlayer
- **Interface**: `function _isActivePlayer() internal view returns (bool)`
- **Functionality**: Checks if the caller is an active player in the current raffle.

#### _baseURI
- **Interface**: `function _baseURI() internal pure returns (string memory)`
- **Functionality**: Provides the base URI for token metadata.

#### tokenURI
- **Interface**: `function tokenURI(uint256 tokenId) public view virtual override returns (string memory)`
- **Functionality**: Returns the full URI for a given token ID, including encoded rarity.

