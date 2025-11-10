
## PROTOCOL OVERVIEW:

## Sequence Smart Wallet Protocol – Detailed Technical Overview

### Introduction
Sequence v3 is a **modular, fully-on-chain smart-contract wallet** designed around three core ideas:

1.  Image-hash based configurations (Merkkle-root of a sparse tree) that capture *everything* required to authorise the wallet: signers + weights, thresholds, checkpoints, extensions, pre-authorised digests, checkpointer address, etc.
2.  Flexible, gas-efficient **signature encoding** able to express many signer types (EOA, ERC-1271 contracts, Sapient extensions, nested multisigs) *and* chain configuration updates together in a single “state-channel-like” proof – the **chained signature**.
3.  An execution engine that can interpret **compact, binary-encoded payloads** (transactions / messages / config updates / digests) and run them through a battle-tested permission system, ERC-4337 account-abstraction entrypoints, hook delegates, smart-sessions, passkeys, recovery, etc.

Everything lives in a single Wallet proxy that self-upgrades from Stage 1 (counter-factual deployment) to Stage 2 (live wallet). Ownership always sits with the wallet’s configuration; there is **no privileged admin key** in the contracts.

---

## 1.  Life-cycle & Deployment

1. **Factory.deploy(mainModule, salt)** – uses CREATE2 where `salt == imageHash`.  The deterministic address is therefore *bound* to the configuration root.
2. **Stage1Module (initial implementation)** validates that the wallet’s address is indeed derived from the claimed `imageHash`.  Once a first valid signature is presented, Stage 1 stores the hash and **self-upgrades** the proxy to Stage2Module.
3. **Stage2Module** enables the full feature-set: Calls batching, ERC-4337 v0.7, ReentrancyGuard, Hooks, Sessions, Passkeys, Recovery, etc.
4.  The wallet can then be driven either directly (execute) or through the EntryPoint (validateUserOp + executeUserOp).

---

## 2.  Configuration Tree ( `imageHash` )

A sparse Merkle tree encodes:

* `checkpointer` (20 bytes) – optional.
* `checkpoint` (uint) – monotonic version counter.
* `threshold` – how many *weight* points are required for a signature.
* Any number of leaves:
  * **Signer leaf** – address & weight.
  * **Nested config** – inner root + internal threshold + external weight (N-of-M subgroup).
  * **Sapient signer** – contract address, weight, and that contract’s *own* root.
  * **Hard-coded subdigest** – pre-authorised payload hash (or “any address” variant for counter-factual approval).

Because the root is all that is stored on-chain, every signature must supply the **proof** for the leaves it touches so the wallet can recompute the root.

### Checkpoint rules

* Each config carries a `checkpoint` integer.
* Chained signatures **must** go strictly upwards: later chunks must have a strictly larger checkpoint so old channel segments cannot be replayed.
* A *checkpointer* contract may publish the “latest known good” `{imageHash, checkpoint}` snapshot for multi-chain wallets.  Wallet logic enforces that signatures are either ahead of, or pass exactly through, that snapshot.

---

## 3.  Signature Encoding

```
[globalFlag][opt checkpointer addr + data][opt checkpoint][opt threshold][branch bytes]
```

Bit-layout of `globalFlag` (LSB-first):

* bit 0 – `IS_CHAINED` (1 = the following bytes are a chain of signatures)
* bit 1 – `NO_CHAIN_ID` (signs over a 0 chainId domain)
* bits 2-4 – `CHECKPOINT_SIZE` (0-7 → number of bytes that encode checkpoint)
* bit 5 – `THRESHOLD_IS_2_BYTES`
* bit 6 – `HAS_CHECKPOINTER`
* bit 7 – `STATIC_SIGNATURE` (pre-stored in wallet storage, zero-gas path)

### Branch format (parsed iteratively)

Each item starts with one byte whose **top nibble** is the *flag*, bottom nibble provides item-specific data (weight, length size &c).

Flag definitions (selected):

0.  `FLAG_SIGNATURE_HASH` – EIP-2098 compact ECDSA over `_opHash`
1.  `FLAG_ADDRESS` – counts weight without signature (used inside nested groups)
2.  `FLAG_SIGNATURE_ERC1271`
4.  `FLAG_BRANCH` – nested branch with its own byte length field
5.  `FLAG_SUBDIGEST` – hard-coded digest leaf (infinite weight if matches)
6.  `FLAG_NESTED` – nested configuration leaf
7.  `FLAG_SIGNATURE_ETH_SIGN` – eth-sign prefix
8.  `FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST` – counter-factual variant
9.  `FLAG_SIGNATURE_SAPIENT` – calls `ISapient.recoverSapientSignature`
10. `FLAG_SIGNATURE_SAPIENT_COMPACT` – compact version passing only digest

### Chained signatures

If `IS_CHAINED` is set the byte-stream turns into:

```
[3-byte len][sig1][3-byte len][sig2]…[3-byte len][sigN]
```

* `sig1` signs the **payload**.
* `sig2` signs an *image-hash + KIND_CONFIG_UPDATE* payload, authorising `sig1`’s configuration, and so on…
* The final chunk must recover to the wallet’s on-chain configuration.

This lets users stage multiple configuration updates off-chain and push them atomically only when they need to spend.

---

## 4.  Payload System

Four kinds:

| Kind | Purpose |
|------|---------|
| 0x00 | Transactions (batched calls) |
| 0x01 | Message (arbitrary bytes) |
| 0x02 | Config update (new `imageHash`) |
| 0x03 | Digest (pre-hashed message for ERC-1271) |

Transactions are compact-encoded for gas:

```
[globalFlag][opt space][opt nonce][opt callCount][call…]
call := [callFlags][opt addr][opt value][opt dataSize+data][opt gasLimit]
```

Per-call flags embed *delegateCall*, *onlyFallback*, *behaviorOnError*, etc.  This reduces calldata by >70 % compared to naïve ABI.

The library returns a `Decoded` struct used everywhere:

```solidity
struct Decoded {
    uint8 kind;
    bool noChainId;
    Call[] calls;
    uint256 space;
    uint256 nonce;
    bytes message;
    bytes32 imageHash;
    bytes32 digest;
    address[] parentWallets; // recursion for sapients
}
```

---

## 5.  Execution Flow (Calls.sol)

1.  **decode payload** → `Decoded`
2.  **_consumeNonce** (replay protection)
3.  **signatureValidation** via BaseAuth → ensures `weight ≥ threshold` and `imageHash` = wallet storage.
4.  Loop over calls:
   * choose `to` (self vs external), compute gas limit.
   * `delegatecall` goes through IDelegatedExtension interface so extensions can differentiate context.
   * on failure apply `behaviorOnError`: IGNORE, REVERT_ALL, or ABORT_REST.
   * emit events per outcome.

ReentrancyGuard protects the whole `execute` entry.

---

## 6.  Extensions

### Hooks

Map `bytes4 selector → implementation`.  Fallback delegatecalls to that address, giving the wallet infinite extensibility while keeping storage unified.

### ERC-4337 (module `ERC4337v07.sol`)

Implements `validateUserOp` + `executeUserOp` restricted to a single immutable EntryPoint.  Signature checking simply forwards to wallet’s `isValidSignature` (
`IERC1271`).  Missing funds are auto-deposited.

### Smart Sessions

Two modes:

* **Explicit sessions** – a special sapient signer whose configuration is embedded inside the signature itself.  Permissions are explicit leaves (target → rules[], valueLimit, deadline…); validation happens in `ExplicitSessionManager` and cumulative limits can be persisted on-chain.
* **Implicit sessions** – off-chain attestation signed by an **identity signer** authorises a session signer.  Every call must carry an attestation index.  Additional safety: mandatory sorted blacklist, no value, no delegatecall, and the *target contract* must return a magic value from `acceptImplicitRequest`.

Session signatures are decoded by `SessionSig` then re-validated by `SessionManager` which enforces limits, blacklist and per-call rules.

### Passkeys (WebAuthn)

`Passkeys.sol` implements an `ISapientCompact` signer.  Signatures include authenticatorData / clientDataJSON indexes, R + S, P-256 public key, and optional metadata.  Verification is done with `WebAuthn.verify` + P-256 precompile (RIP-7212).  The resulting imageHash leaf is deterministic on (x, y, requireUV, metadata).

### Recovery

`Recovery.sol` is a sapient signer that introduces a **time-delay queue**:

* Any signer may `queuePayload(wallet, payloadHash)` with their signature.
* A merkle leaf encodes `minDelay` & `minTimestamp`.
* Later, during `recoverSapientSignatureCompact`, the Recovery contract walks the merkle proof and checks that `block.timestamp ≥ queuedTimestamp + minDelay` before considering weight.

This provides a social-recovery escape hatch without new on-chain configuration.

---

## 7.  Security Model

* **Threshold / weight** – Every branch returns a total weight; if below threshold, signature is rejected.
* **Checkpoint monotonicity** – prevents “state channel” replay of past configurations.
* **Checkpointer** – optional oracle that rejects signatures older than published snapshot, protecting inactive chains.
* **Nonce spaces** – independent replay-protection domains so sessions can run in their own counters.
* **Gas limits per call** – prevents griefing by burning unlimited gas.
* **ReentrancyGuard & onlySelf** – internal mutations require delegate-call from wallet itself.
* **Static signatures** – explicitly bound to `caller` (msg.sender) and `expires`, so leaked calldata cannot be reused.
* **Blacklist & target-side acceptance (implicit sessions)** – blocks phishing targets and enforces app-level approval.
* **Low-s normalisation (P-256) and `s ≤ n/2` check** – mitigate malleability.

---

## 8.  Gas & Storage Footprint

* Sparse tree + selective proof → minimal calldata.
* Flag-based payload encoding → ~3 bytes overhead per call vs 68 bytes in ABI.
* ERC-2098 compact ECDSA (64 bytes) used everywhere.
* Only one storage write per successful execute: nonce++ (plus optional limit usage writes).
* Reentrancy-guard status stored at deterministic slot for EIP-2200 refund.

---

## 9.  Upgrade & Extensibility Strategy

* Wallet is a **self-upgradable proxy** (`Implementation.sol`) – only the wallet (i.e.
 a signed config update) can change its implementation address.
* Hook table lets users map arbitrary selectors to delegate-extensions without upgrade.
* New signer types or leaf kinds can be added by allocating a new `FLAG_*` value (>10) – the BaseSig parser will need a new case but existing signatures remain valid.
* Payload format is versionless; new per-call flags can use reserved bits.
* Sessions configs are tag-tree based – unknown tags can be ignored / pre-hashed to stay forward-compatible.

---

## 10.  Integration Guidelines

1.  To **deploy** a counter-factual wallet:
    * Build configuration tree → get `imageHash`.
    * `Factory.deploy(mainModule, imageHash)` using the Stage1Module bytecode as `mainModule`.
2.  To **send a tx**:
    * Encode calls with `Payload.fromPackedCalls`.
    * Prepare signature (single or chained) following rules; include checkpointer data if wallet uses one.
    * `wallet.execute(payloadBytes, signature)` **or** craft an ERC-4337 `UserOperation` where `callData = wallet.executeUserOp(payloadBytes)`.
3.  To **update signer set**:
    * Build new tree with higher `checkpoint`.
    * Create a `KIND_CONFIG_UPDATE` payload.
    * Authorise with current signers; optionally chain the signature with a spend payload so update & spend happen atomically.
4.  To **use smart sessions**:
    * For explicit – embed session config + per-call signatures via SessionSig library.
    * For implicit – obtain an attestation signed by identity signer, encode calls with flag byte MSB =1 referencing that attestation index.

---

## Conclusion

Sequence v3 combines **Merkle-root configurations**, **state-channel-style chained signatures**, **compact payload encoding**, and a rich catalogue of extensions (sessions, passkeys, recovery, ERC-4337, hooks) to deliver a **truly programmable, multi-sig smart-wallet** that stays gas-competitive and upgrade-ready.  The protocol enforces strict replay & configuration ordering guarantees while remaining flexible enough to integrate new signer modalities or permission systems without redeployment.  All critical paths are permission-less and self-custodial—only the signers defined in the current `imageHash` control the wallet.



## Main List of Files in Project

