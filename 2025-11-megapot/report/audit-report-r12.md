# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[M-1]. Denial of Service in Drawing Settlement due to Unbounded Gas Consumption
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[H-2]. Incorrect Ticket Matching via Bit Shift Overflow in `TicketComboTracker`
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[M-3]. Unbounded `bonusballMax` causes DoS in `runJackpot` due to excessive gas limit calculation
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-4]. Bit packing overflow in TicketComboTracker leads to incorrect bonus ball matches and prize pool drainage
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[M-5]. Unbounded Bonusball Scaling Leads to Permanent Settlement DOS via Block Gas Limit
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-6]. LP Solvency Risk due to Bonusball Max Truncation in `_setNewDrawingState`
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-7]. LP edge collapse due to uint8 overflow in bonus ball difficulty calculation
**Derived From** : Integer Overflow
Finding Status: Valid
Privilege: Permissionless



Finding Status: InvalidGovernanceRisk


[M-8]. Bit Packing Overflow due to Incompatible Ball Ranges
**Derived From** : ConfigFootgun
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-9]. LP Profitability Footgun due to Unchecked Referral Fees
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-10]. Insolvency due to bit-packing overflow allowing tickets to match any bonus ball
**Derived From** : IntegerOverflow
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless


[M-11]. LP Value Bleed via Duplicate Tickets when Referral Fee > LP Edge
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-12]. Permanent DoS of LP system if accumulator becomes zero due to 100% loss
**Derived From** : ConfigFootgun
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-13]. DoS of Settlement via Config Misalignment (Referral Fee > LP Edge)
**Derived From** : ConfigFootgun
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidByDesign


[M-14]. Lack of user commitment in entropy request enables provider collusion
**Derived From** : BlockVarsAsPrimaryRandomnessSource
Finding Status: LowSeverityDueToRareLikelihood + InvalidByDesign
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood


[M-15]. Unbounded LP earnings allows `bonusballMax` to exceed bit-packing limits, corrupting tickets
**Derived From** : IntegerOverflow
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 5
- M: 10
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. Denial of Service in Drawing Settlement due to Unbounded Gas Consumption

## id: pI528eFNb5Ood25reaybS

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: Valid
### Finding Status Justification: The liveness risk stems from unbounded loops over bonusballMax, which is derived automatically from prizePool growth and can be influenced by user activity; it is not solely a privileged-user error/misuse scenario.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function calls `_calculateDrawingUserWinnings`, which invokes `TicketComboTracker.countTierMatchesWithBonusball`. This function contains a loop that iterates from 1 to `bonusballMax`. Inside this loop, there is another loop over the tiers, which performs multiple storage reads from the `comboCounts` mapping. 

If `bonusballMax` is large (close to its max of 255), the number of storage reads (approx 31 reads * 255 iterations = ~7900 reads) combined with logic overhead can consume excessive gas, potentially exceeding the block gas limit or the configured entropy gas limit. If the callback reverts due to Out of Gas, the jackpot remains locked (`jackpotLock` is true), and the protocol cannot progress to the next drawing.

## Impact
Permanent freezing of the jackpot protocol if the gas cost to settle a drawing exceeds the block gas limit.

## Command to Run Test


## Proof of Concept
1. The prize pool grows large enough to push `bonusballMax` to 255.
2. A drawing is executed.
3. `scaledEntropyCallback` is triggered.
4. The loop in `countTierMatchesWithBonusball` iterates 255 times, reading thousands of storage slots.
5. Transaction runs out of gas and reverts.
6. `jackpotLock` remains true; no new tickets can be bought, and `runJackpot` cannot be called again for the current drawing.

## Proof of Code
function testGasDos() public {
    // Setup state where bonusballMax = 255
    // Populate comboCounts to ensure non-zero storage reads
    // Measure gas used by countTierMatchesWithBonusball
    // Assert gas > reasonable limit
}

## Suggested Mitigation
Optimize the winner counting logic to avoid iterating `bonusballMax` (e.g., track total counts separately) or enforce a stricter upper bound on `bonusballMax` (and `normalBallMax`) to ensure settlement fits within block limits.


## [H-2]. Incorrect Ticket Matching via Bit Shift Overflow in `TicketComboTracker`

## id: QiJrOEAJsl4lK2cf3_E6O

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: The missing constraint on (normalMax + bonusball) is an invariant failure that can be reached via automatic bonusballMax scaling as prizePool grows; it is not exclusively a privileged-user mistake scenario.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library packs ticket numbers into a `uint256` bitmask. The bonus ball is stored at the bit index `_bonusball + _tracker.normalMax`. If the sum of `_bonusball` and `_tracker.normalMax` equals or exceeds 256, the operation `1 << (_bonusball + _tracker.normalMax)` overflows (resulting in 0 in EVM logic for shifts >= 256). 

This causes tickets with high bonus ball numbers to effectively lose their bonus ball bit. Consequently, multiple distinct high bonus ball numbers will map to the same internal representation (all zeros for the bonus ball part). If the winning ticket's bonus ball also triggers this overflow, all tickets with overflow-triggering bonus balls will be considered matches, breaking the fairness of the lottery.

