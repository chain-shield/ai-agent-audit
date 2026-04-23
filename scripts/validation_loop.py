#!/usr/bin/env python3
"""Deterministic helpers for the validation prompt iteration loop.

The script owns queue state and artifact path selection. Codex still spawns
fresh workers, but it should ask this script what the next unit of work is
instead of re-deriving the workflow from prose every heartbeat.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Iterable


REPO_ROOT = Path(__file__).resolve().parents[1]
AUDIT_ROOT = Path("/Users/apmfree/Desktop/Audit")
TRUTH_ROOT = Path("/Users/apmfree/.ai-agent-audit-validation-truth")
APPEND_ANCHOR = "<!-- APPEND FINDING BLOCKS ABOVE THIS LINE -->"

REPORT_FINDING_RE = re.compile(r"^## \[([HML]-\d+)\]\. (.+)$")
RAW_FINDING_RE = re.compile(r"^### ([HML]-\d+) / `([^`]+)`")
RAW_TABLE_FINDING_RE = re.compile(r"^\| ([HML]-\d+)(?: / `[^`]+`)? \|")
PROMPT_VERSION_RE = re.compile(r"^v(\d+)\.md$")
RUN_RE = re.compile(r"^(?P<benchmark>.+)-run-(?P<num>\d{3})\.md$")


@dataclass(frozen=True)
class Finding:
    fid: str
    title: str
    report_id: str
    start_line: int
    end_line: int
    block: str


@dataclass(frozen=True)
class RawStatus:
    prompt_version: str
    benchmark: str
    run_id: str
    report_path: str
    raw_path: str
    results_path: str
    total_findings: int
    completed_findings: int
    remaining_findings: int
    next_finding: str | None
    phase: str


def report_path(benchmark: str) -> Path:
    return REPO_ROOT / benchmark / "report" / "audit-report.md"


def prompt_path(prompt_version: str) -> Path:
    return REPO_ROOT / "validation-prompts" / f"{prompt_version}.md"


def raw_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return REPO_ROOT / "validation-runs" / prompt_version / f"{benchmark}-{run_id}.md"


def results_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return REPO_ROOT / "validation-results" / prompt_version / f"{benchmark}-{run_id}.md"


def benchmark_source_root(benchmark: str) -> Path:
    return AUDIT_ROOT / benchmark


def truth_path(benchmark: str) -> Path:
    return TRUTH_ROOT / f"{benchmark}.md"


def read_text(path: Path) -> str:
    try:
        return path.read_text()
    except FileNotFoundError:
        raise SystemExit(f"Missing required file: {path}") from None


def parse_findings(report: Path) -> list[Finding]:
    text = read_text(report)
    lines = text.splitlines()
    starts: list[tuple[int, re.Match[str]]] = []

    for idx, line in enumerate(lines):
        match = REPORT_FINDING_RE.match(line)
        if match:
            starts.append((idx, match))

    findings: list[Finding] = []
    for pos, (start_idx, match) in enumerate(starts):
        end_idx = starts[pos + 1][0] if pos + 1 < len(starts) else len(lines)
        block_lines = lines[start_idx:end_idx]
        report_id = ""
        for line in block_lines[:12]:
            if line.startswith("## id:"):
                report_id = line.split(":", 1)[1].strip()
                break

        findings.append(
            Finding(
                fid=match.group(1),
                title=match.group(2).strip(),
                report_id=report_id,
                start_line=start_idx + 1,
                end_line=end_idx,
                block="\n".join(block_lines).rstrip() + "\n",
            )
        )

    return findings


def completed_raw_findings(raw: Path) -> set[str]:
    if not raw.exists():
        return set()

    completed: set[str] = set()
    for line in raw.read_text().splitlines():
        match = RAW_FINDING_RE.match(line)
        if match:
            completed.add(match.group(1))
            continue

        match = RAW_TABLE_FINDING_RE.match(line)
        if match:
            completed.add(match.group(1))
    return completed


def latest_prompt_version() -> str:
    versions: list[tuple[int, str]] = []
    for path in (REPO_ROOT / "validation-prompts").glob("v*.md"):
        match = PROMPT_VERSION_RE.match(path.name)
        if match:
            versions.append((int(match.group(1)), path.stem))
    if not versions:
        raise SystemExit("No validation prompt versions found.")
    return sorted(versions)[-1][1]


def next_prompt_version(prompt_version: str) -> str:
    match = re.fullmatch(r"v(\d+)", prompt_version)
    if not match:
        raise SystemExit(f"Invalid prompt version: {prompt_version}")
    return f"v{int(match.group(1)) + 1}"


def next_run_id(prompt_version: str, benchmark: str) -> str:
    run_dir = REPO_ROOT / "validation-runs" / prompt_version
    highest = 0
    if run_dir.exists():
        for path in run_dir.glob(f"{benchmark}-run-*.md"):
            match = RUN_RE.match(path.name)
            if match and match.group("benchmark") == benchmark:
                highest = max(highest, int(match.group("num")))
    return f"run-{highest + 1:03d}"


def init_raw(prompt_version: str, benchmark: str, run_id: str, force: bool = False) -> Path:
    raw = raw_path(prompt_version, benchmark, run_id)
    if raw.exists() and not force:
        return raw

    raw.parent.mkdir(parents=True, exist_ok=True)
    source_root = benchmark_source_root(benchmark)
    report = report_path(benchmark)
    prompt = prompt_path(prompt_version)

    content = f"""# {benchmark} Raw Validation {run_id}

