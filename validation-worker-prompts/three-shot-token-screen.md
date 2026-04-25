You are the stage 2 worker for a three-shot validation run.

Workspace root: {{REPO_ROOT}}

Benchmark:
- slug: {{BENCHMARK}}
- report: {{REPORT}}
- source root: {{SOURCE_ROOT}}
- prior scope screen: {{SCOPE_PATH}}
- token screen file to fill: {{STAGE_PATH}}
- prompt baseline: {{PROMPT}}

Mandatory benchmark artifacts to read before anything else:
- {{README_PATH}}
- {{OLAS_SCOPE_PATH}}
- {{OLAS_DOCS_PATH}}
- `{{V12_FINDINGS_PATH}}` when present
- `{{V12_CHECKLIST_PATH}}` when present

Your job in stage 2:
- evaluate only the findings that stage 1 kept
- perform only the unsupported-token / unsupported-asset screen
- do not do full exploitability or severity analysis yet

Critical token-scope rules:
- unsupported ERC20 or non-standard token behavior is out of scope unless the benchmark docs explicitly say that behavior is supported
- treat USDT-style missing return values, forced zero-reset approvals, fee-on-transfer semantics, rebasing semantics, and exotic decimal quirks as out of scope unless support is explicitly declared
- do not upgrade a report merely because the token is common or popular elsewhere
- keep the finding for stage 3 only if no unsupported-token blocker is present

Decision rubric:
- `Exclude`
  - the H/M claim depends on unsupported non-standard ERC20 behavior
  - the report only matters for USDT-like or other weird-token semantics that benchmark docs do not explicitly support
- `Keep`
  - no unsupported-token blocker found

Output requirements:
- preserve the provided report order
- use `apply_patch` for editing
- replace the single append anchor with table rows
- do not modify the table header
- keep `Reason` concise and avoid `|` characters

Emit exactly these columns:

| Finding | Finding Title | Token Decision | Confidence | Reason Category | Reason |

Allowed `Reason Category` examples:
- `supported-behavior`
- `unsupported-token`
- `unsupported-erc20-approve`
- `unsupported-erc20-return`
- `unsupported-token-semantics`

Findings to screen, in order:

{{ALL_FINDINGS_BLOCKS}}

When done, reply with a concise summary and list the files you changed.
