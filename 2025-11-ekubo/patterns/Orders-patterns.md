## Verified Patterns Found: 13

## Verified Patterns Found in following Categories:

- TWAPWindowPinningOrLowLiquidity
- Reentrancy
- FeeAccountingDrift
- ReserveOrPriceDesync
- UnboundedLoops
- MulticallCrossPathReentrancy
- FlashLoanEconomicManipulation
- StandardViolation
- SlippageMissingOrInsufficient
- PrecisionDriftAccumulation



## Summary of Patterns

Broken Interface Integration in Orders Contract

Fee Accounting Drift in Reward Distribution

Missing slippage protection in TWAMM Order creation

Stale Price Execution in TWAMM after Inactivity

Infinite Recursion via Extension Hook Reentrancy

MEV Capture Fee Avoidance on Price Reversion

Oracle Snapshot Pinning Vulnerability

Reentrancy DoS in TWAMM Virtual Order Execution

Infinite recursion loop in TWAMM virtual order execution

Recursion DoS via Hook Reentrancy in TWAMM Swaps

Unbounded Loop in TWAMM Execution

Principal Loss via Rounding in High-Frequency Execution

One-Way TWAMM Execution Lacks Slippage Protection

## Patterns



 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Orders.executeVirtualOrdersAndGetCurrentOrderInfo

 ### Title
Broken Interface Integration in Orders Contract
 ### Description/Code Snippet
The `Orders.sol` contract calls `TWAMM_EXTENSION.executeVirtualOrdersAndGetCurrentOrderInfo`, but this function is not declared in the `ITWAMM` interface nor implemented in the `TWAMM` contract provided. This results in a broken function `executeVirtualOrdersAndGetCurrentOrderInfo` in the `Orders` contract, which prevents users from querying order info or triggering virtual order execution via this entry point.
 ### Static Signals
standard-required function missing or has wrong signature, function call to missing interface method
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Fee Accounting Drift in Reward Distribution
 ### Description/Code Snippet
The `TWAMM` contract calculates order rewards (proceeds) by updating `rewardRates` based on the difference between the expected input amount and the actual pool swap delta. This calculation uses `FixedPointMathLib.rawDiv` on the delta scaled by 128 bits. Due to rounding down in division, small amounts of proceeds (dust) may not be accounted for in the `rewardRates`, leading to a systematic accumulation of tokens in the contract that are never distributed to order owners. Over time or with high sale rates, this drift can become significant.
 ### Static Signals
rawDiv, rounding down in reward rate calculation
 ### Assets at Risk
User Proceeds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Orders.increaseSellAmount

 ### Title
Missing slippage protection in TWAMM Order creation
 ### Description/Code Snippet
The `Orders.increaseSellAmount` function allows users to create or add to TWAMM orders by specifying a `maxSaleRate` but does not allow specifying a `minAmountOut` or minimum effective price for the order. While TWAMM orders are executed over time, the lack of a minimum acceptable output exposes users to potential price manipulation or sandwich attacks during the blocks where their virtual orders are executed, especially since `executeVirtualOrders` is permissionless and can be sandwiched.
 ### Static Signals
amountOutMin=0 or missing, no minAmountOut parameter
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Stale Price Execution in TWAMM after Inactivity
 ### Description/Code Snippet
The TWAMM extension executes virtual orders using the pool's *current* spot price (`CORE.poolState(poolId).sqrtRatio()`) as the starting point for the interval since the last execution. If a pool has been inactive for a significant period (e.g., multiple blocks), and the market price has moved significantly, the first transaction to touch the pool will execute all pending TWAMM orders starting from the *old, stale* price before the price is updated by new swaps. This allows arbitrageurs to force TWAMM orders to trade at unfavorable stale prices, causing loss to LPs or order holders.
 ### Static Signals
uses totalSupply/totalAssets in same tx as deposit/withdraw, assumes invariant without verifying
 ### Assets at Risk
treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: MulticallCrossPathReentrancy

 ### Relevant Function/Location: TWAMM.beforeSwap

 ### Title
Infinite Recursion via Extension Hook Reentrancy
 ### Description/Code Snippet
The `TWAMM` contract, acting as a pool extension, triggers virtual order execution in its `beforeSwap` hook via `lockAndExecuteVirtualOrders`. This calls `CORE.lock`, which callbacks `TWAMM.locked`, which executes `_executeVirtualOrdersFromWithinLock`. Crucially, this execution function calls `CORE.swap` to settle virtual orders against the pool. `CORE.swap` triggers the `beforeSwap` hook again for the same pool. Since the `TWAMM` contract does not update its `lastVirtualOrderExecutionTime` state until *after* the swap loop completes, the recursive call sees stale state and re-executes the same logic, leading to infinite recursion and denial of service for all swaps in TWAMM pools.
 ### Static Signals
state update after external call, hook calls function that triggers same hook
 ### Assets at Risk
Pool Availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEV Capture Fee Avoidance on Price Reversion
 ### Description/Code Snippet
The `MEVCapture` extension calculates fees based on the tick displacement from the start of the block (`tickLast`) rather than the previous transaction. `tickLast` is only updated once per block. This allows an attacker (e.g., in a sandwich attack) to execute a second swap that reverts the price to the original tick without paying any MEV capture fees, as `abs(stateAfter.tick() - tickLast)` would be zero. This reduces the efficacy of the MEV capture mechanism for sandwich attacks and high-frequency volatility.
 ### Static Signals
fee calculated from spot state, state update skipped in hot path
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.maybeInsertSnapshot

 ### Title
Oracle Snapshot Pinning Vulnerability
 ### Description/Code Snippet
