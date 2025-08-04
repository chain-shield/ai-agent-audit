
## PROTOCOL OVERVIEW:

**Puppy Raffle** is an on-chain raffle that lets anyone win a collectible Puppy NFT while sharing ETH prize money with a fee recipient.

• Players call `enterRaffle(address[] newPlayers)` and send `entranceFee` (1 ETH by default) for _each_ supplied address. The contract rejects duplicate entries and under-payment, keeping a clean `players` array.

• At any time before a winner is drawn a player may invoke `refund(uint256 index)` to cancel their ticket and reclaim the full fee, automatically removing their slot.

• Once `raffleDuration` (1 day in the deployment script) has elapsed **and** at least four unique players exist, anyone can call `selectWinner()`. A pseudo-random index is chosen, the winner is minted an ERC-721 Puppy whose metadata encodes rarity (common, rare, legendary), and receives the pooled ETH minus a protocol fee.

• Collected fees accumulate in `totalFees`; the owner can `withdrawFees()` and can update the `feeAddress` via `changeFeeAddress()`.

• The accompanying Foundry tests exhaustively cover entry logic, refunds, winner selection, NFT URI accuracy, and fee withdrawal, while `DeployPuppyRaffle.sol` automates deployment with preset parameters.

The result is a transparent, trust-minimized raffle with provable NFT rewards and built-in revenue sharing.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
### Contract: PuppyRaffleTest

This test contract is designed to ensure the correct functionality of the `PuppyRaffle` smart contract. It utilizes Foundry's testing framework.

### Function: setUp()
```solidity
function setUp() public
```
This initializes the `PuppyRaffle` contract with specified parameters like entrance fee, a fee address, and raffle duration. It ensures the contract is set up before tests are executed.

### Function: testCanEnterRaffle()
```solidity
function testCanEnterRaffle() public
```
Tests if a player can enter the raffle upon sending the requisite entrance fee. It asserts playerOne’s participation in the raffle.

### Function: testCantEnterWithoutPaying()
```solidity
function testCantEnterWithoutPaying() public
```
Checks the condition where entering the raffle without paying results in a revert, ensuring only those sending the correct amount can enter.

### Function: testCanEnterRaffleMany()
```solidity
function testCanEnterRaffleMany() public
```
Validates that multiple players can enter the raffle concurrently if the correct aggregate fee is paid, asserting their entries.

### Function: testCantEnterWithoutPayingMultiple()
```solidity
function testCantEnterWithoutPayingMultiple() public
```
This confirms that attempting multiple entries with insufficient payment reverts, ensuring proper fee handling.

### Function: testCantEnterWithDuplicatePlayers()
```solidity
function testCantEnterWithDuplicatePlayers() public
```
Ensures the contract knows when duplicate addresses try to enter, leading to a revert to maintain fairness.

### Function: testCantEnterWithDuplicatePlayersMany()
```solidity
function testCantEnterWithDuplicatePlayersMany() public
```
Validates multiple duplicate entries don't bypass checks, enforcing unique participation.

### Modifier: playerEntered
This modifier simulates a state where a player has entered the raffle to facilitate testing refund functionalities.

### Function: testCanGetRefund()
```solidity
function testCanGetRefund() public playerEntered
```
Tests that a player can cancel their participation and receive their entrance fee back, modifying their balance accordingly.

### Function: testGettingRefundRemovesThemFromArray()
```solidity
function testGettingRefundRemovesThemFromArray() public playerEntered
```
Checks that after getting a refund, the player is removed from the active participant list.

### Function: testOnlyPlayerCanRefundThemself()
```solidity
function testOnlyPlayerCanRefundThemself() public playerEntered
```
Confirms that only the player themselves can request a refund, preventing others from executing on their behalf.

### Function: testGetActivePlayerIndexManyPlayers()
```solidity
function testGetActivePlayerIndexManyPlayers() public
```
Confirms the retrieval of player indices when multiple players have entered, validating correct indexing.

### Modifier: playersEntered
Simulates multiple players entering the raffle for comprehensive functionality testing such as winner selection.

### Function: testCantSelectWinnerBeforeRaffleEnds()
```solidity
function testCantSelectWinnerBeforeRaffleEnds() public playersEntered
```
Ensures the raffle cannot conclude prematurely which is essential to maintain the raffle's integrity.

