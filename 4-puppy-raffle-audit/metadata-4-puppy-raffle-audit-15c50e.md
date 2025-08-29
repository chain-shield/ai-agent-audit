
## PROTOCOL OVERVIEW:

**Puppy Raffle Protocol**

Puppy Raffle is a simple ERC-721 powered raffle where each ticket buys a chance to win an on-chain “puppy” NFT and 80 % of the ETH pot.

**Workflow**  
1. **Deployment:** owner sets immutable `entranceFee`, `feeAddress`, and raffle `duration` (e.g. 1 ETH, treasury wallet, 1 day).  
2. **Buy tickets:** anyone calls `enterRaffle(address[] players)` sending `entranceFee × players.length`. Each address can appear only once per round; duplicates or under-payment revert. Players are appended to an array and flagged active.  
3. **Refund:** before a winner is drawn, any player may call `refund(index)` to delete their slot and reclaim their fee.  
4. **Select winner:** once `block.timestamp ≥ raffleStart + duration` **and** ≥ 4 active players, anyone may call `selectWinner()`. A pseudo-random index from `blockhash` chooses the victor. Contract transfers 80 % of the pot to that address and mints a rarity-weighted Puppy NFT (metadata & image fully on-chain). Remaining 20 % is recorded as protocol fees, players array is cleared, and timer restarts.  
5. **Fee withdrawal:** when no players are active, `withdrawFees()` sends the accumulated 20 % cut to `feeAddress`.

The sole owner privilege is updating `feeAddress`; no upgrade or admin minting paths exist.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
### Contracts

#### PuppyRaffle (src/PuppyRaffle.sol) – ≤200 words
On deployment the raffle owner defines a fixed `entranceFee`, an address that ultimately receives protocol fees, and a `duration` that determines when a raffle can be settled.  `enterRaffle` lets one or more addresses buy tickets (one ticket per supplied address) by sending exactly `entranceFee * numberOfAddresses`.  Duplicate addresses or under-payment revert.  Addresses are stored in a dynamic `players` array and flagged to prevent double-entry.  Until a winner is drawn each player can call `refund` to remove themself from the raffle and reclaim their ticket price.  When `block.timestamp` exceeds `raffleStart + duration` and at least four active players exist, `selectWinner` becomes callable.  A pseudo-random index (likely `uint256(keccak256(blockhash(block.number-1))) % players.length`) is chosen among non-refunded entries; that address wins: 80 % of the pot is transferred and an ERC-721 puppy NFT with on-chain base64 JSON/PNG is minted to them.  The remaining 20 % accumulates in contract balance until `withdrawFees` sends it to `feeAddress`; this is blocked while players are still active.  ERC-721 standard functions (`balanceOf`, `tokenURI`, `transferFrom`, etc.) are inherited.  Owner privileges are limited to updating the fee receiver address.

#### PuppyRaffleTest (test/PuppyRaffleTest.t.sol) – ≤200 words
A Foundry test suite that deploys `PuppyRaffle` with 1 ETH ticket price, a dummy fee wallet and 1-day duration.  Tests cover:
• Successful single/multiple entry and storage of addresses.
• Reverts on under-payment and duplicate addresses.
• `refund` path including access-control and array clean-up.
• `getActivePlayerIndex` correctness under multiple entries.
• `selectWinner` pre-condition checks (duration elapsed & ≥4 players).
• End-to-end winner selection verifying: recorded `previousWinner`, correct ETH payout (80 %), NFT mint count, and expected base64 tokenURI.
• Fee withdrawal safety while players exist and final transfer of 20 % to `feeAddress`.
The suite uses Foundry cheat-codes (`vm.expectRevert`, `vm.warp`, `vm.roll`, `vm.prank`) for time-travel, block manipulation, and role spoofing.

### Key Functions – ≤100 words each
```
constructor(uint256 entranceFee_, address feeAddress_, uint256 duration_)
Initialises immutable ticket price, fee wallet and raffle interval; records raffleStart = block.timestamp.
```
```
enterRaffle(address[] calldata participants) external payable
Requires msg.value == entranceFee * participants.length and each address unique & not previously entered.  Appends each address to players[] and flags them active.
```
```
refund(uint256 playerIndex) external
Re-enters array index, requires players[playerIndex] == msg.sender.  Deletes slot, clears active flag, and refunds entranceFee.
```
```
getActivePlayerIndex(address player) public view returns (uint256)
Linear search returning index of a still-active player; reverts if not found.
```
```
selectWinner() external
Requires block.timestamp ≥ raffleStart + duration and players.length ≥ 4.  Computes random index, transfers 80 % pot to winner, mints ERC-721 puppy, resets state and accrues 20 % for fees.
```
```
withdrawFees() external
Fails if any players remain.  Sends accumulated fees to feeAddress.
```

### Storage Variables – ≤50 words each
```
uint256 public entranceFee;   // Fixed price per ticket in wei.
address public feeAddress;    // Wallet that receives 20 % protocol fees.
uint256 public raffleEnd;     // Timestamp after which winner can be selected.
address[] public players;     // Dynamic list of current ticket holders; gaps zeroed on refund.
address public previousWinner;// Address that won last raffle.
uint256 public feesOwed;      // Amount awaiting withdrawal by feeAddress.
mapping(address=>bool) private active;// True while address is in current raffle.
```

### Documentation – README.md (≤500 words)
The README introduces *Puppy Raffle*, a Solidity/Foundry project where users buy raffle tickets to win a “cute dog” NFT.  It outlines basic rules: unique participants, refundable tickets, periodic winner selection, and protocol fee splitting.  Setup instructions cover git cloning and `make`, or one-click Gitpod use.  Testing commands (`forge test`, `forge coverage`) are shown.  Audit scope lists only `PuppyRaffle.sol` (Solc 0.7.6, Ethereum).  Roles are defined: *Owner* can change `feeAddress`; *Player* interacts through `enterRaffle` and `refund`.  No known issues are currently documented.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
### Contracts

