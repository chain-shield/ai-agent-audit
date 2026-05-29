#!/usr/bin/env python3
"""Archive Code4rena benchmark material for AI Agent Audit.

The public Code4rena API exposes contest metadata, repo links, and report pages.
Submission lists and individual submissions require authentication, so this
script also provides an importer for CSV files captured from a logged-in
Code4rena browser session.
"""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import re
import shutil
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
import tempfile
from html.parser import HTMLParser
from pathlib import Path
from typing import Any


BASE_URL = "https://code4rena.com"
AUDITS_API = f"{BASE_URL}/api/v1/audits"
SCHEMA_VERSION = "code4rena-corpus-v1"
DEFAULT_OUT = Path("benchmarks/code4rena-corpus")
DEFAULT_EVM_LEAGUES = ("ETH", "eth", "Blast", "Hyperliquid")


REJECTION_CATEGORY_PATTERNS: list[tuple[str, tuple[str, ...]]] = [
    ("duplicate", ("duplicate", "duplicated", "same root cause", "same issue", "dupe")),
    ("out_of_scope", ("out of scope", "oos", "not in scope", "outside scope", "not within scope")),
    ("known_issue", ("known issue", "publicly known", "known finding", "known vulnerability")),
    ("sponsor_disputed", ("sponsor disputed", "disputed by sponsor", "sponsor does not agree")),
    ("insufficient_poc", ("insufficient poc", "missing poc", "poc is insufficient", "not runnable", "does not prove")),
    ("no_valid_impact", ("no valid impact", "no impact", "insufficient impact", "impact is low", "no loss of funds")),
    ("low_or_qa_severity", ("low severity", "informational", "qa", "gas", "at most low", "downgraded to low")),
    ("user_error", ("user error", "user mistake", "self-inflicted", "user must")),
    ("admin_or_trusted_role", ("trusted role", "trusted admin", "admin", "owner", "governance", "centralization")),
    (
        "unsupported_token_or_external_dependency",
        ("non-standard erc20", "unsupported token", "weird erc20", "external dependency", "usdt"),
    ),
    ("speculative_or_future_issue", ("speculative", "hypothetical", "future", "not proven", "assumes")),
    ("by_design", ("by design", "intended behavior", "intended behaviour", "expected behavior", "expected behaviour")),
    ("safeguard_exists", ("safeguard", "already checked", "already protected", "guard exists", "cannot happen")),
    ("incorrect_code_path", ("incorrect code path", "wrong function", "wrong contract", "not called", "unreachable")),
    ("not_exploitable", ("not exploitable", "cannot be exploited", "not possible", "no attack path")),
    (
        "environment_or_deployment_assumption",
        ("deployment", "configuration", "environment", "setup", "initialization", "initialisation"),
    ),
    ("spam", ("spam", "invalid duplicate spam")),
]


class TextExtractor(HTMLParser):
    """Small dependency-free HTML text/link extractor."""

    def __init__(self) -> None:
        super().__init__()
        self.tokens: list[str] = []
        self.links: list[dict[str, str]] = []
        self._skip_depth = 0
        self._href: str | None = None

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag in {"script", "style", "noscript"}:
            self._skip_depth += 1
            return
        if tag == "a":
            attrs_dict = dict(attrs)
            self._href = attrs_dict.get("href")

    def handle_endtag(self, tag: str) -> None:
        if tag in {"script", "style", "noscript"} and self._skip_depth:
            self._skip_depth -= 1
        if tag == "a":
            self._href = None

    def handle_data(self, data: str) -> None:
        if self._skip_depth:
            return
        text = " ".join(data.split())
        if not text:
            return
        self.tokens.append(text)
        if self._href:
            self.links.append({"text": text, "href": self._href})


def utc_now() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def slug_dir(out: Path, slug: str) -> Path:
    return out / "competitions" / slug


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_jsonl(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(row, sort_keys=True) + "\n")


def http_get(url: str, *, accept: str = "application/json", retries: int = 2) -> bytes:
    headers = {
        "Accept": accept,
        "User-Agent": "ai-agent-audit-code4rena-corpus/0.1",
    }
    last_error: Exception | None = None
    for attempt in range(retries + 1):
        try:
            request = urllib.request.Request(url, headers=headers)
            with urllib.request.urlopen(request, timeout=30) as response:
                return response.read()
        except (urllib.error.URLError, TimeoutError) as error:
            last_error = error
            if attempt < retries:
                time.sleep(0.5 * (attempt + 1))
                continue
            raise
    raise RuntimeError(f"GET failed for {url}: {last_error}")


def fetch_json(url: str) -> Any:
    return json.loads(http_get(url).decode("utf-8"))


def iter_public_audits(per_page: int = 100) -> list[dict[str, Any]]:
    audits: list[dict[str, Any]] = []
    page = 1
    while True:
        url = f"{AUDITS_API}?{urllib.parse.urlencode({'perPage': per_page, 'page': page})}"
        payload = fetch_json(url)
        data = payload.get("data", {})
        page_audits = data.get("audits", [])
        audits.extend(page_audits)
        pagination = data.get("pagination") or payload.get("pagination") or {}
        next_page = pagination.get("nextPage")
        if not next_page:
            break
        page = int(next_page)
    return audits


def parse_year(timestamp: str | None) -> int | None:
    if not timestamp or len(timestamp) < 4:
        return None
    try:
        return int(timestamp[:4])
    except ValueError:
        return None


def should_include_audit(
    audit: dict[str, Any],
    *,
    year_start: int,
    year_end: int,
    leagues: set[str],
    audit_only: bool,
) -> bool:
    year = parse_year(audit.get("startTime"))
    if year is None or year < year_start or year > year_end:
        return False
    if leagues and audit.get("league") not in leagues:
        return False
    if audit_only and "mitigation" in str(audit.get("auditType", "")).lower():
        return False
    return True


def normalize_competition(
    audit: dict[str, Any],
    *,
    captured_at: str,
    report_status: str,
    source_repositories_from_report: list[dict[str, str | None]],
) -> dict[str, Any]:
    slug = audit["slug"]
    uid = audit["uid"]
    audit_type = audit.get("auditType")
    is_mitigation = "mitigation" in str(audit_type or "").lower()
    is_completed = audit.get("status") == "Completed"
    report_available = report_status == "captured"
    benchmark_candidate = bool(is_completed and report_available and not is_mitigation)
    if benchmark_candidate:
        candidate_reason = "completed public Solidity/EVM audit with final report"
    elif is_mitigation:
        candidate_reason = "mitigation review; keep for corpus, usually exclude from primary benchmark"
    elif not report_available:
        candidate_reason = "final report not captured yet"
    else:
        candidate_reason = "not a completed final-report audit"

    return {
        "schema_version": SCHEMA_VERSION,
        "slug": slug,
        "uid": uid,
        "contest_id": audit.get("contestId"),
        "title": audit.get("title"),
        "audit_type": audit_type,
        "league": audit.get("league"),
        "status": audit.get("status"),
        "start_time": audit.get("startTime"),
        "end_time": audit.get("endTime"),
        "repo_url": audit.get("repo"),
        "findings_repo_url": audit.get("findingsRepo"),
        "known_findings_repo_url": audit.get("knownFindingsRepo"),
        "report_url": f"{BASE_URL}/reports/{slug}",
        "audit_url": f"{BASE_URL}/audits/{slug}",
        "source_repositories_from_report": source_repositories_from_report,
        "benchmark_candidate": benchmark_candidate,
        "benchmark_candidate_reason": candidate_reason,
        "raw_code4rena_audit": audit,
        "archive": {
            "captured_at": captured_at,
            "public_metadata_status": "captured",
            "final_report_status": report_status,
            "submissions_status": "requires_authentication",
            "authenticated_submissions_endpoint": f"{AUDITS_API}/{uid}/submissions/csv",
        },
    }


