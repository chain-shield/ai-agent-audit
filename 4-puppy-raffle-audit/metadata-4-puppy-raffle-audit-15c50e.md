
## PROTOCOL OVERVIEW:

**Puppy Raffle Protocol**  

PuppyRaffle is a Solidity-based ERC-721 raffle that lets users compete for a randomly generated “puppy” NFT.  

• Entering: Anyone calls `enterRaffle(address[] participants)` and pays `entranceFee` (1 ETH) per address supplied. Duplicate addresses in the same or previous rounds are rejected, letting a single wallet buy multiple legitimate tickets.  

• Refunds: A ticket holder may call `refund(index)` before the draw to reclaim their stake; the address is zeroed in the players array, preserving array order while freeing the slot.  

• Draw: After `raffleDuration` (default 1 day) and with ≥ 4 active players, `selectWinner()` can be triggered. 90 % of pooled ETH is sent to the winner, 10 % accrues to `totalFees` for the `feeAddress`. A new ERC-721 token is minted to the winner with rarity determined by pseudo-randomness; `tokenURI` serves on-chain JSON containing name, description and image link.  

• Fees: When no players are active, owner calls `withdrawFees()` to move accumulated fees to the designated address; `changeFeeAddress()` lets the owner update that wallet.  

Comprehensive Forge tests and a deployment script are included.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
# PuppyRaffleTest Contract Summary

The `PuppyRaffleTest` is a testing contract written for the `PuppyRaffle` Solidity contract, utilizing the Forge testing framework. The tests validate the functionality of entering the raffle, refunding participation fees, finding player indices, selecting the winner, verifying NFT attributes, and withdrawing fees. Key parameters include an entrance fee of 1 ether, a fee address, and a raffle duration of 1 day.

## Functions

### setUp Function
```solidity
function setUp() public
```
Sets up the environment for testing by initializing the `PuppyRaffle` contract with the entrance fee, fee address, and duration.

### testCanEnterRaffle Function
```solidity
function testCanEnterRaffle() public
```
Tests that a player can successfully enter the raffle by sending the correct entrance fee.

### testCantEnterWithoutPaying Function
```solidity
function testCantEnterWithoutPaying() public
```
Ensures that entering the raffle without sufficient payment results in a revert.

### testCanEnterRaffleMany Function
```solidity
function testCanEnterRaffleMany() public
```
Verifies multiple players can enter the raffle simultaneously.

### testCantEnterWithoutPayingMultiple Function
```solidity
function testCantEnterWithoutPayingMultiple() public
```
Checks that entering without paying for multiple players is disallowed.

### testCantEnterWithDuplicatePlayers Function
```solidity
function testCantEnterWithDuplicatePlayers() public
```
Ensures duplicate player entries are prevented.

### testCantEnterWithDuplicatePlayersMany Function
```solidity
function testCantEnterWithDuplicatePlayersMany() public
```
Tests that duplicate entries in a larger group are not allowed.

### testCanGetRefund Function
```solidity
function testCanGetRefund() public playerEntered
```
Verifies that players can receive refunds after entering the raffle.

### testGettingRefundRemovesThemFromArray Function
```solidity
function testGettingRefundRemovesThemFromArray() public playerEntered
```
Ensures refunded players are removed from the active players' list.

### testOnlyPlayerCanRefundThemself Function
```solidity
function testOnlyPlayerCanRefundThemself() public playerEntered
```
Checks that only the player who entered can request a refund.

### testGetActivePlayerIndexManyPlayers Function
```solidity
function testGetActivePlayerIndexManyPlayers() public
```
Tests the retrieval of player indices with multiple active participants.

### testCantSelectWinnerBeforeRaffleEnds Function
```solidity
function testCantSelectWinnerBeforeRaffleEnds() public playersEntered
```
Prevents winner selection before the raffle duration ends.

### testCantSelectWinnerWithFewerThanFourPlayers Function
```solidity
function testCantSelectWinnerWithFewerThanFourPlayers() public
```
Ensures that a minimum of four players is required to select a winner.

### testSelectWinner Function
```solidity
function testSelectWinner() public playersEntered
```
Tests the process of selecting a winner from active players.

### testSelectWinnerGetsPaid Function
```solidity
function testSelectWinnerGetsPaid() public playersEntered
```
Verifies that the winner receives the correct payout.

