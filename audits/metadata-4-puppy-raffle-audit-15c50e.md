
## SLITHER GENERATED METADATA 


## List of Files in Src Folder
PuppyRaffle.sol
## 4-puppy-raffle-audit/src/PuppyRaffle.sol SUMMARY OF MAIN FILES
**Contract Summary**

**PuppyRaffle Contract**: This contract allows users to participate in a raffle to win an NFT depicting a random puppy. Participants can enter the raffle, get refunded, and a winner is periodically selected to receive a unique puppy NFT.

**Key Functions**:

- **enterRaffle(address[] memory newPlayers)**: Allows users to enter the raffle by providing a list of participant addresses. Each address needs to pay a fee, and duplicates are not allowed.
  ```solidity
  function enterRaffle(address[] memory newPlayers) public payable
  ```
- **refund(uint256 playerIndex)**: Enables users to get a refund from the raffle. Uses the player's index in the list for refund transactions.
  ```solidity
  function refund(uint256 playerIndex) public
  ```
- **getActivePlayerIndex(address player)**: Returns the index of a player in the participants' list, or 0 if not active.
  ```solidity
  function getActivePlayerIndex(address player) external view returns (uint256)
  ```
- **selectWinner()**: Selects a raffle winner using random on-chain data, mints an NFT for them, and distributes the prize funds.
  ```solidity
  function selectWinner() external
  ```
- **withdrawFees()**: Withdraws collected fees to the designated fee address after confirming no active players.
  ```solidity
  function withdrawFees() external
  ```
- **changeFeeAddress(address newFeeAddress)**: Updates the address to which fees are sent; can only be changed by the contract owner.
  ```solidity
  function changeFeeAddress(address newFeeAddress) external onlyOwner
  ```
- **tokenURI(uint256 tokenId)**: Returns the metadata URI for the given token ID, used to retrieve the NFT's displayed information.
  ```solidity
  function tokenURI(uint256 tokenId) public view virtual override returns (string memory)
  ```

**Key Variables**:

- **entranceFee**: The immutable fee required to enter the raffle.
  ```solidity
  uint256 public immutable entranceFee;
  ```
- **players**: An array holding the addresses of current raffle entrants.
  ```solidity
  address[] public players;
  ```
- **raffleDuration**: Duration for which the raffle continues before a winner is drawn.
  ```solidity
  uint256 public raffleDuration;
  ```
- **feeAddress**: The address designated to receive a share of collected funds.
  ```solidity
  address public feeAddress;
  ```
- **totalFees**: Keeps track of total fees collected for withdrawal.
  ```solidity
  uint64 public totalFees = 0;
  ```
- **tokenIdToRarity**: Mapping from token IDs to rare traits (common, rare, legendary).
  ```solidity
  mapping(uint256 => uint256) public tokenIdToRarity;
  ```


 ## DOCUMENTATION: 

