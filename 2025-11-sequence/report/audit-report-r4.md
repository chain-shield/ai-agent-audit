# 2025 11 sequence - Findings Report
## Commit hash: 593987c2b569f3212d2cadb31fc8d14b84bc30a9

USING MULTI_PATTERN_TO_FINDING_ANALYSIS_MODE = false; and gpt-5
NICHE_PATTERN_ANALYSIS_MODE = false;

##Findings by Pattern
2 or 3 UNIQUES


 **Derived From** : Tstorish fallback uses caller storage under delegatecall, risking sentinel DoS via TSTORE mis-selection

[M-1]. Delegatecall storage collision in Tstorish can brick validateOpHashAndSweep and sweeps on non‑TSTORE chains
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 8
Privilege: Permissionless



 **Derived From** : EIP-712 domain/struct chainId mismatch can brick intents after chainId change

[L-2]. EIP-712 chainId mismatch in TrailsIntentEntrypoint bricking deposits after chainId change
Finding Status: Valid
Status Confidence: Confident
Finding Complexity: 6
Privilege: Permissionless



 **Derived From** : Balance injection leaves persistent ERC20 allowance to arbitrary target

[H-3]. Persistent ERC20 allowance in TrailsRouter._injectAndExecuteCall lets arbitrary targets drain wallet balances
Finding Status: Valid
Status Confidence: VeryConfident
Finding Complexity: 4
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 1
- M: 1
- L: 1
- I: 0

##Findings by Pattern


 **Derived From** : Tstorish fallback uses caller storage under delegatecall, risking sentinel DoS via TSTORE mis-selection

## [M-1]. Delegatecall storage collision in Tstorish can brick validateOpHashAndSweep and sweeps on non‑TSTORE chains

### Finding Severity Justification: The issue can brick the sentinel-based success tracking and thus prevent validateOpHashAndSweep (and any flows depending on opHash success sentinels) from working correctly in certain environments. On pre‑TSTORE chains where Tstorish was wired to use the SSTORE/SLOAD fallback but the delegatecall context makes _tstoreSupport read as true, any attempt to use _getTstorish/_setTstorish will revert (invalid opcode) and effectively DoS the Trails flow for affected wallets on that chain. This impacts protocol availability and correct fee/sweep semantics, but does not directly allow theft or loss of user assets, so it aligns with Medium: protocol function and value flows are impacted without direct asset compromise.
## Derived From Pattern/Invariant
Tstorish fallback uses caller storage under delegatecall, risking sentinel DoS via TSTORE mis-selection

## Exploit Type
StorageLayout

## Location
TrailsRouter.validateOpHashAndSweep

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 8
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
TrailsRouter inherits Tstorish, a helper that decides at runtime whether to use transient storage (TSTORE/TLOAD) or normal storage (SSTORE/SLOAD) via a boolean flag `_tstoreSupport` stored in contract storage:

`bool private _tstoreSupport; // slot 0`

When TSTORE is not supported at deployment, Tstorish wires its internal function pointers to the fallback helpers:

`function _getTstorishWithSloadFallback(uint256 storageSlot) private view returns (uint256 value) {
    if (_tstoreSupport) {
        assembly { value := tload(storageSlot) }
    } else {
        assembly { value := sload(storageSlot) }
    }
}

function _setTstorishWithSstoreFallback(uint256 storageSlot, uint256 value) private {
    if (_tstoreSupport) {
        assembly { tstore(storageSlot, value) }
    } else {
        assembly { sstore(storageSlot, value) }
    }
}`

TrailsRouter is **never used directly**; it is always delegatecalled from a Sequence v3 wallet (via TrailsRouterShim). Under delegatecall, all `sload`/`sstore`/`tload`/`tstore` instructions in Tstorish operate on **the wallet’s storage**, not the router’s. This includes `_tstoreSupport`, which is compiled to storage slot 0. In a Sequence wallet, slot 0 is already used (e.g. for an owner/config field), so in practice `_tstoreSupport` in the delegatecall context is whatever non‑zero value lives at `wallet.slot0`.

The success sentinel check in TrailsRouter is:

`function validateOpHashAndSweep(bytes32 opHash, address _token, address _recipient)
    public
    payable
    onlyDelegatecall
{
    uint256 slot = TrailsSentinelLib.successSlot(opHash);
    if (_getTstorish(slot) != TrailsSentinelLib.SUCCESS_VALUE) {
        revert SuccessSentinelNotSet();
    }
    sweep(_token, _recipient);
}`

And TrailsRouterShim (per docs) sets the same sentinel using `_setTstorish(...)` in the **same wallet storage context** under delegatecall.

