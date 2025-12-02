## Verified Patterns Found: 7

## Verified Patterns Found in following Categories:

- FlashLoanEconomicManipulation
- GriefableCallbacks
- FeeOnTransferAssumption
- AccessControlOrAuthByPass
- UntrustedDelegateCall



## Summary of Patterns

Balance-injection uses entire wallet balance, manipulable via donations/flash liquidity

Unbounded return-data copying in balance injection calls enables return-bomb griefing

Balance injection assumes non‑taxed ERC20 transfers, breaking with fee‑on‑transfer tokens

Balance-injection uses pre-transfer balance and assumes non‑taxed ERC20 semantics

Delegatecall into external Multicall3 singleton without codehash/impl validation

Public injectAndCall lets anyone drain tokens/ETH held by TrailsRouter implementation

Balance injection assumes non‑taxed tokens, breaks with fee‑on‑transfer ERC20s

## Patterns



 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: TrailsRouter.injectAndCall

 ### Title
Balance-injection uses entire wallet balance, manipulable via donations/flash liquidity
 ### Description/Code Snippet
The router’s balance-injection helpers always use the **entire current balance** of a token/ETH as the injected amount, without any cap tied to the user’s signed intent. Because anyone can freely transfer tokens/ETH to a Sequence wallet between intent signing and execution, this balance is **intra‑transaction and cross‑transaction manipulable**, enabling economic manipulation of downstream protocols.

Key code paths:

```solidity
function injectSweepAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) external payable {
    uint256 callerBalance;

    if (token == address(0)) {
        callerBalance = msg.value;
        if (callerBalance == 0) revert NoEthSent();
    } else {
        callerBalance = _getBalance(token, msg.sender);
        if (callerBalance == 0) revert NoTokensToSweep();
        _safeTransferFrom(token, msg.sender, address(this), callerBalance);
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}

function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable {
    uint256 callerBalance = _getSelfBalance(token);
    if (callerBalance == 0) {
        if (token == address(0)) {
            revert NoEthAvailable();
        } else {
            revert NoTokensToSweep();
        }
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}

function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    // ... placeholder replacement ...

    if (token == address(0)) {
        (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);

        (bool success, bytes memory result = target.call(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    }
}
```

In both `injectSweepAndCall` (standalone) and `injectAndCall` (delegatecall context), the router computes `callerBalance` as **the full token/native balance** of `msg.sender` or `address(this)` and:
- Replaces a 32‑byte placeholder in `callData` with that full balance; and
- For ETH, forwards `callerBalance` as `msg.value` to `target.call{value: callerBalance}`; or
- For ERC‑20, calls `forceApprove(erc20, target, callerBalance)` and then `target.call(callData)`.

There is **no binding** between `callerBalance` and any amount encoded in the signed intent; the Merkle leaf only fixes `amountOffset` and the placeholder bytes, not the eventual numeric value read from on-chain balance.

### Why this is a FlashLoanEconomicManipulation pattern

- **Balances are manipulable within a single transaction**:
  - For the non‑delegate version (`injectSweepAndCall`), a contract caller can flash‑mint/borrow tokens, transfer them to `msg.sender` just before calling the router, then reclaim them later in the same tx.
  - For the delegatecall version (`injectAndCall` within a Sequence wallet), *any third party* can donate tokens or ETH to the wallet between intent signing and execution, inflating `_getSelfBalance(token)` above what the user anticipated when approving the route. This is feasible on many chains and tokens.

- **Downstream logic often assumes a specific intended amount**:
  - The injected amount typically becomes a swap size, bridge amount, or deposit size in `target.call(callData)`. If an attacker artificially increases the wallet’s balance, the router blindly passes a larger value into the DEX/bridge/vault than the user expected.
  - In AMM swaps, larger trade size worsens price execution and can be combined with attacker‑controlled price manipulation on the paired pool, letting the attacker **profit** by:
    1. Front‑running to inject liquidity into the victim wallet (or flash‑mint to the wallet in the same tx if the wallet is contract‑driven), inflating `callerBalance`.
    2. Manipulating the AMM price with their own trades.
    3. Letting the victim’s `injectAndCall` route execute with the inflated amount at a bad rate.
    4. Back‑running to restore prices and capture the difference.

