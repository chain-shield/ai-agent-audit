# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[M-1]. Protocol DoS due to excessive gas limit scaling
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[M-2]. `JackpotBridgeManager` transaction failure or fund loss due to ticket price mismatch
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: Valid
Privilege: RequiresAdminRole


[H-3]. Permanent Denial of Service (Jackpot Bricking) due to uint8 overflow in TicketComboTracker
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-4]. Protocol insolvency via late entropy callback after emergency refunds
**Derived From** : EmergencyModeStateStuck
Finding Status: Valid
Privilege: RequiresAdminRole


[M-5]. DoS in drawing settlement due to excessive storage reads
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[M-6]. DoS in entropy callback due to O(N) loop over `bonusballMax`
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-7]. First depositor inflation attack allows stealing the entire LP pool
**Derived From** : ERC4626SharePriceMismatch
Finding Status: Valid
Privilege: Permissionless


[M-8]. DoS in `scaledEntropyCallback` due to excessive gas consumption when `bonusballMax` is large
**Derived From** : UnboundedLoops
Finding Status: Valid
Privilege: Permissionless


[H-9]. LP Vault Inflation Attack via Donation/Rounding
**Derived From** : ERC4626 Inflation Attack
Finding Status: Valid
Privilege: Permissionless


[H-10]. DoS in drawing settlement due to unbounded storage reads scaling with prize pool
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-11]. Protocol Insolvency via scaledEntropyCallback execution during Emergency Mode
**Derived From** : EmergencyModeStateStuck
Finding Status: Valid
Privilege: Permissionless


[H-12]. Emergency Refund "Ghost Winners" Cause Permanent LP Fund Loss
**Derived From** : EmergencyModeStateStuck
Finding Status: Valid
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood


[H-13]. Silent truncation in bonusball calculation breaks LP edge guarantee
**Derived From** : IntegerOverflow
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[M-14]. Ticket bitpacking overflow prevents winning bonus tiers
**Derived From** : IntegerOverflow
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[H-15]. Ticket corruption via bit packing overflow allows bonusball bypass
**Derived From** : BitwiseLogicError
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[H-16]. Unbounded Ticket Sales Bypass Pool Cap causing Bonusball Difficulty Overflow
**Derived From** : IntegerOverflow
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[H-17]. Unsafe cast of `bonusballMax` breaks LP edge protection
**Derived From** : Integer Overflow / Unsafe Cast
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[H-18]. Downcasting in dynamic bonusball calculation allows difficulty manipulation and LP drainage
**Derived From** : UnsafeAssembyTypeCasts
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[H-19]. Dynamic bonusballMax calculation causes bit packing failure and broken game logic
**Derived From** : IntegerOverflow
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[H-20]. Ticket storage corruption when `bonusballMax + normalBallMax >= 256`
**Derived From** : BitwiseLogicError
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-21]. Permanent DoS of LP System via Accumulator Zero State
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-22]. LP funds permanently locked if reserve ratio is zero and pool is drained
**Derived From** : ConfigFootgun
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-23]. Protocol DoS in drawing settlement if `referralFee` exceeds `lpEdgeTarget`
**Derived From** : ConfigFootgun
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[H-24]. Permanent LP Lockup if Drawing Accumulator Drops to Zero
**Derived From** : DivideByZeroOrOverFlowInCustomMath
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: InvalidGovernanceRisk


[M-25]. Referral fee arbitrage on duplicate tickets
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless


[H-26]. Protocol insolvency via emergency refund followed by late settlement
**Derived From** : EmergencyModeStateStuck
Finding Status: InvalidGovernanceRisk
Privilege: RequiresRole


[H-27]. Insolvency due to phantom LP earnings after emergency refund
**Derived From** : EmergencyModeStateStuck
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-28]. Protocol insolvency due to accounting mismatch in emergency refunds
**Derived From** : EmergencyModeStateStuck
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless


[M-29]. Emergency refund value discrepancy due to global referral fee modification
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


### Number of Findings
- C: 0
- H: 18
- M: 11
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. Protocol DoS due to excessive gas limit scaling

## id: vAyTdGd9Jo7Ce7DPzZI6Y

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot._calculateEntropyGasLimit

## Finding Status: Valid
### Finding Status Justification: `_calculateEntropyGasLimit` scales with `bonusballMax` and defaults (`entropyVariableGasLimit=250k`) can exceed practical limits as `bonusballMax` grows via normal economic scaling. This is a protocol design/runtime risk, not purely an admin error condition.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract calculates the gas limit for the entropy callback based on `bonusballMax`:

```solidity
function _calculateEntropyGasLimit(uint8 _bonusballMax) internal view returns (uint32) {
    return entropyBaseGasLimit + entropyVariableGasLimit * uint32(_bonusballMax);
}
```

With default `entropyVariableGasLimit` of 250,000, if `bonusballMax` scales up (e.g., to 150) due to a large prize pool, the calculated gas limit (37.5M) exceeds the block gas limit (30M on Ethereum/Base). `runJackpot` calls `entropy.getFee` with this excessive limit, which will likely revert or cause the callback transaction to be un-mineable, bricking the protocol.

## Impact
Protocol enters a state where new drawings cannot be settled or started because the required gas limit exceeds the block limit. Requires admin intervention to lower config values.

## Command to Run Test


## Proof of Concept
1. `bonusballMax` reaches 150.
2. `entropyVariableGasLimit` is 250,000.
3. Calculated limit = 37,500,000.
4. Block limit is 30,000,000.
5. Transaction fails.

## Proof of Code
function testGasLimit() public {
    uint32 base = 0;
    uint32 variable = 250000;
    uint8 bonus = 150;
    uint32 limit = base + variable * bonus;
    assertGt(limit, 30000000);
}

## Suggested Mitigation
Drastically reduce the default `entropyVariableGasLimit` (e.g., to ~70k, which matches estimated usage) or cap the total calculated gas limit to a safe threshold below the block limit.


## [M-2]. `JackpotBridgeManager` transaction failure or fund loss due to ticket price mismatch

## id: AcT-PurMMGDX1M0r9oipW

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
JackpotBridgeManager.buyTickets

## Finding Status: Valid
### Finding Status Justification: Even if the owner updates `ticketPrice` with the intent to affect only future drawings (as per spec), BridgeManager uses the global `jackpot.ticketPrice()` while `Jackpot.buyTickets` charges the per-drawing snapshotted price. This creates user overpayment (stuck funds) or DoS without requiring an admin “mistake”, just a normal parameter update mid-drawing.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
`JackpotBridgeManager.buyTickets` fetches the current global ticket price using `jackpot.ticketPrice()`. However, `Jackpot.buyTickets` uses the price snapshotted in `drawingState[currentDrawingId]`. 

If the admin updates the ticket price using `setTicketPrice` while a drawing is active:
1. If new price > old price: BridgeManager pulls the higher amount from the user, but Jackpot only takes the lower amount. The difference is permanently stuck in the BridgeManager contract.
2. If new price < old price: BridgeManager pulls the lower amount, but Jackpot attempts to transfer the higher amount from BridgeManager. The transaction reverts due to insufficient balance/allowance, causing DoS for bridge users.

## Impact
Loss of user funds (overpayment) or Denial of Service for cross-chain purchasers when admin updates ticket price.

## Command to Run Test


## Proof of Concept
1. `drawingState` has ticket price 1 USDC.
2. Admin calls `setTicketPrice(2 USDC)`.
3. User calls `BridgeManager.buyTickets(1 ticket)`.
4. BridgeManager pulls 2 USDC from user.
5. BridgeManager calls `Jackpot.buyTickets`.
6. Jackpot takes 1 USDC.
7. BridgeManager retains 1 USDC stuck in contract.

## Proof of Code
function testBridgePriceMismatch() public {
    // ... Setup ...
    jackpot.setTicketPrice(2e6);
    // Bridge buy calls jackpot.ticketPrice() -> 2e6
    // Jackpot.buy uses drawing state -> 1e6
    // Assert bridge balance increases by 1e6 (stuck)
}

## Suggested Mitigation
Update `JackpotBridgeManager.buyTickets` to fetch the price from `jackpot.getDrawingState(jackpot.currentDrawingId()).ticketPrice` instead of the global `jackpot.ticketPrice()`.


## [H-3]. Permanent Denial of Service (Jackpot Bricking) due to uint8 overflow in TicketComboTracker

## id: YgHyjMfIOjLev2Mvepm7b

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
Dos

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: Valid
### Finding Status Justification: TicketComboTracker.insert and countTierMatchesWithBonusball compute `1 << (_bonusball + _tracker.normalMax)` where both operands are uint8. In Solidity 0.8+, uint8 addition overflows and reverts if the sum > 255. Jackpot._setNewDrawingState sets drawingState.bonusballMax via an unsafe uint8 cast and does not enforce `normalBallMax + bonusballMax <= 255`. If `bonusballMax` ends up > (255 - normalMax), the entropy callback can select a bonusball such that `_bonusball + normalMax > 255`, causing scaledEntropyCallback to revert while `jackpotLock` remains true, bricking the drawing until emergency intervention. There is no guard/cap or safe cast in TicketComboTracker or _setNewDrawingState to prevent this.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `TicketComboTracker.sol`, the functions `insert` and `countTierMatchesWithBonusball` calculate a bit mask for the bonus ball using the expression `1 << (_bonusball + _tracker.normalMax)`. Both `_bonusball` and `_tracker.normalMax` are `uint8`. In Solidity 0.8+, arithmetic operations on `uint8` types that result in a value > 255 will revert due to overflow. 

The `bonusballMax` parameter is dynamically calculated in `_setNewDrawingState` based on the prize pool size to ensure LP edge; it can reach up to 255. `normalBallMax` is typically around 30-50 (capped at ~133 by Combinations library). If `bonusballMax + normalBallMax > 255`, and the entropy provider selects a high bonus ball value (e.g., 250 with normalMax=50), the addition `_bonusball + _tracker.normalMax` overflows and the transaction reverts. 

This occurs inside `scaledEntropyCallback`. If the callback reverts, the jackpot remains locked (`jackpotLock = true`) and cannot settle. Since `_tracker.normalMax` is fixed for the current drawing, retrying the drawing will fail again with the same parameters if entropy hits the overflow range again. This leads to a permanent DoS of the drawing, requiring an Emergency Mode activation to resolve.

