## Verified Patterns Found: 31

## Verified Patterns Found in following Categories:

- StaleOracleAcceptance
- FlashLoanEconomicManipulation
- OracleUsingDEXorTWAP
- PricePrecisionOrRoundingError
- ERC20DecimalsMismatch
- GriefableCallbacks
- StandardViolation
- UnboundedLoops
- AccessControlOrAuthByPass
- ReserveOrPriceDesync
- UnsafeRecipient
- AccountingInvariantViolation
- FeeOnTransferAssumption
- TWAPWindowPinningOrLowLiquidity
- SlippageMissingOrInsufficient
- AllowanceRace



## Summary of Patterns

Orders sale rate truncation leads to incorrect order execution

Fee-on-Transfer Token Incompatibility in Orders and RevenueBuybacks

RevenueBuybacks approves wrong spender causing denial of service

Allowance Race Condition in TokenWrapper

Missing Slippage Protection in Positions Withdraw

Denial of Service in MEVCapture Swaps

TokenWrapper Reverts on Tokens Missing Optional Decimals

Oracle History Erasure via Ring Buffer Rotation

Default Oracle capacity of 1 causes history erasure and TWAP DoS

Lack of Zero Address Validation in Withdrawals

Unsafe Storage Layout in Oracle Extension Causes State Collision

Indefinite extrapolation of stale oracle snapshots

DoS in PriceFetcher due to Zero Division with High Liquidity

Oracle relies on isolated zero-fee pools likely to have low liquidity

Misleading derived liquidity for concentrated pools

Indefinite Extrapolation of Stale Prices

Flash Loan Economic Manipulation via PriceFetcher

Liquidity Overflow in PriceFetcher.getAveragesOverPeriod

Misleading Non-Zero Liquidity Reporting for Empty Pools

Unbounded Loops in Historical Data Fetching

Silent failure on insufficient history returns valid-looking Tick 0 (Price 1.0)

Sale Rate Truncation in Orders.increaseSellAmount

MEVCapture Extension Denial of Service via Unconditional Revert

Missing minimum TWAP window enforcement

TokenWrapper tokens are non-transferable due to missing balance update

Unbounded Loop in Historical Data Fetching

No Minimum TWAP Window Enforcement

Misleading Cross-Pair Liquidity Calculation via Geometric Mean

Recursive Derived Price Feed Dependency

Short TWAP periods enable multi-block spot price manipulation via spot-tick projection

Unbounded Loop in TWAMM Virtual Order Execution

## Patterns



 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Orders.increaseSellAmount

 ### Title
Orders sale rate truncation leads to incorrect order execution
 ### Description/Code Snippet
In `Orders.sol`, the `increaseSellAmount` function calculates the `saleRate` using `computeSaleRate` and explicitly casts the result to `uint112`. There is no check to ensure the result fits within `uint112`. If a user provides a large `amount` and a short `duration`, the sale rate can exceed `uint112`, causing silent truncation. This results in the order being created with a significantly lower sale rate than intended, failing to sell the user's tokens as expected.
 ### Static Signals
uint112(...) cast without check, saleRate derived from user input
 ### Assets at Risk
User funds in Orders
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Orders.handleLockData

 ### Title
Fee-on-Transfer Token Incompatibility in Orders and RevenueBuybacks
 ### Description/Code Snippet
The `Orders` contract (and by extension `RevenueBuybacks`) assumes that the amount of tokens transferred from the user is exactly the amount received by the protocol. In `Orders.increaseSellAmount`, `CORE.updateSaleRate` is called with the specified `amount`, which updates the TWAMM sale rate and effectively registers a debt of `amount` in the `FlashAccountant`. The contract then uses `FlashAccountantLib.payFrom` to pull `amount` from the user. If the token has a fee-on-transfer, the `FlashAccountant` will detect a balance increase smaller than `amount` (i.e., `amount - fee`). Since the debt registered is `amount` but the credit received is `amount - fee`, the transaction will revert with `DebtsNotZeroed`, making the protocol incompatible with fee-on-transfer tokens for order creation.
 ### Static Signals