### Function: testCantSelectWinnerWithFewerThanFourPlayers()
```solidity
function testCantSelectWinnerWithFewerThanFourPlayers() public
```
Tests that a minimum requisite of four participants is enforced before winner selection is allowed.

### Function: testSelectWinner()
```solidity
function testSelectWinner() public playersEntered
```
Simulates the selection of a winner appropriately after the raffle ends, verifying winner assignment.

### Function: testSelectWinnerGetsPaid()
```solidity
function testSelectWinnerGetsPaid() public playersEntered
```
Ensures the raffle winner receives the correct prize amount, guaranteeing financial accuracy.

### Function: testSelectWinnerGetsAPuppy()
```solidity
function testSelectWinnerGetsAPuppy() public playersEntered
```
Checks that the winner is awarded an NFT puppy, confirming NFT delivery to winners.

### Function: testPuppyUriIsRight()
```solidity
function testPuppyUriIsRight() public playersEntered
```
Verifies that the NFT minted has the expected properties by comparing token URIs.

### Function: testCantWithdrawFeesIfPlayersActive()
```solidity
function testCantWithdrawFeesIfPlayersActive() public playersEntered
```
Ensures no withdrawal of fees occurs while active players are present, protecting entered funds.

### Function: testWithdrawFees()
```solidity
function testWithdrawFees() public playersEntered
```
Confirms that fees can be withdrawn by the fee address once a winner is selected and the raffle concludes.

These tests provide comprehensive coverage of the core functionalities of `PuppyRaffle`, ensuring the contract behaves as expected under various scenarios.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
### Contract `DeployPuppyRaffle`
The `DeployPuppyRaffle` contract is a deployment script for the `PuppyRaffle` smart contract. It inherits from `Script` provided by the `forge-std` library and is responsible for setting up initial parameters and deploying the `PuppyRaffle` contract.

#### Storage Variables:
- `entranceFee (uint256)`: A fixed fee of 1 ETH, used as the entrance fee for the raffle.
- `feeAddress (address)`: An address that represents the fee receiver, initialized with the deployer's address.
- `duration (uint256)`: A fixed duration of 1 day for the raffle.

#### Function `run`:
```solidity
def run() public
```
The `run` function initializes `feeAddress` with the address of the deployer and broadcasts a transaction to deploy the `PuppyRaffle` contract. The constructor of `PuppyRaffle` is called with predefined parameters: entrance fee, fee address, and duration.

Overall, this script automates the deployment process by using predefined configurations for the `PuppyRaffle` contract, thus facilitating a smooth and repetitive deployment when necessary.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### PuppyRaffle Contract

The `PuppyRaffle` contract is an Ethereum-based raffle game using ERC721 tokens, allowing participants to win NFTs representing cute puppies with varying rarities (common, rare, legendary). 

### Key Features:
- **enterRaffle(address[] memory newPlayers)**: Users can join the raffle, paying the entrance fee for each participant without duplicate entries.
- **refund(uint256 playerIndex)**: Allows players to get a refund of their entrance fee, marking their spot as empty.
- **getActivePlayerIndex(address player) external view returns (uint256)**: Returns the index of a player in the raffle array.
- **selectWinner() external**: Chooses a winner and awards them an NFT, distributes fees, and resets the raffle.
- **withdrawFees() external**: Enables the owner to withdraw accumulated fees.
- **changeFeeAddress(address newFeeAddress) external onlyOwner**: Allows the contract owner to change the address to which fees are sent.
- **tokenURI(uint256 tokenId) public view virtual override returns (string memory)**: Generates the metadata URI for a token, encoding its rarity and associated image.

### Storage Variables:
- `uint256 public entranceFee`: Fee to enter the raffle.
- `address[] public players`: Active participants in the ongoing raffle.
- `uint256 public raffleDuration`: Duration of each raffle session in seconds.
- `address public feeAddress`: Address to which collected fees are sent.
- `uint64 public totalFees`: Total accumulated fees, withdrawn by the owner.

The contract utilizes Solidity's storage optimization techniques and uses OpenZeppelin's Solidity libraries for best practices.


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


