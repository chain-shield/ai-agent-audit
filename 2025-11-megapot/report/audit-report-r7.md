# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[M-1]. DoS of jackpot settlement due to unbounded storage reads in TicketComboTracker
**Derived From** : Custom
Finding Status: Valid
Privilege: Permissionless


[M-2]. DoS of governance parameter updates via front-running LP deposits
**Derived From** : GovernanceFrontrunDoS
Finding Status: Valid
Privilege: Permissionless


[H-3]. Catastrophic LP Edge Collapse via BonusballMax Overflow in Jackpot.sol
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[M-4]. Governance DoS: Liquidity Providers can block `setGovernancePoolCap` updates via front-running deposits
**Derived From** : GovernanceFrontrunDoS
Finding Status: Valid
Privilege: Permissionless


[M-5]. DoS on governance parameter updates via LP pool cap manipulation
**Derived From** : GovernanceFrontrunDoS
Finding Status: Valid
Privilege: Permissionless


[M-6]. Governance DoS via front-running pool cap reduction
**Derived From** : GovernanceFrontrunDoS
Finding Status: Valid
Privilege: Permissionless


[H-7]. Randomness Replay Attack via Unchecked Sequence Number in Jackpot Settlement
**Derived From** : Oracle
Finding Status: Valid
Privilege: Permissionless


[M-8]. DoS on parameter updates due to strict pool cap validation allows LPs to block admin changes
**Derived From** : GovernanceFrontrunDoS
Finding Status: Valid
Privilege: Permissionless


[H-9]. Unbounded ticket sales cause `bonusballMax` overflow leading to broken game logic and LP insolvency
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood


[H-10]. Permanent DoS of Jackpot settlement due to Out-of-Gas in TicketComboTracker loop when bonusballMax is high
**Derived From** : DoS
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[M-11]. Settlement Denial of Service due to excessive gas consumption in `TicketComboTracker` loops
**Derived From** : Gas Limit Denial of Service
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[H-12]. Broken tickets and DoS via bit shift overflow in dynamic bonusball calculation
**Derived From** : IntegerOverflow
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[H-13]. Unsafe downcasting of `bonusballMax` in `Jackpot.sol` leads to catastrophic difficulty reduction and LP pool drainage
**Derived From** : Integer Overflow
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidUserErrorOrMistake


[H-14]. LP Pool Inflation Attack via Share Price Manipulation in JackpotLPManager
**Derived From** : ERC4626 Share Price Inflation
Finding Status: LowSeverityDueToRareLikelihood + InvalidUserErrorOrMistake
Privilege: Permissionless


[H-15]. LP Pool Inflation Attack allows stealing new deposits by manipulating accumulator
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood + InvalidUserErrorOrMistake
Privilege: Permissionless



Finding Status: LowSeverityDueToLowImpact


[M-16]. Incorrect Emergency Refunds due to Mutable Referral Fee
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: LowSeverityDueToLowImpact
Privilege: RequiresAdminRole



Finding Status: InvalidGovernanceRisk


[M-17]. Incorrect tier classification due to bit packing overflow if bonusballMin or normalBallMax are large
**Derived From** : IntegerMath
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-18]. Changing Entropy Provider Locks Active Drawing
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-19]. Administrator can brick active drawings or deny payouts by changing global contracts mid-flow
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-20]. Cross-provider request collision in ScaledEntropyProvider
**Derived From** : ExternalProtocolKeyCollision
Finding Status: InvalidGovernanceRisk
Privilege: RequiresRole


[H-21]. Randomness corruption due to sequence number collision upon entropy provider rotation
**Derived From** : ExternalProtocolKeyCollision
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-22]. Changing Payout Calculator Bricks Historical Winnings Claims
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-23]. Protocol key collision in ScaledEntropyProvider allows request hijacking or state corruption when provider is changed
**Derived From** : ExternalProtocolKeyCollision
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-24]. Protocol fee manipulation mid-drawing affects settlement of LP earnings
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-25]. LPs Lose Money on Duplicate Tickets if Referral Fee Exceeds LP Edge
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-26]. Permanent DoS of Jackpot Settlement via Referral Fee Insolvency
**Derived From** : DoS due to resource exhaustion or logic error leading to revert
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[H-27]. Permanent DoS of LP System if Accumulator drops to Zero
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 14
- M: 13
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. DoS of jackpot settlement due to unbounded storage reads in TicketComboTracker

## id: a-IRkD47kGUPpb-Z_CLl-

## Derived From Pattern/Invariant
Custom

## Exploit Type
Custom

## Location
Jackpot.runJackpot

## Finding Status: Valid
### Finding Status Justification: Settlement does a bonusballMax-sized loop with many storage reads and subset recomputation; there is no on-chain safeguard ensuring it remains under block/keeper gas constraints, so callback DoS is plausible.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library iterates over all possible bonus balls (from 1 to `bonusballMax`) during settlement in `countTierMatchesWithBonusball`. Inside this loop, it iterates over all 31 subsets of the winning normal numbers and performs a storage read `_tracker.comboCounts[i][subsets[l]]` for each.

`bonusballMax` is dynamically calculated based on the prize pool size to maintain LP edge, and can grow up to 255. If `bonusballMax` is high (e.g., 200), the function performs ~6,200 storage reads. While this fits within the block gas limit, the `_calculateEntropyGasLimit` function requests gas from Pyth based on `entropyVariableGasLimit` (250k) per bonus ball. For 200 bonus balls, this requests ~50M gas, which exceeds block gas limits on most chains (e.g., Ethereum 30M), causing `runJackpot` to revert or Pyth to fail the callback.

## Impact
The jackpot becomes un-settleable (DoS) as the prize pool grows, bricks the protocol, and locks user/LP funds until Emergency Mode is activated.

## Command to Run Test


## Proof of Concept
1. Prize pool grows large enough such that `bonusballMax` calculates to 200.
2. Keeper calls `runJackpot`.
3. `_calculateEntropyGasLimit` returns `base + 250,000 * 200 = 50,000,000`.
4. Call to `entropy.requestAndCallbackScaledRandomness` requests 50M gas limit.
5. Transaction reverts because 50M exceeds block gas limit (30M on Eth Mainnet/Base).

## Proof of Code
function testGasDoS() public {
    // Mock state to produce high bonusballMax
    // ... setup large LP pool ...
    // Verify calculated gas limit > block gas limit
    uint32 limit = jackpot.getEntropyCallbackFee(); // internally calls _calculateEntropyGasLimit
    assertTrue(limit > 30_000_000);
}

## Suggested Mitigation
Optimize `TicketComboTracker` to avoid iterating `bonusballMax` (e.g., store total counts per subset independent of bonus ball), or cap `bonusballMax` to a lower safe limit (e.g., 50).


## [M-2]. DoS of governance parameter updates via front-running LP deposits

## id: qAHiKFjJJBKxun0H_7tcb

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
Jackpot.setTicketPrice

## Finding Status: Valid
### Finding Status Justification: This is not an admin mistake/misuse; it is a permissionless actor front-running to block an admin action. Impact is mostly administrative friction (no direct fund loss).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract allows the owner to update critical parameters like `ticketPrice`, `normalBallMax`, `lpEdgeTarget`, and `reserveRatio`. These functions internally call `jackpotLPManager.setLPPoolCap` to recalculate and update the LP pool cap based on the new parameters. The `setLPPoolCap` function strictly reverts if the new calculated cap is less than the current total pool size (`lpPoolTotal + pendingDeposits`).

This creates a denial-of-service vector where any user (or existing LP) can front-run a governance transaction that would lower the pool cap (e.g., lowering ticket price or increasing LP edge) by depositing enough USDC into the LP pool to exceed the new cap. This causes the governance transaction to revert, effectively preventing the protocol from updating its parameters.

## Impact
Protocol administration is blocked; owner cannot update critical economic parameters if LPs oppose the change.

## Command to Run Test


## Proof of Concept
1. Admin decides to lower `ticketPrice` from 5 USDC to 2 USDC, which reduces the calculated `lpPoolCap` from 10M to 4M.
2. Current `lpPoolTotal` is 3.9M.
3. Attacker (LP) sees the admin transaction in the mempool.
4. Attacker front-runs with `lpDeposit(200k USDC)`, raising the pool total to 4.1M.
5. Admin transaction executes: calculates new cap of 4M.
6. `setLPPoolCap` checks `4M < 4.1M` and reverts with `InvalidLPPoolCap`.
7. Admin is unable to update the ticket price.

## Proof of Code
function testGovernanceFrontrun() public {
    // Setup initial state
    vm.startPrank(owner);
    jackpot.initializeLPDeposits(1000e18);
    jackpot.initializeJackpot(block.timestamp + 1000);
    vm.stopPrank();
    
    // Deposit to near cap
    uint256 currentCap = jackpotLPManager.lpPoolCap();
    vm.prank(user);
    usdc.approve(address(jackpot), currentCap);
    vm.prank(user);
    jackpot.lpDeposit(currentCap - 100e6); // Fill most of pool

    // Admin tries to lower ticket price (which lowers cap)
    // Attacker frontruns
    vm.prank(attacker);
    usdc.approve(address(jackpot), 200e6);
    vm.prank(attacker);
    jackpot.lpDeposit(200e6); // Push over the *new* cap threshold

    // Admin tx reverts
    vm.startPrank(owner);
    vm.expectRevert(JackpotErrors.InvalidLPPoolCap.selector);
    jackpot.setTicketPrice(1e6); // Try to lower price
    vm.stopPrank();
}

## Suggested Mitigation
Modify `setLPPoolCap` to cap the value at `max(newCap, currentTotal)` instead of reverting, or allow the cap update to proceed while disabling *new* deposits until the pool shrinks below the cap.


## [H-3]. Catastrophic LP Edge Collapse via BonusballMax Overflow in Jackpot.sol

## id: 81Im1gNmO1b1-MaJGC1WZ

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.sol._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: No safeguard prevents uint8 truncation in Jackpot._setNewDrawingState; with uncapped ticket-driven lpEarnings, required bonusballMax can exceed 255 and wrap, collapsing difficulty and breaking LP-edge assumptions.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, the `_setNewDrawingState` function calculates the `newBonusball` count for the next drawing to ensure LPs maintain a statistical edge. The calculation `newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)))` casts the result to `uint8`. 

If the required bonus ball count exceeds 255 (which happens when `prizePool` is large relative to `combosPerBonusball`), the cast wraps around (modulo 256). 

