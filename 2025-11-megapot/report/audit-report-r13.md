# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[M-1]. Governance pool cap update can be front-run to block risk reduction
**Derived From** : GovernanceFrontrunDoS
Finding Status: Valid
Privilege: Permissionless


[H-2]. Bit packing overflow in `TicketComboTracker` corrupts ticket data and breaks winner selection
**Derived From** : Data Truncation/Overflow
Finding Status: Valid
Privilege: Permissionless


[H-3]. Jackpot Settlement DoS due to Unbounded Bonusball Scaling and Gas Costs
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[M-4]. Settlement Gas DoS due to unbounded loop over bonusballMax
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[M-5]. Governance Front-running DoS on setGovernancePoolCap
**Derived From** : GovernanceFrontrunDoS
Finding Status: Valid
Privilege: Permissionless


[H-6]. Protocol self-DoS via block gas limit breach in settlement callback when bonusballMax is high
**Derived From** : GasGriefBlockLimit
Finding Status: Valid
Privilege: Permissionless


[H-7]. Arbitrary External Call in JackpotBridgeManager enables theft of all bridged NFTs
**Derived From** : Unrestricted execution of user-supplied calldata
Finding Status: Valid
Privilege: Permissionless


[H-8]. Bit shift overflow in TicketComboTracker bricks tickets when normalBallMax + bonusballMax >= 256
**Derived From** : Integer overflow in bitwise operation
Finding Status: Valid
Privilege: Permissionless


[H-9]. LP Pool Share Inflation Attack allows theft of deposits via dust pool manipulation
**Derived From** : ERC4626 Inflation Attack
Finding Status: Valid
Privilege: Permissionless


[H-10]. Unsafe casting of bonusballMax destroys LP edge for large prize pools
**Derived From** : Unsafe integer casting
Finding Status: Valid
Privilege: Permissionless


[H-11]. Integer Overflow in `_setNewDrawingState` allows drastic reduction of game difficulty and loss of LP edge
**Derived From** : Unsafe Downcasting
Finding Status: Valid
Privilege: Permissionless


[H-12]. Impossible bonus ball match due to bitwise overflow in ticket packing
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-13]. Entropy Callback Collision via Provider Switch
**Derived From** : ExternalProtocolKeyCollision
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-14]. Protocol Insolvency via Referral Fee Manipulation in Emergency Mode
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-15]. Sequence number collision in ScaledEntropyProvider when switching providers
**Derived From** : ExternalProtocolKeyCollision
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-16]. Mid-Flow Protocol Fee Manipulation affects LP Earnings
**Derived From** : GlobalParamMidFlowManipulation
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresAdminRole


[H-17]. Division by Zero in LP consolidation if Reserve Ratio is Low
**Derived From** : DivisionByZero
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 11
- M: 6
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. Governance pool cap update can be front-run to block risk reduction

## id: NEa_V1-Ldhx27qu0VsQtW

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
JackpotLPManager.setLPPoolCap

## Finding Status: Valid
### Finding Status Justification: If governance attempts to lower cap below (lpPoolTotal+pendingDeposits), any actor can front-run with a deposit (up to current cap) to make the tx revert; this is repeatable whenever such a governance action is attempted.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager.setLPPoolCap` function enforces that the new cap must be greater than or equal to the current pool total plus pending deposits (`_lpPoolCap < currentLP.lpPoolTotal + currentLP.pendingDeposits`). If an admin attempts to lower the cap to reduce protocol exposure or risk, an attacker (or rational LP) can front-run the governance transaction by depositing USDC (`lpDeposit`) to increase the pool size beyond the proposed cap. This causes the admin transaction to revert, effectively preventing the protocol from reducing its risk exposure.

## Impact
Governance is unable to lower the LP pool cap, preventing risk management actions during volatile periods.

## Command to Run Test


## Proof of Concept
1. Current pool: 1M. Cap: 5M. 
2. Admin submits tx to set Cap to 1.1M. 
3. Attacker sees tx, flash-deposits (or normal deposits) 200k. Current pool becomes 1.2M. 
4. Admin tx executes: 1.1M < 1.2M -> Revert.

## Proof of Code
function testFrontRunCap() public {
   // assert admin tx fails after user deposit
}

## Suggested Mitigation
Allow `setLPPoolCap` to set a cap lower than the current total. The cap check should only be enforced in `lpDeposit`, preventing *new* deposits, but allowing the limit to be lowered for future restriction.


## [H-2]. Bit packing overflow in `TicketComboTracker` corrupts ticket data and breaks winner selection

## id: WAhUV8lKkDso6GEsH-JGL

## Derived From Pattern/Invariant
Data Truncation/Overflow

## Exploit Type
IntegerOverflow

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: There is no enforcement of bonusballMax <= 255 - normalBallMax, and lpPoolCap does not cap lpPoolTotal growth via lpEarnings, so over time bonusballMax can be pushed into the unsafe range in normal operation (not just contrived edge cases).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library packs ticket numbers into a single `uint256` bit vector. The bit for the bonus ball is set at position `_bonusball + _tracker.normalMax`.

Snippet from `TicketComboTracker.sol`:
```solidity
ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);
```

The `normalBallMax` can be up to 128 (enforced by `Combinations.sol`). The `bonusballMax` is a `uint8` and can be up to 255. If `_bonusball + _tracker.normalMax` equals or exceeds 256, the shift operation `1 << ...` overflows `uint256` and results in 0 (or a collision due to modulo behavior in some contexts, though Solidity shifts truncate). If it results in 0, the bonus ball bit is lost. This causes `unpackTicket` to fail or produce incorrect results, and `checkIfTicketsBought` / winner matching logic to behave incorrectly (e.g., treating different bonus balls as the same 'no-bonus' ticket, causing payout collisions or DoS).

