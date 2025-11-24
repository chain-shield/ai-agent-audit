## Verified Patterns Found: 19

## Verified Patterns Found in following Categories:

- StandardViolation
- GriefableCallbacks
- FlashLoanEconomicManipulation
- UnsafeRecipient
- TWAPWindowPinningOrLowLiquidity
- ForcedAssetVsStrictEquality
- Reentrancy
- FeeOnTransferAssumption
- AccountingInvariantViolation
- OracleUsingDEXorTWAP
- SlippageMissingOrInsufficient
- UnboundedLoops



## Summary of Patterns

MEVCapture extension permanently reverts all swaps

Transient balance in TokenWrapper breaks ERC20 persistence

Denial of Service via forced excess payments in FlashAccountant

TWAMM Virtual Orders Vulnerable to Sandwiching due to Instant Execution

TokenWrapper wrapping mechanism mints via debt accounting without explicit user credit

Oracle extrapolation returns stale pre-block state potentially misleading integrators

Missing slippage protection in Positions.withdraw

Oracle TWAP manipulation via empty pool initialization

Oracle storage key collision due to raw address shifting

MEV Capture fees can be bypassed by back-running

Infinite Recursion / DoS in TWAMM pools

Fee Accounting Drift and Loss on Zero Liquidity

Unsafe Recipient in FlashAccountant Withdrawal

Unsafe Default Slippage Parameter in Router Swap Overloads

Positions withdrawal lacks minimum output parameters for slippage protection

TokenWrapper violates ERC20 standard by not updating balances on wrap

Potential Denial of Service in TokenWrapper due to strict calldata length check

Incentives funding assumes 1:1 transfer and breaks with Fee-on-Transfer tokens

Unbounded iteration in TWAMM virtual order execution enables DoS

## Patterns



 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture extension permanently reverts all swaps
 ### Description/Code Snippet
The `MEVCapture` extension registers the `beforeSwap` hook (`beforeSwap: true` in `mevCaptureCallPoints`) but implements the hook to unconditionally `revert SwapMustHappenThroughForward()`. While this is intended to prevent direct swaps, the `MEVCapture` logic itself calls `CORE.swap` inside `handleForwardData`. Since `CORE.swap` invokes the `beforeSwap` hook of the configured extension, the extension's internal swap triggers its own revert. This circular dependency causes a Denial of Service for any pool configured with the `MEVCapture` extension, as no swap can ever complete.
 ### Static Signals
revert in hook, callback success required for core flow
 ### Assets at Risk
Liquidity Pool Usability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.transfer

 ### Title
Transient balance in TokenWrapper breaks ERC20 persistence
 ### Description/Code Snippet
The `TokenWrapper` contract uses a `transient` variable `coreBalance` to track tokens transferred to `address(CORE)`. This logic is inside the `transfer` and `transferFrom` functions. Since transient storage is cleared at the end of every transaction, any tokens transferred to `address(CORE)` outside of a specific atomic transaction flow will be permanently lost (balance resets to zero). This violates the standard ERC20 invariant that balances are persistent.
 ### Static Signals
transient coreBalance, if (to == address(CORE)) coreBalance += amount
 ### Assets at Risk
User tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: FlashAccountant.completePayments

 ### Title
Denial of Service via forced excess payments in FlashAccountant
 ### Description/Code Snippet
The `FlashAccountant` (inherited by `Core`) enforces that the number of non-zero debt slots (`nonzeroDebtCount`) must be zero at the end of a lock. The system considers negative debt (credit) as a non-zero state. If an attacker can force a token or ETH transfer to the Core contract during a victim's transaction execution (e.g., via a hook or callback), the victim's subsequent `completePayments` or `receive` call will attribute the extra balance to the victim, resulting in a net credit (negative debt). This causes `nonzeroDebtCount` to remain positive, triggering the `DebtsNotZeroed` revert and griefing the transaction.
 ### Static Signals
