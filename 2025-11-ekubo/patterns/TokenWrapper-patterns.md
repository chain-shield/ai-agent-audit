## Verified Patterns Found: 24

## Verified Patterns Found in following Categories:

- AccountingInvariantViolation
- FeeAccountingDrift
- FeeOnTransferAssumption
- GriefableCallbacks
- SlippageMissingOrInsufficient
- TWAPWindowPinningOrLowLiquidity
- UnsafeRecipient
- UnboundedLoops
- StandardViolation
- Reentrancy
- PricePrecisionOrRoundingError
- FlashLoanEconomicManipulation



## Summary of Patterns

Transient Storage for ERC20 Balances Violates Standard

Missing Slippage Protection in TWAMM Orders

TokenWrapper Transfers to Core Result in Permanent Fund Loss

Router uses contract ETH balance to cover user debts

Missing slippage protection in BasePositions withdraw function

TWAMM orders susceptible to spot price manipulation on execution

Unbounded loops in TWAMM virtual order execution can lead to Pool DoS

RevenueBuybacks creates TWAMM orders without minimum output guarantees

Swap Output Clamping Leads to Accounting Invariant Violation

Protocol fee leakage via rounding in Positions withdrawal

Incompatibility with Fee-on-Transfer tokens in Positions and Orders

Oracle extrapolation can return stale tick data within the same block

Public swap function defaults to zero slippage protection

Fee-on-Transfer tokens cause insolvency in Incentives contract

Oracle extension defaults to unsafe low capacity (1 snapshot)

Unsafe recipient in FlashAccountant withdrawal

Missing slippage protection in Position withdrawals

TWAMM Economic Manipulation via Sandwich Attack

Rounding error in Orders.increaseSellAmount leads to stuck ETH

State Corruption via Reentrancy in Core.swap

MEVCapture Extension Bricks Swaps via Always-Reverting Callback

Missing slippage protection in Positions.withdraw

MEV Capture Fee Avoidance and Victim Griefing via Stale State

Unbounded Loop in TWAMM Virtual Order Execution enables DoS

## Patterns



 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.transfer

 ### Title
Transient Storage for ERC20 Balances Violates Standard
 ### Description/Code Snippet
The `TokenWrapper` contract uses `transient` storage for the `CORE` address's balance (`coreBalance`). This means any tokens transferred to the `CORE` address (e.g., via `transfer`) will persist only for the duration of the transaction and vanish afterwards. This violates the expected persistence of ERC20 balances and puts user funds at risk if they interact with `CORE` outside of the intended atomic flows.
 ### Static Signals
transient coreBalance, balanceOf returns transient
 ### Assets at Risk
User tokens transferred to CORE
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Orders.mintAndIncreaseSellAmount

 ### Title
Missing Slippage Protection in TWAMM Orders
 ### Description/Code Snippet
The `Orders` contract allows users to create and fund TWAMM orders via `mintAndIncreaseSellAmount` and `increaseSellAmount`. While users can specify `maxSaleRate`, there is no parameter to define a minimum total output amount (`amountOutMin`) or a minimum price floor for the execution. This exposes users to unlimited slippage if the pool price crashes or is manipulated over the duration of the order.
 ### Static Signals
amountOutMin=0 or missing, no price limit parameter
 ### Assets at Risk
User tokens deposited into Orders
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: TokenWrapper.transfer

 ### Title
TokenWrapper Transfers to Core Result in Permanent Fund Loss
 ### Description/Code Snippet
The `TokenWrapper` contract implements a custom `balanceOf` and `transfer` logic to support the 'Till' accounting pattern, where the `Core` address balance is tracked in transient storage (`coreBalance`). 

If a user transfers `TokenWrapper` tokens to the `Core` address using `transfer` or `transferFrom` (instead of interacting via `FlashAccountant.payFrom` or `lock`), the `coreBalance` increases transiently. However, at the end of the transaction, the transient storage is cleared, and the `coreBalance` resets to 0. 