## Impact
Incorrect winner determination; losing tickets can claim prizes, or winning tickets may fail to match correctly depending on the specific overflow values.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` = 200.
2. `bonusballMax` scales to 60.
3. User buys ticket with bonus ball 60. Index = 260.
4. `1 << 260` results in 0. Ticket stored with no bonus bit.
5. User buys ticket with bonus ball 70. Index = 270.
6. `1 << 270` results in 0. Ticket stored with no bonus bit.
7. Winning number is 60. Winning mask has no bonus bit.
8. User holding 70 is calculated as a winner (bonus match) because both packed values lack the bonus bit.

## Proof of Code
function testBitShiftOverflow() public {
    // ... setup tracker with normalMax 200, bonusMax 60
    // Insert ticket with bonusball 60
    // Insert ticket with bonusball 70
    // Check matching logic
}

## Suggested Mitigation
Ensure that `normalBallMax + bonusballMax < 256` is enforced at the contract level (in `_setNewDrawingState`), or use a different storage mechanism for tickets if the range requires > 256 bits.


## [M-3]. Unbounded `bonusballMax` causes DoS in `runJackpot` due to excessive gas limit calculation

## id: tgWBYGxnyYylWNMAnvXsk

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.runJackpot

## Finding Status: Valid
### Finding Status Justification: Having an admin-tunable entropyVariableGasLimit is not a true safeguard because the protocol can still enter an uncallable/unfulfillable state until governance intervenes. Also, the triggering condition (bonusballMax growth from prizePool scaling) is not purely an admin mistake; it can arise from organic/system growth and user-driven activity.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.runJackpot`, the entropy gas limit is calculated as `entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax`. The `entropyVariableGasLimit` defaults to 250,000.

As described in the 'Bit packing overflow' finding, `bonusballMax` can grow significantly (up to ~220-250) if the prize pool increases. If `bonusballMax` reaches 200, the calculated gas limit becomes `250,000 * 200 = 50,000,000` gas. This exceeds the block gas limit of most EVM chains (e.g., 30M on Ethereum/Base). 

When `runJackpot` attempts to call `entropy.requestAndCallbackScaledRandomness` with this gas limit, the call will fail (either rejected by the Pyth contract or logically impossible to fulfill in a block). This permanently locks the jackpot in the current state until the owner intervenes to lower `entropyVariableGasLimit`.

## Impact
Denial of Service. The jackpot cannot be run or settled, freezing user funds and protocol operations.

## Command to Run Test


## Proof of Concept
1. `lpEarnings` inflate `prizePool`, causing `bonusballMax` to be set to 200 in `_setNewDrawingState`.
2. `runJackpot` is called for the new drawing.
3. `_calculateEntropyGasLimit` returns `250,000 * 200 = 50M`.
4. The entropy request fails because 50M gas cannot be provided/requested within block limits.
5. The drawing cannot be initiated.

## Proof of Code
function testGasLimitDoS() public {
    uint8 bonusballMax = 200;
    uint32 variableGas = 250000;
    uint256 calculated = uint256(variableGas) * bonusballMax;
    assertGt(calculated, 30_000_000); // Exceeds block limit
}

## Suggested Mitigation
Cap `bonusballMax` to a reasonable limit, or reduce the default `entropyVariableGasLimit` to ensure the maximum possible gas limit calculation stays within block limits.


## [H-4]. Bit packing overflow in TicketComboTracker leads to incorrect bonus ball matches and prize pool drainage

## id: jdz48NBU3N49-CEbl_Sj7

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: The missing bound on (_bonusball + normalMax) is a protocol invariant issue that can be reached via automatic bonusballMax scaling with prizePool (not only via an explicit admin mis-set), so it is not purely an admin-mistake/governance-only risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library packs the 5 normal balls and 1 bonus ball of a ticket into a single `uint256` bit vector. The bonus ball bit is set using `set |= 1 << (_bonusball + _tracker.normalMax)`. If the sum of `_bonusball` and `_tracker.normalMax` equals or exceeds 256, the shift operation overflows in the `uint256` context, resulting in `0`. This effectively erases the bonus ball information from the packed ticket. The same overflow occurs when packing the winning numbers in `_calculateDrawingUserWinnings`. When `_calculateTicketTierId` is subsequently called to determine matches, it shifts the packed ticket by `normalBallMax + 1` to extract the bonus ball. Due to the prior overflow, this extracts `0`. Since both the user ticket and the winning ticket have an effective bonus ball of `0`, they match (`0 == 0`). This allows any ticket to be counted as having a bonus ball match regardless of the actual bonus ball number selected, drastically inflating payouts and potentially draining the prize pool. This state can be reached either by an admin setting a high `normalBallMax` or automatically if the prize pool grows large enough (via unbounded `lpEarnings` from ticket sales) to cause the dynamically calculated `bonusballMax` to result in an overflow when added to `normalBallMax`.

