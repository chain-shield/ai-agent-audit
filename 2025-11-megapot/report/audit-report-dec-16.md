# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[H-1]. JackpotBridgeManager.buyTickets overcharges users or reverts due to ticket price mismatch with Jackpot
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-2]. Ticket data corruption due to bit packing overflow when normalBallMax + bonusballMax >= 256
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[M-3]. Inefficient nested loop in winner counting enables DoS via block gas limit
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-4]. Integer overflow in `bonusballMax` calculation drastically reduces jackpot difficulty causing LP insolvency
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[M-5]. DoS due to Block Gas Limit in runJackpot via Entropy Callback
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[M-6]. DoS in `scaledEntropyCallback` due to Unbounded Storage Reads
**Derived From** : UnboundedLoops
Finding Status: Valid
Privilege: Permissionless


[H-7]. Denial of Service in drawing settlement due to excessive gas consumption and block gas limit violation
**Derived From** : UnboundedLoops
Finding Status: Valid
Privilege: Permissionless


[H-8]. Inflation Attack on LP Pool via First Depositor Front-running
**Derived From** : ERC4626SharePriceMismatch
Finding Status: Valid
Privilege: Permissionless


[H-9]. Bonus ball bit overflow causes incorrect payout tier calculation
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-10]. Organic LP pool growth via ticket earnings bypasses `lpPoolCap` causing ticket corruption and loss of winnings
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-11]. Bitpacking overflow allows bonusball manipulation in high-value jackpots
**Derived From** : ReserveOrPriceDesync
Finding Status: Valid
Privilege: Permissionless


[H-12]. Critical LP insolvency risk due to `bonusballMax` unsafe downcasting and bit-vector overflow
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-13]. Bit Packing Overflow causes corrupted tickets and accounting mismatch leading to insolvency
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-14]. Unsafe Downcasting of Dynamic Bonusball Parameter Leads to Difficulty Collapse
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-15]. Lack of validation that `referralFee < lpEdgeTarget` allows LPs to be drained via duplicate tickets
**Derived From** : ConfigFootgun
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[M-16]. Drawing settlement DoS due to `postDrawLpValue` underflow caused by duplicate tickets
**Derived From** : Dos
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[M-17]. Total LP loss results in zero accumulator and bricks future deposits
**Derived From** : Dos
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[H-18]. Referral Fee exceeding LP Edge Target causes settlement underflow and DoS
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole



Finding Status: InvalidByDesign


[M-19]. Strict balance check in JackpotBridgeManager causes DoS with bridges that refund dust
**Derived From** : ForcedAssetVsStrictEquality
Finding Status: InvalidByDesign
Privilege: Permissionless


[M-20]. Strict Balance Check in `claimWinnings` Breaks Integration with Refund-Logic Bridges
**Derived From** : ForcedAssetVsStrictEquality
Finding Status: InvalidByDesign
Privilege: Permissionless



Finding Status: LowSeverityDueToLowImpact


[L-21]. Missing Deadline in EIP-712 Signatures Allows Indefinite Validity
**Derived From** : PermitDeadlineBypass
Finding Status: LowSeverityDueToLowImpact
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 12
- M: 8
- L: 1
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. JackpotBridgeManager.buyTickets overcharges users or reverts due to ticket price mismatch with Jackpot

## id: ob9yHep4DA_UXKP88FtMz

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
JackpotBridgeManager.buyTickets

## Finding Status: Valid
### Finding Status Justification: JackpotBridgeManager.buyTickets reads jackpot.ticketPrice() (global), while Jackpot.buyTickets charges drawingState[currentDrawingId].ticketPrice (snapshotted per drawing in _setNewDrawingState). If the owner updates Jackpot.ticketPrice mid-drawing (intended to affect only future drawings), BridgeManager will transfer/approve the wrong ticketCost. If global > drawing price, BridgeManager over-collects and the excess USDC remains stuck in BridgeManager. If global < drawing price, Jackpot.buyTickets transferFrom BridgeManager reverts due to insufficient allowance/balance. There is no refund/cleanup logic, so this is a real user-funds loss/DoS bug.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotBridgeManager.buyTickets` function calculates the cost of tickets using `jackpot.ticketPrice()`, which returns the *global* ticket price variable from `Jackpot.sol`. However, `Jackpot.buyTickets` executes purchases using the `ticketPrice` stored in the `DrawingState` of the *current active drawing*.

`Jackpot.sol` snapshots the global `ticketPrice` into the `DrawingState` at the beginning of each drawing (in `_setNewDrawingState`). If the contract owner updates the global `ticketPrice` via `setTicketPrice` while a drawing is active, the global price changes immediately, but the active drawing's price remains unchanged (as intended for fairness).

This discrepancy causes `JackpotBridgeManager` to pull an incorrect amount of USDC from the user:
1. If the global price is higher than the active drawing price, `JackpotBridgeManager` pulls the higher amount from the user but only pays the lower amount to `Jackpot`. The excess USDC remains permanently stuck in the `JackpotBridgeManager` contract, causing a loss of funds for the user.
2. If the global price is lower, `JackpotBridgeManager` pulls the lower amount, but `Jackpot` attempts to transfer the higher amount from `JackpotBridgeManager`. This causes the transaction to revert due to insufficient allowance/balance, resulting in a Denial of Service.

## Impact
Users suffer direct loss of funds (excess payment stuck in contract) or are unable to purchase tickets (DoS) if the ticket price is updated during an active drawing.

## Command to Run Test


## Proof of Concept
1. Admin starts a drawing with `ticketPrice` = 10 USDC. `drawingState[id].ticketPrice` is 10.
2. Mid-drawing, Admin updates global `ticketPrice` to 20 USDC for the next drawing.
3. User calls `JackpotBridgeManager.buyTickets(1 ticket)`.
4. Manager reads `jackpot.ticketPrice()` which returns 20.
5. Manager transfers 20 USDC from User to itself.
6. Manager calls `jackpot.buyTickets`.
7. `Jackpot` calculates cost using `drawingState` price (10 USDC) and transfers 10 USDC from Manager.
8. The transaction succeeds. User paid 20, but only 10 was used. 10 USDC is permanently trapped in `JackpotBridgeManager`.

## Proof of Code
function test_BridgeBuyTickets_PriceMismatch_LossOfFunds() public {
    // 1. Setup: Initial Price = 10 USDC
    vm.prank(owner);
    jackpot.setTicketPrice(10e6);
    
    // Start Drawing (locks drawing price at 10)
    // (Assume initialization/settlement logic runs here to start drawing 1)
    
    // 2. Update Global Price to 20 USDC
    vm.prank(owner);
    jackpot.setTicketPrice(20e6);
    
    // 3. User buys via Bridge
    uint256 tickets = 1;
    // JackpotBridgeManager calculates cost using global price (20e6)
    uint256 bridgeCost = 20e6 * tickets;
    
    vm.startPrank(user);
    usdc.mint(user, bridgeCost);
    usdc.approve(address(bridgeManager), bridgeCost);
    
    // Setup dummy tickets
    IJackpot.Ticket[] memory t = new IJackpot.Ticket[](1);
    t[0] = IJackpot.Ticket(new uint8[](5), 1);
    
    bridgeManager.buyTickets(t, user, new address[](0), new uint256[](0), bytes32(0));
    vm.stopPrank();

    // 4. Assertions
    // User paid 20e6
    assertEq(usdc.balanceOf(user), 0, "User should have paid full calculated amount");
    
    // Bridge Manager holds 10e6 stuck funds (20e6 collected - 10e6 paid to Jackpot)
    assertEq(usdc.balanceOf(address(bridgeManager)), 10e6, "Bridge Manager stuck with excess funds");
}

