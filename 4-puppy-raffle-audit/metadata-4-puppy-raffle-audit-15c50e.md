
## PROTOCOL OVERVIEW:

## Puppy Raffle Protocol
Puppy Raffle is an on-chain game where users pay a fixed entranceFee to join a time-boxed raffle for a unique Puppy NFT and 80 % of the ETH pot.

Workflow
1. **enterRaffle(address[] players)** – Sender supplies entranceFee × players.length; duplicates or under-payment revert, and every address becomes an active ticket.
2. **refund(uint256 index)** – Before the draw, a player can self-refund, receiving their stake and freeing the slot.
3. **selectWinner()** – After raffleDuration seconds and with ≥4 active players, anyone may trigger:
   • pseudo-random winner selection
   • ERC-721 Puppy mint with rarity-linked metadata to the winner
   • transfer of 80 % of contract balance to the winner
   • allocation of the remaining 20 % to protocol fees (totalFees).
4. **withdrawFees()** – Ownerless, but callable by anyone when no active players; sends accumulated fees to feeAddress.
5. **changeFeeAddress()** – Only owner control; updates fee recipient.

Security & Design
• Funds are custodial only until a round ends; players can exit at will.
• No admin keys affect game integrity; owner only reroutes fees.
• Extensive Foundry tests cover entry, refunds, winner logic, payouts, and fee withdrawal.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
### PuppyRaffleTest (100 words)
Purpose: Foundry test suite validating PuppyRaffle. Holds no user funds; interacts with live PuppyRaffle instance. No admin.

Storage
• puppyRaffle – instance under test  
• entranceFee – 1e18 wei  
• playerOne–playerFour – sample EOA addrs  
• feeAddress – fee recipient  
• duration – raffle length

Public / external fns (all test helpers; non-payable unless noted)
1. setUp() public  – deploy new PuppyRaffle before each test.  
2. testCanEnterRaffle() public  – assert single entry works.  
3. testCantEnterWithoutPaying() public  – reverts if fee missing.  
4. testCanEnterRaffleMany() public  – assert multi entry.  
5. testCantEnterWithoutPayingMultiple() public  – revert low fee multi.  
6. testCantEnterWithDuplicatePlayers() public – revert dup 2.  
7. testCantEnterWithDuplicatePlayersMany() public – revert dup 3.
8. testCanGetRefund() public – player refund success.  
9. testGettingRefundRemovesThemFromArray() public – refund cleans array.  
10. testOnlyPlayerCanRefundThemself() public – access control.  
11. testGetActivePlayerIndexManyPlayers() public – index helper.  
12. testCantSelectWinnerBeforeRaffleEnds() public – time guard.  
13. testCantSelectWinnerWithFewerThanFourPlayers() public – min players guard.  
14. testSelectWinner() public – selects & stores winner.  
15. testSelectWinnerGetsPaid() public – payout 80%.  
16. testSelectWinnerGetsAPuppy() public – NFT mint.  
17. testPuppyUriIsRight() public – tokenURI correctness.  
18. testCantWithdrawFeesIfPlayersActive() public – withdraw guard.  
19. testWithdrawFees() public – feeAddress gets 20%.

All functions: /// @notice Test helper ensuring PuppyRaffle behaves as specified.


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
### DeployPuppyRaffle
Purpose: Simple deployment script; no user funds held, only used by admin/operator to deploy main PuppyRaffle contract.

Storage
- entranceFee (uint256) ‒ ETH ticket price
- feeAddress (address) ‒ where protocol fees go
- duration (uint256) ‒ raffle length

External/Public Functions
1. `function run() public nonpayable`  
   @notice Deploy PuppyRaffle with preset params and set caller as feeAddress


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
### PuppyRaffle (src/PuppyRaffle.sol)
NFT raffle where users pre-pay entranceFee to join. Owner sets feeAddress; on each raffle end the contract picks pseudo-random winner, mints ERC721 puppy with rarity, sends 80 % pot to winner, accrues 20 % fees. Players may refund before draw. Fees withdrawable only when no active players. Trust: users trust owner only for fee address change; funds custodial until draw.

Storage
- entranceFee – cost per ticket (wei)
- players – current entrants
- raffleDuration – seconds per round
- raffleStartTime – round start
- previousWinner – last winner
- feeAddress – fee receiver
- totalFees – accumulated protocol fees
- tokenIdToRarity – id ⇒ rarity
- rarityToUri – rarity ⇒ image URI
- rarityToName – rarity ⇒ name

External/Public API
1. enterRaffle(address[] newPlayers) public payable
   /// Pay fee * n players; adds entrants, rejects duplicates
2. refund(uint256 playerIndex) public
   /// Player withdraws ticket; slot set to 0
3. getActivePlayerIndex(address player) external view
   /// Returns index or 0 if not found
4. selectWinner() external
   /// Ends round, picks winner, mints NFT, splits funds
5. withdrawFees() external
   /// Sends accumulated fees to feeAddress; only when no players
6. changeFeeAddress(address newFeeAddress) external onlyOwner
   /// Owner updates fee recipient
7. tokenURI(uint256 tokenId) public view override
   /// Metadata JSON with rarity-based image


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


