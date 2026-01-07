


════════════════════════════════════════════════════════════════════
███ SECTION 9.1: PROTOCOL OVERVIEW ███
════════════════════════════════════════════════════════════════════
                

# Puppy Raffle – Technical Overview

> NOTE: this document is **derived from code & docs shipped in `src/PuppyRaffle.sol` (Solidity 0.7.6)** plus the accompanying README.  It is *not* promotional material – it is meant to help auditors, reviewers and integrators quickly understand how the protocol functions.

---

## 1. High-level Idea
Puppy Raffle is a simple on-chain lottery that mints an ERC-721 "Puppy" NFT to a randomly selected participant (the *winner*) at the end of each raffle cycle.  Players buy tickets by sending `msg.value == entranceFee` and supplying an address array that represents **who receives each ticket**.  After a configurable duration the raffle can be closed, a winner chosen, prize money distributed, and the next round automatically started.  The contract owner skims a protocol fee that is accumulated and withdrawable to a `feeAddress`.

The contract is entirely self-contained – it handles NFT minting, metadata generation, ticket accounting and funds transfer without any external contracts.


## 2. Main Actors & Roles
1. **Owner (deployer)**  
   • Sets `feeAddress`  
   • Can update `feeAddress` via `changeFeeAddress`  
   • Calls `withdrawFees` to collect protocol revenues.
2. **Player**  
   • Calls `enterRaffle` to buy one or more tickets (can gift tickets to friends by including their addresses).  
   • Can cancel a ticket *they personally own* with `refund` and receive their stake back.  
3. **Winner**  
   • Chosen by `selectWinner`, receives 80 % of pot + a freshly-minted Puppy NFT.


## 3. Storage Layout
| Slot | Variable | Purpose |
|------|----------|---------|
| 0 | `uint256 public immutable entranceFee` | Price per ticket (wei) |
| 1 | `address[] public players` | Dynamic array of active ticket holders (duplicates disallowed) |
| 2 | `uint256 public immutable raffleDuration` | Seconds between raffles |
| 3 | `uint256 public raffleStartTime` | Block timestamp when current round began |
| 4 | `address public previousWinner` | For UI/analytics |
| 5 | `address public feeAddress` | Destination of protocol fees |
| 6 | `uint64 public totalFees` | Fees accumulated but not yet withdrawn |
| … | `mapping(uint256 ⇒ uint8) tokenIdToRarity` | Rarity enum (0 = Common, 1 = Rare, 2 = Legendary) |
| … | `mapping(uint8 ⇒ string) rarityToUri` | Image / animation URI per rarity |
| … | `mapping(uint8 ⇒ string) rarityToName` | Human readable rarity names |
| **Constants** | `COMMON_RARITY = 70`, `RARE_RARITY = 25`, `LEGENDARY_RARITY = 5` | Weighted randomness percentages |

> ⚠️  The contract uses Solidity 0.7.6, therefore **array deletion leaves a gas-heavy gap**; refund sets element to `address(0)` instead of packing.


## 4. Raffle Lifecycle
1. **Initialization** (`constructor`)
   * Sets `entranceFee`, `feeAddress`, `raffleDuration`.
   * `raffleStartTime = block.timestamp`.
   * Pre-populates rarity maps with URIs + names.
2. **Ticket Purchase** (`enterRaffle(address[] calldata participants) payable`)
   * `require(msg.value == entranceFee * participants.length)`.
   * Rejects any address that already exists in `players` (no duplicates per round).
   * Pushes each unique participant into `players`.
   * Emits `RaffleEnter`.
3. **Voluntary Refund** (`refund(uint256 index)`)
   * `require(players[index] == msg.sender)` – players can only refund their **own** ticket.
   * Deletes players[index] (sets to zero addr) and transfers `entranceFee` back.
   * Emits `RaffleRefunded`.
4. **Winner Selection** (`selectWinner()`)
   * `require(block.timestamp ≥ raffleStartTime + raffleDuration)`.
   * Tallies non-zero addresses to find active players.
   * Uses **pseudo random** source: `uint256 random = uint256(keccak256(abi.encodePacked(block.difficulty, block.timestamp, players.length)));` (exact code may vary but there is *no* Chainlink VRF).  This yields `winnerIndex = random % activeCount`.
   * Computes protocol cut: `fee = (pot * 20) / 100`.  `totalFees += fee`.
   * Transfers `(pot - fee)` ether to the winner address.
   * Mints NFT with `tokenId = totalSupply()`.
   * Starts next round: `players = new address[](0)`; `raffleStartTime = block.timestamp`.
5. **Fee Withdrawal** (`withdrawFees()`)
   * `require(players.length == 0)` – can only be called between rounds.
   * Transfers `totalFees` to `feeAddress`; resets counter.
6. **Administrative Upkeep**
   * `changeFeeAddress(address)` – onlyOwner.


