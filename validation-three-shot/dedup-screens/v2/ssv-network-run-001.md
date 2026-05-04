# ssv-network Three-Shot Round 4 Canonicalization Screen

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/ssv-network-run-001.md`
Output submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/ssv-network-run-001.md`

## Decisions

| Finding | Finding Title | Decision | Confidence | Canonical Finding | Root Cause Group | Report Planning Note | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| C-5 | Premature ETH cluster liquidation due to inconsistent fee rounding in ClusterLib.isLiquidatableWithEB | Keep | High | - | liquidation-threshold-rounding | separate-reports | keep-distinct-path | Premature liquidation comes from threshold rounding mismatch and can pay out remaining cluster ETH, which is materially different from stale-hash griefing. |
| M-9 | Dust deposits can frontrun liquidations by invalidating caller-supplied cluster state | Keep | High | - | cluster-hash-frontrun-griefing | separate-reports | keep-distinct-path | This uses a permissionless dust deposit to mutate the stored cluster hash and revert pending liquidations, with a distinct action, impact, and mitigation. |