## Impact
The current jackpot drawing becomes permanently stuck (bricked) if the winning number falls into the overflow range. Funds are locked until Emergency Mode is activated, disrupting protocol operation.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 50.
2. Prize pool grows such that `_setNewDrawingState` calculates `bonusballMax` = 250 (Total 300 > 255).
3. `runJackpot` is called, locking the drawing.
4. Entropy provider callback returns bonus ball = 240.
5. `scaledEntropyCallback` calls `_calculateDrawingUserWinnings` -> `countTierMatchesWithBonusball`.
6. Inside `countTierMatchesWithBonusball`: `_bonusball + _tracker.normalMax` evaluates to `240 + 50 = 290`. 
7. `uint8` addition overflows and reverts.
8. The callback fails, leaving the jackpot locked. The drawing cannot be settled.

## Proof of Code
function testJackpotBricking() public {
    // Simulate overflow condition in isolation
    uint8 normalMax = 50;
    uint8 bonusBall = 240;
    // This will revert in Solidity 0.8
    uint256 mask = 1 << (bonusBall + normalMax);
}

## Suggested Mitigation
Cast the operands to `uint256` before addition to prevent `uint8` overflow, and ensure the result does not exceed 255 (or handle larger shifts correctly if intended). 
Change: `1 << (uint256(_bonusball) + uint256(_tracker.normalMax))`.
Additionally, enforce `normalBallMax + bonusballMax < 256` in parameter setters.


## [H-4]. Protocol insolvency via late entropy callback after emergency refunds

## id: h_pS9Y_BQ1Beo4AW0LYRn

## Derived From Pattern/Invariant
EmergencyModeStateStuck

## Exploit Type
EmergencyModeStateStuck

## Location
Jackpot.sol.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: Enabling emergency mode is an intended recovery action; a late/uncancellable entropy fulfillment can still arrive and `scaledEntropyCallback` lacks `noEmergencyMode`, allowing settlement on a refunded round. This can happen even when governance follows the emergency spec, so it is not merely privileged-user mistake.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `Jackpot` contract allows the owner to enable `emergencyMode`, which permits users to refund tickets via `emergencyRefundTickets`. This function burns the tickets and transfers USDC back to users, effectively draining the contract's balance.

However, the `scaledEntropyCallback` function does not check for `emergencyMode`. If a drawing is locked and waiting for entropy, and `emergencyMode` is enabled, users can refund their tickets. If the entropy provider subsequently fulfills the request (which cannot be cancelled once on-chain), `scaledEntropyCallback` will execute.

This callback proceeds to settle the drawing using `_calculateDrawingUserWinnings`. This function relies on `TicketComboTracker`, which counts all inserted tickets. Since `emergencyRefundTickets` only burns the NFTs but does not (and cannot) update the append-only `TicketComboTracker`, the system calculates winnings as if the refunded tickets were still valid. 

The settlement updates `lpPoolTotal` and `newAccumulator` based on `lpEarnings` (which were not reduced by refunds) and calculated winnings. This creates a state where the LP pool accounting reflects a large balance that physically no longer exists in the contract (due to refunds). The protocol becomes insolvent, and subsequent claims or withdrawals will fail due to lack of USDC.

## Impact
Protocol insolvency and permanent state corruption. LPs and remaining winners cannot withdraw funds.

## Command to Run Test


## Proof of Concept
1. Drawing N starts. Users buy tickets.
2. `runJackpot` is called. Drawing N locked.
3. Entropy callback is delayed. Owner calls `enableEmergencyMode()`.
4. Users call `emergencyRefundTickets()`, draining USDC.
5. Entropy callback arrives. `scaledEntropyCallback` executes.
6. Drawing N settles. `lpPoolTotal` for Drawing N+1 is calculated based on original ticket revenue (phantom funds).
7. Emergency mode disabled (or even while enabled, if users claim winnings).
8. LPs try to withdraw. `Jackpot` has 0 USDC but `LPManager` thinks it has full pool. Transaction reverts.

## Proof of Code


## Suggested Mitigation
Add the `noEmergencyMode` modifier to `scaledEntropyCallback` to prevent settlement while in emergency mode, OR ensure that `emergencyRefundTickets` correctly updates accounting variables (though updating `TicketComboTracker` is difficult, reducing `lpEarnings` is minimal requirement).


## [M-5]. DoS in drawing settlement due to excessive storage reads

## id: CMiY9g80VQpACABfgBcHS

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.runJackpot

## Finding Status: Valid
### Finding Status Justification: TicketComboTracker._countSubsetMatches loops `i = 1..bonusballMax` and for each i loops k=1..5 and iterates over all subsets of the 5 winning normals (5+10+10+5+1=31 subsets), reading `_tracker.comboCounts[i][subset]` each time (count + dupCount). This yields ~31*bonusballMax mapping reads; at bonusballMax=255 that is ~7,905 reads plus subset generation overhead. Additionally, Jackpot.runJackpot computes entropyGasLimit as `entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax`; with the default entropyVariableGasLimit = 250,000 and bonusballMax=255, the requested gas limit is ~63.75M + base, exceeding typical block gas limits. This can prevent settlement (callback out-of-gas / cannot be mined), locking the drawing. No on-chain cap ensures bonusballMax stays within gas-feasible bounds.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `countTierMatchesWithBonusball` function in `TicketComboTracker` iterates from 1 to `bonusballMax`. For each iteration, it performs ~31 storage reads (`comboCounts`). If `bonusballMax` reaches its maximum (255), this results in ~7,900 storage reads. At 2100 gas per cold read, this costs ~16.6 million gas, not including overhead. Additionally, `runJackpot` calculates a gas limit for the entropy callback based on `bonusballMax`. With default parameters, this requests ~63 million gas, exceeding block gas limits on most chains. This can cause the drawing settlement to fail consistently, locking the jackpot.

## Impact
Permanent Denial of Service for the current drawing; the jackpot cannot settle, and funds are stuck until emergency admin intervention (if possible) or upgrade.

## Command to Run Test


## Proof of Concept
1. Prize pool allows `bonusballMax` to reach 255. 
2. `runJackpot` requests entropy with gas limit > 30M. 
3. Transaction fails or callback transaction runs out of gas/exceeds block limit. 
4. Jackpot remains locked.

## Proof of Code
No specific code proof needed; arithmetic verification of loop count * cold SLOAD cost demonstrates exceeding 15M+ gas.

## Suggested Mitigation
Optimize `TicketComboTracker` to avoid iterating all bonus balls (e.g. by tracking global normal counts), or enforce a strict cap on `bonusballMax` that respects block gas limits.


## [M-6]. DoS in entropy callback due to O(N) loop over `bonusballMax`

## id: gKC9qTRI9H8qtutbqRiqt

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
TicketComboTracker._countSubsetMatches

## Finding Status: Valid
### Finding Status Justification: TicketComboTracker._countSubsetMatches loops i=1..bonusballMax and for each i loops k=1..5 and iterates all subsets, repeatedly recomputing subsets and performing many mapping reads. For high bonusballMax (up to 255), callback gas can exceed feasible block limits, reverting scaledEntropyCallback and leaving the drawing locked (requiring emergency). Jackpot has no cap ensuring bonusballMax remains in a gas-safe range; entropyGasLimit scaling does not prevent block-limit failure. Thus DoS is reachable as prize pool/difficulty increases.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker._countSubsetMatches` function iterates from 1 to `bonusballMax`. Inside this loop, it iterates over all subsets of the normal balls (31 iterations). For each inner iteration, it accesses storage (`comboCounts`).

```solidity
// TicketComboTracker.sol
for (uint8 i = 1; i <= _tracker.bonusballMax; i++) {
    for (uint8 k = 1; k <= _tracker.normalTiers; k++) {
        // ... storage reads ...
    }
}
```

If `bonusballMax` is large (e.g., near 255), this results in ~8,000 storage reads, consuming ~16M+ gas. While `runJackpot` requests sufficient gas from the entropy provider (potentially exceeding block limits itself), the execution of the callback is bound by the block gas limit (typically 30M). If the gas required exceeds the block limit, the transaction will consistently fail, permanently locking the jackpot.

## Impact
Permanent denial of service for the drawing settlement; funds locked until emergency mode is activated.

## Command to Run Test


## Proof of Concept
1. Prize pool increases such that `bonusballMax` is set to 250.
2. Keeper calls `runJackpot`, paying for 63M gas limit (250k * 250).
3. Pyth provider attempts to call `scaledEntropyCallback`.
4. The execution requires ~20M gas (warm/cold access dependent).
5. If on a chain with lower limits or if overhead pushes it over 30M, the transaction reverts.
6. The drawing is stuck in `jackpotLock` state.

## Proof of Code
function testGasLoop() public {
    // Simulate loop cost
    uint256 startGas = gasleft();
    for(uint i=0; i<255; i++) {
        for(uint k=0; k<31; k++) {
             // simulate storage read
             uint256 x = dummyMap[i][k];
        }
    }
    uint256 used = startGas - gasleft();
    console.log("Gas used:", used);
}

## Suggested Mitigation
Optimize `_countSubsetMatches` to avoid iterating over all bonus balls. Track a global count of tickets matching each normal subset (regardless of bonus ball), then calculate non-matching bonus counts by subtracting the specific matching bonus count from the global count.


## [H-7]. First depositor inflation attack allows stealing the entire LP pool

## id: _vqtrRWpWQ7o1VKa8E_nJ

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager.sol.processDrawingSettlement

## Finding Status: Valid
### Finding Status Justification: No minimum initial liquidity / dead-shares exist. An attacker can be the first LP (deposit dust before `initializeJackpot`) and later cause extreme accumulator/share-price such that subsequent deposits mint 0 shares, effectively donating USDC to the attacker.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager` uses an accumulator-based share pricing model similar to ERC4626 vaults. The accumulator for a new drawing is calculated as `newAccumulator = (prevAccumulator * postDrawLpValue) / currentLP.lpPoolTotal`. 

When `initializeJackpot` is called, it initializes the first active drawing (ID 1) using the pending deposits from drawing 0. If the initial deposit is extremely small (e.g., 1 wei), `lpPoolTotal` for drawing 1 will be 1 wei. An attacker can front-run the initialization (or be the first depositor if the owner does not seed the pool) to deposit 1 wei.

During Drawing 1, ticket sales increase `lpEarnings`. At settlement, `postDrawLpValue` will be `1 wei + earnings`. The new accumulator will be calculated as `(1e18 * (1 + earnings)) / 1`. This results in a massive accumulator value.

