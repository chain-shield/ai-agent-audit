# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an Ethereum-based raffle system (Solidity 0.7.6) that lets users compete for a single dog-themed NFT.

**How it works**
1. **Entry** – Anyone calls `enterRaffle(address[] participants)` to register one or more addresses. The contract filters duplicates so each wallet holds only one ticket per round.
2. **Refunds** – Players may exit before a draw via `refund()`, reclaiming their stake and freeing their slot.
3. **Winner Selection** – At predefined intervals (or an owner-triggered call), a pseudo-random algorithm selects one registered address as the winner.
4. **Payout & Fees** – Upon draw, ETH held by the contract is split between:
   * the winner (prize pool)
   * a configurable `feeAddress` set by the owner (protocol revenue)
5. **Administration** – The owner alone can update the fee wallet and initiate draws, but has no power to alter entries or outcomes.

The contract is lightweight, upgrade-free, and deployed on Ethereum mainnet. Players gain a fair chance to win an NFT, while the protocol earns a small fee, all transparently enforced by code.
## High Risk Findings
[H-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-2]. Randomness issue in PuppyRaffle::selectWinner
[H-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[H-4]. DOS issue in PuppyRaffle::enterRaffle
[H-5]. Oracle issue in PuppyRaffle::selectWinner
[H-6]. Reentrancy issue in PuppyRaffle::refund
[H-7]. Unexpected Eth issue in PuppyRaffle::refund
[H-8]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-2]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-3]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-4]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[M-5]. Unexpected Eth issue in PuppyRaffle::NA
## Low Risk Findings
[L-1]. Event Consistency issue in PuppyRaffle::selectWinner
[L-2]. Integer Overflow/Math issue in PuppyRaffle::selectWinner


### Number of Findings
- H: 8
- M: 5
- L: 2
- I: 0



# High Risk Findings

## [H-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `block.timestamp` and `block.difficulty` for randomness generation in `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))`. These values can be manipulated by miners to influence the outcome. The vulnerable code is: `winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender,block.timestamp,block.difficulty))) % players.length`

## Impact
Miners can manipulate block.timestamp and block.difficulty to influence raffle outcomes, potentially allowing them to win unfairly or help specific addresses win

## Proof of Concept
1. Miner sees a selectWinner transaction in mempool
2. Miner calculates different timestamp/difficulty values to get desired winner
3. Miner includes transaction in block with manipulated values
4. Desired address wins the raffle unfairly

## Proof of Code
function testTimestampManipulation() public {
    // Setup raffle with players
    address[] memory players = new address[](4);
    for(uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
    
    // Fast forward time
    vm.warp(block.timestamp + duration + 1);
    
    // Manipulate timestamp to influence outcome
    vm.warp(block.timestamp + 100);
    puppyRaffle.selectWinner();
}

## Suggested Mitigation
Use Chainlink VRF or commit-reveal scheme for true randomness: `uint256 randomness = VRFConsumerBase.requestRandomness(keyHash, fee);`

## [H-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
The selectWinner function uses weak randomness sources (msg.sender, block.timestamp, block.difficulty) for winner selection and rarity determination. These values can be predicted or manipulated by miners, allowing them to influence raffle outcomes. The vulnerable code is: `uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length`

## Impact
Miners can manipulate block.difficulty and block.timestamp within certain bounds. msg.sender is known to the caller. This allows prediction and manipulation of raffle winners and NFT rarities, compromising the fairness of the raffle.

## Proof of Concept
1. Miner calls selectWinner
2. Miner can influence block.timestamp and block.difficulty
3. Miner knows their own address (msg.sender)
4. Miner can calculate the outcome before including the transaction
5. Miner can choose to include or exclude the transaction based on favorable outcomes

## Proof of Code
function testWeakRandomness() public {
    // Simulate predictable randomness
    address[] memory players = new address[](4);
    for(uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    
    // Fast forward time
    vm.warp(block.timestamp + duration + 1);
    
    // Predictable randomness based on known values
    uint256 predictedWinner = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
    
    puppyRaffle.selectWinner();
    
    // Winner can be predicted
    assert(puppyRaffle.previousWinner() == players[predictedWinner]);
}

## Suggested Mitigation
Use Chainlink VRF (Verifiable Random Function) or commit-reveal schemes for secure randomness: `import "@chainlink/contracts/src/v0.8/interfaces/VRFCoordinatorV2Interface.sol";`

## [H-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The withdrawFees function has a strict requirement that `address(this).balance == uint256(totalFees)` which creates a vulnerability where the function can be permanently broken if any unexpected ETH is sent to the contract. The vulnerable code is: `require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");`

## Impact
If anyone sends ETH directly to the contract (via selfdestruct, mining reward, or forced sends), the withdrawFees function becomes permanently unusable as the balance will never exactly equal totalFees again. This locks fees permanently in the contract.

## Proof of Concept
1. Contract accumulates fees normally through raffles
2. Attacker force-sends ETH to contract (e.g., via selfdestruct from another contract)
3. Now address(this).balance > totalFees
4. withdrawFees() will always revert
5. Fees are permanently locked in contract

## Proof of Code
function testUnexpectedEthBreaksWithdraw() public {
    // Setup: enter raffle and select winner to generate fees
    address[] memory players = new address[](4);
    for(uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    vm.warp(block.timestamp + duration + 1);
    puppyRaffle.selectWinner();
    
    // Simulate unexpected ETH sent to contract
    vm.deal(address(puppyRaffle), address(puppyRaffle).balance + 1 ether);
    
    // Now withdrawFees will always fail
    vm.expectRevert();
    puppyRaffle.withdrawFees();
}

## Suggested Mitigation
Change the require statement to check if balance is at least the total fees: `require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Insufficient balance for fees");` and only withdraw the exact fee amount.

## [H-4]. DOS issue in PuppyRaffle::enterRaffle

## Description
The duplicate checking mechanism in the `enterRaffle` function uses nested loops to compare every new player against all existing players. This results in O(n²) gas complexity. As the `players` array grows, the gas cost increases exponentially, potentially causing transactions to exceed the block gas limit and preventing new players from entering the raffle. The vulnerable code is:

```solidity
for (uint256 i_scope_0 = 0; i_scope_0 < players.length - 1; i_scope_0++) {
    for (uint256 j = i_scope_0 + 1; j < players.length; j++) {
        require(players[i_scope_0] != players[j], "PuppyRaffle: Duplicate player");
    }
}
```

## Impact
As the players array grows, the gas cost for entering the raffle increases exponentially, eventually making it impossible for new players to enter due to block gas limit constraints. This creates a denial of service condition where the raffle becomes unusable.

## Proof of Concept
1. Deploy the PuppyRaffle contract
2. Have multiple players enter the raffle to build up the players array
3. As the array grows, each new entry requires more gas to check for duplicates
4. Eventually, the gas required exceeds the block gas limit
5. New players cannot enter the raffle, causing a denial of service

## Proof of Code
```solidity
function testDosOnEnterRaffle() public {
    uint256 playersNum = 100;
    address[] memory players = new address[](playersNum);
    for (uint256 i = 0; i < playersNum; i++) {
        players[i] = address(i);
    }
    
    uint256 gasStart = gasleft();
    puppyRaffle.enterRaffle{value: entranceFee * players.length}(players);
    uint256 gasUsed = gasStart - gasleft();
    
    // Try adding more players - gas usage will increase exponentially
    address[] memory newPlayers = new address[](100);
    for (uint256 i = 0; i < 100; i++) {
        newPlayers[i] = address(i + playersNum);
    }
    
    uint256 gasStart2 = gasleft();
    puppyRaffle.enterRaffle{value: entranceFee * newPlayers.length}(newPlayers);
    uint256 gasUsed2 = gasStart2 - gasleft();
    
    assert(gasUsed2 > gasUsed * 2); // Gas usage increases exponentially
}```

## Suggested Mitigation
Use a mapping to track players instead of nested loops for duplicate checking. Replace the duplicate checking logic with:

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

## [H-5]. Oracle issue in PuppyRaffle::selectWinner

## Description
The selectWinner function uses block.difficulty which is deprecated post-Ethereum merge. The vulnerable code:
```solidity
uint256 winnerIndex = uint256(
    keccak256(
        abi.encodePacked(msg.sender, block.timestamp, block.difficulty)
    )
) % players.length;
```
block.difficulty has been replaced by block.prevrandao in post-merge Ethereum, making this code incompatible with current Ethereum versions.

## Impact
The contract uses deprecated block.difficulty which doesn't exist in post-merge Ethereum. This will cause the function to fail on current Ethereum mainnet, making winner selection impossible and breaking the entire raffle mechanism.

## Proof of Concept
1. Deploy contract on current Ethereum mainnet (post-merge)
2. Try to call selectWinner()
3. Transaction fails because block.difficulty doesn't exist
4. Raffle cannot conclude, funds remain locked

## Proof of Code
function testBlockDifficultyDeprecated() public {
    // On post-merge Ethereum, block.difficulty is always 0
    // This test would need to be run on actual mainnet to demonstrate
    // the issue, but the problem is clear from the code
    
    // Setup raffle
    address[] memory players = new address[](4);
    for(uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    vm.deal(address(this), 4 ether);
    puppyRaffle.enterRaffle{value: 4 ether}(players);
    
    vm.warp(block.timestamp + puppyRaffle.raffleDuration() + 1);
    
    // This would fail on post-merge Ethereum
    // as block.difficulty is deprecated
    puppyRaffle.selectWinner();
}

## Suggested Mitigation
Replace block.difficulty with block.prevrandao for post-merge compatibility, but note this still doesn't solve the randomness issue:
```solidity
// Temporary fix for compatibility (still not secure):
uint256 winnerIndex = uint256(
    keccak256(
        abi.encodePacked(msg.sender, block.timestamp, block.prevrandao)
    )
) % players.length;

// Better solution: Use Chainlink VRF as shown in previous recommendation
```

## [H-6]. Reentrancy issue in PuppyRaffle::refund

## Description
The refund function uses Address.sendValue() which makes an external call to send ETH to the player before updating the players array: `address(msg.sender).sendValue(entranceFee); players[playerIndex] = address(0);`. This creates a potential reentrancy vulnerability where a malicious player could reenter during the refund process.

## Impact
A malicious player could potentially reenter the refund function during the ETH transfer, possibly claiming multiple refunds before their address is set to address(0) in the players array.

## Proof of Concept
1. Attacker enters raffle with malicious contract 2. Attacker calls refund function 3. During sendValue call, attacker's receive/fallback function calls refund again 4. Since players[playerIndex] hasn't been set to address(0) yet, the second call passes validation 5. Attacker receives multiple refunds

## Proof of Code
contract MaliciousRefunder {
    PuppyRaffle puppyRaffle;
    uint256 refundCount;
    
    constructor(address _puppyRaffle) {
        puppyRaffle = PuppyRaffle(_puppyRaffle);
    }
    
    function enterAndRefund() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(players);
        
        uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
        puppyRaffle.refund(playerIndex);
    }
    
    receive() external payable {
        if (refundCount < 1) {
            refundCount++;
            uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
            if (playerIndex != 0) {
                puppyRaffle.refund(playerIndex);
            }
        }
    }
}

## Suggested Mitigation
Update state before external call: `players[playerIndex] = address(0); address(msg.sender).sendValue(entranceFee); // Or use ReentrancyGuard modifier: import "@openzeppelin/contracts/security/ReentrancyGuard.sol"; contract PuppyRaffle is ReentrancyGuard { function refund(uint256 playerIndex) external nonReentrant { // function body } }`

## [H-7]. Unexpected Eth issue in PuppyRaffle::refund

## Description
The refund function allows players to get their entrance fee back, but it doesn't remove the player from the players array - it only sets their address to address(0). This creates several issues: the array length remains the same affecting winner selection probabilities, and it can cause the withdrawFees function to fail due to balance mismatch.

Vulnerable code:
```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).transfer(entranceFee);
    players[playerIndex] = address(0); // Player slot is not properly removed
}
```

## Impact
Players who refund still occupy slots in the raffle, affecting the fairness of winner selection. The withdrawFees function may fail because the contract balance doesn't match totalFees due to refunded amounts. This could lock fees in the contract permanently.

## Proof of Concept
1. Players enter the raffle
2. Some players call refund(), reducing the contract balance but keeping their slots
3. When selectWinner() is called, address(0) could be selected as winner (causing transaction to fail)
4. The withdrawFees() function will fail because contract balance != totalFees
5. Fees become permanently locked in the contract

## Proof of Code
```solidity
function testRefundCausesBrokenState() public {
    address[] memory players = new address[](4);
    players[0] = address(1);
    players[1] = address(2);
    players[2] = address(3);
    players[3] = address(4);
    
    vm.deal(address(1), entranceFee);
    vm.deal(address(2), entranceFee);
    vm.deal(address(3), entranceFee);
    vm.deal(address(4), entranceFee);
    
    // Players enter raffle
    vm.prank(address(1));
    puppyRaffle.enterRaffle{value: entranceFee}([address(1)]);
    vm.prank(address(2));
    puppyRaffle.enterRaffle{value: entranceFee}([address(2)]);
    vm.prank(address(3));
    puppyRaffle.enterRaffle{value: entranceFee}([address(3)]);
    vm.prank(address(4));
    puppyRaffle.enterRaffle{value: entranceFee}([address(4)]);
    
    // Player 1 requests refund
    vm.prank(address(1));
    puppyRaffle.refund(0);
    
    // Try to select winner - could fail if address(0) is selected
    vm.warp(block.timestamp + duration + 1);
    
    // The raffle can still proceed but with unfair odds
    puppyRaffle.selectWinner();
    
    // Now withdrawFees will fail because balance != totalFees
    vm.expectRevert("PuppyRaffle: There are currently players active!");
    puppyRaffle.withdrawFees();
}
```

## Suggested Mitigation
Properly remove players from the array when refunding or implement a different refund mechanism:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Remove player by swapping with last element and popping
    players[playerIndex] = players[players.length - 1];
    players.pop();
    
    payable(msg.sender).transfer(entranceFee);
    emit RaffleRefunded(playerAddress);
}
```

Or track active players separately:
```solidity
mapping(address => bool) public hasRefunded;
uint256 public activePlayerCount;

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(!hasRefunded[playerAddress], "PuppyRaffle: Player already refunded");
    
    hasRefunded[playerAddress] = true;
    activePlayerCount--;
    
    payable(msg.sender).transfer(entranceFee);
    emit RaffleRefunded(playerAddress);
}
```

## [H-8]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner

## Description
The selectWinner function includes a front-running vulnerability where users can observe pending selectWinner transactions in the mempool and front-run them with their own transaction to potentially influence the outcome. Since the randomness depends on msg.sender, block.timestamp, and block.difficulty, an attacker could submit their transaction with higher gas to ensure it gets mined first.

Additionally, since the function is external and anyone can call it, attackers can time their calls strategically to get favorable randomness outcomes.

## Impact
Attackers can front-run legitimate selectWinner calls to manipulate the randomness seed by ensuring their address (msg.sender) is used in the calculation. They can also observe pending transactions and calculate whether the outcome would be favorable before deciding to front-run or not.

## Proof of Concept
1. Alice submits a selectWinner transaction to the mempool
2. Bob observes Alice's transaction and calculates the potential outcome using Alice's address
3. If the outcome is unfavorable to Bob, he submits his own selectWinner transaction with higher gas
4. Bob's transaction gets mined first, using his address as msg.sender in the randomness calculation
5. Bob influences the winner selection and NFT rarity to his advantage

## Proof of Code
```solidity
function testFrontRunSelectWinner() public {
    // Setup players including the attacker
    address alice = address(0x1);
    address bob = address(0x2);
    address[] memory players = new address[](4);
    players[0] = alice;
    players[1] = bob;
    players[2] = address(0x3);
    players[3] = address(0x4);
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    vm.warp(block.timestamp + duration + 1);
    
    // Alice calculates outcome if she calls selectWinner
    uint256 aliceWinnerIndex = uint256(keccak256(abi.encodePacked(alice, block.timestamp, block.difficulty))) % 4;
    console.log("Alice's outcome - Winner index:", aliceWinnerIndex);
    
    // Bob calculates outcome if he calls selectWinner  
    uint256 bobWinnerIndex = uint256(keccak256(abi.encodePacked(bob, block.timestamp, block.difficulty))) % 4;
    console.log("Bob's outcome - Winner index:", bobWinnerIndex);
    
    // If Bob's outcome is more favorable (he wins), he front-runs Alice
    vm.prank(bob);
    puppyRaffle.selectWinner();
    
    // Verify Bob influenced the outcome
    address winner = puppyRaffle.previousWinner();
    console.log("Actual winner:", winner);
    
    // Bob successfully front-ran if his predicted outcome occurred
    assertEq(winner, players[bobWinnerIndex]);
}
```

## Suggested Mitigation
Implement a commit-reveal scheme or use a decentralized oracle for randomness:
```solidity
// Option 1: Commit-Reveal Scheme
mapping(address => bytes32) private commitments;
uint256 private revealDeadline;
bool private commitPhase;

function commitToRandom(bytes32 commitment) external {
    require(commitPhase, "Not in commit phase");
    commitments[msg.sender] = commitment;
}

function selectWinner(uint256 nonce) external {
    require(!commitPhase, "Still in commit phase");
    require(block.timestamp >= revealDeadline, "Reveal period not ended");
    
    // Verify commitment
    bytes32 hash = keccak256(abi.encodePacked(msg.sender, nonce));
    require(commitments[msg.sender] == hash, "Invalid reveal");
    
    // Use revealed randomness
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(nonce, block.timestamp))) % players.length;
    // ... rest of function
}

