
## PROTOCOL OVERVIEW:

## Puppy Raffle – Technical Protocol Overview  

---

### 1. What *is* Puppy Raffle?
Puppy Raffle is a single-contract, non-upgradeable ERC-721 raffle system written for Solidity 0.7.6.  Players pay a fixed entrance fee (default 1 ETH) to join a time-boxed raffle round.  When the round expires, anyone can trigger `selectWinner()`, which
1. pseudo-randomly chooses one **active** player,
2. transfers 80 % of the pot to that winner,
3. accrues 20 % as protocol fees, and
4. mints an NFT that represents a *puppy* whose **rarity** (Common, Rare, Legendary) is derived from the same randomness.

The protocol is permissionless for participation but centralised for fee withdrawal & address management, which are restricted to the owner (initial deployer).  No Chainlink oracles or external dependencies are used for randomness; instead, `blockhash` and on-chain state are hashed—​a weak source acceptable only for low-value games.

---

### 2. High-Level Lifecycle
```
┌────────────┐            ┌────────────────┐         ┌──────────────────┐
│ Raffle Open│─enterFee─►│ players[] adds │         │   Pot keeps ETH  │
└────────────┘            └────────────────┘         └──────────────────┘
       │                                              ▲
       │duration elapsed                              │20 %
       ▼                                              │
┌────────────────┐  selectWinner()  ┌──────────────┐  │
│ RNG & winner   │───────────────► │ Winner gets  │──┘
│ choose rarity  │                 │ 80 % of pot  │
└────────────────┘                 └──────────────┘
           │                            ▲
           ▼                            │
┌────────────────────┐                 │
│ Mint Puppy NFT     │                 │
└────────────────────┘                 │
           │                            │
           ▼                            │
┌────────────────────────┐             │
│ fees accumulate (uint64)│◄───────────┘
└────────────────────────┘
```

Players may also call `refund()` **before** the raffle closes to reclaim their ticket, freeing that slot.

---

### 3. Contract Architecture
Contract: `PuppyRaffle.sol` (≈600 SLOC)

3.1 Storage layout (abridged)
* `entranceFee uint256 immutable` — cost per ticket.
* `players address[]` — dynamic list; “empty” slots are set to address(0) when refunded.
* `raffleDuration, raffleStartTime` — time keeping.
* `previousWinner address` — last cycle’s victor.
* `feeAddress address` — withdraw target (initially deployer EOA).
* `totalFees uint64` — accumulated fees pending withdrawal.
* NFT support: `tokenIdToRarity`, `rarityToUri`, `rarityToName` plus constants for rarity thresholds.

3.2 Constructor
```solidity
constructor(
  uint256 _entranceFee,
  address _feeAddress,
  uint256 _raffleDuration
) ERC721("Puppy Raffle", "PR")
```
Initialises fees/duration, seeds metadata URIs & names, and sets `raffleStartTime = block.timestamp` so the first round starts immediately.

3.3 Core public/external functions
* `enterRaffle(address[] calldata newPlayers)`
  * `msg.value` must equal `entranceFee * newPlayers.length`.
  * Rejects duplicate addresses across **entire** `players[]`.
* `refund(uint256 playerIndex)`
  * Callable only by the address stored at `players[playerIndex]`.
  * Sends `entranceFee` back and blanks the slot (address(0)).
* `selectWinner()`
  * `require(block.timestamp ≥ raffleStartTime + raffleDuration)`
  * Uses `uint256(keccak256(abi.encodePacked(blockhash(block.number-1), players.length, address(this)))) % players.length` for index.
  * Calculates payouts: `prize = (address(this).balance * 80) / 100; fee = …20%`.
  * Transfers `prize` to winner, increments `totalFees`, mints NFT with tokenId = `totalSupply()+1`, stores rarity.
  * Resets players array & `raffleStartTime`.
* `withdrawFees()` — onlyOwner, only when `players.length == 0`, sends `totalFees` to `feeAddress`.
* `changeFeeAddress(address)` — onlyOwner.
* View helpers: `getActivePlayerIndex`, overrides for `_baseURI` and `tokenURI` (on-chain Base64 metadata).

No upgrade functions, pausing, or emergency controls are present.

---

### 4. Economic & Fee Model
* Entrance cost: **fixed** at deployment (1 ETH in scripted deployment but constructor-configurable).
* Payout split: 80 % → round winner, 20 % → protocol (`totalFees`).
* Fees are pulled **out-of-band** by owner via `withdrawFees()`.  This design allows gas savings inside `selectWinner()` but introduces custodial risk if owner never calls `withdrawFees()`.

---

### 5. Randomness & Rarity Mapping
Random seed: `blockhash(block.number-1)` + deterministic state.  Because miners can influence the block hash, the system is not provably fair for high-value prizes.  *Mitigations*: economic caps, commit-reveal or Chainlink VRF in future.

Rarity thresholds are hard-coded constants:
* `COMMON_RARITY = 8075` (≈80.75 %)
* `RARE_RARITY = 9575`  (≈15.5 %)
* Remaining ≈3 % → Legendary.

`selectWinner()` computes `rand % 10000` to pick rarity bucket.