def report_text_from_html(html: str) -> tuple[list[str], list[dict[str, str]]]:
    parser = TextExtractor()
    parser.feed(html)
    return parser.tokens, parser.links


def extract_source_repositories(tokens: list[str]) -> list[dict[str, str | None]]:
    repos: list[dict[str, str | None]] = []
    for index, token in enumerate(tokens):
        if token != "Repository:":
            continue
        repo_url = tokens[index + 1] if index + 1 < len(tokens) else None
        if not repo_url or not repo_url.startswith("http"):
            continue
        commit_hash: str | None = None
        for lookahead in range(index + 2, min(index + 8, len(tokens))):
            if tokens[lookahead] == "Commit hash:" and lookahead + 1 < len(tokens):
                commit_hash = tokens[lookahead + 1]
                break
        repos.append({"repo_url": repo_url, "commit_hash": commit_hash})
    return repos


def parse_finding_heading(token: str, severity_codes: str = "HMLQGI") -> tuple[str, str, str] | None:
    clean = normalize_space(token)
    bracket_match = re.match(rf"^\[([{severity_codes}])-(\d+)\]\s+(.+)", clean)
    if not bracket_match:
        bracket_match = re.match(rf"^\[([{severity_codes}])-(\d+)\]$", clean)
    if not bracket_match:
        bracket_match = re.match(rf"^(?:\*+\s*)?#+\s+\[?\[([{severity_codes}])-(\d+)\]\s*(.*)", clean)
    if not bracket_match:
        return None
    groups = bracket_match.groups()
    severity_code, number = groups[0], groups[1]
    title = groups[2] if len(groups) > 2 else ""
    title = re.sub(r"\]\([^)]+\)\s*$", "", title or "").strip()
    title = title.lstrip("] ").strip()
    return severity_code, number, title


def extract_hm_findings(tokens: list[str], *, report_url: str) -> list[dict[str, Any]]:
    findings: dict[str, dict[str, Any]] = {}
    title_stop_re = re.compile(
        r"^(High Risk Findings|Medium Risk Findings|Low Risk|QA Report|This was found by|Submitted by|"
        r"Proof of Concept|Impact|Recommended Mitigation|Mitigation|Discussion|The code under review)"
    )
    for index, token in enumerate(tokens):
        heading = parse_finding_heading(token, "HM")
        if not heading:
            continue
        severity_code, number, title = heading
        finding_id = f"{severity_code}-{int(number):02d}"
        title_parts = [title.strip()] if title.strip() else []
        for lookahead in range(index + 1, min(index + 10, len(tokens))):
            next_token = tokens[lookahead].strip()
            if not next_token:
                continue
            if parse_finding_heading(next_token, "HM") or title_stop_re.match(next_token):
                break
            title_parts.append(next_token)
            if len(" ".join(title_parts)) > 240:
                break
        title_text = " ".join(title_parts).strip()
        if finding_id in findings:
            # The report table of contents and the section heading both appear
            # in the text stream. The later section gives a better excerpt.
            findings[finding_id]["raw_excerpt"] = " ".join(tokens[index : index + 90])
            if title_text and len(title_text) > len(findings[finding_id]["title"]):
                findings[finding_id]["title"] = title_text
            continue
        severity = "High" if severity_code == "H" else "Medium"
        excerpt = " ".join(tokens[index : index + 90])
        findings[finding_id] = {
            "schema_version": SCHEMA_VERSION,
            "id": finding_id,
            "severity": severity,
            "title": title_text,
            "source_report_url": report_url,
            "selected_primary_mentioned": "selected as the primary submission" in excerpt.lower(),
            "raw_excerpt": excerpt,
        }
    return [findings[key] for key in sorted(findings)]


def capture_public_report(competition_dir: Path, slug: str, *, force: bool) -> tuple[str, list[dict[str, Any]], list[dict[str, str | None]]]:
    html_path = competition_dir / "final_report.html"
    findings_path = competition_dir / "final_findings.json"
    report_url = f"{BASE_URL}/reports/{slug}"

    try:
        if html_path.exists() and not force:
            raw_html = html_path.read_text(encoding="utf-8")
        else:
            raw_html = http_get(report_url, accept="text/html").decode("utf-8")
    except urllib.error.HTTPError as error:
        status = f"http_{error.code}"
        write_json(findings_path, [])
        return status, [], []
    except urllib.error.URLError:
        write_json(findings_path, [])
        return "fetch_error", [], []

    if "ERR_NOT_FOUND" in raw_html or "This page could not be found" in raw_html:
        write_json(findings_path, [])
        return "not_found", [], []

    competition_dir.mkdir(parents=True, exist_ok=True)
    if force or not html_path.exists():
        html_path.write_text(raw_html, encoding="utf-8")

    tokens, _links = report_text_from_html(raw_html)
    source_repos = extract_source_repositories(tokens)
    findings = extract_hm_findings(tokens, report_url=report_url)
    write_json(findings_path, findings)
    return "captured", findings, source_repos


def write_submission_readme(path: Path, competition: dict[str, Any]) -> None:
    endpoint = competition["archive"]["authenticated_submissions_endpoint"]
    content = f"""# Submissions for {competition['slug']}

Submission capture requires a logged-in Code4rena browser session.

Authenticated CSV endpoint:

```text
{endpoint}
```

After downloading the CSV from a logged-in session, run:

```bash
python3 scripts/archive_code4rena_corpus.py import-submissions-csv \\
  --out benchmarks/code4rena-corpus \\
  --slug {competition['slug']} \\
  --csv /path/to/submissions.csv
```
"""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def discover(args: argparse.Namespace) -> int:
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    captured_at = utc_now()
    leagues = set() if args.all_leagues else {item.strip() for item in args.leagues.split(",") if item.strip()}
    all_audits = iter_public_audits(per_page=args.per_page)
    selected = [
        audit
        for audit in all_audits
        if should_include_audit(
            audit,
            year_start=args.year_start,
            year_end=args.year_end,
            leagues=leagues,
            audit_only=args.audit_only,
        )
    ]
    if args.limit:
        selected = selected[: args.limit]

    manifest_rows: list[dict[str, Any]] = []
    todo_rows: list[dict[str, Any]] = []
    report_counts = {"captured": 0, "not_found": 0, "fetch_error": 0}
    hm_count = 0

    for index, audit in enumerate(selected, start=1):
        slug = audit["slug"]
        competition_dir = slug_dir(out, slug)
        report_status, findings, source_repos = capture_public_report(competition_dir, slug, force=args.force)
        report_counts[report_status] = report_counts.get(report_status, 0) + 1
        hm_count += len(findings)
        competition = normalize_competition(
            audit,
            captured_at=captured_at,
            report_status=report_status,
            source_repositories_from_report=source_repos,
        )
        write_json(competition_dir / "competition.json", competition)
        write_submission_readme(competition_dir / "submissions" / "README.md", competition)

        row = {
            "schema_version": SCHEMA_VERSION,
            "slug": slug,
            "uid": audit.get("uid"),
            "title": audit.get("title"),
            "audit_type": audit.get("auditType"),
            "league": audit.get("league"),
            "start_time": audit.get("startTime"),
            "end_time": audit.get("endTime"),
            "status": audit.get("status"),
            "repo_url": audit.get("repo"),
            "report_url": competition["report_url"],
            "findings_repo_url": audit.get("findingsRepo"),
            "final_report_status": report_status,
            "accepted_hm_count": len(findings),
            "submissions_status": "requires_authentication",
            "benchmark_candidate": competition["benchmark_candidate"],
        }
        manifest_rows.append(row)
        todo_rows.append(
            {
                "slug": slug,
                "uid": audit.get("uid"),
                "title": audit.get("title"),
                "authenticated_submissions_endpoint": competition["archive"]["authenticated_submissions_endpoint"],
                "expected_outputs": [
                    f"competitions/{slug}/submissions/all.jsonl",
                    f"competitions/{slug}/submissions/primaries.jsonl",
                    f"competitions/{slug}/submissions/rejected_primaries.jsonl",
                ],
            }
        )
        print(f"[{index:03d}/{len(selected):03d}] {slug}: report={report_status}, H/M={len(findings)}")

    write_jsonl(out / "manifest.jsonl", manifest_rows)
    write_jsonl(out / "needs-auth-submissions.todo.jsonl", todo_rows)
    write_json(
        out / "manifest.json",
        {
            "schema_version": SCHEMA_VERSION,
            "generated_at": captured_at,
            "source": {
                "audits_api": AUDITS_API,
                "reports_url_template": f"{BASE_URL}/reports/{{slug}}",
            },
            "filters": {
                "year_start": args.year_start,
                "year_end": args.year_end,
                "leagues": "all" if args.all_leagues else sorted(leagues),
                "audit_only": args.audit_only,
            },
            "counts": {
                "public_audits_seen": len(all_audits),
                "competitions_included": len(selected),
                "accepted_hm_findings_parsed": hm_count,
                "report_status": report_counts,
            },
        },
    )
    return 0


