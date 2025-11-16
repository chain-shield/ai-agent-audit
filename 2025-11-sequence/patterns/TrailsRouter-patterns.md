## Verified Patterns Found: 8

## Verified Patterns Found in following Categories:

- FeeOnTransferAssumption
- DoubleExecutionOrReplay
- UntrustedDelegateCall
- AccessControlOrAuthByPass
- MaturityorGatingByPass
- ExternalCallAfterStateChange
- StorageCollisionOrSelectorClash



## Summary of Patterns

Balance injection misaccounts fee-on-transfer tokens via pre-transfer balance

ERC20 target receives full-balance allowance that is never revoked

Balance injection grants large ERC20 allowances before external call and never clears them

Multicall allowFailure gating bypass via injectAndCall and sentinel sweep

Delegatecall to fixed Multicall3 address without codehash verification

injectAndCall lets anyone approve and drain entire router token balance

Tstorish __activateTstore can corrupt caller storage when router is delegatecalled

OpHash success sentinel not consumed, enabling repeated validateOpHashAndSweep sweeps

## Patterns



 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TrailsRouter.injectSweepAndCall

 ### Title
Balance injection misaccounts fee-on-transfer tokens via pre-transfer balance
 ### Description/Code Snippet
The balance injection helpers assume that ERC20 transfers are 1:1 and do not account for fee-on-transfer or deflationary tokens.

In `injectSweepAndCall`, for ERC20 tokens the router derives an amount from the *pre-transfer* balance of the caller, then attempts to pull exactly that amount and passes it through as the injected amount:

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

`callerBalance` is computed as `IERC20(token).balanceOf(msg.sender)` and then blindly used as the `amount` for `safeTransferFrom`. If the token is fee-on-transfer or otherwise deflationary, the router will receive *less* than `callerBalance`. However, `_injectAndExecuteCall` still uses `callerBalance` for both calldata replacement and approvals:

```solidity
function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    // ... placeholder replacement with callerBalance ...

    if (token == address(0)) {
        (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
        if (!success) revert TargetCallFailed(result);
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);

        (bool success, bytes memory result) = target.call(callData);
        if (!success) revert TargetCallFailed(result);
    }
}
```

Static Fee-on-transfer assumptions:
- The code assumes `safeTransferFrom(token, msg.sender, this, callerBalance)` credits exactly `callerBalance` tokens to the router.
- There is no balance-before/balance-after check to compute the actual amount received.
- The injected amount and ERC20 approval (`forceApprove`) use the *requested* amount rather than the real balance.

Likely consequences with fee-on-transfer tokens:
- The injected amount in calldata can overstate the router’s actual balance, causing downstream protocol calls that assume the placeholder reflects real available funds to revert, or to misbehave economically.
- The router sets an allowance equal to `callerBalance` even though it actually holds fewer tokens. Downstream contracts may rely on that allowance and attempt `transferFrom` for the full amount, leading to reverts or inconsistent behavior.
- For complex multicall/intents flows, these misassumptions can cause unexpected failures or dust imbalances whenever a non-vanilla ERC20 is used.

This pattern also appears (though with slightly different impact) wherever `_safeTransferFrom` is used with a caller-provided `amount` that is subsequently assumed to have been fully received (e.g. in `pullAmountAndExecute`).
 ### Static Signals
uses pre-transfer balance as transfer amount, no balanceBefore/After check around safeTransferFrom, approval and calldata placeholder use requested amount, not actual received
 ### Assets at Risk
user deposits, bridge or DEX leg inputs, intent wallet token balances
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: TrailsRouter._injectAndExecuteCall

 ### Title
ERC20 target receives full-balance allowance that is never revoked
 ### Description/Code Snippet
In TrailsRouter, ERC20 balance injection for arbitrary targets is implemented in `_injectAndExecuteCall`, which is reached via `injectSweepAndCall`, `injectAndCall`, and `_injectAndCallDelegated`.

Key code:
- `injectSweepAndCall` (external):
  `callerBalance = _getBalance(token, msg.sender);`
  `... _safeTransferFrom(token, msg.sender, address(this), callerBalance);`
  `... _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);`