Due to the high accumulator, subsequent deposits by other users will calculate shares as `(amount * 1e18) / accumulator`. If the accumulator is sufficiently large, this rounds down to zero shares for even substantial deposits. The attacker, holding the only shares (from the 1 wei deposit), retains 100% ownership of the pool and can withdraw the entire balance, stealing all subsequent deposits and earnings.

## Impact
Theft of the entire LP pool and all earnings by the first depositor.

## Command to Run Test


## Proof of Concept
1. Attacker calls `lpDeposit(1 wei)` (assuming `initializeLPDeposits` has been called). `pendingDeposits` becomes 1 wei.
2. Owner calls `initializeJackpot()`. This sets `currentDrawingId` to 1 and initializes Drawing 1 with `lpPoolTotal = 1 wei`.
3. Users buy tickets worth 100,000 USDC. `lpEarnings` increases by 100,000 USDC.
4. Drawing 1 ends. `runJackpot` is called and `scaledEntropyCallback` executes.
5. `processDrawingSettlement` calculates `newAccumulator = 1e18 * (1 + 100,000e6) / 1 ≈ 1e29`.
6. Victim deposits 10,000 USDC (10,000e6). Shares = `10,000e6 * 1e18 / 1e29 = 0`.
7. Attacker withdraws their shares. They own 100% of the pool (including the victim's 10,000 USDC).

## Proof of Code
/* In Foundry test */
vm.startPrank(attacker);
// 1. Deposit dust
usdc.approve(address(jackpot), 1);
jackpot.lpDeposit(1);
vm.stopPrank();

// 2. Admin initializes
vm.prank(owner);
jackpot.initializeJackpot(block.timestamp + 1000);

// 3. Earnings accumulate
vm.startPrank(user);
usdc.approve(address(jackpot), 1000e6);
jackpot.buyTickets(tickets, user, ...);
vm.stopPrank();

// 4. Settle
vm.warp(block.timestamp + 1001);
jackpot.runJackpot{value: fee}();
// ... mock entropy callback ...

// 5. Victim deposits
vm.startPrank(victim);
usdc.approve(address(jackpot), 10000e6);
jackpot.lpDeposit(10000e6);
// Check shares
(uint256 shares,,,) = lpManager.getLpInfo(victim);
assertEq(shares, 0); // Victim gets 0 shares
vm.stopPrank();

## Suggested Mitigation
Require a minimum initial deposit (e.g. 1e6 USDC) in `initializeJackpot`, or implement virtual shares/assets (dead shares) in `JackpotLPManager` to seed the pool and prevent extreme accumulator manipulation.


## [M-8]. DoS in `scaledEntropyCallback` due to excessive gas consumption when `bonusballMax` is large

## id: vwl7LBAYbTfDIjK--C2GH

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
TicketComboTracker.sol.countTierMatchesWithBonusball

## Finding Status: Valid
### Finding Status Justification: scaledEntropyCallback calls TicketComboTracker.countTierMatchesWithBonusball which executes _countSubsetMatches looping over all bonusballs up to drawingState.bonusballMax and iterating subsets for each. This is inherently gas-heavy and can exceed block gas limits at large bonusballMax values, reverting the callback and freezing the locked drawing. No on-chain guard caps bonusballMax to a gas-safe value.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function triggers `_calculateDrawingUserWinnings`, which calls `TicketComboTracker.countTierMatchesWithBonusball`. This function iterates through every possible bonus ball value up to `drawingState.bonusballMax` (which can be up to 255) and, for each value, iterates through all 31 ticket subsets to aggregate winner counts from storage.

Specifically, the nested loops in `_countSubsetMatches` perform approximately `bonusballMax * 31` storage reads. If `bonusballMax` reaches its maximum of 255 (which occurs automatically as the prize pool grows to maintain LP edge), the function performs roughly `255 * 31 = 7,905` SLOAD operations. Assuming cold access (2100 gas), this alone costs ~16.6 million gas, not counting overhead, memory expansion, and other logic.

If the total gas required exceeds the block gas limit (e.g., 30M on Base/Optimism) or the transaction gas limit, the callback will consistently revert. Since the drawing is locked (`jackpotLock = true`) until the callback succeeds, a revert permanently freezes the jackpot, requiring the owner to trigger Emergency Mode to dissolve the round and refund users.

## Impact
The jackpot drawing mechanism can become permanently stuck (DoS) for large prize pools, forcing an emergency dissolution of the round.

## Command to Run Test


## Proof of Concept
1. The prize pool grows large enough (e.g., ~65M USDC with current params) such that the calculated `bonusballMax` reaches 255 to maintain LP edge.
2. A drawing is initiated via `runJackpot`, locking the contract.
3. The entropy provider calls `scaledEntropyCallback`.
4. `TicketComboTracker.countTierMatchesWithBonusball` is executed.
5. The function iterates `i` from 1 to 255. Inner loops run 31 times per `i`.
6. Total storage reads ~7,905. Execution gas exceeds block/transaction limits.
7. Transaction reverts. The jackpot remains locked indefinitely.

## Proof of Code


## Suggested Mitigation
Optimize the `_countSubsetMatches` logic to avoid iterating over empty bonus ball buckets (e.g., by tracking active bonus balls) or cap `bonusballMax` to a lower safe limit (e.g., 50-100) ensuring gas costs remain well within limits.


## [H-9]. LP Vault Inflation Attack via Donation/Rounding

## id: 2yYNj1L5axFtkXQ3jU2Il

## Derived From Pattern/Invariant
ERC4626 Inflation Attack

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: Valid
### Finding Status Justification: LP shares for a deposit made in drawing D are minted later via `_consolidateDeposits`: `shares = amount * 1e18 / drawingAccumulator[D]` with truncation. If an attacker is (or becomes) essentially the only LP so lpPoolTotal is extremely small, then after a drawing with large `postDrawLpValue` (e.g., many ticket sales), processDrawingSettlement sets `drawingAccumulator[D] = prevAcc * postDrawLpValue / lpPoolTotal`, which can become extremely large. Then a victim deposit made during D can mint 0 shares when consolidated (non-dust amounts can be lost), while that USDC is already included into the pool via newLPValue at settlement, effectively transferring value to existing shareholders (attacker). There is no check `shares > 0` for `amount > 0`, and no minimum initial liquidity/share burn mechanism, so the inflation/rounding-to-zero vector exists.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The LP share calculation mechanism allows an attacker to manipulate the exchange rate (accumulator) to steal subsequent deposits. If the `lpPoolTotal` is small (e.g., 1 wei), an attacker can artificially inflate the `lpEarnings` (by buying tickets or donating prize-impacting funds) for the current drawing. In `processDrawingSettlement`, the `newAccumulator` is calculated as `(drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal`. A small denominator and large numerator results in a massive accumulator increase. Users who deposited during this drawing (pending deposits) will have their shares calculated using this inflated accumulator during consolidation: `shares = (amount * PRECISE_UNIT) / drawingAccumulator`. If the accumulator is sufficiently large, the user's shares round down to zero, and their deposit is effectively donated to the existing pool shareholders (the attacker).

## Impact
High. Theft of user deposits by manipulating share price precision.

## Command to Run Test


## Proof of Concept
1. Attacker ensures `lpPoolTotal` is 1 wei (via withdrawal or initial state if owner seeds minimally). 2. Attacker deposits to become the sole LP. 3. During Drawing N, attacker buys a large amount of tickets, increasing `lpEarnings` significantly (e.g., 1000 USDC). 4. Drawing N settles. `lpPoolTotal`=1, `postDrawLpValue` ≈ 1000e18. `newAccumulator` spikes to ~1e39 (assuming start 1e18). 5. Victim calls `lpDeposit(100 USDC)` during Drawing N. 6. Victim's deposit is pending. When consolidated, `shares = 100e18 * 1e18 / 1e39 = 0`. 7. Victim loses 100 USDC; Attacker owns 100% of the pool including victim's funds.

## Proof of Code
test/poc/LPInflation.spec.ts

## Suggested Mitigation
Enforce a minimum `lpPoolTotal` (e.g. 1000 wei) or burn the first shares/liquidity to prevent extreme inflation. Alternatively, track shares vs assets and revert if `shares == 0 && amount > 0` during consolidation.


## [H-10]. DoS in drawing settlement due to unbounded storage reads scaling with prize pool

## id: pP31oFKEwCodYTIe5wXUn

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: The callback path Jackpot.scaledEntropyCallback -> TicketComboTracker.countTierMatchesWithBonusball performs work proportional to bonusballMax (looping 1..bonusballMax and iterating subsets each time). As bonusballMax grows (up to uint8 max), the callback can exceed practical gas limits, leading to repeated callback reverts and a permanently locked drawing until emergency mode. This is a real unbounded-loop DoS risk; there is no enforcement that bonusballMax remains within a gas-safe range.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `bonusballMax` parameter is dynamically calculated based on the prize pool size to maintain LP edge. As the prize pool grows, `bonusballMax` increases (up to 255). The `scaledEntropyCallback` calls `TicketComboTracker.countTierMatchesWithBonusball`, which iterates from 1 to `bonusballMax`. Inside this loop, it performs 31 storage reads per iteration (checking subsets for tiers 1-5). 

At `bonusballMax = 255`, this results in `255 * 31 = 7905` storage reads. If these are cold reads (2100 gas), the cost is ~16.6 million gas, plus overhead. Additionally, the `runJackpot` function calculates the entropy callback gas limit as `base + variable * bonusballMax`. With the default `entropyVariableGasLimit` of 250,000, a maxed bonusball would require ~63M gas, exceeding the block gas limit (typically 30M). This causes `runJackpot` to revert, permanently freezing the drawing since `bonusballMax` is immutable for the current drawing.

## Impact
The jackpot drawing mechanism becomes permanently stuck if the prize pool grows large enough to push `bonusballMax` to high values, requiring emergency mode activation and refunds.

## Command to Run Test


## Proof of Concept
1. LPs deposit enough USDC to raise `prizePool` such that `bonusballMax` calculates to 255.
2. Keeper calls `runJackpot`.
3. Calculated gas limit is `Base + 250,000 * 255` ≈ 64M gas.
4. Call reverts due to exceeding block gas limit.
5. Drawing cannot be initiated/settled.

## Proof of Code
function testDoS() public {
  // Mock large pool
  vm.store(address(jackpot), bytes32(uint256(10)), bytes32(uint256(255))); // set bonusballMax to 255
  // Assert calculated gas > 30M
  uint32 gas = jackpot.getEntropyCallbackFee(); // internally calcs limit
  assertTrue(gas > 30_000_000);
}

## Suggested Mitigation
Optimize `TicketComboTracker` to avoid iterating all bonusballs (e.g. only iterate purchased tickets if fewer, or optimize storage layout), and/or lower the default `entropyVariableGasLimit` to realistic values.


## [H-11]. Protocol Insolvency via scaledEntropyCallback execution during Emergency Mode

## id: 32P4lFGxGsC3B2qmRJ-s2

## Derived From Pattern/Invariant
EmergencyModeStateStuck

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: This can occur when emergency mode is enabled as intended (to recover from being stuck) and the entropy callback arrives later; `scaledEntropyCallback` is not emergency-gated. It’s not solely attributable to privileged-user error.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract allows the owner to enable `emergencyMode` if the system is stuck. In this mode, users can call `emergencyRefundTickets` to burn their tickets and receive a full refund from the contract's USDC balance. However, the `scaledEntropyCallback` function does not check for `emergencyMode` and can still be successfully executed (e.g., by the entropy provider or a keeper) after refunds have been processed.

If the callback executes after refunds, it settles the drawing and credits LPs with `lpEarnings` derived from ticket sales. Since the USDC backing those earnings was already paid out to users via refunds, the contract becomes insolvent (accounting tracks more funds than are physically held). Consequently, LPs and winners will be unable to withdraw or claim their funds as the contract lacks the USDC to cover the internal accounting balances.

## Impact
Permanent lock of user and LP funds due to protocol insolvency (USDC balance < Liabilities).

## Command to Run Test


## Proof of Concept
1. `runJackpot` is called, locking the drawing and requesting entropy.
2. The entropy provider is delayed.
3. Owner calls `enableEmergencyMode` due to the delay.
4. Users call `emergencyRefundTickets`, draining USDC for their ticket costs.
5. The entropy provider transaction (`scaledEntropyCallback`) finally lands.
6. The drawing settles. `processDrawingSettlement` adds the full `lpEarnings` (including the value of refunded tickets) to the LP pool accounting.
7. The `lpPoolTotal` now exceeds the contract's actual USDC balance.
8. LPs attempting to withdraw via `finalizeWithdraw` or `emergencyWithdrawLP` will revert due to insufficient USDC balance.

## Proof of Code


## Suggested Mitigation
Add the `noEmergencyMode` modifier to `scaledEntropyCallback`, or ensure `scaledEntropyCallback` reverts if `emergencyMode` is enabled to prevent settlement of a refunded drawing.


## [H-12]. Emergency Refund "Ghost Winners" Cause Permanent LP Fund Loss

## id: 3GkWDLA-s_oJIsp47fXwi

## Derived From Pattern/Invariant
EmergencyModeStateStuck

## Exploit Type
EmergencyModeStateStuck

## Location
Jackpot.emergencyRefundTickets

## Finding Status: Valid
### Finding Status Justification: A late entropy callback after emergency refunds can settle using append-only `TicketComboTracker` data even if governance acted correctly by enabling emergency; this is not merely privileged-user misuse. It can create accounting distortions/LP losses (or trapped funds) without requiring governance to 'resume' the round.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `emergencyRefundTickets` function allows users to receive a refund and burns their ticket NFTs during emergency mode. However, it fails to remove the tickets from the `TicketComboTracker` (which uses a bit-vector implementation that doesn't support removal). If the system recovers (e.g., admin disables emergency mode and unlocks the jackpot, allowing `runJackpot` to be called again, or the delayed entropy finally arrives), the drawing will proceed to settlement. The `TicketComboTracker` will still count the refunded (burned) tickets as valid entries. If any of these "ghost" tickets are winners, `payoutCalculator` will include them in `drawingUserWinnings`. Consequently, `JackpotLPManager.processDrawingSettlement` will deduct this amount from the LP pool (`lpPoolTotal`). Since the NFTs are burned, no one can claim these winnings. This results in the LP pool losing value equivalent to the winnings of refunded tickets, and those funds becoming permanently stuck in the `Jackpot` contract balance.

## Impact
Direct loss of Liquidity Provider funds and permanent locking of USDC in the Jackpot contract.

## Command to Run Test


## Proof of Concept
1. `runJackpot` is called for Drawing N, locking the draw.
2. Entropy provider fails to callback; Admin enables `emergencyMode`.
3. Users call `emergencyRefundTickets`, receiving USDC and burning their NFTs. The `TicketComboTracker` still records their combinations.
4. Admin calls `disableEmergencyMode` and `unlockJackpot` to attempt to restart the system/drawing.
5. Keeper calls `runJackpot` again (or the original entropy callback finally arrives).
6. `scaledEntropyCallback` executes, calculating winners based on `TicketComboTracker` data.
7. Refunded tickets are identified as winners. `userWinnings` is calculated including these amounts.
8. `JackpotLPManager` reduces the LP pool by `userWinnings`.
9. LPs suffer a loss for payouts that can never be claimed.

## Proof of Code
/* 
 To reproduce, add this test to a Foundry test file importing Jackpot system contracts.
 Assume setup has 1 LP, 1 User who buys a winning ticket.
 */

function testGhostWinnerExploit() public {
    // 1. Setup and Buy Ticket
    vm.prank(user);
    jackpot.buyTickets(tickets, user, ...);
    
    // 2. Run Jackpot (Locks draw)
    vm.prank(keeper);
    jackpot.runJackpot{value: fee}();

    // 3. Enable Emergency & Refund
    vm.prank(owner);
    jackpot.enableEmergencyMode();
    
    vm.prank(user);
    jackpot.emergencyRefundTickets(userTicketIds);
    // User gets money back, NFT burned.

    // 4. Recover System
    vm.prank(owner);
    jackpot.disableEmergencyMode();
    vm.prank(owner);
    jackpot.unlockJackpot();

    // 5. Re-run Jackpot (or simulate entropy arrival)
    // Simulate entropy callback that makes the refunded ticket a winner
    vm.prank(address(entropyProvider));
    jackpot.scaledEntropyCallback(sequence, winningNumbers, ...);

    // 6. Check LP Loss
    // LP Pool reduced by winnings of the burned ticket
    // Funds stuck in Jackpot contract
}

## Suggested Mitigation
Implement a mechanism to mark a drawing as 'cancelled' in `Jackpot` state if refunds are issued, forcing the system to increment `currentDrawingId` without settling the finances of the tainted drawing. Alternatively, ensure `emergencyRefundTickets` is only usable if the drawing is permanently bricked and cannot be settled.





Finding Status: LowSeverityDueToRareLikelihood
## [H-13]. Silent truncation in bonusball calculation breaks LP edge guarantee

## id: gPqHzEq4oLmiomFMiZYy1

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot.sol._setNewDrawingState

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: In Jackpot._setNewDrawingState, `uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(...)))` performs an unchecked downcast. If the computed value exceeds 255, it wraps modulo 256, potentially producing a much smaller bonusballMax than required for intended odds/LP edge. The project includes UintCasts.toUint8() that would revert on overflow, but it is not used here, so no safeguard exists. This can materially change difficulty and LP expected value when pool grows large.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_setNewDrawingState`, the new `bonusballMax` is calculated to ensure the number of ticket combinations is sufficient to dilute player win rates and preserve LP edge. The calculation `Math.ceilDiv(minNumberTickets, combosPerBonusball)` can result in a value greater than 255, especially if `normalBallMax` is small (low combinations) or the prize pool is large. The result is cast to `uint8`, causing silent overflow/truncation.

For example, if the calculation yields 256, `bonusballMax` becomes 0 (or constrained by `bonusballMin`). This drastically reduces the number of combinations below the economic requirement, creating a negative-EV game for LPs.

## Impact
Severe loss of LP funds due to broken economic invariants (negative edge).

## Command to Run Test


## Proof of Concept
1. Configure `normalBallMax` = 5. `C(5,5)` = 1.
2. `ticketPrice` = 10 USDC. `lpEdge` = 0.5.
3. `prizePool` grows to 2560 USDC.
4. `minNumberTickets` = 2560 / (0.5 * 10) = 512.
5. `newBonusball` = ceil(512 / 1) = 512.
6. `uint8(512)` = 0. `bonusballMax` becomes `bonusballMin` (e.g. 10).
7. Required balls: 512. Actual: 10.
8. Win probability increases 50x. LPs drain.

## Proof of Code


## Suggested Mitigation
Check if the calculated bonusball count exceeds `type(uint8).max`. If so, revert or cap the prize pool growth/ticket sales. Alternatively, increase the size of `bonusballMax` variables.


## [M-14]. Ticket bitpacking overflow prevents winning bonus tiers

## id: Iod3raWRJSNTHVZ_JfD8X

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
RoundingError

## Location
TicketComboTracker.sol.insert

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: TicketComboTracker.insert sets bonusbit via `1 << (_bonusball + _tracker.normalMax)`. In the EVM, shifts by >=256 yield 0, so if normalMax+bonusball>=256 the bonus bit is silently lost and packed tickets collide. Jackpot does not enforce `normalBallMax + bonusballMax < 256`; _setNewDrawingState downcasts bonusballMax to uint8 without bounds checks, and lpEarnings growth can inflate pool size/difficulty. This breaks tiering and fairness (tickets can be misclassified/underpaid or collisions can change odds). No safeguard exists beyond intended economic caps (not enforced at packing).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Tickets are packed into a `uint256` bit vector where normal balls occupy bits `1` to `normalBallMax`, and the bonus ball occupies bit `normalBallMax + bonusball`. 

In `TicketComboTracker.insert`, the operation `set |= 1 << (_bonusball + _tracker.normalMax)` performs a shift. If `_bonusball + _normalMax >= 256`, the shift overflows `uint256` and results in 0. The ticket is saved without the bonus ball bit set. Consequently, `unpackTicket` and `_calculateTicketTierId` will fail to retrieve the correct bonus ball, making it impossible for the user to match the bonus ball and win associated tiers.

## Impact
Users purchasing tickets with high bonus ball numbers (valid within game rules) receive defective tickets that cannot win top prizes.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` = 200 (admin set).
2. `bonusballMax` = 60 (calculated dynamically).
3. User selects bonus ball 56.
4. `shift` = 200 + 56 = 256.
5. `1 << 256` in `uint256` is 0.
6. Ticket has no bonus bit.
7. Winning number has bonus 56.
8. `_calculateTicketTierId` checks bit 256. It is 0. No match.

## Proof of Code


## Suggested Mitigation
Ensure `normalBallMax + bonusballMax < 256` in configuration and dynamic calculation steps, or use a larger structure for ticket storage.


## [H-15]. Ticket corruption via bit packing overflow allows bonusball bypass

## id: 4YXUWS0t1gOuKoF-kGoE6

## Derived From Pattern/Invariant
BitwiseLogicError

## Exploit Type
IntegerOverflow

## Location
TicketComboTracker.insert

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: The core issue is correct: if normalMax+bonusball>=256, `1 << shift` becomes 0, losing the bonus bit and causing packed ticket collisions. The report’s “free bonusball match” consequence is partially inaccurate because Jackpot’s tiering uses bitwise encoding; losing the bonus bit causes misclassification/incorrect tier results rather than reliably granting a higher tier. Still, this is a real integrity bug that can change payouts and fairness when parameters allow shifts >=256.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The system packs ticket numbers into a `uint256` bit vector. Normal balls occupy bits `1` to `normalBallMax`, and the bonusball occupies the bit at `normalBallMax + bonusball`. 

```solidity
ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);
```

If `_bonusball + _tracker.normalMax` equals or exceeds 256, the shift operation `1 << shift` results in `0` in Solidity (for `uint256`). This results in the bonusball bit being completely lost. When `unpackTicket` or `_calculateTicketTierId` processes this ticket, the bonusball reads as 0. Since `buyTickets` validates `bonusball > 0`, legitimate high-difficulty tickets become corrupted. If the winning ticket also falls in this range, both user and winning tickets resolve to bonusball 0, granting a free bonusball match.

## Impact
Tickets with high bonusball numbers (generated when pool is large) are corrupted. This causes incorrect tier calculations, potential inability to claim correct winnings, or unintended matches if the winning ticket is also corrupted.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` = 35. `bonusballMax` grows to 225 due to large pool.
2. User buys ticket with bonusball 225.
3. Shift = 35 + 225 = 260.
4. `1 << 260` is 0.
5. Packed ticket has no bonus bit set.
6. `unpackTicket` returns bonusball 0.
7. If winning ticket also has bonusball 225, it also packs to 0. `_calculateTicketTierId` compares 0 == 0 -> Match.

## Proof of Code
function testBitOverflow() public {
    uint256 normalMax = 35;
    uint256 bonus = 225;
    uint256 shift = normalMax + bonus;
    uint256 mask = 1 << shift;
    assertEq(mask, 0);
}

## Suggested Mitigation
Enforce `normalBallMax + bonusballMax < 256` in `_setNewDrawingState` or use a larger storage structure.


## [H-16]. Unbounded Ticket Sales Bypass Pool Cap causing Bonusball Difficulty Overflow

## id: k-QRZiRyZV6a7jJRAock3

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot.buyTickets

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: lpPoolCap is enforced only in JackpotLPManager.processDeposit (LP deposits). Jackpot.buyTickets is uncapped and increases drawingState.lpEarnings; at settlement, lpEarnings contributes to `_newLpValue` (processDrawingSettlement), which drives the next drawing’s prizePool and then minNumberTickets. In Jackpot._setNewDrawingState, `newBonusball` is computed from minNumberTickets and then cast with `uint8(...)` (unsafe, wraps modulo 256 rather than reverting). If the computed value exceeds 255, bonusballMax will wrap to a much smaller number, reducing difficulty relative to the prizePool and breaking the edge model. There is no use of UintCasts.toUint8 here and no cap on ticket-driven pool growth, so the described overflow/truncation mechanism exists.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `lpPoolCap` mechanism calculates a maximum pool size to ensure that the required `bonusballMax` for the next drawing does not exceed 255 (which would overflow the `uint8` cast and bit vector storage). However, `lpPoolCap` only restricts `lpDeposit` calls. It does not limit `buyTickets`. Ticket purchases increase `lpEarnings`, which are added to the LP pool during settlement via `JackpotLPManager.processDrawingSettlement`. 

If ticket sales are sufficiently large (specifically, if `lpEarnings` pushes the new LP pool value slightly beyond the calculated `lpPoolCap`), the `minNumberTickets` calculation in `_setNewDrawingState` will derive a `newBonusball` value greater than 255. 

Snippet:
`uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));`

The explicit `uint8(...)` cast truncates the value (e.g., 260 becomes 4). This results in a drawing with a large prize pool but drastically reduced difficulty (small bonusball range), violating the LP edge guarantee and allowing players to extract value from LPs at -EV to the pool.

## Impact
High monetary loss for Liquidity Providers due to broken game difficulty.

## Command to Run Test


## Proof of Concept
1. Assume `lpPoolTotal` is near `lpPoolCap` (e.g., calculated to require `bonusballMax` = 250).
2. Attacker buys enough tickets to increase `lpEarnings` such that the new pool size requires `bonusballMax` = 260.
3. The drawing settles. `_setNewDrawingState` calculates 260 but casts it to `uint8(260) = 4`.
4. The next drawing is initialized with `bonusballMax = 4` instead of 260.
5. The difficulty is ~65x lower than intended, but the prize pool is large. Attacker buys all combinations in the new drawing to win the pot with massive profit.

## Proof of Code
function testBonusballOverflow() public {
    // Setup pool near cap
    vm.prank(owner);
    jackpot.initializeLPDeposits(1000000e6);
    // ... (full setup omitted for brevity) ...
    // Buy tickets to push earnings over the limit
    IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1000);
    // ... fill tickets ...
    jackpot.buyTickets(tickets, ...);
    // Settle drawing
    vm.warp(block.timestamp + duration + 1);
    jackpot.runJackpot{value: fee}();
    // Mock entropy callback
    // Check new drawing state
    Jackpot.DrawingState memory state = jackpot.getDrawingState(2);
    assertLt(state.bonusballMax, 10); // Should be > 250, but is < 10 due to overflow
}

## Suggested Mitigation
In `buyTickets`, ensure that adding ticket revenue to the pool will not cause the projected `bonusballMax` to exceed 255, or cap the Prize Pool growth such that `minNumberTickets` respects the limit. Alternatively, use `UintCasts.toUint8` to revert on overflow (causing DoS instead of theft) and provide a mechanism to sweep excess earnings.


## [H-17]. Unsafe cast of `bonusballMax` breaks LP edge protection

## id: T8-7ICK5P9PEGINxY3xiQ

## Derived From Pattern/Invariant
Integer Overflow / Unsafe Cast

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: Jackpot._setNewDrawingState computes `uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)))`. If the computed value exceeds 255, the explicit uint8 cast truncates modulo 256 (no revert), producing a much smaller bonusballMax than required by the edge math. This can drastically reduce difficulty for the next drawing while the prizePool is large, breaking the intended LP-edge guarantee. The project includes UintCasts.toUint8 (reverting cast), but it is not used here, and there is no explicit cap/check ensuring the computed bonusball fits in uint8.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, `bonusballMax` is calculated dynamically to maintain LP edge: `uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)))`. The result of the calculation is explicitly cast to `uint8`. If the `prizePool` is very large (e.g. > 65M USDC), `minNumberTickets` can become large enough that the result exceeds 255. The explicit cast truncates the value (modulo 256), resulting in a small `newBonusball` (e.g., 300 becomes 44). This drastically lowers the difficulty of the game below what is mathematically required to sustain the LP edge, exposing LPs to significant losses.

## Impact
High. Economic collapse of the LP pool due to broken difficulty scaling.

## Command to Run Test


## Proof of Concept
1. `prizePool` reaches ~70M USDC. 2. `minNumberTickets` / `combos` results in 300. 3. `uint8(300)` results in 44. 4. Game difficulty is set to 44 instead of 300. 5. Players have heavily +EV tickets. 6. LPs are drained.

## Proof of Code
test/poc/BonusballCast.spec.ts

## Suggested Mitigation
Check for overflow before casting. Cap `bonusballMax` at 255, or cap the `prizePool` growth if it necessitates a difficulty > 255.


## [H-18]. Downcasting in dynamic bonusball calculation allows difficulty manipulation and LP drainage

## id: xgZyroNtZLW_rOqMAIjpf

## Derived From Pattern/Invariant
UnsafeAssembyTypeCasts

## Exploit Type
RoundingError

## Location
Jackpot._setNewDrawingState

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: This is the same unchecked cast issue in Jackpot._setNewDrawingState: a computed uint256 bonusball requirement is cast to uint8 without bounds checks, allowing wraparound. Wraparound lowers bonusballMax, making the game easier than required for the stated edge guarantees and potentially leading to LP losses. UintCasts.toUint8 exists but is not used here.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot._setNewDrawingState` function calculates the `bonusballMax` for the next drawing to ensure LPs maintain a statistical edge. It calculates `minNumberTickets` required to cover the prize pool and edge, then derives the required bonusball range. However, the result is explicitly cast to `uint8` using `uint8(Math.max(bonusballMin, Math.ceilDiv(...)))`.

