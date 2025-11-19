## Verified Patterns Found: 8

## Verified Patterns Found in following Categories:

- StateGrowthOrStorageBloat
- AccountingInvariantViolation
- StandardViolation
- GriefableCallbacks
- Reentrancy



## Summary of Patterns

Invalid signatures can revert ERC1271 and ERC4337 flows instead of returning failure codes

Checkpointer callback can grief all signatures using that configuration

selfExecute can be re‑entered via internal/self calls despite nonReentrant guard

ERC1271 isValidSignature reverts instead of returning 0 on invalid signatures

Undefined behaviorOnError=3 lets failed calls be reported as succeeded

Reverts in ERC-1271 path can break ERC-4337 validateUserOp semantics

Recovery module unbounded queuedPayloadHashes array causes storage bloat

Unrecognized behaviorOnError value causes failed calls to be emitted as succeeded

## Patterns



 ### Issue Type: StandardViolation

 ### Relevant Function/Location: BaseAuth.signatureValidation / isValidSignature / ERC4337v07.validateUserOp

 ### Title
Invalid signatures can revert ERC1271 and ERC4337 flows instead of returning failure codes
 ### Description/Code Snippet
The wallet’s signature validation logic in `BaseAuth` is used both for ERC‑1271 (`isValidSignature`) and for ERC‑4337 account validation (`validateUserOp` via `this.isValidSignature`). In several invalid-signature scenarios, `signatureValidation` reverts with custom errors instead of returning a simple failure code.

Key code paths:

```solidity
abstract contract BaseAuth is IAuth, IPartialAuth, ISapient, IERC1271, SelfAuth {
  error InvalidSapientSignature(Payload.Decoded _payload, bytes _signature);
  error InvalidSignatureWeight(uint256 _threshold, uint256 _weight);
  error InvalidStaticSignatureExpired(bytes32 _opHash, uint256 _expires);
  error InvalidStaticSignatureWrongCaller(bytes32 _opHash, address _caller, address _expectedCaller);

  function signatureValidation(
    Payload.Decoded memory _payload,
    bytes calldata _signature
  ) internal view virtual returns (bool isValid, bytes32 opHash) {
    bytes1 signatureFlag = _signature[0];

    if (signatureFlag & 0x80 == 0x80) {
      opHash = _payload.hash();

      (address addr, uint256 timestamp) = _getStaticSignature(opHash);
      if (timestamp <= block.timestamp) {
        revert InvalidStaticSignatureExpired(opHash, timestamp);
      }

      if (addr != address(0) && addr != msg.sender) {
        revert InvalidStaticSignatureWrongCaller(opHash, msg.sender, addr);
      }

      return (true, opHash);
    }

    uint256 threshold;
    uint256 weight;
    bytes32 imageHash;

    (threshold, weight, imageHash,, opHash) = BaseSig.recover(_payload, _signature, false, address(0));

    if (weight < threshold) {
      revert InvalidSignatureWeight(threshold, weight);
    }

    isValid = _isValidImage(imageHash);
  }

  function isValidSignature(bytes32 _hash, bytes calldata _signature) external view returns (bytes4) {
    Payload.Decoded memory payload = Payload.fromDigest(_hash);
    (bool isValid,) = signatureValidation(payload, _signature);
    if (!isValid) {
      return bytes4(0);
    }
    return IERC1271_MAGIC_VALUE_HASH;
  }
}
```

For static signatures and certain threshold failures, `signatureValidation` throws:

- `InvalidStaticSignatureExpired` when timestamp has passed,
- `InvalidStaticSignatureWrongCaller` when the stored address mismatch occurs,
- `InvalidSignatureWeight` when recovered weight is below threshold.

These revert conditions are not caught in:

1. **ERC‑1271 interface** (`isValidSignature`): the spec calls for returning `0x1626ba7e` on success and `0x00000000` on failure. Here, some invalid signatures cause the function to revert rather than returning the failure magic value. Integrations that rely on ERC‑1271 semantics and expect a simple false/zero value on invalid signatures can be broken or DOSed by signatures that deliberately trigger these revert branches.

2. **ERC‑4337 validation** (`ERC4337v07.validateUserOp`):

