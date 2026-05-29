# Rejected Primary Findings: Kakarot

# `execute_from_outside()` does not check if an account has been initialized, allowing an attacker to impersonate transactions from the Ethereum zero address on Kakarot.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-271
- **Submitter:** 0xTonraq
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/271
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-271.md

## Brief Summary

The `execute_from_outside()` does not check if an account instance has been initialised, hence the `Account_evm_address` storage_var defaults to `0`, which when read by `execute_from_outside()` would default to `0x0...`, which is the Ethereum `zero/burn` address, therefore an attacker can successfully submit transactions made by the Ethereum `zero/burn` address. Vulnerability details Kakarot privides a mechanism for validating and executing Ethereum transactions through the `execute_from_outside()` functions present in both `src/kakarot/accounts/account_contract.cairo` and `src/kakarot/accounts/library.cairo` (core logic), These functions contains a series of validation checks and verificat...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential overflow in the function is_account_alive()

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-194
- **Submitter:** 0xtomsmith
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/194
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-194.md

## Brief Summary

The function `is_account_alive()` may incorrectly return `FALSE` if an overflow happens on line 427 inside state.cairo: If the calculation on line 427 `nonce + code_len + balance.low + balance.high` overflows and results in 0, then the function incorrectly returns `FALSE` due to the overflow. As a result the protocol may proceed in doing wrong calculations. For example system_operations.cairo is using `State.is_account_alive()` to determine gas costs. Also inside system_operations.cairo the function `exec_selfdestruct()` may calculate incorrect gas costs.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `selfdestruct` function state modification is incorrect

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-122
- **Submitter:** 13u9
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/122
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-122.md

## Brief Summary

The `kakarot evm` is implemented according to the EIP instruction specifications. However, the `selfdestruct` instruction of `kakarot evm` does not adhere to the specification.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Redeployment to the same address is impossible using `CREATE2` in the `exec_create` instruction.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-123
- **Submitter:** 13u9
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/123
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-123.md

## Brief Summary

In `kakarot evm`, the `exec_create` instruction generates the `CREATE` and `CREATE2` operations. The standard EIP `CREATE2` instruction is designed to allow redeployment to a deleted address (an address that has undergone `selfdestruct`). However, in `kakarot evm`, redeployment using `CREATE2` is not possible in the `exec_create` instruction.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# In the case of a non-deploy transaction, the nonce of the `env.origin` EVM account does not increase.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-134
- **Submitter:** 13u9
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/134
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-134.md

## Brief Summary

In the Kakarot EVM, there are two types of transactions: 1. Transactions that deploy an EVM address. 2. All other transactions. Among these, in case of sending a transaction of type 2, if the transaction is a simple one that sends `value` to a `to` address, rather than containing Kakarot EVM instruction bytecode that increments the `nonce`, the `nonce` of the `evm.address account` of the `sender` does not increase.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Underflow in SSTORE gas refund calculation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-159
- **Submitter:** 1AutumnLeaf777
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/159
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-159.md

## Brief Summary

Description: When calculating the refunds for `SSTORE`, there is an underflow in the following code: The computation of `gas_refund` can underflow in the following scenario: current_value = 0, new_value != 0, original_value !=0, new_value = original value. This will cause the `gas_refund` value to be equal to `-5000`, which means if the current `evm.gas_refund` is lower than 5000, the addition will underflow. Then in `Interpreter.execute` the gas refund is computed: https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/interpreter.cairo#L1007-L1015, However the following `is_nn` check is bypassed because the gas refund is definitely larger than the m...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Authorized pre eip155 txs will revert

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-162
- **Submitter:** 1AutumnLeaf777
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/162
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-162.md

## Brief Summary

When calling a pre eip155 tx, there is a check to ensure that the msg is whitelisted: https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/accounts/library.cairo#L209-L221 It will then proceed to call the `eth_send_raw_unsigned_tx` function: https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/eth_rpc.cairo#L248-L254 which checks that the chain id is equal to the kakarot chain id. However pre eip155 txs are considered legacy txs: https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/utils/eth_transaction.cairo#L63-L93 and as a result the chain id will be set to 0 during rlp d...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Origin and extra precompiles are not warmed up when preparing a transaction

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-185
- **Submitter:** 1AutumnLeaf777
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/185
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-185.md

## Brief Summary

The origin along with kakarot and rollup precompiles are not warmed up: https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/interpreter.cairo#L914-L920 This causes users to spend more gas in the case that they either invoke a precompile outside of the ethereum specified precompile list or if they call `tx.origin` in a transaction. This opcode mispricing also deviates from the EVM specification.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Reverted txs in root ctx do not give refunds

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-220
- **Submitter:** 1AutumnLeaf777
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/220
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-220.md

## Brief Summary

If a create tx fails in the root context (for reasons like nonce overflow or more practically the balance being sent is more than allowed) then geth refunds all gas: https://github.com/ethereum/go-ethereum/blob/65e5ca7d8126f7a8c708f8affb64f16c22cc63c0/core/vm/evm.go#L442-L444 (Notice the entire gas is set to `st.GasRemaining`: https://github.com/ethereum/go-ethereum/blob/65e5ca7d8126f7a8c708f8affb64f16c22cc63c0/core/state_transition.go#L454-L461 which is then refunded in `refundGas`: https://github.com/ethereum/go-ethereum/blob/65e5ca7d8126f7a8c708f8affb64f16c22cc63c0/core/state_transition.go#L491-L518) However Kakarot will consider this an `EXCEPTIONAL_HALT` and thus set the gas_left and g...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# max_fee_per_gas is not checked to have a reasonable value potentially introducing overflow

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-142
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/142
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-142.md

