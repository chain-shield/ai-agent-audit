#!/usr/bin/env python3
"""Prepare, assemble, and score three-shot validation experiments.

Three-shot validation splits the full-report pass into:
1. scope / known-issue screening
2. unsupported-token screening
3. full validation on the surviving findings
4. optional canonicalization cleanup on the surviving H/M candidates
4a. optional second V12 overlap sweep on post-canonicalization candidates
5. optional PoC generation and verification on submission candidates

This is intended to cheaply strip obvious false positives before the expensive
final gate analysis while preserving recall on approved H/M roots.
"""

from __future__ import annotations

import argparse
import glob
import json
import re
import shlex
from dataclasses import dataclass
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
TRUTH_ROOT = Path("/Users/apmfree/.ai-agent-audit-validation-truth")
APPEND_ANCHOR = "<!-- APPEND FINDING BLOCKS ABOVE THIS LINE -->"
REPORT_FINDING_RE = re.compile(r"^## \[([HML]-\d+)\]\. (.+)$")
RAW_FINDING_RE = re.compile(r"^### ([HML]-\d+) / `([^`]+)`")
ACTIVE_CONFIG: dict[str, object] = {}


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
V12_CHECKLIST_PATH = THREE_SHOT_ROOT / "v12-checklist.md"
DEFAULT_CONFIG_PATH = THREE_SHOT_ROOT / "config.yaml"
STAGE_WORKER_MODEL = "gpt-5.5"
STAGE_WORKER_REASONING = "xhigh"
DEDUP_WORKER_MODEL = "gpt-5.4"
DEDUP_WORKER_REASONING = "high"
V12_SWEEP_WORKER_MODEL = "gpt-5.4"
V12_SWEEP_WORKER_REASONING = "high"
POC_WORKER_MODEL = "gpt-5.5"
POC_WORKER_REASONING = "xhigh"
REPORT_WORKER_MODEL = "gpt-5.5"
REPORT_WORKER_REASONING = "xhigh"
SCORING_WORKER_MODEL = "gpt-5.4"
SCORING_WORKER_REASONING = "xhigh"
DEFAULT_WORKER_LAUNCHER = "/Users/apmfree/codex-minimal-worker"
SCREEN_ANCHOR = "<!-- APPEND SCREEN ROWS ABOVE THIS LINE -->"


def config_path_value(keys: list[str]) -> str:
    return nested_get(ACTIVE_CONFIG, keys, "")


def resolve_configured_path(value: str) -> Path:
    path = Path(value).expanduser()
    if path.is_absolute():
        return path
    return (REPO_ROOT / path).resolve()


def report_path(benchmark: str) -> Path:
    configured = config_path_value(["paths", "audit_report"])
    if configured:
        return resolve_configured_path(configured)
    return REPO_ROOT / benchmark / "report" / "audit-report.md"


def benchmark_source_root(benchmark: str) -> Path:
    configured = config_path_value(["paths", "source_root"])
    if configured:
        return resolve_configured_path(configured)
    return REPO_ROOT / benchmark / "source"


def truth_path(benchmark: str) -> Path:
    configured = config_path_value(["paths", "truth_file"])
    if configured:
        return resolve_configured_path(configured)
    return TRUTH_ROOT / f"{benchmark}.md"


def truth_key_path(benchmark: str) -> Path:
    configured = config_path_value(["paths", "truth_file"])
    if configured:
        return resolve_configured_path(configured)
    return REPO_ROOT / "APPROVED_FINDINGS_KEY.md"


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


def v12_sweep_screen_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "v12-sweeps" / prompt_version / f"{benchmark}-{run_id}.md"


def final_run_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "runs" / prompt_version / f"{benchmark}-{run_id}.md"


def submission_candidates_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "submission-candidates" / prompt_version / f"{benchmark}-{run_id}.md"


def poc_run_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "poc-runs" / prompt_version / f"{benchmark}-{run_id}.md"


def poc_run_json_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "poc-runs" / prompt_version / f"{benchmark}-{run_id}.json"