- `injectAndCall` / `_injectAndCallDelegated` (delegatecall path):
  `uint256 callerBalance = _getSelfBalance(token);`
  `... _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);`

- `_injectAndExecuteCall` ERC20 branch:
  `IERC20 erc20 = IERC20(token);`
  `SafeERC20.forceApprove(erc20, target, callerBalance);`
  `(bool success, bytes memory result) = target.call(callData);`

Here, `callerBalance` is computed as the *entire* balance of `token` held by the caller (`_getBalance(token, msg.sender)` in the non-delegatecall path) or the entire balance held by `address(this)` in the current context (`_getSelfBalance(token)`, which under delegatecall is the full wallet balance). That full balance is then granted as allowance to the arbitrary `target` via `forceApprove`, and this allowance is **not revoked or reduced** by the router after `target.call` returns.

If the `target` contract only consumes part of the allowance during the injected call (or even none), the remaining allowance persists. When TrailsRouter is delegatecalled from a Sequence wallet, this leaves `target` with a standing ERC20 allowance on the **wallet itself** for up to the full balance of `token`. The target can later call `transferFrom` directly (outside the TrailsRouter flow) to pull additional tokens from the wallet without going through the Merkle-tree–authorised intent or router-level checks.

Because `target` is fully caller-controlled and intended to be arbitrary (DEXes, bridges, etc.), any compromised or malicious target, or one whose logic changes over time, can abuse this lingering allowance to drain user funds beyond what was intended for the single Trails operation.
 ### Static Signals
forceApprove used with amount = _getSelfBalance(token) or full msg.sender balance, no allowance reset to 0 after target.call, target address is fully user-controlled, router used via delegatecall so allowance is on wallet, not on isolated router
 ### Assets at Risk
user wallet ERC20 balances, router-held ERC20 balances during injectSweepAndCall flows
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ExternalCallAfterStateChange

 ### Relevant Function/Location: TrailsRouter._injectAndExecuteCall

 ### Title
Balance injection grants large ERC20 allowances before external call and never clears them
 ### Description/Code Snippet
In the ERC20 branch of `_injectAndExecuteCall`, the router mutates external token state (approval) before performing a low-level call to an arbitrary `target`, and it never clears or rolls back that approval.

Relevant flow:
- `callerBalance` is computed as the full token balance of the calling context via `_getSelfBalance(token)` or `_getBalance(token, msg.sender)`.
- For ERC20 tokens, the function executes `SafeERC20.forceApprove(erc20, target, callerBalance);` and only then does `target.call(callData)`.
- If `target` reverts or behaves maliciously, the increased allowance on the ERC20 token contract remains in place because it is an external state change; there is no subsequent `approve(0)` or other cleanup.

When `TrailsRouter` is used via `delegatecall` from a Sequence wallet, this approval is from the wallet to the arbitrary `target` specified in the intent. A compromised or malicious `target` contract can later call `transferFrom` using this leftover allowance (even if the original route reverted or only partially executed) and drain up to `callerBalance` tokens from the wallet. Because `callerBalance` is the entire token balance of the wallet in many call paths (e.g., `injectAndCall` / `_injectAndCallDelegated`), this can expose all of the wallet's holdings of that token.

There is no restriction in `TrailsRouter` on what `target` can be, and no allowance revocation after the call. Callers reach `_injectAndExecuteCall` via `injectSweepAndCall`, `injectAndCall`, and `_injectAndCallDelegated` (used from `handleSequenceDelegateCall`), so both direct usage and delegated-extension usage can leave dangerous residual approvals on failure or misconfiguration.
 ### Static Signals
ERC20 allowance updated via SafeERC20.forceApprove before external call, target is arbitrary address passed as parameter, no allowance reset (approve(0)) after call, no try/catch around target.call, used from delegatecall context so approvals come from wallet, not router
 ### Assets at Risk
User ERC20 balances held by Sequence wallets using TrailsRouter, Tokens approved to arbitrary external targets via balance injection
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: TrailsRouter._injectAndExecuteCall

 ### Title
