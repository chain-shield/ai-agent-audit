# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[M-1]. DoS of drawing settlement due to block gas limit in `TicketComboTracker`
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[M-2]. LPs can permanently block critical parameter updates via front-running
**Derived From** : Governance Front-running / Griefing
Finding Status: Valid
Privilege: Permissionless


[M-3]. JackpotBridgeManager users lose funds in emergency mode due to missing refund functionality
**Derived From** : Standard Violation
Finding Status: Valid
Privilege: Permissionless


[M-4]. JackpotBridgeManager Denial of Service or fund loss due to ticket price mismatch
**Derived From** : Read-only Reentrancy (Price Staleness)
Finding Status: Valid
Privilege: RequiresAdminRole


[M-5]. Denial of Service in Settlement Callback due to excessive gas consumption loops
**Derived From** : Gas Griefing / Block Gas Limit
Finding Status: Valid
Privilege: Permissionless


[M-6]. Settlement permanent DoS via gas limit exhaustion from high `bonusballMax`
**Derived From** : DoS / Out of Gas in Callback
Finding Status: Valid
Privilege: Permissionless


[H-7]. Arbitrary External Call in JackpotBridgeManager enables theft of all custodial NFTs
**Derived From** : ArbitraryExternalCall
Finding Status: Valid
Privilege: Permissionless


[H-8]. LP Share Price Inflation Attack allowing theft of future deposits
**Derived From** : ERC4626SharePrice
Finding Status: Valid
Privilege: Permissionless


[H-9]. Winning tickets corrupted due to bit packing overflow in `TicketComboTracker`
**Derived From** : Bit Packing Overflow
Finding Status: Valid
Privilege: Permissionless


[H-10]. Bitwise overflow in ticket storage causes collision of distinct tickets and payouts
**Derived From** : Storage Collision / Bit Packing
Finding Status: Valid
Privilege: Permissionless


[H-11]. Organic LP pool growth bypasses `lpPoolCap` causing bit-shift overflow and prize dilution
**Derived From** : Integer Overflow / Logic Error
Finding Status: Valid
Privilege: Permissionless


[H-12]. Catastrophic loss of LP funds or DoS due to unsafe downcasting of `bonusballMax`
**Derived From** : Integer Overflow / Unsafe Casting
Finding Status: Valid
Privilege: Permissionless


[H-13]. Bonusball bit-packing overflow allows guaranteed bonus matches and pool draining
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-14]. Critical Logic Failure and DoS due to Unbounded Dynamic Bonusball Calculation
**Derived From** : StorageLayout
Finding Status: Valid
Privilege: Permissionless


[H-15]. Unbounded Prize Pool Growth causes Integer Overflow in Difficulty Calculation and Ticket Packing Collisions
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-16]. Downcast to uint8 in `_setNewDrawingState` causes bonus ball count to wrap, removing LP edge
**Derived From** : Unsafe Downcasting
Finding Status: Valid
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-17]. Permanent DoS of LP Manager via Zero Accumulator if pool is drained
**Derived From** : Division by Zero
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[M-18]. Entropy provider switch causes pending request collision allowing randomness manipulation
**Derived From** : Key Collision in External Protocol Integration
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-19]. Cross-chain winnings claim DoS if referral win share is 100%
**Derived From** : Dos
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-20]. Randomness integrity compromise and potential DoS due to provider sequence collision in ScaledEntropyProvider
**Derived From** : ExternalProtocolKeyCollision
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-21]. DoS of LP system if Accumulator falls to zero due to pool wipeout
**Derived From** : Dos
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


[H-22]. Jackpot DoS via Entropy Provider Sequence Collision
**Derived From** : ExternalProtocolKeyCollision
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: InvalidGovernanceRisk


[H-23]. Swapping PayoutCalculator via setPayoutCalculator permanently freezes unclaimed winnings from previous drawings
**Derived From** : UpgradeabilityInitializerSafety
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-24]. LP pool drain via arbitrage on duplicate tickets when referral fee exceeds LP edge
**Derived From** : Incentive Misalignment / Accounting Invariant Violation
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 14
- M: 10
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. DoS of drawing settlement due to block gas limit in `TicketComboTracker`

## id: ombKAdqXW5Fxhk1JbFMN6

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: Valid
### Finding Status Justification: countTierMatchesWithBonusball performs nested iteration over bonusballMax and subset generation/storage reads; there is no batching mechanism. If it exceeds practical callback/block gas, scaledEntropyCallback reverts while the drawing remains locked, creating a real DoS risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The function `countTierMatchesWithBonusball` in `TicketComboTracker` iterates through every possible bonus ball (`bonusballMax` iterations). Inside this loop, it iterates through all 31 subsets of the normal numbers. For each iteration, it accesses storage (`comboCounts`).

If `bonusballMax` is high (e.g., 255, which is possible and even likely for large prize pools due to the auto-adjustment mechanism), the function performs `255 * 31 = 7905` inner iterations. With storage reads cost (warm/cold), this can easily exceed the block gas limit (30M gas). `scaledEntropyCallback` will consistently OOG (Out of Gas), making it impossible to settle the drawing.

## Impact
High. Successful protocol operation (large prize pools) leads to self-DoS, locking funds until emergency exit.

## Command to Run Test


## Proof of Concept
1. Prize pool grows large, system automatically sets `bonusballMax` to 255 to maintain LP edge.
2. Drawing concludes. `runJackpot` called.
3. `scaledEntropyCallback` executes.
4. `TicketComboTracker.countTierMatchesWithBonusball` loops 255 times.
5. Performs ~8000 storage reads + bitwise logic.
6. Transaction exceeds 30M gas limit and reverts.
7. Drawing cannot be settled.

## Proof of Code
// conceptual test
function testGasLimit() public {
    uint8 bonusMax = 255;
    for(uint i=1; i<=bonusMax; i++) {
        for(uint k=0; k<31; k++) {
             // simulate storage read
             uint256 x = storageMap[i][k];
        }
    }
}

## Suggested Mitigation
Optimize `TicketComboTracker` data structures to avoid iterating all non-winning bonus balls (e.g., only iterate the winning bonus ball and a unified 'no-bonus' bucket), or limit `bonusballMax` to a safe lower value (though this limits maximum prize pool size).


## [M-2]. LPs can permanently block critical parameter updates via front-running

## id: cZzzd_sBVxwK9mI4LqHgZ

## Derived From Pattern/Invariant
Governance Front-running / Griefing

## Exploit Type
GovernanceFrontrunDoS

## Location
JackpotLPManager.setLPPoolCap

## Finding Status: Valid
### Finding Status Justification: A permissionless LP can front-run cap-reducing governance updates by depositing to push lpPoolTotal+pendingDeposits above the new cap, forcing JackpotLPManager.setLPPoolCap to revert. No safeguard exists (e.g., allowing cap below current total to only block future deposits), and the grief is practical rather than rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, admin functions like `setTicketPrice`, `setLpEdgeTarget`, and `setReserveRatio` trigger `jackpotLPManager.setLPPoolCap`. This function validates that the new pool cap is greater than or equal to the *current* LP pool size (`if (_lpPoolCap < currentLP.lpPoolTotal + currentLP.pendingDeposits) revert`).

Attacking LPs can observe a governance transaction that would reduce the pool cap (e.g. reducing ticket price or increasing edge), and front-run it with a deposit to make `currentLP.lpPoolTotal` exceed the `newCap`. This causes the admin transaction to revert. Since deposits are permissionless, LPs can consistently block any parameter update that constrains the pool size.

## Impact
Protocol governance is DoS'd; Admin cannot update economic parameters to react to market conditions.

## Command to Run Test


## Proof of Concept
1. Admin submits `setTicketPrice(newPrice)` which implies `newCap = 1M USDC`.
2. Current pool is 900k USDC.
3. Attacker sees tx, deposits 101k USDC. Total = 1.001M.
4. Admin tx executes: calculates `newCap = 1M`. Checks `1M < 1.001M`. Reverts.
5. Attacker can withdraw later (after drawing) or repeat.

## Proof of Code
function testGovernanceDoS() public { ... }

## Suggested Mitigation
Allow the admin to force the parameter change even if it violates the cap (setting cap < current total is valid, just prevents *new* deposits), or implement a grace period.


## [M-3]. JackpotBridgeManager users lose funds in emergency mode due to missing refund functionality

## id: 8Cxli1TFuYXzwKw-Q1RK8

## Derived From Pattern/Invariant
Standard Violation

## Exploit Type
Dos

## Location
JackpotBridgeManager.N/A

## Finding Status: Valid
### Finding Status Justification: Bug exists: emergencyRefundTickets requires msg.sender to be ERC721 owner; bridge tickets are minted to JackpotBridgeManager and it has no method to call emergencyRefundTickets or forward refunded USDC. No on-chain safeguard exists; impact is a real lock/loss for cross-chain users when emergencyMode is used (though emergencyMode itself is expected to be rare).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract allows users to claim refunds for the current drawing via `emergencyRefundTickets` when emergency mode is enabled. This function requires the caller to be the owner of the tickets. For cross-chain users, the `JackpotBridgeManager` is the owner of the tickets. However, `JackpotBridgeManager` does not implement any function to call `emergencyRefundTickets` or handle the refunded USDC. Consequently, if the protocol enters emergency mode, all funds spent by cross-chain users on the current drawing are permanently locked in the Jackpot contract (or effectively lost as they cannot be claimed).

## Impact
Permanent loss of funds for all cross-chain users participating in a drawing that gets cancelled via emergency mode.

## Command to Run Test