- **No per‑call caps or intent‑level maximums**:
  - The router exposes no explicit `maxAmount` / `expectedBalance` in these functions; the amount comes entirely from live balances. If the signed intent was constructed assuming a certain balance (e.g. 100 USDC), a later donation or flash‑injection to 10,000 USDC will cause a 100x larger trade without violating any on‑chain checks.

This fits the `FlashLoanEconomicManipulation` pattern: critical economic decisions (swap/bridge/deposit size) are based on **current pool/wallet balances**, which an attacker can skew intra‑tx or between signing and execution, without any multi‑block observation, TWAP, or hard cap enforced by the router.

Even if the attacker must temporarily “donate” funds to the wallet, they can often recover them via a companion contract (e.g., have `target` send residual funds back) or profit via price slippage on the external protocol.

### Static signals
- Uses full `balanceOf` / native `balance` as the injected amount without user‑specified maximum:
  - `_getBalance(token, msg.sender)` in `injectSweepAndCall`
  - `_getSelfBalance(token)` in `injectAndCall`
- No multi‑block observation or TWAP; directly uses spot balances and spot AMM behavior.
- Critical downstream actions (`target.call`) are parameterized by this balance‑derived value.

### Assets at risk
- User wallet’s tokens and ETH being routed through `injectAndCall` / `injectSweepAndCall` (e.g., stablecoins, bridged assets, DEX LP tokens).
- Indirectly, DEX/bridge protocol value where skewed trade size can be used for MEV/arbitrage extraction.

### Notes

The actual exploitability depends on how integrators construct Merkle trees and downstream `target` contracts (DEX/bridge adapters). However, given the generality of the router and the fact that integrators are expected to use these helpers as “send whole balance” primitives, this is a realistically exploitable **economic manipulation** pattern which higher‑level code may not defend against.
 ### Static Signals
trade amount derived from wallet balance, no user-specified max amount, target.call uses balance-derived parameter, no multi-block observation or TWAP, donation/flash-mint can skew balance intra-tx
 ### Assets at Risk
user wallet tokens, user wallet ETH, downstream DEX/bridge value
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: TrailsRouter._injectAndExecuteCall

 ### Title
Unbounded return-data copying in balance injection calls enables return-bomb griefing
 ### Description/Code Snippet
The balance injection helpers `_injectAndExecuteCall` (used by `injectSweepAndCall`, `injectAndCall`, and `_injectAndCallDelegated`) call an arbitrary `target` and copy the full return data into memory and events without any upper bound. A malicious or misconfigured target can exploit this to create a "return bomb" that consumes excessive gas or hits block gas limits when the router processes the return data.

Relevant code (TrailsRouter):

```solidity
function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    ...
    if (token == address(0)) {
        (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);

        (bool success, bytes memory result = target.call(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    }
}
```

Static signals:
- `target` is arbitrary (provided via `injectSweepAndCall`, `injectAndCall`, or via delegated entry `handleSequenceDelegateCall`).
- Calls forward essentially all remaining gas to `target.call(...)`.
- The return data `result` is allocated and copied into memory (potentially very large).
- `result` is then:
  * emitted in the `BalanceInjectorCall` event, and
  * re-used in the revert `TargetCallFailed(result)`.

A malicious `target` can deliberately return an extremely large byte array. Copying this data multiple times (once from returndata into memory, then into the event logs, then again in the revert payload) can:
- dramatically increase gas usage,
- cause out-of-gas or memory expansion failures,
- or make the entire intent execution effectively unexecutable (DoS), especially when used via Sequence delegatecall in complex multicall graphs.

Because these functions are designed as generic plumbing that will be pointed at arbitrary DEXes/bridges/protocols chosen off-chain, they should defensively bound or truncate returned data, or avoid including the full bytes in events and revert data. As written, any integration with a malicious or compromised target contract can grief relayers and users without gaining funds but still preventing successful execution.

 ### Static Signals
