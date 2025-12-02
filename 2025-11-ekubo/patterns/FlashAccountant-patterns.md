## Verified Patterns Found: 17

## Verified Patterns Found in following Categories:

- FlashLoanEconomicManipulation
- UnboundedLoops
- StandardViolation
- TWAPWindowPinningOrLowLiquidity
- FeeOnTransferAssumption
- SlippageMissingOrInsufficient
- ERC20DecimalsMismatch
- PrecisionDriftAccumulation



## Summary of Patterns

FlashAccountant updateDebt enforces non-standard ABI encoding preventing integration

Oracle extension initializes with unsafe buffer capacity

Incentives contract insolvent for fee-on-transfer tokens

MEV Capture extension imposes disproportionate fees on users following large price movements

TWAMM Execution Vulnerable to Liquidity Manipulation Sandwich

RevenueBuybacks collect lacks minimum amount parameter

Precision Drift in TWAMM Rewards due to Frequent Execution

MEV Capture fee logic disproportionately penalizes subsequent swaps

Silent sale rate truncation in Orders for high-supply tokens

Router swap overload disables slippage protection

Fee-on-transfer tokens break Incentives accounting

Positions.withdraw lacks slippage protection arguments

Unbounded loop in TWAMM virtual order execution

MEVCapture Extension Permanently Reverts Swaps

Revenue buybacks lack price floor or slippage protection

Potential Gas DoS via unbounded TWAMM virtual order execution loop

Missing slippage protection in Positions withdrawal

## Patterns



 ### Issue Type: StandardViolation

 ### Relevant Function/Location: FlashAccountant.updateDebt

 ### Title
FlashAccountant updateDebt enforces non-standard ABI encoding preventing integration
 ### Description/Code Snippet
The `updateDebt` function in `FlashAccountant` (inherited by `Core`) explicitly reverts if `msg.data.length != 20`. This enforces a custom packed encoding (4-byte selector + 16-byte int128) instead of the standard Solidity ABI encoding (which pads arguments to 32 bytes, resulting in 36 bytes). This deviation from the standard prevents standard smart contracts, multisigs, and tooling from interacting with this function, potentially causing integration failures or Denial of Service for valid use cases attempting to settle debt via standard calls.
 ### Static Signals
msg.data.length != 20, calldataload(4), signextend(15, ...)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Oracle extension initializes with unsafe buffer capacity
 ### Description/Code Snippet
The `Oracle` extension initializes the snapshot buffer capacity to 1 in `beforeInitializePool` (via `createCounts`). The `maybeInsertSnapshot` function overwrites existing snapshots in a circular buffer. With a capacity of 1, the oracle only stores the single most recent observation. This makes `extrapolateSnapshot` (which relies on finding a 'previous' snapshot) effectively useless for any time window longer than the time between interactions, and highly susceptible to spot price manipulation (pinning), unless the user manually calls `expandCapacity`.
 ### Static Signals
observationCardinality/min not enforced, twapWindow < 10–30 minutes
 ### Assets at Risk
oracles
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Incentives contract insolvent for fee-on-transfer tokens
 ### Description/Code Snippet
The `fund` function in `Incentives.sol` accepts a `fundedAmount` and calls `safeTransferFrom` to transfer that amount from the funder to the contract. It then updates `dropState.funded` with the input `fundedAmount`. However, it does not check the actual balance increase of the contract. If the token charges a fee on transfer, the contract receives less than `fundedAmount`, but the internal accounting records the full amount. This leads to insolvency where the last users attempting to `claim` will fail due to insufficient balance, breaking the `fund` vs `claim` invariant.
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEV Capture extension imposes disproportionate fees on users following large price movements
 ### Description/Code Snippet
The `MEVCapture` extension calculates an additional fee based on the absolute difference between the current tick and `tickLast`. However, `tickLast` is only updated when `lastUpdateTime` (the block timestamp) changes. 

In `handleForwardData`, the fee logic is:
`feeMultiplierX64 = (FixedPointMathLib.abs(stateAfter.tick() - tickLast) << 64) / poolKey.config.concentratedTickSpacing();`
`additionalFee = ... (feeMultiplierX64 * poolFee) ...`

