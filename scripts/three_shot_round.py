#!/usr/bin/env python3
"""Prepare, assemble, and score three-shot validation experiments.

Three-shot validation splits the full-report pass into:
1. scope / known-issue screening
2. unsupported-token screening
3. full validation on the surviving findings

This is intended to cheaply strip obvious false positives before the expensive
final gate analysis while preserving the same final scoring layer used by the
one-shot experiment.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

import validation_loop as vl
import oneshot_round as osr


THREE_SHOT_ROOT = vl.REPO_ROOT / "validation-three-shot"
THREE_SHOT_PROMPT_ROOT = THREE_SHOT_ROOT / "prompts"
STAGE_WORKER_MODEL = "gpt-5.5"
STAGE_WORKER_REASONING = "xhigh"
SCORING_WORKER_MODEL = "gpt-5.4"
SCORING_WORKER_REASONING = "xhigh"
SCREEN_ANCHOR = "<!-- APPEND SCREEN ROWS ABOVE THIS LINE -->"


def scope_screen_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "scope-screens" / prompt_version / f"{benchmark}-{run_id}.md"


def token_screen_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "token-screens" / prompt_version / f"{benchmark}-{run_id}.md"


def stage3_run_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "stage3-runs" / prompt_version / f"{benchmark}-{run_id}.md"


def final_run_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "runs" / prompt_version / f"{benchmark}-{run_id}.md"


def result_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "results" / prompt_version / f"{benchmark}-{run_id}.md"


def score_json_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return THREE_SHOT_ROOT / "results" / prompt_version / f"{benchmark}-{run_id}.json"


def benchmark_scope_docs(source_root: Path) -> list[Path]:
    paths = [source_root / "README.md"]
    paths.extend(sorted(source_root.glob("*-scope.md")))
    paths.extend(sorted(source_root.glob("*-docs.md")))
    paths.extend([source_root / "v12-findings.md", source_root / "v12-checklist.md"])
    return paths


def markdown_path_list(paths: list[Path]) -> str:
    return "\n".join(f"- `{path}`" for path in paths)


def render_findings_blocks(findings: list[vl.Finding]) -> str:
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


def all_findings(benchmark: str) -> list[vl.Finding]:
    return vl.parse_findings(vl.report_path(benchmark))


def init_screen(path: Path, title: str, benchmark: str, prompt_version: str) -> Path:
    if path.exists():
        return path

    path.parent.mkdir(parents=True, exist_ok=True)
    source_root = vl.benchmark_source_root(benchmark)
    content = f"""# {benchmark} {title}

