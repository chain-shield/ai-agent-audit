# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Status


Finding Status: Valid


[H-1]. Reentrancy in refund allows draining all contract funds
**Derived From** : contract.balance_after == contract.balance_before - entranceFee
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[M-2]. withdrawFees permanently disabled by forced ETH transfer due to strict equality check
**Derived From** : players.length == 0 => withdrawFees() does not revert
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[M-3]. enterRaffle becomes unusable after multiple refunds due to duplicate check on zero addresses
**Derived From** : players[i] != players[j] check allows distinct addresses to enter even if refunds occurred
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[M-4]. DoS via quadratic gas cost in enterRaffle duplicate check
**Derived From** : duplicate check complexity should scale linearly or better
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[H-5]. Incorrect accounting in selectWinner causes revert due to insufficient balance
**Derived From** : totalAmountCollected == count_of_active_players * entranceFee
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[H-6]. Weak Randomness in selectWinner allows winner manipulation
**Derived From** : Winner selection outcome is independent of the caller (msg.sender)
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[M-7]. selectWinner reverts if RNG selects a refunded player
**Derived From** : players[winnerIndex] != address(0)
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood


[H-8]. Integer Overflow in totalFees leads to loss of protocol revenue
**Derived From** : totalFees accumulates monotonically without overflow
Finding Status: LowSeverityDueToRareLikelihood
Finding Complexity: 0
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 4
- M: 4
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Reentrancy in refund allows draining all contract funds

## id: aiLsTBviV1PiTY888PWUb

## Derived From Pattern/Invariant
contract.balance_after == contract.balance_before - entranceFee

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
Classic reentrancy vulnerability with external call before state update. The refund function sends ETH via sendValue before zeroing the player slot, allowing recursive calls. No preconditions needed - any player can exploit by implementing a malicious receive/fallback. Results in total loss of contract funds. This is a textbook CEI violation exploitable by anyone who enters the raffle.

--- Round 2 ---
The refund function violates CEI pattern by calling sendValue before updating players[playerIndex] to address(0). The code path exists: refund() at line 89 sends ETH via sendValue (external call) then updates state at line 91. A malicious contract can re-enter refund() in its receive/fallback since the playerIndex slot is not yet zeroed. The require checks pass again because players[playerIndex] still equals msg.sender. No reentrancy guard (nonReentrant modifier) is present. Solidity 0.7.6 does not have built-in reentrancy protection. This allows draining the contract balance.

--- Round 3 ---
The refund function violates CEI pattern by calling sendValue before updating players[playerIndex] to address(0). An attacker can deploy a contract with a receive/fallback function that recursively calls refund. Since the state update happens after the external call, the require checks pass on each reentry, allowing multiple refunds for a single ticket. This is a classic reentrancy vulnerability exploitable with a simple malicious contract, draining all funds.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refund` function violates the Checks-Effects-Interactions pattern. It performs an external call `payable(msg.sender).sendValue(entranceFee)` before updating the `players` array state (`players[playerIndex] = address(0)`). A malicious player can re-enter the `refund` function via their `receive` or `fallback` function to claim refunds repeatedly for a single ticket until the contract balance is drained.

## Impact
Total loss of contract assets (user funds and accumulated fees).

## Command to Run Test


## Proof of Concept
1. Attacker enters raffle with 1 ticket. 2. Attacker calls `refund`. 3. Contract sends ETH. 4. Attacker's fallback receives ETH and calls `refund` again. 5. Since `players[index]` is not yet zeroed, the check passes and ETH is sent again.

## Proof of Code
contract ReentrancyAttacker { PuppyRaffle raffle; uint256 index; constructor(PuppyRaffle _raffle) { raffle = _raffle; } function attack() external payable { address[] memory p = new address[](1); p[0] = address(this); raffle.enterRaffle{value: 1e18}(p); index = raffle.getActivePlayerIndex(address(this)); raffle.refund(index); } receive() external payable { if (address(raffle).balance >= 1e18) { raffle.refund(index); } } } function testReentrancy() public { ReentrancyAttacker attacker = new ReentrancyAttacker(puppyRaffle); vm.deal(address(attacker), 1e18); attacker.attack(); assertEq(address(puppyRaffle).balance, 0); }

## Suggested Mitigation
Update the state before making the external call. Move `players[playerIndex] = address(0);` before `sendValue`.


## [M-2]. withdrawFees permanently disabled by forced ETH transfer due to strict equality check

## id: 8s-7RWVFRwMbylZFX4OF-

## Derived From Pattern/Invariant
players.length == 0 => withdrawFees() does not revert

## Exploit Type
UnexpectedEth

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
The strict equality check in withdrawFees can be permanently broken by forcing ETH into the contract via selfdestruct. While selfdestruct requires deploying a contract with funds, it's a realistic attack vector. Impact is permanent loss of protocol fee revenue (not user funds). The attack is deterministic once executed but requires some setup and gas costs, making it occasional rather than common.

--- Round 2 ---
The withdrawFees function at line 150 uses strict equality: require(address(this).balance == uint256(totalFees)). An attacker can force ETH into the contract via selfdestruct or by sending ETH to the contract address before deployment (CREATE2 precomputation). Once address(this).balance exceeds totalFees by even 1 wei, the equality check fails permanently. There is no safeguard preventing forced ETH transfers. The invariant that withdrawFees should work when players.length == 0 is violated. This permanently locks protocol fees.

--- Round 3 ---
The withdrawFees function uses strict equality require(address(this).balance == uint256(totalFees)). An attacker can force ETH into the contract via selfdestruct or by sending ETH to the contract address before deployment (CREATE2 precompute). Once balance exceeds totalFees by even 1 wei, withdrawFees permanently reverts. This is exploitable permissionlessly and causes permanent DoS of fee withdrawal functionality, freezing protocol revenue.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdrawFees` function checks `require(address(this).balance == uint256(totalFees))`. If an attacker forces ETH into the contract (e.g., via `selfdestruct`), the contract balance will exceed `totalFees`. This causes the requirement to fail, making it impossible for the owner to withdraw fees.

