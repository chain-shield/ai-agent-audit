# 2025-11-megapot run-001 Three-Shot Validation Results

Status: Completed
Completed: 2026-05-27T04:49:01Z

Source audit report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/2025-11-megapot/report/audit-report.md`
Source root: `/Users/apmfree/Desktop/Audit/2025-11-megapot-538649/2025-11-megapot`

## Summary

- R4a kept 13 submission candidates after V12 overlap sweep.
- R5 created 13 PoCs.
- R6 verified 13 PoCs: 12 verified, 1 fixed and verified, 0 invalid.
- R7 created 13 submission-ready reports.
- R8 reviewed 13 reports: 9 ready, 4 fixed and ready, 0 need further review.
- R9 simulated judge review for 13 reports: 13 accepted, 1 High, 12 Medium.

## R9 Judge Simulation Decisions

| Finding | Judge Decision | Judge Severity | Confidence | Final Report | R8 Review | R9 Simulation |
| --- | --- | --- | --- | --- | --- | --- |
| H-1 | Accepted | Medium | Medium | `H-1-Arbitrary-bridge-call-leaves-reusable-USDC-allowan.md` | `H-1-Arbitrary-bridge-call-leaves-reusable-USDC-allowan.md` | `H-1-Arbitrary-bridge-call-leaves-reusable-USDC-allowan.md` |
| H-12 | Accepted | High | High | `H-12-Arbitrary-bridge-call-lets-a-winning-ticket-holder.md` | `H-12-Arbitrary-bridge-call-lets-a-winning-ticket-holder.md` | `H-12-Arbitrary-bridge-call-lets-a-winning-ticket-holder.md` |
| H-60 | Accepted | Medium | High | `H-60-ScaledEntropyProvider-reuses-one-entropy-seed-for.md` | `H-60-ScaledEntropyProvider-reuses-one-entropy-seed-for.md` | `H-60-ScaledEntropyProvider-reuses-one-entropy-seed-for.md` |
| M-2 | Accepted | Medium | High | `M-2-ECDSA-only-bridge-claims-permanently-lock-tickets.md` | `M-2-ECDSA-only-bridge-claims-permanently-lock-tickets.md` | `M-2-ECDSA-only-bridge-claims-permanently-lock-tickets.md` |
| M-5 | Accepted | Medium | High | `M-5-Changing-entropy-provider-while-a-drawing-is-pendi.md` | `M-5-Changing-entropy-provider-while-a-drawing-is-pendi.md` | `M-5-Changing-entropy-provider-while-a-drawing-is-pendi.md` |
| M-10 | Accepted | Medium | High | `M-10-No-referral-winner-share-is-credited-to-the-curren.md` | `M-10-No-referral-winner-share-is-credited-to-the-curren.md` | `M-10-No-referral-winner-share-is-credited-to-the-curren.md` |
| M-14 | Accepted | Medium | High | `M-14-Mid-drawing-payout-calculator-update-causes-active.md` | `M-14-Mid-drawing-payout-calculator-update-causes-active.md` | `M-14-Mid-drawing-payout-calculator-update-causes-active.md` |
| M-18 | Accepted | Medium | High | `M-18-Changing-entropy-providers-can-overwrite-pending-r.md` | `M-18-Changing-entropy-providers-can-overwrite-pending-r.md` | `M-18-Changing-entropy-providers-can-overwrite-pending-r.md` |
| M-20 | Accepted | Medium | High | `M-20-Bridge-ticket-purchases-use-live-global-ticketPric.md` | `M-20-Bridge-ticket-purchases-use-live-global-ticketPric.md` | `M-20-Bridge-ticket-purchases-use-live-global-ticketPric.md` |
| M-27 | Accepted | Medium | High | `M-27-Tier-zero-winners-are-excluded-from-settlement-obl.md` | `M-27-Tier-zero-winners-are-excluded-from-settlement-obl.md` | `M-27-Tier-zero-winners-are-excluded-from-settlement-obl.md` |
| M-83 | Accepted | Medium | High | `M-83-Bonusball-ranges-above-the-bit-packing-boundary-ca.md` | `M-83-Bonusball-ranges-above-the-bit-packing-boundary-ca.md` | `M-83-Bonusball-ranges-above-the-bit-packing-boundary-ca.md` |
| M-91 | Accepted | Medium | High | `M-91-Payout-calculator-rotation-reprices-already-initia.md` | `M-91-Payout-calculator-rotation-reprices-already-initia.md` | `M-91-Payout-calculator-rotation-reprices-already-initia.md` |
| M-101 | Accepted | Medium | High | `M-101-Updating-the-payout-calculator-breaks-already-sett.md` | `M-101-Updating-the-payout-calculator-breaks-already-sett.md` | `M-101-Updating-the-payout-calculator-breaks-already-sett.md` |

## Artifact Directories

- Final reports: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/finding-reports/v2/2025-11-megapot-run-001`
- R8 reviews: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/finding-report-reviews/v2/2025-11-megapot-run-001`
- R9 judge simulations: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/judge-simulations/v2/2025-11-megapot-run-001`
- R5 PoC summary: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-runs/v2/2025-11-megapot-run-001.md`
- R6 PoC review summary: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/poc-verification/v2/2025-11-megapot-run-001.md`

## Main R9 Pre-Submission Caveats

- H-1: accepted as Medium, not High; tighten stale-allowance attacker framing before submission.
- H-12: accepted as High; explicitly state the attacker needs a bridge-custodied ticket with nonzero claimable winnings.
- H-60: accepted as Medium; strengthen reachability/EV quantification if submitting.
- M-5, M-14, M-20, M-91, M-101: accepted as Medium, but explicitly cite the contest's admin-change/prior-drawing scope to avoid trusted-role rejection.
- M-18: accepted as Medium; cite Pyth V2 provider-scoped sequence namespace.
- M-27: accepted as Medium; acknowledge non-default tier-zero payout configuration.
- M-83: accepted as Medium; distinguish from V12 `uint8` downcast and cite the `normalMax + bonusball <= 255` packing boundary.
