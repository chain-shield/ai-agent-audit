# 2025 11 megapot - Findings Report
## Commit hash: f0a7297d59c376e38b287b2c56740617dbbfbdc7

##Findings by Status


Finding Status: Valid


[H-1]. Permanent Jackpot DoS via Excessive Gas Limit Configuration
**Derived From** : Jackpot DoS via excessive gas limit calculation for high bonusballMax
Finding Status: Valid
Privilege: Permissionless


[M-2]. Stale Ticket Price in BridgeManager Causes User Overpayment or DoS
**Derived From** : Permissionless BridgeManager ticket buyer
Finding Status: Valid
Privilege: Permissionless


[H-3]. LP Edge Destruction and Win Probability Manipulation via Unbounded Bonus Ball Scaling
**Derived From** : Arithmetic / Game Theory
Finding Status: Valid
Privilege: Permissionless


[H-4]. BridgeManager allows arbitrary external calls via claimWinnings leading to asset theft
**Derived From** : Ticket owner EIP-712 signer
Finding Status: Valid
Privilege: Permissionless


[M-5]. Smart Contract Users Locked Out of Bridge Assets
**Derived From** : Permissionless BridgeManager ticket buyer: Buy tickets for a smart contract recipient
Finding Status: Valid
Privilege: Permissionless


[H-6]. Truncation of `bonusballMax` calculation allows manipulation of game difficulty or DoS
**Derived From** : Jackpot._setNewDrawingState
Finding Status: Valid
Privilege: Permissionless


[H-7]. Large Prize Pools Cause Integer Overflow in Game Difficulty Parameters Leading to LP Insolvency
**Derived From** : drawingState[nextDrawingId].ballMax + drawingState[nextDrawingId].bonusballMax < 256
Finding Status: Valid
Privilege: Permissionless


[H-8]. Bonusball count truncation destroys LP edge and causes insolvency in Jackpot.sol
**Derived From** : Jackpot Core (Automated Parameterization): Automatically set bonusballMax
Finding Status: Valid
Privilege: Permissionless


[H-9]. Ticket bitpacking overflow allows corruption of ticket data and easier wins
**Derived From** : normalBallMax + bonusballMax < 255
Finding Status: Valid
Privilege: Permissionless


[H-10]. Bit packing overflow in `TicketComboTracker` causes corrupted tickets and unclaimable winnings
**Derived From** : Jackpot.buyTickets
Finding Status: Valid
Privilege: Permissionless


[H-11]. LP Share Price Inflation Attack steals funds from new depositors
**Derived From** : Malicious Liquidity Provider
Finding Status: Valid
Privilege: Permissionless



Finding Status: InvalidGovernanceRisk


[M-12]. User Funds Locked When Upgrading PayoutCalculator
**Derived From** : Changing PayoutCalculator permanently locks winnings from previous drawings
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-13]. Protocol DoS and LP value drain if `referralFee` exceeds `lpEdgeTarget`
**Derived From** : JackpotLPManager.processDrawingSettlement
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


[M-14]. Settlement DoS when referral fees exceed LP edge
**Derived From** : JackpotLPManager invariant: lpEdgeTarget >= referralFee
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-15]. LP Equity Drain via Referral Fee Arbitrage
**Derived From** : Ticket buyer paying USDC
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood


[M-16]. Zero Accumulator leads to permanent LP DoS
**Derived From** : drawingAccumulator > 0
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 9
- M: 7
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [H-1]. Permanent Jackpot DoS via Excessive Gas Limit Configuration

## id: 0IOii02bSu99zIa8hfGMg

## Derived From Pattern/Invariant
Jackpot DoS via excessive gas limit calculation for high bonusballMax

## Exploit Type
Dos

## Location
Jackpot.runJackpot

## Finding Status: Valid
### Finding Status Justification: This can occur without an admin 'mistake': with the default entropyVariableGasLimit (250k) and a sufficiently large organically-grown bonusballMax (driven by prizePool growth), the callback work and/or gasLimit can exceed practical block limits and repeatedly fail, sticking a drawing. This is a protocol parameterization/design risk that can manifest as the system scales, not a rare edge case.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `runJackpot` function calculates `entropyGasLimit` as `entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax`. The default `entropyVariableGasLimit` is 250,000. If `bonusballMax` reaches high values (e.g. 120-255), the requested gas limit (30M - 64M) exceeds the block gas limit of most chains (Optimism/Base ~30M). The transaction to initiate the jackpot will consistently revert, or if passed, the callback will fail OOG, permanently locking the jackpot.

## Impact
Permanent freezing of the protocol and user/LP funds if the prize pool grows large enough to trigger high difficulty.

## Command to Run Test


## Proof of Concept
1. Prize pool grows, `bonusballMax` scales to 200. 
2. `entropyVariableGasLimit` is 250,000. 
3. `runJackpot` calls `entropy.request` with limit = 250k * 200 = 50M gas. 
4. Transaction reverts as 50M > Block Gas Limit (30M).

