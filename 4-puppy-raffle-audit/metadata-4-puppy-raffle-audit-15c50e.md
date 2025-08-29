
## PROTOCOL OVERVIEW:

**Puppy Raffle Protocol**

Puppy Raffle is an on-chain game that lets players buy tickets for a chance to win an ERC-721 “cute dog” NFT and 80 % of the ETH pot.

How it works:
1. **Enter** – Anyone calls `enterRaffle(address[] participants)` sending exactly `entranceFee × numberOfAddresses`.  Each address may appear only once per round; duplicates revert.
2. **Refund** – A player can reclaim their stake before the draw by calling `refund(index)`, which zeroes their slot in the players array and returns the fee.
3. **Draw** – After `raffleDuration` seconds (default 1 day) and with ≥4 active players, anyone may call `selectWinner()`.  A pseudo-random pick chooses the winner, a second roll assigns NFT rarity (common 70 %, rare 25 %, legendary 5 %).  The contract:
   • Mints the puppy NFT to the winner with on-chain, Base64-encoded metadata.
   • Pays out 80 % of the contract balance to the winner.
   • Records the remaining 20 % as protocol fees.
4. **Fees** – When no players are active, anyone can call `withdrawFees()` to send accrued fees to `feeAddress`.  The owner may update this address via `changeFeeAddress()`.

The contract is immutable except for the fee recipient, uses Solidity 0.7.6, and is fully covered by Foundry tests.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
### README.md (Documentation)
The README introduces **Puppy Raffle**, a Solidity/Foundry project for running timed raffles that mint a “cute dog” NFT to the winner.  Users enter by calling `enterRaffle(address[] participants)` and paying an `entranceFee`.  Duplicate addresses are rejected.  Players can reclaim their ticket via `refund(uint256 index)`.  After a fixed `duration`, anyone can call `selectWinner()` which (1) picks a random player, (2) mints the puppy NFT to that winner, (3) sends 80 % of the pooled ETH to the winner and 20 % to the protocol fee address.  The contract owner can update the fee address through `changeFeeAddress(address)`.  Test, deployment, and coverage commands are provided; scope uses Solidity 0.7.6 on Ethereum.  Roles: Owner (controls fee address) and Player (enters raffles / refunds).  No known issues.

### PuppyRaffleTest.t.sol (Tests)
The Foundry test suite instantiates `PuppyRaffle` with a 1 ETH entrance fee, a fee receiver, and a 1-day raffle duration.

Core test groups:
1. **enterRaffle** – verifies single/multiple entry success, value requirements, and duplicate-player reverts.
2. **refund** – checks that a player can refund themselves, array slot is cleared, and other callers are blocked.
3. **getActivePlayerIndex** – ensures correct indexing after multiple entries.
4. **selectWinner** – enforces raffle-end time & minimum player count, confirms selected winner, ETH payout (80 %), NFT mint, and correct tokenURI.
5. **withdrawFees** – reverts when active players remain and otherwise transfers 20 % of the pot to `feeAddress`.

Utilities: `playerEntered` and `playersEntered` modifiers set up common states.  Test helpers advance `warp`/`roll` for time & block changes.

No contract source file was included in the prompt, so contract, function, and storage summaries are omitted.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
### File: script/DeployPuppyRaffle.sol

**Contract Definition**
`contract DeployPuppyRaffle is Script` – a Foundry deployment helper that instantiates the main `PuppyRaffle` contract on-chain.

**Contract Summary (≤200 words)**
DeployPuppyRaffle is a lightweight script used only during development or automated deployment. It inherits Foundry’s `Script` utilities to broadcast transactions. The script collects three deployment parameters—raffle ticket price (`entranceFee`), the treasury address that receives protocol fees (`feeAddress`), and game round duration (`duration`). When `run()` is executed with `forge script`, the caller’s address is stored as the fee recipient and a new `PuppyRaffle` instance is deployed with those constants, all inside a single `vm.broadcast()` to push the transaction on-chain. The script contains no game logic, access control, or state mutation beyond deployment; its sole purpose is repeatable, parameterised contract creation.

**Storage Variables (≤50 words each)**
1. `uint256 entranceFee = 1e18;` – default cost (1 ETH) for a raffle ticket.
2. `address feeAddress;` – wallet entitled to collect protocol fees; set to deployer at runtime.
3. `uint256 duration = 1 days;` – length of each raffle round.

**Function Interfaces & Summaries (≤100 words each)**
• `function run() public` – Picks the caller as `feeAddress`, starts a broadcast session, and deploys a `PuppyRaffle` with `(entranceFee, feeAddress, duration)`. It emits one on-chain transaction and returns nothing. No access restrictions.

---
### File: README.md

**Puppy Raffle (Title Section)** – Introduces a raffle protocol where users buy entries to win a random puppy NFT. Ensures no duplicate entrants, allows ticket refunds, periodically selects a winner, and splits collected ETH between a fee address and the winner.

