# ssv-network Three-Shot Stage 2 Bounty Exploitability Screen

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
| H-1 / `QG_ICWJZysYLIuVfb6oiu` | Permissionless fee sync can permanently strand staking ETH reward dust in SSVStaking._syncFees | Keep | High | current-exploit | Public sync advances stakingEthPoolBalance after a floored accEthPerShare update and never carries the undistributed remainder forward. |
| M-3 / `aODXOBpkJU2LTjPYsmixx` | Stale direct whitelist survives switch to whitelisting contract in SSVNetwork.setOperatorsWhitelistingContract | Exclude | High | trusted-role-error | Direct and contract whitelists are additive by design and stale access depends on the operator owner not calling removeOperatorsWhitelists. |
| M-4 / `DcWqUJnMiTIlMosXEUW4o` | Old direct whitelist remains authorized after switching an operator to a whitelist contract | Exclude | High | trusted-role-error | Same duplicate whitelist theory as M-3 and the retained bitmap entry is an intended migrated authorization with an explicit removal path. |
| C-5 / `xV5HkuxJmnZDvpMrER57I` | Premature ETH cluster liquidation due to inconsistent fee rounding in ClusterLib.isLiquidatableWithEB | Keep | Medium | current-exploit | Liquidation settles fees with separate floors but checks a combined-rate threshold, leaving a reachable rounding window for public liquidation. |
| H-6 / `q-CPmfz22UKqoCt56c1Ht` | uint128 truncation in SSVStaking._syncFees can permanently under-account ETH rewards | Exclude | High | future-speculation | The downcast exists but overflow needs a future near-dust cSSV supply and huge unsynced fees, not a current unprivileged exploit path. |
| H-7 / `zzWCpNrNBYw-Cdvv_tgOw` | Frequent EB updates undercharge cluster fees by repeatedly flooring vUnit fee math | Keep | Medium | current-exploit | Public valid EB updates floor vUnit operator and network fee debt each time without remainders, so repeated normal updates can retain undercharged ETH. |
| M-9 / `JDUeqSWcB7YKK1PeXVxfX` | Dust deposits can frontrun liquidations by invalidating caller-supplied cluster state | Keep | High | current-exploit | Permissionless dust deposits change the stored cluster hash before liquidation validation, enabling cheap mempool griefing of public liquidator calldata. |