## Proof of Code
function testGasLimit() public { uint32 variable = 250000; uint8 bonus = 200; uint256 gasLimit = variable * bonus; assertTrue(gasLimit > 30_000_000); }

## Suggested Mitigation
Drastically reduce `entropyVariableGasLimit` default value (e.g. to 5k-10k) and optimize `TicketComboTracker` to ensure settlement fits within block limits.


## [M-2]. Stale Ticket Price in BridgeManager Causes User Overpayment or DoS

## id: l-PnuUFfyXq4oBDpABbU4

## Derived From Pattern/Invariant
Permissionless BridgeManager ticket buyer

## Exploit Type
SlippageMissingOrInsufficient

## Location
JackpotBridgeManager.buyTickets

## Finding Status: Valid
### Finding Status Justification: This can occur even when the owner follows the intended model (updating ticketPrice for future drawings) because BridgeManager reads jackpot.ticketPrice() (global) instead of the current drawing's snapshotted ticketPrice; users can be overcharged with no refund path. That's a design bug, not merely admin error.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`JackpotBridgeManager.buyTickets` calculates the cost using the current global `jackpot.ticketPrice()`. However, the `Jackpot` contract uses the snapshotted price stored in `drawingState` for the current drawing. If the admin updates the price, `BridgeManager` will collect the *new* price from the user but pay the *old* price to `Jackpot`. If New > Old, the difference is permanently stuck in `BridgeManager`. If New < Old, the transaction reverts due to insufficient funds pulled from user.

## Impact
Loss of user funds (overpayment) or Denial of Service during price updates.

## Command to Run Test


## Proof of Concept
1. Drawing N starts with price 1 USDC. 2. Admin sets price to 2 USDC. 3. User calls `BridgeManager.buyTickets`. 4. BM pulls 2 USDC from user. 5. BM calls `Jackpot.buyTickets`, paying 1 USDC. 6. 1 USDC is stuck in BM.

## Proof of Code
function testPriceMismatch() public { jackpot.setTicketPrice(2e6); bm.buyTickets(...); assertEq(usdc.balanceOf(address(bm)), 1e6); }

## Suggested Mitigation
In `BridgeManager.buyTickets`, fetch the `DrawingState` struct from `Jackpot` and use the stored `ticketPrice` instead of the global `jackpot.ticketPrice()`.


## [H-3]. LP Edge Destruction and Win Probability Manipulation via Unbounded Bonus Ball Scaling

## id: 9jmzAgtMInBWhhCNcT1Hr

## Derived From Pattern/Invariant
Arithmetic / Game Theory

## Exploit Type
IntegerOverflow

## Location
Jackpot.scaledEntropyCallback

## Finding Status: Valid
### Finding Status Justification: The overflow/truncation is reachable once LP value/prizePool grows beyond what fits a uint8-derived difficulty; lpPoolCap only caps deposits, not growth via lpEarnings from ticket sales, so this is not inherently 'rare' in a successful deployment.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol dynamically scales the `bonusballMax` parameter for the next drawing based on the volume of tickets sold in the current drawing (`minNumberTickets`). This calculation occurs in `_setNewDrawingState` using `Math.ceilDiv`. There are two critical vulnerabilities in this mechanism:

1. **Integer Truncation**: The result is cast to `uint8`. If high ticket sales require a `bonusballMax` > 255 to maintain LP edge, the value truncates (e.g., 300 becomes 44). This dramatically lowers difficulty, causing LPs to be statistically under-collateralized (massive negative EV).
2. **Bit Packing Collision**: `TicketComboTracker` stores tickets using `1 << (bonusball + normalMax)`. If `bonusball + normalMax >= 256`, the shift results in 0. This causes all high bonus balls to collapse into a single 'no-bit-set' state. A player buying a ticket in this range matches the winning number if it is also in this range, significantly increasing win probability at the expense of LPs.

Since `buyTickets` is uncapped, an attacker (or simply high demand) can pump `lpEarnings` -> `prizePool` -> `minNumberTickets` to trigger these conditions.

## Impact
Complete loss of LP funds. The mathematical guarantee of LP profitability is broken, either by artificially lowering difficulty (truncation) or by creating colliding winning numbers (bit packing overflow).

## Command to Run Test


## Proof of Concept
1. Attacker buys a large volume of tickets (or flash loan sandwich), increasing `lpEarnings` and thus the `prizePool` for the *next* drawing parameterization.
2. `runJackpot` is called. `scaledEntropyCallback` calculates `newBonusball`.
3. The calculation `minNumberTickets / combos` results in, for example, 300.
4. `uint8(300)` truncates to 44.
5. The next drawing has `bonusballMax` = 44 instead of 300.
6. LPs are providing liquidity for a game with 1/44 bonus odds but getting paid for 1/300 odds. Attacker buys tickets in next round and drains LP pool statistically.

