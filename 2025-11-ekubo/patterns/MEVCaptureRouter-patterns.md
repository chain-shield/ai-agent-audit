## Verified Patterns Found: 24

## Verified Patterns Found in following Categories:

- GriefableCallbacks
- PrecisionDriftAccumulation
- FlashLoanEconomicManipulation
- FeeOnTransferAssumption
- ForcedAssetVsStrictEquality
- FeeAccountingDrift
- StandardViolation
- AccountingInvariantViolation
- SlippageMissingOrInsufficient
- UnsafeRecipient
- UnboundedLoops



## Summary of Patterns

Incompatibility with Fee-on-Transfer tokens due to strict debt accounting

Accumulation of 1-wei dust in MEVCapture saved balances

Router fails to pay ETH to Core for exact-amount swaps

TokenWrapper emits incorrect Transfer event parameters violating ERC-20 standard

MEVCapture extension fee bypass via back-running

MEVCapture extension permanently reverts on swaps

MEVCapture extension fee logic creates uncleared surplus causing DoS

Revenue Buybacks Broken for Fee-on-Transfer Tokens

Missing Transaction Deadline Check allows pending swaps to be executed at unfavorable times

SlippageMissingOrInsufficient: Exact output swaps lack input protection against MEV fees

Stuck ETH in MEVCaptureRouter Exposed to Theft

Fragile ETH Handling in MEVCaptureRouter via Manual Transfer

Unsafe recipient address in Router lock callback

Fee-on-transfer tokens break Router/FlashAccountant integration causing revert

Static Tick Reference Allows Tax-Free Mean Reversion

MEV Capture Fees Diverted to LPs instead of Protocol Revenue

Unsafe Recipient Allowance permits loss of funds to zero address

MEVCapture extension renders swaps impossible via beforeSwap revert

Router Incompatibility with Fee-On-Transfer Tokens

Missing zero-address check for swap recipient in Router

Missing Transaction Deadline Check

Router swap overload allows zero slippage protection

Unconditional Revert in MEVCapture Extension Blocks All Swaps

TWAMM Pool DoS via Virtual Order Interval Spamming

## Patterns



 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: MEVCaptureRouter.handleLockData

 ### Title
Incompatibility with Fee-on-Transfer tokens due to strict debt accounting
 ### Description/Code Snippet
The `MEVCaptureRouter` (via inherited `Router` logic) handles input payments using `ACCOUNTANT.payFrom`. This function records the exact amount requested (`balanceUpdate.delta1()`) as debt, transfers that amount from the user, and then credits the debt based on the *actual* balance change of the Accountant. For Fee-on-Transfer tokens, the balance change will be less than the transfer amount, leaving a residual debt. Since the `FlashAccountant` enforces that all debts must be zeroed before the lock releases, any swap with a Fee-on-Transfer token will revert, causing denial of service for these assets.
 ### Static Signals
assumes transferFrom(amount) credits exactly amount, strict debt zeroing
 ### Assets at Risk
availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PrecisionDriftAccumulation

 ### Relevant Function/Location: MEVCapture.loadCoreState

 ### Title
Accumulation of 1-wei dust in MEVCapture saved balances
 ### Description/Code Snippet
In `MEVCapture.sol`, the function `loadCoreState` calculates fees to be distributed to LPs. It performs `fees0 := sub(fees0, gt(fees0, 0))` and similar for `fees1`. This logic subtracts 1 wei from the available collected fees (if non-zero) before calling `CORE.accumulateAsFees`. While likely intended to keep storage slots non-zero for gas optimization, this results in the systematic accumulation of 1 wei per token per update in the extension's saved balances, which is never distributed to LPs.
 ### Static Signals
sub(fees0, gt(fees0, 0)), systematic rounding/truncation
 ### Assets at Risk
Protocol/LP Revenue (dust amounts)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: Router.handleLockData

 ### Title
Router fails to pay ETH to Core for exact-amount swaps
 ### Description/Code Snippet
