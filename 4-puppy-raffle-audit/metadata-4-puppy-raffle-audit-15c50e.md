
## PROTOCOL OVERVIEW:

# Puppy Raffle Protocol

PuppyRaffle is an on-chain ERC721 raffle: users pay a fixed entrance fee to join rounds for a chance to win both a puppy NFT and most of the pot.

## How it works
- Enter: Call `enterRaffle(address[] participants)` and send `msg.value == entranceFee * participants.length`. You can include yourself multiple times or batch friends. Duplicate addresses revert.
- Refunds: Any entrant may self-refund by index, receiving `entranceFee` back. The player slot is set to `address(0)` (array not compacted).
- Winner selection: After `raffleDuration` and with ≥4 players, anyone can call `selectWinner`. A pseudo-random index is chosen; the winner gets 80% of contract funds, and a Puppy NFT is minted to them with Common/Rare/Legendary rarity and base64 on-chain metadata. 20% is accrued as protocol fees. The round resets for the next raffle.
- Fees: Fees accumulate to `feeAddress` and can be withdrawn only when no active player funds remain. Owner can update `feeAddress`.

## Deployment/Tests
- Deploy script uses 1 ETH entrance fee, 1-day duration, and sets `feeAddress` to the deployer. Comprehensive Foundry tests verify entries, refunds, timing, payouts, NFT minting/URI, and fee withdrawal.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
# Docs: Main List of Files in Project
- script/DeployPuppyRaffle.sol: Deployment script for the PuppyRaffle contract, likely wiring constructor params (entrance fee, fee recipient, raffle duration) and broadcasting the deployment via Foundry scripts.
- src/PuppyRaffle.sol: Core raffle/NFT contract implementing player entry, refund, winner selection, fee withdrawal, and NFT minting/tokenURI.
- test/PuppyRaffleTest.t.sol: Comprehensive Foundry test suite covering entry rules, refunds, indexing, winner selection timing and minimum players, payouts, NFT minting/metadata, and fee withdrawals.

---

## Contract: PuppyRaffleTest (tests)
Contract definition: contract PuppyRaffleTest is Test

Summary (≤200 words):
PuppyRaffleTest is a Foundry test contract validating the behavior of PuppyRaffle. It deploys a fresh instance with a fixed entrance fee, fee recipient, and raffle duration. Tests cover single and multiple entries with exact payment, rejection of underpayment and duplicate players, refund mechanics (self-only, correct amount, array hole), player indexing, and winner selection constraints (time-bound, minimum 4 players). It simulates time/blocks with vm.warp/roll, then asserts deterministic winner selection (playerFour), payout splitting (80% to winner, 20% fees), NFT minting to the winner, tokenURI correctness, and fee withdrawals only when no active players remain. Modifiers set up common preconditions.

Storage variables (≤50 words each):
- PuppyRaffle public puppyRaffle: The contract under test; deployed in setUp and used across tests.
- uint256 public entranceFee = 1e18: Fixed per-player cost to enter the raffle (1 ETH in tests).
- address public playerOne = address(1): Test participant address #1.
- address public playerTwo = address(2): Test participant address #2.
- address public playerThree = address(3): Test participant address #3.
- address public playerFour = address(4): Test participant address #4.
- address public feeAddress = address(99): Recipient of protocol fee upon withdrawal.
- uint256 public duration = 1 days: Raffle duration; used to gate winner selection timing.

Functions (≤100 words each):
- function setUp() public: Deploys a new PuppyRaffle with entranceFee, feeAddress, and duration for isolated testing.
- function testCanEnterRaffle() public: Enters one player by paying entranceFee; asserts players(0) equals playerOne.
- function testCantEnterWithoutPaying() public: Expects revert when calling enterRaffle without sufficient msg.value.
- function testCanEnterRaffleMany() public: Enters two players with exact aggregate payment; verifies order in players array.
- function testCantEnterWithoutPayingMultiple() public: Expects revert when underpaying for multiple players.
- function testCantEnterWithDuplicatePlayers() public: Expects revert when the same address appears twice in a single entry batch.
- function testCantEnterWithDuplicatePlayersMany() public: Expects revert when a batch contains any duplicates among three addresses.
- modifier playerEntered(): Pre-loads the raffle with playerOne by paying entranceFee; used by refund tests.
- function testCanGetRefund() public playerEntered: As playerOne, refunds by index; asserts ETH balance increased by entranceFee.
- function testGettingRefundRemovesThemFromArray() public playerEntered: After refund, verifies players(0) is zeroed (address(0)).
- function testOnlyPlayerCanRefundThemself() public playerEntered: As a different address, refund reverts with permission error.
- function testGetActivePlayerIndexManyPlayers() public: After two entries, validates getActivePlayerIndex returns correct indices.
- modifier playersEntered(): Enters four distinct players with exact total payment; used in winner/fees tests.
- function testCantSelectWinnerBeforeRaffleEnds() public playersEntered: Selecting winner before duration elapses reverts.
- function testCantSelectWinnerWithFewerThanFourPlayers() public: With only three players and elapsed time, selectWinner reverts.
- function testSelectWinner() public playersEntered: After time/block advance, selectWinner executes; asserts previousWinner() == playerFour.
- function testSelectWinnerGetsPaid() public playersEntered: Validates winner receives 80% of pot in ETH post selection.
- function testSelectWinnerGetsAPuppy() public playersEntered: Asserts the winner receives one NFT (balanceOf == 1).
- function testPuppyUriIsRight() public playersEntered: Asserts tokenURI(0) matches the expected base64 JSON data URI.
- function testCantWithdrawFeesIfPlayersActive() public playersEntered: With active players, withdrawFees reverts.
- function testWithdrawFees() public playersEntered: After winner selection, withdraws protocol fees; asserts feeAddress balance equals 20% of pot.

