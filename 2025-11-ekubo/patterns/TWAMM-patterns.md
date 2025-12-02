## Verified Patterns Found: 26

## Verified Patterns Found in following Categories:

- UnsafeRecipient
- GriefableCallbacks
- SlippageMissingOrInsufficient
- ExternalCallAfterStateChange
- TWAPWindowPinningOrLowLiquidity
- StandardViolation
- StorageCollisionOrSelectorClash
- FeeOnTransferAssumption
- StaleOracleAcceptance
- UnboundedLoops
- AccountingInvariantViolation
- OracleUsingDEXorTWAP
- ReserveOrPriceDesync
- FlashLoanEconomicManipulation



## Summary of Patterns

TWAMM Virtual Orders Executed at Manipulatable Spot Price

Unbounded Tick Iteration Loop

Oracle relies on zero-fee pools prone to low liquidity and manipulation

Synthetic price clamping in Oracle Adapter causes silent price deviation

Missing slippage protection in position withdrawals

TokenWrapper transfers to Core desynchronize accounting

Multi-block TWAMM sandwiching via price manipulation

Unsafe Recipient in TokenWrapper Causes Fund Loss

State Overwrite via Reentrancy in Swap

Unbounded storage indexing allows overwriting other Airdrop states

Missing Slippage Protection in Positions Withdrawal

TWAP degenerates to Spot Price for inactive pools

Oracle susceptible to manipulation if configured with short TWAP duration

Critical price desync risk due to unvalidated proxy token addresses

MEV Capture fees bypassed via atomic arbitrage (round-trip swaps)

DoS of Oracle functionality due to insufficient default snapshot capacity

Fee-on-transfer tokens break Incentives funding accounting

Read-Hook-Write reentrancy in Core swap enables state corruption

Missing Zero-Address Check in Positions Withdrawal

MEVCapture extension permanently reverts swaps

Unsafe Oracle Wrapper for Low Liquidity Pools

Unsafe TWAP Duration and Liquidity Concentration Risk

Unbounded Loop DoS in TWAMM Virtual Order Execution

Oracle accepts stale prices from inactive pools

Missing Slippage Protection in Revenue Buybacks

Missing minimum TWAP duration enforcement

## Patterns



 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM Virtual Orders Executed at Manipulatable Spot Price
 ### Description/Code Snippet
The `TWAMM` extension executes pending virtual orders via `_executeVirtualOrdersFromWithinLock` by calling `CORE.swap` using the pool's current spot price/liquidity (`corePoolState.sqrtRatio()`). This execution is permissionless via `lockAndExecuteVirtualOrders` and occurs automatically during swaps. If there is a time gap since the last execution (accumulating volume) or if total sale rates are high, a significant amount of liquidity will be swapped. An attacker can use a flash loan to manipulate the spot price in the `Core` pool, call `lockAndExecuteVirtualOrders` to force the execution of the pending TWAMM volume at the distorted price, and then arbitrage the price back, effectively sandwiching the TWAMM orders without any slippage protection for the virtual execution.
 ### Static Signals
uses totalSupply/totalAssets in same tx as deposit/withdraw, price fetched at execution without user-specified floor
 ### Assets at Risk
User funds in TWAMM orders
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: Core.swap_6269342730

 ### Title
Unbounded Tick Iteration Loop
 ### Description/Code Snippet
The `swap` function in `Core` contains a `while (true)` loop that iterates through initialized ticks until the swap amount is exhausted or the price limit is reached. An attacker can create a 'tick bomb' by initializing many ticks with dust liquidity, causing swaps to consume excessive gas and revert (DoS), preventing legitimate trading.
 ### Static Signals
while (true), findNextInitializedTick
 ### Assets at Risk
Pool Availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Oracle relies on zero-fee pools prone to low liquidity and manipulation
 ### Description/Code Snippet
The underlying `Oracle` extension (in `Oracle.sol`) strictly enforces `key.config.fee() == 0` for any pool it tracks. Zero-fee pools lack economic incentives for Liquidity Providers (no swap fees earned), typically resulting in low liquidity. Furthermore, the absence of swap fees dramatically lowers the cost of manipulation (attackers only pay gas), making the TWAP provided by `ERC7726` significantly easier to pin or manipulate compared to standard fee-tier pools.
 ### Static Signals