On a chain where TSTORE/TLOAD were *not* supported at TrailsRouter deployment time (e.g. a pre‑Cancun EVM or any EVM that hasn’t enabled EIP‑1153), Tstorish’s constructor sets `_getTstorish = _getTstorishWithSloadFallback` and `_setTstorish = _setTstorishWithSstoreFallback`. The intended lifecycle is:
- Initially: `_tstoreSupport == false` ⇒ only SSTORE/SLOAD are used.
- After a future hardfork adds TSTORE, an EOA calls `__activateTstore()` once on the router implementation to flip `_tstoreSupport` to `true` so the fallback helpers switch to TSTORE/TLOAD.

However, in the actual delegatecall integration:
- `_tstoreSupport` as seen by `_getTstorishWithSloadFallback` / `_setTstorishWithSstoreFallback` is **not** the router’s flag; it is `wallet.slot0`.
- Sequence wallets almost certainly store a non‑zero value at slot 0 (e.g., owner address, signature threshold), so `_tstoreSupport` will appear `true` in the router as soon as it is used via delegatecall.
- As a result, on a pre‑TSTORE chain, `_getTstorishWithSloadFallback` and `_setTstorishWithSstoreFallback` will execute `tload`/`tstore` under delegatecall, even though the EVM does not support these opcodes.

On such chains, **any call path that touches the success sentinel under delegatecall will revert** with an invalid opcode:
- The shim’s attempt to set the sentinel via `_setTstorish(...)` at the end of a successful operation will revert, rolling back the entire Trails route for that wallet.
- A later call to `validateOpHashAndSweep` will also revert when `_getTstorish(...)` tries to execute `tload`.

Because `validateOpHashAndSweep` is the normal success‑gated sweep entrypoint (and is only gated by `onlyDelegatecall`), this cross‑contract storage collision means:
- On any non‑TSTORE chain where TrailsRouter was deployed, *every* Sequence wallet whose slot 0 is non‑zero will find `validateOpHashAndSweep` (and any shim sentinel writes) permanently reverting.
- Fees and destination sweeps that rely on the sentinel can never complete for those wallets, effectively **bricking the Trails flows for that chain/wallet combination**.

Crucially, the router implementation cannot fix this at runtime: `__activateTstore()` is guarded by `OnlyDirectCalls` (requires `msg.sender == tx.origin`), so it cannot be called under delegatecall to synchronize the `_tstoreSupport` flag in the wallet’s storage. The flag seen by the fallback helpers is entirely determined by the wallet’s own storage layout, which is outside the router’s control.

Even on chains that support TSTORE today, this design is extremely brittle: if the router was deployed before the hardfork and `_tstoreInitialSupport` is `false`, any change to `wallet.slot0` (e.g., owner rotation) can silently flip the sentinel semantics from SSTORE to TSTORE or vice versa for that wallet, causing difficult‑to‑debug, wallet‑specific breakage of `validateOpHashAndSweep`.

## Impact
There are two distinct but related impacts stemming from Tstorish’s use of the `_tstoreSupport` flag in a contract that is designed to be used only via `delegatecall`:

1. **Hard DoS on chains without TSTORE/TLOAD at router deployment time**
   - On a chain where TSTORE/TLOAD are *not* supported when `TrailsRouter` is deployed, Tstorish detects lack of support and wires `_getTstorish` / `_setTstorish` to the `*_WithSload/SstoreFallback` helpers.
   - Under `delegatecall` from a Sequence wallet, all `sload/sstore` operations in Tstorish operate on the *wallet’s* storage. The `_tstoreSupport` flag is stored at compiler slot 0, which under delegatecall resolves to `wallet.slot0`, not the router’s own storage.
   - If `wallet.slot0` is non-zero (which is very likely in a real wallet), `_tstoreSupport` will read as `true` inside the fallback helpers, so they will unconditionally execute `tload` / `tstore` even though the underlying chain does not support those opcodes.
   - Any code path that uses `_getTstorish` or `_setTstorish` under `delegatecall` will then hit an invalid opcode and revert. In particular, the Trails shim’s sentinel writes and the router’s `validateOpHashAndSweep` will be unusable for affected wallets, causing a *permanent DoS* of Trails flows that rely on the success sentinel on those chains.

2. **Semantic breakage on TSTORE-enabled chains due to delegatecall storage collision**
   - On chains where TSTORE is supported at deployment, `_getTstorish`/`_setTstorish` are wired directly to the pure TSTORE/TLOAD versions and do not consult `_tstoreSupport`, so the invalid-opcode DoS does not occur.
   - However, on chains where TSTORE was *not* supported at deployment but becomes available later (or on chains where the team intends to use `__activateTstore()`), the design becomes brittle: the decision to use TSTORE vs SSTORE depends on `_tstoreSupport` which, under `delegatecall`, is again read from `wallet.slot0` instead of a dedicated namespace.
   - Different wallets (or changes to their storage layout, e.g., owner rotation) can therefore flip `_tstoreSupport` between `false` and `true` in a way the router cannot control, leading to per-wallet divergence where some wallets use SSTORE for sentinels while others unexpectedly use TSTORE/transient storage.
   - This can cause situations where a sentinel is written using SSTORE semantics (persistent storage), while subsequent reads use TLOAD semantics (transient storage), or vice versa, making `validateOpHashAndSweep` incorrectly revert with `SuccessSentinelNotSet()` even though the operation logically succeeded. This is a protocol-level correctness and availability issue for Trails routes and fee/sweep semantics and can manifest in a wallet-specific, non-obvious way.

## Command to Run Test


## Proof of Concept
Conceptual exploit, separated into the two environments:

**Case A – Non-TSTORE chain (hard DoS via invalid opcode)**

1. Deploy `TrailsRouter` on an EVM chain that does **not** support TSTORE/TLOAD at the time of deployment (e.g., a pre-Cancun network).
2. In Tstorish’s constructor, `_testTload()` fails, so `_tstoreInitialSupport = false`, and the internal function pointers are wired to the fallback helpers:
   - `_getTstorish = _getTstorishWithSloadFallback`
   - `_setTstorish = _setTstorishWithSstoreFallback`
3. A user has a Sequence v3 wallet `W` that will act as the intent account. According to the protocol design, `TrailsRouter` is **always** invoked via `delegatecall` from `W` (through `TrailsRouterShim`). In `W`’s own layout, `slot0` stores some non-zero configuration value (e.g., an owner or config word).
4. Under `delegatecall`, all `sload`/`sstore` in Tstorish operate on `W`’s storage. Thus `_tstoreSupport` (declared at slot 0 in `Tstorish`) resolves to `W.slot0`, which is non-zero, so `_tstoreSupport == true` inside `_getTstorishWithSloadFallback` / `_setTstorishWithSstoreFallback`.
5. When the Trails shim tries to mark an op as successful, it calls (under `delegatecall`):
   - `_setTstorish(successSlot(opHash), SUCCESS_VALUE)`
   which forwards to `_setTstorishWithSstoreFallback`. Because `_tstoreSupport` reads as `true`, the helper executes `tstore(slot, SUCCESS_VALUE)`.
6. On this chain, `tstore` is an unsupported opcode; the EVM raises an invalid opcode and the entire delegatecall reverts. The successful Trails route that should be recorded by the sentinel instead fails and is rolled back. No opHash can be marked successful for that wallet.
7. If, counterfactually, a sentinel was somehow written using `sstore` while `_tstoreSupport` was false (e.g., before slot 0 became non-zero), a later call to `validateOpHashAndSweep(opHash, token, recipient)` under `delegatecall` would do:
   - `_getTstorish(slot) → _getTstorishWithSloadFallback`
   - The helper sees `_tstoreSupport == true` (wallet.slot0 non-zero) and executes `tload(slot)`, which is again an invalid opcode and reverts.
8. In both write and read paths, any wallet with `slot0 != 0` cannot interact with the Trails sentinel system; `validateOpHashAndSweep` and related flows are effectively bricked for that wallet on that chain.

**Case B – TSTORE-enabled chain (semantic collision / wrong storage backend)**

1. Deploy `TrailsRouter` on a chain where TSTORE is *initially not available*; constructor wires the fallback helpers as in Case A.
2. Sometime later, a hardfork introduces TSTORE support. The operator calls `__activateTstore()` directly on the router implementation to flip `_tstoreSupport` to true in the router’s own storage.
3. Now, for wallets that have `slot0 == 0`, `_tstoreSupport` (under `delegatecall`) is read as `false`, so the fallback helpers use SSTORE/SLOAD for sentinels; for wallets with `slot0 != 0`, `_tstoreSupport` reads as `true`, so they use TSTORE/TLOAD instead.
4. Assume a particular wallet `W1` had `slot0 == 0` when a sentinel was written (so SSTORE was used). Later, `W1` updates its configuration (e.g., owner rotation, enabling a module) and `slot0` becomes non-zero.
5. Now calls to `validateOpHashAndSweep` on `W1` under `delegatecall` see `_tstoreSupport == true` and use `tload` instead of `sload`. The previously SSTORE’d sentinel is invisible to TLOAD, and the read returns 0. As a result, `validateOpHashAndSweep` reverts with `SuccessSentinelNotSet()` even though the op logically succeeded and the persistent sentinel is present.
6. Conversely, a wallet could write a sentinel while `_tstoreSupport` appears true (using TSTORE/transient storage) and later read it when `_tstoreSupport` appears false (using SLOAD). In that case, the sentinel disappears after the transaction that wrote it, again causing `SuccessSentinelNotSet()` or other misbehavior.
7. These behaviors are per-wallet and depend on how `slot0` evolves over time, which is outside `TrailsRouter`’s control. This creates a fragile, wallet-specific failure mode that can silently break Trails flows and fee/sweep semantics on otherwise “healthy” chains.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {TrailsSentinelLib} from "src/libraries/TrailsSentinelLib.sol";

