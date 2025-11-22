## Verified Patterns Found: 15

## Verified Patterns Found in following Categories:

- OracleUsingDEXorTWAP
- FlashLoanEconomicManipulation
- SlippageMissingOrInsufficient
- TWAPWindowPinningOrLowLiquidity
- FeeOnTransferAssumption
- UnsafeRecipient
- UnboundedLoops



## Summary of Patterns

TWAMM Orders Subject to Low Liquidity Manipulation

Positions.withdraw lacks slippage protection (minAmount parameters)

Unsafe Recipient in Router Swap

Missing Deadline Check in Router Swaps

FlashAccountant withdrawals susceptible to sender-side fee-on-transfer tokens

TWAMM virtual orders execute at manipulable spot price

Missing Deadline Checks in Router and Positions

Positions.deposit assumes 1:1 Transfer for Fee-on-Transfer Tokens

MEV Capture Fee Avoidance on Backrun Trades

DoS via Unbounded Loop in TWAMM Virtual Order Execution

Sandwichable TWAMM Virtual Order Execution

Revenue Buybacks Execute Without Slippage Protection

Unsafe Recipient in Position Withdrawal

Unbounded Loop in TWAMM Virtual Order Execution

Missing Slippage Protection in Position Withdrawal

## Patterns



 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM Orders Subject to Low Liquidity Manipulation
 ### Description/Code Snippet
The `TWAMM` extension executes virtual orders by pushing the pool price based on the sale rate and time elapsed. It does not enforce a minimum liquidity threshold in the pool. If the pool has very low liquidity, the virtual orders (triggered by `RevenueBuybacks` or users) will incur extreme price impact, effectively selling tokens for dust.
 ### Static Signals
uses single low-liquidity pair without liquidity floor, execution based on pool state without min liquidity check
 ### Assets at Risk
user funds, revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.sol.withdraw

 ### Title
Positions.withdraw lacks slippage protection (minAmount parameters)
 ### Description/Code Snippet
The `Positions.withdraw` function allows users to burn liquidity and receive underlying tokens, but it does not accept `amount0Min` or `amount1Min` parameters. The return values `amount0` and `amount1` are calculated based on the current pool tick/price. If the price changes unfavorably (e.g., due to a sandwich attack or volatility) between transaction signing and execution, the user may receive significantly fewer tokens or a different ratio of assets than expected. Standard LP management contracts (like Uniswap V3's NonfungiblePositionManager) include minimum amount checks to prevent this.
 ### Static Signals
liquidity input but no minAmount output params, returns (amount0, amount1) without require checks
 ### Assets at Risk
User liquidity/tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Router.handleLockData

 ### Title
Unsafe Recipient in Router Swap
 ### Description/Code Snippet
The `Router` and `MEVCaptureRouter` do not validate that the `recipient` address provided in swap data is non-zero. Since the `FlashAccountant.withdraw` function uses a low-level call or transfer to the recipient, sending tokens to `address(0)` will result in permanent loss of funds for the user.
 ### Static Signals
no zero-address guard, recipient parameter passed to withdraw
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router._swap

 ### Title
Missing Deadline Check in Router Swaps
 ### Description/Code Snippet
The Router contract's swap functions (single and multihop) accept `SwapParameters` but lack a user-defined `deadline` timestamp. Transactions stuck in the mempool can be executed at a much later time than intended, potentially exposing users to unfavorable market conditions or MEV exploitation even if the slippage check holds.
 ### Static Signals
deadline omitted, SwapParameters struct lacks deadline
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: FlashAccountant.withdraw

 ### Title
FlashAccountant withdrawals susceptible to sender-side fee-on-transfer tokens
 ### Description/Code Snippet
The `FlashAccountant.withdraw` function executes ERC20 transfers to withdraw tokens to users. It assumes that the token transfer logic only reduces the `FlashAccountant`'s (Core's) balance by the `amount` specified. However, certain non-standard fee-on-transfer tokens implement fees by burning an additional amount from the sender's balance (e.g., transfer 100, sender balance decreases by 101) rather than deducting from the recipient amount.

Since `FlashAccountant` does not verify the actual balance change of `address(this)` after the transfer, such tokens will cause the Core contract to leak value silently. Over time, the physical balance of tokens in Core will drop below the sum of all `savedBalances`, leading to insolvency for that token.
 ### Static Signals
call(gas(), token, ...), no balanceBefore/After check
 ### Assets at Risk
Core assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM virtual orders execute at manipulable spot price
 ### Description/Code Snippet
The `TWAMM` extension executes accumulated virtual orders using the pool's spot price at the moment of execution (via `_executeVirtualOrdersFromWithinLock`). Although execution is rate-limited to once per block, it uses the `sqrtRatio` from the pool state at the beginning of the interaction. An attacker can manipulate the pool price in block `N` and then trigger the TWAMM execution in block `N+1`. The virtual orders will then execute against the manipulated price, causing significant loss to TWAMM order holders. This pattern fits `OracleUsingDEXorTWAP` as it relies on a spot-like price (the price at the start of the first interaction in a block) for settling significant value without a TWAP or deviation check.
 ### Static Signals
uses spot price (sqrtRatio) for trade execution, no TWAP check, delayed execution sensitive to price manipulation
 ### Assets at Risk
TWAMM order funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router.swap

 ### Title
Missing Deadline Checks in Router and Positions
 ### Description/Code Snippet
The `Router.swap`, `Router.multihopSwap`, `Positions.deposit`, and `Positions.withdraw` functions (and their variants) do not accept a timestamp deadline parameter. Without a deadline, transaction signatures remain valid indefinitely, allowing malicious validators or MEV bots to hold pending transactions and execute them at a later time when market conditions are unfavorable to the user (e.g., executing a swap after a significant price movement that still satisfies the `calculatedAmountThreshold` but represents a loss compared to the time of submission).
 ### Static Signals
deadline omitted or far future, router calls with no deadline
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Positions.sol.deposit

 ### Title
Positions.deposit assumes 1:1 Transfer for Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `Positions.deposit` function calculates the required token amounts based on the requested liquidity and calls `ACCOUNTANT.payTwoFrom` to transfer tokens from the user. This helper likely performs a `transferFrom` and updates the debt in the `FlashAccountant` by the requested amount, assuming a 1:1 transfer. If fee-on-transfer tokens are used, Core receives less than the recorded debt/liquidity implies. Since `Positions` does not use the `startPayments`/`completePayments` pattern (which checks balance deltas) for this operation, it subjects the protocol to insolvency or accounting discrepancies with FOT tokens.
 ### Static Signals
uses payTwoFrom/transferFrom instead of balance delta checks, liquidity calculated from input amount
 ### Assets at Risk
Protocol solvency, User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.sol.handleForwardData

 ### Title
MEV Capture Fee Avoidance on Backrun Trades
 ### Description/Code Snippet
The `MEVCapture` extension calculates fees based on the absolute difference between the post-swap tick and `tickLast` (`abs(stateAfter.tick() - tickLast)`). `tickLast` is snapshot of the tick at the beginning of the block (or last timestamp update) and is **not** updated between swaps within the same block. In a sandwich attack or atomic arbitrage, the front-running trade displaces the price (incurring a fee), but the back-running trade returns the price to the original `tickLast`. Since the post-swap tick of the backrun equals `tickLast`, the displacement is zero, resulting in zero MEV capture fees for the closing leg of the strategy. This allows searchers to bypass the intended tax on the second half of their atomic transactions.
 ### Static Signals
FixedPointMathLib.abs(stateAfter.tick() - tickLast), lastUpdateTime != currentTime
 ### Assets at Risk
Protocol Revenue (MEV fees)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM.sol._executeVirtualOrdersFromWithinLock

 ### Title
DoS via Unbounded Loop in TWAMM Virtual Order Execution
 ### Description/Code Snippet