src/Factory.sol
src/Guest.sol
src/Stage1Module.sol
src/Stage2Module.sol
src/Wallet.sol
src/extensions/passkeys/Passkeys.sol
src/extensions/recovery/Recovery.sol
src/extensions/sessions/SessionErrors.sol
src/extensions/sessions/SessionManager.sol
src/extensions/sessions/SessionSig.sol
src/extensions/sessions/explicit/ExplicitSessionManager.sol
src/extensions/sessions/explicit/IExplicitSessionManager.sol
src/extensions/sessions/explicit/Permission.sol
src/extensions/sessions/explicit/PermissionValidator.sol
src/extensions/sessions/implicit/Attestation.sol
src/extensions/sessions/implicit/ISignalsImplicitMode.sol
src/extensions/sessions/implicit/ImplicitSessionManager.sol
src/modules/Calls.sol
src/modules/ERC4337v07.sol
src/modules/Hooks.sol
src/modules/Implementation.sol
src/modules/Nonce.sol
src/modules/Payload.sol
src/modules/ReentrancyGuard.sol
src/modules/Storage.sol
src/modules/auth/BaseAuth.sol
src/modules/auth/BaseSig.sol
src/modules/auth/SelfAuth.sol
src/modules/auth/Stage1Auth.sol
src/modules/auth/Stage2Auth.sol
src/modules/interfaces/IAccount.sol
src/modules/interfaces/IAuth.sol
src/modules/interfaces/ICheckpointer.sol
src/modules/interfaces/IDelegatedExtension.sol
src/modules/interfaces/IERC1155Receiver.sol
src/modules/interfaces/IERC1271.sol
src/modules/interfaces/IERC223Receiver.sol
src/modules/interfaces/IERC721Receiver.sol
src/modules/interfaces/IERC777Receiver.sol
src/modules/interfaces/IEntryPoint.sol
src/modules/interfaces/IPartialAuth.sol
src/modules/interfaces/ISapient.sol
src/utils/Base64.sol
src/utils/LibBytes.sol
src/utils/LibOptim.sol
src/utils/P256.sol
src/utils/WebAuthn.sol


 ## DOCUMENTATION: 

 ### sequence-docs.md

# Sequence Chained Signatures Documentation

This document provides an overview of the rationale behind the “chained” signatures of the Sequence wallet contracts.

## **1. Overview**

Sequence uses chained signatures as a mechanism to perform “configuration updates”, in such a way that configuration updates are both: 1. Valid on all networks 2. Zero cost until utilized

In this way, “chained signatures” transform the representation of a configuration in Sequence v3 from a simple static element to a state channel.

A configuration can sign a “configuration update”, which acts as an authorization to another configuration to fully act on behalf of the wallet. These authorized configurations can in turn perform additional authorizations, forming a chain of N configurations.

Configurations are strictly ordered by their “checkpoint”. A configuration cannot “delegate” into a configuration with a checkpoint that is below or equal to the current configuration. This ensures that no old sections of the state channel can be resurfaced.

```
Wallet lifecycle
───────────────────────────────────────────────────────────────▷
┌──────────┐      ┌──────────┐      ┌──────────┐      ┌─────────┐
│ Config 1 ├──┬──▶│ Config 2 ├──┬──▶│ Config 3 ├──┬──▶│ Payload │
└──────────┘  │   └──────────┘  │   └──────────┘  │   └─────────┘
          Authorize         Authorize           Sign
```

## **2. Format**

Chained signatures are encoded as a list of signatures, in reverse order. This is done to allow for the contract to recover from the payload to the configuration that is currently defined by the contract. Intermediary configurations are not directly encoded, but rather recovered from the previous signature, until the last configuration which is recovered and validated against the contract’s current configuration.

All chained signatures start by setting the global signature flag to `XXXX XXX1` (`flag & 0x01`), only the 8th bit is read, every other bit is ignored.

Afterwards, each signature part is encoded, prefixed by 3 bytes that determine their size.

```
 ┌───▶ Global flag for chained     ┌─▶ Intermediary
 │     XXXX XXX1                   │   signature
 │     Signals we are reading      │   repeats N times
 │     a chained signature         │
 │                                 │
 ○                      ───────────┴────────
01 0001ff 5151515151... 000803 5252525252... 00cb55 5353535353...
   ──┬─── ──┬──────────  Size    Signature   ──┬─────────────────
     │      │                                  │
     │      │   Dynamic length                 │   Final signature
     │      │   first signature                │   should recover
     │      └─▶ (for Payload)                  └─▶ to configuration
     │                                             defined by contract
     │    3 bytes:
     └──▶ Next signature
          part size
```

## **2. Rules**

Chained signatures MUST comply with the following rules to be considered valid.

### Weight > Threshold

All signature parts must meet their respective thresholds.

### Payload order

1. The first signature recovers the given Payload. If its threshold is met, then it generates an `imageHash` which corresponds to the root of a Merkle tree that defines the configuration of the wallet.

2. Following signatures recover the `imageHash` of the previous signature, bundled within a payload of the `KIND_CONFIG_UPDATE` kind. They, in turn, recover to their own next `imageHash`.

3. After the last signature has been recovered, the resulting `imageHash` must match the one defined by the contract.

```
┌──────────────┐        ┌─────────────────────┐        ┌─────────────────────┐
│Main payload  │        │Config update payload│        │Config update payload│
│(e.g. send tx)│        │ImageHash 1          │        │ImageHash 2          │
└─────────────┬┘        └─▲──────────────────┬┘        └─▲──────────────────┬┘
              │           │                  │           │                  │
    Recover ──┤           │        Recover ──┤           │        Recover ──┤
              │           │                  │           │                  │
   ┌──────────▼┐ Generate │       ┌──────────▼┐ Generate │       ┌──────────▼┐
   │ImageHash 1├──────────┘       │ImageHash 2├──────────┘       │ImageHash 3│
   └───────────┘                  └───────────┘                  └─────┬─────┘
                                                                       │
                                                                       │
                                                                       │
                                                                 ╔═════▼═════╗
                                                                 ║ Is valid? ║
                                                                 ╚═══════════╝
```

### Checkpoint

Each wallet configuration defines a checkpoint, checkpoints define a strict order in which these configurations can be used **in the context of a chained signature**, this acts as a form of replay protection that blocks the usage of a previous section of the state channel **in case the state channel ever repeats configurations**.

For example, a wallet has a configuration with signers `A & B`, then it is updated to `A, B & C` and then updated back again to `A & B`. Afterwards, another update is queued, this time to `A, B & D`. Without the checkpoints, a malicious actor (C) may exploit the first update (where `C` was added) even if the wallet already evicted `C` as a signer.

```
                      Checkpoint 2 -> 0 forbidden
              ┌─────────────────────────────────────────────┐
              │                                             │
              x                                             │
┌───────────────┐     ┌─────────────────┐     ┌─────────────┴─┐     ┌─────────────────┐
│Signers: A & B │     │Signers: A, B & C│     │Signers: A & B │     │Signers: A, B & D│
│Checkpoint: 0  ├────▶│Checkpoint: 1    ├────▶│Checkpoint: 2  ├────▶│Checkpoint: 3    │
└───────────────┘     └─────────────────┘     └───────────────┘     └─────────────────┘
```

## **3. Checkpointer**

The checkpointer is an optional interface that helps wallets keep track of the latest valid configuration without publishing every intermediate step to all chains.

Because wallets exist on multiple chains, unused networks often lag behind and accumulate old states. The checkpointer solves this by announcing the last known valid configuration. Anything older is automatically considered invalid.

Wallet contracts don't specify how the checkpointer works internally. Its only job is to clearly state the latest valid configuration to prevent older states from being reused.

### Disabled checkpointer

Even when a checkpointer is defined by the configuration, the checkpointer has the capability to "disable" itself. This is useful for implementing escape hatches like timelocks or challenge periods, in case that the checkpointer becomes unresponsive or malicious. When the checkpointer announces a snapshot with `imageHash == bytes32(0)` it is considered disabled.

Notice that there is **no built in mechanism** to force the checkpointer to become disabled from within the wallet, any mechanism that may challenge and disable the checkpointer must be implemented on the checkpointer contract.

### Checkpointer data

When a checkpointer is defined by the configuration, obtaining the latest snapshot from it requires passing a generic `data` field to the checkpointer contract. This `data` field is provided within the signature, and it is entirely opaque to the wallet contract.

It wouldn't make sense for the checkpointer to be able to provide the snapshot directly from contract storage, as this would move the requirement of having to settle the state channel from the wallet contract to the checkpointer contract, instead, it is expected that the checkpointer will use the `data` field to validate a proof of the snapshot.

The proof may be implemented as a simple merkle proof, a signature from a trusted party or T.E.E., a zk proof from a keystore rollup, or any other mechanism that may allow the checkpointer to provide a valid snapshot.

### Checkpointer scenarios

Different scenarios exist depending on the current state of the wallet, state channel and checkpointer, and the relationship between them.

#### 1. Full sync

This happens when the wallet contract configuration is set to the latest configuration defined by the state channel, and the checkpointer reports the same configuration.

This scenario does not require the usage of chained signatures, as regular signatures are sufficient when the configuration that is being used matches the one defined by the contract.

```
  Checkpointer
 ┌───────────────────────────────────────────────┐
 │                        Wallet                 │
 │                       ┌─────────────────────┐ │
 │ ┌──────────────┐      │  ┌──────────────┐   │ │
 │ │ Config A & B ├──────┼──▶ Config A & C │   │ │
 │ └──────┬───────┘      │  └──────────────┘   │ │
 │        │              └─────────────────────┘ │
 └────────┼──────────────────────────────────────┘
          │
          ▼
Old state - Irrelevant
```

> Notice that even if no chained signatures are used, the checkpointer is still required to provide a valid snapshot of the latest configuration.

#### 2. Checkpointer Ahead, Wallet Behind

The state channel is ahead of the wallet contract, but the checkpointer knows about the latest configuration. In this scenario, the checkpointer provides a snapshot for the latest configuration, and the wallet **must** use a chained signature to sign the payload, using that latest configuration.

```
  Checkpointer
 ┌────────────────────────────────────────────────────────────────────┐
 │                        Wallet                                      │
 │                       ┌─────────────────────┐                      │
 │ ┌──────────────┐      │  ┌──────────────┐   │     ┌──────────────┐ │
 │ │ Config A & B ├──────┼──▶ Config A & C ├───┼─────▶ Config D & C │ │
 │ └──────┬───────┘      │  └──────────────┘   │     └───────┬──────┘ │
 │        │              └─────────────────────┘             │        │
 └────────┼──────────────────────────────────────────────────┼────────┘
          │                                                  │
          ▼                                                  ▼
Old state - Irrelevant                                  Latest state
```

#### 3. Checkpointer Behind, Wallet Synced

The checkpointer is behind the wallet contract, and there are no pending configurations in the state channel. In this scenario, the checkpointer will provide a snapshot with a checkpoint that is behind the wallet contract, and this will make the wallet contract ignore the checkpointer's snapshot.

This scenario may happen if the wallet is manually updated to a new configuration without notifying the checkpointer, or if the checkpointer hasn't had time to "react" and "catch up" with the wallet contract. It may be possible that the checkpointer is offline, and not reacting to any configuration changes.

```
  Checkpointer            Wallet
 ┌──────────────────┐    ┌─────────────────────┐
 │ ┌──────────────┐ │    │  ┌──────────────┐   │
 │ │ Config A & B ├─┼────┼──▶ Config A & C │   │
 │ └──────┬───────┘ │    │  └──────────────┘   │
 └────────┼─────────┘    └─────────────────────┘
          │
          │
          ▼
Old state - Irrelevant
```

#### 4. Wallet Behind, Checkpointer Further Behind

The checkpointer is behind the wallet contract, and there are pending configurations in the state channel. This scenario is very similar to the previous one, the only difference is that the wallet will use a chained signature to sign the payload, using the latest configuration.

Even if the wallet uses a chained signature, since the checkpointer's snapshot checkpoint is behind the wallet contract, the wallet will not use the checkpointer's snapshot.

```
  Checkpointer            Wallet
 ┌──────────────────┐    ┌─────────────────────┐
 │ ┌──────────────┐ │    │  ┌──────────────┐   │     ┌──────────────┐
 │ │ Config A & B ├─┼────┼──▶ Config A & C ├───┼─────▶ Config B & C │
 │ └──────┬───────┘ │    │  └──────────────┘   │     └──────┬───────┘
 └────────┼─────────┘    └─────────────────────┘            │
          │                                                 │
          │                                                 │
          ▼                                                 ▼
Old state - Irrelevant                                 Latest state
```

#### 5. Checkpointer Ahead, Wallet Behind, State Channel Further Ahead

The state channel has multiple configuration updates, the checkpointer is ahead of the wallet contract, yet the state channel goes further than the checkpointer. This scenario may happen if the latest configuration update happened recently and the checkpointer hasn't updated yet.

In this scenario, the checkpointer provides a snapshot, the wallet contract enforces a chained signature to sign the payload, forcing that the chained signature, at some point, goes over the checkpointer's snapshot.