def normalize_key(name: str) -> str:
    return re.sub(r"[^a-z0-9]+", "", name.lower())


def get_row_value(row: dict[str, str], candidates: tuple[str, ...]) -> str | None:
    normalized = {normalize_key(key): value for key, value in row.items()}
    for candidate in candidates:
        value = normalized.get(normalize_key(candidate))
        if value is not None and str(value).strip():
            return str(value).strip()
    return None


def parse_bool(value: str | None) -> bool | None:
    if value is None:
        return None
    lowered = value.strip().lower()
    if lowered in {"true", "yes", "y", "1", "primary", "best"}:
        return True
    if lowered in {"false", "no", "n", "0"}:
        return False
    return None


def parse_int(value: str | None) -> int | None:
    if not value:
        return None
    match = re.search(r"\d+", value)
    if not match:
        return None
    return int(match.group(0))


def classify_rejection_reason(text: str | None, *, status: str) -> str:
    if status == "accepted":
        return "accepted"
    if not text:
        return "unknown"
    normalized = " ".join(text.lower().split())
    for category, needles in REJECTION_CATEGORY_PATTERNS:
        if any(needle in normalized for needle in needles):
            return category
    return "other"


def infer_status(*, final_severity: str | None, validity: str | None, review: str | None, reason: str | None) -> str:
    combined = " ".join(value for value in (final_severity, validity, review, reason) if value).lower()
    if any(token in combined for token in ("accepted", "confirmed", "valid")) and "invalid" not in combined:
        return "accepted"
    if "duplicate" in combined:
        return "duplicate"
    if "out of scope" in combined or " oos " in f" {combined} ":
        return "out_of_scope"
    if "spam" in combined:
        return "spam"
    if "low" in combined or "qa" in combined or "informational" in combined or "gas" in combined:
        return "low_or_qa"
    if "invalid" in combined or "disputed" in combined or "rejected" in combined:
        return "rejected"
    return "unknown"


def risk_to_severity(risk: str | None) -> str | None:
    normalized = (risk or "").strip().upper()
    if normalized in {"3", "H", "HIGH", "HIGH RISK"}:
        return "High"
    if normalized in {"2", "M", "MED", "MEDIUM", "MEDIUM RISK"}:
        return "Medium"
    if normalized in {"Q", "QA", "LOW", "LOW RISK"}:
        return "QA"
    if normalized in {"G", "GAS"}:
        return "Gas"
    return risk


def github_repo_full_name(repo_url: str) -> str:
    parsed = urllib.parse.urlparse(repo_url)
    path = parsed.path.strip("/")
    if path.endswith(".git"):
        path = path[:-4]
    parts = path.split("/")
    if len(parts) < 2:
        raise ValueError(f"Cannot parse GitHub repo URL: {repo_url}")
    return "/".join(parts[:2])


def derive_validation_repo_url(competition: dict[str, Any]) -> str:
    findings_url = competition.get("findings_repo_url") or competition.get("raw_code4rena_audit", {}).get("findingsRepo")
    if findings_url and findings_url.endswith("-findings"):
        return findings_url[: -len("-findings")] + "-validation"
    if findings_url and findings_url.endswith("-findings.git"):
        return findings_url[: -len("-findings.git")] + "-validation.git"
    repo_url = competition.get("repo_url") or competition.get("raw_code4rena_audit", {}).get("repo")
    if repo_url:
        return repo_url.rstrip("/") + "-validation"
    raise ValueError(f"Cannot derive validation repo URL for {competition.get('slug')}")


