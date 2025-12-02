## Verified Patterns Found: 19

## Verified Patterns Found in following Categories:

- PricePrecisionOrRoundingError
- GriefableCallbacks
- StandardViolation
- FlashLoanEconomicManipulation
- AccountingInvariantViolation
- SlippageMissingOrInsufficient
- MaturityorGatingByPass
- OracleUsingDEXorTWAP
- UnboundedLoops
- TimelockEdgeCase
- AccessControlOrAuthByPass
- FeeOnTransferAssumption



## Summary of Patterns

Strict configuration check in withdrawAndRoll causes stuck protocol fees

RevenueBuybacks sale rate manipulable via token donation

Unbounded Loop in TWAMM Virtual Order Execution

Strict configuration check in withdrawAndRoll prevents fee withdrawal (Code/Comment Mismatch)

Public execution of withdrawAndRoll allows unauthorized timing of buybacks

Unconditional revert in MEVCapture `beforeSwap` hook bricks swaps

Pool Bricking via TWAMM Sale Rate Overflow

Missing Slippage/Minimum Output Parameters in Position Withdrawal

Silent sale rate truncation in Orders.increaseSellAmount via RevenueBuybacks

TWAMM Virtual Order Execution DoS via Checkpoint Stuffing

Fee-On-Transfer Token Incompatibility in Incentives Funding

TokenWrapper functionality is broken and enables fake accounting

Partial Configuration DOS on Revenue Collection

Rounding to zero sale rate permanently locks revenue dust

Gas limit DoS on fee withdrawal due to coupled token rolling

MEVCapture extension causes permanent DoS on its pools

Oracle extension susceptible to same-block spot price manipulation

DoS on fee withdrawal for mixed-configuration token pairs

Missing slippage protection in RevenueBuybacks roll function

## Patterns



 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: PositionsOwner.withdrawAndRoll

 ### Title
Strict configuration check in withdrawAndRoll causes stuck protocol fees
 ### Description/Code Snippet
The `withdrawAndRoll` function enforces that *both* tokens in a pair must be configured for buybacks (`minOrderDuration > 0`) in the `RevenueBuybacks` contract. Ekubo stores protocol fees keyed by the token pair. If a pair consists of one configured token and one unconfigured token, the check `s0.minOrderDuration() == 0 || s1.minOrderDuration() == 0` causes the transaction to revert. This prevents the withdrawal of the accumulated fees for the configured token from that specific pair, locking the funds in the `Positions` contract until the unconfigured token is configured or ownership is transferred.
 ### Static Signals
minOrderDuration() == 0, revert RevenueTokenNotConfigured()
 ### Assets at Risk
protocol fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
RevenueBuybacks sale rate manipulable via token donation
 ### Description/Code Snippet
The `roll` function in `RevenueBuybacks.sol` determines the new `saleRate` for TWAMM orders based on the contract's current token balance (`amountToSpend`). An attacker can donate a large amount of `token` to the contract and then call `roll` (which is public) to drastically increase the `saleRate`. This creates immense sell pressure on the revenue token in the `BUY_TOKEN` pool, potentially manipulating the market price for profit in a derivative position.
 ### Static Signals
balanceOf(token, address(this)), increaseSellAmount
 ### Assets at Risk
treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded Loop in TWAMM Virtual Order Execution
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function in `TWAMM.sol` contains a `while` loop that iterates from the last execution time until `block.timestamp`. It calls `searchForNextInitializedTime` to find the next time bucket with active orders. An attacker can create a large number of orders with start/end times spaced by small intervals (e.g., every second or minimum tick), populating the time bitmap densely. When a victim subsequently interacts with the pool (via swap or position update), the system is forced to iterate through all these time intervals, executing heavy logic (including `CORE.swap`) for each. This can consume excessive gas, causing the victim's transaction to run out of gas (DoS) or forcing them to pay exorbitant fees to process the attacker's backlog.
 ### Static Signals
while (time != block.timestamp), complex logic inside loop
 ### Assets at Risk
gas
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: PositionsOwner.withdrawAndRoll

 ### Title
Strict configuration check in withdrawAndRoll prevents fee withdrawal (Code/Comment Mismatch)
 ### Description/Code Snippet