```
  Checkpointer
 ┌────────────────────────────────────────────────────────────────────┐
 │                        Wallet                                      │
 │                       ┌─────────────────────┐                      │
 │ ┌──────────────┐      │  ┌──────────────┐   │     ┌──────────────┐ │       ┌──────────────┐
 │ │ Config A & B ├──────┼──▶ Config A & C ├───┼─────▶ Config D & C ├─┼───────▶ Config D & E │
 │ └──────┬───────┘      │  └──────────────┘   │     └──────────────┘ │       └──────┬───────┘
 │        │              └─────────────────────┘                      │              │
 └────────┼───────────────────────────────────────────────────────────┘              │
          │                                                                          │
          ▼                                                                          ▼
Old state - Irrelevant                                                          Latest state
```

#### 6. Wallet Synced, Checkpointer Dangling Ahead

The checkpointer may report a future configuration that is not part of the state channel. This may happen if the checkpointer is misbehaving, malicious, or there is a state channel update that has only been revealed to the checkpointer.

In this scenario, the checkpointer contract **must** provide with an escape hatch mechanism for either:

1. Provide the link to the dangling configuration
2. Disable itself

```
  Checkpointer
 ┌────────────────────────────────────────────────────────────────────┐
 │                        Wallet                                      │
 │                       ┌─────────────────────┐                      │
 │ ┌──────────────┐      │  ┌──────────────┐   │     ┌──────────────┐ │
 │ │ Config A & B ├──────┼──▶ Config A & C ├───┼─x   │ Config D & C │ │
 │ └──────┬───────┘      │  └───────┬──────┘   │     └───────┬──────┘ │
 │        │              └──────────┼──────────┘             │        │
 └────────┼─────────────────────────┼────────────────────────┼────────┘
          │                         │                        │
          ▼                         ▼                        ▼
Old state - Irrelevant         Latest state         Dangling, not linked
```

### Checkpointer signature format

Signatures that involve a checkpointer contain two additional pieces of data, the "checkpointer address" that is used to identify the checkpointer (and to be able to reconstruct the merkle root), and the "checkpointer data" which is needed to obtain the snapshot from the checkpointer.

#### Non-chained signatures

When non-chained signatures are used, the signature flag **must** have the `X1XX XXXX` bit set (`flag & 0x08 == 0x08`), this signals to the wallet contract that the signature involves a checkpointer, afterwards fixed 20 bytes are read to obtain the checkpointer address, and the next 3 bytes are read to obtain the checkpointer data size, after which the checkpointer data is read.

```
  ┌───▶ 1 byte global flag
  │     must have the 2nd bit
  │     set to 1 to signal a chained             ┌───▶ 3 bytes define the size
  │     signature                                │     of the checkpointer data
  │                                              │
──┴─                                          ───┴──
0x40 6974206973207265616c6C792073756E6E791aDd 001271 6b656570206275696c64696e67...
     ──┬─────────────────────────────────────        ─┬───────────────────────────
       │                                              │
       └─▶ 20 byte address                            └─▶ Dynamic size checkpointer
           determines the address                         data
           of the checkpointer
```

#### Chained signatures

If chained signatures are used, then the encoding becomes slightly more complex. The reason for this is that the **only** checkpointer that is used is the one that is defined by the wallet contract, **checkpointers provided by the state channel are ignored**.

This is intended behavior, as the checkpointer is in charge of overseeing the state channel, thus it wouldn't be safe for the state channel to be able to provide a checkpointer, as it could self-authorize the state channel. This would negate the purpose of having a checkpointer in the first place.

There is a challenge with this, when a chained signature is encoded, the configuration that corresponds to the wallet contract is provided **at the end of the chain**, which is a problem, because we need to obtain the snapshot at the beginning of the chain, in order to validate if the state channel ever "goes over" the snapshot provided by the checkpointer.

Additionally, intermediary configurations need to have their checkpointers provided, as it is needed to reconstruct the merkle root. However no checkpointer data is needed, as these intermediate checkpointers are not called.

To solve this, when a chained signature is used, and the onchain configuration contains a checkpointer, the checkpointer of the **last** configuration is moved at the beginning of the chain, alongside its checkpointer data, and no checkpointer data is provided for the intermediate signature parts.

```
     ┌─▶ 2nd. bit signals checkpointer    ┌──▶ The checkpointer and data is only
     │                                    │    provided once, as part of the
     │      ┌▶ 8th. bit signals chained   │    chained signatures, and not of
     ┴      ┴                             │    the individual parts
    0100 0001                             │
    ▲ ┌───────────────────────────────────┴─────────────────────────────────────────┐
    │ │                                       3 byte Size                           │
  ──┴─│                                         ──────                              │
  0x41│6974206973207265616c6C792073756E6E791aDd 001271 6b656570206275696c64696e67...│─┐
      │──────────────────────────────────────── ────────────────────────────────────│ │
      │Checkpointer address □                   Checkpointer data                   │ │
      └─────────────────────╫───────────────────────────────────────────────────────┘ │
╔═══════════════════════════╝                                                         │
║    ┌────────────────────────────────────────────────────────────────────────────────┘
║    │
║    │     Payload signature                          ┌─▶ No checkpointer data
║    │    ┌───────────────────────────────────────────┼───────────────────┐
║    │    │                                           x                   │
║    └───▶│44 6974206973207265616c6C792073756E6E791aDd 03 01 5151515151...│─┐
║         │─┬─────────────────────────────────────────                    │ │
║         └─┼─────────────────────────────────────────────────────────────┘ │
║           │                                                               │
║           └───▶ Signature requires checkpointer                           │
║                 but no checkpointer data is                               │
║                 required as it is only for root                           │
║                 recovery                                                  │
║                                                                           │
║                 Checkpointer may or may not match                         │
║                                                                           │
║          ┌──────────────────────────────────────────────────────┐         │
║    ┌─────┤  Intermediary signatures follow the previous format  ├─────────┘
║    │     └──────────────────────────────────────────────────────┘
║    │
║    │     Final signature
║    │    ┌────────────────────────────────┐
║    │    │                                │
║    └───▶│04 03 01 5252525252552525252... │
║         │  x      ───────────────────┬── │
║         └──┼─────────────────────────┼───┘
║            │                         │
║            └▶ No checkpointer        │
║               No checkpointer data   └────▶ Regular signature body
║
╚════════════▷  Recovery uses the checkpointer
                that got at the start of the
                chained signature
```

### Checkpointer control flow

In summary, only the checkpointer that is defined by the wallet contract is used. If a non-chained signature is used, it has to either match the checkpointer's snapshot, or it must be behind the wallet contract.

If a chained signature is used, then at some point during the chain, the snapshot provided by the checkpointer must match with one of the recovered configurations, this may also happen at the final signature. If this does not happen, the final signature (that corresponds to the wallet contract) **must** be ahead of the checkpointer's snapshot.

```
                                     ╔═══════════════════╗
                                     ║                   ║
                                     ║    Validation     ║
                                     ║       start       ║
                                     ║                   ║
                                     ╚═════════╤═════════╝
                                               │
                                               │
                                               │
                                     ┌─────────▼─────────┐
                                     │                   │
                                     │  Obtain snapshot  │
                                     │                   │
                                     │                   │
                                     └─────────┬─────────┘
                                               │
                                               │
                                               │
        ╔═══════════════════╗        ╭─────────▼─────────╮
        ║                   ║        │    Is snapshot    │
        ║    Approved by    ║     Yes│     equal to      │
        ║   checkpointer    ◀────────┤    bytes32(0)?    │
        ║                   ║        │                   │
        ╚═══════════════════╝        ╰─────────┬─────────╯
                                               │No
                                               │
                                               │
                                     ╭─────────▼─────────╮
                                     │                   │
                                     │    Is chained     │ Yes
                                     │    signature?     ├────────────────╮
                                     │                   │                │
                                     ╰─────────┬─────────╯                │
                                               │No                        │
                                               │                          │
                                               │                          │
        ╔═══════════════════╗        ┌─────────▼─────────┐                │
        ║                   ║        │                   │                │
        ║    Rejected by    ║        │      Recover      │                │
        ║   checkpointer    ║        │    image hash     │                │
        ║                   ║        │                   │                │
        ╚═════════▲═════════╝        └─────────┬─────────┘                │
                  │                            │                          │
                  │                            │                          │
                  │No                          │                          │
        ╭─────────┴─────────╮        ╭─────────▼─────────╮                │
        │                   │        │                   │                │
        │ Checkpoint below  │     No │      Matches      │                │
        │     snapshot?     ◀────────┤     snapshot?     │                │
        │                   │        │                   │                │
        ╰─────────┬─────────╯        ╰─────────┬─────────╯                │
                  │Yes                         │Yes                       │
                  │                            │                          │
                  │                            │                          │
                  │                  ╔═════════▼═════════╗                │
                  │                  ║                   ║                │
                  │                  ║    Approved by    ║                │
                  └──────────────────▶   checkpointer    ║                │
                                     ║                   ║                │
                                     ╚═══════════════════╝                │
                                                                          │
                                                                          │
                                                                          │
                                                          ┌───────────────▼───┐
                                                          │                   │
                                                          │      Recover      │
                                       ┌──────────────────▶    image hash     │
                                       │                  │                   │
                                       │                  └─────────┬─────────┘
                                       │                            │
                                       │                            │
                                       │No                          │
                             ╭─────────┴─────────╮        ╭─────────▼─────────╮
                             │                   │        │                   │
                          Yes│   Was the last    │     No │      Matches      │
          ┌──────────────────┤  signature part?  ◀────────┤     snapshot?     │
          │                  │                   │        │                   │
          │                  ╰─────────▲─────────╯        ╰─────────┬─────────╯
          │                            │                            │Yes
          │                            │                            │
          │                            │                            │
╭─────────▼─────────╮                  │                  ┌─────────▼─────────┐
│                   │                  │                  │                   │
│ Was the snapshot  │Yes               │                  │      Consume      │
│     consumed?     ├─────────────┐    └──────────────────┤     snapshot      │
│                   │             │                       │                   │
╰─────────┬─────────╯             │                       └───────────────────┘
          │No                     │
          │                       │
          │                       │
╭─────────▼─────────╮        ╔════▼══════════════╗
│                   │        ║                   ║
│  Last checkpoint  │Yes     ║    Approved by    ║
│  below snapshot?  ├────────▶   checkpointer    ║
│                   │        ║                   ║
╰─────────┬─────────╯        ╚═══════════════════╝
          │No
          │
          │
╔═════════▼═════════╗
║                   ║
║    Rejected by    ║
║   checkpointer    ║
║                   ║
╚═══════════════════╝
```

# Sequence Wallet Configuration Documentation

This document provides a description on how the Sequence wallet contracts define and handle sets of signers, checkpoints, extensions and other parameters that fall within the scope of its "configuration".

## **1. Overview**

All Sequence wallets have a defined "configuration", this is the set of parameters that governs the wallet's behavior.

The configuration includes:

- The threshold needed to authorize a transaction.
- The set of signers, and their corresponding weights.
- The current "checkpoint", which determines the order of configuration updates.
- Any "extensions" that are enabled, with their corresponding parameters.
- The "checkpointer" contract, which acts as a connector for a keystore rollup.
- Any pre-authorized transactions or messages, that are considered signed by the wallet.

All these parameters are encoded into leaves of a merkle tree, of which the root is stored either as counter-factual information during wallet creation (as the salt of the `CREATE2` opcode), or directly within the contract storage; this "merkle root" is internally referred as the `imageHash` of the wallet.

Since the wallet contract does not have direct access to the configuration, every time a signature is provided, the signature must provide both the "signature parts" that may be needed for the individual signers, as well as the "merkle proof" that allows the wallet contract to reconstruct the `imageHash` and thus validate the signature.

## **2. Tree structure**

The tree is a **sparse binary merkle tree**, this allows for more frequently accessed parameters to be stored closer to the root, making merkle proofs more efficient. Some elements like the `checkpointer`, `checkpoint` and `threshold` have fixed positions at the top of the tree, afterwards the other leaves can be arranged in any order.

The top of the tree looks like:

```
imageHash = keccak256(
    keccak256(
        keccak256(
          ...leaves,
          threshold
        ),
        checkpoint
    ),
    checkpointer || address(0)
)
```

The `checkpointer` is optional, if it is not provided it still has to be included in the merkle tree, it will be automatically set to `address(0)` if the signature flags do not set the `CHECKPOINTER` bit.

```
             ┌────────────────┐
             │                │
             │   Image hash   │
             │                │
             └───▲────────▲───┘
                 │        │
             ┌───┘        └────┐
             │      ┌──────────┼───────────┐
        ╭────┴───╮  │  ┌───────┴────────┐  │
        │        │  │  │                │  ├────▶ This section is
        │        │  │  │  Checkpointer  │  │      always in the tree.
        │        │  │  │                │  │
        ╰─▲────▲─╯  │  └────────────────┘  │      It won't be shown
          │    │    │                      │      in the other diagrams.
          │    └────┼──────────┐           │
          │         │          │           │
        ╭─┴──────╮  │  ┌───────┴────────┐  │
        │        │  │  │                │  │
        │        │  │  │   Checkpoint   │  │
        │        │  │  │                │  │
        ╰─▲────▲─╯  │  └────────────────┘  │
          │    │    │                      │
          │    └────┼──────────┐           │
          │         │          │           │
╔═════════╧══════╗  │  ┌───────┴────────┐  │
║    ... rest    ║  │  │                │  │
║  of the tree   ║  │  │   Threshold    │  │
║                ║  │  │                │  │
╚════════════════╝  │  └────────────────┘  │
                    └──────────────────────┘
```

## **3. Leaf types**

### 3.1 Signer leaf

Signer leaves are used to specify signer addresses, meaning that when a valid signature is provided, the wallet will count their weight towards the threshold. Signer leaves do not specify if they are ERC1271 signers or not, instead this is determined when the signature is encoded.

```
                     ┌────────────────┐
                     │                │
                     │   Image hash   │
                     │                │
                     └───▲────────▲───┘
                         │        │
                     ┌───┘        └──────┐
                     │                   │
                ╭────┴───╮     ╔═════════╧═════════╗
                │        │     ║Checkpointer: 0x   ║
                │        │     ║Checkpoint: 0      ║
                │        │     ║Threshold: 1       ║
                ╰─▲────▲─╯     ╚═══════════════════╝
                  │    │
          ┌───────┘    └──────┐
          │                   │
          │                   │
┌────────────────┐     ┌────────────────┐
│  Signer 0xaaa  │     │  Signer 0xbbb  │
│   Weight: 1    │     │   Weight: 1    │
│                │     │                │
└────────────────┘     └────────────────┘
```

The hash of a signer leaf is computed as:

```
leaf = keccak256(
  "Sequence signer:\n",
  address,
  weight
)
```

> Notice that any address that is registered **more than once** will be counted **multiple times** in the threshold.

### 3.2 Hardcoded signature leaf

Hardcoded signature leaves allow for the configuration to automatically sign a specific payload, this is useful for pre-approving transactions, for example pre-approving an approval for an ERC20 at the same time as the wallet is created.

If a payload hash matches the hardcoded hash (subdigest), the weight counted towards the threshold will automatically be bumped to `type(uint256).max`, ensuring the signature to be valid.

```
            ┌────────────────┐
            │                │
            │   Image hash   │
            │                │
            └───▲────────▲───┘
                │        │
        ┌───────┘        └────────┐
        │                         │
┌───────┴────────┐      ╔═════════╧═════════╗
│   Subdigest:   │      ║Checkpointer: 0x   ║
│   0x11223344   │      ║Checkpoint: 0      ║
│                │      ║Threshold: 1       ║
└────────────────┘      ╚═══════════════════╝
```

The hash of a hardcoded signature leaf is computed as:

```
leaf = keccak256(
  "Sequence static digest:\n",
  subdigest
)
```

### 3.3 Any address hardcoded signature leaf

This works very similarly to the hardcoded signature leaf, but the hash of the payload is computed setting its ERC712 domain separator to `address(0)` instead of the wallet's address.

This allows for pre-approving transactions before the wallet has counterfactually been defined, otherwise attempting to compute the hash would require the wallet's address to be known beforehand, and determining the address would require the hash to be known beforehand, causing a recursive dependency.

After a wallet is created, it is recommended to use the hardcoded signature leaf instead, as it is more efficient.

The hash of an any address hardcoded signature leaf is computed as:

```
leaf = keccak256(
  "Sequence any address subdigest:\n",
  subdigest
)
```

### 3.4 Nested configuration leaf

Nested configuration leaves are used to specify nested configuration trees, this allows for certain sets of signers to be grouped in such a way that their signing power is limited by a threshold. It can also allow to augment the signing power of a set of signers, by making them have no signing power individually but still be able to sign as a group.

They have their own internal thresholds, internally they work exactly as the parent "main" tree, but externally they can only contribute their own weight.

```
                     ┌────────────────┐
                     │                │
                     │   Image hash   │
                     │                │
                     └───▲────────▲───┘
                         │        │
                     ┌───┘        └──────┐
                     │                   │
                ╭────┴───╮     ╔═════════╧═════════╗
                │        │     ║ Checkpointer: 0x  ║
                │        │     ║   Checkpoint: 0   ║
                │        │     ║   Threshold: 2    ║
                ╰─▲────▲─╯     ╚═══════════════════╝
                  │    │
           ┌──────┘    └──────┐
           │                  │                    ────┐
┌──────────┴─────┐      ┌─────┴──────────┐             │
│  Signer 0xaaa  │      │  Threshold: 1  │             ├─▶ The maximum weight
│   Weight: 1    │      │   Weight: 1    │             │   of 0xbbb and 0xccc
│                │      │                │             │   can't exceed 1
└────────────────┘      └─────▲────▲─────┘             │
                              │    │                   │   Forced either of
                       ┌──────┘    └──────┐            │   them to be combined
                       │                  │            │   with 0xaaa
            ┌──────────┴─────┐      ┌─────┴──────────┐ │
            │ Signer: 0xbbb  │      │  Signer 0xccc  │ │
            │   Weight: 1    │      │   Weight: 2    │ │
            │                │      │                │ │
            └────────────────┘      └────────────────┘ │
                                                   ────┘
```

The hash of a nested tree is computed as:

```
leaf = keccak256(
  "Sequence nested config:\n",
  node,
  threshold,
  weight
)
```

### 3.5 Sapient Signer Leaf

Sapient signer leaves work similarly to Signer leaves, but have 3 key distinctions:

1. Sapient signers **must** always be smart contracts, EOA signers are not allowed
2. Sapient signers **do not** use the ERC1271 interface, instead they get passed a fully decoded `Payload` to sign
3. Sapient signatures are not validated, they are **recovered** into a hash

When a sapient signer leaf is added to a configuration, it must define its `address`, `weight` and `imageHash`. The `imageHash` represents the expected value that the sapient signer **must recover** for the signature to be valid.

This allows sapient signers to act as **extensions** of the configuration tree, they can define their own leaf types and overall structures, alongside arbitrary logic. This allows for extensions to be configured per wallet without having to rely on the contract storage of the extension, as the definition of the configuration resides in the same `imageHash`.

```
                           ┌────────────────┐
                           │                │
                           │   Image hash   │
                           │                │
                           └───▲────────▲───┘
                               │        │
                           ┌───┘        └──────┐
                           │                   │
                      ╭────┴───╮     ╔═════════╧═════════╗
                      │        │     ║ Checkpointer: 0x  ║
                      │        │     ║   Checkpoint: 0   ║
                      │        │     ║   Threshold: 2    ║
                      ╰─▲────▲─╯     ╚═══════════════════╝
                        │    │
                 ┌──────┘    └──────┐
                 │                  │                                      ────┐
      ┌──────────┴─────┐      ┌─────┴──────────┐                               ├─▶ The passkey sapient
      │  Signer 0xaaa  │      │    Passkey     │                               │   signer uses its tree
      │   Weight: 1    │      │   weight: 1    │                               │   to define public
      │                │      │imageHash: 0xabc│                               │   keys and
      └────────────────┘      └─────▲────▲─────┘                               │   configuration
                                    │    │                                     │   parameters.
                    ┌───────────────┘    └───────────────┐                     │
                    │                                    │                     │   Signatures must be
              ╭─────┴──╮                              ╭──┴─────╮               │   able to recover the
              │        │                              │        │               │   inner root of the
              │        │                              │        │               │   tree to be
              │        │                              │        │               │   considered valid.
              ╰─▲────▲─╯                              ╰─▲────▲─╯               │
                │    │                                  │    │                 │
         ┌──────┘    └──────┐                    ┌──────┘    └──────┐          │
         │                  │                    │                  │          │
┌────────┴───────┐  ┌───────┴────────┐  ┌────────┴───────┐  ┌───────┴────────┐ │
│                │  │                │  │  Require user  │  │                │ │
│    Pubkey X    │  │    Pubkey Y    │  │  verification  │  │    Metadata    │ │
│                │  │                │  │   true/false   │  │                │ │
└────────────────┘  └────────────────┘  └────────────────┘  └────────────────┘ │
                                                                           ────┘
```

The hash of a sapient signer leaf is:

```
leaf = keccak256(
  "Sequence sapient config:\n",
  address,
  weight,
  imageHash
)
```

# Sequence Payload System Documentation

This document provides a comprehensive overview of how the Sequence wallet contracts encode, decode, and execute payloads. It explains the payload structure, encoding schemes, execution flow, and how payloads integrate with the broader signature and configuration systems.

---

## **1. Overview**

The Sequence payload system is the core mechanism through which wallets execute batched operations. Payloads are encoded data structures that contain:

- **Transaction calls** with target addresses, values, and calldata
- **Configuration updates** for wallet parameters
- **Message validations** for arbitrary data signing
- **Digest verifications** for ERC-1271 compatibility

Payloads are designed to be:

1. **Gas-efficient** through compact binary encoding
2. **Flexible** to support various operation types
3. **Secure** through EIP-712 structured hashing
4. **Batchable** to execute multiple operations atomically

---

## **2. Payload Types**

The system supports four distinct payload kinds, each serving different purposes:

### **2.1 Transaction Payloads (`KIND_TRANSACTIONS = 0x00`)**

Transaction payloads contain batched calls to external contracts or self-execution logic. These are the most common payload type and support:

- Multiple contract calls in sequence
- Value transfers with calls
- Delegate calls for extension execution
- Gas limit specifications
- Error handling behaviors
- Fallback-only execution modes

### **2.2 Message Payloads (`KIND_MESSAGE = 0x01`)**

Message payloads allow wallets to sign arbitrary data for off-chain verification. This is useful for:

- Meta-transactions
- Off-chain authorization
- Cross-chain message signing
- General-purpose data signing

### **2.3 Configuration Update Payloads (`KIND_CONFIG_UPDATE = 0x02`)**

Configuration update payloads enable wallet reconfiguration through:

- Signer set modifications
- Threshold adjustments
- Extension enablement/disablement
- Checkpoint updates

### **2.4 Digest Payloads (`KIND_DIGEST = 0x03`)**

Digest payloads provide ERC-1271 compatibility by allowing wallets to validate pre-computed message hashes.

---

## **3. Payload Structure**

### **3.1 Core Payload Structure**

All payloads share a common structure defined by the `Decoded` struct:

```solidity
struct Decoded {
    uint8 kind;           // Payload type identifier
    bool noChainId;       // Chain ID inclusion flag
    // Transaction-specific fields
    Call[] calls;         // Array of call operations
    uint256 space;        // Nonce space identifier
    uint256 nonce;        // Nonce value for replay protection
    // Message-specific fields
    bytes message;        // Raw message data
    // Configuration-specific fields
    bytes32 imageHash;    // New configuration hash
    // Digest-specific fields
    bytes32 digest;       // Pre-computed message hash
    // Common fields
    address[] parentWallets; // Parent wallet addresses
}
```

### **3.2 Call Structure**

Individual calls within transaction payloads are defined by the `Call` struct:

```solidity
struct Call {
    address to;                    // Target contract address
    uint256 value;                 // ETH value to send
    bytes data;                    // Calldata for the call
    uint256 gasLimit;              // Gas limit for execution
    bool delegateCall;             // Delegate call flag
    bool onlyFallback;             // Fallback-only execution
    uint256 behaviorOnError;       // Error handling strategy
}
```

---

## **4. Payload Encoding**

### **4.1 Transaction Payload Encoding**

Transaction payloads use a compact binary encoding scheme optimized for gas efficiency. The encoding follows this structure:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Global Flag (1 byte)                                                        │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ Bit 0: Space flag (0 = read space, 1 = space is 0)                      │ │
│ │ Bits 1-3: Nonce size (0-7 bytes)                                        │ │
│ │ Bit 4: Single call flag (0 = multiple calls, 1 = single call)           │ │
│ │ Bit 5: Call count size (0 = 1 byte, 1 = 2 bytes)                        │ │
│ │ Bits 6-7: Reserved                                                      │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────────────────────┤
│ Space (0 or 20 bytes) - only if space flag is 0                             │
├─────────────────────────────────────────────────────────────────────────────┤
│ Nonce (0-7 bytes) - size determined by bits 1-3                             │
├─────────────────────────────────────────────────────────────────────────────┤
│ Call Count (1-2 bytes) - size determined by bit 5                           │
├─────────────────────────────────────────────────────────────────────────────┤
│ Call Array (call count length)                                              │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ Call Flags (1 byte)                                                     │ │
│ │ ┌─────────────────────────────────────────────────────────────────────┐ │ │
│ │ │ Bit 0: Self-call flag (0 = external, 1 = self)                      │ │ │
│ │ │ Bit 1: Value flag (0 = no value, 1 = has value)                     │ │ │
│ │ │ Bit 2: Data flag (0 = no data, 1 = has data)                        │ │ │
│ │ │ Bit 3: Gas limit flag (0 = no limit, 1 = has limit)                 │ │ │
│ │ │ Bit 4: Delegate call flag                                           │ │ │
│ │ │ Bit 5: Fallback-only flag                                           │ │ │
│ │ │ Bits 6-7: Behavior on error (00=ignore, 01=revert, 10=abort)        │ │ │
│ │ └─────────────────────────────────────────────────────────────────────┘ │ │
│ ├─────────────────────────────────────────────────────────────────────────┤ │
│ │ Target Address (0 or 20 bytes) - only if not self-call                  │ │
│ ├─────────────────────────────────────────────────────────────────────────┤ │
│ │ Value (0 or 32 bytes) - only if value flag is set                       │ │
│ ├─────────────────────────────────────────────────────────────────────────┤ │
│ │ Data Size (0 or 3 bytes) - only if data flag is set                     │ │
│ ├─────────────────────────────────────────────────────────────────────────┤ │
│ │ Data (0 or N bytes) - only if data flag is set                          │ │
│ ├─────────────────────────────────────────────────────────────────────────┤ │
│ │ Gas Limit (0 or 32 bytes) - only if gas limit flag is set               │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────┘
```

### **4.2 Global Flag Breakdown**

The global flag byte controls the overall payload structure:

```
Global Flag: [7][6][5][4][3][2][1][0]
             │  │  │  │  │  │  │  └── [0] Space flag
             │  │  │  │  │  │  └───── [1][0] Nonce size
             │  │  │  │  │  └──────── [2][1] Nonce size
             │  │  │  │  └─────────── [3][2] Nonce size
             │  │  │  └────────────── [4] Single call flag
             │  │  └───────────────── [5] Call count size
             │  └──────────────────── [6] Reserved
             └─────────────────────── [7] Reserved
```

**Space Flag (Bit 0):**

- `0`: Read 20-byte space address from payload
- `1`: Set space to 0 (default space)

**Nonce Size (Bits 1-3):**

- `000`: No nonce (nonce = 0)
- `001`: 1-byte nonce
- `010`: 2-byte nonce
- `011`: 3-byte nonce
- `100`: 4-byte nonce
- `101`: 5-byte nonce
- `110`: 6-byte nonce
- `111`: 7-byte nonce

**Single Call Flag (Bit 4):**

- `0`: Multiple calls (read call count)
- `1`: Single call (call count = 1)

**Call Count Size (Bit 5):**

- `0`: Call count stored in 1 byte
- `1`: Call count stored in 2 bytes

### **4.3 Call Flags Breakdown**

Each call begins with a flags byte that controls its execution parameters:

```
Call Flags: [7][6][5][4][3][2][1][0]
            │  │  │  │  │  │  │  └─── [0] Self-call flag
            │  │  │  │  │  │  └────── [1] Value flag
            │  │  │  │  │  └───────── [2] Data flag
            │  │  │  │  └──────────── [3] Gas limit flag
            │  │  │  └─────────────── [4] Delegate call flag
            │  │  └────────────────── [5] Fallback-only flag
            │  └───────────────────── [6][0] Behavior on error
            └──────────────────────── [7][1] Behavior on error
```

**Self-call Flag (Bit 0):**

- `0`: External call (read target address)
- `1`: Self-call (target = address(this))

**Value Flag (Bit 1):**

- `0`: No ETH value
- `1`: Read 32-byte value from payload

**Data Flag (Bit 2):**

- `0`: No calldata
- `1`: Read 3-byte data size + N bytes of data

**Gas Limit Flag (Bit 3):**

- `0`: Use remaining gas
- `1`: Read 32-byte gas limit from payload

**Delegate Call Flag (Bit 4):**

- `0`: Regular call
- `1`: Delegate call

**Fallback-only Flag (Bit 5):**

- `0`: Execute normally
- `1`: Only execute if previous call failed

**Behavior on Error (Bits 6-7):**

- `00`: Ignore error, continue execution
- `01`: Revert entire transaction
- `10`: Abort execution, stop processing
- `11`: Reserved

---

## **5. Payload Execution**

### **5.1 Execution Flow**

The payload execution follows this sequence:

1. **Decode Payload**: Parse the binary payload into structured data
2. **Nonce Validation**: Consume the nonce to prevent replay attacks
3. **Signature Verification**: Validate the signature against the payload
4. **Call Execution**: Execute each call in sequence
5. **Error Handling**: Apply error handling based on call configuration

### **5.2 Call Execution Modes**

#### **Regular Calls**

Standard contract calls that execute with the specified parameters and return control to the wallet.

#### **Delegate Calls**

Execute code in the context of the wallet contract, allowing extensions to modify wallet state.

#### **Fallback-only Calls**

Only execute when the immediately preceding call fails (and has the `BEHAVIOR_IGNORE_ERROR` behavior), enabling conditional execution flows. See Error Handling Strategies below.

### **5.3 Error Handling Strategies**

The system provides three error handling behaviors:

1. **Ignore Error (`BEHAVIOR_IGNORE_ERROR`)**

   - Continue execution with subsequent calls
   - Set error flag for fallback-only calls
   - Emit `CallFailed` event

2. **Revert on Error (`BEHAVIOR_REVERT_ON_ERROR`)**

   - Revert entire transaction
   - Rollback all state changes
   - Emit `Reverted` error

3. **Abort on Error (`BEHAVIOR_ABORT_ON_ERROR`)**
   - Stop processing remaining calls
   - Keep successful call results
   - Emit `CallAborted` event

---

## **6. EIP-712 Integration**

### **6.1 Domain Separator**

Payloads use EIP-712 for structured hashing with the domain:

```
EIP712Domain(
    name: "Sequence Wallet"
    version: "3"
    chainId: block.chainid (or 0 if noChainId)
    verifyingContract: wallet address
)
```

### **6.2 Type Hashes**

Each payload type has a specific type hash:

- **Calls**: `keccak256("Calls(Call[] calls,uint256 space,uint256 nonce,address[] wallets)")`
- **Message**: `keccak256("Message(bytes message,address[] wallets)")`
- **ConfigUpdate**: `keccak256("ConfigUpdate(bytes32 imageHash,address[] wallets)")`

### **6.3 Call Hashing**

Individual calls are hashed using:

```
keccak256(
    CALL_TYPEHASH,
    to,
    value,
    keccak256(data),
    gasLimit,
    delegateCall,
    onlyFallback,
    behaviorOnError
)
```

---

## **7. Gas Optimization Features**

### **7.1 Compact Encoding**

The payload system uses several techniques to minimize gas costs:

- **Flag-based encoding**: Single bytes control multiple parameters
- **Variable-length fields**: Only encode necessary data
- **Self-call optimization**: Avoid 20-byte address encoding for self-calls
- **Conditional encoding**: Skip fields that have default values

### **7.2 Batch Processing**

Multiple calls can be executed in a single transaction, reducing:

- Transaction overhead
- Gas costs for multiple operations
- Network congestion
- User interaction requirements

---

## **8. Security Considerations**

### **8.1 Replay Protection**

- **Nonce system**: Each payload requires a unique nonce
- **Space isolation**: Different nonce spaces prevent cross-contamination
- **Chain ID binding**: Prevents cross-chain replay attacks

### **8.2 Access Control**

- **Signature validation**: All payloads require valid signatures
- **Configuration binding**: Payloads are tied to specific wallet configurations
- **Extension isolation**: Delegate calls are restricted to approved extensions

### **8.3 Error Handling**

- **Graceful degradation**: Failed calls don't necessarily fail the entire batch
- **Gas protection**: Individual gas limits prevent infinite loops
- **State consistency**: Error behaviors maintain wallet integrity

---

## **Conclusion**

The Sequence payload system provides a powerful, gas-efficient mechanism for executing complex wallet operations. Through its compact binary encoding, flexible call structures, and comprehensive error handling, it enables wallets to perform sophisticated multi-step operations while maintaining security and efficiency.

The system's integration with the broader Sequence ecosystem—including chained signatures, smart sessions, and configuration management—creates a cohesive framework for advanced wallet functionality. The payload system serves as the execution engine that brings together all these components into a unified user experience.

This documentation serves as a technical reference for developers implementing and extending the payload system, providing both high-level architectural understanding and detailed implementation guidance.

# Ecosystem Wallets Smart Sessions Documentation

This document provides an in-depth overview of the smart sessions system in Ecosystem wallets. It explains the encoding of signatures and configurations, details the permissions system, and distinguishes between explicit sessions and implicit sessions.

---

## Overview

Ecosystem wallets smart sessions enable batched call authorization via signed payloads. Two primary session modes are supported:

- **Explicit Sessions:**  
  Explicit sessions are part of the wallet's configuration. Their permissions are granted counter factually - derived from signature calldata. These permissions can be added or removed with a configuration update. As the configuration is tied to the wallet's image hash, any change to the wallet (and thus its image hash) immediately affects which explicit session permissions remain valid.

- **Implicit Sessions:**  
  Implicit sessions are automatically able to sign on behalf of the wallet when they present an attestation that is signed by the wallet's identity signer. This mode leverages off-chain attestations and enforces additional constraints (e.g., blacklisting) to protect against misuse.

---

## Signature Encoding

Signature encoding consists of **three** main parts:

1. **Session Configuration Encoding**
2. **Attestation List Encoding**
3. **Call Signatures Encoding**

Each part uses a specific layout and bit-level structure to efficiently encode the required data.

---

### 1. Session Configuration Encoding

The session configuration is embedded within the signature as follows:

```
┌─────────────────────────────────────────────────────┐
│ uint24 dataSize                                     │
│ ┌───────────────────────────────────────────────┐   │
│ │ Session Configuration Bytes (dataSize bytes)  │   │
│ └───────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

Within these configuration bytes, the data is structured as a series of tagged nodes. Each node begins with a flag byte that indicates the node type and any associated metadata.

#### Flag Byte Structure

```
 ┌───────────────────────────────┐
 │  Bits 7..4: FLAG              │  (Identifies the node type)
 │  Bits 3..0: Additional Data   │  (Depends on the FLAG)
 └───────────────────────────────┘
```

The following flags are defined:

- **0x00: Permissions Node**
- **0x01: Hash Node (Pre-hashed 32-byte value)**
- **0x02: Branch Node (Nested encoding)**
- **0x03: Blacklist Node**
- **0x04: Identity Signer Node**

> [!IMPORTANT]
> During validation there must be **exactly one** Identity Signer and **at most one** Blacklist node. Multiple entries will trigger a validation error. If there are any implicit sessions (attestations), a blacklist is mandatory.

> [!TIP]
> Unused nodes may be hashed into hash nodes to recover the correct image hash with reduced processing. A complete configuration may have multiple Identity Signers but must only include one unhashed Identity Signer node for validation.

#### Permissions Node (FLAG 0x00)

This node encodes session permissions for a specific signer:

```
Permissions Node Layout:
 ┌─────────────────────────────────────────────┐
 │ Flag Byte                                   │
 │   ┌────────────────────────┐                │
 │   │ Bits 7..4: FLAG (0x00) │                │
 │   │ Bits 3..0: Unused      │                │
 │   └────────────────────────┘                │
 │ Signer (address)                            │
 │ Value Limit (uint256)                       │
 │ Deadline (uint256)                          │
 │ Permissions Array (encoded permissions)     │
 └─────────────────────────────────────────────┘
```

##### Permission Object Encoding

Each permission object is structured as follows:

```
Permission Encoding:
 ┌─────────────────────────────┐
 │ Target Address              │
 │ Rules Count (uint8)         │
 │ ┌─────────────────────────┐ │
 │ │ Parameter Rule 1        │ │
 │ │ Parameter Rule 2        │ │  ... (if any)
 │ └─────────────────────────┘ │
 └─────────────────────────────┘
```

If the **Rules Count** is zero, the permission is considered _open_, allowing any call that targets the specified address without additional parameter restrictions.

##### Parameter Rule Encoding

Each parameter rule enforces conditions on the call data:

```
Parameter Rule Encoding:
 ┌──────────────────────────────────────────────────────────────┐
 │ Operation & Cumulative Flag (1 byte)                         │
 │   ┌────────────────────────────────────────────────────────┐ │
 │   │ Bits 7..1  (0xFE): Operation (e.g., 0 = EQUAL, etc.)   │ │
 │   │ Bit 0 (0x01): Cumulative flag (1 = cumulative)         │ │
 │   └────────────────────────────────────────────────────────┘ │
 │ Value (bytes32)                                              │
 │ Offset (uint256)                                             │
 │ Mask (bytes32)                                               │
 └──────────────────────────────────────────────────────────────┘
```

> [!TIP]
> A permission with an empty rules array is treated as _open_, granting unrestricted access to the target, subject only to other constraints such as value limits and deadlines.

#### Hash Node (FLAG 0x01)

This node includes a 32-byte pre-hashed value:

```
Node Layout:
 ┌──────────────────────────────┐
 │ Flag Byte                    │
 │   ┌────────────────────────┐ │
 │   │ Bits 7..4: FLAG (0x01) │ │
 │   │ Bits 3..0: Unused      │ │
 │   └────────────────────────┘ │
 │ Node Hash (bytes32)          │
 └──────────────────────────────┘
```

This node is an optimization to reduce the size of the configuration tree in calldata. By using this node, unused permissions or configuration segments can be hidden, while still allowing the complete image hash to be derived.

#### Branch (FLAG 0x02)

Branches allow for the recursive grouping of nested configuration nodes into a single unit. They are used to bundle together multiple nodes - such as several permissions nodes or even other branch nodes - so that the entire collection can be processed as one entity. This design minimizes redundancy and optimizes the calldata size by avoiding repeated encoding of common structures.

```
Branch Node Layout:
 ┌──────────────────────────────────────────────┐
 │ Flag Byte                                    │
 │   ┌────────────────────────────────────────┐ │
 │   │ Bits 7..4: FLAG (0x02)                 │ │
 │   │ Bits 3..0: Size of size field in bytes │ │
 │   └────────────────────────────────────────┘ │
 │ Size (uintX, where X is determined above)    │
 │ Branch Data (nested configuration bytes)     │
 └──────────────────────────────────────────────┘
```

The **Size** field specifies the total number of bytes that the branch occupies. The size of this field is determined by the additional data portion of the flag byte (bits 3..0), which indicates how many bytes are used to encode the size. The branch data that follows can include a mix of permissions nodes, pre-hashed nodes, blacklists, and even other branches. When processing a branch:

- The branch data is parsed recursively, with each nested node being processed according to its own flag.
- The leaf hashes of all nested nodes are computed.
- These individual hashes are then combined (e.g., using `LibOptim.fkeccak256`) to produce a single cumulative hash representing the entire branch.
- This branch hash is then integrated into the parent configuration's image hash, ensuring that all the nested information contributes to the final cryptographic fingerprint.

> [!TIP]
> Branch nodes are especially useful for modularizing the configuration structure. They allow logically related nodes to be grouped together, which not only improves organization but also potentially reduces the overall size of the calldata by allowing unused leaves to be rolled up into a single node.

#### Blacklist (FLAG 0x03)

The blacklist node specifies addresses that are disallowed for implicit sessions. This includes both target addresses that cannot be called and session signers that are not allowed to make implicit calls.

```
Blacklist Node Layout:
 ┌──────────────────────────────────────────────┐
 │ Flag Byte                                    │
 │   ┌────────────────────────────────────────┐ │
 │   │ Bits 7..4: FLAG (0x03)                 │ │
 │   │ Bits 3..0: Blacklist count or 0x0F     │ │
 │   └────────────────────────────────────────┘ │
 │ [Optional] Extended Count (uint16)           │
 │ Blacklisted Addresses (sorted array)         │
 └──────────────────────────────────────────────┘
```

The blacklist count is encoded in the additional data portion of the flag byte (bits 3..0):

- If the count is 14 or less, it is stored directly in these bits
- If the count is 15 or more, these bits are set to 0x0F and the actual count is stored in the next 2 bytes as a uint16

The blacklist serves two security purposes:

1. Prevents implicit sessions from calling specific target addresses
2. Blocks specific session signers from making any implicit calls

When an implicit session call is made, both the session signer and the target address are checked against the blacklist. If either appears in the blacklist, the call will be rejected with a `BlacklistedAddress` error.

> [!IMPORTANT]
> For implicit sessions, the blacklist is mandatory. The blacklist addresses must be sorted or validation will fail. This is to allow a binary search during validation.

> [!WARNING]
> The blacklist doesn't not prevent explicit sessions from calling blacklisted addresses or prevent explicit signers. To block an explicit session or it's permissions, update the wallet configuration to remove the explicit session.

#### Identity Signer (FLAG 0x04)

Specifies the identity signer used for attestation verification:

```
Identity Signer Layout:
 ┌──────────────────────────────┐
 │ Flag Byte                    │
 │   ┌────────────────────────┐ │
 │   │ Bits 7..4: FLAG (0x04) │ │
 │   │ Bits 3..0: Unused      │ │
 │   └────────────────────────┘ │
 │ Identity Signer (address)    │
 └──────────────────────────────┘
```

> [!IMPORTANT]
> The configuration must include exactly one identity signer during validation. Duplicate or missing entries trigger an error.

> [!NOTE]
> An Identity Signer can be any address capable or authorizing an implicit session. This should not be confused with other uses of the term Identity outside the sessions extension.

---

### 2. Attestation List Encoding

After reading the session configuration, a single byte `attestationCount` indicates how many attestations follow:

```
┌─────────────────────────────────────────────────────┐
│ uint8 attestationCount                              │
│ ┌────────────────────────────────────────────────┐  │
│ │ Attestation + identity signature               │  │
│ │ ... repeated attestationCount times ...        │  │
│ └────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

Each attestation is encoded as described in [Attestation (Implicit Sessions)](#attestation-implicit-sessions) below, then followed by a single identity signature from the configured identity signer (in EIP-2098 compact form).

If `attestationCount > 0` but no blacklist node was present in the configuration, validation fails.

---

### 3. Call Signatures Encoding

Each call in the payload is accompanied by a call signature. The encoding differs slightly for explicit sessions and implicit sessions.

#### Call Signature Structure

```
Call Signature Layout:
 ┌─────────────────────────────────────────────────────────────┐
 │ Flag Byte                                                   │
 │   ┌────────────────────────────────────────────────────────┐│
 │   │ Bit 7 (0x80): isImplicit flag                          ││
 │   │ Bits 6..0 (0x7F): If implicit, this is the attestation ││
 │   │               index; if explicit, this is the          ││
 │   │               session permission index                 ││
 │   └────────────────────────────────────────────────────────┘│
 │ Session Signature (EIP-2098 compact: see below)             │
 └─────────────────────────────────────────────────────────────┘
```

> [!IMPORTANT]
> The flag byte is critical for distinguishing call types. For implicit sessions, the most significant bit (Bit 7) must be set. For explicit sessions, the lower 7 bits represent the permission index.

The session signature is an ECDSA signature of the call and replay protection information in the payload (nonce, space and chainId).

No attestation data is embedded here for implicit calls; instead, each implicit call references an attestation by index from the Attestation List.

#### EIP-2098 Compact Signature Encoding

Compact signatures follows the [EIP-2098](https://eip.tools/eip/2098) compact signature format. In this format, the signature is encoded as follows:

```
EIP-2098 Compact Encoding:
 ┌─────────────────────────────────────────────────────────────┐
 │ 256-bit r value                                             │
 │ 1-bit yParity (encoded into s)                              │
 │ 255-bit s value                                             │
 └─────────────────────────────────────────────────────────────┘
```

This encoding merges the `v` value into the `s` value, reducing the overall signature size while maintaining full signature recovery capability.

---

## Permissions System

The permissions system governs what actions a session signer is allowed to execute within explicit sessions. It is designed to be flexible, allowing validations on any field within the call data through the use of **value**, **offset**, and **mask** parameters.

### Session Permissions (Explicit Sessions)

Defined in the `SessionPermissions` struct, these include:

- **Signer:** Authorized session signer.
- **Value Limit:** Maximum native token value allowed.
- **Deadline:** Expiration timestamp (0 indicates no deadline).
- **Permissions Array:** List of permission objects.

> [!WARNING]
> If a session's deadline is set and the current block timestamp exceeds it, the session is considered expired and all calls will be rejected.

### Permission Object and Parameter Rules

Each permission object specifies a target contract and a set of rules that define acceptable call parameters.

#### Permission Object Recap

```
Permission Object:
 ┌────────────────────────────────┐
 │ Target Address                 │
 │ Rules Count (uint8)            │
 │ Rules (array of ParameterRule) │
 └────────────────────────────────┘
```

#### Parameter Rule Recap

```
Parameter Rule:
 ┌────────────────────────────────────────────────────────────┐
 │ Operation & Cumulative Flag (1 byte)                       │
 │   ┌──────────────────────────────────────────────────────┐ │
 │   │ Bits 7..1  (0xFE): Operation (e.g., 0 = EQUAL, etc.) │ │
 │   │ Bit 0 (0x01): Cumulative flag (1 = cumulative)       │ │
 │   └──────────────────────────────────────────────────────┘ │
 │ Value (bytes32)                                            │
 │ Offset (uint256)                                           │
 │ Mask (bytes32)                                             │
 └────────────────────────────────────────────────────────────┘
```

> [!TIP]
> A permission with an empty rules array is treated as _open_, granting unrestricted access to the target, subject only to other constraints such as value limits and deadlines.

---

## Detailed Permission Rules and Validation

The permission rules mechanism provides a powerful and flexible method to validate any field within the call data. Here's a detailed look at how the rules work:

### Components of a Permission Rule

- **Value:**  
  The expected value (stored as a `bytes32`) used for comparison.

- **Offset:**  
  The byte offset in the call data from which the 32-byte parameter is extracted.

- **Mask:**  
  A bitmask applied to the extracted data. This isolates the relevant bits, allowing validation even when the field is embedded within a larger data structure.

### Validation Process

For each rule, the validation function performs the following steps:

1. **Extraction:**  
   Read 32 bytes from the call data starting at the specified offset:

   ```solidity
   bytes32 extracted_value = call.data.readBytes32(rule.offset);
   ```

2. **Masking:**  
   Apply the mask to isolate the target bits:

   ```solidity
   bytes32 masked_value = extracted_value & rule.mask;
   ```

3. **Comparison:**  
   Compare the masked value with the expected value using the defined operation:
   - **EQUAL:** The masked value must exactly equal the expected value.
   - **LESS_THAN_OR_EQUAL:** The masked value must be less than or equal to the expected value.
   - **GREATER_THAN_OR_EQUAL:** The masked value must be greater than or equal to the expected value.
   - **NOT_EQUAL:** The masked value must not equal the expected value.

> [!TIP]
> This approach allows validation on any field within the call data regardless of its format or position.

### The Cumulative Flag

When the **cumulative** flag is set on a permission rule:

1. **Cumulative Calculation:**  
   The value extracted from the current call is added to a previously recorded usage amount (stored in the payload's usage limits or persistent storage).

2. **Threshold Comparison:**  
   The cumulative total (current value plus previous usage) is compared against the threshold defined by the rule.

3. **Preceding Update:**  
   Because cumulative values persist across multiple calls, a preceding call to `incrementUsageLimit` is required. This call updates the on-chain storage with the new cumulative total, ensuring that future validations reflect the updated usage.

> [!WARNING]
> Cumulative usage is tracked using hashes: `keccak256(abi.encode(signer, permission, ruleIdx))` for rules and `keccak256(abi.encode(signer, VALUE_TRACKING_ADDRESS))` for native tokens. Modifying a permission creates a new hash, so the old usage state must be considered when modifying a permission.

### Example: ERC20.transfer

Consider an ERC20 token `transfer` function:

```solidity
function transfer(address to, uint256 amount) returns (bool);
```

**Call Data Layout:**

- **4 bytes:** Function selector.
- **32 bytes:** Encoded `to` address.
- **32 bytes:** Encoded `amount`.

**Permission Rule Setup:**

- **Target:**  
  The ERC20 token contract address.

- **Offset:**  
  `36` bytes (4 bytes for the function selector + 32 bytes for the `to` address) - this is where the `amount` parameter begins.

- **Mask:**  
  A full mask (`0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF`) to extract the entire 32-byte value.

- **Value:**  
  `100 * 10^18` (expressed as a `bytes32` value) to represent a maximum transfer amount of 100 tokens (assuming 18 decimals).

- **Operation:**  
  `LESS_THAN_OR_EQUAL` - ensuring the transfer amount does not exceed the threshold.

- **Cumulative Flag (optional):**  
  If you want to enforce a cumulative limit (e.g., a daily cap), set the cumulative flag. Each transfer's amount is then added to a cumulative total that must not exceed the threshold, with an `incrementUsageLimit` call required to update the stored value.

> [!NOTE]
> ERC20.transfer Example Recap: The permission rule extracts the `amount` parameter from call data at offset 36, applies a mask to isolate the full value, and verifies that the value is less than or equal to 100 \* 10^18. Optionally, if the cumulative flag is set, it enforces a cumulative limit across multiple calls. In practice, the permission would also include a rule to check the function selector, ensuring that the call is to the `transfer` function.

---

## Attestation (Implicit Sessions)

Implicit sessions use an attestation to verify that the session signer is approved. The attestation is encoded and then validated by the target contract.

### Attestation Encoding

```
Attestation Encoding:
 ┌──────────────────────────────────────────────┐
 │ Approved Signer (address)                    │
 │ Identity Type (bytes4)                       │
 │ Issuer Hash (bytes32)                        │
 │ Audience Hash (bytes32)                      │
 │ Application Data Length (uint24)             │
 │ Application Data (variable bytes)            │
 │ Redirect URL Length (uint24)                 │
 │ Redirect URL (variable string)               │
 │ Issued At (uint64)                           │
 └──────────────────────────────────────────────┘
```

The Attestation data obtained during authentication. The `Identity Type` is the type of identity that was used to authenticate the user. The `Issuer Hash` is the hash of the issuer. The `Audience Hash` is the hash of the audience. The `Application Data` can be provided by the dapp. The `Auth Data` contains the redirect URL (string) and issuance timestamp (uint64).

> [!WARNING]
> The `Application Data` length is encoded using a `uint24`. Ensure that data lengths are within these limits.

> [!NOTE]
> The `Redirect URL` length is encoded using a `uint24`, and the `Issued At` field is a `uint64` timestamp representing when the attestation was issued. The encoding order is: `redirectUrlLength` (uint24), `redirectUrl` (string), `issuedAt` (uint64).

### Attestation Validation

- The attestation's **approved signer** must match the session signer.
- A magic value is generated using a combination of a prefix, the wallet address, the attestation's audience hash, and issuer hash.
- The attestation signature is validated against the identity signer from the configuration.
- The target contract's `acceptImplicitRequest` function must return the expected magic value; otherwise, the call is rejected.

> [!WARNING]
> Implicit sessions require a properly encoded blacklist in the configuration. Calls to a blacklisted address will be rejected, and missing blacklist data will cause validation errors.

---

## Future Improvements

Several improvements can be made

> [!NOTE]
> Configuration Flexibility: Introduce versioning or additional flags in the configuration encoding to support new features while preserving backward compatibility. Allow dynamic adjustments without breaking the merkle tree-based image hash structure.

> [!NOTE]
> Gas Optimization: Optimize the recursive encoding/decoding logic for configurations with a large number of permissions or deep branch nesting to reduce gas costs.

> [!NOTE]
> Call Signature Optimization: Optimize the call signature encoding to reduce the size of the calldata. A potential target for optimization is to remove repeated encodings of the same attestation data.

> [!NOTE]
> Advanced Permission Rules: Extend the permission system to support more complex conditional checks or dynamic rule adjustments. Provide improved error messages and diagnostic tools for failed validations.

---

## Conclusion

The smart sessions system in Ecosystem wallets offers a flexible framework for authorizing batched operations via signed payloads. By leveraging detailed encoding schemes for configuration, permissions, and attestations - and by deriving an image hash that cryptographically fingerprints the configuration tree (even when sparse) - the system supports both explicit sessions and implicit sessions while ensuring robust validation. The detailed permission rules, including the use of **value**, **offset**, and **mask**, provide granular control over call data validation, and the cumulative flag facilitates persistent limits across calls. The outlined future improvements aim to enhance security, efficiency, and usability as the system evolves.

This documentation serves as a technical guide for developers integrating and extending the smart sessions framework, providing both detailed encoding breakdowns and practical considerations for deployment and further development.

# **Technical Document: Sequence Signature Encoding**

This document describes, in detail, how the Sequence signature encoding is structured, how each part is packed into bytes, and how chained signatures and top-level signatures work. It is purely technical, explaining the bit layouts, flags, and usage with examples.

---

## **1. Overview**

Sequence uses a specialized signature format that:

1. Has a **top-level signature** that includes:

   - A single "signature flag" byte that encodes multiple fields (checkpointer usage, signature type, checkpoint size, threshold size, etc.).
   - (Optionally) data related to an on-chain checkpointer contract.
   - A notion of _chained signatures_ vs. _normal signatures_ vs. _“no chain id”_ signatures.
   - A final threshold and checkpoint value that tie into the overall wallet or contract logic.

2. Contains a **merkle-like structure** for the signers at the “branch” level, where each “leaf” or “node” is encoded using a separate mini-flag nibble. This is parsed with a loop in the `recoverBranch` function.

3. Supports multiple sub-signature types, such as ECDSA (`FLAG_SIGNATURE_HASH` or `FLAG_SIGNATURE_ETH_SIGN`), ERC-1271 contract-based checks, nested “multi-sig inside multi-sig,” and special “sapient” signatures. Each sub-signature or branch piece is prefixed by one byte: the top nibble is the “flag type” and the bottom nibble contains per-flag configuration bits (like weight, sizes, or additional bits for `v`).

---

## **2. Top-level Signature Format**

When `recover` is first invoked, it reads the **first byte** of the signature as `signatureFlag`. That byte is bit-packed as follows (with bit `0` as the least-significant bit):

```
 ┌─────────────── Bit 7 (0x80) : Static signature
 │ ┌───────────── Bit 6 (0x40) : Checkpointer usage flag
 │ │ ┌─────────── Bit 5 (0x20) : Threshold size indicator (0 => 1 byte, 1 => 2 bytes)
 │ │ │  ┌──────── Bits 4..2 (0x1C) : Checkpoint size (encoded as an integer 0..7)
 │ │ │  │  ┌───── Bit 1 (0x02) : "no chain id" signature type
 │ │ │  │  │ ┌─── Bit 0 (0x01) : "chained" signature type
[7 6 5 432 1 0]
```

We can break this down more concretely:

1. **Bit 7** set (`0x80`) indicates a static signature:
   - When set, the signature has been pre-stored in contract storage and bypasses normal validation
   - Validation only checks:
     - That the stored expiry timestamp has not passed
     - That the stored signer matches the transaction sender (or is unset with `address(0)`)
1. **Bit 6** set (`0x40`) means the signature includes an external **imageHash checkpointer**:
   - If set, the signature will contain:
     - The checkpointer contract `address`
     - A 3-byte length for the “checkpointer data”
     - That data, passed to `ICheckpointer(checkpointer).snapshotFor(...)`
1. **Bits 4..2** (the field `((signatureFlag & 0x1c) >> 2)`) define the **checkpoint size** in bytes. Possible values are `0..7`. If this value is `N`, then the next `N` bytes of the signature after reading the flag (and optional checkpointer data) represent the **checkpoint**.
1. **Bit 5** (`0x20`) sets how many bytes are used to read the threshold. If it is `0`, the threshold is read as 1 byte; if it is `1`, the threshold is read as 2 bytes. (Hence `( (signatureFlag & 0x20) >> 5 ) + 1`.)
1. **Bit 1** (`0x02`) indicates the “no chain id” signature. If set, `_payload.noChainId` is true. This affects how `_payload.hash()` is computed.
1. **Bit 0** (`0x01`) indicates the signature is **chained**. In that case, the code calls `recoverChained`, which processes multiple sub-signatures in sequence.

Putting it together:

- If bit `0` is set, we do a **chained** approach: The signature is composed of chunks, each chunk specifying a length and then a nested signature.
- Otherwise, we do a “regular” top-level parse: we read the checkpoint size, threshold size, then parse the “branch” for signers.

### **Example of a Top-level Signature Byte**

Suppose the top-level `signatureFlag` is `0x74` in hex. Converting `0x74` to binary:

```
0x74 = 01110100 in binary
        ^ ^ ^ ^
bit 7:  0 (reserved)
bit 6:  1 => checkpointer usage
bit 5:  1 => threshold uses 2 bytes
bits 4..2: 101 => checkpoint size = 5 bytes
bit 1:  0 => normal (not "no chain id")
bit 0:  0 => not chained
```

From this:

- We first read an `address` for the checkpointer, then read 3 bytes for the checkpointer data length, etc.
- We know we must parse **5 bytes** for the checkpoint value.
- Then parse **2 bytes** for the threshold.
- Then parse the remainder as the merkle-branch structure for signers.

---

## **3. Chained Signatures**

When **bit 0** is set (the least-significant bit), the signature is **chained**. Instead of the usual approach (parsing threshold, checkpoint, etc. from that same byte), the code calls:

```solidity
recoverChained(_payload, snapshot, _signature);
```

A chained signature is a series of **signature chunks**, each chunk defined like this:

```
[3-byte length] [chunk of that length]
[3-byte length] [chunk of that length]
...
```

Each chunk can itself be a top-level signature in the sense that it calls `recover(...)` again—except it ignores checkpointer details after the first chunk. The code enforces:

- Each chunk recovers `(threshold, weight, imageHash, checkpoint)`.
- If `weight < threshold`, it reverts with `LowWeightChainedSignature`.
- The `checkpoint` must be **strictly less** than the previous chunk’s `checkpoint`, ensuring correct ordering (`WrongChainedCheckpointOrder`).
- All but the first chunk are interpreted as a “configuration update” with a special “linkedPayload.”

This allows multiple signature instructions to be “chained” in a single byte array.

```
   0           1           2           3
   |----- byte indices: 0..2 => 3-byte length L1
   |----- next L1 bytes => chunk #1
            ...
   |----- next 3 bytes => length L2
   |----- next L2 bytes => chunk #2
            ...
   |----- next 3 bytes => length L3
   |----- next L3 bytes => chunk #3
            ...
   <end of signature>
```

Each chunk is itself a “top-level style signature” minus the repeated checkpointer usage. The final `(threshold, weight, imageHash, checkpoint)` from the last chunk can be used to validate the overall signature.

---

## **4. Branch-Level Parsing**

Regardless of whether it is a chained signature or a direct one, eventually the code calls:

```solidity
recoverBranch(_payload, opHash, _signature)
```

This function loops over the remainder of the signature, reading one byte at a time as the “header” for a sub-signature or branch item. We’ll call that one byte `firstByte`. The code extracts:

- `flag = (firstByte & 0xf0) >> 4;` (the top nibble)
- The lower nibble is used as “free bits.”

### **4.1 Flag Values**

The contract defines constants:

| Constant Name                          | Value (Decimal) | Purpose                                                                                               |
| -------------------------------------- | --------------- | ----------------------------------------------------------------------------------------------------- |
| `FLAG_SIGNATURE_HASH`                  | 0               | ECDSA signature with `r,yParityAndS` (ERC-2098 compact) directly against `_opHash`.                   |
| `FLAG_ADDRESS`                         | 1               | Just an address “leaf” (with no actual ECDSA check)                                                   |
| `FLAG_SIGNATURE_ERC1271`               | 2               | A contract-based signature check using `isValidSignature(opHash, signature)`                          |
| `FLAG_NODE`                            | 3               | Includes a raw 32-byte node hash in the merkle root. No weight added.                                 |
| `FLAG_BRANCH`                          | 4               | Nested branch. The next bytes specify length, then recursion into `recoverBranch`.                    |
| `FLAG_SUBDIGEST`                       | 5               | Hard-coded “accepted subdigest.” If `_opHash` matches the stored 32 bytes, infinite weight.           |
| `FLAG_NESTED`                          | 6               | A nested multi-sig node with an internal threshold plus an external weight.                           |
| `FLAG_SIGNATURE_ETH_SIGN`              | 7               | ECDSA signature in “Eth_sign” format (`"\x19Ethereum Signed Message:\n32" + opHash`), using ERC-2098. |
| `FLAG_SIGNATURE_ANY_ADDRESS_SUBDIGEST` | 8               | `FLAG_SUBDIGEST` but with counter factual support.                                                    |
| `FLAG_SIGNATURE_SAPIENT`               | 9               | A specialized “sapient” signature with an `ISapient` contract check.                                  |
| `FLAG_SIGNATURE_SAPIENT_COMPACT`       | 10              | A specialized “sapient” signature with `ISapientCompact` and `_opHash` only.                          |

When the parser sees `flag == someValue`, it dispatches to the corresponding block. Each block interprets the lower nibble differently.

---

## **5. Detailed Flag-by-Flag Format**

Below are the internal mini-formats for each **flag**. Recall that in code, `firstByte` is the single byte at the start of each item, and we do:

```
flag = (firstByte & 0xf0) >> 4;      // top nibble
// "free nibble" = (firstByte & 0x0f)
```

Each bullet will show how the bits in the “free nibble” are used.

---

### 5.1 **Signature Hash** (`flag = 0`)

- Uses **ERC-2098** to parse the signature in 64 bytes (`r` + `yParityAndS`).
- The free nibble bits [3..0] define the signer's weight (0 => we read the weight from the next byte, else 1..15).
- After reading `r` (32 bytes) and `yParityAndS` (32 bytes), the top bit of `yParityAndS` (bit 255) is `yParity` (0 or 1), which is added to 27 to form `v`. The remaining 255 bits are `s`.
- We then perform `ecrecover(_opHash, v, r, s)`.

**Example**  
If the sub-signature byte is `0x05` (`0000 0101` in binary), then top nibble=0 => `FLAG_SIGNATURE_HASH`, free nibble=5 => weight=5. We do **not** read an extra byte for the weight. Next, we read 64 bytes as the compact signature: 32 bytes for `r`, 32 bytes for `yParityAndS`. If the top bit of `yParityAndS` is 0 => `v=27`; if it is 1 => `v=28`. The rest is `s`. Then we do `ecrecover`.

---

### 5.2 **Address** (`flag = 1`)

- Takes an address leaf (no ECDSA).
- The free nibble bits 3..0 define the weight in the same scheme:
  - If those bits are zero, read an extra byte for weight.
  - Else use that 1..15 as the weight.
- Then reads 20 bytes for the address.
- Merges `_leafForAddressAndWeight(addr, weight)`.

---

### 5.3 **Signature ERC-1271** (`flag = 2`)

- The free nibble bits are used as:
  - The bottom two bits are the weight (with the same “0 => dynamic read, else 1..3” logic).
  - The next two bits define the size of the “signature size” field: 0..3 means we read 0..3 bytes to get the dynamic length of the next part.
- Then we read 20 bytes for the contract address, read that dynamic-size signature, and call `IERC1271(addr).isValidSignature(_opHash, data)`. If it returns the magic value `0x1626ba7e`, it is valid; otherwise revert.
- Weight is added if valid.

**Example**

```
firstByte = 0x2D  ->  0010 1101 in binary
 top nibble = 2 -> FLAG_SIGNATURE_ERC1271
 free nibble = 0xD = 1101 in binary
 bits 3..2 = 11 -> sizeSize=3 => read 3 bytes to get length
 bits 1..0 = 01 -> weight=1
