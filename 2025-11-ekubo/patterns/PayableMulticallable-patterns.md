## Verified Patterns Found: 8

## Verified Patterns Found in following Categories:

- SlippageMissingOrInsufficient
- AccessControlOrAuthByPass
- ForcedAssetVsStrictEquality
- AccountingInvariantViolation
- GriefableCallbacks
- DoubleExecutionOrReplay
- MulticallCrossPathReentrancy



## Summary of Patterns

Unprotected ETH Sweep via refundNativeToken

Unprotected `refundNativeToken` allows MEV sweeping of residual ETH

Unprotected refundNativeToken allows sweeping contract ETH balance

Griefable Refund causing DoS for Non-Payable Callers

Payable Multicall `msg.value` Reuse

Msg.value Replay in Payable Multicall

Msg.value Persistence in Payable Multicall

Permissionless Sweeping of Native Tokens

## Patterns



 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: PayableMulticallable.refundNativeToken

 ### Title
Unprotected ETH Sweep via refundNativeToken
 ### Description/Code Snippet
The `refundNativeToken` function sends `address(this).balance` to `msg.sender` without tracking the caller's accumulated surplus. Any ETH accidentally sent to the contract, forced via `selfdestruct`, or left over from previous incomplete batches can be swept by any arbitrary caller.
 ### Static Signals
address(this).balance usage, transfer to msg.sender, no user balance tracking
 ### Assets at Risk
stuck ETH, accidental deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: PayableMulticallable.refundNativeToken

 ### Title
Unprotected `refundNativeToken` allows MEV sweeping of residual ETH
 ### Description/Code Snippet
The `refundNativeToken` function is `external` and transfers the entire `address(this).balance` to `msg.sender`. It does not restrict refunds to the original transaction initiator. If a user accidentally sends ETH to the contract (e.g., via a direct transfer or a failed swap that left dust), or if a batch operation fails to sweep its own dust, any external actor (MEV bot) can immediately call this function to claim the funds. While intended for batch cleanup, its public visibility creates a race condition for user funds.
 ### Static Signals
external refund function, transfers address(this).balance to msg.sender, no access control
 ### Assets at Risk
User ETH, Residual Contract Balance
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: PayableMulticallable.refundNativeToken

 ### Title
Unprotected refundNativeToken allows sweeping contract ETH balance
 ### Description/Code Snippet
The `refundNativeToken` function transfers the entire `address(this).balance` to `msg.sender` without any checks on the origin or ownership of the funds. While intended to refund a user's leftover ETH from the current batch, being `external` means it can be called by anyone at any time. If the inheriting contract (e.g., a Router or TokenWrapper) holds ETH from other users, stuck transfers, or protocol fees, an attacker can simply call this function to drain the contract's entire ETH balance.
 ### Static Signals
transfers address(this).balance, no ownership check, no amount parameter, public/external visibility
 ### Assets at Risk
ETH, User Funds, Accumulated Fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: PayableMulticallable.refundNativeToken

 ### Title
Griefable Refund causing DoS for Non-Payable Callers
 ### Description/Code Snippet
The `refundNativeToken` function performs a `safeTransferETH` to `msg.sender` at the end of a flow. If `msg.sender` is a smart contract wallet that reverts on ETH receipt (or lacks a receive function), the entire multicall transaction reverts, denying service to these users.
 ### Static Signals
safeTransferETH to msg.sender, no try/catch, external call affects control flow
 ### Assets at Risk
user transaction validity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: PayableMulticallable.multicall

 ### Title
Payable Multicall `msg.value` Reuse
 ### Description/Code Snippet
The `multicall` function overrides Solady's base implementation to allow `payable` calls, bypassing the library's safety check against `msg.value` reuse. Since `delegatecall` preserves `msg.value` across loop iterations, if the inheriting contract (e.g., Router) exposes any function that credits internal accounting or performs actions based on `msg.value` (like crediting a 'saved balance' in the Till pattern) without strictly consuming the native token balance immediately, an attacker can batch multiple calls to spend the same `msg.value` multiple times.
 ### Static Signals
payable override, delegatecall loop, msg.value preservation, overrides Solady safety check
 ### Assets at Risk
Protocol Accounting, Liquidity, User Funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: DoubleExecutionOrReplay

 ### Relevant Function/Location: PayableMulticallable.multicall

 ### Title
Msg.value Replay in Payable Multicall
 ### Description/Code Snippet
The `multicall` function overrides Solady's safety check to allow `payable` execution. The underlying `_multicall` implementation uses a loop of `delegatecall`s to `address(this)`. Since `delegatecall` preserves `msg.value`, the full `msg.value` sent with the transaction is available to *every* function called in the batch. If the inheriting contract (e.g., Router) contains functions that use `msg.value` for accounting (e.g., `deposit`, `addLiquidityETH`, or internal crediting), an attacker can batch multiple calls to reuse the same ETH, potentially crediting their balance multiple times for a single payment.
 ### Static Signals
payable override, delegatecall loop, msg.value not consumed
 ### Assets at Risk
ETH, Protocol Solvency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: MulticallCrossPathReentrancy

 ### Relevant Function/Location: PayableMulticallable.multicall

 ### Title
Msg.value Persistence in Payable Multicall
 ### Description/Code Snippet
The `multicall` function overrides the base `Multicallable` check to allow `msg.value`. Since `delegatecall` preserves `msg.value` across the loop, an attacker can batch multiple calls to functions that consume ETH (e.g., `deposit` or `wrap`), effectively spending the same `msg.value` multiple times.
 ### Static Signals
payable override, delegatecall in loop, msg.value preserved
 ### Assets at Risk
protocol funds, wrapped token reserves
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: PayableMulticallable.refundNativeToken

 ### Title
Permissionless Sweeping of Native Tokens
 ### Description/Code Snippet
The `refundNativeToken` function allows any caller (`msg.sender`) to withdraw the entire ETH balance (`address(this).balance`) of the contract. While this is intended to refund the current caller's residual ETH, the lack of access control or association with the funds' origin means that any ETH left in the contract (e.g., from a user's miscalculated swap, accidental transfer, or failing to append the refund call in a batch) can be immediately swept by an MEV bot or attacker.
 ### Static Signals
no access control, transfer entire balance, msg.sender recipient
 ### Assets at Risk
User stuck funds, Accidental ETH transfers
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