---

### 6. Deployment Script (`script/DeployPuppyRaffle.sol`)
* Foundry `forge script` style; no environment file.
* Arguments are **fixed** in source: `entranceFee = 1e18`, `duration = 1 days`.
* `feeAddress = msg.sender` fetched at script runtime.
* `vm.broadcast(privateKey)` wraps a single `new PuppyRaffle(...)` TX.
* Idempotency: none — every run deploys another instance.
* Post-deploy verification or ownership transfer: none.

Trust model: whoever owns the supplied private key is the on-chain owner & fee beneficiary.

---

### 7. Roles & Permissions
1. **Owner** (deployer EOA)
   * `onlyOwner` modifier on `withdrawFees()` & `changeFeeAddress()`.
2. **Player**
   * Anyone able to pay `entranceFee`.
   * May call `refund()` for *their* slot.

No other roles, no multisig, no upgrade admin.

---

### 8. Time-boxed Raffle Cycle
1. `raffleStartTime` set in constructor & after each `selectWinner()`.
2. While `block.timestamp < raffleStartTime + raffleDuration` players may `enterRaffle` or `refund`.
3. After duration expires:
   * Anyone can `selectWinner()` (permissionless execution).
   * Internally resets state for a fresh round; no external “start” call is necessary.

Because duration is **fixed forever**, the game cadence can only be changed by redeploying.

---

### 9. Security & Edge-Case Review
* **Randomness bias** — predictable to miners/validators.  Suitable only for small stakes.
* **Duplicate checking** — O(n) loop inside `enterRaffle`, gas scales linearly with players; DOS risk if player list grows large (> 400 ish).
* **Empty slots** — refunds set address(0) but index remains, so winner selection might land on a null slot → function reverts (`address(0)` cannot receive prize).  Therefore players[] is cleared after each round, mitigating leftover holes, but holes *within* the current round are dangerous because `selectWinner()` chooses among `players.length`, not *active* players.  A malicious user could DOS the raffle by buying tickets and refunding, making array full of zeros.
* **Fee withdrawal gating** — requires `players.length == 0`; after DOS-refund attacker, owner cannot withdraw.
* **Re-entrancy** — relies on Solidity 0.7 re-entrancy pattern; prize & refund transfers use native `transfer()` which forwards 2300 gas (safe in 0.7.x) but will break for contracts behind proxies post-EIP-1884/2929.  Consider `call{value:…}` with re-entrancy guard instead.
* **URI Hard-coding** — images stored as raw URI strings; cannot be changed later; NFTs are immutable.

---

### 10. Gas & Optimisation Notes
* `uint64 totalFees` packs tightly but balance arithmetic uses uint256 casts.
* `players` array cleaning (`delete players;`) costs O(n) refund gas after each round.
* Duplicate check nested loop ➜ O(n²) worst-case over lifetime; consider mapping for constant-time membership.
* Constant strings could be `bytes32` to save storage.

---

### 11. Upgradeability & Extensibility
The contract is not upgrade-safe (no proxy hooks, `immutable` variables).  Any parameter change (fee, duration, randomness source) requires redeployment and user migration.

Potential roadmap improvements:
* Replace RNG with Chainlink VRF.
* Introduce `pause()` & emergency drain.
* Make entrance fee denominated in ERC-20.
* Support multiple concurrent raffles (struct-based mapping).

---

### 12. Developer Usage Guide
1. **Run Local Tests**
   ```bash
   forge install
   forge test -vvv
   ```
2. **Deploy**
   ```bash
   export PRIVATE_KEY=<pk>
   forge script script/DeployPuppyRaffle.sol:DeployPuppyRaffle \
        --private-key $PRIVATE_KEY \
        --rpc-url https://your.rpc.node \
        --broadcast -vvvv
   ```
3. **Interact** (cast or front-end)
   * `enterRaffle([<yourAddr>])`  (pay 1 ETH)
   * wait 24 h → `selectWinner()`
   * Owner: `withdrawFees()`

---

### 13. Conclusion
Puppy Raffle delivers a minimalistic on-chain raffle with NFT rewards and a simple fee split.  Its elegance lies in having *one* contract handle custody, winner selection, and ERC-721 minting.  However, the absence of a secure randomness oracle and some gas/edge-case deficiencies limit production readiness.  For educational or low-stakes deployments it serves as a clear reference implementation of time-boxed raffles, but a professional launch should harden randomness, array management, and administrative safety controls.



## Main List of Files in Project

src/PuppyRaffle.sol


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



 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
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

### lib/forge-std/lib/ds-test/package.json

{
  "name": "ds-test",
  "version": "1.0.0",
  "description": "Assertions, equality checks and other test helpers ",
  "bugs": "https://github.com/dapphub/ds-test/issues",
  "license": "GPL-3.0",
  "author": "Contributors to ds-test",
  "files": [

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


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### foundry.toml

[profile.default]
src = "src"
out = "out"
libs = ["lib"]

remappings = ['@openzeppelin/contracts=lib/openzeppelin-contracts/contracts']

# See more config options https://github.com/foundry-rs/foundry/blob/master/crates/config/README.md#all-options


