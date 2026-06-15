# AI Agent Audit v2.0

## Overview
AI Agent Audit is the first of its kind open source tools that: 

- automatically discovers security vulnerability in solidity EVM-based codebases
- dedupes, validates, and creates runnable PoC for each valid High / Medium vulnerability found 
- creates professional audit reports for each valid finding

I used this tool to compete in Code4rena (I am not a security researcher), and the results are promising: https://code4rena.com/@saraswati

It performed on par with a human security researcher, and even achieved the distinction of SR Warden, which only ~1% of security researcher attain.

This tool is a Rust command-line tool for AI-assisted security review of Solidity repositories. It clones and builds a target repo in a local audit workspace, extracts semantic data with Slither, generates per-contract code slices, runs LLM-based discovery and verification passes, and writes Markdown audit reports.

This repository is being released as a GitHub-first public beta. It is meant to accelerate expert review, not replace manual auditing.

## Status

- Public beta.
- Solidity and EVM-focused.
- Repository source, docs, and derived context are sent to third-party LLM providers you configure.
- The current default audit pipeline uses ChatGPT/Codex OAuth for OpenAI access and runs the active review flow on `gpt-5.5`. Deduplication helpers use `gpt-5.4` with low reasoning.
- Codex is the recommended standard path because long audit runs are roughly 25x more cost-effective through a Codex/ChatGPT subscription than direct per-token API billing.
- Startup performs a one-time ChatGPT sign-in if needed and reuses the cached session on later runs until the token expires.
- `OPENAI_API_KEY` is supported as a secondary fallback for Rust OpenAI calls by setting `AI_AGENT_AUDIT_OPENAI_BACKEND=api`.
- `ANTHROPIC_API_KEY`, `GEMINI_API_KEY` / `GOOGLE_AI_API_KEY`, and `DEEPSEEK_API_KEY` are still supported by the agent layer, but they are not required by the default review path.
- Discovery-style runs can be switched back to Gemini with env config if you want to use Google AI for patterns, actors, and invariants while keeping verification/reporting on OpenAI/Codex.
- PoC-related config fields exist, but automatic PoC generation is currently disabled in the public beta.

## What It Does

- Clones and builds Foundry or Hardhat repositories under `~/Desktop/Audit` by default.
- Generates audit scope and protocol docs from README/configured entry files into `audit-docs/`.
- Uses Slither-derived call graph and semantic data when static analysis succeeds.
- Builds inheritance and interface-implementation indexes from Solidity source.
- Generates contextual codeblocks for each in-scope contract.
- Uses pattern libraries, invariant prompts, and actor-oriented context to discover candidate findings.
- Verifies and deduplicates findings before producing report output.
- Stores local SQLite state in `.ai-agent-audit/`.

## Who It Is For

- Smart contract auditors and security researchers.
- Protocol teams doing internal review of Solidity codebases.
- Engineers experimenting with AI-assisted audit workflows on repos they are allowed to share with external model providers.

This project is not a hosted service, not a generic SAST scanner for every language, and not a substitute for human validation.

## Requirements

- Rust stable toolchain.
- Git.
- Slither.
- Foundry (`forge`) for Foundry repositories.
- Node.js plus `npm`/`npx`, Yarn, or pnpm for Hardhat repositories.
- Optional `GITHUB_TOKEN` for private GitHub repositories.

## Quickstart

1. Clone this repository and enter it.

```bash
git clone https://github.com/chain-shield/ai-agent-audit.git
cd ai-agent-audit
```

2. Create a local env file from the template.

```bash
cp .env.example .env
```

3. Edit `.env` and set the values you actually need:

```bash
RUST_LOG=info
AI_AGENT_AUDIT_OPENAI_BACKEND=codex
# Codex is the recommended default for cost. Optional Rust API fallback:
# AI_AGENT_AUDIT_OPENAI_BACKEND=api
# OPENAI_API_KEY=your_openai_api_key
# Optional non-OpenAI provider keys:
# ANTHROPIC_API_KEY=...
# GEMINI_API_KEY=...
# DEEPSEEK_API_KEY=...
```

4. The first Codex-backed run will prompt you to sign in with ChatGPT if there is no cached Codex session yet. After that, the session is reused automatically until expiry. If you set `AI_AGENT_AUDIT_OPENAI_BACKEND=api`, Rust OpenAI calls use `OPENAI_API_KEY` instead and do not require Codex sign-in.

5. Copy the example config and point it at a Solidity repository.

```bash
cp examples/audit-config.example.yaml audit-config.yaml
```

Minimal example:

```yaml
repo: "https://github.com/example/protocol.git"
audit_type: "Client"
code_folders:
  - "src"
```

6. Build and run the tool.

```bash
cargo build --release
cargo run --release -- --config audit-config.yaml
```

If you prefer not to use YAML for a simple run:

```bash
cargo run --release -- https://github.com/example/protocol.git --audit-type Client
```

For repos that keep contracts under `contracts/` instead of `src/`, set `code_folders` accordingly.

## Private Repositories

If the target repository is private, set `GITHUB_TOKEN` before running the tool. The clone path uses that token for GitHub HTTPS URLs.

```bash
export GITHUB_TOKEN=...
```

## Configuration

`--config <file>` loads YAML, and explicit CLI flags override YAML values. The current example file lives at [examples/audit-config.example.yaml](examples/audit-config.example.yaml).

### Supported `audit_type` Values

- `Code4rena`
- `Code4renaBounty`
- `Sherlock`
- `Cantina`
- `Client`

Use `Client` for internal or client-style audits. Use the contest values when you want severity handling and report language aligned more closely with those platforms. Use `Code4renaBounty` for C4 bug bounty programs where only currently exploitable Critical/High issues with runnable PoCs should become submission candidates.

### YAML And CLI Fields

| YAML key / CLI flag | Purpose |
| --- | --- |
| `repo` / positional `repo` | Required HTTP(S) Git repository URL. |
| `config` / `--config` | Load a YAML config file. |
| `subfolder` / `--subfolder` | Analyze a subdirectory inside the cloned repo, useful for monorepos. |
| `code_folders` / `--code-folders` | Source roots to scan for contracts. Defaults to `["src"]`. |
| `audit_scope` / `--audit-scope` | Local Markdown file containing scope notes or reviewer guidance. |
| `doc_folder` / `--doc-folder` | Repo-relative folder containing Markdown docs to ingest. |
| `custom_doc` / `--custom-doc` | Local Markdown file to use instead of auto-discovered root docs. |
| `monorepo_folders` / `--monorepo-folders` | Local text file listing repo-relative package roots for monorepo-aware analysis. |
| `exclude_folders` / `--exclude-folders` | Repo-relative folders to exclude from scope. |
| `scoped_files` / `--scoped-files` | Local text file listing repo-relative files that should be treated as in scope. |
| `validation_supervision` / `--validation-supervision` | Emit a Codex GUI three-shot validation job after report export. Defaults to `gui`; set `off` to disable. |
| `validation_supervision_overwrite` / `--validation-supervision-overwrite` | Replace an existing non-terminal GUI validation job with the same run id. Defaults to `false`; use only for intentional reruns. |
| `context` | YAML-only block for generated audit scope/docs context. Defaults to `README.md`, `audit-docs`, `force_regenerate: true`, and 5000 tokens per generated Markdown file. |
| `audit_type` / `--audit-type` | One of `Code4rena`, `Code4renaBounty`, `Sherlock`, `Cantina`, `Client`. |
| `builder` / `--builder` | One of `Foundry`, `Hardhat`, `HardhatYarn`, `Custom`, `Auto`. Default is `Auto`. |
| `build_cmd` / `--build-cmd` | Required when `builder: "Custom"` is used. |
| `via_ir` / `--via-ir` | Adds `--via-ir` to the Foundry build command. |
| `force_rebuild` / `--force-rebuild` | Re-clone and rebuild even if a cached workspace already exists. |

### Path Conventions