arbitrary target.call with all gas, bytes memory result = target.call(...), emit event including full result, revert TargetCallFailed(result) with unbounded returndata, no size limit or truncation on callback return data
 ### Assets at Risk
user intents (execution DoS), relayer gas budgets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsRouter.injectSweepAndCall / injectAndCall / pullAmountAndExecute / _injectAndExecuteCall

 ### Title
Balance injection assumes non‑taxed ERC20 transfers, breaking with fee‑on‑transfer tokens
 ### Description/Code Snippet
The balance‑injection helpers assume that ERC20 transfers move exactly the requested amount, and then reuse that pre‑transfer balance as the amount injected/approved to the downstream target. With fee‑on‑transfer or otherwise non‑standard tokens, the actual tokens received by the router (or wallet when used via delegatecall) can be less than this assumed amount. This mismatch can cause downstream calls to revert (DoS) or behave unexpectedly.

Key flows:

1. `injectSweepAndCall`
```solidity
function injectSweepAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) external payable {
    uint256 callerBalance;

    if (token == address(0)) {
        callerBalance = msg.value;
        if (callerBalance == 0) revert NoEthSent();
    } else {
        callerBalance = _getBalance(token, msg.sender);
        if (callerBalance == 0) revert NoTokensToSweep();
        _safeTransferFrom(token, msg.sender, address(this), callerBalance);
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}
```

For ERC20 tokens, `callerBalance` is computed as `IERC20(token).balanceOf(msg.sender)` (the full balance), and the router then calls `_safeTransferFrom` for that exact `callerBalance`. With a fee‑on‑transfer token, the actual amount credited to the router will be `callerBalance - fee`.

However, `callerBalance` is passed unchanged into `_injectAndExecuteCall`:

```solidity
function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    ...
    if (token == address(0)) {
        (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
        ...
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);

        (bool success, bytes memory result) = target.call(callData);
        ...
    }
}
```

In the ERC20 branch, the router:
- Approves the `target` for `callerBalance`, even though it actually holds only `callerBalance - fee` tokens.
- The downstream `target` is expected (via the placeholder replacement) to operate on `callerBalance`.

If `target` attempts to `transferFrom` the full approved `callerBalance`, the transfer will revert due to insufficient actual balance on the router, causing the entire injected call to revert and potentially breaking the user’s intent flow. The user’s tokens remain stuck on the router (or in the wallet when used via delegatecall) until some sweep logic is triggered, and the behavior is inconsistent with the nominal amount injected.

Similar assumptions appear in other helpers:

2. `injectAndCall`
```solidity
function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable {
    uint256 callerBalance = _getSelfBalance(token);
    if (callerBalance == 0) {
        if (token == address(0)) {
            revert NoEthAvailable();
        } else {
            revert NoTokensToSweep();
        }
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}
```

Here `callerBalance` is the router’s (or wallet’s) full token balance. If that balance itself was previously derived from a fee‑on‑transfer operation (e.g., via `injectSweepAndCall` or external transfers with taxes), downstream contracts relying on this injected amount will still assume a precise, fee‑free value.

3. `pullAmountAndExecute`
```solidity
function pullAmountAndExecute(address token, uint256 amount, bytes calldata data)
    public
    payable
    returns (IMulticall3.Result[] memory returnResults)
{
    _validateRouterCall(data);
    if (token == address(0)) {
        if (msg.value < amount) revert InsufficientEth(amount, msg.value);
    } else {
        _safeTransferFrom(token, msg.sender, address(this), amount);
    }

    (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
    if (!success) revert TargetCallFailed(returnData);
    return abi.decode(returnData, (IMulticall3.Result[]));
}
```

For ERC20 tokens, the function assumes `amount` tokens are received, but with a fee‑on‑transfer token, the router will receive less than `amount`. Any downstream Multicall3 sub‑calls that rely on the implicit assumption that `amount` was fully credited (e.g., a swap expecting exact input) can revert or behave incorrectly.

