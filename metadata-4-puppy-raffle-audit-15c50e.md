
## List of Files in Src Folder
src/PuppyRaffle.sol

## README.md summary
The "Puppy Raffle" project is a protocol designed for users to participate in a raffle to win a dog NFT. Key functionalities include entering the raffle via the `enterRaffle` function, which requires a unique list of participant addresses. Duplicate entries are not allowed. Participants can also request a refund of their ticket value by invoking the `refund` function. Periodically, the raffle draws a winner, granting them a random puppy NFT. The protocol owner can designate a feeAddress to receive a portion of the entry costs, with the balance awarded to the raffle winner.

### Getting Started
- **Requirements**: Users need Git and Foundry installed to interact with the repository and run tests.
- **Quickstart**: Clone the repository, then navigate and build using `make`. Gitpod is supported for those who prefer not to install locally.

### Usage
- Testing can be initiated with `forge test`, while test coverage is accessible via `forge coverage` and related commands.

### Audit Scope
- The audit covers the `PuppyRaffle.sol` file, compatible with Solidity version 0.7.6, intended for deployment on the Ethereum network.

### Roles
- **Owner**: Can change the fee wallet address; deploys the protocol.
- **Player**: Enters the raffle and can request refunds.

No known issues with the project have been identified.


## src/PuppyRaffle.sol summary
The `PuppyRaffle` contract is a Solidity-based Ethereum smart contract that manages a raffle for winning a random NFT of a puppy. Entrants pay an entrance fee, and each raffle period, a winner is randomly selected to receive a puppy NFT and the remainder of the raffle pot while the owner collects a fee.

**Contract Definition:**
### PuppyRaffle
Inherits from ERC721 and Ownable, providing NFT functionalities and access control.

### Storage Variables
- **entranceFee (uint256):** Immutable; fee to enter the raffle.
- **players (address[]):** Array of addresses entered in the raffle.
- **raffleDuration (uint256):** The duration of the raffle in seconds.
- **raffleStartTime (uint256):** Timestamp of when the raffle started.
- **previousWinner (address):** Address of the last raffle winner.
- **feeAddress (address):** Address for collecting the fee from raffle pot.
- **totalFees (uint64):** Total fees collected.
- **tokenIdToRarity (mapping(uint256 => uint256)):** Mapping of tokenId to rarity.
- **rarityToUri (mapping(uint256 => string)):** Mapping of rarity to image URI.
- **rarityToName (mapping(uint256 => string)):** Mapping rarity levels to their names.
- **commonImageUri, rareImageUri, legendaryImageUri (string):** URIs for different puppy types.

**Functions:**

### constructor
Initializes the contract with specified entrance fee, fee address, and raffle duration. Also sets initial metadata values for the rarity of NFTs.

### enterRaffle
```solidity
function enterRaffle(address[] memory newPlayers) public payable;
```
Users enter the raffle by sending an appropriate amount of ETH. Duplicate addresses are filtered and logged in `players` array.

### refund
```solidity
function refund(uint256 playerIndex) public;
```
Allows participants to refund their entrance fee by specifying their index in the `players` array.

### getActivePlayerIndex
```solidity
function getActivePlayerIndex(address player) external view returns (uint256);
```
Returns the index of a specific player within the `players` array or 0 if not found.

### selectWinner
```solidity
function selectWinner() external;
```
Randomly selects a winner to receive an NFT and prize. Generates NFT rarity and distributes ETH accordingly.

### withdrawFees
```solidity
function withdrawFees() external;
```
Allows withdrawal of collected fees to the `feeAddress`.

### changeFeeAddress
```solidity
function changeFeeAddress(address newFeeAddress) external onlyOwner;
```
The owner changes the `feeAddress`.

### _isActivePlayer
```solidity
function _isActivePlayer() internal view returns (bool);
```
Checks if the sender is an active player.

### _baseURI
```solidity
function _baseURI() internal pure returns (string memory);
```
Returns base URI for encoding token metadata.

### tokenURI
```solidity
function tokenURI(uint256 tokenId) public view virtual override returns (string memory);
```
Generates an encoded URI for a given tokenId based on its rarity.

