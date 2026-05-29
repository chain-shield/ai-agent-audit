# Rejected Primary Findings: Nibiru

# Potential Chain Split Due to CometBFT State Sync Vulnerability (ASA-2024-009)

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-14
- **Submitter:** 0x41
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/14
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-14.md

## Brief Summary

The Nibiru project uses CometBFT 0.37.5 as specified in its `go.mod`, which is affected by a medium severity vulnerability ([ASA-2024-009](https://github.com/cometbft/cometbft/security/advisories/GHSA-g5xx-c4hv-9ccc)). The vulnerability allows a malicious state sync node to provide an invalid state of the proposer selection algorithm, which could lead to a chain split. When validators state sync from RPC endpoints with invalid proposer priority states, they will: 1. Disagree with other validators about block proposer selection 2. Reject valid blocks due to proposer mismatch 3. Have their own proposed blocks rejected by the network 4. Eventually contribute to network halt if enough validator...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Hardcoded gas limits may lead to failed execution when opcode costs change

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-62
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/62
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-62.md

## Brief Summary

Currently, the gas limits for several operations are hardcoded in the `erc20` file. This can create a situation where the future calls / deployments will fail due to the opcodes cost changes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Coin validation in fee handling would lead to broken accounting for fees

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-164
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/164
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-164.md

## Brief Summary

Borderline, considering the direct construction of `sdk.Coin` without proper validation could lead to not only invalid fee calculations and potential panics in production, but also the fact that the `Add` function would never return coins i.e when we have the amount to be non-negative for one of the coins, see documentation from `Add()` above.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_31_group

# Nibiru lacks support for TSTORE

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-167
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/167
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-167.md

## Brief Summary

Smart contracts relying on transient storage will fail, i.e DApps designed with transient storage optimizations won't work (incompatibility with Ethereum mainnet behavior). Which also showcases how the first window for attack ideas requested by Nibiru for the audit is not followed: Considering users that expect the same implementation of `tstore` even in Nibiru would not get this as they are used to it on Chains like Mainnet/Arbitrum.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# One could still provide gas for tx lower than base fee

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-168
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/168
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-168.md

## Brief Summary

Inconsistent fee market behavior, also seems to portray how network could be congested since we now provide lower gas fees than the base fee.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lower decimal tokens would be non-transferrable

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-193
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/193
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-193.md

## Brief Summary

> 2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements. MEDIUM. Considering the strict minimum transfer requirement of 10^12 wei causes: - Inability to transfer small amounts of low-decimal tokens... \_this ends up as not being able to integrate these set of tokens, contrary to what's been stated in the docs.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unsafe Integer Conversion in GetTransactionLogs

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-100
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/100
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-100.md

## Brief Summary

The res.MsgIndex field is cast to an integer without verifying bounds or type. Impact: High May cause incorrect indexing of logs or runtime panics.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Authorization for Critical Methods in function SendRawTransaction, GetTransactionByHash, and GetTransactionReceipt

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-102
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/102
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-102.md

## Brief Summary

Many methods such as SendRawTransaction, GetTransactionByHash, and GetTransactionReceipt do not appear to have any access control mechanisms Impact: Malicious actors could potentially send arbitrary transactions to the blockchain if no checks are in place to verify the source or authenticity of the requests. This can result in unauthorized transactions being broadcasted to the network. Risk: High. If there are no access controls in place for sensitive functions, this could lead to unauthorized users interacting with the blockchain.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Memory Exhaustion in Gas Estimation

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-106
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/106
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-106.md

## Brief Summary

- May allow DoS attacks via large transaction inputs. - Affects service availability. Risk High – Memory exhaustion vulnerabilities are a common vector for attacks.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Insufficient Validation of hexutil.Bytes Input

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-107
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/107
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-107.md

## Brief Summary

In SendRawTransaction, raw transaction bytes are decoded without additional validation. Impact: Denial of service through malformed payloads. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Insufficient Granularity in Multi-Execution Error Handling in executeMulti function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-108
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/108
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-108.md

## Brief Summary

The executeMulti method stops execution for all subsequent calls if any individual execution fails. Impact: A single failure could prevent legitimate operations, leading to poor user experience. Risk: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Access Control on instantiate

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-109
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/109
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-109.md

## Brief Summary

The instantiate method doesn't enforce strict access control, allowing any caller to instantiate contracts. Impact: Malicious actors could deploy unverified contracts, leading to trust issues. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation in queryRaw

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-111
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/111
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-111.md

## Brief Summary

The function queryRaw assumes args[argIdx] is always of type []byte. If an invalid argument type is passed, it will panic instead of returning an error. Impact: - Unexpected panics can disrupt EVM execution. - May be exploitable if raw inputs are not sanitized.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Validation of Contract Addresses in execute, executeMulti, instantiate, and query function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-112
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/112
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-112.md

## Brief Summary

Addresses passed in execute, executeMulti, instantiate, and query are not validated to ensure they are correctly formatted Bech32 addresses.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked Type Assertions in Multiple Functions like retrieveEVMTxFeesFromBlock and AllTxLogsFromEvents

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-113
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/113
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-113.md

## Brief Summary

Several functions, such as retrieveEVMTxFeesFromBlock and AllTxLogsFromEvents, use unchecked type assertions that may cause runtime panics. Impact: - Runtime panics due to invalid type assertions. - Inconsistent behavior and potential Denial-of-Service (DoS) vectors. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Out-of-Bounds Error in TxLogsFromEvents

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-114
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/114
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-114.md

## Brief Summary

The function decrements msgIndex without bounds checking, leading to a potential negative index. Impact: - The function might access unintended or undefined memory. - Could result in unexpected application behavior. Risk: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Gas Limit Validation in retrieveEVMTxFeesFromBlock

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-115
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/115
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-115.md

## Brief Summary

The function does not check for invalid or zero values for blockGasUsed, which can cause division-by-zero errors or invalid calculations. Impact: - Division-by-zero errors if gasLimitUint64 is invalid. - Incorrect gas usage ratio calculations. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# Potential Index Out of Range in retrieveEVMTxFeesFromBlock

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-116
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/116
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-116.md

## Brief Summary

The code uses txIndex to iterate through the sorter array, but there is no check to ensure that txIndex does not exceed the length of the array. Impact: - Index Out of Range: If the array sorter is empty or if txIndex exceeds ethTxCount-1, the code will panic due to an "index out of range" error. Risk: High - This can cause the service to crash, making it unreliable.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Input Validation for Trace Call

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-119
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/119
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-119.md

## Brief Summary

In the TraceCall function, there is no input validation to ensure that the parameters passed, especially the txArgs and contextBlock, are valid and not nil. User-provided inputs like hash, txArgs, or config are not properly validated for correctness. Malformed inputs could lead to panics or unexpected errors. Impact: - Malformed inputs can cause system instability. - Exposes backend to DoS attacks. - If txArgs is nil or improperly formatted, this could result in a runtime error, crashing the application. It may also lead to incorrect behavior when invoking trace functionality. Risk: High This missing validation can lead to unexpected crashes, incorrect results, or failures in tracing due to...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unnecessary Access to NetworkClient

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-120
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/120
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-120.md

## Brief Summary

In TraceTransaction, TraceCall and TraceBlock, the code accesses NetworkClient using the following pattern: nc, ok := b.clientCtx.Client.(tmrpcclient.NetworkClient) if !ok { return nil, errors.New("invalid rpc client") } This cast is performed multiple times and is unnecessary when the client context (b.clientCtx.Client) can be validated once at the start. Impact: Repeated casting of the client context might impact performance if the client is accessed multiple times within a block or transaction trace. Additionally, it introduces redundancy in the code. Risk: While this doesn't create a significant security risk, it contributes to code inefficiency and redundancy, which could make maintena...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Validation in parseArgsWasmInstantiate

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-123
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/123
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-123.md

## Brief Summary

- The admin string and msgArgs byte array are not validated for length, potentially leading to oversized or malformed payloads. - This oversight introduces potential vulnerabilities related to oversized or malformed input, which can have downstream effects. The parseArgsWasmInstantiate function processes arguments for contract instantiation but lacks constraints on: 1. admin: A string that should represent an optional administrator address. There is no check for: - Maximum permissible length. - Whether the string conforms to Bech32 encoding when provided. 2. msgArgs: A byte array that represents the contract initialization message payload. There is no restriction on: - The maximum allowable...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_40_group

# Missing Synchronization in Concurrent Environments

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-128
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/128
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-128.md

## Brief Summary

The journal struct's methods (append, Revert, etc.) modify shared state (e.g., entries, dirties). There is no synchronization mechanism to prevent data races in concurrent environments. Impact: - Data races leading to unpredictable behavior. - Corrupted state data in entries and dirties. Risk Level: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inconsistent Snapshot Handling in Revert Function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-130
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/130
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-130.md

## Brief Summary

The Revert function depends on a snapshot index without validating its range. Impact: Improper handling can lead to crashes or undefined behavior during state rollbacks. Risk: High — as state rollback is critical to blockchain execution integrity.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Gas Inefficiency and Redundant Operations in sortedDirties

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-131
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/131
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-131.md

## Brief Summary

The sortedDirties function creates a new slice and sorts it each time it is called, even if the dirties map has not changed. Sorting dirty addresses may lead to performance issues when the number of entries is large. Impact: - Increased gas costs in cases with frequent dirty address accesses. - Inefficiency when used in a loop or multiple times. - Performance degradation in scenarios with a high number of dirty addresses. Risk Level: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Integer Overflow/Underflow Handling in AddBalance and SubBalance

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-132
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/132
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-132.md

## Brief Summary

Functions AddBalance and SubBalance directly modify balances without validating that the operations do not result in overflow/underflow. - In the AddBalance and SubBalance methods, there is no explicit check for overflow or underflow when performing arithmetic operations on the account balance. - Although Go supports arbitrary-precision integers with *big.Int, overflow or underflow could occur with operations involving external inputs (such as user-provided amounts). AddBalance: s.SetBalance(new(big.Int).Add(s.Balance(), amount)) SubBalance: s.SetBalance(new(big.Int).Sub(s.Balance(), amount)) Even though *big.Int handles large values, operations that might result in invalid values (such as...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_20_group

# Improper Handling of Code Hash & Code Modifications in SetCode

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-133
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/133
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-133.md

## Brief Summary

The SetCode function assumes the provided codeHash is valid and directly updates the state, making it susceptible to invalid or tampered hashes. The method SetCode allows changing the contract code by updating the CodeHash and the associated code. However, there is no validation that checks if the new code is different from the previous one before applying changes. Impact: Invalid code hashes could lead to execution errors or contract code inconsistencies. 1. Inefficiency: Setting the code unnecessarily adds redundant operations to the journal, which may increase computational and storage costs. 2. Possible security risks: If the new code is not validated correctly, an attacker could exploi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Access Control for Sensitive Functions like Mint

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-135
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/135
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-135.md

## Brief Summary

The Mint function allows minting tokens, which is a highly sensitive operation. While the Solidity function restricts access using the onlyOwner modifier, there is no explicit check in the Go implementation to ensure only authorized users can invoke this function. The Mint method does not explicitly verify if the caller is the contract owner. While the documentation specifies onlyOwner, the implementation does not enforce this constraint programmatically. Impact - If the from address is not properly validated, unauthorized parties could mint tokens, inflating the total supply and potentially compromising the token's value and ecosystem. - An attacker could exploit this by calling the Mint f...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Insufficient Error Handling in Transfer

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-136
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/136
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-136.md

## Brief Summary

The Transfer function assumes that the recipient's balance always increases when the transfer succeeds. This assumption may not hold for fee-on-transfer tokens, which deduct fees from the transferred amount. The Transfer function only checks the success boolean but does not revert when success == false. Impact For tokens with transfer fees, this validation will falsely flag legitimate transfers as failed, disrupting the functionality for fee-on-transfer tokens. Users may perceive the system as unreliable due to ambiguous failure handling Risk High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Invalid Balance Parsing GetBalance and GetProof

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-140
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/140
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-140.md

## Brief Summary

Balance conversion from string in GetBalance and GetProof lacks proper validation and error handling. Impact Malformed balance strings can crash the node or cause undefined behavior. Risk High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inefficient Storage Proof Handling in GetProof

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-141
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/141
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-141.md

## Brief Summary

GetProof queries storage proofs in a loop, potentially causing performance bottlenecks for large storageKeys arrays. Impact Nodes can experience significant delays when processing multiple storage keys due to sequential requests. Risk Moderate

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Panic on Error in GetCode function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-142
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/142
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-142.md

## Brief Summary

The GetCode function calls panic(err) when an error occurs while fetching contract bytecode. Panicking in production code is generally not recommended, as it can lead to unhandled exceptions, breaking the flow of the system. Impact: The use of panic can cause the application to terminate unexpectedly, leading to a poor user experience and potential loss of functionality. System downtime or disruption in contract processing could occur. Risk: High: The system can crash, causing interruptions in services.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Inefficient Use of big.Int for Balance Comparison in SetAccBalance

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-143
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/143
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-143.md

## Brief Summary

The balance comparison in the SetAccBalance function uses big.Int for each coin calculation and comparison, which can be inefficient. The logic creates new big.Int objects multiple times in each case. Impact: Medium: The frequent creation of new big.Int objects can cause unnecessary memory allocation, impacting the performance of the contract especially in a high-frequency environment. Risk: Medium: It can cause higher gas costs and reduced performance due to inefficient memory handling.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential for nil Dereference in GetAccount

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-144
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/144
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-144.md

## Brief Summary

In the GetAccount method, if the GetAccountWithoutBalance returns nil (i.e., no account is found), the function continues to access the acct object, leading to a potential nil dereference. Impact: High: A nil dereference can cause a runtime panic, leading to a crash of the application. The system may not recover and could lead to data inconsistencies. Risk: High: A crash could interrupt processing and affect multiple contracts and accounts.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Unchecked Error in GetContractBytecode and GetState

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-145
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/145
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-145.md

## Brief Summary

In the GetContractBytecode and GetState methods, there's an implicit handling of default values for missing keys. However, when using the GetOr function, the code does not handle potential errors explicitly. While missing data is defaulted to an empty byte array (for GetContractBytecode) or an empty Hash (for GetState), there is no check or logging for the errors that might occur, which could lead to silent failures. Impact: Silent Failures: The code will silently return default values if an error occurs, making it difficult to diagnose issues. Risk: This could lead to faulty contract behavior or misleading results, especially if an actual data retrieval error occurs. Risk: Critical: Lack o...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Security Risk with SetAccState (Inconsistent State Deletion)

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-146
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/146
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-146.md

## Brief Summary

In the SetAccState function, if the provided stateValue is empty, the corresponding entry in AccState is deleted. This might inadvertently erase important state information that could be necessary, especially for non-empty state values in other contexts. Impact: Data Loss: Deleting state without any checks may lead to accidental removal of data that could be needed for contract execution, leading to potential data corruption. Risk: Moderate to High: Deletion logic should include safeguards, as unintended state deletions could cause the smart contract to behave unpredictably or lead to vulnerabilities in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_23_group

# No Validation of CreateFuntokenFee in SetParams

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-147
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/147
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-147.md

## Brief Summary

The SetParams function checks if CreateFuntokenFee is negative, but no validation occurs on other parameters of params. There might be other parameters that should be validated for correct ranges, types, or values. Impact: Incomplete Validation: Missing validation for other parameters could allow unintended values to be set, potentially causing vulnerabilities or improper contract execution. Risk: Moderate: Improper parameter values could introduce logic flaws or security issues, especially if the contract deals with funds or important configurations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# Incorrect Handling of Zero refundQuotient in GasToRefund

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-148
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/148
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-148.md

## Brief Summary

The GasToRefund function assumes refundQuotient is always non-zero. A zero value will cause a division by zero error, leading to a runtime panic. Impact: This issue could cause the application to crash if the refundQuotient value is accidentally set to zero, disrupting the state machine. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Validation for leftoverGas in RefundGas

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-149
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/149
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-149.md

## Brief Summary

The RefundGas function assumes leftoverGas is valid. While unlikely, a bug elsewhere in the system could pass an invalid or excessively large leftoverGas value. Impact: Could lead to an overflow when calculating leftoverWei, resulting in incorrect refunds or system instability. Risk: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Checks in DeductTxCostsFromUserBalance

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-150
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/150
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-150.md

## Brief Summary

The function does not verify if fees is non-zero. A zero fee could bypass deductions. Impact: This could allow transactions to proceed without proper fee deductions, leading to economic inconsistencies. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# PendingTransactions Missing Check for TxDecoder

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-153
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/153
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-153.md

## Brief Summary

The PendingTransactions method assumes TxDecoder is always non-nil, leading to a potential panic if it's not initialized. Impact: High. Critical functionality relies on TxDecoder. Risk: High. Can crash the application during runtime.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# FeeHistory Block Calculation Logic

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-154
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/154
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-154.md

## Brief Summary

Description: In the FeeHistory function, blockEnd+1 < blocks can result in an underflow if blockEnd is negative or small compared to blocks. Impact: High. Could cause incorrect fee history retrieval or unexpected behavior. Risk: High. Affects core functionality.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_58_group

# Missing Input Validation methodById and decomposeInput

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-156
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/156
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-156.md

## Brief Summary

The methodById and decomposeInput functions do not adequately validate the input data passed to them. For example, while there is a length check (len(sigdata) != 4), there is no safeguard against malformed or malicious inputs. Impact Can result in panics or undefined behavior. Might expose the EVM to denial-of-service (DoS) attacks by exploiting gas consumption for erroneous inputs. Risk High – Input validation is a critical component of contract security.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Gas Cost Calculation Vulnerability in requiredGas

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-157
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/157
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-157.md

## Brief Summary

In requiredGas, the isMutation map is accessed without checking for the presence of the method. While tests may ensure valid methods, introducing new methods without updating isMutation can lead to panics. Impact - Contracts could break if unexpected methods are introduced. - Potential for deployment downtime or unintended halts. Risk Medium – A well-maintained codebase reduces the risk, but unhandled edge cases can still cause issues.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# epetitive StateDB Synchronization Logic

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-174
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/174
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-174.md

## Brief Summary

The logic to synchronize the StateDB is repeated across multiple functions such as MintCoins, BurnCoins, SendCoins, etc. This duplication leads to a larger and harder-to-maintain codebase. The SyncStateDBWithAccount function silently fails if StateDB is nil. This could lead to inconsistent state if StateDB is not properly initialized. Impact: - Silent failure may lead to debugging difficulties. - Could cause desynchronization between Cosmos SDK state and Ethereum VM state.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Issue: Missing Input Validation for Coin Denomination

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-175
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/175
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-175.md

## Brief Summary

The contract does not validate the denomination (Denom) of the coins provided in the findEtherBalanceChangeFromCoins function. If Denom contains invalid or unexpected values (e.g., empty string or incorrect format), it could cause logical errors or unintended behavior. Impact: Risk of processing invalid or malicious coin denominations. Potential inconsistencies in state updates. Risk Level: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_28_group

# Missing Checks for Negative Coin Balances

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-176
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/176
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-176.md

## Brief Summary

The MintCoins, BurnCoins, and SendCoins functions assume that balances are non-negative. There are no checks to prevent negative balances due to logic errors. Impact: - Negative balances can lead to state inconsistencies. - Could be exploited to create invalid state transitions. Risk Level: Critical

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Replay Protection

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-178
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/178
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-178.md

## Brief Summary

There is no mechanism in the code to prevent replay attacks. A malicious user could reuse a signed transaction multiple times, especially if signatures are not properly handled. Impact: - Potential double-spending or unauthorized state modifications. - Risks to user funds and overall chain integrity. Risk Level: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# Metadata Overlap in ToBankMetadata

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-180
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/180
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-180.md

## Brief Summary

In the ToBankMetadata function, there's a potential overlap in the denomUnits and symbol fields. If a token has the same name as another, it might overwrite existing metadata or cause conflicts. Impact: Conflicts in DenomUnits can lead to users seeing incorrect balances or misidentification of assets. Risk: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked User Input for Gas Limits

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-181
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/181
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-181.md

## Brief Summary

The txData.GetGas() method is used directly to determine gas limits without robust validation to ensure it aligns with the block gas limit. Impact: Can lead to denial of service (DoS) by exceeding gas thresholds or resource exhaustion. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_24_group

# Function AddToBlockGasUsed: Integer Overflow

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-183
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/183
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-183.md

## Brief Summary

The function calculates blockGasUsed as k.EvmState.BlockGasUsed.GetOr(ctx, 0) + gasUsed. An integer overflow may occur if gasUsed is large enough, causing blockGasUsed to wrap around and become smaller than gasUsed. Impact: High — Incorrect gas accounting can destabilize the EVM module, potentially allowing malicious actors to bypass gas limits. Risk: Severe.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Function BaseFeeWeiPerGas: Invalid Context Instantiation

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-184
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/184
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-184.md

## Brief Summary

The function calls k.BaseFeeMicronibiPerGas(sdk.Context{}). Passing an empty context (sdk.Context{}) may lead to undefined behavior or incorrect calculations if BaseFeeMicronibiPerGas depends on contextual parameters. Impact: Medium — Incorrect fee calculations may lead to under- or over-charging users, reducing system trust. Risk: Moderate.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Panic Handling During Out of Gas

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-185
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/185
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-185.md

## Brief Summary

The HandleOutOfGasPanic mechanism used with defer assumes all panics are gas-related, which may not always hold true. Other critical issues causing panics could be masked. Impact: - Non-gas-related errors are incorrectly categorized, potentially hiding critical bugs or malicious attempts. - Leads to poor debugging experience and reduced reliability. Risk Level: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Gas Limit Mismanagement

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-186
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/186
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-186.md

## Brief Summary

Gas is consumed unconditionally in some scenarios, such as when EVM execution fails due to an error other than ErrOutOfGas. This could lead to over-consumption of gas. Impact: - Unnecessary depletion of gas for transactions that fail due to logical errors or external constraints. - Potential Denial of Service (DoS) if this behavior is exploited. Risk Level: High

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Nil Pointer Dereference on baseFeeWeiPerGas

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-188
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/188
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-188.md

## Brief Summary

The contract checks whether baseFeeWeiPerGas is nil, but the subsequent call to msgEthTx.EffectiveGasCapWei(baseFeeWeiPerGas) does not handle the scenario where it is nil. This creates the potential for a runtime panic. Impact High: This could cause a denial of service (DoS) by panicking the system when processing specific transactions. Risk Level High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Integer Overflows in TxConfig

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-189
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/189
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-189.md

## Brief Summary

The TxIndex and LogIndex are retrieved using uint conversion, which may result in undefined behavior if the source value exceeds the bounds of unit. Impact: Integer overflow could lead to incorrect configuration or crashing nodes. Risk: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Index Management in DeleteSlot

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-190
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/190
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-190.md

## Brief Summary

In the DeleteSlot function, the deletion of the slot may result in incorrect management of the slots array. The logic to handle "last slot deletion" (al.slots = al.slots[:idx]) truncates the array incorrectly. This can lead to removal of unrelated slots and address mismanagement. Impact: This issue can lead to the loss of critical access list data, which might cause unintended access or denial of access to slots. Risk: High Incorrect state management can result in unpredictable behavior, impacting transaction correctness.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Validation for idx in Contains

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-191
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/191
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-191.md

## Brief Summary

The Contains function does not validate the idx before accessing al.slots[idx]. If idx is out of bounds due to an invalid state, it will panic. Impact: A panic could halt execution unexpectedly, causing disruptions in smart contract processing. Risk: Medium Although this requires an invalid state, improper initialization or corrupted state can trigger it.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No Check for Nonce Exhaustion in deployERC20ForBankCoin

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-194
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/194
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-194.md

## Brief Summary

The deployERC20ForBankCoin function creates a deterministic address using the nonce of the EVM_MODULE_ADDRESS. If the nonce reaches its limit, the contract creation will fail. Impact: Failure to deploy new ERC20 contracts once the nonce is exhausted. Risk Level: High.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Reentrancy Protection in CallContractWithInput function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-195
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/195
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-195.md

## Brief Summary

If the CallContractWithInput function interacts with external contracts, there is a risk of reentrancy attacks. Impact: Unauthorized modifications or data leakage during contract calls. Risk Level: High.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# Absence of Rate-Limiting or Validation in RPCFilterCap

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-196
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/196
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-196.md

## Brief Summary

The RPCFilterCap function lacks mechanisms to enforce or validate the filter cap dynamically, which might lead to resource exhaustion. Impact: Resource exhaustion from excessive filter creation. Risk Level: High

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Handling for Duplicate Insertions

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-198
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/198
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-198.md

## Brief Summary

The SafeInsert function directly inserts the token without checking if a token with the same ID already exists in the state. This could overwrite existing tokens without notice. Impact: Overwriting tokens may cause data loss and inconsistencies, especially if the same erc20 address maps to multiple bankDenom values. Risk: Medium - While recoverable, this could lead to business logic errors or unintended overwrites.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# Infinite Loop Risk and Incorrect Iteration Terminationin ForEachStorage

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-74
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/74
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-74.md

## Brief Summary

The stopIter callback is expected to stop iteration when it returns false, but if it always returns true, the loop can iterate indefinitely. The iterator loop in ForEachStorage uses stopIter incorrectly. When stopIter returns false, the iteration stops prematurely, but it is intended to continue iterating. Impact: Medium. May lead to node resource exhaustion. Important storage keys may be skipped, leading to incomplete storage traversal. Risk: Dependent on usage patterns and callback implementations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Input in SetState

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-75
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/75
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-75.md

## Brief Summary

The SetState function allows updating contract storage by setting the value for a given key. However, there is no validation of the stateKey or stateValue inputs, leaving the system vulnerable to issues such as: Risk: 1. State Corruption: Invalid keys or values can introduce inconsistencies or unexpected behavior in the contract state. 2. Denial of Service (DoS): Excessively large or malformed inputs can strain node resources, leading to degraded performance or system crashes. 3. Unauthorized Access: Lack of validation may enable attackers to overwrite critical data unintentionally or maliciously. Impact: - Data integrity is compromised if invalid keys or values corrupt the storage. - Perfo...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inefficient Error Handling in Transaction Decoding in function EthMsgsFromTendermintBlock

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-91
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/91
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-91.md

## Brief Summary

The function EthMsgsFromTendermintBlock decodes transactions as follows: tx, err := b.clientCtx.TxConfig.TxDecoder()(tx) if err != nil { b.logger.Debug("failed to decode transaction in block", "height", block.Height, "error", err.Error()) continue } The error handling logs the error and proceeds to the next iteration of the loop, ignoring the failure silently. While logging errors is useful, it doesn't properly handle the error scenario and continues processing without addressing the issue. Impact 1. Silent failure: The system might continue processing invalid or incomplete data, leading to inaccurate or incomplete information being returned to the client. 2. Data integrity risk: Invalid tr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Pruning Configurations Leading to Missing Data in Functions like RPCBlockFromTendermintBlock, EthBlockFromTendermintBlock

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-93
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/93
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-93.md

## Brief Summary

The smart contract backend contains several references to handling pruned nodes, such as fetching data from pruned blocks and failing gracefully when the block data is unavailable due to node pruning. Specifically, the issue arises in methods like RPCBlockFromTendermintBlock, EthBlockFromTendermintBlock, and others when attempting to fetch historical data that has been pruned. Bug Locations 1. RPCBlockFromTendermintBlock: - Error handling for BaseFeeWei fails when querying pruned blocks. The log output explicitly states, "failed to fetch Base Fee from prunned block". 2. EthBlockFromTendermintBlock: - Similar error occurs while attempting to retrieve BaseFeeWei. The error is logged but does...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Input Validation Issues in sendToBank function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-94
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/94
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-94.md

## Brief Summary

The contract's sendToBank method does not validate inputs comprehensively. The argument parsing (parseArgsSendToBank) assumes that inputs will always be of the expected type and format. If malformed input is passed, the contract will crash or behave unpredictably. - The amount parameter in sendToBank is validated only as amount.Cmp(big.NewInt(0)) != 1, which excludes nil but doesn't ensure sufficient precision (e.g., very small amounts). - The to parameter validation relies solely on sdk.AccAddressFromBech32, which doesn’t verify if the address is active or capable of receiving tokens. Impact: - Untrusted or invalid inputs could lead to unexpected behavior, contract failures, or vulnerabili...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Inefficient Transaction Search in GetTransactionByHash and getTransactionByHashPending methods

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-98
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/98
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-98.md

## Brief Summary

In both methods, the logic includes redundant or inefficient checks such as iterating over the entire list of transactions in a block or mempool in order to find a specific transaction. This can be particularly problematic when dealing with large blocks or high transaction volumes. Impact: Increased computational overhead and slower response times, which could degrade user experience and application performance. Potential scalability issues as the blockchain grows.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation for GetTransactionByHash

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-99
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/99
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-99.md

## Brief Summary

The method directly processes hash without ensuring that it is valid or follows the expected format. Impact: Medium. This can lead to application crashes or potential Denial-of-Service (DoS) if exploited repeatedly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_33_group

# Overflow risk due to improper block number casting in `GetTransactionCount()`

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-26
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/26
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-26.md

## Brief Summary

In the `GetTransactionCount()` function, a potential `integer overflow` issue exists due to the absence of a boundary check when converting a block number (`bn`) `int64`. While the `GetProof()` function implements a safeguard by checking if bn exceeds `math.MaxInt64` before casting, `GetTransactionCount()` directly casts bn to `int64` without this validation. This inconsistency could lead to `overflow`, resulting in erroneous or negative block heights, which may cause logic errors or unintended behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# GetEthIntrinsicGas Function Should Read Homestead and Istanbul Flags from Config Instead of Using Fixed true Values

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-41
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/41
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-41.md

## Brief Summary

The GetEthIntrinsicGas function in the Cosmos SDK-based EVM module (likely in x/evm/keeper/gas.go) is responsible for calculating the intrinsic gas cost for Ethereum transactions. The issue arises from the fact that the function currently uses fixed true values for the homestead and istanbul parameters when calling core.IntrinsicGas. These values should instead be derived from the configuration (cfg) based on the current block height, as Ethereum's network conditions change with different network upgrades (such as Homestead and Istanbul).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Negative Value Transfer Vulnerability

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-43
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/43
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-43.md

## Brief Summary

The current implementation only checks for positive value transfers in the CanTransferDecorator, potentially allowing negative value transfers to bypass balance checks. - Theoretical possibility of negative value transfers - Potential balance manipulation - Bypass of transfer validation checks

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# wrong implement of AddPrecompiles

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-53
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/53
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-53.md

## Brief Summary

There is a wrong implementation of AddPrecompiles as the is an extra comma.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Validation for Exchange Rate Bounds in `precompileOracle` Smart Contract

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-89
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/89
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-89.md

## Brief Summary

**Missing Validation for Exchange Rate Bounds in `precompileOracle` Smart Contract** --- Summary The lack of validation for minimum and maximum exchange rate bounds in the `queryExchangeRate` method will cause a mispricing vulnerability for the protocol. This allows stale or manipulated exchange rates to propagate through the system, potentially causing incorrect calculations or economic losses for protocol users and stakeholders. --- Root Cause In the `precompileOracle` smart contract: - In `queryExchangeRate` , the contract fetches the exchange rate using `p.oracleKeeper.GetDatedExchangeRate`. - The returned `price` is directly packed and returned without checking whether it falls within...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Weak permissions and potential for permission escalation in "ModuleAccPerms"

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-5
- **Submitter:** felon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/5
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-5.md

## Brief Summary

The ModuleAccPerms function returns permissions that allow minting and burning tokens without fine-grained control. Allowing multiple modules to mint and burn tokens presents a high-risk for token manipulation or supply inflation, especially if exploited by modules with loose or insufficiently controlled access.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Error wrapping with errors.Wrapf

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-6
- **Submitter:** felon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/6
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-6.md

## Brief Summary

The use of "errors.Wrapf" provides contextual error information, but improper handling of wrapped errors could expose internal details, potentially leading to information leakage.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Validity Check for Parameters in the balance Function in x/evm/precompile/funtoken.go

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-28
- **Submitter:** gxh191
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/28
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-28.md

## Brief Summary

The `balance` function in Go is used to implement the `IFunToken.balance` contract method. Its purpose is to return the ERC20 balance of a user address, the bank balance, FunToken-related information, and the Nibiru base account address. solidity // function balance( // address who, // address funtoken // ) // external // returns ( // uint256 erc20Balance, // uint256 bankBalance, // FunToken memory token, // NibiruAccount memory whoAddrs // ); // In this code, the `balance` function calls `parseArgsBalance` to parse the arguments and obtain `addrEth`, `addrBech32`, and `funtoken`: However, after this, there is no validation of these parameters for their correctness and safety before they ar...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# QA Report

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-7
- **Submitter:** klmldl
- **Claimed severity:** QA
- **Final severity:** QA
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/7
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-7.md

## Brief Summary

See the markdown file with the details of this report [here](https://github.com/code-423n4/2024-11-nibiru-validation/blob/main/data/klmldl-Q.md).

## Rejection Reason

GitHub validation labels: bug, insufficient quality report, QA (Quality Assurance), :robot:_primary
