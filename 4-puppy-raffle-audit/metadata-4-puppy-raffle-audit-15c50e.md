
## PROTOCOL OVERVIEW:

### Puppy Raffle Protocol

Puppy Raffle is an on-chain raffle that lets anyone vie for a cute dog NFT. Players call `enterRaffle`, passing an array of addresses and sending `entranceFee × n` ETH. The contract rejects duplicate addresses and records entrants in `players`. Until `raffleDuration` elapses, any entrant may call `refund` to reclaim their stake; only the player herself can trigger her own refund.

When the timer is up, anyone can invoke `selectWinner`. The function picks a pseudo-random player index, mints an ERC-721 puppy with rarity metadata, transfers the prize pot minus fees to the winner, and routes accumulated fees to `feeAddress`. The last winner is stored in `previousWinner` for transparency.

If no players are active, the owner can call `withdrawFees` to collect residual fees and may update `feeAddress` through `changeFeeAddress`.

Extensive Foundry tests cover entering, duplicate prevention, refunds, winner payout, URI correctness, and fee withdrawal. A Forge script automates deployment with a 1 ETH entrance fee, the deployer as `feeAddress`, and a 1-day raffle duration.

In short, Puppy Raffle combines a fair ticketing mechanism, secure fund handling, and NFT rewards in under 300 lines of Solidity.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
### Contract Definition
The `PuppyRaffleTest` contract is a solidity test file for the `PuppyRaffle` contract. It is designed using the Foundry testing framework.

### Function: `setUp`
```solidity
function setUp() public
```
This function initializes the `PuppyRaffle` contract with a specified entrance fee, fee address, and raffle duration. It sets up the environment for subsequent tests.

### Function: `testCanEnterRaffle`
```solidity
function testCanEnterRaffle() public
```
This test verifies that a player can successfully enter the raffle by sending the correct entrance fee.

### Function: `testCantEnterWithoutPaying`
```solidity
function testCantEnterWithoutPaying() public
```
This test checks that entering the raffle without paying the entrance fee is reverted with the correct error message.

### Function: `testCanEnterRaffleMany`
```solidity
function testCanEnterRaffleMany() public
```
This test ensures multiple players can enter the raffle simultaneously when the correct total fee is sent.

### Function: `testCantEnterWithoutPayingMultiple`
```solidity
function testCantEnterWithoutPayingMultiple() public
```
This test validates that entering with multiple players without sufficient total fees is reverted.

### Function: `testCantEnterWithDuplicatePlayers`
```solidity
function testCantEnterWithDuplicatePlayers() public
```
This test checks that attempting to enter the raffle with duplicate players results in a revert.

### Modifier: `playerEntered`
```solidity
modifier playerEntered()
```
Ensures that a player has entered the raffle before proceeding with the executed function.

### Function: `testCanGetRefund`
```solidity
function testCanGetRefund() public playerEntered
```
This test verifies that a player can successfully get a refund and is removed from the players array.

### Function: `testOnlyPlayerCanRefundThemself`
```solidity
function testOnlyPlayerCanRefundThemself() public playerEntered
```
This test asserts that only the player who entered can initiate their refund.

### Function: `testGetActivePlayerIndexManyPlayers`
```solidity
function testGetActivePlayerIndexManyPlayers() public
```
Tests that the correct indices are returned for multiple players within the raffle.

### Function: `testCantSelectWinnerBeforeRaffleEnds`
```solidity
function testCantSelectWinnerBeforeRaffleEnds() public playersEntered
```
This test ensures that trying to select a winner before the raffle ends is reverted with the appropriate error message.

### Modifier: `playersEntered`
```solidity
modifier playersEntered()
```
Ensures multiple players have entered the raffle before continuing with the function execution.

### Function: `testSelectWinner`
```solidity
function testSelectWinner() public playersEntered
```
Tests that a winner can be selected correctly once the raffle duration has ended.

### Function: `testSelectWinnerGetsPaid`
```solidity
function testSelectWinnerGetsPaid() public playersEntered
```
This test ensures the raffle winner receives the correct payout upon winning.

### Function: `testPuppyUriIsRight`
```solidity
function testPuppyUriIsRight() public playersEntered
```
Ensures the token URI for the winning puppy is set correctly.

### Function: `testCantWithdrawFeesIfPlayersActive`
```solidity
function testCantWithdrawFeesIfPlayersActive() public playersEntered
```
Validates that fees cannot be withdrawn while there are active players.

### Function: `testWithdrawFees`
```solidity
function testWithdrawFees() public playersEntered
```
Tests that fees are correctly withdrawn to the fee address after a winner is selected.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
### DeployPuppyRaffle Contract

