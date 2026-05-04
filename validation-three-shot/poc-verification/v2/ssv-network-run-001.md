# ssv-network Three-Shot Round 6 PoC Verification

Status: Complete
Source submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/ssv-network-run-001.md`
Source R5 PoC run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-runs/v2/ssv-network-run-001.md`
JSON summary: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-verification/v2/ssv-network-run-001.json`

## PoC Verification Results

| Finding | Finding Title | Status | Final PoC Test Path | Test Command | Verification Result | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| C-5 | Premature ETH cluster liquidation due to inconsistent fee rounding in ClusterLib.isLiquidatableWithEB | Verified PoC | /Users/apmfree/Desktop/Audit/ssv-network-9bb7b2/ssv-network/test/forked/v2.0.0/C-5-premature-liquidation-rounding.poc.test.ts | set -a; . "/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/.env"; set +a; NO_GAS_ENFORCE=true npx hardhat test test/forked/v2.0.0/C-5-premature-liquidation-rounding.poc.test.ts | passed and proves vulnerability | Mainnet fork uses approved SSVNetwork and Views addresses. Liquidator receives remaining ETH while separate rounded fee runway is still covered. |
| M-9 | Dust deposits can frontrun liquidations by invalidating caller-supplied cluster state | Verified PoC | /Users/apmfree/Desktop/Audit/ssv-network-9bb7b2/ssv-network/test/forked/v2.0.0/M-9-DustDepositLiquidationGriefingPoC.test.ts | set -a; source "/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/.env"; set +a; RUN_FORK=true FORK_USE_DEPLOYED_STATE=true FORK_CSSV_TOKEN=0xe018D31F120A637828F46aFD6c64EC099d960546 npx hardhat test test/forked/v2.0.0/M-9-DustDepositLiquidationGriefingPoC.test.ts | passed and proves vulnerability | R5 command passed. Stricter deployed-state mainnet fork also passed. A 1 wei third-party deposit changes the cluster hash, making stale public liquidation revert while updated liquidation succeeds. |
<!-- APPEND SCREEN ROWS ABOVE THIS LINE -->