The `withdrawAndRoll` function in `PositionsOwner.sol` contains a check `if (s0.minOrderDuration() == 0 || s1.minOrderDuration() == 0)`. The logic OR operator requires *both* tokens in a pair to be configured in `RevenueBuybacks` to proceed. However, the comment above states: `// Check if at least one token is configured for buybacks`. This discrepancy indicates a logic error. If a pool consists of one configured token (e.g. WETH) and one unconfigured/garbage token, the check fails, reverting the transaction. This effectively locks the protocol fees for the configured token (WETH) in that pool, causing a Denial of Service for fee collection.
 ### Static Signals
comment 'Check if at least one' vs code '||' (requires both)
 ### Assets at Risk
Protocol Fees (revenue)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: PositionsOwner.withdrawAndRoll

 ### Title
Public execution of withdrawAndRoll allows unauthorized timing of buybacks
 ### Description/Code Snippet
The `withdrawAndRoll` function in `PositionsOwner.sol` is `external` and has no access control modifiers. It calls `POSITIONS.withdrawProtocolFees` (sending fees to the `BUYBACKS` contract) and then `BUYBACKS.roll`. While the destination of funds is fixed/trusted, allowing any user to trigger fee withdrawal and buyback execution at arbitrary times could allow MEV searchers or griefers to manipulate the timing of these operations. For example, triggering a buyback (creating a TWAMM order) right before or after specific market events to influence the TWAMM execution parameters or gas usage.
 ### Static Signals
no onlyOwner, external function, state changing call to POSITIONS, state changing call to BUYBACKS
 ### Assets at Risk
Protocol Fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
Unconditional revert in MEVCapture `beforeSwap` hook bricks swaps
 ### Description/Code Snippet
The `MEVCapture` extension registers the `beforeSwap` hook as true in `mevCaptureCallPoints` but implements the hook to unconditionally revert with `SwapMustHappenThroughForward()`. When a user swaps via `MEVCaptureRouter`, the router forwards the call to `MEVCapture`, which internally calls `CORE.swap`. `CORE.swap` then triggers the `beforeSwap` hook on the `MEVCapture` extension. Since the hook unconditionally reverts and `Core` does not exempt the extension itself from hooks, the swap will always fail. This effectively renders any pool using the `MEVCapture` extension unusable.
 ### Static Signals
revert SwapMustHappenThroughForward(), getCallPoints returning true for beforeSwap
 ### Assets at Risk
Pool Availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Pool Bricking via TWAMM Sale Rate Overflow
 ### Description/Code Snippet
In `TWAMM._executeVirtualOrdersFromWithinLock`, the contract calculates the token amount to swap/distribute as `amount = computeAmountFromSaleRate(...)`, which is roughly `saleRate * timeElapsed`. This amount is then passed to `CORE.updateSavedBalances`, which reverts if the balance delta overflows `int128`. An attacker can manipulate the `saleRate` to be very high (up to `type(uint112).max`) by creating orders with large amounts (potentially using flash loans or high-supply tokens). If the pool remains inactive for a sufficient period (e.g., days or weeks depending on the rate), the `amount` calculation will exceed `type(uint128).max` or the `updateSavedBalances` call will overflow `int128`. This causes `executeVirtualOrders` to strictly revert, rendering the pool unusable.
 ### Static Signals
int128(uint128(amount)), CORE.updateSavedBalances
 ### Assets at Risk
