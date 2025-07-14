
## List of Files in Src Folder
4-puppy-raffle-audit/src/PuppyRaffle.sol
## 4-puppy-raffle-audit/src/PuppyRaffle.sol summary
### Contract: PuppyRaffle
The `PuppyRaffle` contract implements a raffle system where participants can enter to win a dog-themed NFT. The contract inherits from `ERC721` and `Ownable` to manage NFTs and ownership.

### Key Storage Variables
- **entranceFee**: `uint256 public immutable` - Cost to enter the raffle.
- **players**: `address[] public` - Array holding all participants in the raffle.
- **raffleDuration**: `uint256 public` - Duration of the raffle in seconds.
- **raffleStartTime**: `uint256 public` - Timestamp when the raffle started.
- **previousWinner**: `address public` - Address of the last raffle winner.
- **feeAddress**: `address public` - Address where a percentage of raffle entry fees are sent.
- **totalFees**: `uint64 public` - Total collected fees.

### Functions:
1. **constructor**: 
   ```solidity
   constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration) 
   ```
   Initializes raffle parameters, participant lists, and maps rarity to URIs.

2. **enterRaffle**: 
   ```solidity
   function enterRaffle(address[] memory newPlayers) public payable
   ```
   Allows new participants to join the raffle by paying the entrance fee.

3. **refund**:
   ```solidity
   function refund(uint256 playerIndex) public 
   ```
   Allows users to withdraw from the raffle, receiving their entry fee back.

4. **getActivePlayerIndex**:
   ```solidity
   function getActivePlayerIndex(address player) external view returns (uint256)
   ```
   Returns a player's index in the current raffle.

5. **selectWinner**:
   ```solidity
   function selectWinner() external 
   ```
   Picks a winner from the players, distributing prize money and minting an NFT to the winner.

6. **withdrawFees**:
   ```solidity
   function withdrawFees() external 
   ```
   Allows the contract owner to withdraw collected fees.

7. **changeFeeAddress**:
   ```solidity
   function changeFeeAddress(address newFeeAddress) external onlyOwner 
   ```
   Updates the address where entry fees are sent.

8. **_isActivePlayer**:
   ```solidity
   function _isActivePlayer() internal view returns (bool) 
   ```
   Confirms if the caller is currently in the raffle.

9. **_baseURI**:
   ```solidity
   function _baseURI() internal pure returns (string memory) 
   ```
   Returns the base URI for NFT metadata.

10. **tokenURI**:
    ```solidity
    function tokenURI(uint256 tokenId) public view virtual override returns (string memory) 
    ```
    Provides a metadata URI for a given NFT, detailing rarity and image.


 ## DOCUMENTATION: 