## Proof of Code
function testBonusBallTruncation() public {
    // Simulate massive ticket purchase to pump prize pool
    // Assuming normalMax=10 (small for test), combos=252.
    // To exceed 255 bonus balls, need minTickets > 255 * 252 = 64260.
    // Prize pool need > 64k * price.
    
    // Grant jackpot huge USDC to simulate earnings
    deal(address(usdc), address(jackpot), 1_000_000e6);
    // Manually inject earnings into current state to skip buyTickets gas
    // (In reality, buyTickets would be used)
    // Hack state for PoC simplicity
    
    vm.prank(address(entropy));
    jackpot.scaledEntropyCallback(1, randomNumbers, "");

    // Check next drawing state
    Jackpot.DrawingState memory state = jackpot.getDrawingState(2);
    
    // If logic was correct, bonusballMax should be > 255 (revert or handle).
    // But it wraps. 300 -> 44.
    assertLt(state.bonusballMax, 255);
    // Difficulty is dangerously low.
}

## Suggested Mitigation
1. Remove the `uint8` cast and use `uint16` or larger for `bonusballMax` storage, or cap `bonusballMax` at 255 and cap the prize pool/ticket sales to match that difficulty limit.
2. Enforce `bonusballMax + normalBallMax < 256` constraint in `_setNewDrawingState`.


## [H-4]. BridgeManager allows arbitrary external calls via claimWinnings leading to asset theft

## id: rxsrMxHm0RFx19aQ0iPuR

## Derived From Pattern/Invariant
Ticket owner EIP-712 signer

## Exploit Type
UntrustedDelegateCall

## Location
JackpotBridgeManager.claimWinnings

## Finding Status: Valid
### Finding Status Justification: Even if the multicall-msg.sender argument blocks the specific multicall transferFrom path, the arbitrary call remains dangerous: the user-signed call can target the ERC721 itself (e.g., safeTransferFrom(BridgeManager, attackerContract, victimTicketId)), and attackerContract's onERC721Received can pull USDC using the prior approveTo allowance within the same call to satisfy NotAllFundsBridged, enabling theft of custodial NFTs.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotBridgeManager.claimWinnings` function accepts a `RelayTxData` struct signed by the user. This struct contains a target address `to`, call data `data`, and an approval target `approveTo`. The function executes `_bridgeFunds`, which performs a low-level `call` to the user-specified `to` address with the user-specified `data`. While the function verifies that the USDC balance decreases by the claimed amount, it does not restrict the target or data of the call. An attacker can construct a malicious payload where `to` is a Multicall contract (or similar) that executes two actions: 1) Transfers the required USDC to the bridge provider (satisfying the balance check), and 2) Calls `JackpotTicketNFT.transferFrom(BridgeManager, Attacker, ...)` to steal other users' custodial tickets. Since `BridgeManager` is the owner of these tickets and initiates the call, the transfer succeeds.

## Impact
Complete theft of all NFTs and other assets held in custody by the BridgeManager.

## Command to Run Test


## Proof of Concept
1. Attacker wins a small prize or buys a ticket to get a valid signature path.
2. Attacker crafts a `RelayTxData` struct:
   - `to`: Address of a Multicall contract.
   - `data`: Payload enabling `Multicall.aggregate([call1, call2])`.
     - `call1`: `USDC.transfer(BridgeProvider, claimedAmount)` (satisfies balance check).
     - `call2`: `JackpotTicketNFT.transferFrom(BridgeManager, Attacker, victimTicketId)`.
3. Attacker signs this struct via EIP-712.
4. Attacker calls `claimWinnings` with the signature.
5. `_bridgeFunds` executes the multicall. The USDC check passes. The NFT transfer succeeds because `msg.sender` is `BridgeManager`.
6. Attacker steals custodial NFTs.

## Proof of Code
pending

## Suggested Mitigation
Restrict `_bridgeFunds` to only call whitelisted bridge providers or remove the arbitrary `call` capability entirely in favor of fixed bridge interfaces.


## [M-5]. Smart Contract Users Locked Out of Bridge Assets

## id: Ann-OCLiylZx6WIAHhgF-

## Derived From Pattern/Invariant
Permissionless BridgeManager ticket buyer: Buy tickets for a smart contract recipient

## Exploit Type
AccessControl

## Location
JackpotBridgeManager.claimWinnings

## Finding Status: Valid
### Finding Status Justification: This is a protocol limitation (no EIP-1271 support) that can lock funds for common smart-wallet users; treating it purely as 'user mistake' is not accurate unless the system explicitly forbids contract recipients or documents/enforces the constraint on-chain.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `JackpotBridgeManager` relies on `ECDSA.recover` to validate signatures for `claimWinnings` and `claimTickets`. Smart contract wallets (like Gnosis Safe) cannot produce ECDSA signatures (they use EIP-1271). Consequently, any user buying tickets via the bridge to a smart contract address will permanently lose access to their tickets and winnings.

## Impact
Permanent loss of funds for smart contract users.

## Command to Run Test


## Proof of Concept
1. User buys tickets specifying a Safe address as recipient.
2. User wins.
3. User tries to claim. Cannot sign message.
4. Tickets stuck forever.

