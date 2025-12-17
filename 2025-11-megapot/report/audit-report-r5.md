# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[H-1]. Ticket corruption and collision due to bit shifting overflow in `TicketComboTracker`
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[M-2]. DoS in Settlement due to O(N) Gas Cost in `TicketComboTracker`
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-3]. Bit vector overflow corruption bricks ticket verification
**Derived From** : StorageLayout
Finding Status: Valid
Privilege: Permissionless


[H-4]. Valid entropy callback after emergency refunds permanently locks winnings in contract
**Derived From** : EmergencyModeStateStuck
Finding Status: Valid
Privilege: Permissionless


[M-5]. DoS of Jackpot Settlement due to excessive gas usage in `scaledEntropyCallback`
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-6]. LP Share Price Inflation Attack via First Depositor Manipulation
**Derived From** : ERC4626SharePriceMismatch
Finding Status: Valid
Privilege: Permissionless


[H-7]. Ticket bit-packing collision allows guaranteed bonus ball match for high bonusball values
**Derived From** : IntegerMath
Finding Status: Valid
Privilege: Permissionless


[H-8]. TicketComboTracker bitwise overflow corrupts tickets
**Derived From** : StorageLayout
Finding Status: Valid
Privilege: Permissionless


[H-9]. Broken ticket encoding causes incorrect payouts when total ball range exceeds 256
**Derived From** : IntegerMath
Finding Status: Valid
Privilege: Permissionless


[H-10]. Difficulty parameter truncation allows trivial jackpot wins and LP draining
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-11]. Catastrophic LP edge collapse due to uint8 casting truncation in bonusball calculation
**Derived From** : IntegerMath
Finding Status: Valid
Privilege: Permissionless


[M-12]. DoS and Logic Corruption via Uncapped Ticket Sales (Bitpacking Overflow)
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[H-13]. Unsafe Downcast of `bonusballMax` leads to critical game parameter corruption
**Derived From** : CastingOverflow
Finding Status: Valid
Privilege: Permissionless



Finding Status: InvalidGovernanceRisk


[M-14]. DoS of Jackpot execution due to excessive gas limit configuration
**Derived From** : GasGriefBlockLimit
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless


[H-15]. Emergency refund drains contract if referral fee lowered
**Derived From** : ConfigFootgun
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-16]. Replacing PayoutCalculator freezes all unclaimed winnings
**Derived From** : ConfigFootgun
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-17]. Referral arbitrage allows risk-free draining of LP pool
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-18]. LP Pool DoS via Zero Accumulator when pool value is drained
**Derived From** : Dos
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 12
- M: 6
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Ticket corruption and collision due to bit shifting overflow in `TicketComboTracker`

## id: PR3UqVLLcuvw_jQJHea-W

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot.insert

## Finding Status: Valid
### Finding Status Justification: The shift-by->=256 corruption is real (EVM SHL yields 0), there is no on-chain invariant enforcing normalBallMax + bonusballMax < 256, and lpPoolCap does not strictly prevent reaching unsafe bonusballMax because lpPoolTotal can grow beyond cap via lpEarnings. Once reached, tickets can be mis-packed and bonusball matching/tiering can be manipulated or made incorrect, which is high impact and not inherently rare over long protocol operation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library packs ticket numbers (5 normals + 1 bonusball) into a single `uint256` bit vector. The bit position for the bonus ball is calculated as `_bonusball + _tracker.normalMax`. 

```solidity
ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);
```

If `_bonusball + _tracker.normalMax >= 256`, the shift operation `1 << ...` results in 0 in Solidity (EVM SHL). This effectively deletes the bonus ball from the ticket data. This occurs when `bonusballMax` is high (due to a large prize pool) or `normalBallMax` is high. 

Crucially, this corruption affects both user tickets and the winning ticket. If a user buys a ticket with a high bonus ball (e.g. 230 with normalMax 35 -> index 265), the bonus bit is lost (recorded as 0). If the winning number also has a high bonus ball, its bit is also lost (recorded as 0). The system will perceive these two tickets as having matching bonus balls (both 0/missing), creating a 100% match rate for all bonus balls in the overflow range.