ACCOUNTANT.payFrom, uses input amount instead of post-transfer delta
 ### Assets at Risk
None (DoS for FoT tokens)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: RevenueBuybacks.approveMax

 ### Title
RevenueBuybacks approves wrong spender causing denial of service
 ### Description/Code Snippet
In `RevenueBuybacks.sol`, the `approveMax` function approves the `ORDERS` contract to spend the revenue token. However, when `roll` is called, `ORDERS.increaseSellAmount` triggers a lock where `Core` (acting as the Accountant) attempts to pull tokens directly from `RevenueBuybacks` using `ACCOUNTANT.payFrom`. Since `RevenueBuybacks` has approved `ORDERS` but not `Core`, the transfer will fail for ERC20 tokens, making the `roll` function unusable for buybacks.
 ### Static Signals
approve(ORDERS), indirect pull by Core
 ### Assets at Risk
Protocol revenue (cannot be used)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AllowanceRace

 ### Relevant Function/Location: TokenWrapper.approve

 ### Title
Allowance Race Condition in TokenWrapper
 ### Description/Code Snippet
The `TokenWrapper` contract implements standard ERC20 `approve` logic without mitigation for the known allowance race condition. A spender can front-run a call to `approve` (changing allowance from non-zero to non-zero) to spend both the old and new amounts.
 ### Static Signals
changes allowance from X to Y without zeroing
 ### Assets at Risk
User wrapped tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing Slippage Protection in Positions Withdraw
 ### Description/Code Snippet
The `withdraw` and `collectFees` functions in `Positions.sol` burn liquidity and return the resulting tokens to the user based on the current pool price (`sqrtRatio`). These functions do not accept minimum output amount parameters (`amount0Min`, `amount1Min`). As a result, users cannot enforce slippage protection on-chain within the call. If the pool price changes unfavorably (e.g., via a sandwich attack or natural volatility) between the transaction submission and execution, the user may receive significantly less of the valuable token than expected.
 ### Static Signals
no amountOutMin parameter, payout calculated at execution time without minimum bound
 ### Assets at Risk
User liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
Denial of Service in MEVCapture Swaps
 ### Description/Code Snippet
The `MEVCapture` extension implements `beforeSwap` by reverting with `SwapMustHappenThroughForward()`. However, when `MEVCaptureRouter` forwards a swap call to `Core` (via `MEVCapture` as the locker), `Core.swap` internally calls `beforeSwap` on the extension (which is `MEVCapture`). This triggers the revert, making it impossible to execute swaps on pools configured with the `MEVCapture` extension, effectively locking functionality.
 ### Static Signals
revert SwapMustHappenThroughForward(), Core.swap calls extension.beforeSwap
 ### Assets at Risk
MEV Capture Pool Liquidity (Unusable)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.decimals

 ### Title
TokenWrapper Reverts on Tokens Missing Optional Decimals
 ### Description/Code Snippet
The `TokenWrapper` contract implements the `decimals()` function by directly calling `UNDERLYING_TOKEN.decimals()`. While `decimals()` is part of the ERC20 standard's optional metadata extensions, it is not mandatory. Some tokens do not implement it. Creating a `TokenWrapper` for such a token will result in a broken contract where `decimals()` (and potentially other integrations relying on it) always reverts, violating the robust integration expectations of the protocol.
 ### Static Signals
UNDERLYING_TOKEN.decimals(), standard-required function missing or has wrong signature
 ### Assets at Risk
User funds (if wrapped and cannot be handled by UIs)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.sol.maybeInsertSnapshot

 ### Title
Oracle History Erasure via Ring Buffer Rotation
 ### Description/Code Snippet
The `Oracle` extension uses a ring buffer to store snapshots, with a default capacity of 1 (unless expanded via `expandCapacity`). The `maybeInsertSnapshot` function overwrites old snapshots in the circular buffer (`index = (index + 1) % count`). An attacker can cheaply spam small swaps or position updates to rotate the `index` and overwrite historical snapshots. This prevents `PriceFetcher` from retrieving historical averages (causing reverts) or forces it to use a very recent, manipulatable window if `getAvailableHistoricalPeriodAverages` is used.
 ### Static Signals
index = (index + 1) % count, capacity defaults to 1, Overwrite of historical data
 ### Assets at Risk
Protocols relying on TWAP history
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.maybeInsertSnapshot

 ### Title
Default Oracle capacity of 1 causes history erasure and TWAP DoS
 ### Description/Code Snippet
The `Oracle` contract initializes snapshot capacity to 1 by default (`beforeInitializePool`). The `maybeInsertSnapshot` function overwrites the oldest snapshot in a circular buffer when the capacity is reached. With a capacity of 1, any new action (swap/liquidity update) overwrites the sole historical snapshot with the current block's timestamp. This causes `searchRangeForPrevious` to revert with `NoPreviousSnapshotExists` for any query requesting a time prior to the current block's snapshot. Effectively, active pools lose all history immediately, making `PriceFetcher` revert on historical TWAP queries and effectively functioning only as a spot oracle in active blocks.
 ### Static Signals
c.capacity() initialized to 1, overwrites snapshot in circular buffer, no min observation / heartbeat
 ### Assets at Risk
Integrations relying on historical TWAP
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: FlashAccountant.withdraw

 ### Title
Lack of Zero Address Validation in Withdrawals
 ### Description/Code Snippet
The `withdraw` function in `FlashAccountant.sol` (and by extension `Positions.withdraw` and `Router.swap`) allows withdrawing tokens to `recipient` without checking if `recipient` is `address(0)`. If a user or UI accidentally passes the zero address, the tokens are permanently lost (burned).
 ### Static Signals
no zero-address guard
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Oracle.maybeInsertSnapshot

 ### Title
Unsafe Storage Layout in Oracle Extension Causes State Collision
 ### Description/Code Snippet
The Oracle extension uses a custom storage layout where `Counts` are stored at slot `uint256(token)` and `Snapshots` are stored at slot `(uint256(token) << 32) | index`. This packing allows for storage collisions between different tokens. For example, if `tokenA` has an address `A` and `tokenB` has an address `B` such that `B = (A << 32) | index`, writing a snapshot for `tokenA` will overwrite the `Counts` struct for `tokenB`. Since addresses are 160 bits, a token with an address less than `2^128` (having leading zeros) can collide with the snapshot storage of another token. This corruption allows an attacker to manipulate oracle data consumed by `PriceFetcher`.
 ### Static Signals
sstore(token, c), sstore(or(shl(32, token), index), snapshot)
 ### Assets at Risk
Oracle Data Integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: Oracle.sol.extrapolateSnapshotInternal

 ### Title
Indefinite extrapolation of stale oracle snapshots
 ### Description/Code Snippet
The `extrapolateSnapshotInternal` function extrapolates the last recorded snapshot's state (tick and liquidity) to the requested `atTime` using the current pool state. There is no check to ensure the time difference between `atTime` and the last snapshot is within a safe staleness threshold. If a pool becomes inactive (no swaps or position updates), the oracle will confidently report the last known price as the current price indefinitely, ignoring market movements on other venues. Consumers of this oracle may execute transactions based on significantly stale prices.
 ### Static Signals
extrapolateSnapshot, snapshot.timestamp(), no heartbeat check, no max age check
 ### Assets at Risk
Protocol funds relying on oracle pricing, User collateral in lending markets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: PriceFetcher.getAveragesOverPeriod

 ### Title
DoS in PriceFetcher due to Zero Division with High Liquidity
 ### Description/Code Snippet
