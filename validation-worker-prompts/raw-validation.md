You are the raw-validation worker for exactly one finding.

Workspace root: {{REPO_ROOT}}

Benchmark:
- slug: {{BENCHMARK}}
- report: {{REPORT}}
- source root: {{SOURCE_ROOT}}
- raw run file to append: {{RAW}}
- append anchor: {{APPEND_ANCHOR}}
- prompt: {{PROMPT}}

Validate exactly this finding and no others:
- Finding ID: {{FINDING_ID}}
- Finding title: {{FINDING_TITLE}}
- Report id: {{REPORT_ID}}
- Report lines: {{FINDING_START_LINE}}-{{FINDING_END_LINE}} in {{REPORT}}

Finding report block:
```md
{{FINDING_BLOCK}}
```

Required evidence sources to use:
- {{README_PATH}}
- `{{SOURCE_ROOT}}/v12-findings.md` when present
- every `{{SOURCE_ROOT}}/*-scope.md` file
- every `{{SOURCE_ROOT}}/*-docs.md` file
- Relevant Solidity code under {{SOURCE_ROOT}}

Critical evidence hygiene:
- Do not look for any hidden truth, approved findings list, answer key, or benchmark scoring artifact.
- Do not read local post-hoc analysis files, validator outputs, or user-generated markdown that appears to summarize likely findings.
- Prioritize actual Solidity code and canonical benchmark docs over the report narrative.

Critical scope hygiene:
- Use the client-declared scope/docs files and `v12-findings.md` as part of Gate 1 scope review.
- If a finding is already present in `v12-findings.md`, apply this exact duplicate rule:

```text
Same root cause as V12?
├─ NO → Submit
└─ YES → Same impact?
    ├─ NO → Submit
    └─ YES → Same severity?
        ├─ NO → Submit
        └─ YES → Exclude (duplicate)
```

- Same root cause alone is not enough to exclude; same root cause + same impact + same severity is the duplicate case.

Important constraints:
- This phase is raw validation only. Do not score the run or revise the prompt.
- Append exactly one new finding block above the anchor in the raw run file, then stop.
- Do not rewrite prior finding blocks.
- Use apply_patch for file editing.
- You are not alone in the codebase. Do not revert others' edits.

Append in this exact shape:

### {{FINDING_ID}} / `{{REPORT_ID}}`
- Finding Title: {{FINDING_TITLE}}
- Decision: <Valid|Invalid|Needs Review>
- Confidence: <High|Med|Low>
- Bug Exists: <Yes|No|Unclear>
- Severity Assessment: <High|Medium|Low / QA|Low / Unclear>
- Root Cause Family: `<short-family-name>`
- Checklist Gates Passed: `<comma-separated>`
- Checklist Gates Failed: `<comma-separated or ->`
- Detailed Reason: <1 concise paragraph grounded in code and benchmark docs>
- Code Evidence: <1 concise paragraph citing the most relevant file paths and functions>

When done, reply with a concise summary and list the files you changed.