## Impact
Users can guarantee a bonus ball match by selecting any bonus ball in the overflow range, significantly increasing their odds of winning (e.g. hitting the Jackpot or high tiers). This exploits the collision where all high bonus balls map to 'no bonus ball'.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` = 35. Prize pool is large enough that `bonusballMax` is set to 230.
2. User buys ticket with bonus ball 225. Shift index = 225 + 35 = 260.
3. `1 << 260` == 0. Ticket stored with only normal balls set.
4. Winning numbers drawn: bonus ball 230. Shift index = 230 + 35 = 265.
5. `1 << 265` == 0. Winning ticket stored with only normal balls set.
6. `_calculateTicketTierId` extracts bonus via right shift: `packed >> 36`. Both return 0.
7. The tickets match on the bonus ball despite being different numbers (225 vs 230).

## Proof of Code
function testShiftOverflow() public {
    uint256 normalMax = 35;
    uint256 bonus = 225;
    uint256 shift = bonus + normalMax;
    uint256 packed = 1 << shift;
    assertEq(packed, 0);
}

## Suggested Mitigation
Ensure that `normalBallMax + bonusballMax < 256`. Add a check in `_setNewDrawingState` and `setNormalBallMax` to enforce this invariant. If the calculated `bonusballMax` would violate this, cap it at `255 - normalBallMax`.


## [M-2]. DoS in Settlement due to O(N) Gas Cost in `TicketComboTracker`

## id: ZQU6eEzkN8XogAwDDgfPV

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: Valid
### Finding Status Justification: `Jackpot.scaledEntropyCallback` calls `_calculateDrawingUserWinnings`, which calls `TicketComboTracker.countTierMatchesWithBonusball`, which calls `_countSubsetMatches`. `_countSubsetMatches` loops `i=1.._tracker.bonusballMax` and, for each `i` and `k=1..5`, recomputes subsets via `Combinations.generateSubsets` and performs storage reads on `comboCounts`. This is concretely O(bonusballMax) work and is executed inside the entropy callback path (critical liveness path while `jackpotLock` is true). With high `bonusballMax` (up to 255), gas usage can become very large and risks reverting the callback; since `scaledEntropyCallback` requires `jackpotLock==true`, repeated callback failures can leave the drawing stuck/locked until emergency intervention. There is no hard cap/optimized iteration over only-used bonusballs; the only “mitigation” is requesting more callback gas, which does not guarantee fitting within block/provider caps.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `countTierMatchesWithBonusball` function in `TicketComboTracker` iterates from 1 to `bonusballMax`. For each bonusball, it iterates through all subsets of the normal balls (31 subsets for 5 balls) and performs storage reads on `comboCounts`. If `bonusballMax` reaches its maximum of 255 (which occurs automatically when the prize pool is large relative to ticket price), the loop performs approximately 7,905 storage reads. At 2,100 gas per cold SLOAD, this alone costs ~16.6 million gas, not counting overhead. This creates a high risk of exceeding the block gas limit (typically 30M on EVM chains) or the gas limit configured for the Pyth callback. If the required gas exceeds the limit, the `scaledEntropyCallback` will consistently revert. Since the drawing is locked (`jackpotLock = true`) until the callback succeeds, the protocol becomes permanently stuck in the current drawing, preventing new ticket purchases or LP withdrawals.

## Impact
Permanent freezing of the jackpot protocol (DoS); funds locked until emergency mode is activated.

## Command to Run Test


## Proof of Concept
1. The prize pool grows large enough (via LP deposits) such that `_setNewDrawingState` calculates `bonusballMax` close to 255.
2. The drawing concludes and `runJackpot` is called.
3. The entropy provider calls back `scaledEntropyCallback`.
4. `_calculateDrawingUserWinnings` calls `TicketComboTracker.countTierMatchesWithBonusball`.
5. The function attempts to iterate 255 times, accessing `comboCounts` mapping 31 times per iteration.
6. The transaction runs out of gas (OOG) because the cost exceeds the block gas limit or the `entropyGasLimit`.
7. The callback reverts, and the jackpot remains locked forever.

## Proof of Code


## Suggested Mitigation
Optimize `TicketComboTracker` to avoid iterating all bonusballs (e.g., by tracking which bonusballs have purchased tickets), or enforce a lower hard cap on `bonusballMax` (e.g., 100) to ensure settlement fits comfortably within block gas limits.


## [H-3]. Bit vector overflow corruption bricks ticket verification

## id: wD6ybhWg8nCr20kD-w32z

## Derived From Pattern/Invariant
StorageLayout

## Exploit Type
IntegerOverflow

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: Tickets are packed in `TicketComboTracker.insert` via `ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax)`. If `_bonusball + normalMax >= 256`, EVM `SHL` yields 0 and the bonus bit is lost. There is no check in `Jackpot._setNewDrawingState`, `TicketComboTracker.init`, or `buyTickets` that enforces `normalBallMax + bonusballMax < 256`. The pool-cap computation intends to respect a 255-bit vector, but it only caps *LP deposits*, while LP value (and thus next drawing’s `bonusballMax`) can grow via `lpEarnings` from ticket sales without any cap. Corrupted packed tickets can break `unpackTicket` (revert/out-of-bounds) and distort tier matching in `_calculateTicketTierId`, making tickets behave incorrectly or become unclaimable.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Tickets are stored as `uint256` bit vectors. The bonus ball is stored at bit index `normalBallMax + bonusball`. `Jackpot._setNewDrawingState` calculates `newBonusball` dynamically up to 255. There is no check ensuring `normalBallMax + newBonusball < 256`. 

If the prize pool grows large enough to demand a high `bonusballMax` (e.g., 230) and `normalBallMax` is moderate (e.g., 30), the sum exceeds 255. The shift `1 << (bonus + normal)` overflows to 0. The ticket is saved without a set bonus bit. `unpackTicket` fails or returns incorrect values, and `claimWinnings` fails because the bonus match check fails.

## Impact
Tickets purchased during high-value pots are corrupted and worthless. Users cannot claim winnings. Funds stuck.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` = 30. `prizePool` grows such that `newBonusball` = 230. 
2. Drawing starts. User buys ticket with bonus ball 230. 
3. `TicketComboTracker.insert` computes `1 << (230 + 30) = 1 << 260`. 
4. In Solidity, this shift results in 0. 
5. `packedTicket` has no bonus bit. 
6. User cannot win.

## Proof of Code
function testShiftOverflow() public { uint256 res = 1 << 260; assertEq(res, 0); }

## Suggested Mitigation
In `_setNewDrawingState` and `setNormalBallMax`, enforce `normalBallMax + bonusballMax < 256`. If the dynamic calculation requires more, cap the bonus ball count or the prize pool.


## [H-4]. Valid entropy callback after emergency refunds permanently locks winnings in contract

## id: yEkeXtPfQeS6NGm6mt02x

## Derived From Pattern/Invariant
EmergencyModeStateStuck

## Exploit Type
EmergencyModeStateStuck

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: `enableEmergencyMode` allows `emergencyRefundTickets` which burns NFTs and transfers refunds, but does not update `TicketComboTracker` state (no deletion) and does not adjust drawing accounting. Importantly, `Jackpot.scaledEntropyCallback` is *not* guarded by `noEmergencyMode`; it can execute while `emergencyMode==true` as long as `jackpotLock` is true and caller is `entropy`. If callback arrives after refunds, winner counting uses stale tracker counts (including refunded tickets), and `processDrawingSettlement` subtracts `drawingUserWinnings` from the LP pool accounting. Refunded/burned tickets cannot be claimed, so the portion of “winnings” attributed to them becomes stuck (not claimable by users, not credited back to LPs) and can also desynchronize accounting vs real USDC after refunds. This is a concrete, reachable state machine bug on the critical settlement path.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `enableEmergencyMode` function allows users to refund tickets via `emergencyRefundTickets`, which burns the NFT. However, `TicketComboTracker` (which tracks sold tickets for winner counting) does not support deletion and retains the refunded ticket counts. The `scaledEntropyCallback` function is not blocked by `noEmergencyMode` and can execute if the entropy provider eventually responds (e.g., after a delay that triggered the emergency).