def fetch_github_issues(repo_full_name: str) -> dict[int, dict[str, Any]]:
    if shutil.which("gh"):
        result = subprocess.run(
            [
                "gh",
                "api",
                "--paginate",
                f"repos/{repo_full_name}/issues?state=all&per_page=100",
                "--jq",
                ".[] | @json",
            ],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        if result.returncode == 0:
            issues: dict[int, dict[str, Any]] = {}
            for line in result.stdout.splitlines():
                if not line.strip():
                    continue
                issue = json.loads(line)
                if "pull_request" not in issue:
                    issues[int(issue["number"])] = issue
            return issues

    issues: dict[int, dict[str, Any]] = {}
    page = 1
    while True:
        query = urllib.parse.urlencode({"state": "all", "per_page": 100, "page": page})
        url = f"https://api.github.com/repos/{repo_full_name}/issues?{query}"
        batch = fetch_json(url)
        if not batch:
            break
        for issue in batch:
            if "pull_request" not in issue:
                issues[int(issue["number"])] = issue
        page += 1
    return issues


def robot_group_label(labels: list[str]) -> str | None:
    for label in labels:
        if re.fullmatch(r":robot:_\d+_group", label):
            return label
    return None


def normalize_validation_repo_row(
    entry: dict[str, Any],
    *,
    issue: dict[str, Any] | None,
    slug: str,
    competition_uid: str | None,
    captured_at: str,
    raw_detail_text_file: str | None,
    validation_repo_url: str,
) -> dict[str, Any]:
    labels = [label.get("name", "") for label in (issue or {}).get("labels", []) if label.get("name")]
    label_text = " ".join(labels).lower()
    issue_id = int(entry["issueId"])
    validated_issue_id = entry.get("validatedIssueId")
    is_primary = ":robot:_primary" in labels
    group_label = robot_group_label(labels)
    risk = str(entry.get("risk") or "")
    claimed_severity = risk_to_severity(risk)
    final_severity = claimed_severity

    if validated_issue_id:
        status = "accepted"
        category = "accepted"
    elif "duplicate" in label_text:
        status = "duplicate"
        category = "duplicate"
    elif "invalid" in label_text:
        status = "invalid"
        category = "other"
    elif "insufficient quality report" in label_text:
        status = "invalid"
        category = "other"
    elif "withdrawn by warden" in label_text:
        status = "invalid"
        category = "other"
    elif "out of scope" in label_text or "oos" in label_text:
        status = "out_of_scope"
        category = "out_of_scope"
    elif claimed_severity in {"QA", "Gas"}:
        status = "low_or_qa"
        category = "low_or_qa_severity"
    else:
        status = "unknown"
        category = "unknown"

    reason = None if status == "accepted" else f"GitHub validation labels: {', '.join(labels) or 'none'}"
    issue_url = entry.get("issueUrl") or (issue or {}).get("html_url")
    return {
        "schema_version": SCHEMA_VERSION,
        "competition_slug": slug,
        "competition_uid": competition_uid,
        "submission_uid": None,
        "submission_id": f"V-{issue_id}",
        "submission_number": issue_id,
        "title": entry.get("title") or (issue or {}).get("title"),
        "claimed_severity": claimed_severity,
        "final_severity": final_severity,
        "status": status,
        "is_primary": is_primary,
        "accepted_group_id": str(validated_issue_id) if validated_issue_id is not None else group_label,
        "duplicate_of": None if is_primary else group_label,
        "submitter": entry.get("handle"),
        "affected_contracts": [],
        "affected_functions": [],
        "judge_comment": reason,
        "sponsor_comment": None,
        "warden_comment": (issue or {}).get("body"),
        "rejection_reason_raw": reason,
        "rejection_reason_category": category,
        "confidence": "high" if status != "unknown" else "medium",
        "raw_row": {**entry, "github_labels": labels, "github_state": (issue or {}).get("state")},
        "raw_detail_text_file": raw_detail_text_file,
        "source": {
            "kind": "code4rena_github_validation_repo",
            "url": issue_url,
            "validation_repo_url": validation_repo_url,
            "captured_at": captured_at,
        },
    }


def normalize_submission_row(row: dict[str, str], *, slug: str, competition_uid: str | None, captured_at: str) -> dict[str, Any]:
    submission_uid = get_row_value(row, ("uid", "submission uid", "submissionUid"))
    number_text = get_row_value(row, ("number", "submission number", "finding number", "id", "s"))
    submission_number = parse_int(number_text)
    submission_id = f"S-{submission_number}" if submission_number is not None else (number_text or submission_uid)
    title = get_row_value(row, ("title", "finding", "finding title", "submission title"))
    claimed_severity = get_row_value(row, ("severity", "claimed severity", "original severity", "risk"))
    final_severity = get_row_value(row, ("latest severity", "final severity", "evaluated severity", "judge severity"))
    validity = get_row_value(row, ("validity", "latest validity", "judge validity", "status"))
    review = get_row_value(row, ("review", "latest review", "evaluation", "judge evaluation"))
    reason = get_row_value(
        row,
        (
            "reason",
            "rejection reason",
            "judge reason",
            "judge comment",
            "comment",
            "comments",
            "validation notes",
            "triage notes",
        ),
    )
    status = infer_status(final_severity=final_severity, validity=validity, review=review, reason=reason)
    category = classify_rejection_reason(reason or " ".join([validity or "", review or ""]), status=status)
    is_primary = parse_bool(get_row_value(row, ("primary", "is primary", "isPrimary", "best representation")))
    duplicate_of = get_row_value(row, ("duplicate of", "duplicateOf", "primary", "primary id"))
    return {
        "schema_version": SCHEMA_VERSION,
        "competition_slug": slug,
        "competition_uid": competition_uid,
        "submission_uid": submission_uid,
        "submission_id": submission_id,
        "submission_number": submission_number,
        "title": title,
        "claimed_severity": claimed_severity,
        "final_severity": final_severity,
        "status": status,
        "is_primary": is_primary,
        "accepted_group_id": get_row_value(row, ("finding uid", "findingUid", "group id", "primary group")),
        "duplicate_of": duplicate_of,
        "submitter": get_row_value(row, ("submitter", "handle", "warden", "user")),
        "affected_contracts": [],
        "affected_functions": [],
        "judge_comment": get_row_value(row, ("judge comment", "judge comments")),
        "sponsor_comment": get_row_value(row, ("sponsor comment", "sponsor comments")),
        "warden_comment": get_row_value(row, ("warden comment", "warden comments")),
        "rejection_reason_raw": reason,
        "rejection_reason_category": category,
        "confidence": "medium" if reason else "unknown",
        "raw_row": row,
        "source": {
            "kind": "code4rena_submissions_csv",
            "url": f"{BASE_URL}/audits/{slug}/submissions",
            "captured_at": captured_at,
        },
    }


def import_submissions_csv(args: argparse.Namespace) -> int:
    out = Path(args.out)
    competition_path = slug_dir(out, args.slug) / "competition.json"
    if not competition_path.exists():
        raise SystemExit(f"Missing competition.json for {args.slug}: {competition_path}")
    competition = json.loads(competition_path.read_text(encoding="utf-8"))
    competition_uid = competition.get("uid")
    captured_at = utc_now()
    csv_path = Path(args.csv)
    with csv_path.open("r", encoding=args.encoding, newline="") as handle:
        reader = csv.DictReader(handle)
        rows = [normalize_submission_row(row, slug=args.slug, competition_uid=competition_uid, captured_at=captured_at) for row in reader]

    primaries = [row for row in rows if row["is_primary"] is True]
    if not primaries:
        # Some CSV exports only include grouped primary rows. Keep the import
        # useful by treating unknown primary state as candidate primary material.
        primaries = [row for row in rows if row["is_primary"] is not False]
    rejected_primaries = [row for row in primaries if row["status"] != "accepted"]

    submissions_dir = slug_dir(out, args.slug) / "submissions"
    raw_dir = submissions_dir / "raw"
    raw_dir.mkdir(parents=True, exist_ok=True)
    write_jsonl(submissions_dir / "all.jsonl", rows)
    write_jsonl(submissions_dir / "primaries.jsonl", primaries)
    write_jsonl(submissions_dir / "rejected_primaries.jsonl", rejected_primaries)
    write_json(raw_dir / f"{csv_path.stem}.metadata.json", {"captured_at": captured_at, "source_csv": str(csv_path), "row_count": len(rows)})

    competition["archive"]["submissions_status"] = "captured_from_csv"
    competition["archive"]["submissions_captured_at"] = captured_at
    competition["archive"]["submissions_counts"] = {
        "all": len(rows),
        "primaries": len(primaries),
        "rejected_primaries": len(rejected_primaries),
    }
    write_json(competition_path, competition)

    print(
        f"Imported {len(rows)} submissions for {args.slug}: "
        f"{len(primaries)} primaries, {len(rejected_primaries)} rejected primaries"
    )
    return 0


def import_validation_repo(args: argparse.Namespace) -> int:
    out = Path(args.out)
    competition_path = slug_dir(out, args.slug) / "competition.json"
    if not competition_path.exists():
        raise SystemExit(f"Missing competition.json for {args.slug}: {competition_path}")
    competition = json.loads(competition_path.read_text(encoding="utf-8"))
    competition_uid = competition.get("uid")
    captured_at = utc_now()
    validation_repo_url = args.validation_repo_url or derive_validation_repo_url(competition)
    repo_full_name = github_repo_full_name(validation_repo_url)

    tmp_root = Path(tempfile.mkdtemp(prefix="code4rena-validation-"))
    repo_dir = tmp_root / "validation"
    try:
        subprocess.run(
            ["git", "clone", "--depth", "1", validation_repo_url, str(repo_dir)],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            text=True,
        )
        data_dir = repo_dir / "data"
        if not data_dir.exists():
            raise SystemExit(f"Validation repo has no data directory: {validation_repo_url}")
        issue_metadata = fetch_github_issues(repo_full_name)
        submissions_dir = slug_dir(out, args.slug) / "submissions"
        raw_dir = submissions_dir / "raw"
        raw_dir.mkdir(parents=True, exist_ok=True)
        rows: list[dict[str, Any]] = []
        for json_path in sorted(data_dir.glob("*.json")):
            entry = json.loads(json_path.read_text(encoding="utf-8"))
            issue_id = int(entry["issueId"])
            issue = issue_metadata.get(issue_id)
            raw_detail_file = raw_dir / f"V-{issue_id}.md"
            body = (issue or {}).get("body") or ""
            raw_detail_file.write_text(body, encoding="utf-8")
            rows.append(
                normalize_validation_repo_row(
                    entry,
                    issue=issue,
                    slug=args.slug,
                    competition_uid=competition_uid,
                    captured_at=captured_at,
                    raw_detail_text_file=str(raw_detail_file.relative_to(out)),
                    validation_repo_url=validation_repo_url,
                )
            )
    except subprocess.CalledProcessError as error:
        raise SystemExit(f"Failed to clone {validation_repo_url}: {error.stderr.strip()}") from error
    finally:
        shutil.rmtree(tmp_root, ignore_errors=True)

    if args.hm_only:
        rows = [row for row in rows if row.get("claimed_severity") in {"High", "Medium"}]
    primaries = [row for row in rows if row["is_primary"] is True]
    rejected_primaries = [row for row in primaries if row["status"] != "accepted"]

    write_jsonl(submissions_dir / "all.jsonl", rows)
    write_jsonl(submissions_dir / "primaries.jsonl", primaries)
    write_jsonl(submissions_dir / "rejected_primaries.jsonl", rejected_primaries)
    write_json(
        raw_dir / "validation-repo.metadata.json",
        {
            "captured_at": captured_at,
            "source_validation_repo": validation_repo_url,
            "row_count": len(rows),
            "primary_count": len(primaries),
            "rejected_primary_count": len(rejected_primaries),
            "hm_only": args.hm_only,
        },
    )

    competition["archive"]["submissions_status"] = "captured_from_github_validation_repo"
    competition["archive"]["submissions_captured_at"] = captured_at
    competition["archive"]["submissions_counts"] = {
        "all": len(rows),
        "primaries": len(primaries),
        "rejected_primaries": len(rejected_primaries),
    }
    write_json(competition_path, competition)

    status_path = Path(args.status_path) if args.status_path else out / "authenticated-scrape-status.jsonl"
    status_path.parent.mkdir(parents=True, exist_ok=True)
    status_row = {
        "slug": args.slug,
        "status": "captured",
        "rows": len(primaries),
        "accepted": sum(1 for row in primaries if row["status"] == "accepted"),
        "rejected": sum(1 for row in primaries if row["status"] != "accepted"),
        "empty_reason": None if primaries else "no_primary_validation_rows",
        "detail_errors": 0,
        "pending_details": 0,
        "fetched_details": len(primaries),
        "source": "github_validation_repo",
        "started_at": captured_at,
        "finished_at": utc_now(),
    }
    with status_path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(status_row, sort_keys=True) + "\n")

    print(
        f"Imported {len(rows)} validation rows for {args.slug}: "
        f"{len(primaries)} primaries, {len(rejected_primaries)} rejected primaries"
    )
    return 0


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    rows: list[dict[str, Any]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def corpus_relative(path: Path, root: Path) -> str:
    return str(path.relative_to(root))


def sanitize_filename(value: str) -> str:
    cleaned = re.sub(r"[^A-Za-z0-9_.-]+", "-", value.strip())
    cleaned = cleaned.strip(".-")
    return cleaned or "item"


def normalize_space(value: str) -> str:
    return " ".join(str(value or "").split())


def markdown_escape_cell(value: Any) -> str:
    return normalize_space("" if value is None else str(value)).replace("|", "\\|")


def clean_title(value: Any) -> str:
    title = re.sub(r"^\s*[:\-–—]\s*", "", normalize_space("" if value is None else str(value)))
    return re.sub(r"\s*\(?modify old one due to poc\)?", "", title, flags=re.IGNORECASE).strip()


def source_url(row: dict[str, Any]) -> str | None:
    source = row.get("source")
    if isinstance(source, dict):
        return source.get("url")
    return row.get("source_report_url")


POC_SECTION_RE = re.compile(
    r"^(?:view detailed )?(?:(?:coded|written)\s+)?proof[-\s]*of[-\s]*concepts?\b|"
    r"^(?:coded\s+)?poc\b|"
    r"^(test case|manual reproduce|reproduce|reproduction steps?|attack scenario)\b",
    re.IGNORECASE,
)
POC_TOKEN_RE = re.compile(r"\b(?:poc|proof[-\s]*of[-\s]*concepts?)\b", re.IGNORECASE)
POC_SECTION_PREFIX_RE = re.compile(
    r"^(step[-\s]*by[-\s]*step|exploit|full|provided|expand)\b",
    re.IGNORECASE,
)
POC_REFERENCE_RE = re.compile(
    r"\s*(?:see\s+(?:the\s+)?|as\s+shown\s+in\s+(?:the\s+)?)"
    r"(?:(?:written\s+and\s+coded|coded|written)\s+)?(?:proof[-\s]*of[-\s]*concept|poc)[^.]*\.?",
    re.IGNORECASE,
)
INLINE_POC_BLOCK_RE = re.compile(
    r"(?is)\s*(?:but\s+|and\s+)?(?:here(?:'|’)?s\s+)?(?:a\s+)?"
    r"(?:provided|coded|written)\s+poc\b.*?"
    r"(?=\b[\w.-]+\s+\((?:judge|warden|sponsor)[^)]+\)\s+commented:|\n## |\Z)"
)
SUMMARY_CUTOFF_RE = re.compile(
    r"(?i)(```|(?:\*\*)?\b(?:proof[-\s]*of[-\s]*concepts?|poc|code snippets?|"
    r"reproduction|recommended mitigation|recommendation)\b)"
)
POST_POC_SECTION_RE = re.compile(
    r"^(recommended mitigation|recommended mitigation steps|mitigation|remediation|assessed type|"
    r"discussion|sponsor comment|judge comment|links? to affected code|impact|vulnerability details)\b",
    re.IGNORECASE,
)
TERMINAL_REPORT_SECTION_RE = re.compile(
    r"^(low risk|low-severity|gas optimizations|qa report|audit analysis|automated findings|"
    r"mitigation review|mitigation review scope|disclosures|scope|severity criteria|appendix)\b",
    re.IGNORECASE,
)
REPORT_SECTION_HEADING_RE = re.compile(
    r"^(vulnerability details|finding description and impact|impact|recommended mitigation|"
    r"recommended mitigation steps|mitigation|remediation|assessed type|links? to affected code|discussion)$",
    re.IGNORECASE,
)


def is_poc_section_start(clean: str) -> bool:
    if POC_SECTION_RE.match(clean):
        return True
    if not POC_TOKEN_RE.search(clean):
        return False
    if POC_SECTION_PREFIX_RE.match(clean):
        return True
    return len(clean.split()) <= 8


def finding_section_tokens(tokens: list[str]) -> dict[str, list[str]]:
    starts: dict[str, int] = {}
    boundary_indexes: list[int] = []
    for index, token in enumerate(tokens):
        heading = parse_finding_heading(token)
        if not heading:
            if TERMINAL_REPORT_SECTION_RE.match(normalize_space(token)):
                boundary_indexes.append(index)
            continue
        severity_code, number, _title = heading
        finding_id = f"{severity_code}-{int(number):02d}"
        starts[finding_id] = index
        boundary_indexes.append(index)

    sections: dict[str, list[str]] = {}
    sorted_boundaries = sorted(set(boundary_indexes))
    for finding_id, start in starts.items():
        end = len(tokens)
        for boundary_index in sorted_boundaries:
            if boundary_index > start:
                end = boundary_index
                break
        sections[finding_id] = tokens[start:end]
    return sections


def strip_poc_tokens(tokens: list[str]) -> tuple[list[str], bool]:
    stripped: list[str] = []
    skipping = False
    removed = False
    for token in tokens:
        clean = normalize_space(token)
        if not clean:
            continue
        if is_poc_section_start(clean):
            skipping = True
            removed = True
            continue
        if skipping and POST_POC_SECTION_RE.match(clean):
            skipping = False
        if skipping:
            continue
        stripped.append(clean)
    return stripped, removed


def tokens_to_markdown(tokens: list[str]) -> str:
    lines: list[str] = []
    paragraph: list[str] = []
    previous = ""

    def flush_paragraph() -> None:
        nonlocal paragraph
        if not paragraph:
            return
        text = normalize_space(" ".join(paragraph))
        if text:
            lines.extend([text, ""])
        paragraph = []

    for token in tokens:
        clean = normalize_space(token)
        if not clean:
            continue
        if clean == previous:
            continue
        previous = clean
        if parse_finding_heading(clean):
            continue
        if REPORT_SECTION_HEADING_RE.match(clean):
            flush_paragraph()
            lines.extend(["", f"## {clean}", ""])
        elif clean.startswith("http://") or clean.startswith("https://"):
            flush_paragraph()
            lines.append(f"- {clean}")
        else:
            paragraph.append(clean)
            if len(" ".join(paragraph)) > 700 or clean.endswith((".", ":", "?", "!")):
                flush_paragraph()
    flush_paragraph()
    return "\n".join(lines).strip() + "\n"


def scrub_poc_references(text: str) -> str:
    scrubbed = INLINE_POC_BLOCK_RE.sub("", text)
    scrubbed = POC_REFERENCE_RE.sub("", scrubbed)
    scrubbed = re.sub(r"\s+([,.;:])", r"\1", scrubbed)
    scrubbed = re.sub(r"\n{3,}", "\n\n", scrubbed)
    return scrubbed.strip() + ("\n" if scrubbed.strip() else "")


def accepted_finding_markdown(
    *,
    competition: dict[str, Any],
    finding: dict[str, Any],
    body_markdown: str,
    source_snapshot_path: str | None,
) -> str:
    source = finding.get("source_report_url") or competition.get("report_url")
    fields = [
        ("Contest", competition.get("title") or competition.get("slug")),
        ("Slug", competition.get("slug")),
        ("Finding ID", finding.get("id")),
        ("Severity", finding.get("severity")),
        ("Source URL", source),
        ("Source snapshot", source_snapshot_path),
    ]
    lines = [f"# [{finding.get('id')}] {clean_title(finding.get('title')) or 'Untitled finding'}", ""]
    lines.extend(f"- **{name}:** {value}" for name, value in fields if value)
    body = scrub_poc_references(body_markdown.strip() or normalize_space(finding.get("raw_excerpt", "")))
    lines.extend(["", body.strip(), ""])
    return "\n".join(lines)


def strip_markdown_poc(text: str) -> str:
    lines = text.splitlines()
    kept: list[str] = []
    skipping = False
    for line in lines:
        clean = line.strip()
        heading_text = re.sub(r"^#+\s*", "", clean).strip()
        if POC_SECTION_RE.match(heading_text):
            skipping = True
            continue
        if skipping and clean.startswith("#") and POST_POC_SECTION_RE.match(heading_text):
            skipping = False
        if not skipping:
            kept.append(line)
    return "\n".join(kept)


def strip_text_poc(text: str) -> str:
    lines = text.splitlines()
    kept: list[str] = []
    skipping = False
    for line in lines:
        clean = line.strip()
        if POC_SECTION_RE.match(clean):
            skipping = True
            continue
        if skipping and POST_POC_SECTION_RE.match(clean):
            skipping = False
        if not skipping:
            kept.append(line)
    return "\n".join(kept)


def rejection_source_text(row: dict[str, Any], corpus_root: Path) -> tuple[str, str | None]:
    if row.get("warden_comment"):
        return str(row["warden_comment"]), row.get("raw_detail_text_file")
    raw_detail = row.get("raw_detail_text_file")
    if raw_detail:
        raw_path = corpus_root / raw_detail
        if raw_path.exists():
            return raw_path.read_text(encoding="utf-8", errors="replace"), raw_detail
    return "", raw_detail


def brief_rejection_summary(row: dict[str, Any], source_text_value: str, *, max_chars: int) -> str:
    text = strip_text_poc(strip_markdown_poc(source_text_value))
    text = re.sub(r"```.*?```", " ", text, flags=re.DOTALL)
    text = SUMMARY_CUTOFF_RE.split(text, maxsplit=1)[0]
    summary_stop_re = re.compile(
        r"^(recommended mitigation|recommended mitigation steps|mitigation|tools used|assessed type|"
        r"links? to affected code|submissions touching same files|duplicates|code snippet)\b",
        re.IGNORECASE,
    )
    noise = {
        "news",
        "leaderboard",
        "submissions",
        "explore c4",
        "audits",
        "bounties",
        "reports",
        "support",
        "resources",
        "docs",
        "terms",
        "privacy",
        "view full submission",
        "duplicates",
        "tools used",
        "recommended mitigation steps",
        "recommended mitigation",
        "links to affected code",
        "submissions touching same files",
        "you must be logged in to view this page.",
    }
    parts: list[str] = []
    title = normalize_space(row.get("title", "")).lower()
    for raw in re.split(r"\n\s*\n|\n", text):
        clean = normalize_space(re.sub(r"^#+\s*", "", raw))
        if not clean:
            continue
        lowered = clean.lower()
        if summary_stop_re.match(clean):
            break
        if lowered in noise or lowered == title:
            continue
        if lowered.startswith("http://") or lowered.startswith("https://"):
            continue
        if lowered.startswith(("s-", "f-", "v-")) and len(clean.split()) <= 3:
            continue
        if len(clean) < 35 and not parts:
            continue
        parts.append(clean)
        if len(" ".join(parts)) >= max_chars:
            break
    summary = " ".join(parts).strip()
    summary = SUMMARY_CUTOFF_RE.split(summary, maxsplit=1)[0].strip()
    summary = re.sub(r"\s*\(?modify old one due to poc\)?", "", summary, flags=re.IGNORECASE).strip()
    summary = re.sub(r"\s*\(?modify old one due to\s*$", "", summary, flags=re.IGNORECASE).strip()
    if not summary:
        summary = clean_title(row.get("raw_row", {}).get("title") or row.get("title") or "No summary text captured.")
    if summary.lower() == "you must be logged in to view this page.":
        summary = clean_title(row.get("raw_row", {}).get("title") or row.get("title") or "No summary text captured.")
    if len(summary) > max_chars:
        summary = summary[: max_chars - 1].rstrip() + "..."
    return summary


def rejection_reason(row: dict[str, Any]) -> str:
    reason = row.get("rejection_reason_raw") or row.get("judge_comment")
    if reason:
        return normalize_space(reason)
    raw_row = row.get("raw_row") if isinstance(row.get("raw_row"), dict) else {}
    labels = raw_row.get("github_labels")
    if labels:
        return "GitHub validation labels: " + ", ".join(str(label) for label in labels)
    judge = raw_row.get("judge_evaluation")
    review = raw_row.get("review")
    return normalize_space("; ".join(str(value) for value in (judge, review) if value)) or "No explicit rejection reason captured."


def rejected_finding_markdown(
    *,
    competition: dict[str, Any],
    row: dict[str, Any],
    summary: str,
    raw_snapshot_path: str | None,
) -> str:
    fields = [
        ("Contest", competition.get("title") or competition.get("slug")),
        ("Slug", competition.get("slug")),
        ("Submission", row.get("submission_id")),
        ("Submitter", row.get("submitter")),
        ("Claimed severity", row.get("claimed_severity")),
        ("Final severity", row.get("final_severity")),
        ("Status", row.get("status")),
        ("Rejection category", row.get("rejection_reason_category")),
        ("Source URL", source_url(row)),
        ("Source snapshot", raw_snapshot_path),
    ]
    lines = [f"# {clean_title(row.get('title')) or 'Untitled rejected primary'}", ""]
    lines.extend(f"- **{name}:** {value}" for name, value in fields if value)
    lines.extend(["", "## Brief Summary", "", summary, "", "## Rejection Reason", "", rejection_reason(row), ""])
    return "\n".join(lines)


def update_rows_with_markdown_paths(path: Path, rows_by_id: dict[str, dict[str, Any]]) -> None:
    rows = read_jsonl(path)
    if not rows:
        return
    changed = False
    for row in rows:
        update = rows_by_id.get(str(row.get("submission_id")))
        if update:
            row.update(update)
            changed = True
    if changed:
        write_jsonl(path, rows)


def export_ground_truth_markdown(args: argparse.Namespace) -> int:
    out = Path(args.out)
    manifest_path = out / "manifest.jsonl"
    if not manifest_path.exists():
        raise SystemExit(f"Missing manifest: {manifest_path}")
    manifest_rows = read_jsonl(manifest_path)
    benchmark_rows: list[dict[str, Any]] = []
    totals = {
        "competitions": 0,
        "accepted_markdown": 0,
        "rejected_markdown": 0,
        "accepted_fallbacks": 0,
        "rejected_without_summary": 0,
    }

    for manifest_row in manifest_rows:
        slug = manifest_row["slug"]
        competition_dir = slug_dir(out, slug)
        competition_path = competition_dir / "competition.json"
        if not competition_path.exists():
            continue
        totals["competitions"] += 1
        competition = json.loads(competition_path.read_text(encoding="utf-8"))
        ground_truth_dir = competition_dir / "ground_truth"
        accepted_dir = ground_truth_dir / "accepted"
        rejected_dir = ground_truth_dir / "rejected"
        accepted_dir.mkdir(parents=True, exist_ok=True)
        rejected_dir.mkdir(parents=True, exist_ok=True)

        report_html_path = competition_dir / "final_report.html"
        report_tokens: list[str] = []
        sections: dict[str, list[str]] = {}
        if report_html_path.exists():
            report_tokens, _links = report_text_from_html(report_html_path.read_text(encoding="utf-8", errors="replace"))
            sections = finding_section_tokens(report_tokens)

        findings_path = competition_dir / "final_findings.json"
        findings = json.loads(findings_path.read_text(encoding="utf-8")) if findings_path.exists() else []
        archive = competition.get("archive") if isinstance(competition.get("archive"), dict) else {}
        submission_counts = archive.get("submissions_counts") if isinstance(archive.get("submissions_counts"), dict) else {}
        accepted_index_parts: list[str] = [f"# Accepted H/M Findings: {competition.get('title') or slug}", ""]
        if not findings:
            accepted_index_parts.extend(
                [
                    "No accepted High/Medium findings were parsed from a final report in the current corpus.",
                    "",
                    f"- **Accepted H/M count:** {len(findings)}",
                    f"- **Final report status:** {archive.get('final_report_status') or 'unknown'}",
                    f"- **Submission status:** {archive.get('submissions_status') or 'unknown'}",
                    "",
                ]
            )
        for finding in findings:
            finding_id = str(finding.get("id") or "")
            section = sections.get(finding_id, [])
            body_tokens, poc_removed = strip_poc_tokens(section)
            body_markdown = tokens_to_markdown(body_tokens)
            fallback_used = False
            if not body_markdown.strip():
                body_markdown = normalize_space(finding.get("raw_excerpt", ""))
                totals["accepted_fallbacks"] += 1
                fallback_used = True
            filename = f"{sanitize_filename(finding_id)}.md"
            markdown_path = accepted_dir / filename
            source_snapshot = corpus_relative(report_html_path, out) if report_html_path.exists() else None
            finding["title"] = clean_title(finding.get("title"))
            markdown = accepted_finding_markdown(
                competition=competition,
                finding=finding,
                body_markdown=body_markdown,
                source_snapshot_path=source_snapshot,
            )
            markdown_path.write_text(markdown, encoding="utf-8")
            local_path = corpus_relative(markdown_path, out)
            finding.update(
                {
                    "local_markdown_path": local_path,
                    "source_snapshot_path": source_snapshot,
                    "source_url": finding.get("source_report_url") or competition.get("report_url"),
                    "poc_stripped": True,
                    "poc_section_removed": poc_removed,
                    "markdown_fallback_used": fallback_used,
                }
            )
            accepted_index_parts.extend([markdown.strip(), ""])
            totals["accepted_markdown"] += 1
        write_json(findings_path, findings)

        rejected_rows = read_jsonl(competition_dir / "submissions" / "rejected_primaries.jsonl")
        rejected_index_parts: list[str] = [f"# Rejected Primary Findings: {competition.get('title') or slug}", ""]
        if not rejected_rows:
            rejected_index_parts.extend(
                [
                    "No rejected primary findings were captured for this contest in the current corpus.",
                    "",
                    f"- **Rejected primary count:** {len(rejected_rows)}",
                    f"- **Primary submission rows captured:** {submission_counts.get('primaries', 0)}",
                    f"- **Submission status:** {archive.get('submissions_status') or 'unknown'}",
                    "",
                ]
            )
        rejected_updates: dict[str, dict[str, Any]] = {}
        rejected_filename_counts: dict[str, int] = {}
        for row in rejected_rows:
            submission_id = str(row.get("submission_id") or row.get("submission_number") or "unknown")
            rejected_filename_counts[submission_id] = rejected_filename_counts.get(submission_id, 0) + 1
            filename_id = submission_id if rejected_filename_counts[submission_id] == 1 else f"{submission_id}-{rejected_filename_counts[submission_id]}"
            source_text_value, raw_snapshot = rejection_source_text(row, out)
            summary = brief_rejection_summary(row, source_text_value, max_chars=args.rejected_summary_chars)
            if summary == "No summary text captured.":
                totals["rejected_without_summary"] += 1
            markdown_path = rejected_dir / f"{sanitize_filename(filename_id)}.md"
            markdown = rejected_finding_markdown(
                competition=competition,
                row=row,
                summary=summary,
                raw_snapshot_path=raw_snapshot,
            )
            markdown_path.write_text(markdown, encoding="utf-8")
            local_path = corpus_relative(markdown_path, out)
            update = {
                "local_markdown_path": local_path,
                "source_snapshot_path": raw_snapshot,
                "source_url": source_url(row),
                "brief_summary": summary,
                "poc_stripped": True,
            }
            row.update(update)
            rejected_updates.setdefault(submission_id, update)
            rejected_index_parts.extend([markdown.strip(), ""])
            totals["rejected_markdown"] += 1
        write_jsonl(competition_dir / "submissions" / "rejected_primaries.jsonl", rejected_rows)
        update_rows_with_markdown_paths(competition_dir / "submissions" / "primaries.jsonl", rejected_updates)
        update_rows_with_markdown_paths(competition_dir / "submissions" / "all.jsonl", rejected_updates)
        accepted_markdown = "\n".join(accepted_index_parts).rstrip() + "\n"
        rejected_markdown = "\n".join(rejected_index_parts).rstrip() + "\n"
        (ground_truth_dir / "accepted_findings.md").write_text(accepted_markdown, encoding="utf-8")
        (ground_truth_dir / "rejected_primaries.md").write_text(rejected_markdown, encoding="utf-8")

        # Convenience copies in the competition root make the benchmark material
        # obvious while preserving final_report.html as the raw source snapshot.
        (competition_dir / "accepted_findings.md").write_text(accepted_markdown, encoding="utf-8")
        (competition_dir / "rejected_primaries.md").write_text(rejected_markdown, encoding="utf-8")
        (competition_dir / "benchmark_ground_truth.md").write_text(
            "\n".join(
                [
                    f"# Benchmark Ground Truth: {competition.get('title') or slug}",
                    "",
                    "## Accepted H/M Findings",
                    "",
                    accepted_markdown.strip(),
                    "",
                    "## Rejected Primary Findings",
                    "",
                    rejected_markdown.strip(),
                    "",
                ]
            ),
            encoding="utf-8",
        )

        index_lines = [
            f"# Ground Truth: {competition.get('title') or slug}",
            "",
            f"- **Slug:** {slug}",
            f"- **Repository:** {competition.get('repo_url') or ''}",
            f"- **Report URL:** {competition.get('report_url') or ''}",
            f"- **Accepted H/M markdown files:** {len(findings)}",
            f"- **Rejected primary markdown files:** {len(rejected_rows)}",
            "",
            "## Local Artifacts",
            "",
            "- `accepted_findings.md`",
            "- `rejected_primaries.md`",
            "- `accepted/`",
            "- `rejected/`",
            "",
        ]
        (ground_truth_dir / "index.md").write_text("\n".join(index_lines), encoding="utf-8")

        benchmark_rows.append(
            {
                "schema_version": SCHEMA_VERSION,
                "slug": slug,
                "uid": competition.get("uid"),
                "title": competition.get("title"),
                "audit_type": competition.get("audit_type"),
                "league": competition.get("league"),
                "repo_url": competition.get("repo_url"),
                "source_repositories_from_report": competition.get("source_repositories_from_report", []),
                "report_url": competition.get("report_url"),
                "findings_repo_url": competition.get("findings_repo_url"),
                "competition_json_path": corpus_relative(competition_path, out),
                "accepted_findings_markdown_path": corpus_relative(ground_truth_dir / "accepted_findings.md", out),
                "rejected_primaries_markdown_path": corpus_relative(ground_truth_dir / "rejected_primaries.md", out),
                "ground_truth_index_path": corpus_relative(ground_truth_dir / "index.md", out),
                "accepted_hm_count": len(findings),
                "rejected_primary_count": len(rejected_rows),
                "has_report_html": report_html_path.exists(),
                "has_primary_rows": (competition_dir / "submissions" / "primaries.jsonl").exists(),
                "has_rejected_rows": bool(rejected_rows),
                "markdown_parse_ok": len(findings) == 0 or any((accepted_dir / f"{sanitize_filename(str(item.get('id')))}.md").exists() for item in findings),
            }
        )

    write_jsonl(out / "benchmark_manifest.jsonl", benchmark_rows)
    write_json(out / "ground_truth_export_summary.json", {"schema_version": SCHEMA_VERSION, "generated_at": utc_now(), "counts": totals})
    print(
        "Exported ground-truth markdown: "
        f"{totals['accepted_markdown']} accepted findings, "
        f"{totals['rejected_markdown']} rejected primaries, "
        f"{totals['accepted_fallbacks']} accepted fallbacks"
    )
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    discover_parser = subparsers.add_parser("discover", help="Archive public Code4rena contest metadata and reports")
    discover_parser.add_argument("--out", default=str(DEFAULT_OUT))
    discover_parser.add_argument("--year-start", type=int, default=2024)
    discover_parser.add_argument("--year-end", type=int, default=2026)
    discover_parser.add_argument("--leagues", default=",".join(DEFAULT_EVM_LEAGUES))
    discover_parser.add_argument("--all-leagues", action="store_true", help="Include every Code4rena league/codebase type")
    discover_parser.add_argument("--audit-only", action="store_true", help="Exclude mitigation reviews")
    discover_parser.add_argument("--per-page", type=int, default=100)
    discover_parser.add_argument("--limit", type=int)
    discover_parser.add_argument("--force", action="store_true", help="Refetch report HTML even if already archived")
    discover_parser.set_defaults(func=discover)

    import_parser = subparsers.add_parser("import-submissions-csv", help="Normalize an authenticated Code4rena submissions CSV export")
    import_parser.add_argument("--out", default=str(DEFAULT_OUT))
    import_parser.add_argument("--slug", required=True)
    import_parser.add_argument("--csv", required=True)
    import_parser.add_argument("--encoding", default="utf-8-sig")
    import_parser.set_defaults(func=import_submissions_csv)

    validation_parser = subparsers.add_parser(
        "import-validation-repo",
        help="Normalize a public Code4rena GitHub validation repo into submission JSONL files",
    )
    validation_parser.add_argument("--out", default=str(DEFAULT_OUT))
    validation_parser.add_argument("--slug", required=True)
    validation_parser.add_argument("--validation-repo-url", help="Override the derived GitHub validation repo URL")
    validation_parser.add_argument("--status-path", help="Append a scrape status row to this JSONL path")
    validation_parser.add_argument(
        "--hm-only",
        action="store_true",
        help="Only import High/Medium validation rows; by default QA/Gas rows are retained in all.jsonl",
    )
    validation_parser.set_defaults(func=import_validation_repo)

    export_parser = subparsers.add_parser(
        "export-ground-truth-markdown",
        help="Write durable local markdown ground-truth files for accepted and rejected findings",
    )
    export_parser.add_argument("--out", default=str(DEFAULT_OUT))
    export_parser.add_argument(
        "--rejected-summary-chars",
        type=int,
        default=700,
        help="Maximum characters to keep in rejected-primary brief summaries",
    )
    export_parser.set_defaults(func=export_ground_truth_markdown)

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    return int(args.func(args))


if __name__ == "__main__":
    sys.exit(main())
