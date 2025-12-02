## Verified Patterns Found: 22

## Verified Patterns Found in following Categories:

- FeeOnTransferAssumption
- FlashLoanEconomicManipulation
- ReadOnlyReentrancy
- SlippageMissingOrInsufficient
- TWAPWindowPinningOrLowLiquidity
- AccountingInvariantViolation
- UnboundedLoops
- UnsafeRecipient
- StandardViolation



## Summary of Patterns

Missing slippage and deadline protection in withdrawal

Unsafe recipient in `withdraw` allows fund loss

NFT Positions minted to contracts without onERC721Received check may be locked

Router breaks with Fee-On-Transfer tokens

Storage Collision in Oracle Extension allows state corruption

Incompatibility with fee-on-transfer tokens due to strict debt settlement

Sandwich attack on TWAMM virtual order backlog via spot price manipulation

Oracle defaults to unsafe observation cardinality of 1

MEV Capture extension fails to capture fees from intra-block backrunning/arbitrage

Unbounded loop in TWAMM virtual order execution allows permanent DoS

Router swap() overloads default to no slippage protection

Unbounded sale rate in RevenueBuybacks allows dump attacks via donation

Revenue Buybacks create TWAMM orders without price protection

MEVCapture pools cannot swap due to unconditional revert in hook

Read-only reentrancy in Oracle extension

Incompatibility with Fee-on-Transfer Tokens causes DoS in Router

Missing slippage protection in withdrawal function

Incompatibility with Fee-On-Transfer tokens

Missing Zero-Address Check for Position Withdrawal Recipient

Missing Slippage Protection in Positions Withdrawal

Missing slippage protection in Positions.withdraw

Unsafe recipient in Router and Positions

## Patterns



 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.withdraw

 ### Title
Missing slippage and deadline protection in withdrawal
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` (inherited by `Positions.sol`) allows users to burn liquidity to receive underlying tokens but accepts no parameters for minimum output amounts (`minAmount0`, `minAmount1`) nor a timestamp `deadline`. The amounts returned are determined by the pool's current tick at execution time. Without slippage bounds, a user's withdrawal can be sandwiched or executed at an unfavorable price, altering the asset ratio significantly. Without a deadline, a transaction can remain pending and execute in changed market conditions.
 ### Static Signals
withdraw() converts shares to assets at current rate without minimum, deadline omitted or far future
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: BasePositions.sol.withdraw

 ### Title
Unsafe recipient in `withdraw` allows fund loss
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` accepts a `recipient` address parameter to which the withdrawn tokens are sent. There is no check to ensure that `recipient` is not `address(0)`. If a user accidentally specifies the zero address (or if a frontend defaults to it), the tokens will be sent to the zero address and effectively burned. While the `FlashAccountantLib` uses `call` for ETH transfers (which succeeds to 0x0) and standard `call` for ERC20s (which usually revert on 0x0 transfer, but not always), funds can still be lost for native tokens or non-standard ERC20s that do not revert.
 ### Static Signals
no zero-address guard
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Positions.mint

 ### Title
NFT Positions minted to contracts without onERC721Received check may be locked
 ### Description/Code Snippet
The `Positions` contract functions `mint` and `mintAndDeposit` use Solady's `_mint` function, which does not check if the recipient implements `onERC721Received`. If a user mints a position to a contract wallet that does not support ERC721 handling (and lacks a method to transfer it out), the financial position will be permanently locked. Standard NFT implementations typically recommend `safeMint` to prevent this user error.
 ### Static Signals
_mint called instead of _safeMint, missing onERC721Received check
 ### Assets at Risk
User liquidity/positions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Router.handleLockData

 ### Title
Router breaks with Fee-On-Transfer tokens
 ### Description/Code Snippet
The `Router` contract handles exact-input swaps by calling `CORE.swap` and then settling the debt via `ACCOUNTANT.payFrom(swapper, token, amount)`. The `FlashAccountant` reduces the locker's debt by the *actual* balance increase (`currentBalance - lastBalance`). For fee-on-transfer tokens, the balance increase is less than the `amount` transferred. This leaves a residual debt in the `FlashAccountant`, causing the `lock` call to revert with `DebtsNotZeroed`. This effectively renders the Router unusable for fee-on-transfer tokens.
 ### Static Signals