fee accumulator reset depends on exact equality, require(debt == 0)
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM Virtual Orders Vulnerable to Sandwiching due to Instant Execution
 ### Description/Code Snippet
The `TWAMM` extension executes virtual orders (`lockAndExecuteVirtualOrders`) based on the current pool state at the moment of the first interaction in a block. Because this execution happens atomically with the user interaction and uses the spot price (or calculated price movement based on spot) without an external oracle or averaging window, it acts as a predictable, unprotected trade. MEV searchers can manipulate the pool price ('sandwich') immediately before triggering the TWAMM execution (via any call like `beforeSwap`), forcing the virtual orders to execute at a disadvantageous price, extracting value from the long-term order holders.
 ### Static Signals
uses spot price at execution, no freshness/age bound on observations, predictable manipulation
 ### Assets at Risk
TWAMM Order Collateral
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
TokenWrapper wrapping mechanism mints via debt accounting without explicit user credit
 ### Description/Code Snippet
The `TokenWrapper` contract allows users to wrap tokens by incurring debt in the FlashAccountant system. When a user calls `forward` with a positive amount, the `TokenWrapper` (as locker) increases its saved balance of the underlying token and decreases its debt of the wrapper token. Since the debt is tracked by the Locker ID (which is shared with the user during forward), the user exits the forward with a credit (negative debt) of wrapper tokens. The user must then call `withdraw` to mint the actual ERC20 wrapper tokens. This unusual accounting flow relies on the `FlashAccountant` allowing `withdraw` to trigger `TokenWrapper.transfer`, which mints tokens when the sender is Core. While functional, this complex debt-based minting creates a risk of accounting invariant violations if the user fails to withdraw or if integrations misinterpret the debt state.
 ### Static Signals
updateDebt(-amount), no _mint call, transfer mints if msg.sender is Core
 ### Assets at Risk
User funds, Wrapper solvency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: Oracle.extrapolateSnapshot

 ### Title
Oracle extrapolation returns stale pre-block state potentially misleading integrators
 ### Description/Code Snippet
The `Oracle` extension's `extrapolateSnapshot` function is designed to return the state from the *last* snapshot. Due to `maybeInsertSnapshot` logic, a snapshot for the current block is only inserted once (at the first interaction), capturing the state *before* that interaction. Subsequent reads in the same block (even after large price movements) will return the pre-movement state (effectively the previous block's closing state) because `timePassed` will be 0. Integrators expecting `extrapolateSnapshot(now)` to reflect the *current* spot price or an updated TWAP including the current block's activity may be relying on stale data, potentially leading to mispricing in downstream protocols.
 ### Static Signals
timePassed == 0, CORE.poolState
 ### Assets at Risk
integrator funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.withdraw

 ### Title
