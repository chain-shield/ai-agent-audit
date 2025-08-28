
## PROTOCOL OVERVIEW:

# Puppy Raffle Protocol

- Overview
  - Puppy Raffle is an ERC721-based on-chain raffle that mints a “Puppy” NFT to the winner. Players enter by paying a fixed entrance fee per slot.

- How it works
  - enterRaffle(address[] participants): pay entranceFee × participants.length; duplicate addresses are rejected.
  - Players can self-refund by index, reclaiming their entrance fee and freeing their slot.
  - After raffleDuration elapses and at least four entries exist, anyone can call selectWinner:
    - 80% of the pot goes to the winner; 20% accrues as protocol fees for feeAddress.
    - The winner is minted one NFT with on-chain Base64 metadata; previousWinner is updated and the round resets.
  - Fees are withdrawn by feeAddress via withdrawFees; the owner can change feeAddress.

- Deployment
  - A Foundry script deploys PuppyRaffle with parameters (entranceFee, feeAddress = deployer, duration = 1 day).

- Caveats
  - RNG uses block data and msg.sender (manipulable under certain conditions).
  - Duplicate checks and accounting rely on players.length and include refunded “holes,” which can cause gas blowups, misaccounting, or blocked fee withdrawals.
  - tokenURI JSON has minor formatting quirks; ensure consumer robustness.


## SUMMARY OF FILE: 4-puppy-raffle-audit/test/PuppyRaffleTest.t.sol
# Project Overview (Docs)
- script/DeployPuppyRaffle.sol: Deployment script for PuppyRaffle, likely wiring constructor params (entranceFee, feeAddress, duration) and broadcasting a deploy transaction.
- src/PuppyRaffle.sol: Core raffle/NFT contract the tests target. Manages player entries, refunds, winner selection, fee withdrawal, and NFT minting with a fixed tokenURI.
- test/PuppyRaffleTest.t.sol: Foundry test suite validating all critical flows: entering, refunds, indexing, winner selection timing and constraints, payout split, NFT minting/URI, and fee withdrawals.

# Contract: PuppyRaffleTest (contract PuppyRaffleTest is Test)
A Foundry test contract that stands up a PuppyRaffle instance and rigorously verifies behavior. It configures constants like entranceFee (1 ETH), a fee recipient, and raffle duration (1 day). Tests cover: correct payment gating for single/multi-entry, duplicate prevention, player self-refund mechanics, player indexing, winner selection timing and minimum participant constraints, deterministic winner expectation (playerFour given test seeding), payout split (80% to winner, 20% to fees), ERC721 minting to winner with a specific base64 tokenURI, and fee withdrawal only after the raffle round resolves. Uses cheatcodes (vm.prank, vm.expectRevert, vm.warp, vm.roll) to simulate users, time, and block changes.

Storage Variables
- PuppyRaffle puppyRaffle; Instance under test deployed in setUp to run all scenarios.
- uint256 entranceFee = 1e18; Fixed ticket price (1 ETH) used in payment assertions.
- address playerOne/Two/Three/Four; Mock participant addresses entering and interacting.
- address feeAddress = address(99); Recipient of 20% fee upon fee withdrawal.
- uint256 duration = 1 days; Raffle round length used to gate selectWinner timing.

Functions and Modifiers
- function setUp() public
  Deploys PuppyRaffle with entranceFee, feeAddress, and duration, preparing test state. Interface: setUp().

- function testCanEnterRaffle() public
  Verifies single-player entry succeeds when exact fee sent and player is recorded at index 0. Interface: testCanEnterRaffle().

- function testCantEnterWithoutPaying() public
  Ensures enterRaffle reverts without sufficient ETH with message “Must send enough to enter raffle.” Interface: testCantEnterWithoutPaying().

- function testCanEnterRaffleMany() public
  Checks multi-entry for two players succeeds when paying 2x fee and both are stored in order. Interface: testCanEnterRaffleMany().

- function testCantEnterWithoutPayingMultiple() public
  Validates underpayment for multiple players reverts with the same insufficient funds error. Interface: testCantEnterWithoutPayingMultiple().

- function testCantEnterWithDuplicatePlayers() public
  Asserts duplicate addresses in a batch revert with “Duplicate player.” Interface: testCantEnterWithDuplicatePlayers().

- function testCantEnterWithDuplicatePlayersMany() public
  Extends duplicate-checking to larger batches; any duplicate causes revert. Interface: testCantEnterWithDuplicatePlayersMany().

- modifier playerEntered()
  Helper that enrolls playerOne with exact fee, then runs the test body. Interface: playerEntered().

- function testCanGetRefund() public playerEntered
  Confirms a player can self-refund by index, increasing their balance by entranceFee. Interface: testCanGetRefund().

- function testGettingRefundRemovesThemFromArray() public playerEntered
  Ensures refund zeroes the player slot (sets address(0)) to mark removal. Interface: testGettingRefundRemovesThemFromArray().

- function testOnlyPlayerCanRefundThemself() public playerEntered
  Verifies only the indexed player can call refund; others revert with corresponding message. Interface: testOnlyPlayerCanRefundThemself().

