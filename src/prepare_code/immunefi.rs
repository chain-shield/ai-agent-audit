//! Immunefi bounty ingestion and normalization.
//!
//! Immunefi exposes all bounty context through rendered web pages rather than a
//! stable public API. This module fetches the Information, Scope, and Resources
//! tabs, extracts the embedded Next.js data, and preserves enough structure for
//! the audit-preparation pipeline to generate docs, severity rubrics, scope
//! files, clone targets, and PoC runtime hints.
//!
//! NatSpec-style contract for this module:
//! - `@notice` Convert an Immunefi bug-bounty URL into structured
//!   smart-contract bounty metadata.
//! - `@dev` The application audits smart contracts only. Web/App assets and
//!   rewards are retained only when needed for traceability, and most public
//!   accessors filter them out before downstream use.
//! - `@custom:invariant` Smart-contract scope must be derived from explicit
//!   Smart Contract assets, impacts, rewards, and codebase links whenever those
//!   are available; generic Resources links must not over-scope the audit.
//! - `@custom:invariant` GitHub tree/blob refs are preserved as raw refs until
//!   `git_clone` can resolve them against real remote branches and tags. This is
//!   required for slash-named branches such as `release/v2`.

use anyhow::{Context, Result, anyhow};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const IMMUNEFI_HTTP_TIMEOUT_SECS: u64 = 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Full parsed Immunefi bounty model used by context and clone preparation.
///
/// The fields mirror the core bounty page concepts: program metadata,
/// codebases/documentation/audits, assets, impacts, rewards, and exclusion
/// rules. Methods on this struct provide smart-contract-only views for the
/// audit pipeline.
pub struct ImmunefiBountyData {
    /// User-provided URL before canonicalization.
    pub input_url: String,
    /// Canonical Information, Scope, and Resources tab URLs.
    pub urls: ImmunefiBountyUrls,
    /// Project name as displayed by Immunefi.
    pub project: String,
    /// Stable bounty slug from `/bug-bounty/<slug>/...`.
    pub slug: String,
    /// Program overview/body text.
    pub description: Option<String>,
    /// Official project website URL when present.
    pub website_url: Option<String>,
    /// Top-level GitHub URL from the page header, if present.
    pub github_url: Option<String>,
    /// Maximum bounty amount in USD-denominated integer units when parsed.
    pub max_bounty: Option<u64>,
    /// Program launch date text.
    pub launch_date: Option<String>,
    /// Last updated date text.
    pub updated_date: Option<String>,
    /// Immunefi PoC requirement label from the page.
    pub proof_of_concept_type: Option<String>,
    /// Primacy of Impact / Primacy of Rules summary text when detected.
    pub primacy: Option<String>,
    /// Immunefi-wide severity system version referenced by the page.
    pub severity_system: Option<ImmunefiSeveritySystem>,
    /// Reward token symbol, if the page exposes it.
    pub rewards_token: Option<String>,
    /// Network on which rewards are paid, if exposed.
    pub rewards_token_network: Option<String>,
    /// Candidate codebase links from Resources and smart-contract assets.
    pub codebases: Vec<ImmunefiResourceLink>,
    /// Documentation links relevant to the program.
    pub documentations: Vec<ImmunefiResourceLink>,
    /// Public audit links disclosed by the program.
    pub audits: Vec<ImmunefiAuditLink>,
    /// Assets in scope across Immunefi categories.
    pub assets: Vec<ImmunefiAsset>,
    /// Impacts in scope across Immunefi categories.
    pub impacts: Vec<ImmunefiImpact>,
    /// Reward rows across Immunefi categories.
    pub rewards: Vec<ImmunefiReward>,
    /// Immunefi default smart-contract out-of-scope text.
    pub default_out_of_scope_smart_contract: Option<String>,
    /// Immunefi default general out-of-scope text.
    pub default_out_of_scope_general: Option<String>,
    /// Prohibited activity text from the program.
    pub prohibited_activities: Option<String>,
    /// Program-specific out-of-scope text.
    pub custom_out_of_scope: Option<String>,
    /// Known issues or accepted risks disclosed by the program.
    pub known_issues: Vec<String>,
    /// Raw tab captures retained for tests/debugging; not serialized to artifacts.
    #[serde(skip)]
    pub tabs: Vec<ImmunefiTabCapture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonical Immunefi tab URLs for a bounty.
pub struct ImmunefiBountyUrls {
    /// Base `/bug-bounty/<slug>` URL.
    pub base: String,
    /// Information tab URL.
    pub information: String,
    /// Scope tab URL.
    pub scope: String,
    /// Resources tab URL.
    pub resources: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Link extracted from Immunefi Resources or codebase/documentation lists.
pub struct ImmunefiResourceLink {
    /// Absolute URL.
    pub url: String,
    /// Display title from the Immunefi payload.
    pub title: Option<String>,
    /// Description or row label associated with the link.
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Public audit link disclosed by an Immunefi program.
pub struct ImmunefiAuditLink {
    /// Audit report URL or repository path.
    pub url: String,
    /// Auditor name when present.
    pub auditor: Option<String>,
    /// Completion/date text when present.
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// One Immunefi asset row.
pub struct ImmunefiAsset {
    /// Asset URL, which may be an explorer address, deployment page, repo, or app URL.
    pub url: String,
    /// Immunefi asset category, e.g. `smart_contract`.
    pub asset_type: Option<String>,
    /// Asset description/name from the Scope table.
    pub description: Option<String>,
    /// Whether this is Immunefi's Primacy of Impact placeholder.
    pub is_primacy_of_impact: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// One Immunefi impact row.
pub struct ImmunefiImpact {
    /// Critical/High/Medium/Low label from the program.
    pub severity: String,
    /// Immunefi asset category to which this impact applies.
    pub asset_type: Option<String>,
    /// Impact description used as the program-specific severity criterion.
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// One Immunefi reward row.
pub struct ImmunefiReward {
    /// Critical/High/Medium/Low label.
    pub severity: String,
    /// Immunefi asset category to which the reward applies.
    pub asset_type: Option<String>,
    /// Reward model label, such as fixed or range.
    pub reward_model: Option<String>,
    /// Fixed reward amount when applicable.
    pub fixed_reward: Option<u64>,
    /// Minimum reward for range models.
    pub min_reward: Option<u64>,
    /// Maximum reward for range models.
    pub max_reward: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Immunefi-wide severity system reference.
pub struct ImmunefiSeveritySystem {
    /// Version label, e.g. `v2.3`.
    pub version: String,
    /// Documentation URL for the severity system.
    pub url: String,
}

#[derive(Debug, Clone)]
/// Raw fetched tab payload used by the parser.
pub struct ImmunefiTabCapture {
    /// Which Immunefi tab was fetched.
    pub kind: ImmunefiTabKind,
    /// Fetched URL.
    pub url: String,
    /// Raw HTML payload.
    pub html: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Immunefi tab identifiers.
pub enum ImmunefiTabKind {
    /// `/information/`
    Information,
    /// `/scope/`
    Scope,
    /// `/resources/`
    Resources,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// GitHub codebase candidate after URL parsing but before clone/build.
///
/// `raw_tree_refs` stores unresolved branch-plus-path strings, because GitHub
/// URLs cannot be split safely without consulting remote refs. For example,
/// `blob/release/v2/src/Foo.sol` might mean branch `release/v2` and path
/// `src/Foo.sol`, not branch `release` and path `v2/src/Foo.sol`.
pub struct ResolvedGitCodebase {
    /// Normalized clone URL without `/tree` or `/blob` suffixes.
    pub repo_url: String,
    /// Resolved branch/tag/ref when known without remote lookup.
    pub branch: Option<String>,
    /// Resolved tree paths inside the repo, if known.
    pub tree_paths: Vec<String>,
    /// Unresolved branch/tag/path segments requiring remote ref resolution.
    pub raw_tree_refs: Vec<String>,
}

/// Fetches all Immunefi tabs and parses them into bounty metadata.
///
/// `@notice` This is the primary entry point for Immunefi bounty preparation.
/// `@dev` Fetching all tabs is required because scope, codebase, and severity
/// information are distributed across Information, Scope, and Resources.
pub async fn fetch_immunefi_bounty(input_url: &str) -> Result<ImmunefiBountyData> {
    let urls = normalize_immunefi_bounty_urls(input_url)?;
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(IMMUNEFI_HTTP_TIMEOUT_SECS))
        .user_agent("ai-agent-audit-immunefi-context/0.1")
        .build()?;