Liquidity Pool
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing Slippage/Minimum Output Parameters in Position Withdrawal
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` allows users to remove liquidity from a position but lacks `amount0Min` and `amount1Min` parameters to enforce minimum output amounts. While liquidity removal determines amounts based on the current pool price (sqrtRatio), price manipulation or high volatility before the transaction executes can alter the ratio of assets returned significantly. Unlike standard AMM patterns (e.g., Uniswap V3 NonfungiblePositionManager), users cannot define acceptable bounds for the assets retrieved, exposing them to unintended portfolio rebalancing or loss of value during withdrawal.
 ### Static Signals
no minAmount parameters, liquidity burn without output bounds
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Orders.increaseSellAmount

 ### Title
Silent sale rate truncation in Orders.increaseSellAmount via RevenueBuybacks
 ### Description/Code Snippet
In `RevenueBuybacks.roll`, the contract calls `ORDERS.increaseSellAmount` passing `type(uint112).max` as the `maxSaleRate`. Inside `Orders.sol`, the sale rate is computed using `computeSaleRate` (amount / duration) and then explicitly cast to `uint112`: `saleRate = uint112(computeSaleRate(...))`. For tokens with very high supply (high decimals) or short durations, the calculated rate can exceed `uint112`. The cast silently truncates the high bits, and because `maxSaleRate` is `type(uint112).max`, the truncated (incorrect) rate is accepted. This results in the TWAMM order executing at a vastly different (likely lower) rate than intended, delaying revenue buybacks indefinitely.
 ### Static Signals
saleRate = uint112(computeSaleRate(...)), saleRate > maxSaleRate
 ### Assets at Risk
Protocol Revenue (delayed execution)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TimelockEdgeCase

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM Virtual Order Execution DoS via Checkpoint Stuffing
 ### Description/Code Snippet
The `TWAMM._executeVirtualOrdersFromWithinLock` function iterates through all initialized time checkpoints between the last execution time and `block.timestamp` using a `while` loop. There is no limit on the number of checkpoints processed in a single transaction. An attacker can cheaply create many orders with distinct start and end times (e.g., at 1-second intervals) via `Orders.mintAndIncreaseSellAmount`. This populates the `poolInitializedTimesBitmap` with thousands of checkpoints. When a user subsequently interacts with the pool (e.g., via `swap` or `updatePosition`), the `before*` hooks trigger `executeVirtualOrders`, which iterates through all these checkpoints. If the gas cost of the loop exceeds the block gas limit, the transaction reverts. This permanently bricks the pool, as the execution must catch up to the current time to proceed.
 ### Static Signals
while (time != block.timestamp), searchForNextInitializedTime
 ### Assets at Risk
Liquidity Pool
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Fee-On-Transfer Token Incompatibility in Incentives Funding
 ### Description/Code Snippet
The `fund` function in `Incentives.sol` calculates `fundedAmount` based on the difference between the requested minimum and current funded amount, then transfers this exact amount using `safeTransferFrom`. It assumes the contract receives the full `fundedAmount`. However, for fee-on-transfer tokens, the actual amount received by the contract is less than `fundedAmount`. The `dropState` is updated as if the full amount was received (`setFunded`). Subsequent claims (`claim`) rely on `dropState` checks and `safeTransfer` of claim amounts. Since the contract holds less than the accounting tracks, the last users attempting to claim will face reverts due to insufficient balance, effectively locking their rewards.
 ### Static Signals
safeTransferFrom without balance check, accounting updates based on input amount
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
TokenWrapper functionality is broken and enables fake accounting
 ### Description/Code Snippet
The TokenWrapper contract fails to update the user's internal ERC20 balance (`_balanceOf`) in `handleForwardData`, rendering the token untransferable and unusable. Additionally, the function manually cancels the debt created by `updateSavedBalances` using `updateDebt` without requiring any actual token transfer. This allows the contract to mint `SavedBalance` in Core backed by nothing (creating an insolvent position) whenever any user calls `forward`, breaking accounting invariants.
 ### Static Signals
_balanceOf not updated in minting logic, updateDebt cancels debt from updateSavedBalances, no transfer of underlying tokens
 ### Assets at Risk
User funds forwarded to wrapper, Core accounting integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: PositionsOwner.withdrawAndRoll

 ### Title
Partial Configuration DOS on Revenue Collection
 ### Description/Code Snippet
The `withdrawAndRoll` function strictly requires both tokens in a pair to be configured in `RevenueBuybacks`. If a pool pairs a configured revenue token (e.g. USDC) with an unconfigured one, fees accrued in that pool (including the USDC fees) cannot be collected via this standard function, effectively locking the revenue until governance manually transfers ownership to intervene.
 ### Static Signals
s0.minOrderDuration() == 0 || s1.minOrderDuration() == 0, revert RevenueTokenNotConfigured
 ### Assets at Risk
Protocol Fees/Revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Rounding to zero sale rate permanently locks revenue dust
 ### Description/Code Snippet
In `RevenueBuybacks.roll`, the sale rate is calculated as `amount / duration` (via `Orders.increaseSellAmount` logic). If `amount < duration`, integer division rounds the sale rate to 0. The `Orders` contract accepts this, transferring the revenue tokens to itself but setting a sale rate of 0/sec. Since `RevenueBuybacks` lacks functions to cancel orders, decrease sale rates, or withdraw principal (only proceeds), these small amounts of revenue become permanently locked within the `Orders` contract.
 ### Static Signals
divide before multiply, no zero check on result
 ### Assets at Risk
revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: PositionsOwner.withdrawAndRoll

 ### Title
Gas limit DoS on fee withdrawal due to coupled token rolling
 ### Description/Code Snippet
The `withdrawAndRoll` function calls `BUYBACKS.roll` for both `token0` and `token1` sequentially. The `roll` function triggers the TWAMM extension to update orders, which executes a loop over unvisited time intervals (`while (time != block.timestamp)` in `TWAMM.sol`) to execute virtual orders. If one token in the pair belongs to a pool that has not been updated for a long time, the gas cost to 'catch up' that pool may be excessive or exceed the block limit. Because the withdrawals are coupled, a 'zombie' pool for `token1` will revert the transaction and block the withdrawal of fees for `token0`, even if `token0` is active and cheap to process.
 ### Static Signals
BUYBACKS.roll(token0), BUYBACKS.roll(token1), sequential external calls
 ### Assets at Risk
protocol fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture extension causes permanent DoS on its pools
 ### Description/Code Snippet
The `MEVCapture` extension enforces that swaps must happen via forward by reverting in `beforeSwap`. However, the extension itself calls `CORE.swap` inside `handleForwardData` to execute the swap. Since `CORE.swap` unconditionally calls the `beforeSwap` hook of the registered extension (which is `MEVCapture`), the extension's own swap attempts will trigger the revert, causing a Denial of Service for all pools using this extension.
 ### Static Signals
revert unconditionally in beforeSwap, CORE.swap calls hooks without checking caller
 ### Assets at Risk
Pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: Oracle.extrapolateSnapshot

 ### Title
Oracle extension susceptible to same-block spot price manipulation
 ### Description/Code Snippet
The `Oracle` extension's `extrapolateSnapshot` function uses the current `CORE.poolState()` to interpolate values when the target time is after the last stored snapshot. Since the stored snapshot is only written by the first interaction in a block, subsequent transactions in the same block update `CORE.poolState()` without updating the snapshot. An attacker can manipulate the pool's spot price and then call `extrapolateSnapshot` within the same transaction to obtain a manipulated TWAP value, effectively bypassing the time-weighting protection for the current block.
 ### Static Signals
uses CORE.poolState() for current data, snapshot only written once per block
 ### Assets at Risk
Protocols relying on this Oracle
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: PositionsOwner.withdrawAndRoll

 ### Title
DoS on fee withdrawal for mixed-configuration token pairs
 ### Description/Code Snippet
The `withdrawAndRoll` function strictly enforces that both `token0` and `token1` must be configured in the `BUYBACKS` contract (non-zero `minOrderDuration`). In permissionless pools where a major asset (e.g., WETH) is paired with a long-tail or unconfigured asset, this check fails, causing the transaction to revert. Consequently, protocol revenue accumulated in the major asset becomes locked and cannot be withdrawn using this function until the minor asset is also configured.
 ### Static Signals
s0.minOrderDuration() == 0 || s1.minOrderDuration() == 0, revert RevenueTokenNotConfigured()
 ### Assets at Risk
revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Missing slippage protection in RevenueBuybacks roll function
 ### Description/Code Snippet
The `roll` function in `RevenueBuybacks.sol` creates or updates TWAMM orders by calling `ORDERS.increaseSellAmount` with `type(uint112).max` as the `maxSaleRate` argument. This explicitly bypasses the safety check in `Orders.sol` that prevents the sale rate (amount sold per second) from exceeding a threshold. Since `saleRate` is derived from `amount / duration`, and `amount` is the entire token balance of the contract (accumulated fees) while `duration` is constrained by configuration, a large accumulated balance can result in an excessively high sale rate. This effectively dumps protocol revenue onto the market with high slippage/price impact. Because `PositionsOwner.withdrawAndRoll` is public, an attacker can trigger this function when fee accumulation is high to force a bad execution for the protocol.
 ### Static Signals
increaseSellAmount(..., type(uint112).max), maxSaleRate parameter disabled
 ### Assets at Risk
Protocol fees, Revenue tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