```solidity
function validateUserOp(
  PackedUserOperation calldata userOp,
  bytes32 userOpHash,
  uint256 missingAccountFunds
) external returns (uint256 validationData) {
  ...
  if (this.isValidSignature(userOpHash, userOp.signature) != IERC1271_MAGIC_VALUE_HASH) {
    return SIG_VALIDATION_FAILED;
  }
  return 0;
}
```

Because `validateUserOp` calls `this.isValidSignature` as an external call with no `try/catch`, any revert from `signatureValidation` bubbles up and reverts the entire `validateUserOp` call instead of returning `SIG_VALIDATION_FAILED`. Under ERC‑4337, the account is expected to *signal* validation failure (e.g., wrong signature) via the returned `validationData`, not by reverting. An attacker can craft signatures that cause `InvalidStaticSignatureExpired` or `InvalidStaticSignatureWrongCaller` and thereby **DOS** simulation/validation for that account (or at least make integration brittle), since bundlers and EntryPoint expect a non-reverting response.

This behavior matches the `StandardViolation` pattern:

- **ERC‑1271 standard violation**: invalid signatures can revert instead of returning `0x00000000`, breaking composability with tools and protocols expecting pure return-based failure.
- **ERC‑4337 behavioral violation**: `validateUserOp` can revert instead of returning `SIG_VALIDATION_FAILED`, allowing malformed signatures to cause denial-of-service at the validation layer rather than being cleanly rejected.

Static signals:
- `isValidSignature` directly propagates `signatureValidation` reverts (no try/catch)
- `validateUserOp` calls `this.isValidSignature` and only checks the return value, assuming no revert
- Custom errors thrown on common invalid signature conditions instead of boolean failure

These are realistically exploitable by any external caller supplying a malicious or expired static signature.

 ### Static Signals
ERC1271 isValidSignature reverts on invalid static signatures, ERC4337 validateUserOp assumes isValidSignature never reverts, custom errors for InvalidStaticSignatureExpired / WrongCaller / InvalidSignatureWeight
 ### Assets at Risk
ERC4337 user operations (availability), integrations relying on ERC1271 signature checks
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: BaseSig.recover

 ### Title
Checkpointer callback can grief all signatures using that configuration
 ### Description/Code Snippet
In `BaseSig.recover`, when the top-level signature flag has the checkpointer bit set, the code unconditionally calls an external checkpointer contract:

```solidity
if (signatureFlag & 0x40 == 0x40 && _checkpointer == address(0)) {
  (_checkpointer, rindex) = _signature.readAddress(rindex);

  if (!_ignoreCheckpointer) {
    uint256 checkpointerDataSize;
    (checkpointerDataSize, rindex) = _signature.readUint24(rindex);

    bytes memory checkpointerData = _signature[rindex:rindex + checkpointerDataSize];

    snapshot = ICheckpointer(_checkpointer).snapshotFor(address(this), checkpointerData);

    rindex += checkpointerDataSize;
  }
}
```

This is a view call but it is an untrusted external callback:
- Any revert in `ICheckpointer.snapshotFor` will bubble up and revert the entire signature validation.
- The call forwards essentially all remaining gas and does not bound return data size, so a malicious checkpointer can consume excessive gas or return-bomb.
- Because the checkpointer address is part of the wallet configuration (`imageHash`), a misbehaving/upgraded checkpointer can permanently grief all wallets using that configuration on that chain: every attempt to validate a signature that references the checkpointer will revert before even reaching the signer-tree logic.

There is no `try/catch`, no fallback path, and no way to ignore a failing checkpointer at validation time (other than changing configuration via another chain or via some future update). This matches the `GriefableCallbacks` pattern: an external hook is required for core auth flow to proceed, and any failure in that hook bricks signature validation for the affected config.
 ### Static Signals
external call to user-configured ICheckpointer.snapshotFor without try/catch, callback success required for all signatures using checkpointer configs, forwards essentially all gas and accepts arbitrary return data
 ### Assets at Risk
wallet ability to execute any calls under affected configuration, all funds controlled by that imageHash on the chain
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: Calls.selfExecute

 ### Title
selfExecute can be re‑entered via internal/self calls despite nonReentrant guard
 ### Description/Code Snippet
The wallet’s main external entrypoints `execute` (in `Calls`) and `executeUserOp` (in `ERC4337v07`) are protected by `nonReentrant`, but the internal privileged execution path `selfExecute` is not.