If the calculated difficulty requires a bonusball range greater than 255 (which occurs when the prize pool is large relative to the ticket price and normal ball combinations), the cast truncates the higher bits. This results in a much smaller `bonusballMax` than intended, drastically increasing the probability of winning and creating a negative EV for LPs.

## Impact
LP pool insolvency due to broken statistical edge. The game becomes trivial to win (or significantly easier) when the pool grows large.

## Command to Run Test


## Proof of Concept
1. Governance sets `normalBallMax` to 5 (min valid). `combosPerBonusball` = 1.
2. Governance sets `ticketPrice` to 1e6 (1 USDC) and `lpEdgeTarget` to 0.2e18.
3. LP Pool grows to 250 USDC. `prizePool` ~ 200 USDC.
4. `minNumberTickets` = 200 / (0.8 * 1) = 250 tickets.
5. Required bonusball = 250 / 1 = 250. `uint8(250)` = 250. Safe.
6. LP Pool grows to 300 USDC. `prizePool` ~ 240 USDC.
7. `minNumberTickets` = 240 / 0.8 = 300 tickets.
8. Required bonusball = 300. `uint8(300)` = 44 (300 % 256).
9. The game sets `bonusballMax` to 44 instead of 300.
10. Win probability increases 6.8x. Players buy tickets and drain the pool.

