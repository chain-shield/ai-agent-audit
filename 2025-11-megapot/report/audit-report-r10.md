# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[M-1]. Bit-packing overflow in TicketComboTracker leads to unclaimable winning tickets
**Derived From** : drawingState[id].ballMax + drawingState[id].bonusballMax <= 255
Finding Status: Valid
Privilege: Permissionless


[H-2]. Incorrect Ticket Matching and Potential Theft of Winnings due to Bitwise Shift Overflow
**Derived From** : Large ticket buyer covering many combos
Finding Status: Valid
Privilege: Permissionless


[H-3]. Insolvency due to execution of scaledEntropyCallback during Emergency Mode
**Derived From** : Jackpot.scaledEntropyCallback: !emergencyMode
Finding Status: Valid
Privilege: Permissionless


[H-4]. Bitpacking overflow in TicketComboTracker allows manipulation of winning odds
**Derived From** : TicketComboTracker.insert: normalBallMax + bonusballMax < 256
Finding Status: Valid
Privilege: Permissionless


[H-5]. Insolvency due to phantom LP earnings after emergency refunds
**Derived From** : Balance
Finding Status: Valid
Privilege: Permissionless


[H-6]. Emergency refunds cause permanent LP value loss due to settlement accounting mismatch
**Derived From** : Jackpot.emergencyRefundTickets: Balance Invariant
Finding Status: Valid
Privilege: RequiresRole


[H-7]. Unsafe downcast of `bonusballMax` breaks LP edge or causes DoS
**Derived From** : Arithmetic
Finding Status: Valid
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-8]. DoS of `runJackpot` due to gas limit exceeding block maximums
**Derived From** : Jackpot contract owner
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[M-9]. Permanent DoS via Zero Accumulator
**Derived From** : Arithmetic
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole



Finding Status: InvalidGovernanceRisk


[M-10]. Phantom Winners Stranding Funds after Emergency Refund Resumption
**Derived From** : Emergency-mode ticket refunder
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless


[M-11]. Bit shift overflow in `TicketComboTracker` corrupts tickets
**Derived From** : PricePrecision
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole



Finding Status: LowSeverityDueToRareLikelihood


[M-12]. Bitpacking overflow allows gaining unfair winning odds via 'phantom' bonus balls
**Derived From** : Jackpot._setNewDrawingState: newBonusballMax <= 255 - normalBallMax
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 6
- M: 6
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. Bit-packing overflow in TicketComboTracker leads to unclaimable winning tickets

## id: As32ZSuwSBN1_FEiX3I9l

## Derived From Pattern/Invariant
drawingState[id].ballMax + drawingState[id].bonusballMax <= 255

## Exploit Type
IntegerOverflow

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: There is no on-chain invariant enforcing normalBallMax + bonusballMax <= 255, and lpEarnings from ticket sales can grow the LP/prizePool beyond the lpPoolCap assumptions, making the unsafe parameter region reachable over time (not inherently “rare”).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol packs ticket numbers into a `uint256` bit vector. Normal balls use bits `1` to `normalBallMax`, and the bonus ball uses the bit at `normalBallMax + bonusball`. 

In `Jackpot.buyTickets`, there is a check `ticket.bonusball <= bonusballMax`. However, there is no check that `normalBallMax + bonusballMax <= 255`. If `_setNewDrawingState` calculates a `bonusballMax` such that the sum exceeds 255 (but `bonusballMax` itself fits in `uint8`), users can purchase tickets where `normalBallMax + bonusball > 255`.

In `TicketComboTracker.insert`, the operation `1 << (_bonusball + _tracker.normalMax)` will overflow/wrap (in Solidity `1 << 256 == 0`) for shift amounts >= 256. This results in the bonus ball bit being lost (set to 0) in the packed ticket. 

When claiming winnings, the system reconstructs the bonus ball from the packed ticket. Since the bit is missing, the reconstruction fails or yields an incorrect value, making a potentially winning ticket unclaimable.

## Impact
Users purchasing tickets with valid high bonus ball numbers (allowed by the contract) will receive corrupted tickets that cannot win, resulting in loss of user funds (ticket cost + potential winnings).

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 35.
2. `prizePool` grows such that `bonusballMax` is calculated to be 230 (valid `uint8`).
3. User buys a ticket with `bonusball = 230`.
4. `buyTickets` calls `TicketComboTracker.insert`.
5. Shift amount = 35 + 230 = 265.
6. `1 << 265` results in 0 (or `1 << 9` if unchecked assembly, but Solidity standard shift results in 0 for >=256).
7. The packed ticket has no bonus ball bit set.
8. `claimWinnings` fails to match the user's chosen bonus ball.