Key code:

```solidity
contract Calls is ReentrancyGuard, BaseAuth, Nonce {
  function execute(bytes calldata _payload, bytes calldata _signature)
    external
    payable
    virtual
    nonReentrant
  {
    uint256 startingGas = gasleft();
    Payload.Decoded memory decoded = Payload.fromPackedCalls(_payload);

    _consumeNonce(decoded.space, decoded.nonce);
    (bool isValid, bytes32 opHash) = signatureValidation(decoded, _signature);
    if (!isValid) revert InvalidSignature(decoded, _signature);

    _execute(startingGas, opHash, decoded);
  }

  function selfExecute(bytes calldata _payload) external payable virtual onlySelf {
    uint256 startingGas = gasleft();
    Payload.Decoded memory decoded = Payload.fromPackedCalls(_payload);
    bytes32 opHash = Payload.hash(decoded);
    _execute(startingGas, opHash, decoded);
  }
}
```

`selfExecute` lacks `nonReentrant` and is `external` but guarded only by `onlySelf`. However, the wallet can call itself as an external call from within `_execute`:

- `Payload.fromPackedCalls` supports self-calls: if the per-call flag bit 0 is set, `to` is set to `address(this)`.
- During `_execute`, when such a call is processed with `delegateCall == false`, the wallet executes `LibOptim.call(call.to, call.value, ..., call.data)`, which makes an external call from the wallet to itself.
- In that self-call, `msg.sender == address(this)`, so `onlySelf` passes and `selfExecute` is reachable *inside* an already-running `_execute` frame.

Similarly, any delegate extension (`IDelegatedExtension.handleSequenceDelegateCall`) running via `delegatecall` has full wallet context and can also execute an external call back to `selfExecute` (with `msg.sender == address(this)`), again bypassing the `nonReentrant` guard.

This means:

- `_execute` can be entered recursively via `selfExecute` without tripping `ReentrancyGuard`, because `selfExecute` has no `nonReentrant` modifier.
- Nested `_execute` invocations share wallet state (nonce, hooks, storage), but maintain their own local `errorFlag`, `startingGas` and loop indices, which the code was not designed to assume can be active concurrently.

Potential consequences include:

- Complex, multi-level call graphs where a signer (or a malicious extension) causes nested batches to run during the execution of the outer batch, with non-obvious control flow.
- Invariants around fallback-only calls (`onlyFallback`) and error propagation (`behaviorOnError`) can be violated if nested batches alter state while the outer batch still expects a simple linear progression.
- Extension authors may assume that `execute`/`_execute` is non-reentrant, and therefore implement logic that is unsafe when a nested `selfExecute` executes a further batch in the middle of their own `handleSequenceDelegateCall` logic.

Although a successful exploit requires either:

- A payload already signed by the legitimate signers that includes a self-call to `selfExecute`, or
- A malicious or buggy extension that triggers `selfExecute` during delegatecall,

this pattern opens a realistic avenue for reentrancy-style bugs in extensions or future modules that assume `_execute` is single-frame. At minimum, it significantly increases the attack surface for extension authors.

Static signals:
- `selfExecute()` lacks `nonReentrant`
- External self-call path to `selfExecute` via `LibOptim.call(to=address(this))`
- Delegatecall extensions execute in wallet context and can call `selfExecute`

This matches the Reentrancy pattern: an untrusted call context (extension or self-call) can re-enter core execution logic without the intended guard.
 ### Static Signals
selfExecute external onlySelf without nonReentrant, LibOptim.call(to=address(this)) inside _execute, IDelegatedExtension.handleSequenceDelegateCall via delegatecall has wallet context
 ### Assets at Risk
wallet funds, wallet configuration state, extension-managed state
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: BaseAuth.isValidSignature

 ### Title
ERC1271 isValidSignature reverts instead of returning 0 on invalid signatures
 ### Description/Code Snippet
The wallet exposes ERC‑1271 via `BaseAuth.isValidSignature(bytes32,bytes)`, but invalid signatures frequently cause a revert instead of returning `0x00000000` as required by the standard. This can break integrations and upstream protocols that assume non-reverting ERC‑1271 contracts.

ERC‑1271 expects:
- For valid signatures: return magic value `0x1626ba7e`.
- For invalid signatures: return `0x00000000` (or any value != magic), **not** revert.