For example, if `normalBallMax=10` (252 combinations) and the prize pool requires 258 bonus balls to maintain edge (~$65k pool at $1 ticket), the calculation results in `uint8(258) = 2`. The difficulty collapses from 258 to 2. This drastically increases player win rates far beyond the priced probabilities, ensuring the LP pool is drained over time.

## Impact
LPs suffer massive negative expected value (EV) leading to insolvency of the protocol.

## Command to Run Test


## Proof of Concept
1. Admin deploys Jackpot with `normalBallMax=10` (yielding 252 combinations) and `ticketPrice=1e6` (1 USDC).
2. Users buy tickets, pushing `prizePool` to ~66,000 USDC.
3. `runJackpot` is called. In `scaledEntropyCallback`, `_setNewDrawingState` runs.
4. `minNumberTickets` ≈ 66,000 / (1-edge). Assuming edge=0.1, tickets=73,333.
5. `combosPerBonusball` = 252.
6. `rawBonus` = ceil(73,333 / 252) ≈ 291.
7. `newBonusball` = `uint8(291)` = 35. (291 % 256).
8. The difficulty required is 291, but the system sets it to 35. The cost to cover all combinations is drastically lower than the payout, allowing attackers to print money.

## Proof of Code
function testOverflow() public { uint256 raw = 258; uint8 capped = uint8(raw); assert(capped == 2); }

## Suggested Mitigation
Cap the `bonusballMax` at 255 instead of casting/wrapping, or use a larger integer type (and ensure bitpacking supports it).


## [M-4]. Governance DoS: Liquidity Providers can block `setGovernancePoolCap` updates via front-running deposits

## id: MO6I4Bv9bOZMsyRfok8lN

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
Jackpot.setGovernancePoolCap

## Finding Status: Valid
### Finding Status Justification: This is not caused by admin mistake; it is a permissionless front-run DoS of an admin cap-reduction. It is primarily an admin-operability issue (no direct user fund theft).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `setGovernancePoolCap` function allows the admin to reduce the maximum size of the LP pool. This function calls `JackpotLPManager.setLPPoolCap`, which enforces that the new cap must be greater than or equal to the current total pool size (`lpPoolTotal + pendingDeposits`). 

```solidity
if (_lpPoolCap < currentLP.lpPoolTotal + currentLP.pendingDeposits) revert InvalidLPPoolCap();
```

If the admin attempts to lower the cap, existing LPs can observe the transaction in the mempool and front-run it with a `lpDeposit` transaction. By depositing enough funds to make the total pool size exceed the admin's target `_governancePoolCap`, they force the admin's transaction to revert. This allows LPs to effectively veto pool size reductions and maintain a larger pool size than the protocol intends.

## Impact
Governance is unable to reduce protocol exposure or limit pool size, potentially locking the protocol in a high-risk state.

## Command to Run Test


## Proof of Concept
1. Current `lpPoolTotal` is 5M USDC. `governancePoolCap` is 10M.
2. Admin submits tx to set `governancePoolCap` to 6M.
3. Attacker (LP) sees tx and front-runs with a deposit of 1.1M USDC.
4. New total is 6.1M.
5. Admin tx executes: `6M < 6.1M` check fails. Revert.
6. Admin cannot lower the cap.

## Proof of Code
function testBlockGovernanceCapChange() public {
    // Admin tries to lower cap
    // User frontruns with deposit
    vm.prank(user);
    jackpot.lpDeposit(amount);
    
    vm.prank(owner);
    vm.expectRevert(JackpotErrors.InvalidLPPoolCap.selector);
    jackpot.setGovernancePoolCap(newLowerCap);
}

## Suggested Mitigation
Remove the strict check in `setLPPoolCap`. Instead, allow the new cap to be set lower than the current total, but prevent *new* deposits until the total falls below the new cap naturally (via withdrawals or losses).


## [M-5]. DoS on governance parameter updates via LP pool cap manipulation

## id: JOscw8_Z3AomXv7ZViVrK

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
Jackpot.setNormalBallMax

## Finding Status: Valid
### Finding Status Justification: Permissionless users can front-run cap-reducing governance updates by increasing pendingDeposits, causing setLPPoolCap to revert; there is no protocol safeguard against this kind of governance griefing.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract allows the owner to update critical parameters like `normalBallMax`, `ticketPrice`, `lpEdgeTarget`, and `reserveRatio`. These setters trigger `jackpotLPManager.setLPPoolCap`, which calculates a new `lpPoolCap` based on the new parameters. The `setLPPoolCap` function enforces that the new cap must be greater than or equal to the current total LP deposits (`currentLP.lpPoolTotal + currentLP.pendingDeposits`).

Snippet in `JackpotLPManager.sol`:
```solidity
function setLPPoolCap(uint256 _drawingId, uint256 _lpPoolCap) external onlyJackpot {
    LPDrawingState storage currentLP = lpDrawingState[_drawingId];
    if (_lpPoolCap < currentLP.lpPoolTotal + currentLP.pendingDeposits) revert InvalidLPPoolCap();
    lpPoolCap = _lpPoolCap;
}
```
An attacker can front-run a governance transaction (e.g., reducing `normalBallMax` or `ticketPrice`) by depositing a large amount of USDC into the LP pool. This increases `pendingDeposits`. If the parameter change results in a lower calculated pool cap that is below the new total deposits, the governance transaction will revert. This effectively allows LPs to block parameter updates they view as unfavorable.

## Impact
Governance is unable to update protocol parameters, potentially preventing critical economic adjustments or security fixes.

## Command to Run Test


## Proof of Concept
1. Admin submits transaction to call `setNormalBallMax(newMax)` which would result in a lower `lpPoolCap` of 1,000,000 USDC.
2. Current LP pool is 900,000 USDC.
3. Attacker sees the tx and front-runs it with `lpDeposit(150,000 USDC)`.
4. Total deposits become 1,050,000 USDC.
5. Admin tx executes. `_calculateLpPoolCap` returns 1,000,000 USDC.
6. `setLPPoolCap` checks `1,000,000 < 1,050,000` and reverts with `InvalidLPPoolCap`.
7. Admin transaction fails.

## Proof of Code
pending

## Suggested Mitigation
Remove the revert in `setLPPoolCap` and instead clamp the new `lpPoolCap` to `max(calculatedCap, currentTotalDeposits)`, or allow the cap to be set lower than current deposits (blocking only *new* deposits).


## [M-6]. Governance DoS via front-running pool cap reduction

## id: vU6IIm9kDsGt2K01gDtl0

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
Jackpot.setReserveRatio

## Finding Status: Valid
### Finding Status Justification: The DoS is caused by permissionless deposits, not privileged misuse. The primary consequence is delaying/restricting admin parameter changes rather than direct loss of funds.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Several administrative functions in `Jackpot` (`setNormalBallMax`, `setTicketPrice`, `setLpEdgeTarget`, `setReserveRatio`, `setGovernancePoolCap`) trigger a recalculation of the LP pool cap via `jackpotLPManager.setLPPoolCap`. This function reverts if the new calculated cap is lower than the current total deposits (`currentLP.lpPoolTotal + currentLP.pendingDeposits`).

Because parameters like `reserveRatio` directly affect the pool cap calculation (specifically, decreasing the `reserveRatio` increases the capital efficiency requirement, thereby lowering the maximum allowed LP pool size for a fixed max prize liability), an attacker can front-run a governance transaction that intends to lower the cap (e.g., decreasing reserve ratio) by depositing enough funds to fill the pool up to the current cap. This causes the governance transaction to revert, effectively preventing the protocol from adjusting risk parameters.

## Impact
Prevents protocol administrators from updating critical economic parameters, potentially leaving the protocol in a suboptimal or risky configuration.

## Command to Run Test


## Proof of Concept
1. Current `reserveRatio` is 0.5. `lpPoolCap` is 200,000 USDC. Current deposits are 100,000 USDC.
2. Admin submits tx to `setReserveRatio(0.1)`. This reduces the required collateral buffer, but based on the formula `maxPrizePool / (1-R)`, a lower R actually reduces the multiplier? Wait, `maxPrizePool` is fixed. `LP <= maxPrizePool / (1-R)`. If R=0.5, LP <= 2*Max. If R=0.1, LP <= 1.11*Max. So decreasing reserve ratio DECREASES the pool cap.
3. The new cap would be 111,000 USDC.
4. Attacker observes the tx and front-runs with an `lpDeposit` of 90,000 USDC. Total deposits = 190,000 USDC.
5. Admin tx executes. `_calculateLpPoolCap` returns 111,000.
6. `JackpotLPManager.setLPPoolCap` checks `111,000 < 190,000`. Reverts.
7. Admin is blocked from changing the parameter.

## Proof of Code
function testGovernanceDoS() public {
    // Setup initial state with high cap
    // ... 
    // Admin tries to lower cap
    // Attacker deposits
    vm.prank(attacker);
    jackpot.lpDeposit(fillAmount);
    // Admin tx reverts
    vm.prank(owner);
    vm.expectRevert(JackpotErrors.InvalidLPPoolCap.selector);
    jackpot.setReserveRatio(newRatio);
}

## Suggested Mitigation
In `JackpotLPManager.setLPPoolCap`, do not revert if the new cap is below current deposits. Instead, allow the cap to be set lower, and simply block *new* deposits until the total pool size falls below the new cap (which is already handled by `processDeposit`).


## [H-7]. Randomness Replay Attack via Unchecked Sequence Number in Jackpot Settlement

## id: ZBzljivZOyC0Mw1ak2o07

## Derived From Pattern/Invariant
Oracle

## Exploit Type
Oracle

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: Ignoring the sequence in Jackpot.scaledEntropyCallback can allow fulfilling a later-locked drawing with an earlier pending request if a prior reveal+callback attempt reverted (e.g., OOG), since the old request remains fulfillable and its randomness becomes computable from reverted tx calldata.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract's `scaledEntropyCallback` function fails to validate that the randomness provided corresponds to the current drawing's request. Specifically, it ignores the `_sequence` number passed from the `ScaledEntropyProvider`. 

In the Pyth Entropy workflow, `runJackpot` triggers a request returning a unique sequence number. If a callback for a specific drawing fails (e.g. due to gas or intentional obstruction) but the randomness is revealed on-chain, and the drawing is subsequently unlocked by the admin to reset state, a malicious user can purchase winning tickets based on the known randomness. When the admin re-runs `runJackpot` (generating a NEW request ID), a keeper can replay the OLD callback (which is still pending in `ScaledEntropyProvider`). `Jackpot` will accept this old randomness for the current drawing because it checks only `jackpotLock` and `msg.sender`, ignoring the sequence number mismatch. This allows attackers to guarantee a win.

