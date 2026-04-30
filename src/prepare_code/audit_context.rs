use anyhow::{anyhow, Context, Result};
use log::{debug, info, warn};
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::{
    cli_args::parse::{Cli, ContextConfig, V12Source},
    config::{AuditType, OPENAI_MODEL, OPENAI_REASONING_EFFORT},
    cost::cost_data::get_token_count,
    llm_review::agent::{
        agent_enums::AIAgent,
        agent_factory::{AgentConfig, AgentFactory},
    },
};

const MAX_PROMPT_SOURCE_TOKENS_PER_ITEM: usize = 2_500;
const MAX_CONTEXT_BUNDLE_TOKENS: usize = 18_000;
const MARKDOWN_COMPRESSION_ATTEMPTS: usize = 3;
const HTTP_TIMEOUT_SECS: u64 = 20;
const MAX_REMOTE_LINK_FETCHES: usize = 12;
const MAX_REMOTE_PRIOR_AUDIT_FETCHES: usize = 2;
const ENTRY_CONTEXT_REASON: &str = "Configured context file";

#[derive(Debug, Clone)]
pub struct GeneratedAuditContext {
    pub artifact_prefix: String,
    pub output_dir: PathBuf,
    pub scope_txt: PathBuf,
    pub scope_md: PathBuf,
    pub docs_md: PathBuf,
    pub sources_json: PathBuf,
    pub regenerated: bool,
    pub source_report: ContextSourceReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSourceReport {
    pub artifact_prefix: String,
    pub commit_hash: String,
    pub generated_at: String,
    pub sources: Vec<ContextSource>,
    pub link_decisions: Vec<LinkDecision>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSource {
    pub id: String,
    pub kind: ContextSourceKind,
    pub location: String,
    pub title: Option<String>,
    pub token_count: usize,
    pub decision: SourceDecision,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextSourceKind {
    LocalReadme,
    LocalMarkdown,
    LocalScopeTxt,
    LocalKnownIssues,
    GithubMarkdown,
    GithubRaw,
    WebMarkdown,
    WebHtml,
    V12Report,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceDecision {
    UsedForScope,
    UsedForDocs,
    UsedForBoth,
    Skipped,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkDecision {
    pub from: String,
    pub url: String,
    pub classification: LinkClassification,
    pub action: LinkAction,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LinkClassification {
    Scope,
    Documentation,
    KnownIssues,
    PriorAudit,
    V12,
    SourceCode,
    Social,
    Marketing,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkAction {
    Fetched,
    ResolvedLocal,
    Skipped,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeFileList {
    pub files: Vec<ScopeFileEntry>,
    pub source: ScopeFileSource,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeFileEntry {
    pub path: String,
    pub exists: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScopeFileSource {
    CopiedScopeTxt,
    ExtractedFromReadme,
    FallbackCodeFolders,
}

#[derive(Debug, Clone)]
struct SourceContent {
    id: String,
    kind: ContextSourceKind,
    location: String,
    title: Option<String>,
    content: String,
    decision: SourceDecision,
    reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct GeneratedMarkdown {
    markdown: String,
    omitted_items: Vec<String>,
    source_notes: Vec<String>,
}

pub async fn generate_audit_context(
    cli: &Cli,
    workspace_root: &Path,
    protocol_root: &Path,
    repo_name: &str,
    commit_hash: &str,
) -> Result<GeneratedAuditContext> {
    let context_config = cli.context.clone().unwrap_or_default();
    let artifact_prefix = artifact_prefix_from_repo_name(repo_name);
    let output_dir = context_output_dir(&context_config.output_dir)?;
    fs::create_dir_all(&output_dir).with_context(|| {
        format!(
            "Failed to create generated audit context directory {}",
            output_dir.display()
        )
    })?;

    let scope_txt = output_dir.join(format!("{artifact_prefix}-scope.txt"));
    let scope_md = output_dir.join(format!("{artifact_prefix}-scope.md"));
    let docs_md = output_dir.join(format!("{artifact_prefix}-docs.md"));
    let sources_json = output_dir.join(format!("{artifact_prefix}-context-sources.json"));

    let mut report = ContextSourceReport {
        artifact_prefix: artifact_prefix.clone(),
        commit_hash: commit_hash.to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        sources: Vec::new(),
        link_decisions: Vec::new(),
        warnings: Vec::new(),
    };

    let all_outputs_exist = scope_txt.exists() && scope_md.exists() && docs_md.exists();
    if all_outputs_exist && !context_config.force_regenerate {
        info!(
            "Reusing generated audit context files in {}",
            output_dir.display()
        );
        return Ok(GeneratedAuditContext {
            artifact_prefix,
            output_dir,
            scope_txt,
            scope_md,
            docs_md,
            sources_json,
            regenerated: false,
            source_report: report,
        });
    }

    info!(
        "Generating audit context artifacts for {} from {}",
        artifact_prefix,
        protocol_root.display()
    );

    let (sources, link_decisions) = collect_context_sources(
        &context_config,
        workspace_root,
        protocol_root,
        repo_name,
        &cli.audit_type,
    )
    .await?;
    report.link_decisions = link_decisions;
    info!(
        "Collected {} context sources and {} entry/second-level link decisions",
        sources.len(),
        report.link_decisions.len()
    );

    for source in &sources {
        let token_count = get_token_count(&source.content);
        info!(
            "Context source included: id={}, kind={:?}, decision={:?}, tokens={}, location={}",
            source.id, source.kind, source.decision, token_count, source.location
        );
        report.sources.push(ContextSource {
            id: source.id.clone(),
            kind: source.kind.clone(),
            location: source.location.clone(),
            title: source.title.clone(),
            token_count,
            decision: source.decision.clone(),
            reason: source.reason.clone(),
        });
    }

    let scope_file_list = generate_scope_txt(cli, protocol_root, &scope_txt, &sources).await?;
    report.warnings.extend(scope_file_list.warnings.clone());

    let agent = build_context_agent()?;
    let scope_context_bundle = build_context_bundle(
        &artifact_prefix,
        repo_name,
        commit_hash,
        &sources,
        &scope_file_list,
        &report.link_decisions,
        ContextBundlePurpose::Scope,
    );
    let docs_context_bundle = build_context_bundle(
        &artifact_prefix,
        repo_name,
        commit_hash,
        &sources,
        &scope_file_list,
        &report.link_decisions,
        ContextBundlePurpose::Docs,
    );

    let scope_markdown = generate_scope_markdown(
        &agent,
        &context_config,
        &artifact_prefix,
        &scope_context_bundle,
        context_config.max_tokens_per_file,
    )
    .await?;
    let scope_markdown_tokens = get_token_count(&scope_markdown);
    fs::write(&scope_md, &scope_markdown)
        .with_context(|| format!("Failed to write {}", scope_md.display()))?;
    info!(
        "Wrote generated scope markdown: path={}, tokens={}, limit={}",
        scope_md.display(),
        scope_markdown_tokens,
        context_config.max_tokens_per_file
    );

    let docs_markdown = generate_docs_markdown(
        &agent,
        &context_config,
        &artifact_prefix,
        &docs_context_bundle,
        context_config.max_tokens_per_file,
    )
    .await?;
    let docs_markdown_tokens = get_token_count(&docs_markdown);
    fs::write(&docs_md, &docs_markdown)
        .with_context(|| format!("Failed to write {}", docs_md.display()))?;
    info!(
        "Wrote generated docs markdown: path={}, tokens={}, limit={}",
        docs_md.display(),
        docs_markdown_tokens,
        context_config.max_tokens_per_file
    );

    fs::write(&sources_json, serde_json::to_string_pretty(&report)?)
        .with_context(|| format!("Failed to write {}", sources_json.display()))?;
    info!(
        "Wrote generated context source report: path={}",
        sources_json.display()
    );

    Ok(GeneratedAuditContext {
        artifact_prefix,
        output_dir,
        scope_txt,
        scope_md,
        docs_md,
        sources_json,
        regenerated: true,
        source_report: report,
    })
}

pub fn should_generate_context(cli: &Cli) -> bool {
    let has_complete_manual_context =
        cli.custom_doc.is_some() && cli.audit_scope.is_some() && cli.scoped_files.is_some();

    match &cli.context {
        Some(config) => config.force_regenerate || !has_complete_manual_context,
        None => !has_complete_manual_context,
    }
}

fn build_context_agent() -> Result<AIAgent> {
    AgentFactory::create_openai_agent(
        &AgentConfig::new(None)
            .with_model(OPENAI_MODEL)
            .with_openai_reasoning_effort(OPENAI_REASONING_EFFORT)
            .with_preamble(
                "You generate concise, security-review-ready audit context for Solidity protocol audits. \
                 Preserve scope, known issues, V12/prior findings, invariants, trusted roles, and protocol mechanics. \
                 Do not invent facts. If source material is uncertain, say so briefly.",
            ),
    )
    .map_err(anyhow::Error::from)
}

async fn collect_context_sources(
    config: &ContextConfig,
    workspace_root: &Path,
    protocol_root: &Path,
    repo_name: &str,
    audit_type: &AuditType,
) -> Result<(Vec<SourceContent>, Vec<LinkDecision>)> {
    let mut sources = Vec::new();
    let mut link_decisions = Vec::new();
    let mut seen_locations = HashSet::new();
    let code4rena_audit = matches!(audit_type, AuditType::Code4rena);
    let explicit_v12_url = matches!(config.v12_url, V12Source::Url(_));

    let configured_files = context_files(config);
    debug!(
        "Codex context discovery: reading configured context files: {}",
        configured_files.join(", ")
    );
    for file in configured_files {
        let path = protocol_root.join(&file);
        if path.exists() {
            debug!(
                "Codex context discovery: loading local context file {}",
                path.display()
            );
            push_local_source(
                &mut sources,
                &mut seen_locations,
                if file.eq_ignore_ascii_case("README.md") {
                    ContextSourceKind::LocalReadme
                } else {
                    ContextSourceKind::LocalMarkdown
                },
                &path,
                SourceDecision::UsedForBoth,
                ENTRY_CONTEXT_REASON,
            )?;
        } else {
            warn!("Configured context file missing: {}", path.display());
        }
    }

    let known_issue_files = discover_known_issue_files(protocol_root);
    debug!(
        "Codex context discovery: found {} possible known-issues/security/audit markdown files",
        known_issue_files.len()
    );
    for path in known_issue_files {
        debug!(
            "Codex context discovery: loading possible known-issues file {}",
            path.display()
        );
        push_local_source(
            &mut sources,
            &mut seen_locations,
            ContextSourceKind::LocalKnownIssues,
            &path,
            SourceDecision::UsedForScope,
            "Potential known-issues or prior-audit file",
        )?;
    }

    if protocol_root.join("scope.txt").exists() {
        debug!(
            "Codex context discovery: found repository scope.txt at {}",
            protocol_root.join("scope.txt").display()
        );
        push_local_source(
            &mut sources,
            &mut seen_locations,
            ContextSourceKind::LocalScopeTxt,
            &protocol_root.join("scope.txt"),
            SourceDecision::UsedForScope,
            "Repository-provided machine-readable scope",
        )?;
    }

    let mut links = Vec::new();
    for source in &sources {
        if !should_extract_links_from_source(source) {
            debug!(
                "Codex context discovery: not extracting links from non-entry source {}; second-level link traversal is terminal",
                source.location
            );
            continue;
        }
        let source_links = extract_links(&source.content, &source.location);
        debug!(
            "Codex context discovery: extracted {} links from {}",
            source_links.len(),
            source.location
        );
        links.extend(source_links);
    }
    debug!(
        "Codex context discovery: collected {} second-level candidates from entry context files",
        links.len()
    );
    for url in &config.urls {
        debug!(
            "Codex context discovery: adding configured context URL {}",
            url
        );
        links.push(ExtractedLink {
            from: "context.urls".to_string(),
            url: url.clone(),
            label: url.clone(),
        });
    }
    let source_v12_link_count = links
        .iter()
        .filter(|link| classify_link(&link.url, &link.label) == LinkClassification::V12)
        .count();
    let v12_links = if !code4rena_audit && matches!(config.v12_url, V12Source::Auto) {
        debug!(
            "Codex context discovery: skipping V12 auto lookup because audit_type={:?}; V12 context is Code4rena-only",
            audit_type
        );
        Vec::new()
    } else if matches!(config.v12_url, V12Source::Auto) && source_v12_link_count > 0 {
        debug!(
            "Codex context discovery: found {source_v12_link_count} V12 report links in entry context sources; skipping external V12 search"
        );
        Vec::new()
    } else {
        v12_candidate_links(config, repo_name).await
    };
    debug!(
        "Codex context discovery: found {} V12 candidate links",
        v12_links.len()
    );
    links.extend(v12_links);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(HTTP_TIMEOUT_SECS))
        .user_agent("ai-agent-audit-context-generator/0.1")
        .build()?;

    let deduped_links = dedupe_links(links);
    debug!(
        "Codex context discovery: evaluating {} unique second-level/configured URL candidates",
        deduped_links.len()
    );
    let mut remote_fetch_count = 0usize;
    let mut prior_audit_remote_fetch_count = 0usize;
    let mut skipped_by_reason = BTreeMap::<String, usize>::new();
    let mut failed_count = 0usize;
    let mut accepted_count = 0usize;
    for link in deduped_links {
        let classification = classify_link(&link.url, &link.label);
        debug!(
            "Codex context discovery: checking link from={}, classification={:?}, url={}",
            link.from, classification, link.url
        );
        if classification == LinkClassification::V12 && !code4rena_audit && !explicit_v12_url {
            let reason = format!(
                "Skipped V12 link because audit_type={audit_type:?}; V12 is Code4rena-only"
            );
            *skipped_by_reason.entry(reason.clone()).or_default() += 1;
            link_decisions.push(LinkDecision {
                from: link.from.clone(),
                url: link.url.clone(),
                classification,
                action: LinkAction::Skipped,
                reason,
            });
            continue;
        }
        if let Some(reason) = link_skip_reason(&link.url, &classification) {
            *skipped_by_reason.entry(reason.to_string()).or_default() += 1;
            link_decisions.push(LinkDecision {
                from: link.from.clone(),
                url: link.url.clone(),
                classification,
                action: LinkAction::Skipped,
                reason: reason.to_string(),
            });
            continue;
        }

        let remote_link = is_remote_link(&link.url);
        if remote_link {
            if remote_fetch_count >= MAX_REMOTE_LINK_FETCHES {
                let reason = format!(
                    "Skipped remote link after reaching fetch budget of {MAX_REMOTE_LINK_FETCHES}"
                );
                *skipped_by_reason.entry(reason.clone()).or_default() += 1;
                link_decisions.push(LinkDecision {
                    from: link.from.clone(),
                    url: link.url.clone(),
                    classification,
                    action: LinkAction::Skipped,
                    reason,
                });
                continue;
            }

            if classification == LinkClassification::PriorAudit
                && prior_audit_remote_fetch_count >= MAX_REMOTE_PRIOR_AUDIT_FETCHES
            {
                let reason = format!(
                    "Skipped prior-audit link after reaching fetch budget of {MAX_REMOTE_PRIOR_AUDIT_FETCHES}"
                );
                *skipped_by_reason.entry(reason.clone()).or_default() += 1;
                link_decisions.push(LinkDecision {
                    from: link.from.clone(),
                    url: link.url.clone(),
                    classification,
                    action: LinkAction::Skipped,
                    reason,
                });
                continue;
            }
        }

        debug!(
            "Codex context discovery: following second-level link classification={:?}, url={}",
            classification, link.url
        );
        if remote_link {
            remote_fetch_count += 1;
            if classification == LinkClassification::PriorAudit {
                prior_audit_remote_fetch_count += 1;
            }
        }
        match resolve_or_fetch_link(
            &client,
            workspace_root,
            protocol_root,
            &link.url,
            &classification,
        )
        .await
        {
            Ok(Some(source)) => {
                debug!(
                    "Codex context discovery: accepted second-level link as {:?}: {}",
                    source.kind, source.location
                );
                accepted_count += 1;
                link_decisions.push(LinkDecision {
                    from: link.from.clone(),
                    url: link.url.clone(),
                    classification,
                    action: match source.kind {
                        ContextSourceKind::LocalMarkdown | ContextSourceKind::LocalScopeTxt => {
                            LinkAction::ResolvedLocal
                        }
                        _ => LinkAction::Fetched,
                    },
                    reason: source.reason.clone(),
                });
                if seen_locations.insert(source.location.clone()) {
                    sources.push(source);
                } else {
                    debug!(
                        "Codex context discovery: skipped duplicate linked source {}",
                        source.location
                    );
                }
            }
            Ok(None) => {
                debug!(
                    "Codex context discovery: skipped link classification={:?}, url={}",
                    classification, link.url
                );
                let reason = "Link was not relevant to audit scope or protocol docs".to_string();
                *skipped_by_reason.entry(reason.clone()).or_default() += 1;
                link_decisions.push(LinkDecision {
                    from: link.from.clone(),
                    url: link.url.clone(),
                    classification,
                    action: LinkAction::Skipped,
                    reason,
                });
            }
            Err(err) => {
                failed_count += 1;
                debug!(
                    "Codex context discovery: failed link classification={:?}, url={}, error={}",
                    classification, link.url, err
                );
                link_decisions.push(LinkDecision {
                    from: link.from.clone(),
                    url: link.url.clone(),
                    classification,
                    action: LinkAction::Failed,
                    reason: err.to_string(),
                });
            }
        }
    }
    if !skipped_by_reason.is_empty() {
        debug!(
            "Codex context discovery: skipped link summary: {}",
            skipped_by_reason
                .iter()
                .map(|(reason, count)| format!("{count}x {reason}"))
                .collect::<Vec<_>>()
                .join("; ")
        );
    }
    debug!(
        "Codex context discovery: link follow summary: accepted={}, failed={}, remote_fetches={}, prior_audit_remote_fetches={}",
        accepted_count, failed_count, remote_fetch_count, prior_audit_remote_fetch_count
    );

    if sources.is_empty() {
        anyhow::bail!(
            "Could not find README.md or configured context files under {}",
            protocol_root.display()
        );
    }

    Ok((sources, link_decisions))
}

fn context_files(config: &ContextConfig) -> Vec<String> {
    if config.files.is_empty() {
        vec!["README.md".to_string()]
    } else {
        config.files.clone()
    }
}

fn push_local_source(
    sources: &mut Vec<SourceContent>,
    seen_locations: &mut HashSet<String>,
    kind: ContextSourceKind,
    path: &Path,
    decision: SourceDecision,
    reason: &str,
) -> Result<()> {
    let location = path.to_string_lossy().to_string();
    if !seen_locations.insert(location.clone()) {
        return Ok(());
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read context file {}", path.display()))?;
    log_source_content_loaded(&location, &content);
    sources.push(SourceContent {
        id: format!("source-{}", sources.len() + 1),
        kind,
        location,
        title: path.file_name().map(|s| s.to_string_lossy().to_string()),
        content,
        decision,
        reason: reason.to_string(),
    });
    Ok(())
}

fn discover_known_issue_files(protocol_root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(protocol_root)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let rel = path
            .strip_prefix(protocol_root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_ascii_lowercase();
        if rel == "readme.md" {
            continue;
        }
        if rel.contains("known")
            || rel.contains("issue")
            || rel.contains("audit")
            || rel.contains("security")
        {
            paths.push(path.to_path_buf());
        }
    }
    paths.sort();
    paths
}

fn should_extract_links_from_source(source: &SourceContent) -> bool {
    matches!(
        source.kind,
        ContextSourceKind::LocalReadme | ContextSourceKind::LocalMarkdown
    ) && source.reason == ENTRY_CONTEXT_REASON
}

#[derive(Debug, Clone)]
struct ExtractedLink {
    from: String,
    url: String,
    label: String,
}

fn extract_links(content: &str, from: &str) -> Vec<ExtractedLink> {
    let mut links = Vec::new();
    let md_re = Regex::new(r#"\[([^\]]+)\]\(([^)\s]+)(?:\s+"[^"]*")?\)"#).unwrap();
    for caps in md_re.captures_iter(content) {
        let url = normalize_extracted_url(caps.get(2).map(|m| m.as_str()).unwrap_or_default());
        links.push(ExtractedLink {
            from: from.to_string(),
            label: caps
                .get(1)
                .map(|m| m.as_str())
                .unwrap_or_default()
                .to_string(),
            url,
        });
    }

    let html_re = Regex::new(r#"href=["']([^"']+)["']"#).unwrap();
    for caps in html_re.captures_iter(content) {
        let url = caps.get(1).map(|m| m.as_str()).unwrap_or_default();
        links.push(ExtractedLink {
            from: from.to_string(),
            label: url.to_string(),
            url: normalize_extracted_url(url),
        });
    }

    let bare_re = Regex::new(r#"https?://[^\s<>)"']+"#).unwrap();
    for m in bare_re.find_iter(content) {
        links.push(ExtractedLink {
            from: from.to_string(),
            label: m.as_str().to_string(),
            url: normalize_extracted_url(m.as_str()),
        });
    }

    dedupe_links(links)
}

fn normalize_extracted_url(url: &str) -> String {
    let decoded_entities = decode_basic_entities(url.trim());
    let protocol_normalized = if decoded_entities.starts_with("//") {
        format!("https:{}", decoded_entities)
    } else {
        decoded_entities
    };
    unwrap_duckduckgo_redirect(&protocol_normalized).unwrap_or(protocol_normalized)
}

fn unwrap_duckduckgo_redirect(url: &str) -> Option<String> {
    let lower = url.to_ascii_lowercase();
    if !lower.contains("duckduckgo.com/l/") {
        return None;
    }

    let query = url.split('?').nth(1)?;
    for part in query.split('&') {
        let (key, value) = part.split_once('=')?;
        if key == "uddg" {
            return Some(percent_decode_lossy(value));
        }
    }
    None
}

fn dedupe_links(links: Vec<ExtractedLink>) -> Vec<ExtractedLink> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for link in links {
        let key = link.url.trim().trim_end_matches('/').to_string();
        if !key.is_empty() && seen.insert(key) {
            out.push(link);
        }
    }
    out
}

fn classify_link(url: &str, label: &str) -> LinkClassification {
    let text = format!("{} {}", url, label).to_ascii_lowercase();
    if is_v12_report_url(&text) {
        return LinkClassification::V12;
    }
    if text.contains("scope") {
        return LinkClassification::Scope;
    }
    if text.contains("known") || text.contains("issue") || text.contains("limitation") {
        return LinkClassification::KnownIssues;
    }
    if text.contains("audit") || text.ends_with(".pdf") {
        return LinkClassification::PriorAudit;
    }
    if text.contains("docs")
        || text.contains("doc.")
        || text.contains("gitbook")
        || text.contains("guide")
        || text.contains("architecture")
    {
        return LinkClassification::Documentation;
    }
    if text.contains("github.com") && (text.contains("/blob/") || text.contains("/raw/")) {
        return LinkClassification::SourceCode;
    }
    if text.contains("twitter.com")
        || text.contains("x.com/")
        || text.contains("discord")
        || text.contains("t.me/")
        || text.contains("telegram")
    {
        return LinkClassification::Social;
    }
    if text.contains("website") || text.contains("/blog/") || is_generic_vendor_homepage(&text) {
        return LinkClassification::Marketing;
    }
    LinkClassification::Unknown
}

fn is_v12_report_url(text: &str) -> bool {
    text.contains("v12.sh/runs")
        || text.contains("v12.zellic.io/runs")
        || (text.contains("v12") && text.contains("findings") && text.contains(".md"))
}

fn is_generic_vendor_homepage(text: &str) -> bool {
    let primary_url = text
        .split_whitespace()
        .next()
        .unwrap_or(text)
        .trim_end_matches('/');
    matches!(
        primary_url,
        "https://zellic.io" | "https://www.zellic.io" | "https://v12.zellic.io"
    )
}

fn link_skip_reason(url: &str, classification: &LinkClassification) -> Option<&'static str> {
    if is_generic_code4rena_docs(url) {
        return Some(
            "Skipped generic Code4rena documentation link; not protocol-specific scope/docs",
        );
    }

    if is_binary_or_pdf_url(url) {
        return Some("Skipped binary/PDF link; only text and HTML sources are fetched");
    }

    if is_github_tree_url(url) {
        return Some(
            "Skipped GitHub tree/directory link; only text files and documentation pages are fetched",
        );
    }

    if is_github_blob_or_raw_url(url) && !has_text_like_extension(url) {
        return Some(
            "Skipped GitHub blob/raw link without a text extension; likely directory or non-text audit artifact",
        );
    }

    if *classification == LinkClassification::SourceCode {
        return Some(
            "Skipped source-code link; entry link traversal only follows scope, docs, known-issues, and V12 text",
        );
    }

    if *classification == LinkClassification::PriorAudit
        && is_remote_link(url)
        && !has_text_like_extension(url)
    {
        return Some(
            "Skipped prior-audit website link; prior-audit web pages are too noisy unless provided as text/markdown",
        );
    }

    if matches!(
        classification,
        LinkClassification::Social | LinkClassification::Marketing | LinkClassification::Unknown
    ) {
        return Some("Skipped low-signal link classification");
    }

    None
}

fn is_remote_link(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

fn is_generic_code4rena_docs(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.contains("docs.code4rena.com")
}

fn is_github_blob_or_raw_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.contains("raw.githubusercontent.com")
        || (lower.contains("github.com") && lower.contains("/blob/"))
}

fn is_github_tree_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.contains("github.com") && lower.contains("/tree/")
}

fn has_text_like_extension(url: &str) -> bool {
    let Some(extension) = url_extension(url) else {
        return false;
    };
    matches!(
        extension.as_str(),
        "md" | "markdown"
            | "mdx"
            | "txt"
            | "rst"
            | "adoc"
            | "json"
            | "yaml"
            | "yml"
            | "toml"
            | "csv"
            | "sol"
            | "html"
            | "htm"
    )
}

fn is_binary_or_pdf_url(url: &str) -> bool {
    let Some(extension) = url_extension(url) else {
        return false;
    };
    matches!(
        extension.as_str(),
        "pdf"
            | "png"
            | "jpg"
            | "jpeg"
            | "gif"
            | "svg"
            | "webp"
            | "zip"
            | "tar"
            | "gz"
            | "tgz"
            | "bz2"
            | "7z"
            | "mp4"
            | "mov"
            | "avi"
            | "webm"
            | "mp3"
            | "wav"
    )
}

fn url_extension(url: &str) -> Option<String> {
    let clean = url
        .split('#')
        .next()
        .unwrap_or(url)
        .split('?')
        .next()
        .unwrap_or(url)
        .trim_end_matches('/');
    let basename = clean.rsplit('/').next()?;
    let extension = basename.rsplit_once('.')?.1;
    if extension.is_empty() || extension.len() > 12 {
        return None;
    }
    Some(extension.to_ascii_lowercase())
}

async fn resolve_or_fetch_link(
    client: &reqwest::Client,
    workspace_root: &Path,
    protocol_root: &Path,
    url: &str,
    classification: &LinkClassification,
) -> Result<Option<SourceContent>> {
    if matches!(
        classification,
        LinkClassification::Social | LinkClassification::Marketing | LinkClassification::Unknown
    ) {
        debug!(
            "Codex context discovery: not following low-signal link classification={:?}, url={}",
            classification, url
        );
        return Ok(None);
    }

    if let Some(local_path) = resolve_local_link(workspace_root, protocol_root, url) {
        if local_path.exists() && local_path.is_file() {
            debug!(
                "Codex context discovery: resolving local link {} -> {}",
                url,
                local_path.display()
            );
            let decision = match classification {
                LinkClassification::Documentation => SourceDecision::UsedForDocs,
                LinkClassification::Scope
                | LinkClassification::KnownIssues
                | LinkClassification::PriorAudit
                | LinkClassification::V12 => SourceDecision::UsedForScope,
                _ => SourceDecision::UsedForBoth,
            };
            let kind = if local_path.file_name().and_then(|n| n.to_str()) == Some("scope.txt") {
                ContextSourceKind::LocalScopeTxt
            } else {
                ContextSourceKind::LocalMarkdown
            };
            let content = fs::read_to_string(&local_path)?;
            let location = local_path.to_string_lossy().to_string();
            log_source_content_loaded(&location, &content);
            return Ok(Some(SourceContent {
                id: "linked-local".to_string(),
                kind,
                location,
                title: local_path
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string()),
                content,
                decision,
                reason: "Resolved from second-level entry context link".to_string(),
            }));
        }
        debug!(
            "Codex context discovery: local link target not found or not a file: {} -> {}",
            url,
            local_path.display()
        );
        return Ok(None);
    }

    let Some(fetch_url) = github_raw_url(url).or_else(|| {
        if url.starts_with("http://") || url.starts_with("https://") {
            Some(url.to_string())
        } else {
            None
        }
    }) else {
        debug!(
            "Codex context discovery: cannot resolve non-HTTP/non-local link {}",
            url
        );
        return Ok(None);
    };

    debug!(
        "Codex context discovery: fetching remote link classification={:?}, url={}",
        classification, fetch_url
    );
    let response = client.get(&fetch_url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow!(
            "Fetch failed with status {} for {}",
            response.status(),
            fetch_url
        ));
    }
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_ascii_lowercase();
    let text = response.text().await?;
    let is_html = content_type.contains("html") || looks_like_html(&text);
    let content = if is_html { html_to_text(&text) } else { text };
    let kind = match classification {
        LinkClassification::V12 => ContextSourceKind::V12Report,
        _ if fetch_url.contains("raw.githubusercontent.com") => ContextSourceKind::GithubRaw,
        _ if is_html => ContextSourceKind::WebHtml,
        _ => ContextSourceKind::WebMarkdown,
    };
    let decision = match classification {
        LinkClassification::Documentation => SourceDecision::UsedForDocs,
        LinkClassification::Scope
        | LinkClassification::KnownIssues
        | LinkClassification::PriorAudit
        | LinkClassification::V12 => SourceDecision::UsedForScope,
        _ => SourceDecision::UsedForBoth,
    };
    let location = fetch_url;
    log_source_content_loaded(&location, &content);

    Ok(Some(SourceContent {
        id: "linked-remote".to_string(),
        kind,
        location,
        title: Some(url.to_string()),
        content,
        decision,
        reason: "Fetched from second-level entry context link".to_string(),
    }))
}

fn resolve_local_link(workspace_root: &Path, protocol_root: &Path, url: &str) -> Option<PathBuf> {
    if url.starts_with("http://") || url.starts_with("https://") {
        return None;
    }
    if url.starts_with('#') || url.starts_with("mailto:") {
        return None;
    }
    let clean = url
        .split('#')
        .next()
        .unwrap_or(url)
        .split('?')
        .next()
        .unwrap_or(url);
    let path = Path::new(clean);
    if path.is_absolute() {
        Some(workspace_root.join(path.strip_prefix("/").unwrap_or(path)))
    } else {
        Some(protocol_root.join(path))
    }
}

fn github_raw_url(url: &str) -> Option<String> {
    if url.contains("raw.githubusercontent.com") {
        return Some(url.to_string());
    }
    if !url.contains("github.com") || !url.contains("/blob/") {
        return None;
    }
    let clean = url.split('#').next().unwrap_or(url);
    let after_host = clean.split("github.com/").nth(1)?;
    let parts = after_host.splitn(5, '/').collect::<Vec<_>>();
    if parts.len() < 5 || parts[2] != "blob" {
        return None;
    }
    Some(format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        parts[0], parts[1], parts[3], parts[4]
    ))
}

async fn v12_candidate_links(config: &ContextConfig, repo_name: &str) -> Vec<ExtractedLink> {
    match &config.v12_url {
        V12Source::Disabled => {
            debug!("Codex context discovery: V12 lookup disabled by config");
            Vec::new()
        }
        V12Source::Url(url) => {
            debug!(
                "Codex context discovery: using configured V12 report URL {}",
                url
            );
            vec![ExtractedLink {
                from: "context.v12_url".to_string(),
                url: url.clone(),
                label: "V12 report".to_string(),
            }]
        }
        V12Source::Auto => {
            debug!(
                "Codex context discovery: last-resort public V12 search for {} after entry sources yielded no V12 links",
                repo_name
            );
            search_v12_links(repo_name).await
        }
    }
}

async fn search_v12_links(repo_name: &str) -> Vec<ExtractedLink> {
    let query = format!("site:v12.sh/runs OR site:v12.zellic.io {}", repo_name);
    let url = format!("https://duckduckgo.com/html/?q={}", percent_encode(&query));
    debug!("Codex context discovery: V12 search query: {}", query);
    let Ok(response) = reqwest::Client::new()
        .get(&url)
        .header(
            reqwest::header::USER_AGENT,
            "ai-agent-audit-context-generator/0.1",
        )
        .send()
        .await
    else {
        debug!("Codex context discovery: V12 search request failed");
        return Vec::new();
    };
    let Ok(text) = response.text().await else {
        debug!("Codex context discovery: failed to read V12 search response");
        return Vec::new();
    };
    let links = extract_links(&text, "v12-search")
        .into_iter()
        .filter(|link| {
            let lower = link.url.to_ascii_lowercase();
            lower.contains("v12.sh") || lower.contains("v12.zellic.io")
        })
        .take(5)
        .collect::<Vec<_>>();
    for link in &links {
        debug!(
            "Codex context discovery: V12 candidate discovered: {}",
            link.url
        );
    }
    links
}

async fn generate_scope_txt(
    cli: &Cli,
    protocol_root: &Path,
    output_path: &Path,
    sources: &[SourceContent],
) -> Result<ScopeFileList> {
    let source_scope = protocol_root.join("scope.txt");
    if source_scope.exists() {
        info!(
            "Codex task: creating scope.txt by copying repository-provided scope file {}",
            source_scope.display()
        );
        let raw = fs::read_to_string(&source_scope)?;
        let files = normalize_scope_lines(&raw, protocol_root);
        write_scope_lines(output_path, &files)?;
        let missing_count = files.iter().filter(|entry| !entry.exists).count();
        info!(
            "Wrote generated scope file list: path={}, source={:?}, files={}, missing={}",
            output_path.display(),
            ScopeFileSource::CopiedScopeTxt,
            files.len(),
            missing_count
        );
        return Ok(ScopeFileList {
            files,
            source: ScopeFileSource::CopiedScopeTxt,
            warnings: Vec::new(),
        });
    }

    let mut extracted = deterministic_scope_extract(sources, protocol_root);
    let mut source = ScopeFileSource::ExtractedFromReadme;
    let mut warnings = Vec::new();
    info!(
        "Deterministic entry-context scope extraction found {} candidate Solidity files",
        extracted.len()
    );

    extracted.sort_by(|a, b| a.path.cmp(&b.path));
    extracted.dedup_by(|a, b| a.path == b.path);
    let existing_count = extracted.iter().filter(|entry| entry.exists).count();
    if extracted.is_empty() || existing_count == 0 {
        let fallback = fallback_scope_from_code_folders(cli, protocol_root);
        if fallback.is_empty() {
            anyhow::bail!(
                "Could not generate {} from entry context files and found no Solidity files under configured code_folders {:?}. Provide context.files or a scope.txt.",
                output_path.display(),
                cli.code_folders
            );
        }

        let message = if extracted.is_empty() {
            format!(
                "No explicit scope.txt or README scope file list was found; falling back to all Solidity files under configured code_folders {:?}",
                cli.code_folders
            )
        } else {
            format!(
                "Extracted {} scope candidates but none resolved locally; falling back to all Solidity files under configured code_folders {:?}",
                extracted.len(),
                cli.code_folders
            )
        };
        warn!("{message}");
        warnings.push(message);
        extracted = fallback;
        source = ScopeFileSource::FallbackCodeFolders;
    }
    write_scope_lines(output_path, &extracted)?;
    let missing_count = extracted.iter().filter(|entry| !entry.exists).count();
    info!(
        "Wrote generated scope file list: path={}, source={:?}, files={}, missing={}",
        output_path.display(),
        source,
        extracted.len(),
        missing_count
    );
    Ok(ScopeFileList {
        files: extracted,
        source,
        warnings,
    })
}

fn normalize_scope_lines(raw: &str, protocol_root: &Path) -> Vec<ScopeFileEntry> {
    raw.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| normalize_scope_path(line, protocol_root, None))
        .collect()
}

fn deterministic_scope_extract(
    sources: &[SourceContent],
    protocol_root: &Path,
) -> Vec<ScopeFileEntry> {
    let sol_path_re = Regex::new(r#"(?x)(?:\.?/)?[A-Za-z0-9_./-]+\.sol"#).unwrap();
    let scope_dir_re = Regex::new(r#"(?x)(?:^|[\s`|])((?:\.?/)?[A-Za-z0-9_./-]+/)\s*$"#).unwrap();
    let mut entries = Vec::new();
    for source in sources {
        if !matches!(
            source.kind,
            ContextSourceKind::LocalReadme
                | ContextSourceKind::LocalMarkdown
                | ContextSourceKind::GithubMarkdown
                | ContextSourceKind::GithubRaw
                | ContextSourceKind::WebMarkdown
        ) {
            continue;
        }
        for section in likely_scope_sections(&source.content) {
            let mut current_scope_dir: Option<String> = None;
            for line in section.lines() {
                if let Some(captures) = scope_dir_re.captures(line)
                    && let Some(dir) = captures.get(1)
                {
                    current_scope_dir = Some(dir.as_str().to_string());
                    continue;
                }

                for m in sol_path_re.find_iter(line) {
                    let raw = m.as_str();
                    let scoped_path = if raw.contains('/') {
                        raw.to_string()
                    } else if let Some(dir) = &current_scope_dir {
                        format!("{}/{}", dir.trim_end_matches('/'), raw)
                    } else {
                        raw.to_string()
                    };
                    entries.push(normalize_scope_path(
                        &scoped_path,
                        protocol_root,
                        Some("Extracted from scope section".to_string()),
                    ));
                }
            }
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    entries.dedup_by(|a, b| a.path == b.path);
    entries
}

fn fallback_scope_from_code_folders(cli: &Cli, protocol_root: &Path) -> Vec<ScopeFileEntry> {
    let mut entries = Vec::new();

    for folder in &cli.code_folders {
        let trimmed_folder = folder.trim();
        if trimmed_folder.is_empty() {
            continue;
        }

        let folder_path = protocol_root.join(trimmed_folder);
        if !folder_path.exists() {
            warn!(
                "Scope fallback skipped missing configured code folder: {}",
                folder_path.display()
            );
            continue;
        }

        for entry in WalkDir::new(&folder_path)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("sol") {
                continue;
            }

            let Ok(relative) = path.strip_prefix(protocol_root) else {
                continue;
            };
            let relative = relative.to_string_lossy().replace('\\', "/");
            if scope_fallback_excludes(&relative, cli) {
                continue;
            }

            entries.push(ScopeFileEntry {
                path: format!("./{}", relative.trim_start_matches("./")),
                exists: true,
                reason: Some(format!(
                    "Fallback from configured code folder `{trimmed_folder}`"
                )),
            });
        }
    }

    entries.sort_by(|a, b| a.path.cmp(&b.path));
    entries.dedup_by(|a, b| a.path == b.path);
    entries
}

fn scope_fallback_excludes(relative: &str, cli: &Cli) -> bool {
    let Some(excluded_folders) = &cli.exclude_folders else {
        return false;
    };
    let relative = relative.trim_start_matches("./");

    excluded_folders.iter().any(|folder| {
        let folder = folder.trim().trim_start_matches("./").trim_end_matches('/');
        !folder.is_empty() && (relative == folder || relative.starts_with(&format!("{folder}/")))
    })
}

fn likely_scope_sections(content: &str) -> Vec<String> {
    let mut sections = Vec::new();
    let mut current = Vec::new();
    let mut in_scope = false;

    for line in content.lines() {
        let heading = line.trim_start().starts_with('#');
        if heading {
            let lower = line.to_ascii_lowercase();
            if in_scope && !current.is_empty() {
                sections.push(current.join("\n"));
                current.clear();
            }
            in_scope = lower.contains("scope")
                || lower.contains("files in")
                || lower.contains("contracts in");
        }
        if in_scope {
            current.push(line.to_string());
        }
    }
    if in_scope && !current.is_empty() {
        sections.push(current.join("\n"));
    }
    sections
}

fn normalize_scope_path(raw: &str, protocol_root: &Path, reason: Option<String>) -> ScopeFileEntry {
    let mut path = raw
        .trim()
        .trim_matches('`')
        .trim_matches('|')
        .trim()
        .trim_start_matches('/')
        .to_string();
    if path.starts_with("blob/") || path.starts_with("tree/") {
        path = path.split('/').skip(2).collect::<Vec<_>>().join("/");
    }
    if !path.starts_with("./") {
        path = format!("./{}", path);
    }
    let exists = protocol_root.join(path.trim_start_matches("./")).exists();
    ScopeFileEntry {
        path,
        exists,
        reason,
    }
}

fn write_scope_lines(path: &Path, files: &[ScopeFileEntry]) -> Result<()> {
    let mut content = files
        .iter()
        .map(|entry| entry.path.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    content.push('\n');
    fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))
}

async fn generate_scope_markdown(
    agent: &AIAgent,
    config: &ContextConfig,
    artifact_prefix: &str,
    context_bundle: &str,
    max_tokens: usize,
) -> Result<String> {
    info!(
        "Codex task: creating scope.md from entry/second-level sources: artifact_prefix={}, context_bundle_tokens={}, limit={}",
        artifact_prefix,
        get_token_count(context_bundle),
        max_tokens
    );
    let prompt = format!(
        r#"
Create `{artifact_prefix}-scope.md` for a Solidity security audit.

Hard requirements:
- Markdown only.
- Be highly discriminating: include only material that changes audit scope, threat model, assumptions, exclusions, or reviewer priorities.
- Include ALL relevant audit scope context from the source material, but summarize aggressively when the raw source is long.
- Include public known issues, files in/out of scope, areas of concern, invariants, trusted roles, and V12/prior findings when present.
- For V12/prior findings, do not copy full reports. Summarize finding titles, affected areas, and audit implications.
- Prefer exact file/path tables for scope. Prefer concise summaries for prose-heavy docs and historical reports.
- If a source is generic, duplicated, marketing-oriented, or low-signal, omit it and record that in `omitted_items`.
- Do not invent facts or files.
- Keep the final markdown at or below {max_tokens} tokens.
- Prefer concise tables and bullets.
- Before returning, self-edit once for token economy: remove repetition, background narrative, and anything an auditor does not need during review.

Source material:
{context_bundle}
"#
    );
    let generated: GeneratedMarkdown = agent.extract_with_retry(&prompt).await?;
    info!(
        "Codex task: received scope.md draft: tokens={}, omitted_items={}, source_notes={}",
        get_token_count(&generated.markdown),
        generated.omitted_items.len(),
        generated.source_notes.len()
    );
    enforce_markdown_token_limit(
        agent,
        generated.markdown,
        max_tokens,
        "audit scope",
        config.max_tokens_per_file,
    )
    .await
}

async fn generate_docs_markdown(
    agent: &AIAgent,
    config: &ContextConfig,
    artifact_prefix: &str,
    context_bundle: &str,
    max_tokens: usize,
) -> Result<String> {
    info!(
        "Codex task: creating docs.md from entry/second-level sources: artifact_prefix={}, context_bundle_tokens={}, limit={}",
        artifact_prefix,
        get_token_count(context_bundle),
        max_tokens
    );
    let prompt = format!(
        r#"
Create `{artifact_prefix}-docs.md` for a Solidity security audit.

Hard requirements:
- Markdown only.
- Explain what the protocol does and how it works.
- Focus on architecture, main flows, accounting/value flow, external integrations, trust boundaries, and security-relevant assumptions.
- Pull only useful protocol documentation from entry and second-level sources.
- Be highly discriminating: include docs that help an auditor understand mechanics, assets, permissions, invariants, integrations, and failure modes.
- Summarize large documentation pages instead of copying them. Do not include exhaustive docs, changelogs, marketing copy, setup instructions, or generic contest rules.
- If multiple sources repeat the same concept, merge them into one concise explanation and cite/source-note the strongest source.
- Skip marketing fluff unless it affects the threat model.
- If a source is low-signal or tangential, omit it and record that in `omitted_items`.
- Do not invent facts.
- Keep the final markdown at or below {max_tokens} tokens.
- Before returning, self-edit once for token economy: remove repetition, onboarding prose, and details that do not help security review.

Source material:
{context_bundle}
"#
    );
    let generated: GeneratedMarkdown = agent.extract_with_retry(&prompt).await?;
    info!(
        "Codex task: received docs.md draft: tokens={}, omitted_items={}, source_notes={}",
        get_token_count(&generated.markdown),
        generated.omitted_items.len(),
        generated.source_notes.len()
    );
    enforce_markdown_token_limit(
        agent,
        generated.markdown,
        max_tokens,
        "protocol docs",
        config.max_tokens_per_file,
    )
    .await
}

async fn enforce_markdown_token_limit(
    agent: &AIAgent,
    markdown: String,
    max_tokens: usize,
    doc_kind: &str,
    configured_limit: usize,
) -> Result<String> {
    let effective_limit = max_tokens.min(configured_limit);
    let initial_tokens = get_token_count(&markdown);
    if initial_tokens <= effective_limit {
        info!(
            "Generated {doc_kind} markdown is within token limit: tokens={}, limit={}",
            initial_tokens, effective_limit
        );
        return Ok(markdown);
    }

    warn!(
        "Generated {doc_kind} markdown exceeds token limit; asking model to summarize/compress: tokens={}, limit={}",
        initial_tokens, effective_limit
    );

    let mut current_markdown = markdown;
    let mut current_tokens = initial_tokens;
    for attempt in 1..=MARKDOWN_COMPRESSION_ATTEMPTS {
        let prompt = format!(
            r#"
Summarize and compress this {doc_kind} markdown to <= {effective_limit} tokens.

Preserve required sections, scope, known issues, summarized V12/prior findings, invariants, trusted roles, and security-relevant protocol mechanics. Remove repetition, copied report prose, duplicated source material, generic docs, and low-value background first. Prefer concise tables and bullets.

Do not blindly truncate the tail. Rewrite overlong sections into compact summaries so important context is retained.

Return JSON with fields:
- markdown
- omitted_items
- source_notes

Markdown:
{current_markdown}
"#
        );
        let compressed: GeneratedMarkdown = agent.extract_with_retry(&prompt).await?;
        let compressed_tokens = get_token_count(&compressed.markdown);
        info!(
            "Codex compression attempt for {doc_kind}: attempt={}, before_tokens={}, after_tokens={}, limit={}, omitted_items={}, source_notes={}",
            attempt,
            current_tokens,
            compressed_tokens,
            effective_limit,
            compressed.omitted_items.len(),
            compressed.source_notes.len()
        );

        if compressed_tokens <= effective_limit {
            info!(
                "Compressed {doc_kind} markdown is within token limit: tokens={}, limit={}",
                compressed_tokens, effective_limit
            );
            return Ok(compressed.markdown);
        }

        current_markdown = compressed.markdown;
        current_tokens = compressed_tokens;
    }

    anyhow::bail!(
        "Generated {doc_kind} markdown still exceeds the {effective_limit}-token limit after {MARKDOWN_COMPRESSION_ATTEMPTS} Codex summarization attempts (last draft: {current_tokens} tokens). Refusing to truncate because truncation can drop audit-relevant context."
    )
}

#[derive(Debug, Clone, Copy)]
enum ContextBundlePurpose {
    Scope,
    Docs,
}

fn build_context_bundle(
    artifact_prefix: &str,
    repo_name: &str,
    commit_hash: &str,
    sources: &[SourceContent],
    scope_file_list: &ScopeFileList,
    link_decisions: &[LinkDecision],
    purpose: ContextBundlePurpose,
) -> String {
    let mut bundle = String::new();
    bundle.push_str(&format!(
        "# Context Bundle ({:?})\n\nArtifact prefix: `{}`\nRepository: `{}`\nCommit: `{}`\n\n",
        purpose, artifact_prefix, repo_name, commit_hash
    ));
    bundle.push_str("## Machine-Readable Scope\n\n");
    for file in &scope_file_list.files {
        bundle.push_str(&format!(
            "- {}{}\n",
            file.path,
            if file.exists {
                ""
            } else {
                " (not found locally)"
            }
        ));
    }
    bundle.push_str("\n## Link Decisions\n\n");
    let skipped_count = link_decisions
        .iter()
        .filter(|decision| matches!(decision.action, LinkAction::Skipped))
        .count();
    if skipped_count > 0 {
        bundle.push_str(&format!(
            "- Skipped {} low-signal, binary, directory, or over-budget links during context discovery.\n",
            skipped_count
        ));
    }
    for decision in link_decisions.iter().filter(|decision| {
        !matches!(decision.action, LinkAction::Skipped)
            && link_decision_relevant_to_purpose(decision, purpose)
    }) {
        bundle.push_str(&format!(
            "- {:?}: {} -> {:?} ({})\n",
            decision.classification, decision.url, decision.action, decision.reason
        ));
    }
    bundle.push_str("\n## Source Material\n\n");
    let included_sources = append_sources_for_prompt_with_budget(
        &mut bundle,
        sources,
        purpose,
        MAX_CONTEXT_BUNDLE_TOKENS,
    );
    info!(
        "Context bundle built: purpose={:?}, included_sources={}, omitted_sources={}, tokens={}, limit={}, prompt_source_token_limit={}",
        purpose,
        included_sources.included,
        included_sources.omitted,
        get_token_count(&bundle),
        MAX_CONTEXT_BUNDLE_TOKENS,
        MAX_PROMPT_SOURCE_TOKENS_PER_ITEM
    );
    bundle
}

#[derive(Debug, Default)]
struct PromptSourceBudgetResult {
    included: usize,
    omitted: usize,
}

fn append_sources_for_prompt_with_budget(
    bundle: &mut String,
    sources: &[SourceContent],
    purpose: ContextBundlePurpose,
    max_bundle_tokens: usize,
) -> PromptSourceBudgetResult {
    let mut result = PromptSourceBudgetResult::default();
    let mut omitted = Vec::new();

    for source in sources
        .iter()
        .filter(|source| source_relevant_to_purpose(source, purpose))
    {
        let rendered = render_source_for_prompt(source);
        let candidate_tokens = get_token_count(&format!("{}{}", bundle, rendered));
        if candidate_tokens <= max_bundle_tokens {
            bundle.push_str(&rendered);
            result.included += 1;
        } else {
            result.omitted += 1;
            omitted.push(format!(
                "- {} ({:?}, {:?}, {} tokens)",
                source.location,
                source.kind,
                source.decision,
                get_token_count(&source.content)
            ));
        }
    }

    if !omitted.is_empty() {
        let detailed_note = format!(
            "\n### Omitted Source Material\nThe following available sources were omitted because the {:?} context bundle reached the {} token budget:\n{}\n",
            purpose,
            max_bundle_tokens,
            omitted.join("\n")
        );
        if get_token_count(&format!("{}{}", bundle, detailed_note)) <= max_bundle_tokens + 500 {
            bundle.push_str(&detailed_note);
        } else {
            bundle.push_str(&format!(
                "\n### Omitted Source Material\n{} available sources were omitted because the {:?} context bundle reached the {} token budget.\n",
                omitted.len(),
                purpose,
                max_bundle_tokens
            ));
        }
    }

    result
}

fn render_source_for_prompt(source: &SourceContent) -> String {
    let source_tokens = get_token_count(&source.content);
    let content = if source_tokens > MAX_PROMPT_SOURCE_TOKENS_PER_ITEM {
        truncate_to_token_limit(source.content.clone(), MAX_PROMPT_SOURCE_TOKENS_PER_ITEM)
    } else {
        source.content.clone()
    };

    format!(
        "\n### Source: {} ({:?})\nLocation: {}\nDecision: {:?}\nOriginal tokens: {}\nPrompt excerpt tokens: {}\n\n{}\n",
        source.title.as_deref().unwrap_or(&source.id),
        source.kind,
        source.location,
        source.decision,
        source_tokens,
        get_token_count(&content),
        content
    )
}

fn source_relevant_to_purpose(source: &SourceContent, purpose: ContextBundlePurpose) -> bool {
    match purpose {
        ContextBundlePurpose::Scope => matches!(
            source.decision,
            SourceDecision::UsedForScope | SourceDecision::UsedForBoth
        ),
        ContextBundlePurpose::Docs => matches!(
            source.decision,
            SourceDecision::UsedForDocs | SourceDecision::UsedForBoth
        ),
    }
}

fn link_decision_relevant_to_purpose(
    decision: &LinkDecision,
    purpose: ContextBundlePurpose,
) -> bool {
    match purpose {
        ContextBundlePurpose::Scope => matches!(
            decision.classification,
            LinkClassification::Scope
                | LinkClassification::KnownIssues
                | LinkClassification::PriorAudit
                | LinkClassification::V12
        ),
        ContextBundlePurpose::Docs => matches!(
            decision.classification,
            LinkClassification::Documentation | LinkClassification::SourceCode
        ),
    }
}

fn context_output_dir(output_dir: &str) -> Result<PathBuf> {
    let path = PathBuf::from(output_dir);
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn artifact_prefix_from_repo_name(repo_name: &str) -> String {
    let mut prefix = repo_name.trim_end_matches(".git").to_string();
    prefix = prefix
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    if prefix.is_empty() {
        "protocol".to_string()
    } else {
        prefix
    }
}

fn looks_like_html(text: &str) -> bool {
    let prefix = text
        .chars()
        .take(512)
        .collect::<String>()
        .to_ascii_lowercase();
    prefix.contains("<html") || prefix.contains("<!doctype html") || prefix.contains("<body")
}

fn html_to_text(html: &str) -> String {
    let without_scripts = Regex::new(r"(?is)<script[^>]*>.*?</script>|<style[^>]*>.*?</style>")
        .unwrap()
        .replace_all(html, " ");
    let with_newlines = Regex::new(r"(?i)</?(p|br|div|li|h[1-6]|tr|table|section|article)[^>]*>")
        .unwrap()
        .replace_all(&without_scripts, "\n");
    let stripped = Regex::new(r"(?s)<[^>]+>")
        .unwrap()
        .replace_all(&with_newlines, " ");
    decode_basic_entities(&stripped)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn decode_basic_entities(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

fn log_source_content_loaded(location: &str, content: &str) {
    info!(
        "Context source loaded: location={}, tokens={}",
        location,
        get_token_count(content)
    );
}

fn truncate_to_token_limit(content: String, max_tokens: usize) -> String {
    if get_token_count(&content) <= max_tokens {
        return content;
    }

    let marker = "\n\n[Prompt excerpt shortened by audit context generator]\n";
    let marker_tokens = get_token_count(marker);
    let body_budget = max_tokens.saturating_sub(marker_tokens).max(1);
    let mut boundaries = content
        .char_indices()
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    boundaries.push(content.len());

    let mut low = 0usize;
    let mut high = boundaries.len();
    let mut best = 0usize;

    while low < high {
        let mid = low + (high - low) / 2;
        let byte_index = boundaries[mid];
        if get_token_count(&content[..byte_index]) <= body_budget {
            best = byte_index;
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    let mut body = content[..best].trim_end().to_string();
    while !body.is_empty() && get_token_count(&format!("{body}{marker}")) > max_tokens {
        body.pop();
    }
    body.push_str(marker);
    body
}

fn percent_encode(input: &str) -> String {
    input
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (byte as char).to_string()
            }
            b' ' => "+".to_string(),
            _ => format!("%{:02X}", byte),
        })
        .collect()
}

fn percent_decode_lossy(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let (Some(high), Some(low)) =
                (hex_value(bytes[index + 1]), hex_value(bytes[index + 2]))
        {
            out.push(high * 16 + low);
            index += 3;
            continue;
        }
        out.push(if bytes[index] == b'+' {
            b' '
        } else {
            bytes[index]
        });
        index += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_prefix_preserves_repo_folder_identity() {
        assert_eq!(
            artifact_prefix_from_repo_name("2026-04-monetrix"),
            "2026-04-monetrix"
        );
        assert_eq!(
            artifact_prefix_from_repo_name("2025-11-sequence/contracts"),
            "2025-11-sequence-contracts"
        );
        assert_eq!(artifact_prefix_from_repo_name("MyProtocol"), "MyProtocol");
        assert_eq!(artifact_prefix_from_repo_name("my-protocol"), "my-protocol");
    }

    #[test]
    fn github_blob_urls_become_raw_urls() {
        assert_eq!(
            github_raw_url("https://github.com/org/repo/blob/main/docs/a.md").unwrap(),
            "https://raw.githubusercontent.com/org/repo/main/docs/a.md"
        );
    }

    #[test]
    fn scope_lines_are_normalized() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(tmp.path().join("src/A.sol"), "").unwrap();

        let lines = normalize_scope_lines("\n/src/A.sol\n# comment\n./src/B.sol\n", tmp.path());
        assert_eq!(lines[0].path, "./src/A.sol");
        assert!(lines[0].exists);
        assert_eq!(lines[1].path, "./src/B.sol");
        assert!(!lines[1].exists);
    }

    #[test]
    fn deterministic_scope_extract_preserves_directory_tree_parent() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(tmp.path().join("src/PuppyRaffle.sol"), "").unwrap();
        let sources = vec![SourceContent {
            id: "source-1".to_string(),
            kind: ContextSourceKind::LocalReadme,
            location: "README.md".to_string(),
            title: None,
            content: r#"
# Audit Scope Details

```
./src/
└── PuppyRaffle.sol
```
"#
            .to_string(),
            decision: SourceDecision::UsedForBoth,
            reason: ENTRY_CONTEXT_REASON.to_string(),
        }];

        let entries = deterministic_scope_extract(&sources, tmp.path());

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "./src/PuppyRaffle.sol");
        assert!(entries[0].exists);
    }

    #[test]
    fn fallback_scope_uses_configured_code_folders() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src/nested")).unwrap();
        fs::create_dir_all(tmp.path().join("test")).unwrap();
        fs::write(tmp.path().join("src/A.sol"), "").unwrap();
        fs::write(tmp.path().join("src/nested/B.sol"), "").unwrap();
        fs::write(tmp.path().join("test/OutOfScope.t.sol"), "").unwrap();
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
code_folders:
  - "src"
"#,
        )
        .unwrap();

        let entries = fallback_scope_from_code_folders(&cli, tmp.path());
        let paths = entries
            .iter()
            .map(|entry| entry.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(paths, vec!["./src/A.sol", "./src/nested/B.sol"]);
        assert!(entries.iter().all(|entry| entry.exists));
    }

    #[tokio::test]
    async fn scope_txt_falls_back_to_code_folders_when_extracted_paths_do_not_exist() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(tmp.path().join("src/PuppyRaffle.sol"), "").unwrap();
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
code_folders:
  - "src"
"#,
        )
        .unwrap();
        let sources = vec![SourceContent {
            id: "source-1".to_string(),
            kind: ContextSourceKind::LocalReadme,
            location: "README.md".to_string(),
            title: None,
            content: r#"
# Audit Scope Details

- In Scope: `PuppyRaffle.sol`
"#
            .to_string(),
            decision: SourceDecision::UsedForBoth,
            reason: ENTRY_CONTEXT_REASON.to_string(),
        }];
        let output_path = tmp.path().join("generated-scope.txt");

        let list = generate_scope_txt(&cli, tmp.path(), &output_path, &sources)
            .await
            .unwrap();

        assert!(matches!(list.source, ScopeFileSource::FallbackCodeFolders));
        assert_eq!(
            fs::read_to_string(output_path).unwrap(),
            "./src/PuppyRaffle.sol\n"
        );
    }

    #[tokio::test]
    async fn scope_txt_uses_code_folders_when_readme_has_no_explicit_scope() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("contracts/core")).unwrap();
        fs::write(tmp.path().join("contracts/Token.sol"), "").unwrap();
        fs::write(tmp.path().join("contracts/core/Vault.sol"), "").unwrap();
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
code_folders:
  - "contracts"
"#,
        )
        .unwrap();
        let sources = vec![SourceContent {
            id: "source-1".to_string(),
            kind: ContextSourceKind::LocalReadme,
            location: "README.md".to_string(),
            title: None,
            content: "# Protocol\n\nGeneral protocol docs without an audit scope section."
                .to_string(),
            decision: SourceDecision::UsedForBoth,
            reason: ENTRY_CONTEXT_REASON.to_string(),
        }];
        let output_path = tmp.path().join("generated-scope.txt");

        let list = generate_scope_txt(&cli, tmp.path(), &output_path, &sources)
            .await
            .unwrap();

        assert!(matches!(list.source, ScopeFileSource::FallbackCodeFolders));
        assert_eq!(
            fs::read_to_string(output_path).unwrap(),
            "./contracts/Token.sol\n./contracts/core/Vault.sol\n"
        );
    }

    #[test]
    fn prompt_excerpt_shortening_is_token_bounded_and_utf8_safe() {
        let content = format!("{}{}", "scope🚀 ".repeat(10_000), "終");
        let truncated = truncate_to_token_limit(content, 100);

        assert!(get_token_count(&truncated) <= 100);
        assert!(truncated.contains("[Prompt excerpt shortened by audit context generator]"));
        assert!(std::str::from_utf8(truncated.as_bytes()).is_ok());
    }

    #[test]
    fn prompt_excerpt_shortening_keeps_small_utf8_content_unchanged() {
        let content = "README scope 🚀 終".to_string();

        assert_eq!(truncate_to_token_limit(content.clone(), 1_000), content);
    }

    #[test]
    fn link_classification_identifies_docs_and_scope() {
        assert_eq!(
            classify_link("https://docs.example.com", "Docs"),
            LinkClassification::Documentation
        );
        assert_eq!(
            classify_link("https://github.com/org/repo/blob/main/scope.txt", "scope"),
            LinkClassification::Scope
        );
    }

    #[test]
    fn link_filter_skips_binary_and_extensionless_github_blob_links() {
        assert_eq!(
            link_skip_reason(
                "https://github.com/valory-xyz/autonolas-governance/blob/main/audits/internal7",
                &LinkClassification::PriorAudit,
            ),
            Some(
                "Skipped GitHub blob/raw link without a text extension; likely directory or non-text audit artifact",
            )
        );
        assert_eq!(
            link_skip_reason(
                "https://example.com/audit.pdf",
                &LinkClassification::PriorAudit,
            ),
            Some("Skipped binary/PDF link; only text and HTML sources are fetched")
        );
        assert_eq!(
            link_skip_reason(
                "https://github.com/org/repo/blob/main/docs/scope.md",
                &LinkClassification::Scope,
            ),
            None
        );
        assert_eq!(
            link_skip_reason(
                "https://docs.example.com/protocol",
                &LinkClassification::Documentation,
            ),
            None
        );
        assert_eq!(
            link_skip_reason(
                "https://github.com/org/repo/blob/main/contracts/Vault.sol",
                &LinkClassification::SourceCode,
            ),
            Some(
                "Skipped source-code link; entry link traversal only follows scope, docs, known-issues, and V12 text",
            )
        );
    }

    #[test]
    fn link_filter_skips_generic_docs_and_prior_audit_websites() {
        assert_eq!(
            link_skip_reason(
                "https://docs.code4rena.com/competitions",
                &LinkClassification::Documentation,
            ),
            Some("Skipped generic Code4rena documentation link; not protocol-specific scope/docs")
        );
        assert_eq!(
            link_skip_reason(
                "https://sourcehat.com/audits/ValoryAgentRegistries/",
                &LinkClassification::PriorAudit,
            ),
            Some(
                "Skipped prior-audit website link; prior-audit web pages are too noisy unless provided as text/markdown"
            )
        );
    }

    #[test]
    fn link_extraction_only_runs_for_entry_context_sources() {
        let primary = SourceContent {
            id: "source-1".to_string(),
            kind: ContextSourceKind::LocalReadme,
            location: "README.md".to_string(),
            title: None,
            content: String::new(),
            decision: SourceDecision::UsedForBoth,
            reason: ENTRY_CONTEXT_REASON.to_string(),
        };
        let secondary = SourceContent {
            id: "source-2".to_string(),
            kind: ContextSourceKind::LocalKnownIssues,
            location: "audits/README.md".to_string(),
            title: None,
            content: String::new(),
            decision: SourceDecision::UsedForScope,
            reason: "Potential known-issues or prior-audit file".to_string(),
        };

        assert!(should_extract_links_from_source(&primary));
        assert!(!should_extract_links_from_source(&secondary));
    }

    #[test]
    fn context_bundles_keep_scope_and_docs_sources_separate() {
        let sources = vec![
            SourceContent {
                id: "source-1".to_string(),
                kind: ContextSourceKind::LocalReadme,
                location: "README.md".to_string(),
                title: None,
                content: "shared entry context".to_string(),
                decision: SourceDecision::UsedForBoth,
                reason: ENTRY_CONTEXT_REASON.to_string(),
            },
            SourceContent {
                id: "source-2".to_string(),
                kind: ContextSourceKind::V12Report,
                location: "v12.md".to_string(),
                title: None,
                content: "scope only V12 findings".to_string(),
                decision: SourceDecision::UsedForScope,
                reason: "Fetched from second-level entry context link".to_string(),
            },
            SourceContent {
                id: "source-3".to_string(),
                kind: ContextSourceKind::WebMarkdown,
                location: "docs.md".to_string(),
                title: None,
                content: "docs only architecture".to_string(),
                decision: SourceDecision::UsedForDocs,
                reason: "Fetched from second-level entry context link".to_string(),
            },
        ];
        let scope_files = ScopeFileList {
            files: vec![ScopeFileEntry {
                path: "./src/A.sol".to_string(),
                exists: true,
                reason: None,
            }],
            source: ScopeFileSource::ExtractedFromReadme,
            warnings: Vec::new(),
        };
        let decisions = Vec::new();

        let scope_bundle = build_context_bundle(
            "protocol",
            "protocol",
            "abc123",
            &sources,
            &scope_files,
            &decisions,
            ContextBundlePurpose::Scope,
        );
        let docs_bundle = build_context_bundle(
            "protocol",
            "protocol",
            "abc123",
            &sources,
            &scope_files,
            &decisions,
            ContextBundlePurpose::Docs,
        );

        assert!(scope_bundle.contains("shared entry context"));
        assert!(scope_bundle.contains("scope only V12 findings"));
        assert!(!scope_bundle.contains("docs only architecture"));
        assert!(docs_bundle.contains("shared entry context"));
        assert!(docs_bundle.contains("docs only architecture"));
        assert!(!docs_bundle.contains("scope only V12 findings"));
    }

    #[test]
    fn extracted_links_unwrap_duckduckgo_redirects() {
        assert_eq!(
            normalize_extracted_url(
                "//duckduckgo.com/l/?uddg=https%3A%2F%2Fv12.sh%2Fruns%2F1378%2Fpublic&amp;rut=abc",
            ),
            "https://v12.sh/runs/1378/public"
        );
    }

    #[test]
    fn classification_keeps_generic_zellic_pages_out_of_v12_reports() {
        assert_eq!(
            classify_link("https://zellic.io", "Zellic"),
            LinkClassification::Marketing
        );
        assert_eq!(
            classify_link(
                "https://www.zellic.io/blog/introducing-v12/",
                "Introducing V12"
            ),
            LinkClassification::Marketing
        );
        assert_eq!(
            classify_link("https://v12.sh/runs/1378/public", "V12 report"),
            LinkClassification::V12
        );
        assert_eq!(
            classify_link(
                "https://github.com/code-423n4/2026-01-olas/blob/main/code_423n4_autonolas_v12_governance__main_e7b6039_findings_2026-01-23-findings.md",
                "V12 findings",
            ),
            LinkClassification::V12
        );
    }

    #[test]
    fn generation_is_skipped_for_legacy_yaml_with_complete_manual_context() {
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
audit_scope: "audit-docs/protocol-scope.md"
custom_doc: "audit-docs/protocol-docs.md"
scoped_files: "audit-docs/protocol-scope.txt"
"#,
        )
        .unwrap();

        assert!(!should_generate_context(&cli));
    }
}