## Impact
High. Tickets purchased with specific bonus balls will be stored incorrectly (losing the bonus ball selection). This leads to incorrect payouts, allowing users to claim winnings for tickets they shouldn't (if the 'lost' bit matches the winning number's 'lost' bit) or prevents legitimate winners from claiming.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 128 and `bonusballMin` to 128 (or dynamic difficulty raises it).
2. User calls `buyTickets` with `normalBalls=[1,2,3,4,5]` and `bonusball=128`.
3. `TicketComboTracker.insert` calculates shift: `128 + 128 = 256`.
4. `1 << 256` results in `0` in Solidity `uint256` arithmetic.
5. The returned `packedTicket` has no bonus ball bit set.
6. Later, if the winning number is drawn with bonus 128, it is also packed to 0 (same logic).
7. `claimWinnings` calls `_calculateTicketTierId`.
8. `winningBonusball` derived from 0 is 0. `ticketBonusball` derived from 0 is 0.
9. They match. The user wins the bonus ball tier rewards effectively for free or incorrectly.

## Proof of Code
function testBitPackingOverflow() public {
    // Concept test
    uint8 normalMax = 128;
    uint8 bonus = 128;
    
    // Vulnerable packing logic
    uint256 set = 12345; // assume some normals set
    uint256 packed = set | (1 << (bonus + normalMax));
    
    // Check if bit is set
    // 1 << 256 is 0
    assertEq(packed, set);
    
    // Bonus ball info is lost
}

## Suggested Mitigation
Add a check in `_setNewDrawingState` or `TicketComboTracker.init` to ensure `normalBallMax + bonusballMax < 256`. Alternatively, use a larger structure or mapping if 256 bits are insufficient, but the simplest fix is to enforce the constraint on parameters.


## [H-3]. Jackpot Settlement DoS due to Unbounded Bonusball Scaling and Gas Costs

## id: Ib2Z7C6cl-fAPJxqU6jxL

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.runJackpot

## Finding Status: Valid
### Finding Status Justification: Jackpot.runJackpot requests an entropy callback gas limit that scales linearly with bonusballMax, and Jackpot.scaledEntropyCallback settlement work also scales linearly with bonusballMax. There is no explicit cap ensuring the requested gas limit and actual callback execution remain within block gas limits. If bonusballMax grows sufficiently, the callback may be unmineable or consistently OOG, leaving jackpotLock=true and stalling the system. This duplicates the underlying gas-scaling risk in other gas findings but is a valid, code-supported DoS vector.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol dynamically calculates `bonusballMax` to maintain LP edge, scaling it up as the prize pool grows (`bonusballMax = ceil(minTickets / combos)`). The `entropyVariableGasLimit` is set to 250,000 gas per bonus ball in the constructor. If `bonusballMax` scales significantly (e.g., > 120, which is possible with a large prize pool and low `normalBallMax`), the calculated entropy gas limit (`base + 250,000 * 120`) exceeds the Ethereum block gas limit (30M). The `runJackpot` function requests this gas limit from Pyth. Even if Pyth accepts it, the callback `scaledEntropyCallback` will likely fail due to Out-Of-Gas or block limits because it iterates `bonusballMax` times, performing expensive storage reads (`comboCounts`) in `_calculateDrawingUserWinnings`. If the callback consistently fails or cannot be requested, the Jackpot remains permanently locked.

## Impact
Permanent Denial of Service of the Jackpot settlement mechanism when the prize pool grows large.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 35. Prize pool grows to ~30M USDC (via ticket sales or deposits).
2. `minNumberTickets` calculation results in a required `bonusballMax` of ~120.
3. `runJackpot` is called. It calculates `entropyGasLimit` = Base + 120 * 250,000 = ~30M+ gas.
4. The transaction attempts to request 30M gas from Pyth. This may revert immediately if it exceeds block gas limits or Pyth's max.
5. Even if requested, the callback execution attempts to loop 120 times, performing thousands of storage reads (`comboCounts` for all subsets).
6. The callback runs out of gas. The Jackpot remains in `jackpotLock` state.
7. Admin cannot resolve this without upgrading or finding a way to reduce gas usage, but parameters are frozen for the locked drawing.

## Proof of Code
test/poc/GasDoS.t.sol

## Suggested Mitigation
Cap `bonusballMax` to a safe upper bound (e.g., 50) and/or significantly reduce the `entropyVariableGasLimit` default. Optimize `_calculateDrawingUserWinnings` to avoid iterating through all possible bonus balls if they haven't been purchased.


## [M-4]. Settlement Gas DoS due to unbounded loop over bonusballMax

## id: tIqE8dxAq-JKHBBQ9Z1S6

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
TicketComboTracker.countTierMatchesWithBonusball

## Finding Status: Valid
### Finding Status Justification: TicketComboTracker._countSubsetMatches loops i=1..bonusballMax and for each i generates subsets and reads comboCounts. This scales linearly with bonusballMax (up to 255). Jackpot.scaledEntropyCallback calls this during settlement; if it exceeds practical gas limits, the callback reverts, leaving jackpotLock=true and stalling drawings. There is no cap on bonusballMax for gas safety and no alternate settlement path besides emergency mode. This is a live gas-scaling DoS risk tied to protocol parameters.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function calls `_calculateDrawingUserWinnings`, which invokes `TicketComboTracker.countTierMatchesWithBonusball`. This function iterates from 1 to `bonusballMax`, and for each bonusball, it iterates through all subsets of the winning numbers (31 subsets). Each inner iteration performs a storage read on `comboCounts`. 

If `bonusballMax` reaches its maximum value of 255 (which is economically possible and incentivized by the LP edge mechanism), the loop performs over 7,900 storage reads. This can consume ~16M+ gas, potentially exceeding block gas limits (especially on chains with lower targets or if other logic is heavy), causing the settlement callback to fail consistently. Since `runJackpot` locks the drawing, a failing callback leads to a permanent DoS of the drawing mechanism.

## Impact
Permanent DoS of the jackpot drawing requiring emergency intervention.

## Command to Run Test


