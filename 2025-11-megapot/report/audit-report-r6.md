# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[M-1]. LP Earnings accumulated during Emergency Mode are permanently locked
**Derived From** : EmergencyModeStateStuck
Finding Status: Valid
Privilege: RequiresRole


[H-2]. Unsafe downcast of dynamically calculated `bonusballMax` destroys LP edge protection
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-3]. Payout Corruption due to Bitwise Overflow in Ticket Bit Packing
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: RequiresAdminRole


[M-4]. Settlement DoS via Gas Limit due to excessive storage reads in high-bonusball drawings
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-5]. Bitwise overflow in ticket encoding leads to incorrect tier calculation and loss of winnings
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[M-6]. Bitwise shift overflow in TicketComboTracker corrupts high-value bonusballs
**Derived From** : Integer Overflow
Finding Status: Valid
Privilege: Permissionless


[H-7]. Winning ticket collision due to bit-packing overflow allows claiming prizes with losing numbers
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[M-8]. Ticket collision and payout error due to bit packing overflow in `TicketComboTracker`
**Derived From** : StorageLayout
Finding Status: Valid
Privilege: Permissionless


[M-9]. Permanent DoS of Drawing Settlement due to Excessive Gas Consumption in TicketComboTracker
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[M-10]. Settlement callback exceeds block gas limit for large pools
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-11]. Execution of scaledEntropyCallback during Emergency Mode leads to protocol insolvency after refunds
**Derived From** : EmergencyModeStateStuck
Finding Status: Valid
Privilege: Permissionless


[H-12]. Broken Game Logic due to Bonusball Max Wrap-Around and Bit-Shift Overflow
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[M-13]. DoS in drawing settlement due to excessive storage reads in winner counting
**Derived From** : Unbounded Loop / Excessive Gas
Finding Status: Valid
Privilege: Permissionless


[H-14]. Insolvency due to failure to decrement `lpEarnings` in `emergencyRefundTickets`
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-15]. Emergency refunds cause protocol insolvency and payout dilution if drawing resumes
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: RequiresAdminRole


[M-16]. LP Share Precision Loss Due to Accumulator Inflation
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-17]. LP Edge Violation due to uint8 casting truncation in bonusball calculation
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[M-18]. DoS and ticket corruption due to bonusball overflow in Jackpot._setNewDrawingState
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[H-19]. Uncapped lpEarnings allow bonusballMax to exceed bit-packing limits, breaking game fairness via guaranteed matches
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-20]. Unbounded prize pool growth causes bit vector overflow and corruption of winner selection
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-21]. Permanent freezing of LP funds if reserve ratio is zero and pool is drained
**Derived From** : Dos
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[M-22]. Jackpot Denial of Service via Entropy Provider Key Collision in ScaledEntropyProvider
**Derived From** : ExternalProtocolKeyCollision
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-23]. Protocol Insolvency and DoS due to unchecked Referral Fee exceeding LP Edge
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresRole


[M-24]. Risk-Free Arbitrage due to Unchecked Referral Fee vs LP Edge Configuration
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-25]. Denial of Service if Referral Fee exceeds LP Edge Target due to settlement underflow
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[M-26]. Protocol DoS due to Reserve Ratio allowing Accumulator to fall to zero
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole



Finding Status: InvalidGovernanceRisk


[M-27]. Arbitrage opportunity draining LP pool when Referral Fee exceeds LP Edge
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: InvalidByDesign


[M-28]. Governance DoS via front-running pool cap updates
**Derived From** : GovernanceFrontrunDoS
Finding Status: InvalidByDesign
Privilege: Permissionless


[M-29]. Governance DoS via LP Deposit Front-running on Parameter Updates
**Derived From** : GovernanceFrontrunDoS
Finding Status: InvalidByDesign
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood


[M-30]. Total LP pool loss sets accumulator to zero, permanently locking subsequent LP deposits
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 12
- M: 18
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. LP Earnings accumulated during Emergency Mode are permanently locked

## id: L3WNNrmg1hToRI9pjWoxr

## Derived From Pattern/Invariant
EmergencyModeStateStuck

## Exploit Type
EmergencyModeStateStuck

## Location
JackpotLPManager.emergencyWithdrawLP

## Finding Status: Valid
### Finding Status Justification: Jackpot.claimWinnings can add `referrerShare` to `drawingState[currentDrawingId].lpEarnings` when ticketInfo.referralScheme == 0. This function is not gated by noEmergencyMode, so it can occur during emergency mode. However, JackpotLPManager.emergencyWithdrawLP only accounts for lpPoolTotal/pendingDeposits/pendingWithdrawals/claimableWithdrawals and ignores Jackpot’s per-drawing lpEarnings. Thus USDC retained in the Jackpot contract due to “referrer share goes to LP” accounting can become stranded during emergency exits. This is a real accounting/escape-hatch shortfall that can lock funds.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
When Emergency Mode is enabled, `runJackpot` is disabled but users can still call `claimWinnings` for past drawings. If a ticket has no referral scheme, the referrer share is legally added to the `lpEarnings` of the *current* drawing (`drawingState[currentDrawingId].lpEarnings`). Since the current drawing is halted, these earnings accumulate in the contract's accounting. However, the `emergencyWithdrawLP` function, designed to allow LPs to exit during a system failure, only calculates withdrawable amounts based on `lpPoolTotal` and `pendingDeposits`. It completely ignores the `lpEarnings` of the current drawing. Consequently, any funds accumulated in `lpEarnings` during the emergency period are permanently trapped in the `Jackpot` contract balance with no mechanism to distribute them to LPs or withdraw them.

## Impact
Permanent loss of LP yield/funds accumulated during emergency mode.

## Command to Run Test


## Proof of Concept
1. Owner enables Emergency Mode. 
2. Users call `claimWinnings` for past tickets. 
3. Referral shares from these claims are added to `lpEarnings` of the current drawing (which is stuck). 
4. LPs call `emergencyWithdrawLP` to exit the protocol. 
5. The function calculates their share of the pool and deposits, but fails to include a share of the current `lpEarnings`. 
6. The USDC corresponding to those earnings remains stuck in the `Jackpot` contract.

## Proof of Code


## Suggested Mitigation
In `emergencyWithdrawLP`, include a proportional share of the current drawing's `lpEarnings` in the withdrawal calculation, or provide a separate mechanism to sweep these earnings.


## [H-2]. Unsafe downcast of dynamically calculated `bonusballMax` destroys LP edge protection

## id: SToXLsV2g1fcmrYq48Iv4

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot.sol._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: `_setNewDrawingState` uses `uint8(...)` truncation without bounds checks. Because lpValue/prizePool can grow beyond the intended cap path (lpEarnings not capped), computed required bonusball counts can exceed 255 and wrap, so this is not merely a rare theoretical issue.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the new `bonusballMax` is calculated to ensure the lottery difficulty is high enough to protect the LP edge. The formula `uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)))` casts the result to `uint8`. If the required `bonusballMax` exceeds 255 (which can happen if the prize pool is large relative to `combosPerBonusball`), the cast truncates the value (modulo 256). This results in a much smaller `bonusballMax` than required. Consequently, the number of available ticket combinations becomes far smaller than the `minNumberTickets` threshold. Players can buy all combinations for a fraction of the prize pool value, guaranteeing a win that exceeds the ticket cost, thereby draining the LP pool and violating the LP edge guarantee.

## Impact
LPs suffer massive losses as the prize pool becomes under-collateralized relative to the probability of winning. Attackers can drain the LP pool risk-free.

## Command to Run Test


## Proof of Concept
1. `prizePool` is 100M USDC. `ticketPrice` 1 USDC. `lpEdge` 20%.
2. `minNumberTickets` = 100M / 0.8 = 125M.
3. `normalBallMax` is 10. `combosPerBonusball` = C(10,5) = 252.
4. Required `bonusball` = 125,000,000 / 252 ≈ 496,031.
5. `newBonusball` = `uint8(496031)` = `496031 % 256` = 159.
6. Actual total combos = 252 * 159 ≈ 40,000.
7. Cost to buy all tickets = 40,000 USDC.
8. Payout = 100M USDC.
9. Attacker profits 99.96M USDC from LPs.

## Proof of Code
function testBonusballCastOverflow() public {
    uint256 minTickets = 125000000;
    uint256 combos = 252;
    uint256 calc = (minTickets + combos - 1) / combos;
    uint8 casted = uint8(calc);
    assert(calc > 255);
    assert(casted < 255);
    // Resulting difficulty is drastically lower than required
}

## Suggested Mitigation
Cap `bonusballMax` at 255 in the calculation (using `Math.min(255, ...)`), or revert if the required difficulty exceeds 255. If capped, the LP edge target cannot be guaranteed, so the system should likely revert or cap the prize pool.


## [H-3]. Payout Corruption due to Bitwise Overflow in Ticket Bit Packing

## id: RFYjOH4sQ_x3XEzYRg8U0

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._calculateTicketTierId

## Finding Status: Valid
### Finding Status Justification: The packing overflow condition (shift >= 256) can occur whenever `bonusball + normalMax >= 256`. Although `normalBallMax` is effectively capped by `Combinations.choose(n,5)` (n<=128), `bonusballMax` can still exceed `255 - normalBallMax` if prizePool/lpValue grows beyond the intended cap path (e.g., via uncapped lpEarnings), so this is not necessarily rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
Tickets are stored as bit vectors where the bonus ball bit is shifted by `normalBallMax`. The formula used in `TicketComboTracker` is `1 << (bonusball + normalMax)`. If `bonusball + normalMax >= 256`, the shift operation overflows in `uint256` arithmetic, resulting in 0. This causes the bonus ball bit to be lost during ticket minting (`insert`) and during winning number calculation (`countTierMatchesWithBonusball`). When `Jackpot` calculates payouts, `unpackTicket` (or the bit shifting logic in `_calculateTicketTierId`) retrieves 0 for both the user's ticket bonus ball and the winning ticket bonus ball. Consequently, every ticket is treated as having matched the bonus ball, upgrading all tiers (e.g., Tier 0 becomes Tier 1, Tier 10 becomes Jackpot).

## Impact
Incorrect payout calculations where every ticket is awarded a bonus ball match. This leads to massive overpayment of winnings and protocol insolvency.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 140.
2. Dynamic calculation (or random draw) sets `bonusballMax` to 120.
3. Sum = 260. `1 << 260` results in 0.
4. User buys ticket. `packedTicket` stored with NO bonus bit set.
5. Winning numbers drawn. `winningTicket` stored with NO bonus bit set.
6. `_calculateTicketTierId` extracts bonus ball via `>> (140 + 1)`.
7. `ticketBonus = 0`, `winningBonus = 0`.
8. `bonusballMatch = 1`.
9. All non-winning tickets become Tier 1 winners. All Match-5 tickets become Jackpots.

