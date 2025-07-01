
## List of Files in Src Folder
src/PuppyRaffle.sol

## README.md summary
Puppy Raffle is a Solidity-based raffle system where users buy entries to win a random dog NFT. Core workflow: anyone calls enterRaffle(address[] participants) supplying ETH and a list of unique addresses; duplicates are rejected. Entrants may later reclaim their stake via refund(). At configurable time intervals the contract chooses a random winner, mints the puppy NFT to them, sends them the prize pool minus protocol fees, and forwards those fees to a feeAddress settable by the owner. 

Project setup uses Foundry (solc 0.7.6). Clone repo, run `make`, then `forge test` or `forge coverage` for unit tests and coverage reports. An optional Gitpod button enables browser development.

Audit scope includes only src/PuppyRaffle.sol at commit 2a47715b30cf11ca82db148704e67652ad679cd8. Roles: Owner (deployer) can update feeAddress via changeFeeAddress; Player participates via enterRaffle and refund. No known issues reported.

Compatibility: Ethereum deployments, solc 0.7.6.

Folder list: src/PuppyRaffle.sol


## src/PuppyRaffle.sol summary
### Contract: PuppyRaffle
ERC-721 raffle that lets users buy entries, refunds tickets, selects a random winner after a fixed period, mints a puppy NFT of random rarity, pays 80 % of pot to winner and accumulates 20 % fees for protocol. Owner can change fee recipient and withdraw accumulated fees. Rarity-specific metadata is returned fully on-chain via Base64 JSON.

---
#### Contract Interface
```solidity
constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration)
function enterRaffle(address[] calldata newPlayers) external payable
function refund(uint256 playerIndex) external
function getActivePlayerIndex(address player) external view returns (uint256)
function selectWinner() external
function withdrawFees() external
function changeFeeAddress(address newFeeAddress) external
function tokenURI(uint256 tokenId) external view returns (string)
// internal
function _isActivePlayer() internal view returns (bool)
function _baseURI() internal pure returns (string)
```
Events: `RaffleEnter(address[] newPlayers)`, `RaffleRefunded(address player)`, `FeeAddressChanged(address newFeeAddress)`

---
#### Function Summaries
* **constructor** – Sets immutable entrance fee, fee recipient, raffle duration, starts first round, stores URIs & rarity names. 
* **enterRaffle** – Pay `entranceFee * newPlayers.length`, appends players, reverts on duplicates, emits `RaffleEnter`.
* **refund** – Player at `playerIndex` can reclaim entrance fee; slot set to zero to keep indices, emits `RaffleRefunded`.
* **getActivePlayerIndex** – Linear search for `player` in array, returns its index or 0.
* **selectWinner** – Requires duration elapsed & ≥4 players; picks pseudo-random winner & rarity, splits pot 80/20, records fees, mints NFT, resets round.
* **withdrawFees** – Sends accumulated `totalFees` to `feeAddress` only when no active players, resets counter.
* **changeFeeAddress** – Owner-only setter emitting `FeeAddressChanged`.
* **_isActivePlayer** – Internal linear membership check.
* **_baseURI** – Returns constant data URI prefix.
* **tokenURI** – Builds on-chain JSON metadata with rarity, name and IPFS image.

---
#### Storage Variables
* `entranceFee (uint256 immutable)` – Price per entry in wei.
* `players (address[])` – Current round participants; refunded slots set to address(0).
* `raffleDuration (uint256)` – Seconds each round lasts.
* `raffleStartTime (uint256)` – Timestamp when current round began.
* `previousWinner (address)` – Last round’s champion NFT owner.
* `feeAddress (address)` – Where protocol fees are sent; owner changeable.
* `totalFees (uint64)` – Accumulated fees awaiting withdrawal.
* `tokenIdToRarity (mapping(uint256=>uint256))` – Token-id → rarity value.
* `rarityToUri (mapping(uint256=>string))` – Rarity → IPFS image URI.
* `rarityToName (mapping(uint256=>string))` – Rarity → name label used in metadata.
* `commonImageUri (string)` – IPFS link for common pug.
* `COMMON_RARITY (uint256 constant = 70)` – Draw range percentage for common.
* `COMMON (string constant)` – The word "common".
* `rareImageUri (string)` – IPFS for rare St. Bernard.
* `RARE_RARITY (uint256 constant = 25)` – Percentage for rare.
* `RARE (string constant)` – The word "rare".
* `legendaryImageUri (string)` – IPFS for legendary Shiba Inu.
* `LEGENDARY_RARITY (uint256 constant = 5)` – Percentage for legendary.
* `LEGENDARY (string constant)` – The word "legendary".