## Impact
Complete theft of the prize pool by predicting winning numbers.

## Command to Run Test


## Proof of Concept
1. Admin calls `runJackpot` for Drawing N (Sequence 1). Jackpot locks.
2. Keeper calls Pyth reveal, but the `scaledEntropyCallback` reverts (e.g. OOG). Randomness R1 is now public.
3. Request 1 remains pending in `ScaledEntropyProvider`.
4. Admin calls `unlockJackpot` to fix the stuck state.
5. Attacker buys tickets matching R1.
6. Admin calls `runJackpot` again (Sequence 2). Jackpot locks.
7. Attacker (Keeper) triggers the Pyth callback for Sequence 1 manually (or it is retried).
8. `ScaledEntropyProvider` calls `Jackpot.scaledEntropyCallback` with R1.
9. `Jackpot` verifies `jackpotLock` is true, ignores that sequence is 1 (not 2), and settles Drawing N with R1.
10. Attacker wins.

## Proof of Code
function testExploitRandomnessReplay() public { 
  // Simulating state where req 1 is pending and randomness known 
  uint64 staleSeq = 1; 
  uint256[][] memory randoms = new uint256[][](2); 
  // ... setup randoms ... 
  vm.prank(address(entropyProvider)); 
  // Jackpot expects lock, admin runs runJackpot -> locks it 
  // But entropyProvider passes staleSeq, Jackpot ignores it 
  jackpot.scaledEntropyCallback(staleSeq, randoms, ""); 
  // Assert winner is determined by stale randoms 
}

## Suggested Mitigation
Store the `requestId` (sequence number) returned by `entropy.requestAndCallbackScaledRandomness` in the `DrawingState` struct. In `scaledEntropyCallback`, verify that the `_sequence` argument matches the stored `requestId` for the current drawing.


## [M-8]. DoS on parameter updates due to strict pool cap validation allows LPs to block admin changes

## id: k5j5_2SdzZqOTKAUypCsC

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
JackpotLPManager.setLPPoolCap

## Finding Status: Valid
### Finding Status Justification: This is permissionless interference with governance actions, not an admin error. The effect is mainly operational/administrative (no direct theft).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager.setLPPoolCap` function enforces that the new pool cap must be greater than or equal to the current total LP deposits (`currentLP.lpPoolTotal + currentLP.pendingDeposits`). This function is called internally when the admin updates core parameters via `Jackpot` functions like `setTicketPrice`, `setNormalBallMax`, or `setLpEdgeTarget`, as these parameters fundamentally change the risk profile and thus the calculated maximum safe pool size (`_calculateLpPoolCap`).

If the admin attempts to change a parameter that results in a lower pool cap (e.g., lowering `ticketPrice`), and the current pool usage is high, the transaction will revert. Malicious LPs or users can front-run the admin's transaction by depositing enough USDC into the LP pool to ensure `total + pending > new_calculated_cap`, thereby causing the admin's update transaction to revert. This effectively allows LPs to veto protocol parameter changes.

## Impact
Inability for governance to update critical protocol parameters (Ticket Price, Ball Max, Edge Target) if the pool is heavily utilized, potentially locking the protocol in a suboptimal or vulnerable configuration.

## Command to Run Test


## Proof of Concept
1. Current `ticketPrice` is 2 USDC. `lpPoolTotal` is 900k. `lpPoolCap` is 1M.
2. Admin submits tx to `setTicketPrice(1 USDC)`. This recalculates `lpPoolCap` to 500k (halved).
3. Attacker sees tx in mempool.
4. Attacker calls `lpDeposit(200k)`. `lpPoolTotal` + `pending` becomes 1.1M.
5. Admin tx executes. `_calculateLpPoolCap` returns 500k.
6. `setLPPoolCap` checks `500k < 1.1M` and reverts with `InvalidLPPoolCap`.

## Proof of Code
function testGovernanceDoS() public {
    uint256 currentCap = jackpotLPManager.lpPoolCap();
    // Admin wants to lower price which lowers cap
    // Attacker deposits to fill pool
    vm.startPrank(attacker);
    usdc.approve(address(jackpot), currentCap);
    jackpot.lpDeposit(currentCap); // Fill the pool
    vm.stopPrank();
    
    // Admin update fails
    vm.startPrank(owner);
    vm.expectRevert(JackpotErrors.InvalidLPPoolCap.selector);
    jackpot.setTicketPrice(newLowerPrice);
    vm.stopPrank();
}

## Suggested Mitigation
Modify `setLPPoolCap` to allow the cap to be set lower than current deposits (perhaps with a warning event), or restrict deposits only when they *increase* the total beyond the cap, rather than enforcing the cap invariant strictly during the setting of the cap itself.


## [H-9]. Unbounded ticket sales cause `bonusballMax` overflow leading to broken game logic and LP insolvency

## id: OuWHK0H75mdg5C0FwDIr1

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Ticket revenue (lpEarnings) is uncapped and feeds into newLpValue/prizePool, which can push the required bonusballMax past the bitpacking/casting assumptions; there is no hard on-chain cap tying ticket sales to (255-normalBallMax) or preventing the unsafe uint8 cast wrap.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol dynamically calculates `bonusballMax` for each new drawing to ensure the number of possible ticket combinations is sufficient to guarantee an LP edge. The formula is `bonusballMax = ceil(minNumberTickets / combinations(normalBallMax, 5))`, where `minNumberTickets` is derived from the `prizePool`. 

The `prizePool` for the next drawing is determined by `lpPoolTotal + lpEarnings - payouts`. Crucially, while `lpPoolTotal` is capped by `lpPoolCap` (limiting LP deposits), `lpEarnings` (ticket revenue) is **uncapped**. 

If ticket sales are sufficiently high, the `prizePool` will grow large enough that the calculated `bonusballMax` exceeds the system's technical limits. Specifically:
1. If `bonusballMax + normalBallMax >= 256`, the bit-packing logic in `TicketComboTracker` fails (`1 << shift` results in 0 for shift >= 256), causing the bonus ball component of tickets to be effectively lost (recorded as 0). This breaks the game logic, as all tickets with the same normal numbers will be treated as duplicates regardless of their bonus ball selection, and winning matching logic will be corrupted.
2. If the calculated `bonusballMax` exceeds 255, the explicit `uint8` cast in `_setNewDrawingState` truncates the value (e.g., 300 becomes 44). This results in a much smaller probability space than required for the prize pool size, giving players a massively positive Expected Value (EV) at the expense of LPs, leading to insolvency.

With a small `normalBallMax` (e.g., 20), the safe pool cap is approx $3.6M USDC. Ticket sales exceeding this amount in a single drawing will trigger this vulnerability.

## Impact
Catastrophic loss of LP funds due to broken game mechanics (bit packing failure) or drastically reduced game difficulty (truncation) leading to +EV for players.

## Command to Run Test


## Proof of Concept
1. Initialize Jackpot with `normalBallMax = 10`. `C(10,5) = 252`. Max safe `bonusballMax = 255 - 10 = 245`. Max safe prize pool ~ 252 * 245 * $1 = $61,740.
2. LPs deposit funds.
3. Attackers (or genuine demand) buy tickets worth $100,000 USDC. `lpEarnings` increases by ~$100k.
4. Drawing settles. `_setNewDrawingState` calculates `minNumberTickets` based on the new >$100k pool.
5. `minNumberTickets` > 61,740. Calculated `bonusballMax` > 245.
6. If calculation results in 250: `250 + 10 = 260`. `TicketComboTracker` uses `1 << 260` which is 0. All tickets have bonus ball 0. Game broken.
7. If calculation results in 300: `uint8(300)` truncates to 44. Difficulty is set for a $11k pool but actual pool is >$100k. Players have massive edge.

## Proof of Code
function testBonusBallOverflow() public {
    // Setup: normalBallMax=10, ticketPrice=1e6 (1 USDC)
    // C(10,5) = 252.
    // Max valid bonusballMax = 245.
    // Max Pool = 252 * 245 * 1e6 = 61,740 USDC.
    
    // Assume we are in a state where drawing 1 is active
    // Buy tickets exceeding the safe cap
    uint256 safeCap = 61740 * 1e6; 
    uint256 buyAmount = safeCap + 20000 * 1e6; // Exceed cap
    
    vm.startPrank(user);
    usdc.approve(address(jackpot), buyAmount);
    // Buy tickets (simplified simulation of purchasing loop)
    // In real test, loop buyTickets calls
    // For PoC, we assume lpEarnings is increased directly or via helper
    vm.store(address(jackpot), bytes32(uint256(10)), bytes32(buyAmount)); // Mock lpEarnings slot
    vm.stopPrank();

    // Trigger settlement
    vm.prank(address(entropyProvider));
    jackpot.scaledEntropyCallback(1, randomNumbers, "");
    
    // Check new drawing state
    (,,,,,,, uint8 newBonusballMax,,) = jackpot.getDrawingState(2);
    
    // If it truncated, it will be small. If it didn't truncate but overflowed packing:
    // In this specific config, overflow is likely if not truncated.
    // If buyAmount implies bonusballMax = 300, newBonusballMax will be 44 (truncation)
    // This confirms the vulnerability.
}

## Suggested Mitigation
In `_setNewDrawingState`, cap the `prizePool` used for parameter calculation such that `bonusballMax` never exceeds `255 - normalBallMax`. Any excess `lpEarnings` should be held in reserve for future drawings or the reserve ratio should be dynamically adjusted.





Finding Status: LowSeverityDueToRareLikelihood
## [H-10]. Permanent DoS of Jackpot settlement due to Out-of-Gas in TicketComboTracker loop when bonusballMax is high

## id: cvsZDtMCx1pMhyYqfSQoh

## Derived From Pattern/Invariant
DoS

## Exploit Type
Dos

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: TicketComboTracker._countSubsetMatches loops i=1..bonusballMax and for each i iterates all 31 subsets of the 5 winning normals, reading comboCounts[i][subset] (count/dupCount). Gas is linear in bonusballMax and uses many cold mapping reads plus repeated Combinations.generateSubsets allocations. bonusballMax can reach near 255 in principle (it is uint8 and not clamped to a small value). If gas exceeds the entropy callback’s effective block gas constraints, Jackpot.scaledEntropyCallback will revert and the drawing remains locked. There is no batching or alternative settlement path; emergency mode is the only recovery. Whether this hits limits depends on chain gas limits and configured parameters, so likelihood is Rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `runJackpot` process triggers `scaledEntropyCallback`, which calls `JackpotLPManager.processDrawingSettlement`. This eventually executes `TicketComboTracker.countTierMatchesWithBonusball`. This function contains nested loops that iterate through every possible bonusball (from 1 to `bonusballMax`), every normal tier (1 to 5), and every subset combination of the winning numbers. Specifically, `_countSubsetMatches` performs approximately `bonusballMax * 31` iterations. Inside each iteration, it performs storage reads on `comboCounts` to retrieve ticket counts. 

