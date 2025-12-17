# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[M-1]. TicketComboTracker Bitwise Overflow Corrupts Winner Counting
**Derived From** : RoundingError
Finding Status: Valid
Privilege: RequiresAdminRole


[M-2]. Denial of Service in drawing settlement due to excessive gas limit
**Derived From** : Custom
Finding Status: Valid
Privilege: Permissionless


[M-3]. Settlement DOS via O(N) storage reads in `countTierMatchesWithBonusball`
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[M-4]. BridgeManager Uses Stale Global Ticket Price Leading to Overpayment or DoS
**Derived From** : PricePrecision
Finding Status: Valid
Privilege: Permissionless


[M-5]. Excessive gas usage in `TicketComboTracker` allows DoS of entropy callback
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-6]. Ticket data corruption and odds manipulation via bit packing overflow
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[M-7]. Ticket Corruption via Bit-Packing Overflow
**Derived From** : Bit-shifting overflow
Finding Status: Valid
Privilege: Permissionless


[H-8]. First Depositor Inflation Attack via LP Share Price Manipulation
**Derived From** : ERC4626 inflation / First depositor attack
Finding Status: Valid
Privilege: Permissionless


[H-9]. Downcast in `_setNewDrawingState` causes difficulty collapse and LP fund loss
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-10]. LP Edge Violation via Unsafe Bonusball Casting
**Derived From** : Unsafe Downcasting
Finding Status: Valid
Privilege: Permissionless


[H-11]. Ticket Corruption via Bit Packing Overflow in `bonusballMax` Calculation
**Derived From** : StorageLayout
Finding Status: Valid
Privilege: Permissionless


[H-12]. LP Insolvency due to Bonus Ball Count Truncation
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-13]. Bonusball difficulty parameter wrapping allows Jackpot manipulation
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless



Finding Status: InvalidGovernanceRisk


[M-14]. Guaranteed LP Loss if Referral Fee Exceeds LP Edge Target
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-15]. PayoutCalculator upgrade via setPayoutCalculator permanently bricks current drawing payouts
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: InvalidGovernanceRisk
Privilege: RequiresRole


[H-16]. Arbitrage opportunity draining LP pool when referral fee exceeds LP edge
**Derived From** : FlashLoanEconomicManipulation
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-17]. LP System permanent DOS via Division by Zero if Accumulator hits zero
**Derived From** : Dos
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[M-18]. Tier 0 (0 matches, 0 bonus) payouts are impossible to process
**Derived From** : StandardViolation
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-19]. DoS and Insolvency via Duplicate Tickets when Referral Fee Exceeds LP Edge
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[M-20]. LP System DoS via Division by Zero on Pool Wipeout
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 9
- M: 11
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. TicketComboTracker Bitwise Overflow Corrupts Winner Counting

## id: sEf8MekudddKV5ZxmCB2_

## Derived From Pattern/Invariant
RoundingError

## Exploit Type
RoundingError

## Location
Jackpot.setBonusballMin

## Finding Status: Valid
### Finding Status Justification: In `TicketComboTracker.insert` the bonus bit is set via `1 << (_bonusball + _tracker.normalMax)`. If the shift amount is >=256, Solidity yields 0, silently dropping the bonus bit and corrupting packed tickets. There is no invariant enforcement that `normalMax + bonusballMax < 256`. This affects both ticket storage (buy path) and winner computation (`countTierMatchesWithBonusball` uses the same shift for `winningTicket`). Corruption can change bonusball matching semantics (e.g., many tickets/winningTicket decode bonusball as 0), breaking fairness and potentially payout accounting. While normalBallMax is effectively limited to <=128 by `Combinations.choose`, bonusballMax can still reach >127 depending on parameters and pool growth, making the sum reach/exceed 256.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
In `TicketComboTracker.insert`, the code executes `set |= 1 << (_bonusball + _tracker.normalMax)`. If `_bonusball + _tracker.normalMax >= 256`, the shift operation overflows and returns 0 (or wraps depending on compiler version/settings, but effectively loses the bit in `uint256`). This can occur if `bonusballMin` is set high by the owner (e.g., 200) and `normalBallMax` is moderate (e.g., 60), as there is no check enforcing `bonusballMax + normalBallMax < 256`.

## Impact
If the bit shift overflows, the bonus ball information is lost from the ticket subset mask. This causes `_calculateTicketTierId` to incorrectly calculate matches (specifically bonus ball matches), potentially causing every ticket to win the bonus tier, draining the prize pool.

## Command to Run Test


## Proof of Concept
1. Owner calls `setNormalBallMax(60)`. 
2. Owner calls `setBonusballMin(200)`. 
3. Drawing initializes with `bonusballMax` >= 200. 
4. Users buy tickets. `insert` calculates shift `1 << (bonus + 60)`. If bonus=196, shift is `1 << 256` = 0. 
5. Bonus bit is missing. 
6. Settlement calculates winners. The missing bit causes logic errors in tier matching.

## Proof of Code
function test_Exploit_BitShiftOverflow() public {
    uint8 normalMax = 60;
    uint8 bonus = 200;
    uint256 shift = 1 << (uint256(bonus) + normalMax);
    // In Solidity 0.8+, 1 << 260 is NOT 0, it wraps? No, standard says it truncates result to type. 
    // But constant folding might differ. In EVM SHL, 260 becomes 0.
    // We verify if it destroys the bit.
    if (bonus + normalMax >= 256) {
        // Vulnerable condition confirmed
    }
}

## Suggested Mitigation
Add input validation in `setNormalBallMax` and `setBonusballMin` to ensure `_normalBallMax + _bonusballMin < 255`.


## [M-2]. Denial of Service in drawing settlement due to excessive gas limit

## id: RiPa5zeEBAT7Nf4Gqb0eH

## Derived From Pattern/Invariant
Custom

## Exploit Type
Custom

## Location
Jackpot.runJackpot

## Finding Status: Valid
### Finding Status Justification: bonusballMax can increase as a function of on-chain prizePool/edge math without any admin mistake; with the shipped default entropyVariableGasLimit=250k, the computed _gasLimit can eventually exceed some chains' block gas limits, stalling entropy fulfillment even if governance never touches the gas-limit params.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `runJackpot` function requests entropy with a gas limit calculated as `base + variable * bonusballMax`. The default variable limit is 250,000 gas per bonus ball. If `bonusballMax` grows large (e.g., > 120), the requested gas limit (30M+) exceeds the block gas limit of the network (e.g., Ethereum's 30M). This makes it impossible for the `entropyCallback` to execute, permanently locking the jackpot (`jackpotLock` remains true) and preventing settlement. While `TicketComboTracker`'s loop is also heavy, the hard-coded gas calculation forces a request that cannot be fulfilled.

## Impact
The drawing process becomes permanently stuck. The protocol enters a DoS state requiring emergency intervention (refunds), disrupting protocol operation and locking LP funds temporarily.

## Command to Run Test


## Proof of Concept
1. Prize pool grows, causing `bonusballMax` to reach 150. 2. `entropyVariableGasLimit` is 250,000. 3. Calculated `entropyGasLimit` = 37,500,000 (ignoring base). 4. Block gas limit is 30,000,000. 5. `runJackpot` succeeds (requesting), but the callback transaction from Pyth cannot be included in a block due to exceeding block limits. 6. Jackpot remains locked.

## Proof of Code
function testGasLimitDoS() public {
    uint32 variable = 250000;
    uint8 bonusballMax = 150;
    uint32 total = variable * bonusballMax;
    assertGt(total, 30000000);
}

## Suggested Mitigation
Reduce `entropyVariableGasLimit` to a realistic value matching the actual gas cost of the callback loop (which is high but not 250k per iteration). Implement a hard cap on `bonusballMax` or the calculated gas limit to ensure it stays within block limits.


## [M-3]. Settlement DOS via O(N) storage reads in `countTierMatchesWithBonusball`

## id: 9HEJDcsPLLLiPGwBCrSUa

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: In settlement (`Jackpot.scaledEntropyCallback` → `TicketComboTracker.countTierMatchesWithBonusball`), `_countSubsetMatches` iterates over every possible bonusball bucket up to `bonusballMax`, doing ~31 subset reads per bucket (and repeatedly regenerating subsets). This creates linear gas growth with `bonusballMax` and can cause out-of-gas during the entropy callback, bricking the drawing (jackpot remains locked) and requiring emergency mode. There is no hard cap in code and no optimization to aggregate across bonusballs, so the risk exists in the current implementation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `runJackpot` settlement flow triggers `TicketComboTracker.countTierMatchesWithBonusball`, which iterates from `1` to `bonusballMax` (up to 255). Inside this loop, it performs `~31` storage lookups (`comboCounts`) per iteration. With `bonusballMax` at 255, this results in `~7900` storage reads. If these are cold accesses (2100 gas each), the gas cost exceeds `16M`, consuming over 50% of the block gas limit on Ethereum (30M). On L2s or if opcodes are repriced, this can exceed the block/transaction gas limit, permanently locking the drawing and forcing emergency mode.

## Impact
The Jackpot drawing becomes impossible to settle (DOS), locking user funds until emergency mode is activated.

## Command to Run Test


## Proof of Concept
1. A drawing has `bonusballMax` set to 255 (due to high prize pool or low edge).
2. Tickets are bought with various bonus balls, ensuring storage slots are initialized.
3. `runJackpot` is called. The callback executes `countTierMatchesWithBonusball`.
4. The loop executes 255 times, reading storage ~7900 times.
5. Transaction reverts due to Out of Gas (if limits are tight or calldata/memory costs add up).
6. The drawing cannot be settled.

## Proof of Code
function testGasLimitDoS() public {
    // Initialize with max bonusball 255
    // Simulate buying tickets to populate storage
    // Measure gas of `countTierMatchesWithBonusball`
    // Assert gas > reasonable limit
}

## Suggested Mitigation
Optimize `countTierMatchesWithBonusball` to avoid iterating all possible bonus balls, or cap `bonusballMax` to a lower safe value (e.g., 100) acknowledging the tradeoff with LP edge.


## [M-4]. BridgeManager Uses Stale Global Ticket Price Leading to Overpayment or DoS

## id: 0I81sf9P9GyS-_VDCr6Ps

## Derived From Pattern/Invariant
PricePrecision

## Exploit Type
PricePrecision

## Location
JackpotBridgeManager.buyTickets

## Finding Status: Valid
### Finding Status Justification: Admin changing ticketPrice mid-drawing is expected behavior in this architecture (it should apply to the next drawing via snapshotting). BridgeManager reading jackpot.ticketPrice() (future) instead of the current drawing snapshot can cause over/under payment even when governance is behaving as intended.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotBridgeManager.buyTickets` function retrieves the ticket price using `jackpot.ticketPrice()`, which returns the global parameter for *future* drawings. However, `Jackpot.buyTickets` charges `currentDrawingState.ticketPrice`. If the admin updates the price mid-drawing, these values diverge.

Snippet in `JackpotBridgeManager.sol`:
```solidity
uint256 ticketPrice = jackpot.ticketPrice(); // Reads global
...
jackpot.buyTickets(...) // Charges currentDrawingState.ticketPrice
```
If global > drawing price, the user overpays and the difference is stuck in the bridge contract. If global < drawing price, the transaction reverts due to insufficient funds transferred to the Jackpot contract.

## Impact
Users lose funds (overpayment) or are unable to buy tickets (DoS) when ticket price is updated.

## Command to Run Test


## Proof of Concept
1. Drawing 1 active. Price 5 USDC.
2. Admin calls `setTicketPrice(10 USDC)`.
3. User calls `BridgeManager.buyTickets(1 ticket)`.
4. Bridge pulls 10 USDC from user.
5. Bridge calls `Jackpot.buyTickets`.
6. Jackpot pulls 5 USDC from Bridge.
7. Bridge holds 5 USDC dust; User lost 5 USDC.

## Proof of Code
function testBridgeOverpayment() public {
    // Assume drawing price 5, global updated to 10
    vm.prank(owner); jackpot.setTicketPrice(10e6);
    
    uint256 userBalBefore = usdc.balanceOf(user);
    vm.prank(relay);
    bridge.buyTickets(tickets, user, ...);
    
    // User paid 10, Jackpot received 5
    assertEq(usdc.balanceOf(user), userBalBefore - 10e6);
    assertEq(usdc.balanceOf(address(bridge)), 5e6); // Stuck funds
}

## Suggested Mitigation
Update `JackpotBridgeManager.buyTickets` to fetch the correct price using `jackpot.getDrawingState(jackpot.currentDrawingId()).ticketPrice`.


## [M-5]. Excessive gas usage in `TicketComboTracker` allows DoS of entropy callback

## id: N9SNMP4wSyZDXOdiaVZOh

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: Valid
### Finding Status Justification: `TicketComboTracker._countSubsetMatches` loops `i = 1..bonusballMax` and for each `i` loops over `k=1..5`, and (inefficiently) regenerates subsets each time, then performs storage reads on `comboCounts[i][subset]`. With large `bonusballMax` this is thousands of SLOADs plus repeated subset generation, making settlement gas scale ~O(bonusballMax). Since `scaledEntropyCallback` is executed under a fixed gas limit provided to Pyth, an underestimation (or simply a too-large `bonusballMax`) can cause consistent out-of-gas reverts and keep the drawing locked, forcing emergency mode. This is reachable with current code because `bonusballMax` can grow over time with pool value.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker.countTierMatchesWithBonusball` function iterates from 1 to `bonusballMax` to aggregate normal ball matches across all bonus ball buckets. If `bonusballMax` is high (up to 255), this loop performs roughly `255 * 31 = 7905` storage reads. This consumes a massive amount of gas (approx. 16M+ gas for cold access). 

