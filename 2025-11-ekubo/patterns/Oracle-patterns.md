## Verified Patterns Found: 12

## Verified Patterns Found in following Categories:

- FeeOnTransferAssumption
- PrecisionDriftAccumulation
- StaleOracleAcceptance
- StorageCollisionOrSelectorClash
- TWAPWindowPinningOrLowLiquidity



## Summary of Patterns

Oracle enforces zero fees enabling costless TWAP manipulation

Stale Oracle Fallback to Spot Price

Revenue Buybacks Missing Slippage Protection

Default Oracle capacity of 1 overwrites history preventing TWAP usage

Default Oracle Capacity Prevents Historical TWAP

Oracle Manipulation via Zero-Fee Empty Pool Pinning

Fee-on-transfer tokens cause insolvency in Incentives contract

Default Oracle Capacity of 1 Disables TWAP History

Incentives Insolvency via Fee-on-Transfer Tokens

Oracle Default Cardinality Allows TWAP Manipulation

TWAMM Rewards Precision Loss Locks Tokens

Storage Collision via Unsafe Key Packing in Oracle

## Patterns



 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Oracle enforces zero fees enabling costless TWAP manipulation
 ### Description/Code Snippet
The `Oracle` extension explicitly enforces `key.config.fee() == 0` in `beforeInitializePool`. While this might be intended for specific use cases, it removes the trading fee cost that typically protects TWAP oracles from manipulation. An attacker can manipulate the pool's price (tick) and hold it ('pinning') for a period of time to skew the accumulator values (`tickCumulative`), incurring only gas costs. Any downstream system relying on this oracle for price data would be consuming an easily manipulatable signal.
 ### Static Signals
if (key.config.fee() != 0) revert FeeMustBeZero()
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: Oracle.extrapolateSnapshotInternal

 ### Title
Stale Oracle Fallback to Spot Price
 ### Description/Code Snippet
The `extrapolateSnapshot` function in `Oracle.sol` extrapolates cumulative values from the last recorded snapshot to the requested time using the *current* pool state (`CORE.poolState(poolId)`). If the pool has not been updated recently (stale), the time interval between the last snapshot and the requested time is large. Consequently, the calculated TWAP is derived almost entirely from the current spot price read from Core. This allows an attacker to manipulate the spot price (e.g., via a flash loan or swap) and immediately query the oracle, which returns a 'TWAP' that reflects the manipulated spot price, bypassing the security guarantees of a time-weighted average.
 ### Static Signals
extrapolates using current state, no check for snapshot freshness, logicalIndex == c.count() - 1 path uses CORE.poolState
 ### Assets at Risk
treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Revenue Buybacks Missing Slippage Protection
 ### Description/Code Snippet
The `RevenueBuybacks.roll` function creates TWAMM orders to sell the contract's entire revenue balance for `BUY_TOKEN` without any slippage protection or liquidity validation. It calls `ORDERS.increaseSellAmount` with `maxSaleRate` set to `type(uint112).max`. If the pool for the revenue token has low liquidity, or if a large amount of revenue has accumulated, the resulting high sale rate (amount / duration) will cause significant price impact/slippage during TWAMM execution. This allows arbitrageurs to extract value from the protocol's revenue at the expense of the treasury.
 ### Static Signals
increaseSellAmount called with type(uint112).max, no min output amount parameter, sells full balance in one order
 ### Assets at Risk
treasury, rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Default Oracle capacity of 1 overwrites history preventing TWAP usage
 ### Description/Code Snippet
The `beforeInitializePool` function initializes the oracle ring buffer capacity to `max(1, c.capacity())`. Since a new pool's capacity is 0, it defaults to 1. A capacity of 1 means the oracle stores only the single most recent snapshot. Every new swap or position update immediately overwrites this snapshot. Consequently, `findPreviousSnapshot` will revert with `NoPreviousSnapshotExists` for any time `T` prior to the last interaction, as the historical snapshot required to interpolate values at `T` has been overwritten. This renders the oracle unusable for Time-Weighted Average Price (TWAP) queries by default, creating a Denial of Service or unsafe condition for downstream integrations expecting standard oracle behavior.
 ### Static Signals
capacity defaults to 1, observationCardinality/min not enforced
 ### Assets at Risk
Integrations relying on TWAP data
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Default Oracle Capacity Prevents Historical TWAP
 ### Description/Code Snippet
The `Oracle` extension initializes the snapshot buffer `Counts` with a default capacity of `max(1, c.capacity())` in `beforeInitializePool`. Since the storage is initially zero, new pools default to a capacity of 1. With a capacity of 1, the buffer stores only the single most recent snapshot, as `maybeInsertSnapshot` overwrites the oldest snapshot (index 0) on every update. This effectively pins the oracle history to the current block timestamp (or the timestamp of the last interaction). Any attempt to calculate a Time-Weighted Average Price (TWAP) over a window larger than the time elapsed since the last transaction will fail with `NoPreviousSnapshotExists` or rely on a single data point, making the oracle useless for historical TWAP unless `expandCapacity` is explicitly called.
 ### Static Signals