The system caps `bonusballMax` indirectly via `lpPoolCap`, which ensures `bonusballMax + normalBallMax <= 255`. If `normalBallMax` is 35 (standard), `bonusballMax` can reach 220. With 220 bonusballs, the loop performs roughly 220 * 31 = 6,820 iterations. Each iteration reads two storage slots (count and dupCount). Assuming cold access (2100 gas), 6,820 * 2 * 2100 ≈ 28.6 million gas. This approaches or exceeds the standard Ethereum/L2 block gas limit (30M). Liquidity Providers can intentionally or accidentally fund the pool to the cap, maximizing `prizePool` and thus `bonusballMax`, causing the settlement callback to consistently revert due to Out-of-Gas. This permanently locks the drawing and requires Emergency Mode to resolve.

## Impact
The current drawing cannot be settled, locking all ticket funds and LP deposits until emergency mode is activated.

## Command to Run Test


## Proof of Concept
1. LPs deposit USDC into the Jackpot until the pool reaches `lpPoolCap`. 
2. This maximizes the `prizePool` for the next drawing.
3. The `_setNewDrawingState` function calculates `bonusballMax` based on the `prizePool`. With a full pool, `bonusballMax` is set to its maximum theoretical value (e.g., 220).
4. Users buy tickets. `TicketComboTracker` stores them.
5. `runJackpot` is called after `drawingTime`. It requests entropy.
6. The entropy provider calls `scaledEntropyCallback`.
7. The callback executes `_countSubsetMatches`. The loop over 220 bonusballs consumes ~29M+ gas reading storage.
8. The transaction reverts due to Out-of-Gas.
9. The drawing remains locked (`jackpotLock` = true), and no new drawing can start.

## Proof of Code
function testDoSWithMaxBonusball() public {
    // Setup drawing with max pool parameters
    // Assume normalBallMax = 35
    // Deposit enough LP to push bonusballMax to 220
    
    // Mock internal storage state for 220 bonusballs
    // Call countTierMatchesWithBonusball
    // Assert gas used > 30_000_000
}

## Suggested Mitigation
Optimize `TicketComboTracker` to avoid iterating over all `bonusballMax` in the settlement phase. Instead of storing counts per bonusball, store aggregate counts for 'any bonusball' vs 'specific bonusball' separately during insertion, or reduce the `lpPoolCap` calculation to ensure `bonusballMax` stays within a safe gas limit (e.g., < 100).


## [M-11]. Settlement Denial of Service due to excessive gas consumption in `TicketComboTracker` loops

## id: xS2Y_fAus1r_8J88erfAp

## Derived From Pattern/Invariant
Gas Limit Denial of Service

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.scaledEntropyCallback

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: Same gas-scaling issue as cvsZDt: Jackpot.scaledEntropyCallback calls TicketComboTracker.countTierMatchesWithBonusball(), which iterates across all bonusball values up to bonusballMax and performs many mapping reads and subset generations. If bonusballMax is high and chain/block gas constraints are tight, the callback can revert, leaving the drawing locked. No batching exists; emergency mode is the only recovery. The exact gas ceiling depends on chain and parameters, so likelihood is Rare but impact is material.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function calls `_calculateDrawingUserWinnings`, which invokes `TicketComboTracker.countTierMatchesWithBonusball`. This function contains nested loops: it iterates `i` from 1 to `_bonusballMax`, and for each bonusball, it generates all subsets of the winning numbers (31 subsets) and performs storage reads (`comboCounts`).

If `bonusballMax` is high (near 255), the function performs 255 * 31 = 7,905 storage reads. With cold storage access (2100 gas), this alone costs ~16.6 million gas, not including other logic. This can exceed the block gas limit on many chains (or the `entropyGasLimit` budgeted), causing the callback to revert. Since the drawing is locked (`jackpotLock = true`) until the callback succeeds, a revert permanently locks the jackpot, forcing an emergency exit.

## Impact
The jackpot settlement transaction reverts due to Out of Gas. The protocol becomes stuck in a locked state, requiring the owner to enable emergency mode and users to withdraw funds, effectively killing the protocol instance.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is configured such that `combosPerBonusball` is relatively low (e.g., 20).
2. `prizePool` grows large enough that the dynamic `bonusballMax` calculation sets it to 250.
3. The drawing is run (`runJackpot`).
4. Pyth invokes `scaledEntropyCallback`.
5. The callback iterates 250 times. Each iteration reads 31 storage slots.
6. Total gas > 15M. Transaction reverts.
7. Jackpot remains locked forever.

## Proof of Code
function testGasCost() public {
    // Simulation of storage reads loop
    uint256 gasStart = gasleft();
    uint256 bonusballMax = 250;
    for (uint i=0; i < bonusballMax; i++) {
        for (uint j=0; j < 31; j++) {
             // Simulate SLOAD cost
             assembly { pop(sload(j)) }
        }
    }
    uint256 gasUsed = gasStart - gasleft();
    console.log("Gas used:", gasUsed);
}

## Suggested Mitigation
Optimize the `TicketComboTracker` data structure to avoid iterating all bonusballs (e.g., store a separate mapping for bonusball matches). Alternatively, implement a multi-step settlement process where winners are counted in batches.


## [H-12]. Broken tickets and DoS via bit shift overflow in dynamic bonusball calculation

## id: 7YNZKiA4B-bJnebeCJLsO

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: Jackpot._setNewDrawingState computes newDrawingState.bonusballMax as uint8(...) without clamping to 255-normalBallMax. TicketComboTracker.insert() and countTierMatchesWithBonusball() both compute (1 << (_bonusball + _tracker.normalMax)) where _bonusball and normalMax are uint8; if _bonusball + normalMax > 255, the addition overflows and REVERTS (Solidity 0.8 checked arithmetic). That can (a) prevent buying tickets for high bonusball values and, worse, (b) revert settlement if the winning bonusball drawn exceeds 255-normalMax, locking the drawing. The report’s “shift becomes 0” detail is inaccurate, but the DoS-by-overflow is real.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract dynamically calculates `bonusballMax` based on the prize pool size to maintain LP edge. This value is cast to `uint8` and can reach up to 255. The `TicketComboTracker` library packs the ticket numbers into a `uint256` bit vector using the shift operation `1 << (_bonusball + _tracker.normalMax)`. 

However, there is no check ensuring that `_bonusball + _tracker.normalMax < 256`. If the sum equals or exceeds 256, the shift operation overflows to 0 in Solidity 0.8+, resulting in a ticket stored without the bonus ball bit set. 

This creates a mismatch between the stored ticket (bonusball = 0/invalid) and the user's intended bonus ball. Consequently, `unpackTicket` will revert due to `LibBit.fls` underflow when trying to retrieve the bonus ball, causing a DoS on view functions and potentially the `claimWinnings` function (depending on implementation specifics of `getTicketInfo` usage). More critically, the user pays for a ticket that effectively has a bonus ball of 0 (impossible to match winning numbers >= 1), creating a guaranteed loss of the bonus match portion of the prize.

## Impact
Users purchase validly minted but functionally broken tickets that cannot win bonus matches. View functions revert, degrading UI/UX. Permanent loss of user funds spent on broken tickets.

## Command to Run Test


## Proof of Concept
1. Deploy Jackpot with `normalBallMax = 20`. `combos = 15504`.
2. LPs deposit and ticket sales push `prizePool` to ~3.7M USDC.
3. `runJackpot` is called. `_setNewDrawingState` calculates `minNumberTickets` ~ 3.7M.
4. `newBonusball` calculated as `3.7M / 15504` ~= 238.
5. `bonusballMax` set to 238.
6. User calls `buyTickets` picking bonus ball 238.
7. `TicketComboTracker.insert` computes shift: `1 << (238 + 20) = 1 << 258 = 0`.
8. Ticket stored with only normal ball bits.
9. User calls `getExtendedTicketInfo` -> `unpackTicket` -> `fls(set) - 20`. `fls` returns <= 20. Underflow revert.

## Proof of Code
function testBonusBallOverflow() public {
    // Setup handled in setUp()
    // Mock internal state to simulate high bonusball condition
    // Assume normalBallMax = 20
    // Set bonusball = 240
    uint8 normalMax = 20;
    uint8 bonusball = 240;
    
    // Logic from TicketComboTracker.insert
    uint256 set = 1; // dummy normal bit
    uint256 packed = set | (1 << (bonusball + normalMax));
    
    // packed should be 'set' (1) because shift overflowed to 0
    assertEq(packed, 1);
    
    // Verify unpack revert
    vm.expectRevert();
    // Logic from TicketComboTracker.unpackTicket
    uint8 recoveredBonus = uint8(LibBit.fls(packed) - normalMax);
}

## Suggested Mitigation
In `Jackpot._setNewDrawingState`, clamp `newBonusball` such that `newBonusball + normalBallMax < 256`. Example: `newBonusball = uint8(Math.min(calculatedBonus, 255 - normalBallMax));`.


## [H-13]. Unsafe downcasting of `bonusballMax` in `Jackpot.sol` leads to catastrophic difficulty reduction and LP pool drainage

## id: AdtLyq-trzWccFEjwg5-2

## Derived From Pattern/Invariant
Integer Overflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: In Jackpot._setNewDrawingState: uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball))); If the required value exceeds 255, the uint8 cast truncates modulo 256 (no revert). This can set bonusballMax far below what economic sizing requires, reducing game difficulty and undermining the LP edge model. Additionally, the truncated value may still exceed (255-normalBallMax), causing the separate uint8 overflow reverts in TicketComboTracker (DoS). There is no safe-cast (UintCasts.toUint8) or clamp to 255-normalBallMax.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, the `_setNewDrawingState` function dynamically calculates the `bonusballMax` for the next drawing to ensure the Liquidity Provider (LP) edge is maintained. The formula calculates `minNumberTickets` based on the prize pool and edge target, then determines the necessary `bonusballMax` to dilute winning probabilities sufficiently. 

