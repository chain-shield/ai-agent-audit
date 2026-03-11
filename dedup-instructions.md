See full security audit report for olas here: 2026-01-olas/report/audit-report.md

There's a ton of findings here, lets first triage and dedup the top 50 findings.

Please deduplicate the first 50 findings (1-50) findings in the following format below.

Place results in olas-dedup-findings.md

**EXAMPLE OF HOW TO GENERATE DEDUP REPORT**

```markdown
# 2025 11 brix money - Deduplicated Findings Report
## Commit hash: 79e36aeda1b0091fa3ecd96f398517b31603f5d2

## Summary of VERIFIED Findings
- **Total Findings**: 18
- **High Severity**: 7 (H-9, H-11, H-32, H-65, H-75, H-85, H-88)
- **Medium Severity**: 11 (M-7, M-15, M-18, M-19, M-20, M-29, M-39, M-42, M-45, M-63)

### Number of Findings (After Deduplication, Scope Review, and Validation Gates)
- C: 0
- H: 14 (1 Out of Scope - Centralization Risk, 1 Invalid - Speculation)
- M: 33 (3 Out of Scope - Centralization Risk, 2 Invalid - Speculation/Unsupported Token)
- L: 0 (excluded)
- I: 0

**Total: 47 unique findings**
**Valid & In Scope: 40 findings (12 High, 28 Medium)**
**Out of Scope: 4 findings (1 High, 3 Medium) - Centralization/Governance Risks**
**Invalid: 3 findings (1 High, 2 Medium) - Failed Validation Gates (GATE 6: Unsupported Token, GATE 7: Speculation)**

---

## High Severity Findings

### [H-9]. Blacklist Bypass via Cross-chain Unstake
**Pattern**: AccessControlOrAuthByPass
**Duplicates**: M-46, M-47, M-64, H-90
**Status**: Valid
**Privilege**: RequiresRole

..etc..
....
```