    let tabs = vec![
        fetch_tab(&client, ImmunefiTabKind::Information, &urls.information).await?,
        fetch_tab(&client, ImmunefiTabKind::Scope, &urls.scope).await?,
        fetch_tab(&client, ImmunefiTabKind::Resources, &urls.resources).await?,
    ];

    Ok(parse_immunefi_bounty_data(input_url, urls, tabs))
}

async fn fetch_tab(
    client: &reqwest::Client,
    kind: ImmunefiTabKind,
    url: &str,
) -> Result<ImmunefiTabCapture> {
    // The fetcher intentionally does no parsing. Tests can provide captured tab
    // HTML to `parse_immunefi_bounty_data`, while live runs share the same
    // deterministic parser.
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        anyhow::bail!(
            "Immunefi bounty tab fetch failed with status {} for {}",
            response.status(),
            url
        );
    }
    Ok(ImmunefiTabCapture {
        kind,
        url: url.to_string(),
        html: response.text().await?,
    })
}

/// Normalizes any Immunefi tab URL to the full canonical tab set.
///
/// `@dev` A user may provide `/information/`, `/scope/`, `/resources/`, or a
/// base bounty URL. The preparation pipeline always consumes all three tabs.
pub fn normalize_immunefi_bounty_urls(input_url: &str) -> Result<ImmunefiBountyUrls> {
    let input = input_url.trim().trim_end_matches('/');
    let captures = Regex::new(r#"(?i)^(https?://[^/]+)/bug-bounty/([^/#?]+)(?:/[^#?]*)?"#)
        .unwrap()
        .captures(input)
        .ok_or_else(|| {
            anyhow!(
                "Invalid Immunefi bounty URL `{input_url}`; expected https://immunefi.com/bug-bounty/<slug>/..."
            )
        })?;
    let origin = captures
        .get(1)
        .map(|m| m.as_str())
        .unwrap_or("https://immunefi.com");
    let slug = captures
        .get(2)
        .map(|m| m.as_str())
        .context("missing Immunefi bounty slug")?;
    let base = format!("{origin}/bug-bounty/{slug}");
    Ok(ImmunefiBountyUrls {
        information: format!("{base}/information/"),
        scope: format!("{base}/scope/"),
        resources: format!("{base}/resources/"),
        base,
    })
}

/// Parses fetched Immunefi tab HTML into structured bounty data.
///
/// `@notice` This is the deterministic parser used by live fetches and tests.
/// `@dev` Immunefi's Next.js payload is escaped and may duplicate fields across
/// tabs, so the parser first merges all tab HTML into one normalized corpus.
pub fn parse_immunefi_bounty_data(
    input_url: &str,
    urls: ImmunefiBountyUrls,
    tabs: Vec<ImmunefiTabCapture>,
) -> ImmunefiBountyData {
    // Merge tabs before extraction. Immunefi can move fields between tabs while
    // preserving the same underlying payload names, and the audit prep should
    // remain stable across those frontend changes.
    let raw = tabs
        .iter()
        .map(|tab| tab.html.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let normalized = normalize_next_data_text(&raw);
    let slug = urls
        .base
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("immunefi-bounty")
        .to_string();

    let project = extract_string_field(&normalized, "project")
        .or_else(|| extract_title_project(&normalized))
        .unwrap_or_else(|| slug.clone());

    let program_codebase_segment = extract_array_segment(&normalized, "programCodebases");
    let mut codebases = program_codebase_segment
        .as_deref()
        .map(parse_resource_links)
        .unwrap_or_default();
    if codebases.is_empty()
        && let Some(github_url) = extract_string_field(&normalized, "githubUrl")
    {
        codebases.push(ImmunefiResourceLink {
            url: github_url,
            title: Some(format!("{project} Codebase")),
            description: Some("Github Codebase".to_string()),
        });
    }

    let documentations = extract_array_segment(&normalized, "programDocumentations")
        .as_deref()
        .map(parse_resource_links)
        .unwrap_or_default();
    let audits = extract_array_segment(&normalized, "audits")
        .as_deref()
        .map(parse_audit_links)
        .unwrap_or_default();
    let assets = extract_array_segment(&normalized, "assets")
        .as_deref()
        .map(parse_assets)
        .unwrap_or_default();
    let impacts = extract_array_segment(&normalized, "programImpacts")
        .or_else(|| extract_array_segment(&normalized, "impacts"))
        .as_deref()
        .map(parse_impacts)
        .unwrap_or_default();
    let rewards = extract_array_segment(&normalized, "programRewards")
        .or_else(|| extract_array_segment(&normalized, "rewards"))
        .as_deref()
        .map(parse_rewards)
        .unwrap_or_default();

    ImmunefiBountyData {
        input_url: input_url.to_string(),
        urls,
        project,
        slug,
        description: extract_string_field(&normalized, "description"),
        website_url: extract_string_field(&normalized, "websiteUrl"),
        github_url: extract_string_field(&normalized, "githubUrl"),
        max_bounty: extract_u64_field(&normalized, "maxBounty"),
        launch_date: extract_string_field(&normalized, "launchDate"),
        updated_date: extract_string_field(&normalized, "updatedDate"),
        proof_of_concept_type: extract_string_field(&normalized, "proofOfConceptType"),
        primacy: extract_string_field(&normalized, "primacy"),
        severity_system: detect_severity_system(&normalized),
        rewards_token: extract_string_field(&normalized, "rewardsToken"),
        rewards_token_network: extract_string_field(&normalized, "rewardsTokenNetwork"),
        codebases,
        documentations,
        audits,
        assets,
        impacts,
        rewards,
        default_out_of_scope_smart_contract: extract_string_field(
            &normalized,
            "defaultOutOfScopeSmartContract",
        ),
        default_out_of_scope_general: extract_string_field(&normalized, "defaultOutOfScopeGeneral"),
        prohibited_activities: extract_string_field(&normalized, "prohibitedActivites")
            .or_else(|| extract_string_field(&normalized, "defaultProhibitedActivities")),
        custom_out_of_scope: extract_string_field(&normalized, "customOutOfScopeInformation"),
        known_issues: extract_array_segment(&normalized, "knownIssues")
            .as_deref()
            .map(parse_known_issues)
            .unwrap_or_default(),
        tabs,
    }
}

impl ImmunefiBountyData {
    /// Returns the Smart Contract asset rows for this bounty.
    ///
    /// `@dev` Primacy-of-Impact placeholder rows can be included for rule and
    /// rubric generation, but concrete scope-file generation usually excludes
    /// them because they are not deployable contracts.
    pub fn smart_contract_assets(&self, include_primacy: bool) -> Vec<&ImmunefiAsset> {
        self.assets
            .iter()
            .filter(|asset| include_primacy || !asset.is_primacy_of_impact)
            .filter(|asset| is_smart_contract_asset_type(asset.asset_type.as_deref()))
            .collect()
    }

    /// Returns Smart Contract impact rows as program-specific severity criteria.
    pub fn smart_contract_impacts(&self) -> Vec<ImmunefiImpact> {
        self.impacts
            .iter()
            .filter(|impact| is_smart_contract_asset_type(impact.asset_type.as_deref()))
            .cloned()
            .collect()
    }

    /// Returns Smart Contract reward rows for dynamic rubric generation.
    pub fn smart_contract_rewards(&self) -> Vec<ImmunefiReward> {
        self.rewards
            .iter()
            .filter(|reward| is_smart_contract_asset_type(reward.asset_type.as_deref()))
            .cloned()
            .collect()
    }

    /// Resolves cloneable GitHub codebases for the Smart Contract audit.
    ///
    /// `@notice` This is the strict repo-derivation API used by `git_clone`.
    /// `@dev` It intentionally refuses generic GitHub org listings and web-app
    /// repos when they cannot be tied to Smart Contract assets.
    pub fn resolved_git_codebases(&self) -> Result<Vec<ResolvedGitCodebase>> {
        let candidates = self.git_codebase_candidates();
        if candidates.is_empty() {
            let org_links = self
                .codebases
                .iter()
                .filter(|link| is_github_org_or_user_listing_url(&link.url))
                .map(|link| link.url.clone())
                .collect::<Vec<_>>();
            if org_links.is_empty() {
                anyhow::bail!(
                    "Immunefi bounty `{}` did not expose a cloneable GitHub Codebase URL in Smart Contract assets or Resources. Specify `repo` manually.",
                    self.project
                );
            }
            anyhow::bail!(
                "Immunefi bounty `{}` only exposed GitHub org/user listing URLs ({}) and no unique smart-contract source repo. Specify `repo` manually.",
                self.project,
                org_links.join(", ")
            );
        }

        merge_git_codebase_candidates(&self.project, candidates)
    }

    /// Returns the only resolved codebase, failing when a bounty is polyrepo.
    ///
    /// `@dev` Newer setup paths use `resolved_git_codebases`; this helper is
    /// kept for single-repo callers that need an explicit guard.
    pub fn resolved_git_codebase(&self) -> Result<ResolvedGitCodebase> {
        let codebases = self.resolved_git_codebases()?;
        if codebases.len() > 1 {
            anyhow::bail!(
                "Immunefi bounty `{}` references multiple independent GitHub repos: {}. Use the polyrepo-aware setup path.",
                self.project,
                codebases
                    .iter()
                    .map(|codebase| codebase.repo_url.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        codebases
            .into_iter()
            .next()
            .context("internal error: empty Immunefi codebase resolution")
    }

    /// Collects GitHub repo candidates from Smart Contract assets and resources.
    ///
    /// `@dev` On mixed-category bounties, Resources links are accepted only
    /// when their title/description looks Smart Contract specific. This prevents
    /// ENS-style Web & App repos from being cloned for a smart-contract audit.
    pub fn git_codebase_candidates(&self) -> Vec<ResolvedGitCodebase> {
        let mut smart_asset_candidates = Vec::new();

        for asset in self.smart_contract_assets(false) {
            smart_asset_candidates.extend(github_codebase_urls_from_text(&asset.url));
            if let Some(description) = &asset.description {
                smart_asset_candidates.extend(github_codebase_urls_from_text(description));
            }
        }

        let mixed_asset_categories = self.has_non_smart_contract_assets();
        let mut resource_candidates = Vec::new();
        for link in &self.codebases {
            let link_candidates = github_candidates_from_resource_link(link);
            if mixed_asset_categories
                && !resource_link_is_smart_contract_specific(link)
                && !resource_link_is_generic_program_codebase(link, &link_candidates)
            {
                continue;
            }
            resource_candidates.extend(link_candidates);
        }
        for link in &self.documentations {
            if !documentation_link_is_smart_contract_codebase(link) {
                continue;
            }
            resource_candidates.extend(github_codebase_urls_from_text(&link.url));
            if let Some(description) = &link.description {
                resource_candidates.extend(github_codebase_urls_from_text(description));
            }
            if let Some(title) = &link.title {
                resource_candidates.extend(github_codebase_urls_from_text(title));
            }
        }

        let mut candidates = smart_asset_candidates;
        candidates.extend(resource_candidates);
        candidates
    }

    fn has_non_smart_contract_assets(&self) -> bool {
        self.assets.iter().any(|asset| {
            !asset.is_primacy_of_impact
                && asset.asset_type.is_some()
                && !is_smart_contract_asset_type(asset.asset_type.as_deref())
        })
    }

    /// Renders tab-specific context markdown for the audit context generator.
    ///
    /// `@notice` This output feeds LLM workers and should be concise, scoped,
    /// and explicit about excluded Web & App categories.
    pub fn context_markdown_for_tab(&self, kind: ImmunefiTabKind, visible_text: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!("# Immunefi Bounty - {}\n\n", self.project));
        out.push_str(&format!("Source: {}\n\n", self.url_for_tab(kind)));

        match kind {
            ImmunefiTabKind::Information => self.append_information_markdown(&mut out),
            ImmunefiTabKind::Scope => self.append_scope_markdown(&mut out),
            ImmunefiTabKind::Resources => self.append_resources_markdown(&mut out),
        }

        if visible_text.contains("Web & App") || visible_text.contains("websites_and_applications")
        {
            out.push_str("\n## Excluded Categories\n\n");
            out.push_str("- Web & App / websites_and_applications rows are intentionally excluded for `AuditType::ImmunefiBugBounty`; this app audits Smart Contract scope only.\n");
        }
        out
    }

    fn url_for_tab(&self, kind: ImmunefiTabKind) -> &str {
        match kind {
            ImmunefiTabKind::Information => &self.urls.information,
            ImmunefiTabKind::Scope => &self.urls.scope,
            ImmunefiTabKind::Resources => &self.urls.resources,
        }
    }

    fn append_information_markdown(&self, out: &mut String) {
        if let Some(description) = &self.description {
            out.push_str("## Program Overview\n\n");
            out.push_str(description);
            out.push_str("\n\n");
        }
        if let Some(max_bounty) = self.max_bounty {
            out.push_str(&format!("- Maximum bounty: ${max_bounty}\n"));
        }
        if let Some(poc) = &self.proof_of_concept_type {
            out.push_str(&format!("- Proof of Concept: {poc}\n"));
        }
        if let Some(primacy) = &self.primacy {
            out.push_str(&format!("- Primacy: {primacy}\n"));
        }
        if let Some(token) = &self.rewards_token {
            out.push_str(&format!("- Rewards token: {token}\n"));
        }
        if let Some(network) = &self.rewards_token_network {
            out.push_str(&format!("- Rewards token network: {network}\n"));
        }
        out.push('\n');
        append_rewards(out, &self.smart_contract_rewards());
        append_audits(out, &self.audits);
        if let Some(prohibited) = &self.prohibited_activities {
            out.push_str("## Prohibited Activities\n\n");
            out.push_str(prohibited);
            out.push_str("\n\n");
        }
    }

    fn append_scope_markdown(&self, out: &mut String) {
        out.push_str("## Assets in Scope\n\n");
        let smart_assets = self.smart_contract_assets(true);
        out.push_str(
            "> Smart Contract category only. Web & App assets are intentionally excluded.\n\n",
        );
        for asset in smart_assets {
            out.push_str(&format!(
                "- {}{}: {}\n",
                asset
                    .asset_type
                    .as_deref()
                    .unwrap_or("asset")
                    .replace('_', " "),
                if asset.is_primacy_of_impact {
                    " (Primacy of Impact placeholder)"
                } else {
                    ""
                },
                asset.url
            ));
            if let Some(description) = &asset.description {
                out.push_str(&format!("  Description: {description}\n"));
            }
        }
        out.push('\n');
        append_impacts(out, &self.smart_contract_impacts());
        if let Some(oos) = &self.default_out_of_scope_smart_contract {
            out.push_str("## Smart Contract Out of Scope\n\n");
            out.push_str(oos);
            out.push_str("\n\n");
        }
        if let Some(oos) = &self.default_out_of_scope_general {
            out.push_str("## General Out of Scope\n\n");
            out.push_str(oos);
            out.push_str("\n\n");
        }
        if let Some(oos) = &self.custom_out_of_scope {
            out.push_str("## Custom Out of Scope\n\n");
            out.push_str(oos);
            out.push_str("\n\n");
        }
    }

    fn append_resources_markdown(&self, out: &mut String) {
        out.push_str("## GitHub Codebases\n\n");
        for link in &self.codebases {
            out.push_str(&format!(
                "- {}: {}\n",
                link.title.as_deref().unwrap_or("Codebase"),
                link.url
            ));
            if let Some(description) = &link.description {
                out.push_str(&format!("  Description: {description}\n"));
            }
        }
        out.push_str("\n## Documentation\n\n");
        for link in &self.documentations {
            out.push_str(&format!(
                "- {}: {}\n",
                link.title.as_deref().unwrap_or("Documentation"),
                link.url
            ));
            if let Some(description) = &link.description {
                out.push_str(&format!("  Description: {description}\n"));
            }
        }
        append_audits(out, &self.audits);
        if !self.known_issues.is_empty() {
            out.push_str("## Known Issues\n\n");
            for issue in &self.known_issues {
                out.push_str(&format!("- {issue}\n"));
            }
            out.push('\n');
        }
    }
}

fn append_impacts(out: &mut String, impacts: &[ImmunefiImpact]) {
    if impacts.is_empty() {
        return;
    }
    out.push_str("## Impacts in Scope\n\n");
    for impact in impacts {
        out.push_str(&format!(
            "- {}{}: {}\n",
            impact.severity,
            impact
                .asset_type
                .as_deref()
                .map(|kind| format!(" ({})", kind.replace('_', " ")))
                .unwrap_or_default(),
            impact.description
        ));
    }
    out.push('\n');
}

fn append_rewards(out: &mut String, rewards: &[ImmunefiReward]) {
    if rewards.is_empty() {
        return;
    }
    out.push_str("## Rewards\n\n");
    for reward in rewards {
        let mut details = Vec::new();
        if let Some(fixed) = reward.fixed_reward {
            details.push(format!("fixed ${fixed}"));
        }
        if let Some(min) = reward.min_reward {
            details.push(format!("min ${min}"));
        }
        if let Some(max) = reward.max_reward {
            details.push(format!("max ${max}"));
        }
        out.push_str(&format!(
            "- {}{}{}{}\n",
            reward.severity,
            reward
                .asset_type
                .as_deref()
                .map(|kind| format!(" ({})", kind.replace('_', " ")))
                .unwrap_or_default(),
            reward
                .reward_model
                .as_deref()
                .map(|model| format!(" [{model}]"))
                .unwrap_or_default(),
            if details.is_empty() {
                String::new()
            } else {
                format!(": {}", details.join(", "))
            }
        ));
    }
    out.push('\n');
}

fn append_audits(out: &mut String, audits: &[ImmunefiAuditLink]) {
    if audits.is_empty() {
        return;
    }
    out.push_str("## Audits / Known Issues\n\n");
    for audit in audits {
        out.push_str(&format!(
            "- {}{}: {}\n",
            audit.auditor.as_deref().unwrap_or("Audit"),
            audit
                .date
                .as_deref()
                .map(|date| format!(" ({date})"))
                .unwrap_or_default(),
            audit.url
        ));
    }
    out.push('\n');
}

fn is_smart_contract_asset_type(asset_type: Option<&str>) -> bool {
    // Older Immunefi payloads sometimes omit `assetType`; treat missing values
    // as smart-contract-compatible so legacy pages are not accidentally emptied.
    asset_type
        .map(|asset_type| {
            asset_type
                .replace([' ', '-'], "_")
                .eq_ignore_ascii_case("smart_contract")
        })
        .unwrap_or(true)
}

fn resource_link_is_smart_contract_specific(link: &ImmunefiResourceLink) -> bool {
    // In mixed programs, Resources may include web repos beside contract repos.
    // Require a contract/build-system hint before accepting those generic
    // resource links for smart-contract cloning.
    let haystack = [
        link.url.as_str(),
        link.title.as_deref().unwrap_or_default(),
        link.description.as_deref().unwrap_or_default(),
    ]
    .join(" ")
    .to_ascii_lowercase();
    [
        "smart contract",
        "smart-contract",
        "smart_contract",
        "solidity",
        "contract",
        "contracts",
        "foundry",
        "hardhat",
        "forge",
    ]
    .iter()
    .any(|marker| haystack.contains(marker))
}

fn github_candidates_from_resource_link(link: &ImmunefiResourceLink) -> Vec<ResolvedGitCodebase> {
    let mut candidates = github_codebase_urls_from_text(&link.url);
    if let Some(description) = &link.description {
        candidates.extend(github_codebase_urls_from_text(description));
    }
    if let Some(title) = &link.title {
        candidates.extend(github_codebase_urls_from_text(title));
    }
    candidates
}

fn resource_link_is_generic_program_codebase(
    link: &ImmunefiResourceLink,
    candidates: &[ResolvedGitCodebase],
) -> bool {
    // Some Immunefi mixed-category programs label the only source repository as
    // a generic "Program Codebase" even when the repo contains the in-scope
    // contracts. Accept cloneable repo links with that exact program-codebase
    // shape, while keeping explicit web/app resource repos filtered out.
    if candidates.is_empty() {
        return false;
    }
    let title = link
        .title
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let description = link
        .description
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();

    title.contains("codebase") && description.contains("program codebase")
}

fn documentation_link_is_smart_contract_codebase(link: &ImmunefiResourceLink) -> bool {
    // Some programs put source-code deployment pages in documentation tables.
    // Accept those only when they look like codebase references, and avoid audit
    // report links because they are context, not clone targets.
    let haystack = [
        link.url.as_str(),
        link.title.as_deref().unwrap_or_default(),
        link.description.as_deref().unwrap_or_default(),
    ]
    .join(" ")
    .to_ascii_lowercase();
    let audit_only = haystack.contains("audit");
    let codebase_hint = [
        "smart contract",
        "smart-contract",
        "smart_contract",
        "contracts",
        "solidity",
        "source",
        "codebase",
        "repository",
        "readme",
    ]
    .iter()
    .any(|marker| haystack.contains(marker));

    codebase_hint && !audit_only
}

pub fn github_codebase_urls_from_text(text: &str) -> Vec<ResolvedGitCodebase> {
    // Extract URLs from free-form text first, then allow the whole input to be a
    // URL. This handles both markdown prose and direct YAML/config values.
    let mut candidates = Vec::new();
    let github_re = Regex::new(r#"https?://github\.com/[^\s)\]"'<>,]+"#).unwrap();
    for m in github_re.find_iter(text) {
        let url = m.as_str();
        if let Some(candidate) = parse_github_codebase_url(url) {
            candidates.push(candidate);
        }
    }
    if candidates.is_empty()
        && let Some(candidate) = parse_github_codebase_url(text)
    {
        candidates.push(candidate);
    }
    candidates
}

pub fn merge_git_codebase_candidates(
    project: &str,
    candidates: Vec<ResolvedGitCodebase>,
) -> Result<Vec<ResolvedGitCodebase>> {
    // Multiple page links can point to different folders in the same repo. Fold
    // those into one repo candidate while preserving all tree/raw refs. Distinct
    // repos remain distinct so the polyrepo path can clone them independently.
    let mut by_repo = std::collections::BTreeMap::<String, Vec<ResolvedGitCodebase>>::new();
    for candidate in candidates {
        by_repo
            .entry(normalize_repo_url_for_compare(&candidate.repo_url))
            .or_default()
            .push(candidate);
    }

    let mut out = Vec::new();
    for (_, repo_candidates) in by_repo {
        let repo_url = repo_candidates
            .first()
            .map(|candidate| candidate.repo_url.clone())
            .context("internal error: empty repo candidate group")?;
        let branches = repo_candidates
            .iter()
            .filter_map(|candidate| candidate.branch.clone())
            .collect::<BTreeSet<_>>();
        if branches.len() > 1 {
            anyhow::bail!(
                "Immunefi bounty `{}` references repo `{}` with multiple tree branches/refs: {}. Configure this bounty manually for now.",
                project,
                repo_url,
                branches.into_iter().collect::<Vec<_>>().join(", ")
            );
        }
        let branch = branches.into_iter().next();
        let mut tree_paths = repo_candidates
            .iter()
            .flat_map(|candidate| candidate.tree_paths.clone())
            .collect::<Vec<_>>();
        tree_paths.sort();
        tree_paths.dedup();
        let mut raw_tree_refs = repo_candidates
            .into_iter()
            .flat_map(|candidate| candidate.raw_tree_refs)
            .collect::<Vec<_>>();
        raw_tree_refs.sort();
        raw_tree_refs.dedup();
        out.push(ResolvedGitCodebase {
            repo_url,
            branch,
            tree_paths,
            raw_tree_refs,
        });
    }

    out.sort_by(|left, right| left.repo_url.cmp(&right.repo_url));
    Ok(out)
}

pub fn parse_github_codebase_url(url: &str) -> Option<ResolvedGitCodebase> {
    // Do not guess slash-named branches here. Store raw tree/blob refs and let
    // `git_clone::resolve_github_tree_refs` split them after it sees the real
    // branch/tag list from `git ls-remote`.
    let clean = url
        .trim()
        .trim_matches(['"', '\'', '`', '[', ']', '(', ')'])
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .split('?')
        .next()
        .unwrap_or(url)
        .split('#')
        .next()
        .unwrap_or(url);
    let after_host = clean.split("github.com/").nth(1)?;
    let parts = after_host.split('/').collect::<Vec<_>>();
    if parts.len() < 2 {
        return None;
    }
    if matches!(
        parts[0],
        "orgs" | "users" | "marketplace" | "topics" | "collections"
    ) {
        return None;
    }
    if matches!(
        parts.get(2).copied(),
        Some("settings" | "actions" | "issues" | "pulls")
    ) {
        return None;
    }
    let repo_url = format!("https://github.com/{}/{}", parts[0], parts[1]);
    let mut branch = None;
    let mut tree_paths = Vec::new();
    let mut raw_tree_refs = Vec::new();
    if parts.len() >= 3 {
        match parts[2] {
            "tree" if parts.len() >= 4 => {
                branch = Some(parts[3].to_string());
                raw_tree_refs.push(parts[3..].join("/"));
                if parts.len() > 4 {
                    tree_paths.push(parts[4..].join("/"));
                }
            }
            "wiki" | "releases" => {}
            "blob" | "raw" if parts.len() >= 4 => {
                raw_tree_refs.push(parts[3..].join("/"));
            }
            "blob" | "raw" => {}
            _ => {}
        }
    }
    Some(ResolvedGitCodebase {
        repo_url,
        branch,
        tree_paths,
        raw_tree_refs,
    })
}

pub fn is_github_org_or_user_listing_url(url: &str) -> bool {
    // Org/user repository listings are not clone targets. They tell us a human
    // may need to choose a repo unless scoped assets point at a concrete repo.
    let clean = url
        .trim()
        .trim_end_matches('/')
        .split('?')
        .next()
        .unwrap_or(url)
        .split('#')
        .next()
        .unwrap_or(url);
    let Some(after_host) = clean.split("github.com/").nth(1) else {
        return false;
    };
    let parts = after_host.split('/').collect::<Vec<_>>();
    matches!(
        parts.as_slice(),
        ["orgs", _, "repositories"] | ["orgs", _] | ["users", _, "repositories"] | ["users", _]
    ) || parts.len() == 1
}

pub fn normalize_repo_url_for_compare(url: &str) -> String {
    // Repository identity ignores trailing slashes, `.git`, and case. Tree/blob
    // suffixes are stripped by `parse_github_codebase_url` before comparison.
    url.trim()
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .to_ascii_lowercase()
}

pub fn same_github_repo(left: &str, right: &str) -> bool {
    // Compare by canonical clone URL when possible so a configured repo can
    // match bounty URLs that include `/tree/...` or `/blob/...` suffixes.
    match (
        parse_github_codebase_url(left),
        parse_github_codebase_url(right),
    ) {
        (Some(left), Some(right)) => {
            normalize_repo_url_for_compare(&left.repo_url)
                == normalize_repo_url_for_compare(&right.repo_url)
        }
        _ => normalize_repo_url_for_compare(left) == normalize_repo_url_for_compare(right),
    }
}

fn normalize_next_data_text(raw: &str) -> String {
    decode_basic_entities(raw)
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .replace("\\/", "/")
        .replace("\\u0026", "&")
        .replace("\\u003c", "<")
        .replace("\\u003e", ">")
        .replace("\\u0027", "'")
        .replace("\\u2019", "'")
        .replace("\\u2013", "-")
        .replace("\\u2014", "-")
}

fn decode_basic_entities(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

fn extract_title_project(normalized: &str) -> Option<String> {
    Regex::new(r#"(?is)<title[^>]*>(.*?)\s+Bug\s+Bounties\s+\|\s+Immunefi.*?</title>"#)
        .unwrap()
        .captures(normalized)
        .and_then(|captures| captures.get(1).map(|m| clean_json_text(m.as_str())))
}

fn detect_severity_system(normalized: &str) -> Option<ImmunefiSeveritySystem> {
    let lower = normalized.to_ascii_lowercase();
    let version = if lower.contains("v2.2")
        || lower.contains("vulnerability-severity-classification-system-v2-2")
    {
        "v2.2"
    } else if lower.contains("v2.3")
        || lower.contains("vulnerability-severity-classification-system-v2-3")
    {
        "v2.3"
    } else {
        return None;
    };

    let url_version = version.replace('.', "-");
    Some(ImmunefiSeveritySystem {
        version: version.to_string(),
        url: format!(
            "https://immunefi.com/immunefi-vulnerability-severity-classification-system-{url_version}/"
        ),
    })
}

fn extract_string_field(normalized: &str, field: &str) -> Option<String> {
    let pattern = format!(r#"(?s)"{}"\s*:\s*"([^"]*)""#, regex::escape(field));
    Regex::new(&pattern)
        .ok()?
        .captures(normalized)
        .and_then(|captures| captures.get(1).map(|m| clean_json_text(m.as_str())))
        .filter(|value| !value.is_empty() && !value.starts_with('$') && value != "_blank_")
}

fn extract_u64_field(normalized: &str, field: &str) -> Option<u64> {
    let pattern = format!(r#""{}"\s*:\s*([0-9]+)"#, regex::escape(field));
    Regex::new(&pattern)
        .ok()?
        .captures(normalized)
        .and_then(|captures| captures.get(1).and_then(|m| m.as_str().parse().ok()))
}

fn extract_bool_field(object: &str, field: &str) -> bool {
    let pattern = format!(r#""{}"\s*:\s*(true|false)"#, regex::escape(field));
    Regex::new(&pattern)
        .ok()
        .and_then(|regex| regex.captures(object))
        .and_then(|captures| captures.get(1).map(|m| m.as_str() == "true"))
        .unwrap_or(false)
}

fn extract_object_string_field(object: &str, field: &str) -> Option<String> {
    extract_string_field(object, field)
}

fn extract_object_u64_field(object: &str, field: &str) -> Option<u64> {
    extract_u64_field(object, field)
}

fn extract_array_segment(normalized: &str, field: &str) -> Option<String> {
    let marker = format!(r#""{field}":["#);
    let marker_start = normalized.find(&marker)?;
    let mut start = marker_start + marker.len() - 1;
    let bytes = normalized.as_bytes();
    while start < bytes.len() && bytes[start] != b'[' {
        start += 1;
    }
    if start >= bytes.len() {
        return None;
    }

    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, ch) in normalized[start..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        if ch == '[' {
            depth += 1;
        } else if ch == ']' {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                let end = start + offset + ch.len_utf8();
                return Some(normalized[start..end].to_string());
            }
        }
    }
    None
}

fn parse_resource_links(segment: &str) -> Vec<ImmunefiResourceLink> {
    parse_jsonish_objects(segment)
        .into_iter()
        .filter_map(|object| {
            let url = extract_object_string_field(&object, "url")?;
            Some(ImmunefiResourceLink {
                url,
                title: extract_object_string_field(&object, "title"),
                description: extract_object_string_field(&object, "description"),
            })
        })
        .collect()
}

fn parse_audit_links(segment: &str) -> Vec<ImmunefiAuditLink> {
    parse_jsonish_objects(segment)
        .into_iter()
        .filter_map(|object| {
            let url = extract_object_string_field(&object, "url")?;
            Some(ImmunefiAuditLink {
                url,
                auditor: extract_object_string_field(&object, "auditor"),
                date: extract_object_string_field(&object, "date"),
            })
        })
        .collect()
}

fn parse_assets(segment: &str) -> Vec<ImmunefiAsset> {
    parse_jsonish_objects(segment)
        .into_iter()
        .filter_map(|object| {
            let url = extract_object_string_field(&object, "url")?;
            Some(ImmunefiAsset {
                url,
                asset_type: extract_object_string_field(&object, "type"),
                description: extract_object_string_field(&object, "description"),
                is_primacy_of_impact: extract_bool_field(&object, "isPrimacyOfImpact"),
            })
        })
        .collect()
}

fn parse_impacts(segment: &str) -> Vec<ImmunefiImpact> {
    parse_jsonish_objects(segment)
        .into_iter()
        .filter_map(|object| {
            let severity = extract_object_string_field(&object, "severity")?;
            let description = extract_object_string_field(&object, "description")
                .or_else(|| extract_object_string_field(&object, "title"))?;
            Some(ImmunefiImpact {
                severity,
                asset_type: extract_object_string_field(&object, "assetType")
                    .or_else(|| extract_object_string_field(&object, "type")),
                description,
            })
        })
        .collect()
}

fn parse_rewards(segment: &str) -> Vec<ImmunefiReward> {
    parse_jsonish_objects(segment)
        .into_iter()
        .filter_map(|object| {
            let severity = extract_object_string_field(&object, "severity")?;
            Some(ImmunefiReward {
                severity,
                asset_type: extract_object_string_field(&object, "assetType"),
                reward_model: extract_object_string_field(&object, "rewardModel"),
                fixed_reward: extract_object_u64_field(&object, "fixedReward"),
                min_reward: extract_object_u64_field(&object, "minReward"),
                max_reward: extract_object_u64_field(&object, "maxReward"),
            })
        })
        .collect()
}

fn parse_known_issues(segment: &str) -> Vec<String> {
    parse_jsonish_objects(segment)
        .into_iter()
        .filter_map(|object| {
            extract_object_string_field(&object, "title")
                .or_else(|| extract_object_string_field(&object, "description"))
        })
        .collect()
}

fn parse_jsonish_objects(segment: &str) -> Vec<String> {
    let mut objects = Vec::new();
    let mut start = None;
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for (index, ch) in segment.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        if ch == '{' {
            if depth == 0 {
                start = Some(index);
            }
            depth += 1;
        } else if ch == '}' {
            depth = depth.saturating_sub(1);
            if depth == 0
                && let Some(start_index) = start.take()
            {
                objects.push(segment[start_index..index + ch.len_utf8()].to_string());
            }
        }
    }
    objects
}

fn clean_json_text(raw: &str) -> String {
    raw.replace("\\n", "\n")
        .replace("\\/", "/")
        .replace("\\\"", "\"")
        .replace("\\u0026", "&")
        .replace("\\u2019", "'")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immunefi_urls_normalize_from_any_tab() {
        let urls =
            normalize_immunefi_bounty_urls("https://immunefi.com/bug-bounty/ssvnetwork/resources/")
                .unwrap();

        assert_eq!(urls.base, "https://immunefi.com/bug-bounty/ssvnetwork");
        assert_eq!(
            urls.information,
            "https://immunefi.com/bug-bounty/ssvnetwork/information/"
        );
        assert_eq!(
            urls.scope,
            "https://immunefi.com/bug-bounty/ssvnetwork/scope/"
        );
        assert_eq!(
            urls.resources,
            "https://immunefi.com/bug-bounty/ssvnetwork/resources/"
        );
    }

    #[test]
    fn github_codebase_parser_extracts_tree_branch_and_path() {
        let parsed =
            parse_github_codebase_url("https://github.com/org/repo/tree/main/packages/contracts")
                .unwrap();

        assert_eq!(parsed.repo_url, "https://github.com/org/repo");
        assert_eq!(parsed.branch, Some("main".to_string()));
        assert_eq!(parsed.tree_paths, vec!["packages/contracts"]);
    }

    #[test]
    fn github_codebase_parser_keeps_full_blob_ref_for_slash_named_branch() {
        let parsed =
            parse_github_codebase_url("https://github.com/org/repo/blob/release/v2/src/Foo.sol")
                .unwrap();

        assert_eq!(parsed.repo_url, "https://github.com/org/repo");
        assert_eq!(parsed.branch, None);
        assert_eq!(parsed.raw_tree_refs, vec!["release/v2/src/Foo.sol"]);
    }

    #[test]
    fn github_codebase_parser_rejects_org_repository_listing() {
        assert!(
            parse_github_codebase_url(
                "https://github.com/orgs/ensdomains/repositories?utm_source=immunefi"
            )
            .is_none()
        );
        assert!(is_github_org_or_user_listing_url(
            "https://github.com/orgs/ensdomains/repositories?utm_source=immunefi"
        ));
    }

    #[test]
    fn github_codebase_extraction_keeps_multiple_tree_paths_for_same_repo() {
        let text = "Codebases: https://github.com/org/repo/tree/main/packages/a and https://github.com/org/repo/tree/main/packages/b";

        let repos =
            merge_git_codebase_candidates("Example", github_codebase_urls_from_text(text)).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].repo_url, "https://github.com/org/repo");
        assert_eq!(repos[0].branch.as_deref(), Some("main"));
        assert_eq!(
            repos[0].tree_paths,
            vec!["packages/a".to_string(), "packages/b".to_string()]
        );
    }

    #[test]
    fn immunefi_codebase_resolution_prefers_smart_contract_asset_repo() {
        let urls =
            normalize_immunefi_bounty_urls("https://immunefi.com/bug-bounty/ens/information/")
                .unwrap();
        let html = r#"
<script>self.__next_f.push([1,"{\"project\":\"ENS\",\"programCodebases\":[{\"id\":88,\"url\":\"https://github.com/orgs/ensdomains/repositories\",\"title\":\"ENS Codebase\",\"description\":\"Program Codebase\"}],\"assets\":[{\"id\":\"web\",\"url\":\"https://github.com/ensdomains/ens-app-v3\",\"type\":\"websites_and_applications\",\"description\":\"ENS app source code\",\"isPrimacyOfImpact\":false},{\"id\":\"sc\",\"url\":\"https://github.com/ensdomains/ens-contracts/wiki/ENS-Contract-Deployments\",\"type\":\"smart_contract\",\"description\":\"Smart Contracts\",\"isPrimacyOfImpact\":false},{\"id\":\"poi\",\"url\":\"https://immunefi.com\",\"type\":\"smart_contract\",\"description\":\"Primacy of Impact\",\"isPrimacyOfImpact\":true}],\"programImpacts\":[{\"id\":1,\"severity\":\"critical\",\"assetType\":\"websites_and_applications\",\"description\":\"XSS\"},{\"id\":2,\"severity\":\"critical\",\"assetType\":\"smart_contract\",\"description\":\"Direct theft of funds\"}],\"programRewards\":[{\"id\":1,\"severity\":\"critical\",\"assetType\":\"websites_and_applications\",\"fixedReward\":25000,\"rewardModel\":\"fixed\"},{\"id\":2,\"severity\":\"critical\",\"assetType\":\"smart_contract\",\"maxReward\":250000,\"minReward\":10000,\"rewardModel\":\"range\"}]}"]);</script>
"#;
        let data = parse_immunefi_bounty_data(
            "https://immunefi.com/bug-bounty/ens/information/",
            urls,
            vec![ImmunefiTabCapture {
                kind: ImmunefiTabKind::Scope,
                url: "https://immunefi.com/bug-bounty/ens/scope/".to_string(),
                html: html.to_string(),
            }],
        );

        let repos = data.resolved_git_codebases().unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(
            repos[0].repo_url,
            "https://github.com/ensdomains/ens-contracts"
        );
        assert_eq!(data.smart_contract_impacts().len(), 1);
        assert_eq!(data.smart_contract_rewards().len(), 1);
    }

    #[test]
    fn mixed_immunefi_program_rejects_generic_web_resource_repo_for_sc_audit() {
        let urls =
            normalize_immunefi_bounty_urls("https://immunefi.com/bug-bounty/example/information/")
                .unwrap();
        let html = r#"
<script>self.__next_f.push([1,"{\"project\":\"Mixed\",\"programCodebases\":[{\"id\":1,\"url\":\"https://github.com/example/web-app\",\"title\":\"App source code\",\"description\":\"Web application repository\"}],\"assets\":[{\"id\":\"web\",\"url\":\"https://app.example\",\"type\":\"websites_and_applications\",\"description\":\"Web app\",\"isPrimacyOfImpact\":false},{\"id\":\"sc\",\"url\":\"https://etherscan.io/address/0x0000000000000000000000000000000000000001\",\"type\":\"smart_contract\",\"description\":\"Vault\",\"isPrimacyOfImpact\":false}],\"programImpacts\":[{\"id\":1,\"severity\":\"critical\",\"assetType\":\"smart_contract\",\"description\":\"Direct theft of funds\"}]}"]);</script>
"#;

        let data = parse_immunefi_bounty_data(
            "https://immunefi.com/bug-bounty/example/information/",
            urls,
            vec![ImmunefiTabCapture {
                kind: ImmunefiTabKind::Resources,
                url: "https://immunefi.com/bug-bounty/example/resources/".to_string(),
                html: html.to_string(),
            }],
        );

        let err = data.resolved_git_codebases().unwrap_err().to_string();

        assert!(err.contains("Specify `repo` manually"));
    }

    #[test]
    fn mixed_immunefi_program_accepts_generic_program_codebase_repo() {
        let urls =
            normalize_immunefi_bounty_urls("https://immunefi.com/bug-bounty/ethena/information/")
                .unwrap();
        let html = r#"
<script>self.__next_f.push([1,"{\"project\":\"Ethena\",\"programCodebases\":[{\"id\":113,\"url\":\"https://github.com/ethena-labs/bbp-public-assets?utm_source=immunefi\",\"title\":\"Ethena Codebase\",\"description\":\"Program Codebase\"}],\"assets\":[{\"id\":\"web\",\"url\":\"https://app.example\",\"type\":\"websites_and_applications\",\"description\":\"Web app\",\"isPrimacyOfImpact\":false},{\"id\":\"sc\",\"url\":\"https://etherscan.io/address/0x4c9EDD5852cd905f086C759E8383e09bff1E68B3\",\"type\":\"smart_contract\",\"description\":\"USDe.sol\",\"isPrimacyOfImpact\":false}],\"programImpacts\":[{\"id\":1,\"severity\":\"critical\",\"assetType\":\"smart_contract\",\"description\":\"Direct theft of funds\"}]}"]);</script>
"#;

        let data = parse_immunefi_bounty_data(
            "https://immunefi.com/bug-bounty/ethena/information/",
            urls,
            vec![ImmunefiTabCapture {
                kind: ImmunefiTabKind::Resources,
                url: "https://immunefi.com/bug-bounty/ethena/resources/".to_string(),
                html: html.to_string(),
            }],
        );

        let repos = data.resolved_git_codebases().unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(
            repos[0].repo_url,
            "https://github.com/ethena-labs/bbp-public-assets"
        );
    }

    #[test]
    fn mixed_immunefi_program_accepts_smart_contract_specific_resource_repo() {
        let urls =
            normalize_immunefi_bounty_urls("https://immunefi.com/bug-bounty/example/information/")
                .unwrap();
        let html = r#"
<script>self.__next_f.push([1,"{\"project\":\"Mixed\",\"programCodebases\":[{\"id\":1,\"url\":\"https://github.com/example/protocol-contracts\",\"title\":\"Smart Contract Codebase\",\"description\":\"Solidity contracts repository\"}],\"assets\":[{\"id\":\"web\",\"url\":\"https://app.example\",\"type\":\"websites_and_applications\",\"description\":\"Web app\",\"isPrimacyOfImpact\":false},{\"id\":\"sc\",\"url\":\"https://etherscan.io/address/0x0000000000000000000000000000000000000001\",\"type\":\"smart_contract\",\"description\":\"Vault\",\"isPrimacyOfImpact\":false}],\"programImpacts\":[{\"id\":1,\"severity\":\"critical\",\"assetType\":\"smart_contract\",\"description\":\"Direct theft of funds\"}]}"]);</script>
"#;

        let data = parse_immunefi_bounty_data(
            "https://immunefi.com/bug-bounty/example/information/",
            urls,
            vec![ImmunefiTabCapture {
                kind: ImmunefiTabKind::Resources,
                url: "https://immunefi.com/bug-bounty/example/resources/".to_string(),
                html: html.to_string(),
            }],
        );

        let repos = data.resolved_git_codebases().unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(
            repos[0].repo_url,
            "https://github.com/example/protocol-contracts"
        );
    }

    #[test]
    fn immunefi_codebase_resolution_uses_smart_contract_documentation_repo() {
        let urls =
            normalize_immunefi_bounty_urls("https://immunefi.com/bug-bounty/example/information/")
                .unwrap();
        let html = r#"
<script>self.__next_f.push([1,"{\"project\":\"The Graph\",\"programCodebases\":[{\"id\":1,\"url\":\"https://github.com/graphprotocol\",\"title\":\"The Graph Codebase\",\"description\":\"Github Codebase\"}],\"programDocumentations\":[{\"id\":2,\"url\":\"https://github.com/graphprotocol/contracts/tree/main/packages/contracts/audits\",\"title\":\"Past Security Audits\",\"description\":\"A list of all security audits performed since 2020.\"},{\"id\":3,\"url\":\"https://github.com/graphprotocol/contracts/tree/main/packages\",\"title\":\"Smart Contracts\",\"description\":\"Each Smart Contract in scope has its own documentation (README.md)\"}],\"assets\":[{\"id\":\"web\",\"url\":\"https://app.example\",\"type\":\"websites_and_applications\",\"description\":\"Web app\",\"isPrimacyOfImpact\":false},{\"id\":\"sc\",\"url\":\"https://etherscan.io/address/0x0000000000000000000000000000000000000001\",\"type\":\"smart_contract\",\"description\":\"Vault\",\"isPrimacyOfImpact\":false}],\"programImpacts\":[{\"id\":1,\"severity\":\"critical\",\"assetType\":\"smart_contract\",\"description\":\"Direct theft of funds\"}]}"]);</script>
"#;

        let data = parse_immunefi_bounty_data(
            "https://immunefi.com/bug-bounty/example/information/",
            urls,
            vec![ImmunefiTabCapture {
                kind: ImmunefiTabKind::Resources,
                url: "https://immunefi.com/bug-bounty/example/resources/".to_string(),
                html: html.to_string(),
            }],
        );

        let repos = data.resolved_git_codebases().unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(
            repos[0].repo_url,
            "https://github.com/graphprotocol/contracts"
        );
        assert_eq!(repos[0].branch.as_deref(), Some("main"));
        assert_eq!(repos[0].tree_paths, vec!["packages".to_string()]);
    }

    #[test]
    fn immunefi_data_parser_accepts_rewards_field_alias() {
        let urls =
            normalize_immunefi_bounty_urls("https://immunefi.com/bug-bounty/example/information/")
                .unwrap();
        let html = r#"
<script>self.__next_f.push([1,"{\"project\":\"Example\",\"assets\":[{\"id\":\"a\",\"url\":\"https://etherscan.io/address/0x0000000000000000000000000000000000000001\",\"type\":\"smart_contract\",\"description\":\"Vault\",\"isPrimacyOfImpact\":false}],\"impacts\":[{\"id\":1,\"severity\":\"critical\",\"assetType\":\"smart_contract\",\"description\":\"Direct theft of funds\"}],\"rewards\":[{\"id\":2,\"severity\":\"critical\",\"assetType\":\"smart_contract\",\"maxReward\":100000,\"minReward\":10000,\"rewardModel\":\"range\"}]}"]);</script>
"#;

        let data = parse_immunefi_bounty_data(
            "https://immunefi.com/bug-bounty/example/information/",
            urls,
            vec![ImmunefiTabCapture {
                kind: ImmunefiTabKind::Information,
                url: "https://immunefi.com/bug-bounty/example/information/".to_string(),
                html: html.to_string(),
            }],
        );

        assert_eq!(data.smart_contract_rewards().len(), 1);
        assert_eq!(data.smart_contract_rewards()[0].max_reward, Some(100000));
        assert_eq!(data.smart_contract_impacts().len(), 1);
    }

    #[test]
    fn immunefi_data_parser_extracts_codebase_assets_and_impacts() {
        let urls = normalize_immunefi_bounty_urls(
            "https://immunefi.com/bug-bounty/ssvnetwork/information/",
        )
        .unwrap();
        let html = r#"
<script>self.__next_f.push([1,"{\"project\":\"SSV Network\",\"githubUrl\":\"https://github.com/ssvlabs/ssv-network\",\"programCodebases\":[{\"id\":675,\"url\":\"https://github.com/ssvlabs/ssv-network\",\"title\":\"SSV Network Codebase\",\"description\":\"Github Codebase\"}],\"programDocumentations\":[{\"id\":48,\"url\":\"https://docs.ssv.network/developers/smart-contracts\",\"title\":\"SSV Network Documentation\",\"description\":\"Program Documentation\"}],\"assets\":[{\"id\":\"a\",\"url\":\"https://etherscan.io/address/0xDD9BC35aE942eF0cFa76930954a156B3fF30a4E1\",\"type\":\"smart_contract\",\"description\":\"SSV Network\",\"isPrimacyOfImpact\":false},{\"id\":\"p\",\"url\":\"https://immunefi.com\",\"type\":\"smart_contract\",\"description\":\"Primacy of Impact\",\"isPrimacyOfImpact\":true}],\"programImpacts\":[{\"id\":15,\"severity\":\"critical\",\"assetType\":\"smart_contract\",\"description\":\"Direct theft of funds\"}],\"defaultOutOfScopeGeneral\":\"- leaked keys\"}"]);</script>
"#;
        let data = parse_immunefi_bounty_data(
            "https://immunefi.com/bug-bounty/ssvnetwork/information/",
            urls,
            vec![ImmunefiTabCapture {
                kind: ImmunefiTabKind::Information,
                url: "https://immunefi.com/bug-bounty/ssvnetwork/information/".to_string(),
                html: html.to_string(),
            }],
        );

        assert_eq!(data.project, "SSV Network");
        assert_eq!(
            data.codebases[0].url,
            "https://github.com/ssvlabs/ssv-network"
        );
        assert_eq!(
            data.documentations[0].url,
            "https://docs.ssv.network/developers/smart-contracts"
        );
        assert_eq!(data.assets.len(), 2);
        assert!(data.assets[1].is_primacy_of_impact);
        assert_eq!(data.impacts[0].description, "Direct theft of funds");
        assert_eq!(
            data.resolved_git_codebase().unwrap().repo_url,
            "https://github.com/ssvlabs/ssv-network"
        );
    }

    #[test]
    fn immunefi_data_parser_detects_severity_system_version() {
        let urls =
            normalize_immunefi_bounty_urls("https://immunefi.com/bug-bounty/example/information/")
                .unwrap();
        let data = parse_immunefi_bounty_data(
            "https://immunefi.com/bug-bounty/example/information/",
            urls,
            vec![ImmunefiTabCapture {
                kind: ImmunefiTabKind::Information,
                url: "https://immunefi.com/bug-bounty/example/information/".to_string(),
                html: "Rewards according to Immunefi Vulnerability Severity Classification System V2.2".to_string(),
            }],
        );

        let system = data.severity_system.unwrap();
        assert_eq!(system.version, "v2.2");
        assert_eq!(
            system.url,
            "https://immunefi.com/immunefi-vulnerability-severity-classification-system-v2-2/"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn live_fetch_ssv_bounty_extracts_current_codebase_and_scope_assets() {
        let data = fetch_immunefi_bounty("https://immunefi.com/bug-bounty/ssvnetwork/information/")
            .await
            .unwrap();

        assert_eq!(data.project, "SSV Network");
        assert_eq!(
            data.resolved_git_codebase().unwrap().repo_url,
            "https://github.com/ssvlabs/ssv-network"
        );
        assert!(
            data.assets
                .iter()
                .any(|asset| asset.url.contains("etherscan.io/address/"))
        );
        assert!(!data.impacts.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn live_fetch_ens_bounty_prefers_smart_contract_deployment_repo() {
        let data = fetch_immunefi_bounty("https://immunefi.com/bug-bounty/ens/information/")
            .await
            .unwrap();

        let repos = data.resolved_git_codebases().unwrap();
        assert!(
            repos
                .iter()
                .any(|repo| repo.repo_url == "https://github.com/ensdomains/ens-contracts")
        );
        assert!(
            data.smart_contract_assets(false)
                .iter()
                .any(|asset| asset.url.contains("ENS-Contract-Deployments"))
        );
        assert!(
            data.smart_contract_impacts()
                .iter()
                .all(|impact| impact.asset_type.as_deref() == Some("smart_contract"))
        );
    }
}
