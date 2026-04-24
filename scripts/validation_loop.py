#!/usr/bin/env python3
"""Deterministic helpers for the validation prompt iteration loop.

The script owns queue state and artifact path selection. Codex still spawns
fresh workers, but it should ask this script what the next unit of work is
instead of re-deriving the workflow from prose every heartbeat.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
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
SCORING_SECTION_START = "<!-- SCORING SECTION START -->"
SCORING_SECTION_END = "<!-- SCORING SECTION END -->"
SCORING_PENDING_MARKER = "<!-- SCORING STATUS: PENDING -->"
SCORING_COMPLETE_MARKER = "<!-- SCORING STATUS: COMPLETE -->"
PROMPT_ANALYSIS_COMPLETE_MARKER = "<!-- PROMPT ANALYSIS STATUS: COMPLETE -->"

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
    worker_log_path: str
    worker_log_jsonl_path: str
    total_findings: int
    completed_findings: int
    remaining_findings: int
    next_finding: str | None
    phase: str


@dataclass(frozen=True)
class WorkerExecutionConfig:
    model: str
    reasoning_effort: str


def report_path(benchmark: str) -> Path:
    return REPO_ROOT / benchmark / "report" / "audit-report.md"


def prompt_path(prompt_version: str) -> Path:
    return REPO_ROOT / "validation-prompts" / f"{prompt_version}.md"


def raw_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return REPO_ROOT / "validation-runs" / prompt_version / f"{benchmark}-{run_id}.md"


def results_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return REPO_ROOT / "validation-results" / prompt_version / f"{benchmark}-{run_id}.md"


def worker_log_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return (
        REPO_ROOT
        / "validation-results"
        / prompt_version
        / f"{benchmark}-{run_id}-worker-log.md"
    )


def worker_log_jsonl_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return (
        REPO_ROOT
        / "validation-results"
        / prompt_version
        / f"{benchmark}-{run_id}-worker-log.jsonl"
    )


def prompt_analysis_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return (
        REPO_ROOT
        / "validation-results"
        / prompt_version
        / f"{benchmark}-{run_id}-prompt-analysis.md"
    )


def thread_progress_state_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return (
        REPO_ROOT
        / "validation-results"
        / prompt_version
        / f"{benchmark}-{run_id}-thread-progress-state.json"
    )


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


def worker_execution_config(worker_type: str) -> WorkerExecutionConfig | None:
    if worker_type in {"raw-finding", "scoring"}:
        return WorkerExecutionConfig(model="gpt-5.4", reasoning_effort="xhigh")
    if worker_type in {"prompt-analysis", "prompt-revision"}:
        return WorkerExecutionConfig(model="gpt-5.5", reasoning_effort="xhigh")
    return None


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


def init_worker_log(prompt_version: str, benchmark: str, run_id: str) -> tuple[Path, Path]:
    log_md = worker_log_path(prompt_version, benchmark, run_id)
    log_jsonl = worker_log_jsonl_path(prompt_version, benchmark, run_id)
    log_md.parent.mkdir(parents=True, exist_ok=True)

    if not log_md.exists():
        log_md.write_text(
            f"""# {benchmark} Worker Log {run_id}

Prompt version: `{prompt_version}`
Benchmark: `{benchmark}`
Run id: `{run_id}`
Raw run: `{raw_path(prompt_version, benchmark, run_id)}`
Scored result: `{results_path(prompt_version, benchmark, run_id)}`

