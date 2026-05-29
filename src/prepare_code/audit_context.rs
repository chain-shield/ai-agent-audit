//! Audit-context artifact generation.
//!
//! This module turns repository files plus bounty/platform metadata into the
//! three core artifacts the audit engine consumes:
//! - `<protocol>-scope.txt`: exact Solidity entrypoints to audit.
//! - `<protocol>-scope.md`: human/LLM-readable scope, exclusions, and known
//!   issue context.
//! - `<protocol>-docs.md`: protocol documentation distilled from trusted
//!   sources.
//!
//! NatSpec-style contract for this module:
//! - `@notice` Generate deterministic, security-review-ready context artifacts
//!   before static analysis and LLM discovery begin.
//! - `@dev` Scope precision is more important than convenience for bounty
//!   audits. Code4rena/Immunefi explorer-linked assets must map to local source
//!   files or the app should fail closed instead of silently auditing too much or
//!   too little.
//! - `@custom:invariant` `scope.txt` paths are always relative to the effective
//!   protocol root. In polyrepo workspaces that means paths include the cloned
//!   member folder, such as `./org-repo/contracts/Vault.sol`.
//! - `@custom:invariant` Generated docs should preserve scope, severity rules,
//!   known issues, previous audits, trusted roles, and PoC/runtime constraints
//!   while filtering platform navigation and marketing noise.
//! - `@custom:safety` External links are fetched under strict budgets; generated
//!   context must prefer structured bounty data and local files over broad web
//!   crawling.

use anyhow::{Context, Result, anyhow};
use log::{debug, info, warn};
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::{
    cli_args::parse::{Cli, ContextConfig, PocConfig, V12Source},
    config::{AuditType, OPENAI_MODEL, OPENAI_REASONING_EFFORT},
    cost::cost_data::get_token_count,
    llm_review::agent::{
        agent_enums::AIAgent,
        agent_factory::{AgentConfig, AgentFactory},
    },
    llm_review::prompt_support::severity_rubics::CODE4RENA_BOUNTY_SEVERITY_RUBRIC,
    prepare_code::code4rena_bounty::{Code4renaBountyData, fetch_code4rena_bounty},
    prepare_code::immunefi::{
        ImmunefiBountyData, ImmunefiTabKind, fetch_immunefi_bounty, same_github_repo,
    },
};

const MAX_PROMPT_SOURCE_TOKENS_PER_ITEM: usize = 2_500;
const MAX_CONTEXT_BUNDLE_TOKENS: usize = 18_000;
const MARKDOWN_COMPRESSION_ATTEMPTS: usize = 3;
const HTTP_TIMEOUT_SECS: u64 = 20;
const MAX_REMOTE_LINK_FETCHES: usize = 12;
const MAX_REMOTE_PRIOR_AUDIT_FETCHES: usize = 2;
const MAX_EXTERNAL_CONTRACT_METADATA_FETCHES: usize = 100;
const MAX_GITHUB_TREE_DOC_FILES: usize = 24;
const MAX_GITHUB_TREE_DOC_TOKENS: usize = 12_000;
const MAX_SCOPED_CODE_DOC_TOKENS_PER_FILE: usize = 1_200;
const ENTRY_CONTEXT_REASON: &str = "Configured context file";
const IMMUNEFI_BOUNTY_ENTRY_REASON: &str = "Configured Immunefi bounty tab";
const CODE4RENA_BOUNTY_ENTRY_REASON: &str = "Configured Code4rena bounty page";
const CODE4RENA_BOUNTY_GUIDE_URL: &str = "https://docs.code4rena.com/bounties";
const CODE4RENA_BOUNTY_CRITERIA_URL: &str = "https://docs.code4rena.com/bounties/bounty-criteria";