Multicall allowFailure gating bypass via injectAndCall and sentinel sweep
 ### Description/Code Snippet
TrailsRouter enforces a strict invariant for Multicall3 usage in the `execute` / `pull*` paths: all underlying calls in `aggregate3Value` must have `allowFailure == false`, so any failed subcall reverts the whole multicall. This is implemented in `_validateRouterCall`:

```solidity
function _validateRouterCall(bytes memory callData) internal pure {
    if (callData.length < 4) revert InvalidFunctionSelector(bytes4(0));

    bytes4 selector;
    assembly {
        selector := mload(add(callData, 32))
    }

    // Only allow `aggregate3Value` calls (0x174dea71)
    if (selector != 0x174dea71) {
        revert InvalidFunctionSelector(selector);
    }

    IMulticall3.Call3Value[] memory calls = abi.decode(_sliceCallData(callData, 4), (IMulticall3.Call3Value[]));

    for (uint256 i = 0; i < calls.length; i++) {
        if (calls[i].allowFailure) {
            revert AllowFailureMustBeFalse(i);
        }
    }
}
```

This check is applied only in `execute`, `pullAndExecute`, and `pullAmountAndExecute` before the `delegatecall` to `MULTICALL3`:

```solidity
function execute(bytes calldata data) public payable returns (IMulticall3.Result[] memory) {
    _validateRouterCall(data);
    (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
    if (!success) revert TargetCallFailed(returnData);
    return abi.decode(returnData, (IMulticall3.Result[]));
}
```

However, the **balance injection** paths (`injectSweepAndCall`, `injectAndCall`, and the delegated entry `_injectAndCallDelegated`) all funnel into `_injectAndExecuteCall`, which performs an unconstrained external call to an arbitrary `target` with arbitrary `callData` and does **not** invoke `_validateRouterCall`:

```solidity
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

        (bool success, bytes memory result) = target.call(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    }
}
```

As a result, a solver/relayer or integrator can set:

- `target = MULTICALL3` (the same Multicall3 instance), and
- `callData` = ABI‑encoded `aggregate3Value(Call3Value[] calls)` where some `calls[i].allowFailure == true`.

In this scenario:

1. `_injectAndExecuteCall` sees only the outer `target.call` to Multicall3. As long as Multicall3 itself does not revert, `success == true`, so `_injectAndExecuteCall` returns normally.
2. Inside Multicall3, any `Call3Value` entries with `allowFailure == true` can **fail silently** (their individual `success` is set false in the returned `Result[]`, but the whole `aggregate3Value` call does not revert).
3. Because `_injectAndExecuteCall` only checks the outer `success`, the delegated extension (TrailsRouterShim) will treat this operation as a success and set the success sentinel for the corresponding `opHash`.
4. Later, `validateOpHashAndSweep(opHash, token, recipient)` will see the sentinel set and allow sweeping of remaining balances:

```solidity
function validateOpHashAndSweep(bytes32 opHash, address _token, address _recipient)
    public
    payable
    onlyDelegatecall
{
    uint256 slot = TrailsSentinelLib.successSlot(opHash);
    if (_getTstorish(slot) != TrailsSentinelLib.SUCCESS_VALUE) {
        revert SuccessSentinelNotSet();
    }
    sweep(_token, _recipient);
}
```

This breaks the intended invariant that sweeps/refunds/next steps only occur if all prerequisite calls (e.g., swaps, bridge sends, or other protocol interactions encoded inside Multicall3) **actually succeeded**. A malicious or buggy route could:

- Encode a critical step (e.g., a swap or bridge send) as a `Call3Value` with `allowFailure = true`.
- Have that internal call revert, while Multicall3 still returns `success = true` at the top level.
- Cause TrailsRouter to consider the whole op as successful, set the sentinel via the shim, and then allow `validateOpHashAndSweep` to sweep funds or collect fees based on an operation that in fact only partially executed.

