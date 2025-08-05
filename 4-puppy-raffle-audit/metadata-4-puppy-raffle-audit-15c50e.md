
## PROTOCOL OVERVIEW:

### Puppy Raffle Protocol
Puppy Raffle is an on-chain game where users buy raffle tickets to win a randomly generated “puppy” NFT and a share of the ether pot.

1. **Enter** – Anyone calls `enterRaffle(address[] newPlayers)` supplying an array of participant addresses and `msg.value == entranceFee * newPlayers.length`. Duplicate addresses are rejected and the list is stored in `players`.
2. **Refund** – A participant can leave before the draw via `refund(index)`, receiving their ticket price back. Only the original player can claim their refund; their slot in `players` is then deleted.
3. **Draw** – After the configurable `duration` has elapsed and at least four players remain, anyone may call `selectWinner()`. A pseudo-random index is picked, the winner receives 80 % of the contract balance, and a Puppy NFT (ERC-721) with rarity-based metadata is minted to them. The remaining 20 % is earmarked as protocol fees.
4. **Fee withdrawal** – Once no active players remain, the owner can send accumulated fees to `feeAddress` with `withdrawFees()`. The owner alone may change `feeAddress`, but cannot touch player funds.

All logic is covered by extensive Foundry tests and a deployment script, ensuring secure, reproducible launches.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
### Contract Summary
The `PuppyRaffleTest` contract is a suite of tests for the `PuppyRaffle` contract in a Solidity environment set up using Foundry, a smart contract development toolkit. The tests ensure proper functionality and constraints for a raffle system involving player entries and winner selection.

### Functions

#### setUp
**Function Interface**: `function setUp() public`
This function initializes a new `PuppyRaffle` instance before each test, setting the entrance fee, fee address, and duration of the raffle.

#### testCanEnterRaffle
**Function Interface**: `function testCanEnterRaffle() public`
Tests that a player can successfully enter the raffle by sending the appropriate entrance fee, verifying that the player’s address is stored correctly.

#### testCantEnterWithoutPaying
**Function Interface**: `function testCantEnterWithoutPaying() public`
Ensures that entering the raffle without paying the required fee reverts the transaction with an appropriate error message.

#### testCanEnterRaffleMany
**Function Interface**: `function testCanEnterRaffleMany() public`
Validates that multiple players can enter the raffle simultaneously when the correct total entrance fee is paid.

#### testCantEnterWithoutPayingMultiple
**Function Interface**: `function testCantEnterWithoutPayingMultiple() public`
Checks that multiple players cannot enter the raffle if the combined paid amount is less than the required total entrance fee.

#### testCantEnterWithDuplicatePlayers
**Function Interface**: `function testCantEnterWithDuplicatePlayers() public`
Ensures that entering the raffle with duplicate player addresses reverts the transaction to prevent duplicates.

#### testCantEnterWithDuplicatePlayersMany
**Function Interface**: `function testCantEnterWithDuplicatePlayersMany() public`
Similar to the previous test, checks duplicate player restriction with multiple entries.

#### testCanGetRefund
**Function Interface**: `function testCanGetRefund() public`
Verifies that a player can get a refund after entering the raffle by storing player entry and using `getActivePlayerIndex` to validate the refund process.

#### testGettingRefundRemovesThemFromArray
**Function Interface**: `function testGettingRefundRemovesThemFromArray() public`
Checks that players are removed from active players array post-refund, ensuring proper state management.

#### testOnlyPlayerCanRefundThemself
**Function Interface**: `function testOnlyPlayerCanRefundThemself() public`
Ensures only the entering player can refund themselves, validating player-specific access control for refunds.

#### testGetActivePlayerIndexManyPlayers
**Function Interface**: `function testGetActivePlayerIndexManyPlayers() public`
Tests the retrieval of correct index positions for multiple players entered into the raffle.

#### testCantSelectWinnerBeforeRaffleEnds
**Function Interface**: `function testCantSelectWinnerBeforeRaffleEnds() public`
Validates that the `selectWinner` functionality can't be invoked before the specified raffle duration has elapsed.

#### testCantSelectWinnerWithFewerThanFourPlayers
**Function Interface**: `function testCantSelectWinnerWithFewerThanFourPlayers() public`
Prevents the selection of a winner if there are fewer than four players in total, ensuring fair raffle conduct.