In `PriceFetcher.getAveragesOverPeriod` (and other averaging functions), the code divides by `secondsPerLiquidityCumulativeEnd - secondsPerLiquidityCumulativeStart`. If a pool has extremely high liquidity (e.g. > 10^38), the `secondsPerLiquidity` accumulator in `Oracle` (which adds `time << 128 / liquidity`) may not increment over short durations due to precision loss, resulting in a zero difference. This causes `PriceFetcher` to revert with a division by zero, creating a Denial of Service for any integrations relying on it for such assets.
 ### Static Signals
(secondsPerLiquidityCumulativeEnd - secondsPerLiquidityCumulativeStart)
 ### Assets at Risk
Availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Oracle relies on isolated zero-fee pools likely to have low liquidity
 ### Description/Code Snippet
The `Oracle` extension enforces that it only tracks pools with `fee` set to 0 (see `Oracle.beforeInitializePool`). Consequently, `PriceFetcher` hardcodes the pool key to use `fee=0` when querying the oracle. In the Ekubo model, liquidity providers typically deposit into fee-generating pools (e.g., 5bps). Since the zero-fee pool generates no swap revenue, rational LPs will not provide liquidity there unless externally incentivized. This creates a high risk that the 'Oracle Pool' will have thin or zero liquidity, allowing cost-effective manipulation of the price reported by `PriceFetcher` without affecting the main trading pools.
 ### Static Signals
if (key.config.fee() != 0) revert FeeMustBeZero(), uses single low-liquidity pair without liquidity floor
 ### Assets at Risk
Protocol funds relying on PriceFetcher
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: PriceFetcher.sol.getAveragesOverPeriod

 ### Title
Misleading derived liquidity for concentrated pools
 ### Description/Code Snippet
In `getAveragesOverPeriod`, the derived liquidity for cross-pair tokens (Token A -> Token B) is calculated by projecting the harmonic mean of active liquidity (`base.liquidity`) down to `MIN_SQRT_RATIO` using `amount1Delta`. This calculation assumes that the active liquidity extends across the full price range (Uniswap V2 invariant). For concentrated liquidity pools (Uniswap V3/Ekubo style), where liquidity is often localized to a narrow tick range, this calculation drastically overestimates the available market depth and virtual reserves, potentially misleading downstream risk engines into overvaluing the pool's liquidity resilience.
 ### Static Signals
amount1Delta(..., MIN_SQRT_RATIO, ...), FixedPointMathLib.sqrt(uint256(amountBase) * uint256(amountQuote))
 ### Assets at Risk
Risk management modules relying on liquidity depth
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: Oracle.extrapolateSnapshotInternal

 ### Title
Indefinite Extrapolation of Stale Prices
 ### Description/Code Snippet
The `Oracle` extension's `extrapolateSnapshot` function (used by `PriceFetcher`) extrapolates values based on the last recorded snapshot and the current pool state. If a pool has low activity and hasn't traded for a significant period, the 'current' state returned by `PriceFetcher` will reflect the price of the last trade, which may be arbitrarily old (stale). There is no mechanism to flag or revert if the data is too old (e.g., a heartbeat check), potentially feeding stale prices to downstream consumers.
 ### Static Signals
no heartbeat check, no stale timestamp check
 ### Assets at Risk
User funds in integrating protocols
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: PriceFetcher.getOracleTokenAverages

 ### Title
Flash Loan Economic Manipulation via PriceFetcher
 ### Description/Code Snippet
The `PriceFetcher.getOracleTokenAverages` function hardcodes `endTime` to `block.timestamp`, and `getAveragesOverPeriod` allows it. When `endTime` is `block.timestamp`, `Oracle.extrapolateSnapshot` uses the current pool state (via `Core.poolState`) to extrapolate the price. The current pool state reflects spot price changes from swaps in the same transaction. If the `observationPeriod` is short, the returned TWAP is heavily weighted by the manipulated spot price, allowing attackers to manipulate the value via flash loans.
 ### Static Signals
