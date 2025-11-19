## Verified Patterns Found: 5

## Verified Patterns Found in following Categories:

- FeeOnTransferAssumption



## Summary of Patterns

Permit-based deposits also assume exact ERC20 transfers, ignoring token fees

Intents assume non-fee-on-transfer tokens for deposits and fees

Entrypoint assumes 1:1 ERC20 transfers, incompatible with fee-on-transfer tokens

Intent deposits assume non-fee-on-transfer tokens and may mis-handle taxed tokens

Entrypoint assumes 1:1 ERC20 transfers, breaking with fee-on-transfer/rebasing tokens

## Patterns



 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsIntentEntrypoint.depositToIntentWithPermit

 ### Title
Permit-based deposits also assume exact ERC20 transfers, ignoring token fees
 ### Description/Code Snippet
`depositToIntentWithPermit` repeats the same 1:1 transfer assumption as `depositToIntent`, but in a path explicitly designed for arbitrary EIP-2612 tokens, increasing the likelihood of interaction with non-standard or fee-on-transfer tokens. After verifying the intent and checking `permitAmount == amount + feeAmount`, it does:

```solidity
IERC20Permit(token).permit(user, address(this), permitAmount, deadline, permitV, permitR, permitS);
IERC20(token).safeTransferFrom(user, intentAddress, amount);

if (feeAmount > 0 && feeCollector != address(0)) {
    IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
    emit FeePaid(user, token, feeAmount, feeCollector);
}

emit IntentDeposit(user, intentAddress, amount);
```

There is still no balance delta check around the transfers, and the emitted events assume full `amount`/`feeAmount` delivery. For fee-on-transfer or elastic-supply tokens, this leads to the same misalignment between:
- What the signature and events say was deposited (`amount`, `feeAmount`), and
- What the intent address and fee collector actually receive on-chain.

Because this path is the primary UX for "pay gas with any permit-compatible token", it is particularly likely to be used with exotic tokens. Any such token that takes a transfer fee or burns on transfer can underfund the intent, causing downstream execution failures or inconsistent accounting.
 ### Static Signals
relies on IERC20Permit.permit + transferFrom without balance delta checks, no special handling for fee-on-transfer or rebasing tokens, events and off-chain assumptions use nominal amount instead of actual received
 ### Assets at Risk
user deposits to intent addresses, protocol fee revenue, cross-chain execution that assumes fully funded intents
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsIntentEntrypoint.depositToIntent / depositToIntentWithPermit

 ### Title
Intents assume non-fee-on-transfer tokens for deposits and fees
 ### Description/Code Snippet
Both `depositToIntent` and `depositToIntentWithPermit` assume that ERC-20 transfers are 1:1 (the amount requested equals the amount actually received), and never reconcile balances before/after transfers. This breaks when used with fee-on-transfer, rebasing, or otherwise elastic-supply tokens.

Flow in `depositToIntentWithPermit`:
```solidity
_verifyAndMarkIntent(
    user, token, amount, intentAddress, deadline, nonce, feeAmount, feeCollector, sigV, sigR, sigS
);

unchecked {
    if (permitAmount != amount + feeAmount) revert PermitAmountMismatch();
}

IERC20Permit(token).permit(user, address(this), permitAmount, ...);
IERC20(token).safeTransferFrom(user, intentAddress, amount);

if (feeAmount > 0 && feeCollector != address(0)) {
    IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
    emit FeePaid(user, token, feeAmount, feeCollector);
}

emit IntentDeposit(user, intentAddress, amount);
```

Similarly in `depositToIntent`:
```solidity
_verifyAndMarkIntent(...);
IERC20(token).safeTransferFrom(user, intentAddress, amount);
if (feeAmount > 0 && feeCollector != address(0)) {
    IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
    emit FeePaid(user, token, feeAmount, feeCollector);
}
emit IntentDeposit(user, intentAddress, amount);
```

Static signals:
- The contract credits the intent address with the signed `amount` and charges the user `amount + feeAmount`, but never checks how many tokens the intent address or `feeCollector` actually receive.
- No `balanceBefore` / `balanceAfter` checks are used around `safeTransferFrom`.
- There is no restriction in the contract that tokens must be non-fee-on-transfer or non-rebasing.

