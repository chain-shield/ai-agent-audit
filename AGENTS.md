# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

## Overview

AI Agent Audit is a Rust-based smart contract security analysis tool that combines static analysis and multiple LLM providers to perform comprehensive security audits of Solidity codebases. The tool processes Git repositories containing smart contracts and generates detailed audit reports with vulnerability findings.

## Core Architecture

### High-Level Workflow
1. **Repository Preparation** (`prepare_code/`) - Clone and build repositories in Docker containers
2. **Static Analysis** (`build_brain/`) - Extract call graphs, IR, and storage layouts using Slither
3. **Code Enumeration** (`enumerator/`) - Generate contextual code slices for focused analysis
4. **AI Analysis** (`llm_review/`) - Multi-phase security analysis with 80+ vulnerability patterns
5. **Report Generation** (`reporting/`) - Create professional audit reports with findings

### Key Components
- **AI Agents** (`ai_bot/`) - Vector-enhanced AI agents with semantic context retrieval
- **LLM Configuration** (`llm_review/agent/agent_factory.rs`) - Multi-provider LLM support (OpenAI, Anthropic, Gemini, DeepSeek)
- **Vulnerability Patterns** (`llm_review/threat_models/patterns.rs`) - 80+ distinct vulnerability patterns
- **Pattern Categories** (`llm_review/threat_models/pattern_category.rs`) - Organized pattern libraries (Signature Validation, Vault Share-Based, Oracle Price Feed, AMM/DEX, etc.)
- **Invariant Analysis** (`llm_review/threat_models/invariants.rs`) - Protocol invariant analysis across 6 categories
- **Actor Discovery** (`llm_review/threat_models/actors.rs`) - Systematic threat actor identification
- **Cost Tracking** (`cost/`) - Real-time inference cost monitoring across providers

## Common Development Commands

### Build and Run
```bash
# Build the project
cargo build --release

# Run with a repository URL
cargo run --release -- https://github.com/example/solidity-project.git

# Run with logging
RUST_LOG=info cargo run --release -- <repo-url>
```

### Development
```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy for linting
cargo clippy

# Build documentation
cargo doc --open
```

## Configuration

### Environment Variables
Required environment variables (create `.env` file):
```bash
# Required
OPENAI_API_KEY=your_openai_api_key

# Optional LLM Providers
ANTHROPIC_API_KEY=your_anthropic_api_key
GOOGLE_AI_API_KEY=your_gemini_api_key
DEEPSEEK_API_KEY=your_deepseek_api_key

# Logging
RUST_LOG=info
```

### Key Configuration Constants
- `MAX_DEPTH: usize = 3` - Call graph traversal depth (src/main.rs)
- `TOKEN_BUDGET: usize = 150_000` - Maximum tokens per code block (src/main.rs)
- `RUNS: usize = 3` - Number of discovery rounds per contract (src/config.rs)
- `INVARIANT_RUNS: usize = 1` - Number of invariant discovery rounds (src/config.rs)

## Module Architecture

### Core Analysis Pipeline
- **`build_brain/enrichment.rs`** - Slither integration and semantic database building
- **`build_brain/callgraph.rs`** - Call graph analysis and traversal
- **`enumerator/codeblocks.rs`** - Code slice generation with call graph context and token budgeting
- **`llm_review/analysis/code_review_v2.rs`** - Main security analysis orchestration

### AI Analysis System
- **`llm_review/agent/agent_factory.rs`** - Centralized AI agent creation with multi-provider support
- **`llm_review/utils/review_utils.rs`** - AI agent builders and utilities
- **`llm_review/dynamic_prompts/`** - Dynamic prompt generation (findings, invariants, actors)
- **`llm_review/prompt_support/`** - Multi-stage prompt engineering (PoC generation, deduplication, report creation)
- **`ai_bot/agent.rs`** - Vector-enhanced AI agents with semantic search

### Vulnerability Detection
The system uses a comprehensive pattern-based approach:
- **80+ Vulnerability Patterns** (`llm_review/threat_models/patterns.rs`) - Distinct vulnerability types including:
  - Access Control & Auth (6 patterns): AccessControlOrAuthByPass, GovernanceDelegationFlaw, DoubleExecutionOrReplay, etc.
  - Reentrancy & Call Order (4 patterns): Reentrancy, ReadOnlyReentrancy, CEIViolation, etc.
  - Economic & Oracle (5 patterns): SlippageMissingOrInsufficient, OracleUsingDEXorTWAP, FlashLoanEconomicManipulation, etc.
  - Accounting & Invariants (4 patterns): AccountingInvariantViolation, PrecisionDriftAccumulation, etc.
  - Game Theory & Incentives (9 patterns): UnincentivizedMaintenanceOrKeeperlessProgress, FirstOrLastMoverAdvantage, etc.