## Proof of Code
function testTicketCorruption() public {
    uint8 normalMax = 35;
    uint8 bonusball = 230;
    // Sum = 265
    
    uint256 shift = uint256(normalMax) + bonusball;
    uint256 bit = 1 << shift;
    
    // Solidity behavior: 1 << 265 is 0
    assertEq(bit, 0);
}

## Suggested Mitigation
In `_setNewDrawingState`, ensure that `newBonusball` is capped such that `normalBallMax + newBonusball <= 255`.


## [H-2]. Incorrect Ticket Matching and Potential Theft of Winnings due to Bitwise Shift Overflow

## id: Bihq1Xv0amm7iSkcejKth

## Derived From Pattern/Invariant
Large ticket buyer covering many combos

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._calculateTicketTierId

## Finding Status: Valid
### Finding Status Justification: Even if the reporter’s theft narrative is overstated, the underlying shift>=256 condition is reachable because bonusballMax is dynamically derived from prizePool/LP value and is not capped to 255-normalBallMax, and LP value can grow via uncapped ticket-sale earnings.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library packs ticket numbers (normal balls and bonus ball) into a single `uint256` bit vector. The bonus ball is stored by shifting `1` to the left by `normalBallMax + bonusball`. 

Snippet from `TicketComboTracker.sol`:
```solidity
ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);
```

If the sum `_bonusball + _tracker.normalMax` equals or exceeds 256, the shift operation `1 << sum` results in `0` in Solidity for a `uint256` type (due to effective modulo 256 behavior on shift count or simply shifting out of range). This causes the bonus ball bit to be lost (effectively 0) in the packed ticket representation.

The `Jackpot` contract uses this packed representation to verify winnings in `_calculateTicketTierId`:

```solidity
uint256 ticketBonusball = _ticketNumbers >> (_normalBallMax + 1);
uint256 winningBonusball = _winningNumbers >> (_normalBallMax + 1);
uint256 bonusballMatch = (ticketBonusball == winningBonusball) ? 1 : 0;
```

If `normalBallMax` is 35 (standard) and `prizePool` grows sufficiently large, `bonusballMax` scales up (e.g., to 221+). If `bonusballMax` + `normalBallMax` >= 256, any bonus ball selection in the overflow range will result in a packed bonus bit of 0. Consequently, `ticketBonusball` and `winningBonusball` will both resolve to 0 during extraction, resulting in a false `bonusballMatch`. 

This allows a user holding a ticket with a losing bonus ball (e.g., 222) to claim a jackpot tier win if the winning bonus ball was also in the overflow range (e.g., 221), effectively stealing funds allocated for the true winner or draining the LP pool.

## Impact
Direct theft of winnings. Users can claim top-tier prizes with non-winning tickets, leading to insolvency of the prize pool and loss of LP funds.

## Command to Run Test


## Proof of Concept
1. Assume `normalBallMax` is 35.
2. `prizePool` grows such that `bonusballMax` calculation results in 225 (which is valid < 256).
3. `35 + 225 = 260`, which is >= 256.
4. Attacker buys a ticket with normals `{1,2,3,4,5}` and bonusball `225`. The packed ticket has normals bits set, but bonus bit is 0 (overflow).
5. The winning ticket is `{1,2,3,4,5}` with bonusball `221`. Packed winning ticket also has bonus bit 0.
6. Attacker calls `claimWinnings`. `_calculateTicketTierId` extracts bonusball for both as 0.
7. Contract registers a match (Tier 11 - Jackpot) despite the bonus balls being different (225 vs 221).
8. Attacker claims jackpot winnings they are not entitled to.

## Proof of Code
function testBitwiseOverflowExploit() public {
    // Setup: Admin sets normalBallMax
    vm.prank(owner);
    jackpot.setNormalBallMax(35);
    
    // Simulate large pot causing high bonusballMax
    // Force bonusballMax to be 225 (Total 260 > 256)
    // Implementation details omitted for brevity, assumes state manipulation or large buys
    
    // Attacker buys ticket with bonusball 225
    uint8[] memory normals = new uint8[](5);
    for(uint8 i=0; i<5; i++) normals[i] = i+1;
    
    IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
    tickets[0] = IJackpot.Ticket(normals, 225);
    vm.prank(attacker);
    jackpot.buyTickets(tickets, attacker, new address[](0), new uint256[](0), bytes32(0));
    
    // Winning ticket has bonusball 221
    // Both 225 and 221 cause overflow (35+225=260, 35+221=256)
    // Mock entropy to return 221
    // ... execution of draw ...
    
    // Verify attacker can claim jackpot
    vm.prank(attacker);
    jackpot.claimWinnings(ticketIds);
    // Assert attacker balance increased by jackpot amount
}