The `DeployPuppyRaffle` contract is a deployment script for the `PuppyRaffle` contract. It extends the `Script` from Forge, facilitating automation in test and deployment scenarios. This script sets up deployment parameters and executes the creation of the `PuppyRaffle` contract instance with predefined configuration.

### Key Components:

- **Variables**:
  - `uint256 entranceFee`: Set to 1 ether, represents the fee required to enter the raffle.
  - `address feeAddress`: The address to which fees are sent, initialized as the deployer’s address in the `run` function.
  - `uint256 duration`: Duration for the raffle event, set to 1 day.

### Function:

- **run()**: `public`
  - **Interface**: `function run() public`
  - **Summary**: This function is responsible for actually deploying the `PuppyRaffle` contract. It sets `feeAddress` to the caller's address and invokes `vm.broadcast()` to signal deployment through Forge’s script environment. Then, it instantiates a new `PuppyRaffle` contract with the specified `entranceFee`, `feeAddress`, and `duration`. This deploys the `PuppyRaffle` contract on the blockchain.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### PuppyRaffle Contract

The `PuppyRaffle` contract is an Ethereum smart contract developed by PuppyLoveDAO, allowing participants to enter a raffle to win an NFT of a cute dog. The contract inherits from OpenZeppelin's `ERC721` and `Ownable` contracts. It manages a raffle system, ensuring no duplicate entries, allowing refunds, and handling the drawing of winners and minting of NFTs.

#### Functions:

- **enterRaffle(address[] memory newPlayers)**: 
  Takes a list of player addresses, collects entrance fees, checks for duplicates, and emits a RaffleEnter event. 
  ```solidity
  function enterRaffle(address[] memory newPlayers) public payable 
```

- **refund(uint256 playerIndex)**: 
  Allows participants to receive refunds; checks that the caller is the rightful player and not already refunded. 
  ```solidity
  function refund(uint256 playerIndex) public 
```

- **getActivePlayerIndex(address player)**: 
  Retrieves the index of a player in the array; returns 0 if not active.
  ```solidity
  function getActivePlayerIndex(address player) external view returns (uint256) 
```

- **selectWinner()**: 
  Selects a raffle winner if conditions are met, mints the NFT, and distributes the prize and fees.
  ```solidity
  function selectWinner() external 
```

- **withdrawFees()**: 
  Allows withdrawal of accumulated fees to the feeAddress if no players are active.
  ```solidity
  function withdrawFees() external
```

- **changeFeeAddress(address newFeeAddress)**: 
  Owner-only function to update the fee collecting address.
  ```solidity
  function changeFeeAddress(address newFeeAddress) external onlyOwner
```

- **_isActivePlayer()**: 
  Checks if the sender is an active player.
  ```solidity
  function _isActivePlayer() internal view returns (bool) 
```

- **_baseURI()**: 
  Defines the base URI as a constant. 
  ```solidity
  function _baseURI() internal pure returns (string memory)
```

- **tokenURI(uint256 tokenId)**: 
  Overrides the base function to return the token URI with relevant metadata.
  ```solidity
  function tokenURI(uint256 tokenId) public view virtual override returns (string memory)
```

#### Storage Variables:

- **entranceFee**: The cost of entering the raffle, set at contract construction.
  ```solidity
  uint256 public immutable entranceFee;
```

- **players**: An array of player addresses participating in the raffle.
  ```solidity
  address[] public players;
```

- **raffleDuration**: Duration of the raffle in seconds.
  ```solidity
  uint256 public raffleDuration;
```

- **raffleStartTime**: Timestamp of when the raffle began.
  ```solidity
  uint256 public raffleStartTime;
```

- **previousWinner**: Stores the address of the previous winner of the raffle.
  ```solidity
  address public previousWinner;
```

- **feeAddress**: Address where the owner-defined fee portion of collected funds is sent.
  ```solidity
  address public feeAddress;
```

- **totalFees**: Accumulated fees collected over time.
  ```solidity
  uint64 public totalFees = 0;
```

- **tokenIdToRarity, rarityToUri, rarityToName**: Mappings to track the rarity and metadata URIs for NFTs by token ID.
  ```solidity
  mapping(uint256 => uint256) public tokenIdToRarity; 
mapping(uint256 => string) public rarityToUri;
mapping(uint256 => string) public rarityToName;
```

The contract implements NFT minting based on raffle outcomes, with varying puppy rarities represented through metadata. It handles secure fund distribution, refund processes, and owner-controlled fee management.


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