### testSelectWinnerGetsAPuppy Function
```solidity
function testSelectWinnerGetsAPuppy() public playersEntered
```
Checks that the winner receives the appropriate NFT.

### testPuppyUriIsRight Function
```solidity
function testPuppyUriIsRight() public playersEntered
```
Validates the correct URI format for the NFT.

### testCantWithdrawFeesIfPlayersActive Function
```solidity
function testCantWithdrawFeesIfPlayersActive() public playersEntered
```
Prevents fee withdrawals when players are still active.

### testWithdrawFees Function
```solidity
function testWithdrawFees() public playersEntered
```
Ensures the correct withdrawal of fees after selecting a winner and no active players.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
### Contract: DeployPuppyRaffle

This contract is designed to deploy the `PuppyRaffle` contract. It imports functionalities from `Script` and references the `PuppyRaffle` contract.

#### Storage Variables
- **entranceFee** (`uint256`): Set to `1e18`. This represents the entrance fee for the raffle.
- **feeAddress** (`address`): Stores the address that will receive the fees.
- **duration** (`uint256`): Duration of the raffle, set to `1 days`.

#### Function: run
- **Definition**: `function run() public`
- **Summary**: Initializes the fee address with the caller's address, then deploys a new `PuppyRaffle` contract using a broadcast mechanism.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### Contract: PuppyRaffle

The `PuppyRaffle` contract is an ERC721 token implementation that allows users to enter a raffle to win a dog NFT. Participants can enter multiple times, but duplicate entries are not allowed. A refund option is available, and a regularly occurring draw selects a winner who receives a randomly minted dog NFT. A portion of fees is directed to a dedicated fee address.

### Functions:

#### constructor
```solidity
constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration)
```
Initializes the contract with entrance fee, fee address, and raffle duration. It sets up rarity mappings for different puppy types.

#### enterRaffle
```solidity
function enterRaffle(address[] memory newPlayers) public payable
```
Allows new players to enter the raffle, requiring an entrance fee for each player. Duplicate entries are checked and disallowed.

#### refund
```solidity
function refund(uint256 playerIndex) public
```
Allows a player to get a refund for their ticket, setting their spot to zero in the array of players.

#### getActivePlayerIndex
```solidity
function getActivePlayerIndex(address player) external view returns (uint256)
```
Returns the index of a player if they are active; otherwise, returns zero.

#### selectWinner
```solidity
function selectWinner() external
```
Selects a winner if the raffle duration is complete and there are at least 4 players. It distributes the prize and fee, resets participants, and mints an NFT.

#### withdrawFees
```solidity
function withdrawFees() external
```
Withdraws accumulated fees to the set fee address, provided no players are active.

#### changeFeeAddress
```solidity
function changeFeeAddress(address newFeeAddress) external onlyOwner
```
Allows the contract owner to change the fee address.

#### _isActivePlayer
```solidity
function _isActivePlayer() internal view returns (bool)
```
Checks if the message sender is an active player.

#### _baseURI
```solidity
function _baseURI() internal pure returns (string memory)
```
Returns the base URI as a constant for token metadata.

#### tokenURI
```solidity
function tokenURI(uint256 tokenId) public view virtual override returns (string memory)
```
Generates and returns token metadata URI in JSON format.

### Storage Variables:

- **entranceFee**: `uint256 public immutable entranceFee;`
  The cost to enter the raffle in wei.

- **players**: `address[] public players;`
  An array of addresses representing current raffle participants.

- **raffleDuration**: `uint256 public raffleDuration;`
  Duration of the raffle in seconds.

- **raffleStartTime**: `uint256 public raffleStartTime;`
  Timestamp when the raffle started.

- **previousWinner**: `address public previousWinner;`
  Records the address of the last raffle winner.

- **feeAddress**: `address public feeAddress;`
  Address where a portion of fees will be sent.

- **totalFees**: `uint64 public totalFees = 0;`
  Accumulates the total fees collected for withdrawal.

- **tokenIdToRarity**: `mapping(uint256 => uint256) public tokenIdToRarity;`
  Maps token IDs to their rarity.

- **rarityToUri & rarityToName**:
  Mappings for storing token rarity details and URIs.


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