## Suggested Mitigation
Ensure that the sum of `normalBallMax` and `bonusballMax` never exceeds 255. Add a check in `_setNewDrawingState` and `setNormalBallMax`. Alternatively, use a more robust packing scheme that uses two separate uint256 variables or ensures bit-width safety.


## [H-3]. Insolvency due to execution of scaledEntropyCallback during Emergency Mode

## id: tgz0u54rTFbaRCVaoKZjT

## Derived From Pattern/Invariant
Jackpot.scaledEntropyCallback: !emergencyMode

## Exploit Type
PausableEmergencyStop

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: The callback executing during emergency mode is a real code path. It becomes relevant only if the owner enables emergency mode while an entropy request is pending (an admin decision), but if that happens, permissionless users can refund and create accounting divergence leading to insolvency/incorrect LP accounting; impact is not low.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function lacks the `noEmergencyMode` modifier, allowing it to execute even when the system is in Emergency Mode. If `runJackpot` is called, and then `enableEmergencyMode` is triggered (e.g. due to delay), users can call `emergencyRefundTickets` to withdraw their funds. Refunded funds leave the contract, but the `lpEarnings` and `globalTicketsBought` state variables are not decremented. When the entropy callback eventually arrives, `processDrawingSettlement` is executed using the inflated `lpEarnings` and the pre-refund `lpPoolTotal`. This calculates a new LP value that includes funds that have already been refunded to users. The next drawing initializes with this phantom value as the prize pool. Since the actual USDC balance is lower than the accounting value, the protocol becomes insolvent, preventing future payouts and LP withdrawals.

## Impact
Protocol insolvency and loss of funds for LPs/Winners as the accounting state diverges from the actual token balance.

## Command to Run Test


## Proof of Concept
1. Keeper calls `runJackpot()` to start the drawing process and lock the jackpot. 
2. Due to network delay or provider issue, the callback is delayed.
3. Owner calls `enableEmergencyMode()`.
4. Users call `emergencyRefundTickets()` to get their money back. The contract sends USDC but `lpEarnings` remains set to the ticket revenue.
5. The entropy provider finally submits the callback, triggering `scaledEntropyCallback`.
6. `processDrawingSettlement` adds the (now refunded) `lpEarnings` to the LP pool.
7. A new drawing starts with a prize pool backed by non-existent funds.
8. Subsequent winners or LPs attempting to withdraw will cause the contract to revert due to insufficient USDC balance.

## Proof of Code
function testExploitEmergencyInsolvency() public {
    // Setup: Buy tickets, lock jackpot, enable emergency, refund, then callback
    vm.startPrank(user);
    usdc.approve(address(jackpot), ticketPrice * 10);
    IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
    tickets[0] = IJackpot.Ticket(new uint8[](5), 1);
    jackpot.buyTickets(tickets, user, new address[](0), new uint256[](0), bytes32(0));
    vm.stopPrank();

    vm.prank(address(jackpot)); // Simulate keeper
    jackpot.runJackpot{value: 0.1 ether}();

    vm.prank(owner);
    jackpot.enableEmergencyMode();

    uint256[] memory ticketIds = jackpotNFT.getUserTickets(user, 1)[0].ticketIds;
    vm.prank(user);
    jackpot.emergencyRefundTickets(ticketIds);

    // Simulate delayed callback
    vm.prank(address(entropy));
    jackpot.scaledEntropyCallback(1, randomNumbers, "");

    // Check insolvency: LP Pool > Balance
    uint256 lpTotal = jackpotLPManager.getLPDrawingState(2).lpPoolTotal;
    assertGt(lpTotal, usdc.balanceOf(address(jackpot)));
}

## Suggested Mitigation
Add the `noEmergencyMode` modifier to `scaledEntropyCallback` to prevent settlement during emergency mode. Alternatively, ensure `emergencyRefundTickets` decrements `lpEarnings` and potentially `lpPoolTotal` if applicable, though blocking settlement is safer.


## [H-4]. Bitpacking overflow in TicketComboTracker allows manipulation of winning odds

## id: 10zo6n3N8CXvzbPLe8miQ

## Derived From Pattern/Invariant
TicketComboTracker.insert: normalBallMax + bonusballMax < 256