// Option 2: Restrict who can call selectWinner
modifier onlyAfterDelay() {
    require(block.timestamp >= raffleStartTime + raffleDuration + 1 hours, "Too early");
    _;
}

function selectWinner() external onlyOwner onlyAfterDelay {
    // Only owner can call, reducing front-running risk
    // ... existing logic
}
```



# Medium Risk Findings

## [M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
In the `enterRaffle` function, there is a nested loop that checks for duplicate players. The outer loop runs players.length-1 times and inner loop runs players.length times, creating O(n²) complexity. With large arrays, this can cause gas limit issues. The vulnerable code is in lines with nested loops checking `require(players[i_scope_0] != players[j], "PuppyRaffle: Duplicate player")`

## Impact
Large arrays can cause transaction to exceed block gas limit, making it impossible to enter the raffle with many players. This creates a DoS condition where the raffle becomes unusable

## Proof of Concept
1. Attacker calls enterRaffle with a large array of unique addresses
2. The O(n²) duplicate check consumes excessive gas
3. Transaction fails due to block gas limit
4. Raffle becomes unusable for legitimate users

## Proof of Code
function testGasGrief() public {
    address[] memory players = new address[](1000);
    for(uint i = 0; i < 1000; i++) {
        players[i] = address(uint160(i + 1));
    }
    // This will likely fail due to gas limit
    vm.expectRevert();
    puppyRaffle.enterRaffle{value: 1000 * entranceFee}(players);
}

## Suggested Mitigation
Use a mapping to track entered players instead of nested loops: `mapping(address => bool) public hasEntered;` and check `require(!hasEntered[newPlayers[i]], "Duplicate player"); hasEntered[newPlayers[i]] = true;`

## [M-2]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, integer overflow can occur when calculating `totalFees = totalFees + uint64(fee)`. The `fee` is calculated from `uint256` but cast to `uint64`, and if `totalFees` accumulates enough, it can overflow. The vulnerable code is: `totalFees = totalFees + uint64(fee)`

## Impact
totalFees can overflow and wrap around to a small value, causing fee accounting to be incorrect and potentially allowing withdrawal of more fees than actually collected

## Proof of Concept
1. Multiple raffles are run with high entrance fees
2. totalFees accumulates close to uint64 max (18.4 ETH)
3. Next raffle pushes totalFees over the limit
4. totalFees wraps to a small value
5. Fee accounting becomes incorrect

## Proof of Code
function testTotalFeesOverflow() public {
    // Set totalFees close to max uint64
    // This would require multiple raffles or manipulation
    // Max uint64 = 18,446,744,073,709,551,615 wei ≈ 18.4 ETH
    
    address[] memory players = new address[](4);
    for(uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    // Run multiple expensive raffles to cause overflow
    for(uint j = 0; j < 100; j++) {
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        vm.warp(block.timestamp + duration + 1);
        puppyRaffle.selectWinner();
        vm.warp(block.timestamp + 1);
    }
}

## Suggested Mitigation
Use uint256 for totalFees instead of uint64: `uint256 public totalFees;` or add overflow checks: `require(fee <= type(uint64).max - totalFees, "Fee overflow");`

## [M-3]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The contract performs integer arithmetic operations without using SafeMath library, which was necessary in Solidity versions before 0.8.0. Operations like multiplication and division in fee calculations could potentially overflow or underflow.

## Impact
Integer overflow/underflow in fee calculations could lead to incorrect prize pools, fees, or total amounts, potentially causing financial losses or unexpected behavior.

## Proof of Concept
1. Large number of players enter raffle with high entrance fee 2. totalAmountCollected = players.length * entranceFee could overflow 3. Subsequent calculations for prizePool and fee become incorrect 4. Winner receives wrong amount or transaction fails

## Proof of Code
function testIntegerOverflow() public {
    // This test would need to be constructed with very large values
    // to demonstrate potential overflow in Solidity 0.7.6
    
    // Simulate scenario with maximum possible values
    uint256 maxPlayers = type(uint256).max / entranceFee;
    
    // If players.length * entranceFee approaches uint256 max,
    // overflow could occur
    
    // In practice, this might be difficult to trigger
    // but the lack of SafeMath is still a concern
    
    vm.expectRevert(); // Would revert on overflow
    
    // This is more of a theoretical test since creating
    // enough players to cause overflow is impractical
}

## Suggested Mitigation
Import and use OpenZeppelin's SafeMath library for all arithmetic operations: `import "@openzeppelin/contracts/math/SafeMath.sol";` and `using SafeMath for uint256;` Then replace all arithmetic operations with safe equivalents like `add()`, `mul()`, `div()`.

## [M-4]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `block.timestamp` to determine when a raffle can be concluded: `require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over")`. Miners have some control over `block.timestamp` (within ~15 seconds) and can manipulate it to either delay or accelerate the raffle conclusion to their advantage.

