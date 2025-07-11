# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

AI Agent Audit is a Rust-based smart contract security analysis tool that combines static analysis, multiple LLM providers, and vector embeddings to perform comprehensive security audits of Solidity codebases. The tool processes Git repositories containing smart contracts and generates detailed audit reports with vulnerability findings.

## Core Architecture

### High-Level Workflow
1. **Repository Preparation** (`prepare_code/`) - Clone and build repositories in Docker containers
2. **Static Analysis** (`build_brain/`) - Extract call graphs, IR, and storage layouts using Slither
3. **Code Enumeration** (`enumerator/`) - Generate contextual code slices for focused analysis
4. **AI Analysis** (`llm_review/`) - Multi-LLM security analysis across 24+ vulnerability categories
5. **Vector Database** (`build_brain/vector_db.rs`) - Store embeddings in Qdrant for semantic search
6. **Report Generation** (`reporting/`) - Create professional audit reports with findings

### Key Components
- **AI Agents** (`ai_bot/`) - Vector-enhanced AI agents with semantic context retrieval
- **LLM Configuration** (`llm_review/config.rs`) - Multi-provider LLM support (OpenAI, Anthropic, Gemini, DeepSeek)
- **Vulnerability Detection** (`prompts/`) - 21 specialized vulnerability detection modules
- **Invariant Analysis** (`invariant_prompts/`) - Protocol invariant analysis across 6 categories
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

### Environment Setup
```bash
# Start Qdrant vector database
docker-compose up -d

# Clean up Docker volumes
docker-compose down --volumes
```

## Configuration

### Environment Variables
Required environment variables (create `.env` file):
```bash
# Required
OPENAI_API_KEY=your_openai_api_key
QDRANT_URL=http://localhost:6334

# Optional LLM Providers
ANTHROPIC_API_KEY=your_anthropic_api_key
GOOGLE_AI_API_KEY=your_gemini_api_key
DEEPSEEK_API_KEY=your_deepseek_api_key

# Logging
RUST_LOG=info
```

### Key Configuration Constants
- `MAX_DEPTH: usize = 3` - Call graph traversal depth (src/main.rs:30)
- `TOKEN_BUDGET: usize = 150_000` - Maximum tokens per code block (src/main.rs:33)
- `RUNS: usize = 3` - Number of discovery rounds per contract (src/llm_review/config.rs:43)

## Module Architecture

### Core Analysis Pipeline
- **`build_brain/enrichment.rs`** - Slither integration and semantic database building
- **`build_brain/callgraph.rs`** - Call graph analysis and traversal
- **`build_brain/vector_db.rs`** - Qdrant vector database operations
- **`enumerator/codeblock_maker.rs`** - Code slice generation with call graph context
- **`llm_review/code_review.rs`** - Main security analysis orchestration

### AI Analysis System
- **`llm_review/config.rs`** - LLM provider configuration and model definitions
- **`llm_review/review_utils.rs`** - AI agent builders and utilities
- **`llm_review/prompt_support/`** - Multi-stage prompt engineering (pre/post/qualify/verify)
- **`ai_bot/agent.rs`** - Vector-enhanced AI agents with semantic search

### Vulnerability Detection
The system detects 24 distinct vulnerability categories:
- **Core Security** (8 active): Reentrancy, Access Control, DoS, Integer Math, Pragma, Randomness, Unexpected ETH, MEV
- **Quality Checks** (24 total): All vulnerability types for verification and deduplication
- **Invariant Analysis** (6 types): Arithmetic, Balance, Permission, Referential, State Machine, Temporal

### Docker Integration
- **`docker-compose.yml`** - Qdrant vector database service
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
- **`Finding`** (src/llm_review/config.rs:46) - Individual vulnerability finding with severity, impact, PoC, and mitigation
- **`InvariantFinding`** (src/llm_review/config.rs:67) - Protocol invariant violation with pre/post state analysis

### Analysis Configuration
- **`VulnerabilityType`** enum - 24 distinct vulnerability categories
- **`Severity`** enum - High, Medium, Low, Info severity levels
- **`LlmCostType`** enum - Cost tracking across different LLM providers

## Vector Database Integration

The tool creates unique Qdrant collections for each repository:
- Collection naming: `{repo_hash}-contract_chunks`
- Embeddings: OpenAI text-embedding-3-small
- Metadata: Contract name, function context, file paths
- Query endpoint: `http://localhost:6334`

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