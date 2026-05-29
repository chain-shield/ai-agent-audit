# Code4rena Corpus Archive Status

Generated: 2026-05-25

## Public Layer

- Public audits scanned from Code4rena API: 475
- Corpus scope: all Code4rena leagues/codebase types from 2024 through 2026
- Competitions included: 175
- Final reports captured: 130
- Parsed accepted H/M findings from final reports: 1395
- Previously archived Solidity/EVM subset: 143 competitions, 106 final reports, 1188 parsed accepted H/M findings
- Full multi-codebase public refresh command: `python3 scripts/archive_code4rena_corpus.py discover --out benchmarks/code4rena-corpus --year-start 2024 --year-end 2026 --all-leagues`

The public archive is in:

```text
benchmarks/code4rena-corpus/
```

Each competition has:

- `competition.json`
- `final_report.html` when public
- `final_findings.json` when public
- `accepted_findings.md` with readable accepted H/M findings when available
- `rejected_primaries.md` with readable rejected-primary summaries when available
- `benchmark_ground_truth.md` combining accepted and rejected benchmark material
- `submissions/README.md` with the authenticated capture endpoint

## Authenticated Layer

Code4rena primary submissions require login. Public requests to submission pages and `/api/v1/audits/{uid}/submissions/csv` return missing-authentication responses.

Authenticated browser capture writes:

- `submissions/all.jsonl`
- `submissions/primaries.jsonl`
- `submissions/rejected_primaries.jsonl`

The CSV importer is ready:

```bash
python3 scripts/archive_code4rena_corpus.py import-submissions-csv \
  --out benchmarks/code4rena-corpus \
  --slug 2026-03-intuition \
  --csv /path/to/downloaded/submissions.csv
```

Rejected primaries are categorized with the taxonomy in:

```text
benchmarks/code4rena-corpus/schemas/submission.schema.json
```

## Current Status

The Codex Chrome connector is working. Authenticated capture helper:

```text
scripts/code4rena_authenticated_browser_scrape.mjs
```

Authenticated scrape progress as of 2026-05-25 21:43 EDT:

- Captured competitions: 111
- Captured competitions with primary rows: 91
- Captured explicit-empty competitions: 20
- Primary submission rows captured: 4943
- Accepted primary rows captured: 1345
- Rejected/non-accepted primary rows captured: 3598
- Restricted live-code competitions: 1 (`2026-04-k2`)
- GitHub validation repo fallback captures: 31
- Submissions-route unavailable/deferred competitions: 63 (full list in `authenticated-scrape-status.jsonl`; latest classified: `2024-02-tapioca-invitational`, `2024-02-spectra`, `2024-02-unistaker-infrastructure`, `2024-02-wise-lending`, `2024-02-thruster-invitational`, `2024-02-althea-liquid-infrastructure`, `2024-02-ai-arena`, `2024-02-hydradx`, `2024-01-init-capital-invitational`, `2024-01-canto-invitational`, `2024-01-decent`, `2024-01-nftperp-invitational`, `2024-01-saltyio`, `2024-01-opus`, `2024-01-curves`, `2024-01-renft`)
- Temporarily blocked/deferred competitions: 0
- Generic scrape errors: 0
- Remaining authenticated scrape targets: 0

The 2024-2026 manifest queue is fully classified. Large contests that exposed submissions were captured with resumable detail chunks via `maxDetails`. Older 2024 browser submissions routes that redirect to the Code4rena News page or return a Next.js not-found route were marked as deferred only after checking that no public validation repo fallback was exposed in the Code4rena GitHub org.

```bash
python3 scripts/archive_code4rena_corpus.py import-validation-repo \
  --out benchmarks/code4rena-corpus \
  --slug 2024-11-nibiru
```

Direct browser navigation to `/api/v1/audits/{uid}/submissions` currently returns `net::ERR_BLOCKED_BY_CLIENT`.

The validation fallback now uses authenticated `gh api` for GitHub issue metadata when available, avoiding the unauthenticated GitHub API rate limit. The public `code-423n4` GitHub org currently exposes 31 validation repos, all of which have been imported.

No authenticated scrape targets remain in the current manifest.

## Ground Truth Markdown Export

Generated: 2026-05-26 15:26 EDT

Durable local benchmark artifacts now live under each contest's `ground_truth/` directory:

- `ground_truth/accepted/<finding-id>.md`
- `ground_truth/rejected/<submission-id>.md`
- `ground_truth/accepted_findings.md`
- `ground_truth/rejected_primaries.md`
- `ground_truth/index.md`

Convenience copies also live in each contest root:

- `accepted_findings.md`
- `rejected_primaries.md`
- `benchmark_ground_truth.md`

Corpus-level entrypoint:

```text
benchmarks/code4rena-corpus/benchmark_manifest.jsonl
```

Export command:

```bash
python3 scripts/archive_code4rena_corpus.py export-ground-truth-markdown \
  --out benchmarks/code4rena-corpus
```

Export counts:

- Accepted H/M markdown files: 1395
- Rejected primary markdown files: 3598
- Contests with explicit zero accepted-H/M marker files: 58
- Contests with explicit zero rejected-primary marker files: 88
- Accepted parser fallbacks: 0
- Rejected summaries without usable text: 0

The markdown export strips Proof-of-Concept sections from accepted findings, reduces rejected primaries to headline/summary/rejection reason, and stores Code4rena/GitHub URLs as metadata while relying on local snapshot paths for durability.

Corpus validation checked every contest aggregate file, every accepted finding markdown path, every rejected primary markdown path, root/`ground_truth` convenience-copy parity, and stale raw report text files. Current validation issues: 0.

An independent read-only subagent audit also checked 175 competitions, 175 `manifest.jsonl` rows, 175 `benchmark_manifest.jsonl` rows, 1,225 required artifacts, 1,395 accepted markdown files, 3,598 rejected markdown files, 3,713 unique source snapshot paths, and 700 benchmark manifest path fields. Current P0/P1 benchmark-blocking issues: 0. Remaining P2 cleanup: 3 rejected summaries are title-only or weak because the archived source has little non-PoC narrative.

The old raw `final_report.txt` files were removed because they were hard to read and duplicated data derivable from `final_report.html`.
