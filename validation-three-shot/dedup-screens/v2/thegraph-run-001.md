# thegraph Three-Shot Round 4 Canonicalization Screen

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/thegraph-run-001.md`
Output submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/thegraph-run-001.md`

## Decisions

| Finding | Finding Title | Decision | Confidence | Canonical Finding | Root Cause Group | Report Planning Note | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H-31 | Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts | Keep | High | - | unsigned-rav-payout-destination | few-reports | keep-distinct-action | Keep separate from H-115 because this is the focused receiverDestination theft path, while H-115 also covers unsigned cut and paymentType choices with broader mitigation scope. |
| H-115 | RAV signatures omit payment parameters allowing data service to redirect or skim collections | Keep | High | - | unsigned-rav-payment-params | few-reports | keep-distinct-action | Keep separate from H-31 because it adds unsigned dataServiceCut and paymentType manipulation, so the exploit action and mitigation are materially broader. |
| H-116 | Unsigned receiverDestination lets GraphTallyCollector.collect redirect service-provider payouts | Drop | High | H-31 | unsigned-rav-payout-destination | one-report | drop-duplicate-equivalent | Same receiverDestination path, same malicious dataService action, same direct theft invariant, and same destination-binding mitigation as H-31, which is at least as clear and complete. |
