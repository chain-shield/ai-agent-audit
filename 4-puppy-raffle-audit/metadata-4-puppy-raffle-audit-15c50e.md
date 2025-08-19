
## PROTOCOL OVERVIEW:

### Puppy Raffle Overview  
Puppy Raffle is an on-chain ERC-721 raffle that lets anyone buy tickets to win a randomly generated dog NFT.

1. Deployment sets an immutable ticket price (`entranceFee`), a fee recipient, and how long each round lasts (`raffleDuration`).  
2. `enterRaffle(address[] newPlayers)` is payable; `msg.value` must equal `entranceFee * newPlayers.length`. It records each unique address, rejecting duplicates or under-payment. Group entries or multiple tickets are allowed by passing multiple addresses.  
3. Any player can exit before the draw by calling `refund(index)`, voiding their ticket and returning their funds.  
4. When `raffleDuration` has elapsed and at least four active players exist, anyone may call `selectWinner()`. A pseudo-random index picks the winner, PuppyRaffle mints an NFT to them with rarity metadata, and sends the pot minus a protocol fee to the winner. The fee is stored in `totalFees`.  
5. After all players have either won or refunded, the owner can `withdrawFees()` to move the accumulated fees to `feeAddress`, and can update that address via `changeFeeAddress()`.  

Comprehensive Foundry tests cover duplicate entries, refunds, prize payment, URI correctness, and fee withdrawal.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
## PuppyRaffleTest Contract
- **Contract Definition**: `contract PuppyRaffleTest is Test {...}`
- **Purpose**: This contract is designed to test the `PuppyRaffle` contract functionalities, including entering the raffle, issuing refunds, selecting a winner, and withdrawing fees.

## Functions
### setUp
- **Interface**: `function setUp() public {...}`
- **Summary**: Initializes a `PuppyRaffle` contract instance with predefined parameters including entrance fee, fee address, and raffle duration.

### testCanEnterRaffle
- **Interface**: `function testCanEnterRaffle() public {...}`
- **Summary**: Verifies that a player can enter the raffle by sending the correct entrance fee, and asserts that the player is recorded.

### testCantEnterWithoutPaying
- **Interface**: `function testCantEnterWithoutPaying() public {...}`
- **Summary**: Ensures that entering the raffle without paying the required fee results in a transaction revert.

### testCanEnterRaffleMany
- **Interface**: `function testCanEnterRaffleMany() public {...}`
- **Summary**: Confirms multiple players can enter the raffle simultaneously if the total entrance fee is paid.

### testCantEnterWithoutPayingMultiple
- **Interface**: `function testCantEnterWithoutPayingMultiple() public {...}`
- **Summary**: Checks that multiple players cannot enter the raffle unless the fee is fully paid for all entrants.

### testCantEnterWithDuplicatePlayers
- **Interface**: `function testCantEnterWithDuplicatePlayers() public {...}`
- **Summary**: Validates that a player cannot enter the raffle more than once by rejecting duplicate entries.

### testCantEnterWithDuplicatePlayersMany
- **Interface**: `function testCantEnterWithDuplicatePlayersMany() public {...}`
- **Summary**: Ensures that the raffle rejects entries with duplicate players, even with multiple entrants.

### testCanGetRefund
- **Interface**: `function testCanGetRefund() public playerEntered {...}`
- **Summary**: Tests the refund functionality, confirming that players can retrieve their entrance fee, and their record is removed.

### testGettingRefundRemovesThemFromArray
- **Interface**: `function testGettingRefundRemovesThemFromArray() public playerEntered {...}`
- **Summary**: Ensures that once a refund is processed for a player, the player's entry is removed from the raffle list.

### testOnlyPlayerCanRefundThemself
- **Interface**: `function testOnlyPlayerCanRefundThemself() public playerEntered {...}`
- **Summary**: Verifies that only a player who entered the raffle can request a refund, preventing unauthorized refunds.

### testGetActivePlayerIndexManyPlayers
- **Interface**: `function testGetActivePlayerIndexManyPlayers() public {...}`
- **Summary**: Confirms the correct retrieval of a player's index in the active players' list when multiple players are involved.

### testCantSelectWinnerBeforeRaffleEnds
- **Interface**: `function testCantSelectWinnerBeforeRaffleEnds() public playersEntered {...}`
- **Summary**: Ensures that the raffle cannot prematurely select a winner before the raffle ends.