The `TWAMM._executeVirtualOrdersFromWithinLock` function contains a `while (time != block.timestamp)` loop that iterates through initialized time buckets to execute virtual swaps. An attacker can create many orders with distinct end times (e.g., every minute) to populate the `poolInitializedTimesBitmap` with many set bits. When `_executeVirtualOrdersFromWithinLock` is triggered (via `beforeSwap` or `beforeUpdatePosition`), it must iterate through all these time buckets, performing a gas-intensive `CORE.swap` in each iteration. If the gas cost exceeds the block limit, the pool becomes unusable (DoS).
 ### Static Signals
while loop depending on block.timestamp, complex operations (CORE.swap) inside loop, loop bound controlled by user input (order end times)
 ### Assets at Risk
Pool availability (DoS)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Sandwichable TWAMM Virtual Order Execution
 ### Description/Code Snippet
The `TWAMM` extension executes accumulated virtual orders using the current spot price (`corePoolState.sqrtRatio()`) inside `_executeVirtualOrdersFromWithinLock`. Since this execution can be triggered permissionlessly via `lockAndExecuteVirtualOrders` (or automatically via hooks), an attacker can use a flash loan or large swap to manipulate the pool's spot price, trigger the TWAMM execution to fill orders at the manipulated price, and then swap back. This allows the attacker to sandwich the entire accumulated TWAMM volume for a profit at the expense of TWAMM order owners.
 ### Static Signals
uses spot price for trade execution, accumulates volume over time but executes at instantaneous price, permissionless trigger
 ### Assets at Risk
TWAMM order funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Revenue Buybacks Execute Without Slippage Protection
 ### Description/Code Snippet
The `RevenueBuybacks.roll` function triggers the creation or extension of TWAMM orders using protocol revenue (`amountToSpend`). It calls `ORDERS.increaseSellAmount` with `maxSaleRate` set to `type(uint112).max`, effectively authorizing the sale at any rate required to clear the amount over the duration. If the `minOrderDuration` is configured to be short or if the pool has low liquidity, this can result in executing sales with unlimited slippage, allowing MEV bots to sandwich the buyback or arbitrage the price impact, causing loss of protocol revenue.
 ### Static Signals
payout calculated at execution time without minimum bound, price fetched at execution without user-specified floor
 ### Assets at Risk
revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Unsafe Recipient in Position Withdrawal
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` passes the `recipient` address to `ACCOUNTANT.withdrawTwo`, which eventually calls `FlashAccountant.withdraw`. Neither contract validates that the `recipient` address is non-zero. If a user accidentally passes `address(0)` as the recipient, the withdrawn tokens will be sent to the zero address and permanently lost.
 ### Static Signals
no zero-address check for recipient
 ### Assets at Risk
withdrawn tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded Loop in TWAMM Virtual Order Execution
 ### Description/Code Snippet
The `TWAMM` extension executes virtual orders by iterating through time intervals from the last execution time up to the current block timestamp. This loop runs inside `_executeVirtualOrdersFromWithinLock`, which is triggered by swaps and position updates. An attacker can create orders that initialize a large number of future time intervals (bins). If the pool is left untouched for a significant period, the number of initialized intervals to process may grow large enough that executing the loop exceeds the block gas limit, effectively causing a Denial of Service (DoS) for the pool.
 ### Static Signals
loop over time intervals, iteration depends on elapsed time
 ### Assets at Risk
Pool Availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing Slippage Protection in Position Withdrawal
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` (and `Positions.sol`) allows users to remove liquidity by specifying the position ID and liquidity amount, but it does not accept minimum output parameters (`amount0Min`, `amount1Min`). The function calculates the amounts based on the current pool reserves/price. If the pool price is manipulated or volatile at the time of withdrawal, the user may receive an unfavorable ratio of tokens (slippage) without the transaction reverting.
 ### Static Signals
missing minAmount parameters, amounts calculated at execution time without bounds
 ### Assets at Risk
user liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