endTime = uint64(block.timestamp), extrapolateSnapshot(..., endTime), CORE.poolState(poolId)
 ### Assets at Risk
Protocol funds relying on PriceFetcher
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: PriceFetcher.sol.getAveragesOverPeriod

 ### Title
Liquidity Overflow in PriceFetcher.getAveragesOverPeriod
 ### Description/Code Snippet
The `PriceFetcher` contract calculates harmonic mean liquidity using `(time << 128) / spcDelta`. The result is cast to `uint128`. If the pool has extremely high liquidity (e.g. Stableswap with max uint128 liquidity), `spcDelta` (seconds per liquidity) becomes very small (approaching `time`). The division result can effectively equal `2^128`, which exceeds `type(uint128).max` (`2^128 - 1`). The explicit `uint128(...)` cast overflows to 0. This causes `PriceFetcher` to report 0 liquidity for the most liquid pools, potentially causing denial of service or pricing errors in downstream integrations.
 ### Static Signals
uint128((uint160(...) << 128) / ...), Explicit cast causing overflow
 ### Assets at Risk
Integrations relying on PriceFetcher liquidity data
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: PriceFetcher.getAveragesOverPeriod

 ### Title
Misleading Non-Zero Liquidity Reporting for Empty Pools
 ### Description/Code Snippet
The `getAveragesOverPeriod` function calculates average liquidity using `(endTime - startTime) / deltaSecondsPerLiquidity`. Due to the underlying Oracle's mechanism of treating zero liquidity as 1 (to prevent division by zero during accumulation), the `PriceFetcher` reports an average liquidity of 1 for periods where the pool was completely empty (Liquidity = 0). This creates a precision error where empty pools appear to have 1 unit of liquidity, potentially misleading downstream components that rely on `liquidity > 0` checks.
 ### Static Signals
consistent floor toward sender/receiver
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: PriceFetcher.getHistoricalPeriodAverages

 ### Title
Unbounded Loops in Historical Data Fetching
 ### Description/Code Snippet
The `getHistoricalPeriodAverages` function allocates memory arrays (`timestamps`, `averages`) and iterates based on the `numIntervals` parameter. This parameter is user-controlled. If called on-chain (e.g., by a volatility settling contract), a large `numIntervals` can cause the transaction to run out of gas (DoS). The function `getOracleTokenAverages` also iterates over an unbounded `baseTokens` array.
 ### Static Signals
new uint256[](numIntervals + 1), for (uint256 i = 0; i < numIntervals; i++), new PeriodAverage[](baseTokens.length)
 ### Assets at Risk
Gas
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: PriceFetcher.getOracleTokenAverages

 ### Title
Silent failure on insufficient history returns valid-looking Tick 0 (Price 1.0)
 ### Description/Code Snippet
In `getOracleTokenAverages`, the function checks if `ORACLE.getMaximumObservationPeriod(token)` is sufficient for the requested `observationPeriod`. If the history is insufficient (e.g., for a newly initialized pool), the function skips the `getAveragesOverPeriod` call and leaves the `results[i]` struct as default zero-initialized memory. A `PeriodAverage` with `tick=0` corresponds to a valid price of `1.0001^0 = 1`. Downstream integrations may misinterpret this missing data as a valid stable price of 1.0, leading to incorrect valuations.
 ### Static Signals
conditional validation skipped via flag/parameter manipulation, missing revert on insufficient data
 ### Assets at Risk
Integrations interpreting uninitialized PeriodAverage structs
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Orders.sol.increaseSellAmount

 ### Title
Sale Rate Truncation in Orders.increaseSellAmount
 ### Description/Code Snippet
In `Orders.increaseSellAmount`, the `saleRate` is computed as `(amount << 32) / duration` using `computeSaleRate`, which returns a `uint256`. This value is explicitly cast to `uint112`: `saleRate = uint112(...)`. If `amount` is large (e.g. high-supply tokens or high decimals) and `duration` is small, the result can exceed `type(uint112).max`. The explicit cast silently truncates the high bits, resulting in a much lower sale rate than intended. This effectively locks the user's funds in a zombie order that executes negligibly, and the `maxSaleRate` check passes because the truncated value is small.
 ### Static Signals