In `Router.sol`'s `handleLockData`, the logic for handling native token (`NATIVE_TOKEN_ADDRESS`) payment handles cases where the user overpaid (`valueDifference > 0`) or underpaid (`valueDifference < 0`), but fails to handle the exact payment case (`valueDifference == 0`). If `msg.value` exactly matches the required `delta0` (the debt to Core), the Router performs no transfer to the Accountant (Core). Since the base `Router._swap` implementation does not forward `msg.value` to Core (unlike `MEVCaptureRouter`), the Router retains the ETH while Core records an outstanding debt. The transaction subsequently reverts with `DebtsNotZeroed`, causing a DoS for exact-amount ETH swaps.
 ### Static Signals
if (valueDifference > 0), else if (valueDifference < 0), strict equality skipped
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.transferFrom

 ### Title
TokenWrapper emits incorrect Transfer event parameters violating ERC-20 standard
 ### Description/Code Snippet
In `TokenWrapper.transferFrom`, the `Transfer` event is emitted as `emit Transfer(msg.sender, to, amount);`. In the context of `transferFrom`, `msg.sender` is the spender, not the token owner (`from`). This violates the ERC-20 specification which requires the first parameter to be the address tokens are moved from. This breaks compatibility with indexers, wallets, and downstream integrations relying on standard event signatures.
 ### Static Signals
emit Transfer(msg.sender, ...), transferFrom
 ### Assets at Risk
Integration correctness
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEVCapture extension fee bypass via back-running
 ### Description/Code Snippet
The `MEVCapture` extension calculates fees based on the absolute difference between the post-swap tick (`stateAfter.tick()`) and `tickLast`. However, `tickLast` is only updated to the current tick when `lastUpdateTime != currentTime` (i.e., at the start of the block or first interaction). It is NOT updated after each swap within the same block. This allows an arbitrageur to back-run a transaction (moving the price back to the starting tick) and pay zero MEV capture fees, because `|currentTick - tickLast|` evaluates to zero. This defeats the purpose of capturing MEV from arbitrageurs.
 ### Static Signals
state not updated when underlying asset is swapped, tickLast read from storage but not written back after swap
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture extension permanently reverts on swaps
 ### Description/Code Snippet
The `MEVCapture.beforeSwap` hook is declared as `external pure` and unconditionally reverts with `SwapMustHappenThroughForward`. Since the extension registers `beforeSwap: true`, `Core.swap` (which is called inside `MEVCapture.handleForwardData`) triggers this hook. Because the hook is `pure` and lacks context awareness (e.g., checking if the locker is the extension itself), it reverts even for valid forwarded swaps, rendering any pool using this extension completely unusable (DoS).
 ### Static Signals
unconditional revert in hook, pure modifier on context-sensitive hook
 ### Assets at Risk
liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
MEVCapture extension fee logic creates uncleared surplus causing DoS
 ### Description/Code Snippet
In `MEVCapture.sol`, the `handleForwardData` function calculates an additional fee and adds it to `saveDelta0` or `saveDelta1` (which are positive). It then calls `CORE.updateSavedBalances` with these positive deltas. In `Core`, a positive delta in `updateSavedBalances` increases the `savedBalances` of the locker (MEVCapture) AND calls `_updatePairDebtWithNative` with a positive amount. In `FlashAccountant`, a positive debt change adds to the current value. If the current value is debt (negative), adding a positive value REDUCES the debt. 

Scenario:
1. `Core.swap` returns `delta` (e.g., 100 cost). Locker debt is -100.
2. `MEVCapture` adds fee (e.g. 1). Calls `updateSavedBalances(+1)`. Locker debt becomes -99.
3. `MEVCapture` returns `balanceUpdate` with `delta` 101 (100+1).
4. `Router` receives 101. Pays 101. Locker debt becomes -99 + 101 = +2 (Surplus).
5. `FlashAccountant` checks `nonzeroDebtCount`. It is non-zero (+2). Transaction REVERTS.

This logic error causes all swaps routed through `MEVCaptureRouter` to revert, effectively freezing the pools.
 ### Static Signals