Assets at risk are the balances held in the user’s intent/wallet context during the Trails flow (the full `callerBalance` injected via `_injectAndExecuteCall`) and any balances later swept to fee collectors or destination addresses. The effect is that partial or failed internal operations inside Multicall3 can be incorrectly treated as fully successful, undermining the protocol’s "all‑or‑nothing" execution guarantees for certain routes that use balance injection with Multicall3 as the target.
 ### Static Signals
no _validateRouterCall on injectAndCall paths, target.call with arbitrary callData, can set target=MULTICALL3, Multicall3.aggregate3Value supports allowFailure=true, success sentinel checked only at outer call level
 ### Assets at Risk
user intent wallet balances, bridged or swapped funds, protocol fee sweeps
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UntrustedDelegateCall

 ### Relevant Function/Location: TrailsRouter.execute

 ### Title
Delegatecall to fixed Multicall3 address without codehash verification
 ### Description/Code Snippet
The TrailsRouter delegates execution to a hard‑coded Multicall3 contract at `0xcA11bde05977b3631167028862bE2a173976CA11` without any on‑chain verification that this address actually contains the expected canonical Multicall3 implementation.

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

`_validateRouterCall` ensures only `aggregate3Value` is invoked and `allowFailure == false`, but it does **not** confirm that `MULTICALL3` actually contains the genuine Multicall3 bytecode (no codehash check, no minimal interface check).

When TrailsRouter is used via `delegatecall` from a Sequence v3 wallet (the intended path), these `delegatecall`s to `MULTICALL3` effectively execute arbitrary code at `0xcA11...` **in the wallet’s storage context**. If, on a given chain, an unexpected or malicious contract is deployed at this address (e.g., on a new or non‑canonical chain where the Multicall3 singleton has not been pre‑deployed, or is replaced on a fork), that contract can:

- Mutate the wallet’s storage (owners, modules, sentinels, balances) arbitrarily.
- Perform arbitrary external calls while masquerading as the wallet.
- Drain or lock funds held by the wallet / intent account.

Because the address is hard‑coded and immutable, there is no escape hatch if the wrong code exists at this address on a particular network; all calls through `execute` / `pullAmountAndExecute` on that chain would be compromised. The contract fully trusts the environment to have the right implementation deployed at that address, but does not enforce this assumption on‑chain.

This matches the **UntrustedDelegateCall** pattern:
- A `delegatecall` is made to an external contract whose safety is assumed but not verified.
- No codehash/bytecode/ownership checks or allow‑listing beyond the fixed address.
- In delegatecall context (via Sequence wallets), this gives that external contract full control over the caller’s storage and funds.

While the intention is clearly to rely on the well‑known canonical Multicall3 deployment, the absence of any on‑chain verification leaves room for misconfiguration or malicious deployments on non‑standard networks, making this a realistic, if environment‑dependent, risk.
 ### Static Signals
delegatecall to external contract, delegatecall target is hard-coded address 0xcA11... without codehash check, _validateRouterCall only restricts selector/params, not implementation, no allowlist or upgrade safety around delegatecall target
 ### Assets at Risk
user funds in Sequence wallets / intent accounts, wallet storage and configuration
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: TrailsRouter.injectAndCall

 ### Title
injectAndCall lets anyone approve and drain entire router token balance
 ### Description/Code Snippet
The TrailsRouter exposes injectAndCall as a public function without any delegatecall or role-based restriction, even though the documentation says it is "for delegatecall context".

Code paths:

1) Public entry point (no auth/guard):

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

2) Internal execution for ERC20 tokens:

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

Because injectAndCall uses _getSelfBalance(token), it always operates on the **entire token or ETH balance of the TrailsRouter contract itself** (or of the caller when used via delegatecall), not on msg.sender’s balance. There is no onlyDelegatecall modifier or other access control on injectAndCall, so any external caller can:

1. Choose an arbitrary ERC20 `token` and `target` address.
2. Cause the router to set an allowance of its **full token balance** for `token` in favor of `target` via SafeERC20.forceApprove(erc20, target, callerBalance).
3. Have the router immediately call `target` with arbitrary `callData`.