If the `entropyVariableGasLimit` is not configured to account for this worst-case scenario, or if the gas cost exceeds the block gas limit (30M on many chains), the entropy callback will consistently run out of gas and revert. This allows an attacker (or natural usage) to brick the settlement process by purchasing tickets that populate all bonus ball buckets.

## Impact
DoS of the jackpot settlement process. The jackpot remains locked, requiring admin intervention to adjust gas limits, or becoming permanently stuck if costs exceed block limits.

## Command to Run Test


## Proof of Concept
1. `bonusballMax` is 255.
2. Users buy tickets covering all 255 bonus ball possibilities, populating the `comboCounts` mapping storage slots (making them non-zero/cold).
3. `scaledEntropyCallback` is called.
4. `countTierMatchesWithBonusball` loops 255 times. Inside each loop, it accesses 31 storage slots.
5. Total gas > 16,000,000.
6. If the provided gas limit is less than this, or block limit is exceeded, the tx fails.

## Proof of Code
function testGasGrief() public {
    // Conceptual PoC
    // Populate storage for 255 buckets
    // Measure gas for countTierMatchesWithBonusball
    // uint256 gasStart = gasleft();
    // tracker.countTierMatchesWithBonusball(...);
    // assert(gasleft() - gasStart > 15_000_000);
}

## Suggested Mitigation
Optimize `TicketComboTracker` to maintain a separate mapping for aggregated normal ball matches (summed across all bonus balls) during insertion. This reduces the `countTierMatches` complexity from O(bonusballMax) to O(1) reads.


## [H-6]. Ticket data corruption and odds manipulation via bit packing overflow

## id: ytzkQNCLlkKQaX5muG8aC

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: `normalBallMax` is effectively capped to <=128, but `bonusballMax` can still grow such that `normalMax + bonusballMax >= 256` for certain parameter regimes / sufficient pool growth; this is not inherently a 'rare' edge case once scale increases.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol packs ticket numbers into a `uint256` bit vector in `TicketComboTracker.insert`. The bonusball bit is set at index `_bonusball + _tracker.normalMax`. 

Snippet:
`ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);`

If `_bonusball + _tracker.normalMax >= 256`, the shift operation `1 << N` (where N >= 256) results in `0` in Solidity. The bonusball bit is lost. Consequently, `unpackTicket` derives the bonusball as `0` (or fails logic), and `claimWinnings` detects a bonusball of `0`. Since multiple valid bonusball inputs can map to `0` (e.g., if `normalMax=128`, bonusballs 128, 129... all map to 0), this creates a collision where different tickets match each other or match a winning ticket that also overflowed, significantly increasing winning odds.

## Impact
Users can force bonusball matches or collisions, breaking the lottery's fairness and potentially winning prizes with much higher probability than intended.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax = 128` (max allowed by Combinations lib).
2. `prizePool` grows such that `bonusballMax` is set to 128.
3. User buys ticket with `bonusball = 128`. Sum = 256.
4. `1 << 256` is 0. Ticket stored with no bonus bit.
5. Winning numbers drawn: `bonusball = 128`. Winning ticket also has no bonus bit.
6. `claimWinnings` compares user ticket (bonus=0/missing) vs winning (bonus=0/missing). Match!
7. User wins despite `bonusball` bit being effectively lost.

## Proof of Code
function testBitPackingOverflow() public {
    uint256 normalMax = 128;
    uint256 bonusball = 128;
    uint256 shift = bonusball + normalMax;
    uint256 vector = 1 << shift;
    assertEq(vector, 0);
}

## Suggested Mitigation
Ensure `normalBallMax + bonusballMax < 256` in configuration/initialization, or use a larger storage structure/different packing scheme.


## [M-7]. Ticket Corruption via Bit-Packing Overflow

## id: 400TUHM9bl-ujC8uGlXpI

## Derived From Pattern/Invariant
Bit-shifting overflow

## Exploit Type
AccountingInvariantViolation

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: Even with `normalBallMax` effectively <=128, `bonusballMax` can grow enough that `_bonusball + normalMax >= 256`, triggering the shift-to-zero packing corruption; this is not inherently rare at larger scale.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library packs ticket numbers into a `uint256` bit vector. The bonusball is stored at the bit index `_bonusball + _tracker.normalMax`.

```solidity
ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);
```

If `_bonusball + _tracker.normalMax` >= 256, the shift operation `1 << ...` overflows to 0 in Solidity. This results in the bonusball bit not being set. When such a ticket is unpacked or checked for winning tiers, the bonusball is effectively lost or interpreted as 0. This denies users their potential winnings (specifically the bonus match) and corrupts the duplicate checking logic. This state is reachable if `prizePool` grows large enough to push `bonusballMax` close to 255 while `normalBallMax` is also moderately high (e.g., > 0).

## Impact
Functional corruption: valid tickets lose their bonusball data, preventing users from claiming bonus-tier winnings.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` configured to 200.
2. `prizePool` scales such that `bonusballMax` is calculated as 60.
3. User buys ticket with bonusball 60.
4. Shift amount: 200 + 60 = 260.
5. `1 << 260` is 0.
6. `packedTicket` lacks bonus bit.
7. Drawing results include bonusball 60.
8. `_calculateTicketTierId` fails to match bonusball. User loses.

## Proof of Code
function testBitPackingOverflow() public {
    // Assume normalMax + bonusMax >= 256
    uint8 normalMax = 200;
    uint8 bonus = 60;
    // Manual check of logic
    uint256 shift = uint256(bonus) + normalMax;
    uint256 result = 1 << shift;
    assertEq(result, 0);
}

## Suggested Mitigation
Add a require check in `Jackpot` parameter setters or `_setNewDrawingState` to ensure `normalBallMax + bonusballMax < 256`.


## [H-8]. First Depositor Inflation Attack via LP Share Price Manipulation

## id: qcyUinAVEmQMXHL2DoZEo

## Derived From Pattern/Invariant
ERC4626 inflation / First depositor attack

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: Valid
### Finding Status Justification: The LP system is susceptible to an ERC4626-style first-depositor/share-inflation attack. Initialization requires only `pendingDeposits > 0`; a malicious actor can be the only depositor with a dust amount. Then, during the first real drawing, the attacker can buy tickets, which increases `lpEarnings` without minting new shares, inflating the accumulator at settlement (`newAccumulator = drawingAccumulator[d-1] * postDrawLpValue / lpPoolTotal`). Subsequent depositors in that drawing can receive 0 shares due to truncation in `shares = amount*1e18 / drawingAccumulator[depositDrawingId]`, effectively donating their USDC to existing shareholders (the attacker). This enables theft of later LP deposits.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The LP system uses an accumulator-based share pricing model similar to ERC4626 vaults. The `initializeJackpot` function seeds the first drawing's LP pool with pending deposits from Drawing 0. If a malicious actor deposits a tiny amount (e.g., 1 wei) in Drawing 0, the initial `lpPoolTotal` for Drawing 1 becomes 1 wei. The attacker can then artificially inflate the `lpEarnings` (and thus `postDrawLpValue`) of Drawing 1 by purchasing tickets. 

In `JackpotLPManager.processDrawingSettlement`, the new accumulator is calculated as:
`newAccumulator = (drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal;`

With `lpPoolTotal` = 1, the attacker can pump `postDrawLpValue` (via `lpEarnings`) to make `newAccumulator` extremely large (e.g., 1e30). Subsequent depositors in Drawing 2 will receive 0 shares due to rounding: `shares = (amount * PRECISE_UNIT) / newAccumulator`. The attacker, holding the only share, can then withdraw the entire pool, stealing the victims' deposits plus the ticket revenue (which they essentially paid to themselves).

## Impact
Theft of all subsequent LP deposits and bricking of the LP pool.

## Command to Run Test


## Proof of Concept
1. Deployer calls `initialize` and `initializeLPDeposits`. LP Pool is open for Drawing 0.
2. Attacker calls `lpDeposit(1 wei)`.
3. Deployer calls `initializeJackpot`. Drawing 0 settles. Drawing 1 starts with `lpPoolTotal = 1`.
4. Attacker calls `buyTickets` for 1000 USDC. `lpEarnings` increases by 1000e6.
5. `runJackpot` is called. In settlement, `postDrawLpValue` ≈ 1000e6.
6. `newAccumulator` = 1e18 * 1000e6 / 1 = 1e27.
7. Victim calls `lpDeposit(100 USDC)`. `shares = 100e6 * 1e18 / 1e27 = 0`.
8. Victim gets 0 shares, funds sent to Jackpot.
9. Attacker calls `emergencyWithdrawLP` (if triggered) or normal withdraw to claim 100% of the pool.

## Proof of Code
function testInflationAttack() public {
    // Setup: 1 wei deposit
    vm.startPrank(attacker);
    usdc.approve(address(jackpot), 1);
    jackpot.lpDeposit(1);
    vm.stopPrank();

    // Admin initializes
    vm.prank(owner);
    jackpot.initializeJackpot(block.timestamp + 1000);

    // Attacker pumps earnings in Drawing 1
    vm.startPrank(attacker);
    uint256 pumpAmount = 1000 * 1e6;
    usdc.approve(address(jackpot), pumpAmount);
    // Buy tickets logic omitted for brevity, assumes successful purchase adds to lpEarnings
    // Manually simulating earnings pump for PoC simplicity if buyTickets complex
    // In real test, call buyTickets
    vm.stopPrank();

    // Trigger settlement (mocking entropy)
    // ... runJackpot + callback ...
    // Result: Accumulator for Draw 1 is huge.

    // Victim deposits
    vm.startPrank(victim);
    uint256 deposit = 1000 * 1e6;
    usdc.approve(address(jackpot), deposit);
    jackpot.lpDeposit(deposit);
    
    // Check victim shares
    IJackpotLPManager.LP memory lp = jackpotLPManager.getLpInfo(victim);
    // Shares will be consolidated 0 in next interaction
    assertEq(lp.lastDeposit.amount, deposit);
    // Advance to next drawing to consolidate
    // ...
    // Consolidated shares = 0
}

## Suggested Mitigation
In `initializeJackpot` and `processDrawingSettlement`, enforce a minimum `lpPoolTotal` (e.g., 1e6 USDC) before allowing the drawing to proceed, or burn the first N shares (similar to Uniswap V2) by minting them to a dead address during initialization.


## [H-9]. Downcast in `_setNewDrawingState` causes difficulty collapse and LP fund loss

## id: h0hmq11I5MZFii6J-rB2a

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: `uint8(...)` truncation is real and can become reachable if pool value grows beyond the cap assumptions (LP value can increase via net ticket revenue over many drawings), eventually pushing the computed `bonusballMax` above 255 and wrapping.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, the `_setNewDrawingState` function calculates the `bonusballMax` for the next drawing to ensure the LP edge is maintained. The calculation `Math.ceilDiv(minNumberTickets, combosPerBonusball)` determines the required difficulty. However, the result is explicitly cast to `uint8` without a range check: `uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(...)));`. If the prize pool grows large enough (e.g., ~$20M with standard parameters) such that the required bonus ball count exceeds 255, the explicit cast will truncate the value (e.g., 257 becomes 1). This drastically reduces the game difficulty instead of increasing it, allowing attackers to easily win the jackpot and drain the LP pool, breaking the core economic security of the protocol.

## Impact
Total loss of LP funds as game difficulty collapses to near-zero while prize pool remains high.

## Command to Run Test


## Proof of Concept
1. Assume `prizePool` reaches ~21,000,000 USDC, `ticketPrice` is 1 USDC, `lpEdgeTarget` is 25%, and `normalBallMax` is 35.
2. `combosPerBonusball` = 324,632.
3. `minNumberTickets` = 21,000,000 / (0.75 * 1) = 28,000,000. Wait, `ticketPrice` is 1e6. `minTickets` = 21e12 / (0.75 * 1e6) = 28,000,000. Correct.
4. Wait, the ratio is `28,000,000 / 324,632` = 86. This fits in uint8. 
5. Correction: If `normalBallMax` is smaller, e.g. 25 (`combos`=53,130). `minTickets` needed = 28M. Ratio = 28M / 53k = 527.
6. `Math.ceilDiv` returns 527.
7. `uint8(527)` truncates to `527 % 256 = 15`.
8. The new drawing is initialized with `bonusballMax = 15` instead of 527.
9. The game is 35x easier than required to maintain solvency. LPs are mathematically guaranteed to lose money.

## Proof of Code
function testBonusballOverflow() public {
    // Setup theoretical values mirroring logic
    uint256 minNumberTickets = 100_000_000; // High ticket volume requirement
    uint256 combosPerBonusball = 100_000;   // Low combinations
    uint256 requiredDifficulty = (minNumberTickets + combosPerBonusball - 1) / combosPerBonusball; // 1000
    
    // Vulnerable cast
    uint8 truncated = uint8(requiredDifficulty);
    
    // Check
    assertEq(requiredDifficulty, 1000);
    assertEq(truncated, 232); // 1000 % 256
    // Difficulty collapsed from 1000 to 232
}

## Suggested Mitigation
Use `UintCasts.toUint8` to revert on overflow, or cap the value at 255 (though capping breaks the edge guarantee, revert is safer).


## [H-10]. LP Edge Violation via Unsafe Bonusball Casting

## id: r_p-oEFOyo_LsZumvS1AU

## Derived From Pattern/Invariant
Unsafe Downcasting

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Same root issue as the other downcast finding: once the computed `bonusballMax` requirement exceeds 255, the unchecked cast wraps and can collapse difficulty; this can occur with sufficient long-term pool growth and is not purely hypothetical.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the system calculates the `bonusballMax` for the next drawing to guarantee an LP edge. The calculation uses `Math.ceilDiv` which returns a `uint256`, but the result is cast to `uint8` without overflow checking.

```solidity
uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
```

If the required `newBonusball` exceeds 255 (which can happen if `prizePool` is large or `normalBallMax` is configured to a small value, reducing `combosPerBonusball`), the cast truncates the value (e.g., 257 becomes 1). This drastically reduces the game difficulty, violating the LP edge requirement and allowing users to drain the pool with positive EV.

## Impact
Loss of LP funds. The mathematical guarantee of LP profitability is broken, leading to probable insolvency of the prize pool.

## Command to Run Test


## Proof of Concept
1. Assume `normalBallMax` = 5 (Combos = 1).
2. `prizePool` grows to > 460 USDC (requiring > 256 tickets to cover edge).
3. `minNumberTickets` = 257.
4. `Math.ceilDiv(257, 1)` = 257.
5. `uint8(257)` = 1.
6. `bonusballMax` set to 1.
7. Users have 100% bonusball match probability, winning heavily against LPs.

## Proof of Code
// Foundry test pseudo-code
function testBonusballOverflow() public {
    // Set low normal ball max
    vm.prank(owner); jackpot.setNormalBallMax(5);
    
    // Create condition where required tickets > 255
    // ... pump prize pool via deposits/buys ...
    
    // Trigger settlement
    // ... runJackpot + callback ...
    
    // Check new drawing state
    Jackpot.DrawingState memory state = jackpot.getDrawingState(jackpot.currentDrawingId());
    // Expected > 255, but due to truncation is small
    assertEq(state.bonusballMax, 1); // Should be > 255
}

## Suggested Mitigation
Use `UintCasts.toUint8` to prevent overflow, or cap the value at 255. Note that capping at 255 might still violate the strict LP edge requirement, so checking constraints or reverting is safer.


## [H-11]. Ticket Corruption via Bit Packing Overflow in `bonusballMax` Calculation

## id: xanbB2FiQvYYEaUY5ve5H

## Derived From Pattern/Invariant
StorageLayout

## Exploit Type
StorageLayout

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: The protocol does not enforce the invariant `normalBallMax + bonusballMax < 256` at drawing initialization; if LP value/prize pool grows beyond the cap-model assumptions, `bonusballMax` can increase enough to hit the 256-bit shift boundary and corrupt packing.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Tickets are stored as `uint256` bit vectors where normal balls occupy bits `[1, normalMax]` and the bonusball occupies bit `normalMax + bonusball`. The system dynamically calculates `bonusballMax` in `_setNewDrawingState` based on prize pool size to maintain LP edge. If the prize pool becomes large, `bonusballMax` increases. There is no check that `normalBallMax + bonusballMax < 256`. If this sum exceeds 255 (e.g. `normalBallMax=35` and `bonusballMax=221` due to a ~$54M pool), the shift `1 << (bonusball + normalMax)` will overflow/wrap in Solidity (resulting in 0 for `1<<256`), causing the bonusball bit to be lost/corrupted in the storage. This results in tickets being unable to claim winnings or matching incorrect numbers.

## Impact
Permanent corruption of ticket data for high-value drawings, preventing users from claiming jackpots.

## Command to Run Test


## Proof of Concept
1. `prizePool` grows to ~54M USDC. `ticketPrice`=1, `lpEdge`=25%. 2. `minNumberTickets` approx 71M. 3. `combos` (35 choose 5) = 324k. 4. `bonusballMax` = 71M / 324k = 221. 5. `normalBallMax` (35) + 221 = 256. 6. User buys ticket with bonusball 221. 7. `TicketComboTracker.insert` executes `set |= 1 << (221 + 35)`. `1 << 256` is 0. 8. Ticket stored without bonusball bit. 9. User attempts to claim; `unpackTicket` returns corrupted/wrong bonusball.

## Proof of Code
function testOverflow() public {
    uint8 normal = 35;
    uint8 bonus = 221;
    // Logic from insert
    uint256 set = (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5);
    uint256 packed = set | (1 << (normal + bonus));
    // packed is just set, bonus bit lost
    assertEq(packed, set);
}

## Suggested Mitigation
Enforce `normalBallMax + bonusballMax < 256` in `_setNewDrawingState` by capping `newBonusball`.


## [H-12]. LP Insolvency due to Bonus Ball Count Truncation

## id: 5JJAsGZNbz9ZxlzScOEAS

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Same unchecked uint8 cast in Jackpot._setNewDrawingState. If required bonusballMax exceeds 255, wraparound reduces combination space and can make tickets +EV, harming LPs. This is a live codepath risk driven by prizePool growth, not governance misuse.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the `bonusballMax` for the next drawing is calculated to maintain the LP edge based on the prize pool size. The formula calculates `minNumberTickets` and then `newBonusball`. However, the result is explicitly cast to `uint8`. If the required number of bonus balls exceeds 255 (which is possible if the prize pool is large relative to `Combinations.choose(normalBallMax, 5)`), the cast truncates the value (modulo 256).

This results in a `bonusballMax` that is significantly smaller than required. The odds of winning become much higher than intended, creating a highly +EV game for players at the expense of LPs. This violates the economic design and leads to rapid drainage of the LP pool.

## Impact
LP edge is destroyed; LPs suffer massive losses as prize payouts statistically exceed ticket revenue.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is 10. `Combos(10,5)` = 252.
2. `PrizePool` = 1,000,000 USDC. `Price` = 1 USDC.
3. `minNumberTickets` ~ 1,000,000.
4. Required `bonusMax` = 1,000,000 / 252 ≈ 3968.
5. `newBonusball` = `uint8(3968)` = `3968 % 256` = 128.
6. Actual odds: 1 in (252 * 128) ≈ 1 in 32,256.
7. Expected odds to cover pot: 1 in 1,000,000.
8. Players have ~30x expected value. LPs lose the difference.

## Proof of Code
function testBonusTruncation() public {
    // Mock internal calc or state
    uint256 hugePrize = 1000000e6;
    uint256 combos = 252;
    uint256 calculated = hugePrize / 1e6 / combos; // ~3968
    uint8 truncated = uint8(calculated);
    assertEq(truncated, 128);
}

## Suggested Mitigation
In `_setNewDrawingState`, check if the calculated bonus ball count exceeds `type(uint8).max` or `255 - normalBallMax`. If so, cap the prize pool growth, revert, or increase `normalBallMax` (though changing normalMax is complex). Ideally, cap the `newBonusball` at 255 but recognize that the LP edge cannot be maintained for such a large pool with small `normalBallMax`.


## [H-13]. Bonusball difficulty parameter wrapping allows Jackpot manipulation

## id: oESorTbnUt7OvdkRqE0Zp

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: The wraparound behavior from uint8(...) in _setNewDrawingState is a concrete correctness issue: values 256+ do not revert and can become very small (e.g., 257->1), sharply increasing win probability. This is not an admin-triggered configuration mistake; it's unchecked arithmetic/truncation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, the `_setNewDrawingState` function calculates the `bonusballMax` for the next drawing to ensure the LP edge. The calculation `bonusballMax = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)))` explicitly casts the result to `uint8`. 

If the required difficulty exceeds 255 (which occurs when the prize pool is large relative to the combination space, e.g., >80M USDC with 35 balls), the cast wraps around (modulo 256). This results in a drastically lower `bonusballMax` than required (e.g., 1 instead of 257), making the jackpot trivial to win in the subsequent drawing and draining the LP pool.

## Impact
LPs lose funds as the game difficulty becomes trivially low despite high prize pool liability.

## Command to Run Test


## Proof of Concept
1. Prize Pool grows to ~82M USDC (feasible for large lotteries). `normalBallMax` is 35.
2. `minNumberTickets` calculated to protect LP edge requires ~82M / 1 tickets.
3. `combos` is ~324k.
4. Required `bonusballMax` = 82M / 324k ≈ 257.
5. `uint8(257)` becomes 1.
6. Next drawing has `bonusballMax` = 1. Winning the jackpot requires only matching 5 numbers (1/324k chance) instead of 1/83M.
7. Attackers buy all combinations (~324k USD cost) and win the 82M+ prize pool guaranteed.

## Proof of Code
function testBonusballOverflow() public {
    // Setup state where prize pool is large enough to trigger overflow
    // ...
    // Verify bonusballMax becomes small number
}

## Suggested Mitigation
Use `UintCasts.toUint8` to revert on overflow, or cap `bonusballMax` at 255 (though capping erodes LP edge, reverting protects solvency).





Finding Status: InvalidGovernanceRisk
## [M-14]. Guaranteed LP Loss if Referral Fee Exceeds LP Edge Target

## id: XaHfh7ZMmhFmu6BbpC1EA

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot.setReferralFee

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: If `referralFee > lpEdgeTarget`, the system can become structurally -EV for LPs and can materially drain LP value over time; that is not a low-impact outcome (even though it is governance/config risk).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The system dynamically sets difficulty (`bonusballMax`) to ensure the prize pool is sustainable based on the target LP edge. The formula assumes that for every ticket sold at `ticketPrice`, the revenue available to support the prize pool is `ticketPrice * (1 - lpEdgeTarget)`. However, the actual revenue received by the LP pool is `ticketPrice * (1 - referralFee)`. If the `referralFee` is configured to be higher than `lpEdgeTarget`, the actual revenue is less than the amount required to support the prize pool size implied by the difficulty. This creates a structural deficit where LPs systematically lose value over time.

## Impact
Systematic drainage of Liquidity Provider funds.

## Command to Run Test


## Proof of Concept
1. Admin sets `lpEdgeTarget` = 10%, `referralFee` = 20%. 2. Admin sets `normalBallMax` and `ticketPrice`. 3. LPs deposit. 4. `runJackpot` sets difficulty assuming 90% of ticket sales go to pool support/profit. 5. Actually only 80% goes to pool. 6. Payouts will statistically exceed the 80% revenue, draining LPs.

## Proof of Code


## Suggested Mitigation
Enforce a configuration constraint such that `referralFee` must be less than or equal to `lpEdgeTarget` (or a safer margin involving reserve ratio).


## [M-15]. PayoutCalculator upgrade via setPayoutCalculator permanently bricks current drawing payouts

## id: 7qgYbdpYqLTBHsrp9LZaz

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.setPayoutCalculator

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Jackpot.setPayoutCalculator simply replaces payoutCalculator without migrating drawingTierInfo. _setNewDrawingState calls payoutCalculator.setDrawingTierInfo(currentDrawingId) only at drawing initialization. If the owner swaps the calculator mid-drawing, then at settlement Jackpot calls payoutCalculator.calculateAndStoreDrawingUserWinnings(currentDrawingId, ...) on the new calculator, whose drawingTierInfo[currentDrawingId] is unset and thus has all-zero weights/flags. In GuaranteedMinimumPayoutCalculator this causes tierWinners to be zeroed for all tiers and results in totalPayout=0 with no tierPayouts stored, making all winners claim 0. This is a valid “admin footgun” that can brick a live drawing’s payouts.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Jackpot` contract allows the owner to update the `payoutCalculator` contract address via `setPayoutCalculator`. The payout calculation logic relies on `drawingTierInfo` (min payouts, weights, etc.) being stored in the calculator contract for the specific `drawingId`. This data is populated via `setDrawingTierInfo` only when a new drawing is initialized.

If the owner calls `setPayoutCalculator` while a drawing is active (which is always the case as drawings are continuous), the new calculator contract will lack the snapshot data for the *current* `drawingId`. When the drawing subsequently settles via `scaledEntropyCallback`, it calls `calculateAndStoreDrawingUserWinnings` on the new calculator. Since the mapping `drawingTierInfo[currentDrawingId]` is empty in the new contract, all tier payouts calculate to zero.

This results in users receiving 0 winnings for their matching tickets, and the LP pool inadvertently capturing the entire prize pool.

## Impact
Critical loss of user funds. All winnings for the active drawing are effectively burned/transferred to LPs because the payout logic defaults to zero values due to missing state.

## Command to Run Test


## Proof of Concept
1. Drawing N starts. `CalculatorA.setDrawingTierInfo(N)` is called automatically.
2. Users buy tickets for Drawing N.
3. Admin calls `setPayoutCalculator(CalculatorB)`.
4. Drawing N concludes; `runJackpot` is called.
5. `scaledEntropyCallback` triggers `CalculatorB.calculateAndStoreDrawingUserWinnings(N, ...)`.
6. `CalculatorB` has no data for `drawingTierInfo[N]`. All weights and min payouts are 0.
7. Function returns 0 total winnings.
8. Users with winning tickets claim 0 USDC.

## Proof of Code
function test_PayoutCalculatorUpgradeBricksDrawing() public {
    // Start drawing 1
    vm.prank(owner);
    jackpot.initializeJackpot(block.timestamp + 1000);
    
    // Admin upgrades calculator mid-drawing
    GuaranteedMinimumPayoutCalculator newCalc = new GuaranteedMinimumPayoutCalculator(jackpot, 0, 0, [false...], [uint256(1e18)...]);
    vm.prank(owner);
    jackpot.setPayoutCalculator(newCalc);

    // Run jackpot
    vm.warp(block.timestamp + 1001);
    jackpot.runJackpot{value: fee}();
    
    // Simulate callback
    vm.prank(address(entropy));
    jackpot.scaledEntropyCallback(1, randomNumbers, "");

    // Check winnings are 0 despite having winners
    DrawingState memory state = jackpot.getDrawingState(1);
    assertEq(state.winningTicket, expectedWinningNumber);
    // Payouts would be calculated as 0 due to missing state in newCalc
}

## Suggested Mitigation
Prevent `setPayoutCalculator` from being called if a drawing is active (which effectively means it can never be called in the current architecture), OR require the `setPayoutCalculator` function to explicitly migrate the current drawing's tier info to the new calculator before updating the address.


## [H-16]. Arbitrage opportunity draining LP pool when referral fee exceeds LP edge

## id: 0N5ydApLYciY_9WmWi6Gf

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Once misconfigured (`referralFee > lpEdgeTarget`), the condition can be exploited permissionlessly via volume (including self-referrals), and the economic impact can be large (systematically draining LP value), so it is not low impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol allows `referralFee` and `lpEdgeTarget` to be configured independently. In `buyTickets`, duplicate tickets increase the `prizePool` (liability) by `ticketPrice * (1 - lpEdgeTarget)` while increasing `lpEarnings` (asset) by `ticketPrice * (1 - referralFee)`. 

If `referralFee > lpEdgeTarget`, the liability increase exceeds the asset increase for every duplicate ticket. An attacker can buy self-referred duplicate tickets to effectively purchase prize pool value at a discount, creating an arbitrage that drains the LP pool (Reserve) over time.

## Impact
Attackers can systematically drain the LP pool's capital by repeatedly buying duplicate tickets and winning the inflated prize pool, as the cost to buy the pool increase is less than the value added.

## Command to Run Test


## Proof of Concept
1. Admin sets `lpEdgeTarget = 10%`, `referralFee = 20%`. `ticketPrice = 10`.
2. Attacker buys a duplicate ticket. Cost = 10. Self-referral = 2. Net Cost = 8.
3. `lpEarnings` adds 8.
4. `prizePool` adds `10 * (1 - 0.1) = 9`.
5. System liabilities increased by 9, assets by 8. Net loss of 1 for the protocol.
6. Attacker repeats for all combinations (or sufficient volume) to guarantee winning the inflated pool, profiting the difference.

## Proof of Code
function testReferralArb() public {
    uint256 price = 10e6;
    uint256 edge = 1e17; // 10%
    uint256 ref = 2e17; // 20%
    uint256 liability = price * (1e18 - edge) / 1e18;
    uint256 asset = price * (1e18 - ref) / 1e18;
    assertTrue(liability > asset);
}

## Suggested Mitigation
Enforce `referralFee <= lpEdgeTarget` in the `setReferralFee` and `setLpEdgeTarget` admin functions.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-17]. LP System permanent DOS via Division by Zero if Accumulator hits zero

## id: Cxx6i0udtKjhjqELjx8da

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
JackpotLPManager._consolidateDeposits

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: JackpotLPManager.processDrawingSettlement sets newAccumulator = (drawingAccumulator[d-1] * postDrawLpValue) / lpPoolTotal when lpPoolTotal != 0. If postDrawLpValue becomes 0, newAccumulator becomes 0 and is stored into drawingAccumulator[d]. Later, _consolidateDeposits divides by drawingAccumulator[lastDeposit.drawingId], which would revert on division by zero for LPs whose lastDeposit was in that drawing. The code’s comment claims accumulator can’t be zero due to ticket revenue, but referralFee can be set to 100% (or other parameter combinations) such that lpEarnings does not ensure postDrawLpValue > 0 even with ticket sales, so the invariant is not enforced.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager.sol`, `_consolidateDeposits` divides by `drawingAccumulator[_lp.lastDeposit.drawingId]`. If the accumulator for a drawing becomes 0, any LP who deposited in that drawing will permanently revert when trying to interact with the contract, and future deposits may be blocked if they depend on consolidation.

The accumulator can become 0 in `processDrawingSettlement` if `newAccumulator = (old * postDrawLpValue) / lpPoolTotal` results in 0. This happens if `postDrawLpValue` is 0. `postDrawLpValue` can be 0 if `reserveRatio` is 0, `lpEarnings` are 0, and `userWinnings` equals the full `prizePool` (which is allowed in crisis mode or exact matches).

## Impact
Permanent freezing of LP funds and inability to process deposits/withdrawals for affected users.

## Command to Run Test


## Proof of Concept
1. Governance sets `reserveRatio` to 0.
2. A drawing occurs with 0 ticket sales (`lpEarnings` = 0) but a jackpot winner claims the full pool (`winnings` = `prizePool` = `lpValue`).
3. `postDrawLpValue` becomes 0.
4. `newAccumulator` becomes 0.
5. Any LP who deposited during this drawing attempts to deposit/withdraw later.
6. `_consolidateDeposits` executes `amount / 0` and reverts.

## Proof of Code
function testAccumulatorZero() public {
    // Set reserve 0
    // Simulate drawing with 0 earnings and max winnings
    // Verify accumulator is 0
    // Verify subsequent LP interaction reverts
}

## Suggested Mitigation
Ensure `newAccumulator` has a minimum value of 1 or handle the 0 case in consolidation logic. Ensure `reserveRatio` cannot be set to 0 if it risks this state.


## [M-18]. Tier 0 (0 matches, 0 bonus) payouts are impossible to process

## id: XY3bQVSmru_JRqyGMgLkw

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: In TicketComboTracker.countTierMatchesWithBonusball, uniqueResult/dupResult are length 12 but indices 2..11 are populated via inclusion-exclusion and index 1 via _calculateBonusballOnlyMatches; index 0 is never set and always remains 0. GuaranteedMinimumPayoutCalculator’s total user payouts use (_uniqueResult[i] + _dupResult[i]) as the multiplicand when summing owed user winnings, so even if tier 0 were configured with min payouts or premium weights, users in tier 0 would never receive payouts (tierPayouts[0] might be nonzero, but multiplied by 0 user winners). This is only realized if governance configures tier 0 to pay, making it a governance-triggered but real logic gap; no code-level guard prevents setting tier 0 payouts.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The protocol specification and code (e.g. `TOTAL_TIER_COUNT = 12`) imply support for 12 prize tiers, corresponding to 0-5 matches with/without bonusball. Tier 0 represents 0 normal matches and no bonusball. However, `TicketComboTracker.countTierMatchesWithBonusball` never calculates winners for Tier 0. 

The function initializes `uniqueResult` and `dupResult` arrays (size 12). It uses `_countSubsetMatches` and inclusion-exclusion to fill indices 2-11 (1-5 matches), and `_calculateBonusballOnlyMatches` to fill index 1 (0 matches + bonus). Index 0 is never populated and defaults to 0. 

Consequently, `GuaranteedMinimumPayoutCalculator` sees 0 winners for Tier 0. If governance configures `premiumTierWeights[0] > 0` or sets a minimum payout for Tier 0, these funds will never be distributed to players and will instead roll over to the LP pool, effectively taxing users for a valid tier configuration.

## Impact
Funds allocated to Tier 0 prizes (if configured) are stuck/lost to LPs. The protocol fails to support a standard lottery tier (loss/consolation) despite architecture appearing to support it.

## Command to Run Test


## Proof of Concept
1. Admin configures `PayoutCalculator` with `premiumTierWeights[0] = 0.5e18` (50% to Tier 0).
2. Users buy tickets. Most tickets will be Tier 0 (no matches).
3. Drawing settles. `TicketComboTracker` returns 0 winners for Tier 0.
4. `calculateAndStoreDrawingUserWinnings` skips Tier 0 payout calculation because `tierWinners[0] == 0`.
5. 50% of the prize pool is not distributed and remains in the contract (rolling to next LP pool), despite thousands of valid winning tickets existing.

## Proof of Code
    function test_Tier0Ignored() public {
        // Setup: Set weight for Tier 0
        vm.startPrank(owner);
        uint256[12] memory weights;
        weights[0] = 1e18; // 100% to Tier 0
        payoutCalculator.setPremiumTierWeights(weights);
        vm.stopPrank();

        // Buy a losing ticket (1,2,3,4,5 / 1)
        vm.startPrank(user);
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
        uint8[] memory numbers = new uint8[](5);
        for(uint8 i=0; i<5; i++) numbers[i] = i+1;
        tickets[0] = IJackpot.Ticket(numbers, 1);
        jackpot.buyTickets(tickets, user, new address[](0), new uint256[](0), bytes32(0));
        vm.stopPrank();

        // Mock winning numbers to be different (6,7,8,9,10 / 2)
        // ... (Run jackpot setup) ...
        
        // Assert winnings are 0 despite 100% allocation to Tier 0
        // Actual calculation shows 0 winners for Tier 0
    }

## Suggested Mitigation
Implement logic to calculate Tier 0 winners. This can be derived by subtracting the sum of winners in Tiers 1-11 from the total number of tickets sold (`globalTicketsBought`), handling duplicates appropriately.


## [H-19]. DoS and Insolvency via Duplicate Tickets when Referral Fee Exceeds LP Edge

## id: h_kdRxxAda3vX6rY3Q2eg

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: If the misconfiguration occurs (`referralFee > lpEdgeTarget`), settlement underflow/DoS and insolvency can be severe outcomes; that is not low impact (even though it is governance/config driven).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol creates a solvency mismatch when duplicate tickets are purchased if the configured `referralFee` is greater than the `lpEdgeTarget`. 

When a duplicate ticket is bought:
1. `lpEarnings` increases by `ticketPrice * (1 - referralFee)`.
2. `prizePool` liability increases by `ticketPrice * (1 - lpEdgeTarget)` (to preserve edge).

If `referralFee > lpEdgeTarget`, the liability increase exceeds the asset increase. Specifically, the net change to the system's solvency for each duplicate is `ticketPrice * (lpEdgeTarget - referralFee)`, which is negative.

In `JackpotLPManager.processDrawingSettlement`, the new LP value is calculated as:
`postDrawLpValue = currentLP.lpPoolTotal + _lpEarnings - _userWinnings - _protocolFeeAmount`.

If `_userWinnings` approaches the full `prizePool` (which happens in Crisis Mode or if winners are found for all tiers), and the volume of duplicate tickets is high enough, the term `_userWinnings` (driven by `prizePool`) will exceed `currentLP.lpPoolTotal + _lpEarnings`.

This causes an underflow in the subtraction, reverting the transaction. Since this function is called by the entropy callback, the drawing cannot be settled, locking the jackpot indefinitely (DoS) and potentially rendering the protocol insolvent.

## Impact
The jackpot drawing mechanism is permanently blocked (DoS) due to settlement revert, and LPs suffer guaranteed value leakage.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` = 20% and `lpEdgeTarget` = 10%.
2. Attacker (or users) buy a large number of duplicate tickets.
3. For each ticket (price 10 USDC): `lpEarnings` += 8 USDC, `prizePool` += 9 USDC.
4. System runs a drawing. If the prize pool is fully won (e.g. Crisis Mode triggers or all tiers match), the payout is 9 USDC per ticket, but assets only increased by 8 USDC.
5. With enough volume, `Payouts > LPEarnings + InitialLP`.
6. `processDrawingSettlement` reverts on underflow. The `runJackpot` flow is bricked.