## Brief Summary

max_fee_per_gas is not checked to have a reasonable value potentially introducing overflow

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# The check for the validity of the init code is incorrect

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-143
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/143
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-143.md

## Brief Summary

The check for the validity of the init code is incorrect

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Gas may not fit into felt value when computing message call gas value

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-144
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/144
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-144.md

## Brief Summary

Gas may not fit into felt value when computing message call gas value

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# There are no static gas costs for TLOAD / TSTORE opcodes

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-212
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/212
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-212.md

## Brief Summary

There are no static gas costs for TLOAD / TSTORE opcodes

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Transient storage is not reset at the beginning of transaction execution

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-221
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/221
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-221.md

## Brief Summary

Transient storage is not reset at the beginning of transaction execution

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Access lists are not processed correctly according to the EIP-2929

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-223
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/223
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-223.md

## Brief Summary

Access lists are not processed correctly according to the EIP-2929

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Karakot VM does not follow EIP-3651 making the coinbase payments more expensive for users

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-227
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/227
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-227.md

## Brief Summary

Karakot VM does not follow EIP-3651 making the coinbase payments more expensive for users

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Incompatibility with Smart Contract Account Signatures in Kakarot

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-279
- **Submitter:** AshutoshSB
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/279
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-279.md

## Brief Summary

The Kakarot protocol currently verifies user signatures in a way that supports only basic, single-key accounts. This approach assumes each user account is an Externally Owned Account (EOA), where the user is the only key holder. The issue is that ecrecover can’t verify signatures from smart contract accounts, meaning: Smart Contract Accounts Won't Verify: Without support for EIP-1271, smart contract accounts won’t pass signature checks. EIP-4337 and Account Abstraction are Incompatible: If users have smart contract accounts that follow EIP-4337, their signatures can’t be verified by Kakarot. Impact : This limitation restricts the types of accounts that can interact with Kakarot: Incompatibi...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Memory Overwrite in ` felt_to_ascii` due to Uninitialized Memory Allocation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-246
- **Submitter:** Auditor2947
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/246
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-246.md

## Brief Summary

The function `felt_to_ascii` casts `asci`i directly from `[fp]` without initializing memory. Without proper allocation, `ascii` may overwrite other memory segments used by other functions, causing unexpected behavior, data corruption, or leaks. Repeated calls to `felt_to_ascii` increase the risk of memory overlap, which could lead to data loss or even crashes. Impact: This bug could lead to memory leaks, crashes, or unexpected program behavior. It also introduces a security risk, as an attacker might manipulate memory content, compromising both function integrity and application stability.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Uninitialized Pointer Usage in `dict_values` Function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-249
- **Submitter:** Auditor2947
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/249
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-249.md

## Brief Summary

In the `dict_values` function, the `pointer` is cast to `Uint256*` without validating that `dict.new_value `is correctly initialized or points to valid memory. Accessing `pointer[0]` without this verification may result in reading uninitialized memory, leading to unpredictable behavior and potential data integrity issues. Impact Accessing uninitialized memory can produce random or unexpected values, compromising data integrity and causing inconsistent dictionary outputs, which may affect downstream functions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Assertion on `range_check_ptr` in `decode_type_unsafe` Function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-250
- **Submitter:** Auditor2947
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/250
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-250.md

## Brief Summary

The assertion in the `decode_type_unsafe` function checks whether `range_check_ptr` equals `string_len`. However, it does not validate the overall consistency of the data. This can lead to situations where `string_len` is inconsistent with `range_check_ptr`, allowing for erroneous data to be accepted without proper validation. Impact Inconsistencies between `range_check_ptr` and `string_len` can cause `false-positive` assertion passes. This flaw could enable invalid data to bypass crucial validation checks, potentially resulting in unpredictable behavior in downstream processes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Carry Handling in `Addition ` and ` Subtraction ` Functions

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-253
- **Submitter:** Auditor2947
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/253
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-253.md

## Brief Summary

The `uint256_add` and `uint256_sub` functions exhibit incorrect handling of the carry during addition and subtraction operations on the `Uint256` type. When the result of an addition or subtraction exceeds the limits of a 256-bit integer, the carry should be properly propagated from the low part to the high part of the `Uint256` structure. Failure to do so may result in incorrect arithmetic outcomes. Impact: Improper carry handling can lead to erroneous results during arithmetic operations, which may introduce vulnerabilities such as `overflows` or `underflows`. These inaccuracies can affect subsequent calculations and potentially expose contracts to exploitations, especially in scenarios r...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Vulnerability in Out of Gas Error Handling for Memory Operations

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-256
- **Submitter:** Auditor2947
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/256
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-256.md

## Brief Summary

The current implementation does not adequately handle scenarios where the available gas is insufficient to cover the `memory_expansion.cost`. This oversight may result in unexpected `out-of-gas` errors, which can be exploited by malicious actors through the manipulation of memory operations, potentially exhausting the contract's gas. Impact: This vulnerability poses a significant risk as it may lead to `denial-of-service` `(DoS)` attacks. An attacker can intentionally trigger `out-of-gas` conditions, effectively locking out legitimate users and disrupting essential operations of the smart contract. This disruption not only affects user experience but also exposes the contract to further sec...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Duplicate `Event` Definitions

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-257
- **Submitter:** Auditor2947
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/257
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-257.md

