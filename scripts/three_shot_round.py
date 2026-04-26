#!/usr/bin/env python3
"""Prepare, assemble, and score three-shot validation experiments.

Three-shot validation splits the full-report pass into:
1. scope / known-issue screening
2. unsupported-token screening
3. full validation on the surviving findings
4. optional canonicalization cleanup on the surviving H/M candidates

This is intended to cheaply strip obvious false positives before the expensive
final gate analysis while preserving recall on approved H/M roots.
"""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
AUDIT_ROOT = Path("/Users/apmfree/Desktop/Audit")
TRUTH_ROOT = Path("/Users/apmfree/.ai-agent-audit-validation-truth")
APPEND_ANCHOR = "<!-- APPEND FINDING BLOCKS ABOVE THIS LINE -->"
REPORT_FINDING_RE = re.compile(r"^## \[([HML]-\d+)\]\. (.+)$")
RAW_FINDING_RE = re.compile(r"^### ([HML]-\d+) / `([^`]+)`")


@dataclass(frozen=True)
class Finding:
    fid: str
    title: str
    report_id: str
    start_line: int
    end_line: int
    block: str


THREE_SHOT_ROOT = REPO_ROOT / "validation-three-shot"
THREE_SHOT_PROMPT_ROOT = THREE_SHOT_ROOT / "prompts"
THREE_SHOT_VALIDATION_PROMPT_ROOT = THREE_SHOT_ROOT / "validation-prompts"
DEFAULT_CONFIG_PATH = THREE_SHOT_ROOT / "config.yaml"
STAGE_WORKER_MODEL = "gpt-5.5"
STAGE_WORKER_REASONING = "xhigh"
DEDUP_WORKER_MODEL = "gpt-5.4"
DEDUP_WORKER_REASONING = "high"
SCORING_WORKER_MODEL = "gpt-5.4"
SCORING_WORKER_REASONING = "xhigh"
SCREEN_ANCHOR = "<!-- APPEND SCREEN ROWS ABOVE THIS LINE -->"


def report_path(benchmark: str) -> Path:
    return REPO_ROOT / benchmark / "report" / "audit-report.md"


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


def raw_finding_blocks(raw: Path) -> dict[str, str]:
    if not raw.exists():
        return {}

    lines = raw.read_text().splitlines()
    starts: list[tuple[int, re.Match[str]]] = []
    anchors: list[int] = []
    for idx, line in enumerate(lines):
        match = RAW_FINDING_RE.match(line)
        if match:
            starts.append((idx, match))
        if line == APPEND_ANCHOR:
            anchors.append(idx)

    if not starts:
        return {}

    blocks: dict[str, str] = {}
    for pos, (start_idx, match) in enumerate(starts):
        next_start_idx = starts[pos + 1][0] if pos + 1 < len(starts) else len(lines)
        next_anchor_idx = next((idx for idx in anchors if idx > start_idx), len(lines))
        end_idx = min(next_start_idx, next_anchor_idx)
        blocks[match.group(1)] = "\n".join(lines[start_idx:end_idx]).rstrip() + "\n"

    return blocks


def block_field(block: str, label: str) -> str:
    prefix = f"- {label}: "
    for line in block.splitlines():
        if line.startswith(prefix):
            return line[len(prefix) :].strip()
    return "-"


def scope_screen_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "scope-screens" / prompt_version / f"{benchmark}-{run_id}.md"


def token_screen_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "token-screens" / prompt_version / f"{benchmark}-{run_id}.md"


def stage3_run_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "stage3-runs" / prompt_version / f"{benchmark}-{run_id}.md"


def dedup_screen_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "dedup-screens" / prompt_version / f"{benchmark}-{run_id}.md"


def final_run_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "runs" / prompt_version / f"{benchmark}-{run_id}.md"


def submission_candidates_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "submission-candidates" / prompt_version / f"{benchmark}-{run_id}.md"


def result_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "results" / prompt_version / f"{benchmark}-{run_id}.md"


def score_json_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "results" / prompt_version / f"{benchmark}-{run_id}.json"


def round1_input_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "inputs" / "r1" / prompt_version / f"{benchmark}-{run_id}.md"


def round2_input_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "inputs" / "r2" / prompt_version / f"{benchmark}-{run_id}.md"


def round3_input_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "inputs" / "r3" / prompt_version / f"{benchmark}-{run_id}.md"


def round4_input_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "inputs" / "r4" / prompt_version / f"{benchmark}-{run_id}.md"


def validation_prompt_path(prompt_version: str) -> Path:
    path = THREE_SHOT_VALIDATION_PROMPT_ROOT / f"{prompt_version}.md"
    if not path.exists():
        raise SystemExit(f"Missing three-shot validation prompt: {path}")
    return path


def parse_yaml_scalar(value: str) -> object:
    value = value.strip()
    if not value:
        return ""
    if value in {"true", "True"}:
        return True
    if value in {"false", "False"}:
        return False
    if value in {"null", "Null", "NULL", "~"}:
        return None
    if (
        (value.startswith('"') and value.endswith('"'))
        or (value.startswith("'") and value.endswith("'"))
    ):
        return value[1:-1]
    return value