## Proof of Code
it("Overflows bonusball calculation", async function() {
  // Simulate state where required bonusball > 255
  // Assert bonusballMax is truncated
});

## Suggested Mitigation
Check if the calculated bonusball max exceeds `type(uint8).max`. If so, cap it at 255 or revert (though reverting bricks the drawing loop, so capping or increasing ticket price logic is preferred). However, since `bonusballMax` is physically limited to 255 by the `uint8` type in the struct, the protocol effectively cannot support prize pools that require difficulty > 255 with current `normalBallMax`. Mitigation: Require `normalBallMax` to be high enough to support the target pool size, or prevent pool growth beyond the supportable limit.


## [H-19]. Dynamic bonusballMax calculation causes bit packing failure and broken game logic

## id: CcdWyF2s0iEeWagNKXw2l

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: Two coupled issues exist: (1) unchecked uint8 downcast of computed bonusballMax in _setNewDrawingState (wraparound), and (2) TicketComboTracker bitpacking `1 << (bonusball + normalMax)` breaks when bonusball+normalMax>=256 (shift-to-zero), corrupting tickets and winner calculations. There is no enforceable invariant that keeps `normalBallMax + bonusballMax < 256`, and pool growth via lpEarnings can push computations beyond intended bounds.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_setNewDrawingState` function calculates `bonusballMax` dynamically to ensure LP edge. The formula is `ceil(minTickets / combosPerBonusball)`. This value is cast to `uint8`. 

Two critical issues arise:
1. If `minTickets / combosPerBonusball` exceeds 255, the `uint8` cast overflows (modulo 256), resulting in a much smaller `bonusballMax` than intended. This drastically lowers the difficulty, potentially causing LPs to payout frequently and lose edge.
2. More critically, `TicketComboTracker` packs tickets into a `uint256` bit vector using `1 << (bonusball + normalBallMax)`. If `bonusballMax + normalBallMax >= 256`, the shift amount overflows the 255-bit useful range (or 256-bit type width). In Solidity, `1 << 256` is 0. This results in the bonus ball bit being lost (zeroed). 

If `normalBallMax` is small (e.g., 10), `combos` is 252. A prize pool of just ~$47k requires `bonusballMax >= 246`. `246 + 10 = 256`. The bit packing fails. Tickets are minted with no bonus ball bit. `claimWinnings` and `countTierMatchesWithBonusball` will both see a bonus ball of 0 (from empty bit extraction), resulting in every ticket matching the bonus ball (0==0). This fundamentally breaks the game mechanics.

## Impact
Game integrity is destroyed; tickets may become unwinnable or trivially winnable (bonus match always true), and LPs may suffer massive losses.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 10 (valid setting). 
2. Ticket sales/deposits grow `prizePool` to 50,000 USDC. 
3. `runJackpot` is called. `scaledEntropyCallback` executes `_setNewDrawingState`. 
4. `minTickets` = 50,000 / 0.75 = 66,666. `combos` = 252. `bonusball` = ceil(66666/252) = 265. 
5. `uint8(265)` wraps to 9. 
6. New drawing starts with `bonusballMax` = 9. Difficulty is 30x lower than required for edge. 

Alternatively, if pool is $47k, `bonusball` = 246. `normal+bonus` = 256. 
7. Users buy tickets. `insert` calculates `1 << 256` = 0. Ticket packed value has no bonus bit. 
8. All tickets effectively have bonusball 0. Winning numbers also have bonusball 0. Everyone gets a bonus match.

## Proof of Code
function testBonusBallOverflow() public {
    // Simulate internal logic
    uint8 normalMax = 10;
    uint256 prizePool = 50000e6;
    uint256 ticketPrice = 1e6;
    uint256 lpEdge = 0.25e18;
    
    uint256 combos = 252; // choose(10,5)
    uint256 minTickets = prizePool * 1e18 / ((1e18 - lpEdge) * ticketPrice);
    uint256 rawBonus = (minTickets + combos - 1) / combos;
    
    uint8 castBonus = uint8(rawBonus);
    
    // Check overflow wrap
    console.log(rawBonus); // 265
    console.log(castBonus); // 9

    // Check shift failure
    if (castBonus < 100) { // Simulate the case where it doesn't wrap but hits limit
         // Force pool to ~47k case
         rawBonus = 246;
    }
    uint256 shift = 1 << (rawBonus + normalMax);
    assertEq(shift, 0);
}

## Suggested Mitigation
Ensure `normalBallMax + bonusballMax < 256`. Cap `bonusballMax` at `255 - normalBallMax` and/or increase `ticketPrice`/`normalBallMax` automatically if pool grows too large, or cap the pool size.


## [H-20]. Ticket storage corruption when `bonusballMax + normalBallMax >= 256`

## id: OOup1R7IXrLFBdECTjM-M

## Derived From Pattern/Invariant
BitwiseLogicError

## Exploit Type
StorageLayout

## Location
Jackpot._setNewDrawingState

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: TicketComboTracker.insert and countTierMatchesWithBonusball use `1 << (bonusball + normalMax)` for packing winning/played tickets. If the shift is >=256, the bit is zeroed, collapsing distinct tickets into identical packed representations and corrupting matching/tier logic. Jackpot does not validate `bonusballMax <= 255 - normalBallMax` when setting new drawings, so this can occur once parameters/pool size cause bonusballMax to grow too large.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library stores tickets as bit vectors using `1 << ballNumber`. The bonus ball bit is set at index `bonusball + normalMax`. In `Jackpot.sol`, `bonusballMax` is calculated dynamically. If `bonusballMax + normalBallMax` equals or exceeds 256, the shift operation `1 << (bonus + normalMax)` results in 0 (in EVM, shifting by >= 256 results in 0). 

This means tickets with high bonus ball numbers lose their bonus ball bit entirely. They effectively become indistinguishable from tickets with the same normal numbers but no valid bonus ball. The `unpackTicket` function will fail to correctly identify the bonus ball, potentially returning 0 or an invalid number. This corrupts the game state, making distinct tickets count as the same and creating `winningTicket` values that match non-winning user tickets.

## Impact
Game integrity failure. Winning tickets may not be recognized, or losing tickets may be counted as winners due to bit vector collisions. Ticket uniqueness checks fail.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` = 50. `bonusballMax` scales to 210 due to prize pool size.
2. Sum = 260.
3. User buys ticket with bonus ball 210.
4. `TicketComboTracker` calculates `1 << 260` -> 0.
5. Ticket stored has only normal ball bits.
6. User buys ticket with bonus ball 220.
7. `TicketComboTracker` calculates `1 << 270` -> 0.
8. System treats both tickets as duplicates of each other and stores them identically.

