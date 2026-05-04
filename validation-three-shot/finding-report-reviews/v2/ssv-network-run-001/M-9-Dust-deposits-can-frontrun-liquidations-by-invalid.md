# M-9 Immunefi Finding Report Review

Status: Ready
Benchmark: `ssv-network`
Finding: `M-9`
Report reviewed: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/finding-reports/v2/ssv-network-run-001/M-9-Dust-deposits-can-frontrun-liquidations-by-invalid.md`

## Review Result

The report is submission-ready. I did not edit the R7 report.

## Checks

- Severity is Medium, matching the round 3 Immunefi classification and the generated SSV Network rubric.
- `Bounty Criteria Match` names exact in-scope Medium smart contract impact rows: `Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)` and `Theft of gas`.
- The attacker is unprivileged: the exploit uses public `deposit` and `liquidate` entrypoints and does not require leaked keys, compromised credentials, or trusted-role behavior.
- The issue is currently exploitable in in-scope code: `SSVNetwork.deposit` and `SSVNetwork.liquidate` delegate to `SSVClusters`; `SSVClusters.deposit` accepts third-party ETH dust and stores a new `ethClusters` hash; `SSVClusters.liquidate` calls `validateHashedCluster` before liquidatability checks; `ClusterLib.validateHashedCluster` rejects stale caller-supplied cluster data with `IncorrectClusterState`.
- The impact is covered by the bounty's listed Medium griefing and theft-of-gas rows. It is not framed as Critical theft, permanent freezing, insolvency, QA, hardening, best practice, or speculation.
- The report uses function-level GitHub links and concise source snippets.
- The report PoC matches the verified PoC test path `test/forked/v2.0.0/M-9-DustDepositLiquidationGriefingPoC.test.ts`: it registers a near-liquidatable ETH cluster, mines until liquidatable, sends a one-wei third-party deposit, proves stale liquidation reverts with `IncorrectClusterState`, and proves liquidation succeeds with the updated cluster state.
- The PoC is a non-harmful local Hardhat fork simulation and does not broadcast live transactions or mutate live protocol state.
- The final report contains no local absolute paths, local commands, console output, icons, fluff, or custom code wrappers.

## PoC Run

Command:

```bash
set -a; source "/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/.env"; set +a; RUN_FORK=true FORK_USE_DEPLOYED_STATE=true FORK_CSSV_TOKEN=0xe018D31F120A637828F46aFD6c64EC099d960546 npx hardhat test test/forked/v2.0.0/M-9-DustDepositLiquidationGriefingPoC.test.ts
```

Result: passed.
