## Verified Patterns Found: 24

## Verified Patterns Found in following Categories:

- FeeAccountingDrift
- GriefableCallbacks
- StorageCollisionOrSelectorClash
- StandardViolation
- TWAPWindowPinningOrLowLiquidity
- PrecisionDriftAccumulation
- SlippageMissingOrInsufficient
- UnboundedLoops
- AccountingInvariantViolation
- ERC20DecimalsMismatch
- FlashLoanEconomicManipulation
- FeeOnTransferAssumption
- ReserveOrPriceDesync



## Summary of Patterns

Revenue Buybacks Acceleration via Flash Loan Donation

Griefable Batch Withdrawals in FlashAccountant

Silent Sale Rate Truncation in Orders.increaseSellAmount

Accounting Drift due to Incorrect 1-wei Offset

TokenWrapper Fails to Mint Tokens on Deposit

Protocol Fee Accounting Drift via Dust Withdrawals

Missing Slippage Protection in Revenue Buybacks

TWAMM Orders Execute Without Slippage Protection

TokenWrapper Core Balance is Transient Causing Liquidity Loss

Silent integer truncation in Orders sale rate calculation

TokenWrapper ERC20 Invariant Violation and Broken Transfers

Revenue Buybacks Funds Stuck Due to Rounding

Missing Slippage Protection in Positions Withdrawal

Precision Drift in TWAMM Order Accounting

RevenueBuybacks Approves Wrong Contract Causing DoS

Oracle Default Capacity Unsafe for TWAP

Incompatibility with Fee-on-Transfer tokens due to strict accounting

TWAMM Denial of Service via Dense Order Bitmaps

Stale tickLast Leads to Incorrect MEV Fee Calculation

Unconditional Revert in beforeSwap Blocks All Swaps

Incentives Insolvency via Fee-on-Transfer Tokens

Slippage Protection Bypassed by Post-Swap Fee Application

MEV Capture Fee Avoidance and Victim Penalization

Unbounded Loop in TWAMM Virtual Order Execution

## Patterns



 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Revenue Buybacks Acceleration via Flash Loan Donation
 ### Description/Code Snippet
The `RevenueBuybacks.roll` function determines the size of the buyback order based on the contract's current token balance (`balanceOf(token)` or `address(this).balance`). It calls `ORDERS.increaseSellAmount` with the entire balance. 

An attacker can flash-loan or donate a large amount of revenue tokens to the contract and call `roll`. This forces the contract to commit the donated amount (plus any legitimate accumulated revenue) into a TWAMM order immediately. 