saleRate = uint112(computeSaleRate(...)), Truncation on casting
 ### Assets at Risk
User funds deposited into TWAMM orders
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture Extension Denial of Service via Unconditional Revert
 ### Description/Code Snippet
The `MEVCapture` extension implements the `beforeSwap` hook to enforce that swaps are executed via the `forward` mechanism. However, the implementation of `beforeSwap` unconditionally reverts with `SwapMustHappenThroughForward`. When a user swaps via `MEVCaptureRouter`, the router calls `Core.forward`, which invokes `MEVCapture.handleForwardData`. This function subsequently calls `Core.swap`. inside `Core.swap`, the `beforeSwap` hook is triggered again because `MEVCapture` is the registered extension for the pool. Since the `beforeSwap` hook does not check if the locker is the extension itself (authorized recursion), it reverts again. This creates a permanent Denial of Service for all pools using the `MEVCapture` extension, as no swap can ever succeed.
 ### Static Signals
revert unconditionally in hook, recursive hook invocation
 ### Assets at Risk
Liquidity in MEVCapture pools
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: PriceFetcher.sol.getAveragesOverPeriod

 ### Title
Missing minimum TWAP window enforcement
 ### Description/Code Snippet
The `PriceFetcher` contract's `getAveragesOverPeriod` function calculates averages for a requested time range `[startTime, endTime]` without enforcing a minimum window duration. Callers can request a window of 0 or 1 second (e.g. `endTime = block.timestamp`, `startTime = block.timestamp - 1`). This effectively returns a spot price derived from the latest snapshot or current state, which is susceptible to sandwich attacks and manipulation within a single block. Integrating protocols that do not implement their own minimum window checks will be vulnerable to price manipulation.
 ### Static Signals
endTime <= startTime check only, no MIN_WINDOW constant, arbitrary startTime input
 ### Assets at Risk
Integrator funds, User slippage
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
TokenWrapper tokens are non-transferable due to missing balance update
 ### Description/Code Snippet
In `TokenWrapper.sol`, the `handleForwardData` function processes wrapping operations by updating the `savedBalances` in the Core contract, which correctly updates the total supply. However, it fails to update the local `_balanceOf` mapping for the user. Since the `transfer` and `transferFrom` functions rely on `_balanceOf` to verify token ownership, users who wrap tokens receive a balance of 0 in the `TokenWrapper` ERC20 contract and are unable to transfer or utilize their wrapped tokens.
 ### Static Signals
balanceOf mapping not updated, totalSupply derived from external call
 ### Assets at Risk
User funds wrapped in TokenWrapper
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: PriceFetcher.getAvailableHistoricalPeriodAverages

 ### Title
Unbounded Loop in Historical Data Fetching
 ### Description/Code Snippet
The function `getAvailableHistoricalPeriodAverages` calculates `numIntervals` based on the difference between `endTime` and the earliest snapshot time, divided by `period`. If `earliestObservationTime` is significantly in the past (e.g., initialized years ago) and the user-supplied `period` is small (e.g., 1 second), the calculated `numIntervals` can be extremely large (millions). This leads to a massive memory allocation for `averages` and a loop in `getHistoricalPeriodAverages` that exceeds the block gas limit, causing a Denial of Service for historical queries.
 ### Static Signals
loop bound depends on attacker-controlled value, iteration count grows with contract state, no pagination or batching mechanism
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: PriceFetcher.getAveragesOverPeriod

 ### Title
No Minimum TWAP Window Enforcement
 ### Description/Code Snippet
