//! Code4rena bug-bounty page ingestion.
//!
//! This module is intentionally narrow: it turns a public Code4rena bounty
//! listing into structured data that the rest of `prepare_code` can use to
//! clone repositories, generate audit scope/docs, and build a dynamic severity
//! rubric.  It does **not** decide whether a finding is valid; it only preserves
//! the bounty page's source-of-truth material with enough structure for later
//! analysis stages.
//!
//! NatSpec-style contract for this module:
//! - `@notice` Fetch and parse Code4rena bounty listings for smart-contract
//!   audit preparation.
//! - `@dev` Scope links may point to GitHub repos, GitHub blobs, explorer
//!   addresses, deployment pages, or plain text. Keep those links intact here;
//!   path/address resolution happens later in `audit_context` and `git_clone`.
//! - `@custom:invariant` Program-specific rules override default C4 bounty
//!   guidance, so parsed custom sections must be preserved verbatim enough for
//!   downstream prompts.
//! - `@custom:safety` Wrapper repos such as `code-423n4/<project>-bug-bounty`
//!   are useful documentation sources but should not win over the actual
//!   protocol source repository when both are present.

use anyhow::{Context, Result, anyhow};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::prepare_code::immunefi::{
    ResolvedGitCodebase, github_codebase_urls_from_text, merge_git_codebase_candidates,
};

const CODE4RENA_HTTP_TIMEOUT_SECS: u64 = 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Parsed representation of a single Code4rena bounty page.
///
/// The struct deliberately stores both structured slices (`scope_assets`,
/// `rewards`, rule sections) and `full_text`. Code4rena bounty pages are not a
/// stable API, so retaining the cleaned page text gives context generation a
/// fallback when the extraction heuristics miss a newly styled section.
pub struct Code4renaBountyData {
    /// User-provided URL, preserved for traceability in generated artifacts.
    pub input_url: String,
    /// Canonical `/bounties/<slug>` URL after normalization.
    pub url: String,
    /// URL slug used as a stable artifact/workspace name when no repo name is available.
    pub slug: String,
    /// Human-readable project name extracted from the page title/body.
    pub project: String,
    /// Short project description when the page exposes one.
    pub summary: Option<String>,
    /// Maximum advertised bounty text, kept as display text because pages may mix units.
    pub max_bounty: Option<String>,
    /// Reward rows extracted from the page's bounty/severity table.
    pub rewards: Vec<Code4renaReward>,
    /// All anchor links found on the page after URL resolution.
    pub links: Vec<Code4renaLink>,
    /// Smart-contract scope assets extracted from likely scope sections.
    pub scope_assets: Vec<Code4renaScopeAsset>,
    /// Program-specific severity criteria, if present.
    pub severity_section: Option<String>,
    /// Program-specific rules/prohibited activity text.
    pub rules_section: Option<String>,
    /// Program-specific out-of-scope text.
    pub out_of_scope_section: Option<String>,
    /// Known issue or accepted-risk text from the bounty page.
    pub known_issues_section: Option<String>,
    /// Previous audit links/text, used to avoid rediscovering excluded findings.
    pub previous_audits_section: Option<String>,
    /// Cleaned text of the full page for fallback context and debugging.
    pub full_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// One Code4rena reward row, represented as page text rather than normalized USD.
pub struct Code4renaReward {
    /// Severity label as displayed by Code4rena or the sponsor.
    pub severity: String,
    /// Payout/range text as displayed by the page.
    pub payout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Anchor link extracted from the bounty page.
pub struct Code4renaLink {
    /// Human-readable anchor label after HTML cleanup.
    pub label: String,
    /// Absolute URL after resolving relative links against the bounty origin.
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Candidate smart-contract asset listed by the bounty page.
///
/// Assets can be source repositories, direct Solidity blobs, explorer URLs, or
/// deployment pages. Downstream code decides how strict mapping must be based on
/// the URL type.
pub struct Code4renaScopeAsset {
    /// Display label or nearby row text for the asset.
    pub label: String,
    /// URL or raw asset token found in the scope section.
    pub url: String,
    /// Text section that produced the asset, useful for auditability.
    pub source_section: String,
}

/// Fetches and parses a Code4rena bounty listing.
///
/// `@notice` The returned data is the canonical raw material for Code4rena
/// bounty context generation.
/// `@dev` This performs only a single page fetch. Additional linked docs,
/// deployment pages, and explorer metadata are followed later by
/// `audit_context`, where scope/dedup rules can be applied consistently.
pub async fn fetch_code4rena_bounty(input_url: &str) -> Result<Code4renaBountyData> {
    let url = normalize_code4rena_bounty_url(input_url)?;
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(CODE4RENA_HTTP_TIMEOUT_SECS))
        .user_agent("ai-agent-audit-code4rena-bounty-context/0.1")
        .build()?;
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        anyhow::bail!(
            "Code4rena bounty fetch failed with status {} for {}",
            response.status(),
            url
        );
    }
    let html = response.text().await?;
    parse_code4rena_bounty_data(input_url, &url, &html)
}

/// Normalizes any supported Code4rena bounty URL to `/bounties/<slug>`.
///
/// `@dev` Query strings, hash fragments, and trailing path components are not
/// part of bounty identity and are intentionally dropped.
pub fn normalize_code4rena_bounty_url(input_url: &str) -> Result<String> {
    let input = input_url.trim().trim_end_matches('/');
    let captures = Regex::new(r#"(?i)^(https?://[^/]+)/bounties/([^/#?]+)"#)
        .unwrap()
        .captures(input)
        .ok_or_else(|| {
            anyhow!(
                "Invalid Code4rena bounty URL `{input_url}`; expected https://code4rena.com/bounties/<slug>"
            )
        })?;
    let origin = captures
        .get(1)
        .map(|m| m.as_str())
        .unwrap_or("https://code4rena.com");
    let slug = captures
        .get(2)
        .map(|m| m.as_str())
        .context("missing Code4rena bounty slug")?;
    Ok(format!("{origin}/bounties/{slug}"))
}

/// Extracts the bounty slug from a canonical or near-canonical Code4rena URL.
pub fn code4rena_slug_from_url(url: &str) -> Option<&str> {
    let after = url.split("/bounties/").nth(1)?;
    after
        .split(['/', '#', '?'])
        .find(|segment| !segment.is_empty())
}

/// Parses Code4rena bounty HTML into structured bounty data.
///
/// `@notice` This is the offline/testable entry point used by unit tests.
/// `@dev` The parser is heuristic because Code4rena pages are rendered HTML, not
/// a stable machine API. The implementation favors preserving evidence over
/// aggressively normalizing it.
pub fn parse_code4rena_bounty_data(
    input_url: &str,
    normalized_url: &str,
    html: &str,
) -> Result<Code4renaBountyData> {
    // The parser starts with broad page normalization, then extracts narrow
    // sections from the cleaned text. Keeping both forms lets the app recover
    // when Code4rena changes section labels without silently discarding context.
    let slug = code4rena_slug_from_url(normalized_url)
        .unwrap_or("code4rena-bounty")
        .to_string();
    let links = extract_code4rena_links(html, normalized_url);
    let full_text = html_to_readable_text(html);
    let project = extract_project_title(&full_text).unwrap_or_else(|| slug.clone());
    let summary = extract_project_summary(&full_text, &project);
    let max_bounty = extract_max_bounty(&full_text);
    let severity_section = section_between(
        &full_text,
        &[
            "Scope & Severity Criteria",
            "Scope and Severity Criteria",
            "Severity matrix",
            "Severity Criteria",
        ],
        &["Out-of-Scope", "Out of Scope"],
    );
    let rules_section = section_between(
        &full_text,
        &["Specific Types of Issues", "Prohibited Activities"],
        &["Additional Context", "Miscellaneous"],
    );
    let out_of_scope_section = section_between(
        &full_text,
        &["Out-of-Scope", "Out of Scope"],
        &["Additional Context"],
    );
    let known_issues_section = section_between(
        &full_text,
        &["Known Issues"],
        &[
            "Previous Audits",
            "Previous audits",
            "Specific Types of Issues",
        ],
    );
    let previous_audits_section = section_between(
        &full_text,
        &["Previous Audits", "Previous audits"],
        &[
            "Specific Types of Issues",
            "Prohibited Activities",
            "Additional Context",
        ],
    );
    let scope_section = section_between(
        &full_text,
        &[
            "Smart Contracts in Scope",
            "Smart Contracts and Repos in Scope",
            "Smart Contract Deployments",
        ],
        &["Websites and Apps in Scope", "Out-of-Scope", "Out of Scope"],
    )
    .or_else(|| severity_section.clone())
    .unwrap_or_else(|| full_text.clone());
    let scope_assets = extract_scope_assets(&scope_section, &links);
    let rewards = extract_rewards(&full_text);

    Ok(Code4renaBountyData {
        input_url: input_url.to_string(),
        url: normalized_url.to_string(),
        slug,
        project,
        summary,
        max_bounty,
        rewards,
        links,
        scope_assets,
        severity_section,
        rules_section,
        out_of_scope_section,
        known_issues_section,
        previous_audits_section,
        full_text,
    })
}

impl Code4renaBountyData {
    /// Returns the GitHub codebases that should be cloned for this bounty.
    ///
    /// `@notice` This is the strict repo-derivation API used by
    /// `git_clone`.
    /// `@dev` It fails when no cloneable GitHub repo can be found because a
    /// bounty audit against the wrong source tree is worse than stopping and
    /// asking for a manual `repo`.
    pub fn resolved_git_codebases(&self) -> Result<Vec<ResolvedGitCodebase>> {
        let candidates = self.git_codebase_candidates();
        if candidates.is_empty() {
            anyhow::bail!(
                "Code4rena bounty `{}` did not expose a cloneable GitHub source repo. Specify `repo` manually.",
                self.project
            );
        }
        merge_git_codebase_candidates(&self.project, candidates)
    }

    /// Collects raw GitHub repo/tree/blob candidates from scope first, then links.
    ///
    /// `@dev` Scope assets are higher-confidence than generic page links. If
    /// both a wrapper bounty repository and a protocol repository are present,
    /// the protocol repository wins unless the wrapper is the only evidence.
    pub fn git_codebase_candidates(&self) -> Vec<ResolvedGitCodebase> {
        let mut scoped_candidates = Vec::new();
        for asset in &self.scope_assets {
            scoped_candidates.extend(github_codebase_urls_from_text(&asset.url));
            scoped_candidates.extend(github_codebase_urls_from_text(&asset.label));
        }

        if !scoped_candidates.is_empty() {
            let non_wrapper = scoped_candidates
                .iter()
                .filter(|candidate| !is_code4rena_wrapper_repo(&candidate.repo_url))
                .cloned()
                .collect::<Vec<_>>();
            return if non_wrapper.is_empty() {
                scoped_candidates
            } else {
                non_wrapper
            };
        }

        let all_candidates = self
            .links
            .iter()
            .flat_map(|link| {
                let mut out = github_codebase_urls_from_text(&link.url);
                out.extend(github_codebase_urls_from_text(&link.label));
                out
            })
            .collect::<Vec<_>>();
        let non_wrapper = all_candidates
            .iter()
            .filter(|candidate| !is_code4rena_wrapper_repo(&candidate.repo_url))
            .cloned()
            .collect::<Vec<_>>();
        if non_wrapper.is_empty() {
            all_candidates
        } else {
            non_wrapper
        }
    }

    /// Reports whether the parser found any explicit scope assets.
    ///
    /// `@dev` A repo-level GitHub asset counts as structured, but later scope
    /// generation may still fall back to configured code folders when no
    /// precise `.sol` or explorer mapping is required.
    pub fn has_structured_scope_assets(&self) -> bool {
        !self.scope_assets.is_empty()
    }

    /// Renders a compact markdown evidence bundle for context generation.
    ///
    /// `@notice` This markdown is meant for LLM workers, not for final reports.
    /// It preserves the page's smart-contract scope, reward/rule snippets, and
    /// relevant links without bringing in Code4rena navigation noise.
    pub fn context_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# Code4rena Bounty - {}\n\n", self.project));
        out.push_str(&format!("Source: {}\n\n", self.url));
        if let Some(summary) = &self.summary {
            out.push_str("## Program Overview\n\n");
            out.push_str(summary);
            out.push_str("\n\n");
        }
        if let Some(max_bounty) = &self.max_bounty {
            out.push_str(&format!("- Maximum bounty: {max_bounty}\n"));
        }
        if !self.rewards.is_empty() {
            out.push_str("\n## Rewards\n\n");
            for reward in &self.rewards {
                out.push_str(&format!("- {}: {}\n", reward.severity, reward.payout));
            }
        }
        out.push_str("\n## Smart Contract Scope Assets\n\n");
        if self.scope_assets.is_empty() {
            out.push_str("- No structured smart-contract scope assets were extracted. Read the bounty page directly.\n");
        } else {
            for asset in &self.scope_assets {
                out.push_str(&format!("- {}: {}\n", asset.label, asset.url));
            }
        }
        out.push('\n');
        append_optional_section(
            &mut out,
            "Scope And Severity Criteria",
            &self.severity_section,
        );
        append_optional_section(&mut out, "Known Issues", &self.known_issues_section);
        append_optional_section(&mut out, "Previous Audits", &self.previous_audits_section);
        append_optional_section(&mut out, "Out Of Scope", &self.out_of_scope_section);
        append_optional_section(&mut out, "Program Rules", &self.rules_section);

        out.push_str("## Relevant Links\n\n");
        for link in &self.links {
            if relevant_link_for_bounty_context(link) {
                out.push_str(&format!("- {}: {}\n", link.label, link.url));
            }
        }
        out
    }
}

fn is_code4rena_wrapper_repo(repo_url: &str) -> bool {
    // Code4rena frequently links a README-oriented wrapper repository from the
    // `code-423n4` org. That repo can be valuable context, but it should not be
    // selected as the clone target when the real protocol source repo is also
    // present in scope.
    repo_url
        .trim_end_matches('/')
        .to_ascii_lowercase()
        .starts_with("https://github.com/code-423n4/")
}

fn relevant_link_for_bounty_context(link: &Code4renaLink) -> bool {
    // Keep links that can affect smart-contract scope, rules, severity, or
    // prior-issue review. Drop platform chrome so Codex workers do not waste
    // browsing budget on login/navigation/marketing links.
    let haystack = format!("{} {}", link.label, link.url).to_ascii_lowercase();
    [
        "github.com",
        "docs",
        "audit",
        "scope",
        "contract",
        "etherscan",
        "basescan",
        "arbiscan",
        "optimistic.etherscan",
        "moonscan",
        "kitescan",
    ]
    .iter()
    .any(|marker| haystack.contains(marker))
        && ![
            "twitter",
            "x.com",
            "discord",
            "media kit",
            "privacy",
            "terms",
            "log in",
            "submission",
        ]
        .iter()
        .any(|marker| haystack.contains(marker))
}

fn append_optional_section(out: &mut String, title: &str, value: &Option<String>) {
    let Some(value) = value
        .as_ref()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    else {
        return;
    };
    out.push_str(&format!("## {title}\n\n"));
    out.push_str(value);
    out.push_str("\n\n");
}

fn extract_code4rena_links(html: &str, base_url: &str) -> Vec<Code4renaLink> {
    let mut links = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    let origin = Regex::new(r#"(?i)^(https?://[^/]+)"#)
        .unwrap()
        .captures(base_url)
        .and_then(|captures| captures.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_else(|| "https://code4rena.com".to_string());
    let re = Regex::new(r#"(?is)<a\b[^>]*href=["']([^"']+)["'][^>]*>(.*?)</a>"#).unwrap();
    for captures in re.captures_iter(html) {
        let Some(raw_url) = captures.get(1).map(|m| m.as_str()) else {
            continue;
        };
        let url = resolve_url(raw_url, &origin);
        if url.starts_with("mailto:") || url.starts_with("javascript:") {
            continue;
        }
        let label = captures
            .get(2)
            .map(|m| strip_tags(m.as_str()))
            .unwrap_or_else(|| url.clone());
        let label = clean_text(&decode_basic_entities(&label));
        let key = format!(
            "{}|{}",
            label.to_ascii_lowercase(),
            url.to_ascii_lowercase()
        );
        if seen.insert(key) {
            links.push(Code4renaLink { label, url });
        }
    }
    links
}

fn resolve_url(raw_url: &str, origin: &str) -> String {
    let trimmed = decode_basic_entities(raw_url.trim());
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed
    } else if trimmed.starts_with("//") {
        format!("https:{trimmed}")
    } else if trimmed.starts_with('/') {
        format!("{origin}{trimmed}")
    } else {
        trimmed
    }
    .split('#')
    .next()
    .unwrap_or("")
    .trim_end_matches('/')
    .to_string()
}

fn html_to_readable_text(html: &str) -> String {
    let mut text = html
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n");
    for tag in [
        "</p>",
        "</li>",
        "</tr>",
        "</div>",
        "</section>",
        "</h1>",
        "</h2>",
        "</h3>",
        "</h4>",
        "</td>",
        "</th>",
    ] {
        text = text.replace(tag, "\n");
    }
    text = strip_tags(&text);
    clean_multiline_text(&decode_basic_entities(&text))
}

fn strip_tags(text: &str) -> String {
    Regex::new(r#"(?is)<script.*?</script>|<style.*?</style>|<[^>]+>"#)
        .unwrap()
        .replace_all(text, " ")
        .to_string()
}

fn decode_basic_entities(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .replace("&#x27;", "'")
        .replace("&#x2F;", "/")
}

fn clean_multiline_text(text: &str) -> String {
    let mut out = Vec::new();
    let mut blank = false;
    for line in text.lines() {
        let cleaned = clean_text(line);
        if cleaned.is_empty() {
            if !blank {
                out.push(String::new());
            }
            blank = true;
        } else {
            out.push(cleaned);
            blank = false;
        }
    }
    out.join("\n").trim().to_string()
}

fn clean_text(text: &str) -> String {
    Regex::new(r#"\s+"#)
        .unwrap()
        .replace_all(text, " ")
        .trim()
        .to_string()
}

fn extract_project_title(text: &str) -> Option<String> {
    let ignored = [
        "skip navigation",
        "ready to secure your project?",
        "find bugs. get paid.",
        "login",
        "code4rena logo",
    ];
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .find(|line| {
            let lower = line.to_ascii_lowercase();
            !ignored.iter().any(|ignored| lower.contains(ignored))
                && !lower.starts_with("for projects")
                && !lower.starts_with("for wardens")
        })
        .map(|line| {
            let before_pipe = line.split('|').next().unwrap_or(line);
            before_pipe
                .trim_start_matches('#')
                .trim()
                .trim_end_matches(" Bug Bounty")
                .to_string()
        })
}

fn extract_project_summary(text: &str, project: &str) -> Option<String> {
    let mut lines = text.lines().map(str::trim).filter(|line| !line.is_empty());
    while let Some(line) = lines.next() {
        if line.eq_ignore_ascii_case(project) || line.trim_start_matches('#').trim() == project {
            return lines
                .find(|line| {
                    !line.starts_with("View Repo")
                        && !line.starts_with("Max bounty")
                        && !line.to_ascii_lowercase().contains("image:")
                })
                .map(str::to_string);
        }
    }
    None
}

fn extract_max_bounty(text: &str) -> Option<String> {
    Regex::new(r#"(?i)Max bounty\s*([^\n]+)"#)
        .unwrap()
        .captures(text)
        .and_then(|captures| captures.get(1).map(|m| clean_text(m.as_str())))
}

fn extract_rewards(text: &str) -> Vec<Code4renaReward> {
    let mut rewards = Vec::new();
    let re = Regex::new(r#"(?im)^\s*(Critical|High|Medium|Low)\s+([^\n]+)"#).unwrap();
    for captures in re.captures_iter(text) {
        let severity = captures.get(1).unwrap().as_str().to_string();
        let payout = clean_text(captures.get(2).unwrap().as_str());
        if payout.to_ascii_lowercase().contains("impact:") {
            continue;
        }
        rewards.push(Code4renaReward { severity, payout });
    }
    rewards.sort_by(|a, b| {
        a.severity
            .cmp(&b.severity)
            .then_with(|| a.payout.cmp(&b.payout))
    });
    rewards.dedup_by(|a, b| a.severity == b.severity && a.payout == b.payout);
    rewards
}

fn extract_scope_assets(scope_section: &str, links: &[Code4renaLink]) -> Vec<Code4renaScopeAsset> {
    // Scope is not consistently structured across Code4rena bounty pages. Some
    // pages use tables, some embed direct URLs, and some rely on prose. We join
    // nearby anchor data with direct URL extraction, then dedupe by normalized
    // URL so downstream scope generation has one record per asset.
    let section_lower = scope_section.to_ascii_lowercase();
    let mut assets = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    for link in links {
        let haystack = format!("{} {}", link.label, link.url).to_ascii_lowercase();
        let in_section = section_lower.contains(&link.label.to_ascii_lowercase())
            || section_lower.contains(&link.url.to_ascii_lowercase());
        let looks_relevant = haystack.contains("contract")
            || haystack.contains(".sol")
            || haystack.contains("deployment")
            || haystack.contains("deployments")
            || haystack.contains("github.com")
            || haystack.contains("etherscan")
            || haystack.contains("basescan")
            || haystack.contains("moonscan")
            || haystack.contains("arbiscan")
            || haystack.contains("optimistic.etherscan")
            || haystack.contains("kitescan");
        if !in_section || !looks_relevant {
            continue;
        }
        if haystack.contains("websites and apps") || haystack.contains("media kit") {
            continue;
        }
        let key = link.url.to_ascii_lowercase();
        if seen.insert(key) {
            assets.push(Code4renaScopeAsset {
                label: link.label.clone(),
                url: link.url.clone(),
                source_section: "Smart Contracts in Scope".to_string(),
            });
        }
    }

    assets
}

fn section_between(text: &str, start_markers: &[&str], end_markers: &[&str]) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let start = start_markers
        .iter()
        .filter_map(|marker| lower.find(&marker.to_ascii_lowercase()))
        .min()?;
    let after_start = &lower[start..];
    let end = end_markers
        .iter()
        .filter_map(|marker| after_start.find(&marker.to_ascii_lowercase()))
        .filter(|index| *index > 0)
        .min()
        .map(|index| start + index)
        .unwrap_or(text.len());
    Some(text[start..end].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code4rena_bounty_parser_extracts_scope_repo_and_custom_severity() {
        let html = r#"
        <h1>Intuition</h1>
        <p>The decentralized knowledge network.</p>
        <h1>Intuition Bug Bounty</h1>
        <p>Max bounty$100,000 in USDC</p>
        <h2>Scope & Severity Criteria</h2>
        <h2>Severity matrix:</h2>
        <p>Critical Systemic user fund loss.</p>
        <h2>Smart Contracts and Repos in Scope</h2>
        <a href="https://github.com/0xIntuition/intuition-contracts-v2">repo</a>
        <a href="https://www.docs.intuition.systems/docs/developer-tools/contracts/deployments">Smart Contract Deployments</a>
        <h2>Out-of-Scope</h2>
        <p>Known issues are out.</p>
        "#;
        let data = parse_code4rena_bounty_data(
            "https://code4rena.com/bounties/intuition",
            "https://code4rena.com/bounties/intuition",
            html,
        )
        .unwrap();
        assert_eq!(data.slug, "intuition");
        assert!(
            data.severity_section
                .as_ref()
                .unwrap()
                .contains("Systemic user fund loss")
        );
        assert!(
            data.scope_assets
                .iter()
                .any(|asset| asset.url.contains("intuition-contracts-v2"))
        );
        let codebases = data.resolved_git_codebases().unwrap();
        assert_eq!(
            codebases[0].repo_url,
            "https://github.com/0xIntuition/intuition-contracts-v2"
        );
    }

    #[test]
    fn code4rena_bounty_prefers_non_wrapper_scope_repo() {
        let html = r#"
        <h1>Legion</h1>
        <a href="https://github.com/code-423n4/legion-bug-bounty">View Repo</a>
        <h2>Smart Contracts in Scope</h2>
        <a href="https://github.com/Legion-Team/legion-protocol-contracts/blob/master/src/sales/LegionFixedPriceSale.sol">LegionFixedPriceSale</a>
        <h2>Out-of-Scope</h2>
        "#;
        let data = parse_code4rena_bounty_data(
            "https://code4rena.com/bounties/legion.cc",
            "https://code4rena.com/bounties/legion.cc",
            html,
        )
        .unwrap();
        let codebases = data.resolved_git_codebases().unwrap();
        assert_eq!(codebases.len(), 1);
        assert_eq!(
            codebases[0].repo_url,
            "https://github.com/Legion-Team/legion-protocol-contracts"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn live_code4rena_bounty_pages_are_parseable() {
        for url in [
            "https://code4rena.com/bounties/moonwell",
            "https://code4rena.com/bounties/intuition",
            "https://code4rena.com/bounties/legion.cc",
            "https://code4rena.com/bounties/kite-ai",
        ] {
            let data = fetch_code4rena_bounty(url).await.unwrap();
            assert!(!data.project.trim().is_empty(), "{url}");
            assert!(!data.links.is_empty(), "{url}");
            assert!(
                !data.scope_assets.is_empty() || !data.full_text.trim().is_empty(),
                "{url}"
            );
        }
    }
}