## Impact
Permanent freezing of protocol fee revenue.

## Command to Run Test


## Proof of Concept
1. Fees accumulate normally. 2. Attacker creates a contract with 1 wei and calls `selfdestruct(puppyRaffle)`. 3. `address(this).balance` becomes `totalFees + 1`. 4. `withdrawFees` reverts.

## Proof of Code
contract SelfDestruct { function kill(address t) external payable { selfdestruct(payable(t)); } } function testUnexpectedEth() public { address[] memory p = new address[](4); p[0]=address(1); p[1]=address(2); p[2]=address(3); p[3]=address(4); puppyRaffle.enterRaffle{value: entranceFee*4}(p); vm.warp(block.timestamp + duration + 1); puppyRaffle.selectWinner(); SelfDestruct s = new SelfDestruct(); s.kill{value: 1}(address(puppyRaffle)); vm.expectRevert("PuppyRaffle: There are currently players active!"); puppyRaffle.withdrawFees(); }

## Suggested Mitigation
Change the check to `require(address(this).balance >= totalFees)`.


## [M-3]. enterRaffle becomes unusable after multiple refunds due to duplicate check on zero addresses

## id: CUc5rVlM8-vh12ESLoetJ

## Derived From Pattern/Invariant
players[i] != players[j] check allows distinct addresses to enter even if refunds occurred

## Exploit Type
StandardViolation

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
After two or more refunds, the duplicate check compares address(0) with address(0) and reverts, permanently bricking new entries. This is a critical DoS that requires only normal user behavior (refunds). No special conditions or attacker resources needed. While it doesn't steal funds, it completely breaks core protocol functionality (entering raffles), justifying High impact. Likelihood is Common as refunds are expected normal operations.

--- Round 2 ---
The enterRaffle duplicate check at lines 79-83 iterates through all players including refunded slots (address(0)). When multiple players refund, the array contains multiple address(0) entries. The check players[i] != players[j] compares address(0) with address(0), which evaluates to false, triggering the revert 'Duplicate player'. The code does not skip address(0) entries. This creates a permanent DoS after two or more refunds occur. No safeguard exists to filter out zero addresses in the duplicate check loop.

--- Round 3 ---
When players refund, their array slot is set to address(0). The duplicate check in enterRaffle compares all players[i] != players[j]. If two or more refunds occur, multiple address(0) entries exist. The check finds address(0) == address(0) and reverts with 'Duplicate player'. This permanently bricks enterRaffle functionality. Easily reproducible: two players enter, both refund, then any new enterRaffle call reverts. This is a critical DoS vulnerability.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function iterates through `players` to check for duplicates. Refunded players are replaced with `address(0)`. If more than one player refunds, the array contains multiple `address(0)` entries. The check `players[i] != players[j]` compares `address(0)` with `address(0)`, evaluates to false, and reverts with 'PuppyRaffle: Duplicate player'. This prevents anyone from entering the raffle.