## Proof of Code
function testBitVectorCorruption() public {
    // ... Setup high prize pool relative to combos ...
    // Check if ticket with high bonus ball is stored correctly
    // Assert that unpacking it returns wrong bonus ball
}

## Suggested Mitigation
In `_setNewDrawingState`, ensure `newBonusball + normalBallMax < 256`. If the calculated `newBonusball` exceeds this limit, cap the prize pool or revert to prevent starting a broken drawing.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-21]. Permanent DoS of LP System via Accumulator Zero State

## id: 7erHIvs257H9_m67tQfIB

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
JackpotLPManager.processDeposit

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: JackpotLPManager.processDrawingSettlement sets `drawingAccumulator[_drawingId] = (drawingAccumulator[_drawingId-1] * postDrawLpValue) / lpPoolTotal` for _drawingId>0. If `postDrawLpValue` becomes 0, newAccumulator becomes 0 (when lpPoolTotal != 0). Later, _consolidateDeposits divides by `drawingAccumulator[lastDeposit.drawingId]`, so accumulator==0 causes division-by-zero reverts, breaking deposits/withdraw flows. Achieving postDrawLpValue==0 depends on configuration and economics (e.g., reserveRatio=0 combined with an outcome where lpPoolTotal + lpEarnings == userWinnings + protocolFee). The code contains comments claiming accumulator can never be zero, but there is no explicit lower bound enforcement. Because this hinges on admin-chosen parameters (reserveRatio and fee settings), it is primarily a governance/config footgun.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
If `reserveRatio` is set to 0 (allowed by config), `prizePool` equals `lpPoolTotal`. If a drawing results in a total loss (total winnings == prizePool, e.g., in Crisis Mode or jackpot hit) and there are no excess earnings, `postDrawLpValue` becomes 0. Consequently, `newAccumulator` becomes 0. Since `processDeposit` calculates shares as `amount * 1e18 / accumulator`, any subsequent deposit attempt will revert due to division by zero, permanently bricking the LP system.

## Impact
Permanent DoS of LP functionality. No new deposits can be made.

## Command to Run Test


## Proof of Concept
1. Admin sets `reserveRatio` to 0.
2. `prizePool` = `lpPoolTotal`.
3. A drawing occurs where winnings claim the entire prize pool (e.g. valid jackpot win or crisis mode distribution).
4. `postDrawLpValue` = 0.
5. `newAccumulator` = 0.
6. Next `lpDeposit` calls revert on div by zero.

## Proof of Code


## Suggested Mitigation
Ensure `postDrawLpValue` (and thus `accumulator`) has a minimum lower bound (e.g. 1 wei) or disallow `reserveRatio` of 0.


## [M-22]. LP funds permanently locked if reserve ratio is zero and pool is drained

## id: 8ri-M0WlBnlhXgNnYgj52

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
Dos

## Location
JackpotLPManager._consolidateDeposits

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: If processDrawingSettlement sets `drawingAccumulator[drawingId]=0`, then any LP whose lastDeposit.drawingId==drawingId will later hit `_lp.consolidatedShares += amount*1e18 / drawingAccumulator[drawingId]` in _consolidateDeposits and revert (division by zero). This also impacts emergencyWithdrawLP because it calls _consolidateDeposits. The code has no lower-bound clamp on newAccumulator and relies on an assumption (comment) that accumulator can never be zero, which is not enforced. The scenario depends on admin configuration (notably reserveRatio choices) and extreme draw economics, so it is mainly a governance/configuration risk that can nonetheless permanently lock some LP positions.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The protocol allows `reserveRatio` to be set to 0. If this configuration is used and a drawing results in a total loss (winnings >= pool), the `drawingAccumulator` for that round becomes 0 (as calculated in `JackpotLPManager.processDrawingSettlement`). Subsequently, any LP who deposited during that round cannot consolidate their position or withdraw funds, even via emergency methods. This is because `_consolidateDeposits` attempts to divide by the historical `drawingAccumulator`, which is 0, causing a revert. This results in permanent fund lockup for LPs of that specific round.