Implementation:
```solidity
function isValidSignature(bytes32 _hash, bytes calldata _signature) external view returns (bytes4) {
  Payload.Decoded memory payload = Payload.fromDigest(_hash);

  (bool isValid,) = signatureValidation(payload, _signature);
  if (!isValid) {
    return bytes4(0);
  }

  return IERC1271_MAGIC_VALUE_HASH;
}
```
`signatureValidation` is where many failure cases revert instead of returning `false`:
```solidity
function signatureValidation(Payload.Decoded memory _payload, bytes calldata _signature)
  internal
  view
  returns (bool isValid, bytes32 opHash)
{
  bytes1 signatureFlag = _signature[0];

  if (signatureFlag & 0x80 == 0x80) { // static signature path
    opHash = _payload.hash();

    (address addr, uint256 timestamp) = _getStaticSignature(opHash);
    if (timestamp <= block.timestamp) {
      revert InvalidStaticSignatureExpired(opHash, timestamp);
    }

    if (addr != address(0) && addr != msg.sender) {
      revert InvalidStaticSignatureWrongCaller(opHash, msg.sender, addr);
    }

    return (true, opHash);
  }

  uint256 threshold;
  uint256 weight;
  bytes32 imageHash;

  (threshold, weight, imageHash,, opHash) = BaseSig.recover(_payload, _signature, false, address(0));

  if (weight < threshold) {
    revert InvalidSignatureWeight(threshold, weight);
  }

  isValid = _isValidImage(imageHash); // returns false on image mismatch
}
```
Failure behaviors:
- Static signatures:
  - Expired: `revert InvalidStaticSignatureExpired`.
  - Wrong caller bound to static signature: `revert InvalidStaticSignatureWrongCaller`.
- Dynamic signatures:
  - Insufficient weight: `revert InvalidSignatureWeight`.
  - Only the final `_isValidImage(imageHash)` failure produces `isValid = false` and thus a clean ERC‑1271 return of `0x00000000`.

Thus, many invalid signatures cause the whole call to revert rather than returning `0x00000000`. This violates ERC‑1271’s expected semantics and can:
- Break upstream ERC‑4337 entrypoint implementations or other account-code that expect invalid signatures to return a failure code, not revert.
- Cause inconsistent behavior between different types of invalid signatures (some revert, some return 0), making it difficult for callers to handle errors reliably.

Because `isValidSignature` is public and callable by any contract, this is a protocol-level standard violation rather than a governance-only concern.
 ### Static Signals
ERC1271 interface implemented, signatureValidation uses revert on several invalid cases, isValidSignature does not catch or normalize reverts to return 0x00000000
 ### Assets at Risk
ERC1271-based integrations, ERC4337 validation flows relying on non-reverting isValidSignature, generic tooling and dApps that assume ERC1271 failure is signaled via return code
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Calls._execute

 ### Title
Undefined behaviorOnError=3 lets failed calls be reported as succeeded
 ### Description/Code Snippet
In the batched call executor, the 2-bit `behaviorOnError` field is decoded from user-controlled flags but only three values are defined in `Payload`: `BEHAVIOR_IGNORE_ERROR=0`, `BEHAVIOR_REVERT_ON_ERROR=1`, and `BEHAVIOR_ABORT_ON_ERROR=2`.

```solidity
// Payload.sol
_decoded.calls[i].behaviorOnError = (flags & 0xC0) >> 6; // 0..3
```

In `Calls._execute`, only these three cases are handled inside the `if (!success)` block:

```solidity
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
```

If a caller sets the top two bits of `flags` to `11` (i.e. `behaviorOnError == 3`), the external call can fail (`success == false`), all three `if` branches are skipped, and the function falls through to `emit CallSucceeded(_opHash, i);` as if the call had succeeded.

This creates a spec mismatch: a failed low-level call can be reported as successful with no `CallFailed`/`CallAborted` event and without reverting or aborting the batch. Any off-chain tooling or higher-level logic that interprets `CallSucceeded` as an indication that the sub-call actually succeeded can be misled. A malicious signer can craft such payloads so that critical operations silently fail while the wallet reports success, potentially breaking accounting or business logic in integrations that depend on accurate execution status.