def load_simple_yaml(path: Path) -> dict[str, object]:
    """Load the small config subset used by validation-three-shot/config.yaml."""
    if not path.exists():
        return {}

    root: dict[str, object] = {}
    stack: list[tuple[int, dict[str, object]]] = [(-1, root)]

    for raw_line in path.read_text().splitlines():
        if not raw_line.strip() or raw_line.lstrip().startswith("#"):
            continue
        line = raw_line.split("#", 1)[0].rstrip()
        if not line.strip():
            continue
        indent = len(line) - len(line.lstrip(" "))
        if indent % 2:
            raise SystemExit(f"Invalid indentation in config {path}: {raw_line}")
        stripped = line.strip()
        if ":" not in stripped:
            raise SystemExit(f"Invalid YAML line in config {path}: {raw_line}")

        key, value = stripped.split(":", 1)
        key = key.strip()
        value = value.strip()
        while stack and indent <= stack[-1][0]:
            stack.pop()
        parent = stack[-1][1]
        if value:
            parent[key] = parse_yaml_scalar(value)
            continue

        child: dict[str, object] = {}
        parent[key] = child
        stack.append((indent, child))

    return root


def nested_get(config: dict[str, object], keys: list[str], default: str) -> str:
    current: object = config
    for key in keys:
        if not isinstance(current, dict) or key not in current:
            return default
        current = current[key]
    return str(current) if current is not None else default


def load_config(config_path: str | None) -> dict[str, object]:
    return load_simple_yaml(Path(config_path or DEFAULT_CONFIG_PATH))


def resolve_run_args(args: argparse.Namespace) -> dict[str, object]:
    config = load_config(getattr(args, "config", None))
    args.prompt_version = args.prompt_version or nested_get(config, ["prompt_version"], "v2")
    args.benchmark = args.benchmark or nested_get(config, ["benchmark"], "")
    args.run_id = args.run_id or nested_get(config, ["run_id"], "run-001")
    if not args.benchmark:
        raise SystemExit("Missing benchmark. Set --benchmark or validation-three-shot/config.yaml benchmark.")
    return config


def worker_model(config: dict[str, object], worker: str, default: str) -> str:
    return nested_get(config, ["workers", worker, "model"], default)


def worker_reasoning(config: dict[str, object], worker: str, default: str) -> str:
    return nested_get(config, ["workers", worker, "reasoning_effort"], default)


def worker_replacements(config: dict[str, object], worker: str, default_model: str, default_reasoning: str) -> dict[str, str]:
    return {
        "WORKER_MODEL": worker_model(config, worker, default_model),
        "WORKER_REASONING": worker_reasoning(config, worker, default_reasoning),
    }


def benchmark_scope_docs(source_root: Path) -> list[Path]:
    paths = [source_root / "README.md"]
    paths.extend(sorted(source_root.glob("*-scope.md")))
    paths.extend(sorted(source_root.glob("*-docs.md")))
    paths.extend([source_root / "v12-findings.md", source_root / "v12-checklist.md"])
    return paths


def markdown_path_list(paths: list[Path]) -> str:
    return "\n".join(f"- `{path}`" for path in paths)


def render_findings_blocks(findings: list[Finding]) -> str:
    return "\n\n".join(
        [
            "\n".join(
                [
                    f"### {finding.fid} / `{finding.report_id}`",
                    f"- Finding title: {finding.title}",
                    f"- Report lines: {finding.start_line}-{finding.end_line}",
                    "```md",
                    finding.block.rstrip(),
                    "```",
                ]
            )
            for finding in findings
        ]
    )


def write_round_input(path: Path, benchmark: str, findings: list[Finding]) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    content = "\n".join(
        [
            f"# {benchmark} Round Input",
            "",
            f"Source report: `{report_path(benchmark)}`",
            f"Finding count: `{len(findings)}`",
            "",
            "## Findings",
            "",
            render_findings_blocks(findings) if findings else "_No findings in this round._",
            "",
        ]
    )
    path.write_text(content)
    return path


def render_stage_prompt(template_name: str, replacements: dict[str, object]) -> str:
    template_path = THREE_SHOT_PROMPT_ROOT / template_name
    if not template_path.exists():
        raise SystemExit(f"Missing three-shot prompt template: {template_path}")

    text = template_path.read_text()
    for key, value in replacements.items():
        text = text.replace("{{" + key + "}}", str(value))
    unresolved = sorted(set(re.findall(r"{{[A-Z0-9_]+}}", text)))
    if unresolved:
        raise SystemExit(
            f"Unresolved placeholders in {template_path}: " + ", ".join(unresolved)
        )
    return text


def clean(value: str) -> str:
    value = " ".join(value.split())
    if not value:
        return "-"
    return value.replace("|", "\\|")


def parse_truth_key() -> dict[str, dict[str, str]]:
    key_path = REPO_ROOT / "APPROVED_FINDINGS_KEY.md"
    rows: dict[str, dict[str, str]] = {}

    for line in key_path.read_text().splitlines():
        if not line.startswith("| "):
            continue
        parts = [part.strip() for part in line.strip().strip("|").split("|")]
        if len(parts) < 5 or parts[0] in {"Finding", "---"}:
            continue
        fid = parts[0]
        if not re.fullmatch(r"[HML]-\d+", fid):
            continue
        rows[fid] = {
            "finding_id": fid,
            "title": parts[1],
            "truth": parts[2],
            "matching_c4_listing": parts[3],
            "reason": parts[4],
        }

    return rows


def total_canonical_approved_hm_findings(benchmark: str) -> int | None:
    truth_text = truth_path(benchmark).read_text()
    match = re.search(
        r"total canonical approved C4 H/M findings:\s*`?(\d+)`?",
        truth_text,
    )
    return int(match.group(1)) if match else None


def all_findings(benchmark: str) -> list[Finding]:
    return parse_findings(report_path(benchmark))


def init_screen(path: Path, title: str, benchmark: str, prompt_version: str) -> Path:
    if path.exists():
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    source_root = benchmark_source_root(benchmark)
    content = f"""# {benchmark} {title}

Status: In progress
Benchmark report: `{report_path(benchmark)}`
Benchmark source root: `{source_root}`
Validation prompt: `{validation_prompt_path(prompt_version)}`

Mandatory benchmark docs:
{markdown_path_list(benchmark_scope_docs(source_root))}

## Decisions

| Finding | Finding Title | Decision | Confidence | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- |
{SCREEN_ANCHOR}
"""
    path.write_text(content)
    return path