The tokens are effectively burned from the user's `_balanceOf` ledger, but the `TokenWrapper`'s `totalSupply` (which tracks the underlying assets locked in Core) is not reduced. The underlying assets backing these tokens become permanently locked in Core with no corresponding circulating wrapper tokens. This violates standard ERC20 behavior and accounting invariants.
 ### Static Signals
uint256 private transient coreBalance, if (to == address(CORE)) { coreBalance += amount; }
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Router.handleLockData

 ### Title
Router uses contract ETH balance to cover user debts
 ### Description/Code Snippet
In `Router.handleLockData`, if `poolKey.token0` is the native token and the amount owed by the user (`balanceUpdate.delta0()`) exceeds the `value` sent by the user, the Router calculates a negative `valueDifference` and transfers ETH from itself to the Accountant using `safeTransferETH`. This allows an attacker to drain any ETH held by the Router contract (e.g. from `multicall` residue or accidental transfers) by executing a swap where they owe more ETH than they send.
 ### Static Signals
balance tracking references different token than actual holdings
 ### Assets at Risk
Router ETH balance
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.sol.withdraw

 ### Title
Missing slippage protection in BasePositions withdraw function
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` allows users to burn liquidity and collect tokens based on the current pool tick/price. However, it lacks parameters for `minAmount0` or `minAmount1` (slippage bounds). If the pool price is manipulated (e.g., via a sandwich attack) right before the withdrawal transaction is executed, the user may receive a significantly different ratio of tokens than expected, resulting in value loss. Unlike `deposit`, which has `minLiquidity`, `withdraw` offers no execution guarantees.
 ### Static Signals
amountOutMin=0, no minAmount parameters
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM orders susceptible to spot price manipulation on execution
 ### Description/Code Snippet
The `TWAMM` extension executes virtual orders that have accumulated since the last update in a single batch (or loop) starting at the *current* spot price (via `_executeVirtualOrdersFromWithinLock` calling `CORE.swap`). If a pool has low liquidity or is infrequently updated, a significant volume of orders may accumulate. An attacker can manipulate the spot price of the pool immediately before triggering the TWAMM execution (e.g., via a sandwich attack or simply trading before the update in the same block). The accumulated TWAMM orders will then execute at this manipulated price, transferring value to the attacker. This fits the pattern of time-weighted orders being pinned to a manipulatable spot price due to discrete execution windows.
 ### Static Signals
execution of time-weighted orders at spot price, accumulated volume executed in single transaction
 ### Assets at Risk
user funds, TWAMM order value
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM.sol._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded loops in TWAMM virtual order execution can lead to Pool DoS
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function in `TWAMM.sol` iterates through time intervals from the last execution time to the current block timestamp using `searchForNextInitializedTime`. If an attacker initializes a large number of consecutive time slots (e.g., by placing dust orders expiring at 1-second intervals), the loop in `_executeVirtualOrdersFromWithinLock` will perform a large number of iterations. Since each iteration may involve expensive calls to `CORE.swap`, the gas cost can easily exceed the block gas limit. This would permanently brick the pool, as the state cannot be updated without processing the pending time intervals, and no mechanism exists to paginate or skip this processing.
 ### Static Signals
while (time != block.timestamp), searchForNextInitializedTime
 ### Assets at Risk
Pool functionality (DoS)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
RevenueBuybacks creates TWAMM orders without minimum output guarantees
 ### Description/Code Snippet
The `RevenueBuybacks` contract's `roll` function creates or updates a TWAMM order to sell revenue tokens for `BUY_TOKEN`. It calculates the sale rate based on the current balance and duration but does not verify the pool's liquidity or enforce a minimum amount of `BUY_TOKEN` to be received (`minOutput`). If the pool is illiquid or manipulated, the protocol revenue could be sold for negligible value.
 ### Static Signals
payout calculated at execution time without minimum bound, amountOutMin=0 or missing
 ### Assets at Risk
Protocol Revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Core.swap_6269342730

 ### Title
Swap Output Clamping Leads to Accounting Invariant Violation
 ### Description/Code Snippet
In `Core.swap_6269342730`, the `calculatedAmountDelta` (the amount the pool owes the user or vice versa) is clamped to `type(int128).min` / `max`. If a swap involves an amount exceeding `2^127` (approx `1.7e38` units), the `PoolBalanceUpdate` and subsequent `FlashAccountant` debt update will under-report the actual amount exchanged. A user could effectively swap `2^128` tokens but only incur `2^127` debt, draining the pool.
 ### Static Signals
FixedPointMathLib.max(type(int128).min, calculatedAmount), SafeCastLib.toInt128
 ### Assets at Risk
Liquidity Providers
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Protocol fee leakage via rounding in Positions withdrawal
 ### Description/Code Snippet
In `BasePositions.sol`, the `withdraw` function calculates protocol fees using `_computeWithdrawalProtocolFees`, which applies `computeFee` (a fraction) to the withdrawn amounts. For small withdrawal amounts, this fee can round down to zero. Attackers can split their withdrawals into many small transactions to avoid paying the protocol withdrawal fee.
 ### Static Signals
fee taken before scaling normalization, caller skims dust each claim via rounding
 ### Assets at Risk
Protocol Fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Positions.deposit

 ### Title
Incompatibility with Fee-on-Transfer tokens in Positions and Orders
 ### Description/Code Snippet
The `Positions.deposit` and `Orders.increaseSellAmount` functions calculate the required token amount based on the desired liquidity/order size and attempt to pay exactly that amount via `ACCOUNTANT.payTwoFrom` / `payFrom`. These functions rely on the `FlashAccountant` logic which tracks debt based on the balance delta. For fee-on-transfer (FOT) tokens, the Core receives less than the amount transferred, causing the `DebtsNotZeroed` check to fail at the end of the lock because the locker (Positions/Orders contract) is credited with less than the debt it incurred. This makes these core features unusable for FOT tokens.
 ### Static Signals
ACCOUNTANT.payTwoFrom, ACCOUNTANT.payFrom, DebtsNotZeroed
 ### Assets at Risk
User funds (gas spent on reverting transactions)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: Oracle.extrapolateSnapshot

 ### Title
Oracle extrapolation can return stale tick data within the same block
 ### Description/Code Snippet
The `Oracle` extension records snapshots via `maybeInsertSnapshot` in `beforeSwap` / `beforeUpdatePosition`. This captures the state *before* the current transaction's changes. If `extrapolateSnapshot` is called in the same block after a swap, it calculates the cumulative values using `timePassed = 0` (since the snapshot was just updated to `now` with the pre-swap tick). As a result, the returned observation does not reflect the tick change from the most recent swap in the same block, potentially serving stale price data to other protocols integrating deeply in the same transaction.
 ### Static Signals
timePassed == 0, snapshot.timestamp() == block.timestamp
 ### Assets at Risk
Integrator funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router.swap

 ### Title
Public swap function defaults to zero slippage protection
 ### Description/Code Snippet
The `Router` contract exposes a public `swap` overload that accepts `SqrtRatio sqrtRatioLimit` but defaults the `calculatedAmountThreshold` (minimum output amount) to `type(int256).min`. While `sqrtRatioLimit` can act as a price limit, it is often set to min/max values by default in integrations. If a user or integrator calls this specific overload without calculating a tight `sqrtRatioLimit`, the swap executes with effectively no slippage protection on the output amount. This design encourages unsafe usage patterns compared to requiring an explicit minimum amount.
 ### Static Signals
amountOutMin=0 or missing, defaults to no slippage check
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Fee-on-Transfer tokens cause insolvency in Incentives contract
 ### Description/Code Snippet
The `Incentives.fund` function transfers tokens using `safeTransferFrom` but assumes the received amount equals the input `minimum`. It updates `dropState.funded` with this assumed amount. If a fee-on-transfer token is used, the contract receives fewer tokens than recorded. When users call `claim`, the contract attempts to transfer the full amounts. Eventually, the contract will hold insufficient balance to cover the remaining 'funded' claims, causing the last claims to revert and locking those users' rewards.
 ### Static Signals
safeTransferFrom without balance check, accounting based on input amount
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Oracle extension defaults to unsafe low capacity (1 snapshot)
 ### Description/Code Snippet
The `Oracle` extension initializes with a snapshot capacity of 1 (or the existing capacity) in `beforeInitializePool`. If users or integrators do not explicitly call `expandCapacity`, the oracle only retains the single most recent snapshot. This effectively reduces the oracle to a spot price feed or a very short-window TWAP, making it vulnerable to manipulation or creating denial-of-service (`NoPreviousSnapshotExists`) for applications requiring historical data.
 ### Static Signals
observationCardinality/min not enforced, twapWindow < 10–30 minutes
 ### Assets at Risk
Protocol/User funds relying on Oracle
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: FlashAccountant.withdraw

 ### Title
Unsafe recipient in FlashAccountant withdrawal
 ### Description/Code Snippet
The `withdraw` function in `FlashAccountant.sol` (inherited by `Core.sol`) accepts a `recipient` address and performs transfers (native ETH or ERC20) to it without verifying that the address is non-zero. If a user or integrating contract accidentally passes `address(0)` as the recipient, native ETH will be burned (sent to 0x0) and ERC20 tokens may be lost (transferred to 0x0, depending on token implementation), leading to irreversible fund loss.
 ### Static Signals
no zero-address guard, call(gas(), recipient, ...)
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.withdraw

 ### Title
Missing slippage protection in Position withdrawals
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` (inherited by `Positions.sol`) allows users to remove liquidity from a pool. While it accepts a `liquidity` amount to remove, it does not provide parameters for `minAmount0` or `minAmount1`. The withdrawal amounts are calculated based on the current pool price (tick) and the liquidity range. An attacker can front-run the withdrawal transaction to manipulate the pool price (sandwich attack), causing the user to withdraw a highly unfavorable ratio of assets (e.g. mostly the less valuable token) compared to what was expected, effectively suffering uncapped slippage.
 ### Static Signals
