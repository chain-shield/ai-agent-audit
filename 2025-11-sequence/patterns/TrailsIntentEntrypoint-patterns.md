## Verified Patterns Found: 3

## Verified Patterns Found in following Categories:

- FeeOnTransferAssumption



## Summary of Patterns

Entrypoint assumes non-fee-on-transfer ERC20 semantics for deposits and fees

Entrypoint deposit functions assume non‑deflationary ERC20 tokens

Deposits assume 1:1 ERC20 transfers, breaking intents for fee-on-transfer tokens

## Patterns



 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsIntentEntrypoint.depositToIntentWithPermit

 ### Title
Entrypoint assumes non-fee-on-transfer ERC20 semantics for deposits and fees
 ### Description/Code Snippet
The TrailsIntentEntrypoint contract assumes standard 1:1 ERC20 transfer semantics and does not account for fee-on-transfer, rebasing, or otherwise non-standard tokens when moving funds from the user to the intent address or fee collector.

In both `depositToIntentWithPermit` and `depositToIntent`, the contract pulls tokens from the user using the requested `amount` and `feeAmount` parameters, but never verifies the actual balance changes on the sender or recipients:

```solidity
function depositToIntentWithPermit(...) external nonReentrant {
    _verifyAndMarkIntent(...);

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
}

function depositToIntent(...) external nonReentrant {
    _verifyAndMarkIntent(...);

    IERC20(token).safeTransferFrom(user, intentAddress, amount);

    if (feeAmount > 0 && feeCollector != address(0)) {
        IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
        emit FeePaid(user, token, feeAmount, feeCollector);
    }

    emit IntentDeposit(user, intentAddress, amount);
}
```

Static symptoms:
- The contract uses `SafeERC20.safeTransferFrom(user, intentAddress, amount)` and `safeTransferFrom(user, feeCollector, feeAmount)` and then assumes that exactly `amount` and `feeAmount` tokens arrived.
- There is no `balanceBefore / balanceAfter` check on either the user, the intent address, or the fee collector.
- The `IntentDeposit` and `FeePaid` events emit the requested `amount` / `feeAmount`, not the actual token balances received.

If a fee-on-transfer or deflationary token is used:
- The user may be debited `amount` from their balance but the intent address receives less than `amount`, causing downstream routing/bridging logic (which expects full `amount`) to misbehave or fail.
- The protocol’s fee collector may receive less than the emitted `feeAmount` while the event still reports the full value, making on-chain analytics and accounting misleading.

If a malicious or non-conforming ERC20 reports success but doesn’t move the full amount, intents can be marked as funded (off-chain and via events) when the actual on-chain balance at the intent address is lower than expected.

Because the contract aims to support arbitrary ERC20 tokens (including via permit), and emits values based on the input parameters instead of the actual token flow, this is a classic fee-on-transfer / non-standard token assumption that can lead to accounting inconsistencies and confusing or misleading state for integrators and users.
 ### Static Signals
uses SafeERC20.safeTransferFrom(user, intentAddress, amount) without balance delta check, uses SafeERC20.safeTransferFrom(user, feeCollector, feeAmount) without balance delta check, events IntentDeposit and FeePaid emit requested amount values, not actual received, no handling for fee-on-transfer or rebasing token behavior
 ### Assets at Risk
user intent deposits, protocol fee revenue, accuracy of on-chain accounting and analytics for deposits and fees
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsIntentEntrypoint.depositToIntentWithPermit / depositToIntent

 ### Title
Entrypoint deposit functions assume non‑deflationary ERC20 tokens
 ### Description/Code Snippet
Both `depositToIntentWithPermit` and `depositToIntent` assume that `IERC20(token).safeTransferFrom(user, ...)` moves exactly the `amount` or `feeAmount` specified, without accounting for fee‑on‑transfer or rebasing behaviour.

Relevant code:

```solidity
function depositToIntentWithPermit(
    address user,
    address token,
    uint256 amount,
    uint256 permitAmount,
    address intentAddress,
    uint256 deadline,
    uint256 nonce,
    uint256 feeAmount,
    address feeCollector,
    ...
) external nonReentrant {
    _verifyAndMarkIntent(
        user, token, amount, intentAddress, deadline, nonce, feeAmount, feeCollector, sigV, sigR, sigS
    );

    unchecked {
        if (permitAmount != amount + feeAmount) revert PermitAmountMismatch();
    }

    IERC20Permit(token).permit(user, address(this), permitAmount, deadline, permitV, permitR, permitS);
    IERC20(token).safeTransferFrom(user, intentAddress, amount);

    if (feeAmount > 0 && feeCollector != address(0)) {
        IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
        emit FeePaid(user, token, feeAmount, feeCollector);
    }

    emit IntentDeposit(user, intentAddress, amount);
}

function depositToIntent(
    address user,
    address token,
    uint256 amount,
    address intentAddress,
    uint256 deadline,
    uint256 nonce,
    uint256 feeAmount,
    address feeCollector,
    ...
) external nonReentrant {
    _verifyAndMarkIntent(
        user, token, amount, intentAddress, deadline, nonce, feeAmount, feeCollector, sigV, sigR, sigS
    );

    IERC20(token).safeTransferFrom(user, intentAddress, amount);

    if (feeAmount > 0 && feeCollector != address(0)) {
        IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
        emit FeePaid(user, token, feeAmount, feeCollector);
    }

    emit IntentDeposit(user, intentAddress, amount);
}
```

There is no balanceBefore/balanceAfter check around the transfers, and no handling for fee‑on‑transfer / rebasing tokens. If a deflationary or tax token is used, the `intentAddress` and `feeCollector` will receive fewer tokens than the signed `amount` / `feeAmount`. The EIP‑712 intent encodes the nominal `amount`, and off‑chain routing/settlement logic may assume that full amount is available at the intent address. This desync can:

* Cause downstream intent execution (swaps/bridges in the Sequence wallet) to revert because less than the expected amount arrived.
* Lead to systematic underpayment of protocol fees (fee collector receives less than `feeAmount`).
* Create subtle accounting/monitoring discrepancies between what on‑chain events (`IntentDeposit`, `FeePaid`) state and the actual balances moved.

Because the entrypoint is meant to be generic over ERC‑20 tokens, and nothing in the contract/documentation enforces that tokens must be non‑deflationary, this is a realistic FeeOnTransferAssumption pattern that can break integrations when such tokens are used.
 ### Static Signals
uses transferFrom(amount) without balance delta check, no handling for fee-on-transfer or rebasing tokens, accounting/events based on input amount, not actual received amount
 ### Assets at Risk
user deposits at intent addresses, protocol fee revenue, downstream swap/bridge flows relying on full deposit amount
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsIntentEntrypoint.depositToIntentWithPermit, depositToIntent

 ### Title
Deposits assume 1:1 ERC20 transfers, breaking intents for fee-on-transfer tokens
 ### Description/Code Snippet
The TrailsIntentEntrypoint contract assumes standard ERC-20 behavior where `transferFrom` moves exactly the requested `amount`, with no tax, burn, or rebasing side effects. No balance-before/balance-after checks are performed to confirm how many tokens were actually received by the intent address or the fee collector.

In both `depositToIntentWithPermit` and `depositToIntent`, the contract transfers `amount` to the intent address and then (optionally) `feeAmount` to the fee collector:

```solidity
IERC20(token).safeTransferFrom(user, intentAddress, amount);

// Pay fee if specified (fee token is same as deposit token)
if (feeAmount > 0 && feeCollector != address(0)) {
    IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
    emit FeePaid(user, token, feeAmount, feeCollector);
}

emit IntentDeposit(user, intentAddress, amount);
```

No check is made that `IERC20(token).balanceOf(intentAddress)` increased by exactly `amount`, nor that `feeCollector` actually received `feeAmount`. For fee-on-transfer / deflationary tokens, hooks, or malicious ERC-20s that deduct or redirect part of the transfer, the actual balances at the intent address and fee collector will differ from `amount` and `feeAmount`.

This can cause:
- The intent address to receive fewer tokens than the signed `amount`, potentially breaking downstream routing/bridging assumptions (e.g., insufficient balance for the planned route).
- The `FeePaid` and `IntentDeposit` events to log values that do not match actual received amounts, misleading off-chain accounting and monitoring.
- Extra value loss for users when tokens charge fees on each of the two transfers (deposit + fee), while off-chain solvers/orchestrators assume 1:1 transfers.

Because `token` is only validated to be non-zero and is otherwise arbitrary, this pattern can manifest for any non-standard ERC-20 or fee-on-transfer token used as the deposit/fee token.
 ### Static Signals
uses transferFrom(amount) without balance delta check, no handling for fee-on-transfer or rebasing tokens, events emit requested amount, not actual received amount, token address is arbitrary user-supplied ERC20
 ### Assets at Risk
user deposits, intent execution funds, protocol fee flows
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