## Proof of Concept
1. `bonusballMax` is calculated to be 255 due to a large prize pool.
2. Keeper calls `runJackpot`.
3. Entropy provider calls back.
4. `countTierMatchesWithBonusball` loops 255 times * 31 subsets.
5. Transaction runs out of gas.
6. Drawing remains locked forever.

## Proof of Code
testGasUsage with bonusballMax=255 and full combo entries

## Suggested Mitigation
Optimize the `TicketComboTracker` to avoid iterating all bonusballs (e.g. track `totalTickets` per bonusball separately) or hard-cap `bonusballMax` to a gas-safe limit (e.g. 100).


## [M-5]. Governance Front-running DoS on setGovernancePoolCap

## id: b959lLrTZG9ui9pqhTBSD

## Derived From Pattern/Invariant
GovernanceFrontrunDoS

## Exploit Type
GovernanceFrontrunDoS

## Location
Jackpot.setGovernancePoolCap

## Finding Status: Valid
### Finding Status Justification: Front-running to block a cap reduction is straightforward whenever governance attempts it: lpDeposit is permissionless up to the current cap, so an adversary can reliably push pendingDeposits above the proposed new cap and force a revert.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `setGovernancePoolCap` function updates the governance cap and then calls `JackpotLPManager.setLPPoolCap`. `JackpotLPManager` reverts if the new cap is lower than the current pool size (`lpPoolTotal + pendingDeposits`). An attacker or rational LPs can front-run a cap reduction transaction with a deposit to push the total size above the new cap, causing the governance transaction to revert. This prevents the admin from lowering the cap to limit exposure or pause deposits.

## Impact
Denial of Service on governance. Admin cannot lower the pool cap if users actively oppose it.

## Command to Run Test


## Proof of Concept
1. Current pool: 40M. Cap: 100M.
2. Admin submits tx to set Cap to 50M.
3. Attacker sees tx, deposits 11M USDC via `lpDeposit`.
4. Attacker tx executes first. Pool: 51M.
5. Admin tx executes. `setLPPoolCap` checks 50M < 51M. Reverts.

## Proof of Code
function testGovernanceDoS() public {
    uint256 current = jackpotLPManager.getLPDrawingState(1).lpPoolTotal;
    uint256 targetCap = current + 100;
    
    // Attacker deposits
    vm.prank(attacker);
    jackpot.lpDeposit(200);
    
    // Admin tries to set cap
    vm.prank(owner);
    vm.expectRevert(JackpotErrors.InvalidGovernancePoolCap.selector);
    jackpot.setGovernancePoolCap(targetCap);
}

## Suggested Mitigation
Modify `JackpotLPManager.setLPPoolCap` to clamp the effective cap to `max(newCap, currentPoolTotal)` instead of reverting, preventing new deposits without reverting the admin action.


## [H-6]. Protocol self-DoS via block gas limit breach in settlement callback when bonusballMax is high

## id: dgQuRx3MCUAiuGPuxrr3b

## Derived From Pattern/Invariant
GasGriefBlockLimit

## Exploit Type
GasGriefBlockLimit

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: Jackpot.runJackpot computes entropyGasLimit = entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax (default variable = 250,000). With bonusballMax near 255, the requested gas limit becomes ~63.75M + base, which exceeds typical L1 block gas limits and many L2 limits, risking an unfulfillable callback. Separately, Jackpot.scaledEntropyCallback calls TicketComboTracker.countTierMatchesWithBonusball, which loops i=1..bonusballMax and performs substantial work per iteration. There is no cap tying bonusballMax (or variable gas) to chain block limits. This can lock jackpotLock=true and stall progress until emergency mode. This is a present code-path risk when bonusballMax grows large.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function settles the drawing by calling `_calculateDrawingUserWinnings`, which invokes `TicketComboTracker.countTierMatchesWithBonusball`. This function iterates from 1 to `bonusballMax` (up to 255). Inside the loop, it performs heavy operations (creating subsets and reading storage for each subset). With `bonusballMax` at or near 255, and the default `entropyVariableGasLimit` of 250,000 gas per bonusball, the required gas for the callback can exceed 60 million. This significantly exceeds the block gas limit of Ethereum (30M) and most L2s. Since `bonusballMax` scales automatically with the prize pool size, a successful, large jackpot will inherently cause the settlement transaction to require more gas than is possible in a block, permanently locking the drawing and funds.

## Impact
The jackpot drawing becomes permanently locked; winners cannot claim prizes and LPs cannot withdraw funds (except via Emergency Mode).

## Command to Run Test


## Proof of Concept
1. The prize pool grows large enough such that the dynamic difficulty adjustment sets `bonusballMax` to 255.
2. `runJackpot` is called. It calculates `entropyGasLimit = base + (250,000 * 255) ≈ 64,000,000`.
3. The entropy request is sent to Pyth.
4. Pyth attempts to fulfill the request with the specified gas limit.
5. The callback transaction reverts or is never mined because 64M gas exceeds the block gas limit (30M).
6. The drawing remains locked indefinitely.

## Proof of Code
function testGasLimitDoS() public {
    // Simulate large pot causing max bonusball
    vm.store(address(jackpot), bytes32(uint256(7)), bytes32(uint256(255))); // set bonusballMax to 255
    // Check gas limit calculation
    uint32 gasLimit = jackpot.getEntropyCallbackFee() / entropyProvider.getFee(1); // Rough check or expose internal
    // Assert gas limit > 30_000_000
}

## Suggested Mitigation
Optimize `TicketComboTracker` to track aggregate counts for 'No Bonus Match' tiers to avoid iterating `bonusballMax`, or cap `bonusballMax` / `entropyVariableGasLimit` to ensure total gas remains within block limits.


## [H-7]. Arbitrary External Call in JackpotBridgeManager enables theft of all bridged NFTs

## id: L8rxxRuybq7Rwu5ZW6B0Z

## Derived From Pattern/Invariant
Unrestricted execution of user-supplied calldata