Impact scenarios:
- A user or dApp routes a fee‑on‑transfer token through Trails. The router overestimates the amount available for injection/approval and constructs calldata accordingly. When the target tries to pull or use that full amount, the transaction reverts, potentially halting an entire cross‑chain orchestration leg and leaving tokens stranded.
- Downstream protocols may miscalculate minimums or revert due to mismatched balances, creating a denial‑of‑service for intents involving such tokens, even though the user’s signature and off‑chain routing were valid.

The core issue is the assumption that `transferFrom` and `transfer` are 1:1 and that `balanceOf` before `transferFrom` is an accurate amount to both transfer and inject. For generic routers intended to work with arbitrary ERC20s, this is a fragile assumption and can realistically be hit by many popular taxed or fee‑on‑transfer tokens.

Mitigation ideas:
- For injection flows (`injectSweepAndCall`), compute the actual received amount using a pre/post balance delta and use that real amount for both `callerBalance` and approvals/placeholder replacement.
- In `pullAmountAndExecute`, either restrict supported tokens to non‑taxed ones (and enforce via a registry), or similarly rely on pre/post balance deltas and pass the actual credited amount down to the multicall execution.
 ### Static Signals
uses full balanceOf(sender) as amount to transfer, no balanceBefore/After delta check, assumes transferFrom(amount) credits exactly amount, approval amount equals pre-transfer balance, not actual received, payout/injection amount computed from balance without post-transfer verification
 ### Assets at Risk
user funds routed through Trails intents, cross-chain payment/swap flows, dust balances stranded in routers/wallets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsRouter.pullAndExecute / injectSweepAndCall / _injectAndExecuteCall

 ### Title
Balance-injection uses pre-transfer balance and assumes non‑taxed ERC20 semantics
 ### Description/Code Snippet
The router’s ERC20 flows assume that a transfer of `amount` tokens from the user or wallet results in the router having exactly `amount` tokens, and then propagate that `amount` onward via approvals and calldata injection. Fee‑on‑transfer / rebasing tokens break this assumption, causing mis‑accounting between the *intended* amount and the *actual* amount received.

Key locations:

1. `pullAndExecute` → `pullAmountAndExecute`
```solidity
function pullAndExecute(address token, bytes calldata data)
    public
    payable
    returns (IMulticall3.Result[] memory returnResults)
{
    uint256 amount;
    if (token == address(0)) {
        ...
    } else {
        amount = _getBalance(token, msg.sender); // balance of sender
        if (amount == 0) revert NoTokensToPull();
    }

    return pullAmountAndExecute(token, amount, data);
}

function pullAmountAndExecute(address token, uint256 amount, bytes calldata data)
    public
    payable
    returns (IMulticall3.Result[] memory returnResults)
{
    _validateRouterCall(data);
    if (token == address(0)) {
        ...
    } else {
        _safeTransferFrom(token, msg.sender, address(this), amount);
    }

    (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
    ...
}
```

Here `amount` is set to `IERC20(token).balanceOf(msg.sender)` and passed as the transfer amount. For fee‑on‑transfer tokens, `transferFrom(msg.sender, address(this), amount)` can succeed while the router receives strictly less than `amount`. Downstream calldata in `data` (e.g. approvals / swaps inside Multicall3) is constructed assuming the router actually holds `amount`, which is incorrect. This can cause:
- Over‑approval of tokens the router does not actually possess, and
- Subsequent calls that rely on `amount` (e.g. DEX/bridge calls) to revert or malfunction because the expected balance is not present.

Because `pullAmountAndExecute` doesn’t reconcile the *actual* change in balance, it blindly trusts fee‑less semantics.

2. `injectSweepAndCall` + `_injectAndExecuteCall`
```solidity
function injectSweepAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) external payable {
    uint256 callerBalance;

    if (token == address(0)) {
        ...
    } else {
        callerBalance = _getBalance(token, msg.sender); // balance of sender
        if (callerBalance == 0) revert NoTokensToSweep();
        _safeTransferFrom(token, msg.sender, address(this), callerBalance);
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}

function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    ...
    if (shouldReplace) {
        ...
        // replace placeholder with callerBalance
        assembly {
            mstore(add(add(callData, 32), amountOffset), callerBalance)
        }
    }

    if (token == address(0)) {
        (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
        ...
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);

        (bool success, bytes memory result) = target.call(callData);
        ...
    }
}
```

