# H-115 R8 Finding Report Review

Status: Fixed And Ready

Reviewed only H-115. The report is Immunefi submission-ready after a small reproduction wording edit that anchors `Save as` and `Run` to the `graphprotocol-contracts` repo root and the `packages/horizon` package.

Checklist result:
- Severity matches the round 3 classification: Critical.
- `Bounty Criteria Match` names the exact The Graph Immunefi Critical smart contract impact row for significant `>$1M` user funds lost or stolen directly from protocol smart contracts, excluding slashing.
- Affected assets are in scope: GraphTallyCollector, PaymentsEscrow, GraphPayments, and L2GraphToken on Arbitrum One.
- The exploit path is an unprivileged protocol-participant path: the attacker is the RAV `dataService`, not governance, an admin, a leaked key holder, or a mistaken trusted role.
- The report explains current exploitability, preconditions, threat model, root cause, exploit walkthrough, impact, mitigation, references, and feasibility limits.
- The prose count is about 838 words after excluding PoC/source code and reference-only material, under the 1,000-word cap.
- Source snippets and function-level links identify the unsigned RAV hash, caller-supplied settlement parameters, escrow debit, and payment distribution logic.
- The embedded Proof of Code matches the verified PoC file exactly and is standalone under `graphprotocol-contracts/packages/horizon/test/UnsignedRavPaymentParamsPoC.t.sol`.
- The secret Gist PoC bundle is linked in the report, is unlisted/secret, contains `README.md`, `.env.example`, and `UnsignedRavPaymentParamsPoC.t.sol`, and the Gist PoC file SHA-256 matches the verified local PoC file: `a96b103f97b82c99ffe0010f9cca214eb67815990b1550051c5f32288c973b8c`.
- The PoC is a local Arbitrum fork simulation only and does not broadcast or mutate live protocol state.

PoC verification:
- Command run from `packages/horizon` with `ARBITRUM_RPC_URL` loaded from the local audit `.env`:
  `forge test --match-path test/UnsignedRavPaymentParamsPoC.t.sol --fork-url "$ARBITRUM_RPC_URL" --fork-block-number 460064183`
- Result: passed. Foundry reported `1 passed; 0 failed; 0 skipped`.
- Key assertion coverage: the signed RAV is unchanged while attacker-selected payment parameters differ; collection drains 20,000,000 GRT from escrow, pays attacker-controlled recipients, leaves the provider balance/stake unchanged, and records the RAV as collected.

Edits made to report:
- Changed the Proof of Code `Save as` and `Run` labels to explicitly say they are from the `graphprotocol-contracts` repo root and `packages/horizon` package.
- Added and validated the secret Gist PoC bundle link.

Residual risk:
- None blocking submission. The report relies on the stated ordinary preconditions: valid high-value RAV, payer escrow balance, attacker as named data service, and active provision.
