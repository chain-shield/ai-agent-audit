# legion-protocol-contracts Three-Shot Validation run-001

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/legion-protocol-contracts/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`
Stage 1 scope screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/scope-screens/v2/legion-protocol-contracts-run-001.md`
Stage 2 bounty exploitability screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/token-screens/v2/legion-protocol-contracts-run-001.md`
Stage 3 final validation run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/stage3-runs/v2/legion-protocol-contracts-run-001.md`

## Assembly Summary

- Excluded at stage 1 (scope / known issue): `12`
- Excluded at stage 2 (bounty exploitability): `0`
- Fully validated at stage 3: `0`

## Per-Finding Validation

### C-1 / `uoil2rJMmqsP86suE8nmu`
- Finding Title: Replayable position-transfer signature can burn a later position and erase the recipient position accounting
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `closed-bounty-report`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Closed bounty issue #88 covers replayable transfer authorization theft across these position-transfer paths.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### C-2 / `iD7WrY6Dj3h9mjw-VVWl5`
- Finding Title: Inherited native ETH release() bypasses LegionLinearVesting cliff and allows premature withdrawal
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `closed-bounty-report`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Closed bounty issue #80 covers the same native ETH release before cliff in LegionLinearVesting.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### H-3 / `cqFeWIWPbvD3ZADRTIFts`
- Finding Title: Reusable investment signatures allow uncapped repeated bids in LegionSealedBidAuctionSale.invest
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `known-issue-or-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Bounty known issues explicitly exclude signature reuse when investing with the same signature.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### C-4 / `kM-dNdUpDkp_c64tx7EYq`
- Finding Title: Replayable transfer authorization corrupts investor position ownership in LegionFixedPriceSale
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `closed-bounty-report`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Closed bounty issue #88 specifically covers replayed position-transfer authorization in fixed price sales.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### C-5 / `jHvRScpU_KDrXKUzzXRV9`
- Finding Title: Position merge can credit a victim's position to another account while burning an unrelated position
- Decision: Invalid
- Confidence: Medium
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `trusted-role-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Standalone path requires a trusted signer or Legion caller to authorize an inconsistent from and positionId tuple.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### C-6 / `HbNRKLz6M6miWKv_ggwB2`
- Finding Title: Stale transfer authorizations can merge another user's position and burn the signer's current position
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `closed-bounty-report`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Closed bounty issue #88 covers missing transfer signature consumption across sale, raise, and position manager paths.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### H-7 / `R8fv_EAzMcPv_V-7xAYyM`
- Finding Title: Stale distributor claim signatures can overclaim and exhaust tokens for later valid claimants
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `closed-bounty-report`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Closed bounty issue #86 covers LegionTokenDistributor over-claims beyond total allocation causing later claims to fail.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### H-8 / `IJy4rzcAerefpKytgAG47`
- Finding Title: Stale position-transfer authorization can merge another holder's position and burn an unrelated position
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `closed-bounty-report`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Closed bounty issue #88 covers stale position-transfer authorization replay and resulting NFT or entitlement theft.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### C-9 / `E7_tysEzu7-rWQaUfS2Tg`
- Finding Title: Replayable position transfer authorization can burn a later investor position and permanently freeze entitlements
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `closed-bounty-report`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Closed bounty issue #88 covers replayable position-transfer authorization with permanent entitlement loss.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### C-10 / `dcLDkpfg2UzZJBkdHIGE1`
- Finding Title: Replayable position transfer authorizations can re-steal returned investor positions
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `closed-bounty-report`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Closed bounty issue #88 covers cyclic replay after a position returns to the original signer.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### H-11 / `zjjz-K9B_RVshExZkHdoH`
- Finding Title: Fee-on-transfer bid tokens over-credit deposits and can leave later investor refunds insolvent
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `known-issue-or-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Bounty known issues explicitly exclude lack of support for fee-on-transfer and rebasing tokens.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.

### H-12 / `N3pyy1ob4Oimqad_K-3yo`
- Finding Title: Fee-on-transfer askToken makes distributor insolvent and freezes later valid token claims
- Decision: Invalid
- Confidence: High
- Bug Exists: Unclear
- Severity Assessment: Low / Unclear
- Root Cause Family: `known-issue-or-oos`
- Checklist Gates Passed: `Pre-gate sanity`
- Checklist Gates Failed: `Gate 1`
- Detailed Reason: Bounty known issues explicitly exclude lack of support for fee-on-transfer and rebasing tokens.
- Code Evidence: Excluded during the stage 1 scope / known-issue screen using `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-docs.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/legion-protocol-contracts-scope.md`, `/Users/apmfree/Desktop/Audit/legion-protocol-contracts-314e40/legion-protocol-contracts/README.md`, `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/code4rena-bounty-criteria.md`.