// Minimal wallet mock that delegatecalls into TrailsRouter and has a configurable slot0
contract WalletMock {
    // Occupies storage slot 0 in the wallet
    uint256 public slot0Value;

    constructor(uint256 initial) {
        slot0Value = initial;
    }

    function execDelegate(address target, bytes calldata data) external payable returns (bytes memory) {
        (bool ok, bytes memory ret) = target.delegatecall(data);
        require(ok, "delegatecall failed");
        return ret;
    }
}

contract TstorishDelegatecallCollisionTest is Test {
    TrailsRouter router;
    WalletMock wallet;

    function setUp() public {
        // On a modern Foundry EVM, TSTORE/TLOAD are supported, so Tstorish will likely set
        // `_tstoreInitialSupport = true` and wire `_getTstorish` directly to `_getTstore`.
        // That means this test cannot reproduce the invalid-opcode behavior of a pre-TSTORE
        // chain, but it *can* show the semantic collision under delegatecall where the
        // sentinel is written using persistent storage and later read via TSTORE.
        router = new TrailsRouter();

        // slot0Value = 1, so any `_tstoreSupport` read from slot 0 in the delegatecall
        // context will appear as `true` if the fallback helpers were used. This models the
        // critical assumption that wallet.slot0 is non-zero.
        wallet = new WalletMock(1);
    }

    function test_validateOpHashAndSweep_does_not_see_persistent_sentinel_under_delegatecall() public {
        // This test demonstrates that a sentinel written into the wallet's *persistent* storage
        // (what SSTORE semantics would do) is not visible to `_getTstorish` under delegatecall
        // when TSTORE/TLOAD are used, causing `validateOpHashAndSweep` to revert with
        // SuccessSentinelNotSet().

        // Prepare an arbitrary opHash and compute its sentinel slot
        bytes32 opHash = keccak256("test-op");
        uint256 slot = TrailsSentinelLib.successSlot(opHash);

        // Simulate TrailsRouterShim having written SUCCESS_VALUE into the wallet's *persistent*
        // storage for this opHash, as would be expected if SSTORE/SLOAD were used for sentinels.
        vm.store(address(wallet), bytes32(slot), bytes32(TrailsSentinelLib.SUCCESS_VALUE));

        // Now construct calldata for `validateOpHashAndSweep` and execute it under delegatecall
        bytes memory data = abi.encodeWithSelector(
            router.validateOpHashAndSweep.selector,
            opHash,
            address(0),          // native token
            address(0xBEEF)      // sweep recipient
        );

        // Because `_getTstorish` in Tstorish uses TLOAD on this chain (transient storage), and
        // we have only written to persistent storage, the read will return 0 instead of
        // `SUCCESS_VALUE`, and the router will revert with `SuccessSentinelNotSet()`.
        vm.expectRevert(TrailsRouter.SuccessSentinelNotSet.selector);
        wallet.execDelegate(address(router), data);
    }
}


## Suggested Mitigation
A robust fix should ensure that the decision about whether to use TSTORE vs SSTORE is not coupled to the caller’s storage layout under `delegatecall`, and preferably is not mutable per-wallet at all.

Concrete steps:

1. **Namespace the `_tstoreSupport` flag away from slot 0**
   - Do not rely on the compiler-assigned slot for `_tstoreSupport` in a contract intended to be used via `delegatecall`.
   - Replace the current `bool private _tstoreSupport;` with an explicitly namespaced slot, e.g.:
     - `bytes32 private constant TSTORE_SUPPORT_SLOT = keccak256("org.sequence.tstorish.flag");`
   - Read/write the flag using inline assembly `sload(TSTORE_SUPPORT_SLOT)` / `sstore(TSTORE_SUPPORT_SLOT, ...)`.
   - This ensures that even when executed under `delegatecall`, the flag lives at a dedicated, collision-resistant location that is independent of the wallet’s own layout.

2. **Prefer fixed backend per deployment instead of per-wallet runtime toggling**
   - In `Tstorish` constructor, if `_testTload()` succeeds, permanently bind `_getTstorish` / `_setTstorish` / `_clearTstorish` to the pure TSTORE/TLOAD versions and do *not* expose `__activateTstore()` for that deployment.
   - If `_testTload()` fails, permanently bind to the SSTORE/SLOAD versions and either:
     - Omit `__activateTstore()` entirely, or
     - Make future upgrades use a new router deployment compiled and initialized for TSTORE instead of flipping a flag in-place.
   - This avoids any runtime switching logic based on mutable state that could differ across delegatecall contexts.