If a fee-on-transfer token is used:
- `safeTransferFrom(user, intentAddress, amount)` will often reduce the sender by `amount` but credit the intent address with less (e.g., `amount * (1 - fee)`), so intent execution can be underfunded and later steps (swaps/bridges) may revert.
- The `FeePaid` event emits `feeAmount` even though the `feeCollector` may receive less due to transfer fees.
- Because `_verifyAndMarkIntent` has already incremented the user nonce and marked the digest as used, the user cannot re-run the same signed intent with a different route; funds may end up stranded in the intent wallet (requiring manual recovery) whenever downstream execution fails due to underfunding.

This matches the FeeOnTransferAssumption pattern: accounting is based on the nominal `amount` and `feeAmount` parameters, not on the actual post-transfer balances. Any non-standard token (fee-on-transfer, rebasing, elastic supply) can cause protocol behavior to diverge from user expectations or from off-chain routing assumptions.
 ### Static Signals
uses SafeERC20.safeTransferFrom without balanceBefore/After check, assumes transferFrom(amount) credits exactly amount, events emit requested amounts, not actual received, no restriction or detection for fee-on-transfer / rebasing tokens
 ### Assets at Risk
user deposits, intent wallet balances, fee payments to feeCollector
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsIntentEntrypoint.depositToIntentWithPermit / depositToIntent

 ### Title
Entrypoint assumes 1:1 ERC20 transfers, incompatible with fee-on-transfer tokens
 ### Description/Code Snippet
TrailsIntentEntrypoint's deposit functions assume that ERC20 transfers move exactly the requested `amount` and `feeAmount`, without verifying the actual balance delta. In `depositToIntentWithPermit` and `depositToIntent`, the contract calls `IERC20(token).safeTransferFrom(user, intentAddress, amount);` and, when a fee is configured, `IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);` but never checks how many tokens were actually received by `intentAddress` or `feeCollector`. No balance-before/after accounting is performed.

For fee-on-transfer, rebasing, or otherwise non-standard tokens, the actual amount credited can be less than `amount`/`feeAmount`. The EIP-712 intent, off-chain routing logic, and emitted `IntentDeposit` / `FeePaid` events will reflect the nominal amounts, not the real balances:
- `depositToIntentWithPermit`: assumes `permitAmount == amount + feeAmount` and then performs two transfers that may each be taxed or adjusted by the token.
- `depositToIntent`: similarly uses `safeTransferFrom` with the requested amounts, with no adjustment.

This mismatch can cause downstream routers or off-chain components that rely on the signed `amount` and `feeAmount` to overestimate how many tokens are available at the intent address, leading to failed executions, stuck flows, or silent shortfalls when integrating non-standard tokens.
 ### Static Signals
uses SafeERC20.safeTransferFrom(user, intentAddress, amount) without balance delta check, uses SafeERC20.safeTransferFrom(user, feeCollector, feeAmount) without balance delta check, no handling for fee-on-transfer or rebasing tokens, events emit requested amount, not actual received amount
 ### Assets at Risk
user deposits, intent address balances, downstream routing assumptions
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsIntentEntrypoint.depositToIntent

 ### Title
Intent deposits assume non-fee-on-transfer tokens and may mis-handle taxed tokens
 ### Description/Code Snippet
Both `depositToIntent` and `depositToIntentWithPermit` assume that ERC‑20 transfers are 1:1 and do not account for fee-on-transfer/deflationary or rebasing token behavior.

In `depositToIntentWithPermit`:
```solidity
IERC20Permit(token).permit(user, address(this), permitAmount, deadline, permitV, permitR, permitS);
IERC20(token).safeTransferFrom(user, intentAddress, amount);

// Pay fee if specified (fee token is same as deposit token)
if (feeAmount > 0 && feeCollector != address(0)) {
    IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
    emit FeePaid(user, token, feeAmount, feeCollector);
}

emit IntentDeposit(user, intentAddress, amount);
```

