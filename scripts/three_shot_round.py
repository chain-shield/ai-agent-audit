#!/usr/bin/env python3
"""Prepare, assemble, and score three-shot validation experiments.

Three-shot validation splits the full-report pass into:
1. scope / known-issue screening
2. profile-specific screening: unsupported-token, bounty exploitability, or private-client assumption triage
3. full validation on the surviving findings, or severity/eligibility classification for bounty/private-client profiles
3a. optional Immunefi feasibility-limitations gate on reportable candidates
4. optional canonicalization cleanup on the surviving reportable candidates
4a. optional second V12 overlap sweep on post-canonicalization candidates for competition runs
5. optional PoC generation and verification on submission candidates
6. optional submission report creation, review, and judge simulation

This is intended to cheaply strip obvious false positives before the expensive
final gate analysis while preserving recall on approved roots.
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import re
import shlex
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
TRUTH_ROOT = Path(
    os.environ.get("AI_AGENT_AUDIT_VALIDATION_TRUTH_ROOT", "~/.ai-agent-audit-validation-truth")
).expanduser()
APPEND_ANCHOR = "<!-- APPEND FINDING BLOCKS ABOVE THIS LINE -->"
FINDING_ID_PATTERN = r"(?:C|H|M|L|QA|I)-\d+"
REPORT_FINDING_RE = re.compile(rf"^## \[({FINDING_ID_PATTERN})\]\. (.+)$")
RAW_FINDING_RE = re.compile(rf"^### ({FINDING_ID_PATTERN}) / `([^`]+)`")
ACTIVE_CONFIG: dict[str, object] = {}


@dataclass(frozen=True)
class Finding:
    fid: str
    title: str
    report_id: str
    start_line: int
    end_line: int
    block: str


def is_finding_id(value: str) -> bool:
    return re.fullmatch(FINDING_ID_PATTERN, value) is not None


THREE_SHOT_ROOT = REPO_ROOT / "validation-three-shot"
THREE_SHOT_PROMPT_ROOT = THREE_SHOT_ROOT / "prompts"
V12_CHECKLIST_PATH = THREE_SHOT_ROOT / "v12-checklist.md"
BOUNTY_CRITERIA_PATH = THREE_SHOT_ROOT / "code4rena-bounty-criteria.md"
DEFAULT_CONFIG_PATH = THREE_SHOT_ROOT / "config.yaml"
STAGE_WORKER_MODEL = "gpt-5.5"
STAGE_WORKER_REASONING = "xhigh"
DEDUP_WORKER_MODEL = "gpt-5.4"
DEDUP_WORKER_REASONING = "high"
V12_SWEEP_WORKER_MODEL = "gpt-5.4"
V12_SWEEP_WORKER_REASONING = "high"
FEASIBILITY_WORKER_MODEL = "gpt-5.4"
FEASIBILITY_WORKER_REASONING = "high"
POC_WORKER_MODEL = "gpt-5.5"
POC_WORKER_REASONING = "xhigh"
REPORT_WORKER_MODEL = "gpt-5.5"
REPORT_WORKER_REASONING = "xhigh"
JUDGE_WORKER_MODEL = "gpt-5.5"
JUDGE_WORKER_REASONING = "xhigh"
SCORING_WORKER_MODEL = "gpt-5.4"
SCORING_WORKER_REASONING = "xhigh"
DEFAULT_WORKER_LAUNCHER = os.environ.get("AI_AGENT_AUDIT_WORKER_LAUNCHER", "codex")
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
    audit_root = config_path_value(["paths", "audit_root"])
    if audit_root:
        return resolve_configured_path(audit_root) / "report" / "audit-report.md"
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
    raise SystemExit(
        "Scoring requires paths.truth_file in the validation config; no public default truth file is bundled."
    )


def truth_key_path(benchmark: str) -> Path:
    configured = config_path_value(["paths", "truth_file"])
    if configured:
        return resolve_configured_path(configured)
    raise SystemExit(
        "Scoring requires paths.truth_file in the validation config; no public default truth file is bundled."
    )


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


def optional_override(value: str | None) -> str:
    normalized = (value or "").strip()
    if normalized.lower() in {"", "-", "—", "n/a", "na", "none", "null"}:
        return ""
    return normalized


def scope_screen_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "scope-screens" / prompt_version / f"{benchmark}-{run_id}.md"


def token_screen_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "token-screens" / prompt_version / f"{benchmark}-{run_id}.md"


def stage3_run_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "stage3-runs" / prompt_version / f"{benchmark}-{run_id}.md"


def feasibility_screen_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "feasibility-screens" / prompt_version / f"{benchmark}-{run_id}.md"


def feasibility_run_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "feasibility-runs" / prompt_version / f"{benchmark}-{run_id}.md"


def current_validated_run_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    if feasibility_gate_enabled():
        path = feasibility_run_path(prompt_version, benchmark, run_id)
        if not path.exists():
            raise SystemExit(
                "R3a feasibility gate is enabled but has not been applied. "
                "Run prepare-feasibility and apply-feasibility before scoring, dedup, or downstream rounds."
            )
        return path
    return final_run_path(prompt_version, benchmark, run_id)


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
    heading = re.search(rf"^###\s+{FINDING_ID_PATTERN}\s*/\s*`[^`]+`\s*$", block, re.MULTILINE)
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


def judge_simulation_unit_dir(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "judge-simulations" / prompt_version / f"{benchmark}-{run_id}"


def judge_simulation_unit_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return judge_simulation_unit_dir(prompt_version, benchmark, run_id) / (
        f"{finding_id}-{finding_report_slug(prompt_version, benchmark, run_id, finding_id)}.md"
    )


def judge_simulation_unit_json_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return judge_simulation_unit_dir(prompt_version, benchmark, run_id) / f"{finding_id}.json"


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


def round3a_input_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "inputs" / "r3a" / prompt_version / f"{benchmark}-{run_id}.md"


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


def round9_input_path(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> Path:
    return THREE_SHOT_ROOT / "inputs" / "r9" / prompt_version / f"{benchmark}-{run_id}" / f"{finding_id}.md"


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


def validation_profile(config: dict[str, object] | None = None) -> str:
    source = config if config is not None else ACTIVE_CONFIG
    return nested_get(source, ["validation_profile"], "default")


def is_bounty_profile(config: dict[str, object] | None = None) -> bool:
    return validation_profile(config) in {"code4rena-bounty", "immunefi-bounty"}


def is_private_client_profile(config: dict[str, object] | None = None) -> bool:
    return validation_profile(config) == "private-client"


def is_immunefi_bounty_profile(config: dict[str, object] | None = None) -> bool:
    return validation_profile(config) == "immunefi-bounty"


def round_enabled(config: dict[str, object], round_name: str, default: bool = True) -> bool:
    value = nested_value(config, ["rounds", round_name])
    if value is None:
        return default
    if isinstance(value, bool):
        return value
    return str(value).strip().lower() in {"1", "true", "yes", "on"}


def feasibility_gate_enabled(config: dict[str, object] | None = None) -> bool:
    source = config if config is not None else ACTIVE_CONFIG
    return is_immunefi_bounty_profile(source) and round_enabled(source, "r3a_feasibility_gate", False)


def reportable_severities(config: dict[str, object] | None = None) -> set[str]:
    if is_private_client_profile(config):
        return {"Critical", "High", "Medium", "Low", "QA", "Informational", "Info"}
    if is_immunefi_bounty_profile(config):
        return {"Critical", "High", "Medium", "Low"}
    if is_bounty_profile(config):
        return {"Critical", "High"}
    return {"High", "Medium"}


def profile_worker_type(stage: str, config: dict[str, object]) -> str:
    profile = validation_profile(config)
    if profile == "private-client":
        suffix = {
            "r7": "report-finding",
            "r8": "report-review-finding",
            "r9": "judge-simulation-finding",
        }.get(stage, "worker")
        return f"three-shot-{stage}-private-client-{suffix}"
    if stage == "r7":
        if is_immunefi_bounty_profile(config):
            return "three-shot-r7-immunefi-report-finding"
        if is_bounty_profile(config):
            return "three-shot-r7-c4-bounty-report-finding"
        return "three-shot-r7-c4-report-finding"
    if stage == "r8":
        if is_immunefi_bounty_profile(config):
            return "three-shot-r8-immunefi-report-review-finding"
        if is_bounty_profile(config):
            return "three-shot-r8-c4-bounty-report-review-finding"
        return "three-shot-r8-c4-report-review-finding"
    if stage == "r9":
        if is_immunefi_bounty_profile(config):
            return "three-shot-r9-immunefi-judge-simulation-finding"
        if is_bounty_profile(config):
            return "three-shot-r9-c4-bounty-judge-simulation-finding"
        return "three-shot-r9-c4-judge-simulation-finding"
    return f"three-shot-{stage}-{profile}"


def stage2_screen_label(config: dict[str, object] | None = None) -> str:
    if is_bounty_profile(config):
        return "bounty exploitability"
    if is_private_client_profile(config):
        return "assumption / client-triage"
    return "unsupported token"


def stage2_screen_title(config: dict[str, object] | None = None) -> str:
    if is_bounty_profile(config):
        return "Three-Shot Stage 2 Bounty Exploitability Screen"
    if is_private_client_profile(config):
        return "Three-Shot Stage 2 Assumption / Client-Triage Screen"
    return "Three-Shot Stage 2 Unsupported-Token Screen"


def stage2_worker_type(config: dict[str, object]) -> str:
    if is_bounty_profile(config):
        return "three-shot-r2-bounty-exploitability"
    if is_private_client_profile(config):
        return "three-shot-r2-private-client-assumption-screen"
    return "three-shot-r2-token"


def stage3_worker_type(config: dict[str, object]) -> str:
    if is_private_client_profile(config):
        return "three-shot-r3-private-client-validation"
    if is_immunefi_bounty_profile(config):
        return "three-shot-r3-immunefi-severity-validation"
    if is_bounty_profile(config):
        return "three-shot-r3-bounty-critical-high-validation"
    return "three-shot-r3-final-validation"


def report_review_input_scope(config: dict[str, object]) -> str:
    if is_private_client_profile(config):
        return "single_private_client_report"
    if is_immunefi_bounty_profile(config):
        return "single_immunefi_bounty_report"
    if is_bounty_profile(config):
        return "single_c4_bounty_report"
    return "single_c4_report"


def normalize_severity(value: str) -> str:
    normalized = value.strip()
    aliases = {
        "Info": "Informational",
        "Information": "Informational",
        "Low / QA": "QA",
        "Low/QA": "QA",
    }
    return aliases.get(normalized, normalized)


def severity_requires_verified_poc(severity: str, config: dict[str, object] | None = None) -> bool:
    if not is_private_client_profile(config):
        return True
    return normalize_severity(severity) in {"Critical", "High", "Medium"}


def severity_allows_reportable_without_poc(severity: str, config: dict[str, object] | None = None) -> bool:
    return is_private_client_profile(config) and normalize_severity(severity) in {
        "Low",
        "QA",
        "Informational",
    }


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
    env_launcher = os.environ.get("AI_AGENT_AUDIT_WORKER_LAUNCHER")
    if env_launcher:
        return env_launcher
    return nested_get(
        config,
        ["workers", worker, "launcher"],
        nested_get(config, ["workers", "default", "launcher"], DEFAULT_WORKER_LAUNCHER),
    )


def is_codex_launcher(launcher: str) -> bool:
    try:
        first_arg = shlex.split(launcher)[0]
    except (IndexError, ValueError):
        first_arg = launcher
    return "codex" in Path(first_arg).name


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
        "requires_filesystem_agent_worker": True,
        "requires_minimal_codex_worker": is_codex_launcher(launcher),
        "worker_launch_note": (
            "Launch this unit with worker_spawn_command. Validation workers must be able to read "
            "and edit local files and run tests. The default Codex launcher provides that agentic "
            "filesystem/tool access; a raw OpenAI API key alone is not enough for these worker prompts."
        ),
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
        if is_bounty_profile():
            paths.append(BOUNTY_CRITERIA_PATH)
        else:
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


def context_docs_with_suffix(paths: list[Path], suffix: str) -> list[Path]:
    return [path for path in paths if path.name.endswith(suffix)]


def prompt_file_contents(paths: list[Path], label: str, max_chars: int = 24000) -> str:
    if not paths:
        return f"_No {label} file configured._"
    blocks: list[str] = []
    for path in paths:
        if not path.exists():
            blocks.append(f"## `{path}`\n\n_Missing file._")
            continue
        text = path.read_text(errors="replace")
        if len(text) > max_chars:
            text = text[:max_chars].rstrip() + "\n\n[truncated]"
        blocks.append(f"## `{path}`\n\n{text}")
    return "\n\n".join(blocks)


def load_dotenv_values(path: Path) -> dict[str, str]:
    if not path.exists():
        return {}
    values: dict[str, str] = {}
    for line in path.read_text(errors="replace").splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        if stripped.startswith("export "):
            stripped = stripped[len("export ") :].lstrip()
        if "=" not in stripped:
            continue
        key, value = stripped.split("=", 1)
        key = key.strip()
        if not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", key):
            continue
        value = value.strip()
        if (
            len(value) >= 2
            and value[0] == value[-1]
            and value[0] in {"'", '"'}
        ):
            value = value[1:-1]
        if value:
            values[key] = value
    return values


def env_var_available(env_var: str, dotenv_values: dict[str, str]) -> bool:
    return bool(os.environ.get(env_var) or dotenv_values.get(env_var))


def immunefi_poc_runtime_json_paths(source_root: Path) -> list[Path]:
    paths: list[Path] = []
    for doc in context_docs_with_suffix(benchmark_scope_docs(source_root), "-immunefi-poc-runtime.md"):
        candidate = doc.with_suffix(".json")
        if candidate.exists():
            paths.append(candidate)
    return paths


def immunefi_poc_runtime_payload(benchmark: str) -> dict[str, object]:
    source_root = benchmark_source_root(benchmark)
    runtime_paths = immunefi_poc_runtime_json_paths(source_root)
    repo_dotenv = REPO_ROOT / ".env"
    dotenv_values = load_dotenv_values(repo_dotenv)
    payloads: list[dict[str, object]] = []
    available_env: set[str] = set()
    missing_env: set[str] = set()
    in_scope_addresses: list[dict[str, object]] = []

    for path in runtime_paths:
        try:
            payload = json.loads(path.read_text())
        except json.JSONDecodeError:
            continue
        if not isinstance(payload, dict):
            continue
        payloads.append(payload)
        for network in payload.get("networks", []):
            if not isinstance(network, dict):
                continue
            env_var = str(network.get("rpc_env_var", ""))
            if not env_var:
                continue
            if env_var_available(env_var, dotenv_values):
                available_env.add(env_var)
            else:
                missing_env.add(env_var)
        for asset in payload.get("assets", []):
            if not isinstance(asset, dict):
                continue
            in_scope_addresses.append({
                "address": asset.get("address"),
                "network": asset.get("network"),
                "rpc_env_var": asset.get("rpc_env_var"),
                "description": asset.get("description"),
                "explorer_url": asset.get("explorer_url"),
            })

    return {
        "runtime_json_paths": [str(path) for path in runtime_paths],
        "prefer_fork": any(bool(payload.get("prefer_fork")) for payload in payloads),
        "allow_fork": any(bool(payload.get("allow_fork")) for payload in payloads),
        "available_rpc_env": sorted(available_env),
        "missing_rpc_env": sorted(missing_env - available_env),
        "in_scope_addresses": in_scope_addresses,
        "env_sources_checked": [
            "process environment",
            str(repo_dotenv) if repo_dotenv.exists() else f"{repo_dotenv} (missing)",
        ],
    }


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


def prompt_template_path(template_name: str, config: dict[str, object] | None = None) -> Path:
    profile = validation_profile(config)
    candidates = [THREE_SHOT_PROMPT_ROOT / profile / template_name]
    if profile != "default":
        candidates.append(THREE_SHOT_PROMPT_ROOT / "default" / template_name)
    for candidate in candidates:
        if candidate.exists():
            return candidate
    raise SystemExit(
        "Missing three-shot prompt template: "
        + " or ".join(str(candidate) for candidate in candidates)
    )


def render_stage_prompt(template_name: str, replacements: dict[str, object], config: dict[str, object] | None = None) -> str:
    template_path = prompt_template_path(template_name, config)
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
        if not is_finding_id(fid):
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


def init_feasibility_screen(prompt_version: str, benchmark: str, run_id: str, reset: bool = False) -> Path:
    path = feasibility_screen_path(prompt_version, benchmark, run_id)
    if path.exists() and not reset:
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    content = f"""# {benchmark} Three-Shot Round 3a Immunefi Feasibility Screen

