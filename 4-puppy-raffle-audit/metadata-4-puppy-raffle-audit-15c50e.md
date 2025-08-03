
## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
### Test Suite for Puppy Raffle Contract

This Solidity test suite is written for a smart contract `PuppyRaffle` using the Forge testing framework. The primary focus of the tests is to ensure the correct functionality of the `PuppyRaffle` contract. The tests are organized to cover different functionalities of the contract such as entering the raffle, refunds, selecting a winner, and fee withdrawals.

#### Key Tests:

- **Entering the Raffle:** Tests ensure that players can enter the raffle by paying the entrance fee. Validations are in place to check that entries without fees or with duplicate players are rejected.
- **Refunding Players:** Tests check if players can withdraw their funds correctly and ensure only the player who entered can request a refund.
- **Selecting Winner:** Tests verify the conditions under which a winner can be selected, such as the raffle's timing and minimum player count. It checks that the winner receives the prize and the balance updates correctly.
- **Withdrawing Fees:** Tests confirm that fees can only be withdrawn when no players are active, ensuring the fee balance is transferred correctly.

Through modifiers like `playerEntered` and `playersEntered` and mock commands like `vm.prank` and `vm.expectRevert`, the tests simulate various real-world scenarios and edge cases of the raffle operation.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
### DeployPuppyRaffle Contract
This contract is a deployment script for the `PuppyRaffle` smart contract using Solidity 0.7.6. It utilizes the Foundry framework's `Script` utility to deploy the `PuppyRaffle` contract on the blockchain.

#### Contract Definition
```solidity
contract DeployPuppyRaffle is Script { ... }
```

### run Function
This function deploys the `PuppyRaffle` contract. The caller's address is set as the `feeAddress`, and the `vm.broadcast()` function, a Foundry utility, is used to initiate the on-chain transaction for deployment.

**Function Interface:**
```solidity
function run() public
```

### Storage Variables
- `uint256 entranceFee`: Hardcoded entrance fee set to 1 ether.
  
- `address feeAddress`: The address that will receive the entrance fees, initialized as the caller of the `run` function.
  
- `uint256 duration`: Hardcoded raffle duration set to 1 day.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### PuppyRaffle Contract
**Contract Definition:** A smart contract for a raffle system where participants can enter to win an NFT of a puppy, with players paying an entrance fee and receiving a puppy with a rarity determined at random.

#### Storage Variables:
- **entranceFee** (`uint256`): The cost to participate in the raffle in wei. Immutable after initialization.
- **players** (`address[]`): Dynamic array storing the addresses of current participants in the raffle.
- **raffleDuration** (`uint256`): Duration of the raffle, in seconds. Set during contract deployment.
- **raffleStartTime** (`uint256`): Records the start time of the current raffle. Updated each winner selection.
- **previousWinner** (`address`): Stores the address of the last raffle's winner.
- **feeAddress** (`address`): Address where the fees from the raffle are sent.
- **totalFees** (`uint64`): Tracks the total fees accumulated for withdrawal.
- **tokenIdToRarity** (`mapping(uint256 => uint256)`): Links NFT token IDs to their rarity.
- **rarityToUri, rarityToName** (`mapping`): Define URIs and human-readable names associated with each rarity tier.

#### Key Functions:
- **constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration)**: Initializes the contract with entrance fee, fee address, and raffle duration.
- **enterRaffle(address[] memory newPlayers)**: Allows participants to join the raffle while ensuring no duplicates. Validates the necessary payment for the number of entries.
- **refund(uint256 playerIndex)**: Enables players to claim a refund, setting their slot in the participants' list to zero, while returning their entrance fee.
- **getActivePlayerIndex(address player)**: Returns a player's index or zero if inactive.
- **selectWinner()**: Determines the raffle winner post duration, distributing 80% of collected funds to them and 20% to the fee address. Mints a new puppy NFT based on a rarity calculation.
- **withdrawFees()**: Allows fee withdrawal to the designated address if no active players are present.
- **changeFeeAddress(address newFeeAddress)**: Allows the contract owner to update the fee address.
- **_isActivePlayer() internal view**: Checks if the sender is an active player in the current raffle.
- **tokenURI(uint256 tokenId)**: Provides the URI containing metadata of an NFT based on its rarity. Returns JSON format with image and rarity attributes.


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