This file records orchestrator-visible worker lifecycle events and the exact raw finding block appended by each completed raw worker.
"""
        )

    if not log_jsonl.exists():
        log_jsonl.touch()

    return log_md, log_jsonl


def default_scoring_section() -> list[str]:
    return [
        SCORING_PENDING_MARKER,
        "## Scoring",
        "",
        "- Status: Pending raw validation completion.",
        "- Primary H/M metrics: Pending.",
        "- Unique canonical H/M coverage: Pending.",
        "- Mistake analysis: Pending.",
    ]


def extract_scoring_section(results: Path) -> list[str] | None:
    if not results.exists():
        return None

    lines = results.read_text().splitlines()
    try:
        start_idx = lines.index(SCORING_SECTION_START)
        end_idx = lines.index(SCORING_SECTION_END)
    except ValueError:
        return None

    if end_idx <= start_idx:
        return None

    return lines[start_idx + 1 : end_idx]


def raw_finding_blocks(raw: Path) -> dict[str, str]:
    if not raw.exists():
        return {}

    lines = raw.read_text().splitlines()
    starts: list[tuple[int, re.Match[str]]] = []
    for idx, line in enumerate(lines):
        match = RAW_FINDING_RE.match(line)
        if match:
            starts.append((idx, match))

    if not starts:
        return {}

    try:
        anchor_idx = lines.index(APPEND_ANCHOR)
    except ValueError:
        anchor_idx = len(lines)

    blocks: dict[str, str] = {}
    for pos, (start_idx, match) in enumerate(starts):
        end_idx = starts[pos + 1][0] if pos + 1 < len(starts) else anchor_idx
        blocks[match.group(1)] = "\n".join(lines[start_idx:end_idx]).rstrip() + "\n"

    return blocks


def scoring_complete(results: Path) -> bool:
    section = extract_scoring_section(results)
    if not section:
        return False
    return any(line.strip() == SCORING_COMPLETE_MARKER for line in section)


def prompt_analysis_complete(path: Path) -> bool:
    if not path.exists():
        return False
    return PROMPT_ANALYSIS_COMPLETE_MARKER in path.read_text()


def block_field(block: str, label: str) -> str:
    prefix = f"- {label}: "
    for line in block.splitlines():
        if line.startswith(prefix):
            return line[len(prefix) :].strip()
    return "-"


def clean_table_value(value: str) -> str:
    cleaned = " ".join(value.split())
    if not cleaned:
        return "-"
    cleaned = cleaned.replace("`", "")
    return cleaned.replace("|", "\\|")


def sync_results_progress(prompt_version: str, benchmark: str, run_id: str) -> Path:
    report = report_path(benchmark)
    raw = raw_path(prompt_version, benchmark, run_id)
    results = results_path(prompt_version, benchmark, run_id)
    worker_log = worker_log_path(prompt_version, benchmark, run_id)
    findings = parse_findings(report)
    finding_map = {finding.fid: finding for finding in findings}
    blocks = raw_finding_blocks(raw)
    scoring_section = extract_scoring_section(results) or default_scoring_section()

    counts = {"Valid": 0, "Invalid": 0, "Needs Review": 0}
    rows: list[str] = []
    for finding in findings:
        block = blocks.get(finding.fid)
        if not block:
            continue

        decision = clean_table_value(block_field(block, "Decision"))
        counts.setdefault(decision, 0)
        counts[decision] += 1

        rows.append(
            "| {fid} / `{report_id}` | {title} | {decision} | {confidence} | {bug_exists} | {severity} | `{root_cause}` |".format(
                fid=finding.fid,
                report_id=finding.report_id,
                title=clean_table_value(block_field(block, "Finding Title") or finding.title),
                decision=decision,
                confidence=clean_table_value(block_field(block, "Confidence")),
                bug_exists=clean_table_value(block_field(block, "Bug Exists")),
                severity=clean_table_value(block_field(block, "Severity Assessment")),
                root_cause=clean_table_value(block_field(block, "Root Cause Family")),
            )
        )

    completed = len(blocks)
    total = len(findings)
    remaining = max(total - completed, 0)
    phase_label = "Raw validation complete; scoring pending" if completed == total else "Raw validation in progress"

    lines = [
        f"# {benchmark} Validation Results {run_id}",
        "",
        f"Prompt version: `{prompt_version}`",
        f"Benchmark: `{benchmark}`",
        f"Run id: `{run_id}`",
        f"Benchmark report: `{report}`",
        f"Benchmark source root: `{benchmark_source_root(benchmark)}`",
        f"Raw run: `{raw}`",
        f"Worker log: `{worker_log}`",
        "",
        "## Run Status",
        "",
        f"- Phase: `{phase_label}`",
        f"- Progress: `{completed} / {total}`",
        f"- Remaining findings: `{remaining}`",
        f"- Decision totals so far: `Valid={counts.get('Valid', 0)}`, `Invalid={counts.get('Invalid', 0)}`, `Needs Review={counts.get('Needs Review', 0)}`",
        "",
        "## Per-Finding Decisions",
        "",
        "| Finding | Title | Decision | Confidence | Bug Exists | Severity | Root Cause Family |",
        "| --- | --- | --- | --- | --- | --- | --- |",
    ]

    if rows:
        lines.extend(rows)
    else:
        lines.append("| - | No completed findings yet. | - | - | - | - | - |")

    lines.extend(
        [
            "",
            SCORING_SECTION_START,
            *scoring_section,
            SCORING_SECTION_END,
            "",
        ]
    )

    results.parent.mkdir(parents=True, exist_ok=True)
    results.write_text("\n".join(lines))
    return results


def latest_completed_finding_summary(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object] | None:
    findings = parse_findings(report_path(benchmark))
    finding_map = {finding.fid: finding for finding in findings}
    blocks = raw_finding_blocks(raw_path(prompt_version, benchmark, run_id))
    log_jsonl = worker_log_jsonl_path(prompt_version, benchmark, run_id)
    latest_payload = None
    for payload in reversed(iter_log_payloads(log_jsonl)):
        if (
            payload.get("worker_type") == "raw-finding"
            and payload.get("event") in {"completed", "backfill-completed"}
            and isinstance(payload.get("finding_id"), str)
            and payload.get("finding_id")
        ):
            latest_payload = payload
            break

    if latest_payload is None:
        return None

    latest_fid = latest_payload["finding_id"]
    finding = finding_map.get(latest_fid)
    block = blocks.get(latest_fid, "")
    title = block_field(block, "Finding Title")
    if title == "-" and isinstance(latest_payload.get("title"), str):
        title = latest_payload["title"]

    return {
        "finding_id": latest_fid,
        "report_id": finding.report_id if finding else None,
        "title": title,
        "decision": block_field(block, "Decision"),
        "confidence": block_field(block, "Confidence"),
        "bug_exists": block_field(block, "Bug Exists"),
        "severity": block_field(block, "Severity Assessment"),
        "root_cause_family": block_field(block, "Root Cause Family"),
        "timestamp": latest_payload.get("timestamp"),
    }


def logged_completed_finding_ids(prompt_version: str, benchmark: str, run_id: str) -> list[str]:
    ordered: list[str] = []
    seen: set[str] = set()
    log_jsonl = worker_log_jsonl_path(prompt_version, benchmark, run_id)

    for payload in iter_log_payloads(log_jsonl):
        if payload.get("worker_type") != "raw-finding":
            continue
        if payload.get("event") not in {"completed", "backfill-completed"}:
            continue
        finding_id = payload.get("finding_id")
        if not isinstance(finding_id, str) or not finding_id or finding_id in seen:
            continue
        seen.add(finding_id)
        ordered.append(finding_id)

    return ordered


def progress_snapshot(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    status = append_status(prompt_version, benchmark, run_id)
    findings = parse_findings(report_path(benchmark))
    completed_ids = logged_completed_finding_ids(prompt_version, benchmark, run_id)
    completed_set = set(completed_ids)
    completed_findings = len(completed_ids)
    total_findings = len(findings)
    remaining_findings = max(total_findings - completed_findings, 0)
    next_finding = next((finding.fid for finding in findings if finding.fid not in completed_set), None)
    if next_finding:
        phase = "raw-validation"
    elif not scoring_complete(results_path(prompt_version, benchmark, run_id)):
        phase = "scoring"
    elif not prompt_analysis_complete(prompt_analysis_path(prompt_version, benchmark, run_id)):
        phase = "prompt-analysis"
    elif not prompt_path(next_prompt_version(prompt_version)).exists():
        phase = "prompt-revision"
    else:
        phase = "complete"
    latest = latest_completed_finding_summary(prompt_version, benchmark, run_id)

    if phase == "raw-validation":
        token = f"raw:{completed_findings}:{latest['finding_id'] if latest else 'none'}"
    elif phase == "scoring":
        token = "phase:scoring"
    elif phase == "prompt-analysis":
        token = "phase:prompt-analysis"
    elif phase == "prompt-revision":
        token = "phase:prompt-revision"
    else:
        token = "phase:complete"

    if phase == "raw-validation":
        if latest:
            message = (
                f"{completed_findings}/{total_findings} findings validated for "
                f"{prompt_version} / {benchmark} / {run_id}. Most recent: {latest['finding_id']} "
                f"{latest['title']} -> {latest['decision']} ({latest['severity']}). "
                f"Next: {next_finding}."
            )
        else:
            message = (
                f"Validation started for {prompt_version} / {benchmark} / {run_id}. "
                f"No findings have completed yet. Next: {next_finding}."
            )
    elif phase == "scoring":
        message = (
            f"Raw validation finished for {prompt_version} / {benchmark} / {run_id} at "
            f"{completed_findings}/{total_findings}. Scoring is now running."
        )
    elif phase == "prompt-analysis":
        message = (
            f"Scoring finished for {prompt_version} / {benchmark} / {run_id}. "
            f"Prompt-analysis is now running."
        )
    elif phase == "prompt-revision":
        message = (
            f"Prompt-analysis finished for {prompt_version} / {benchmark} / {run_id}. "
            f"Prompt revision is now running."
        )
    else:
        message = f"Validation loop completed for {prompt_version} / {benchmark} / {run_id}."

    return {
        **asdict(status),
        "completed_findings": completed_findings,
        "token": token,
        "total_findings": total_findings,
        "remaining_findings": remaining_findings,
        "next_finding": next_finding,
        "phase": phase,
        "latest_completed_finding": latest,
        "user_message": message,
        "thread_progress_state_path": str(thread_progress_state_path(prompt_version, benchmark, run_id)),
    }


def current_timestamp() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def normalize_summary(summary: str | None) -> str | None:
    if summary is None:
        return None
    cleaned = " ".join(summary.split())
    return cleaned or None


def read_json(path: Path) -> dict[str, object] | None:
    if not path.exists():
        return None
    try:
        data = json.loads(path.read_text())
    except json.JSONDecodeError:
        return None
    return data if isinstance(data, dict) else None


def iter_log_payloads(log_jsonl: Path) -> list[dict[str, object]]:
    if not log_jsonl.exists():
        return []

    payloads: list[dict[str, object]] = []
    for line in log_jsonl.read_text().splitlines():
        if not line.strip():
            continue
        try:
            payload = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(payload, dict):
            payloads.append(payload)
    return payloads


def append_worker_log(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    *,
    event: str,
    worker_type: str,
    agent_id: str | None = None,
    finding_id: str | None = None,
    report_id: str | None = None,
    title: str | None = None,
    summary: str | None = None,
    include_raw_block: bool = False,
) -> dict[str, object]:
    log_md, log_jsonl = init_worker_log(prompt_version, benchmark, run_id)
    raw_block = None
    if include_raw_block and finding_id:
        raw_block = raw_finding_blocks(raw_path(prompt_version, benchmark, run_id)).get(finding_id)

    payload: dict[str, object] = {
        "timestamp": current_timestamp(),
        "prompt_version": prompt_version,
        "benchmark": benchmark,
        "run_id": run_id,
        "event": event,
        "worker_type": worker_type,
        "agent_id": agent_id,
        "finding_id": finding_id,
        "report_id": report_id,
        "title": title,
        "summary": summary,
        "raw_block": raw_block,
        "worker_log_path": str(log_md),
        "worker_log_jsonl_path": str(log_jsonl),
    }

    header = f"## {payload['timestamp']} `{event}` `{worker_type}`"
    if finding_id:
        header += f" `{finding_id}`"

    md_lines = ["", header]
    if agent_id:
        md_lines.append(f"- Agent ID: `{agent_id}`")
    if report_id:
        md_lines.append(f"- Report ID: `{report_id}`")
    if title:
        md_lines.append(f"- Title: {title}")
    if summary:
        md_lines.append(f"- Summary: {normalize_summary(summary)}")
    if raw_block:
        md_lines.extend(
            [
                "- Raw Run Block:",
                "```md",
                raw_block.rstrip(),
                "```",
            ]
        )

    with log_md.open("a") as handle:
        handle.write("\n".join(md_lines) + "\n")

    with log_jsonl.open("a") as handle:
        handle.write(json.dumps(payload, sort_keys=True) + "\n")

    if worker_type == "raw-finding" and event in {"completed", "backfill-completed"}:
        sync_results_progress(prompt_version, benchmark, run_id)

    return payload


def logged_finding_ids(log_jsonl: Path) -> set[str]:
    if not log_jsonl.exists():
        return set()

    finding_ids: set[str] = set()
    for line in log_jsonl.read_text().splitlines():
        if not line.strip():
            continue
        try:
            payload = json.loads(line)
        except json.JSONDecodeError:
            continue
        finding_id = payload.get("finding_id")
        if isinstance(finding_id, str) and finding_id:
            finding_ids.add(finding_id)
    return finding_ids


def append_status(prompt_version: str, benchmark: str, run_id: str) -> RawStatus:
    report = report_path(benchmark)
    raw = raw_path(prompt_version, benchmark, run_id)
    results = results_path(prompt_version, benchmark, run_id)
    worker_log = worker_log_path(prompt_version, benchmark, run_id)
    worker_log_jsonl = worker_log_jsonl_path(prompt_version, benchmark, run_id)
    findings = parse_findings(report)
    completed = completed_raw_findings(raw)
    next_finding = next((finding.fid for finding in findings if finding.fid not in completed), None)

    if next_finding:
        phase = "raw-validation"
    elif not scoring_complete(results):
        phase = "scoring"
    elif not prompt_analysis_complete(prompt_analysis_path(prompt_version, benchmark, run_id)):
        phase = "prompt-analysis"
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
        worker_log_path=str(worker_log),
        worker_log_jsonl_path=str(worker_log_jsonl),
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
4. Update the scoring section inside {results} by replacing only the content between:
   - {SCORING_SECTION_START}
   - {SCORING_SECTION_END}
5. Preserve the existing run-status header and per-finding decision table.

Optional context:
- benchmark source root: {source_root}
- benchmark report: {report_path(benchmark)}

Required output:
- begin the scoring section with `{SCORING_COMPLETE_MARKER}`
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
"""