An attacker can deploy a malicious `target` contract whose function, when called by the router, executes `IERC20(token).transferFrom(address(this), attackerEOA, amount)` or similar logic, using the freshly granted allowance to pull **all tokens held by the router** for that token.

This is effectively a public "approve and arbitrary call with full router balance" primitive. If any tokens/ETH are ever held at the TrailsRouter address (e.g., from direct transfers, partial use of pullAndExecute/injectSweepAndCall, mistakenly sent funds, or mis-integration), they can be drained by any permissionless caller using injectAndCall.

In contrast, the delegatecall-only entry point handleSequenceDelegateCall routes balance injection through _injectAndCallDelegated (internal), which is protected by onlyDelegatecall on the outer handleSequenceDelegateCall. The mismatch is that injectAndCall itself is left publicly callable without the same guard, enabling unauthorized use of router-held balances.
 ### Static Signals
injectAndCall is external/public and lacks onlyDelegatecall or role checks, uses _getSelfBalance(token) (router’s full balance) rather than msg.sender’s balance, calls SafeERC20.forceApprove(erc20, target, callerBalance) for arbitrary target, target.call(callData) executed with router as msg.sender
 ### Assets at Risk
ERC20 tokens accidentally or residually held by TrailsRouter, ETH balances held by TrailsRouter from misrouted or leftover funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: TrailsRouter.__activateTstore

 ### Title
Tstorish __activateTstore can corrupt caller storage when router is delegatecalled
 ### Description/Code Snippet
TrailsRouter inherits `Tstorish`, which is designed to abstract over the presence or absence of the EVM `TSTORE/TLOAD` opcodes. `Tstorish` defines regular storage variables starting at slot 0:

```solidity
contract Tstorish {
    bool private _tstoreSupport;           // slot 0
    // ... immutables & function pointers ...

    function __activateTstore() external {
        if (msg.sender != tx.origin) {
            revert OnlyDirectCalls();
        }
        if (_tstoreInitialSupport || _tstoreSupport) {
            revert TStoreAlreadyActivated();
        }
        if (!_testTload(_tloadTestContract)) {
            revert TStoreNotSupported();
        }
        _tstoreSupport = true;             // writes to slot 0 via sstore
    }
}
```

TrailsRouter is explicitly intended to be used via `delegatecall` from a Sequence wallet or other host contract (see `DelegatecallGuard` and the `handleSequenceDelegateCall` entry point). Under `delegatecall`, any storage writes performed by the callee impact the **caller’s** storage layout, not the router’s.

`__activateTstore` is `external` and callable through TrailsRouter, and it only checks `msg.sender == tx.origin` (to ensure an EOA), but **does not prohibit delegatecall context**. If a host wallet (e.g., a Sequence v3 account) delegatecalls into TrailsRouter and then executes `__activateTstore` (for example via a misconfigured Merkle tree or an integration bug), the assignment `_tstoreSupport = true` will write to storage slot 0 of the *wallet contract*, potentially overwriting critical state there (owner address, nonce, configuration struct, etc.).

This is a classic storage-collision hazard:
- The extension (TrailsRouter via Tstorish) assumes it owns slot 0 for `_tstoreSupport`.
- When used via `delegatecall`, slot 0 actually belongs to the host contract.
- A single call to `__activateTstore` can permanently corrupt that slot.

Because `__activateTstore` is part of TrailsRouter’s inherited ABI and guarded only by `msg.sender == tx.origin` (which still holds when an EOA calls the host wallet that then delegatecalls Router), there is no on-chain protection preventing this from being invoked in delegatecall context, leaving host storage integrity dependent on off-chain discipline.
 ### Static Signals
inherits Tstorish which declares un-namespaced storage variable _tstoreSupport at slot 0, router is designed for delegatecall use (DelegatecallGuard, handleSequenceDelegateCall), external function __activateTstore writes to slot 0 via sstore, OnlyDirectCalls check uses msg.sender == tx.origin but does not forbid delegatecall
 ### Assets at Risk
