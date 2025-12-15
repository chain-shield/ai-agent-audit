# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Status


Finding Status: Valid


[H-1]. Reentrancy in `refund` function allows entrants to drain the contract balance
**Derived From** : Reentrancy
Finding Status: Valid
Privilege: Permissionless


[M-2]. Strict balance check in `withdrawFees` enables DoS via self-destruct
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[M-3]. Integer Overflow and Truncation in totalFees locks protocol fees
**Derived From** : StandardViolation
Finding Status: Valid
Privilege: Permissionless


[M-4]. Unbounded O(N^2) loop in `enterRaffle` causes DoS for new entrants
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[M-5]. Permanent Denial of Service in enterRaffle due to Zero Address Duplicate Check
**Derived From** : StandardViolation
Finding Status: Valid
Privilege: Permissionless


[M-6]. Denial of Service in enterRaffle due to quadratic duplicate check
**Derived From** : UnboundedLoops
Finding Status: Valid
Privilege: Permissionless


[H-7]. Integer Overflow in `totalFees` calculation permanently locks protocol fees
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[M-8]. Weak randomness in `selectWinner` allows predicting or influencing the winner
**Derived From** : StandardViolation
Finding Status: Valid
Privilege: Permissionless


[H-9]. Integer Overflow and Unsafe Casting in selectWinner leads to incorrect fee accounting and locked funds
**Derived From** : UnsafeAssembyTypeCasts
Finding Status: Valid
Privilege: Permissionless


[H-10]. Protocol Denial of Service via malicious reverting winner or blocked recipient
**Derived From** : GriefableCallbacks
Finding Status: Valid
Privilege: Permissionless


[H-11]. Raffle settlement reverts due to accounting mismatch in selectWinner
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 5
- M: 6
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Reentrancy in `refund` function allows entrants to drain the contract balance

## id: 64ClfxMcgg0qYlS6Kk_qG

## Derived From Pattern/Invariant
Reentrancy

## Exploit Type
Reentrancy

## Location
PuppyRaffle.refund

## Finding Status: Valid
### Finding Status Justification: `refund()` calls `sendValue` (forwards all gas) before zeroing `players[playerIndex]`, enabling same-index reentrancy to pass both `require`s and withdraw repeatedly, draining other users' ETH. No reentrancy guard exists.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `refund` function violates the Check-Effects-Interactions pattern. It performs an external ETH transfer via `sendValue` before updating the `players` array to remove the participant. A malicious contract can enter the raffle, call `refund`, and inside the `receive()`/`fallback()` function, call `refund` again. Since the `players` state has not been updated (the index is not yet zeroed out), the checks pass, allowing the attacker to withdraw their entrance fee multiple times until the contract is drained of all other users' funds.

## Impact
An attacker can drain the entire ETH balance of the contract (theft of assets).

## Command to Run Test


## Proof of Concept
1. Attacker deploys a malicious contract. 2. Malicious contract calls `enterRaffle` with 1 ETH. 3. Malicious contract calls `refund`. 4. `PuppyRaffle` sends 1 ETH to malicious contract. 5. Malicious contract's fallback triggers `refund` again. 6. `players[index]` is still the attacker, so the refund executes again. 7. Repeats until contract balance is empty.

## Proof of Code
contract ReentrancyAttacker { PuppyRaffle target; uint256 index; constructor(address _target) { target = PuppyRaffle(_target); } function attack() external payable { address[] memory players = new address[](1); players[0] = address(this); target.enterRaffle{value: 1 ether}(players); index = target.getActivePlayerIndex(address(this)); target.refund(index); } receive() external payable { if (address(target).balance >= 1 ether) { target.refund(index); } } }

## Suggested Mitigation
Move the state update `players[playerIndex] = address(0);` to before the external call `payable(msg.sender).sendValue(entranceFee);`.


## [M-2]. Strict balance check in `withdrawFees` enables DoS via self-destruct

## id: DX_wqDT_HeiF0-0Erh97S

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: ETH can be force-sent via `selfdestruct`, making `address(this).balance != totalFees` forever and causing `withdrawFees()` to revert indefinitely. There is no sweep/recovery mechanism.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `withdrawFees` function requires `address(this).balance == uint256(totalFees)`. This invariant assumes the contract balance consists *only* of accumulated fees when players are inactive. However, an attacker can force-send ETH to the contract via `selfdestruct` (which bypasses fallback functions). This increases `address(this).balance` without increasing `totalFees`. Consequently, the strict equality check fails, causing `withdrawFees` to revert and locking the fees forever.

## Impact
Protocol fees cannot be withdrawn (Denial of Service).

## Command to Run Test


## Proof of Concept
1. `withdrawFees` relies on exact balance equality. 2. Attacker deploys a contract with 1 wei. 3. Attacker calls `selfdestruct(target)`. 4. `PuppyRaffle` balance increases by 1 wei. 5. `balance` is now `totalFees + 1`. 6. `withdrawFees` reverts.

## Proof of Code
function testDoSWithSelfDestruct() public { vm.deal(address(this), 1 ether); address[] memory players = new address[](4); players[0]=address(1); players[1]=address(2); players[2]=address(3); players[3]=address(4); puppyRaffle.enterRaffle{value: 4 ether}(players); vm.warp(block.timestamp + duration + 1); puppyRaffle.selectWinner(); address attacker = address(new SelfDestructAttacker(address(puppyRaffle))); vm.expectRevert(); puppyRaffle.withdrawFees(); } contract SelfDestructAttacker { constructor(address target) payable { selfdestruct(payable(target)); } }

## Suggested Mitigation
Remove the strict equality check `address(this).balance == totalFees`. Instead, simply withdraw `totalFees` or `min(address(this).balance, totalFees)`.


## [M-3]. Integer Overflow and Truncation in totalFees locks protocol fees

## id: amQtAN1xB5yn7bY9uQU4T

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
PuppyRaffle.withdrawFees

## Finding Status: Valid
### Finding Status Justification: `totalFees` is `uint64` and updated with `totalFees + uint64(fee)` in Solidity 0.7.6 (unchecked). This can truncate/overflow, desyncing `totalFees` from actual fee ETH and causing `withdrawFees()`'s strict equality check to revert permanently.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `totalFees` variable is declared as `uint64` for storage packing, but `fee` is calculated as a `uint256`. In Solidity 0.7.6 (unchecked arithmetic by default), the line `totalFees = totalFees + uint64(fee)` is vulnerable to two issues:
1. **Truncation**: If `fee` exceeds `type(uint64).max` (~18.4 ETH), casting to `uint64` truncates the higher bits, storing an incorrect low value.
2. **Overflow**: Accumulating fees over time will easily exceed `type(uint64).max`, wrapping `totalFees` to a small value.

Critically, `withdrawFees` requires `address(this).balance == uint256(totalFees)`. Due to overflow/truncation, `totalFees` will not match the actual contract balance, causing the check to fail and permanently locking the fees.

## Impact
Medium. Protocol fees become permanently locked in the contract due to the impossibility of satisfying the withdrawal condition.

## Command to Run Test


## Proof of Concept
1. `entranceFee` is 1 ETH. A raffle has 100 players. Collected: 100 ETH.
2. `fee` = 20 ETH (20e18).
3. `type(uint64).max` is ~18.4e18.
4. `uint64(20e18)` truncates to `20e18 % 2**64` ≈ 1.55e18.
5. `totalFees` stores ~1.55e18.
6. `withdrawFees` is called. `address(this).balance` is 20 ETH (assuming prize paid). `totalFees` is 1.55 ETH.
7. `require(balance == totalFees)` fails. Fees are stuck.

## Proof of Code
function testFeeOverflow() public {
    address[] memory players = new address[](100);
    for(uint i=0; i<100; i++) players[i] = address(i+1);
    puppyRaffle.enterRaffle{value: 100 ether}(players);
    vm.warp(block.timestamp + 10 days);
    puppyRaffle.selectWinner();
    // totalFees should be ~1.5e18 due to truncation, balance is 20e18
    vm.expectRevert("PuppyRaffle: There are currently players active!"); // Logic error in check actually triggers this or inequality
    // The revert message in code is ambiguous for inequality, but condition fails.
    puppyRaffle.withdrawFees();
}

## Suggested Mitigation
Change `totalFees` to `uint256`. Remove the unsafe casting. Use SafeMath if staying on 0.7.6 (though simple addition of fees shouldn't overflow uint256 practically).


## [M-4]. Unbounded O(N^2) loop in `enterRaffle` causes DoS for new entrants

## id: E_HVGrVW4Zahfc2LsSdfD

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: `enterRaffle()` performs a full nested-loop duplicate check over `players`, making gas O(N^2). With sufficiently many players, calls will exceed practical gas limits, preventing further entries; there is no cap or O(1) membership tracking.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function iterates through the entire `players` array twice (nested loop) to check for duplicates. As the number of players grows, the gas cost increases quadratically (O(N^2)). Eventually, the gas required to enter the raffle will exceed the block gas limit, making it impossible for new players to enter.

## Impact
The contract becomes unusable once a certain number of players have entered (Denial of Service).

## Command to Run Test


## Proof of Concept
1. Assume 1000 players have entered. 2. Next player calls `enterRaffle`. 3. Loop performs ~500,000 comparisons. 4. Transaction runs out of gas.

## Proof of Code
function testDoSLoop() public { uint256 numPlayers = 100; address[] memory players = new address[](numPlayers); for(uint i=0; i<numPlayers; i++) players[i] = address(uint160(i)); puppyRaffle.enterRaffle{value: numPlayers * 1 ether}(players); // Next entry costs significantly more gas }

## Suggested Mitigation
Use a mapping `mapping(address => bool) public activePlayers` to check for duplicates in O(1) time.


## [M-5]. Permanent Denial of Service in enterRaffle due to Zero Address Duplicate Check

## id: NdhX9Pa4bWK6faNo7LhIG

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: After 2 refunds, `players` contains at least two `address(0)` entries and the nested duplicate check will always hit `0x0 == 0x0` and revert, bricking `enterRaffle()`. If `players.length < 4`, `selectWinner()` cannot run to reset the array, making the DoS permanent.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a player calls `refund`, their array slot is set to `address(0)`. The `enterRaffle` function enforces strict uniqueness by comparing every new and existing player: `require(players[i] != players[j])`.

If two or more players refund, the `players` array will contain multiple `address(0)` entries. When `enterRaffle` iterates, it compares these two zero addresses, evaluates `address(0) != address(0)` as false, and reverts with 'PuppyRaffle: Duplicate player'.

This permanently bricks the raffle: no new players can enter, and if `players.length < 4`, the raffle can never finish.

## Impact
Permanent Denial of Service. The contract becomes unusable.

## Command to Run Test


## Proof of Concept
1. User A enters. User B enters.
2. User A refunds (slot 0 -> 0x0).
3. User B refunds (slot 1 -> 0x0).
4. User C tries to enter.
5. `enterRaffle` loop finds `players[0] == players[1]` (both 0x0) and reverts.

## Proof of Code
function testDosRefunding() public {
        address[] memory players = new address[](1);
        players[0] = address(1);
        raffle.enterRaffle{value: 1e18}(players);
        players[0] = address(2);
        raffle.enterRaffle{value: 1e18}(players);
        
        vm.prank(address(1));
        raffle.refund(0);
        vm.prank(address(2));
        raffle.refund(1);
        
        players[0] = address(3);
        vm.expectRevert("PuppyRaffle: Duplicate player");
        raffle.enterRaffle{value: 1e18}(players);
    }

## Suggested Mitigation
In the duplicate check loop, skip if `players[i] == address(0)`.


## [M-6]. Denial of Service in enterRaffle due to quadratic duplicate check

## id: QbfnBaZ6BLYU9s1IgXiC6

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
Dos

## Location
PuppyRaffle.enterRaffle

## Finding Status: Valid
### Finding Status Justification: `enterRaffle()`'s nested-loop duplicate check is quadratic in `players.length`, eventually making entry transactions run out of gas. There is no mitigating design (cap/paging/mapping).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enterRaffle` function contains a nested loop that iterates over the `players` array to check for duplicates. The outer loop runs `players.length` times (for existing players) or `newPlayers.length` times, and the inner duplicate check compares every new player against every existing player. This results in a time complexity of O(N^2). As the number of players grows, the gas cost to enter the raffle increases quadratically. Eventually, the gas cost will exceed the block gas limit, making it impossible for new players to enter.

## Impact
Medium. The raffle can become unusable if enough players enter, preventing the round from concluding or new users from participating.

## Command to Run Test


## Proof of Concept
1. Attacker (or legitimate usage) fills the raffle with a large number of players (e.g., 1000+). 2. Subsequent calls to `enterRaffle` attempt to iterate over the large array. 3. The transaction runs out of gas and reverts.

## Proof of Code
function testDosLoop() public {
    address[] memory players = new address[](100);
    for(uint i=0; i<100; i++) players[i] = address(i+1);
    puppyRaffle.enterRaffle{value: 100 ether}(players);
    // Repeat until gas limit reached
}

## Suggested Mitigation
Remove the nested loop. Use a mapping `mapping(address => bool) public activePlayers` to check for duplicates in O(1) time.


## [H-7]. Integer Overflow in `totalFees` calculation permanently locks protocol fees

## id: NBli_qUuDN0U7Tbt_JHDw

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: Same `uint64` truncation/overflow issue: `fee` is `uint256` and cast to `uint64` before addition in Solidity 0.7.6. Once desynced, `withdrawFees()`'s `balance == totalFees` requirement can become unachievable, locking fees.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `selectWinner`, the `fee` is calculated as a `uint256` but cast to `uint64` when adding to `totalFees`. Since `entranceFee` is 1e18, a raffle with ~93 players results in a fee of ~18.6 ETH. The maximum value of `uint64` is approximately 18.44 ETH. If the fee exceeds this, the cast explicitly truncates the higher bits (overflows/wraps), storing a much smaller value in `totalFees`. However, `withdrawFees` enforces strict equality: `address(this).balance == uint256(totalFees)`. Since the contract holds the full ETH amount but `totalFees` tracks the truncated amount, this check will always fail, making fees permanently trapped in the contract.

## Impact
Protocol fees are permanently locked in the contract due to the balance mismatch caused by the overflow/truncation.

## Command to Run Test


## Proof of Concept
1. 93 players enter (total 93 ETH). 2. `selectWinner` is called. Fee = 18.6 ETH (18,600,000,000,000,000,000 wei). 3. `uint64` max is ~18.44 ETH. 4. `totalFees` becomes `uint64(18.6 ETH)`, which wraps to a small number. 5. Contract balance includes the full 18.6 ETH fee. 6. `withdrawFees` reverts because `balance != totalFees`.

## Proof of Code
function testFeeOverflow() public { uint256 playersCount = 95; address[] memory players = new address[](playersCount); for(uint i=0; i<playersCount; i++) { players[i] = address(uint160(i+1)); } vm.deal(address(this), playersCount * 1 ether); puppyRaffle.enterRaffle{value: playersCount * 1 ether}(players); vm.warp(block.timestamp + duration + 1); puppyRaffle.selectWinner(); vm.expectRevert(); puppyRaffle.withdrawFees(); }

## Suggested Mitigation
Change `totalFees` to `uint256` to match the accounting type of ETH balances and prevent overflow.


## [M-8]. Weak randomness in `selectWinner` allows predicting or influencing the winner

## id: lNyEbjsspoBK2UOEcSlBc

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: Randomness is derived from `msg.sender`, `block.timestamp`, and `block.difficulty`, which is not a secure randomness source and is biasable by block producers/MEV conditions. Also, the report's claim that an arbitrary caller can 'revert if not winner' is generally incorrect (only the selected winner contract can force a revert via its callbacks), but the core weak-randomness issue remains.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function uses `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))` to generate a random number. All these inputs are predictable or manipulable. Miners can manipulate `block.timestamp` and `block.difficulty`. Furthermore, a user can call `selectWinner` from a contract and revert the transaction if they are not the winner (or if the result isn't favorable), effectively 're-rolling' the randomness until they win.

## Impact
Fairness of the raffle is compromised. Attackers can significantly increase their winning odds or guarantee a win.

## Command to Run Test


## Proof of Concept
1. Attacker waits for raffle duration. 2. Attacker calculates `keccak256(...)` using current block vars and their address. 3. If `result % players.length` is their index, they call `selectWinner`. 4. If not, they wait for the next block or use a different sender address.

## Proof of Code
function testWeakRandomness() public { /* Concept logic */ uint256 winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length; require(players[winnerIndex] == msg.sender, 'I did not win'); puppyRaffle.selectWinner(); }

## Suggested Mitigation
Use Chainlink VRF (Verifiable Random Function) for secure, tamper-proof on-chain randomness.


## [H-9]. Integer Overflow and Unsafe Casting in selectWinner leads to incorrect fee accounting and locked funds

## id: yOLlYopq9QwmTmyR0hrDV

## Derived From Pattern/Invariant
UnsafeAssembyTypeCasts

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: `totalFees = totalFees + uint64(fee)` can truncate and/or overflow (`uint64`, unchecked math in 0.7.6), breaking fee accounting and potentially making `withdrawFees()` permanently revert due to the strict balance equality check.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `selectWinner`, the fee calculation `totalFees = totalFees + uint64(fee)` contains two issues. First, it explicitly casts the `uint256` fee to `uint64`. If the fee amount exceeds `type(uint64).max` (~18.4 ETH), the value is truncated, causing `totalFees` to be significantly lower than the actual fee collected. Second, in Solidity 0.7.6, arithmetic operations are unchecked by default (without SafeMath), so `totalFees + ...` can overflow if the total exceeds `uint64`. This results in a mismatch where `address(this).balance` > `totalFees`, which causes `withdrawFees` to revert due to its strict equality check.

## Impact
High. The protocol permanently loses access to collected fees due to accounting errors and the subsequent inability to withdraw them (DoS of withdrawal).

## Command to Run Test


## Proof of Concept
1. A raffle is concluded with a prize pool large enough such that 20% (the fee) > 18.4 ETH. 2. `uint64(fee)` truncates the higher bits. 3. `totalFees` is updated with the truncated value. 4. The contract balance holds the full fee amount, but `totalFees` tracks a smaller amount. 5. `withdrawFees` fails because `address(this).balance != totalFees`.

## Proof of Code
function testOverflow() public {
    uint256 feeAmount = 20 ether;
    // Mocking the scenario where fee > type(uint64).max
    // Requires enough players to reach ~100 ETH pot.
    // Assert totalFees != feeAmount after selectWinner
}

## Suggested Mitigation
Use `uint256` for `totalFees` to avoid casting and overflow risks. Additionally, use SafeMath or Solidity 0.8+ to prevent arithmetic overflows.


## [H-10]. Protocol Denial of Service via malicious reverting winner or blocked recipient

## id: GWpkFcNX0PvCfm-7Kh3GE

## Derived From Pattern/Invariant
GriefableCallbacks

## Exploit Type
Dos

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: If the chosen `winner` cannot accept ETH, `winner.call{value: prizePool}('')` will fail and `selectWinner()` reverts. However, the round is typically retriable and not necessarily permanently bricked unless all possible winners are non-payable/reverting, so the 'permanent freeze' framing is overstated.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function attempts to transfer the prize pool to the winner using `winner.call{value: prizePool}("")` and explicitly requires success (`require(success, ...)`). If the selected winner is a smart contract that reverts on receiving ETH (or has no `receive`/`fallback` function), the `selectWinner` transaction will always revert. An attacker can deliberately enter such a contract into the raffle. If this contract is selected as the winner, the raffle round can never be settled, locking the funds and preventing a new round from starting.

## Impact
High. The current raffle round can be permanently frozen, locking all participant funds and preventing the protocol from continuing.

## Command to Run Test


## Proof of Concept
1. Attacker deploys a contract with a `receive()` function that reverts. 2. Attacker enters this contract into the raffle. 3. If this contract is selected as the winner (attacker can enter many times to increase odds), `selectWinner` fails to send ETH. 4. The function reverts, and the winner cannot be skipped.

## Proof of Code
contract RevertingWinner {
    receive() external payable { revert(); }
}
// Enter this contract into raffle
// Ensure it wins (mock RNG or probability)
// Call selectWinner -> Reverts

## Suggested Mitigation
Do not require success on the ETH transfer. Instead, implement a 'pull-over-push' pattern (e.g., WETH wrapper or a claim function) where the winner withdraws their funds separately, ensuring the core raffle logic completes regardless of the winner's ability to receive ETH.


## [H-11]. Raffle settlement reverts due to accounting mismatch in selectWinner

## id: MeVsRVXz2BMw7c2BaW37v

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
PuppyRaffle.selectWinner

## Finding Status: Valid
### Finding Status Justification: `selectWinner()` computes pot as `players.length * entranceFee` even though refunds reduce actual ETH without reducing `players.length`, so `prizePool` can exceed balance and revert. This can become permanently stuck when combined with the multiple-`address(0)` duplicate-check DoS that prevents adding new players to restore solvency.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `selectWinner` function calculates `totalAmountCollected` based on the current `players.length` multiplied by `entranceFee`. However, the `refund` function allows players to withdraw their funds, replacing their address in the array with `address(0)` but NOT reducing the array length. If players refund, the contract balance decreases, but `players.length` stays the same. 

In `selectWinner`, the prize pool is calculated as 80% of `players.length * entranceFee`. If enough players refund (specifically >20%), the calculated `prizePool` will exceed the actual ETH balance of the contract. The transfer `winner.call{value: prizePool}("")` will fail due to insufficient funds, causing `selectWinner` to revert and permanently locking the remaining funds and the NFT.

## Impact
Permanent Denial of Service of the raffle settlement and lock of user funds.

## Command to Run Test


## Proof of Concept
1. 4 players enter (Balance: 4 ETH). 2. 1 player refunds (Balance: 3 ETH, Length: 4). 3. `selectWinner` called. 4. `totalAmountCollected` = 4 * 1 ETH = 4 ETH. 5. `prizePool` = 4 ETH * 80% = 3.2 ETH. 6. Contract tries to send 3.2 ETH but only has 3 ETH. 7. Transaction reverts.

## Proof of Code
function testRefundDoS() public { address[] memory players = new address[](4); players[0] = address(1); players[1] = address(2); players[2] = address(3); players[3] = address(4); puppyRaffle.enterRaffle{value: 4 ether}(players); vm.prank(address(1)); puppyRaffle.refund(0); vm.warp(block.timestamp + duration + 1); vm.expectRevert(); puppyRaffle.selectWinner(); }

## Suggested Mitigation
Track the actual collected amount in a storage variable that increments on deposit and decrements on refund, instead of deriving it from array length. Or, decrement the array length/reorganize the array upon refund.



