#!/usr/bin/env python3
"""Map invalid audit-report findings to scraped C4 rejection reasons.

This script builds a best-effort rejected-findings key that complements
APPROVED_FINDINGS_KEY.md. It keeps only report findings already treated as
Invalid by the approved key, then maps each one to the closest scraped
Code4rena low / QA / invalid submission and captures that submission's judge
rejection rationale.

The mapping is intentionally heuristic:
- there is no stable ID join between audit-report.md findings and C4 F-ids
- report titles are often paraphrased relative to the site submissions
- some mappings are exact conceptual matches while others are category-level
  exemplars

To make this usable downstream, the script emits:
- a markdown key for human review
- a JSON file with full structured metadata
"""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import asdict, dataclass
from difflib import SequenceMatcher
from pathlib import Path
from typing import Iterable

from three_shot_round import Finding, REPO_ROOT, parse_findings, report_path


OUTPUT_MD = REPO_ROOT / "C4_REJECTED_FINDINGS_KEY.md"
OUTPUT_JSON = REPO_ROOT / "C4_REJECTED_FINDINGS_KEY.json"
REJECTED_CORPUS = REPO_ROOT / "C4_LOW_QA_INVALID_FINDINGS.md"
APPROVED_KEY = REPO_ROOT / "APPROVED_FINDINGS_KEY.md"


@dataclass(frozen=True)
class ApprovedKeyRow:
    finding_id: str
    title: str
    truth: str
    matching_c4_listing: str
    reason_why: str


@dataclass(frozen=True)
class RejectedEntry:
    c4_id: str
    title: str
    severity: str
    review: str
    validity: str
    sufficiency: str
    why_marked: str
    judge_reason: str
    details: str
    category: str
    norm_title: str
    title_tokens: frozenset[str]
    combined_tokens: frozenset[str]
    reason_tokens: frozenset[str]
    keyword_terms: frozenset[str]


@dataclass(frozen=True)
class RejectedMatch:
    finding_id: str
    finding_title: str
    report_id: str
    key_reason_why: str
    key_reason_category: str
    matched_c4_id: str
    matched_c4_title: str
    matched_c4_category: str
    matched_c4_severity: str
    matched_c4_validity: str
    matched_c4_sufficiency: str
    matched_c4_review: str
    judge_rejection_reason: str
    title_overlap: float
    text_overlap: float
    reason_overlap: float
    sequence_ratio: float
    score: float
    confidence: str


@dataclass(frozen=True)
class PreparedFinding:
    finding: Finding
    key_row: ApprovedKeyRow
    norm_title: str
    title_tokens: frozenset[str]
    combined_tokens: frozenset[str]
    reason_tokens: frozenset[str]
    keyword_terms: frozenset[str]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--benchmark", required=True)
    parser.add_argument("--write-json", action="store_true")
    return parser.parse_args()


def clean_table_value(value: str) -> str:
    value = " ".join(value.split())
    if not value:
        return "-"
    return value.replace("|", "\\|")


def normalize(text: str) -> str:
    text = text.lower().replace("`", " ")
    text = text.replace("’", "'").replace("“", '"').replace("”", '"')
    text = re.sub(r"[^a-z0-9]+", " ", text)
    return re.sub(r"\s+", " ", text).strip()


STOP_WORDS = {
    "the",
    "and",
    "or",
    "of",
    "to",
    "in",
    "via",
    "due",
    "for",
    "on",
    "with",
    "a",
    "an",
    "is",
    "are",
    "be",
    "by",
    "this",
    "that",
    "if",
    "it",
    "as",
    "at",
    "from",
    "into",
    "allows",
    "allow",
    "causes",
    "cause",
    "using",
    "through",
    "because",
    "when",
    "lack",
    "missing",
    "can",
    "could",
    "would",
    "should",
    "under",
    "after",
    "before",
    "during",
    "invalid",
    "valid",
    "issue",
    "finding",
}


KEY_TERMS = [
    "usdt",
    "erc20",
    "approve",
    "bridge",
    "wormhole",
    "arbitrum",
    "assembly",
    "slippage",
    "oracle",
    "initialize",
    "signature",
    "multisig",
    "burn",
    "recovery",
    "timelock",
    "tokenomics",
    "reentrancy",
    "deadline",
    "identity",
    "balancer",
    "uniswap",
    "guardcm",
    "redeem",
    "collectfees",
    "checkpoint",
    "staking",
    "value",
]


def token_set(text: str) -> set[str]:
    return {
        token
        for token in normalize(text).split()
        if len(token) > 2 and token not in STOP_WORDS
    }


def jaccard(a: Iterable[str], b: Iterable[str]) -> float:
    set_a = set(a)
    set_b = set(b)
    if not set_a and not set_b:
        return 0.0
    return len(set_a & set_b) / len(set_a | set_b)


def classify_reason(text: str) -> str:
    norm = normalize(text)

    if any(token in norm for token in ("usdt", "weird erc20", "non standard token", "non compliant erc20", "unsupported token", "void return", "zero allowance reset")):
        return "unsupported-token"
    if any(token in norm for token in ("known v12", "known issue", "publicly known", "out of scope", " oos ", "is oos", "falls under the following oos", "not within scope")):
        return "known-issue-or-oos"
    if any(token in norm for token in ("missing mandatory poc", "insufficient mandatory poc", "provided poc does not prove", "insufficient poc", "missing poc")):
        return "missing-or-insufficient-poc"
    if any(token in norm for token in ("by design", "contract name", "not utilised", "not integrated", "intention to burn", "intended behaviour", "intended behavior")):
        return "by-design-or-not-integrated"
    if any(token in norm for token in ("not realistically incentivized", "not realistically incentivised", "no user would be incentivised", "no user would be incentivized", "unrealistic", "economically irrational", "minimum minacceptedeth", "too costly", "griefing cost", "not profit")):
        return "uneconomic-or-unrealistic"
    if any(token in norm for token in ("actual vulnerability exists within", "does not justify addressing it by adding a fix in a different part", "duplicates", "duplicate", "should be reported and resolved there", "same reasoning", "lower severity", "at most low severity")):
        return "derivative-or-lower-severity"
    if any(token in norm for token in ("deployed and initialized", "deployed and initialised", "initialization", "initialisation", "setup", "configuration", "grant role", "grant permission", "deployment")):
        return "deployment-or-setup"
    if any(token in norm for token in ("trusted", "admin", "governance", "dao", "deployer", "reckless", "sensible", "malicious admin", "malicious/reckless admin", "trusted roles")):
        return "trusted-role-or-operational"
    if any(token in norm for token in ("speculative", "not directly integrated", "not established", "future", "not proven", "assumes")):
        return "speculative-or-unproven"
    return "other"


def parse_approved_key() -> dict[str, ApprovedKeyRow]:
    rows: dict[str, ApprovedKeyRow] = {}
    for line in APPROVED_KEY.read_text().splitlines():
        if not line.startswith("| "):
            continue
        parts = [part.strip() for part in line.strip().strip("|").split("|")]
        if len(parts) < 5 or parts[0] in {"Finding", "---"}:
            continue
        fid = parts[0]
        if not re.fullmatch(r"[HML]-\d+", fid):
            continue
        rows[fid] = ApprovedKeyRow(
            finding_id=fid,
            title=parts[1],
            truth=parts[2],
            matching_c4_listing=parts[3],
            reason_why=parts[4],
        )
    return rows


def parse_rejected_entries() -> list[RejectedEntry]:
    entries: list[RejectedEntry] = []
    current: dict[str, str] | None = None
    detail_lines: list[str] = []

    def flush() -> None:
        nonlocal current, detail_lines
        if current is None:
            return
        details = "\n".join(detail_lines).strip()
        reason = current.get("judge_reason") or current.get("why_marked", "")
        title = current["title"]
        combined_text = title + " " + reason + " " + details[:1200]
        combined_tokens = frozenset(token_set(combined_text))
        reason_tokens = frozenset(token_set(reason))
        title_tokens = frozenset(token_set(title))
        keyword_terms = frozenset(term for term in KEY_TERMS if term in normalize(combined_text))
        entries.append(
            RejectedEntry(
                c4_id=current["c4_id"],
                title=title,
                severity=current.get("severity", ""),
                review=current.get("review", ""),
                validity=current.get("validity", ""),
                sufficiency=current.get("sufficiency", ""),
                why_marked=current.get("why_marked", ""),
                judge_reason=reason,
                details=details,
                category=classify_reason(reason or details or current["title"]),
                norm_title=normalize(title),
                title_tokens=title_tokens,
                combined_tokens=combined_tokens,
                reason_tokens=reason_tokens,
                keyword_terms=keyword_terms,
            )
        )
        current = None
        detail_lines = []

    for line in REJECTED_CORPUS.read_text().splitlines():
        heading = re.match(r"## \[(F-\d+)\] (.*)", line)
        if heading:
            flush()
            current = {"c4_id": heading.group(1), "title": heading.group(2).strip()}
            continue
        if current is None:
            continue
        if line.startswith("Severity:"):
            current["severity"] = line.split(":", 1)[1].strip().strip("`")
        elif line.startswith("Review:"):
            current["review"] = line.split(":", 1)[1].strip().strip("`")
        elif line.startswith("Validity:"):
            current["validity"] = line.split(":", 1)[1].strip().strip("`")
        elif line.startswith("Sufficiency:"):
            current["sufficiency"] = line.split(":", 1)[1].strip().strip("`")
        elif line.startswith("Why Marked Low/QA Or Rejected:"):
            current["why_marked"] = line.split(":", 1)[1].strip()
        elif line.startswith("Judge Rejection / Classification Reason:"):
            current["judge_reason"] = line.split(":", 1)[1].strip()
        elif line.startswith("Full Finding Details:"):
            continue
        else:
            detail_lines.append(line)

    flush()
    return entries


def category_from_key_reason(reason: str) -> str:
    return classify_reason(reason)


def prepare_finding(
    finding: Finding,
    key_row: ApprovedKeyRow,
) -> PreparedFinding:
    combined_text = finding.title + " " + finding.block[:1200]
    combined_tokens = frozenset(token_set(combined_text))
    reason_tokens = frozenset(token_set(key_row.reason_why))
    title_tokens = frozenset(token_set(finding.title))
    keyword_terms = frozenset(term for term in KEY_TERMS if term in normalize(combined_text))
    return PreparedFinding(
        finding=finding,
        key_row=key_row,
        norm_title=normalize(finding.title),
        title_tokens=title_tokens,
        combined_tokens=combined_tokens,
        reason_tokens=reason_tokens,
        keyword_terms=keyword_terms,
    )


def similarity_score(
    prepared: PreparedFinding,
    rejected: RejectedEntry,
) -> tuple[float, float, float, float]:
    title_overlap = jaccard(prepared.title_tokens, rejected.title_tokens)
    text_overlap = jaccard(prepared.combined_tokens, rejected.combined_tokens)
    reason_overlap = jaccard(prepared.reason_tokens, rejected.reason_tokens)

    sequence_ratio = SequenceMatcher(
        None,
        prepared.norm_title,
        rejected.norm_title,
    ).ratio()

    return title_overlap, text_overlap, reason_overlap, sequence_ratio


def weighted_score(
    prepared: PreparedFinding,
    rejected: RejectedEntry,
) -> tuple[float, float, float, float, float]:
    title_overlap, text_overlap, reason_overlap, sequence_ratio = similarity_score(
        prepared, rejected
    )

    score = (
        0.38 * title_overlap
        + 0.24 * text_overlap
        + 0.18 * reason_overlap
        + 0.20 * sequence_ratio
    )

    score += 0.03 * len(prepared.keyword_terms & rejected.keyword_terms)

    key_category = category_from_key_reason(prepared.key_row.reason_why)
    if key_category != "other" and key_category == rejected.category:
        score += 0.08

    return score, title_overlap, text_overlap, reason_overlap, sequence_ratio


def confidence_from_score(score: float) -> str:
    if score >= 0.34:
        return "High"
    if score >= 0.24:
        return "Medium"
    return "Low"


def best_rejected_match(
    prepared: PreparedFinding,
    rejected_entries: list[RejectedEntry],
) -> RejectedMatch:
    key_category = category_from_key_reason(prepared.key_row.reason_why)

    primary_pool = (
        [entry for entry in rejected_entries if entry.category == key_category]
        if key_category != "other"
        else rejected_entries
    )
    if not primary_pool:
        primary_pool = rejected_entries

    scored = [
        (weighted_score(prepared, entry), entry)
        for entry in primary_pool
    ]
    scored.sort(key=lambda item: item[0][0], reverse=True)
    best_score, best_entry = scored[0]

    # Fall back to the global best match if category gating hid a much stronger candidate.
    global_scored = [
        (weighted_score(prepared, entry), entry)
        for entry in rejected_entries
    ]
    global_scored.sort(key=lambda item: item[0][0], reverse=True)
    global_best_score, global_best_entry = global_scored[0]
    if best_score[0] < 0.24 or global_best_score[0] > best_score[0] + 0.03:
        best_score, best_entry = global_best_score, global_best_entry

    score, title_overlap, text_overlap, reason_overlap, sequence_ratio = best_score

    return RejectedMatch(
        finding_id=prepared.finding.fid,
        finding_title=prepared.finding.title,
        report_id=prepared.finding.report_id,
        key_reason_why=prepared.key_row.reason_why,
        key_reason_category=key_category,
        matched_c4_id=best_entry.c4_id,
        matched_c4_title=best_entry.title,
        matched_c4_category=best_entry.category,
        matched_c4_severity=best_entry.severity,
        matched_c4_validity=best_entry.validity,
        matched_c4_sufficiency=best_entry.sufficiency,
        matched_c4_review=best_entry.review,
        judge_rejection_reason=best_entry.judge_reason or best_entry.why_marked,
        title_overlap=round(title_overlap, 4),
        text_overlap=round(text_overlap, 4),
        reason_overlap=round(reason_overlap, 4),
        sequence_ratio=round(sequence_ratio, 4),
        score=round(score, 4),
        confidence=confidence_from_score(score),
    )