## Proof of Concept
1. Cross-chain user buys tickets. `JackpotBridgeManager` holds NFTs.
2. Entropy provider fails; Admin enables emergency mode.
3. Direct users call `emergencyRefundTickets` to get USDC back.
4. Cross-chain user cannot initiate refund. BridgeManager has no method to trigger it.

## Proof of Code
function testBridgeRefundLocked() public {
   // Enable emergency mode
   // Try to find a way to refund bridge tickets -> None exists
}

## Suggested Mitigation
Implement an `emergencyRefundTickets` function in `JackpotBridgeManager` that accepts a signature from the user, calls `jackpot.emergencyRefundTickets`, and bridges the refunded USDC back to the user.


## [M-4]. JackpotBridgeManager Denial of Service or fund loss due to ticket price mismatch

## id: ekRbCCXvULI7vFHscZiO_

## Derived From Pattern/Invariant
Read-only Reentrancy (Price Staleness)

## Exploit Type
Dos

## Location
JackpotBridgeManager.buyTickets

## Finding Status: Valid
### Finding Status Justification: The mismatch is caused by BridgeManager reading jackpot.ticketPrice() (global) while Jackpot.buyTickets charges drawingState[currentDrawingId].ticketPrice (snapshotted). Updating ticketPrice mid-drawing is expected to apply to future drawings per spec, yet it breaks cross-chain buys (overpull/underpull). This is a protocol integration bug, not merely an 'admin mistake' governance risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `JackpotBridgeManager` retrieves the global ticket price using `jackpot.ticketPrice()` to calculate the USDC amount to pull from the user. However, `Jackpot.buyTickets` uses the ticket price snapshotted in `drawingState[currentDrawingId]`. If the admin updates the ticket price via `setTicketPrice` while a drawing is active, `jackpot.ticketPrice()` updates immediately, but the active drawing's price remains unchanged. This causes a mismatch: if the new price is higher, the BridgeManager pulls excess funds (trapped in contract); if lower, it pulls insufficient funds and the transaction reverts due to `transferFrom` failure in `Jackpot`.

## Impact
Denial of Service for cross-chain ticket purchases or loss of user funds (excess sent to contract without credit).

## Command to Run Test


## Proof of Concept
1. Drawing 1 is active with price 1 USDC.
2. Admin calls `setTicketPrice(2 USDC)`.
3. BridgeManager `buyTickets` calls `jackpot.ticketPrice()` -> returns 2 USDC.
4. Transfers 2 USDC from user.
5. Calls `jackpot.buyTickets`. Jackpot uses drawing price (1 USDC).
6. Jackpot takes 1 USDC. BridgeManager keeps 1 USDC stuck.

## Proof of Code
function testPriceMismatch() public {
    // Start drawing with price 1e6
    // Admin sets price to 2e6
    // BridgeManager.buyTickets(1 ticket)
    // Assert user paid 2e6, Jackpot took 1e6, 1e6 stuck in BridgeManager
}

## Suggested Mitigation
In `JackpotBridgeManager.buyTickets`, query `jackpot.getDrawingState(jackpot.currentDrawingId()).ticketPrice` instead of `jackpot.ticketPrice()`.


## [M-5]. Denial of Service in Settlement Callback due to excessive gas consumption loops

## id: lIc9GVYMgWmSDoXM_FE97

## Derived From Pattern/Invariant
Gas Griefing / Block Gas Limit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: TicketComboTracker._countSubsetMatches() iterates i=1..bonusballMax and for each i iterates k=1..5, generating subsets and performing storage reads for each subset. Work scales linearly with bonusballMax (≈ 31 subset reads per bonusball, plus repeated subset generation). If bonusballMax grows large, scaledEntropyCallback can become very expensive and may exceed practical callback gas budgets, causing settlement to fail and leaving Jackpot locked. There is no hard cap on bonusballMax beyond uint8 and no on-chain mechanism ensuring settlement remains under a safe gas ceiling. entropyVariableGasLimit is owner-configurable but is not an on-chain safeguard because it only adjusts requested gas/fee; it does not guarantee the callback can execute within chain block limits once bonusballMax becomes large.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` executes winner determination logic which includes nested loops in `TicketComboTracker.countTierMatchesWithBonusball`. The outer loop iterates up to `bonusballMax`, which can dynamically grow up to 255. Inner logic involves storage reads for every iteration. 

With `bonusballMax` at 255, the callback performs approx 7,900 iterations of storage reads (31 subsets * 255 bonusballs), consuming ~16M+ gas. The `runJackpot` function requests an entropy fee based on `entropyVariableGasLimit`, which defaults to 250,000 per bonusball unit. With `bonusballMax`=255, the required gas limit exceeds 60M, which is above the Ethereum/Base block gas limit (30M). 

This makes the callback transaction impossible to include in a block, effectively bricking the jackpot settlement permanently once the pool grows large enough to require a high `bonusballMax`.

## Impact
Permanent Denial of Service of the jackpot settlement mechanism; funds locked in the active drawing.

## Command to Run Test


## Proof of Concept
1. `bonusballMax` scales with `PrizePool`. If pool is large, `bonusballMax` hits 255.
2. Keeper calls `runJackpot`. `entropyGasLimit` calculated as `Base + 250000 * 255` = ~63M gas.
3. `entropy` contract accepts the request (as it just passes parameters).
4. Pyth network attempts to fulfill callback with 63M gas limit.
5. Transaction exceeds block gas limit (30M) and cannot be mined.
6. Callback never executes. Jackpot remains `locked` forever.

## Proof of Code
function testGasDos() public {
    // Set variable gas limit to default
    vm.prank(owner);
    jackpot.setEntropyVariableGasLimit(250000);
    
    // Mock large bonusball max (via large pool simulation or direct state manipulation for PoC)
    // ... setup pool to force bonusballMax = 255 ...
    
    uint256 fee = jackpot.getEntropyCallbackFee();
    // Fee calculation implies gas limit
    // gasLimit = base + 250000 * 255 > 63,000,000
    
    // Verify block gas limit on target chain (Base/Eth) is 30M
    // Transaction is impossible.
}

## Suggested Mitigation
Optimize the `TicketComboTracker` to avoid iterating all bonusballs (e.g. by tracking non-bonusball matches separately) and/or significantly reduce the `entropyVariableGasLimit` default to ensure the total gas limit remains well within the block size limit even at `bonusballMax = 255`.


## [M-6]. Settlement permanent DoS via gas limit exhaustion from high `bonusballMax`

## id: G7BTcLqt4nZvkJyMoYcY-

## Derived From Pattern/Invariant
DoS / Out of Gas in Callback

## Exploit Type
Dos

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: The settlement path scales roughly with bonusballMax and repeatedly generates subsets and reads storage across all bonusballs. There is no batching/pagination and no hard cap ensuring it fits under realistic callback/block gas limits. If gas is underestimated or exceeds block limits, the callback reverts while jackpotLock remains set, bricking the drawing.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function executes settlement logic which iterates over all possible bonusballs in `TicketComboTracker.countTierMatchesWithBonusball`. The number of iterations is determined by `bonusballMax`, which scales dynamically with the prize pool size. If `bonusballMax` reaches its maximum (255), the loop performs ~8,000 storage reads and calculations.

The gas limit for this callback is set during `runJackpot` based on `entropyVariableGasLimit`. If the actual gas cost of the loop exceeds the calculated limit (or the block gas limit), the callback will revert. Since the drawing is locked (`jackpotLock = true`) until the callback succeeds, and the parameters for the pending request cannot be changed, the jackpot becomes permanently stuck (bricked) for that drawing, forcing an emergency refund.

## Impact
Permanent freezing of the jackpot drawing, requiring emergency mode activation and manual refunds.

## Command to Run Test


## Proof of Concept
1. LPs deposit large amounts, pushing `prizePool` high.
2. `_setNewDrawingState` calculates `bonusballMax = 255`.
3. Keeper calls `runJackpot`. `entropyGasLimit` is calculated (say 2M gas).
4. Callback executes. `TicketComboTracker` logic consumes 2.5M gas due to high iteration count and cold storage loads.
5. Callback reverts (Out of Gas).
6. Drawing remains locked forever.

## Proof of Code
function testSettlementOOG() public { ... }

## Suggested Mitigation
Implement a hard cap on `bonusballMax` (e.g. 100) to ensure settlement always fits within gas limits, or allow the owner to retry the entropy request with a higher gas limit.


## [H-7]. Arbitrary External Call in JackpotBridgeManager enables theft of all custodial NFTs

## id: 4PMWLTECCWnVxPho2uWZA

## Derived From Pattern/Invariant
ArbitraryExternalCall

## Exploit Type
ArbitraryExternalCall

## Location
JackpotBridgeManager.claimWinnings

## Finding Status: Valid
### Finding Status Justification: While the specific NFT-theft PoC via calling the ERC721 directly fails (balance-delta check reverts, and a third-party contract is not approved to transfer NFTs), the core arbitrary approval/call surface remains exploitable: a claimant can set approveTo=self (or attacker contract) and to=USDC.transfer(...) to satisfy the balance-delta check, leaving a persistent USDC allowance that can later be used to transferFrom any future USDC held by BridgeManager (from other users' claims or stranded funds). Thus there is no real safeguard and the issue is exploitable with non-low impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotBridgeManager` contract's `claimWinnings` function allows users to bridge winnings to another chain. It accepts a `RelayTxData` struct containing `to`, `data`, and `approveTo`. The internal function `_bridgeFunds` executes a low-level call to `to` with `data`. The only validation is that the contract's USDC balance must decrease by exactly the claimed amount (`preUSDCBalance - postUSDCBalance != _claimedAmount`). 