assumes transferFrom(amount) credits exactly amount
 ### Assets at Risk
Availability (DoS for specific tokens)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Oracle.sol.maybeInsertSnapshot

 ### Title
Storage Collision in Oracle Extension allows state corruption
 ### Description/Code Snippet
The `Oracle` extension uses a manual storage layout where `Counts` are stored at slot `token` (address) and `Snapshots` are stored at slot `(token << 32) | index`. If a token address `A` has 32 leading zeros, its snapshot storage key `(A << 32) | index` forms a valid 160-bit address `B`. Consequently, writing a snapshot for token `A` at `index` will overwrite the `Counts` struct (containing count, capacity, and lastTimestamp) for token `B`. An attacker can generate/mine a token address with leading zeros (`A`) to target and corrupt the oracle state of another token (`B`), causing denial of service or manipulation of the oracle data used by other protocols.
 ### Static Signals
sstore(token, c), sstore(or(shl(32, token), index), snapshot)
 ### Assets at Risk
oracle data integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Positions.sol.deposit

 ### Title
Incompatibility with fee-on-transfer tokens due to strict debt settlement
 ### Description/Code Snippet
The `deposit` function in `Positions.sol` uses `FlashAccountantLib` to transfer tokens from the user to the Core. The logic calculates the exact amounts required for the liquidity and calls `transferFrom` for those specific amounts. If a token implements a fee-on-transfer mechanism, the Core contract receives less than the amount transferred. The `FlashAccountant` inside Core enforces that the net balance change matches the debt incurred by the liquidity update. Since the received amount is lower, the debt is not fully cleared, causing the transaction to revert with `DebtsNotZeroed`. This effectively renders the protocol unusable for standard fee-on-transfer tokens.
 ### Static Signals
transferFrom(amount), strict debt equality check, no balance check before/after transfer
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Sandwich attack on TWAMM virtual order backlog via spot price manipulation
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function executes pending virtual orders using the pool's current spot price (`corePoolState.sqrtRatio()`). If a pool has a backlog of unexecuted virtual orders, an attacker can manipulate the spot price (e.g., via a large swap) in a separate transaction or interaction within the same block, and then trigger the TWAMM execution. The accumulated virtual orders will execute at the manipulated price, allowing the attacker to arbitrage the price difference.
 ### Static Signals
CORE.swap, uses pool spot price for execution
 ### Assets at Risk
User funds in TWAMM orders
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Oracle defaults to unsafe observation cardinality of 1
 ### Description/Code Snippet
The `Oracle` extension initializes the snapshot buffer with a `count` and `capacity` of 1 in `beforeInitializePool`. Unless a user explicitly calls `expandCapacity` at their own expense, every new write overwrites the previous snapshot. This lack of historical buffer means TWAP queries for any duration longer than the time since the last action will revert with `NoPreviousSnapshotExists`, or potentially return manipulated recent data, rendering the oracle unsafe for downstream integrations by default.
 ### Static Signals
observationCardinality/min not enforced
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEV Capture extension fails to capture fees from intra-block backrunning/arbitrage
 ### Description/Code Snippet
The `MEVCapture` extension calculates fees based on the difference between the current tick and `tickLast`. However, `tickLast` is only updated when `lastUpdateTime != block.timestamp` (i.e., once per block or upon the first interaction in a block). 

This creates a static reference point for the entire block. If a transaction moves the price away from `tickLast` (e.g., a large swap moving tick 100 -> 150), it pays fees. However, an arbitrageur backrunning this transaction to restore the price (150 -> 100) will calculate the fee based on `abs(100 - 100) = 0`. 

Consequently, backrunning arbitrage, which constitutes a significant portion of MEV, completely bypasses the fee mechanism.
 ### Static Signals
lastUpdateTime != currentTime, abs(stateAfter.tick() - tickLast)
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded loop in TWAMM virtual order execution allows permanent DoS
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function iterates through initialized time checkpoints from the last execution time to the current block timestamp using a `while` loop. An attacker can create a dense sequence of initialized times (e.g., by creating many TWAMM orders with different end times) such that the number of iterations required to catch up to the current block exceeds the block gas limit. Since the loop must run to completion to update the state to `block.timestamp`, and the fallback `lockAndExecuteVirtualOrders` executes the same logic, the pool becomes permanently unusable (DoS).
 ### Static Signals