In `depositToIntent`:
```solidity
IERC20(token).safeTransferFrom(user, intentAddress, amount);

// Pay fee if specified (fee token is same as deposit token)
if (feeAmount > 0 && feeCollector != address(0)) {
    IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
    emit FeePaid(user, token, feeAmount, feeCollector);
}

emit IntentDeposit(user, intentAddress, amount);
```

Static signals of the pattern:
- The contract uses the *requested* `amount` and `feeAmount` as the semantic deposit/fee amounts, but never checks the actual token balance deltas of `intentAddress` or `feeCollector`.
- Events `IntentDeposit` and `FeePaid` emit the input `amount` / `feeAmount` without verifying how many tokens were actually received.
- There is no balance-before / balance-after accounting, and the contract does not attempt to detect or forbid fee-on-transfer or rebasing tokens.

If a fee-on-transfer token is used, `safeTransferFrom(user, intentAddress, amount)` can:
- deduct a transfer fee from the user,
- credit only `amount - fee` to `intentAddress`,
- optionally route the fee to some third-party address.

The `IntentDeposit` event will still claim that `amount` tokens were deposited, while the intent contract may only receive `amount - fee`. Off-chain solvers, routes, or downstream contracts that assume the on-chain event matches actual balances can overestimate available liquidity at the intent address, leading to route failures, stuck flows, or inconsistent accounting between off-chain orchestration and on-chain state.

Similarly, the `FeePaid` event may over-report fees actually reaching `feeCollector` for deflationary tokens, causing protocol-side accounting or revenue tracking discrepancies.

While the entrypoint itself remains non-custodial and does not maintain internal balances, this assumption can realistically break higher-level invariants (e.g., "intent address has at least `amount` tokens"), especially if integrators treat `IntentDeposit.amount` as authoritative rather than checking actual token balances.
 ### Static Signals
uses SafeERC20.safeTransferFrom without balance delta checks, events emit requested amount, not actual received amount, no guard or detection for fee-on-transfer/rebasing tokens, no balanceBefore/balanceAfter accounting around transfers
 ### Assets at Risk
user deposits at intentAddress, fee revenue accuracy for feeCollector, off-chain routing assumptions about available liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsIntentEntrypoint.depositToIntent, depositToIntentWithPermit

 ### Title
Entrypoint assumes 1:1 ERC20 transfers, breaking with fee-on-transfer/rebasing tokens
 ### Description/Code Snippet
Both deposit functions treat the requested `amount` and `feeAmount` as if the ERC20 token will transfer those exact values, without checking the actual balance changes. For example, `depositToIntentWithPermit` does:
- `_verifyAndMarkIntent(...)` (no token interaction)
- `IERC20Permit(token).permit(user, address(this), permitAmount, ...)`
- `IERC20(token).safeTransferFrom(user, intentAddress, amount);`
- optionally `IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);`

`depositToIntent` similarly calls:
- `IERC20(token).safeTransferFrom(user, intentAddress, amount);`
- optionally `IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);`

In both cases there is no `balanceBefore/balanceAfter` check on the intent address or feeCollector, and events (`IntentDeposit(user, intentAddress, amount)` and `FeePaid(user, token, feeAmount, feeCollector)`) use the nominal `amount` / `feeAmount` rather than the actual amounts received.

If `token` is fee-on-transfer, rebasing, or otherwise non-standard (e.g., burns a portion on transfer), the intent address may receive fewer tokens than `amount`, and the feeCollector fewer than `feeAmount`. Off-chain/intents logic and external integrators will still treat the deposit as `amount`, potentially leaving the intent underfunded relative to the signed EIP-712 intent. This can cause downstream bridges/swaps executed from the intent address to be underfunded or to revert, leaving user funds stranded or flows DoS’d for certain tokens. The contract thus implicitly assumes standard 1:1 ERC20 semantics and may misbehave or become incompatible with fee-on-transfer/rebasing tokens.
 ### Static Signals
uses SafeERC20.safeTransferFrom(user, intentAddress, amount) without balance delta checks, events emit the requested amount, not the actual received amount, no handling for fee-on-transfer or rebasing tokens, accounting / expectations based on transfer parameter, not actual balance change
 ### Assets at Risk
user deposits, intent address funding, fee payments to feeCollector, downstream bridge/swap operations funded from the intent
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

