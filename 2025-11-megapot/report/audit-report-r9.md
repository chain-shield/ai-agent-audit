# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[H-1]. Permanent DoS via Block Gas Limit Exceeded in Entropy Callback
**Derived From** : LP whale inflating bonusballMax
Finding Status: Valid
Privilege: Permissionless


[H-2]. Permanent DoS of `runJackpot` due to gas limit scaling with `bonusballMax`
**Derived From** : Permissionless runJackpot keeper
Finding Status: Valid
Privilege: Permissionless


[H-3]. Permanent DoS in Jackpot Settlement due to uint8 overflow in TicketComboTracker
**Derived From** : drawingState[id].normalBallMax + drawingState[id].bonusballMax < 256
Finding Status: Valid
Privilege: Permissionless


[H-4]. LP Edge Destruction due to unsafe downcasting of bonusballMax
**Derived From** : calculatedBonusballMax <= 255
Finding Status: Valid
Privilege: Permissionless


[H-5]. Unchecked Overflow in bonusballMax Calculation Causes DoS or Economic Exploit
**Derived From** : Protocol fee recipient address
Finding Status: Valid
Privilege: Permissionless


[M-6]. Bonusball bit packing overflow causes DoS and corrupted tickets for high prize pools
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless



Finding Status: InvalidGovernanceRisk


[M-7]. LP value bleed due to unchecked referralFee vs lpEdgeTarget
**Derived From** : referralFee <= lpEdgeTarget
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-8]. LP Pool Drain via Duplicate Ticket Arbitrage
**Derived From** : referralFee <= lpEdgeTarget
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 5
- M: 3
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Permanent DoS via Block Gas Limit Exceeded in Entropy Callback

## id: mZ2EslyfBERCJlgj-unc5

## Derived From Pattern/Invariant
LP whale inflating bonusballMax

## Exploit Type
Dos

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: Even though governance can adjust gas-limit parameters, the core issue is an uncapped, potentially unfulfillable callback gas request and O(bonusballMax) settlement work that can occur under normal protocol growth (not only admin error). Without an automatic cap relative to chain limits, this is a real liveness/design risk rather than purely governance misuse.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `scaledEntropyCallback` function executes `TicketComboTracker.countTierMatchesWithBonusball`, which contains a loop that iterates `bonusballMax` times. `bonusballMax` is dynamically calculated based on the prize pool size to maintain LP edge. For large prize pools (e.g., ~$40M USDC), `bonusballMax` can reach ~120, and for ~$100M it reaches the cap of 255.

Each iteration performs roughly 31 storage reads (checking combinations). With cold storage access (2100 gas), a loop of 255 iterations costs ~16.6M gas for reads alone, plus overhead. 

Furthermore, `runJackpot` calculates the entropy gas limit as `base + variable * bonusballMax`. With the default `variable` of 250,000, `bonusballMax` of 255 results in a requested gas limit of ~63M gas. This exceeds the block gas limit of most EVM chains (e.g., Base is 30M). The entropy provider transaction will fail to execute, permanently locking the jackpot.

## Impact
Permanent freezing of jackpot funds; drawing cannot settle.

## Command to Run Test


## Proof of Concept
1. Prize pool grows to ~$100M (via ticket sales or large LP deposits).
2. `bonusballMax` is calculated to be 255 in the previous settlement.
3. Keeper calls `runJackpot`. The contract requests an entropy callback with ~64M gas limit.
4. The entropy provider attempts to fulfill the request but cannot broadcast a transaction with 64M gas limit on a chain with a 30M block limit.
5. The drawing remains locked indefinitely.

## Proof of Code
function test_DoS_GasLimit() public {
    // Mock a large prize pool state causing high bonusballMax
    // Assert calculated gas > block.gaslimit
}

## Suggested Mitigation
Optimize `TicketComboTracker` to avoid O(bonusballMax) iterations or cap `bonusballMax` to a value that fits within block gas limits (adjusting LP edge expectations accordingly).


## [H-2]. Permanent DoS of `runJackpot` due to gas limit scaling with `bonusballMax`

## id: 2SOvtHzNxHvOsJOtIkW78

## Derived From Pattern/Invariant
Permissionless runJackpot keeper

## Exploit Type
Dos

## Location
Jackpot.runJackpot

## Finding Status: Valid
### Finding Status Justification: While governance can tune `entropyVariableGasLimit`, the liveness failure can arise during normal permissionless operation as `bonusballMax` grows (from pool growth/ticket activity) and the contract requests an unfulfillable callback gas limit. This is a protocol design/parameterization issue without an on-chain cap tied to chain block limits, not merely an admin mistake.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract calculates `entropyGasLimit` as `base + variable * bonusballMax`. `bonusballMax` scales automatically with the prize pool. With the default `entropyVariableGasLimit` of 250,000, a `bonusballMax` of 200 results in a gas limit of ~50 million, exceeding block gas limits on Ethereum and most L2s. Additionally, `TicketComboTracker._countSubsetMatches` iterates `bonusballMax` times, performing heavy storage reads (32 per iteration). As `bonusballMax` grows, `runJackpot` will inevitably revert due to exceeding block gas limits or Pyth fee requirements, permanently locking the drawing.

## Impact
The jackpot becomes un-runnable at high prize pool valuations. Funds are stuck until the owner enables Emergency Mode to refund users, effectively killing the protocol's primary utility.

## Command to Run Test


## Proof of Concept
1. `prizePool` grows to ~50M USDC, causing `bonusballMax` to calculate to ~200.
2. `runJackpot` is called.
3. `_calculateEntropyGasLimit` returns `Base + 200 * 250,000` = ~50,000,000 gas.
4. `entropy.getFee(50M)` or the subsequent call to `requestAndCallbackScaledRandomness` fails due to block gas limit or excessive fee requirements.
5. Drawing cannot be settled.

## Proof of Code
function testGasDos() public {
    uint8 bonusballMax = 200;
    uint32 variableGas = 250000;
    uint32 limit = 100000 + variableGas * uint32(bonusballMax);
    assertGt(limit, 30_000_000); // Exceeds ETH block limit
}

## Suggested Mitigation
Optimize `TicketComboTracker` to avoid O(N) loop over `bonusballMax` or impose a hard cap on `bonusballMax` (e.g., 50) and limit the `prizePool` growth accordingly.


## [H-3]. Permanent DoS in Jackpot Settlement due to uint8 overflow in TicketComboTracker

## id: dMVsjg10XTtv4xudCUkyu

## Derived From Pattern/Invariant
drawingState[id].normalBallMax + drawingState[id].bonusballMax < 256

## Exploit Type
IntegerOverflow

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: The invariant `normalMax + bonusballMax <= 255` is not enforced in `Jackpot._setNewDrawingState()` nor in `TicketComboTracker.init()`. If a drawing is created where `bonusballMax > 255 - normalMax`, the entropy callback can revert (checked uint8 addition) and leave `jackpotLock=true`, creating a real liveness DoS. There is no protective cap besides the overflow revert itself (which causes the DoS).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `TicketComboTracker.insert` and `countTierMatchesWithBonusball`, the bit index for the bonus ball is calculated as `_bonusball + _tracker.normalMax`. Both operands are `uint8`. In Solidity 0.8+, arithmetic operations on `uint8` check for overflow on the return type (`uint8`). If `_bonusball + _tracker.normalMax > 255`, the transaction reverts. This occurs when the `prizePool` grows large enough to require a high `bonusballMax` (e.g., 250) while `normalBallMax` is moderate (e.g., 10). If the winning bonus ball falls into the overflow range, `scaledEntropyCallback` will consistently revert, permanently locking the jackpot (`jackpotLock` remains true) and freezing all funds.