Function interfaces:
- function setUp() public;
- function testCanEnterRaffle() public;
- function testCantEnterWithoutPaying() public;
- function testCanEnterRaffleMany() public;
- function testCantEnterWithoutPayingMultiple() public;
- function testCantEnterWithDuplicatePlayers() public;
- function testCantEnterWithDuplicatePlayersMany() public;
- modifier playerEntered();
- function testCanGetRefund() public;
- function testGettingRefundRemovesThemFromArray() public;
- function testOnlyPlayerCanRefundThemself() public;
- function testGetActivePlayerIndexManyPlayers() public;
- modifier playersEntered();
- function testCantSelectWinnerBeforeRaffleEnds() public;
- function testCantSelectWinnerWithFewerThanFourPlayers() public;
- function testSelectWinner() public;
- function testSelectWinnerGetsPaid() public;
- function testSelectWinnerGetsAPuppy() public;
- function testPuppyUriIsRight() public;
- function testCantWithdrawFeesIfPlayersActive() public;
- function testWithdrawFees() public;


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
# Docs Summary
- File index: The project contains three main files — `script/DeployPuppyRaffle.sol` (deployment script), `src/PuppyRaffle.sol` (core raffle contract), and `test/PuppyRaffleTest.t.sol` (tests). This layout follows standard Foundry structure: source in `src`, deployment logic in `script`, and tests in `test`.

# Contract: DeployPuppyRaffle (deploy script)
- Definition: `contract DeployPuppyRaffle is Script`
- Summary (≤200 words): A Foundry deploy script that broadcasts a transaction to deploy the `PuppyRaffle` contract. It sets `feeAddress` to the script caller (`msg.sender`), then calls `vm.broadcast()` to start broadcasting, and deploys `PuppyRaffle` with parameters: entrance fee `1e18` (1 ETH in wei), the `feeAddress`, and a raffle `duration` of `1 days`. It predefines `entranceFee` and `duration` as script state variables; however, the deployment currently hardcodes `1e18` rather than using the `entranceFee` variable, which could be unified for consistency. Intended usage: run with `forge script` to deploy on a selected network. No access control or environment variable reading is present; parameters are static aside from `feeAddress` derived from the caller.

## Functions
- Interface: `function run() public`
  - Summary (≤100 words): Initializes `feeAddress = msg.sender`, starts transaction broadcasting via Foundry’s `vm.broadcast()`, and deploys a new `PuppyRaffle(1e18, feeAddress, duration)`. Returns nothing and does not store the deployed address. Assumes the caller’s EOA should receive fees. To customize, modify `entranceFee`/`duration` or pass via env/CLI; currently `1e18` is inlined.

## Storage Variables
- Definition: `uint256 entranceFee = 1e18;`
  - Explanation (≤50 words): Default entrance fee for the raffle, set to 1 ETH. Not actually used in the deployment call (literal `1e18` is passed instead).

- Definition: `address feeAddress;`
  - Explanation (≤50 words): Destination address for collected fees. Set to `msg.sender` at runtime in `run()`.

- Definition: `uint256 duration = 1 days;`
  - Explanation (≤50 words): Raffle duration parameter forwarded to the `PuppyRaffle` constructor, set to one day by default.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
# Project Docs

## Main List of Files in Project
- script/DeployPuppyRaffle.sol: Deployment script that likely broadcasts and configures constructor params (not provided here).
- src/PuppyRaffle.sol: Core ERC721 raffle NFT contract implementing entry, refunds, winner selection, fee handling, and on-chain metadata.
- test/PuppyRaffleTest.t.sol: Foundry tests validating raffle flows, fees, NFTs, and edge cases (not provided here).

---

# src/PuppyRaffle.sol

Contract: `PuppyRaffle is ERC721, Ownable`

