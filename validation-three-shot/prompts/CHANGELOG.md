# Validation Prompt Changelog

## Private-client validation profile on 2026-05-20

- Added `validation_profile: private-client` prompts for R1-R9.
- Private-client mode keeps Critical/High/Medium/Low/QA/Informational findings, labels attack class and likelihood separately, and does not downgrade solely for low likelihood.
- R6 is profile-aware: Critical/High/Medium require verified PoC, while Low/QA/Informational can proceed as reportable without PoC only when evidence is strong.
- R7/R8 create bug-bounty-style reports for Critical/High/Medium and concise C4-style remediation notes for Low/QA/Informational.
- R9 simulates a rigorous private-client technical review that rejects false positives but allows labeled governance, trusted-role, user-mistake, operational, and integration risks when useful for remediation.

## Practical attack path update on 2026-05-11

- Added a profile-aware practical exploitability gate across Immunefi bounty, Code4rena bounty, and Code4rena contest prompts.
- Explicitly separates exploitability from likelihood: rare, timing-sensitive, high-capital, or low-probability states are not rejected merely for being rare.
- Bounty prompts now block or require review when impact depends on unsupported routes, victim confusion, social engineering, victim misuse, or no credible attacker-controlled route.
- Code4rena contest prompts now preserve rare-but-real H/M findings while surfacing unsupported or impractical victim-path issues as `Needs Review` / judge-risk.
- Tightened R9 judge simulation prompts so workers must produce profile-specific gate verdicts in both Markdown and JSON before recommending acceptance.

## Delegated authority trust-model update on 2026-05-08

- Hardened bounty and validation prompts after an Immunefi trust-model rejection.
- Added concise delegated-authority handling: malicious use of intentionally granted capabilities is trusted-role/OOS unless the report proves incremental unauthorized impact.
- Strengthened R5/R6/R8/R9 to reject or block PoCs/reports that only demonstrate malicious use of delegated authority.

## Immunefi secret-Gist PoC bundle update on 2026-05-07

- Added Immunefi R7 requirements to create one secret GitHub Gist per finding containing `README.md`, `.env.example`, and the exact report-ready PoC file.
- Added Immunefi R8 checks to verify the Gist is secret/unlisted, contains the expected files, matches the verified PoC, and contains no secrets, raw RPC URLs, local absolute paths, or broadcast commands.
- Added `Secret Gist PoC bundle:` and `gist_url` report/JSON fields for Immunefi bounty submissions.

## Judge simulation update on 2026-05-06

- Added R9 judge-simulation prompts for Code4rena contests, Code4rena bounties, Immunefi bounties, and the default profile.
- R9 is non-mutating: one fresh minimal worker per R8-reviewed report, writing Markdown and JSON under `validation-three-shot/judge-simulations/`.
- R9 asks workers to simulate a skeptical judge decision, likely final severity, acceptance risks, likely objections, and bounty payout estimate when bounty docs support one.
- R9 judges only the post-R8 finalized report plus benchmark/source context, and must independently investigate source code to verify the bug is real, reachable, not guarded, and not by design.

## Bounty report concision update on 2026-05-06

- Capped Immunefi and Code4rena bounty R7 reports at 1,000 prose words excluding PoC source, source-code snippets, GitHub/Etherscan links, run commands, and reference-only material.
- Added an 800-1,000 prose-word target and explicit anti-repetition guidance so workers keep exploit dossiers comprehensive without restating the same root cause, exploit path, and impact across multiple sections.
- Tightened R8 review prompts so reviewers must trim overlong or repetitive bounty reports while preserving exploit mechanics, feasibility, impact mapping, and PoC reproducibility.

## Bug bounty exploit-dossier update on 2026-05-05

- Upgraded Immunefi and Code4rena bounty R5/R6 prompts from minimal PoC tests to full reproduction packages with attacker/victim setup, exploit trigger, concrete assertions, expected results, and clean-checkout run commands.
- Upgraded bounty R7/R8 prompts from concise contest-style reports to long-form exploit dossiers with executive summary, root cause, threat model, exploit walkthrough, impact/severity mapping, full PoC package, mitigation, and references.
- Clarified that bounty reports should be complete before short; later superseded by the 2026-05-06 1,000-word prose cap.
- Kept normal Code4rena contest prompts unchanged so contest reports remain concise.

## Bounty PoC portability update on 2026-05-04

- Tightened bounty R5/R6 PoC rules so final PoCs must be copy/paste runnable from the repo's top-level `test/` folder.
- Tightened bounty R7/R8 report rules to require `Save as:` and portable `Run:` lines with relative paths and env var placeholders.
- Added explicit rejection/repair criteria for PoCs that only compile from generated nested test paths.
- Clarified that submission-facing PoC filenames must be vulnerability-descriptive and omit pipeline IDs such as `M-9` or `C-5`, even when internal artifacts keep IDs for traceability.
- Added optional Immunefi-only R3a feasibility-limitations gate between R3 and R4 so R3 stays focused on bug/severity validation.

## Runtime worker spec update on 2026-04-27

- Added `{{WORKER_LAUNCHER}}` to R1-R8 prompt headers.
- Updated controller worker payloads to emit `worker_launcher`, `worker_spawn_command_template`, and exact `worker_spawn_command` values when prompts are written to disk.
- Default launcher is `/Users/apmfree/codex-minimal-worker` so parallel workers do not load plugin MCP servers.

## Reset on 2026-04-25

- Discarded the `v2` / `v3` prompt lineage and the derived `v4` draft after recall regressed across iterations.
- Reset prompt evolution back to `v1` as the active baseline.
- Future prompt creation is now explicitly recall-first: maximize approved-root H/M recall while keeping H/M precision near or above the 50% guardrail.

## v1 -> v2

- Added an explicit evidence hierarchy and durable `Core Validation Principles` section so workers separate bug existence, root-cause independence, and severity instead of collapsing them.
- Added a mandatory scope-first stage that tells workers to read configured benchmark context docs, benchmark V12/prior-findings context whether standalone or embedded, and the shared three-shot `v12-checklist.md` before any deeper validation work.
- Strengthened the prompt so client-declared scope, known issues, and V12 duplicates are filtered before bug-existence or severity analysis.
- Reframed unsupported ERC20 / non-standard token behavior as out of scope unless the benchmark docs explicitly declare support for that exact token class or semantic.
- Added a root-cause independence gate that rejects only non-independent downstream symptoms or alternate patch sites while preserving same-root articulations that still prove the live broken invariant.
- Reframed safeguards, likelihood, governance, and by-design handling so real runtime H/M issues are not downgraded merely because exploitation needs realistic timing, expected admin / keeper workflows, or nominal-but-non-binding protections.
- Added stronger positive H/M guidance for fail-open pricing or oracle guards, missing execution-time output checks, stale authorization, missing value binding, and core accounting / liveness / reconciliation failures.
- Kept precision guardrails against unsupported non-standard asset behavior, speculative future failures, deployment / setup mistakes, documentation-only issues, and economically irrational nuisance griefing.
- Removed report-local deduping from raw validation: V12 / client-known duplicates are still filtered at scope time, but same-family report findings are now evaluated independently and left for a later cleanup pass.