- `custom_doc`, `audit_scope`, `scoped_files`, and `monorepo_folders` are read from local files you provide on the machine running the tool.
- `subfolder`, `code_folders`, `doc_folder`, and `exclude_folders` are interpreted relative to the cloned target repository.
- If no manual `custom_doc`, `audit_scope`, or `scoped_files` are provided, the tool generates `<repo-folder>-docs.md`, `<repo-folder>-scope.md`, and `<repo-folder>-scope.txt` in `audit-docs/`.
- The generated filename prefix preserves the cloned repo folder identity, including date/contest prefixes such as `2026-04-monetrix`.
- `context.force_regenerate` defaults to `true` for generated context. Legacy YAMLs that already provide all three manual context files are left alone when no `context` block is present.

### Generated Context

Minimal generated-context config:

```yaml
context:
  files:
    - README.md
  output_dir: "audit-docs"
  force_regenerate: true
  max_tokens_per_file: 5000
```

The generator copies `scope.txt` from the cloned repo when present. If no `scope.txt` exists, it extracts in-scope Solidity paths from entry context files. By default the only entry file is `README.md`; `context.files` can add or replace entry files. Links found in those entry files are treated as second-level candidates and fetched only when they look relevant to scope, known issues, protocol documentation, prior audits, or Code4rena V12 reports. For `Code4renaBounty`, the generator also fetches Code4rena's bounty guide and bounty criteria, preserves global bounty out-of-scope rules, and can map contract-name-only scope tables to local Solidity definitions. Fetched second-level pages do not emit more links. If generated `scope.md` or `docs.md` exceeds `max_tokens_per_file`, Codex summarizes it down to the limit; the generator refuses to silently truncate final Markdown.

### Environment Variables

| Variable | Required | Notes |
| --- | --- | --- |
| ChatGPT/Codex sign-in | Yes for the default pipeline | Performed interactively once at startup when needed, then cached locally until expiry. |
| `AI_AGENT_AUDIT_OPENAI_BACKEND` | No | `codex` by default. Set to `api` to use direct OpenAI API billing. |
| `OPENAI_API_KEY` | Only for API fallback | Required when `AI_AGENT_AUDIT_OPENAI_BACKEND=api`. Not used by the default Codex path. |
| `GEMINI_API_KEY` | No | Supported by the agent layer, not required by the default path. |
| `GOOGLE_AI_API_KEY` | Legacy alias | Accepted as a fallback for Gemini. |
| `ANTHROPIC_API_KEY` | No | Supported by the agent layer, not required by the default path. |
| `DEEPSEEK_API_KEY` | No | Supported by the agent layer, not required by the default path. |
| `GITHUB_TOKEN` | No | Used for private GitHub repo cloning. |
| `AI_AGENT_AUDIT_DATA_DIR` | No | Overrides the local cache directory. Defaults to `.ai-agent-audit`. |
| `AI_AGENT_AUDIT_WORKSPACE_ROOT` | No | Overrides where target repos are cloned and built. Defaults to `~/Desktop/Audit`. |
| `AI_AGENT_AUDIT_WORKER_LAUNCHER` | No | Codex-compatible validation worker launcher. Defaults to `codex` on PATH and overrides YAML launcher values. |
| `RUST_LOG` | No | Standard Rust log level, defaults to `info`. |

The shipped template is [`.env.example`](.env.example).

Discovery provider/model defaults now live in [src/config.rs](src/config.rs). Edit `DISCOVERY_PROVIDER`, `GEMINI_DISCOVERY_MODEL`, and `DISCOVERY_GEMINI_THINKING_LEVEL` there if you want to switch discovery between OpenAI and Gemini.

## How The Pipeline Works

1. Repository preparation. The tool validates the repo URL, resolves the current `HEAD` commit, clones the target into a local workspace under `~/Desktop/Audit/<project-id>/`, and builds it with Foundry, Hardhat, or a custom command.

2. Audit context generation. The tool reads README/configured entry files, follows relevant second-level links, copies or extracts scope, and writes generated scope/docs files under `audit-docs/`.