observationCardinality/min not enforced, capacity defaults to 1
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.sol.beforeInitializePool

 ### Title
Oracle Manipulation via Zero-Fee Empty Pool Pinning
 ### Description/Code Snippet
The `Oracle` extension enforces that pools must have a 0% fee (`key.config.fee() != 0` revert in `beforeInitializePool`). In a zero-fee pool, an attacker can manipulate the price (tick) to an arbitrary value with zero fee cost (only gas). Furthermore, the `maybeInsertSnapshot` function continues to accumulate time-weighted values based on the last recorded tick even if the pool's liquidity is subsequently removed or negligible. An attacker can set an extreme price, withdraw liquidity, and let the Oracle accumulate this manipulated price over a window, skewing the TWAP at minimal cost.
 ### Static Signals
fee() != 0 revert, maybeInsertSnapshot accumulates without liquidity check
 ### Assets at Risk
Protocol pricing integrity, Dependent contracts using this Oracle
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Fee-on-transfer tokens cause insolvency in Incentives contract
 ### Description/Code Snippet
The `Incentives.fund` function updates the `funded` amount based on the input `minimum` parameter rather than the actual amount of tokens received. If a Fee-On-Transfer (FoT) token is used, the contract receives less than the recorded `funded` amount. This leads to insolvency for that specific drop, as the contract will track more tokens than it actually holds. Furthermore, since the `Incentives` contract holds tokens for all drops in a shared balance, this discrepancy allows claimers of the FoT-token drop to withdraw tokens belonging to other drops of the same token (cross-drop theft).
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check
 ### Assets at Risk
rewards, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Default Oracle Capacity of 1 Disables TWAP History
 ### Description/Code Snippet
The `beforeInitializePool` function initializes the snapshot ring buffer with a default capacity of 1 (`_count: 1`). This means the Oracle overwrites the previous observation on every update, effectively storing only the latest state. Attempts to query a historical TWAP window via `findPreviousSnapshot` or `extrapolateSnapshot` will revert with `NoPreviousSnapshotExists` for any window longer than the last update interval. This creates a Denial of Service for TWAP integrations or forces improper fallbacks to spot price unless `expandCapacity` is explicitly called by a third party.
 ### Static Signals
observationCardinality/min not enforced, capacity initialized to 1
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Incentives Insolvency via Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `fund` function calculates the required `fundedAmount` to meet a minimum target and transfers exactly that amount using `safeTransferFrom`. It updates the internal `funded` accounting by the full amount. If the token charges a fee on transfer, the contract receives less than the accounted amount, causing insolvency where the last valid claimers (via `claim`) will fail due to insufficient token balance.
 ### Static Signals
uses input amount instead of post-transfer delta, accounting based on transfer parameter, no balanceBefore/After check
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Oracle Default Cardinality Allows TWAP Manipulation
 ### Description/Code Snippet
The `Oracle` extension initializes pools with a default observation capacity of 1 (in `beforeInitializePool`). It relies on users to manually call `expandCapacity`. With capacity 1, the Oracle overwrites the single stored snapshot on every update, preventing the calculation of any historical TWAP (window length 0) or allowing easy manipulation of the 'current' price as the 'average' price. This renders the Oracle unsafe by default.
 ### Static Signals
observationCardinality/min not enforced, buffer overwrites immediately, default capacity too small
 ### Assets at Risk
Protocol
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM Rewards Precision Loss Locks Tokens
 ### Description/Code Snippet
In `_executeVirtualOrdersFromWithinLock`, the calculation of `rewardRates` performs integer division of the swap delta by the sale rate (`FixedPointMathLib.rawDiv`). The remainder of this division (dust) is not accounted for in the reward rate but is still retained in the `TWAMM` contract's `savedBalances` within Core. As there is no mechanism to sweep these accumulated dust amounts, they are permanently locked.
 ### Static Signals
consistent floor toward sender/receiver, division before distribution, no sweep mechanism for dust
 ### Assets at Risk
User yields
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: Oracle.maybeInsertSnapshot

 ### Title
Storage Collision via Unsafe Key Packing in Oracle
 ### Description/Code Snippet
The `Oracle` contract maps token addresses directly to storage slots for `Counts` structs, and packs snapshot entries at `(token << 32) | index`. This packing strategy creates a storage collision if a token address `B` has its top 32 bits as zero (i.e., `B < 2^128`). Specifically, the snapshot entry for token `B` at index `i` will be stored at `(B << 32) | i`. If another token `A` exists such that `A = (B << 32) | i`, the `Counts` struct for token `A` (stored at slot `A`) will be overwritten by the snapshot data of token `B`. An attacker can mine a token address `B` with 32 leading zeros (computational cost ~2^32 hashes) and deploy a corresponding token `A` to deliberately corrupt the oracle state for `A`, causing denial of service or incorrect pricing data.
 ### Static Signals
sstore(or(shl(32, token), index), snapshot), c := sload(token), manual assembly slots without namespace
 ### Assets at Risk
Oracle data integrity, Protocol pricing reliability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