3. **If runtime activation is strictly required, make it delegatecall-safe and documented**
   - If maintaining `__activateTstore()` is necessary, combine it with the namespaced slot approach and adjust semantics:
     - Keep the activation flag in a namespaced slot as in (1) so that under `delegatecall` the same logical flag is observed independent of wallet storage layout.
     - Clarify that activation is *global* to the deployment, not per-wallet. Avoid designs where activation depends on or modifies storage in the caller.
   - Alternatively, if you want per-wallet activation, remove the `OnlyDirectCalls` guard and design `Tstorish` such that `_tstoreSupport` is intentionally stored in the wallet’s own namespaced slot and known to be safe. This is a more complex design and should be carefully documented and tested.

4. **Add regression tests around delegatecall usage**
   - Add Foundry tests that:
     - Use a mock wallet with a non-zero storage slot 0 and delegatecall into `TrailsRouter`.
     - Assert that sentinel writes and reads behave consistently on TSTORE-enabled and non-enabled configurations (where you can simulate non-TSTORE by hard-wiring the SSTORE path in a test-only variant of `Tstorish`).
   - Include tests that ensure changing fields in the wallet (e.g., updating slot 0 equivalent) cannot alter the persistence or visibility of sentinels.

In summary, either (a) freeze the TSTORE vs SSTORE choice per deployment and avoid mutable flags entirely, or (b) ensure any such flag is stored in a dedicated, namespaced slot that is independent of the caller’s storage under `delegatecall`. Avoid using compiler slot 0 for configuration flags in delegatecall-only contracts, as this leads directly to storage collisions and the described DoS/semantic issues.





 **Derived From** : EIP-712 domain/struct chainId mismatch can brick intents after chainId change

## [L-2]. EIP-712 chainId mismatch in TrailsIntentEntrypoint bricking deposits after chainId change

### Finding Severity Justification: The issue is a standards / robustness concern around EIP‑712 domain handling on rare chainId changes. Impact is loss of liveness for new deposits on that chain after a hypothetical chainId change; no existing funds can be stolen or misdirected, and the protocol can be redeployed on the new chainId. ChainId changes on production L1/L2s are extremely rare, and the system remains safe (just bricked for new use) if it ever occurred. This aligns best with a QA/Low severity per the rubric (no direct asset loss, but a configuration/design shortcoming).
## Derived From Pattern/Invariant
EIP-712 domain/struct chainId mismatch can brick intents after chainId change

## Exploit Type
StandardViolation

## Location
TrailsIntentEntrypoint._verifyAndMarkIntent

## Finding Status: Valid
## Status Confidence: Confident
### Finding Complexity: 6
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The TrailsIntentEntrypoint contract violates EIP-712 expectations around the chainId used in the domain separator vs. the typed struct, creating a protocol-level liveness risk if the underlying network ever changes its chainId.

In the constructor, the DOMAIN_SEPARATOR is computed once using the deployment-time block.chainid and stored immutably:

  constructor() {
      DOMAIN_SEPARATOR = keccak256(
          abi.encode(
              keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
              keccak256(bytes("TrailsIntentEntrypoint")),
              keccak256(bytes(VERSION)),
              block.chainid,
              address(this)
          )
      );
  }

However, the TrailsIntent struct hash used for signature verification in _verifyAndMarkIntent uses the *current* chainid() at runtime:

  assembly {
      let ptr := mload(0x40)
      mstore(ptr, _typehash)
      mstore(add(ptr, 0x20), user)
      mstore(add(ptr, 0x40), token)
      mstore(add(ptr, 0x60), amount)
      mstore(add(ptr, 0x80), intentAddress)
      mstore(add(ptr, 0xa0), deadline)
      mstore(add(ptr, 0xc0), chainid())
      mstore(add(ptr, 0xe0), nonce)
      mstore(add(ptr, 0x100), feeAmount)
      mstore(add(ptr, 0x120), feeCollector)
      intentHash := keccak256(ptr, 0x140)
  }

The final digest is built as:

  // keccak256(abi.encodePacked("\x19\x01", DOMAIN_SEPARATOR, intentHash));
  assembly {
      let ptr := mload(0x40)
      mstore(ptr, 0x1901)
      mstore(add(ptr, 0x20), _domainSeparator)
      mstore(add(ptr, 0x40), intentHash)
      digest := keccak256(add(ptr, 0x1e), 0x42)
  }

So the contract always verifies signatures against digest = keccak256("\x19\x01" || DOMAIN_SEPARATOR(chainId_deploy) || intentHash(chainId_runtime)). EIP-712 signers, however, will construct digests using a *single* chainId for both the domain and the struct (typically the current network chainId). If the network's chainId ever changes after deployment (a realistic possibility for some rollups or testnets), then at the time of signing:
- The off-chain EIP-712 domain will use chainId = new,
- The TrailsIntent struct will use chainId = new,
- But the contract will still use DOMAIN_SEPARATOR with chainId = old while computing intentHash with chainId = new.

No standard EIP-712 signer can produce a signature that matches this mixed-chainId digest. As a result, all calls to depositToIntent and depositToIntentWithPermit will revert with InvalidIntentSignature() once block.chainid diverges from the chainId at deployment. This permanently bricks the entrypoint on that network for all future intents until a new contract is deployed and integrators migrate. Existing intents and funds are not directly stolen, but protocol liveness and integrations are effectively broken.

This is a StandardViolation of EIP-712 expectations: the domain separator is cached against an outdated chainId while the typed struct hash uses the current chainId, making compliant signatures unverifiable after a chainId change.

## Impact
If the underlying network changes its chainId after deployment, all future intent deposits (depositToIntent / depositToIntentWithPermit) will permanently revert with InvalidIntentSignature(). No new intents can be funded via this entrypoint on that chain, effectively bricking the protocol’s deposit path and breaking all integrations until a new contract is deployed and adopted.

## Command to Run Test


## Proof of Concept
1. Assume TrailsIntentEntrypoint is deployed on a chain with chainId = 1. In the constructor it caches DOMAIN_SEPARATOR with chainId = 1.
2. Later, the network governance changes the chainId to 2 (this is rare on mainnet but has precedent on various rollups/testnets).
3. A user now wants to create a new intent and signs EIP-712 typed data using a standard wallet:
   - EIP-712 domain: { name: "TrailsIntentEntrypoint", version: "1", chainId: 2, verifyingContract: entrypointAddress }
   - TrailsIntent struct: { user, token, amount, intentAddress, deadline, chainId: 2, nonce, feeAmount, feeCollector }.
   The wallet computes digest_user = keccak256("\x19\x01" || domainSeparator(chainId=2) || intentHash(chainId=2)) and signs it.
4. A relayer calls depositToIntent with the signed (v, r, s). Inside _verifyAndMarkIntent, the contract recomputes intentHash using chainid() == 2, but uses the immutable DOMAIN_SEPARATOR that was built with chainId = 1. So the on-chain digest = keccak256("\x19\x01" || domainSeparator(chainId=1) || intentHash(chainId=2)).
5. digest_user (what the user signed) != digest (what the contract verifies), so ECDSA.recover returns an address different from user, and the call reverts with InvalidIntentSignature().
6. There is no way to construct a signature that is both EIP-712-compliant (single chainId used in domain and struct) and matches the contract’s mixed-chainId digest. Thus every future call to depositToIntent / depositToIntentWithPermit on this chain will revert, permanently disabling new deposits via this entrypoint.

The Foundry test below simulates this by:
- Deploying TrailsIntentEntrypoint with vm.chainId(1).
- Then switching to vm.chainId(2).
- Constructing an EIP-712 digest using chainId = 2 for both domain and struct (what a compliant wallet would do).
- Showing that depositToIntent reverts with InvalidIntentSignature() despite the signature being correct under the EIP-712 spec for the new chainId.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsIntentEntrypoint} from "src/TrailsIntentEntrypoint.sol";

contract ChainIdMismatchIntentTest is Test {
    TrailsIntentEntrypoint internal entrypoint;
    uint256 internal userPk;
    address internal user;

    function setUp() external {
        // Simulate initial deployment on chainId = 1
        vm.chainId(1);
        entrypoint = new TrailsIntentEntrypoint();

        userPk = 0xA11CE;
        user = vm.addr(userPk);
    }

    function testChainIdChangeBricksDeposits() external {
        // Simulate a later chainId change to 2
        vm.chainId(2);
        assertEq(block.chainid, 2);

        // Build an EIP-712-compliant digest using chainId = 2
        address token = address(1); // dummy, transfer never reached
        uint256 amount = 1 ether;
        address intentAddress = address(2);
        uint256 deadline = block.timestamp + 1 days;
        uint256 nonce = 0;
        uint256 feeAmount = 0;
        address feeCollector = address(0);

        bytes32 domainSeparator = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes("TrailsIntentEntrypoint")),
                keccak256(bytes(entrypoint.VERSION())),
                block.chainid, // 2
                address(entrypoint)
            )
        );

        bytes32 structHash = keccak256(
            abi.encode(
                entrypoint.TRAILS_INTENT_TYPEHASH(),
                user,
                token,
                amount,
                intentAddress,
                deadline,
                block.chainid, // 2
                nonce,
                feeAmount,
                feeCollector
            )
        );

        bytes32 digest = keccak256(
            abi.encodePacked("\x19\x01", domainSeparator, structHash)
        );

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPk, digest);

        // Even though the signature is correct under EIP-712 for chainId=2,
        // the contract will verify it against DOMAIN_SEPARATOR(chainId=1)
        // and intentHash(chainId=2), so it must revert with InvalidIntentSignature().
        vm.expectRevert(TrailsIntentEntrypoint.InvalidIntentSignature.selector);
        entrypoint.depositToIntent(
            user,
            token,
            amount,
            intentAddress,
            deadline,
            nonce,
            feeAmount,
            feeCollector,
            v,
            r,
            s
        );
    }
}


## Suggested Mitigation
Ensure that the EIP-712 domain separator and the TrailsIntent struct use a consistent, current chainId source, and do not cache a domain separator that becomes stale when chainId changes.

A robust fix is to follow the OpenZeppelin EIP712 pattern and compute the domain separator dynamically based on block.chainid instead of storing an immutable DOMAIN_SEPARATOR tied to the deployment-time chainId:

1. Store hashed name and version as immutables, and replace the immutable DOMAIN_SEPARATOR with a view function:

  bytes32 public constant EIP712_DOMAIN_TYPEHASH = keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
  bytes32 public constant NAME_HASH = keccak256(bytes("TrailsIntentEntrypoint"));
  bytes32 public constant VERSION_HASH = keccak256(bytes(VERSION));

  function DOMAIN_SEPARATOR() public view returns (bytes32) {
      return keccak256(
          abi.encode(
              EIP712_DOMAIN_TYPEHASH,
              NAME_HASH,
              VERSION_HASH,
              block.chainid,
              address(this)
          )
      );
  }

2. In _verifyAndMarkIntent, use DOMAIN_SEPARATOR() instead of the immutable variable, and continue to use chainid() for the TrailsIntent struct’s chainId field. This keeps both parts in sync after a chainId change:

  bytes32 _domainSeparator = DOMAIN_SEPARATOR();
  // build digest as before using _domainSeparator and intentHash (with chainid()).

3. Optionally, simplify the design by removing the chainId field from the TrailsIntent struct entirely and relying solely on the domain’s chainId for replay separation. This would require changing TRAILS_INTENT_TYPEHASH and updating off-chain signing logic (i.e., a version bump), but is closer to standard EIP-712 usage.

With these changes, if the network’s chainId ever changes, both the domain separator and the struct’s chainId will consistently reflect the new value, and new EIP-712 signatures will continue to verify successfully instead of bricking the entrypoint.





 **Derived From** : Balance injection leaves persistent ERC20 allowance to arbitrary target

## [H-3]. Persistent ERC20 allowance in TrailsRouter._injectAndExecuteCall lets arbitrary targets drain wallet balances

### Finding Severity Justification: In the ERC20 branch of _injectAndExecuteCall, the router (when delegatecalled from a Sequence wallet) sets an allowance of callerBalance (the wallet’s full balance of that token) to an arbitrary target via SafeERC20.forceApprove and never clears it. Because target is fully attacker-controlled via calldata and the allowance persists after the call, any malicious or later-compromised target can subsequently call transferFrom on the wallet to drain tokens up to that allowance without any further user signature. This is a direct, unbounded theft of user funds already in the wallet, not just dust or yield, and the attack path is realistic for any integration that uses injectAndCall/injectSweepAndCall with arbitrary targets, so it qualifies as High severity under the rubric.
## Derived From Pattern/Invariant
Balance injection leaves persistent ERC20 allowance to arbitrary target

## Exploit Type
AccountingInvariantViolation

## Location
TrailsRouter._injectAndExecuteCall

## Finding Status: Valid
## Status Confidence: VeryConfident
### Finding Complexity: 4
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The ERC20 branch of TrailsRouter._injectAndExecuteCall breaks the intended accounting/authorization invariant that balance injection is scoped to a single call. When called with a non‑zero token address, the function computes callerBalance as the entire ERC20 balance of address(this) (Sequence wallet under delegatecall), then grants an allowance of callerBalance to an arbitrary target using SafeERC20.forceApprove, and never clears or reduces that allowance afterward.

Vulnerable code (ERC20 branch):

function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    // ... optional placeholder replacement ...

    if (token == address(0)) {
        (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);

        (bool success, bytes memory result) = target.call(callData);
        emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
        if (!success) revert TargetCallFailed(result);
    }
}

In delegatecall context (the intended Sequence wallet use‑case), address(this) is the wallet. _injectAndCallDelegated and injectAndCall both compute callerBalance = _getSelfBalance(token), then call _injectAndExecuteCall. As a result, the wallet ends up approving target to spend up to its entire ERC20 balance at call time. The allowance is not cleared after the call, so target retains long‑lived rights to pull tokens from the wallet with transferFrom, even in later, independent transactions, up to the original callerBalance. This breaks the invariant that Trails balance injection only exposes funds for the specific, atomic call composed in the intent.

Because target is an arbitrary external address supplied via calldata (by integrators/SDK/solvers), a malicious or later‑compromised target can: (1) be called once through Trails, doing minimal or no token spending during the injected call, and (2) later be invoked by anyone to exploit the leftover allowance and drain tokens from the wallet/router without any new user signature.

## Impact
Any ERC20 balance injection to a malicious or later-compromised target grants that target a persistent allowance equal to the wallet’s/router’s full token balance at injection time. In the Sequence wallet delegatecall context, this allows arbitrary third-party contracts to unilaterally pull tokens from the wallet after the original Trails operation completes, up to callerBalance, stealing user funds and violating the intended one-call accounting of injected balances.