saveDelta0 += fee, updateSavedBalances(..., saveDelta0, saveDelta1)
 ### Assets at Risk
Pool availability (DoS)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
Revenue Buybacks Broken for Fee-on-Transfer Tokens
 ### Description/Code Snippet
The `roll` function in `RevenueBuybacks.sol` calculates the `amountToSpend` using `balanceOf` and attempts to pay this exact amount to the accountant via `ORDERS` and `ACCOUNTANT.payFrom`. If the revenue token is a fee-on-transfer token, the `FlashAccountant` receives less than the specified amount. Since the accountant tracks debt based on the `amount` argument but credits based on the actual balance increase, a debt equal to the transfer fee remains. This causes the transaction to revert in `FlashAccountant.lock`, effectively locking the revenue tokens in the contract.
 ### Static Signals
SafeTransferLib.balanceOf, ACCOUNTANT.payFrom
 ### Assets at Risk
revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: MEVCaptureRouter.swap

 ### Title
Missing Transaction Deadline Check allows pending swaps to be executed at unfavorable times
 ### Description/Code Snippet
The `swap` functions (inherited from `Router` and used by `MEVCaptureRouter`) accept a `SwapParameters` struct and slippage threshold but lack a user-specified deadline timestamp. In Ethereum, transactions can hang in the mempool. Without a deadline check, a swap transaction signed by a user could be executed long after submission when market conditions have changed unfavorably, exposing the user to unexpected execution context despite slippage protection.
 ### Static Signals
deadline omitted, no expiration check
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router.handleLockData

 ### Title
SlippageMissingOrInsufficient: Exact output swaps lack input protection against MEV fees
 ### Description/Code Snippet
The `Router` contract (inherited by `MEVCaptureRouter`) performs slippage checks in `handleLockData` using `calculatedAmountThreshold`. For exact output swaps (where `params.amount` is negative), the router checks that the *output* amount received meets the threshold. Since the output amount is fixed by the swap parameters in exact output mode, this check is redundant and fails to bound the *input* amount paid. 

This vulnerability is critical in `MEVCaptureRouter` because the `MEVCapture` extension charges dynamic fees based on intra-block tick movement. If a prior transaction moves the tick significantly, the MEV fee (added to the input amount) can be arbitrarily large. Without a valid `amountInMax` check, users performing exact output swaps will pay these unbounded fees without the transaction reverting.
 ### Static Signals
amountCalculated based on output token for exact output swaps, No amountInMax parameter in high-level swap interface
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: MEVCaptureRouter._swap

 ### Title
Stuck ETH in MEVCaptureRouter Exposed to Theft
 ### Description/Code Snippet
`MEVCaptureRouter` calculates `value` from swap parameters and transfers exactly that amount to Core. If a user sends `msg.value > value`, the excess ETH remains in the router contract. Since `refundNativeToken` (inherited from `Router`) refunds the contract's entire balance to `msg.sender`, any excess ETH left by a user can be immediately swept by an MEV bot calling `refundNativeToken`.
 ### Static Signals
safeTransferETH(address(CORE), value), refundNativeToken
 ### Assets at Risk
User ETH
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: src/MEVCaptureRouter.sol._swap

 ### Title
Fragile ETH Handling in MEVCaptureRouter via Manual Transfer
 ### Description/Code Snippet
The `MEVCaptureRouter` manually transfers ETH to the Core contract using `SafeTransferLib.safeTransferETH(address(CORE), value)` instead of passing it via a payable function call. This relies on the `Core.receive()` fallback function to correctly credit the debt of the current locker ID. While currently functional, this pattern separates value transfer from execution logic (`forward`), creating a risk where `value` calculated by the Router (based on complex ternary logic) might not match the execution expectations, potentially leaving the Router with unintended debt or surplus if the `Core` debt accounting logic changes or if `value` is miscalculated.
 ### Static Signals
SafeTransferLib.safeTransferETH(address(CORE), value), CORE.forward
 ### Assets at Risk
Native Token (ETH)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Router.handleLockData

 ### Title