The same pattern exists in helper contracts (`Estimator._estimate`, `Simulator.simulate`) where `behaviorOnError==3` also falls through unhandled, but those are out of scope here. The core vulnerability lies in `Calls._execute`, which is used by both direct `execute` and ERC-4337 `executeUserOp` flows.
 ### Static Signals
behaviorOnError decoded as (flags & 0xC0) >> 6 (0..3), no validation that behaviorOnError <= 2, missing else branch for behaviorOnError == 3, failed call can still emit CallSucceeded
 ### Assets at Risk
user wallet funds, downstream protocol state relying on successful side effects
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: ERC4337v07.validateUserOp

 ### Title
Reverts in ERC-1271 path can break ERC-4337 validateUserOp semantics
 ### Description/Code Snippet
ERC4337v07.validateUserOp relies on this.isValidSignature(userOpHash, userOp.signature) to decide whether to return 0 or SIG_VALIDATION_FAILED. However, BaseAuth.signatureValidation (used by isValidSignature) has several failure modes that REVERT instead of returning a boolean failure:

- Static signatures:
  - If the stored expiry timestamp is <= block.timestamp it reverts with InvalidStaticSignatureExpired.
  - If the stored "allowed caller" is non-zero and != msg.sender it reverts with InvalidStaticSignatureWrongCaller.
- Dynamic signatures:
  - If recovered weight < threshold it reverts with InvalidSignatureWeight.
  - BaseSig.recover can also revert (e.g., InvalidERC1271Signature, InvalidSignatureFlag, WrongChainedCheckpointOrder, UnusedSnapshot).

validateUserOp does:

```solidity
function validateUserOp(
  PackedUserOperation calldata userOp,
  bytes32 userOpHash,
  uint256 missingAccountFunds
) external returns (uint256 validationData) {
  ...
  if (this.isValidSignature(userOpHash, userOp.signature) != IERC1271_MAGIC_VALUE_HASH) {
    return SIG_VALIDATION_FAILED;
  }
  return 0;
}
```

There is no try/catch around the isValidSignature call. Any revert inside BaseAuth / BaseSig bubbles up and causes validateUserOp itself to revert. Under ERC-4337, validateUserOp is expected to return a bit-packed validationData (or SIG_VALIDATION_FAILED) to signal signature failure, not revert for normal invalid-signature conditions.

Implications:
- A user (or attacker) can supply a signature that intentionally triggers one of the revert paths (e.g., using an expired static signature). The EntryPoint’s simulateValidation / handleOps call will then revert instead of getting SIG_VALIDATION_FAILED, potentially griefing bundlers or preventing inclusion of otherwise valid ops in the same batch.
- This is a protocol-level standard deviation: both ERC-4337 (validateUserOp return semantics) and ERC-1271 (invalid signature should normally return 0x00000000 instead of reverting) are not strictly followed, which can break integrations and cause DoS on accounts using these wallets with 4337.

This matches the StandardViolation pattern: a standard-required interface (ERC-4337 IAccount / ERC-1271) is implemented in a way that can revert instead of returning the documented failure codes, enabling denial-of-service or broken integrations.
 ### Static Signals
validateUserOp relies on isValidSignature return without try/catch, BaseAuth.signatureValidation uses revert for invalid static signatures, BaseSig.recover can revert for multiple malformed or stale signature conditions, ERC-4337 expects validateUserOp to return error code, not revert, on invalid signatures
 ### Assets at Risk
user operations, wallet availability, bundler/entrypoint integration
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StateGrowthOrStorageBloat

 ### Relevant Function/Location: Recovery.queuePayload

 ### Title
Recovery module unbounded queuedPayloadHashes array causes storage bloat
 ### Description/Code Snippet
In the Recovery sapient signer, each queued payload hash is pushed into an ever-growing array without any pruning or bounds, leading to unbounded state growth and increasing gas costs over time.

Relevant code:

