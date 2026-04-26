# 2026-01-olas R4 Canonicalization Check run-003

Prompt version: `v2`
Benchmark: `2026-01-olas`
Run id: `run-003`

Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/2026-01-olas-run-003.md`
Canonicalization screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/dedup-screens/v2/2026-01-olas-run-003.md`
Submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/2026-01-olas-run-003.md`

## Canonicalization Summary

- Candidate H/M findings before R4: `66`
- Kept after R4 canonicalization: `44`
- Dropped as duplicate-equivalent: `22`
- Dropped finding ids: `M-32, H-36, M-54, H-61, H-69, M-81, H-85, H-89, H-93, H-95, M-97, M-100, M-108, H-114, H-116, H-118, M-119, H-125, M-156, H-157, M-194, H-195`

## Metric Check

Pre-R4 assembled run:
- H/M finding-level recall: `91.3%` (`42 / 46`)
- H/M finding-level precision: `63.6%` (`42 / 66`)
- Present-root recall: `86.7%` (`13 / 15`)
- End-to-end unique recall: `56.5%` (`13 / 23`)

Post-R4 canonical submission set:
- H/M finding-level recall: `50.0%` (`23 / 46`)
- H/M finding-level precision: `52.3%` (`23 / 44`)
- Present-root recall: `86.7%` (`13 / 15`)
- End-to-end unique recall: `56.5%` (`13 / 23`)

## Conclusion

`R4` canonicalization reduced the candidate set from `66` to `44` without reducing unique-root recall.

The raw finding-level recall drop is expected because this pass intentionally collapses true-positive duplicate-equivalent candidates. The submission-oriented recall checks stayed flat.