## Impact
The protocol pays out bonus ball premiums to tickets that did not match the winning bonus ball, leading to rapid insolvency and loss of LP funds.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 250 (allowed, max 255).
2. `bonusballMin` is set to 10.
3. A drawing is initialized. `bonusballMax` is at least 10. `250 + 10 = 260`, which causes overflow.
4. User buys a ticket with normal balls [1,2,3,4,5] and bonus ball 10.
5. `TicketComboTracker.insert` calculates `1 << (10 + 250)` -> `0`. Packed ticket contains only normal bits.
6. Keeper calls `runJackpot`. Pyth callback returns winning numbers: normals [1,2,3,4,5], bonus ball 20.
7. Winning ticket packing also overflows: `1 << (20 + 250)` -> `0`.
8. `_calculateDrawingUserWinnings` -> `countTierMatchesWithBonusball` correctly identifies matches for unique/dup counts using unpacked values? No, `_calculateTicketTierId` used in `claimWinnings` uses the packed values.
9. User calls `claimWinnings`. `_calculateTicketTierId` extracts bonus ball from packed ticket (0) and winning ticket (0). Match!
10. User receives payout for Tier 11 (Jackpot) or Tier 10+Bonus, despite having wrong bonus ball.

## Proof of Code
function testBitPackingOverflow() public {
    // Setup: normalBallMax = 250, bonusballMin = 10
    vm.prank(owner);
    jackpot.setNormalBallMax(250);
    vm.prank(owner);
    jackpot.setBonusballMin(10);
    
    // Advance to next drawing to apply params
    // ... (omitted setup for drawing transition) ...
    
    // Buy ticket with bonus ball 10
    IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
    uint8[] memory normals = new uint8[](5);
    for(uint8 i=0; i<5; i++) normals[i] = i+1;
    tickets[0] = IJackpot.Ticket(normals, 10);
    jackpot.buyTickets(tickets, address(this), new address[](0), new uint256[](0), bytes32(0));
    
    // Mock randomness to return same normals but DIFFERENT bonus ball (e.g., 20)
    // ... trigger callback ...
    
    // Claim winnings
    uint256[] memory claimIds = new uint256[](1);
    claimIds[0] = 0; // ticketId
    jackpot.claimWinnings(claimIds); // Should revert or pay lower tier, but pays bonus tier
}

## Suggested Mitigation
Ensure `normalBallMax + bonusballMax < 256` in both `setNormalBallMax` and `_setNewDrawingState`. Additionally, verify `1 << (bonusball + normalMax)` does not overflow `uint256` or use a larger structure for tracking.


## [M-5]. Unbounded Bonusball Scaling Leads to Permanent Settlement DOS via Block Gas Limit

## id: emGvkWHn4Prd1PgDGr5FE

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: Settlement gas blowups can occur from the protocol’s automatic bonusballMax scaling as prizePool grows (and from user-driven ticket volume affecting future prize pools), without any admin mistake. Governance can mitigate via tuning caps/gas params, but the liveness risk is not solely caused by privileged-user error.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function settles the jackpot by counting winners, which involves iterating through all possible bonusball values in `TicketComboTracker`. The gas cost of this loop scales linearly with `bonusballMax`. Furthermore, the `runJackpot` function enforces a gas limit for the callback calculated as `entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax`. With the default `entropyVariableGasLimit` of 250,000, a `bonusballMax` of ~120 results in a requested gas limit exceeding 30 million (Ethereum block gas limit). If the prize pool grows large enough (e.g. ~40M USDC) to require such a `bonusballMax`, the `runJackpot` call will fail or the callback transaction will inevitably revert due to Out of Gas, permanently locking the jackpot.

## Impact
Permanent Denial of Service (Jackpot locked indefinitely). The only recovery is enabling Emergency Mode and refunding all tickets.

## Command to Run Test


## Proof of Concept
1. Prize pool reaches ~40M USDC.
2. `_setNewDrawingState` calculates `bonusballMax` around 120 to maintain LP edge.
3. Next drawing begins.
4. When `runJackpot` is called, `_calculateEntropyGasLimit` returns > 30,000,000.
5. The entropy provider cannot fulfill the callback within the block gas limit.
6. The jackpot state remains `jackpotLock = true` forever.

## Proof of Code
function testGasLimitExceeded() public {
    uint32 base = 100000;
    uint32 variable = 250000; // Default in constructor
    uint8 bonusball = 125;
    uint256 totalGas = base + variable * bonusball;
    // 31,350,000 > 30,000,000 (ETH Block Limit)
    assertGt(totalGas, 30_000_000);
}

## Suggested Mitigation
Optimize `TicketComboTracker` to avoid iterating over all bonusballs or impose a strict upper bound on `bonusballMax` (e.g., 100) and ensure `entropyVariableGasLimit` is tuned such that the max gas limit fits comfortably within the block gas limit.


## [H-6]. LP Solvency Risk due to Bonusball Max Truncation in `_setNewDrawingState`

## id: uwkdnUVqKNOf3NeD46zSe

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: The uint8 truncation is in core automatic parameter computation and can occur purely from prizePool growth (no admin mistake required). Governance can influence parameters, but the bug is not inherently a governance-only risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, the function `_setNewDrawingState` calculates the `newBonusball` (bonusballMax) for the next drawing to ensure the LP edge is maintained. The calculation `Math.ceilDiv(minNumberTickets, combosPerBonusball)` returns a `uint256`, which is then explicitly cast to `uint8`. 