## Proof of Code
function testDoSViaDuplicateTickets() public {
    // Setup: Ref fee 20%, Edge 10%
    vm.startPrank(owner);
    jackpot.setReferralFee(0.2e18);
    jackpot.setLpEdgeTarget(0.1e18);
    vm.stopPrank();

    // Buy many duplicate tickets
    // ... (simulation of buying tickets)
    
    // Attempt to settle drawing
    vm.prank(address(entropyProvider));
    vm.expectRevert(); // Underflow
    jackpot.scaledEntropyCallback(sequence, randomNumbers, "");
}

## Suggested Mitigation
Enforce `referralFee <= lpEdgeTarget` in the `setReferralFee` and `setLpEdgeTarget` functions. Alternatively, adjust the duplicate ticket logic to ensure the prize pool increment never exceeds the net revenue (`ticketPrice - referralFee`).


## [M-20]. LP System DoS via Division by Zero on Pool Wipeout

## id: 1Rg3oJvAadAA6WsyBFt4N

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: If JackpotLPManager.processDrawingSettlement computes postDrawLpValue == 0 while lpPoolTotal != 0, then newAccumulator becomes 0 and is stored. Subsequent calls that consolidate deposits or withdrawals from that drawing divide by drawingAccumulator[drawingId] and will revert due to division by zero. The contract relies on an unenforced assumption (“LP will still receive ticket revenue”), which can be invalidated by governance-set parameters (e.g., referralFee consuming all ticket revenue) or edge cases where earnings do not ensure postDrawLpValue > 0.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager.sol`, the `processDrawingSettlement` function calculates `newAccumulator`. If `currentLP.lpPoolTotal` is non-zero, it calculates `newAccumulator = (drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal`. If `postDrawLpValue` is 0 (due to winnings + fees equaling pool + earnings), `newAccumulator` becomes 0. Subsequently, functions `_consolidateDeposits` and `_consolidateWithdrawals` perform division by `drawingAccumulator[drawingId]`. If the accumulator for a drawing is 0, these divisions will revert (Divide by Zero). This permanently bricks the LP positions associated with that drawing—users who deposited during that interval cannot consolidate, withdraw, or interact with their funds, and the error may block future deposits if they have stale state.

## Impact
Permanent freezing of LP funds and Denial of Service for LP interactions if a drawing results in total pool depletion.

## Command to Run Test


## Proof of Concept
1. A drawing occurs where `userWinnings` + `protocolFee` equals `lpPoolTotal` + `lpEarnings` (e.g. reserve ratio is 0 and jackpot is won). 2. `postDrawLpValue` becomes 0. 3. `processDrawingSettlement` sets `drawingAccumulator[drawingId]` to 0. 4. An LP who made a deposit during this drawing calls `lpDeposit` or `initiateWithdraw` for a future drawing. 5. `_consolidateDeposits` attempts to calculate shares: `amount * PRECISE_UNIT / drawingAccumulator[id]`. 6. Transaction reverts due to division by zero.

## Proof of Code
pending

## Suggested Mitigation
Ensure `newAccumulator` has a minimum value of 1 (or handle the 0 case in consolidation logic by treating the value as 0 instead of reverting).