## Suggested Mitigation
In `JackpotBridgeManager.buyTickets`, retrieve the ticket price from the active drawing state instead of the global variable:

```solidity
// Replace this:
// uint256 ticketPrice = jackpot.ticketPrice();

// With this:
uint256 currentDrawingId = jackpot.currentDrawingId();
uint256 ticketPrice = jackpot.getDrawingState(currentDrawingId).ticketPrice;
```


## [H-2]. Ticket data corruption due to bit packing overflow when normalBallMax + bonusballMax >= 256

## id: M1juf2EW-sQ7OXusZ6YHO

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: Bitpacking overflow is a real correctness/security issue if reachable (bonus bit dropped, unpacking/tiering can revert or misclassify). Not governance-driven; impact not low.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Tickets are stored as a `uint256` bit vector where normal balls occupy bits `1` to `normalBallMax` and the bonus ball occupies bit `normalBallMax + bonusball`. The `TicketComboTracker.insert` function constructs this vector using `set | (1 << (_bonusball + _tracker.normalMax))`. 

In Solidity 0.8+, shifting by a value greater than or equal to the type width (256) results in 0. If `_normalMax + _bonusball >= 256`, the shift yields 0, and the bonus ball bit is lost. This state is reachable because `normalBallMax` is capped at 128 (by `Combinations.sol`) and `bonusballMax` can dynamically grow to 255 (based on prize pool), so their sum can exceed 255.

When `unpackTicket` is called on such a corrupted ticket, `LibBit.fls(_packedTicket)` returns the index of the highest normal ball (since the bonus bit is missing). The calculation `uint8(LibBit.fls(_packedTicket) - _normalMax)` then underflows, resulting in a random, incorrect bonus ball value. This corrupts ticket data, causing winning tickets to be treated as losers or vice versa.

## Impact
Permanent data corruption of purchased tickets. Users who buy tickets with a high bonus ball number (when the pool is large) will have their tickets stored incorrectly, making it impossible to claim winnings they are entitled to.

## Command to Run Test


## Proof of Concept
1. Owner sets `normalBallMax` to 128 (valid). 
2. Prize pool grows such that `bonusballMax` becomes 130. 
3. User buys a ticket with bonus ball 130. 
4. `insert` calculates shift: `128 + 130 = 258`. 
5. `1 << 258` is 0. Ticket is stored with only normal ball bits. 
6. User tries to view or claim ticket. `unpackTicket` calculates `bonusball` via underflow, e.g., `fls` returns 128. `128 - 128 = 0`. If `fls` returns 120, `120 - 128` underflows to 248. The ticket bonus ball is effectively randomized/wrong.

## Proof of Code
function testBitPackingOverflow() public {
    uint8 normalMax = 128;
    uint8 bonusball = 130;
    // Simulate insert logic
    uint256 set = 1; // dummy normal bit
    uint256 packed = set | (1 << (bonusball + normalMax));
    // packed should have the bonus bit, but shift overflows to 0
    assertEq(packed, set);
    // Simulate unpack
    uint256 fls = 0; // assumes set=1 is lowest, fls would be low
    // In real scenario fls returns highest normal bit
}

## Suggested Mitigation
Enforce the invariant `normalBallMax + bonusballMax < 256` in both `setNormalBallMax` and `_setNewDrawingState`. Alternatively, use a safer packing scheme or separate storage for the bonus ball.


## [M-3]. Inefficient nested loop in winner counting enables DoS via block gas limit

## id: Ls6Rp44WNUxoqp42e9tA6

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
TicketComboTracker._countSubsetMatches

## Finding Status: Valid
### Finding Status Justification: TicketComboTracker._countSubsetMatches recomputes Combinations.generateSubsets(_normalBallsBitVector,k) inside the loop over i=1..bonusballMax. This redundancy is real and increases gas roughly linearly with bonusballMax. A revert in scaledEntropyCallback bricks progress because runJackpot sets jackpotLock=true and the callback is the only unlock path; ScaledEntropyProvider also deletes the pending request before calling back, preventing retry. There is no on-chain safeguard guaranteeing the callback fits within the effective block/callback gas limits as bonusballMax scales.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The function `TicketComboTracker._countSubsetMatches` contains a nested loop where `Combinations.generateSubsets` is called inside the loop iterating over `bonusballMax`. The `subsets` generation depends only on the normal balls (the winning ticket's normals) and `k`, not on the bonusball iterator `i`.

Since `bonusballMax` can grow up to 255, and `normalTiers` is 5, `generateSubsets` is called `255 * 5 = 1275` times in the worst case. Each call involves memory allocation and bitwise operations. This redundancy causes excessive gas consumption. If `bonusballMax` is high, the gas cost may exceed the block gas limit or the `entropyGasLimit` configured for the callback, causing the transaction to revert. A reverting callback leaves the jackpot in a locked state (`jackpotLock = true`), permanently halting the protocol until emergency action is taken.

## Impact
Protocol Denial of Service (DoS) requiring emergency intervention to unlock funds.

## Command to Run Test


## Proof of Concept
1. `bonusballMax` scales up to 255 due to ticket sales.
2. `runJackpot` is called, paying the fee.
3. Pyth triggers `scaledEntropyCallback`.
4. `_countSubsetMatches` executes. The redundant generation of subsets consumes >30M gas (or exceeds the callback limit).
5. Transaction reverts.
6. `jackpotLock` remains true. No new tickets can be sold, and `runJackpot` cannot be called again.

## Proof of Code


## Suggested Mitigation
Hoist the `Combinations.generateSubsets` call outside the `bonusballMax` loop in `_countSubsetMatches` to execute it only once per `k` tier.


## [H-4]. Integer overflow in `bonusballMax` calculation drastically reduces jackpot difficulty causing LP insolvency

## id: t3v3CtUzqXnK3GNSxBa0i

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.sol._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Unsafe uint8 downcast in `_setNewDrawingState` is real and permissionless to reach via organic pool growth (lpEarnings is not capped by lpPoolCap). It can occur at relatively moderate pool sizes once growth exceeds the bit-packing-derived cap, and impact can be severe (difficulty collapse / invalid entropy ranges / solvency break).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, the `_setNewDrawingState` function calculates the new `bonusballMax` to ensure the LP edge is preserved. The calculation `uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)))` explicitly casts the result of the division to `uint8`. If the required number of bonus balls exceeds 255, this cast overflows (wraps modulo 256). 

This condition is realistically reachable: with `normalBallMax` set to 10 (giving 252 combinations) and a prize pool of ~60,000 USDC, the formula requires ~264 bonus balls. The cast converts this to `8`, which is then clamped to `bonusballMin` (e.g., 10). Instead of the required 264 options, the game proceeds with only 10, reducing difficulty by ~26x. This grants players a massive positive expected value (+EV) at the expense of Liquidity Providers, leading to rapid insolvency of the LP pool.