If `scaledEntropyCallback` executes after users have refunded tickets:
1. It counts winners using `TicketComboTracker`, including the refunded tickets.
2. It calculates `userWinnings` for all winning tickets.
3. It calls `JackpotLPManager.processDrawingSettlement`, which subtracts the full `userWinnings` amount from the LP pool.

Since the refunded users have burned their NFTs, they cannot call `claimWinnings`. The portion of `userWinnings` corresponding to refunded tickets remains locked in the `Jackpot` contract balance, while the LP pool is debited for that amount. LPs suffer a loss equal to the unclaimed winnings.

## Impact
Permanent loss of funds for Liquidity Providers; funds become stuck in the Jackpot contract.

## Command to Run Test


## Proof of Concept
1. Owner enables `emergencyMode` due to entropy delay.
2. User with a potential winning ticket calls `emergencyRefundTickets`, receiving `ticketPrice` and burning the NFT.
3. Entropy provider finally calls `scaledEntropyCallback`.
4. Callback calculates winners. The user's refunded ticket matches.
5. Callback deducts the user's prize amount from `lpPoolTotal`.
6. The prize amount sits in the contract. User cannot claim (NFT burned). LP cannot withdraw (pool deducted).

## Proof of Code
function testEmergencyStuckFunds() public {
    // Setup emergency mode
    vm.prank(owner);
    jackpot.enableEmergencyMode();
    // User refunds
    vm.prank(user);
    jackpot.emergencyRefundTickets(userTickets);
    // Entropy arrives
    vm.prank(address(entropy));
    jackpot.scaledEntropyCallback(1, randomNumbers, "");
    // Check LP pool
    // LP pool reduced by winnings of refunded ticket
}

## Suggested Mitigation
Add the `noEmergencyMode` modifier to `scaledEntropyCallback` to prevent settlement during emergency. Alternatively, verify `jackpotNFT` existence/validity before counting winners in `TicketComboTracker` (requires logic change) or ensure `processDrawingSettlement` can account for refunded tickets.


## [M-5]. DoS of Jackpot Settlement due to excessive gas usage in `scaledEntropyCallback`

## id: _RaWw9PqTyPiu_7c-cC3g

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: The settlement logic is O(bonusballMax * 31) storage reads plus subset generation, and there is no hard cap ensuring callback gas stays within practical limits. If callback OOGs, the drawing remains locked, which is high impact (liveness failure). This can be induced by sufficiently large bonusballMax/prizePool growth, so it is not reliably 'not exploitable' or 'rare'.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function in `Jackpot.sol` triggers the winner calculation process. This calls `TicketComboTracker.countTierMatchesWithBonusball`, which iterates through every possible bonus ball (from 1 to `bonusballMax`) and every possible subset of winning numbers (32 subsets per bonus ball). 

Inside this loop, it accesses the `comboCounts` storage mapping. Storage reads are gas-expensive (2100 gas cold, 100 warm). As `bonusballMax` scales linearly with the prize pool to maintain LP edge, the number of storage reads increases. 

If `bonusballMax` reaches its theoretical max of 255, the loop performs ~8,160 storage reads, consuming ~17M gas just for reads (excluding overhead), which exceeds the block gas limit of many chains or the configured `entropyGasLimit`. If the callback runs out of gas, the drawing remains permanently locked (`jackpotLock` stays true), requiring emergency intervention.

## Impact
The jackpot drawing mechanism can become permanently stuck if the prize pool grows large enough, forcing the protocol into emergency mode and requiring manual intervention/refunds.

## Command to Run Test


## Proof of Concept
1. Prize pool grows, causing `bonusballMax` to be set to 255.
2. `runJackpot` is called. It calculates `entropyGasLimit` linearly.
3. `scaledEntropyCallback` executes.
4. `_countSubsetMatches` loops 255 times.
5. Inner loops execute 32 times per outer loop.
6. 8160 `SLOAD` operations occur.
7. Transaction reverts due to Out of Gas.
8. Jackpot remains locked.

## Proof of Code


## Suggested Mitigation
Optimize the winner counting logic to avoid iterating all bonus balls (e.g., only iterate the winning bonus ball bucket plus global buckets) or enforce a hard cap on `bonusballMax` that keeps gas usage well within block limits.


## [H-6]. LP Share Price Inflation Attack via First Depositor Manipulation

## id: QeRj0gyedYy6EWilMygKZ

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: Valid
### Finding Status Justification: Shares are minted on consolidation as `shares = deposit * 1e18 / drawingAccumulator[depositDrawingId]` (see `_consolidateDeposits`), with no check that shares > 0. `drawingAccumulator[d]` is updated in `processDrawingSettlement` as `(prevAcc * postDrawLpValue) / currentLP.lpPoolTotal` when `lpPoolTotal != 0`. If `lpPoolTotal` is made extremely small and `postDrawLpValue` becomes large (e.g., via large `lpEarnings` from ticket sales, which are not capped by `lpPoolCap`), the accumulator can become huge. Subsequent depositors can then receive 0 shares due to truncation, effectively donating USDC to existing share holders. There is no “virtual shares” offset or minimum deposit/shares guard, so this ERC4626-style inflation attack is feasible under realistic states (small pool after withdrawals/emergency, then high ticket volume).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The LP share pricing mechanism in `JackpotLPManager` uses an accumulator (`drawingAccumulator`) to determine the exchange rate between USDC deposits and LP shares. The accumulator for a drawing is calculated based on the LP pool's performance: `newAccumulator = (prevAccumulator * postDrawLpValue) / prevLpPoolTotal`. 

If the `lpPoolTotal` is small (e.g., 1 wei), an attacker can significantly inflate the accumulator by generating LP earnings (buying tickets) for that drawing. Since `postDrawLpValue` includes ticket earnings, the ratio `postDrawLpValue / prevLpPoolTotal` can become extremely large. 

