You are the round 4a worker for a focused V12 / prior-finding overlap sweep.

Run model: `{{WORKER_MODEL}}`
Run reasoning effort: `{{WORKER_REASONING}}`
Worker launcher: `{{WORKER_LAUNCHER}}`

Workspace root: {{REPO_ROOT}}

Benchmark:
- slug: {{BENCHMARK}}
- source root: {{SOURCE_ROOT}}
- post-R4 submission candidates: {{SUBMISSION_PATH}}
- round 4a input file: {{INPUT_PATH}}
- round 4a V12 sweep screen file to fill: {{STAGE_PATH}}

Fresh-context requirement:
- Treat this as a standalone worker run with cleared context.
- Do not rely on prior conversation, prior workers, or hidden artifacts.
- Read only the candidate findings from `{{INPUT_PATH}}`.

Mandatory context to read before deciding:
{{SCOPE_DOC_PATHS}}

Round 4a goal:
- perform a second, stricter V12 / prior-finding overlap sweep after R4 canonicalization
- exclude candidates that are not materially distinct from V12 / known prior findings
- keep candidates only when the material difference is clear enough to explain to a Code4rena judge

Critical V12 policy:
- severity mismatch is not enough to keep a candidate
- different title wording is not enough to keep a candidate
- a different PoC variant is not enough to keep a candidate
- a different function surface is not enough if the practical exploit root and harm are the same
- if the candidate has the same root cause and same practical impact as V12, exclude it even if one is labeled High and the other is labeled Medium or Low
- if the material delta cannot be stated in one crisp sentence, default to `Exclude`

Keep only if at least one of these is materially different:
- broken invariant or protocol assumption
- exploit precondition
- exploit action
- affected victim or value flow
- impact class or practical harm
- mitigation required to fix the issue

Do not keep for cosmetic differences:
- severity only
- title only
- broader or narrower wording only
- same exploit with a different numeric example
- same root cause with a slightly different accounting endpoint
- same dust / virtual-share / rounding family unless the candidate proves a distinct value flow
- same mutable live-balance / pending-yield family unless the candidate proves a distinct source of harm

Output requirements:
- preserve report order
- use `apply_patch` for editing
- replace the single append anchor with table rows
- do not modify the table header
- keep `Reason` concise and avoid `|` characters

Emit exactly these columns:

| Finding | Finding Title | Decision | Confidence | Closest V12 Finding | Material Difference | Reason Category | Reason |

Allowed `Decision` values:
- `Keep`
- `Exclude`

Allowed `Reason Category` examples:
- `keep-materially-distinct-root`
- `keep-materially-distinct-impact`
- `keep-materially-distinct-action`
- `keep-materially-distinct-value-flow`
- `exclude-v12-overlap`
- `exclude-known-issue`

Closest V12 Finding guidance:
- identify the closest V12 / prior finding by title or short description
- use `-` only if there is no plausible V12 neighbor

Material Difference guidance:
- if `Decision` is `Keep`, state the material delta in one crisp sentence
- if `Decision` is `Exclude`, state `none`

Candidates to compare:
- Read them from `{{INPUT_PATH}}`.

When done, reply with a concise summary and list the files you changed.
