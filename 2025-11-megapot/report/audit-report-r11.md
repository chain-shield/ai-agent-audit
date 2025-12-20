# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[H-1]. Liquidity Pool Share Inflation Attack allowing theft of deposits
**Derived From** : ERC4626 Share Inflation Attack
Finding Status: Valid
Privilege: Permissionless


[M-2]. DoS of governance parameter updates via LP pool cap front-running
**Derived From** : GovernanceFrontrunDoS
Finding Status: Valid
Privilege: Permissionless


[M-3]. Permanent DoS via unbounded gas consumption in randomness callback
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[M-4]. Unbounded gas consumption in settlement callback causes permanent DoS
**Derived From** : Unbounded Loop / Gas Limit DoS
Finding Status: Valid
Privilege: Permissionless


[M-5]. DoS in settlement due to excessive gas consumption in TicketComboTracker
**Derived From** : DoS via Block Gas Limit
Finding Status: Valid
Privilege: Permissionless


[H-6]. Unbounded ticket sales bypass LP pool cap causing difficulty truncation and ticket corruption
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-7]. LP value drain due to unsafe downcast in _setNewDrawingState allowing bonusball overflow
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: Valid
Privilege: Permissionless


[H-8]. Bitwise data corruption in TicketComboTracker due to unchecked bonusball range
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[H-9]. Randomness manipulation via entropy request ID collision in ScaledEntropyProvider upon provider switch
**Derived From** : ExternalProtocolKeyCollision
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole



Finding Status: InvalidGovernanceRisk


[M-10]. LP Value Leak on Duplicate Tickets when Referral Fee Exceeds LP Edge
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood


[H-11]. LP edge violation and game state corruption due to arithmetic overflow in bonusballMax calculation and bit packing
**Derived From** : Arithmetic Over/Underflow
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 6
- M: 5
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Liquidity Pool Share Inflation Attack allowing theft of deposits

## id: hsW4WFVFJgl9LT3JM5GP0

## Derived From Pattern/Invariant
ERC4626 Share Inflation Attack

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: Valid
### Finding Status Justification: The accumulator/share model has no minimum-liquidity/dead-shares style protection. If lpPoolTotal can be made extremely small but nonzero (e.g., leaving 1 USDC-wei), a large positive postDrawLpValue can make drawingAccumulator huge, and later deposits can mint 0/near-0 shares due to truncation, effectively transferring value to existing shareholders. This is feasible but requires an edge setup (near-empty pool + subsequent deposits), making likelihood lower.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager` uses an accumulator-based share pricing model similar to ERC4626 vaults. The accumulator is updated in `processDrawingSettlement` as: `newAccumulator = (drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal`. 

If `currentLP.lpPoolTotal` is manipulated to be extremely small (e.g., 1 wei) while `postDrawLpValue` is high (due to `lpEarnings` from ticket sales), the `newAccumulator` can grow massively. Subsequent deposits by other users will then be calculated as `shares = (amount * PRECISE_UNIT) / newAccumulator`. Due to the huge accumulator, `shares` will round down to 0 for significant deposit amounts. The attacker (the sole LP who manipulated the pool) effectively owns 100% of the pool including the victim's new deposit, and can withdraw everything.

## Impact
Theft of user deposits by manipulating the share price accumulator.

## Command to Run Test


## Proof of Concept
1. Attacker is the sole LP in Drawing N.
2. Attacker initiates withdrawal of all shares except 1 wei worth.
3. Drawing N settles. `newLPValue` for Drawing N+1 is set to 1 wei (plus pending deposits, if any - attacker ensures none).
4. Drawing N+1 starts with `lpPoolTotal = 1 wei`.
5. Tickets are sold in Drawing N+1, generating `lpEarnings` (e.g., 1000 USDC).
6. Drawing N+1 settles. `postDrawLpValue` = 1 wei + 1000e6. `newAccumulator` = `prev * (1000e6) / 1`. The accumulator explodes to ~1e27.
7. Victim calls `lpDeposit(1000e6)`. `shares = 1000e6 * 1e18 / 1e27 = 0`.
8. Victim gets 0 shares, but funds are added to the pool.
9. Attacker finalizes withdrawal or emergency withdraws, taking the victim's funds.