Subsequent deposits by other users in the next drawing will be converted to shares using this inflated accumulator: `shares = deposit * PRECISE_UNIT / accumulator`. Due to integer division, if the accumulator is sufficiently large relative to the user's deposit, the user will receive 0 shares, effectively losing their deposit to the existing pool holder (the attacker).

## Impact
Theft of subsequent LP deposits. An attacker can steal funds from later depositors by owning 100% of the pool shares after inflating the price.

## Command to Run Test


## Proof of Concept
1. Attacker observes `lpPoolTotal` is low (e.g., after initialization or emergency withdrawal). 
2. Attacker deposits 1 wei of USDC. 
3. Attacker buys tickets worth 1000 USDC. This increases `lpEarnings` by ~900 USDC (assuming 10% referral). 
4. The drawing settles. `postDrawLpValue` ≈ 900 USDC. `prevLpPoolTotal` = 1 wei. 
5. `newAccumulator` becomes `1e18 * 900e6 / 1` = `9e26`. 
6. Victim deposits 100 USDC (`1e8` wei). 
7. Victim's shares = `1e8 * 1e18 / 9e26` = 0. 
8. Victim gets 0 shares; Attacker owns 100% of the pool including Victim's 100 USDC.

## Proof of Code
lpManager.processDeposit(drawingId, attacker, 1);
// ... generate earnings ...
jackpot.scaledEntropyCallback(...); // settlement
// accumulator is now huge
lpManager.processDeposit(nextDrawingId, victim, 100e6);
// victim gets 0 shares

## Suggested Mitigation
Enforce a minimum initial LP deposit amount in `initializeJackpot` and `processDeposit`, or implement a 'virtual shares' offset (like ERC4626 mitigations) to prevent extreme accumulator inflation with small pool sizes.


## [H-7]. Ticket bit-packing collision allows guaranteed bonus ball match for high bonusball values

## id: -3wgMc-LlnlsH-1RLeyuM

## Derived From Pattern/Invariant
IntegerMath

## Exploit Type
IntegerMath

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: The underlying collision condition is real: if `_bonusball + normalMax >= 256`, `1 << (_bonusball + normalMax)` becomes 0, so multiple “overflow-range” bonusball values collapse to the same encoded bonus portion (0). `_calculateTicketTierId` compares the shifted bonus portion (`>> (normalMax+1)`), so two different overflow bonusballs both compare equal (0), incorrectly counting as a bonus match. The report’s phrasing “guaranteed” is overstated (it’s guaranteed *conditional on the winning bonusball also being in the overflow range*), but the packing collision is a high-severity correctness/economic issue that can increase win probability and misallocate payouts.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `TicketComboTracker.sol`, the `insert` function packs ticket numbers into a `uint256` bit vector using the shift operation: `set |= 1 << (_bonusball + _tracker.normalMax)`. If `_bonusball + _tracker.normalMax` equals or exceeds 256, the shift operation in EVM (`1 << 256`) results in 0 due to overflow, effectively losing the bonus ball bit.

This condition is reachable if `normalBallMax` and `bonusballMax` sum to >= 256 (e.g., `normalMax`=35 and `bonusballMax`=221). When this occurs, any ticket with a bonus ball in the overflow range is stored without a bonus bit. If the winning number also falls in this range, it also lacks the bonus bit.

In `Jackpot._calculateTicketTierId`, the match logic `_ticketNumbers >> (_normalBallMax + 1)` compares the bonus parts. Since both are 0 (due to lost bits), they evaluate as a match. This allows users to buy any ticket in the overflow range and be guaranteed a bonus ball match if the winner is also in that range, effectively increasing the win probability from `1/N` to `1` for the bonus component.

## Impact
Broken game logic allows guaranteed bonus matches for high-difficulty drawings, significantly increasing payout liability and draining the prize pool.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is 35. `prizePool` grows such that `bonusballMax` becomes 221.
2. User buys a ticket with bonus ball 221. `35 + 221 = 256`.
3. `insert` executes `1 << 256`, which is 0. The packed ticket contains only normal bits.
4. Drawing executes. Winning bonus ball is 221. Winning packed ticket also has bonus bit 0.
5. User claims winnings. `_calculateTicketTierId` extracts bonus part: `0 >> 36` is 0. Winning part is 0. `0 == 0`, so it counts as a bonus match.

## Proof of Code
function testBitPackingOverflow() public {
    uint8 normalMax = 35;
    uint8 bonusball = 221;
    // Sum is 256
    uint256 packed = 1 << (uint256(bonusball) + normalMax);
    // packed is 0
    assertEq(packed, 0);
}

## Suggested Mitigation
Modify `TicketComboTracker` to use a `BitMap` or multiple `uint256` slots if the combined bit range exceeds 256, or enforce a hard cap such that `normalBallMax + bonusballMax < 256` (though this limits the prize pool scaling).


## [H-8]. TicketComboTracker bitwise overflow corrupts tickets

## id: dTaLSqwaiB9FSt09lb326

## Derived From Pattern/Invariant
StorageLayout

## Exploit Type
IntegerMath

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: Same root cause exists in `TicketComboTracker.insert`: `1 << (_bonusball + _tracker.normalMax)` yields 0 for shift >=256. This corrupts packed tickets by losing the bonus bit. Additionally, `TicketComboTracker.unpackTicket` assumes exactly one bonus bit exists; if it is lost, `popCount` becomes 5 and it allocates `normalBalls` length 4, then writes 5 normals and will revert out-of-bounds. This makes `Jackpot.getUnpackedTicket` and `JackpotTicketNFT.getExtendedTicketInfo` revert for corrupted tickets, and also breaks tier computations/bonus matching. No invariant enforcement exists to prevent sum overflow.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library stores tickets as `uint256` bit vectors. The bonusball bit is set at index `_bonusball + _tracker.normalMax`. The EVM `SHL` opcode (used via `1 << index`) produces 0 if the shift amount is >= 256. 

If `normalBallMax + bonusballMax >= 256`, tickets with high bonusball numbers will attempt to set a bit at index >= 256. This results in the bit being lost (0). Both the user's ticket and the winning ticket (if strictly high) will store 0 for the bonusball bit. This results in these tickets effectively having 'bonusball 0', which is not a valid selection. Furthermore, `unpackTicket` will return corrupted data (underflowing `fls - normalMax`). This occurs if the pool grows large enough to push `bonusballMax` > 230 (assuming `normalMax`=25).

## Impact
Ticket data corruption leading to incorrect win/loss determination. Tickets with high bonusballs effectively become 'wildcards' that match other corrupted tickets, increasing win probability unfairly.

## Command to Run Test


## Proof of Concept
1. Pool grows such that `bonusballMax` is set to 255 and `normalBallMax` is 5. Sum is 260.
2. User buys ticket with bonusball 255.
3. `insert` calculates shift `5 + 255 = 260`. `1 << 260` is 0.
4. Ticket stored with only normal bits set.
5. Winning numbers drawn with bonusball 255.
6. Winning ticket stored with only normal bits set.
7. Comparison `ticket & winning` matches perfectly (ignoring bonusball difference). User wins despite collision logic.

## Proof of Code
function testBitOverflow() public {
    uint256 normalMax = 5;
    uint256 bonusball = 255;
    uint256 shift = normalMax + bonusball;
    uint256 vector = 1 << shift;
    assertEq(vector, 0);
}

## Suggested Mitigation
Enforce `normalBallMax + bonusballMax < 256` in `Jackpot._setNewDrawingState` and `Jackpot.setNormalBallMax`.


## [H-9]. Broken ticket encoding causes incorrect payouts when total ball range exceeds 256

## id: nYCKARv0is4af6PBSDgxB

## Derived From Pattern/Invariant
IntegerMath

## Exploit Type
IntegerMath

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: The packing scheme stores normals in bits `1..normalMax` and bonus at `normalMax+bonusball`. When `normalMax+bonusball >= 256`, the bonus bit is lost and the extracted bonus portion in `_calculateTicketTierId` becomes 0. If the winning ticket’s bonus is also in the overflow range, the equality check treats them as matching, leading to incorrect tier assignment/payouts. The system lacks an explicit constraint enforcing `normalBallMax + bonusballMax < 256`, so this failure mode is reachable once parameters drift beyond the 256-bit encoding space.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library packs ticket numbers into a single `uint256` bit vector. The bonusball is stored at bit position `_bonusball + _tracker.normalMax`. The bit setting logic is `set |= 1 << (_bonusball + _tracker.normalMax)`. 
In Solidity 0.8+, the `<<` operator pushes 0 if the shift amount is >= 256. If `normalBallMax + bonusballMax` exceeds 255, tickets with high bonusball values will have their bonus bit shifted out (resulting in 0). 
Subsequently, `_calculateTicketTierId` in `Jackpot` attempts to retrieve the bonusball via `_ticketNumbers >> (_normalBallMax + 1)`. If the bit was lost, this evaluates to 0. Since the winning ticket is also subject to the same overflow, a user picking a high bonusball (resulting in 0) will match a winning ticket with a high bonusball (resulting in 0), incorrectly awarding a bonus match.

## Impact
Game logic breaks for high ball ranges; unrelated bonusballs match each other, leading to incorrect prize payouts and potential fund drainage.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 100.
2. `bonusballMax` dynamically scales to 160 (Sum = 260).
3. User buys ticket with bonusball 160. `shift` = 260. `1 << 260` = 0. Packed ticket has no bonus bit.
4. Winning number is bonusball 160. Packed winner has no bonus bit.
5. Tier calculation compares `0 == 0`. Match! User wins bonus tier despite potential logic breakdown.

## Proof of Code
function testBitShiftOverflow() public {
    uint256 normalMax = 100;
    uint256 bonus = 160;
    uint256 packed = 1 << (normalMax + bonus);
    assertEq(packed, 0);
}

## Suggested Mitigation
Ensure `normalBallMax + bonusballMax < 256` in `_setNewDrawingState` and `setNormalBallMax`. Revert if the configuration violates this constraint.


## [H-10]. Difficulty parameter truncation allows trivial jackpot wins and LP draining

## id: p-4TGk45owF0Idt-xY7Mu

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Not inherently rare: bonusballMax is computed from prizePool/newLpValue and then unsafely cast to uint8. Since lpPoolTotal can grow via uncapped lpEarnings (not bounded by lpPoolCap), reaching a computed value >255 is plausible over time for a successful protocol, making wraparound a realistic failure mode.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the new `bonusballMax` is calculated to maintain the LP edge based on the prize pool size. The calculation `Math.ceilDiv(minNumberTickets, combosPerBonusball)` returns a `uint256` which is explicitly cast to `uint8`. If the required number of bonus balls exceeds 255 (which occurs with large prize pools or low `normalBallMax`), the value is truncated (modulo 256). 

For example, if the math requires 4409 bonus balls to secure the pool, casting to `uint8` results in 57. The difficulty becomes ~77x lower than required. Attackers can buy all combinations at a fraction of the intended cost, statistically guaranteeing a win that drains the LP pool.

## Impact
Collapse of LP pool solvency due to drastically lowered win difficulty.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 10 (252 combos). 
2. LPs deposit to reach 1M USDC prize pool. 
3. `runJackpot` executes. `_setNewDrawingState` calculates `minNumberTickets` approx 1.1M. 
4. `1.1M / 252` approx 4409. 
5. `uint8(4409)` becomes 57. 
6. New drawing difficulty is 1 in ~14k instead of 1 in ~1.1M. 
7. Attacker buys all tickets for $14k, wins $1M.

## Proof of Code
function testTruncation() public { uint256 val = 4409; uint8 truncated = uint8(val); assertEq(truncated, 57); }

## Suggested Mitigation
Ensure `bonusballMax` calculation does not exceed 255. If the required difficulty exceeds 255, cap the prize pool or revert/cap `bonusballMax` (though capping `bonusballMax` implies accepting lower edge). Ideally, use a larger type for `bonusball` or limit pool growth.


## [H-11]. Catastrophic LP edge collapse due to uint8 casting truncation in bonusball calculation

## id: 08TiopoJlp8SF5acStTTt

## Derived From Pattern/Invariant
IntegerMath

## Exploit Type
IntegerMath

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Same issue as the previous finding: because lpPoolTotal/prizePool growth via lpEarnings is not capped by lpPoolCap, reaching a computed bonusball requirement >255 is a plausible state, so the wrap/truncation scenario is not necessarily rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, the `_setNewDrawingState` function dynamically calculates the `newBonusball` (difficulty parameter) to ensure a sufficient number of combinations for the Liquidity Provider (LP) edge based on the prize pool size. The calculation `Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball))` is performed in `uint256`, but the result is explicitly cast to `uint8`.

As the prize pool grows (e.g., exceeding ~75M USDC with standard parameters), the required `newBonusball` value can exceed 255. Due to the explicit cast `uint8(...)`, the value wraps around (e.g., 257 becomes 1). This drastically reduces the game difficulty instead of increasing it, causing the statistical Expected Value (EV) for players to potentially exceed 1.0 significantly, leading to the rapid drainage of the LP pool.

## Impact
LPs suffer massive losses as the game difficulty collapses to trivial levels for large prize pools, effectively subsidizing player winnings.

## Command to Run Test


## Proof of Concept
1. Assume `normalBallMax`=35 (combos=324,632), `ticketPrice`=1e6 (1 USDC), `lpEdgeTarget`=0.25 (25%).
2. The system calculates `minNumberTickets` to guarantee edge. If `prizePool` grows to ~75M USDC, `minNumberTickets` becomes ~83M.
3. `ceilDiv(83M, 324,632)` results in 257.
4. `uint8(257)` results in 1.
5. The new drawing is initialized with `bonusballMax` = 1 (or `bonusballMin`).
6. Difficulty is 257x lower than required. Players buy tickets with massive +EV, draining the pool.

## Proof of Code
function testBonusballTruncation() public {
    uint256 largePrizePool = 75_000_000 * 1e6; // 75M USDC
    uint256 ticketPrice = 1e6;
    uint256 lpEdgeTarget = 25 * 1e16; // 25%
    uint256 normalBallMax = 35;
    uint256 combos = 324632;
    
    uint256 minNumberTickets = largePrizePool * 1e18 / ((1e18 - lpEdgeTarget) * ticketPrice);
    uint256 requiredBonusball = (minNumberTickets + combos - 1) / combos;
    
    // requiredBonusball is 257
    uint8 truncated = uint8(requiredBonusball);
    // truncated is 1
    assertEq(requiredBonusball, 257);
    assertEq(truncated, 1);
}

## Suggested Mitigation
Increase the size of `bonusballMax` and related storage to `uint16` or ensure `newBonusball` is capped at 255 (though capping violates the edge guarantee, so storage expansion is preferred).


## [M-12]. DoS and Logic Corruption via Uncapped Ticket Sales (Bitpacking Overflow)

## id: dwNMklUYa63-oqBQhHwgX

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Core claim is correct: ticket encoding and winner logic rely on a 256-bit vector, but neither `Jackpot._setNewDrawingState` nor `TicketComboTracker` enforces `normalBallMax + bonusballMax < 256`. Although `_calculateLpPoolCap` is designed around `255 - normalBallMax`, it only caps LP deposits. `lpEarnings` from ticket sales are not capped and are included in `newLpValue` at settlement, so the next drawing can be initialized with unsafe `bonusballMax` values. This can produce (a) encoding collisions/lost bonus bits (`1 << (bonus+normalMax)` becomes 0) and (b) difficulty truncation via the unsafe `uint8` cast in `_setNewDrawingState`. The finding’s mention of `UintCasts.toUint8` reverting is inaccurate (the code uses an unsafe cast), but the underlying overflow/logic-corruption risk exists.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol relies on bitpacking in `TicketComboTracker` to store ticket combinations. The bonus ball is stored at bit index `normalBallMax + bonusball`. This index must not exceed 255. While `normalBallMax` is capped at 128, `bonusballMax` is calculated dynamically based on the prize pool size to ensure LP edge. 

The `lpPoolCap` mechanism limits *deposits* to keep the potential pool size safe, but it does not limit *earnings* from ticket sales. If ticket sales drive the `prizePool` sufficiently high (bypassing the cap logic), the calculated `bonusballMax` for the next drawing can exceed `255 - normalBallMax`. This results in either: 
1. `UintCasts.toUint8` reverting (if > 255), causing the entropy callback to revert and the jackpot to be stuck (DoS). 
2. Bitpacking overflow (if between 256-normalMax and 255), where the bonus ball bit wraps around or is lost, corrupting ticket validation logic.

## Impact
Jackpot settlement failure (DoS) requiring emergency mode, or corruption of game logic leading to incorrect payouts.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 128. Safe `bonusballMax` is 127. 
2. Users buy a massive amount of tickets, pushing `lpEarnings` and thus `prizePool` to a level where the calculated `newBonusball` is 150. 
3. The current drawing settles. `_setNewDrawingState` attempts to cast 150 to uint8 (ok) and init tracker. 
4. `TicketComboTracker` tries to shift `1 << (150 + 128)` = `1 << 278` which overflows 256 bits, resulting in 0. 
5. Winning ticket is stored with NO bonus ball bit. Payouts are corrupted.

## Proof of Code
// Simulate huge earnings
vm.store(address(jackpot), slot_lpEarnings, hugeValue);
jackpot.scaledEntropyCallback(...);
// Reverts or corrupts state

## Suggested Mitigation
Enforce `lpPoolCap` (or a derived earnings cap) during `buyTickets`. If the projected prize pool exceeds the safety limit for the current `normalBallMax` configuration, revert ticket purchases.


## [H-13]. Unsafe Downcast of `bonusballMax` leads to critical game parameter corruption

## id: Jqa6zhv4VVvy-cmU9HyHy

## Derived From Pattern/Invariant
CastingOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Unsafe downcast-triggering states are plausibly reachable as the protocol scales because lpEarnings can grow lpPoolTotal beyond lpPoolCap over time, which can push the computed bonusball requirement above 255 and cause deterministic wraparound.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the new bonus ball upper bound is calculated and explicitly cast to `uint8`: `newDrawingState.bonusballMax = uint8(...)`. The calculation `Math.ceilDiv(minNumberTickets, combosPerBonusball)` depends on `minNumberTickets`, which scales linearly with the prize pool. Since `lpEarnings` bypass the `lpPoolCap`, the prize pool can grow arbitrarily large. If the calculated value exceeds 255, the explicit `uint8` cast will truncate the value (wrap modulo 256). 

For example, if the calculation yields 257, `bonusballMax` becomes 1. This drastically reduces the game difficulty, destroying the intended LP edge and potentially causing the prize pool to be drained rapidly by users having near-guaranteed wins.

## Impact
High. The economic security of the protocol (LP edge) is destroyed. LPs will suffer guaranteed losses as the game becomes trivially easy to win compared to the ticket price/prize ratio.

## Command to Run Test


## Proof of Concept
1. `prizePool` grows large enough such that `minNumberTickets / combos` results in 257.
2. `_setNewDrawingState` casts `257` to `uint8`, resulting in `1`.
3. The new drawing is initialized with `bonusballMax = 1`.
4. Users can now buy tickets where the bonus ball match is guaranteed (100% probability), while the payouts are calculated assuming a much lower probability.
5. Users extract value from the LP pool.

## Proof of Code
function testBonusBallTruncation() public {
    uint256 calculatedBonus = 257;
    uint8 truncated = uint8(calculatedBonus);
    assertEq(truncated, 1, "Truncation failed");
}

## Suggested Mitigation
Use `UintCasts.toUint8` (which reverts on overflow) instead of explicit casting, or cap the value at 255 before casting.





Finding Status: InvalidGovernanceRisk
## [M-14]. DoS of Jackpot execution due to excessive gas limit configuration

## id: wKu0UVgtuJYHoH7y5Zy0x

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.runJackpot

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Jackpot.runJackpot computes entropyGasLimit as `entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax`, and the constructor sets entropyVariableGasLimit to 250,000. With sufficiently large bonusballMax, the requested gas limit can exceed realistic per-tx/block limits, making Pyth fulfillment infeasible and leaving the jackpot locked. The code has no cap or sanity check against block gas constraints or the provider’s max. While the owner can adjust entropyVariableGasLimit via setEntropyVariableGasLimit, failure to do so (or setting it incorrectly) is a governance/configuration risk that can brick progression once bonusballMax grows.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract calculates the gas limit for the entropy callback as `entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax`. The default `entropyVariableGasLimit` is set to 250,000 in the constructor. If `bonusballMax` grows large (e.g., > 120), the calculated gas limit exceeds the Ethereum block gas limit (30M). 

While the `runJackpot` function will succeed (as it only requests entropy), the Pyth network provider will fail to execute the callback due to the impossible gas limit. This leaves the jackpot in a locked state (`jackpotLock = true`), preventing new ticket purchases or progression. Recovery requires Emergency Mode, disrupting the protocol.

## Impact
The Jackpot protocol becomes permanently stuck if the pool grows large enough to require high difficulty, necessitating emergency withdrawal and protocol restart.

## Command to Run Test


## Proof of Concept
1. `bonusballMax` scales up to 125 due to pool growth.
2. Keeper calls `runJackpot`. `entropyGasLimit` = Base + 250,000 * 125 = ~31.25M.
3. Request is submitted to Pyth.
4. Pyth attempts to fulfill callback but transaction requires > 30M gas.
5. Callback reverts or cannot be included in a block.
6. Jackpot remains locked indefinitely.

## Proof of Code
function testGasDos() public {
    uint32 variable = 250000;
    uint8 bonusMax = 125;
    uint32 limit = variable * uint32(bonusMax);
    assertGt(uint256(limit), 30_000_000);
}

## Suggested Mitigation
Reduce `entropyVariableGasLimit` to a realistic value (e.g., 20k-50k) reflecting the actual incremental cost of processing one bonusball, and ensure the total never exceeds the block gas limit.


## [H-15]. Emergency refund drains contract if referral fee lowered

## id: ASGh6Bleq5loZThmVtc6a

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot.emergencyRefundTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Impact is not low: changing referralFee after ticket purchase can cause over-refunds (relative to funds retained after referral payouts), potentially exhausting liquidity and preventing other users/LPs from receiving refunds/withdrawals. It is governance risk because it requires owner parameter changes.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `emergencyRefundTickets` function calculates the refund amount as `ticketPrice * (1 - referralFee)`. It uses the *current* `referralFee` state variable. However, the ticket purchase distributed the *historical* `referralFee` to referrers. 

If the owner lowers the `referralFee` (e.g., from 10% to 0%) after tickets are bought but before an emergency refund, the contract will attempt to refund `ticketPrice * (1 - 0) = 100%`. Since 10% was already paid out to referrers, the contract only holds 90% of the funds. Refunding 100% drains the contract's remaining liquidity, potentially preventing other users from refunding or withdrawing.

## Impact
Insolvency of the Jackpot contract during emergency mode, causing loss of funds for remaining users or LPs.

## Command to Run Test


## Proof of Concept
1. `referralFee` is 20%. Ticket price 100.
2. User buys ticket. 20 sent to referrer, 80 to `lpEarnings`. Contract holds 80.
3. Owner changes `referralFee` to 0%.
4. Emergency mode enabled.
5. User calls `emergencyRefundTickets`. Refund calc: `100 * (1 - 0) = 100`.
6. Contract transfers 100 to user, but only holds 80 from that ticket.
7. Contract balance decreases by 20 more than expected, stealing from other deposits.

## Proof of Code
function testRefundDrain() public {
    // Mock setup
    uint256 price = 100;
    uint256 fee = 20; // 20%
    uint256 held = price - fee;
    // Admin change
    uint256 newFee = 0;
    uint256 refund = price * (100 - newFee) / 100;
    assertGt(refund, held);
}