def poc_unit_dir(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "poc-runs" / prompt_version / f"{benchmark}-{run_id}"


def poc_unit_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return poc_unit_dir(prompt_version, benchmark, run_id) / f"{finding_id}.md"


def poc_unit_json_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return poc_unit_dir(prompt_version, benchmark, run_id) / f"{finding_id}.json"


def poc_review_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "poc-verification" / prompt_version / f"{benchmark}-{run_id}.md"


def poc_review_json_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "poc-verification" / prompt_version / f"{benchmark}-{run_id}.json"


def poc_review_unit_dir(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "poc-verification" / prompt_version / f"{benchmark}-{run_id}"


def poc_review_unit_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return poc_review_unit_dir(prompt_version, benchmark, run_id) / f"{finding_id}.md"


def poc_review_unit_json_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return poc_review_unit_dir(prompt_version, benchmark, run_id) / f"{finding_id}.json"


def finding_report_unit_dir(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "finding-reports" / prompt_version / f"{benchmark}-{run_id}"


def finding_title_from_block(block: str) -> str:
    match = re.search(r"^- Finding Title:\s*(.+)$", block, re.MULTILINE)
    if match:
        return match.group(1).strip()
    heading = re.search(r"^###\s+[HML]-\d+\s*/\s*`[^`]+`\s*$", block, re.MULTILINE)
    if heading:
        return heading.group(0).strip("# `")
    return "finding-report"


def filename_slug(value: str, max_chars: int = 50) -> str:
    slug = re.sub(r"[^A-Za-z0-9]+", "-", value).strip("-")
    if len(slug) > max_chars:
        slug = slug[:max_chars].rstrip("-")
    return slug or "finding-report"


def finding_report_slug(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> str:
    try:
        block = submission_candidate_blocks(prompt_version, benchmark, run_id).get(finding_id, "")
    except SystemExit:
        block = ""
    return filename_slug(finding_title_from_block(block))


def finding_report_unit_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return finding_report_unit_dir(prompt_version, benchmark, run_id) / (
        f"{finding_id}-{finding_report_slug(prompt_version, benchmark, run_id, finding_id)}.md"
    )


def finding_report_unit_json_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return finding_report_unit_dir(prompt_version, benchmark, run_id) / f"{finding_id}.json"


def finding_report_review_unit_dir(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "finding-report-reviews" / prompt_version / f"{benchmark}-{run_id}"


def finding_report_review_unit_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return finding_report_review_unit_dir(prompt_version, benchmark, run_id) / (
        f"{finding_id}-{finding_report_slug(prompt_version, benchmark, run_id, finding_id)}.md"
    )


def finding_report_review_unit_json_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return finding_report_review_unit_dir(prompt_version, benchmark, run_id) / f"{finding_id}.json"


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


def round4a_input_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "inputs" / "r4a" / prompt_version / f"{benchmark}-{run_id}.md"


def round5_input_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return THREE_SHOT_ROOT / "inputs" / "r5" / prompt_version / f"{benchmark}-{run_id}" / f"{finding_id}.md"


def round7_input_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return THREE_SHOT_ROOT / "inputs" / "r7" / prompt_version / f"{benchmark}-{run_id}" / f"{finding_id}.md"


def round8_input_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return THREE_SHOT_ROOT / "inputs" / "r8" / prompt_version / f"{benchmark}-{run_id}" / f"{finding_id}.md"


def validation_prompt_path(prompt_version: str) -> Path:
    path = THREE_SHOT_PROMPT_ROOT / f"validation-{prompt_version}.md"
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
    stack: list[tuple[int, dict[str, object] | list[object], dict[str, object] | None, str | None]] = [
        (-1, root, None, None)
    ]

    for raw_line in path.read_text().splitlines():
        if not raw_line.strip() or raw_line.lstrip().startswith("#"):
            continue
        line = raw_line.split("#", 1)[0].rstrip()
        if not line.strip():
            continue
        indent = len(line) - len(line.lstrip(" "))
        if indent % 2:
            raise SystemExit(f"Invalid indentation in config {path}: {raw_line}")
        while stack and indent <= stack[-1][0]:
            stack.pop()
        parent = stack[-1][1]

        stripped = line.strip()
        if stripped.startswith("- "):
            if not isinstance(parent, list):
                current_indent, current, parent_container, parent_key = stack[-1]
                if (
                    isinstance(current, dict)
                    and not current
                    and isinstance(parent_container, dict)
                    and parent_key
                ):
                    parent = []
                    parent_container[parent_key] = parent
                    stack[-1] = (current_indent, parent, parent_container, parent_key)
                else:
                    raise SystemExit(f"Invalid YAML list item in config {path}: {raw_line}")
            parent.append(parse_yaml_scalar(stripped[2:].strip()))
            continue

        if ":" not in stripped:
            raise SystemExit(f"Invalid YAML line in config {path}: {raw_line}")

        if not isinstance(parent, dict):
            raise SystemExit(f"Invalid YAML mapping inside list in config {path}: {raw_line}")

        key, value = stripped.split(":", 1)
        key = key.strip()
        value = value.strip()
        if value:
            parent[key] = parse_yaml_scalar(value)
            continue

        child: dict[str, object] = {}
        parent[key] = child
        stack.append((indent, child, parent, key))

    return root


def nested_get(config: dict[str, object], keys: list[str], default: str) -> str:
    current: object = config
    for key in keys:
        if not isinstance(current, dict) or key not in current:
            return default
        current = current[key]
    return str(current) if current is not None else default


def nested_value(config: dict[str, object], keys: list[str]) -> object | None:
    current: object = config
    for key in keys:
        if not isinstance(current, dict) or key not in current:
            return None
        current = current[key]
    return current


def load_config(config_path: str | None) -> dict[str, object]:
    return load_simple_yaml(Path(config_path or DEFAULT_CONFIG_PATH))


def resolve_run_args(args: argparse.Namespace) -> dict[str, object]:
    global ACTIVE_CONFIG
    config = load_config(getattr(args, "config", None))
    ACTIVE_CONFIG = config
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


def worker_launcher(config: dict[str, object], worker: str) -> str:
    return nested_get(
        config,
        ["workers", worker, "launcher"],
        nested_get(config, ["workers", "default", "launcher"], DEFAULT_WORKER_LAUNCHER),
    )


def worker_payload(config: dict[str, object], worker: str, default_model: str, default_reasoning: str) -> dict[str, str | bool]:
    model = worker_model(config, worker, default_model)
    reasoning = worker_reasoning(config, worker, default_reasoning)
    launcher = worker_launcher(config, worker)
    return {
        "worker_model": model,
        "worker_reasoning_effort": reasoning,
        "worker_launcher": launcher,
        "worker_spawn_command_template": (
            f"{shlex.quote(launcher)} exec -C {shlex.quote(str(REPO_ROOT))} "
            f"-m {shlex.quote(model)} -c model_reasoning_effort={shlex.quote(reasoning)} "
            "--dangerously-bypass-approvals-and-sandbox - < <worker_prompt_path>"
        ),
        "requires_minimal_codex_worker": True,
        "worker_launch_note": "Launch this unit with worker_launcher so plugins and MCP servers stay disabled.",
    }


def attach_worker_prompt_path(payload: dict[str, object], prompt_text: str, prompt_path: Path) -> None:
    prompt_path.write_text(prompt_text)
    payload["worker_prompt_path"] = str(prompt_path)
    template = payload.get("worker_spawn_command_template")
    if isinstance(template, str):
        payload["worker_spawn_command"] = template.replace("<worker_prompt_path>", shlex.quote(str(prompt_path)))


def worker_replacements(config: dict[str, object], worker: str, default_model: str, default_reasoning: str) -> dict[str, str]:
    return {
        "WORKER_MODEL": worker_model(config, worker, default_model),
        "WORKER_REASONING": worker_reasoning(config, worker, default_reasoning),
        "WORKER_LAUNCHER": worker_launcher(config, worker),
    }


def contest_replacements(config: dict[str, object]) -> dict[str, str]:
    return {
        "CONTEST_POC_INSTRUCTIONS": nested_get(
            config,
            ["contest", "poc_instructions"],
            "Read the contest README near the end of the document and follow the contest-specific PoC submission instructions.",
        ),
        "CONTEST_REPO_LAYOUT": nested_get(
            config,
            ["contest", "repo_layout"],
            "Identify the relevant repository or package under {{SOURCE_ROOT}} and place each PoC in that repo's own test folder.",
        ),
        "CONTEST_TEST_COMMAND_HINTS": nested_get(
            config,
            ["contest", "test_command_hints"],
            "Use the repository README and existing tests to determine the minimal command that runs the new PoC.",
        ),
    }


def has_glob_magic(value: str) -> bool:
    return any(char in value for char in "*?[")


def expand_context_doc_entry(entry: object, source_root: Path) -> list[Path]:
    value = str(entry)
    value = value.replace("{{SOURCE_ROOT}}", str(source_root))
    value = value.replace("{{THREE_SHOT_ROOT}}", str(THREE_SHOT_ROOT))
    path_text = str(Path(value).expanduser())
    if has_glob_magic(path_text):
        return [Path(match) for match in sorted(glob.glob(path_text))]
    path = Path(path_text)
    if path.is_absolute():
        return [path]
    return [(REPO_ROOT / path).resolve()]


def benchmark_scope_docs(source_root: Path) -> list[Path]:
    configured_docs = nested_value(ACTIVE_CONFIG, ["context_docs"])
    if isinstance(configured_docs, list):
        paths: list[Path] = []
        for entry in configured_docs:
            paths.extend(expand_context_doc_entry(entry, source_root))
    else:
        paths = [source_root / "README.md"]
        paths.extend(sorted(source_root.glob("*-scope.md")))
        paths.extend(sorted(source_root.glob("*-docs.md")))
        paths.extend([source_root / "v12-findings.md", V12_CHECKLIST_PATH])

    deduped: list[Path] = []
    seen: set[str] = set()
    for path in paths:
        key = str(path)
        if key not in seen:
            deduped.append(path)
            seen.add(key)
    return deduped


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
    for _ in range(3):
        original = text
        for key, value in replacements.items():
            text = text.replace("{{" + key + "}}", str(value))
        if text == original:
            break
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


def parse_truth_key(key_path: Path) -> dict[str, dict[str, str]]:
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


def init_v12_sweep_screen(prompt_version: str, benchmark: str, run_id: str, reset: bool = False) -> Path:
    path = v12_sweep_screen_path(prompt_version, benchmark, run_id)
    if path.exists() and not reset:
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    source_root = benchmark_source_root(benchmark)
    content = f"""# {benchmark} Three-Shot Round 4a V12 Overlap Sweep

Status: In progress
Source submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`
Output filtered submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`
Benchmark source root: `{source_root}`

Mandatory benchmark docs:
{markdown_path_list(benchmark_scope_docs(source_root))}

## Decisions

| Finding | Finding Title | Decision | Confidence | Closest V12 Finding | Material Difference | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- |
{SCREEN_ANCHOR}
"""
    path.write_text(content)
    return path


def init_poc_run(prompt_version: str, benchmark: str, run_id: str, reset: bool = False) -> Path:
    path = poc_run_path(prompt_version, benchmark, run_id)
    if path.exists() and not reset:
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    content = f"""# {benchmark} Three-Shot Round 5 PoC Generation

Status: In progress
Source submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`
JSON summary: `{poc_run_json_path(prompt_version, benchmark, run_id)}`

## PoC Results

| Finding | Finding Title | Status | PoC Test Path | Test Command | Test Result | Notes |
| --- | --- | --- | --- | --- | --- | --- |
{SCREEN_ANCHOR}
"""
    path.write_text(content)
    return path


def init_poc_unit_run(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    reset: bool = False,
) -> Path:
    path = poc_unit_path(prompt_version, benchmark, run_id, finding_id)
    if path.exists() and not reset:
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    content = f"""# {benchmark} Three-Shot Round 5 PoC Generation: {finding_id}

Status: In progress
Source submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`
Per-finding input: `{round5_input_path(prompt_version, benchmark, run_id, finding_id)}`
JSON summary: `{poc_unit_json_path(prompt_version, benchmark, run_id, finding_id)}`

## PoC Results

| Finding | Finding Title | Status | PoC Test Path | Test Command | Test Result | Notes |
| --- | --- | --- | --- | --- | --- | --- |
{SCREEN_ANCHOR}
"""
    path.write_text(content)
    return path


def init_poc_review(prompt_version: str, benchmark: str, run_id: str, reset: bool = False) -> Path:
    path = poc_review_path(prompt_version, benchmark, run_id)
    if path.exists() and not reset:
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    content = f"""# {benchmark} Three-Shot Round 6 PoC Verification

Status: In progress
Source submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`
Source R5 PoC run: `{poc_run_path(prompt_version, benchmark, run_id)}`
JSON summary: `{poc_review_json_path(prompt_version, benchmark, run_id)}`

## PoC Verification Results

| Finding | Finding Title | Status | Final PoC Test Path | Test Command | Verification Result | Notes |
| --- | --- | --- | --- | --- | --- | --- |
{SCREEN_ANCHOR}
"""
    path.write_text(content)
    return path


def init_poc_review_unit(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    reset: bool = False,
) -> Path:
    path = poc_review_unit_path(prompt_version, benchmark, run_id, finding_id)
    if path.exists() and not reset:
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    content = f"""# {benchmark} Three-Shot Round 6 PoC Verification: {finding_id}

Status: In progress
Source submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`
Source R5 PoC run: `{poc_unit_path(prompt_version, benchmark, run_id, finding_id)}`
JSON summary: `{poc_review_unit_json_path(prompt_version, benchmark, run_id, finding_id)}`

## PoC Verification Results

| Finding | Finding Title | Status | Final PoC Test Path | Test Command | Verification Result | Notes |
| --- | --- | --- | --- | --- | --- | --- |
{SCREEN_ANCHOR}
"""
    path.write_text(content)
    return path


def init_finding_report_unit(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    reset: bool = False,
) -> Path:
    path = finding_report_unit_path(prompt_version, benchmark, run_id, finding_id)
    if path.exists() and not reset:
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    content = f"""# {finding_id} C4 Finding Report

Status: In progress
Benchmark: `{benchmark}`
Source input: `{round7_input_path(prompt_version, benchmark, run_id, finding_id)}`
JSON summary: `{finding_report_unit_json_path(prompt_version, benchmark, run_id, finding_id)}`

<!-- R7 WORKER REPLACES THIS FILE WITH THE SUBMISSION-READY REPORT -->
"""
    path.write_text(content)
    return path


def init_finding_report_review_unit(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    reset: bool = False,
) -> Path:
    path = finding_report_review_unit_path(prompt_version, benchmark, run_id, finding_id)
    if path.exists() and not reset:
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    content = f"""# {finding_id} C4 Finding Report Review

Status: In progress
Benchmark: `{benchmark}`
Source input: `{round8_input_path(prompt_version, benchmark, run_id, finding_id)}`
Report under review: `{finding_report_unit_path(prompt_version, benchmark, run_id, finding_id)}`
JSON summary: `{finding_report_review_unit_json_path(prompt_version, benchmark, run_id, finding_id)}`

<!-- R8 WORKER REPLACES THIS FILE WITH THE REVIEW RESULT -->
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


def parse_v12_sweep_screen(path: Path) -> dict[str, dict[str, str]]:
    if not path.exists():
        raise SystemExit(f"Missing required V12 sweep screen file: {path}")

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
                "closest_v12_finding": mapped.get("closest_v12_finding", ""),
                "material_difference": mapped.get("material_difference", ""),
                "reason_category": mapped.get("reason_category", ""),
                "reason": mapped.get("reason", ""),
            }
            continue
        rows[fid] = {
            "finding_id": fid,
            "title": parts[1],
            "decision": parts[2],
            "confidence": parts[3],
            "closest_v12_finding": parts[4],
            "material_difference": parts[5],
            "reason_category": parts[6],
            "reason": "|".join(parts[7:]).strip(),
        }
    return rows


def parse_poc_rows(path: Path) -> dict[str, dict[str, str]]:
    if not path.exists():
        raise SystemExit(f"Missing required PoC result file: {path}")

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
                "status": mapped.get("status", ""),
                "test_path": mapped.get("poc_test_path", mapped.get("final_poc_test_path", "")),
                "test_command": mapped.get("test_command", ""),
                "result": mapped.get("test_result", mapped.get("verification_result", "")),
                "notes": mapped.get("notes", ""),
            }
            continue
        rows[fid] = {
            "finding_id": fid,
            "title": parts[1],
            "status": parts[2],
            "test_path": parts[3],
            "test_command": parts[4],
            "result": parts[5],
            "notes": "|".join(parts[6:]).strip(),
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


def write_round4a_input(path: Path, prompt_version: str, benchmark: str, run_id: str) -> Path:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    path.parent.mkdir(parents=True, exist_ok=True)
    content = "\n".join(
        [
            f"# {benchmark} Round 4a V12 Sweep Input",
            "",
            f"Source submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`",
            f"Candidate count: `{len(blocks)}`",
            "",
            "This file contains only post-R4 submission candidates. R4a must decide whether each candidate is",
            "materially distinct from the configured V12 / prior-findings context.",
            "",
            "## Candidates",
            "",
            "\n\n".join(block.strip() for block in blocks.values()),
            "",
        ]
    )
    path.write_text(content)
    return path


def require_v12_sweep_rows(prompt_version: str, benchmark: str, run_id: str) -> dict[str, dict[str, str]]:
    rows = parse_v12_sweep_screen(v12_sweep_screen_path(prompt_version, benchmark, run_id))
    expected = set(raw_finding_blocks(submission_candidates_path(prompt_version, benchmark, run_id)))
    if expected and not rows:
        raise SystemExit("V12 sweep screen has no decisions yet. Complete round 4a before applying it.")
    missing = sorted(expected - set(rows))
    if missing:
        raise SystemExit(
            "V12 sweep screen is incomplete. Missing findings: " + ", ".join(missing[:10])
        )
    return rows


def require_poc_rows(prompt_version: str, benchmark: str, run_id: str) -> dict[str, dict[str, str]]:
    rows = parse_poc_rows(poc_run_path(prompt_version, benchmark, run_id))
    expected = set(raw_finding_blocks(submission_candidates_path(prompt_version, benchmark, run_id)))
    if expected and not rows:
        raise SystemExit("R5 PoC run has no result rows yet. Complete round 5 before preparing round 6.")
    missing = sorted(expected - set(rows))
    if missing:
        raise SystemExit(
            "R5 PoC run is incomplete. Missing findings: " + ", ".join(missing[:10])
        )
    json_path = poc_run_json_path(prompt_version, benchmark, run_id)
    if not json_path.exists():
        raise SystemExit(f"Missing R5 PoC JSON summary: {json_path}")
    try:
        payload = json.loads(json_path.read_text())
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Invalid R5 PoC JSON summary {json_path}: {exc}") from None
    json_rows = {
        row.get("finding_id")
        for row in payload.get("results", [])
        if isinstance(row, dict) and row.get("finding_id")
    }
    missing_json = sorted(expected - json_rows)
    if missing_json:
        raise SystemExit(
            "R5 PoC JSON summary is incomplete. Missing findings: "
            + ", ".join(missing_json[:10])
        )
    return rows


def submission_candidate_blocks(prompt_version: str, benchmark: str, run_id: str) -> dict[str, str]:
    submission_path = submission_candidates_path(prompt_version, benchmark, run_id)
    if not submission_path.exists():
        raise SystemExit(f"Missing submission candidates file: {submission_path}")
    return raw_finding_blocks(submission_path)


def validate_finding_id(finding_id: str, blocks: dict[str, str]) -> str:
    if not re.fullmatch(r"[HML]-\d+", finding_id):
        raise SystemExit(f"Invalid finding id: {finding_id}")
    if finding_id not in blocks:
        raise SystemExit(f"Finding {finding_id} is not present in the submission candidates file.")
    return finding_id


def write_round5_input(
    path: Path,
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    block: str,
) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    content = "\n".join(
        [
            f"# {benchmark} Round 5 Single-Finding PoC Input: {finding_id}",
            "",
            f"Source submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`",
            "Finding count: `1`",
            "",
            "## Candidate",
            "",
            block.strip(),
            "",
        ]
    )
    path.write_text(content)
    return path


def json_has_finding(path: Path, finding_id: str, result_key: str = "results") -> bool:
    if not path.exists():
        return False
    try:
        payload = json.loads(path.read_text())
    except json.JSONDecodeError:
        return False
    if not isinstance(payload, dict):
        return False
    if payload.get("finding_id") == finding_id:
        return True
    return any(
        isinstance(row, dict) and row.get("finding_id") == finding_id
        for row in payload.get(result_key, [])
    )


def poc_unit_complete(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> bool:
    try:
        rows = parse_poc_rows(poc_unit_path(prompt_version, benchmark, run_id, finding_id))
    except SystemExit:
        return False
    return finding_id in rows and json_has_finding(
        poc_unit_json_path(prompt_version, benchmark, run_id, finding_id),
        finding_id,
    )


def poc_review_unit_complete(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> bool:
    try:
        rows = parse_poc_rows(poc_review_unit_path(prompt_version, benchmark, run_id, finding_id))
    except SystemExit:
        return False
    return finding_id in rows and json_has_finding(
        poc_review_unit_json_path(prompt_version, benchmark, run_id, finding_id),
        finding_id,
    )


def next_poc_finding(prompt_version: str, benchmark: str, run_id: str) -> str | None:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    for finding_id in blocks:
        if not poc_unit_complete(prompt_version, benchmark, run_id, finding_id):
            return finding_id
    return None


def next_poc_review_finding(prompt_version: str, benchmark: str, run_id: str) -> str | None:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    for finding_id in blocks:
        if not poc_unit_complete(prompt_version, benchmark, run_id, finding_id):
            continue
        if not poc_review_unit_complete(prompt_version, benchmark, run_id, finding_id):
            return finding_id
    return None


def require_poc_unit(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> dict[str, dict[str, str]]:
    rows = parse_poc_rows(poc_unit_path(prompt_version, benchmark, run_id, finding_id))
    if finding_id not in rows:
        raise SystemExit(f"R5 PoC unit for {finding_id} has no result row yet.")
    json_path = poc_unit_json_path(prompt_version, benchmark, run_id, finding_id)
    if not json_has_finding(json_path, finding_id):
        raise SystemExit(f"Missing or incomplete R5 PoC JSON summary for {finding_id}: {json_path}")
    return rows


def require_poc_review_unit(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> dict[str, dict[str, str]]:
    rows = parse_poc_rows(poc_review_unit_path(prompt_version, benchmark, run_id, finding_id))
    if finding_id not in rows:
        raise SystemExit(f"R6 PoC review unit for {finding_id} has no result row yet.")
    json_path = poc_review_unit_json_path(prompt_version, benchmark, run_id, finding_id)
    if not json_has_finding(json_path, finding_id):
        raise SystemExit(f"Missing or incomplete R6 PoC review JSON summary for {finding_id}: {json_path}")
    return rows


def poc_review_status(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> str:
    if not poc_review_unit_complete(prompt_version, benchmark, run_id, finding_id):
        return ""
    rows = parse_poc_rows(poc_review_unit_path(prompt_version, benchmark, run_id, finding_id))
    return rows.get(finding_id, {}).get("status", "")


def reportable_after_poc_review(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> bool:
    return poc_review_status(prompt_version, benchmark, run_id, finding_id) in {
        "Verified PoC",
        "Fixed And Verified PoC",
    }


def report_unit_complete(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> bool:
    path = finding_report_unit_path(prompt_version, benchmark, run_id, finding_id)
    if not path.exists():
        return False
    text = path.read_text()
    if "Status: In progress" in text or "R7 WORKER REPLACES THIS FILE" in text:
        return False
    return json_has_finding(
        finding_report_unit_json_path(prompt_version, benchmark, run_id, finding_id),
        finding_id,
    )


def report_review_unit_complete(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> bool:
    path = finding_report_review_unit_path(prompt_version, benchmark, run_id, finding_id)
    if not path.exists():
        return False
    text = path.read_text()
    if "Status: In progress" in text or "R8 WORKER REPLACES THIS FILE" in text:
        return False
    return json_has_finding(
        finding_report_review_unit_json_path(prompt_version, benchmark, run_id, finding_id),
        finding_id,
    )


def next_report_finding(prompt_version: str, benchmark: str, run_id: str) -> str | None:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    for finding_id in blocks:
        if not reportable_after_poc_review(prompt_version, benchmark, run_id, finding_id):
            continue
        if not report_unit_complete(prompt_version, benchmark, run_id, finding_id):
            return finding_id
    return None


def next_report_review_finding(prompt_version: str, benchmark: str, run_id: str) -> str | None:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    for finding_id in blocks:
        if not reportable_after_poc_review(prompt_version, benchmark, run_id, finding_id):
            continue
        if not report_unit_complete(prompt_version, benchmark, run_id, finding_id):
            continue
        if not report_review_unit_complete(prompt_version, benchmark, run_id, finding_id):
            return finding_id
    return None


def write_round7_input(
    path: Path,
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    block: str,
) -> Path:
    rows = require_poc_review_unit(prompt_version, benchmark, run_id, finding_id)
    row = rows[finding_id]
    if row.get("status") not in {"Verified PoC", "Fixed And Verified PoC"}:
        raise SystemExit(f"Finding {finding_id} is not reportable after R6 status: {row.get('status')}")
    path.parent.mkdir(parents=True, exist_ok=True)
    content = "\n".join(
        [
            f"# {benchmark} Round 7 Single-Finding Report Input: {finding_id}",
            "",
            f"Source submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`",
            f"R6 PoC review unit: `{poc_review_unit_path(prompt_version, benchmark, run_id, finding_id)}`",
            f"R6 JSON summary: `{poc_review_unit_json_path(prompt_version, benchmark, run_id, finding_id)}`",
            f"Final PoC test path: `{row.get('test_path', '-')}`",
            f"Verified test command: `{row.get('test_command', '-')}`",
            "",
            "## Candidate",
            "",
            block.strip(),
            "",
            "## R6 PoC Verification Row",
            "",
            f"- Status: {row.get('status', '-')}",
            f"- Final PoC Test Path: {row.get('test_path', '-')}",
            f"- Test Command: {row.get('test_command', '-')}",
            f"- Verification Result: {row.get('result', '-')}",
            f"- Notes: {row.get('notes', '-')}",
            "",
        ]
    )
    path.write_text(content)
    return path


def write_round8_input(
    path: Path,
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    block: str,
) -> Path:
    if not report_unit_complete(prompt_version, benchmark, run_id, finding_id):
        raise SystemExit(f"R7 finding report for {finding_id} is incomplete.")
    path.parent.mkdir(parents=True, exist_ok=True)
    content = "\n".join(
        [
            f"# {benchmark} Round 8 Single-Finding Report Review Input: {finding_id}",
            "",
            f"Source submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`",
            f"R7 report: `{finding_report_unit_path(prompt_version, benchmark, run_id, finding_id)}`",
            f"R7 JSON summary: `{finding_report_unit_json_path(prompt_version, benchmark, run_id, finding_id)}`",
            f"R6 PoC review unit: `{poc_review_unit_path(prompt_version, benchmark, run_id, finding_id)}`",
            "",
            "## Candidate",
            "",
            block.strip(),
            "",
        ]
    )
    path.write_text(content)
    return path


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
        "V12_CHECKLIST_PATH": V12_CHECKLIST_PATH,
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


def render_v12_sweep_prompt(prompt_version: str, benchmark: str, run_id: str, config: dict[str, object]) -> str:
    input_path = write_round4a_input(
        round4a_input_path(prompt_version, benchmark, run_id),
        prompt_version,
        benchmark,
        run_id,
    )
    return render_stage_prompt(
        "r4a.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **worker_replacements(config, "r4a", V12_SWEEP_WORKER_MODEL, V12_SWEEP_WORKER_REASONING),
            "INPUT_PATH": input_path,
            "STAGE_PATH": v12_sweep_screen_path(prompt_version, benchmark, run_id),
            "SUBMISSION_PATH": submission_candidates_path(prompt_version, benchmark, run_id),
        },
    )


def render_poc_prompt(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    config: dict[str, object],
) -> str:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    finding_id = validate_finding_id(finding_id, blocks)
    input_path = write_round5_input(
        round5_input_path(prompt_version, benchmark, run_id, finding_id),
        prompt_version,
        benchmark,
        run_id,
        finding_id,
        blocks[finding_id],
    )
    return render_stage_prompt(
        "r5.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **contest_replacements(config),
            **worker_replacements(config, "r5", POC_WORKER_MODEL, POC_WORKER_REASONING),
            "FINDING_ID": finding_id,
            "SUBMISSION_PATH": input_path,
            "FULL_SUBMISSION_PATH": submission_candidates_path(prompt_version, benchmark, run_id),
            "POC_RUN_PATH": poc_unit_path(prompt_version, benchmark, run_id, finding_id),
            "POC_RUN_JSON_PATH": poc_unit_json_path(prompt_version, benchmark, run_id, finding_id),
        },
    )


def render_poc_review_prompt(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    config: dict[str, object],
) -> str:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    finding_id = validate_finding_id(finding_id, blocks)
    input_path = write_round5_input(
        round5_input_path(prompt_version, benchmark, run_id, finding_id),
        prompt_version,
        benchmark,
        run_id,
        finding_id,
        blocks[finding_id],
    )
    poc_path = poc_unit_path(prompt_version, benchmark, run_id, finding_id)
    if not poc_path.exists():
        raise SystemExit(f"Missing R5 PoC unit file for {finding_id}: {poc_path}")
    require_poc_unit(prompt_version, benchmark, run_id, finding_id)
    return render_stage_prompt(
        "r6.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **contest_replacements(config),
            **worker_replacements(config, "r6", POC_WORKER_MODEL, POC_WORKER_REASONING),
            "FINDING_ID": finding_id,
            "SUBMISSION_PATH": input_path,
            "FULL_SUBMISSION_PATH": submission_candidates_path(prompt_version, benchmark, run_id),
            "POC_RUN_PATH": poc_path,
            "POC_RUN_JSON_PATH": poc_unit_json_path(prompt_version, benchmark, run_id, finding_id),
            "POC_REVIEW_PATH": poc_review_unit_path(prompt_version, benchmark, run_id, finding_id),
            "POC_REVIEW_JSON_PATH": poc_review_unit_json_path(prompt_version, benchmark, run_id, finding_id),
        },
    )


def render_finding_report_prompt(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    config: dict[str, object],
) -> str:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    finding_id = validate_finding_id(finding_id, blocks)
    input_path = write_round7_input(
        round7_input_path(prompt_version, benchmark, run_id, finding_id),
        prompt_version,
        benchmark,
        run_id,
        finding_id,
        blocks[finding_id],
    )
    return render_stage_prompt(
        "r7.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **worker_replacements(config, "r7", REPORT_WORKER_MODEL, REPORT_WORKER_REASONING),
            "FINDING_ID": finding_id,
            "REPORT_INPUT_PATH": input_path,
            "FINDING_REPORT_PATH": finding_report_unit_path(prompt_version, benchmark, run_id, finding_id),
            "FINDING_REPORT_JSON_PATH": finding_report_unit_json_path(prompt_version, benchmark, run_id, finding_id),
        },
    )


def render_finding_report_review_prompt(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    config: dict[str, object],
) -> str:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    finding_id = validate_finding_id(finding_id, blocks)
    input_path = write_round8_input(
        round8_input_path(prompt_version, benchmark, run_id, finding_id),
        prompt_version,
        benchmark,
        run_id,
        finding_id,
        blocks[finding_id],
    )
    return render_stage_prompt(
        "r8.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **worker_replacements(config, "r8", REPORT_WORKER_MODEL, REPORT_WORKER_REASONING),
            "FINDING_ID": finding_id,
            "REPORT_REVIEW_INPUT_PATH": input_path,
            "FINDING_REPORT_PATH": finding_report_unit_path(prompt_version, benchmark, run_id, finding_id),
            "FINDING_REPORT_REVIEW_PATH": finding_report_review_unit_path(prompt_version, benchmark, run_id, finding_id),
            "FINDING_REPORT_REVIEW_JSON_PATH": finding_report_review_unit_json_path(prompt_version, benchmark, run_id, finding_id),
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
            f"- Dropped by R4 cleanup: `{len(dropped)}`",
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


def apply_v12_sweep(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    candidates_path = submission_candidates_path(prompt_version, benchmark, run_id)
    if not candidates_path.exists():
        raise SystemExit(f"Missing submission candidates file: {candidates_path}")

    rows = require_v12_sweep_rows(prompt_version, benchmark, run_id)
    blocks = raw_finding_blocks(candidates_path)
    original_text = candidates_path.read_text()
    kept_blocks: list[str] = []
    excluded: list[str] = []
    kept: list[str] = []

    for fid, block in blocks.items():
        decision = rows[fid]["decision"].strip().lower()
        if decision == "keep":
            kept.append(fid)
            kept_blocks.append(block.strip())
            continue
        if decision.startswith("exclude") or decision == "drop":
            excluded.append(fid)
            continue
        raise SystemExit(f"Invalid R4a V12 sweep decision for {fid}: {rows[fid]['decision']}")

    if "## R4a V12 Sweep Summary" in original_text:
        prefix = original_text.split("## R4a V12 Sweep Summary", 1)[0].rstrip()
    else:
        prefix = original_text.split("## Submission Candidates", 1)[0].rstrip()
    content = "\n".join(
        [
            prefix,
            "",
            "## R4a V12 Sweep Summary",
            "",
            f"- Input candidates before R4a: `{len(kept) + len(excluded)}`",
            f"- Kept after R4a V12 sweep: `{len(kept)}`",
            f"- Excluded as V12 / prior-finding overlap: `{len(excluded)}`",
            f"- Excluded finding ids: `{', '.join(excluded) if excluded else '-'}`",
            f"- V12 sweep screen: `{v12_sweep_screen_path(prompt_version, benchmark, run_id)}`",
            "",
            "## Submission Candidates",
            "",
            "\n\n".join(kept_blocks) if kept_blocks else "_No H/M candidates remain after the V12 sweep._",
            "",
        ]
    )
    candidates_path.write_text(content)
    return {
        "benchmark": benchmark,
        "prompt_version": prompt_version,
        "run_id": run_id,
        "v12_sweep_screen_path": str(v12_sweep_screen_path(prompt_version, benchmark, run_id)),
        "submission_candidates_path": str(candidates_path),
        "input_candidates": len(kept) + len(excluded),
        "kept_candidates": len(kept),
        "excluded_candidates": len(excluded),
        "excluded_findings": excluded,
    }


def score_three_shot(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    run_path = final_run_path(prompt_version, benchmark, run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    blocks = raw_finding_blocks(run_path)
    findings = parse_findings(report_path(benchmark))
    truth_rows = parse_truth_key(truth_key_path(benchmark))

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
        **worker_payload(config, "r1", STAGE_WORKER_MODEL, STAGE_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r1.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
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
        **worker_payload(config, "r2", STAGE_WORKER_MODEL, STAGE_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r2.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
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
        **worker_payload(config, "r3", STAGE_WORKER_MODEL, STAGE_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r3.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
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
        **worker_payload(config, "r4", DEDUP_WORKER_MODEL, DEDUP_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r4.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_v12_sweep(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    blocks = submission_candidate_blocks(args.prompt_version, args.benchmark, args.run_id)
    path = init_v12_sweep_screen(args.prompt_version, args.benchmark, args.run_id, reset=args.reset)
    prompt_text = render_v12_sweep_prompt(args.prompt_version, args.benchmark, args.run_id, config)
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "stage_path": str(path),
        "input_findings": len(blocks),
        "worker_type": "three-shot-r4a-v12-sweep",
        **worker_payload(config, "r4a", V12_SWEEP_WORKER_MODEL, V12_SWEEP_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r4a.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_poc(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    blocks = submission_candidate_blocks(args.prompt_version, args.benchmark, args.run_id)
    requested_finding = args.finding_id or next_poc_finding(args.prompt_version, args.benchmark, args.run_id)
    if requested_finding is None:
        payload = {
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "worker_type": "none",
            "phase": "complete",
            "input_scope": "per_finding_post_r4_submission_candidates",
            "input_findings": len(blocks),
            "completed_findings": len(blocks),
            "source_candidates_path": str(submission_candidates_path(args.prompt_version, args.benchmark, args.run_id)),
            "summary": "All R5 per-finding PoC units are complete.",
        }
        print(json.dumps(payload, indent=2, sort_keys=True))
        return

    finding_id = validate_finding_id(requested_finding, blocks)
    input_path = write_round5_input(
        round5_input_path(args.prompt_version, args.benchmark, args.run_id, finding_id),
        args.prompt_version,
        args.benchmark,
        args.run_id,
        finding_id,
        blocks[finding_id],
    )
    path = init_poc_unit_run(args.prompt_version, args.benchmark, args.run_id, finding_id, reset=args.reset)
    prompt_text = render_poc_prompt(args.prompt_version, args.benchmark, args.run_id, finding_id, config)
    completed = sum(
        1
        for fid in blocks
        if poc_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
    )
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "finding_id": finding_id,
        "stage_path": str(path),
        "json_path": str(poc_unit_json_path(args.prompt_version, args.benchmark, args.run_id, finding_id)),
        "single_finding_input_path": str(input_path),
        "source_candidates_path": str(submission_candidates_path(args.prompt_version, args.benchmark, args.run_id)),
        "input_scope": "single_post_r4_submission_candidate",
        "input_findings": 1,
        "total_findings": len(blocks),
        "completed_findings": completed,
        "remaining_findings": len(blocks) - completed,
        "worker_type": "three-shot-r5-poc-generation-finding",
        **worker_payload(config, "r5", POC_WORKER_MODEL, POC_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r5.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_poc_review(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    blocks = submission_candidate_blocks(args.prompt_version, args.benchmark, args.run_id)
    requested_finding = args.finding_id or next_poc_review_finding(args.prompt_version, args.benchmark, args.run_id)
    if requested_finding is None:
        completed_r5 = sum(
            1
            for fid in blocks
            if poc_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
        )
        completed_r6 = sum(
            1
            for fid in blocks
            if poc_review_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
        )
        phase = "complete" if completed_r6 == len(blocks) else "blocked"
        summary = (
            "All R6 per-finding PoC review units are complete."
            if phase == "complete"
            else "No reviewable R6 unit is available because some R5 units are incomplete."
        )
        payload = {
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "worker_type": "none",
            "phase": phase,
            "input_scope": "per_finding_post_r4_submission_candidates",
            "input_findings": len(blocks),
            "completed_r5_findings": completed_r5,
            "completed_r6_findings": completed_r6,
            "source_candidates_path": str(submission_candidates_path(args.prompt_version, args.benchmark, args.run_id)),
            "summary": summary,
        }
        print(json.dumps(payload, indent=2, sort_keys=True))
        return

    finding_id = validate_finding_id(requested_finding, blocks)
    if not poc_unit_complete(args.prompt_version, args.benchmark, args.run_id, finding_id):
        raise SystemExit(f"R5 PoC unit for {finding_id} is incomplete; cannot prepare R6.")
    path = init_poc_review_unit(args.prompt_version, args.benchmark, args.run_id, finding_id, reset=args.reset)
    prompt_text = render_poc_review_prompt(args.prompt_version, args.benchmark, args.run_id, finding_id, config)
    completed = sum(
        1
        for fid in blocks
        if poc_review_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
    )
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "finding_id": finding_id,
        "stage_path": str(path),
        "json_path": str(poc_review_unit_json_path(args.prompt_version, args.benchmark, args.run_id, finding_id)),
        "source_candidates_path": str(submission_candidates_path(args.prompt_version, args.benchmark, args.run_id)),
        "input_scope": "single_post_r4_submission_candidate",
        "input_findings": 1,
        "total_findings": len(blocks),
        "completed_findings": completed,
        "remaining_findings": len(blocks) - completed,
        "worker_type": "three-shot-r6-poc-verification-finding",
        **worker_payload(config, "r6", POC_WORKER_MODEL, POC_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r6.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_report(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    blocks = submission_candidate_blocks(args.prompt_version, args.benchmark, args.run_id)
    reportable = [
        fid
        for fid in blocks
        if reportable_after_poc_review(args.prompt_version, args.benchmark, args.run_id, fid)
    ]
    requested_finding = args.finding_id or next_report_finding(args.prompt_version, args.benchmark, args.run_id)
    if requested_finding is None:
        completed_r6 = sum(
            1
            for fid in blocks
            if poc_review_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
        )
        completed = sum(
            1
            for fid in reportable
            if report_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
        )
        phase = "complete" if completed_r6 == len(blocks) and completed == len(reportable) else "blocked"
        summary = (
            "All R7 per-finding C4 report units are complete."
            if phase == "complete"
            else "No reportable R7 unit is available because some R6 units are incomplete or not verified."
        )
        payload = {
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "worker_type": "none",
            "phase": phase,
            "input_scope": "single_verified_poc_finding",
            "total_submission_candidates": len(blocks),
            "reportable_findings": len(reportable),
            "completed_r6_findings": completed_r6,
            "completed_findings": completed,
            "source_candidates_path": str(submission_candidates_path(args.prompt_version, args.benchmark, args.run_id)),
            "summary": summary,
        }
        print(json.dumps(payload, indent=2, sort_keys=True))
        return

    finding_id = validate_finding_id(requested_finding, blocks)
    if not reportable_after_poc_review(args.prompt_version, args.benchmark, args.run_id, finding_id):
        raise SystemExit(f"Finding {finding_id} is not reportable until R6 marks its PoC Verified or Fixed And Verified.")
    input_path = write_round7_input(
        round7_input_path(args.prompt_version, args.benchmark, args.run_id, finding_id),
        args.prompt_version,
        args.benchmark,
        args.run_id,
        finding_id,
        blocks[finding_id],
    )
    path = init_finding_report_unit(args.prompt_version, args.benchmark, args.run_id, finding_id, reset=args.reset)
    prompt_text = render_finding_report_prompt(args.prompt_version, args.benchmark, args.run_id, finding_id, config)
    completed = sum(
        1
        for fid in reportable
        if report_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
    )
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "finding_id": finding_id,
        "stage_path": str(path),
        "json_path": str(finding_report_unit_json_path(args.prompt_version, args.benchmark, args.run_id, finding_id)),
        "single_finding_input_path": str(input_path),
        "source_candidates_path": str(submission_candidates_path(args.prompt_version, args.benchmark, args.run_id)),
        "input_scope": "single_verified_poc_finding",
        "input_findings": 1,
        "total_reportable_findings": len(reportable),
        "completed_findings": completed,
        "remaining_findings": len(reportable) - completed,
        "worker_type": "three-shot-r7-c4-report-finding",
        **worker_payload(config, "r7", REPORT_WORKER_MODEL, REPORT_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r7.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_report_review(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    blocks = submission_candidate_blocks(args.prompt_version, args.benchmark, args.run_id)
    expected_reports = [
        fid
        for fid in blocks
        if reportable_after_poc_review(args.prompt_version, args.benchmark, args.run_id, fid)
    ]
    reviewable = [
        fid
        for fid in expected_reports
        if report_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
    ]
    requested_finding = args.finding_id or next_report_review_finding(args.prompt_version, args.benchmark, args.run_id)
    if requested_finding is None:
        completed_r6 = sum(
            1
            for fid in blocks
            if poc_review_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
        )
        completed_r7 = sum(
            1
            for fid in expected_reports
            if report_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
        )
        completed = sum(
            1
            for fid in expected_reports
            if report_review_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
        )
        phase = (
            "complete"
            if completed_r6 == len(blocks) and completed_r7 == len(expected_reports) and completed == len(expected_reports)
            else "blocked"
        )
        summary = (
            "All R8 per-finding C4 report review units are complete."
            if phase == "complete"
            else "No reviewable R8 unit is available because some R7 report units are incomplete."
        )
        payload = {
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "worker_type": "none",
            "phase": phase,
            "input_scope": "single_c4_report",
            "reportable_findings": len(expected_reports),
            "reviewable_findings": len(reviewable),
            "completed_r6_findings": completed_r6,
            "completed_r7_findings": completed_r7,
            "completed_findings": completed,
            "source_candidates_path": str(submission_candidates_path(args.prompt_version, args.benchmark, args.run_id)),
            "summary": summary,
        }
        print(json.dumps(payload, indent=2, sort_keys=True))
        return

    finding_id = validate_finding_id(requested_finding, blocks)
    if not reportable_after_poc_review(args.prompt_version, args.benchmark, args.run_id, finding_id):
        raise SystemExit(f"Finding {finding_id} is not reportable until R6 marks its PoC Verified or Fixed And Verified.")
    if not report_unit_complete(args.prompt_version, args.benchmark, args.run_id, finding_id):
        raise SystemExit(f"R7 finding report for {finding_id} is incomplete; cannot prepare R8.")
    input_path = write_round8_input(
        round8_input_path(args.prompt_version, args.benchmark, args.run_id, finding_id),
        args.prompt_version,
        args.benchmark,
        args.run_id,
        finding_id,
        blocks[finding_id],
    )
    path = init_finding_report_review_unit(args.prompt_version, args.benchmark, args.run_id, finding_id, reset=args.reset)
    prompt_text = render_finding_report_review_prompt(args.prompt_version, args.benchmark, args.run_id, finding_id, config)
    completed = sum(
        1
        for fid in reviewable
        if report_review_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
    )
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "finding_id": finding_id,
        "stage_path": str(path),
        "json_path": str(finding_report_review_unit_json_path(args.prompt_version, args.benchmark, args.run_id, finding_id)),
        "single_finding_input_path": str(input_path),
        "source_candidates_path": str(submission_candidates_path(args.prompt_version, args.benchmark, args.run_id)),
        "input_scope": "single_c4_report",
        "input_findings": 1,
        "total_reportable_findings": len(reviewable),
        "completed_findings": completed,
        "remaining_findings": len(reviewable) - completed,
        "worker_type": "three-shot-r8-c4-report-review-finding",
        **worker_payload(config, "r8", REPORT_WORKER_MODEL, REPORT_WORKER_REASONING),
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r8.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def row_from_poc_unit(path: Path, finding_id: str) -> dict[str, str]:
    rows = parse_poc_rows(path)
    if finding_id not in rows:
        raise SystemExit(f"Missing result row for {finding_id} in {path}")
    return rows[finding_id]


def json_result_from_unit(path: Path, finding_id: str) -> dict[str, object]:
    if not path.exists():
        raise SystemExit(f"Missing JSON unit for {finding_id}: {path}")
    try:
        payload = json.loads(path.read_text())
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Invalid JSON unit {path}: {exc}") from None
    for row in payload.get("results", []):
        if isinstance(row, dict) and row.get("finding_id") == finding_id:
            return row
    raise SystemExit(f"Missing JSON result for {finding_id} in {path}")


def assemble_poc_units(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    rows: list[dict[str, str]] = []
    json_rows: list[dict[str, object]] = []
    missing: list[str] = []

    for finding_id in blocks:
        if not poc_unit_complete(prompt_version, benchmark, run_id, finding_id):
            missing.append(finding_id)
            continue
        rows.append(row_from_poc_unit(poc_unit_path(prompt_version, benchmark, run_id, finding_id), finding_id))
        json_rows.append(json_result_from_unit(poc_unit_json_path(prompt_version, benchmark, run_id, finding_id), finding_id))

    if missing:
        raise SystemExit("Cannot assemble R5; incomplete PoC units: " + ", ".join(missing[:10]))

    out_path = init_poc_run(prompt_version, benchmark, run_id, reset=True)
    lines = out_path.read_text().splitlines()
    lines = ["Status: Complete" if line == "Status: In progress" else line for line in lines]
    rendered_rows = [
        "| {fid} | {title} | {status} | {test_path} | {test_command} | {result} | {notes} |".format(
            fid=clean(row["finding_id"]),
            title=clean(row["title"]),
            status=clean(row["status"]),
            test_path=clean(row["test_path"]),
            test_command=clean(row["test_command"]),
            result=clean(row["result"]),
            notes=clean(row["notes"]),
        )
        for row in rows
    ]
    out_path.write_text("\n".join(
        line if line != SCREEN_ANCHOR else "\n".join(rendered_rows + [SCREEN_ANCHOR])
        for line in lines
    ) + "\n")
    json_path = poc_run_json_path(prompt_version, benchmark, run_id)
    json_path.write_text(
        json.dumps(
            {
                "benchmark": benchmark,
                "source_candidates": str(submission_candidates_path(prompt_version, benchmark, run_id)),
                "results": json_rows,
            },
            indent=2,
            sort_keys=True,
        )
        + "\n"
    )
    return {
        "benchmark": benchmark,
        "prompt_version": prompt_version,
        "run_id": run_id,
        "stage_path": str(out_path),
        "json_path": str(json_path),
        "assembled_findings": len(rows),
    }


def assemble_poc_review_units(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    rows: list[dict[str, str]] = []
    json_rows: list[dict[str, object]] = []
    missing: list[str] = []

    for finding_id in blocks:
        if not poc_review_unit_complete(prompt_version, benchmark, run_id, finding_id):
            missing.append(finding_id)
            continue
        rows.append(row_from_poc_unit(poc_review_unit_path(prompt_version, benchmark, run_id, finding_id), finding_id))
        json_rows.append(
            json_result_from_unit(poc_review_unit_json_path(prompt_version, benchmark, run_id, finding_id), finding_id)
        )

    if missing:
        raise SystemExit("Cannot assemble R6; incomplete PoC review units: " + ", ".join(missing[:10]))

    out_path = init_poc_review(prompt_version, benchmark, run_id, reset=True)
    lines = out_path.read_text().splitlines()
    lines = ["Status: Complete" if line == "Status: In progress" else line for line in lines]
    rendered_rows = [
        "| {fid} | {title} | {status} | {test_path} | {test_command} | {result} | {notes} |".format(
            fid=clean(row["finding_id"]),
            title=clean(row["title"]),
            status=clean(row["status"]),
            test_path=clean(row["test_path"]),
            test_command=clean(row["test_command"]),
            result=clean(row["result"]),
            notes=clean(row["notes"]),
        )
        for row in rows
    ]
    out_path.write_text("\n".join(
        line if line != SCREEN_ANCHOR else "\n".join(rendered_rows + [SCREEN_ANCHOR])
        for line in lines
    ) + "\n")
    json_path = poc_review_json_path(prompt_version, benchmark, run_id)
    json_path.write_text(
        json.dumps(
            {
                "benchmark": benchmark,
                "source_candidates": str(submission_candidates_path(prompt_version, benchmark, run_id)),
                "source_poc_run": str(poc_run_path(prompt_version, benchmark, run_id)),
                "results": json_rows,
            },
            indent=2,
            sort_keys=True,
        )
        + "\n"
    )
    return {
        "benchmark": benchmark,
        "prompt_version": prompt_version,
        "run_id": run_id,
        "stage_path": str(out_path),
        "json_path": str(json_path),
        "assembled_findings": len(rows),
    }


def cmd_assemble(args: argparse.Namespace) -> None:
    resolve_run_args(args)
    print(json.dumps(assemble_run(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_apply_dedup(args: argparse.Namespace) -> None:
    resolve_run_args(args)
    print(json.dumps(apply_dedup(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_apply_v12_sweep(args: argparse.Namespace) -> None:
    resolve_run_args(args)
    print(json.dumps(apply_v12_sweep(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_assemble_poc(args: argparse.Namespace) -> None:
    resolve_run_args(args)
    print(json.dumps(assemble_poc_units(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_assemble_poc_review(args: argparse.Namespace) -> None:
    resolve_run_args(args)
    print(json.dumps(assemble_poc_review_units(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


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
            "APPROVED_FINDINGS_KEY": truth_key_path(args.benchmark),
        },
    )
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "run_path": str(run_path),
        "result_path": str(result_path(args.prompt_version, args.benchmark, args.run_id)),
        "worker_type": "three-shot-scoring",
        **worker_payload(config, "scoring", SCORING_WORKER_MODEL, SCORING_WORKER_REASONING),
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
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

    prepare_v12_sweep = subparsers.add_parser("prepare-v12-sweep", help="Initialize optional round 4a V12 overlap sweep and emit the worker prompt.")
    add_run_args(prepare_v12_sweep)
    prepare_v12_sweep.add_argument("--include-prompt", action="store_true")
    prepare_v12_sweep.add_argument("--write-prompt")
    prepare_v12_sweep.add_argument("--reset", action="store_true", help="Reset the round 4a V12 sweep before emitting the prompt.")
    prepare_v12_sweep.set_defaults(func=cmd_prepare_v12_sweep)

    prepare_poc = subparsers.add_parser("prepare-poc", help="Initialize round 5 PoC generation and emit the worker prompt.")
    add_run_args(prepare_poc)
    prepare_poc.add_argument("--finding-id", help="Prepare this exact finding. Defaults to the next unfinished R5 unit.")
    prepare_poc.add_argument("--include-prompt", action="store_true")
    prepare_poc.add_argument("--write-prompt")
    prepare_poc.add_argument("--reset", action="store_true", help="Reset this finding's round 5 PoC unit before emitting the prompt.")
    prepare_poc.set_defaults(func=cmd_prepare_poc)

    prepare_poc_review = subparsers.add_parser("prepare-poc-review", help="Initialize round 6 PoC verification and emit the worker prompt.")
    add_run_args(prepare_poc_review)
    prepare_poc_review.add_argument("--finding-id", help="Prepare this exact finding. Defaults to the next R5-complete/R6-unfinished unit.")
    prepare_poc_review.add_argument("--include-prompt", action="store_true")
    prepare_poc_review.add_argument("--write-prompt")
    prepare_poc_review.add_argument("--reset", action="store_true", help="Reset this finding's round 6 PoC verification unit before emitting the prompt.")
    prepare_poc_review.set_defaults(func=cmd_prepare_poc_review)

    assemble_poc = subparsers.add_parser("assemble-poc", help="Assemble completed per-finding round 5 PoC units.")
    add_run_args(assemble_poc)
    assemble_poc.set_defaults(func=cmd_assemble_poc)

    assemble_poc_review = subparsers.add_parser("assemble-poc-review", help="Assemble completed per-finding round 6 PoC review units.")
    add_run_args(assemble_poc_review)
    assemble_poc_review.set_defaults(func=cmd_assemble_poc_review)

    prepare_report = subparsers.add_parser("prepare-report", help="Initialize round 7 C4 report creation for one finding.")
    add_run_args(prepare_report)
    prepare_report.add_argument("--finding-id", help="Prepare this exact finding. Defaults to the next R6-verified/R7-unfinished unit.")
    prepare_report.add_argument("--include-prompt", action="store_true")
    prepare_report.add_argument("--write-prompt")
    prepare_report.add_argument("--reset", action="store_true", help="Reset this finding's round 7 report before emitting the prompt.")
    prepare_report.set_defaults(func=cmd_prepare_report)

    prepare_report_review = subparsers.add_parser("prepare-report-review", help="Initialize round 8 C4 report review for one finding.")
    add_run_args(prepare_report_review)
    prepare_report_review.add_argument("--finding-id", help="Prepare this exact finding. Defaults to the next R7-complete/R8-unfinished unit.")
    prepare_report_review.add_argument("--include-prompt", action="store_true")
    prepare_report_review.add_argument("--write-prompt")
    prepare_report_review.add_argument("--reset", action="store_true", help="Reset this finding's round 8 report review before emitting the prompt.")
    prepare_report_review.set_defaults(func=cmd_prepare_report_review)

    assemble = subparsers.add_parser("assemble", help="Assemble the final three-shot run from stages 1-3.")
    add_run_args(assemble)
    assemble.set_defaults(func=cmd_assemble)

    apply_dedup_cmd = subparsers.add_parser("apply-dedup", help="Apply round 4 canonicalization decisions to create submission candidates.")
    add_run_args(apply_dedup_cmd)
    apply_dedup_cmd.set_defaults(func=cmd_apply_dedup)

    apply_v12_sweep_cmd = subparsers.add_parser("apply-v12-sweep", help="Apply round 4a V12 decisions to filter submission candidates.")
    add_run_args(apply_v12_sweep_cmd)
    apply_v12_sweep_cmd.set_defaults(func=cmd_apply_v12_sweep)

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