def init_stage3_run(prompt_version: str, benchmark: str, run_id: str) -> Path:
    path = stage3_run_path(prompt_version, benchmark, run_id)
    if path.exists():
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    source_root = benchmark_source_root(benchmark)
    content = f"""# {benchmark} Three-Shot Stage 3 {run_id}

Status: In progress
Benchmark report: `{report_path(benchmark)}`
Benchmark source root: `{source_root}`
Validation prompt: `{validation_prompt_path(prompt_version)}`

Mandatory benchmark docs:
{markdown_path_list(benchmark_scope_docs(source_root))}

## Per-Finding Validation

{APPEND_ANCHOR}
"""
    path.write_text(content)
    return path


def init_dedup_screen(prompt_version: str, benchmark: str, run_id: str, reset: bool = False) -> Path:
    path = dedup_screen_path(prompt_version, benchmark, run_id)
    if path.exists() and not reset:
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    content = f"""# {benchmark} Three-Shot Round 4 Canonicalization Screen

Status: In progress
Source assembled run: `{final_run_path(prompt_version, benchmark, run_id)}`
Output submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`

## Decisions

| Finding | Finding Title | Decision | Confidence | Canonical Finding | Root Cause Group | Report Planning Note | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
{SCREEN_ANCHOR}
"""
    path.write_text(content)
    return path


def parse_screen(path: Path) -> dict[str, dict[str, str]]:
    if not path.exists():
        raise SystemExit(f"Missing required screen file: {path}")

    rows: dict[str, dict[str, str]] = {}
    for line in path.read_text().splitlines():
        if not line.startswith("| "):
            continue
        parts = [part.strip() for part in line.strip().strip("|").split("|")]
        if len(parts) < 6 or parts[0] in {"Finding", "---"}:
            continue
        finding_cell = parts[0]
        fid = finding_cell.split(" / ", 1)[0].strip()
        if not re.fullmatch(r"[HML]-\d+", fid):
            continue
        title = parts[1]
        decision = parts[2]
        confidence = parts[3]
        reason_category = parts[4]
        reason = "|".join(parts[5:]).strip()
        rows[fid] = {
            "finding_id": fid,
            "finding_cell": finding_cell,
            "title": title,
            "decision": decision,
            "confidence": confidence,
            "reason_category": reason_category,
            "reason": reason,
        }
    return rows


def parse_dedup_screen(path: Path) -> dict[str, dict[str, str]]:
    if not path.exists():
        raise SystemExit(f"Missing required dedup screen file: {path}")

    rows: dict[str, dict[str, str]] = {}
    header: list[str] | None = None
    for line in path.read_text().splitlines():
        if not line.startswith("| "):
            continue
        parts = [part.strip() for part in line.strip().strip("|").split("|")]
        if parts[0] == "Finding":
            header = [re.sub(r"[^a-z0-9]+", "_", part.lower()).strip("_") for part in parts]
            continue
        if parts[0] == "---" or len(parts) < 7:
            continue
        fid = parts[0].split(" / ", 1)[0].strip()
        if not re.fullmatch(r"[HML]-\d+", fid):
            continue
        if header and len(parts) >= len(header):
            mapped = dict(zip(header, parts, strict=False))
            rows[fid] = {
                "finding_id": fid,
                "title": mapped.get("finding_title", ""),
                "decision": mapped.get("decision", ""),
                "confidence": mapped.get("confidence", ""),
                "canonical_finding": mapped.get("canonical_finding", ""),
                "root_cause_group": mapped.get("root_cause_group", ""),
                "report_planning_note": mapped.get("report_planning_note", ""),
                "reason_category": mapped.get("reason_category", ""),
                "reason": mapped.get("reason", ""),
            }
            continue
        rows[fid] = {
            "finding_id": fid,
            "title": parts[1],
            "decision": parts[2],
            "confidence": parts[3],
            "canonical_finding": parts[4],
            "root_cause_group": "",
            "report_planning_note": "",
            "reason_category": parts[5],
            "reason": "|".join(parts[6:]).strip(),
        }
    return rows


def require_scope_rows(prompt_version: str, benchmark: str, run_id: str) -> dict[str, dict[str, str]]:
    rows = parse_screen(scope_screen_path(prompt_version, benchmark, run_id))
    expected = {finding.fid for finding in all_findings(benchmark)}
    if not rows:
        raise SystemExit(
            "Scope screen has no decisions yet. Complete stage 1 before preparing stage 2."
        )
    missing = sorted(expected - set(rows))
    if missing:
        raise SystemExit(
            "Scope screen is incomplete. Missing findings: " + ", ".join(missing[:10])
        )
    return rows


def require_token_rows(prompt_version: str, benchmark: str, run_id: str) -> dict[str, dict[str, str]]:
    scope_rows = require_scope_rows(prompt_version, benchmark, run_id)
    rows = parse_screen(token_screen_path(prompt_version, benchmark, run_id))
    required = {
        finding.fid
        for finding in all_findings(benchmark)
        if scope_rows.get(finding.fid, {}).get("decision") == "Keep"
    }
    if required and not rows:
        raise SystemExit(
            "Token screen has no decisions yet. Complete stage 2 before preparing stage 3."
        )
    missing = sorted(required - set(rows))
    if missing:
        raise SystemExit(
            "Token screen is incomplete. Missing findings: " + ", ".join(missing[:10])
        )
    return rows