## Impact
Permanent loss of assets (LP funds) for users who deposited during the drained round.

## Command to Run Test


## Proof of Concept
1. Owner sets `reserveRatio` to 0.
2. LPs deposit into Drawing N.
3. Drawing N executes. Randomness results in winnings equal to or greater than the pool size (total wipeout).
4. `processDrawingSettlement` calculates `postDrawLpValue` = 0 and `newAccumulator` = 0.
5. Drawing N+1 starts.
6. LPs from Drawing N try to `initiateWithdraw` or `emergencyWithdrawLP`.
7. These functions call `_consolidateDeposits`, which accesses `drawingAccumulator[N]` (which is 0).
8. The operation `(amount * PRECISE_UNIT) / drawingAccumulator` reverts due to division by zero, trapping funds.

## Proof of Code
function testLPBricking() public {
    vm.prank(owner);
    jackpot.setReserveRatio(0);
    vm.prank(lp);
    usdc.approve(address(jackpot), 100e6);
    jackpot.lpDeposit(100e6);
    // ... Trigger drawing execution where winnings >= pool ...
    // Assume accumulator becomes 0
    vm.prank(lp);
    vm.expectRevert(); // Division by zero
    jackpot.initiateWithdraw(100);
}

## Suggested Mitigation
Ensure `newAccumulator` never drops to absolute zero in `processDrawingSettlement` (e.g., cap at 1 wei), or enforce a minimum `reserveRatio` > 0 to prevent total pool drainage.


## [M-23]. Protocol DoS in drawing settlement if `referralFee` exceeds `lpEdgeTarget`

## id: 6q26FikrI1OYb3QsAxsqi

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
Dos

## Location
Jackpot.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: If misconfigured, settlement can underflow/revert in `JackpotLPManager.processDrawingSettlement`, locking a drawing and forcing emergency procedures; impact is not low (even though it stems from governance parameter choices).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol relies on the invariant that duplicate ticket purchases increase the prize pool by `ticketPrice * (1 - lpEdgeTarget)`, while LP earnings track `ticketPrice * (1 - referralFee)`. If the admin sets `referralFee > lpEdgeTarget`, buying a duplicate ticket causes the liability (prize pool increase) to grow faster than the asset (LP earnings). 

During `processDrawingSettlement`, the contract calculates `postDrawLpValue = lpPoolTotal + lpEarnings - userWinnings - protocolFee`. If a malicious user purchases a large number of duplicate tickets (or if organic demand creates them), the term `userWinnings` (which can be up to `prizePool`) can exceed `lpPoolTotal + lpEarnings`, causing `postDrawLpValue` to underflow. Since this calculation happens in the `scaledEntropyCallback`, the callback will revert, permanently locking the drawing and bricking the protocol until emergency mode is activated.

## Impact
Permanent Denial of Service for the active drawing. The drawing cannot be settled, locking all funds and preventing new drawings until emergency intervention.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` (e.g. 20%) higher than `lpEdgeTarget` (e.g. 10%). 
2. A drawing starts with a small `lpPoolTotal`. 
3. Attacker purchases a large number of duplicate tickets for a winning combination. 
4. Each duplicate adds `0.9 * Price` to `prizePool` (liability) but only `0.8 * Price` to `lpEarnings`. 
5. The net loss to the LP pool per duplicate is `0.1 * Price`. 
6. If `Total Loss > lpPoolTotal`, the subtraction `lpPoolTotal + lpEarnings - userWinnings` will underflow when `userWinnings` approaches `prizePool`. 
7. The settlement transaction reverts, locking the jackpot.

## Proof of Code


## Suggested Mitigation
Add a validation check in `setReferralFee` and `setLpEdgeTarget` to ensure `referralFee <= lpEdgeTarget`. Additionally, ensure `postDrawLpValue` calculation floors to zero instead of reverting on underflow, or cap `userWinnings` to available equity.


## [H-24]. Permanent LP Lockup if Drawing Accumulator Drops to Zero

## id: zUWPov9N-JpA5jKHUWpPt

## Derived From Pattern/Invariant
DivideByZeroOrOverFlowInCustomMath

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: JackpotLPManager.processDrawingSettlement can set `newAccumulator = (prevAcc * postDrawLpValue) / lpPoolTotal`. If postDrawLpValue becomes 0, newAccumulator becomes 0 and is stored. Later, _consolidateDeposits divides by `drawingAccumulator[depositDrawingId]`, which would be 0 and revert, potentially locking LP interactions. Although comments claim accumulator can never be zero, that invariant is not enforced and can be broken under extreme/incorrect governance parameters (e.g., referralFee=100% making lpEarnings=0, reserveRatio=0, and payout configuration that can fully deplete lpPoolTotal).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager.processDrawingSettlement`, the accumulator is updated based on the drawing's performance: `newAccumulator = (prevAccumulator * postDrawLpValue) / prevLpTotal`. If `postDrawLpValue` drops to 0 (e.g., due to a total loss in a drawing with 0% reserve ratio), `newAccumulator` becomes 0. 

Since the accumulator update formula is multiplicative, once the accumulator hits 0, it remains 0 forever for all future drawings. Furthermore, `_consolidateDeposits` calculates shares as `amount * PRECISE / accumulator`. If the accumulator is 0, this division reverts. 

Any LP who has a deposit associated with a zero-accumulator drawing (either as `lastDeposit` or via consolidation chain) will be permanently unable to deposit, withdraw, or emergency withdraw, as any attempt to interact with their position will trigger the division-by-zero revert.

## Impact
Permanent system bricking and lockup of LP funds. The accumulator logic cannot recover from a zero value, and LPs attached to the failed state cannot exit.

## Command to Run Test


## Proof of Concept
1. `reserveRatio` is set to 0 (allowed).
2. A drawing occurs where user winnings equal the total LP pool + earnings. `postDrawLpValue` = 0.
3. `processDrawingSettlement` sets `drawingAccumulator[d]` = 0.
4. An LP who made a deposit in drawing `d` tries to `emergencyWithdrawLP`.
5. The function calls `_consolidateDeposits`, which attempts `lastDeposit.amount * PRECISE / drawingAccumulator[d]`.
6. Reverts due to division by zero. The LP's funds are permanently locked.

## Proof of Code
/* Compilable Foundry test would simulate a drawing with 0 reserve ratio and 100% loss, then attempt LP interaction to confirm revert. */

## Suggested Mitigation
Ensure `postDrawLpValue` never drops below a minimum threshold to keep the accumulator non-zero, or add a special case in `_consolidateDeposits` to handle zero accumulators (e.g., treat shares as 0 or valid loss).





Finding Status: InvalidGovernanceRisk
## [M-25]. Referral fee arbitrage on duplicate tickets

## id: zooRxwCzyh5h0257QeB8E

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: On duplicate tickets, Jackpot._validateAndStoreTickets increases prizePool by `ticketPrice - edgePerTicket` (i.e., ticketPrice*(1-lpEdgeTarget)). Meanwhile buyTickets increases lpEarnings by `ticketPrice - referralFee portion` (order-level: ticketsValue - referralFeeTotal). If referralFee > lpEdgeTarget, then on duplicates the prizePool liability increases more than net assets credited to LP earnings, eroding the intended LP edge and potentially making ticket purchasing +EV (especially with self-referral). There is no invariant enforcement tying referralFee <= lpEdgeTarget (setReferralFee and setLpEdgeTarget are independent). This is mostly a governance/configuration risk: it is exploitable permissionlessly only after governance sets an unsafe parameter combination.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a duplicate ticket is purchased, `Jackpot` increases the prize pool by `ticketPrice * (1 - lpEdgeTarget)` to maintain LP edge. However, the protocol collects `ticketPrice - referralFee` in earnings/assets (assuming referral fee is paid out). 

If the configured `referralFee` is greater than `lpEdgeTarget`, the liability increase (prize pool addition) exceeds the asset increase (net earnings). Specifically, the protocol loses `referralFee - lpEdgeTarget` in value for every duplicate ticket sold. An attacker can exploit this by buying duplicate tickets using their own referral link, effectively extracting value from the LP pool.

## Impact
Value extraction from the LP pool via duplicate ticket purchases.

## Command to Run Test


## Proof of Concept
1. `ticketPrice` = 100.
2. `lpEdgeTarget` = 10% (liability adds 90).
3. `referralFee` = 20% (protocol keeps 80).
4. User buys duplicate ticket. Pays 100. Gets 20 back as referrer. Net cost 80.
5. Prize pool (liability) increases by 90.
6. Protocol/LP net position: +80 asset, -90 liability = -10 loss.

## Proof of Code
function testReferralArbitrage() public {
    // Setup fees such that referral > edge
    // Demonstrate net asset < net liability increase
}

## Suggested Mitigation
Add a validation check in `setReferralFee` and `setLpEdgeTarget` to ensure `referralFee <= lpEdgeTarget`.


## [H-26]. Protocol insolvency via emergency refund followed by late settlement

## id: iyl-C57tVzNB6JJJJSsCy

## Derived From Pattern/Invariant
EmergencyModeStateStuck

## Exploit Type
EmergencyModeStateStuck