## Impact
Denial of Service; new players cannot enter.

## Command to Run Test


## Proof of Concept
1. 2 players enter. 2. Both refund (array has two 0x0). 3. Any new user calls `enterRaffle`. 4. Loop finds 0x0 == 0x0 and reverts.

## Proof of Code
function testDosZero() public { address[] memory p = new address[](2); p[0]=address(1); p[1]=address(2); puppyRaffle.enterRaffle{value: entranceFee*2}(p); vm.prank(address(1)); puppyRaffle.refund(0); vm.prank(address(2)); puppyRaffle.refund(1); address[] memory newP = new address[](1); newP[0]=address(3); vm.expectRevert("PuppyRaffle: Duplicate player"); puppyRaffle.enterRaffle{value: entranceFee}(newP); }

## Suggested Mitigation
Skip `address(0)` entries in the duplicate check loop, or use an `EnumerableSet`.


## [M-4]. DoS via quadratic gas cost in enterRaffle duplicate check

## id: TnCszzXY1iHp2EDcjlMXE

## Derived From Pattern/Invariant
duplicate check complexity should scale linearly or better

## Exploit Type
ArrayLimits

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
The O(N²) duplicate check creates gas costs that scale quadratically. An attacker can fill the array to make subsequent entries prohibitively expensive or exceed block gas limits. This requires capital to enter many times (paying entrance fees) and the DoS is temporary (resets each raffle). Impact is Medium as it blocks new entries but doesn't steal funds. Likelihood is Occasional as it requires significant upfront capital and coordination.

--- Round 2 ---
The duplicate check uses nested loops at lines 79-83, creating O(n²) complexity. For each new player batch, the function checks all existing players against each other. With 100 players, this is 10,000 comparisons; with 200 players, 40,000 comparisons. Gas cost grows quadratically. An attacker can fill the array with addresses (using multiple wallets or contract-generated addresses) until subsequent enterRaffle calls exceed block gas limit. No pagination, gas limit checks, or alternative data structure (mapping/EnumerableSet) is used. This is a classic quadratic DoS vulnerability.

--- Round 3 ---
The nested loop duplicate check has O(n²) complexity. As players.length grows, gas costs increase quadratically. An attacker can enter with many addresses (e.g., 100-200 players) making subsequent enterRaffle calls prohibitively expensive or exceeding block gas limit. This is exploitable by anyone with sufficient funds to enter multiple times. While the protocol may intend duplicate checking, the inefficient implementation creates a practical DoS vector.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function uses nested loops to check for duplicates (`O(N^2)`). As the number of players grows, the gas cost increases quadratically. An attacker can fill the array with enough addresses such that subsequent calls to `enterRaffle` exceed the block gas limit, preventing real users from joining.

## Impact
Denial of Service for new entrants.

## Command to Run Test


## Proof of Concept
1. Attacker adds 100 players. 2. Next entry costs X gas. 3. Attacker adds another 100. 4. Entry cost increases quadratically until out of gas.

## Proof of Code
function testQuadratic() public { address[] memory p = new address[](100); for(uint i=0;i<100;i++){p[i]=address(uint160(i+1));} puppyRaffle.enterRaffle{value: entranceFee*100}(p); /* Gas metering would show high usage here */ }

## Suggested Mitigation
Use a mapping or OpenZeppelin's `EnumerableSet` to check for duplicates in O(1) or O(N) time.


## [H-5]. Incorrect accounting in selectWinner causes revert due to insufficient balance

## id: Vlwuy7vEAMgqQTyUA2_Ay

## Derived From Pattern/Invariant
totalAmountCollected == count_of_active_players * entranceFee

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
The selectWinner function calculates prize pool based on players.length rather than actual balance. After refunds (normal user behavior), the calculation exceeds available funds, causing permanent revert and locking all funds. No attacker needed - normal refund operations trigger this. Impact is High as funds become permanently locked. Likelihood is Common because refunds are expected and if >20% of players refund, the raffle bricks.

--- Round 2 ---
The selectWinner function at line 127 calculates totalAmountCollected as players.length * entranceFee. However, refund() returns fees but only zeroes the array slot without reducing players.length. If players refund, the contract balance decreases but totalAmountCollected calculation remains inflated. The prizePool (80%) is calculated on this inflated amount. If more than 20% of players refund, prizePool exceeds actual balance, causing the transfer at line 145 to revert. This permanently bricks the raffle. No safeguard tracks active player count or uses address(this).balance for calculations.

