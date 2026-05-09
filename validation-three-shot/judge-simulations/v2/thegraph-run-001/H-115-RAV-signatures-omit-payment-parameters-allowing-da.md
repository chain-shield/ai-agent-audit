# H-115 Judge Simulation

Status: Complete

## Simulated Judge Decision

- Decision: Accepted
- Severity: Critical
- Confidence: High
- Estimated payout: Critical range, likely $15,000-$50,000 equivalent paid in GRT on Arbitrum; exact amount not determinable from the provided docs.

## Rationale

The impacted contracts are in scope: `GraphTallyCollector`, `PaymentsEscrow`, `GraphPayments`, `HorizonStaking`, and L2 GRT are all listed Arbitrum smart-contract assets. The affected code path exists in the in-scope source. `GraphTallyCollector._encodeRAV()` signs only `collectionId`, `payer`, `serviceProvider`, `dataService`, `timestampNs`, `valueAggregate`, and `metadata`, while `_collect()` decodes `dataServiceCut` and `receiverDestination` from caller-supplied bytes and forwards them to `PaymentsEscrow.collect()`. `PaymentsEscrow.collect()` debits the payer's escrow for the collector/provider tuple and passes the same unsigned parameters into `GraphPayments.collect()`, which pays the selected data-service cut and sends the remaining receiver amount to `receiverDestination` when nonzero.

The surrounding guards do not neutralize the issue. The collector checks that `msg.sender` is the RAV `dataService`, that the RAV signer is authorized by the payer, that the service provider has an active provision with that data service, and that the escrow has enough balance. None of those checks binds the payment destination, data-service cut, or payment type to the signed authorization. A read-only fork check at block `460064183` showed the Horizon transition restriction was cleared (`__DEPRECATED_getThawingPeriod() == 0`), so arbitrary verifier/data-service provisions are permitted, not only the deployed `SubgraphService` contract. The submitted PoC passed on the stated Arbitrum fork and demonstrated an unchanged signed RAV draining escrow while the intended provider received no balance or stake increase.

The strongest contrary fact is that the normal `SubgraphService.collect()` route constrains these parameters: it encodes the curation cut and the service provider's stored `paymentsDestination` before calling `GraphTallyCollector`. That wrapper lowers the risk for the canonical subgraph-service flow, but it does not remove the generic in-scope collector's direct external surface. `GraphTallyCollector` is separately scoped and documented as a collector usable with a GraphTally RAV where the caller is the RAV data service.

The claimed impact maps to the Critical smart-contract row when the uncollected RAV and escrowed balance exceed $1M: direct loss/theft of user GRT from protocol smart contracts, excluding slashing. The report proves the mechanics and does not rely on governance, leaked keys, phishing, oracle manipulation, live-chain mutation, or privileged access. No matching known issue surfaced in a targeted search of the provided Horizon audit artifacts.

## Likely Judge Questions Or Objections

- Does the report prove that the direct `GraphTallyCollector` path is intended for arbitrary Horizon data services, instead of only the deployed `SubgraphService` wrapper?
- Is the RAV `dataService` a trusted counterparty selected by the service provider, making malicious data-service behavior a by-design trust assumption rather than an untrusted attacker path?
- Does the current value at risk satisfy the program's `>$1M` Critical threshold? The PoC uses 20,000,000 GRT, but the report should avoid relying on stale GRT/USD assumptions and should show a current >$1M amount or scalable live exposure.
- The local fork PoC mints/deals victim funds into escrow; a triager may ask for evidence that comparable high-value escrow/RAV balances are realistic in production.
- The normal `SubgraphService` route derives the destination from `paymentsDestination[indexer]`; judges may ask for an explicit explanation of why that wrapper does not fully scope the collector's intended use.

## Recommended Pre-Submission Edits

- Add a short threat-model paragraph distinguishing the direct `GraphTallyCollector` API from the `SubgraphService` wrapper, including the fork-verified fact that arbitrary verifier provisions are enabled.
- Update the PoC or impact math so the demonstrated amount clearly exceeds $1M at current GRT pricing, or explicitly justify Critical using scalable escrow/RAV exposure.
- State that the service provider's active provision is an ordinary protocol precondition and does not authorize the data service to redirect the provider's receiver payout.
- Add one sentence explaining that no prior audit/known-issue match was found in the public Horizon audit artifacts, while acknowledging Immunefi may still check private duplicates.