If a large swap (or sequence of swaps) occurs at the beginning of a block, it moves `stateAfter.tick()` far away from `tickLast` (which remains pinned to the block's start tick). Any subsequent user interacting with the pool in the same block will have their fee calculated based on this large displacement, even if their own transaction only moves the tick slightly or moves it back. This creates a griefing vector where a large transaction forces all subsequent users in the block to pay exorbitant fees, potentially causing transactions to fail (due to slippage/output checks) or users to overpay significantly.
 ### Static Signals
tickLast updated only on timestamp change, Fee depends on state.tick() - tickLast
 ### Assets at Risk
User funds (excessive fees paid to protocol)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM Execution Vulnerable to Liquidity Manipulation Sandwich
 ### Description/Code Snippet
TWAMM executes accumulated virtual orders lazily in `_executeVirtualOrdersFromWithinLock` using the pool's *current* spot liquidity (`CORE.poolState(poolId).liquidity()`). An attacker can manipulate the execution price by withdrawing liquidity in block N, waiting for virtual orders to accumulate, and then triggering execution in block N+1 via a swap or update. The virtual orders will execute against the thin liquidity, causing massive price impact. The attacker can then arbitrage the price and restore liquidity, extracting value from TWAMM users.
 ### Static Signals
CORE.poolState(poolId).liquidity(), lazy execution, spot liquidity usage
 ### Assets at Risk
TWAMM order funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: RevenueBuybacks.collect

 ### Title
RevenueBuybacks collect lacks minimum amount parameter
 ### Description/Code Snippet
The `collect` function in `RevenueBuybacks` calls `ORDERS.collectProceeds` to withdraw the results of a TWAMM order. It does not accept a `minAmountOut` parameter, nor does it check the returned `proceeds` against any minimum. This exposes the protocol's revenue buybacks to loss if the TWAMM execution was manipulated or had high slippage due to low liquidity.
 ### Static Signals
payout calculated at execution time without minimum bound, missing minAmountOut parameter
 ### Assets at Risk
rewards, treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Precision Drift in TWAMM Rewards due to Frequent Execution
 ### Description/Code Snippet
TWAMM orders execute virtually over time. The user's `amountSold` is calculated based on the total duration elapsed since the last update. However, the actual swap execution in the pool (`_executeVirtualOrdersFromWithinLock`) occurs incrementally over time intervals found in the bitmap. If these intervals are small (e.g., 1 second, potentially forced by an attacker), the swap amount (`saleRate * duration`) may round down to zero or suffer significant precision loss for smaller orders. Consequently, no swap occurs in the pool, and no reward (buy token) is generated. The user is still debited the 'sold' amount in accounting but receives zero or reduced proceeds, effectively leaking value to LPs.
 ### Static Signals
computeAmountFromSaleRate, CORE.swap, amount0 != 0 && amount1 != 0
 ### Assets at Risk
User order principal
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.sol.handleForwardData

 ### Title
MEV Capture fee logic disproportionately penalizes subsequent swaps
 ### Description/Code Snippet
In `MEVCapture.sol`, the fee is calculated based on the tick difference between the current tick and `tickLast`. `tickLast` is updated only when `lastUpdateTime != currentTime`. Consequently, for multiple swaps occurring in the same block (same `currentTime`), `tickLast` remains fixed at the start-of-block value. Users executing swaps later in the block are charged fees based on the cumulative price movement of all previous swaps in that block, effectively paying for price impact caused by others. This allows for griefing and makes trading economically unviable later in the block.
 ### Static Signals
tickLast not updated after swap, fee based on cumulative movement
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: Orders.increaseSellAmount

 ### Title
Silent sale rate truncation in Orders for high-supply tokens
 ### Description/Code Snippet
In `Orders.increaseSellAmount`, the `saleRate` is calculated using `computeSaleRate` (which returns uint256) and then explicitly cast to `uint112`. This cast truncates the upper bits without checking for overflow. For tokens with high supply or decimals (e.g. >1e24 units), or short order durations, the calculated rate can exceed `type(uint112).max`. The truncation results in a much lower sale rate than intended, bypassing the subsequent `maxSaleRate` check and executing the order incorrectly.
 ### Static Signals
uint112(computeSaleRate(...)), saleRate > maxSaleRate
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router.swap

 ### Title
Router swap overload disables slippage protection
 ### Description/Code Snippet
The `Router` contract includes an overloaded `swap` function that sets `calculatedAmountThreshold` (the minimum output amount) to `type(int256).min`. If a user calls this function while also leaving `sqrtRatioLimit` as the default (min/max), the swap executes with absolutely no slippage protection, exposing the user to sandwich attacks or bad execution.
 ### Static Signals
type(int256).min, amountOutMin=0 or missing
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.sol.fund

 ### Title
Fee-on-transfer tokens break Incentives accounting
 ### Description/Code Snippet
The `Incentives` contract's `fund` function uses `safeTransferFrom` to deposit tokens but credits the `funded` state with the full input `minimum` amount. If the token charges a fee on transfer, the contract receives fewer tokens than recorded. Since `claim` and `refund` rely on the `funded` state to determine validity, the contract will eventually run out of tokens, causing valid claims or refunds to revert due to insufficient balance.
 ### Static Signals
safeTransferFrom, accounting based on input amount, no balance check
 ### Assets at Risk
Incentives rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.sol.withdraw

 ### Title
Positions.withdraw lacks slippage protection arguments
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` allows Liquidity Providers to burn their position liquidity and receive the underlying tokens (`amount0` and `amount1`). The function accepts the `liquidity` amount to burn but does not include parameters for `minAmount0` or `minAmount1`. 

The amounts of token0 and token1 received for a given amount of liquidity depend on the pool's current tick (price). If the pool price is manipulated (e.g., via a sandwich attack or flash loan) immediately before the withdrawal transaction is executed, the user may receive an unfavorable ratio of assets, suffering unexpected impermanent loss or receiving a lower total value than anticipated. 

While `Positions.deposit` includes `maxAmount0`, `maxAmount1`, and `minLiquidity` to protect against slippage during minting, the withdrawal counterpart lacks the inverse protections.
 ### Static Signals
liquidity input without minAmount0/minAmount1 inputs, returns calculated amounts without check
 ### Assets at Risk
User liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM.sol._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded loop in TWAMM virtual order execution
 ### Description/Code Snippet
The `TWAMM` extension iterates through all initialized time intervals between the last execution and the current block timestamp in `_executeVirtualOrdersFromWithinLock`. An attacker can cheaply create many orders with distinct end times (e.g., every minute) to densely populate the `initializedTimesBitmap`. This forces the `while` loop to execute many iterations, each involving heavy `Core.swap` calls and storage updates, potentially exceeding the block gas limit and causing a Denial of Service for the pool.
 ### Static Signals
while loop, state-dependent iteration, loop over time intervals
 ### Assets at Risk
Pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture Extension Permanently Reverts Swaps
 ### Description/Code Snippet
The `MEVCapture` extension implements the `beforeSwap` hook by unconditionally reverting with `SwapMustHappenThroughForward()`. However, `Core.swap` calls `maybeCallBeforeSwap` for the registered extension regardless of the caller (even if the caller is the extension itself via `handleForwardData`). Since `MEVCapture` registers `beforeSwap: true` in its call points, any attempt to swap against a pool using this extension (even via the intended `MEVCaptureRouter` flow) will trigger the hook and revert, rendering the pool unusable.
 ### Static Signals
revert SwapMustHappenThroughForward(), beforeSwap: true
 ### Assets at Risk
User funds (DoS of liquidity)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Revenue buybacks lack price floor or slippage protection
 ### Description/Code Snippet
The `RevenueBuybacks.sol` contract converts protocol revenue into `BUY_TOKEN` by creating TWAMM orders via `roll()`. While `roll()` accepts current balances, it does not enforce any minimum price or slippage bound for the `BUY_TOKEN`. If the `BUY_TOKEN` price crashes or the pool is manipulated, the protocol will continue selling revenue tokens for a negligible amount of `BUY_TOKEN` over the order duration, effectively leaking protocol revenue.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound
 ### Assets at Risk
treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Potential Gas DoS via unbounded TWAMM virtual order execution loop
 ### Description/Code Snippet
The `TWAMM` extension executes virtual orders lazily in `_executeVirtualOrdersFromWithinLock`. This function runs a `while` loop from `realLastVirtualOrderExecutionTime` to `block.timestamp`, stepping through initialized time slots found via `searchForNextInitializedTime`. If an attacker creates many orders that initialize a large number of time slots (setting bits in the bitmap), and the pool is subsequently left untouched for a period of time, the next user interaction (swap or position update) will trigger this loop. If the accumulated gas cost of processing the gap exceeds the block gas limit, the pool becomes permanently unusable (DoS).
 ### Static Signals
while (time != block.timestamp), loop processing state dependent on elapsed time
 ### Assets at Risk
liquidity pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing slippage protection in Positions withdrawal
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` allows users to burn liquidity and receive the underlying tokens. It accepts `liquidity` (amount to burn) but does not provide parameters for `minAmount0` or `minAmount1`. The amounts returned depend on the current pool tick (price) and the position's range. Without minimum output parameters, users are exposed to unlimited slippage/price shifts during the transaction, potentially receiving an unexpected ratio of tokens.
 ### Static Signals
redeem() uses spot price at execution without minCollateral guard, no minAmountOut parameter in liquidation/redemption
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

