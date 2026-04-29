You are the scoring worker for one completed three-shot validation run.

Workspace root: {{REPO_ROOT}}

Your task:
1. Read the completed three-shot validation run at {{RUN_PATH}}
2. Read the benchmark truth file at {{TRUTH_PATH}}
3. Read the benchmark report at {{REPORT}}
4. Use benchmark source context from {{SOURCE_ROOT}} only when needed to resolve ambiguity
5. Read these canonical truth artifacts:
   - {{C4_APPROVED_FINDINGS}}
   - {{APPROVED_FINDINGS_KEY}}
6. Write a scored result report to {{RESULT_PATH}}

Scoring rubric:
- score the primary KPI layer on `H-` / `M-` rows only
- truth-positive = rows marked `Valid` in `APPROVED_FINDINGS_KEY.md`
- run-positive = rows marked `Decision=Valid` with `Severity Assessment=High` or `Medium`
- run-side `Valid` rows scored as `Low / QA` or `Low / Unclear`, plus `Invalid` rows, count as negatives
- `Needs Review` remains an abstention bucket outside `TP` / `FP` / `TN` / `FN`
- also report unique canonical H/M root-cause coverage

Required output structure:
- benchmark
- prompt version tested
- run id
- source three-shot run path
- goal check section
  - remind the reader the target is roughly `50%` to `60%` H/M precision with recall as close to `100%` as possible
- scoring section
  - H/M confusion matrix
  - H/M recall
  - H/M precision
  - H/M specificity
  - H/M abstentions / `Needs Review`
  - unique approved H/M findings present in the report
  - unique approved H/M findings accepted as `Valid` with `High` / `Medium` severity
  - present-root recall
  - total canonical approved C4 H/M findings
  - end-to-end unique recall
  - truth-valid H/M findings missed
  - truth-invalid H/M findings incorrectly accepted
- per-finding decision table with:
  - finding id
  - title
  - decision
  - confidence
  - bug exists
  - severity
  - truth
  - matching C4 listing
  - root cause family

Important constraints:
- This phase is scoring only. Do not revise the prompt.
- Use apply_patch for file creation/editing.
- Be explicit about assumptions if any truth mapping is ambiguous.

When done, reply with a concise summary and list the files you changed.