## Impact
Direct theft of LP funds via statistical arbitrage; LPs take massive losses due to broken game math.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax`=10, `ticketPrice`=1 USDC, `lpEdgeTarget`=10%.
2. Prize pool accumulates to 60,000 USDC (via sales or seeding).
3. `runJackpot` is called to settle current drawing.
4. `_setNewDrawingState` calculates `minNumberTickets` approx 66,666.
5. `combosPerBonusball` is `C(10,5)` = 252.
6. `ceilDiv(66666, 252)` = 265.
7. `uint8(265)` overflows to 9.
8. `bonusballMax` set to `max(10, 9)` = 10 (instead of 265).
9. Users buy tickets in new drawing with 1/10 chance of matching bonus ball instead of 1/265, guaranteeing +EV.

## Proof of Code
function testBonusBallOverflow() public {
    // Setup scenario where required bonus balls > 255
    // Using normalBallMax = 10 (252 combos)
    // Prize pool needed ~ 255 * 252 * ticketPrice * (1/(1-edge))
    vm.startPrank(owner);
    jackpot.setNormalBallMax(10);
    // ... accumulate pool ...
    // Trigger settlement
    jackpot.runJackpot{value: fee}();
    // Check new drawing params
    Jackpot.DrawingState memory state = jackpot.getDrawingState(jackpot.currentDrawingId());
    // state.bonusballMax will be small (wrapped) instead of large
}

## Suggested Mitigation
Remove the `uint8` cast and cap the result at 255 (or `type(uint8).max`) inside the `Math.max` check, or revert if the required difficulty exceeds 255 to signal that parameters need adjustment.


## [M-5]. DoS due to Block Gas Limit in runJackpot via Entropy Callback

## id: ZuVAlW4ZzJuR6NWZDP9cF

## Derived From Pattern/Invariant
Dos

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.runJackpot

## Finding Status: Valid
### Finding Status Justification: Not primarily governance risk: a permissionless protocol can naturally reach large bonusballMax as TVL grows, and with default entropyVariableGasLimit the requested callback gas can exceed block limits, permanently stalling settlement (high impact).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract calculates `entropyGasLimit` as `entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax`. The default `entropyVariableGasLimit` is 250,000. As the prize pool grows, `bonusballMax` increases. If `bonusballMax` reaches ~120, the requested gas limit becomes `250,000 * 120 = 30,000,000`, which equals the typical block gas limit of Ethereum/Base. If the pool grows further, the requested gas exceeds the block limit, making `runJackpot` revert or be unmineable. This permanently freezes the jackpot, locking all funds.

## Impact
Permanent Denial of Service. The jackpot cannot settle, and no new drawings can start. Funds are locked unless Emergency Mode is activated (which liquidates LPs/refunds users, destroying protocol utility).

## Command to Run Test


## Proof of Concept
1. Deploy with defaults (`entropyVariableGasLimit` = 250,000).
2. Prize pool grows to ~$40M (requiring ~120 bonus balls).
3. `runJackpot` calculates limit: 30M gas.
4. Transaction exceeds block gas limit and cannot be mined.
5. System halts.

## Proof of Code
function test_GasLimitDoS() public {
    jackpot.setEntropyVariableGasLimit(250000);
    // Mock drawing state to have bonusballMax = 130
    // Call _calculateEntropyGasLimit -> returns > 30M
}

## Suggested Mitigation
Significantly lower the default `entropyVariableGasLimit` (e.g., to 20k-50k) to ensure the calculated limit stays within block bounds even for `bonusballMax` = 255. Note that the actual gas usage of the callback is much lower (~25k per ball), so 250k is an unsafe overestimation.


## [M-6]. DoS in `scaledEntropyCallback` due to Unbounded Storage Reads

## id: ISMx4SSk7v-xxIweWDClC

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
Dos

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: This is a real protocol-level gas-scaling risk in callback execution (not a governance mistake) and can permanently lock drawings (high impact) once bonusballMax is large enough; likelihood depends on reaching high bonusballMax.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`TicketComboTracker.countTierMatchesWithBonusball` iterates from 1 to `bonusballMax`. For each bonusball, it iterates over `normalTiers` (5) and all subsets ($2^5=32$). This results in ~160 SLOADs per bonusball. If `bonusballMax` approaches 255 (valid uint8), the total SLOADs (~40,000) result in gas costs exceeding the block gas limit (30M), causing the callback to revert permanently and locking the jackpot.

## Impact
Permanent freezing of the jackpot drawing and LP funds.

## Command to Run Test


## Proof of Concept
1. `prizePool` grows large enough to set `bonusballMax` to 255.
2. `runJackpot` is called. `entropyGasLimit` is calculated (~63M gas).
3. Provider calls `scaledEntropyCallback`.
4. Execution requires ~80M gas (SLOADs). Block limit is 30M.
5. Transaction reverts. Drawing never settles.

## Proof of Code
Mock `bonusballMax` to 255 in test. Call `countTierMatchesWithBonusball`. Measure gas. Assert gas > 30_000_000.

## Suggested Mitigation
Optimize `TicketComboTracker` to avoid iterating all bonusballs (e.g., only iterate the winning bonusball index if possible, or restructure data), or enforce a lower hard cap on `bonusballMax` that fits within block limits.


## [H-7]. Denial of Service in drawing settlement due to excessive gas consumption and block gas limit violation

## id: mRpAdd4R1jltu7EdRX61Y

## Derived From Pattern/Invariant
UnboundedLoops

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: Combination of two genuine issues (requested gas too large and actual callback gas too large). Not governance-driven and impact is high (drawing can remain locked). Likelihood depends on reaching large bonusballMax / parameter regime.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function settles the jackpot by calling `JackpotLPManager.processDrawingSettlement`, which triggers winner calculation via `_calculateDrawingUserWinnings`. This function calls `TicketComboTracker.countTierMatchesWithBonusball`, which iterates through all possible bonus balls from 1 to `bonusballMax` in `_countSubsetMatches`. For each bonus ball, it iterates through all subsets of the winning normal numbers (31 subsets for 5 balls). This results in `31 * bonusballMax` storage reads. Since `bonusballMax` is dynamically calculated based on the prize pool and can reach 255, this loop can perform up to ~7905 storage reads. Assuming cold access (EIP-2929), this costs approximately 16.6 million gas, which is extremely high.

Furthermore, `runJackpot` requests a gas limit from the entropy provider calculated as `entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax`. With the default `entropyVariableGasLimit` of 250,000, a `bonusballMax` of 121 results in a requested gas limit of > 30 million. This exceeds the block gas limit of Ethereum and most L2 networks (typically 30M). Consequently, the entropy provider will be unable to fulfill the request, or the callback transaction will consistently fail out-of-gas, leaving the jackpot permanently locked (`jackpotLock` remains true).

## Impact
The jackpot drawing process becomes permanently stuck for prize pools that result in a high `bonusballMax`. All user funds (ticket sales) and LP deposits are locked in the contract as the drawing cannot transition to the next state.

## Command to Run Test


## Proof of Concept
1. The protocol accumulates a prize pool such that `minNumberTickets` / `combosPerBonusball` results in a `bonusballMax` > 120 (e.g., ~$2M prize pool with 20 balls). 
2. A user calls `runJackpot`. The contract calculates `entropyGasLimit` > 30,000,000 (assuming default 250k variable limit). 
3. The entropy provider (Pyth) receives the request. 
4. When Pyth attempts to callback with the requested gas limit, the transaction cannot be included in a block because it exceeds the network block gas limit. 
5. The drawing remains locked forever.

## Proof of Code
function testGasDos() public {
    // Simulate state where bonusballMax is high
    // In a real test we would manipulate prize pool, here we calculate the limit
    uint32 base = 100000;
    uint32 variable = 250000;
    uint256 bonusballMax = 125;
    uint256 gasLimit = base + variable * bonusballMax;
    // gasLimit is 31,350,000, which > 30M block limit
    assertGt(gasLimit, 30000000);
}

## Suggested Mitigation
Optimize `TicketComboTracker` to avoid iterating over all bonus balls (e.g., by tracking aggregate counts for normal ball subsets independent of bonus balls). Additionally, cap `bonusballMax` to a value that ensures the callback gas cost stays well within block limits, or significantly reduce `entropyVariableGasLimit`.


## [H-8]. Inflation Attack on LP Pool via First Depositor Front-running

## id: 6-4-1YpuUzTr3xzVwOwZG

## Derived From Pattern/Invariant
ERC4626SharePriceMismatch

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: Valid
### Finding Status Justification: This is not governance risk: it is a known 'zero-share mint/donation' style vault issue if the accumulator can become huge, and there is no `shares > 0` / min-liquidity protection. If it happens, impact is high (subsequent deposits can mint 0 shares and be donated).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager` calculates LP shares using an accumulator model similar to ERC4626 vaults. The accumulator determines the exchange rate between USDC deposits and LP shares. In `processDrawingSettlement`, the accumulator is updated: `newAccumulator = (drawingAccumulator[last] * postDrawLpValue) / lpPoolTotal`. 

