
## PROTOCOL OVERVIEW:

**Puppy Raffle Protocol**

Puppy Raffle is an on-chain raffle that lets anyone compete for a unique ERC-721 “puppy” NFT by paying a fixed entrance fee (1 ETH in the reference deployment).

Workflow
1. Entry: Players call `enterRaffle(address[] newPlayers)` and supply the exact fee per address. The contract rejects under-payment, duplicate addresses, or re-entry of existing players. Each address is stored in `players`.
2. Voluntary refund: Before the draw, a participant may reclaim their stake via `refund(playerIndex)`, freeing their slot.
3. Draw window: After `raffleDuration` (default 1 day) the owner or anyone can trigger `selectWinner()`. Preconditions: at least 4 active players and the raffle period has ended.
4. Winner selection & payout: A pseudo-random index chooses the winner, the contract mints them a new puppy NFT, and transfers the prize pool minus protocol fees. Fees are forwarded to `feeAddress` which the owner can update with `changeFeeAddress()`.
5. Reset: Player array is cleared for the next round.
6. Fee withdrawal: If no players are registered, the owner can pull any accumulated fees via `withdrawFees()`.

The accompanying Foundry tests and deployment script verify correct entry logic, refund behavior, winner selection, and administrative controls.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
### Contract: PuppyRaffleTest

The `PuppyRaffleTest` contract is a unit test suite for the `PuppyRaffle` smart contract. It simulates various scenarios to ensure that the `PuppyRaffle` contract behaves as expected. Key functionalities tested include entering the raffle, processing refunds, managing player indices, selecting a winner, and handling fee withdrawals.

#### Key Variables:
- **`puppyRaffle` (`PuppyRaffle`)**: An instance of the `PuppyRaffle` contract.
- **`entranceFee` (`uint256`)**: Fee to enter the raffle, set at 1 ether.
- **`duration` (`uint256`)**: Duration of the raffle period, set to 1 day.
- **Player Addresses** (`address`): Four distinct players and a fee address for interaction.

#### Key Functions:

- **`setUp`**: Initializes the `PuppyRaffle` instance with predefined parameters.

- **`testCanEnterRaffle`**: Tests single player entry by asserting the player list includes the new entrant when the correct fee is paid.

- **`testCantEnterWithoutPaying`**: Ensures players cannot enter without sending the entrance fee.

- **`testCantEnterWithDuplicatePlayers`**: Validates that duplicate player entries are prohibited.

- **`testCantSelectWinnerBeforeRaffleEnds`**: Verifies restrictions on winner selection before the raffle period ends.

- **`testSelectWinner`**: Confirms the correct winner is selected and compensated after the raffle duration ends.

The test suite is structured to verify all critical aspects of the `PuppyRaffle` contract, ensuring secure and intended operation.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
# DeployPuppyRaffle.sol
## Contract Definition
The `DeployPuppyRaffle` contract is responsible for deploying the `PuppyRaffle` smart contract with specified parameters: entrance fee, fee address, and duration.

## Variables
- `uint256 entranceFee`: (1e18) Specifies the cost of entering the raffle, set to 1 Ether.
- `address feeAddress`: Address where the entrance fee is sent.
- `uint256 duration`: (1 days) Duration for which the raffle will be active.

## Function
### run()
- **Definition**: `function run() public`
- **Summary**: The `run` function initializes the `feeAddress` to the deployer's address and deploys the `PuppyRaffle` contract using the specified parameters. It utilizes `vm.broadcast()` to emit the transaction, supporting deployment processes in testing environments.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
```solidity
contract PuppyRaffle is ERC721, Ownable {
    ...
}
```

**Contract Overview:**
PuppyRaffle is an ERC721 compliant contract designed by PuppyLoveDAO that allows participants to enter a raffle for the chance to win a unique puppy NFT. Participants pay an entrance fee, and a winner is determined at regular intervals. 

**Functions:**
- `enterRaffle(address[] memory newPlayers) public payable`
    - **Description:** Allows players to enter the raffle by paying an entrance fee.
    - **Details:** Requires the correct fee, checks for duplicate entries, and emits `RaffleEnter` event.
- `refund(uint256 playerIndex) public`
    - **Description:** Refunding entry fees for players.
    - **Details:** Only callable by the player, marks the spot as blank.
- `selectWinner() external`
    - **Description:** Selects a winner and mints a new puppy NFT.
    - **Details:** Requires at least four players and that raffle is over, distributes rewards and resets the state.
- `withdrawFees() external`
    - **Description:** Withdraws collected fees.
    - **Details:** Only callable when no players are active.
- `changeFeeAddress(address newFeeAddress) external onlyOwner`
    - **Description:** Changes the address to which fees are sent.

**Storage Variables:**
- `uint256 public immutable entranceFee`
    - **Role:** Sets cost to enter the raffle.
- `address[] public players`
    - **Role:** Array storing current raffle entrants.
- `uint256 public raffleDuration`
    - **Role:** Sets duration between raffle draws.
- `address public feeAddress`
    - **Role:** Address used for collecting fees.


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