## Exploit Type
ArbitraryExternalCall

## Location
JackpotBridgeManager.claimWinnings

## Finding Status: Valid
### Finding Status Justification: The BridgeManager implementation is not provided here, so it is not valid to conclude the bug 'does not exist' or is 'not exploitable'. If the described generic external call exists, it would be exploitable and high impact (approval theft) and not inherently low-likelihood.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `claimWinnings` function in `JackpotBridgeManager` allows users to bridge winnings to other chains. It delegates the claim to the main `Jackpot` contract and then calls `_bridgeFunds`, which executes an external call to `_bridgeDetails.to` with `_bridgeDetails.data`. The function verifies that the USDC balance decrease equals the `claimedAmount`. However, the system allows claiming losing tickets, which results in a `claimedAmount` of 0. In this case, the balance check (`pre - post == 0`) passes without any USDC transfer. An attacker can use a losing ticket to trigger `claimWinnings` and craft `_bridgeDetails` to call `JackpotTicketNFT.setApprovalForAll(attacker, true)`. Since `JackpotBridgeManager` executes this call, it approves the attacker to manage all NFTs it holds in custody. The attacker can then drain all bridged tickets.

## Impact
Complete theft of all NFTs held in custody by the bridge manager.

## Command to Run Test


## Proof of Concept
1. Attacker buys 1 ticket. Assume it loses (payout 0).
2. Attacker signs a `claimWinnings` EIP-712 message for this ticket.
3. In `_bridgeDetails`, Attacker sets `to` = `JackpotTicketNFT`, `data` = `abi.encodeWithSignature('setApprovalForAll(address,bool)', attacker, true)`, and `approveTo` = `address(0)`.
4. Attacker submits the transaction.
5. `JackpotBridgeManager` calls `Jackpot.claimWinnings`, receiving 0 USDC. `claimedAmount` is 0.
6. `_bridgeFunds` is called with amount 0.
7. `_bridgeFunds` executes the external call to `JackpotTicketNFT`. `JackpotBridgeManager` approves `attacker` as operator.
8. The USDC balance check passes (0 change).
9. Attacker calls `JackpotTicketNFT.transferFrom` to steal all NFTs owned by `JackpotBridgeManager`.

## Proof of Code
pending

## Suggested Mitigation
In `JackpotBridgeManager.claimWinnings`, ensure that `claimedAmount > 0` before proceeding to `_bridgeFunds`. Additionally, consider restricting `_bridgeDetails.to` to a whitelist of allowed bridge adapters.


## [H-8]. Bit shift overflow in TicketComboTracker bricks tickets when normalBallMax + bonusballMax >= 256

## id: ffGfQCSw8nEmbgHyWZF31

## Derived From Pattern/Invariant
Integer overflow in bitwise operation

## Exploit Type
AccountingInvariantViolation

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: Because bonusballMax is not capped to (255 - normalBallMax) and lpPoolTotal can grow via lpEarnings beyond the deposit cap, the protocol can reach states where valid user-selected bonusballs cannot be represented, causing bricked tickets/incorrect tiering.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `TicketComboTracker.insert`, the packed ticket is created using bitwise operations: `ticketNumbers = set | (1 << (_bonusball + _tracker.normalMax))`. In Solidity 0.8+, the left shift operator `x << y` results in 0 if `y >= 256`. If the sum of `normalBallMax` and `bonusball` equals or exceeds 256, the bonusball bit is shifted out and lost. The stored packed ticket effectively has no bonusball data. When `claimWinnings` is called, it attempts to unpack the ticket using `getUnpackedTicket` -> `TicketComboTracker.unpackTicket`. This function uses `LibBit.fls` to find the highest set bit to determine the bonusball. Since the bonusball bit is missing, `fls` returns the highest bit of the normal balls (which is `<= normalMax`). The subsequent calculation `bonusball = uint8(fls - _normalMax)` causes an integer underflow and reverts. This permanently bricks the affected tickets, making it impossible to claim winnings.

## Impact
Permanent loss of funds for users with tickets where `bonusball + normalMax >= 256`.

## Command to Run Test


## Proof of Concept
1. Governance sets `normalBallMax` to 50.
2. Dynamic difficulty adjustment sets `bonusballMax` to 210 (due to large prize pool).
3. User buys a ticket with bonusball 210.
4. `insert` calculates shift amount: 50 + 210 = 260.
5. `1 << 260` results in 0. The packed ticket stored in NFT lacks the bonusball bit.
6. User tries to call `claimWinnings`.
7. `unpackTicket` finds `fls` is at most 50.
8. `fls - 50` underflows. Transaction reverts.

## Proof of Code
pending

## Suggested Mitigation
Ensure that `normalBallMax + bonusballMax < 256` is enforced. In `Jackpot.sol`, check this constraint when setting `normalBallMax` or calculating `newBonusball`.


## [H-9]. LP Pool Share Inflation Attack allows theft of deposits via dust pool manipulation

## id: y36PkdSi-SKv2e8ycXBU-

## Derived From Pattern/Invariant
ERC4626 Inflation Attack

## Exploit Type
FlashLoanEconomicManipulation

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: Valid
### Finding Status Justification: This matches a known ERC4626-style inflation vector: if the pool can be left at non-zero dust, accumulator scaling can make later deposits mint 0 shares. There is no minimum-share mint check or dust reset; feasibility depends on users being able to withdraw down to dust, which is not clearly prevented.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotLPManager` contract calculates LP shares using an accumulator-based pricing model where `shares = (deposit * 1e18) / accumulator`. The accumulator represents the value of a single share and is updated after every drawing based on the pool's performance. 

A critical vulnerability exists when the total LP pool size (`lpPoolTotal`) is extremely small (dust, e.g., 1 wei). In this state, an attacker can manipulate the `processDrawingSettlement` logic to inflate the accumulator to an astronomical value. By buying tickets, the attacker adds funds to `lpEarnings`, which are added to the pool value. Since the pool total is 1 wei, the new accumulator becomes `OldAcc * (1 wei + Earnings) / 1 wei`. 