For ERC20 tokens, `callerBalance` is taken from `IERC20(token).balanceOf(msg.sender)` *before* the transfer and then used as:
- The transfer amount in `_safeTransferFrom(token, msg.sender, address(this), callerBalance)`; and
- The injected calldata amount; and
- The approval size in `forceApprove(erc20, target, callerBalance)`.

With fee‑on‑transfer or rebasing tokens:
- The router receives less than `callerBalance` after `transferFrom`, but still injects `callerBalance` into calldata and approves that amount.
- The target contract believes it can pull `callerBalance`, but the router’s actual balance is smaller; hooks inside `target` that rely on the passed amount will see a config that is inconsistent with on‑chain balances.

This mismatch can:
- Cause downstream calls (e.g. swaps, vault deposits) to revert because expected balance is missing, or
- Lead to subtle accounting errors if the target contract trusts the passed `amount` / allowance and does not independently check `balanceOf`.

The pattern is systemic: the code consistently uses pre‑transfer balance (`callerBalance` / `amount`) as the authoritative amount without verifying the post‑transfer delta, which assumes standard 1:1 ERC20 semantics and breaks with fee‑on‑transfer, rebasing, or otherwise non‑standard tokens.

Given Trails’ role as a generic router that may be pointed at arbitrary tokens, these assumptions are realistically exploitable when a route accidentally or maliciously involves a fee‑on‑transfer or elastic‑supply token.
 ### Static Signals
uses IERC20(token).balanceOf(msg.sender) as transfer amount, no balanceBefore/After reconciliation, approvals based on pre-transfer balance, amount parameter injected into calldata from pre-transfer balance
 ### Assets at Risk
user routed funds, Sequence wallet balances on which router is delegatecalled
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UntrustedDelegateCall

 ### Relevant Function/Location: TrailsRouter.execute / pullAmountAndExecute

 ### Title
Delegatecall into external Multicall3 singleton without codehash/impl validation
 ### Description/Code Snippet
The TrailsRouter uses `delegatecall` into an external, chain-local Multicall3 singleton at a hard-coded address without any runtime codehash or implementation validation. If that address does not contain the canonical Multicall3 implementation (e.g., on a new chain where the address is unset or where an attacker/front‑runner has deployed arbitrary code at `0xcA11bde05977b3631167028862bE2a173976CA11`), then any call into `execute` / `pullAmountAndExecute` will delegate arbitrary logic into the caller’s storage context (typically a Sequence v3 wallet).

Relevant code:

```solidity
address public immutable MULTICALL3 = 0xcA11bde05977b3631167028862bE2a173976CA11;

function execute(bytes calldata data) public payable returns (IMulticall3.Result[] memory returnResults) {
    _validateRouterCall(data);
    (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
    if (!success) revert TargetCallFailed(returnData);
    return abi.decode(returnData, (IMulticall3.Result[]));
}

function pullAmountAndExecute(address token, uint256 amount, bytes calldata data)
    public
    payable
    returns (IMulticall3.Result[] memory returnResults)
{
    _validateRouterCall(data);
    if (token == address(0)) {
        if (msg.value < amount) revert InsufficientEth(amount, msg.value);
    } else {
        _safeTransferFrom(token, msg.sender, address(this), amount);
    }

    (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
    if (!success) revert TargetCallFailed(returnData);
    return abi.decode(returnData, (IMulticall3.Result[]));
}
```

Static characteristics:
- The delegatecall target is an **external contract** whose code is not part of this repo and not verified at deployment time.
- There is **no codehash/ABI check** or other guard that the contract at `MULTICALL3` is indeed the canonical, non-upgradeable Multicall3.
- The router is designed to be `delegatecall`‑ed from Sequence wallets; thus `MULTICALL3.delegatecall` actually runs in the wallet’s storage context. A malicious contract at that address can arbitrarily mutate wallet storage, steal funds, or subvert sentinels.