no minAmountOut parameter in liquidation/redemption, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM Economic Manipulation via Sandwich Attack
 ### Description/Code Snippet
The `TWAMM` extension executes pending virtual orders based on the pool's state at the moment of the transaction (specifically, the state *before* the user's swap in `beforeSwap`). An attacker can manipulate the pool price using a flash loan or large swap immediately before triggering the TWAMM execution (e.g., via a small swap or `lockAndExecuteVirtualOrders`), causing the virtual orders to fill at a distorted price favorable to the attacker, and then swap back to profit. The system lacks a mechanism to verify the execution price against a manipulation-resistant oracle or time-weighted average.
 ### Static Signals
execution based on spot reserves/price, no slippage check for virtual orders, executed in same tx as trigger
 ### Assets at Risk
TWAMM Order Value
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: Orders.sol.increaseSellAmount

 ### Title
Rounding error in Orders.increaseSellAmount leads to stuck ETH
 ### Description/Code Snippet
In `Orders.sol`, the `increaseSellAmount` function calculates `saleRate` by dividing the input `amount` by the duration. This integer division rounds down. The `TWAMM` extension uses this `saleRate` to calculate the actual amount of tokens to pull/sell (`saleRate * duration`), which will be less than or equal to the original `amount`. When the sell token is ETH (NATIVE_TOKEN), the user sends `msg.value` equal to `amount`, but `TWAMM` only utilizes `saleRate * duration`. The difference (`msg.value - utilized`) remains stuck in the `Orders` contract, as there is no mechanism to refund the excess ETH.
 ### Static Signals