--- Round 3 ---
selectWinner calculates totalAmountCollected as players.length * entranceFee, but refund returns fees while leaving players.length unchanged. If players refund, the contract holds less ETH than calculated. When prizePool (80% of inflated total) exceeds actual balance, the transfer reverts, permanently bricking the raffle. Example: 5 players enter (5 fees), 2 refund (3 fees remain), selectWinner calculates 5 fees and tries to send 4 fees but only 3 exist. Easily exploitable and causes permanent DoS.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function calculates `totalAmountCollected` as `players.length * entranceFee`. While `refund` returns the fee to the user, it only zeroes out the array slot, leaving `players.length` unchanged. If players refund, `totalAmountCollected` includes fees that are no longer in the contract. The prize pool (80%) and fee (20%) are calculated on this inflated amount. If the calculated prize pool exceeds the actual contract balance (which happens if >20% of players refund), the transfer to the winner reverts, permanently bricking the raffle settlement.

## Impact
Permanent Denial of Service of the raffle; funds stuck in contract.

## Command to Run Test


## Proof of Concept
1. 4 players enter (Balance: 4 fees). 2. 1 player refunds (Balance: 3 fees). 3. `selectWinner` is called. 4. `totalAmountCollected` = 4 * fee. `prizePool` = 3.2 * fee. 5. Contract tries to send 3.2 fees but only holds 3 fees. Transaction reverts.

## Proof of Code
function testAccountingRevert() public { address[] memory players = new address[](4); players[0] = address(1); players[1] = address(2); players[2] = address(3); players[3] = address(4); puppyRaffle.enterRaffle{value: entranceFee * 4}(players); vm.prank(address(1)); puppyRaffle.refund(0); vm.warp(block.timestamp + duration + 1); vm.expectRevert(); puppyRaffle.selectWinner(); }

## Suggested Mitigation
Calculate `totalAmountCollected` based on `address(this).balance` or track the count of active players separately.


## [H-6]. Weak Randomness in selectWinner allows winner manipulation

## id: BTSbRH35ZsfdmiBe26vRv

## Derived From Pattern/Invariant
Winner selection outcome is independent of the caller (msg.sender)

## Exploit Type
TimestampManipulation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
The RNG uses predictable on-chain data (msg.sender, timestamp, difficulty). An attacker can compute the outcome off-chain and only call selectWinner when they win. This allows theft of the entire prize pool. Impact is High due to direct fund theft. Likelihood is Occasional because it requires: (1) attacker must be a player, (2) timing the call correctly, (3) some computational resources to simulate outcomes, and (4) gas costs for failed attempts.

--- Round 2 ---
The RNG at line 126 uses keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty)). All inputs are predictable: msg.sender is the caller's address, block.timestamp is known before transaction, block.difficulty is public. A malicious contract can simulate the RNG off-chain, calculate the winner index, and only call selectWinner when they win. Miners can manipulate block.timestamp within bounds. No Chainlink VRF or commit-reveal scheme is used. This violates the invariant that winner selection should be independent of the caller. The vulnerability is real and exploitable.

--- Round 3 ---
The RNG uses keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty)), all of which are predictable or manipulatable. A malicious contract can simulate the RNG off-chain and only call selectWinner when it wins. Miners can manipulate block.timestamp and block.difficulty. This allows attackers to guarantee winning or obtaining rare NFTs. The documentation mentions using on-chain data for randomness but does not indicate this weakness is intentional. Exploitable with a simple smart contract wrapper.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The random number generation uses `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))`. All these values are predictable or manipulatable by miners or the calling contract. A malicious user can calculate the winner index off-chain and only call `selectWinner` when the result is favorable to them (i.e., they win or get a Rare/Legendary NFT).

## Impact
Theft of prize pool and unfair distribution of high-value NFTs.

## Command to Run Test


## Proof of Concept
1. Attacker writes a contract that simulates the RNG using current block variables. 2. The contract calls `selectWinner` only if the calculated winner index matches the attacker's index. 3. Attacker guarantees a win.

## Proof of Code
function testWeakRandomness() public { /* Concept: simulate keccak256 in test loop until favorable block values found */ }

## Suggested Mitigation
Use Chainlink VRF or a commit-reveal scheme for true randomness.


## [M-7]. selectWinner reverts if RNG selects a refunded player

## id: bIwKQHwaYqbsPwuin17dz

## Derived From Pattern/Invariant
players[winnerIndex] != address(0)

## Exploit Type
StandardViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: --- Round 1 ---
If the RNG selects an index with address(0) (refunded player), _safeMint reverts as ERC721 prohibits minting to zero address. This causes DoS of raffle settlement. Impact is Medium as it blocks raffle completion but funds aren't permanently lost (can retry or manipulate RNG). Likelihood is Occasional because it depends on: (1) at least one refund occurring, (2) RNG randomly selecting that specific index, making it probabilistic rather than guaranteed.

--- Round 2 ---
The selectWinner function at line 126 selects winnerIndex from the full players array length, including refunded (address(0)) slots. If the RNG selects a refunded index, winner becomes address(0). The _safeMint call at line 147 reverts because ERC721 prohibits minting to address(0) (OpenZeppelin 3.4.0 implementation checks this). The code does not validate winner != address(0) or skip zero addresses when selecting. This causes a DoS where selectWinner cannot complete until RNG happens to pick a non-zero index. No safeguard exists.

--- Round 3 ---
selectWinner picks winnerIndex from players array without checking if players[winnerIndex] is address(0). If a refunded slot is selected, winner becomes address(0) and _safeMint reverts (ERC721 prohibits minting to zero address). This causes DoS of raffle settlement. The probability increases with more refunds. While the RNG may eventually select a valid player, this creates unpredictable failures and potential griefing. Exploitable by refunding and waiting for unlucky RNG outcome.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function selects a winner index from the `players` array. If the selected index corresponds to a player who has refunded (slot is `address(0)`), the variable `winner` becomes `address(0)`. The subsequent call to `_safeMint(winner, ...)` reverts because ERC721 does not allow minting to the zero address.

## Impact
Denial of Service of the raffle completion. The raffle cannot be settled until the RNG happens to pick a non-zero index (which might be manipulated or take many blocks).

## Command to Run Test


## Proof of Concept
1. Player enters. 2. Player refunds. 3. `selectWinner` is called. 4. If `winnerIndex` points to the refunded slot, transaction reverts.

## Proof of Code
function testWinnerIsZero() public { address[] memory players = new address[](4); players[0]=address(this); players[1]=address(1); players[2]=address(2); players[3]=address(3); puppyRaffle.enterRaffle{value: entranceFee*4}(players); puppyRaffle.refund(0); vm.warp(block.timestamp + duration + 1); /* Assume RNG hits index 0 */ vm.expectRevert("ERC721: mint to the zero address"); puppyRaffle.selectWinner(); }

## Suggested Mitigation
In `selectWinner`, check if `winner == address(0)`. If so, select the next available index or skip/retry.





Finding Status: LowSeverityDueToRareLikelihood
## [H-8]. Integer Overflow in totalFees leads to loss of protocol revenue

## id: QN3i9NX0i5tZr42usjGoc

## Derived From Pattern/Invariant
totalFees accumulates monotonically without overflow

## Exploit Type
IntegerOverflow

## Location
PuppyRaffle.selectWinner

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: --- Round 1 ---
Solidity 0.7.6 lacks overflow protection and totalFees is uint64 (max ~18.4 ETH). Overflow causes fee counter to wrap, losing protocol revenue and potentially bricking withdrawFees. Impact is Medium as it affects protocol fees, not user prize funds directly. Likelihood is Rare because it requires accumulating 18+ ETH in fees across multiple raffles, which may take significant time and volume depending on entrance fee and participation rates.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The contract uses Solidity 0.7.6, which does not check for arithmetic overflow by default. `totalFees` is a `uint64`. In `selectWinner`, the line `totalFees = totalFees + uint64(fee);` can overflow if the accumulated fees exceed `type(uint64).max` (~18.4 ETH). This resets the fee counter, causing a loss of revenue for the protocol.

## Impact
Loss of protocol fees; potentially bricks `withdrawFees` due to balance mismatch.

## Command to Run Test


## Proof of Concept
1. Accumulate fees close to uint64 max. 2. Call `selectWinner`. 3. `totalFees` wraps around to a small value.

## Proof of Code
function testOverflow() public { address[] memory players = new address[](4); players[0] = address(1); players[1] = address(2); players[2] = address(3); players[3] = address(4); puppyRaffle.enterRaffle{value: entranceFee * 4}(players); vm.warp(block.timestamp + duration + 1); stdstore.target(address(puppyRaffle)).sig("totalFees()").checked_write(type(uint64).max); puppyRaffle.selectWinner(); assertLt(puppyRaffle.totalFees(), type(uint64).max); }

## Suggested Mitigation
Use `SafeMath` for addition or upgrade to Solidity ^0.8.0. Ideally change `totalFees` to `uint256`.