while (time != block.timestamp), searchForNextInitializedTime
 ### Assets at Risk
Pool liquidity (frozen)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router.swap

 ### Title
Router swap() overloads default to no slippage protection
 ### Description/Code Snippet
The `Router` contract provides `swap` overloads that do not require a `calculatedAmountThreshold` argument, defaulting it to `type(int256).min` (effectively zero/negative infinity). Integrators or users calling these convenience functions might inadvertently execute swaps with zero slippage protection, exposing them to MEV sandwich attacks.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound
 ### Assets at Risk
User tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: RevenueBuybacks.sol.roll

 ### Title
Unbounded sale rate in RevenueBuybacks allows dump attacks via donation
 ### Description/Code Snippet
The `RevenueBuybacks.roll` function creates or updates TWAMM orders using the contract's entire token balance (`amountToSpend`) and passes `type(uint112).max` as `maxSaleRate` to `ORDERS.increaseSellAmount`. This disables the sale rate (slippage/impact) protection in the TWAMM order. An attacker can donate a large amount of tokens to the `RevenueBuybacks` contract and call `roll` when the order's `timeRemaining` is small (but valid). This forces a disproportionately high sale rate (amount/time), causing the TWAMM to execute a rapid, high-impact sell-off of protocol revenue in the pool. The attacker can profit by front-running/shorting the token before triggering the dump.
 ### Static Signals
type(uint112).max, amountToSpend = ... balance ...
 ### Assets at Risk
treasury, revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Revenue Buybacks create TWAMM orders without price protection
 ### Description/Code Snippet
The `RevenueBuybacks.roll` function creates or extends TWAMM sell orders for protocol revenue tokens. It calculates the sale rate based on the available balance and duration, and passes `type(uint112).max` as the `maxSaleRate` to `ORDERS.increaseSellAmount`. This effectively executes a market sell order over time with no minimum price floor or slippage protection. If the `BUY_TOKEN` pool is manipulated or has low liquidity, protocol revenue may be sold at a significant loss.
 ### Static Signals
ORDERS.increaseSellAmount(..., type(uint112).max), no minAmountOut parameter
 ### Assets at Risk
Protocol Revenue / Treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture pools cannot swap due to unconditional revert in hook
 ### Description/Code Snippet
The `MEVCapture` extension implements `beforeSwap` which unconditionally reverts with `SwapMustHappenThroughForward`. When swapping through `MEVCaptureRouter`, the flow forwards the call to `MEVCapture` (making it the locker), which then calls `CORE.swap`. `CORE.swap` invokes the `beforeSwap` hook of the extension (MEVCapture). Since `beforeSwap` reverts unconditionally and does not exempt calls where the locker is the extension itself, all swaps on MEV Capture pools will revert.
 ### Static Signals
revert SwapMustHappenThroughForward(), unconditional revert in hook
 ### Assets at Risk
Pool usability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReadOnlyReentrancy

 ### Relevant Function/Location: Oracle.sol.extrapolateSnapshot

 ### Title
Read-only reentrancy in Oracle extension
 ### Description/Code Snippet
The `Oracle.extrapolateSnapshot` function computes the current time-weighted average by extrapolating from the last snapshot using the current `CORE.poolState(poolId)`. The Core pool state updates immediately during swap steps. If an external contract calls `extrapolateSnapshot` during a callback (e.g., `afterSwap`) or within the same transaction while the pool is being manipulated, the oracle will return a manipulated value based on the transient dirty state of the pool. This allows attackers to manipulate on-chain pricing for integrations that rely on this oracle.
 ### Static Signals
uses getReserves/spot price that can change intratx, external call before view function stabilizes
 ### Assets at Risk
integrated protocol funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Router.swap

 ### Title
Incompatibility with Fee-on-Transfer Tokens causes DoS in Router
 ### Description/Code Snippet
The Router and FlashAccountant architecture assumes that `transferFrom(amount)` will result in the accountant receiving exactly `amount` tokens to settle the debt incurred by a swap. `FlashAccountant.completePayments` calculates the credit based on the actual balance change (`currentBalance - lastBalance`). For fee-on-transfer tokens, the received amount is less than the transferred amount. Consequently, the debt is not fully cleared, and the `FlashAccountant.lock` function reverts with `DebtsNotZeroed`. This renders the standard Router `swap` functions unusable for exact-input swaps involving fee-on-transfer tokens.
 ### Static Signals