- function testGetActivePlayerIndexManyPlayers() public
  Validates getActivePlayerIndex returns correct indices after multi-entry. Interface: testGetActivePlayerIndexManyPlayers().

- modifier playersEntered()
  Enrolls four players for tests that require full quorum, then proceeds. Interface: playersEntered().

- function testCantSelectWinnerBeforeRaffleEnds() public playersEntered
  Asserts selecting a winner before duration elapses reverts with “Raffle not over.” Interface: testCantSelectWinnerBeforeRaffleEnds().

- function testCantSelectWinnerWithFewerThanFourPlayers() public
  Ensures winner selection requires at least 4 players; reverts otherwise even after time passes. Interface: testCantSelectWinnerWithFewerThanFourPlayers().

- function testSelectWinner() public playersEntered
  After time/roll advance, calls selectWinner and expects previousWinner == playerFour (determinism under test seed). Interface: testSelectWinner().

- function testSelectWinnerGetsPaid() public playersEntered
  Confirms winner receives 80% of pot (4 * fee * 80/100). Interface: testSelectWinnerGetsPaid().

- function testSelectWinnerGetsAPuppy() public playersEntered
  Verifies ERC721 mint: winner’s balance increases to 1. Interface: testSelectWinnerGetsAPuppy().

- function testPuppyUriIsRight() public playersEntered
  Checks tokenURI(0) equals a specific base64-encoded JSON URI. Interface: testPuppyUriIsRight().

- function testCantWithdrawFeesIfPlayersActive() public playersEntered
  Prevents fee withdrawal while players remain active; reverts appropriately. Interface: testCantWithdrawFeesIfPlayersActive().

- function testWithdrawFees() public playersEntered
  After winner selection, allows withdrawFees to send 20% of pot to feeAddress. Interface: testWithdrawFees().


## SUMMARY OF FILE: 4-puppy-raffle-audit/script/DeployPuppyRaffle.sol
Docs Summary: Main List of Files in Project
- script/DeployPuppyRaffle.sol: Foundry deploy script that broadcasts a transaction and deploys the PuppyRaffle contract with preset parameters (1 ETH entrance fee, deployer as feeAddress, 1-day duration).
- src/PuppyRaffle.sol: Core raffle contract (not shown here).
- test/PuppyRaffleTest.t.sol: Foundry tests for PuppyRaffle (not shown here).

Contract: DeployPuppyRaffle (deploy script)
- Definition: `contract DeployPuppyRaffle is Script`
- Summary (≤200 words): A Foundry Script used to deploy the PuppyRaffle contract on-chain. It sets the fee recipient to the script caller, then uses Foundry’s `vm.broadcast()` cheatcode to send a transaction that deploys `PuppyRaffle` with hard-coded parameters. The script configures an entrance fee of 1 ETH and a raffle duration of one day. This script encapsulates deployment logic for reproducibility across environments, ensuring the deployer address becomes the fee collection address. It relies on Solidity 0.7.6 and imports `forge-std/Script.sol` for scripting utilities and `PuppyRaffle.sol` for the target contract type. No access control, validation, or environment branching is included; configuration is simple and inline.

Storage Variables (≤50 words each)
- `uint256 entranceFee = 1e18;` Fixed entrance fee of 1 ETH used during deployment when constructing PuppyRaffle. Not modified after initialization in this script.
- `address feeAddress;` Fee recipient set to the script caller (`msg.sender`) at runtime in `run()`. Passed to the PuppyRaffle constructor.
- `uint256 duration = 1 days;` Fixed raffle duration of 1 day used for deployment. Provided to the PuppyRaffle constructor.

Functions (≤100 words each)
- Interface: `function run() public`
  Summary: Sets `feeAddress` to `msg.sender`, invokes `vm.broadcast()` to begin sending subsequent operations as a real transaction, and deploys a new `PuppyRaffle(1e18, feeAddress, duration)`. This produces an on-chain deployment with the configured parameters, using the deployer as the fee recipient. No return value, minimal configuration.


## SUMMARY OF FILE: 4-puppy-raffle-audit/src/PuppyRaffle.sol
Docs: Main List of Files in Project
- Overview: The project contains a deployment script (script/DeployPuppyRaffle.sol), the core ERC721 raffle contract (src/PuppyRaffle.sol), and Foundry tests (test/PuppyRaffleTest.t.sol). The script likely deploys and wires initial parameters, while tests validate raffle entry, refunding, winner selection, fees, and metadata.

Contract: PuppyRaffle (ERC721, Ownable)
- Definition: contract PuppyRaffle is ERC721, Ownable
- Summary (<=200 words): ERC721 raffle where users pay an entrance fee to enter via batched addresses; duplicate entrants are disallowed. Participants can refund to vacate their slot (leaving holes). After a duration, anyone can select a winner: 80% of funds go to the winner and 20% to a fee address; an NFT puppy is minted with rarity-based metadata. Critical issues: O(n^2) duplicate check across all historical players causing gas blowups and potential permanent reverts when multiple refunded (address(0)) entries exist. RNG uses block timestamp/difficulty and msg.sender, making it manipulable. Accounting uses players.length (including refunded zeros) to compute prize/fee, which can exceed contract balance, burn prizes to address(0), and desync totalFees vs balance blocking withdrawals. getActivePlayerIndex returns 0 for “not found,” colliding with real index 0. tokenURI JSON omits quotes around rarity value. Off-by-one rarity distribution (71/25/4 instead of 70/25/5). Uses ERC721.totalSupply without inheriting Enumerable, likely a compile error.