Summary (≤200 words):
PuppyRaffle is an ERC721 raffle where users pay an entrance fee to join. Players are stored in an array; duplicates are checked via an O(n^2) pass. Entrants can refund, leaving a blank slot. After a time duration and with ≥4 players, anyone can call selectWinner, which pseudo-randomly selects a winner, mints an NFT with rarity-based metadata, sends 80% of funds to the winner, and accrues 20% fees to feeAddress. Fees are withdrawable only when no active players are present. Metadata is Base64-encoded JSON using rarity mappings. Notable risks/bugs: insecure RNG, reentrancy in refund, potential winner = address(0) due to refunded slots, ambiguous getActivePlayerIndex return value, tokenURI JSON likely malformed (missing quotes around rarity value), uses totalSupply() which isn’t defined in OZ ERC721 without Enumerable, fee math relies on players.length not active count, block.difficulty entropy is manipulable.

Functions (interface + ≤100 words each):
- constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration)
  Initializes entrance fee, fee recipient, duration, start time, and rarity URI/name maps. Mints nothing. No validation on _feeAddress. Sets ERC721 name/symbol.

- function enterRaffle(address[] memory newPlayers) public payable
  Requires exact fee per address. Appends all addresses, then checks for duplicates with nested loops (gas-heavy). Emits RaffleEnter. No validation for zero addresses; duplicates revert. Refund-created blanks increase cost of duplicate scan.

- function refund(uint256 playerIndex) public
  Only the indexed player can refund; sends back entranceFee and sets slot to address(0). Emits RaffleRefunded. Vulnerable to reentrancy because state is updated after sending Ether; attacker contract can reenter and double-refund.

- function getActivePlayerIndex(address player) external view returns (uint256)
  Linear search returning matching index, else 0. Ambiguity: genuine index 0 indistinguishable from “not found.” Consider returning (found,bool) or sentinel like type(uint256).max.

- function selectWinner() external
  Requires duration elapsed and ≥4 players. Chooses winnerIndex via hash of caller, timestamp, difficulty; may select address(0) if refunded, causing mint revert. Calculates prize/fess from players.length (not active count), risking insufficient balance and revert. Uses totalSupply() for tokenId (likely missing). Rarity via weak RNG. Resets players, sends prize, mints NFT, updates previousWinner.

- function withdrawFees() external
  Withdraws accumulated fees to feeAddress. Requires contract balance equals totalFees, implying no active player funds remain. Resets totalFees to 0 before transfer. Will revert if anyone sends stray ETH to contract.

- function changeFeeAddress(address newFeeAddress) external onlyOwner
  Updates feeAddress and emits event. No zero-address check.

- function _isActivePlayer() internal view returns (bool)
  Linear check if msg.sender is in players. Not used elsewhere. O(n) gas.

- function _baseURI() internal pure returns (string memory)
  Returns Base64 JSON data URI prefix. Not overriding OZ’s virtual view _baseURI signature; used as an internal helper.

- function tokenURI(uint256 tokenId) public view override returns (string memory)
  Builds Base64-encoded JSON using rarity mappings. Reverts for nonexistent tokens. Likely malformed JSON: rarity value inserted without quotes. Returns on-chain metadata with image IPFS URIs.

Storage (definition + ≤50 words each):
- uint256 public immutable entranceFee
  Fixed wei cost per entry set at construction.

- address[] public players
  Dynamic list of entrants; refunds set slots to address(0), leaving blanks.

- uint256 public raffleDuration
  Duration in seconds between winner selections.

- uint256 public raffleStartTime
  Timestamp when current raffle started; reset on winner selection.

- address public previousWinner
  Address of last raffle winner.

- address public feeAddress
  Recipient of protocol fees; changeable by owner.

- uint64 public totalFees = 0
  Accumulated fees from raffles. Potential overflow in extreme cases; narrow type.

- mapping(uint256 => uint256) public tokenIdToRarity
  TokenId to rarity bucket (COMMON_RARITY, RARE_RARITY, LEGENDARY_RARITY).

- mapping(uint256 => string) public rarityToUri
  Rarity bucket to IPFS image URI.

- mapping(uint256 => string) public rarityToName
  Rarity bucket to rarity name string.

- string private commonImageUri
  IPFS image for common puppy.

- uint256 public constant COMMON_RARITY = 70
  Probability weight threshold for common.

- string private constant COMMON
  Name label for common rarity.

- string private rareImageUri
  IPFS image for rare puppy.

- uint256 public constant RARE_RARITY = 25
  Probability weight threshold for rare.

- string private constant RARE
  Name label for rare rarity.

- string private legendaryImageUri
  IPFS image for legendary puppy.

- uint256 public constant LEGENDARY_RARITY = 5
  Probability weight threshold for legendary.

- string private constant LEGENDARY
  Name label for legendary rarity.


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


