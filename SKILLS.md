# Skills: Deduplicating Audit Findings (LLM-Reproducible Playbook)

This repository contains a **script-assisted** workflow for deduplicating a large audit report into a smaller set of **root-cause unique** findings, while preserving a **mechanical guarantee** that *every original finding ID is accounted for exactly once*.

## Goal
Given an audit report with many findings (e.g., `H-12`, `M-54`, `L-168`), produce a deduped report where:
- Each original ID appears **exactly once** (either as a **primary** or a **duplicate** under a primary).
- Groups are formed by **root cause**, not by title similarity.
- You can mechanically prove coverage + uniqueness.

## Files in this repo (reference implementation)
- Source report: `2026-01-olas/report/audit-report.md`
- Dedup output: `olas-dedup-findings.md`
- Mechanical verifier: [`dedup_verify.py`](dedup_verify.py)
- Semantic flagger: [`dedup_semantic_review.py`](dedup_semantic_review.py)

The scripts write JSON artifacts to:
- `/tmp/olas_dedup_verify.json`
- `/tmp/olas_dedup_flagged_groups.json`

## Adapting this workflow to another protocol (edit these constants)
This repo’s scripts are intentionally simple and **hardcode paths**.

1. Put your source report somewhere in your repo (any path is fine).
2. Update the path constants at the top of [`dedup_verify.py`](dedup_verify.py):
   - `REPORT_PATH = Path('...')` (your source report)
   - `DEDUP_PATH = Path('...')` (your dedup output file)
3. If your report headings don’t look like `[H-12]. Title`, update the regexes in `dedup_verify.py`:
   - `RE_REPORT_HEAD` (how to detect a finding header)
   - `RE_DERIVED`, `RE_STATUS`, `RE_PRIV` (how to extract metadata)
4. Optional: change the JSON output locations if you don’t want `/tmp/...`:
   - In `dedup_verify.py`: `Path('/tmp/olas_dedup_verify.json')`
   - In `dedup_semantic_review.py`: `out_path = Path('/tmp/olas_dedup_flagged_groups.json')`

Note: `dedup_semantic_review.py` imports `dedup_verify`, so changing `REPORT_PATH/DEDUP_PATH` in
`dedup_verify.py` automatically affects both scripts.

## Dedup format (required)
Each group in `olas-dedup-findings.md` must follow this pattern:
- `### [X-N]. <title>` (primary)
- `**Pattern**: <...>`
- `**Duplicates**: <comma-separated IDs> | None`
- `**Status**: <...>`
- `**Privilege**: <...>`

## Process (LLM-friendly, script-assisted)

### 1) Normalize the problem
1. Confirm the report uses stable IDs (e.g., `H-7`, `M-31`, `L-165`).
2. Decide the dedup rule:
   - Default: **root cause** (shared bug/mechanism) defines duplicates.
   - Don’t over-split if the contest scope treats a class as OOS (e.g., “non-standard ERC20” buckets).

### 2) Produce an initial dedup draft
Create `olas-dedup-findings.md` by grouping findings using high-signal features:
- **Location** (contract/function) and call-path
- **Exploit mechanism** (e.g., “fails open”, “missing minOut”, “unsafe assembly load”) 
- **Required preconditions** (permissions, configuration, timing)
- **Impact type** (theft vs DoS vs misconfiguration)

Heuristic that works well for LLMs:
- Start from the largest/most repeated themes (oracle, slippage, access control, assembly parsing, initialization).
- For each finding, decide: “Is this the *same bug* or just the same *area*?”

### 3) Mechanical verification (coverage + uniqueness)
Run:

```bash
python3 dedup_verify.py
```

Interpretation:
- **missing IDs** must be 0
- **extra IDs** must be 0
- **IDs assigned to >1 group** must be 0

If any are non-zero, fix the dedup file first (this is non-negotiable).

### 4) Semantic review (identify suspicious groupings)
Run:

```bash
python3 dedup_semantic_review.py
```

This script does *not* prove incorrectness. It flags groups where titles have low similarity.
Treat the output as a **triage list**.

### 5) Human/LLM semantic adjudication (the critical step)
For each flagged group:
1. Read the primary + each member’s **Description/Justification** in the source report.
2. Classify the grouping:
   - **Confirmed duplicate**: same root cause; keep grouped.
   - **Combo writeup**: a member mentions 2 issues; ensure the *other* issue is covered by another primary (or split if not).
   - **Misgrouped**: move the member to a better-fitting group or split into a new primary.
3. When in doubt, prefer splitting *unless* the contest scope says the class is OOS/invalid (then keep umbrella and mark for invalidation).

### 6) Iterate until stable
After any edits:

```bash
python3 dedup_verify.py && python3 dedup_semantic_review.py
```

Stop when:
- Mechanical checks stay green.
- Flagged groups are either (a) corrected or (b) explicitly accepted as umbrella/OOS.

## What the scripts guarantee vs what they don’t
### Guaranteed by scripts
- Every audit ID is present **exactly once** in the dedup output.
- No ID is accidentally assigned to multiple primaries.

### Not guaranteed by scripts
- Root-cause correctness (requires reading the writeups).
- Severity correctness (a separate judging step).

## Common pitfalls (and how to avoid them)
- **Mixing root causes in one bucket**: split “execution slippage missing” vs “oracle fails open” even if both appear in one writeup.
- **Deadlines vs slippage**: “deadline=block.timestamp” is a time-bounds bug; “spot-derived minOut” is a sandwichability bug.
- **OOS classes**: if a big bucket is OOS (e.g., USDT quirks), don’t waste time over-splitting; keep as umbrella and mark invalid later.

## Minimal checklist for applying to another protocol
1. Put the source report in a stable Markdown format with IDs.
2. Create a dedup output file using the required group schema (name it however you want).
3. Update `REPORT_PATH` and `DEDUP_PATH` in `dedup_verify.py`.
4. Run `dedup_verify.py` until 0 missing / 0 multi-assigned.
5. Use `dedup_semantic_review.py` to drive a focused human/LLM review.
6. Re-run both scripts after every regrouping.

