# thegraph Three-Shot Submission Candidates run-001

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/thegraph-run-001.md`
Canonicalization screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/dedup-screens/v2/thegraph-run-001.md`

## Canonicalization Summary

- Candidate H/M findings before R4: `3`
- Kept after R4 canonicalization: `2`
- Dropped by R4 cleanup: `1`
- Dropped finding ids: `H-116`

## Root Cause Groups

- `unsigned-rav-payment-params`: H-115
- `unsigned-rav-payout-destination`: H-31, H-116 -> H-31

## Submission Candidates

### H-31 / `B1QFkkzj9DYXaowCh7Vf_`
- Finding Title: Unsigned receiverDestination lets the data service redirect GraphTallyCollector payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `unsigned-rav-payout-destination`
- Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged protocol-participant path, exact direct-theft impact row, no listed exclusion`
- Checklist Gates Failed: `-`
- Detailed Reason: The RAV binds the payer, service provider, and data service but not the service provider's payout destination. A malicious data service named in a valid high-value RAV can call the in-scope collector directly with an attacker-controlled destination, consuming the aggregate and causing escrowed GRT to be paid away from the service provider; for balances above $1M this maps cleanly to the Critical direct-theft row.
- Code Evidence: `GraphTallyCollector._collect()` decodes `receiverDestination` from caller data and forwards it to `PaymentsEscrow.collect()`, while `_encodeRAV()` hashes only RAV fields and omits the destination; `SubgraphService.setPaymentsDestination()` shows the intended provider-controlled destination is separate from caller calldata.

### H-115 / `ehVpD-cV2madnpbmgU4kx`
- Finding Title: RAV signatures omit payment parameters allowing data service to redirect or skim collections
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Critical
- Root Cause Family: `unsigned-rav-payment-params`
- Bounty Criteria Match: critical (smart contract): A bug that could cause significant (>$1M) User funds to be lost or stolen directly from protocol smart contracts (not including slashing)
- Checklist Gates Passed: `in-scope asset, code path exists, unprivileged protocol-participant path, exact direct-theft impact row, no listed exclusion`
- Checklist Gates Failed: `-`
- Detailed Reason: The signed RAV omits payment-affecting execution parameters, while `GraphTallyCollector` accepts those parameters from the data service caller and forwards them into escrow settlement. A malicious data service with a valid high-value RAV can set a 100% data-service cut and/or attacker payout destination, causing escrowed GRT to be stolen or misdirected; this is direct theft from an in-scope protocol smart contract when value exceeds $1M.
- Code Evidence: `GraphTallyCollector._collect()` decodes `dataServiceCut` and `receiverDestination` from `_data` and forwards `_paymentType` to `PaymentsEscrow.collect()`, while `GraphTallyCollector._encodeRAV()` hashes only `collectionId`, `payer`, `serviceProvider`, `dataService`, `timestampNs`, `valueAggregate`, and `metadata`; `GraphPayments.collect()` then pays the chosen cut/destination.