#### testSelectWinner
**Function Interface**: `function testSelectWinner() public`
Simulates the complete raffle cycle where a winner is selected, asserting the stored previous winner matches the drawn winner.

#### testSelectWinnerGetsPaid
**Function Interface**: `function testSelectWinnerGetsPaid() public`
Checks that the winner receives the correct payout amount calculated as 80% of the total collected pot.

#### testSelectWinnerGetsAPuppy
**Function Interface**: `function testSelectWinnerGetsAPuppy() public`
Tests that the winner of the raffle receives a tokenized puppy, simulating a reward mechanism.

#### testPuppyUriIsRight
**Function Interface**: `function testPuppyUriIsRight() public`
Ensures the token URI for the puppy prize is accurate, extending validity verification for token rewards.

#### testCantWithdrawFeesIfPlayersActive
**Function Interface**: `function testCantWithdrawFeesIfPlayersActive() public`
Prevents fee withdrawal if active players remain, enforcing proper lifecycle management.

#### testWithdrawFees
**Function Interface**: `function testWithdrawFees() public`
Ensures the designated fee address receives the correct fee amount after winner selection, ensuring proper distribution.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
### Contract `DeployPuppyRaffle`
This contract is used to automate the deployment of a `PuppyRaffle` smart contract. It is a script designed for use with the Foundry framework, specifically leveraging the `forge-std/Script.sol` library for deployment scripting. The primary purpose is to initialize and deploy the `PuppyRaffle` contract with predefined parameters.

### Function `run`

```solidity
def run()
```
The `run` function is the main execution point for the deployment script. It sets the address receiving fees to the deployer's address (`msg.sender`), broadcasts a transaction using the `vm.broadcast()` method, and then deploys a new instance of the `PuppyRaffle` contract with specified parameters: an entrance fee of `1e18` wei, the current deployer's address as the fee recipient, and a raffle duration of 1 day.

### Storage Variables
- **`entranceFee`** (`uint256`): Defines the entrance fee for participating in the raffle, set to `1e18` wei.
- **`feeAddress`** (`address`): The Ethereum address designated to receive fees from the raffle participation. Initially set to the address of the deployer (`msg.sender`).
- **`duration`** (`uint256`): Specifies the duration for which the raffle will be active, set as 1 day.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### Contract: PuppyRaffle
The `PuppyRaffle` contract allows users to participate in a raffle for a chance to win an NFT of a "puppy" with varying rarity. It's built using Solidity 0.7.6 and extends OpenZeppelin's ERC721 and Ownable contracts. The owner can set a fee address to distribute funds collected from entrants.

#### Function: `enterRaffle`
**Interface**: `function enterRaffle(address[] memory newPlayers) public payable`
Allows users to enter the raffle by submitting an array of player addresses and paying a fee per entrant. Prevents any duplicate entries and fires a `RaffleEnter` event.


#### Function: `refund`
**Interface**: `function refund(uint256 playerIndex) public`
Allows players to claim a refund by providing their indexed position in the players list. Ensures the caller is the entrant requesting a refund.


#### Function: `getActivePlayerIndex`
**Interface**: `function getActivePlayerIndex(address player) external view returns (uint256)`
Finds and returns the index of a player in the array of current entrants. If the player is not active, returns 0.


#### Function: `selectWinner`
**Interface**: `function selectWinner() external`
Selects a random winner from the players once the raffle duration ends, mints a "puppy" NFT, and transfers 80% of the pooled funds to the winner, with 20% going to the fee address. Resets the raffle state by deleting players.


#### Function: `withdrawFees`
**Interface**: `function withdrawFees() external`
Withdraws accumulated fees to the specified fee address. Preconditions include no active players and the contract balance matching the total fees.


#### Variable: `entranceFee`
**Definition**: `uint256 public immutable entranceFee;`
Stores the cost of entry into the raffle. It is an immutable value set upon contract creation and used as a multiplier for calculating entry costs based on participants.


#### Variable: `feeAddress`
**Definition**: `address public feeAddress;`
Holds the address where accumulated fees are sent. The contract owner can change this address via `changeFeeAddress`. Default is set during construction.


#### Variable: `players`
**Definition**: `address[] public players;`
Keeps track of the entered players in the raffle. The list excludes active players who have successfully claimed a refund and is cleared upon winner selection.


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