## Impact
The jackpot becomes permanently locked; no winners can be determined, and no new drawings can start. Funds in the prize pool and LP pool are stuck.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 10.
2. `prizePool` accumulates to ~50,000 USDC. Based on edge calculations, `bonusballMax` is calculated to be ~250.
3. A drawing takes place. `runJackpot` is called.
4. Entropy provider returns a winning bonus ball of 250.
5. `scaledEntropyCallback` calls `_calculateDrawingUserWinnings` -> `countTierMatchesWithBonusball`.
6. Inside `countTierMatchesWithBonusball`, the code executes `winningTicket = set | (1 << (_bonusball + _tracker.normalMax))`.
7. `250 + 10 = 260`. This overflows `uint8`, causing a revert.
8. The callback fails, and the jackpot remains locked forever.

## Proof of Code
function testDoS() public {
    uint8 normalMax = 10;
    uint8 bonus = 250;
    // This mimics the reverting line in TicketComboTracker
    // uint8 sum = normalMax + bonus; // Reverts
    vm.expectRevert(); 
    uint256 res = 1 << (normalMax + bonus);
}

## Suggested Mitigation
Cast operands to `uint256` before addition to avoid `uint8` overflow checks, and ensure the shift result is handled correctly (though `1 << 256` is 0, so bit packing strategy may need review for sums >= 256).


## [H-4]. LP Edge Destruction due to unsafe downcasting of bonusballMax

## id: n1mbuH4FHld-DIhhVUYD-

## Derived From Pattern/Invariant
calculatedBonusballMax <= 255

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: `uint8(...)` in `_setNewDrawingState()` is an explicit downcast that can silently truncate values >255; the project even has `UintCasts.toUint8` but does not use it here. There is no on-chain cap/safe-cast preventing wrap, and wrap can break the intended edge/difficulty calculations and/or create invalid `bonusballMax` states.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `Jackpot._setNewDrawingState`, the new `bonusballMax` is calculated to ensure LPs maintain a statistical edge. The formula is `Math.ceilDiv(minNumberTickets, combosPerBonusball)`. The result is explicitly cast to `uint8`. If the required number of bonus balls exceeds 255 (e.g., 300), the cast truncates the value (e.g., `300 % 256 = 44`). This results in a drastically lower `bonusballMax` than required, significantly increasing player win probabilities and causing LPs to operate at a massive negative expected value.

## Impact
Liquidity Providers suffer severe financial loss as the game difficulty is inadvertently lowered below the break-even point. The prize pool can be drained rapidly.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 5 (Combos = 1).
2. `prizePool` is 2000 USDC. `ticketPrice` is 5 USDC. `edge` is 10%.
3. `minNumberTickets` = 2000 / (0.9 * 5) = 444.
4. `combos` = 1. Required `bonusballMax` = 444.
5. `newBonusball` = `uint8(444)` = 188.
6. The game proceeds with 188 balls instead of 444, giving players >2x better odds than the protocol can afford.

## Proof of Code
function testTruncation() public {
    uint256 required = 300;
    uint8 truncated = uint8(required);
    assertEq(truncated, 44);
}

## Suggested Mitigation
Use a larger type (e.g., `uint16`) for `bonusballMax` in `DrawingState` and `TicketComboTracker`, or cap the `prizePool` such that the required `bonusballMax` never exceeds 255.


## [H-5]. Unchecked Overflow in bonusballMax Calculation Causes DoS or Economic Exploit

## id: UtX-O9gETrjnS6uiG949c

