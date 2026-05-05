# ens-contracts Three-Shot Stage 3 run-001

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/ens-contracts/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-bounty-rules.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-severity-rubric.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-immunefi-poc-runtime.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ens-contracts/ens-contracts-scope.txt`
- `/Users/apmfree/Desktop/Audit/ens-contracts-91c966/ens-contracts/README.md`

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