Storage Variables (<=50 words each)
- uint256 public immutable entranceFee: Fixed cost per entry in wei, set at construction; used for payment, refunds, and accounting.
- address[] public players: Current raffle participants; refunds set entries to address(0), leaving holes and affecting accounting and winner selection.
- uint256 public raffleDuration: Seconds each raffle round must run before selecting a winner.
- uint256 public raffleStartTime: Timestamp when current round started; used to enforce duration before selecting winner.
- address public previousWinner: Stores last round’s winner address for reference/UX.
- address public feeAddress: Recipient of protocol fee; can be changed by owner.
- uint64 public totalFees = 0: Accumulated fees pending withdrawal; compared to contract balance to gate withdrawals.
- mapping(uint256 => uint256) public tokenIdToRarity: TokenID to rarity tier (keys: 70, 25, 5) for metadata.
- mapping(uint256 => string) public rarityToUri: Rarity tier to image URI; initialized in constructor.
- mapping(uint256 => string) public rarityToName: Rarity tier to human-readable name string; initialized in constructor.
- string private commonImageUri: IPFS image for common puppy; used to seed rarityToUri.
- uint256 public constant COMMON_RARITY = 70: Target percent weight for common tier; also used as key in maps.
- string private constant COMMON = "common": Name label for common tier; stored in rarityToName.
- string private rareImageUri: IPFS image for rare puppy; used in maps.
- uint256 public constant RARE_RARITY = 25: Weight/key for rare tier.
- string private constant RARE = "rare": Name label for rare tier.
- string private legendaryImageUri: IPFS image for legendary puppy; used in maps.
- uint256 public constant LEGENDARY_RARITY = 5: Weight/key for legendary tier.
- string private constant LEGENDARY = "legendary": Name label for legendary tier.

Functions (<=100 words each, with interface)
- constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration) ERC721("Puppy Raffle", "PR")
  Initializes entrance fee, fee address, raffle duration, and start time. Seeds rarity-to-URI and rarity-to-name lookups. No validation on feeAddress (could be zero). Sets immutable entranceFee. Note: relies on ERC721 implementation providing name(), _exists, etc. Potential missing Enumerable for totalSupply later.

- function enterRaffle(address[] memory newPlayers) public payable
  Requires exact payment for number of addresses. Appends all provided addresses to players, then performs O(n^2) global duplicate check across entire array, reverting on any repeated address (including address(0)). After multiple refunds create two zeros, all future entries can revert. Emits RaffleEnter.

- function refund(uint256 playerIndex) public
  Only the indexed player can refund; pays back entranceFee and sets their slot to address(0). Leaves holes in players, enabling zero-address “participants,” affecting length-based accounting/winner selection. Emits RaffleRefunded. No reentrancy guard; uses Address.sendValue.

- function getActivePlayerIndex(address player) external view returns (uint256)
  Linear search returning the first index of player, else 0. Ambiguous: a real participant at index 0 is indistinguishable from “not found.” Holes persist; function doesn’t skip zeros. Purely a helper for finding refund indices.

- function selectWinner() external
  Requires duration elapsed and at least 4 entries by array length (includes zeros). Picks winnerIndex via insecure RNG (timestamp/difficulty/msg.sender), so manipulable; winner can be address(0). Computes prize/fee from players.length (overcounts due to refunds), potentially exceeding balance and desyncing totalFees. Off-by-one rarity distribution. Resets players, records previousWinner, sends prize via call, mints tokenId = totalSupply() (likely missing Enumerable).

- function withdrawFees() external
  Allows fee withdrawal only if contract balance equals totalFees (assumes no active players). Due to misaccounting (length includes zeros), totalFees can exceed balance, permanently blocking withdrawals or causing earlier revert. Resets totalFees to 0 after transfer.

- function changeFeeAddress(address newFeeAddress) external onlyOwner
  Updates feeAddress and emits event. No validation on zero address. Owner-only.

- function _isActivePlayer() internal view returns (bool)
  Linear scan to check if msg.sender exists in players. Unused in the contract; potential helper for future guards.

- function _baseURI() internal pure returns (string memory)
  Returns the base prefix for on-chain Base64 metadata: "data:application/json;base64,". Not marked override; may or may not override depending on OZ version.

- function tokenURI(uint256 tokenId) public view virtual override returns (string memory)
  Requires token exists, builds Base64-encoded JSON using rarity lookups for name/image. JSON bug: rarity value lacks quotes (invalid JSON). Depends on tokenIdToRarity set in selectWinner. Uses name() from ERC721. Relies on _baseURI prefix.


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