The `Oracle` extension records snapshots in `maybeInsertSnapshot` using the state *before* the current interaction (swap/position update). This effectively captures the pool state as it existed at the end of the *previous* transaction. An attacker can manipulate the price at the end of a block (via a final swap), and this manipulated price will be recorded as the price for the entire duration until the next interaction (potentially multiple blocks), allowing cheap TWAP manipulation (pinning).
 ### Static Signals
snapshot records previous state over timePassed, no cap on price movement between snapshots
 ### Assets at Risk
Protocol relying on Oracle TWAP
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Reentrancy DoS in TWAMM Virtual Order Execution
 ### Description/Code Snippet
The `TWAMM` contract's `_executeVirtualOrdersFromWithinLock` function executes virtual orders by calling `CORE.swap`. However, `CORE.swap` triggers the `beforeSwap` hook on the pool's extension, which is the `TWAMM` contract itself. The `beforeSwap` hook calls `lockAndExecuteVirtualOrders`, which re-enters `_executeVirtualOrdersFromWithinLock`. 

Crucially, the `TwammPoolState` (containing `lastVirtualOrderExecutionTime`) is only updated in storage at the *end* of `_executeVirtualOrdersFromWithinLock`. Therefore, the recursive call reads the stale `lastVirtualOrderExecutionTime`, determines that time has passed, and attempts to execute the same virtual orders again for the same time interval. This creates an infinite recursion loop that will cause the transaction to revert (Out of Gas), effectively causing a Denial of Service for any pool with active TWAMM orders.
 ### Static Signals
state update after external call, external call triggers hook back to contract, missing nonReentrant modifier
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: MulticallCrossPathReentrancy

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Infinite recursion loop in TWAMM virtual order execution
 ### Description/Code Snippet
The `TWAMM._executeVirtualOrdersFromWithinLock` function iterates through time intervals and calls `CORE.swap` to execute virtual orders. `CORE.swap` triggers the `TWAMM.beforeSwap` hook, which calls `lockAndExecuteVirtualOrders`, which in turn calls `CORE.lock` and re-enters `_executeVirtualOrdersFromWithinLock`. Since the state variable `lastVirtualOrderExecutionTime` is only updated in storage after the swap loop completes, the re-entrant call sees the old timestamp, passes the `realLastVirtualOrderExecutionTime != block.timestamp` check, and attempts to execute the same virtual orders again. This results in an infinite recursion (until gas exhaustion) whenever `executeVirtualOrders` is triggered for a pool with active orders, effectively causing a DoS of the pool.
 ### Static Signals
CORE.swap call inside loop, storage update after external call
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: TWAMM.beforeSwap

 ### Title
Recursion DoS via Hook Reentrancy in TWAMM Swaps
 ### Description/Code Snippet
The `TWAMM` extension registers the `beforeSwap` hook. When `TWAMM` executes virtual orders via `_executeVirtualOrdersFromWithinLock`, it calls `CORE.swap`. `CORE.swap` invokes the `beforeSwap` hook on the pool's extension (which is `TWAMM`). `TWAMM.beforeSwap` calls `lockAndExecuteVirtualOrders`, which calls `CORE.lock` and re-enters `_executeVirtualOrdersFromWithinLock`. This creates an infinite recursion loop, causing all swaps and virtual order executions on TWAMM-enabled pools to revert due to out-of-gas (DoS).
 ### Static Signals
untrusted call before all updates, missing/nonfunctional nonReentrant, call to CORE.swap inside lock callback, hook triggers core function that triggers hook
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded Loop in TWAMM Execution
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function iterates through time intervals using `searchForNextInitializedTime` inside a `while (time != block.timestamp)` loop. If a pool has orders expiring at frequent intervals (e.g., every few minutes) and goes untouched for a significant period (e.g., weeks or months), the number of iterations required to catch up to the current block timestamp may exceed the block gas limit. This would permanently DoS the pool, preventing any new swaps or position updates.
 ### Static Signals
while (time != block.timestamp), searchForNextInitializedTime in loop, state updated only after loop
 ### Assets at Risk
Pool availability (DoS)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Principal Loss via Rounding in High-Frequency Execution
 ### Description/Code Snippet
Virtual order sale amounts are calculated as `(saleRate * duration) >> 32`. Since `lockAndExecuteVirtualOrders` can be called frequently by anyone, an attacker can trigger execution with small `duration` intervals. If `saleRate * duration < 2^32`, the sold amount rounds down to zero, but the time duration is consumed in `OrderState`. This leads to `PrecisionDriftAccumulation` where user funds are permanently stuck/lost because the system accounts for the time passing without crediting the user with sold tokens or refunding the unsold amount.
 ### Static Signals
consistent floor toward sender/receiver, downcasts without range checks, rounding in distribution
 ### Assets at Risk
user funds in orders
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
One-Way TWAMM Execution Lacks Slippage Protection
 ### Description/Code Snippet
In `_executeVirtualOrdersFromWithinLock`, when executing one-way virtual orders (where only one token has a non-zero sale rate), the contract calls `CORE.swap` with `MIN_SQRT_RATIO` or `MAX_SQRT_RATIO` as the limit. This effectively accepts any price for the swap. An attacker can sandwich this execution by JIT-manipulating the pool liquidity or price before the TWAMM execution (which is permissionless via `lockAndExecuteVirtualOrders`), forcing the protocol to swap at an extremely unfavorable rate, and then arbitraging the difference.
 ### Static Signals
_sqrtRatioLimit: MIN_SQRT_RATIO, _sqrtRatioLimit: MAX_SQRT_RATIO
 ### Assets at Risk
User Funds in TWAMM Orders
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