## Exploit Type
IntegerOverflow

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: While owner parameters influence reachability, there is no on-chain invariant enforcing normalMax + bonusballMax < 256, and bonusballMax is also derived from economic state. Once the system reaches the bound, the overflow/mis-tiering is permissionlessly exploitable and breaks core game correctness, so it is not merely 'governance risk'.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `TicketComboTracker.insert`, the bonus ball is packed into a uint256 bit vector using `1 << (_bonusball + _tracker.normalMax)`. If `_bonusball + _tracker.normalMax >= 256`, the shift operation results in 0 due to EVM semantics for 256-bit shifts. This effectively sets the bonus ball bit to 0 for that ticket. The same logic applies to the winning ticket in `countTierMatchesWithBonusball`. 

If the admin sets `normalBallMax` and the dynamically calculated `bonusballMax` such that their sum exceeds 255, users can purchase tickets with high bonus ball numbers that result in a '0' bit. If the winning bonus ball is also in this range, it will also result in a '0' bit. Consequently, these tickets will register a bonus ball match (0 == 0) despite having different nominal values, significantly increasing the probability of winning the jackpot or higher tiers. This breaks the fairness invariant of the lottery.

## Impact
Players can exploit bitpacking overflow to guarantee bonus ball matches, effectively bypassing the bonus ball difficulty and draining the prize pool.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 120.
2. High ticket volume drives `bonusballMax` to 140.
3. User buys ticket with bonus ball 136. `120 + 136 = 256`. Shift `1 << 256` is 0. Ticket stored with 0 bonus bit.
4. Winning number drawn has bonus ball 140. `120 + 140 = 260`. Shift is 0.
5. `_calculateTicketTierId` compares extracted bonus bits: 0 == 0. Match confirmed.

## Proof of Code
function testBitpackingOverflow() public {
    uint8 normalMax = 130;
    uint8 bonus = 130;
    uint256 shift = uint256(1) << (normalMax + bonus);
    assertEq(shift, 0);
}

## Suggested Mitigation
Enforce `normalBallMax + bonusballMax < 256` in `Jackpot.sol` whenever these parameters are updated (in `setNormalBallMax` and `_setNewDrawingState`). Ideally, verify `_bonusball + _tracker.normalMax < 256` in `TicketComboTracker`.


## [H-5]. Insolvency due to phantom LP earnings after emergency refunds

## id: 3OBfr-GYZCXPakKOGhcFy

## Derived From Pattern/Invariant
Balance

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: Valid
### Finding Status Justification: lpEarnings is not decremented on emergency refunds, and scaledEntropyCallback is not blocked in emergency mode; if emergency is enabled while entropy is pending, this can inflate LP accounting and enable incorrect withdrawals/insolvency. Impact is not low; it hinges on an admin emergency-mode decision.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `emergencyRefundTickets` function allows users to reclaim funds during emergency mode, effectively decrementing the contract's USDC balance. However, it fails to decrement the `lpEarnings` of the current drawing. If `runJackpot` was called prior to enabling emergency mode, the `scaledEntropyCallback` can still execute (as it lacks the `noEmergencyMode` modifier). When it executes, it settles the drawing using the original `lpEarnings`, crediting LPs with value that has been refunded to users. This inflates the LP pool value and the share price accumulator. When LPs subsequently withdraw (via `emergencyWithdrawLP` or normal flows), they withdraw based on this inflated value, draining more USDC than the contract holds, leading to insolvency.

## Impact
The contract becomes insolvent, preventing remaining LPs or winners from claiming their funds.

## Command to Run Test


## Proof of Concept
1. `runJackpot` is called, locking the drawing and requesting entropy.
2. Admin calls `enableEmergencyMode`.
3. Users call `emergencyRefundTickets`, receiving USDC refunds. `lpEarnings` remains high.
4. Entropy callback arrives (Pyth), executing `scaledEntropyCallback`. This settles the drawing, adding the phantom `lpEarnings` to `lpPoolTotal` and updating the accumulator.
5. An LP calls `emergencyWithdrawLP`. The withdrawal amount is calculated using the new (inflated) accumulator/pool value.
6. The LP withdraws more USDC than is available backing their shares, leaving the contract insolvent.

## Proof of Code
function testEmergencyRefundInsolvency() public {
    // Setup: Buy tickets, lock jackpot, enable emergency
    vm.startPrank(user);
    jackpot.buyTickets(tickets, user, ...);
    vm.stopPrank();
    jackpot.runJackpot{value: fee}();
    jackpot.enableEmergencyMode();
    
    // Refund tickets - balance drops, lpEarnings stays same
    uint256 balanceBefore = usdc.balanceOf(address(jackpot));
    vm.prank(user);
    jackpot.emergencyRefundTickets(ticketIds);
    assertEq(usdc.balanceOf(address(jackpot)), balanceBefore - ticketPrice);
    
    // Entropy callback executes settlement
    vm.prank(address(entropyProvider));
    jackpot.scaledEntropyCallback(1, randomNumbers, "");
    
    // LP withdraws based on phantom earnings
    vm.prank(lp);
    jackpot.emergencyWithdrawLP(); // Withdraws inflated amount
}

