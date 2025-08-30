
## PROTOCOL OVERVIEW:

**Puppy Raffle Protocol**  
Puppy Raffle is an on-chain, winner-takes-most raffle that mints a uniquely-rarified Puppy NFT to the champion.

1. Ticketing  
• Anyone calls `enterRaffle(address[] participants)` sending `entranceFee × n` ETH to register *n* unique addresses.  
• Each address represents one ticket; duplicates revert.  
• A ticket holder can call `refund(index)` to withdraw and delete their slot before the draw.

2. Draw Cycle  
• The raffle starts at deployment and lasts `raffleDuration` seconds.  
• Once the period elapses and ≥ 4 active players remain, the owner calls `selectWinner()`.  
• A pseudo-random index (block.timestamp, block.difficulty, players length) is chosen.  
• Winner receives 80 % of the escrowed pot and is minted one Puppy NFT.  
• NFT rarity is picked on-chain: Common 70 %, Rare 25 %, Legendary 5 %; each rarity maps to its own IPFS image/metadata.

3. Fees & Admin  
• The remaining 20 % accumulates in `totalFees`; owner can update `feeAddress`.  
• `withdrawFees()` transfers fees to `feeAddress` only when no active players remain.

4. Ecosystem  
Built with Solidity 0.7.6, Foundry tests ensure entry, refund, randomness, payouts, NFT URI, and fee logic. `DeployPuppyRaffle.sol` script deploys the contract with preset `entranceFee`, `duration`, and sets the deployer as fee receiver.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
Puppy Raffle project: README outlines a raffle where players pay an entrance fee to join via enterRaffle(address[] participants). Duplicate addresses disallowed. Players may refund their ticket (refund(uint256 index)). After a fixed duration the contract owner can call selectWinner() if ≥4 active players; function picks pseudo-random winner, mints 1 Puppy NFT (ERC-721) to them, pays winner 80% of pot, keeps 20% as protocol fee. Fee address adjustable by owner; withdrawFees() sends accumulated fees when no active players. Tests confirm:
• Correct entry payment & duplication checks
• Refund restrictions and array deletion
• getActivePlayerIndex helper
• selectWinner timing, player count, payout, NFT URI, previousWinner()
• withdrawFees only when players array empty. Constructor takes entranceFee, feeAddress, duration. Deploy script instantiates PuppyRaffle with those params. No known issues.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
### Contract: DeployPuppyRaffle (script)
Purpose/Trust: Off-chain deployment helper; holds no user funds, only instantiates PuppyRaffle and sets fee receiver to deployer.

Storage
• entranceFee – uint256 – ETH required per ticket
• feeAddress – address – receiver of protocol fees
• duration – uint256 – raffle length

Major Entrypoints
1. run() public
   ‑ Visibility: public
   ‑ Modifiers: vm.broadcast (Foundry cheatcode)  
   ‑ Mutability: none (but creates contract)
   ‑ Natspec: Deploys PuppyRaffle with predefined params and sets caller as fee receiver.

### Documentation (README)
Puppy Raffle lets players buy raffle tickets via `enterRaffle(address[] participants)`; duplicates disallowed, tickets refundable via `refund()`. At configurable intervals a winner is drawn and gets both a random Puppy NFT and ticket pot minus protocol fee. Owner can update `feeAddress`. Scope uses Solidity 0.7.6, Foundry tests, and provides deployment script above.  


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### PuppyRaffle.sol
PuppyRaffle is an ERC721 raffle that sells tickets (entranceFee wei each). Anyone can add one or more addresses via enterRaffle; duplicates are rejected. Players may self-refund before the draw. After raffleDuration has passed and at least four active players remain, selectWinner pays 80 % of the pot to a pseudorandom winner, mints them a puppy NFT (common/rare/legendary), and adds 20 % to totalFees. Owner can update feeAddress; fees are withdrawable once no players are active. Trust model: users escrow ETH; only owner privilege is changing fee receiver. Randomness depends on block variables (not provably fair).

Storage variables
• entranceFee – ticket cost in wei
• players – dynamic array of entrants
• raffleDuration – seconds between draws
• raffleStartTime – timestamp raffle began
• previousWinner – last winner address
• feeAddress – address receiving protocol cut
• totalFees – accumulated 20 % fees
• tokenIdToRarity – NFT id → rarity code
• rarityToUri – rarity → image URI
• rarityToName – rarity → rarity name
• commonImageUri – IPFS for common pug
• COMMON_RARITY – 70 probability points
• rareImageUri – IPFS for rare st. bernard
• RARE_RARITY – 25 probability points
• legendaryImageUri – IPFS for legendary shiba
• LEGENDARY_RARITY – 5 probability points

Functions (interface ‑ natspec ≤100 chars)
• constructor(uint256,address,uint256) public – init fees,duration,uris
• enterRaffle(address[]) public payable – Pay fee*len & add unique entrants
• refund(uint256) public – Player gets fee back, slot nulled
• getActivePlayerIndex(address) external view – Find player index or 0
• selectWinner() external – Draw winner, mint NFT, split pot
• withdrawFees() external – Send accumulated fees to feeAddress
• changeFeeAddress(address) external onlyOwner – Update fee recipient
• _isActivePlayer() internal view – Helper to check sender in players
• _baseURI() internal pure – Returns data:app/json base64 prefix
• tokenURI(uint256) public view – On-chain metadata renderer

### Documentation Summary (README ≤200 words)
Repository shows a raffle protocol audited exercise. Requirements: git & Foundry. Quickstart clones repo and runs `make`. Testing via `forge test` and coverage commands. Audit scope includes only PuppyRaffle.sol (solc 0.7.6) deployed on Ethereum. Roles: Owner can change feeAddress; Player can enter and refund. Known issues: none.


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