## Command to Run Test


## Proof of Concept
1. Assume a Sequence v3 intent wallet (or any contract) uses TrailsRouter via delegatecall. The wallet holds 1,000 MOCK tokens.
2. A Trails route is constructed that uses injectAndCall (via handleSequenceDelegateCall/_injectAndCallDelegated) with token = MOCK and target = MaliciousTarget, an attacker-controlled contract. callData points to MaliciousTarget.doNothing(), and amountOffset/placeholder are zero so no replacement occurs.
3. The wallet delegatecalls TrailsRouter.injectAndCall. Inside injectAndCall, callerBalance = _getSelfBalance(token) = 1,000 MOCK (balance of the wallet). _injectAndExecuteCall runs the ERC20 branch:
   - SafeERC20.forceApprove(MOCK, target, 1,000) is executed in the wallet context, giving MaliciousTarget an allowance of 1,000 MOCK from the wallet.
   - target.call(callData) calls MaliciousTarget.doNothing(), which performs no token transfer.
4. The injectAndCall invocation returns successfully. The wallet still holds all 1,000 MOCK (no tokens were moved), but allowance(wallet, target) is 1,000.
5. Later, without any further action from the wallet owner, an attacker calls MaliciousTarget.steal(attacker, 1,000). Inside steal, the contract executes MOCK.transferFrom(wallet, attacker, 1,000), which succeeds thanks to the leftover allowance.
6. Result: the Sequence wallet loses 1,000 MOCK to the attacker, even though the user only authorized a single Trails operation and the injected call itself did not spend these tokens. The persistent allowance violates the invariant that injected balances are only available for the scoped call.

## Proof of Code
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor() ERC20("Mock", "MOCK") {}
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract MaliciousTarget {
    IERC20 public token;
    address public victim;

    constructor(IERC20 _token, address _victim) {
        token = _token;
        victim = _victim;
    }

    function doNothing() external {}

    function steal(address to, uint256 amount) external {
        token.transferFrom(victim, to, amount);
    }
}

contract WalletMock {
    address public router;

    constructor(TrailsRouter _router) {
        router = address(_router);
    }

    function runInjectAndCall(address token, address target, bytes memory callData) external {
        (bool ok, bytes memory ret) = router.delegatecall(
            abi.encodeWithSelector(
                TrailsRouter.injectAndCall.selector,
                token,
                target,
                callData,
                0,
                bytes32(0)
            )
        );
        require(ok, string(ret));
    }
}

contract TrailsRouterAllowanceTest is Test {
    TrailsRouter router;
    MockERC20 token;
    WalletMock wallet;
    MaliciousTarget target;
    address attacker = address(0xBEEF);

    function setUp() public {
        router = new TrailsRouter();
        token = new MockERC20();
        wallet = new WalletMock(router);
        token.mint(address(wallet), 1000 ether);
        target = new MaliciousTarget(IERC20(address(token)), address(wallet));
    }

    function testPersistentAllowanceAllowsTheft() public {
        // Wallet starts with 1000 tokens
        assertEq(token.balanceOf(address(wallet)), 1000 ether);

        // Build calldata for the benign-looking injected call
        bytes memory callData = abi.encodeWithSelector(target.doNothing.selector);

        // Delegatecall into TrailsRouter.injectAndCall from the wallet context
        wallet.runInjectAndCall(address(token), address(target), callData);

        // Wallet still holds all tokens, but has approved target for full balance
        assertEq(token.balanceOf(address(wallet)), 1000 ether);
        assertEq(token.allowance(address(wallet), address(target)), 1000 ether);

        // Later, attacker uses the leftover allowance to drain the wallet
        vm.prank(attacker);
        target.steal(attacker, 1000 ether);

        assertEq(token.balanceOf(address(wallet)), 0);
        assertEq(token.balanceOf(attacker), 1000 ether);
    }
}


## Suggested Mitigation
In the ERC20 branch of _injectAndExecuteCall, avoid granting and leaving a broad, long-lived allowance equal to the full wallet/router balance. Instead:
- Compute the actual amount that target should be allowed to spend (e.g., decode from the placeholder at amountOffset) and use that value instead of callerBalance; and
- Immediately clear the allowance after the external call returns.

Concretely, in the ERC20 path:
1) Before the call, set allowance to the minimal required amount using SafeERC20.forceApprove(erc20, target, amountNeeded).
2) After the call, unconditionally reset the allowance back to zero with SafeERC20.forceApprove(erc20, target, 0), even if the target partially used the allowance.

Alternatively, avoid allowances entirely by transferring tokens directly to target (safeTransfer) and constraining the protocol design to pull-only patterns that do not require arbitrary third-party contracts to retain spend rights. This restores the invariant that balance injection only exposes funds for the duration and scope of the single Trails operation.