- **Pattern Categories** (`llm_review/threat_models/pattern_category.rs`) - Organized libraries:
  - Signature Validation (10 patterns)
  - Vault Share-Based (11 patterns)
  - Oracle Price Feed (12 patterns)
  - AMM/DEX (14 patterns)
  - Marketplace/Exchange (14 patterns)
  - Common Patterns (20 patterns)
  - Library Analysis (20 patterns)
  - Frequent Patterns (15 patterns)
  - Relevant Patterns (61 curated high/medium severity patterns)
- **Invariant Analysis** (6 types): Arithmetic, Balance, Permission, Referential, State Machine, Temporal
- **24 Vulnerability Types** (`llm_review/findings/finding_enums.rs`) - For categorization and deduplication

### Analysis Phases
The tool uses a multi-phase analysis workflow:

#### Pattern Discovery Phase (`pattern_phases/`)
- **`generate_patterns.rs`** - Discovers vulnerability patterns using AI agents
- **`generate_actors.rs`** - Identifies threat actors and their capabilities
- **`generate_direct_findings.rs`** - Direct finding generation from patterns
- **`verify_patterns.rs`** - Deduplicates and verifies discovered patterns

#### Verification & Validation Phase (`phases/`)
- **`verify_rounds.rs`** - Multi-round verification of findings with 11-gate analysis:
  1. Verify Security Finding Exists
  2. Existing Safeguards Check
  3. Scope Check
  4. By Design Check
  5. Exploitability Check
  6. Impact Classification Check
  7. Likelihood Assessment Check
  8. User Error Check
  9. Governance/Centralization Risk Check
  10. Speculation Check
  11. Non-Standard ERC20 Token Check
- **`rounds/validate_round.rs`** - Validation round for false negative detection
- **`add_poc_findings.rs`** - Generates Proof-of-Concept tests for findings
- **`create_report.rs`** - Creates professional audit reports

### Docker Integration
- **`entrypoint.sh`** - Repository building script (supports Foundry and Hardhat)
- **`prepare_code/git_clone.rs`** - Docker-based repository cloning and building

## Output Files

After analysis, the tool generates:
- `{repo-name}-audit-{hash}-audit-report.md` - Comprehensive audit report
- `{repo-name}-audit-{hash}-free-audit-report.md` - Limited audit report
- `{ContractName}-{repo-name}-audit-{hash}.md` - Individual contract analysis
- `metadata-{repo-name}-audit-{hash}.md` - Protocol metadata and context
- `callgraph.json` - Complete call graph data
- `inheritance.json` - Contract inheritance relationships
- `graph.json` - Semantic graph database
- `sarif.json` - SARIF format analysis results

## Key Data Structures

### Security Findings
- **`Finding`** (src/llm_review/findings/findings.rs) - Individual vulnerability finding with severity, impact, PoC, and mitigation
- **`Findings`** - Collection of findings for a contract
- **`Pattern`** (src/llm_review/threat_models/patterns.rs) - Vulnerability pattern instance with contract/function location
- **`InvariantFinding`** (src/llm_review/threat_models/invariants.rs) - Protocol invariant violation with pre/post state analysis

### Analysis Configuration
- **`VulnerabilityPattern`** enum - 80+ distinct vulnerability patterns
- **`VulnerabilityType`** enum - 24 distinct vulnerability categories for deduplication
- **`Severity`** enum - High, Medium, Low, Info severity levels
- **`ImpactHint`** enum - High, HighMedium, Medium, MediumLow, Low impact hints
- **`LlmCostType`** enum - Cost tracking across different LLM providers

## Security Considerations

This tool is designed for defensive security analysis only:
- All prompts focus on vulnerability detection and mitigation
- No offensive security capabilities
- Secure Docker-based repository processing
- Input validation for repository URLs and paths

## Cost Management

Real-time cost tracking across providers:
- OpenAI: $2.00/$8.00 per 1M tokens (input/output)
- Anthropic: $3.00/$15.00 per 1M tokens
- Gemini: $1.25/$10.00 per 1M tokens
- DeepSeek: $0.07/$1.10 per 1M tokens

Cost tracking in `cost/cost_data.rs` with provider-specific calculations.

## Development Notes

- No unit tests currently exist in the codebase
- The tool uses `edition = "2024"` in Cargo.toml
- Heavy reliance on async/await patterns with tokio runtime
- Extensive use of error handling with anyhow crate
- Vector database operations require Qdrant running on localhost:6334