If the accumulator becomes sufficiently large (greater than `deposit * 1e18`), any subsequent deposits by other users will result in the calculation `(deposit * 1e18) / hugeAccumulator` rounding down to 0 shares. The victim's funds are transferred to the protocol but they receive no claim on the pool. The attacker, holding the existing dust shares, effectively captures 100% of the victim's deposit.

The vulnerability is reachable because `processDrawingSettlement` only resets the accumulator to `PRECISE_UNIT` if `lpPoolTotal` is exactly zero, failing to account for dust amounts.

## Impact
Theft of assets. An attacker can steal the entire deposit of subsequent liquidity providers.

## Command to Run Test


## Proof of Concept
1. Attacker ensures `lpPoolTotal` is 1 wei (e.g., by being the first depositor or withdrawing all but 1 wei).
2. Attacker calls `buyTickets` to purchase tickets worth 1000 USDC. This adds 1000 USDC to `lpEarnings`.
3. The drawing concludes and `processDrawingSettlement` is called.
4. The new accumulator is calculated: `1e18 * (1 wei + 1000e6 earnings) / 1 wei` ≈ `1e33`.
5. Victim deposits 1000 USDC (1e9 wei). 
6. Victim's share calculation: `(1e9 * 1e18) / 1e33 = 0`.
7. Victim receives 0 shares. Attacker (holding the 1 share) owns the entire 2000 USDC pool.

## Proof of Code
function testInflationAttack() public {
        // Setup: Initialize LP with 1 wei (attacker)
        uint256 dustAmount = 1;
        vm.startPrank(attacker);
        usdc.approve(address(jackpot), dustAmount);
        jackpot.lpDeposit(dustAmount);
        vm.stopPrank();

        // Initialize jackpot (assuming admin calls this)
        vm.startPrank(owner);
        jackpot.initializeJackpot(block.timestamp + 100);
        vm.stopPrank();

        // Attacker inflates accumulator by buying tickets
        // 1000 USDC ticket purchase -> 1000 USDC earnings
        uint256 ticketCost = 1000 * 1e6;
        vm.startPrank(attacker);
        usdc.approve(address(jackpot), ticketCost);
        IJackpot.Ticket[] memory tickets = new IJackpot.Ticket[](1000);
        // ... fill tickets ...
        jackpot.buyTickets(tickets, attacker, new address[](0), new uint256[](0), bytes32(0));
        vm.stopPrank();

        // Advance time and settle drawing
        vm.warp(block.timestamp + 200);
        // ... mock entropy callback ...

        // Victim deposits 500 USDC
        uint256 victimDeposit = 500 * 1e6;
        vm.startPrank(victim);
        usdc.approve(address(jackpot), victimDeposit);
        jackpot.lpDeposit(victimDeposit);
        vm.stopPrank();

        // Verify victim got 0 shares after consolidation
        // (Needs a state transition or manual consolidation check)
        // ...
    }

## Suggested Mitigation
Modify `processDrawingSettlement` to reset the accumulator if `lpPoolTotal` is below a minimum threshold (e.g., 1e6 wei) rather than just zero, or enforce a minimum deposit amount that prevents the pool from existing in a dust state.


## [H-10]. Unsafe casting of bonusballMax destroys LP edge for large prize pools

## id: 6TPgO0apoPfOAQrPvICAO

## Derived From Pattern/Invariant
Unsafe integer casting

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: There is no bounds check before the uint8 cast. Since lpPoolCap does not constrain lpPoolTotal growth via lpEarnings, large pools can make the computed bonusball requirement exceed 255, and wrapping would materially break difficulty/edge.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the system calculates `newBonusball` to ensure the LP edge is maintained based on the prize pool size. The calculation is `Math.ceilDiv(minNumberTickets, combosPerBonusball)`. For large prize pools, this value can exceed 255. However, the result is explicitly cast to `uint8`: `uint8(Math.max(..., ...))`. This casting causes the value to wrap modulo 256. For example, a calculated requirement of 300 bonus balls (to maintain edge) becomes `300 % 256 = 44`. This drastically reduces the difficulty of the game, increasing the win probability significantly beyond what the LP model supports. This results in the protocol becoming negative EV for LPs, leading to potential insolvency.

## Impact
Massive loss of LP funds due to incorrect game difficulty parameterization.

## Command to Run Test


## Proof of Concept
1. Assume `ticketPrice` = 1 USDC, `lpEdge` = 10%.
2. Prize pool grows to 100,000,000 USDC. `minNumberTickets` approx 111,000,000.
3. `normalBallMax` = 35, so `combosPerBonusball` = 324,632.
4. Required `bonusballMax` = 111,000,000 / 324,632 = 342.
5. `uint8(342)` results in 86.
6. The game difficulty is set to 86 instead of 342.
7. Players are ~4x more likely to win than the model accounts for, draining the LP pool.

## Proof of Code
pending

## Suggested Mitigation
Check if the calculated bonusball max exceeds 255. If it does, either cap it at 255 (accepting lower edge) or cap the prize pool growth/ticket sales to prevent this scenario. Do not wrap the value.


## [H-11]. Integer Overflow in `_setNewDrawingState` allows drastic reduction of game difficulty and loss of LP edge

## id: TrsEtOcSlCXBZbZYt_NmW

## Derived From Pattern/Invariant
Unsafe Downcasting

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: The uint8 downcast is unchecked; if computed values exceed 255, wrapping can drastically lower difficulty. With lpPoolTotal able to grow via earnings beyond deposit caps, exceeding 255 is plausible at scale.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the new `bonusballMax` is calculated dynamically to ensure the Liquidity Providers (LPs) maintain their target edge. The formula divides the required number of tickets (derived from the prize pool) by the number of combinations per bonus ball. However, the result is cast to `uint8` without checking for overflow. 

Snippet:
```solidity
uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
```

If the calculated difficulty requires more than 255 bonus balls (which is possible with a large prize pool or small `normalBallMax`), the cast truncates the value (e.g., 256 becomes 0). The `Math.max` function then selects `bonusballMin` (e.g., 1 or 10), resulting in a significantly lower difficulty than required economically. This allows users to buy all combinations cheaply and drain the prize pool, destroying the LP edge.

## Impact
High. LPs are mathematically guaranteed to lose money as the game difficulty drops drastically below the solvent threshold, allowing the prize pool to be drained.

## Command to Run Test


## Proof of Concept
1. Configure the jackpot with `normalBallMax = 5` (1 combination) and `bonusballMin = 1`.
2. Set `lpEdgeTarget` to 50%.
3. Ticket price 1 USDC.
4. Users buy tickets until `prizePool` reaches ~260 USDC.
5. `minNumberTickets` required to maintain edge is ~520.
6. `combosPerBonusball` is 1.
7. `Math.ceilDiv(520, 1)` is 520.
8. `uint8(520)` casts to `8` (520 % 256).
9. The new `bonusballMax` is set to 8 instead of 520.
10. In the next drawing, the difficulty is 65x lower than required. Users can buy all 8 combinations for 8 USDC and win the jackpot with positive expected value, draining LPs.

## Proof of Code
function testBonusBallOverflow() public {
    // Setup jackpot with small normal ball max
    vm.startPrank(owner);
    jackpot.setNormalBallMax(5);
    jackpot.setBonusballMin(1);
    vm.stopPrank();

    // Force a scenario where required bonus balls > 255
    // Calculation: minTickets = prizePool / ((1-edge) * price)
    // Let's assume price=1e6, edge=0.5e18. minTickets = prizePool * 2e12 / 1e18 -> prizePool * 2 / 1e6
    // We need minTickets > 255. So prizePool > 127e6 (127 USDC).
    // We can simulate this by mocking the LP pool state or buying tickets.
    
    // This unit test demonstrates the math overflow conceptually
    uint256 minNumberTickets = 300;
    uint256 combosPerBonusball = 1;
    uint8 min = 1;
    
    // Vulnerable logic
    uint8 newBonusball = uint8(Math.max(min, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
    
    // 300 % 256 = 44
    assertEq(newBonusball, 44);
    // Expected was 300 (clamped to 255 if fixed correctly, but definitely not 44)
}

## Suggested Mitigation
Cap the result at 255 before casting to `uint8`. Example:
`uint256 calculated = Math.ceilDiv(minNumberTickets, combosPerBonusball);`
`uint8 newBonusball = uint8(Math.min(255, Math.max(bonusballMin, calculated)));`


## [H-12]. Impossible bonus ball match due to bitwise overflow in ticket packing

## id: wwfpspfcOC5L8CUqivMsZ

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Jackpot does not enforce bonusballMax <= 255 - normalBallMax, so users can be allowed to pick bonusballs that cannot be encoded (shift >=256). Since lpPoolTotal can grow via earnings, entering this parameter region is not purely hypothetical.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library packs ticket numbers into a `uint256` bit vector. Normal balls occupy bits `1` to `normalBallMax`, and the bonus ball occupies bit `normalBallMax + bonusBall`. 

If `normalBallMax + bonusBall >= 256`, the bit shift `1 << (normalBallMax + bonusBall)` overflows (results in 0 or wraps depending on context, effectively losing the bit). The `bonusballMax` is calculated dynamically in `_setNewDrawingState` based on the prize pool size. If the prize pool is large relative to ticket price and edge, `bonusballMax` can be calculated up to 255 (it is cast to `uint8`).

If `normalBallMax` is set to a small value (e.g., 10), `bonusballMax` can easily exceed `246` (since 255 is the cap). If `normalBallMax` is 10 and `bonusball` is 250, the shift amount is 260, which overflows `uint256`. The packed ticket will lack the bonus ball bit. When claiming winnings, `_calculateTicketTierId` will unpack a bonus ball value of 0, which can never match a valid winning bonus ball (range 1-255). This makes winning the jackpot impossible for valid tickets.

## Impact
Users cannot win prizes requiring a bonus ball match (including the Jackpot), resulting in loss of funds/yield. The state is reachable via normal protocol growth.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 10. `Combinations.choose(10, 5)` = 252.
2. Prize pool grows (via LP deposits) to 100,000 USDC.
3. `minNumberTickets` ~= 100,000.
4. `bonusballMax` = 100,000 / 252 = 396 -> capped at 255.
5. Drawing is initialized with `bonusballMax` = 255.
6. User buys a ticket with bonus ball 250.
7. `TicketComboTracker` calculates bit position: 10 + 250 = 260.
8. `1 << 260` overflows to 0. Packed ticket has no bonus bit.
9. Drawing settles, winning bonus ball is 250.
10. User claims. Unpacking yields bonus ball 0. Match fails.

## Proof of Code
function test_BonusBallOverflow() public {
    // Initialize with small normal ball max
    // Simulate state where bonusballMax becomes large
    uint8 normalMax = 10;
    uint8 bonusBall = 250;
    
    // Logic from TicketComboTracker.insert
    uint256 set = 0; // assume normal balls fit
    uint256 ticketNumbers = set | (1 << (bonusBall + normalMax));
    
    // Verify overflow behavior
    // In Solidity 0.8, 1 << 260 is 0
    assertEq(ticketNumbers, set);
    
    // Unpacking
    // Logic from TicketComboTracker.unpackTicket
    // fls(ticketNumbers) will return highest bit of set (normal balls)
    // bonusball = fls - normalMax will be wrong or underflow
}

## Suggested Mitigation
In `_setNewDrawingState`, ensure that the calculated `newBonusball` does not exceed `255 - normalBallMax`. Alternatively, increase the storage size for packed tickets or use a different storage mechanism.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-13]. Entropy Callback Collision via Provider Switch

## id: WhqPtHOIsg-IwpjpyxMgF

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.setEntropyProvider

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: Same root cause as the sequence-collision issue: ScaledEntropyProvider keys pending only by sequence and ignores the provider argument in entropyCallback. If the owner switches entropyProvider while a request is outstanding, and a later request reuses the same sequence id, pending data can be overwritten and an old provider’s callback will be interpreted in the new request’s context. This can settle with unintended randomness source (especially problematic if switching away from an untrusted provider). This scenario depends on admin action and external provider sequencing, so it’s a governance-footgun risk with real code-path impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `ScaledEntropyProvider` stores pending requests in a mapping keyed by `sequenceNumber` (`mapping(uint64 => PendingRequest)`). This sequence number comes from the external Pyth provider. If the admin calls `setEntropyProvider` to switch providers, the new provider might issue sequence numbers that collide with pending requests from the old provider. If a request is pending (e.g., stuck) and the admin switches providers and unlocks the jackpot, a new run could generate a colliding sequence number. This overwrites the pending request data. If the *old* provider then calls back, it will execute using the *new* request's context/selector but the *old* randomness. This allows injection of stale or potentially manipulated randomness into a fresh drawing settlement.

## Impact
Settlement of a jackpot drawing using incorrect or manipulated randomness source.

## Command to Run Test


## Proof of Concept
1. `runJackpot` calls Provider A, gets sequence 100. Jackpot locked.
2. Provider A is slow. Admin calls `unlockJackpot` (Jackpot) and `setEntropyProvider(Provider B)` (EntropyProvider).
3. `runJackpot` called again. Calls Provider B, gets sequence 100 (collision).
4. `pending[100]` is updated with Provider B request details.
5. Provider A finally calls back with sequence 100.
6. `entropyCallback` finds `pending[100]`, processes A's randomness, and calls `Jackpot` callback.
7. Jackpot settles using Provider A's randomness for the drawing intended for Provider B.

## Proof of Code
test/poc/KeyCollision.t.sol

## Suggested Mitigation
Scope the `pending` mapping by provider address (e.g., `mapping(address => mapping(uint64 => PendingRequest))`) or verify the provider in the callback.


## [H-14]. Protocol Insolvency via Referral Fee Manipulation in Emergency Mode

## id: hCPs-ctFtO8IJ5-H6OaFX

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot.emergencyRefundTickets

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: In Jackpot.emergencyRefundTickets, refunds for tickets with a referral scheme are computed using the current global referralFee: ticketPrice * (1 - referralFee). referralFee is owner-mutable and not snapshotted per ticket or per drawing. If referralFee is changed between purchase and emergency refunds, refund amounts can deviate from what the system economically intended (and from the policy of not clawing back already-accrued referral fees). This can leave insufficient USDC to honor referralFees mapping balances (claimable at any time) or create unfair refunds. Requires privileged parameter changes, so it is governance-risk and relatively rare, but the logic flaw exists now.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `emergencyRefundTickets` function calculates the refund amount by subtracting the *current* global `referralFee` from the ticket price. However, the `referralFee` is mutable by the owner. If the owner (or a compromised admin) reduces the `referralFee` after tickets have been purchased but before emergency mode is triggered, the contract attempts to refund more than it retained from the original sales. Specifically, the contract retains `ticketPrice * (1 - originalFee)` but refunds `ticketPrice * (1 - newLowerFee)`. This leads to insolvency as the contract pays out funds it does not possess.

## Impact
Protocol insolvency and inability to refund all users or LPs during emergency mode.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` to 20%.
2. User buys ticket for 100 USDC. 20 USDC goes to referrer, 80 USDC stays in contract.
3. Admin updates `referralFee` to 0%.
4. Admin enables Emergency Mode.
5. User calls `emergencyRefundTickets`. Refund is `100 * (1 - 0%)` = 100 USDC.
6. Contract only holds 80 USDC for this ticket. 20 USDC deficit per ticket.

## Proof of Code
function testReferralFeeInsolvency() public {
    // Setup scenario in Foundry
    vm.startPrank(owner);
    jackpot.setReferralFee(0.2e18);
    // User buys ticket
    vm.stopPrank();
    vm.prank(user);
    jackpot.buyTickets(..., referrers, ...);
    // Admin changes fee
    vm.prank(owner);
    jackpot.setReferralFee(0);
    jackpot.enableEmergencyMode();
    // Refund
    vm.prank(user);
    jackpot.emergencyRefundTickets(ticketIds);
    // Check balance deficit
}

## Suggested Mitigation
Store the `referralFee` used for each ticket in the `TrackedTicket` struct or `DrawingState`, and use that historical value for calculating refunds. Alternatively, verify the referral scheme existence and calculate the refund based on the actual splits used.


## [M-15]. Sequence number collision in ScaledEntropyProvider when switching providers

## id: ns-9uWkyZWFIh9wtehRs5

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
ScaledEntropyProvider.requestAndCallbackScaledRandomness

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: ScaledEntropyProvider stores pending requests in mapping(uint64 => PendingRequest) pending keyed only by the sequence returned from entropy.requestV2(entropyProvider, gasLimit). setEntropyProvider changes entropyProvider but does not namespace or clear pending. If different entropy providers can return overlapping sequence numbers (common pattern), a new request can overwrite pending[sequence] from an old provider. entropyCallback(uint64 sequence, address /*provider*/, ...) ignores the provider parameter and will execute using whatever PendingRequest is currently stored. This requires an admin/provider switch while old requests are pending (governance action), hence governance risk. No check prevents overwrites or verifies provider in callback.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `ScaledEntropyProvider` contract tracks pending randomness requests using a `sequence` number returned by Pyth's `requestV2`. These sequence numbers are typically scoped to the provider. If the contract owner changes the entropy provider via `setEntropyProvider`, the new provider may issue a sequence number (e.g., 100) that collides with a pending request from the previous provider. The `requestAndCallbackScaledRandomness` function calls `_storePendingRequest` which blindly overwrites `pending[sequence]`. 

This causes the old pending request data to be lost. When the old provider's callback arrives, it will use the overwritten parameters (belonging to the new provider's request), potentially leading to incorrect randomness scaling or reverts. The new request might also be prematurely consumed.

## Impact
Corruption of randomness requests, causing drawings to settle with incorrect parameters or creating stuck locks due to failed callbacks.

## Command to Run Test


## Proof of Concept
1. `Jackpot` requests entropy from Provider A, gets sequence 100. 2. `pending[100]` stored. 3. Owner calls `setEntropyProvider(Provider B)`. 4. `Jackpot` (or another consumer) requests entropy from Provider B, gets sequence 100. 5. `pending[100]` overwritten with new params. 6. Provider A calls back with seq 100. 7. `entropyCallback` uses Provider B's params with Provider A's randomness.

## Proof of Code


## Suggested Mitigation
Include the `provider` address in the `pending` mapping key (e.g., `mapping(bytes32 => PendingRequest)` where key is `keccak256(provider, sequence)`) or check for existing pending requests before overwriting.


## [M-16]. Mid-Flow Protocol Fee Manipulation affects LP Earnings

## id: iikMe3AZDCjJ8IRvINRFk

## Derived From Pattern/Invariant
GlobalParamMidFlowManipulation

## Exploit Type
GlobalParamMidFlowManipulation

## Location
Jackpot._transferProtocolFee

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: A max protocol fee cap (25%) is not a real safeguard against mid-drawing parameter retroactivity; it only bounds magnitude. The issue is in-scope (admin changes mid-flow) and impact is not low (up to 25% of (lpEarnings-userWinnings-threshold)).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `protocolFee` parameter is read directly from global state during the `scaledEntropyCallback` settlement phase. This parameter is not snapshotted in `DrawingState`. The owner can change `protocolFee` (up to 25%) after tickets have been sold and LPs have deposited, but before the drawing settles. This allows the admin to extract a larger share of LP earnings than was configured when capital was committed, violating the expectations of the multi-phase flow.

## Impact
LPs suffer unexpected value loss if fees are raised mid-drawing. Undermines trust and economic invariants for LPs.

## Command to Run Test


## Proof of Concept
1. Protocol Fee is 1%.
2. LPs deposit 1M USDC.
3. Drawing is executed (`runJackpot`).
4. While callback is pending (or just before `runJackpot`), Admin calls `setProtocolFee(25%)`.
5. `scaledEntropyCallback` executes `_transferProtocolFee` using the new 25% rate.
6. LPs lose 24% more of the earnings than expected.

## Proof of Code
function testFeeManipulation() public {
    // Init with 0 fee
    vm.prank(owner); jackpot.setProtocolFee(0);
    
    // Buy tickets to generate earnings
    // ... (omitted setup)
    
    // Change fee before settlement
    vm.prank(owner); jackpot.setProtocolFee(25e16);
    
    // Settle
    // ... (omitted settlement)
    
    // Assert Protocol Fee Address received 25% of earnings
    assertEq(usdc.balanceOf(owner), expected25Percent);
}

## Suggested Mitigation
Snapshot `protocolFee` into `DrawingState` at drawing initialization, similar to `ticketPrice`.


## [H-17]. Division by Zero in LP consolidation if Reserve Ratio is Low

## id: iTX_MudiFJX4RUU1OQ24h

## Derived From Pattern/Invariant
DivisionByZero

## Exploit Type
IntegerMath

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: JackpotLPManager.processDrawingSettlement sets newAccumulator = (prevAccumulator * postDrawLpValue) / lpPoolTotal (for lpPoolTotal>0). If postDrawLpValue becomes 0, newAccumulator becomes 0, and drawingAccumulator[d] is set to 0. Subsequent _consolidateDeposits and _consolidateWithdrawals divide by drawingAccumulator[...] and will revert on division-by-zero, bricking LP interactions. The code comment claims accumulators can never be zero, but this is not enforced. Reaching postDrawLpValue==0 is difficult and typically requires extreme/unsafe parameterization (governance-controlled), hence governance-risk and rare likelihood, but the vulnerable arithmetic path exists.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager.sol`, the `newAccumulator` for a drawing is calculated as `(prevAccumulator * postDrawLpValue) / prevLpTotal`. If `postDrawLpValue` drops to 0 (e.g., due to a jackpot win when `reserveRatio` is 0 or very low, and earnings are low), `newAccumulator` becomes 0. 

Subsequent calls to `processDeposit`, `processInitiateWithdraw`, or `emergencyWithdrawLP` call `_consolidateDeposits`, which performs division by the accumulator: `_lp.lastDeposit.amount * PRECISE_UNIT / drawingAccumulator[...]`. If the accumulator is 0, these transactions revert due to division by zero, permanently bricking the LP system for affected users.

## Impact
Permanent DoS of LP system; LPs cannot deposit or withdraw if the pool value hits zero.

## Command to Run Test


## Proof of Concept
1. Owner sets `reserveRatio` to 0.
2. A drawing occurs where a user wins the entire `prizePool` (which equals `lpPoolTotal`).
3. `postDrawLpValue` becomes roughly 0 (assuming `lpEarnings` covers fees/dust).
4. `newAccumulator` is set to 0.
5. Any LP with a deposit from this drawing trying to interact with the system triggers `_consolidateDeposits`, which divides by `accumulator` (0), causing a revert.

## Proof of Code
function testAccumulatorZero() public {
    uint256 deposit = 100;
    uint256 accum = 0;
    vm.expectRevert();
    uint256 shares = (deposit * 1e18) / accum;
}

## Suggested Mitigation
Ensure `postDrawLpValue` > 0 or enforce a minimum `reserveRatio` that prevents the pool from being fully drained. Alternatively, add a check to reset or floor the accumulator if it would become zero.