## Suggested Mitigation
Store the `referralFee` used for each drawing in `DrawingState` and use that value during refunds, or store the specific fee paid per ticket.


## [H-16]. Replacing PayoutCalculator freezes all unclaimed winnings

## id: FSrzFhMnh8jlpjVar1YKx

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
UpgradeabilityInitializerSafety

## Location
Jackpot.setPayoutCalculator

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Impact is not low: switching to a fresh calculator can cause past-drawing claims to pay 0 while still burning tickets, permanently destroying user winnings. It is governance risk because onlyOwner must perform the change.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `Jackpot` contract allows the owner to update the `payoutCalculator` contract via `setPayoutCalculator`. The `claimWinnings` function delegates payout lookup to the *current* `payoutCalculator`. The `GuaranteedMinimumPayoutCalculator` stores snapshot data (`drawingTierInfo`, `tierPayouts`) internally. 

If the calculator is replaced, the new instance has no record of previous drawings. Calls to `getTierPayout` return 0. Users attempting to claim winnings from drawings settled under the old calculator will receive 0 USDC and have their tickets burned.

## Impact
Permanent loss of all unclaimed winnings from prior drawings upon calculator upgrade.

## Command to Run Test


## Proof of Concept
1. Users win in Drawing 1. 
2. Owner upgrades PayoutCalculator to fix a rate. 
3. User calls `claimWinnings` for Drawing 1. 
4. `Jackpot` calls `newCalc.getTierPayout(1, ...)` which returns 0. 
5. Ticket burned, 0 USDC transferred.

## Proof of Code


## Suggested Mitigation
Either make `payoutCalculator` immutable, or require data migration, or modify `Jackpot` to store which calculator controlled which drawing.


## [M-17]. Referral arbitrage allows risk-free draining of LP pool

## id: FBinLgGt7Dy0ARfw7bYqs

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: If misconfigured (e.g., referralFee > lpEdgeTarget with self-referrals allowed), the economic impact can be material and can erode/drain LP value over time; it is not low impact. It is governance risk because the condition depends on admin-set parameters.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The protocol allows `referralFee` and `lpEdgeTarget` to be set independently. Duplicate tickets increase the prize pool by `ticketPrice * (1 - lpEdgeTarget)` (maintaining LP edge theoretically) but cost the user `ticketPrice`. However, users can refer themselves. 

If `referralFee > lpEdgeTarget`, the cost to the user (`Price * (1 - referralFee)`) is less than the value added to the pot (`Price * (1 - lpEdgeTarget)`). An attacker can buy duplicate tickets for the same winning combination (or all combinations). The liability to the LP pool increases more than the revenue net of refunds, effectively draining LP equity.

## Impact
Direct leakage of LP funds to arbitrageurs.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` 20%, `lpEdgeTarget` 10%. 
2. Attacker buys duplicate ticket for $1. 
3. Attacker gets $0.20 referral. Net cost $0.80. 
4. Prize Pool increases by $0.90 ($1 - 10%). 
5. If attacker holds the winning ticket, they claim the $0.90 increase. 
6. Profit $0.10. LP loses $0.10.

## Proof of Code


## Suggested Mitigation
Enforce `referralFee <= lpEdgeTarget` in administrative setters.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-18]. LP Pool DoS via Zero Accumulator when pool value is drained

## id: 2y0q6XIpq0BdgnRlIrqGz

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: `JackpotLPManager.processDrawingSettlement` can set `drawingAccumulator[_drawingId]` to 0 if `postDrawLpValue == 0` while `currentLP.lpPoolTotal != 0` (since it directly computes `(prevAcc * postDrawLpValue) / lpPoolTotal`). A zero accumulator bricks `_consolidateDeposits` / `_consolidateWithdrawals` (division by zero). Under typical parameters, `postDrawLpValue` should stay >0 because ticket revenue exists when there are winners, but the code does not enforce this and governance can configure edge cases (e.g., `referralFee == 100%` so `lpEarnings` can be 0 even with ticket sales; `reserveRatio == 0` so prizePool can equal lpPoolTotal; then a full prize win can yield `postDrawLpValue==0`). Because this requires parameter choices by the trusted owner and a particular draw outcome, it is a rare governance footgun, but the zero-accumulator state is real and unhandled.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager`, the `drawingAccumulator` is updated in `processDrawingSettlement` using the formula `newAccumulator = (drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal`. If `postDrawLpValue` drops to 0 (which is possible if `reserveRatio` is 0 or low, and winnings + fees consume the entire pool + earnings), `newAccumulator` becomes 0. 

If the accumulator for a drawing becomes 0, any subsequent call to `_consolidateDeposits` for deposits made in that drawing will revert due to division by zero (`shares = amount * PRECISE_UNIT / accumulator`). This permanently bricks the LP positions of any user who deposited during that specific drawing, locking their funds.

## Impact
Medium. Liquidity Providers can have their funds permanently locked if a specific drawing settles with zero value.

## Command to Run Test


## Proof of Concept
1. `reserveRatio` is set to 0 (allowed).
2. A drawing occurs where `lpEarnings` are low and a user wins the entire `prizePool` (which equals `lpPoolTotal` since reserve is 0).
3. `postDrawLpValue` becomes 0.
4. `processDrawingSettlement` sets `drawingAccumulator[id]` to 0.
5. An LP who deposited during this drawing later calls `lpDeposit` or `initiateWithdraw`.
6. `_consolidateDeposits` attempts to calculate shares using `drawingAccumulator[id]` (which is 0).
7. Transaction reverts due to division by zero.

## Proof of Code
function testZeroAccumulator() public {
    uint256 amount = 100;
    uint256 accum = 0;
    // Simulates _consolidateDeposits logic
    vm.expectRevert();
    uint256 shares = (amount * 1e18) / accum;
}

## Suggested Mitigation
In `processDrawingSettlement`, ensure `newAccumulator` has a minimum value (e.g., 1) or handle the zero case in `_consolidateDeposits` by issuing 0 shares (though 0 shares for a deposit is also problematic, a minimum nonzero floor for `postDrawLpValue` or `accumulator` is preferred).



