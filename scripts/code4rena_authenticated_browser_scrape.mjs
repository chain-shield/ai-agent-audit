import fs from "node:fs";
import path from "node:path";

const SCHEMA_VERSION = "code4rena-corpus-v1";

function ensureDir(dirPath) {
  fs.mkdirSync(dirPath, { recursive: true });
}

function writeJson(pathname, value) {
  ensureDir(path.dirname(pathname));
  fs.writeFileSync(pathname, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

function writeJsonl(pathname, rows) {
  ensureDir(path.dirname(pathname));
  fs.writeFileSync(
    pathname,
    rows.map((row) => JSON.stringify(row)).join("\n") + (rows.length ? "\n" : ""),
    "utf8",
  );
}

function readJson(pathname) {
  return JSON.parse(fs.readFileSync(pathname, "utf8"));
}

function normalizeTitle(title) {
  return String(title || "")
    .toLowerCase()
    .replace(/[`"'’“”]/g, "")
    .replace(/[^a-z0-9]+/g, " ")
    .trim();
}

function parseClaimedSeverity(detailText, fallback) {
  const marker = "Timeline\nSubmitted";
  const idx = detailText.indexOf(marker);
  if (idx < 0) return fallback || null;
  const tail = detailText
    .slice(idx)
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
  const severities = new Set(["Critical", "High", "Medium", "Low", "QA", "Gas", "Informational"]);
  for (const token of tail.slice(0, 12)) {
    if (severities.has(token)) return token;
  }
  return fallback || null;
}

function classifyReason(row, detailText, acceptedTitles) {
  const finalSeverity = String(row.final_severity || "").toLowerCase();
  const evalText = `${row.triage || ""}\n${row.review || ""}\n${row.judge_evaluation || ""}\n${detailText || ""}`.toLowerCase();
  const acceptedByTitle = acceptedTitles.has(normalizeTitle(row.title));
  const acceptedBySeverity =
    ["critical", "high", "medium"].includes(finalSeverity) && evalText.includes("valid");

  if (acceptedByTitle || acceptedBySeverity) return { status: "accepted", category: "accepted", reason: null };
  if (evalText.includes("duplicate")) {
    return { status: "duplicate", category: "duplicate", reason: "Marked duplicate in authenticated Code4rena submission detail/table." };
  }
  if (evalText.includes("out of scope") || /\boos\b/.test(evalText)) {
    return { status: "out_of_scope", category: "out_of_scope", reason: "Marked out of scope in authenticated Code4rena submission detail/table." };
  }
  if (evalText.includes("known issue")) {
    return { status: "invalid", category: "known_issue", reason: "Marked known issue in authenticated Code4rena submission detail/table." };
  }
  if (evalText.includes("insufficient poc") || evalText.includes("missing poc")) {
    return { status: "invalid", category: "insufficient_poc", reason: "Marked insufficient PoC in authenticated Code4rena submission detail/table." };
  }
  if (evalText.includes("invalid") || evalText.includes("disputed")) {
    return { status: "invalid", category: "other", reason: `Final severity/evaluation: ${row.final_severity}; ${row.judge_evaluation || ""}` };
  }
  if (["low", "qa", "gas", "informational", "info"].includes(finalSeverity)) {
    return { status: "low_or_qa", category: "low_or_qa_severity", reason: `Final severity/evaluation: ${row.final_severity}; ${row.judge_evaluation || ""}` };
  }
  return { status: "unknown", category: "unknown", reason: `Final severity/evaluation: ${row.final_severity || ""}; ${row.judge_evaluation || ""}` };
}

function parseSubmissionNumber(submissionId) {
  const match = String(submissionId || "").match(/\d+/);
  return match ? Number(match[0]) : null;
}

function dedupeRows(rows) {
  const bySubmissionId = new Map();
  for (const row of rows) {
    if (!bySubmissionId.has(row.submission_id)) {
      bySubmissionId.set(row.submission_id, row);
    }
  }
  return [...bySubmissionId.values()];
}

function hasSubmissionDetailContent(bodyText) {
  const text = String(bodyText || "");
  return text.includes("Timeline\nSubmitted") || text.includes("\nTitle\n") || text.length > 1000;
}

function pageNumber(bodyText) {
  const match = bodyText.match(/Page\s+(\d+)\s+of\s+(\d+)/i);
  if (!match) return null;
  return { current: Number(match[1]), last: Number(match[2]) };
}

function isLoginPage(bodyText) {
  return bodyText.includes("You must be logged in") || bodyText.includes("Log in / Register");
}

function isBlockedPage(bodyText) {
  const normalized = String(bodyText || "").trim().toLowerCase();
  return normalized === "403 forbidden" || normalized.startsWith("403 forbidden\n");
}

function isRestrictedPage(bodyText) {
  const normalized = String(bodyText || "").toLowerCase();
  return (
    normalized.includes("access to submissions is restricted") ||
    normalized.includes("submissions for this audit are hidden because the code is live")
  );
}

function isSubmissionsRouteUnavailablePage(pageSignals) {
  const signals =
    typeof pageSignals === "string"
      ? { bodyText: pageSignals }
      : {
          bodyText: pageSignals?.bodyText || "",
          title: pageSignals?.title || "",
          pathname: pageSignals?.pathname || "",
          isNextError: Boolean(pageSignals?.isNextError),
          hasNextNotFound: Boolean(pageSignals?.hasNextNotFound),
        };
  const text = `${signals.bodyText}\n${signals.title}\n${signals.pathname}`;
  return (
    (text.includes("Recent news") &&
      text.includes("The $") &&
      !text.includes("Filter\nPrimary submissions") &&
      !/Including\s+\d+\s+submissions?/i.test(text)) ||
    (signals.isNextError && signals.hasNextNotFound && /\/audits\/[^/]+\/submissions\b/.test(signals.pathname)) ||
    (signals.hasNextNotFound && /Submissions\s+\|\s+Code4rena/i.test(signals.title))
  );
}

function isExplicitEmptyPage(bodyText) {
  return /no submissions|no results|there are no submissions matching/i.test(String(bodyText || ""));
}

async function currentPageSignals(tab, fallbackBodyText = "") {
  return tab.playwright
    .evaluate(
      (bodyText) => {
        const html = document.documentElement?.outerHTML || "";
        return {
          bodyText: document.body?.innerText || bodyText || "",
          title: document.title || "",
          pathname: window.location?.pathname || "",
          isNextError: document.documentElement?.id === "__next_error__",
          hasNextNotFound: html.includes("NEXT_NOT_FOUND"),
        };
      },
      fallbackBodyText,
      { timeoutMs: 5000 },
    )
    .catch(() => ({ bodyText: fallbackBodyText }));
}

async function throwIfSubmissionsRouteUnavailable(tab, slug, bodyText) {
  const signals = await currentPageSignals(tab, bodyText);
  if (isSubmissionsRouteUnavailablePage(signals)) {
    throw new Error(`Submissions route unavailable for ${slug}: Code4rena redirected to news/not-found page`);
  }
}

async function scrapePrimaryRows(tab, slug, { perPageWaitMs, maxPages }) {
  const rows = [];
  let page = 1;
  await tab.goto(`https://code4rena.com/audits/${slug}/submissions?page=1&filter=primaries`);
  while (true) {
    await tab.playwright.waitForLoadState({ state: "domcontentloaded", timeoutMs: 20000 }).catch(() => {});
    await tab.playwright.waitForTimeout(perPageWaitMs);
    let bodyText = await tab.playwright.locator("body").innerText({ timeout: 10000 }).catch(() => "");
    if (isLoginPage(bodyText)) throw new Error(`Not logged in for ${slug}`);
    if (isBlockedPage(bodyText)) throw new Error(`Code4rena returned 403 Forbidden for ${slug}`);
    if (isRestrictedPage(bodyText)) throw new Error(`Restricted submissions for ${slug}: live-code audit access is hidden`);
    await throwIfSubmissionsRouteUnavailable(tab, slug, bodyText);
    let expectedSubmissions = null;
    let sawExplicitEmptyState = false;

    for (let attempt = 0; attempt < 12; attempt += 1) {
      const linkCount = await tab.playwright
        .evaluate(() => document.querySelectorAll('a[href*="/submissions/F-"]').length)
        .catch(() => 0);
      if (linkCount > 0) break;
      bodyText = await tab.playwright.locator("body").innerText({ timeout: 10000 }).catch(() => "");
      if (isLoginPage(bodyText)) throw new Error(`Not logged in for ${slug}`);
      if (isBlockedPage(bodyText)) throw new Error(`Code4rena returned 403 Forbidden for ${slug}`);
      if (isRestrictedPage(bodyText)) throw new Error(`Restricted submissions for ${slug}: live-code audit access is hidden`);
      await throwIfSubmissionsRouteUnavailable(tab, slug, bodyText);
      const included = bodyText.match(/Including\s+(\d+)\s+submissions?/i);
      if (included) expectedSubmissions = Number(included[1]);
      if (included && expectedSubmissions === 0) {
        sawExplicitEmptyState = true;
        break;
      }
      if (isExplicitEmptyPage(bodyText)) {
        sawExplicitEmptyState = true;
        break;
      }
      await tab.playwright.waitForTimeout(500);
    }

    const pageRows = await tab.playwright.evaluate(() => {
      const bodyText = document.body?.innerText || "";
      const hasTriageColumn = /\nTriage\nReview\nJudge\nAuthor\nSubmitted\b/.test(bodyText);
      return [...document.querySelectorAll(".submission-row")]
        .map((row) => {
          const cells = [...row.children].map((child) => child.innerText.trim());
          const link = row.querySelector('a[href*="/submissions/F-"]');
          const offset = hasTriageColumn ? 1 : 0;
          return {
            submission_id: cells[1] || null,
            final_severity: cells[2] || null,
            title: cells[3] || link?.textContent?.trim() || null,
            triage: hasTriageColumn ? cells[4] || null : null,
            review: cells[4 + offset] || null,
            judge_evaluation: cells[5 + offset] || null,
            submitter: cells[6 + offset] || null,
            submitted_at_label: cells[7 + offset] || null,
            href: link?.href || null,
            raw_table_text: row.innerText || "",
          };
        })
        .filter((row) => row.href && row.submission_id);
    });
    bodyText = await tab.playwright.locator("body").innerText({ timeout: 10000 }).catch(() => bodyText);
    if (pageRows.length === 0 && isExplicitEmptyPage(bodyText)) {
      sawExplicitEmptyState = true;
    }
    if (expectedSubmissions && pageRows.length === 0) {
      throw new Error(`Expected ${expectedSubmissions} submissions for ${slug} page ${page}, but no submission rows rendered`);
    }
    if (pageRows.length === 0 && !sawExplicitEmptyState) {
      throw new Error(`No submission rows rendered for ${slug} page ${page}, and no explicit empty state was found`);
    }
    rows.push(...pageRows);
    const pageInfo = pageNumber(bodyText);
    if (!pageInfo || pageInfo.current >= pageInfo.last) break;
    if (maxPages && page >= maxPages) break;
    await tab.playwright.locator("button.pagination-row__button").nth(2).click({ timeoutMs: 10000 });
    await tab.playwright.waitForTimeout(perPageWaitMs);
    page += 1;
  }
  return dedupeRows(rows);
}

function normalizedSubmission({ row, detailText, detailFile, detailError, competition, slug, acceptedTitles, corpusRoot }) {
  const classification = classifyReason(row, detailText, acceptedTitles);
  return {
    schema_version: SCHEMA_VERSION,
    competition_slug: slug,
    competition_uid: competition.uid,
    submission_uid: null,
    submission_id: row.submission_id,
    submission_number: parseSubmissionNumber(row.submission_id),
    title: row.title,
    claimed_severity: parseClaimedSeverity(detailText, row.final_severity),
    final_severity: row.final_severity,
    status: classification.status,
    is_primary: /primary/i.test(`${row.judge_evaluation}\n${detailText}`),
    accepted_group_id: classification.status === "accepted" ? row.submission_id : null,
    duplicate_of: null,
    submitter: row.submitter,
    affected_contracts: [],
    affected_functions: [],
    judge_comment: null,
    sponsor_comment: null,
    warden_comment: null,
    rejection_reason_raw: classification.reason,
    rejection_reason_category: classification.category,
    confidence: classification.status === "accepted" ? "high" : "medium",
    detail_capture_error: detailError,
    raw_row: row,
    raw_detail_text_file: path.relative(corpusRoot, detailFile),
    source: { kind: "code4rena_authenticated_browser", url: row.href, captured_at: new Date().toISOString() },
  };
}

function isUsableDetailText(detailText) {
  return detailText && !isLoginPage(detailText) && !isBlockedPage(detailText) && hasSubmissionDetailContent(detailText);
}

async function archiveCompetition({
  browser,
  corpusRoot,
  slug,
  detailWaitMs,
  perPageWaitMs,
  skipDetails,
  maxPages,
  maxDetails,
}) {
  const competitionDir = path.join(corpusRoot, "competitions", slug);
  const competitionPath = path.join(competitionDir, "competition.json");
  const competition = readJson(competitionPath);
  const findingsPath = path.join(competitionDir, "final_findings.json");
  const acceptedTitles = new Set(
    fs.existsSync(findingsPath)
      ? readJson(findingsPath).map((finding) => normalizeTitle(finding.title)).filter(Boolean)
      : [],
  );
  const submissionsDir = path.join(competitionDir, "submissions");
  const rawDir = path.join(submissionsDir, "raw");
  ensureDir(rawDir);
  const tab = await browser.tabs.new();
  try {
    const rows = await scrapePrimaryRows(tab, slug, { perPageWaitMs, maxPages });
    const existingRawDetails = fs.existsSync(rawDir)
      ? fs.readdirSync(rawDir).filter((filename) => /^F-\d+\.txt$/.test(filename)).length
      : 0;
    if (rows.length === 0 && existingRawDetails > 0) {
      throw new Error(`Refusing to overwrite ${existingRawDetails} existing raw submissions for ${slug} with zero rendered rows`);
    }
    const normalized = [];
    let detailErrors = 0;
    let pendingDetails = 0;
    let fetchedDetails = 0;
    for (const row of rows) {
      let detailText = "";
      let detailError = null;
      const detailFile = path.join(rawDir, `${row.submission_id}.txt`);
      if (!skipDetails) {
        if (fs.existsSync(detailFile)) {
          detailText = fs.readFileSync(detailFile, "utf8");
        }
        if (!isUsableDetailText(detailText)) {
          if (maxDetails && fetchedDetails >= maxDetails) {
            detailText = "";
            detailError = "detail_capture_pending";
            pendingDetails += 1;
          } else {
            await tab.goto(row.href);
            await tab.playwright.waitForLoadState({ state: "domcontentloaded", timeoutMs: 20000 }).catch(() => {});
            await tab.playwright.waitForTimeout(detailWaitMs);
            for (let attempt = 0; attempt < 10; attempt += 1) {
              detailText = await tab.playwright.locator("body").innerText({ timeout: 10000 }).catch(() => "");
              if (isLoginPage(detailText)) {
                detailError = `Not logged in for ${slug} detail ${row.submission_id}`;
              } else if (isBlockedPage(detailText)) {
                detailError = `Code4rena returned 403 Forbidden for ${slug} detail ${row.submission_id}`;
              } else if (hasSubmissionDetailContent(detailText)) {
                detailError = null;
                break;
              } else {
                detailError = `Detail content did not finish rendering for ${slug} detail ${row.submission_id}`;
              }
              await tab.playwright.waitForTimeout(500);
            }
            fetchedDetails += 1;
            fs.writeFileSync(detailFile, detailText, "utf8");
          }
        }
        if (detailError) detailErrors += 1;
      }
      normalized.push(
        normalizedSubmission({ row, detailText, detailFile, detailError, competition, slug, acceptedTitles, corpusRoot }),
      );
    }
    writeJsonl(path.join(submissionsDir, "all.jsonl"), normalized);
    writeJsonl(path.join(submissionsDir, "primaries.jsonl"), normalized);
    writeJsonl(path.join(submissionsDir, "rejected_primaries.jsonl"), normalized.filter((row) => row.status !== "accepted"));
    const resultStatus = pendingDetails > 0 ? "partial" : "captured";
    competition.archive.submissions_status =
      resultStatus === "partial" ? "partial_from_authenticated_browser" : "captured_from_authenticated_browser";
    competition.archive.submissions_captured_at = new Date().toISOString();
    competition.archive.submissions_counts = {
      all: normalized.length,
      primaries: normalized.length,
      rejected_primaries: normalized.filter((row) => row.status !== "accepted").length,
      pending_details: pendingDetails,
    };
    writeJson(competitionPath, competition);
    return {
      slug,
      status: resultStatus,
      rows: normalized.length,
      accepted: normalized.filter((row) => row.status === "accepted").length,
      rejected: normalized.filter((row) => row.status !== "accepted").length,
      empty_reason: normalized.length === 0 ? "explicit_empty_state" : null,
      detail_errors: detailErrors,
      pending_details: pendingDetails,
      fetched_details: fetchedDetails,
    };
  } finally {
    await tab.close().catch(() => {});
  }
}

export async function scrapeAuthenticatedPrimaries(options = {}) {
  const {
    browser = globalThis.browser,
    corpusRoot = "benchmarks/code4rena-corpus",
    slugs = null,
    limit = null,
    detailWaitMs = 800,
    perPageWaitMs = 1000,
    contestPauseMs = 0,
    skipDetails = false,
    maxPages = null,
    maxDetails = null,
    statusPath = path.join(corpusRoot, "authenticated-scrape-status.jsonl"),
  } = options;
  if (!browser) throw new Error("globalThis.browser is not initialized");
  const manifestRows = fs
    .readFileSync(path.join(corpusRoot, "manifest.jsonl"), "utf8")
    .split("\n")
    .filter(Boolean)
    .map((line) => JSON.parse(line));
  const allowed = slugs ? new Set(slugs) : null;
  const selected = manifestRows.filter((row) => !allowed || allowed.has(row.slug)).slice(0, limit || undefined);
  ensureDir(path.dirname(statusPath));
  const results = [];
  for (const row of selected) {
    const startedAt = new Date().toISOString();
    let result;
    try {
      result = await archiveCompetition({
        browser,
        corpusRoot,
        slug: row.slug,
        detailWaitMs,
        perPageWaitMs,
        skipDetails,
        maxPages,
        maxDetails,
      });
    } catch (error) {
      const message = error?.message || String(error);
      const status = message.includes("Restricted submissions")
        ? "restricted"
        : message.includes("Submissions route unavailable")
          ? "restricted"
        : message.includes("403 Forbidden")
          ? "blocked"
          : "error";
      result = { slug: row.slug, status, error: message };
    }
    result.started_at = startedAt;
    result.finished_at = new Date().toISOString();
    fs.appendFileSync(statusPath, `${JSON.stringify(result)}\n`, "utf8");
    results.push(result);
    if (contestPauseMs && results.length < selected.length) {
      await new Promise((resolve) => setTimeout(resolve, contestPauseMs));
    }
  }
  return {
    selected: selected.length,
    captured: results.filter((row) => row.status === "captured").length,
    errors: results.filter((row) => row.status === "error").length,
    rows: results.reduce((sum, row) => sum + (row.rows || 0), 0),
    results,
  };
}