```solidity
contract Recovery is ISapientCompact {
  /// @notice Mapping of queued timestamps
  /// @dev wallet -> signer -> payloadHash -> timestamp
  mapping(address => mapping(address => mapping(bytes32 => uint256))) public timestampForQueuedPayload;

  /// @notice Mapping of queued payload hashes
  /// @dev wallet -> signer -> payloadHash[]
  mapping(address => mapping(address => bytes32[])) public queuedPayloadHashes;

  /// @notice Get the total number of queued payloads
  function totalQueuedPayloads(address _wallet, address _signer) public view returns (uint256) {
    return queuedPayloadHashes[_wallet][_signer].length;
  }

  /// @notice Queue a payload for recovery
  function queuePayload(
    address _wallet,
    address _signer,
    Payload.Decoded calldata _payload,
    bytes calldata _signature
  ) external {
    if (!isValidSignature(_wallet, _signer, _payload, _signature)) {
      revert InvalidSignature(_wallet, _signer, _payload, _signature);
    }

    bytes32 payloadHash = Payload.hashFor(_payload, _wallet);
    if (timestampForQueuedPayload[_wallet][_signer][payloadHash] != 0) {
      revert AlreadyQueued(_wallet, _signer, payloadHash);
    }

    timestampForQueuedPayload[_wallet][_signer][payloadHash] = block.timestamp;
    queuedPayloadHashes[_wallet][_signer].push(payloadHash);

    emit NewQueuedPayload(_wallet, _signer, payloadHash, block.timestamp);
  }
```

The `queuedPayloadHashes[_wallet][_signer]` array is append-only: entries are never removed, expired, or compacted. Over the lifetime of a wallet, repeated use of recovery (or griefing via many distinct payloads) will cause these arrays to grow without bound. Although `totalQueuedPayloads` is O(1), future extensions or off-chain tooling may need to iterate over these arrays, and the persistent storage growth directly increases gas costs for any interaction involving this contract's storage (e.g., SSTORE refunds reduced, higher cold/warm access overhead). This matches the StateGrowthOrStorageBloat pattern: append-only arrays without pruning lead to ever-increasing state and potential gas-based DoS as the contract ages.

 ### Static Signals
append-only array: queuedPayloadHashes[_wallet][_signer].push(payloadHash), no function to remove or prune queued payloads, mapping-of-array used as history without bounds
 ### Assets at Risk
user gas costs, long-term usability of recovery module
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Calls._execute

 ### Title
Unrecognized behaviorOnError value causes failed calls to be emitted as succeeded
 ### Description/Code Snippet
In the Calls._execute interpreter, the per-call error handling is driven by the 2-bit behaviorOnError field decoded from the call flags in Payload.fromPackedCalls:

```solidity
// Payload.fromPackedCalls
_decoded.calls[i].behaviorOnError = (flags & 0xC0) >> 6; // values 0..3
```

The protocol defines three behaviors:
- 0 = BEHAVIOR_IGNORE_ERROR
- 1 = BEHAVIOR_REVERT_ON_ERROR
- 2 = BEHAVIOR_ABORT_ON_ERROR

However, the 2-bit slice can also take the value 3 (binary 11) if the caller sets both high bits in the flags byte. In Calls._execute, only the cases 0, 1, and 2 are handled explicitly:

```solidity
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
```

When behaviorOnError == 3 and the external call fails (success == false), none of the three if-branches match. The code falls through the error handling block without setting errorFlag or emitting CallFailed/CallAborted/Reverted, and then unconditionally executes:

```solidity
emit CallSucceeded(_opHash, i);
```

This leads to:
- A failed external call being recorded on-chain as a CallSucceeded event.
- errorFlag remaining false, so a subsequent onlyFallback call will be skipped as if no error occurred.

This violates the natural accounting invariant that the emitted status events should reflect the actual outcome of each call, and it may confuse off-chain tooling or dApps that rely on CallSucceeded/CallFailed/CallAborted to reconstruct wallet behavior. It also effectively introduces an undocumented fourth error behavior where failures are silently treated as successes.

An attacker who can craft the raw packed payload (or a buggy SDK) could set the high two bits of the call flags to 11, yielding behaviorOnError == 3, and cause failed calls to appear as succeeded in logs, and skip error-driven fallback calls. This is a protocol-level semantic discrepancy that should be normalized (e.g., by rejecting behaviorOnError == 3 or mapping it to a defined behavior) to preserve the intended invariants.
 ### Static Signals
behaviorOnError decoded from (flags & 0xC0) >> 6 allowing value 3, no handling branch for behaviorOnError == 3, failed call falls through to emit CallSucceeded
 ### Assets at Risk
user funds, downstream protocol interactions, off-chain accounting/monitoring based on events
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole

