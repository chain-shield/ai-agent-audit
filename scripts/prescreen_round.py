#!/usr/bin/env python3
"""Build and score prescreen candidate sets from rigorous raw validation runs.

Prescreen is intended as a cheaper, recall-first filter before the main
severity-sensitive validation rounds. It reuses an existing rigorous
one-finding-per-worker raw run (typically `v1`) and derives a keep-set of
findings that should advance.

Important design rule:
- prescreen does NOT use a simplified worker prompt
- the worker still performs the same full validation task
- only the downstream interpretation / keep-set scoring is lighter
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

import validation_loop as vl


KEEP_RULE_DESCRIPTIONS = {
    "decision-not-invalid": "Keep any finding whose final raw decision is not `Invalid`.",
    "bug-exists-yes": "Keep any finding whose raw block says `Bug Exists: Yes`.",
    "bug-exists-not-no": "Keep any finding whose raw block does not say `Bug Exists: No`.",
}


def prescreen_dir(prompt_version: str) -> Path:
    return vl.REPO_ROOT / "validation-prescreens" / prompt_version


def prescreen_summary_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return prescreen_dir(prompt_version) / f"{benchmark}-{run_id}.md"


def prescreen_kept_json_path(prompt_version: str, benchmark: str, run_id: str) -> Path:
    return prescreen_dir(prompt_version) / f"{benchmark}-{run_id}-kept.json"


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


def parse_raw_records(prompt_version: str, benchmark: str, run_id: str) -> list[dict[str, str]]:
    blocks = vl.raw_finding_blocks(vl.raw_path(prompt_version, benchmark, run_id))
    findings = vl.parse_findings(vl.report_path(benchmark))
    records: list[dict[str, str]] = []

    for finding in findings:
        block = blocks.get(finding.fid)
        if not block:
            continue
        records.append(
            {
                "finding_id": finding.fid,
                "report_id": finding.report_id,
                "title": vl.block_field(block, "Finding Title") or finding.title,
                "decision": vl.block_field(block, "Decision"),
                "confidence": vl.block_field(block, "Confidence"),
                "bug_exists": vl.block_field(block, "Bug Exists"),
                "severity": vl.block_field(block, "Severity Assessment"),
                "root_cause_family": vl.block_field(block, "Root Cause Family"),
            }
        )

    return records


def keep_candidate(record: dict[str, str], keep_rule: str) -> bool:
    if keep_rule == "decision-not-invalid":
        return record["decision"] != "Invalid"
    if keep_rule == "bug-exists-yes":
        return record["bug_exists"] == "Yes"
    if keep_rule == "bug-exists-not-no":
        return record["bug_exists"] != "No"
    raise SystemExit(f"Unsupported keep rule: {keep_rule}")


def clean(value: str) -> str:
    value = " ".join(value.split())
    if not value:
        return "-"
    return value.replace("|", "\\|")


def build_prescreen(prompt_version: str, benchmark: str, run_id: str, keep_rule: str) -> dict[str, object]:
    raw = vl.raw_path(prompt_version, benchmark, run_id)
    if not raw.exists():
        raise SystemExit(f"Missing raw run file: {raw}")

    truth_rows = parse_truth_key()
    records = parse_raw_records(prompt_version, benchmark, run_id)
    record_map = {record["finding_id"]: record for record in records}

    hm_truth_rows = {
        fid: row
        for fid, row in truth_rows.items()
        if fid in record_map and fid.startswith(("H-", "M-"))
    }

    kept_ids: list[str] = []
    dropped_ids: list[str] = []
    kept_rows: list[dict[str, str]] = []

    tp = fp = tn = fn = 0
    kept_truth_roots: set[str] = set()
    present_truth_roots: set[str] = set()
    false_negatives: list[str] = []
    false_positives: list[str] = []

    for fid, truth_row in hm_truth_rows.items():
        record = record_map[fid]
        keep = keep_candidate(record, keep_rule)
        if keep:
            kept_ids.append(fid)
            kept_rows.append({**record, **truth_row})
        else:
            dropped_ids.append(fid)

        if truth_row["truth"] == "Valid" and truth_row["matching_c4_listing"]:
            present_truth_roots.add(truth_row["matching_c4_listing"])
            if keep:
                kept_truth_roots.add(truth_row["matching_c4_listing"])

        if truth_row["truth"] == "Valid":
            if keep:
                tp += 1
            else:
                fn += 1
                false_negatives.append(fid)
        else:
            if keep:
                fp += 1
                false_positives.append(fid)
            else:
                tn += 1

    recall = tp / (tp + fn) if (tp + fn) else 0.0
    precision = tp / (tp + fp) if (tp + fp) else 0.0
    specificity = tn / (tn + fp) if (tn + fp) else 0.0

    total_canonical = total_canonical_approved_hm_findings(benchmark)
    present_root_recall = (
        len(kept_truth_roots) / len(present_truth_roots) if present_truth_roots else 0.0
    )
    end_to_end_unique_recall = (
        len(kept_truth_roots) / total_canonical if total_canonical else None
    )

    kept_rows.sort(key=lambda row: (row["finding_id"][0], int(row["finding_id"].split("-")[1])))

    summary_path = prescreen_summary_path(prompt_version, benchmark, run_id)
    kept_json_path = prescreen_kept_json_path(prompt_version, benchmark, run_id)
    summary_path.parent.mkdir(parents=True, exist_ok=True)

    lines = [
        f"# {benchmark} Prescreen {run_id}",
        "",
        f"Source prompt version: `{prompt_version}`",
        f"Benchmark: `{benchmark}`",
        f"Run id: `{run_id}`",
        f"Source raw run: `{raw}`",
        f"Keep rule: `{keep_rule}`",
        f"Keep rule meaning: {KEEP_RULE_DESCRIPTIONS[keep_rule]}",
        "",
        "## Summary",
        "",
        "- Prescreen purpose: weed out obviously bad findings before the main validation rounds.",
        "- Prescreen-positive means the finding is kept for downstream review.",
        f"- H/M rows scored: `{len(hm_truth_rows)}`",
        f"- Kept H/M candidates: `{tp + fp}`",
        f"- Dropped H/M candidates: `{tn + fn}`",
        "",
        "## Scoring",
        "",
        f"- H/M prescreen confusion matrix: `TP={tp}`, `FP={fp}`, `TN={tn}`, `FN={fn}`",
        f"- H/M prescreen recall: `{recall:.1%}` (`{tp} / {tp + fn if (tp + fn) else 0}`)",
        f"- H/M prescreen precision: `{precision:.1%}` (`{tp} / {tp + fp if (tp + fp) else 0}`)",
        f"- H/M prescreen specificity: `{specificity:.1%}` (`{tn} / {tn + fp if (tn + fp) else 0}`)",
        f"- Unique approved H/M findings present in the report: `{len(present_truth_roots)}`",
        f"- Unique approved H/M findings kept by prescreen: `{len(kept_truth_roots)}`",
        f"- Present-root recall: `{present_root_recall:.1%}` (`{len(kept_truth_roots)} / {len(present_truth_roots) if present_truth_roots else 0}`)",
    ]

    if total_canonical is not None and end_to_end_unique_recall is not None:
        lines.extend(
            [
                f"- Total canonical approved C4 H/M findings: `{total_canonical}`",
                f"- End-to-end unique recall: `{end_to_end_unique_recall:.1%}` (`{len(kept_truth_roots)} / {total_canonical}`)",
            ]
        )

    lines.extend(
        [
            f"- Truth-valid findings dropped by prescreen: `{', '.join(false_negatives) if false_negatives else '-'}`",
            f"- Truth-invalid findings kept by prescreen: `{', '.join(false_positives) if false_positives else '-'}`",
            "",
            "## Kept Candidates",
            "",
            "| Finding | Title | Decision | Bug Exists | Severity | Truth | Matching C4 Listing | Root Cause Family |",
            "| --- | --- | --- | --- | --- | --- | --- | --- |",
        ]
    )

    if kept_rows:
        for row in kept_rows:
            lines.append(
                "| {fid} / `{report_id}` | {title} | {decision} | {bug_exists} | {severity} | {truth} | `{c4}` | `{root}` |".format(
                    fid=row["finding_id"],
                    report_id=clean(row["report_id"]),
                    title=clean(row["title"]),
                    decision=clean(row["decision"]),
                    bug_exists=clean(row["bug_exists"]),
                    severity=clean(row["severity"]),
                    truth=clean(row["truth"]),
                    c4=clean(row["matching_c4_listing"]),
                    root=clean(row["root_cause_family"]),
                )
            )
    else:
        lines.append("| - | No kept candidates. | - | - | - | - | - | - |")

    summary_path.write_text("\n".join(lines) + "\n")

    kept_payload = {
        "benchmark": benchmark,
        "run_id": run_id,
        "source_prompt_version": prompt_version,
        "source_raw_run": str(raw),
        "keep_rule": keep_rule,
        "keep_rule_description": KEEP_RULE_DESCRIPTIONS[keep_rule],
        "kept_finding_ids": kept_ids,
        "dropped_finding_ids": dropped_ids,
        "metrics": {
            "hm_rows_scored": len(hm_truth_rows),
            "tp": tp,
            "fp": fp,
            "tn": tn,
            "fn": fn,
            "recall": recall,
            "precision": precision,
            "specificity": specificity,
            "present_root_recall": present_root_recall,
            "total_canonical_approved_hm_findings": total_canonical,
            "unique_present_roots": len(present_truth_roots),
            "unique_kept_roots": len(kept_truth_roots),
            "end_to_end_unique_recall": end_to_end_unique_recall,
        },
    }
    kept_json_path.write_text(json.dumps(kept_payload, indent=2, sort_keys=True) + "\n")

    return {
        "summary_path": str(summary_path),
        "kept_json_path": str(kept_json_path),
        **kept_payload,
    }


def cmd_build(args: argparse.Namespace) -> None:
    payload = build_prescreen(args.prompt_version, args.benchmark, args.run_id, args.keep_rule)
    print(json.dumps(payload, indent=2, sort_keys=True))


def main() -> None:
    parser = argparse.ArgumentParser(description="Build and score prescreen candidate sets.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    build = subparsers.add_parser("build", help="Build prescreen artifacts from a raw run.")
    build.add_argument("--prompt-version", default="v1")
    build.add_argument("--benchmark", required=True)
    build.add_argument("--run-id", default="run-001")
    build.add_argument(
        "--keep-rule",
        choices=sorted(KEEP_RULE_DESCRIPTIONS),
        default="bug-exists-not-no",
        help="Rule for deciding which findings survive prescreen.",
    )
    build.set_defaults(func=cmd_build)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