Attack scenario (chain‑specific):
1. On a chain where `0xcA11…CA11` is not yet occupied by the canonical Multicall3, an attacker pre‑deploys a contract with arbitrary logic at that address (e.g., via CREATE2).
2. TrailsRouter is then deployed; its immutable `MULTICALL3` points to the attacker’s contract.
3. When a Sequence wallet delegates into TrailsRouter and calls `execute` / `pullAmountAndExecute`, the router performs `delegatecall` into the attacker contract. This executes with the wallet’s storage layout and `msg.sender` context.
4. The attacker’s code can modify wallet storage, grant approvals, or transfer assets out of the wallet while still returning data shaped like `IMulticall3.Result[]` so the router does not revert.

Because the address is hard-coded and not verified, the safety of every `execute`/`pullAmountAndExecute` call depends on an off-chain deployment convention. Any divergence (new chain, re-deployment, or malicious contract at that address) turns this into a state-hijacking vector.
 ### Static Signals
delegatecall to external hard-coded address, no codehash or implementation validation, target not enforced via allowlist or ownership, router intended to be delegatecalled from wallets, so delegatecall target runs in wallet storage context
 ### Assets at Risk
user wallet balances controlled via Sequence v3, any approvals or internal accounting in the wallet storage
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: TrailsRouter.injectAndCall

 ### Title
Public injectAndCall lets anyone drain tokens/ETH held by TrailsRouter implementation
 ### Description/Code Snippet
The `TrailsRouter` contract is designed to be mostly stateless and used via delegatecall through Sequence wallets. Sweep-style operations that move **all** of a balance (e.g. `sweep`, `refundAndSweep`, `validateOpHashAndSweep`) are correctly protected with the `onlyDelegatecall` modifier so they can only operate on the caller wallet’s storage/balance, not on the router implementation itself.

However, the balance-injection helper `injectAndCall` is declared `public` and is **not** guarded by `onlyDelegatecall`:

```solidity
function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable {
    uint256 callerBalance = _getSelfBalance(token);
    if (callerBalance == 0) {
        if (token == address(0)) {
            revert NoEthAvailable();
        } else {
            revert NoTokensToSweep();
        }
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}
```

`_getSelfBalance(token)` reads the balance of **`address(this)`**, i.e. the `TrailsRouter` implementation contract when called directly (not via delegatecall). It then passes that entire balance as `callerBalance` into `_injectAndExecuteCall`, which for ERC‑20 tokens does:

```solidity
IERC20 erc20 = IERC20(token);
SafeERC20.forceApprove(erc20, target, callerBalance);

(bool success, bytes memory result) = target.call(callData);
```

and for ETH:

```solidity
(bool success, bytes memory result) = target.call{value: callerBalance}(callData);
```

This means **any external caller** can:
- Call `injectAndCall(token, target, callData, ...)` directly on the router implementation,
- The router will read its own balance of `token` or ETH,
- Then approve `target` for that full `callerBalance` (ERC‑20) or send it as `msg.value` (ETH) and execute arbitrary `target.call(callData)`.

Therefore, **anyone can freely drain any tokens or ETH that reside on the TrailsRouter implementation address**, by choosing a `target` that pulls the approved ERC‑20 or that forwards the received native ETH to the attacker. This is a classic access control / auth bypass at the implementation level because there is no restriction (owner/role/onlyDelegatecall) on who can initiate this sweeping behaviour.

The documentation for `injectAndCall` in `ITrailsRouter` suggests it is intended for delegatecall context:

```solidity
/// @notice Injects balance and calls target (for delegatecall context).
/// @dev For delegatecalls from Sequence wallets. Reads balance from address(this).
```

But the implementation does not enforce that context. If **any tokens or ETH are accidentally sent** to `TrailsRouter` (e.g. via a misconfigured integration, failed multicall, or simple user mistake), they become trivially stealable by any attacker via `injectAndCall`.

While the protocol’s design aims for the router to remain stateless and not hold funds, this function exposes a realistic and exploitable pattern where the protocol cannot rely on the implementation address being safe to hold stray balances.