**Getting Started / Requirements** – Lists prerequisite tooling (git, Foundry) with version check commands.

**Quickstart** – Provides clone, navigate, and `make` commands to set up the repo locally. Includes an optional Gitpod one-click link for cloud development.

**Usage / Testing** – Shows how to run unit tests and generate coverage reports using `forge test` and `forge coverage` commands.

**Audit Scope Details** – Specifies commit hash, in-scope file (`src/PuppyRaffle.sol`), solidity version 0.7.6, and target chain (Ethereum) for audit clarity.

**Compatibilities** – Restates compiler and chain compatibility.

**Roles** – Defines Owner (can change fee address) and Player (can enter raffle and request refund).

**Known Issues** – Currently none reported.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
# PuppyRaffle.sol

```solidity
contract PuppyRaffle is ERC721, Ownable
```

**Contract Overview (≈150 words)**  
PuppyRaffle is an ERC-721 raffle system where players pay a fixed `entranceFee` to join a draw for a randomly generated puppy NFT. Duplicate entrants are forbidden and any participant can reclaim their ticket via `refund` before a draw. After `raffleDuration` seconds and with at least four active players, anyone can call `selectWinner`, which pseudo-randomly picks a winner, splits the pot 80/20 between the winner and the protocol, and mints a puppy whose rarity (common, rare, legendary) is decided by another RNG pass. Fees accumulate in `totalFees` and may be withdrawn to `feeAddress` once no players are active. The owner can update `feeAddress`. NFT metadata is served on-chain using a Base64 encoded JSON data‐URI.

---
## Functions

* **constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration)** – sets immutable fee, initial fee receiver, raffle length, rarity URIs/names.  
* **enterRaffle(address[] newPlayers) external payable** – requires `msg.value == entranceFee*count`; appends players; reverts on duplicates; emits `RaffleEnter`.  
* **refund(uint256 playerIndex) external** – sender must match indexed player; sends back `entranceFee`; marks slot empty; emits `RaffleRefunded`.  
* **getActivePlayerIndex(address player) external view returns (uint256)** – linear search returning index or 0.  
* **selectWinner() external** – enforces time & min players; draws winner & rarity; distributes 80% pot, tracks 20% fees; resets state; mints NFT; records `previousWinner`.  
* **withdrawFees() external** – only callable when no active players (contract balance == totalFees); sends fees to `feeAddress`; zeroes counter.  
* **changeFeeAddress(address newFeeAddress) external onlyOwner** – updates fee receiver; emits `FeeAddressChanged`.  
* **_isActivePlayer() internal view** – helper loop to check membership.  
* **_baseURI() internal pure returns (string)** – returns data-URI prefix.  
* **tokenURI(uint256 id) public view override returns (string)** – constructs base64 JSON with name, rarity attribute and image link derived from stored rarity.

---
## Storage Variables

* `uint256 immutable entranceFee` – fixed ETH cost per ticket.  
* `address[] public players` – active entrants; refunded slots set to 0.  
* `uint256 raffleDuration` – seconds a round lasts.  
* `uint256 raffleStartTime` – timestamp when current round began.  
* `address previousWinner` – last draw’s champion.  
* `address feeAddress` – receiver of protocol fees.  
* `uint64 totalFees` – accrued unpaid fees.  
* `mapping(uint256⇒uint256) tokenIdToRarity` – NFT id → rarity tier.  
* `mapping(uint256⇒string) rarityToUri` – rarity tier → image URI.  
* `mapping(uint256⇒string) rarityToName` – rarity tier → textual name.  
* `string commonImageUri` – IPFS hash for common puppy.  
* `uint256 constant COMMON_RARITY=70` – ≤70% chance.  
* `string COMMON="common"` – label.  
* `string rareImageUri` – IPFS for rare.  
* `uint256 constant RARE_RARITY=25` – 25% slice.  
* `string RARE="rare"` – label.  
* `string legendaryImageUri` – IPFS for legendary.  
* `uint256 constant LEGENDARY_RARITY=5` – 5% chance.  
* `string LEGENDARY="legendary"` – label.

---
# README.md Summary (≈350 words)

**Puppy Raffle** – Repository hosts a solidity raffle enabling users to win a dog NFT; highlights core features (no duplicates, refundable tickets, periodic draws, fee split).

**Getting Started** – Requires Git & Foundry; clone repo then run `make` to install dependencies/build. Gitpod button offers cloud IDE alternative.

**Usage / Testing** – Run `forge test` for unit tests; `forge coverage` (optionally with `--report debug`) measures coverage.

**Audit Scope Details & Compatibilities** – Audit hash 2a47715…; only `src/PuppyRaffle.sol` in-scope; targets Solidity 0.7.6 on Ethereum.

**Roles** – Owner (deploys, can update fee address) vs Player (enters raffle, can refund).

**Known Issues** – None reported.



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