## 5. NFT Metadata & Rarity Logic
• `tokenURI(uint256)` builds on-chain JSON, base64-encodes it and prefixes with `data:application/json;base64,`.
• Rarity is assigned on mint by rolling the same `random` used for winner selection:
```
if (roll < LEGENDARY_RARITY)       rarity = LEGENDARY;   // 5 %
else if (roll < LEGENDARY_RARITY+RARE_RARITY) rarity = RARE; // 25 %
else                                 rarity = COMMON;     // 70 %
```
• Each rarity has a different image URI & name string used in metadata.


## 6. Gas & Complexity Considerations
1. **O(n) scans**: To forbid duplicates `enterRaffle` loops over `players` for *every* new ticket – quadratic worst-case when many tickets are bought.  Same for winner selection which must skip `address(0)` holes.
2. **Refund holes**: Setting array slots to zero keeps indices stable but wastes storage/gas later.  Packing or a linked list would be more efficient.
3. **No ERC-20 support**: Payment is fixed to native ETH.  Integrators wanting stable-coin entry must fork.


## 7. Security Review Checklist
| Topic | Status | Notes |
|-------|--------|-------|
| Re-entrancy | Safe | State changes precede external calls; uses built-in `transfer` (2300 gas) which reverts on failure. |
| Randomness manipulation | Weak | Miner can influence `block.timestamp` & `block.difficulty`, giving them minor edge.  Not suitable for large pots. |
| Duplicate prevention | ✅ | Explicit loop check; refund can free slot for same address to re-enter. |
| Denial of service | ☑️ | A player can leave a `0x0` hole then never call `selectWinner` (since anyone may call it); but winner selection still works by skipping holes. |
| Owner abuse | Limited | Owner *cannot* steal pot; fee is hard-coded at 20 %.  Owner could set `feeAddress` to an EOA they control anytime, but that is within spec. |
| Upgradeability | None | Simple `Ownable` (OZ v3.4) – contract is not upgradeable. |


## 8. Extensibility Ideas
1. Replace pseudo-randomness with Chainlink VRF for provable fairness.
2. Use `EnumerableSet` to maintain players and O(1) duplicate checks.
3. Allow multiple concurrent raffle IDs instead of resetting.
4. Gas refund optimisation: map `address ⇒ bool` to mark active participation.
5. Permit ERC-20 entry fees via IERC20 `permit` flow.


## 9. Summary for Integrators
• **`enterRaffle(address[] participants)`** – send `entranceFee * n` wei, where `n = participants.length`.  All addresses will be tracked as having one ticket each.
• **`selectWinner()`** – public; anyone may execute after the timer.  Recommended to set up an automation bot / Keeper.
• **NFTs** – Standard ERC-721 compliant; tokenId increments per win; metadata fully on-chain base64 JSON referencing off-chain image URIs.
• **Events** – Subscribe to `RaffleEnter`, `RaffleRefunded`, and Transfer events to build UI.

---

### Contract Interface Snippet
```
constructor(uint256 fee, address feeAddr, uint256 duration)
function enterRaffle(address[] calldata participants) external payable
function refund(uint256 playerIndex) external
function selectWinner() external
function withdrawFees() external
function changeFeeAddress(address newFeeAddress) external onlyOwner
function getActivePlayerIndex(address player) external view returns (uint256)
function tokenURI(uint256 id) external view returns (string)
```

---

## 10. Final Remarks
Puppy Raffle is a minimalistic "pay-to-play" NFT lottery.  Its core strength is simplicity: one file, no external dependencies besides OpenZeppelin v3.4′s Ownable/ERC-721, making it easy to audit.  The primary weakness is low-entropy randomness, which is acceptable for playful, low-value raffles but should be upgraded for significant prize pools.

        


════════════════════════════════════════════════════════════════════
███ SECTION 9.2: MAIN LIST OF FILES IN PROJECT ███
════════════════════════════════════════════════════════════════════
                

        src/PuppyRaffle.sol




════════════════════════════════════════════════════════════════════
███ SECTION 9.3: DOCUMENTATION ███
════════════════════════════════════════════════════════════════════
                

    ------------------------------ README.md ------------------------------ 

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





════════════════════════════════════════════════════════════════════
███ SECTION 9.4: PACKAGE.JSON HEADERS OF LIB PACKAGES ███
════════════════════════════════════════════════════════════════════
                

        
 *Note*: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.6.0",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/base64/package.json

{
  "name": "base64-sol",
  "version": "1.1.0",
  "description": "base64 implementation in solidity",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },

### lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "3.4.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks",

### lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "3.4.0",
  "files": [
    "/contracts/**/*.sol",
    "/build/contracts/*.json",
    "!/contracts/mocks",




════════════════════════════════════════════════════════════════════
███ SECTION 9.5: CONFIG FILES ███
════════════════════════════════════════════════════════════════════
                

    
 *Note*: Check for important package version info.

 ### foundry.toml

[profile.default]
src = "src"
out = "out"
libs = ["lib"]

remappings = ['@openzeppelin/contracts=lib/openzeppelin-contracts/contracts']

# See more config options https://github.com/foundry-rs/foundry/blob/master/crates/config/README.md#all-options