### testCantSelectWinnerWithFewerThanFourPlayers
- **Interface**: `function testCantSelectWinnerWithFewerThanFourPlayers() public {...}`
- **Summary**: Confirms that a minimum of four players is required to select a winner for the raffle.

### testSelectWinner
- **Interface**: `function testSelectWinner() public playersEntered {...}`
- **Summary**: Validates the selection of a winner once the raffle ends, ensuring accurate record keeping of the winner.

### testSelectWinnerGetsPaid
- **Interface**: `function testSelectWinnerGetsPaid() public playersEntered {...}`
- **Summary**: Asserts that the winner receives the correct payout after being selected in the raffle.

### testSelectWinnerGetsAPuppy
- **Interface**: `function testSelectWinnerGetsAPuppy() public playersEntered {...}`
- **Summary**: Ensures the winner receives a puppy (NFT) as part of their prize after winning the raffle.

### testPuppyUriIsRight
- **Interface**: `function testPuppyUriIsRight() public playersEntered {...}`
- **Summary**: Verifies the NFT token URI for the winner's puppy is set correctly, ensuring metadata is accurate.

### testCantWithdrawFeesIfPlayersActive
- **Interface**: `function testCantWithdrawFeesIfPlayersActive() public playersEntered {...}`
- **Summary**: Prevents fee withdrawal when there are active players in the raffle, ensuring funds are appropriately allocated.

### testWithdrawFees
- **Interface**: `function testWithdrawFees() public playersEntered {...}`
- **Summary**: Confirms fees can be withdrawn once the raffle concludes, allocating a portion of entrance fees as operational funds.

## Variables
- **`puppyRaffle`**: Instance of `PuppyRaffle` used for executing test scenarios.
- **`entranceFee`**: Amount required for a player to enter the raffle.
- **`feeAddress`**: Address designated to receive fees from the raffle.
- **`duration`**: Specifies the time duration for which the raffle runs.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
#### DeployPuppyRaffle Contract
The `DeployPuppyRaffle` is a deployment script for a PuppyRaffle smart contract. It leverages the `Script` class from "forge-std/Script.sol" and imports `PuppyRaffle` from "../src/PuppyRaffle.sol". 

##### Variables
- **entranceFee** : `uint256 entranceFee = 1e18;` - The fee participants must pay to enter the raffle, set at 1 ether.
- **feeAddress** : `address feeAddress;` - Address where collected fees are sent, initialized to the sender's address in the `run` function.
- **duration** : `uint256 duration = 1 days;` - Duration of the raffle, set to one day.

##### run Function
- **Interface** : `function run() public {}`
- **Summary** : This function sets the `feeAddress` to the message sender. It then broadcasts the transaction and creates a new instance of the `PuppyRaffle` contract by passing the entrance fee, fee address, and duration as parameters, effectively deploying the contract on the blockchain.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### Contract: PuppyRaffle
The `PuppyRaffle` contract is an ERC721 token contract for entering a raffle to win a dog NFT. Participants can enter the raffle by paying an entrance fee. The contract restricts duplicate addresses and allows refunds. At the end of the raffle duration, a winner is selected randomly. The prize funds are split between the winner and a fee address.

#### Functions:
##### constructor
```solidity
constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration)
```
Initializes the contract with the entrance fee, fee address, and raffle duration. Sets the start time and defines rarity mappings for NFTs.

##### enterRaffle
```solidity
function enterRaffle(address[] memory newPlayers) public payable
```
Players enter the raffle by paying a fee for each participant. Emits `RaffleEnter` event. Requires no duplicates among new players.

##### refund
```solidity
function refund(uint256 playerIndex) public
```
Allows players to refund their entrance fee. Sets their entry slot to zero and emits `RaffleRefunded`.

##### getActivePlayerIndex
```solidity
function getActivePlayerIndex(address player) external view returns (uint256)
```
Returns the index of a player if active, or zero otherwise.

##### selectWinner
```solidity
function selectWinner() external
```
Selects and rewards a winner from participants if the raffle duration has passed and there are enough players. Resets the raffle state.

##### withdrawFees
```solidity
function withdrawFees() external
```
Withdraws the collected fees to the fee address after all active players are refunded.

##### changeFeeAddress
```solidity
function changeFeeAddress(address newFeeAddress) external onlyOwner
```
Allows the contract owner to change the fee address.