## Suggested Mitigation
In `emergencyRefundTickets`, decrement `drawingState[currentDrawingId].lpEarnings` by the refunded amount (net of referral fees) and `globalTicketsBought`. Also, add `noEmergencyMode` modifier to `scaledEntropyCallback` or ensure settlement logic handles emergency state.


## [H-6]. Emergency refunds cause permanent LP value loss due to settlement accounting mismatch

## id: x-vb0z9ClxIMeVUBD8L08

## Derived From Pattern/Invariant
Jackpot.emergencyRefundTickets: Balance Invariant

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: Valid
### Finding Status Justification: Because refunded/burned tickets remain in the combo tracker, a late entropy callback after emergency refunds can incorrectly account for unclaimable userWinnings and reduce LP accounting. This is not low impact if triggered; it depends on the owner enabling emergency while a callback is still possible.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
When `emergencyRefundTickets` is called, it burns the ticket NFT and refunds the user, but does not remove the ticket from `TicketComboTracker`. If the `scaledEntropyCallback` subsequently executes (which is allowed even in emergency mode), the settlement logic counts the refunded tickets as potential winners. `drawingUserWinnings` calculates the payouts for these refunded tickets, and `processDrawingSettlement` deducts this amount from the LP pool (`postDrawLpValue = lpPoolTotal + lpEarnings - userWinnings`). 

Since the tickets are burned, the winnings can never be claimed, leaving the deducted funds stuck in the contract while the LP pool value is permanently reduced. This creates an accounting mismatch where LPs suffer losses equal to the unclaimed winnings of refunded tickets.

## Impact
LPs lose equity equal to the winnings of refunded tickets. The contract accumulates excess USDC that belongs to LPs but is unaccounted for.

## Command to Run Test


## Proof of Concept
1. User buys winning ticket. Jackpot locked.
2. Admin enables Emergency Mode.
3. User calls `emergencyRefundTickets`, gets refund, ticket burned.
4. `scaledEntropyCallback` executes. Ticket counts as winner.
5. `userWinnings` includes payout for burned ticket.
6. `processDrawingSettlement` subtracts payout from LP pool.
7. LP pool value drops; payout is unclaimed.

## Proof of Code
vm.prank(owner); jackpot.enableEmergencyMode();
vm.prank(user); jackpot.emergencyRefundTickets(ids);
vm.prank(entropy); jackpot.scaledEntropyCallback(...);

## Suggested Mitigation
Modify `scaledEntropyCallback` to revert if `emergencyMode` is enabled, or adjust `TicketComboTracker` to allow removing tickets (difficult), or track refunded volume and adjust `drawingUserWinnings` in settlement.


## [H-7]. Unsafe downcast of `bonusballMax` breaks LP edge or causes DoS

## id: dRTSEFiJeXrElS046U78t

