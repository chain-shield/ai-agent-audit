# H-31 Judge Simulation

Status: Complete

## Simulated Judge Decision

- Decision: Accepted
- Severity: Critical
- Confidence: High
- Estimated payout: $15,000-$50,000 equivalent, paid in GRT on Arbitrum; exact amount is not determined by the provided docs

## Rationale

I would accept this report as a real in-scope Critical. The affected contracts and token are explicitly in scope: `GraphTallyCollector`, `PaymentsEscrow`, `GraphPayments`, `SubgraphService`, `HorizonStaking`, and L2 GRT are all listed as Arbitrum smart-contract assets in the bounty artifacts. The claimed impact maps to the program's exact Critical row: a bug that can cause significant `>$1M` user funds to be lost or stolen directly from protocol smart contracts, not including slashing.

The source code supports the report's core claim. `GraphTallyCollector._encodeRAV()` signs only `collectionId`, `payer`, `serviceProvider`, `dataService`, `timestampNs`, `valueAggregate`, and `metadata`; `receiverDestination` is not in the EIP-712 typehash or encoded digest. The public `collect()` path decodes `(SignedRAV, dataServiceCut, receiverDestination)` from caller calldata, verifies only that `msg.sender` is the RAV `dataService`, the signer is authorized for the payer, and the service provider has an active provision with that data service, then forwards the caller-supplied `receiverDestination` to `PaymentsEscrow.collect()`. `PaymentsEscrow` debits the payer escrow for `(payer, collector, receiver)` and calls `GraphPayments.collect()`, which transfers the receiver's remaining GRT to `receiverDestination` when it is nonzero.

The surrounding checks do not block the exploit. The caller-data-service check is satisfied by the attacker when the valid RAV names the attacker's data service. The active-provision check is also reachable: `HorizonStaking.provision()` allows the service provider itself to provision a verifier/data-service address, and the deployed read-only state returned `__DEPRECATED_getThawingPeriod() == 0`, so the transition restriction that previously limited arbitrary verifiers is cleared. This uses normal service-provider and payer setup, not governance, leaked keys, compromised credentials, phishing, privileged addresses, or live-chain mutation.

The issue is not merely a documentation mismatch. `SubgraphService` keeps a provider-controlled `paymentsDestination` and, on its mediated query-fee path, encodes that registered destination into the `GraphTallyCollector` payload. But `GraphTallyCollector` is itself public and accepts direct calls from the RAV data service. The receiver destination should therefore either be signed by the payer/RAV signer or derived from a receiver-controlled registry. Giving the data service an unsigned override lets it redirect the service provider's receiver share.

The embedded Foundry PoC is portable for the stated runtime. It uses the deployed Arbitrum addresses, `ARBITRUM_RPC_URL`, fork block `460062146`, and a local fork only. Its assertions are impact-focused rather than mere call success: the same signed RAV is accepted after only `receiverDestination` changes, `20,000,000` GRT is collected, `19,800,000` GRT reaches the attacker after the deployed `1%` protocol cut, payer escrow becomes zero, `tokensCollected` is consumed for the legitimate tuple, and the provider's liquid balance and stake remain unchanged. I would not reject the PoC for synthetic local-fork funding because it demonstrates the normal high-value escrow/RAV state needed for the listed `>$1M` impact.

No decisive known-issue or exclusion blocker appears in the supplied bounty artifacts. The strongest residual triage risk is not code existence, but explaining the trust boundary: the attacker must be the RAV `dataService` or control a provisioned data-service verifier. In my view that is still an unprivileged protocol-participant path, not a trusted-admin or privileged-key requirement.

## Likely Judge Questions Or Objections

- Does this affect canonical `SubgraphService` RAVs where `dataService` is the `SubgraphService` contract, or only RAVs for a malicious/provisioned Horizon data service?
- Why is a provisioned data service/verifier not treated as a trusted role whose malicious behavior is out of scope?
- Does the payer's RAV signature, metadata, or off-chain GraphTally flow ever bind or imply the intended receiver destination?
- Is the `>$1M` impact based on realistic escrow/RAV values, or only on a locally minted fork balance?
- Should this be downgraded if the provider voluntarily provisioned the attacker's data service, or if a data service is expected to choose settlement parameters?
- Is there a prior audit or private duplicate for unsigned GraphTally settlement parameters?

## Recommended Pre-Submission Edits

- Add one sentence stating that the deployed `HorizonStaking.__DEPRECATED_getThawingPeriod()` is `0`, so arbitrary verifier/data-service provisions are currently allowed.
- Clarify that the victim impact is the service provider's receiver share: payer escrow is consumed, `tokensCollected` prevents recollection, and the provider receives neither liquid GRT nor restaked GRT.
- Preempt the canonical-path objection by explaining that `SubgraphService` safely uses the provider's `paymentsDestination`, but direct `GraphTallyCollector.collect()` remains public for any RAV data service and does not enforce that registry.
- Include a compact proof note that changing only `receiverDestination` leaves `encodeRAV(rav)` and `recoverRAVSigner(signedRAV)` unchanged.
- Include the exact PoC file path, fork block, and forge command, plus the passing assertion summary, so the triager can reproduce the local-fork proof quickly.