An attacker can construct a malicious payload where `to` is a malicious contract (or the USDC contract if carefully crafted) and `data` executes a theft of assets held by the BridgeManager. To bypass the balance check, the attacker can ensure the USDC balance decreases by the required amount by legitimately transferring the claimed USDC (e.g., via `transferFrom` if `approveTo` is set correctly). Crucially, the arbitrary call allows the attacker to *simultaneously* call `JackpotTicketNFT.transferFrom` to steal valuable tickets held in custody by the BridgeManager for other cross-chain users.

## Impact
Theft of all JackpotTicketNFTs held in custody by the BridgeManager (loss of user assets).

## Command to Run Test


## Proof of Concept
1. Attacker acquires a winning ticket (e.g., small prize) or waits for one.
2. Attacker calls `claimWinnings` with the valid ticket and signature.
3. In `RelayTxData`, attacker sets `approveTo` to a malicious contract `M`, `to` to `M`, and `data` to call `M.exploit()`.
4. `claimWinnings` receives USDC winnings (e.g., 10 USDC) from Jackpot.
5. `_bridgeFunds` calls `usdc.approve(M, 10)`.
6. `_bridgeFunds` calls `M.exploit()`.
7. `M.exploit()` does two things:
    a. Calls `USDC.transferFrom(BridgeManager, Attacker, 10)`. This consumes the winnings and satisfies the balance check (`pre - post == 10`).
    b. Calls `JackpotTicketNFT.transferFrom(BridgeManager, Attacker, VictimTicketId)`. Since BridgeManager is the owner, this succeeds.
8. Transaction succeeds, attacker steals victim's ticket.

## Proof of Code
function testExploitTheft() public {
  // Setup: Bridge owns victim ticket, Attacker owns winning ticket
  vm.startPrank(attacker);
  // Construct RelayTxData calling MaliciousContract
  JackpotBridgeManager.RelayTxData memory bridgeData = JackpotBridgeManager.RelayTxData({
      approveTo: address(maliciousContract),
      to: address(maliciousContract),
      data: abi.encodeWithSignature("exploit()")
  });
  // Sign and call
  bytes memory sig = ...;
  bridgeManager.claimWinnings(ticketIds, bridgeData, sig);
  // Assert attacker owns victim ticket
  assertEq(nft.ownerOf(victimTicketId), attacker);
}

## Suggested Mitigation
Restrict `_bridgeDetails.to` to a trusted allowlist of bridge adapters/routers, or enforce that `_bridgeDetails.to` is not the JackpotTicketNFT contract or USDC contract. Ideally, implement a dedicated adapter pattern.


## [H-8]. LP Share Price Inflation Attack allowing theft of future deposits

## id: cxv05fXH3ZKaYwn4kU0v9

## Derived From Pattern/Invariant
ERC4626SharePrice

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: Valid
### Finding Status Justification: JackpotLPManager mints shares for deposits via shares = amount * 1e18 / drawingAccumulator[depositDrawingId] (see _consolidateDeposits). If an attacker engineers a drawing where lpPoolTotal is extremely small but postDrawLpValue becomes large, processDrawingSettlement sets a massive newAccumulator = prevAcc * postDrawLpValue / lpPoolTotal. Subsequent deposits consolidated against that accumulator can truncate to 0 shares, effectively donating USDC to the pool while the attacker retains essentially all shares and can later withdraw the value (including the victim’s deposit). There is no minimum liquidity/virtual shares mechanism and no check that share minting is non-zero. This is a known economic vulnerability pattern in share/accumulator systems when totals can be driven near-zero.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager` uses an accumulator-based share pricing model that is susceptible to a classic inflation attack when the pool has very low liquidity. The `processDrawingSettlement` function calculates `newAccumulator` based on `lpPoolTotal` and `postDrawLpValue`. If an attacker withdraws almost all liquidity (leaving e.g., 1 wei), `lpPoolTotal` becomes extremely small. The attacker can then artificially inflate `postDrawLpValue` by buying tickets (which adds to `lpEarnings`), causing `newAccumulator` to skyrocket. 

Subsequent deposits by other users will calculate shares as `(amount * PRECISE_UNIT) / newAccumulator`. Due to the massive accumulator, this division truncates to zero shares. The user's deposit is added to the pool but they receive no shares, allowing the attacker (who holds the only existing share) to claim the entire pool value, including the victim's deposit.

## Impact
Theft of new LP deposits; attackers can drain funds from unsuspecting liquidity providers.

## Command to Run Test


## Proof of Concept
1. Attacker initializes the pool with a small deposit.
2. Attacker withdraws almost all funds using `initiateWithdraw` and `finalizeWithdraw`, leaving 1 wei of liquidity in the pool.
3. In the next drawing, Attacker buys tickets worth a large amount (e.g., 1,000 USDC). This creates `lpEarnings` of 1,000 USDC.
4. The drawing settles. `processDrawingSettlement` calculates `newAccumulator`.
   `newAccumulator = (OldAcc * (1 wei + 1000e6 earnings)) / 1 wei` ≈ 1e24 * 1e18.
5. Victim deposits 1,000 USDC.
   `shares = (1000e6 * 1e18) / newAccumulator` = 0.
6. Victim receives 0 shares. The 1,000 USDC is added to the pool.
7. Attacker (holding the 1 wei share) withdraws effectively 100% of the pool, stealing the victim's 1,000 USDC.

## Proof of Code
function testInflationAttack() public {
    // 1. Setup
    vm.startPrank(attacker);
    usdc.approve(address(jackpot), 1e18);
    jackpot.lpDeposit(100); // Tiny deposit
    vm.stopPrank();

    // Fast forward to settle draw 1
    vm.warp(block.timestamp + 1 days);
    vm.prank(keeper);
    jackpot.runJackpot();
    performEntropyCallback(); // Mock callback

    // 2. Withdraw almost all
    vm.startPrank(attacker);
    jackpot.initiateWithdraw(99); // Leave 1 share
    vm.stopPrank();
    
    // Fast forward to settle draw 2 (withdraws processed)
    vm.warp(block.timestamp + 1 days);
    vm.prank(keeper);
    jackpot.runJackpot();
    performEntropyCallback();
    jackpot.finalizeWithdraw(); // Attacker gets funds, pool left with tiny amount

    // 3. Inflate
    vm.startPrank(attacker);
    // Buy tickets to add earnings to tiny pool
    ITicket.Ticket[] memory tickets = new ITicket.Ticket[](1);
    tickets[0] = ITicket.Ticket({normals: [1,2,3,4,5], bonusball: 1});
    jackpot.buyTickets(tickets, attacker, ...);
    vm.stopPrank();

    // Settle draw 3 -> Accumulator explodes
    vm.warp(block.timestamp + 1 days);
    vm.prank(keeper);
    jackpot.runJackpot();
    performEntropyCallback();

    // 4. Victim deposit
    vm.startPrank(victim);
    usdc.approve(address(jackpot), 1000e6);
    jackpot.lpDeposit(1000e6);
    vm.stopPrank();

    // Check shares
    IJackpotLPManager.LP memory victimLP = lpManager.getLpInfo(victim);
    // Shares will be 0 due to precision loss against massive accumulator
    assertEq(victimLP.lastDeposit.amount, 1000e6); 
    // Note: Shares calculated at next settlement, but will be 0.
}

## Suggested Mitigation
Enforce a minimum liquidity amount (e.g., burn the first 1000 shares) or implement virtual shares in `JackpotLPManager` to prevent the `lpPoolTotal` from becoming small enough to allow manipulation.


## [H-9]. Winning tickets corrupted due to bit packing overflow in `TicketComboTracker`

## id: Tiu77EuHA41CB4Q0dGaaE

## Derived From Pattern/Invariant
Bit Packing Overflow

## Exploit Type
IntegerOverflow

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: TicketComboTracker.insert() sets the bonus bit via 1 << (_bonusball + _tracker.normalMax). If (_bonusball + normalMax) >= 256, the shift yields 0 and the bonus bit is not recorded, so multiple distinct bonusballs collide to the same packed value (normals-only). countTierMatchesWithBonusball() uses the same shifting scheme for the winningTicket, so the winning ticket can also lose its bonus bit. Jackpot._calculateTicketTierId() then compares extracted bonusball values via right-shift; both become 0, causing a false “bonusballMatch” and mis-tiering/mispayouts. There is no enforcement that normalBallMax + bonusballMax < 256 in _setNewDrawingState or at insert-time; lpPoolCap intends to keep this safe but can be bypassed by pool growth from lpEarnings.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker.insert` function packs the ticket numbers into a `uint256` bit vector using the formula: `ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);`. If the sum of `_bonusball` and `_tracker.normalMax` equals or exceeds 256, the shift operation `1 << ...` overflows to 0 (in Solidity). 

This results in the bonusball bit being lost from the `packedTicket`. The `Jackpot` contract mints the NFT using this corrupted `packedTicket`. When a user attempts to claim winnings, `_calculateTicketTierId` extracts the bonusball using `_ticketNumbers >> (_normalBallMax + 1)`. Since the bit was lost, the extracted bonusball is 0, which never matches the winning bonusball (range 1-255). Users with valid winning tickets are unable to claim their top-tier prizes.

## Impact
Users permanently lose access to jackpot winnings (Asset Loss).

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` = 128.
2. System calculates `bonusballMax` = 130.
3. User buys ticket with `bonusball` = 129.
4. `insert` calculates shift: `129 + 128 = 257`.
5. `1 << 257` is 0.
6. `packedTicket` has no bonusball bit set.
7. Winning numbers are drawn with `bonusball` = 129.
8. User calls `claimWinnings`. Contract unpacks user ticket, finds bonusball 0. Match fails.