Status: In progress
Benchmark report: `{vl.report_path(benchmark)}`
Benchmark source root: `{source_root}`
Validation prompt: `{vl.prompt_path(prompt_version)}`

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
    source_root = vl.benchmark_source_root(benchmark)
    content = f"""# {benchmark} Three-Shot Stage 3 {run_id}

Status: In progress
Benchmark report: `{vl.report_path(benchmark)}`
Benchmark source root: `{source_root}`
Validation prompt: `{vl.prompt_path(prompt_version)}`

Mandatory benchmark docs:
{markdown_path_list(benchmark_scope_docs(source_root))}

## Per-Finding Validation

{vl.APPEND_ANCHOR}
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


def kept_after_scope(prompt_version: str, benchmark: str, run_id: str) -> list[vl.Finding]:
    rows = require_scope_rows(prompt_version, benchmark, run_id)
    return [finding for finding in all_findings(benchmark) if rows.get(finding.fid, {}).get("decision") == "Keep"]


def kept_after_token(prompt_version: str, benchmark: str, run_id: str) -> list[vl.Finding]:
    scope_rows = require_scope_rows(prompt_version, benchmark, run_id)
    token_rows = require_token_rows(prompt_version, benchmark, run_id)
    kept: list[vl.Finding] = []
    for finding in all_findings(benchmark):
        if scope_rows.get(finding.fid, {}).get("decision") != "Keep":
            continue
        if token_rows.get(finding.fid, {}).get("decision") == "Keep":
            kept.append(finding)
    return kept


def render_common_replacements(prompt_version: str, benchmark: str) -> dict[str, object]:
    source_root = vl.benchmark_source_root(benchmark)
    scope_paths = sorted(source_root.glob("*-scope.md"))
    docs_paths = sorted(source_root.glob("*-docs.md"))
    return {
        "REPO_ROOT": vl.REPO_ROOT,
        "BENCHMARK": benchmark,
        "REPORT": vl.report_path(benchmark),
        "SOURCE_ROOT": source_root,
        "PROMPT": vl.prompt_path(prompt_version),
        "README_PATH": source_root / "README.md",
        "SCOPE_DOC_PATHS": markdown_path_list(benchmark_scope_docs(source_root)),
        "SCOPE_PATHS": markdown_path_list(scope_paths) if scope_paths else "- `(none found)`",
        "DOCS_PATHS": markdown_path_list(docs_paths) if docs_paths else "- `(none found)`",
        "V12_FINDINGS_PATH": source_root / "v12-findings.md",
        "V12_CHECKLIST_PATH": source_root / "v12-checklist.md",
    }


def render_scope_prompt(prompt_version: str, benchmark: str, run_id: str) -> str:
    return render_stage_prompt(
        "r1.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            "STAGE_PATH": scope_screen_path(prompt_version, benchmark, run_id),
            "ALL_FINDINGS_BLOCKS": render_findings_blocks(all_findings(benchmark)),
        },
    )


def render_token_prompt(prompt_version: str, benchmark: str, run_id: str) -> str:
    return render_stage_prompt(
        "r2.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            "SCOPE_PATH": scope_screen_path(prompt_version, benchmark, run_id),
            "STAGE_PATH": token_screen_path(prompt_version, benchmark, run_id),
            "ALL_FINDINGS_BLOCKS": render_findings_blocks(kept_after_scope(prompt_version, benchmark, run_id)),
        },
    )


def render_stage3_prompt(prompt_version: str, benchmark: str, run_id: str) -> str:
    return render_stage_prompt(
        "r3.md",
        {
            **render_common_replacements(prompt_version, benchmark),
            "SCOPE_PATH": scope_screen_path(prompt_version, benchmark, run_id),
            "TOKEN_PATH": token_screen_path(prompt_version, benchmark, run_id),
            "STAGE_PATH": stage3_run_path(prompt_version, benchmark, run_id),
            "APPEND_ANCHOR": vl.APPEND_ANCHOR,
            "ALL_FINDINGS_BLOCKS": render_findings_blocks(kept_after_token(prompt_version, benchmark, run_id)),
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


def build_scope_block(finding: vl.Finding, row: dict[str, str], source_root: Path) -> str:
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


def build_token_block(finding: vl.Finding, row: dict[str, str], source_root: Path) -> str:
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
    source_root = vl.benchmark_source_root(benchmark)
    findings = all_findings(benchmark)
    scope_rows = parse_screen(scope_screen_path(prompt_version, benchmark, run_id))
    token_rows = parse_screen(token_screen_path(prompt_version, benchmark, run_id))
    stage3_blocks = vl.raw_finding_blocks(stage3_run_path(prompt_version, benchmark, run_id))

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
            f"Benchmark report: `{vl.report_path(benchmark)}`",
            f"Benchmark source root: `{source_root}`",
            f"Validation prompt: `{vl.prompt_path(prompt_version)}`",
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


def score_three_shot(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    run_path = final_run_path(prompt_version, benchmark, run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    blocks = vl.raw_finding_blocks(run_path)
    findings = vl.parse_findings(vl.report_path(benchmark))
    truth_rows = osr.parse_truth_key()

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

        decision = vl.block_field(block, "Decision")
        severity = vl.block_field(block, "Severity Assessment")
        confidence = vl.block_field(block, "Confidence")
        bug_exists = vl.block_field(block, "Bug Exists")
        root_cause_family = vl.block_field(block, "Root Cause Family")

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
                "title": vl.block_field(block, "Finding Title") or finding.title,
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
    total_canonical = osr.total_canonical_approved_hm_findings(benchmark)
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
                    report_id=osr.clean(row["report_id"]),
                    title=osr.clean(row["title"]),
                    decision=osr.clean(row["decision"]),
                    confidence=osr.clean(row["confidence"]),
                    bug_exists=osr.clean(row["bug_exists"]),
                    severity=osr.clean(row["severity"]),
                    truth=osr.clean(row["truth"]),
                    c4=osr.clean(row["matching_c4_listing"]),
                    root=osr.clean(row["root_cause_family"]),
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
    path = init_screen(
        scope_screen_path(args.prompt_version, args.benchmark, args.run_id),
        "Three-Shot Stage 1 Scope Screen",
        args.benchmark,
        args.prompt_version,
    )
    prompt_text = render_scope_prompt(args.prompt_version, args.benchmark, args.run_id)
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "stage_path": str(path),
        "worker_type": "three-shot-r1-scope",
        "worker_model": STAGE_WORKER_MODEL,
        "worker_reasoning_effort": STAGE_WORKER_REASONING,
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r1.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_token(args: argparse.Namespace) -> None:
    input_findings = kept_after_scope(args.prompt_version, args.benchmark, args.run_id)
    path = init_screen(
        token_screen_path(args.prompt_version, args.benchmark, args.run_id),
        "Three-Shot Stage 2 Unsupported-Token Screen",
        args.benchmark,
        args.prompt_version,
    )
    prompt_text = render_token_prompt(args.prompt_version, args.benchmark, args.run_id)
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "stage_path": str(path),
        "input_findings": len(input_findings),
        "worker_type": "three-shot-r2-token",
        "worker_model": STAGE_WORKER_MODEL,
        "worker_reasoning_effort": STAGE_WORKER_REASONING,
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r2.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_prepare_final(args: argparse.Namespace) -> None:
    input_findings = kept_after_token(args.prompt_version, args.benchmark, args.run_id)
    path = init_stage3_run(args.prompt_version, args.benchmark, args.run_id)
    prompt_text = render_stage3_prompt(args.prompt_version, args.benchmark, args.run_id)
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "stage_path": str(path),
        "input_findings": len(input_findings),
        "worker_type": "three-shot-r3-final-validation",
        "worker_model": STAGE_WORKER_MODEL,
        "worker_reasoning_effort": STAGE_WORKER_REASONING,
        "worker_prompt_source": str(THREE_SHOT_PROMPT_ROOT / "r3.md"),
        "requires_fresh_worker_context": True,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_assemble(args: argparse.Namespace) -> None:
    print(json.dumps(assemble_run(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def cmd_score_prompt(args: argparse.Namespace) -> None:
    run_path = final_run_path(args.prompt_version, args.benchmark, args.run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing assembled three-shot run file: {run_path}")

    prompt_text = vl.render_worker_prompt(
        "one-shot-scoring.md",
        {
            "REPO_ROOT": vl.REPO_ROOT,
            "PROMPT_VERSION": args.prompt_version,
            "BENCHMARK": args.benchmark,
            "RUN_ID": args.run_id,
            "RUN_PATH": run_path,
            "RESULT_PATH": result_path(args.prompt_version, args.benchmark, args.run_id),
            "TRUTH_PATH": vl.truth_path(args.benchmark),
            "REPORT": vl.report_path(args.benchmark),
            "SOURCE_ROOT": vl.benchmark_source_root(args.benchmark),
            "C4_APPROVED_FINDINGS": vl.REPO_ROOT / "C4_APPROVED_FINDINGS.md",
            "APPROVED_FINDINGS_KEY": vl.REPO_ROOT / "APPROVED_FINDINGS_KEY.md",
        },
    )
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "run_path": str(run_path),
        "result_path": str(result_path(args.prompt_version, args.benchmark, args.run_id)),
        "worker_type": "three-shot-scoring",
        "worker_model": SCORING_WORKER_MODEL,
        "worker_reasoning_effort": SCORING_WORKER_REASONING,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))
    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_score_local(args: argparse.Namespace) -> None:
    print(json.dumps(score_three_shot(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def main() -> None:
    parser = argparse.ArgumentParser(description="Prepare, assemble, and score three-shot validation experiments.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    prepare_scope = subparsers.add_parser("prepare-scope", help="Initialize stage 1 scope screening and emit the worker prompt.")
    prepare_scope.add_argument("--prompt-version", default="v2")
    prepare_scope.add_argument("--benchmark", required=True)
    prepare_scope.add_argument("--run-id", default="run-001")
    prepare_scope.add_argument("--include-prompt", action="store_true")
    prepare_scope.add_argument("--write-prompt")
    prepare_scope.set_defaults(func=cmd_prepare_scope)

    prepare_token = subparsers.add_parser("prepare-token", help="Initialize stage 2 unsupported-token screening and emit the worker prompt.")
    prepare_token.add_argument("--prompt-version", default="v2")
    prepare_token.add_argument("--benchmark", required=True)
    prepare_token.add_argument("--run-id", default="run-001")
    prepare_token.add_argument("--include-prompt", action="store_true")
    prepare_token.add_argument("--write-prompt")
    prepare_token.set_defaults(func=cmd_prepare_token)

    prepare_final = subparsers.add_parser("prepare-final", help="Initialize stage 3 final validation and emit the worker prompt.")
    prepare_final.add_argument("--prompt-version", default="v2")
    prepare_final.add_argument("--benchmark", required=True)
    prepare_final.add_argument("--run-id", default="run-001")
    prepare_final.add_argument("--include-prompt", action="store_true")
    prepare_final.add_argument("--write-prompt")
    prepare_final.set_defaults(func=cmd_prepare_final)

    assemble = subparsers.add_parser("assemble", help="Assemble the final three-shot run from stages 1-3.")
    assemble.add_argument("--prompt-version", default="v2")
    assemble.add_argument("--benchmark", required=True)
    assemble.add_argument("--run-id", default="run-001")
    assemble.set_defaults(func=cmd_assemble)

    score_prompt = subparsers.add_parser("score-prompt", help="Emit the scoring worker prompt for an assembled three-shot run.")
    score_prompt.add_argument("--prompt-version", default="v2")
    score_prompt.add_argument("--benchmark", required=True)
    score_prompt.add_argument("--run-id", default="run-001")
    score_prompt.add_argument("--include-prompt", action="store_true")
    score_prompt.add_argument("--write-prompt")
    score_prompt.set_defaults(func=cmd_score_prompt)

    score_local = subparsers.add_parser("score-local", help="Locally score an assembled three-shot run.")
    score_local.add_argument("--prompt-version", default="v2")
    score_local.add_argument("--benchmark", required=True)
    score_local.add_argument("--run-id", default="run-001")
    score_local.set_defaults(func=cmd_score_local)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
