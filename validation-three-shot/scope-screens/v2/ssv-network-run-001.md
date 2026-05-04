# ssv-network Three-Shot Stage 1 Scope Screen

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/ssv-network/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/ssv-network-9bb7b2/ssv-network`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-immunefi-bounty-rules.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-immunefi-severity-rubric.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-immunefi-poc-runtime.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-scope.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/ssv-network/ssv-network-scope.txt`
- `/Users/apmfree/Desktop/Audit/ssv-network-9bb7b2/ssv-network/README.md`

## Decisions

| Finding | Finding Title | Decision | Confidence | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- |
| H-1 | Permissionless fee sync can permanently strand staking ETH reward dust in SSVStaking._syncFees | Keep | High | in-scope | Affects SSVNetwork staking syncFees and maps to permanent freezing of unclaimed yield; no decisive OOS or known issue found. |
| L-2 | SSVStaking.claimEthRewards deletes sub-precision rewards for fully unstaked users | Exclude | High | known-issue-or-oos | SPEC and FLOWS document zero-cSSV sub-precision ETH reward dust as intentionally forfeited and redistributed. |
| M-3 | Stale direct whitelist survives switch to whitelisting contract in SSVNetwork.setOperatorsWhitelistingContract | Keep | High | in-scope | Affects private operator whitelist checks reached through SSVNetwork and can map to Medium griefing; no stage-1 blocker found. |
| M-4 | Old direct whitelist remains authorized after switching an operator to a whitelist contract | Keep | High | in-scope | Affects private operator whitelist checks reached through SSVNetwork and can map to Medium griefing; no stage-1 blocker found. |
| C-5 | Premature ETH cluster liquidation due to inconsistent fee rounding in ClusterLib.isLiquidatableWithEB | Keep | High | in-scope | Affects SSVNetwork ETH cluster liquidation and claimed liquidator payout maps to direct theft of user funds; no OOS blocker found. |
| H-6 | uint128 truncation in SSVStaking._syncFees can permanently under-account ETH rewards | Keep | Medium | in-scope | Affects SSVNetwork staking syncFees and claimed under-accounting maps to permanent freezing of unclaimed yield; feasibility is for later rounds. |
| H-7 | Frequent EB updates undercharge cluster fees by repeatedly flooring vUnit fee math | Keep | Medium | in-scope | Affects SSVNetwork ETH cluster fee accounting and claimed undercharge can map to theft of unclaimed yield; no decisive stage-1 blocker found. |
| C-8 | Unset delegated modules silently no-op and trap ETH in SSVNetwork payable entrypoints | Exclude | High | deployment-or-setup | Requires module slots left unset during deployment or upgrade, while mainnet artifacts and playbook attach all modules; setup misconfiguration is OOS. |
| M-9 | Dust deposits can frontrun liquidations by invalidating caller-supplied cluster state | Keep | High | in-scope | Affects SSVNetwork deposit and liquidation paths and claimed repeated gas griefing maps to Medium griefing or theft of gas. |