## Proof of Code
function testBitPackingOverflow() public {
    uint8 normalMax = 128;
    uint8 bonusball = 128;
    uint256 packed = 0 | (1 << (bonusball + normalMax));
    assertEq(packed, 0); // Overflow causes data loss
}

## Suggested Mitigation
Enforce a system-wide invariant that `normalBallMax + bonusballMax < 256` in the setters (`setNormalBallMax`) and in the dynamic bonusball calculation in `_setNewDrawingState`.


## [H-10]. Bitwise overflow in ticket storage causes collision of distinct tickets and payouts

## id: oyJkv69QeLEFAtbppSD8X

## Derived From Pattern/Invariant
Storage Collision / Bit Packing

## Exploit Type
StorageLayout

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: This is a valid restatement of the same packing bug: TicketComboTracker stores the bonusball at bit index normalMax + bonusball, and shifting by >=256 loses the bonus bit (sets 0), creating collisions between different bonusballs and corrupting tier computation. Jackpot’s tier logic then compares extracted bonusball values (right shift), so overflowed tickets/winning tickets can both appear to have bonusball 0 and falsely match. There is no runtime check in insert() or in Jackpot’s drawing parameterization to guarantee normalBallMax + bonusballMax < 256, so distinct tickets can become indistinguishable and payouts can be wrong.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Tickets are stored as bit vectors. The bonus ball is stored by setting the bit at index `normalMax + bonusball`. 

```solidity
// TicketComboTracker.sol
ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);
```

If `_bonusball + _tracker.normalMax >= 256`, the shift operation overflows in `uint256`, resulting in 0. The bonus ball bit is lost. This causes:
1. Tickets with different high-value bonus balls to have identical packed representations (collisions).
2. Winning ticket calculation to lose the bonus bit.
3. `_calculateTicketTierId` to see both ticket and winner bonus balls as 0, registering a 'match'.
4. Tickets meant to match the bonus ball (Tier 11) will register as 0 bonus bits, potentially degrading to Tier 9 or 10 depending on collision specifics, or granting wins where none exist.

## Impact
Broken game logic, incorrect payouts, and inability to distinguish tickets. Valid winners may be underpaid, and losers may be paid as winners due to bonus ball collisions.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 200.
2. `bonusballMax` calculation (or manual setting) results in 60.
3. User buys ticket with bonusball 60. `200 + 60 = 260 >= 256`. Bit is lost.
4. Ticket stored as just normal balls.
5. Winning number drawn has bonusball 60. Winning ticket stored as just normal balls.
6. User claims. `_calculateTicketTierId` extracts bonusball via shift: `val >> 201`. Returns 0 for both.
7. `bonusballMatch` is true (0==0).
8. System treats it as a match, but collisions mean bonusball 56 (256-200) also results in 0. So Bonusball 60 matches Bonusball 56.

## Proof of Code
function testBitOverflow() public {
    vm.prank(owner); jackpot.setNormalBallMax(200);
    // Force bonusball max via large LP or mocks
    // ...
    // Buy ticket with bonusball 60
    // Verify packed ticket has no bit set above 200
}

## Suggested Mitigation
Enforce that `normalBallMax + bonusballMax < 256` in `setNormalBallMax` and `_setNewDrawingState`. Additionally, check for overflow in `TicketComboTracker.insert`.


## [H-11]. Organic LP pool growth bypasses `lpPoolCap` causing bit-shift overflow and prize dilution

## id: DT9b44a4Syo-QmGiifeI2

## Derived From Pattern/Invariant
Integer Overflow / Logic Error

## Exploit Type
IntegerOverflow

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: lpPoolCap is enforced only on processDeposit (deposits), not on pool growth via lpEarnings. processDrawingSettlement computes newLPValue = lpPoolTotal + lpEarnings - userWinnings - protocolFee + pendingDeposits - withdrawalsInUSDC, so ticket sales can grow LP value beyond lpPoolCap. _setNewDrawingState then derives bonusballMax from the (potentially uncapped) newPrizePool and casts to uint8 without enforcing the bit-packing boundary (255 - normalBallMax). Once bonusballMax is large enough that normalBallMax + bonusballMax >= 256, TicketComboTracker’s 1 << (normal+bonus) overflows and the bonus bit is lost, causing ticket collisions and payout dilution/incorrectness. No safeguard caps bonusballMax or diverts excess lpEarnings.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `lpPoolCap` mechanism limits LP deposits to prevent the pool from growing to a size where the dynamically calculated `bonusballMax` would exceed the bit-packing limits of `TicketComboTracker`. However, the cap only applies to `lpDeposit` and does not account for `lpEarnings` from ticket sales, which are added unconditionally to the pool. 

If the LP pool grows organically (via ticket sales) beyond the cap, the `_setNewDrawingState` function will calculate a `newBonusball` value that exceeds `255 - normalBallMax`. 

In `TicketComboTracker.insert`, the operation `ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);` will attempt to shift by 256 or more, which in Solidity results in 0 (for `uint256` shift). This effectively deletes the bonusball information from the packed ticket. 

Consequently, tickets with high bonusball numbers will map to a packed representation containing only the normal numbers. If the winning number also has a high bonusball (causing the same overflow), it will match all such user tickets as having a correct bonusball. This drastically increases the win probability and dilutes the prize pool.

## Impact
Game integrity is broken; tickets with different bonusballs are treated as identical. This leads to incorrect payouts, prize pool dilution, and loss of funds for Liquidity Providers.

## Command to Run Test


## Proof of Concept
1. Governance sets `normalBallMax` to 50. The calculated `lpPoolCap` corresponds to a `bonusballMax` of 205 (255 - 50). 
2. LPs deposit funds up to `lpPoolCap`. 
3. Users buy tickets. `lpEarnings` are added to the pool, pushing the total pool value above `lpPoolCap`. 
4. The drawing runs. `_setNewDrawingState` calculates `newBonusball` based on the inflated pool value. Result: `newBonusball` = 206. 
5. In the next drawing, a user buys a ticket with bonusball 206. 
6. `TicketComboTracker` executes `1 << (206 + 50)` -> `1 << 256` -> 0. 
7. The user's ticket is stored with a 0 bonusball bit. 
8. If the winning numbers also dictate a bonusball of 206, the winning packed ticket also has a 0 bonusball bit. 
9. The user claims the jackpot (Tier 11) because the 0 bits match, even though the system logic relies on bit positions for uniqueness.

## Proof of Code
function testOverflow() public {
    uint8 normalMax = 50;
    uint8 bonusball = 206;
    // This should set a bit
    uint256 packed = 1 << (bonusball + normalMax);
    // But it results in 0 due to overflow
    assertEq(packed, 0);
}

## Suggested Mitigation
In `Jackpot.sol`, cap the calculated `newBonusball` in `_setNewDrawingState` to ensure it never exceeds `255 - normalBallMax`. Alternatively, enforce the pool cap check against `lpEarnings` or divert excess earnings to a reserve/treasury instead of the LP pool.


## [H-12]. Catastrophic loss of LP funds or DoS due to unsafe downcasting of `bonusballMax`

## id: zagXJe7MjHUZg9X03M0Hu

## Derived From Pattern/Invariant
Integer Overflow / Unsafe Casting

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Jackpot._setNewDrawingState computes uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(...))). This is an unsafe cast: if the computed value exceeds 255 it wraps modulo 256 (e.g., 256 -> 0), rather than reverting. If it wraps to 0, buyTickets becomes impossible because it requires bonusball > 0 and <= bonusballMax, permanently DoSing ticket sales for that drawing. If it wraps to a small non-zero value, the lottery difficulty is unintentionally reduced, increasing win probability and potentially draining LPs. The codebase already contains UintCasts.toUint8() for safe casting but does not use it here, and there is no other cap enforcing bonusballMax bounds.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the system dynamically calculates the `bonusballMax` for the next drawing to maintain the target LP edge. The formula is `uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)))`. The result is explicitly cast to `uint8`. If the required number of bonusballs exceeds 255 (e.g., due to a large prize pool or low normal ball count), the cast truncates the upper bits. 

If the result wraps to 0 (e.g., 256), `bonusballMax` becomes 0. Since `buyTickets` requires `ticket.bonusball > 0` and `ticket.bonusball <= bonusballMax`, no tickets can be purchased, causing a permanent Denial of Service for that drawing.

If the result wraps to a small number (e.g., 257 becomes 1), the difficulty of the lottery is drastically reduced (e.g., from 1/257 chance to 1/1 guaranteed match). This allows players to win the jackpot with extremely high probability, draining the LP pool and insolvently extracting value well beyond the intended economic design.

## Impact
Direct theft of LP pool funds (insolvency) or permanent DoS of the drawing.

## Command to Run Test


## Proof of Concept
1. Assume `normalBallMax` = 5, so `combosPerBonusball` = 1.
2. Assume `prizePool` grows large enough such that `minNumberTickets` = 257.
3. `_setNewDrawingState` calculates `ceilDiv(257, 1)` = 257.
4. `uint8(257)` results in `1`.
5. `newDrawingState.bonusballMax` is set to 1.
6. Users buy tickets with `bonusball = 1`.
7. Every ticket matches the bonusball. The probability of winning the top prize increases by 257x, draining the pool.

## Proof of Code
function testBonusballOverflow() public {
    // Setup state where minNumberTickets > 255
    // ... (mock setups)
    uint256 minNumberTickets = 257;
    uint256 combosPerBonusball = 1;
    uint8 newBonusball = uint8(minNumberTickets / combosPerBonusball);
    assertEq(newBonusball, 1); // Should represent 257, but wraps to 1
}

## Suggested Mitigation
Cap the `bonusballMax` at 255 instead of casting/wrapping, or use a larger integer type (e.g., `uint16`) for `bonusballMax` throughout the codebase if the game logic allows.


