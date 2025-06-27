
## List of Files in Src Folder
src/PuppyRaffle.sol

## README.md summary
The Puppy Raffle is a blockchain-based raffle system where participants can enter to win a dog-themed NFT. Participants enter the raffle by calling the `enterRaffle` function with a list of addresses, ensuring no duplicates. Participants can also refund their tickets via the `refund` function. A random winner is drawn at intervals, and the prize includes funds remaining after a fee is deducted. The project requires Git and Foundry for setup and testing. The code uses Solc version 0.7.6 and is compatible with Ethereum. The owner of the protocol can change the fee address, which collects a portion of the raffle's value, while players can enter raffles and refund their value.


## src/PuppyRaffle.sol summary
The `PuppyRaffle` contract enables users to participate in a raffle to win a dog NFT. Participants can enter the raffle by paying an entrance fee, and are prohibited from entering more than once. A refund function allows participants to withdraw if necessary. After a set duration and ensuring a minimum number of participants, a winner is selected using a pseudo-random process.

### Storage Variables Summary
- `entranceFee`: An immutable value representing the cost to join the raffle.
- `players`: An array storing addresses of all participants.
- `raffleDuration`: Duration of the raffle, measured in seconds.
- `raffleStartTime`: Records the start time of the raffle.
- `previousWinner`: Stores the address of the past raffle winner.
- `feeAddress`: Address to which fees are sent.
- `totalFees`: Maintains total fees collected, stored in less gas-consuming format.
- `tokenIdToRarity`, `rarityToUri`, `rarityToName`: Mappings correlating token IDs with rarity, URIs, and names, respectively.

### Function Overview

- **enterRaffle**
  ```solidity
  function enterRaffle(address[] memory newPlayers) public payable
  ```
  Allows players to enter the raffle by sending entrance fees proportionate to the number of new entries. It ensures no duplicate addresses among participants.

- **refund**
  ```solidity
  function refund(uint256 playerIndex) public
  ```
  Allows players to refund their entrance fee by referring to their index in the players' list.

- **getActivePlayerIndex**
  ```solidity
  function getActivePlayerIndex(address player) external view returns (uint256)
  ```
  Returns the index of the player in the participants' list, identifying inactive players by returning 0.

- **selectWinner**
  ```solidity
  function selectWinner() external
  ```
  Selects a raffle winner if conditions are met, mints NFT to winner, allocates prize pool, and updates historic results.

- **withdrawFees**
  ```solidity
  function withdrawFees() external
  ```
  Transfers collected fees to designated fee address contingent on absence of active players.

- **changeFeeAddress**
  ```solidity
  function changeFeeAddress(address newFeeAddress) external onlyOwner
  ```
  Updates the address designated to receive fees, restricted to contract owner.

- **_isActivePlayer**
  ```solidity
  function _isActivePlayer() internal view returns (bool)
  ```
  Checks if the caller is a current raffle participant.

- **_baseURI**
  ```solidity
  function _baseURI() internal pure returns (string memory)
  ```
  Provides the base URI used in encoding metadata as base64.

- **tokenURI**
  ```solidity
  function tokenURI(uint256 tokenId) public view virtual override returns (string memory)
  ```
  Generates the URI containing metadata for a specific NFT based on its token ID and rarity.