Status: In progress
Benchmark report: `{report}`
Benchmark source root: `{source_root}`
Validation prompt: `{prompt}`

Starting benchmark docs:
- `{source_root / "README.md"}`
- `{source_root / "olas-docs.md"}`
- `{source_root / "olas-scope.md"}`

Progress note:
- Raw validation is serialized into one-finding worker appends.
- Each finding is validated by a fresh worker with `fork_context=false`.
- Use `python3 scripts/validation_loop.py next --prompt-version {prompt_version} --benchmark {benchmark} --run-id {run_id} --write-prompt /tmp/validation-worker.md` to prepare the next worker.

## Per-Finding Validation

{APPEND_ANCHOR}
"""
    raw.write_text(content)
    return raw


def append_status(prompt_version: str, benchmark: str, run_id: str) -> RawStatus:
    report = report_path(benchmark)
    raw = raw_path(prompt_version, benchmark, run_id)
    results = results_path(prompt_version, benchmark, run_id)
    findings = parse_findings(report)
    completed = completed_raw_findings(raw)
    next_finding = next((finding.fid for finding in findings if finding.fid not in completed), None)

    if next_finding:
        phase = "raw-validation"
    elif not results.exists():
        phase = "scoring"
    elif not prompt_path(next_prompt_version(prompt_version)).exists():
        phase = "prompt-revision"
    else:
        phase = "complete"

    return RawStatus(
        prompt_version=prompt_version,
        benchmark=benchmark,
        run_id=run_id,
        report_path=str(report),
        raw_path=str(raw),
        results_path=str(results),
        total_findings=len(findings),
        completed_findings=len(completed),
        remaining_findings=max(len(findings) - len(completed), 0),
        next_finding=next_finding,
        phase=phase,
    )


def next_finding(prompt_version: str, benchmark: str, run_id: str) -> Finding | None:
    raw = init_raw(prompt_version, benchmark, run_id)
    completed = completed_raw_findings(raw)
    for finding in parse_findings(report_path(benchmark)):
        if finding.fid not in completed:
            return finding
    return None


def append_worker_prompt(prompt_version: str, benchmark: str, run_id: str, finding: Finding) -> str:
    source_root = benchmark_source_root(benchmark)
    raw = raw_path(prompt_version, benchmark, run_id)
    report = report_path(benchmark)
    prompt = prompt_path(prompt_version)

    return f"""You are the raw-validation worker for exactly one finding.

Workspace root: {REPO_ROOT}

Benchmark:
- slug: {benchmark}
- report: {report}
- source root: {source_root}
- raw run file to append: {raw}
- append anchor: {APPEND_ANCHOR}
- prompt: {prompt}

Validate exactly this finding and no others:
- Finding ID: {finding.fid}
- Finding title: {finding.title}
- Report id: {finding.report_id}
- Report lines: {finding.start_line}-{finding.end_line} in {report}

Finding report block:
```md
{finding.block.rstrip()}
```

Required evidence sources to use:
- {source_root / "README.md"}
- {source_root / "olas-docs.md"}
- {source_root / "olas-scope.md"}
- Relevant Solidity code under {source_root}

Critical evidence hygiene:
- Do not look for any hidden truth, approved findings list, answer key, or benchmark scoring artifact.
- Do not read local post-hoc analysis files, validator outputs, or user-generated markdown that appears to summarize likely findings.
- Prioritize actual Solidity code and canonical benchmark docs over the report narrative.

Important constraints:
- This phase is raw validation only. Do not score the run or revise the prompt.
- Append exactly one new finding block above the anchor in the raw run file, then stop.
- Do not rewrite prior finding blocks.
- Use apply_patch for file editing.
- You are not alone in the codebase. Do not revert others' edits.

Append in this exact shape:

### {finding.fid} / `{finding.report_id}`
- Finding Title: {finding.title}
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
"""


def scoring_worker_prompt(prompt_version: str, benchmark: str, run_id: str) -> str:
    source_root = benchmark_source_root(benchmark)
    raw = raw_path(prompt_version, benchmark, run_id)
    results = results_path(prompt_version, benchmark, run_id)
    truth = truth_path(benchmark)

    return f"""You are the scoring worker for one completed validation run.

Workspace root: {REPO_ROOT}

Your task:
1. Read the raw run at {raw}
2. Read the benchmark truth file at {truth}
3. Score the raw run against the truth file.
4. Write the scored result to {results}

Optional context:
- benchmark source root: {source_root}
- benchmark report: {report_path(benchmark)}

Required output:
- benchmark name
- prompt version tested
- run id
- scoring scope, with the primary confusion matrix computed on H/M findings only
- TP, FP, TN, FN on the H/M subset
- recall, precision, specificity on the H/M subset
- abstentions / Needs Review on the H/M subset, if any
- total canonical approved C4 H/M findings
- unique approved H/M findings present in the report
- unique approved H/M findings accepted as Valid
- present-root recall
- end-to-end unique recall
- valid findings missed
- invalid findings incorrectly accepted
- checklist gates causing the most mistakes
- root-cause families causing false negatives and false positives
- short plain-English summary

Important constraints:
- This phase is scoring only. Do not revise the prompt.
- Use apply_patch for file creation/editing.
- Be explicit about assumptions if the truth mapping is incomplete.

When done, reply with a concise summary and list the files you changed.
"""


def prompt_revision_worker_prompt(prompt_version: str, benchmark: str, run_id: str) -> str:
    next_version = next_prompt_version(prompt_version)
    source_root = benchmark_source_root(benchmark)
    raw = raw_path(prompt_version, benchmark, run_id)
    results = results_path(prompt_version, benchmark, run_id)
    truth = truth_path(benchmark)

    return f"""You are the prompt-revision worker for one completed validation iteration.

Workspace root: {REPO_ROOT}

Your task:
1. Read the current prompt at {prompt_path(prompt_version)}
2. Read the scored result at {results}
3. Read the raw run at {raw}
4. Read the benchmark report at {report_path(benchmark)}
5. Use benchmark source context from {source_root} as needed.
6. Read the specific truth artifacts needed for analysis:
   - {truth}
   - {REPO_ROOT / "C4_APPROVED_FINDINGS.md"}
   - {REPO_ROOT / "APPROVED_FINDINGS_KEY.md"}
7. Draft the next prompt version at {prompt_path(next_version)} if the evidence justifies a protocol-agnostic improvement.
8. Update {REPO_ROOT / "validation-prompts" / "CHANGELOG.md"} with the {prompt_version} -> {next_version} rationale if you create the next prompt.