divide before multiply logic in separate contracts, no refund mechanism
 ### Assets at Risk
User ETH
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: Core.swap_6269342730

 ### Title
State Corruption via Reentrancy in Core.swap
 ### Description/Code Snippet
The `swap_6269342730` function in `Core.sol` reads the `PoolState` into memory (`stateAfter`) before calling the `beforeSwap` extension hook. If an extension (like `TWAMM`) performs a swap on the same pool within this hook (which `TWAMM` does via `lockAndExecuteVirtualOrders`), the pool's storage is updated. However, the outer `swap` function continues execution using the stale in-memory `stateAfter` and overwrites the storage at the end with `writePoolState`, effectively reverting the price/tick/liquidity updates from the hook while preserving the debt/accounting side effects.
 ### Static Signals
readPoolState before external call, writePoolState after external call, Hook calls function that modifies same state
 ### Assets at Risk
Pool liquidity, User funds (swapped at wrong price)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture Extension Bricks Swaps via Always-Reverting Callback
 ### Description/Code Snippet
The `MEVCapture` extension registers the `beforeSwap` hook (`mevCaptureCallPoints` sets `beforeSwap: true`) but implements it to unconditionally `revert SwapMustHappenThroughForward()`. When `CORE.swap` is called, it triggers `extension.maybeCallBeforeSwap`, which calls `MEVCapture.beforeSwap`, causing the transaction to revert. While `MEVCapture` intends for users to use `forward`, the internal call to `CORE.swap` within `handleForwardData` *also* triggers the `beforeSwap` hook on the extension, creating a circular failure that makes swapping in the pool impossible.
 ### Static Signals