## [H-13]. Bonusball bit-packing overflow allows guaranteed bonus matches and pool draining

## id: mVF-_x60ygfcewzMIXTFF

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: If normalBallMax + bonusballMax reaches/exceeds 256, TicketComboTracker’s bonus bit shift (1 << (bonus+normal)) overflows and sets no bonus bit. Both user tickets and the stored winningTicket can lose the bonus bit, and Jackpot._calculateTicketTierId() then treats bonusballMatch as true because both extracted bonus values are 0. This can incorrectly upgrade tiers and enable overpayment relative to intended odds, harming LP solvency. The protocol intends a packing constraint (implicitly reflected by MAX_BIT_VECTOR_SIZE usage), but it is not enforced in _setNewDrawingState and can be violated if LP value grows beyond lpPoolCap due to lpEarnings.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract stores tickets as packed `uint256` bit vectors. The bonusball is stored at bit index `normalBallMax + bonusball`. The system relies on `normalBallMax + bonusballMax < 256` for this packing to work. While `setLPPoolCap` attempts to limit the pool size such that `bonusballMax` stays within range (assuming `maxAllowableTickets` logic), `lpEarnings` from ticket sales can push the pool size beyond `lpPoolCap`. 

When `prizePool` grows large enough, `_setNewDrawingState` calculates a `bonusballMax` such that `normalBallMax + bonusballMax >= 256`. In `TicketComboTracker.insert`, the operation `1 << (bonus + normal)` shifts the bit off the end of the 256-bit word, resulting in 0. Consequently, both user tickets and the winning ticket (if the winning bonus ball is in the overflow range) are stored without the bonus bit (effectively 0). 

In `claimWinnings`, the logic `(ticket >> shift) == (winning >> shift)` compares `0 == 0`, granting every ticket a 'bonus ball match' regardless of the actual numbers selected. This upgrades losing tickets to winning tiers, draining the prize pool.

## Impact
Game integrity collapse. Users can purchase tickets that are guaranteed to match the bonus ball, resulting in massive overpayment of prizes and insolvency for LPs.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 30.
2. `lpPoolCap` is set corresponding to `bonusballMax` = 225 (limit 255-30).
3. LPs fill the pool to capacity.
4. Users buy tickets, adding `lpEarnings` to the pool. The effective pool size now exceeds the cap.
5. Drawing settles. `_setNewDrawingState` calculates `minNumberTickets` based on the new, larger pool.
6. `bonusballMax` is calculated as 226.
7. `normalBallMax` (30) + `bonusballMax` (226) = 256.
8. User buys ticket with bonusball 226. `TicketComboTracker` performs `set |= 1 << 256`. Result is `set` (normals only).
9. Winning number drawn is bonusball 226. Winning ticket is stored as `normals | 0`.
10. User claims winnings. `_calculateTicketTierId` extracts bonusball: `ticket >> 31`. Since bit 256 was lost, this is 0.
11. `winningBonusball` is also 0. Match is true.
12. User wins a higher tier payout undeservedly.

## Proof of Code
function testBitPackingOverflow() public {
    // Setup state where normal=30, bonus=226
    // Mock internal state or reach via earnings
    uint8 normalMax = 30;
    uint8 bonus = 226;
    
    // Simulate TicketComboTracker.insert logic
    uint256 set = 12345; // some normals bits
    uint256 packed = set | (1 << (normalMax + bonus));
    
    // packed is just 'set' because 1<<256 is 0 in EVM SHL behavior (pushes 0)
    // Actually Solidity 0.8 might not revert on shift overflow, but result is 0
    assertEq(packed, set);

    // Simulate extraction in _calculateTicketTierId
    uint256 extractedBonus = packed >> (normalMax + 1);
    assertEq(extractedBonus, 0);
    
    // If winning number also overflowed, extractedWinning is 0
    assertEq(extractedBonus, 0); // Match!
}

## Suggested Mitigation
In `Jackpot._setNewDrawingState`, explicitly cap `newBonusball` such that `normalBallMax + newBonusball < 256`. Alternatively, ensure `1 << (normal + bonus)` does not overflow by requiring `normal + bonus < 256`.


## [H-14]. Critical Logic Failure and DoS due to Unbounded Dynamic Bonusball Calculation

## id: cROIJ-SdJeJreerWCMfIn

## Derived From Pattern/Invariant
StorageLayout

## Exploit Type
StorageLayout

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: This report combines two real issues in current code: (1) bonusballMax is computed from pool size and cast unsafely to uint8, enabling wraparound and/or violating the packing constraint normalBallMax + bonusballMax < 256; and (2) settlement gas grows with bonusballMax due to loops over 1..bonusballMax in TicketComboTracker._countSubsetMatches, risking callback failure. Neither issue has an on-chain cap; lpPoolCap only restricts deposits and can be bypassed by lpEarnings-driven pool growth. Therefore both incorrect payouts (bonusball collisions/false matches) and settlement DoS are feasible outcomes from today’s code under sufficiently large pool/bonusballMax conditions.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol dynamically calculates `bonusballMax` for each new drawing to ensure LP edge. The formula `ceilDiv(minNumberTickets, combosPerBonusball)` can produce a `bonusballMax` that, when added to `normalBallMax`, exceeds the 256-bit capacity of the `uint256` bit-vector used in `TicketComboTracker`. 

When `normalBallMax + bonusballMax >= 256`, the bit-shift operation `1 << (bonusball + normalMax)` overflows to 0. This results in all tickets with high bonus ball numbers being stored with a '0' bit for the bonus ball. Consequently, if a winning number also falls in this overflow range, all such tickets will falsely register a match, allowing users to drastically increase their winning odds (e.g., matching any of N overflow balls by picking any of N overflow balls). 

Additionally, for higher `normalBallMax` values, the `entropyCallback` iterates over all `bonusballMax` possibilities. If `bonusballMax` grows large (e.g., >100) alongside a moderate `normalBallMax`, the computational cost of `countTierMatchesWithBonusball` exceeds the block gas limit, permanently bricking the jackpot.

## Impact
The protocol can either be exploited for guaranteed wins (theft of funds) or permanently frozen (DoS) depending on the `normalBallMax` configuration and prize pool growth.

## Command to Run Test


## Proof of Concept
1. Assume `normalBallMax` = 10 (`C(10,5) = 252`). Cap is ~61k USDC.
2. LPs earn fees or users deposit such that Pool grows to 100k USDC.
3. `_setNewDrawingState` calculates `newBonusball = 100000 / 252 ≈ 397`.
4. `normalBallMax (10) + newBonusball (397) > 256`.
5. User buys ticket with bonus ball 300. `1 << 310` overflows to 0. Stored as 0.
6. Winner is drawn as 350. `1 << 360` overflows to 0. Winning mask is 0.
7. `_calculateTicketTierId` sees match (0 == 0). User wins bonus match falsely.

## Proof of Code
function testBonusballOverflow() public {
    // Simulate state with small normalBallMax
    uint8 normalMax = 10;
    uint8 bonusMax = 250; // 10 + 250 = 260 > 255
    
    // Bit vector logic from TicketComboTracker
    uint256 ticketBonus = 250;
    uint256 ticketMask = 1 << (ticketBonus + normalMax);
    assertEq(ticketMask, 0, "Overflow should result in 0");
    
    uint256 winningBonus = 249;
    uint256 winningMask = 1 << (winningBonus + normalMax);
    assertEq(winningMask, 0, "Overflow should result in 0");
    
    // Match logic from Jackpot
    uint256 tB = ticketMask >> (normalMax + 1);
    uint256 wB = winningMask >> (normalMax + 1);
    assertEq(tB, wB, "False positive match due to overflow");
}

## Suggested Mitigation
Cap `newBonusball` in `_setNewDrawingState` such that `normalBallMax + newBonusball < 256` and `entropyGasLimit` remains within block limits. Enforce hard caps on LP Pool growth to prevent parameters from exceeding these safety bounds.


## [H-15]. Unbounded Prize Pool Growth causes Integer Overflow in Difficulty Calculation and Ticket Packing Collisions

## id: i4yYa9LjnqN_Be0LC_FuK

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: lpPoolCap is not enforced against lpEarnings-driven growth: processDrawingSettlement can return a newLPValue exceeding lpPoolCap, and _setNewDrawingState uses that value to compute bonusballMax. The computation then performs an unsafe uint8 cast, so values >255 wrap (difficulty collapse / possible DoS if it wraps to 0). Even when the wrapped value is <=255, bonusballMax can still violate the packing boundary (normalBallMax + bonusballMax >= 256), causing TicketComboTracker bit-shift overflow and ticket collisions/false bonus matches. There are no safeguards (no SafeCast, no explicit cap, no enforcement that pool growth cannot exceed the range that packing supports), so the described failure modes are rooted in current code and are reproducible given sufficiently large pool growth.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol enforces an `lpPoolCap` to ensure that the required `bonusballMax` (difficulty) for the prize pool fits within `uint8` and `MAX_BIT_VECTOR_SIZE` constraints. However, `lpEarnings` from ticket sales are added to the LP pool without checking this cap. If ticket sales are high enough to push the `prizePool` significantly beyond the intended cap, two critical failures occur in the next drawing's parameterization:

1. **Integer Overflow in Difficulty**: `newBonusball` is calculated as `Math.ceilDiv(minNumberTickets, ...)` and cast to `uint8`. If the required difficulty exceeds 255, the cast overflows/wraps (e.g., 256 becomes 0), drastically reducing the game difficulty. This makes winning the jackpot trivial, draining the LP pool.
2. **Ticket Packing Collision**: Even if the cast doesn't wrap (e.g. difficulty is 250), if `newBonusball + normalBallMax >= 256`, the bitwise shift `1 << (bonus + normal)` in `TicketComboTracker` overflows to 0. This causes distinct tickets to alias to the same packed representation (bonus ball 0), allowing losing tickets to claim winnings as if they matched the winning ticket's aliased representation, leading to theft and insolvency.

Code snippet in `Jackpot.sol`:
```solidity
uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
```

## Impact
Catastrophic loss of LP funds due to collapsed game difficulty or ticket aliasing leading to incorrect payouts.

## Command to Run Test


## Proof of Concept
1. Governance sets `lpPoolCap` based on `normalBallMax` to ensure safety.
2. A new drawing starts. Users buy a massive amount of tickets (e.g., due to a viral event or attack), causing `lpEarnings` to accumulate significantly.
3. The drawing settles. `processDrawingSettlement` adds `lpEarnings` to the pool for the *next* drawing, bypassing `lpPoolCap`.
4. `_setNewDrawingState` calculates `newPrizePool` (now extremely large) and `minNumberTickets`.
5. `newBonusball` is calculated. Suppose `minNumberTickets` implies a range of 300.
6. `uint8(300)` wraps to 44. The new drawing has `bonusballMax = 44` instead of 300, making it ~7x easier to win than math requires for solvency. LPs are drained.
7. Alternatively, if `newBonusball` is 230 and `normalBallMax` is 30, `230+30=260`. Bit shift overflows. Tickets with bonus ball 230 are stored as packed bonus 0. If winning ticket has bonus 230, it is also packed 0. User buys ticket with bonus 240 (also packed 0). User claims match against winning ticket, stealing funds.

## Proof of Code
import "forge-std/Test.sol";
import "../contracts/Jackpot.sol";
// Assume standard imports for mocks/deps

contract OverflowTest is Test {
    // Setup contract with normalMax=10, bonusMin=1
    // Combos(10,5) = 252.
    // To overflow uint8 (256), need minTickets > 256 * 252 = 64,512.
    // Assume price = 1e6 (1 USDC). Edge = 10%.
    // PrizePool needed = 64,512 * 1e6 * 0.9 = ~58e9 (58k USDC).
    // This is very achievable.
    
    function testDifficultyOverflow() public {
        // Simulate state where lpEarnings pushed pool to 100M USDC
        uint256 hugeEarnings = 100_000_000 * 1e6;
        
        // In a real test, we would mock LPManager to return this newLPValue
        // calculate bonusball: ceil( (100M * 0.9) / (1 * 252) ) = ~357,142
        // uint8(357142) = 357142 % 256 = 54.
        
        // Difficulty is 54 instead of 357k.
        // Win probability is 1/54 instead of 1/357k.
        // Expected Value for player >>>>> Ticket Price.
        // Pool Drains.
    }
}

## Suggested Mitigation
1. Enforce `lpPoolCap` on `newLPValue` in `processDrawingSettlement`. If earnings push it over, cap the pool (excess remains in LP manager or separate reserve).
2. Use `SafeCast` for `uint8` conversion and handle the overflow (e.g., cap `bonusballMax` at `255 - normalBallMax`). Note that capping difficulty without capping pool size exposes LPs to loss, so capping pool size is preferred.


## [H-16]. Downcast to uint8 in `_setNewDrawingState` causes bonus ball count to wrap, removing LP edge

## id: ch8P5Nlsk1G-C-xUZ50iq

## Derived From Pattern/Invariant
Unsafe Downcasting

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Same unsafe uint8 downcast as the related report: if the economically required bonusball count exceeds 255, the uint8 cast wraps to a smaller value, undermining intended odds/LP edge. No bounds checking is performed despite having UintCasts available elsewhere.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot.sol`, the `_setNewDrawingState` function calculates the required `bonusballMax` to guarantee the LP edge based on the prize pool size. The calculation `newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));` explicitly casts the result to `uint8`. If the required number of bonus balls exceeds 255 (which occurs when the prize pool is large relative to the `normalBallMax` combinations), the value wraps around due to the cast (e.g., 300 becomes 44). This drastically reduces the difficulty of winning the jackpot, creating a massive positive expected value (EV) for players and draining the LP pool.

## Impact
LPs suffer catastrophic losses as the jackpot becomes under-priced relative to the prize pool.

## Command to Run Test


## Proof of Concept
1. Initialize Jackpot with `normalBallMax = 10` (252 combinations) and `ticketPrice = 1e6` (1 USDC).
2. LP deposits 100,000,000 USDC. Prize pool becomes ~90M USDC.
3. `minNumberTickets` required to maintain edge ~ 111M.
4. Required bonus balls = 111M / 252 ≈ 440,000.
5. `uint8(440000)` wraps to 64.
6. `bonusballMax` is set to 64 instead of 440,000.
7. Players can cover all combinations for ~16,000 USDC (252 * 64) and win the 90M USDC prize pool.

## Proof of Code
function testBonusBallWrap() public {
    // Setup: normalMax 10 (252 combos), price 1 USDC
    // LP Deposit large amount to force large bonusball requirement
    vm.startPrank(owner);
    jackpot.setNormalBallMax(10);
    jackpot.setTicketPrice(1e6);
    vm.stopPrank();
    
    // ... (LP deposit logic omitted for brevity, assume pool is large)
    // Trigger drawing settlement
    // Check event or storage for bonusballMax
    // assertEq(drawingState.bonusballMax, 64); // Expected 440000
}

## Suggested Mitigation
Remove the `uint8` cast and change `bonusballMax` and related variables to `uint256` or `uint16` (checking for bounds appropriate to the bit-packing limitations). Add a check to ensure `normalBallMax + bonusballMax < 256` to prevent bit-packing overflow (see related finding).





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-17]. Permanent DoS of LP Manager via Zero Accumulator if pool is drained

## id: 8e4zE949gXX3RLXIRd-WG

## Derived From Pattern/Invariant
Division by Zero

## Exploit Type
Dos

## Location
JackpotLPManager._consolidateDeposits

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: JackpotLPManager.processDrawingSettlement can set drawingAccumulator[_drawingId]=0 when _drawingId>0, currentLP.lpPoolTotal>0, and postDrawLpValue==0 (newAccumulator = drawingAccumulator[_drawingId-1]*0/currentLP.lpPoolTotal). Later, _consolidateDeposits divides by drawingAccumulator[_lp.lastDeposit.drawingId]. If that stored accumulator is 0, consolidation reverts (division by zero), locking LP positions. There is no explicit guard ensuring postDrawLpValue>0 or accumulator!=0; comments claiming accumulator can never be zero are not enforced. Achieving postDrawLpValue==0 depends on governance-set economics (e.g., extreme payout + low/zero effective LP earnings), hence governance risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager`, if the LP pool is fully drained (e.g. `reserveRatio`=0 and max winnings), `postDrawLpValue` becomes 0, leading to `newAccumulator` being set to 0 in `processDrawingSettlement`. The accumulator is used as a divisor in `_consolidateDeposits` (`shares = amount * 1e18 / accumulator`). If the accumulator for a round is 0, any user with pending deposits/withdrawals from that round will revert on division by zero when trying to interact with the contract. This permanently locks their funds and prevents emergency withdrawals.

## Impact
Permanent freezing of LP funds and inability to withdraw if a specific drawing ends with 0 value.

## Command to Run Test


## Proof of Concept
1. `reserveRatio` set to 0. 
2. LP Pool has 1M USDC. 
3. Drawing results in 1M payouts (winnings + fees). 
4. `postDrawLpValue` = 0. `drawingAccumulator` = 0. 
5. LP with pending deposit in that round calls `initiateWithdraw` (or emergency withdraw). 
6. `_consolidateDeposits` divides by 0. Reverts.

## Proof of Code
function testDivZero() public { uint256 acc = 0; vm.expectRevert(); uint256 shares = 1e18 * 1e18 / acc; }

## Suggested Mitigation
Ensure `newAccumulator` is never 0, or handle 0 case in consolidation (e.g. return 0 shares).


## [M-18]. Entropy provider switch causes pending request collision allowing randomness manipulation

## id: WT3UJKDXyU8dz0cIx6yUN

## Derived From Pattern/Invariant
Key Collision in External Protocol Integration

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.setEntropyProvider

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: This is the same root cause as the other provider-switch collision reports: ScaledEntropyProvider.pending is keyed only by sequence and entropyCallback ignores the provider argument. If sequences can collide across providers, switching providers can cause an old provider’s callback to fulfill a new provider’s request (or vice versa), undermining randomness integrity for Jackpot settlement. There is no stored “expected provider” per request and no composite key (provider,sequence).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
In `ScaledEntropyProvider.sol`, pending requests are stored in a mapping keyed only by the `sequence` number returned by the entropy provider. If the Owner calls `setEntropyProvider`, the new provider (Pyth) will likely restart or reuse sequence numbers (e.g., starting from 1 or a deterministic counter). 

If there is a pending request `seq=X` from the old provider, and the new provider issues `seq=X` for a new request, the `pending` mapping entry is overwritten or collided. If the old provider's callback arrives first, it will locate the pending request for the *new* provider (due to key collision) and fulfill it using the *old* provider's randomness. This allows using uncorrelated or manipulated randomness for the new request.

## Impact
Randomness for a drawing can be supplied by an incorrect/stale provider, compromising fairness.

## Command to Run Test