def kept_after_scope(prompt_version: str, benchmark: str, run_id: str) -> list[Finding]:
    rows = require_scope_rows(prompt_version, benchmark, run_id)
    return [finding for finding in all_findings(benchmark) if rows.get(finding.fid, {}).get("decision") == "Keep"]


def kept_after_token(prompt_version: str, benchmark: str, run_id: str) -> list[Finding]:
    scope_rows = require_scope_rows(prompt_version, benchmark, run_id)
    token_rows = require_token_rows(prompt_version, benchmark, run_id)
    kept: list[Finding] = []
    for finding in all_findings(benchmark):
        if scope_rows.get(finding.fid, {}).get("decision") != "Keep":
            continue
        if token_rows.get(finding.fid, {}).get("decision") == "Keep":
            kept.append(finding)
    return kept


def dedup_candidates(prompt_version: str, benchmark: str, run_id: str) -> list[tuple[Finding, str]]:
    run_path = final_run_path(prompt_version, benchmark, run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    findings_by_id = {finding.fid: finding for finding in all_findings(benchmark)}
    blocks = raw_finding_blocks(run_path)
    candidates: list[tuple[Finding, str]] = []
    for finding in all_findings(benchmark):
        block = blocks.get(finding.fid)
        if not block:
            continue
        if block_field(block, "Decision") != "Valid":
            continue
        if block_field(block, "Severity Assessment") not in {"High", "Medium"}:
            continue
        candidates.append((findings_by_id[finding.fid], block))
    return candidates


def write_round4_input(
    path: Path,
    prompt_version: str,
    benchmark: str,
    run_id: str,
    candidates: list[tuple[Finding, str]],
) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    sections: list[str] = []
    for finding, block in candidates:
        sections.extend(
            [
                f"## {finding.fid} / `{finding.report_id}`",
                f"- Finding title: {finding.title}",
                f"- Report lines: {finding.start_line}-{finding.end_line}",
                "",
                "### Original Report Block",
                "```md",
                finding.block.rstrip(),
                "```",
                "",
                "### Current Validated Block",
                block.strip(),
                "",
            ]
        )
    content = "\n".join(
        [
            f"# {benchmark} Round 4 Canonicalization Input",
            "",
            f"Source assembled run: `{final_run_path(prompt_version, benchmark, run_id)}`",
            f"Candidate count: `{len(candidates)}`",
            "",
            "This file contains only findings currently marked `Valid` with `High` or `Medium` severity.",
            "",
            "## Candidates",
            "",
            "\n".join(sections) if sections else "_No H/M candidates to dedup._",
            "",
        ]
    )
    path.write_text(content)
    return path


def require_dedup_rows(prompt_version: str, benchmark: str, run_id: str) -> dict[str, dict[str, str]]:
    rows = parse_dedup_screen(dedup_screen_path(prompt_version, benchmark, run_id))
    expected = {finding.fid for finding, _ in dedup_candidates(prompt_version, benchmark, run_id)}
    if expected and not rows:
        raise SystemExit("Dedup screen has no decisions yet. Complete round 4 before applying dedup.")
    missing = sorted(expected - set(rows))
    if missing:
        raise SystemExit(
            "Dedup screen is incomplete. Missing findings: " + ", ".join(missing[:10])
        )
    return rows


def render_common_replacements(prompt_version: str, benchmark: str) -> dict[str, object]:
    source_root = benchmark_source_root(benchmark)
    scope_paths = sorted(source_root.glob("*-scope.md"))
    docs_paths = sorted(source_root.glob("*-docs.md"))
    return {
        "REPO_ROOT": REPO_ROOT,
        "BENCHMARK": benchmark,
        "REPORT": report_path(benchmark),
        "SOURCE_ROOT": source_root,
        "PROMPT": validation_prompt_path(prompt_version),
        "README_PATH": source_root / "README.md",
        "SCOPE_DOC_PATHS": markdown_path_list(benchmark_scope_docs(source_root)),
        "SCOPE_PATHS": markdown_path_list(scope_paths) if scope_paths else "- `(none found)`",
        "DOCS_PATHS": markdown_path_list(docs_paths) if docs_paths else "- `(none found)`",
        "V12_FINDINGS_PATH": source_root / "v12-findings.md",
        "V12_CHECKLIST_PATH": source_root / "v12-checklist.md",
    }


def render_scope_prompt(prompt_version: str, benchmark: str, run_id: str, config: dict[str, object]) -> str:
    input_path = write_round_input(
        round1_input_path(prompt_version, benchmark, run_id),
        benchmark,
        all_findings(benchmark),
    )
    return render_stage_prompt(
        "r1.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **worker_replacements(config, "r1", STAGE_WORKER_MODEL, STAGE_WORKER_REASONING),
            "STAGE_PATH": scope_screen_path(prompt_version, benchmark, run_id),
            "INPUT_PATH": input_path,
        },
    )


def render_token_prompt(prompt_version: str, benchmark: str, run_id: str, config: dict[str, object]) -> str:
    input_path = write_round_input(
        round2_input_path(prompt_version, benchmark, run_id),
        benchmark,
        kept_after_scope(prompt_version, benchmark, run_id),
    )
    return render_stage_prompt(
        "r2.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **worker_replacements(config, "r2", STAGE_WORKER_MODEL, STAGE_WORKER_REASONING),
            "STAGE_PATH": token_screen_path(prompt_version, benchmark, run_id),
            "INPUT_PATH": input_path,
        },
    )