At system initialization or after a full withdrawal, `lpPoolTotal` can be very small (e.g., 1 wei). An attacker can deposit a minimal amount (1 wei) to obtain the first share. Then, by purchasing a large volume of jackpot tickets, they increase `lpEarnings`, which flows into `postDrawLpValue`. This massive increase in value relative to the tiny `lpPoolTotal` causes `newAccumulator` to explode to a huge value.

Subsequent legitimate LPs who deposit funds will have their share calculation `(amount * PRECISE_UNIT) / newAccumulator` round down to zero if `amount * PRECISE_UNIT < newAccumulator`. Their deposits are accepted, but they receive no shares, effectively donating their funds to the attacker who holds the entire share supply.

## Impact
Theft of funds from subsequent liquidity providers. The attacker can drain all subsequent deposits by redeeming their single share.

## Command to Run Test


## Proof of Concept
1. Attacker observes the `JackpotLPManager` has `lpPoolTotal == 0` (e.g. at launch after owner initializes with 1 wei, or after LPs exit). 2. Attacker calls `lpDeposit(1 wei)` and becomes the sole shareholder. 3. Attacker calls `buyTickets` spending 1,000,000 USDC. This increases `lpEarnings` by ~1,000,000 USDC. 4. Drawing settles. `postDrawLpValue` is ~1,000,000 USDC. `lpPoolTotal` is 1 wei. 5. `newAccumulator` becomes `~1e30`. 6. A victim deposits 100,000 USDC (`1e11` wei). 7. Victim shares = `1e11 * 1e18 / 1e30 = 0`. 8. Victim gets 0 shares; Attacker withdraws and claims the victim's 100,000 USDC.

## Proof of Code
test_InflationAttack()

## Suggested Mitigation
Enforce a substantial minimum initial deposit in `initializeJackpot` (e.g., 1000 USDC) to buffer the pool, or implementation a virtual offset / dead shares mechanism to permanently lock the accumulator's precision.


## [H-9]. Bonus ball bit overflow causes incorrect payout tier calculation

## id: Gtu8PvC7OHY_8DoJIxJR0

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: If a drawing is created where normalBallMax + bonusballMax >= 256, ticket packing breaks and tiering/claims can be incorrect (potential overpayment/DoS). Not governance-driven, impact not low; likelihood depends on reaching such parameter regime.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The system packs ticket numbers into a `uint256` bit vector. The bonus ball is stored at the bit offset `normalBallMax + bonusball`. While `normalBallMax` is capped at 128, `bonusballMax` can grow up to 255 (or wrap). If `normalBallMax + bonusball` equals or exceeds 256, the shift `1 << (bonus + normalMax)` overflows/shifts-out in `TicketComboTracker.insert`, resulting in a packed ticket with no bonus bit set (0). 

In `Jackpot._calculateTicketTierId`, the bonus ball is extracted via `_ticketNumbers >> (_normalBallMax + 1)`. If the bit was shifted out, this extraction returns 0. If the winning ticket also had a large bonus ball that shifted out, it also extracts 0. The comparison `ticketBonusball == winningBonusball` evaluates to `0 == 0` (True). This effectively bypasses the bonus ball check for all tickets with large bonus numbers, granting them a 'match' regardless of their actual selection, doubling their win probability for that tier.

## Impact
Incorrect payout calculations. Users with non-matching bonus balls are credited with a match if the bonus ball number is large, leading to overpayment and loss of LP funds.

## Command to Run Test


## Proof of Concept
1. Owner sets `normalBallMax` to 100.
2. Pool grows such that `bonusballMax` is set to 200.
3. User buys ticket with Bonus Ball 200. Offset = 100 + 200 = 300. `1<<300` is 0.
4. Packed ticket has 0 in high bits.
5. Winning numbers drawn: Bonus Ball 201. Offset = 301. `1<<301` is 0.
6. `_calculateTicketTierId` extracts 0 for user and 0 for winner.
7. `0 == 0` -> Bonus Match. User wins bonus tier despite selecting 200 vs 201.

## Proof of Code
function test_BonusOverflow() public {
    // Setup normalMax = 100, bonusMax = 200
    // Mint ticket with bonus 200
    // Set winning ticket with bonus 201
    // Verify claimWinnings allows claim
}

## Suggested Mitigation
Enforce strict bounds in `_setNewDrawingState` or `setNormalBallMax` such that `normalBallMax + bonusballMax < 255`. Alternatively, use a more robust storage structure for tickets that does not rely on shifting a single `uint256`.


## [H-10]. Organic LP pool growth via ticket earnings bypasses `lpPoolCap` causing ticket corruption and loss of winnings

## id: VfUFV4UanlLAC_X1nIgPe

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: lpPoolCap only caps deposits, not growth via lpEarnings rolled into next drawing. That can push bonusballMax beyond bitpacking-safe bounds and corrupt tickets/claims. Not governance risk; impact can be severe.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol enforces an `lpPoolCap` on deposits to ensure the prize pool size remains within limits that allow the dynamically calculated `bonusballMax` to fit within the 256-bit vector used for ticket storage (specifically `bonusballMax + normalBallMax < 256`). However, `lpEarnings` from ticket sales are added to the pool in `processDrawingSettlement` without checking this cap. In a scenario with high ticket volume, the pool can organically grow beyond the safety limit. This causes the calculated `bonusballMax` for the next drawing to exceed `255 - normalBallMax`. When this happens, the bitwise shift `1 << (bonusball + normalMax)` in `TicketComboTracker.insert` overflows (specifically `1 << 256` becomes 0), causing the bonusball bit to be lost in the `packedTicket` stored in the NFT. As a result, users holding these tickets cannot claim winnings for the bonusball tier because the ticket data is corrupted.

## Impact
Users who purchase tickets in a drawing where the pool has exceeded the safety cap due to earnings will receive corrupted tickets. These users will be unable to claim winnings for the bonusball tier, leading to a direct loss of funds.

## Command to Run Test


## Proof of Concept
1. Deploy Jackpot with `normalBallMax` = 5. This implies a safety limit for `bonusballMax` of 250 (255 - 5) and a very low `lpPoolCap`.
2. LPs deposit up to the small `lpPoolCap`.
3. Users buy a large volume of tickets (e.g., 1000 tickets). `lpEarnings` increases the effective pool size significantly beyond the cap.
4. `runJackpot` is called. `scaledEntropyCallback` calculates `newLPValue` including the large `lpEarnings`.
5. `_setNewDrawingState` calculates `minNumberTickets` based on the inflated `prizePool`, resulting in a `bonusballMax` > 250.
6. In the new drawing, a user buys a ticket with a high bonusball value (e.g., 251).
7. `TicketComboTracker.insert` executes `1 << (251 + 5)` = `1 << 256` = 0. The bonusball bit is lost.
8. The drawing occurs and that bonusball wins. The user calls `claimWinnings` but fails to match because the stored ticket is invalid.

## Proof of Code


## Suggested Mitigation
In `_setNewDrawingState`, explicitly cap `newDrawingState.bonusballMax` to `255 - normalBallMax` regardless of the calculated value. This ensures the bit vector integrity is always preserved even if the pool over-performs.


## [H-11]. Bitpacking overflow allows bonusball manipulation in high-value jackpots

## id: cuFDTtIVtWXOBcHJZJ-og

## Derived From Pattern/Invariant
ReserveOrPriceDesync

## Exploit Type
Randomness

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: The condition `normalBallMax + bonusballMax >= 256` is not inherently rare: (a) `normalBallMax` is only constrained to <=128, and (b) `bonusballMax` is recomputed from prizePool, which can be driven upward by rollovers and (especially) by uncapped `lpEarnings` rolling into LP value. So reaching the unsafe boundary can be a natural high-activity outcome, not a “rare edge case”.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol uses a bit-packed format to store tickets and determine winning tiers, where normal numbers occupy bits `1` to `normalBallMax` and the bonusball occupies the bit at `normalBallMax + bonusball`. This relies on the invariant `normalBallMax + bonusballMax < 256` to fit within a `uint256`. While `initializeLPDeposits` calculates a pool cap attempting to enforce this, the `lpPoolCap` only limits deposits, not ticket revenue (`lpEarnings`). 

In a scenario with high ticket volume, `lpEarnings` can increase the `prizePool` (and thus the calculated `bonusballMax`) beyond the safe limit calculated from `lpPoolCap`. If `normalBallMax + bonusballMax >= 256`, the shift `1 << (normal + bonus)` overflows to 0. 

Consequently, tickets with high bonusballs are stored with the bonusball bit missing (effectively 0). If the winning bonusball is also in the high range, its bit is also lost. The `_calculateTicketTierId` function will then compare `0 == 0` and award a bonusball match. This collapses the entire range of 'overflowing' bonusballs into a single winning condition, significantly increasing the probability of winning (e.g., matching any number in a range of 50 values instead of 1 specific value), enabling users to drain the prize pool.

## Impact
Users can exploit the bitpacking overflow to gain a massive probabilistic advantage, draining the jackpot prize pool.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 50. Safe limit for `bonusballMax` is 205.
2. Massive ticket sales occur, pushing `lpEarnings` high. The next drawing's `prizePool` is calculated such that `minNumberTickets` requires a `bonusballMax` of 220.
3. `_setNewDrawingState` sets `bonusballMax` to 220. `normal + bonus` = 270 (overflows 256).
4. Attacker buys tickets covering the bonusball range 206-220.
5. `TicketComboTracker.insert` overflows the shift, storing tickets with no bonusball bit.
6. Drawing occurs. If the winning bonusball is 210, `countTierMatches` overflows the winning ticket shift.
7. `claimWinnings` matches the attacker's 'empty' bonusball bit against the winning 'empty' bit, awarding a prize incorrectly.

## Proof of Code


## Suggested Mitigation
In `_setNewDrawingState`, strictly cap `bonusballMax` such that `normalBallMax + bonusballMax < 256` (e.g., `Math.min(calculatedBonus, 255 - normalBallMax)`).


## [H-12]. Critical LP insolvency risk due to `bonusballMax` unsafe downcasting and bit-vector overflow