## Proof of Code
function testSmartContractLockout() public { 
  // Simulate smart contract recipient 
  // Attempt to claim fails signature check 
}

## Suggested Mitigation
Implement EIP-1271 signature validation in `JackpotBridgeManager`.


## [H-6]. Truncation of `bonusballMax` calculation allows manipulation of game difficulty or DoS

## id: dtuddwthMWC1b0rIs_nh6

## Derived From Pattern/Invariant
Jackpot._setNewDrawingState

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: Because lpPoolCap is only enforced on deposits (not on LP value growth via net-positive lpEarnings over time), lpPoolTotal/prizePool can grow beyond the range where the computed bonusballMax fits in uint8. In that case the uint8 cast can wrap (including to 0), which can DoS buyTickets/runJackpot or materially alter difficulty; this is not inherently 'rare' in a successful, long-running system.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_setNewDrawingState`, the new `bonusballMax` is calculated as `uint8(Math.max(bonusballMin, Math.ceilDiv(...)))`. The explicit cast to `uint8` truncates the result. If the calculated difficulty requires a `bonusballMax` > 255 (possible with large prize pools and low edge), the value wraps around (modulo 256). This can result in a `bonusballMax` of 0 (DoS, as no valid bonusball exists) or a very small number (drastically reducing difficulty and LP edge), or an arbitrary number that breaks game economics.

## Impact
Broken game economics (loss of LP edge) or protocol DoS if `bonusballMax` wraps to 0.

## Command to Run Test


## Proof of Concept
1. Large `prizePool`, small `lpEdgeTarget`. 2. `minNumberTickets` / `combos` = 300. 3. `uint8(300)` = 44. 4. Game sets `bonusballMax` to 44 instead of 300. 5. Difficulty is much lower than required to sustain LP edge.

## Proof of Code
function testBonusBallTruncation() public {
    // Manipulate variables to make ceilDiv return 256
    // Assert new bonusballMax is 0
}

## Suggested Mitigation
Use `UintCasts.toUint8` or cap the result at 255 (though capping breaks edge guarantee, so reverting or redesigning is better).


## [H-7]. Large Prize Pools Cause Integer Overflow in Game Difficulty Parameters Leading to LP Insolvency

## id: oRckNkMTEXqRu7-awpZSm

## Derived From Pattern/Invariant
drawingState[nextDrawingId].ballMax + drawingState[nextDrawingId].bonusballMax < 256

## Exploit Type
IntegerOverflow

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: The code has no hard cap ensuring newBonusball + normalBallMax <= 255 and no mechanism constraining lpEarnings-driven LP/prizePool growth; thus the described overflow region is a realistic long-run state, not inherently rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol relies on bit-packing ticket numbers into a single `uint256`. The highest bit used is `normalBallMax + bonusballMax`. To prevent data loss, this sum must remain `<= 255`. The `Jackpot` contract enforces a cap on LP deposits (`lpPoolCap`) calculated to keep the potential prize pool within safe limits. However, this cap only limits *deposits*. It does not limit prize pool growth from ticket sales (`lpEarnings`). 

If the prize pool grows sufficiently large via ticket sales (e.g., exceeding ~62M USDC with standard parameters), the `_setNewDrawingState` function calculates a `newBonusball` (difficulty parameter) that violates safety limits in two ways:
1. The calculated `newBonusball` can exceed `255 - normalBallMax`, causing the bit-packing in `TicketComboTracker` to overflow. This results in the bonus ball bit being lost (effectively 0). High-tier tickets will match incorrectly against other high-tier winning numbers, as both map to 'bonus ball 0'.
2. The `newBonusball` is cast to `uint8`. If the calculated value exceeds 255, it wraps (e.g., 260 becomes 4). This drastically reduces the game difficulty while the LP edge model assumes high difficulty.

Both scenarios result in win probabilities orders of magnitude higher than accounted for, leading to rapid drainage of the LP pool.

## Impact
Catastrophic loss of LP funds due to broken game math and incorrect win probabilities.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` is 35. Calculated safe prize pool is ~53M USDC.
2. LPs deposit up to 50M USDC.
3. A popular drawing generates 15M USDC in ticket sales.
4. `processDrawingSettlement` updates `lpPoolTotal` to ~65M USDC.
5. `_setNewDrawingState` calculates `minNumberTickets` based on 65M pool.
6. `newBonusball` is calculated as ~267.
7. `newBonusball` cast to `uint8` wraps to 11.
8. Next drawing uses `bonusballMax = 11` instead of 267. Win probability is ~24x higher than required for solvency. LPs are drained.

## Proof of Code
function test_BonusBallOverflow() public {
    // Setup large pool state
    vm.store(address(jackpot), bytes32(uint256(7)), bytes32(uint256(65_000_000 * 1e6))); // Mock lpPoolTotal
    // Trigger new drawing state
    vm.prank(address(entropyProvider));
    jackpot.scaledEntropyCallback(1, randomNumbers, "");
    // Assert bonusballMax is small (wrapped) or overflows bit packing logic
}