##### _isActivePlayer
```solidity
function _isActivePlayer() internal view returns (bool)
```
Checks if the sender is an active player.

##### _baseURI
```solidity
function _baseURI() internal pure returns (string memory)
```
Returns the base URI for token metadata.

##### tokenURI
```solidity
function tokenURI(uint256 tokenId) public view virtual override returns (string memory)
```
Returns the metadata URI for a given token ID based on its rarity.

#### Storage Variables:
- **entranceFee**: `uint256 public immutable`; The cost to enter the raffle.
- **players**: `address[] public`; List of participants in the raffle.
- **raffleDuration**: `uint256 public`; Duration of the raffle in seconds.
- **feeAddress**: `address public`; Address for fee collection.
- **totalFees**: `uint64 public`; Accumulated fee amounts.
- **tokenIdToRarity**: `mapping(uint256 => uint256) public`; Maps a token ID to its rarity level.
- **rarityToUri**: `mapping(uint256 => string) public`; Maps a rarity level to an image URI.
- **rarityToName**: `mapping(uint256 => string) public`; Maps a rarity level to a name.


## Main List of Files in Project

script/DeployPuppyRaffle.sol
src/PuppyRaffle.sol
test/PuppyRaffleTest.t.sol


 ## DOCUMENTATION: 

 ### README.md

<p align="center">
<img src="./images/puppy-raffle.svg" width="400" alt="puppy-raffle">
<br/>

# Puppy Raffle

This project is to enter a raffle to win a cute dog NFT. The protocol should do the following:

1. Call the `enterRaffle` function with the following parameters:
   1. `address[] participants`: A list of addresses that enter. You can use this to enter yourself multiple times, or yourself and a group of your friends.
2. Duplicate addresses are not allowed
3. Users are allowed to get a refund of their ticket & `value` if they call the `refund` function
4. Every X seconds, the raffle will be able to draw a winner and be minted a random puppy
5. The owner of the protocol will set a feeAddress to take a cut of the `value`, and the rest of the funds will be sent to the winner of the puppy.

- [Puppy Raffle](#puppy-raffle)
- [Getting Started](#getting-started)
  - [Requirements](#requirements)
  - [Quickstart](#quickstart)
    - [Optional Gitpod](#optional-gitpod)
- [Usage](#usage)
  - [Testing](#testing)
    - [Test Coverage](#test-coverage)
- [Audit Scope Details](#audit-scope-details)
  - [Compatibilities](#compatibilities)
- [Roles](#roles)
- [Known Issues](#known-issues)

# Getting Started

## Requirements

- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
  - You'll know you did it right if you can run `git --version` and you see a response like `git version x.x.x`
- [foundry](https://getfoundry.sh/)
  - You'll know you did it right if you can run `forge --version` and you see a response like `forge 0.2.0 (816e00b 2023-03-16T00:05:26.396218Z)`

## Quickstart

```
git clone https://github.com/Cyfrin/4-puppy-raffle-audit
cd 4-puppy-raffle-audit
make
```

### Optional Gitpod

If you can't or don't want to run and install locally, you can work with this repo in Gitpod. If you do this, you can skip the `clone this repo` part.

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#github.com/Cyfrin/4-puppy-raffle-audit)

# Usage

## Testing

```
forge test
```

### Test Coverage

```
forge coverage
```

and for coverage based testing:

```
forge coverage --report debug
```

# Audit Scope Details

- Commit Hash: 2a47715b30cf11ca82db148704e67652ad679cd8
- In Scope:

```
./src/
└── PuppyRaffle.sol
```

## Compatibilities

- Solc Version: 0.7.6
- Chain(s) to deploy contract to: Ethereum

# Roles

Owner - Deployer of the protocol, has the power to change the wallet address to which fees are sent through the `changeFeeAddress` function.
Player - Participant of the raffle, has the power to enter the raffle with the `enterRaffle` function and refund value through `refund` function.

# Known Issues

None



 ## CONFIG FILES: 

 ### foundry.toml

[profile.default]
src = "src"
out = "out"
libs = ["lib"]

remappings = ['@openzeppelin/contracts=lib/openzeppelin-contracts/contracts']

# See more config options https://github.com/foundry-rs/foundry/blob/master/crates/config/README.md#all-options


