#!/usr/bin/env python3
"""Prepare and score one-shot validation experiments.

One-shot validation asks a single worker to adjudicate the entire benchmark
report in one pass using the same rigorous prompt used for per-finding raw
validation. The experiment's success condition is simple: if one worker can
hit acceptable H/M precision with very high recall, the serialized loop may
not be necessary for that benchmark class.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

import validation_loop as vl


ONE_SHOT_ROOT = vl.REPO_ROOT / "validation-one-shot"
VALIDATION_WORKER_MODEL = "gpt-5.5"
VALIDATION_WORKER_REASONING = "xhigh"
SCORING_WORKER_MODEL = "gpt-5.4"
SCORING_WORKER_REASONING = "xhigh"


def one_shot_run_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return ONE_SHOT_ROOT / "runs" / prompt_version / f"{benchmark}-{run_id}.md"


def one_shot_result_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return ONE_SHOT_ROOT / "results" / prompt_version / f"{benchmark}-{run_id}.md"


def one_shot_score_json_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return ONE_SHOT_ROOT / "results" / prompt_version / f"{benchmark}-{run_id}.json"


def parse_truth_key() -> dict[str, dict[str, str]]:
    key_path = vl.REPO_ROOT / "APPROVED_FINDINGS_KEY.md"
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
    truth_text = vl.truth_path(benchmark).read_text()
    match = re.search(
        r"total canonical approved C4 H/M findings:\s*`?(\d+)`?",
        truth_text,
    )
    return int(match.group(1)) if match else None


def init_run(prompt_version: str, benchmark: str, run_id: str, force: bool = False) -> Path:
    run_path = one_shot_run_path(prompt_version, benchmark, run_id)
    if run_path.exists() and not force:
        return run_path

    run_path.parent.mkdir(parents=True, exist_ok=True)
    source_root = vl.benchmark_source_root(benchmark)
    report = vl.report_path(benchmark)
    prompt = vl.prompt_path(prompt_version)

    content = f"""# {benchmark} One-Shot Validation {run_id}

Status: In progress
Benchmark report: `{report}`
Benchmark source root: `{source_root}`
Validation prompt: `{prompt}`

Starting benchmark docs:
- `{source_root / "README.md"}`
- `{source_root / "v12-findings.md"}`
- every `*-scope.md` under `{source_root}`
- every `*-docs.md` under `{source_root}`

Progress note:
- This file is written by exactly one worker adjudicating the full report in one pass.
- The worker should preserve the report order and emit one finding block per report finding.
- Use `python3 scripts/oneshot_round.py prepare --prompt-version {prompt_version} --benchmark {benchmark} --run-id {run_id} --write-prompt /tmp/one-shot-worker.md` to generate the worker prompt.

## Per-Finding Validation

