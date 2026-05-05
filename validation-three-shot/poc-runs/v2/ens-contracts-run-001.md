# ens-contracts Three-Shot Round 5 PoC Generation

Status: Complete
Source submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/ens-contracts-run-001.md`
JSON summary: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-runs/v2/ens-contracts-run-001.json`

## PoC Results

| Finding | Finding Title | Status | PoC Test Path | Test Command | Test Result | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| C-1 | Expired ENS names can receive hidden ERC721 approvals that become active again after renewal | PoC Created | test/ExpiredApprovalRenewalTheftPoC.test.ts | bun run test test/ExpiredApprovalRenewalTheftPoC.test.ts | passed: 1 file 1 test | Local source-tree simulation. Runtime artifact lists no deploy address or RPC target. Demonstrates hidden approval, revocation, renewal, transfer, and reclaim. |
| C-3 | Expired .eth name re-registration without resolver leaves prior owner's resolver controlling resolution | PoC Created | test/ExpiredNameNoResolverReRegistrationPoC.test.ts | bun run test test/ExpiredNameNoResolverReRegistrationPoC.test.ts | passed | Local Hardhat/Vitest simulation because runtime lists no extracted deployed address or network. Uses real ETHRegistrarController and OwnedResolver. |
<!-- APPEND SCREEN ROWS ABOVE THIS LINE -->