def render_stage3_prompt(prompt_version: str, benchmark: str, run_id: str, config: dict[str, object]) -> str:
    input_path = write_round_input(
        round3_input_path(prompt_version, benchmark, run_id),
        benchmark,
        kept_after_token(prompt_version, benchmark, run_id),
    )
    return render_stage_prompt(
        "r3.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **worker_replacements(config, "r3", STAGE_WORKER_MODEL, STAGE_WORKER_REASONING),
            "STAGE_PATH": stage3_run_path(prompt_version, benchmark, run_id),
            "APPEND_ANCHOR": APPEND_ANCHOR,
            "INPUT_PATH": input_path,
        },
    )


def render_dedup_prompt(prompt_version: str, benchmark: str, run_id: str, config: dict[str, object]) -> str:
    input_path = write_round4_input(
        round4_input_path(prompt_version, benchmark, run_id),
        prompt_version,
        benchmark,
        run_id,
        dedup_candidates(prompt_version, benchmark, run_id),
    )
    return render_stage_prompt(
        "r4.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **worker_replacements(config, "r4", DEDUP_WORKER_MODEL, DEDUP_WORKER_REASONING),
            "INPUT_PATH": input_path,
            "STAGE_PATH": dedup_screen_path(prompt_version, benchmark, run_id),
            "FINAL_RUN_PATH": final_run_path(prompt_version, benchmark, run_id),
            "SUBMISSION_PATH": submission_candidates_path(prompt_version, benchmark, run_id),
        },
    )


def auto_scope_code_evidence(source_root: Path) -> str:
    docs = ", ".join(f"`{path}`" for path in benchmark_scope_docs(source_root))
    return (
        "Excluded during the stage 1 scope / known-issue screen using "
        f"{docs}."
    )


def auto_token_code_evidence(source_root: Path) -> str:
    docs = ", ".join(f"`{path}`" for path in benchmark_scope_docs(source_root))
    return (
        "Excluded during the stage 2 unsupported-token screen using "
        f"{docs}, and the stage 1 scope screen."
    )


def build_scope_block(finding: Finding, row: dict[str, str], source_root: Path) -> str:
    return "\n".join(
        [
            f"### {finding.fid} / `{finding.report_id}`",
            f"- Finding Title: {finding.title}",
            f"- Decision: Invalid",
            f"- Confidence: {row['confidence'] or 'High'}",
            f"- Bug Exists: Unclear",
            f"- Severity Assessment: Low / Unclear",
            f"- Root Cause Family: `{row['reason_category'] or 'scope-exclusion'}`",
            f"- Checklist Gates Passed: `Pre-gate sanity`",
            f"- Checklist Gates Failed: `Gate 1`",
            f"- Detailed Reason: {row['reason']}",
            f"- Code Evidence: {auto_scope_code_evidence(source_root)}",
        ]
    )


def build_token_block(finding: Finding, row: dict[str, str], source_root: Path) -> str:
    return "\n".join(
        [
            f"### {finding.fid} / `{finding.report_id}`",
            f"- Finding Title: {finding.title}",
            f"- Decision: Invalid",
            f"- Confidence: {row['confidence'] or 'High'}",
            f"- Bug Exists: Unclear",
            f"- Severity Assessment: Low / QA",
            f"- Root Cause Family: `{row['reason_category'] or 'unsupported-token'}`",
            f"- Checklist Gates Passed: `Gate 1`",
            f"- Checklist Gates Failed: `Gate 2`",
            f"- Detailed Reason: {row['reason']}",
            f"- Code Evidence: {auto_token_code_evidence(source_root)}",
        ]
    )


