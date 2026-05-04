# ssv-network Three-Shot Round 5 PoC Generation

Status: Complete
Source submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/ssv-network-run-001.md`
JSON summary: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-runs/v2/ssv-network-run-001.json`

## PoC Results

| Finding | Finding Title | Status | PoC Test Path | Test Command | Test Result | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| C-5 | Premature ETH cluster liquidation due to inconsistent fee rounding in ClusterLib.isLiquidatableWithEB | PoC Created | /Users/apmfree/Desktop/Audit/ssv-network-9bb7b2/ssv-network/test/forked/v2.0.0/C-5-premature-liquidation-rounding.poc.test.ts | set -a; . "/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/.env"; set +a; NO_GAS_ENFORCE=true npx hardhat test test/forked/v2.0.0/C-5-premature-liquidation-rounding.poc.test.ts | passed: 1 passing | Mainnet fork attaches to in-scope SSVNetwork and Views. It proves liquidation succeeds with balance below the combined threshold by 1 wei while still above the separately rounded threshold. |
| M-9 | Dust deposits can frontrun liquidations by invalidating caller-supplied cluster state | PoC Created | /Users/apmfree/Desktop/Audit/ssv-network-9bb7b2/ssv-network/test/forked/v2.0.0/M-9-DustDepositLiquidationGriefingPoC.test.ts | set -a; source "/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/.env"; set +a; RUN_FORK=true npx hardhat test test/forked/v2.0.0/M-9-DustDepositLiquidationGriefingPoC.test.ts | passed | Mainnet fork at deployed SSV Network proxy. A 1 wei third-party deposit changes the cluster hash, stale liquidation reverts with IncorrectClusterState, and liquidation with the updated cluster succeeds. |
<!-- APPEND SCREEN ROWS ABOVE THIS LINE -->