def prompt_analysis_worker_prompt(prompt_version: str, benchmark: str, run_id: str) -> str:
    source_root = benchmark_source_root(benchmark)
    raw = raw_path(prompt_version, benchmark, run_id)
    results = results_path(prompt_version, benchmark, run_id)
    worker_log = worker_log_path(prompt_version, benchmark, run_id)
    truth = truth_path(benchmark)
    analysis = prompt_analysis_path(prompt_version, benchmark, run_id)

    return f"""You are the prompt-analysis worker for one completed validation iteration.

Workspace root: {REPO_ROOT}

Your task:
1. Read the current prompt at {prompt_path(prompt_version)}
2. Read the scored result at {results}
3. Read the raw run at {raw}
4. Read the worker log at {worker_log}
5. Read the benchmark report at {report_path(benchmark)}
6. Use benchmark source context from {source_root} as needed.
7. Read the specific truth artifacts needed for analysis:
   - {truth}
   - {REPO_ROOT / "C4_APPROVED_FINDINGS.md"}
   - {REPO_ROOT / "APPROVED_FINDINGS_KEY.md"}
   - {REPO_ROOT / "C4_LOW_QA_INVALID_FINDINGS.md"}
8. Produce a detailed analysis document at {analysis} that explains the validation mistakes in depth before any prompt revision is attempted.

Required analysis goals:
- determine why Low / QA / Invalid findings were mistakenly accepted as H/M
- determine why true H/M findings were mistakenly downgraded to Low / QA, rejected as Invalid, or abstained as Needs Review
- group mistakes into recurring, protocol-agnostic error families
- distinguish benchmark-specific facts from reusable validation lessons
- produce concrete prompt-revision recommendations grounded in the error analysis

Required output document shape:
- begin the file with `{PROMPT_ANALYSIS_COMPLETE_MARKER}`
- benchmark name
- prompt version tested
- run id
- artifact list reviewed
- current scored metrics summary
- executive summary of the biggest validation failure modes
- section: false positives / over-severity
  - list the main reasons Low / QA / Invalid findings were mistakenly accepted as H/M
  - include representative finding ids and any relevant judge-rationale patterns
- section: false negatives / under-severity
  - list the main reasons true H/M findings were mistakenly marked Low / QA, Invalid, or Needs Review
  - include representative finding ids, approved roots, and the failure mode for each cluster
- section: severity-calibration mistakes
  - explain when the system found a real issue but mis-scored its severity
- section: gate-level mistakes
  - summarize which validation gates or heuristics failed most often on the false-positive side and on the false-negative side
- section: protocol-agnostic lessons
  - convert the analysis into reusable lessons without embedding benchmark-specific facts
- section: prompt-revision recommendations
  - propose concrete, protocol-agnostic edits for the next prompt
  - clearly separate recommendations aimed at recall improvement from those aimed at precision improvement

Important constraints:
- This phase is analysis only. Do not create the next prompt version yet.
- Use apply_patch for file creation/editing.
- Do not copy benchmark-specific exploit narratives into reusable rules.
- You are not alone in the codebase. Do not revert others' edits.

When done, reply with a concise summary and list the files you changed.
"""