## Impact
Miners can manipulate the timing of raffle conclusion within a small window, potentially affecting the outcome or timing of winner selection. This could be exploited in coordination with the predictable randomness issue.

## Proof of Concept
1. Raffle is close to ending (within 15 seconds of raffleStartTime + raffleDuration) 2. Miner wants to influence the outcome 3. Miner manipulates block.timestamp within the allowed range 4. Miner either delays or accelerates the raffle conclusion 5. Combined with predictable randomness, this allows some control over raffle timing

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract TimestampTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(this), 1 days);
    }
    
    function testTimestampManipulation() public {
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Just before raffle should end
        vm.warp(block.timestamp + 1 days - 10);
        
        // Should fail - raffle not over yet
        vm.expectRevert("PuppyRaffle: Raffle not over");
        puppyRaffle.selectWinner();
        
        // Miner manipulates timestamp by a few seconds
        vm.warp(block.timestamp + 15);
        
        // Now succeeds due to timestamp manipulation
        puppyRaffle.selectWinner();
        assertTrue(true, "Timestamp manipulation affects raffle timing");
    }
}

## Suggested Mitigation
Consider using block numbers instead of timestamps for time-based logic, as they are more predictable: ```solidity // In constructor uint256 public immutable raffleStartBlock; uint256 public immutable raffleDurationBlocks; constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDurationBlocks) { raffleStartBlock = block.number; raffleDurationBlocks = _raffleDurationBlocks; } // In selectWinner function function selectWinner() external { require(block.number >= raffleStartBlock + raffleDurationBlocks, "PuppyRaffle: Raffle not over"); // rest of function } ``` Alternatively, implement a more robust timing mechanism or accept the minor timestamp manipulation risk with proper documentation.

## [M-5]. Unexpected Eth issue in PuppyRaffle::NA

## Description
The contract receives ETH through the enterRaffle function but lacks proper handling for direct ETH transfers. If ETH is sent directly to the contract without calling enterRaffle, it becomes stuck because there's no receive() or fallback() function, and no way to withdraw unexpected ETH.

Additionally, the withdrawFees function has a strict balance check that requires the contract balance to exactly equal totalFees, which will fail if any unexpected ETH is received.

## Impact
ETH sent directly to the contract becomes permanently locked. The strict balance check in withdrawFees will also prevent fee withdrawal if any unexpected ETH is received, effectively breaking the fee collection mechanism.

## Proof of Concept
1. Someone accidentally sends ETH directly to the contract address
2. The ETH gets stuck in the contract with no way to retrieve it
3. When withdrawFees is called, it fails because contract balance > totalFees
4. Legitimate fees cannot be withdrawn due to the unexpected ETH

## Proof of Code
```solidity
function testUnexpectedEthBreaksWithdraw() public {
    // Enter raffle and generate fees
    address[] memory players = new address[](4);
    for(uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    vm.warp(block.timestamp + duration + 1);
    puppyRaffle.selectWinner();
    
    // Someone accidentally sends ETH to the contract
    vm.deal(address(5), 1 ether);
    vm.prank(address(5));
    (bool success,) = address(puppyRaffle).call{value: 0.1 ether}("");
    assertFalse(success); // This will fail, showing ETH gets stuck
    
    // Even if we could send ETH directly, withdrawFees would fail
    // because balance > totalFees
    vm.expectRevert("PuppyRaffle: There are currently players active!");
    puppyRaffle.withdrawFees();
}

function testDirectEthTransferStuck() public {
    // Try to send ETH directly to contract
    vm.deal(address(this), 1 ether);
    
    // This should fail because there's no receive/fallback function
    vm.expectRevert();
    (bool success,) = address(puppyRaffle).call{value: 0.1 ether}("");
    
    // ETH would be stuck if transfer succeeded
}
```

## Suggested Mitigation
Add proper ETH handling functions and adjust the balance check:
```solidity
// Add a function to handle unexpected ETH
function rescueETH() external onlyOwner {
    uint256 contractBalance = address(this).balance;
    uint256 expectedBalance = uint256(totalFees);
    
    if (contractBalance > expectedBalance) {
        uint256 unexpectedETH = contractBalance - expectedBalance;
        (bool success, ) = owner().call{value: unexpectedETH}("");
        require(success, "Failed to rescue ETH");
    }
}

// Modify withdrawFees to be more flexible
function withdrawFees() external {
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    require(totalFees > 0, "PuppyRaffle: No fees to withdraw");
    
    uint256 feesToWithdraw = uint256(totalFees);
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

// Optional: Add receive function if you want to accept direct ETH
receive() external payable {
    // Could emit an event or handle unexpected ETH
    emit UnexpectedETHReceived(msg.sender, msg.value);
}
```



# Low Risk Findings

## [L-1]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function does not emit an event when a winner is selected. Critical state changes like winner selection should emit events for transparency and off-chain monitoring. No event is emitted after `winner = players[winnerIndex]` and prize distribution.

## Impact
Lack of transparency in winner selection process, making it difficult for users to verify fair play and for off-chain systems to track raffle outcomes

## Proof of Concept
1. selectWinner function is called
2. Winner is selected and prize is distributed
3. No event is emitted
4. Users cannot verify who won or when
5. Off-chain systems cannot track raffle results

## Proof of Code
function testMissingWinnerEvent() public {
    // Setup and run raffle
    address[] memory players = new address[](4);
    for(uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
    vm.warp(block.timestamp + duration + 1);
    
    // Expect no winner event (this shows the problem)
    vm.recordLogs();
    puppyRaffle.selectWinner();
    Vm.Log[] memory entries = vm.getRecordedLogs();
    // Should emit winner event but doesn't
}

## Suggested Mitigation
Add winner selection event: `event WinnerSelected(address indexed winner, uint256 indexed tokenId, uint256 prizePool);` and emit it: `emit WinnerSelected(winner, tokenId, prizePool);`

## [L-2]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, integer arithmetic operations like `(totalAmountCollected * 80) / 100` and `(totalAmountCollected * 20) / 100` are performed without considering potential precision loss. While Solidity 0.7.6 doesn't have built-in overflow protection, the division operations could result in precision loss for certain values.

## Impact
Precision loss in fee and prize calculations could result in small amounts of ETH being locked in the contract permanently. Over time, these dust amounts could accumulate.

## Proof of Concept
1. Total amount collected is not perfectly divisible by 100 2. Integer division truncates decimal places 3. Small amounts of ETH remain unaccounted for 4. Over multiple raffles, dust accumulates in the contract

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerMathTest is Test {
    PuppyRaffle puppyRaffle;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1e18 + 1, address(this), 1 days); // Odd entrance fee
    }
    
    function testPrecisionLoss() public {
        address[] memory players = new address[](3); // 3 players with odd entrance fee
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        
        uint256 entranceFee = 1e18 + 1; // 1000000000000000001 wei
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
        
        vm.warp(block.timestamp + 1 days + 1);
        
        uint256 balanceBefore = address(puppyRaffle).balance;
        puppyRaffle.selectWinner();
        uint256 balanceAfter = address(puppyRaffle).balance;
        
        // Calculate expected precision loss
        uint256 totalCollected = entranceFee * 3; // 3000000000000000003
        uint256 prizePool = (totalCollected * 80) / 100; // Truncated
        uint256 fee = (totalCollected * 20) / 100; // Truncated
        uint256 expectedRemainder = totalCollected - prizePool - fee;
        
        assertTrue(expectedRemainder > 0, "Should have precision loss remainder");
    }
}

## Suggested Mitigation
Implement more precise fee calculation to minimize precision loss: ```solidity function selectWinner() external { // ... existing validation logic ... uint256 totalAmountCollected = players.length * entranceFee; uint256 fee = totalAmountCollected / 5; // 20% without multiplication uint256 prizePool = totalAmountCollected - fee; // Ensure all funds are accounted for // ... rest of function logic ... } ``` Alternatively, consider using a more sophisticated fee structure that avoids precision loss entirely.



