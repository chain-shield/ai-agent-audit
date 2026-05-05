# ens-contracts Three-Shot Round 4 Canonicalization Screen

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/ens-contracts-run-001.md`
Output submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/ens-contracts-run-001.md`

## Decisions

| Finding | Finding Title | Decision | Confidence | Canonical Finding | Root Cause Group | Report Planning Note | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| C-1 | Expired ENS names can receive hidden ERC721 approvals that become active again after renewal | Keep | High | - | expired-token-hidden-approval | separate-reports | keep-distinct-path | Approval during expiry then post-renewal transfer is a distinct registrar authorization path with different invariant impact and mitigation. |
| C-3 | Expired .eth name re-registration without resolver leaves prior owner's resolver controlling resolution | Keep | High | - | stale-resolver-on-reregistration | separate-reports | keep-distinct-path | Re-registration without resolver preserves stale ENS resolution state on a different controller and registry path with different exploit action and mitigation. |