{vl.APPEND_ANCHOR}
"""
    run_path.write_text(content)
    return run_path


def render_worker_prompt(prompt_version: str, benchmark: str, run_id: str) -> str:
    findings = vl.parse_findings(vl.report_path(benchmark))
    findings_md = "\n\n".join(
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

    return vl.render_worker_prompt(
        "one-shot-validation.md",
        {
            "REPO_ROOT": vl.REPO_ROOT,
            "BENCHMARK": benchmark,
            "REPORT": vl.report_path(benchmark),
            "SOURCE_ROOT": vl.benchmark_source_root(benchmark),
            "RAW": one_shot_run_path(prompt_version, benchmark, run_id),
            "APPEND_ANCHOR": vl.APPEND_ANCHOR,
            "PROMPT": vl.prompt_path(prompt_version),
            "README_PATH": vl.benchmark_source_root(benchmark) / "README.md",
            "OLAS_SCOPE_PATH": vl.benchmark_source_root(benchmark) / "olas-scope.md",
            "OLAS_DOCS_PATH": vl.benchmark_source_root(benchmark) / "olas-docs.md",
            "V12_FINDINGS_PATH": vl.benchmark_source_root(benchmark) / "v12-findings.md",
            "V12_CHECKLIST_PATH": vl.benchmark_source_root(benchmark) / "v12-checklist.md",
            "SCOPE_GLOB": f"{vl.benchmark_source_root(benchmark)}/*-scope.md",
            "DOCS_GLOB": f"{vl.benchmark_source_root(benchmark)}/*-docs.md",
            "ALL_FINDINGS_BLOCKS": findings_md,
        },
    )


def render_scoring_prompt(prompt_version: str, benchmark: str, run_id: str) -> str:
    return vl.render_worker_prompt(
        "one-shot-scoring.md",
        {
            "REPO_ROOT": vl.REPO_ROOT,
            "PROMPT_VERSION": prompt_version,
            "BENCHMARK": benchmark,
            "RUN_ID": run_id,
            "RUN_PATH": one_shot_run_path(prompt_version, benchmark, run_id),
            "RESULT_PATH": one_shot_result_path(prompt_version, benchmark, run_id),
            "TRUTH_PATH": vl.truth_path(benchmark),
            "REPORT": vl.report_path(benchmark),
            "SOURCE_ROOT": vl.benchmark_source_root(benchmark),
            "C4_APPROVED_FINDINGS": vl.REPO_ROOT / "C4_APPROVED_FINDINGS.md",
            "APPROVED_FINDINGS_KEY": vl.REPO_ROOT / "APPROVED_FINDINGS_KEY.md",
        },
    )


def clean(value: str) -> str:
    value = " ".join(value.split())
    if not value:
        return "-"
    return value.replace("|", "\\|")


def score_one_shot(prompt_version: str, benchmark: str, run_id: str) -> dict[str, object]:
    run_path = one_shot_run_path(prompt_version, benchmark, run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing one-shot run file: {run_path}")

    blocks = vl.raw_finding_blocks(run_path)
    findings = vl.parse_findings(vl.report_path(benchmark))
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
    total_canonical = total_canonical_approved_hm_findings(benchmark)
    present_root_recall = (
        len(accepted_truth_roots) / len(present_truth_roots) if present_truth_roots else 0.0
    )
    end_to_end_unique_recall = (
        len(accepted_truth_roots) / total_canonical if total_canonical else None
    )

    result_path = one_shot_result_path(prompt_version, benchmark, run_id)
    result_path.parent.mkdir(parents=True, exist_ok=True)

    lines = [
        f"# {benchmark} One-Shot Results {run_id}",
        "",
        f"Prompt version: `{prompt_version}`",
        f"Benchmark: `{benchmark}`",
        f"Run id: `{run_id}`",
        f"Source one-shot run: `{run_path}`",
        "",
        "## Goal Check",
        "",
        "- Target experiment: one worker validates the full report in one pass.",
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

    result_path.write_text("\n".join(lines) + "\n")

    score_json = {
        "benchmark": benchmark,
        "prompt_version": prompt_version,
        "run_id": run_id,
        "run_path": str(run_path),
        "result_path": str(result_path),
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
    one_shot_score_json_path(prompt_version, benchmark, run_id).write_text(
        json.dumps(score_json, indent=2, sort_keys=True) + "\n"
    )
    return score_json


def cmd_prepare(args: argparse.Namespace) -> None:
    run_path = init_run(args.prompt_version, args.benchmark, args.run_id, force=args.force)
    result_path = one_shot_result_path(args.prompt_version, args.benchmark, args.run_id)
    result_path.parent.mkdir(parents=True, exist_ok=True)

    prompt_text = render_worker_prompt(args.prompt_version, args.benchmark, args.run_id)
    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "run_path": str(run_path),
        "result_path": str(result_path),
        "worker_type": "one-shot-validation",
        "worker_model": VALIDATION_WORKER_MODEL,
        "worker_reasoning_effort": VALIDATION_WORKER_REASONING,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))

    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_score_prompt(args: argparse.Namespace) -> None:
    run_path = one_shot_run_path(args.prompt_version, args.benchmark, args.run_id)
    if not run_path.exists():
        raise SystemExit(f"Missing one-shot run file: {run_path}")

    result_path = one_shot_result_path(args.prompt_version, args.benchmark, args.run_id)
    result_path.parent.mkdir(parents=True, exist_ok=True)
    prompt_text = render_scoring_prompt(args.prompt_version, args.benchmark, args.run_id)

    payload = {
        "benchmark": args.benchmark,
        "prompt_version": args.prompt_version,
        "run_id": args.run_id,
        "run_path": str(run_path),
        "result_path": str(result_path),
        "worker_type": "one-shot-scoring",
        "worker_model": SCORING_WORKER_MODEL,
        "worker_reasoning_effort": SCORING_WORKER_REASONING,
        "worker_prompt": prompt_text if args.include_prompt else None,
    }
    if args.write_prompt:
        Path(args.write_prompt).write_text(prompt_text)
        payload["worker_prompt_path"] = str(Path(args.write_prompt))

    print(json.dumps(payload, indent=2, sort_keys=True))


def cmd_score_local(args: argparse.Namespace) -> None:
    print(json.dumps(score_one_shot(args.prompt_version, args.benchmark, args.run_id), indent=2, sort_keys=True))


def main() -> None:
    parser = argparse.ArgumentParser(description="Prepare and score one-shot validation experiments.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    prepare = subparsers.add_parser("prepare", help="Initialize a one-shot run and emit the worker prompt.")
    prepare.add_argument("--prompt-version", default="v1")
    prepare.add_argument("--benchmark", required=True)
    prepare.add_argument("--run-id", default="run-001")
    prepare.add_argument("--include-prompt", action="store_true")
    prepare.add_argument("--write-prompt")
    prepare.add_argument("--force", action="store_true")
    prepare.set_defaults(func=cmd_prepare)

    score_prompt = subparsers.add_parser(
        "score-prompt",
        help="Emit the scoring worker prompt for a completed one-shot run.",
    )
    score_prompt.add_argument("--prompt-version", default="v1")
    score_prompt.add_argument("--benchmark", required=True)
    score_prompt.add_argument("--run-id", default="run-001")
    score_prompt.add_argument("--include-prompt", action="store_true")
    score_prompt.add_argument("--write-prompt")
    score_prompt.set_defaults(func=cmd_score_prompt)

    score_local = subparsers.add_parser(
        "score-local",
        help="Locally score a completed one-shot run without a scoring worker.",
    )
    score_local.add_argument("--prompt-version", default="v1")
    score_local.add_argument("--benchmark", required=True)
    score_local.add_argument("--run-id", default="run-001")
    score_local.set_defaults(func=cmd_score_local)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