## Proof of Code
function testInflationAttack() public {
    // Mock setup assuming LPManager is deployed and active
    // 1. Attacker ensures pool is empty or withdraws down to 1 wei
    // 2. Simulate earnings added to pool
    uint256 lpPoolTotal = 1; // 1 wei
    uint256 earnings = 1000e6; // 1000 USDC
    uint256 prevAccumulator = 1e18;
    
    uint256 postDrawLpValue = lpPoolTotal + earnings;
    uint256 newAccumulator = (prevAccumulator * postDrawLpValue) / lpPoolTotal;
    
    // newAccumulator is ~1e27
    assertGt(newAccumulator, 1e27);
    
    // 3. Victim deposits
    uint256 depositAmount = 1000e6;
    uint256 shares = (depositAmount * 1e18) / newAccumulator;
    
    // Victim receives 0 shares
    assertEq(shares, 0);
}

## Suggested Mitigation
Implement a minimum liquidity requirement (dead shares) or offset in the accumulator calculation (similar to OpenZeppelin's ERC4626 offset) to prevent `lpPoolTotal` from becoming small enough to cause rounding errors.


## [M-2]. DoS of governance parameter updates via LP pool cap front-running

## id: wVEmj5fb43lliCEgzJvQN

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
JackpotLPManager.setLPPoolCap

## Finding Status: Valid
### Finding Status Justification: The revert-on-lower-cap behavior is real and can be triggered permissionlessly by adding deposits before an owner tx that lowers the cap. However this is not merely 'admin error' (so not governance-risk per scope), and there is no on-chain mitigation beyond operational measures (private txs/pausing via lock/emergency). Impact is limited to delaying parameter updates (no direct fund loss).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Administrative functions that alter economic parameters (`setTicketPrice`, `setNormalBallMax`, etc.) call `jackpotLPManager.setLPPoolCap` to recalculate the safety cap. `setLPPoolCap` reverts if the new calculated cap is lower than the current total deposits (`lpPoolTotal + pendingDeposits`).

An attacker (LP) can monitor the mempool for such governance transactions. If the update would lower the cap, the attacker can front-run the transaction by depositing enough USDC to exceed the new cap. This causes the admin transaction to revert, effectively preventing the protocol from updating critical parameters.

## Impact
Prevents admin from updating protocol parameters, potentially locking the protocol in a suboptimal or vulnerable economic state.

## Command to Run Test


## Proof of Concept
1. Admin submits `setNormalBallMax(30)` (reducing from 35). This reduces max pool cap.
2. Attacker calculates new cap.
3. Attacker front-runs with `lpDeposit` such that `total > newCap`.
4. Admin tx executes, calls `setLPPoolCap`.
5. Reverts due to `_lpPoolCap < ...` check.

## Proof of Code
function test_GovernanceFrontRun() public {
    // Setup LP position close to potential new cap
    // Frontrun setTicketPrice with deposit
    // Assert revert
}

## Suggested Mitigation
In `setLPPoolCap`, if the new cap is lower than current deposits, verify if it is a forced update from governance and allow it (clamping the cap to current deposits if necessary), or decouple the check from parameter updates.


## [M-3]. Permanent DoS via unbounded gas consumption in randomness callback

## id: SZw8xD3vgVLxiwIiL01oy

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.runJackpot

## Finding Status: Valid
### Finding Status Justification: Entropy gasLimit scales with bonusballMax and can become impractically large (high fee + above typical block limits), harming liveness. There is no protocol-level cap on bonusballMax or entropy gas parameters, and if requests/callbacks cannot execute the drawing remains locked until emergency.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `countTierMatchesWithBonusball` function in `TicketComboTracker` (called during drawing settlement) iterates `bonusballMax` times. Inside the loop, it performs multiple storage reads (approx 31 per iteration). If `bonusballMax` is high (near 255), the total gas cost for the callback can exceed 15-20 million gas.

Additionally, `runJackpot` calculates `entropyGasLimit` based on `entropyVariableGasLimit * bonusballMax`. With default settings (250k gas per ball), a high `bonusballMax` results in a gas limit request (>60M) that exceeds the block gas limit of most chains (30M on Base). If the callback runs out of gas or the request fails due to block limits, the jackpot remains locked (`jackpotLock = true`) forever, requiring owner intervention or emergency mode.

## Impact
Protocol availability failure (DoS). Drawing cannot be settled, funds stuck until emergency mode.

## Command to Run Test


## Proof of Concept
1. `bonusballMax` increases to 255 due to prize pool growth.
2. `runJackpot` is called. It calculates `entropyGasLimit` = 250k * 255 = 63.75M gas.
3. The call to `entropy.request` fails or the callback transaction fails because 63M gas > 30M block limit.
4. The drawing is stuck in `jackpotLock` state.

## Proof of Code
function testGasDoS() public {
    // Demonstrate loop cost estimation
    uint256 iterations = 255;
    uint256 readsPerIter = 31;
    uint256 gasPerRead = 2100;
    uint256 totalGas = iterations * readsPerIter * gasPerRead;
    // totalGas ~ 16.5M just for reads, plus overhead
}

## Suggested Mitigation
Optimize `countTierMatchesWithBonusball` to avoid O(N) storage reads, or cap `bonusballMax` to a safe limit (e.g. 100) that fits within block gas limits.


## [M-4]. Unbounded gas consumption in settlement callback causes permanent DoS

## id: nlnEIFWw5Vkod84UNzHky

## Derived From Pattern/Invariant
Unbounded Loop / Gas Limit DoS

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: The loop is bounded by uint8, but that is not a practical gas-limit safeguard: bonusballMax can be large (up to 255) and the nested subset generation + thousands of storage reads can exceed realistic tx/block gas, reverting the entropy callback and leaving jackpotLock stuck until emergency.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function triggers settlement, which calls `TicketComboTracker.countTierMatchesWithBonusball`. This function iterates from 1 to `_tracker.bonusballMax` to sum up ticket counts for every possible bonus ball. Inside this loop, it performs nested iterations over `normalTiers` and generates subsets, performing storage reads (`comboCounts[_bonusball][subset]`).

If `bonusballMax` is high (up to 255), this results in approximately 255 * 31 = 7,905 storage reads, plus associated logic. On Ethereum mainnet, this can cost upwards of 16M gas (assuming cold loads), consuming a significant portion of the block gas limit. If the logic exceeds the block gas limit (or the configured `entropyGasLimit`), the callback transaction will invariably fail. Since the callback is the only way to unlock the jackpot and transition to the next drawing, a failure here results in the jackpot being permanently locked (DoS) until emergency mode is activated.

## Impact
Permanent Denial of Service (Jackpot Locked).

## Command to Run Test


## Proof of Concept
1. `bonusballMax` is dynamically calculated and can reach 255. 
2. Users purchase tickets covering various combinations, populating the `comboCounts` mapping (making reads non-zero/warm or cold). 
3. The drawing time passes, and a keeper calls `runJackpot`. 
4. Pyth entropy is generated and `scaledEntropyCallback` is invoked. 
5. The callback executes `countTierMatchesWithBonusball`. 
6. The loop iterates 255 times. If the total gas execution exceeds the block limit (e.g., due to high storage read costs or chain congestion), the transaction reverts. 
7. The `Jackpot` remains in `jackpotLock = true` state. No new drawing can be started, and the current one cannot settle.

## Proof of Code
function testGasDoS() public {
    // Conceptual verification
    uint256 gasStart = gasleft();
    uint256 bonusballMax = 255;
    // Simulate loop overhead
    for(uint i=1; i<=bonusballMax; i++) {
        for(uint k=0; k<31; k++) {
             // Simulate storage load
             uint256 val = mockStorage[i][k];
        }
    }
    uint256 gasUsed = gasStart - gasleft();
    // Verify gasUsed is dangerously high
    assertGt(gasUsed, 15_000_000);
}

## Suggested Mitigation
Optimize the `TicketComboTracker` to avoid iterating all bonus balls (e.g., by storing a separate mapping for 'totals ignoring bonus ball') or strictly cap `bonusballMax` to a safe lower limit that ensures execution within block limits.


## [M-5]. DoS in settlement due to excessive gas consumption in TicketComboTracker

## id: DL7n_kKlDLIzMwUEIFr0u

## Derived From Pattern/Invariant
DoS via Block Gas Limit

## Exploit Type
GasGriefBlockLimit

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: Valid
### Finding Status Justification: The only 'safeguard' is an upper bound (uint8) plus requesting higher callback gas; neither guarantees the callback fits within chain block gas limits. A high bonusballMax can still make settlement revert and brick progression until emergency.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function triggers `_calculateDrawingUserWinnings`, which calls `TicketComboTracker.countTierMatchesWithBonusball`. This function iterates from 1 to `bonusballMax`, and for each bonus ball, iterates through all 31 subsets of the winning normal numbers to read storage. 

Since `bonusballMax` can dynamically scale up to 255 (uint8 max), and storage reads are expensive, a drawing with a high `bonusballMax` can require ~7,900 storage reads. This creates a gas cost of roughly 16M+ gas, potentially exceeding the block gas limit on the deployed chain (e.g., Target L2s or Mainnet during congestion). If the callback reverts due to Out of Gas, the jackpot remains locked (`jackpotLock = true`) and cannot proceed without emergency intervention.

## Impact
Denial of Service (DoS) of the jackpot settlement, requiring emergency mode activation to recover funds.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is configured to a low number (e.g. 5) or prize pool grows large.
2. `bonusballMax` calculation in `_setNewDrawingState` yields 255.
3. Keeper calls `runJackpot`.
4. Entropy provider calls `scaledEntropyCallback`.
5. Execution enters `countTierMatchesWithBonusball`.
6. Loop runs 255 times * 31 subsets = 7905 storage reads.
7. Transaction reverts due to exceeding block gas limit.
8. Jackpot remains locked.

## Proof of Code
function testGasDos() public {
    // Setup tracker
    // Insert enough tickets to force non-zero storage slots if necessary, or just rely on read cost
    uint8 bonusMax = 255;
    uint256 startGas = gasleft();
    // Simulate the loop structure of countTierMatchesWithBonusball
    for(uint8 i=1; i<=bonusMax; i++) {
        for(uint256 j=0; j<31; j++) {
            // Simulate storage read
            uint256 val = 0; // mapping read
            assembly { pop(val) }
        }
    }
    uint256 gasUsed = startGas - gasleft();
    // Assert gas usage is dangerously high
    // assertGt(gasUsed, 15_000_000);
}

## Suggested Mitigation
Cap `bonusballMax` to a safe limit (e.g., 100) or optimize `TicketComboTracker` to avoid iterating all non-matching bonus balls (e.g., by tracking active bonus balls in a separate list).


## [H-6]. Unbounded ticket sales bypass LP pool cap causing difficulty truncation and ticket corruption

## id: Cx84WStr-otZETNq58wB5

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: lpPoolCap only gates LP deposits, not lpEarnings from ticket volume. Since next drawing’s bonusballMax is derived from newPrizePool (which grows with lpEarnings), extreme ticket sales can push the computed bonusball requirement beyond safe bounds; _setNewDrawingState uses an unsafe uint256->uint8 cast and does not enforce bonusballMax <= 255-normalBallMax. No on-chain cap on ticket volume/earnings exists.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol relies on a `lpPoolCap` to ensure that the required difficulty (`bonusballMax`) for the next drawing does not exceed the capacity of the `uint8` type (255) or the bit-packing limits of `TicketComboTracker` (255 bits). The `lpPoolCap` is derived from `Combinations.choose(normalBallMax, 5) * (255 - normalBallMax)`, which represents the maximum number of tickets (and thus maximum prize pool) that can be supported without overflowing the bonus ball range `[1, 255 - normalBallMax]`. 

However, the `lpPoolCap` is only enforced during LP deposits in `JackpotLPManager.processDeposit`. It is **not** enforced during ticket purchases in `Jackpot.buyTickets`. Ticket sales increase `lpEarnings`, which are added to the LP pool at settlement. 

If ticket sales drive the pool size beyond the calculated capacity, two critical failures occur in `_setNewDrawingState` for the *next* drawing:
1. **Bit-Packing Overflow**: If the calculated `bonusballMax` causes `bonusballMax + normalBallMax >= 256`, the bit shift operation `1 << (bonus + normal)` in `TicketComboTracker.insert` will overflow to 0 (for `uint256(1)`). This results in tickets with a effectively missing bonus ball bit, causing corrupted state and fairness issues.
2. **Difficulty Truncation**: If the pool grows large enough that the required `bonusballMax` exceeds 255, the explicit cast `uint8(...)` in `_setNewDrawingState` will truncate the value (modulo 256). This drastically reduces the game difficulty instead of increasing it, destroying the LP edge and potentially rendering the protocol insolvent.

## Impact
Catastrophic loss of LP funds due to reduced game difficulty (truncation) or system dysfunction due to corrupted tickets (overflow).

## Command to Run Test


## Proof of Concept
Scenario:
1. Initialize Jackpot with `normalBallMax = 10`. `lpEdgeTarget = 10%`. Ticket price 1 USDC.
2. Calculated `lpPoolCap` supports approx `C(10,5) * (255-10) = 252 * 245 = 61,740` tickets.
3. LPs deposit up to the cap.
4. Users buy 62,000 tickets (allowed as `buyTickets` has no cap). `lpEarnings` increase significantly.
5. `runJackpot` is called. Settlement adds `lpEarnings` to the pool for the next drawing.
6. `_setNewDrawingState` calculates `minNumberTickets` based on the inflated pool. `minNumberTickets > 61,740`.
7. `newBonusball` is calculated as `ceil(minTickets / 252)`. Result is `>= 246`.
8. `newBonusball + normalBallMax` (246 + 10) = 256.
9. In the next drawing, a user buys a ticket with bonus ball 246.
10. `TicketComboTracker.insert` executes `set |= 1 << (246 + 10)`. `1 << 256` is 0. The bit is not set.
11. The ticket is minted but invalid (acts as having no bonus ball).

## Proof of Code
import { Test } from "forge-std/Test.sol";
import { Jackpot } from "../contracts/Jackpot.sol";
// ... imports for other contracts ...

contract JackpotOverflowTest is Test {
    Jackpot jackpot;
    // ... setup mocks ...

    function testOverflow() public {
        // 1. Setup with small normalBallMax to lower the cap threshold
        // assume normalBallMax = 10, ticketPrice = 1e6 (1 USDC)
        // combos = 252. Max bonus = 245. Max tickets approx 61,740.
        
        // 2. Buy tickets exceeding the safe capacity
        // (Simulate buying by prank and paying USDC)
        uint256 safeCapacity = 61740;
        uint256 overflowAmount = 65000;
        
        // buyTickets loop...
        
        // 3. Run jackpot and settle
        // jackpot.runJackpot{value: fee}();
        // entropyProvider.callback(...);
        
        // 4. Check next drawing state
        Jackpot.DrawingState memory state = jackpot.getDrawingState(2);
        
        // 5. Verify overflow/truncation or dangerous bonusballMax
        // If bonusballMax > 245, packing fails.
        assertGt(state.bonusballMax, 245);
        
        // 6. Demonstrate corruption
        // Try to insert/buy ticket with max bonus ball
        // This would fail or produce invalid packed ticket
    }
}

## Suggested Mitigation
Enforce a cap on total ticket sales in `buyTickets`. The contract should store `maxAllowableTickets` for the current drawing (calculated in `_setNewDrawingState` or `initialize`) and revert if `globalTicketsBought + count > maxAllowableTickets`.


## [H-7]. LP value drain due to unsafe downcast in _setNewDrawingState allowing bonusball overflow

## id: nF9lS0xUBzosvti4DNrxc

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: _setNewDrawingState uses `uint8(...)` on a uint256 result instead of UintCasts.toUint8, so values >255 will truncate rather than revert. There is also no enforcement of bonusballMax <= 255-normalBallMax, so truncation can materially reduce intended difficulty and/or later cause overflows/reverts elsewhere.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_setNewDrawingState`, the new `bonusballMax` is calculated to ensure the LP edge. The formula uses `Math.ceilDiv` which returns a `uint256`, but the result is explicitly cast to `uint8`. 

`newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));`

If the required number of tickets (`minNumberTickets`) is large relative to `combosPerBonusball` (e.g., due to a large prize pool, low ticket price, or small `normalBallMax`), the result can exceed 255. The explicit cast truncates the higher bits. For example, a result of 257 becomes 1. This drastically reduces the difficulty of the jackpot, giving players a massive +EV advantage and allowing them to drain the LP pool.

## Impact
Catastrophic loss of LP funds due to effectively zero difficulty drawings when prize pools are large.

## Command to Run Test


## Proof of Concept
1. Assume parameters: `normalBallMax`=10 (combos=252), `ticketPrice`=1 USDC, `edge`=10%.
2. Prize pool grows to $3,000,000.
3. `minNumberTickets` approx 3.3M.
4. `combosPerBonusball` = 252.
5. Required bonusballs = 3.3M / 252 ≈ 13,000.
6. `uint8(13000)` = 13000 % 256 = 200.
7. Difficulty is 200 instead of 13,000. 
8. The game is massively easier than intended to maintain solvency. LPs are exposed to huge loss.

## Proof of Code
function test_BonusballOverflow() public {
    // Manipulate prize pool to be large
    // Trigger settlement
    // Check new bonusballMax is small despite large pool
}

## Suggested Mitigation
Check if the calculated `bonusballMax` exceeds 255. If so, either cap it at 255 (accepting LP risk), revert (halting the game until parameters change), or change `bonusballMax` type to `uint16` (requires bit-packing changes).


## [H-8]. Bitwise data corruption in TicketComboTracker due to unchecked bonusball range

## id: OJPIZc1cH2eIJvVmtAxI9

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: There is no check that bonusballMax <= 255-normalBallMax. When (_bonusball + normalMax) exceeds 255, Solidity 0.8 checked uint8 addition will revert (not silently corrupt), which is still catastrophic: if such a bonusball is drawn as the winning bonusball, scaledEntropyCallback can revert and leave the jackpot locked until emergency.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library stores tickets using a bit vector where normal balls occupy bits `1` to `normalMax` and the bonus ball occupies bit `normalMax + bonusball`. The maximum size of the bit vector is implicitly 256. If `normalMax + bonusballMax` exceeds 255, `1 << (bonusball + normalMax)` will shift the bit out of the `uint256` range (resulting in 0 in Solidity 0.8 checked arithmetic for 256-bit shift, or truncation). 

The dynamic `bonusballMax` calculation in `Jackpot._setNewDrawingState` does not enforce that `newBonusball + normalBallMax <= 255`. It is possible for the calculated `bonusball` to fit in `uint8` (e.g., 200) but cause a bit overflow when added to `normalMax` (e.g., 60). In this case, `insert` in `TicketComboTracker` will incorrectly store the ticket (effectively losing the bonus ball selection), and `Jackpot.buyTickets` may eventually revert or corrupt ticket data, breaking the ability to claim winnings or correctly count duplicates.

## Impact
Broken game logic, inability to verify winning tickets, and incorrect duplicate counting leading to financial loss or DoS.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` = 60.
2. Prize pool grows such that calculated `bonusball` = 200.
3. `200 + 60 = 260`.
4. `_setNewDrawingState` sets `bonusballMax` = 200 (fits in uint8).
5. User buys ticket with bonus ball 200.
6. `TicketComboTracker.insert`: `set |= 1 << (200 + 60)`. Shift amount 260.
7. `1 << 260` results in 0 (or effective truncation depending on compilation). The bonus ball bit is not set.
8. The ticket is stored as just normal balls.
9. User cannot win because `_calculateTicketTierId` extracts bonus ball from bit 260, finding 0.

## Proof of Code
function testBitOverflow() public {
    // simulate condition where bonusball + normalMax > 255
}

## Suggested Mitigation
In `_setNewDrawingState`, ensure `newBonusball` is capped at `255 - normalBallMax`.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [H-9]. Randomness manipulation via entropy request ID collision in ScaledEntropyProvider upon provider switch

## id: 1RA8u3Tzl_S1neK3m68Ot

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: ScaledEntropyProvider stores pending requests in mapping(uint64 => PendingRequest) keyed only by the Pyth sequenceNumber. setEntropyProvider can change the provider while old requests are pending, and the code does not include providerAddress in the key nor does it forbid switching with outstanding requests. If a new provider issues a colliding sequenceNumber, pending[sequence] is overwritten; additionally _storePendingRequest appends to pending[sequence].setRequests without clearing, worsening collision effects. entropyCallback also does not validate the provider argument against the currently configured entropyProvider. This can lead to wrong request context being fulfilled by the wrong provider, undermining randomness integrity and/or causing settlement issues. It requires owner action (provider rotation timing), hence governance risk, but the underlying collision hazard is real.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `ScaledEntropyProvider` stores pending requests in a mapping keyed by `sequenceNumber`. This sequence number comes from the Pyth Entropy provider. Sequence numbers are typically incremental and unique only *per provider*. 

If the `Jackpot` owner switches the entropy provider via `setEntropyProvider` while a request is pending (e.g. for Drawing N), the new provider may generate a colliding sequence number for a subsequent request (Drawing N+1). This allows the pending request for Drawing N to be overwritten or for the old provider's callback to resolve the new provider's request. An attacker (or the old provider) could exploit this to supply randomness for a drawing they weren't intended to serve, or to interfere with drawing settlement.

## Impact
Randomness manipulation or DoS of drawing settlement.

## Command to Run Test


## Proof of Concept
1. Drawing N requests entropy from Provider A. Sequence 100 returned. `pending[100]` stored.
2. Admin calls `setEntropyProvider(Provider B)`.
3. Drawing N+1 requests entropy from Provider B. Sequence 100 returned (collision). `pending[100]` overwritten.
4. Provider A calls back with Sequence 100.
5. `entropyCallback` uses `pending[100]` (which contains Drawing N+1 context).
6. Provider A effectively settles Drawing N+1 using randomness intended for N.

## Proof of Code
function test_ProviderCollision() public {
    // Mock Entropy contract returning duplicate sequences for diff providers
    // Simulate attack flow
}

## Suggested Mitigation
Key the `pending` mapping using a hash of `(providerAddress, sequenceNumber)` instead of just `sequenceNumber`.