If the ratio of `minNumberTickets` to `combosPerBonusball` exceeds 255, the explicit cast truncates the higher bits. This occurs when the prize pool is large relative to the ticket price and the number of combinations (determined by `normalBallMax`). 

For example, if `normalBallMax` is small (e.g., 5) or the prize pool is very large, the required bonus balls could be > 255. The truncation forces `bonusballMax` to a small value (e.g., 256 becomes 0, 300 becomes 44). A artificially small `bonusballMax` dramatically increases the player's odds of winning, far beyond what the math intends, causing the game to be +EV for players and rapidly draining the LP pool.

## Impact
Catastrophic loss of LP funds due to incorrectly low game difficulty.

## Command to Run Test


## Proof of Concept
1. Assume `normalBallMax` = 5 (Combos = 1).
2. Assume `prizePool` corresponds to 1,000,000 tickets.
3. `minNumberTickets` = 1,000,000.
4. `newBonusball` calculation: `ceilDiv(1,000,000, 1) = 1,000,000`.
5. Cast to uint8: `uint8(1,000,000) = 64` (1000000 % 256).
6. The game sets `bonusballMax` to 64 instead of 1,000,000.
7. Players have 1/64 chance to match bonus ball instead of 1/1,000,000, easily draining the pool.

## Proof of Code
function testBonusballTruncation() public {
    // Setup: normalBallMax = 5 (1 combo), huge prize pool
    vm.prank(owner);
    jackpot.setNormalBallMax(5);
    // Mock huge LP deposit to create huge prize pool
    // ... (setup LP/deposits)
    // Trigger settlement
    vm.prank(address(entropyProvider));
    jackpot.scaledEntropyCallback(1, randomNumbers, "");
    // Check next drawing state
    Jackpot.DrawingState memory state = jackpot.getDrawingState(2);
    // state.bonusballMax should be 255 (capped) but will be truncated
    assertLt(state.bonusballMax, 255);
}

## Suggested Mitigation
Check if the calculated value exceeds 255 before casting. If it does, either cap it at 255 (though this breaks the edge guarantee) or revert/handle gracefully. Alternatively, ensure `normalBallMax` and `ticketPrice` parameters prevent this ratio from exceeding 255.


## [H-7]. LP edge collapse due to uint8 overflow in bonus ball difficulty calculation

## id: 5KFWnQ9qWLREbrJnQPQXJ

## Derived From Pattern/Invariant
Integer Overflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Same as the paired report: uint8(Math.ceilDiv(...)) truncation is an unconditional coding issue that can trigger from sufficiently large computed values, independent of admin error.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the new bonus ball maximum (`newBonusball`) is calculated to ensure the LP edge is maintained. The calculation uses `Math.ceilDiv` to determine the required range size based on ticket sales volume and prize pool size. However, the result is explicitly cast to `uint8`. If the required difficulty (range size) exceeds 255, the cast overflows (wraps modulo 256), resulting in a much smaller `bonusballMax` than required. This drastically reduces the difficulty of winning the bonus ball, destroying the LP edge and making the game -EV for liquidity providers. This condition is easily reachable if `normalBallMax` is set to a low value (reducing combinations) or if the `prizePool` grows large enough.

## Impact
Economic ruin for LPs as the game difficulty becomes significantly lower than the mathematical requirement to sustain the prize pool.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 5 (minimum). `Combinations.choose(5,5)` is 1.
2. LPs deposit 2000 USDC. `prizePool` approx 2000e6.
3. `minNumberTickets` approx 2000e6 * 1e18 / (0.75e18 * 1e6) = 2666 tickets.
4. `newBonusball` calculation: `ceilDiv(2666, 1) = 2666`.
5. `uint8(2666)` casts to `2666 % 256 = 106`.
6. The next drawing sets `bonusballMax` to 106 instead of 2666.
7. Players have 1/106 chance to match bonus ball instead of 1/2666. LPs suffer massive statistical loss.

## Proof of Code


## Suggested Mitigation
Increase the size of `bonusballMax` to `uint16` or `uint256` in `DrawingState` and `Jackpot` logic, or cap the result at 255 (though capping violates the edge guarantee, so expanding the type is necessary).





Finding Status: InvalidGovernanceRisk
## [M-8]. Bit Packing Overflow due to Incompatible Ball Ranges

## id: o0O8qKSdjBUmpgRmVs6Aw

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.setNormalBallMax

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Once governance configures an incompatible range (e.g., bonusballMin high enough), the failure mode affects permissionless users (mis-tiering/claim reverts) and can be used strategically/probabilistically by users; it is not purely “not exploitable”, though it is triggered by governance configuration.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The ticket packing logic `1 << (_bonusball + _tracker.normalMax)` requires that the sum of the bonus ball and max normal ball does not exceed 255. The `setNormalBallMax` and `setBonusballMin` functions do not enforce `normalBallMax + bonusballMin < 256`. If configured incompatibly, the bonus ball bit will be shifted out of the `uint256` range, causing ticket verification to fail and potentially bricking the drawing logic.

## Impact
Broken ticket logic; inability to determine winners or unpack tickets correctly.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` = 250.
2. Admin sets `bonusballMin` = 10.
3. A ticket is bought with bonusball 10.
4. `insert` calculates `1 << (10 + 250)` = `1 << 260` = 0.
5. The packed ticket loses the bonusball information.

## Proof of Code


## Suggested Mitigation
Enforce `normalBallMax + bonusballMin < 255` in setter functions.


## [M-9]. LP Profitability Footgun due to Unchecked Referral Fees

## id: 5T1xONgFb-2d2koLPbiZV

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot.setReferralFee

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Jackpot.setReferralFee allows referralFee up to 100% and does not constrain it relative to lpEdgeTarget. In buyTickets, lpEarnings increases by `ticketsValue - referralFeeTotal`, while duplicate tickets increase prizePool by `ticketPrice - edgePerTicket` (where edgePerTicket = lpEdgeTarget*ticketPrice). If referralFee exceeds lpEdgeTarget, then for duplicates the protocol can create more prizePool liability per duplicate than LP earnings credited, weakening/negating the intended edge and potentially making the system -EV for LPs under that configuration. This requires owner misconfiguration (trusted role), but once configured, any user can take the other side by buying tickets.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The protocol relies on the `bonusballMax` dynamic difficulty adjustment to guarantee LPs an edge. This calculation assumes the LP receives `ticketPrice` in revenue for every ticket sold, aiming to keep expected payouts below `ticketPrice * (1 - lpEdgeTarget)`. However, the actual revenue to LPs is `ticketPrice - referralFee`. 

If the Admin configures `referralFee` (plus protocol overhead) to be greater than `lpEdgeTarget` (as a percentage of ticket price), the actual revenue per ticket will be less than the expected payout. The code does not enforce `referralFee <= lpEdgeTarget`. In such a configuration, LPs mathematically lose money on every ticket sold, draining the pool over time.

## Impact
Guaranteed loss of LP funds for every ticket sold if fees are misconfigured. LPs drain until insolvency.

## Command to Run Test


## Proof of Concept
1. Admin sets `ticketPrice` = 1 USDC.
2. Admin sets `lpEdgeTarget` = 10% (0.1e18). System tunes difficulty so Payouts ~= 0.9 USDC.
3. Admin sets `referralFee` = 20% (0.2e18).
4. User buys ticket for 1 USDC.
5. `referralFee` of 0.2 USDC is paid to referrer.
6. `lpEarnings` increases by 0.8 USDC.
7. Expected Payout liability created is 0.9 USDC.
8. Net LP PnL = 0.8 - 0.9 = -0.1 USDC per ticket.

## Proof of Code
function testLPGuaranteedLoss() public {
        vm.startPrank(owner);
        jackpot.setLpEdgeTarget(1e17); // 10%
        jackpot.setReferralFee(2e17);  // 20%
        vm.stopPrank();
        
        // Logic check: 
        // Expected Revenue = 0.8 * Price
        // Target Payout (via bonusball math) = 0.9 * Price
        // LP loses 0.1 * Price per ticket.
    }

## Suggested Mitigation
Enforce `referralFee <= lpEdgeTarget` in `setReferralFee` and `setLpEdgeTarget` functions to prevent creating a negative-sum game for LPs.


## [H-10]. Insolvency due to bit-packing overflow allowing tickets to match any bonus ball

## id: Z8LYXse68avrqIeC0L1Fi

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot.sol.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: TicketComboTracker.insert packs the bonus ball as `1 << (_bonusball + _tracker.normalMax)` with no bound enforcing `_bonusball + normalMax < 256`. In Solidity/EVM, shifting by >=256 yields 0, so the bonus bit is not set. Jackpot._calculateTicketTierId then extracts bonusball as `packed >> (normalBallMax+1)`, yielding 0 when the bonus bit is missing, making bonusballMatch spuriously true (0==0) and potentially underflowing `matches - bonusballMatch` for 0-normal-match tickets (DoS) and/or misclassifying tiers (overpay). This can be triggered by unsafe parameterization (e.g., high normalBallMax and/or high bonusballMin/bonusballMax). No guard exists in setters or _setNewDrawingState.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol packs a ticket's normal numbers and bonus ball into a single `uint256`. Normal numbers occupy bits `1` to `normalBallMax`, and the bonus ball bit is set at index `normalBallMax + bonusball`. 

There is no check that `normalBallMax + bonusballMax < 256`. If `normalBallMax` is small (e.g., 10) and `bonusballMax` is large (e.g., 246), their sum can equal or exceed 256. `bonusballMax` is dynamically calculated based on prize pool size and can grow large, or can be forced high via `bonusballMin`. 

When `1 << (normalMax + bonus)` is executed with a shift amount >= 256, the result is 0 (due to EVM `SHL` semantics or effective overflow outside `uint256`). This results in the bonus ball bit NOT being set in the `packedTicket`. 

Consequently, `_calculateTicketTierId` reads the bonus ball as `0` for such tickets (`packed >> (normalMax + 1)`). If the winning ticket also overflows, it too registers a bonus ball of `0`. This causes a guaranteed 'match' for the bonus ball component, even if the user picked a different number than the winner (provided both are in the overflow range). 

This leads to inflating payout tiers (e.g., non-matches become matches) and allows multiple distinct tickets (e.g., bonus 246, 247) to claim prizes for the same winning number, draining the prize pool beyond the calculated winner accounting and leading to insolvency.

## Impact
Protocol insolvency and theft of prize pool funds via manipulated ticket payouts.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 10 and `bonusballMin` to 246 (or organic growth causes `bonusballMax` to reach 246). 
2. Attacker buys a ticket with `bonusball = 246`. The packing logic computes `1 << (10 + 246) = 1 << 256 = 0`. 
3. The ticket is stored with only normal ball bits set. 
4. The winning number is drawn. Suppose the winning bonus ball is also 247. Its packed representation also has the bonus bit as 0. 
5. Attacker claims winnings. `_calculateTicketTierId` extracts `ticketBonus = 0` and `winningBonus = 0`. Match is true. 
6. Attacker receives a payout for a bonus match despite having different numbers, and potentially across multiple tickets.

## Proof of Code
function testBitPackingOverflow() public {
    // Setup
    vm.startPrank(owner);
    jackpot.setNormalBallMax(10);
    jackpot.setBonusballMin(246);
    vm.stopPrank();

    // User buys ticket with bonus 246
    uint8[] memory normals = new uint8[](5);
    for(uint8 i=0; i<5; i++) normals[i] = i+1;
    
    IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
    tickets[0] = IJackpot.Ticket(normals, 246);
    
    address[] memory refs = new address[](0);
    uint256[] memory splits = new uint256[](0);
    
    vm.prank(user);
    usdc.approve(address(jackpot), 10e6);
    vm.prank(user);
    jackpot.buyTickets(tickets, user, refs, splits, bytes32(0));
    
    // Verify packed ticket has NO bonus bit (should be 0 due to overflow)
    // Normals 1-5 set bits 1,2,3,4,5. Mask is 0x3E.
    // Bonus bit should be at 256 (overflows to 0)
    
    // In claim logic, check tier
    // Winning ticket with bonus 247 also overflows
    // Result: Match
}

## Suggested Mitigation
Enforce `normalBallMax + bonusballMax < 256` in `setNormalBallMax`, `setBonusballMin`, and inside `_setNewDrawingState` where `bonusballMax` is dynamically calculated.


## [M-11]. LP Value Bleed via Duplicate Tickets when Referral Fee > LP Edge

## id: clRLakTdluvGWSFYed0qM

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: In Jackpot.buyTickets, duplicates increase prizePool by `ticketPrice - edgePerTicket` while lpEarnings increases by `ticketPrice - referralFeePortion` (net of referral fees). If referralFee is configured higher than lpEdgeTarget, then per-duplicate the accounted liability added to prizePool can exceed the accounted LP earnings credited, eroding LP value relative to the protocol’s intended edge model. This is a configuration footgun (owner-controlled) rather than a pure code exploit, but once configured, it is permissionlessly “exploitable” in the sense that users can buy duplicates under negative economics for LPs.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a duplicate ticket is purchased, the system increases the `prizePool` by `ticketPrice * (1 - lpEdgeTarget)` and increases `lpEarnings` by `ticketPrice * (1 - referralFee)`. The LP pool's net value change is derived from `lpEarnings` (asset addition) minus the liability increase in `prizePool`. 

If the `referralFee` is set higher than `lpEdgeTarget`, the liability increase (prize pool addition) exceeds the asset increase (earnings), creating a deficit that is subsidized by the existing LP pool. An attacker or users can exploit this by buying duplicate tickets to dilute the LP share value.

## Impact
Leakage of LP funds; reduction in LP share value for every duplicate ticket purchased.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` = 20%, `lpEdgeTarget` = 10%.
2. Ticket Price = 100.
3. User buys duplicate. `lpEarnings` += 80. `prizePool` += 90.
4. LP Pool accounting: Net Change = +80 (Assets) - 90 (Liabilities) = -10.
5. The LP pool loses 10 units of value.

## Proof of Code
function testLPBleed() public {
    // Set referral fee > edge
    // Record initial LP value
    // Buy duplicates
    // Settle drawing
    // Check final LP value decreased unexpectedly
}

## Suggested Mitigation
Enforce invariant `referralFee <= lpEdgeTarget` in the setter functions `setReferralFee` and `setLpEdgeTarget`.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-12]. Permanent DoS of LP system if accumulator becomes zero due to 100% loss

## id: kUQ0krs9R05YUD19_m-gj

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: JackpotLPManager.processDrawingSettlement can set newAccumulator to 0 when postDrawLpValue is 0 and lpPoolTotal > 0 (newAccumulator = prevAcc*0/lpPoolTotal). Subsequent deposit/share conversions divide by drawingAccumulator[...] (e.g., _consolidateDeposits does amount*1e18/accumulator), which would revert on division by zero and brick LP operations. Although comments claim accumulator can never be zero, this is not enforced. Achieving postDrawLpValue==0 appears to require extreme/unsafe configurations (e.g., reserveRatio=0 and referralFee very high so lpEarnings≈0) and extreme outcomes, hence rare and governance-dependent.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
In `JackpotLPManager.processDrawingSettlement`, if the LP pool suffers a 100% loss (possible if `reserveRatio` is 0 and `referralFee` is 100% or `lpEarnings` are zero/negative relative to winnings), `postDrawLpValue` becomes 0. This sets `drawingAccumulator` to 0. Subsequent calls to `processDeposit` divide by `drawingAccumulator` (`shares = amount * PRECISE_UNIT / accumulator`), causing a division by zero revert. This permanently bricks the LP deposit system.