However, the result of `Math.ceilDiv(minNumberTickets, combosPerBonusball)` (a `uint256`) is explicitly cast to `uint8`: 
`uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));`

In Solidity 0.8+, explicit casting does not revert on overflow; it truncates. If the calculated requirement exceeds 255 (e.g., 300), it wraps around (e.g., 44). This results in a game difficulty far lower than economically required, allowing users to win significantly more often than the protocol accounts for, rapidly draining the LP pool.

## Impact
The game difficulty is drastically reduced (e.g., from 1/300 odds to 1/44), breaking the economic design. Players win frequently, draining the prize pool and causing massive losses for Liquidity Providers.

## Command to Run Test


## Proof of Concept
1. Assume `normalBallMax` = 15. `combosPerBonusball` = choose(15,5) = 3003.
2. Assume `prizePool` grows to 2,000,000 USDC. `ticketPrice` = 1 USDC. `lpEdgeTarget` = 0.2 (20%).
3. `minNumberTickets` = 2,000,000 / (0.8 * 1) = 2,500,000.
4. Required `bonusballMax` = ceil(2,500,000 / 3003) = 833.
5. The code executes `uint8(833)`. 833 % 256 = 65.
6. The new drawing sets `bonusballMax` to 65 instead of 833.
7. Winning probability is ~12x higher than intended, draining the pool.

## Proof of Code
contract TestOverflow {
    function testTruncation() public pure returns (uint8, uint256) {
        uint256 required = 833;
        uint8 truncated = uint8(required);
        return (truncated, required); // returns (65, 833)
    }
}

## Suggested Mitigation
Use the provided `UintCasts` library or safe casting to revert on overflow. Additionally, cap the `bonusballMax` at 255 in the logic, or cap the `prizePool` growth to ensure the required difficulty never exceeds 255.





Finding Status: LowSeverityDueToRareLikelihood + InvalidUserErrorOrMistake
## [H-14]. LP Pool Inflation Attack via Share Price Manipulation in JackpotLPManager

## id: gEPoPkb5aYGF_3RnIOCz2

## Derived From Pattern/Invariant
ERC4626 Share Price Inflation

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager.processDeposit

## Finding Status: LowSeverityDueToRareLikelihood + InvalidUserErrorOrMistake
### Finding Status Justification: LP share minting uses integer division: _consolidateDeposits mints shares as (amount * 1e18) / drawingAccumulator[depositDrawingId]. If accumulator becomes very large, later depositors can receive 0 shares (rounding to zero) and effectively donate their deposit to existing shareholders. Accumulator can become very large if lpDrawingState[d].lpPoolTotal is extremely small relative to postDrawLpValue in processDrawingSettlement (newAccumulator = prevAcc * postDrawLpValue / lpPoolTotal). There is no check that minted shares > 0 and no minimum deposit/bootstrapping share burn. Exploit requires attacker to engineer very small pool total and victim to deposit an amount that rounds to 0 shares (a user mistake: depositing without previewing share output), hence Rare likelihood but High impact (theft of deposits).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager` calculates the value of LP shares using an accumulator that updates after each drawing based on the pool's performance (including ticket revenue/earnings). If the LP pool has very low liquidity (e.g., 1 wei), an attacker can significantly inflate the share price (accumulator) by buying tickets, which adds to `lpEarnings`. 

In `processDrawingSettlement`, the new accumulator is calculated as:
`newAccumulator = (drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal;`

If `currentLP.lpPoolTotal` is 1 wei and `postDrawLpValue` is large (due to ticket purchases), `newAccumulator` becomes extremely large. Subsequent depositors will receive 0 shares due to precision loss in `processDeposit` (specifically during consolidation using the inflated accumulator), while their assets are added to the pool. The attacker can then withdraw their single share to claim the entire pool, stealing the victim's deposit.

## Impact
Theft of funds from liquidity providers who deposit into the pool after it has been manipulated. An attacker can drain 100% of subsequent deposits.

## Command to Run Test


## Proof of Concept
1. Attacker ensures the LP pool is empty or starts fresh. They deposit 1 wei of USDC via `lpDeposit`. `lpPoolTotal` becomes 1 wei after the next settlement.
2. In the next drawing, the attacker buys tickets worth 1,000 USDC. `lpEarnings` increases by ~1,000 USDC.
3. The drawing settles. The `postDrawLpValue` is ~1,000 USDC + 1 wei. The `newAccumulator` is calculated as `1e18 * (1000e6) / 1` = `1e27` (approx).
4. Victim calls `lpDeposit` with 500 USDC (500e6). This is stored as `pendingDeposits`.
5. In the subsequent drawing (or upon user interaction triggering consolidation), the victim's deposit is consolidated. `shares = 500e6 * 1e18 / 1e27 = 0`.
6. The victim receives 0 shares. The pool now holds the victim's 500 USDC.
7. The attacker withdraws their 1 share, which represents 100% of the pool, receiving their initial deposit + costs + the victim's 500 USDC.

## Proof of Code
function testLPInflationAttack() public {
        // Setup: Initialize jackpot
        vm.startPrank(owner);
        jackpot.initialize(address(usdc), jackpotLPManager, jackpotNFT, entropy, payoutCalculator);
        jackpot.initializeLPDeposits(1_000_000e6);
        usdc.mint(owner, 1000e6);
        usdc.approve(address(jackpot), 1000e6);
        jackpotLPManager.processDeposit(0, owner, 1000e6); // Bootstrap
        jackpot.initializeJackpot(block.timestamp + 100);
        vm.stopPrank();

        // Simulate LPs leaving, pool empties (or attacker starts fresh if allowed)
        // For PoC, we assume attacker is the sole LP with 1 wei after a reset or fresh start
        // Direct manipulation of state for brevity to simulate 1 wei pool
        vm.store(address(jackpotLPManager), bytes32(uint256(0)), bytes32(uint256(1))); // lpPoolTotal = 1

        address attacker = address(0x1);
        address victim = address(0x2);
        usdc.mint(attacker, 2000e6);
        usdc.mint(victim, 500e6);

        // Attacker deposits 1 wei (already simulated by state set) and buys tickets
        vm.startPrank(attacker);
        usdc.approve(address(jackpot), 2000e6);
        IJackpot.Ticket[] memory t = new IJackpot.Ticket[](1);
        t[0] = IJackpot.Ticket({normals: new uint8[](5), bonusball: 1});
        // fix normals
        for(uint8 i=0;i<5;i++) t[0].normals[i] = i+1;
        
        // Buy tickets to inflate earnings. Cost 1000e6
        // Mock entropy/settlement needed to update accumulator
        // ... (This requires mocking the full lifecycle, simplified description in PoC text)
    }

## Suggested Mitigation
Enforce a minimum liquidity amount (e.g., burn the first 1000 shares) in `JackpotLPManager` or ensure `lpPoolTotal` never drops below a safe threshold (e.g. 1e6 wei) by requiring a minimum deposit.


## [H-15]. LP Pool Inflation Attack allows stealing new deposits by manipulating accumulator

## id: RZiVY92MsPXOY-l-UgEJ0

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidUserErrorOrMistake
### Finding Status Justification: This is a variant of the gEPoP issue: accumulator can become extremely large when currentLP.lpPoolTotal is tiny relative to postDrawLpValue (newAccumulator = prevAcc * postDrawLpValue / lpPoolTotal). New deposits are converted to shares later via shares = amount*1e18 / drawingAccumulator[depositDrawingId]; if that rounds to 0, the depositor loses funds and existing shareholders capture them. No minimum-share minting requirement exists. It is exploitable if an attacker can engineer very small lpPoolTotal and lure/induce victims to deposit amounts that mint 0 shares; victim depositing without checking share output is a user mistake component.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager` contract calculates LP shares based on an accumulator that tracks the pool's value per share. The accumulator is updated in `processDrawingSettlement` using the formula: `newAccumulator = (drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal`.

An attacker (acting as an LP) can exploit this by withdrawing nearly all liquidity from the pool, leaving just 1 wei (`lpPoolTotal = 1`). If the attacker can then cause `postDrawLpValue` to increase significantly (e.g., by ensuring `lpEarnings` are added via ticket purchases or claiming winnings from a prior round), the `newAccumulator` will inflate massively (e.g., by a factor of 1,000,000+).

When a victim subsequently calls `lpDeposit`, their shares are calculated as `amount * 1e18 / newAccumulator`. Due to the inflated accumulator and integer division, the result rounds down to 0 for even large deposits. The victim's funds are added to the pool, but they receive 0 shares, effectively donating their deposit to the existing shareholder (the attacker).

## Impact
Theft of assets. An attacker can steal 100% of the funds deposited by new LPs following the attack setup.

## Command to Run Test


## Proof of Concept
1. Attacker is the sole LP in Drawing N. They `initiateWithdraw` such that `lpPoolTotal` for Drawing N will be 1 wei.
2. Attacker ensures `lpEarnings` for Drawing N are high (e.g., 100 USDC) by claiming winnings from a previous round (referral fees return to LP pool if no scheme).
3. Drawing N settles. `postDrawLpValue` = 1 wei + 100 USDC.
4. `newAccumulator` = `OldAcc * 100e6e18 / 1`. The accumulator inflates by 10^24.
5. Drawing N+1 starts with this huge accumulator.
6. Victim deposits 1,000 USDC. `shares = 1000e6 * 1e18 / 1e24` = 0.
7. Victim transfers 1,000 USDC but gets 0 shares. Attacker (holding the 1 wei share) owns the entire pool.

## Proof of Code
function testInflationAttack() public {
    // Conceptual Test
    uint256 poolTotal = 1; // 1 wei
    uint256 earnings = 100e6; // 100 USDC
    uint256 oldAcc = 1e18;
    
    uint256 newAcc = (oldAcc * (poolTotal + earnings)) / poolTotal;
    
    uint256 deposit = 1000e6; // 1000 USDC
    uint256 shares = (deposit * 1e18) / newAcc;
    
    // Shares should be 0, proving loss of funds
    assertEq(shares, 0);
}

## Suggested Mitigation
Enforce a minimum LP pool size (e.g., 1000 wei) before allowing drawings to proceed, or burn the first portion of LP shares (similar to Uniswap V2) to make the inflation attack prohibitively expensive.





Finding Status: LowSeverityDueToLowImpact
## [M-16]. Incorrect Emergency Refunds due to Mutable Referral Fee