revert FeeMustBeZero(), revert FullRangePoolOnly()
 ### Assets at Risk
Assets priced by this oracle
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: ERC7726.getAverageTick

 ### Title
Synthetic price clamping in Oracle Adapter causes silent price deviation
 ### Description/Code Snippet
The `getAverageTick` function calculates the relative price between two non-native tokens by subtracting their ticks relative to the native token (`quoteTick - baseTick`). It then explicitly clamps the result to `MIN_TICK` / `MAX_TICK` (approx ±887,228). While individual Ekubo pools are bounded by this range, a synthetic cross-pair (e.g., TokenA/TokenB) derived via the native token could theoretically have a relative price difference exceeding this range (approx 2^128 ratio). In such extreme cases, the oracle returns a clamped value (the max/min tick) instead of reverting or reporting the true synthetic price, leading to a silent and potentially massive price deviation for downstream integrations.
 ### Static Signals
FixedPointMathLib.min(MAX_TICK, FixedPointMathLib.max(MIN_TICK
 ### Assets at Risk
User funds relying on accurate oracle quotes
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing slippage protection in position withdrawals
 ### Description/Code Snippet
The `withdraw` function in `Positions` (and `BasePositions`) accepts a `liquidity` amount to burn but does not provide parameters for `amount0Min` and `amount1Min`. Users cannot enforce a minimum return of tokens, exposing them to unfavorable execution prices (slippage) or sandwich attacks during withdrawal, where they might receive a manipulated ratio of assets.
 ### Static Signals
no amountOutMin parameters, returns (uint128 amount0, uint128 amount1)
 ### Assets at Risk
User liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: TokenWrapper.transfer

 ### Title
TokenWrapper transfers to Core desynchronize accounting
 ### Description/Code Snippet
The `TokenWrapper` contract allows standard ERC20 transfers to the `Core` address, which increases `coreBalance` but does not update Core's internal `savedBalances` ledger (the 'till'). Since Core's withdrawal logic relies strictly on `savedBalances`, any tokens transferred directly to Core via `transfer()` become permanently stuck, violating the accounting invariant between the token's ledger and the protocol's ledger.
 ### Static Signals
balance tracking references different token than actual holdings, accounting state not updated when underlying asset is swapped/upgraded
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: TWAMM.executeVirtualOrdersFromWithinLock

 ### Title
Multi-block TWAMM sandwiching via price manipulation
 ### Description/Code Snippet
The `TWAMM` extension executes virtual orders based on the pool's price movement over time. While `executeVirtualOrders` is atomic with the first interaction in a block, an attacker can manipulate the pool price in one transaction, wait for the TWAMM to execute against this manipulated price in a subsequent block/transaction, and then reverse the trade, effectively sandwiching the TWAMM order flow.
 ### Static Signals
atomic swap → read price → execute logic pattern, share price derived from manipulable pool state
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: TokenWrapper.transfer

 ### Title
Unsafe Recipient in TokenWrapper Causes Fund Loss
 ### Description/Code Snippet
The `TokenWrapper` contract overrides `transfer` and `transferFrom` to handle transfers to the `CORE` address specially by updating a transient `coreBalance` variable. This mechanism is intended for Core's internal accounting during flash accounting settlements. However, if a user manually transfers tokens to the `CORE` address (e.g., by mistake or attempting to deposit directly), the `coreBalance` increases only for the duration of the transaction. Since transient storage is cleared at the end of the transaction and no `savedBalances` update is triggered, these tokens are effectively burned.
 ### Static Signals
if (to == address(CORE)), coreBalance += amount, transient coreBalance
 ### Assets at Risk
User Funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ExternalCallAfterStateChange

 ### Relevant Function/Location: Core.swap_6269342730

 ### Title
State Overwrite via Reentrancy in Swap
 ### Description/Code Snippet
Core.swap reads pool state into memory before calling the `beforeSwap` extension hook. If the extension (e.g., TWAMM) re-enters `swap` to execute virtual orders, the inner swap updates the pool state. However, when the hook returns, the outer swap continues using the stale memory state and eventually overwrites the storage, effectively reverting the inner swap's price/liquidity updates while keeping the debt/accounting changes. This breaks TWAMM price impact logic.
 ### Static Signals
readPoolState, maybeCallBeforeSwap, writePoolState
 ### Assets at Risk
Liquidity Provider Funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: Incentives.claim

 ### Title
Unbounded storage indexing allows overwriting other Airdrop states
 ### Description/Code Snippet
In `Incentives.claim`, the storage slot for the claim bitmap is calculated as `keccak(key) + 1 + c.index/256`. Since `c.index` is user-provided (verified only against a Merkle root set by the drop owner) and unbounded, a malicious drop owner can craft a Merkle tree with a massive `index` to generate a storage pointer that wraps around or overlaps with the storage of other drops (e.g., `DropState` at `sload(other_id)`), corrupting their funded/claimed amounts.
 ### Static Signals
manual assembly slots without namespace, storage index derived from user input
 ### Assets at Risk
rewards, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.withdraw

 ### Title
Missing Slippage Protection in Positions Withdrawal
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` allows users to burn liquidity and receive tokens without specifying minimum output amounts (`amount0Min`, `amount1Min`). In `Positions.sol`, this is exposed as `withdraw`. While the function returns the amounts received, it does not revert if they are lower than an expected threshold. In a volatile market or during a sandwich attack, the ratio of assets in the position can shift drastically (e.g., from 50/50 to 0/100), causing the user to withdraw an unfavorable composition of assets or less value than intended without any on-chain guardrail.
 ### Static Signals
withdraw() converts shares to assets at current rate without minimum, collateral payout uses current price without slippage protection
 ### Assets at Risk
User liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: Oracle.extrapolateSnapshotInternal

 ### Title
TWAP degenerates to Spot Price for inactive pools
 ### Description/Code Snippet
The `extrapolateSnapshot` function extrapolates the cumulative values from the last snapshot using the *current* spot tick (`CORE.poolState(poolId)`). If a pool has been inactive for longer than the configured `TWAP_DURATION` (meaning the last snapshot is outside the window), the function uses the current spot tick to weight the entire duration. This effectively degenerates the TWAP into a Spot Price Oracle, allowing an attacker to manipulate the price in the same block (via flash loan) and have the oracle report that manipulated price as the valid TWAP.
 ### Static Signals
if (logicalIndex == c.count() - 1), CORE.poolState(poolId), tickCumulative += int64(state.tick()) * int64(uint64(timePassed))
 ### Assets at Risk
Integrator funds, Lending protocols using ERC7726
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: ERC7726.constructor

 ### Title
Oracle susceptible to manipulation if configured with short TWAP duration
 ### Description/Code Snippet
The `ERC7726` contract allows setting `TWAP_DURATION` to any value > 0 in the constructor. If configured with a very short duration (e.g., equal to or less than the block time), the TWAP effectively degenerates into a spot price (or single-block average). While Ekubo's snapshot mechanism protects against same-transaction manipulation, a short TWAP window allows multi-block manipulation where an attacker manipulates the pool price in block N and queries the oracle in block N+1. The contract does not enforce a minimum safe duration.
 ### Static Signals
twapDuration == 0, block.timestamp - TWAP_DURATION
 ### Assets at Risk
Protocols integrating the oracle
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: ERC7726.constructor

 ### Title
Critical price desync risk due to unvalidated proxy token addresses
 ### Description/Code Snippet
The `ERC7726` constructor accepts addresses for `usdProxyToken`, `btcProxyToken`, and `ethProxyToken` without validating that they are non-zero. The `normalizeAddress` function maps specific ERC-7726 identifiers to these proxy tokens. If `usdProxyToken` (or others) is initialized to `address(0)` (default `NATIVE_TOKEN_ADDRESS` in Ekubo), `normalizeAddress` will map `IERC7726_USD_ADDRESS` to `NATIVE_TOKEN_ADDRESS`. This results in the oracle reporting `USD` prices as `ETH` prices (1:1), leading to a catastrophic pricing desync for any integration relying on these standard identifiers.
 ### Static Signals
address public immutable USD_PROXY_TOKEN, normalizeAddress
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEV Capture fees bypassed via atomic arbitrage (round-trip swaps)
 ### Description/Code Snippet
The `MEVCapture` extension calculates the additional fee based on the absolute tick displacement from the start of the block (`tickLast`). The `tickLast` value is updated only if `lastUpdateTime != block.timestamp`, effectively pinning it to the state before the first interaction in the block. 

Atomic arbitrage strategies typically move the price (tick) to capture value and then immediately move it back (or another transaction does so in the same block) to align with external markets. If a transaction or sequence of transactions in the same block results in the tick returning to (or near) `tickLast` (e.g., `A -> B -> A`), the calculated `additionalFee` for the second leg (or the net effect) becomes zero or negligible. This allows arbitrageurs to extract MEV without paying the intended capture fee, undermining the extension's purpose.
 ### Static Signals
lastUpdateTime != currentTime, tickLast not updated in same block, fee calculation based on tick displacement
 ### Assets at Risk
protocol revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC7726.getAverageTick

 ### Title
DoS of Oracle functionality due to insufficient default snapshot capacity
 ### Description/Code Snippet
The `ERC7726` contract relies on the Ekubo `Oracle` extension to provide historical tick data at `block.timestamp - TWAP_DURATION`. However, the `Oracle` extension initializes new pools with a default snapshot capacity of 1 (via `max(1, c.capacity())`). If a pool is active (updated frequently), the single snapshot slot is constantly overwritten with the latest block's data. Consequently, `extrapolateSnapshot` will invariably fail to find a snapshot older than `TWAP_DURATION` for any active pool, causing `getQuote` to revert with `NoPreviousSnapshotExists`. The `ERC7726` contract does not verify capacity or expand it, rendering it unusable for active pools without manual external intervention.
 ### Static Signals
ORACLE.extrapolateSnapshot, TWAP_DURATION
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Fee-on-transfer tokens break Incentives funding accounting
 ### Description/Code Snippet
The `fund` function calculates `fundedAmount` based on the requested `minimum` minus current funded state, then calls `safeTransferFrom` for that amount. It updates the internal `funded` state with the full amount. If the token charges a fee on transfer, the contract receives less than the accounting record reflects. Subsequent `claim` calls will fail for the last claimers due to insufficient actual token balance.
 ### Static Signals
dropState.setFunded(minimum), safeTransferFrom(..., fundedAmount), no balance check after transfer
 ### Assets at Risk
Incentive rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: Core.swap

 ### Title
Read-Hook-Write reentrancy in Core swap enables state corruption
 ### Description/Code Snippet
In `Core.swap`, `readPoolState` loads the pool state into memory before calling `maybeCallBeforeSwap`. If a malicious or complex extension re-enters `Core.swap` (or `updatePosition`) on the same pool within the hook, the inner call updates the storage. When the hook returns, the outer execution continues using the stale in-memory `stateAfter` and eventually overwrites the storage via `writePoolState`, discarding the inner execution's updates and corrupting the pool's liquidity/tick tracking.
 ### Static Signals
readPoolState, maybeCallBeforeSwap, writePoolState, no nonReentrant modifier
 ### Assets at Risk
Liquidity Provider funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing Zero-Address Check in Positions Withdrawal
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` allows users to specify a `recipient` address. This address is passed to `ACCOUNTANT.withdrawTwo`, which eventually calls `FlashAccountant.withdraw`. The logic does not verify that `recipient` is non-zero. If a user accidentally passes `address(0)` (or if a frontend defaults to it), the tokens or ETH will be sent to the zero address and permanently lost. While `SafeTransferLib` handles transfers, burning tokens is usually not the intended behavior for a withdrawal function without explicit intent.
 ### Static Signals
no zero-address guard
 ### Assets at Risk
User LP tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture extension permanently reverts swaps
 ### Description/Code Snippet
The `MEVCapture` extension registers for the `beforeSwap` hook but implements it to strictly `revert SwapMustHappenThroughForward()`. However, `handleForwardData` calls `CORE.swap`, which triggers the registered `beforeSwap` hook again. Since the hook does not differentiate between a direct call and a recursive call from `handleForwardData`, it reverts, causing a permanent DoS for any pool using this extension.
 ### Static Signals
revert SwapMustHappenThroughForward(), beforeSwap: true in call points, unconditional revert
 ### Assets at Risk
Pool usability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: ERC7726.getAverageTick

 ### Title
Unsafe Oracle Wrapper for Low Liquidity Pools
 ### Description/Code Snippet
The `ERC7726` adapter allows deriving a price for any token pair by routing through the Native token (e.g. Token->Native->Quote). It does not enforce any minimum liquidity depth, observation cardinality, or volume thresholds on the underlying Ekubo pools. An attacker can manipulate the price of a low-liquidity pool on Ekubo cheaply and the `ERC7726` contract will report this manipulated TWAP as the canonical price.
 ### Static Signals
uses single low-liquidity pair without liquidity floor, observationCardinality/min not enforced
 ### Assets at Risk
Protocol funds relying on this oracle for pricing
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: ERC7726.constructor

 ### Title
Unsafe TWAP Duration and Liquidity Concentration Risk
 ### Description/Code Snippet
The ERC7726 oracle contract allows setting a `TWAP_DURATION` as low as 1 second in the constructor. Short TWAP windows are susceptible to manipulation via multi-block MEV or spot price manipulation if the cost of sustaining the price is low. Additionally, the reliance on `Oracle.sol` forces all pricing to be derived from pools paired with the NATIVE token (ETH). If the NATIVE pair has low liquidity compared to other direct pairs (e.g., USDC/USDT vs USDC/ETH), the oracle will provide an easily manipulatable price despite robust liquidity existing elsewhere.
 ### Static Signals
twapWindow < 10–30 minutes, uses single low-liquidity pair, no min trades/volume threshold before trusting price
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded Loop DoS in TWAMM Virtual Order Execution
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function in `TWAMM.sol` iterates through all initialized time intervals between the `lastVirtualOrderExecutionTime` and `block.timestamp`. An attacker can create numerous TWAMM orders with distinct expiration times (e.g., every minute), populating the `poolInitializedTimesBitmap`. If the pool is not interacted with for a significant period, the number of intervals to process in the loop may grow large enough to exceed the block gas limit, effectively permanently denying service (DoS) to the pool.
 ### Static Signals
while (time != block.timestamp), searchForNextInitializedTime
 ### Assets at Risk
Liquidity Pool Availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: ERC7726.getAverageTick

 ### Title
Oracle accepts stale prices from inactive pools
 ### Description/Code Snippet
The `ERC7726` contract retrieves price data via `ORACLE.extrapolateSnapshot`. If the underlying Ekubo pool has not traded recently, `Oracle.sol` extrapolates values using the pool's current `state.tick()`, which reflects the price at the time of the last trade. Since there is no heartbeat or recency check in `ERC7726` (nor in the `IOracle` interface), a pool that has been inactive for a long period (e.g., due to liquidity migration) will report a stale price as current, allowing attacks against protocols relying on fresh data.
 ### Static Signals
no updatedAt/answeredInRound checks, extrapolateSnapshot used without age check, uses current pool state for extrapolation
 ### Assets at Risk
Protocols using this oracle for collateral/lending
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Missing Slippage Protection in Revenue Buybacks
 ### Description/Code Snippet
The `RevenueBuybacks.roll` function executes a TWAMM sell order using the contract's entire revenue balance. It calls `ORDERS.increaseSellAmount` passing `type(uint112).max` as the `maxSaleRate`. There is no parameter to specify a minimum output amount or a price limit (`sqrtRatioLimit`) for the resulting virtual orders. While TWAMM orders execute over time, the lack of slippage protection exposes the protocol's revenue to potential price manipulation or poor execution, particularly if the configured `targetOrderDuration` is short.
 ### Static Signals
type(uint112).max, amountOutMin=0, increaseSellAmount
 ### Assets at Risk
Protocol Revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: ERC7726.constructor

 ### Title
Missing minimum TWAP duration enforcement
 ### Description/Code Snippet
The `ERC7726` constructor sets `TWAP_DURATION` based on user input, validating only that it is non-zero. If deployed with a short duration (e.g., < 30 minutes), the oracle becomes equivalent to a spot price feed or short-window TWAP, susceptible to sandwich attacks and single-block manipulation, especially given the zero-fee nature of the underlying pools.
 ### Static Signals
twapWindow < 10–30 min, no min observation / heartbeat, single spot read from AMM/DEX
 ### Assets at Risk
Assets priced by this oracle
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole

