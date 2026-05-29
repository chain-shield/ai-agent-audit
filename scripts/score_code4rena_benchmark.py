#!/usr/bin/env python3
"""Score AI Agent Audit benchmark runs against the archived Code4rena corpus.

This scorer is intentionally deterministic and corpus-local. It does not call an
LLM; it matches app-produced candidates to archived Code4rena accepted findings
and rejected primaries using weighted token similarity, then emits per-run JSON
and Markdown scorecards.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
import shutil
from collections import Counter, defaultdict
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]
CORPUS_ROOT = REPO_ROOT / "benchmarks" / "code4rena-corpus" / "competitions"
THREE_SHOT_ROOT = REPO_ROOT / "validation-three-shot"

FINDING_HEADER_RE = re.compile(r"^###\s+((?:H|M|L|QA|I|C)-\d+)\s+/\s+`([^`]+)`", re.MULTILINE)
MD_HM_RE = re.compile(r"^\s*#\s+\[((?:H|M)-\d+)\]\s+(.+)$", re.MULTILINE)
SOURCE_PATH_RE = re.compile(r"(?:[A-Za-z0-9_.@-]+/)*(?:contracts|src|libraries|lib|test|script|node_modules)/[A-Za-z0-9_./@+\-]+\.sol")

STAGE_ORDER = [
    "discovery_raw",
    "rust_dedup_kept",
    "rust_post_verification",
    "rust_final",
    "three_shot_r3",
    "three_shot_submission",
    "poc_created",
    "poc_verified",
    "report_created",
    "report_ready",
    "judge_accepted",
]

LOSS_STAGE_PRIORITY = [
    ("judge_accepted", "survived_end_to_end"),
    ("report_ready", "judge_simulation_reject_or_missing"),
    ("report_created", "report_review_loss"),
    ("poc_verified", "report_creation_loss"),
    ("poc_created", "poc_verification_loss"),
    ("three_shot_submission", "poc_creation_loss"),
    ("three_shot_r3", "three_shot_canonicalization_or_v12_loss"),
    ("rust_final", "three_shot_validation_loss"),
    ("rust_post_verification", "rust_final_report_omission"),
    ("rust_dedup_kept", "rust_verification_loss"),
    ("rust_dedup_dropped", "dedup_overmerge_or_drop"),
    ("discovery_raw", "dedup_or_rust_verification_loss"),
]

STOPWORDS = {
    "about",
    "after",
    "against",
    "allow",
    "allows",
    "also",
    "amount",
    "and",
    "any",
    "are",
    "because",
    "been",
    "before",
    "being",
    "between",
    "both",
    "but",
    "can",
    "cannot",
    "causes",
    "code",
    "contract",
    "contracts",
    "could",
    "current",
    "does",
    "during",
    "each",
    "external",
    "finding",
    "from",
    "function",
    "has",
    "have",
    "impact",
    "into",
    "issue",
    "its",
    "later",
    "line",
    "lines",
    "make",
    "may",
    "more",
    "not",
    "only",
    "path",
    "protocol",
    "revert",
    "same",
    "set",
    "should",
    "than",
    "that",
    "the",
    "their",
    "then",
    "there",
    "this",
    "through",
    "when",
    "where",
    "which",
    "while",
    "with",
    "within",
    "without",
}


@dataclass
class ReferenceFinding:
    id: str
    kind: str
    title: str
    severity: str
    status: str
    text: str
    path: str
    rejection_category: str = ""
    source_url: str = ""


@dataclass
class Candidate:
    id: str
    title: str = ""
    text: str = ""
    severity: str = ""
    status: str = ""
    source_id: str = ""
    stages: set[str] = field(default_factory=set)
    stage_details: dict[str, dict[str, Any]] = field(default_factory=dict)

    def add_stage(self, stage: str, payload: dict[str, Any] | None = None) -> None:
        self.stages.add(stage)
        self.stage_details[stage] = payload or {}

    @property
    def best_stage(self) -> str | None:
        present = [stage for stage in STAGE_ORDER if stage in self.stages]
        return present[-1] if present else None


def read_json(path: Path) -> Any:
    with path.open() as file:
        return json.load(file)


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    rows: list[dict[str, Any]] = []
    with path.open() as file:
        for line in file:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


def compact_text(value: object) -> str:
    if value is None:
        return ""
    if isinstance(value, (list, tuple, set)):
        return " ".join(compact_text(item) for item in value)
    if isinstance(value, dict):
        return " ".join(compact_text(item) for item in value.values())
    return str(value)


def text_from_finding(payload: dict[str, Any]) -> str:
    fields = [
        payload.get("title"),
        payload.get("contract"),
        payload.get("function"),
        payload.get("exploit_type"),
        payload.get("derived_from"),
        payload.get("description"),
        payload.get("impact"),
        payload.get("proof_of_concept"),
        payload.get("status_justification"),
        payload.get("notes"),
    ]
    return "\n".join(part for part in (compact_text(field) for field in fields) if part)


def token_parts(raw: str) -> list[str]:
    parts: list[str] = []
    for token in re.findall(r"[A-Za-z0-9_]{2,}", raw):
        parts.append(token)
        split = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", token).replace("_", " ")
        parts.extend(split.split())
    return parts


def tokenize(raw: str) -> list[str]:
    tokens: list[str] = []
    for token in token_parts(raw):
        normalized = token.lower()
        if len(normalized) < 3:
            continue
        if normalized in STOPWORDS:
            continue
        if normalized.isdigit():
            continue
        tokens.append(normalized)
    return tokens


def vector(raw: str, limit: int = 900) -> Counter[str]:
    return Counter(tokenize(raw)[:limit])


def cosine(left: Counter[str], right: Counter[str]) -> float:
    if not left or not right:
        return 0.0
    overlap = set(left) & set(right)
    dot = sum(left[token] * right[token] for token in overlap)
    left_norm = math.sqrt(sum(value * value for value in left.values()))
    right_norm = math.sqrt(sum(value * value for value in right.values()))
    if not left_norm or not right_norm:
        return 0.0
    return dot / (left_norm * right_norm)


def jaccard(left: set[str], right: set[str]) -> float:
    if not left or not right:
        return 0.0
    return len(left & right) / len(left | right)


def similarity(candidate: Candidate, reference: ReferenceFinding) -> dict[str, float]:
    candidate_title = vector(candidate.title, limit=80)
    reference_title = vector(reference.title, limit=80)
    candidate_body = vector(candidate.text, limit=1200)
    reference_body = vector(reference.text, limit=1200)
    candidate_short = vector(f"{candidate.title}\n{candidate.text[:2200]}", limit=500)
    reference_short = vector(f"{reference.title}\n{reference.text[:2200]}", limit=500)
    entity_score = jaccard(set(tokenize(candidate.title + " " + candidate.text)[:120]), set(tokenize(reference.title + " " + reference.text)[:120]))
    title_score = cosine(candidate_title, reference_title)
    body_score = cosine(candidate_body, reference_body)
    short_score = cosine(candidate_short, reference_short)
    score = (0.35 * title_score) + (0.40 * body_score) + (0.20 * short_score) + (0.05 * entity_score)
    return {
        "score": score,
        "title": title_score,
        "body": body_score,
        "short": short_score,
        "entity": entity_score,
    }


def best_match(candidate: Candidate, references: list[ReferenceFinding]) -> tuple[ReferenceFinding | None, dict[str, float]]:
    best_ref: ReferenceFinding | None = None
    best_scores: dict[str, float] = {"score": 0.0}
    for reference in references:
        scores = similarity(candidate, reference)
        if scores["score"] > best_scores["score"]:
            best_ref = reference
            best_scores = scores
    return best_ref, best_scores


def load_accepted_references(competition_dir: Path) -> list[ReferenceFinding]:
    final_findings_path = competition_dir / "final_findings.json"
    references: list[ReferenceFinding] = []

    if final_findings_path.exists():
        for row in read_json(final_findings_path):
            local_path = competition_dir.parent.parent / row.get("local_markdown_path", "")
            text = row.get("raw_excerpt", "")
            if local_path.exists():
                text = local_path.read_text(errors="replace")
            severity = row.get("severity", "")
            if severity not in {"High", "Medium"}:
                continue
            references.append(
                ReferenceFinding(
                    id=row.get("id", ""),
                    kind="accepted",
                    title=row.get("title", ""),
                    severity=severity,
                    status="accepted",
                    text=text,
                    path=str(local_path if local_path.exists() else final_findings_path),
                    source_url=row.get("source_url", ""),
                )
            )
        return references

    accepted_dir = competition_dir / "ground_truth" / "accepted"
    for path in sorted(accepted_dir.glob("*.md")):
        text = path.read_text(errors="replace")
        match = MD_HM_RE.search(text)
        fid = path.stem
        title = match.group(2).strip() if match else fid
        severity = "High" if fid.startswith("H-") else "Medium"
        references.append(
            ReferenceFinding(
                id=fid,
                kind="accepted",
                title=title,
                severity=severity,
                status="accepted",
                text=text,
                path=str(path),
            )
        )
    return references


def load_rejected_references(competition_dir: Path) -> list[ReferenceFinding]:
    path = competition_dir / "submissions" / "rejected_primaries.jsonl"
    references: list[ReferenceFinding] = []
    for row in read_jsonl(path):
        title = row.get("title") or row.get("raw_row", {}).get("title", "")
        local_md = competition_dir.parent.parent / row.get("local_markdown_path", "")
        text = "\n".join(
            part
            for part in [
                title,
                row.get("brief_summary", ""),
                row.get("rejection_reason_raw", ""),
                row.get("sponsor_comment", ""),
                row.get("judge_comment", ""),
                row.get("warden_comment", ""),
            ]
            if part
        )
        if local_md.exists():
            text += "\n" + local_md.read_text(errors="replace")
        references.append(
            ReferenceFinding(
                id=row.get("submission_id", ""),
                kind="rejected",
                title=title,
                severity=row.get("final_severity") or row.get("claimed_severity", ""),
                status=row.get("status", ""),
                text=text,
                path=str(local_md if local_md.exists() else path),
                rejection_category=row.get("rejection_reason_category", ""),
                source_url=row.get("source_url") or row.get("source", {}).get("url", ""),
            )
        )
    return references


def normalize_source_path(raw: str) -> str:
    value = raw.strip().strip("`'\"),.;:")
    value = re.sub(r"^https?://github\.com/[^/]+/[^/]+/(?:blob|tree)/[^/]+/", "", value)
    for anchor in ["/contracts/", "/src/", "/libraries/", "/lib/", "/test/", "/script/", "/node_modules/"]:
        if anchor in value:
            return anchor.strip("/") + "/" + value.split(anchor, 1)[1]
    return value.lstrip("./")


def extract_source_paths(text: str) -> list[str]:
    paths = {normalize_source_path(match.group(0)) for match in SOURCE_PATH_RE.finditer(text or "")}
    return sorted(path for path in paths if path.endswith(".sol"))


def source_path_present(path: str, included_files: set[str]) -> bool:
    normalized = normalize_source_path(path)
    for included in included_files:
        if included == normalized or included.endswith("/" + normalized) or normalized.endswith("/" + included):
            return True
    return False


def load_context_index(run_dir: Path) -> dict[str, Any]:
    rows = read_jsonl(run_dir / "codeblock_manifest.jsonl")
    included_files: set[str] = set()
    contracts: dict[str, dict[str, Any]] = {}
    budget_exceeded_contracts: list[str] = []
    for record in rows:
        payload = record.get("payload", {})
        contract = compact_text(payload.get("contract"))
        files = {normalize_source_path(path) for path in payload.get("included_files", [])}
        included_files.update(files)
        if payload.get("budget_exceeded"):
            budget_exceeded_contracts.append(contract)
        contracts[contract] = {
            "included_files": sorted(files),
            "final_token_count": payload.get("final_token_count"),
            "budget_exceeded": bool(payload.get("budget_exceeded")),
            "traversal_mode": payload.get("traversal_mode"),
            "slither_available": payload.get("slither_available"),
            "counts": payload.get("counts", {}),
        }
    return {
        "manifest_rows": len(rows),
        "included_files": sorted(included_files),
        "contracts": contracts,
        "budget_exceeded_contracts": sorted(contract for contract in budget_exceeded_contracts if contract),
    }


def context_coverage(reference: ReferenceFinding, context_index: dict[str, Any]) -> dict[str, Any]:
    source_paths = extract_source_paths(f"{reference.source_url}\n{reference.text}")
    included_files = set(context_index.get("included_files", []))
    present = [path for path in source_paths if source_path_present(path, included_files)]
    missing = [path for path in source_paths if path not in present]
    if not source_paths:
        status = "no_source_paths_extracted"
    elif not missing:
        status = "all_referenced_files_present"
    elif present:
        status = "some_referenced_files_missing"
    else:
        status = "referenced_files_absent"
    return {
        "status": status,
        "referenced_files": source_paths,
        "present_files": present,
        "missing_files": missing,
    }


def candidate_key(candidate_id: str, source_id: str = "") -> str:
    return source_id or candidate_id


def upsert_candidate(candidates: dict[str, Candidate], candidate_id: str, stage: str, payload: dict[str, Any], source_id: str = "") -> Candidate:
    key = candidate_key(candidate_id, source_id)
    candidate = candidates.get(key)
    if candidate is None:
        candidate = Candidate(id=key, source_id=source_id)
        candidates[key] = candidate
    candidate.title = candidate.title or compact_text(payload.get("title") or payload.get("Finding Title"))
    candidate.text = candidate.text or text_from_finding(payload)
    candidate.severity = candidate.severity or compact_text(payload.get("severity") or payload.get("Severity Assessment"))
    candidate.status = candidate.status or compact_text(payload.get("status") or payload.get("Decision"))
    candidate.add_stage(stage, payload)
    return candidate


def load_rust_candidates(run_dir: Path) -> dict[str, Candidate]:
    candidates: dict[str, Candidate] = {}

    for record in read_jsonl(run_dir / "raw_candidates.jsonl"):
        for finding in record.get("payload", {}).get("findings", []):
            upsert_candidate(candidates, finding.get("id", ""), "discovery_raw", finding)

    for record in read_jsonl(run_dir / "dedup_clusters.jsonl"):
        stage_name = record.get("payload", {}).get("stage", "dedup")
        for cluster in record.get("payload", {}).get("clusters", []):
            for finding in cluster.get("kept", []):
                upsert_candidate(candidates, finding.get("id", ""), "rust_dedup_kept", finding).stage_details["rust_dedup_kept"]["dedup_stage"] = stage_name
            for finding in cluster.get("dropped", []):
                upsert_candidate(candidates, finding.get("id", ""), "rust_dedup_dropped", finding).stage_details["rust_dedup_dropped"]["dedup_stage"] = stage_name

    for record in read_jsonl(run_dir / "finding_lifecycle.jsonl"):
        payload = record.get("payload", {})
        stage = payload.get("stage")
        if stage == "post_verification_retained":
            upsert_candidate(candidates, payload.get("entity_id", ""), "rust_post_verification", payload)
        elif stage == "final_report_candidate":
            upsert_candidate(candidates, payload.get("entity_id", ""), "rust_final", payload)

    for record in read_jsonl(run_dir / "final_candidates.jsonl"):
        for finding in record.get("payload", {}).get("findings", []):
            upsert_candidate(candidates, finding.get("id", ""), "rust_final", finding)

    return candidates


def raw_finding_blocks(path: Path) -> dict[str, tuple[str, str]]:
    if not path.exists():
        return {}
    text = path.read_text(errors="replace")
    matches = list(FINDING_HEADER_RE.finditer(text))
    blocks: dict[str, tuple[str, str]] = {}
    for idx, match in enumerate(matches):
        end = matches[idx + 1].start() if idx + 1 < len(matches) else len(text)
        blocks[match.group(1)] = (match.group(2), text[match.start() : end].strip())
    return blocks


def block_field(block: str, label: str) -> str:
    prefix = f"- {label}: "
    for line in block.splitlines():
        if line.startswith(prefix):
            return line[len(prefix) :].strip().strip("`")
    return ""


def load_three_shot_candidates(
    candidates: dict[str, Candidate],
    benchmark: str,
    run_id: str,
    prompt_version: str,
) -> dict[str, str]:
    source_by_finding: dict[str, str] = {}

    for stage, path in [
        ("three_shot_r3", THREE_SHOT_ROOT / "runs" / prompt_version / f"{benchmark}-{run_id}.md"),
        ("three_shot_submission", THREE_SHOT_ROOT / "submission-candidates" / prompt_version / f"{benchmark}-{run_id}.md"),
    ]:
        for finding_id, (source_id, block) in raw_finding_blocks(path).items():
            source_by_finding[finding_id] = source_id
            payload = {
                "id": finding_id,
                "source_id": source_id,
                "title": block_field(block, "Finding Title"),
                "severity": block_field(block, "Severity Assessment"),
                "status": block_field(block, "Decision"),
                "description": block,
            }
            upsert_candidate(candidates, finding_id, stage, payload, source_id=source_id)

    for stage, path in [
        ("poc_created", THREE_SHOT_ROOT / "poc-runs" / prompt_version / f"{benchmark}-{run_id}.json"),
        ("poc_verified", THREE_SHOT_ROOT / "poc-verification" / prompt_version / f"{benchmark}-{run_id}.json"),
    ]:
        if not path.exists():
            continue
        for row in read_json(path).get("results", []):
            finding_id = row.get("finding_id", "")
            upsert_candidate(candidates, finding_id, stage, row, source_id=source_by_finding.get(finding_id, ""))

    report_dir = THREE_SHOT_ROOT / "finding-reports" / prompt_version / f"{benchmark}-{run_id}"
    for json_path in sorted(report_dir.glob("*.json")):
        row = read_json(json_path)
        finding_id = row.get("finding_id") or json_path.stem
        report_md = Path(row.get("report_path", ""))
        if not report_md.exists():
            md_candidates = sorted(report_dir.glob(f"{finding_id}-*.md"))
            report_md = md_candidates[0] if md_candidates else Path()
        if report_md.exists():
            row["description"] = report_md.read_text(errors="replace")
        upsert_candidate(candidates, finding_id, "report_created", row, source_id=source_by_finding.get(finding_id, ""))

    review_dir = THREE_SHOT_ROOT / "finding-report-reviews" / prompt_version / f"{benchmark}-{run_id}"
    for json_path in sorted(review_dir.glob("*.json")):
        row = read_json(json_path)
        finding_id = row.get("finding_id") or json_path.stem
        status = str(row.get("status", "")).lower()
        stage = "report_ready" if "ready" in status and "need" not in status else "report_review_failed"
        upsert_candidate(candidates, finding_id, stage, row, source_id=source_by_finding.get(finding_id, ""))

    judge_dir = THREE_SHOT_ROOT / "judge-simulations" / prompt_version / f"{benchmark}-{run_id}"
    for json_path in sorted(judge_dir.glob("*.json")):
        row = read_json(json_path)
        finding_id = row.get("finding_id") or json_path.stem
        decision = str(row.get("judge_decision", "")).lower()
        stage = "judge_accepted" if decision == "accepted" else "judge_rejected"
        payload = {
            **row,
            "title": row.get("finding_id", finding_id),
            "severity": row.get("judge_severity", ""),
            "status": row.get("judge_decision", ""),
            "description": compact_text(row),
        }
        upsert_candidate(candidates, finding_id, stage, payload, source_id=source_by_finding.get(finding_id, ""))

    return source_by_finding


def match_candidates_to_references(
    candidates: list[Candidate],
    references: list[ReferenceFinding],
    threshold: float,
) -> dict[str, dict[str, Any]]:
    matches: dict[str, dict[str, Any]] = {}
    for candidate in candidates:
        reference, scores = best_match(candidate, references)
        if reference is None or scores["score"] < threshold:
            continue
        current = matches.get(reference.id)
        if current is None or scores["score"] > current["score"]:
            matches[reference.id] = {
                "reference": reference,
                "candidate": candidate,
                "score": scores["score"],
                "scores": scores,
            }
    return matches


def stage_recall(matches_by_stage: dict[str, set[str]], accepted_total: int) -> dict[str, float]:
    if not accepted_total:
        return {stage: 0.0 for stage in STAGE_ORDER}
    return {
        stage: len(matches_by_stage.get(stage, set())) / accepted_total
        for stage in STAGE_ORDER
    }


def format_pct(value: float | None) -> str:
    if value is None:
        return "n/a"
    return f"{value:.1%}"


def match_confidence(score: float, threshold: float) -> str:
    if score >= max(0.42, threshold + 0.15):
        return "high"
    if score >= threshold + 0.06:
        return "medium"
    if score >= threshold:
        return "low"
    return "below_threshold"


def classify_loss(matched_stages: set[str]) -> str:
    for stage, label in LOSS_STAGE_PRIORITY:
        if stage in matched_stages:
            return label
    return "context_or_discovery_miss"


def score_run(args: argparse.Namespace) -> dict[str, Any]:
    competition_dir = CORPUS_ROOT / args.slug
    if not competition_dir.exists():
        raise SystemExit(f"Missing Code4rena corpus competition directory: {competition_dir}")

    accepted_refs = load_accepted_references(competition_dir)
    rejected_refs = load_rejected_references(competition_dir)
    candidates_by_id = load_rust_candidates(Path(args.run_dir))
    load_three_shot_candidates(candidates_by_id, args.three_shot_benchmark or args.slug, args.three_shot_run_id, args.prompt_version)
    candidates = list(candidates_by_id.values())

    accepted_matches_by_stage: dict[str, set[str]] = defaultdict(set)
    accepted_root_rows: list[dict[str, Any]] = []
    all_accepted_matches: dict[str, dict[str, Any]] = {}

    for root in accepted_refs:
        highest_score_candidate: Candidate | None = None
        highest_scores: dict[str, float] = {"score": 0.0}
        deepest_candidate: Candidate | None = None
        deepest_scores: dict[str, float] = {"score": 0.0}
        deepest_stage_index = -1
        matched_stages: set[str] = set()
        stage_hits: dict[str, dict[str, Any]] = {}
        for candidate in candidates:
            scores = similarity(candidate, root)
            if scores["score"] >= args.accepted_threshold:
                matched_stages.update(candidate.stages)
                for stage in candidate.stages:
                    if stage in STAGE_ORDER:
                        accepted_matches_by_stage[stage].add(root.id)
                stage_hits[candidate.id] = {
                    "candidate_id": candidate.id,
                    "title": candidate.title,
                    "score": scores["score"],
                    "best_stage": candidate.best_stage,
                    "stages": sorted(candidate.stages, key=lambda item: STAGE_ORDER.index(item) if item in STAGE_ORDER else 999),
                }
                candidate_stage = candidate.best_stage
                candidate_stage_index = STAGE_ORDER.index(candidate_stage) if candidate_stage in STAGE_ORDER else -1
                if candidate_stage_index > deepest_stage_index or (
                    candidate_stage_index == deepest_stage_index and scores["score"] > deepest_scores["score"]
                ):
                    deepest_candidate = candidate
                    deepest_scores = scores
                    deepest_stage_index = candidate_stage_index
            if scores["score"] > highest_scores["score"]:
                highest_score_candidate = candidate
                highest_scores = scores

        best_stage = None
        if deepest_candidate:
            best_stage = deepest_candidate.best_stage
            all_accepted_matches[root.id] = {
                "reference": root,
                "candidate": deepest_candidate,
                "score": deepest_scores["score"],
                "scores": deepest_scores,
            }

        accepted_root_rows.append(
            {
                "id": root.id,
                "severity": root.severity,
                "title": root.title,
                "best_candidate_id": deepest_candidate.id if deepest_candidate else None,
                "best_candidate_title": deepest_candidate.title if deepest_candidate else None,
                "best_stage": best_stage,
                "match_score": deepest_scores["score"] if deepest_candidate else highest_scores["score"],
                "match_confidence": match_confidence(
                    deepest_scores["score"] if deepest_candidate else highest_scores["score"],
                    args.accepted_threshold,
                ),
                "highest_score_candidate_id": highest_score_candidate.id if highest_score_candidate else None,
                "highest_score_candidate_title": highest_score_candidate.title if highest_score_candidate else None,
                "highest_match_score": highest_scores["score"],
                "loss_stage": classify_loss(matched_stages),
                "matched_stages": sorted(
                    matched_stages,
                    key=lambda item: STAGE_ORDER.index(item) if item in STAGE_ORDER else 999,
                ),
                "stage_hits": list(stage_hits.values())[:5],
            }
        )

    final_stage_candidates = [
        candidate
        for candidate in candidates
        if "judge_accepted" in candidate.stages or "report_ready" in candidate.stages or "three_shot_submission" in candidate.stages
    ]

    accepted_by_candidate: dict[str, tuple[ReferenceFinding, dict[str, float]]] = {}
    for candidate in final_stage_candidates:
        ref, scores = best_match(candidate, accepted_refs)
        if ref and scores["score"] >= args.accepted_threshold:
            accepted_by_candidate[candidate.id] = (ref, scores)

    false_positive_rows: list[dict[str, Any]] = []
    for candidate in final_stage_candidates:
        if candidate.id in accepted_by_candidate:
            continue
        rejected_ref, rejected_scores = best_match(candidate, rejected_refs)
        false_positive_rows.append(
            {
                "candidate_id": candidate.id,
                "title": candidate.title,
                "severity": candidate.severity,
                "best_stage": candidate.best_stage,
                "stages": sorted(candidate.stages, key=lambda item: STAGE_ORDER.index(item) if item in STAGE_ORDER else 999),
                "classification": "matched_rejected_primary" if rejected_ref and rejected_scores["score"] >= args.rejected_threshold else "unmatched_unadjudicated",
                "rejected_submission_id": rejected_ref.id if rejected_ref and rejected_scores["score"] >= args.rejected_threshold else None,
                "rejected_title": rejected_ref.title if rejected_ref and rejected_scores["score"] >= args.rejected_threshold else None,
                "rejection_category": rejected_ref.rejection_category if rejected_ref and rejected_scores["score"] >= args.rejected_threshold else None,
                "rejected_status": rejected_ref.status if rejected_ref and rejected_scores["score"] >= args.rejected_threshold else None,
                "match_score": rejected_scores["score"] if rejected_ref else 0.0,
                "match_confidence": match_confidence(rejected_scores["score"], args.rejected_threshold) if rejected_ref else "below_threshold",
            }
        )

    stage_recalls = stage_recall(accepted_matches_by_stage, len(accepted_refs))
    judge_tp_roots = accepted_matches_by_stage.get("judge_accepted", set())
    report_ready_tp_roots = accepted_matches_by_stage.get("report_ready", set())
    final_tp_roots = judge_tp_roots or report_ready_tp_roots or accepted_matches_by_stage.get("three_shot_submission", set())
    final_candidates_count = len(final_stage_candidates)
    final_precision = len(final_tp_roots) / final_candidates_count if final_candidates_count else None
    judge_candidates_count = sum(1 for candidate in candidates if "judge_accepted" in candidate.stages or "judge_rejected" in candidate.stages)
    judge_precision = len(judge_tp_roots) / judge_candidates_count if judge_candidates_count else None

    loss_counts = Counter(row["loss_stage"] for row in accepted_root_rows)
    duplicate_final_groups = defaultdict(list)
    for candidate_id, (ref, scores) in accepted_by_candidate.items():
        duplicate_final_groups[ref.id].append({"candidate_id": candidate_id, "score": scores["score"]})
    undermerge = {root_id: rows for root_id, rows in duplicate_final_groups.items() if len(rows) > 1}

    payload = {
        "schema_version": "code4rena-benchmark-score-v1",
        "slug": args.slug,
        "run_dir": str(Path(args.run_dir).resolve()),
        "three_shot_benchmark": args.three_shot_benchmark or args.slug,
        "three_shot_run_id": args.three_shot_run_id,
        "prompt_version": args.prompt_version,
        "thresholds": {
            "accepted": args.accepted_threshold,
            "rejected": args.rejected_threshold,
        },
        "ground_truth": {
            "accepted_hm_total": len(accepted_refs),
            "rejected_primary_total": len(rejected_refs),
        },
        "candidate_counts": {
            "all_unique_candidates": len(candidates),
            **{stage: sum(1 for candidate in candidates if stage in candidate.stages) for stage in STAGE_ORDER},
            "final_stage_candidates": final_candidates_count,
            "judge_candidates": judge_candidates_count,
        },
        "metrics": {
            "stage_recall": stage_recalls,
            "accepted_hm_judge_matched": len(judge_tp_roots),
            "accepted_hm_report_ready_matched": len(report_ready_tp_roots),
            "accepted_hm_rust_final_matched": len(accepted_matches_by_stage.get("rust_final", set())),
            "accepted_hm_judge_recall": stage_recalls.get("judge_accepted", 0.0),
            "accepted_hm_report_ready_recall": stage_recalls.get("report_ready", 0.0),
            "accepted_hm_rust_final_recall": stage_recalls.get("rust_final", 0.0),
            "final_precision": final_precision,
            "judge_precision": judge_precision,
            "false_positive_count": len(false_positive_rows),
            "matched_rejected_fp_count": sum(1 for row in false_positive_rows if row["classification"] == "matched_rejected_primary"),
            "unmatched_unadjudicated_fp_count": sum(1 for row in false_positive_rows if row["classification"] == "unmatched_unadjudicated"),
            "dedup_undermerge_groups": len(undermerge),
        },
        "loss_counts": dict(sorted(loss_counts.items())),
        "accepted_roots": accepted_root_rows,
        "false_positives": false_positive_rows,
        "dedup_undermerge": undermerge,
    }

    out_dir = Path(args.out_dir) if args.out_dir else Path(args.run_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    json_path = out_dir / "benchmark_score.json"
    md_path = out_dir / "benchmark_score.md"
    json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    md_path.write_text(render_score_markdown(payload) + "\n")
    payload["output_paths"] = {"json": str(json_path), "markdown": str(md_path)}
    return payload


def render_score_markdown(payload: dict[str, Any]) -> str:
    metrics = payload["metrics"]
    counts = payload["candidate_counts"]
    truth = payload["ground_truth"]
    accepted_total = truth["accepted_hm_total"]
    lines = [
        f"# Benchmark Score: {payload['slug']}",
        "",
        "## Headline Recall",
        "",
        f"- Rust final H/M recall: `{metrics.get('accepted_hm_rust_final_matched', 0)} / {accepted_total}` (`{format_pct(metrics['accepted_hm_rust_final_recall'])}`)",
        f"- End-to-end three-shot/judge-sim H/M recall: `{metrics.get('accepted_hm_judge_matched', 0)} / {accepted_total}` (`{format_pct(metrics['accepted_hm_judge_recall'])}`)",
        "",
        f"- Run dir: `{payload['run_dir']}`",
        f"- Three-shot run: `{payload['three_shot_benchmark']}-{payload['three_shot_run_id']}`",
        f"- Accepted H/M ground truth roots: `{accepted_total}`",
        f"- Rejected primary references: `{truth['rejected_primary_total']}`",
        "",
        "## Summary Metrics",
        "",
        f"- Rust final accepted-root recall: `{format_pct(metrics['accepted_hm_rust_final_recall'])}`",
        f"- Report-ready accepted-root recall: `{format_pct(metrics['accepted_hm_report_ready_recall'])}`",
        f"- Judge-accepted end-to-end recall: `{format_pct(metrics['accepted_hm_judge_recall'])}`",
        f"- Final-stage precision: `{format_pct(metrics['final_precision'])}`",
        f"- Judge-stage precision: `{format_pct(metrics['judge_precision'])}`",
        f"- Final-stage false positives: `{metrics['false_positive_count']}`",
        f"- Matched rejected-primary FPs: `{metrics['matched_rejected_fp_count']}`",
        f"- Unadjudicated final-stage candidates: `{metrics['unmatched_unadjudicated_fp_count']}`",
        f"- Dedup under-merge groups: `{metrics['dedup_undermerge_groups']}`",
        "",
        "## Stage Counts",
        "",
        "| Stage | Candidates | Accepted-Root Recall |",
        "| --- | ---: | ---: |",
    ]
    for stage in STAGE_ORDER:
        lines.append(f"| `{stage}` | {counts.get(stage, 0)} | {format_pct(metrics['stage_recall'].get(stage, 0.0))} |")

    lines.extend(["", "## Accepted Root Outcomes", "", "| Root | Severity | Best Stage | Score | Confidence | Loss Stage | Title |", "| --- | --- | --- | ---: | --- | --- | --- |"])
    for row in payload["accepted_roots"]:
        lines.append(
            "| `{id}` | {severity} | `{stage}` | {score:.3f} | `{confidence}` | `{loss}` | {title} |".format(
                id=row["id"],
                severity=row["severity"],
                stage=row["best_stage"] or "-",
                score=row["match_score"],
                confidence=row["match_confidence"],
                loss=row["loss_stage"],
                title=clean_md(row["title"]),
            )
        )

    lines.extend(["", "## False Positives", "", "| Candidate | Best Stage | Classification | Match | Confidence | Rejection | Title |", "| --- | --- | --- | ---: | --- | --- | --- |"])
    for row in payload["false_positives"]:
        rejection = row["rejected_submission_id"] or "-"
        if row["rejection_category"]:
            rejection = f"{rejection} / {row['rejection_category']}"
        lines.append(
            "| `{candidate}` | `{stage}` | `{classification}` | {score:.3f} | `{confidence}` | `{rejection}` | {title} |".format(
                candidate=row["candidate_id"],
                stage=row["best_stage"] or "-",
                classification=row["classification"],
                score=row["match_score"],
                confidence=row["match_confidence"],
                rejection=clean_md(rejection),
                title=clean_md(row["title"]),
            )
        )

    lines.extend(["", "## Loss Counts", "", "| Loss Stage | Accepted Roots |", "| --- | ---: |"])
    for loss, count in payload["loss_counts"].items():
        lines.append(f"| `{loss}` | {count} |")

    return "\n".join(lines)


def clean_md(value: object) -> str:
    text = compact_text(value)
    text = " ".join(text.split())
    return text.replace("|", "\\|") or "-"


def compare_runs(args: argparse.Namespace) -> dict[str, Any]:
    score_paths = [Path(path) for path in args.score]
    rows = []
    for path in score_paths:
        payload = read_json(path)
        rows.append(
            {
                "score_path": str(path),
                "slug": payload["slug"],
                "run_dir": payload["run_dir"],
                "three_shot_run_id": payload["three_shot_run_id"],
                "metrics": payload["metrics"],
                "candidate_counts": payload["candidate_counts"],
                "loss_counts": payload["loss_counts"],
            }
        )

    baseline = rows[0] if rows else None
    comparisons = []
    if baseline:
        for row in rows[1:]:
            comparisons.append(
                {
                    "score_path": row["score_path"],
                    "against": baseline["score_path"],
                    "delta": metric_delta(row["metrics"], baseline["metrics"]),
                }
            )

    payload = {
        "schema_version": "code4rena-benchmark-compare-v1",
        "baseline": baseline["score_path"] if baseline else None,
        "runs": rows,
        "comparisons": comparisons,
    }

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    json_path = out_dir / "benchmark_compare.json"
    md_path = out_dir / "benchmark_compare.md"
    json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    md_path.write_text(render_compare_markdown(payload) + "\n")
    payload["output_paths"] = {"json": str(json_path), "markdown": str(md_path)}
    return payload


def metric_delta(current: dict[str, Any], baseline: dict[str, Any]) -> dict[str, Any]:
    keys = [
        "accepted_hm_judge_recall",
        "accepted_hm_report_ready_recall",
        "accepted_hm_rust_final_recall",
        "final_precision",
        "judge_precision",
        "false_positive_count",
        "matched_rejected_fp_count",
        "dedup_undermerge_groups",
    ]
    delta = {}
    for key in keys:
        cur = current.get(key)
        base = baseline.get(key)
        if cur is None or base is None:
            delta[key] = None
        else:
            delta[key] = cur - base
    return delta


def render_compare_markdown(payload: dict[str, Any]) -> str:
    lines = ["# Benchmark Run Comparison", ""]
    if payload["baseline"]:
        lines.append(f"Baseline: `{payload['baseline']}`")
        lines.append("")
    lines.extend(["| Score | Judge Recall | Report Recall | Final Precision | FPs | Under-Merge |", "| --- | ---: | ---: | ---: | ---: | ---: |"])
    for row in payload["runs"]:
        metrics = row["metrics"]
        lines.append(
            "| `{score}` | {judge} | {report} | {precision} | {fps} | {undermerge} |".format(
                score=row["score_path"],
                judge=format_pct(metrics.get("accepted_hm_judge_recall")),
                report=format_pct(metrics.get("accepted_hm_report_ready_recall")),
                precision=format_pct(metrics.get("final_precision")),
                fps=metrics.get("false_positive_count"),
                undermerge=metrics.get("dedup_undermerge_groups"),
            )
        )
    if payload["comparisons"]:
        lines.extend(["", "## Deltas Versus Baseline", "", "| Score | Judge Recall Δ | Report Recall Δ | Precision Δ | FP Δ |", "| --- | ---: | ---: | ---: | ---: |"])
        for row in payload["comparisons"]:
            delta = row["delta"]
            lines.append(
                "| `{score}` | {judge} | {report} | {precision} | {fp} |".format(
                    score=row["score_path"],
                    judge=format_pct(delta.get("accepted_hm_judge_recall")),
                    report=format_pct(delta.get("accepted_hm_report_ready_recall")),
                    precision=format_pct(delta.get("final_precision")),
                    fp=delta.get("false_positive_count"),
                )
            )
    return "\n".join(lines)


def yes_no(value: bool) -> str:
    return "yes" if value else "no"


def analysis_focus(row: dict[str, Any], coverage: dict[str, Any]) -> str:
    loss_stage = row.get("loss_stage", "")
    matched_stages = set(row.get("matched_stages", []))
    if loss_stage == "survived_end_to_end":
        return "retained"
    if coverage.get("status") in {"referenced_files_absent", "some_referenced_files_missing"} and "discovery_raw" not in matched_stages:
        return "context_or_scope_gap"
    if "discovery_raw" not in matched_stages:
        return "discovery_gap"
    if loss_stage in {"dedup_or_rust_verification_loss", "dedup_overmerge_or_drop"}:
        return "dedup_overmerge_or_drop"
    if loss_stage in {"rust_verification_loss", "rust_final_report_omission"}:
        return "rust_verification_or_export_gap"
    if loss_stage in {"three_shot_validation_loss", "three_shot_canonicalization_or_v12_loss"}:
        return "three_shot_retention_gap"
    if loss_stage in {"poc_creation_loss", "poc_verification_loss"}:
        return "poc_gap"
    if loss_stage in {"report_creation_loss", "report_review_loss", "judge_simulation_reject_or_missing"}:
        return "report_or_judge_gap"
    return "manual_review"


def expected_fp_gate(row: dict[str, Any]) -> str:
    category = compact_text(row.get("rejection_category")).lower()
    status = compact_text(row.get("rejected_status")).lower()
    combined = f"{category} {status}"
    if "duplicate" in combined:
        return "dedup_or_three_shot_canonicalization"
    if any(token in combined for token in ["low", "qa", "informational", "non-critical", "severity"]):
        return "rust_verification_or_three_shot_validation"
    if any(token in combined for token in ["invalid", "incorrect", "no impact", "not valid", "false"]):
        return "rust_verification_or_three_shot_validation"
    if "out" in combined and "scope" in combined:
        return "scope_screen"
    if "known" in combined:
        return "known_issue_screen"
    if row.get("classification") == "unmatched_unadjudicated":
        return "manual_adjudication"
    return "manual_rejection_taxonomy"


def fp_failure_type(row: dict[str, Any]) -> str:
    category = compact_text(row.get("rejection_category")).lower()
    status = compact_text(row.get("rejected_status")).lower()
    combined = f"{category} {status}"
    if row.get("classification") == "unmatched_unadjudicated":
        return "potentially_novel_or_unadjudicated"
    if "duplicate" in combined:
        return "duplicate_variant"
    if any(token in combined for token in ["low", "qa", "informational", "non-critical", "severity"]):
        return "severity_overstatement"
    if any(token in combined for token in ["invalid", "incorrect", "no impact", "not valid", "false"]):
        return "true_fp"
    return "adjudicated_non_accepted"


def analyze_failures(args: argparse.Namespace) -> dict[str, Any]:
    score_path = Path(args.score)
    score = read_json(score_path)
    slug = score["slug"]
    run_dir = Path(score["run_dir"])
    competition_dir = CORPUS_ROOT / slug
    accepted_refs = {reference.id: reference for reference in load_accepted_references(competition_dir)}
    context_index = load_context_index(run_dir)

    accepted_rows: list[dict[str, Any]] = []
    for row in score.get("accepted_roots", []):
        reference = accepted_refs.get(row.get("id"))
        coverage = context_coverage(reference, context_index) if reference else {
            "status": "reference_missing",
            "referenced_files": [],
            "present_files": [],
            "missing_files": [],
        }
        matched_stages = set(row.get("matched_stages", []))
        accepted_rows.append(
            {
                **row,
                "context": coverage,
                "phase4_checks": {
                    "scoped_files_and_docs_present": coverage["status"],
                    "matching_raw_candidate_appeared": "discovery_raw" in matched_stages,
                    "survived_dedup": "rust_dedup_kept" in matched_stages,
                    "retained_by_rust_verification": "rust_post_verification" in matched_stages,
                    "retained_by_rust_final_export": "rust_final" in matched_stages,
                    "retained_by_three_shot_r3": "three_shot_r3" in matched_stages,
                    "retained_by_canonicalization_or_v12": "three_shot_submission" in matched_stages,
                    "poc_created": "poc_created" in matched_stages,
                    "poc_verified": "poc_verified" in matched_stages,
                    "report_ready": "report_ready" in matched_stages,
                    "judge_accepted": "judge_accepted" in matched_stages,
                },
                "analysis_focus": analysis_focus(row, coverage),
            }
        )

    fp_rows: list[dict[str, Any]] = []
    for row in score.get("false_positives", []):
        fp_rows.append(
            {
                **row,
                "expected_removal_gate": expected_fp_gate(row),
                "failure_type": fp_failure_type(row),
            }
        )

    focus_counts = Counter(row["analysis_focus"] for row in accepted_rows)
    context_counts = Counter(row["context"]["status"] for row in accepted_rows)
    fp_gate_counts = Counter(row["expected_removal_gate"] for row in fp_rows)
    fp_type_counts = Counter(row["failure_type"] for row in fp_rows)

    payload = {
        "schema_version": "code4rena-phase4-failure-analysis-v1",
        "created_at": datetime.now(timezone.utc).isoformat(),
        "slug": slug,
        "score_path": str(score_path.resolve()),
        "run_dir": score["run_dir"],
        "three_shot_benchmark": score.get("three_shot_benchmark"),
        "three_shot_run_id": score.get("three_shot_run_id"),
        "prompt_version": score.get("prompt_version"),
        "headline": {
            "accepted_hm_total": score["ground_truth"]["accepted_hm_total"],
            "rust_final_matched": score["metrics"].get("accepted_hm_rust_final_matched", 0),
            "rust_final_recall": score["metrics"].get("accepted_hm_rust_final_recall"),
            "end_to_end_matched": score["metrics"].get("accepted_hm_judge_matched", 0),
            "end_to_end_recall": score["metrics"].get("accepted_hm_judge_recall"),
            "final_false_positives": score["metrics"].get("false_positive_count"),
            "matched_rejected_false_positives": score["metrics"].get("matched_rejected_fp_count"),
        },
        "context_index_summary": {
            "codeblock_manifest_rows": context_index["manifest_rows"],
            "included_file_count": len(context_index["included_files"]),
            "budget_exceeded_contracts": context_index["budget_exceeded_contracts"],
        },
        "accepted_roots": accepted_rows,
        "false_positives": fp_rows,
        "counts": {
            "accepted_focus": dict(sorted(focus_counts.items())),
            "context_status": dict(sorted(context_counts.items())),
            "fp_expected_gate": dict(sorted(fp_gate_counts.items())),
            "fp_failure_type": dict(sorted(fp_type_counts.items())),
        },
    }

    out_dir = Path(args.out_dir) if args.out_dir else score_path.parent
    out_dir.mkdir(parents=True, exist_ok=True)
    json_path = out_dir / "failure_analysis.json"
    md_path = out_dir / "failure_analysis.md"
    json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    md_path.write_text(render_failure_analysis_markdown(payload) + "\n")
    payload["output_paths"] = {"json": str(json_path), "markdown": str(md_path)}
    return payload


def render_failure_analysis_markdown(payload: dict[str, Any]) -> str:
    headline = payload["headline"]
    total = headline["accepted_hm_total"]
    lines = [
        f"# Phase 4 Failure Analysis: {payload['slug']}",
        "",
        "## Headline Recall",
        "",
        f"- Rust final H/M recall: `{headline['rust_final_matched']} / {total}` (`{format_pct(headline['rust_final_recall'])}`)",
        f"- End-to-end three-shot/judge-sim H/M recall: `{headline['end_to_end_matched']} / {total}` (`{format_pct(headline['end_to_end_recall'])}`)",
        f"- Final-stage false positives: `{headline['final_false_positives']}`",
        f"- Matched rejected-primary FPs: `{headline['matched_rejected_false_positives']}`",
        "",
        "## Evidence Note",
        "",
        "This file is deterministic benchmark analysis. Low-confidence matches and unadjudicated false positives should be manually reviewed before they are used in whitepaper claims.",
        "",
        "## Bottleneck Counts",
        "",
        "| Category | Count |",
        "| --- | ---: |",
    ]
    for key, count in payload["counts"]["accepted_focus"].items():
        lines.append(f"| `{key}` | {count} |")

    lines.extend(
        [
            "",
            "## Accepted Root Survival Matrix",
            "",
            "| Root | Context | Raw | Rust Final | End-to-End | Loss | Focus | Title |",
            "| --- | --- | --- | --- | --- | --- | --- | --- |",
        ]
    )
    for row in payload["accepted_roots"]:
        checks = row["phase4_checks"]
        lines.append(
            "| `{root}` | `{context}` | {raw} | {rust_final} | {e2e} | `{loss}` | `{focus}` | {title} |".format(
                root=row["id"],
                context=row["context"]["status"],
                raw=yes_no(checks["matching_raw_candidate_appeared"]),
                rust_final=yes_no(checks["retained_by_rust_final_export"]),
                e2e=yes_no(checks["judge_accepted"]),
                loss=row["loss_stage"],
                focus=row["analysis_focus"],
                title=clean_md(row["title"]),
            )
        )

    lost_rows = [row for row in payload["accepted_roots"] if row["loss_stage"] != "survived_end_to_end"]
    if lost_rows:
        lines.extend(["", "## Missed Or Lost Accepted Roots", ""])
        for row in lost_rows:
            lines.extend(
                [
                    f"### {row['id']}: {clean_md(row['title'])}",
                    "",
                    f"- Loss stage: `{row['loss_stage']}`",
                    f"- Analysis focus: `{row['analysis_focus']}`",
                    f"- Context status: `{row['context']['status']}`",
                    f"- Referenced files: `{', '.join(row['context']['referenced_files']) or 'none extracted'}`",
                    f"- Missing files: `{', '.join(row['context']['missing_files']) or 'none'}`",
                    f"- Best candidate: `{row.get('best_candidate_id') or '-'}` / {clean_md(row.get('best_candidate_title'))}",
                    f"- Highest near match: `{row.get('highest_score_candidate_id') or '-'}` / `{row.get('highest_match_score', 0):.3f}`",
                    "",
                ]
            )

    lines.extend(
        [
            "## Final-Stage FP Triage",
            "",
            "| Candidate | Classification | Failure Type | Expected Removal Gate | Rejection | Confidence | Title |",
            "| --- | --- | --- | --- | --- | --- | --- |",
        ]
    )
    for row in payload["false_positives"]:
        rejection = row.get("rejected_submission_id") or "-"
        if row.get("rejection_category"):
            rejection = f"{rejection} / {row['rejection_category']}"
        lines.append(
            "| `{candidate}` | `{classification}` | `{failure_type}` | `{gate}` | `{rejection}` | `{confidence}` | {title} |".format(
                candidate=row["candidate_id"],
                classification=row["classification"],
                failure_type=row["failure_type"],
                gate=row["expected_removal_gate"],
                rejection=clean_md(rejection),
                confidence=row["match_confidence"],
                title=clean_md(row["title"]),
            )
        )
    return "\n".join(lines)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as file:
        for chunk in iter(lambda: file.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def archive_rel_path(path: Path) -> Path:
    resolved = path.resolve()
    try:
        return resolved.relative_to(REPO_ROOT)
    except ValueError:
        sanitized = [part for part in resolved.parts if part not in {"/", ""}]
        return Path("external") / Path(*sanitized)


def copy_evidence_file(source: Path, archive_root: Path, category: str, entries: list[dict[str, Any]], seen: set[Path]) -> None:
    source = source.resolve()
    if source in seen or not source.exists() or not source.is_file():
        return
    seen.add(source)
    relative = archive_rel_path(source)
    destination = archive_root / "files" / relative
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, destination)
    entries.append(
        {
            "category": category,
            "source_path": str(source),
            "archive_path": str(destination.relative_to(archive_root)),
            "size_bytes": source.stat().st_size,
            "sha256": sha256_file(source),
        }
    )


def copy_evidence_path(source: Path, archive_root: Path, category: str, entries: list[dict[str, Any]], seen: set[Path]) -> None:
    if not source.exists():
        return
    if source.is_file():
        copy_evidence_file(source, archive_root, category, entries, seen)
        return
    for path in sorted(source.rglob("*")):
        if path.is_file():
            copy_evidence_file(path, archive_root, category, entries, seen)


def collect_three_shot_paths(score: dict[str, Any]) -> list[Path]:
    benchmark = score.get("three_shot_benchmark") or score["slug"]
    run_id = score.get("three_shot_run_id", "run-001")
    run_key = f"{benchmark}-{run_id}"
    paths = [THREE_SHOT_ROOT / "jobs" / benchmark / run_id]
    if THREE_SHOT_ROOT.exists():
        for path in THREE_SHOT_ROOT.rglob("*"):
            if path.is_file() and run_key in str(path.relative_to(THREE_SHOT_ROOT)):
                paths.append(path)
    return paths


def archive_evidence(args: argparse.Namespace) -> dict[str, Any]:
    score_path = Path(args.score)
    score = read_json(score_path)
    slug = score["slug"]
    run_dir = Path(score["run_dir"])
    run_name = run_dir.name
    out_dir = Path(args.out_dir) if args.out_dir else REPO_ROOT / "benchmarks" / "whitepaper-data" / slug / run_name
    out_dir.mkdir(parents=True, exist_ok=True)

    entries: list[dict[str, Any]] = []
    seen: set[Path] = set()

    copy_evidence_path(run_dir, out_dir, "benchmark_run_artifacts", entries, seen)
    copy_evidence_path(score_path, out_dir, "scorecard", entries, seen)
    failure_path = Path(args.failure_analysis) if args.failure_analysis else score_path.parent / "failure_analysis.json"
    if failure_path.exists():
        copy_evidence_path(failure_path, out_dir, "phase4_failure_analysis", entries, seen)
        md_failure = failure_path.with_suffix(".md")
        copy_evidence_path(md_failure, out_dir, "phase4_failure_analysis", entries, seen)

    competition_dir = CORPUS_ROOT / slug
    for source in [
        competition_dir / "competition.json",
        competition_dir / "final_findings.json",
        competition_dir / "final_report.html",
        competition_dir / "benchmark_ground_truth.md",
        competition_dir / "accepted_findings.md",
        competition_dir / "rejected_primaries.md",
        competition_dir / "ground_truth",
        competition_dir / "submissions" / "all.jsonl",
        competition_dir / "submissions" / "primaries.jsonl",
        competition_dir / "submissions" / "rejected_primaries.jsonl",
    ]:
        copy_evidence_path(source, out_dir, "code4rena_corpus_ground_truth", entries, seen)

    audit_docs_dir = REPO_ROOT / "audit-docs" / slug
    copy_evidence_path(audit_docs_dir, out_dir, "generated_audit_context", entries, seen)

    comparisons_dir = REPO_ROOT / "benchmarks" / f"code4rena-{slug}" / "comparisons"
    copy_evidence_path(comparisons_dir, out_dir, "cross_run_comparisons", entries, seen)

    for path in collect_three_shot_paths(score):
        copy_evidence_path(path, out_dir, "three_shot_validation_artifacts", entries, seen)

    category_counts = Counter(entry["category"] for entry in entries)
    manifest = {
        "schema_version": "code4rena-whitepaper-evidence-pack-v1",
        "created_at": datetime.now(timezone.utc).isoformat(),
        "slug": slug,
        "run_name": run_name,
        "run_dir": str(run_dir.resolve()),
        "score_path": str(score_path.resolve()),
        "failure_analysis_path": str(failure_path.resolve()) if failure_path.exists() else None,
        "archive_dir": str(out_dir.resolve()),
        "file_count": len(entries),
        "total_size_bytes": sum(entry["size_bytes"] for entry in entries),
        "category_counts": dict(sorted(category_counts.items())),
        "files": sorted(entries, key=lambda entry: (entry["category"], entry["archive_path"])),
    }

    manifest_json = out_dir / "evidence_manifest.json"
    manifest_md = out_dir / "evidence_manifest.md"
    manifest_json.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    manifest_md.write_text(render_evidence_manifest_markdown(manifest) + "\n")
    manifest["output_paths"] = {"json": str(manifest_json), "markdown": str(manifest_md)}
    return manifest


def render_evidence_manifest_markdown(payload: dict[str, Any]) -> str:
    lines = [
        f"# Whitepaper Evidence Pack: {payload['slug']}",
        "",
        f"- Run: `{payload['run_name']}`",
        f"- Archive dir: `{payload['archive_dir']}`",
        f"- File count: `{payload['file_count']}`",
        f"- Total size: `{payload['total_size_bytes']}` bytes",
        "",
        "## Categories",
        "",
        "| Category | Files |",
        "| --- | ---: |",
    ]
    for category, count in payload["category_counts"].items():
        lines.append(f"| `{category}` | {count} |")
    lines.extend(["", "## Files", "", "| Category | Archive Path | Size | SHA-256 |", "| --- | --- | ---: | --- |"])
    for entry in payload["files"]:
        lines.append(
            "| `{category}` | `{path}` | {size} | `{sha}` |".format(
                category=entry["category"],
                path=entry["archive_path"],
                size=entry["size_bytes"],
                sha=entry["sha256"],
            )
        )
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser(description="Score Code4rena benchmark runs.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    score = subparsers.add_parser("score", help="Score one benchmark run against the Code4rena corpus.")
    score.add_argument("--slug", required=True, help="Code4rena corpus slug, e.g. 2025-11-megapot.")
    score.add_argument("--run-dir", required=True, help="Benchmark telemetry run directory.")
    score.add_argument("--three-shot-benchmark", help="Three-shot benchmark name. Defaults to --slug.")
    score.add_argument("--three-shot-run-id", default="run-001")
    score.add_argument("--prompt-version", default="v2")
    score.add_argument("--out-dir", help="Output directory. Defaults to --run-dir.")
    score.add_argument("--accepted-threshold", type=float, default=0.245)
    score.add_argument("--rejected-threshold", type=float, default=0.225)

    compare = subparsers.add_parser("compare", help="Compare two or more benchmark_score.json files.")
    compare.add_argument("--score", action="append", required=True, help="Path to benchmark_score.json. Pass multiple times; first is baseline.")
    compare.add_argument("--out-dir", required=True)

    analyze = subparsers.add_parser("analyze", help="Create Phase 4 failure-analysis artifacts from a benchmark scorecard.")
    analyze.add_argument("--score", required=True, help="Path to benchmark_score.json.")
    analyze.add_argument("--out-dir", help="Output directory. Defaults to the scorecard directory.")

    archive = subparsers.add_parser("archive-evidence", help="Copy benchmark, corpus, and validation artifacts into a whitepaper evidence pack.")
    archive.add_argument("--score", required=True, help="Path to benchmark_score.json.")
    archive.add_argument("--failure-analysis", help="Path to failure_analysis.json. Defaults to the scorecard directory.")
    archive.add_argument("--out-dir", help="Evidence-pack directory. Defaults to benchmarks/whitepaper-data/<slug>/<run-name>.")

    args = parser.parse_args()
    if args.command == "score":
        result = score_run(args)
    elif args.command == "compare":
        result = compare_runs(args)
    elif args.command == "analyze":
        result = analyze_failures(args)
    else:
        result = archive_evidence(args)
    print(json.dumps(result.get("output_paths", result), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