## Proof of Code
function testBitOverflow() public {
    uint256 normalMax = 140;
    uint256 bonus = 120;
    uint256 packed = 1 << (bonus + normalMax);
    assertEq(packed, 0);
}

## Suggested Mitigation
Enforce `normalBallMax + bonusballMax < 255` in `_setNewDrawingState` and `setNormalBallMax`. If dynamic `bonusballMax` exceeds this limit, cap it or revert to prevent the overflow state.


## [M-4]. Settlement DoS via Gas Limit due to excessive storage reads in high-bonusball drawings

## id: 9fZSnBPO-1xLQeEwgt0J3

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.sol.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: High `bonusballMax` is a normal outcome for large prize pools; the settlement path is inherently O(bonusballMax * subset work). This can plausibly exceed tx/block gas budgets on target chains, so it isn't clearly 'rare'.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function executes `countTierMatchesWithBonusball`, which iterates `bonusballMax` times. Inside the loop, it performs storage reads on `_tracker.comboCounts`. If `bonusballMax` is high (up to 255), the number of storage reads (255 * 31 subsets) results in gas consumption that can exceed the block gas limit. Since `bonusballMax` is fixed for the duration of a drawing, if settlement runs out of gas, the drawing becomes permanently locked (`jackpotLock` remains true), freezing the protocol and all funds.

## Impact
Permanent Denial of Service (DoS) of the protocol; funds stuck in active drawing until `emergencyMode` is activated (which is a destructive recovery).

## Command to Run Test


## Proof of Concept
1. `bonusballMax` is dynamically set to 255.
2. `runJackpot` is called. It calculates `entropyGasLimit` based on `variableGas * 255`. Assume this limit is within block limits.
3. Callback executes `countTierMatchesWithBonusball`.
4. Loop runs 255 times. Inner logic generates 31 subsets. Total 7905 storage reads.
5. 7905 * 2100 (cold read) > 16M gas. If `entropyVariableGasLimit` was set too low, transaction reverts.
6. Even if limit is raised, if it exceeds block max, it cannot execute.
7. Drawing remains locked.

## Proof of Code


## Suggested Mitigation
Optimize `countTierMatchesWithBonusball` to avoid iterating all bonusballs (e.g., iterate purchased tickets if fewer, or use a more efficient data structure). Alternatively, enforce a stricter upper bound on `bonusballMax` that respects block gas limits.


## [H-5]. Bitwise overflow in ticket encoding leads to incorrect tier calculation and loss of winnings

## id: Rf3tigFOP2_TKz1JaIReq

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
TicketComboTracker.sol.insert

## Finding Status: Valid
### Finding Status Justification: Shift>=256 in `TicketComboTracker.insert` is reachable if `bonusballMax` grows beyond `255-normalMax` (no explicit enforcement in code). Given uncapped lpEarnings/prizePool growth paths, this is not clearly rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library packs ticket numbers into a `uint256` bit vector using the formula `set |= 1 << (_bonusball + _tracker.normalMax)`. If `_bonusball + _normalMax >= 256`, the shift operation overflows (results in 0), and the bonus ball bit is lost. This occurs because `bonusballMax` is dynamically calculated and can be up to 255, while `normalMax` can also be up to 255. In the `Jackpot` contract, `_calculateTicketTierId` relies on this bit being present. If the bit is missing (due to overflow), `ticketBonusball` evaluates to 0. If the winning ticket also suffered overflow (or has bonusball 0, impossible normally but 0 via overflow), `bonusballMatch` becomes 1. However, `matches` (popcount) will count only the normal matches (since bonus bit is 0). The tier formula `2 * (matches - bonusballMatch) + bonusballMatch` then calculates `2 * (matches - 1) + 1`. For a Tier 1 win (0 normal matches + bonus), this results in `2 * (-1) + 1`, causing an underflow and a huge tier ID, resulting in 0 payout. For a Jackpot win (5 normal matches + bonus), this results in `2 * 4 + 1 = 9` (Tier 9), paying out significantly less than Tier 11.

## Impact
Users with winning tickets (especially top-tier) receive zero or reduced payouts if the ball parameters result in bit positions >= 256.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 50.
2. Dynamic calculation sets `bonusballMax` to 210 (due to high prize pool or low edge).
3. User buys ticket with bonusball 210. `50 + 210 = 260`.
4. `insert` shifts `1 << 260` -> 0. Ticket stored without bonus bit.
5. Winning numbers also have bonusball 210.
6. `claimWinnings` calls `_calculateTicketTierId`.
7. `ticketBonusball` (0) == `winningBonusball` (0). `bonusballMatch` = 1.
8. `matches` (popcount) = 5 (normals).
9. Result: `2 * (5 - 1) + 1 = 9`. User gets Tier 9 payout instead of Jackpot.

## Proof of Code
function testBitwiseOverflow() public {
    uint8 normalMax = 50;
    uint8 bonusball = 210;
    uint256 set = (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5);
    uint256 ticket = set | (1 << (bonusball + normalMax));
    // ticket should have bonus bit, but 1 << 260 is 0 in Solidity 0.8 assembly/unchecked or wraps to 0
    assertEq(ticket, set);
}

## Suggested Mitigation
Ensure `normalBallMax + bonusballMax < 256` when setting parameters or calculating new drawing state. Add a check in `_setNewDrawingState`.


## [M-6]. Bitwise shift overflow in TicketComboTracker corrupts high-value bonusballs

## id: K0v9jAOck0ZQgy816pZ8j

## Derived From Pattern/Invariant
Integer Overflow

## Exploit Type
IntegerOverflow

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: Nothing enforces `bonusballMax <= 255 - normalMax`. If bonusballMax grows, shifts >=256 are reachable and corrupt encoding/decoding; not clearly rare for a growing system.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `TicketComboTracker.insert`, ticket combinations are stored as a bitmask using `ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);`. The shift operation `1 << x` in Solidity returns 0 if `x >= 256`.

The system constraints allow `normalBallMax` up to 128 and `bonusballMax` up to 255. If `_bonusball + _tracker.normalMax >= 256`, the shift results in 0, and the bonusball bit is effectively lost (not set in the packed ticket). 

Similarly, `_calculateTicketTierId` extracts the bonusball using `_ticketNumbers >> (_normalBallMax + 1)`. If the bit was lost during storage, the extracted value is 0. Since both the user's ticket and the winning ticket (if it also falls in this range) will resolve to a bonusball value of 0, they will match incorrectly. Effectively, all bonusballs in the overflow range map to '0' and match each other, significantly altering the winning odds and breaking fairness.

