# ens-contracts Three-Shot Submission Candidates run-001

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/ens-contracts-run-001.md`
Canonicalization screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/dedup-screens/v2/ens-contracts-run-001.md`

## Canonicalization Summary

- Candidate H/M findings before R4: `2`
- Kept after R4 canonicalization: `2`
- Dropped by R4 cleanup: `0`
- Dropped finding ids: `-`

## Root Cause Groups

- `expired-token-hidden-approval`: C-1
- `stale-resolver-on-reregistration`: C-3

## Submission Candidates

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