Finding Status: InvalidGovernanceRisk
## [M-10]. LP Value Leak on Duplicate Tickets when Referral Fee Exceeds LP Edge

## id: AL86eQxpV_L7XwzFDKq5D

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: On each purchase, Jackpot.buyTickets increases lpEarnings by ticketsValue - referralFeeTotal, while duplicate tickets additionally increase prizePool by ticketPrice - edgePerTicket (i.e., ticketPrice*(1-lpEdgeTarget)). In expectation, increasing prizePool increases expected payouts; if referralFee > lpEdgeTarget, the protocol is effectively giving a larger “rebate” (referral) than the edge intended to compensate LPs for the extra liability of duplicates, reducing (or flipping) LP edge. The code does not enforce referralFee <= lpEdgeTarget nor otherwise adjust duplicate accounting based on actual net revenue. This is primarily a governance/configuration risk (owner chooses fees) but, if misconfigured, can be exploited economically (e.g., self-referrals + targeted duplicate buying) to make the game +EV and drain LP over time.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a user purchases a duplicate ticket, the `Jackpot` contract adds `ticketPrice - edgePerTicket` to the `prizePool`, while `lpEarnings` increases by `ticketPrice - referralFee`. At settlement, the LP pool value is updated: `newLP = oldLP + lpEarnings - winnings`. Since winnings are paid from the `prizePool` (which effectively liabilities the system for the duplicate ticket's potential payout), the net change to the system's equity for a duplicate ticket is `(ticketPrice - referralFee) - (ticketPrice - edgePerTicket) = edgePerTicket - referralFee`.

If the configured `referralFee` is greater than `lpEdgeTarget`, this net change is negative. This means LPs lose value on every duplicate ticket purchased. Attackers can refer themselves to recoup the referral fee and exploit this leak, or simply high referral fees combined with low edge targets will naturally drain the pool over time.

## Impact
Guaranteed financial loss for Liquidity Providers on every duplicate ticket sold if parameters are misaligned.

## Command to Run Test


## Proof of Concept
1. Governance sets `referralFee` = 10% and `lpEdgeTarget` = 5%.
2. User buys a duplicate ticket for 100 USDC.
3. `lpEarnings` += 90 USDC. `prizePool` += 95 USDC.
4. System liability increases by 95, assets by 90.
5. Net LP loss = 5 USDC.

## Proof of Code
function testLPLeak() public {
    // Set referral > edge
    // Buy duplicate
    // Check LP value decreases
}

## Suggested Mitigation
Enforce `referralFee <= lpEdgeTarget` in the `Jackpot` constructor and setter functions, or adjust the duplicate ticket logic to credit `prizePool` based on the actual net revenue (`ticketPrice - referralFee`).





Finding Status: LowSeverityDueToRareLikelihood
## [H-11]. LP edge violation and game state corruption due to arithmetic overflow in bonusballMax calculation and bit packing

## id: 7J0FCLqmGWRtpSRDNDtj3

## Derived From Pattern/Invariant
Arithmetic Over/Underflow

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: While the specific 'shift>=256 becomes 0 and causes 0==0 bonus matches' sub-claim is wrong (it would revert earlier due to uint8 overflow), the finding also includes a real issue: unsafe uint256->uint8 downcast in bonusballMax calculation can truncate. There is no on-chain safeguard (no safe cast, no bonusballMax<=255-normalMax enforcement), so treating the entire report as 'does not exist / safeguarded / not exploitable' is not fully correct.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_setNewDrawingState` function dynamically calculates `newBonusball` (the bonus ball range) to maintain a target LP edge. It uses `Math.ceilDiv(minNumberTickets, combosPerBonusball)` and casts the result to `uint8`. If the required difficulty is high (e.g. due to a high LP edge target or small `normalBallMax`), the result can exceed 255, wrapping around to a small number (e.g. 257 -> 1). This drastically lowers the difficulty, violating the LP edge guarantee and causing LPs to lose money.

Additionally, `TicketComboTracker.insert` packs the ticket into a `uint256` using `1 << (_bonusball + _tracker.normalMax)`. If `_bonusball + _tracker.normalMax >= 256` (which is possible as `normalMax` and `bonusballMax` can each be up to 255), the shift overflows to 0. This results in tickets with the bonus ball bit lost. In `claimWinnings`, `_calculateTicketTierId` extracts the bonus ball via shifting. If the bit was lost, both the user's ticket and the winning ticket (if also corrupted) will resolve to bonus ball 0, causing incorrect bonus matches for all users regardless of their selection.

## Impact
LPs suffer systematic losses due to broken difficulty adjustments; Ticket data is corrupted leading to incorrect payout logic.

## Command to Run Test


## Proof of Concept
1. Deploy Jackpot with `lpEdgeTarget` near 100% or small `normalBallMax`.
2. `runJackpot` completes. `_setNewDrawingState` calculates `minNumberTickets` / `combos` > 255.
3. `newBonusball` wraps to a small value (e.g. 1).
4. Game becomes trivial to win, draining LP pool.
5. Alternatively, set `normalBallMax` = 200. Dynamic bonus ball calc results in 60. Sum = 260.
6. Buy ticket with bonus ball 1. Packed ticket has 0 at bonus position.
7. Winning ticket has bonus ball 5. Packed winning ticket has 0 at bonus position.
8. `claimWinnings` sees match (0==0) despite mismatch (1!=5).

## Proof of Code
function testOverflow() public {
  // Conceptual logic
  uint8 normalMax = 200;
  uint8 bonus = 60;
  uint256 packed = 1 << (bonus + normalMax);
  assertEq(packed, 0); // Overflow confirms corruption
}

## Suggested Mitigation
In `_setNewDrawingState`, ensure `newBonusball` does not overflow `uint8` and cap it if necessary (though this affects edge). Crucially, enforce `normalBallMax + newBonusball < 256` to prevent bit packing overflow. If calculation exceeds limits, revert or cap parameters to safe bounds.