Status: In progress
Source assembled run: `{final_run_path(prompt_version, benchmark, run_id)}`
Output feasibility-adjusted run: `{feasibility_run_path(prompt_version, benchmark, run_id)}`

## Decisions

| Finding | Finding Title | Decision | Confidence | Feasibility Category | Revised Severity | Required Report Note | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- |
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
    content = f"""# {finding_id} Finding Report

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
    content = f"""# {finding_id} Finding Report Review

Status: In progress
Benchmark: `{benchmark}`
Source input: `{round8_input_path(prompt_version, benchmark, run_id, finding_id)}`
Report under review: `{finding_report_unit_path(prompt_version, benchmark, run_id, finding_id)}`
JSON summary: `{finding_report_review_unit_json_path(prompt_version, benchmark, run_id, finding_id)}`

<!-- R8 WORKER REPLACES THIS FILE WITH THE REVIEW RESULT -->
"""
    path.write_text(content)
    return path


def init_judge_simulation_unit(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    reset: bool = False,
) -> Path:
    path = judge_simulation_unit_path(prompt_version, benchmark, run_id, finding_id)
    if path.exists() and not reset:
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    content = f"""# {finding_id} Judge Simulation

Status: In progress
Benchmark: `{benchmark}`
Source input: `{round9_input_path(prompt_version, benchmark, run_id, finding_id)}`
Post-R8 report under judge simulation: `{finding_report_unit_path(prompt_version, benchmark, run_id, finding_id)}`
JSON summary: `{judge_simulation_unit_json_path(prompt_version, benchmark, run_id, finding_id)}`

<!-- R9 WORKER REPLACES THIS FILE WITH THE JUDGE SIMULATION -->
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
        if not is_finding_id(fid):
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
        if not is_finding_id(fid):
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


def parse_feasibility_screen(path: Path) -> dict[str, dict[str, str]]:
    if not path.exists():
        raise SystemExit(f"Missing required feasibility screen file: {path}")

    rows: dict[str, dict[str, str]] = {}
    header: list[str] | None = None
    for line in path.read_text().splitlines():
        if not line.startswith("| "):
            continue
        parts = [part.strip() for part in line.strip().strip("|").split("|")]
        if parts[0] == "Finding":
            header = [re.sub(r"[^a-z0-9]+", "_", part.lower()).strip("_") for part in parts]
            continue
        if parts[0] == "---":
            continue
        fid = parts[0].split(" / ", 1)[0].strip()
        if not is_finding_id(fid):
            continue
        expected_columns = len(header) if header else 8
        if len(parts) < expected_columns:
            raise SystemExit(
                f"Incomplete R3a feasibility row for {fid}: expected {expected_columns} columns, got {len(parts)}."
            )
        if header and len(parts) >= len(header):
            mapped = dict(zip(header, parts, strict=False))
            rows[fid] = {
                "finding_id": fid,
                "title": mapped.get("finding_title", ""),
                "decision": mapped.get("decision", ""),
                "confidence": mapped.get("confidence", ""),
                "feasibility_category": mapped.get("feasibility_category", ""),
                "revised_severity": mapped.get("revised_severity", ""),
                "required_report_note": mapped.get("required_report_note", ""),
                "reason": mapped.get("reason", ""),
            }
            continue
        rows[fid] = {
            "finding_id": fid,
            "title": parts[1],
            "decision": parts[2],
            "confidence": parts[3],
            "feasibility_category": parts[4],
            "revised_severity": parts[5],
            "required_report_note": parts[6],
            "reason": "|".join(parts[7:]).strip(),
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
        if not is_finding_id(fid):
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
        if not is_finding_id(fid):
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


def reportable_stage3_candidates(prompt_version: str, benchmark: str, run_id: str) -> list[tuple[Finding, str]]:
    run_path = final_run_path(prompt_version, benchmark, run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    blocks = raw_finding_blocks(run_path)
    severities = reportable_severities()
    candidates: list[tuple[Finding, str]] = []
    for finding in all_findings(benchmark):
        block = blocks.get(finding.fid)
        if not block:
            continue
        if block_field(block, "Decision") != "Valid":
            continue
        if normalize_severity(block_field(block, "Severity Assessment")) not in severities:
            continue
        candidates.append((finding, block))
    return candidates


def dedup_candidates(prompt_version: str, benchmark: str, run_id: str) -> list[tuple[Finding, str]]:
    run_path = current_validated_run_path(prompt_version, benchmark, run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    findings_by_id = {finding.fid: finding for finding in all_findings(benchmark)}
    blocks = raw_finding_blocks(run_path)
    severities = reportable_severities()
    candidates: list[tuple[Finding, str]] = []
    for finding in all_findings(benchmark):
        block = blocks.get(finding.fid)
        if not block:
            continue
        if block_field(block, "Decision") != "Valid":
            continue
        if normalize_severity(block_field(block, "Severity Assessment")) not in severities:
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
            f"Source validated run: `{current_validated_run_path(prompt_version, benchmark, run_id)}`",
            f"Candidate count: `{len(candidates)}`",
            "",
            "This file contains only findings currently marked `Valid` with reportable severity for the configured validation profile.",
            "",
            "## Candidates",
            "",
            "\n".join(sections) if sections else "_No reportable candidates to dedup._",
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


def write_round3a_input(path: Path, prompt_version: str, benchmark: str, run_id: str) -> Path:
    candidates = reportable_stage3_candidates(prompt_version, benchmark, run_id)
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
                "### R3 Validated Block",
                block.strip(),
                "",
            ]
        )
    content = "\n".join(
        [
            f"# {benchmark} Round 3a Immunefi Feasibility Input",
            "",
            f"Source assembled run: `{final_run_path(prompt_version, benchmark, run_id)}`",
            f"Candidate count: `{len(candidates)}`",
            "",
            "This file contains only R3 candidates currently marked `Valid` with reportable Immunefi severity.",
            "R3a must apply Immunefi feasibility-limitation standards without redoing R3 bug/severity analysis.",
            "",
            "## Candidates",
            "",
            "\n".join(sections) if sections else "_No reportable candidates for feasibility review._",
            "",
        ]
    )
    path.write_text(content)
    return path


def require_feasibility_rows(prompt_version: str, benchmark: str, run_id: str) -> dict[str, dict[str, str]]:
    rows = parse_feasibility_screen(feasibility_screen_path(prompt_version, benchmark, run_id))
    expected = {finding.fid for finding, _ in reportable_stage3_candidates(prompt_version, benchmark, run_id)}
    if expected and not rows:
        raise SystemExit("Feasibility screen has no decisions yet. Complete round 3a before applying it.")
    missing = sorted(expected - set(rows))
    if missing:
        raise SystemExit(
            "Feasibility screen is incomplete. Missing findings: " + ", ".join(missing[:10])
        )
    return rows


def set_block_field(block: str, label: str, value: str) -> str:
    prefix = f"- {label}: "
    lines = block.rstrip().splitlines()
    for idx, line in enumerate(lines):
        if line.startswith(prefix):
            lines[idx] = prefix + value
            return "\n".join(lines) + "\n"
    lines.append(prefix + value)
    return "\n".join(lines) + "\n"


def apply_feasibility_to_block(block: str, row: dict[str, str]) -> str:
    decision = row["decision"].strip().lower()
    category = row.get("feasibility_category", "") or "-"
    note = row.get("required_report_note", "") or "-"
    reason = row.get("reason", "") or "-"
    confidence = row.get("confidence", "") or block_field(block, "Confidence") or "Medium"
    revised_severity = optional_override(row.get("revised_severity"))

    updated = block
    if decision == "keep":
        pass
    elif decision in {"reclassify as medium griefing", "downgrade to medium griefing"}:
        updated = set_block_field(updated, "Decision", "Valid")
        updated = set_block_field(updated, "Severity Assessment", "Medium")
        updated = set_block_field(updated, "Root Cause Family", "`immunefi-griefing-feasibility`")
    elif decision == "needs review":
        updated = set_block_field(updated, "Decision", "Needs Review")
        updated = set_block_field(updated, "Severity Assessment", revised_severity or "Needs Review")
    elif decision == "exclude":
        updated = set_block_field(updated, "Decision", "Invalid")
        updated = set_block_field(updated, "Severity Assessment", revised_severity or "Low / Invalid")
        updated = set_block_field(updated, "Root Cause Family", "`immunefi-feasibility-exclusion`")
    else:
        raise SystemExit(f"Invalid R3a feasibility decision: {row['decision']}")

    if revised_severity and decision == "keep":
        updated = set_block_field(updated, "Severity Assessment", revised_severity)
    updated = set_block_field(updated, "Confidence", confidence)
    updated = set_block_field(updated, "Feasibility Assessment", category)
    updated = set_block_field(updated, "Feasibility Report Note", note)
    updated = set_block_field(updated, "Feasibility Reason", reason)
    return updated.strip()


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
    if not is_finding_id(finding_id):
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


def submission_candidate_severity(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> str:
    block = submission_candidate_blocks(prompt_version, benchmark, run_id).get(finding_id, "")
    return normalize_severity(block_field(block, "Severity Assessment"))


def reportable_after_poc_review(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> bool:
    status = poc_review_status(prompt_version, benchmark, run_id, finding_id)
    if status in {
        "Verified PoC",
        "Fixed And Verified PoC",
    }:
        return True

    if status in {"Reportable Without PoC", "Evidence Only"}:
        severity = submission_candidate_severity(prompt_version, benchmark, run_id, finding_id)
        return severity_allows_reportable_without_poc(severity)

    return False


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


def judge_simulation_unit_complete(prompt_version: str, benchmark: str, run_id: str, finding_id: str) -> bool:
    path = judge_simulation_unit_path(prompt_version, benchmark, run_id, finding_id)
    if not path.exists():
        return False
    text = path.read_text()
    if "Status: In progress" in text or "R9 WORKER REPLACES THIS FILE" in text:
        return False
    return json_has_finding(
        judge_simulation_unit_json_path(prompt_version, benchmark, run_id, finding_id),
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


def next_judge_simulation_finding(prompt_version: str, benchmark: str, run_id: str) -> str | None:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    for finding_id in blocks:
        if not reportable_after_poc_review(prompt_version, benchmark, run_id, finding_id):
            continue
        if not report_review_unit_complete(prompt_version, benchmark, run_id, finding_id):
            continue
        if not judge_simulation_unit_complete(prompt_version, benchmark, run_id, finding_id):
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
    if not reportable_after_poc_review(prompt_version, benchmark, run_id, finding_id):
        raise SystemExit(f"Finding {finding_id} is not reportable after R6 status: {row.get('status')}")
    severity = submission_candidate_severity(prompt_version, benchmark, run_id, finding_id)
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
            f"PoC policy: `{'verified-poc-required' if severity_requires_verified_poc(severity) else 'reportable-without-poc-allowed'}`",
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


def write_round9_input(
    path: Path,
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    block: str,
) -> Path:
    if not report_review_unit_complete(prompt_version, benchmark, run_id, finding_id):
        raise SystemExit(f"R8 finding report review for {finding_id} is incomplete.")
    path.parent.mkdir(parents=True, exist_ok=True)
    content = "\n".join(
        [
            f"# {benchmark} Round 9 Single-Finding Judge Simulation Input: {finding_id}",
            "",
            f"Source submission candidates: `{submission_candidates_path(prompt_version, benchmark, run_id)}`",
            f"Post-R8 finalized report to judge: `{finding_report_unit_path(prompt_version, benchmark, run_id, finding_id)}`",
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
    context_docs = benchmark_scope_docs(source_root)
    scope_paths = sorted({*source_root.glob("*-scope.md"), *context_docs_with_suffix(context_docs, "-scope.md")})
    docs_paths = sorted({*source_root.glob("*-docs.md"), *context_docs_with_suffix(context_docs, "-docs.md")})
    immunefi_rules = context_docs_with_suffix(context_docs, "-immunefi-bounty-rules.md")
    immunefi_rubrics = context_docs_with_suffix(context_docs, "-immunefi-severity-rubric.md")
    immunefi_poc_runtime = context_docs_with_suffix(context_docs, "-immunefi-poc-runtime.md")
    return {
        "REPO_ROOT": REPO_ROOT,
        "BENCHMARK": benchmark,
        "REPORT": report_path(benchmark),
        "SOURCE_ROOT": source_root,
        "PROMPT": validation_prompt_path(prompt_version),
        "README_PATH": source_root / "README.md",
        "SCOPE_DOC_PATHS": markdown_path_list(context_docs),
        "SCOPE_PATHS": markdown_path_list(scope_paths) if scope_paths else "- `(none found)`",
        "DOCS_PATHS": markdown_path_list(docs_paths) if docs_paths else "- `(none found)`",
        "IMMUNEFI_BOUNTY_RULES_PATHS": markdown_path_list(immunefi_rules) if immunefi_rules else "- `(none configured)`",
        "IMMUNEFI_SEVERITY_RUBRIC_PATHS": markdown_path_list(immunefi_rubrics) if immunefi_rubrics else "- `(none configured)`",
        "IMMUNEFI_POC_RUNTIME_PATHS": markdown_path_list(immunefi_poc_runtime) if immunefi_poc_runtime else "- `(none configured)`",
        "IMMUNEFI_BOUNTY_RULES": prompt_file_contents(immunefi_rules, "Immunefi bounty rules"),
        "IMMUNEFI_SEVERITY_RUBRIC": prompt_file_contents(immunefi_rubrics, "Immunefi severity rubric"),
        "IMMUNEFI_POC_RUNTIME": prompt_file_contents(immunefi_poc_runtime, "Immunefi PoC runtime"),
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
        config,
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
        config,
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
        config,
    )


def render_feasibility_prompt(prompt_version: str, benchmark: str, run_id: str, config: dict[str, object]) -> str:
    input_path = write_round3a_input(
        round3a_input_path(prompt_version, benchmark, run_id),
        prompt_version,
        benchmark,
        run_id,
    )
    return render_stage_prompt(
        "r3a.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **worker_replacements(config, "r3a", FEASIBILITY_WORKER_MODEL, FEASIBILITY_WORKER_REASONING),
            "INPUT_PATH": input_path,
            "STAGE_PATH": feasibility_screen_path(prompt_version, benchmark, run_id),
            "FINAL_RUN_PATH": final_run_path(prompt_version, benchmark, run_id),
            "FEASIBILITY_RUN_PATH": feasibility_run_path(prompt_version, benchmark, run_id),
        },
        config,
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
        config,
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
        config,
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
        config,
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
        config,
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
        config,
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
        config,
    )


def render_judge_simulation_prompt(
    prompt_version: str,
    benchmark: str,
    run_id: str,
    finding_id: str,
    config: dict[str, object],
) -> str:
    blocks = submission_candidate_blocks(prompt_version, benchmark, run_id)
    finding_id = validate_finding_id(finding_id, blocks)
    input_path = write_round9_input(
        round9_input_path(prompt_version, benchmark, run_id, finding_id),
        prompt_version,
        benchmark,
        run_id,
        finding_id,
        blocks[finding_id],
    )
    return render_stage_prompt(
        "r9.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            **contest_replacements(config),
            **worker_replacements(config, "r9", JUDGE_WORKER_MODEL, JUDGE_WORKER_REASONING),
            "FINDING_ID": finding_id,
            "JUDGE_INPUT_PATH": input_path,
            "FINDING_REPORT_PATH": finding_report_unit_path(prompt_version, benchmark, run_id, finding_id),
            "JUDGE_SIMULATION_PATH": judge_simulation_unit_path(prompt_version, benchmark, run_id, finding_id),
            "JUDGE_SIMULATION_JSON_PATH": judge_simulation_unit_json_path(prompt_version, benchmark, run_id, finding_id),
        },
        config,
    )


def auto_scope_code_evidence(source_root: Path) -> str:
    docs = ", ".join(f"`{path}`" for path in benchmark_scope_docs(source_root))
    return (
        "Excluded during the stage 1 scope / known-issue screen using "
        f"{docs}."
    )


def auto_token_code_evidence(source_root: Path) -> str:
    docs = ", ".join(f"`{path}`" for path in benchmark_scope_docs(source_root))
    if is_bounty_profile():
        return (
            "Excluded during the stage 2 bounty exploitability screen using "
            f"{docs}, and the stage 1 scope screen."
        )
    if is_private_client_profile():
        return (
            "Excluded during the stage 2 private-client assumption screen using "
            f"{docs}, and the stage 1 scope screen."
        )
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
            f"Stage 2 {stage2_screen_label()} screen: `{token_screen_path(prompt_version, benchmark, run_id)}`",
            f"Stage 3 final validation run: `{stage3_run_path(prompt_version, benchmark, run_id)}`",
            "",
            "## Assembly Summary",
            "",
            f"- Excluded at stage 1 (scope / known issue): `{scope_excluded}`",
            f"- Excluded at stage 2 ({stage2_screen_label()}): `{token_excluded}`",
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


def apply_feasibility(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    run_path = final_run_path(prompt_version, benchmark, run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    rows = require_feasibility_rows(prompt_version, benchmark, run_id)
    original_text = run_path.read_text()
    blocks = raw_finding_blocks(run_path)
    adjusted_blocks: list[str] = []
    keep = reclassified = needs_review = excluded = 0

    for fid, block in blocks.items():
        row = rows.get(fid)
        if row:
            decision = row["decision"].strip().lower()
            if decision == "keep":
                keep += 1
            elif decision in {"reclassify as medium griefing", "downgrade to medium griefing"}:
                reclassified += 1
            elif decision == "needs review":
                needs_review += 1
            elif decision == "exclude":
                excluded += 1
            adjusted_blocks.append(apply_feasibility_to_block(block, row))
        else:
            adjusted_blocks.append(block.strip())

    prefix = original_text.split("## Per-Finding Validation", 1)[0].rstrip()
    out_path = feasibility_run_path(prompt_version, benchmark, run_id)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    content = "\n".join(
        [
            prefix,
            f"Stage 3a Immunefi feasibility screen: `{feasibility_screen_path(prompt_version, benchmark, run_id)}`",
            "",
            "## R3a Feasibility Summary",
            "",
            f"- Reviewed reportable candidates: `{len(rows)}`",
            f"- Kept as classified: `{keep}`",
            f"- Reclassified as Medium griefing: `{reclassified}`",
            f"- Marked Needs Review: `{needs_review}`",
            f"- Excluded by feasibility gate: `{excluded}`",
            "",
            "## Per-Finding Validation",
            "",
            "\n\n".join(adjusted_blocks),
            "",
        ]
    )
    out_path.write_text(content)
    return {
        "benchmark": benchmark,
        "prompt_version": prompt_version,
        "run_id": run_id,
        "source_run_path": str(run_path),
        "feasibility_screen_path": str(feasibility_screen_path(prompt_version, benchmark, run_id)),
        "feasibility_run_path": str(out_path),
        "reviewed_candidates": len(rows),
        "kept_as_classified": keep,
        "reclassified_as_medium_griefing": reclassified,
        "needs_review": needs_review,
        "excluded": excluded,
    }


def apply_dedup(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    run_path = current_validated_run_path(prompt_version, benchmark, run_id)
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
            f"- Candidate reportable findings before R4: `{len(kept) + len(dropped)}`",
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
            "\n\n".join(kept_blocks) if kept_blocks else "_No reportable candidates remain after dedup._",
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
            "\n\n".join(kept_blocks) if kept_blocks else "_No reportable candidates remain after the V12 sweep._",
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
    run_path = current_validated_run_path(prompt_version, benchmark, run_id)
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
        "worker_type": "three-shot-r1-bounty-scope"
        if is_bounty_profile(config)
        else "three-shot-r1-private-client-scope"
        if is_private_client_profile(config)
        else "three-shot-r1-scope",
        **worker_payload(config, "r1", STAGE_WORKER_MODEL, STAGE_WORKER_REASONING),
        "worker_prompt_source": str(
            prompt_template_path("r1.md", config)
        ),
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
        stage2_screen_title(config),
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
        "worker_type": stage2_worker_type(config),
        **worker_payload(config, "r2", STAGE_WORKER_MODEL, STAGE_WORKER_REASONING),
        "worker_prompt_source": str(
            prompt_template_path("r2.md", config)
        ),
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
        "worker_type": stage3_worker_type(config),
        **worker_payload(config, "r3", STAGE_WORKER_MODEL, STAGE_WORKER_REASONING),
        "worker_prompt_source": str(
            prompt_template_path("r3.md", config)
        ),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_feasibility(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    if not feasibility_gate_enabled(config):
        payload = {
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "worker_type": "none",
            "reason": (
                "R3a feasibility gate is disabled. It only runs for validation_profile: "
                f"immunefi-bounty with rounds.r3a_feasibility_gate: true; current profile is {validation_profile(config)}."
            ),
        }
        print(json.dumps(payload, indent=2, sort_keys=True))
        return
    candidates = reportable_stage3_candidates(args.prompt_version, args.benchmark, args.run_id)
    path = init_feasibility_screen(args.prompt_version, args.benchmark, args.run_id, reset=args.reset)
    prompt_text = render_feasibility_prompt(args.prompt_version, args.benchmark, args.run_id, config)
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "stage_path": str(path),
        "input_findings": len(candidates),
        "worker_type": "three-shot-r3a-immunefi-feasibility-gate",
        **worker_payload(config, "r3a", FEASIBILITY_WORKER_MODEL, FEASIBILITY_WORKER_REASONING),
        "worker_prompt_source": str(prompt_template_path("r3a.md", config)),
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
        "worker_prompt_source": str(prompt_template_path("r4.md", config)),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_v12_sweep(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    if is_bounty_profile(config):
        payload = {
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "worker_type": "none",
            "reason": f"R4a V12 sweep is disabled for {validation_profile(config)} validation_profile.",
        }
        print(json.dumps(payload, indent=2, sort_keys=True))
        return
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
        "worker_prompt_source": str(prompt_template_path("r4a.md", config)),
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
        "poc_runtime": immunefi_poc_runtime_payload(args.benchmark)
        if is_immunefi_bounty_profile(config)
        else None,
        "input_scope": "single_post_r4_submission_candidate",
        "input_findings": 1,
        "total_findings": len(blocks),
        "completed_findings": completed,
        "remaining_findings": len(blocks) - completed,
        "worker_type": "three-shot-r5-poc-generation-finding",
        **worker_payload(config, "r5", POC_WORKER_MODEL, POC_WORKER_REASONING),
        "worker_prompt_source": str(prompt_template_path("r5.md", config)),
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
        "poc_runtime": immunefi_poc_runtime_payload(args.benchmark)
        if is_immunefi_bounty_profile(config)
        else None,
        "input_scope": "single_post_r4_submission_candidate",
        "input_findings": 1,
        "total_findings": len(blocks),
        "completed_findings": completed,
        "remaining_findings": len(blocks) - completed,
        "worker_type": "three-shot-r6-poc-verification-finding",
        **worker_payload(config, "r6", POC_WORKER_MODEL, POC_WORKER_REASONING),
        "worker_prompt_source": str(prompt_template_path("r6.md", config)),
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
            "All R7 per-finding report units are complete."
            if phase == "complete"
            else "No reportable R7 unit is available because some R6 units are incomplete or not profile-reportable."
        )
        payload = {
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "worker_type": "none",
            "phase": phase,
            "input_scope": "single_poc_reviewed_reportable_finding",
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
        raise SystemExit(f"Finding {finding_id} is not reportable until R6 marks it verified or profile-reportable without PoC.")
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
        "input_scope": "single_poc_reviewed_reportable_finding",
        "input_findings": 1,
        "total_reportable_findings": len(reportable),
        "completed_findings": completed,
        "remaining_findings": len(reportable) - completed,
        "worker_type": profile_worker_type("r7", config),
        **worker_payload(config, "r7", REPORT_WORKER_MODEL, REPORT_WORKER_REASONING),
        "worker_prompt_source": str(
            prompt_template_path("r7.md", config)
        ),
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
            "All R8 per-finding report review units are complete."
            if phase == "complete"
            else "No reviewable R8 unit is available because some R7 report units are incomplete."
        )
        payload = {
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "worker_type": "none",
            "phase": phase,
            "input_scope": report_review_input_scope(config),
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
        raise SystemExit(f"Finding {finding_id} is not reportable until R6 marks it verified or profile-reportable without PoC.")
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
        "input_scope": report_review_input_scope(config),
        "input_findings": 1,
        "total_reportable_findings": len(reviewable),
        "completed_findings": completed,
        "remaining_findings": len(reviewable) - completed,
        "worker_type": profile_worker_type("r8", config),
        **worker_payload(config, "r8", REPORT_WORKER_MODEL, REPORT_WORKER_REASONING),
        "worker_prompt_source": str(
            prompt_template_path("r8.md", config)
        ),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        attach_worker_prompt_path(payload, prompt_text, Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_judge(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    blocks = submission_candidate_blocks(args.prompt_version, args.benchmark, args.run_id)
    expected_reports = [
        fid
        for fid in blocks
        if reportable_after_poc_review(args.prompt_version, args.benchmark, args.run_id, fid)
    ]
    judgeable = [
        fid
        for fid in expected_reports
        if report_review_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
    ]
    requested_finding = args.finding_id or next_judge_simulation_finding(args.prompt_version, args.benchmark, args.run_id)
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
        completed_r8 = sum(
            1
            for fid in expected_reports
            if report_review_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
        )
        completed = sum(
            1
            for fid in expected_reports
            if judge_simulation_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
        )
        phase = (
            "complete"
            if (
                completed_r6 == len(blocks)
                and completed_r7 == len(expected_reports)
                and completed_r8 == len(expected_reports)
                and completed == len(expected_reports)
            )
            else "blocked"
        )
        summary = (
            "All R9 per-finding judge simulation units are complete."
            if phase == "complete"
            else "No judgeable R9 unit is available because some upstream R7/R8 units are incomplete."
        )
        payload = {
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "worker_type": "none",
            "phase": phase,
            "input_scope": "single_reviewed_submission_report",
            "reportable_findings": len(expected_reports),
            "judgeable_findings": len(judgeable),
            "completed_r6_findings": completed_r6,
            "completed_r7_findings": completed_r7,
            "completed_r8_findings": completed_r8,
            "completed_findings": completed,
            "source_candidates_path": str(submission_candidates_path(args.prompt_version, args.benchmark, args.run_id)),
            "summary": summary,
        }
        print(json.dumps(payload, indent=2, sort_keys=True))
        return

    finding_id = validate_finding_id(requested_finding, blocks)
    if not reportable_after_poc_review(args.prompt_version, args.benchmark, args.run_id, finding_id):
        raise SystemExit(f"Finding {finding_id} is not judgeable until R6 marks it verified or profile-reportable without PoC.")
    if not report_unit_complete(args.prompt_version, args.benchmark, args.run_id, finding_id):
        raise SystemExit(f"R7 finding report for {finding_id} is incomplete; cannot prepare R9.")
    if not report_review_unit_complete(args.prompt_version, args.benchmark, args.run_id, finding_id):
        raise SystemExit(f"R8 finding report review for {finding_id} is incomplete; cannot prepare R9.")
    input_path = write_round9_input(
        round9_input_path(args.prompt_version, args.benchmark, args.run_id, finding_id),
        args.prompt_version,
        args.benchmark,
        args.run_id,
        finding_id,
        blocks[finding_id],
    )
    path = init_judge_simulation_unit(args.prompt_version, args.benchmark, args.run_id, finding_id, reset=args.reset)
    prompt_text = render_judge_simulation_prompt(args.prompt_version, args.benchmark, args.run_id, finding_id, config)
    completed = sum(
        1
        for fid in judgeable
        if judge_simulation_unit_complete(args.prompt_version, args.benchmark, args.run_id, fid)
    )
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "finding_id": finding_id,
        "stage_path": str(path),
        "json_path": str(judge_simulation_unit_json_path(args.prompt_version, args.benchmark, args.run_id, finding_id)),
        "single_finding_input_path": str(input_path),
        "source_candidates_path": str(submission_candidates_path(args.prompt_version, args.benchmark, args.run_id)),
        "input_scope": "single_reviewed_submission_report",
        "input_findings": 1,
        "total_judgeable_findings": len(judgeable),
        "completed_findings": completed,
        "remaining_findings": len(judgeable) - completed,
        "worker_type": profile_worker_type("r9", config),
        **worker_payload(config, "r9", JUDGE_WORKER_MODEL, JUDGE_WORKER_REASONING),
        "worker_prompt_source": str(prompt_template_path("r9.md", config)),
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


def cmd_apply_feasibility(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    if not feasibility_gate_enabled(config):
        print(json.dumps({
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "skipped": True,
            "reason": (
                "R3a feasibility gate is disabled. It only runs for validation_profile: "
                f"immunefi-bounty with rounds.r3a_feasibility_gate: true; current profile is {validation_profile(config)}."
            ),
        }, indent=2, sort_keys=True))
        return
    print(json.dumps(apply_feasibility(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_apply_v12_sweep(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    if is_bounty_profile(config):
        print(json.dumps({
            "benchmark": args.benchmark,
            "prompt_version": args.prompt_version,
            "run_id": args.run_id,
            "skipped": True,
            "reason": f"R4a V12 sweep is disabled for {validation_profile(config)} validation_profile.",
        }, indent=2, sort_keys=True))
        return
    print(json.dumps(apply_v12_sweep(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_assemble_poc(args: argparse.Namespace) -> None:
    resolve_run_args(args)
    print(json.dumps(assemble_poc_units(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_assemble_poc_review(args: argparse.Namespace) -> None:
    resolve_run_args(args)
    print(json.dumps(assemble_poc_review_units(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_score_prompt(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    run_path = current_validated_run_path(args.prompt_version, args.benchmark, args.run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    prompt_text = render_stage_prompt(
        "score.md",
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
        },
        config,
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


def controller_args(args: argparse.Namespace, command: str, extra: list[str] | None = None) -> list[str]:
    cmd = [
        sys.executable,
        str(Path(__file__).resolve()),
        command,
        "--config",
        str(args.config),
        "--prompt-version",
        args.prompt_version,
        "--benchmark",
        args.benchmark,
        "--run-id",
        args.run_id,
    ]
    if extra:
        cmd.extend(extra)
    return cmd


def run_controller_json(args: argparse.Namespace, command: str, extra: list[str] | None = None) -> dict[str, object]:
    result = subprocess.run(
        controller_args(args, command, extra),
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        if result.stdout:
            print(result.stdout, end="")
        if result.stderr:
            print(result.stderr, end="", file=sys.stderr)
        raise SystemExit(result.returncode)

    try:
        payload = json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"{command} did not emit JSON: {exc}\n{result.stdout}") from exc

    print(json.dumps(payload, indent=2, sort_keys=True))
    return payload


def run_controller_plain(args: argparse.Namespace, command: str, extra: list[str] | None = None) -> None:
    subprocess.run(controller_args(args, command, extra), cwd=REPO_ROOT, check=True)


def run_prepared_worker(payload: dict[str, object], dry_run: bool) -> bool:
    worker_type = str(payload.get("worker_type", ""))
    if worker_type == "none":
        print(json.dumps({"worker_type": worker_type, "skipped": True, "reason": payload.get("reason") or payload.get("summary")}, indent=2, sort_keys=True))
        return False

    command = payload.get("worker_spawn_command")
    if not isinstance(command, str) or not command.strip():
        raise SystemExit(f"Prepared worker payload is missing worker_spawn_command for {worker_type}")

    if dry_run:
        print(json.dumps({
            "worker_type": worker_type,
            "dry_run": True,
            "prepared_only": True,
            "worker_spawn_command": command,
            "note": "Dry-run stops after preparing this worker because later controller steps require worker-written artifacts.",
        }, indent=2, sort_keys=True))
        return False

    subprocess.run(command, cwd=REPO_ROOT, shell=True, executable="/bin/bash", check=True)
    return True


def run_worker_stage(
    args: argparse.Namespace,
    command: str,
    prompt_dir: Path,
    counter: int,
    extra: list[str] | None = None,
) -> tuple[dict[str, object], int, bool]:
    prompt_path = prompt_dir / f"{counter:03}-{command}.md"
    payload = run_controller_json(args, command, [*(extra or []), "--write-prompt", str(prompt_path)])
    ran = run_prepared_worker(payload, args.dry_run)
    return payload, counter + 1, ran


def run_worker_units_until_done(
    args: argparse.Namespace,
    command: str,
    prompt_dir: Path,
    counter: int,
) -> int:
    units = 0
    while True:
        if units >= args.max_worker_units:
            raise SystemExit(f"Stopped after {args.max_worker_units} {command} units; increase --max-worker-units to continue.")
        payload, counter, ran = run_worker_stage(args, command, prompt_dir, counter)
        if not ran:
            phase = str(payload.get("phase", "complete"))
            if phase not in {"complete", "none"} and payload.get("worker_type") == "none":
                raise SystemExit(f"{command} is blocked: {payload.get('summary') or payload.get('reason')}")
            return counter
        units += 1


def cmd_run(args: argparse.Namespace) -> None:
    config = resolve_run_args(args)
    prompt_dir = (
        REPO_ROOT
        / ".ai-agent-audit"
        / "validation-prompts"
        / f"{args.benchmark}-{args.run_id}"
    )
    prompt_dir.mkdir(parents=True, exist_ok=True)
    counter = 1

    print(json.dumps({
        "mode": "codex-immediate",
        "benchmark": args.benchmark,
        "run_id": args.run_id,
        "prompt_version": args.prompt_version,
        "prompt_dir": str(prompt_dir),
        "dry_run": args.dry_run,
    }, indent=2, sort_keys=True))

    _, counter, ran = run_worker_stage(args, "prepare-scope", prompt_dir, counter)
    if not ran:
        return
    _, counter, ran = run_worker_stage(args, "prepare-token", prompt_dir, counter)
    if not ran:
        return
    _, counter, ran = run_worker_stage(args, "prepare-final", prompt_dir, counter)
    if not ran:
        return
    run_controller_plain(args, "assemble")

    if args.stop_after == "r3":
        return

    if round_enabled(config, "r3a_feasibility_gate", False) and feasibility_gate_enabled(config):
        _, counter, ran = run_worker_stage(args, "prepare-feasibility", prompt_dir, counter)
        if ran:
            run_controller_plain(args, "apply-feasibility")

    if round_enabled(config, "r4_canonicalization", True):
        _, counter, ran = run_worker_stage(args, "prepare-dedup", prompt_dir, counter)
        if ran:
            run_controller_plain(args, "apply-dedup")

    if args.stop_after == "r4":
        return

    if round_enabled(config, "r4a_v12_sweep", True) and not is_bounty_profile(config):
        _, counter, ran = run_worker_stage(args, "prepare-v12-sweep", prompt_dir, counter)
        if ran:
            run_controller_plain(args, "apply-v12-sweep")

    if args.stop_after == "r4a":
        return

    if args.skip_poc:
        print(json.dumps({"skipped": True, "phase": "poc_and_reports", "reason": "--skip-poc was set"}, indent=2, sort_keys=True))
        return

    if round_enabled(config, "r5_poc_generation", True):
        counter = run_worker_units_until_done(args, "prepare-poc", prompt_dir, counter)
        run_controller_plain(args, "assemble-poc")

    if round_enabled(config, "r6_poc_verification", True):
        counter = run_worker_units_until_done(args, "prepare-poc-review", prompt_dir, counter)
        run_controller_plain(args, "assemble-poc-review")

    if args.stop_after == "r6":
        return

    if args.skip_reports:
        print(json.dumps({"skipped": True, "phase": "reports", "reason": "--skip-reports was set"}, indent=2, sort_keys=True))
        return

    if round_enabled(config, "r7_c4_report_generation", True):
        counter = run_worker_units_until_done(args, "prepare-report", prompt_dir, counter)

    if round_enabled(config, "r8_c4_report_review", True):
        counter = run_worker_units_until_done(args, "prepare-report-review", prompt_dir, counter)

    if args.stop_after == "r8":
        return

    if args.include_judge:
        counter = run_worker_units_until_done(args, "prepare-judge", prompt_dir, counter)

    if args.include_scoring and round_enabled(config, "scoring", False):
        _, counter, _ = run_worker_stage(args, "score-prompt", prompt_dir, counter)

    print(json.dumps({
        "status": "complete",
        "benchmark": args.benchmark,
        "run_id": args.run_id,
        "prompt_dir": str(prompt_dir),
    }, indent=2, sort_keys=True))


def add_run_args(subparser: argparse.ArgumentParser) -> None:
    subparser.add_argument("--config", default=str(DEFAULT_CONFIG_PATH), help="YAML config path.")
    subparser.add_argument("--prompt-version", default=None)
    subparser.add_argument("--benchmark", default=None)
    subparser.add_argument("--run-id", default=None)


def main() -> None:
    parser = argparse.ArgumentParser(description="Prepare, assemble, and score three-shot validation experiments.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    run_cmd = subparsers.add_parser("run", help="Run configured validation rounds immediately with sequential Codex workers.")
    add_run_args(run_cmd)
    run_cmd.add_argument("--dry-run", action="store_true", help="Prepare the next runnable worker prompt/command, then stop before dependent assemble/apply steps.")
    run_cmd.add_argument("--skip-poc", action="store_true", help="Stop after validation/canonicalization and skip PoC/report rounds.")
    run_cmd.add_argument("--skip-reports", action="store_true", help="Run PoC rounds but skip report creation/review.")
    run_cmd.add_argument("--include-judge", action="store_true", help="Also run round 9 judge simulations.")
    run_cmd.add_argument("--include-scoring", action="store_true", help="Also run the scoring worker when configured.")
    run_cmd.add_argument("--max-worker-units", type=int, default=200, help="Safety cap for per-finding worker units.")
    run_cmd.add_argument("--stop-after", choices=["r3", "r4", "r4a", "r6", "r8"], default=None)
    run_cmd.set_defaults(func=cmd_run)

    prepare_scope = subparsers.add_parser("prepare-scope", help="Initialize stage 1 scope screening and emit the worker prompt.")
    add_run_args(prepare_scope)
    prepare_scope.add_argument("--include-prompt", action="store_true")
    prepare_scope.add_argument("--write-prompt")
    prepare_scope.set_defaults(func=cmd_prepare_scope)

    prepare_token = subparsers.add_parser("prepare-token", help="Initialize stage 2 profile-specific screening and emit the worker prompt.")
    add_run_args(prepare_token)
    prepare_token.add_argument("--include-prompt", action="store_true")
    prepare_token.add_argument("--write-prompt")
    prepare_token.set_defaults(func=cmd_prepare_token)

    prepare_final = subparsers.add_parser("prepare-final", help="Initialize stage 3 final validation and emit the worker prompt.")
    add_run_args(prepare_final)
    prepare_final.add_argument("--include-prompt", action="store_true")
    prepare_final.add_argument("--write-prompt")
    prepare_final.set_defaults(func=cmd_prepare_final)

    prepare_feasibility = subparsers.add_parser("prepare-feasibility", help="Initialize optional round 3a Immunefi feasibility gate and emit the worker prompt.")
    add_run_args(prepare_feasibility)
    prepare_feasibility.add_argument("--include-prompt", action="store_true")
    prepare_feasibility.add_argument("--write-prompt")
    prepare_feasibility.add_argument("--reset", action="store_true", help="Reset the round 3a feasibility screen before emitting the prompt.")
    prepare_feasibility.set_defaults(func=cmd_prepare_feasibility)

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

    prepare_report = subparsers.add_parser("prepare-report", help="Initialize round 7 report creation for one finding.")
    add_run_args(prepare_report)
    prepare_report.add_argument("--finding-id", help="Prepare this exact finding. Defaults to the next R6-verified/R7-unfinished unit.")
    prepare_report.add_argument("--include-prompt", action="store_true")
    prepare_report.add_argument("--write-prompt")
    prepare_report.add_argument("--reset", action="store_true", help="Reset this finding's round 7 report before emitting the prompt.")
    prepare_report.set_defaults(func=cmd_prepare_report)

    prepare_report_review = subparsers.add_parser("prepare-report-review", help="Initialize round 8 report review for one finding.")
    add_run_args(prepare_report_review)
    prepare_report_review.add_argument("--finding-id", help="Prepare this exact finding. Defaults to the next R7-complete/R8-unfinished unit.")
    prepare_report_review.add_argument("--include-prompt", action="store_true")
    prepare_report_review.add_argument("--write-prompt")
    prepare_report_review.add_argument("--reset", action="store_true", help="Reset this finding's round 8 report review before emitting the prompt.")
    prepare_report_review.set_defaults(func=cmd_prepare_report_review)

    prepare_judge = subparsers.add_parser("prepare-judge", help="Initialize round 9 judge simulation for one finding.")
    add_run_args(prepare_judge)
    prepare_judge.add_argument("--finding-id", help="Prepare this exact finding. Defaults to the next R8-complete/R9-unfinished unit.")
    prepare_judge.add_argument("--include-prompt", action="store_true")
    prepare_judge.add_argument("--write-prompt")
    prepare_judge.add_argument("--reset", action="store_true", help="Reset this finding's round 9 judge simulation before emitting the prompt.")
    prepare_judge.set_defaults(func=cmd_prepare_judge)

    assemble = subparsers.add_parser("assemble", help="Assemble the final three-shot run from stages 1-3.")
    add_run_args(assemble)
    assemble.set_defaults(func=cmd_assemble)

    apply_feasibility_cmd = subparsers.add_parser("apply-feasibility", help="Apply round 3a Immunefi feasibility decisions to create a feasibility-adjusted run.")
    add_run_args(apply_feasibility_cmd)
    apply_feasibility_cmd.set_defaults(func=cmd_apply_feasibility)

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