The `PriceFetcher` contract allows callers to specify arbitrary `startTime` and `endTime` for `getAveragesOverPeriod` and related functions. If a caller (or integrating protocol) specifies a very short window (e.g., 1 second), the returned value effectively mirrors the spot price. This bypasses the security properties of a Time-Weighted Average Price (TWAP) and exposes users to multi-block price manipulation attacks, especially in low-liquidity pools.
 ### Static Signals
endTime - startTime, no minimum validation
 ### Assets at Risk
User funds in integrating protocols
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: PriceFetcher.getAveragesOverPeriod

 ### Title
Misleading Cross-Pair Liquidity Calculation via Geometric Mean
 ### Description/Code Snippet
In `PriceFetcher.getAveragesOverPeriod`, the liquidity for a derived cross-pair (Base/Quote) is calculated as the geometric mean (`sqrt(amountBase * amountQuote)`) of the liquidity depths of the two underlying pairs (Base/Native and Quote/Native). This method significantly overestimates the effective liquidity when one leg of the trade is illiquid. For example, if Base/Native has $1 liquidity and Quote/Native has $1M liquidity, the derived liquidity is ~$1k. This masks the fact that the price is easily manipulatable (cost proportional to $1), leading downstream consumers to trust a fragile price.
 ### Static Signals
FixedPointMathLib.sqrt(uint256(amountBase) * uint256(amountQuote))
 ### Assets at Risk
Protocol Solvency (if dependent on PriceFetcher liquidity score)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: PriceFetcher.getAveragesOverPeriod

 ### Title
Recursive Derived Price Feed Dependency
 ### Description/Code Snippet
In `getAveragesOverPeriod`, if the base/quote tokens are not the native token, the contract recursively calculates the price by deriving it from `Base/Native` and `Quote/Native` pairs. This derived oracle logic introduces a dependency on the liquidity depth and manipulation resistance of *two* intermediate pools. If either the `Base/Native` or `Quote/Native` pool has low liquidity, the derived `Base/Quote` price can be easily manipulated, even if the direct `Base/Quote` pool is liquid.
 ### Static Signals
recursive call, getAveragesOverPeriod(NATIVE_TOKEN_ADDRESS, ...)
 ### Assets at Risk
User funds in integrating protocols
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: PriceFetcher.getAveragesOverPeriod

 ### Title
Short TWAP periods enable multi-block spot price manipulation via spot-tick projection
 ### Description/Code Snippet
The `PriceFetcher` contract allows callers to request time-weighted averages over arbitrary periods via `getAveragesOverPeriod` without enforcing a minimum duration. The underlying `ORACLE.extrapolateSnapshot` function projects the current spot tick (`CORE.poolState(poolId).tick()`) over the time elapsed since the last snapshot to the current block timestamp. While the Oracle prevents intra-transaction manipulation by forcing snapshots before state changes, it does not prevent manipulation persisted across blocks. If a user requests a short averaging period (e.g., comparable to the block interval) in block N+1, the average will be dominated by the spot price established at the end of block N. An attacker can manipulate the spot price in block N, and the `PriceFetcher` in block N+1 will report a 'TWAP' that effectively mirrors the manipulated spot price.
 ### Static Signals
no minimum period check, extrapolateSnapshot uses current spot price, TWAP window < 10–30 min
 ### Assets at Risk
Protocols relying on PriceFetcher for asset valuation
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded Loop in TWAMM Virtual Order Execution
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function in `TWAMM.sol` iterates through all initialized time intervals between the last execution time and `block.timestamp`. If a pool has not been interacted with for a significant period (e.g., due to low activity), and there are orders expiring at many different time intervals (e.g., an attacker creates orders expiring every 30 minutes for a month), the number of iterations can become large. Since each iteration involves a `Core.swap` (which is gas-intensive) and storage updates, the gas cost to execute the catch-up logic can exceed the block gas limit. This would permanently brick the pool, preventing any future swaps or liquidity updates.
 ### Static Signals
while (time != block.timestamp), Core.swap inside loop, no limit on iterations
 ### Assets at Risk
Pool availability (DoS)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