## Impact
Broken game logic: Users who purchase tickets with bonusball numbers in the overflow range will have their tickets corrupted. This causes incorrect payout calculations (tickets matching when they shouldn't, or failing to match specific numbers), leading to loss of funds or unfair winnings.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 128.
2. Dynamic calculation sets `bonusballMax` to 130.
3. User 1 buys a ticket with bonusball 128. Shift is `128 + 128 = 256`. Result `1 << 256` is 0. Stored bonusball is 0.
4. User 2 buys a ticket with bonusball 129. Shift is `129 + 128 = 257`. Result 0. Stored bonusball is 0.
5. Drawing happens. Winning bonusball is 130. Shift is `130 + 128 = 258`. Result 0. Winning bonusball decoded as 0.
6. Both User 1 and User 2 claim winnings. The system compares their decoded bonusball (0) with the winning bonusball (0). Both are awarded a bonusball match despite picking different, losing numbers.

## Proof of Code
function testBitShiftOverflow() public {
    uint256 normalMax = 128;
    uint256 bonusball = 128;
    uint256 shiftAmount = bonusball + normalMax;
    uint256 result = 1 << shiftAmount;
    assertEq(result, 0);
}

## Suggested Mitigation
Ensure that `normalBallMax + bonusballMax < 256` in the configuration setters or drawing initialization logic. Alternatively, use a mapping or a different storage structure for the bonusball instead of packing it into the same `uint256` bitmask if the range requires it.


## [H-7]. Winning ticket collision due to bit-packing overflow allows claiming prizes with losing numbers

## id: lBBf_ecCpUp9Vn_4_op2M

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: The bit-packing uses shifts that yield 0 for (bonusball+normalMax) >= 256, collapsing multiple distinct bonusballs into the same packed representation and corrupting tier logic. There is no check enforcing normalBallMax+bonusballMax < 256 at drawing creation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol uses a bit-packing scheme in `TicketComboTracker` where the bonus ball is stored at bit position `_bonusball + _tracker.normalMax`. The EVM `SHL` opcode (used by `1 << x`) produces 0 if the shift amount `x` is greater than or equal to 256.

If the sum of `bonusballMax` and `normalBallMax` exceeds 255, any ticket selected with a bonus ball number `B` such that `B + normalBallMax >= 256` will have its bonus ball bit shifted out, resulting in a packed ticket with effectively NO bonus ball (value 0). 

Since multiple high-value bonus balls will all map to 0, they collide. Furthermore, if the winning number's bonus ball also falls into this overflow range, the winning ticket will also have a 0 bonus bit. The `claimWinnings` function compares the packed bits (via `_calculateTicketTierId`). Since both user and winner yield 0 for the bonus component, they will be considered a match.

This allows a user holding a ticket with bonus ball `X` (where X overflows) to win the jackpot even if the winning number is `Y` (where Y also overflows), significantly increasing the probability of winning the bonus tier.

## Impact
Players can win the top prize tier with incorrect numbers, draining the prize pool and LP funds.

## Command to Run Test


## Proof of Concept
1. Governance sets `normalBallMax` = 10.
2. Prize pool grows such that `bonusballMax` is calculated to be 250 (valid uint8).
3. `normalBallMax` (10) + `bonusballMax` (250) = 260.
4. User buys ticket with bonus ball 250. Bit shift `1 << 260` -> 0. Packed ticket has no bonus bit.
5. User buys ticket with bonus ball 246. Bit shift `1 << 256` -> 0. Packed ticket has no bonus bit.
6. Winning number drawn is 250. Winning packed ticket has no bonus bit.
7. User claims winnings with ticket 246.
8. `_calculateTicketTierId` sees 0 for both tickets' bonus part. Match confirmed.
9. User wins jackpot with losing number 246.

## Proof of Code
function testBitPackingCollision() public {
    // Mock setup where normalMax=10, bonusball=246
    uint256 normalMax = 10;
    uint256 bonusball = 246;
    uint256 packed = 1 << (bonusball + normalMax); // Result is 0
    assertEq(packed, 0);
    // This confirms the bit is lost and collisions occur for all bonusballs >= 246
}

## Suggested Mitigation
Ensure that `normalBallMax + bonusballMax < 256` is enforced. In `_setNewDrawingState`, when calculating `newBonusball`, cap the result such that `newBonusball + normalBallMax < 256`.


## [M-8]. Ticket collision and payout error due to bit packing overflow in `TicketComboTracker`

## id: z69HY6vgUpnoheIRItJL_

## Derived From Pattern/Invariant
StorageLayout

## Exploit Type
IntegerOverflow

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: The protocol does not enforce `normalMax + bonusballMax < 256`, and prizePool/lpValue growth can push `bonusballMax` beyond packing-safe bounds. This is not clearly rare once the system grows.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` packs normal balls and the bonus ball into a `uint256`. Normal balls use bits `1` to `normalBallMax`. The bonus ball uses bit `normalBallMax + bonusball`. 

There is no check that `normalBallMax + bonusballMax < 256`. While `normalBallMax` is capped at 128, `bonusballMax` is calculated dynamically and can reach 255. If the sum exceeds 255, the shift operation `1 << (normalMax + bonusball)` wraps modulo 256 (in Solidity 0.8+). This causes the bonus ball bit to collide with normal ball bits (e.g., bit 1). 

This corrupts ticket data. Furthermore, `Jackpot._calculateTicketTierId` uses `>>` to extract the bonus ball. If the bit wrapped, the shift returns 0. If both the user ticket and the winning ticket have high bonus balls that wrap, both evaluate to 0, resulting in a false positive bonus ball match and incorrect higher-tier payouts.

## Impact
Corrupted ticket data, inability to unpack tickets (reverts), and incorrect payouts due to false positive bonus ball matches.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 50. `prizePool` grows such that `bonusballMax` is calculated as 210.
2. User buys ticket with bonus ball 210.
3. `insert`: `1 << (50 + 210)` = `1 << 260` = `1 << 4`.
4. Bit 4 (representing normal ball 4) is set. The actual bonus ball info is lost/aliased.
5. Winning numbers also have bonus ball 210.
6. `_calculateTicketTierId`: `ticket >> 51` returns 0. `winning >> 51` returns 0. Match detected.
7. User gets paid for a bonus match even if the aliased bit implies a different ball.

## Proof of Code
uint256 packed = 1 << 260; assertEq(packed, 16); // Bit 4 set

## Suggested Mitigation
Ensure `normalBallMax + bonusballMax < 256` in `_setNewDrawingState`. Cap `bonusballMax` or revert if the condition cannot be met.


## [M-9]. Permanent DoS of Drawing Settlement due to Excessive Gas Consumption in TicketComboTracker

## id: T30OKjxipbLU_iVfF-mmT

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
TicketComboTracker._countSubsetMatches

## Finding Status: Valid
### Finding Status Justification: The settlement loop complexity grows with `bonusballMax`, which can grow with prize pool. There is no hard on-chain bound guaranteeing the callback remains under block/tx gas limits, so it is not clearly rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker._countSubsetMatches` function iterates through every possible bonusball (`_tracker.bonusballMax`) to calculate winning counts. Inside this loop, it iterates tiers and subsets, performing storage reads (`comboCounts[i][subset]`). If `bonusballMax` is high (e.g., near 255), the number of storage reads can exceed 7,900. With cold SLOAD costs (2100 gas), the gas cost for this loop alone can exceed 16.6 million gas, not counting memory expansion and execution overhead. Since `bonusballMax` scales dynamically with the prize pool size, a successful/large jackpot can result in a settlement transaction that exceeds the block gas limit (typically 30M on EVM chains, but often lower effectively). If the settlement transaction runs out of gas, the callback reverts, leaving the jackpot in a locked state (`jackpotLock = true`) permanently, requiring emergency intervention.

## Impact
The Jackpot contract becomes permanently locked (DoS) for the current drawing, preventing settlement and requiring emergency mode activation to refund users.

## Command to Run Test


## Proof of Concept
1. `prizePool` grows to a significant amount (e.g., $30M+ with standard params, or less with small `normalBallMax`).
2. `bonusballMax` scales to ~250 to maintain LP edge.
3. `runJackpot` is called, locking the drawing.
4. Entropy provider calls `scaledEntropyCallback`.
5. `_countSubsetMatches` executes loop 250 times. Each iteration reads ~31 storage slots.
6. Total gas > 16M-20M+. Transaction fails Out of Gas.
7. Drawing remains locked. Re-running `runJackpot` (after admin unlock) fails again as parameters are fixed for the drawing.

## Proof of Code
function testGasDos() public {
    // Simulate loop cost
    uint256 gasStart = gasleft();
    uint256 iterations = 250 * 31;
    for(uint i; i<iterations; ++i) {
        // Simulate SLOAD
        uint256 val = uint256(keccak256(abi.encode(i))); 
    }
    uint256 gasUsed = gasStart - gasleft();
    // assert(gasUsed > BLOCK_LIMIT);
}

## Suggested Mitigation
Optimize `TicketComboTracker` to avoid O(N) iteration over all bonusballs during settlement. Maintain a global count of subsets (ignoring bonusball) during insertion. In settlement, calculate 'No Bonus Match' counts by subtracting the specific 'Bonus Match' count (O(1)) from the global count, rather than summing all non-matching bonusballs.


## [M-10]. Settlement callback exceeds block gas limit for large pools

## id: 52pcD71okeo4f9NPwTZai

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: Valid
### Finding Status Justification: `TicketComboTracker._countSubsetMatches` is O(bonusballMax * subsets * bit-scan work). With bonusballMax near 255 and normal bits potentially high, this can plausibly exceed block/tx limits and brick settlement; it's not inherently a 'rare' edge case.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker.countTierMatchesWithBonusball` function iterates through combinations of subsets for every possible bonusball value. The loop structure roughly performs `bonusballMax * 31` iterations. Inside the loop, it accesses the `comboCounts` storage mapping.

As the prize pool grows, `bonusballMax` dynamically increases up to 255. In this scenario, the loop performs 255 * 31 = 7,905 iterations. Each iteration involves storage reads (SLOAD). Assuming cold access cost (2100 gas), 7,905 reads cost ~16.6 million gas. With overhead and other logic, the transaction can exceed the block gas limit (typically 30M for Optimism/Base) or the transaction gas limit.

If the gas cost exceeds the block limit, the `scaledEntropyCallback` will consistently fail (OOG), and the jackpot drawing will be stuck forever, unable to settle.

## Impact
Protocol liveness failure; valid drawings cannot settle if the pool is large enough to require high `bonusballMax`.

## Command to Run Test


## Proof of Concept
1. Pool grows such that `bonusballMax` calculation yields 255.
2. `runJackpot` is called.
3. `scaledEntropyCallback` is triggered by Pyth.
4. `countTierMatchesWithBonusball` runs loop 1 to 255.
5. Inner loops run 31 times per outer loop.
6. Total SLOADs ~7900.
7. Gas used > 16M + overhead.
8. If deployed on a chain with tighter limits or if overhead is high, tx reverts.

## Proof of Code
function testGasLimit() public {
    // Setup tracker with max bonusball
    // Measure gas for countTierMatchesWithBonusball
    // If > Block Limit, confirmed
}

## Suggested Mitigation
Optimize the `countTierMatchesWithBonusball` logic to avoid iterating all bonusballs or restructure storage to reduce SLOADs. Alternatively, hard cap `bonusballMax` to a safe lower value (e.g., 100) that fits comfortably within gas limits.


## [H-11]. Execution of scaledEntropyCallback during Emergency Mode leads to protocol insolvency after refunds

## id: f_3eKSWfk9tlo9-bZaYTc

## Derived From Pattern/Invariant
EmergencyModeStateStuck

## Exploit Type
EmergencyModeStateStuck

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: Jackpot.scaledEntropyCallback lacks the noEmergencyMode modifier and does not check emergencyMode. During emergency mode, users can call emergencyRefundTickets, which transfers USDC out but does not adjust drawingState[currentDrawingId].lpEarnings or combo-tracker counts. If a delayed entropy callback executes afterwards, processDrawingSettlement uses stale lpEarnings and winner counts, inflating LP accounting versus actual USDC and/or causing settlement reverts/ghost balances. This can lead to insolvency and blocked withdrawals or mis-accounted payouts.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function in `Jackpot.sol` handles the settlement of a drawing after receiving randomness from Pyth. However, it lacks the `noEmergencyMode` modifier. If a drawing is delayed or stuck and the owner enables Emergency Mode, users can call `emergencyRefundTickets` to burn their NFTs and reclaim their USDC (deducted from the contract's balance). 

If the entropy callback subsequently executes (e.g., via a delayed Pyth retry or network recovery), the settlement logic in `processDrawingSettlement` proceeds. It credits the full `lpEarnings` (accumulated from ticket sales) to the LP pool and calculates the new LP value. Crucially, it fails to account for the fact that the funds corresponding to `lpEarnings` have already been paid out as refunds. The LP pool is credited with 'ghost' funds that no longer exist in the contract's USDC balance. This results in immediate protocol insolvency, where the recorded LP pool value exceeds the actual collateral, preventing LPs from withdrawing their legitimate positions.

## Impact
The protocol becomes insolvent. LPs are credited with funds that have already been refunded to users, leading to a deficit in the contract's USDC balance and an inability to fulfill withdrawal requests.

## Command to Run Test


## Proof of Concept
1. A drawing is initiated via `runJackpot`, locking the jackpot.
2. The entropy callback is delayed or stuck.
3. The owner calls `enableEmergencyMode`.
4. Users call `emergencyRefundTickets`, burning tickets and draining the USDC corresponding to ticket sales from the contract.
5. The `scaledEntropyCallback` is successfully executed (e.g., the transaction is finally mined or retried).
6. The callback calls `jackpotLPManager.processDrawingSettlement`, which adds the recorded `lpEarnings` to the LP pool total.
7. LPs attempt to withdraw their funds via `emergencyWithdrawLP` (or normal flow if emergency is disabled), but the transaction reverts because the contract lacks sufficient USDC to cover the inflated LP pool value.

## Proof of Code
function test_EmergencyModeInsolvency() public {
    // 1. Setup drawing and buy tickets
    vm.startPrank(user);
    jackpot.buyTickets(tickets, user, referrers, splits, bytes32(0));
    vm.stopPrank();
    
    // 2. Run jackpot (locks drawing)
    vm.warp(block.timestamp + duration + 1);
    jackpot.runJackpot{value: fee}();

    // 3. Enable Emergency Mode
    vm.prank(owner);
    jackpot.enableEmergencyMode();

    // 4. User refunds tickets
    vm.prank(user);
    jackpot.emergencyRefundTickets(ticketIds);

    // 5. Entropy callback executes (simulated)
    vm.prank(address(entropy));
    jackpot.scaledEntropyCallback(sequence, randomNumbers, "");

    // 6. LP tries to withdraw -> REVERT due to insufficient funds
    vm.prank(lp);
    vm.expectRevert(); // Insufficient balance
    jackpot.emergencyWithdrawLP(); 
}

## Suggested Mitigation
Add the `noEmergencyMode` modifier to the `scaledEntropyCallback` function in `Jackpot.sol`. Additionally, ensure that enabling Emergency Mode or processing refunds clears the `lpEarnings` for the current drawing to prevent double-counting.


## [H-12]. Broken Game Logic due to Bonusball Max Wrap-Around and Bit-Shift Overflow

## id: 7NjGpgZhAkjC4mF2MdAke

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: Jackpot._setNewDrawingState casts to uint8 without bounds checks, and TicketComboTracker packs bonusball via 1 << (bonusball+normalMax) which becomes 0 for shifts >=256. There is no invariant enforcing bonusballMax <= 255-normalBallMax, and lpPoolCap does not cap lpPoolTotal growth via lpEarnings, so unsafe states are reachable and have high-impact correctness/solvency consequences.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Jackpot mechanism determines the `bonusballMax` parameter dynamically to guarantee an LP edge. However, the calculation `uint8(Math.max(..., Math.ceilDiv(...)))` casts the result to `uint8` without overflow protection. If the required number of tickets (derived from the prize pool) implies a `bonusballMax` greater than 255, the value wraps around (e.g., 300 becomes 44), significantly lowering the difficulty and destroying the LP edge.

Furthermore, in `TicketComboTracker.insert`, ticket numbers are packed into a `uint256` bit vector using the operation `1 << (_bonusball + _tracker.normalMax)`. If `_bonusball + _tracker.normalMax >= 256`, the shift operation overflows in Solidity (resulting in 0). This causes the bonusball bit to be lost, effectively setting the bonusball to 0 for all affected tickets. With a `normalBallMax` of 10 (Combos=252), a prize pool of just ~62k USDC requires a `bonusballMax` > 245, causing `10 + 246 = 256` (overflow). This results in all tickets matching the winning bonusball (which is also packed as 0), leading to massive over-payouts and protocol drainage.

## Impact
Complete loss of LP edge due to wrapped difficulty parameter, or incorrect high-value payouts due to corrupted ticket data (Bit-Shift Overflow) leading to protocol insolvency.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 10 (Combos=252).
2. Prize pool accumulates to 65,000 USDC.
3. `runJackpot` is called. `minNumberTickets` calculated as ~65,000 / (1-edge). `bonusballMax` calculated as ~257.
4. **Scenario A (Wrap)**: If `bonusballMax` calc wraps, it becomes 1. LP edge is lost.
5. **Scenario B (Shift Overflow)**: If logic doesn't wrap (or before it wraps), `normalMax (10) + bonusballMax (257) = 267`. `1 << 267` is 0.
6. The new drawing is initialized with these parameters.
7. Users buy tickets. `TicketComboTracker.insert` fails to set the bonus bit. All tickets stored with bonus bit 0.
8. Drawing settles. Winning ticket also has bonus bit 0.
9. Every ticket sold is considered a bonusball match (Tiers 1, 3, 5...).
10. Payouts drastically exceed expectations, draining the prize pool/contract.

## Proof of Code
function testExploit() public {
  // Assume current params
  uint8 normalMax = 10;
  uint8 bonusball = 250;
  // Internal logic check
  uint256 packed = 1 << (bonusball + normalMax);
  assertEq(packed, 0); // Fails to set bit
}

## Suggested Mitigation
1. Use `UintCasts.toUint8` to revert on `bonusballMax` overflow instead of wrapping.
2. Add a sanity check in `_setNewDrawingState` or `TicketComboTracker` ensuring `normalBallMax + bonusballMax < 256`.


## [M-13]. DoS in drawing settlement due to excessive storage reads in winner counting

## id: JBmcHGSwYvw0wk7Mvzrhb

## Derived From Pattern/Invariant
Unbounded Loop / Excessive Gas

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: While callback gas can be configured, it cannot exceed block/tx constraints and does not reduce intrinsic work. If the entropy callback runs OOG, the drawing remains locked, which is not low impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function calls `_calculateDrawingUserWinnings`, which executes `TicketComboTracker.countTierMatchesWithBonusball`. This function iterates from 1 to `bonusballMax` and, for each bonus ball, iterates through all 31 subsets of the winning normal numbers to check `comboCounts`. 

Snippet:
`for (uint8 i = 1; i <= _tracker.bonusballMax; i++) { ... for (uint256 l = 0; l < subsets.length; l++) ... }`

Each inner iteration reads from storage (`comboCounts[i][subset]`). With `bonusballMax` up to 255 and 31 subsets, this results in nearly 8,000 distinct storage reads. At 2,100 gas per cold SLOAD, this costs ~16.8 million gas, plus logic overhead. This can approach or exceed block gas limits (e.g., 30M), causing the settlement transaction to revert. Since the callback is automated by Pyth, the jackpot becomes permanently stuck if the gas cost exceeds the limit.

## Impact
Protocol Denial of Service. The jackpot cannot settle, locking user funds and LP capital.

## Command to Run Test


## Proof of Concept
1. `prizePool` grows, `bonusballMax` scales to 255.
2. `runJackpot` is called. Entropy requested.
3. Pyth calls `scaledEntropyCallback`.
4. Execution hits loop: 255 * 31 = 7905 iterations.
5. Each reads distinct storage slot. Total gas > 17M.
6. Transaction fails OOG (Out of Gas).

## Proof of Code
function testGasDoS() public { // Simulate loop gas cost in Foundry vm.gasMeter start/end }

## Suggested Mitigation
Optimize the data structure to avoid iterating all bonus balls (e.g., store aggregate counts) or hard-cap `bonusballMax` to a safe limit (e.g., 100) that fits comfortably within block gas limits.


## [H-14]. Insolvency due to failure to decrement `lpEarnings` in `emergencyRefundTickets`

## id: 3GioetmIelMOvLQ-B2sUJ

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: Valid
### Finding Status Justification: Jackpot.emergencyRefundTickets transfers USDC out but does not decrement drawingState[currentDrawingId].lpEarnings (which was increased on buyTickets) nor adjust globalTicketsBought. If settlement later occurs (e.g., due to a delayed entropy callback that is currently allowed, or via disabling emergency), processDrawingSettlement will use stale lpEarnings, inflating LP accounting relative to actual USDC. This can lead to insolvency/withdrawal failures. No code-level safeguard prevents this accounting mismatch.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `emergencyRefundTickets` function allows users to get a refund during emergency mode. It transfers USDC from the contract to the user. However, it fails to decrement `drawingState[currentDrawingId].lpEarnings` or `globalTicketsBought`. 

If the emergency is resolved (`disableEmergencyMode`) and the jackpot is resumed (`runJackpot` -> callback), the settlement logic in `processDrawingSettlement` uses the stale `lpEarnings` value. This value includes the revenue from tickets that were already refunded. Consequently, the `newLPValue` (and thus `lpPoolTotal`) is calculated as if the contract still holds the refunded funds. The system becomes insolvent, tracking more USDC liabilities than assets.

## Impact
Protocol insolvency. Users and LPs will be unable to withdraw funds as the contract balance will be insufficient to cover the tracked `lpPoolTotal`.

## Command to Run Test


## Proof of Concept
1. Users buy 100k USDC worth of tickets. `lpEarnings` = 100k. Contract Balance = 100k.
2. Owner enables emergency mode.
3. Users call `emergencyRefundTickets` for all tickets. Contract Balance = 0.
4. Owner disables emergency mode and runs jackpot.
5. Settlement calculates `postDrawLpValue` using `lpEarnings` (100k).
6. `lpPoolTotal` increases by ~100k (minus winnings).
7. LPs try to withdraw. Contract has 0 USDC but `lpPoolTotal` says 100k. Revert.

## Proof of Code
// Mock sequence inside test
vm.startPrank(user);
jackpot.buyTickets(tickets, user, ...);
vm.stopPrank();
vm.prank(owner);
jackpot.enableEmergencyMode();
vm.prank(user);
jackpot.emergencyRefundTickets(ticketIds);
// assert(jackpot.getDrawingState(id).lpEarnings > 0); // Earnings not decreased

## Suggested Mitigation
Decrement `drawingState[currentDrawingId].lpEarnings` by the refunded amount (excluding referral fees if they were not refunded) inside `emergencyRefundTickets`.


## [H-15]. Emergency refunds cause protocol insolvency and payout dilution if drawing resumes

## id: myfV21EulUkzdUxBAdDu-

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: Valid
### Finding Status Justification: emergencyRefundTickets burns NFTs and refunds USDC but does not remove refunded tickets from TicketComboTracker or adjust lpEarnings/globalTicketsBought. If an entropy callback later executes (it is not blocked in emergency mode), settlement and payout-table computation will still treat refunded/burned tickets as sold winners, diluting per-ticket payouts for remaining valid ticket holders and mis-accounting lpEarnings vs actual funds. This can create insolvency or incorrect payouts/locked withdrawals even without a deliberate ‘resume’ action, due to delayed callback execution.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `emergencyRefundTickets` function allows users to get a refund for tickets in the current drawing when Emergency Mode is enabled. This function burns the ticket NFT and transfers USDC back to the user. However, it fails to decrement `lpEarnings`, `globalTicketsBought`, or remove the ticket from `TicketComboTracker`. 

If the emergency is resolved and the owner calls `disableEmergencyMode`, the drawing can proceed via `runJackpot`. During settlement (`processDrawingSettlement`), the `lpEarnings` variable will still include the value of the refunded tickets. The system will calculate the new LP pool value (`newLPValue`) assuming it holds these funds, inflating the share price (`newAccumulator`). Additionally, the refunded tickets remain in the `TicketComboTracker`, so they are counted as 'sold' when calculating the total winners denominator in `GuaranteedMinimumPayoutCalculator`, diluting the payouts for valid ticket holders. Finally, since the contract no longer holds the refunded USDC, any attempt by LPs to withdraw their inflated balances or by winners to claim payouts will eventually revert due to insufficient USDC balance, rendering the protocol insolvent.

## Impact
The protocol becomes insolvent (liabilities > assets) and LPs are credited with non-existent profit. Valid winners receive diluted payouts. Withdrawals revert.

## Command to Run Test


## Proof of Concept
1. LPs deposit 100,000 USDC. LP Pool = 100k.
2. Users buy 50,000 USDC worth of tickets. `lpEarnings` = 50k. Contract Balance = 150k.
3. Admin enables Emergency Mode.
4. Users call `emergencyRefundTickets` for all tickets (50k). Contract Balance = 100k. `lpEarnings` remains 50k.
5. Admin disables Emergency Mode.
6. Keeper calls `runJackpot`, drawing completes.
7. `processDrawingSettlement` calculates `newLPValue` = 100k (Pool) + 50k (Earnings) = 150k.
8. `newAccumulator` updates based on 150k value.
9. LPs try to withdraw their shares (valued at 150k). Transaction reverts because Contract Balance is only 100k.

## Proof of Code
function testEmergencyRefundInsolvency() public {
    // Setup: LP deposits and Ticket buys
    vm.startPrank(lp);
    usdc.approve(address(jackpot), 100000e6);
    jackpot.lpDeposit(100000e6);
    vm.stopPrank();
    
    vm.startPrank(user);
    usdc.approve(address(jackpot), 50000e6);
    IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
    uint8[] memory numbers = new uint8[](5);
    for(uint8 i=0; i<5; i++) numbers[i] = i+1;
    tickets[0] = IJackpot.Ticket(numbers, 10);
    // Buy enough tickets to equal 50k
    // ... (simplified for PoC text)
    jackpot.buyTickets(tickets, user, new address[](0), new uint256[](0), bytes32(0));
    vm.stopPrank();

    // Emergency Refund
    vm.startPrank(owner);
    jackpot.enableEmergencyMode();
    vm.stopPrank();

    vm.startPrank(user);
    uint256[] memory ticketIds = new uint256[](1); ticketIds[0] = ...;
    jackpot.emergencyRefundTickets(ticketIds);
    vm.stopPrank();

    // Resume
    vm.startPrank(owner);
    jackpot.disableEmergencyMode();
    vm.stopPrank();

    // Run Jackpot
    vm.warp(block.timestamp + duration + 1);
    jackpot.runJackpot{value: fee}();
    // Simulate callback...
    
    // Assert Insolvency
    // LP withdraws checks balance, reverts.
}

## Suggested Mitigation
In `emergencyRefundTickets`, decrement `lpEarnings` by the refunded amount (excluding referral fees which were already paid/deducted). Addressing the `TicketComboTracker` inconsistency is harder; consider marking the drawing as 'cancelled' effectively preventing `runJackpot` for that ID, or requiring a full reset of the drawing state if refunds occur.


## [M-16]. LP Share Precision Loss Due to Accumulator Inflation

## id: deTOwMgEl45Aq3sFvwv1I

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: Valid
### Finding Status Justification: There is no guard that enforces a minimum non-dust lpPoolTotal or a minimum shares-minted constraint; if lpPoolTotal becomes extremely small (but nonzero) the accumulator can inflate enough that realistic USDC deposits mint 0 shares, which is not low impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager` calculates `newAccumulator` based on the ratio of `postDrawLpValue` to `lpPoolTotal`. If `lpPoolTotal` becomes extremely small (e.g., due to withdrawals leaving dust), even a small amount of `lpEarnings` (ticket sales) can cause the accumulator to grow massively (e.g., from 1e18 to 1e30). Subsequent deposits are converted to shares using `deposit * 1e18 / accumulator`. If the accumulator is sufficiently large, this division yields 0 shares for reasonable deposit amounts, causing users to lose their deposits immediately.

## Impact
Loss of user deposits due to precision loss.

## Command to Run Test


## Proof of Concept
1. `lpPoolTotal` is reduced to 1 wei via withdrawals. 2. A ticket is bought for 10 USDC (10e6 wei). 3. `postDrawLpValue` ≈ 10e6. 4. `newAccumulator` = `1e18 * 10e6 / 1` = `1e24`. 5. User deposits 100 USDC (1e8 wei). 6. `shares` = `1e8 * 1e18 / 1e24` = 0. 7. User gets 0 shares for 100 USDC.

## Proof of Code
function testAccumulatorInflation() public { 
    // Drain LP pool to 1 wei 
    // Buy ticket to inflate earnings 
    // Settle drawing 
    // New LP deposits, gets 0 shares 
}

## Suggested Mitigation
Enforce a minimum `lpPoolTotal` threshold or reset the accumulator if liquidity drops below a safe level.


## [H-17]. LP Edge Violation due to uint8 casting truncation in bonusball calculation

## id: EqAKuy52AVtoNxC-E7_qB

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
RoundingError

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: `_setNewDrawingState` uses unchecked `uint8(...)` truncation. With prizePool/lpValue growth beyond the intended cap path, computed bonusball requirements can exceed 255 and wrap, so this is not merely rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the new `bonusballMax` is calculated to ensure the LP edge is preserved. The calculation `Math.ceilDiv(minNumberTickets, combosPerBonusball)` returns a `uint256`. However, this result is explicitly cast to `uint8` without overflow checking. If the calculated difficulty (bonus balls required) exceeds 255, the cast truncates the higher bits (modulo 256). 

For example, if `normalBallMax` is 10 (combos=252) and `prizePool` requires 65,000 tickets to maintain edge, the required bonus balls would be ~258. The cast converts this to `2`. The game difficulty drops by a factor of ~129, causing the win probability to skyrocket far beyond what the ticket price covers. LPs will face massive negative EV and the pool will likely be drained by winners.

## Impact
Catastrophic loss of LP funds due to drastically reduced game difficulty.

## Command to Run Test


## Proof of Concept
1. Deploy Jackpot with `normalBallMax` = 10. 
2. Buy enough tickets or seed LP such that `prizePool` > 65,000 USDC (assuming $1 ticket). 
3. Run jackpot. 
4. `scaledEntropyCallback` calculates next drawing parameters. 
5. `minNumberTickets` ~ 86,000. `combos` = 252. 
6. `ceilDiv` = 344. 
7. `uint8(344)` = 88. 
8. Intended difficulty: 344. Actual: 88. 
9. LPs are exposed to 4x higher win rate than accounted for.

## Proof of Code
function testTruncation() public { uint256 res = 300; uint8 casted = uint8(res); assertEq(casted, 44); }

## Suggested Mitigation
Check if the calculated bonus ball count exceeds 255. If so, cap `normalBallMax` or `bonusballMax` appropriately, or revert to prevent creating a broken drawing. Ideally, use a larger type for `bonusballMax` if possible, but storage limits apply.


## [M-18]. DoS and ticket corruption due to bonusball overflow in Jackpot._setNewDrawingState

## id: KYQ9ZSpWHn_bK-fr_Z0e9

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: There is no enforcement that `bonusballMax <= 255 - normalBallMax` (packing safety). As the pool grows, `bonusballMax` can exceed the safe range, making shift>=256 reachable; not clearly rare in a successful system.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_setNewDrawingState` function dynamically calculates `bonusballMax` based on the `prizePool` size to ensure a mathematical edge for LPs. The formula `bonusballMax = ceil(minNumberTickets / combosPerBonusball)` scales linearly with the prize pool. 

However, the system relies on bit-packing in `TicketComboTracker` where the bonus ball is stored at bit position `bonusball + normalBallMax`. The maximum bit position is 255. If `prizePool` grows sufficiently large (e.g., via high ticket sales or large LP deposits), `bonusballMax` can exceed `255 - normalBallMax`. 

When this occurs, `1 << (bonusball + normalMax)` in `TicketComboTracker.insert` overflows (results in 0 for shift >= 256), effectively erasing the bonus ball selection from the packed ticket. This corrupts ticket storage, causing collisions between tickets with different bonus balls and potentially bricking the drawing mechanism or preventing valid claims.

## Impact
Corruption of ticket data, inability to distinguish winning tickets, and Denial of Service for drawings with large pools.

## Command to Run Test


## Proof of Concept
1. Assume `normalBallMax` = 35. Max safe `bonusballMax` is 220.
2. `combosPerBonusball` = 324,632.
3. `ticketPrice` = 1 USDC. `lpEdgeTarget` = 20%.
4. Required `minNumberTickets` to overflow = 221 * 324,632 = ~71.7 million.
5. If `prizePool` > 71.7M USDC (feasible for large jackpots), `newBonusball` is set to 221.
6. Next drawing starts with `bonusballMax` = 221.
7. User buys ticket with bonus ball 221.
8. `insert` calculates shift: `221 + 35 = 256`.
9. `1 << 256` is 0. The bonus ball bit is lost.
10. `unpackTicket` fails or returns incorrect data.

## Proof of Code
function testBonusballOverflow() public {
    // Mock large prize pool via LP deposit logic or direct state manip
    // assert(newBonusball + normalMax > 255);
    // Ticket purchase should fail or store corrupt data
}

## Suggested Mitigation
Cap `newBonusball` at `255 - normalBallMax` inside `_setNewDrawingState`.


## [H-19]. Uncapped lpEarnings allow bonusballMax to exceed bit-packing limits, breaking game fairness via guaranteed matches

## id: j_JX9kw-Yijlnk-htKByu

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: `lpPoolCap` constrains deposits but not `lpEarnings` (ticket revenue). Over time, revenue can push `_newLpValue`/prizePool high enough to require `bonusballMax` beyond packing-safe bounds, so this is not a rare edge case.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol calculates a dynamic `bonusballMax` for each drawing to ensure a minimum LP edge, scaling with the `prizePool`. The `lpPoolCap` parameter is intended to limit the pool size such that `bonusballMax + normalBallMax < 256`, preventing bit-packing overflows in `TicketComboTracker`. However, `lpPoolCap` only restricts `lpDeposit`. It does not restrict `lpEarnings` (from ticket sales), which are added to the LP pool. If `lpEarnings` push the `prizePool` beyond the safety threshold calculated in `_calculateLpPoolCap`, the computed `bonusballMax` for the next drawing will create a sum `bonusballMax + normalBallMax >= 256`. 

In `TicketComboTracker.insert` and `countTierMatchesWithBonusball`, the bit-packing logic `1 << (_bonusball + _tracker.normalMax)` will overflow (shift >= 256), resulting in 0. This causes all tickets with bonus balls in the overflow range to map to a ticket with NO bonus ball bit set. If the winning bonus ball also falls in this range, it triggers a guaranteed bonus ball match for any ticket in the overflow range. An attacker can fill the pool to the cap, buy tickets to push it over via earnings, and then exploit the guaranteed matches in the next drawing to drain the pool.

## Impact
Game fairness is broken; attackers can guarantee bonus ball matches, dramatically increasing their odds and draining the prize pool.

## Command to Run Test


## Proof of Concept
1. Initialize Jackpot with `normalBallMax` = 35. This sets `lpPoolCap` based on a max `bonusballMax` of 220 (since 220+35=255).
2. LPs deposit funds up to `lpPoolCap`.
3. Users (or attacker) buy tickets, adding `lpEarnings` to the pool. `lpEarnings` are not checked against the cap.
4. Settlement occurs. `newLPValue` > `lpPoolCap`. `prizePool` increases accordingly.
5. `_setNewDrawingState` calculates `newBonusballMax`. Due to the increased pool, it calculates `newBonusballMax` = 221.
6. `newBonusballMax` (221) + `normalBallMax` (35) = 256.
7. In the next drawing, an attacker buys a ticket with `bonusball = 221`. `packTicket` computes `1 << 256`, which overflows to 0. The ticket is stored as just the normal numbers.
8. If the winning bonus ball is also 221 (or any other overflow value), the winning ticket is also computed with 0 for the bonus part.
9. The attacker's ticket matches the winning ticket's bonus component (both 0), securing a guaranteed bonus match.

## Proof of Code
function testOverflow() public {
    // Simulate state
    uint8 normalMax = 35;
    uint8 bonusMax = 221;
    // Overflow condition
    assert(uint256(normalMax) + bonusMax >= 256);
    // Bit packing fail
    uint256 packed = uint256(1) << (normalMax + bonusMax);
    assertEq(packed, 0);
}

## Suggested Mitigation
In `Jackpot._setNewDrawingState`, strictly cap the calculated `newBonusballMax` such that `newBonusballMax + normalBallMax < 256`. Specifically: `newBonusballMax = uint8(Math.min(newBonusballMax, 255 - normalBallMax));`.


## [H-20]. Unbounded prize pool growth causes bit vector overflow and corruption of winner selection

## id: EPDtiW8ChvheAoeRJf2Mg

## Derived From Pattern/Invariant
Dos

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Prize pool/lpValue can grow via ticket revenue and rollovers without a corresponding on-chain enforcement of `bonusballMax + normalBallMax < 256`. This makes eventual overflow/corruption plausible rather than rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol relies on bit vectors in `TicketComboTracker` to track tickets and calculate winners. The position of the bonus ball in the bit vector is determined by `1 << (bonusball + normalBallMax)`. Since EVM shifts of >= 256 result in 0, the sum `bonusball + normalBallMax` must be strictly less than 256. 

The `bonusballMax` for a drawing is calculated dynamically based on the prize pool size to maintain LP edge. While `lpDeposit` enforces a pool cap, `lpEarnings` (from ticket sales and rollovers) are added to the pool without checking this cap. A successful lottery with rollovers can grow the prize pool significantly, causing `bonusballMax` to increase until `bonusballMax + normalBallMax >= 256`.

When this threshold is reached, tickets with high bonus balls map to a shift of 0. This corrupts the `winningTicket` calculation (treating high bonus balls as bonus ball 0 or 'phantom' matches) and potentially reverts `unpackTicket` calls, leading to a denial of service or incorrect payouts.

## Impact
If the pool grows large enough, the drawing logic breaks. Winning number calculation becomes corrupt (phantom matches), and ticket processing functions may revert, effectively freezing the protocol or paying out incorrectly.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 50.
2. Through ticket sales and rollovers, `prizePool` grows large enough that the dynamic difficulty calculation sets `bonusballMax` to 210.
3. Sum 210 + 50 = 260 >= 256.
4. A user buys a ticket with bonus ball 210.
5. `TicketComboTracker` calculates shift `1 << 260` -> 0.
6. The ticket is effectively stored with 'phantom' characteristics.
7. If the winning number is also 210, it also maps to 0. It matches any other ticket that mapped to 0 (e.g. bonus ball 209), causing incorrect payouts.

## Proof of Code
// Pseudo-code simulation of condition
function test_BitVectorOverflow() public {
    uint8 normalMax = 50;
    // Simulate huge prize pool via manipulation or growth
    // ...
    // dynamic calculation results in bonusballMax = 210
    uint256 shift = 1 << (210 + normalMax);
    assertEq(shift, 0); // Overflow confirmed
}

## Suggested Mitigation
In `_setNewDrawingState`, constrain the calculated `newBonusball` such that `newBonusball + normalBallMax < 256`. Additionally, consider capping `lpEarnings` contribution to the next drawing's parameterization or enforcing a hard cap on `bonusballMax`.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-21]. Permanent freezing of LP funds if reserve ratio is zero and pool is drained

## id: eQw0_2k357eOXKpPf_9uu

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
JackpotLPManager._consolidateDeposits

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: Code path exists: `JackpotLPManager.processDrawingSettlement` sets `drawingAccumulator[_drawingId]` to `(drawingAccumulator[_drawingId-1] * postDrawLpValue) / lpPoolTotal` when `_drawingId>0`. If `postDrawLpValue==0`, accumulator becomes 0. Then `_consolidateDeposits` computes `(_lp.lastDeposit.amount * 1e18) / drawingAccumulator[lastDeposit.drawingId]` and will revert on division by zero, permanently preventing consolidation/withdrawals for deposits made in that drawing. The comment claiming “Accumulators can never be zero” is not enforced. Achieving `postDrawLpValue==0` requires extreme governance settings (e.g., `reserveRatio=0` and `lpEarnings=0` such as `referralFee=100%` with referrers) plus a drawing where users capture the whole prizePool. Thus it is real but strongly configuration-dependent (governance footgun).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager.sol`, the `processDrawingSettlement` function calculates the new share price accumulator. If the LP pool is completely drained (total loss) and the `reserveRatio` is set to 0 (allowed by governance), the `postDrawLpValue` becomes 0, leading to a `newAccumulator` of 0. 

Normally, `accumulator` tracks the share price. If it becomes 0, any subsequent call to `_consolidateDeposits` for deposits made in that 'zeroed' round will revert due to division by zero: `shares = amount * PRECISE_UNIT / drawingAccumulator[...]`. 

LPs who deposited 'pending' funds during the round that went to zero effectively re-capitalized the pool for the *next* round (since `newLPValue` includes `pendingDeposits`), but they can never claim their shares or withdraw because their consolidation logic divides by the zero accumulator of the previous round. Their funds are permanently locked in the pool with no way to attribute ownership.

## Impact
Permanent loss of funds for LPs who deposited during a round where the active pool was drained to zero (requires reserveRatio=0).

## Command to Run Test


## Proof of Concept
1. Admin sets `reserveRatio` to 0.
2. LPs deposit 1M USDC into the active pool (`lpPoolTotal`).
3. Alice deposits 100k USDC as 'pending' for the current drawing (intended for next drawing).
4. Winners claim 100% of the active pool (possible since reserve is 0). `postDrawLpValue` = 0.
5. `processDrawingSettlement` calculates `newAccumulator` = 0.
6. The new drawing starts with `lpPoolTotal` = 100k (Alice's funds).
7. Alice calls `initiateWithdraw` or deposits more. `_consolidateDeposits` is called for her previous deposit.
8. The function attempts `100k * 1e18 / accumulator[zero_round]`.
9. Division by zero reverts transaction. Alice's 100k is stuck in the pool forever, effectively donated.

## Proof of Code
function testZeroAccumulatorBrick() public {
    // Mock scenario logic
    uint256 lpPoolTotal = 1000e6;
    uint256 lpEarnings = 0;
    uint256 userWinnings = 1000e6; // Full drain
    uint256 protocolFee = 0;
    uint256 prevAccumulator = 1e18;

    uint256 postDrawLpValue = lpPoolTotal + lpEarnings - userWinnings - protocolFee;
    // newAccumulator calculation from JackpotLPManager
    uint256 newAccumulator = (prevAccumulator * postDrawLpValue) / lpPoolTotal;
    
    assertEq(newAccumulator, 0);

    // Consolidation logic
    uint256 depositAmount = 100e6;
    // This line reverts
    // uint256 shares = (depositAmount * 1e18) / newAccumulator;
}

## Suggested Mitigation
Ensure `newAccumulator` never drops to absolute zero (e.g., verify `postDrawLpValue > 0` or enforce a minimum `reserveRatio` > 0). Alternatively, handle the zero accumulator case in consolidation by assigning 0 shares or handling the total loss scenario explicitly.


## [M-22]. Jackpot Denial of Service via Entropy Provider Key Collision in ScaledEntropyProvider

## id: VvEvOI6ZnOukL5Krfz1UQ

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: `ScaledEntropyProvider` stores `pending` requests in `mapping(uint64=>PendingRequest)` keyed only by `sequence`. Owner can call `setEntropyProvider` at any time. Because `requestAndCallbackScaledRandomness` is permissionless, an attacker can spam requests after the switch until the new provider’s `sequence` equals an existing in-flight `sequence` from the old provider, overwriting `pending[sequence]`. When the old provider fulfills, `entropyCallback` will execute the attacker’s stored callback, and the Jackpot’s callback won’t run, leaving `Jackpot` locked (requires admin action/emergency to recover). There is no keying by provider address, nor a guard preventing provider changes while requests are pending.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `ScaledEntropyProvider` contract stores pending entropy requests in the `pending` mapping, keyed solely by the `sequence` number returned by the underlying entropy provider. If the contract owner changes the entropy provider using `setEntropyProvider` while a request (e.g., from `Jackpot.runJackpot`) is pending, the new provider may issue a sequence number that collides with the existing pending request key. An attacker can intentionally trigger this collision by making permissionless requests to the new provider until the sequence matches. When the original provider returns randomness for the Jackpot's request, the callback will execute using the attacker's stored context (callback address and selector) instead of the Jackpot's. Consequently, the Jackpot's `scaledEntropyCallback` is never triggered, leaving the Jackpot in a permanently locked state (`jackpotLock = true`) and preventing any future drawings until administrative intervention.

## Impact
Permanent Denial of Service of the Jackpot protocol; the current drawing cannot be settled and no new drawings can start.

## Command to Run Test


## Proof of Concept
1. The Jackpot contract calls `runJackpot`, which triggers `ScaledEntropyProvider.requestAndCallbackScaledRandomness`. It receives sequence number `S` (e.g., 100) from the current Entropy Provider A.
2. The `pending[100]` mapping is updated with Jackpot's callback details.
3. The Admin calls `setEntropyProvider` to switch to Provider B.
4. An attacker calls `requestAndCallbackScaledRandomness` repeatedly using Provider B. Since Provider B is fresh (or has a different sequence stream), it eventually issues sequence number `100`.
5. The `pending[100]` mapping is overwritten with the Attacker's callback details.
6. Provider A fulfills the Jackpot's request with sequence `100`.
7. `ScaledEntropyProvider.entropyCallback` retrieves `pending[100]` (Attacker's data) and executes the callback to the Attacker.
8. The Jackpot's `scaledEntropyCallback` is never called. The Jackpot remains locked indefinitely.

## Proof of Code
pending

## Suggested Mitigation
Modify the `pending` mapping to include the entropy provider address in the key, e.g., `mapping(address => mapping(uint64 => PendingRequest)) pending;`, and ensure `entropyCallback` uses the correct provider's mapping.


## [H-23]. Protocol Insolvency and DoS due to unchecked Referral Fee exceeding LP Edge

## id: tac8zBSLxSm1gibWYK6BD

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot.setReferralFee

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: Under misconfiguration (`referralFee > lpEdgeTarget`), users can permissionlessly amplify LP losses via duplicate buys, so it's not 'not exploitable'. It's not 'user mistake' (owner sets the parameters). Insolvency/settlement revert is not low impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The protocol fails to validate that `referralFee` is less than or equal to `lpEdgeTarget`. In `Jackpot.sol`, the system logic assumes that ticket sales cover both the prize pool allocation and the LP profit margin. Specifically, when a duplicate ticket is purchased, `prizePool` is increased by `ticketPrice * (1 - lpEdgeTarget)` to preserve the LP edge on paper. However, the actual revenue added to the system's `lpEarnings` is `ticketPrice * (1 - referralFee)`. 

If `referralFee > lpEdgeTarget`, the liability added to the prize pool (and potential payouts) exceeds the actual revenue collected from the ticket. This mathematically guarantees a loss for the LP pool on every duplicate ticket purchased. An attacker can exploit this by purchasing a large number of duplicate tickets, effectively draining the LP pool's value. If `lpPoolTotal + lpEarnings` falls below the required payouts (`userWinnings`), the `processDrawingSettlement` function in `JackpotLPManager` will revert due to underflow, permanently causing a Denial of Service (DoS) for the drawing settlement.

## Impact
Insolvency of the LP pool and permanent DoS of the drawing process.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` to 20% (2000 bps) and `lpEdgeTarget` to 10% (1000 bps). 
2. Attacker buys one ticket. 
3. Attacker buys a large number of duplicate tickets of the first ticket. Each duplicate adds 90% of ticket price to the prize pool liability but only contributes 80% of ticket price to LP earnings. 
4. The `postDrawLpValue` calculation in `JackpotLPManager.processDrawingSettlement` is `lpPoolTotal + lpEarnings - userWinnings`. If the duplicate tickets win, the payout liability exceeds the capital available, causing the transaction to revert. Even if they don't win, the LP pool value is eroded over time.

## Proof of Code


## Suggested Mitigation
In `setReferralFee` and `setLpEdgeTarget`, enforce the invariant `referralFee <= lpEdgeTarget`.


## [M-24]. Risk-Free Arbitrage due to Unchecked Referral Fee vs LP Edge Configuration

## id: _lJzGq0zZw_1c52Hr-Ydy

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot.buyTickets

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: With `referralFee > lpEdgeTarget` set by the owner, users can permissionlessly buy duplicates/self-refer and worsen LP economics, so it is not 'not exploitable'. It's not user mistake (it's owner configuration). The impact can be material (negative EV for LPs / solvency pressure), not low.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The protocol allows the admin to set `referralFee` (paid to referrer) and `lpEdgeTarget` (kept by LP) independently. When a duplicate ticket is purchased, `ticketPrice * (1 - lpEdgeTarget)` is added to the prize pool liability, while the LP collects `ticketPrice * (1 - referralFee)`. If `referralFee > lpEdgeTarget`, the LP collects less cash than the liability increases, causing a net loss for the LP pool on every duplicate ticket. An attacker acting as their own referrer can purchase duplicate tickets to extract this value difference risk-free, as they receive the high referral fee + the fair value of the ticket (which is subsidized by the LP loss).

## Impact
LPs are drained of funds risk-free by any user who refers themselves and buys duplicate tickets, provided `referralFee > lpEdgeTarget`.

## Command to Run Test


## Proof of Concept
1. Admin sets `lpEdgeTarget` = 5% and `referralFee` = 15%.
2. Attacker buys a duplicate ticket for 100 USDC, referring themselves.
3. Attacker receives 15 USDC referral fee.
4. Prize pool liability increases by 95 USDC (100 * (1 - 0.05)).
5. LP cash earnings increase by 85 USDC (100 - 15).
6. System Net Change: +85 Cash - 95 Liability = -10 USDC (Loss).
7. Attacker Net: -100 Cost + 15 Fee + 95 Equity = +10 USDC (Risk-free Profit).

## Proof of Code
function testReferralArb() public {
    // Configure fee > edge
    vm.prank(owner);
    jackpot.setLpEdgeTarget(0.05e18);
    vm.prank(owner);
    jackpot.setReferralFee(0.15e18);
    // Execute buy with self-referral logic
    // Assert LP pool total value (cash - liability) decreases
}

## Suggested Mitigation
In `setReferralFee`, enforce that `referralFee <= lpEdgeTarget` to ensure LPs always capture enough value to cover the prize pool liability increase.


## [M-25]. Denial of Service if Referral Fee exceeds LP Edge Target due to settlement underflow

## id: 8hVfvaClgsyGURb78OGoF

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: If the owner configures `referralFee > lpEdgeTarget`, users can permissionlessly worsen solvency via duplicate buying, so it's not 'not exploitable'. It's not user mistake (owner params). A settlement revert that locks the drawing is not low impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager.processDrawingSettlement`, the `postDrawLpValue` is calculated as `lpPoolTotal + lpEarnings - userWinnings - protocolFee`. When a duplicate ticket is purchased, `userWinnings` liability effectively increases by `price * (1 - edge)` (via `prizePool` addition), but `lpEarnings` only increases by `price * (1 - referralFee)`. If `referralFee > lpEdgeTarget`, each duplicate ticket creates a net deficit in the solvency calculation. If enough duplicate tickets win (e.g., during a high payout event), the net negative change can exceed the LP reserve buffer, causing `currentLP.lpPoolTotal + _lpEarnings - _userWinnings` to underflow. This reverts the transaction, bricking the `scaledEntropyCallback` and permanently locking the jackpot.

## Impact
Permanent freezing of the jackpot contract (DoS) requiring emergency mode intervention to rescue funds.

## Command to Run Test


## Proof of Concept
1. Admin sets `lpEdgeTarget` to 5% and `referralFee` to 10%. 
2. Attacker (or users) buys many duplicate tickets for a specific combination. 
3. Each duplicate adds net -5% of ticket price to the system solvency (Assets increase by 90%, Liabilities increase by 95%). 
4. If the combination wins and drains the prize pool, the payout calculation assumes funds exist, but the LP pool calculation subtracts the full payout. 
5. If the deficit exceeds the initial `lpPoolTotal` (reserve), the subtraction underflows and reverts.

## Proof of Code
function testDoSReferralFee() public {
    uint256 lpPool = 1000e6;
    uint256 earnings = 9000e6; // 1000 tickets * 10 usdc * 0.9 (10% ref fee)
    uint256 winnings = 9500e6; // 1000 tickets * 10 usdc * 0.95 (5% edge)
    // lpPool + earnings - winnings
    // 1000 + 9000 - 9500 = 500 (Safe)
    // If duplicates = 3000:
    // earnings = 27000, winnings = 28500
    // 1000 + 27000 - 28500 = Underflow
    // vm.expectRevert();
    // uint256 newLp = lpPool + earnings - winnings;
}

## Suggested Mitigation
Add a validation check in `setReferralFee` and `setLpEdgeTarget` to ensure `referralFee <= lpEdgeTarget`, or adjust the duplicate ticket logic to subtract the referral fee from the prize pool addition.


## [M-26]. Protocol DoS due to Reserve Ratio allowing Accumulator to fall to zero

## id: bGBEytoppaTQUFxqaEWip

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: `JackpotLPManager.processDrawingSettlement` can set `newAccumulator` to 0 when `postDrawLpValue` becomes 0 and `lpPoolTotal>0`. While `reserveRatio=0` alone is not sufficient to make `postDrawLpValue=0` (because normally `lpEarnings>0` when tickets sell), the code allows configurations where `lpEarnings` can be driven to 0 (e.g., `referralFee=100%` with referrers), making a full prizePool payout capable of zeroing the pool and the accumulator. Once an accumulator is 0, deposit consolidation reverts (division by zero) and share/withdraw conversions can produce 0 amounts, effectively bricking LP accounting. No explicit invariant enforcement prevents accumulator=0.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `Jackpot` system allows `reserveRatio` to be configured by the owner (up to `PRECISE_UNIT`). If `reserveRatio` is set to 0 (or a very low value), the `prizePool` can effectively be equal to the entire `lpPoolTotal`. If a drawing results in winnings that claim the entire prize pool, the `postDrawLpValue` (calculated as `lpPoolTotal + earnings - winnings`) can drop to zero (or round to zero if `lpPoolTotal` is small). 

In `JackpotLPManager.processDrawingSettlement`, `newAccumulator` is calculated as `(oldAcc * postDrawLpValue) / lpPoolTotal`. If `postDrawLpValue` is 0, the `newAccumulator` becomes 0. 

Once the accumulator is 0, the system is bricked: 
1. `processDeposit` divides by `drawingAccumulator` to calculate shares. Division by zero reverts all deposits.
2. `processInitiateWithdraw` calculates value as `shares * accumulator`, resulting in 0 withdrawable value for all existing LPs.

## Impact
Protocol enters a permanent denial of service state where no new deposits can be made and existing LPs lose all value.

## Command to Run Test


## Proof of Concept
1. Admin sets `reserveRatio` to 0.
2. `lpPoolTotal` is 100 USDC.
3. Ticket sales are low (low earnings).
4. Users win the jackpot (100 USDC).
5. `postDrawLpValue` = 100 + 0 - 100 = 0.
6. `newAccumulator` becomes 0.
7. Next `lpDeposit` reverts due to division by zero.

## Proof of Code


## Suggested Mitigation
Enforce a minimum `postDrawLpValue` or a minimum `reserveRatio` to ensures `lpPoolTotal` cannot be fully drained by winnings, or handle the `newAccumulator == 0` case in `processDrawingSettlement` by resetting it to `PRECISE_UNIT` or a floor value.





Finding Status: InvalidGovernanceRisk
## [M-27]. Arbitrage opportunity draining LP pool when Referral Fee exceeds LP Edge

## id: JBVDS8kzrdwnDMQZbxV5U

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.sol.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: If `referralFee > lpEdgeTarget` is configured, any user can worsen LP solvency by buying duplicates/self-referring (a permissionless action under that config), so it's not 'not exploitable'. It's also not a 'user mistake' (it’s an owner misconfiguration), and the impact can be material (systematic LP value erosion), not low.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol allows buying duplicate tickets, which adds `ticketPrice * (1 - lpEdgeTarget)` to the prize pool and `ticketPrice * (1 - referralFee)` to LP earnings. If `referralFee > lpEdgeTarget`, the amount added to the prize pool (liability) exceeds the amount added to LP earnings (asset). An attacker can buy duplicate tickets referring themselves, paying `ticketPrice`, receiving `referralFee` back, and increasing the prize pool by more than their net cost. This creates a negative sum game for LPs, draining the pool.

## Impact
LPs lose value on every duplicate ticket purchased. Attackers can drain LP liquidity risk-free.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` = 20%, `lpEdgeTarget` = 10%.
2. Attacker buys duplicate ticket for 100 USDC, referring self.
3. Attacker pays 100, gets 20 back. Net cost 80.
4. LP earnings increase by 80.
5. Prize pool increases by 90 (100 - 10).
6. Net LP position change: +80 (earnings) - 90 (new liability) = -10.
7. Attacker repeats. LP pool drains.

## Proof of Code


## Suggested Mitigation
Enforce `referralFee <= lpEdgeTarget` in the setter functions (`setReferralFee`, `setLpEdgeTarget`) or in `buyTickets` logic.





Finding Status: InvalidByDesign
## [M-28]. Governance DoS via front-running pool cap updates

## id: 7EbIcoUPHroO24O_CB15Y

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
JackpotLPManager.setLPPoolCap

## Finding Status: InvalidByDesign
### Finding Status Justification: This is a permissionless griefing/DoS vector against admin parameter changes, not an admin mistake. Preventing cap-lowering/risk-reduction can be high impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The function `setLPPoolCap` (called directly or via `setTicketPrice`, `setLpEdgeTarget`, `setReserveRatio`, `setNormalBallMax`) reverts if the new cap is lower than the current `lpPoolTotal + pendingDeposits`. 

An attacker (or rational LP) can front-run any governance transaction that intends to lower the cap (or change parameters resulting in a lower cap) by depositing enough funds to exceed the new target cap. This causes the governance transaction to revert, effectively preventing the admin from updating critical protocol parameters.

## Impact
Denial of Service on governance operations; inability to update ticket prices or risk parameters.

## Command to Run Test


## Proof of Concept
1. Admin submits transaction to reduce `ticketPrice` (which reduces `lpPoolCap`).
2. New Cap calculated would be 1,000,000 USDC. Current Pool is 900,000 USDC.
3. Attacker sees tx, flash-deposits (or normal deposits) 100,001 USDC.
4. Total Pool becomes 1,000,001 USDC.
5. Admin tx executes: `_calculateLpPoolCap` returns 1,000,000. `setLPPoolCap` checks `1,000,000 < 1,000,001` and reverts.

## Proof of Code
function testFrontRun() public { 
  // Simulate front-run deposit blocking setTicketPrice 
}

## Suggested Mitigation
Instead of reverting, `setLPPoolCap` should allow the new cap to be set below the current total, effectively disabling new deposits until the pool shrinks (via withdrawals or losses) to fit the new cap.


## [M-29]. Governance DoS via LP Deposit Front-running on Parameter Updates

## id: 6wEvraS-BvFSuXdryJENo

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
Jackpot.setTicketPrice

## Finding Status: InvalidByDesign
### Finding Status Justification: While the revert-on-too-low-cap behavior is documented (by design), the DoS is caused by permissionless LP deposits front-running admin updates, not by privileged-user mistake, so labeling it as governance-risk-from-admin-error is incorrect. Blocking risk-reducing parameter changes can be high impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `setTicketPrice` and `setNormalBallMax` functions in `Jackpot` automatically recalculate the LP Pool Cap via `_calculateLpPoolCap` and update it in `JackpotLPManager`. The `setLPPoolCap` function in `JackpotLPManager` enforces that the new cap must be greater than or equal to the current total deposits (`lpPoolTotal + pendingDeposits`).

This creates a vulnerability where an attacker (or an LP protecting their interest) can front-run a governance transaction intended to lower the ticket price (or normal ball max) by depositing a calculated amount of LP capital. By increasing `pendingDeposits` such that the total exceeds the *new* calculated cap, the attacker forces the governance transaction to revert. This allows LPs to effectively veto economic adjustments that would reduce the pool cap.

## Impact
Denial of Service on critical governance functions. The protocol may be unable to adjust ticket prices or ball ranges downwards to respond to market conditions or risk parameters.

## Command to Run Test


## Proof of Concept
1. Current LP Pool: 1,000,000 USDC. Current Cap: 2,000,000 USDC. Ticket Price: 2 USDC.
2. Admin decides to lower Ticket Price to 1 USDC. This calculates a new Max Prize Pool and results in a new LP Pool Cap of 1,000,000 USDC.
3. Attacker sees the transaction in the mempool.
4. Attacker front-runs with an `lpDeposit` of 100,000 USDC. New Total Deposits = 1,100,000 USDC.
5. Admin transaction executes. It calculates new cap = 1,000,000 USDC.
6. It calls `jackpotLPManager.setLPPoolCap(1,000,000)`.
7. LPManager checks: `1,000,000 < 1,100,000`. Reverts with `InvalidLPPoolCap`.
8. Admin update fails.

## Proof of Code
function test_GovernanceFrontrun() public {
    // Setup
    vm.startPrank(owner);
    jackpot.initializeLPDeposits(10_000_000e6);
    jackpot.initializeJackpot(block.timestamp + 1000);
    jackpot.setTicketPrice(5e6); // High price, high cap
    vm.stopPrank();

    // LP fills up to near the limit
    vm.startPrank(user1);
    usdc.mint(user1, 5_000_000e6);
    usdc.approve(address(jackpot), 5_000_000e6);
    jackpot.lpDeposit(5_000_000e6);
    vm.stopPrank();

    // Admin tries to lower price -> lower cap
    // Calculated cap for price 1e6 is much lower than 5m
    // Attacker ensures deposits > new cap (already satisfied or deposits more)
    
    vm.prank(owner);
    vm.expectRevert(JackpotErrors.InvalidLPPoolCap.selector);
    jackpot.setTicketPrice(1e6); // Should revert
}

## Suggested Mitigation
Modify `setLPPoolCap` to allow setting a cap lower than current deposits. Instead of reverting, the contract should simply disable *new* deposits until withdrawals bring the total below the new cap (which `processDeposit` already enforces).





Finding Status: LowSeverityDueToRareLikelihood
## [M-30]. Total LP pool loss sets accumulator to zero, permanently locking subsequent LP deposits

## id: DRRjBzsKid7u554DcK9Ol

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: In JackpotLPManager.processDrawingSettlement, if postDrawLpValue becomes exactly 0 while currentLP.lpPoolTotal > 0, then newAccumulator = drawingAccumulator[_drawingId-1] * 0 / lpPoolTotal = 0. Later, _consolidateDeposits and _consolidateWithdrawals divide by drawingAccumulator[drawingId], assuming it is non-zero (comments claim it can never be 0). If it is 0, share consolidation will revert (division by zero), potentially bricking LP interactions. While reaching exactly zero is an edge case, the code permits it and lacks a guard against accumulator==0.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The LP share pricing uses an accumulator model. `newAccumulator = (oldAccumulator * postDrawLPValue) / preDrawLPTotal`. If a drawing results in a total loss for LPs (e.g. `userWinnings >= lpPoolTotal + lpEarnings`), `postDrawLPValue` becomes 0, and thus `newAccumulator` becomes 0.

However, pending deposits from that same 'wiped out' drawing are carried over to initialize the *next* drawing's pool (`newLPValue = pendingDeposits`). These pending deposits are the sole capital for the new drawing.

When these LPs (or any LPs interacting with the system) try to consolidate their position later via `_consolidateDeposits`, the function attempts to calculate shares: `amount * 1e18 / accumulator`. Since the accumulator for their deposit drawing was set to 0, this results in a division by zero, reverting the transaction. These LPs are permanently bricked and cannot withdraw or interact.

## Impact
If the LP pool suffers a 100% loss (wipeout), any LPs who deposited during that specific drawing interval are permanently locked out of their funds. Their deposits initialized the next pool, but they cannot claim their shares.

## Command to Run Test


## Proof of Concept
1. LP Pool has 100k USDC.
2. Users win 150k USDC (winnings > pool + earnings). `postDrawLPValue` clamped to 0 (assuming strict solvency checks don't revert, or exact wipeout).
3. During this drawing, a new LP deposits 50k USDC (`pendingDeposits`).
4. Settlement happens. `newAccumulator` becomes 0. `newLPValue` for next round becomes 50k USDC.
5. The new LP tries to `initiateWithdraw` or deposit more.
6. `_consolidateDeposits` runs, looks up `accumulator` for the wipeout round (0), divides by 0, and reverts.

## Proof of Code
function test_LPWipeoutBrick() public {
    // 1. Create wipeout scenario
    // Mock outcome where winnings > pool
    // 2. Deposit during that round
    vm.prank(lp2);
    jackpot.lpDeposit(100e6);
    // 3. Settle round
    // ... settlement logic sets accumulator to 0 ...
    // 4. LP2 tries to interact
    vm.prank(lp2);
    vm.expectRevert(); // Division by zero
    jackpot.initiateWithdraw(1);
}

## Suggested Mitigation
In `processDrawingSettlement`, if `postDrawLpValue` is 0 (or `newAccumulator` results in 0), handle the reset gracefully. If the pool is wiped out, the accumulator for *new* deposits (pending) should likely be reset to `PRECISE_UNIT` or the logic should explicitly handle accumulator=0 as a special case for share issuance.



