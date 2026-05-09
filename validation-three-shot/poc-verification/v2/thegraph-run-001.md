# thegraph Three-Shot Round 6 PoC Verification

Status: Complete
Source submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/thegraph-run-001.md`
Source R5 PoC run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-runs/v2/thegraph-run-001.md`
JSON summary: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-verification/v2/thegraph-run-001.json`

## PoC Verification Results

| Finding | Finding Title | Status | Final PoC Test Path | Test Command | Verification Result | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| H-31 | Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts | Verified PoC | /Users/apmfree/Desktop/Audit/thegraph-023b7fdc8558/graphprotocol-contracts/packages/horizon/test/UnsignedRavPayoutDestinationPoC.t.sol | cd graphprotocol-contracts/packages/horizon && forge test --match-path test/UnsignedRavPayoutDestinationPoC.t.sol --fork-url "$ARBITRUM_RPC_URL" --fork-block-number 460062146 | passed and proves vulnerability | Arbitrum fork passed 1 test. Same signed RAV with changed receiverDestination redirected 19,800,000 GRT to attacker, emptied escrow, and left provider balance and stake unchanged. |
| H-115 | RAV signatures omit payment parameters allowing data service to redirect or skim collections | Verified PoC | packages/horizon/test/UnsignedRavPaymentParamsPoC.t.sol | cd packages/horizon && forge test --match-path test/UnsignedRavPaymentParamsPoC.t.sol --fork-url "$ARBITRUM_RPC_URL" --fork-block-number 460064183 | passed and proves vulnerability | Arbitrum fork block 460064183. Valid RAV signature excludes dataServiceCut and receiverDestination. Malicious data service collected 20M GRT escrow into itself and attacker destination while provider received zero. |
<!-- APPEND SCREEN ROWS ABOVE THIS LINE -->
