# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : Unhandled behaviorOnError==3 makes failed low-level calls appear successful

[L-1]. Unhandled behaviorOnError==3 in Calls._execute treats failed sub‑calls as success and can mislead tooling
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: RequiresRole


### Number of Findings
- C: 0
- H: 0
- M: 0
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Unhandled behaviorOnError==3 makes failed low-level calls appear successful

## [L-1]. Unhandled behaviorOnError==3 in Calls._execute treats failed sub‑calls as success and can mislead tooling

### Finding Severity Justification: The bug causes failed sub-calls to be reported as succeeded and suppresses fallback-only flows when behaviorOnError decodes to 3. It does not enable unauthorized execution or loss of funds, but it misrepresents execution outcomes via events and can break intended batch control flow. This aligns with QA/Low under Code4rena: functional correctness/event issues without direct asset risk.
## Derived From Pattern/Invariant
Unhandled behaviorOnError==3 makes failed low-level calls appear successful

## Exploit Type
UncheckedReturn

## Location
Calls._execute

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `Calls` executor decodes each batched call’s `behaviorOnError` from the top two bits of the per‑call `flags` byte. Only three values (0,1,2) are defined in `Payload` as `BEHAVIOR_IGNORE_ERROR`, `BEHAVIOR_REVERT_ON_ERROR`, and `BEHAVIOR_ABORT_ON_ERROR`. However, the raw decode produces values 0–3, and the `Calls._execute` logic does not handle the case `behaviorOnError == 3`. When a low‑level call fails and `behaviorOnError == 3`, the function falls through the error‑handling branch and emits `CallSucceeded` even though `success == false`.

Decoding in `Payload.fromPackedCalls`:

```solidity
// Last 2 bits are directly mapped to the behavior on error
_decoded.calls[i].behaviorOnError = (flags & 0xC0) >> 6; // values 0..3
```

Execution in `Calls._execute`:

```solidity
function _execute(uint256 _startingGas, bytes32 _opHash, Payload.Decoded memory _decoded) private {
  bool errorFlag = false;
  uint256 numCalls = _decoded.calls.length;
  for (uint256 i = 0; i < numCalls; i++) {
    Payload.Call memory call = _decoded.calls[i];
    ...
    bool success;
    if (call.delegateCall) {
      (success) = LibOptim.delegatecall(...);
    } else {
      (success) = LibOptim.call(...);
    }

    if (!success) {
      if (call.behaviorOnError == Payload.BEHAVIOR_IGNORE_ERROR) {
        errorFlag = true;
        emit CallFailed(_opHash, i, LibOptim.returnData());
        continue;
      }

      if (call.behaviorOnError == Payload.BEHAVIOR_REVERT_ON_ERROR) {
        revert Reverted(_decoded, i, LibOptim.returnData());
      }

      if (call.behaviorOnError == Payload.BEHAVIOR_ABORT_ON_ERROR) {
        emit CallAborted(_opHash, i, LibOptim.returnData());
        break;
      }
    }

    emit CallSucceeded(_opHash, i);
  }
}
```

For `behaviorOnError == 3`, none of the three `if` branches match, so the code reaches `emit CallSucceeded(_opHash, i);` even though `success` is false. The internal `errorFlag` also remains `false`, so the following effects occur:

- The sub‑call’s failure is not reflected in events (`CallSucceeded` is emitted instead of `CallFailed`/`CallAborted`).
- `errorFlag` is not set, so any subsequent `onlyFallback` calls (meant to execute only if the previous call failed) are skipped.
- Off‑chain monitoring and tooling that rely on the event log and `behaviorOnError` semantics may reach incorrect conclusions about what actually executed.

`Estimator._estimate` duplicates the same logic for gas estimation, and `Simulator.simulate` uses a separate but similar interpreter. In `Estimator._estimate`, the same missing branch means the estimation path will also treat failed calls with `behaviorOnError == 3` as successes.

## Impact
This bug does not let external attackers bypass authentication or steal funds, but it corrupts the observable semantics of batched execution for any wallet owner who (directly or via custom tooling) uses `flags` values where `(flags & 0xC0) >> 6 == 3`. Failing sub‑calls will be reported as succeeded, and fallback‑only compensating calls will not run. This can confuse monitoring, auditing, or automated remediation flows built around `CallSucceeded`/`CallFailed`/`CallAborted` events, potentially causing mis‑accounting or failure to apply expected safeguards when calls fail.

## Command to Run Test


## Proof of Concept
Revised PoC (concise):
1) Deploy wallet (Stage2Module) and a target contract whose fallback always reverts.
2) Construct a packed calls payload with two calls:
   - Call[0]: to target, no value, no data, gasLimit set, behaviorOnError=3 (bits 7..6=11), so flags=0xC8.
   - Call[1]: any address, onlyFallback=true (bit5=1), no gas/data/value.
   Use a globalFlag with space=0 (bit0=1), nonceSize=1 (bit1=1), multi-call, 1-byte call count.
3) Invoke wallet.selfExecute(payload) by pranking as the wallet (onlySelf gate).
4) The first low-level call fails. Since behaviorOnError==3 is unhandled, the code falls through and emits CallSucceeded for a failed call, without setting errorFlag.
5) The second call is marked onlyFallback but is skipped (CallSkipped) because errorFlag remained false.
6) Off-chain/event-based tooling is misled: it sees CallSucceeded for a failed call, and compensating onlyFallback flows don’t run.

## Proof of Code
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import {Stage2Module} from "src/Stage2Module.sol";

contract AlwaysRevert {
    fallback() external payable { revert("fail"); }
}

contract BehaviorOnErrorThreeTest is Test {
    Stage2Module wallet;
    AlwaysRevert target;

    // Mirror events from Calls
    event CallSucceeded(bytes32 _opHash, uint256 _index);
    event CallSkipped(bytes32 _opHash, uint256 _index);

    function setUp() public {
        wallet = new Stage2Module(address(0)); // 4337 disabled
        target = new AlwaysRevert();
    }

    function test_BehaviorOnError3_FailedCallReportedAsSuccess_and_FallbackSkipped() public {
        // Build packed payload with 2 calls:
        //  - Call[0]: behaviorOnError == 3, gasLimit present, external target that always reverts
        //  - Call[1]: onlyFallback == true, will be skipped since errorFlag was not set
        bytes memory payload = buildPayload(address(target), 100000, 3, address(0xBEEF));

        // Expect a misleading success event for the failed first call
        vm.expectEmit(false, false, false, false, address(wallet));
        emit CallSucceeded(bytes32(0), 0);

        // Expect the fallback-only second call to be skipped
        vm.expectEmit(false, false, false, false, address(wallet));
        emit CallSkipped(bytes32(0), 1);

        // onlySelf: prank as the wallet to call selfExecute
        vm.prank(address(wallet));
        wallet.selfExecute(payload);
    }

    function buildPayload(address to1, uint256 gas1, uint8 behavior1, address to2) internal pure returns (bytes memory) {
        // Global flag: space=0 (bit0=1), nonceSize=1 (bit1=1), multi-call, 1-byte call count
        bytes memory b = abi.encodePacked(bytes1(0x03));
        // Nonce (1 byte)
        b = abi.encodePacked(b, bytes1(0x00));
        // Call count (1 byte) = 2
        b = abi.encodePacked(b, bytes1(0x02));

        // Call[0] flags: gasLimit present (bit3=1), behaviorOnError=3 => bits6..7=11 (0xC0), result 0xC8
        bytes1 flags0 = bytes1(uint8(0xC8));
        b = abi.encodePacked(b, flags0, to1, bytes32(gas1));

        // Call[1] flags: onlyFallback (bit5=1), external (bit0=0), no gas/value/data, behavior default 0
        bytes1 flags1 = bytes1(uint8(0x20));
        b = abi.encodePacked(b, flags1, to2);

        return b;
    }
}


## Suggested Mitigation
Fully validate behaviorOnError and disallow the reserved value:
- Enforce at execution time in Calls, Estimator, and Simulator:
  if (!success) {
    if (call.behaviorOnError == Payload.BEHAVIOR_IGNORE_ERROR) { ... }
    else if (call.behaviorOnError == Payload.BEHAVIOR_REVERT_ON_ERROR) { ... }
    else if (call.behaviorOnError == Payload.BEHAVIOR_ABORT_ON_ERROR) { ... }
    else { revert InvalidBehaviorOnError(call.behaviorOnError); }
  }
- Additionally, validate during decoding in Payload.fromPackedCalls: if ((flags >> 6) == 3) revert InvalidBehaviorOnError(3). This prevents malformed flags from reaching executors and improves determinism across Estimator/Simulator.
- Optionally, change behaviorOnError to a uint8/enum and clamp to known values at boundaries.
- Update Estimator._estimate and Simulator.simulate with the same guard to keep tooling behavior consistent.