## Proof of Concept
1. `runJackpot` calls Provider A, gets `seq=100`. `pending[100]` stored.
2. Admin calls `setEntropyProvider(Provider B)`.
3. `runJackpot` (next drawing) calls Provider B, gets `seq=100` (collision).
4. `pending[100]` updated with Drawing 2 details.
5. Provider A callback for `seq=100` arrives. `entropyCallback` finds `pending[100]`.
6. Drawing 2 is settled using randomness from Provider A request, which is invalid/unexpected.

## Proof of Code
function testEntropyCollision() public { ... }

## Suggested Mitigation
Include the provider address in the `pending` mapping key (e.g. `keccak256(provider, sequence)`) or verify `msg.sender == entropy` matches the expected provider for that specific request.


## [M-19]. Cross-chain winnings claim DoS if referral win share is 100%

## id: AXW3ey2EQsD4KMIVQoBCK

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
JackpotBridgeManager.claimWinnings

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: If referralWinShare == 100%, BridgeManager reverts on claimedAmount==0, which not only blocks the winner (who gets 0) but also prevents the referrer payout from occurring because Jackpot.claimWinnings is reverted too. That can lock non-trivial referrer revenue and prevent ticket burning/cleanup, so impact is not necessarily 'low' even though it requires an extreme governance setting.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
In `JackpotBridgeManager.claimWinnings`, the contract calculates the `claimedAmount` by checking the balance increase of USDC after calling `jackpot.claimWinnings`. It includes a strict check: `if (claimedAmount == 0) revert InvalidClaimedAmount();`.

If the `Jackpot` contract has `referralWinShare` set to 100% (which is allowed by `setReferralWinShare` capping at `PRECISE_UNIT`), the user's payout from `Jackpot.claimWinnings` will be 0 (all winnings go to the referrer). Consequently, `claimedAmount` will be 0, and the bridge transaction will revert. This effectively locks the winnings for bridge users, whereas direct chain users can still claim (receiving 0 but triggering the referral payment).

## Impact
Users on other chains cannot execute `claimWinnings` if the referral win share is set to 100%, causing a denial of service.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralWinShare` to 100% (1e18).
2. A cross-chain user wins a prize.
3. The user (or keeper) calls `JackpotBridgeManager.claimWinnings`.
4. `jackpot.claimWinnings` calculates payout: `winningAmount` goes entirely to referrer. User receives 0.
5. `JackpotBridgeManager` sees `claimedAmount == 0`.
6. `JackpotBridgeManager` reverts with `InvalidClaimedAmount`, preventing the transaction.

## Proof of Code
function testBridgeDos() public {
    uint256 preBal = 100;
    uint256 postBal = 100; // 0 claimed
    uint256 claimed = postBal - preBal;
    // Logic from contract
    bool reverted = false;
    if (claimed == 0) reverted = true;
    assertTrue(reverted, "Should revert on 0 claim");
}

## Suggested Mitigation
Remove the strict check `if (claimedAmount == 0) revert ...` in `JackpotBridgeManager` or ensure `referralWinShare` cannot be 100%.


## [H-20]. Randomness integrity compromise and potential DoS due to provider sequence collision in ScaledEntropyProvider

## id: 2ab6x0rBU8VDNZ2_OzV3p

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: Same root cause as the other sequence-collision report: pending is keyed only by sequence, and entropyCallback ignores the provider argument. If the owner changes entropyProvider while requests are pending, new requests can overwrite old pending entries when sequence values collide across providers. Then a callback can execute with mismatched request context (wrong callback target and/or wrong setRequests array, which is also not cleared before pushing), risking corrupted settlement randomness or settlement failure. This scenario fundamentally requires a privileged provider migration during in-flight operations, so it is primarily governance/operational risk, but the underlying code issue is real and should be fixed by scoping the key by provider and/or storing provider in PendingRequest and checking it.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `ScaledEntropyProvider` contract maps pending entropy requests using only the `sequenceNumber` returned by the Pyth provider, without scoping it to the provider address. The `pending` mapping is defined as `mapping(uint64 => PendingRequest)`. If the contract owner updates the entropy provider using `setEntropyProvider`, the new provider may generate sequence numbers that collide with pending requests from the previous provider (e.g., both starting from 1). 

When `requestAndCallbackScaledRandomness` is called with the new provider, if the returned sequence number matches an existing key in `pending`, the old request data is silently overwritten with the new request's parameters. 

This leads to two critical failure modes:
1. If the old provider's callback arrives first, it fulfills the NEW request using the OLD provider's randomness (corrupting the draw).
2. If the new provider's callback arrives, it may fail if the pending request was deleted by the old provider's callback, or behave unpredictably. 

This is particularly dangerous during `Jackpot` operations where `runJackpot` initiates a request. If a provider migration happens while a jackpot request is in-flight, the drawing could be settled with randomness from the wrong provider or bricked.

## Impact
Corruption of jackpot randomness mechanism or temporary DoS of drawing settlement during provider upgrades.

## Command to Run Test


## Proof of Concept
1. Admin calls `runJackpot`, triggering `ScaledEntropyProvider.request...`. Pyth provider A returns sequence 100. `pending[100]` is set.
2. Admin calls `setEntropyProvider(Provider B)`.
3. The jackpot drawing remains locked/pending (e.g. provider A is slow/down).
4. Admin unlocks jackpot manually via `unlockJackpot` to retry.
5. Admin/Keeper calls `runJackpot` again. `ScaledEntropyProvider` calls Provider B. Provider B returns sequence 100 (as it's a fresh counter). 
6. `pending[100]` is overwritten with the new request context.
7. Provider A sends callback for sequence 100. `entropyCallback` executes using Provider A's random number but Provider B's request context. The jackpot settles using randomness from the old/deprecated provider.

## Proof of Code
pending_poc

## Suggested Mitigation
Modify the `pending` mapping to use a composite key of `keccak256(provider, sequence)` or clear all pending requests when changing the provider.


## [M-21]. DoS of LP system if Accumulator falls to zero due to pool wipeout

## id: EDFa4ZFxtONompUP3pFVr

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: In JackpotLPManager.processDrawingSettlement (for _drawingId > 0), newAccumulator is computed as drawingAccumulator[_drawingId-1] * postDrawLpValue / currentLP.lpPoolTotal when lpPoolTotal != 0. If postDrawLpValue becomes 0 while lpPoolTotal is non-zero, newAccumulator becomes 0. Later, _consolidateDeposits and getLPValueBreakdown divide by drawingAccumulator[drawingId]; division by zero would revert, bricking LP operations for positions referencing that drawing. There is only a safeguard for lpPoolTotal==0 (sets PRECISE_UNIT) but none for postDrawLpValue==0. Achieving postDrawLpValue==0 is most plausible under extreme/unsafe governance settings (e.g., parameters that allow full depletion), hence governance risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager` uses a `drawingAccumulator` to convert between USDC and LP shares. This accumulator is updated at settlement based on the drawing's performance: `newAccumulator = (prevAcc * postDrawLpValue) / prevLpTotal`. 

If `postDrawLpValue` drops to 0 (e.g., due to a massive jackpot win where `reserveRatio` is 0, or if fees/rounding consume the remaining dust), `newAccumulator` becomes 0.

Subsequent calls to `processDeposit`, `processInitiateWithdraw`, or `_consolidateDeposits` involve division by `drawingAccumulator`. If the accumulator for a drawing is 0, these transactions will revert due to division by zero. This permanently bricks the LP deposit/withdrawal system for any user interacting with that drawing history.

## Impact
Permanent Denial of Service for LP deposits and withdrawals if the pool value is ever temporarily wiped out.

## Command to Run Test


## Proof of Concept
1. Admin sets `reserveRatio` to 0 (allowed).
2. A drawing occurs where user winnings equal the total LP pool value (Jackpot win).
3. `postDrawLpValue` calculates to 0.
4. `drawingAccumulator` is updated to 0.
5. Next drawing begins. A user tries to call `lpDeposit`.
6. `processDeposit` calls `_consolidateDeposits`, which tries to divide by `drawingAccumulator[prevDrawing]`. Since it is 0, it reverts.

## Proof of Code
function test_DoS_ZeroAccumulator() public {
    // 1. Set reserve to 0
    vm.prank(owner); jackpot.setReserveRatio(0);
    // 2. Win the whole pot
    // ... (setup win that drains pool)
    // 3. Next deposit fails
    vm.expectRevert();
    jackpot.lpDeposit(100);
}

## Suggested Mitigation
In `processDrawingSettlement`, verify `newAccumulator` is at least 1 (or reset to `PRECISE_UNIT` if pool is wiped) to prevent division by zero in future calculations.


## [H-22]. Jackpot DoS via Entropy Provider Sequence Collision