callback success required for core flow to proceed, no bypass on callback failure
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing slippage protection in Positions.withdraw
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` (and `BasePositions.sol`) calculates the amount of tokens to return based on the burned liquidity and the current pool tick/price, but does not allow the user to specify minimum output amounts (`amount0Min`, `amount1Min`). If the pool price is manipulated or volatile, the user may receive significantly fewer tokens than expected.
 ### Static Signals
withdraw() converts shares to assets at current rate without minimum, no minAmountOut parameter in liquidation/redemption
 ### Assets at Risk
User liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEV Capture Fee Avoidance and Victim Griefing via Stale State
 ### Description/Code Snippet
The `MEVCapture` extension calculates variable fees based on the tick displacement relative to `tickLast`. However, `tickLast` is only updated at the start of the block (or via explicit call), not after every swap. In a sandwich attack scenario:
1. Attacker swaps 100->110. `tickLast`=100. Fee on 10 ticks.
2. Victim swaps 110->120. `tickLast`=100. Fee on |120-100|=20 ticks (Victim pays fee on attacker's displacement).
3. Attacker backruns 120->100. `tickLast`=100. Fee on |100-100|=0 ticks.

The attacker pays 0 variable fees on the backrun, whereas updating `tickLast` would force them to pay fees on the 20-tick reversion. This design subsidizes sandwich attacks and increases costs for victims.
 ### Static Signals
lastUpdateTime != currentTime, tickLast not updated after swap
 ### Assets at Risk
User funds (via inflated fees), Protocol Revenue (lost backrun fees)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded Loop in TWAMM Virtual Order Execution enables DoS
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function in `TWAMM.sol` contains a `while` loop that iterates through time intervals from the last execution time up to the current block timestamp. It calls `searchForNextInitializedTime` and executes logic (swaps, updates) for each initialized time found. An attacker can create many small TWAMM orders with distinct expiration times (one per second or tick) to populate the `poolInitializedTimesBitmapSlot` densely. When a user subsequently interacts with the pool (triggering the loop), the loop may iterate enough times to exceed the block gas limit, causing the transaction to revert and effectively freezing the pool.
 ### Static Signals
while (time != block.timestamp), no gas limit checks in loop body, iteration count grows with contract state
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

