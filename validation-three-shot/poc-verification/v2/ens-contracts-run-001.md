# ens-contracts Three-Shot Round 6 PoC Verification

Status: Complete
Source submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/ens-contracts-run-001.md`
Source R5 PoC run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-runs/v2/ens-contracts-run-001.md`
JSON summary: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-verification/v2/ens-contracts-run-001.json`

## PoC Verification Results

| Finding | Finding Title | Status | Final PoC Test Path | Test Command | Verification Result | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| C-1 | Expired ENS names can receive hidden ERC721 approvals that become active again after renewal | Verified PoC | test/ExpiredApprovalRenewalTheftPoC.test.ts | bun run test test/ExpiredApprovalRenewalTheftPoC.test.ts | passed and proves vulnerability | Local source-tree simulation uses in-scope ENSRegistry and BaseRegistrarImplementation because runtime artifact has no deployed address or RPC target. Hidden approval persists through renewal and authorizes transfer plus reclaim. |
| C-3 | Expired .eth name re-registration without resolver leaves prior owner's resolver controlling resolution | Verified PoC | test/ExpiredNameNoResolverReRegistrationPoC.test.ts | bun run test test/ExpiredNameNoResolverReRegistrationPoC.test.ts | passed and proves vulnerability | Local simulation accepted because runtime extracts no deployed address or network. Real registrar and registry path shows ownership changes while stale attacker resolver and addr record persist. |
<!-- APPEND SCREEN ROWS ABOVE THIS LINE -->