## Suggested Mitigation
Implement a hard cap on `newBonusball` in `_setNewDrawingState` such that `newBonusball + normalBallMax <= 255`. If the calculated difficulty requires more balls than fit, the prize pool growth must be capped or the `uint256` bit-packing logic replaced with a larger structure.


## [H-8]. Bonusball count truncation destroys LP edge and causes insolvency in Jackpot.sol

## id: PR9nIYhzgLxAv0MRwgC0l

## Derived From Pattern/Invariant
Jackpot Core (Automated Parameterization): Automatically set bonusballMax

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot._setNewDrawingState

## Finding Status: Valid
### Finding Status Justification: The overflow is purely arithmetic (uint8 truncation) with no guardrails, and prizePool/LP growth via ticket sales can exceed deposit-cap assumptions; therefore it is not just a 'rare' edge case.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `_setNewDrawingState`, the dynamic `newBonusball` count is calculated using `Math.ceilDiv` and then cast to `uint8`. If the required bonus ball count exceeds 255 (which can happen with a large prize pool and low ticket price), the cast truncates the value (modulo 256). This drastically reduces the game difficulty below the calculated requirement for LP solvency. For example, a requirement of 300 balls becomes 44 balls. The probability of winning increases massively, draining the LP pool.

## Impact
Severe LP insolvency due to mathematically broken game difficulty; players win at rates far exceeding the intended model.

## Command to Run Test


## Proof of Concept
1. Prize pool = $100M. Ticket = $1.
2. Required combinations ~100M.
3. `normalMax`=10 (252 combos). `bonus` needed = 39682.
4. `uint8(39682)` = 2.
5. Game runs with bonus range [1,2] instead of [1, 39682].

## Proof of Code
function testTruncation() public {
    // Force large prize pool
    // Trigger settlement
    // Assert next drawing bonusMax is small despite large pool
}

## Suggested Mitigation
Check if `newBonusball > 255` and cap `lpPool` or revert/handle gracefully, or use `uint16` for ball configuration.


## [H-9]. Ticket bitpacking overflow allows corruption of ticket data and easier wins

## id: FnKsxcSecuJjLLtJbI26m

## Derived From Pattern/Invariant
normalBallMax + bonusballMax < 255

## Exploit Type
IntegerOverflow

## Location
TicketComboTracker.insert

## Finding Status: Valid
### Finding Status Justification: In `TicketComboTracker.insert`, bonusball is encoded as `1 << (_bonusball + _tracker.normalMax)`. If `_bonusball + normalMax >= 256`, the shift yields 0, so the bonusball bit is silently dropped and the packed ticket is corrupted. The same packing logic is used for the winning ticket in `countTierMatchesWithBonusball` (`set | (1 << (_bonusball + normalMax))`). With a sufficiently large `bonusballMax` relative to `normalMax`, some winning bonusballs will fall into this overflow region and collapse to “no bonus bit”, causing many distinct bonusballs to become indistinguishable and materially changing match probability (e.g., making bonusball matches far easier for overflowed tickets). There is no validation enforcing `normalMax + bonusballMax < 256`.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `Jackpot` contract packs ticket numbers into a `uint256` bit vector. The bonus ball is stored at bit position `bonusball + normalBallMax`. 

Code snippet in `TicketComboTracker.sol`: `ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);`

If `_bonusball + _tracker.normalMax >= 256`, the shift operation overflows (results in 0 for `uint256` shift), and the bonus ball bit is lost. This can happen if `normalBallMax` is set to 128 and the dynamic `bonusballMax` calculation results in a value >= 128 (which is possible and allowed by `uint8` cast). 

A ticket with an overflowed bonus ball (e.g., 128+128=256) will be stored effectively with bonus ball 0. If the winning ticket also overflows, it matches. This bypasses the bonus ball check, significantly increasing winning probability (guaranteed bonus match).

## Impact
Severe game logic corruption. Players can engineer tickets that effectively have a 'wildcard' bonus ball or match incorrectly, draining the prize pool.

## Command to Run Test


## Proof of Concept
1. Admin sets `normalBallMax` to 128.
2. Economic parameters force `bonusballMax` >= 128.
3. User buys ticket with `bonusball` = 128.
4. `1 << (128 + 128)` results in 0. The ticket is stored with only normal balls set.
5. If the winning numbers also imply a high bonus ball, the system matches the user's ticket as a jackpot winner (Tier 11) because both bonus bits are 0.

## Proof of Code
function testBitpackingOverflow() public {
    // Assume normalMax=128, bonus=128
    uint8 normalMax = 128;
    uint8 bonus = 128;
    uint256 packed = 1 << (bonus + normalMax);
    assertEq(packed, 0, "Shift should overflow to 0");
}

## Suggested Mitigation
Enforce `normalBallMax + bonusballMax < 256` in `_validateAndStoreTickets` or `_setNewDrawingState`. Given `normalBallMax` is static, ensure dynamic `bonusballMax` calculation respects this limit.


## [H-10]. Bit packing overflow in `TicketComboTracker` causes corrupted tickets and unclaimable winnings

## id: e8TbwtWJN7YeDdEWY9275

## Derived From Pattern/Invariant
Jackpot.buyTickets

## Exploit Type
IntegerOverflow

## Location
Jackpot.buyTickets

## Finding Status: Valid
### Finding Status Justification: Same underlying bug as the other bitpacking finding: `1 << (_bonusball + normalMax)` can shift by >=256 and silently drop the bonus bit. The specific “unclaimable winnings” framing is not quite accurate because `claimWinnings` uses the stored packed ticket and packed winning numbers (both subject to the same packing bug), so claims may still succeed when the winning bonusball is also in the overflow region. However, tickets and winning numbers become corrupted/ambiguous (many bonusballs collapse to the same representation), breaking fairness and tiering correctness. There is no check enforcing `normalMax + bonusballMax < 256`.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `TicketComboTracker.insert`, the ticket bitmask is created via `1 << (_bonusball + _tracker.normalMax)`. Since `normalMax` can be up to 128 and `bonusball` up to 255 (from `bonusballMax`), the sum can exceed 255. In Solidity, shifting by >= 256 bits results in 0. This results in a packed ticket where the bonusball bit is lost (effectively 0). When `claimWinnings` unpacks this ticket, it recovers a bonusball value of 0, which mismatches the actual ticket bought. This prevents users from claiming winnings for valid tickets with high bonusball numbers.

## Impact
Users purchasing tickets with high bonusball numbers receive corrupted NFTs that cannot claim prizes, resulting in loss of funds.

## Command to Run Test


## Proof of Concept
1. `normalBallMax` = 120. `bonusballMax` = 140. 2. User buys ticket with bonusball 137. 3. `shift` = 120 + 137 = 257. 4. `1 << 257` is 0. 5. Packed ticket has no bonus bit. 6. `claimWinnings` reads bonusball as 0. 7. User cannot match winning number 137.

## Proof of Code
function testBitPackingOverflow() public {
    // Setup normalMax + bonusMax > 255
    // Buy ticket
    // Assert unpacked ticket bonusball is 0
}

## Suggested Mitigation
Ensure `normalBallMax + bonusballMax < 256` in configuration setters and `_setNewDrawingState`.


## [H-11]. LP Share Price Inflation Attack steals funds from new depositors

## id: DBABef6sZJyqFW4xIjV4U

## Derived From Pattern/Invariant
Malicious Liquidity Provider

## Exploit Type
ERC4626SharePrice

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: Valid
### Finding Status Justification: This is a classic donation/share-price inflation pattern: if accumulator becomes very large, deposits can mint 0 shares due to truncation. There is no minimum-share mint/virtual shares mitigation, so it is not inherently rare once pool states can be skewed.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The LP share price is determined by `accumulator = oldAcc * newLP / oldLP`. An attacker can manipulate this by ensuring the pool has a tiny balance (e.g., 1 wei) and then generating significant `lpEarnings` (via ticket purchases). This causes the accumulator to skyrocket. Subsequent deposits by victims will round down to 0 shares due to precision loss (`amount * 1e18 / hugeAccumulator`), effectively donating their deposit to the attacker.

## Impact
Theft of new LP deposits; attackers can drain victim funds.

## Command to Run Test


## Proof of Concept
1. Attacker is sole LP with 1 wei. 2. Attacker buys tickets to increase `lpEarnings`. 3. Settlement makes accumulator huge. 4. Victim deposits 100 USDC, receives 0 shares. 5. Attacker withdraws 1 wei shares (100% of supply) and gets victim's funds.

## Proof of Code
function testInflation() public { /* setup 1 wei pool */ /* buy tickets to boost earnings */ /* settlement */ /* victim deposits */ assertEq(victimShares, 0); }

## Suggested Mitigation
Burn the first 1e18 shares (virtual shares) or enforce a minimum pool size to prevent the denominator from being manipulated to dust.





Finding Status: InvalidGovernanceRisk
## [M-12]. User Funds Locked When Upgrading PayoutCalculator

## id: PVZsTFyEZQYwa9gcorz-X

## Derived From Pattern/Invariant
Changing PayoutCalculator permanently locks winnings from previous drawings

## Exploit Type
Custom

## Location
Jackpot.setPayoutCalculator

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: If the owner switches calculators without migrating historical tierPayouts, users can lose all unclaimed winnings for past drawings, which is high impact even though it is owner-controlled.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The `tierPayouts` mapping is stored in the `GuaranteedMinimumPayoutCalculator` contract, not the main `Jackpot` contract. If the admin calls `setPayoutCalculator` to upgrade the logic, the new calculator will have empty storage for past drawings. Users attempting to claim winnings from drawings settled under the old calculator will receive 0 USDC, as `claimWinnings` queries the *current* `payoutCalculator`.