## id: qef3_sFMOPVtHOgGv2Mma

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: LowSeverityDueToLowImpact
### Finding Status Justification: Refund miscalculation can occur even if governance is acting normally (changing referralFee for future drawings) and then an emergency occurs; the bug is the lack of per-ticket/per-drawing referralFee snapshot for refunds. Impact is largely confined to emergency-mode refunds.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `emergencyRefundTickets` function calculates refunds based on the *current* `referralFee` global parameter (`ticketPrice * (1 - referralFee)`), not the fee that was actually paid when the ticket was purchased. If the admin changes the `referralFee` between ticket purchase and emergency refund, the refund amount will be incorrect. If the fee is increased, users receive less than they are owed. If decreased, the protocol overpays, causing a loss to LPs.

## Impact
Financial loss to users or the protocol (LPs) during emergency refunds due to parameter mismatch.

## Command to Run Test


## Proof of Concept
1. `referralFee` is 10%. User buys ticket for 100 USDC. Protocol keeps 90 USDC.
2. Admin updates `referralFee` to 50%.
3. Emergency mode enabled.
4. User calls `emergencyRefundTickets`. Refund = 100 * (1 - 0.50) = 50 USDC.
5. User loses 40 USDC unfairly.

## Proof of Code
function testUnfairRefund() public {
  jackpot.setReferralFee(0.1e18);
  jackpot.buyTickets(...);
  jackpot.setReferralFee(0.5e18);
  jackpot.enableEmergencyMode();
  jackpot.emergencyRefundTickets(...);
  // Assert user received significantly less than paid - fee
}

## Suggested Mitigation
Store the `referralFee` used at purchase time in the ticket metadata, or use the `referralFee` from the `DrawingState` if it were stored there (currently it is not stored in DrawingState for refunds, only referralWinShare is).





Finding Status: InvalidGovernanceRisk
## [M-17]. Incorrect tier classification due to bit packing overflow if bonusballMin or normalBallMax are large

## id: VGQ8c3DUQHdIWnDCX3B-x

## Derived From Pattern/Invariant
IntegerMath

## Exploit Type
IntegerMath

## Location
Jackpot.setBonusballMin

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The core issue is lack of enforcement that `normalMax + bonusballMax <= 255`. In `TicketComboTracker.insert` and `countTierMatchesWithBonusball`, the bonus bit uses `1 << (_bonusball + _tracker.normalMax)` where both operands are `uint8`. If the sum exceeds 255, the `uint8` addition overflows and reverts (so the failure mode is a DoS during ticket insertion or settlement), not silent misclassification. `Jackpot.setBonusballMin` has no constraint, and `_setNewDrawingState` can compute a `bonusballMax` that exceeds `255 - normalBallMax` when prize pools grow. Because emergency mode is not guaranteed to prevent late callbacks, this can materially threaten settlement liveness; it is largely a governance/configuration safety issue but can also be triggered by large, permissionless ticket volume.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `TicketComboTracker` library packs a ticket's normal balls and bonus ball into a single `uint256` bit vector. The normal balls occupy bits `1` to `normalMax`, and the bonus ball occupies bit `normalMax + bonusball`. The `setBonusballMin` and `setNormalBallMax` functions in `Jackpot` allow the owner to update these parameters without validating that `normalBallMax + bonusballMax <= 255`. If `normalBallMax + bonusballMax > 255`, the bit shift operation `1 << (bonusball + normalMax)` in `TicketComboTracker.insert` (and implicit in `countTierMatchesWithBonusball`) will overflow/truncate to 0 for high bonus ball values. This causes the packed representation of the ticket to lose the bonus ball information (effectively treated as 0). 

If the winning ticket also has a high bonus ball value causing overflow, its packed representation will also have a 0 bonus bit. When `Jackpot` calculates winnings, `_calculateTicketTierId` extracts the bonus ball via shifting; both the user's ticket and the winning ticket will evaluate to having no bonus ball bit (0), resulting in a false positive match (0 == 0). This causes tickets to be classified into higher payout tiers (bonus ball match) incorrectly, potentially leading to protocol insolvency.

## Impact
Inflated payouts due to incorrect bonus ball matching, leading to potential depletion of the prize pool and LP funds.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 35 and `bonusballMin` to 225 (Total 260 > 255).
2. `_setNewDrawingState` updates `bonusballMax` to at least 225.
3. User buys a ticket with bonus ball 250. `TicketComboTracker` packs this; the bonus bit `1 << (250+35)` overflows to 0.
4. Winning numbers are drawn with bonus ball 250. Winning ticket packed has bonus bit 0.
5. `claimWinnings` calls `_calculateTicketTierId`. It extracts bonus parts from packed tickets.
6. Both are 0. Logic considers this a Match.
7. User gets paid for a bonus match even if they didn't match (though in this specific case they did match the number, the overflow allows ANY overflowing bonus ball to match ANY OTHER overflowing bonus ball). Example: Winning ball 240 (overflows to 0), User ball 250 (overflows to 0) -> Match.

## Proof of Code
function testBitPackingOverflow() public {
    // Assume admin set params such that normalMax + bonusMax > 255
    uint8 normalMax = 35;
    uint8 bonusVal = 250; // 250 + 35 = 285 > 255
    
    // Simulate packing
    uint256 packed = 0;
    // Set normals (dummy)
    packed |= (1 << 1);
    // Set bonus
    // In Solidity: 1 << 285 is 0
    packed |= (1 << (bonusVal + normalMax));
    
    // Verify bonus bit is missing
    assertEq(packed >> (normalMax + 1), 0);
}

## Suggested Mitigation
In `Jackpot.setBonusballMin` and `Jackpot.setNormalBallMax`, add a requirement that `_normalBallMax + _bonusballMin <= 255` (and similarly for the dynamic `bonusballMax` calculation, though `lpPoolCap` constraints likely limit the dynamic growth, the explicit setter must be guarded).


## [M-18]. Changing Entropy Provider Locks Active Drawing

## id: OXbYdF0kxxlM6K0uEuId-

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.setEntropy

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Bricking a live drawing (until owner intervention/emergency) is not low impact; it halts the core protocol loop and can strand current-drawing participants.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `scaledEntropyCallback` function enforces that `msg.sender` equals the current `entropy` address. If the admin calls `setEntropy` to change the provider after `runJackpot` has been called (locking the drawing) but before the callback arrives, the callback from the original provider will revert due to the address mismatch. This leaves the drawing permanently locked in the 'JackpotLocked' state, requiring emergency intervention.

## Impact
Denial of Service for the current drawing; disrupts protocol flow and necessitates emergency unlock/refund.

## Command to Run Test


## Proof of Concept
1. Keeper calls `runJackpot`. Drawing locks. Request sent to Provider A.
2. Admin calls `setEntropy(Provider B)`.
3. Provider A calls `scaledEntropyCallback`.
4. Revert: `msg.sender (Provider A) != entropy (Provider B)`.
5. Drawing remains locked.

## Proof of Code
function testEntropySwapDoS() public {
  jackpot.runJackpot{value: fee}();
  jackpot.setEntropy(newProvider);
  // Mock callback from old provider
  vm.prank(oldProvider);
  vm.expectRevert();
  jackpot.scaledEntropyCallback(...);
}

## Suggested Mitigation
Prevent changing the entropy provider while `drawingState[currentDrawingId].jackpotLock` is true, or store the request's provider in mapping to validate against the specific request rather than global state.


## [M-19]. Administrator can brick active drawings or deny payouts by changing global contracts mid-flow

## id: 8uDMqvQ7E7ChOQiE_tC-_

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.setEntropy

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Replacing entropy/payoutCalculator mid-flow can halt settlement or break payouts for an active drawing; that is not low impact even if privileged.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `Jackpot` contract allows the owner to update critical external contracts (`entropy`, `payoutCalculator`) via `setEntropy` and `setPayoutCalculator` without checking if a drawing is currently active (locked). 

1. **DoS Vector**: If `setEntropy` is called after `runJackpot` (which locks the drawing and requests entropy) but before the callback, the callback from the old provider will revert because `scaledEntropyCallback` enforces `modifier onlyEntropy` (checking `msg.sender == address(entropy)`). This leaves the drawing permanently locked.
2. **Loss of Assets Vector**: If `setPayoutCalculator` is called mid-draw, the `scaledEntropyCallback` will use the new calculator for settlement. The `GuaranteedMinimumPayoutCalculator` relies on `drawingTierInfo` snapshotted by drawing ID. A newly deployed calculator will have empty data for the current `drawingId`, resulting in zero payouts for all winners. User winnings stay in the LP pool.

## Impact
Can result in a permanent Denial of Service (requiring emergency mode and breaking the drawing) or loss of user winnings (funds effectively transferred to LPs).

## Command to Run Test


## Proof of Concept
1. `Jackpot.runJackpot()` is called, setting `jackpotLock = true` and sending a request to `OldEntropy`.
2. Admin calls `Jackpot.setEntropy(NewEntropy)`.
3. `OldEntropy` attempts to call `scaledEntropyCallback`.
4. The `onlyEntropy` modifier checks `msg.sender` (OldEntropy) against `entropy` state var (NewEntropy).
5. Check fails, transaction reverts.
6. The drawing cannot be settled or unlocked normally.

## Proof of Code
function testSetEntropyBricksDrawing() public {
    // Assume setup complete and jackpot ready
    vm.prank(keeper);
    jackpot.runJackpot{value: fee}();
    
    // Admin changes entropy provider mid-flight
    vm.prank(owner);
    jackpot.setEntropy(IScaledEntropyProvider(address(newProvider)));
    
    // Old provider tries to callback
    vm.prank(address(oldProvider));
    vm.expectRevert(JackpotErrors.UnauthorizedEntropyCaller.selector);
    jackpot.scaledEntropyCallback(requestId, randomNumbers, "");
    
    // Drawing remains locked
    assertEq(jackpot.getDrawingState(jackpot.currentDrawingId()).jackpotLock, true);
}

## Suggested Mitigation
Add a check in `setEntropy` and `setPayoutCalculator` (and other critical parameter setters) to ensure `drawingState[currentDrawingId].jackpotLock` is false, or disallow changes while a request is pending.


## [H-20]. Cross-provider request collision in ScaledEntropyProvider

## id: pC6d2-z_D7hl2dySPs3vs

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Impact is not low: can corrupt which request is fulfilled or cause settlement failure during provider rotation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
`ScaledEntropyProvider` stores pending entropy requests in a `pending` mapping keyed only by the `sequence` number returned by the provider. Pyth Entropy sequence numbers are scoped to the provider address, not globally unique. 