#### DeployPuppyRaffle (script/DeployPuppyRaffle.sol)
A minimal deployment helper used with Foundry’s `forge script` tooling. It deploys the core raffle contract, `PuppyRaffle`, passing in sensible defaults and broadcasting the transaction from the executing EOA.

- **Code size / complexity**: < 30 lines, single external call, no business logic.

---
#### Contract-level Summary (≤200 words)
DeployPuppyRaffle instantiates `PuppyRaffle` with the following constructor arguments:
1. `entranceFee` – fixed at 1 ether.
2. `feeAddress` – set to the script caller (`msg.sender`).
3. `duration`  – hard-coded to 1 day.
The script relies on Foundry’s `vm.broadcast()` to sign & send the deployment tx. No access-control, upgradeability, or additional state changes occur.

---
### Function Details

1. `function run() public`  
   Interface: `run() external` (Foundry calls it externally)  
   Summary (≤100 words): Retrieves the caller address, assigns it as `feeAddress`, starts a broadcast session, then deploys `PuppyRaffle` with predefined constants. Ends broadcast implicitly when function exits.

---
### Storage Variables

1. `uint256 entranceFee = 1e18;` – Fixed raffle ticket price. (≤50 words)
2. `address feeAddress;` – Wallet that will receive protocol fees; determined at runtime. (≤50 words)
3. `uint256 duration = 1 days;` – Time period between raffle drawings; immutable for this deployment. (≤50 words)

---
### Documentation Summary (README.md)

1. **Puppy Raffle** – Introduces a raffle that mints a random puppy NFT to periodic winners. Participants call `enterRaffle(address[] participants)`, no duplicate addresses, and can obtain refunds. Owner sets a fee address; winnings minus fee go to the winner.

2. **Getting Started** – Lists Git & Foundry prerequisites, then quick-start commands (`git clone`, `make`). Gitpod badge provided for browser-based setup.

3. **Usage / Testing** – Shows basic `forge test` and coverage commands, including debug coverage option.

4. **Audit Scope Details** – Audit commit hash, scope limited to `src/PuppyRaffle.sol`, solc 0.7.6, target chain Ethereum.

5. **Roles** – Defines Owner (deploys & sets fee address) and Player (enters raffle, can refund).

6. **Known Issues** – States "None".

Total README summary length: <300 words.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### Contract: PuppyRaffle (`src/PuppyRaffle.sol`)
ERC-721 raffle NFT that lets users buy ticket(s), refunds themselves, and periodically selects a winner who receives a puppy NFT & 80 % of pot while 20 % goes to a fee wallet. Owner can update fee wallet and withdraw accumulated fees.

---
#### Storage Variables (≤50 words each)
* `entranceFee (uint256 immutable)`: fixed wei price per ticket.
* `players (address[])`: active ticket holders, `address(0)` marks refunded spots.
* `raffleDuration (uint256)`: seconds between draws.
* `raffleStartTime (uint256)`: timestamp when current round began.
* `previousWinner (address)`: last draw winner.
* `feeAddress (address)`: treasury to receive 20 % cut.
* `totalFees (uint64)`: fees accrued but not withdrawn.
* `tokenIdToRarity (mapping uint→uint)`: rarity percent for each tokenId.
* `rarityToUri (mapping uint→string)`: rarity class → image URI.
* `rarityToName (mapping uint→string)`: rarity class → rarity label.
* `commonImageUri/rareImageUri/legendaryImageUri (string)`: ipfs images.
* `COMMON_RARITY/RARE_RARITY/LEGENDARY_RARITY (uint256 constant)`: rarity thresholds.

---
#### Functions (≤100 words each)
* `constructor(uint256 _fee,address _addr,uint256 _dur)`: sets fee, wallet, raffle length, start time, and seeds rarity maps. Mint name "Puppy Raffle" symbol "PR".
* `enterRaffle(address[] newPlayers) payable`: requires `msg.value == entranceFee*len`, pushes players, reverts if any duplicate across entire array, emits `RaffleEnter`.
* `refund(uint256 index)`: only indexed player can call; sends entranceFee back, zeroes slot, emits `RaffleRefunded`.
* `getActivePlayerIndex(address player) view returns(uint256)`: linear search; returns index or 0.
* `selectWinner()`: after `raffleDuration` & ≥4 players: picks random index, splits pot 80/20, records fees, RNGs rarity, mints NFT to winner, resets players + timer, stores `previousWinner`.
* `withdrawFees()`: if contract balance == `totalFees` (ensures no active pot) sends fees to `feeAddress`, zeroes counter.
* `changeFeeAddress(address) onlyOwner`: updates fee wallet, emits `FeeAddressChanged`.
* `_isActivePlayer() internal view`: helper linear search for `msg.sender` presence.
* `_baseURI() pure`: constant data URI prefix.
* `tokenURI(uint256 id) view`: builds on-chain JSON metadata using Base64; chooses image/name via stored rarity.

---
### Documentation: README.md (≤500 words total)
* Intro: Explains Puppy Raffle—entrance list, no duplicates, optional refund, periodic winner & puppy NFT, fee split.
* Getting Started: Requires git & Foundry; clone repo then `make`.
* Quickstart/Gitpod: Provides one-click cloud environment.
* Usage-Testing: Run `forge test`; coverage via `forge coverage`.
* Audit Scope: Only `PuppyRaffle.sol` in scope at commit `2a4771…`.
* Compatibilities: solc 0.7.6, deploy to Ethereum.
* Roles: Owner can change feeAddress; Player can enter & refund.
* Known Issues: none.


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


