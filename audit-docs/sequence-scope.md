## Publicly known issues

_Anything included in this section is considered a publicly known issue and is therefore ineligible for awards._

### Configurational Assumptions

* There are multiple ways for a user to brick the wallet (e.g., setting an invalid imageHash, using a set of signers that doesn't reach the threshold, losing the contents of the tree, etc.); these contracts are meant to be used with an SDK that guards against those scenarios, which are beyond the scope of the audit.
* Hooks are meant to have admin privileges; it is expected that once installed, they have full rein to affect the wallet. The SDK guards against installing non-whitelisted hooks.
* It is possible to define a configuration tree so large that attempting to use it may cause out-of-gas errors; this is a known issue and the SDK guards against that scenario.

### Off-Chain Assumptions

* Exploits that involve tricking a relayer into relaying a transaction that fails and never pays for gas are out of scope; the relayer has its own layer of protections that are independent from the contracts.
* It is possible to desync the wallet across chains if the signers sign configuration updates with `chainId != 0`, or if they perform a one-off configuration or implementation update on-chain. The signers are responsible for not doing this to keep the chains in sync; the SDK handles it automatically.

### Operational Assumptions

* Rule changes immediately reset usage limits for a permission; this is a known issue.
* Calls with value are not forwarded to the implementation; this is by design. The `payable` modifiers are a gas optimization to avoid checking `msg.value` twice.

### Signer Assumptions

* When sending a transaction, the signers have free rein to update the wallet configuration and implementation; this is by design. The calls can be restricted if needed using sapient signers.
* Intermediary configurations of the state channel (that haven't been invalidated by the checkpointer) are usable. To properly evict a removed signer, the configuration has to be updated on-chain or the checkpointer must reflect the change.
* Signature malleability is not a concern, as signatures are not expected to be unique.

# Overview

Sequence Ecosystem Wallet is a non-custodial smart wallet designed for chains and ecosystems. It combines passkeys, social auth, timed recovery keys, and sandboxed permissions to deliver higher security with less friction.

The codebase represents the V3 implementation of this infrastructure, utilizing a minimal proxy pattern and a novel Merkle-proof based configuration approach for smart wallets.

## Links

- **Previous audits:**  Audits can be found here: 
    - Consensys Diligence: https://github.com/0xsequence/wallet-contracts-v3/blob/master/audits/consensys-audit.pdf
    - Quantstamp Audit: https://github.com/0xsequence/wallet-contracts-v3/blob/master/audits/quantstamp-audit.pdf
    - Rotcivegaf Audit: https://github.com/0xsequence/wallet-contracts-v3/blob/master/audits/rotcivegaf-audit.md
- **Documentation:** https://github.com/0xsequence/wallet-contracts-v3/tree/master/docs
- **Website:** https://sequence.xyz/
- **X/Twitter:** https://x.com/0xsequence

---

# Scope

### Files out of scope

| File         |
| ------------ |
| [script/\*\*.\*\*](https://github.com/code-423n4/2025-10-sequence/tree/main/script) |
| [src/Estimator.sol](https://github.com/code-423n4/2025-10-sequence/blob/main/src/Estimator.sol) |
| [src/Simulator.sol](https://github.com/code-423n4/2025-10-sequence/blob/main/src/Simulator.sol) |
| [test/\*\*.\*\*](https://github.com/code-423n4/2025-10-sequence/tree/main/test) |

# Additional context

## Areas of concern (where to focus for bugs)

1. Privilege escalation either from a signer that belongs to the configuration or from a non-signer, allowing it to sign transactions on behalf of the wallet bypassing the threshold.
2. Correctness of the checkpointer and the chained signatures.
3. Malleability of packed payloads.
4. Privilege escalation within smart sessions.
5. Timelock bypasses on the recovery module.

## Main invariants

### Authorized Signers Only

Only the wallet’s designated signers (meeting the required signing threshold) can execute transactions or make state changes. No external party can operate the wallet without a valid EIP-712 signature from the correct signer set. The contract enforces this by requiring a proper signature for every execute call – if the signature check fails, the transaction is rejected.

### Image Hash & Configuration Integrity

Each Sequence wallet is defined by an image hash that encodes its owner configuration (the set of signer addresses and their threshold scheme). This image hash is tied to the wallet’s deployment address. On deployment, the wallet contract checks that its own address was generated using the image hash (via CREATE2 with the factory) and stores this hash on-chain. Every future transaction recomputes the image hash from the current config and compares it to the stored value, ensuring the signer set or threshold cannot be tampered with undetected. In short, the wallet’s address and its authorized signer configuration are cryptographically bound – any unauthorized change breaks the hash check and invalidates signatures.

### Deterministic Wallet Address per Config

Given the above, a particular signer configuration always corresponds to a single unique wallet address. The factory uses the image hash as a salt to deploy the wallet, meaning the mapping between a wallet’s config and its address is one-to-one. This prevents an attacker from, say, front-running the deployment of a user’s wallet with a different contract – the address is predetermined by the intended signers. No two distinct configs will produce the same address, and the same config cannot be deployed twice on the same network. After deployment, two wallets could upgrade their image hash to the same configuration.

### Strict Nonce Sequencing

Sequence wallets implement a multi-space nonce system to prevent replay attacks. Each wallet has independent nonce “spaces” (to allow parallel sequence streams), and in each space the nonce must match exactly the next expected value. Nonces increment sequentially per space and cannot be reused. If a transaction’s provided nonce is out of sequence for that space, it will be rejected as an INVALID_NONCE. This invariant guarantees proper ordering of transactions and that each signed transaction is unique to a single execution.

### Domain-Separated Signatures

All signatures are domain-separated and network-specific. The wallet’s EIP-712 signing scheme includes the current chain ID and the wallet’s address in the hashed message. This means a signature intended for one particular Sequence wallet on one network cannot be replayed on a different wallet or chain. The contract explicitly pulls the chain ID in at hash time and prefixes the data with `0x19_01 || chainId || address(this)`, binding the signature to that wallet instance. This invariant protects against cross-chain or cross-contract replay of signed messages.

### Privileged Operations Require Self-Call

Sensitive operations on the wallet (such as upgrading the implementation, adding/removing module hooks, or deploying new contracts from the wallet) are guarded by a modifier onlySelf. This means the function can only be called by the wallet itself (i.e. via an internal delegatecall from the wallet’s own context) and never by an external EOA or unprivileged contract . For example, the updateImplementation function (used to upgrade the wallet’s logic) is onlySelf, so it can only execute if initiated from an authorized wallet transaction, and cannot be invoked by an attacker directly. This invariant ensures no admin or external contract can unilaterally change the wallet’s state – only the wallet’s owners, via a proper signed transaction, can trigger such changes.

### All External Actions Go Through execute

Users interact with their Sequence wallet exclusively via the execute function (or meta-transaction workflows that ultimately call execute). There is no alternative public method to trigger arbitrary calls from the wallet without signature verification. Even batched calls are executed internally by _execute after the signature and nonce have been validated. This invariant means there’s no “backdoor” to bypass authentication – every funds transfer or contract call from the wallet is explicitly authorized by the wallet’s signers.

### ERC-4337 Integration

Sequence wallets fully support ERC-4337 account abstraction by implementing the required validateUserOp interface. When a User Operation is submitted through an ERC-4337 entrypoint, the wallet validates that the sender is the configured entrypoint contract and processes the operation through the executeUserOp function. The validateUserOp function calls the wallet's own ERC-1271 isValidSignature function to validate signature correctness, where the signature is of the userOpHash rather than the Payload contents. This design protects against replay attacks by ensuring that signatures are bound to the specific User Operation hash, which includes critical fields like nonce, gas parameters, and operation data. The User Operation's data field contains a nested Sequence Payload, which gets forwarded to the wallet's selfExecute function. This selfExecute call follows the same execution flow as the standard _execute function described above. This invariant guarantees that ERC-4337 operations maintain the same security guarantees as direct wallet interactions – the account abstraction layer cannot bypass the wallet's core authentication mechanisms or authorization requirements.

### Batched Transactions & Atomicity

The wallet supports batching multiple actions in a single execute call for efficiency. By default, the batch is atomic – if any call in the batch fails and is marked as critical, the entire batch will revert. However, the wallet allows certain calls to be flagged as non-critical (revertOnError = false), in which case a failure of that call will not stop the batch: it will emit a TxFailed event for that specific sub-transaction and continue with the next one. This invariant ensures that optional or best-effort operations can be attempted without jeopardizing the main transaction, while still transparently logging any failures. Importantly, a sub-call failing without revertOnError cannot corrupt subsequent calls – the revert is trapped and the wallet moves on, maintaining overall state consistency for the rest of the batch.

### Contract Signers and ERC-1271

Sequence wallets can have other smart-contract wallets or contracts as signers (not just EOAs), and the wallet fully supports nested signatures via ERC-1271 and the new Sapient Signer interface. If a signer is a contract, the Sequence wallet will call that contract’s isValidSignature method to confirm that the payload was approved by that contract’s logic. A contract signer only counts as valid if its own internal approval check returns true, per ERC-1271. This means adding a contract (even another Sequence wallet) as a signer does not bypass the signature requirement – it simply shifts it to that contract’s own signature/approval mechanism. The system even supports multiple layers of nested Sequence wallets as signers, as covered in tests (e.g. wallets signing for wallets), all of which must resolve to true approvals. Invariantly, a signature from a contract signer is treated with the same rigor as a human signer: no contract signer can “auto-approve” transactions unless explicitly programmed to, and it cannot be used to circumvent the threshold or nonce rules.

### Sapient Signers

Sapient signers represent an advanced interface designed to support more complex Sequence wallet signer configurations. While ERC-1271 can only validate a hash and signature by returning the magic value, a Sapient Signer receives the complete transaction payload and signature and is expected to return its configuration or image hash. This returned image hash is then used by the Sequence wallet to reconstruct the wallet's image hash. Since the signer's image hash is derived for every payload and signature combination, a sapient signer can counterfactually determine its image hash without relying on on-chain state. This capability is achieved by encoding the intended sapient signer configuration directly within the signature being validated. As a result, a wallet can support a specific configuration of the sapient signer without requiring updates to that contract's state, enabling more flexible and gas-efficient signer management. The sapient signer may validate the contents of the payload or signature in whichever way it deems fit.

### Controlled Module Hooks

The wallet allows installing hook modules to handle specific function selectors (for example, to custom-handle incoming token transfers or to extend wallet functionality). These hooks are strictly controlled by the wallet’s owners. Only one hook implementation can be registered per function signature at any time, and adding or removing a hook can only be done via a valid wallet transaction (which, as noted, requires signer authorization and onlySelf). If a hook is set for a function, the wallet’s fallback will delegatecall into the hook’s contract when that function is invoked; if no hook is set, such calls are simply ignored by the wallet’s fallback (no action taken, aside from possibly receiving ETH). Hooks do not get to override the wallet’s security model – they execute within the wallet context under the same onlySelf restrictions for any state changes. In essence, a hook can extend functionality but cannot, for example, surreptitiously initiate an execute on its own. The invariant here is that hooks augment the wallet but cannot violate its core access controls.

### Non-Privileged Helper Modules

The Sequence system includes certain helper modules (e.g. Estimator, Simulator, and a Guest module for new wallets) which have no privileged rights in the protocol. These components are used for off-chain simulation, gas estimation or temporary guest session logic, and they cannot modify wallet state or perform sensitive actions. Invariants are not impacted by these modules – they operate with read-only or strictly limited scope. This means auditors and users can largely ignore these modules in terms of security critical paths, as they cannot bypass authorization or affect funds. All the critical invariants remain focused on the core wallet, its factory, and the authorized modules described above.


## All trusted roles in the protocol
N/A

## V12 findings

[V12](https://v12.zellic.io/) is [Zellic](https://zellic.io)'s in-house AI auditing tool. It is the only autonomous Solidity auditor that [reliably finds Highs and Criticals](https://www.zellic.io/blog/introducing-v12/). 

**NOTE**: All issues found by V12 will be judged as out of scope and ineligible for awards.

V12 findings can be viewed below

## Critical Severity Findings from V12 - ALL THESE ARE OUT OF SCOPE


### Arbitrary Module Injection in Wallet Clone Factory

**Severity:** Critical  

**Affected Contract(s):**
- `Factory`

**Affected Function(s):**
- `deploy()`

**Description:**

The deploy function allows anyone to specify any address as the `_mainModule` without any validation or access control. This address is embedded directly into the creation bytecode of a new Wallet clone. If the Wallet contract trusts this module address for core logic (e.g., via delegatecalls), an attacker can deploy a wallet pointing to a malicious module implementation, effectively gaining arbitrary control over the wallet’s behavior.

---

### Undefined behaviorOnError value slip leads to unhandled execution path

**Severity:** Critical    

**Affected Contract(s):**
- `Connext (core)`

**Affected Function(s):**
- `_execute()`

**Description:**

The `fromPackedCalls` function in the `Payload` library decodes a 2‐bit `behaviorOnError` value from the high bits of each call’s `flags` byte. Although only three behaviors (0–2) are defined (`IGNORE`, `REVERT`, `ABORT`), the raw 2‐bit slice can yield 3. The execution logic in Connext’s `_execute` function switches on `behaviorOnError` but only handles cases 0, 1, and 2. If `behaviorOnError == 3`, no branch matches, causing an unexpected fall‐through or silent skip with undefined or unsafe behavior.

---

### Unverified Merkle Proof Allows Arbitrary Root Forging

**Severity:** Critical  

**Affected Contract(s):**
- `Recovery`

**Affected Function(s):**
- `_recoverBranch()`

**Description:**

The `_recoverBranch` function sets a `verified` flag if any single recovery leaf meets timing checks, then accumulates a Merkle‐style `root` by hashing in all subsequent `FLAG_NODE` and `FLAG_BRANCH` values straight from calldata. Because the contract never compares this computed `root` against an on-chain commitment, an attacker who controls one valid leaf can supply arbitrary node and branch data to forge any desired root.

---

### Incorrect Handling of Initial Branch in Merkle Proof Reconstruction

**Severity:** Critical  

**Affected Contract(s):**
- `Recovery`

**Affected Function(s):**
- `_recoverBranch()`

**Description:**

In the FLAG_BRANCH case, the code always computes root = fkeccak256(root, nroot) without checking if root is zero. When the first proof element is a branch, this produces keccak256(0‖subtreeRoot) instead of subtreeRoot, corrupting the reconstructed Merkle root.

---

### Cumulative Usage Updates Not Persisted

**Severity:** Critical  

**Affected Contract(s):**
- `PermissionValidator`

**Affected Function(s):**
- `validatePermission()`

**Description:**

The validatePermission function constructs a newUsageLimits array and inserts or locates UsageLimit structs for cumulative rules. However, after updating usageLimit.usageAmount in memory, it never writes this modified struct back into newUsageLimits[j]. As a result, the returned newUsageLimits always contains stale usage amounts (defaulting to zero or the previous on-chain value), breaking cumulative limit tracking.

---

### Missing Persistence of Updated UsageAmount in Cumulative Rules

**Severity:** Critical  

**Affected Contract(s):**
- `PermissionValidator`

**Affected Function(s):**
- `validatePermission()`

**Description:**

Within the cumulative‐rule branch, the function reads or creates a UsageLimit entry into a local `usageLimit` variable, updates `usageLimit.usageAmount`, but never writes this updated struct back to the `newUsageLimits` array for existing entries. As a result, the returned `newUsageLimits` always contains stale or zero `usageAmount` values rather than the intended cumulative totals.

---

### Lost Updates on Cumulative Usage Limits

**Severity:** Critical  

**Affected Contract(s):**
- `PermissionValidator`

**Affected Function(s):**
- `validatePermission()`

**Description:**

When handling cumulative rules, the function copies an existing UsageLimit from the in-memory newUsageLimits array into a local struct, updates its usageAmount, but never writes it back into newUsageLimits. As a result, any previous usage is retrieved correctly, but the updated value is lost and the returned newUsageLimits array contains stale usage values. This allows callers to repeatedly satisfy cumulative limits and bypass intended caps.

---

### Threshold Overwritten in recoverChained

**Severity:** Critical  

**Affected Contract(s):**
- `BaseSig`

**Affected Function(s):**
- `recoverChained()`

**Description:**

In the loop over chained signature segments, the named return variable `threshold` is reassigned each iteration by calls to `recover(...)`. No variable preserves the initial (root) segment’s threshold. As a result, recoverChained returns the threshold of the last segment while `opHash` remains that of the first segment, causing callers to apply the wrong (lower) threshold to the original operation hash.

---

### Incorrect Flags Offset in WebAuthn Authenticator Data

**Severity:** Critical  

**Affected Contract(s):**
- `WebAuthn`

**Affected Function(s):**
- `verify()`

**Description:**

The inline assembly in the `verify` function reads the user presence (UP) and user verification (UV) flags from the wrong memory offset in `authenticatorData`. It uses `mload(add(mload(auth), 0x21))`, which points into the `rpIDHash` region rather than the single-byte flags field immediately after the 32-byte `rpIDHash`. As a result, the code never actually masks and checks the real UP and UV bits.

---

### Unchecked Clearing of Snapshot.imageHash Allows Replay of Stale Segments

**Severity:** Critical  

**Affected Contract(s):**
- `BaseSig`

**Affected Function(s):**
- `recoverChained()`

**Description:**

Within recoverChained, the code zeroes out the passed-in Snapshot.imageHash immediately upon matching any segment’s imageHash—even if it’s not the final segment. Because the final UnusedSnapshot guard only reverts when Snapshot.imageHash is still non-zero, an attacker can first match an intermediate segment to clear imageHash, then supply a last segment with a stale checkpoint (≤ original snapshot.checkpoint) and bypass the guard entirely, resulting in replay of old segments.

---


## Medium Severity Findings


### Factory Allows Deploying Proxies Pointing to Zero Implementation

**Severity:** Medium  

**Affected Contract(s):**
- `Factory`

**Affected Function(s):**
- `deploy()`

**Description:**

The Factory.deploy function lacks validation of the _mainModule address. It simply packs the provided address into the Wallet.creationCode and uses CREATE2 to deploy a proxy. If _mainModule is zero, the resulting proxy stores a zero implementation and will revert on every call, rendering the proxy unusable.

---

### Unaligned Memory Write Corrupts JSON Region

**Severity:** Medium  

**Affected Contract(s):**
- `WebAuthn`

**Affected Function(s):**
- `tryEncodeAuthCompact()`

**Description:**

After copying clientDataJSON of arbitrary non-32-aligned length into memory, the code uses mstore at the end pointer without padding. Because mstore always writes a full 32 bytes, if clientDataJSON.length % 32 ≠ 0 it will overwrite bytes immediately following the JSON data, corrupting the freshly copied JSON region.

---

### Missing clientDataJSON Length Prefix in Compact Encoding

**Severity:** Medium  

**Affected Contract(s):**
- `WebAuthn`

**Affected Function(s):**
- `tryEncodeAuthCompact()`

**Description:**

In tryEncodeAuthCompact, the helper copyBytes writes a 2-byte length prefix before each dynamic field, then advances the write pointer by c_ bytes so the prefix remains visible. However, when encoding clientDataJSON, copyBytes is invoked with c_ = 0, causing o_ not to advance and the subsequent data copy to overwrite the just-stored length prefix. Consequently, the length header for clientDataJSON is entirely lost in the final byte array.

---

### Inconsistent Error Handling in Signature Validation Breaking ERC-1271 Semantics

**Severity:** Medium  

**Affected Contract(s):**
- `BaseAuth`

**Affected Function(s):**
- `signatureValidation / isValidSignature()`

**Description:**

The BaseAuth contract’s signatureValidation function reverts on certain invalid signature conditions (expired static signature, wrong static signer, insufficient weight) but returns `false` on others (invalid image). The isValidSignature function then returns `bytes4(0)` only when it sees `false`, but lets reverts bubble up for the other failure paths. This inconsistent handling leads to a mix of reverts and return codes, violating ERC-1271 expectations for uniform failure semantics.

---

### Inconsistent Error Handling in isValidSignature

**Severity:** Medium  

**Affected Contract(s):**
- `BaseAuth`

**Affected Function(s):**
- `isValidSignature()`

**Description:**

The `isValidSignature` function in BaseAuth inconsistently handles invalid signatures: static-signature failures revert, while dynamic-signature failures return false (mapped to a zero return). This behavior violates the ERC-1271 spec, which expects invalid signatures to return `0x00000000` without reverting.

---

### Unchecked external signature validation reverts in validateUserOp causes denial of service

**Severity:** Medium   

**Affected Contract(s):**
- `ERC4337v07`

**Affected Function(s):**
- `validateUserOp()`

**Description:**

The function validateUserOp makes an external call to this.isValidSignature(userOpHash, userOp.signature) without any try/catch or error handling. The underlying implementation in BaseAuth.signatureValidation can revert for expired static signatures, wrong caller addresses, or insufficient signature weight. Those reverts bubble up and abort validateUserOp instead of returning the intended SIG_VALIDATION_FAILED error code, allowing an attacker to craft a signature that intentionally triggers a revert and block all user operations for that account.

---

### SignatureValidation Reverts Instead of Returning Failure

**Severity:** Medium   

**Affected Contract(s):**
- `ERC4337v07/BaseAuth`

**Affected Function(s):**
- `validateUserOp()`

**Description:**

The internal signatureValidation function in BaseAuth reverts on failure conditions (expired static signature, wrong caller, insufficient weight) instead of returning false. Because isValidSignature does not catch these reverts, they bubble up through validateUserOp as custom errors rather than causing validateUserOp to return SIG_VALIDATION_FAILED as intended.

---

### Static Signature Validation Reverts Cause DoS

**Severity:** Medium  

**Affected Contract(s):**
- `BaseAuth`

**Affected Function(s):**
- `signatureValidation()`

**Description:**

The static‐signature branch in signatureValidation uses revert for expired signatures (InvalidStaticSignatureExpired) or wrong caller (InvalidStaticSignatureWrongCaller) instead of returning a failure code. Since isValidSignature does not catch these reverts, they propagate through the ERC-1271 interface, causing the entire transaction to revert rather than returning the expected 0x00000000 on failure.

---

### Unhandled behaviorOnError Value Leads to False Success

**Severity:** Medium  

**Affected Contract(s):**
- `Calls`

**Affected Function(s):**
- `_execute()`

**Description:**

The `_execute` function reads `behaviorOnError` from user-supplied packed calldata as `(flags & 0xC0) >> 6`, yielding integer values 0 through 3. It explicitly handles values 0 (IGNORE_ERROR), 1 (REVERT_ON_ERROR), and 2 (ABORT_ON_ERROR). However, if `behaviorOnError` equals 3 (due to maliciously crafted flags), the function’s `if` blocks for error handling are all skipped and execution falls through to `emit CallSucceeded`, incorrectly treating a failed external call as successful.

---

### Unbounded Growth of queuedPayloadHashes

**Severity:** Medium  

**Affected Contract(s):**
- `Recovery`

**Affected Function(s):**
- `queuePayload()`

**Description:**

The queuePayload function unconditionally appends each new payloadHash to the queuedPayloadHashes mapping without ever removing, expiring, or limiting entries. Since no other function in the Recovery contract prunes or caps this array, it can grow indefinitely as more payloads are queued.

---

### Missing Root Initialization Check in Branch Case

**Severity:** Medium  

**Affected Contract(s):**
- `Recovery`

**Affected Function(s):**
- `_recoverBranch()`

**Description:**

The function mistakenly hashes a zero root with a nested branch root when the first or a nested flag is FLAG_BRANCH. Unlike FLAG_RECOVERY_LEAF and FLAG_NODE, which use the child node directly when the accumulated root is zero, FLAG_BRANCH always computes root = fkeccak256(root, nroot). An attacker can therefore craft or break proofs by beginning with FLAG_BRANCH, causing valid proofs to fail or malicious proofs to collide.

---

### Incorrect Root Calculation for Initial Nested Branch Proof

**Severity:** Medium  

**Affected Contract(s):**
- `Recovery`

**Affected Function(s):**
- `_recoverBranch()`

**Description:**

Within the FLAG_BRANCH case, the code always computes `root = keccak256(root, subRoot)` without checking whether `root` is still its zero value. When a signature’s very first element is a nested branch proof, this causes the calculated root to become `keccak256(0x00…, subRoot)` instead of simply `subRoot`, resulting in an incorrect aggregated Merkle root.

---

### LibBytes Out-of-Bounds Calldata Read

**Severity:** Medium  

**Affected Contract(s):**
- `LibBytes`

**Affected Function(s):**
- `readUintX (and related helpers)()`

**Description:**

The LibBytes library’s low-level parsing helpers—readUintX, readUint24, readAddress, and readFirstUint8—use EVM calldataload with an index and length derived directly from untrusted input. They do not perform any explicit bounds checks against the calldata length. When a malicious caller supplies an out-of-range offset or length, calldataload pads the result with zeros rather than reverting. As a result, these functions can silently return zeroes or arbitrary data, misleading downstream logic that expects valid values.

---

### Missing Support for 65-Byte Signatures in isValidSignature

**Severity:** Medium  

**Affected Contract(s):**
- `Recovery`

**Affected Function(s):**
- `isValidSignature()`

**Description:**

The function only handles 64-byte “compact” (EIP-2098) signatures and never processes the 65-byte (r, s, v) format. As a result, any valid 65-byte signature from an EOA (_signer.code.length == 0) will fall through and return false, causing legitimate signatures to be rejected.


---

### Mutating payload causes static signature verification to always fail

**Severity:** Medium   

**Affected Contract(s):**
- `BaseAuth`

**Affected Function(s):**
- `recoverSapientSignature()`

**Description:**

recoverSapientSignature unconditionally appends msg.sender to the payload.parentWallets array before calling signatureValidation. The static-signature branch in signatureValidation computes an EIP-712 hash (opHash) over the mutated payload, but the on-chain static signature was generated over the original payload. This hash mismatch causes _getStaticSignature(opHash) to never find the stored signature, triggering a revert in all static-signature cases.