Key static signals:
- No `onlyOwner` / `onlyDelegatecall` / role check on `injectAndCall`.
- Function operates on `_getSelfBalance(token)` (router’s own balance), not the caller’s.
- Approves or transfers the **entire** router-held balance to an arbitrary `target`.
 ### Static Signals
no onlyOwner/onlyDelegatecall on injectAndCall, _getSelfBalance(address(this)) used as swept amount, forceApprove(token, target, callerBalance) to arbitrary target, target.call(callData) with callerBalance value
 ### Assets at Risk
any ERC20 tokens mistakenly sent to TrailsRouter, any ETH mistakenly sent to TrailsRouter
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsRouter.injectSweepAndCall

 ### Title
Balance injection assumes non‑taxed tokens, breaks with fee‑on‑transfer ERC20s
 ### Description/Code Snippet
The balance‑injection helpers assume that an ERC20 `transferFrom` of `amount` will credit the router with exactly `amount` tokens, and then they reuse that same `amount` (`callerBalance`) for both calldata replacement and allowance configuration. This breaks for fee‑on‑transfer / rebasing / deflationary tokens where the actual balance increase is less than the nominal transfer amount.

Relevant flow (ERC20 branch):

```solidity
function injectSweepAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) external payable {
    uint256 callerBalance;

    if (token == address(0)) {
        callerBalance = msg.value;
        if (callerBalance == 0) revert NoEthSent();
    } else {
        callerBalance = _getBalance(token, msg.sender);
        if (callerBalance == 0) revert NoTokensToSweep();
        _safeTransferFrom(token, msg.sender, address(this), callerBalance);
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}

function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    // ... optional placeholder replacement with `callerBalance` ...

    if (token == address(0)) {
        (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);

        (bool success, bytes memory result) = target.call(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    }
}
```

Static signals:
- `_safeTransferFrom(token, msg.sender, address(this), callerBalance)` assumes the full `callerBalance` is received.
- The code never recomputes the actual balance delta; it blindly trusts `callerBalance` drawn from `balanceOf(msg.sender)` *before* the transfer.
- `_injectAndExecuteCall`:
  - Replaces the placeholder in `callData` with `callerBalance`, so downstream logic encodes that value as if it were really available.
  - Uses `SafeERC20.forceApprove(erc20, target, callerBalance)` granting `target` an allowance equal to `callerBalance`.

If `token` is a fee‑on‑transfer/deflationary token, the router will receive `callerBalance - fee` tokens, but the placeholder and allowance are still set to `callerBalance`. A typical downstream protocol call inside `target` might then attempt to `transferFrom(address(this), ...)` for `callerBalance`, assuming that amount is available because the calldata and allowance say so. The transfer will fail due to insufficient balance, reverting the entire `injectSweepAndCall` flow. The user can’t adjust around this because the router forcibly injects `callerBalance` based on `msg.sender`’s pre‑transfer balance.

While this does not directly steal value, it leads to realistic DoS / unusable flows for fee‑on‑transfer tokens and misalignment between what the router encodes in calldata and what balances actually exist. It also complicates reasoning for integrators who may assume that the injected amount is always spendable.

A more robust pattern would:
- After `_safeTransferFrom`, compute the actual received amount as `balanceAfter - balanceBefore` and use that for placeholder replacement and `forceApprove`.
- Or, avoid relying on pre‑transfer `balanceOf(msg.sender)` entirely and instead let the caller pass an explicit amount that is checked against the actual received balance.

The same assumption exists in the delegatecall‑only variant `_injectAndCallDelegated`, which reads `callerBalance` from `_getSelfBalance(token)` and forwards it directly into `_injectAndExecuteCall` without reconciling against any previous accounting or transfer behavior when interacting with unusual tokens.
 ### Static Signals
uses pre-transfer balanceOf(sender) as amount, no balanceBefore/After delta check, placeholder replaced with assumed full amount, forceApprove called with assumed amount
 ### Assets at Risk
user funds routed through injectSweepAndCall, downstream protocol calls relying on injected amount
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

