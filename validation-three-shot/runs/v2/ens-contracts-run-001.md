# ens-contracts Three-Shot Validation run-001

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/ens-contracts/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`
Stage 1 scope screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/scope-screens/v2/ens-contracts-run-001.md`
Stage 2 bounty exploitability screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/token-screens/v2/ens-contracts-run-001.md`
Stage 3 final validation run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/stage3-runs/v2/ens-contracts-run-001.md`

## Assembly Summary

- Excluded at stage 1 (scope / known issue): `1`
- Excluded at stage 2 (bounty exploitability): `17`
- Fully validated at stage 3: `5`

## Per-Finding Validation

### C-1 / `_qAeE89D1CAdrEK_KI7DX`
- Finding Title: Expired ENS names can receive hidden ERC721 approvals that become active again after renewal
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `expired-token-approval`
- Bounty Criteria Match: critical (smart contract): Direct theft of any user NFTs, whether at-rest or in-motion, other than unclaimed royalties
- Checklist Gates Passed: `in-scope asset, current code path, unprivileged attacker path, exact impact row, feasible exploit path`
- Checklist Gates Failed: `-`
- Detailed Reason: The issue maps cleanly to the ENS program's Critical NFT theft row: a non-privileged address that previously held normal ERC721 operator authority can create a hidden per-token approval while the name is expired, survive revocation of the operator approval, and transfer the renewed registrar NFT once it becomes live again. The impact is not merely stale state or UX confusion; it is unauthorized transfer of an ENS ERC721 after renewal, and it does not require DAO/admin access, leaked keys, or a malicious trusted protocol role.
- Code Evidence: In `contracts/ethregistrar/BaseRegistrarImplementation.sol`, `ownerOf` treats names as unowned after expiry by requiring `expiries[tokenId] > block.timestamp`, and `_isApprovedOrOwner` uses that expiry-aware `ownerOf`; however the contract does not override ERC721 `approve`. The inherited OpenZeppelin `ERC721.approve` calls `ERC721.ownerOf(tokenId)` directly, while `renew` only increments `expiries[id]` and does not clear approvals, so the stored `getApproved(id)` becomes usable again by `transferFrom` or `reclaim` after renewal.

### M-2 / `51MUeJkjADHOWM6JQl0XX`
- Finding Title: Reusable reverse-name signatures let stale relayers overwrite newer DefaultReverseRegistrar records
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `signature-replay`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, current code path, unprivileged caller path`
- Checklist Gates Failed: `exact impact row, non-temporary impact`
- Detailed Reason: The replay behavior exists, but the demonstrated consequence is a temporary overwrite of a standalone reverse-name string during a signature window capped at one hour. Under the strict ENS Immunefi rules this is too close to temporary nuisance/view-level identity griefing to submit as a bounty finding: it does not steal or freeze funds/NFTs, does not affect registration fees or treasury funds, and does not establish user/protocol damage beyond an ambiguous Medium griefing theory.
- Code Evidence: `contracts/reverseRegistrar/DefaultReverseRegistrar.sol` builds a signed message from the registrar, selector, address, expiry, and name, validates it, and calls `_setName(addr, name)` without a nonce or consumed-digest check. `contracts/reverseRegistrar/SignatureUtils.sol` only checks signature validity plus an expiry no more than one hour in the future, so replay within that window is possible, but the stored effect is only the `_names[addr]` value in `StandaloneReverseRegistrar`.

### C-3 / `J8z405q9aEEOyFyX089YE`
- Finding Title: Expired .eth name re-registration without resolver leaves prior owner's resolver controlling resolution
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `stale-resolver-state`
- Bounty Criteria Match: critical (smart contract): Unintended alteration of what the NFT represents (e.g. token URI, payload, artistic content)
- Checklist Gates Passed: `in-scope asset, current code path, unprivileged attacker path, exact impact row, feasible exploit path`
- Checklist Gates Failed: `-`
- Detailed Reason: This is a clear submission-grade issue because a previous unprivileged registrant can leave a resolver they control attached to an expired name, and a later no-resolver re-registration changes ownership without clearing that resolver. For an ENS name NFT, the resolver is the active payload users and integrations consult for what the name represents; retaining attacker-controlled resolution after ownership transfers is an unintended alteration of the NFT/name representation, with payment redirection as a plausible follow-on effect.
- Code Evidence: In `contracts/ethregistrar/ETHRegistrarController.sol`, the `registration.resolver == address(0)` branch calls only `base.register(...)`; the resolver-setting branch is the one that calls `ens.setRecord(...)`. `BaseRegistrarImplementation._register` then calls `ens.setSubnodeOwner(baseNode, bytes32(id), owner)`, and `contracts/registry/ENSRegistry.sol` shows `setSubnodeOwner` updates only the subnode owner, while resolver and TTL are changed only through `setRecord`, `setSubnodeRecord`, or `_setResolverAndTTL`.

### H-4 / `VBCthqJsNe58AfV8cKuzh`
- Finding Title: P256SHA256Algorithm.verify accepts DNSKEY material that is not bound to DNSSEC Algorithm 13
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `safeguard-blocks`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: DNSSECImpl parses DNSKEY protocol and algorithm before dispatch, so malformed Algorithm 13 headers do not reach P256 verification in the oracle path.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-5 / `6iLweelVLw3pz3C8yl6Jl`
- Finding Title: UniversalResolver multicall returns failed CCIP batch lookups as successful resolver results
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `trusted-role-error`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Spoofed successful multicall output requires a malicious or wrong batch gateway response; an unprivileged caller cannot impose those bytes on victims.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-6 / `czakDZoLlxs5lgJdEaovq`
- Finding Title: Multicoin addr resolution returns address ABI and rejects valid non-20-byte ENSIP-11 records
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The ABI bug is real but only affects caller-supplied or DNS-owner-controlled TXT context and gives no unprivileged path against another user or protocol state.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-7 / `6a-XhkrZKMLYM7rhTBUe9`
- Finding Title: Stale token approvals survive transfers after CANNOT_APPROVE, letting an old approved address extend subnames
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Unclear
- Root Cause Family: `stale-wrapper-approval`
- Bounty Criteria Match: medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)
- Checklist Gates Passed: `in-scope asset, current code path, unprivileged caller path`
- Checklist Gates Failed: `clear damage threshold, submission-ready impact`
- Detailed Reason: The stale approval behavior is real, but the qualifying Immunefi impact is not clearly submission-ready. The old approved address can retain `canExtendSubnames` authority after a parent transfer and force child-name expiry extensions up to the parent expiry, but the finding does not show theft, freezing, registration-fee loss, or a clearly material griefing damage threshold beyond unwanted expiry extension. This should be reviewed by a human before treating it as a Medium griefing bounty.
- Code Evidence: `contracts/wrapper/NameWrapper.sol` rejects new `approve` calls once `CANNOT_APPROVE` is burned, but `_beforeTransfer` deletes `_tokenApprovals[id]` only when `CANNOT_APPROVE` is not burned. `canExtendSubnames` treats `getApproved(uint256(node)) == addr` as authorization, and `extendExpiry` uses that result before updating child expiry, while `ERC1155Fuse._transfer` simply calls `_beforeTransfer` and then `_setData(id, to, fuses, expiry)`.

### H-8 / `BCFvJ221KeZUhP6VU5eBq`
- Finding Title: Stale or invalid ETH/USD oracle answers can underprice ENS registration and renewal fees
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `external-system-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: The impact depends on stale or invalid data from a third-party ETH/USD pricing oracle, which the bounty excludes.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`.

### M-9 / `lVVCkIB8v6NnxPXfbcKu2`
- Finding Title: Strict less-than grace-period checks let expired .eth names be transferred at the registrar expiry timestamp
- Decision: Invalid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Do Not Submit
- Root Cause Family: `expiry-boundary`
- Bounty Criteria Match: -
- Checklist Gates Passed: `in-scope asset, current code path`
- Checklist Gates Failed: `exact impact row, non-temporary impact, exploit feasibility`
- Detailed Reason: The equality gap exists, but it is not a strict Immunefi submission. The claimed exploit only applies at the exact registrar-expiry timestamp, gives a stale owner/operator one boundary opportunity to perform an action they could already perform immediately before expiry, and does not clearly cause direct theft, freezing, registration-fee loss, or sustained griefing. That makes it a hardening/edge-case issue rather than a listed bounty impact.
- Code Evidence: `contracts/wrapper/NameWrapper.sol` subtracts `GRACE_PERIOD` from `.eth` wrapper expiry in `_beforeTransfer` and checks `expiry < block.timestamp`, while `_isETH2LDInGracePeriod` checks `expiry - GRACE_PERIOD < block.timestamp`; both exclude equality. `BaseRegistrarImplementation.ownerOf` uses `expiries[tokenId] > block.timestamp`, so the registrar and wrapper disagree at equality, but the demonstrated effect is limited to that boundary condition.

### L-10 / `trbxeQBTMBIC-ALIGbQLF`
- Finding Title: ETH reverse resolver returns addr.reverse registrar for unrelated reverse namespaces
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: This is a direct view resolver namespace mismatch and does not let an attacker change records or force clients to query the wrong resolver.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### L-11 / `BDPZ_5wcfnzQUS0E-X5fl`
- Finding Title: Odd-length reverse labels resolve as aliases for canonical EVM addresses
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Odd-length aliases only affect direct crafted view calls and do not create a state change or attacker-controlled impact on another account.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-12 / `DRKsNuQtFB8DiYxxNr00-`
- Finding Title: Non-canonical 39-nibble reverse labels resolve to canonical address names in DefaultReverseResolver.resolve
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Same canonicality issue as L-11; crafted view input can alias an address but no unprivileged harmful state transition follows.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-13 / `B80Sn7wWoAV__-X9emW1F`
- Finding Title: Default registrar is returned for unrelated reverse namespaces in DefaultReverseResolver.resolve
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Registrar discovery is mis-scoped in a view call, but the attacker cannot alter namespace ownership or force a victim resolver route.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-14 / `0MzfSAZHFu-yafYBgn7kd`
- Finding Title: Malformed short DNS TXT context reverts ExtendedDNSResolver._findValue and DoSes resolution
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Malformed context reverts, but the context is caller-supplied or DNS-owner-controlled and cannot be forced onto a victim name without DNS or gateway control.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-15 / `5YV4UNdiG3l7fe3YbslGB`
- Finding Title: Ignored TXT records ending at EOF trigger out-of-bounds reads and DoS ExtendedDNSResolver lookups
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: EOF parser reverts require malformed TXT context controlled by the DNS owner or caller, with no concrete unprivileged victim path.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-16 / `juVObwZbLvsqXFJSoaFF9`
- Finding Title: Ignored bracketed TXT key at EOF DoS in ExtendedDNSResolver._findValue reverts all matching lookups
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: This duplicates the ignored bracket EOF parser revert and still lacks a path beyond caller-controlled or DNS-owner-controlled context.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### H-17 / `ykxAX67ZAlhbqeuUQ6hwf`
- Finding Title: StaticBulkRenewal.renewAll refunds the entire contract ETH balance to the caller
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The global-balance refund can sweep forced ETH, but normal operation leaves no user funds in this non-payable helper and the PoC profit is forced or mistaken ETH.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### H-18 / `lgI_AYG83YjljxWSCyCrG`
- Finding Title: StaticBulkRenewal.renewAll fails at the final grace-period timestamp when premium refund is returned
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: The boundary refund failure is a self-failing helper path at one timestamp, not an attacker-driven freeze of another user's renewal.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-19 / `fupzTRRvDjR5RojlZjHzX`
- Finding Title: StaticBulkRenewal.renewAll always calls msg.sender refund and blocks non-receivable contract callers
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Rejecting contract callers DoS their own bulk renewal because they cannot receive the refund call; no external attacker path is shown.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-20 / `RLL39_tA45x7uvx02N6gb`
- Finding Title: Malformed DNSKEY exponent length makes RSASHA256Algorithm.verify revert instead of rejecting the proof
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Malformed RSA keys can revert verification, but an attacker only makes their own proof transaction fail absent a relayer or trusted DNS input dependency.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### H-21 / `Ovl2IoLGc0ESEgrGyixaa`
- Finding Title: RSASHA256Algorithm.verify accepts forged signatures when DNSKEY exponent is 1
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Exponent-one direct verify succeeds, but exploitation requires a trusted DNSKEY or DS chain for the target zone, which an attacker cannot create.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-22 / `AyWIZjZJ3bf1WXvDs4EXH`
- Finding Title: Malformed RSA key material makes RSASHA256Algorithm.verify revert instead of returning false
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `weak-poc-path`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Same malformed RSA input class as M-20; reverts are paid by the caller and no victim-submitted proof path is concrete.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.

### M-23 / `fkK11fgPckx9ByS1Icpau`
- Finding Title: Unbounded RSA key and signature sizes allow gas griefing through RSASHA256Algorithm.verify
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / QA
- Root Cause Family: `not-exploitable`
- Checklist Gates Passed: `Gate 1`
- Checklist Gates Failed: `Gate 2`
- Detailed Reason: Oversized RSA inputs burn gas in the caller's own verification transaction and no protocol path forces another party to process attacker-sized proofs.
- Code Evidence: Excluded during the stage 2 bounty exploitability screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`, `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`, and the stage 1 scope screen.