Missing slippage protection in Positions.withdraw
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` (inherited by `Positions.sol`) calculates the amount of token0 and token1 to return to the user based on the liquidity being burned and the current pool tick. However, it does not accept `amount0Min` and `amount1Min` parameters to enforce minimum output amounts. In concentrated liquidity pools, the ratio of assets in a position changes as the price moves. An attacker can sandwich a withdrawal transaction, manipulating the pool price to force the user's liquidity to be converted entirely into the less valuable asset or withdrawn at a disadvantageous exchange rate, without the user's consent or ability to revert.
 ### Static Signals
no minAmountOut parameter in liquidation/redemption, withdraw() converts shares to assets at current rate without minimum
 ### Assets at Risk
User liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Oracle TWAP manipulation via empty pool initialization
 ### Description/Code Snippet
The `Oracle` extension begins recording `tickCumulative` snapshots immediately upon pool initialization (`beforeInitializePool`), even if the pool has zero liquidity. An attacker can initialize a pool with an extreme tick value, wait for a significant duration without adding liquidity, and then fund the pool. The Oracle will effectively 'backdate' the extreme tick for the entire waiting period, producing a manipulated TWAP that could be exploited by downstream protocols relying on this Oracle.
 ### Static Signals
snapshot created with 0 liquidity, tickCumulative increments based on initial tick
 ### Assets at Risk
Downstream protocol funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Oracle.expandCapacity

 ### Title
Oracle storage key collision due to raw address shifting
 ### Description/Code Snippet
The `expandCapacity` function in `Oracle.sol` uses `sstore(or(shl(32, token), i), 1)` to initialize storage slots. The key generation logic `(token << 32) | i` creates a collision hazard. Specifically, the storage slot for `Counts` of `TokenA` (key `TokenA`) collides with the storage slot for `Snapshot` index `0` of `TokenB` if `TokenA == TokenB << 32`. Since addresses are 160 bits, this condition is satisfiable for specific valid address pairs (e.g., `TokenB = 0x...01`, `TokenA = 0x...0100000000`). An attacker can corrupt the `Counts` struct of a target token (resetting index/count/capacity) by calling `expandCapacity` on the colliding token address, effectively bricking the oracle for the target.
 ### Static Signals
sstore(or(shl(32, token), i), 1), assembly writes to unchecked slots
 ### Assets at Risk
Oracle integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEV Capture fees can be bypassed by back-running
 ### Description/Code Snippet
The `MEVCapture` extension calculates fees based on the absolute difference between the current tick and the tick at the start of the block (`tickLast`). Logic: `(abs(stateAfter.tick() - tickLast) << 64) / tickSpacing`. If an arbitrageur or sandwich attacker performs a swap that restores the price to `tickLast` (closing the loop), the difference is zero, and they pay no MEV capture fee on the closing trade. This asymmetry allows MEV searchers to evade fees on the back-run leg of their strategy.
 ### Static Signals
FixedPointMathLib.abs(stateAfter.tick() - tickLast)
 ### Assets at Risk
Protocol Revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: TWAMM.beforeSwap

 ### Title
Infinite Recursion / DoS in TWAMM pools
 ### Description/Code Snippet
The TWAMM extension executes virtual orders by calling `CORE.swap`. However, `CORE.swap` triggers the `beforeSwap` hook on the pool's extension. Since the pool's extension is TWAMM itself, `beforeSwap` calls `lockAndExecuteVirtualOrders`, which attempts to execute virtual orders again. This creates an infinite recursion loop (Stack Overflow) for any swap on a TWAMM-enabled pool that has active virtual orders (amount > 0). This effectively denies service to the pool once orders are active.
 ### Static Signals
calls CORE.swap, beforeSwap calls lockAndExecuteVirtualOrders, recursive hook execution
 ### Assets at Risk
Pool availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
Fee Accounting Drift and Loss on Zero Liquidity
 ### Description/Code Snippet
The `MEVCapture` extension collects fees into its own saved balance and periodically distributes them to LPs via `CORE.accumulateAsFees`. However, `accumulateAsFees` skips distribution if the pool's active liquidity is zero (to avoid division by zero). If `MEVCapture` distributes collected fees while the pool is in a zero-liquidity state (e.g. price moved to a gap), the fees are removed from the extension's balance but not added to `feesPerLiquidity`, effectively burning them or leaving them inaccessible in the Core contract.
 ### Static Signals
CORE.accumulateAsFees called, liquidity checked for zero in Core, saved balance decremented
 ### Assets at Risk
Accumulated Protocol/MEV Fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: FlashAccountant.withdraw

 ### Title
Unsafe Recipient in FlashAccountant Withdrawal
 ### Description/Code Snippet
The `FlashAccountant.withdraw` function executes token transfers or ETH calls to a `recipient` address without checking if `recipient` is the zero address. If a user (or derived contract like `Positions`) mistakenly passes `address(0)`, funds are burned or irretrievably lost. While typically user error, standard safety patterns require a zero-address check for fund withdrawals.
 ### Static Signals
call(gas(), recipient, ...), no require(recipient != address(0))
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router.swap

 ### Title
Unsafe Default Slippage Parameter in Router Swap Overloads
 ### Description/Code Snippet
The Router contract provides `swap` overloads that default the `calculatedAmountThreshold` (minimum output amount) to `type(int256).min`. Users or integrators utilizing these overloads (e.g., to specify a `sqrtRatioLimit` and `skipAhead`) will unwittingly execute swaps with absolutely no output amount guarantee, relying solely on the price limit. In pools with low liquidity or during high volatility, a price limit alone may not prevent significant value loss on the total output amount, effectively creating a 'missing slippage protection' vulnerability.
 ### Static Signals
calculatedAmountThreshold = type(int256).min, no minAmountOut parameter in overload
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Positions withdrawal lacks minimum output parameters for slippage protection
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` accepts a `liquidity` amount to burn but does not provide parameters for `amount0Min` and `amount1Min`. It calculates the output amounts based on the current pool price and tick, then transfers them to the recipient. Without minimum output constraints, a user interacting directly with this contract (as implied by it being a top-level `BaseLocker`) is vulnerable to sandwich attacks or price manipulation, potentially receiving significantly fewer tokens than expected.
 ### Static Signals