Unsafe recipient address in Router lock callback
 ### Description/Code Snippet
In `Router.handleLockData`, the `recipient` address is decoded from user-provided data and used in `ACCOUNTANT.withdraw`. There is no check that `recipient != address(0)`. If a user (or UI) accidentally encodes `address(0)`, the tokens or ETH are withdrawn to the zero address and effectively burned/lost. Given the complexity of the encoded data, such errors are plausible.
 ### Static Signals
abi.decode(..., recipient), no zero-address check
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Router._swap

 ### Title
Fee-on-transfer tokens break Router/FlashAccountant integration causing revert
 ### Description/Code Snippet
The `Router` and `FlashAccountantLib` rely on `token.transferFrom` to settle the exact debt amount returned by `Core.swap`. The `FlashAccountant` validates debt clearance by comparing the balance change of the Core contract. If a fee-on-transfer (FOT) token is used, the Core receives less than the debt amount. Consequently, `FlashAccountant.lock` will revert with `DebtsNotZeroed`, making standard Router swaps impossible for FOT tokens without manual intervention.
 ### Static Signals
payFrom calls transferFrom, FlashAccountant checks balance delta
 ### Assets at Risk
User funds (gas spent on failed tx)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: MEVCapture.handleForwardData

 ### Title
Static Tick Reference Allows Tax-Free Mean Reversion
 ### Description/Code Snippet
The `MEVCapture` extension snaps `tickLast` at the first interaction of the block. Variable fees are calculated based on the delta between the current tick and `tickLast`. Trades moving the price away from `tickLast` pay fees, but trades moving the price *back* to `tickLast` (e.g., arbitrageurs closing a gap, or sandwich attackers backrunning) pay zero variable fee. This asymmetric taxation may fail to capture MEV from mean-reverting flows.
 ### Static Signals
stateAfter.tick() - tickLast, lastUpdateTime != currentTime
 ### Assets at Risk
Protocol Revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: src/extensions/MEVCapture.sol.locked_6416899205

 ### Title
MEV Capture Fees Diverted to LPs instead of Protocol Revenue
 ### Description/Code Snippet
The MEV Capture extension documentation states it 'diverts the extra value to protocol revenue'. However, the `MEVCapture.accumulatePoolFees` function calls `CORE.accumulateAsFees`, which adds the captured fees to `poolFeesPerLiquiditySlot`. This distributes the fees to Liquidity Providers (LPs) of the pool rather than the Protocol's revenue stream (e.g., `RevenueBuybacks`). While the protocol may capture a percentage of LP fees via `Positions.sol` protocol fees, the bulk of the captured MEV value is effectively socialized among LPs instead of being captured as protocol revenue as described.
 ### Static Signals
CORE.accumulateAsFees(poolKey, fees0, fees1), FeesAccumulated
 ### Assets at Risk
Protocol Revenue
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: MEVCaptureRouter.swap

 ### Title
Unsafe Recipient Allowance permits loss of funds to zero address
 ### Description/Code Snippet
The `swap` functions accept an arbitrary `recipient` address which receives the swap output. There is no check to ensure `recipient` is not `address(0)`. If a user or integration accidentally passes the zero address, the `handleLockData` function will instruct the `Accountant` to withdraw tokens (or ETH) to `address(0)`, effectively burning them. Given the complexity of the struct parameters, integration errors are plausible.
 ### Static Signals
no zero-address guard, withdraw to recipient
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
MEVCapture extension renders swaps impossible via beforeSwap revert
 ### Description/Code Snippet
The `MEVCapture` extension registers the `beforeSwap` hook but implements it to unconditionally revert with `SwapMustHappenThroughForward`. When `MEVCaptureRouter` routes a swap via `CORE.forward` -> `MEVCapture.handleForwardData` -> `CORE.swap`, Core detects the registered extension and invokes `MEVCapture.beforeSwap`. This triggers the revert, causing a permanent Denial of Service for all swaps on pools configured with this extension.
 ### Static Signals
revert SwapMustHappenThroughForward()
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Router.handleLockData

 ### Title
Router Incompatibility with Fee-On-Transfer Tokens
 ### Description/Code Snippet
The `Router` contract settles user debt by calling `ACCOUNTANT.payFrom` with the exact amount returned by the swap (`balanceUpdate.delta`). The `payFrom` function transfers this exact amount from the user to the Core. If the token implements a fee-on-transfer mechanism, the Core receives less than the expected amount. Since the Core's Flash Accountant enforces strict zero-debt settlement (`DebtsNotZeroed`), the transaction will revert. This effectively makes the Router and `MEVCaptureRouter` unusable for any fee-on-transfer tokens, satisfying the pattern where the system assumes `transferFrom(amount)` credits exactly `amount`.
 ### Static Signals
assumes transferFrom(amount) credits exactly amount, accounting based on transfer parameter
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Router.swap

 ### Title
Missing zero-address check for swap recipient in Router
 ### Description/Code Snippet
The `Router` (and `MEVCaptureRouter`) contracts allow the `recipient` parameter to be `address(0)` in swap functions. In `handleLockData`, this address is passed to `ACCOUNTANT.withdraw` (or `withdrawTwo`). For native token withdrawals, `FlashAccountant` executes a low-level call to the recipient. If `recipient` is `address(0)`, the ETH is burned. For ERC20s, it depends on the token implementation, but funds are likely lost.
 ### Static Signals
recipient passed to withdraw without check, no zero-address guard
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router.swap

 ### Title
Missing Transaction Deadline Check
 ### Description/Code Snippet
The `swap`, `multihopSwap`, and `multiMultihopSwap` functions in `Router.sol` (and by extension `MEVCaptureRouter.sol`) allow users to specify a price slippage limit (`calculatedAmountThreshold` or `sqrtRatioLimit`) but do not include a deadline timestamp parameter. Without a deadline, a transaction can be held in the mempool by validators and executed at a later time when market conditions are less favorable for the user (but still within price bounds), or to extract MEV. This is a deviation from standard AMM router patterns (e.g., Uniswap V2/V3) which strictly enforce deadlines.
 ### Static Signals
deadline omitted, no timestamp check
 ### Assets at Risk
user funds (suboptimal execution)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router.swap

 ### Title
Router swap overload allows zero slippage protection
 ### Description/Code Snippet
The `Router` contract (inherited by `MEVCaptureRouter`) exposes a public `swap` overload that sets `calculatedAmountThreshold` to `type(int256).min`. This completely disables slippage protection for users calling this specific function signature. While arguably a user choice, providing an unsafe default in a public router function is a vulnerability pattern that can lead to loss of funds if integrators use the simpler signature.
 ### Static Signals
type(int256).min, calculatedAmountThreshold
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: MEVCapture.beforeSwap

 ### Title
Unconditional Revert in MEVCapture Extension Blocks All Swaps
 ### Description/Code Snippet
The `MEVCapture` extension forces swaps to go through `forward` by reverting in `beforeSwap`. However, when the extension executes the swap via `CORE.swap` inside `handleForwardData`, Core recursively triggers the `beforeSwap` hook on the extension. Since `beforeSwap` reverts unconditionally (without checking if the locker is the extension itself), the transaction fails. This results in a permanent DoS for any pool using this extension.
 ### Static Signals
revert SwapMustHappenThroughForward(), beforeSwap: true
 ### Assets at Risk
Pool liquidity (frozen)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
TWAMM Pool DoS via Virtual Order Interval Spamming
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function iterates through all initialized time intervals between the last execution time and the current block timestamp. An attacker can cheaply create many TWAMM orders with distinct end times (via `Orders.mintAndIncreaseSellAmount` with minimal amounts), populating the `poolInitializedTimesBitmapSlot` with many set bits. Once time advances past these intervals, the next user interaction with the pool will trigger the loop, which attempts to process all intervals in a single transaction. If the gas cost exceeds the block gas limit, the pool becomes permanently frozen (DoS).
 ### Static Signals
while (time != block.timestamp), searchForNextInitializedTime
 ### Assets at Risk
liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