If the protocol owner changes the entropy provider via `setEntropyProvider`, the new provider may generate sequence numbers that collide with pending requests from the previous provider. `_storePendingRequest` will overwrite the old request's context. Furthermore, if a callback arrives from the old provider with a sequence number matching a pending request for the new provider, it will be accepted (as `entropyCallback` implementation does not validate the `provider` argument against a stored provider), leading to the usage of incorrect/untrusted random values.

## Impact
Corruption of randomness data, potential DoS by overwriting requests, or acceptance of entropy from a deprecated/untrusted provider.

## Command to Run Test


## Proof of Concept
1. Admin calls `setEntropyProvider(ProviderA)`.
2. `runJackpot` calls `request...`. ProviderA returns `sequence = 1`. `pending[1]` is set.
3. Admin calls `setEntropyProvider(ProviderB)` before ProviderA callbacks.
4. `runJackpot` (next drawing) calls `request...`. ProviderB returns `sequence = 1` (sequences start from 1). `pending[1]` is overwritten with new context.
5. ProviderA callback arrives for `sequence = 1`. `entropyCallback` finds `pending[1]`, validates it (checks msg.sender is entropy contract, which is constant), and fulfills the request using ProviderA's number but ProviderB's context.

## Proof of Code
function testProviderKeyCollision() public {
    // Mock Entropy contract returning duplicate sequences for diff providers
    // Call request with Provider A -> Seq 1
    // Switch provider
    // Call request with Provider B -> Seq 1
    // Assert pending[1] is overwritten
}

## Suggested Mitigation
Include the `provider` address in the `PendingRequest` struct and check that `msg.sender` (in `_entropyCallback`) or the `provider` argument matches the stored provider. Alternatively, scope the `pending` mapping as `mapping(address => mapping(uint64 => PendingRequest))`.


## [H-21]. Randomness corruption due to sequence number collision upon entropy provider rotation

## id: Z3baZ6elYmiJyQDIl9tEL

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.entropyCallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Impact is not low because it can settle a drawing using randomness from the wrong provider/context or break settlement, which is material to fairness and liveness.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `ScaledEntropyProvider` contract tracks pending entropy requests using a `mapping(uint64 => PendingRequest) pending`, keyed solely by the `sequence` number returned by the Pyth Entropy provider. The contract also allows the owner to rotate the entropy provider address via `setEntropyProvider`.

If the provider is changed, the new provider (e.g., a different Pyth deployment or mock) may generate sequence numbers starting from 1 or overlapping with the old provider's sequence numbers. If a request is made to the new provider while a request to the old provider is still pending with the same sequence number, the new request data will overwrite the old one in the `pending` mapping.