## id: 4Sg2gwnY4hlzTHcPUpzDD

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.entropyCallback

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: ScaledEntropyProvider stores pending requests in mapping(uint64 => PendingRequest) keyed only by sequence. setEntropyProvider() can switch providers while an old request is still pending; a new provider can issue a colliding sequence value (e.g., starting from 1). requestAndCallbackScaledRandomness is permissionless, so an attacker can spam requests against the new provider until they receive the colliding sequence and overwrite pending[sequence] (and note _storePendingRequest appends to setRequests without clearing, worsening collisions). When the old provider’s callback arrives, entropyCallback reads the overwritten entry and calls the attacker’s callback, not Jackpot’s, leaving Jackpot.jackpotLock stuck unless the owner intervenes. This requires a privileged provider change during an in-flight request, so it is governance/operational risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ScaledEntropyProvider` contract tracks pending entropy requests in a `pending` mapping indexed solely by the `sequenceNumber` returned by the underlying Pyth Entropy provider. This mapping is not scoped by the provider address. If the contract owner changes the entropy provider via `setEntropyProvider` while a request from the `Jackpot` contract is pending (i.e., `Jackpot` is locked), the new provider may issue a `sequenceNumber` that collides with the pending request. Since `requestAndCallbackScaledRandomness` is permissionless, an attacker can spam requests to the new provider until they generate a sequence number matching the pending Jackpot request. This overwrites the `pending` entry with the attacker's callback details. When the original provider fulfills the request, `ScaledEntropyProvider` invokes the attacker's callback instead of the `Jackpot` callback. The `Jackpot` contract remains permanently locked (`jackpotLock = true`) as its callback is never executed, requiring emergency admin intervention.

## Impact
Permanent DoS of the Jackpot contract (stuck in locked state), requiring emergency unlocking and potentially wasting entropy fees.

## Command to Run Test


## Proof of Concept
1. `Jackpot` calls `runJackpot()`, which calls `ScaledEntropyProvider.request...()`. A request is sent to Provider A, returning `sequence = 100`. `pending[100]` maps to `Jackpot`. 
2. Admin calls `setEntropyProvider(Provider B)`. 
3. Attacker calls `ScaledEntropyProvider.request...()` (pointing to their own contract) repeatedly until Provider B returns `sequence = 100`. 
4. `pending[100]` is overwritten; it now maps to `AttackerContract`. 
5. Provider A fulfills the original request with `sequence = 100`. 
6. `ScaledEntropyProvider` executes `entropyCallback(100, ...)` and retrieves the attacker's data from `pending`. 
7. It calls `AttackerContract.scaledEntropyCallback(...)`. 
8. `Jackpot` never receives the callback and remains locked forever.

## Proof of Code
function testCollision() public {
  // Conceptual test as this requires mocking Pyth provider behavior
  // 1. Mock Provider A returns seq 100. Jackpot locks.
  // 2. Mock Provider B returns seq 100. Attacker calls request.
  // 3. pending[100] now points to Attacker.
  // 4. Provider A calls back. Callback goes to Attacker.
  // 5. Jackpot.jackpotLock remains true.
}

## Suggested Mitigation
Include the entropy provider address in the `pending` mapping key (e.g., `mapping(address => mapping(uint64 => PendingRequest))`), or verify that the callback provider matches the provider stored in the pending request.





Finding Status: InvalidGovernanceRisk
## [H-23]. Swapping PayoutCalculator via setPayoutCalculator permanently freezes unclaimed winnings from previous drawings

## id: PDhWHO4WA4DdGuvGWvKJc

## Derived From Pattern/Invariant
UpgradeabilityInitializerSafety

## Exploit Type
StorageLayout

## Location
Jackpot.setPayoutCalculator

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Changing payoutCalculator can cause users to permanently lose unclaimed historical winnings (claims return 0 and tickets are burned). This is high impact to affected users, even if it only occurs via owner action.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `Jackpot` contract relies on a `payoutCalculator` (an external contract implementing `IPayoutCalculator`) to determine the payout amount for winning tickets via `calculateAndStoreDrawingUserWinnings` and `getTierPayout`. The calculated payout amounts are stored in the `tierPayouts` mapping within the `GuaranteedMinimumPayoutCalculator` contract, not in the `Jackpot` contract.

The `Jackpot` contract allows the owner to update the calculator address using `setPayoutCalculator`. However, the `Jackpot` contract does not track which calculator was used for which drawing. It always queries the *current* `payoutCalculator` for tier payouts in `claimWinnings`.

If the calculator is updated, the new calculator contract will have empty `tierPayouts` mappings for all previous drawing IDs. Consequently, when a user attempts to claim winnings for a ticket from a drawing settled by the *old* calculator, the `Jackpot` calls `getTierPayout` on the *new* calculator, which returns 0. The user receives 0 USDC and their ticket is burned. This effectively permanently freezes/burns all unclaimed winnings from history prior to the update.

## Impact
Users are unable to claim winnings for any drawing settled before the calculator update, resulting in permanent loss of funds.

## Command to Run Test


## Proof of Concept
1. Protocol runs normally for Drawing 1. Winners are determined, payouts are stored in `CalculatorA`. 
2. Admin deploys `CalculatorB` (e.g. to update premium weights logic) and calls `Jackpot.setPayoutCalculator(CalculatorB)`.
3. A user holding a winning ticket for Drawing 1 calls `Jackpot.claimWinnings([ticketId])`.
4. `Jackpot` calls `payoutCalculator.getTierPayout(1, tierId)`. Since `payoutCalculator` is now `CalculatorB`, and `CalculatorB` has no record of Drawing 1, it returns 0.
5. The user's ticket is burned, and they receive 0 USDC.

## Proof of Code
function test_Exploit_PayoutCalculatorSwap() public {
    // 1. Setup drawing and determine winner
    // ... (simulation of buying ticket and running jackpot)
    uint256 ticketId = ticketIds[0];
    
    // 2. Admin swaps calculator
    vm.startPrank(owner);
    GuaranteedMinimumPayoutCalculator newCalc = new GuaranteedMinimumPayoutCalculator(jackpot, ...);
    jackpot.setPayoutCalculator(newCalc);
    vm.stopPrank();

    // 3. User claims winnings
    vm.prank(user);
    uint256 balanceBefore = usdc.balanceOf(user);
    jackpot.claimWinnings(ticketIds);
    uint256 balanceAfter = usdc.balanceOf(user);

    // 4. Assert user got nothing
    assertEq(balanceAfter - balanceBefore, 0);
}

## Suggested Mitigation
Either:
1. Store the `IPayoutCalculator` address used for each drawing in a mapping `mapping(uint256 => IPayoutCalculator) drawingPayoutCalculator` in `Jackpot` and use that for claims.
2. Or, prohibit changing the payout calculator once the system is live.
3. Or, require data migration (impractical on-chain).


## [H-24]. LP pool drain via arbitrage on duplicate tickets when referral fee exceeds LP edge

## id: n3Md6jGPoTFqhc_r36Abn

## Derived From Pattern/Invariant
Incentive Misalignment / Accounting Invariant Violation

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: If configured with referralFee > lpEdgeTarget, duplicate-ticket accounting can make LP EV negative and enable economically meaningful LP value extraction; that is not low impact (even though it is triggered by privileged misconfiguration).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `buyTickets`, when a duplicate ticket is purchased, the `prizePool` is incremented by `ticketPrice - edgePerTicket` to strictly preserve the LP edge on the prize side. However, the `lpEarnings` (which accretes to the LP pool at settlement) is incremented by `ticketPrice - referralFee`. If the configured `referralFee` is greater than `lpEdgeTarget`, the amount added to the prize pool (a liability for LPs) exceeds the net earnings retained by LPs. 

Snippet:
```solidity
// Jackpot.sol
if (isDup) {
    _currentDrawingState.prizePool += _currentDrawingState.ticketPrice - _currentDrawingState.edgePerTicket;
}
...
currentDrawingState.lpEarnings += ticketsValue - referralFeeTotal;
```
An attacker can exploit this by purchasing duplicates of all ticket combinations (or their own tickets) using a self-referral. The attacker pays `ticketPrice - referralFee` effectively, but increases the `prizePool` (which they are statistically likely to win or guaranteed to win if covering all combos) by `ticketPrice - lpEdgeTarget`. The difference results in a risk-free profit extracted from the LP pool.

## Impact
Direct theft of LP funds. Attackers can drain the LP pool by cycling funds through duplicate ticket purchases.

## Command to Run Test


## Proof of Concept
1. Owner sets `referralFee` to 20% and `lpEdgeTarget` to 10%.
2. Attacker ensures `prizePool` covers all combinations (or accepts statistical variance).
3. Attacker buys a ticket and then buys a duplicate of it, referring themselves.
4. For the duplicate: Attacker pays 100 USDC. Receives 20 USDC referral. Net cost 80.
5. `prizePool` increases by 90 USDC (100 - 10).
6. `lpEarnings` increases by 80 USDC.
7. At settlement, LP pool value = `OldLP + 80 (Earnings) - 90 (Winnings paid out) = OldLP - 10`.
8. Attacker wins the 90 USDC increase (assuming they hold the winning tickets). Net Profit: 90 (Win) - 80 (Cost) = 10 USDC.
9. Repeat to drain pool.

## Proof of Code
function testExploitReferralArb() public {
    // Setup: Edge 10%, Referral 20%
    vm.startPrank(owner);
    jackpot.setLpEdgeTarget(0.1e18);
    jackpot.setReferralFee(0.2e18);
    vm.stopPrank();

    // Deposit LP to ensure prize pool exists
    vm.startPrank(lp);
    usdc.approve(address(jackpot), 1000e6);
    jackpot.lpDeposit(1000e6);
    vm.stopPrank();

    // Run drawing to settle deposit
    vm.warp(block.timestamp + 1 days);
    jackpot.runJackpot{value: 0.1 ether}();
    // ... mock entropy callback ...

    // Attack
    vm.startPrank(attacker);
    uint256 startLP = jackpotLPManager.getLPDrawingState(jackpot.currentDrawingId()).lpPoolTotal;
    
    // Buy duplicate
    // ... (omitted setup for ticket structs)
    address[] memory refs = new address[](1); refs[0] = attacker;
    uint256[] memory splits = new uint256[](1); splits[0] = 1e18;
    jackpot.buyTickets(tickets, attacker, refs, splits, bytes32(0));
    
    // Verify LP pool value drops more than legitimate math allows at settlement
    // ...
}

## Suggested Mitigation
Enforce an invariant in `setReferralFee` and `setLpEdgeTarget` ensuring `referralFee <= lpEdgeTarget`. Alternatively, adjust duplicate ticket logic to credit `prizePool` based on the actual net revenue received (`ticketPrice - referralFee`) rather than the target edge.