ACCOUNTANT.payFrom(swapper, poolKey.token0, ...), revert DebtsNotZeroed(uint256 id)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.sol.withdraw

 ### Title
Missing slippage protection in withdrawal function
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` (inherited by `Positions.sol`) allows users to burn liquidity and receive the underlying tokens. However, unlike the `deposit` function, it does not accept minimum return amount parameters (`amount0Min`, `amount1Min`). The amounts returned are calculated based on the pool's current price (`sqrtRatio`) and the burned liquidity. If the pool price is manipulated (e.g., via a sandwich attack) or moves significantly before the transaction is mined, the user may receive a much less favorable ratio of assets than expected, realizing impermanent loss at a manipulated price without any revert protection.
 ### Static Signals
withdraw function missing minAmount parameters, liquidity burn without slippage checks
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Positions.deposit

 ### Title
Incompatibility with Fee-On-Transfer tokens
 ### Description/Code Snippet
The protocol's `FlashAccountant` architecture assumes that `transferFrom(amount)` credits exactly `amount` to the contract. In `Positions.deposit`, `Core` establishes a debt based on the liquidity minted. `FlashAccountant` attempts to satisfy this debt by transferring tokens from the user. For fee-on-transfer tokens, the actual balance increase is less than the transfer amount, leaving a residual debt. The `lock` modifier then reverts with `DebtsNotZeroed`, effectively making the protocol unusable for such tokens.
 ### Static Signals
assumes transferFrom(amount) credits exactly amount, accounting based on transfer parameter, not actual balance change
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing Zero-Address Check for Position Withdrawal Recipient
 ### Description/Code Snippet
The `Positions.withdraw` function accepts a `recipient` address but does not validate that it is non-zero. The underlying `FlashAccountant.withdraw` function handles native tokens by attempting a call, which burns ETH if sent to address(0), and handles ERC20 tokens via `call` which may succeed (and burn tokens) depending on the token implementation. This exposes users to accidental loss of funds.
 ### Static Signals
no zero-address guard, ACCOUNTANT.withdraw(..., recipient, ...)
 ### Assets at Risk
User Funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing Slippage Protection in Positions Withdrawal
 ### Description/Code Snippet
The `Positions.withdraw` function allows users to burn liquidity and receive underlying tokens but does not accept minimum output amount parameters (`amount0Min`, `amount1Min`). While the value of the liquidity is generally preserved, the ratio of token0 to token1 returned depends on the current tick (price). If the pool price is manipulated (e.g. via a sandwich attack or MEV) immediately prior to the withdrawal, the user may receive an unfavorable composition of assets without any reversion protection.
 ### Static Signals
withdraw(...) returns (uint128 amount0, uint128 amount1), no minAmount0/minAmount1 arguments
 ### Assets at Risk
User Liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.sol.withdraw

 ### Title
Missing slippage protection in Positions.withdraw
 ### Description/Code Snippet
The `withdraw` function in `Positions.sol` allows users to burn liquidity and receive tokens based on the current pool state without specifying minimum output amounts (`minAmount0`, `minAmount1`). If the pool price is manipulated (e.g. via a sandwich attack) before the withdrawal, the user may receive a distorted ratio of tokens, effectively realizing an immediate impermanent loss and donating value to the attacker. The function only accepts `liquidity` amount and a `recipient`, returning the calculated amounts without validation.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound
 ### Assets at Risk
user liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Positions.sol.withdraw

 ### Title
Unsafe recipient in Router and Positions
 ### Description/Code Snippet
Both `Positions.withdraw` and `Router.swap` accept a `recipient` address. The underlying `FlashAccountant.withdraw` logic executes a low-level call to the recipient for native token withdrawals. If `recipient` is `address(0)` (which is often the default value for uninitialized variables or user error), the native tokens (ETH) will be sent to the zero address and burned. There is no zero-address check in the user-facing functions.
 ### Static Signals
no zero-address guard, transfers to zero or non-receivable addresses
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

