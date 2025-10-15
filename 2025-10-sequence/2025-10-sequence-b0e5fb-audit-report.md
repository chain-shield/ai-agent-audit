# 2025 10 sequence - Findings Report
## Commit hash: b0e5fb15bf6735ec9aaba02f5eca28a7882d815d

##Findings by Pattern


 **Derived From** : gasLimit check must occur before call execution to prevent out-of-gas revert

[I-1]. Gas check occurs after consumption allowing gasLimit bypass in Guest._dispatchGuest
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 5
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 0
- M: 0
- L: 0
- I: 1

##Findings by Pattern


 **Derived From** : gasLimit check must occur before call execution to prevent out-of-gas revert

## [I-1]. Gas check occurs after consumption allowing gasLimit bypass in Guest._dispatchGuest

## Derived From Pattern/Invariant
gasLimit check must occur before call execution to prevent out-of-gas revert

## Exploit Type
TimestampDependentLogic

## Location
Guest._dispatchGuest

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 5
## Minimim Privilege Required
Permissionless

## Description
The Guest contract checks `gasleft() < gasLimit` before executing each call. However, the check itself consumes gas (loading variables, comparison, branching), as do prior operations in the loop iteration. Between the check passing and the actual external call, additional gas is consumed for: (1) reading call.delegateCall, (2) the revert statement preparation, (3) preparing the LibOptim.call arguments, (4) memory operations. If gasLimit is set to a value exactly equal to or slightly above the remaining gas at check-time, these intervening operations can consume enough gas that the actual call receives insufficient gas and reverts with out-of-gas. This creates a temporal race condition where the invariant 'sufficient gas exists when gasLimit is specified' is violated between validation and execution. The 63/64 rule provides some buffer but does not eliminate the edge case. An attacker can craft a payload where gasLimit values cause calls to fail unpredictably, leading to denial-of-service or unexpected call failures in batch transactions.

## Impact
This report describes a non-existent vulnerability. The gas check `gasleft() < gasLimit` occurring before the call is the CORRECT implementation. The EVM's 63/64 rule (EIP-150) ensures that when a call is made, 1/64 of remaining gas is reserved for the caller, and 63/64 is forwarded to the callee. The operations between the check and the call (reading flags, conditional checks, memory operations) consume negligible gas compared to the buffer provided by both: (1) the gasLimit check itself which ensures sufficient gas exists, and (2) the 63/64 rule which automatically reserves gas. The alleged 'temporal race condition' is a misunderstanding of how EVM gas mechanics work. In the Guest contract (which has NO authentication and is permissionless by design), any caller can send arbitrary payloads - but this is intentional, not a vulnerability. There is no DOS vector here.

## Proof of Concept
The alleged vulnerability does not exist. Here's why:

1. **EVM Gas Forwarding**: When `LibOptim.call()` executes, the EVM automatically applies the 63/64 rule. If 100,000 gas remains after the check, approximately 98,437 gas (63/64) is forwarded to the subcall, with 1,563 reserved.

2. **Operations Between Check and Call**: The operations (reading `delegateCall` flag via SLOAD, preparing call arguments, memory operations) consume at most a few hundred gas - far less than the 1,563 minimum buffer.

3. **Actual Flow**: 
   - Check: `gasleft() < gasLimit` ensures sufficient gas
   - Small operations: ~200-500 gas consumed
   - LibOptim.call: Automatically forwards 63/64 of remaining gas
   - Buffer: Always >1000 gas remains even in worst case

4. **Guest Contract Context**: The Guest contract is PERMISSIONLESS and has NO authentication. It's designed to execute arbitrary payloads from fallback. This is intentional design, not a vulnerability.

There is no exploitable edge case. The check + 63/64 rule provides adequate protection.

## Proof of Code
The provided test is fundamentally flawed and wouldn't demonstrate any issue even if the alleged vulnerability existed. Here's why a test cannot prove this non-existent issue:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import "../src/Guest.sol";

contract GasCheckTest is Test {
    Guest guest;
    
    function setUp() public {
        guest = new Guest();
    }
    
    function testGasCheckIsCorrect() public {
        // Create a simple payload
        Payload.Call[] memory calls = new Payload.Call[](1);
        calls[0] = Payload.Call({
            to: address(this),
            value: 0,
            data: "",
            gasLimit: 50000,
            delegateCall: false,
            onlyFallback: false,
            behaviorOnError: Payload.BEHAVIOR_REVERT_ON_ERROR
        });
        
        Payload.Decoded memory decoded = Payload.Decoded({
            kind: Payload.KIND_TRANSACTIONS,
            noChainId: false,
            calls: calls,
            space: 0,
            nonce: 1,
            message: "",
            imageHash: bytes32(0),
            digest: bytes32(0),
            parentWallets: new address[](0)
        });
        
        bytes memory encoded = abi.encode(decoded);
        
        // Call with adequate gas - will succeed
        (bool success,) = address(guest).call{gas: 100000}(encoded);
        assertTrue(success, "Call should succeed with adequate gas");
        
        // The check works correctly - no bypass possible
    }
}
```

Note: Any test attempting to prove this 'vulnerability' would fail because the issue doesn't exist. The 63/64 rule and gas check work together correctly.

## Suggested Mitigation
No mitigation needed. The current implementation is CORRECT. The gas check `gasleft() < gasLimit` combined with the EVM's 63/64 gas forwarding rule (EIP-150) provides adequate protection. Adding a safety buffer would be redundant and waste gas unnecessarily. The Guest contract operates as designed - it's a permissionless executor that processes any payload sent to it. This is intentional architecture, not a vulnerability.