Rules for the new prompt:
- Every change must be protocol-agnostic.
- Do not bake in protocol names, contest facts, contract names, addresses, or benchmark-specific exploit details as reusable rules.
- Revise the stable prompt structure in place: maintain one `Core Validation Principles` section and fold lessons into the relevant gates whenever possible.
- Do not append version-named principles blocks to the active prompt.
- Add a new gate only when the error pattern is genuinely distinct and recurring.
- Prioritize precision once recall is already high.
- Preserve same-or-higher H/M recall and same-or-higher H/M precision versus the current prompt.
- Treat at least 50% H/M precision as the convergence floor.
- Do not improve recall by broadly accepting more invalid findings.
- If no clearly generalizable improvement exists, say so and do not force a new version.

Important constraints:
- This phase is prompt revision only. Do not rescore the run.
- Use apply_patch for file creation/editing.
- You are not alone in the codebase. Do not revert others' edits.

When done, reply with a concise summary and list the files you changed.
"""


def write_prompt(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


def print_json(data: object) -> None:
    print(json.dumps(data, indent=2, sort_keys=True))


def cmd_status(args: argparse.Namespace) -> None:
    prompt_version = args.prompt_version or latest_prompt_version()
    run_id = args.run_id or "run-001"
    status = append_status(prompt_version, args.benchmark, run_id)
    print_json(asdict(status))


def cmd_next(args: argparse.Namespace) -> None:
    prompt_version = args.prompt_version or latest_prompt_version()
    run_id = args.run_id or "run-001"
    finding = next_finding(prompt_version, args.benchmark, run_id)
    status = append_status(prompt_version, args.benchmark, run_id)

    if finding is None and status.phase == "scoring":
        prompt = scoring_worker_prompt(prompt_version, args.benchmark, run_id)
        payload = {
            **asdict(status),
            "worker_type": "scoring",
            "worker_prompt": prompt if args.include_prompt else None,
        }
    elif finding is None and status.phase == "prompt-revision":
        prompt = prompt_revision_worker_prompt(prompt_version, args.benchmark, run_id)
        payload = {
            **asdict(status),
            "worker_type": "prompt-revision",
            "worker_prompt": prompt if args.include_prompt else None,
        }
    elif finding is None:
        prompt = ""
        payload = {
            **asdict(status),
            "worker_type": "none",
            "worker_prompt": None,
        }
    else:
        prompt = append_worker_prompt(prompt_version, args.benchmark, run_id, finding)
        payload = {
            **asdict(status),
            "worker_type": "raw-finding",
            "finding": asdict(finding) | {"block": None},
            "worker_prompt": prompt if args.include_prompt else None,
        }

    if args.write_prompt and prompt:
        write_prompt(Path(args.write_prompt), prompt)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))

    print_json(payload)


def cmd_init_raw(args: argparse.Namespace) -> None:
    prompt_version = args.prompt_version or latest_prompt_version()
    run_id = args.run_id or next_run_id(prompt_version, args.benchmark)
    raw = init_raw(prompt_version, args.benchmark, run_id, force=args.force)
    print_json(
        {
            "prompt_version": prompt_version,
            "benchmark": args.benchmark,
            "run_id": run_id,
            "raw_path": str(raw),
        }
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(required=True)

    def add_common(subparser: argparse.ArgumentParser) -> None:
        subparser.add_argument("--benchmark", default="2026-01-olas")
        subparser.add_argument("--prompt-version")
        subparser.add_argument("--run-id")

    status = subparsers.add_parser("status", help="Show raw/scoring progress.")
    add_common(status)
    status.set_defaults(func=cmd_status)

    init_raw_parser = subparsers.add_parser("init-raw", help="Initialize a raw run file.")
    add_common(init_raw_parser)
    init_raw_parser.add_argument("--force", action="store_true")
    init_raw_parser.set_defaults(func=cmd_init_raw)

    next_parser = subparsers.add_parser("next", help="Emit the next worker unit.")
    add_common(next_parser)
    next_parser.add_argument("--include-prompt", action="store_true")
    next_parser.add_argument("--write-prompt")
    next_parser.set_defaults(func=cmd_next)

    return parser


def main(argv: Iterable[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    args.func(args)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