#[derive(Debug, Clone)]
/// Paths and metadata for generated audit-context artifacts.
pub struct GeneratedAuditContext {
    /// Stable filename prefix derived from the repo/protocol identity.
    pub artifact_prefix: String,
    /// Directory containing all generated context artifacts.
    pub output_dir: PathBuf,
    /// Generated list of scoped Solidity files.
    pub scope_txt: PathBuf,
    /// Generated markdown scope/rules document.
    pub scope_md: PathBuf,
    /// Generated markdown protocol docs document.
    pub docs_md: PathBuf,
    /// Generated validation-only sidecar; not injected into discovery docs.
    pub validation_md: PathBuf,
    /// JSON report of source files/links considered during generation.
    pub sources_json: PathBuf,
    /// Additional docs injected for bounty modes, such as rubric/runtime files.
    pub extra_docs: Vec<PathBuf>,
    /// Whether artifacts were regenerated in this run.
    pub regenerated: bool,
    /// Structured source and link decision report.
    pub source_report: ContextSourceReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Audit trail for context-source selection and link handling.
pub struct ContextSourceReport {
    /// Artifact prefix used for this generation run.
    pub artifact_prefix: String,
    /// Commit hash/fingerprint associated with the prepared repo workspace.
    pub commit_hash: String,
    /// UTC timestamp string for artifact generation.
    pub generated_at: String,
    /// Local/remote sources considered for scope/docs.
    pub sources: Vec<ContextSource>,
    /// Per-link fetch/skip decisions.
    pub link_decisions: Vec<LinkDecision>,
    /// Non-fatal warnings emitted during generation.
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Public record of one source used or skipped by context generation.
pub struct ContextSource {
    /// Stable source id within the report.
    pub id: String,
    /// Source category.
    pub kind: ContextSourceKind,
    /// File path or URL.
    pub location: String,
    /// Optional human-readable title.
    pub title: Option<String>,
    /// Approximate token count after text extraction.
    pub token_count: usize,
    /// How the source was used.
    pub decision: SourceDecision,
    /// Concise reason for the decision.
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Source categories understood by context generation.
pub enum ContextSourceKind {
    /// Repository README.
    LocalReadme,
    /// Other local markdown file.
    LocalMarkdown,
    /// Local explicit scope file.
    LocalScopeTxt,
    /// Local known-issues/security/audit markdown.
    LocalKnownIssues,
    /// Local protocol documentation discovered from README-adjacent root files, docs/, or audits/.
    LocalProtocolDocs,
    /// Contest README/docs/scope material fetched from a Code4rena contest repository.
    Code4renaContestRepo,
    /// Documentation extracted directly from scoped Solidity source files.
    ScopedCodeDocs,
    /// Remote GitHub markdown page.
    GithubMarkdown,
    /// Remote GitHub raw text file.
    GithubRaw,
    /// Remote markdown-like web page.
    WebMarkdown,
    /// Remote HTML page converted to text.
    WebHtml,
    /// Code4rena V12/prior report.
    V12Report,
    /// Default Code4rena bounty guide.
    Code4renaBountyGuide,
    /// Default Code4rena bounty criteria page.
    Code4renaBountyCriteria,
    /// Program-specific Code4rena bounty page.
    Code4renaBountyPage,
    /// Immunefi Information tab.
    ImmunefiInformation,
    /// Immunefi Scope tab.
    ImmunefiScope,
    /// Immunefi Resources tab.
    ImmunefiResources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// How a source contributed to generated artifacts.
pub enum SourceDecision {
    /// Used only for scope generation.
    UsedForScope,
    /// Used only for docs generation.
    UsedForDocs,
    /// Used for both scope and docs.
    UsedForBoth,
    /// Intentionally skipped.
    Skipped,
    /// Fetch/read failed.
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Audit trail for an extracted link.
pub struct LinkDecision {
    /// Source id/location where the link was found.
    pub from: String,
    /// Link URL.
    pub url: String,
    /// Link category.
    pub classification: LinkClassification,
    /// Fetch/skip outcome.
    pub action: LinkAction,
    /// Explanation of the outcome.
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Link categories used to decide fetch priority and prompt relevance.
pub enum LinkClassification {
    /// Scope, deployment, explorer, or contract-list material.
    Scope,
    /// Protocol docs.
    Documentation,
    /// Known issues / accepted risks.
    KnownIssues,
    /// Prior audit report.
    PriorAudit,
    /// Code4rena V12/prior findings report.
    V12,
    /// Bounty criteria/rules material.
    BountyRules,
    /// Source-code repository/blob/tree.
    SourceCode,
    /// Social/profile link.
    Social,
    /// Marketing or homepage link.
    Marketing,
    /// Unclassified link.
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// What happened to an extracted link.
pub enum LinkAction {
    /// Remote content was fetched.
    Fetched,
    /// Link was resolved to a local repo file.
    ResolvedLocal,
    /// Link was skipped intentionally.
    Skipped,
    /// Fetch/resolve failed.
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Generated `scope.txt` contents plus provenance.
pub struct ScopeFileList {
    /// Scoped Solidity entries.
    pub files: Vec<ScopeFileEntry>,
    /// Strategy that produced the list.
    pub source: ScopeFileSource,
    /// Non-fatal warnings from scope generation.
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// One line in generated `scope.txt`.
pub struct ScopeFileEntry {
    /// Relative path, always formatted with a leading `./`.
    pub path: String,
    /// Whether the path exists under the effective protocol root.
    pub exists: bool,
    /// Optional provenance note.
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Provenance of a generated scope file list.
pub enum ScopeFileSource {
    /// Existing local `scope.txt` was copied.
    CopiedScopeTxt,
    /// README/docs deterministic extraction.
    ExtractedFromReadme,
    /// Structured bounty scope table/list extraction.
    ExtractedFromBountyScope,
    /// Explorer/deployment-page contract metadata mapping.
    ExtractedFromExternalContractScope,
    /// Conservative fallback to configured code folders.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BountyPlatform {
    Code4rena,
    Immunefi,
}

#[derive(Debug, Clone)]
struct BountyArtifactPaths {
    platform: BountyPlatform,
    source_json: PathBuf,
    bounty_rules_md: PathBuf,
    severity_rubric_md: PathBuf,
    severity_rubric_json: PathBuf,
    poc_runtime_md: Option<PathBuf>,
    poc_runtime_json: Option<PathBuf>,
}

#[derive(Debug, Clone)]
enum FetchedBounty {
    Code4rena(Code4renaBountyData),
    Immunefi(ImmunefiBountyData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExternalContractScopeAsset {
    label: String,
    url: String,
    address: String,
    explorer: String,
    source_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExternalContractMetadata {
    url: String,
    contract_names: Vec<String>,
    implementation_addresses: Vec<String>,
    source_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExternalContractResolutionRecord {
    label: String,
    address: String,
    explorer: String,
    url: String,
    paths: Vec<String>,
    confidence: String,
    reason: String,
}

#[derive(Debug, Default)]
struct ExternalContractScopeResolution {
    entries: Vec<ScopeFileEntry>,
    assets: Vec<ExternalContractScopeAsset>,
    resolved: Vec<ExternalContractResolutionRecord>,
    unresolved: Vec<ExternalContractScopeAsset>,
    metadata: Vec<ExternalContractMetadata>,
    warnings: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ExternalContractScopeResolutionReport {
    assets: Vec<ExternalContractScopeAsset>,
    resolved: Vec<ExternalContractResolutionRecord>,
    unresolved: Vec<ExternalContractScopeAsset>,
    metadata: Vec<ExternalContractMetadata>,
    warnings: Vec<String>,
}

#[derive(Debug, Clone)]
struct DeploymentScopePageCandidate {
    url: String,
    label: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ImmunefiPocRuntime {
    project: String,
    slug: String,
    allow_fork: bool,
    prefer_fork: bool,
    assets: Vec<ImmunefiPocRuntimeAsset>,
    networks: Vec<ImmunefiPocRuntimeNetwork>,
}

#[derive(Debug, Clone, Serialize)]
struct ImmunefiPocRuntimeAsset {
    description: Option<String>,
    asset_type: Option<String>,
    address: String,
    explorer_url: String,
    explorer_host: String,
    network: String,
    network_kind: String,
    rpc_env_var: String,
    rpc_env_available: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ImmunefiPocRuntimeNetwork {
    network: String,
    network_kind: String,
    rpc_env_var: String,
    rpc_env_available: bool,
    asset_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExplorerNetwork {
    id: &'static str,
    kind: &'static str,
}

#[derive(Debug, Clone)]
struct SolidityDefinition {
    name: String,
    path: String,
    file_stem: String,
    normalized_name: String,
    normalized_file_stem: String,
    tokens: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CodexExternalScopeResolution {
    resolutions: Vec<CodexExternalScopeResolvedAsset>,
    unresolved: Vec<CodexExternalScopeUnresolvedAsset>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CodexExternalScopeResolvedAsset {
    label: String,
    address: String,
    paths: Vec<String>,
    confidence: String,
    reason: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CodexExternalScopeUnresolvedAsset {
    label: String,
    address: String,
    reason: String,
}

impl BountyPlatform {
    // Platform-specific files share the same lifecycle: write raw source JSON,
    // markdown rules, and machine-readable severity data before LLM context.
    fn artifact_paths(self, output_dir: &Path, artifact_prefix: &str) -> BountyArtifactPaths {
        match self {
            BountyPlatform::Code4rena => BountyArtifactPaths {
                platform: self,
                source_json: output_dir
                    .join(format!("{artifact_prefix}-code4rena-bounty-source.json")),
                bounty_rules_md: output_dir
                    .join(format!("{artifact_prefix}-code4rena-bounty-rules.md")),
                severity_rubric_md: output_dir
                    .join(format!("{artifact_prefix}-code4rena-severity-rubric.md")),
                severity_rubric_json: output_dir
                    .join(format!("{artifact_prefix}-code4rena-severity-rubric.json")),
                poc_runtime_md: None,
                poc_runtime_json: None,
            },
            BountyPlatform::Immunefi => BountyArtifactPaths {
                platform: self,
                source_json: output_dir.join(format!("{artifact_prefix}-immunefi-source.json")),
                bounty_rules_md: output_dir
                    .join(format!("{artifact_prefix}-immunefi-bounty-rules.md")),
                severity_rubric_md: output_dir
                    .join(format!("{artifact_prefix}-immunefi-severity-rubric.md")),
                severity_rubric_json: output_dir
                    .join(format!("{artifact_prefix}-immunefi-severity-rubric.json")),
                poc_runtime_md: Some(
                    output_dir.join(format!("{artifact_prefix}-immunefi-poc-runtime.md")),
                ),
                poc_runtime_json: Some(
                    output_dir.join(format!("{artifact_prefix}-immunefi-poc-runtime.json")),
                ),
            },
        }
    }
}

fn scope_bounty_platform(cli: &Cli) -> Option<BountyPlatform> {
    // Scope policy is keyed by audit type, even when a platform URL is missing,
    // because fallback behavior must still fail closed for bounty modes.
    match cli.audit_type {
        AuditType::Code4renaBounty => Some(BountyPlatform::Code4rena),
        AuditType::ImmunefiBugBounty => Some(BountyPlatform::Immunefi),
        _ => None,
    }
}

impl BountyArtifactPaths {
    fn for_cli(cli: &Cli, output_dir: &Path, artifact_prefix: &str) -> Option<Self> {
        // Only create C4 bounty artifacts when there is an actual configured
        // bounty page. Immunefi bounty mode always requires a live bounty URL.
        if matches!(cli.audit_type, AuditType::Code4renaBounty) && cli.code4rena_bounty.is_some() {
            return Some(BountyPlatform::Code4rena.artifact_paths(output_dir, artifact_prefix));
        }
        if matches!(cli.audit_type, AuditType::ImmunefiBugBounty) {
            return Some(BountyPlatform::Immunefi.artifact_paths(output_dir, artifact_prefix));
        }
        None
    }

    fn extra_docs(&self) -> Vec<PathBuf> {
        let mut docs = vec![
            self.bounty_rules_md.clone(),
            self.severity_rubric_md.clone(),
        ];
        if let Some(poc_runtime_md) = &self.poc_runtime_md {
            docs.push(poc_runtime_md.clone());
        }
        docs
    }
}

impl FetchedBounty {
    fn code4rena(&self) -> Option<&Code4renaBountyData> {
        match self {
            FetchedBounty::Code4rena(bounty) => Some(bounty),
            FetchedBounty::Immunefi(_) => None,
        }
    }

    fn immunefi(&self) -> Option<&ImmunefiBountyData> {
        match self {
            FetchedBounty::Code4rena(_) => None,
            FetchedBounty::Immunefi(bounty) => Some(bounty),
        }
    }
}

/// Generates all audit-context artifacts for a prepared repository workspace.
///
/// `@notice` This is the top-level context-generation entry point called after
/// clone/build and before static analysis.
/// `@dev` The function is deliberately orchestration-heavy: it decides whether
/// cached artifacts can be reused, fetches bounty-specific metadata when
/// required, collects source material, writes `scope.txt`, then asks the context
/// agent to create `scope.md` and `docs.md`.
/// `@custom:fail-closed` Bounty modes may abort when structured scope exists
/// but cannot be mapped precisely to local Solidity files.
pub async fn generate_audit_context(
    cli: &Cli,
    workspace_root: &Path,
    protocol_root: &Path,
    repo_name: &str,
    commit_hash: &str,
) -> Result<GeneratedAuditContext> {
    let context_config = cli.context.clone().unwrap_or_default();
    let artifact_prefix = artifact_prefix_from_repo_name(repo_name);
    let output_dir = context_output_dir(&context_config.output_dir)?.join(&artifact_prefix);
    fs::create_dir_all(&output_dir).with_context(|| {
        format!(
            "Failed to create generated audit context directory {}",
            output_dir.display()
        )
    })?;

    let scope_txt = output_dir.join(format!("{artifact_prefix}-scope.txt"));
    let scope_md = output_dir.join(format!("{artifact_prefix}-scope.md"));
    let docs_md = output_dir.join(format!("{artifact_prefix}-docs.md"));
    let validation_md = output_dir.join(format!("{artifact_prefix}-validation.md"));
    let sources_json = output_dir.join(format!("{artifact_prefix}-context-sources.json"));
    // Bounty modes emit platform-specific rule/rubric files alongside the
    // generic context artifacts. Keep path construction centralized so C4 and
    // Immunefi do not drift as the artifact set evolves.
    let bounty_artifacts = BountyArtifactPaths::for_cli(cli, &output_dir, &artifact_prefix);
    let extra_docs = bounty_artifacts
        .as_ref()
        .map(BountyArtifactPaths::extra_docs)
        .unwrap_or_default();

    let mut report = ContextSourceReport {
        artifact_prefix: artifact_prefix.clone(),
        commit_hash: commit_hash.to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        sources: Vec::new(),
        link_decisions: Vec::new(),
        warnings: Vec::new(),
    };

    let all_outputs_exist = scope_txt.exists()
        && scope_md.exists()
        && docs_md.exists()
        && validation_md.exists()
        && extra_docs.iter().all(|path| path.exists());
    if all_outputs_exist && !context_config.force_regenerate {
        // Cached artifacts are safe to reuse only when every mode-specific
        // markdown artifact exists. Raw JSON sidecars are diagnostic, not inputs.
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
            validation_md,
            sources_json,
            extra_docs,
            regenerated: false,
            source_report: report,
        });
    }

    // Fetch once, render all platform artifacts, then pass borrowed bounty data
    // into generic source collection and scope resolution.
    let fetched_bounty = if let Some(artifacts) = &bounty_artifacts {
        Some(fetch_and_write_bounty_artifacts(cli, artifacts).await?)
    } else {
        None
    };
    let code4rena_bounty = fetched_bounty.as_ref().and_then(FetchedBounty::code4rena);
    let immunefi_bounty = fetched_bounty.as_ref().and_then(FetchedBounty::immunefi);

    info!(
        "Generating audit context artifacts for {} from {}",
        artifact_prefix,
        protocol_root.display()
    );

    let (mut sources, link_decisions) = collect_context_sources(
        &context_config,
        workspace_root,
        protocol_root,
        repo_name,
        cli,
        code4rena_bounty,
        immunefi_bounty,
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

    let scope_file_list = generate_scope_txt(
        cli,
        protocol_root,
        &scope_txt,
        &sources,
        code4rena_bounty,
        immunefi_bounty,
    )
    .await?;
    report.warnings.extend(scope_file_list.warnings.clone());

    let scoped_code_sources =
        collect_scoped_code_documentation_sources(protocol_root, &scope_file_list);
    for source in scoped_code_sources {
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
        sources.push(source);
    }

    let agent = build_context_agent(cli)?;
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

    let validation_markdown =
        render_validation_sidecar(&artifact_prefix, &scope_file_list, &sources);
    fs::write(&validation_md, &validation_markdown)
        .with_context(|| format!("Failed to write {}", validation_md.display()))?;
    info!(
        "Wrote generated validation sidecar: path={}, tokens={}",
        validation_md.display(),
        get_token_count(&validation_markdown)
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
        validation_md,
        sources_json,
        extra_docs,
        regenerated: true,
        source_report: report,
    })
}

async fn fetch_and_write_bounty_artifacts(
    cli: &Cli,
    artifacts: &BountyArtifactPaths,
) -> Result<FetchedBounty> {
    match artifacts.platform {
        BountyPlatform::Code4rena => {
            // Code4rena bounty pages provide smart-contract scope, custom
            // severity rules, and program-specific exclusions in one page.
            let bounty_url = cli
                .code4rena_bounty
                .as_deref()
                .context("code4rena_bounty is required for Code4renaBounty context generation")?;
            let bounty = fetch_code4rena_bounty(bounty_url).await.with_context(|| {
                format!("Failed to fetch Code4rena bounty metadata from {bounty_url}")
            })?;
            fs::write(
                &artifacts.source_json,
                serde_json::to_string_pretty(&bounty)?,
            )
            .with_context(|| format!("Failed to write {}", artifacts.source_json.display()))?;
            fs::write(
                &artifacts.bounty_rules_md,
                render_code4rena_bounty_rules_markdown(&bounty),
            )
            .with_context(|| format!("Failed to write {}", artifacts.bounty_rules_md.display()))?;
            fs::write(
                &artifacts.severity_rubric_md,
                render_code4rena_severity_rubric_markdown(&bounty),
            )
            .with_context(|| {
                format!("Failed to write {}", artifacts.severity_rubric_md.display())
            })?;
            fs::write(
                &artifacts.severity_rubric_json,
                render_code4rena_severity_rubric_json(&bounty)?,
            )
            .with_context(|| {
                format!(
                    "Failed to write {}",
                    artifacts.severity_rubric_json.display()
                )
            })?;
            info!(
                "Wrote Code4rena bounty rules and severity rubric: rules={}, rubric={}",
                artifacts.bounty_rules_md.display(),
                artifacts.severity_rubric_md.display()
            );
            Ok(FetchedBounty::Code4rena(bounty))
        }
        BountyPlatform::Immunefi => {
            // Immunefi has separate tabs plus PoC runtime constraints. Keep
            // Smart Contract data, fork preferences, and severity rows together.
            let bounty_url = cli
                .immunefi_bounty
                .as_deref()
                .context("immunefi_bounty is required for AuditType::ImmunefiBugBounty")?;
            let bounty = fetch_immunefi_bounty(bounty_url).await.with_context(|| {
                format!("Failed to fetch Immunefi bounty metadata from {bounty_url}")
            })?;
            fs::write(
                &artifacts.source_json,
                serde_json::to_string_pretty(&bounty)?,
            )
            .with_context(|| format!("Failed to write {}", artifacts.source_json.display()))?;
            info!(
                "Wrote Immunefi bounty source report: path={}",
                artifacts.source_json.display()
            );
            let poc_runtime = build_immunefi_poc_runtime(&bounty, &cli.poc);
            fs::write(
                &artifacts.bounty_rules_md,
                render_immunefi_bounty_rules_markdown(&bounty),
            )
            .with_context(|| format!("Failed to write {}", artifacts.bounty_rules_md.display()))?;
            fs::write(
                &artifacts.severity_rubric_md,
                render_immunefi_severity_rubric_markdown(&bounty),
            )
            .with_context(|| {
                format!("Failed to write {}", artifacts.severity_rubric_md.display())
            })?;
            fs::write(
                &artifacts.severity_rubric_json,
                render_immunefi_severity_rubric_json(&bounty)?,
            )
            .with_context(|| {
                format!(
                    "Failed to write {}",
                    artifacts.severity_rubric_json.display()
                )
            })?;
            let poc_runtime_md = artifacts
                .poc_runtime_md
                .as_ref()
                .context("internal error: Immunefi PoC runtime markdown path missing")?;
            let poc_runtime_json = artifacts
                .poc_runtime_json
                .as_ref()
                .context("internal error: Immunefi PoC runtime JSON path missing")?;
            fs::write(
                poc_runtime_md,
                render_immunefi_poc_runtime_markdown(&poc_runtime),
            )
            .with_context(|| format!("Failed to write {}", poc_runtime_md.display()))?;
            fs::write(
                poc_runtime_json,
                serde_json::to_string_pretty(&poc_runtime)?,
            )
            .with_context(|| format!("Failed to write {}", poc_runtime_json.display()))?;
            info!(
                "Wrote Immunefi bounty rules, severity rubric, and PoC runtime: rules={}, rubric={}, runtime={}",
                artifacts.bounty_rules_md.display(),
                artifacts.severity_rubric_md.display(),
                poc_runtime_md.display()
            );
            Ok(FetchedBounty::Immunefi(bounty))
        }
    }
}

pub fn should_generate_context(cli: &Cli) -> bool {
    // Bounty modes always regenerate/consume dynamic context because the bounty
    // page is the source of truth for rules, severity, scope, and runtime hints.
    if matches!(cli.audit_type, AuditType::ImmunefiBugBounty) {
        return true;
    }
    if matches!(cli.audit_type, AuditType::Code4renaBounty) && cli.code4rena_bounty.is_some() {
        return true;
    }

    let has_complete_manual_context =
        cli.custom_doc.is_some() && cli.audit_scope.is_some() && cli.scoped_files.is_some();

    match &cli.context {
        Some(config) => config.force_regenerate || !has_complete_manual_context,
        None => !has_complete_manual_context,
    }
}

const GENERIC_CONTEXT_PREAMBLE: &str = "You generate concise, security-review-ready audit context for Solidity protocol audits. Preserve scope, known issues, prior findings, invariants, trusted roles, and protocol mechanics. Do not invent facts. If source material is uncertain, say so briefly.";

const CODE4RENA_COMPETITION_CONTEXT_PREAMBLE: &str = "You generate concise, security-review-ready audit context for Code4rena Solidity competitions. Preserve the README scope, sponsor-provided docs, known issues, prior findings, V12/prior report links, invariants, trusted roles, and protocol mechanics. V12 and prior-finding context is Code4rena competition-only. Do not invent facts. If source material is uncertain, say so briefly.";

const CODE4RENA_BOUNTY_CONTEXT_PREAMBLE: &str = "You generate concise, security-review-ready audit context for Code4rena smart-contract bug bounties. Treat the configured Code4rena bounty page as the source of truth for smart contract scope, custom severity criteria, custom exclusions, known issues, previous audits, and repo links. Include default Code4rena bounty criteria, but let program-specific rules override or narrow the defaults. There is no V12 stage for Code4rena bounties. Do not invent facts. If source material is uncertain, say so briefly.";

const IMMUNEFI_BOUNTY_CONTEXT_PREAMBLE: &str = "You generate concise, security-review-ready audit context for Immunefi smart-contract bug bounties. Use only Smart Contract category assets, impacts, rewards, repositories, and scope. Ignore Web & App assets, impacts, resources, and findings except to note that they were intentionally excluded. Preserve program impacts, rewards, out-of-scope rules, prohibited activities, PoC requirements, primacy rules, known issues, prior audits, trusted roles, invariants, and protocol mechanics. Do not invent facts. If source material is uncertain, say so briefly.";

fn context_generation_preamble(cli: &Cli) -> &'static str {
    match cli.audit_type {
        AuditType::Code4rena => CODE4RENA_COMPETITION_CONTEXT_PREAMBLE,
        AuditType::Code4renaBounty => CODE4RENA_BOUNTY_CONTEXT_PREAMBLE,
        AuditType::ImmunefiBugBounty => IMMUNEFI_BOUNTY_CONTEXT_PREAMBLE,
        _ => GENERIC_CONTEXT_PREAMBLE,
    }
}

fn build_context_agent(cli: &Cli) -> Result<AIAgent> {
    AgentFactory::create_openai_agent(
        &AgentConfig::new(None)
            .with_model(OPENAI_MODEL)
            .with_openai_reasoning_effort(OPENAI_REASONING_EFFORT)
            .with_preamble(context_generation_preamble(cli)),
    )
    .map_err(anyhow::Error::from)
}

async fn collect_context_sources(
    config: &ContextConfig,
    workspace_root: &Path,
    protocol_root: &Path,
    repo_name: &str,
    cli: &Cli,
    code4rena_bounty: Option<&Code4renaBountyData>,
    immunefi_bounty: Option<&ImmunefiBountyData>,
) -> Result<(Vec<SourceContent>, Vec<LinkDecision>)> {
    // Source collection is intentionally two-phase. First we add high-trust
    // local and structured bounty sources. Then we extract/fetch only relevant
    // links under budget. This keeps generated context grounded and prevents
    // workers from chasing navigation or marketing pages.
    let mut sources = Vec::new();
    let mut link_decisions = Vec::new();
    let mut seen_locations = HashSet::new();
    let audit_type = &cli.audit_type;
    let code4rena_competition = matches!(audit_type, AuditType::Code4rena);
    let code4rena_bounty_audit = matches!(audit_type, AuditType::Code4renaBounty);
    let immunefi_bug_bounty = matches!(audit_type, AuditType::ImmunefiBugBounty);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(HTTP_TIMEOUT_SECS))
        .user_agent("ai-agent-audit-context-generator/0.1")
        .build()?;

    // Entry sources are the high-trust roots. Immunefi uses captured tabs;
    // other modes start from configured local files, usually README.md.
    if immunefi_bug_bounty {
        let bounty = immunefi_bounty
            .context("internal error: missing fetched Immunefi bounty data for context sources")?;
        push_immunefi_bounty_sources(&mut sources, &mut seen_locations, bounty)?;
    } else {
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
    }

    if code4rena_competition {
        let discovered_docs = discover_local_protocol_doc_files(protocol_root);
        debug!(
            "Codex context discovery: found {} local protocol doc candidates",
            discovered_docs.len()
        );
        for path in discovered_docs {
            push_local_source(
                &mut sources,
                &mut seen_locations,
                ContextSourceKind::LocalProtocolDocs,
                &path,
                SourceDecision::UsedForDocs,
                "Auto-discovered local protocol documentation",
            )?;
        }

        push_code4rena_contest_repo_sources(
            &client,
            cli,
            protocol_root,
            &mut sources,
            &mut seen_locations,
        )
        .await?;
    }

    if let Some(bounty) = code4rena_bounty {
        push_code4rena_bounty_source(&mut sources, &mut seen_locations, bounty)?;
    }

    // Local known-issue and audit notes are scope context, but not traversal
    // roots. This avoids recursively chasing old report/navigation links.
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
        // A repo-provided scope.txt is the strongest machine-readable scope
        // signal, so preserve it as source material for provenance as well.
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
    // Second-level traversal is intentionally shallow and budgeted. Entry
    // sources can point us to docs/scope/rules; linked pages do not fan out.
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
        // User-configured URLs bypass entry-source extraction but still pass
        // through the same classification, skip, and fetch-budget rules.
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
    if code4rena_bounty_audit {
        // C4 bounty defaults are mandatory context unless the bounty page
        // narrows them, so they are fetched outside the normal remote budget.
        links.push(ExtractedLink {
            from: "code4rena-bounty-defaults".to_string(),
            url: CODE4RENA_BOUNTY_GUIDE_URL.to_string(),
            label: "Code4rena bounty guide".to_string(),
        });
        links.push(ExtractedLink {
            from: "code4rena-bounty-defaults".to_string(),
            url: CODE4RENA_BOUNTY_CRITERIA_URL.to_string(),
            label: "Code4rena bounty severity and out-of-scope criteria".to_string(),
        });
    }
    let source_v12_link_count = links
        .iter()
        .filter(|link| classify_link(&link.url, &link.label) == LinkClassification::V12)
        .count();
    // V12/prior finding reports are competition-only. Bounty audits must not
    // inherit competition-stage assumptions or old contest submission rules.
    let v12_links = if !code4rena_competition {
        debug!(
            "Codex context discovery: skipping V12 lookup because audit_type={:?}; V12 context is Code4rena competition-only",
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
        // Every link gets a report entry so skipped/fetched decisions are
        // auditable in `<protocol>-context-sources.json`.
        let classification = classify_link(&link.url, &link.label);
        debug!(
            "Codex context discovery: checking link from={}, classification={:?}, url={}",
            link.from, classification, link.url
        );
        if classification == LinkClassification::V12 && !code4rena_competition {
            let reason = format!(
                "Skipped V12 link because audit_type={audit_type:?}; V12 is Code4rena competition-only"
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
        let counts_against_remote_budget = remote_link_counts_against_fetch_budget(&classification);
        if remote_link && counts_against_remote_budget {
            // Remote pages are the noisiest inputs. Keep total fetches small,
            // and keep prior audits even tighter because they are often long.
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
        if remote_link && counts_against_remote_budget {
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

async fn push_code4rena_contest_repo_sources(
    client: &reqwest::Client,
    cli: &Cli,
    protocol_root: &Path,
    sources: &mut Vec<SourceContent>,
    seen_locations: &mut HashSet<String>,
) -> Result<()> {
    let Some(contest_repo) = code4rena_contest_context_repo_url(cli) else {
        return Ok(());
    };

    let explicit_contest_context =
        cli.code4rena_contest_repo.is_some() || cli.code4rena_contest_url.is_some();
    let same_as_analyzed_repo = cli
        .repo
        .as_deref()
        .and_then(normalize_github_repo_url)
        .map(|repo| repo.eq_ignore_ascii_case(&contest_repo))
        .unwrap_or(false);
    if !explicit_contest_context
        && same_as_analyzed_repo
        && protocol_root.join("README.md").exists()
    {
        debug!(
            "Codex context discovery: using cloned Code4rena contest repo files instead of refetching {}",
            contest_repo
        );
        return Ok(());
    }

    let mut refs = Vec::new();
    if same_as_analyzed_repo && let Some(branch) = &cli.repo_branch {
        refs.push(branch.clone());
    }
    refs.push("main".to_string());
    refs.push("master".to_string());
    refs.sort();
    refs.dedup();

    let candidates = [
        (
            "README.md",
            SourceDecision::UsedForBoth,
            "Code4rena contest README from contest repository",
        ),
        (
            "scope.txt",
            SourceDecision::UsedForScope,
            "Code4rena contest machine-readable scope from contest repository",
        ),
        (
            "out_of_scope.txt",
            SourceDecision::UsedForScope,
            "Code4rena contest out-of-scope notes from contest repository",
        ),
        (
            "out-of-scope.txt",
            SourceDecision::UsedForScope,
            "Code4rena contest out-of-scope notes from contest repository",
        ),
    ];

    for (path, decision, reason) in candidates {
        let mut fetched = None;
        for repo_ref in &refs {
            if let Some(raw_url) = github_repo_raw_file_url(&contest_repo, repo_ref, path)
                && let Some(content) = fetch_optional_text(client, &raw_url).await?
            {
                fetched = Some((raw_url, content));
                break;
            }
        }
        let Some((location, content)) = fetched else {
            continue;
        };
        if !seen_locations.insert(location.clone()) {
            continue;
        }
        log_source_content_loaded(&location, &content);
        sources.push(SourceContent {
            id: format!("source-{}", sources.len() + 1),
            kind: ContextSourceKind::Code4renaContestRepo,
            location,
            title: Some(path.to_string()),
            content,
            decision,
            reason: reason.to_string(),
        });
    }

    Ok(())
}

async fn fetch_optional_text(client: &reqwest::Client, url: &str) -> Result<Option<String>> {
    let response = client.get(url).send().await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        return Err(anyhow!(
            "Fetch failed with status {} for {}",
            response.status(),
            url
        ));
    }
    Ok(Some(response.text().await?))
}

fn code4rena_contest_context_repo_url(cli: &Cli) -> Option<String> {
    cli.code4rena_contest_repo
        .as_deref()
        .or(cli.code4rena_contest_url.as_deref())
        .or(cli.repo.as_deref())
        .and_then(code4rena_contest_url_to_github_repo)
}

fn code4rena_contest_url_to_github_repo(url: &str) -> Option<String> {
    if let Some(github_url) = normalize_github_repo_url(url) {
        return Some(github_url);
    }

    let clean = url.trim().trim_end_matches('/');
    let lower = clean.to_ascii_lowercase();
    if !lower.contains("code4rena.com/audits/") {
        return None;
    }
    let slug = clean
        .split("/audits/")
        .nth(1)?
        .split('/')
        .next()?
        .split('?')
        .next()?
        .split('#')
        .next()?;
    if slug.is_empty() {
        None
    } else {
        Some(format!("https://github.com/code-423n4/{slug}"))
    }
}

fn normalize_github_repo_url(url: &str) -> Option<String> {
    let clean = url
        .trim()
        .split('#')
        .next()
        .unwrap_or(url)
        .split('?')
        .next()
        .unwrap_or(url)
        .trim_end_matches('/')
        .trim_end_matches(".git");
    let after_host = clean
        .split("github.com/")
        .nth(1)
        .or_else(|| clean.split("www.github.com/").nth(1))?;
    let parts = after_host.split('/').collect::<Vec<_>>();
    if parts.len() < 2 || parts[0].is_empty() || parts[1].is_empty() {
        return None;
    }
    Some(format!("https://github.com/{}/{}", parts[0], parts[1]))
}

fn github_repo_raw_file_url(repo_url: &str, repo_ref: &str, path: &str) -> Option<String> {
    let normalized = normalize_github_repo_url(repo_url)?;
    let after_host = normalized.split("github.com/").nth(1)?;
    let (owner, repo) = after_host.split_once('/')?;
    Some(format!(
        "https://raw.githubusercontent.com/{owner}/{repo}/{repo_ref}/{}",
        path.trim_start_matches('/')
    ))
}

fn push_immunefi_bounty_sources(
    sources: &mut Vec<SourceContent>,
    seen_locations: &mut HashSet<String>,
    bounty: &ImmunefiBountyData,
) -> Result<()> {
    for tab in &bounty.tabs {
        let content = bounty.context_markdown_for_tab(tab.kind, &html_to_text(&tab.html));
        let (kind, decision, title) = match tab.kind {
            ImmunefiTabKind::Information => (
                ContextSourceKind::ImmunefiInformation,
                SourceDecision::UsedForBoth,
                "Immunefi Information",
            ),
            ImmunefiTabKind::Scope => (
                ContextSourceKind::ImmunefiScope,
                SourceDecision::UsedForScope,
                "Immunefi Scope",
            ),
            ImmunefiTabKind::Resources => (
                ContextSourceKind::ImmunefiResources,
                SourceDecision::UsedForBoth,
                "Immunefi Resources",
            ),
        };
        if !seen_locations.insert(tab.url.clone()) {
            continue;
        }
        log_source_content_loaded(&tab.url, &content);
        sources.push(SourceContent {
            id: format!("source-{}", sources.len() + 1),
            kind,
            location: tab.url.clone(),
            title: Some(title.to_string()),
            content,
            decision,
            reason: IMMUNEFI_BOUNTY_ENTRY_REASON.to_string(),
        });
    }
    Ok(())
}

fn push_code4rena_bounty_source(
    sources: &mut Vec<SourceContent>,
    seen_locations: &mut HashSet<String>,
    bounty: &Code4renaBountyData,
) -> Result<()> {
    if !seen_locations.insert(bounty.url.clone()) {
        return Ok(());
    }
    let content = bounty.context_markdown();
    log_source_content_loaded(&bounty.url, &content);
    sources.push(SourceContent {
        id: format!("source-{}", sources.len() + 1),
        kind: ContextSourceKind::Code4renaBountyPage,
        location: bounty.url.clone(),
        title: Some("Code4rena Bounty Page".to_string()),
        content,
        decision: SourceDecision::UsedForBoth,
        reason: CODE4RENA_BOUNTY_ENTRY_REASON.to_string(),
    });
    Ok(())
}

fn build_immunefi_poc_runtime(
    bounty: &ImmunefiBountyData,
    poc_config: &PocConfig,
) -> ImmunefiPocRuntime {
    let mut assets = Vec::new();
    for asset in bounty.smart_contract_assets(false) {
        if asset.is_primacy_of_impact {
            continue;
        }
        let Some(address) = extract_evm_address(&asset.url) else {
            continue;
        };
        let Some(network) = infer_explorer_network(&asset.url) else {
            continue;
        };
        let Some(rpc_env_var) = poc_config.rpc_env_var_for(network.id) else {
            continue;
        };
        assets.push(ImmunefiPocRuntimeAsset {
            description: asset.description.clone(),
            asset_type: asset.asset_type.clone(),
            address,
            explorer_host: explorer_host_text(&asset.url),
            explorer_url: asset.url.clone(),
            network: network.id.to_string(),
            network_kind: network.kind.to_string(),
            rpc_env_available: std::env::var_os(&rpc_env_var).is_some(),
            rpc_env_var,
        });
    }
    assets.sort_by(|left, right| {
        left.network
            .cmp(&right.network)
            .then(left.address.cmp(&right.address))
    });
    assets.dedup_by(|left, right| {
        left.network == right.network && left.address.eq_ignore_ascii_case(&right.address)
    });

    let mut network_counts = BTreeMap::<String, (String, String, bool, usize)>::new();
    for asset in &assets {
        let entry = network_counts
            .entry(asset.network.clone())
            .or_insert_with(|| {
                (
                    asset.network_kind.clone(),
                    asset.rpc_env_var.clone(),
                    asset.rpc_env_available,
                    0,
                )
            });
        entry.2 |= asset.rpc_env_available;
        entry.3 += 1;
    }
    let networks = network_counts
        .into_iter()
        .map(
            |(network, (network_kind, rpc_env_var, rpc_env_available, asset_count))| {
                ImmunefiPocRuntimeNetwork {
                    network,
                    network_kind,
                    rpc_env_var,
                    rpc_env_available,
                    asset_count,
                }
            },
        )
        .collect();

    ImmunefiPocRuntime {
        project: bounty.project.clone(),
        slug: bounty.slug.clone(),
        allow_fork: poc_config.allow_fork,
        prefer_fork: poc_config.prefer_fork,
        assets,
        networks,
    }
}

fn render_immunefi_poc_runtime_markdown(runtime: &ImmunefiPocRuntime) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Immunefi PoC Runtime - {}\n\n", runtime.project));
    out.push_str("Use this file as mandatory context for Immunefi R5 PoC generation and R6 PoC verification.\n\n");
    out.push_str("## Fork Preference\n\n");
    out.push_str(&format!("- Fork PoCs allowed: `{}`\n", runtime.allow_fork));
    out.push_str(&format!(
        "- Fork PoCs preferred: `{}`\n",
        runtime.prefer_fork
    ));
    out.push_str("- Prefer a mainnet fork PoC whenever the finding touches deployed in-scope mainnet contracts and a matching RPC env var is available.\n");
    out.push_str("- Use a public-testnet fork only when the in-scope asset itself is a public-testnet deployment, or when no matching mainnet deployment exists but a relevant in-scope public-testnet deployment does.\n");
    out.push_str("- Do not use a local non-fork test as the primary proof for an Immunefi deployed-asset finding.\n");
    out.push_str("- Never invent RPC URLs, deployed addresses, networks, or block numbers.\n\n");

    out.push_str("## In-Scope Deployed Contracts\n\n");
    if runtime.assets.is_empty() {
        out.push_str("- No EVM deployed contract addresses with recognized explorer networks were extracted from the Immunefi Scope tab.\n\n");
    } else {
        out.push_str(
            "| Description | Type | Address | Network | RPC Env Var | Env Available | Explorer |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for asset in &runtime.assets {
            out.push_str(&format!(
                "| {} | {} | `{}` | `{}` | `{}` | `{}` | {} |\n",
                markdown_table_cell(asset.description.as_deref().unwrap_or("-")),
                markdown_table_cell(asset.asset_type.as_deref().unwrap_or("-")),
                asset.address,
                asset.network,
                asset.rpc_env_var,
                asset.rpc_env_available,
                markdown_table_cell(&asset.explorer_url)
            ));
        }
        out.push('\n');
    }

    out.push_str("## Network RPC Availability\n\n");
    if runtime.networks.is_empty() {
        out.push_str("- No recognized EVM networks were extracted.\n\n");
    } else {
        out.push_str("| Network | Kind | RPC Env Var | Env Available | Asset Count |\n");
        out.push_str("| --- | --- | --- | --- | --- |\n");
        for network in &runtime.networks {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                network.network,
                network.network_kind,
                network.rpc_env_var,
                network.rpc_env_available,
                network.asset_count
            ));
        }
        out.push('\n');
    }

    out.push_str("## Foundry Command Templates\n\n");
    out.push_str("Use env vars, not raw URLs:\n\n");
    out.push_str("```bash\n");
    out.push_str("set -a; source \"<AI_AGENT_AUDIT_ROOT>/.env\"; set +a; forge test --match-test <testName> --fork-url \"$MAINNET_RPC_URL\"\n");
    out.push_str("set -a; source \"<AI_AGENT_AUDIT_ROOT>/.env\"; set +a; forge test --match-path test/<PoCFile>.t.sol --fork-url \"$ARBITRUM_RPC_URL\"\n");
    out.push_str("```\n\n");

    out.push_str("## Safety Rules\n\n");
    out.push_str("- Fork PoCs must be local simulations only.\n");
    out.push_str("- Do not broadcast transactions.\n");
    out.push_str("- Do not use live private keys or live privileged accounts.\n");
    out.push_str("- Do not mutate live mainnet or public-testnet protocol state.\n");
    out.push_str(
        "- Do not steal, freeze, transfer, or manipulate real assets, even tiny amounts.\n",
    );
    out
}

fn markdown_table_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn extract_evm_address(value: &str) -> Option<String> {
    Regex::new(r"(?i)0x[a-f0-9]{40}")
        .unwrap()
        .find(value)
        .map(|m| m.as_str().to_string())
}

fn explorer_host_text(url: &str) -> String {
    let without_scheme = url.split("://").nth(1).unwrap_or(url);
    without_scheme
        .split('/')
        .next()
        .unwrap_or_default()
        .trim_start_matches("www.")
        .to_ascii_lowercase()
}

fn infer_explorer_network(url: &str) -> Option<ExplorerNetwork> {
    let host = explorer_host_text(url);
    match host.as_str() {
        "etherscan.io" => Some(ExplorerNetwork {
            id: "ethereum-mainnet",
            kind: "mainnet",
        }),
        "sepolia.etherscan.io" => Some(ExplorerNetwork {
            id: "ethereum-sepolia",
            kind: "testnet",
        }),
        "arbiscan.io" => Some(ExplorerNetwork {
            id: "arbitrum-mainnet",
            kind: "mainnet",
        }),
        "sepolia.arbiscan.io" => Some(ExplorerNetwork {
            id: "arbitrum-sepolia",
            kind: "testnet",
        }),
        "optimistic.etherscan.io" => Some(ExplorerNetwork {
            id: "optimism-mainnet",
            kind: "mainnet",
        }),
        "sepolia-optimism.etherscan.io" => Some(ExplorerNetwork {
            id: "optimism-sepolia",
            kind: "testnet",
        }),
        "basescan.org" => Some(ExplorerNetwork {
            id: "base-mainnet",
            kind: "mainnet",
        }),
        "sepolia.basescan.org" => Some(ExplorerNetwork {
            id: "base-sepolia",
            kind: "testnet",
        }),
        "polygonscan.com" => Some(ExplorerNetwork {
            id: "polygon-mainnet",
            kind: "mainnet",
        }),
        "amoy.polygonscan.com" => Some(ExplorerNetwork {
            id: "polygon-amoy",
            kind: "testnet",
        }),
        "era.zksync.network" | "explorer.zksync.io" | "zksync-era.blockscout.com" => {
            Some(ExplorerNetwork {
                id: "zksync-mainnet",
                kind: "mainnet",
            })
        }
        "sepolia.explorer.zksync.io" | "sepolia-era.zksync.network" => Some(ExplorerNetwork {
            id: "zksync-sepolia",
            kind: "testnet",
        }),
        "bscscan.com" => Some(ExplorerNetwork {
            id: "bnb-mainnet",
            kind: "mainnet",
        }),
        "testnet.bscscan.com" => Some(ExplorerNetwork {
            id: "bnb-testnet",
            kind: "testnet",
        }),
        "snowtrace.io" => Some(ExplorerNetwork {
            id: "avalanche-mainnet",
            kind: "mainnet",
        }),
        "testnet.snowtrace.io" => Some(ExplorerNetwork {
            id: "avalanche-fuji",
            kind: "testnet",
        }),
        "lineascan.build" => Some(ExplorerNetwork {
            id: "linea-mainnet",
            kind: "mainnet",
        }),
        "sepolia.lineascan.build" => Some(ExplorerNetwork {
            id: "linea-sepolia",
            kind: "testnet",
        }),
        "scrollscan.com" => Some(ExplorerNetwork {
            id: "scroll-mainnet",
            kind: "mainnet",
        }),
        "sepolia.scrollscan.com" => Some(ExplorerNetwork {
            id: "scroll-sepolia",
            kind: "testnet",
        }),
        "mantlescan.xyz" => Some(ExplorerNetwork {
            id: "mantle-mainnet",
            kind: "mainnet",
        }),
        "sepolia.mantlescan.xyz" => Some(ExplorerNetwork {
            id: "mantle-sepolia",
            kind: "testnet",
        }),
        "blastscan.io" => Some(ExplorerNetwork {
            id: "blast-mainnet",
            kind: "mainnet",
        }),
        "sepolia.blastscan.io" => Some(ExplorerNetwork {
            id: "blast-sepolia",
            kind: "testnet",
        }),
        "gnosisscan.io" => Some(ExplorerNetwork {
            id: "gnosis-mainnet",
            kind: "mainnet",
        }),
        "gnosis-chiado.blockscout.com" | "blockscout.chiadochain.net" => Some(ExplorerNetwork {
            id: "gnosis-chiado",
            kind: "testnet",
        }),
        "celoscan.io" => Some(ExplorerNetwork {
            id: "celo-mainnet",
            kind: "mainnet",
        }),
        "alfajores.celoscan.io" => Some(ExplorerNetwork {
            id: "celo-alfajores",
            kind: "testnet",
        }),
        "uniscan.xyz" | "unichain.blockscout.com" => Some(ExplorerNetwork {
            id: "unichain-mainnet",
            kind: "mainnet",
        }),
        "sepolia.uniscan.xyz" | "unichain-sepolia.blockscout.com" => Some(ExplorerNetwork {
            id: "unichain-sepolia",
            kind: "testnet",
        }),
        "sonicscan.org" => Some(ExplorerNetwork {
            id: "sonic-mainnet",
            kind: "mainnet",
        }),
        "testnet.sonicscan.org" => Some(ExplorerNetwork {
            id: "sonic-testnet",
            kind: "testnet",
        }),
        "berascan.com" => Some(ExplorerNetwork {
            id: "berachain-mainnet",
            kind: "mainnet",
        }),
        "bepolia.beratrail.io" | "testnet.berascan.com" => Some(ExplorerNetwork {
            id: "berachain-bepolia",
            kind: "testnet",
        }),
        "hyperevmscan.io" | "hyperliquid.cloud.blockscout.com" => Some(ExplorerNetwork {
            id: "hyperliquid-mainnet",
            kind: "mainnet",
        }),
        "testnet.hyperevmscan.io" => Some(ExplorerNetwork {
            id: "hyperliquid-testnet",
            kind: "testnet",
        }),
        _ => None,
    }
}

fn render_immunefi_bounty_rules_markdown(bounty: &ImmunefiBountyData) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Immunefi Bounty Rules - {}\n\n", bounty.project));
    out.push_str(
        "Use this file as mandatory context for `validation_profile: immunefi-bounty`.\n\n",
    );
    out.push_str("## Source URLs\n\n");
    out.push_str(&format!("- Information: {}\n", bounty.urls.information));
    out.push_str(&format!("- Scope: {}\n", bounty.urls.scope));
    out.push_str(&format!("- Resources: {}\n\n", bounty.urls.resources));

    out.push_str("## Program Requirements\n\n");
    append_optional_bullet(
        &mut out,
        "Proof of Concept",
        bounty.proof_of_concept_type.as_deref(),
    );
    append_optional_bullet(&mut out, "Primacy", bounty.primacy.as_deref());
    append_optional_bullet(&mut out, "Rewards token", bounty.rewards_token.as_deref());
    append_optional_bullet(
        &mut out,
        "Rewards token network",
        bounty.rewards_token_network.as_deref(),
    );
    if let Some(max_bounty) = bounty.max_bounty {
        out.push_str(&format!("- Maximum bounty: ${max_bounty}\n"));
    }
    out.push('\n');

    out.push_str("## Assets In Scope\n\n");
    let smart_assets = bounty.smart_contract_assets(true);
    out.push_str(
        "> Smart Contract category only. Web & App assets are intentionally excluded.\n\n",
    );
    if smart_assets.is_empty() {
        out.push_str("- No structured Smart Contract assets were extracted. Read the Scope tab directly.\n\n");
    } else {
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
                out.push_str(&format!("  - Description: {description}\n"));
            }
        }
        out.push('\n');
    }

    append_immunefi_impacts_section(&mut out, &bounty.smart_contract_impacts());

    out.push_str("## Out Of Scope And Exclusions\n\n");
    append_optional_section(
        &mut out,
        "Smart Contract Out Of Scope",
        bounty.default_out_of_scope_smart_contract.as_deref(),
    );
    append_optional_section(
        &mut out,
        "General Out Of Scope",
        bounty.default_out_of_scope_general.as_deref(),
    );
    append_optional_section(
        &mut out,
        "Custom Out Of Scope",
        bounty.custom_out_of_scope.as_deref(),
    );
    append_optional_section(
        &mut out,
        "Prohibited Activities",
        bounty.prohibited_activities.as_deref(),
    );
    if !bounty.known_issues.is_empty() {
        out.push_str("### Known Issues\n\n");
        for issue in &bounty.known_issues {
            out.push_str(&format!("- {issue}\n"));
        }
        out.push('\n');
    }

    out.push_str("## Audit And Documentation Exclusions\n\n");
    if bounty.audits.is_empty() {
        out.push_str("- No structured audit links were extracted. Read Resources directly.\n\n");
    } else {
        for audit in &bounty.audits {
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

    out.push_str("## Stage 1 Eligibility Rules\n\n");
    out.push_str("- A finding must affect an in-scope asset, unless the exact category and severity are covered by the program's Primacy of Impact rules.\n");
    out.push_str("- A finding must produce an impact listed in the program's Impacts in Scope.\n");
    out.push_str("- Exclude known issues, prior audit findings, documented accepted risks, closed duplicate reports, and program-specific OOS cases.\n");
    out.push_str("- Exclude cases requiring privileged access, leaked credentials, social engineering, malicious or mistaken trusted roles, deployment mistakes, test/mock files, public disclosure, or third-party-only failures.\n");
    out.push_str("- Do not perform final exploitability or severity scoring in stage 1; keep only when there is no decisive eligibility blocker.\n\n");

    out.push_str("## Link Handling\n\n");
    out.push_str("- Do not follow Immunefi navigation, marketing, login, social, newsletter, or platform-help links during validation.\n");
    out.push_str("- Use only the Source URLs above, in-scope explorer links, codebase links, documentation links, and prior-audit links when live verification is necessary.\n\n");

    out
}

fn render_code4rena_bounty_rules_markdown(bounty: &Code4renaBountyData) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# Code4rena Bounty Rules - {}\n\n",
        bounty.project
    ));
    out.push_str(
        "Use this file as mandatory context for `validation_profile: code4rena-bounty`.\n\n",
    );
    out.push_str("## Source URLs\n\n");
    out.push_str(&format!("- Bounty page: {}\n", bounty.url));
    out.push_str(&format!(
        "- Default C4 bounty guide: {CODE4RENA_BOUNTY_GUIDE_URL}\n"
    ));
    out.push_str(&format!(
        "- Default C4 bounty criteria: {CODE4RENA_BOUNTY_CRITERIA_URL}\n\n"
    ));

    out.push_str("## Program Requirements\n\n");
    if let Some(max_bounty) = &bounty.max_bounty {
        out.push_str(&format!("- Maximum bounty: {max_bounty}\n"));
    }
    out.push_str("- This app audits Smart Contract bounty scope only. Ignore website/app/social/media scope unless explicitly tied to deployed smart-contract impact.\n");
    out.push_str("- Custom program-specific Code4rena bounty rules override or narrow the default Code4rena bounty criteria for this program.\n\n");

    out.push_str("## Smart Contract Assets In Scope\n\n");
    if bounty.scope_assets.is_empty() {
        out.push_str("- No structured smart-contract scope assets were extracted. Read the bounty page directly and fail safe if scope cannot be resolved precisely.\n\n");
    } else {
        for asset in &bounty.scope_assets {
            out.push_str(&format!("- {}: {}\n", asset.label, asset.url));
        }
        out.push('\n');
    }

    append_optional_section(
        &mut out,
        "Program-Specific Rules",
        bounty.rules_section.as_deref(),
    );
    append_optional_section(
        &mut out,
        "Program-Specific Out Of Scope",
        bounty.out_of_scope_section.as_deref(),
    );
    append_optional_section(
        &mut out,
        "Known Issues",
        bounty.known_issues_section.as_deref(),
    );
    append_optional_section(
        &mut out,
        "Previous Audits",
        bounty.previous_audits_section.as_deref(),
    );

    out.push_str("## Stage 1 Eligibility Rules\n\n");
    out.push_str("- A finding must affect an in-scope smart contract asset or code path that is reachable from in-scope smart contract assets.\n");
    out.push_str("- Exclude known issues, prior audit findings, documented accepted risks, closed duplicate reports, and program-specific OOS cases.\n");
    out.push_str("- Exclude cases requiring privileged access, leaked credentials, social engineering, malicious or mistaken trusted roles, deployment mistakes, test/mock files, public disclosure, or third-party-only failures.\n");
    out.push_str("- Exclude speculative future risks and findings that cannot be exploited by a non-privileged attacker under current deployed or in-scope code.\n");
    out.push_str("- Do not perform final exploitability or severity scoring in stage 1; keep only when there is no decisive eligibility blocker.\n\n");

    out.push_str("## Link Handling\n\n");
    out.push_str("- Do not follow Code4rena navigation, marketing, login, social, newsletter, or platform-help links during validation.\n");
    out.push_str("- Use only the bounty page, default C4 bounty criteria, in-scope explorer links, source-code links, documentation links, and prior-audit links when live verification is necessary.\n\n");

    out
}

fn render_code4rena_severity_rubric_markdown(bounty: &Code4renaBountyData) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# Code4rena Bounty Severity Rubric - {}\n\n",
        bounty.project
    ));
    out.push_str("Use this file as mandatory severity context for `validation_profile: code4rena-bounty`.\n\n");
    out.push_str("## Source URLs\n\n");
    out.push_str(&format!("- Bounty page: {}\n", bounty.url));
    out.push_str(&format!(
        "- Default C4 bounty criteria: {CODE4RENA_BOUNTY_CRITERIA_URL}\n\n"
    ));

    out.push_str("## Program-Specific Severity Source Of Truth\n\n");
    if !bounty.rewards.is_empty() {
        out.push_str("### Rewards\n\n");
        for reward in &bounty.rewards {
            out.push_str(&format!("- {}: {}\n", reward.severity, reward.payout));
        }
        out.push('\n');
    }
    append_optional_section(
        &mut out,
        "Custom Severity And Rules From Bounty Page",
        bounty.severity_section.as_deref(),
    );

    out.push_str("## Default Code4rena Bounty Severity Baseline\n\n");
    out.push_str(CODE4RENA_BOUNTY_SEVERITY_RUBRIC);
    out.push_str("\n\n");

    out.push_str("## Code4rena Bounty Severity Decision Rules\n\n");
    out.push_str("- Stage 3 must match the finding to the bounty page's custom severity criteria when present.\n");
    out.push_str("- If the bounty page has no custom severity rule for the finding class, apply the default Code4rena bounty criteria.\n");
    out.push_str("- Treat custom program exclusions, known issues, previous audit findings, and explicit payout restrictions as mandatory constraints.\n");
    out.push_str("- Require current exploitability by a non-privileged attacker without leaked credentials or malicious trusted-role behavior.\n");
    out.push_str("- Mark ambiguous, medium-only, best-practice-only, or weak-evidence findings as `Invalid` or `Needs Review`, not submission-ready.\n");
    out.push_str("- PoC policy is recorded for later PoC stages only; stage 3 should not require an already-created PoC.\n\n");

    out
}

fn render_code4rena_severity_rubric_json(bounty: &Code4renaBountyData) -> Result<String> {
    #[derive(Serialize)]
    struct Code4renaSeverityRubricJson<'a> {
        project: &'a str,
        bounty_url: &'a str,
        default_criteria_url: &'static str,
        rewards: &'a [crate::prepare_code::code4rena_bounty::Code4renaReward],
        custom_severity_section: Option<&'a str>,
        out_of_scope_section: Option<&'a str>,
        known_issues_section: Option<&'a str>,
    }

    serde_json::to_string_pretty(&Code4renaSeverityRubricJson {
        project: &bounty.project,
        bounty_url: &bounty.url,
        default_criteria_url: CODE4RENA_BOUNTY_CRITERIA_URL,
        rewards: &bounty.rewards,
        custom_severity_section: bounty.severity_section.as_deref(),
        out_of_scope_section: bounty.out_of_scope_section.as_deref(),
        known_issues_section: bounty.known_issues_section.as_deref(),
    })
    .map_err(anyhow::Error::from)
}

fn render_immunefi_severity_rubric_markdown(bounty: &ImmunefiBountyData) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# Immunefi Severity Rubric - {}\n\n",
        bounty.project
    ));
    out.push_str("Use this file as mandatory severity context for `validation_profile: immunefi-bounty`.\n\n");
    out.push_str("## Source URLs\n\n");
    out.push_str(&format!("- Information: {}\n", bounty.urls.information));
    out.push_str(&format!("- Scope: {}\n", bounty.urls.scope));
    out.push_str(&format!("- Resources: {}\n", bounty.urls.resources));
    if let Some(system) = &bounty.severity_system {
        out.push_str(&format!(
            "- Immunefi severity system {}: {}\n",
            system.version, system.url
        ));
    } else {
        out.push_str("- Immunefi severity system: not detected from page payload; use program impact rows first and verify the live bounty page before submission.\n");
    }
    out.push('\n');

    out.push_str("## Program-Specific Severity Source Of Truth\n\n");
    append_optional_bullet(&mut out, "Primacy", bounty.primacy.as_deref());
    append_optional_bullet(
        &mut out,
        "Proof of Concept",
        bounty.proof_of_concept_type.as_deref(),
    );
    out.push('\n');
    append_immunefi_impacts_section(&mut out, &bounty.smart_contract_impacts());
    append_immunefi_rewards_section(&mut out, &bounty.smart_contract_rewards());

    out.push_str("## Platform Smart Contract Severity Summary\n\n");
    out.push_str("Always prefer the program's exact impact rows above. Use this summary only to interpret the referenced Immunefi severity system.\n\n");
    match bounty
        .severity_system
        .as_ref()
        .map(|system| system.version.as_str())
    {
        Some("v2.2") => append_immunefi_v22_summary(&mut out),
        Some("v2.3") => append_immunefi_v23_summary(&mut out),
        _ => {
            append_immunefi_v23_summary(&mut out);
            out.push_str("The page did not expose a precise severity-system version in structured data; verify the live bounty page before final submission.\n\n");
        }
    }

    out.push_str("## Immunefi Severity Decision Rules\n\n");
    out.push_str("- Stage 3 must match the finding to an exact program impact row and severity.\n");
    out.push_str("- Apply Primacy of Impact only for the category and severity levels explicitly covered by this bounty.\n");
    out.push_str(
        "- Under Primacy of Rules, both the impacted asset and impact must be in scope.\n",
    );
    out.push_str("- Downgrade or reject findings requiring privileged access, leaked keys, malicious trusted roles, unusual user mistakes, unrealistic repeated interactions, or external-only failures.\n");
    out.push_str("- Feasibility limitations can affect payout and confidence; they should not replace the program's listed impact rows.\n");
    out.push_str("- Mark ambiguous, medium-only, best-practice-only, or weak-evidence findings as `Invalid` or `Needs Review`, not submission-ready.\n");
    out.push_str("- PoC policy is recorded here for later PoC stages only; stage 3 should not require an already-created PoC.\n\n");

    out.push_str("## PoC Policy For Later Stages Only\n\n");
    out.push_str("- Prefer a runnable local mainnet fork PoC when the affected in-scope asset is deployed on mainnet.\n");
    out.push_str("- Use the generated Immunefi PoC runtime artifact to select deployed addresses, networks, and RPC env var names.\n");
    out.push_str("- Use a local public-testnet fork only when the affected in-scope asset itself is a public-testnet deployment, or when no matching mainnet deployment exists but a relevant in-scope public-testnet deployment does.\n");
    out.push_str(
        "- Do not use local non-fork tests as the primary proof for deployed-asset findings.\n",
    );
    out.push_str("- Never broadcast live transactions, mutate live protocol state, steal funds, freeze funds, manipulate live governance, or cause real harm, even for a tiny amount.\n\n");

    out.push_str("## Link Handling\n\n");
    out.push_str("- Do not follow Immunefi navigation, marketing, login, social, newsletter, or platform-help links during validation.\n");
    out.push_str("- Use only the Source URLs above, in-scope explorer links, codebase links, documentation links, and prior-audit links when live verification is necessary.\n\n");

    out
}

fn render_immunefi_severity_rubric_json(bounty: &ImmunefiBountyData) -> Result<String> {
    let payload = serde_json::json!({
        "project": &bounty.project,
        "slug": &bounty.slug,
        "source_urls": {
            "information": &bounty.urls.information,
            "scope": &bounty.urls.scope,
            "resources": &bounty.urls.resources,
        },
        "severity_system": &bounty.severity_system,
        "proof_of_concept_type": &bounty.proof_of_concept_type,
        "primacy": &bounty.primacy,
        "assets": &bounty.smart_contract_assets(true),
        "impacts": &bounty.smart_contract_impacts(),
        "rewards": &bounty.smart_contract_rewards(),
        "default_out_of_scope_smart_contract": &bounty.default_out_of_scope_smart_contract,
        "default_out_of_scope_general": &bounty.default_out_of_scope_general,
        "custom_out_of_scope": &bounty.custom_out_of_scope,
        "prohibited_activities": &bounty.prohibited_activities,
        "known_issues": &bounty.known_issues,
        "audits": &bounty.audits,
        "documentations": &bounty.documentations,
        "codebases": &bounty.codebases,
    });
    serde_json::to_string_pretty(&payload).map_err(anyhow::Error::from)
}

fn append_optional_bullet(out: &mut String, label: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
        out.push_str(&format!("- {label}: {value}\n"));
    }
}

fn append_optional_section(out: &mut String, title: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
        let value = clean_immunefi_structured_text(value);
        out.push_str(&format!("### {title}\n\n{value}\n\n"));
    }
}

fn clean_immunefi_structured_text(value: &str) -> String {
    value
        .lines()
        .map(|line| line.trim_end_matches('\\').trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn append_immunefi_impacts_section(
    out: &mut String,
    impacts: &[crate::prepare_code::immunefi::ImmunefiImpact],
) {
    out.push_str("## Impacts In Scope\n\n");
    if impacts.is_empty() {
        out.push_str(
            "- No structured impact rows were extracted. Read the Scope tab directly.\n\n",
        );
        return;
    }
    for impact in impacts {
        out.push_str(&format!(
            "- {}{}: {}\n",
            impact.severity,
            impact
                .asset_type
                .as_deref()
                .map(|asset_type| format!(" ({})", asset_type.replace('_', " ")))
                .unwrap_or_default(),
            impact.description
        ));
    }
    out.push('\n');
}

fn append_immunefi_rewards_section(
    out: &mut String,
    rewards: &[crate::prepare_code::immunefi::ImmunefiReward],
) {
    out.push_str("## Rewards By Threat Level\n\n");
    if rewards.is_empty() {
        out.push_str(
            "- No structured reward rows were extracted. Read the Information tab directly.\n\n",
        );
        return;
    }
    for reward in rewards {
        let mut details = Vec::new();
        if let Some(min_reward) = reward.min_reward {
            details.push(format!("min ${min_reward}"));
        }
        if let Some(max_reward) = reward.max_reward {
            details.push(format!("max ${max_reward}"));
        }
        if let Some(fixed_reward) = reward.fixed_reward {
            details.push(format!("fixed ${fixed_reward}"));
        }
        out.push_str(&format!(
            "- {}{}{}{}\n",
            reward.severity,
            reward
                .asset_type
                .as_deref()
                .map(|asset_type| format!(" ({})", asset_type.replace('_', " ")))
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

fn append_immunefi_v23_summary(out: &mut String) {
    out.push_str("### v2.3 Smart Contract Summary\n\n");
    out.push_str("- Critical: direct theft of funds or NFTs, permanent freezing, protocol insolvency, governance result manipulation, unauthorized NFT minting, manipulable RNG abuse, or NFT representation alteration when listed by the program.\n");
    out.push_str("- High: theft or permanent freezing of unclaimed yield/royalties, temporary freezing of funds/NFTs, or other High rows listed by the program.\n");
    out.push_str("- Medium: griefing, block stuffing, gas theft, unbounded gas, or liveness failures only when listed by the program.\n");
    out.push_str(
        "- Low/Insight: lower-impact failures only when listed and rewarded by the program.\n\n",
    );
}

fn append_immunefi_v22_summary(out: &mut String) {
    out.push_str("### v2.2 Smart Contract Summary\n\n");
    out.push_str("- Critical and High are still impact-first, but some program pages using v2.2 add stricter profitability, freezing, or end-effect clauses.\n");
    out.push_str("- Always use the program's listed impact rows and reward body when v2.2 text conflicts with generic platform summaries.\n");
    out.push_str("- Treat project-specific exclusions for non-standard tokens, malicious integrations, and trusted components as mandatory.\n\n");
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
        if default_generated_scope_path_excludes(&rel) {
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

fn discover_local_protocol_doc_files(protocol_root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(protocol_root)
        .max_depth(4)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if !has_protocol_doc_extension(path) {
            continue;
        }
        let rel = path
            .strip_prefix(protocol_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        if is_protocol_doc_candidate(&rel) {
            paths.push(path.to_path_buf());
        }
    }
    paths.sort();
    paths.dedup();
    paths
}

fn has_protocol_doc_extension(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some("md" | "mdx" | "txt")
    )
}

fn is_protocol_doc_candidate(relative: &str) -> bool {
    let rel = relative.trim_start_matches("./").to_ascii_lowercase();
    if rel == "readme.md" || default_generated_scope_path_excludes(&rel) {
        return false;
    }

    let file_name = rel.rsplit('/').next().unwrap_or(&rel);
    if matches!(
        file_name,
        "license.md"
            | "license.txt"
            | "copying.md"
            | "copying.txt"
            | "changelog.md"
            | "changelog.txt"
            | "contributing.md"
            | "contributing.txt"
            | "code_of_conduct.md"
            | "package.md"
            | "package.txt"
    ) {
        return false;
    }

    if rel.starts_with("docs/") || rel.starts_with("audits/") {
        return true;
    }

    if rel.contains('/') {
        return false;
    }

    [
        "architecture",
        "audit",
        "contract",
        "deploy",
        "design",
        "invariant",
        "issue",
        "known",
        "protocol",
        "risk",
        "scope",
        "security",
        "spec",
    ]
    .iter()
    .any(|needle| file_name.contains(needle))
}

fn should_extract_links_from_source(source: &SourceContent) -> bool {
    let entry_kind = matches!(
        source.kind,
        ContextSourceKind::LocalReadme
            | ContextSourceKind::LocalMarkdown
            | ContextSourceKind::LocalProtocolDocs
            | ContextSourceKind::Code4renaContestRepo
            | ContextSourceKind::Code4renaBountyPage
            | ContextSourceKind::ImmunefiInformation
            | ContextSourceKind::ImmunefiScope
            | ContextSourceKind::ImmunefiResources
    );
    entry_kind
        && (source.reason == ENTRY_CONTEXT_REASON
            || source.reason == IMMUNEFI_BOUNTY_ENTRY_REASON
            || source.reason == CODE4RENA_BOUNTY_ENTRY_REASON
            || matches!(source.kind, ContextSourceKind::Code4renaContestRepo))
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
    if is_code4rena_bounty_required_doc(&text) {
        return LinkClassification::BountyRules;
    }
    if is_code4rena_bounty_listing_url(&text) {
        return LinkClassification::Scope;
    }
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
    if text.contains("github.com") && text.contains("/tree/") {
        if text.contains("docs") || text.contains("documentation") {
            return LinkClassification::Documentation;
        }
        if text.contains("audit") {
            return LinkClassification::PriorAudit;
        }
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

fn is_code4rena_bounty_required_doc(text: &str) -> bool {
    let clean = text
        .split_whitespace()
        .next()
        .unwrap_or(text)
        .trim_end_matches('/');
    clean == CODE4RENA_BOUNTY_GUIDE_URL
        || clean == CODE4RENA_BOUNTY_CRITERIA_URL
        || clean == "https://docs.code4rena.com/bounties/"
        || clean == "https://docs.code4rena.com/bounties/bounty-criteria/"
}

fn is_code4rena_bounty_listing_url(text: &str) -> bool {
    let clean = text
        .split_whitespace()
        .next()
        .unwrap_or(text)
        .trim_end_matches('/');
    clean.starts_with("https://code4rena.com/bounties/")
        || clean.starts_with("http://code4rena.com/bounties/")
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
    if *classification == LinkClassification::BountyRules {
        return None;
    }

    if is_generic_code4rena_docs(url) {
        return Some(
            "Skipped generic Code4rena documentation link; not protocol-specific scope/docs",
        );
    }

    if is_binary_or_pdf_url(url) && !is_pdf_url(url) {
        return Some(
            "Skipped binary link; only text, HTML, and best-effort PDF sources are fetched",
        );
    }

    if is_github_tree_url(url) && *classification == LinkClassification::SourceCode {
        return Some(
            "Skipped source-code GitHub tree; entry link traversal follows documentation trees only",
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
        && !is_pdf_url(url)
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

fn remote_link_counts_against_fetch_budget(classification: &LinkClassification) -> bool {
    *classification != LinkClassification::BountyRules
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

fn is_pdf_url(url: &str) -> bool {
    url_extension(url).as_deref() == Some("pdf")
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
            let decision = source_decision_for_link_classification(classification);
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

    if is_github_tree_url(url) {
        return fetch_github_tree_text_bundle(client, url, classification).await;
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
    if is_pdf_url(&fetch_url) {
        return fetch_pdf_text_source(client, url, &fetch_url, classification).await;
    }

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
        LinkClassification::BountyRules
            if fetch_url
                .trim_end_matches('/')
                .eq_ignore_ascii_case(CODE4RENA_BOUNTY_CRITERIA_URL) =>
        {
            ContextSourceKind::Code4renaBountyCriteria
        }
        LinkClassification::BountyRules => ContextSourceKind::Code4renaBountyGuide,
        _ if fetch_url.contains("raw.githubusercontent.com") => ContextSourceKind::GithubRaw,
        _ if is_html => ContextSourceKind::WebHtml,
        _ => ContextSourceKind::WebMarkdown,
    };
    let decision = source_decision_for_link_classification(classification);
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
    let Some(after_host) = clean.split("github.com/").nth(1) else {
        return None;
    };
    let parts = after_host.splitn(5, '/').collect::<Vec<_>>();
    if parts.len() < 5 || !matches!(parts[2], "blob" | "raw") {
        return None;
    }
    Some(format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        parts[0], parts[1], parts[3], parts[4]
    ))
}

fn source_decision_for_link_classification(classification: &LinkClassification) -> SourceDecision {
    match classification {
        LinkClassification::Documentation => SourceDecision::UsedForDocs,
        LinkClassification::Scope
        | LinkClassification::KnownIssues
        | LinkClassification::PriorAudit
        | LinkClassification::V12
        | LinkClassification::BountyRules => SourceDecision::UsedForScope,
        _ => SourceDecision::UsedForBoth,
    }
}

async fn fetch_github_tree_text_bundle(
    client: &reqwest::Client,
    url: &str,
    classification: &LinkClassification,
) -> Result<Option<SourceContent>> {
    let Some(tree) = parse_github_tree_url(url) else {
        return Ok(None);
    };
    let mut stack = vec![tree.path.clone()];
    let mut files = Vec::<(String, String)>::new();

    while let Some(path) = stack.pop() {
        if files.len() >= MAX_GITHUB_TREE_DOC_FILES {
            break;
        }
        let api_url = github_contents_api_url(&tree.owner, &tree.repo, &path, &tree.repo_ref);
        let response = client.get(&api_url).send().await?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "GitHub contents fetch failed with status {} for {}",
                response.status(),
                api_url
            ));
        }
        let value: serde_json::Value = response.json().await?;
        let items = match value {
            serde_json::Value::Array(items) => items,
            single @ serde_json::Value::Object(_) => vec![single],
            _ => Vec::new(),
        };

        for item in items {
            let item_type = item
                .get("type")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            let item_path = item
                .get("path")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            if item_path.is_empty() || default_generated_scope_path_excludes(item_path) {
                continue;
            }

            if item_type == "dir" {
                stack.push(item_path.to_string());
                continue;
            }

            if item_type != "file" || !has_text_like_extension(item_path) {
                continue;
            }
            let Some(download_url) = item.get("download_url").and_then(|value| value.as_str())
            else {
                continue;
            };
            if let Some(text) = fetch_optional_text(client, download_url).await? {
                files.push((item_path.to_string(), text));
                if files.len() >= MAX_GITHUB_TREE_DOC_FILES {
                    break;
                }
            }
        }
    }

    if files.is_empty() {
        return Ok(None);
    }

    files.sort_by(|a, b| a.0.cmp(&b.0));
    let mut bundle = format!(
        "# GitHub Documentation Tree\n\nSource tree: {}\nRepository: {}/{}\nRef: {}\n\n",
        url, tree.owner, tree.repo, tree.repo_ref
    );
    for (path, content) in files {
        let excerpt = if get_token_count(&content) > MAX_PROMPT_SOURCE_TOKENS_PER_ITEM {
            truncate_to_token_limit(content, MAX_PROMPT_SOURCE_TOKENS_PER_ITEM)
        } else {
            content
        };
        let candidate = format!("## `{path}`\n\n{excerpt}\n\n");
        if get_token_count(&format!("{bundle}{candidate}")) > MAX_GITHUB_TREE_DOC_TOKENS {
            bundle.push_str(&format!(
                "\n[Omitted remaining GitHub tree files after reaching {} token bundle budget]\n",
                MAX_GITHUB_TREE_DOC_TOKENS
            ));
            break;
        }
        bundle.push_str(&candidate);
    }

    log_source_content_loaded(url, &bundle);
    Ok(Some(SourceContent {
        id: "github-tree".to_string(),
        kind: ContextSourceKind::GithubMarkdown,
        location: url.to_string(),
        title: Some("GitHub documentation tree".to_string()),
        content: bundle,
        decision: source_decision_for_link_classification(classification),
        reason: "Fetched text files from linked GitHub documentation tree".to_string(),
    }))
}

#[derive(Debug)]
struct GithubTreeRef {
    owner: String,
    repo: String,
    repo_ref: String,
    path: String,
}

fn parse_github_tree_url(url: &str) -> Option<GithubTreeRef> {
    let clean = url
        .split('#')
        .next()
        .unwrap_or(url)
        .split('?')
        .next()
        .unwrap_or(url)
        .trim_end_matches('/');
    let after_host = clean
        .split("github.com/")
        .nth(1)
        .or_else(|| clean.split("www.github.com/").nth(1))?;
    let parts = after_host.split('/').collect::<Vec<_>>();
    if parts.len() < 5 || parts[2] != "tree" {
        return None;
    }
    Some(GithubTreeRef {
        owner: parts[0].to_string(),
        repo: parts[1].to_string(),
        repo_ref: parts[3].to_string(),
        path: parts[4..].join("/"),
    })
}

fn github_contents_api_url(owner: &str, repo: &str, path: &str, repo_ref: &str) -> String {
    format!(
        "https://api.github.com/repos/{owner}/{repo}/contents/{}?ref={repo_ref}",
        path.trim_start_matches('/')
    )
}

async fn fetch_pdf_text_source(
    client: &reqwest::Client,
    original_url: &str,
    fetch_url: &str,
    classification: &LinkClassification,
) -> Result<Option<SourceContent>> {
    let response = client.get(fetch_url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow!(
            "PDF fetch failed with status {} for {}",
            response.status(),
            fetch_url
        ));
    }
    let bytes = response.bytes().await?;
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let pdf_path = std::env::temp_dir().join(format!(
        "ai-agent-audit-context-{}-{stamp}.pdf",
        std::process::id()
    ));
    fs::write(&pdf_path, &bytes)
        .with_context(|| format!("Failed to write temporary PDF {}", pdf_path.display()))?;
    let output = std::process::Command::new("pdftotext")
        .arg("-layout")
        .arg(&pdf_path)
        .arg("-")
        .output();
    let _ = fs::remove_file(&pdf_path);
    let output = output.with_context(|| {
        "pdftotext is required to extract linked PDF audit/documentation content".to_string()
    })?;
    if !output.status.success() {
        anyhow::bail!(
            "pdftotext failed for {}: {}",
            fetch_url,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let content = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if content.is_empty() {
        return Ok(None);
    }
    log_source_content_loaded(fetch_url, &content);
    Ok(Some(SourceContent {
        id: "linked-pdf".to_string(),
        kind: ContextSourceKind::WebMarkdown,
        location: fetch_url.to_string(),
        title: Some(original_url.to_string()),
        content,
        decision: source_decision_for_link_classification(classification),
        reason: "Extracted text from linked PDF with pdftotext".to_string(),
    }))
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
    code4rena_bounty: Option<&Code4renaBountyData>,
    immunefi_bounty: Option<&ImmunefiBountyData>,
) -> Result<ScopeFileList> {
    // Scope-file generation has a strict preference order:
    // 1. user/protocol supplied scope.txt,
    // 2. deterministic extraction from trusted docs and structured bounty data,
    // 3. external explorer/deployment metadata mapping,
    // 4. fallback to configured code folders only when that is safe.
    //
    // For bounties, fallback is intentionally limited. If a page lists direct
    // explorer assets or direct `.sol` blob links, failing to map them probably
    // means the repo/subfolder is wrong.
    let source_scope = protocol_root.join("scope.txt");
    if source_scope.exists() {
        // A checked-in scope.txt wins over inferred scope. We still normalize
        // and existence-check lines so downstream reports can show misses.
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
    let mut external_scope_asset_count = 0usize;
    let mut external_scope_entry_count = 0usize;
    let mut external_scope_unresolved = Vec::new();
    let scope_platform = scope_bounty_platform(cli);
    let external_contract_scope_audit = scope_platform.is_some();
    // C4 pages often list contract names or GitHub blobs instead of local
    // paths. Resolve those before trying heavier explorer metadata.
    let code4rena_scope =
        collect_code4rena_bounty_scope(cli, protocol_root, sources, code4rena_bounty);
    if !code4rena_scope.entries.is_empty() {
        source = ScopeFileSource::ExtractedFromBountyScope;
        extracted.extend(code4rena_scope.entries);
    }
    warnings.extend(code4rena_scope.warnings);
    if external_contract_scope_audit {
        // Bounty explorer/deployment assets are precise scope claims. Map them
        // to local Solidity files before permitting any broad fallback.
        let external_scope = resolve_external_contract_scope(
            cli,
            protocol_root,
            output_path,
            sources,
            code4rena_bounty,
            immunefi_bounty,
        )
        .await?;
        external_scope_asset_count = external_scope.assets.len();
        external_scope_entry_count = external_scope.entries.len();
        external_scope_unresolved = external_scope.unresolved.clone();
        if !external_scope.entries.is_empty() {
            source = ScopeFileSource::ExtractedFromExternalContractScope;
            extracted.extend(external_scope.entries);
        }
        warnings.extend(external_scope.warnings);
    }
    info!(
        "Deterministic entry-context scope extraction found {} candidate Solidity files",
        extracted.len()
    );

    extracted.sort_by(|a, b| a.path.cmp(&b.path));
    extracted.dedup_by(|a, b| a.path == b.path);
    extracted.retain(|entry| {
        !default_generated_scope_path_excludes(&entry.path)
            && !default_dependency_scope_path_excludes(&entry.path)
            && !default_non_runtime_scope_path_excludes(&entry.path)
    });
    let existing_count = extracted.iter().filter(|entry| entry.exists).count();
    // Contract-name extraction proves the bounty declared concrete contracts.
    // If none map locally, the prepared repo/subfolder is probably wrong.
    if scope_platform == Some(BountyPlatform::Code4rena)
        && !code4rena_scope.discovered_names.is_empty()
        && existing_count == 0
    {
        anyhow::bail!(
            "Found Code4rena bounty scope contract names in context but none mapped to local Solidity files: {}. Provide context.files/context.urls that identify the source repo, adjust code_folders/subfolder, or provide scoped_files explicitly.",
            code4rena_scope.discovered_names.join(", ")
        );
    }
    if external_contract_scope_audit
        && external_scope_asset_count > 0
        && external_scope_entry_count == 0
    {
        // Explorer-linked assets are exact deployed-scope inputs. Auditing all
        // code instead would produce false confidence and mispriced findings.
        anyhow::bail!(
            "Found {external_scope_asset_count} external explorer-linked bounty scope assets but none mapped to local Solidity files. See {} for details. Provide the correct source repo/subfolder or add scoped_files explicitly.",
            external_scope_resolution_report_path(output_path).display()
        );
    }
    if scope_platform == Some(BountyPlatform::Immunefi)
        && external_scope_asset_count == 0
        && immunefi_bounty
            .map(|bounty| !bounty.smart_contract_assets(false).is_empty())
            .unwrap_or(false)
    {
        // Immunefi Smart Contract scope must resolve from structured assets or
        // deployment pages; Web & App / broad repo content is intentionally out.
        anyhow::bail!(
            "Immunefi Smart Contract assets were found, but no explorer-linked deployed contracts could be extracted directly or from deployment pages. See {} for details. Refusing to fall back to all code folders because Immunefi scope must be precise.",
            external_scope_resolution_report_path(output_path).display()
        );
    }
    if external_contract_scope_audit && !external_scope_unresolved.is_empty() {
        // C4 bounties fail closed on any unresolved deployed asset. Immunefi
        // records unresolved assets as warnings after mapped entries exist.
        let preview = external_scope_unresolved
            .iter()
            .take(10)
            .map(|asset| format!("{} ({})", asset.label, asset.address))
            .collect::<Vec<_>>()
            .join(", ");
        if scope_platform == Some(BountyPlatform::Code4rena) {
            anyhow::bail!(
                "Could not map {} external explorer-linked bounty scope assets to local Solidity files: {}. See {} for the full resolution report. Refusing to fall back to all code folders because Code4rena bounty submissions cost money and scope must be precise.",
                external_scope_unresolved.len(),
                preview,
                external_scope_resolution_report_path(output_path).display()
            );
        }
        warnings.push(format!(
            "Could not map {} Immunefi external explorer-linked bounty scope assets to local Solidity files: {}. These assets are omitted from generated scoped_files; see {} for the full resolution report.",
            external_scope_unresolved.len(),
            preview,
            external_scope_resolution_report_path(output_path).display()
        ));
    }
    if extracted.is_empty() || existing_count == 0 {
        // Generic/competition audits can fall back to configured code folders.
        // Bounty modes only reach this point when no precise structured scope
        // exists or when platform policy explicitly allows the fallback.
        if scope_platform == Some(BountyPlatform::Code4rena)
            && code4rena_bounty
                .map(code4rena_structured_scope_requires_precise_mapping)
                .unwrap_or(false)
        {
            anyhow::bail!(
                "Could not map Code4rena bounty smart-contract scope assets to local Solidity files. Refusing to fall back to all Solidity files under configured code_folders {:?}. Provide the correct repo/subfolder or scoped_files explicitly.",
                cli.code_folders
            );
        }
        if scope_platform == Some(BountyPlatform::Immunefi) {
            anyhow::bail!(
                "Could not map Immunefi Smart Contract scope to local Solidity files. Refusing to fall back to all Solidity files under configured code_folders {:?}. Provide the correct repo/subfolder or scoped_files explicitly.",
                cli.code_folders
            );
        }
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
                | ContextSourceKind::LocalProtocolDocs
                | ContextSourceKind::Code4renaContestRepo
                | ContextSourceKind::GithubMarkdown
                | ContextSourceKind::GithubRaw
                | ContextSourceKind::WebMarkdown
                | ContextSourceKind::WebHtml
                | ContextSourceKind::Code4renaBountyPage
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

fn collect_scoped_code_documentation_sources(
    protocol_root: &Path,
    scope_file_list: &ScopeFileList,
) -> Vec<SourceContent> {
    let mut sources = Vec::new();
    let mut seen = HashSet::new();
    for entry in &scope_file_list.files {
        if !entry.exists || !entry.path.ends_with(".sol") {
            continue;
        }
        let relative = entry.path.trim_start_matches("./");
        if !seen.insert(relative.to_string()) {
            continue;
        }
        let path = protocol_root.join(relative);
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let excerpt = extract_solidity_documentation_excerpt(&content);
        if excerpt.trim().is_empty() {
            continue;
        }
        let excerpt = if get_token_count(&excerpt) > MAX_SCOPED_CODE_DOC_TOKENS_PER_FILE {
            truncate_to_token_limit(excerpt, MAX_SCOPED_CODE_DOC_TOKENS_PER_FILE)
        } else {
            excerpt
        };
        let rendered = format!(
            "# Scoped Solidity Documentation\n\nFile: `{}`\n\n{}",
            entry.path, excerpt
        );
        log_source_content_loaded(&path.to_string_lossy(), &rendered);
        sources.push(SourceContent {
            id: format!("scoped-code-docs-{}", sources.len() + 1),
            kind: ContextSourceKind::ScopedCodeDocs,
            location: path.to_string_lossy().to_string(),
            title: Some(entry.path.clone()),
            content: rendered,
            decision: SourceDecision::UsedForDocs,
            reason:
                "Extracted NatSpec, comments, declarations, inheritance, and local imports from scoped Solidity"
                    .to_string(),
        });
    }
    sources
}

fn extract_solidity_documentation_excerpt(content: &str) -> String {
    let declaration_re = Regex::new(
        r"\b(contract|interface|library)\s+\w+|\bfunction\s+\w+|\bevent\s+\w+|\berror\s+\w+",
    )
    .unwrap();
    let state_var_re = Regex::new(
        r"\b(public|external)\b.*\b(address|bool|bytes\d*|int\d*|uint\d*|string|mapping)\b",
    )
    .unwrap();
    let mut out = Vec::new();
    let mut in_block_comment = false;
    let mut pending_comment = false;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            pending_comment = false;
            continue;
        }

        if line.starts_with("import ") {
            if line.contains('"') || line.contains('\'') {
                out.push(line.to_string());
            }
            continue;
        }

        let starts_comment = line.starts_with("//")
            || line.starts_with("/*")
            || line.starts_with('*')
            || line.starts_with("*/");
        if starts_comment || in_block_comment {
            out.push(line.to_string());
            pending_comment = true;
            if line.starts_with("/*") && !line.contains("*/") {
                in_block_comment = true;
            }
            if line.contains("*/") {
                in_block_comment = false;
            }
            continue;
        }

        if declaration_re.is_match(line) || (pending_comment && state_var_re.is_match(line)) {
            out.push(line.to_string());
            pending_comment = false;
        }
    }

    out.join("\n")
}

#[derive(Debug, Default)]
struct BountyContractNameScope {
    entries: Vec<ScopeFileEntry>,
    discovered_names: Vec<String>,
    unmapped_names: Vec<String>,
}

#[derive(Debug, Default)]
struct BountyScopeExtraction {
    entries: Vec<ScopeFileEntry>,
    discovered_names: Vec<String>,
    warnings: Vec<String>,
}

fn collect_code4rena_bounty_scope(
    cli: &Cli,
    protocol_root: &Path,
    sources: &[SourceContent],
    bounty: Option<&Code4renaBountyData>,
) -> BountyScopeExtraction {
    if !matches!(cli.audit_type, AuditType::Code4renaBounty) {
        return BountyScopeExtraction::default();
    }

    // Code4rena bounty scope may be written as contract names, GitHub blob
    // links, or explorer/deployment links. This helper collects the local-file
    // forms; explorer links are handled by external scope resolution later.
    let mut extraction = BountyScopeExtraction::default();
    let name_scope = deterministic_bounty_contract_name_scope_extract(sources, protocol_root, cli);
    extraction.discovered_names = name_scope.discovered_names;
    extraction.entries.extend(name_scope.entries);
    if !name_scope.unmapped_names.is_empty() {
        extraction.warnings.push(format!(
            "Could not map {} bounty scope contract names to local Solidity definitions: {}",
            name_scope.unmapped_names.len(),
            name_scope.unmapped_names.join(", ")
        ));
    }

    if let Some(bounty) = bounty {
        extraction
            .entries
            .extend(code4rena_github_scope_entries(bounty, protocol_root, cli));
    }

    extraction.entries.sort_by(|a, b| a.path.cmp(&b.path));
    extraction.entries.dedup_by(|a, b| a.path == b.path);
    extraction
}

fn deterministic_bounty_contract_name_scope_extract(
    sources: &[SourceContent],
    protocol_root: &Path,
    cli: &Cli,
) -> BountyContractNameScope {
    let mut discovered_names = bounty_contract_names_from_scope_sections(sources);
    discovered_names.sort();
    discovered_names.dedup();

    if discovered_names.is_empty() {
        return BountyContractNameScope::default();
    }

    let excluded_folders = bounty_declared_oos_folders(sources, cli);
    let definition_index = solidity_definition_index(protocol_root, &excluded_folders);
    let mut entries = Vec::new();
    let mut unmapped_names = Vec::new();

    for name in &discovered_names {
        match definition_index.get(name).and_then(|paths| paths.first()) {
            Some(path) => entries.push(ScopeFileEntry {
                path: path.clone(),
                exists: true,
                reason: Some("Mapped from Code4rena bounty scope contract name".to_string()),
            }),
            None => unmapped_names.push(name.clone()),
        }
    }

    entries.sort_by(|a, b| a.path.cmp(&b.path));
    entries.dedup_by(|a, b| a.path == b.path);

    BountyContractNameScope {
        entries,
        discovered_names,
        unmapped_names,
    }
}

fn code4rena_github_scope_entries(
    bounty: &Code4renaBountyData,
    protocol_root: &Path,
    cli: &Cli,
) -> Vec<ScopeFileEntry> {
    // Code4rena scope can link directly to GitHub blob/raw URLs. In single-repo
    // mode those paths are relative to the cloned repo root; in polyrepo mode
    // they must be prefixed with the owner-qualified clone folder so `scope.txt`
    // points at `workspace/<owner-repo>/...` rather than `workspace/...`.
    let mut entries = Vec::new();
    for asset in &bounty.scope_assets {
        let repo_url = github_repo_url_from_scope_url(&asset.url);
        let matched_repo = repo_url.as_deref().and_then(|repo_url| {
            cli.resolved_repos
                .iter()
                .find(|repo| same_github_repo(repo_url, &repo.repo_url))
        });
        let branch_hint = matched_repo
            .and_then(|repo| repo.branch.as_deref())
            .or(cli.repo_branch.as_deref());
        let repo_prefix = if cli.resolved_repos.len() > 1 {
            matched_repo.map(|repo| repo_slug_from_github_url(&repo.repo_url))
        } else {
            None
        };
        if let Some(path) = github_blob_scope_path(
            &asset.url,
            branch_hint,
            protocol_root,
            repo_prefix.as_deref(),
        ) {
            entries.push(normalize_scope_path(
                &path,
                protocol_root,
                Some(format!(
                    "Mapped from Code4rena GitHub scope link `{}`",
                    asset.label
                )),
            ));
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    entries.dedup_by(|a, b| a.path == b.path);
    entries
}

fn github_blob_scope_path(
    url: &str,
    branch_hint: Option<&str>,
    protocol_root: &Path,
    repo_prefix: Option<&str>,
) -> Option<String> {
    // Prefer the candidate that exists locally. This lets slash-named branch
    // hints produce an exact path while keeping a fallback candidate for error
    // reporting if the local repo is wrong or not yet populated as expected.
    let candidates = github_blob_scope_path_candidates(url, branch_hint);
    let candidates = candidates
        .into_iter()
        .map(|path| prefix_scope_path(&path, repo_prefix))
        .collect::<Vec<_>>();
    candidates
        .iter()
        .find(|path| protocol_root.join(path.trim_start_matches("./")).is_file())
        .cloned()
        .or_else(|| candidates.into_iter().next())
}

fn github_blob_scope_path_candidates(url: &str, branch_hint: Option<&str>) -> Vec<String> {
    // GitHub blob/raw paths are ambiguous when branches contain slashes. If the
    // clone stage has resolved a branch hint, strip exactly that branch prefix.
    // Otherwise generate ordered suffix candidates and let existence checks pick
    // the right one.
    let clean = url
        .split('?')
        .next()
        .unwrap_or(url)
        .split('#')
        .next()
        .unwrap_or(url)
        .trim_end_matches('/');
    let Some(after_host) = clean.split("github.com/").nth(1) else {
        return Vec::new();
    };
    let parts = after_host.split('/').collect::<Vec<_>>();
    if parts.len() < 5 || !matches!(parts[2], "blob" | "raw") {
        return Vec::new();
    }

    if let Some(branch) = branch_hint {
        let branch_parts = branch
            .split('/')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();
        let path_start = 3 + branch_parts.len();
        if parts.len() > path_start
            && parts[3..path_start]
                .iter()
                .copied()
                .eq(branch_parts.iter().copied())
        {
            let path = parts[path_start..].join("/");
            if path.ends_with(".sol") {
                return vec![format!("./{path}")];
            }
        }
    }

    let mut candidates = Vec::new();
    let mut seen = HashSet::new();
    for path_start in 4..parts.len() {
        let path = parts[path_start..].join("/");
        if path.ends_with(".sol") {
            let candidate = format!("./{path}");
            if seen.insert(candidate.clone()) {
                candidates.push(candidate);
            }
        }
    }
    candidates
}

fn prefix_scope_path(path: &str, repo_prefix: Option<&str>) -> String {
    // Polyrepo clone folders are owner-qualified (`org-repo`) to avoid basename
    // collisions. Prefix only once so pre-prefixed paths remain idempotent.
    let Some(repo_prefix) = repo_prefix else {
        return path.to_string();
    };
    let repo_prefix = repo_prefix.trim_matches('/');
    if repo_prefix.is_empty() {
        return path.to_string();
    }
    let raw_path = path.trim_start_matches("./");
    if raw_path == repo_prefix || raw_path.starts_with(&format!("{repo_prefix}/")) {
        format!("./{raw_path}")
    } else {
        format!("./{repo_prefix}/{raw_path}")
    }
}

fn github_repo_url_from_scope_url(url: &str) -> Option<String> {
    let clean = url
        .split('?')
        .next()
        .unwrap_or(url)
        .split('#')
        .next()
        .unwrap_or(url)
        .trim_end_matches('/')
        .trim_end_matches(".git");
    let after_host = clean.split("github.com/").nth(1)?;
    let parts = after_host
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if parts.len() < 2 || matches!(parts[0], "orgs" | "users") {
        return None;
    }
    Some(format!("https://github.com/{}/{}", parts[0], parts[1]))
}

fn repo_slug_from_github_url(repo_url: &str) -> String {
    let clean = repo_url
        .trim()
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .split('?')
        .next()
        .unwrap_or(repo_url)
        .split('#')
        .next()
        .unwrap_or(repo_url);
    if let Some(after_host) = clean.split("github.com/").nth(1) {
        let parts = after_host
            .split('/')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();
        if parts.len() >= 2 {
            return sanitize_scope_component(&format!("{}-{}", parts[0], parts[1]));
        }
    }
    sanitize_scope_component(clean.rsplit('/').next().unwrap_or("repo"))
}

fn sanitize_scope_component(raw: &str) -> String {
    raw.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn code4rena_structured_scope_requires_precise_mapping(bounty: &Code4renaBountyData) -> bool {
    bounty.scope_assets.iter().any(|asset| {
        let url = clean_external_contract_url(&asset.url);
        is_explorer_contract_url(&url) || !github_blob_scope_path_candidates(&url, None).is_empty()
    })
}

async fn resolve_external_contract_scope(
    cli: &Cli,
    protocol_root: &Path,
    output_path: &Path,
    sources: &[SourceContent],
    code4rena_bounty: Option<&Code4renaBountyData>,
    immunefi_bounty: Option<&ImmunefiBountyData>,
) -> Result<ExternalContractScopeResolution> {
    // External contract resolution bridges deployed bounty assets back to local
    // source. It first tries deterministic metadata/name matching, then gives a
    // bounded Codex worker one structured chance to resolve remaining assets.
    // Any unresolved explorer-linked asset causes bounty scope generation to
    // fail closed later in `generate_scope_txt`.
    let mut resolution = ExternalContractScopeResolution::default();
    resolution.assets =
        external_contract_scope_assets_for_audit(cli, sources, code4rena_bounty, immunefi_bounty);
    // Deployment/wiki pages are a second structured source for bounty scope.
    // They are platform-specific at the edge, but both resolve into the same
    // explorer asset model before local Solidity matching begins.
    match scope_bounty_platform(cli) {
        Some(BountyPlatform::Code4rena) => {
            if let Some(bounty) = code4rena_bounty {
                let page_assets = fetch_code4rena_deployment_page_scope_assets(bounty).await;
                merge_external_scope_assets(&mut resolution.assets, page_assets);
            }
        }
        Some(BountyPlatform::Immunefi) => {
            if let Some(bounty) = immunefi_bounty {
                let page_assets = fetch_immunefi_deployment_page_scope_assets(bounty).await;
                merge_external_scope_assets(&mut resolution.assets, page_assets);
            }
        }
        None => {}
    }
    if resolution.assets.is_empty() {
        return Ok(resolution);
    }

    info!(
        "External contract scope extraction found {} explorer-linked contract assets",
        resolution.assets.len()
    );

    let excluded_folders = bounty_declared_oos_folders(sources, cli);
    let definitions = solidity_definitions(protocol_root, &excluded_folders);
    if definitions.is_empty() {
        resolution.warnings.push(format!(
            "No Solidity contract/library/interface definitions were found under {}; explorer-linked scope cannot be mapped.",
            protocol_root.display()
        ));
        write_external_contract_scope_report(output_path, &resolution)?;
        return Ok(resolution);
    }

    let mut unresolved = Vec::new();
    for asset in resolution.assets.clone() {
        if let Some(record) = deterministic_external_asset_resolution(&asset, None, &definitions) {
            push_external_resolution_record(&mut resolution, protocol_root, record);
        } else {
            unresolved.push(asset);
        }
    }
    resolution.unresolved = unresolved;

    fetch_metadata_and_retry_external_scope_resolution(
        &mut resolution,
        protocol_root,
        &definitions,
    )
    .await;

    if !resolution.unresolved.is_empty() {
        match build_external_scope_agent() {
            Ok(agent) => {
                match codex_resolve_external_scope(&agent, &resolution, &definitions).await {
                    Ok(model_resolution) => apply_codex_external_scope_resolution(
                        &mut resolution,
                        protocol_root,
                        &definitions,
                        model_resolution,
                    ),
                    Err(err) => resolution.warnings.push(format!(
                        "Codex fallback scope resolver failed; unresolved explorer-linked scope assets remain: {err:#}"
                    )),
                }
            }
            Err(err) => resolution.warnings.push(format!(
                "Could not create Codex fallback scope resolver; unresolved explorer-linked scope assets remain: {err:#}"
            )),
        }
    }

    resolution.entries.sort_by(|a, b| a.path.cmp(&b.path));
    resolution.entries.dedup_by(|a, b| a.path == b.path);
    write_external_contract_scope_report(output_path, &resolution)?;
    Ok(resolution)
}

fn external_contract_scope_assets_for_audit(
    cli: &Cli,
    sources: &[SourceContent],
    code4rena_bounty: Option<&Code4renaBountyData>,
    immunefi_bounty: Option<&ImmunefiBountyData>,
) -> Vec<ExternalContractScopeAsset> {
    // Prefer structured platform assets over scraped source text. If a bounty
    // page has structured scope but no direct explorer links, leave deployment
    // pages to the dedicated fetcher instead of broad source scraping.
    match scope_bounty_platform(cli) {
        Some(BountyPlatform::Code4rena) => {
            if let Some(bounty) = code4rena_bounty {
                let assets = code4rena_external_contract_scope_assets(bounty);
                if !assets.is_empty() {
                    return assets;
                }
                if bounty.has_structured_scope_assets() {
                    return Vec::new();
                }
            }
        }
        Some(BountyPlatform::Immunefi) => {
            if let Some(bounty) = immunefi_bounty {
                let assets = immunefi_external_contract_scope_assets(bounty);
                if !assets.is_empty() {
                    return assets;
                }
                if !bounty.smart_contract_assets(false).is_empty() {
                    return Vec::new();
                }
            }
        }
        None => {}
    }

    external_contract_scope_assets_from_sources(sources)
}

fn code4rena_external_contract_scope_assets(
    bounty: &Code4renaBountyData,
) -> Vec<ExternalContractScopeAsset> {
    let mut assets = Vec::new();
    let mut seen = BTreeSet::new();

    for asset in &bounty.scope_assets {
        push_explorer_scope_asset(
            &mut assets,
            &mut seen,
            &asset.label,
            &asset.url,
            &bounty.url,
        );
    }

    assets.sort_by(|a, b| {
        a.label
            .cmp(&b.label)
            .then_with(|| a.address.cmp(&b.address))
            .then_with(|| a.url.cmp(&b.url))
    });
    assets
}

fn immunefi_external_contract_scope_assets(
    bounty: &ImmunefiBountyData,
) -> Vec<ExternalContractScopeAsset> {
    let mut assets = Vec::new();
    let mut seen = BTreeSet::new();

    for asset in bounty.smart_contract_assets(false) {
        if asset.is_primacy_of_impact {
            continue;
        }
        let label = asset
            .description
            .as_deref()
            .map(str::trim)
            .filter(|description| !description.is_empty())
            .unwrap_or_default()
            .to_string();
        push_explorer_scope_asset(
            &mut assets,
            &mut seen,
            &label,
            &asset.url,
            &bounty.urls.scope,
        );
    }

    assets.sort_by(|a, b| {
        a.label
            .cmp(&b.label)
            .then_with(|| a.address.cmp(&b.address))
            .then_with(|| a.url.cmp(&b.url))
    });
    assets
}

fn push_explorer_scope_asset(
    assets: &mut Vec<ExternalContractScopeAsset>,
    seen: &mut BTreeSet<String>,
    label: &str,
    raw_url: &str,
    source_location: &str,
) {
    // Normalize all platform-specific asset rows into one explorer/address
    // record so later local-file matching does not care where it came from.
    let url = clean_external_contract_url(raw_url);
    if !is_explorer_contract_url(&url) {
        return;
    }
    let Some(address) = contract_address_from_url(&url) else {
        return;
    };
    let explorer = explorer_host(&url).unwrap_or_else(|| "unknown-explorer".to_string());
    let key = format!(
        "{}|{}",
        explorer.to_ascii_lowercase(),
        address.to_ascii_lowercase()
    );
    if seen.insert(key) {
        let label = label.trim();
        assets.push(ExternalContractScopeAsset {
            label: if label.is_empty() {
                address.clone()
            } else {
                label.to_string()
            },
            url,
            address,
            explorer,
            source_location: source_location.to_string(),
        });
    }
}

async fn fetch_immunefi_deployment_page_scope_assets(
    bounty: &ImmunefiBountyData,
) -> Vec<ExternalContractScopeAsset> {
    let candidates = bounty
        .smart_contract_assets(false)
        .into_iter()
        .filter(|asset| !asset.is_primacy_of_impact && !is_explorer_contract_url(&asset.url))
        .map(|asset| DeploymentScopePageCandidate {
            url: asset.url.clone(),
            label: asset.description.clone(),
        })
        .collect::<Vec<_>>();
    fetch_platform_deployment_page_scope_assets(
        "Immunefi",
        "ai-agent-audit-immunefi-scope/0.1",
        candidates,
    )
    .await
}

async fn fetch_code4rena_deployment_page_scope_assets(
    bounty: &Code4renaBountyData,
) -> Vec<ExternalContractScopeAsset> {
    let candidates = bounty
        .scope_assets
        .iter()
        .filter(|asset| !is_explorer_contract_url(&asset.url))
        .map(|asset| DeploymentScopePageCandidate {
            url: asset.url.clone(),
            label: Some(asset.label.clone()),
        })
        .collect::<Vec<_>>();
    fetch_platform_deployment_page_scope_assets(
        "Code4rena",
        "ai-agent-audit-code4rena-bounty-scope/0.1",
        candidates,
    )
    .await
}

async fn fetch_platform_deployment_page_scope_assets(
    platform_name: &str,
    user_agent: &str,
    candidates: Vec<DeploymentScopePageCandidate>,
) -> Vec<ExternalContractScopeAsset> {
    // Some bounty rows point to deployment wikis instead of explorers. Fetch
    // only likely deployment pages, then keep production/mainnet sections.
    let Ok(client) = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(HTTP_TIMEOUT_SECS))
        .user_agent(user_agent)
        .build()
    else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for candidate in candidates {
        let url = clean_external_contract_url(&candidate.url);
        if !is_remote_link(&url) || !likely_deployment_scope_page(&url, candidate.label.as_deref())
        {
            continue;
        }
        match fetch_deployment_scope_page_text(&client, &url).await {
            Ok(text) => {
                let production_text = production_scope_text(&text);
                let parsed = external_contract_assets_from_text(&production_text, &url);
                if parsed.is_empty() {
                    debug!(
                        "No explorer contract links extracted from {platform_name} deployment page {url}"
                    );
                }
                out.extend(parsed);
            }
            Err(err) => {
                warn!("Could not fetch {platform_name} deployment scope page {url}: {err:#}");
            }
        }
    }
    out
}

fn likely_deployment_scope_page(url: &str, description: Option<&str>) -> bool {
    let text = format!("{} {}", url, description.unwrap_or_default()).to_ascii_lowercase();
    text.contains("deployment")
        || text.contains("deployments")
        || text.contains("deployed")
        || text.contains("contract")
        || text.contains("contracts")
}

async fn fetch_deployment_scope_page_text(client: &reqwest::Client, url: &str) -> Result<String> {
    let fetch_url = github_wiki_raw_url(url).unwrap_or_else(|| url.to_string());
    let response = client.get(&fetch_url).send().await?;
    if !response.status().is_success() {
        anyhow::bail!(
            "deployment page fetch failed with status {} for {}",
            response.status(),
            fetch_url
        );
    }
    let text = response.text().await?;
    if looks_like_html(&text) {
        Ok(html_to_text(&text))
    } else {
        Ok(text)
    }
}

fn github_wiki_raw_url(url: &str) -> Option<String> {
    let clean = url
        .split('?')
        .next()
        .unwrap_or(url)
        .split('#')
        .next()
        .unwrap_or(url)
        .trim_end_matches('/');
    let after_host = clean.split("github.com/").nth(1)?;
    let parts = after_host.split('/').collect::<Vec<_>>();
    if parts.len() < 4 || parts[2] != "wiki" {
        return None;
    }
    let page = parts[3..].join("/");
    Some(format!(
        "https://raw.githubusercontent.com/wiki/{}/{}/{}.md",
        parts[0], parts[1], page
    ))
}

fn production_scope_text(text: &str) -> String {
    let mut out = Vec::new();
    let mut include = true;
    let mut saw_heading = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            saw_heading = true;
            let heading = trimmed.trim_start_matches('#').trim().to_ascii_lowercase();
            include = production_scope_heading(&heading);
        }
        if include || !saw_heading {
            out.push(line);
        }
    }
    out.join("\n")
}

fn production_scope_heading(heading: &str) -> bool {
    let testnet_markers = [
        "testnet",
        "sepolia",
        "holesky",
        "goerli",
        "rinkeby",
        "kovan",
        "mumbai",
        "amoy",
        "fuji",
        "chiado",
        "alfajores",
        "bepolia",
    ];
    if testnet_markers
        .iter()
        .any(|marker| heading.contains(marker))
    {
        return false;
    }
    let generic_production_markers = [
        "production",
        "deployment",
        "deployments",
        "deployed",
        "contract",
        "contracts",
        "address",
        "addresses",
        "smart contract",
        "smart contracts",
    ];
    if generic_production_markers
        .iter()
        .any(|marker| heading.contains(marker))
    {
        return true;
    }
    let production_markers = [
        "mainnet",
        "ethereum",
        "arbitrum",
        "arbitrum one",
        "optimism",
        "base",
        "polygon",
        "bnb",
        "bsc",
        "avalanche",
        "linea",
        "scroll",
        "mantle",
        "blast",
        "gnosis",
        "celo",
        "unichain",
        "sonic",
        "berachain",
        "hyperliquid",
    ];
    production_markers
        .iter()
        .any(|marker| heading.contains(marker))
}

fn merge_external_scope_assets(
    existing: &mut Vec<ExternalContractScopeAsset>,
    additional: Vec<ExternalContractScopeAsset>,
) {
    let mut seen = existing
        .iter()
        .map(external_scope_asset_key)
        .collect::<BTreeSet<_>>();
    for asset in additional {
        if seen.insert(external_scope_asset_key(&asset)) {
            existing.push(asset);
        }
    }
}

fn external_contract_scope_assets_from_sources(
    sources: &[SourceContent],
) -> Vec<ExternalContractScopeAsset> {
    let mut assets = Vec::new();
    let mut seen = BTreeSet::new();

    for source in sources {
        if !scope_text_source_kind(&source.kind)
            || !matches!(
                source.decision,
                SourceDecision::UsedForScope | SourceDecision::UsedForBoth
            )
        {
            continue;
        }

        let mut sections = likely_scope_sections(&source.content);
        if sections.is_empty() {
            sections = loose_contract_scope_sections(&source.content);
        }
        if sections.is_empty() && content_contains_explorer_contract_url(&source.content) {
            sections.push(source.content.clone());
        }

        for section in sections {
            for mut asset in external_contract_assets_from_text(&section, &source.location) {
                let key = format!(
                    "{}|{}",
                    asset.address.to_ascii_lowercase(),
                    asset.url.trim_end_matches('/')
                );
                if seen.insert(key) {
                    asset.source_location = source.location.clone();
                    assets.push(asset);
                }
            }
        }
    }

    assets.sort_by(|a, b| {
        a.label
            .cmp(&b.label)
            .then_with(|| a.address.cmp(&b.address))
            .then_with(|| a.url.cmp(&b.url))
    });
    assets
}

fn scope_text_source_kind(kind: &ContextSourceKind) -> bool {
    matches!(
        kind,
        ContextSourceKind::LocalReadme
            | ContextSourceKind::LocalMarkdown
            | ContextSourceKind::GithubMarkdown
            | ContextSourceKind::GithubRaw
            | ContextSourceKind::WebMarkdown
            | ContextSourceKind::WebHtml
            | ContextSourceKind::Code4renaBountyPage
            | ContextSourceKind::ImmunefiInformation
            | ContextSourceKind::ImmunefiScope
            | ContextSourceKind::ImmunefiResources
    )
}

fn loose_contract_scope_sections(content: &str) -> Vec<String> {
    let mut sections = Vec::new();
    let mut current = Vec::new();
    let mut in_scope = false;

    for line in content.lines() {
        let lower = line.to_ascii_lowercase();
        if in_scope
            && (is_out_of_scope_heading(&lower)
                || lower.contains("out of scope")
                || lower.contains("known issues")
                || lower.contains("previous audits")
                || lower.contains("bounty criteria"))
        {
            if !current.is_empty() {
                sections.push(current.join("\n"));
                current.clear();
            }
            in_scope = false;
        }

        let scope_marker = !is_out_of_scope_heading(&lower)
            && ((lower.contains("contracts in scope") || lower.contains("smart contracts"))
                || (lower.contains("in scope") && lower.contains("contract")));
        if scope_marker {
            if in_scope && !current.is_empty() {
                sections.push(current.join("\n"));
                current.clear();
            }
            in_scope = true;
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

fn external_contract_assets_from_text(
    text: &str,
    source_location: &str,
) -> Vec<ExternalContractScopeAsset> {
    let mut assets = Vec::new();
    let mut seen_urls = BTreeSet::new();
    let markdown_link_re =
        Regex::new(r#"\[([^\]]+)\]\((https?://[^\s)]+)(?:\s+"[^"]*")?\)"#).unwrap();
    let bare_url_re = explorer_contract_url_regex();

    for line in text.lines() {
        for captures in markdown_link_re.captures_iter(line) {
            let markdown_label = captures.get(1).map(|m| m.as_str()).unwrap_or_default();
            let url = captures.get(2).map(|m| m.as_str()).unwrap_or_default();
            let url = clean_external_contract_url(url);
            if !is_explorer_contract_url(&url) {
                continue;
            }
            let Some(address) = contract_address_from_url(&url) else {
                continue;
            };
            let explorer = explorer_host(&url).unwrap_or_else(|| "unknown-explorer".to_string());
            seen_urls.insert(url.clone());
            let label_source = if markdown_label.eq_ignore_ascii_case(&address) {
                line
            } else {
                markdown_label
            };
            assets.push(ExternalContractScopeAsset {
                label: clean_external_contract_label(label_source, &url, &address),
                url,
                address,
                explorer,
                source_location: source_location.to_string(),
            });
        }
    }

    for line in text.lines() {
        for m in bare_url_re.find_iter(line) {
            let url = clean_external_contract_url(m.as_str());
            if !is_explorer_contract_url(&url) || seen_urls.contains(&url) {
                continue;
            }
            let Some(address) = contract_address_from_url(&url) else {
                continue;
            };
            assets.push(ExternalContractScopeAsset {
                label: clean_external_contract_label(line, &url, &address),
                url: url.clone(),
                address,
                explorer: explorer_host(&url).unwrap_or_else(|| "unknown-explorer".to_string()),
                source_location: source_location.to_string(),
            });
        }
    }

    assets
}

fn explorer_contract_url_regex() -> Regex {
    Regex::new(r#"https?://[A-Za-z0-9_.:-]+/(?:address|token)/0x[a-fA-F0-9]{40}[^\s<>)"'|]*"#)
        .unwrap()
}

fn content_contains_explorer_contract_url(content: &str) -> bool {
    explorer_contract_url_regex()
        .find_iter(content)
        .any(|m| is_explorer_contract_url(m.as_str()))
}

fn is_explorer_contract_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    if !(lower.contains("/address/0x") || lower.contains("/token/0x")) {
        return false;
    }
    if contract_address_from_url(url).is_none() {
        return false;
    }
    let Some(host) = explorer_host(url) else {
        return false;
    };
    let host = host.to_ascii_lowercase();
    host.contains("etherscan")
        || host.contains("basescan")
        || host.contains("arbiscan")
        || host.contains("polygonscan")
        || host.contains("bscscan")
        || host.contains("snowtrace")
        || host.contains("moonscan")
        || host.contains("blockscout")
        || host.ends_with("scan.org")
        || host.ends_with("scan.io")
        || host.ends_with("scan.com")
}

fn explorer_host(url: &str) -> Option<String> {
    let after_scheme = url.split_once("://")?.1;
    let host = after_scheme.split('/').next()?.trim();
    if host.is_empty() {
        None
    } else {
        Some(host.to_string())
    }
}

fn contract_address_from_url(url: &str) -> Option<String> {
    Regex::new(r#"(?i)0x[a-f0-9]{40}"#)
        .unwrap()
        .find(url)
        .map(|m| m.as_str().to_string())
}

fn clean_external_contract_url(raw: &str) -> String {
    normalize_extracted_url(raw)
        .trim()
        .trim_end_matches(|ch: char| matches!(ch, '.' | ',' | ';' | ':' | '`' | '\'' | '"'))
        .to_string()
}

fn clean_external_contract_label(raw: &str, url: &str, address: &str) -> String {
    let without_url = raw.replace(url, " ").replace(address, " ");
    let without_markdown = Regex::new(r#"\[([^\]]+)\]\([^)]+\)"#)
        .unwrap()
        .replace_all(&without_url, "$1");
    let cleaned = without_markdown
        .replace(['|', '`', '*', '[', ']', '(', ')'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_' && ch != '/')
        .to_string();

    if cleaned.is_empty() {
        address.to_string()
    } else {
        cleaned
    }
}

async fn fetch_metadata_and_retry_external_scope_resolution(
    resolution: &mut ExternalContractScopeResolution,
    protocol_root: &Path,
    definitions: &[SolidityDefinition],
) {
    if resolution.unresolved.is_empty() {
        return;
    }

    let Ok(client) = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(HTTP_TIMEOUT_SECS))
        .user_agent("ai-agent-audit-context-generator/0.1")
        .build()
    else {
        resolution
            .warnings
            .push("Could not build HTTP client for explorer metadata fetches".to_string());
        return;
    };

    let mut metadata_by_url = BTreeMap::new();
    let mut fetch_queue = external_metadata_fetch_queue(resolution);
    let mut queued_urls = fetch_queue
        .iter()
        .map(|asset| metadata_url_key(&asset.url))
        .collect::<BTreeSet<_>>();
    let mut fetch_index = 0;
    let mut fetched_count = 0;
    while fetch_index < fetch_queue.len() && fetched_count < MAX_EXTERNAL_CONTRACT_METADATA_FETCHES
    {
        let asset = fetch_queue[fetch_index].clone();
        fetch_index += 1;
        fetched_count += 1;
        match fetch_external_contract_metadata(&client, &asset).await {
            Ok(metadata) => {
                metadata_by_url.insert(metadata_url_key(&asset.url), metadata.clone());
                for implementation_asset in implementation_metadata_assets(&asset, &metadata) {
                    if queued_urls.insert(metadata_url_key(&implementation_asset.url)) {
                        fetch_queue.push(implementation_asset);
                    }
                }
                resolution.metadata.push(metadata);
            }
            Err(err) => resolution.warnings.push(format!(
                "Could not fetch explorer metadata for {} ({}): {err:#}",
                asset.label, asset.url
            )),
        }
    }

    if fetch_index < fetch_queue.len() {
        resolution.warnings.push(format!(
            "Only fetched explorer metadata for {MAX_EXTERNAL_CONTRACT_METADATA_FETCHES} scope assets out of {}, prioritizing unresolved mappings first",
            fetch_queue.len()
        ));
    }

    let mut metadata_resolved_assets = BTreeSet::new();
    let assets = resolution.assets.clone();
    for asset in assets {
        if let Some(metadata) = combined_external_metadata_for_asset(&asset, &metadata_by_url)
            && let Some(record) =
                deterministic_external_asset_resolution(&asset, Some(&metadata), definitions)
        {
            metadata_resolved_assets.insert(external_scope_asset_key(&asset));
            push_external_resolution_record(resolution, protocol_root, record);
        }
    }
    resolution
        .unresolved
        .retain(|asset| !metadata_resolved_assets.contains(&external_scope_asset_key(asset)));
}

fn metadata_url_key(url: &str) -> String {
    url.trim_end_matches('/').to_ascii_lowercase()
}

fn implementation_metadata_assets(
    proxy_asset: &ExternalContractScopeAsset,
    metadata: &ExternalContractMetadata,
) -> Vec<ExternalContractScopeAsset> {
    let mut seen = BTreeSet::new();
    let mut assets = Vec::new();
    for address in &metadata.implementation_addresses {
        let url = explorer_address_url(&proxy_asset.explorer, address);
        if seen.insert(metadata_url_key(&url)) {
            assets.push(ExternalContractScopeAsset {
                label: format!("{} implementation", proxy_asset.label),
                url,
                address: address.clone(),
                explorer: proxy_asset.explorer.clone(),
                source_location: proxy_asset.url.clone(),
            });
        }
    }
    assets
}

fn explorer_address_url(explorer: &str, address: &str) -> String {
    format!(
        "https://{}/address/{}",
        explorer.trim_end_matches('/'),
        address
    )
}

fn combined_external_metadata_for_asset(
    asset: &ExternalContractScopeAsset,
    metadata_by_url: &BTreeMap<String, ExternalContractMetadata>,
) -> Option<ExternalContractMetadata> {
    let mut combined = metadata_by_url.get(&metadata_url_key(&asset.url))?.clone();
    let mut contract_names = combined
        .contract_names
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut source_files = combined
        .source_files
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut implementation_addresses = combined
        .implementation_addresses
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();

    for implementation in &combined.implementation_addresses {
        let implementation_url = explorer_address_url(&asset.explorer, implementation);
        if let Some(metadata) = metadata_by_url.get(&metadata_url_key(&implementation_url)) {
            contract_names.extend(metadata.contract_names.iter().cloned());
            source_files.extend(metadata.source_files.iter().cloned());
            implementation_addresses.extend(metadata.implementation_addresses.iter().cloned());
        }
    }

    combined.contract_names = contract_names.into_iter().collect();
    combined.source_files = source_files.into_iter().collect();
    combined.implementation_addresses = implementation_addresses.into_iter().collect();
    Some(combined)
}

fn external_scope_asset_key(asset: &ExternalContractScopeAsset) -> String {
    format!(
        "{}|{}|{}",
        asset.explorer.to_ascii_lowercase(),
        asset.address.to_ascii_lowercase(),
        asset.url.trim_end_matches('/').to_ascii_lowercase()
    )
}

fn external_metadata_fetch_queue(
    resolution: &ExternalContractScopeResolution,
) -> Vec<ExternalContractScopeAsset> {
    let mut queued_urls = BTreeSet::new();
    let mut queue = Vec::new();

    for asset in &resolution.unresolved {
        if queued_urls.insert(asset.url.trim_end_matches('/').to_string()) {
            queue.push(asset.clone());
        }
    }

    queue
}

async fn fetch_external_contract_metadata(
    client: &reqwest::Client,
    asset: &ExternalContractScopeAsset,
) -> Result<ExternalContractMetadata> {
    let response = client.get(&asset.url).send().await?;
    if !response.status().is_success() {
        anyhow::bail!(
            "Explorer metadata fetch failed with status {}",
            response.status()
        );
    }
    let html = response.text().await?;
    Ok(external_contract_metadata_from_html(&asset.url, &html))
}

fn external_contract_metadata_from_html(url: &str, html: &str) -> ExternalContractMetadata {
    let mut contract_names = BTreeSet::new();
    let mut implementation_addresses = BTreeSet::new();
    let mut source_files = BTreeSet::new();
    let decoded = decode_basic_entities(html);

    if let Some(title) = Regex::new(r#"(?is)<title[^>]*>(.*?)</title>"#)
        .unwrap()
        .captures(&decoded)
        .and_then(|captures| captures.get(1).map(|m| m.as_str()))
    {
        for candidate in title_name_candidates(title) {
            if is_identifier_like(&candidate) {
                contract_names.insert(candidate);
            }
        }
    }

    let contract_name_raw_re =
        Regex::new(r#"(?is)Contract\s+Name\s*:?.{0,300}?([A-Za-z_][A-Za-z0-9_]*)"#).unwrap();
    for captures in contract_name_raw_re.captures_iter(&decoded) {
        if let Some(name) = captures.get(1).map(|m| m.as_str().to_string())
            && probable_contract_identifier(&name)
        {
            contract_names.insert(name);
        }
    }

    let text = html_to_text(html);
    let lines = text.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        let lower = line.to_ascii_lowercase();
        if lower.contains("contract name") {
            if let Some(after_colon) = line.split_once(':').map(|(_, value)| value.trim()) {
                for candidate in identifier_candidates(after_colon) {
                    contract_names.insert(candidate);
                }
            }
            for next_line in lines.iter().skip(index + 1).take(4) {
                for candidate in identifier_candidates(next_line) {
                    contract_names.insert(candidate);
                }
            }
        }
        if lower.contains("implementation") {
            for candidate_line in lines.iter().skip(index).take(3) {
                for address in Regex::new(r#"(?i)0x[a-f0-9]{40}"#)
                    .unwrap()
                    .find_iter(candidate_line)
                {
                    implementation_addresses.insert(address.as_str().to_string());
                }
            }
        }
    }

    let source_file_re =
        Regex::new(r#"(?i)File\s+\d+\s+of\s+\d+\s*:\s*([A-Za-z0-9_./-]+\.sol)"#).unwrap();
    for captures in source_file_re.captures_iter(&decoded) {
        if let Some(path) = captures.get(1).map(|m| m.as_str().to_string()) {
            source_files.insert(path);
        }
    }

    let definition_name_re = Regex::new(
        r#"\b(?:abstract\s+)?(?:contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)\b"#,
    )
    .unwrap();
    for captures in definition_name_re.captures_iter(&decoded).take(40) {
        if let Some(name) = captures.get(1).map(|m| m.as_str().to_string())
            && probable_contract_identifier(&name)
        {
            contract_names.insert(name);
        }
    }

    ExternalContractMetadata {
        url: url.to_string(),
        contract_names: contract_names.into_iter().collect(),
        implementation_addresses: implementation_addresses.into_iter().collect(),
        source_files: source_files.into_iter().collect(),
    }
}

fn title_name_candidates(title: &str) -> Vec<String> {
    let mut candidates = Vec::new();
    let primary = title
        .split('|')
        .next()
        .unwrap_or(title)
        .replace("Address", " ");
    let parts = if primary.contains(':') {
        primary.rsplit(':').take(1).collect::<Vec<_>>()
    } else {
        primary.split(['-', '/']).collect::<Vec<_>>()
    };
    for part in parts {
        for candidate in identifier_candidates(part) {
            candidates.push(candidate);
        }
    }
    candidates
}

fn identifier_candidates(text: &str) -> Vec<String> {
    Regex::new(r#"\b[A-Za-z_][A-Za-z0-9_]{1,80}\b"#)
        .unwrap()
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .filter(|candidate| probable_contract_identifier(candidate))
        .collect()
}

fn is_identifier_like(value: &str) -> bool {
    Regex::new(r#"^[A-Za-z_][A-Za-z0-9_]{1,80}$"#)
        .unwrap()
        .is_match(value)
}

fn probable_contract_identifier(value: &str) -> bool {
    is_identifier_like(value)
        && !metadata_stopword(value)
        && value
            .chars()
            .next()
            .is_some_and(|ch| ch == '_' || ch.is_ascii_uppercase())
}

fn metadata_stopword(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "address"
            | "all"
            | "an"
            | "arbitrary"
            | "as"
            | "be"
            | "by"
            | "calldata"
            | "called"
            | "calls"
            | "can"
            | "capture"
            | "capturing"
            | "class"
            | "code"
            | "col"
            | "commit"
            | "compiler"
            | "contract"
            | "contracts"
            | "current"
            | "delegates"
            | "div"
            | "external"
            | "file"
            | "function"
            | "gas"
            | "group"
            | "h6"
            | "has"
            | "href"
            | "implements"
            | "implementation"
            | "in"
            | "input"
            | "is"
            | "matches"
            | "may"
            | "meant"
            | "msg"
            | "must"
            | "name"
            | "of"
            | "options"
            | "pragma"
            | "provides"
            | "proxy"
            | "questions"
            | "returns"
            | "row"
            | "sender"
            | "signature"
            | "signatures"
            | "since"
            | "slot"
            | "sol"
            | "solidity"
            | "source"
            | "span"
            | "storage"
            | "string"
            | "subsequent"
            | "that"
            | "the"
            | "thereby"
            | "this"
            | "through"
            | "to"
            | "uint256"
            | "until"
            | "usage"
            | "used"
            | "uses"
            | "using"
            | "value"
            | "version"
            | "verified"
            | "which"
            | "with"
            | "etherscan"
            | "basescan"
            | "moonscan"
            | "arbiscan"
            | "polygonscan"
            | "blockscout"
    )
}

fn deterministic_external_asset_resolution(
    asset: &ExternalContractScopeAsset,
    metadata: Option<&ExternalContractMetadata>,
    definitions: &[SolidityDefinition],
) -> Option<ExternalContractResolutionRecord> {
    let mut paths = BTreeSet::new();
    let mut matched_reasons = Vec::new();
    let candidates = external_asset_name_candidates(asset, metadata);

    for candidate in candidates {
        let normalized = normalize_identifier(&candidate);
        if normalized.is_empty() {
            continue;
        }
        let matched = definitions
            .iter()
            .filter(|definition| {
                definition.normalized_name == normalized
                    || definition.normalized_file_stem == normalized
            })
            .collect::<Vec<_>>();
        if matched.is_empty() {
            continue;
        }
        matched_reasons.push(candidate);
        for definition in matched {
            paths.insert(definition.path.clone());
        }
    }

    if paths.is_empty() {
        return None;
    }

    Some(ExternalContractResolutionRecord {
        label: asset.label.clone(),
        address: asset.address.clone(),
        explorer: asset.explorer.clone(),
        url: asset.url.clone(),
        paths: paths.into_iter().collect(),
        confidence: if metadata.is_some() {
            "High".to_string()
        } else {
            "Medium".to_string()
        },
        reason: format!(
            "Matched explorer scope label/metadata candidates to local Solidity definitions: {}",
            matched_reasons.join(", ")
        ),
    })
}

fn external_asset_name_candidates(
    asset: &ExternalContractScopeAsset,
    metadata: Option<&ExternalContractMetadata>,
) -> Vec<String> {
    let mut candidates = Vec::new();
    push_name_candidate_variants(&mut candidates, &asset.label);

    if let Some(metadata) = metadata {
        for name in &metadata.contract_names {
            push_name_candidate_variants(&mut candidates, name);
        }
        for source_file in &metadata.source_files {
            if let Some(stem) = Path::new(source_file)
                .file_stem()
                .and_then(|stem| stem.to_str())
            {
                push_name_candidate_variants(&mut candidates, stem);
            }
        }
    }

    candidates.sort_by_key(|candidate| normalize_identifier(candidate));
    candidates.dedup_by(|a, b| normalize_identifier(a) == normalize_identifier(b));
    candidates
}

fn push_name_candidate_variants(candidates: &mut Vec<String>, raw: &str) {
    let cleaned = clean_name_candidate_for_matching(raw);
    if !useful_external_name_candidate(&cleaned) {
        return;
    }
    candidates.push(cleaned.clone());
    push_plural_name_candidate(candidates, &cleaned);

    for delimiter in [" - ", ":"] {
        if let Some((prefix, suffix)) = cleaned.rsplit_once(delimiter) {
            let prefix = clean_name_candidate_for_matching(prefix);
            if useful_external_name_candidate(&prefix) {
                push_plural_name_candidate(candidates, &prefix);
                candidates.push(prefix);
            }
            let suffix = clean_name_candidate_for_matching(suffix);
            if useful_external_name_candidate(&suffix) {
                push_plural_name_candidate(candidates, &suffix);
                candidates.push(suffix);
            }
        }
    }

    for part in cleaned.split(['/', ',', ';', '|']) {
        let part = clean_name_candidate_for_matching(part);
        if useful_external_name_candidate(&part) {
            push_plural_name_candidate(candidates, &part);
            candidates.push(part);
        }
    }

    for separator in [" and ", " or "] {
        for part in cleaned.split(separator) {
            let part = clean_name_candidate_for_matching(part);
            if useful_external_name_candidate(&part) {
                push_plural_name_candidate(candidates, &part);
                candidates.push(part);
            }
        }
    }
}

fn push_plural_name_candidate(candidates: &mut Vec<String>, candidate: &str) {
    if candidate.ends_with('s') {
        return;
    }
    let plural = format!("{candidate}s");
    if useful_external_name_candidate(&plural) {
        candidates.push(plural);
    }
}

fn useful_external_name_candidate(candidate: &str) -> bool {
    let normalized = normalize_identifier(candidate);
    if normalized.len() < 3 {
        return false;
    }
    if metadata_stopword(candidate) {
        return false;
    }
    identifier_tokens(candidate)
        .into_iter()
        .any(|token| !metadata_stopword(&token))
}

fn clean_name_candidate_for_matching(raw: &str) -> String {
    let no_address = Regex::new(r#"(?i)0x[a-f0-9]{40}"#)
        .unwrap()
        .replace_all(raw, " ");
    let no_url = Regex::new(r#"https?://\S+"#)
        .unwrap()
        .replace_all(&no_address, " ");
    let generic_re = Regex::new(
        r#"(?i)\b(smart|contract|contracts|address|link|proxy|implementation|source|code|base|ethereum|optimism|moonbeam|moonriver|arbitrum|polygon|bsc|mainnet|testnet)\b"#,
    )
    .unwrap();
    generic_re
        .replace_all(&no_url, " ")
        .replace(['`', '*', '[', ']', '(', ')'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_' && ch != '/')
        .to_string()
}

fn push_external_resolution_record(
    resolution: &mut ExternalContractScopeResolution,
    protocol_root: &Path,
    record: ExternalContractResolutionRecord,
) {
    for path in &record.paths {
        resolution.entries.push(ScopeFileEntry {
            path: path.clone(),
            exists: protocol_root.join(path.trim_start_matches("./")).exists(),
            reason: Some(format!(
                "Mapped from external explorer scope link: {} {}",
                record.label, record.address
            )),
        });
    }
    resolution.resolved.push(record);
}

fn build_external_scope_agent() -> Result<AIAgent> {
    AgentFactory::create_openai_agent(
        &AgentConfig::new(None)
            .with_model(OPENAI_MODEL)
            .with_openai_reasoning_effort(OPENAI_REASONING_EFFORT)
            .with_preamble(
                "You resolve external block-explorer bounty scope entries to local Solidity source files. \
                 For Immunefi bounty scope, resolve only smart_contract assets. Ignore websites_and_applications / Web & App assets and repos. \
                 If a smart-contract asset points to a deployment page or wiki instead of a block explorer URL, use only the extracted contract names and explorer addresses provided in the prompt. \
                 Use only the asset labels, explorer metadata, and local Solidity index provided in the prompt. \
                 Return only paths that appear exactly in the local index. Do not invent paths, contracts, or scope. \
                 If a mapping is uncertain, leave that asset unresolved.",
            ),
    )
    .map_err(anyhow::Error::from)
}

async fn codex_resolve_external_scope(
    agent: &AIAgent,
    resolution: &ExternalContractScopeResolution,
    definitions: &[SolidityDefinition],
) -> Result<CodexExternalScopeResolution> {
    let prompt = external_scope_codex_prompt(resolution, definitions)?;
    agent.extract_with_retry(&prompt).await
}

fn external_scope_codex_prompt(
    resolution: &ExternalContractScopeResolution,
    definitions: &[SolidityDefinition],
) -> Result<String> {
    let unresolved = serde_json::to_string_pretty(&resolution.unresolved)?;
    let metadata = serde_json::to_string_pretty(&resolution.metadata)?;
    let local_index = render_solidity_definition_index_for_prompt(definitions);
    Ok(format!(
        r#"
Map unresolved block-explorer contract scope assets to local Solidity files.

Rules:
- Return JSON matching the schema.
- `paths` must be exact paths from the Local Solidity Index.
- Prefer exact contract name and filename matches.
- Split combined labels such as `Unitroller/Comptroller` and map each real in-scope contract when both names exist locally.
- Convert spaced labels such as `Temporal Governor` to Solidity identifier form when appropriate.
- Explorer metadata is stronger evidence than the public display label.
- Do not map token symbols, chain names, proxy words, or generic labels to unrelated files.
- If unsure, put the asset in `unresolved`; do not guess.

Unresolved explorer assets:
{unresolved}

Fetched explorer metadata:
{metadata}

Local Solidity Index:
{local_index}
"#
    ))
}

fn render_solidity_definition_index_for_prompt(definitions: &[SolidityDefinition]) -> String {
    let mut lines = definitions
        .iter()
        .map(|definition| {
            format!(
                "- path={} name={} file_stem={} tokens={}",
                definition.path,
                definition.name,
                definition.file_stem,
                definition.tokens.join(",")
            )
        })
        .collect::<Vec<_>>();
    lines.sort();
    truncate_to_token_limit(lines.join("\n"), 15_000)
}

fn apply_codex_external_scope_resolution(
    resolution: &mut ExternalContractScopeResolution,
    protocol_root: &Path,
    definitions: &[SolidityDefinition],
    model_resolution: CodexExternalScopeResolution,
) {
    let valid_paths = definitions
        .iter()
        .map(|definition| definition.path.clone())
        .collect::<BTreeSet<_>>();
    let mut resolved_addresses = BTreeSet::new();

    for item in model_resolution.resolutions {
        let CodexExternalScopeResolvedAsset {
            label,
            address,
            paths,
            confidence,
            reason,
        } = item;
        let paths = paths
            .into_iter()
            .filter(|path| {
                let valid = valid_paths.contains(path);
                if !valid {
                    resolution.warnings.push(format!(
                        "Codex scope resolver returned non-index path for {} ({}): {}",
                        label, address, path
                    ));
                }
                valid
            })
            .collect::<BTreeSet<_>>();
        if paths.is_empty() {
            continue;
        }

        let Some(asset) = resolution
            .unresolved
            .iter()
            .find(|asset| {
                asset.address.eq_ignore_ascii_case(&address)
                    || normalize_identifier(&asset.label) == normalize_identifier(&label)
            })
            .cloned()
        else {
            resolution.warnings.push(format!(
                "Codex scope resolver returned a mapping for an unknown asset: {} ({})",
                label, address
            ));
            continue;
        };

        resolved_addresses.insert(asset.address.to_ascii_lowercase());
        push_external_resolution_record(
            resolution,
            protocol_root,
            ExternalContractResolutionRecord {
                label: asset.label,
                address: asset.address,
                explorer: asset.explorer,
                url: asset.url,
                paths: paths.into_iter().collect(),
                confidence,
                reason: format!("Codex fallback mapping: {}", reason),
            },
        );
    }

    for item in model_resolution.unresolved {
        resolution.warnings.push(format!(
            "Codex scope resolver left unresolved: {} ({}) - {}",
            item.label, item.address, item.reason
        ));
    }

    resolution
        .unresolved
        .retain(|asset| !resolved_addresses.contains(&asset.address.to_ascii_lowercase()));
}

fn external_scope_resolution_report_path(output_path: &Path) -> PathBuf {
    let stem = output_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("scope");
    output_path.with_file_name(format!("{stem}-resolution.json"))
}

fn write_external_contract_scope_report(
    output_path: &Path,
    resolution: &ExternalContractScopeResolution,
) -> Result<()> {
    let report_path = external_scope_resolution_report_path(output_path);
    let report = ExternalContractScopeResolutionReport {
        assets: resolution.assets.clone(),
        resolved: resolution.resolved.clone(),
        unresolved: resolution.unresolved.clone(),
        metadata: resolution.metadata.clone(),
        warnings: resolution.warnings.clone(),
    };
    fs::write(&report_path, serde_json::to_string_pretty(&report)?)
        .with_context(|| format!("Failed to write {}", report_path.display()))
}

fn bounty_contract_names_from_scope_sections(sources: &[SourceContent]) -> Vec<String> {
    let mut names = Vec::new();
    for source in sources {
        if !matches!(
            source.kind,
            ContextSourceKind::LocalReadme
                | ContextSourceKind::LocalMarkdown
                | ContextSourceKind::GithubMarkdown
                | ContextSourceKind::GithubRaw
                | ContextSourceKind::WebMarkdown
                | ContextSourceKind::WebHtml
                | ContextSourceKind::Code4renaBountyPage
        ) {
            continue;
        }
        for section in likely_scope_sections(&source.content) {
            for line in section.lines() {
                if let Some(name) = bounty_contract_name_from_scope_line(line) {
                    names.push(name);
                }
            }
        }
    }
    names
}

fn bounty_contract_name_from_scope_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty()
        || trimmed.starts_with('#')
        || trimmed.starts_with("---")
        || trimmed.contains("Severity level")
        || trimmed.contains("Likelihood:")
    {
        return None;
    }

    let candidate = if trimmed.contains('|') {
        trimmed
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .find(|cell| {
                !cell.is_empty()
                    && !cell.contains("---")
                    && !cell.eq_ignore_ascii_case("name")
                    && !cell.to_ascii_lowercase().contains("address link")
                    && !cell.eq_ignore_ascii_case("repo")
            })?
            .to_string()
    } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
        trimmed[2..].trim().to_string()
    } else if trimmed.contains("github.com") || trimmed.contains("etherscan.io") {
        trimmed.to_string()
    } else {
        return None;
    };

    clean_contract_name_candidate(&candidate)
}

fn clean_contract_name_candidate(candidate: &str) -> Option<String> {
    let markdown_link = Regex::new(r#"\[([^\]]+)\]\([^)]+\)"#).unwrap();
    let mut value = markdown_link
        .captures(candidate)
        .and_then(|captures| captures.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_else(|| candidate.to_string());

    value = value
        .replace(['`', '*'], "")
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .to_string();

    let contract_name_re = Regex::new(r#"^[A-Za-z_][A-Za-z0-9_]*$"#).unwrap();
    if !contract_name_re.is_match(&value) {
        return None;
    }

    let lower = value.to_ascii_lowercase();
    let reserved = [
        "source",
        "name",
        "repo",
        "scope",
        "severity",
        "likelihood",
        "impact",
        "critical",
        "high",
        "medium",
        "low",
        "risk",
        "payout",
        "contract",
        "contracts",
    ];
    if reserved.contains(&lower.as_str()) {
        return None;
    }

    Some(value)
}

fn solidity_definition_index(
    protocol_root: &Path,
    excluded_folders: &[String],
) -> BTreeMap<String, Vec<String>> {
    let mut index = BTreeMap::<String, Vec<String>>::new();
    for definition in solidity_definitions(protocol_root, excluded_folders) {
        index
            .entry(definition.name)
            .or_default()
            .push(definition.path);
    }
    for paths in index.values_mut() {
        paths.sort_by_key(|path| (path.matches('/').count(), path.len(), path.clone()));
    }
    index
}

fn solidity_definitions(
    protocol_root: &Path,
    excluded_folders: &[String],
) -> Vec<SolidityDefinition> {
    let definition_re = Regex::new(
        r#"\b(?:abstract\s+)?(?:contract|library|interface)\s+([A-Za-z_][A-Za-z0-9_]*)\b"#,
    )
    .unwrap();
    let mut definitions = Vec::new();

    for entry in WalkDir::new(protocol_root)
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
        let Ok(relative_path) = path.strip_prefix(protocol_root) else {
            continue;
        };
        let relative = relative_path.to_string_lossy().replace('\\', "/");
        if bounty_oos_excludes(&relative, excluded_folders)
            || default_generated_scope_path_excludes(&relative)
            || default_dependency_scope_path_excludes(&relative)
            || default_non_runtime_scope_path_excludes(&relative)
        {
            continue;
        }

        let Ok(content) = fs::read_to_string(path) else {
            continue;
        };
        let file_stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default()
            .to_string();
        for captures in definition_re.captures_iter(&content) {
            let Some(name) = captures.get(1).map(|m| m.as_str().to_string()) else {
                continue;
            };
            let normalized_name = normalize_identifier(&name);
            let normalized_file_stem = normalize_identifier(&file_stem);
            let mut tokens = identifier_tokens(&name);
            tokens.extend(identifier_tokens(&file_stem));
            tokens.sort();
            tokens.dedup();
            definitions.push(SolidityDefinition {
                name,
                path: format!("./{}", relative.trim_start_matches("./")),
                file_stem: file_stem.clone(),
                normalized_name,
                normalized_file_stem,
                tokens,
            });
        }
    }

    definitions.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| a.file_stem.cmp(&b.file_stem))
    });
    definitions
}

fn normalize_identifier(raw: &str) -> String {
    raw.chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}

fn identifier_tokens(raw: &str) -> Vec<String> {
    let with_boundaries = Regex::new(r#"([a-z0-9])([A-Z])"#)
        .unwrap()
        .replace_all(raw, "$1 $2");
    Regex::new(r#"[A-Za-z0-9]+"#)
        .unwrap()
        .find_iter(&with_boundaries)
        .map(|m| m.as_str().to_ascii_lowercase())
        .filter(|token| token.len() > 1 && !metadata_stopword(token))
        .collect()
}

fn bounty_declared_oos_folders(sources: &[SourceContent], cli: &Cli) -> Vec<String> {
    let mut folders = cli
        .exclude_folders
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|folder| normalize_folder_for_exclusion(&folder))
        .filter(|folder| !folder.is_empty())
        .collect::<Vec<_>>();

    let github_tree_re =
        Regex::new(r#"github\.com/[^)\s]+/tree/[^/\s)]+/([A-Za-z0-9_./-]+)"#).unwrap();
    let inline_code_re = Regex::new(r#"`([^`]+)`"#).unwrap();

    for source in sources {
        for section in likely_out_of_scope_sections(&source.content) {
            for captures in github_tree_re.captures_iter(&section) {
                if let Some(path) = captures.get(1) {
                    folders.push(normalize_folder_for_exclusion(path.as_str()));
                }
            }
            for captures in inline_code_re.captures_iter(&section) {
                if let Some(path) = captures.get(1) {
                    let normalized = normalize_folder_for_exclusion(path.as_str());
                    if !normalized.ends_with(".sol") {
                        folders.push(normalized);
                    }
                }
            }
        }
    }

    folders.retain(|folder| !folder.is_empty());
    folders.sort();
    folders.dedup();
    folders
}

fn normalize_folder_for_exclusion(raw: &str) -> String {
    raw.trim()
        .trim_matches('`')
        .trim_matches('/')
        .trim_start_matches("./")
        .trim_end_matches('/')
        .to_string()
}

fn bounty_oos_excludes(relative: &str, excluded_folders: &[String]) -> bool {
    let relative = relative.trim_start_matches("./");
    excluded_folders.iter().any(|folder| {
        let folder = folder.trim().trim_start_matches("./").trim_end_matches('/');
        !folder.is_empty() && (relative == folder || relative.starts_with(&format!("{folder}/")))
    })
}

fn default_generated_scope_path_excludes(relative: &str) -> bool {
    path_has_any_component(
        relative,
        &[
            ".git",
            "artifacts",
            "broadcast",
            "build",
            "cache",
            "coverage",
            "dist",
            "node_modules",
            "out",
        ],
    )
}

fn default_dependency_scope_path_excludes(relative: &str) -> bool {
    let normalized = relative
        .trim()
        .trim_start_matches("./")
        .trim_start_matches('/')
        .replace('\\', "/")
        .to_ascii_lowercase();
    let parts = normalized.split('/').collect::<Vec<_>>();
    parts.windows(2).any(|window| {
        window[0] == "lib"
            && matches!(
                window[1],
                "forge-std"
                    | "ds-test"
                    | "openzeppelin-contracts"
                    | "openzeppelin-contracts-upgradeable"
                    | "solady"
                    | "solmate"
                    | "prb-test"
                    | "erc4626-tests"
                    | "halmos-cheatcodes"
            )
    })
}

fn default_non_runtime_scope_path_excludes(relative: &str) -> bool {
    path_has_any_component(
        relative,
        &["mock", "mocks", "script", "scripts", "test", "tests"],
    )
}

fn path_has_any_component(relative: &str, components: &[&str]) -> bool {
    let normalized = relative
        .trim()
        .trim_start_matches("./")
        .trim_start_matches('/')
        .replace('\\', "/")
        .to_ascii_lowercase();
    normalized
        .split('/')
        .any(|part| components.iter().any(|component| part == *component))
}

fn fallback_scope_from_code_folders(cli: &Cli, protocol_root: &Path) -> Vec<ScopeFileEntry> {
    let mut entries = Vec::new();
    let code_folders = scope_fallback_code_folders(cli);

    for folder in &code_folders {
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

fn scope_fallback_code_folders(cli: &Cli) -> Vec<String> {
    if scope_bounty_platform(cli).is_some()
        && cli.code_folders == vec!["src".to_string()]
        && !cli.repo_tree_paths.is_empty()
    {
        let mut folders = cli
            .repo_tree_paths
            .iter()
            .map(|path| {
                path.trim()
                    .trim_matches('/')
                    .trim_start_matches("./")
                    .trim_end_matches('/')
                    .to_string()
            })
            .filter(|path| !path.is_empty())
            .collect::<Vec<_>>();
        folders.sort();
        folders.dedup();
        folders
    } else {
        cli.code_folders.clone()
    }
}

fn scope_fallback_excludes(relative: &str, cli: &Cli) -> bool {
    if default_generated_scope_path_excludes(relative)
        || default_dependency_scope_path_excludes(relative)
        || default_non_runtime_scope_path_excludes(relative)
    {
        return true;
    }

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
            in_scope = !is_out_of_scope_heading(&lower)
                && (lower.contains("scope")
                    || lower.contains("files in")
                    || lower.contains("contracts in"));
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

fn likely_out_of_scope_sections(content: &str) -> Vec<String> {
    let mut sections = Vec::new();
    let mut current = Vec::new();
    let mut in_out_of_scope = false;

    for line in content.lines() {
        let heading = line.trim_start().starts_with('#');
        if heading {
            let lower = line.to_ascii_lowercase();
            if in_out_of_scope && !current.is_empty() {
                sections.push(current.join("\n"));
                current.clear();
            }
            in_out_of_scope = is_out_of_scope_heading(&lower)
                || lower.contains("known issues")
                || lower.contains("previous audits")
                || lower.contains("specific types of issues");
        }
        if in_out_of_scope {
            current.push(line.to_string());
        }
    }
    if in_out_of_scope && !current.is_empty() {
        sections.push(current.join("\n"));
    }
    sections
}

fn is_out_of_scope_heading(lower_heading: &str) -> bool {
    lower_heading.contains("out-of-scope")
        || lower_heading.contains("out of scope")
        || lower_heading.contains("out_of_scope")
}

fn normalize_scope_path(raw: &str, protocol_root: &Path, reason: Option<String>) -> ScopeFileEntry {
    let mut path = raw
        .trim()
        .trim_matches('`')
        .trim_matches('|')
        .trim()
        .trim_start_matches('/')
        .to_string();
    if let Some(github_path) = github_blob_path_from_scope_candidate(&path) {
        path = github_path;
    }
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

fn github_blob_path_from_scope_candidate(raw: &str) -> Option<String> {
    let clean = raw
        .split('#')
        .next()
        .unwrap_or(raw)
        .split('?')
        .next()
        .unwrap_or(raw)
        .trim_end_matches('/');
    let marker = "/blob/";
    let marker_index = clean.find(marker)?;
    let after_blob = &clean[marker_index + marker.len()..];
    let mut parts = after_blob.splitn(2, '/');
    let _branch = parts.next()?;
    let path = parts.next()?.trim_start_matches('/');
    if path.ends_with(".sol") {
        Some(path.to_string())
    } else {
        None
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
- For Code4rena bounty sources, explicitly preserve the global bounty out-of-scope scenarios and Critical/High eligibility criteria from the bounty criteria page.
- For Immunefi bounty sources, explicitly preserve assets in scope, impacts in scope, out-of-scope rules, prohibited activities, PoC requirements, primacy rules, and prior-audit/known-issue exclusions.
- For V12/prior findings, do not copy full reports. Summarize finding titles, affected areas, and audit implications under known issues/out-of-scope so discovery does not resubmit them.
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
- Treat scoped-code documentation extracts as primary evidence for contracts, inheritance, NatSpec, local imports, events, errors, and public/external function behavior.
- For Immunefi bounty sources, treat Resources documentation links and the program overview as primary protocol documentation.
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

fn render_validation_sidecar(
    artifact_prefix: &str,
    scope_file_list: &ScopeFileList,
    sources: &[SourceContent],
) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {artifact_prefix} Validation Context\n\n"));
    out.push_str("Use this file only after discovery, during three-step validation and PoC planning. Do not feed this file into discovery prompts.\n\n");
    out.push_str("## Scoped Files\n\n");
    for entry in &scope_file_list.files {
        out.push_str(&format!(
            "- {}{}\n",
            entry.path,
            if entry.exists {
                ""
            } else {
                " (not found locally)"
            }
        ));
    }

    let v12_sources = sources
        .iter()
        .filter(|source| matches!(source.kind, ContextSourceKind::V12Report))
        .collect::<Vec<_>>();
    out.push_str("\n## Known-Issue Sources\n\n");
    if v12_sources.is_empty() {
        out.push_str("- No V12 findings detected in collected context.\n");
    } else {
        for source in v12_sources {
            out.push_str(&format!("- V12/prior findings: {}\n", source.location));
        }
    }

    let known_issue_sources = sources
        .iter()
        .filter(|source| {
            matches!(
                source.kind,
                ContextSourceKind::LocalKnownIssues | ContextSourceKind::Code4renaContestRepo
            ) && matches!(
                source.decision,
                SourceDecision::UsedForScope | SourceDecision::UsedForBoth
            )
        })
        .collect::<Vec<_>>();
    for source in known_issue_sources {
        out.push_str(&format!(
            "- Scope/known-issue context: {}\n",
            source.location
        ));
    }

    out.push_str("\n## Validation Guidance\n\n");
    out.push_str("- Validate candidates against generated scope, docs, V12/prior known issues, and local source code.\n");
    out.push_str("- Use repository tests and user-supplied PoC config (`poc_instructions`, `poc_template`, `test_folder`) when present.\n");
    out.push_str("- Do not require PoC mechanics during discovery; apply them only when validating surviving candidate findings.\n");
    out
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

Preserve required sections, scope, known issues, Code4rena/Immunefi bounty criteria/OOS rules, summarized V12/prior findings, invariants, trusted roles, and security-relevant protocol mechanics. Remove repetition, copied report prose, duplicated source material, generic docs, and low-value background first. Prefer concise tables and bullets.

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
                | LinkClassification::BountyRules
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
    let with_anchor_links = preserve_html_anchor_links(&without_scripts);
    let with_newlines = Regex::new(r"(?i)</?(p|br|div|li|h[1-6]|tr|table|section|article)[^>]*>")
        .unwrap()
        .replace_all(&with_anchor_links, "\n");
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

fn preserve_html_anchor_links(html: &str) -> String {
    Regex::new(r#"(?is)<a\b[^>]*href=["']([^"']+)["'][^>]*>(.*?)</a>"#)
        .unwrap()
        .replace_all(html, |captures: &regex::Captures<'_>| {
            let href = decode_basic_entities(captures.get(1).map(|m| m.as_str()).unwrap_or(""));
            let label_html = captures.get(2).map(|m| m.as_str()).unwrap_or("");
            let label = decode_basic_entities(
                &Regex::new(r"(?s)<[^>]+>")
                    .unwrap()
                    .replace_all(label_html, " "),
            )
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

            if href.is_empty() {
                label
            } else if label.is_empty() || label == href {
                href
            } else {
                format!("{label} ({href})")
            }
        })
        .into_owned()
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
    use crate::cli_args::parse::ResolvedRepoConfig;
    use crate::prepare_code::code4rena_bounty::{Code4renaBountyData, Code4renaScopeAsset};
    use crate::prepare_code::immunefi::{
        ImmunefiAsset, ImmunefiBountyData, ImmunefiBountyUrls, ImmunefiTabCapture, ImmunefiTabKind,
    };

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
    fn local_protocol_doc_discovery_includes_relevant_root_and_docs_files() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("README.md"), "# readme").unwrap();
        fs::write(tmp.path().join("ARCHITECTURE.md"), "# architecture").unwrap();
        fs::write(tmp.path().join("CHANGELOG.md"), "# changes").unwrap();
        fs::create_dir_all(tmp.path().join("docs")).unwrap();
        fs::write(tmp.path().join("docs/protocol.md"), "# protocol docs").unwrap();
        fs::create_dir_all(tmp.path().join("audits")).unwrap();
        fs::write(tmp.path().join("audits/report.txt"), "audit").unwrap();

        let rels = discover_local_protocol_doc_files(tmp.path())
            .into_iter()
            .map(|path| {
                path.strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect::<Vec<_>>();

        assert!(rels.contains(&"ARCHITECTURE.md".to_string()));
        assert!(rels.contains(&"docs/protocol.md".to_string()));
        assert!(rels.contains(&"audits/report.txt".to_string()));
        assert!(!rels.contains(&"README.md".to_string()));
        assert!(!rels.contains(&"CHANGELOG.md".to_string()));
    }

    #[test]
    fn scoped_code_documentation_extracts_natspec_imports_and_declarations() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("contracts")).unwrap();
        fs::write(
            tmp.path().join("contracts/Vault.sol"),
            r#"
import {Math} from "../src/Math.sol";

/// @notice Stores user deposits.
contract Vault is Ownable {
    /// @notice Total managed assets.
    uint256 public totalAssets;

    /// @notice Deposit assets for a receiver.
    function deposit(uint256 assets, address receiver) external returns (uint256 shares) {}

    /// @notice Emitted after a deposit.
    event Deposit(address indexed receiver, uint256 assets);
}
"#,
        )
        .unwrap();
        let scope = ScopeFileList {
            files: vec![ScopeFileEntry {
                path: "./contracts/Vault.sol".to_string(),
                exists: true,
                reason: None,
            }],
            source: ScopeFileSource::CopiedScopeTxt,
            warnings: Vec::new(),
        };

        let sources = collect_scoped_code_documentation_sources(tmp.path(), &scope);

        assert_eq!(sources.len(), 1);
        assert!(matches!(sources[0].kind, ContextSourceKind::ScopedCodeDocs));
        assert!(sources[0].content.contains("import {Math}"));
        assert!(sources[0].content.contains("@notice Stores user deposits"));
        assert!(sources[0].content.contains("contract Vault is Ownable"));
        assert!(sources[0].content.contains("function deposit"));
        assert!(sources[0].content.contains("event Deposit"));
    }

    #[test]
    fn code4rena_contest_url_normalizes_to_github_repo() {
        assert_eq!(
            code4rena_contest_url_to_github_repo(
                "https://code4rena.com/audits/2025-11-megapot/submissions?page=1"
            )
            .unwrap(),
            "https://github.com/code-423n4/2025-11-megapot"
        );
        assert_eq!(
            code4rena_contest_url_to_github_repo(
                "https://github.com/code-423n4/2025-11-megapot.git"
            )
            .unwrap(),
            "https://github.com/code-423n4/2025-11-megapot"
        );
    }

    #[test]
    fn explorer_network_inference_covers_major_mainnets_and_testnets() {
        assert_eq!(
            infer_explorer_network(
                "https://etherscan.io/address/0x1111111111111111111111111111111111111111",
            )
            .unwrap()
            .id,
            "ethereum-mainnet"
        );
        assert_eq!(
            infer_explorer_network(
                "https://basescan.org/address/0x1111111111111111111111111111111111111111",
            )
            .unwrap()
            .id,
            "base-mainnet"
        );
        assert_eq!(
            infer_explorer_network(
                "https://sepolia.arbiscan.io/address/0x1111111111111111111111111111111111111111",
            )
            .unwrap()
            .id,
            "arbitrum-sepolia"
        );
        assert_eq!(
            infer_explorer_network(
                "https://berascan.com/address/0x1111111111111111111111111111111111111111",
            )
            .unwrap()
            .id,
            "berachain-mainnet"
        );
    }

    #[test]
    fn immunefi_poc_runtime_extracts_deployed_assets_and_rpc_env_names() {
        let bounty = ImmunefiBountyData {
            input_url: "https://immunefi.com/bug-bounty/example/information/".to_string(),
            urls: ImmunefiBountyUrls {
                base: "https://immunefi.com/bug-bounty/example".to_string(),
                information: "https://immunefi.com/bug-bounty/example/information/".to_string(),
                scope: "https://immunefi.com/bug-bounty/example/scope/".to_string(),
                resources: "https://immunefi.com/bug-bounty/example/resources/".to_string(),
            },
            project: "Example".to_string(),
            slug: "example".to_string(),
            description: None,
            website_url: None,
            github_url: None,
            max_bounty: None,
            launch_date: None,
            updated_date: None,
            proof_of_concept_type: None,
            primacy: None,
            severity_system: None,
            rewards_token: None,
            rewards_token_network: None,
            codebases: Vec::new(),
            documentations: Vec::new(),
            audits: Vec::new(),
            assets: vec![
                ImmunefiAsset {
                    url: "https://basescan.org/address/0xfbb21d0380bee3312b33c4353c8936a0f13ef26c"
                        .to_string(),
                    asset_type: Some("smart_contract".to_string()),
                    description: Some("Vault".to_string()),
                    is_primacy_of_impact: false,
                },
                ImmunefiAsset {
                    url: "https://immunefi.com".to_string(),
                    asset_type: Some("smart_contract".to_string()),
                    description: Some("Primacy of Impact".to_string()),
                    is_primacy_of_impact: true,
                },
            ],
            impacts: Vec::new(),
            rewards: Vec::new(),
            default_out_of_scope_smart_contract: None,
            default_out_of_scope_general: None,
            prohibited_activities: None,
            custom_out_of_scope: None,
            known_issues: Vec::new(),
            tabs: Vec::new(),
        };
        let mut rpc_env = BTreeMap::new();
        rpc_env.insert(
            "base-mainnet".to_string(),
            "BASE_RPC_URL_FOR_POC_RUNTIME_TEST".to_string(),
        );
        let runtime = build_immunefi_poc_runtime(
            &bounty,
            &PocConfig {
                allow_fork: true,
                prefer_fork: true,
                rpc_env,
            },
        );

        assert_eq!(runtime.assets.len(), 1);
        assert_eq!(runtime.assets[0].network, "base-mainnet");
        assert_eq!(
            runtime.assets[0].rpc_env_var,
            "BASE_RPC_URL_FOR_POC_RUNTIME_TEST"
        );
        assert_eq!(runtime.networks.len(), 1);
        let rendered = render_immunefi_poc_runtime_markdown(&runtime);
        assert!(rendered.contains("BASE_RPC_URL_FOR_POC_RUNTIME_TEST"));
        assert!(rendered.contains("Prefer a mainnet fork PoC"));
    }

    #[test]
    fn immunefi_worker_docs_do_not_include_rendered_page_noise() {
        let noisy_html = r#"
<html><body>
Open menu
Blog (https://immunefi.com/blog/)
Platform Bug Bounty Programs PR Reviews Audits
Security Researchers Join Immunefi Find bugs. Get paid.
Token Foundation Docs Buy IMU
<main>Real clause</main>
</body></html>
"#;
        let bounty = ImmunefiBountyData {
            input_url: "https://immunefi.com/bug-bounty/example/information/".to_string(),
            urls: ImmunefiBountyUrls {
                base: "https://immunefi.com/bug-bounty/example".to_string(),
                information: "https://immunefi.com/bug-bounty/example/information/".to_string(),
                scope: "https://immunefi.com/bug-bounty/example/scope/".to_string(),
                resources: "https://immunefi.com/bug-bounty/example/resources/".to_string(),
            },
            project: "Example".to_string(),
            slug: "example".to_string(),
            description: None,
            website_url: None,
            github_url: None,
            max_bounty: None,
            launch_date: None,
            updated_date: None,
            proof_of_concept_type: Some("required".to_string()),
            primacy: Some("primacy_of_impact".to_string()),
            severity_system: None,
            rewards_token: None,
            rewards_token_network: None,
            codebases: Vec::new(),
            documentations: Vec::new(),
            audits: Vec::new(),
            assets: Vec::new(),
            impacts: Vec::new(),
            rewards: Vec::new(),
            default_out_of_scope_smart_contract: Some(
                "- First excluded item\\\n- Second excluded item\\".to_string(),
            ),
            default_out_of_scope_general: None,
            prohibited_activities: None,
            custom_out_of_scope: None,
            known_issues: Vec::new(),
            tabs: vec![ImmunefiTabCapture {
                kind: ImmunefiTabKind::Information,
                url: "https://immunefi.com/bug-bounty/example/information/".to_string(),
                html: noisy_html.to_string(),
            }],
        };

        let rules = render_immunefi_bounty_rules_markdown(&bounty);
        let rubric = render_immunefi_severity_rubric_markdown(&bounty);

        for rendered in [&rules, &rubric] {
            assert!(!rendered.contains("Rendered Bounty Page Text Excerpts"));
            assert!(!rendered.contains("Open menu"));
            assert!(!rendered.contains("Join Immunefi"));
            assert!(!rendered.contains("https://immunefi.com/blog/"));
        }
        assert!(rules.contains("- First excluded item\n- Second excluded item"));
        assert!(!rules.contains("item\\"));
        assert!(rubric.contains("## Link Handling"));
    }

    #[test]
    fn immunefi_context_generates_even_with_complete_manual_context() {
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/example/information/"
custom_doc: "docs.md"
audit_scope: "scope.md"
scoped_files: "scope.txt"
"#,
        )
        .unwrap();

        assert!(should_generate_context(&cli));
    }

    #[test]
    fn context_preamble_is_audit_type_specific() {
        let code4rena_bounty: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
audit_type: "Code4renaBounty"
"#,
        )
        .unwrap();
        let immunefi_bounty: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/example/information/"
"#,
        )
        .unwrap();
        let competition: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
audit_type: "Code4rena"
"#,
        )
        .unwrap();

        let c4_bounty = context_generation_preamble(&code4rena_bounty);
        assert!(c4_bounty.contains("configured Code4rena bounty page"));
        assert!(c4_bounty.contains("There is no V12 stage"));
        assert!(!c4_bounty.contains("Immunefi"));

        let immunefi = context_generation_preamble(&immunefi_bounty);
        assert!(immunefi.contains("Immunefi smart-contract bug bounties"));
        assert!(immunefi.contains("Ignore Web & App"));
        assert!(!immunefi.contains("Code4rena"));

        let c4_competition = context_generation_preamble(&competition);
        assert!(c4_competition.contains("V12"));
        assert!(c4_competition.contains("competition-only"));
    }

    #[tokio::test]
    async fn immunefi_cached_context_reuses_artifacts_without_fetching() {
        let tmp = tempfile::tempdir().unwrap();
        let output_root = tmp.path().join("audit-docs");
        let artifact_dir = output_root.join("protocol");
        fs::create_dir_all(&artifact_dir).unwrap();
        for name in [
            "protocol-scope.txt",
            "protocol-scope.md",
            "protocol-docs.md",
            "protocol-validation.md",
            "protocol-immunefi-bounty-rules.md",
            "protocol-immunefi-severity-rubric.md",
            "protocol-immunefi-poc-runtime.md",
        ] {
            fs::write(artifact_dir.join(name), "# cached\n").unwrap();
        }
        let mut cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "not-an-immunefi-url"
"#,
        )
        .unwrap();
        cli.context = Some(ContextConfig {
            output_dir: output_root.to_string_lossy().to_string(),
            force_regenerate: false,
            ..Default::default()
        });

        let context = generate_audit_context(
            &cli,
            tmp.path(),
            tmp.path(),
            "protocol",
            "0123456789abcdef0123456789abcdef01234567",
        )
        .await
        .unwrap();

        assert!(!context.regenerated);
        assert_eq!(context.extra_docs.len(), 3);
        assert!(
            context
                .extra_docs
                .iter()
                .any(|path| path.ends_with("protocol-immunefi-severity-rubric.md"))
        );
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

        let list = generate_scope_txt(&cli, tmp.path(), &output_path, &sources, None, None)
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

        let list = generate_scope_txt(&cli, tmp.path(), &output_path, &sources, None, None)
            .await
            .unwrap();

        assert!(matches!(list.source, ScopeFileSource::FallbackCodeFolders));
        assert_eq!(
            fs::read_to_string(output_path).unwrap(),
            "./contracts/Token.sol\n./contracts/core/Vault.sol\n"
        );
    }

    #[tokio::test]
    async fn code4rena_repo_level_scope_can_fall_back_to_code_folders() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("contracts")).unwrap();
        fs::write(tmp.path().join("contracts/Vault.sol"), "").unwrap();
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
audit_type: "Code4renaBounty"
code_folders:
  - "contracts"
"#,
        )
        .unwrap();
        let bounty = Code4renaBountyData {
            input_url: "https://code4rena.com/bounties/example".to_string(),
            url: "https://code4rena.com/bounties/example".to_string(),
            slug: "example".to_string(),
            project: "Example".to_string(),
            summary: None,
            max_bounty: None,
            rewards: Vec::new(),
            links: Vec::new(),
            scope_assets: vec![Code4renaScopeAsset {
                label: "Smart contract repo".to_string(),
                url: "https://github.com/example/protocol".to_string(),
                source_section: "Smart Contracts in Scope".to_string(),
            }],
            severity_section: None,
            rules_section: None,
            out_of_scope_section: None,
            known_issues_section: None,
            previous_audits_section: None,
            full_text: String::new(),
        };
        let output_path = tmp.path().join("generated-scope.txt");

        let list = generate_scope_txt(&cli, tmp.path(), &output_path, &[], Some(&bounty), None)
            .await
            .unwrap();

        assert!(matches!(list.source, ScopeFileSource::FallbackCodeFolders));
        assert_eq!(
            fs::read_to_string(output_path).unwrap(),
            "./contracts/Vault.sol\n"
        );
    }

    #[tokio::test]
    async fn code4rena_blob_scope_uses_full_branch_hint_for_path() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(tmp.path().join("src/Foo.sol"), "").unwrap();
        let mut cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
audit_type: "Code4renaBounty"
code_folders:
  - "src"
"#,
        )
        .unwrap();
        cli.repo_branch = Some("release/v2".to_string());
        let bounty = Code4renaBountyData {
            input_url: "https://code4rena.com/bounties/example".to_string(),
            url: "https://code4rena.com/bounties/example".to_string(),
            slug: "example".to_string(),
            project: "Example".to_string(),
            summary: None,
            max_bounty: None,
            rewards: Vec::new(),
            links: Vec::new(),
            scope_assets: vec![Code4renaScopeAsset {
                label: "Foo".to_string(),
                url: "https://github.com/example/protocol/blob/release/v2/src/Foo.sol".to_string(),
                source_section: "Smart Contracts in Scope".to_string(),
            }],
            severity_section: None,
            rules_section: None,
            out_of_scope_section: None,
            known_issues_section: None,
            previous_audits_section: None,
            full_text: String::new(),
        };
        let output_path = tmp.path().join("generated-scope.txt");

        let list = generate_scope_txt(&cli, tmp.path(), &output_path, &[], Some(&bounty), None)
            .await
            .unwrap();

        assert!(matches!(
            list.source,
            ScopeFileSource::ExtractedFromBountyScope
        ));
        assert_eq!(fs::read_to_string(output_path).unwrap(), "./src/Foo.sol\n");
    }

    #[tokio::test]
    async fn code4rena_polyrepo_blob_scope_is_prefixed_with_member_repo_slug() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("alpha-repo-a/src")).unwrap();
        fs::create_dir_all(tmp.path().join("beta-repo-b/contracts")).unwrap();
        fs::write(tmp.path().join("alpha-repo-a/src/Foo.sol"), "").unwrap();
        fs::write(tmp.path().join("beta-repo-b/contracts/Bar.sol"), "").unwrap();
        let mut cli: Cli = serde_yaml::from_str(
            r#"
audit_type: "Code4renaBounty"
code4rena_bounty: "https://code4rena.com/bounties/example"
code_folders:
  - "src"
"#,
        )
        .unwrap();
        cli.resolved_repos = vec![
            ResolvedRepoConfig {
                repo_url: "https://github.com/alpha/repo-a".to_string(),
                branch: Some("main".to_string()),
                tree_paths: Vec::new(),
            },
            ResolvedRepoConfig {
                repo_url: "https://github.com/beta/repo-b".to_string(),
                branch: Some("main".to_string()),
                tree_paths: Vec::new(),
            },
        ];
        let bounty = Code4renaBountyData {
            input_url: "https://code4rena.com/bounties/example".to_string(),
            url: "https://code4rena.com/bounties/example".to_string(),
            slug: "example".to_string(),
            project: "Example".to_string(),
            summary: None,
            max_bounty: None,
            rewards: Vec::new(),
            links: Vec::new(),
            scope_assets: vec![
                Code4renaScopeAsset {
                    label: "Foo".to_string(),
                    url: "https://github.com/alpha/repo-a/blob/main/src/Foo.sol".to_string(),
                    source_section: "Smart Contracts in Scope".to_string(),
                },
                Code4renaScopeAsset {
                    label: "Bar".to_string(),
                    url: "https://github.com/beta/repo-b/blob/main/contracts/Bar.sol".to_string(),
                    source_section: "Smart Contracts in Scope".to_string(),
                },
            ],
            severity_section: None,
            rules_section: None,
            out_of_scope_section: None,
            known_issues_section: None,
            previous_audits_section: None,
            full_text: String::new(),
        };
        let output_path = tmp.path().join("generated-scope.txt");

        let list = generate_scope_txt(&cli, tmp.path(), &output_path, &[], Some(&bounty), None)
            .await
            .unwrap();

        assert!(matches!(
            list.source,
            ScopeFileSource::ExtractedFromBountyScope
        ));
        assert_eq!(
            fs::read_to_string(output_path).unwrap(),
            "./alpha-repo-a/src/Foo.sol\n./beta-repo-b/contracts/Bar.sol\n"
        );
        assert!(list.files.iter().all(|entry| entry.exists));
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
        assert_eq!(
            classify_link("https://code4rena.com/bounties/moonwell", "Moonwell"),
            LinkClassification::Scope
        );
        assert_eq!(
            link_skip_reason(
                "https://code4rena.com/bounties/moonwell",
                &LinkClassification::Scope,
            ),
            None
        );
    }

    #[test]
    fn code4rena_bounty_docs_are_required_scope_sources() {
        assert_eq!(
            classify_link(CODE4RENA_BOUNTY_GUIDE_URL, "Code4rena bounty guide"),
            LinkClassification::BountyRules
        );
        assert_eq!(
            classify_link(CODE4RENA_BOUNTY_CRITERIA_URL, "Code4rena bounty criteria"),
            LinkClassification::BountyRules
        );
        assert_eq!(
            link_skip_reason(
                CODE4RENA_BOUNTY_CRITERIA_URL,
                &LinkClassification::BountyRules
            ),
            None
        );
        assert!(!remote_link_counts_against_fetch_budget(
            &LinkClassification::BountyRules
        ));
        assert!(remote_link_counts_against_fetch_budget(
            &LinkClassification::Documentation
        ));
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
            None
        );
        assert_eq!(
            link_skip_reason(
                "https://example.com/audit.zip",
                &LinkClassification::PriorAudit,
            ),
            Some("Skipped binary link; only text, HTML, and best-effort PDF sources are fetched")
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
    fn scope_sections_ignore_out_of_scope_headings() {
        let sections = likely_scope_sections(
            r#"
# Smart Contracts in Scope
- `src/InScope.sol`

## Out-of-Scope
- `src/OutOfScope.sol`
"#,
        );

        let joined = sections.join("\n");
        assert!(joined.contains("src/InScope.sol"));
        assert!(!joined.contains("src/OutOfScope.sol"));
    }

    #[test]
    fn normalize_scope_path_extracts_github_blob_paths() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src/access")).unwrap();
        fs::write(tmp.path().join("src/access/LegionBouncer.sol"), "").unwrap();

        let entry = normalize_scope_path(
            "https://github.com/Legion-Team/legion-protocol-contracts/blob/master/src/access/LegionBouncer.sol",
            tmp.path(),
            None,
        );

        assert_eq!(entry.path, "./src/access/LegionBouncer.sol");
        assert!(entry.exists);
    }

    #[test]
    fn bounty_contract_names_map_to_local_solidity_definitions() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src/access")).unwrap();
        fs::create_dir_all(tmp.path().join("src/mocks")).unwrap();
        fs::write(
            tmp.path().join("src/access/LegionBouncer.sol"),
            "contract LegionBouncer {}",
        )
        .unwrap();
        fs::write(
            tmp.path().join("src/mocks/MockSale.sol"),
            "contract MockSale {}",
        )
        .unwrap();
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
audit_type: "Code4renaBounty"
"#,
        )
        .unwrap();
        let sources = vec![SourceContent {
            id: "source-1".to_string(),
            kind: ContextSourceKind::LocalReadme,
            location: "README.md".to_string(),
            title: None,
            content: r#"
# Smart Contracts in Scope

| Name (Address Link) | Repo |
| --- | --- |
| LegionBouncer | |

## Out-of-Scope
- Anything in `src/mocks`
"#
            .to_string(),
            decision: SourceDecision::UsedForBoth,
            reason: ENTRY_CONTEXT_REASON.to_string(),
        }];

        let extracted =
            deterministic_bounty_contract_name_scope_extract(&sources, tmp.path(), &cli);

        assert_eq!(extracted.discovered_names, vec!["LegionBouncer"]);
        assert_eq!(extracted.unmapped_names, Vec::<String>::new());
        assert_eq!(extracted.entries.len(), 1);
        assert_eq!(extracted.entries[0].path, "./src/access/LegionBouncer.sol");
    }

    #[test]
    fn html_to_text_preserves_anchor_href_for_scope_links() {
        let address = "0xfbb21d0380bee3312b33c4353c8936a0f13ef26c";
        let html = format!(
            r#"<html><body><h2>Smart Contracts in Scope</h2><a href="https://basescan.org/address/{address}">Comptroller</a></body></html>"#
        );

        let text = html_to_text(&html);

        assert!(text.contains(&format!(
            "Comptroller (https://basescan.org/address/{address})"
        )));
    }

    #[test]
    fn external_contract_assets_extract_markdown_and_html_preserved_links() {
        let address = "0xfbb21d0380bee3312b33c4353c8936a0f13ef26c";
        let sources = vec![SourceContent {
            id: "source-1".to_string(),
            kind: ContextSourceKind::LocalReadme,
            location: "README.md".to_string(),
            title: None,
            content: format!(
                r#"
# Smart Contracts in Scope

| Name | Address |
| --- | --- |
| [Smart Contract - Unitroller/Comptroller](https://basescan.org/address/{address}) | Base |
"#
            ),
            decision: SourceDecision::UsedForBoth,
            reason: ENTRY_CONTEXT_REASON.to_string(),
        }];

        let assets = external_contract_scope_assets_from_sources(&sources);

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].label, "Smart Contract - Unitroller/Comptroller");
        assert_eq!(assets[0].address, address);
        assert_eq!(assets[0].explorer, "basescan.org");
    }

    #[test]
    fn external_contract_assets_ignore_docs_only_sources() {
        let address = "0xfbb21d0380bee3312b33c4353c8936a0f13ef26c";
        let sources = vec![SourceContent {
            id: "source-1".to_string(),
            kind: ContextSourceKind::WebHtml,
            location: "https://docs.example/protocol".to_string(),
            title: None,
            content: format!(
                r#"
# Smart Contracts in Scope

Reference deployment: https://basescan.org/address/{address}
"#
            ),
            decision: SourceDecision::UsedForDocs,
            reason: "Documentation link".to_string(),
        }];

        let assets = external_contract_scope_assets_from_sources(&sources);

        assert!(assets.is_empty());
    }

    #[test]
    fn external_metadata_fetch_queue_prioritizes_unresolved_assets() {
        let mut resolution = ExternalContractScopeResolution::default();
        for index in 0..40 {
            let address = format!("0x{index:040x}");
            resolution.assets.push(ExternalContractScopeAsset {
                label: format!("Resolved {index}"),
                url: format!("https://basescan.org/address/{address}"),
                address,
                explorer: "basescan.org".to_string(),
                source_location: "README.md".to_string(),
            });
        }
        let unresolved = ExternalContractScopeAsset {
            label: "Generic Proxy".to_string(),
            url: "https://basescan.org/address/0xffffffffffffffffffffffffffffffffffffffff"
                .to_string(),
            address: "0xffffffffffffffffffffffffffffffffffffffff".to_string(),
            explorer: "basescan.org".to_string(),
            source_location: "README.md".to_string(),
        };
        resolution.assets.push(unresolved.clone());
        resolution.unresolved.push(unresolved);

        let queue = external_metadata_fetch_queue(&resolution);

        assert_eq!(queue[0].label, "Generic Proxy");
        assert!(
            queue
                .iter()
                .take(MAX_EXTERNAL_CONTRACT_METADATA_FETCHES)
                .any(|asset| asset.address == "0xffffffffffffffffffffffffffffffffffffffff")
        );
    }

    #[test]
    fn external_scope_asset_key_keeps_same_address_chains_distinct() {
        let address = "0xfbb21d0380bee3312b33c4353c8936a0f13ef26c";
        let base = ExternalContractScopeAsset {
            label: "Vault".to_string(),
            url: format!("https://basescan.org/address/{address}"),
            address: address.to_string(),
            explorer: "basescan.org".to_string(),
            source_location: "README.md".to_string(),
        };
        let arbitrum = ExternalContractScopeAsset {
            label: "Vault".to_string(),
            url: format!("https://arbiscan.io/address/{address}"),
            address: address.to_string(),
            explorer: "arbiscan.io".to_string(),
            source_location: "README.md".to_string(),
        };
        let resolved = BTreeSet::from([external_scope_asset_key(&base)]);
        let mut unresolved = vec![base.clone(), arbitrum.clone()];

        unresolved.retain(|asset| !resolved.contains(&external_scope_asset_key(asset)));

        assert_ne!(
            external_scope_asset_key(&base),
            external_scope_asset_key(&arbitrum)
        );
        assert_eq!(unresolved.len(), 1);
        assert_eq!(unresolved[0].explorer, "arbiscan.io");
    }

    #[test]
    fn external_asset_resolution_matches_chain_suffixed_display_label_prefix() {
        let asset = ExternalContractScopeAsset {
            label: "SubgraphService - Arbitrum One".to_string(),
            url: "https://arbiscan.io/address/0xb2bb92d0de618878e438b55d5846cfecd9301105"
                .to_string(),
            address: "0xb2bb92d0de618878e438b55d5846cfecd9301105".to_string(),
            explorer: "arbiscan.io".to_string(),
            source_location: "https://immunefi.com/bug-bounty/thegraph/scope/".to_string(),
        };
        let definitions = vec![SolidityDefinition {
            name: "SubgraphService".to_string(),
            path:
                "./graphprotocol-contracts/packages/subgraph-service/contracts/SubgraphService.sol"
                    .to_string(),
            file_stem: "SubgraphService".to_string(),
            normalized_name: normalize_identifier("SubgraphService"),
            normalized_file_stem: normalize_identifier("SubgraphService"),
            tokens: vec!["service".to_string(), "subgraph".to_string()],
        }];

        let record = deterministic_external_asset_resolution(&asset, None, &definitions).unwrap();

        assert_eq!(record.paths, vec![definitions[0].path.clone()]);
    }

    #[test]
    fn scanner_metadata_parser_extracts_contract_names_source_files_and_implementation() {
        let implementation = "0x1111111111111111111111111111111111111111";
        let html = format!(
            r#"
<html>
  <head><title>Moonwell: Comptroller | Address 0xfbb21d0380bee3312b33c4353c8936a0f13ef26c</title></head>
  <body>
    <div>Contract Name:</div><div><span>Unitroller</span></div>
    <div>File 1 of 24 : Unitroller.sol</div>
    <div>Implementation: {implementation}</div>
    <pre>contract Unitroller {{}}</pre>
  </body>
</html>
"#
        );

        let metadata =
            external_contract_metadata_from_html("https://basescan.org/address/x", &html);

        assert!(metadata.contract_names.contains(&"Comptroller".to_string()));
        assert!(metadata.contract_names.contains(&"Unitroller".to_string()));
        assert_eq!(metadata.source_files, vec!["Unitroller.sol"]);
        assert_eq!(metadata.implementation_addresses, vec![implementation]);
    }

    #[test]
    fn proxy_metadata_resolution_uses_implementation_metadata() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("contracts")).unwrap();
        fs::write(
            tmp.path().join("contracts/VaultImplementation.sol"),
            "contract VaultImplementation {}",
        )
        .unwrap();
        let proxy = ExternalContractScopeAsset {
            label: "TransparentUpgradeableProxy".to_string(),
            url: "https://etherscan.io/address/0x2222222222222222222222222222222222222222"
                .to_string(),
            address: "0x2222222222222222222222222222222222222222".to_string(),
            explorer: "etherscan.io".to_string(),
            source_location: "scope".to_string(),
        };
        let implementation = "0x1111111111111111111111111111111111111111";
        let proxy_metadata = ExternalContractMetadata {
            url: proxy.url.clone(),
            contract_names: vec!["TransparentUpgradeableProxy".to_string()],
            implementation_addresses: vec![implementation.to_string()],
            source_files: Vec::new(),
        };
        let implementation_metadata = ExternalContractMetadata {
            url: explorer_address_url(&proxy.explorer, implementation),
            contract_names: vec!["VaultImplementation".to_string()],
            implementation_addresses: Vec::new(),
            source_files: vec!["contracts/VaultImplementation.sol".to_string()],
        };
        let implementation_assets = implementation_metadata_assets(&proxy, &proxy_metadata);
        let mut metadata_by_url = BTreeMap::new();
        metadata_by_url.insert(metadata_url_key(&proxy.url), proxy_metadata);
        metadata_by_url.insert(
            metadata_url_key(&implementation_metadata.url),
            implementation_metadata,
        );
        let definitions = solidity_definitions(tmp.path(), &[]);

        let combined = combined_external_metadata_for_asset(&proxy, &metadata_by_url).unwrap();
        let record =
            deterministic_external_asset_resolution(&proxy, Some(&combined), &definitions).unwrap();

        assert_eq!(implementation_assets.len(), 1);
        assert_eq!(implementation_assets[0].address, implementation);
        assert_eq!(record.paths, vec!["./contracts/VaultImplementation.sol"]);
    }

    #[test]
    fn external_contract_scope_maps_split_labels_to_local_definitions() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("contracts")).unwrap();
        fs::write(
            tmp.path().join("contracts/Unitroller.sol"),
            "contract Unitroller {}",
        )
        .unwrap();
        fs::write(
            tmp.path().join("contracts/Comptroller.sol"),
            "contract Comptroller {}",
        )
        .unwrap();
        let asset = ExternalContractScopeAsset {
            label: "Smart Contract - Unitroller/Comptroller".to_string(),
            url: "https://basescan.org/address/0xfbb21d0380bee3312b33c4353c8936a0f13ef26c"
                .to_string(),
            address: "0xfbb21d0380bee3312b33c4353c8936a0f13ef26c".to_string(),
            explorer: "basescan.org".to_string(),
            source_location: "README.md".to_string(),
        };
        let definitions = solidity_definitions(tmp.path(), &[]);

        let record = deterministic_external_asset_resolution(&asset, None, &definitions).unwrap();

        assert_eq!(
            record.paths,
            vec!["./contracts/Comptroller.sol", "./contracts/Unitroller.sol"]
        );
    }

    #[test]
    fn immunefi_external_scope_uses_structured_asset_descriptions() {
        let bounty = ImmunefiBountyData {
            input_url: "https://immunefi.com/bug-bounty/example/information/".to_string(),
            urls: ImmunefiBountyUrls {
                base: "https://immunefi.com/bug-bounty/example".to_string(),
                information: "https://immunefi.com/bug-bounty/example/information/".to_string(),
                scope: "https://immunefi.com/bug-bounty/example/scope/".to_string(),
                resources: "https://immunefi.com/bug-bounty/example/resources/".to_string(),
            },
            project: "Example".to_string(),
            slug: "example".to_string(),
            description: None,
            website_url: None,
            github_url: None,
            max_bounty: None,
            launch_date: None,
            updated_date: None,
            proof_of_concept_type: None,
            primacy: None,
            severity_system: None,
            rewards_token: None,
            rewards_token_network: None,
            codebases: Vec::new(),
            documentations: Vec::new(),
            audits: Vec::new(),
            assets: vec![
                ImmunefiAsset {
                    url: "https://etherscan.io/address/0xdddddddddddddddddddddddddddddddddddddddd"
                        .to_string(),
                    asset_type: Some("smart_contract".to_string()),
                    description: Some("SSV Network".to_string()),
                    is_primacy_of_impact: false,
                },
                ImmunefiAsset {
                    url: "https://etherscan.io/address/0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .to_string(),
                    asset_type: Some("smart_contract".to_string()),
                    description: Some("SSV Network View".to_string()),
                    is_primacy_of_impact: false,
                },
                ImmunefiAsset {
                    url: "https://immunefi.com".to_string(),
                    asset_type: Some("smart_contract".to_string()),
                    description: Some("Primacy of Impact".to_string()),
                    is_primacy_of_impact: true,
                },
            ],
            impacts: Vec::new(),
            rewards: Vec::new(),
            default_out_of_scope_smart_contract: None,
            default_out_of_scope_general: None,
            prohibited_activities: None,
            custom_out_of_scope: None,
            known_issues: Vec::new(),
            tabs: Vec::new(),
        };

        let assets = immunefi_external_contract_scope_assets(&bounty);

        assert_eq!(assets.len(), 2);
        assert!(assets.iter().any(|asset| asset.label == "SSV Network"));
        assert!(assets.iter().any(|asset| asset.label == "SSV Network View"));
        assert!(
            assets
                .iter()
                .all(|asset| asset.source_location == bounty.urls.scope)
        );
    }

    #[test]
    fn immunefi_structured_deployment_assets_skip_broad_source_scraping() {
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/org/repo"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/example/information/"
"#,
        )
        .unwrap();
        let bounty = ImmunefiBountyData {
            input_url: "https://immunefi.com/bug-bounty/example/information/".to_string(),
            urls: ImmunefiBountyUrls {
                base: "https://immunefi.com/bug-bounty/example".to_string(),
                information: "https://immunefi.com/bug-bounty/example/information/".to_string(),
                scope: "https://immunefi.com/bug-bounty/example/scope/".to_string(),
                resources: "https://immunefi.com/bug-bounty/example/resources/".to_string(),
            },
            project: "Example".to_string(),
            slug: "example".to_string(),
            description: None,
            website_url: None,
            github_url: None,
            max_bounty: None,
            launch_date: None,
            updated_date: None,
            proof_of_concept_type: None,
            primacy: None,
            severity_system: None,
            rewards_token: None,
            rewards_token_network: None,
            codebases: Vec::new(),
            documentations: Vec::new(),
            audits: Vec::new(),
            assets: vec![ImmunefiAsset {
                url: "https://github.com/ensdomains/ens-contracts/wiki/ENS-Contract-Deployments"
                    .to_string(),
                asset_type: Some("smart_contract".to_string()),
                description: Some("ENS Contract Deployments".to_string()),
                is_primacy_of_impact: false,
            }],
            impacts: Vec::new(),
            rewards: Vec::new(),
            default_out_of_scope_smart_contract: None,
            default_out_of_scope_general: None,
            prohibited_activities: None,
            custom_out_of_scope: None,
            known_issues: Vec::new(),
            tabs: Vec::new(),
        };
        let sources = vec![SourceContent {
            id: "resources".to_string(),
            kind: ContextSourceKind::ImmunefiResources,
            location: "https://immunefi.com/bug-bounty/example/resources/".to_string(),
            title: Some("Resources".to_string()),
            content: "Documentation noise links https://etherscan.io/address/0x1111111111111111111111111111111111111111".to_string(),
            decision: SourceDecision::UsedForBoth,
            reason: "resources".to_string(),
        }];

        assert_eq!(
            external_contract_scope_assets_from_sources(&sources).len(),
            1
        );
        assert!(
            external_contract_scope_assets_for_audit(&cli, &sources, None, Some(&bounty))
                .is_empty()
        );
    }

    #[test]
    fn external_scope_mapping_excludes_dependencies_tests_and_pluralizes_labels() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("contracts/test/mocks")).unwrap();
        fs::create_dir_all(tmp.path().join("node_modules/@openzeppelin/contracts")).unwrap();
        fs::write(
            tmp.path().join("contracts/SSVNetwork.sol"),
            "contract SSVNetwork {}",
        )
        .unwrap();
        fs::write(
            tmp.path().join("contracts/SSVNetworkViews.sol"),
            "contract SSVNetworkViews {}",
        )
        .unwrap();
        fs::write(
            tmp.path().join("contracts/test/mocks/SSVNetworkView.sol"),
            "contract SSVNetworkView {}",
        )
        .unwrap();
        fs::write(
            tmp.path()
                .join("node_modules/@openzeppelin/contracts/SSVNetworkViews.sol"),
            "contract SSVNetworkViews {}",
        )
        .unwrap();
        fs::create_dir_all(
            tmp.path()
                .join("contracts/lib/openzeppelin-contracts/contracts"),
        )
        .unwrap();
        fs::write(
            tmp.path()
                .join("contracts/lib/openzeppelin-contracts/contracts/SSVNetworkViews.sol"),
            "contract SSVNetworkViews {}",
        )
        .unwrap();
        fs::create_dir_all(tmp.path().join("contracts/contracts/lib/math")).unwrap();
        fs::write(
            tmp.path()
                .join("contracts/contracts/lib/math/InternalMath.sol"),
            "contract SSVNetworkViewsHelper {}",
        )
        .unwrap();
        let definitions = solidity_definitions(tmp.path(), &[]);
        let asset = ExternalContractScopeAsset {
            label: "SSV Network View".to_string(),
            url: "https://etherscan.io/address/0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                .to_string(),
            address: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            explorer: "etherscan.io".to_string(),
            source_location: "https://immunefi.com/bug-bounty/example/scope/".to_string(),
        };

        let record = deterministic_external_asset_resolution(&asset, None, &definitions).unwrap();

        assert_eq!(record.paths, vec!["./contracts/SSVNetworkViews.sol"]);
        assert!(definitions.iter().any(|definition| {
            definition.path == "./contracts/contracts/lib/math/InternalMath.sol"
        }));
    }

    #[test]
    fn deployment_page_scope_extracts_only_production_links_with_row_labels() {
        let raw = r#"
# mainnet
| Assets | Contracts |
|--------|----------|
| ENSRegistry | [0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e](https://etherscan.io/address/0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e) |
| NameWrapper | [0xD4416b13d2b3a9aBae7AcD5D6C2BbDBE25686401](https://etherscan.io/address/0xD4416b13d2b3a9aBae7AcD5D6C2BbDBE25686401) |
# sepolia
| Assets | Contracts |
|--------|----------|
| ENSRegistry | [0x1111111111111111111111111111111111111111](https://sepolia.etherscan.io/address/0x1111111111111111111111111111111111111111) |
"#;

        let production = production_scope_text(raw);
        let assets = external_contract_assets_from_text(&production, "deployment-page");

        assert_eq!(assets.len(), 2);
        assert!(assets.iter().any(|asset| asset.label == "ENSRegistry"));
        assert!(assets.iter().any(|asset| asset.label == "NameWrapper"));
        assert!(
            assets
                .iter()
                .all(|asset| !asset.url.contains("sepolia.etherscan.io"))
        );
    }

    #[test]
    fn deployment_page_scope_keeps_generic_production_deployment_sections() {
        let raw = r#"
# Contract Deployments
| Assets | Contracts |
|--------|----------|
| Registry | [0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e](https://etherscan.io/address/0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e) |
# Testnet Deployments
| Assets | Contracts |
|--------|----------|
| Registry | [0x1111111111111111111111111111111111111111](https://sepolia.etherscan.io/address/0x1111111111111111111111111111111111111111) |
"#;

        let production = production_scope_text(raw);
        let assets = external_contract_assets_from_text(&production, "deployment-page");

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].label, "Registry");
        assert_eq!(assets[0].explorer, "etherscan.io");
        assert!(!production.contains("sepolia.etherscan.io"));
    }

    #[tokio::test]
    #[ignore]
    async fn live_ens_deployment_page_scope_extracts_mainnet_assets() {
        let bounty = crate::prepare_code::immunefi::fetch_immunefi_bounty(
            "https://immunefi.com/bug-bounty/ens/information/",
        )
        .await
        .unwrap();

        let assets = fetch_immunefi_deployment_page_scope_assets(&bounty).await;

        assert!(
            assets
                .iter()
                .any(|asset| asset.label == "ENSRegistry" && asset.explorer == "etherscan.io")
        );
        assert!(
            assets
                .iter()
                .any(|asset| asset.label == "NameWrapper" && asset.explorer == "etherscan.io")
        );
        assert!(
            assets
                .iter()
                .all(|asset| !asset.url.contains("sepolia.etherscan.io"))
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
