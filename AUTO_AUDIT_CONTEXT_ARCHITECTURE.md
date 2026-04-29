# Auto Audit Context Architecture

## Goal

Generate the three audit context artifacts that are currently prepared by hand:

- `<repo-folder>-scope.txt`
- `<repo-folder>-scope.md`
- `<repo-folder>-docs.md`

The generator runs after the target repository is cloned and built under `~/Desktop/Audit/<project-id>/`, but before the normal `RepoPaths` file discovery finishes. Existing manual YAML fields remain supported, but the default workflow should no longer require `custom_doc`, `audit_scope`, or `scoped_files`.

The generated filename prefix is the sanitized repo folder name, preserving date and contest identity. Example: repo folder `2026-04-monetrix` generates `2026-04-monetrix-scope.md`, not `monetrix-scope.md`.

The generated Markdown files must be concise: `scope.md` and `docs.md` are each capped at 5000 tokens.

## User-Facing Configuration

Add an optional `context` block to YAML and CLI deserialization. If omitted, the generator uses `README.md` from the analyzed protocol root.

```yaml
repo: "https://github.com/code-423n4/2026-04-monetrix.git"
audit_type: "Code4rena"

context:
  files:
    - README.md
    - docs/architecture.md
  urls:
    - https://example.com/protocol-docs
  v12_url: "auto"
  output_dir: "audit-docs"
  force_regenerate: true
```

Defaults:

- `files`: `["README.md"]`
- `urls`: `[]`
- `v12_url`: `"auto"`
- `output_dir`: `"audit-docs"`
- `force_regenerate`: `true`
- `max_tokens_per_file`: `5000`

Backwards compatibility:

- If `custom_doc`, `audit_scope`, or `scoped_files` is explicitly set and `context.force_regenerate == false`, keep the manual file path.
- If `force_regenerate == true`, generate fresh artifacts and use them as the effective paths.
- If no `context` block is present and all three manual context paths are set, preserve legacy behavior and do not generate/override context files.
- Manual files may still be useful for fixture-like runs, but generated context is the default.

## Pipeline Placement

Current `clone_and_filter_git_repo` does:

1. validate repo URL
2. resolve commit hash
3. clone/build repo
4. scan Solidity/docs/config files
5. resolve manually supplied docs/scope files
6. return `RepoPaths`

New flow:

1. validate repo URL
2. resolve commit hash
3. clone/build repo
4. determine protocol root from `repo.root + repo.repo_name`
5. generate or reuse audit context artifacts
6. rewrite effective CLI-derived paths for docs/scope:
   - `custom_doc = generated_docs_md`
   - `audit_scope = generated_scope_md`
   - `scoped_files = generated_scope_txt`
7. scan Solidity/docs/config files
8. return `RepoPaths`

This keeps the rest of the review pipeline mostly unchanged: `RepoPaths::extract_content_from_docs`, `extract_content_from_scope_file`, and `extract_scoped_files` can continue reading files from paths.

## Proposed Modules

```text
src/prepare_code/audit_context.rs
src/prepare_code/audit_context_prompts.rs
src/prepare_code/context_sources.rs
```

`audit_context.rs` owns orchestration and public structs.

`context_sources.rs` owns local file reads, README link extraction, HTTP fetches, GitHub raw resolution, and source classification helpers.

`audit_context_prompts.rs` owns Codex prompts and structured response schemas.

If this feels too much for the first patch, start with a single `audit_context.rs` and split once the implementation stabilizes.

## Data Model

