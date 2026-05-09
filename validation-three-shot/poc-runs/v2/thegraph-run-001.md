# thegraph Three-Shot Round 5 PoC Generation

Status: Complete
Source submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/thegraph-run-001.md`
JSON summary: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-runs/v2/thegraph-run-001.json`

## PoC Results

| Finding | Finding Title | Status | PoC Test Path | Test Command | Test Result | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| H-31 | Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts | PoC Created | /Users/apmfree/Desktop/Audit/thegraph-023b7fdc8558/graphprotocol-contracts/packages/horizon/test/UnsignedRavPayoutDestinationPoC.t.sol | cd graphprotocol-contracts/packages/horizon && export ARBITRUM_RPC_URL="<arbitrum rpc url>" && forge test --match-path test/UnsignedRavPayoutDestinationPoC.t.sol --fork-url "$ARBITRUM_RPC_URL" --fork-block-number 460062146 | passed: 1 test passed, 0 failed | Arbitrum fork at block 460062146. Same signed RAV accepted with attacker receiverDestination. 20,000,000 GRT escrow drained, 19,800,000 GRT reached attacker after 1 percent protocol cut, provider balance and stake unchanged. |
| H-115 | RAV signatures omit payment parameters allowing data service to redirect or skim collections | PoC Created | packages/horizon/test/UnsignedRavPaymentParamsPoC.t.sol | cd packages/horizon && export ARBITRUM_RPC_URL="<arbitrum rpc url>" && forge test --match-path test/UnsignedRavPaymentParamsPoC.t.sol --fork-url "$ARBITRUM_RPC_URL" --fork-block-number 460064183 | passed | Arbitrum fork block 460064183. Same signed RAV collected with unsigned 50 percent dataServiceCut and attacker receiverDestination, draining escrow while provider balance and stake stay unchanged. |
<!-- APPEND SCREEN ROWS ABOVE THIS LINE -->
