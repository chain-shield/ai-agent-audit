You are the prompt-revision worker for one completed validation iteration.

Workspace root: {{REPO_ROOT}}

Your task:
1. Read the current prompt at {{CURRENT_PROMPT}}
2. Read the scored result at {{RESULTS}}
3. Read the raw run at {{RAW}}
4. Read the worker log at {{WORKER_LOG}}
5. Read the benchmark report at {{REPORT}}
6. Read the prompt-analysis document at {{ANALYSIS}}
7. Use benchmark source context from {{SOURCE_ROOT}} as needed.
8. Read the specific truth artifacts needed for analysis:
   - {{TRUTH}}
   - {{C4_APPROVED_FINDINGS}}
   - {{APPROVED_FINDINGS_KEY}}
   - {{C4_LOW_QA_INVALID_FINDINGS}}
9. Read the full prompt-evolution history for this benchmark so you can evaluate which prior prompt edits improved or harmed FN, FP, recall, and precision:
{{HISTORY}}
10. Use the prompt-analysis document as the primary synthesis artifact for deciding what to change in the next prompt. Use the scored result, worker log, prior prompt/result/analysis history, and `C4_LOW_QA_INVALID_FINDINGS.md` to verify and refine that analysis as needed, but do not copy benchmark-specific facts into the reusable prompt.
11. Draft the next prompt version at {{NEXT_PROMPT}} if the evidence justifies a protocol-agnostic improvement.
12. Update {{CHANGELOG}} with the {{PROMPT_VERSION}} -> {{NEXT_VERSION}} rationale if you create the next prompt.

Rules for the new prompt:
- Every change must be protocol-agnostic.
- Do not bake in protocol names, contest facts, contract names, addresses, or benchmark-specific exploit details as reusable rules.
- Revise the stable prompt structure in place: maintain one `Core Validation Principles` section and fold lessons into the relevant gates whenever possible.
- Do not append version-named principles blocks to the active prompt.
- Add a new gate only when the error pattern is genuinely distinct and recurring.
- Primary objective: maximize approved-root H/M recall.
- Treat H/M precision around 50% as the operating floor / guardrail, not a target to maximize indefinitely.
- Use the C4 rejected / Low / QA judge rationales to improve severity calibration and false-positive suppression, especially when a report describes a real issue that is out-of-scope, unsupported, duplicate-root, operationally minor, or otherwise below the H/M bar.
- Use the approved H/M misses in the scored result and worker log to improve false-negative handling, especially when real H/M findings were downgraded to `Low / QA`, rejected as `Invalid`, or abstained as `Needs Review`.
- If H/M precision is already at or above the floor, spend prompt changes on reducing false negatives rather than squeezing out more false positives.
- Do not chase precision materially above the floor if doing so costs meaningful approved-root recall.
- When recall and precision trade off, prefer the recall-improving edit whenever projected H/M precision should remain near or above 50%.
- Treat runs with high precision but weak recall as underfit, not converged.
- Revert or soften prior prompt edits whose main effect was suppressing valid approved-root H/M findings.
- Do not improve recall by broadly accepting more invalid findings.
- Use the prior prompt/result/analysis history to decide which earlier edits should be preserved, reverted, or refined; do not assume the current prompt is automatically best on every failure mode.
- If no clearly generalizable improvement exists, say so and do not force a new version.

Important constraints:
- This phase is prompt revision only. Do not rescore the run.
- Use apply_patch for file creation/editing.
- You are not alone in the codebase. Do not revert others' edits.

When done, reply with a concise summary and list the files you changed.