```

Then parse next 3 bytes to discover how big the signature is, read it, do the 1271 check.

---

### 5.4 **Node** (`flag = 3`)

- No free bits used.
- Simply reads a 32-byte “node hash” and merges it.
- No weight is added.

---

### 5.5 **Branch** (`flag = 4`)

- The free nibble bits 3..0 define how many bytes are used to read the upcoming “branch size.”
- Once the branch size is read, we extract that many bytes as a sub-branch, and recursively call `recoverBranch` on that sub-slice.
- We get `(nweight, nodeHash)` from that sub-branch, add `nweight` to the total, and merge the nodeHash into the root.

---

### 5.6 **Subdigest** (`flag = 5`)

- The code reads a 32-byte “hardcoded subdigest.” If it matches the `_opHash`, sets `weight = type(uint256).max`.
- Merges `_leafForHardcodedSubdigest(hardcoded)`.

This effectively means “if the 32 bytes match the current operation hash, we grant infinite weight.”

---

### 5.7 **Nested** (`flag = 6`)

- The free nibble is split:
  - The bottom two bits define the “external weight.” Again, `0 => read from next byte, else 1..3`.
  - The next two bits define the “internal threshold” size. If `0`, read 2 bytes from the next portion for that threshold, else 1..3 is just 1..3?
  - Then read 3 bytes to get the length of the nested sub-branch, parse it. That yields `(internalWeight, internalRoot)`.
  - If `internalWeight >= internalThreshold`, we add the external weight to the total. Finally, we merge `_leafForNested(internalRoot, internalThreshold, externalWeight)` into the root.

**Example**

```
firstByte = 0x64  ->  0110 0100 in binary
 top nibble = 6 -> FLAG_NESTED
 free nibble = 0x4 = 0100 in binary
   bits 3..2 = 01 -> internalThreshold=1
   bits 1..0 = 00 -> externalWeight => read from next byte
```

Then read next byte for externalWeight, read next 2 bytes for threshold if needed, etc.

---

### 5.8 **Signature ETH Sign** (`flag = 7`)

- Similar to `FLAG_SIGNATURE_HASH`, but recovers via:

```
ecrecover( keccak256("\x19Ethereum Signed Message:\n32" + _opHash), v, r, s )
```

- Uses **ERC-2098**: we read 64 bytes (32 for `r`, 32 for `yParityAndS`), retrieve `yParity` from the top bit, add 27 to form `v`, and use the remainder as `s`.
- The free nibble bits [3..0] define the weight (0 => dynamic read, else 1..15).

---

### 5.9 **Signature Any Address Subgiest** (`flag = 8`)

- The code reads a 32-byte "hardcoded subdigest." If it matches `_payload.hashFor(address(0))`, sets `weight = type(uint256).max`.
- Merges `_leafForAnyAddressSubdigest(anyAddressOpHash)`.

This effectively means "if the 32 bytes match the operation hash computed for address(0), we grant infinite weight." This allows for counter-factual payloads.

---

### 5.10 **Signature Sapient** (`flag = 9`)

- The free nibble is structured like `ERC1271`: some bits define how many bytes to read for the signature, some bits define the weight.
- Then it calls `ISapient(addr).recoverSapientSignature(_payload, data)`, which must return a “sapientImageHash” used in `_leafForSapient`.
- Weight is added if valid.

---

### 5.11 **Signature Sapient Compact** (`flag = 10`)

- Same approach as `FLAG_SIGNATURE_SAPIENT`, except the contract uses `ISapientCompact.recoverSapientSignatureCompact(_opHash, data)` instead, passing only `_opHash`.

---

## **6. Merkle Root Construction**

The branch parser accumulates a “root” by repeatedly combining leaves with the function:

```solidity
root = LibOptim.fkeccak256(root, leaf)
```

In each sub-flag block, a leaf is computed, for example:

- `_leafForAddressAndWeight(address, weight)`
- `_leafForNested(internalRoot, threshold, externalWeight)`
- `_leafForHardcodedSubdigest(someDigest)`
- etc.

The final `root` is combined with the threshold and checkpoint (and checkpointer address, if present) to yield the final “imageHash.” That is used to tie the signatures to a specific configuration or permission set.

---

## **7. Example Putting it All Together**

Below is a hypothetical top-level signature that is **not** chained, uses a checkpointer, has a 2-byte threshold, a 1-byte checkpoint, and then includes a single ECDSA leaf:

1. **signatureFlag** = `0x6C` => in binary `0110 1100`
   - Bit 6 => `1`, so we have a checkpointer
   - Bit 5 => `1`, threshold uses 2 bytes
   - Bits 4..2 => `110` => checkpoint size = 6 bytes
   - Bit 1 => `0`, normal chain id usage
   - Bit 0 => `0`, not chained
2. We read:
   - `checkpointer` address (20 bytes)
   - 3-byte checkpointer data size => parse that data
   - 6 bytes => the “checkpoint” number
   - 2 bytes => the threshold
3. We jump into `recoverBranch`, and the next 1-byte might be `0x02` in hex => top nibble=0 => `FLAG_SIGNATURE_HASH` with free nibble=2 => weight=2. Then parse 64 bytes for ERC-2098. We derive `v` from the top bit of the second 32 bytes, do ecrecover, and merge the address in the merkle root.

Finally, the code compares the final computed image hash, checks if we pass threshold vs. weight, checks snapshot logic, and returns `(threshold, weight, imageHash, checkpoint)`.

---

## **8. Snapshot and Checkpointer Logic**

If the top-level byte indicates we have a checkpointer (`bit 6` set), we read:

- The checkpointer’s address
- The next 3 bytes => `checkpointerDataSize`
- That many bytes => `checkpointerData`

We call:

```solidity
snapshot = ICheckpointer(checkpointer).snapshotFor(address(this), checkpointerData);
```

This yields a `Snapshot { imageHash, checkpoint }`. If the final signature’s computed `imageHash` and `checkpoint` do not properly exceed or match the snapshot, the code can revert with `UnusedSnapshot`.

---

## **9. Summary**

1. **Top-level “signatureFlag”** byte sets the “checkpointer usage,” “signature type,” “checkpoint size,” “threshold size,” etc.
2. If the signature is **chained**, parse a series of sub-signatures, each of which in turn calls the normal `recover`.
3. Eventually, a **branch** parse is done with `recoverBranch`, which looks at many items. Each item is marked by a single byte whose **top nibble** identifies the flag (ECDSA, ERC1271, sub-branch, nested multi-sig, etc.), and whose **bottom nibble** has special bits (like the signers’ weight, or signature-size format).
4. The final output is `(threshold, weight, imageHash, checkpoint)` plus snapshot checks if any.

This structure allows advanced multi-signature logic, nested multi-sigs, infinite weight if a known subdigest matches `_opHash`, and optional checkpointer extension.




 ## PACKAGE.JSON HEADERS OF LIB PACKAGES: 

 Note: Check for important lib version info

 
 When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.

 ### lib/forge-std/package.json

{
  "name": "forge-std",
  "version": "1.9.7",
  "description": "Forge Standard Library is a collection of helpful contracts and libraries for use with Forge and Foundry.",
  "homepage": "https://book.getfoundry.sh/forge/forge-std",
  "bugs": "https://github.com/foundry-rs/forge-std/issues",
  "license": "(Apache-2.0 OR MIT)",
  "author": "Contributors to Forge Standard Library",

### lib/account-abstraction/contracts/package.json

{
  "name": "@account-abstraction/contracts",
  "description": "Account Abstraction (EIP 4337) contracts",
  "version": "0.7.0",
  "scripts": {
    "prepack": "../scripts/prepack-contracts-package.sh",
    "postpack": "../scripts/postpack-contracts-package.sh"
  },

### lib/account-abstraction/package.json

{
  "name": "accountabstraction",
  "version": "0.7.0",
  "description": "ERC-4337 Account Abstraction Implementation",
  "scripts": {
    "clean": "rm -rf cache artifacts typechain typechain-types",
    "compile": "./scripts/hh-wrapper compile",
    "tsc": "tsc",

### lib/openzeppelin-contracts/contracts/package.json

{
  "name": "@openzeppelin/contracts",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.2.0",
  "files": [
    "**/*.sol",
    "/build/contracts/*.json",
    "!/mocks/**/*"

### lib/openzeppelin-contracts/package.json

{
  "name": "openzeppelin-solidity",
  "description": "Secure Smart Contract library for Solidity",
  "version": "5.2.0",
  "private": true,
  "files": [
    "/contracts/**/*.sol",
    "!/contracts/mocks/**/*"

### lib/openzeppelin-contracts/scripts/solhint-custom/package.json

{
  "name": "solhint-plugin-openzeppelin",
  "version": "0.0.0",
  "private": true
}


 ## CONFIG FILES: 

 Note: Check for important package version info.

 ### foundry.toml

[profile.default]
src = "src"
out = "out"
libs = ["lib"]
via-ir = true
optimizer = true
optimizer_runs = 4294967295
fs_permissions = [{ access = "read-write", path = "/tmp"}]
solc = "0.8.28"
evm_version = "paris"
remappings = [
  "account-abstraction/=lib/account-abstraction/contracts/"
]

[fuzz]
max_test_rejects = 1000000

[fmt]
# Explicitly define all formatting rules
multiline_func_header = "params_first"
single_line_statement_blocks = "multi"
sort_imports = true
contract_new_lines = true
override_spacing = false
line_length = 120
tab_width = 2
bracket_spacing = true
int_types = "long"
quote_style = "double"
hex_underscore = "remove"
wrap_comments = false


### .env.sample

PRIVATE_KEY=
ERC4337_ENTRY_POINT_V7=0x0000000071727De22E5E9d8BAf0edAc6f37da032

SEQ_SDK_RPC_URL_PREFIX="http://localhost:"
SEQ_SDK_RPC_URL_SUFFIX="/rpc"
SEQ_SDK_RPC_MIN_PORT=9999
SEQ_SDK_RPC_MAX_PORT=9999


### package.json

{
  "devDependencies": {
    "lefthook": "^1.6.1",
    "prettier": "^3.2.5"
  },
  "scripts": {
    "postinstall": "lefthook install",
    "format:prettier": "prettier --write \"**/*.{js,jsx,ts,tsx,json,md,yml,yaml}\"",
    "format:forge": "forge fmt",
    "test": "forge test",
    "coverage": "forge coverage --no-match-coverage \"(script|test)\""
  }
}


### remappings.txt

account-abstraction/=lib/account-abstraction/contracts/