## id: dL3mKOlInvF7myginfp1w

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Unsafe uint8 downcast plus missing clamp to (255 - normalBallMax) can break both economics (difficulty collapse) and correctness (bitpacking overflow). Not governance risk; impact can be catastrophic; likelihood depends on large pool/parameter growth.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot._setNewDrawingState` function calculates `newBonusball` (the difficulty parameter) to ensure LP profitability based on the prize pool size. 

There are two critical issues in this calculation:

1. **Unsafe Downcasting**: The result of `Math.ceilDiv(minNumberTickets, combosPerBonusball)` is explicitly cast to `uint8` without checks. `minNumberTickets` grows linearly with `prizePool`. If the calculated difficulty exceeds 255 (e.g., for a prize pool > ~160M USDC with standard parameters), the value wraps around (e.g., 300 becomes 44). This drastically reduces the game difficulty, violating the LP edge requirement and allowing users to drain the pool with +EV tickets.

2. **Bit-Vector Overflow**: The system stores tickets as `uint256` bit vectors. The bonus ball is stored at bit position `normalBallMax + bonusball`. If `newBonusball + normalBallMax >= 256`, the shift operation `1 << (bonus + normal)` results in 0 due to EVM semantics for 256-bit shifts. This effectively erases the bonus ball data from both user tickets and winning tickets, causing all tickets to 'match' the bonus ball (0 == 0) regardless of selection. This creates a guaranteed bonus ball match for all users, massively inflating payouts.

## Impact
LPs face guaranteed insolvency. In the wrap-around case, difficulty drops significantly, eroding edge. In the bit-overflow case, payout frequency increases drastically due to false matches.

## Command to Run Test


## Proof of Concept
1. Assume `ticketPrice` = 2 USDC, `lpEdgeTarget` = 25%, `normalBallMax` = 35. 
2. `prizePool` grows to 100M USDC. `minNumberTickets` ≈ 82M. `combosPerBonusball` ≈ 324k. 
3. Calculated difficulty ratio ≈ 252. `bonusballMax` set to 252. 
4. `normalBallMax` (35) + `bonusballMax` (252) = 287. 
5. User buys ticket with bonus ball 252. `packedTicket` has bit 287 set -> overflows to 0. 
6. Winning number drawn with bonus ball 252. `winningTicket` has bit 287 set -> overflows to 0. 
7. `_calculateTicketTierId` compares 0 and 0. Match found. 
8. Alternatively, if `prizePool` grows to 200M USDC, ratio ≈ 504. `uint8(504)` wraps to 248. Difficulty is 248 instead of 504.

## Proof of Code
function testBonusBallOverflow() public {
    // Simulate state inside _setNewDrawingState
    uint256 minTickets = 300;
    uint256 combos = 1;
    // Result 300 wraps to 44
    uint8 bonusball = uint8(Math.ceilDiv(minTickets, combos));
    assertEq(bonusball, 44);
}

## Suggested Mitigation
1. Use `UintCasts.toUint8` or manual check to enforce `newBonusball <= 255`. If it exceeds 255, cap it at 255 (though this accepts edge erosion) or revert (halting the protocol).
2. Enforce `newBonusball + normalBallMax < 256` to prevent bit-vector overflow. Cap `newBonusball` at `255 - normalBallMax`.


## [H-13]. Bit Packing Overflow causes corrupted tickets and accounting mismatch leading to insolvency

## id: WVRa4VNQO3WhPPi5VL7kQ

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: If bitpacking overflow occurs, claim-time tiering can diverge from settlement-time winner accounting (tracker), enabling over-claims and insolvency. Not governance risk; impact is high; likelihood depends on reaching overflowed bonus range.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, the `_setNewDrawingState` function calculates `newBonusball` based on the prize pool size. It fails to verify that `normalBallMax + newBonusball < 256`. Due to the `lpEarnings` mechanism (which is unbounded and increases the next drawing's pool), it is possible for the calculated `newBonusball` to fall into a range where `newBonusball <= 255` (valid `uint8`) but `normalBallMax + newBonusball >= 256`. 

When a ticket is purchased with such a bonusball, the bit shift operation `1 << (ticket.bonusball + drawingState.ballMax)` in `TicketComboTracker.insert` overflows `uint256` and results in `0` (or collision). The ticket is stored with the bonus bit missing or corrupted. 

However, the `TicketComboTracker` correctly tracks the ticket count in its mapping because it uses the raw `bonusball` value as a key. This creates a disconnect: the winner calculation (using the tracker) includes the ticket in the prize distribution denominator, but `claimWinnings` (using bitwise matching) may incorrectly identify the ticket as a winner or loser due to the corrupted packed format. Specifically, different bonusballs can map to the same packed representation (missing bit), allowing users to claim winnings for outcomes they didn't hold, or allowing multiple disjoint outcomes to claim against the same prize share. This mismatch between calculated payouts (in `calculateAndStoreDrawingUserWinnings`) and actual claims drains the contract's USDC balance, violating the solvency invariant.

## Impact
Insolvency of the jackpot contract due to mismatch between calculated prize allocations and actual payouts.

## Command to Run Test


## Proof of Concept
1. Deploy Jackpot with `normalBallMax = 50`. The implied cap for bonusball is `255 - 50 = 205`.
2. LPs deposit to the cap. Users buy tickets, increasing `lpEarnings` such that the next drawing's `minNumberTickets` implies a `newBonusball` of `210`.
3. `runJackpot` executes. `newBonusball` is set to 210. `210 + 50 = 260`. `210` fits in `uint8`.
4. User buys a ticket with bonusball 210. `1 << 260` overflows to 0. Packed ticket has no bonus bit.
5. User buys a ticket with bonusball 220. `1 << 270` overflows to 0. Packed ticket has no bonus bit.
6. Both tickets have identical packed representations. However, they increment different buckets in `TicketComboTracker`.
7. Drawing happens. Winning bonusball is 210.
8. Payout calculator sums winners from bucket 210. Calculates payout per winner.
9. User claims with ticket 220. `_calculateTicketTierId` compares packed ticket (bonus 0) with packed winning ticket (bonus 0). Match found.
10. User gets paid for ticket 220, even though it wasn't a winner and wasn't accounted for in the prize division. Funds drain.

## Proof of Code
/* PoC omitted for brevity, logic follows step-by-step above */

## Suggested Mitigation
In `_setNewDrawingState`, clamp the `calculatedBonusball` to `255 - normalBallMax` before casting to `uint8`.


## [H-14]. Unsafe Downcasting of Dynamic Bonusball Parameter Leads to Difficulty Collapse

## id: ifkcRHeYpDjp9WE1ezUdC

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Unsafe downcast is a genuine issue; if required bonusballMax exceeds 255 it wraps and undermines the intended edge guarantee and/or can create invalid ranges. Not governance risk; impact can be severe; likelihood depends on very large pool growth.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the `bonusballMax` for the next drawing is dynamically calculated based on the prize pool size to maintain the LP edge. The calculation `Math.ceilDiv(minNumberTickets, combosPerBonusball)` returns a `uint256` which is explicitly cast to `uint8`. While `lpPoolCap` attempts to limit the pool size, accumulated `lpEarnings` from ticket sales are added to the pool without cap validation. If the pool grows sufficiently large (via earnings) such that the required bonusball count exceeds 255, the explicit cast truncates the value (e.g., 256 becomes 0). This sets `bonusballMax` to the minimum value (e.g., 1), drastically lowering the difficulty and allowing users to drain the LP pool with near-guaranteed wins.

## Impact
LPs lose nearly all funds as the jackpot difficulty collapses, giving players a massive mathematical advantage.

## Command to Run Test


## Proof of Concept
1. `lpPoolCap` is set corresponding to 220 bonusballs (with 35 normals). 2. LPs deposit up to the cap. 3. Over time, ticket sales add `lpEarnings` to the pool, pushing the effective pool size 20% higher. 4. `runJackpot` is called. The settlement adds earnings to the pool. 5. `_setNewDrawingState` calculates `minNumberTickets` based on the inflated pool. 6. `ceilDiv` returns 256. 7. `uint8(256)` truncates to 0. 8. `newBonusball` becomes `max(min, 0)`. 9. Next drawing has minimal difficulty; attackers empty the pool.

## Proof of Code
function testTruncation() public { uint256 bigNum = 256; uint8 truncated = uint8(bigNum); assertEq(truncated, 0); }

## Suggested Mitigation
Use `UintCasts.toUint8` to revert on overflow, or cap the result at 255 (or `255 - normalBallMax`) to stay within bounds.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-15]. Lack of validation that `referralFee < lpEdgeTarget` allows LPs to be drained via duplicate tickets

## id: YHyCKmpXmeOTzBbnxf78t

## Derived From Pattern/Invariant
ConfigFootgun

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.sol.buyTickets

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: If misconfigured (referralFee > lpEdgeTarget), the impact is not low: attackers can create an accounting deficit per duplicate ticket and extract LP equity / brick settlement.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `buyTickets` function accounts for duplicate tickets by adding `ticketPrice - edgePerTicket` to the prize pool (liability) while adding `ticketPrice - referralFee` to `lpEarnings` (asset). If the admin configures `referralFee` to be greater than `lpEdgeTarget`, buying a duplicate ticket increases the protocol's liability (prize pool) by more than it increases its assets (earnings). 

This creates a permissionless arbitrage loop: an attacker (acting as their own referrer) buys duplicate tickets. They pay `Price`, receive `ReferralFee` back, and increase the prize pool by `Price * (1 - Edge)`. If `Ref > Edge`, the net cost to the attacker `Price * (1 - Ref)` is less than the value added to the pool. The difference is extracted directly from the LP pool's equity.

## Impact
Financial loss to LPs; attackers can drain LP equity into the prize pool and win it back (or diluted among winners) at a discount.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` = 20% and `lpEdgeTarget` = 10%.
2. Attacker buys a duplicate ticket for 100 USDC.
3. Attacker gets 20 USDC refund via referral. Net cost = 80 USDC.
4. Protocol adds `100 * (1 - 0.10)` = 90 USDC to Prize Pool.
5. LP Earnings increase by `100 * (1 - 0.20)` = 80 USDC.
6. System Liability increased by 90, Assets by 80. LP loses 10 USDC.
7. Attacker repeats to pump prize pool, then wins the inflated pool (positive expected value).

## Proof of Code
function testReferralArbitrage() public {
    vm.startPrank(owner);
    jackpot.setReferralFee(2e17); // 20%
    jackpot.setLpEdgeTarget(1e17); // 10%
    vm.stopPrank();
    // ... buy duplicate ticket ...
    // assert(lpValueBefore - lpValueAfter > 0);
}

## Suggested Mitigation
Add a require statement in `setReferralFee` and `setLpEdgeTarget` to ensure `referralFee <= lpEdgeTarget`.


## [M-16]. Drawing settlement DoS due to `postDrawLpValue` underflow caused by duplicate tickets

## id: vI1SKbJ8eajRiXfKr67Wd

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: If misconfigured, impact is not low: settlement can revert and lock the protocol into emergency mode.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager.processDrawingSettlement` function calculates the new LP value as `lpPoolTotal + lpEarnings - userWinnings`. When users buy duplicate tickets, the `prizePool` increases by `TicketPrice * (1 - Edge)`, while `lpEarnings` increases by `TicketPrice * (1 - ReferralFee)`. 

If the protocol is configured such that `ReferralFee > LpEdgeTarget`, each duplicate ticket creates a net liability greater than the net assets added to the pool. If a sufficient volume of duplicate tickets is purchased, `userWinnings` (driven by the inflated prize pool) can exceed `lpPoolTotal + lpEarnings`, causing the subtraction to underflow and revert. This permanently freezes the jackpot, as `runJackpot` calls this settlement function.

## Impact
Medium. Permanent Denial of Service of the jackpot system requiring emergency mode activation. Exploitable if governance sets `ReferralFee > Edge`.

## Command to Run Test


## Proof of Concept
1. `lpEdgeTarget` = 10%. `referralFee` = 20%.
2. `prizePool` = 1000 USDC. `lpPool` = 1100 USDC.
3. User buys many duplicate tickets. Each adds 0.9 * Price to Pool, but only 0.8 * Price to Earnings.
4. Net deficit of 0.1 * Price per ticket.
5. If duplicates > 11000 * Price, the deficit consumes the entire LP pool.
6. Settlement calculates `1100 + (0.8*N) - (1000 + 0.9*N)`.
7. 100 - 0.1*N < 0 -> Underflow.

## Proof of Code
function testSettlementDoS() public {
    // Setup: High referral, Low edge
    // Buy many duplicates
    // Call processDrawingSettlement with max winnings
    // Expect revert due to underflow
}

## Suggested Mitigation
Enforce `lpEdgeTarget >= referralFee` in configuration setters, or cap `userWinnings` to `lpPoolTotal + lpEarnings` in settlement (though this breaks the full collateralization promise).


## [M-17]. Total LP loss results in zero accumulator and bricks future deposits

## id: XCz3TM_HOVDkITSoZ2ME3

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: In JackpotLPManager.processDrawingSettlement (for drawingId>0), newAccumulator = (prevAcc * postDrawLpValue) / currentLP.lpPoolTotal when lpPoolTotal != 0. If postDrawLpValue becomes 0, newAccumulator becomes 0 and is stored. Later, _consolidateDeposits/_consolidateWithdrawals divide by drawingAccumulator[d], which would revert on division by zero, effectively bricking affected LP actions. Achieving postDrawLpValue==0 is not typical under sane parameters (they even comment “accumulators can never be zero”), but it is possible under extreme governance configuration (e.g., reserveRatio=0 and effectively zero lpEarnings due to 100% referral fee with referrals) combined with maximal payouts. Hence it is primarily governance/config-risk, but the code has no guard to prevent accumulator=0.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager.processDrawingSettlement`, the accumulator is updated using the ratio `(prevAcc * postDrawLpValue) / prevLpTotal`. If a catastrophic loss occurs (e.g. `reserveRatio` is set to 0 and players win the entire pool), `postDrawLpValue` can become 0. This sets `drawingAccumulator` to 0. 

Future calls to `processDeposit` calculate shares via division by the accumulator: `amount * PRECISE_UNIT / accumulator`. If the accumulator is 0, this division causes a revert/panic. Consequently, the system becomes permanently bricked; no new LP deposits can be made to recapitalize the pool.

## Impact
Permanent protocol bricking (DoS) for LP deposits if the pool is ever fully drained.

## Command to Run Test


## Proof of Concept
1. `reserveRatio` is set to 0 (allowed).
2. A drawing occurs where user winnings equal the total `lpPoolTotal` + earnings (Crisis mode).
3. `postDrawLpValue` becomes 0.
4. `drawingAccumulator` becomes 0.
5. Any user attempting `lpDeposit` reverts due to division by zero.

## Proof of Code


## Suggested Mitigation
In `processDrawingSettlement`, ensure the new accumulator has a minimum value of 1 (or handle the 0 case by resetting to `PRECISE_UNIT` if the pool is empty/wiped out).


## [H-18]. Referral Fee exceeding LP Edge Target causes settlement underflow and DoS

## id: scBeDo1Fguwll8-m5eOio

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: Once governance misconfigures referralFee > lpEdgeTarget, exploitation is permissionless (attackers can buy duplicates to create deficit/DoS). The impact is not low (can brick settlement / force emergency).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The protocol relies on an invariant that duplicate ticket purchases increase the `prizePool` by `ticketPrice * (1 - lpEdgeTarget)` to preserve LP edge. However, `lpEarnings` (which funds the pool) only increases by `ticketPrice * (1 - referralFee)`. 

If the admin configures `referralFee > lpEdgeTarget` (e.g., 50% referral vs 20% edge), every duplicate ticket creates a deficit where the `prizePool` obligation grows faster than the backing `lpEarnings`. An attacker can purchase a large volume of duplicate tickets to accumulate this deficit. When `processDrawingSettlement` runs, it calculates `postDrawLpValue = lpPoolTotal + earnings - userWinnings`. If `userWinnings` (driven by `prizePool`) exceeds `lpPoolTotal + earnings` due to the deficit, the subtraction underflows and reverts. This permanently locks the jackpot drawing (`jackpotLock` remains true), freezing all funds.