While TWAMM spreads execution over time, this mechanism allows an attacker to override the natural pacing of revenue deployment, forcing the protocol to sell its treasury faster or in larger chunks than intended (e.g., forcing a year's worth of revenue to be queued for sale in a single week), potentially impacting the market price of the revenue token.
 ### Static Signals
balanceOf(address(this)), increaseSellAmount(..., balance, ...)
 ### Assets at Risk
Protocol Treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: FlashAccountant.withdraw

 ### Title
Griefable Batch Withdrawals in FlashAccountant
 ### Description/Code Snippet
The `withdraw` function in `FlashAccountant.sol` (and by extension `BaseLocker` implementations like `Router`) iterates over a calldata-provided list of withdrawals and executes them. For native token withdrawals, it uses a low-level `call`. If any single recipient in the batch reverts (e.g., a malicious contract rejecting ETH), the entire transaction reverts. This allows a malicious user included in a batched withdrawal (e.g., via a multicall or aggregator) to grief other users by causing their legitimate withdrawals to fail.
 ### Static Signals
loop over calldata, external call in loop, revert on failure
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Orders.sol.increaseSellAmount

 ### Title
Silent Sale Rate Truncation in Orders.increaseSellAmount
 ### Description/Code Snippet
In `Orders.increaseSellAmount`, the `saleRate` is calculated using `computeSaleRate` (which returns a `uint256` derived from a `uint128` amount) and then explicitly cast to `uint112` without an overflow check. If a user creates an order with a large amount (up to `type(uint128).max`) and a short duration, the resulting sale rate can exceed `type(uint112).max`. The explicit cast silently truncates the high bits, potentially resulting in a sale rate of 0 or a value much lower than intended. The user pays the full `amount` to the accountant, but the TWAMM order is created with the truncated rate, causing funds to be locked and unsold (or sold too slowly), with no way to recover the full principal via cancellation if the rate was truncated to 0.
 ### Static Signals
saleRate = uint112(computeSaleRate(amount, ...)), uint128 amount
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: MEVCapture.loadCoreState

 ### Title
Accounting Drift due to Incorrect 1-wei Offset
 ### Description/Code Snippet
In `MEVCapture.sol`, the function `loadCoreState` reads the extension's saved balance from Core storage and subtracts 1 wei (`fees0 := sub(fees0, gt(fees0, 0))`). However, `Core.sol`'s `updateSavedBalances` stores the raw balance without any offset. This logic appears to mistakenly mirror the +1 offset pattern used in `FlashAccountant`'s transient storage, which does not apply to Core's persistent saved balances. As a result, `accumulatePoolFees` consistently under-reads the available fees by 1 wei, transfers `amount - 1` to LPs, and fails to clear the last 1 wei from the extension's balance. This causes permanent dust accumulation in the extension's Core balance.
 ### Static Signals
fees0 := sub(fees0, gt(fees0, 0)), fees1 := sub(fees1, gt(fees1, 0))
 ### Assets at Risk
Protocol Revenue (Fees stuck as dust)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
TokenWrapper Fails to Mint Tokens on Deposit
 ### Description/Code Snippet
In `TokenWrapper.handleForwardData`, when a user wraps tokens (amount > 0), the contract correctly updates the Core's saved balances and debt to reflect the transfer of underlying assets from the user to the Wrapper. However, it fails to mint the corresponding `TokenWrapper` ERC20 tokens to the user (via `_balanceOf` update). The user pays the underlying asset but receives no wrapped tokens in return, leading to loss of funds.
 ### Static Signals
amount > 0, CORE.updateSavedBalances, CORE.updateDebt, Missing _balanceOf update
 ### Assets at Risk
User funds sent for wrapping
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: Positions._computeWithdrawalProtocolFees

 ### Title
Protocol Fee Accounting Drift via Dust Withdrawals
 ### Description/Code Snippet
The `Positions` contract collects protocol fees on withdrawals using `_computeWithdrawalProtocolFees`. This function likely relies on `computeFee`, which performs division (e.g., `amount * fee / scale`). Because the operation rounds down, users can withdraw liquidity in small increments ('dust amounts') such that the calculated protocol fee rounds to zero. By batching many small withdrawals (e.g. via `multicall` or separate transactions on low-cost chains), a user can bypass the withdrawal protocol fee entirely.
 ### Static Signals
fee taken before scaling normalization, caller skims dust each claim via rounding
 ### Assets at Risk
protocol fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Missing Slippage Protection in Revenue Buybacks
 ### Description/Code Snippet
The `roll` function in `RevenueBuybacks.sol` creates or extends a TWAMM order to convert protocol revenue into a buy token. It uses `type(uint112).max` as the `maxSaleRate`, effectively allowing the sale rate to be determined solely by `amount / duration`. There is no parameter to enforce a minimum output amount (minBuyAmount) or a worst-case execution price for the TWAMM order. This exposes protocol revenue to potential value loss if the buyback pool has low liquidity or is manipulated during the order's duration.
 ### Static Signals
no minAmountOut, type(uint112).max maxSaleRate, market order over time
 ### Assets at Risk
treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM Orders Execute Without Slippage Protection
 ### Description/Code Snippet
In `TWAMM.sol` (used by `Orders.sol`), virtual orders are executed via `_executeVirtualOrdersFromWithinLock` calling `CORE.swap` with `MIN_SQRT_RATIO` or `MAX_SQRT_RATIO` as the limit. There is no mechanism for the order creator to specify a minimum output amount or price floor for the time-weighted execution. While intrinsic to basic TWAMM designs, this exposes users to unbounded slippage if the pool liquidity is manipulated or effectively nonexistent during the execution windows.
 ### Static Signals
_sqrtRatioLimit: MIN_SQRT_RATIO, _sqrtRatioLimit: MAX_SQRT_RATIO
 ### Assets at Risk
User funds (sold at bad prices)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: TokenWrapper.balanceOf

 ### Title
TokenWrapper Core Balance is Transient Causing Liquidity Loss
 ### Description/Code Snippet
The `TokenWrapper` contract uses a `transient` variable `coreBalance` to track the balance held by the Core contract. The `balanceOf(address(CORE))` function returns this transient value. Since transient storage is cleared at the end of every transaction, any `TokenWrapper` tokens deposited into Core (e.g., as liquidity in a pool) will have their balance reset to zero in the next transaction. This results in the complete loss of all `TokenWrapper` assets deposited into the Ekubo Protocol.
 ### Static Signals
uint256 private transient coreBalance, if (account == address(CORE)) return coreBalance
 ### Assets at Risk
User deposits of TokenWrapper
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Orders.increaseSellAmount

 ### Title
Silent integer truncation in Orders sale rate calculation
 ### Description/Code Snippet
In `Orders.sol`, the `increaseSellAmount` function calculates the new sale rate using `saleRate = uint112(computeSaleRate(amount, ...))`. The `computeSaleRate` function performs division of `amount` (uint128) by `duration` (uint32), which can result in a value up to roughly `2^128` (if duration is 1). The explicit cast to `uint112` silently truncates the most significant bits if the result exceeds `type(uint112).max`. This occurs for high-supply tokens or large order amounts with short durations. The truncation results in a drastically incorrect (lower) sale rate, causing the order to sell tokens much slower than intended, effectively locking user funds for an extended period without their consent.
 ### Static Signals
unsafe cast, downcasting without check
 ### Assets at Risk
User Order Funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
TokenWrapper ERC20 Invariant Violation and Broken Transfers
 ### Description/Code Snippet
The `TokenWrapper` contract implements `IERC20` but fails to update the `_balanceOf` mapping when tokens are wrapped via `handleForwardData`. While `totalSupply` correctly reflects the `TokenWrapper`'s holding of the underlying asset in Core, the sum of user balances remains zero because no minting logic exists. This violates the `totalSupply == sum(balances)` invariant and renders the ERC20 functionality (transfers, approvals) unusable, as users effectively receive no tokens upon wrapping.
 ### Static Signals
totalSupply != sum(balances), balance tracking references different token than actual holdings
 ### Assets at Risk
User funds (locked in wrapper without transferability)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Revenue Buybacks Funds Stuck Due to Rounding
 ### Description/Code Snippet
In `RevenueBuybacks.roll`, the `saleRate` for a new TWAMM order is calculated as `amountToSpend / duration`. This calculation is passed to `Orders.increaseSellAmount` which casts it to `uint112`. If `amountToSpend` is less than the duration (in seconds), `saleRate` rounds down to 0 due to integer division. This is likely for low-decimal tokens (e.g. USDC) or dust amounts. The funds are transferred to the `Orders` contract but the order effectively sells nothing. `RevenueBuybacks` lacks a function to withdraw principal or cancel orders (only `collect` proceeds), causing these funds to be stuck in an inactive order until administrative rescue via `take` (if the token balance accumulates again).
 ### Static Signals
mixes token amounts with 18-decimal math unscaled, rounding bias always favors caller
 ### Assets at Risk
treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing Slippage Protection in Positions Withdrawal
 ### Description/Code Snippet
The `Positions.withdraw` function allows users to withdraw liquidity from a position but does not include parameters to enforce minimum return amounts (`amount0Min`, `amount1Min`). In a concentrated liquidity AMM, the composition of a position (ratio of token0 to token1) changes with the pool price. Without a minimum amount check during withdrawal, a user's transaction can be sandwiched or executed after a significant price move, causing them to receive a different and potentially less valuable combination of tokens than anticipated.
 ### Static Signals
no minAmountOut parameter in liquidation/redemption, withdraw() converts shares to assets at current rate without minimum
 ### Assets at Risk
user liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: Orders.increaseSellAmount

 ### Title
Precision Drift in TWAMM Order Accounting
 ### Description/Code Snippet
When creating or updating TWAMM orders via `Orders.increaseSellAmount`, the sale rate is calculated as `amount / duration` (integer division). Any remainder (`amount % duration`) is truncated and effectively lost from the order's active sale logic. While the tokens remain in the contract's Core balance, they are not accounted for in `amountSold` or returned to the user, leading to a systematic accumulation of dust (precision drift) in the contract over time.
 ### Static Signals
division integer truncation, rate * duration accumulation
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: RevenueBuybacks.approveMax

 ### Title
RevenueBuybacks Approves Wrong Contract Causing DoS
 ### Description/Code Snippet
The `RevenueBuybacks` contract approves the `ORDERS` contract to spend its tokens via `approveMax`. However, when `roll` is called, `ORDERS` triggers `ACCOUNTANT.payFrom`, which causes the `FlashAccountant` (Core) to attempt a `transferFrom` on the revenue tokens. Since `RevenueBuybacks` approved `ORDERS` and not `CORE`, the transfer will fail, making the buyback functionality unusable.
 ### Static Signals
approveMax(address token), SafeTransferLib.safeApproveWithRetry(token, address(ORDERS), type(uint256).max)
 ### Assets at Risk
Protocol Revenue (Buyback functionality)
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Oracle Default Capacity Unsafe for TWAP
 ### Description/Code Snippet
The `Oracle` extension initializes with a default snapshot buffer capacity of 1 in `beforeInitializePool`. While the contract provides `expandCapacity`, the default configuration only stores the most recent snapshot. If a protocol integrates this Oracle for TWAP (Time-Weighted Average Price) reads without explicitly expanding capacity, the observations may essentially reflect the spot price (or a very recent price), making the TWAP manipulatable via `TWAPWindowPinning`. The contract does not enforce a minimum observation cardinality for TWAP queries.
 ### Static Signals
_capacity: uint32(FixedPointMathLib.max(1, c.capacity())), observationCardinality/min not enforced
 ### Assets at Risk
Dependent protocol funds (via manipulated oracle prices)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: MEVCaptureRouter._swap

 ### Title
Incompatibility with Fee-on-Transfer tokens due to strict accounting
 ### Description/Code Snippet
The `MEVCaptureRouter` uses `CORE.forward` to execute swaps via the `MEVCapture` extension, which in turn calls `CORE.swap`. The Core's `FlashAccountant` enforces strictly that debts are zeroed at the end of the transaction. For fee-on-transfer (FoT) tokens, the amount received by the Core will be less than the amount sent by the user/router, leaving a residual debt that causes the transaction to revert with `DebtsNotZeroed`. This effectively creates a Denial of Service for FoT tokens on pools using this router/extension stack.
 ### Static Signals
DebtsNotZeroed, CORE.swap, balanceOf
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: extensions/TWAMM.sol.executeVirtualOrdersFromWithinLock

 ### Title
TWAMM Denial of Service via Dense Order Bitmaps
 ### Description/Code Snippet
The `TWAMM` extension executes virtual orders by iterating through time intervals defined in `poolInitializedTimesBitmap`. The function `executeVirtualOrdersFromWithinLock` loops from the last execution time to `block.timestamp`, performing significant state updates for each initialized interval. An attacker can permissionlessly mint many TWAMM orders with end times spaced by 1 second (or the minimum tick resolution) to densely populate this bitmap. If the pool is not touched for a significant period (e.g., 24 hours), the number of intervals to process in the next interaction will result in gas consumption exceeding the block gas limit. This causes `lockAndExecuteVirtualOrders` to revert, which is called by `beforeSwap`, `beforeUpdatePosition`, and `beforeCollectFees`, effectively bricking the pool and locking all assets.
 ### Static Signals
while (time != block.timestamp), searchForNextInitializedTime
 ### Assets at Risk
liquidity, revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
Stale tickLast Leads to Incorrect MEV Fee Calculation
 ### Description/Code Snippet
`handleForwardData` updates the `tickLast` state only when `lastUpdateTime` differs from the current block timestamp (i.e., the first transaction in a block). It does not update `tickLast` to the new tick value *after* a swap completes. As a result, if multiple swaps occur in the same block, subsequent swaps calculate their MEV fee based on the tick difference relative to the start of the block (or last update) rather than the tick immediately preceding their swap. This leads to desynchronization where users pay fees for price movements caused by prior users in the same block.
 ### Static Signals
state update conditional on timestamp, fee calculated from stale baseline
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
Unconditional Revert in beforeSwap Blocks All Swaps
 ### Description/Code Snippet
The `MEVCapture` extension registers the `beforeSwap` hook but implements it to unconditionally revert with `SwapMustHappenThroughForward`. However, the intended swap execution path via `handleForwardData` calls `CORE.swap`, which automatically triggers the `beforeSwap` hook because the extension is registered for it. Consequently, `beforeSwap` reverts, causing `CORE.swap` to revert, which causes `handleForwardData` to revert. This creates a self-griefing loop that renders the pool permanently unusable for swaps.
 ### Static Signals
revert() in hook, callback triggers core function that calls callback
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Incentives Insolvency via Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `fund` function in `Incentives.sol` calculates the required funding as `minimum - currentFunded` and immediately updates the `funded` state to `minimum`. It then calls `safeTransferFrom` to pull `fundedAmount` from the caller. If the token implements a fee-on-transfer mechanism, the contract receives fewer tokens than `fundedAmount`. Since the internal accounting (`dropState.funded`) assumes the full amount was received, the contract will eventually hold insufficient tokens to cover all claims, leaving the last claimers unable to withdraw.
 ### Static Signals
sstore(id, dropState), SafeTransferLib.safeTransferFrom, no balance check
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
Slippage Protection Bypassed by Post-Swap Fee Application
 ### Description/Code Snippet
The `MEVCapture` extension applies an additional fee to the input/output amounts *after* the swap is executed by Core. While `CORE.swap` enforces the user's `sqrtRatioLimit`, it does not enforce a minimum output amount on the final result including the extension's fee. If a user interacts with the extension via `CORE.forward` without wrapping it in a strict slippage check (like the Router's), the variable `additionalFee` can erode their output beyond expected limits, specifically because the fee is dynamic and based on tick movement.
 ### Static Signals
fee added to balanceUpdate, no minAmount check inside extension
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEV Capture Fee Avoidance and Victim Penalization
 ### Description/Code Snippet
The `MEVCapture` extension calculates fees based on the absolute difference between the post-swap tick and `tickLast`, where `tickLast` is fixed at the start of the block (updated only when `lastUpdateTime != block.timestamp`). This creates a perverse incentive structure for intra-block volatility (e.g., sandwich attacks). 

1. **Victim Penalization**: If an attacker moves the price (Tick 100 -> 110), `tickLast` remains 100. A subsequent user (victim) moving the price further (110 -> 150) pays fees based on the total displacement (`|150 - 100| = 50`), essentially paying for the attacker's displacement as well.
2. **Free Reversion**: An attacker back-running the trade to restore the price (150 -> 100) pays a fee based on `|100 - 100| = 0`. 

This allows sandwich attackers to pay reduced fees on the opening leg and zero fees on the closing leg, while forcing victims to pay inflated fees.
 ### Static Signals
lastUpdateTime != currentTime, tickLast not updated per swap, fee based on start-of-block state
 ### Assets at Risk
User funds (excessive fees), Protocol Revenue (lost fees from arbitrageurs)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded Loop in TWAMM Virtual Order Execution
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function in `TWAMM.sol` iterates through time intervals from the last execution time to the current block timestamp. The loop steps are determined by `searchForNextInitializedTime`, which finds set bits in `poolInitializedTimesBitmapSlot`. An attacker can manipulate this bitmap by creating many TWAMM orders with end times spaced at short intervals (e.g., every second). If a pool has active sale rates, each iteration invokes `CORE.swap`, which is gas-intensive. If a pool is left untouched for a period (e.g., 1 hour) while populated with 1-second interval orders, the next interaction will trigger thousands of swaps, exceeding the block gas limit and causing a Denial of Service (DoS) for the pool.
 ### Static Signals
while (time != block.timestamp), searchForNextInitializedTime
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