3. Semantic extraction. It runs the Slither-based enrichment path to build a local semantic SQLite database with function metadata and call graph edges.

4. Metadata context. It generates protocol-level context used later by the audit prompts and saves a metadata Markdown artifact.

5. Solidity indexing. It builds inheritance information from source and then derives an interface-implementation index.

6. Codeblock generation. It slices the codebase into contextual per-contract codeblocks using call graph depth and token-budget settings.

7. AI review. Verification, deduplication, summaries, and report-writing use the configured OpenAI backend: Codex by default, or direct API when `AI_AGENT_AUDIT_OPENAI_BACKEND=api`. Discovery-style phases (patterns, actors, invariants) use the provider configured in [src/config.rs](src/config.rs). Findings are aggregated across contracts and deduplicated at the end.

8. Report export and local persistence. The tool writes Markdown outputs, records findings in local SQLite databases, and keeps cached repo metadata for later runs.

## Outputs And Local State

Outputs are written relative to the current working directory. The main output folder is named after the target repo, or `repo/subfolder` if `subfolder` is configured.

### Generated Markdown

- `<repo_name>/report/audit-report.md`
  The main aggregated audit report.
- `<repo_name>/report/<sanitized-finding-title>.md`
  One file per finding when a detailed competition-style report was generated for that finding. Filenames are sanitized and truncated.
- `<repo_name>/metadata-<unique_repo_hash>.md`
  Saved metadata context used during analysis.
- `<repo_name>/<ContractType>-<Contract>-size-<tokens>.md`
  Exported codeblocks for in-scope contracts.

### Local SQLite State

By default, the tool stores local state under `.ai-agent-audit/`:

- `.ai-agent-audit/semantic.db`
- `.ai-agent-audit/codeblock.db`
- `.ai-agent-audit/findings.db`
- `.ai-agent-audit/repo_data.db`

Set `AI_AGENT_AUDIT_DATA_DIR` if you want those files elsewhere.

## Validation Workflow

The Rust pipeline performs discovery and initial verification, then emits a `validation-three-shot` config/job for deeper validation, PoC generation, and report creation.

Codex-supervised mode is the strongest path for high-stakes work because fresh Codex workers can inspect files, create PoCs, and repair reports round by round:

```bash
python3 scripts/three_shot_round.py prepare-scope --config validation-three-shot/config.yaml --write-prompt /tmp/three-shot-r1.md
```

For users who want the validation phase to run immediately instead of being monitored by a GUI supervisor:

```bash
cp validation-three-shot/config.yaml validation-three-shot/my-run.yaml
# edit benchmark, run_id, paths.source_root, paths.audit_root, and paths.audit_report
python3 scripts/three_shot_round.py run --config validation-three-shot/my-run.yaml
```

For a cheaper validation-only pass before PoCs and report review:

```bash
python3 scripts/three_shot_round.py run --config validation-three-shot/my-run.yaml --skip-poc
```

The validation runner uses the Codex CLI by default. These prompts require an agent worker with local file read/write and test execution, so a raw `OPENAI_API_KEY` alone cannot run the PoC/report validation workflow. Set `AI_AGENT_AUDIT_WORKER_LAUNCHER` or `workers.default.launcher` if your Codex-compatible binary or wrapper has a different name; the env var wins for one-off runs.

## Analysis Coverage

The analysis system combines several sources of context:

- Pattern libraries covering access control, reentrancy, accounting and invariant drift, oracle and AMM behavior, governance and timelocks, upgradeability, bridging, token standards, marketplace flows, and more.
- Invariant analysis across arithmetic, balance, permission, temporal, referential, and state-machine categories.
- Actor-oriented prompt context for threat modeling each reviewed contract.
- Contest-aware severity handling for `Code4rena`, `Sherlock`, and `Cantina`, bounty-specific Critical/High handling for `Code4renaBounty`, plus a more open-ended `Client` mode.

The exact prompts and pattern catalogs continue to evolve, so the README intentionally describes this at the capability level instead of freezing brittle counts.

## Safety And Limitations