## Impact
The LP system becomes permanently unusable (DoS). New deposits are impossible. Existing positions might be stuck depending on withdrawal logic.

## Command to Run Test


## Proof of Concept
1. Admin sets `reserveRatio` to 0 and `referralFee` to 100% (or high enough to negate ticket earnings for LP).
2. `lpPoolTotal` is 100 USDC.
3. A user wins the entire 100 USDC prize pool.
4. `postDrawLpValue` = 100 (pool) + 0 (earnings) - 100 (winnings) = 0.
5. `newAccumulator` becomes 0.
6. Next `lpDeposit` divides by 0 and reverts.

## Proof of Code
newAccumulator = currentLP.lpPoolTotal == 0 ? PRECISE_UNIT : (drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal;

## Suggested Mitigation
Ensure `postDrawLpValue` is at least 1 wei or handle `accumulator == 0` in deposit logic (e.g. reset to `PRECISE_UNIT` if pool was wiped out but is restarting).


## [M-13]. DoS of Settlement via Config Misalignment (Referral Fee > LP Edge)

## id: hpgzj3DJhm6t19uf5ZBau

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
Dos

## Location
Jackpot.scaledEntropyCallback

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: The underlying economic misconfiguration (referralFee > lpEdgeTarget) is real and can make prizePool accounting grow faster than lpEarnings on duplicates. There is no explicit on-chain safeguard enforcing referralFee<=lpEdgeTarget or preventing settlement from reverting if postDrawLpValue underflows in an extreme lucky outcome. While not a deterministic permissionless DoS, it is still (probabilistically) exploitable under the misconfiguration and would be high impact if it materializes.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol allows the owner to configure `referralFee` and `lpEdgeTarget` independently. If `referralFee` is set higher than `lpEdgeTarget`, duplicate tickets become a liability for the LP pool. For every duplicate ticket purchased, `lpEarnings` increases by `ticketPrice - referralFee`, but `prizePool` (which represents the liability) increases by `ticketPrice - lpEdgeTarget`. The net impact on the LP pool value is `(lpEdgeTarget - referralFee)`. If this value is negative, each duplicate ticket reduces the total LP value. An attacker can purchase a large volume of duplicate tickets to drain the theoretical LP value. If the `postDrawLpValue` calculation in `JackpotLPManager.processDrawingSettlement` underflows (becomes negative) due to these losses, the settlement transaction (the entropy callback) will revert. This permanently bricks the drawing, as settlement cannot proceed.

## Impact
Permanent Denial of Service for the active drawing. Settlement becomes impossible, locking all funds for that drawing unless Emergency Mode is used (which forces a manual refund process).

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` = 20% (0.2e18) and `lpEdgeTarget` = 10% (0.1e18).
2. Attacker buys a large number of duplicate tickets.
3. For each duplicate, the system effectively deducts 10% of the ticket price from the LP pool's net value (Earnings - Liability).
4. If `total_duplicates * (0.1 * price)` exceeds the current `lpPoolTotal`, the calculated `postDrawLpValue` in `processDrawingSettlement` becomes negative.
5. The transaction reverts due to uint256 underflow.
6. The drawing cannot be settled.

## Proof of Code
function testConfigDoS() public {
    // Setup config where referral > edge
    vm.prank(owner);
    jackpot.setLpEdgeTarget(0.1e18);
    vm.prank(owner);
    jackpot.setReferralFee(0.2e18);
    
    // Attacker buys duplicates to cause underflow
    // Note: requires massive capital or small LP pool. 
    // For test, assume small LP pool.
    // ... buyTickets loop ...
    
    // Trigger settlement
    vm.expectRevert(); // Underflow in processDrawingSettlement
    // entropy provider callback
}

## Suggested Mitigation
Enforce the invariant `referralFee <= lpEdgeTarget` in the `setReferralFee` and `setLpEdgeTarget` functions. Additionally, in `processDrawingSettlement`, handle the case where `postDrawLpValue` is negative (e.g., by capping it at 0 or handling insolvency gracefully) rather than allowing it to revert.





Finding Status: LowSeverityDueToRareLikelihood + InvalidByDesign
## [M-14]. Lack of user commitment in entropy request enables provider collusion

## id: RsNQGwuHNH3cYyZm3DvPr

## Derived From Pattern/Invariant
BlockVarsAsPrimaryRandomnessSource

## Exploit Type
Randomness

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: LowSeverityDueToRareLikelihood + InvalidByDesign
### Finding Status Justification: ScaledEntropyProvider.requestAndCallbackScaledRandomness uses entropy.requestV2(entropyProvider, gasLimit) (the variant without userRandomNumber). Per the provided IEntropyV2 comments, this relies on an in-contract PRNG for the user contribution and weakens the trust model: a dishonest validator+provider can collude to manipulate outcomes. This is not a Solidity bug but a real security-property downgrade for a lottery; there is no extra commitment mixed in by the protocol. Exploitability depends on external collusion, hence rare but high impact on fairness if it occurs.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ScaledEntropyProvider` contract calls `entropy.requestV2` without providing a user-generated random number (commitment). It uses the function signature `requestV2(provider, gasLimit)` instead of `requestV2(provider, userRandomNumber, gasLimit)`. As noted in the Pyth Entropy SDK documentation, omitting the user commitment means the security model relies entirely on the honesty of the Entropy provider and validator. If they collude, they can manipulate the generated random number. Since the jackpot determines winners based on this randomness, a malicious provider could theoretically rig the lottery to select specific winning numbers or avoid hitting a jackpot.

## Impact
Compromised fairness of the lottery. A colluding provider can influence the outcome, potentially stealing the prize pool or ensuring no one wins, violating the protocol's core promise of provably fair drawings.

## Command to Run Test


## Proof of Concept
1. `Jackpot.runJackpot` calls `entropy.requestAndCallbackScaledRandomness` with `bytes("")` as context/data.
2. `ScaledEntropyProvider` calls `entropy.requestV2(provider, gasLimit)`.
3. The Pyth contract generates the request without mixing in a user commitment (e.g., `keccak256(blockhash, ...) `).
4. The provider (off-chain) observes the request and can choose a random value that results in a specific outcome (e.g., numbers that no one bought).
5. The provider fulfills the request with the manipulated value.

## Proof of Code
// Inspection of ScaledEntropyProvider.sol
// Line 116
sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit);
// This calls the overload without userRandomNumber.