## Brief Summary

The contract defines multiple `Transfer` events with the `same name` but differing parameter types. This redundancy can lead to ambiguity regarding which event is emitted, complicating event `logging` and monitoring. Impact: Inaccurate event `logging` may occur, hindering external systems (like `wallets` and `user interfaces`) from accurately tracking token transfers. This confusion can affect user experience and trust in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `saturated_sub` would return wrong data in some instances

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-156
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/156
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-156.md

## Brief Summary

Any ext/internal query to this, i.e `saturated_sub` would have the protocol easily wrongly assume that the answer is wrongly non-negative whereas it is rightly non-negative. _Considering the sort of contest Kakarot is, I assume this should be flagged, as one can consider this functionality to be similar to the native underflow protected subtraction on solidity EVM._

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# SHA-256 precompile's price would be inefficient post EIP 7667

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-158
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/158
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-158.md

## Brief Summary

Completely broken logic in regards to the amount of gas used, i.e: - For small inputs (<32 bytes): Gas cost will increase from 72 to 300. - For medium-sized inputs (32-64 bytes): Gas cost will increase from 84 to 360. - For larger inputs (>64 bytes): Gas cost will increase proportionally. Which then has Kakarot undercharging for these operations and is misaligning with the actual resource consumption on the network, which could affect the economics of applications relying on this precompile. Alternatively this could cause for abuse if users exploit the difference between the reported and actual gas costs.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# ECADD/ECMUL's implementation deviates from Ethereum's VM

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-189
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/189
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-189.md

## Brief Summary

ECADD/ECMUL deviates from EIP 196 and is not equivalent to the Ethereum VM. Since in the case the tx to call the precompile fails the gas cost is [assumed to be zero](https://github.com/kkrt-labs/kakarot-ssj/blob/d4a7873d6f071813165ca7c7adb2f029287d14ca/crates/contracts/src/cairo1_helpers.cairo#L142-L149) whereas it should still be paid in full.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_90_group

# `decode_1559()` does not work for transactions with fee market

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-190
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/190
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-190.md

## Brief Summary

Ethereum transactions with fee markets can never be decoded.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_56_group

# ecrecover precompile can never be called

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-196
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/196
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-196.md

## Brief Summary

Some precompiles can't be accessed, in this case ecrecover, which bricks any needed signature logic in the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Valid transactions like those to create contracts would always fail

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-197
- **Submitter:** Bauchibred
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/197
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-197.md

## Brief Summary

The [check ](https://github.com/kkrt-labs/kakarot/blob/6496ee1d18380dae5b069c27b3129d82ae282419/cairo_zero/utils/utils.cairo#L240) for `"Bytes has length {bytes_len}, expected 0 or 20"` is wrong and instead only enforces that the tx address length is `20` which causes for valid txs to no addresses or addresses of `0` length to always fail.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Reentrancy leading to drainage of user funds or operation disruption

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-203
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/203
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-203.md

## Brief Summary

Reentrancy leading to drainage of user funds or operation disruption

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Unrestricted delegatecall that can lead to complete loss of control over the calling contract's state, leading to significant financial and functional damage

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-205
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/205
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-205.md

## Brief Summary

Unrestricted delegatecall that can lead to complete loss of control over the calling contract's state, leading to significant financial and functional damage

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Missing Fallback for staticcallCairo which can cause the transaction to fail unexpectedly without clear error handling.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-206
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/206
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-206.md

## Brief Summary

Missing Fallback for staticcallCairo which can cause the transaction to fail unexpectedly without clear error handling.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Inputs and Malformed Data can lead to Crashes, DoS, or incorrect execution

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-208
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/208
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-208.md

## Brief Summary

Unchecked Inputs and Malformed Data can lead to Crashes, DoS, or incorrect execution

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Replay Attacks Across Networks can lead to potential fund loss

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-247
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/247
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-247.md

## Brief Summary

Replay Attacks Across Networks can lead to potential fund loss

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Reentrancy Attacks causing state locking, check-effects-interactions pattern

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-251
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/251
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-251.md

## Brief Summary

Reentrancy Attacks causing state locking, check-effects-interactions pattern

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_146_group

# Integer Overflow/Underflow can lead to loss of funds, resource exhaustion, incorrect transaction processing

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-255
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/255
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-255.md

## Brief Summary

Integer Overflow/Underflow can lead to loss of funds, resource exhaustion, incorrect transaction processing

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_43_group

# Division by Zero could result in a runtime failure or revert the transaction, leading to potential denial-of-service scenarios.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-263
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/263
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-263.md

## Brief Summary

Division by Zero could result in a runtime failure or revert the transaction, leading to potential denial-of-service scenarios.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_54_group

# Overflow in Cairo's PRIME Field could result in incorrect outputs that violate the function’s assumptions.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-276
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/276
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-276.md

## Brief Summary

Overflow in Cairo's PRIME Field could result in incorrect outputs that violate the function’s assumptions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_79_group

# Lack of Access Control in write_storage Function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-104
- **Submitter:** Davymutinda77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/104
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-104.md

## Brief Summary

The code lacks explicit access control mechanisms, allowing any function, such as write_storage, to be invoked by arbitrary users. For example, in the write_storage function: func write_storage{ syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr, state: model.State* }(evm_address: felt, key: Uint256*, value: Uint256*) { // No access control let account = get_account(evm_address); let account = Account.write_storage(account, key, value); update_account(account); } This function allows users to write to any account's storage without restrictions. Without proper checks, malicious actors could call this function and modify the storage or state of arbitrary accounts, leading to unau...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unverified Keccak Hash Usage in Ethereum Address Recovery May Lead to Incorrect Address Derivation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-151
- **Submitter:** Davymutinda77
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/151
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-151.md

## Brief Summary

The PrecompileEcRecover contract relies on the ICairo1Helpers interface to generate Keccak hashes for recovering Ethereum addresses. The Keccak hash is essential in this process as it transforms the public key into the corresponding Ethereum address. However, the hash generation is dependent on external helper functions from the ICairo1Helpers interface. If these external helper functions are not implemented correctly or securely, it can lead to incorrect address recovery or potentially even more severe vulnerabilities. Key Issues: Dependency on external helpers: The library_call_keccak function used to generate the Keccak hash is called through ICairo1Helpers. If this function is insecure...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Data Integrity Risks Due to Misaligned Memory Access in Store and Load Functions

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-183
- **Submitter:** Davymutinda77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/183
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-183.md

## Brief Summary

In both the store and load functions, if the offset provided is misaligned (i.e., not a multiple of 16 bytes), the functions engage in complex operations involving the splitting and merging of 128-bit values using masking techniques. If these mask operations encounter issues such as rounding errors or overflow, it can lead to: Corrupted Data: Data written to or read from memory may become incorrect or unusable. Unexpected Contract Behavior: Misalignment can lead to unintended outcomes, affecting the integrity of the entire contract. Potential for Exploits: Malicious actors may exploit this vulnerability to manipulate contract data, potentially leading to significant security breaches. Refer...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# Performance Degradation Risk Due to Inefficient Memory Finalization in finalize Function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-184
- **Submitter:** Davymutinda77
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/184
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-184.md

## Brief Summary

The finalize function, responsible for memory compaction or squashing (via default_dict_finalize), does not impose explicit constraints on the size of the memory region it operates on. This could lead to performance degradation, especially in scenarios where the memory size becomes excessively large. Key risks include: Performance Bottlenecks: High memory usage can lead to slow finalization processes, degrading the overall performance of the contract. Denial of Service (DoS): In extreme cases, if memory compaction becomes too resource-intensive, it could result in a denial-of-service vulnerability, making the contract temporarily unavailable or unusable. Unbounded Memory Growth: Without res...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation in eth_send_raw_unsigned_tx Leading to Potential Overflow and Malleability Attacks

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-198
- **Submitter:** Davymutinda77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/198
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-198.md

## Brief Summary

In the function eth_send_raw_unsigned_tx, the parameters tx_data_len and tx_data are used without any validation. This lack of validation poses a risk of unexpected behavior, such as overflow attacks, transaction malleability, and the potential to pass malformed or malicious input data. Similar validation issues may also exist in other functions like eth_call, eth_send_transaction, etc. Without proper input validation, malicious actors could pass data of incorrect lengths or formats, causing issues such as: Buffer overflows, leading to unexpected behavior or security vulnerabilities. Malleability of transactions, where attackers could modify transaction data in unintended ways. Invalid or c...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# Unchecked Return Values in ERC20 and External Function Calls Could Lead to Loss of Funds and Functionality

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-199
- **Submitter:** Davymutinda77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/199
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-199.md

## Brief Summary

The functions IERC20.transfer(), IERC20.transferFrom(), IERC20.approve(), and ICairo1Helpers.exec_precompile() return critical success indicators, but their return values are not consistently checked in the code. Failure to verify these return values could result in undetected errors, leading to failed transactions or operations without any visible error or revert, especially during ERC20 token transfers and approvals. This introduces potential vulnerabilities such as: Loss of Funds: If an ERC20 transfer fails but is assumed successful, funds may not reach the intended recipient. Failure to Execute Approvals or Precompile Calls: Unchecked failures in approve() or exec_precompile() can preve...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Misuse of replace_class in Upgrade Function Poses Security Risks

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-95
- **Submitter:** Davymutinda77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/95
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-95.md

## Brief Summary

The upgrade() function in the Kakarot contract uses the replace_class syscall to upgrade the contract. This operation, while necessary for upgrades, introduces significant risk. If the owner’s private key is compromised or the upgrade process is mishandled, a malicious actor could replace the entire contract code with malicious logic, potentially leading to unauthorized transfers, loss of assets, or a complete contract takeover. Location: kakarot/src/kakarot/kakarot.cairo, Line 42 replace_class(new_class_hash); Note on Testing: Although testing frameworks such as kakarot/tests/utils/syscall_handler.py implement mock environments for replace_class, these do not affect the real risks in the p...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Overflow when calculating memory_expansion_cost via calculate_gas_extend_memory due to error in memory_cost function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-228
- **Submitter:** Emmanuel
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/228
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-228.md

## Brief Summary

Overflow when calculating memory_expansion_cost via calculate_gas_extend_memory due to error in memory_cost function

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Throughout the codebase, kakarot instructions deals with uint128 offset values, but geth deals with uint64 values.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-268
- **Submitter:** Emmanuel
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/268
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-268.md

## Brief Summary

Throughout the codebase, kakarot instructions deals with uint128 offset values, but geth deals with uint64 values.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# push_uint128 does not check that element is actually < 2^128, leading to data corruption

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-272
- **Submitter:** Emmanuel
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/272
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-272.md

## Brief Summary

push_uint128 does not check that element is actually < 2^128, leading to data corruption

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unauthorized Transaction Execution Due to Missing Account Contract Validation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-243
- **Submitter:** Koolex
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/243
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-243.md

## Brief Summary

Any contract can bypass signature validation by directly calling `eth_send_raw_unsigned_tx`, allowing unauthorized execution of EVM transactions. This breaks the fundamental security model where transactions must be signed by the account owner. The allows: - Execution of transactions without valid signatures - Impersonation of any EVM address as long as nonce matches

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# Nonce Increment Inconsistency in Contract Creation Against Geth

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-252
- **Submitter:** Koolex
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/252
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-252.md

## Brief Summary

Critical inconsistency between Kakarot and Geth in nonce handling during contract creation failures. Geth always increments nonce regardless of error type (except a few such as nonce overflow), while Kakarot only increments nonce in limited scenarios. This causes state divergence between Kakarot and Geth, particularly with cases like max code size exceeded.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_59_group

# Potential Infinite Recursion Leading to Gas Exhaustion in Recursive Functions

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-137
- **Submitter:** MrxSnowden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/137
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-137.md

## Brief Summary

Potential Infinite Recursion Leading to Gas Exhaustion in Recursive Functions

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Flawed Byte-to-uint256 Conversion in L1KakarotMessaging Enables Cross-Layer Message Spoofing

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-132
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/132
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-132.md

## Brief Summary

The L1KakarotMessaging contract serves as a bridge for cross-layer communication between Ethereum (L1) and StarkNet (L2) in the Kakarot project. The consumeMessageFromL2 function is responsible for processing messages sent from L2 to L1. The consumeMessageFromL2 function incorrectly converts a structured byte array into an array of uint256 values, resulting in the loss of crucial data structure and corruption of address information.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Contract Existence Checks Lead to Silent Failures in L1-L2 Communication

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-229
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/229
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-229.md

## Brief Summary

The CairoLib's implementation of Cairo contract calls lacks proper existence checks for both precompile contracts on L1 and target contracts on L2 Starknet. Due to EVM's behavior of returning success for calls to non-existent contracts, this could lead to silent failures in cross-chain communication.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Replay Protection in L2 to L1 Communication Enables Message Duplication

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-234
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/234
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-234.md

## Brief Summary

The `sendMessageToL1` function in the `CairoLib` library is vulnerable to replay attacks. This vulnerability arises from the lack of any mechanism to ensure message uniqueness or prevent the resubmission of previously processed messages. An attacker could intercept a legitimate message and replay it to the `CAIRO_MESSAGE_PRECOMPILE` contract, potentially triggering unintended consequences on the L1 chain. The `sendMessageToL1` function simply forwards the provided `payload` to the `CAIRO_MESSAGE_PRECOMPILE` contract without any checks to prevent replay attacks: This stateless message passing mechanism allows an attacker to capture a message and resubmit it later, as the `CAIRO_MESSAGE_PRECO...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# Message Duplication and State Inconsistency During StarkNet Reorgs

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-235
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/235
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-235.md

## Brief Summary

Background: The StarkNet Block Reorganization On April 4, 2024, StarkNet experienced a significant block reorganization incident due to a rounding error bug. This event led to transaction backlog and disruption in block production, highlighting vulnerabilities in cross-layer messaging systems. The incident demonstrated that applications leveraging StarkNet's cross-layer messaging must be designed with robust reorg protection mechanisms. Description The CairoLib library implementation contains critical vulnerability in its cross-layer messaging and state management functions (`sendMessageToL1` and `callCairo`). The vulnerability becomes particularly acute during StarkNet block reorganization...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect stack handling for `CREATE` and `CREATE2` Opcodes in `exec_create` function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-115
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/115
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-115.md

## Brief Summary

Incorrect stack handling for `CREATE` and `CREATE2` Opcodes in `exec_create` function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Uninitialized caller starknet aaddress after account deployment in `cairo_precompile` function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-71
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/71
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-71.md

## Brief Summary

The `caller_starknet_address` is used in the `IAccount.execute_starknet_call()` function call: If `caller_starknet_address` is uninitialized or incorrect, this call could fail or, worse, execute with the wrong caller address, potentially leading to unauthorized actions or other security issues.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# use memory safe in byteArrayToString

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-100
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/100
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-100.md

## Brief Summary

use memory safe in byteArrayToString

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# wrong implement of pop_n

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-245
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/245
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-245.md

## Brief Summary

wrong implement of pop_n

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inadequate Authorization in set_balance

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-165
- **Submitter:** bhatmuneeb
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/165
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-165.md

## Brief Summary

Inadequate Authorization in set_balance

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked Return Value from dict_write in Multiple Locations

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-169
- **Submitter:** bhatmuneeb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/169
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-169.md

## Brief Summary

Unchecked Return Value from dict_write in Multiple Locations

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Resource Constraints in fetch_or_create Function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-170
- **Submitter:** bhatmuneeb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/170
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-170.md

## Brief Summary

Missing Resource Constraints in fetch_or_create Function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# Potential for Inconsistent Account Storage Initialization

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-171
- **Submitter:** bhatmuneeb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/171
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-171.md

## Brief Summary

Potential for Inconsistent Account Storage Initialization

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Front-Running Vulnerability in Account Registration

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-175
- **Submitter:** bhatmuneeb
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/175
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-175.md

## Brief Summary

Front-Running Vulnerability in Account Registration

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# "Denial of Service (DoS) Vulnerabilities in `saturated_sub`, `initialize_jumpdests`, and `try_parse_destination_from_bytes` due to Unchecked Return Values, Improper Input Validation, and Potential Infinite Loops"

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-277
- **Submitter:** coders99
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/277
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-277.md

## Brief Summary

- In the `saturated_sub` function, there’s a return value of 0 if the subtraction result is negative. This means that an attacker can theoretically keep calling this function with values that lead to repeated calls with predictable results, thus exhausting the computational resources. 2.Denial of Service: - In the `initialize_jumpdests` function, improper handling of bytecode lengths could lead to infinite loops or excessive memory allocation for unsuccessful jump verifications. 3.Potential Infinite Loops: - The `try_parse_destination_from_bytes` function incorrectly confirms the bytes format which may lead to unnecessary computations or a potential DoS vector if the provided values are not...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# starknet has Under-Constrained Computations in Internal Function _save_valid_jumpdests

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-105
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/105
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-105.md

## Brief Summary

Within the Internals namespace of the Cairo contract, the _save_valid_jumpdests function is designed to iterate through a dictionary of jump destinations (dict_start to dict_end) and store valid jump destinations into the StarkNet account storage. Although this function is internal and not directly exposed to external actors, the current implementation presents potential risks related to under-constrained computations that could affect the contract’s reliability and performance. The loop termination condition relies on the difference between dict_end and the current dict pointer: If the parameters dict_start and dict_end are not strictly controlled or validated before invocation, there’s a...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# account has Incorrect Assignment of code_hash in set_code Function Leading to Under-Constrained Computation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-106
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/106
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-106.md

## Brief Summary

The set_code function within the Account namespace incorrectly assigns the code_hash of an account. Instead of utilizing the result from the compute_code_hash function, it erroneously derives code_hash from the allocation pointer (ap). This oversight leads to under-constrained computations, allowing potential manipulation of the code_hash, which is critical for account integrity and security. Vulnerable Code The code_hash does not reflect the actual hash of the provided code, undermining the contract’s integrity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# DualVmToken has Unchecked transferFrom Allows Unauthorized Token Transfers

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-66
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/66
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-66.md

## Brief Summary

The transferFrom function in the DualVmToken contract lacks proper access control checks. Allowing any caller to transfer tokens from any account to any other account without authorization. This vulnerability can lead to a complete loss of funds for token holders. As attackers can arbitrarily transfer tokens from any address. Vulnerable Code File: DualVmToken.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_69_group

# L2KakarotMessaging has Unrestricted Access to sendMessageToL1 Allows Unauthorized L2 to L1 Messaging

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-67
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/67
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-67.md

## Brief Summary

The sendMessageToL1 function in the L2KakarotMessaging contract lacks proper access control. Allowing any external account to invoke it and send arbitrary messages from L2 to L1. This can lead to unauthorized actions on the L1 contract. Vulnerable Code

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Chain ID Calculation in Kakarot Contracts May Lead to Security and Compatibility Issues

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-129
- **Submitter:** eta
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/129
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-129.md

## Brief Summary

The Kakarot contracts exhibit an inconsistency in how they calculate the `chain_id`. While most of the code uses the `unsigned_div_rem` function to limit the `chain_id` to 53 bits, ensuring compatibility with systems like JavaScript, the `eth_transaction.cairo` contract's decoding functions (`decode_legacy_tx`, `decode_2930`, and `decode_1559`) rely on `bytes_to_felt`, which does not enforce this limit. This discrepancy can lead to several problems, including potential security risks from chain ID mismatches, possible overflow issues, repay attacks, and integration difficulties with external systems expecting a uniform format.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Bypassing `assert_only_self`: Delegate Call Vulnerability and Reentrancy Risks in accounts/library.cairo Contracts

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-130
- **Submitter:** eta
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/130
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-130.md

## Brief Summary

Bypassing `assert_only_self`: Delegate Call Vulnerability and Reentrancy Risks in accounts/library.cairo Contracts

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Pointer Misuse in StarkNet Account Commit Function Can Lead to Memory Errors and Vulnerabilities

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-148
- **Submitter:** eta
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/148
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-148.md

## Brief Summary

The `_commit_account` function in the `starknet.cairo` contract is designed to commit account state to StarkNet’s storage backend. However, while creating the `model.Transfer` structure, the `burn_address` field is passed as a value, whereas the `recipient` field is expected to be a pointer (`Address*`). Passing the `recipient` as a value (`Address`) instead of a pointer can cause type mismatches, resulting in either compilation errors or memory access issues. This can lead to crashes, unpredictable behavior, or even security vulnerabilities like unauthorized memory access.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Issues and Recommendations for First Element in `get_constructor_calldata` in Cairo Contracts

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-222
- **Submitter:** eta
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/222
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-222.md

## Brief Summary

The `get_constructor_calldata` function in the `account.cairo` contract hardcodes the first element of constructor data to `1`, which could lead to challenges with version control, cross-contract compatibility, and code maintainability. Hardcoding a value that may be used for versioning or validation risks security and adaptability issues. To address this, it is recommended to replace hardcoding with a constant or configurable variable, include clear comments, and consider using enums or a version control mechanism to enhance flexibility and maintainability.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Issues with Address Conversion and Validation in DualVmToken.sol Contract

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-77
- **Submitter:** eta
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/77
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-77.md

## Brief Summary

The `DualVmToken.sol` contract has issues with how Ethereum and StarkNet addresses are handled in functions like `approve` and `transfer`. Specifically, when an Ethereum address is used as `msg.sender`, it may be converted into a StarkNet address that is greater than or equal to `STARKNET_FIELD_PRIME`. Similarly, StarkNet addresses might also exceed this limit. The `_approve` and `_transfer` functions do not currently handle these conversions or validate if the converted or original StarkNet address is within the allowed range. This causes problems with querying `balanceOf` and `allowance`, and prevents authorized spenders from using the `transferFrom` function to transfer tokens on behalf...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_97_group

# Underconstrained `execute_from_outside` Leading to Arbitrary Transaction Manipulation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-136
- **Submitter:** g_x_w
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/136
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-136.md

## Brief Summary

The `execute_from_outside` function in `account_contract.cairo` processes user transactions within the EVM. However, it lacks validation for `data_offset` in the `call_array` and `tx_data_len` in the `packed_tx_data`, leading to potential out-of-bounds access in calldata. This vulnerability allows a malicious prover to manipulate `packed_tx_data`, which could result in unintended transaction execution. The impact is critical, as it enables arbitrary transaction manipulation in the EVM. Affected Code The issue occurs in the following section of the code: Vulnerability Description The function does not validate whether `[call_array].data_offset` is within the bounds of `calldata`. If `data_of...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Out of Bounds Read in 2024-09-kakarot/kakarot/src/kakarot/errors.cairo

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-180
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/180
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-180.md

## Brief Summary

Out of Bounds Read in 2024-09-kakarot/kakarot/src/kakarot/errors.cairo

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_51_group

# Missing Return Values in IERC20.transferFrom.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-146
- **Submitter:** lightoasis
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/146
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-146.md

## Brief Summary

Some tokens do not return a bool (e.g. USDT, BNB, OMG) on ERC20 methods. see [here](https://gist.githubusercontent.com/lukas-berlin/f587086f139df93d22987049f3d8ebd2/raw/1f937dc8eb1d6018da59881cbc633e01c0286fb0/Tokens%20missing%20return%20values%20in%20transfer) for a comprehensive (if somewhat outdated) list. Some tokens (e.g. BNB) may return a bool for some methods, but fail to do so for others. This resulted in stuck BNB tokens in Uniswap v1 ([details](https://mobile.twitter.com/UniswapProtocol/status/1072286773554876416)). Some particularly pathological tokens (e.g. Tether Gold) declare a bool return, but then return false even when the transfer was successful ([code](https://etherscan.i...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Anyone can transfer ownership of the contract to themselves because of missing access control.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-147
- **Submitter:** lightoasis
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/147
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-147.md

## Brief Summary

Users can transfer ownership of the contract due to missing access control. Vulnerability details If we look through the admin section of the kakarot contract, (see [admin section](https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/kakarot.cairo#L32-#L71)), we see that only the owner should be able to execute the following commands: * Pause / unpause the contract * Upgrade the contract * Transfer ownership of the contract, etc. But because of a missing access control on the `transfer_ownership` function, non-admin users can transfer ownership of the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_104_group

# Attackers can steal user's tokens because there is no check for token approval in `transferFrom`.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-63
- **Submitter:** lightoasis
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/63
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-63.md

## Brief Summary

There is no check for token allowance before transferring tokens in the `transferFrom` function. This could result in user's tokens being stolen. Vulnerability details The `transferFrom` function transfers tokens from one user to another without checking to ensure that the caller (msg.sender) has the approval of the user to transfer their tokens. this allows attackers to steal users tokens by transferring tokens from the user without the user's approval.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_37_group

# Event length is not sufficiently validated when committing the message

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-141
- **Submitter:** minglei-wang-3570
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/141
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-141.md

## Brief Summary

Event length is not sufficiently validated when committing the message

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inconsistent gas refund logic comparing to geth implementation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-258
- **Submitter:** minglei-wang-3570
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/258
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-258.md

## Brief Summary

In geth implementation, the code handles the [gas refund below](https://github.com/ethereum/go-ethereum/blob/80bdab757dfb0f6d73fb869d834979536fe474e5/core/state_transition.go#L459): and [st.refundGas](https://github.com/ethereum/go-ethereum/blob/80bdab757dfb0f6d73fb869d834979536fe474e5/core/state_transition.go#L496) the code actually add the refunded gas back to the user account. Instead of charge the full gas, then refund at most 4 / 5 gas amount, the code in kakarot charge [at most 4 / 5 directly.](https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/interpreter.cairo#L1032) where [total_gas_used](https://github.com/kkrt-labs/kakarot/blob/7411a552...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_72_group

# Incorrect Handling of Signed Integer Conversion in `pow()` Function Can Lead to Panics

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-181
- **Submitter:** mrjorystewartbaxter
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/181
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-181.md

## Brief Summary

The `pow()` function in the `Exponentiation` from `math.cairo` (https://github.com/kkrt-labs/kakarot-ssj/blob/d4a7873d6f071813165ca7c7adb2f029287d14ca/crates/utils/src/math.cairo) trait does not correctly validate the base value, leading to potential panics in an edge case involving the conversion of negative signed integers to unsigned types. This flaw can trigger unintended behavior, particularly when using type conversions. **Vulnerability Details** The `pow()` function expects an **unsigned integer** as the base, and the current test cases only account for unsigned inputs. Although the function doesn't directly accept signed integers, an edge case arises when a negative signed integer i...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Public Key Validation for secp256r1 Coordinates

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-182
- **Submitter:** mrjorystewartbaxter
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/182
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-182.md

## Brief Summary

The `verify_signature_secp256r1()` function from cairo1_helpers.cairo (https://github.com/kkrt-labs/kakarot-ssj/blob/d4a7873d6f071813165ca7c7adb2f029287d14ca/crates/contracts/src/cairo1_helpers.cairo#L206) does not explicitly verify whether the provided `x` and `y` coordinates represent a valid point on the secp256r1 elliptic curve before the syscall. This omission could lead to potential issues if invalid public key coordinates are used during the verification process. Description In the function `verify_signature_secp256r1()`, the public key is constructed using the `x` and `y` coordinates provided as inputs. However, there is no explicit check to verify that these coordinates form a vali...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# Incorrect Function Name

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-58
- **Submitter:** rare_one
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/58
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-58.md

## Brief Summary

The function name _transfer_eth is misleading, suggesting that it transfers Ether (ETH) instead of ERC20 tokens. And If this function is intended to transfer ETH, then using the IERC20.transferFrom method is incorrect. ETH transfers should be done through native Cairo or StarkNet-specific mechanisms for sending ETH (e.g., via system calls or contract logic for ETH). This function suggests confusion between ETH and ERC20 logic. Impact: Confusion: The misleading name could lead to confusion for developers or users, potentially resulting in incorrect usage or unintended consequences. Misuse: If developers mistakenly believe the function transfers ETH, they might use it in contexts where it's n...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_106_group

# Unchecked Return Value from fetch_balance Leading to Uninitialized Pointer Dereference

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-70
- **Submitter:** rare_one
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/70
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-70.md

## Brief Summary

The function allocates memory for balance_ptr but doesn't check if fetch_balance actually returns a value. If fetch_balance fails, the program might attempt to write to an uninitialized pointer balance_ptr Allocation: The line local balance_ptr: Uint256*; declares a local pointer to Uint256, but it does not allocate or initialize memory for this pointer yet. fetch_balance Call: The function then calls fetch_balance(address) to retrieve the balance associated with the address. However, there is no check to ensure that fetch_balance successfully returns a value or that the value it returns is valid. Vulnerability: If fetch_balance fails or returns null/undefined/invalid data (e.g., due to an...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Array Initialization/Out-of-Bounds Memory Access Due to Pointer Reassignment and Memory Initialization

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-78
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/78
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-78.md

## Brief Summary

The vulnerability identified is related to the initialization and subsequent manipulation of the arr_x array. Here's a breakdown of the issue: Initialization: The code initially initializes the arr_x array with 12 bytes of zeros using the memset function. This effectively fills the first 12 bytes of the array with zeros. Shifting: In the following line, the arr_x array is shifted by 12 bytes using the arr_x: felt* = arr_x + 12 statement. This effectively moves the pointer to arr_x to point to the 13th byte of the original array. Uninitialized Bytes: As a result of this shifting, the first 12 bytes of the original array, which were filled with zeros during initialization, are now effectively...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Incorrect Stack Initialization Due to Duplicate Pointer Assignment

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-80
- **Submitter:** rare_one
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/80
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-80.md

## Brief Summary

In the following line of the init function: **return new model.Stack(dict_ptr_start, dict_ptr_start, 0);** The same dictionary pointer (dict_ptr_start) is passed twice to the model.Stack constructor, which might lead to an incorrect initialization of the stack. The stack structure appears to expect three arguments: two pointers (dict_ptr_start and dict_ptr) and a size. Typically, the first pointer represents the starting location of the stack, while the second pointer is expected to represent the current location within the stack. By passing the same pointer (dict_ptr_start) twice, the stack’s starting point and current point are initialized to the same value, which may not reflect the actu...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Underflow in charge_gas() could lead to a transaction being executed indefinitely

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-278
- **Submitter:** superpozycja
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/278
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-278.md

## Brief Summary

***Submitter note**: This is speculation on future code. This issue is currently not exploitable, and some stars have to align for it to become as such - but if it did happen, the consequences seem severe. This edge case is not really outlined in the [severity categorization doc](https://docs.code4rena.com/awarding/judging-criteria/severity-categorization). Therefore, according to [other risk assessment techniques](https://en.wikipedia.org/wiki/Risk_matrix), I'm defaulting to a Medium risk issue assuming the possibility of downgrading.* The tempvar `a` [might underflow](https://github.com/kkrt-labs/kakarot/blob/038b3a3fa66cd1b8959665fed3e2eb7934e146b1/src/kakarot/evm.cairo#L173) if the `amo...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The function `ICairo1Helpers::library_call_keccak()` is not implemented.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-218
- **Submitter:** wasm_it
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/218
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-218.md

## Brief Summary

The function `ICairo1Helpers::library_call_keccak()` is not implemented.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient validation of items_len

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-262
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/262
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-262.md

## Brief Summary

Insufficient validation of items_len

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Use iteration instead of recursion in the parse_storage_keys function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-266
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/266
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-266.md

## Brief Summary

Use iteration instead of recursion in the parse_storage_keys function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# _emit_events does not record the evm contract from which the event was issued

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-116
- **Submitter:** zhaojie
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/116
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-116.md

## Brief Summary

The off-chain node or cross-chain bridge cannot work

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `handle_l1_message` may fail to execute

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-209
- **Submitter:** zhaojie
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/209
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-209.md

## Brief Summary

`handle_l1_message` fail to execute. However, the ETH of the user has been deducted from l1, resulting in a loss of user funds.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# BlockInformation.selfbalance without calculating gas fee

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-97
- **Submitter:** zhaojie
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/97
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-97.md

## Brief Summary

the gas fee is not calculated, it may cause a Dos attack.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary
