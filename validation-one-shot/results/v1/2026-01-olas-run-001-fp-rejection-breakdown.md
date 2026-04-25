# 2026-01-olas One-Shot FP Rejection Breakdown

Source scored run:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-one-shot/results/v1/2026-01-olas-run-001.md`

Source rejected-finding mapping:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/C4_REJECTED_FINDINGS_KEY.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/C4_REJECTED_FINDINGS_KEY.json`

Method:
- Consider only one-shot `H-` / `M-` false positives from the primary KPI layer.
- False positive means: `Truth=Invalid` and run-side `Decision=Valid` with `Severity=High` or `Medium`.
- Join those false positives to the closest rejected C4 listing from `C4_REJECTED_FINDINGS_KEY.json`.

## Summary

- Total one-shot H/M false positives: `68`

| Rejection category | Count | Share |
| --- | ---: | ---: |
| `unsupported-token` | `29` | `42.6%` |
| `other` | `15` | `22.1%` |
| `known-issue-or-oos` | `10` | `14.7%` |
| `missing-or-insufficient-poc` | `7` | `10.3%` |
| `speculative-or-unproven` | `4` | `5.9%` |
| `trusted-role-or-operational` | `2` | `2.9%` |
| `by-design-or-not-integrated` | `1` | `1.5%` |

## High-Signal Families

### 1. Unsupported token / USDT assumptions

Representative false positives:
- `M-2` -> `F-76`
- `H-9` -> `F-228`
- `M-10` -> `F-228`
- `M-13` -> `F-228`
- `M-20` -> `F-228`

Interpretation:
- The one-shot worker repeatedly promoted reports that depended on non-standard ERC20 / USDT semantics.
- This is the single biggest cheap precision win.

### 2. Known issue / OOS / benchmark-listed exclusions

Representative false positives:
- `H-1` -> `F-333`
- `M-16` -> `F-333`
- `H-36` -> `F-236`
- `M-50` -> `F-307`
- `H-58` -> `F-132`

Interpretation:
- These are the findings most likely to improve with stronger emphasis on README / scope / docs / public-known-issues filtering.

### 3. Missing or insufficient PoC on overstated exploit claims

Representative false positives:
- `M-33` -> `F-443`
- `M-46` -> `F-26`
- `M-94` -> `F-56`
- `M-119` -> `F-441`
- `M-127` -> `F-441`

Interpretation:
- Several accepted reports described a plausible idea but the closest rejected C4 rationale was that impact was not actually demonstrated well enough.
- This especially shows up in permissionless nuisance / slippage-overstatement / griefing claims.

### 4. Speculative or unproven runtime harm

Representative false positives:
- `M-80` -> `F-238`
- `M-81` -> `F-238`
- `M-151` -> `F-245`
- `H-186` -> `F-238`

Interpretation:
- These generally assume a revert path, stale-path abuse, or downstream exploitability without proving the material impact.

## The `other` Bucket

The `other` bucket is not one family; it is a mixed cluster. The most useful sub-patterns inside it are:

- token-bond / staking-invariant assumptions
  - `H-4`, `H-53`
- permissionless `collectFees` / leftover-funds nuisance reports
  - `H-49`, `M-52`
- GuardCM overstatement where arbitrary interaction does not translate into real privileged impact
  - `H-103`, `H-105`
- whitelist / recovery / management-flow reports that are either by-design, uneconomic, or operationally non-material
  - `H-107`, `H-112`, `M-155`, `H-200`

## Practical Takeaway

If the goal is to raise precision without hurting recall too much, the lowest-hanging-fruit FP filters are:

1. Unsupported token / USDT semantics
2. Known issue / OOS / benchmark-doc exclusions
3. Overstated nuisance-griefing and permissionless leftover-funds reports
4. Claims that need stronger exploit evidence before earning H/M treatment