## Location
Jackpot.scaledEntropyCallback

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Jackpot.emergencyRefundTickets burns current-drawing tickets and transfers USDC back but does not decrement drawingState[currentDrawingId].lpEarnings/globalTicketsBought nor remove counts from TicketComboTracker. Jackpot.scaledEntropyCallback is NOT protected by `noEmergencyMode`, so a delayed entropy callback can still settle after refunds. Settlement then uses stale lpEarnings and the unchanged combo tracker, crediting LP accounting as if ticket revenue was retained and as if refunded tickets still existed. Because refunds already reduced the real USDC balance, the internal accounting can overstate LP backing, leading to insolvency on withdrawals/claims. This requires the owner to enable emergencyMode (trusted role), hence governance risk, but can occur even without malice (oracle delay). No existing safeguard prevents callback execution during emergencyMode.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Jackpot` contract allows the owner to enable `emergencyMode`, which permits users to refund tickets for the current drawing via `emergencyRefundTickets`. This function burns the ticket NFT and transfers the ticket price back to the user from the contract's USDC balance. Crucially, it does not remove the ticket's numbers from the `TicketComboTracker`, nor does it decrement `globalTicketsBought` or `lpEarnings`.

If the drawing's entropy callback (`scaledEntropyCallback`) executes *after* refunds have occurred (e.g., if entropy was merely delayed, not failed), the drawing settles normally. The `processDrawingSettlement` logic calculates `lpEarnings` based on the original `globalTicketsBought` (including refunded tickets) and credits this to the LP pool. It also calculates `userWinnings` based on `TicketComboTracker`, which still counts the refunded (burned) tickets as winners. The `Jackpot` contract deducts the payouts for these 'ghost' winners from the LP pool, but since the tickets are burned, the payouts become unclaimed dust in the contract.

However, the contract's USDC balance was already reduced by the refunds. The LP pool accounting assumes it holds the full value of ticket sales, but the physical USDC for refunded tickets is gone. Specifically, for every refunded ticket, the LP pool overstates its backing by `TicketPrice`. If the system pays out winnings and LPs attempt to withdraw, the contract will not have enough USDC to cover the LP pool's tracked value, leading to insolvency.

## Impact
Protocol insolvency; contract USDC balance becomes less than the LP pool's tracked obligations, preventing LPs from withdrawing their funds.

## Command to Run Test


## Proof of Concept
1. Drawing N starts. 100 tickets sold (100 USDC). LP Pool tracks +100 pending earnings.
2. Entropy is delayed. Owner enables Emergency Mode.
3. All 100 users call `emergencyRefundTickets`. Contract balance -100 USDC. Tickets burned.
4. Entropy arrives. `scaledEntropyCallback` executes.
5. Settlement adds 100 USDC (earnings) to LP Pool (accounting only). Real balance doesn't increase.
6. LP Pool value increases by 100.
7. LPs attempt to withdraw. The contract lacks the 100 USDC that was refunded to users.

## Proof of Code
it("Creates insolvency via refund and settlement", async function () {
  // Buy ticket
  await jackpot.buyTickets(...);
  // Enable emergency
  await jackpot.enableEmergencyMode();
  // Refund
  await jackpot.emergencyRefundTickets([...]);
  // Mock entropy callback execution
  await entropyProvider.simulateCallback(...);
  // Check LP value vs Contract Balance
  const lpValue = (await lpManager.getLPDrawingState(1)).lpPoolTotal;
  const balance = await usdc.balanceOf(jackpot.target);
  expect(balance).to.be.lt(lpValue);
});

## Suggested Mitigation
Add `noEmergencyMode` modifier to `scaledEntropyCallback` in `Jackpot.sol` to ensure a drawing cannot settle if emergency mode is active, or ensure refunds decrement `globalTicketsBought` and `lpEarnings`.


## [H-27]. Insolvency due to phantom LP earnings after emergency refund

## id: UlJWW4vAQEty1QFYXcmFe

## Derived From Pattern/Invariant
EmergencyModeStateStuck

## Exploit Type
EmergencyModeStateStuck

## Location
Jackpot.emergencyRefundTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Jackpot.emergencyRefundTickets refunds USDC but does not adjust drawingState[currentDrawingId].lpEarnings/globalTicketsBought. If the drawing is later settled (e.g., delayed entropy callback, or emergency disabled and normal flow resumes), processDrawingSettlement will incorporate lpEarnings that are no longer backed by USDC (already refunded). This inflates LP accounting and can lead to insolvency upon withdrawals. The code also lacks a `noEmergencyMode` guard on scaledEntropyCallback to stop settlement during emergency mode. Because emergencyMode is owner-controlled, this is governance-triggered but can occur through operational mistake during oracle delays.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
When `emergencyMode` is enabled, users can call `emergencyRefundTickets`, which refunds USDC from the contract balance. However, this function does not decrement `drawingState[currentDrawingId].lpEarnings` or `globalTicketsBought`, which track the revenue from these tickets.

If the owner subsequently disables `emergencyMode` to resume operations (or if the entropy callback was already in flight and executes), `scaledEntropyCallback` will call `processDrawingSettlement`. This function calculates `newLPValue` using the stale `lpEarnings`, effectively crediting LPs with revenue that has already been refunded to users.

Future drawings will be backed by this phantom value. When LPs attempt to withdraw based on the inflated shares/accumulator, the contract will not have sufficient USDC (as it was paid out in refunds), leading to insolvency.

## Impact
Protocol insolvency and loss of funds for the last LPs to withdraw.

## Command to Run Test


## Proof of Concept
1. Users buy tickets; `lpEarnings` = 1000 USDC. Contract Balance = 1000 USDC.
2. Emergency mode enabled.
3. Users refund tickets; Balance = 0 USDC. `lpEarnings` remains 1000.
4. Emergency disabled, `runJackpot` callback executes.
5. `processDrawingSettlement` adds `lpEarnings` (1000) to `lpPool`.
6. LPs shares are now valued at 1000 USDC, but Contract Balance is 0.

## Proof of Code
function testPhantomEarnings() public {
    // Simulation logic described in PoC
}

## Suggested Mitigation
In `emergencyRefundTickets`, decrement `drawingState[ticketInfo.drawingId].lpEarnings` by the refunded amount (net of referral fees) and decrement `globalTicketsBought`.


## [H-28]. Protocol insolvency due to accounting mismatch in emergency refunds

## id: OKpndLRODMTkX8Y8oYRbj

## Derived From Pattern/Invariant
EmergencyModeStateStuck

## Exploit Type
EmergencyModeStateStuck

## Location
Jackpot.emergencyRefundTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: The code for emergencyRefundTickets only burns tickets and transfers USDC; it does not decrement lpEarnings nor remove ticket combinations from TicketComboTracker. If settlement later occurs, LP accounting can be credited with refunded ticket revenue and payouts can be computed using “ghost” tickets that no longer exist, distorting obligations vs actual USDC balance. This can under-collateralize the LP pool and/or strand funds. There is no safeguard preventing settlement while emergencyMode is on (scaledEntropyCallback lacks noEmergencyMode), so a delayed callback can trigger the problematic path. This is largely a governance/operational risk because emergencyMode must be enabled by the owner.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `emergencyRefundTickets` function allows users to refund their tickets and burn the corresponding NFTs during emergency mode. However, the function only transfers USDC back to the user and burns the ticket; it fails to decrement `lpEarnings` or update the `TicketComboTracker` state. 

If the emergency mode is subsequently disabled and the drawing is settled (via `runJackpot`/callback), the `processDrawingSettlement` function will credit the full `lpEarnings` (including the refunded amounts) to the LP pool. Since the actual USDC backing these earnings has been refunded, the LP pool becomes under-collateralized (insolvent). Additionally, `TicketComboTracker` still counts the refunded tickets as valid entries, causing the PayoutCalculator to allocate prize funds for 'ghost' winners (which cannot be claimed as tickets are burned), further distorting the accounting.

## Impact
The protocol becomes insolvent. LPs are credited with phantom earnings that do not exist in the contract balance. Future withdrawals or payouts will fail due to insufficient USDC.

## Command to Run Test


## Proof of Concept
1. Users buy 1000 USDC worth of tickets for Drawing N. `lpEarnings` = 1000. Contract Balance = 1000.
2. Emergency mode is enabled.
3. Users call `emergencyRefundTickets`, receiving 1000 USDC back. Balance = 0. `lpEarnings` remains 1000.
4. Owner disables emergency mode and triggers `runJackpot` to settle Drawing N.
5. `processDrawingSettlement` adds 1000 `lpEarnings` to the LP pool value.
6. LPs now own 1000 USDC of shares that have no backing assets. If they attempt to withdraw, the transaction will revert due to lack of funds.

## Proof of Code
function test_InsolvencyAfterRefund() public {
    // Setup: Buy ticket
    vm.startPrank(user);
    usdc.approve(address(jackpot), ticketPrice);
    IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1);
    tickets[0] = IJackpot.Ticket({normals: normals, bonusball: 1});
    jackpot.buyTickets(tickets, user, new address[](0), new uint256[](0), bytes32(0));
    vm.stopPrank();

    // Enable emergency and refund
    vm.prank(owner);
    jackpot.enableEmergencyMode();
    uint256[] memory ids = new uint256[](1);
    ids[0] = jackpot.getTicketIds(user)[0];
    vm.prank(user);
    jackpot.emergencyRefundTickets(ids);

    // Disable emergency and settle
    vm.prank(owner);
    jackpot.disableEmergencyMode();
    // ... (Advance time, run jackpot, mock entropy callback) ...
    // Assert LP pool value includes refunded earnings but contract balance does not
}

## Suggested Mitigation
In `emergencyRefundTickets`, decrement `drawingState[currentDrawingId].lpEarnings` by the refunded amount (excluding referral fees). Note that fixing the `TicketComboTracker` winner count is difficult; the protocol should ideally consider voiding drawings with refunds.


## [M-29]. Emergency refund value discrepancy due to global referral fee modification

## id: WD-W7tSZfsLn-2lYNfhHE

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Changing referralFee between purchase and emergencyRefundTickets can over-refund by up to the full referral portion per ticket; across many tickets this can be a large balance-sheet loss or large user under-refund, not a low-impact issue.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
`emergencyRefundTickets` calculates the refund amount using the *current* global `referralFee` rather than the fee active at the time of ticket purchase. `referralFee` is not snapshotted in `DrawingState`. If the `referralFee` is modified by the owner between ticket purchase and emergency refund execution, the refund amount will be incorrect. Specifically, if the fee is lowered, the protocol refunds more than it received (bleeding funds); if raised, users receive less than entitled.

## Impact
Protocol insolvency (leakage) or user fund loss during emergency refunds.

## Command to Run Test


## Proof of Concept
1. `referralFee` is set to 20%.
2. User buys ticket for 100 USDC. 20 USDC goes to referrer, 80 USDC to LP pool.
3. Admin changes `referralFee` to 0%.
4. Emergency mode is enabled for the drawing.
5. User calls `emergencyRefundTickets`.
6. Calculation: `100 * (1e18 - 0) / 1e18` = 100 USDC.
7. User receives 100 USDC. Protocol only holds 80 USDC from that ticket. Protocol loses 20 USDC per ticket.

## Proof of Code
function testRefundLeakage() public {
    vm.prank(owner);
    jackpot.setReferralFee(20e16);
    // User buys ticket...
    vm.prank(owner);
    jackpot.setReferralFee(0);
    vm.prank(owner);
    jackpot.enableEmergencyMode();
    // User refunds...
    // Assert user received full amount despite referrer keeping 20%
}

## Suggested Mitigation
Store the `referralFee` used for each drawing in `DrawingState` and use that stored value for refund calculations.