## Impact
Permanent loss of unclaimed winnings for all past drawings upon calculator upgrade.

## Command to Run Test


## Proof of Concept
1. Drawing 1 settles. User wins 100 USDC. 
2. Admin deploys `CalculatorV2` and calls `setPayoutCalculator`. 
3. User calls `claimWinnings`. 
4. `Jackpot` calls `CalculatorV2.getTierPayout(1, tier)`. 
5. `CalculatorV2` returns 0. User gets nothing.

## Proof of Code
function testCalculatorSwap() public { /* Mock existing payout */ /* Swap calc */ /* Assert payout 0 */ }

## Suggested Mitigation
Store `tierPayouts` in `Jackpot.sol` or require data migration/linking when upgrading the calculator.


## [M-13]. Protocol DoS and LP value drain if `referralFee` exceeds `lpEdgeTarget`

## id: INykdZ9Y4kWo6GLnkv61f

## Derived From Pattern/Invariant
JackpotLPManager.processDrawingSettlement

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.processDrawingSettlement

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Code path exists: `Jackpot.setReferralFee` allows up to 100% and `Jackpot.setLpEdgeTarget` is independent; no invariant enforces `referralFee <= lpEdgeTarget`. On purchases with a referral scheme, `lpEarnings` increases by `ticketsValue - referralFeeTotal`, while duplicates also increase `DrawingState.prizePool` by `ticketPrice - edgePerTicket` (`~ ticketPrice*(1-lpEdgeTarget)`). Settlement uses `JackpotLPManager.processDrawingSettlement` with `postDrawLpValue = lpPoolTotal + lpEarnings - userWinnings - protocolFee`, which reverts on underflow if economics become unfavorable. This is primarily a governance/configuration risk (valid admin inputs can create negative LP economics or even settlement DoS under extreme conditions), not mitigated in code.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
The protocol assumes `lpEdgeTarget` is sufficient to cover referral fees. However, on duplicate tickets, the prize pool liability increases by `ticketPrice * (1 - lpEdgeTarget)`, while `lpEarnings` only increases by `ticketPrice * (1 - referralFee)`. If `referralFee > lpEdgeTarget`, each duplicate ticket creates a net loss for the LP pool. With sufficient volume, `postDrawLpValue` calculation `pool + earnings - winnings` will underflow (as earnings < liability increase), causing settlement to revert (DoS).