## Derived From Pattern/Invariant
Arithmetic

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: The uint8 cast in _setNewDrawingState is unchecked and can wrap if the computed value exceeds 255. Since lpPoolTotal can grow via ticket-sale earnings (not capped by lpPoolCap), overflow is reachable in principle and can cause DoS (bonusballMax=0) or materially incorrect difficulty. This is not purely governance, though it is plausibly rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_setNewDrawingState`, the new `bonusballMax` is dynamically calculated to maintain the LP edge. The result is cast to `uint8` without overflow checks: `uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)))`. If the calculated value exceeds 255 (which is common if `normalBallMax` is small, e.g., 5, and the prize pool grows large), the value truncates. 
- If it truncates to 0, `buyTickets` reverts (DoS) because `bonusball` cannot be 0.
- If it truncates to a small number, the game difficulty drops drastically below the required level to sustain the LP edge, resulting in massive negative EV for LPs.

## Impact
Direct loss of LP funds (negative EV) or denial of service for ticket purchases.

## Command to Run Test


## Proof of Concept
1. Initialize Jackpot with `normalBallMax = 5`. `combosPerBonusball` becomes 1.
2. Users buy tickets until `prizePool` > 255 * `ticketPrice` (e.g., 300 USDC).
3. `minNumberTickets` calculation yields ~300.
4. `_setNewDrawingState` calculates `newBonusball` = 300.
5. Cast to `uint8` turns 300 into 44.
6. The new drawing has `bonusballMax = 44` instead of 300, making the jackpot ~7x easier to win than the math requires.

## Proof of Code
function testBonusballTruncation() public {
    // Init with normalBallMax = 5
    // Buy tickets to raise pool
    // Settle drawing
    // Check newDrawingState.bonusballMax
    // Assert it is != calculated expected value (wrapped)
}

## Suggested Mitigation
Use `UintCasts.toUint8` to revert on overflow, or cap the value at 255. If capping at 255 violates the LP edge requirement (i.e., requires > 255 balls), the system should potentially cap the prize pool or halt.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-8]. DoS of `runJackpot` due to gas limit exceeding block maximums

## id: fPjqpi9kfdBkR5WEyLfSw

## Derived From Pattern/Invariant
Jackpot contract owner

## Exploit Type
Dos

## Location
Jackpot.runJackpot

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: Jackpot._calculateEntropyGasLimit is `entropyBaseGasLimit + entropyVariableGasLimit * uint32(bonusballMax)`. With default `entropyVariableGasLimit=250,000` and large `bonusballMax`, the requested callback gas limit can exceed typical per-block limits (e.g., ~64M at 255). Even if the request transaction succeeds, the provider may be unable to execute the callback within block constraints, effectively stalling the drawing (necessitating emergency mode). There is no on-chain cap/enforcement tying `bonusballMax` to a safe gas range. The owner can mitigate by tuning `entropyVariableGasLimit`/parameters, so it is partly governance/ops risk, but the protocol currently has no automatic safeguard.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `runJackpot` function calculates the gas limit for the entropy callback based on `bonusballMax`: `entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax`. The default `entropyVariableGasLimit` is 250,000. 

As the prize pool grows, `bonusballMax` increases (up to 255). If `bonusballMax` is 255, the requested gas limit is `Base + 250,000 * 255` ≈ 63.75 million gas. This exceeds the block gas limit of most EVM chains (e.g., Ethereum/Base is ~30M). The call to `entropy.requestAndCallbackScaledRandomness` (and subsequently `requestV2`) will revert or fail to be mined because the gas limit parameter is invalidly high.

While the actual gas usage of the callback might fit in a block (~17M gas for 255 balls in `TicketComboTracker`), the *requested* limit forces a DoS. The owner must intervene to lower `entropyVariableGasLimit`, but if they lower it too much, the callback might run out of gas during execution.

## Impact
The jackpot cannot be run (locked state) when the prize pool gets large, requiring admin intervention to retune parameters. If the actual gas cost also exceeds block limits, the DoS is unfixable without upgrades.

## Command to Run Test


## Proof of Concept
1. Prize pool grows such that `bonusballMax` calculates to 255.
2. Keeper calls `runJackpot`.
3. `_calculateEntropyGasLimit` returns ~64,000,000.
4. `entropy.requestV2` is called with `gasLimit = 64M`.
5. Transaction reverts because 64M > Block Gas Limit (30M).

## Proof of Code
function test_GasLimitDoS() public {
    // Set bonusballMax to 255 in state
    // Call runJackpot
    // Assert revert due to gas limit issues
}

## Suggested Mitigation
Implement a cap on `bonusballMax` within `_setNewDrawingState` such that the resulting gas limit and execution cost stays well within block limits. Also, optimize `TicketComboTracker` to reduce the high gas cost of settlement.


## [M-9]. Permanent DoS via Zero Accumulator

## id: U0ZAqsG7nol1yaXM-HFvs

## Derived From Pattern/Invariant
Arithmetic

## Exploit Type
IntegerMath

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: With extreme but allowed governance settings (e.g., very high referralFee combined with payout weight choices), postDrawLpValue can hit 0 and set accumulator to 0, bricking LP ops via division-by-zero. While governance-driven and likely rare, it is a real bricking condition, so not correctly described as 'not exploitable'.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
In `JackpotLPManager.processDrawingSettlement`, the `newAccumulator` is calculated as `(prevAccumulator * postDrawLpValue) / lpPoolTotal`. If `postDrawLpValue` becomes 0, the `newAccumulator` becomes 0. A zero accumulator causes division by zero in `processDeposit` and `processInitiateWithdraw`, permanently bricking all LP operations. `postDrawLpValue` can reach 0 if `reserveRatio` is 0 (allowed) and `referralFee` is 100% (allowed) and a winner takes the entire pool.

## Impact
Permanent freezing of the LP system (deposits and withdrawals bricked).

## Command to Run Test


## Proof of Concept
1. Admin sets `reserveRatio` to 0 and `referralFee` to 100%.
2. Users buy tickets (revenue goes to referrers, `lpEarnings` = 0).
3. A user wins the jackpot (takes entire `lpPoolTotal`).
4. `postDrawLpValue` = `lpPoolTotal` + 0 - `lpPoolTotal` = 0.
5. `newAccumulator` becomes 0.
6. Any subsequent call to `lpDeposit` reverts due to division by zero.

## Proof of Code
function testZeroAccumulatorBricking() public {
    // Set reserve 0, referral 100%
    // Win jackpot
    // Settle
    // Try lpDeposit -> Revert
}

## Suggested Mitigation
In `processDrawingSettlement`, ensure `newAccumulator` has a minimum value of 1 or handle the zero case to prevent bricking.





Finding Status: InvalidGovernanceRisk
## [M-10]. Phantom Winners Stranding Funds after Emergency Refund Resumption

## id: ZsaCTHFClH4RnKo94XhU5

## Derived From Pattern/Invariant
Emergency-mode ticket refunder

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Emergency refunds burn the ERC721 ticket but do not remove entries from `drawingEntries` (TicketComboTracker). If a drawing is later settled (due to callback arrival or explicit resumption), `_calculateDrawingUserWinnings` uses TicketComboTracker counts that still include refunded tickets, so payoutCalculator computes `drawingUserWinnings` including those refunded/burned tickets’ shares. LP settlement subtracts this amount from LP value immediately. Since the tickets are burned, those winnings can never be claimed, leaving USDC “stuck/unaccounted” and reducing LP value. No current code prevents settling a refunded drawing (no emergency gating in scaledEntropyCallback), and implementing removal in tracker is non-trivial, so a state flag/cancellation is needed.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When `emergencyRefundTickets` burns a ticket, it does not remove the ticket's numbers from the `TicketComboTracker`. If the drawing resumes, `payoutCalculator` will still count these "phantom" tickets as valid winners if their numbers match the draw. 

The system will calculate `userWinnings` including the phantom ticket's share and subtract this amount from `lpPoolTotal`. Since the ticket is burned, no one can claim these winnings. This results in the winning amount being permanently stranded in the contract and LPs suffering a loss of value for payouts that never occur.

## Impact
Permanent loss of LP value and stranded USDC in the contract.

## Command to Run Test


## Proof of Concept
1. User buys ticket [1,2,3,4,5], bonus 1.
2. Emergency Mode enabled; User refunds ticket.
3. Emergency Mode disabled; Drawing resumes.
4. Entropy returns [1,2,3,4,5], bonus 1.
5. `TicketComboTracker` still reports the ticket as sold.
6. `processDrawingSettlement` reduces LP Pool by the winning amount.
7. Ticket is burned, so winnings are never claimed. Funds stuck, LP value lost.

## Proof of Code
function testPhantomWinners() public {
    // Buy and refund ticket
    // ...
    // Resume and rig entropy to match refund ticket
    // ...
    // Check LP Pool reduced by winning amount
    // Check contract balance has stranded funds
}

## Suggested Mitigation
Similar to the insolvency finding, preventing drawing resumption after any refund is the safest fix. Implementing a `remove` function in `TicketComboTracker` is complex and gas-intensive.


## [M-11]. Bit shift overflow in `TicketComboTracker` corrupts tickets

## id: 6ItjeVbn5ooBIJdd8SoBz

## Derived From Pattern/Invariant
PricePrecision

## Exploit Type
PricePrecision

## Location
Jackpot._setNewDrawingState

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Users can buy tickets that pass on-chain range checks yet still have the bonus bit lost if system parameters/state allow normalMax + bonusball >= 256; that is not a 'user mistake'. It is primarily enabled by governance configuration (e.g., bonusballMin/normalBallMax choices) and missing invariant enforcement.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
In `TicketComboTracker.insert`, the ticket is packed into a bit vector using `set |= 1 << (_bonusball + _tracker.normalMax)`. If `_bonusball + _tracker.normalMax >= 256`, the shift operation overflows in Solidity `uint256`, resulting in 0. The bonus ball bit is lost. `Jackpot.sol` allows `normalBallMax` and `bonusballMin` to be set such that their sum exceeds 255 (e.g., normal=50, bonusMin=210). Users buying tickets with high bonus balls will receive corrupted tickets that have a bonus ball value of 0 when unpacked (due to logic in `unpackTicket` failing or returning incorrect data), preventing them from winning bonus tiers.

## Impact
Users pay for tickets that are technically invalid/corrupted and cannot win bonus tiers, leading to fund loss.

## Command to Run Test


## Proof of Concept
1. Owner sets `normalBallMax` = 50, `bonusballMin` = 210.
2. `_setNewDrawingState` sets `bonusballMax` >= 210.
3. User buys ticket with bonusball 210.
4. `TicketComboTracker` shifts `1 << (210 + 50) = 1 << 260` -> 0.
5. Ticket stored without bonus bit.
6. User cannot claim bonus match winnings.

## Proof of Code
function testBitShiftOverflow() public {
    // Setup params causing overflow
    // Buy ticket
    // Assert packed ticket is incorrect
}

## Suggested Mitigation
In `_setNewDrawingState`, ensure `normalBallMax + newBonusball < 256`. Also enforce validation in setters.





Finding Status: LowSeverityDueToRareLikelihood
## [M-12]. Bitpacking overflow allows gaining unfair winning odds via 'phantom' bonus balls

## id: 82t2HCPeTx5ndSUqD_GlZ

## Derived From Pattern/Invariant
Jackpot._setNewDrawingState: newBonusballMax <= 255 - normalBallMax

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: This is a concrete manifestation of the bit-shift overflow in `TicketComboTracker` when `normalBallMax + bonusball >= 256`, causing the bonus bit to be lost (evaluating as 0). When the winning bonusball also lies in the overflow range, the winning ticket’s bonus bit is also lost, so tickets with any overflow-range bonusball can match any overflow-range winning bonusball (all map to 0), increasing bonusball-match probability. Although achieving `bonusballMax` large enough can require large `prizePool` growth (potentially via duplicates) and/or parameter choices, the underlying bug is present and can be exploited once the state reaches those bounds. No invariant enforces `bonusballMax <= 255 - normalBallMax`.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_setNewDrawingState` function calculates `newBonusball` based on `lpEarnings` and `prizePool`, which are effectively uncapped (ticket sales increase them without limit). The `lpPoolCap` only limits LP deposits, not earnings from sales. If sufficient tickets are sold, `newBonusball` can be pushed such that `newBonusball + normalBallMax >= 256`. In `TicketComboTracker.insert`, the bonus ball is stored via bitshifting: `1 << (_bonusball + _tracker.normalMax)`. If the shift amount is >= 256, the result is 0, and the bonus bit is lost. This results in a ticket with an effective bonus ball of 0. If the winning random number also falls into this overflow range, its effective bonus ball is also 0. Consequently, a single ticket with an 'overflow' bonus ball matches *any* winning bonus ball that also overflows, significantly increasing winning probabilities (e.g., matching 5 different bonus balls with 1 ticket).

## Impact
Broken game fairness and probability logic. Attackers can buy tickets with significantly higher odds of winning the bonus match than intended.

## Command to Run Test


## Proof of Concept
1. Attackers buy a large volume of tickets (or one duplicate ticket many times) to inflate `lpEarnings` and thus `newPrizePool`.
2. `_setNewDrawingState` calculates a `newBonusball` such that `newBonusball + normalBallMax >= 256`.
3. In the new drawing, attackers buy tickets with bonus balls in the overflow range (e.g., 226 to 230 if normalMax is 30).
4. These tickets are stored with the bonus bit shifted out (effectively 0).
5. If the winning number is also in the overflow range (226-230), it is processed as having bonus ball 0.
6. The attacker's ticket matches the winning bonus ball, effectively covering multiple bonus ball outcomes with a single ticket.

## Proof of Code
function testExploitBitpackingOverflow() public {
    // Simulate condition where parameters allow overflow
    // normalBallMax = 30. We need bonusball >= 226.
    // Assume huge prize pool generated via sales.
    // This test verifies the bitpacking logic failure specifically.
    
    TicketComboTracker.Tracker storage tracker = drawingEntries[1];
    tracker.init(30, 230, 5);
    
    uint8[] memory normals = new uint8[](5);
    for(uint8 i=0; i<5; i++) normals[i] = i+1;
    
    // Insert ticket with bonusball 226. 226 + 30 = 256 (Overflow)
    (uint256 packed,) = TicketComboTracker.insert(tracker, normals, 226);
    
    // Verify bonus bit is lost (packed ticket only has normal bits)
    // Normals 1-5 set bits 2,4,8,16,32. Sum = 62.
    assertEq(packed, 62);
    
    // Insert winning number with bonusball 227. 227 + 30 = 257 (Overflow)
    // countTierMatchesWithBonusball logic simulation:
    uint256 winningPacked = TicketComboTracker.toNormalsBitVector(normals, 30) | (1 << (227 + 30));
    assertEq(winningPacked, 62);
    
    // They match exactly despite different bonus balls
    assertEq(packed, winningPacked);
}

## Suggested Mitigation
In `_setNewDrawingState`, explicitly cap `newBonusball` such that `newBonusball + normalBallMax < 256`. Additionally, check this invariant in `setNormalBallMax`.



