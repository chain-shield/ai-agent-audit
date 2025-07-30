
## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### Contract: PuppyRaffle

The `PuppyRaffle` smart contract, authored by PuppyLoveDAO, facilitates a raffle system where participants can win a cute dog NFT. It allows multiple addresses to enter the raffle, prohibits duplicate entries, and provides a refund mechanism. The system charges a fee, with a portion going to an owner-designated address and the rest to the raffle winner.

#### enterRaffle function

```solidity
function enterRaffle(address[] memory newPlayers) public payable
```
Allows participants to enter the raffle by paying the entrance fee for each participant. It checks for sufficient payment and scrutinizes the list of players to prevent duplicates. Emits a `RaffleEnter` event upon successful entry.

#### refund function

```solidity
function refund(uint256 playerIndex) public
```
Enables players to get a refund of their entrance fee based on their player index. It ensures only the player requesting the refund gets reimbursed and emits `RaffleRefunded` event.

#### getActivePlayerIndex function

```solidity
function getActivePlayerIndex(address player) external view returns (uint256)
```
Returns the index of a player in the players array, or 0 if inactive. Useful for finding a player's position.

#### selectWinner function

```solidity
function selectWinner() external
```
Randomly selects a winner and mints a puppy NFT, transferring 80% of the prize pool to the winner and 20% to the fee address. Requires at least 4 players and the raffle duration must have elapsed. Updates the raffle state.

#### withdrawFees function

```solidity
function withdrawFees() external
```
Allows the contract owner to withdraw accumulated fees, ensuring no active players are present when withdrawing. Sends all collected fees to the feeAddress.

#### changeFeeAddress function

```solidity
function changeFeeAddress(address newFeeAddress) external onlyOwner
```
Permits the contract owner to change the address where fees are sent. Emits a `FeeAddressChanged` event on execution.

#### _isActivePlayer function

```solidity
function _isActivePlayer() internal view returns (bool)
```
Checks and returns whether the caller is an active player in the current raffle.

#### _baseURI function

```solidity
function _baseURI() internal pure returns (string memory)
```
Returns the base URI for NFT metadata, encoded as JSON.

#### tokenURI function

```solidity
function tokenURI(uint256 tokenId) public view virtual override returns (string memory)
```
Generates the token URI for a given tokenId, incorporating rarity and image attributes into JSON metadata.

### Storage Variables

- **entranceFee**: Cost for entering the raffle, set at deployment.
- **players**: Array of participant addresses in the raffle.
- **raffleDuration & raffleStartTime**: Timer mechanics for the raffle's lifecycle; sets duration and records start time.
- **previousWinner**: Address of the last raffle winner.
- **feeAddress**: The address designated to receive the fee cut of the raffle. Initially set at contract deployment.
- **totalFees**: Aggregated value of fees accrued from raffle entries, initialized to zero. 
- **tokenIdToRarity, rarityToUri, rarityToName**: Mappings to associate NFTs with rarity levels and corresponding metadata strings.


## Main List of Files in Project

script/DeployPuppyRaffle.sol
src/PuppyRaffle.sol
test/PuppyRaffleTest.t.sol

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


