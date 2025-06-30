
## List of Files in Src Folder
src/PuppyRaffle.sol

## README.md summary
The Puppy Raffle project facilitates entry into a raffle to win a cute dog NFT by allowing users to call the `enterRaffle` function with a list of participants' addresses. Duplicate entries are prohibited, and participants can request a refund through the `refund` function. At regular intervals (every X seconds), a winner is selected to mint a random puppy NFT. The protocol owner can designate a feeAddress to collect a portion of the funds, with the remainder going to the winner.

**Getting Started**: The main requirements are git and foundry installation, confirming versions with simple commands. After cloning the repo and navigating into the directory, users can build the project.

**Usage**: Testing can be done using the `forge test` command, with coverage checked via `forge coverage` commands, including a debug mode.

**Audit Scope**: The focus is on the `PuppyRaffle.sol` file in the `src` directory, from commit hash 2a47715b30cf11ca82db148704e67652ad679cd8. The contract is compatible with Solidity version 0.7.6 and is intended for Ethereum deployment.

**Roles**: The Owner can change the fee-receiving address, while Players can enter the raffle or request a refund.

**Known Issues**: No issues have been reported.


## src/PuppyRaffle.sol summary
The `PuppyRaffle` contract is a blockchain raffle system allowing users to enter in conjunction to win a non-fungible token (NFT) representing a virtual puppy. This contract inherits from the OpenZeppelin `ERC721` and `Ownable` contracts, implementing an NFT structure with raffle mechanics and owner-controlled fee settings.

Main Storage Variables:
- `address[] public players`: Array storing participants in the raffle lottery. Ensures no duplicates and triggers winner selection once criteria met.
- `uint256 public immutable entranceFee`: The set cost for each player to enter the raffle, calculated as the product of participants.
- `uint256 public raffleDuration`: Duration in seconds after which the raffle draws a winner, ensuring adherence and timing control.
- `address public feeAddress`: Address designated for collecting fees generated from raffle entries.
- `uint64 public totalFees`: Tracks accumulated fees due to be withdrawn by owner, enabling profit calculation.
- `mapping(uint256 => uint256) public tokenIdToRarity`: Associates token IDs with rarity for NFTs, defining attributes on mint.
- `mapping(uint256 => string) public rarityToUri` and `mapping(uint256 => string) public rarityToName`: Define URI and names for varying puppy rarities (common, rare, legendary), enhancing metadata diversity.

Key Functions:
**constructor**: `constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration) ERC721("Puppy Raffle", "PR")` initializes contract parameters and sets up rarity data storage for later NFT minting.
**enterRaffle**: `function enterRaffle(address[] memory newPlayers) public payable` checks for correct payment and adds non-duplicate players to the list, ensuring eligibility for winner drawing.
**refund**: `function refund(uint256 playerIndex) public` allows participants to reclaim entrance fees by specifying their array index, maintaining mutable entry lists.
**getActivePlayerIndex**: `function getActivePlayerIndex(address player) external view returns (uint256)` fetches the raffle entry index for a given player address, assisting refund operations.
**selectWinner**: `function selectWinner() external` determines the winner and mints pet NFTs by managing fees, participant reset, and inheritance settings, fostering randomness.
**withdrawFees**: `function withdrawFees() external` moves the cumulative fees collected to the specified fee address, ensuring proper reward distribution post-raffle.
**changeFeeAddress**: `function changeFeeAddress(address newFeeAddress) external onlyOwner` provides owner-adjusted fee collection changing, maintaining economic flexibility.
**_isActivePlayer**: `function _isActivePlayer() internal view returns (bool)` planned to internally verify current player's active status through looped address search.
**_baseURI** and **tokenURI**: `function _baseURI() internal pure returns (string memory)`, `function tokenURI(uint256 tokenId) public view virtual override returns (string memory)` manage base URI encoding for tokens, establishing decentralized data storage.