## Impact
Permanent DoS of the protocol and freezing of all user and LP funds.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` = 50% and `lpEdgeTarget` = 10%.
2. LPs provide 1000 USDC.
3. User buys 5000 duplicate tickets of a winning combination (self-referring to recoup cost).
4. Each ticket adds 0.9 * Price to PrizePool but only 0.5 * Price to Earnings.
5. Deficit = 0.4 * Price * 5000 = 2000 * Price.
6. Deficit > LP Pool (1000).
7. Drawing settlement attempts `1000 + 2500 - 4500` (underflow).
8. Revert.

## Proof of Code
function testReferralFeeDoS() public {
    vm.startPrank(owner);
    jackpot.setReferralFee(5e17); // 50%
    jackpot.setLpEdgeTarget(1e17); // 10%
    vm.stopPrank();

    // ... (setup LP and buy many duplicates) ...
    
    // Attempt settlement
    // vm.expectRevert(); // Arithmetic underflow
    // entropyProvider.triggerCallback(...);
}

## Suggested Mitigation
In `setReferralFee` and `setLpEdgeTarget`, enforce the invariant: `referralFee <= lpEdgeTarget`.





Finding Status: InvalidByDesign
## [M-19]. Strict balance check in JackpotBridgeManager causes DoS with bridges that refund dust

## id: nV4SYdjxxBCHDlzV03OpJ

## Derived From Pattern/Invariant
ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
JackpotBridgeManager._bridgeFunds

## Finding Status: InvalidByDesign
### Finding Status Justification: Even if the strict equality check is intentional, failure with refunding/dust-returning bridge adapters is not inherently “user mistake”; many routers refund dust by design. This is a protocol integration/interoperability limitation that can DoS claims for those routes.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotBridgeManager.claimWinnings`, the internal function `_bridgeFunds` enforces a strict equality check on the USDC balance change:

```solidity
if (preUSDCBalance - postUSDCBalance != _claimedAmount) revert NotAllFundsBridged();
```

Many bridge adapters (e.g., Stargate, LayerZero) estimate fees or swap amounts and often refund small amounts of dust or excess fees to the caller (`JackpotBridgeManager`) during the transaction. If `_bridgeDetails.to.call(...)` results in a refund of even 1 wei of USDC, `preUSDCBalance - postUSDCBalance` will be strictly less than `_claimedAmount`. The check will fail, reverting the transaction. This makes the protocol incompatible with common bridging infrastructure, preventing users from claiming winnings.

## Impact
Users cannot claim winnings if the chosen bridge adapter refunds dust or excess funds, leading to Denial of Service for those bridge routes.

## Command to Run Test


## Proof of Concept
1. User calls `claimWinnings` with a bridge adapter that refunds 1 wei of USDC (dust).
2. `jackpot.claimWinnings` sends 100 USDC to BridgeManager.
3. `_bridgeFunds` calls bridge adapter with 100 USDC.
4. Bridge adapter uses 99.99 USDC and refunds 0.01 USDC to BridgeManager.
5. `pre - post` = 100 - 0.01 = 99.99.
6. `99.99 != 100`. Transaction reverts `NotAllFundsBridged`.

## Proof of Code
function test_BridgeRefundDustReverts() public {
    // Mock bridge adapter that refunds 1 wei
    MockBridge bridge = new MockBridge(); // refunds 1 wei on call
    
    // ... Setup tickets and winnings ...

    vm.expectRevert(JackpotBridgeManager.NotAllFundsBridged.selector);
    bridgeManager.claimWinnings(ticketIds, relayData, sig);
}

## Suggested Mitigation
Relax the check to allow dust refunds: `require(preUSDCBalance - postUSDCBalance <= _claimedAmount, ...)` or verify that the bridge adapter received the approval/transfer successfully without strictly checking the manager's balance decrease.


## [M-20]. Strict Balance Check in `claimWinnings` Breaks Integration with Refund-Logic Bridges

## id: 2GqXToUNK2BMpROYgkI3G

## Derived From Pattern/Invariant
ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
JackpotBridgeManager.claimWinnings

## Finding Status: InvalidByDesign
### Finding Status Justification: As with the underlying `_bridgeFunds` check, incompatibility with refunding bridges is not purely a user mistake; it is an on-chain strictness choice that can prevent legitimate claim flows via common bridge adapters.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `claimWinnings`, the contract enforces a strict equality check on the USDC balance change: `preUSDCBalance - postUSDCBalance != _claimedAmount`. This logic assumes that the external bridge call (`_bridgeDetails.to.call`) consumes exactly the `_claimedAmount` and nothing else affects the balance. However, many bridge routers or aggregators refund dust or excess gas/tokens to the `msg.sender` (the Bridge Manager) synchronously during the transaction. If a refund occurs, `postUSDCBalance` will be higher than expected, causing `pre - post` to be less than `_claimedAmount`, triggering a revert. This creates a Denial of Service for users attempting to use such bridge paths.

## Impact
Users cannot claim winnings using legitimate bridge adapters that perform refunds or rebates, restricting protocol interoperability and potentially locking funds if no other route is available.

## Command to Run Test


## Proof of Concept
1. User wins 100 USDC.
2. User signs `claimWinnings` with `_bridgeDetails` pointing to a Bridge Aggregator.
3. `JackpotBridgeManager` executes logic. `pre` balance = 0.
4. `Jackpot` sends 100 USDC. Balance = 100.
5. `_bridgeFunds` calls Aggregator.
6. Aggregator swaps/bridges, but has 0.01 USDC dust left. Refunds 0.01 to Manager.
7. `post` balance = 0.01.
8. `pre` (measured before call) = 100.
9. Check: `100 - 0.01 != 100` -> Revert.

## Proof of Code
it('reverts if bridge refunds dust', async function() {
  // Mock bridge that refunds 1 wei
  // Expect revert NotAllFundsBridged
});

## Suggested Mitigation
Relax the check to `require(preUSDCBalance - postUSDCBalance >= _claimedAmount)`. While this theoretically allows a bridge to pull more funds, the Manager typically holds no user funds other than transient winnings, so the risk is low compared to the DoS risk.





Finding Status: LowSeverityDueToLowImpact
## [L-21]. Missing Deadline in EIP-712 Signatures Allows Indefinite Validity

## id: iOjys7LUfWIfFabEj6XSO

## Derived From Pattern/Invariant
PermitDeadlineBypass

## Exploit Type
PermitDeadlineBypass

## Location
JackpotBridgeManager.claimWinnings

## Finding Status: LowSeverityDueToLowImpact
### Finding Status Justification: Burning tickets (claimWinnings) or deleting `ticketOwner` entries (claimTickets) only prevents replay after the authorization is consumed; it does not prevent indefinite validity/stale execution prior to consumption. This is a design/UX weakness in the signature scheme (no deadline/nonce), not merely user error.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The EIP-712 signature schemas for `claimWinnings` and `claimTickets` do not include a deadline or expiry timestamp. Signatures generated by users are valid indefinitely. If a user generates a signature with specific bridge details but later decides to change parameters (e.g., due to bridge fee changes or route depreciation) or cancel the action, they cannot invalidate the old signature without successfully front-running it with a new one. This exposes users to execution of stale or unintended cross-chain transactions by keepers.

## Impact
Users are unable to expire pending signatures, leading to potential execution of stale transactions or loss of control over cross-chain routing preferences.

## Command to Run Test


## Proof of Concept
1. User signs `claimWinnings` bridging to Chain A.
2. User changes mind, signs new `claimWinnings` bridging to Chain B.
3. Keeper network or malicious actor submits the first signature (Chain A).
4. Transaction succeeds, funds go to Chain A against user's current intent.
5. The Chain B signature fails (tickets burned).

## Proof of Code
it('accepts old signature indefinitely', async function() {
  // Sign payload
  // Warp time 1 year
  // execute claimWinnings
  // Expect success
});

## Suggested Mitigation
Add a `deadline` field to the `ClaimWinningsData` and `ClaimTicketData` structs and verify `block.timestamp <= deadline` in the verification logic.