def assemble_run(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    source_root = benchmark_source_root(benchmark)
    findings = all_findings(benchmark)
    scope_rows = parse_screen(scope_screen_path(prompt_version, benchmark, run_id))
    token_rows = parse_screen(token_screen_path(prompt_version, benchmark, run_id))
    stage3_blocks = raw_finding_blocks(stage3_run_path(prompt_version, benchmark, run_id))

    blocks: list[str] = []
    scope_excluded = 0
    token_excluded = 0
    stage3_kept = 0

    for finding in findings:
        scope_row = scope_rows.get(finding.fid)
        if scope_row and scope_row["decision"] == "Exclude":
            blocks.append(build_scope_block(finding, scope_row, source_root))
            scope_excluded += 1
            continue

        token_row = token_rows.get(finding.fid)
        if token_row and token_row["decision"] == "Exclude":
            blocks.append(build_token_block(finding, token_row, source_root))
            token_excluded += 1
            continue

        block = stage3_blocks.get(finding.fid)
        if not block:
            raise SystemExit(
                f"Missing final-stage block for {finding.fid}. Run stage 3 or fix earlier screens first."
            )
        blocks.append(block.strip())
        stage3_kept += 1

    final_path = final_run_path(prompt_version, benchmark, run_id)
    final_path.parent.mkdir(parents=True, exist_ok=True)
    content = "\n".join(
        [
            f"# {benchmark} Three-Shot Validation {run_id}",
            "",
            "Status: Complete",
            f"Benchmark report: `{report_path(benchmark)}`",
            f"Benchmark source root: `{source_root}`",
            f"Validation prompt: `{validation_prompt_path(prompt_version)}`",
            f"Stage 1 scope screen: `{scope_screen_path(prompt_version, benchmark, run_id)}`",
            f"Stage 2 token screen: `{token_screen_path(prompt_version, benchmark, run_id)}`",
            f"Stage 3 final validation run: `{stage3_run_path(prompt_version, benchmark, run_id)}`",
            "",
            "## Assembly Summary",
            "",
            f"- Excluded at stage 1 (scope / known issue): `{scope_excluded}`",
            f"- Excluded at stage 2 (unsupported token): `{token_excluded}`",
            f"- Fully validated at stage 3: `{stage3_kept}`",
            "",
            "## Per-Finding Validation",
            "",
            "\n\n".join(blocks),
            "",
        ]
    )
    final_path.write_text(content)

    return {
        "benchmark": benchmark,
        "prompt_version": prompt_version,
        "run_id": run_id,
        "final_run_path": str(final_path),
        "scope_excluded": scope_excluded,
        "token_excluded": token_excluded,
        "stage3_kept": stage3_kept,
    }


def apply_dedup(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    run_path = final_run_path(prompt_version, benchmark, run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    rows = require_dedup_rows(prompt_version, benchmark, run_id)
    blocks = raw_finding_blocks(run_path)
    kept_blocks: list[str] = []
    dropped: list[str] = []
    kept: list[str] = []
    root_cause_groups: dict[str, list[str]] = {}

    for finding, block in dedup_candidates(prompt_version, benchmark, run_id):
        row = rows.get(finding.fid)
        group = (row or {}).get("root_cause_group") or "ungrouped"
        root_cause_groups.setdefault(group, []).append(
            f"{finding.fid} -> {(row or {}).get('canonical_finding') or '-'}"
            if row and row["decision"] == "Drop"
            else finding.fid
        )
        if row and row["decision"] == "Drop":
            dropped.append(finding.fid)
            continue
        kept.append(finding.fid)
        kept_blocks.append(block.strip())

    out_path = submission_candidates_path(prompt_version, benchmark, run_id)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    content = "\n".join(
        [
            f"# {benchmark} Three-Shot Submission Candidates {run_id}",
            "",
            "Status: Complete",
            f"Source assembled run: `{run_path}`",
            f"Canonicalization screen: `{dedup_screen_path(prompt_version, benchmark, run_id)}`",
            "",
            "## Canonicalization Summary",
            "",
            f"- Candidate H/M findings before R4: `{len(kept) + len(dropped)}`",
            f"- Kept after R4 canonicalization: `{len(kept)}`",
            f"- Dropped as duplicate-equivalent: `{len(dropped)}`",
            f"- Dropped finding ids: `{', '.join(dropped) if dropped else '-'}`",
            "",
            "## Root Cause Groups",
            "",
            "\n".join(
                f"- `{group}`: {', '.join(items)}"
                for group, items in sorted(root_cause_groups.items())
            )
            if root_cause_groups
            else "- `ungrouped`: -",
            "",
            "## Submission Candidates",
            "",
            "\n\n".join(kept_blocks) if kept_blocks else "_No H/M candidates remain after dedup._",
            "",
        ]
    )
    out_path.write_text(content)
    return {
        "benchmark": benchmark,
        "prompt_version": prompt_version,
        "run_id": run_id,
        "source_run_path": str(run_path),
        "dedup_screen_path": str(dedup_screen_path(prompt_version, benchmark, run_id)),
        "submission_candidates_path": str(out_path),
        "input_candidates": len(kept) + len(dropped),
        "kept_candidates": len(kept),
        "dropped_candidates": len(dropped),
        "dropped_findings": dropped,
    }


def score_three_shot(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    run_path = final_run_path(prompt_version, benchmark, run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    blocks = raw_finding_blocks(run_path)
    findings = parse_findings(report_path(benchmark))
    truth_rows = parse_truth_key()

    results_rows: list[dict[str, str]] = []
    tp = fp = tn = fn = abstain = 0
    present_truth_roots: set[str] = set()
    accepted_truth_roots: set[str] = set()
    false_negatives: list[str] = []
    false_positives: list[str] = []

    for finding in findings:
        block = blocks.get(finding.fid)
        if not block:
            continue

        truth_row = truth_rows.get(finding.fid)
        if truth_row is None:
            continue

        decision = block_field(block, "Decision")
        severity = block_field(block, "Severity Assessment")
        confidence = block_field(block, "Confidence")
        bug_exists = block_field(block, "Bug Exists")
        root_cause_family = block_field(block, "Root Cause Family")

        if finding.fid.startswith(("H-", "M-")):
            truth_positive = truth_row["truth"] == "Valid"
            run_positive = decision == "Valid" and severity in {"High", "Medium"}
            is_abstention = decision == "Needs Review"

            if truth_positive and truth_row["matching_c4_listing"]:
                present_truth_roots.add(truth_row["matching_c4_listing"])

            if is_abstention:
                abstain += 1
            elif truth_positive:
                if run_positive:
                    tp += 1
                    if truth_row["matching_c4_listing"]:
                        accepted_truth_roots.add(truth_row["matching_c4_listing"])
                else:
                    fn += 1
                    false_negatives.append(finding.fid)
            else:
                if run_positive:
                    fp += 1
                    false_positives.append(finding.fid)
                else:
                    tn += 1

        results_rows.append(
            {
                "finding_id": finding.fid,
                "report_id": finding.report_id,
                "title": block_field(block, "Finding Title") or finding.title,
                "decision": decision,
                "confidence": confidence,
                "bug_exists": bug_exists,
                "severity": severity,
                "root_cause_family": root_cause_family,
                "truth": truth_row["truth"],
                "matching_c4_listing": truth_row["matching_c4_listing"],
            }
        )

    recall = tp / (tp + fn) if (tp + fn) else 0.0
    precision = tp / (tp + fp) if (tp + fp) else 0.0
    specificity = tn / (tn + fp) if (tn + fp) else 0.0
    total_canonical = total_canonical_approved_hm_findings(benchmark)
    present_root_recall = (
        len(accepted_truth_roots) / len(present_truth_roots) if present_truth_roots else 0.0
    )
    end_to_end_unique_recall = (
        len(accepted_truth_roots) / total_canonical if total_canonical else None
    )

    out_path = result_path(prompt_version, benchmark, run_id)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    lines = [
        f"# {benchmark} Three-Shot Results {run_id}",
        "",
        f"Prompt version: `{prompt_version}`",
        f"Benchmark: `{benchmark}`",
        f"Run id: `{run_id}`",
        f"Source three-shot run: `{run_path}`",
        "",
        "## Goal Check",
        "",
        "- Target experiment: three `gpt-5.5 xhigh` validation passes strip obvious false positives before the final gate analysis.",
        "- Success condition under current goal framing:",
        "  - H/M precision roughly `50%` to `60%`",
        "  - H/M recall as close to `100%` as possible",
        "",
        "## Scoring",
        "",
        f"- H/M confusion matrix: `TP={tp}`, `FP={fp}`, `TN={tn}`, `FN={fn}`",
        f"- H/M recall: `{recall:.1%}` (`{tp} / {tp + fn if (tp + fn) else 0}`)",
        f"- H/M precision: `{precision:.1%}` (`{tp} / {tp + fp if (tp + fp) else 0}`)",
        f"- H/M specificity: `{specificity:.1%}` (`{tn} / {tn + fp if (tn + fp) else 0}`)",
        f"- H/M abstentions / `Needs Review`: `{abstain}`",
        f"- Unique approved H/M findings present in the report: `{len(present_truth_roots)}`",
        f"- Unique approved H/M findings accepted as `Valid` with `High` / `Medium` severity: `{len(accepted_truth_roots)}`",
        f"- Present-root recall: `{present_root_recall:.1%}` (`{len(accepted_truth_roots)} / {len(present_truth_roots) if present_truth_roots else 0}`)",
    ]

    if total_canonical is not None and end_to_end_unique_recall is not None:
        lines.extend(
            [
                f"- Total canonical approved C4 H/M findings: `{total_canonical}`",
                f"- End-to-end unique recall: `{end_to_end_unique_recall:.1%}` (`{len(accepted_truth_roots)} / {total_canonical}`)",
            ]
        )

    lines.extend(
        [
            f"- Truth-valid H/M findings missed: `{', '.join(false_negatives) if false_negatives else '-'}`",
            f"- Truth-invalid H/M findings incorrectly accepted: `{', '.join(false_positives) if false_positives else '-'}`",
            "",
            "## Per-Finding Decisions",
            "",
            "| Finding | Title | Decision | Confidence | Bug Exists | Severity | Truth | Matching C4 Listing | Root Cause Family |",
            "| --- | --- | --- | --- | --- | --- | --- | --- | --- |",
        ]
    )

    if results_rows:
        for row in results_rows:
            lines.append(
                "| {fid} / `{report_id}` | {title} | {decision} | {confidence} | {bug_exists} | {severity} | {truth} | `{c4}` | `{root}` |".format(
                    fid=row["finding_id"],
                    report_id=clean(row["report_id"]),
                    title=clean(row["title"]),
                    decision=clean(row["decision"]),
                    confidence=clean(row["confidence"]),
                    bug_exists=clean(row["bug_exists"]),
                    severity=clean(row["severity"]),
                    truth=clean(row["truth"]),
                    c4=clean(row["matching_c4_listing"]),
                    root=clean(row["root_cause_family"]),
                )
            )
    else:
        lines.append("| - | No completed finding blocks found. | - | - | - | - | - | - | - |")

    out_path.write_text("\n".join(lines) + "\n")

    payload = {
        "benchmark": benchmark,
        "prompt_version": prompt_version,
        "run_id": run_id,
        "run_path": str(run_path),
        "result_path": str(out_path),
        "metrics": {
            "tp": tp,
            "fp": fp,
            "tn": tn,
            "fn": fn,
            "abstain": abstain,
            "recall": recall,
            "precision": precision,
            "specificity": specificity,
            "unique_present_roots": len(present_truth_roots),
            "unique_accepted_roots": len(accepted_truth_roots),
            "present_root_recall": present_root_recall,
            "total_canonical_approved_hm_findings": total_canonical,
            "end_to_end_unique_recall": end_to_end_unique_recall,
        },
        "false_negatives": false_negatives,
        "false_positives": false_positives,
    }
    score_json_path(prompt_version, benchmark, run_id).write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n"
    )
    return payload


def cmd_prepare_scope(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    path = init_screen(
        scope_screen_path(args.prompt_version, args.benchmark, args.run_id),
        "Three-Shot Stage 1 Scope Screen",
        args.benchmark,
        args.prompt_version,
    )
    prompt_text = render_scope_prompt(args.prompt_version, args.benchmark, args.run_id, config)
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "stage_path": str(path),
        "worker_type": "three-shot-r1-scope",
        "worker_model": worker_model(config, "r1", STAGE_WORKER_MODEL),
        "worker_reasoning_effort": worker_reasoning(config, "r1", STAGE_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r1.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_token(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    input_findings = kept_after_scope(args.prompt_version, args.benchmark, args.run_id)
    path = init_screen(
        token_screen_path(args.prompt_version, args.benchmark, args.run_id),
        "Three-Shot Stage 2 Unsupported-Token Screen",
        args.benchmark,
        args.prompt_version,
    )
    prompt_text = render_token_prompt(args.prompt_version, args.benchmark, args.run_id, config)
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "stage_path": str(path),
        "input_findings": len(input_findings),
        "worker_type": "three-shot-r2-token",
        "worker_model": worker_model(config, "r2", STAGE_WORKER_MODEL),
        "worker_reasoning_effort": worker_reasoning(config, "r2", STAGE_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r2.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_final(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    input_findings = kept_after_token(args.prompt_version, args.benchmark, args.run_id)
    path = init_stage3_run(args.prompt_version, args.benchmark, args.run_id)
    prompt_text = render_stage3_prompt(args.prompt_version, args.benchmark, args.run_id, config)
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "stage_path": str(path),
        "input_findings": len(input_findings),
        "worker_type": "three-shot-r3-final-validation",
        "worker_model": worker_model(config, "r3", STAGE_WORKER_MODEL),
        "worker_reasoning_effort": worker_reasoning(config, "r3", STAGE_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r3.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_dedup(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    candidates = dedup_candidates(args.prompt_version, args.benchmark, args.run_id)
    path = init_dedup_screen(args.prompt_version, args.benchmark, args.run_id, reset=args.reset)
    prompt_text = render_dedup_prompt(args.prompt_version, args.benchmark, args.run_id, config)
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "stage_path": str(path),
        "input_findings": len(candidates),
        "worker_type": "three-shot-r4-canonicalization",
        "worker_model": worker_model(config, "r4", DEDUP_WORKER_MODEL),
        "worker_reasoning_effort": worker_reasoning(config, "r4", DEDUP_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r4.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_assemble(args: argparse.Namespace) -> None:
    resolve_run_args(args)
    print(json.dumps(assemble_run(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_apply_dedup(args: argparse.Namespace) -> None:
    resolve_run_args(args)
    print(json.dumps(apply_dedup(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_score_prompt(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    run_path = final_run_path(args.prompt_version, args.benchmark, args.run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    prompt_text = render_stage_prompt(
        "scoring.md",
        {
            "REPO_ROOT": REPO_ROOT,
            "PROMPT_VERSION": args.prompt_version,
            "BENCHMARK": args.benchmark,
            "RUN_ID": args.run_id,
            "RUN_PATH": run_path,
            "RESULT_PATH": result_path(args.prompt_version, args.benchmark, args.run_id),
            "TRUTH_PATH": truth_path(args.benchmark),
            "REPORT": report_path(args.benchmark),
            "SOURCE_ROOT": benchmark_source_root(args.benchmark),
            "C4_APPROVED_FINDINGS": REPO_ROOT / "C4_APPROVED_FINDINGS.md",
            "APPROVED_FINDINGS_KEY": REPO_ROOT / "APPROVED_FINDINGS_KEY.md",
        },
    )
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "run_path": str(run_path),
        "result_path": str(result_path(args.prompt_version, args.benchmark, args.run_id)),
        "worker_type": "three-shot-scoring",
        "worker_model": worker_model(config, "scoring", SCORING_WORKER_MODEL),
        "worker_reasoning_effort": worker_reasoning(config, "scoring", SCORING_WORKER_REASONING),
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_score_local(args: argparse.Namespace) -> None:
    resolve_run_args(args)
    print(json.dumps(score_three_shot(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def add_run_args(subparser: argparse.ArgumentParser) -> None:
    subparser.add_argument("--config", default=str(DEFAULT_CONFIG_PATH), help="YAML config path.")
    subparser.add_argument("--prompt-version", default=None)
    subparser.add_argument("--benchmark", default=None)
    subparser.add_argument("--run-id", default=None)


def main() -> None:
    parser = argparse.ArgumentParser(description="Prepare, assemble, and score three-shot validation experiments.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    prepare_scope = subparsers.add_parser("prepare-scope", help="Initialize stage 1 scope screening and emit the worker prompt.")
    add_run_args(prepare_scope)
    prepare_scope.add_argument("--include-prompt", action="store_true")
    prepare_scope.add_argument("--write-prompt")
    prepare_scope.set_defaults(func=cmd_prepare_scope)

    prepare_token = subparsers.add_parser("prepare-token", help="Initialize stage 2 unsupported-token screening and emit the worker prompt.")
    add_run_args(prepare_token)
    prepare_token.add_argument("--include-prompt", action="store_true")
    prepare_token.add_argument("--write-prompt")
    prepare_token.set_defaults(func=cmd_prepare_token)

    prepare_final = subparsers.add_parser("prepare-final", help="Initialize stage 3 final validation and emit the worker prompt.")
    add_run_args(prepare_final)
    prepare_final.add_argument("--include-prompt", action="store_true")
    prepare_final.add_argument("--write-prompt")
    prepare_final.set_defaults(func=cmd_prepare_final)

    prepare_dedup = subparsers.add_parser("prepare-dedup", help="Initialize optional round 4 canonicalization cleanup and emit the worker prompt.")
    add_run_args(prepare_dedup)
    prepare_dedup.add_argument("--include-prompt", action="store_true")
    prepare_dedup.add_argument("--write-prompt")
    prepare_dedup.add_argument("--reset", action="store_true", help="Reset the round 4 screen before emitting the prompt.")
    prepare_dedup.set_defaults(func=cmd_prepare_dedup)

    assemble = subparsers.add_parser("assemble", help="Assemble the final three-shot run from stages 1-3.")
    add_run_args(assemble)
    assemble.set_defaults(func=cmd_assemble)

    apply_dedup_cmd = subparsers.add_parser("apply-dedup", help="Apply round 4 canonicalization decisions to create submission candidates.")
    add_run_args(apply_dedup_cmd)
    apply_dedup_cmd.set_defaults(func=cmd_apply_dedup)

    score_prompt = subparsers.add_parser("score-prompt", help="Emit the scoring worker prompt for an assembled three-shot run.")
    add_run_args(score_prompt)
    score_prompt.add_argument("--include-prompt", action="store_true")
    score_prompt.add_argument("--write-prompt")
    score_prompt.set_defaults(func=cmd_score_prompt)

    score_local = subparsers.add_parser("score-local", help="Locally score an assembled three-shot run.")
    add_run_args(score_local)
    score_local.set_defaults(func=cmd_score_local)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
