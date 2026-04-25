You are the scoring worker for one completed validation run.

Workspace root: {{REPO_ROOT}}

Your task:
1. Read the raw run at {{RAW}}
2. Read the benchmark truth file at {{TRUTH}}
3. Score the raw run against the truth file.
4. Update the scoring section inside {{RESULTS}} by replacing only the content between:
   - {{SCORING_SECTION_START}}
   - {{SCORING_SECTION_END}}
5. Preserve the existing run-status header and per-finding decision table.

Optional context:
- benchmark source root: {{SOURCE_ROOT}}
- benchmark report: {{REPORT}}

Required output:
- begin the scoring section with `{{SCORING_COMPLETE_MARKER}}`
- benchmark name
- prompt version tested
- run id
- scoring scope, with the primary confusion matrix computed on H/M findings only under this rubric:
  - truth-positive = rows marked `Valid` in `APPROVED_FINDINGS_KEY.md`
  - run-positive = rows marked `Decision=Valid` with `Severity Assessment=High` or `Medium`
  - run-side `Valid` rows scored as `Low / QA` or `Low / Unclear`, plus `Invalid` rows, count as negatives
  - `Needs Review` remains an abstention bucket outside `TP` / `FP` / `TN` / `FN`
- TP, FP, TN, FN on that severity-aware H/M subset
- recall, precision, specificity on the H/M subset
- abstentions / Needs Review on the H/M subset, if any
- total canonical approved C4 H/M findings
- unique approved H/M findings present in the report
- unique approved H/M findings accepted as Valid with High / Medium severity
- present-root recall
- end-to-end unique recall
- real H/M findings missed
- non-H/M findings incorrectly accepted as H/M
- checklist gates causing the most mistakes
- root-cause families causing false negatives and false positives
- short plain-English summary

Important constraints:
- This phase is scoring only. Do not revise the prompt.
- Use apply_patch for file creation/editing.
- Be explicit about assumptions if the truth mapping is incomplete.
- Replace only the scoring section content; do not delete the existing per-finding decision table above it.

When done, reply with a concise summary and list the files you changed.