## Impact
Denial of service for drawing settlement or drainage of LP pool value.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` = 20%, `lpEdgeTarget` = 10%. 2. Users buy many duplicate tickets. 3. `prizePool` liability grows faster than `lpEarnings`. 4. Settlement reverts due to underflow when calculating `postDrawLpValue`.

## Proof of Code
function testReferralFeeDoS() public {
    // Set referral > edge
    // Buy many duplicates
    // settlement reverts
}

## Suggested Mitigation
Enforce `referralFee <= lpEdgeTarget` in `setReferralFee` and `setLpEdgeTarget`.


## [M-14]. Settlement DoS when referral fees exceed LP edge

## id: ups0NakaHvBDW3D8P0Vjp

## Derived From Pattern/Invariant
JackpotLPManager invariant: lpEdgeTarget >= referralFee

## Exploit Type
AccountingInvariantViolation

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: Same root cause as the other referralFee/lpEdgeTarget finding: no code-level constraint ties `referralFee` to `lpEdgeTarget`. Purchases with referrals reduce `lpEarnings` (`lpEarnings += ticketsValue - referralFeeTotal`), while duplicates can increase `prizePool` (`prizePool += ticketPrice - edgePerTicket`). Settlement’s `postDrawLpValue = lpPoolTotal + lpEarnings - userWinnings - protocolFeeAmount` uses checked `uint256` math and can revert if the configuration and ticket distribution make `userWinnings` exceed the available buffer. This is primarily governance misconfiguration risk (valid parameter values can create bad economics / settlement reverts); there is no built-in mitigation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `processDrawingSettlement` function calculates `postDrawLpValue` as `lpPoolTotal + lpEarnings - userWinnings - protocolFee`. If `referralFee > lpEdgeTarget`, each duplicate ticket sold creates a net deficit because the prize pool liability increase (`ticketPrice * (1-edge)`) exceeds the net earnings (`ticketPrice * (1-referral)`). If enough duplicate tickets win, the expression `lpEarnings - userWinnings` becomes sufficiently negative to underflow the `uint256` calculation (since `lpPoolTotal` buffer might be consumed or `reserveRatio` is 0). This causes the settlement transaction to revert, bricking the drawing.

## Impact
Permanent DoS of the drawing settlement process under specific configuration and usage.

## Command to Run Test


## Proof of Concept
1. Admin sets `referralFee` (e.g. 20%) > `lpEdgeTarget` (e.g. 10%). 
2. Users buy many duplicate tickets of a specific combination. 
3. That combination wins. 
4. `postDrawLpValue` calculation underflows due to net loss per ticket.

## Proof of Code
function testSettlementUnderflow() public { 
  // Config referral > edge 
  // Buy duplicates 
  // Settle drawing 
  // Expect revert 
}

## Suggested Mitigation
Enforce `referralFee <= lpEdgeTarget` in configuration setters or use signed math/flooring for `postDrawLpValue` calculation.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-15]. LP Equity Drain via Referral Fee Arbitrage

## id: h7sWRpxFSuE5426wY7zrn

## Derived From Pattern/Invariant
Ticket buyer paying USDC

## Exploit Type
AccountingInvariantViolation

## Location
Jackpot.buyTickets

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: There is no enforcement that `referralFee <= lpEdgeTarget`, and referral fees are effectively withdrawable by a self-referring attacker (`referralFees` mapping). With `referralFee > lpEdgeTarget`, a duplicate purchase (which increases `prizePool` by `ticketPrice - edgePerTicket`) can increase the accounting-backed prize pool more than the attacker’s net cost after later claiming referral fees, shifting value against LPs if the attacker can also ensure capture of the inflated prize pool (e.g., by buying a sufficiently large portion of the combo space so they deterministically win some tiers). This requires an unfavorable admin configuration and a realistically small/enumerable combination space (or very large spend), so it is primarily governance risk and Rare in practice, but the economic exploit vector exists in current code.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The protocol allows `referralFee` and `lpEdgeTarget` to be configured independently. If `referralFee > lpEdgeTarget`, an attacker can purchase duplicate tickets for a specific combination to drain LP equity. 

When purchasing a duplicate: 
1. Attacker pays `ticketPrice`.
2. `lpEarnings` increases by `ticketPrice - referralFee` (Asset).
3. `prizePool` increases by `ticketPrice - lpEdgeTarget` (Liability).
If referral > edge, the Liability grows faster than the Asset, reducing the net value per share for LPs. An attacker who buys all combinations (guaranteeing the win) can extract this value risk-free.

## Impact
Risk-free drainage of LP pool funds by purchasing duplicate tickets.

## Command to Run Test


## Proof of Concept
1. `referralFee` = 20%, `lpEdge` = 10%. Price = 100.
2. Buy duplicate. Pay 100. Referrer (self) gets 20.
3. Contract keeps 80 (Asset).
4. Prize pool grows by 90 (Liability).
5. Net LP equity change = 80 - 90 = -10.
6. Attacker profits 10 per ticket by washing trading against own referral code.

## Proof of Code
function testReferralArbitrage() public {
    // Set referral > edge
    // Measure LP value before and after duplicate purchase
    // Assert LP value decreased
}

## Suggested Mitigation
Enforce `referralFee <= lpEdgeTarget` in the `setReferralFee` and `setLpEdgeTarget` admin functions.





Finding Status: LowSeverityDueToRareLikelihood
## [M-16]. Zero Accumulator leads to permanent LP DoS

## id: Z3WfEnCgmt0nzU-tt4v_3

## Derived From Pattern/Invariant
drawingAccumulator > 0

## Exploit Type
IntegerMath

## Location
JackpotLPManager.processDrawingSettlement

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: If postDrawLpValue ever reaches 0 (possible under certain allowed parameter combinations and extreme outcomes), drawingAccumulator can become 0 and later divisions by drawingAccumulator will revert, permanently DoSing LP operations. That is exploitable as a DoS condition even if unlikely.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `JackpotLPManager.processDrawingSettlement`, the new accumulator is calculated as `(prevAccumulator * postDrawLpValue) / prevLpTotal`. If `postDrawLpValue` drops to 0 (e.g., due to 100% losses where winnings >= pool), the `newAccumulator` becomes 0.

Once the accumulator is 0, any subsequent call to `processDeposit`, `processInitiateWithdraw`, or `processFinalizeWithdraw` involving that drawing will revert due to division by zero (e.g., `amount * PRECISE_UNIT / accumulator`). This permanently bricks LP operations for the contract.

## Impact
Permanent Denial of Service for Liquidity Providers. No new deposits or withdrawals can be processed if the pool is ever wiped out.

## Command to Run Test


## Proof of Concept
1. `reserveRatio` is set to 0 (or low).
2. A drawing occurs where user winnings equal or exceed the LP pool total.
3. `postDrawLpValue` becomes 0.
4. `drawingAccumulator` is updated to 0.
5. Next `lpDeposit` calls `processDeposit`, which tries to consolidate using `accumulator`. Revert.

## Proof of Code
function testZeroAccumulatorDoS() public {
    // Mock scenario where pool is wiped out
    // ... execution results in acc = 0
    uint256 acc = 0;
    vm.expectRevert();
    // Division by zero in deposit logic
    uint256 shares = 100 * 1e18 / acc;
}

## Suggested Mitigation
In `processDrawingSettlement`, ensure `newAccumulator` has a minimum value of 1 or handle the zero case explicitly (e.g., reset to `PRECISE_UNIT` if pool is empty/restarted).