- This project sends code and documentation to external AI providers. Do not use it on repositories you are not allowed to share with those providers.
- The tool is designed for defensive review support. It can miss real issues and it can produce false positives.
- The default Rust OpenAI path depends on a valid cached ChatGPT/Codex session. API fallback for Rust OpenAI calls requires `AI_AGENT_AUDIT_OPENAI_BACKEND=api` and `OPENAI_API_KEY`; deep validation workers still require a Codex-compatible agent launcher.
- `audit_type` affects severity, rubric behavior, context gathering, and validation profile selection. `Code4renaBounty` disables V12-specific validation stages and uses a stricter Critical/High submit/no-submit flow.
- Runnable PoC generation and verification now live in the separate `validation-three-shot` workflow, not the main audit pipeline.
- If the target repo does not build cleanly on the local machine, analysis quality will degrade or the run may fail.
- Automatic build commands do not install package dependencies. Because execution is host-local rather than Docker-isolated, run `npm install`, `yarn install`, `pnpm install`, or `forge install` yourself only when you trust the target repo, or provide an explicit `build_cmd`.
- If Slither cannot extract semantic data, the tool falls back to a reduced analysis path with less Slither-derived context.

## Troubleshooting

### Missing API Keys

If Codex startup cannot authenticate OpenAI access, rerun the tool and complete the ChatGPT/Codex sign-in prompt. The default path does not require `OPENAI_API_KEY`.

If you do not have Codex access and want to run the Rust audit path with direct API billing, set:

```bash
AI_AGENT_AUDIT_OPENAI_BACKEND=api
OPENAI_API_KEY=your_openai_api_key
```

### Build System Not Detected

If the build output shows `No build system detected`, the target repo likely does not expose a recognizable `foundry.toml` or `hardhat.config.*` at the analyzed root. Set `subfolder`, `monorepo_folders`, or `builder: "Custom"` plus `build_cmd`.

### Missing Runtime Dependencies

The Docker execution path has been removed. If `git`, `slither`, `forge`, `node`, `npm`, `npx`, `yarn`, or `pnpm` is required and missing or incompatible, startup/build/static-analysis will fail with an install note for the missing command. Node-based builds require Node.js 18 or newer. Modern Foundry/solc projects require Slither 0.11.5 or newer.

### Wrong Source Folder

If the run finishes but contract coverage looks wrong, check `code_folders`. The default is `src`, but many repos use `contracts`, `src/contracts`, or multiple package roots.

### Stale Cached Workspace

If a rerun is clearly using stale build artifacts, set `force_rebuild: true` or pass `--force-rebuild`.

### Slither Fails But The Run Continues

That is expected behavior. The tool can continue in a reduced mode, but some Slither-derived metadata will be skipped.

## Project Structure

The current codebase is organized around these modules:

```text
src/
  main.rs                 CLI entrypoint and top-level orchestration
  config.rs               environment/config loading and constants
  cli_args/               clap/YAML argument parsing
  prepare_code/           repo cloning, generated context, filtering, native builds, repo metadata
  build_brain/            Slither enrichment and graph DB
  enumerator/             codeblock generation, Solidity parsing, interface indexing
  llm_review/             prompt generation, agent setup, findings, review phases
  reporting/              audit reports, finding reports, exported artifacts
  cost/                   inference cost tracking
  utils/                  shared helpers
tests/                    hermetic and manual integration tests
scripts/run_ci_tests.sh   curated public CI test runner
```

## Testing And CI

Public CI currently runs:

- `cargo fmt --check`
- `cargo check --tests`
- `bash scripts/run_ci_tests.sh`

That hermetic test runner is defined in [scripts/run_ci_tests.sh](scripts/run_ci_tests.sh) and wired through [`.github/workflows/ci.yml`](.github/workflows/ci.yml).

For local development:

```bash
cargo fmt
cargo check
bash scripts/run_ci_tests.sh
```

Manual or live-provider diagnostics are kept behind ignored tests:

```bash
cargo test -- --ignored
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution expectations and [SECURITY.md](SECURITY.md) for private vulnerability reporting.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).