host wallet contract storage (e.g., owner/signers/configuration), any contract state that uses slot 0 when using TrailsRouter via delegatecall
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: DoubleExecutionOrReplay

 ### Relevant Function/Location: TrailsRouter.validateOpHashAndSweep

 ### Title
OpHash success sentinel not consumed, enabling repeated validateOpHashAndSweep sweeps
 ### Description/Code Snippet
The TrailsRouter uses a per-opHash "success sentinel" to gate sweeping via `validateOpHashAndSweep`, but this sentinel is never consumed or cleared, so any context that can legitimately set it once can call `validateOpHashAndSweep` multiple times for the same `opHash` and token.

In `validateOpHashAndSweep` the router checks only that the sentinel is set to `SUCCESS_VALUE` and does not record that a sweep for that `opHash` has already been performed:

```solidity
function validateOpHashAndSweep(bytes32 opHash, address _token, address _recipient)
    public
    payable
    onlyDelegatecall
{
    uint256 slot = TrailsSentinelLib.successSlot(opHash);
    if (_getTstorish(slot) != TrailsSentinelLib.SUCCESS_VALUE) {
        revert SuccessSentinelNotSet();
    }
    sweep(_token, _recipient);
}
```

The sentinel slot is computed via:

```solidity
library TrailsSentinelLib {
    bytes32 public constant SENTINEL_NAMESPACE = keccak256("org.sequence.trails.router.sentinel");
    uint256 public constant SUCCESS_VALUE = uint256(1);

    function successSlot(bytes32 opHash) internal pure returns (uint256 result) {
        bytes32 namespace = SENTINEL_NAMESPACE;
        assembly {
            mstore(0x00, namespace)
            mstore(0x20, opHash)
            result := keccak256(0x00, 0x40)
        }
    }
}
```

and is read via Tstorish’s `_getTstorish` function pointer. Nowhere in `TrailsRouter` is the sentinel cleared or an `executed[opHash]` flag maintained. The shim (out of this snippet) is responsible for setting the sentinel, but not for clearing or consuming it either.

This means that once the shim (or any other Tstorish-using module in the same wallet context) has written `SUCCESS_VALUE` at `successSlot(opHash)`, any later call to:

```solidity
validateOpHashAndSweep(opHash, token, recipient)
```
will succeed as long as:
- it is executed via `delegatecall` (to satisfy `onlyDelegatecall`), and
- the wallet still holds a positive balance of `token`.

There is no on-chain enforcement that `validateOpHashAndSweep` is used at most once per `opHash`. If upstream systems (Sequence kernel, TrailsIntentEntrypoint, or off-chain relayers) ever re-submit the same Merkle leaf / opHash for execution (for example due to a race condition, replay, or misconfiguration), the router will happily re-sweep whatever balance is currently present in the wallet for that token to the configured `recipient`.

In the Trails design, this `recipient` is often a fee collector or the next-hop contract in a multi-leg flow. A repeated call could therefore:
- over-collect protocol fees from a wallet if the balance was topped up again later, or
- repeatedly sweep bridged funds that arrive in multiple transactions under the same `opHash` identifier, draining the wallet beyond what a single intent was supposed to authorize.

Because the contract does not store any `usedOpHash`/`executed[opHash]` mapping and never clears the sentinel (even via Tstorish `_clearTstorish`), idempotency is entirely delegated to off-chain / external coordination. If that coordination layer has any bug or replay window, the router itself provides no additional protection against double execution of the sweep step.

This matches the DoubleExecution/Replay pattern: there is a one-way "success" bit but no per-opHash consumption or replay protection for the actual sweeping side-effect.

 ### Static Signals
no executed[opHash] flag or mapping, success sentinel never cleared/consumed, validateOpHashAndSweep can be called multiple times with same opHash, handleSequenceDelegateCall can route validateOpHashAndSweep repeatedly for the same opHash, idempotency of sweep step delegated entirely to off-chain coordination
 ### Assets at Risk
user wallet token balances, user wallet native ETH balances, bridged funds held in Sequence wallets/intents, protocol fee balances collected via sweep flows
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole

