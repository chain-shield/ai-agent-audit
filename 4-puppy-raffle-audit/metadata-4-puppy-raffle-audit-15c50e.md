
## PROTOCOL OVERVIEW:

**Puppy Raffle Protocol (Solidity 0.7.6)**  
A time-boxed raffle where players purchase entries to win an on-chain Puppy NFT and 80 % of the ether pot. Each ticket costs `entranceFee` (default 1 ETH).  

1. **Entering**  
   • Anyone calls `enterRaffle(address[] participants)` sending `entranceFee × n`.  
   • All supplied addresses are recorded as active players; duplicates revert.  

2. **Refunds**  
   • Before the draw, a player may call `refund(index)` to reclaim their full fee; their slot in `players` is zero-ed.  

3. **Drawing a Winner**  
   • After `raffleDuration` (default 24 h) and with ≥ 4 active players, anyone can call `selectWinner()`.  
   • Pseudo-randomly chooses a winner, determines NFT rarity (common/rare/legendary), mints an ERC-721 Puppy with on-chain metadata, and sends:  
     – 80 % of contract balance to the winner.  
     – 20 % accumulated into `totalFees`.  
   • Resets player list and start time for the next round.  

4. **Fee Handling**  
   • Owner can update `feeAddress` via `changeFeeAddress`.  
   • When no active players remain (`balance == totalFees`) the owner withdraws fees through `withdrawFees()`.  

5. **Deployment Script**  
   `DeployPuppyRaffle.sol` broadcasts a deployment using the caller as `feeAddress`.  

Comprehensive tests verify entry logic, refunds, winner selection, NFT metadata, payout splits, and fee withdrawal, ensuring protocol integrity.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
### README.md (documentation)
- Purpose: Describe Puppy Raffle protocol where users buy entries to win a dog-NFT, with refunds possible and periodic winner selection. Outlines requirements (git, Foundry), quickstart commands, testing/coverage, audit scope (only `src/PuppyRaffle.sol`, solc 0.7.6), roles (Owner, Player), and states there are no known issues.  
- Key points: Owner can change `feeAddress`. Players call `enterRaffle` with unique address list & value, may later `refund`. Every X seconds a winner is chosen; 20% of pot sent to `feeAddress`, 80% + NFT to winner.

### PuppyRaffleTest.t.sol (tests – treated as contract for summary)
Definition: Solidity 0.7.6 Forge test contract validating `PuppyRaffle` behaviour. Deploys raffle with `entranceFee`, `feeAddress`, `duration`, then runs unit tests.
Storage vars: 
• `puppyRaffle` – instance under test.
• `entranceFee`, `playerOne…playerFour`, `feeAddress`, `duration` – static params for scenarios. (≤50 words each collectively: constants used to craft test cases.)

Key functions (≤100 words each):
• `setUp()` – deploys new `PuppyRaffle` before each test.
• `testCanEnterRaffle/Many` – ensures single & multi-entry succeed and players array records addresses.
• `testCantEnter…` variants – verify reverts on insufficient payment or duplicate entrants.
• `playerEntered` modifier – reusable setup adding one entrant.
• `testCanGetRefund` & related – confirm correct refund logic, array cleanup, and auth checks.
• `testGetActivePlayerIndexManyPlayers` – checks index lookup utility.
• `playersEntered` modifier – adds 4 entrants for winner tests.
• `testCantSelectWinner…` – ensure timing & min-player guards.
• `testSelectWinner*` – validate winner chosen, payout (80%), NFT mint & URI.
• `testCantWithdrawFeesIfPlayersActive`/`testWithdrawFees` – assure fee withdrawal only after game ends, paying 20% to fee address.

Overall, the test suite exhaustively covers entry validation, refund mechanics, winner selection, NFT metadata, payout splits, and fee withdrawal, ensuring contract adheres to protocol rules.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
### README.md (Documentation)
Puppy Raffle is a Solidity/Forged‐based raffle that lets users buy entries to win a random puppy NFT. Key rules:
1. Players call `enterRaffle(address[] participants)` supplying one or many participant addresses – duplicates forbidden.
2. Tickets can be fully refunded via `refund()` returning the original payment.
3. After a fixed time interval the contract draws a winner, mints the puppy NFT, sends prize money to the winner, and routes a configurable fee cut to an owner-controlled `feeAddress`.
4. Owner may update `feeAddress` using `changeFeeAddress`.

The repo supplies Foundry scripts/tests and targets Solidity 0.7.6.  Running `forge test` executes the suite; `forge coverage` reports coverage.  Audit scope only includes `src/PuppyRaffle.sol`.

---

## DeployPuppyRaffle.sol (Contract)
```
contract DeployPuppyRaffle is Script {
    uint256 entranceFee = 1e18;  // 1 ETH ticket price
    address feeAddress;          // address collecting protocol fees
    uint256 duration = 1 days;   // raffle interval

    function run() public {
        feeAddress = msg.sender;             // deployer is fee recipient
        vm.broadcast();                      // start a broadcasted tx
        new PuppyRaffle(entranceFee, feeAddress, duration);
    }
}
```

Contract summary (≤200 words):
DeployPuppyRaffle is a Foundry script that permissionlessly deploys the core `PuppyRaffle` contract.  It presets an entrance fee of 1 ETH and a raffle duration of 24 hours.  During `run()` execution it sets the fee collector to the script invoker, opens a broadcasted transaction, and instantiates `PuppyRaffle` with `(entranceFee, feeAddress, duration)`.

Function summaries (≤100 words each):
• `run() external`: Initializes `feeAddress` to `msg.sender`, activates Foundry’s `vm.broadcast()` to sign subsequent transactions, and deploys a fresh `PuppyRaffle` instance.  Returns nothing.

Storage variable summaries (≤50 words each):
• `uint256 entranceFee`: Hard-coded ticket cost (1 ETH).
• `address feeAddress`: Wallet that will receive protocol fees; set during `run()`.
• `uint256 duration`: Time window between raffle draws, fixed at 1 day.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### Contract: PuppyRaffle (PuppyRaffle.sol)
ERC-721 raffle that lets users buy entries to win a randomly-rarified puppy NFT.  Participants pay `entranceFee` per address, duplicates disallowed.  Players can individually `refund` before draw.  Once `raffleDuration` elapses and ≥4 active players exist, anyone may `selectWinner`, which pseudo-randomly picks a winner, mints a puppy with common/rare/legendary rarity, pays 80 % of pot to winner and accrues 20 % fees.  Owner can redirect fees and withdraw them when no active players remain.  Contract packs some variables for gas and stores image URIs & names for each rarity.

---
#### Storage Variables (≤50 words each)
- `uint256 entranceFee` – immutable price per ticket.
- `address[] players` – active entrants, may hold zeroed slots after refunds.
- `uint256 raffleDuration` – seconds each round lasts.
- `uint256 raffleStartTime` – timestamp current round began.
- `address previousWinner` – last round’s winner.
- `address feeAddress` – wallet receiving protocol fees.
- `uint64 totalFees` – accumulated, withdrawable fees.
- `mapping(uint256⇒uint256) tokenIdToRarity` – NFT id → rarity constant.
- `mapping(uint256⇒string) rarityToUri` – rarity constant → metadata image URI.
- `mapping(uint256⇒string) rarityToName` – rarity constant → rarity name.
- `string commonImageUri/rareImageUri/legendaryImageUri` – IPFS images.
- `uint256 COMMON_RARITY/RARE_RARITY/LEGENDARY_RARITY` – probability weights.
- `string COMMON/RARE/LEGENDARY` – text labels.

---
#### Functions (interface + ≤100 words)
`enterRaffle(address[] newPlayers) payable` – Adds addresses to `players` after collecting `entranceFee × len`. Rejects duplicates via nested loop. Emits `RaffleEnter`.

`refund(uint256 playerIndex)` – Active player at index recovers `entranceFee`; slot is zeroed. Emits `RaffleRefunded`.

`getActivePlayerIndex(address player) view returns(uint256)` – Linear search returns first index of address or 0 if absent.

`selectWinner()` – Requires round finished and ≥4 players. Uses hash of sender, timestamp & difficulty to pick winner, second hash for rarity. Sends 80 % pot to winner, adds 20 % to `totalFees`, resets players array, restarts timer, stores winner, mints NFT. Reverts on failed payout.

`withdrawFees()` – Sends `totalFees` to `feeAddress` only when contract balance equals fees (i.e., no active funds from players). Resets `totalFees`.

`changeFeeAddress(address newFeeAddress)` – Owner only; updates fee recipient and emits `FeeAddressChanged`.

`_isActivePlayer() view returns(bool)` – Internal linear check if `msg.sender` is still in `players`.

`_baseURI() pure returns(string)` – Hard-coded data URI prefix.

`tokenURI(uint256 tokenId) view returns(string)` – Base64-encodes on-chain JSON using stored image URI and rarity name for given token.

---
### Documentation: README.md (summary ≤500 words)
Repo hosts Puppy Raffle audit exercise. Requirements: git & Foundry. Quickstart: clone, `make`, or launch Gitpod. Usage focuses on `forge test` & coverage. Audit Scope: only `PuppyRaffle.sol`, commit 2a47715. Compatibility: Solidity 0.7.6, Ethereum. Roles defined: Owner (sets fee address), Player (enters/refunds). No known issues listed. The README mainly guides setup/testing and briefly reiterates protocol rules enumerated in contract comments.


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