Critically, when the callback arrives (from either provider), `entropyCallback` retrieves the request data using only the `sequence` number. It does not verify that the callback sender matches the provider associated with the request (as the provider isn't stored in `PendingRequest`). This allows the randomness from the old provider to be applied to the new request (or vice versa), violating the integrity of the RNG.

## Impact
Compromised randomness integrity; randomness from one source can be substituted for another, potentially leading to predictable or manipulatable jackpot outcomes.

## Command to Run Test


## Proof of Concept
1. `ScaledEntropyProvider` is using Provider A.
2. User calls `runJackpot`, triggering `requestV2` to Provider A. Provider A returns `sequence = 100`. `pending[100]` is stored.
3. Admin calls `setEntropyProvider(Provider B)`.
4. User (or attacker) calls `runJackpot` again. `requestV2` to Provider B returns `sequence = 100` (collision).
5. `pending[100]` is overwritten with details for the new request.
6. Provider A's callback arrives first with `sequence = 100` and random value `R_A`.
7. `entropyCallback` executes, looking up `pending[100]`. It finds the params for the *second* request (intended for B).
8. The contract accepts `R_A` as the randomness for the second request, finalizing the jackpot using the wrong entropy source.

## Proof of Code
function testKeyCollision() public {
    // Simulate collision logic
    // Assume provider A and B both return sequence 1
    vm.prank(owner);
    entropy.setEntropyProvider(address(providerA));
    
    jackpot.runJackpot{value: fee}(); // Gets seq 1 from A
    
    vm.prank(owner);
    entropy.setEntropyProvider(address(providerB));
    
    jackpot.unlockJackpot(); // Force unlock for test
    jackpot.runJackpot{value: fee}(); // Gets seq 1 from B, overwrites pending[1]
    
    // Callback from A (seq 1) arrives
    vm.prank(address(entropyContract));
    // Fulfills using B's request params but A's randomness
    scaledEntropyProvider.entropyCallback(1, address(providerA), bytes32(uint256(123))); 
}

## Suggested Mitigation
Include the provider address in the `pending` mapping key (e.g., `keccak256(abi.encode(provider, sequence))`) or store the expected provider in the `PendingRequest` struct and validate `msg.sender` (or the provider arg) against it in `entropyCallback`.


## [H-22]. Changing Payout Calculator Bricks Historical Winnings Claims

## id: CXMDrVe1AbUkW3J1qj8wc

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.setPayoutCalculator

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: If executed, it can effectively deny claims for past drawings (economic loss to users), which is not low impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `Jackpot` contract allows the owner to change the `payoutCalculator` address via `setPayoutCalculator`. The `claimWinnings` function calls `payoutCalculator.getTierPayout(drawingId, ...)` to determine winnings. Since tier payout data is stored in the calculator contract's storage (`tierPayouts`), replacing the calculator contract with a new one (which has empty storage for past drawings) causes all unclaimed winnings from previous drawings to resolve to zero. This effectively bricks the claiming process for all historical winners.

## Impact
Users permanently lose access to unclaimed winnings from past drawings if the calculator is updated.

## Command to Run Test


## Proof of Concept
1. Users buy tickets for Drawing 1. Drawing 1 settles using `CalculatorA`, which stores payout data.
2. Users wait to claim.
3. Admin calls `setPayoutCalculator(CalculatorB)`.
4. User calls `claimWinnings` for Drawing 1.
5. `Jackpot` calls `CalculatorB.getTierPayout(1, ...)`.
6. `CalculatorB` has no data for Drawing 1; returns 0.
7. User receives 0 USDC.

## Proof of Code
function testCalculatorSwapBricksClaims() public {
  // Setup drawing and win
  // ...
  // Admin swaps calculator
  vm.prank(owner);
  jackpot.setPayoutCalculator(newCalculator);
  // User claims
  vm.prank(user);
  jackpot.claimWinnings(ticketIds);
  // Assert winnings are 0
}

## Suggested Mitigation
Make `payoutCalculator` immutable, or require data migration, or verify the new calculator has the necessary historical data before switching.


## [H-23]. Protocol key collision in ScaledEntropyProvider allows request hijacking or state corruption when provider is changed

## id: mK4aQQI-tfSUxKhiTvfLr

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.setEntropyProvider

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: If it happens, impact is not low: it can corrupt/DoS randomness delivery for drawings (wrong context consumed or requests deleted), potentially requiring emergency procedures.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `ScaledEntropyProvider` contract uses `uint64 sequence` (returned by Pyth's `IEntropyV2.requestV2`) as the key for the `pending` mapping. The Pyth Entropy contract maintains sequence numbers *per provider* (stored in `ProviderInfo.sequenceNumber`). If the contract owner calls `setEntropyProvider` to switch to a new entropy provider, the new provider's sequence numbers will overlap with those of the previous provider (e.g., both starting from 1 or overlapping at the current count). 

If a pending request exists for the old provider (e.g. sequence 100), and a new request is made to the new provider which also assigns sequence 100, the `pending[100]` entry will be overwritten. This leads to two critical failure modes:
1. If the old provider's callback arrives first, it will be processed using the request data of the *new* request (hijacking the context/callback selector), potentially settling a drawing with randomness from the wrong provider.
2. If the new request overwrites the old one, the old request's callback might fail or process incorrectly, and the new request's callback might fail if the entry was deleted by the old provider's callback (race condition).

## Impact
Data corruption, processing of randomness from unintended providers, or denial of service for pending drawings during provider migration.

## Command to Run Test


## Proof of Concept
1. `ScaledEntropyProvider` is configured with Provider A.
2. User calls `runJackpot()`. `requestAndCallbackScaledRandomness` calls Provider A, returning `sequence = 50`. `pending[50]` is stored.
3. Provider A is slow. Admin calls `setEntropyProvider(Provider B)` to switch.
4. Admin or user calls `runJackpot()` (assuming the previous one is stuck/unlocked via `unlockJackpot`). Provider B is called, returns `sequence = 50` (since sequences are isolated per provider).
5. `pending[50]` is overwritten with Provider B's request details.
6. Provider A sends callback for sequence 50. `entropyCallback` executes using the data in `pending[50]` (Provider B's params).
7. The jackpot settles using Provider A's randomness but Provider B's context context.

## Proof of Code
function testKeyCollision() public {
    // Mock Entropy contract returning same sequence for different providers
    mockEntropy.setNextSequence(100);
    scaledEntropy.requestAndCallbackScaledRandomness(gas, reqs, callback, ""); // Provider A, seq 100
    
    scaledEntropy.setEntropyProvider(address(providerB));
    mockEntropy.setNextSequence(100);
    scaledEntropy.requestAndCallbackScaledRandomness(gas, reqs, callback, ""); // Provider B, seq 100
    
    // pending[100] is now overwritten. Previous request is lost/corrupted.
}

## Suggested Mitigation
Modify the `pending` mapping key to include the provider address (e.g., `keccak256(abi.encode(provider, sequence))`) or verify the `provider` argument in `entropyCallback` matches the stored provider for that request.


## [M-24]. Protocol fee manipulation mid-drawing affects settlement of LP earnings

## id: 4QAJUcxZKTKDHQ1VpS-b6

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.setProtocolFee

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Retroactively changing protocol fee for an already-running drawing can materially change LP economics (up to 25% of excess) and violates the stated invariant that admin changes apply only to future drawings.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `protocolFee` parameter is read directly from global storage during the drawing settlement (`_transferProtocolFee` inside `scaledEntropyCallback`) rather than being snapshotted at the start of the drawing. This allows the admin to change the fee rate (up to 25%) after LPs have deposited and users have bought tickets, but before settlement.

This violates the `GlobalParamMidFlowManipulation` pattern: LPs participate under one fee regime, but settlement occurs under another. An admin can front-run the settlement to maximize fee extraction or set it to zero, manipulating the net yield for LPs.

## Impact
Manipulation of LP yield and protocol revenue; violation of economic expectations for locked liquidity.

## Command to Run Test


## Proof of Concept
1. LPs deposit assuming 1% protocol fee.
2. Tickets are bought, generating `lpEarnings`.
3. Drawing closes.
4. Admin calls `setProtocolFee(25%)` (max).
5. Settlement executes `_transferProtocolFee` using 25%.
6. LPs receive significantly less earnings than expected.

## Proof of Code
function testFeeManip() public {
    // 1. Set low fee, buy tickets
    // 2. Change to high fee
    vm.prank(owner);
    jackpot.setProtocolFee(25e16);
    // 3. Settle and check LP return
}

## Suggested Mitigation
Snapshot `protocolFee` and `protocolFeeThreshold` into `DrawingState` when a new drawing is initialized, similar to `ticketPrice` and `referralWinShare`.


## [M-25]. LPs Lose Money on Duplicate Tickets if Referral Fee Exceeds LP Edge

## id: o0q7UPASRqCdMQxSoJrTo

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: On duplicate tickets, prizePool is increased by ticketPrice - edgePerTicket (i.e., ticketPrice*(1-lpEdgeTarget)) in Jackpot._validateAndStoreTickets(). LP earnings are increased by ticketsValue - referralFeeTotal, where referralFeeTotal = ticketsValue*referralFee if referrers are provided. If referralFee > lpEdgeTarget, then (1-referralFee) < (1-lpEdgeTarget), so each duplicate can add more liability (prizePool) than it adds assets (lpEarnings) when a referral scheme is used. No invariant enforces referralFee <= lpEdgeTarget. This is enabled/disabled by owner-configured parameters, so it is governance risk but can lead to LP losses and potential settlement insolvency/DoS in extreme cases.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
When a duplicate ticket is purchased, `Jackpot` increases the `prizePool` by `ticketPrice * (1 - edgePerTicket)` to maintain the LP edge ratio. However, the `lpEarnings` (which fund the pool) only receive `ticketPrice * (1 - referralFee)`. If `referralFee` is set higher than `lpEdgeTarget`, the liability added to the prize pool exceeds the assets collected, resulting in a guaranteed net loss for LPs on every duplicate ticket sold. There is no validation preventing `referralFee > lpEdgeTarget`.

## Impact
Liquidity Providers suffer guaranteed losses on duplicate tickets, violating the protocol's solvency model and edge guarantee.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` = 20% and `lpEdgeTarget` = 10%.
2. User buys a duplicate ticket for 100 USDC.
3. `lpEarnings` increases by 80 USDC (100 - 20).
4. `prizePool` increases by 90 USDC (100 - 10).
5. Net LP position change: +80 (asset) - 90 (liability) = -10 USDC.

## Proof of Code
function testLPLossOnDup() public {
  jackpot.setReferralFee(0.2e18);
  jackpot.setLpEdgeTarget(0.1e18);
  // Buy original then duplicate
  // Check LP value impact
}

## Suggested Mitigation
Enforce invariant `referralFee <= lpEdgeTarget` in the setter functions.


## [H-26]. Permanent DoS of Jackpot Settlement via Referral Fee Insolvency

## id: 4b4kHWqSyWqcsQg1rYOF2

## Derived From Pattern/Invariant
DoS due to resource exhaustion or logic error leading to revert

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Settlement in JackpotLPManager.processDrawingSettlement will revert on underflow if userWinnings + protocolFee > lpPoolTotal + lpEarnings. The protocol’s intended “duplicate ticket preserves LP edge” logic assumes lpEarnings grows at least as fast as prizePool liabilities created by duplicates. However, if referralFee is set above lpEdgeTarget and users include referrers (making referralFeeTotal apply), lpEarnings per ticket becomes ticketPrice*(1-referralFee) while duplicate prizePool increments are ticketPrice*(1-lpEdgeTarget). This mismatch can make prizePool-based payouts exceed assets, causing settlement revert/DoS. Requires governance misconfiguration, hence governance risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager.processDrawingSettlement` function calculates the new LP pool value as `postDrawLpValue = lpPoolTotal + lpEarnings - userWinnings - protocolFeeAmount`. It reverts if `postDrawLpValue` underflows (i.e., if liabilities exceed assets). 

There is a critical accounting imbalance when `referralFee` is configured higher than `lpEdgeTarget`. When a duplicate ticket is purchased:
1. The user pays `ticketPrice`.
2. `lpEarnings` increases by `ticketPrice * (1 - referralFee)`.
3. `prizePool` (potential `userWinnings`) increases by `ticketPrice * (1 - lpEdgeTarget)`.

If `referralFee > lpEdgeTarget`, the `prizePool` grows faster than `lpEarnings`. An attacker can purchase a large number of duplicate tickets for a winning combination (or ensure a win via covering combinations), creating a state where `userWinnings > lpPoolTotal + lpEarnings`. This causes the settlement transaction to revert due to underflow. Since settlement is required to unlock the jackpot and start the next drawing, the protocol becomes permanently stuck in the locked state until Emergency Mode is activated.

## Impact
The jackpot settlement transaction permanently reverts, locking all funds for the current drawing and preventing the start of any new drawings (Protocol DoS).

## Command to Run Test


## Proof of Concept
1. Owner sets `referralFee` to 20% and `lpEdgeTarget` to 10% (valid configuration).
2. Attacker waits for a drawing with some LP liquidity.
3. Attacker buys a ticket.
4. Attacker buys many duplicate tickets of the same combination. For each dup, `prizePool` adds 0.9 * Price, `lpEarnings` adds 0.8 * Price. Net accounting change is -0.1 * Price.
5. If Attacker buys enough duplicates, the 'net accounting change' exceeds the `lpPoolTotal` buffer.
6. When the drawing closes and `runJackpot` is called, the `scaledEntropyCallback` invokes `processDrawingSettlement`.
7. If the ticket wins (or if min payouts force winnings > earnings + pool), the calculation `lpPoolTotal + earnings - winnings` underflows.
8. The transaction reverts, and the jackpot remains locked.

## Proof of Code
function testDoSSettlement() public {
    // Config: 20% referral, 10% edge
    vm.startPrank(owner);
    jackpot.setReferralFee(0.2e18);
    jackpot.setLpEdgeTarget(0.1e18);
    jackpot.setTicketPrice(10e6);
    vm.stopPrank();

    // Initial state: LP Pool has 1000 USDC
    // Prize pool approx 900 USDC
    
    // User buys 1 winning ticket + 2000 duplicates
    // Each dup: earnings += 8, prizePool += 9. Net -1.
    // 2000 dups -> Net -2000. 
    // 1000 (Initial) + 8*2000 - (900 + 9*2000) = 1000 + 16000 - 18900 = -1900 -> Underflow

    // (Mock setup code for purchasing and triggering settlement omitted for brevity)
    // Assert revert in processDrawingSettlement
}

## Suggested Mitigation
Enforce invariant `referralFee <= lpEdgeTarget` in the setter functions (`setReferralFee`, `setLpEdgeTarget`) and constructor. Alternatively, cap `userWinnings` or handle the insolvency gracefully in `processDrawingSettlement` by triggering a deficit handling mechanism instead of reverting.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [H-27]. Permanent DoS of LP System if Accumulator drops to Zero

## id: 5EuHz5lUcfvc-D6KsNAZe

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: JackpotLPManager.processDrawingSettlement computes newAccumulator = (drawingAccumulator[d-1] * postDrawLpValue) / lpPoolTotal when lpPoolTotal>0. If postDrawLpValue becomes exactly 0 (possible under certain parameterizations, e.g., reserveRatio=0 and ticket revenue routed away from lpEarnings), newAccumulator becomes 0. Then _consolidateDeposits divides by drawingAccumulator[lastDeposit.drawingId], causing division-by-zero reverts and breaking LP operations. The code explicitly assumes “Accumulators can never be zero”, but that assumption is not enforced. This scenario is strongly influenced by governance parameters (reserveRatio/referralFee) and edge cases, so likelihood is Rare and governance-related.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager`, the share price is tracked via `drawingAccumulator`. If a drawing results in a total loss for LPs (e.g., `reserveRatio` is 0 and a large win pays out the entire pool), `postDrawLpValue` becomes 0. Consequently, `newAccumulator` is calculated as 0 via `(drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal`.

Once `drawingAccumulator` is 0 for a drawing, any future interaction by LPs who deposited during or before that drawing will revert. Specifically, `processDeposit`, `initiateWithdraw`, and even `emergencyWithdrawLP` all call `_consolidateDeposits`. This function calculates `consolidatedShares += (deposit.amount * 1e18) / accumulator`. If the accumulator is 0, this causes a Division by Zero revert.

Furthermore, if the pool is wiped (0 value), `initializeDrawingLP` sets the next drawing's `lpPoolTotal` to 0 (plus any pending deposits). If there are pending deposits, the next drawing starts, but since the previous accumulator was 0, the new accumulator remains 0. LPs can never recover, and the LP system is permanently DOS'd for affected users.

## Impact
Permanent locking of LP funds and inability to deposit new funds. The LP system becomes unusable.

## Command to Run Test


## Proof of Concept
1. Owner sets `reserveRatio` to 0.
2. LP deposits 1,000 USDC. `lpPoolTotal` = 1,000.
3. User buys ticket winning the jackpot (or large enough payouts to drain pool). 
4. `runJackpot` -> `scaledEntropyCallback` calls `processDrawingSettlement`.
5. `postDrawLpValue` = 1000 (pool) + 1 (earnings) - 1001 (winnings) = 0.
6. `newAccumulator` becomes 0.
7. Next drawing starts. LP tries to deposit again or withdraw.
8. `_consolidateDeposits` is called for the LP's state.
9. It attempts division by `drawingAccumulator` (which is 0). Reverts.

## Proof of Code
function testAccumulatorDoS() public {
    // Setup LP with deposit
    vm.prank(lp);
    lpManager.processDeposit(1, lp, 1000e6);
    
    // Simulate settlement wiping the pool
    // Earnings 0, Winnings 1000, Fee 0
    (uint256 newLP, uint256 newAcc) = lpManager.processDrawingSettlement(1, 0, 1000e6, 0);
    
    assertEq(newAcc, 0, "Accumulator should be 0");
    
    // Try to deposit in next round
    vm.prank(lp);
    vm.expectRevert(); // Panic: Division by zero
    lpManager.processDeposit(2, lp, 100e6);
}

## Suggested Mitigation
Ensure `newAccumulator` has a minimum value of 1, or handle the 0 accumulator case in `_consolidateDeposits` and `_consolidateWithdrawals` by clearing shares/value instead of reverting.