def prompt_revision_worker_prompt(prompt_version: str, benchmark: str, run_id: str) -> str:
    next_version = next_prompt_version(prompt_version)
    source_root = benchmark_source_root(benchmark)
    raw = raw_path(prompt_version, benchmark, run_id)
    results = results_path(prompt_version, benchmark, run_id)
    worker_log = worker_log_path(prompt_version, benchmark, run_id)
    truth = truth_path(benchmark)
    analysis = prompt_analysis_path(prompt_version, benchmark, run_id)

    return f"""You are the prompt-revision worker for one completed validation iteration.

Workspace root: {REPO_ROOT}

Your task:
1. Read the current prompt at {prompt_path(prompt_version)}
2. Read the scored result at {results}
3. Read the raw run at {raw}
4. Read the worker log at {worker_log}
5. Read the benchmark report at {report_path(benchmark)}
6. Read the prompt-analysis document at {analysis}
7. Use benchmark source context from {source_root} as needed.
8. Read the specific truth artifacts needed for analysis:
   - {truth}
   - {REPO_ROOT / "C4_APPROVED_FINDINGS.md"}
   - {REPO_ROOT / "APPROVED_FINDINGS_KEY.md"}
   - {REPO_ROOT / "C4_LOW_QA_INVALID_FINDINGS.md"}
9. Use the prompt-analysis document as the primary synthesis artifact for deciding what to change in the next prompt. Use the scored result, worker log, and `C4_LOW_QA_INVALID_FINDINGS.md` to verify and refine that analysis as needed, but do not copy benchmark-specific facts into the reusable prompt.
10. Draft the next prompt version at {prompt_path(next_version)} if the evidence justifies a protocol-agnostic improvement.
11. Update {REPO_ROOT / "validation-prompts" / "CHANGELOG.md"} with the {prompt_version} -> {next_version} rationale if you create the next prompt.

Rules for the new prompt:
- Every change must be protocol-agnostic.
- Do not bake in protocol names, contest facts, contract names, addresses, or benchmark-specific exploit details as reusable rules.
- Revise the stable prompt structure in place: maintain one `Core Validation Principles` section and fold lessons into the relevant gates whenever possible.
- Do not append version-named principles blocks to the active prompt.
- Add a new gate only when the error pattern is genuinely distinct and recurring.
- Prioritize precision once recall is already high.
- Use the C4 rejected / Low / QA judge rationales to improve severity calibration and false-positive suppression, especially when a report describes a real issue that is out-of-scope, unsupported, duplicate-root, operationally minor, or otherwise below the H/M bar.
- Use the approved H/M misses in the scored result and worker log to improve false-negative handling, especially when real H/M findings were downgraded to `Low / QA`, rejected as `Invalid`, or abstained as `Needs Review`.
- Improve both recall and precision; do not optimize one by sacrificing the other.
- Preserve same-or-higher H/M recall and same-or-higher H/M precision versus the current prompt.
- Treat at least 80% H/M recall and greater than 50% H/M precision as the convergence floor.
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
        worker_type = "scoring"
        prompt = scoring_worker_prompt(prompt_version, args.benchmark, run_id)
        payload = {
            **asdict(status),
            "worker_type": worker_type,
            "worker_prompt": prompt if args.include_prompt else None,
        }
    elif finding is None and status.phase == "prompt-analysis":
        worker_type = "prompt-analysis"
        prompt = prompt_analysis_worker_prompt(prompt_version, args.benchmark, run_id)
        payload = {
            **asdict(status),
            "worker_type": worker_type,
            "worker_prompt": prompt if args.include_prompt else None,
        }
    elif finding is None and status.phase == "prompt-revision":
        worker_type = "prompt-revision"
        prompt = prompt_revision_worker_prompt(prompt_version, args.benchmark, run_id)
        payload = {
            **asdict(status),
            "worker_type": worker_type,
            "worker_prompt": prompt if args.include_prompt else None,
        }
    elif finding is None:
        worker_type = "none"
        prompt = ""
        payload = {
            **asdict(status),
            "worker_type": worker_type,
            "worker_prompt": None,
        }
    else:
        worker_type = "raw-finding"
        prompt = append_worker_prompt(prompt_version, args.benchmark, run_id, finding)
        payload = {
            **asdict(status),
            "worker_type": worker_type,
            "finding": asdict(finding) | {"block": None},
            "worker_prompt": prompt if args.include_prompt else None,
        }

    execution = worker_execution_config(worker_type)
    if execution:
        payload["worker_model"] = execution.model
        payload["worker_reasoning_effort"] = execution.reasoning_effort

    if args.write_prompt and prompt:
        write_prompt(Path(args.write_prompt), prompt)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))

    print_json(payload)


def cmd_init_raw(args: argparse.Namespace) -> None:
    prompt_version = args.prompt_version or latest_prompt_version()
    run_id = args.run_id or next_run_id(prompt_version, args.benchmark)
    raw = init_raw(prompt_version, args.benchmark, run_id, force=args.force)
    worker_log, worker_log_jsonl = init_worker_log(prompt_version, args.benchmark, run_id)
    results = sync_results_progress(prompt_version, args.benchmark, run_id)
    print_json(
        {
            "prompt_version": prompt_version,
            "benchmark": args.benchmark,
            "run_id": run_id,
            "raw_path": str(raw),
            "results_path": str(results),
            "worker_log_path": str(worker_log),
            "worker_log_jsonl_path": str(worker_log_jsonl),
        }
    )


def cmd_log_worker(args: argparse.Namespace) -> None:
    prompt_version = args.prompt_version or latest_prompt_version()
    run_id = args.run_id or "run-001"
    payload = append_worker_log(
        prompt_version,
        args.benchmark,
        run_id,
        event=args.event,
        worker_type=args.worker_type,
        agent_id=args.agent_id,
        finding_id=args.finding_id,
        report_id=args.report_id,
        title=args.title,
        summary=args.summary,
        include_raw_block=args.include_raw_block,
    )
    print_json(payload)


def cmd_backfill_worker_log(args: argparse.Namespace) -> None:
    prompt_version = args.prompt_version or latest_prompt_version()
    run_id = args.run_id or "run-001"
    report_findings = {finding.fid: finding for finding in parse_findings(report_path(args.benchmark))}
    ordered_fids = [finding.fid for finding in parse_findings(report_path(args.benchmark))]
    blocks = raw_finding_blocks(raw_path(prompt_version, args.benchmark, run_id))
    _, log_jsonl = init_worker_log(prompt_version, args.benchmark, run_id)
    already_logged = logged_finding_ids(log_jsonl)

    added: list[str] = []
    for fid in ordered_fids:
        if fid not in blocks or fid in already_logged:
            continue
        finding = report_findings[fid]
        append_worker_log(
            prompt_version,
            args.benchmark,
            run_id,
            event="backfill-completed",
            worker_type="raw-finding",
            finding_id=finding.fid,
            report_id=finding.report_id,
            title=finding.title,
            summary="Backfilled from the existing raw validation run after worker logging was introduced.",
            include_raw_block=True,
        )
        added.append(fid)

    print_json(
        {
            "prompt_version": prompt_version,
            "benchmark": args.benchmark,
            "run_id": run_id,
            "backfilled_findings": added,
            "count": len(added),
            "results_path": str(sync_results_progress(prompt_version, args.benchmark, run_id)),
            "worker_log_path": str(worker_log_path(prompt_version, args.benchmark, run_id)),
            "worker_log_jsonl_path": str(worker_log_jsonl_path(prompt_version, args.benchmark, run_id)),
        }
    )


def cmd_progress_heartbeat(args: argparse.Namespace) -> None:
    prompt_version = args.prompt_version or latest_prompt_version()
    run_id = args.run_id or "run-001"
    snapshot = progress_snapshot(prompt_version, args.benchmark, run_id)
    state_path = thread_progress_state_path(prompt_version, args.benchmark, run_id)
    previous = read_json(state_path) or {}
    previous_token = previous.get("token")
    current_token = snapshot["token"]
    should_notify = previous_token != current_token

    payload = {
        **snapshot,
        "previous_token": previous_token,
        "should_notify": should_notify,
    }

    if args.write_state:
        state_path.parent.mkdir(parents=True, exist_ok=True)
        state_path.write_text(
            json.dumps(
                {
                    "token": current_token,
                    "updated_at": current_timestamp(),
                    "phase": snapshot["phase"],
                    "completed_findings": snapshot["completed_findings"],
                    "latest_completed_finding": snapshot["latest_completed_finding"],
                },
                indent=2,
                sort_keys=True,
            )
            + "\n"
        )

    print_json(payload)


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

    log_worker = subparsers.add_parser(
        "log-worker",
        help="Append one worker lifecycle event to the worker log.",
    )
    add_common(log_worker)
    log_worker.add_argument("--event", required=True)
    log_worker.add_argument("--worker-type", required=True)
    log_worker.add_argument("--agent-id")
    log_worker.add_argument("--finding-id")
    log_worker.add_argument("--report-id")
    log_worker.add_argument("--title")
    log_worker.add_argument("--summary")
    log_worker.add_argument("--include-raw-block", action="store_true")
    log_worker.set_defaults(func=cmd_log_worker)

    backfill = subparsers.add_parser(
        "backfill-worker-log",
        help="Backfill worker-log entries from an existing raw validation run.",
    )
    add_common(backfill)
    backfill.set_defaults(func=cmd_backfill_worker_log)

    progress = subparsers.add_parser(
        "progress-heartbeat",
        help="Emit thread notification progress for the current run and optionally persist state.",
    )
    add_common(progress)
    progress.add_argument("--write-state", action="store_true")
    progress.set_defaults(func=cmd_progress_heartbeat)

    return parser


def main(argv: Iterable[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    args.func(args)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