no minAmountOut parameter, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
TokenWrapper violates ERC20 standard by not updating balances on wrap
 ### Description/Code Snippet
The `TokenWrapper` contract implements `IERC20` but its wrapping mechanism (via `handleForwardData`) only updates `Core`'s internal `savedBalances` and the user's debt/credit in the `FlashAccountant`. It does not update the `_balanceOf` mapping in `TokenWrapper`. Consequently, a user who wraps tokens will see a `balanceOf` of 0 and cannot transfer them using standard ERC20 functions unless they perform a subsequent `withdraw` from the `FlashAccountant` in the same transaction to 'mint' the actual ERC20 balance. This breaks atomicity and composability expectations for ERC20 wrappers.
 ### Static Signals
no _mint, updates external accounting but not local balance
 ### Assets at Risk
User funds (integration confusion)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
Potential Denial of Service in TokenWrapper due to strict calldata length check
 ### Description/Code Snippet
The `Core.updateDebt` function strictly enforces `msg.data.length == 20` to ensure safety. However, `TokenWrapper` calls this function via `FlashAccountantLib` using `CORE.updateDebt(SafeCastLib.toInt128(-amount))`. If `FlashAccountantLib` or the compiler uses standard ABI encoding (selector + 32-byte padded argument = 36 bytes), the call will revert inside `Core`, rendering the `TokenWrapper` non-functional for wrapping/unwrapping. This relies on the assumption that the library performs manual packing.
 ### Static Signals
msg.data.length != 20, abi.encodeCall
 ### Assets at Risk
user funds (locked in wrapper)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Incentives funding assumes 1:1 transfer and breaks with Fee-on-Transfer tokens
 ### Description/Code Snippet
The `fund` function in `Incentives.sol` uses `SafeTransferLib.safeTransferFrom` to transfer the `minimum` amount from the caller to the contract, and then strictly updates `dropState.funded` by the same `minimum` amount. It does not check the actual balance increase of the contract. If a Fee-on-Transfer token is used, the contract will receive less than `minimum`, but the internal accounting will reflect the full amount. This discrepancy leads to insolvency when users attempt to `claim`, as the contract will eventually run out of tokens before all tracked claims are satisfied.
 ### Static Signals
uses input amount instead of post-transfer delta, assumes transferFrom(amount) credits exactly amount
 ### Assets at Risk
user rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded iteration in TWAMM virtual order execution enables DoS
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function in `TWAMM.sol` iterates through initialized time slots between the last execution time and the current block timestamp. This loop processes virtual orders by performing heavy computations and potential Core swaps for each time interval. An attacker can mint numerous orders with distinct end times (e.g., every second), populating the `initializedTimesBitmap` with many set bits. This forces subsequent interactions (swaps/updates) to iterate through all these time slots, potentially exceeding the block gas limit and causing a Denial of Service (DoS) for the pool, as the state cannot be advanced.
 ### Static Signals
while (time != block.timestamp), CORE.swap
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

