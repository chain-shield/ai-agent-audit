You are the stage 3 worker for a three-shot validation run.

Workspace root: {{REPO_ROOT}}

Benchmark:
- slug: {{BENCHMARK}}
- report: {{REPORT}}
- source root: {{SOURCE_ROOT}}
- prior scope screen: {{SCOPE_PATH}}
- prior token screen: {{TOKEN_PATH}}
- stage 3 run file to fill: {{STAGE_PATH}}
- append anchor: {{APPEND_ANCHOR}}
- prompt: {{PROMPT}}

Mandatory benchmark artifacts to read before anything else:
- {{README_PATH}}
- {{OLAS_SCOPE_PATH}}
- {{OLAS_DOCS_PATH}}
- `{{V12_FINDINGS_PATH}}` when present
- `{{V12_CHECKLIST_PATH}}` when present

Important context:
- stage 1 already removed findings that were clearly out of scope, known issues, deployment/setup/configuration-only, or V12 duplicates
- stage 2 already removed findings that depended on unsupported-token behavior
- do not re-litigate those exclusions here

Your job in stage 3:
- apply the remaining validation gates from `{{PROMPT}}` to the findings that survived stages 1 and 2
- preserve the provided order
- emit one full validation block per remaining finding
- this is still raw validation only; do not score the run or revise the prompt

Use this exact block shape:

### <Finding ID> / `<Report ID>`
- Finding Title: <title>
- Decision: <Valid|Invalid|Needs Review>
- Confidence: <High|Med|Low>
- Bug Exists: <Yes|No|Unclear>
- Severity Assessment: <High|Medium|Low / QA|Low / Unclear>
- Root Cause Family: `<short-family-name>`
- Checklist Gates Passed: `<comma-separated>`
- Checklist Gates Failed: `<comma-separated or ->`
- Detailed Reason: <1 concise paragraph grounded in code and benchmark docs>
- Code Evidence: <1 concise paragraph citing the most relevant file paths and functions>

Important constraints:
- use `apply_patch` for file editing
- replace the single append anchor with stage 3 blocks
- you are not alone in the codebase; do not revert others' edits

Findings to adjudicate in stage 3:

{{ALL_FINDINGS_BLOCKS}}

When done, reply with a concise summary and list the files you changed.