def build_markdown(
    benchmark: str,
    matches: list[RejectedMatch],
    category_counts: dict[str, int],
) -> str:
    lines = [
        "# C4 Rejected Findings Key",
        "",
        f"Benchmark: `{benchmark}`",
        f"Source report: `{report_path(benchmark)}`",
        f"Source approved key: `{APPROVED_KEY}`",
        f"Source rejected corpus: `{REJECTED_CORPUS}`",
        "",
        "Method used:",
        "- Keep only report findings already treated as `Invalid` in `APPROVED_FINDINGS_KEY.md`.",
        "- Map each invalid report finding to the closest scraped C4 low / invalid / QA submission.",
        "- Prefer category-consistent matches when the approved key reason already strongly implies a rejection family.",
        "- This is a best-effort heuristic mapping; it is intended to surface representative judge rejection rationales, not claim an authoritative one-to-one C4 submission identity.",
        "",
        "## Category Summary",
        "",
        "| Rejection Category | Count |",
        "| --- | ---: |",
    ]

    for category, count in sorted(category_counts.items(), key=lambda item: (-item[1], item[0])):
        lines.append(f"| `{clean_table_value(category)}` | `{count}` |")

    lines.extend(
        [
            "",
            "## Per-Finding Mapping",
            "",
            "| Finding | Finding Title | Key Invalid Basis | Rejection Category | Closest Rejected C4 Listing | Match Confidence | Judge Rejection / Classification Reason |",
            "| --- | --- | --- | --- | --- | --- | --- |",
        ]
    )

    for match in matches:
        lines.append(
            "| {finding} / `{report_id}` | {title} | {key_reason} | `{category}` | [{c4_id}](https://code4rena.com/audits/2026-01-olas/submissions/{c4_id}) - {c4_title} | `{confidence}` (`score={score}`) | {judge_reason} |".format(
                finding=match.finding_id,
                report_id=clean_table_value(match.report_id),
                title=clean_table_value(match.finding_title),
                key_reason=clean_table_value(match.key_reason_why),
                category=clean_table_value(match.matched_c4_category),
                c4_id=match.matched_c4_id,
                c4_title=clean_table_value(match.matched_c4_title),
                confidence=match.confidence,
                score=match.score,
                judge_reason=clean_table_value(match.judge_rejection_reason),
            )
        )

    return "\n".join(lines) + "\n"


def main() -> None:
    args = parse_args()
    report_findings = {
        finding.fid: finding for finding in parse_findings(report_path(args.benchmark))
    }
    approved_rows = parse_approved_key()
    rejected_entries = parse_rejected_entries()

    matches: list[RejectedMatch] = []
    for fid, key_row in approved_rows.items():
        if key_row.truth != "Invalid":
            continue
        finding = report_findings.get(fid)
        if finding is None:
            continue
        matches.append(best_rejected_match(prepare_finding(finding, key_row), rejected_entries))

    matches.sort(key=lambda match: (match.finding_id[0], int(match.finding_id.split("-")[1])))
    category_counts: dict[str, int] = {}
    for match in matches:
        category_counts[match.matched_c4_category] = (
            category_counts.get(match.matched_c4_category, 0) + 1
        )

    OUTPUT_MD.write_text(build_markdown(args.benchmark, matches, category_counts))

    if args.write_json:
        OUTPUT_JSON.write_text(
            json.dumps(
                {
                    "benchmark": args.benchmark,
                    "source_report": str(report_path(args.benchmark)),
                    "source_approved_key": str(APPROVED_KEY),
                    "source_rejected_corpus": str(REJECTED_CORPUS),
                    "matches": [asdict(match) for match in matches],
                },
                indent=2,
            )
            + "\n"
        )


if __name__ == "__main__":
    main()