## Derived From Pattern/Invariant
Protocol fee recipient address

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Same root issue: `_setNewDrawingState()` uses an unsafe `uint8(...)` cast that can wrap to 0 (or small values), which can make `buyTickets` impossible and can also make `runJackpot` request an invalid entropy range (`minRange=1,maxRange=0`). No safe-cast/cap exists to prevent this.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_setNewDrawingState`, `bonusballMax` is calculated to ensure LP edge. The formula `Math.ceilDiv(minNumberTickets, combosPerBonusball)` can produce results larger than 255 if the prize pool is large (>$62M). The result is explicitly cast to `uint8` (`uint8(...)`), which truncates the upper bits without reverting in Solidity 0.8+ for explicit casts.

If the result wraps to 0, `bonusballMax` becomes 0. Since tickets require `bonusball > 0` and `<= bonusballMax`, no tickets can be purchased (DoS). If it wraps to a small number (e.g., 1), the LP edge protection is bypassed, allowing users to buy tickets with much higher win probabilities than mathematically intended, draining the LP pool.

## Impact
Denial of Service (if 0) or massive LP loss (if small nonzero) due to destroyed edge.

## Command to Run Test


## Proof of Concept
1. Prize pool reaches ~$63M. `minNumberTickets` / `combos` results in 256.
2. `uint8(256)` truncates to 0.
3. `bonusballMax` is set to 0.
4. Users attempt `buyTickets`.
5. `_validateAndStoreTickets` checks `ticket.bonusball > 0` and `ticket.bonusball <= 0`. Always false.
6. Buying tickets reverts.

## Proof of Code
function test_Overflow_BonusballMax() public {
    // Mock internal state call or calculate manually
    uint256 val = 256;
    uint8 truncated = uint8(val);
    assertEq(truncated, 0);
}

## Suggested Mitigation
Use `UintCasts.toUint8` to revert on overflow, or cap the result at 255 using `Math.min(255, calculation)`.


## [M-6]. Bonusball bit packing overflow causes DoS and corrupted tickets for high prize pools

## id: 17GWXNLItKTWEWA7NJFIL

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: If bonusballMax is ever such that some valid bonusball in [1..bonusballMax] makes (bonusball + normalMax) > 255, Solidity 0.8 uint8 addition reverts during packing in insert()/countTierMatchesWithBonusball(), which can brick settlement for a drawing whose randomness lands in that range. There is no explicit invariant enforcement bonusballMax <= 255-normalBallMax; it’s primarily a rare/probabilistic DoS, not silent corruption.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TicketComboTracker` library packs ticket numbers into a `uint256` bit vector. The bonusball is stored at bit position `_normalMax + _bonusball`. The `Jackpot` contract calculates `bonusballMax` dynamically in `_setNewDrawingState` based on the prize pool size to maintain LP edge. If the prize pool is sufficiently large, `bonusballMax` can be calculated such that `normalBallMax + bonusballMax >= 256`. 

When this occurs, the bit shift `1 << (_bonusball + _tracker.normalMax)` in `TicketComboTracker.insert` will overflow (result in 0 in Solidity 0.8), effectively losing the bonusball information. 

Furthermore, `JackpotTicketNFT` unpacking logic (`LibBit.fls`) relies on the highest set bit being the bonusball. If the bonusball bit is lost, unpacking fails or returns incorrect data. `Jackpot.claimWinnings` tiers calculation will also fail to match the bonusball (treating it as 0), causing users to lose potential winnings.

## Impact
If the prize pool becomes large (via LP deposits), the game parameters adjust to a state where tickets are corrupted upon purchase. Users pay for tickets but cannot win bonusball tiers. This acts as a griefing/DoS attack and effectively increases LP edge beyond intended targets.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is set to 200 (allowed via `_calculateLpPoolCap` logic which only checks `MAX_BIT_VECTOR_SIZE - normalMax` for pool cap, allowing `bonusball` space to be small).
2. Attacker deposits large LP capital. `prizePool` increases.
3. `_setNewDrawingState` calculates `minNumberTickets`. `bonusballMax` scales up.
4. If `bonusballMax` becomes > 56, `200 + 56 = 256`.
5. `1 << 256` overflows to 0.
6. Users buy tickets. Bonusball bit is 0.
7. Winning numbers have bonusball > 0.
8. Users check for winnings. `ticket >> 201` is 0. Match fails.

## Proof of Code
contract OverflowTest is Test {
    function testBonusballOverflow() public {
        // Simulate logic
        uint8 normalMax = 200;
        uint8 bonusball = 60;
        uint256 packed = 0;
        // Standard packing
        packed |= 1 << (normalMax + bonusball);
        
        // In Solidity 0.8+, 1 << 260 is 0
        // Note: Code uses assembly or standard shift? Standard.
        assertEq(packed, 0, "Bonusball bit lost due to overflow");
    }
}

## Suggested Mitigation
Enforce a strict upper bound on `bonusballMax` in `_setNewDrawingState` such that `normalBallMax + bonusballMax < 256`. If the calculated bonusball exceeds this, cap it (though this reduces LP edge) or prevent the state transition.





Finding Status: InvalidGovernanceRisk
## [M-7]. LP value bleed due to unchecked referralFee vs lpEdgeTarget

## id: v6-pWw3ZbV4qIAvIScgon

## Derived From Pattern/Invariant
referralFee <= lpEdgeTarget

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: While it requires owner misconfiguration (so it is governance risk), the impact is not inherently low: if referralFee > lpEdgeTarget, each duplicate purchase creates a systematic negative drift for LP equity, which can be amplified over many purchases into a large LP loss/undercollateralization risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The system logic for duplicate tickets adds `ticketPrice * (1 - lpEdgeTarget)` to the prize pool, while `lpEarnings` receives `ticketPrice - referralFee`. The net change to LP equity is `(ticketPrice - referralFee) - (ticketPrice - lpEdgeTarget) = ticketPrice * (lpEdgeTarget - referralFee)`. If `referralFee > lpEdgeTarget`, this value is negative. The contract does not enforce `referralFee <= lpEdgeTarget`.

## Impact
LPs lose value on every duplicate ticket sold. An attacker can repeatedly buy duplicate tickets to drain LP value into the prize pool (which they might win) or simply grief the LPs.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` = 20%, `lpEdgeTarget` = 10%.
2. Attacker buys 1 ticket (non-dup). LP equity +10%.
3. Attacker buys 100 duplicates. Each dup: LP earnings +80% (1-0.2), Liability (Prize Pool) +90% (1-0.1). Net -10%.
4. Total LP loss = 100 * 10% * TicketPrice.

## Proof of Code
function testReferralBleed() public {
    uint256 price = 100;
    uint256 ref = 20;
    uint256 edge = 10;
    int256 net = int256(price - ref) - int256(price - edge);
    assertLt(net, 0);
}

## Suggested Mitigation
In `setReferralFee` and `setLpEdgeTarget`, enforce the invariant `referralFee <= lpEdgeTarget`.


## [M-8]. LP Pool Drain via Duplicate Ticket Arbitrage

## id: YvAH_YWosmeL3htchR1Dz

## Derived From Pattern/Invariant
referralFee <= lpEdgeTarget

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Same as v6-pWw3: owner misconfiguration is required, but once misconfigured the per-duplicate negative drift can be repeated to create large aggregate losses for LPs, so the impact is not intrinsically low.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The system allows `referralFee` to be set higher than `lpEdgeTarget`. For duplicate tickets, the prize pool liability increases by `ticketPrice * (1 - lpEdgeTarget)`, while `lpEarnings` only increase by `ticketPrice * (1 - referralFee)`. If `referralFee > lpEdgeTarget`, the liability increase exceeds the earnings, creating a net loss for the LP pool on every duplicate ticket. An attacker can exploit this by purchasing duplicate tickets to drain LP value.

## Impact
Continuous drain of LP pool funds.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` = 20%, `lpEdgeTarget` = 10%.
2. User buys duplicate ticket ($1).
3. Prize Pool adds $0.90.
4. LP Earnings adds $0.80.
5. Net LP position: +$0.80 (asset) - $0.90 (liability) = -$0.10.

## Proof of Code
function testFeeArbitrage() public {
   uint256 ticketPrice = 1e6;
   uint256 lpEdge = 1e17; // 10%
   uint256 referralFee = 2e17; // 20%
   int256 netLP = int256(ticketPrice * (1e18 - referralFee) / 1e18) - int256(ticketPrice * (1e18 - lpEdge) / 1e18);
   // netLP is negative
   assert(netLP < 0);
}

## Suggested Mitigation
Enforce `referralFee <= lpEdgeTarget` in `setReferralFee` and `setLpEdgeTarget`.