## Suggested Mitigation
Update `ScaledEntropyProvider` to accept a `userRandomNumber` (or generate one internally using `block.prevrandao` or `blockhash`) and pass it to `entropy.requestV2`. Alternatively, allow `Jackpot.runJackpot` to pass a seed.





Finding Status: LowSeverityDueToRareLikelihood
## [M-15]. Unbounded LP earnings allows `bonusballMax` to exceed bit-packing limits, corrupting tickets

## id: AyGbTmdC5K6cSinVRHRrG

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: Ticket-driven lpEarnings/prizePool growth is permissionless; reaching unsafe normalMax+bonusballMax bounds is not exclusively an admin mistake. However, reaching the extreme scale required can plausibly be rare in practice.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, the `_setNewDrawingState` function calculates `newBonusball` based on the prize pool size to ensure LP edge. The `bonusballMax` logic assumes the prize pool is constrained by `lpPoolCap` (which limits capital such that `normalBallMax + bonusballMax < 256`). However, `lpPoolCap` only constrains *deposits*. `lpEarnings` (ticket revenue) are unbounded and added to the LP value during settlement. If a drawing has extremely high ticket volume, `lpEarnings` can push the `newLPValue` (and thus `newPrizePool`) well beyond the calculated cap. This causes `newBonusball` to be calculated as a value where `normalBallMax + newBonusball >= 256`. 

In the subsequent drawing, `TicketComboTracker.insert` packs tickets using `set |= 1 << (_bonusball + _tracker.normalMax)`. If the sum exceeds 255, the bit shift wraps around (modulo 256), colliding with the normal ball bits (0-255). This corrupts the ticket data, making tickets effectively random or invalid, and breaking `getUnpackedTicket` logic.

## Impact
Corruption of ticket data for drawings with high prize pools, leading to incorrect winners and broken game mechanics.

## Command to Run Test


## Proof of Concept
1. Initialize Jackpot with `normalBallMax` = 250 (leaving only 5 bits for bonus ball).
2. LP Pool is filled to capacity.
3. Users buy a massive amount of tickets for the current drawing, generating huge `lpEarnings`.
4. `runJackpot` and settlement occur.
5. `processDrawingSettlement` adds `lpEarnings` to `newLPValue`, bypassing `lpPoolCap`.
6. `_setNewDrawingState` calculates `newBonusball`. Due to high value, `newBonusball` > 5 (e.g., 10).
7. `250 + 10 = 260`. 
8. Next drawing: User buys ticket with bonus ball 10.
9. `TicketComboTracker.insert` performs `1 << 260`, which wraps to `1 << 4`.
10. This sets the bit for Normal Ball 4, corrupting the ticket.

## Proof of Code
function testBonusballOverflow() public {
    // Mock massive earnings
    uint256 hugeEarnings = 100_000_000 * 1e18;
    // Force settlement with huge earnings
    // Assert next drawing bonusballMax causes wrapping
}

## Suggested Mitigation
In `_setNewDrawingState`, explicitly clamp `newBonusball` such that `normalBallMax + newBonusball < 256`. Alternatively, ensure `lpEarnings` cannot push `prizePool` beyond the safe theoretical limit derived from `MAX_BIT_VECTOR_SIZE`.