### CLI/YAML Config

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct ContextConfig {
    #[serde(default = "default_context_files")]
    pub files: Vec<String>,

    #[serde(default)]
    pub urls: Vec<String>,

    #[serde(default = "default_v12_url")]
    pub v12_url: V12Source,

    #[serde(default = "default_context_output_dir")]
    pub output_dir: String,

    #[serde(default = "default_force_regenerate")]
    pub force_regenerate: bool,

    #[serde(default = "default_context_token_limit")]
    pub max_tokens_per_file: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum V12Source {
    Auto(String),      // accepts "auto"
    Disabled(bool),   // false disables V12 fetch
    Url(String),      // explicit public report URL
}

fn default_context_files() -> Vec<String> {
    vec!["README.md".to_string()]
}

fn default_v12_url() -> V12Source {
    V12Source::Auto("auto".to_string())
}

fn default_context_output_dir() -> String {
    "audit-docs".to_string()
}

fn default_force_regenerate() -> bool {
    true
}

fn default_context_token_limit() -> usize {
    5000
}
```

In `Cli`:

```rust
#[derive(Parser, Clone, Debug, Deserialize)]
pub struct Cli {
    // existing fields...

    #[serde(default)]
    pub context: Option<ContextConfig>,
}
```

Merge behavior in `Cli::parse_args` should mirror existing optional fields: YAML `context` is used when present. CLI flags for nested fields can be deferred until needed; YAML support is enough for the initial feature.

### Generation Result

```rust
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
```

### Source Tracking

```rust
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
```

### Scope File List

```rust
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
    ExtractedByCodex,
}
```

## Public API

```rust
pub async fn generate_audit_context(
    cli: &Cli,
    workspace_root: &Path,
    protocol_root: &Path,
    repo_name: &str,
    commit_hash: &str,
) -> anyhow::Result<GeneratedAuditContext>
```

This is async because Codex and HTTP fetches are async or blocking wrapped in async. `clone_and_filter_git_repo` is currently sync, so there are two implementation options:

1. Make `clone_and_filter_git_repo` async and update `main`.
2. Keep `clone_and_filter_git_repo` sync, and run the async generator through a small runtime helper.

Preferred: make `clone_and_filter_git_repo` async. The main function is already async, and this avoids nested runtime footguns.

Call shape in `main`:

```rust
let repo = prepare_code::git_clone::clone_and_filter_git_repo(&cli).await?;
```

## Source Discovery

### Local Files

The generator reads:

- `context.files`, defaulting to `README.md`
- any root-level Markdown files whose names suggest known issues or audit material:
  - `KNOWN_ISSUES.md`
  - `known-issues.md`
  - `SECURITY.md`
  - `AUDIT.md`
  - `audits/*.md`
- `<protocol-root>/scope.txt`, when available

Do not deeply ingest the whole repo by default. The normal code analysis pipeline already covers Solidity; this feature is for audit context.

### Link Extraction

Parse Markdown links only from entry context files:

- default entry file: `README.md`
- additional entry files: explicit `context.files`
- fetched second-level sources do not emit more links

- inline Markdown links: `[label](url)`
- bare URLs
- HTML anchors: `<a href="...">`

Resolve relative links:

- repo-relative Markdown path -> local file under protocol root
- GitHub blob URL -> raw content URL
- GitHub tree URL -> skip unless it points to obvious docs/scope file

Skip by default:

- social links
- generic websites with no docs/scope signal
- images/assets unless SVG/diagram text extraction becomes useful later
- source-code links unless they are clearly scope/docs/known-issues/V12 text

Fetch by default:

- docs links
- scope links
- known issue/prior audit links
- V12 links
- GitHub Markdown/raw links

## Codex Agents

Use OpenAI/Codex via existing `AgentFactory`.

```rust
let agent = AgentFactory::create_openai_agent(
    &AgentConfig::new(None)
        .with_model(OPENAI_MODEL)
        .with_openai_reasoning_effort("high")
        .with_preamble(AUDIT_CONTEXT_PREAMBLE)
)?;
```

The generator should use structured extraction, not free-form text, for intermediate steps.

### Structured Responses

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContextExtraction {
    pub overview: String,
    pub links: Vec<ExtractedLink>,
    pub files_in_scope: Vec<String>,
    pub files_out_of_scope: Vec<String>,
    pub known_issues: Vec<String>,
    pub areas_of_concern: Vec<String>,
    pub invariants: Vec<String>,
    pub trusted_roles: Vec<TrustedRole>,
    pub docs_topics: Vec<String>,
    pub v12_candidates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExtractedLink {
    pub url: String,
    pub label: String,
    pub classification: String,
    pub should_fetch: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TrustedRole {
    pub role: String,
    pub description: String,
    pub trust_assumption: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GeneratedMarkdown {
    pub markdown: String,
    pub token_estimate: usize,
    pub omitted_items: Vec<String>,
    pub source_notes: Vec<String>,
}
```

## Scope.txt Algorithm

1. If `<protocol-root>/scope.txt` exists:
   - read it
   - normalize line endings
   - trim blank lines
   - preserve order
   - write to `<output_dir>/<repo-folder>-scope.txt`
   - mark source as `CopiedScopeTxt`
2. Else attempt deterministic README extraction:
   - parse Markdown tables under headings matching `scope`, `files in scope`, `contracts in scope`
   - extract local file paths ending in `.sol`
   - normalize leading slash to `./`
   - validate existence
3. If deterministic extraction finds no usable paths:
   - ask Codex to extract in-scope Solidity files from context
   - validate and write paths
4. If still empty:
   - warn and fall back to existing `code_folders` scan, but do not pretend this is contest scope.

## Scope.md Composition

`scope.md` must be comprehensive but compact.

Required skeleton:

```md
# <Protocol> Audit Scope

## Table of Contents

## Publicly Known Issues

## Overview

## Links

## Scope
### Files In Scope
### Files Out Of Scope

## Areas Of Concern

## Main Invariants

## Trusted Roles

## V12 / Prior Findings

## Source Notes
```

Priority order when compressing:

1. keep scope and known issues
2. keep V12/prior findings
3. keep invariants and trusted roles
4. compress overview
5. compress links/source notes

## Docs.md Composition

`docs.md` should explain the protocol mechanics needed for security review.

Suggested skeleton:

```md
# <Protocol> Protocol Notes

## Table of Contents

## What The Protocol Does

## Core Architecture

## Main User Flows

## Accounting / Value Flow

## External Integrations

## Trust Boundaries

## Security-Relevant Assumptions

## Source Notes
```

Do not include large copied docs verbatim. The generated file is an audit briefing, not a mirror.

## Token Enforcement

Use existing `get_token_count`.

```rust
fn enforce_token_limit(
    agent: &AIAgent,
    markdown: String,
    max_tokens: usize,
    doc_kind: GeneratedDocKind,
) -> anyhow::Result<String>
```

Algorithm:

1. Count tokens.
2. If `<= max_tokens`, accept.
3. Ask Codex for a summarization/compression pass preserving required sections.
4. Count again.
5. Retry summarization a small fixed number of times if still over.
6. If still over, fail loudly with an actionable error.

Never silently truncate generated `scope.md` or `docs.md`. Truncation can drop the exact scope or known-issue detail an auditor needed; summarization is safer.

## V12 / Prior Findings

Resolution order:

1. explicit `context.v12_url`
2. V12 links found in entry context files
3. last-resort web search by repo slug and contest slug for Code4rena only
4. no report found

The first implementation can support explicit links and README-discovered links. Web search can be a follow-up if the Rust runtime does not yet have a clean search client.

Generated `scope.md` should include:

- V12 source URL
- summary counts if available
- compact table of severity, title, summary
- duplicate policy note

## Browser / MCP Boundary

Default implementation should use deterministic HTTP fetches and GitHub raw URLs. Browser/Computer Use should only be required for pages that cannot be fetched as static HTML/Markdown.

If browser support is added:

- tool access is narrowly scoped to browsing public docs
- no GitHub write access
- no Gmail/Drive
- no repo mutation from the browser worker
- Rust remains responsible for writing generated files

## Error Handling

Non-fatal warnings:

- README missing
- link fetch failed
- V12 not found
- some extracted scope paths do not exist
- generated docs had to omit sections to stay under token budget

Fatal errors:

- no source context at all and no fallback path
- Codex generation fails after retries
- generated `scope.txt` is empty for `Code4rena` and no fallback is accepted
- generated Markdown exceeds token limit after summarization/compression retries

## Tests

Unit tests:

- `ContextConfig` defaults set `force_regenerate = true`
- README link extraction
- GitHub blob-to-raw conversion
- scope table parsing
- scope.txt normalization
- token limit trim ordering

Integration-style tests with fixtures:

```text
tests/fixtures/audit_context/code4rena_with_scope_txt/
tests/fixtures/audit_context/readme_only_scope/
tests/fixtures/audit_context/readme_with_docs_links/
tests/fixtures/audit_context/known_issues_files/
```

Tests should avoid live web and live Codex by injecting a mock generator:

```rust
#[async_trait]
pub trait ContextGenerator {
    async fn extract_context(&self, input: ContextInput) -> anyhow::Result<ContextExtraction>;
    async fn write_scope_md(&self, input: ScopeMarkdownInput) -> anyhow::Result<GeneratedMarkdown>;
    async fn write_docs_md(&self, input: DocsMarkdownInput) -> anyhow::Result<GeneratedMarkdown>;
    async fn compress(&self, input: CompressionInput) -> anyhow::Result<GeneratedMarkdown>;
}
```

Production implementation: `CodexContextGenerator`.

Test implementation: `MockContextGenerator`.

## Incremental Implementation Plan

1. Add `ContextConfig` to CLI/YAML parsing with defaults.
2. Add `audit_context.rs` with local-only generation:
   - copy `scope.txt`
   - read README/configured entry files
   - parse links
   - write minimal source report
3. Add Codex generation for `scope.md` and `docs.md`.
4. Add HTTP/GitHub link fetching and classification.
5. Add V12 explicit/discovered link support.
6. Wire generated paths into `RepoPaths`.
7. Update README and example YAML.
8. Add fixture tests.

## Open Design Decisions

- Whether to make `clone_and_filter_git_repo` async immediately or bridge with a runtime helper.
- Whether live web search for V12 should be built into Rust now or deferred behind explicit/discovered URLs.
- Whether generated files should always live in repo-local `audit-docs/` or allow absolute `output_dir`.
- Whether generated docs should be committed-like durable artifacts or treated as cache outputs that can be regenerated every run.
