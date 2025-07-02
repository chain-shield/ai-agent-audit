
## List of Files in Src Folder
src/PuppyRaffle.sol

## README.md summary
### Puppy Raffle Documentation

The "Puppy Raffle" project enables users to enter a raffle to win a dog NFT. Users enter the raffle using the `enterRaffle` function, passing an array of addresses representing participants (no duplicates allowed). A refund mechanism is available via the `refund` function. Every X seconds, the system selects a winner to receive a mintable puppy NFT. A fee is collected by the protocol owner, who can designate a fee address, while the remainder of the value is sent to the winner.

### Getting Started

#### Requirements
- **Git** is needed for cloning and managing your local repo. Verify installation with `git --version`.
- **Foundry** is required for building the project. Verify installation with `forge --version`.

#### Quickstart
1. Clone the repository using `git clone https://github.com/Cyfrin/4-puppy-raffle-audit`
2. Navigate into the project directory with `cd 4-puppy-raffle-audit`
3. Run `make` to set things up.

You can also use **Gitpod** to work remotely without local installations.

### Usage

#### Testing
- Conduct tests using `forge test`.
- Coverage can be checked with `forge coverage`, and debug information with `forge coverage --report debug`.

### Audit Scope Details
- This audit includes the source file `src/PuppyRaffle.sol`, with a Solc version of 0.7.6. It is designed for deployment on the Ethereum network.

### Roles
- **Owner:** Deploys the protocol and can change the fee address.
- **Player:** Enters the raffle and can request a refund.

### Known Issues
There are no known issues at this time.


## src/PuppyRaffle.sol summary
### PuppyRaffle Contract Summary

**Contract Definition:**
The `PuppyRaffle` is a smart contract for entering a raffle to win a puppy-themed NFT. It leverages OpenZeppelin's ERC721 and Ownable contracts to manage NFTs and ownership capabilities respectively. Players enter the raffle by paying an entrance fee, and a winner is selected periodically.

**Contract Functions:**

- **enterRaffle**
  ```solidity
  function enterRaffle(address[] memory newPlayers) public payable
  ```
  Allows players to enter the raffle by sending the required entrance fee. Duplicate entries are checked and prevented. An event is emitted upon successful entry.

- **refund**
  ```solidity
  function refund(uint256 playerIndex) public
  ```
  Allows players to get a refund of their entrance fee. The player's address is reset in the players array, and an event is emitted.

- **getActivePlayerIndex**
  ```solidity
  function getActivePlayerIndex(address player) external view returns (uint256)
  ```
  Returns the index of a player's address within the players' array or 0 if not active.

- **selectWinner**
  ```solidity
  function selectWinner() external
  ```
  Selects a winner using on-chain randomness and mints a puppy NFT. Distributes the prize pool and fees, resets the players' array, and emits events.

- **withdrawFees**
  ```solidity
  function withdrawFees() external
  ```
  Withdraws collected fees to the fee address if no players are currently active. Resets total fees to zero.

- **changeFeeAddress**
  ```solidity
  function changeFeeAddress(address newFeeAddress) external onlyOwner
  ```
  Allows the contract owner to change the fee address and emits an event.

- **_isActivePlayer**
  ```solidity
  function _isActivePlayer() internal view returns (bool)
  ```
  Checks if the message sender is an active player in the raffle.

- **_baseURI**
  ```solidity
  function _baseURI() internal pure returns (string memory)
  ```
  Returns a base URI used for constructing token metadata.

- **tokenURI**
  ```solidity
  function tokenURI(uint256 tokenId) public view virtual override returns (string memory)
  ```
  Returns the metadata URI for a specific NFT token, including rarity attributes.

**Storage Variables:**

- **entranceFee**: `uint256`
  This is the cost required to enter the raffle. It is set during contract initialization.

- **players**: `address[]`
  An array storing the addresses of current raffle participants.

- **raffleDuration**: `uint256`
  Defines the duration, in seconds, after which a winner can be drawn.

- **raffleStartTime**: `uint256`
  Records the start time of the current raffle period.

- **previousWinner**: `address`
  The address of the most recent raffle winner.

- **feeAddress**: `address`
  Designates where a portion of raffle fees are sent.

- **totalFees**: `uint64`
  A running total of fees collected by the contract.

- **tokenIdToRarity**: `mapping(uint256 => uint256)`
  Maps NFT token IDs to their rarity values.

- **rarityToUri**: `mapping(uint256 => string)`
  Maps rarity values to corresponding image URIs.

- **rarityToName**: `mapping(uint256 => string)`
  Maps rarity values to their names for metadata purposes.

